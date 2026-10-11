//! SPEC: canon §11 statuses, buildup, DoT, direct duration, erasure; §2.3
//! status active intervals and the rate accumulator; §17.3 enemy ranks.
//!
//! Everything here is a pure function over explicit inputs or `const` data.
//! The status catalog (canon §11.1) is `const` data in ascending status ID
//! byte order, so the derived `Ord` of [`StatusId`] is the canon "ascending
//! status ID order" (§11.7) and a `BTreeMap<StatusId, _>` iterates in it.
//!
//! Interpretations of canon gaps are marked `GAP:` in the doc comments and
//! listed in the module tests' comments; the canon wins wherever it speaks.

use crate::damage::DamageType;
use palimpsest_math::int::{floor_div, mul_bp, mul_bp_i32, mul_bp_wide, narrow_i32};
use palimpsest_math::tables::power_s;
use palimpsest_math::{Fx, Fx3, dist2};

// ---------------------------------------------------------------------------
// Time (canon §2.1, §2.3)
// ---------------------------------------------------------------------------

/// SPEC: canon §2.1 `TICK_HZ = 60`.
pub const TICK_HZ: u32 = 60;

/// SPEC: canon §2.3 "N s anywhere in the spec means exactly N × 60 ticks".
pub const fn secs(n: u32) -> u32 {
    n * TICK_HZ
}

/// SPEC: canon §2.2 status pulse period (DoT, HoT, buildup decay): 15 ticks.
pub const STATUS_PULSE_PERIOD: u64 = 15;
/// SPEC: canon §2.2 status pulse phase: 0.
pub const STATUS_PULSE_PHASE: u64 = 0;
/// SPEC: canon §2.3 accumulator denominator of the status pulse: `k = 60 / 15 = 4`.
pub const PULSE_K: i64 = 4;

/// SPEC: canon §2.2. True when the status pulse runs on tick `t`:
/// `(t + phase) % period == 0`.
pub const fn is_status_pulse(t: u64) -> bool {
    (t + STATUS_PULSE_PHASE).is_multiple_of(STATUS_PULSE_PERIOD)
}

/// SPEC: canon §2.3 rate accumulator. A per-second rate `rate` applied by a
/// subsystem with denominator `k = 60 / period`:
/// `total = rate + acc; q = floor_div(total, k); acc = total - k*q`; returns `q`.
/// Panics if `k <= 0`.
pub fn accumulate(acc: &mut i64, rate: i64, k: i64) -> i64 {
    let total = rate + *acc;
    let q = floor_div(total, k);
    *acc = total - k * q;
    q
}

// ---------------------------------------------------------------------------
// Status identifiers and catalog (canon §11.1)
// ---------------------------------------------------------------------------

/// SPEC: canon §11.1. Every status ID. Variants are declared in ascending
/// content-ID byte order, so the derived `Ord` is the canon §11.7 order.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(missing_docs)]
pub enum StatusId {
    Berserk,
    Bleeding,
    Blessed,
    Blinded,
    Bound,
    Burning,
    Charmed,
    Chilled,
    Confused,
    Contradicted,
    Corroded,
    Deafened,
    DeathsDoor,
    Disarmed,
    Diseased,
    Downed,
    Dread,
    Empowered,
    Encumbered,
    Erasure,
    Exhausted,
    Exposed,
    Feared,
    Focused,
    Fortified,
    Frozen,
    Ghost,
    Grappled,
    Hacked,
    Hasted,
    Hidden,
    Hungry,
    Incubating,
    Inspired,
    Invisible,
    Irradiated,
    KnockedDown,
    Levitating,
    Madness,
    Marked,
    Mounted,
    Oiled,
    Overburdened,
    Poisoned,
    Polymorphed,
    Regenerating,
    Rooted,
    Shielded,
    Shocked,
    Silenced,
    Sleeping,
    Slowed,
    Soaked,
    Staggered,
    Starving,
    Stunned,
    Suffocating,
    Taunted,
    Unmoored,
    Unraveled,
    Weakened,
    Withering,
}

/// SPEC: canon §11.1. Number of statuses in the canon table.
pub const STATUS_COUNT: usize = 62;

/// SPEC: canon §11.1 "Category" column.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(missing_docs)]
pub enum Category {
    Dot,
    Debuff,
    Control,
    Mental,
    Buff,
    Need,
    Indicator,
    Special,
}

/// SPEC: canon §11.1 "Threshold group" column. `Body`, `Nerve`, `Mind` and
/// `Arcane` build through meters (§6.4 rows 22 to 25); `Poise` is driven by
/// `res.poise`; `Direct` is applied only directly; `Special` is state-driven.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(missing_docs)]
pub enum ThresholdGroup {
    Body,
    Nerve,
    Mind,
    Arcane,
    Poise,
    Direct,
    Special,
}

/// SPEC: canon §11.1 "Scaling" column and §6.4 rows 22 to 25: the threshold
/// group used for direct-application duration (§11.4) and buildup meters.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(missing_docs)]
pub enum ScalingGroup {
    Body,
    Nerve,
    Mind,
    Arcane,
}

/// SPEC: canon §11.1 state-driven "Stacking" conditions: the status exists
/// while (or until) the condition holds; the owning doc adds and removes it.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Condition {
    /// `st.madness`: until composure ≥ 25% of max.
    UntilComposureQuarter,
    /// `st.hungry`: while satiation ≤ 600.
    WhileSatiationAtMost600,
    /// `st.starving`: while satiation = 0.
    WhileSatiationZero,
    /// `st.encumbered`: while `10000 < load_bp <= 12500`.
    WhileEncumberedLoad,
    /// `st.overburdened`: while `load_bp > 12500`.
    WhileOverburdenedLoad,
    /// `st.exhausted`: 2 s after stamina hits 0.
    AfterStaminaZero,
    /// `st.contradicted`: while any equipped component has C ≥ 3.
    WhileComponentContradicted,
    /// `st.hidden`: while no hostile holds awareness `aware.alert` or higher.
    WhileUnnoticed,
    /// `st.downed`: until revived or bled out.
    UntilRevivedOrBledOut,
    /// `st.suffocating`: while `res.breath` = 0.
    WhileBreathZero,
    /// `st.mounted`: while riding.
    WhileRiding,
    /// `st.bound`: while a minion is bound to its owner.
    WhileMinionBound,
    /// `st.ghost`: until resurrected.
    UntilResurrected,
}

/// SPEC: canon §11.1 stacking rules.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Stacking {
    /// Reapplying sets the expiry tick to `max(old_expiry, t + D)`.
    Refresh,
    /// Each application adds one stack up to N; all stacks share one expiry,
    /// which refreshes on each application.
    Intensity(u8),
    /// The status holds a quantity (an overshield amount) instead of stacks.
    Pool,
    /// State-driven (§11.1 "while ..." and "until ..." rows).
    Conditional(Condition),
}

/// SPEC: canon §11.1 "Base duration" column.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BaseDuration {
    /// A fixed duration in ticks (§2.3: "N s" is N × 60 ticks).
    Ticks(u32),
    /// "none": the status lasts while its condition holds.
    None,
    /// `st.erasure`: until Folio exit.
    UntilFolioExit,
}

/// SPEC: canon §11.1. One row of the status table.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct StatusDef {
    /// The status.
    pub id: StatusId,
    /// Its content ID, for example `st.bleeding`.
    pub key: &'static str,
    /// Category column.
    pub category: Category,
    /// Threshold group column.
    pub threshold: ThresholdGroup,
    /// Scaling column (`None` is the canon `—`, unscaled).
    pub scaling: Option<ScalingGroup>,
    /// Stacking column.
    pub stacking: Stacking,
    /// Base duration column.
    pub base: BaseDuration,
    /// Owner column (spec document number).
    pub owner_doc: u8,
}

// One argument per canon §11.1 column keeps the table readable.
#[allow(clippy::too_many_arguments)]
const fn row(
    id: StatusId,
    key: &'static str,
    category: Category,
    threshold: ThresholdGroup,
    scaling: Option<ScalingGroup>,
    stacking: Stacking,
    base: BaseDuration,
    owner_doc: u8,
) -> StatusDef {
    StatusDef {
        id,
        key,
        category,
        threshold,
        scaling,
        stacking,
        base,
        owner_doc,
    }
}

/// SPEC: canon §11.1. The full status catalog, indexed by `StatusId as usize`
/// (ascending content-ID byte order). Durations are in ticks.
pub static CATALOG: [StatusDef; STATUS_COUNT] = {
    use BaseDuration::Ticks as T;
    use Category as C;
    use Condition as Cond;
    use ScalingGroup as S;
    use Stacking::{Conditional, Intensity, Pool, Refresh};
    use StatusId as Id;
    use ThresholdGroup as G;
    [
        row(
            Id::Berserk,
            "st.berserk",
            C::Control,
            G::Mind,
            Some(S::Mind),
            Refresh,
            T(360),
            6,
        ),
        row(
            Id::Bleeding,
            "st.bleeding",
            C::Dot,
            G::Nerve,
            Some(S::Nerve),
            Intensity(5),
            T(360),
            5,
        ),
        row(
            Id::Blessed,
            "st.blessed",
            C::Buff,
            G::Direct,
            None,
            Refresh,
            T(1800),
            6,
        ),
        row(
            Id::Blinded,
            "st.blinded",
            C::Debuff,
            G::Nerve,
            Some(S::Nerve),
            Refresh,
            T(240),
            5,
        ),
        row(
            Id::Bound,
            "st.bound",
            C::Indicator,
            G::Special,
            None,
            Conditional(Cond::WhileMinionBound),
            BaseDuration::None,
            13,
        ),
        row(
            Id::Burning,
            "st.burning",
            C::Dot,
            G::Nerve,
            Some(S::Nerve),
            Refresh,
            T(240),
            5,
        ),
        row(
            Id::Charmed,
            "st.charmed",
            C::Control,
            G::Mind,
            Some(S::Mind),
            Refresh,
            T(360),
            6,
        ),
        row(
            Id::Chilled,
            "st.chilled",
            C::Debuff,
            G::Direct,
            Some(S::Nerve),
            Refresh,
            T(180),
            5,
        ),
        row(
            Id::Confused,
            "st.confused",
            C::Control,
            G::Mind,
            Some(S::Mind),
            Refresh,
            T(300),
            6,
        ),
        row(
            Id::Contradicted,
            "st.contradicted",
            C::Indicator,
            G::Special,
            None,
            Conditional(Cond::WhileComponentContradicted),
            BaseDuration::None,
            12,
        ),
        row(
            Id::Corroded,
            "st.corroded",
            C::Dot,
            G::Body,
            Some(S::Body),
            Intensity(5),
            T(600),
            5,
        ),
        row(
            Id::Deafened,
            "st.deafened",
            C::Debuff,
            G::Nerve,
            Some(S::Nerve),
            Refresh,
            T(360),
            9,
        ),
        row(
            Id::DeathsDoor,
            "st.deaths_door",
            C::Special,
            G::Special,
            None,
            Refresh,
            T(60),
            4,
        ),
        row(
            Id::Disarmed,
            "st.disarmed",
            C::Control,
            G::Direct,
            Some(S::Nerve),
            Refresh,
            T(180),
            5,
        ),
        row(
            Id::Diseased,
            "st.diseased",
            C::Debuff,
            G::Body,
            Some(S::Body),
            Refresh,
            T(18_000),
            4,
        ),
        row(
            Id::Downed,
            "st.downed",
            C::Special,
            G::Special,
            None,
            Conditional(Cond::UntilRevivedOrBledOut),
            T(1800),
            16,
        ),
        row(
            Id::Dread,
            "st.dread",
            C::Mental,
            G::Mind,
            Some(S::Mind),
            Refresh,
            T(360),
            6,
        ),
        row(
            Id::Empowered,
            "st.empowered",
            C::Buff,
            G::Direct,
            None,
            Refresh,
            T(480),
            6,
        ),
        row(
            Id::Encumbered,
            "st.encumbered",
            C::Need,
            G::Special,
            None,
            Conditional(Cond::WhileEncumberedLoad),
            BaseDuration::None,
            4,
        ),
        row(
            Id::Erasure,
            "st.erasure",
            C::Special,
            G::Direct,
            None,
            Intensity(10),
            BaseDuration::UntilFolioExit,
            13,
        ),
        row(
            Id::Exhausted,
            "st.exhausted",
            C::Need,
            G::Special,
            None,
            Conditional(Cond::AfterStaminaZero),
            T(120),
            4,
        ),
        row(
            Id::Exposed,
            "st.exposed",
            C::Debuff,
            G::Body,
            Some(S::Body),
            Intensity(5),
            T(360),
            5,
        ),
        row(
            Id::Feared,
            "st.feared",
            C::Control,
            G::Mind,
            Some(S::Mind),
            Refresh,
            T(240),
            6,
        ),
        row(
            Id::Focused,
            "st.focused",
            C::Buff,
            G::Direct,
            None,
            Refresh,
            T(600),
            8,
        ),
        row(
            Id::Fortified,
            "st.fortified",
            C::Buff,
            G::Direct,
            None,
            Refresh,
            T(600),
            6,
        ),
        row(
            Id::Frozen,
            "st.frozen",
            C::Control,
            G::Nerve,
            Some(S::Nerve),
            Refresh,
            T(120),
            5,
        ),
        row(
            Id::Ghost,
            "st.ghost",
            C::Special,
            G::Special,
            None,
            Conditional(Cond::UntilResurrected),
            BaseDuration::None,
            16,
        ),
        row(
            Id::Grappled,
            "st.grappled",
            C::Control,
            G::Direct,
            Some(S::Nerve),
            Refresh,
            T(180),
            5,
        ),
        row(
            Id::Hacked,
            "st.hacked",
            C::Control,
            G::Mind,
            Some(S::Mind),
            Refresh,
            T(360),
            9,
        ),
        row(
            Id::Hasted,
            "st.hasted",
            C::Buff,
            G::Direct,
            None,
            Refresh,
            T(480),
            6,
        ),
        row(
            Id::Hidden,
            "st.hidden",
            C::Indicator,
            G::Special,
            None,
            Conditional(Cond::WhileUnnoticed),
            BaseDuration::None,
            9,
        ),
        row(
            Id::Hungry,
            "st.hungry",
            C::Need,
            G::Special,
            None,
            Conditional(Cond::WhileSatiationAtMost600),
            BaseDuration::None,
            4,
        ),
        row(
            Id::Incubating,
            "st.incubating",
            C::Special,
            G::Direct,
            None,
            Refresh,
            T(7200),
            4,
        ),
        row(
            Id::Inspired,
            "st.inspired",
            C::Buff,
            G::Direct,
            None,
            Refresh,
            T(600),
            4,
        ),
        row(
            Id::Invisible,
            "st.invisible",
            C::Buff,
            G::Direct,
            None,
            Refresh,
            T(600),
            9,
        ),
        row(
            Id::Irradiated,
            "st.irradiated",
            C::Dot,
            G::Body,
            Some(S::Body),
            Intensity(10),
            T(1800),
            8,
        ),
        row(
            Id::KnockedDown,
            "st.knocked_down",
            C::Control,
            G::Direct,
            Some(S::Nerve),
            Refresh,
            T(60),
            5,
        ),
        row(
            Id::Levitating,
            "st.levitating",
            C::Buff,
            G::Direct,
            None,
            Refresh,
            T(600),
            6,
        ),
        row(
            Id::Madness,
            "st.madness",
            C::Mental,
            G::Special,
            None,
            Conditional(Cond::UntilComposureQuarter),
            BaseDuration::None,
            4,
        ),
        row(
            Id::Marked,
            "st.marked",
            C::Debuff,
            G::Direct,
            None,
            Refresh,
            T(480),
            9,
        ),
        row(
            Id::Mounted,
            "st.mounted",
            C::Indicator,
            G::Special,
            None,
            Conditional(Cond::WhileRiding),
            BaseDuration::None,
            5,
        ),
        row(
            Id::Oiled,
            "st.oiled",
            C::Debuff,
            G::Direct,
            None,
            Refresh,
            T(1200),
            8,
        ),
        row(
            Id::Overburdened,
            "st.overburdened",
            C::Need,
            G::Special,
            None,
            Conditional(Cond::WhileOverburdenedLoad),
            BaseDuration::None,
            4,
        ),
        row(
            Id::Poisoned,
            "st.poisoned",
            C::Dot,
            G::Body,
            Some(S::Body),
            Intensity(5),
            T(480),
            8,
        ),
        row(
            Id::Polymorphed,
            "st.polymorphed",
            C::Special,
            G::Direct,
            Some(S::Arcane),
            Refresh,
            T(1800),
            6,
        ),
        row(
            Id::Regenerating,
            "st.regenerating",
            C::Buff,
            G::Direct,
            None,
            Refresh,
            T(600),
            6,
        ),
        row(
            Id::Rooted,
            "st.rooted",
            C::Control,
            G::Direct,
            Some(S::Nerve),
            Refresh,
            T(120),
            6,
        ),
        row(
            Id::Shielded,
            "st.shielded",
            C::Buff,
            G::Direct,
            None,
            Pool,
            T(600),
            6,
        ),
        row(
            Id::Shocked,
            "st.shocked",
            C::Control,
            G::Nerve,
            Some(S::Nerve),
            Refresh,
            T(180),
            5,
        ),
        row(
            Id::Silenced,
            "st.silenced",
            C::Control,
            G::Arcane,
            Some(S::Arcane),
            Refresh,
            T(240),
            6,
        ),
        row(
            Id::Sleeping,
            "st.sleeping",
            C::Control,
            G::Mind,
            Some(S::Mind),
            Refresh,
            T(480),
            6,
        ),
        row(
            Id::Slowed,
            "st.slowed",
            C::Debuff,
            G::Direct,
            Some(S::Nerve),
            Refresh,
            T(240),
            5,
        ),
        row(
            Id::Soaked,
            "st.soaked",
            C::Debuff,
            G::Direct,
            None,
            Refresh,
            T(600),
            8,
        ),
        row(
            Id::Staggered,
            "st.staggered",
            C::Control,
            G::Poise,
            None,
            Refresh,
            T(30),
            5,
        ),
        row(
            Id::Starving,
            "st.starving",
            C::Need,
            G::Special,
            None,
            Conditional(Cond::WhileSatiationZero),
            BaseDuration::None,
            4,
        ),
        row(
            Id::Stunned,
            "st.stunned",
            C::Control,
            G::Nerve,
            Some(S::Nerve),
            Refresh,
            T(90),
            5,
        ),
        row(
            Id::Suffocating,
            "st.suffocating",
            C::Dot,
            G::Special,
            None,
            Conditional(Cond::WhileBreathZero),
            BaseDuration::None,
            4,
        ),
        row(
            Id::Taunted,
            "st.taunted",
            C::Control,
            G::Mind,
            Some(S::Mind),
            Refresh,
            T(240),
            5,
        ),
        row(
            Id::Unmoored,
            "st.unmoored",
            C::Special,
            G::Special,
            None,
            Refresh,
            T(300),
            12,
        ),
        row(
            Id::Unraveled,
            "st.unraveled",
            C::Debuff,
            G::Arcane,
            Some(S::Arcane),
            Refresh,
            T(300),
            6,
        ),
        row(
            Id::Weakened,
            "st.weakened",
            C::Debuff,
            G::Direct,
            None,
            Refresh,
            T(360),
            5,
        ),
        row(
            Id::Withering,
            "st.withering",
            C::Dot,
            G::Body,
            Some(S::Body),
            Refresh,
            T(480),
            6,
        ),
    ]
};

impl StatusId {
    /// SPEC: canon §11.1 and §11.7. Every status in ascending ID byte order.
    pub const ALL: [StatusId; STATUS_COUNT] = {
        let mut out = [StatusId::Berserk; STATUS_COUNT];
        let mut i = 0;
        while i < STATUS_COUNT {
            out[i] = CATALOG[i].id;
            i += 1;
        }
        out
    };

    /// SPEC: canon §11.1. This status's catalog row.
    pub const fn def(self) -> &'static StatusDef {
        &CATALOG[self as usize]
    }

    /// SPEC: canon §1.1, §11.1. The content ID, for example `st.bleeding`.
    pub const fn key(self) -> &'static str {
        CATALOG[self as usize].key
    }

    /// SPEC: canon §1.1, §11.1. Parse a status content ID.
    pub fn from_key(key: &str) -> Option<StatusId> {
        CATALOG.iter().find(|d| d.key == key).map(|d| d.id)
    }

    /// SPEC: canon §11.1. Base duration in ticks, when the row gives one.
    pub const fn base_ticks(self) -> Option<u32> {
        match self.def().base {
            BaseDuration::Ticks(n) => Some(n),
            BaseDuration::None | BaseDuration::UntilFolioExit => None,
        }
    }

    /// SPEC: canon §11.1, §11.2. The buildup meter group, for statuses whose
    /// threshold group is body, nerve, mind or arcane (§6.4 rows 22 to 25).
    pub const fn buildup_group(self) -> Option<ScalingGroup> {
        match self.def().threshold {
            ThresholdGroup::Body => Some(ScalingGroup::Body),
            ThresholdGroup::Nerve => Some(ScalingGroup::Nerve),
            ThresholdGroup::Mind => Some(ScalingGroup::Mind),
            ThresholdGroup::Arcane => Some(ScalingGroup::Arcane),
            ThresholdGroup::Poise | ThresholdGroup::Direct | ThresholdGroup::Special => None,
        }
    }

    /// SPEC: canon §11.1. True for category `control`.
    pub const fn is_control(self) -> bool {
        matches!(self.def().category, Category::Control)
    }
}

// ---------------------------------------------------------------------------
// Canonical effect constants used by other formulas (canon §11.1)
// ---------------------------------------------------------------------------

/// SPEC: canon §11.1 `st.exposed`: damage taken × `(10000 + 800 × stacks)`.
pub const fn exposed_damage_taken_bp(stacks: u8) -> i32 {
    10_000 + 800 * stacks as i32
}
/// SPEC: canon §11.1 `st.knocked_down`: damage taken × 11000.
pub const KNOCKED_DOWN_DAMAGE_TAKEN_BP: i32 = 11_000;
/// SPEC: canon §11.1 `st.berserk`: damage dealt × 13000.
pub const BERSERK_DAMAGE_BP: i32 = 13_000;
/// SPEC: canon §11.1 `st.empowered`: damage dealt × 12000.
pub const EMPOWERED_DAMAGE_BP: i32 = 12_000;
/// SPEC: canon §11.1 `st.taunted`: damage dealt to others than the taunter × 7500.
pub const TAUNTED_DAMAGE_BP: i32 = 7_500;
/// SPEC: canon §11.1 `st.weakened`: damage dealt × 7500.
pub const WEAKENED_DAMAGE_BP: i32 = 7_500;
/// SPEC: canon §11.1 `st.frozen` and §10.5 step 3a: the next blunt hit × 15000.
pub const FROZEN_BLUNT_BP: i32 = 15_000;
/// SPEC: canon §11.1 `st.fortified`: AR × 12000 and poise × 12000.
pub const FORTIFIED_BP: i32 = 12_000;
/// SPEC: canon §11.1 `st.corroded`: AR × `(10000 - 1000 × stacks)`.
pub const fn corroded_ar_bp(stacks: u8) -> i32 {
    10_000 - 1_000 * stacks as i32
}
/// SPEC: canon §11.1 `st.irradiated`: Max HP × `(10000 - 200 × stacks)`.
pub const fn irradiated_max_hp_bp(stacks: u8) -> i32 {
    10_000 - 200 * stacks as i32
}
/// SPEC: canon §11.1, §10.9 `st.withering`: healing received × 5000.
pub const WITHERING_HEALING_BP: i32 = 5_000;
/// SPEC: canon §11.1 `st.soaked`: shock buildup × 15000.
pub const SOAKED_SHOCK_BUILDUP_BP: i32 = 15_000;
/// SPEC: canon §11.1 `st.soaked`: fire buildup × 5000.
pub const SOAKED_FIRE_BUILDUP_BP: i32 = 5_000;
/// SPEC: canon §11.1 `st.oiled`: fire buildup × 20000.
pub const OILED_FIRE_BUILDUP_BP: i32 = 20_000;
/// SPEC: canon §11.1 `st.blessed`: all thresholds × 11000.
pub const BLESSED_THRESHOLD_BP: i32 = 11_000;
/// SPEC: canon §11.1 `st.blessed`: crit chance +1000 Bp.
pub const BLESSED_CRIT_BP: i32 = 1_000;
/// SPEC: canon §11.1 `st.regenerating`: heals at rate 200 Bp of Max HP per s.
pub const REGENERATING_RATE_BP: i32 = 200;
/// SPEC: canon §11.1 `st.shocked`: every 30 active ticks ...
pub const SHOCKED_PERIOD_TICKS: u32 = 30;
/// SPEC: canon §11.1 `st.shocked`: ... a 6-tick micro-stun.
pub const SHOCKED_MICROSTUN_TICKS: u32 = 6;
/// SPEC: canon §11.1 `st.confused`: every 30 active ticks, a keyed `rng.status` rotation.
pub const CONFUSED_PERIOD_TICKS: u32 = 30;
/// SPEC: canon §11.1 `st.dread`: composure −15 on application (Milli, §1.5).
pub const DREAD_COMPOSURE_MILLI: i64 = -15_000;
/// SPEC: canon §11.1 `st.unraveled`: −10 aether on application (Milli, §1.5).
pub const UNRAVELED_AETHER_MILLI: i64 = -10_000;
/// SPEC: canon §11.1 `st.bleeding`: a single heal of at least 10% Max HP ends it.
pub const BLEEDING_HEAL_END_BP: i32 = 1_000;
/// SPEC: canon §11.8 `st.sleeping` on a player-controlled bearer: duration × 5000.
pub const PLAYER_SLEEP_DURATION_BP: i32 = 5_000;

/// SPEC: canon §11.1 `st.irradiated`. Effective Max HP in mHP under `stacks`.
pub const fn irradiated_max_hp(max_hp_mhp: i64, stacks: u8) -> i64 {
    mul_bp(max_hp_mhp, irradiated_max_hp_bp(stacks))
}

/// SPEC: canon §12 "Max changes". When a pool's maximum changes from
/// `old_max` to `new_max`: if it grew, `current += Δ`; then
/// `current = min(current, new_max)`. Used for `st.irradiated` and every pool.
pub const fn apply_max_change(current: i64, old_max: i64, new_max: i64) -> i64 {
    let delta = new_max - old_max;
    let grown = if delta > 0 { current + delta } else { current };
    if grown < new_max { grown } else { new_max }
}

// ---------------------------------------------------------------------------
// Enemy ranks (canon §17.3)
// ---------------------------------------------------------------------------

/// SPEC: canon §17.3. Enemy rank.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(missing_docs)]
pub enum Rank {
    Minion,
    Standard,
    Elite,
    Champion,
    Boss,
    Editor,
    Hunter,
}

/// SPEC: canon §17.3. One rank row, all columns in Bp.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct RankRow {
    /// The rank.
    pub rank: Rank,
    /// Content ID, for example `rank.elite`.
    pub key: &'static str,
    /// HP multiplier.
    pub hp_bp: i32,
    /// Damage multiplier (§10.4 enemy step 0).
    pub damage_bp: i32,
    /// Kill XP multiplier (§8).
    pub kill_xp_bp: i32,
    /// DoT taken multiplier (§11.3 `rank_dot_bp`).
    pub dot_taken_bp: i32,
    /// Control duration multiplier (§11.4).
    pub control_bp: i32,
}

/// SPEC: canon §17.3. The rank table in canon order.
pub static RANKS: [RankRow; 7] = [
    RankRow {
        rank: Rank::Minion,
        key: "rank.minion",
        hp_bp: 4_000,
        damage_bp: 6_000,
        kill_xp_bp: 3_000,
        dot_taken_bp: 10_000,
        control_bp: 10_000,
    },
    RankRow {
        rank: Rank::Standard,
        key: "rank.standard",
        hp_bp: 10_000,
        damage_bp: 10_000,
        kill_xp_bp: 10_000,
        dot_taken_bp: 10_000,
        control_bp: 10_000,
    },
    RankRow {
        rank: Rank::Elite,
        key: "rank.elite",
        hp_bp: 30_000,
        damage_bp: 13_000,
        kill_xp_bp: 30_000,
        dot_taken_bp: 5_000,
        control_bp: 7_500,
    },
    RankRow {
        rank: Rank::Champion,
        key: "rank.champion",
        hp_bp: 60_000,
        damage_bp: 15_000,
        kill_xp_bp: 60_000,
        dot_taken_bp: 5_000,
        control_bp: 5_000,
    },
    RankRow {
        rank: Rank::Boss,
        key: "rank.boss",
        hp_bp: 200_000,
        damage_bp: 16_000,
        kill_xp_bp: 200_000,
        dot_taken_bp: 2_500,
        control_bp: 2_500,
    },
    RankRow {
        rank: Rank::Editor,
        key: "rank.editor",
        hp_bp: 400_000,
        damage_bp: 18_000,
        kill_xp_bp: 400_000,
        dot_taken_bp: 2_500,
        control_bp: 2_500,
    },
    RankRow {
        rank: Rank::Hunter,
        key: "rank.hunter",
        hp_bp: 120_000,
        damage_bp: 20_000,
        kill_xp_bp: 50_000,
        dot_taken_bp: 2_500,
        control_bp: 2_500,
    },
];

impl Rank {
    /// SPEC: canon §17.3. This rank's row.
    pub const fn row(self) -> &'static RankRow {
        &RANKS[self as usize]
    }

    /// SPEC: canon §11.2, §11.4, §17.3. Bosses, editors and hunters share
    /// immunities, post-proc protection (× 20000 for 15 s) and buildup control scaling.
    pub const fn is_boss_class(self) -> bool {
        matches!(self, Rank::Boss | Rank::Editor | Rank::Hunter)
    }

    /// SPEC: canon §17.3. Parse a rank content ID.
    pub fn from_key(key: &str) -> Option<Rank> {
        RANKS.iter().find(|r| r.key == key).map(|r| r.rank)
    }
}

/// SPEC: canon §11.3, §17.3. `rank_dot_bp` for a bearer; Errata (no rank) use 10000.
pub const fn rank_dot_bp(rank: Option<Rank>) -> i32 {
    match rank {
        Some(r) => r.row().dot_taken_bp,
        None => 10_000,
    }
}

/// SPEC: canon §11.4, §17.3. Rank control multiplier; Errata (no rank) use 10000.
pub const fn rank_control_bp(rank: Option<Rank>) -> i32 {
    match rank {
        Some(r) => r.row().control_bp,
        None => 10_000,
    }
}

// ---------------------------------------------------------------------------
// Bearers, thresholds and eligibility (canon §6.4 rows 22-25, §11.1, §11.4)
// ---------------------------------------------------------------------------

/// SPEC: canon §6.4 rows 22 to 25. A bearer's four buildup thresholds, as
/// computed by `max(40, 100 + 2 × d(x))` from RES, TGH, RSV and ATN.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Thresholds {
    /// Row 22, body group.
    pub body: i32,
    /// Row 23, nerve group.
    pub nerve: i32,
    /// Row 24, mind group.
    pub mind: i32,
    /// Row 25, arcane group.
    pub arcane: i32,
}

impl Thresholds {
    /// SPEC: canon §6.4 rows 22 to 25 at the human baseline (all 40): 100.
    pub const BASELINE: Thresholds = Thresholds {
        body: 100,
        nerve: 100,
        mind: 100,
        arcane: 100,
    };

    /// SPEC: canon §6.4 rows 22 to 25. The threshold of `group`.
    pub const fn get(self, group: ScalingGroup) -> i32 {
        match group {
            ScalingGroup::Body => self.body,
            ScalingGroup::Nerve => self.nerve,
            ScalingGroup::Mind => self.mind,
            ScalingGroup::Arcane => self.arcane,
        }
    }

    /// SPEC: canon §11.1 `st.blessed` "all thresholds × 11000": the bearer's
    /// thresholds with the blessed multiplier applied (one `mul_bp` each).
    pub const fn blessed(self) -> Thresholds {
        Thresholds {
            body: mul_bp_i32(self.body, BLESSED_THRESHOLD_BP),
            nerve: mul_bp_i32(self.nerve, BLESSED_THRESHOLD_BP),
            mind: mul_bp_i32(self.mind, BLESSED_THRESHOLD_BP),
            arcane: mul_bp_i32(self.arcane, BLESSED_THRESHOLD_BP),
        }
    }
}

/// SPEC: canon §11.1, §11.4, §11.8, §17.3. Bearer facts that status rules read.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct Bearer {
    /// Enemy rank; `None` for Errata and rankless entities.
    pub rank: Option<Rank>,
    /// Body plan `body.construct`.
    pub construct: bool,
    /// An electronic entity (`st.hacked` applies to it).
    pub electronic: bool,
    /// An Erratum controlled by a player (§11.8).
    pub player_controlled: bool,
}

impl Bearer {
    /// SPEC: canon §11.4, §17.3. Boss, editor or hunter.
    pub const fn is_boss_class(self) -> bool {
        match self.rank {
            Some(r) => r.is_boss_class(),
            None => false,
        }
    }
}

/// SPEC: canon §11.1 `st.hacked`, §11.4, §17.3. Whether `status` may be
/// applied to `bearer` at all:
/// - bosses, editors and hunters are immune to `st.charmed`, `st.sleeping`,
///   `st.confused`, `st.hacked` and `st.polymorphed` (§11.4; §17.3 lists the
///   first three, §11.4 is the superset);
/// - `st.hacked` applies only to `body.construct` and electronic entities;
/// - constructs are immune to every other mind-group status.
pub const fn can_receive(status: StatusId, bearer: Bearer) -> bool {
    if bearer.is_boss_class()
        && matches!(
            status,
            StatusId::Charmed
                | StatusId::Sleeping
                | StatusId::Confused
                | StatusId::Hacked
                | StatusId::Polymorphed
        )
    {
        return false;
    }
    if matches!(status, StatusId::Hacked) {
        return bearer.construct || bearer.electronic;
    }
    if bearer.construct && matches!(status.def().threshold, ThresholdGroup::Mind) {
        return false;
    }
    true
}

// ---------------------------------------------------------------------------
// Direct application and proc durations (canon §11.4, §11.8)
// ---------------------------------------------------------------------------

/// SPEC: canon §11.4 scaling factor `clamp(floor_div(1_000_000, thr), 2500, 15000)`.
/// Panics if `thr <= 0`.
pub const fn direct_scale_bp(thr: i32) -> i32 {
    let q = floor_div(1_000_000, thr as i64);
    let c = if q < 2_500 {
        2_500
    } else if q > 15_000 {
        15_000
    } else {
        q
    };
    c as i32
}

/// SPEC: canon §11.4 core formula on raw inputs: `duration = mul_bp(base,
/// scale(thr))` when scaled, then × `control_bp` (the rank control multiplier,
/// pass 10000 when it does not apply), then × `extra_bp` (§11.8 player sleep,
/// 10000 otherwise), then at least 1 tick.
pub const fn scaled_duration(
    base: u32,
    scaling_thr: Option<i32>,
    control_bp: i32,
    extra_bp: i32,
) -> u32 {
    let mut d = base as i64;
    if let Some(thr) = scaling_thr {
        d = mul_bp(d, direct_scale_bp(thr));
    }
    d = mul_bp(d, control_bp);
    d = mul_bp(d, extra_bp);
    if d < 1 { 1 } else { d as u32 }
}

const fn player_extra_bp(status: StatusId, bearer: Bearer) -> i32 {
    if bearer.player_controlled && matches!(status, StatusId::Sleeping) {
        PLAYER_SLEEP_DURATION_BP
    } else {
        10_000
    }
}

/// SPEC: canon §11.4 direct application. Returns the duration in ticks, or
/// `None` when the status has no tick duration (state-driven or until Folio
/// exit) or the bearer cannot receive it ([`can_receive`]).
///
/// `thresholds` are the bearer's effective thresholds (apply
/// [`Thresholds::blessed`] first when the bearer has `st.blessed`). Scaled
/// statuses use the threshold of their scaling group; unscaled (`—`) use the
/// base duration. Category `control` is then × the bearer's rank control
/// multiplier; a player-controlled sleeper is × 5000 (§11.8); the result is at
/// least 1 tick.
pub const fn direct_duration(
    status: StatusId,
    thresholds: Thresholds,
    bearer: Bearer,
) -> Option<u32> {
    if !can_receive(status, bearer) {
        return None;
    }
    let base = match status.base_ticks() {
        Some(b) => b,
        None => return None,
    };
    let thr = match status.def().scaling {
        Some(g) => Some(thresholds.get(g)),
        None => None,
    };
    let control = if status.is_control() {
        rank_control_bp(bearer.rank)
    } else {
        10_000
    };
    Some(scaled_duration(
        base,
        thr,
        control,
        player_extra_bp(status, bearer),
    ))
}

/// SPEC: canon §11.2 and §11.4. Duration of a status applied by a buildup
/// proc: the base duration; for bosses, editors and hunters a `control`
/// status is × the rank control multiplier ("their control statuses from
/// buildup also use the rank control multiplier"); a player-controlled
/// sleeper is × 5000 (§11.8); at least 1 tick. `None` when not receivable.
///
/// GAP: canon §11.4 names only bosses, editors and hunters for buildup
/// control scaling, so elites and champions keep the base duration here.
pub const fn proc_duration(status: StatusId, bearer: Bearer) -> Option<u32> {
    if !can_receive(status, bearer) {
        return None;
    }
    let base = match status.base_ticks() {
        Some(b) => b,
        None => return None,
    };
    let control = if status.is_control() && bearer.is_boss_class() {
        rank_control_bp(bearer.rank)
    } else {
        10_000
    };
    Some(scaled_duration(
        base,
        None,
        control,
        player_extra_bp(status, bearer),
    ))
}

// ---------------------------------------------------------------------------
// Status instances and active intervals (canon §2.3, §11.1)
// ---------------------------------------------------------------------------

/// SPEC: canon §2.3, §11.1; doc 03 `comp.statuses` `StatusInst`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct StatusInst {
    /// The status.
    pub id: StatusId,
    /// Tick `a` of the first application (periodic sub-effects use it).
    pub applied: u64,
    /// Last active tick; `None` for state-driven or until-Folio-exit statuses.
    pub expiry: Option<u64>,
    /// Intensity stacks (1 for refresh and pool statuses).
    pub stacks: u8,
    /// Pool quantity in Milli (`st.shielded`), else 0.
    pub pool: i64,
    /// Applier EntityId (0 when environmental).
    pub source: u64,
    /// Applier base-component potency snapshot (§11.3), Bp.
    pub potency_bp: i32,
    /// §2.3 rate accumulator for DoT and HoT.
    pub acc: i64,
}

/// SPEC: canon §11.1. What an application did.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ApplyResult {
    /// The status was absent and now starts.
    Started,
    /// A refresh: expiry set to `max(old, t + D)`.
    Refreshed,
    /// An intensity application added one stack.
    StackAdded,
    /// An intensity application at the cap: only the expiry refreshed.
    AtCap,
    /// A pool application.
    PoolSet,
}

/// SPEC: canon §11.1, §2.3. One application request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Application {
    /// The status.
    pub id: StatusId,
    /// Tick of application.
    pub t: u64,
    /// Duration `D` in ticks; `None` for statuses without one.
    pub duration: Option<u32>,
    /// Applier EntityId.
    pub source: u64,
    /// Applier base-component potency, Bp.
    pub potency_bp: i32,
    /// Pool amount for `pool` statuses (Milli), else 0.
    pub pool: i64,
}

impl StatusInst {
    /// SPEC: canon §2.3 "A status applied during tick `a` with duration `D`
    /// ticks is active on ticks `a+1 ..= a+D`": a fresh instance.
    pub const fn start(app: Application) -> StatusInst {
        StatusInst {
            id: app.id,
            applied: app.t,
            expiry: match app.duration {
                Some(d) => Some(app.t + d as u64),
                None => None,
            },
            stacks: 1,
            pool: app.pool,
            source: app.source,
            potency_bp: app.potency_bp,
            acc: 0,
        }
    }

    /// SPEC: canon §2.3. Active on tick `t`: `applied < t <= expiry`.
    pub const fn is_active(&self, t: u64) -> bool {
        t > self.applied
            && match self.expiry {
                Some(e) => t <= e,
                None => true,
            }
    }

    /// SPEC: canon §2.3 "removed in the status-timer step at the end of tick
    /// `a+D`": true when the status-timer step of tick `t` removes it.
    pub const fn expires_at_end_of(&self, t: u64) -> bool {
        match self.expiry {
            Some(e) => t >= e,
            None => false,
        }
    }

    /// SPEC: canon §2.3 "Periodic sub-effects of a status fire on active ticks
    /// where `(t - a) % period == 0`". Panics if `period == 0`.
    pub const fn periodic_due(&self, t: u64, period: u32) -> bool {
        assert!(period > 0, "periodic_due: period must be > 0");
        self.is_active(t) && (t - self.applied).is_multiple_of(period as u64)
    }

    /// SPEC: canon §2.3, §11.1 refresh: `expiry = max(old_expiry, t + D)`.
    /// GAP: a refresh keeps `applied`, so periodic phase never shifts.
    pub const fn refresh(&mut self, t: u64, duration: u32) {
        let new_e = t + duration as u64;
        self.expiry = match self.expiry {
            Some(e) if e >= new_e => Some(e),
            Some(_) => Some(new_e),
            None => None,
        };
    }
}

/// SPEC: canon §11.1 stacking rules and §2.3 intervals. Applies `app` to the
/// existing instance (if any, still present) and returns the new instance.
///
/// - `refresh`: expiry `max(old, t + D)`;
/// - `intensity N`: one more stack up to N, shared expiry refreshed;
/// - `pool`: GAP: the pool becomes `max(old, new)` and the expiry refreshes;
/// - state-driven rows: start if absent, otherwise unchanged.
///
/// GAP: on reapplication the source and potency snapshot become the latest
/// applier's (canon §11.3 says "potency when applied").
pub fn apply(existing: Option<&StatusInst>, app: Application) -> (StatusInst, ApplyResult) {
    let Some(old) = existing else {
        return (StatusInst::start(app), ApplyResult::Started);
    };
    let mut inst = *old;
    let d = app.duration.unwrap_or(0);
    let result = match app.id.def().stacking {
        Stacking::Refresh => {
            inst.refresh(app.t, d);
            ApplyResult::Refreshed
        }
        Stacking::Intensity(cap) => {
            inst.refresh(app.t, d);
            if inst.stacks < cap {
                inst.stacks += 1;
                ApplyResult::StackAdded
            } else {
                ApplyResult::AtCap
            }
        }
        Stacking::Pool => {
            inst.refresh(app.t, d);
            if app.pool > inst.pool {
                inst.pool = app.pool;
            }
            ApplyResult::PoolSet
        }
        Stacking::Conditional(_) => return (inst, ApplyResult::Refreshed),
    };
    inst.source = app.source;
    inst.potency_bp = app.potency_bp;
    (inst, result)
}

// ---------------------------------------------------------------------------
// Buildup (canon §11.2)
// ---------------------------------------------------------------------------

/// SPEC: canon §11.2 post-proc protection: × 15000 for 10 s.
pub const PROTECT_BP: i32 = 15_000;
/// SPEC: canon §11.2 post-proc protection duration: 10 s.
pub const PROTECT_TICKS: u32 = 600;
/// SPEC: canon §11.2 post-proc protection for bosses, editors and hunters: × 20000.
pub const BOSS_PROTECT_BP: i32 = 20_000;
/// SPEC: canon §11.2 post-proc protection duration for bosses, editors and hunters: 15 s.
pub const BOSS_PROTECT_TICKS: u32 = 900;
/// SPEC: canon §11.2 "After 2 s without new buildup".
pub const DECAY_DELAY_TICKS: u64 = 120;
/// SPEC: canon §11.2 decay per status pulse: 5% of the threshold.
pub const DECAY_BP: i32 = 500;

/// SPEC: canon §11.2. One hit's buildup inputs for the status its type carries.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct BuildupHit {
    /// Status power `p` in threshold points.
    pub power: i32,
    /// `src_lvl` for `S(src_lvl)` (§13.8.1); enemies pass `d`.
    pub src_lvl: u32,
    /// Contradiction potency of the base component, Bp.
    pub potency_bp: i32,
    /// Anachronism `edge_bp` of the base component.
    pub edge_bp: i32,
    /// The target is native (§5.6).
    pub target_native: bool,
    /// Attacker status buildup multiplier (§6.4 row 41), Bp.
    pub attacker_buildup_bp: i32,
    /// Target's clamped resistance `r` to the carrying type (§10.5 step 4).
    pub resist_bp: i32,
    /// The hit was blocked.
    pub blocked: bool,
    /// The target is immune to the carrying type (skips buildup, §10.2).
    pub immune: bool,
    /// Target has `st.oiled`.
    pub target_oiled: bool,
    /// Target has `st.soaked`.
    pub target_soaked: bool,
}

/// SPEC: canon §11.2. Buildup points `p'` a hit adds to `status`'s meter,
/// each step one `mul_bp` in this order, narrowed to `i32` after each step:
/// `p × S(src_lvl) × potency × (10000 + edge_bp) [native] × row41 × (10000 - r)
/// × 5000 [blocked]`, then the target-status multipliers in ascending status
/// ID order (§11.7): `st.oiled` × 20000 on burning, then `st.soaked` × 5000
/// on burning or × 15000 on shocked. Immune types give 0.
///
/// GAP: the §11.2 chain does not list the soaked and oiled buildup factors;
/// they are placed after it as status factors per §11.7.
pub fn buildup_points(status: StatusId, h: &BuildupHit) -> i32 {
    assert!(h.power >= 0, "buildup_points: negative status power");
    if h.immune || h.power == 0 {
        return 0;
    }
    let mut p = narrow_i32(i128::from(mul_bp_wide(
        i64::from(h.power),
        power_s(h.src_lvl),
    )));
    p = mul_bp_i32(p, h.potency_bp);
    if h.target_native {
        p = mul_bp_i32(p, 10_000 + h.edge_bp);
    }
    p = mul_bp_i32(p, h.attacker_buildup_bp);
    p = mul_bp_i32(p, 10_000 - h.resist_bp);
    if h.blocked {
        p = mul_bp_i32(p, 5_000);
    }
    if h.target_oiled && status == StatusId::Burning {
        p = mul_bp_i32(p, OILED_FIRE_BUILDUP_BP);
    }
    if h.target_soaked {
        match status {
            StatusId::Burning => p = mul_bp_i32(p, SOAKED_FIRE_BUILDUP_BP),
            StatusId::Shocked => p = mul_bp_i32(p, SOAKED_SHOCK_BUILDUP_BP),
            _ => {}
        }
    }
    p
}

/// SPEC: canon §11.2; doc 03 `comp.buildup` `Meter`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Meter {
    /// The status this meter builds.
    pub status: StatusId,
    /// Current value in threshold points (starts at 0).
    pub value: i32,
    /// Tick of the last nonzero buildup.
    pub last_add: u64,
    /// Last protected tick (inclusive) of post-proc protection.
    pub protect_until: u64,
    /// Protection multiplier, Bp (0 = never procced).
    pub protect_bp: i32,
}

impl Meter {
    /// SPEC: canon §11.2 "Meters start at 0".
    pub const fn new(status: StatusId) -> Meter {
        Meter {
            status,
            value: 0,
            last_add: 0,
            protect_until: 0,
            protect_bp: 0,
        }
    }

    /// SPEC: canon §11.2 post-proc protection. Protected on tick `t` when a
    /// proc happened and `t <= protect_until`. A proc on tick `a` protects
    /// the rest of tick `a` and ticks `a+1 ..= a+D`.
    pub const fn protected(&self, t: u64) -> bool {
        self.protect_bp != 0 && t <= self.protect_until
    }

    /// SPEC: canon §11.2 and §11.7. Effective threshold on tick `t`: the group
    /// threshold, × the post-proc protection multiplier (a non-status factor)
    /// while protected, then × 11000 when the bearer has `st.blessed`.
    pub const fn threshold(&self, group_thr: i32, t: u64, blessed: bool) -> i32 {
        let mut thr = group_thr;
        if self.protected(t) {
            thr = mul_bp_i32(thr, self.protect_bp);
        }
        if blessed {
            thr = mul_bp_i32(thr, BLESSED_THRESHOLD_BP);
        }
        thr
    }

    /// SPEC: canon §11.2. Add `points` on tick `t`. When the meter reaches the
    /// effective threshold the status procs: the meter resets to 0 (GAP: any
    /// overflow is discarded, one proc per add) and post-proc protection
    /// restarts at × 15000 for 600 ticks (bosses, editors and hunters ×
    /// 20000 for 900 ticks), never compounding. Returns true on a proc.
    pub fn add(
        &mut self,
        points: i32,
        group_thr: i32,
        t: u64,
        boss_class: bool,
        blessed: bool,
    ) -> bool {
        assert!(points >= 0, "Meter::add: negative points");
        if points == 0 {
            return false;
        }
        let thr = self.threshold(group_thr, t, blessed);
        self.value += points;
        self.last_add = t;
        if self.value >= thr {
            self.value = 0;
            let (bp, ticks) = if boss_class {
                (BOSS_PROTECT_BP, BOSS_PROTECT_TICKS)
            } else {
                (PROTECT_BP, PROTECT_TICKS)
            };
            self.protect_bp = bp;
            self.protect_until = t + u64::from(ticks);
            true
        } else {
            false
        }
    }

    /// SPEC: canon §11.2 decay, run on status pulse ticks: after 2 s without
    /// new buildup (`t - last_add >= 120`), the meter loses
    /// `floor_div(thr × 500, 10000)` (5% of its effective threshold), floored
    /// at 0. Returns the amount removed.
    pub fn decay(&mut self, group_thr: i32, t: u64, blessed: bool) -> i32 {
        if self.value <= 0 || t < self.last_add + DECAY_DELAY_TICKS {
            return 0;
        }
        let thr = self.threshold(group_thr, t, blessed);
        let step = mul_bp_i32(thr, DECAY_BP);
        let removed = if step < self.value { step } else { self.value };
        self.value -= removed;
        removed
    }
}

// ---------------------------------------------------------------------------
// Damage over time (canon §11.3) and healing over time
// ---------------------------------------------------------------------------

/// SPEC: canon §11.1, §11.3. One DoT rate-table row.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct DotRate {
    /// The DoT status.
    pub status: StatusId,
    /// Rate in Bp of effective Max HP per second (per stack for intensity rows).
    pub rate_bp: i32,
    /// Damage type the pulse carries into the pipeline (§10.6).
    pub damage: DamageType,
}

/// SPEC: canon §11.1 DoT rate table (`st.irradiated` reduces Max HP instead).
pub static DOT_RATES: [DotRate; 6] = [
    DotRate {
        status: StatusId::Bleeding,
        rate_bp: 100,
        damage: DamageType::True,
    },
    DotRate {
        status: StatusId::Burning,
        rate_bp: 200,
        damage: DamageType::Fire,
    },
    DotRate {
        status: StatusId::Corroded,
        rate_bp: 50,
        damage: DamageType::Acid,
    },
    DotRate {
        status: StatusId::Poisoned,
        rate_bp: 75,
        damage: DamageType::Toxin,
    },
    DotRate {
        status: StatusId::Suffocating,
        rate_bp: 500,
        damage: DamageType::True,
    },
    DotRate {
        status: StatusId::Withering,
        rate_bp: 50,
        damage: DamageType::Umbral,
    },
];

/// SPEC: canon §11.1, §11.3. The DoT row of `status`, if it deals damage over time.
pub fn dot_rate(status: StatusId) -> Option<DotRate> {
    DOT_RATES.iter().copied().find(|r| r.status == status)
}

/// SPEC: canon §11.3 per-second DoT in mHP:
/// `per_s = mul_bp(max_hp_eff, rate_bp × stacks)`, then × the applier's
/// potency snapshot, then × the bearer's `rank_dot_bp`.
///
/// GAP: the formula multiplies by `stacks` for every DoT, so `st.corroded`
/// (intensity 5, "DoT rate 50 Bp") scales with stacks too.
pub const fn dot_per_second(
    max_hp_eff_mhp: i64,
    rate_bp: i32,
    stacks: u8,
    potency_bp: i32,
    rank_dot: i32,
) -> i64 {
    let per_s = mul_bp(max_hp_eff_mhp, rate_bp * stacks as i32);
    let per_s = mul_bp(per_s, potency_bp);
    mul_bp(per_s, rank_dot)
}

/// SPEC: canon §11.3. One DoT pulse result: `q` mHP of `damage` entering
/// the pipeline at step 4 (§10.6).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct DotPulse {
    /// Damage type of the pulse.
    pub damage: DamageType,
    /// Amount for this pulse after the `k = 4` accumulator.
    pub q_mhp: i64,
}

/// SPEC: canon §11.3, §2.3. Run one status pulse of a DoT instance on tick
/// `t`: returns `None` unless `t` is a pulse tick, the instance is active on
/// `t` ("a pulse at tick `t` affects only statuses active on `t`") and the
/// status deals damage over time. Max HP is read at the pulse tick.
pub fn dot_pulse(
    inst: &mut StatusInst,
    t: u64,
    max_hp_eff_mhp: i64,
    bearer_rank: Option<Rank>,
) -> Option<DotPulse> {
    if !is_status_pulse(t) || !inst.is_active(t) {
        return None;
    }
    let row = dot_rate(inst.id)?;
    let per_s = dot_per_second(
        max_hp_eff_mhp,
        row.rate_bp,
        inst.stacks,
        inst.potency_bp,
        rank_dot_bp(bearer_rank),
    );
    let q = accumulate(&mut inst.acc, per_s, PULSE_K);
    Some(DotPulse {
        damage: row.damage,
        q_mhp: q,
    })
}

/// SPEC: canon §11.1 `st.regenerating` and §11.3 "Healing over time uses the
/// same accumulator": per-second healing `mul_bp(max_hp, 200)`.
pub const fn regen_per_second(max_hp_mhp: i64) -> i64 {
    mul_bp(max_hp_mhp, REGENERATING_RATE_BP)
}

// ---------------------------------------------------------------------------
// Erasure (canon §11.6, §17.1)
// ---------------------------------------------------------------------------

/// SPEC: canon §17.1 `ERASURE_ONSET`: 720 s after Folio entry (canon-1.6,
/// softer Erasure).
pub const ERASURE_ONSET_TICKS: u64 = 43_200;
/// SPEC: canon §17.1 `ERASURE_STEP`: 30 s.
pub const ERASURE_STEP_TICKS: u64 = 1_800;
/// SPEC: canon §11.1 `st.erasure` intensity cap.
pub const ERASURE_MAX_STACKS: u8 = 10;
/// SPEC: canon §11.6 hunter-spawn threshold before Red Ink doses.
pub const HUNTER_THRESHOLD: u8 = 10;
/// SPEC: canon §11.6 lowest hunter-spawn threshold under Red Ink.
pub const HUNTER_THRESHOLD_MIN: u8 = 6;
/// SPEC: canon §11.6 (canon-1.6): the Erasure hunter spawns with
/// `rank.elite` stats, and at most one spawns per Folio.
pub const ERASURE_HUNTER_STATS_RANK: Rank = Rank::Elite;
/// SPEC: canon §11.6 (canon-1.6): "No further Erasure hunters spawn in that Folio."
pub const ERASURE_HUNTERS_PER_FOLIO: u64 = 1;
/// SPEC: canon §11.6 Editors erase Errata placeables within 1500 mm ...
pub const PLACEABLE_ERASE_RADIUS_MM: i64 = 1_500;
/// SPEC: canon §11.6 ... at one placeable block per 10 ticks.
pub const PLACEABLE_ERASE_PERIOD_TICKS: u64 = 10;
/// SPEC: canon §12.1 `liq.red_ink`: +1 `st.erasure` per 300 ticks of contact.
pub const RED_INK_CONTACT_TICKS: u64 = 300;

/// SPEC: canon §17.1. Tick on which stack `k >= 1` is granted:
/// `folio_entry_tick + 43200 + 1800 × (k - 1)`, so the tenth stack lands
/// 990 s after entry. Panics if `k == 0`.
pub const fn erasure_stack_tick(folio_entry_tick: u64, k: u32) -> u64 {
    assert!(k >= 1, "erasure_stack_tick: k must be >= 1");
    folio_entry_tick + ERASURE_ONSET_TICKS + ERASURE_STEP_TICKS * (k as u64 - 1)
}

/// SPEC: canon §11.6, §17.1. Scheduled stacks after `accrued` ticks of
/// Erasure accrual since Folio entry, capped at 10. Without pauses
/// `accrued = t - folio_entry_tick`.
///
/// GAP: canon §11.6 pauses accrual in boss Folios while the boss lives and
/// §17.1 stops it once any Erratum enters the Colophon; the caller keeps an
/// accrual clock that does not advance during a pause.
pub const fn erasure_stacks_for_accrued(accrued: u64) -> u8 {
    if accrued < ERASURE_ONSET_TICKS {
        return 0;
    }
    let k = (accrued - ERASURE_ONSET_TICKS) / ERASURE_STEP_TICKS + 1;
    if k >= ERASURE_MAX_STACKS as u64 {
        ERASURE_MAX_STACKS
    } else {
        k as u8
    }
}

/// SPEC: canon §11.6. Healing-received multiplier `10000 - 300 × stacks`.
pub const fn erasure_healing_bp(stacks: u8) -> i32 {
    10_000 - 300 * stacks as i32
}

/// SPEC: canon §11.6. Each stack makes the bearer 1 dB louder (doc 09).
pub const fn erasure_loudness_db(stacks: u8) -> i32 {
    stacks as i32
}

/// SPEC: canon §11.6. Hunter-spawn threshold after `red_ink_doses` doses of
/// `afflict.red_ink`: 10 minus one per dose, at least 6.
pub const fn hunter_threshold(red_ink_doses: u32) -> u8 {
    let lowered = HUNTER_THRESHOLD as u32;
    if red_ink_doses >= lowered - HUNTER_THRESHOLD_MIN as u32 {
        HUNTER_THRESHOLD_MIN
    } else {
        (lowered - red_ink_doses) as u8
    }
}

/// SPEC: canon §11.6 (canon-1.6, softer Erasure). Erasure hunters due by
/// tick `t` when the first Erratum reached the hunter threshold on
/// `first_reach_tick`: exactly one, for the party, from that tick on (with
/// [`ERASURE_HUNTER_STATS_RANK`] stats); none before it, and never more in
/// that Folio, however long the threshold is held.
pub const fn hunters_due(first_reach_tick: u64, t: u64) -> u64 {
    if t < first_reach_tick {
        0
    } else {
        ERASURE_HUNTERS_PER_FOLIO
    }
}

// ---------------------------------------------------------------------------
// Shock chains (canon §11.5)
// ---------------------------------------------------------------------------

/// SPEC: canon §11.5 chain radius: 4 m.
pub const SHOCK_CHAIN_RADIUS_MM: i64 = 4_000;
/// SPEC: canon §10.6 chained `D2 = mul_bp(HP and ward lost, 2000)`.
pub const SHOCK_CHAIN_BP: i32 = 2_000;

/// SPEC: canon §11.5. Whether a hit on a bearer creates a chained hit: the
/// bearer has `st.shocked` active and the hit is not flagged `no_chain`.
pub const fn shock_chain_triggers(shocked_active: bool, hit_no_chain: bool) -> bool {
    shocked_active && !hit_no_chain
}

/// SPEC: canon §11.5 chain target: among `candidates` (EntityId, origin),
/// excluding the bearer and the original attacker, the one with minimum
/// `dist2` from the bearer's origin within 4 m (inclusive), ties broken by
/// lower EntityId. The caller passes only entities that can take hits.
pub fn shock_chain_target(
    bearer: u64,
    bearer_pos: Fx3,
    attacker: Option<u64>,
    candidates: &[(u64, Fx3)],
) -> Option<u64> {
    let r = i128::from(Fx::from_int(4).raw());
    let r2 = r * r;
    let mut best: Option<(i128, u64)> = None;
    for &(id, pos) in candidates {
        if id == bearer || Some(id) == attacker {
            continue;
        }
        let d2 = dist2(bearer_pos, pos);
        if d2 > r2 {
            continue;
        }
        let key = (d2, id);
        best = match best {
            Some(b) if b <= key => Some(b),
            _ => Some(key),
        };
    }
    best.map(|(_, id)| id)
}

/// SPEC: canon §10.6, §11.5. Chained hit `D2 = mul_bp(lost_mhp, 2000)` where
/// `lost_mhp` is the HP and ward the triggering hit removed.
pub const fn shock_chain_d2(lost_mhp: i64) -> i64 {
    mul_bp(lost_mhp, SHOCK_CHAIN_BP)
}

/// SPEC: canon §2.3, §11.1 `st.shocked`: a micro-stun starts on active ticks
/// where `(t - a) % 30 == 0`.
pub const fn shocked_microstun_due(inst: &StatusInst, t: u64) -> bool {
    matches!(inst.id, StatusId::Shocked) && inst.periodic_due(t, SHOCKED_PERIOD_TICKS)
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    // Interpretations of canon gaps exercised here (see doc comments):
    // proc overflow discarded; protection covers the proc tick's remainder;
    // decay uses the effective threshold; pool reapplication keeps the max;
    // refresh keeps `applied`; corroded DoT scales with stacks.

    #[test]
    fn catalog_is_ascending_and_round_trips() {
        assert_eq!(CATALOG.len(), STATUS_COUNT);
        for (i, d) in CATALOG.iter().enumerate() {
            assert_eq!(d.id as usize, i, "row {i} out of enum order");
            assert_eq!(StatusId::from_key(d.key), Some(d.id));
            assert!(d.key.starts_with("st."));
            if i > 0 {
                assert!(
                    CATALOG[i - 1].key.as_bytes() < d.key.as_bytes(),
                    "{} not ascending",
                    d.key
                );
            }
        }
        assert_eq!(StatusId::ALL[0], StatusId::Berserk);
        assert_eq!(StatusId::ALL[STATUS_COUNT - 1], StatusId::Withering);
        assert_eq!(StatusId::from_key("st.nope"), None);
    }

    #[test]
    fn catalog_rows_match_canon_spot_checks() {
        let b = StatusId::Bleeding.def();
        assert_eq!(
            (
                b.category,
                b.threshold,
                b.scaling,
                b.stacking,
                b.base,
                b.owner_doc
            ),
            (
                Category::Dot,
                ThresholdGroup::Nerve,
                Some(ScalingGroup::Nerve),
                Stacking::Intensity(5),
                BaseDuration::Ticks(360),
                5
            )
        );
        assert_eq!(StatusId::Diseased.base_ticks(), Some(18_000));
        assert_eq!(StatusId::Staggered.def().threshold, ThresholdGroup::Poise);
        assert_eq!(StatusId::Staggered.base_ticks(), Some(30));
        assert_eq!(StatusId::Stunned.base_ticks(), Some(90));
        assert_eq!(StatusId::Frozen.base_ticks(), Some(120));
        assert_eq!(StatusId::KnockedDown.base_ticks(), Some(60));
        assert_eq!(StatusId::Chilled.def().scaling, Some(ScalingGroup::Nerve));
        assert_eq!(StatusId::Chilled.def().threshold, ThresholdGroup::Direct);
        assert_eq!(
            StatusId::Polymorphed.def().scaling,
            Some(ScalingGroup::Arcane)
        );
        assert_eq!(StatusId::Shielded.def().stacking, Stacking::Pool);
        assert_eq!(StatusId::Erasure.def().stacking, Stacking::Intensity(10));
        assert_eq!(StatusId::Erasure.def().base, BaseDuration::UntilFolioExit);
        assert_eq!(StatusId::Irradiated.def().stacking, Stacking::Intensity(10));
        assert_eq!(StatusId::Incubating.base_ticks(), Some(7_200));
        assert_eq!(StatusId::Downed.base_ticks(), Some(1_800));
        assert_eq!(StatusId::DeathsDoor.base_ticks(), Some(60));
        assert_eq!(StatusId::Unmoored.base_ticks(), Some(300));
        assert_eq!(StatusId::Oiled.base_ticks(), Some(secs(20)));
        assert_eq!(StatusId::Madness.base_ticks(), None);
        assert_eq!(StatusId::Exhausted.base_ticks(), Some(120));
        assert_eq!(
            StatusId::Hidden.def().stacking,
            Stacking::Conditional(Condition::WhileUnnoticed)
        );
    }

    #[test]
    fn threshold_groups_match_derived_stat_rows_22_to_25() {
        let body = [
            StatusId::Poisoned,
            StatusId::Diseased,
            StatusId::Irradiated,
            StatusId::Withering,
            StatusId::Corroded,
            StatusId::Exposed,
        ];
        let nerve = [
            StatusId::Bleeding,
            StatusId::Burning,
            StatusId::Frozen,
            StatusId::Shocked,
            StatusId::Stunned,
            StatusId::Blinded,
            StatusId::Deafened,
        ];
        let mind = [
            StatusId::Feared,
            StatusId::Charmed,
            StatusId::Confused,
            StatusId::Sleeping,
            StatusId::Dread,
            StatusId::Berserk,
            StatusId::Taunted,
            StatusId::Hacked,
        ];
        let arcane = [StatusId::Silenced, StatusId::Unraveled];
        let mut count = 0;
        for s in StatusId::ALL {
            let expect = if body.contains(&s) {
                Some(ScalingGroup::Body)
            } else if nerve.contains(&s) {
                Some(ScalingGroup::Nerve)
            } else if mind.contains(&s) {
                Some(ScalingGroup::Mind)
            } else if arcane.contains(&s) {
                Some(ScalingGroup::Arcane)
            } else {
                None
            };
            assert_eq!(s.buildup_group(), expect, "{}", s.key());
            if expect.is_some() {
                count += 1;
            }
        }
        assert_eq!(count, 23);
    }

    #[test]
    fn rank_table_matches_canon_17_3() {
        assert_eq!(Rank::Minion.row().damage_bp, 6_000);
        assert_eq!(Rank::Elite.row().control_bp, 7_500);
        assert_eq!(Rank::Champion.row().dot_taken_bp, 5_000);
        assert_eq!(Rank::Boss.row().hp_bp, 200_000);
        assert_eq!(Rank::Editor.row().damage_bp, 18_000);
        assert_eq!(Rank::Hunter.row().kill_xp_bp, 50_000);
        assert_eq!(Rank::Hunter.row().control_bp, 2_500);
        for (i, r) in RANKS.iter().enumerate() {
            assert_eq!(r.rank as usize, i);
            assert_eq!(Rank::from_key(r.key), Some(r.rank));
        }
        assert!(
            Rank::Boss.is_boss_class()
                && Rank::Editor.is_boss_class()
                && Rank::Hunter.is_boss_class()
        );
        assert!(!Rank::Champion.is_boss_class());
        assert_eq!(rank_dot_bp(None), 10_000);
        assert_eq!(rank_control_bp(Some(Rank::Champion)), 5_000);
    }

    fn npc(rank: Rank) -> Bearer {
        Bearer {
            rank: Some(rank),
            ..Bearer::default()
        }
    }

    /// Canon §21 test 28 and the §11.4 boss stun example.
    /// thr 40: `floor_div(10^6, 40) = 25000`, clamped to 15000, `mul_bp(90, 15000) = 135`.
    /// Boss (thr 40): `mul_bp(135, 2500) = 33` (33.75 floors).
    #[test]
    fn canon_t28_direct_duration() {
        let thr40 = Thresholds {
            body: 100,
            nerve: 40,
            mind: 100,
            arcane: 100,
        };
        assert_eq!(
            direct_duration(StatusId::Stunned, thr40, Bearer::default()),
            Some(135)
        );
        assert_eq!(
            direct_duration(StatusId::Stunned, thr40, npc(Rank::Boss)),
            Some(33)
        );
        // Same numbers through the raw formula.
        assert_eq!(scaled_duration(90, Some(40), 2_500, 10_000), 33);
    }

    /// Canon §11.4 boss stun example on its own: base 90, bearer nerve
    /// threshold 40 → scale `clamp(25000, 2500, 15000) = 15000` → 135 ticks;
    /// `st.stunned` is category control, so × the boss control multiplier
    /// 2500 → 33 ticks. Editors and hunters share the 2500 column.
    #[test]
    fn canon_11_4_boss_stun_example() {
        assert_eq!(direct_scale_bp(40), 15_000);
        let thr = Thresholds {
            nerve: 40,
            ..Thresholds::BASELINE
        };
        for r in [Rank::Boss, Rank::Editor, Rank::Hunter] {
            assert_eq!(direct_duration(StatusId::Stunned, thr, npc(r)), Some(33));
        }
        assert_eq!(direct_scale_bp(100), 10_000);
        assert_eq!(direct_scale_bp(1_000), 2_500);
    }

    /// Doc 05 E7: a ward-break stun on a champion at nerve 100:
    /// `mul_bp(90, 10000) = 90`, × champion control 5000 = 45 ticks.
    #[test]
    fn direct_duration_rank_control_and_clamps() {
        let base = Thresholds::BASELINE;
        assert_eq!(
            direct_duration(StatusId::Stunned, base, npc(Rank::Champion)),
            Some(45)
        );
        assert_eq!(
            direct_duration(StatusId::Stunned, base, npc(Rank::Elite)),
            Some(67)
        ); // 90 × 7500 = 67.5
        // Lower clamp 2500 at thr >= 400: 90 × 2500 = 22.5 -> 22.
        let tough = Thresholds {
            nerve: 1_000,
            ..base
        };
        assert_eq!(
            direct_duration(StatusId::Stunned, tough, Bearer::default()),
            Some(22)
        );
        // Non-control statuses ignore rank control: slowed 240 at nerve 100 on a boss stays 240.
        assert_eq!(
            direct_duration(StatusId::Slowed, base, npc(Rank::Boss)),
            Some(240)
        );
        // Unscaled control (staggered, poise group): 30 × rank control; doc 05 parry 30 / champion 15.
        assert_eq!(
            direct_duration(StatusId::Staggered, base, npc(Rank::Standard)),
            Some(30)
        );
        assert_eq!(
            direct_duration(StatusId::Staggered, base, npc(Rank::Champion)),
            Some(15)
        );
        // At least 1 tick.
        assert_eq!(scaled_duration(1, Some(10_000), 2_500, 10_000), 1);
        // No tick duration.
        assert_eq!(
            direct_duration(StatusId::Madness, base, Bearer::default()),
            None
        );
        assert_eq!(
            direct_duration(StatusId::Erasure, base, Bearer::default()),
            None
        );
        // Blessed raises thresholds: nerve 40 -> 44: floor_div(10^6, 44) = 22727 -> 15000, still 135.
        assert_eq!(Thresholds { nerve: 40, ..base }.blessed().nerve, 44);
        // Blessed at nerve 100 -> 110: floor_div(10^6, 110) = 9090: mul_bp(90, 9090) = 81.
        assert_eq!(
            direct_duration(StatusId::Stunned, base.blessed(), Bearer::default()),
            Some(81)
        );
    }

    #[test]
    fn player_sleep_and_immunities() {
        let base = Thresholds::BASELINE;
        let player = Bearer {
            player_controlled: true,
            ..Bearer::default()
        };
        // Sleeping 480 at mind 100 -> 480, × 5000 for a player = 240.
        assert_eq!(
            direct_duration(StatusId::Sleeping, base, Bearer::default()),
            Some(480)
        );
        assert_eq!(direct_duration(StatusId::Sleeping, base, player), Some(240));
        assert_eq!(proc_duration(StatusId::Sleeping, player), Some(240));
        for s in [
            StatusId::Charmed,
            StatusId::Sleeping,
            StatusId::Confused,
            StatusId::Hacked,
            StatusId::Polymorphed,
        ] {
            for r in [Rank::Boss, Rank::Editor, Rank::Hunter] {
                assert!(!can_receive(s, npc(r)), "{} on {:?}", s.key(), r);
            }
        }
        assert!(can_receive(StatusId::Feared, npc(Rank::Boss)));
        let construct = Bearer {
            construct: true,
            ..Bearer::default()
        };
        assert!(can_receive(StatusId::Hacked, construct));
        assert!(!can_receive(StatusId::Hacked, Bearer::default()));
        assert!(can_receive(
            StatusId::Hacked,
            Bearer {
                electronic: true,
                ..Bearer::default()
            }
        ));
        assert!(!can_receive(StatusId::Feared, construct));
        assert!(!can_receive(StatusId::Taunted, construct));
        assert!(can_receive(StatusId::Stunned, construct));
        // Buildup-applied control on a boss uses rank control: stun 90 × 2500 = 22.
        assert_eq!(proc_duration(StatusId::Stunned, npc(Rank::Boss)), Some(22));
        // ... but not on a champion (canon names only bosses, editors and hunters).
        assert_eq!(
            proc_duration(StatusId::Stunned, npc(Rank::Champion)),
            Some(90)
        );
        assert_eq!(
            proc_duration(StatusId::Bleeding, npc(Rank::Boss)),
            Some(360)
        );
    }

    /// The §11.2 chain on exact numbers: p = 50, src_lvl 25 (S = 17200)
    /// -> 86; potency 6000 -> 51 (51.6); native edge 2400 -> 63 (63.24);
    /// row 41 10500 -> 66 (66.15); r = 2000 -> 52 (52.8); blocked -> 26.
    #[test]
    fn buildup_chain_exact() {
        let h = BuildupHit {
            power: 50,
            src_lvl: 25,
            potency_bp: 6_000,
            edge_bp: 2_400,
            target_native: true,
            attacker_buildup_bp: 10_500,
            resist_bp: 2_000,
            blocked: true,
            immune: false,
            target_oiled: false,
            target_soaked: false,
        };
        assert_eq!(buildup_points(StatusId::Bleeding, &h), 26);
        assert_eq!(
            buildup_points(
                StatusId::Bleeding,
                &BuildupHit {
                    blocked: false,
                    ..h
                }
            ),
            52
        );
        assert_eq!(
            buildup_points(StatusId::Bleeding, &BuildupHit { immune: true, ..h }),
            0
        );
        // Negative resistance doubles at most: r = -10000 -> × 20000.
        let plain = BuildupHit {
            power: 40,
            src_lvl: 1,
            potency_bp: 10_000,
            edge_bp: 0,
            target_native: false,
            attacker_buildup_bp: 10_000,
            resist_bp: -10_000,
            blocked: false,
            immune: false,
            target_oiled: false,
            target_soaked: false,
        };
        assert_eq!(buildup_points(StatusId::Burning, &plain), 80);
        // Oiled × 20000 then soaked × 5000 on burning: 40 -> 80 -> 40.
        let wet_oil = BuildupHit {
            resist_bp: 0,
            target_oiled: true,
            target_soaked: true,
            ..plain
        };
        assert_eq!(buildup_points(StatusId::Burning, &wet_oil), 40);
        // Soaked shock × 15000: 40 -> 60; oiled does nothing to shock.
        assert_eq!(buildup_points(StatusId::Shocked, &wet_oil), 60);
    }

    /// Doc 05 §13: bleeding power 23 per hit fills a threshold of 100 on the
    /// fifth hit (4 × 23 = 92 < 100 <= 115); the meter resets to 0.
    #[test]
    fn meter_fills_on_fifth_hit_then_protects() {
        let mut m = Meter::new(StatusId::Bleeding);
        for i in 0..4 {
            assert!(!m.add(23, 100, 10 + i, false, false));
        }
        assert_eq!(m.value, 92);
        assert!(m.add(23, 100, 14, false, false));
        assert_eq!(m.value, 0);
        // Protected: threshold 150 through tick 14 + 600, then 100 again.
        assert_eq!(m.threshold(100, 14, false), 150);
        assert_eq!(m.threshold(100, 614, false), 150);
        assert_eq!(m.threshold(100, 615, false), 100);
        // A re-proc while protected restarts the timer and does not compound.
        assert!(m.add(150, 100, 300, false, false));
        assert_eq!(m.protect_bp, PROTECT_BP);
        assert_eq!(m.threshold(100, 900, false), 150);
        assert_eq!(m.threshold(100, 901, false), 100);
        // Bosses: × 20000 for 900 ticks.
        let mut b = Meter::new(StatusId::Stunned);
        assert!(b.add(100, 100, 0, true, false));
        assert_eq!(b.threshold(100, 900, false), 200);
        assert_eq!(b.threshold(100, 901, false), 100);
        // Blessed applies after protection: 150 -> 165; 100 -> 110.
        assert_eq!(b.threshold(150, 2_000, true), 165);
        assert_eq!(m.threshold(100, 400, true), 165);
    }

    #[test]
    fn meter_decays_after_two_seconds() {
        let mut m = Meter::new(StatusId::Poisoned);
        assert!(!m.add(40, 100, 30, false, false));
        // 119 ticks later: no decay. 120 ticks later: 5% of 100 = 5 per pulse.
        assert_eq!(m.decay(100, 149, false), 0);
        assert_eq!(m.decay(100, 150, false), 5);
        assert_eq!(m.value, 35);
        for k in 1..=7 {
            m.decay(100, 150 + 15 * k, false);
        }
        assert_eq!(m.value, 0);
        assert_eq!(m.decay(100, 300, false), 0);
        // Zero buildup is not "new buildup".
        let mut z = Meter::new(StatusId::Poisoned);
        z.add(10, 100, 0, false, false);
        z.add(0, 100, 100, false, false);
        assert_eq!(z.last_add, 0);
    }

    fn app(id: StatusId, t: u64, d: u32) -> Application {
        Application {
            id,
            t,
            duration: Some(d),
            source: 7,
            potency_bp: 10_000,
            pool: 0,
        }
    }

    /// Canon §2.3: applied at `a` with `D` -> active on `a+1 ..= a+D`,
    /// removed at the end of `a+D`; a refresh takes `max(old, t + D)`.
    #[test]
    fn active_interval_semantics() {
        let (s, r) = apply(None, app(StatusId::Stunned, 100, 90));
        assert_eq!(r, ApplyResult::Started);
        assert!(!s.is_active(100));
        assert!(s.is_active(101));
        assert!(s.is_active(190));
        assert!(!s.is_active(191));
        assert!(!s.expires_at_end_of(189));
        assert!(s.expires_at_end_of(190));
        // Refresh at 150 with D = 30 keeps 190 (180 < 190); with D = 90 extends to 240.
        let (s2, _) = apply(Some(&s), app(StatusId::Stunned, 150, 30));
        assert_eq!(s2.expiry, Some(190));
        let (s3, r3) = apply(Some(&s2), app(StatusId::Stunned, 150, 90));
        assert_eq!(
            (s3.expiry, r3, s3.applied),
            (Some(240), ApplyResult::Refreshed, 100)
        );
        // Periodic sub-effects: (t - a) % 30 == 0 on active ticks.
        let (sh, _) = apply(None, app(StatusId::Shocked, 10, 180));
        assert!(!shocked_microstun_due(&sh, 10)); // not active on a
        assert!(shocked_microstun_due(&sh, 40));
        assert!(!shocked_microstun_due(&sh, 41));
        assert!(shocked_microstun_due(&sh, 190));
        assert!(!shocked_microstun_due(&sh, 220));
    }

    #[test]
    fn intensity_and_pool_stacking() {
        let (mut s, _) = apply(None, app(StatusId::Bleeding, 0, 360));
        for i in 1..5 {
            let (n, r) = apply(Some(&s), app(StatusId::Bleeding, i * 10, 360));
            assert_eq!(r, ApplyResult::StackAdded);
            s = n;
        }
        assert_eq!((s.stacks, s.expiry), (5, Some(400)));
        let (capped, r) = apply(Some(&s), app(StatusId::Bleeding, 100, 360));
        assert_eq!(
            (capped.stacks, capped.expiry, r),
            (5, Some(460), ApplyResult::AtCap)
        );
        // Pool keeps the larger amount and refreshes the expiry.
        let p1 = Application {
            pool: 20_000,
            ..app(StatusId::Shielded, 0, 600)
        };
        let (sh, _) = apply(None, p1);
        let (sh2, r) = apply(
            Some(&sh),
            Application {
                pool: 5_000,
                ..app(StatusId::Shielded, 60, 600)
            },
        );
        assert_eq!(
            (sh2.pool, sh2.expiry, r),
            (20_000, Some(660), ApplyResult::PoolSet)
        );
        let (sh3, _) = apply(
            Some(&sh2),
            Application {
                pool: 30_000,
                ..app(StatusId::Shielded, 61, 600)
            },
        );
        assert_eq!(sh3.pool, 30_000);
    }

    /// Canon §21 test 10 (the `k = 4` part): 850 mHP per second over 4
    /// pulses adds exactly 850.
    #[test]
    fn pulse_accumulator_has_no_drift() {
        let mut acc = 0;
        let sum: i64 = (0..4).map(|_| accumulate(&mut acc, 850, PULSE_K)).sum();
        assert_eq!(sum, 850);
        assert_eq!(acc, 0);
        let mut acc60 = 0;
        let s60: i64 = (0..60).map(|_| accumulate(&mut acc60, 23, 60)).sum();
        assert_eq!(s60, 23);
        assert!(is_status_pulse(0) && is_status_pulse(15) && !is_status_pulse(16));
    }

    /// Bleeding 3 stacks on 85,000 mHP: `mul_bp(85000, 300) = 2550` mHP/s;
    /// × potency 6000 = 1530; × elite DoT taken 5000 = 765.
    /// Pulses at k = 4 deliver 637, 638, 637, 638 for 2550 per second.
    #[test]
    fn dot_formula_and_pulses() {
        assert_eq!(dot_per_second(85_000, 100, 3, 10_000, 10_000), 2_550);
        assert_eq!(dot_per_second(85_000, 100, 3, 6_000, 10_000), 1_530);
        assert_eq!(
            dot_per_second(85_000, 100, 3, 6_000, rank_dot_bp(Some(Rank::Elite))),
            765
        );
        let mut inst = StatusInst::start(app(StatusId::Bleeding, 0, 360));
        inst.stacks = 3;
        let mut got = [0i64; 4];
        for (i, t) in [15u64, 30, 45, 60].into_iter().enumerate() {
            let p = dot_pulse(&mut inst, t, 85_000, None).unwrap();
            assert_eq!(p.damage, DamageType::True);
            got[i] = p.q_mhp;
        }
        assert_eq!(got, [637, 638, 637, 638]);
        // Not a pulse tick, or not active: nothing.
        assert_eq!(dot_pulse(&mut inst, 61, 85_000, None), None);
        assert_eq!(dot_pulse(&mut inst, 375, 85_000, None), None);
        // Statuses without a DoT row.
        let mut stun = StatusInst::start(app(StatusId::Stunned, 0, 90));
        assert_eq!(dot_pulse(&mut stun, 15, 85_000, None), None);
        assert_eq!(
            dot_rate(StatusId::Withering).unwrap().damage,
            DamageType::Umbral
        );
        assert_eq!(dot_rate(StatusId::Suffocating).unwrap().rate_bp, 500);
        assert_eq!(regen_per_second(85_000), 1_700);
        // Irradiated 5 stacks: Max HP × 9000; max-change rule never raises current above new max.
        assert_eq!(irradiated_max_hp(85_000, 5), 76_500);
        assert_eq!(apply_max_change(80_000, 85_000, 76_500), 76_500);
        assert_eq!(apply_max_change(50_000, 76_500, 85_000), 58_500);
    }

    #[test]
    fn erasure_schedule() {
        let entry = 1_000;
        // Canon §17.1 (canon-1.6): onset 720 s, step 30 s, tenth stack 990 s
        // after entry.
        assert_eq!(erasure_stack_tick(entry, 1), 44_200);
        assert_eq!(erasure_stack_tick(entry, 2), 46_000);
        assert_eq!(erasure_stack_tick(entry, 10), 1_000 + 990 * 60);
        assert_eq!(erasure_stacks_for_accrued(43_199), 0);
        assert_eq!(erasure_stacks_for_accrued(43_200), 1);
        assert_eq!(erasure_stacks_for_accrued(44_999), 1);
        assert_eq!(erasure_stacks_for_accrued(45_000), 2);
        assert_eq!(erasure_stacks_for_accrued(59_399), 9);
        assert_eq!(erasure_stacks_for_accrued(59_400), 10);
        assert_eq!(erasure_stacks_for_accrued(900_000), 10);
        assert_eq!(erasure_healing_bp(10), 7_000);
        assert_eq!(erasure_loudness_db(4), 4);
        assert_eq!(
            (
                hunter_threshold(0),
                hunter_threshold(3),
                hunter_threshold(4),
                hunter_threshold(9)
            ),
            (10, 7, 6, 6)
        );
        assert_eq!(
            (
                hunters_due(500, 499),
                hunters_due(500, 500),
                hunters_due(500, 5_900),
                hunters_due(500, 900_000)
            ),
            (0, 1, 1, 1)
        );
        // One hunter for the party, with elite stats (not rank.hunter's).
        assert_eq!(ERASURE_HUNTER_STATS_RANK.row().hp_bp, 30_000);
    }

    #[test]
    fn shock_chain_target_rule() {
        let bearer = 10;
        let at = |x_mm: i64, z_mm: i64| Fx3::from_milli(x_mm, 0, z_mm);
        let cands = [
            (3, at(3_000, 0)),
            (10, at(0, 0)),
            (4, at(0, 2_000)),
            (5, at(2_000, 0)),
            (2, at(0, -2_000)),
        ];
        // Attacker 2 excluded; 4 and 5 tie at 2 m -> lower id 4.
        assert_eq!(
            shock_chain_target(bearer, at(0, 0), Some(2), &cands),
            Some(4)
        );
        // Without excluding 2, id 2 wins the tie.
        assert_eq!(shock_chain_target(bearer, at(0, 0), None, &cands), Some(2));
        // Exactly 4 m is inside; beyond is not.
        assert_eq!(
            shock_chain_target(bearer, at(0, 0), None, &[(9, at(4_000, 0))]),
            Some(9)
        );
        assert_eq!(
            shock_chain_target(bearer, at(0, 0), None, &[(9, at(4_001, 0))]),
            None
        );
        assert!(shock_chain_triggers(true, false));
        assert!(!shock_chain_triggers(true, true));
        assert!(!shock_chain_triggers(false, false));
        assert_eq!(shock_chain_d2(26_761), 5_352);
    }

    #[test]
    fn effect_constants() {
        assert_eq!(exposed_damage_taken_bp(5), 14_000);
        assert_eq!(corroded_ar_bp(3), 7_000);
        assert_eq!(irradiated_max_hp_bp(10), 8_000);
    }

    proptest! {
        #[test]
        fn prop_direct_duration_bounds_and_monotone(base in 1u32..20_000, thr in 1i32..5_000, ctl in prop::sample::select(&[2_500i32, 5_000, 7_500, 10_000][..])) {
            let d = scaled_duration(base, Some(thr), ctl, 10_000);
            prop_assert!(d >= 1);
            let hi = mul_bp(mul_bp(i64::from(base), 15_000), ctl).max(1);
            prop_assert!(i64::from(d) <= hi);
            let d2 = scaled_duration(base, Some(thr + 1), ctl, 10_000);
            prop_assert!(d2 <= d, "duration must not grow with threshold");
        }

        #[test]
        fn prop_buildup_monotone_nonnegative(p in 0i32..2_000, lvl in 1u32..400, pot in 0i32..=10_000, r in -10_000i32..=9_000, blocked: bool) {
            let h = BuildupHit { power: p, src_lvl: lvl, potency_bp: pot, edge_bp: 0, target_native: false, attacker_buildup_bp: 10_000, resist_bp: r, blocked, immune: false, target_oiled: false, target_soaked: false };
            let a = buildup_points(StatusId::Burning, &h);
            let b = buildup_points(StatusId::Burning, &BuildupHit { power: p + 1, ..h });
            prop_assert!(a >= 0 && b >= a);
        }

        #[test]
        fn prop_accumulator_exact_per_second(rate in 0i64..10_000_000, secs_n in 1usize..30) {
            let mut acc = 0;
            let mut sum = 0;
            for _ in 0..(secs_n * 4) {
                sum += accumulate(&mut acc, rate, PULSE_K);
                prop_assert!((0..PULSE_K).contains(&acc));
            }
            prop_assert_eq!(sum, rate * secs_n as i64);
        }

        #[test]
        fn prop_fresh_status_active_exactly_d_ticks(a in 0u64..1_000_000, d in 1u32..20_000) {
            let s = StatusInst::start(app(StatusId::Burning, a, d));
            let active = (a..=a + u64::from(d) + 1).filter(|&t| s.is_active(t)).count();
            prop_assert_eq!(active, d as usize);
        }

        #[test]
        fn prop_refresh_never_shortens(a in 0u64..10_000, d in 1u32..5_000, dt in 0u64..10_000, d2 in 1u32..5_000) {
            let s = StatusInst::start(app(StatusId::Burning, a, d));
            let (r, _) = apply(Some(&s), app(StatusId::Burning, a + dt, d2));
            prop_assert!(r.expiry >= s.expiry);
            prop_assert_eq!(r.expiry, Some((a + u64::from(d)).max(a + dt + u64::from(d2))));
        }

        #[test]
        fn prop_shock_chain_order_invariant(pts in prop::collection::vec((1u64..50, -5_000i64..5_000, -5_000i64..5_000), 0..12), rot in 0usize..12) {
            let cands: Vec<(u64, Fx3)> = pts.iter().map(|&(id, x, z)| (id, Fx3::from_milli(x, 0, z))).collect();
            let mut rotated = cands.clone();
            if !rotated.is_empty() {
                let k = rot % rotated.len();
                rotated.rotate_left(k);
            }
            let a = shock_chain_target(1, Fx3::ZERO, Some(2), &cands);
            let b = shock_chain_target(1, Fx3::ZERO, Some(2), &rotated);
            prop_assert_eq!(a, b);
            prop_assert!(a != Some(1) && a != Some(2));
        }

        #[test]
        fn prop_erasure_monotone(x in 0u64..100_000, dx in 0u64..10_000) {
            let a = erasure_stacks_for_accrued(x);
            let b = erasure_stacks_for_accrued(x + dx);
            prop_assert!(a <= b && b <= ERASURE_MAX_STACKS);
            if a >= 1 {
                prop_assert!(erasure_stack_tick(0, u32::from(a)) <= x);
            }
        }
    }
}
