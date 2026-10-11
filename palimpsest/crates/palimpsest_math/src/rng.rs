//! SPEC: canon §3 xoshiro256++, SplitMix64, streams, keyed draws
//!
//! The generator is canon §3.1 verbatim (xoshiro256++ seeded through
//! SplitMix64). Sequential streams and their labeled sub-streams (canon
//! §3.2), keyed and object-keyed draws (canon §3.3), commit-only offer
//! generators (canon §3.5) and the presentation generators (canon §3.8)
//! derive seeds from the run seed, which every function here takes explicitly.

use crate::hash::fnv1a64;

/// SPEC: canon §3.1 SplitMix64 finalizer. Note: `mix64(0) == 0`.
#[inline]
#[must_use]
pub const fn mix64(mut z: u64) -> u64 {
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// SPEC: canon §3.1 SplitMix64 step: advance `state` by the golden gamma and
/// return `mix64(state)`.
#[inline]
pub const fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
    mix64(*state)
}

/// SPEC: canon §3.1 xoshiro256++ generator. Its four state words are part of
/// every snapshot (doc 02 §4.7).
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rng {
    s: [u64; 4],
}

impl Rng {
    /// SPEC: canon §3.1 `from_seed64`: the four state words are four
    /// consecutive `splitmix64` outputs starting from `seed`.
    #[must_use]
    pub const fn from_seed64(seed: u64) -> Self {
        let mut sm = seed;
        let a = splitmix64(&mut sm);
        let b = splitmix64(&mut sm);
        let c = splitmix64(&mut sm);
        let d = splitmix64(&mut sm);
        Rng { s: [a, b, c, d] }
    }

    /// SPEC: canon §2.5 and doc 02 §4.7. Rebuild a generator from its saved
    /// state words (the inverse of [`Rng::state`]).
    #[inline]
    #[must_use]
    pub const fn from_state(s: [u64; 4]) -> Self {
        Rng { s }
    }

    /// SPEC: canon §2.5 and doc 02 §4.7. The four state words, for saves and
    /// the state hash.
    #[inline]
    #[must_use]
    pub const fn state(&self) -> [u64; 4] {
        self.s
    }

    /// SPEC: canon §3.5 `peek`: the next `next_u64` output, read without
    /// advancing the generator.
    #[inline]
    #[must_use]
    pub const fn peek_u64(&self) -> u64 {
        let mut copy = Rng { s: self.s };
        copy.next_u64()
    }

    /// SPEC: canon §3.1 xoshiro256++ `next_u64`. The `s[1] << 17` is the
    /// reference algorithm's deliberate truncating shift (it is how
    /// xoshiro256++ is defined and how §21 tests 1 to 3 are computed), not a
    /// normative `x << k` of canon §1.3.
    #[inline]
    pub const fn next_u64(&mut self) -> u64 {
        let result = (self.s[0].wrapping_add(self.s[3]))
            .rotate_left(23)
            .wrapping_add(self.s[0]);
        let t = self.s[1] << 17;
        self.s[2] ^= self.s[0];
        self.s[3] ^= self.s[1];
        self.s[1] ^= self.s[2];
        self.s[0] ^= self.s[3];
        self.s[2] ^= t;
        self.s[3] = self.s[3].rotate_left(45);
        result
    }

    /// SPEC: canon §3.1 `range`: uniform integer in `[0, n)` by Lemire's
    /// method. May consume more than one `u64` on rejection. Panics if `n == 0`.
    #[track_caller]
    pub fn range(&mut self, n: u64) -> u64 {
        assert!(n > 0, "Rng::range: n must be > 0");
        let mut m = u128::from(self.next_u64()) * u128::from(n);
        let mut l = m as u64; // low 64 bits (intentional truncation)
        if l < n {
            let t = n.wrapping_neg() % n;
            while l < t {
                m = u128::from(self.next_u64()) * u128::from(n);
                l = m as u64;
            }
        }
        (m >> 64) as u64 // high 64 bits, always < n
    }

    /// SPEC: canon §3.1 `range_incl`, §21 test 45: uniform integer in
    /// `lo..=hi`, one `range(hi - lo + 1)` call. Panics unless `lo <= hi` and
    /// `hi - lo < u64::MAX` (the canon's assertion).
    ///
    /// The span and the sum are computed in `i128`, exactly as the canon-1.11
    /// reference line does, so every span the assertion admits works,
    /// `range_incl(i64::MIN, i64::MAX - 1)` included.
    #[track_caller]
    pub fn range_incl(&mut self, lo: i64, hi: i64) -> i64 {
        assert!(
            lo <= hi && (i128::from(hi) - i128::from(lo)) < i128::from(u64::MAX),
            "Rng::range_incl: need lo <= hi and hi - lo < u64::MAX"
        );
        let span = u64::try_from(i128::from(hi) - i128::from(lo) + 1).expect("span fits u64");
        let v = i128::from(lo) + i128::from(self.range(span));
        i64::try_from(v).expect("range_incl: result within lo..=hi")
    }

    /// SPEC: canon §3.1 `roll_bp`: calls `range(10000)` exactly once
    /// regardless of `chance_bp`; true when the draw is below `chance_bp`.
    #[inline]
    #[track_caller]
    pub fn roll_bp(&mut self, chance_bp: i32) -> bool {
        // range(10000) < 10000 always fits i32.
        (self.range(10_000) as i32) < chance_bp
    }

    /// SPEC: canon §3.1 `weighted`: index chosen with probability proportional
    /// to `weights`; calls `range` exactly once. Panics if the total is 0.
    #[track_caller]
    pub fn weighted(&mut self, weights: &[u32]) -> usize {
        let total: u64 = weights.iter().map(|&w| u64::from(w)).sum();
        assert!(total > 0, "Rng::weighted: total weight must be > 0");
        let mut r = self.range(total);
        for (i, &w) in weights.iter().enumerate() {
            if r < u64::from(w) {
                return i;
            }
            r -= u64::from(w);
        }
        unreachable!("Rng::weighted: r < total")
    }

    /// SPEC: canon §3.1 `shuffle`: Fisher-Yates from the last index down; for
    /// `i` from `len-1` down to 1, swap `v[i]` with `v[range(i+1)]`.
    #[track_caller]
    pub fn shuffle<T>(&mut self, v: &mut [T]) {
        for i in (1..v.len()).rev() {
            let j = self.range((i + 1) as u64) as usize; // j <= i < len
            v.swap(i, j);
        }
    }

    /// SPEC: canon §3.1 `bell3`: sum of three uniform draws in `[0, n)`, drawn
    /// left to right. Panics if `n == 0` or the sum overflows.
    #[track_caller]
    pub fn bell3(&mut self, n: u64) -> u64 {
        self.range(n) + self.range(n) + self.range(n)
    }
}

/// SPEC: canon §3.2 `stream_seed(run_seed, name, folio_index) =
/// mix64(run_seed ^ mix64(fnv1a64(name) ^ mix64(folio_index as u64)))`.
/// Run-scoped streams use `folio_index = 0`; Folio indices start at 1.
#[inline]
#[must_use]
pub const fn stream_seed(run_seed: u64, name: &str, folio_index: u32) -> u64 {
    mix64(run_seed ^ mix64(fnv1a64(name.as_bytes()) ^ mix64(folio_index as u64)))
}

/// SPEC: canon §3.2. A sequential stream generator:
/// `Rng::from_seed64(stream_seed(run_seed, name, folio_index))`.
#[inline]
#[must_use]
pub const fn stream_rng(run_seed: u64, name: &str, folio_index: u32) -> Rng {
    Rng::from_seed64(stream_seed(run_seed, name, folio_index))
}

/// SPEC: canon §3.2 sub-streams: `derive(parent, salt) = mix64(parent ^ mix64(salt))`.
/// A sub-stream never advances its parent, so previews may recompute it and
/// stages may run in any order. Owners may chain `derive` calls.
#[inline]
#[must_use]
pub const fn derive(parent: u64, salt: u64) -> u64 {
    mix64(parent ^ mix64(salt))
}

/// SPEC: canon §3.2. The labeled sub-stream salt `fnv1a64(label) ^ index`.
#[inline]
#[must_use]
pub const fn sub_stream_salt(label: &str, index: u64) -> u64 {
    fnv1a64(label.as_bytes()) ^ index
}

/// SPEC: canon §3.2. The seed of the labeled sub-stream `label`, `index` of
/// the sequential stream `name`:
/// `derive(stream_seed(run_seed, name, folio_index), fnv1a64(label) ^ index)`.
/// Owners that document another salt (doc 12 §7.2's `gen_seed` uses a
/// genre's `id32`) call [`derive()`] directly.
#[inline]
#[must_use]
pub const fn sub_stream_seed(
    run_seed: u64,
    name: &str,
    folio_index: u32,
    label: &str,
    index: u64,
) -> u64 {
    derive(
        stream_seed(run_seed, name, folio_index),
        sub_stream_salt(label, index),
    )
}

/// SPEC: canon §3.2. A labeled sub-stream generator:
/// `Rng::from_seed64(sub_stream_seed(...))`.
#[inline]
#[must_use]
pub const fn sub_stream_rng(
    run_seed: u64,
    name: &str,
    folio_index: u32,
    label: &str,
    index: u64,
) -> Rng {
    Rng::from_seed64(sub_stream_seed(run_seed, name, folio_index, label, index))
}

/// SPEC: canon §3.3. The largest keyed `seq`: `seq` is in `0..=0xFFFF`.
pub const KEYED_SEQ_MAX: u32 = 0xFFFF;

/// SPEC: canon §1.3, §3.3 `(seq as u64) << 48` as the checked multiplication
/// by `2^48` that canon §1.3 makes of every normative shift: it panics for
/// `seq > 0xFFFF` instead of dropping bits.
#[inline]
#[must_use]
#[track_caller]
const fn seq_shifted(seq: u32) -> u64 {
    match (seq as u64).checked_mul(1 << 48) {
        Some(v) => v,
        None => panic!("keyed_rng: seq must be in 0..=0xFFFF (canon §3.3)"),
    }
}

/// SPEC: canon §3.3. The seed of a keyed generator:
/// `mix64(stream_seed(run_seed, stream, folio as u64) ^ mix64(actor ^ mix64(tick ^ ((seq as u64) << 48))))`.
/// The shift is checked (canon §1.3), so `seq > 0xFFFF` panics.
#[inline]
#[must_use]
#[track_caller]
pub const fn keyed_seed(
    run_seed: u64,
    stream: &str,
    folio: u32,
    actor: u64,
    tick: u64,
    seq: u32,
) -> u64 {
    mix64(stream_seed(run_seed, stream, folio) ^ mix64(actor ^ mix64(tick ^ seq_shifted(seq))))
}

/// SPEC: canon §3.3 `keyed_rng(stream, folio, actor, tick, seq)`, with
/// `run_seed` as an explicit first parameter (doc 02 §4.7).
///
/// `actor` is the acting entity's `EntityId` (or the killed entity's id or
/// container uid for loot), `tick` the input tick of the action. `seq` is in
/// `0..=0xFFFF` and separates draws that share `(stream, actor, tick)`; it
/// comes from a fixed allocation the drawing document publishes ([`seq`];
/// for loot, the receiving player's slot), never from a count of other
/// entities' draws, and a running per-actor counter is allowed only inside a
/// range its document reserves (for example doc 05's action and hit slots).
/// The result depends only on these arguments, never on how many other draws
/// happened. Panics if `seq > 0xFFFF`.
#[inline]
#[must_use]
#[track_caller]
pub const fn keyed_rng(
    run_seed: u64,
    stream: &str,
    folio: u32,
    actor: u64,
    tick: u64,
    seq: u32,
) -> Rng {
    Rng::from_seed64(keyed_seed(run_seed, stream, folio, actor, tick, seq))
}

/// SPEC: canon §3.3 object-keyed draws: the `tick` argument of an
/// object-keyed generator. `u64::MAX` is never a real tick, so object keys
/// never collide with action keys.
pub const OBJECT_KEY_TICK: u64 = u64::MAX;

/// SPEC: canon §3.3 object-keyed draws: a value that must stay fixed for one
/// object across ticks uses `keyed_rng(stream, (key >> 40) as u32, key,
/// u64::MAX, purpose)`, where `key` is the object's uid or its owner's stable
/// object key (doc 10's `est_key`) and `purpose` is a code the stream's owner
/// allocates (in `0..=0xFFFF`, the `seq` range). `key >> 40` is the uid's
/// Folio field (canon §1.11), at most 24 bits, so the narrowing is exact.
#[inline]
#[must_use]
#[track_caller]
pub const fn object_keyed_rng(run_seed: u64, stream: &str, key: u64, purpose: u32) -> Rng {
    keyed_rng(
        run_seed,
        stream,
        (key >> 40) as u32,
        key,
        OBJECT_KEY_TICK,
        purpose,
    )
}

/// SPEC: canon §3.5. The seed of an offer's generator:
/// `mix64(peek ^ mix64(context_uid ^ fnv1a64(purpose)))`, where `peek` is the
/// next output of `rng.offer.p<slot>` read without advancing it
/// ([`Rng::peek_u64`]), `context_uid` is the uid or `EntityId` of the
/// offering object and `purpose` is the label the owning document fixes.
#[inline]
#[must_use]
pub const fn offer_seed(peek: u64, context_uid: u64, purpose: &str) -> u64 {
    mix64(peek ^ mix64(context_uid ^ fnv1a64(purpose.as_bytes())))
}

/// SPEC: canon §3.5. An offer's generator, `Rng::from_seed64(offer_seed(...))`,
/// built from the offer stream without advancing it. The stream advances by
/// exactly one `next_u64` only when the player commits.
#[inline]
#[must_use]
pub const fn offer_rng(offer_stream: &Rng, context_uid: u64, purpose: &str) -> Rng {
    Rng::from_seed64(offer_seed(offer_stream.peek_u64(), context_uid, purpose))
}

/// SPEC: canon §3.8 presentation generator `nar_rng(run_seed, folio, job,
/// key) = Rng::from_seed64(mix64(stream_seed(run_seed, job, folio as u64) ^
/// mix64(key)))`, where `job` is a `nar.job.` ID. Rendering, audio and
/// narration only; it never reads or advances a simulation stream, and its
/// output never enters simulation state, saves or `state_hash`.
#[inline]
#[must_use]
pub const fn nar_rng(run_seed: u64, folio: u32, job: &str, key: u64) -> Rng {
    Rng::from_seed64(mix64(stream_seed(run_seed, job, folio) ^ mix64(key)))
}

/// SPEC: canon §3.8 `pres_hash(tag, a, b) = mix64(fnv1a64(tag) ^ mix64(a ^
/// mix64(b)))`, the presentation hash (doc 18 §1.2). Presentation only.
#[inline]
#[must_use]
pub const fn pres_hash(tag: &str, a: u64, b: u64) -> u64 {
    mix64(fnv1a64(tag.as_bytes()) ^ mix64(a ^ mix64(b)))
}

/// SPEC: canon §3.3 keyed `seq` allocations reserved by the canon. Each draw
/// takes its `seq` from a fixed allocation its document publishes; a running
/// per-actor counter is allowed only inside a reserved range, and
/// allocations on one stream never overlap.
pub mod seq {
    use core::ops::RangeInclusive;

    /// SPEC: canon §3.3. `0x0000..=0x01FF` on `rng.combat` and `rng.status`:
    /// doc 05 §1.4 action and hit slots.
    pub const ACTION_HIT_SLOTS: RangeInclusive<u32> = 0x0000..=0x01FF;
    /// SPEC: canon §3.3. Doc 06 range `0x2000..=0x2FFF`.
    pub const DOC06_A: RangeInclusive<u32> = 0x2000..=0x2FFF;
    /// SPEC: canon §3.3. Doc 06 single code `0x3000`.
    pub const DOC06_B: u32 = 0x3000;
    /// SPEC: canon §3.3. Doc 06 range `0x4000..=0x4FFF`.
    pub const DOC06_C: RangeInclusive<u32> = 0x4000..=0x4FFF;
    /// SPEC: canon §3.3. `1000..=1006` on `rng.stealth` (doc 09).
    pub const STEALTH_DOC09: RangeInclusive<u32> = 1000..=1006;
    /// SPEC: canon §3.3. `0xFFFE` on `rng.check`: madness episodes (doc 04).
    pub const CHECK_MADNESS: u32 = 0xFFFE;
    /// SPEC: canon §3.3. `0xFFFF` on `rng.combat`: Death's Door (doc 04).
    pub const COMBAT_DEATHS_DOOR: u32 = 0xFFFF;
    /// SPEC: canon §3.3. The per-tick action counters of docs 04 and 10 stay
    /// below `0x2000`.
    pub const ACTION_COUNTER_END: u32 = 0x2000;
    /// SPEC: canon §3.3. Trap and secret detection uses `seq` = the object's
    /// index among objects in range, which stays below 1000.
    pub const TRAP_DETECTION_END: u32 = 1000;
}

/// SPEC: canon §3.2, §3.3, §3.5 and §3.7 stream names.
pub mod streams {
    /// SPEC: canon §3.2 sequential, folio scope, owner doc 12: genre candidates at a Folio exit.
    pub const LATTICE: &str = "rng.lattice";
    /// SPEC: canon §3.2 sequential, run scope, owner doc 10: run-start appearance shuffle.
    pub const APPEARANCE: &str = "rng.appearance";
    /// SPEC: canon §3.2 sequential, folio scope, owner doc 12: layout, rooms, blocks,
    /// dressing, and the counts and positions of loot spots (chests, floor spots, caches).
    pub const WORLDGEN: &str = "rng.worldgen";
    /// SPEC: canon §3.2 sequential, folio scope, owner doc 13: initial population and waves.
    pub const SPAWN: &str = "rng.spawn";
    /// SPEC: canon §3.2 sequential, folio scope, owner doc 11: everything placed in
    /// loot spots and containers, and shop stock.
    pub const LOOT_PLACE: &str = "rng.loot_place";
    /// SPEC: canon §3.2 sequential, folio scope, owner doc 12: ambient and narrative events.
    pub const EVENT: &str = "rng.event";
    /// SPEC: canon §3.2 sequential, folio scope, owner doc 13: idle and wander behavior.
    pub const AI_IDLE: &str = "rng.ai_idle";
    /// SPEC: canon §3.2 sequential, folio scope, owner doc 03: fire ignition and
    /// spread, and explosions' incendiary ignition (§3.3 exception).
    pub const FIRE: &str = "rng.fire";
    /// SPEC: canon §3.2 sequential, folio scope, owner doc 03: random block ticks.
    pub const BLOCKTICK: &str = "rng.blocktick";
    /// SPEC: canon §3.2 sequential, folio scope, owner doc 03: debris, explosion
    /// block-break variation and explosion block drops (§3.3 exception).
    pub const PHYSICS: &str = "rng.physics";
    /// SPEC: canon §3.2 sequential, folio scope, owner doc 14: Daily setup and
    /// Weekly mutators (`folio_index = 0`), marginal notes and Marginal mutations.
    pub const RUN: &str = "rng.run";

    /// SPEC: canon §3.3 keyed, owner doc 05: crits, spread, recoil, Contradiction failure, procs, Death's Door.
    pub const COMBAT: &str = "rng.combat";
    /// SPEC: canon §3.3 keyed, owner doc 05: declared "X Bp chance to apply" effects and confusion.
    pub const STATUS: &str = "rng.status";
    /// SPEC: canon §3.3 keyed, owner doc 06: wild surges, spell variance, cast interruption.
    pub const MAGIC: &str = "rng.magic";
    /// SPEC: canon §3.3 keyed, owner doc 09: detection, pickpocket, lockpicking, traps, secrets.
    pub const STEALTH: &str = "rng.stealth";
    /// SPEC: canon §3.3 keyed, owner doc 10: appraisal error, curse sensing.
    pub const IDENT: &str = "rng.ident";
    /// SPEC: canon §3.3 keyed, owner doc 07: crafting quality.
    pub const CRAFT: &str = "rng.craft";
    /// SPEC: canon §3.3 keyed, owner doc 08: brewing variance, unknown liquids.
    pub const ALCHEMY: &str = "rng.alchemy";
    /// SPEC: canon §3.3 keyed, owner doc 11: kill and container drops, Reliquary
    /// contents, Fortune Engine dispensing.
    pub const LOOT: &str = "rng.loot";
    /// SPEC: canon §3.3 keyed, owner doc 13: utility AI tie-breaks.
    pub const AI: &str = "rng.ai";
    /// SPEC: canon §3.3 keyed, owner doc 04: talent offers and level-up choices.
    pub const PROGRESS: &str = "rng.progress";
    /// SPEC: canon §3.3 keyed, owner doc 04: non-combat skill checks (doc 04
    /// §4.2) and madness episodes. Disguise challenges are AI detection rolls
    /// and use `rng.stealth` (doc 09).
    pub const CHECK: &str = "rng.check";

    /// SPEC: canon §3.2. Every sequential stream name, in canon table order.
    pub const SEQUENTIAL: [&str; 11] = [
        LATTICE, APPEARANCE, WORLDGEN, SPAWN, LOOT_PLACE, EVENT, AI_IDLE, FIRE, BLOCKTICK, PHYSICS,
        RUN,
    ];
    /// SPEC: canon §3.3. Every keyed stream name, in canon table order.
    pub const KEYED: [&str; 11] = [
        COMBAT, STATUS, MAGIC, STEALTH, IDENT, CRAFT, ALCHEMY, LOOT, AI, PROGRESS, CHECK,
    ];

    /// SPEC: canon §3.2, §3.3. The owning document of a canon stream (the
    /// "Owner" column), `None` for names outside both tables.
    #[must_use]
    pub fn owner(name: &str) -> Option<u8> {
        Some(match name {
            LATTICE | WORLDGEN | EVENT => 12,
            APPEARANCE | IDENT => 10,
            SPAWN | AI_IDLE | AI => 13,
            LOOT_PLACE | LOOT => 11,
            FIRE | BLOCKTICK | PHYSICS => 3,
            RUN => 14,
            COMBAT | STATUS => 5,
            MAGIC => 6,
            STEALTH => 9,
            CRAFT => 7,
            ALCHEMY => 8,
            PROGRESS | CHECK => 4,
            _ => return None,
        })
    }

    /// SPEC: canon §3.5. The run-scoped commit-only offer stream of party
    /// slot `slot` (`rng.offer.p0` to `rng.offer.p3`). Panics for `slot > 3`.
    #[must_use]
    #[track_caller]
    pub const fn offer(slot: u8) -> &'static str {
        match slot {
            0 => "rng.offer.p0",
            1 => "rng.offer.p1",
            2 => "rng.offer.p2",
            3 => "rng.offer.p3",
            _ => panic!("streams::offer: party slot must be 0..=3"),
        }
    }

    /// SPEC: canon §3.7. `true` when `name` is a mod stream
    /// `rng.mod.<mod_id>.<name>` (two non-empty segments after `rng.mod.`).
    /// Does not validate the content-ID grammar of the segments.
    #[must_use]
    pub fn is_mod_stream(name: &str) -> bool {
        match name.strip_prefix("rng.mod.") {
            Some(rest) => match rest.split_once('.') {
                Some((m, n)) => !m.is_empty() && !n.is_empty(),
                None => false,
            },
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Rng;
    use super::*;
    use proptest::prelude::*;

    const RUN: u64 = 0x0123_4567_89AB_CDEF;

    #[test]
    fn canon_t01_xoshiro_vectors() {
        // Canon §21 test 1.
        let mut r = Rng::from_seed64(0);
        assert_eq!(r.next_u64(), 0x5317_5d61_490b_23df);
        assert_eq!(r.next_u64(), 0x61da_6f3d_c380_d507);
        assert_eq!(r.next_u64(), 0x5c0f_df91_ec9a_7bfc);
        assert_eq!(r.next_u64(), 0x02ee_bf8c_3bbe_5e1a);
    }

    #[test]
    fn canon_t02_second_seed() {
        // Canon §21 test 2.
        let mut r = Rng::from_seed64(0x0123_4567_89AB_CDEF);
        assert_eq!(r.next_u64(), 0xb2f2_a310_e96b_d1c5);
        assert_eq!(r.next_u64(), 0xb540_6246_5b95_0493);
        assert_eq!(r.next_u64(), 0x87ac_a4a9_6688_14b0);
        assert_eq!(r.next_u64(), 0xf13d_2e24_48a9_cffb);
    }

    #[test]
    fn canon_t03_lemire_range() {
        // Canon §21 test 3.
        let mut r = Rng::from_seed64(42);
        let v: [u64; 5] = core::array::from_fn(|_| r.range(10_000));
        assert_eq!(v, [8143, 3188, 9838, 7011, 7935]);
    }

    #[test]
    fn canon_t05_mix64() {
        // Canon §21 test 5.
        assert_eq!(mix64(0), 0);
        assert_eq!(mix64(1), 0x5692_161d_100b_05e5);
    }

    #[test]
    fn canon_t06_stream_seed() {
        // Canon §21 test 6.
        assert_eq!(stream_seed(RUN, "rng.loot", 3), 0x3050_a071_1590_9f1c);
        let mut r = stream_rng(RUN, streams::LOOT, 3);
        assert_eq!(r.next_u64(), 0x5bdc_091e_897b_818b);
        assert_eq!(r.next_u64(), 0x53a8_da29_d781_85d1);
        assert_eq!(r.next_u64(), 0x1fc0_2fe2_ca3d_6c88);
        assert_eq!(stream_seed(RUN, "rng.worldgen", 1), 0x276f_33cc_0100_c222);
    }

    #[test]
    fn canon_t07_keyed_draw() {
        // Canon §21 test 7.
        let actor = (3u64 << 40) | 5;
        assert_eq!(
            keyed_seed(RUN, "rng.combat", 3, actor, 1000, 0),
            0x4dcd_b61b_ac36_2a8f
        );
        let mut r = keyed_rng(RUN, streams::COMBAT, 3, actor, 1000, 0);
        assert_eq!(r, Rng::from_seed64(0x4dcd_b61b_ac36_2a8f));
        assert_eq!(r.range(10_000), 8822);
        assert_eq!(r.range(10_000), 5948);
        assert_eq!(r.range(10_000), 1792);
    }

    /// One "entity" drawing its crit roll from `rng.combat` on tick 1000.
    fn crit_roll(actor: u64, seq: u32) -> u64 {
        keyed_rng(RUN, streams::COMBAT, 3, actor, 1000, seq).range(10_000)
    }

    #[test]
    fn canon_t08_keyed_isolation() {
        // Canon §21 test 8: the crit roll for (actor (3 << 40) | 5, tick 1000,
        // seq 0) is identical whether 0 or 50 other entities drew from
        // rng.combat earlier in the tick.
        let actor = (3u64 << 40) | 5;
        let alone = crit_roll(actor, 0);
        let mut others = [0u64; 50];
        for (i, o) in others.iter_mut().enumerate() {
            let other_actor = (3u64 << 40) | (100 + i as u64);
            // Each other entity draws several values first.
            let mut g = keyed_rng(RUN, streams::COMBAT, 3, other_actor, 1000, 0);
            for _ in 0..(i % 7) {
                let _ = g.next_u64();
            }
            *o = g.range(10_000);
        }
        let after_50 = crit_roll(actor, 0);
        assert_eq!(alone, after_50);
        assert_eq!(alone, 8822);
        // Different keys do give different generators.
        assert_ne!(
            keyed_seed(RUN, "rng.combat", 3, actor, 1000, 1),
            keyed_seed(RUN, "rng.combat", 3, actor, 1000, 0)
        );
        assert_ne!(
            keyed_seed(RUN, "rng.combat", 3, actor, 1001, 0),
            keyed_seed(RUN, "rng.combat", 3, actor, 1000, 0)
        );
        assert_ne!(
            keyed_seed(RUN, "rng.loot", 3, actor, 1000, 0),
            keyed_seed(RUN, "rng.combat", 3, actor, 1000, 0)
        );
        assert_ne!(
            keyed_seed(RUN, "rng.combat", 4, actor, 1000, 0),
            keyed_seed(RUN, "rng.combat", 3, actor, 1000, 0)
        );
    }

    #[test]
    fn splitmix_sequence() {
        let mut s = 0u64;
        assert_eq!(splitmix64(&mut s), mix64(0x9E37_79B9_7F4A_7C15));
        assert_eq!(s, 0x9E37_79B9_7F4A_7C15);
        // Published SplitMix64 vector for seed 0: first output 0xe220a8397b1dcdaf.
        let mut s = 0u64;
        assert_eq!(splitmix64(&mut s), 0xe220_a839_7b1d_cdaf);
    }

    #[test]
    fn state_round_trip() {
        let mut a = Rng::from_seed64(7);
        let _ = a.next_u64();
        let mut b = Rng::from_state(a.state());
        assert_eq!(a.next_u64(), b.next_u64());
        assert_eq!(a, b);
    }

    #[test]
    fn range_edge_cases() {
        let mut r = Rng::from_seed64(1);
        for _ in 0..100 {
            assert_eq!(r.range(1), 0);
        }
        let mut a = Rng::from_seed64(9);
        let mut b = Rng::from_seed64(9);
        // n = 2^64 - 1 is valid; the result is the high word of x * n.
        let x = b.next_u64();
        let expect = ((u128::from(x) * u128::from(u64::MAX)) >> 64) as u64;
        let l = (u128::from(x) * u128::from(u64::MAX)) as u64;
        if l >= 1 {
            assert_eq!(a.range(u64::MAX), expect);
        }
        // Power of two: no rejection ever (t = 0).
        let mut c = Rng::from_seed64(11);
        let mut d = Rng::from_seed64(11);
        assert_eq!(c.range(1 << 32), d.next_u64() >> 32);
    }

    #[test]
    #[should_panic(expected = "n must be > 0")]
    fn range_zero_panics() {
        let _ = Rng::from_seed64(0).range(0);
    }

    #[test]
    fn range_incl_examples() {
        let mut r = Rng::from_seed64(42);
        let mut s = Rng::from_seed64(42);
        assert_eq!(r.range_incl(-5, 5), -5 + s.range(11) as i64);
        assert_eq!(r.range_incl(3, 3), 3);
        // Spans wider than i64::MAX are handled, as the canon-1.11 i128 reference line does.
        let mut a = Rng::from_seed64(5);
        let mut b = Rng::from_seed64(5);
        let v = a.range_incl(i64::MIN, i64::MAX - 1);
        let expect = i128::from(i64::MIN) + i128::from(b.range(u64::MAX));
        assert_eq!(i128::from(v), expect);
    }

    #[test]
    #[should_panic(expected = "need lo <= hi")]
    fn range_incl_full_span_panics() {
        let _ = Rng::from_seed64(0).range_incl(i64::MIN, i64::MAX);
    }

    #[test]
    #[should_panic(expected = "need lo <= hi")]
    fn range_incl_inverted_panics() {
        let _ = Rng::from_seed64(0).range_incl(1, 0);
    }

    #[test]
    fn roll_bp_draws_once() {
        for chance in [-5, 0, 1, 5000, 9999, 10_000, 20_000] {
            let mut a = Rng::from_seed64(42);
            let mut b = Rng::from_seed64(42);
            let hit = a.roll_bp(chance);
            let draw = b.range(10_000) as i32;
            assert_eq!(hit, draw < chance);
            assert_eq!(a, b, "roll_bp must call range(10000) exactly once");
        }
        // Seed 42's first range(10000) is 8143 (canon §21 test 3).
        assert!(Rng::from_seed64(42).roll_bp(8144));
        assert!(!Rng::from_seed64(42).roll_bp(8143));
        assert!(!Rng::from_seed64(42).roll_bp(0));
        assert!(Rng::from_seed64(42).roll_bp(10_000));
    }

    #[test]
    fn weighted_examples() {
        // Seed 42 first range(10000) = 8143: with weights [7000, 2000, 1000] -> index 1.
        let mut r = Rng::from_seed64(42);
        assert_eq!(r.weighted(&[7000, 2000, 1000]), 1);
        // Zero weights are never chosen.
        let mut r = Rng::from_seed64(3);
        for _ in 0..1000 {
            assert_eq!(r.weighted(&[0, 5, 0]), 1);
        }
        // Draws exactly once.
        let mut a = Rng::from_seed64(8);
        let mut b = Rng::from_seed64(8);
        let _ = a.weighted(&[1, 2, 3]);
        let _ = b.range(6);
        assert_eq!(a, b);
    }

    #[test]
    #[should_panic(expected = "total weight must be > 0")]
    fn weighted_zero_total_panics() {
        let _ = Rng::from_seed64(0).weighted(&[0, 0]);
    }

    #[test]
    fn shuffle_reference() {
        // Reference: Fisher-Yates from the last index down.
        let mut a = Rng::from_seed64(42);
        let mut b = Rng::from_seed64(42);
        let mut v = [0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        a.shuffle(&mut v);
        let mut w = [0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        for i in (1..w.len()).rev() {
            let j = b.range(i as u64 + 1) as usize;
            w.swap(i, j);
        }
        assert_eq!(v, w);
        let mut sorted = v;
        sorted.sort_unstable();
        assert_eq!(sorted, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        // Empty and single-element slices draw nothing.
        let mut c = Rng::from_seed64(1);
        let before = c.clone();
        c.shuffle::<u8>(&mut []);
        c.shuffle(&mut [1u8]);
        assert_eq!(c, before);
    }

    #[test]
    fn bell3_reference() {
        let mut a = Rng::from_seed64(42);
        // 8143 + 3188 + 9838 from canon §21 test 3's draws.
        assert_eq!(a.bell3(10_000), 8143 + 3188 + 9838);
    }

    /// SPEC: canon §21 test 45. `Rng::from_seed64(42).range_incl(i64::MIN,
    /// i64::MAX - 1)` returns 5797906573132458142 without overflow; three calls
    /// of `range_incl(-5, 5)` on a fresh `Rng::from_seed64(42)` return 3, -2, 5.
    #[test]
    fn canon_t45_range_incl_extremes() {
        assert_eq!(
            Rng::from_seed64(42).range_incl(i64::MIN, i64::MAX - 1),
            5_797_906_573_132_458_142
        );
        let mut r = Rng::from_seed64(42);
        let v: [i64; 3] = core::array::from_fn(|_| r.range_incl(-5, 5));
        assert_eq!(v, [3, -2, 5]);
    }

    /// SPEC: canon §21 test 47. For run seed `0x0123456789ABCDEF`,
    /// `derive(stream_seed(run, "rng.worldgen", 2), fnv1a32("genre.space_opera"))
    /// = 0x1c35d41d581f5eaa` (doc 12 §7.2's `gen_seed` example).
    #[test]
    fn canon_t47_sub_stream() {
        let salt = u64::from(crate::hash::fnv1a32(b"genre.space_opera"));
        assert_eq!(
            derive(stream_seed(RUN, "rng.worldgen", 2), salt),
            0x1c35_d41d_581f_5eaa
        );
    }

    /// SPEC: canon §3.2 labeled sub-streams: `derive(parent, fnv1a64(label) ^
    /// index)`; a sub-stream never advances its parent.
    #[test]
    fn labeled_sub_streams() {
        let parent = stream_seed(RUN, streams::LOOT_PLACE, 4);
        let s = sub_stream_seed(RUN, streams::LOOT_PLACE, 4, "chest", 7);
        assert_eq!(
            s,
            mix64(parent ^ mix64(fnv1a64(b"chest") ^ 7)),
            "derive(parent, salt) = mix64(parent ^ mix64(salt))"
        );
        assert_eq!(
            sub_stream_rng(RUN, streams::LOOT_PLACE, 4, "chest", 7),
            Rng::from_seed64(s)
        );
        assert_ne!(s, sub_stream_seed(RUN, streams::LOOT_PLACE, 4, "chest", 8));
        assert_ne!(s, sub_stream_seed(RUN, streams::LOOT_PLACE, 4, "shop", 7));
        // Deriving touches no generator: the parent stream is unchanged.
        let mut p = stream_rng(RUN, streams::LOOT_PLACE, 4);
        let before = p.clone();
        let _ = sub_stream_rng(RUN, streams::LOOT_PLACE, 4, "chest", 7);
        assert_eq!(p, before);
        assert_eq!(p.next_u64(), Rng::from_seed64(parent).next_u64());
        // Chains of derive are allowed (doc 11 place_rng).
        assert_eq!(
            derive(derive(parent, 1), 2),
            mix64(derive(parent, 1) ^ mix64(2))
        );
    }

    /// SPEC: canon §3.3: `seq` is in `0..=0xFFFF`; `0xFFFF` is valid.
    #[test]
    fn keyed_seq_upper_bound_is_valid() {
        let actor = (3u64 << 40) | 5;
        let a = keyed_seed(RUN, streams::COMBAT, 3, actor, 1000, KEYED_SEQ_MAX);
        let b = mix64(
            stream_seed(RUN, "rng.combat", 3) ^ mix64(actor ^ mix64(1000 ^ (0xFFFFu64 << 48))),
        );
        assert_eq!(a, b);
    }

    /// SPEC: canon §1.3, §3.3: a `seq` above `0xFFFF` panics in the checked shift.
    #[test]
    #[should_panic(expected = "seq must be in 0..=0xFFFF")]
    fn keyed_seq_above_range_panics() {
        let _ = keyed_seed(RUN, streams::COMBAT, 3, 5, 1000, 0x1_0000);
    }

    /// SPEC: canon §3.3 object-keyed draws: `keyed_rng(stream, (key >> 40) as
    /// u32, key, u64::MAX, purpose)`.
    #[test]
    fn object_keyed_draws() {
        let uid = (7u64 << 40) | 42;
        assert_eq!(
            object_keyed_rng(RUN, streams::LOOT, uid, 3),
            keyed_rng(RUN, streams::LOOT, 7, uid, u64::MAX, 3)
        );
        // Never equal to the action key at any real tick of the same object.
        assert_ne!(
            object_keyed_rng(RUN, streams::LOOT, uid, 3),
            keyed_rng(RUN, streams::LOOT, 7, uid, 1000, 3)
        );
    }

    /// SPEC: canon §3.3: the canon's reserved `seq` allocations stay inside
    /// `0..=0xFFFF` and never overlap on one stream (`rng.combat` holds the
    /// action and hit slots and Death's Door; doc 06's ranges are disjoint
    /// from them; the action counters of docs 04 and 10 stay below `0x2000`).
    #[test]
    fn reserved_seq_allocations_are_disjoint() {
        let combat: [(u32, u32); 5] = [
            (*seq::ACTION_HIT_SLOTS.start(), *seq::ACTION_HIT_SLOTS.end()),
            (*seq::DOC06_A.start(), *seq::DOC06_A.end()),
            (seq::DOC06_B, seq::DOC06_B),
            (*seq::DOC06_C.start(), *seq::DOC06_C.end()),
            (seq::COMBAT_DEATHS_DOOR, seq::COMBAT_DEATHS_DOOR),
        ];
        for w in combat.windows(2) {
            assert!(w[0].1 < w[1].0, "{w:?} overlap");
        }
        assert!(combat.iter().all(|r| r.1 <= KEYED_SEQ_MAX));
        assert!(*seq::ACTION_HIT_SLOTS.end() < seq::ACTION_COUNTER_END);
        assert!(seq::ACTION_COUNTER_END <= *seq::DOC06_A.start());
        assert!(*seq::STEALTH_DOC09.start() >= seq::TRAP_DETECTION_END);
        assert_eq!(seq::CHECK_MADNESS, 0xFFFE);
    }

    /// SPEC: canon §3.5 offer generator `Rng::from_seed64(mix64(peek ^
    /// mix64(context_uid ^ fnv1a64(purpose))))`; building it never advances
    /// the offer stream, and a commit advances it by exactly one `next_u64`.
    #[test]
    fn offer_generator() {
        let mut stream = stream_rng(RUN, streams::offer(1), 0);
        let before = stream.clone();
        let peek = stream.peek_u64();
        let g = offer_rng(&stream, 99, "offer.reforge");
        assert_eq!(stream, before, "peeking never advances the stream");
        assert_eq!(
            g,
            Rng::from_seed64(mix64(peek ^ mix64(99 ^ fnv1a64(b"offer.reforge"))))
        );
        // Re-opening shows the same offer.
        assert_eq!(offer_rng(&stream, 99, "offer.reforge"), g);
        // Commit: one next_u64, which is the peeked value.
        assert_eq!(stream.next_u64(), peek);
        assert_ne!(offer_rng(&stream, 99, "offer.reforge"), g);
    }

    /// SPEC: canon §3.8 presentation generators `nar_rng` and `pres_hash`.
    #[test]
    fn presentation_generators() {
        assert_eq!(
            nar_rng(RUN, 3, "nar.job.marginalia", 17),
            Rng::from_seed64(mix64(stream_seed(RUN, "nar.job.marginalia", 3) ^ mix64(17)))
        );
        assert_eq!(
            pres_hash("vfx.sparks", 5, 9),
            mix64(fnv1a64(b"vfx.sparks") ^ mix64(5 ^ mix64(9)))
        );
    }

    /// Review (canon §3.1, §21 test 45): the other extremes of `range_incl`.
    /// `range_incl(i64::MIN + 1, i64::MAX)` (span `u64::MAX`, top end)
    /// returns test 45's value plus one, 5797906573132458143 (exact Python);
    /// one-point ranges at both ends of `i64` draw once and return the point.
    #[test]
    fn range_incl_top_and_point_extremes() {
        assert_eq!(
            Rng::from_seed64(42).range_incl(i64::MIN + 1, i64::MAX),
            5_797_906_573_132_458_143
        );
        let mut c = Rng::from_seed64(7);
        let mut d = Rng::from_seed64(7);
        assert_eq!(c.range_incl(i64::MAX, i64::MAX), i64::MAX);
        assert_eq!(c.range_incl(i64::MIN, i64::MIN), i64::MIN);
        let _ = d.next_u64();
        let _ = d.next_u64();
        assert_eq!(c, d, "a one-point range still draws exactly once");
    }

    /// Review (canon §3.2, §3.3, §3.5, §3.8): seeds of the canon-1.11
    /// generators, pinned to values computed independently in exact Python
    /// (run seed `0x0123456789ABCDEF`), so a change to any formula fails here
    /// even if it is mirrored in the formula-shaped tests above.
    #[test]
    fn canon_1_11_generator_vectors() {
        // §3.3: seq = 0xFFFF on test 7's key.
        let actor = (3u64 << 40) | 5;
        assert_eq!(
            keyed_seed(RUN, streams::COMBAT, 3, actor, 1000, KEYED_SEQ_MAX),
            0x369c_4ec9_b818_d388
        );
        assert_eq!(
            keyed_rng(RUN, streams::COMBAT, 3, actor, 1000, 0xFFFF).range(10_000),
            8921
        );
        // §3.3 object-keyed: uid (7 << 40) | 42, purpose 3 on rng.loot.
        assert_eq!(
            object_keyed_rng(RUN, streams::LOOT, (7 << 40) | 42, 3),
            Rng::from_seed64(0xd797_1816_5ae9_4c6b)
        );
        // The largest Folio field (2^24 - 1) narrows exactly.
        assert_eq!(
            object_keyed_rng(RUN, streams::IDENT, (0xFF_FFFF << 40) | 1, 0xFFFF),
            Rng::from_seed64(0xcc44_b14f_9a91_78b9)
        );
        // §3.2 labeled sub-stream "chest" #7 of rng.loot_place, Folio 4.
        assert_eq!(fnv1a64(b"chest"), 0x82d5_d676_36f6_9a16);
        assert_eq!(
            sub_stream_seed(RUN, streams::LOOT_PLACE, 4, "chest", 7),
            0x49a0_120b_666d_c4b4
        );
        // §3.5: rng.offer.p1's peek and the "offer.reforge" offer for uid 99.
        let stream = stream_rng(RUN, streams::offer(1), 0);
        assert_eq!(stream.peek_u64(), 0x52d0_4a64_6e70_7c3c);
        assert_eq!(
            offer_seed(stream.peek_u64(), 99, "offer.reforge"),
            0x07de_032a_2229_f0b9
        );
        // §3.8 presentation generators.
        assert_eq!(
            nar_rng(RUN, 3, "nar.job.marginalia", 17),
            Rng::from_seed64(0xba51_19ea_f8c5_6fbc)
        );
        assert_eq!(pres_hash("vfx.sparks", 5, 9), 0xc8cf_752a_0d18_70a0);
    }

    /// Review (canon §1.3, §3.3): `u32::MAX` is far outside `0..=0xFFFF`;
    /// `keyed_rng` (not only `keyed_seed`) refuses it.
    #[test]
    #[should_panic(expected = "seq must be in 0..=0xFFFF")]
    fn keyed_rng_rejects_u32_max_seq() {
        let _ = keyed_rng(RUN, streams::COMBAT, 3, 5, 1000, u32::MAX);
    }

    /// Review (canon §3.3): an object-keyed `purpose` is a `seq`, so it is
    /// bounded by `0xFFFF` too.
    #[test]
    #[should_panic(expected = "seq must be in 0..=0xFFFF")]
    fn object_keyed_purpose_above_range_panics() {
        let _ = object_keyed_rng(RUN, streams::LOOT, (7 << 40) | 42, 0x1_0000);
    }

    /// Review (canon §3.2, §3.3): the stream names, their order and their
    /// owners equal the canon tables. Skipped when the spec is not next to
    /// the workspace (a standalone export).
    #[test]
    fn stream_tables_match_canon_text() {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../docs/palimpsest/spec/00_CANON.md"
        );
        let Ok(canon) = std::fs::read_to_string(path) else {
            std::eprintln!("skipped: {path} not found");
            return;
        };
        let mut sequential = std::vec::Vec::new();
        let mut keyed = std::vec::Vec::new();
        for line in canon.lines() {
            let Some(inner) = line.strip_prefix('|').and_then(|r| r.strip_suffix('|')) else {
                continue;
            };
            let c: std::vec::Vec<&str> = inner.split('|').map(str::trim).collect();
            if !c[0].starts_with("`rng.") {
                continue;
            }
            let name = c[0].trim_matches('`');
            // §3.2: | name | scope | owner | draws for |; §3.3: | name | owner | used for |
            match c.len() {
                4 if matches!(c[1], "folio" | "run") => {
                    sequential.push((name, c[2].parse::<u8>().expect("owner")));
                }
                3 => keyed.push((name, c[1].parse::<u8>().expect("owner"))),
                _ => {}
            }
        }
        let seq_names: std::vec::Vec<&str> = sequential.iter().map(|r| r.0).collect();
        let keyed_names: std::vec::Vec<&str> = keyed.iter().map(|r| r.0).collect();
        assert_eq!(seq_names, streams::SEQUENTIAL);
        assert_eq!(keyed_names, streams::KEYED);
        for (name, owner) in sequential.iter().chain(keyed.iter()) {
            assert_eq!(streams::owner(name), Some(*owner), "{name}");
        }
    }

    #[test]
    fn stream_names() {
        assert_eq!(streams::offer(2), "rng.offer.p2");
        assert_eq!(streams::SEQUENTIAL.len(), 11);
        assert_eq!(streams::KEYED.len(), 11);
        // Canon §3.2 and §3.3 table order: rng.run and rng.check come last.
        assert_eq!(streams::SEQUENTIAL[10], "rng.run");
        assert_eq!(streams::KEYED[10], "rng.check");
        assert_eq!(streams::owner(streams::RUN), Some(14));
        assert_eq!(streams::owner(streams::CHECK), Some(4));
        assert_eq!(streams::owner(streams::PROGRESS), Some(4));
        assert_eq!(streams::owner("rng.offer.p0"), None);
        for name in streams::SEQUENTIAL.iter().chain(streams::KEYED.iter()) {
            assert!(streams::owner(name).is_some(), "{name}");
        }
        assert!(streams::is_mod_stream("rng.mod.mymod.sparks"));
        assert!(!streams::is_mod_stream("rng.mod.mymod"));
        assert!(!streams::is_mod_stream("rng.mod..x"));
        assert!(!streams::is_mod_stream("rng.combat"));
        // Folio streams for different Folios are distinct; run-scoped uses folio 0.
        assert_ne!(
            stream_seed(RUN, streams::WORLDGEN, 1),
            stream_seed(RUN, streams::WORLDGEN, 2)
        );
        assert_ne!(
            stream_seed(RUN, streams::APPEARANCE, 0),
            stream_seed(RUN, streams::WORLDGEN, 0)
        );
        // Every stream name is distinct under fnv1a64.
        let mut all: std::vec::Vec<u64> = streams::SEQUENTIAL
            .iter()
            .chain(streams::KEYED.iter())
            .map(|n| fnv1a64(n.as_bytes()))
            .collect();
        all.sort_unstable();
        all.dedup();
        assert_eq!(all.len(), 22);
    }

    #[test]
    #[should_panic(expected = "party slot must be 0..=3")]
    fn offer_slot_out_of_range_panics() {
        let _ = streams::offer(4);
    }

    proptest! {
        #[test]
        fn range_in_bounds(seed in any::<u64>(), n in 1u64..=u64::MAX) {
            let mut r = Rng::from_seed64(seed);
            prop_assert!(r.range(n) < n);
        }

        #[test]
        fn range_incl_in_bounds(seed in any::<u64>(), lo in any::<i64>(), w in 0u64..u64::MAX) {
            let hi = i64::try_from((i128::from(lo) + i128::from(w)).min(i128::from(i64::MAX))).unwrap();
            prop_assume!((i128::from(hi) - i128::from(lo)) < i128::from(u64::MAX));
            let mut r = Rng::from_seed64(seed);
            let v = r.range_incl(lo, hi);
            prop_assert!(lo <= v && v <= hi);
        }

        #[test]
        fn range_incl_matches_canon_reference(seed in any::<u64>(), lo in -(1i64 << 40)..(1i64 << 40), w in 0i64..(1i64 << 40)) {
            // The canon-1.10 i64 reference body, which equals canon-1.11's where it cannot overflow.
            let hi = lo + w;
            let mut a = Rng::from_seed64(seed);
            let mut b = Rng::from_seed64(seed);
            prop_assert_eq!(a.range_incl(lo, hi), lo + b.range((hi - lo + 1) as u64) as i64);
        }

        #[test]
        fn range_incl_matches_canon_i128_reference(seed in any::<u64>(), lo in any::<i64>(), hi in any::<i64>()) {
            // Canon §3.1 (canon-1.11) reference line, computed in i128, over the whole domain.
            let (lo, hi) = if lo <= hi { (lo, hi) } else { (hi, lo) };
            prop_assume!((i128::from(hi) - i128::from(lo)) < i128::from(u64::MAX));
            let mut a = Rng::from_seed64(seed);
            let mut b = Rng::from_seed64(seed);
            let span = (i128::from(hi) - i128::from(lo) + 1) as u64;
            let expect = (i128::from(lo) + i128::from(b.range(span))) as i64;
            prop_assert_eq!(a.range_incl(lo, hi), expect);
        }

        #[test]
        fn determinism(seed in any::<u64>()) {
            let mut a = Rng::from_seed64(seed);
            let mut b = Rng::from_seed64(seed);
            for _ in 0..16 {
                prop_assert_eq!(a.next_u64(), b.next_u64());
            }
        }

        /// Review (canon §3.5): `peek_u64` is the next output and never
        /// advances the generator.
        #[test]
        fn peek_is_next_without_advancing(seed in any::<u64>(), skip in 0usize..8) {
            let mut a = Rng::from_seed64(seed);
            for _ in 0..skip {
                let _ = a.next_u64();
            }
            let before = a.clone();
            let p = a.peek_u64();
            prop_assert_eq!(&a, &before);
            prop_assert_eq!(a.next_u64(), p);
        }

        /// Review (canon §1.3, §3.3): inside `0..=0xFFFF` the checked shift
        /// equals the plain `(seq as u64) << 48` of the canon reference.
        #[test]
        fn keyed_seq_checked_shift_matches_plain(seq in 0u32..=0xFFFF, actor in any::<u64>(), tick in any::<u64>()) {
            let plain = mix64(
                stream_seed(RUN, streams::STATUS, 2) ^ mix64(actor ^ mix64(tick ^ (u64::from(seq) << 48))),
            );
            prop_assert_eq!(keyed_seed(RUN, streams::STATUS, 2, actor, tick, seq), plain);
        }

        /// Review (canon §3.3): an object key's generator is the keyed
        /// generator at Folio `key >> 40` and tick `u64::MAX`, and never equals
        /// the same object's action draw at a real tick.
        #[test]
        fn object_keys_use_folio_field(key in any::<u64>(), purpose in 0u32..=0xFFFF, tick in 0u64..u64::MAX) {
            let folio = u32::try_from(key >> 40).unwrap();
            let obj = object_keyed_rng(RUN, streams::LOOT, key, purpose);
            prop_assert_eq!(&obj, &keyed_rng(RUN, streams::LOOT, folio, key, u64::MAX, purpose));
            prop_assert_ne!(&obj, &keyed_rng(RUN, streams::LOOT, folio, key, tick, purpose));
        }

        #[test]
        fn weighted_in_bounds_and_nonzero(seed in any::<u64>(), w in proptest::collection::vec(0u32..1000, 1..16)) {
            prop_assume!(w.iter().any(|&x| x > 0));
            let mut r = Rng::from_seed64(seed);
            let i = r.weighted(&w);
            prop_assert!(i < w.len());
            prop_assert!(w[i] > 0);
        }

        #[test]
        fn shuffle_is_permutation(seed in any::<u64>(), mut v in proptest::collection::vec(any::<u16>(), 0..64)) {
            let mut sorted = v.clone();
            sorted.sort_unstable();
            Rng::from_seed64(seed).shuffle(&mut v);
            v.sort_unstable();
            prop_assert_eq!(v, sorted);
        }

        #[test]
        fn bell3_in_bounds(seed in any::<u64>(), n in 1u64..(1u64 << 60)) {
            let mut r = Rng::from_seed64(seed);
            prop_assert!(r.bell3(n) <= 3 * (n - 1));
        }

        #[test]
        fn keyed_isolation_property(
            run in any::<u64>(), folio in 1u32..30, actor in any::<u64>(), tick in any::<u64>(), seq in 0u32..8,
            noise in proptest::collection::vec((any::<u64>(), 0u32..8, 0usize..5), 0..50),
        ) {
            let first = keyed_rng(run, streams::COMBAT, folio, actor, tick, seq).range(10_000);
            for (other, s, k) in noise {
                let mut g = keyed_rng(run, streams::COMBAT, folio, other, tick, s);
                for _ in 0..k {
                    let _ = g.next_u64();
                }
            }
            let again = keyed_rng(run, streams::COMBAT, folio, actor, tick, seq).range(10_000);
            prop_assert_eq!(first, again);
        }
    }
}
