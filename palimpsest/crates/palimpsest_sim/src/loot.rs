//! SPEC: canon §13.3-13.7 rarity roll, quality, enhancement
//!
//! The rarity table (§13.3) and the generic rarity roll (§13.4), quality
//! multipliers (§13.5), curse states (§13.6), the enhancement factor with
//! blessing and curse penalty, Emendations and the Marginalia cap (§13.7),
//! Palimpsestic effective item level (§13.3), and Marginalia armor HP
//! `gear_hp` (§13.8.2). Pure functions; the roll takes a keyed generator.

use palimpsest_math::int::{clamp_i32, floor_div, floor_div_i128, mul_bp, mul_bp_wide, narrow_i64};
use palimpsest_math::rng::streams;
use palimpsest_math::tables::power_t;
use palimpsest_math::{Rng, keyed_rng};

/// SPEC: canon §13.3 rarities in table order (the roll's index `r`).
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(missing_docs)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Exotic,
    Palimpsestic,
}

/// SPEC: canon §13.3. One rarity row.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct RarityRow {
    /// Content ID, `rar.<name>`.
    pub id: &'static str,
    /// Display name.
    pub name: &'static str,
    /// Color `0xRRGGBB`; `None` for the animated prismatic gradient.
    pub color: Option<u32>,
    /// Number of affixes.
    pub affixes: u8,
    /// Special rule text as written in the canon table.
    pub special: &'static str,
    /// Base weight of the §13.4 roll.
    pub base_weight: i64,
    /// Rarity power in Bp.
    pub power_bp: i32,
    /// Value multiplier (×1 to ×64).
    pub value_mult: i64,
}

/// SPEC: canon §13.3 table, indexed by `Rarity as usize`.
pub static RARITIES: [RarityRow; 7] = [
    RarityRow {
        id: "rar.common",
        name: "Common",
        color: Some(0xE6E6E6),
        affixes: 0,
        special: "none",
        base_weight: 7000,
        power_bp: 10_000,
        value_mult: 1,
    },
    RarityRow {
        id: "rar.uncommon",
        name: "Uncommon",
        color: Some(0x3CC45A),
        affixes: 1,
        special: "none",
        base_weight: 2200,
        power_bp: 10_500,
        value_mult: 2,
    },
    RarityRow {
        id: "rar.rare",
        name: "Rare",
        color: Some(0x3D8BFF),
        affixes: 2,
        special: "none",
        base_weight: 600,
        power_bp: 11_000,
        value_mult: 4,
    },
    RarityRow {
        id: "rar.epic",
        name: "Epic",
        color: Some(0xA64DFF),
        affixes: 3,
        special: "none",
        base_weight: 150,
        power_bp: 11_500,
        value_mult: 8,
    },
    RarityRow {
        id: "rar.legendary",
        name: "Legendary",
        color: Some(0xFF9A1F),
        affixes: 3,
        special: "one legendary perk",
        base_weight: 45,
        power_bp: 12_500,
        value_mult: 16,
    },
    RarityRow {
        id: "rar.exotic",
        name: "Exotic",
        color: Some(0xFFD84A),
        affixes: 2,
        special: "one fixed named intrinsic; at most one Exotic among slot.weapon_1..3 and \
                  slot.offhand, at most one among the five armor slots, and at most one among \
                  every other slot; Palimpsestic items are exempt",
        base_weight: 15,
        power_bp: 12_500,
        value_mult: 32,
    },
    RarityRow {
        id: "rar.palimpsestic",
        name: "Palimpsestic",
        color: None,
        affixes: 4,
        special: "immune to Contradiction; re-skins itself to the local genre; effective ilvl \
                  max(ilvl, current Folio index), re-evaluated on ev.folio_entered",
        base_weight: 0,
        power_bp: 13_000,
        value_mult: 64,
    },
];

impl Rarity {
    /// SPEC: canon §13.3. All rarities in table order.
    pub const ALL: [Rarity; 7] = [
        Rarity::Common,
        Rarity::Uncommon,
        Rarity::Rare,
        Rarity::Epic,
        Rarity::Legendary,
        Rarity::Exotic,
        Rarity::Palimpsestic,
    ];

    /// SPEC: canon §13.3. The table row.
    #[must_use]
    pub const fn row(self) -> &'static RarityRow {
        &RARITIES[self as usize]
    }

    /// SPEC: canon §13.4. The rarity at roll index `r`, if `r <= 6`.
    #[must_use]
    pub const fn from_index(r: usize) -> Option<Rarity> {
        if r < 7 { Some(Rarity::ALL[r]) } else { None }
    }
}

/// SPEC: canon §13.4 weights for a generic drop at Folio depth `d` with the
/// receiving player's Serendipity `ser`: `w[0] = 7000`, and for `r` in 1..=6
/// `w[r] = max(0, floor_div(base[r] × (10000 + (d - 1) × 400 × r + (SER - 40) × 50 × r), 10000))`.
#[must_use]
pub fn rarity_weights(d: u32, ser: i32) -> [u32; 7] {
    let mut w = [0u32; 7];
    w[0] = 7000;
    for (r, slot) in w.iter_mut().enumerate().skip(1) {
        let r_i = r as i128;
        let shift = 10_000 + (i128::from(d) - 1) * 400 * r_i + (i128::from(ser) - 40) * 50 * r_i;
        let v = floor_div_i128(i128::from(RARITIES[r].base_weight) * shift, 10_000);
        *slot = u32::try_from(v.max(0)).expect("invariant: rarity weight fits u32");
    }
    w
}

/// SPEC: canon §13.4 `rarity = rng.weighted(w)`: exactly one `range` draw.
/// The caller passes the keyed `rng.loot` generator ([`loot_rng`]).
#[must_use]
pub fn roll_rarity(rng: &mut Rng, d: u32, ser: i32) -> Rarity {
    let w = rarity_weights(d, ser);
    Rarity::ALL[rng.weighted(&w)]
}

/// SPEC: canon §3.3. The keyed `rng.loot` generator for a drop: `actor` is
/// the killed entity's `EntityId` or the container's uid, `seq` the receiving
/// player's party slot.
#[must_use]
pub fn loot_rng(run_seed: u64, folio: u32, source_id: u64, tick: u64, player_slot: u8) -> Rng {
    keyed_rng(run_seed, streams::LOOT, folio, source_id, tick, u32::from(player_slot))
}

/// SPEC: canon §13.3. Palimpsestic effective ilvl `max(ilvl, current Folio
/// index)`; every other rarity keeps its `ilvl`.
#[must_use]
pub const fn effective_ilvl(rarity: Rarity, ilvl: u32, folio_index: u32) -> u32 {
    if matches!(rarity, Rarity::Palimpsestic) && folio_index > ilvl { folio_index } else { ilvl }
}

/// SPEC: canon §13.3 Exotic limit groups.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ExoticGroup {
    /// `slot.weapon_1..3` and `slot.offhand`.
    WeaponsAndOffhand,
    /// The five armor slots.
    Armor,
    /// Every other slot.
    Other,
}

/// SPEC: canon §13.3. Valid when no group holds more than one equipped
/// Exotic. Pass the group of each equipped Exotic (Palimpsestic items are
/// exempt and never listed).
#[must_use]
pub fn exotic_loadout_valid(exotics: &[ExoticGroup]) -> bool {
    [ExoticGroup::WeaponsAndOffhand, ExoticGroup::Armor, ExoticGroup::Other]
        .iter()
        .all(|g| exotics.iter().filter(|e| *e == g).count() <= 1)
}

// ---------------------------------------------------------------- §13.5

/// SPEC: canon §13.5 quality grades.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(missing_docs)]
pub enum Quality {
    Awful,
    Poor,
    Normal,
    Good,
    Excellent,
    Masterwork,
    Peerless,
}

impl Quality {
    /// SPEC: canon §13.5. All grades in table order.
    pub const ALL: [Quality; 7] = [
        Quality::Awful,
        Quality::Poor,
        Quality::Normal,
        Quality::Good,
        Quality::Excellent,
        Quality::Masterwork,
        Quality::Peerless,
    ];

    /// SPEC: canon §13.5. Content ID, for example `q.normal`.
    #[must_use]
    pub const fn id(self) -> &'static str {
        ["q.awful", "q.poor", "q.normal", "q.good", "q.excellent", "q.masterwork", "q.peerless"][self as usize]
    }

    /// SPEC: canon §13.5 multiplier on an item's base stats.
    #[must_use]
    pub const fn multiplier_bp(self) -> i32 {
        [7000, 8500, 10_000, 11_000, 12_000, 13_500, 15_000][self as usize]
    }
}

/// SPEC: canon §13.5. Quality multiplies an item's base stat.
#[must_use]
pub const fn apply_quality(base: i64, quality: Quality) -> i64 {
    mul_bp(base, quality.multiplier_bp())
}

// ---------------------------------------------------------------- §13.6, §13.7

/// SPEC: canon §13.6 curse states.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Buc {
    /// `buc.blessed`: consumables × 12500; equipment +1 effective enhancement.
    Blessed,
    /// `buc.uncursed`.
    Uncursed,
    /// `buc.cursed`: cannot be unequipped; `curse_penalty` 1..=3; one curse affix.
    Cursed,
}

impl Buc {
    /// SPEC: canon §13.6. Content ID.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Buc::Blessed => "buc.blessed",
            Buc::Uncursed => "buc.uncursed",
            Buc::Cursed => "buc.cursed",
        }
    }
}

/// SPEC: canon §13.6. Blessed consumables apply positive effects × 12500.
pub const BLESSED_CONSUMABLE_BP: i32 = 12_500;
/// SPEC: canon §13.6. Highest `curse_penalty`.
pub const CURSE_PENALTY_MAX: i32 = 3;
/// SPEC: canon §13.7. Stored enhancement cap before Marginalia.
pub const ENHANCEMENT_MAIN_CAP: i32 = 15;
/// SPEC: canon §13.7. Lower clamp of `eff_enh`.
pub const EFF_ENH_MIN: i32 = -3;
/// SPEC: canon §17.1 `MARGINALIA_START`.
pub const MARGINALIA_START: u32 = 26;
/// SPEC: canon §13.7 `EMENDATIONS_PER_VOLUME`.
pub const EMENDATIONS_PER_VOLUME: u32 = 3;
/// SPEC: canon §13.8 `ITEM_MULT_MAX` for Folios 1 to 25, Bp.
pub const ITEM_MULT_MAX_BP: i64 = 51_870;

/// SPEC: canon §13.7. Enhancement cap: 15, and `15 + floor_div(d_current - 25, 5)`
/// in Marginalia (`d_current >= 26`).
#[must_use]
pub const fn enhancement_cap(d_current: u32) -> i32 {
    if d_current >= MARGINALIA_START {
        ENHANCEMENT_MAIN_CAP + floor_div(d_current as i64 - 25, 5) as i32
    } else {
        ENHANCEMENT_MAIN_CAP
    }
}

/// SPEC: canon §13.7 `eff_enh = clamp(enhancement + [buc = blessed] - curse_penalty, -3, 15)`,
/// with the upper bound generalized to `cap` ([`enhancement_cap`]; 15 before
/// Marginalia, per doc 11 Canon gap 2). `curse_penalty` counts only when cursed.
#[must_use]
pub const fn eff_enh(enhancement: i32, buc: Buc, curse_penalty: i32, cap: i32) -> i32 {
    let bless = if matches!(buc, Buc::Blessed) { 1 } else { 0 };
    let curse = if matches!(buc, Buc::Cursed) { curse_penalty } else { 0 };
    clamp_i32(enhancement + bless - curse, EFF_ENH_MIN, cap)
}

/// SPEC: canon §13.7. The enhancement factor `10000 + 600 × eff_enh` Bp that
/// feeds §10.4 and every other stat.
#[must_use]
pub const fn enhancement_factor_bp(eff_enh: i32) -> i32 {
    10_000 + 600 * eff_enh
}

/// SPEC: canon §13.7, §13.9. The item fields an Emendation reads and writes.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EnhanceState {
    /// Item level.
    pub ilvl: u32,
    /// Stored enhancement.
    pub enhancement: i32,
    /// Curse state.
    pub buc: Buc,
    /// `0..=3`, nonzero only when cursed.
    pub curse_penalty: i32,
    /// Carries the curse affix (doc 10).
    pub curse_affix: bool,
}

/// SPEC: canon §13.7. Result of using an Emendation.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Emendation {
    /// On a cursed item: uncursed, penalty 0, curse affix removed,
    /// enhancement and ilvl unchanged.
    Uncursed(EnhanceState),
    /// Otherwise: `ilvl = max(ilvl, current Folio index)`, then +1 enhancement.
    Enhanced(EnhanceState),
    /// Refused at the enhancement cap; the item is unchanged (doc 11 §16.1).
    RefusedAtCap,
}

/// SPEC: canon §13.7. Apply one Emendation at Folio `folio_index`
/// (`d_current` sets the cap).
#[must_use]
pub const fn apply_emendation(s: EnhanceState, folio_index: u32, d_current: u32) -> Emendation {
    if matches!(s.buc, Buc::Cursed) {
        return Emendation::Uncursed(EnhanceState {
            buc: Buc::Uncursed,
            curse_penalty: 0,
            curse_affix: false,
            ..s
        });
    }
    if s.enhancement >= enhancement_cap(d_current) {
        return Emendation::RefusedAtCap;
    }
    let ilvl = if folio_index > s.ilvl { folio_index } else { s.ilvl };
    Emendation::Enhanced(EnhanceState { ilvl, enhancement: s.enhancement + 1, ..s })
}

// ---------------------------------------------------------------- §13.8.2

/// SPEC: canon §13.8.2 armor slots with `hp_base`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ArmorSlot {
    /// `slot.head`, `hp_base` 14.
    Head,
    /// `slot.body`, `hp_base` 30.
    Body,
    /// `slot.hands`, `hp_base` 6.
    Hands,
    /// `slot.legs`, `hp_base` 14.
    Legs,
    /// `slot.feet`, `hp_base` 6.
    Feet,
}

impl ArmorSlot {
    /// SPEC: canon §13.2. The five armor slots.
    pub const ALL: [ArmorSlot; 5] = [ArmorSlot::Head, ArmorSlot::Body, ArmorSlot::Hands, ArmorSlot::Legs, ArmorSlot::Feet];

    /// SPEC: canon §13.8.2 `hp_base[slot]` in HP.
    #[must_use]
    pub const fn hp_base(self) -> i64 {
        match self {
            ArmorSlot::Body => 30,
            ArmorSlot::Head | ArmorSlot::Legs => 14,
            ArmorSlot::Hands | ArmorSlot::Feet => 6,
        }
    }
}

/// SPEC: canon §13.8.2 one piece's term
/// `mul_bp(hp_base[slot] × 1000, max(0, T(ilvl) - T(25)))` mHP, floored on its
/// own (for per-piece display). Panics if `ilvl == 0`.
#[must_use]
pub const fn gear_hp_piece_mhp(slot: ArmorSlot, ilvl: u32) -> i64 {
    mul_bp_wide(slot.hp_base() * 1000, gear_hp_delta_bp(ilvl))
}

/// `max(0, T(ilvl) - T(25))` in Bp.
const fn gear_hp_delta_bp(ilvl: u32) -> i64 {
    let delta = power_t(ilvl) - power_t(25);
    if delta > 0 { delta } else { 0 }
}

/// SPEC: canon §13.8.2 `gear_hp_mhp` over equipped armor pieces
/// `(slot, ilvl)`, 0 in the main run, computed as
/// `floor_div(Σ hp_base[slot] × 1000 × max(0, T(ilvl) - T(25)), 10000)` with a
/// single floor. This reproduces the canon's 745,143 mHP (ilvl 60) and §21
/// test 39's 4,538,772 mHP (ilvl 100); flooring each piece separately, as the
/// formula's prose reads, gives 745,141 and 4,538,770 (reported spec issue).
#[must_use]
pub fn gear_hp_mhp(pieces: &[(ArmorSlot, u32)]) -> i64 {
    let sum: i128 = pieces
        .iter()
        .map(|&(s, l)| i128::from(s.hp_base() * 1000) * i128::from(gear_hp_delta_bp(l)))
        .sum();
    narrow_i64(floor_div_i128(sum, 10_000))
}

#[cfg(test)]
mod tests {
    use super::*;
    use palimpsest_math::int::mul_bp_chain;
    use proptest::prelude::*;

    /// SPEC: canon §21 test 20. At `d = 25`, `SER = 40`:
    /// `[7000, 4312, 1752, 582, 217, 87, 0]`; at `d = 1`, `SER = 100`:
    /// `[7000, 2860, 960, 285, 99, 37, 0]`.
    #[test]
    fn canon_t20_rarity_weights() {
        assert_eq!(rarity_weights(25, 40), [7000, 4312, 1752, 582, 217, 87, 0]);
        assert_eq!(rarity_weights(1, 100), [7000, 2860, 960, 285, 99, 37, 0]);
        // Base weights at d = 1, SER = 40.
        assert_eq!(rarity_weights(1, 40), [7000, 2200, 600, 150, 45, 15, 0]);
        // Very low SER floors weights at 0 but Common stays 7000.
        assert_eq!(rarity_weights(1, 1), [7000, 1771, 366, 62, 9, 0, 0]);
    }

    /// SPEC: canon §21 test 39. A full armor set at ilvl 100 gives
    /// 4,538,772 mHP; at ilvl 25 or lower it gives 0. §13.8.2 also gives
    /// 745,143 mHP at ilvl 60.
    #[test]
    fn canon_t39_marginalia_armor_hp() {
        let set = |ilvl: u32| ArmorSlot::ALL.map(|s| (s, ilvl));
        assert_eq!(gear_hp_mhp(&set(100)), 4_538_772);
        assert_eq!(gear_hp_mhp(&set(60)), 745_143);
        assert_eq!(gear_hp_mhp(&set(25)), 0);
        assert_eq!(gear_hp_mhp(&set(1)), 0);
        assert_eq!(gear_hp_mhp(&[]), 0);
        assert_eq!(ArmorSlot::ALL.iter().map(|s| s.hp_base()).sum::<i64>(), 70);
        // Per-piece floors (the prose reading) fall 2 mHP short of the canon numbers.
        let per_piece = |ilvl: u32| ArmorSlot::ALL.iter().map(|&s| gear_hp_piece_mhp(s, ilvl)).sum::<i64>();
        assert_eq!(per_piece(100), 4_538_770);
        assert_eq!(per_piece(60), 745_141);
        // Mixed levels: a body at ilvl 100 alone.
        assert_eq!(gear_hp_mhp(&[(ArmorSlot::Body, 100), (ArmorSlot::Head, 20)]), gear_hp_piece_mhp(ArmorSlot::Body, 100));
    }

    /// SPEC: canon §13.4: one `weighted` call, exactly one `range` draw, and a
    /// keyed generator gives the same rarity for the same key.
    #[test]
    fn roll_rarity_draws_once() {
        let key = (0x0123_4567_89AB_CDEF, 7, (7u64 << 40) | 12, 4000, 2u8);
        let mut a = loot_rng(key.0, key.1, key.2, key.3, key.4);
        let mut b = a.clone();
        let r = roll_rarity(&mut a, 25, 40);
        let total: u32 = rarity_weights(25, 40).iter().sum();
        let _ = b.range(u64::from(total));
        assert_eq!(a, b);
        let mut c = loot_rng(key.0, key.1, key.2, key.3, key.4);
        assert_eq!(roll_rarity(&mut c, 25, 40), r);
        assert_ne!(r, Rarity::Palimpsestic);
    }

    /// SPEC: canon §13.3 table and §13.5 quality multipliers.
    #[test]
    fn rarity_and_quality_tables() {
        assert_eq!(Rarity::Exotic.row().base_weight, 15);
        assert_eq!(Rarity::Palimpsestic.row().power_bp, 13_000);
        assert_eq!(Rarity::Palimpsestic.row().color, None);
        assert_eq!(Rarity::Legendary.row().value_mult, 16);
        assert_eq!(Rarity::from_index(6), Some(Rarity::Palimpsestic));
        assert_eq!(Rarity::from_index(7), None);
        for (i, r) in Rarity::ALL.into_iter().enumerate() {
            assert!(r.row().id.starts_with("rar."));
            assert_eq!(r.row().value_mult, 1 << i);
        }
        let q: Vec<i32> = Quality::ALL.iter().map(|q| q.multiplier_bp()).collect();
        assert_eq!(q, [7000, 8500, 10_000, 11_000, 12_000, 13_500, 15_000]);
        assert_eq!(apply_quality(20_000, Quality::Masterwork), 27_000);
        assert_eq!(Quality::Peerless.id(), "q.peerless");
    }

    /// SPEC: canon §13.8 `ITEM_MULT_MAX = 51870`: best rarity power (13000),
    /// quality (15000), enhancement (19000 = +15) and material (14000).
    #[test]
    fn item_mult_max() {
        let best_enh = enhancement_factor_bp(eff_enh(15, Buc::Uncursed, 0, 15));
        assert_eq!(best_enh, 19_000);
        let m = mul_bp_chain(Rarity::Palimpsestic.row().power_bp.into(), &[Quality::Peerless.multiplier_bp(), best_enh, 14_000]);
        assert_eq!(m, ITEM_MULT_MAX_BP);
    }

    /// SPEC: canon §13.7 `eff_enh` with blessing and curse penalty.
    #[test]
    fn effective_enhancement() {
        assert_eq!(eff_enh(5, Buc::Uncursed, 0, 15), 5);
        assert_eq!(eff_enh(5, Buc::Blessed, 0, 15), 6);
        assert_eq!(eff_enh(15, Buc::Blessed, 0, 15), 15);
        assert_eq!(eff_enh(0, Buc::Cursed, 3, 15), -3);
        assert_eq!(eff_enh(1, Buc::Cursed, 2, 15), -1);
        assert_eq!(eff_enh(5, Buc::Uncursed, 3, 15), 5); // penalty only while cursed
        assert_eq!(enhancement_factor_bp(-3), 8200);
        assert_eq!(enhancement_factor_bp(0), 10_000);
        // Marginalia cap lets a blessed +16 item reach 17 at d = 35.
        assert_eq!(eff_enh(16, Buc::Blessed, 0, enhancement_cap(35)), 17);
    }

    /// SPEC: canon §13.7 Marginalia cap `15 + floor_div(d_current - 25, 5)`.
    #[test]
    fn marginalia_cap() {
        assert_eq!(enhancement_cap(1), 15);
        assert_eq!(enhancement_cap(25), 15);
        assert_eq!(enhancement_cap(26), 15);
        assert_eq!(enhancement_cap(29), 15);
        assert_eq!(enhancement_cap(30), 16);
        assert_eq!(enhancement_cap(35), 17);
        assert_eq!(enhancement_cap(125), 35);
    }

    /// SPEC: canon §13.7 Emendation rules: uncurse branch, re-level then +1,
    /// refusal at the cap.
    #[test]
    fn emendations() {
        let base = EnhanceState { ilvl: 3, enhancement: 2, buc: Buc::Uncursed, curse_penalty: 0, curse_affix: false };
        let up = apply_emendation(base, 9, 9);
        assert_eq!(up, Emendation::Enhanced(EnhanceState { ilvl: 9, enhancement: 3, ..base }));
        // Never lowers ilvl.
        let high = EnhanceState { ilvl: 12, ..base };
        assert_eq!(apply_emendation(high, 9, 9), Emendation::Enhanced(EnhanceState { enhancement: 3, ..high }));
        let cursed = EnhanceState { buc: Buc::Cursed, curse_penalty: 2, curse_affix: true, ..base };
        assert_eq!(
            apply_emendation(cursed, 9, 9),
            Emendation::Uncursed(EnhanceState { buc: Buc::Uncursed, curse_penalty: 0, curse_affix: false, ..base })
        );
        let capped = EnhanceState { enhancement: 15, ..base };
        assert_eq!(apply_emendation(capped, 25, 25), Emendation::RefusedAtCap);
        assert_eq!(
            apply_emendation(capped, 30, 30),
            Emendation::Enhanced(EnhanceState { ilvl: 30, enhancement: 16, ..capped })
        );
        let blessed = EnhanceState { buc: Buc::Blessed, ..base };
        assert!(matches!(apply_emendation(blessed, 1, 1), Emendation::Enhanced(s) if s.buc == Buc::Blessed));
        assert_eq!(EMENDATIONS_PER_VOLUME * 5, 15);
    }

    /// SPEC: canon §13.3 Palimpsestic effective ilvl and the Exotic limit.
    #[test]
    fn palimpsestic_and_exotic() {
        assert_eq!(effective_ilvl(Rarity::Palimpsestic, 4, 11), 11);
        assert_eq!(effective_ilvl(Rarity::Palimpsestic, 14, 11), 14);
        assert_eq!(effective_ilvl(Rarity::Exotic, 4, 11), 4);
        use ExoticGroup as E;
        assert!(exotic_loadout_valid(&[]));
        assert!(exotic_loadout_valid(&[E::WeaponsAndOffhand, E::Armor, E::Other]));
        assert!(!exotic_loadout_valid(&[E::Armor, E::Armor]));
        assert!(!exotic_loadout_valid(&[E::Other, E::WeaponsAndOffhand, E::Other]));
    }

    proptest! {
        /// SPEC: canon §13.4: Common is fixed at 7000; every other weight
        /// never falls as depth or Serendipity rises; Palimpsestic is 0.
        #[test]
        fn weights_monotone(d in 1u32..500, ser in 1i32..150) {
            let w = rarity_weights(d, ser);
            let wd = rarity_weights(d + 1, ser);
            let ws = rarity_weights(d, ser + 1);
            prop_assert_eq!(w[0], 7000);
            prop_assert_eq!(w[6], 0);
            for r in 1..7 {
                prop_assert!(wd[r] >= w[r] && ws[r] >= w[r]);
            }
        }

        /// SPEC: canon §13.7: eff_enh stays in -3..=cap and the factor is
        /// monotone in it; an Emendation never lowers ilvl or enhancement.
        #[test]
        fn enhancement_invariants(enh in 0i32..=40, pen in 0i32..=3, b in 0usize..3, d in 1u32..300, f in 1u32..300, ilvl in 1u32..300) {
            let buc = [Buc::Blessed, Buc::Uncursed, Buc::Cursed][b];
            let cap = enhancement_cap(d);
            let e = eff_enh(enh, buc, pen, cap);
            prop_assert!((EFF_ENH_MIN..=cap).contains(&e));
            prop_assert!(enhancement_factor_bp(e + 1) > enhancement_factor_bp(e));
            let s = EnhanceState { ilvl, enhancement: enh, buc, curse_penalty: pen, curse_affix: buc == Buc::Cursed };
            match apply_emendation(s, f, d) {
                Emendation::Enhanced(n) => {
                    prop_assert!(n.ilvl >= s.ilvl && n.ilvl >= f && n.enhancement == enh + 1 && n.enhancement <= cap);
                }
                Emendation::Uncursed(n) => {
                    prop_assert_eq!((n.ilvl, n.enhancement, n.buc, n.curse_penalty), (ilvl, enh, Buc::Uncursed, 0));
                }
                Emendation::RefusedAtCap => prop_assert!(enh >= cap),
            }
        }

        /// SPEC: canon §13.8.2: gear_hp is 0 through ilvl 25, never falls as
        /// ilvl rises, and the single-floor total is within 4 mHP above the
        /// sum of per-piece floors.
        #[test]
        fn gear_hp_monotone(ilvls in prop::array::uniform5(1u32..400), slot in 0usize..5) {
            let s = ArmorSlot::ALL[slot];
            let a = gear_hp_piece_mhp(s, ilvls[0]);
            prop_assert!(gear_hp_piece_mhp(s, ilvls[0] + 1) >= a);
            if ilvls[0] <= 25 { prop_assert_eq!(a, 0); }
            let set: Vec<(ArmorSlot, u32)> = ArmorSlot::ALL.iter().copied().zip(ilvls).collect();
            let floors: i64 = set.iter().map(|&(s, l)| gear_hp_piece_mhp(s, l)).sum();
            let total = gear_hp_mhp(&set);
            prop_assert!(total >= floors && total - floors <= 4);
        }
    }
}
