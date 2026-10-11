//! SPEC: canon §10 damage types, resistances, armor, source magnitude, the
//! damage pipeline, other entry points, blocking, lethal resolution and
//! healing; §17.2 party damage; §17.5 friendly fire; doc 05 §10 and §12.
//!
//! Every function is pure over explicit input structs. [`resolve_hit`] runs
//! canon §10.5 steps 1 to 7 in the exact canonical order and returns a
//! [`DamageBreakdown`] whose per-type values sum to the total (step 9).
//! Step 8 (HP, condition pool, poise, buildup, lethal) is split into the
//! helpers [`apply_hp`], [`apply_part_pool`], [`poise_damage_base_mp`], the
//! breakdown's `buildup` column and [`resolve_lethal`], because each needs
//! state that the caller owns. The §10.6 entry points are separate functions.
//!
//! Interpretations of canon gaps are marked `GAP:` in the doc comments.

use crate::status::{self, BuildupHit, Rank, StatusId, buildup_points};
use palimpsest_math::cordic::yaw_from_dir;
use palimpsest_math::int::{floor_div, floor_div_i128, mul_bp, mul_bp_wide, narrow_i64};
use palimpsest_math::rng::streams;
use palimpsest_math::tables::{power_p, power_s, power_t};
use palimpsest_math::{Bam, Fx, Fx3, Rng, ang_diff, fx_from_milli, keyed_rng};

// ---------------------------------------------------------------------------
// Damage types (canon §10.1)
// ---------------------------------------------------------------------------

/// SPEC: canon §10.1. Number of damage types.
pub const TYPE_COUNT: usize = 15;

/// SPEC: canon §10.1. Damage types in canon row order; the derived `Ord` is
/// that row order ("ties to the earlier row", ward order in step 7).
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(missing_docs)]
pub enum DamageType {
    Slash,
    Pierce,
    Blunt,
    Fire,
    Frost,
    Shock,
    Acid,
    Radiation,
    Sonic,
    Toxin,
    Radiant,
    Umbral,
    Psychic,
    Arcane,
    True,
}

/// SPEC: canon §10.1 "Category" column.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(missing_docs)]
pub enum DamageCategory {
    Physical,
    Elemental,
    Biological,
    Esoteric,
    True,
}

/// SPEC: canon §10.1. One damage-type row.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct DamageTypeDef {
    /// The type.
    pub ty: DamageType,
    /// Content ID, for example `dmg.slash`.
    pub key: &'static str,
    /// Category.
    pub category: DamageCategory,
    /// Default color as a `#RRGGBB` hex string.
    pub color: &'static str,
    /// The status this type builds (`None` for `dmg.true`).
    pub builds: Option<StatusId>,
    /// Armor efficiency, Bp.
    pub armor_eff_bp: i32,
    /// Ward depletion multiplier, Bp.
    pub ward_mult_bp: i32,
}

const fn dt(
    ty: DamageType,
    key: &'static str,
    category: DamageCategory,
    color: &'static str,
    builds: Option<StatusId>,
    armor_eff_bp: i32,
    ward_mult_bp: i32,
) -> DamageTypeDef {
    DamageTypeDef {
        ty,
        key,
        category,
        color,
        builds,
        armor_eff_bp,
        ward_mult_bp,
    }
}

/// SPEC: canon §10.1. The damage-type table in canon row order, indexed by
/// `DamageType as usize`.
pub static DAMAGE_TYPES: [DamageTypeDef; TYPE_COUNT] = {
    use DamageCategory as K;
    use DamageType as D;
    use StatusId as St;
    [
        dt(
            D::Slash,
            "dmg.slash",
            K::Physical,
            "#C9CED6",
            Some(St::Bleeding),
            10_000,
            10_000,
        ),
        dt(
            D::Pierce,
            "dmg.pierce",
            K::Physical,
            "#E8E2C8",
            Some(St::Exposed),
            7_500,
            10_000,
        ),
        dt(
            D::Blunt,
            "dmg.blunt",
            K::Physical,
            "#A68A64",
            Some(St::Stunned),
            5_000,
            10_000,
        ),
        dt(
            D::Fire,
            "dmg.fire",
            K::Elemental,
            "#FF6A1A",
            Some(St::Burning),
            0,
            7_500,
        ),
        dt(
            D::Frost,
            "dmg.frost",
            K::Elemental,
            "#9BE7FF",
            Some(St::Frozen),
            0,
            10_000,
        ),
        dt(
            D::Shock,
            "dmg.shock",
            K::Elemental,
            "#FFE14D",
            Some(St::Shocked),
            0,
            15_000,
        ),
        dt(
            D::Acid,
            "dmg.acid",
            K::Elemental,
            "#9AE13B",
            Some(St::Corroded),
            0,
            7_500,
        ),
        dt(
            D::Radiation,
            "dmg.radiation",
            K::Elemental,
            "#2F6BFF",
            Some(St::Irradiated),
            0,
            10_000,
        ),
        dt(
            D::Sonic,
            "dmg.sonic",
            K::Elemental,
            "#C8B6FF",
            Some(St::Deafened),
            2_500,
            10_000,
        ),
        dt(
            D::Toxin,
            "dmg.toxin",
            K::Biological,
            "#6B8E23",
            Some(St::Poisoned),
            0,
            5_000,
        ),
        dt(
            D::Radiant,
            "dmg.radiant",
            K::Esoteric,
            "#FFF4C2",
            Some(St::Blinded),
            0,
            10_000,
        ),
        dt(
            D::Umbral,
            "dmg.umbral",
            K::Esoteric,
            "#5B2A86",
            Some(St::Withering),
            0,
            10_000,
        ),
        dt(
            D::Psychic,
            "dmg.psychic",
            K::Esoteric,
            "#FF5FD2",
            Some(St::Dread),
            0,
            10_000,
        ),
        dt(
            D::Arcane,
            "dmg.arcane",
            K::Esoteric,
            "#3FE0D0",
            Some(St::Unraveled),
            0,
            12_500,
        ),
        dt(D::True, "dmg.true", K::True, "#FFFFFF", None, 0, 10_000),
    ]
};

impl DamageType {
    /// SPEC: canon §10.1. Every type in canon row order.
    pub const ALL: [DamageType; TYPE_COUNT] = [
        DamageType::Slash,
        DamageType::Pierce,
        DamageType::Blunt,
        DamageType::Fire,
        DamageType::Frost,
        DamageType::Shock,
        DamageType::Acid,
        DamageType::Radiation,
        DamageType::Sonic,
        DamageType::Toxin,
        DamageType::Radiant,
        DamageType::Umbral,
        DamageType::Psychic,
        DamageType::Arcane,
        DamageType::True,
    ];

    /// SPEC: canon §10.1. This type's row.
    pub const fn def(self) -> &'static DamageTypeDef {
        &DAMAGE_TYPES[self as usize]
    }

    /// SPEC: canon §1.1, §10.1. The content ID, for example `dmg.slash`.
    pub const fn key(self) -> &'static str {
        DAMAGE_TYPES[self as usize].key
    }

    /// SPEC: canon §1.1, §10.1. Parse a damage-type content ID.
    pub fn from_key(key: &str) -> Option<DamageType> {
        DAMAGE_TYPES.iter().find(|d| d.key == key).map(|d| d.ty)
    }

    /// SPEC: canon §10.1. Index into per-type arrays.
    pub const fn index(self) -> usize {
        self as usize
    }
}

/// SPEC: canon §10.1 `dmg.blunt`: poise damage × 15000 (doc 05 §12.4 pro rata).
pub const BLUNT_POISE_BP: i32 = 15_000;
/// SPEC: canon §10.1 `dmg.radiant`: × 15000 against undead and fiends (step 3a).
pub const RADIANT_UNHOLY_BP: i32 = 15_000;
/// SPEC: canon §10.1 `dmg.sonic` noise base: `60 + floor_div(d_mhp, 1000)` dB.
pub const SONIC_NOISE_BASE_DB: i32 = 60;

/// SPEC: canon §10.1 `dmg.sonic`: each hit emits `ev.noise` at
/// `60 + floor_div(d_mhp, 1000)` dB at the impact point.
pub const fn sonic_noise_db(d_mhp: i64) -> i64 {
    SONIC_NOISE_BASE_DB as i64 + floor_div(d_mhp, 1_000)
}

// ---------------------------------------------------------------------------
// Resistances (canon §10.2)
// ---------------------------------------------------------------------------

/// SPEC: canon §10.2, §10.5 step 6. Errata and NPC targets differ in
/// resistance caps and chip floor.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TargetKind {
    /// A player character (`fac.errata` Erratum).
    Erratum,
    /// Any NPC, minions included.
    Npc,
}

/// SPEC: canon §10.2 resistance floor: −10000 (takes double).
pub const RESIST_FLOOR_BP: i32 = -10_000;
/// SPEC: canon §10.2 Errata cap for slash, pierce and blunt.
pub const ERRATA_PHYSICAL_RESIST_CAP_BP: i32 = 5_000;
/// SPEC: canon §10.2 Errata cap for every other type (sonic included).
pub const ERRATA_RESIST_CAP_BP: i32 = 7_500;
/// SPEC: canon §10.2 NPC cap for every type.
pub const NPC_RESIST_CAP_BP: i32 = 9_000;

/// SPEC: canon §10.2. Resistance cap; `None` for `dmg.true`, which has no resistance.
pub const fn resist_cap_bp(kind: TargetKind, ty: DamageType) -> Option<i32> {
    match (kind, ty) {
        (_, DamageType::True) => None,
        (TargetKind::Npc, _) => Some(NPC_RESIST_CAP_BP),
        (TargetKind::Erratum, DamageType::Slash | DamageType::Pierce | DamageType::Blunt) => {
            Some(ERRATA_PHYSICAL_RESIST_CAP_BP)
        }
        (TargetKind::Erratum, _) => Some(ERRATA_RESIST_CAP_BP),
    }
}

/// SPEC: canon §10.5 step 4 `r = clamp(resist[t], -10000, cap)`; sources add
/// linearly before the cap (§10.2), so `raw` is that sum. `dmg.true` gives 0.
pub const fn clamp_resist_bp(kind: TargetKind, ty: DamageType, raw: i32) -> i32 {
    match resist_cap_bp(kind, ty) {
        None => 0,
        Some(cap) => {
            if raw < RESIST_FLOOR_BP {
                RESIST_FLOOR_BP
            } else if raw > cap {
                cap
            } else {
                raw
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Type shares and type sets
// ---------------------------------------------------------------------------

/// SPEC: canon §10.5 inputs: type shares `share[t]` in Bp summing to 10000.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Shares([i32; TYPE_COUNT]);

/// SPEC: canon §10.5. Why a share vector was rejected.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SharesError {
    /// A share outside `0..=10000`.
    OutOfRange,
    /// The same type listed twice.
    Duplicate,
    /// Shares do not sum to 10000.
    BadSum,
}

impl Shares {
    /// SPEC: canon §10.5. 100% of one type.
    pub const fn single(ty: DamageType) -> Shares {
        let mut s = [0; TYPE_COUNT];
        s[ty as usize] = 10_000;
        Shares(s)
    }

    /// SPEC: canon §10.5. Build from `(type, share_bp)` pairs; validates range,
    /// duplicates and the 10000 sum.
    pub fn from_pairs(pairs: &[(DamageType, i32)]) -> Result<Shares, SharesError> {
        let mut s = [0; TYPE_COUNT];
        let mut seen = TypeSet::EMPTY;
        let mut sum: i64 = 0;
        for &(ty, bp) in pairs {
            if !(0..=10_000).contains(&bp) {
                return Err(SharesError::OutOfRange);
            }
            if seen.contains(ty) {
                return Err(SharesError::Duplicate);
            }
            seen = seen.with(ty);
            s[ty as usize] = bp;
            sum += i64::from(bp);
        }
        if sum != 10_000 {
            return Err(SharesError::BadSum);
        }
        Ok(Shares(s))
    }

    /// SPEC: canon §10.5. The share of `ty`, Bp.
    pub const fn get(&self, ty: DamageType) -> i32 {
        self.0[ty as usize]
    }

    /// SPEC: canon §10.5. All shares in §10.1 row order.
    pub const fn as_array(&self) -> &[i32; TYPE_COUNT] {
        &self.0
    }

    /// SPEC: canon §10.5 step 3. The type with the largest share, ties to the
    /// earlier §10.1 row, among types for which `eligible` holds.
    pub fn largest_where(&self, eligible: impl Fn(DamageType) -> bool) -> Option<DamageType> {
        let mut best: Option<DamageType> = None;
        for ty in DamageType::ALL {
            if !eligible(ty) {
                continue;
            }
            match best {
                Some(b) if self.get(b) >= self.get(ty) => {}
                _ => best = Some(ty),
            }
        }
        best
    }

    /// SPEC: canon §10.5 step 3. The type with the largest share, ties to the
    /// earlier §10.1 row.
    pub fn largest(&self) -> DamageType {
        self.largest_where(|_| true).unwrap_or(DamageType::Slash)
    }
}

/// SPEC: canon §10.2 `immune: [types]`. A set of damage types.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct TypeSet(u16);

impl TypeSet {
    /// SPEC: canon §10.2. No types.
    pub const EMPTY: TypeSet = TypeSet(0);

    /// SPEC: canon §10.2. This set plus `ty`.
    pub const fn with(self, ty: DamageType) -> TypeSet {
        TypeSet(self.0 | (1u16 << ty as u16))
    }

    /// SPEC: canon §10.2. Whether `ty` is in the set.
    pub const fn contains(self, ty: DamageType) -> bool {
        self.0 & (1u16 << ty as u16) != 0
    }
}

// ---------------------------------------------------------------------------
// Source magnitude, step 0 (canon §10.4, §13.7, §13.8, §13.8.1)
// ---------------------------------------------------------------------------

/// SPEC: canon §13.7, §10.4 enhancement factor `10000 + 600 × eff_enh`, with
/// `eff_enh` in `-3..=15`. Panics outside that range.
pub const fn enhancement_bp(eff_enh: i32) -> i32 {
    assert!(eff_enh >= -3 && eff_enh <= 15, "eff_enh outside -3..=15");
    10_000 + 600 * eff_enh
}

/// SPEC: canon §10.4. Inputs of an Errata item source.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ItemSource {
    /// Archetype `base_mhp` after doc 05 attack factors.
    pub base_mhp: i64,
    /// `src_lvl` (§13.8.1).
    pub src_lvl: u32,
    /// Rarity power (§13.3), Bp.
    pub rarity_bp: i32,
    /// Quality multiplier (§13.5), Bp.
    pub quality_bp: i32,
    /// Effective enhancement (§13.7), `-3..=15`.
    pub eff_enh: i32,
    /// Primary material `dmg_bp` (doc 07), `7000..=14000`.
    pub material_bp: i32,
}

/// SPEC: canon §10.4 Errata item source: `B = base_mhp → × P(src_lvl) →
/// × rarity → × quality → × (10000 + 600 × eff_enh) → × material_bp`, one
/// `mul_bp` per arrow in that order. Panics if the material is outside `7000..=14000`.
pub fn errata_item_source_mhp(s: &ItemSource) -> i64 {
    assert!(
        (7_000..=14_000).contains(&s.material_bp),
        "material_bp outside 7000..=14000"
    );
    let b = mul_bp_wide(s.base_mhp, power_p(s.src_lvl));
    let b = mul_bp(b, s.rarity_bp);
    let b = mul_bp(b, s.quality_bp);
    let b = mul_bp(b, enhancement_bp(s.eff_enh));
    mul_bp(b, s.material_bp)
}

/// SPEC: canon §10.4. Inputs of an Errata armor piece's AR on one part.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ArmorSource {
    /// Archetype base AR for the part.
    pub base_ar: i64,
    /// Item level.
    pub ilvl: u32,
    /// Rarity power, Bp.
    pub rarity_bp: i32,
    /// Quality multiplier, Bp.
    pub quality_bp: i32,
    /// Effective enhancement, `-3..=15`.
    pub eff_enh: i32,
    /// Primary material `ar_bp`, `7000..=14000`.
    pub mat_ar_bp: i32,
}

/// SPEC: canon §10.4 Errata armor: `AR = base_ar → × T(ilvl) → × rarity →
/// × quality → × (10000 + 600 × eff_enh) → × mat_ar_bp`, one `mul_bp` each.
pub fn errata_armor_ar(s: &ArmorSource) -> i64 {
    assert!(
        (7_000..=14_000).contains(&s.mat_ar_bp),
        "mat_ar_bp outside 7000..=14000"
    );
    let a = mul_bp_wide(s.base_ar, power_t(s.ilvl));
    let a = mul_bp(a, s.rarity_bp);
    let a = mul_bp(a, s.quality_bp);
    let a = mul_bp(a, enhancement_bp(s.eff_enh));
    mul_bp(a, s.mat_ar_bp)
}

/// SPEC: canon §10.4 enemy source: `B = template_dmg_mhp → × T(d) → × rank damage`.
pub fn enemy_source_mhp(template_dmg_mhp: i64, d: u32, rank: Rank) -> i64 {
    mul_bp(
        mul_bp_wide(template_dmg_mhp, power_t(d)),
        rank.row().damage_bp,
    )
}

/// SPEC: canon §10.3 "Enemy AR scales with `P(d)`": `mul_bp(base_ar, P(d))`.
pub fn enemy_armor_ar(base_ar: i64, d: u32) -> i64 {
    mul_bp_wide(base_ar, power_p(d))
}

/// SPEC: canon §10.3. AR of a part after `st.corroded` and `st.fortified`,
/// in ascending status ID order (corroded `10000 - 1000 × stacks`, then
/// fortified 12000), one `mul_bp` each.
pub const fn effective_ar(ar: i64, corroded_stacks: u8, fortified: bool) -> i64 {
    let mut a = ar;
    if corroded_stacks > 0 {
        a = mul_bp(a, status::corroded_ar_bp(corroded_stacks));
    }
    if fortified {
        a = mul_bp(a, status::FORTIFIED_BP);
    }
    a
}

/// SPEC: canon §13.8.1. What kind of source a magnitude comes from.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SourceLevel {
    /// Weapon, off-hand or thrown item: its `ilvl`.
    Item {
        /// Item level.
        ilvl: u32,
    },
    /// Spell, prayer or glyph: the casting implement's `ilvl`, or
    /// `max(1, d_current - 3)` without one.
    Spell {
        /// `ilvl` of the equipped casting implement, if any.
        implement_ilvl: Option<u32>,
        /// Current depth.
        d_current: u32,
    },
    /// Crafted or brewed item: the Folio index where it was made (its `ilvl`).
    Crafted {
        /// Stored `ilvl`.
        ilvl: u32,
    },
    /// Summon: the summoner's `src_lvl` for the summoning spell.
    Summon {
        /// Summoner's `src_lvl`.
        summoner_src_lvl: u32,
    },
    /// Innate ability or Echo trait: `d_current`.
    Innate {
        /// Current depth.
        d_current: u32,
    },
    /// Environmental hazard not from an item: `d_current`.
    Environmental {
        /// Current depth.
        d_current: u32,
    },
    /// Every enemy ability: `d_current` (through `T(d)`).
    Enemy {
        /// Current depth.
        d_current: u32,
    },
}

/// SPEC: canon §13.8.1 `src_lvl`. Always at least 1.
pub const fn src_lvl(s: SourceLevel) -> u32 {
    let v = match s {
        SourceLevel::Item { ilvl } | SourceLevel::Crafted { ilvl } => ilvl,
        SourceLevel::Spell {
            implement_ilvl: Some(i),
            ..
        } => i,
        SourceLevel::Spell {
            implement_ilvl: None,
            d_current,
        } => d_current.saturating_sub(3),
        SourceLevel::Summon { summoner_src_lvl } => summoner_src_lvl,
        SourceLevel::Innate { d_current }
        | SourceLevel::Environmental { d_current }
        | SourceLevel::Enemy { d_current } => d_current,
    };
    if v < 1 { 1 } else { v }
}

// ---------------------------------------------------------------------------
// Step 2 helpers (canon §10.5 step 2, §5.7, §6.3, §9, §17.2, §17.5)
// ---------------------------------------------------------------------------

/// SPEC: canon §9 default precision multiplier, ranged weapons.
pub const PRECISION_RANGED_BP: i32 = 15_000;
/// SPEC: canon §9 default precision multiplier, melee weapons.
pub const PRECISION_MELEE_BP: i32 = 12_500;
/// SPEC: canon §6.4 row 14 base crit damage multiplier (NPCs always use it, §6.5).
pub const CRIT_MULT_BP: i32 = 15_000;
/// SPEC: canon §10.5 step 1 lower clamp of the additive bucket.
pub const ADD_MIN_BP: i32 = -9_000;
/// SPEC: canon §10.5 step 2.4a elevation threshold: 2500 mm.
pub const ELEVATION_MM: i64 = 2_500;
/// SPEC: canon §10.5 step 2.4a: attacker at least 2500 mm above.
pub const ELEVATION_HIGH_BP: i32 = 11_000;
/// SPEC: canon §10.5 step 2.4a: attacker at least 2500 mm below.
pub const ELEVATION_LOW_BP: i32 = 9_500;
/// SPEC: canon §10.5 step 2.4b frontal cone half-angle: 10923 Bam.
pub const FRONTAL_CONE_BAM: u16 = 10_923;
/// SPEC: canon §10.5 step 2.4b flanking multiplier.
pub const FLANKING_BP: i32 = 11_500;
/// SPEC: canon §10.5 step 2.10 area damage against `body.swarm`.
pub const SWARM_AREA_BP: i32 = 15_000;

/// SPEC: canon §10.5 step 1 additive bucket: `add = row12 + Σ mul_bp(bonus_i,
/// potency_i)` clamped to at least −9000. `weapon_scaling_bp` is §6.4 row 12
/// for weapon attacks by Errata and 0 otherwise.
pub fn additive_bp(weapon_scaling_bp: i32, bonuses: &[Bonus]) -> i32 {
    let mut add = i64::from(weapon_scaling_bp);
    for b in bonuses {
        add += mul_bp(i64::from(b.bonus_bp), b.potency_bp);
    }
    let add = if add < i64::from(ADD_MIN_BP) {
        i64::from(ADD_MIN_BP)
    } else {
        add
    };
    i32::try_from(add).expect("additive bucket overflows i32")
}

/// SPEC: canon §10.5 step 2.4a. With `dy = attacker.y - target.y`: 11000 if
/// `dy >= 2500 mm`, 9500 if `dy <= -2500 mm`, else 10000 (exact `Fx` compare).
pub fn elevation_bp(attacker_y: Fx, target_y: Fx) -> i32 {
    let dy = i128::from(attacker_y.raw()) - i128::from(target_y.raw());
    let th = i128::from(fx_from_milli(ELEVATION_MM).raw());
    if dy >= th {
        ELEVATION_HIGH_BP
    } else if dy <= -th {
        ELEVATION_LOW_BP
    } else {
        10_000
    }
}

/// SPEC: canon §10.5 step 2.4a "ranged maximum range also grows by `max(0, dy)`".
pub fn ranged_range_bonus(attacker_y: Fx, target_y: Fx) -> Fx {
    let dy = attacker_y - target_y;
    if dy > Fx::ZERO { dy } else { Fx::ZERO }
}

/// SPEC: canon §10.5 steps 2.4a and 2.4b. Positions on the impact tick.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Geometry {
    /// Attacker origin (the owner's, for projectiles and area hits; doc 05 §10.3).
    pub attacker_pos: Fx3,
    /// Target origin.
    pub target_pos: Fx3,
    /// Target yaw.
    pub target_yaw: Bam,
    /// The target's AI target is the attacker (ignored for Errata targets).
    pub target_targeting_attacker: bool,
}

/// SPEC: canon §10.5 step 2.4b. × 11500 when the attacker is outside the
/// target's frontal cone (`ang_diff(target.yaw, bearing) > 10923`, with
/// `bearing = yaw_from_dir(att - tgt)`, doc 05 §10.3) and the target is not
/// targeting the attacker; Errata targets use the cone check only.
pub fn flanking_bp(g: &Geometry, target_is_erratum: bool) -> i32 {
    let dx = g.attacker_pos.x - g.target_pos.x;
    let dz = g.attacker_pos.z - g.target_pos.z;
    let bearing = yaw_from_dir(dx, dz);
    let outside = ang_diff(g.target_yaw, bearing) > FRONTAL_CONE_BAM;
    if outside && (target_is_erratum || !g.target_targeting_attacker) {
        FLANKING_BP
    } else {
        10_000
    }
}

/// SPEC: canon §5.7 MYTHOS damage term, applied once at step 2.7 when the
/// attacker or the target is tagged undead, fiend, divine or eldritch:
/// `10000 + (MYT - 5) × 300` with the local MYTHOS at the attacker (§5.8).
pub const fn mythos_bp(local_mythos: u8, tagged: bool) -> i32 {
    if tagged {
        10_000 + (local_mythos as i32 - 5) * 300
    } else {
        10_000
    }
}

/// SPEC: canon §6.3 requirement shortfall multiplier `10000 - min(5000, s × 100)`
/// with `s = max(0, req - value)`.
pub const fn shortfall_bp(req: i32, value: i32) -> i32 {
    let s = if req > value { req - value } else { 0 };
    let pen = if s >= 50 { 5_000 } else { s * 100 };
    10_000 - pen
}

/// SPEC: canon §17.2 "Enemy damage against Errata" for party sizes 1 to 4.
pub const PARTY_ENEMY_DAMAGE_BP: [i32; 4] = [10_000, 11_000, 12_000, 13_000];

/// SPEC: canon §17.2. Enemy damage multiplier for `party_size` in `1..=4`.
/// Panics outside that range.
pub const fn party_damage_bp(party_size: u8) -> i32 {
    assert!(
        party_size >= 1 && party_size <= 4,
        "party size outside 1..=4"
    );
    PARTY_ENEMY_DAMAGE_BP[party_size as usize - 1]
}

/// SPEC: canon §17.5 friendly-fire class of a hit.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum FfClass {
    /// Weapon, spell and gadget hits: `FF_DIRECT_BP`.
    #[default]
    Direct,
    /// Explosions, fire spread, surfaces, falling blocks and shock chains: `FF_AREA_BP`.
    Area,
}

/// SPEC: canon §17.5 co-op friendly-fire modes.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum FfMode {
    /// Defaults: direct 5000, area 10000.
    #[default]
    Default,
    /// `mode.ff_off`: direct 0, area 5000.
    Off,
    /// `mode.ff_full`: both 10000.
    Full,
}

/// SPEC: canon §17.5 `FF_DIRECT_BP` in the default mode.
pub const FF_DIRECT_BP: i32 = 5_000;
/// SPEC: canon §17.5 `FF_AREA_BP` in the default mode.
pub const FF_AREA_BP: i32 = 10_000;

/// SPEC: canon §17.5. The friendly-fire multiplier for a mode and class.
pub const fn ff_bp(mode: FfMode, class: FfClass) -> i32 {
    match (mode, class) {
        (FfMode::Default, FfClass::Direct) => FF_DIRECT_BP,
        (FfMode::Default, FfClass::Area) => FF_AREA_BP,
        (FfMode::Off, FfClass::Direct) => 0,
        (FfMode::Off, FfClass::Area) => 5_000,
        (FfMode::Full, _) => 10_000,
    }
}

/// SPEC: canon §10.5 step 2.4; doc 09 awareness ladder (ordinals 0 to 5).
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[allow(missing_docs)]
pub enum Awareness {
    #[default]
    Unaware,
    Curious,
    Suspicious,
    Searching,
    Lost,
    Alert,
}

/// SPEC: canon §10.5 step 2.4. The single (attacker, target) pair a sneak
/// check reads, on the impact tick. Other entities' awareness is irrelevant.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct SneakPair {
    /// The target has `st.sleeping`.
    pub target_sleeping: bool,
    /// The target has perception (doc 09; Errata and invaders do not).
    pub target_has_perception: bool,
    /// The target's awareness of this attacker.
    pub target_awareness_of_attacker: Awareness,
}

/// SPEC: canon §10.5 step 2.4 sneak eligibility: the target's awareness of
/// the attacker is below `aware.alert`, or the target has `st.sleeping`
/// (doc 09: targets without perception are eligible only while sleeping).
pub const fn sneak_eligible(p: SneakPair) -> bool {
    p.target_sleeping
        || (p.target_has_perception
            && (p.target_awareness_of_attacker as u8) < Awareness::Alert as u8)
}

/// SPEC: canon §3.3 keyed `rng.combat` generator for one hit's crit roll.
pub const fn combat_rng(run_seed: u64, folio: u32, actor: u64, tick: u64, seq: u32) -> Rng {
    keyed_rng(run_seed, streams::COMBAT, folio, actor, tick, seq)
}

/// SPEC: canon §10.5 step 2.3 and §3.4: each hit draws exactly one crit
/// roll (`roll_bp`), except sneak attacks, which draw none and return `None`.
pub fn roll_crit(rng: &mut Rng, chance_bp: i32, sneak: bool) -> Option<bool> {
    if sneak {
        None
    } else {
        Some(rng.roll_bp(chance_bp))
    }
}

// ---------------------------------------------------------------------------
// Pipeline inputs
// ---------------------------------------------------------------------------

/// SPEC: canon §10.5 step 1. One "+% damage" source and its component potency.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Bonus {
    /// Bonus, Bp.
    pub bonus_bp: i32,
    /// That component's Contradiction potency, Bp.
    pub potency_bp: i32,
}

/// SPEC: canon §10.5 steps 2.3 and 2.4: a hit is either a normal strike
/// (which rolled exactly one crit) or a sneak attack (which never rolls).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Strike {
    /// A normal hit; `crit_mult_bp` is `Some` when the crit roll succeeded.
    Normal {
        /// Crit multiplier (§6.4 row 14) when the roll succeeded.
        crit_mult_bp: Option<i32>,
    },
    /// A sneak attack with doc 09's multiplier.
    Sneak {
        /// Sneak multiplier, Bp.
        mult_bp: i32,
    },
}

/// SPEC: canon §5.7 tags that trigger the MYTHOS term and step 3a.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct CreatureTags {
    /// Tagged undead.
    pub undead: bool,
    /// Tagged fiend.
    pub fiend: bool,
    /// Tagged divine.
    pub divine: bool,
    /// Tagged eldritch.
    pub eldritch: bool,
}

impl CreatureTags {
    /// SPEC: canon §5.7. Undead, fiend, divine or eldritch.
    pub const fn mythic(self) -> bool {
        self.undead || self.fiend || self.divine || self.eldritch
    }

    /// SPEC: canon §10.5 step 3a. Undead or fiend (radiant × 15000).
    pub const fn unholy(self) -> bool {
        self.undead || self.fiend
    }
}

/// SPEC: canon §11.8 `st.charmed` on a player-controlled attacker.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct CharmedPlayer {
    /// The target belongs to the applier's faction (outgoing damage × 0).
    pub target_in_applier_faction: bool,
}

/// SPEC: canon §10.5 step 2.12 attacker statuses (ascending status ID order).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct AttackerStatuses {
    /// `st.berserk`: × 13000.
    pub berserk: bool,
    /// `st.charmed` on a player-controlled bearer (§11.8).
    pub charmed_player: Option<CharmedPlayer>,
    /// `st.empowered`: × 12000.
    pub empowered: bool,
    /// `st.taunted`: the taunter's EntityId; × 7500 when the target is someone else.
    pub taunted_by: Option<u64>,
    /// `st.weakened`: × 7500.
    pub weakened: bool,
}

/// SPEC: canon §10.5 step 2 attacker facts.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct Attacker {
    /// The attacker is an NPC (step 2.14).
    pub is_npc: bool,
    /// The attacker belongs to `fac.errata`, minions included (step 2.15).
    pub errata_faction: bool,
    /// MYTHOS tags (step 2.7).
    pub tags: CreatureTags,
    /// Statuses (step 2.12).
    pub statuses: AttackerStatuses,
}

/// SPEC: canon §11.2 buildup inputs carried by a hit (doc 05 §13 powers).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct HitBuildup {
    /// Status power per damage type, threshold points.
    pub power: [i32; TYPE_COUNT],
    /// `src_lvl` for `S(src_lvl)`; enemies pass `d`.
    pub src_lvl: u32,
    /// Attacker status buildup multiplier (§6.4 row 41), Bp.
    pub attacker_buildup_bp: i32,
}

/// SPEC: canon §10.7, §6.4 row 19. A blocked hit's guard.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct BlockInput {
    /// Block stability (§6.4 row 19, doc 05 §12.2), `0..=10000`.
    pub stability_bp: i32,
    /// Current depth for `T(d_current)`.
    pub d_current: u32,
    /// Blocker's stamina before the hit, milli-stamina.
    pub stamina_mst: i64,
    /// Doc 05 `GUARD_BREAK`: stamina loss × 20000.
    pub guard_break: bool,
}

/// SPEC: canon §10.5. One weapon, spell or gadget hit entering at step 1.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Hit<'a> {
    /// Step 0 magnitude `B`, mHP.
    pub b_mhp: i64,
    /// Type shares.
    pub shares: Shares,
    /// Penetration `0..=10000`.
    pub pen_bp: i32,
    /// Area damage (step 2.10 swarm; no precision per doc 05).
    pub area: bool,
    /// Friendly-fire class (step 2.15).
    pub ff_class: FfClass,
    /// Skips armor (doc 09: sneak attacks at Sneak 100).
    pub ignore_armor: bool,
    /// Doc 05 `counter.ward_breaker`: every ward pool's multiplier × 30000.
    pub ward_breaker: bool,
    /// §6.4 row 12 (weapon attacks by Errata only), Bp.
    pub weapon_scaling_bp: i32,
    /// Step 1 "+% damage" sources.
    pub bonuses: &'a [Bonus],
    /// Step 2.1 spell potency (spells and prayers only).
    pub spell_potency_bp: Option<i32>,
    /// Step 2.2 weapon precision multiplier.
    pub precision_mult_bp: i32,
    /// Steps 2.3 and 2.4.
    pub strike: Strike,
    /// Steps 2.4a and 2.4b.
    pub geometry: Geometry,
    /// Step 2.5 base-function component potency.
    pub base_potency_bp: i32,
    /// Step 2.6 base-function component `edge_bp`.
    pub edge_bp: i32,
    /// Step 2.7 local MYTHOS at the attacker's position.
    pub local_mythos: u8,
    /// Step 2.8 requirement shortfall multiplier ([`shortfall_bp`]).
    pub shortfall_bp: i32,
    /// Step 2.13 difficulty multiplier (doc 14).
    pub difficulty_bp: i32,
    /// Attacker facts.
    pub attacker: Attacker,
    /// Guard outcome: `Some` when the hit was blocked (step 6b).
    pub block: Option<BlockInput>,
    /// Status powers (step 8 buildup).
    pub buildup: Option<HitBuildup>,
}

impl Hit<'_> {
    /// SPEC: canon §10.5. A neutral hit: every multiplier 10000, no crit,
    /// coincident positions (no elevation or flanking), Coherent component.
    pub const fn basic(b_mhp: i64, shares: Shares) -> Hit<'static> {
        Hit {
            b_mhp,
            shares,
            pen_bp: 0,
            area: false,
            ff_class: FfClass::Direct,
            ignore_armor: false,
            ward_breaker: false,
            weapon_scaling_bp: 0,
            bonuses: &[],
            spell_potency_bp: None,
            precision_mult_bp: PRECISION_MELEE_BP,
            strike: Strike::Normal { crit_mult_bp: None },
            geometry: Geometry {
                attacker_pos: Fx3::ZERO,
                target_pos: Fx3::ZERO,
                target_yaw: 0,
                target_targeting_attacker: true,
            },
            base_potency_bp: 10_000,
            edge_bp: 0,
            local_mythos: 5,
            shortfall_bp: 10_000,
            difficulty_bp: 10_000,
            attacker: Attacker {
                is_npc: false,
                errata_faction: false,
                tags: CreatureTags {
                    undead: false,
                    fiend: false,
                    divine: false,
                    eldritch: false,
                },
                statuses: AttackerStatuses {
                    berserk: false,
                    charmed_player: None,
                    empowered: false,
                    taunted_by: None,
                    weakened: false,
                },
            },
            block: None,
            buildup: None,
        }
    }
}

/// SPEC: canon §9. The hit part's multiplier and precision flag.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct HitPart {
    /// `hit_mult`, Bp.
    pub hit_mult_bp: i32,
    /// The part is a precision part.
    pub precision: bool,
}

impl HitPart {
    /// SPEC: canon §9 `bp.torso` (10000, no precision).
    pub const TORSO: HitPart = HitPart {
        hit_mult_bp: 10_000,
        precision: false,
    };
    /// SPEC: canon §9 `bp.head` (10000, precision).
    pub const HEAD: HitPart = HitPart {
        hit_mult_bp: 10_000,
        precision: true,
    };
    /// SPEC: canon §9 humanoid limb (8500, no precision).
    pub const LIMB: HitPart = HitPart {
        hit_mult_bp: 8_500,
        precision: false,
    };
}

/// SPEC: canon §10.5 target statuses the pipeline reads.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct DefenderStatuses {
    /// `st.exposed` stacks (0 when absent), step 2.11.
    pub exposed_stacks: u8,
    /// `st.knocked_down`, step 2.11.
    pub knocked_down: bool,
    /// `st.frozen`, step 3a.
    pub frozen: bool,
    /// `st.deaths_door` (invulnerable): the hit does nothing.
    pub invulnerable: bool,
    /// `st.oiled` (fire buildup × 20000).
    pub oiled: bool,
    /// `st.soaked` (shock buildup × 15000, fire buildup × 5000).
    pub soaked: bool,
}

/// SPEC: canon §10.5 step 7 ward pools. `res.ward` is the `st.shielded`
/// pool plus the ward generator's ward (§12); an NPC `elite.warded` pool
/// uses the generator slot with the neutral profile.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Wards {
    /// `st.shielded` pool, mHP (consumed first, canonical `ward_mult`).
    pub shielded_mhp: i64,
    /// Ward generator pool, mHP.
    pub generator_mhp: i64,
    /// Doc 05 §15.4 generator depletion profile per type, Bp.
    pub generator_profile_bp: [i32; TYPE_COUNT],
}

impl Wards {
    /// SPEC: canon §10.5 step 7. No ward.
    pub const NONE: Wards = Wards {
        shielded_mhp: 0,
        generator_mhp: 0,
        generator_profile_bp: [10_000; TYPE_COUNT],
    };
}

/// SPEC: canon §10.5. The target side of a hit.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Defender {
    /// EntityId (step 2.12 taunt check).
    pub id: u64,
    /// Erratum or NPC.
    pub kind: TargetKind,
    /// Belongs to `fac.errata`, minions included (step 2.15).
    pub errata_faction: bool,
    /// Native to its local genre (§5.6).
    pub native: bool,
    /// MYTHOS and step 3a tags.
    pub tags: CreatureTags,
    /// Body plan `body.swarm` (step 2.10).
    pub swarm: bool,
    /// The hit part.
    pub part: HitPart,
    /// Statuses the pipeline reads.
    pub statuses: DefenderStatuses,
    /// Raw resistance sums per type (step 4 clamps them).
    pub resist_bp: [i32; TYPE_COUNT],
    /// Immune types.
    pub immune: TypeSet,
    /// AR of the hit part after §10.3 status multipliers, plus the shield's
    /// AR when blocked (doc 05 §12.2).
    pub ar_part: i64,
    /// Ward pools.
    pub wards: Wards,
}

impl Defender {
    /// SPEC: canon §10.5. A neutral target: torso hit, no armor, resistance,
    /// immunity, ward or statuses.
    pub const fn new(id: u64, kind: TargetKind) -> Defender {
        Defender {
            id,
            kind,
            errata_faction: matches!(kind, TargetKind::Erratum),
            native: false,
            tags: CreatureTags {
                undead: false,
                fiend: false,
                divine: false,
                eldritch: false,
            },
            swarm: false,
            part: HitPart::TORSO,
            statuses: DefenderStatuses {
                exposed_stacks: 0,
                knocked_down: false,
                frozen: false,
                invulnerable: false,
                oiled: false,
                soaked: false,
            },
            resist_bp: [0; TYPE_COUNT],
            immune: TypeSet::EMPTY,
            ar_part: 0,
            wards: Wards::NONE,
        }
    }

    /// SPEC: canon §10.2. Whether `ty` is immune for this target
    /// (`dmg.true` ignores resistance, so it is never immune).
    pub const fn is_immune(&self, ty: DamageType) -> bool {
        !matches!(ty, DamageType::True) && self.immune.contains(ty)
    }
}

/// SPEC: canon §10.5 steps 2.14, 2.15. World rules for a hit.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Rules {
    /// Party size `1..=4`.
    pub party_size: u8,
    /// Friendly-fire mode.
    pub ff_mode: FfMode,
}

impl Rules {
    /// SPEC: canon §17.2, §17.5. Solo, default friendly fire.
    pub const SOLO: Rules = Rules {
        party_size: 1,
        ff_mode: FfMode::Default,
    };
}

// ---------------------------------------------------------------------------
// Pipeline outputs
// ---------------------------------------------------------------------------

/// SPEC: canon §10.5 step 2: every multiplier the chain applied, in order
/// (10000 where a step did not apply).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ChainTrace {
    /// 2.1 spell potency.
    pub spell_potency: i32,
    /// 2.2 precision.
    pub precision: i32,
    /// 2.3 crit.
    pub crit: i32,
    /// 2.4 sneak.
    pub sneak: i32,
    /// 2.4a elevation.
    pub elevation: i32,
    /// 2.4b flanking.
    pub flanking: i32,
    /// 2.5 Contradiction potency.
    pub potency: i32,
    /// 2.6 Anachronism.
    pub anachronism: i32,
    /// 2.7 MYTHOS.
    pub mythos: i32,
    /// 2.8 requirement shortfall.
    pub shortfall: i32,
    /// 2.9 `hit_mult`.
    pub hit_mult: i32,
    /// 2.10 swarm area.
    pub swarm: i32,
    /// 2.11 `st.exposed`.
    pub exposed: i32,
    /// 2.11 `st.knocked_down`.
    pub knocked_down: i32,
    /// 2.12 `st.berserk`.
    pub berserk: i32,
    /// 2.12 `st.charmed` (player-controlled, §11.8).
    pub charmed: i32,
    /// 2.12 `st.empowered`.
    pub empowered: i32,
    /// 2.12 `st.taunted` (doc 05 §10.2).
    pub taunted: i32,
    /// 2.12 `st.weakened`.
    pub weakened: i32,
    /// 2.13 difficulty.
    pub difficulty: i32,
    /// 2.14 party-size enemy damage.
    pub party: i32,
    /// 2.15 friendly fire.
    pub friendly_fire: i32,
}

impl ChainTrace {
    /// SPEC: canon §10.5 step 2. The multipliers in canonical order.
    pub const fn in_order(&self) -> [i32; 22] {
        [
            self.spell_potency,
            self.precision,
            self.crit,
            self.sneak,
            self.elevation,
            self.flanking,
            self.potency,
            self.anachronism,
            self.mythos,
            self.shortfall,
            self.hit_mult,
            self.swarm,
            self.exposed,
            self.knocked_down,
            self.berserk,
            self.charmed,
            self.empowered,
            self.taunted,
            self.weakened,
            self.difficulty,
            self.party,
            self.friendly_fire,
        ]
    }

    /// SPEC: canon §10.5 step 2, §1.3: `D2` from `D1`, one `mul_bp` per step,
    /// left to right, never reordered.
    pub fn apply(&self, d1: i64) -> i64 {
        let mut x = d1;
        for m in self.in_order() {
            x = mul_bp(x, m);
        }
        x
    }

    /// SPEC: canon §10.5. A trace with every step 10000 (entries after step 2).
    pub const NEUTRAL: ChainTrace = ChainTrace {
        spell_potency: 10_000,
        precision: 10_000,
        crit: 10_000,
        sneak: 10_000,
        elevation: 10_000,
        flanking: 10_000,
        potency: 10_000,
        anachronism: 10_000,
        mythos: 10_000,
        shortfall: 10_000,
        hit_mult: 10_000,
        swarm: 10_000,
        exposed: 10_000,
        knocked_down: 10_000,
        berserk: 10_000,
        charmed: 10_000,
        empowered: 10_000,
        taunted: 10_000,
        weakened: 10_000,
        difficulty: 10_000,
        party: 10_000,
        friendly_fire: 10_000,
    };
}

/// SPEC: canon §10.6. Where a damage instance entered the pipeline.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Entry {
    /// A full §10.5 hit (step 1).
    Hit,
    /// A DoT pulse (step 4), flag `dot`.
    Dot,
    /// A `st.shocked` chained hit (step 3), flag `no_chain`.
    ShockChain,
    /// Fall damage (step 4, blunt).
    Fall,
    /// An environmental hazard not from an item (step 3).
    Environmental,
    /// Reflected damage (thorns, step 3).
    Reflected,
}

/// SPEC: canon §10.5 step 9 `ev.damage_dealt` flags.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct DamageFlags {
    /// The crit multiplier applied.
    pub crit: bool,
    /// A sneak attack.
    pub sneak: bool,
    /// The precision multiplier applied.
    pub precision: bool,
    /// The hit was blocked (step 6b).
    pub blocked: bool,
    /// Area damage.
    pub area: bool,
    /// A DoT pulse.
    pub dot: bool,
    /// A shock-chain hit (never chains further).
    pub no_chain: bool,
    /// The target was invulnerable (`st.deaths_door`); nothing applied.
    pub nullified: bool,
}

/// SPEC: canon §10.7. What a block did.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct BlockOutcome {
    /// Damage turned into stamina loss, mHP.
    pub to_stamina_mhp: i64,
    /// Stamina lost, milli-stamina (normalized by `T(d_current)`).
    pub stamina_loss_mst: i64,
    /// Stamina after the hit, at least 0.
    pub stamina_after_mst: i64,
    /// Stamina reached 0: guard-broken, gains `st.staggered`.
    pub guard_broken: bool,
}

/// SPEC: canon §10.5 step 7. What the ward pools did.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct WardOutcome {
    /// `st.shielded` pool before.
    pub shielded_before: i64,
    /// `st.shielded` pool after.
    pub shielded_after: i64,
    /// Generator pool before.
    pub generator_before: i64,
    /// Generator pool after.
    pub generator_after: i64,
    /// The shielded pool reached 0 (`ev.ward_broken`).
    pub shielded_broken: bool,
    /// The generator pool reached 0 (`ev.ward_broken`).
    pub generator_broken: bool,
}

impl WardOutcome {
    /// SPEC: canon §10.5 step 7. Total ward removed by this hit, mHP.
    pub const fn lost(&self) -> i64 {
        (self.shielded_before - self.shielded_after)
            + (self.generator_before - self.generator_after)
    }

    /// SPEC: canon §10.5 step 7, §18 `ev.ward_broken`. Any pool broke.
    pub const fn broken(&self) -> bool {
        self.shielded_broken || self.generator_broken
    }
}

/// SPEC: canon §10.5 step 9. The full per-type breakdown of one damage
/// instance. `per_type` sums to `total` (HP damage).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct DamageBreakdown {
    /// Entry point.
    pub entry: Entry,
    /// Step 1 additive bucket after the clamp (0 for later entries).
    pub add_bp: i32,
    /// `D1` (equals `D2` for later entries).
    pub d1: i64,
    /// Step 2 multipliers.
    pub chain: ChainTrace,
    /// `D2`, the amount entering step 3 (or step 4).
    pub d2: i64,
    /// After step 3 (split).
    pub split: [i64; TYPE_COUNT],
    /// After step 3a.
    pub after_type_mods: [i64; TYPE_COUNT],
    /// After step 4.
    pub after_resist: [i64; TYPE_COUNT],
    /// Step 5 `DR_bp` per type (0 where armor did not apply).
    pub dr_bp: [i32; TYPE_COUNT],
    /// After step 5.
    pub after_armor: [i64; TYPE_COUNT],
    /// Step 6 chip surplus added.
    pub chip_added: i64,
    /// After step 6.
    pub after_chip: [i64; TYPE_COUNT],
    /// Step 6b block, if blocked.
    pub block: Option<BlockOutcome>,
    /// Step 7 wards.
    pub ward: WardOutcome,
    /// HP damage per type after step 7.
    pub per_type: [i64; TYPE_COUNT],
    /// HP damage, `Σ per_type`.
    pub total: i64,
    /// Step 3a consumed `st.frozen` (remove it).
    pub frozen_consumed: bool,
    /// `dmg.frost` hit: apply `st.chilled` directly (§10.1).
    pub applies_chilled: bool,
    /// `dmg.sonic` hit: `ev.noise` level in dB (§10.1).
    pub sonic_noise_db: Option<i64>,
    /// `counter.ward_breaker` broke a pool: apply `st.stunned` directly (doc 05 §15.1).
    pub breaker_stun: bool,
    /// Step 8 buildup points `p'` per damage type (for the status it builds).
    pub buildup: [i32; TYPE_COUNT],
    /// Step 9 flags.
    pub flags: DamageFlags,
}

impl DamageBreakdown {
    /// SPEC: canon §10.6, §11.5. HP plus ward this instance removed (the
    /// base of a shock chain's `D2`).
    pub const fn hp_and_ward_lost(&self) -> i64 {
        self.total + self.ward.lost()
    }

    const fn empty(entry: Entry, wards: &Wards) -> DamageBreakdown {
        DamageBreakdown {
            entry,
            add_bp: 0,
            d1: 0,
            chain: ChainTrace::NEUTRAL,
            d2: 0,
            split: [0; TYPE_COUNT],
            after_type_mods: [0; TYPE_COUNT],
            after_resist: [0; TYPE_COUNT],
            dr_bp: [0; TYPE_COUNT],
            after_armor: [0; TYPE_COUNT],
            chip_added: 0,
            after_chip: [0; TYPE_COUNT],
            block: None,
            ward: WardOutcome {
                shielded_before: wards.shielded_mhp,
                shielded_after: wards.shielded_mhp,
                generator_before: wards.generator_mhp,
                generator_after: wards.generator_mhp,
                shielded_broken: false,
                generator_broken: false,
            },
            per_type: [0; TYPE_COUNT],
            total: 0,
            frozen_consumed: false,
            applies_chilled: false,
            sonic_noise_db: None,
            breaker_stun: false,
            buildup: [0; TYPE_COUNT],
            flags: DamageFlags {
                crit: false,
                sneak: false,
                precision: false,
                blocked: false,
                area: false,
                dot: false,
                no_chain: false,
                nullified: true,
            },
        }
    }
}

// ---------------------------------------------------------------------------
// Steps 3 to 7 as standalone functions
// ---------------------------------------------------------------------------

fn sum(d: &[i64; TYPE_COUNT]) -> i64 {
    d.iter().sum()
}

/// SPEC: canon §10.5 step 3. `d[t] = mul_bp(D2, share[t])`; the flooring
/// remainder `D2 - Σ d[t]` goes to the type with the largest share, ties to
/// the earlier §10.1 row.
pub fn split_by_type(d2: i64, shares: &Shares) -> [i64; TYPE_COUNT] {
    let mut d = [0; TYPE_COUNT];
    for ty in DamageType::ALL {
        d[ty.index()] = mul_bp(d2, shares.get(ty));
    }
    let rem = d2 - sum(&d);
    d[shares.largest().index()] += rem;
    d
}

/// SPEC: canon §10.5 step 3a. `d[radiant] × 15000` if the target is undead
/// or fiend; `d[blunt] × 15000` if the target has `st.frozen`, which is then
/// removed. Returns true when `st.frozen` is consumed.
///
/// GAP: frozen is consumed by a hit that carries a blunt share, or a sonic
/// share (§10.1 "ends `st.frozen` like blunt"); sonic gets no × 15000.
pub fn per_type_target_mods(
    d: &mut [i64; TYPE_COUNT],
    shares: &Shares,
    tags: CreatureTags,
    frozen: bool,
) -> bool {
    if tags.unholy() {
        let i = DamageType::Radiant.index();
        d[i] = mul_bp(d[i], RADIANT_UNHOLY_BP);
    }
    if !frozen {
        return false;
    }
    let i = DamageType::Blunt.index();
    d[i] = mul_bp(d[i], status::FROZEN_BLUNT_BP);
    shares.get(DamageType::Blunt) > 0 || shares.get(DamageType::Sonic) > 0
}

/// SPEC: canon §10.5 step 4. For each type: `r = clamp(resist[t], -10000,
/// cap)`; immune types become 0; otherwise `d[t] -= mul_bp(d[t], r)`.
/// `dmg.true` skips this step.
pub fn apply_resistance(d: &mut [i64; TYPE_COUNT], def: &Defender) {
    for ty in DamageType::ALL {
        if ty == DamageType::True {
            continue;
        }
        let i = ty.index();
        if def.is_immune(ty) {
            d[i] = 0;
            continue;
        }
        let r = clamp_resist_bp(def.kind, ty, def.resist_bp[i]);
        d[i] -= mul_bp(d[i], r);
    }
}

/// SPEC: canon §10.5 step 5. `effAR = mul_bp(floor_div(AR_part × eff[t],
/// 10000), 10000 - pen_bp)` (computed in `i128`, narrowed to `i64`).
pub fn effective_armor(ar_part: i64, ty: DamageType, pen_bp: i32) -> i64 {
    assert!((0..=10_000).contains(&pen_bp), "pen_bp outside 0..=10000");
    let eff = i128::from(ty.def().armor_eff_bp);
    let scaled = narrow_i64(floor_div_i128(i128::from(ar_part) * eff, 10_000));
    mul_bp(scaled, 10_000 - pen_bp)
}

/// SPEC: canon §10.5 step 5. `DR_bp = min(8500, (effAR × 10^7) /
/// (effAR × 1000 + 4 × d))` in `i128`; 0 when `effAR <= 0`.
pub fn armor_dr_bp(eff_ar: i64, d: i64) -> i32 {
    if eff_ar <= 0 {
        return 0;
    }
    let num = i128::from(eff_ar) * 10_000_000;
    let den = i128::from(eff_ar) * 1_000 + 4 * i128::from(d);
    let dr = floor_div_i128(num, den);
    if dr > 8_500 { 8_500 } else { dr as i32 }
}

/// SPEC: canon §10.5 step 5. Armor for types with armor efficiency above 0;
/// writes each type's `DR_bp` into `dr_out`.
pub fn apply_armor(
    d: &mut [i64; TYPE_COUNT],
    ar_part: i64,
    pen_bp: i32,
    dr_out: &mut [i32; TYPE_COUNT],
) {
    for ty in DamageType::ALL {
        if ty.def().armor_eff_bp <= 0 {
            continue;
        }
        let i = ty.index();
        let e = effective_armor(ar_part, ty, pen_bp);
        if e > 0 {
            let dr = armor_dr_bp(e, d[i]);
            d[i] -= mul_bp(d[i], dr);
            dr_out[i] = dr;
        }
    }
}

/// SPEC: canon §10.5 step 6 chip floor. If at least one type of the hit is
/// not immune, `floor = floor_div(D2, 10)` for Errata and `floor_div(D2,
/// 20)` for NPCs; if `total < floor` the surplus goes to the non-immune type
/// with the largest share, ties to the earlier row. Returns the surplus.
///
/// GAP: "types" means the types this hit carries (share above 0).
pub fn chip_floor(d: &mut [i64; TYPE_COUNT], d2: i64, shares: &Shares, def: &Defender) -> i64 {
    let Some(target) = shares.largest_where(|ty| shares.get(ty) > 0 && !def.is_immune(ty)) else {
        return 0;
    };
    let floor = match def.kind {
        TargetKind::Erratum => floor_div(d2, 10),
        TargetKind::Npc => floor_div(d2, 20),
    };
    let total = sum(d);
    if total < floor {
        let surplus = floor - total;
        d[target.index()] += surplus;
        surplus
    } else {
        0
    }
}

/// SPEC: canon §10.7 `stamina_loss_mhp = floor_div(to_stamina × 10000, T(d_current))`.
pub fn block_stamina_loss_mst(to_stamina_mhp: i64, d_current: u32) -> i64 {
    narrow_i64(floor_div_i128(
        i128::from(to_stamina_mhp) * 10_000,
        i128::from(power_t(d_current)),
    ))
}

/// SPEC: canon §10.7 blocking (step 6b): `to_stamina = mul_bp(total,
/// stability)`, `total -= to_stamina`, stamina loss normalized by
/// `T(d_current)`; stamina reaching 0 guard-breaks. The remaining HP damage
/// is spread per doc 05 §12.3: `d[t] = floor_div(d[t] × new_total, total)`,
/// remainder to the largest-share type (GAP: among types that carried
/// damage). A `guard_break` hit's stamina loss is × 20000 (doc 05).
pub fn apply_block(d: &mut [i64; TYPE_COUNT], shares: &Shares, b: &BlockInput) -> BlockOutcome {
    assert!(
        (0..=10_000).contains(&b.stability_bp),
        "stability_bp outside 0..=10000"
    );
    let total = sum(d);
    let to_stamina = mul_bp(total, b.stability_bp);
    let new_total = total - to_stamina;
    if total > 0 {
        let pre = *d;
        for ty in DamageType::ALL {
            let i = ty.index();
            d[i] = narrow_i64(floor_div_i128(
                i128::from(pre[i]) * i128::from(new_total),
                i128::from(total),
            ));
        }
        let rem = new_total - sum(d);
        if let Some(t) = shares.largest_where(|ty| pre[ty.index()] > 0) {
            d[t.index()] += rem;
        }
    }
    let mut loss = block_stamina_loss_mst(to_stamina, b.d_current);
    if b.guard_break {
        loss = mul_bp(loss, 20_000);
    }
    let after = b.stamina_mst - loss;
    BlockOutcome {
        to_stamina_mhp: to_stamina,
        stamina_loss_mst: loss,
        stamina_after_mst: if after > 0 { after } else { 0 },
        guard_broken: after <= 0,
    }
}

/// SPEC: canon §10.5 step 7, doc 05 §15.1 and §15.4. The effective ward
/// multiplier of `ty` for one pool: `mul_bp(ward_mult, profile)`, then
/// × 30000 under `counter.ward_breaker`.
pub fn ward_mult_eff(ty: DamageType, profile_bp: i32, ward_breaker: bool) -> i32 {
    let m = mul_bp(i64::from(ty.def().ward_mult_bp), profile_bp);
    let m = if ward_breaker { mul_bp(m, 30_000) } else { m };
    i32::try_from(m).expect("ward multiplier overflows i32")
}

/// SPEC: canon §10.5 step 7 for one pool. For each type in §10.1 order with
/// `d[t] > 0` while ward remains: `w = mul_bp(d[t], wm[t])`; if `w <= ward`
/// then `ward -= w` and `d[t] = 0`, otherwise `d[t] -= floor_div(ward ×
/// 10000, wm[t])` and `ward = 0`. Returns the remaining ward.
pub fn absorb_pool(d: &mut [i64; TYPE_COUNT], ward: i64, wm: &[i32; TYPE_COUNT]) -> i64 {
    let mut ward = ward;
    for ty in DamageType::ALL {
        if ward <= 0 {
            break;
        }
        let i = ty.index();
        if d[i] <= 0 {
            continue;
        }
        let w = mul_bp(d[i], wm[i]);
        if w <= ward {
            ward -= w;
            d[i] = 0;
        } else {
            d[i] -= narrow_i64(floor_div_i128(i128::from(ward) * 10_000, i128::from(wm[i])));
            ward = 0;
        }
    }
    ward
}

/// SPEC: canon §10.5 step 7. The `st.shielded` pool first (canonical
/// `ward_mult`), then the generator pool (`ward_mult × profile`); the
/// breaker multiplies both. GAP: `ev.ward_broken` is reported whenever a
/// pool that held ward reaches 0 (§18), including an exact `w == ward`.
pub fn absorb_wards(d: &mut [i64; TYPE_COUNT], wards: &Wards, ward_breaker: bool) -> WardOutcome {
    let mut wm_sh = [0; TYPE_COUNT];
    let mut wm_gen = [0; TYPE_COUNT];
    for ty in DamageType::ALL {
        let i = ty.index();
        wm_sh[i] = ward_mult_eff(ty, 10_000, ward_breaker);
        wm_gen[i] = ward_mult_eff(ty, wards.generator_profile_bp[i], ward_breaker);
    }
    let sh_after = absorb_pool(d, wards.shielded_mhp, &wm_sh);
    let gen_after = absorb_pool(d, wards.generator_mhp, &wm_gen);
    WardOutcome {
        shielded_before: wards.shielded_mhp,
        shielded_after: sh_after,
        generator_before: wards.generator_mhp,
        generator_after: gen_after,
        shielded_broken: wards.shielded_mhp > 0 && sh_after == 0,
        generator_broken: wards.generator_mhp > 0 && gen_after == 0,
    }
}

// ---------------------------------------------------------------------------
// The pipeline (canon §10.5) and its entry points (§10.6)
// ---------------------------------------------------------------------------

/// Which steps after the split run, and with what.
struct Post<'a> {
    entry: Entry,
    shares: &'a Shares,
    pen_bp: i32,
    type_mods: bool,
    armor: bool,
    chip: bool,
    on_hit: bool,
    block: Option<BlockInput>,
    ward_breaker: bool,
    buildup: Option<(HitBuildup, i32, i32)>,
}

fn finish(d2: i64, split: [i64; TYPE_COUNT], def: &Defender, p: &Post<'_>) -> DamageBreakdown {
    let mut d = split;
    let frozen_consumed =
        p.type_mods && per_type_target_mods(&mut d, p.shares, def.tags, def.statuses.frozen);
    let after_type_mods = d;
    apply_resistance(&mut d, def);
    let after_resist = d;
    let mut dr_bp = [0; TYPE_COUNT];
    if p.armor {
        apply_armor(&mut d, def.ar_part, p.pen_bp, &mut dr_bp);
    }
    let after_armor = d;
    let chip_added = if p.chip {
        chip_floor(&mut d, d2, p.shares, def)
    } else {
        0
    };
    let after_chip = d;
    let block = p.block.map(|b| apply_block(&mut d, p.shares, &b));
    let ward = absorb_wards(&mut d, &def.wards, p.ward_breaker);
    let total = sum(&d);

    let mut buildup = [0; TYPE_COUNT];
    if let Some((hb, potency_bp, edge_bp)) = p.buildup {
        for ty in DamageType::ALL {
            let i = ty.index();
            let (Some(st), true) = (ty.def().builds, hb.power[i] > 0) else {
                continue;
            };
            let h = BuildupHit {
                power: hb.power[i],
                src_lvl: hb.src_lvl,
                potency_bp,
                edge_bp,
                target_native: def.native,
                attacker_buildup_bp: hb.attacker_buildup_bp,
                resist_bp: clamp_resist_bp(def.kind, ty, def.resist_bp[i]),
                blocked: p.block.is_some(),
                immune: def.is_immune(ty),
                target_oiled: def.statuses.oiled,
                target_soaked: def.statuses.soaked,
            };
            buildup[i] = buildup_points(st, &h);
        }
    }

    let carries = |ty: DamageType| p.shares.get(ty) > 0;
    let applies_chilled =
        p.on_hit && carries(DamageType::Frost) && !def.is_immune(DamageType::Frost);
    let sonic_noise_db = if p.on_hit && carries(DamageType::Sonic) {
        Some(sonic_noise_db(d[DamageType::Sonic.index()]))
    } else {
        None
    };
    DamageBreakdown {
        entry: p.entry,
        add_bp: 0,
        d1: d2,
        chain: ChainTrace::NEUTRAL,
        d2,
        split,
        after_type_mods,
        after_resist,
        dr_bp,
        after_armor,
        chip_added,
        after_chip,
        block,
        ward,
        per_type: d,
        total,
        frozen_consumed,
        applies_chilled,
        sonic_noise_db,
        breaker_stun: p.ward_breaker && ward.broken(),
        buildup,
        flags: DamageFlags {
            crit: false,
            sneak: false,
            precision: false,
            blocked: block.is_some(),
            area: false,
            dot: matches!(p.entry, Entry::Dot),
            no_chain: matches!(p.entry, Entry::ShockChain),
            nullified: false,
        },
    }
}

/// SPEC: canon §10.5 step 2. Build the multiplier trace for a hit, in the
/// exact canonical order 2.1 to 2.15 (doc 05 §10.2 adds `st.taunted` at 2.12;
/// §11.8 places a charmed player's × 0 at 2.12 and its × 10000 at 2.15).
pub fn chain_trace(hit: &Hit<'_>, def: &Defender, rules: &Rules) -> ChainTrace {
    let a = &hit.attacker;
    let s = &a.statuses;
    let (crit, sneak) = match hit.strike {
        Strike::Normal { crit_mult_bp } => (crit_mult_bp.unwrap_or(10_000), 10_000),
        Strike::Sneak { mult_bp } => (10_000, mult_bp),
    };
    let both_errata = a.errata_faction && def.errata_faction;
    ChainTrace {
        spell_potency: hit.spell_potency_bp.unwrap_or(10_000),
        precision: if def.part.precision && !hit.area {
            hit.precision_mult_bp
        } else {
            10_000
        },
        crit,
        sneak,
        elevation: elevation_bp(hit.geometry.attacker_pos.y, hit.geometry.target_pos.y),
        flanking: flanking_bp(&hit.geometry, def.kind == TargetKind::Erratum),
        potency: hit.base_potency_bp,
        anachronism: if def.native {
            10_000 + hit.edge_bp
        } else {
            10_000
        },
        mythos: mythos_bp(hit.local_mythos, a.tags.mythic() || def.tags.mythic()),
        shortfall: hit.shortfall_bp,
        hit_mult: def.part.hit_mult_bp,
        swarm: if hit.area && def.swarm {
            SWARM_AREA_BP
        } else {
            10_000
        },
        exposed: status::exposed_damage_taken_bp(def.statuses.exposed_stacks),
        knocked_down: if def.statuses.knocked_down {
            status::KNOCKED_DOWN_DAMAGE_TAKEN_BP
        } else {
            10_000
        },
        berserk: if s.berserk {
            status::BERSERK_DAMAGE_BP
        } else {
            10_000
        },
        charmed: match s.charmed_player {
            Some(CharmedPlayer {
                target_in_applier_faction: true,
            }) => 0,
            _ => 10_000,
        },
        empowered: if s.empowered {
            status::EMPOWERED_DAMAGE_BP
        } else {
            10_000
        },
        taunted: match s.taunted_by {
            Some(taunter) if taunter != def.id => status::TAUNTED_DAMAGE_BP,
            _ => 10_000,
        },
        weakened: if s.weakened {
            status::WEAKENED_DAMAGE_BP
        } else {
            10_000
        },
        difficulty: hit.difficulty_bp,
        party: if a.is_npc && def.kind == TargetKind::Erratum {
            party_damage_bp(rules.party_size)
        } else {
            10_000
        },
        friendly_fire: if !both_errata || s.charmed_player.is_some() {
            10_000
        } else {
            ff_bp(rules.ff_mode, hit.ff_class)
        },
    }
}

/// SPEC: canon §10.5 steps 1 to 7 (and the step 8 buildup column), in the
/// canonical order: additive bucket, the 2.1 to 2.15 chain, split with the
/// remainder rule, 3a, resistance, armor (unless `ignore_armor`), chip
/// floor, block (§10.7), wards. An invulnerable target (`st.deaths_door`)
/// takes nothing.
pub fn resolve_hit(hit: &Hit<'_>, def: &Defender, rules: &Rules) -> DamageBreakdown {
    assert!(hit.b_mhp >= 0, "resolve_hit: negative B");
    if def.statuses.invulnerable {
        return DamageBreakdown::empty(Entry::Hit, &def.wards);
    }
    let add = additive_bp(hit.weapon_scaling_bp, hit.bonuses);
    let d1 = mul_bp(hit.b_mhp, 10_000 + add);
    let chain = chain_trace(hit, def, rules);
    for m in chain.in_order() {
        assert!(m >= 0, "resolve_hit: negative multiplier");
    }
    let d2 = chain.apply(d1);
    let post = Post {
        entry: Entry::Hit,
        shares: &hit.shares,
        pen_bp: hit.pen_bp,
        type_mods: true,
        armor: !hit.ignore_armor,
        chip: true,
        on_hit: true,
        block: hit.block,
        ward_breaker: hit.ward_breaker,
        buildup: hit.buildup.map(|b| (b, hit.base_potency_bp, hit.edge_bp)),
    };
    let mut out = finish(d2, split_by_type(d2, &hit.shares), def, &post);
    out.add_bp = add;
    out.d1 = d1;
    out.chain = chain;
    out.flags.crit = matches!(
        hit.strike,
        Strike::Normal {
            crit_mult_bp: Some(_)
        }
    );
    out.flags.sneak = matches!(hit.strike, Strike::Sneak { .. });
    out.flags.precision = def.part.precision && !hit.area;
    out.flags.area = hit.area;
    out
}

fn single(ty: DamageType, amount: i64) -> [i64; TYPE_COUNT] {
    let mut d = [0; TYPE_COUNT];
    d[ty.index()] = amount;
    d
}

/// SPEC: canon §10.6 DoT pulse: enters at step 4 (`dmg.true` skips 4);
/// skips armor, chip floor, crit, buildup and `st.exposed`; wards still
/// absorb; flag `dot`.
pub fn resolve_dot(q_mhp: i64, ty: DamageType, def: &Defender) -> DamageBreakdown {
    assert!(q_mhp >= 0, "resolve_dot: negative amount");
    if def.statuses.invulnerable {
        return DamageBreakdown::empty(Entry::Dot, &def.wards);
    }
    let shares = Shares::single(ty);
    let post = Post {
        entry: Entry::Dot,
        shares: &shares,
        pen_bp: 0,
        type_mods: false,
        armor: false,
        chip: false,
        on_hit: false,
        block: None,
        ward_breaker: false,
        buildup: None,
    };
    finish(q_mhp, single(ty, q_mhp), def, &post)
}

/// SPEC: canon §10.6 and §4.2 fall damage: a `dmg.blunt` hit entering at
/// step 4; skips armor, chip floor and crit.
pub fn resolve_fall(dmg_mhp: i64, def: &Defender) -> DamageBreakdown {
    assert!(dmg_mhp >= 0, "resolve_fall: negative amount");
    if def.statuses.invulnerable {
        return DamageBreakdown::empty(Entry::Fall, &def.wards);
    }
    let shares = Shares::single(DamageType::Blunt);
    let post = Post {
        entry: Entry::Fall,
        shares: &shares,
        pen_bp: 0,
        type_mods: false,
        armor: false,
        chip: false,
        on_hit: false,
        block: None,
        ward_breaker: false,
        buildup: None,
    };
    finish(dmg_mhp, single(DamageType::Blunt, dmg_mhp), def, &post)
}

/// SPEC: canon §10.6 and §11.5 `st.shocked` chain: enters at step 3 with
/// `D2 = mul_bp(HP and ward lost to the triggering hit, 2000)`, 100%
/// `dmg.shock`; skips crit, chip floor, buildup, on-hit effects and further
/// chaining; flag `no_chain`.
///
/// GAP: §17.5 lists shock chains under `FF_AREA_BP`, but the chain enters
/// after step 2.15; the area friendly-fire factor is applied to this `D2`
/// when the instigator and the target are both `fac.errata`.
pub fn resolve_shock_chain(
    lost_mhp: i64,
    instigator_errata: bool,
    def: &Defender,
    rules: &Rules,
) -> DamageBreakdown {
    assert!(lost_mhp >= 0, "resolve_shock_chain: negative amount");
    if def.statuses.invulnerable {
        return DamageBreakdown::empty(Entry::ShockChain, &def.wards);
    }
    let mut d2 = status::shock_chain_d2(lost_mhp);
    if instigator_errata && def.errata_faction {
        d2 = mul_bp(d2, ff_bp(rules.ff_mode, FfClass::Area));
    }
    let shares = Shares::single(DamageType::Shock);
    let post = Post {
        entry: Entry::ShockChain,
        shares: &shares,
        pen_bp: 0,
        type_mods: true,
        armor: true,
        chip: false,
        on_hit: false,
        block: None,
        ward_breaker: false,
        buildup: None,
    };
    finish(d2, split_by_type(d2, &shares), def, &post)
}

/// SPEC: canon §10.6. An environmental hazard not from an item (doc 12,
/// §12.1 blasts).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Hazard {
    /// Hazard base, mHP.
    pub base_mhp: i64,
    /// Type shares.
    pub shares: Shares,
    /// Current depth `d` for `P(d)` / `T(d)`.
    pub d: u32,
    /// The hazard's instigator is `fac.errata` (doc 03 `owner_slot`), so
    /// friendly fire applies `FF_AREA_BP` against Errata targets.
    pub instigator_errata: bool,
    /// Blocked by a guard that blocks area hits.
    pub block: Option<BlockInput>,
    /// Environmental buildup powers (`src_lvl` = `d`, row 41 = 10000).
    pub buildup: Option<HitBuildup>,
}

/// SPEC: canon §10.6 environmental hazard: enters at step 3 with
/// `D2 = base × P(d)` against NPCs or `base × T(d)` against Errata; skips
/// crit. GAP: an Errata instigator applies `FF_AREA_BP` to `D2` (§17.5 "fire
/// spread, surfaces"; doc 03 `owner_slot`).
pub fn resolve_environmental(h: &Hazard, def: &Defender, rules: &Rules) -> DamageBreakdown {
    assert!(h.base_mhp >= 0, "resolve_environmental: negative base");
    if def.statuses.invulnerable {
        return DamageBreakdown::empty(Entry::Environmental, &def.wards);
    }
    let table = match def.kind {
        TargetKind::Npc => power_p(h.d),
        TargetKind::Erratum => power_t(h.d),
    };
    let mut d2 = mul_bp_wide(h.base_mhp, table);
    if h.instigator_errata && def.errata_faction {
        d2 = mul_bp(d2, ff_bp(rules.ff_mode, FfClass::Area));
    }
    let post = Post {
        entry: Entry::Environmental,
        shares: &h.shares,
        pen_bp: 0,
        type_mods: true,
        armor: true,
        chip: true,
        on_hit: true,
        block: h.block,
        ward_breaker: false,
        buildup: h.buildup.map(|b| (b, 10_000, 0)),
    };
    let mut out = finish(d2, split_by_type(d2, &h.shares), def, &post);
    out.flags.area = true;
    out
}

/// SPEC: canon §10.6 reflected damage (thorns): enters at step 3 with the
/// reflected amount as `D2`; skips crit, buildup and further reflection.
pub fn resolve_reflected(d2_mhp: i64, shares: &Shares, def: &Defender) -> DamageBreakdown {
    assert!(d2_mhp >= 0, "resolve_reflected: negative amount");
    if def.statuses.invulnerable {
        return DamageBreakdown::empty(Entry::Reflected, &def.wards);
    }
    let post = Post {
        entry: Entry::Reflected,
        shares,
        pen_bp: 0,
        type_mods: true,
        armor: true,
        chip: true,
        on_hit: true,
        block: None,
        ward_breaker: false,
        buildup: None,
    };
    finish(d2_mhp, split_by_type(d2_mhp, shares), def, &post)
}

// ---------------------------------------------------------------------------
// Step 8 helpers and lethal resolution (canon §10.5 step 8, §10.8)
// ---------------------------------------------------------------------------

/// SPEC: canon §10.8 "Hits in the same tick resolve in ascending (source
/// EntityId, seq)". Sort hits by this key.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HitKey {
    /// Source EntityId.
    pub source: u64,
    /// Per-actor action sequence.
    pub seq: u32,
}

/// SPEC: canon §10.5 step 8. HP after subtracting `total`, and whether the
/// lethal resolution of §10.8 must run (HP would reach 0 or below).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct HpOutcome {
    /// HP after the hit, floored at 0 (lethal resolution then decides).
    pub hp_mhp: i64,
    /// HP would reach 0 or below.
    pub lethal: bool,
}

/// SPEC: canon §10.5 step 8. Subtract `total` from HP.
pub const fn apply_hp(hp_mhp: i64, total: i64) -> HpOutcome {
    let after = hp_mhp - total;
    HpOutcome {
        hp_mhp: if after > 0 { after } else { 0 },
        lethal: after <= 0,
    }
}

/// SPEC: canon §10.5 step 8, §9. Subtract `total` from the hit part's
/// condition pool, floored at 0 (pools range `0..=max`, §12).
pub const fn apply_part_pool(pool_mhp: i64, total: i64) -> i64 {
    let after = pool_mhp - total;
    if after > 0 { after } else { 0 }
}

/// SPEC: canon §10.5 step 8 and doc 05 §12.4 base poise damage:
/// `pd = mul_bp(impact_mp, S(src_lvl))`, then × `(10000 + mul_bp(5000,
/// share_blunt))` (blunt × 15000 pro rata), 0 if blocked or parried. Counter,
/// mount, hyperarmor and affix factors are applied by the caller (doc 05).
pub fn poise_damage_base_mp(
    impact_mp: i64,
    src_lvl: u32,
    blunt_share_bp: i32,
    blocked: bool,
) -> i64 {
    if blocked {
        return 0;
    }
    let pd = mul_bp_wide(impact_mp, power_s(src_lvl));
    let blunt = mul_bp(i64::from(BLUNT_POISE_BP - 10_000), blunt_share_bp);
    mul_bp(pd, 10_000 + i32::try_from(blunt).expect("blunt factor"))
}

/// SPEC: canon §10.8. Facts the lethal decision reads.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct LethalInput {
    /// The dying entity is an Erratum.
    pub is_erratum: bool,
    /// Death's Door already used this Folio.
    pub deaths_door_used: bool,
    /// `st.unmoored` (§5.11): Fortune-derived effects such as Death's Door are off.
    pub unmoored: bool,
    /// At least one other party member is alive and not downed.
    pub other_member_up: bool,
    /// At least one other Erratum lives (downed counts as alive).
    pub other_erratum_alive: bool,
}

/// SPEC: canon §10.8. The three outcomes.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LethalOutcome {
    /// Step 1: HP = 1000 mHP and `st.deaths_door` for 60 ticks.
    DeathsDoor,
    /// Step 2: `st.downed` with HP 0, `ev.player_downed`.
    Downed,
    /// Step 3: `ev.entity_killed`.
    Killed,
}

/// SPEC: canon §10.8, §17.6, §18. Events the decision emits.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct LethalEvents {
    /// `ev.entity_killed`.
    pub entity_killed: bool,
    /// `ev.player_downed`.
    pub player_downed: bool,
    /// `ev.player_died`.
    pub player_died: bool,
    /// `ev.player_ghosted` (the Erratum gains `st.ghost`).
    pub player_ghosted: bool,
}

/// SPEC: canon §10.8. The decision.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct LethalDecision {
    /// Outcome.
    pub outcome: LethalOutcome,
    /// HP after resolution, mHP.
    pub hp_mhp: i64,
    /// Whether the Death's Door roll was drawn.
    pub deaths_door_rolled: bool,
    /// Death's Door used flag after resolution.
    pub deaths_door_used: bool,
    /// Status to apply with its duration in ticks (`None` = state-driven).
    pub apply: Option<(StatusId, Option<u32>)>,
    /// Events.
    pub events: LethalEvents,
}

/// SPEC: canon §10.8 Death's Door HP: 1,000 mHP.
pub const DEATHS_DOOR_HP_MHP: i64 = 1_000;
/// SPEC: canon §10.8 `st.deaths_door` duration: 60 ticks.
pub const DEATHS_DOOR_TICKS: u32 = 60;
/// SPEC: canon §11.1 `st.downed` bleed-out: 30 s.
pub const BLEED_OUT_TICKS: u32 = 1_800;

/// SPEC: canon §10.8 lethal resolution, in order:
/// 1. Errata only: if Death's Door is unused this Folio, call `roll` (§6.4
///    row 43 with keyed `rng.combat`); success sets HP to 1000 and applies
///    `st.deaths_door` for 60 ticks; Death's Door is marked used either way.
/// 2. On failure, if another party member is alive and not downed:
///    `st.downed` with HP 0 and `ev.player_downed`.
/// 3. Otherwise `ev.entity_killed` (NPCs go straight here); a dying Erratum
///    emits `ev.player_died`, and `ev.player_ghosted` if another Erratum lives.
///
/// `roll` is called at most once, only when step 1 runs, so the draw count is
/// exact. GAP: under `st.unmoored` step 1 is skipped and not marked used.
pub fn resolve_lethal(i: LethalInput, roll: impl FnOnce() -> bool) -> LethalDecision {
    let mut used = i.deaths_door_used;
    let mut rolled = false;
    if i.is_erratum && !i.deaths_door_used && !i.unmoored {
        rolled = true;
        used = true;
        if roll() {
            return LethalDecision {
                outcome: LethalOutcome::DeathsDoor,
                hp_mhp: DEATHS_DOOR_HP_MHP,
                deaths_door_rolled: true,
                deaths_door_used: true,
                apply: Some((StatusId::DeathsDoor, Some(DEATHS_DOOR_TICKS))),
                events: LethalEvents::default(),
            };
        }
    }
    if i.is_erratum && i.other_member_up {
        return LethalDecision {
            outcome: LethalOutcome::Downed,
            hp_mhp: 0,
            deaths_door_rolled: rolled,
            deaths_door_used: used,
            apply: Some((StatusId::Downed, Some(BLEED_OUT_TICKS))),
            events: LethalEvents {
                player_downed: true,
                ..LethalEvents::default()
            },
        };
    }
    let ghost = i.is_erratum && i.other_erratum_alive;
    LethalDecision {
        outcome: LethalOutcome::Killed,
        hp_mhp: 0,
        deaths_door_rolled: rolled,
        deaths_door_used: used,
        apply: if ghost {
            Some((StatusId::Ghost, None))
        } else {
            None
        },
        events: LethalEvents {
            entity_killed: true,
            player_downed: false,
            player_died: i.is_erratum,
            player_ghosted: ghost,
        },
    }
}

// ---------------------------------------------------------------------------
// Healing (canon §10.9)
// ---------------------------------------------------------------------------

/// SPEC: canon §10.9 "Healing ... amounts ... are always stated in Bp of the
/// recipient's Max HP": `mul_bp(max_hp, heal_bp)`.
pub const fn heal_amount_mhp(max_hp_mhp: i64, heal_bp: i32) -> i64 {
    mul_bp(max_hp_mhp, heal_bp)
}

/// SPEC: canon §10.9. Result of one heal.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct HealResult {
    /// Amount after multipliers, before the Max HP cap.
    pub gross_mhp: i64,
    /// HP actually restored.
    pub applied_mhp: i64,
    /// HP after.
    pub hp_mhp: i64,
    /// A single heal of at least 10% Max HP ends `st.bleeding` (GAP: measured on `gross_mhp`).
    pub ends_bleeding: bool,
}

/// SPEC: canon §10.9, §11.6, §11.7. `amount × healing received (§6.4 row 3)`,
/// then the status factors in ascending ID order: `st.erasure`
/// `10000 - 300 × stacks`, then `st.withering` × 5000; then capped at Max HP.
pub fn heal(
    hp_mhp: i64,
    max_hp_mhp: i64,
    amount_mhp: i64,
    healing_received_bp: i32,
    erasure_stacks: u8,
    withering: bool,
) -> HealResult {
    let mut g = mul_bp(amount_mhp, healing_received_bp);
    if erasure_stacks > 0 {
        g = mul_bp(g, status::erasure_healing_bp(erasure_stacks));
    }
    if withering {
        g = mul_bp(g, status::WITHERING_HEALING_BP);
    }
    let room = if max_hp_mhp > hp_mhp {
        max_hp_mhp - hp_mhp
    } else {
        0
    };
    let applied = if g < room { g } else { room };
    HealResult {
        gross_mhp: g,
        applied_mhp: applied,
        hp_mhp: hp_mhp + applied,
        ends_bleeding: g >= mul_bp(max_hp_mhp, status::BLEEDING_HEAL_END_BP),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    const fn npc(ar: i64) -> Defender {
        Defender {
            ar_part: ar,
            ..Defender::new(100, TargetKind::Npc)
        }
    }

    const fn erratum(ar: i64) -> Defender {
        Defender {
            ar_part: ar,
            ..Defender::new(200, TargetKind::Erratum)
        }
    }

    fn shares(pairs: &[(DamageType, i32)]) -> Shares {
        Shares::from_pairs(pairs).unwrap()
    }

    fn resist(pairs: &[(DamageType, i32)]) -> [i32; TYPE_COUNT] {
        let mut r = [0; TYPE_COUNT];
        for &(t, v) in pairs {
            r[t.index()] = v;
        }
        r
    }

    fn per_type_sums(b: &DamageBreakdown) -> bool {
        b.per_type.iter().sum::<i64>() == b.total
    }

    /// Positions where the attacker stands 5 m due +Z of the target: the
    /// bearing from target to attacker is `yaw_from_dir(0, 5) = atan2(0, -5)
    /// = 32768`. `target_yaw` sets the angle between facing and bearing.
    fn geometry_at(angle_from_bearing: u16, dy_mm: i64, targeting: bool) -> Geometry {
        Geometry {
            attacker_pos: Fx3::from_milli(0, dy_mm, 5_000),
            target_pos: Fx3::ZERO,
            target_yaw: 32_768u16.wrapping_sub(angle_from_bearing),
            target_targeting_attacker: targeting,
        }
    }

    #[test]
    fn damage_type_table_matches_canon() {
        assert_eq!(DAMAGE_TYPES.len(), TYPE_COUNT);
        for (i, d) in DAMAGE_TYPES.iter().enumerate() {
            assert_eq!(d.ty as usize, i);
            assert_eq!(DamageType::from_key(d.key), Some(d.ty));
            assert!(d.key.starts_with("dmg."));
            assert!(d.color.len() == 7 && d.color.starts_with('#'));
            assert!(
                d.color[1..]
                    .bytes()
                    .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_lowercase())
            );
            if let Some(st) = d.builds {
                assert!(
                    st.buildup_group().is_some(),
                    "{} builds a meterless status",
                    d.key
                );
            }
        }
        let arm: Vec<i32> = DAMAGE_TYPES.iter().map(|d| d.armor_eff_bp).collect();
        assert_eq!(
            arm,
            [10_000, 7_500, 5_000, 0, 0, 0, 0, 0, 2_500, 0, 0, 0, 0, 0, 0]
        );
        let ward: Vec<i32> = DAMAGE_TYPES.iter().map(|d| d.ward_mult_bp).collect();
        assert_eq!(
            ward,
            [
                10_000, 10_000, 10_000, 7_500, 10_000, 15_000, 7_500, 10_000, 10_000, 5_000,
                10_000, 10_000, 10_000, 12_500, 10_000
            ]
        );
        assert_eq!(DamageType::Shock.def().color, "#FFE14D");
        assert_eq!(DamageType::Toxin.def().category, DamageCategory::Biological);
        assert_eq!(DamageType::Sonic.def().category, DamageCategory::Elemental);
        assert_eq!(DamageType::True.def().builds, None);
        assert_eq!(DamageType::Pierce.def().builds, Some(StatusId::Exposed));
        assert_eq!(sonic_noise_db(12_345), 72);
    }

    #[test]
    fn resistance_caps() {
        use DamageType as D;
        assert_eq!(resist_cap_bp(TargetKind::Erratum, D::Slash), Some(5_000));
        assert_eq!(resist_cap_bp(TargetKind::Erratum, D::Blunt), Some(5_000));
        assert_eq!(resist_cap_bp(TargetKind::Erratum, D::Sonic), Some(7_500));
        assert_eq!(resist_cap_bp(TargetKind::Erratum, D::Fire), Some(7_500));
        assert_eq!(resist_cap_bp(TargetKind::Npc, D::Slash), Some(9_000));
        assert_eq!(resist_cap_bp(TargetKind::Npc, D::True), None);
        assert_eq!(
            clamp_resist_bp(TargetKind::Erratum, D::Pierce, 9_000),
            5_000
        );
        assert_eq!(clamp_resist_bp(TargetKind::Npc, D::Pierce, 9_500), 9_000);
        assert_eq!(
            clamp_resist_bp(TargetKind::Npc, D::Pierce, -12_000),
            -10_000
        );
        assert_eq!(clamp_resist_bp(TargetKind::Npc, D::True, 5_000), 0);
    }

    /// Doc 05 E2 step 0: 11,600 → `× P(5)` 16,373 → × 11000 18,010 →
    /// × 11000 19,811 → × 11200 (+2) 22,188 → × iron 10000 22,188.
    /// Doc 05 §11.1 armor: medium body torso 20 at ilvl 10, Rare, Good, +1,
    /// iron: 20 → 32 → 35 → 38 → 40 → 40.
    #[test]
    fn step0_source_magnitude() {
        let s = ItemSource {
            base_mhp: 11_600,
            src_lvl: 5,
            rarity_bp: 11_000,
            quality_bp: 11_000,
            eff_enh: 2,
            material_bp: 10_000,
        };
        assert_eq!(errata_item_source_mhp(&s), 22_188);
        // Best material 14000: 22,188 × 1.4 = 31,063.2 → 31,063.
        assert_eq!(
            errata_item_source_mhp(&ItemSource {
                material_bp: 14_000,
                ..s
            }),
            31_063
        );
        let a = ArmorSource {
            base_ar: 20,
            ilvl: 10,
            rarity_bp: 11_000,
            quality_bp: 11_000,
            eff_enh: 1,
            mat_ar_bp: 10_000,
        };
        assert_eq!(errata_armor_ar(&a), 40);
        // Enemy: 10,000 at d 25 elite: × T(25) 36,145 → × 13000 = 46,988 (46,988.5).
        assert_eq!(enemy_source_mhp(10_000, 25, Rank::Elite), 46_988);
        assert_eq!(enemy_source_mhp(10_000, 1, Rank::Standard), 10_000);
        // Enemy AR: 10 × P(3) = 11 (11.881); 10 × P(5) = 14 (14.115).
        assert_eq!(enemy_armor_ar(10, 3), 11);
        assert_eq!(enemy_armor_ar(10, 5), 14);
        assert_eq!(
            (enhancement_bp(-3), enhancement_bp(0), enhancement_bp(15)),
            (8_200, 10_000, 19_000)
        );
        // Corroded 2 stacks then fortified: 40 × 8000 = 32 → × 12000 = 38 (38.4).
        assert_eq!(effective_ar(40, 2, true), 38);
        assert_eq!(effective_ar(40, 0, false), 40);
    }

    #[test]
    fn source_level_rules() {
        assert_eq!(src_lvl(SourceLevel::Item { ilvl: 7 }), 7);
        assert_eq!(
            src_lvl(SourceLevel::Spell {
                implement_ilvl: Some(9),
                d_current: 20
            }),
            9
        );
        assert_eq!(
            src_lvl(SourceLevel::Spell {
                implement_ilvl: None,
                d_current: 20
            }),
            17
        );
        assert_eq!(
            src_lvl(SourceLevel::Spell {
                implement_ilvl: None,
                d_current: 2
            }),
            1
        );
        assert_eq!(
            src_lvl(SourceLevel::Summon {
                summoner_src_lvl: 4
            }),
            4
        );
        assert_eq!(src_lvl(SourceLevel::Innate { d_current: 12 }), 12);
        assert_eq!(src_lvl(SourceLevel::Environmental { d_current: 3 }), 3);
        assert_eq!(src_lvl(SourceLevel::Enemy { d_current: 30 }), 30);
        assert_eq!(src_lvl(SourceLevel::Crafted { ilvl: 6 }), 6);
    }

    #[test]
    fn shares_validation_and_split_remainder() {
        assert_eq!(
            Shares::from_pairs(&[(DamageType::Slash, 6_000)]),
            Err(SharesError::BadSum)
        );
        assert_eq!(
            Shares::from_pairs(&[(DamageType::Slash, 10_001)]),
            Err(SharesError::OutOfRange)
        );
        assert_eq!(
            Shares::from_pairs(&[(DamageType::Slash, 5_000), (DamageType::Slash, 5_000)]),
            Err(SharesError::Duplicate)
        );
        // 5000/5000 tie: remainder of an odd D2 goes to the earlier row (slash).
        let s = shares(&[(DamageType::Pierce, 5_000), (DamageType::Slash, 5_000)]);
        let d = split_by_type(7, &s);
        assert_eq!((d[0], d[1]), (4, 3));
        // 3334/3333/3333 at D2 = 4: floors 1, 1, 1 → remainder 1 to slash.
        let s3 = shares(&[
            (DamageType::Slash, 3_334),
            (DamageType::Pierce, 3_333),
            (DamageType::Fire, 3_333),
        ]);
        let d3 = split_by_type(4, &s3);
        assert_eq!((d3[0], d3[1], d3[3]), (2, 1, 1));
        assert_eq!(s3.largest(), DamageType::Slash);
    }

    /// Canon §21 test 17 and the §10.5 step 5 example: AR 100, `pen_bp` 0,
    /// a 20,000 mHP slash hit: `DR_bp = floor(10^9 / 180,000) = 5555`, so
    /// the hit becomes `20000 - mul_bp(20000, 5555) = 20000 - 11110 = 8,890`.
    #[test]
    fn canon_t17_armor() {
        assert_eq!(effective_armor(100, DamageType::Slash, 0), 100);
        assert_eq!(armor_dr_bp(100, 20_000), 5_555);
        let b = resolve_hit(
            &Hit::basic(20_000, Shares::single(DamageType::Slash)),
            &npc(100),
            &Rules::SOLO,
        );
        assert_eq!(b.dr_bp[0], 5_555);
        assert_eq!(b.after_armor[0], 8_890);
        assert_eq!(b.total, 8_890);
    }

    /// Canon §10.5 step 5 worked example, step by step on the primitives:
    /// `effAR = mul_bp(floor_div(100 × 10000, 10000), 10000 - 0) = 100`;
    /// `DR_bp = min(8500, (100 × 10^7) / (100 × 1000 + 4 × 20000))
    /// = 10^9 / 180,000 = 5555`; `d = 20000 - mul_bp(20000, 5555) = 8,890`.
    /// With `pen_bp` 5000 the same hit sees effAR 50:
    /// `DR = 5×10^8 / 130,000 = 3846`, `d = 20000 - 7692 = 12,308`.
    #[test]
    fn canon_10_5_armor_example() {
        let mut d = single(DamageType::Slash, 20_000);
        let mut dr = [0; TYPE_COUNT];
        apply_armor(&mut d, 100, 0, &mut dr);
        assert_eq!((dr[0], d[0]), (5_555, 8_890));
        let mut d2 = single(DamageType::Slash, 20_000);
        apply_armor(&mut d2, 100, 5_000, &mut dr);
        assert_eq!((dr[0], d2[0]), (3_846, 12_308));
        // Elemental types are untouched by armor; sonic uses efficiency 2500.
        let mut d3 = single(DamageType::Fire, 20_000);
        apply_armor(&mut d3, 100, 0, &mut dr);
        assert_eq!(d3[DamageType::Fire.index()], 20_000);
        assert_eq!(effective_armor(100, DamageType::Sonic, 0), 25);
    }

    /// Canon §21 test 18: `D2` 10,000 pure slash on an NPC with slash
    /// resistance 9000 and AR 1000. Resistance leaves `10000 - 9000 = 1000`;
    /// `DR = 10^10 / (10^6 + 4000) = 9960` capped at 8500, leaving
    /// `1000 - 850 = 150`; the NPC chip floor `floor_div(10000, 20) = 500`
    /// adds 350.
    #[test]
    fn canon_t18_chip_floor_npc() {
        let def = Defender {
            resist_bp: resist(&[(DamageType::Slash, 9_000)]),
            ..npc(1_000)
        };
        let b = resolve_hit(
            &Hit::basic(10_000, Shares::single(DamageType::Slash)),
            &def,
            &Rules::SOLO,
        );
        assert_eq!(b.d2, 10_000);
        assert_eq!(b.after_resist[0], 1_000);
        assert_eq!(b.dr_bp[0], 8_500);
        assert_eq!(b.after_armor[0], 150);
        assert_eq!(b.chip_added, 350);
        assert_eq!(b.total, 500);
    }

    /// Canon §21 test 19: the same hit on an Erratum: resistance 9000 caps
    /// at 5000, leaving 5,000; `DR = 10^10 / 1,020,000 = 9803` capped at
    /// 8500, leaving 750; chip floor `floor_div(10000, 10) = 1000`.
    #[test]
    fn canon_t19_chip_floor_erratum() {
        let def = Defender {
            resist_bp: resist(&[(DamageType::Slash, 9_000)]),
            ..erratum(1_000)
        };
        let b = resolve_hit(
            &Hit::basic(10_000, Shares::single(DamageType::Slash)),
            &def,
            &Rules::SOLO,
        );
        assert_eq!(b.after_resist[0], 5_000);
        assert_eq!(b.dr_bp[0], 8_500);
        assert_eq!(b.after_armor[0], 750);
        assert_eq!(b.total, 1_000);
    }

    /// Canon §21 test 41: ward 10,000 hit by 8,000 shock: `w = mul_bp(8000,
    /// 15000) = 12,000 > 10,000`, so shock becomes `8000 - floor_div(10^8,
    /// 15000) = 8000 - 6666 = 1,334` and the ward breaks. Doc 05 §15.4: a
    /// kinetic generator (shock profile 15000) gives `ward_mult_eff = 22500`
    /// and HP `8000 - floor_div(10^8, 22500) = 8000 - 4444 = 3,556`.
    #[test]
    fn canon_t41_ward_depletion() {
        let mut d = single(DamageType::Shock, 8_000);
        let out = absorb_wards(
            &mut d,
            &Wards {
                shielded_mhp: 10_000,
                ..Wards::NONE
            },
            false,
        );
        assert_eq!(d[DamageType::Shock.index()], 1_334);
        assert_eq!(out.shielded_after, 0);
        assert!(out.shielded_broken && out.broken());
        assert_eq!(out.lost(), 10_000);
        // Same through the whole pipeline (shock has no armor; NPC chip floor 400 < 8000).
        let def = Defender {
            wards: Wards {
                shielded_mhp: 10_000,
                ..Wards::NONE
            },
            ..npc(0)
        };
        let b = resolve_hit(
            &Hit::basic(8_000, Shares::single(DamageType::Shock)),
            &def,
            &Rules::SOLO,
        );
        assert_eq!(b.total, 1_334);
        assert_eq!(b.hp_and_ward_lost(), 11_334);
        // Generator with the kinetic profile.
        let mut profile = [10_000; TYPE_COUNT];
        profile[DamageType::Shock.index()] = 15_000;
        let mut d2 = single(DamageType::Shock, 8_000);
        let kin = absorb_wards(
            &mut d2,
            &Wards {
                shielded_mhp: 0,
                generator_mhp: 10_000,
                generator_profile_bp: profile,
            },
            false,
        );
        assert_eq!(d2[DamageType::Shock.index()], 3_556);
        assert!(kin.generator_broken && !kin.shielded_broken);
        // Shielded first, generator second, §10.1 order inside each pool.
        let mut d3 = [0; TYPE_COUNT];
        d3[DamageType::Slash.index()] = 3_000;
        d3[DamageType::Fire.index()] = 4_000;
        let both = absorb_wards(
            &mut d3,
            &Wards {
                shielded_mhp: 3_500,
                generator_mhp: 2_000,
                generator_profile_bp: [10_000; TYPE_COUNT],
            },
            false,
        );
        // Shielded: slash w 3000 <= 3500 → 500 left; fire w = 3000 > 500 → fire -= floor_div(500×10^4, 7500) = 666 → 3334.
        // Generator: fire w = mul_bp(3334, 7500) = 2500 > 2000 → fire -= floor_div(2×10^7, 7500) = 2666 → 668.
        assert_eq!(
            (d3[DamageType::Slash.index()], d3[DamageType::Fire.index()]),
            (0, 668)
        );
        assert!(both.shielded_broken && both.generator_broken);
    }

    /// Canon §21 test 42: rifle hit Erratum → Erratum with 10,000 before
    /// step 2.15 gives `D2 = mul_bp(10000, 5000) = 5,000`; a grenade blast
    /// (area class) keeps 10,000. Modes: `ff_off` 0 / 5000, `ff_full` 10000.
    #[test]
    fn canon_t42_friendly_fire() {
        let mut rifle = Hit::basic(10_000, Shares::single(DamageType::Pierce));
        rifle.attacker.errata_faction = true;
        let ally = erratum(0);
        assert_eq!(resolve_hit(&rifle, &ally, &Rules::SOLO).d2, 5_000);
        let grenade = Hit {
            ff_class: FfClass::Area,
            area: true,
            ..rifle
        };
        assert_eq!(resolve_hit(&grenade, &ally, &Rules::SOLO).d2, 10_000);
        let off = Rules {
            party_size: 2,
            ff_mode: FfMode::Off,
        };
        let full = Rules {
            party_size: 2,
            ff_mode: FfMode::Full,
        };
        assert_eq!(resolve_hit(&rifle, &ally, &off).d2, 0);
        assert_eq!(resolve_hit(&grenade, &ally, &off).d2, 5_000);
        assert_eq!(resolve_hit(&rifle, &ally, &full).d2, 10_000);
        // An Errata minion target counts as fac.errata.
        let minion = Defender {
            errata_faction: true,
            ..npc(0)
        };
        assert_eq!(resolve_hit(&rifle, &minion, &Rules::SOLO).d2, 5_000);
        // No friendly fire against NPC enemies.
        assert_eq!(resolve_hit(&rifle, &npc(0), &Rules::SOLO).d2, 10_000);
    }

    /// Canon §21 test 38 at the function level: Erratum A is at
    /// `aware.alert` with guard G1, while G2's awareness of A is
    /// `aware.unaware`. Eligibility reads only the (A, G2) pair, so A's hit
    /// on G2 is a sneak attack, and the hit draws no crit roll.
    #[test]
    fn canon_t38_sneak_eligibility_per_target() {
        use std::collections::BTreeMap;
        let (a, g1, g2) = (1u64, 11u64, 12u64);
        let mut pairs: BTreeMap<(u64, u64), Awareness> = BTreeMap::new();
        pairs.insert((g1, a), Awareness::Alert);
        pairs.insert((g2, a), Awareness::Unaware);
        let pair = |obs: u64| SneakPair {
            target_sleeping: false,
            target_has_perception: true,
            target_awareness_of_attacker: pairs.get(&(obs, a)).copied().unwrap_or_default(),
        };
        assert!(sneak_eligible(pair(g2)));
        assert!(!sneak_eligible(pair(g1)));
        // Everything below alert is eligible; a sleeping target always is.
        for aw in [
            Awareness::Curious,
            Awareness::Suspicious,
            Awareness::Searching,
            Awareness::Lost,
        ] {
            assert!(sneak_eligible(SneakPair {
                target_awareness_of_attacker: aw,
                ..pair(g2)
            }));
        }
        assert!(sneak_eligible(SneakPair {
            target_sleeping: true,
            ..pair(g1)
        }));
        // Errata (no perception) only while sleeping.
        assert!(!sneak_eligible(SneakPair {
            target_has_perception: false,
            ..pair(g2)
        }));
        // No crit draw: the keyed generator's state is untouched.
        let mut rng = combat_rng(0x0123_4567_89AB_CDEF, 3, a, 1_000, 0);
        let before = rng.state();
        assert_eq!(roll_crit(&mut rng, 7_500, sneak_eligible(pair(g2))), None);
        assert_eq!(rng.state(), before);
        // A normal hit draws exactly one value (canon §21 test 7: first range(10000) is 8822).
        let mut r2 = combat_rng(0x0123_4567_89AB_CDEF, 3, (3 << 40) | 5, 1_000, 0);
        assert_eq!(roll_crit(&mut r2, 8_823, false), Some(true));
        let mut r3 = combat_rng(0x0123_4567_89AB_CDEF, 3, (3 << 40) | 5, 1_000, 0);
        assert_eq!(roll_crit(&mut r3, 8_822, false), Some(false));
        // The pipeline applies the sneak multiplier and never a crit for a sneak strike.
        let mut hit = Hit::basic(10_000, Shares::single(DamageType::Pierce));
        hit.strike = Strike::Sneak { mult_bp: 30_000 };
        let b = resolve_hit(&hit, &npc(0), &Rules::SOLO);
        assert_eq!(
            (b.chain.crit, b.chain.sneak, b.d2),
            (10_000, 30_000, 30_000)
        );
        assert!(b.flags.sneak && !b.flags.crit);
    }

    /// Doc 05 test 22: `dy` 2,500 → 11000; 2,499 → 10000; −2,500 → 9500.
    /// `ang_diff` 10,923 → 10000; 10,924 with the target not targeting the
    /// attacker → 11500; 10,924 while targeting → 10000; Errata ignore targeting.
    #[test]
    fn elevation_and_flanking() {
        let y = |mm: i64| fx_from_milli(mm);
        assert_eq!(elevation_bp(y(2_500), y(0)), 11_000);
        assert_eq!(elevation_bp(y(2_499), y(0)), 10_000);
        assert_eq!(elevation_bp(y(0), y(2_500)), 9_500);
        assert_eq!(elevation_bp(y(10_000), y(7_501)), 10_000);
        assert_eq!(ranged_range_bonus(y(3_000), y(0)), y(3_000));
        assert_eq!(ranged_range_bonus(y(0), y(3_000)), Fx::ZERO);
        assert_eq!(flanking_bp(&geometry_at(10_923, 0, false), false), 10_000);
        assert_eq!(flanking_bp(&geometry_at(10_924, 0, false), false), 11_500);
        assert_eq!(flanking_bp(&geometry_at(10_924, 0, true), false), 10_000);
        assert_eq!(flanking_bp(&geometry_at(10_924, 0, true), true), 11_500);
        assert_eq!(flanking_bp(&geometry_at(32_768, 0, false), false), 11_500);
        assert_eq!(flanking_bp(&geometry_at(0, 0, false), false), 10_000);
    }

    #[test]
    fn step1_and_simple_step2_factors() {
        // Bonuses with potency: 2000 at 6000 → 1200; 1500 at 10000; row 12 600: add = 3300.
        let bs = [
            Bonus {
                bonus_bp: 2_000,
                potency_bp: 6_000,
            },
            Bonus {
                bonus_bp: 1_500,
                potency_bp: 10_000,
            },
        ];
        assert_eq!(additive_bp(600, &bs), 3_300);
        // Clamp at −9000: D1 = mul_bp(B, 1000).
        let neg = [Bonus {
            bonus_bp: -9_500,
            potency_bp: 10_000,
        }];
        assert_eq!(additive_bp(0, &neg), -9_000);
        let mut h = Hit::basic(10_000, Shares::single(DamageType::True));
        h.bonuses = &neg;
        assert_eq!(resolve_hit(&h, &npc(0), &Rules::SOLO).d1, 1_000);
        // MYTHOS: MYT 10 tagged → 11500; MYT 0 tagged → 8500; untagged → 10000.
        assert_eq!(
            (
                mythos_bp(10, true),
                mythos_bp(0, true),
                mythos_bp(10, false)
            ),
            (11_500, 8_500, 10_000)
        );
        let mut m = Hit::basic(10_000, Shares::single(DamageType::True));
        m.local_mythos = 10;
        m.attacker.tags.undead = true;
        let ghoul_target = Defender {
            tags: CreatureTags {
                fiend: true,
                ..CreatureTags::default()
            },
            ..npc(0)
        };
        assert_eq!(resolve_hit(&m, &ghoul_target, &Rules::SOLO).d2, 11_500); // once, not twice
        // Shortfall: s = 10 → 9000; s = 60 → 5000.
        assert_eq!(
            (
                shortfall_bp(55, 45),
                shortfall_bp(100, 40),
                shortfall_bp(40, 60)
            ),
            (9_000, 5_000, 10_000)
        );
        assert_eq!(party_damage_bp(3), 12_000);
        // Party damage only NPC → Erratum.
        let mut e = Hit::basic(10_000, Shares::single(DamageType::True));
        e.attacker.is_npc = true;
        let party4 = Rules {
            party_size: 4,
            ff_mode: FfMode::Default,
        };
        assert_eq!(resolve_hit(&e, &erratum(0), &party4).d2, 13_000);
        assert_eq!(resolve_hit(&e, &npc(0), &party4).d2, 10_000);
    }

    #[test]
    fn status_steps_in_ascending_id_order() {
        // Target: exposed 3 (12400) then knocked_down (11000). Attacker: berserk, empowered, weakened.
        let def = Defender {
            statuses: DefenderStatuses {
                exposed_stacks: 3,
                knocked_down: true,
                ..DefenderStatuses::default()
            },
            ..npc(0)
        };
        let mut h = Hit::basic(10_000, Shares::single(DamageType::True));
        h.attacker.statuses = AttackerStatuses {
            berserk: true,
            empowered: true,
            weakened: true,
            ..AttackerStatuses::default()
        };
        let b = resolve_hit(&h, &def, &Rules::SOLO);
        // 10000 → 12400 → 13640 → 17732 → 21278 (21278.4) → 15958 (15958.5).
        assert_eq!(
            (
                b.chain.exposed,
                b.chain.knocked_down,
                b.chain.berserk,
                b.chain.empowered,
                b.chain.weakened
            ),
            (12_400, 11_000, 13_000, 12_000, 7_500)
        );
        assert_eq!(b.d2, 15_958);
        // Taunted: × 7500 only against someone other than the taunter.
        let mut t = Hit::basic(10_000, Shares::single(DamageType::True));
        t.attacker.statuses.taunted_by = Some(100);
        assert_eq!(resolve_hit(&t, &npc(0), &Rules::SOLO).d2, 10_000); // npc() has id 100
        assert_eq!(
            resolve_hit(&t, &Defender { id: 101, ..npc(0) }, &Rules::SOLO).d2,
            7_500
        );
        // Charmed player: × 0 against the applier's faction; × 10000 instead of FF on allies.
        let mut c = Hit::basic(10_000, Shares::single(DamageType::True));
        c.attacker.errata_faction = true;
        c.attacker.statuses.charmed_player = Some(CharmedPlayer {
            target_in_applier_faction: true,
        });
        let hostile = resolve_hit(&c, &npc(0), &Rules::SOLO);
        assert_eq!((hostile.d2, hostile.total), (0, 0));
        c.attacker.statuses.charmed_player = Some(CharmedPlayer {
            target_in_applier_faction: false,
        });
        assert_eq!(resolve_hit(&c, &erratum(0), &Rules::SOLO).d2, 10_000);
    }

    #[test]
    fn precision_swarm_and_type_mods() {
        let mut h = Hit::basic(10_000, Shares::single(DamageType::True));
        h.precision_mult_bp = PRECISION_RANGED_BP;
        let head = Defender {
            part: HitPart::HEAD,
            ..npc(0)
        };
        assert_eq!(resolve_hit(&h, &head, &Rules::SOLO).d2, 15_000);
        let limb = Defender {
            part: HitPart::LIMB,
            ..npc(0)
        };
        assert_eq!(resolve_hit(&h, &limb, &Rules::SOLO).d2, 8_500);
        // Area hits: no precision; × 15000 on a swarm.
        let area = Hit { area: true, ..h };
        assert_eq!(resolve_hit(&area, &head, &Rules::SOLO).d2, 10_000);
        assert_eq!(
            resolve_hit(
                &area,
                &Defender {
                    swarm: true,
                    ..npc(0)
                },
                &Rules::SOLO
            )
            .d2,
            15_000
        );
        // 3a radiant vs undead: 4000 radiant → 6000.
        let rad = Hit::basic(4_000, Shares::single(DamageType::Radiant));
        let undead = Defender {
            tags: CreatureTags {
                undead: true,
                ..CreatureTags::default()
            },
            ..npc(0)
        };
        assert_eq!(resolve_hit(&rad, &undead, &Rules::SOLO).total, 6_000);
        // 3a frozen: blunt × 15000 and frozen consumed; a fire hit leaves it.
        let frozen = Defender {
            statuses: DefenderStatuses {
                frozen: true,
                ..DefenderStatuses::default()
            },
            ..npc(0)
        };
        let blunt = resolve_hit(
            &Hit::basic(4_000, Shares::single(DamageType::Blunt)),
            &frozen,
            &Rules::SOLO,
        );
        assert_eq!((blunt.total, blunt.frozen_consumed), (6_000, true));
        let fire = resolve_hit(
            &Hit::basic(4_000, Shares::single(DamageType::Fire)),
            &frozen,
            &Rules::SOLO,
        );
        assert_eq!((fire.total, fire.frozen_consumed), (4_000, false));
        let sonic = resolve_hit(
            &Hit::basic(4_000, Shares::single(DamageType::Sonic)),
            &frozen,
            &Rules::SOLO,
        );
        assert_eq!(
            (sonic.total, sonic.frozen_consumed, sonic.sonic_noise_db),
            (4_000, true, Some(64))
        );
        // Frost applies chilled unless immune.
        let frost = Hit::basic(4_000, Shares::single(DamageType::Frost));
        assert!(resolve_hit(&frost, &npc(0), &Rules::SOLO).applies_chilled);
        let immune = Defender {
            immune: TypeSet::EMPTY.with(DamageType::Frost),
            ..npc(0)
        };
        assert!(!resolve_hit(&frost, &immune, &Rules::SOLO).applies_chilled);
    }

    #[test]
    fn resistance_immunity_and_chip_allocation() {
        // −5000 fire resistance: 4000 + 2000 = 6000. True ignores resistance.
        let weak = Defender {
            resist_bp: resist(&[(DamageType::Fire, -5_000), (DamageType::True, 9_000)]),
            ..npc(0)
        };
        assert_eq!(
            resolve_hit(
                &Hit::basic(4_000, Shares::single(DamageType::Fire)),
                &weak,
                &Rules::SOLO
            )
            .total,
            6_000
        );
        assert_eq!(
            resolve_hit(
                &Hit::basic(4_000, Shares::single(DamageType::True)),
                &weak,
                &Rules::SOLO
            )
            .total,
            4_000
        );
        // 50% fire / 50% slash on a fire-immune Erratum with slash 5000 and AR 1000:
        // fire → 0; slash 5000 → 2500 → DR capped 8500 → 375; chip floor 1000 → slash 1000.
        let def = Defender {
            immune: TypeSet::EMPTY.with(DamageType::Fire),
            resist_bp: resist(&[(DamageType::Slash, 5_000)]),
            ..erratum(1_000)
        };
        let s = shares(&[(DamageType::Fire, 5_000), (DamageType::Slash, 5_000)]);
        let b = resolve_hit(&Hit::basic(10_000, s), &def, &Rules::SOLO);
        assert_eq!(
            (
                b.per_type[DamageType::Fire.index()],
                b.per_type[DamageType::Slash.index()],
                b.chip_added
            ),
            (0, 1_000, 625)
        );
        // Fully immune hit: no chip floor at all.
        let all_fire = resolve_hit(
            &Hit::basic(10_000, Shares::single(DamageType::Fire)),
            &def,
            &Rules::SOLO,
        );
        assert_eq!((all_fire.total, all_fire.chip_added), (0, 0));
        // Immune types skip buildup.
        let mut hb = Hit::basic(10_000, Shares::single(DamageType::Fire));
        let mut power = [0; TYPE_COUNT];
        power[DamageType::Fire.index()] = 30;
        hb.buildup = Some(HitBuildup {
            power,
            src_lvl: 1,
            attacker_buildup_bp: 10_000,
        });
        assert_eq!(
            resolve_hit(&hb, &def, &Rules::SOLO).buildup[DamageType::Fire.index()],
            0
        );
        assert_eq!(
            resolve_hit(&hb, &npc(0), &Rules::SOLO).buildup[DamageType::Fire.index()],
            30
        );
        // ignore_armor skips step 5.
        let pierce = Hit {
            ignore_armor: true,
            ..Hit::basic(10_000, Shares::single(DamageType::Pierce))
        };
        assert_eq!(resolve_hit(&pierce, &npc(100), &Rules::SOLO).total, 10_000);
    }

    #[test]
    fn invulnerable_target_takes_nothing() {
        let def = Defender {
            statuses: DefenderStatuses {
                invulnerable: true,
                ..DefenderStatuses::default()
            },
            wards: Wards {
                shielded_mhp: 5_000,
                ..Wards::NONE
            },
            ..erratum(0)
        };
        let b = resolve_hit(
            &Hit::basic(50_000, Shares::single(DamageType::True)),
            &def,
            &Rules::SOLO,
        );
        assert!(b.flags.nullified);
        assert_eq!((b.total, b.ward.lost()), (0, 0));
        assert!(resolve_dot(500, DamageType::True, &def).flags.nullified);
    }

    // ---------------- End-to-end worked examples (doc 05 §10.4) ----------------

    const fn long_blade() -> Shares {
        let mut s = [0; TYPE_COUNT];
        s[0] = 8_000; // slash
        s[1] = 2_000; // pierce
        Shares(s)
    }

    /// E1, crit long-blade finisher on the REF target (NPC, AR 10).
    /// Attack factor: `mul_bp(14700, 13000) = 19,110`. Step 0 (Common Normal
    /// +0 iron, ilvl 1) keeps 19,110. Step 1: skill 20 × 30 = add 600,
    /// `D1 = mul_bp(19110, 10600) = 20,256` (20,256.6). Crit × 15000:
    /// `D2 = 30,384`. Split 8000/2000: slash 24,307 + remainder 1 = 24,308,
    /// pierce 6,076. Armor: slash effAR 10, `DR = 10^8 / 107,232 = 932`,
    /// slash `24308 - 2265 = 22,043`; pierce effAR 7, `DR = 7×10^7 / 31,304
    /// = 2236`, pierce `6076 - 1358 = 4,718`. Total 26,761. Without the
    /// crit: `D2 = 20,256`, slash 16,204 + 1 = 16,205 (DR `10^8/74,820 =
    /// 1336`, `16205 - 2164 = 14,041`), pierce 4,051 (DR `7×10^7/23,204 =
    /// 3016`, `4051 - 1221 = 2,830`): total 16,871.
    #[test]
    fn e2e_e1_crit_long_blade() {
        let src = ItemSource {
            base_mhp: mul_bp(14_700, 13_000),
            src_lvl: 1,
            rarity_bp: 10_000,
            quality_bp: 10_000,
            eff_enh: 0,
            material_bp: 10_000,
        };
        let b0 = errata_item_source_mhp(&src);
        assert_eq!(b0, 19_110);
        let mut hit = Hit::basic(b0, long_blade());
        hit.weapon_scaling_bp = 20 * 30;
        hit.strike = Strike::Normal {
            crit_mult_bp: Some(CRIT_MULT_BP),
        };
        let b = resolve_hit(&hit, &npc(10), &Rules::SOLO);
        assert_eq!((b.d1, b.d2), (20_256, 30_384));
        assert_eq!((b.split[0], b.split[1]), (24_308, 6_076));
        assert_eq!((b.dr_bp[0], b.dr_bp[1]), (932, 2_236));
        assert_eq!(
            (b.per_type[0], b.per_type[1], b.total),
            (22_043, 4_718, 26_761)
        );
        assert!(b.flags.crit && per_type_sums(&b));
        let no_crit = resolve_hit(
            &Hit {
                strike: Strike::Normal { crit_mult_bp: None },
                ..hit
            },
            &npc(10),
            &Rules::SOLO,
        );
        assert_eq!(no_crit.total, 16_871);
    }

    /// E2, marksman headshot (pierce, `pen_bp` 3000, precision 17500) on the
    /// Folio-5 standard target. Step 0: 22,188 (see `step0_source_magnitude`).
    /// Step 1: `add = floor_div(7000×20,100) + floor_div(3000×10,100) + 35×30
    /// = 1400 + 300 + 1050 = 2750`; `D1 = mul_bp(22188, 12750) = 28,289`.
    /// Head × 17500 = 49,505. AR `mul_bp(10, P(5)) = 14`; effAR
    /// `mul_bp(floor_div(14×7500, 10000) = 10, 7000) = 7`; `DR = 7×10^7 /
    /// (7000 + 198,020) = 341`; `49505 - floor(49505×341/10^4) = 49505 - 1688
    /// = 47,817`. Torso: `DR = 7×10^7/120,156 = 582`, `28289 - 1646 = 26,643`.
    #[test]
    fn e2e_e2_precision_headshot_from_step0() {
        let b0 = errata_item_source_mhp(&ItemSource {
            base_mhp: 11_600,
            src_lvl: 5,
            rarity_bp: 11_000,
            quality_bp: 11_000,
            eff_enh: 2,
            material_bp: 10_000,
        });
        let mut hit = Hit::basic(b0, Shares::single(DamageType::Pierce));
        hit.pen_bp = 3_000;
        hit.precision_mult_bp = 17_500;
        hit.weapon_scaling_bp = 2_750;
        let head = Defender {
            part: HitPart::HEAD,
            ..npc(enemy_armor_ar(10, 5))
        };
        let b = resolve_hit(&hit, &head, &Rules::SOLO);
        assert_eq!(
            (b.d1, b.d2, b.dr_bp[1], b.total),
            (28_289, 49_505, 341, 47_817)
        );
        assert!(b.flags.precision);
        let torso = resolve_hit(&hit, &npc(enemy_armor_ar(10, 5)), &Rules::SOLO);
        assert_eq!(
            (torso.d2, torso.dr_bp[1], torso.total),
            (28_289, 582, 26_643)
        );
    }

    /// E3, sneak attack with flanking (heavy dagger, pierce 6000 / slash
    /// 4000). Base `mul_bp(10100, 22000) = 22,220`; add 1600 → `D1 =
    /// mul_bp(22220, 11600) = 25,775`. Sneak tier 1 × 30000 = 77,325 (no crit
    /// roll); flanking × 11500 = 88,923. Split: pierce 53,353 + 1 = 53,354,
    /// slash 35,569. AR 10: pierce DR `7×10^7/220,416 = 317` → 51,663; slash
    /// DR `10^8/152,276 = 656` → 33,236. Total 84,899.
    #[test]
    fn e2e_e3_sneak_flank_dagger() {
        let s = shares(&[(DamageType::Pierce, 6_000), (DamageType::Slash, 4_000)]);
        let mut hit = Hit::basic(mul_bp(10_100, 22_000), s);
        hit.weapon_scaling_bp = 1_600;
        hit.strike = Strike::Sneak { mult_bp: 30_000 };
        hit.geometry = geometry_at(32_768, 0, false); // directly behind, not targeted
        let b = resolve_hit(&hit, &npc(10), &Rules::SOLO);
        assert_eq!(
            (b.d1, b.chain.sneak, b.chain.flanking, b.d2),
            (25_775, 30_000, 11_500, 88_923)
        );
        assert_eq!((b.split[1], b.split[0]), (53_354, 35_569));
        assert_eq!((b.dr_bp[1], b.dr_bp[0]), (317, 656));
        assert_eq!(
            (b.per_type[1], b.per_type[0], b.total),
            (51_663, 33_236, 84_899)
        );
    }

    /// E4, canon §5.6 smart SMG in Fairy Tale (C = 4, potency 6000, E = 6,
    /// `edge_bp` 2400). Step 0: 4,500 → `× P(3)` 5,346 → × 10500 5,613;
    /// `D1 = mul_bp(5613, 10600) = 5,949`. Target AR `mul_bp(10, P(3)) = 11`,
    /// pierce effAR 8. Coherent: DR `8×10^7/31,796 = 2516` → 4,453.
    /// Contradicted vs non-native: × 6000 = 3,569, DR 3591 → 2,288.
    /// Native wolf: × 12400 = 4,425, DR 3112 → 3,048.
    #[test]
    fn e2e_e4_contradiction_and_anachronism() {
        let b0 = errata_item_source_mhp(&ItemSource {
            base_mhp: 4_500,
            src_lvl: 3,
            rarity_bp: 10_500,
            quality_bp: 10_000,
            eff_enh: 0,
            material_bp: 10_000,
        });
        assert_eq!(b0, 5_613);
        let mut coherent = Hit::basic(b0, Shares::single(DamageType::Pierce));
        coherent.weapon_scaling_bp = 600;
        let target = npc(enemy_armor_ar(10, 3));
        let c = resolve_hit(&coherent, &target, &Rules::SOLO);
        assert_eq!((c.d2, c.dr_bp[1], c.total), (5_949, 2_516, 4_453));
        let contradicted = Hit {
            base_potency_bp: 6_000,
            edge_bp: 2_400,
            ..coherent
        };
        let x = resolve_hit(&contradicted, &target, &Rules::SOLO);
        assert_eq!((x.d2, x.dr_bp[1], x.total), (3_569, 3_591, 2_288));
        let wolf = Defender {
            native: true,
            ..target
        };
        let w = resolve_hit(&contradicted, &wolf, &Rules::SOLO);
        assert_eq!(
            (w.chain.anachronism, w.d2, w.dr_bp[1], w.total),
            (12_400, 4_425, 3_112, 3_048)
        );
    }

    /// E5, SMG (D1 4,770, `pen` 0) and sniper (D1 43,672, `pen` 5000) at AR
    /// 10 and 40. SMG AR 10: effAR 7, DR `7×10^7/26,080 = 2684` → 3,490;
    /// AR 40: effAR 30, DR `3×10^8/49,080 = 6112` → 1,855. Sniper AR 10:
    /// effAR `mul_bp(7, 5000) = 3`, DR `3×10^7/177,688 = 168` → 42,939;
    /// AR 40: effAR 15, DR 790 → 40,222.
    #[test]
    fn e2e_e5_armor_smg_vs_sniper() {
        let smg = Hit::basic(4_770, Shares::single(DamageType::Pierce));
        let sniper = Hit {
            pen_bp: 5_000,
            ..Hit::basic(43_672, Shares::single(DamageType::Pierce))
        };
        let r = |h: &Hit<'_>, ar: i64| {
            let b = resolve_hit(h, &npc(ar), &Rules::SOLO);
            (b.dr_bp[1], b.total)
        };
        assert_eq!(r(&smg, 10), (2_684, 3_490));
        assert_eq!(r(&smg, 40), (6_112, 1_855));
        assert_eq!(r(&sniper, 10), (168, 42_939));
        assert_eq!(r(&sniper, 40), (790, 40_222));
    }

    /// E6, blocked NPC hit on an Erratum with a heater shield, party of 2.
    /// `B = 10000 × T(1) × standard = 10,000`; × 11000 (party 2) = 11,000.
    /// AR torso 20 + heater 25 = 45: `DR = 4.5×10^8/89,000 = 5056` → 5,439.
    /// Chip floor 1,100 does not bind. Block stability 5000 + 2500 + 600 =
    /// 8100: `to_stamina = mul_bp(5439, 8100) = 4,405`, HP 1,034, stamina
    /// `floor_div(4405 × 10000, T(1)) = 4,405` mst. Unblocked (AR 20): DR
    /// 3125 → 7,563.
    #[test]
    fn e2e_e6_blocked_hit() {
        let b0 = enemy_source_mhp(10_000, 1, Rank::Standard);
        let mut hit = Hit::basic(b0, Shares::single(DamageType::Slash));
        hit.attacker.is_npc = true;
        hit.block = Some(BlockInput {
            stability_bp: 8_100,
            d_current: 1,
            stamina_mst: 90_000,
            guard_break: false,
        });
        let duo = Rules {
            party_size: 2,
            ff_mode: FfMode::Default,
        };
        let b = resolve_hit(&hit, &erratum(45), &duo);
        assert_eq!((b.d2, b.dr_bp[0], b.after_chip[0]), (11_000, 5_056, 5_439));
        let blk = b.block.unwrap();
        assert_eq!(
            (
                blk.to_stamina_mhp,
                b.total,
                blk.stamina_loss_mst,
                blk.guard_broken
            ),
            (4_405, 1_034, 4_405, false)
        );
        assert!(b.flags.blocked && per_type_sums(&b));
        let unblocked = resolve_hit(&Hit { block: None, ..hit }, &erratum(20), &duo);
        assert_eq!((unblocked.dr_bp[0], unblocked.total), (3_125, 7_563));
        // Stamina normalized by depth: at d 25 the same block costs floor_div(4405×10^4, 36145) = 1,218.
        assert_eq!(block_stamina_loss_mst(4_405, 25), 1_218);
        // Guard break: low stamina reaches 0.
        let weak = Hit {
            block: Some(BlockInput {
                stamina_mst: 4_000,
                ..hit.block.unwrap()
            }),
            ..hit
        };
        let wb = resolve_hit(&weak, &erratum(45), &duo).block.unwrap();
        assert_eq!((wb.stamina_after_mst, wb.guard_broken), (0, true));
    }

    /// E7, warded champion and `counter.ward_breaker`. Ward 180,000 (50% of
    /// 360,000). Sniper torso shot after armor 42,939 pierce. With the
    /// breaker `ward_mult_eff = mul_bp(mul_bp(10000, 10000), 30000) = 30000`:
    /// `w = 128,817 <= 180,000` → ward 51,183, HP 0. Second shot: `w >
    /// 51,183` → HP `42939 - floor_div(51183 × 10^4, 30000) = 42939 - 17061 =
    /// 25,878`, ward broken, stun 90 × champion 5000 = 45 ticks. Without the
    /// breaker shot 1 leaves 137,061.
    #[test]
    fn e2e_e7_ward_breaker_champion() {
        let mut shot = Hit::basic(43_672, Shares::single(DamageType::Pierce));
        shot.pen_bp = 5_000;
        shot.ward_breaker = true;
        let mut champ = Defender {
            wards: Wards {
                generator_mhp: 180_000,
                ..Wards::NONE
            },
            ..npc(10)
        };
        let s1 = resolve_hit(&shot, &champ, &Rules::SOLO);
        assert_eq!(
            (
                s1.after_chip[1],
                s1.total,
                s1.ward.generator_after,
                s1.breaker_stun
            ),
            (42_939, 0, 51_183, false)
        );
        champ.wards.generator_mhp = s1.ward.generator_after;
        let s2 = resolve_hit(&shot, &champ, &Rules::SOLO);
        assert_eq!((s2.total, s2.ward.generator_after), (25_878, 0));
        assert!(s2.ward.generator_broken && s2.breaker_stun);
        let stun = status::direct_duration(
            StatusId::Stunned,
            status::Thresholds::BASELINE,
            status::Bearer {
                rank: Some(Rank::Champion),
                ..status::Bearer::default()
            },
        );
        assert_eq!(stun, Some(45));
        let plain = resolve_hit(
            &Hit {
                ward_breaker: false,
                ..shot
            },
            &Defender {
                wards: Wards {
                    generator_mhp: 180_000,
                    ..Wards::NONE
                },
                ..npc(10)
            },
            &Rules::SOLO,
        );
        assert_eq!(plain.ward.generator_after, 137_061);
    }

    /// E8, frag grenade (pierce 6000 / blunt 3000 / fire 1000, area). NPC at
    /// 2,500 mm: falloff 6572, base `mul_bp(48000, 6572) = 31,545`, `D1 =
    /// 33,437`; split 20,063 / 10,031 / 3,343; AR 10: pierce DR 802 → 18,454,
    /// blunt effAR 5 DR 1108 → 8,920, fire 3,343: total 30,717. Allied
    /// Erratum at 3,500 mm in AR 20: falloff 4286, base 20,572, `D1 =
    /// 21,806`, `FF_AREA_BP` 10000: pierce 13,085 (DR 2227) → 10,171, blunt
    /// 6,541 (DR 2765) → 4,733, fire 2,180: total 17,084; with `ff_off`
    /// (5000) `D2 = 10,903` and total 7,104.
    #[test]
    fn e2e_e8_frag_and_friendly_fire() {
        let s = shares(&[
            (DamageType::Pierce, 6_000),
            (DamageType::Blunt, 3_000),
            (DamageType::Fire, 1_000),
        ]);
        let frag = |base: i64| {
            let mut h = Hit::basic(base, s);
            h.weapon_scaling_bp = 600;
            h.area = true;
            h.ff_class = FfClass::Area;
            h.attacker.errata_faction = true;
            h
        };
        let n = resolve_hit(&frag(mul_bp(48_000, 6_572)), &npc(10), &Rules::SOLO);
        assert_eq!(
            (n.d1, n.split[1], n.split[2], n.split[3]),
            (33_437, 20_063, 10_031, 3_343)
        );
        assert_eq!(
            (n.per_type[1], n.per_type[2], n.per_type[3], n.total),
            (18_454, 8_920, 3_343, 30_717)
        );
        let ally = resolve_hit(&frag(mul_bp(48_000, 4_286)), &erratum(20), &Rules::SOLO);
        assert_eq!(
            (ally.d1, ally.chain.friendly_fire, ally.total),
            (21_806, 10_000, 17_084)
        );
        let off = Rules {
            party_size: 2,
            ff_mode: FfMode::Off,
        };
        let ally_off = resolve_hit(&frag(mul_bp(48_000, 4_286)), &erratum(20), &off);
        assert_eq!((ally_off.d2, ally_off.total), (10_903, 7_104));
    }

    /// E9, pistol at 30 m, 3 m above, from the flank: falloff 6250, base
    /// 5,375, `D1 = 5,697`; elevation × 11000 → 6,266; flanking × 11500 →
    /// 7,205; AR 10 with `pen_bp` 1000: effAR `mul_bp(7, 9000) = 6`, DR
    /// `6×10^7/34,820 = 1723` → `7205 - 1241 = 5,964`.
    #[test]
    fn e2e_e9_elevation_and_flanking() {
        let mut hit = Hit::basic(mul_bp(8_600, 6_250), Shares::single(DamageType::Pierce));
        hit.weapon_scaling_bp = 600;
        hit.pen_bp = 1_000;
        hit.geometry = geometry_at(16_384, 3_000, false); // side-on, 3 m higher
        let b = resolve_hit(&hit, &npc(10), &Rules::SOLO);
        assert_eq!(
            (b.d1, b.chain.elevation, b.chain.flanking, b.d2),
            (5_697, 11_000, 11_500, 7_205)
        );
        assert_eq!((b.dr_bp[1], b.total), (1_723, 5_964));
    }

    // ---------------- §10.6 entry points ----------------

    /// A bleeding pulse of 637 mHP `dmg.true` skips resistance and armor; a
    /// 300 mHP shielded pool absorbs 300 (ward_mult 10000), HP takes 337.
    /// A poison pulse of 600 vs toxin resistance 5000 → 300, no chip floor.
    #[test]
    fn entry_dot() {
        let def = Defender {
            resist_bp: resist(&[(DamageType::Toxin, 5_000)]),
            wards: Wards {
                shielded_mhp: 300,
                ..Wards::NONE
            },
            ..npc(500)
        };
        let b = resolve_dot(637, DamageType::True, &def);
        assert_eq!((b.total, b.ward.lost(), b.entry), (337, 300, Entry::Dot));
        assert!(b.flags.dot && per_type_sums(&b));
        let p = resolve_dot(
            600,
            DamageType::Toxin,
            &Defender {
                wards: Wards::NONE,
                ..def
            },
        );
        assert_eq!(
            (
                p.after_resist[DamageType::Toxin.index()],
                p.total,
                p.chip_added
            ),
            (300, 300, 0)
        );
        assert_eq!(p.buildup, [0; TYPE_COUNT]);
    }

    #[test]
    fn entry_fall_skips_armor_and_chip() {
        // Blunt 9000 on AR 1000 with blunt resist 9000 (NPC): 900, no armor, no chip floor.
        let def = Defender {
            resist_bp: resist(&[(DamageType::Blunt, 9_000)]),
            ..npc(1_000)
        };
        let b = resolve_fall(9_000, &def);
        assert_eq!((b.total, b.dr_bp[2], b.chip_added), (900, 0, 0));
    }

    /// Shock chain from E1's 26,761: `D2 = mul_bp(26761, 2000) = 5,352`; NPC
    /// shock resistance 2000 → `5352 - 1070 = 4,282`. Between Errata in
    /// `ff_off` the area factor 5000 gives `D2 = 2,676`.
    #[test]
    fn entry_shock_chain() {
        let def = Defender {
            resist_bp: resist(&[(DamageType::Shock, 2_000)]),
            ..npc(50)
        };
        let b = resolve_shock_chain(26_761, false, &def, &Rules::SOLO);
        assert_eq!((b.d2, b.total, b.chip_added), (5_352, 4_282, 0));
        assert!(b.flags.no_chain && !b.applies_chilled);
        let off = Rules {
            party_size: 2,
            ff_mode: FfMode::Off,
        };
        assert_eq!(
            resolve_shock_chain(26_761, true, &erratum(0), &off).d2,
            2_676
        );
    }

    /// Fire blast on oil (base 10 HP per 1000 ml) at d 5: NPC `× P(5)` =
    /// 14,115; Erratum `× T(5)` = 12,388. Environmental buildup: power 100
    /// at `S(5) = 11200` → 112 burning points.
    #[test]
    fn entry_environmental() {
        let mut power = [0; TYPE_COUNT];
        power[DamageType::Fire.index()] = 100;
        let h = Hazard {
            base_mhp: 10_000,
            shares: Shares::single(DamageType::Fire),
            d: 5,
            instigator_errata: false,
            block: None,
            buildup: Some(HitBuildup {
                power,
                src_lvl: 5,
                attacker_buildup_bp: 10_000,
            }),
        };
        let n = resolve_environmental(&h, &npc(0), &Rules::SOLO);
        assert_eq!(
            (n.d2, n.total, n.buildup[DamageType::Fire.index()]),
            (14_115, 14_115, 112)
        );
        assert_eq!(
            resolve_environmental(&h, &erratum(0), &Rules::SOLO).d2,
            12_388
        );
        // Errata-instigated fire on an Erratum in ff_off: × 5000.
        let off = Rules {
            party_size: 2,
            ff_mode: FfMode::Off,
        };
        assert_eq!(
            resolve_environmental(
                &Hazard {
                    instigator_errata: true,
                    ..h
                },
                &erratum(0),
                &off
            )
            .d2,
            6_194
        );
    }

    #[test]
    fn entry_reflected_uses_armor_and_chip() {
        // 1000 slash reflected onto AR 1000 NPC: DR 8500 → 150; chip floor 50 does not bind.
        let b = resolve_reflected(1_000, &Shares::single(DamageType::Slash), &npc(1_000));
        assert_eq!((b.total, b.entry), (150, Entry::Reflected));
        assert_eq!(b.buildup, [0; TYPE_COUNT]);
    }

    // ---------------- Step 8, lethal, healing ----------------

    #[test]
    fn hp_pool_poise_helpers() {
        assert_eq!(
            apply_hp(10_000, 9_999),
            HpOutcome {
                hp_mhp: 1,
                lethal: false
            }
        );
        assert_eq!(
            apply_hp(10_000, 10_000),
            HpOutcome {
                hp_mhp: 0,
                lethal: true
            }
        );
        assert_eq!(
            apply_hp(10_000, 0),
            HpOutcome {
                hp_mhp: 10_000,
                lethal: false
            }
        );
        assert_eq!(apply_part_pool(7_500, 8_000), 0);
        // Doc 05 §12.5: warhammer light step 1, impact 34 (85% blunt) → mul_bp(34000, 14250) = 48,450.
        assert_eq!(poise_damage_base_mp(34_000, 1, 8_500, false), 48_450);
        assert_eq!(poise_damage_base_mp(34_000, 1, 8_500, true), 0);
    }

    #[test]
    fn same_tick_hits_order_by_source_then_seq() {
        let mut hits = [
            HitKey { source: 7, seq: 0 },
            HitKey { source: 5, seq: 1 },
            HitKey { source: 5, seq: 0 },
        ];
        hits.sort();
        assert_eq!(
            hits,
            [
                HitKey { source: 5, seq: 0 },
                HitKey { source: 5, seq: 1 },
                HitKey { source: 7, seq: 0 }
            ]
        );
    }

    #[test]
    fn lethal_resolution_order() {
        let base = LethalInput {
            is_erratum: true,
            deaths_door_used: false,
            unmoored: false,
            other_member_up: true,
            other_erratum_alive: true,
        };
        let dd = resolve_lethal(base, || true);
        assert_eq!(
            (dd.outcome, dd.hp_mhp, dd.deaths_door_used),
            (LethalOutcome::DeathsDoor, 1_000, true)
        );
        assert_eq!(dd.apply, Some((StatusId::DeathsDoor, Some(60))));
        let down = resolve_lethal(base, || false);
        assert_eq!(
            (down.outcome, down.deaths_door_rolled, down.deaths_door_used),
            (LethalOutcome::Downed, true, true)
        );
        assert!(down.events.player_downed && !down.events.entity_killed);
        // Death's Door already used: no roll at all.
        let used = resolve_lethal(
            LethalInput {
                deaths_door_used: true,
                ..base
            },
            || panic!("must not roll"),
        );
        assert_eq!(
            (used.outcome, used.deaths_door_rolled),
            (LethalOutcome::Downed, false)
        );
        // Other Erratum alive but downed: dies and becomes a ghost.
        let ghost = resolve_lethal(
            LethalInput {
                other_member_up: false,
                ..base
            },
            || false,
        );
        assert_eq!(ghost.outcome, LethalOutcome::Killed);
        assert_eq!(
            ghost.events,
            LethalEvents {
                entity_killed: true,
                player_downed: false,
                player_died: true,
                player_ghosted: true
            }
        );
        assert_eq!(ghost.apply, Some((StatusId::Ghost, None)));
        // Last Erratum standing: dies, no ghost.
        let last = resolve_lethal(
            LethalInput {
                other_member_up: false,
                other_erratum_alive: false,
                ..base
            },
            || false,
        );
        assert_eq!(
            (last.events.player_ghosted, last.events.player_died),
            (false, true)
        );
        // NPCs never roll and go straight to step 3.
        let npc_d = resolve_lethal(
            LethalInput {
                is_erratum: false,
                ..base
            },
            || panic!("NPCs never roll"),
        );
        assert_eq!(
            npc_d.events,
            LethalEvents {
                entity_killed: true,
                ..LethalEvents::default()
            }
        );
        // Unmoored: Death's Door is off and stays unused.
        let um = resolve_lethal(
            LethalInput {
                unmoored: true,
                ..base
            },
            || panic!("off while unmoored"),
        );
        assert_eq!(
            (um.outcome, um.deaths_door_used),
            (LethalOutcome::Downed, false)
        );
    }

    /// Heal 10,000 at row 3 × 10000, Erasure 3 stacks (× 9100) → 9,100, then
    /// withering × 5000 → 4,550; capped by the missing HP.
    #[test]
    fn healing_order_and_cap() {
        let r = heal(50_000, 85_000, 10_000, 10_000, 3, true);
        assert_eq!(
            (r.gross_mhp, r.applied_mhp, r.hp_mhp),
            (4_550, 4_550, 54_550)
        );
        let capped = heal(84_000, 85_000, 10_000, 10_000, 0, false);
        assert_eq!((capped.applied_mhp, capped.hp_mhp), (1_000, 85_000));
        assert!(
            heal(
                10_000,
                85_000,
                heal_amount_mhp(85_000, 1_000),
                10_000,
                0,
                false
            )
            .ends_bleeding
        );
        assert!(
            !heal(
                10_000,
                85_000,
                heal_amount_mhp(85_000, 999),
                10_000,
                0,
                false
            )
            .ends_bleeding
        );
    }

    // ---------------- Properties ----------------

    fn arb_type() -> impl Strategy<Value = DamageType> {
        (0usize..TYPE_COUNT).prop_map(|i| DamageType::ALL[i])
    }

    fn arb_shares() -> impl Strategy<Value = Shares> {
        (arb_type(), arb_type(), 0i32..=10_000).prop_map(|(a, b, x)| {
            if a == b {
                Shares::single(a)
            } else {
                Shares::from_pairs(&[(a, x), (b, 10_000 - x)]).unwrap()
            }
        })
    }

    fn arb_defender() -> impl Strategy<Value = Defender> {
        (
            any::<bool>(),
            0i64..3_000,
            prop::array::uniform15(-10_000i32..=10_000),
            0u16..(1 << 14),
            0i64..60_000,
            0i64..60_000,
            any::<bool>(),
        )
            .prop_map(|(is_erratum, ar, res, imm, sh, gen_w, frozen)| {
                let kind = if is_erratum {
                    TargetKind::Erratum
                } else {
                    TargetKind::Npc
                };
                Defender {
                    ar_part: ar,
                    resist_bp: res,
                    immune: TypeSet(imm),
                    wards: Wards {
                        shielded_mhp: sh,
                        generator_mhp: gen_w,
                        ..Wards::NONE
                    },
                    statuses: DefenderStatuses {
                        frozen,
                        ..DefenderStatuses::default()
                    },
                    ..Defender::new(9, kind)
                }
            })
    }

    proptest! {
        #[test]
        fn prop_breakdown_sums_and_bounds(b in 0i64..5_000_000, s in arb_shares(), pen in 0i32..=10_000, def in arb_defender(), crit: bool) {
            let mut hit = Hit::basic(b, s);
            hit.pen_bp = pen;
            hit.strike = Strike::Normal { crit_mult_bp: crit.then_some(CRIT_MULT_BP) };
            let out = resolve_hit(&hit, &def, &Rules::SOLO);
            prop_assert!(per_type_sums(&out));
            prop_assert!(out.per_type.iter().all(|&v| v >= 0));
            prop_assert!(out.dr_bp.iter().all(|&v| (0..=8_500).contains(&v)));
            prop_assert_eq!(out.split.iter().sum::<i64>(), out.d2);
            prop_assert!(out.ward.shielded_after >= 0 && out.ward.shielded_after <= out.ward.shielded_before);
            prop_assert!(out.ward.generator_after >= 0 && out.ward.generator_after <= out.ward.generator_before);
            for ty in DamageType::ALL {
                let i = ty.index();
                prop_assert!(out.after_armor[i] <= out.after_resist[i], "armor never adds damage");
                if def.is_immune(ty) { prop_assert_eq!(out.per_type[i], 0); }
            }
            // Chip floor guarantee before wards, when the hit carries a non-immune type.
            let carries = DamageType::ALL.iter().any(|&t| s.get(t) > 0 && !def.is_immune(t));
            let floor = if def.kind == TargetKind::Erratum { floor_div(out.d2, 10) } else { floor_div(out.d2, 20) };
            if carries { prop_assert!(out.after_chip.iter().sum::<i64>() >= floor); }
            for e in [resolve_dot(b, s.largest(), &def), resolve_fall(b, &def), resolve_shock_chain(b, false, &def, &Rules::SOLO), resolve_reflected(b, &s, &def)] {
                prop_assert!(per_type_sums(&e));
                prop_assert!(e.per_type.iter().all(|&v| v >= 0));
            }
        }

        #[test]
        fn prop_single_type_monotone_in_b(b in 0i64..2_000_000, db in 0i64..50_000, ty in arb_type(), def in arb_defender(), pen in 0i32..=10_000) {
            let h1 = Hit { pen_bp: pen, ..Hit::basic(b, Shares::single(ty)) };
            let h2 = Hit { pen_bp: pen, ..Hit::basic(b + db, Shares::single(ty)) };
            let a = resolve_hit(&h1, &def, &Rules::SOLO).total;
            let c = resolve_hit(&h2, &def, &Rules::SOLO).total;
            prop_assert!(c >= a, "single-type damage must not drop as B grows: {} -> {}", a, c);
        }

        #[test]
        fn prop_armor_dr_range_and_monotone(ar in 0i64..100_000, d in 0i64..10_000_000, dd in 0i64..100_000) {
            let dr1 = armor_dr_bp(ar, d);
            let dr2 = armor_dr_bp(ar, d + dd);
            prop_assert!((0..=8_500).contains(&dr1) && dr2 <= dr1);
            let after1 = d - mul_bp(d, dr1);
            let after2 = (d + dd) - mul_bp(d + dd, dr2);
            prop_assert!(after2 >= after1);
        }

        #[test]
        fn prop_block_conserves(t in prop::array::uniform15(0i64..200_000), stab in 0i32..=10_000, dcur in 1u32..400) {
            let s = Shares::single(DamageType::Slash);
            let mut d = t;
            let before: i64 = t.iter().sum();
            let out = apply_block(&mut d, &s, &BlockInput { stability_bp: stab, d_current: dcur, stamina_mst: 1_000_000, guard_break: false });
            prop_assert_eq!(d.iter().sum::<i64>() + out.to_stamina_mhp, before);
            // Each type keeps at most its own damage plus the flooring remainder (< 15).
            prop_assert!(d.iter().zip(t.iter()).all(|(a, b)| *a >= 0 && *a <= *b + 15));
        }

        #[test]
        fn prop_split_exact(d2 in 0i64..1_000_000_000, s in arb_shares()) {
            let d = split_by_type(d2, &s);
            prop_assert_eq!(d.iter().sum::<i64>(), d2);
            for ty in DamageType::ALL {
                if s.get(ty) == 0 && ty != s.largest() { prop_assert_eq!(d[ty.index()], 0); }
            }
        }

        #[test]
        fn prop_pipeline_deterministic(b in 0i64..1_000_000, s in arb_shares(), def in arb_defender()) {
            let h = Hit::basic(b, s);
            prop_assert_eq!(resolve_hit(&h, &def, &Rules::SOLO), resolve_hit(&h, &def, &Rules::SOLO));
        }
    }
}
