//! SPEC: canon §2.4-2.5 Sim API skeleton, canonical serialization, state hash, save and restore
//!
//! A minimal, fully deterministic simulation skeleton (doc 02 §5, §6.5):
//!
//! - [`Sim::new`] builds a run from a run seed and a [`SimConfig`]; party
//!   Errata spawn in the hub (`folio_index = 0`, canon §1.11).
//! - [`Sim::enter_folio`] restarts the per-Folio `EntityId` and uid counters at
//!   1, derives the Folio's sequential streams fresh with `stream_seed`
//!   (canon §3.2) and generates a small population, so a Folio's state depends
//!   only on the run seed, the Folio index and its genre inputs.
//! - [`Sim::step`] simulates one tick: commands apply in `(tick, slot)` order
//!   (canon §2.4, slots `0..=4`), then a subset of doc 03 §2.2's system order:
//!   input apply, AI think (per-entity phase), actions (keyed `rng.combat` and
//!   `rng.status` draws), movement (semi-implicit Euler, canon §2.1), status
//!   pulse (accumulator `k = 4`), resources (accumulator `k = 60`), status
//!   timers, despawn. Every mutating loop runs in ascending `EntityId`.
//! - The world is a sorted column store (doc 02 §5.2) with a small component
//!   set: kind, control, inventory, player, pools (`Milli`), position (`Fx3`),
//!   statuses, velocity (`Fx3`).
//! - [`Sim::serialize`] is the hand-written canonical serialization (canon
//!   §2.5, doc 02 §6.5 framing `u8(k) ‖ u32le(len) ‖ S_k` for 13 sections):
//!   fixed field order, keys ascending, little-endian integers, no serde.
//!   [`Sim::state_hash`] is BLAKE3 of it; [`Sim::save`] prefixes the canon
//!   §2.4 header; [`Sim::restore`] decodes strictly and rejects any input that
//!   does not re-serialize to the same bytes.
//!
//! Gameplay numbers marked "skeleton" below are placeholders that exercise
//! determinism; the owning documents (03, 04, 05) define the real systems.

use std::collections::{BTreeMap, BTreeSet};

use crate::entity::{EntityAllocator, EntityId, IdCounter, ItemUid, MAX_FOLIO_INDEX, UidAllocator};
use crate::time::{
    CombatTicks, K_PER_TICK, K_STATUS_PULSE, RateAcc, StatusTimer, Subsystem, Tick, Ticks,
    after_secs_without, boundary_after, is_hash_boundary, runs_on, runs_on_entity, secs,
};
use palimpsest_math::geom::{Fx3, semi_implicit_euler};
use palimpsest_math::int::{ceil_div, floor_div, mul_bp, narrow_i64};
use palimpsest_math::rng::{seq, stream_rng, streams};
use palimpsest_math::trig::clamp_pitch;
use palimpsest_math::{
    Bam, Fx, Rng, blake3_256, cos_bam, dist2, fnv1a32, fx_from_milli, isqrt_u128, keyed_rng,
    sin_bam,
};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// SPEC: canon §2.4, doc 02 §5.5 `SIM_VERSION`, carried in every save,
/// replay and lobby handshake.
pub const SIM_VERSION: u32 = 1;

/// SPEC: canon §2.4. Replay slots `0..=4`: party `0..=3`, invader 4.
pub const SLOTS: usize = 5;

/// SPEC: canon §2.4, §17.9. The invader's replay slot.
pub const INVADER_SLOT: u8 = 4;

/// SPEC: canon §2.4 party slots `0..=3`: at most 4 party members.
pub const MAX_PARTY: u8 = 4;

/// SPEC: canon §2.5, doc 02 §6.5. Number of serialization sections.
pub const SECTIONS: usize = 13;

/// SPEC: canon §2.4. Encoded length of the session header:
/// `sim_version u32 ‖ content_hash [u8; 32] ‖ run_seed u64`.
pub const HEADER_LEN: usize = 44;

/// SPEC: canon §2.5. `state_hash` is a 32-byte BLAKE3 digest.
pub type StateHash = [u8; 32];

/// SPEC: canon §1.2 `Milli`: display value × 1000.
pub type Milli = i64;

/// SPEC: canon §2.4, doc 02 §5.5. Button bits of [`PlayerCommand::buttons`]
/// that the skeleton reads, plus the reserved mask.
pub mod buttons {
    /// SPEC: canon §2.4, doc 02 §5.5 bit 0 `ATTACK_PRIMARY`.
    pub const ATTACK_PRIMARY: u32 = 1 << 0;
    /// SPEC: canon §2.4, doc 02 §5.5 bit 3 `JUMP`.
    pub const JUMP: u32 = 1 << 3;
    /// SPEC: canon §2.4, doc 02 §5.5 bit 5 `SPRINT`.
    pub const SPRINT: u32 = 1 << 5;
    /// SPEC: canon §2.4, doc 02 §5.5 bits 26–31: reserved, must be 0; the replay
    /// validator rejects set bits.
    pub const RESERVED: u32 = 0xFC00_0000;
    /// SPEC: canon §11.8 (canon-1.11), doc 16 §5.4 `HOLD_MASK`: the held bits
    /// a neutral substitute keeps, `CROUCH` (bit 4), `WALK` (bit 24) and
    /// `QUICKBAR_INDEX` (bits 15–17): stance and the selected quickbar index.
    pub const HOLD_MASK: u32 = (1 << 4) | (1 << 24) | (0b111 << 15);
}

/// SPEC: canon §11.8 (canon-1.11), doc 16 §5.4. The neutral substitute the
/// host applies when a slot's command for tick `t` is missing: no movement,
/// every action button released, stance and quickbar index kept, the last
/// yaw, pitch and `action_seq`, `view_tick = t`. It holds no press edge (every
/// bit it holds was already held), so it starts nothing; input is never
/// synthesized for a player. The replay log records it.
#[must_use]
pub const fn neutral_substitute(last: &PlayerCommand, t: Tick) -> PlayerCommand {
    PlayerCommand {
        tick: t,
        slot: last.slot,
        move_x: 0,
        move_z: 0,
        yaw: last.yaw,
        pitch: last.pitch,
        buttons: last.buttons & buttons::HOLD_MASK,
        action_seq: last.action_seq,
        view_tick: t,
    }
}

/// SPEC: canon §1.8. The Folio box spans `x, z ∈ [0, 256)` m; positions clamp
/// to the largest raw value below 256 m.
const BOX_MAX: Fx = Fx::from_raw((256i64 << 32) - 1);
/// SPEC: canon §4.2 walk speed, 4,300 mm/s.
const WALK: Fx = fx_from_milli(4300);
/// SPEC: canon §4.2 sprint speed, 6,500 mm/s.
const SPRINT: Fx = fx_from_milli(6500);
/// SPEC: canon §4 `GRAVITY_BASE`, 20,000 mm/s².
const GRAVITY: Fx = fx_from_milli(20_000);
/// Skeleton: jump launch speed, about `sqrt(2 × 20 m/s² × 1.1 m)`.
const JUMP_SPEED: Fx = fx_from_milli(6633);
/// Skeleton: creature wander speed.
const CREATURE_SPEED: Fx = fx_from_milli(1000);
/// Skeleton: horizontal velocity relaxation rate toward the wish velocity, per s.
const ACCEL_PER_S: i64 = 10;
/// SPEC: canon §6.4 row 1 at all-40, L1: Max HP 85,000 mHP (§21 test 13).
const ERRATUM_HP: Milli = 85_000;
/// SPEC: canon §6.4 row 4 at all-40, L1: Max stamina 90 (90,000 milli).
const ERRATUM_STAMINA: Milli = 90_000;
/// SPEC: canon §6.4 row 2 at d(RES) = 0: `max(10, 50)` Bp of max HP per s.
const HP_REGEN_BP: i32 = 50;
/// SPEC: canon §6.4 row 2: regeneration starts after 5 s without damage.
const HP_REGEN_DELAY_S: u32 = 5;
/// SPEC: canon §6.4 row 5 at all-40: 23 stamina per s (23,000 milli).
const STAMINA_REGEN: Milli = 23_000;
/// SPEC: canon §6.4 row 5: regeneration starts after 1 s without spending.
const STAMINA_REGEN_DELAY_S: u32 = 1;
/// SPEC: canon §6.4 row 6 at all-40: sprint drains 12 stamina per s.
const SPRINT_DRAIN: Milli = 12_000;
/// Skeleton: creature max HP.
const CREATURE_HP: Milli = 20_000;
/// Skeleton: melee reach, 3 m, compared as raw Q64.64 squared distance.
const MELEE_RANGE: Fx = Fx::from_int(3);
/// Skeleton: base melee damage.
const HIT_DAMAGE: Milli = 10_000;
/// SPEC: canon §6.4 row 13 at all-40: crit chance 500 Bp.
const CRIT_CHANCE_BP: i32 = 500;
/// SPEC: canon §6.4 row 14: crit damage multiplier 15000 Bp.
const CRIT_MULT_BP: i32 = 15_000;
/// Skeleton: an "X Bp chance to apply" bleed on hit (canon §3.3 `rng.status`).
const BLEED_CHANCE_BP: i32 = 2500;
/// Skeleton: bleed duration, 5 s.
const BLEED_TICKS: Ticks = secs(5);
/// SPEC: canon §21 test 10: 850 mHP per s through the status pulse, `k = 4`.
const BLEED_DOT: Milli = 850;
/// SPEC: canon §1.1 `id32` of `st.bleeding`.
pub const ST_BLEEDING: u32 = fnv1a32(b"st.bleeding");
/// Skeleton: the hub spawn point.
const HUB_SPAWN: Fx3 = Fx3::from_ints(8, 0, 8);
/// SPEC: canon §3.2. The per-Folio sequential streams this skeleton derives
/// fresh at Folio entry, in ascending name order.
pub const FOLIO_STREAMS: [&str; 3] = [streams::AI_IDLE, streams::SPAWN, streams::WORLDGEN];

// ---------------------------------------------------------------------------
// Display helpers
// ---------------------------------------------------------------------------

/// SPEC: canon §1.5. A `Milli` value displays as `floor(x / 1000)`.
#[must_use]
pub const fn display_milli(x: Milli) -> i64 {
    floor_div(x, 1000)
}

/// SPEC: canon §1.5, §21 test 30. Current HP displays as `ceil_div(x, 1000)`,
/// so a living entity never shows 0 HP: 400 mHP displays 1.
#[must_use]
pub const fn display_hp(current_mhp: Milli) -> i64 {
    ceil_div(current_mhp, 1000)
}

/// SPEC: canon §1.5 (canon-1.11). How a per-second `Milli` rate displays.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum RateDisplay {
    /// Below 10 display units: one decimal place, as a count of tenths.
    Tenths(i64),
    /// Otherwise: whole display units.
    Whole(i64),
}

/// SPEC: canon §1.5 (canon-1.11): per-second rates below 10 display units
/// display with one decimal place as `floor(x / 100)` tenths; others as
/// `floor(x / 1000)`. Doc 17's example: 673 mHP/s displays "0.6".
/// GAP: "below 10 display units" is read as `x < 10,000`, so negative rates
/// also use tenths.
#[must_use]
pub const fn display_rate_per_s(x: Milli) -> RateDisplay {
    if x < 10_000 {
        RateDisplay::Tenths(floor_div(x, 100))
    } else {
        RateDisplay::Whole(floor_div(x, 1000))
    }
}

// ---------------------------------------------------------------------------
// Canonical serialization primitives
// ---------------------------------------------------------------------------

/// SPEC: canon §2.5. Why bytes failed to decode.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DecodeError {
    /// The input ended inside a field.
    UnexpectedEnd,
    /// Bytes remain after the last field.
    TrailingBytes,
    /// An enum or option tag has an unknown value.
    BadTag(&'static str),
    /// A sorted list or map is not strictly ascending.
    NotAscending(&'static str),
    /// A section header carries the wrong index.
    BadSection {
        /// The index expected at this position.
        expected: u8,
        /// The index found.
        found: u8,
    },
    /// A string is not UTF-8.
    BadUtf8,
    /// A field value is outside its domain.
    Invalid(&'static str),
}

/// SPEC: canon §2.5. A strict little-endian reader over canonical bytes.
#[derive(Clone, Debug)]
pub struct Reader<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    /// SPEC: canon §2.5. Read `bytes` from the start.
    #[must_use]
    pub const fn new(bytes: &'a [u8]) -> Reader<'a> {
        Reader { bytes, pos: 0 }
    }

    /// SPEC: canon §2.5. Bytes not yet read.
    #[must_use]
    pub const fn remaining(&self) -> usize {
        self.bytes.len() - self.pos
    }

    /// SPEC: canon §2.5. Take the next `n` bytes.
    pub fn bytes(&mut self, n: usize) -> Result<&'a [u8], DecodeError> {
        let end = self.pos.checked_add(n).ok_or(DecodeError::UnexpectedEnd)?;
        let s = self
            .bytes
            .get(self.pos..end)
            .ok_or(DecodeError::UnexpectedEnd)?;
        self.pos = end;
        Ok(s)
    }

    /// SPEC: canon §2.5. Take the next `N` bytes as an array.
    pub fn array<const N: usize>(&mut self) -> Result<[u8; N], DecodeError> {
        let s = self.bytes(N)?;
        let mut a = [0u8; N];
        a.copy_from_slice(s);
        Ok(a)
    }

    /// SPEC: canon §2.5, §1.11. A `u32le` length or count.
    pub fn len_u32(&mut self) -> Result<usize, DecodeError> {
        let n = u32::read(self)?;
        usize::try_from(n).map_err(|_| DecodeError::Invalid("length exceeds usize"))
    }

    /// SPEC: canon §2.5 "self-delimiting": succeed only when every byte was
    /// consumed.
    pub const fn finish(self) -> Result<(), DecodeError> {
        if self.pos == self.bytes.len() {
            Ok(())
        } else {
            Err(DecodeError::TrailingBytes)
        }
    }
}

/// SPEC: canon §2.5 canonical serialization: a value writes its fields in a
/// fixed order as little-endian integers, and reads them back strictly.
pub trait Canonical: Sized {
    /// SPEC: canon §2.5. Append the canonical bytes of `self`.
    fn put(&self, out: &mut Vec<u8>);
    /// SPEC: canon §2.5. Read one value; reject anything non-canonical that
    /// the field itself can detect.
    fn read(r: &mut Reader<'_>) -> Result<Self, DecodeError>;
}

/// SPEC: canon §2.5. Write a length or count as `u32le`; panics above
/// `u32::MAX` (no state list comes near it).
#[track_caller]
pub fn put_len(out: &mut Vec<u8>, n: usize) {
    let n = u32::try_from(n).expect("put_len: length exceeds u32");
    n.put(out);
}

macro_rules! canonical_int {
    ($($t:ty),*) => {$(
        impl Canonical for $t {
            fn put(&self, out: &mut Vec<u8>) {
                out.extend_from_slice(&self.to_le_bytes());
            }
            fn read(r: &mut Reader<'_>) -> Result<Self, DecodeError> {
                Ok(<$t>::from_le_bytes(r.array()?))
            }
        }
    )*};
}
canonical_int!(u8, i8, u16, i16, u32, i32, u64, i64);

impl Canonical for bool {
    fn put(&self, out: &mut Vec<u8>) {
        out.push(u8::from(*self));
    }
    fn read(r: &mut Reader<'_>) -> Result<Self, DecodeError> {
        match u8::read(r)? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(DecodeError::BadTag("bool")),
        }
    }
}

impl<T: Canonical> Canonical for Option<T> {
    fn put(&self, out: &mut Vec<u8>) {
        match self {
            None => out.push(0),
            Some(v) => {
                out.push(1);
                v.put(out);
            }
        }
    }
    fn read(r: &mut Reader<'_>) -> Result<Self, DecodeError> {
        match u8::read(r)? {
            0 => Ok(None),
            1 => Ok(Some(T::read(r)?)),
            _ => Err(DecodeError::BadTag("option")),
        }
    }
}

impl<T: Canonical> Canonical for Vec<T> {
    fn put(&self, out: &mut Vec<u8>) {
        put_len(out, self.len());
        for v in self {
            v.put(out);
        }
    }
    fn read(r: &mut Reader<'_>) -> Result<Self, DecodeError> {
        let n = r.len_u32()?;
        // Every element is at least one byte, so a bogus count cannot force a
        // huge allocation.
        let mut v = Vec::with_capacity(n.min(r.remaining()));
        for _ in 0..n {
            v.push(T::read(r)?);
        }
        Ok(v)
    }
}

impl Canonical for [u8; 32] {
    fn put(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(self);
    }
    fn read(r: &mut Reader<'_>) -> Result<Self, DecodeError> {
        r.array()
    }
}

impl Canonical for String {
    fn put(&self, out: &mut Vec<u8>) {
        put_len(out, self.len());
        out.extend_from_slice(self.as_bytes());
    }
    fn read(r: &mut Reader<'_>) -> Result<Self, DecodeError> {
        let n = r.len_u32()?;
        let b = r.bytes(n)?;
        String::from_utf8(b.to_vec()).map_err(|_| DecodeError::BadUtf8)
    }
}

impl Canonical for Fx {
    fn put(&self, out: &mut Vec<u8>) {
        self.raw().put(out);
    }
    fn read(r: &mut Reader<'_>) -> Result<Self, DecodeError> {
        Ok(Fx::from_raw(i64::read(r)?))
    }
}

impl Canonical for Fx3 {
    fn put(&self, out: &mut Vec<u8>) {
        self.x.put(out);
        self.y.put(out);
        self.z.put(out);
    }
    fn read(r: &mut Reader<'_>) -> Result<Self, DecodeError> {
        Ok(Fx3::new(Fx::read(r)?, Fx::read(r)?, Fx::read(r)?))
    }
}

impl Canonical for EntityId {
    fn put(&self, out: &mut Vec<u8>) {
        self.raw().put(out);
    }
    fn read(r: &mut Reader<'_>) -> Result<Self, DecodeError> {
        Ok(EntityId::from_raw(u64::read(r)?))
    }
}

impl Canonical for ItemUid {
    fn put(&self, out: &mut Vec<u8>) {
        self.raw().put(out);
    }
    fn read(r: &mut Reader<'_>) -> Result<Self, DecodeError> {
        Ok(ItemUid::from_raw(u64::read(r)?))
    }
}

impl Canonical for RateAcc {
    fn put(&self, out: &mut Vec<u8>) {
        self.raw().put(out);
    }
    fn read(r: &mut Reader<'_>) -> Result<Self, DecodeError> {
        Ok(RateAcc::from_raw(i64::read(r)?))
    }
}

impl Canonical for StatusTimer {
    fn put(&self, out: &mut Vec<u8>) {
        self.applied().put(out);
        self.expiry().put(out);
    }
    fn read(r: &mut Reader<'_>) -> Result<Self, DecodeError> {
        let applied = u64::read(r)?;
        let expiry = Option::<u64>::read(r)?;
        if expiry.is_some_and(|e| e < applied) {
            return Err(DecodeError::Invalid("status expiry before applied tick"));
        }
        Ok(StatusTimer::from_parts(applied, expiry))
    }
}

impl Canonical for CombatTicks {
    fn put(&self, out: &mut Vec<u8>) {
        self.dealt.put(out);
        self.taken.put(out);
        self.alerted.put(out);
    }
    fn read(r: &mut Reader<'_>) -> Result<Self, DecodeError> {
        Ok(CombatTicks {
            dealt: Option::read(r)?,
            taken: Option::read(r)?,
            alerted: Option::read(r)?,
        })
    }
}

impl Canonical for Rng {
    fn put(&self, out: &mut Vec<u8>) {
        for w in self.state() {
            w.put(out);
        }
    }
    fn read(r: &mut Reader<'_>) -> Result<Self, DecodeError> {
        let s = [u64::read(r)?, u64::read(r)?, u64::read(r)?, u64::read(r)?];
        Ok(Rng::from_state(s))
    }
}

// ---------------------------------------------------------------------------
// Commands, session header and replay log (canon §2.4)
// ---------------------------------------------------------------------------

/// SPEC: canon §2.4. The host's applied command for one (tick, player slot):
/// `{ tick: u64, slot: u8, move_x: i8, move_z: i8, yaw: u16, pitch: i16,
/// buttons: u32, action_seq: u32, view_tick: u64 }`. Slots `0..=3` are the
/// party and slot 4 the invader.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PlayerCommand {
    /// Input tick the command applies on.
    pub tick: u64,
    /// Player slot, `0..=4`.
    pub slot: u8,
    /// Strafe axis in the yaw frame, `+127` full right; `-128` maps to `-127`.
    pub move_x: i8,
    /// Forward axis in the yaw frame, `-127` full forward (−Z at yaw 0).
    pub move_z: i8,
    /// View yaw, Bam.
    pub yaw: u16,
    /// View pitch; the sim clamps it to `-16000..=16000` (canon §1.9).
    pub pitch: i16,
    /// Held button state (doc 02 §5.5 bit table).
    pub buttons: u32,
    /// The client's action sequence number.
    pub action_seq: u32,
    /// The tick the client was viewing, for lag-compensated hit tests.
    pub view_tick: u64,
}

impl PlayerCommand {
    /// SPEC: canon §2.4. Bytes of one encoded command (8+1+1+1+2+2+4+4+8).
    pub const ENCODED_LEN: usize = 31;

    /// SPEC: canon §2.4, doc 02 §5.5. Field checks independent of sim state:
    /// slot in `0..=4` and reserved button bits 26–31 clear.
    pub const fn validate(&self) -> Result<(), CommandError> {
        if self.slot as usize >= SLOTS {
            return Err(CommandError::BadSlot(self.slot));
        }
        if self.buttons & buttons::RESERVED != 0 {
            return Err(CommandError::ReservedButtons(self.buttons));
        }
        Ok(())
    }

    /// SPEC: canon §2.4. Encode to the fixed 31-byte little-endian form, in
    /// canon field order.
    #[must_use]
    pub fn encode(&self) -> Vec<u8> {
        let mut v = Vec::with_capacity(Self::ENCODED_LEN);
        self.put(&mut v);
        v
    }

    /// SPEC: canon §2.4. Decode exactly one 31-byte command.
    pub fn decode(bytes: &[u8]) -> Result<PlayerCommand, DecodeError> {
        let mut r = Reader::new(bytes);
        let c = PlayerCommand::read(&mut r)?;
        r.finish()?;
        Ok(c)
    }
}

impl Canonical for PlayerCommand {
    fn put(&self, out: &mut Vec<u8>) {
        self.tick.put(out);
        self.slot.put(out);
        self.move_x.put(out);
        self.move_z.put(out);
        self.yaw.put(out);
        self.pitch.put(out);
        self.buttons.put(out);
        self.action_seq.put(out);
        self.view_tick.put(out);
    }
    fn read(r: &mut Reader<'_>) -> Result<Self, DecodeError> {
        Ok(PlayerCommand {
            tick: u64::read(r)?,
            slot: u8::read(r)?,
            move_x: i8::read(r)?,
            move_z: i8::read(r)?,
            yaw: u16::read(r)?,
            pitch: i16::read(r)?,
            buttons: u32::read(r)?,
            action_seq: u32::read(r)?,
            view_tick: u64::read(r)?,
        })
    }
}

/// SPEC: canon §2.4. Why a command was refused. [`Sim::step`] validates every
/// command before changing any state.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CommandError {
    /// The command's tick is not the tick being simulated.
    WrongTick {
        /// The tick being simulated.
        expected: u64,
        /// The command's tick.
        found: u64,
    },
    /// Slot above 4.
    BadSlot(u8),
    /// Reserved button bits 26–31 are set.
    ReservedButtons(u32),
    /// No entity occupies the slot.
    EmptySlot(u8),
    /// Two commands for one (tick, slot).
    DuplicateSlot(u8),
    /// A replay log entry is not after the previous `(tick, slot)`.
    OutOfOrder,
}

/// SPEC: canon §2.4 (canon-1.11). The header of every save and replay:
/// `sim_version`, `content_hash` and `run_seed`. A lobby handshake carries
/// only `sim_version` and `content_hash`; a lobby receives `run_seed` in the
/// run header the host sends when it commits the run (doc 16).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SessionHeader {
    /// [`SIM_VERSION`] of the writer.
    pub sim_version: u32,
    /// BLAKE3 over the canonical serialization (doc 02 §11.2) of every
    /// loaded content record outside the presentation namespaces `art.`,
    /// `aud.`, `vfx.`, `nar.` and `ui.`, plus the simulation-affecting mods,
    /// the run's unlock set and the simulation-affecting flags. Peers, saves
    /// and replays must match it.
    pub content_hash: [u8; 32],
    /// The run seed.
    pub run_seed: u64,
}

/// SPEC: canon §2.4 (canon-1.11). The content namespaces whose records feed
/// `presentation_hash` instead of `content_hash`.
pub const PRESENTATION_NAMESPACES: [&str; 5] = ["art", "aud", "vfx", "nar", "ui"];

/// SPEC: canon §2.4 (canon-1.11). `true` when a content record with this ID
/// belongs to `presentation_hash` (its namespace is `art.`, `aud.`, `vfx.`,
/// `nar.` or `ui.`), `false` when it belongs to `content_hash`.
#[must_use]
pub fn is_presentation_record(content_id: &str) -> bool {
    content_id
        .split_once('.')
        .is_some_and(|(ns, _)| PRESENTATION_NAMESPACES.contains(&ns))
}

/// SPEC: canon §2.4 (canon-1.11). Comparing a replay's hashes with the local
/// ones: `content_hash` and `sim_version` must match; a `presentation_hash`
/// mismatch only warns.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ReplayCompat {
    /// Everything matches.
    Exact,
    /// The simulation matches; presentation differs (warn, then play).
    PresentationMismatch,
    /// `sim_version` or `content_hash` differs: the replay cannot be played.
    Incompatible,
}

impl Canonical for SessionHeader {
    fn put(&self, out: &mut Vec<u8>) {
        self.sim_version.put(out);
        self.content_hash.put(out);
        self.run_seed.put(out);
    }
    fn read(r: &mut Reader<'_>) -> Result<Self, DecodeError> {
        Ok(SessionHeader {
            sim_version: u32::read(r)?,
            content_hash: <[u8; 32]>::read(r)?,
            run_seed: u64::read(r)?,
        })
    }
}

/// SPEC: canon §2.4. A replay log: the session header, the
/// `presentation_hash` (canon-1.11), and the host's applied commands,
/// strictly ascending by `(tick, slot)`, at most one per pair. The skeleton
/// applies no intents, so the log holds no intent records (the canon §2.4
/// intent list is not in the slice yet).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReplayLog {
    /// The session header.
    pub header: SessionHeader,
    /// Canon §2.4 `presentation_hash`: BLAKE3 over the presentation records,
    /// the localization tables and the presentation-only mods. Replays record
    /// it; a mismatch only warns.
    pub presentation_hash: [u8; 32],
    commands: Vec<PlayerCommand>,
}

impl ReplayLog {
    /// SPEC: canon §2.4. An empty log for a session.
    #[must_use]
    pub const fn new(header: SessionHeader, presentation_hash: [u8; 32]) -> ReplayLog {
        ReplayLog {
            header,
            presentation_hash,
            commands: Vec::new(),
        }
    }

    /// SPEC: canon §2.4 (canon-1.11). Whether this replay plays on a build
    /// with `sim_version` and the local `content_hash` and
    /// `presentation_hash`: the simulation hashes must match, presentation
    /// only warns.
    #[must_use]
    pub fn compat(
        &self,
        sim_version: u32,
        content_hash: &[u8; 32],
        presentation_hash: &[u8; 32],
    ) -> ReplayCompat {
        if self.header.sim_version != sim_version || &self.header.content_hash != content_hash {
            ReplayCompat::Incompatible
        } else if &self.presentation_hash != presentation_hash {
            ReplayCompat::PresentationMismatch
        } else {
            ReplayCompat::Exact
        }
    }

    /// SPEC: canon §2.4. Append a command; it must validate and come strictly
    /// after the last logged `(tick, slot)`.
    pub fn push(&mut self, c: PlayerCommand) -> Result<(), CommandError> {
        c.validate()?;
        if let Some(last) = self.commands.last()
            && (c.tick, c.slot) <= (last.tick, last.slot)
        {
            return Err(CommandError::OutOfOrder);
        }
        self.commands.push(c);
        Ok(())
    }

    /// SPEC: canon §2.4. Every logged command in `(tick, slot)` order.
    #[must_use]
    pub fn commands(&self) -> &[PlayerCommand] {
        &self.commands
    }

    /// SPEC: canon §2.4. The commands logged for tick `t`, in slot order.
    #[must_use]
    pub fn for_tick(&self, t: Tick) -> &[PlayerCommand] {
        let lo = self.commands.partition_point(|c| c.tick < t);
        let hi = self.commands.partition_point(|c| c.tick <= t);
        &self.commands[lo..hi]
    }

    /// SPEC: canon §2.4, §2.5. Canonical bytes: header, the 32-byte
    /// `presentation_hash`, `u32le` count, then each 31-byte command.
    #[must_use]
    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::new();
        self.header.put(&mut out);
        self.presentation_hash.put(&mut out);
        self.commands.put(&mut out);
        out
    }

    /// SPEC: canon §2.4. Decode and validate a log: every command validates
    /// and `(tick, slot)` is strictly ascending.
    pub fn decode(bytes: &[u8]) -> Result<ReplayLog, DecodeError> {
        let mut r = Reader::new(bytes);
        let header = SessionHeader::read(&mut r)?;
        let presentation_hash = <[u8; 32]>::read(&mut r)?;
        let cmds = Vec::<PlayerCommand>::read(&mut r)?;
        r.finish()?;
        let mut log = ReplayLog::new(header, presentation_hash);
        for c in cmds {
            log.push(c).map_err(|e| match e {
                CommandError::OutOfOrder => DecodeError::NotAscending("replay commands"),
                _ => DecodeError::Invalid("replay command"),
            })?;
        }
        Ok(log)
    }
}

// ---------------------------------------------------------------------------
// Column store and components (doc 02 §5.2, doc 03 §3.3)
// ---------------------------------------------------------------------------

/// SPEC: canon §1.11, doc 02 §5.2. One component column: `(EntityId, value)`
/// pairs kept strictly ascending by `EntityId`, so iteration is ascending by
/// construction and lookups are binary searches.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Column<T> {
    ids: Vec<EntityId>,
    vals: Vec<T>,
}

impl<T> Default for Column<T> {
    fn default() -> Self {
        Column {
            ids: Vec::new(),
            vals: Vec::new(),
        }
    }
}

impl<T> Column<T> {
    /// SPEC: canon §1.11, doc 02 §5.2. An empty column.
    #[must_use]
    pub const fn new() -> Column<T> {
        Column {
            ids: Vec::new(),
            vals: Vec::new(),
        }
    }

    /// SPEC: canon §1.11, doc 02 §5.2. Number of entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.ids.len()
    }

    /// SPEC: canon §1.11, doc 02 §5.2. `true` when the column holds no entry.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }

    /// SPEC: canon §1.11. The entity IDs, strictly ascending.
    #[must_use]
    pub fn ids(&self) -> &[EntityId] {
        &self.ids
    }

    /// SPEC: canon §1.11, doc 02 §5.2 `get`: binary search.
    #[must_use]
    pub fn get(&self, id: EntityId) -> Option<&T> {
        self.ids.binary_search(&id).ok().map(|i| &self.vals[i])
    }

    /// SPEC: canon §1.11, doc 02 §5.2 `get_mut`: binary search.
    pub fn get_mut(&mut self, id: EntityId) -> Option<&mut T> {
        match self.ids.binary_search(&id) {
            Ok(i) => Some(&mut self.vals[i]),
            Err(_) => None,
        }
    }

    /// SPEC: canon §1.11, doc 02 §5.2. `true` when `id` has this component.
    #[must_use]
    pub fn contains(&self, id: EntityId) -> bool {
        self.ids.binary_search(&id).is_ok()
    }

    /// SPEC: canon §1.11, doc 02 §5.2 `iter`: ascending `EntityId`.
    pub fn iter(&self) -> impl Iterator<Item = (EntityId, &T)> {
        self.ids.iter().copied().zip(self.vals.iter())
    }

    /// SPEC: canon §1.11, doc 02 §5.2 `insert`: append when `id` exceeds the last ID,
    /// otherwise binary-search insert; an existing entry is overwritten.
    pub fn insert(&mut self, id: EntityId, v: T) {
        if self.ids.last().is_none_or(|&last| id > last) {
            self.ids.push(id);
            self.vals.push(v);
            return;
        }
        match self.ids.binary_search(&id) {
            Ok(i) => self.vals[i] = v,
            Err(i) => {
                self.ids.insert(i, id);
                self.vals.insert(i, v);
            }
        }
    }

    /// SPEC: canon §1.11, doc 02 §5.2. Remove `id`'s entry, returning its value.
    pub fn remove(&mut self, id: EntityId) -> Option<T> {
        match self.ids.binary_search(&id) {
            Ok(i) => {
                self.ids.remove(i);
                Some(self.vals.remove(i))
            }
            Err(_) => None,
        }
    }
}

impl<T: Canonical> Canonical for Column<T> {
    fn put(&self, out: &mut Vec<u8>) {
        put_len(out, self.ids.len());
        for (id, v) in self.iter() {
            id.put(out);
            v.put(out);
        }
    }
    fn read(r: &mut Reader<'_>) -> Result<Self, DecodeError> {
        let n = r.len_u32()?;
        let mut c = Column::new();
        for _ in 0..n {
            let id = EntityId::read(r)?;
            if c.ids.last().is_some_and(|&last| id <= last) {
                return Err(DecodeError::NotAscending("column"));
            }
            let v = T::read(r)?;
            c.ids.push(id);
            c.vals.push(v);
        }
        Ok(c)
    }
}

/// SPEC: canon §1.11, doc 03 §3.2 entity kind codes (row order): 0 erratum, 1 creature.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EntityKind {
    /// `grid.ekind.erratum`: a player character.
    Erratum,
    /// `grid.ekind.creature`: any NPC, also the invader (canon §17.9).
    Creature,
}

impl Canonical for EntityKind {
    fn put(&self, out: &mut Vec<u8>) {
        out.push(match self {
            EntityKind::Erratum => 0,
            EntityKind::Creature => 1,
        });
    }
    fn read(r: &mut Reader<'_>) -> Result<Self, DecodeError> {
        match u8::read(r)? {
            0 => Ok(EntityKind::Erratum),
            1 => Ok(EntityKind::Creature),
            _ => Err(DecodeError::BadTag("entity kind")),
        }
    }
}

/// SPEC: canon §2.4, doc 03 §3.4 `Control` (skeleton subset): the movement axes and held
/// buttons written by `sys.input_apply` or AI think.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Control {
    /// Strafe axis, `-127..=127`.
    pub move_x: i8,
    /// Forward axis, `-127..=127`.
    pub move_z: i8,
    /// Held buttons.
    pub buttons: u32,
}

impl Canonical for Control {
    fn put(&self, out: &mut Vec<u8>) {
        self.move_x.put(out);
        self.move_z.put(out);
        self.buttons.put(out);
    }
    fn read(r: &mut Reader<'_>) -> Result<Self, DecodeError> {
        Ok(Control {
            move_x: i8::read(r)?,
            move_z: i8::read(r)?,
            buttons: u32::read(r)?,
        })
    }
}

/// SPEC: canon §1.11, doc 03 §3.3 `Inventory { uids: Vec<u64> }` ascending.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Inventory {
    /// Item uids, strictly ascending.
    pub uids: Vec<ItemUid>,
}

impl Canonical for Inventory {
    fn put(&self, out: &mut Vec<u8>) {
        self.uids.put(out);
    }
    fn read(r: &mut Reader<'_>) -> Result<Self, DecodeError> {
        let uids = Vec::<ItemUid>::read(r)?;
        if uids.windows(2).any(|w| w[0] >= w[1]) {
            return Err(DecodeError::NotAscending("inventory"));
        }
        Ok(Inventory { uids })
    }
}

/// SPEC: canon §2.4, doc 03 §3.3 `Player` (skeleton subset): the replay slot.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Player {
    /// Replay slot `0..=4`.
    pub slot: u8,
}

impl Canonical for Player {
    fn put(&self, out: &mut Vec<u8>) {
        self.slot.put(out);
    }
    fn read(r: &mut Reader<'_>) -> Result<Self, DecodeError> {
        let slot = u8::read(r)?;
        if usize::from(slot) >= SLOTS {
            return Err(DecodeError::Invalid("player slot"));
        }
        Ok(Player { slot })
    }
}

/// SPEC: canon §12, §2.3, doc 03 §3.3 `Pools` (skeleton subset): HP and
/// stamina in `Milli`, one canon §2.3 accumulator per (entity, rate), and the
/// last-event ticks the gated rates and `in_combat` read.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Pools {
    /// Current HP, mHP.
    pub hp: Milli,
    /// Max HP, mHP.
    pub hp_max: Milli,
    /// Current stamina, milli.
    pub stamina: Milli,
    /// Max stamina, milli.
    pub stamina_max: Milli,
    /// HP regeneration accumulator (`k = 60`).
    pub hp_regen: RateAcc,
    /// Stamina regeneration accumulator (`k = 60`).
    pub stamina_regen: RateAcc,
    /// Sprint drain accumulator (`k = 60`).
    pub sprint_drain: RateAcc,
    /// Last tick any damage was taken (gates HP regeneration).
    pub last_damage_taken: Option<Tick>,
    /// Last tick stamina was spent (gates stamina regeneration).
    pub last_stamina_spend: Option<Tick>,
    /// Non-DoT damage and alert ticks for `in_combat`.
    pub combat: CombatTicks,
}

impl Pools {
    /// SPEC: canon §12. Full pools of the given maxima.
    #[must_use]
    pub const fn full(hp_max: Milli, stamina_max: Milli) -> Pools {
        Pools {
            hp: hp_max,
            hp_max,
            stamina: stamina_max,
            stamina_max,
            hp_regen: RateAcc::ZERO,
            stamina_regen: RateAcc::ZERO,
            sprint_drain: RateAcc::ZERO,
            last_damage_taken: None,
            last_stamina_spend: None,
            combat: CombatTicks {
                dealt: None,
                taken: None,
                alerted: None,
            },
        }
    }
}

impl Canonical for Pools {
    fn put(&self, out: &mut Vec<u8>) {
        self.hp.put(out);
        self.hp_max.put(out);
        self.stamina.put(out);
        self.stamina_max.put(out);
        self.hp_regen.put(out);
        self.stamina_regen.put(out);
        self.sprint_drain.put(out);
        self.last_damage_taken.put(out);
        self.last_stamina_spend.put(out);
        self.combat.put(out);
    }
    fn read(r: &mut Reader<'_>) -> Result<Self, DecodeError> {
        Ok(Pools {
            hp: i64::read(r)?,
            hp_max: i64::read(r)?,
            stamina: i64::read(r)?,
            stamina_max: i64::read(r)?,
            hp_regen: RateAcc::read(r)?,
            stamina_regen: RateAcc::read(r)?,
            sprint_drain: RateAcc::read(r)?,
            last_damage_taken: Option::read(r)?,
            last_stamina_spend: Option::read(r)?,
            combat: CombatTicks::read(r)?,
        })
    }
}

/// SPEC: canon §1.8, §1.9, doc 03 §3.3 `Pos { p: Fx3, yaw: Bam, pitch: i16 }`.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Pos {
    /// Bottom-center of the collision capsule, meters (canon §1.8).
    pub p: Fx3,
    /// Facing yaw.
    pub yaw: Bam,
    /// Pitch, clamped to `-16000..=16000`.
    pub pitch: i16,
}

impl Canonical for Pos {
    fn put(&self, out: &mut Vec<u8>) {
        self.p.put(out);
        self.yaw.put(out);
        self.pitch.put(out);
    }
    fn read(r: &mut Reader<'_>) -> Result<Self, DecodeError> {
        Ok(Pos {
            p: Fx3::read(r)?,
            yaw: u16::read(r)?,
            pitch: i16::read(r)?,
        })
    }
}

/// SPEC: canon §2.1, doc 03 §3.3 `Vel { v: Fx3 }`.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Vel {
    /// Velocity, m/s.
    pub v: Fx3,
}

impl Canonical for Vel {
    fn put(&self, out: &mut Vec<u8>) {
        self.v.put(out);
    }
    fn read(r: &mut Reader<'_>) -> Result<Self, DecodeError> {
        Ok(Vel { v: Fx3::read(r)? })
    }
}

/// SPEC: canon §2.3, §11.3, doc 03 §3.3 `StatusInst` (skeleton subset): the
/// status `id32`, its timer, source, DoT rate and DoT accumulator (`k = 4`).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct StatusInst {
    /// Status `id32` (canon §1.1).
    pub id: u32,
    /// Applied tick and expiry.
    pub timer: StatusTimer,
    /// The applier.
    pub source: EntityId,
    /// Damage per second through the status pulse, mHP.
    pub dot_per_s: Milli,
    /// The status pulse accumulator.
    pub acc: RateAcc,
}

impl Canonical for StatusInst {
    fn put(&self, out: &mut Vec<u8>) {
        self.id.put(out);
        self.timer.put(out);
        self.source.put(out);
        self.dot_per_s.put(out);
        self.acc.put(out);
    }
    fn read(r: &mut Reader<'_>) -> Result<Self, DecodeError> {
        Ok(StatusInst {
            id: u32::read(r)?,
            timer: StatusTimer::read(r)?,
            source: EntityId::read(r)?,
            dot_per_s: i64::read(r)?,
            acc: RateAcc::read(r)?,
        })
    }
}

/// SPEC: canon §2.3, doc 03 §3.3 `Statuses { list }`, sorted by status `id`, at most one
/// instance per id.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Statuses {
    list: Vec<StatusInst>,
}

impl Statuses {
    /// SPEC: canon §2.3, doc 03 §3.3. The instances, ascending by `id`.
    #[must_use]
    pub fn list(&self) -> &[StatusInst] {
        &self.list
    }

    /// SPEC: canon §2.3, doc 03 §3.3. The instance of status `id`, if any.
    #[must_use]
    pub fn get(&self, id: u32) -> Option<&StatusInst> {
        self.list
            .binary_search_by_key(&id, |s| s.id)
            .ok()
            .map(|i| &self.list[i])
    }

    /// SPEC: canon §2.3. Apply status `id` during tick `t` for `d` ticks. A new
    /// status gets `applied = t`, `expiry = t + d`; an existing one is
    /// refreshed to `max(old_expiry, t + d)` and keeps its applier, rate and
    /// accumulator.
    pub fn apply(&mut self, id: u32, t: Tick, d: Ticks, source: EntityId, dot_per_s: Milli) {
        match self.list.binary_search_by_key(&id, |s| s.id) {
            Ok(i) => self.list[i].timer.refresh(t, d),
            Err(i) => self.list.insert(
                i,
                StatusInst {
                    id,
                    timer: StatusTimer::apply(t, d),
                    source,
                    dot_per_s,
                    acc: RateAcc::ZERO,
                },
            ),
        }
    }

    /// SPEC: canon §2.3. The status-timer step at the end of tick `t`: remove
    /// every status whose expiry is `t` or earlier.
    pub fn expire_end_of(&mut self, t: Tick) {
        self.list.retain(|s| !s.timer.expires_at_end_of(t));
    }
}

impl Canonical for Statuses {
    fn put(&self, out: &mut Vec<u8>) {
        self.list.put(out);
    }
    fn read(r: &mut Reader<'_>) -> Result<Self, DecodeError> {
        let list = Vec::<StatusInst>::read(r)?;
        if list.windows(2).any(|w| w[0].id >= w[1].id) {
            return Err(DecodeError::NotAscending("statuses"));
        }
        Ok(Statuses { list })
    }
}

/// SPEC: canon §2.5, doc 02 §5.2, doc 03 §3.3. The skeleton world: one column per
/// component, serialized in ascending component-ID byte order (`comp.control`,
/// `comp.inventory`, `comp.kind`, `comp.player`, `comp.pools`, `comp.pos`,
/// `comp.statuses`, `comp.vel`).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct World {
    /// `comp.control`.
    pub control: Column<Control>,
    /// `comp.inventory`.
    pub inventory: Column<Inventory>,
    /// `comp.kind`.
    pub kind: Column<EntityKind>,
    /// `comp.player`.
    pub player: Column<Player>,
    /// `comp.pools`.
    pub pools: Column<Pools>,
    /// `comp.pos`.
    pub pos: Column<Pos>,
    /// `comp.statuses`.
    pub statuses: Column<Statuses>,
    /// `comp.vel`.
    pub vel: Column<Vel>,
}

impl World {
    fn despawn(&mut self, id: EntityId) {
        self.control.remove(id);
        self.inventory.remove(id);
        self.kind.remove(id);
        self.player.remove(id);
        self.pools.remove(id);
        self.pos.remove(id);
        self.statuses.remove(id);
        self.vel.remove(id);
    }

    /// Writes every column in catalog order, keeping only entries `keep`
    /// accepts (all entries for the state serialization).
    fn put_filtered(&self, out: &mut Vec<u8>, keep: impl Fn(EntityId) -> bool + Copy) {
        fn col<T: Canonical>(out: &mut Vec<u8>, c: &Column<T>, keep: impl Fn(EntityId) -> bool) {
            let n = c.ids().iter().filter(|&&id| keep(id)).count();
            put_len(out, n);
            for (id, v) in c.iter().filter(|&(id, _)| keep(id)) {
                id.put(out);
                v.put(out);
            }
        }
        col(out, &self.control, keep);
        col(out, &self.inventory, keep);
        col(out, &self.kind, keep);
        col(out, &self.player, keep);
        col(out, &self.pools, keep);
        col(out, &self.pos, keep);
        col(out, &self.statuses, keep);
        col(out, &self.vel, keep);
    }
}

impl Canonical for World {
    fn put(&self, out: &mut Vec<u8>) {
        self.put_filtered(out, |_| true);
    }
    fn read(r: &mut Reader<'_>) -> Result<Self, DecodeError> {
        Ok(World {
            control: Column::read(r)?,
            inventory: Column::read(r)?,
            kind: Column::read(r)?,
            player: Column::read(r)?,
            pools: Column::read(r)?,
            pos: Column::read(r)?,
            statuses: Column::read(r)?,
            vel: Column::read(r)?,
        })
    }
}

// ---------------------------------------------------------------------------
// Sim
// ---------------------------------------------------------------------------

/// SPEC: canon §2.4, doc 02 §5.5 `SimConfig` (skeleton subset).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SimConfig {
    /// Canon §2.4 `content_hash` of the loaded content.
    pub content_hash: [u8; 32],
    /// Run mode `id32`.
    pub mode: u32,
    /// Party Errata spawned at run start, `1..=4` (slots `0..party_size`).
    pub party_size: u8,
    /// Spawn an invader in slot 4 at run start.
    pub invader: bool,
    /// Doc 02 §5.4 `HASH_AT_60`: return the state hash at boundaries ≡ 0 mod 60.
    pub hash_at_60: bool,
}

impl Default for SimConfig {
    fn default() -> Self {
        SimConfig {
            content_hash: [0; 32],
            mode: 0,
            party_size: 1,
            invader: false,
            hash_at_60: false,
        }
    }
}

/// SPEC: canon §2.4, doc 02 §5.5 `SimInitError`: why a run could not start.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ConfigError {
    /// `party_size` outside `1..=4`.
    PartySize(u8),
}

/// SPEC: canon §3.2, §1.11. Why a Folio entry was refused.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FolioError {
    /// Folio indices only grow within a run.
    NotIncreasing {
        /// The current Folio index.
        current: u32,
        /// The requested index.
        requested: u32,
    },
    /// The index does not fit the 24-bit `EntityId` field.
    IndexTooLarge(u32),
}

/// SPEC: canon §2.5. Why a save was refused.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LoadError {
    /// The bytes are malformed.
    Decode(DecodeError),
    /// The save was written by another `SIM_VERSION`.
    SimVersion {
        /// The version in the save.
        found: u32,
        /// This build's version.
        expected: u32,
    },
    /// The save's `content_hash` differs from the loaded content.
    ContentHash,
    /// The header and meta section disagree on the run seed.
    RunSeed,
    /// The save's mode differs from the config's.
    Mode,
    /// The state breaks a structural invariant.
    Inconsistent(&'static str),
    /// The bytes decode but do not re-serialize identically.
    NonCanonical,
}

impl From<DecodeError> for LoadError {
    fn from(e: DecodeError) -> Self {
        LoadError::Decode(e)
    }
}

/// SPEC: canon §3.2, test 31. A Folio's genre inputs: its genre `id32` and the
/// bleed-neighbor genre `id32`s.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct FolioInputs {
    /// Genre `id32`.
    pub genre: u32,
    /// Bleed-neighbor genre `id32`s, in the order the generator supplies them.
    pub bleed: Vec<u32>,
}

/// SPEC: canon §3.2, doc 02 §6.5 section 0. The current Folio.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct FolioMeta {
    /// Folio index; 0 is run start and the hub.
    pub index: u32,
    /// Doc 03 §2.1 `folio_entry_tick`: the first tick simulated inside it.
    pub entry_tick: Tick,
    /// The genre inputs the Folio was generated from.
    pub inputs: FolioInputs,
    /// The generated spawn point.
    pub spawn_point: Fx3,
}

/// SPEC: canon §2.4, doc 02 §6.5 section 10. Per-slot input state: the
/// occupying entity, previous buttons and last `action_seq`.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct SlotState {
    /// The slot's entity, if occupied.
    pub entity: Option<EntityId>,
    /// Buttons of the last applied command (presses are derived from it).
    pub prev_buttons: u32,
    /// `action_seq` of the last applied command.
    pub action_seq: u32,
}

/// SPEC: canon §2.5, doc 02 §5.4 `StepOutput` (skeleton subset).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct StepOutput {
    /// The tick just simulated.
    pub tick: Tick,
    /// The state hash at the new boundary when `hash_at_60` is set and the
    /// boundary is a multiple of 60.
    pub hash: Option<StateHash>,
}

/// Per-tick scratch: derived presses, per-actor keyed `seq` counters and the
/// sprinting set. Reset every tick, so never part of saved state.
#[derive(Default)]
struct TickCtx {
    presses: [u32; SLOTS],
    seq: BTreeMap<EntityId, u32>,
    sprinting: BTreeSet<EntityId>,
}

impl TickCtx {
    /// Canon §3.3 (canon-1.11): a running per-actor counter is allowed only
    /// inside a range its document reserves; the skeleton's hit draws use
    /// doc 05 §1.4's action and hit slots `0x0000..=0x01FF` on `rng.combat`
    /// and `rng.status`, reset each tick. Panics past the range.
    fn next_seq(&mut self, actor: EntityId) -> u32 {
        let s = self.seq.entry(actor).or_insert(0);
        let v = *s;
        assert!(
            seq::ACTION_HIT_SLOTS.contains(&v),
            "invariant: keyed seq outside the reserved action and hit slots"
        );
        *s += 1;
        v
    }
}

/// SPEC: canon §2.5 (canon-1.11). When a save request completes.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum SaveTiming {
    /// Single player: at any tick boundary.
    AnyBoundary,
    /// Single player while replay slot 4 holds an invader: the request
    /// completes on the tick `ev.invasion_ended` is emitted.
    AfterInvasion,
    /// Co-op: only at Folio entry.
    FolioEntryOnly,
}

/// SPEC: canon §2, doc 02 §5.5. The deterministic simulation skeleton.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sim {
    config: SimConfig,
    run_seed: u64,
    tick: Tick,
    folio: FolioMeta,
    entities: EntityAllocator,
    uids: UidAllocator,
    streams: BTreeMap<String, Rng>,
    world: World,
    slots: [SlotState; SLOTS],
}

impl Sim {
    /// SPEC: canon §2.4, §1.11, doc 02 §5.5 `Sim::new`. A run at tick 0 in the
    /// hub (`folio_index = 0`) with `party_size` Errata in slots
    /// `0..party_size` and, if configured, an invader in slot 4.
    pub fn new(run_seed: u64, config: SimConfig) -> Result<Sim, ConfigError> {
        if config.party_size == 0 || config.party_size > MAX_PARTY {
            return Err(ConfigError::PartySize(config.party_size));
        }
        let mut sim = Sim {
            config,
            run_seed,
            tick: 0,
            folio: FolioMeta {
                spawn_point: HUB_SPAWN,
                ..FolioMeta::default()
            },
            entities: EntityAllocator::new(0),
            uids: UidAllocator::new(0),
            streams: BTreeMap::new(),
            world: World::default(),
            slots: [SlotState::default(); SLOTS],
        };
        for slot in 0..config.party_size {
            sim.spawn_slot_entity(slot, EntityKind::Erratum);
        }
        if config.invader {
            sim.spawn_slot_entity(INVADER_SLOT, EntityKind::Creature);
        }
        Ok(sim)
    }

    fn slot_offset(slot: u8) -> Fx3 {
        Fx3::from_ints(i32::from(slot), 0, 0)
    }

    fn spawn_slot_entity(&mut self, slot: u8, kind: EntityKind) {
        let id = self.entities.alloc();
        let w = &mut self.world;
        w.control.insert(id, Control::default());
        w.kind.insert(id, kind);
        w.player.insert(id, Player { slot });
        w.pools.insert(id, Pools::full(ERRATUM_HP, ERRATUM_STAMINA));
        w.pos.insert(
            id,
            Pos {
                p: self.folio.spawn_point.add(Self::slot_offset(slot)),
                yaw: 0,
                pitch: 0,
            },
        );
        w.statuses.insert(id, Statuses::default());
        w.vel.insert(id, Vel::default());
        self.slots[usize::from(slot)].entity = Some(id);
    }

    /// SPEC: canon §2.1, doc 02 §5.4. The boundary: the next tick to simulate.
    #[must_use]
    pub const fn tick(&self) -> Tick {
        self.tick
    }

    /// SPEC: canon §2.4. The run seed.
    #[must_use]
    pub const fn run_seed(&self) -> u64 {
        self.run_seed
    }

    /// SPEC: canon §2.4, doc 02 §5.5. The configuration.
    #[must_use]
    pub const fn config(&self) -> &SimConfig {
        &self.config
    }

    /// SPEC: canon §3.2, doc 02 §6.5 section 0. The current Folio.
    #[must_use]
    pub const fn folio(&self) -> &FolioMeta {
        &self.folio
    }

    /// SPEC: canon §1.11, doc 02 §5.2. Read access to the column store.
    #[must_use]
    pub const fn world(&self) -> &World {
        &self.world
    }

    /// SPEC: canon §2.4. The entity in replay slot `slot`, if any.
    #[must_use]
    pub fn slot_entity(&self, slot: u8) -> Option<EntityId> {
        self.slots.get(usize::from(slot)).and_then(|s| s.entity)
    }

    /// SPEC: canon §1.11, §2.5. The next `EntityId` and item uid counters.
    #[must_use]
    pub const fn counters(&self) -> (IdCounter, IdCounter) {
        (self.entities.counter(), self.uids.counter())
    }

    /// SPEC: canon §3.2, §2.5. The state of sequential stream `name`, if live.
    #[must_use]
    pub fn stream_state(&self, name: &str) -> Option<[u64; 4]> {
        self.streams.get(name).map(Rng::state)
    }

    /// SPEC: canon §2.3 `in_combat(e, t)` for entity `id` at tick `t`;
    /// `None` when `id` has no pools.
    #[must_use]
    pub fn in_combat(&self, id: EntityId, t: Tick) -> Option<bool> {
        self.world.pools.get(id).map(|p| p.combat.in_combat(t))
    }

    fn stream(&mut self, name: &str) -> &mut Rng {
        self.streams
            .get_mut(name)
            .expect("invariant: per-Folio stream derived at Folio entry")
    }

    // -- Folio entry --------------------------------------------------------

    /// SPEC: canon §1.11, §3.2, doc 03 §2.1. Enter Folio `index` at the
    /// current boundary. Every entity not held by a replay slot is removed;
    /// the `EntityId` and uid counters restart at 1; `rng.worldgen`,
    /// `rng.spawn` and `rng.ai_idle` are derived fresh as
    /// `stream_seed(run_seed, name, index)`; the Folio is generated from them
    /// and `inputs` alone; slot entities move to the new spawn point with zero
    /// velocity and keep their IDs.
    pub fn enter_folio(&mut self, index: u32, inputs: FolioInputs) -> Result<(), FolioError> {
        if index > MAX_FOLIO_INDEX {
            return Err(FolioError::IndexTooLarge(index));
        }
        if index <= self.folio.index {
            return Err(FolioError::NotIncreasing {
                current: self.folio.index,
                requested: index,
            });
        }
        let leaving: Vec<EntityId> = self
            .world
            .kind
            .ids()
            .iter()
            .copied()
            .filter(|&id| !self.world.player.contains(id))
            .collect();
        for id in leaving {
            self.world.despawn(id);
        }
        self.entities.enter_folio(index);
        self.uids.enter_folio(index);
        for name in FOLIO_STREAMS {
            self.streams
                .insert(String::from(name), stream_rng(self.run_seed, name, index));
        }
        let spawn_point = self.generate_folio();
        self.folio = FolioMeta {
            index,
            entry_tick: self.tick,
            inputs,
            spawn_point,
        };
        for (s, st) in self.slots.iter().enumerate() {
            if let Some(id) = st.entity {
                let off = Self::slot_offset(u8::try_from(s).expect("slot fits u8"));
                if let Some(p) = self.world.pos.get_mut(id) {
                    p.p = spawn_point.add(off);
                }
                if let Some(v) = self.world.vel.get_mut(id) {
                    v.v = Fx3::ZERO;
                }
            }
        }
        Ok(())
    }

    /// Skeleton Folio generation from the freshly derived per-Folio streams:
    /// a spawn point from `rng.worldgen`, then 3 to 6 creatures around it from
    /// `rng.spawn`, each holding one newly allocated item uid. Returns the
    /// spawn point.
    fn generate_folio(&mut self) -> Fx3 {
        let wg = self.stream(streams::WORLDGEN);
        let sx = 32_000 + 1000 * wg.range(192);
        let sz = 32_000 + 1000 * wg.range(192);
        let spawn_point = Fx3::from_milli(to_i64(sx), 0, to_i64(sz));
        let n = 3 + self.stream(streams::SPAWN).range(4);
        for _ in 0..n {
            let sp = self.stream(streams::SPAWN);
            let dx = sp.range_incl(-8000, 8000);
            let dz = sp.range_incl(-8000, 8000);
            let id = self.entities.alloc();
            let uid = self.uids.alloc();
            let w = &mut self.world;
            w.control.insert(id, Control::default());
            w.inventory.insert(id, Inventory { uids: vec![uid] });
            w.kind.insert(id, EntityKind::Creature);
            w.pools.insert(id, Pools::full(CREATURE_HP, 0));
            w.pos.insert(
                id,
                Pos {
                    p: spawn_point.add(Fx3::from_milli(dx, 0, dz)),
                    yaw: 0,
                    pitch: 0,
                },
            );
            w.statuses.insert(id, Statuses::default());
            w.vel.insert(id, Vel::default());
        }
        spawn_point
    }

    // -- Step ---------------------------------------------------------------

    /// SPEC: canon §2.4, doc 02 §5.4. Validate `commands` for the current
    /// tick and return them in `(tick, slot)` order. Nothing changes on error.
    fn order_commands(
        &self,
        commands: &[PlayerCommand],
    ) -> Result<Vec<PlayerCommand>, CommandError> {
        let mut seen = [false; SLOTS];
        for c in commands {
            if c.tick != self.tick {
                return Err(CommandError::WrongTick {
                    expected: self.tick,
                    found: c.tick,
                });
            }
            c.validate()?;
            let s = usize::from(c.slot);
            if seen[s] {
                return Err(CommandError::DuplicateSlot(c.slot));
            }
            if self.slots[s].entity.is_none() {
                return Err(CommandError::EmptySlot(c.slot));
            }
            seen[s] = true;
        }
        let mut v = commands.to_vec();
        v.sort_by_key(|c| (c.tick, c.slot));
        Ok(v)
    }

    /// SPEC: canon §2.1-2.4, §11.8, doc 02 §5.4, doc 03 §2.2. Simulate the
    /// current tick `t` with the host's applied commands for it (at most one
    /// per slot; an occupied slot without a command gets canon §11.8's neutral
    /// substitute, which keeps only the stance and quickbar bits of
    /// [`buttons::HOLD_MASK`] and never moves or presses; a host that logs the
    /// replay passes [`neutral_substitute`] explicitly, with the same result),
    /// then advance the boundary. Systems run in doc 03 order: input apply,
    /// AI think, actions, movement, status pulse, resources, status timers,
    /// despawn.
    pub fn step(&mut self, commands: &[PlayerCommand]) -> Result<StepOutput, CommandError> {
        let ordered = self.order_commands(commands)?;
        let t = self.tick;
        let mut ctx = TickCtx::default();
        self.sys_input_apply(&ordered, &mut ctx);
        self.sys_ai_think(t);
        self.sys_actions(t, &mut ctx);
        self.sys_movement(&mut ctx);
        self.sys_status_pulse(t);
        self.sys_resources(t, &ctx);
        self.sys_status_timers(t);
        self.sys_despawn();
        self.tick = boundary_after(t);
        let hash = if self.config.hash_at_60 && is_hash_boundary(self.tick) {
            Some(self.state_hash())
        } else {
            None
        };
        Ok(StepOutput { tick: t, hash })
    }

    /// Doc 03 §2.2 #4: copy each slot's command in slot order, clamp pitch,
    /// map `-128` to `-127`, derive presses against the previous buttons.
    /// Canon §11.8 (canon-1.11): an occupied slot without a command this tick
    /// gets the neutral substitute ([`neutral_substitute`]): no movement,
    /// action buttons released, stance and quickbar index kept, yaw and pitch
    /// unchanged, no press edge.
    fn sys_input_apply(&mut self, ordered: &[PlayerCommand], ctx: &mut TickCtx) {
        for s in 0..SLOTS {
            let slot = u8::try_from(s).expect("slot fits u8");
            if self.slots[s].entity.is_none() || ordered.iter().any(|c| c.slot == slot) {
                continue;
            }
            let id = self.slots[s].entity.expect("occupied");
            let held = self.slots[s].prev_buttons & buttons::HOLD_MASK;
            if let Some(ctrl) = self.world.control.get_mut(id) {
                ctrl.move_x = 0;
                ctrl.move_z = 0;
                ctrl.buttons = held;
            }
            ctx.presses[s] = 0;
            self.slots[s].prev_buttons = held;
        }
        for c in ordered {
            let s = usize::from(c.slot);
            let Some(id) = self.slots[s].entity else {
                continue;
            };
            if let Some(ctrl) = self.world.control.get_mut(id) {
                ctrl.move_x = c.move_x.max(-127);
                ctrl.move_z = c.move_z.max(-127);
                ctrl.buttons = c.buttons;
            }
            if let Some(p) = self.world.pos.get_mut(id) {
                p.yaw = c.yaw;
                p.pitch = clamp_pitch(c.pitch);
            }
            ctx.presses[s] = c.buttons & !self.slots[s].prev_buttons;
            self.slots[s].prev_buttons = c.buttons;
            self.slots[s].action_seq = c.action_seq;
        }
    }

    /// Doc 03 §2.2 #6, canon §2.2 AI think (period 6, per-entity phase):
    /// creatures not held by a slot pick a wander direction from the
    /// sequential `rng.ai_idle` stream, ascending `EntityId`.
    fn sys_ai_think(&mut self, t: Tick) {
        let agents: Vec<EntityId> = self
            .world
            .kind
            .iter()
            .filter(|&(id, k)| *k == EntityKind::Creature && !self.world.player.contains(id))
            .map(|(id, _)| id)
            .filter(|&id| runs_on_entity(Subsystem::AiThink, t, id))
            .collect();
        for id in agents {
            let choice = self.stream(streams::AI_IDLE).range(5);
            let (mx, mz) = match choice {
                1 => (127, 0),
                2 => (-127, 0),
                3 => (0, 127),
                4 => (0, -127),
                _ => (0, 0),
            };
            if let Some(c) = self.world.control.get_mut(id) {
                c.move_x = mx;
                c.move_z = mz;
            }
        }
    }

    /// Melee target for `actor` at `from`: the living entity of the other
    /// kind with minimum `dist2` within reach, ties to the lower `EntityId`.
    fn melee_target(&self, actor: EntityId, from: Fx3) -> Option<EntityId> {
        let actor_kind = *self.world.kind.get(actor)?;
        let reach = i128::from(MELEE_RANGE.raw()) * i128::from(MELEE_RANGE.raw());
        let mut best: Option<(i128, EntityId)> = None;
        for (id, pos) in self.world.pos.iter() {
            if id == actor || self.world.kind.get(id) == Some(&actor_kind) {
                continue;
            }
            if self.world.pools.get(id).is_none_or(|p| p.hp <= 0) {
                continue;
            }
            let d2 = dist2(from, pos.p);
            if d2 <= reach && best.is_none_or(|b| (d2, id) < b) {
                best = Some((d2, id));
            }
        }
        best.map(|(_, id)| id)
    }

    /// Doc 03 §2.2 #7 (skeleton): an `ATTACK_PRIMARY` press hits the nearest
    /// enemy in reach. Each hit draws exactly one crit roll from keyed
    /// `rng.combat` and one apply roll from keyed `rng.status` (canon §3.3,
    /// §3.4), actors in ascending `EntityId`.
    fn sys_actions(&mut self, t: Tick, ctx: &mut TickCtx) {
        let mut actors: Vec<(EntityId, u32)> = (0..SLOTS)
            .filter_map(|s| self.slots[s].entity.map(|e| (e, ctx.presses[s])))
            .collect();
        actors.sort_by_key(|&(e, _)| e);
        for (actor, presses) in actors {
            if presses & buttons::ATTACK_PRIMARY == 0 {
                continue;
            }
            if self.world.pools.get(actor).is_none_or(|p| p.hp <= 0) {
                continue;
            }
            let Some(from) = self.world.pos.get(actor).map(|p| p.p) else {
                continue;
            };
            let Some(target) = self.melee_target(actor, from) else {
                continue;
            };
            let seq = ctx.next_seq(actor);
            let f = self.folio.index;
            let crit = keyed_rng(self.run_seed, streams::COMBAT, f, actor.raw(), t, seq)
                .roll_bp(CRIT_CHANCE_BP);
            let bleed = keyed_rng(self.run_seed, streams::STATUS, f, actor.raw(), t, seq)
                .roll_bp(BLEED_CHANCE_BP);
            let dmg = if crit {
                mul_bp(HIT_DAMAGE, CRIT_MULT_BP)
            } else {
                HIT_DAMAGE
            };
            if let Some(p) = self.world.pools.get_mut(target) {
                p.hp = p.hp.checked_sub(dmg).expect("hp underflow");
                p.last_damage_taken = Some(t);
                p.combat.taken = Some(t);
            }
            if let Some(p) = self.world.pools.get_mut(actor) {
                p.combat.dealt = Some(t);
            }
            if bleed && let Some(s) = self.world.statuses.get_mut(target) {
                s.apply(ST_BLEEDING, t, BLEED_TICKS, actor, BLEED_DOT);
            }
        }
    }

    /// Doc 03 §2.2 #8 (skeleton mover): relax the horizontal velocity toward
    /// the wish velocity, apply gravity and a `JUMP` press, integrate with
    /// semi-implicit Euler (canon §2.1), then clamp to the floor `y = 0` and
    /// the Folio box. Ascending `EntityId`.
    fn sys_movement(&mut self, ctx: &mut TickCtx) {
        let ids: Vec<EntityId> = self.world.control.ids().to_vec();
        for id in ids {
            let (Some(&ctrl), Some(&pos), Some(&vel)) = (
                self.world.control.get(id),
                self.world.pos.get(id),
                self.world.vel.get(id),
            ) else {
                continue;
            };
            let pools = self.world.pools.get(id).copied().unwrap_or_default();
            let slot = self.world.player.get(id).map(|p| usize::from(p.slot));
            let moving = ctrl.move_x != 0 || ctrl.move_z != 0;
            let speed = match slot {
                _ if pools.hp <= 0 => Fx::ZERO,
                Some(_) if ctrl.buttons & buttons::SPRINT != 0 && moving && pools.stamina > 0 => {
                    ctx.sprinting.insert(id);
                    SPRINT
                }
                Some(_) => WALK,
                None => CREATURE_SPEED,
            };
            let (tx, tz) = wish_velocity(ctrl.move_x, ctrl.move_z, pos.yaw, speed);
            let mut v = vel.v;
            let grounded = pos.p.y == Fx::ZERO && v.y == Fx::ZERO;
            let jump = slot.is_some_and(|s| ctx.presses[s] & buttons::JUMP != 0);
            if jump && grounded && pools.hp > 0 {
                v.y = JUMP_SPEED;
            }
            let a = Fx3::new(
                (tx - v.x).mul_int(ACCEL_PER_S),
                -GRAVITY,
                (tz - v.z).mul_int(ACCEL_PER_S),
            );
            let (mut p2, mut v2) = semi_implicit_euler(pos.p, v, a);
            clamp_axis(&mut p2.x, &mut v2.x, BOX_MAX);
            clamp_axis(&mut p2.z, &mut v2.z, BOX_MAX);
            if p2.y < Fx::ZERO {
                p2.y = Fx::ZERO;
                v2.y = Fx::ZERO;
            }
            if let Some(p) = self.world.pos.get_mut(id) {
                p.p = p2;
            }
            if let Some(vv) = self.world.vel.get_mut(id) {
                vv.v = v2;
            }
        }
    }

    /// Doc 03 §2.2 #25, canon §11.3: on status-pulse ticks every active DoT
    /// applies `q` from its accumulator with `k = 4`. Ascending `EntityId`,
    /// statuses in ascending `id`.
    fn sys_status_pulse(&mut self, t: Tick) {
        if !runs_on(Subsystem::StatusPulse, t, 0) {
            return;
        }
        let World {
            statuses, pools, ..
        } = &mut self.world;
        let ids: Vec<EntityId> = statuses.ids().to_vec();
        for id in ids {
            let Some(st) = statuses.get_mut(id) else {
                continue;
            };
            let mut dmg: Milli = 0;
            for s in &mut st.list {
                if s.dot_per_s != 0 && s.timer.is_active(t) {
                    let q = s.acc.step(s.dot_per_s, K_STATUS_PULSE);
                    dmg = dmg.checked_add(q).expect("dot overflow");
                }
            }
            if dmg != 0
                && let Some(p) = pools.get_mut(id)
            {
                p.hp = p.hp.checked_sub(dmg).expect("hp underflow");
                if dmg > 0 {
                    p.last_damage_taken = Some(t);
                }
            }
        }
    }

    /// Doc 03 §2.2 #26a, canon §6.4 rows 2, 5, 6 with the §2.3 accumulator at
    /// `k = 60`: sprint drain, stamina regeneration after 1 s without spending,
    /// HP regeneration after 5 s without damage. Ascending `EntityId`.
    fn sys_resources(&mut self, t: Tick, ctx: &TickCtx) {
        let ids: Vec<EntityId> = self.world.pools.ids().to_vec();
        for id in ids {
            let Some(p) = self.world.pools.get_mut(id) else {
                continue;
            };
            if ctx.sprinting.contains(&id) {
                let q = p.sprint_drain.step(SPRINT_DRAIN, K_PER_TICK);
                p.stamina = p.stamina.checked_sub(q).expect("stamina underflow").max(0);
                p.last_stamina_spend = Some(t);
            } else {
                p.sprint_drain.reset();
            }
            let q = p.stamina_regen.step_after_without(
                STAMINA_REGEN,
                K_PER_TICK,
                t,
                p.last_stamina_spend,
                STAMINA_REGEN_DELAY_S,
            );
            p.stamina = p
                .stamina
                .checked_add(q)
                .expect("stamina overflow")
                .min(p.stamina_max);
            let rate = mul_bp(p.hp_max, HP_REGEN_BP);
            let alive = p.hp > 0;
            let regen_on = alive && after_secs_without(t, p.last_damage_taken, HP_REGEN_DELAY_S);
            let q = p.hp_regen.step_gated(rate, K_PER_TICK, regen_on);
            p.hp = p.hp.checked_add(q).expect("hp overflow").min(p.hp_max);
        }
    }

    /// Doc 03 §2.2 #33, canon §2.3: end-of-tick status expiry.
    fn sys_status_timers(&mut self, t: Tick) {
        let ids: Vec<EntityId> = self.world.statuses.ids().to_vec();
        for id in ids {
            if let Some(s) = self.world.statuses.get_mut(id) {
                s.expire_end_of(t);
            }
        }
    }

    /// Doc 03 §2.2 #34 (skeleton): creatures not held by a slot despawn at
    /// HP ≤ 0, ascending `EntityId`; slot entities floor at 0 HP (downed).
    fn sys_despawn(&mut self) {
        let mut dead = Vec::new();
        for (id, p) in self.world.pools.iter() {
            if p.hp <= 0 && !self.world.player.contains(id) {
                dead.push(id);
            }
        }
        for id in dead {
            self.world.despawn(id);
        }
        let ids: Vec<EntityId> = self.world.player.ids().to_vec();
        for id in ids {
            if let Some(p) = self.world.pools.get_mut(id) {
                p.hp = p.hp.max(0);
            }
        }
    }

    // -- Serialization -------------------------------------------------------

    fn put_meta(&self, out: &mut Vec<u8>) {
        SIM_VERSION.put(out);
        self.tick.put(out);
        self.run_seed.put(out);
        self.config.mode.put(out);
        self.folio.index.put(out);
        self.folio.entry_tick.put(out);
        self.entities.counter().next_counter().put(out);
        self.uids.counter().next_counter().put(out);
        for s in &self.slots {
            s.entity.put(out);
        }
        self.folio.inputs.genre.put(out);
        self.folio.inputs.bleed.put(out);
        self.folio.spawn_point.put(out);
    }

    fn put_rng(&self, out: &mut Vec<u8>, keep: impl Fn(&str) -> bool) {
        let n = self.streams.keys().filter(|k| keep(k)).count();
        put_len(out, n);
        for (name, rng) in self.streams.iter().filter(|(k, _)| keep(k)) {
            name.put(out);
            rng.put(out);
        }
    }

    fn put_inputs(&self, out: &mut Vec<u8>) {
        for s in &self.slots {
            s.prev_buttons.put(out);
            s.action_seq.put(out);
        }
    }

    fn section(&self, k: usize) -> Vec<u8> {
        let mut out = Vec::new();
        match k {
            0 => self.put_meta(&mut out),
            1 => self.put_rng(&mut out, |_| true),
            2 => self.world.put(&mut out),
            10 => self.put_inputs(&mut out),
            // Sections 3-9, 11 and 12 (items, blocks, light, fluids, thermal,
            // ai, run, mods, stealth) hold no skeleton state.
            _ => {}
        }
        out
    }

    /// SPEC: canon §2.5, doc 02 §6.5. The canonical serialization:
    /// `for k in 0..13: u8(k) ‖ u32le(len(S_k)) ‖ S_k`. Section 0 `meta`:
    /// `SIM_VERSION u32`, boundary tick `u64`, run seed `u64`, mode `u32`,
    /// Folio index `u32`, `folio_entry_tick u64`, next entity counter `u64`,
    /// next uid counter `u64`, slots `0..=4` as `Option<EntityId>`, genre
    /// `u32`, bleed `Vec<u32>`, spawn point `Fx3`. Section 1 `rng`: streams
    /// ascending by name, each `(name, [u64; 4])`. Section 2 `entities`: the
    /// columns of [`World`] in catalog order, each a `u32` count of
    /// `(EntityId, value)` pairs ascending. Section 10 `inputs`: per slot
    /// `prev_buttons u32`, `action_seq u32`. Other sections are empty.
    /// Integers are little-endian, options are a `0`/`1` tag then the value,
    /// lists are a `u32` count then the items.
    #[must_use]
    pub fn serialize(&self) -> Vec<u8> {
        let mut out = Vec::new();
        for k in 0..SECTIONS {
            let s = self.section(k);
            out.push(u8::try_from(k).expect("section index fits u8"));
            put_len(&mut out, s.len());
            out.extend_from_slice(&s);
        }
        out
    }

    /// SPEC: canon §2.5 `state_hash = BLAKE3(serialization)`.
    #[must_use]
    pub fn state_hash(&self) -> StateHash {
        blake3_256(&[&self.serialize()])
    }

    /// SPEC: canon §2.5, doc 02 §6.5 `section_hash = BLAKE3(S_k)`, for desync
    /// localization only.
    #[must_use]
    pub fn section_hashes(&self) -> [StateHash; SECTIONS] {
        core::array::from_fn(|k| blake3_256(&[&self.section(k)]))
    }

    /// SPEC: canon §2.4. This run's session header.
    #[must_use]
    pub const fn header(&self) -> SessionHeader {
        SessionHeader {
            sim_version: SIM_VERSION,
            content_hash: self.config.content_hash,
            run_seed: self.run_seed,
        }
    }

    /// SPEC: canon §2.5 (canon-1.11). When a save request may complete:
    /// co-op (party size above 1) saves only at Folio entry; single player
    /// saves at any tick boundary, except while replay slot 4 holds an
    /// invader, when the request completes on the tick `ev.invasion_ended`
    /// is emitted. [`Sim::save`] itself serializes whenever it is called.
    #[must_use]
    pub fn save_timing(&self) -> SaveTiming {
        if self.config.party_size > 1 {
            SaveTiming::FolioEntryOnly
        } else if self.slots[usize::from(INVADER_SLOT)].entity.is_some() {
            SaveTiming::AfterInvasion
        } else {
            SaveTiming::AnyBoundary
        }
    }

    /// SPEC: canon §2.5. A save: the §2.4 header followed by the canonical
    /// serialization, taken at the current tick boundary.
    #[must_use]
    pub fn save(&self) -> Vec<u8> {
        let mut out = Vec::new();
        self.header().put(&mut out);
        out.extend_from_slice(&self.serialize());
        out
    }

    /// SPEC: canon §2.5, doc 02 §5.5 `deserialize`. Rebuild a run from a save.
    /// The header must carry this build's `SIM_VERSION` and `config`'s
    /// `content_hash`; the mode must match; the state must satisfy the column
    /// and counter invariants and re-serialize to exactly the saved bytes.
    /// Afterwards `state_hash()` equals the saved run's.
    pub fn restore(save: &[u8], config: SimConfig) -> Result<Sim, LoadError> {
        let mut r = Reader::new(save);
        let header = SessionHeader::read(&mut r)?;
        if header.sim_version != SIM_VERSION {
            return Err(LoadError::SimVersion {
                found: header.sim_version,
                expected: SIM_VERSION,
            });
        }
        if header.content_hash != config.content_hash {
            return Err(LoadError::ContentHash);
        }
        let body = r.bytes(r.remaining())?;
        let sim = Self::deserialize(body, config)?;
        if sim.run_seed != header.run_seed {
            return Err(LoadError::RunSeed);
        }
        Ok(sim)
    }

    /// SPEC: canon §2.5. Decode a canonical serialization (no header).
    pub fn deserialize(bytes: &[u8], config: SimConfig) -> Result<Sim, LoadError> {
        let mut r = Reader::new(bytes);
        let mut sections: [&[u8]; SECTIONS] = [&[]; SECTIONS];
        for (k, slot) in sections.iter_mut().enumerate() {
            let expected = u8::try_from(k).expect("section index fits u8");
            let found = u8::read(&mut r)?;
            if found != expected {
                return Err(DecodeError::BadSection { expected, found }.into());
            }
            let n = r.len_u32()?;
            *slot = r.bytes(n)?;
        }
        r.finish()?;
        for (k, s) in sections.iter().enumerate() {
            if !matches!(k, 0 | 1 | 2 | 10) && !s.is_empty() {
                return Err(DecodeError::Invalid("section must be empty").into());
            }
        }
        let mut sim = Self::read_meta(sections[0], config)?;
        sim.streams = Self::read_rng(sections[1])?;
        let mut rw = Reader::new(sections[2]);
        sim.world = World::read(&mut rw)?;
        rw.finish()?;
        let mut ri = Reader::new(sections[10]);
        for s in &mut sim.slots {
            s.prev_buttons = u32::read(&mut ri)?;
            s.action_seq = u32::read(&mut ri)?;
        }
        ri.finish()?;
        sim.check_invariants().map_err(LoadError::Inconsistent)?;
        if sim.serialize() != bytes {
            return Err(LoadError::NonCanonical);
        }
        Ok(sim)
    }

    fn read_meta(bytes: &[u8], config: SimConfig) -> Result<Sim, LoadError> {
        let mut r = Reader::new(bytes);
        let version = u32::read(&mut r)?;
        if version != SIM_VERSION {
            return Err(LoadError::SimVersion {
                found: version,
                expected: SIM_VERSION,
            });
        }
        let tick = u64::read(&mut r)?;
        let run_seed = u64::read(&mut r)?;
        let mode = u32::read(&mut r)?;
        if mode != config.mode {
            return Err(LoadError::Mode);
        }
        let index = u32::read(&mut r)?;
        let entry_tick = u64::read(&mut r)?;
        let next_entity = u64::read(&mut r)?;
        let next_uid = u64::read(&mut r)?;
        let mut slots = [SlotState::default(); SLOTS];
        for s in &mut slots {
            s.entity = Option::<EntityId>::read(&mut r)?;
        }
        let genre = u32::read(&mut r)?;
        let bleed = Vec::<u32>::read(&mut r)?;
        let spawn_point = Fx3::read(&mut r)?;
        r.finish()?;
        let ec = IdCounter::from_parts(index, next_entity)
            .map_err(|_| LoadError::Inconsistent("entity counter"))?;
        let uc = IdCounter::from_parts(index, next_uid)
            .map_err(|_| LoadError::Inconsistent("uid counter"))?;
        Ok(Sim {
            config,
            run_seed,
            tick,
            folio: FolioMeta {
                index,
                entry_tick,
                inputs: FolioInputs { genre, bleed },
                spawn_point,
            },
            entities: EntityAllocator::from_counter(ec),
            uids: UidAllocator::from_counter(uc),
            streams: BTreeMap::new(),
            world: World::default(),
            slots,
        })
    }

    fn read_rng(bytes: &[u8]) -> Result<BTreeMap<String, Rng>, DecodeError> {
        let mut r = Reader::new(bytes);
        let n = r.len_u32()?;
        let mut m = BTreeMap::new();
        let mut last: Option<String> = None;
        for _ in 0..n {
            let name = String::read(&mut r)?;
            if last.as_ref().is_some_and(|l| name <= *l) {
                return Err(DecodeError::NotAscending("rng streams"));
            }
            if !name.starts_with("rng.") {
                return Err(DecodeError::Invalid("stream name"));
            }
            let rng = Rng::read(&mut r)?;
            m.insert(name.clone(), rng);
            last = Some(name);
        }
        r.finish()?;
        Ok(m)
    }

    /// Structural invariants of a decoded state: every entity has the full
    /// skeleton component set; player and inventory entries belong to known
    /// entities; IDs and uids lie inside the allocators' ranges; slots and
    /// player components agree.
    fn check_invariants(&self) -> Result<(), &'static str> {
        let w = &self.world;
        let ids = w.kind.ids();
        if w.control.ids() != ids
            || w.pools.ids() != ids
            || w.pos.ids() != ids
            || w.statuses.ids() != ids
            || w.vel.ids() != ids
        {
            return Err("component columns disagree with comp.kind");
        }
        if !w.player.ids().iter().all(|&id| w.kind.contains(id))
            || !w.inventory.ids().iter().all(|&id| w.kind.contains(id))
        {
            return Err("component without comp.kind");
        }
        let f = self.folio.index;
        let next_e = self.entities.counter().next_counter();
        let next_u = self.uids.counter().next_counter();
        let id_ok = |folio: u32, counter: u64, next: u64| {
            counter >= 1 && (folio < f || (folio == f && counter < next))
        };
        if !ids
            .iter()
            .all(|id| id_ok(id.folio_index(), id.counter(), next_e))
        {
            return Err("entity id outside the allocated range");
        }
        for (_, inv) in w.inventory.iter() {
            if !inv
                .uids
                .iter()
                .all(|u| id_ok(u.folio_index(), u.counter(), next_u))
            {
                return Err("item uid outside the allocated range");
            }
        }
        for (s, st) in self.slots.iter().enumerate() {
            if let Some(id) = st.entity
                && w.player.get(id).map(|p| usize::from(p.slot)) != Some(s)
            {
                return Err("slot entity without matching comp.player");
            }
        }
        for (id, p) in w.player.iter() {
            if self.slots[usize::from(p.slot)].entity != Some(id) {
                return Err("comp.player not held by its slot");
            }
        }
        for (_, st) in w.statuses.iter() {
            if st.list.iter().any(|s| !self.id_allocated(s.source)) {
                return Err("status source outside the allocated range");
            }
        }
        Ok(())
    }

    fn id_allocated(&self, id: EntityId) -> bool {
        let f = self.folio.index;
        id.counter() >= 1
            && (id.folio_index() < f
                || (id.folio_index() == f && id.counter() < self.entities.counter().next_counter()))
    }

    /// SPEC: canon §3.2, §1.11, §21 test 31 (adapted). The Folio-owned part of
    /// the state, canonically encoded: Folio index, genre inputs and spawn
    /// point (not the entry tick), the per-Folio stream states, the entity and
    /// uid counters, and every column entry of entities created in this Folio.
    /// It depends only on the run seed, the Folio index and the genre inputs
    /// while no tick has been simulated in the Folio.
    #[must_use]
    pub fn folio_generation_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        self.folio.index.put(&mut out);
        self.folio.inputs.genre.put(&mut out);
        self.folio.inputs.bleed.put(&mut out);
        self.folio.spawn_point.put(&mut out);
        self.put_rng(&mut out, |k| FOLIO_STREAMS.contains(&k));
        self.entities.counter().next_counter().put(&mut out);
        self.uids.counter().next_counter().put(&mut out);
        let f = self.folio.index;
        self.world
            .put_filtered(&mut out, |id| id.folio_index() == f);
        out
    }
}

/// Checked `u64` to `i64` for bounded draws.
#[track_caller]
fn to_i64(v: u64) -> i64 {
    i64::try_from(v).expect("draw fits i64")
}

/// Clamp a horizontal axis to `[0, hi]`, zeroing that velocity axis at a wall.
fn clamp_axis(p: &mut Fx, v: &mut Fx, hi: Fx) {
    if *p < Fx::ZERO {
        *p = Fx::ZERO;
        *v = Fx::ZERO;
    } else if *p > hi {
        *p = hi;
        *v = Fx::ZERO;
    }
}

/// SPEC: canon §1.8-1.9, doc 02 §5.5 input axes (skeleton wish velocity).
/// `(move_x, move_z)` is in the yaw frame: at yaw 0, `(0, -127)` is full
/// forward (−Z) and `(127, 0)` full right (+X); `-128` maps to `-127`. The
/// world direction is `right × mx + back × mz` with `right = (cos, 0, -sin)`
/// and `back = (sin, 0, cos)` in Q2.30, its length capped at 127, then
/// scaled so a full deflection is `speed`. Returns the target `(vx, vz)`.
#[must_use]
pub fn wish_velocity(move_x: i8, move_z: i8, yaw: Bam, speed: Fx) -> (Fx, Fx) {
    let mx = i64::from(move_x.max(-127));
    let mz = i64::from(move_z.max(-127));
    let s = i64::from(sin_bam(yaw));
    let c = i64::from(cos_bam(yaw));
    let mut wx = i128::from(c * mx + s * mz);
    let mut wz = i128::from(c * mz - s * mx);
    let lim: i128 = 127 * (1i128 << 30);
    let len2 = wx * wx + wz * wz;
    if len2 > lim * lim {
        let len = isqrt_u128(u128::try_from(len2).expect("len2 >= 0"));
        let len = i128::try_from(len).expect("len fits i128");
        wx = (wx * lim) / len;
        wz = (wz * lim) / len;
    }
    let sp = i128::from(speed.raw());
    (
        Fx::from_raw(narrow_i64((sp * wx) / lim)),
        Fx::from_raw(narrow_i64((sp * wz) / lim)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use palimpsest_math::Rng;
    use proptest::prelude::*;

    const RUN: u64 = 0x0123_4567_89AB_CDEF;

    fn cfg() -> SimConfig {
        SimConfig {
            content_hash: blake3_256(&[b"content"]),
            mode: fnv1a32(b"mode.standard"),
            party_size: 2,
            invader: true,
            hash_at_60: true,
        }
    }

    fn inputs(f: u32) -> FolioInputs {
        FolioInputs {
            genre: fnv1a32(b"genre.cyberpunk") ^ f,
            bleed: vec![fnv1a32(b"genre.noir"), fnv1a32(b"genre.wuxia") ^ f],
        }
    }

    /// The Folio entries both runs of the save tests perform, by boundary.
    fn folio_schedule(b: u64) -> Option<(u32, FolioInputs)> {
        match b {
            0 => Some((1, inputs(1))),
            5000 => Some((2, inputs(2))),
            9000 => Some((4, inputs(4))),
            _ => None,
        }
    }

    /// A fixed command log: every occupied slot sends a command on every
    /// tick, with moves, yaw, pitch and buttons drawn from a test stream.
    fn make_log(seed: u64, ticks: u64, slots: &[u8]) -> ReplayLog {
        let mut log = ReplayLog::new(
            SessionHeader {
                sim_version: SIM_VERSION,
                content_hash: cfg().content_hash,
                run_seed: RUN,
            },
            [7; 32],
        );
        let mut g = Rng::from_seed64(seed);
        let mut held = [0u32; SLOTS];
        let mut mv = [(0i8, 0i8); SLOTS];
        let mut yaw = [0u16; SLOTS];
        let mut seq = [0u32; SLOTS];
        for t in 0..ticks {
            for &s in slots {
                let i = usize::from(s);
                if g.range(20) == 0 {
                    let pick = |r: u64| -> i8 { [-128, -127, -64, 0, 0, 64, 127][r as usize] };
                    mv[i] = (pick(g.range(7)), pick(g.range(7)));
                    yaw[i] = u16::try_from(g.range(65536)).unwrap();
                }
                if g.range(6) == 0 {
                    held[i] = 0;
                    for b in [buttons::ATTACK_PRIMARY, buttons::JUMP, buttons::SPRINT] {
                        if g.range(2) == 0 {
                            held[i] |= b;
                        }
                    }
                    seq[i] += 1;
                }
                let pitch = i16::try_from(g.range_incl(-20_000, 20_000)).unwrap();
                log.push(PlayerCommand {
                    tick: t,
                    slot: s,
                    move_x: mv[i].0,
                    move_z: mv[i].1,
                    yaw: yaw[i],
                    pitch,
                    buttons: held[i],
                    action_seq: seq[i],
                    view_tick: t.saturating_sub(3),
                })
                .unwrap();
            }
        }
        log
    }

    /// Advance `sim` to boundary `to`, entering Folios per the schedule and
    /// collecting `(boundary, hash)` for every hashed boundary.
    fn drive(sim: &mut Sim, log: &ReplayLog, to: u64, hashes: &mut Vec<(u64, StateHash)>) {
        while sim.tick() < to {
            let b = sim.tick();
            if let Some((f, inp)) = folio_schedule(b) {
                sim.enter_folio(f, inp).unwrap();
            }
            let out = sim.step(log.for_tick(b)).unwrap();
            assert_eq!(out.tick, b);
            if let Some(h) = out.hash {
                hashes.push((sim.tick(), h));
            }
        }
    }

    const SLOTS_USED: [u8; 3] = [0, 1, INVADER_SLOT];

    #[test]
    fn canon_t30_hp_display() {
        // A character with 400 mHP displays 1 HP.
        assert_eq!(display_hp(400), 1);
        assert_eq!(display_hp(1), 1);
        assert_eq!(display_hp(1000), 1);
        assert_eq!(display_hp(1001), 2);
        assert_eq!(display_hp(85_000), 85);
        assert_eq!(display_hp(0), 0);
        // Other Milli values floor.
        assert_eq!(display_milli(400), 0);
        assert_eq!(display_milli(85_999), 85);
        assert_eq!(display_milli(-15_000), -15);
        assert_eq!(display_milli(-1), -1);
    }

    #[test]
    fn canon_t31_folio_isolation() {
        // Folio 7 from the same run seed and genre inputs is byte-identical
        // whether Folios 1 to 6 were played or skipped.
        let log = make_log(99, 3000, &SLOTS_USED);
        let mut played = Sim::new(RUN, cfg()).unwrap();
        for f in 1..=6u32 {
            played.enter_folio(f, inputs(f)).unwrap();
            for _ in 0..(400 + 37 * u64::from(f)) {
                let b = played.tick();
                played.step(log.for_tick(b)).unwrap();
            }
        }
        played.enter_folio(7, inputs(7)).unwrap();
        let mut skipped = Sim::new(RUN, cfg()).unwrap();
        skipped.enter_folio(7, inputs(7)).unwrap();
        assert_ne!(played.tick(), skipped.tick());
        assert_eq!(
            played.folio_generation_bytes(),
            skipped.folio_generation_bytes()
        );
        // The first Folio-7 entity and item are (7 << 40) | 1 in both runs.
        let first = |s: &Sim| {
            s.world()
                .kind
                .ids()
                .iter()
                .copied()
                .find(|id| id.folio_index() == 7)
        };
        assert_eq!(first(&played), Some(EntityId::new(7, 1)));
        assert_eq!(first(&skipped), Some(EntityId::new(7, 1)));
        let inv = skipped.world().inventory.get(EntityId::new(7, 1)).unwrap();
        assert_eq!(inv.uids, vec![ItemUid::new(7, 1)]);
        for name in FOLIO_STREAMS {
            assert_eq!(played.stream_state(name), skipped.stream_state(name));
        }
        // Slot entities keep their IDs across Folios.
        assert_eq!(played.slot_entity(0), Some(EntityId::new(0, 1)));
        assert_eq!(played.slot_entity(INVADER_SLOT), Some(EntityId::new(0, 3)));
        // Other inputs give other Folios.
        let mut other_genre = Sim::new(RUN, cfg()).unwrap();
        other_genre.enter_folio(7, inputs(8)).unwrap();
        assert_ne!(
            other_genre.folio_generation_bytes(),
            skipped.folio_generation_bytes()
        );
        let mut other_index = Sim::new(RUN, cfg()).unwrap();
        other_index.enter_folio(8, inputs(7)).unwrap();
        assert_ne!(
            other_index.folio_generation_bytes(),
            skipped.folio_generation_bytes()
        );
        let mut other_seed = Sim::new(RUN ^ 1, cfg()).unwrap();
        other_seed.enter_folio(7, inputs(7)).unwrap();
        assert_ne!(
            other_seed.folio_generation_bytes(),
            skipped.folio_generation_bytes()
        );
    }

    #[test]
    fn canon_t32_save_and_load() {
        // Run 10,000 ticks with a fixed input log, save at tick 4,321, load and
        // continue: the final state_hash equals the uninterrupted run's.
        let log = make_log(7, 10_000, &SLOTS_USED);
        let mut whole = Sim::new(RUN, cfg()).unwrap();
        let mut h1 = Vec::new();
        drive(&mut whole, &log, 10_000, &mut h1);

        let mut first = Sim::new(RUN, cfg()).unwrap();
        let mut h2 = Vec::new();
        drive(&mut first, &log, 4321, &mut h2);
        let save = first.save();
        assert_eq!(
            &save[..HEADER_LEN],
            &{
                let mut v = Vec::new();
                first.header().put(&mut v);
                v
            }[..]
        );
        let mut loaded = Sim::restore(&save, cfg()).unwrap();
        assert_eq!(loaded, first);
        assert_eq!(loaded.state_hash(), first.state_hash());
        assert_eq!(loaded.tick(), 4321);
        drop(first);
        drive(&mut loaded, &log, 10_000, &mut h2);

        assert_eq!(loaded.tick(), 10_000);
        assert_eq!(loaded.state_hash(), whole.state_hash());
        assert_eq!(loaded, whole);
        assert_eq!(h1, h2);
        assert_eq!(whole.folio().index, 4);
    }

    #[test]
    fn determinism_same_log_hashes_every_60_ticks() {
        let log = make_log(1234, 10_000, &SLOTS_USED);
        let mut a = Sim::new(RUN, cfg()).unwrap();
        let mut b = Sim::new(RUN, cfg()).unwrap();
        let (mut ha, mut hb) = (Vec::new(), Vec::new());
        drive(&mut a, &log, 10_000, &mut ha);
        drive(&mut b, &log, 10_000, &mut hb);
        assert_eq!(ha.len(), 166);
        assert!(ha.iter().all(|(bnd, _)| bnd % 60 == 0));
        for ((ba, x), (bb, y)) in ha.iter().zip(&hb) {
            assert_eq!(ba, bb);
            assert_eq!(x, y, "hash mismatch at boundary {ba}");
        }
        // The step-returned hash is the boundary's state hash, and a shorter
        // run reproduces the prefix.
        let mut p = Sim::new(RUN, cfg()).unwrap();
        let mut hp = Vec::new();
        drive(&mut p, &log, 600, &mut hp);
        assert_eq!(hp.last().unwrap().1, p.state_hash());
        assert_eq!(hp[..], ha[..10]);
        // The run actually exercised combat, statuses and despawns.
        let anyone_hurt = a
            .world()
            .pools
            .iter()
            .any(|(_, p)| p.combat.taken.is_some() || p.combat.dealt.is_some());
        assert!(anyone_hurt, "the log should produce at least one hit");
        // A different run seed diverges.
        let mut c = Sim::new(RUN ^ 0x55, cfg()).unwrap();
        let mut hc = Vec::new();
        drive(&mut c, &log, 600, &mut hc);
        assert_ne!(hc[9].1, ha[9].1);
    }

    #[test]
    fn serialization_layout_and_hash() {
        let sim = Sim::new(RUN, cfg()).unwrap();
        let bytes = sim.serialize();
        // 13 sections, each tagged with its index and a u32le length.
        let mut r = Reader::new(&bytes);
        for k in 0..13u8 {
            assert_eq!(u8::read(&mut r).unwrap(), k);
            let n = r.len_u32().unwrap();
            let _ = r.bytes(n).unwrap();
        }
        r.finish().unwrap();
        assert_eq!(sim.state_hash(), blake3_256(&[&bytes]));
        // Meta begins with SIM_VERSION, tick 0 and the run seed, little-endian.
        assert_eq!(bytes[0], 0);
        let meta_len = u32::from_le_bytes(bytes[1..5].try_into().unwrap()) as usize;
        let meta = &bytes[5..5 + meta_len];
        assert_eq!(&meta[0..4], &SIM_VERSION.to_le_bytes());
        assert_eq!(&meta[4..12], &0u64.to_le_bytes());
        assert_eq!(&meta[12..20], &RUN.to_le_bytes());
        let sec = sim.section_hashes();
        assert_eq!(sec[3], blake3_256(&[]));
        assert_ne!(sec[0], sec[2]);
    }

    #[test]
    fn restore_rejects_bad_saves() {
        let mut sim = Sim::new(RUN, cfg()).unwrap();
        sim.enter_folio(1, inputs(1)).unwrap();
        let log = make_log(5, 200, &SLOTS_USED);
        for t in 0..200 {
            sim.step(log.for_tick(t)).unwrap();
        }
        let save = sim.save();
        assert_eq!(Sim::restore(&save, cfg()).unwrap(), sim);

        let mut trailing = save.clone();
        trailing.push(0);
        assert_eq!(
            Sim::restore(&trailing, cfg()),
            Err(LoadError::Decode(DecodeError::TrailingBytes))
        );
        assert_eq!(
            Sim::restore(&save[..save.len() - 1], cfg()),
            Err(LoadError::Decode(DecodeError::UnexpectedEnd))
        );
        let mut version = save.clone();
        version[0] = 2;
        assert!(matches!(
            Sim::restore(&version, cfg()),
            Err(LoadError::SimVersion { found: 2, .. })
        ));
        let mut other = cfg();
        other.content_hash[0] ^= 1;
        assert_eq!(Sim::restore(&save, other), Err(LoadError::ContentHash));
        let mut mode = cfg();
        mode.mode ^= 1;
        assert_eq!(Sim::restore(&save, mode), Err(LoadError::Mode));
        let mut seed = save.clone();
        seed[36] ^= 1; // header run_seed
        assert_eq!(Sim::restore(&seed, cfg()), Err(LoadError::RunSeed));
        let mut section = save.clone();
        section[HEADER_LEN] = 9;
        assert!(matches!(
            Sim::restore(&section, cfg()),
            Err(LoadError::Decode(DecodeError::BadSection {
                expected: 0,
                found: 9
            }))
        ));
        // Corrupting every byte of the body must never panic.
        for i in HEADER_LEN..save.len() {
            let mut bad = save.clone();
            bad[i] ^= 0x80;
            let _ = Sim::restore(&bad, cfg());
        }
    }

    #[test]
    fn restore_rejects_unsorted_columns() {
        let mut col: Column<Vel> = Column::new();
        col.insert(EntityId::new(0, 2), Vel::default());
        col.insert(EntityId::new(0, 1), Vel::default());
        assert_eq!(col.ids(), &[EntityId::new(0, 1), EntityId::new(0, 2)]);
        let mut bytes = Vec::new();
        put_len(&mut bytes, 2);
        EntityId::new(0, 2).put(&mut bytes);
        Vel::default().put(&mut bytes);
        EntityId::new(0, 1).put(&mut bytes);
        Vel::default().put(&mut bytes);
        assert_eq!(
            Column::<Vel>::read(&mut Reader::new(&bytes)),
            Err(DecodeError::NotAscending("column"))
        );
    }

    #[test]
    fn column_store_ordering() {
        let mut c: Column<u32> = Column::new();
        assert!(c.is_empty());
        for (f, n, v) in [(1, 5, 50), (0, 9, 9), (1, 1, 10), (2, 1, 21), (1, 5, 55)] {
            c.insert(EntityId::new(f, n), v);
        }
        let got: Vec<(u64, u32)> = c.iter().map(|(id, &v)| (id.raw(), v)).collect();
        assert_eq!(
            got,
            vec![
                (9, 9),
                ((1 << 40) | 1, 10),
                ((1 << 40) | 5, 55),
                ((2 << 40) | 1, 21)
            ]
        );
        assert_eq!(c.len(), 4);
        assert_eq!(c.get(EntityId::new(1, 5)), Some(&55));
        *c.get_mut(EntityId::new(1, 1)).unwrap() = 11;
        assert_eq!(c.remove(EntityId::new(1, 1)), Some(11));
        assert!(!c.contains(EntityId::new(1, 1)));
        assert_eq!(c.remove(EntityId::new(1, 1)), None);
    }

    #[test]
    fn command_validation() {
        let mut sim = Sim::new(RUN, cfg()).unwrap();
        let ok = PlayerCommand::default();
        let before = sim.clone();
        assert_eq!(
            sim.step(&[PlayerCommand { tick: 1, ..ok }]),
            Err(CommandError::WrongTick {
                expected: 0,
                found: 1
            })
        );
        assert_eq!(
            sim.step(&[PlayerCommand { slot: 5, ..ok }]),
            Err(CommandError::BadSlot(5))
        );
        assert_eq!(
            sim.step(&[PlayerCommand { slot: 2, ..ok }]),
            Err(CommandError::EmptySlot(2))
        );
        assert_eq!(
            sim.step(&[ok, PlayerCommand { buttons: 1, ..ok }]),
            Err(CommandError::DuplicateSlot(0))
        );
        assert_eq!(
            sim.step(&[PlayerCommand {
                buttons: 1 << 26,
                ..ok
            }]),
            Err(CommandError::ReservedButtons(1 << 26))
        );
        assert_eq!(sim, before, "a refused step changes nothing");
        // Order of the slice does not matter: slot order is applied.
        let a = [
            PlayerCommand {
                slot: 4,
                buttons: 1,
                ..ok
            },
            PlayerCommand {
                slot: 0,
                move_z: -127,
                ..ok
            },
        ];
        let mut s1 = sim.clone();
        let mut s2 = sim.clone();
        s1.step(&a).unwrap();
        s2.step(&[a[1], a[0]]).unwrap();
        assert_eq!(s1, s2);
    }

    #[test]
    fn input_apply_maps_axes_and_presses() {
        let mut sim = Sim::new(RUN, cfg()).unwrap();
        let id = sim.slot_entity(0).unwrap();
        let c = PlayerCommand {
            tick: 0,
            slot: 0,
            move_x: -128,
            move_z: -128,
            yaw: 100,
            pitch: -30_000,
            buttons: buttons::JUMP,
            action_seq: 9,
            view_tick: 0,
        };
        sim.step(&[c]).unwrap();
        let ctrl = sim.world().control.get(id).unwrap();
        assert_eq!((ctrl.move_x, ctrl.move_z), (-127, -127));
        let pos = sim.world().pos.get(id).unwrap();
        assert_eq!(pos.pitch, -16_000);
        assert_eq!(pos.yaw, 100);
        assert_eq!(sim.slots[0].prev_buttons, buttons::JUMP);
        assert_eq!(sim.slots[0].action_seq, 9);
        // The JUMP press launched the body.
        assert!(sim.world().vel.get(id).unwrap().v.y > Fx::ZERO);
        // Held JUMP is not a new press: no second launch after landing.
        for t in 1..200 {
            sim.step(&[PlayerCommand { tick: t, ..c }]).unwrap();
        }
        let p = sim.world().pos.get(id).unwrap().p;
        assert_eq!(p.y, Fx::ZERO);
        assert_eq!(sim.world().vel.get(id).unwrap().v.y, Fx::ZERO);
    }

    #[test]
    fn movement_is_semi_implicit_euler() {
        let mut sim = Sim::new(RUN, SimConfig::default()).unwrap();
        let id = sim.slot_entity(0).unwrap();
        let p0 = sim.world().pos.get(id).unwrap().p;
        let c = PlayerCommand {
            move_z: -127,
            ..PlayerCommand::default()
        };
        sim.step(&[c]).unwrap();
        // Full forward at yaw 0 targets (0, -4.3 m/s).
        let (tx, tz) = wish_velocity(0, -127, 0, WALK);
        assert_eq!(tx, Fx::ZERO);
        assert_eq!(tz, -WALK);
        let a = Fx3::new(Fx::ZERO, -GRAVITY, tz.mul_int(ACCEL_PER_S));
        let (mut p1, mut v1) = semi_implicit_euler(p0, Fx3::ZERO, a);
        p1.y = Fx::ZERO;
        v1.y = Fx::ZERO;
        assert_eq!(sim.world().pos.get(id).unwrap().p, p1);
        assert_eq!(sim.world().vel.get(id).unwrap().v, v1);
        assert!(p1.z < p0.z, "forward is -Z");
    }

    #[test]
    fn wish_velocity_frame() {
        // Yaw 0: right is +X, forward is -Z.
        assert_eq!(wish_velocity(127, 0, 0, WALK), (WALK, Fx::ZERO));
        assert_eq!(wish_velocity(0, 0, 0, WALK), (Fx::ZERO, Fx::ZERO));
        // Yaw 16384 (90° counter-clockwise): forward is -X, right is -Z.
        let (fx, fz) = wish_velocity(0, -127, 16384, WALK);
        assert_eq!((fx, fz), (-WALK, Fx::ZERO));
        let (rx, rz) = wish_velocity(127, 0, 16384, WALK);
        assert_eq!((rx, rz), (Fx::ZERO, -WALK));
    }

    #[test]
    fn hit_draws_keyed_rolls_and_applies_bleed() {
        let mut sim = Sim::new(RUN, cfg()).unwrap();
        sim.enter_folio(3, inputs(3)).unwrap();
        let actor = sim.slot_entity(0).unwrap();
        let target = EntityId::new(3, 1);
        // Keep only the target creature and the slot entities.
        let others: Vec<EntityId> = sim
            .world()
            .kind
            .ids()
            .iter()
            .copied()
            .filter(|&id| id != target && !sim.world().player.contains(id))
            .collect();
        for id in others {
            sim.world.despawn(id);
        }
        // Put the creature right next to the actor.
        let at = sim.world().pos.get(actor).unwrap().p;
        sim.world.pos.get_mut(target).unwrap().p = at.add(Fx3::from_ints(1, 0, 0));
        // Find a tick whose keyed rolls give a hit with bleed (deterministic).
        let t0 = sim.tick();
        let mut t = t0;
        while !keyed_rng(RUN, streams::STATUS, 3, actor.raw(), t, 0).roll_bp(BLEED_CHANCE_BP) {
            t += 1;
        }
        let crit = keyed_rng(RUN, streams::COMBAT, 3, actor.raw(), t, 0).roll_bp(CRIT_CHANCE_BP);
        while sim.tick() < t {
            let b = sim.tick();
            sim.step(&[PlayerCommand {
                tick: b,
                ..PlayerCommand::default()
            }])
            .unwrap();
            sim.world.pos.get_mut(target).unwrap().p = at.add(Fx3::from_ints(1, 0, 0));
            sim.world.pos.get_mut(actor).unwrap().p = at;
        }
        let hp0 = sim.world().pools.get(target).unwrap().hp;
        sim.step(&[PlayerCommand {
            tick: t,
            buttons: buttons::ATTACK_PRIMARY,
            ..PlayerCommand::default()
        }])
        .unwrap();
        let dmg = if crit { 15_000 } else { 10_000 };
        let tp = sim.world().pools.get(target).unwrap();
        // Resources may regenerate 0 here: damage was taken this tick.
        assert_eq!(tp.hp, hp0 - dmg);
        assert_eq!(tp.combat.taken, Some(t));
        assert_eq!(sim.in_combat(actor, t), Some(true));
        assert_eq!(sim.in_combat(actor, t + 301), Some(false));
        let st = sim.world().statuses.get(target).unwrap();
        let bleed = st.get(ST_BLEEDING).unwrap();
        assert_eq!(bleed.timer.applied(), t);
        assert_eq!(bleed.timer.expiry(), Some(t + 300));
        assert_eq!(bleed.source, actor);
    }

    #[test]
    fn bleed_dot_applies_850_over_four_pulses() {
        // Canon §21 test 10 in situ: 850 mHP/s through the pulse at k = 4.
        let mut sim = Sim::new(RUN, SimConfig::default()).unwrap();
        let id = sim.slot_entity(0).unwrap();
        // Apply during tick 0 for 60 ticks: active 1..=60, pulses 15, 30, 45, 60.
        sim.world
            .statuses
            .get_mut(id)
            .unwrap()
            .apply(ST_BLEEDING, 0, 60, id, BLEED_DOT);
        // Each pulse's damage gates regeneration for 5 s, so HP changes only
        // on pulse ticks.
        let mut lost = 0;
        for t in 0..=61u64 {
            let before = sim.world().pools.get(id).unwrap().hp;
            sim.step(&[]).unwrap();
            let after = sim.world().pools.get(id).unwrap().hp;
            if runs_on(Subsystem::StatusPulse, t, 0) {
                lost += before - after;
            }
        }
        assert_eq!(lost, 850);
        // Removed at the end of tick 60.
        assert!(
            sim.world()
                .statuses
                .get(id)
                .unwrap()
                .get(ST_BLEEDING)
                .is_none()
        );
    }

    #[test]
    fn regen_waits_five_seconds_without_damage() {
        let mut sim = Sim::new(RUN, SimConfig::default()).unwrap();
        let id = sim.slot_entity(0).unwrap();
        {
            let p = sim.world.pools.get_mut(id).unwrap();
            p.hp = 50_000;
            p.last_damage_taken = Some(0);
        }
        for _ in 0..300 {
            sim.step(&[]).unwrap();
        }
        // Ticks 0..=299: t - 0 < 300, regen suspended.
        assert_eq!(sim.world().pools.get(id).unwrap().hp, 50_000);
        for _ in 0..60 {
            sim.step(&[]).unwrap();
        }
        // Ticks 300..=359: exactly one second of 425 mHP/s.
        assert_eq!(sim.world().pools.get(id).unwrap().hp, 50_425);
    }

    #[test]
    fn sprint_drains_and_regen_waits_one_second() {
        let mut sim = Sim::new(RUN, SimConfig::default()).unwrap();
        let id = sim.slot_entity(0).unwrap();
        let c = PlayerCommand {
            move_z: -127,
            buttons: buttons::SPRINT,
            ..PlayerCommand::default()
        };
        for t in 0..60 {
            sim.step(&[PlayerCommand { tick: t, ..c }]).unwrap();
        }
        // One second of 12 stamina per s.
        assert_eq!(sim.world().pools.get(id).unwrap().stamina, 78_000);
        for t in 60..140 {
            sim.step(&[PlayerCommand {
                tick: t,
                buttons: 0,
                ..c
            }])
            .unwrap();
        }
        // Spent last on tick 59; regen runs on ticks 119..=139: 21 ticks.
        let mut acc = RateAcc::ZERO;
        let regen: i64 = (0..21).map(|_| acc.step(STAMINA_REGEN, K_PER_TICK)).sum();
        assert!(78_000 + regen < 90_000);
        assert_eq!(sim.world().pools.get(id).unwrap().stamina, 78_000 + regen);
    }

    #[test]
    fn folio_entry_rules() {
        let mut sim = Sim::new(RUN, cfg()).unwrap();
        assert_eq!(sim.folio().index, 0);
        assert_eq!(sim.slot_entity(1), Some(EntityId::new(0, 2)));
        sim.enter_folio(2, inputs(2)).unwrap();
        assert_eq!(
            sim.enter_folio(2, inputs(2)),
            Err(FolioError::NotIncreasing {
                current: 2,
                requested: 2
            })
        );
        assert_eq!(
            sim.enter_folio(1, inputs(1)),
            Err(FolioError::NotIncreasing {
                current: 2,
                requested: 1
            })
        );
        assert_eq!(
            sim.enter_folio(MAX_FOLIO_INDEX + 1, inputs(1)),
            Err(FolioError::IndexTooLarge(MAX_FOLIO_INDEX + 1))
        );
        let n2 = sim.world().kind.len();
        sim.enter_folio(3, inputs(3)).unwrap();
        // Folio 2 creatures are gone; Folio 3 ones start at counter 1.
        assert!(
            sim.world()
                .kind
                .ids()
                .iter()
                .all(|id| { id.folio_index() == 3 || sim.world().player.contains(*id) })
        );
        assert!(n2 >= 3 + 3);
        assert_eq!(sim.counters().0.folio_index(), 3);
        // Slot entities stand at the new spawn point.
        let sp = sim.folio().spawn_point;
        let p0 = sim.world().pos.get(sim.slot_entity(0).unwrap()).unwrap().p;
        assert_eq!(p0, sp);
        assert_eq!(
            Sim::new(
                RUN,
                SimConfig {
                    party_size: 0,
                    ..cfg()
                }
            ),
            Err(ConfigError::PartySize(0))
        );
        assert_eq!(
            Sim::new(
                RUN,
                SimConfig {
                    party_size: 5,
                    ..cfg()
                }
            ),
            Err(ConfigError::PartySize(5))
        );
    }

    #[test]
    fn replay_log_round_trip_and_order() {
        let log = make_log(3, 50, &SLOTS_USED);
        assert_eq!(log.for_tick(10).len(), 3);
        assert!(log.for_tick(10).iter().all(|c| c.tick == 10));
        assert_eq!(log.for_tick(50).len(), 0);
        let bytes = log.encode();
        assert_eq!(
            bytes.len(),
            HEADER_LEN + 32 + 4 + 150 * PlayerCommand::ENCODED_LEN
        );
        assert_eq!(ReplayLog::decode(&bytes).unwrap(), log);
        // Canon-1.11 §2.4: the replay records presentation_hash.
        assert_eq!(&bytes[HEADER_LEN..HEADER_LEN + 32], &[7u8; 32]);
        let mut l2 = ReplayLog::new(log.header, log.presentation_hash);
        l2.push(PlayerCommand {
            tick: 5,
            slot: 1,
            ..PlayerCommand::default()
        })
        .unwrap();
        assert_eq!(
            l2.push(PlayerCommand {
                tick: 5,
                slot: 1,
                ..PlayerCommand::default()
            }),
            Err(CommandError::OutOfOrder)
        );
        assert_eq!(
            l2.push(PlayerCommand {
                tick: 4,
                slot: 3,
                ..PlayerCommand::default()
            }),
            Err(CommandError::OutOfOrder)
        );
        // An out-of-order encoded log is rejected.
        let mut bad = Vec::new();
        log.header.put(&mut bad);
        log.presentation_hash.put(&mut bad);
        put_len(&mut bad, 2);
        PlayerCommand {
            tick: 2,
            ..PlayerCommand::default()
        }
        .put(&mut bad);
        PlayerCommand {
            tick: 1,
            ..PlayerCommand::default()
        }
        .put(&mut bad);
        assert_eq!(
            ReplayLog::decode(&bad),
            Err(DecodeError::NotAscending("replay commands"))
        );
    }

    /// SPEC: canon §2.4 (canon-1.11): `content_hash` excludes the
    /// presentation namespaces, which feed `presentation_hash`; replays must
    /// match `content_hash`, and a `presentation_hash` mismatch only warns.
    #[test]
    fn presentation_hash_split() {
        for id in [
            "art.post.wuxia",
            "aud.music.noir",
            "vfx.sparks",
            "nar.job.marginalia",
            "ui.hud.minimap",
        ] {
            assert!(is_presentation_record(id), "{id}");
        }
        for id in [
            "item.sword",
            "genre.wuxia",
            "loot.chest.plain",
            "artifact.x",
        ] {
            assert!(!is_presentation_record(id), "{id}");
        }
        let log = make_log(3, 2, &SLOTS_USED);
        let ch = cfg().content_hash;
        assert_eq!(log.compat(SIM_VERSION, &ch, &[7; 32]), ReplayCompat::Exact);
        assert_eq!(
            log.compat(SIM_VERSION, &ch, &[8; 32]),
            ReplayCompat::PresentationMismatch
        );
        assert_eq!(
            log.compat(SIM_VERSION, &[9; 32], &[7; 32]),
            ReplayCompat::Incompatible
        );
        assert_eq!(
            log.compat(SIM_VERSION + 1, &ch, &[7; 32]),
            ReplayCompat::Incompatible
        );
    }

    /// SPEC: canon §2.2 and §2.5 (canon-1.11): boundary `b` follows tick
    /// `b - 1`; the host hashes boundaries with `b % 60 == 0` and `b >= 60`.
    #[test]
    fn named_boundaries_and_hash_boundaries() {
        assert_eq!(boundary_after(0), 1);
        assert_eq!(boundary_after(59), 60);
        assert!(!is_hash_boundary(0));
        assert!(!is_hash_boundary(59));
        assert!(is_hash_boundary(60));
        assert!(!is_hash_boundary(61));
        assert!(is_hash_boundary(120));
        let mut sim = Sim::new(
            RUN,
            SimConfig {
                hash_at_60: true,
                ..cfg()
            },
        )
        .unwrap();
        let mut hashed = Vec::new();
        for _ in 0..121 {
            let out = sim.step(&[]).unwrap();
            if out.hash.is_some() {
                hashed.push(boundary_after(out.tick));
            }
        }
        assert_eq!(hashed, [60, 120]);
    }

    /// SPEC: canon §2.5 (canon-1.11): single player saves at any boundary,
    /// except while slot 4 holds an invader (the save waits for the
    /// invasion's end); co-op saves only at Folio entry.
    #[test]
    fn save_timing_rules() {
        let solo = Sim::new(
            RUN,
            SimConfig {
                party_size: 1,
                invader: false,
                ..cfg()
            },
        )
        .unwrap();
        assert_eq!(solo.save_timing(), SaveTiming::AnyBoundary);
        let invaded = Sim::new(
            RUN,
            SimConfig {
                party_size: 1,
                invader: true,
                ..cfg()
            },
        )
        .unwrap();
        assert_eq!(invaded.save_timing(), SaveTiming::AfterInvasion);
        let coop = Sim::new(
            RUN,
            SimConfig {
                party_size: 3,
                invader: false,
                ..cfg()
            },
        )
        .unwrap();
        assert_eq!(coop.save_timing(), SaveTiming::FolioEntryOnly);
    }

    /// SPEC: canon §11.8 (canon-1.11), doc 16 §5.4: a missing command is
    /// replaced by the neutral substitute, which never moves, releases every
    /// action button, keeps stance and quickbar index, and never presses.
    #[test]
    fn missing_command_gets_neutral_substitute() {
        let last = PlayerCommand {
            tick: 9,
            slot: 2,
            move_x: 90,
            move_z: -127,
            yaw: 1234,
            pitch: -500,
            buttons: buttons::SPRINT | buttons::ATTACK_PRIMARY | (1 << 4) | (0b101 << 15),
            action_seq: 77,
            view_tick: 6,
        };
        let sub = neutral_substitute(&last, 10);
        assert_eq!(
            sub,
            PlayerCommand {
                tick: 10,
                slot: 2,
                move_x: 0,
                move_z: 0,
                yaw: 1234,
                pitch: -500,
                buttons: (1 << 4) | (0b101 << 15),
                action_seq: 77,
                view_tick: 10,
            }
        );
        assert_eq!(sub.buttons & !last.buttons, 0, "no press edge");
        assert_eq!(neutral_substitute(&sub, 11).buttons, sub.buttons);
        // In the skeleton: a sprinting, moving Erratum stops when its command
        // goes missing, exactly as if the substitute had been sent.
        let mut a = Sim::new(RUN, cfg()).unwrap();
        let mut b = Sim::new(RUN, cfg()).unwrap();
        let go = PlayerCommand {
            tick: 0,
            slot: 0,
            move_z: -127,
            buttons: buttons::SPRINT,
            ..PlayerCommand::default()
        };
        a.step(&[go]).unwrap();
        b.step(&[go]).unwrap();
        a.step(&[]).unwrap();
        b.step(&[neutral_substitute(&go, 1)]).unwrap();
        assert_eq!(a.state_hash(), b.state_hash());
        let id = a.slots[0].entity.unwrap();
        let ctrl = a.world.control.get(id).unwrap();
        assert_eq!((ctrl.move_x, ctrl.move_z, ctrl.buttons), (0, 0, 0));
    }

    /// SPEC: canon §1.5 (canon-1.11): per-second rates below 10 display units
    /// show one decimal place as `floor(x / 100)` tenths (doc 17 test 13:
    /// 673 mHP/s displays "0.6").
    #[test]
    fn small_rate_display() {
        assert_eq!(display_rate_per_s(673), RateDisplay::Tenths(6));
        assert_eq!(display_rate_per_s(9_999), RateDisplay::Tenths(99));
        assert_eq!(display_rate_per_s(10_000), RateDisplay::Whole(10));
        assert_eq!(display_rate_per_s(23_000), RateDisplay::Whole(23));
        assert_eq!(display_rate_per_s(0), RateDisplay::Tenths(0));
        assert_eq!(display_rate_per_s(-673), RateDisplay::Tenths(-7));
    }

    /// Review (canon §1.5): the rate-display boundaries and extremes never
    /// overflow: 9,999 and 10,000 straddle the switch, -1 floors to -1
    /// tenth, and the `i64` ends stay in range.
    #[test]
    fn rate_display_extremes() {
        assert_eq!(display_rate_per_s(99), RateDisplay::Tenths(0));
        assert_eq!(display_rate_per_s(100), RateDisplay::Tenths(1));
        assert_eq!(display_rate_per_s(-1), RateDisplay::Tenths(-1));
        assert_eq!(display_rate_per_s(10_999), RateDisplay::Whole(10));
        assert_eq!(
            display_rate_per_s(i64::MAX),
            RateDisplay::Whole(i64::MAX / 1000)
        );
        assert_eq!(
            display_rate_per_s(i64::MIN),
            RateDisplay::Tenths(floor_div(i64::MIN, 100))
        );
    }

    /// Review (canon §2.4): only the namespace segment decides; a namespace
    /// that merely starts with a presentation prefix, or a string without a
    /// dot, is not a presentation record.
    #[test]
    fn presentation_namespace_edges() {
        for id in ["arts.x", "nar_x.y", "uix.hud", "art", "", "input.attack"] {
            assert!(!is_presentation_record(id), "{id}");
        }
        assert!(is_presentation_record("ui.hud.vitals"));
        assert!(is_presentation_record("aud.music.wuxia.combat"));
    }

    /// Review (canon §2.4): a replay log whose 32-byte `presentation_hash`
    /// is cut short does not decode.
    #[test]
    fn replay_log_needs_full_presentation_hash() {
        let log = make_log(4, 1, &SLOTS_USED);
        let bytes = log.encode();
        assert!(ReplayLog::decode(&bytes[..HEADER_LEN + 31]).is_err());
        assert!(ReplayLog::decode(&bytes[..bytes.len() - 1]).is_err());
        assert_eq!(ReplayLog::decode(&bytes).unwrap(), log);
    }

    /// Review (canon §2.2): the boundary after the last representable tick
    /// does not exist, so `boundary_after` panics instead of wrapping.
    #[test]
    #[should_panic(expected = "tick overflow")]
    fn boundary_after_last_tick_panics() {
        let _ = boundary_after(u64::MAX);
    }

    /// Review (canon §3.3): the skeleton's running keyed `seq` stays inside
    /// doc 05's reserved action and hit slots `0x0000..=0x01FF`; slot 0x01FF
    /// is the last one handed out, and a further draw in the same tick panics.
    #[test]
    fn tick_seq_stays_in_reserved_slots() {
        let mut ctx = TickCtx::default();
        let actor = EntityId::new(1, 1);
        for want in 0..=0x01FFu32 {
            assert_eq!(ctx.next_seq(actor), want);
        }
        // Another actor has its own counter.
        assert_eq!(ctx.next_seq(EntityId::new(1, 2)), 0);
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| ctx.next_seq(actor)));
        assert!(r.is_err(), "seq 0x0200 is outside the reserved slots");
    }

    #[test]
    fn player_command_layout() {
        let c = PlayerCommand {
            tick: 0x0102_0304_0506_0708,
            slot: 4,
            move_x: -1,
            move_z: 127,
            yaw: 0xABCD,
            pitch: -2,
            buttons: 0x0300_0001,
            action_seq: 7,
            view_tick: 9,
        };
        let b = c.encode();
        assert_eq!(b.len(), PlayerCommand::ENCODED_LEN);
        assert_eq!(&b[0..8], &[8, 7, 6, 5, 4, 3, 2, 1]);
        assert_eq!(b[8], 4);
        assert_eq!(b[9], 0xFF);
        assert_eq!(b[10], 127);
        assert_eq!(&b[11..13], &[0xCD, 0xAB]);
        assert_eq!(&b[13..15], &[0xFE, 0xFF]);
        assert_eq!(&b[15..19], &[1, 0, 0, 3]);
        assert_eq!(&b[19..23], &[7, 0, 0, 0]);
        assert_eq!(&b[23..31], &[9, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(PlayerCommand::decode(&b), Ok(c));
        assert_eq!(c.validate(), Ok(()));
    }

    fn arb_command() -> impl Strategy<Value = PlayerCommand> {
        (
            any::<u64>(),
            any::<u8>(),
            any::<i8>(),
            any::<i8>(),
            any::<u16>(),
            any::<i16>(),
            any::<u32>(),
            any::<u32>(),
            any::<u64>(),
        )
            .prop_map(
                |(tick, slot, move_x, move_z, yaw, pitch, buttons, action_seq, view_tick)| {
                    PlayerCommand {
                        tick,
                        slot,
                        move_x,
                        move_z,
                        yaw,
                        pitch,
                        buttons,
                        action_seq,
                        view_tick,
                    }
                },
            )
    }

    proptest! {
        #[test]
        fn command_encode_round_trip(c in arb_command()) {
            prop_assert_eq!(PlayerCommand::decode(&c.encode()), Ok(c));
        }

        /// Review (canon §11.8, doc 16 §5.4): the neutral substitute holds
        /// no press edge, keeps only `HOLD_MASK` bits, moves nowhere, keeps
        /// the view and `action_seq`, and is idempotent.
        #[test]
        fn neutral_substitute_never_presses(c in arb_command(), t in any::<u64>()) {
            let s = neutral_substitute(&c, t);
            prop_assert_eq!(s.buttons & !c.buttons, 0);
            prop_assert_eq!(s.buttons & !buttons::HOLD_MASK, 0);
            prop_assert_eq!((s.move_x, s.move_z, s.tick, s.view_tick), (0, 0, t, t));
            prop_assert_eq!(
                (s.slot, s.yaw, s.pitch, s.action_seq),
                (c.slot, c.yaw, c.pitch, c.action_seq)
            );
            prop_assert_eq!(neutral_substitute(&s, t), s);
            prop_assert_eq!(s.validate().is_ok(), usize::from(c.slot) < SLOTS);
        }

        #[test]
        fn wish_velocity_never_exceeds_speed(
            mx in any::<i8>(), mz in any::<i8>(), yaw in any::<u16>(), mm in 0i64..20_000,
        ) {
            let speed = fx_from_milli(mm);
            let (vx, vz) = wish_velocity(mx, mz, yaw, speed);
            let l2 = i128::from(vx.raw()).pow(2) + i128::from(vz.raw()).pow(2);
            // Floor rounding only ever shortens; allow one raw unit per axis.
            let lim = i128::from(speed.raw()) + 2;
            prop_assert!(l2 <= lim * lim);
            if mx == 0 && mz == 0 {
                prop_assert_eq!((vx, vz), (Fx::ZERO, Fx::ZERO));
            }
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(6))]

        #[test]
        fn save_restore_anywhere_matches_uninterrupted(
            seed in any::<u64>(), run in any::<u64>(), cut in 1u64..1200,
        ) {
            let log = make_log(seed, 1200, &SLOTS_USED);
            let mut a = Sim::new(run, cfg()).unwrap();
            let mut ha = Vec::new();
            drive(&mut a, &log, 1200, &mut ha);
            let mut b = Sim::new(run, cfg()).unwrap();
            let mut hb = Vec::new();
            drive(&mut b, &log, cut, &mut hb);
            let mut b = Sim::restore(&b.save(), cfg()).unwrap();
            drive(&mut b, &log, 1200, &mut hb);
            prop_assert_eq!(a.state_hash(), b.state_hash());
            prop_assert_eq!(ha, hb);
        }

        /// Review (canon §11.8, §2.4, doc 16 §5.4): a run whose missing
        /// commands the skeleton substitutes itself equals, tick by tick, the
        /// run in which the host passes (and could log) the neutral
        /// substitute built from each slot's last applied command.
        #[test]
        fn missing_commands_equal_explicit_substitutes(
            seed in any::<u64>(), drop_seed in any::<u64>(),
        ) {
            let ticks = 240;
            let log = make_log(seed, ticks, &SLOTS_USED);
            let mut g = Rng::from_seed64(drop_seed);
            let mut a = Sim::new(RUN, cfg()).unwrap();
            let mut b = Sim::new(RUN, cfg()).unwrap();
            a.enter_folio(1, inputs(1)).unwrap();
            b.enter_folio(1, inputs(1)).unwrap();
            let mut last: [PlayerCommand; SLOTS] = core::array::from_fn(|s| PlayerCommand {
                slot: u8::try_from(s).unwrap(),
                ..PlayerCommand::default()
            });
            for t in 0..ticks {
                let mut sent = Vec::new();
                let mut explicit = Vec::new();
                for c in log.for_tick(t) {
                    let mut c = *c;
                    // Stance and quickbar bits, so HOLD_MASK matters.
                    c.buttons |= (u32::try_from(g.range(2)).unwrap() << 4)
                        | (u32::try_from(g.range(8)).unwrap() << 15)
                        | (u32::try_from(g.range(2)).unwrap() << 24);
                    let i = usize::from(c.slot);
                    if g.range(4) == 0 {
                        let sub = neutral_substitute(&last[i], t);
                        explicit.push(sub);
                        last[i] = sub;
                    } else {
                        sent.push(c);
                        explicit.push(c);
                        last[i] = c;
                    }
                }
                a.step(&sent).unwrap();
                b.step(&explicit).unwrap();
                prop_assert_eq!(a.state_hash(), b.state_hash(), "tick {}", t);
            }
        }
    }
}
