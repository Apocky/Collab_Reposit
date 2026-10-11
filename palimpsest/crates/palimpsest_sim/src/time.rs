//! SPEC: canon §2.2-2.3 tick schedule, durations, accumulators
//!
//! - The canon §2.2 tick-schedule table as const data, with [`runs_on`]
//!   deciding whether a subsystem runs on a tick (per-entity phases use the
//!   canon §1.11 phase key, [`EntityId::phase_key`]).
//! - "N s" durations (§2.3) and the general rate accumulator with
//!   denominator `k = 60 / period` ([`RateAcc`]).
//! - Status timers: applied tick, active interval `a+1 ..= a+D`, end-of-tick
//!   expiry, refresh to `max(old_expiry, t + D)`, periodic sub-effects
//!   ([`StatusTimer`]).
//! - "After N s without X" ([`after_secs_without`]) and the `in_combat`
//!   predicate over caller-provided last-event ticks ([`CombatTicks`]).
//!
//! All arithmetic is integer and overflow-checked (canon §1.3). Tick sums that
//! could exceed `u64` are computed modulo the period, which is the same value.

use crate::entity::EntityId;
use palimpsest_math::int::{floor_div, narrow_i64};

/// SPEC: canon §1.2 `Tick`: absolute simulation time since run start.
pub type Tick = u64;

/// SPEC: canon §1.2 `Ticks`: every duration, cooldown, timer and delay.
pub type Ticks = u32;

/// SPEC: canon §2.1 `TICK_HZ = 60`. One tick is 1/60 s.
pub const TICK_HZ: u32 = 60;

/// SPEC: canon §2.2 random block ticks: 8 random blocks per active chunk per
/// tick, drawn from `rng.blocktick`.
pub const RANDOM_BLOCKS_PER_CHUNK: u32 = 8;

/// SPEC: canon §2.2 light relight: at most 4,096 cell updates per tick, FIFO.
pub const RELIGHT_BUDGET: u32 = 4096;

/// SPEC: canon §2.2 network snapshots and doc 02 §5.4: the state hash and
/// desync check cadence, every 60 ticks (canon §2.5).
pub const HASH_PERIOD: u64 = 60;

/// SPEC: canon §2.2 (canon-1.11) named boundaries: boundary `b` is the state
/// after tick `b - 1` completes and before tick `b` runs. A row that runs on
/// tick `t` reads boundary `t` and writes boundary `t + 1`, so the snapshot
/// row publishes boundary `t + 1` for every even `t`.
#[must_use]
pub const fn boundary_after(t: Tick) -> Tick {
    match t.checked_add(1) {
        Some(b) => b,
        None => panic!("boundary_after: tick overflow"),
    }
}

/// SPEC: canon §2.5 (canon-1.11): the host hashes every boundary `b` with
/// `b % 60 == 0` and `b >= 60`, and broadcasts that `state_hash`.
#[must_use]
pub const fn is_hash_boundary(b: Tick) -> bool {
    b >= HASH_PERIOD && b.is_multiple_of(HASH_PERIOD)
}

/// SPEC: canon §2.3 "N s" means exactly `N × 60` ticks. Panics if the result
/// does not fit `Ticks`.
#[must_use]
#[track_caller]
pub const fn secs(n: u32) -> Ticks {
    match n.checked_mul(TICK_HZ) {
        Some(t) => t,
        None => panic!("secs: N × 60 overflows Ticks"),
    }
}

/// SPEC: canon §2.2. The subsystems of the tick-schedule table, in table
/// order.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Subsystem {
    /// Input apply, movement, collision, projectiles, melee traces: 60 Hz.
    InputMovement,
    /// Perception sight checks, per observer: 20 Hz, per-entity phase.
    Perception,
    /// AI think, per agent: 10 Hz, per-entity phase.
    AiThink,
    /// Liquids cellular automaton: 10 Hz, phase 1.
    Liquids,
    /// Gases cellular automaton: 10 Hz, phase 4.
    Gases,
    /// Fire spread: 5 Hz, phase 2.
    Fire,
    /// Status pulse (DoT, HoT, buildup decay): 4 Hz, phase 0.
    StatusPulse,
    /// Temperature diffusion: 2 Hz, phase 5.
    Temperature,
    /// Needs (satiation, toxicity decay, composure regeneration): 1 Hz, phase 7.
    Needs,
    /// Random block ticks: every tick, [`RANDOM_BLOCKS_PER_CHUNK`] per active chunk.
    RandomBlockTicks,
    /// Light relight: every tick, at most [`RELIGHT_BUDGET`] updates.
    LightRelight,
    /// Sound propagation: on emit, same tick (event-driven).
    SoundPropagation,
    /// Network snapshot (doc 16): 30 Hz, phase 0.
    NetSnapshot,
}

/// SPEC: canon §2.2 "Phase" column.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Phase {
    /// The same phase for the whole subsystem.
    Fixed(u32),
    /// Each entity uses its canon §1.11 phase key modulo the period.
    PerEntity,
}

/// SPEC: canon §2.2. One row of the tick-schedule table.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ScheduleRow {
    /// The subsystem.
    pub subsystem: Subsystem,
    /// Period in ticks; the subsystem runs when `(t + phase) % period == 0`.
    pub period: u32,
    /// Fixed or per-entity phase.
    pub phase: Phase,
    /// `true` for sound propagation, which runs "on emit, same tick": the
    /// system is visited every tick and propagates whatever was emitted on
    /// that tick (doc 03 §2.2 `sys.sound`, period 1, phase 0).
    pub event_driven: bool,
}

impl ScheduleRow {
    /// SPEC: canon §2.2 "Rate" column: runs per second, `60 / period`.
    #[must_use]
    pub const fn rate_hz(&self) -> u32 {
        TICK_HZ / self.period
    }

    /// SPEC: canon §2.3. The accumulator denominator `k = 60 / period` for a
    /// per-second rate applied by this subsystem.
    #[must_use]
    pub const fn k(&self) -> i64 {
        rate_k(self.period)
    }
}

const fn row(subsystem: Subsystem, period: u32, phase: Phase) -> ScheduleRow {
    ScheduleRow {
        subsystem,
        period,
        phase,
        event_driven: false,
    }
}

/// SPEC: canon §2.2. The tick-schedule table, in canon row order. Doc 03 §2.2
/// owns the order of systems inside one tick; this table owns only periods
/// and phases.
pub const SCHEDULE: [ScheduleRow; 13] = [
    row(Subsystem::InputMovement, 1, Phase::Fixed(0)),
    row(Subsystem::Perception, 3, Phase::PerEntity),
    row(Subsystem::AiThink, 6, Phase::PerEntity),
    row(Subsystem::Liquids, 6, Phase::Fixed(1)),
    row(Subsystem::Gases, 6, Phase::Fixed(4)),
    row(Subsystem::Fire, 12, Phase::Fixed(2)),
    row(Subsystem::StatusPulse, 15, Phase::Fixed(0)),
    row(Subsystem::Temperature, 30, Phase::Fixed(5)),
    row(Subsystem::Needs, 60, Phase::Fixed(7)),
    row(Subsystem::RandomBlockTicks, 1, Phase::Fixed(0)),
    row(Subsystem::LightRelight, 1, Phase::Fixed(0)),
    ScheduleRow {
        subsystem: Subsystem::SoundPropagation,
        period: 1,
        phase: Phase::Fixed(0),
        event_driven: true,
    },
    row(Subsystem::NetSnapshot, 2, Phase::Fixed(0)),
];

impl Subsystem {
    /// SPEC: canon §2.2. Every subsystem, in table order.
    pub const ALL: [Subsystem; 13] = [
        Subsystem::InputMovement,
        Subsystem::Perception,
        Subsystem::AiThink,
        Subsystem::Liquids,
        Subsystem::Gases,
        Subsystem::Fire,
        Subsystem::StatusPulse,
        Subsystem::Temperature,
        Subsystem::Needs,
        Subsystem::RandomBlockTicks,
        Subsystem::LightRelight,
        Subsystem::SoundPropagation,
        Subsystem::NetSnapshot,
    ];

    /// SPEC: canon §2.2. This subsystem's row of [`SCHEDULE`].
    #[must_use]
    pub const fn row(self) -> ScheduleRow {
        SCHEDULE[self as usize]
    }

    /// SPEC: canon §2.2. Period in ticks.
    #[must_use]
    pub const fn period(self) -> u32 {
        self.row().period
    }

    /// SPEC: canon §2.2. `true` when the phase is per-entity.
    #[must_use]
    pub const fn is_per_entity(self) -> bool {
        matches!(self.row().phase, Phase::PerEntity)
    }
}

/// SPEC: canon §2.2 "(t + phase) % period == 0", computed as
/// `(t % period + phase % period) % period` so no tick value can overflow.
/// Panics if `period == 0`.
#[must_use]
#[track_caller]
pub const fn due(t: Tick, period: u32, phase: u64) -> bool {
    assert!(period > 0, "due: period must be > 0");
    let p = period as u64;
    (t % p + phase % p).is_multiple_of(p)
}

/// SPEC: canon §2.2, §1.11. Does `subsystem` run on tick `t`?
///
/// For a fixed-phase subsystem `phase_key` is ignored. For a per-entity
/// subsystem the phase is `phase_key % period`, where `phase_key` is
/// [`EntityId::phase_key`] (`EntityId & 0xFF_FFFF_FFFF`). Doc 02 §5.3
/// example: entity `(3 << 40) | 5` thinks (period 6) on `t = 1, 7, 13, …`.
#[must_use]
pub const fn runs_on(subsystem: Subsystem, t: Tick, phase_key: u64) -> bool {
    let r = subsystem.row();
    let phase = match r.phase {
        Phase::Fixed(p) => p as u64,
        Phase::PerEntity => phase_key,
    };
    due(t, r.period, phase)
}

/// SPEC: canon §1.11, §2.2, doc 02 §5.3 `entity_due`. Does `subsystem` run
/// entity `e` on tick `t`? Same as [`runs_on`] with `e.phase_key()`.
#[must_use]
pub const fn runs_on_entity(subsystem: Subsystem, t: Tick, e: EntityId) -> bool {
    runs_on(subsystem, t, e.phase_key())
}

/// SPEC: canon §2.3 rate accumulator denominator `k = 60 / P` for a
/// subsystem of period `P` ticks. Per-tick systems use 60, the status pulse 4,
/// needs 1. Panics unless `P` divides 60.
#[must_use]
#[track_caller]
pub const fn rate_k(period: u32) -> i64 {
    assert!(
        period > 0 && period <= TICK_HZ && TICK_HZ.is_multiple_of(period),
        "rate_k: period must divide 60"
    );
    (TICK_HZ / period) as i64
}

/// SPEC: canon §2.3. `k` for per-tick systems.
pub const K_PER_TICK: i64 = 60;
/// SPEC: canon §2.3. `k` for the status pulse (period 15).
pub const K_STATUS_PULSE: i64 = 4;
/// SPEC: canon §2.3. `k` for needs (period 60).
pub const K_NEEDS: i64 = 1;

/// SPEC: canon §2.3 rate accumulator, one per (entity, rate). Holds `acc`,
/// which stays in `0..k` after every [`RateAcc::step`].
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RateAcc(i64);

impl RateAcc {
    /// SPEC: canon §2.3. A fresh accumulator, `acc = 0`.
    pub const ZERO: RateAcc = RateAcc(0);

    /// SPEC: canon §2.5 restore: rebuild from a saved `acc`.
    #[inline]
    #[must_use]
    pub const fn from_raw(acc: i64) -> RateAcc {
        RateAcc(acc)
    }

    /// SPEC: canon §2.5. The saved `acc`.
    #[inline]
    #[must_use]
    pub const fn raw(self) -> i64 {
        self.0
    }

    /// SPEC: canon §2.3 accumulator rule, applied once per subsystem run:
    ///
    /// ```text
    /// total = R + acc
    /// q     = floor_div(total, k)
    /// acc   = total - k * q
    /// apply q on this run
    /// ```
    ///
    /// Returns `q`. `R` is the per-second rate (`Milli` or whole units, any
    /// sign); `k > 0`. `R + acc` is checked; `k * q` is computed in `i128`, so
    /// the step itself cannot overflow. Over any `k` consecutive runs the sum
    /// of `q` is exactly `R` when `acc` starts at 0: no rate drifts.
    #[track_caller]
    pub fn step(&mut self, rate: i64, k: i64) -> i64 {
        assert!(k > 0, "RateAcc::step: k must be > 0");
        let total = match rate.checked_add(self.0) {
            Some(v) => v,
            None => panic!("RateAcc::step: R + acc overflows"),
        };
        let q = floor_div(total, k);
        self.0 = narrow_i64(i128::from(total) - i128::from(k) * i128::from(q));
        q
    }

    /// SPEC: canon §2.3 "While a rate is suspended this way, its accumulator
    /// resets to 0."
    pub fn reset(&mut self) {
        self.0 = 0;
    }

    /// SPEC: canon §2.3. One run of a rate that may be suspended: when
    /// `active`, [`RateAcc::step`]; otherwise reset the accumulator and apply 0.
    #[track_caller]
    pub fn step_gated(&mut self, rate: i64, k: i64, active: bool) -> i64 {
        if active {
            self.step(rate, k)
        } else {
            self.reset();
            0
        }
    }

    /// SPEC: canon §2.3. One run of a rate gated by "after `n_secs` s without
    /// X" ([`after_secs_without`]); the accumulator resets while suspended.
    #[track_caller]
    pub fn step_after_without(
        &mut self,
        rate: i64,
        k: i64,
        t: Tick,
        last_x: Option<Tick>,
        n_secs: u32,
    ) -> i64 {
        self.step_gated(rate, k, after_secs_without(t, last_x, n_secs))
    }
}

/// SPEC: canon §2.3 "After N s without X" means `t - last_X_tick >= 60 × N`.
///
/// `last_x = None` (X never happened) counts as satisfied. A `last_x` later
/// than `t` is treated as not yet satisfied rather than underflowing.
#[must_use]
#[track_caller]
pub const fn after_secs_without(t: Tick, last_x: Option<Tick>, n_secs: u32) -> bool {
    match last_x {
        None => true,
        Some(last) => match t.checked_sub(last) {
            Some(elapsed) => elapsed >= secs(n_secs) as u64,
            None => false,
        },
    }
}

/// SPEC: canon §2.3 `in_combat` window: non-DoT damage or `alerted`
/// awareness within ticks `(t - 300) ..= t`.
pub const IN_COMBAT_WINDOW: Ticks = 300;

/// SPEC: canon §2.3. `true` when event tick `x` lies in `(t - window) ..= t`.
/// A window that would start before tick 0 starts at 0; `x > t` is outside.
#[must_use]
pub const fn within_window(x: Option<Tick>, t: Tick, window: Ticks) -> bool {
    match x {
        None => false,
        Some(x) => x <= t && t - x <= window as u64,
    }
}

/// SPEC: canon §2.3 `in_combat(e, t)` inputs: the last ticks the caller
/// recorded for entity `e`.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct CombatTicks {
    /// Last tick `e` dealt non-DoT damage.
    pub dealt: Option<Tick>,
    /// Last tick `e` took non-DoT damage.
    pub taken: Option<Tick>,
    /// Last tick any hostile AI held awareness `alerted` toward `e`.
    pub alerted: Option<Tick>,
}

impl CombatTicks {
    /// SPEC: canon §2.3 `in_combat(e, t)`: `e` dealt or took non-DoT damage
    /// within ticks `(t-300) ..= t`, or any hostile AI held awareness
    /// `alerted` toward `e` within that window.
    #[must_use]
    pub const fn in_combat(&self, t: Tick) -> bool {
        in_combat(t, self.dealt, self.taken, self.alerted)
    }
}

/// SPEC: canon §2.3 `in_combat(e, t)` over provided last-event ticks: the
/// last non-DoT damage dealt, the last non-DoT damage taken, and the last tick
/// a hostile AI held `alerted` awareness toward `e`.
#[must_use]
pub const fn in_combat(
    t: Tick,
    last_dealt: Option<Tick>,
    last_taken: Option<Tick>,
    last_alerted: Option<Tick>,
) -> bool {
    within_window(last_dealt, t, IN_COMBAT_WINDOW)
        || within_window(last_taken, t, IN_COMBAT_WINDOW)
        || within_window(last_alerted, t, IN_COMBAT_WINDOW)
}

/// SPEC: canon §2.3 status timer. A status applied during tick `a` with
/// duration `D` is active on ticks `a+1 ..= a+D` and removed in the
/// status-timer step at the end of tick `a+D`. `expiry = a + D`; `None` means
/// no expiry (doc 03 §3.3 `StatusInst.expiry: Option<Tick>`).
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StatusTimer {
    applied: Tick,
    expiry: Option<Tick>,
}

impl StatusTimer {
    /// SPEC: canon §2.3. Applied during tick `a` with duration `d` ticks;
    /// `expiry = a + d` (checked). `d = 0` gives an empty active interval and
    /// removal at the end of tick `a`.
    #[must_use]
    #[track_caller]
    pub const fn apply(a: Tick, d: Ticks) -> StatusTimer {
        let expiry = match a.checked_add(d as u64) {
            Some(e) => e,
            None => panic!("StatusTimer::apply: a + D overflows"),
        };
        StatusTimer {
            applied: a,
            expiry: Some(expiry),
        }
    }

    /// SPEC: canon §2.3, doc 03 §3.3. Applied during tick `a` with no expiry.
    #[must_use]
    pub const fn indefinite(a: Tick) -> StatusTimer {
        StatusTimer {
            applied: a,
            expiry: None,
        }
    }

    /// SPEC: canon §2.5 restore. Rebuild from saved fields. Panics if the
    /// expiry is before the applied tick.
    #[must_use]
    #[track_caller]
    pub const fn from_parts(applied: Tick, expiry: Option<Tick>) -> StatusTimer {
        if let Some(e) = expiry {
            assert!(e >= applied, "StatusTimer: expiry before applied tick");
        }
        StatusTimer { applied, expiry }
    }

    /// SPEC: canon §2.3. The tick `a` the status was applied during.
    #[must_use]
    pub const fn applied(&self) -> Tick {
        self.applied
    }

    /// SPEC: canon §2.3. The last active tick `a + D`, or `None`.
    #[must_use]
    pub const fn expiry(&self) -> Option<Tick> {
        self.expiry
    }

    /// SPEC: canon §2.3. Active on `t` when `a+1 <= t <= expiry`. "A pulse at
    /// tick `t` affects only statuses active on `t`."
    #[must_use]
    pub const fn is_active(&self, t: Tick) -> bool {
        if t <= self.applied {
            return false;
        }
        match self.expiry {
            None => true,
            Some(e) => t <= e,
        }
    }

    /// SPEC: canon §2.3. `true` when the status-timer step at the end of tick
    /// `t` removes the status (`t >= a + D`).
    #[must_use]
    pub const fn expires_at_end_of(&self, t: Tick) -> bool {
        match self.expiry {
            None => false,
            Some(e) => t >= e,
        }
    }

    /// SPEC: canon §2.3 "A refresh sets the expiry tick to
    /// `max(old_expiry, t + D)`." The applied tick is unchanged, so periodic
    /// sub-effects keep their phase. A status with no expiry keeps none.
    #[track_caller]
    pub fn refresh(&mut self, t: Tick, d: Ticks) {
        if let Some(old) = self.expiry {
            let new = match t.checked_add(u64::from(d)) {
                Some(v) => v,
                None => panic!("StatusTimer::refresh: t + D overflows"),
            };
            self.expiry = Some(if new > old { new } else { old });
        }
    }

    /// SPEC: canon §2.3 "Periodic sub-effects of a status fire on active
    /// ticks where `(t - a) % period == 0`." Panics if `period == 0`.
    #[must_use]
    #[track_caller]
    pub const fn periodic_due(&self, t: Tick, period: Ticks) -> bool {
        assert!(period > 0, "periodic_due: period must be > 0");
        self.is_active(t) && (t - self.applied).is_multiple_of(period as u64)
    }

    /// SPEC: canon §2.3. Active ticks left after tick `t`: `expiry - t` while
    /// active or before start, 0 once expired, `None` without expiry.
    #[must_use]
    pub const fn remaining_after(&self, t: Tick) -> Option<Tick> {
        match self.expiry {
            None => None,
            Some(e) => Some(e.saturating_sub(t)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn sum_runs(rate: i64, k: i64, runs: u64) -> i64 {
        let mut acc = RateAcc::ZERO;
        let mut total = 0i64;
        for _ in 0..runs {
            total += acc.step(rate, k);
        }
        total
    }

    #[test]
    fn canon_t10_rate_accumulator() {
        // 23 per second for 60 ticks at k = 60 adds exactly 23.
        assert_eq!(sum_runs(23, K_PER_TICK, 60), 23);
        // 7 per second for 600 ticks adds exactly 70.
        assert_eq!(sum_runs(7, K_PER_TICK, 600), 70);
        // 850 mHP per second over 4 status pulses at k = 4 adds exactly 850.
        assert_eq!(sum_runs(850, K_STATUS_PULSE, 4), 850);
        assert_eq!(rate_k(1), K_PER_TICK);
        assert_eq!(rate_k(15), K_STATUS_PULSE);
        assert_eq!(rate_k(60), K_NEEDS);
        assert_eq!(Subsystem::StatusPulse.row().k(), 4);
        assert_eq!(Subsystem::Needs.row().k(), 1);
        assert_eq!(Subsystem::InputMovement.row().k(), 60);
    }

    #[test]
    fn accumulator_step_by_step_values() {
        // 23 per s at k = 60: total = 23 + acc; the first q = 1 lands when
        // acc reaches 60, i.e. on run 3 (23, 46, 69 -> q=1, acc=9).
        let mut a = RateAcc::ZERO;
        assert_eq!(a.step(23, 60), 0);
        assert_eq!(a.raw(), 23);
        assert_eq!(a.step(23, 60), 0);
        assert_eq!(a.raw(), 46);
        assert_eq!(a.step(23, 60), 1);
        assert_eq!(a.raw(), 9);
        // Needs, k = 1: everything applies at once.
        let mut n = RateAcc::ZERO;
        assert_eq!(n.step(-500, K_NEEDS), -500);
        assert_eq!(n.raw(), 0);
        // Negative rates floor: -7 at k = 60 gives q = -1, acc = 53.
        let mut d = RateAcc::ZERO;
        assert_eq!(d.step(-7, 60), -1);
        assert_eq!(d.raw(), 53);
    }

    #[test]
    fn accumulator_hp_regen_reference() {
        // Canon §6.4 row 2 at all-40: 425 mHP/s per tick at k = 60.
        assert_eq!(sum_runs(425, K_PER_TICK, 60), 425);
        assert_eq!(sum_runs(425, K_PER_TICK, 600), 4250);
        // Canon §6.4 row 5 at all-40: 23 stamina per s = 23,000 milli.
        assert_eq!(sum_runs(23_000, K_PER_TICK, 60), 23_000);
    }

    #[test]
    fn gated_accumulator_resets_while_suspended() {
        let mut a = RateAcc::ZERO;
        assert_eq!(a.step_gated(23, 60, true), 0);
        assert_eq!(a.raw(), 23);
        assert_eq!(a.step_gated(23, 60, false), 0);
        assert_eq!(a.raw(), 0);
        // After 1 s without spending: spent at tick 100.
        let mut s = RateAcc::ZERO;
        assert_eq!(s.step_after_without(30, 60, 159, Some(100), 1), 0);
        assert_eq!(s.raw(), 0);
        assert_eq!(s.step_after_without(30, 60, 160, Some(100), 1), 0);
        assert_eq!(s.raw(), 30);
        assert_eq!(s.step_after_without(30, 60, 161, Some(100), 1), 1);
        assert_eq!(s.raw(), 0);
    }

    #[test]
    #[should_panic(expected = "overflows")]
    fn accumulator_overflow_panics() {
        let mut a = RateAcc::from_raw(59);
        let _ = a.step(i64::MAX, 60);
    }

    #[test]
    fn accumulator_extreme_values_do_not_overflow_internally() {
        let mut a = RateAcc::ZERO;
        let q = a.step(i64::MIN, 60);
        assert_eq!(q, floor_div(i64::MIN, 60));
        assert!((0..60).contains(&a.raw()));
        let mut b = RateAcc::ZERO;
        let q = b.step(i64::MAX, 7);
        assert_eq!(q, i64::MAX / 7);
        assert_eq!(b.raw(), i64::MAX % 7);
    }

    #[test]
    #[should_panic(expected = "period must divide 60")]
    fn rate_k_rejects_non_divisor() {
        let _ = rate_k(7);
    }

    #[test]
    fn secs_is_times_60() {
        assert_eq!(secs(0), 0);
        assert_eq!(secs(1), 60);
        assert_eq!(secs(5), 300);
        assert_eq!(secs(10), 600);
    }

    #[test]
    fn schedule_table_matches_canon() {
        let expect: [(Subsystem, u32, Phase, u32); 13] = [
            (Subsystem::InputMovement, 1, Phase::Fixed(0), 60),
            (Subsystem::Perception, 3, Phase::PerEntity, 20),
            (Subsystem::AiThink, 6, Phase::PerEntity, 10),
            (Subsystem::Liquids, 6, Phase::Fixed(1), 10),
            (Subsystem::Gases, 6, Phase::Fixed(4), 10),
            (Subsystem::Fire, 12, Phase::Fixed(2), 5),
            (Subsystem::StatusPulse, 15, Phase::Fixed(0), 4),
            (Subsystem::Temperature, 30, Phase::Fixed(5), 2),
            (Subsystem::Needs, 60, Phase::Fixed(7), 1),
            (Subsystem::RandomBlockTicks, 1, Phase::Fixed(0), 60),
            (Subsystem::LightRelight, 1, Phase::Fixed(0), 60),
            (Subsystem::SoundPropagation, 1, Phase::Fixed(0), 60),
            (Subsystem::NetSnapshot, 2, Phase::Fixed(0), 30),
        ];
        for (i, (s, period, phase, hz)) in expect.iter().enumerate() {
            let r = SCHEDULE[i];
            assert_eq!(r.subsystem, *s);
            assert_eq!(Subsystem::ALL[i], *s);
            assert_eq!(s.row(), r);
            assert_eq!(r.period, *period);
            assert_eq!(r.phase, *phase);
            assert_eq!(r.rate_hz(), *hz);
            assert_eq!(r.event_driven, *s == Subsystem::SoundPropagation);
        }
        assert!(Subsystem::AiThink.is_per_entity());
        assert!(!Subsystem::Fire.is_per_entity());
    }

    fn due_mod_60(s: Subsystem) -> Vec<u64> {
        (0..60).filter(|&t| runs_on(s, t, 0)).collect()
    }

    #[test]
    fn stagger_matches_doc03_table() {
        // Doc 03 §2.3: due ticks mod 60 of the slow systems.
        assert_eq!(
            due_mod_60(Subsystem::Liquids),
            vec![5, 11, 17, 23, 29, 35, 41, 47, 53, 59]
        );
        assert_eq!(
            due_mod_60(Subsystem::Gases),
            vec![2, 8, 14, 20, 26, 32, 38, 44, 50, 56]
        );
        assert_eq!(due_mod_60(Subsystem::Fire), vec![10, 22, 34, 46, 58]);
        assert_eq!(due_mod_60(Subsystem::StatusPulse), vec![0, 15, 30, 45]);
        assert_eq!(due_mod_60(Subsystem::Temperature), vec![25, 55]);
        assert_eq!(due_mod_60(Subsystem::Needs), vec![53]);
        assert_eq!(due_mod_60(Subsystem::NetSnapshot).len(), 30);
        assert_eq!(due_mod_60(Subsystem::InputMovement).len(), 60);
        // No two of liquids, gases, fire, temperature and the pulse share a tick.
        let slow = [
            Subsystem::Liquids,
            Subsystem::Gases,
            Subsystem::Fire,
            Subsystem::Temperature,
            Subsystem::StatusPulse,
        ];
        for t in 0..60 {
            let n = slow.iter().filter(|&&s| runs_on(s, t, 0)).count();
            assert!(n <= 1, "tick {t}");
        }
    }

    #[test]
    fn per_entity_phase_doc02_example() {
        // Doc 02 §5.3: (3 << 40) | 5 thinks on t = 1, 7, 13, ...
        let e = EntityId::from_raw((3 << 40) | 5);
        let ticks: Vec<u64> = (0..20)
            .filter(|&t| runs_on_entity(Subsystem::AiThink, t, e))
            .collect();
        assert_eq!(ticks, vec![1, 7, 13, 19]);
        // Perception, period 3, phase 5 % 3 = 2: t = 1, 4, 7, ...
        let p: Vec<u64> = (0..10)
            .filter(|&t| runs_on_entity(Subsystem::Perception, t, e))
            .collect();
        assert_eq!(p, vec![1, 4, 7]);
        // Fixed-phase subsystems ignore the key.
        assert_eq!(
            runs_on(Subsystem::Needs, 53, 0),
            runs_on(Subsystem::Needs, 53, 12345)
        );
    }

    #[test]
    fn due_never_overflows_at_tick_max() {
        // (t + phase) would overflow u64; the modular form is still exact.
        let t = u64::MAX; // u64::MAX % 6 == 3
        assert!(due(t, 6, 3));
        assert!(!due(t, 6, 1));
        assert!(runs_on(Subsystem::InputMovement, t, 0));
    }

    #[test]
    fn status_timer_active_interval() {
        // Applied during tick 100 with D = 3: active on 101, 102, 103 and
        // removed at the end of 103.
        let s = StatusTimer::apply(100, 3);
        assert_eq!(s.applied(), 100);
        assert_eq!(s.expiry(), Some(103));
        let active: Vec<u64> = (95..110).filter(|&t| s.is_active(t)).collect();
        assert_eq!(active, vec![101, 102, 103]);
        assert!(!s.expires_at_end_of(102));
        assert!(s.expires_at_end_of(103));
        assert_eq!(s.remaining_after(101), Some(2));
        assert_eq!(s.remaining_after(200), Some(0));
        // D = 0: never active, removed at the end of tick a.
        let z = StatusTimer::apply(50, 0);
        assert!((0..100).all(|t| !z.is_active(t)));
        assert!(z.expires_at_end_of(50));
    }

    #[test]
    fn status_timer_refresh() {
        let mut s = StatusTimer::apply(100, 300);
        assert_eq!(s.expiry(), Some(400));
        // A shorter refresh never shortens.
        s.refresh(150, 60);
        assert_eq!(s.expiry(), Some(400));
        // A longer one extends to t + D.
        s.refresh(200, 300);
        assert_eq!(s.expiry(), Some(500));
        assert_eq!(s.applied(), 100);
        // Indefinite stays indefinite.
        let mut i = StatusTimer::indefinite(7);
        i.refresh(10, 5);
        assert_eq!(i.expiry(), None);
        assert!(i.is_active(1_000_000));
        assert!(!i.is_active(7));
        assert!(!i.expires_at_end_of(u64::MAX));
    }

    #[test]
    fn status_periodic_sub_effects() {
        // Applied at 100, period 20: fires on 120, 140, ... while active.
        let s = StatusTimer::apply(100, 60);
        let fires: Vec<u64> = (100..=200).filter(|&t| s.periodic_due(t, 20)).collect();
        assert_eq!(fires, vec![120, 140, 160]);
    }

    #[test]
    fn status_pulse_affects_only_active() {
        // A status applied on pulse tick 30 is not hit by that pulse; with
        // D = 30 it is hit by pulses 45 and 60, not 75.
        let s = StatusTimer::apply(30, 30);
        let pulses: Vec<u64> = (0..120)
            .filter(|&t| runs_on(Subsystem::StatusPulse, t, 0) && s.is_active(t))
            .collect();
        assert_eq!(pulses, vec![45, 60]);
    }

    #[test]
    fn after_secs_without_rule() {
        assert!(after_secs_without(10, None, 5));
        assert!(!after_secs_without(399, Some(100), 5));
        assert!(after_secs_without(400, Some(100), 5));
        assert!(!after_secs_without(50, Some(100), 0));
        assert!(after_secs_without(100, Some(100), 0));
    }

    #[test]
    fn in_combat_window_edges() {
        let c = CombatTicks {
            dealt: Some(1000),
            taken: None,
            alerted: None,
        };
        assert!(c.in_combat(1000));
        assert!(c.in_combat(1300));
        assert!(!c.in_combat(1301));
        assert!(!c.in_combat(999));
        let early = CombatTicks {
            dealt: None,
            taken: Some(0),
            alerted: None,
        };
        assert!(early.in_combat(0));
        assert!(early.in_combat(300));
        assert!(!early.in_combat(301));
        let alert = CombatTicks {
            dealt: None,
            taken: None,
            alerted: Some(77),
        };
        assert!(alert.in_combat(200));
        assert!(!CombatTicks::default().in_combat(5));
    }

    /// Review (canon §2.2, §2.5 (canon-1.11)): boundary arithmetic at the
    /// ends of `u64`, and exactly one hashed boundary per 60.
    #[test]
    fn hash_boundary_extremes() {
        let top = u64::MAX - u64::MAX % HASH_PERIOD;
        assert!(is_hash_boundary(top));
        assert!(!is_hash_boundary(u64::MAX));
        assert_eq!(boundary_after(u64::MAX - 1), u64::MAX);
        assert_eq!((0..=600u64).filter(|&b| is_hash_boundary(b)).count(), 10);
        assert_eq!(
            (0..=600u64).filter(|&b| is_hash_boundary(b)).min(),
            Some(60)
        );
    }

    const DIVISORS: [u32; 12] = [1, 2, 3, 4, 5, 6, 10, 12, 15, 20, 30, 60];

    proptest! {
        #[test]
        fn accumulator_no_drift(
            rate in -1_000_000_000i64..1_000_000_000,
            pi in 0usize..12,
            seconds in 0u64..50,
        ) {
            let k = rate_k(DIVISORS[pi]);
            let runs = seconds * k as u64;
            prop_assert_eq!(sum_runs(rate, k, runs), rate * seconds as i64);
        }

        #[test]
        fn accumulator_conservation_and_range(
            rates in proptest::collection::vec(-100_000i64..100_000, 0..200),
            pi in 0usize..12,
            acc0 in 0i64..60,
        ) {
            let k = rate_k(DIVISORS[pi]);
            let acc0 = acc0 % k;
            let mut a = RateAcc::from_raw(acc0);
            let mut applied = 0i64;
            for &r in &rates {
                applied += a.step(r, k);
                prop_assert!((0..k).contains(&a.raw()));
            }
            // k × Σq + acc_n == Σ R + acc_0: nothing is lost or invented.
            let sum: i64 = rates.iter().sum();
            prop_assert_eq!(k * applied + a.raw(), sum + acc0);
        }

        #[test]
        fn fixed_subsystem_runs_period_times_per_minute(
            si in 0usize..13,
            start in 0u64..1_000_000_000,
        ) {
            let s = Subsystem::ALL[si];
            let n = (start..start + 60).filter(|&t| runs_on(s, t, 0)).count() as u32;
            prop_assert_eq!(n, 60 / s.period());
        }

        #[test]
        fn per_entity_runs_once_per_period(
            raw in any::<u64>(),
            start in 0u64..1_000_000_000,
        ) {
            let e = EntityId::from_raw(raw);
            for s in [Subsystem::Perception, Subsystem::AiThink] {
                let p = u64::from(s.period());
                let hits: Vec<u64> = (start..start + p)
                    .filter(|&t| runs_on_entity(s, t, e))
                    .collect();
                prop_assert_eq!(hits.len(), 1);
                prop_assert_eq!((hits[0] + e.phase_key() % p) % p, 0);
            }
        }

        #[test]
        fn status_active_for_exactly_d_ticks(a in 0u64..1_000_000, d in 0u32..2000) {
            let s = StatusTimer::apply(a, d);
            let n = (a..=a + u64::from(d) + 2).filter(|&t| s.is_active(t)).count() as u32;
            prop_assert_eq!(n, d);
            prop_assert!(s.expires_at_end_of(a + u64::from(d)));
            if d > 0 {
                prop_assert!(!s.expires_at_end_of(a + u64::from(d) - 1));
            }
        }

        #[test]
        fn refresh_is_monotone(
            a in 0u64..1_000_000,
            d in 0u32..2000,
            refreshes in proptest::collection::vec((0u64..5000, 0u32..2000), 0..20),
        ) {
            let mut s = StatusTimer::apply(a, d);
            let mut t = a;
            for (dt, d2) in refreshes {
                t += dt;
                let before = s.expiry().unwrap();
                s.refresh(t, d2);
                let after = s.expiry().unwrap();
                prop_assert!(after >= before);
                prop_assert_eq!(after, before.max(t + u64::from(d2)));
                prop_assert_eq!(s.applied(), a);
            }
        }

        #[test]
        fn after_without_is_monotone_in_t(last in 0u64..1_000_000, n in 0u32..100, t in 0u64..2_000_000) {
            if after_secs_without(t, Some(last), n) {
                prop_assert!(after_secs_without(t + 1, Some(last), n));
            }
            prop_assert_eq!(
                after_secs_without(t, Some(last), n),
                t >= last && t - last >= u64::from(n) * 60
            );
        }

        #[test]
        fn in_combat_matches_window_definition(x in 0u64..10_000, t in 0u64..10_000) {
            let c = CombatTicks { dealt: None, taken: Some(x), alerted: None };
            let lo = t.saturating_sub(300);
            prop_assert_eq!(c.in_combat(t), (lo..=t).contains(&x));
        }
    }
}
