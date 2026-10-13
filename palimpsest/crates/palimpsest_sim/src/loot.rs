//! SPEC: canon §13.1-13.8 rarity roll, quality, type hiding, enhancement
//!
//! The rarity table (§13.3) and the generic rarity roll (§13.4), quality
//! multipliers (§13.5), item categories (§13.1), unknown-appearance classes,
//! type hiding and knowledge states, curse states (§13.6), the enhancement factor with
//! blessing and curse penalty, Emendations and `ENH_CAP` (§13.7),
//! Palimpsestic effective item level and the Exotic limit over the §13.2
//! equipment slots (§13.3), the `slot.ward` capacity chain and recharge
//! (§13.2), and Marginalia armor HP `gear_hp` (§13.8.2). Pure functions; the
//! roll takes a keyed generator.

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
    keyed_rng(
        run_seed,
        streams::LOOT,
        folio,
        source_id,
        tick,
        u32::from(player_slot),
    )
}

/// SPEC: canon §13.3. Palimpsestic effective ilvl `max(ilvl, current Folio
/// index)`; every other rarity keeps its `ilvl`.
#[must_use]
pub const fn effective_ilvl(rarity: Rarity, ilvl: u32, folio_index: u32) -> u32 {
    if matches!(rarity, Rarity::Palimpsestic) && folio_index > ilvl {
        folio_index
    } else {
        ilvl
    }
}

/// SPEC: canon §13.3 Exotic limit groups.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ExoticGroup {
    /// `slot.weapon_1..3`, `slot.offhand` and (canon-1.11)
    /// `slot.hand_extra_1..2`.
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
    [
        ExoticGroup::WeaponsAndOffhand,
        ExoticGroup::Armor,
        ExoticGroup::Other,
    ]
    .iter()
    .all(|g| exotics.iter().filter(|e| *e == g).count() <= 1)
}

/// SPEC: canon §13.2 equipment slots, in table order.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(missing_docs)]
pub enum EquipSlot {
    Head,
    Body,
    Hands,
    Legs,
    Feet,
    Back,
    Neck,
    RingL,
    RingR,
    Belt,
    Trinket,
    Weapon1,
    Weapon2,
    Weapon3,
    Offhand,
    ImplantHead,
    ImplantTorso,
    ImplantArms,
    ImplantLegs,
    Ward,
    Face,
    HandExtra1,
    HandExtra2,
}

impl EquipSlot {
    /// SPEC: canon §13.2. Every slot in table order (23; canon-1.11 names
    /// `slot.hand_extra_1` and `slot.hand_extra_2`).
    pub const ALL: [EquipSlot; 23] = [
        EquipSlot::Head,
        EquipSlot::Body,
        EquipSlot::Hands,
        EquipSlot::Legs,
        EquipSlot::Feet,
        EquipSlot::Back,
        EquipSlot::Neck,
        EquipSlot::RingL,
        EquipSlot::RingR,
        EquipSlot::Belt,
        EquipSlot::Trinket,
        EquipSlot::Weapon1,
        EquipSlot::Weapon2,
        EquipSlot::Weapon3,
        EquipSlot::Offhand,
        EquipSlot::ImplantHead,
        EquipSlot::ImplantTorso,
        EquipSlot::ImplantArms,
        EquipSlot::ImplantLegs,
        EquipSlot::Ward,
        EquipSlot::Face,
        EquipSlot::HandExtra1,
        EquipSlot::HandExtra2,
    ];

    /// SPEC: canon §13.2. The slot's content ID, for example `slot.ward`.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            EquipSlot::Head => "slot.head",
            EquipSlot::Body => "slot.body",
            EquipSlot::Hands => "slot.hands",
            EquipSlot::Legs => "slot.legs",
            EquipSlot::Feet => "slot.feet",
            EquipSlot::Back => "slot.back",
            EquipSlot::Neck => "slot.neck",
            EquipSlot::RingL => "slot.ring_l",
            EquipSlot::RingR => "slot.ring_r",
            EquipSlot::Belt => "slot.belt",
            EquipSlot::Trinket => "slot.trinket",
            EquipSlot::Weapon1 => "slot.weapon_1",
            EquipSlot::Weapon2 => "slot.weapon_2",
            EquipSlot::Weapon3 => "slot.weapon_3",
            EquipSlot::Offhand => "slot.offhand",
            EquipSlot::ImplantHead => "slot.implant_head",
            EquipSlot::ImplantTorso => "slot.implant_torso",
            EquipSlot::ImplantArms => "slot.implant_arms",
            EquipSlot::ImplantLegs => "slot.implant_legs",
            EquipSlot::Ward => "slot.ward",
            EquipSlot::Face => "slot.face",
            EquipSlot::HandExtra1 => "slot.hand_extra_1",
            EquipSlot::HandExtra2 => "slot.hand_extra_2",
        }
    }

    /// SPEC: canon §13.3 (canon-1.11) Exotic limit group of the slot: the
    /// weapon slots, `slot.offhand` and `slot.hand_extra_1..2` share one
    /// group, the five armor slots another, and every other slot the third.
    #[must_use]
    pub const fn exotic_group(self) -> ExoticGroup {
        match self {
            EquipSlot::Weapon1
            | EquipSlot::Weapon2
            | EquipSlot::Weapon3
            | EquipSlot::Offhand
            | EquipSlot::HandExtra1
            | EquipSlot::HandExtra2 => ExoticGroup::WeaponsAndOffhand,
            EquipSlot::Head
            | EquipSlot::Body
            | EquipSlot::Hands
            | EquipSlot::Legs
            | EquipSlot::Feet => ExoticGroup::Armor,
            _ => ExoticGroup::Other,
        }
    }
}

/// SPEC: canon §13.2 (canon-1.11) `slot.ward` capacity: `cap_base_mhp → ×
/// T(ilvl) → × rarity power → × quality → × (10000 + 600 × eff_enh) → ×
/// Contradiction potency`, one `mul_bp` per arrow in that order, with no
/// material term. Panics if `ilvl == 0` or `eff_enh < -3`.
#[must_use]
pub const fn ward_capacity_mhp(
    cap_base_mhp: i64,
    ilvl: u32,
    rarity: Rarity,
    quality: Quality,
    eff_enh: i32,
    potency_bp: i32,
) -> i64 {
    assert!(eff_enh >= EFF_ENH_MIN, "eff_enh below -3");
    let c = mul_bp_wide(cap_base_mhp, power_t(ilvl));
    let c = mul_bp(c, rarity.row().power_bp);
    let c = mul_bp(c, quality.multiplier_bp());
    let c = mul_bp(c, enhancement_factor_bp(eff_enh));
    mul_bp(c, potency_bp)
}

/// SPEC: canon §13.2. Ward recharge starts 180 ticks after the bearer last took damage.
pub const WARD_RECHARGE_DELAY_TICKS: u64 = 180;
/// SPEC: canon §13.2. Ward recharge restores 2500 Bp of capacity per second.
pub const WARD_RECHARGE_BP_PER_S: i32 = 2_500;

/// SPEC: canon §13.2. Whether the generator recharges on tick `t`: at least
/// 180 ticks after the bearer last took damage (always, if it never did).
#[must_use]
pub const fn ward_recharging(t: u64, last_damage_tick: Option<u64>) -> bool {
    match last_damage_tick {
        None => true,
        Some(l) => t >= l && t - l >= WARD_RECHARGE_DELAY_TICKS,
    }
}

/// SPEC: canon §13.2, §2.3. The recharge rate `mul_bp(capacity, 2500)` mHP
/// per second, applied through the §2.3 rate accumulator.
#[must_use]
pub const fn ward_recharge_per_s(capacity_mhp: i64) -> i64 {
    mul_bp(capacity_mhp, WARD_RECHARGE_BP_PER_S)
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
        [
            "q.awful",
            "q.poor",
            "q.normal",
            "q.good",
            "q.excellent",
            "q.masterwork",
            "q.peerless",
        ][self as usize]
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
/// SPEC: canon §13.7 `ENH_CAP` in the main run.
pub const ENHANCEMENT_MAIN_CAP: i32 = 15;
/// SPEC: canon §13.7. Lower clamp of `eff_enh`.
pub const EFF_ENH_MIN: i32 = -3;
/// SPEC: canon §17.1 `MARGINALIA_START`.
pub const MARGINALIA_START: u32 = 26;
/// SPEC: canon §13.7 `EMENDATIONS_PER_VOLUME`.
pub const EMENDATIONS_PER_VOLUME: u32 = 3;
/// SPEC: canon §13.8 `ITEM_MULT_MAX` for Folios 1 to 25, Bp.
pub const ITEM_MULT_MAX_BP: i64 = 51_870;

/// SPEC: canon §13.7 (canon-1.11) `ENH_CAP`: 15 in the main run and
/// `15 + floor_div(d_current - 25, 5)` in Marginalia (`d_current >= 26`).
/// Stored `enhancement` is `0..=ENH_CAP` (§13.9).
#[must_use]
pub const fn enhancement_cap(d_current: u32) -> i32 {
    if d_current >= MARGINALIA_START {
        ENHANCEMENT_MAIN_CAP + floor_div(d_current as i64 - 25, 5) as i32
    } else {
        ENHANCEMENT_MAIN_CAP
    }
}

/// SPEC: canon §13.7 (canon-1.11) `eff_enh = clamp(enhancement + [buc =
/// blessed] - curse_penalty, -3, ENH_CAP)`, with `cap` = `ENH_CAP`
/// ([`enhancement_cap`]). `curse_penalty` counts only when cursed.
#[must_use]
pub const fn eff_enh(enhancement: i32, buc: Buc, curse_penalty: i32, cap: i32) -> i32 {
    let bless = if matches!(buc, Buc::Blessed) { 1 } else { 0 };
    let curse = if matches!(buc, Buc::Cursed) {
        curse_penalty
    } else {
        0
    };
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
    /// Stored enhancement, `0..=ENH_CAP` (§13.9).
    pub enhancement: i32,
    /// Curse state.
    pub buc: Buc,
    /// `0..=3`, nonzero only when cursed.
    pub curse_penalty: i32,
    /// Carries the curse affix (doc 10).
    pub curse_affix: bool,
    /// Canon §13.6 (canon-1.9) pact curse: the curse affix is welded to a
    /// matching pact boon (`curse.pact.<name>`, doc 10).
    pub pact_boon: bool,
}

/// SPEC: canon §13.7. Result of using an Emendation.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Emendation {
    /// On a cursed item: uncursed, penalty 0, curse affix (and a pact
    /// item's boon, canon §13.6) removed, enhancement and ilvl unchanged.
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
            pact_boon: false,
            ..s
        });
    }
    if s.enhancement >= enhancement_cap(d_current) {
        return Emendation::RefusedAtCap;
    }
    let ilvl = if folio_index > s.ilvl {
        folio_index
    } else {
        s.ilvl
    };
    Emendation::Enhanced(EnhanceState {
        ilvl,
        enhancement: s.enhancement + 1,
        ..s
    })
}

// ---------------------------------------------------------------- §13.1, §13.6 type hiding

/// SPEC: canon §13.1, §13.6. Kinds of `cat.accessory` item; rings, amulets
/// and trinkets each have their own unknown-appearance class.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(missing_docs)]
pub enum AccessoryKind {
    Ring,
    Amulet,
    Trinket,
    Belt,
    Back,
}

/// SPEC: canon §13.1 item categories (`cat.`), in table order. A
/// `cat.accessory` item also names its [`AccessoryKind`].
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(missing_docs)]
pub enum Category {
    WeaponMelee,
    WeaponRanged,
    Wand,
    Armor,
    Offhand,
    Accessory(AccessoryKind),
    Implant,
    Ammo,
    Potion,
    Text,
    Book,
    Food,
    Throwable,
    Gadget,
    Tool,
    Material,
    Ingredient,
    Container,
    Placeable,
    Key,
    Currency,
    Quest,
    Part,
    WardGen,
    Corpse,
    BodyPart,
}

impl Category {
    /// SPEC: canon §13.1. Every category, with each accessory kind listed
    /// separately (31 entries; `cat.mount` is retired, canon §17.7).
    pub const ALL: [Category; 30] = [
        Category::WeaponMelee,
        Category::WeaponRanged,
        Category::Wand,
        Category::Armor,
        Category::Offhand,
        Category::Accessory(AccessoryKind::Ring),
        Category::Accessory(AccessoryKind::Amulet),
        Category::Accessory(AccessoryKind::Trinket),
        Category::Accessory(AccessoryKind::Belt),
        Category::Accessory(AccessoryKind::Back),
        Category::Implant,
        Category::Ammo,
        Category::Potion,
        Category::Text,
        Category::Book,
        Category::Food,
        Category::Throwable,
        Category::Gadget,
        Category::Tool,
        Category::Material,
        Category::Ingredient,
        Category::Container,
        Category::Placeable,
        Category::Key,
        Category::Currency,
        Category::Quest,
        Category::Part,
        Category::WardGen,
        Category::Corpse,
        Category::BodyPart,
    ];

    /// SPEC: canon §13.1. The category's content ID, for example `cat.wand`.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Category::WeaponMelee => "cat.weapon_melee",
            Category::WeaponRanged => "cat.weapon_ranged",
            Category::Wand => "cat.wand",
            Category::Armor => "cat.armor",
            Category::Offhand => "cat.offhand",
            Category::Accessory(_) => "cat.accessory",
            Category::Implant => "cat.implant",
            Category::Ammo => "cat.ammo",
            Category::Potion => "cat.potion",
            Category::Text => "cat.text",
            Category::Book => "cat.book",
            Category::Food => "cat.food",
            Category::Throwable => "cat.throwable",
            Category::Gadget => "cat.gadget",
            Category::Tool => "cat.tool",
            Category::Material => "cat.material",
            Category::Ingredient => "cat.ingredient",
            Category::Container => "cat.container",
            Category::Placeable => "cat.placeable",
            Category::Key => "cat.key",
            Category::Currency => "cat.currency",
            Category::Quest => "cat.quest",
            Category::Part => "cat.part",
            Category::WardGen => "cat.ward_gen",
            Category::Corpse => "cat.corpse",
            Category::BodyPart => "cat.body_part",
        }
    }
}

/// SPEC: canon §13.6 unknown-appearance classes (`idc.`), in table order.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(missing_docs)]
pub enum AppearanceClass {
    Potion,
    Text,
    Ring,
    Amulet,
    Wand,
    Implant,
    Relic,
}

impl AppearanceClass {
    /// SPEC: canon §13.6. The seven classes in table order.
    pub const ALL: [AppearanceClass; 7] = [
        AppearanceClass::Potion,
        AppearanceClass::Text,
        AppearanceClass::Ring,
        AppearanceClass::Amulet,
        AppearanceClass::Wand,
        AppearanceClass::Implant,
        AppearanceClass::Relic,
    ];

    /// SPEC: canon §13.6 appearance shuffle: the classes "in ascending ID
    /// byte order".
    pub const SHUFFLE_ORDER: [AppearanceClass; 7] = [
        AppearanceClass::Amulet,
        AppearanceClass::Implant,
        AppearanceClass::Potion,
        AppearanceClass::Relic,
        AppearanceClass::Ring,
        AppearanceClass::Text,
        AppearanceClass::Wand,
    ];

    /// SPEC: canon §13.6. The class's content ID, for example `idc.ring`.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            AppearanceClass::Potion => "idc.potion",
            AppearanceClass::Text => "idc.text",
            AppearanceClass::Ring => "idc.ring",
            AppearanceClass::Amulet => "idc.amulet",
            AppearanceClass::Wand => "idc.wand",
            AppearanceClass::Implant => "idc.implant",
            AppearanceClass::Relic => "idc.relic",
        }
    }
}

/// SPEC: canon §13.6 the `idc` class that lists an item type: every
/// `cat.potion`, `cat.text`, `cat.wand` and `cat.implant` type, and the ring,
/// amulet and trinket types of `cat.accessory`. `None` for every other item.
#[must_use]
pub const fn appearance_class(cat: Category) -> Option<AppearanceClass> {
    match cat {
        Category::Potion => Some(AppearanceClass::Potion),
        Category::Text => Some(AppearanceClass::Text),
        Category::Accessory(AccessoryKind::Ring) => Some(AppearanceClass::Ring),
        Category::Accessory(AccessoryKind::Amulet) => Some(AppearanceClass::Amulet),
        Category::Wand => Some(AppearanceClass::Wand),
        Category::Implant => Some(AppearanceClass::Implant),
        Category::Accessory(AccessoryKind::Trinket) => Some(AppearanceClass::Relic),
        _ => None,
    }
}

/// SPEC: canon §13.6 knowledge states (`know.`), in ascending order of what
/// the player knows.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Knowledge {
    /// `know.unknown`: appearance only.
    Unknown,
    /// `know.classified`: item type.
    Classified,
    /// `know.appraised`: rarity and a value range.
    Appraised,
    /// `know.identified`: everything.
    Identified,
}

impl Knowledge {
    /// SPEC: canon §13.6. Content ID, for example `know.classified`.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Knowledge::Unknown => "know.unknown",
            Knowledge::Classified => "know.classified",
            Knowledge::Appraised => "know.appraised",
            Knowledge::Identified => "know.identified",
        }
    }
}

/// SPEC: canon §13.6. An item type is type-hidden if and only if an `idc`
/// class lists it.
#[must_use]
pub const fn is_type_hidden(cat: Category) -> bool {
    appearance_class(cat).is_some()
}

/// SPEC: canon §13.6. A generated item's starting knowledge: type-hidden
/// items start at `know.unknown`; every other item (staves, belts, weapons,
/// armor, tools, materials, blocks) at `know.classified`, because only its
/// magic is hidden. Type knowledge already learned this run (doc 10) is
/// applied on top by the caller.
#[must_use]
pub const fn initial_knowledge(cat: Category) -> Knowledge {
    if is_type_hidden(cat) {
        Knowledge::Unknown
    } else {
        Knowledge::Classified
    }
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
    pub const ALL: [ArmorSlot; 5] = [
        ArmorSlot::Head,
        ArmorSlot::Body,
        ArmorSlot::Hands,
        ArmorSlot::Legs,
        ArmorSlot::Feet,
    ];

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
        let per_piece = |ilvl: u32| {
            ArmorSlot::ALL
                .iter()
                .map(|&s| gear_hp_piece_mhp(s, ilvl))
                .sum::<i64>()
        };
        assert_eq!(per_piece(100), 4_538_770);
        assert_eq!(per_piece(60), 745_141);
        // Mixed levels: a body at ilvl 100 alone.
        assert_eq!(
            gear_hp_mhp(&[(ArmorSlot::Body, 100), (ArmorSlot::Head, 20)]),
            gear_hp_piece_mhp(ArmorSlot::Body, 100)
        );
    }

    /// SPEC: canon §21 test 37. A generated trinket, ring and wand start at
    /// `know.unknown`; a staff (`cat.weapon_ranged`), a belt and a longsword
    /// (`cat.weapon_melee`) start at `know.classified`.
    #[test]
    fn canon_t37_type_hiding() {
        use AccessoryKind as A;
        for hidden in [
            Category::Accessory(A::Trinket),
            Category::Accessory(A::Ring),
            Category::Wand,
        ] {
            assert_eq!(initial_knowledge(hidden), Knowledge::Unknown, "{hidden:?}");
        }
        for plain in [
            Category::WeaponRanged,
            Category::Accessory(A::Belt),
            Category::WeaponMelee,
        ] {
            assert_eq!(initial_knowledge(plain), Knowledge::Classified, "{plain:?}");
        }
        assert_eq!(Knowledge::Unknown.id(), "know.unknown");
        assert_eq!(Knowledge::Classified.id(), "know.classified");
    }

    /// SPEC: canon §13.6: exactly the seven `idc` classes hide types, each
    /// listed by exactly one category; the shuffle order is ascending ID.
    #[test]
    fn appearance_classes_exhaustive() {
        let hidden: Vec<AppearanceClass> = Category::ALL
            .iter()
            .filter_map(|&c| appearance_class(c))
            .collect();
        assert_eq!(hidden.len(), 7);
        for class in AppearanceClass::ALL {
            assert_eq!(hidden.iter().filter(|&&h| h == class).count(), 1);
            assert!(class.id().starts_with("idc."));
        }
        assert_eq!(
            appearance_class(Category::Accessory(AccessoryKind::Back)),
            None
        );
        assert_eq!(appearance_class(Category::Armor), None);
        assert_eq!(
            appearance_class(Category::Potion),
            Some(AppearanceClass::Potion)
        );
        assert_eq!(
            appearance_class(Category::Accessory(AccessoryKind::Trinket)),
            Some(AppearanceClass::Relic)
        );
        let ids: Vec<&str> = AppearanceClass::SHUFFLE_ORDER
            .iter()
            .map(|c| c.id())
            .collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        assert_eq!(ids, sorted);
        let mut all: Vec<AppearanceClass> = AppearanceClass::SHUFFLE_ORDER.to_vec();
        all.sort();
        assert_eq!(all, AppearanceClass::ALL.to_vec());
        for c in Category::ALL {
            assert!(c.id().starts_with("cat."));
            assert_eq!(
                is_type_hidden(c),
                initial_knowledge(c) == Knowledge::Unknown
            );
        }
        assert!(Knowledge::Unknown < Knowledge::Identified);
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
        let m = mul_bp_chain(
            Rarity::Palimpsestic.row().power_bp.into(),
            &[Quality::Peerless.multiplier_bp(), best_enh, 14_000],
        );
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
        let base = EnhanceState {
            ilvl: 3,
            enhancement: 2,
            buc: Buc::Uncursed,
            curse_penalty: 0,
            curse_affix: false,
            pact_boon: false,
        };
        let up = apply_emendation(base, 9, 9);
        assert_eq!(
            up,
            Emendation::Enhanced(EnhanceState {
                ilvl: 9,
                enhancement: 3,
                ..base
            })
        );
        // Never lowers ilvl.
        let high = EnhanceState { ilvl: 12, ..base };
        assert_eq!(
            apply_emendation(high, 9, 9),
            Emendation::Enhanced(EnhanceState {
                enhancement: 3,
                ..high
            })
        );
        let cursed = EnhanceState {
            buc: Buc::Cursed,
            curse_penalty: 2,
            curse_affix: true,
            ..base
        };
        assert_eq!(
            apply_emendation(cursed, 9, 9),
            Emendation::Uncursed(EnhanceState {
                buc: Buc::Uncursed,
                curse_penalty: 0,
                curse_affix: false,
                ..base
            })
        );
        // Canon §13.6 pact curse: uncursing removes both the curse and the boon.
        let pact = EnhanceState {
            pact_boon: true,
            ..cursed
        };
        assert_eq!(
            apply_emendation(pact, 9, 9),
            Emendation::Uncursed(EnhanceState {
                buc: Buc::Uncursed,
                curse_penalty: 0,
                curse_affix: false,
                pact_boon: false,
                ..base
            })
        );
        let capped = EnhanceState {
            enhancement: 15,
            ..base
        };
        assert_eq!(apply_emendation(capped, 25, 25), Emendation::RefusedAtCap);
        assert_eq!(
            apply_emendation(capped, 30, 30),
            Emendation::Enhanced(EnhanceState {
                ilvl: 30,
                enhancement: 16,
                ..capped
            })
        );
        let blessed = EnhanceState {
            buc: Buc::Blessed,
            ..base
        };
        assert!(
            matches!(apply_emendation(blessed, 1, 1), Emendation::Enhanced(s) if s.buc == Buc::Blessed)
        );
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
        assert!(exotic_loadout_valid(&[
            E::WeaponsAndOffhand,
            E::Armor,
            E::Other
        ]));
        assert!(!exotic_loadout_valid(&[E::Armor, E::Armor]));
        assert!(!exotic_loadout_valid(&[
            E::Other,
            E::WeaponsAndOffhand,
            E::Other
        ]));
        // Canon-1.11: the extra-hand slots share the weapon group, so an
        // Exotic in slot.hand_extra_1 and one in slot.weapon_2 is too many.
        assert_eq!(EquipSlot::HandExtra1.exotic_group(), E::WeaponsAndOffhand);
        assert_eq!(EquipSlot::HandExtra2.exotic_group(), E::WeaponsAndOffhand);
        assert!(!exotic_loadout_valid(&[
            EquipSlot::HandExtra1.exotic_group(),
            EquipSlot::Weapon2.exotic_group()
        ]));
        assert_eq!(EquipSlot::Feet.exotic_group(), E::Armor);
        assert_eq!(EquipSlot::Ward.exotic_group(), E::Other);
        assert_eq!(EquipSlot::ImplantArms.exotic_group(), E::Other);
        for (i, s) in EquipSlot::ALL.iter().enumerate() {
            assert!(s.id().starts_with("slot."));
            assert!(!EquipSlot::ALL[i + 1..].iter().any(|o| o.id() == s.id()));
        }
        assert_eq!(EquipSlot::HandExtra2.id(), "slot.hand_extra_2");
    }

    /// SPEC: canon §13.2 (canon-1.11) `slot.ward` capacity chain, one
    /// `mul_bp` per arrow, no material term; recharge after 180 ticks at 2500
    /// Bp of capacity per second.
    #[test]
    fn ward_capacity_chain() {
        // ilvl 1, Common, Normal, +0, Coherent: the base.
        assert_eq!(
            ward_capacity_mhp(50_000, 1, Rarity::Common, Quality::Normal, 0, 10_000),
            50_000
        );
        // ilvl 25 (T = 36145), Rare (11000), Good (11000), +2 (11200), potency 9000.
        let expect = mul_bp(
            mul_bp(
                mul_bp(mul_bp(mul_bp_wide(50_000, 36_145), 11_000), 11_000),
                11_200,
            ),
            9_000,
        );
        assert_eq!(
            ward_capacity_mhp(50_000, 25, Rarity::Rare, Quality::Good, 2, 9_000),
            expect
        );
        assert_eq!(expect, 220_425);
        // A Disconnected generator has no capacity.
        assert_eq!(
            ward_capacity_mhp(50_000, 25, Rarity::Rare, Quality::Good, 2, 0),
            0
        );
        assert!(!ward_recharging(1_179, Some(1_000)));
        assert!(ward_recharging(1_180, Some(1_000)));
        assert!(ward_recharging(0, None));
        assert_eq!(ward_recharge_per_s(220_425), 55_106);
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
            let s = EnhanceState { ilvl, enhancement: enh, buc, curse_penalty: pen, curse_affix: buc == Buc::Cursed, pact_boon: false };
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

        /// Review (canon §13.2 (canon-1.11)): the ward capacity is one
        /// `mul_bp` per arrow in the listed order (no material term), and
        /// never falls as Contradiction potency rises.
        #[test]
        fn ward_capacity_chain_order(
            base in 0i64..10_000_000, ilvl in 1u32..=400, r in 0usize..7, q in 0usize..7,
            enh in -3i32..=70, pot in 0i32..=10_000,
        ) {
            let (rar, qual) = (Rarity::ALL[r], Quality::ALL[q]);
            let expect = mul_bp(
                mul_bp(
                    mul_bp(mul_bp(mul_bp_wide(base, power_t(ilvl)), rar.row().power_bp), qual.multiplier_bp()),
                    enhancement_factor_bp(enh),
                ),
                pot,
            );
            let cap = ward_capacity_mhp(base, ilvl, rar, qual, enh, pot);
            prop_assert_eq!(cap, expect);
            if pot < 10_000 {
                prop_assert!(ward_capacity_mhp(base, ilvl, rar, qual, enh, pot + 1) >= cap);
            }
        }

        /// Review (canon §13.2, §2.3): recharge starts exactly 180 ticks
        /// after the last damage, and never on a tick before that damage.
        #[test]
        fn ward_recharge_boundary(last in 0u64..1_000_000_000, dt in 0u64..400) {
            prop_assert_eq!(ward_recharging(last + dt, Some(last)), dt >= 180);
            prop_assert!(!ward_recharging(last, Some(last + 1 + dt)));
        }

        /// Review (canon §13.6 (canon-1.9), §13.7): an Emendation on a cursed
        /// pact item removes the boon with the curse and keeps enhancement.
        #[test]
        fn pact_boon_leaves_with_the_curse(ilvl in 1u32..30, enh in 0i32..=15, pen in 1i32..=3, f in 1u32..30) {
            let s = EnhanceState { ilvl, enhancement: enh, buc: Buc::Cursed, curse_penalty: pen, curse_affix: true, pact_boon: true };
            prop_assert_eq!(
                apply_emendation(s, f, f),
                Emendation::Uncursed(EnhanceState { buc: Buc::Uncursed, curse_penalty: 0, curse_affix: false, pact_boon: false, ..s })
            );
        }
    }

    /// Review (canon §13.2, §13.3 (canon-1.11)): `EquipSlot::ALL` is canon
    /// §13.2's slot list in table order, and the Exotic limit groups hold 6
    /// (weapons, off-hand, extra hands), 5 (armor) and 12 slots. The text
    /// check is skipped when the spec is not next to the workspace.
    #[test]
    fn equip_slots_match_canon_text() {
        let count = |g: ExoticGroup| {
            EquipSlot::ALL
                .iter()
                .filter(|s| s.exotic_group() == g)
                .count()
        };
        assert_eq!(
            (
                count(ExoticGroup::WeaponsAndOffhand),
                count(ExoticGroup::Armor),
                count(ExoticGroup::Other)
            ),
            (6, 5, 12)
        );
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../docs/palimpsest/spec/00_CANON.md"
        );
        let Ok(canon) = std::fs::read_to_string(path) else {
            std::eprintln!("skipped: {path} not found");
            return;
        };
        let ids: Vec<String> = canon
            .lines()
            .filter(|l| l.starts_with("| `slot."))
            .flat_map(|l| {
                l.split('|')
                    .nth(1)
                    .unwrap_or_default()
                    .split(',')
                    .map(|t| t.trim().trim_matches('`').to_string())
                    .collect::<Vec<_>>()
            })
            .collect();
        let code: Vec<&str> = EquipSlot::ALL.iter().map(|s| s.id()).collect();
        assert_eq!(ids, code);
    }

    /// Review (canon §13.2): `ilvl` 0 is not an item level, so the chain
    /// refuses it (`T(0)` panics).
    #[test]
    #[should_panic(expected = "d must be >= 1")]
    fn ward_capacity_rejects_ilvl_zero() {
        let _ = ward_capacity_mhp(50_000, 0, Rarity::Common, Quality::Normal, 0, 10_000);
    }
}
