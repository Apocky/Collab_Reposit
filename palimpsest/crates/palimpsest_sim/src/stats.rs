//! SPEC: canon §6 derived stats, §6.5 NPC rows
//!
//! Sub-attributes and primaries (§6.1, §6.2), item requirement shortfall
//! (§6.3), every master derived-stat row 1 to 44 (§6.4) as one pure function
//! each, the §6.5 NPC row selection, and the Death's Door chance and roll.
//! Per-second rates are returned per second; callers feed them to the canon
//! §2.3 accumulator. `Milli` pools are returned already × 1000 (canon §1.5).

use crate::contradiction::{
    LocalGenre, aether_regen_term_bp, verve_crit_term_bp, verve_deaths_door_term_bp, vrv,
};
use crate::genre::Axiom;
use palimpsest_math::int::{ceil_div, clamp_i32, floor_div, floor_div_i32, mul_bp, mul_bp_i32};
use palimpsest_math::keyed_rng;
use palimpsest_math::rng::streams;

/// SPEC: canon §6.1. The 24 sub-attributes, grouped by primary in table order.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(missing_docs)]
pub enum Sub {
    Power,
    Brawn,
    Grip,
    Precision,
    Agility,
    Sleight,
    Toughness,
    Endurance,
    Resilience,
    Logic,
    Memory,
    Analysis,
    Focus,
    Resolve,
    Attunement,
    Sight,
    Hearing,
    Intuition,
    Charm,
    Menace,
    Guile,
    Fate,
    Serendipity,
    Providence,
}

const SUB_IDS: [(&str, &str); 24] = [
    ("sub.power", "PWR"),
    ("sub.brawn", "BRN"),
    ("sub.grip", "GRP"),
    ("sub.precision", "PRC"),
    ("sub.agility", "AGI"),
    ("sub.sleight", "SLT"),
    ("sub.toughness", "TGH"),
    ("sub.endurance", "END"),
    ("sub.resilience", "RES"),
    ("sub.logic", "LOG"),
    ("sub.memory", "MEM"),
    ("sub.analysis", "ANL"),
    ("sub.focus", "FOC"),
    ("sub.resolve", "RSV"),
    ("sub.attunement", "ATN"),
    ("sub.sight", "SGT"),
    ("sub.hearing", "HRG"),
    ("sub.intuition", "ITU"),
    ("sub.charm", "CHM"),
    ("sub.menace", "MEN"),
    ("sub.guile", "GUI"),
    ("sub.fate", "FAT"),
    ("sub.serendipity", "SER"),
    ("sub.providence", "PRV"),
];

impl Sub {
    /// SPEC: canon §6.1. All sub-attributes in table order.
    pub const ALL: [Sub; 24] = [
        Sub::Power,
        Sub::Brawn,
        Sub::Grip,
        Sub::Precision,
        Sub::Agility,
        Sub::Sleight,
        Sub::Toughness,
        Sub::Endurance,
        Sub::Resilience,
        Sub::Logic,
        Sub::Memory,
        Sub::Analysis,
        Sub::Focus,
        Sub::Resolve,
        Sub::Attunement,
        Sub::Sight,
        Sub::Hearing,
        Sub::Intuition,
        Sub::Charm,
        Sub::Menace,
        Sub::Guile,
        Sub::Fate,
        Sub::Serendipity,
        Sub::Providence,
    ];

    /// SPEC: canon §6.1. Index `0..=23` in table order.
    #[must_use]
    pub const fn index(self) -> usize {
        self as usize
    }

    /// SPEC: canon §6.1. Content ID, for example `sub.power`.
    #[must_use]
    pub const fn id(self) -> &'static str {
        SUB_IDS[self.index()].0
    }

    /// SPEC: canon §6.1. Three-letter code, for example `PWR`.
    #[must_use]
    pub const fn code(self) -> &'static str {
        SUB_IDS[self.index()].1
    }

    /// SPEC: canon §6.1. The primary this sub-attribute belongs to.
    #[must_use]
    pub const fn primary(self) -> Primary {
        Primary::ALL[self.index() / 3]
    }
}

/// SPEC: canon §6.1. The eight primary attributes in table order.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(missing_docs)]
pub enum Primary {
    Might,
    Finesse,
    Vigor,
    Intellect,
    Will,
    Perception,
    Presence,
    Fortune,
}

const PRIMARY_IDS: [(&str, &str); 8] = [
    ("attr.might", "MGT"),
    ("attr.finesse", "FIN"),
    ("attr.vigor", "VIG"),
    ("attr.intellect", "INT"),
    ("attr.will", "WIL"),
    ("attr.perception", "PER"),
    ("attr.presence", "PRE"),
    ("attr.fortune", "FOR"),
];

impl Primary {
    /// SPEC: canon §6.1. All primaries in table order.
    pub const ALL: [Primary; 8] = [
        Primary::Might,
        Primary::Finesse,
        Primary::Vigor,
        Primary::Intellect,
        Primary::Will,
        Primary::Perception,
        Primary::Presence,
        Primary::Fortune,
    ];

    /// SPEC: canon §6.1. Index `0..=7`.
    #[must_use]
    pub const fn index(self) -> usize {
        self as usize
    }

    /// SPEC: canon §6.1. Content ID, for example `attr.might`.
    #[must_use]
    pub const fn id(self) -> &'static str {
        PRIMARY_IDS[self.index()].0
    }

    /// SPEC: canon §6.1. Three-letter code, for example `MGT`.
    #[must_use]
    pub const fn code(self) -> &'static str {
        PRIMARY_IDS[self.index()].1
    }

    /// SPEC: canon §6.1. The three sub-attributes of this primary.
    #[must_use]
    pub const fn subs(self) -> [Sub; 3] {
        let i = self.index() * 3;
        [Sub::ALL[i], Sub::ALL[i + 1], Sub::ALL[i + 2]]
    }
}

/// SPEC: canon §6.2. Stored (base) sub-attribute range `1..=100`.
pub const SUB_BASE_MIN: i32 = 1;
/// SPEC: canon §6.2. Upper bound of a stored sub-attribute.
pub const SUB_BASE_MAX: i32 = 100;
/// SPEC: canon §6.2. Effective sub-attribute range `1..=150`.
pub const SUB_EFFECTIVE_MIN: i32 = 1;
/// SPEC: canon §6.2. Upper bound of an effective sub-attribute.
pub const SUB_EFFECTIVE_MAX: i32 = 150;
/// SPEC: canon §6.2. Human baseline.
pub const HUMAN_BASELINE: i32 = 40;

/// SPEC: canon §6.2. Effective value: base plus modifiers, clamped to `1..=150`.
#[must_use]
pub const fn effective_sub(base: i32, modifiers: i32) -> i32 {
    clamp_i32(
        base.saturating_add(modifiers),
        SUB_EFFECTIVE_MIN,
        SUB_EFFECTIVE_MAX,
    )
}

/// SPEC: canon §6.2 `d(x) = x - 40`.
#[must_use]
pub const fn d(x: i32) -> i32 {
    x - HUMAN_BASELINE
}

/// SPEC: canon §6.1, §6.2. Effective sub-attribute values indexed by [`Sub::index`].
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SubAttrs(pub [i32; 24]);

impl SubAttrs {
    /// SPEC: canon §6.2. Every sub-attribute at `v` (40 is the human baseline).
    #[must_use]
    pub const fn uniform(v: i32) -> SubAttrs {
        SubAttrs([v; 24])
    }

    /// SPEC: canon §6.1. The value of `s`.
    #[must_use]
    pub const fn get(&self, s: Sub) -> i32 {
        self.0[s.index()]
    }

    /// SPEC: canon §6.1. Builder: the same set with `s` replaced by `v`.
    #[must_use]
    pub const fn with(mut self, s: Sub, v: i32) -> SubAttrs {
        self.0[s.index()] = v;
        self
    }

    /// SPEC: canon §6.1. A primary is `floor_div(s1 + s2 + s3, 3)` of its
    /// effective sub-attributes.
    #[must_use]
    pub const fn primary(&self, p: Primary) -> i32 {
        let [a, b, c] = p.subs();
        floor_div_i32(self.get(a) + self.get(b) + self.get(c), 3)
    }
}

// ---------------------------------------------------------------- §6.3

/// SPEC: canon §6.3 shortfall `s = max(0, req - value)`.
#[must_use]
pub const fn requirement_shortfall(req: i32, value: i32) -> i32 {
    if req > value { req - value } else { 0 }
}

/// SPEC: canon §6.3. Damage, attack speed and spread recovery while wielding
/// with a shortfall are each × `10000 - min(5000, s × 100)` Bp.
#[must_use]
pub const fn requirement_multiplier_bp(req: i32, value: i32) -> i32 {
    let s = requirement_shortfall(req, value) as i64 * 100;
    10_000 - if s > 5000 { 5000 } else { s as i32 }
}

// ---------------------------------------------------------------- §6.4 rows

/// SPEC: canon §6.4 row 1. Max HP `40 + TGH + L × (3 + floor_div(TGH, 20))`
/// HP, × 1000 to mHP, plus `gear_hp` in mHP (§13.8.2).
#[must_use]
pub const fn max_hp_mhp(tgh: i32, level: u32, gear_hp_mhp: i64) -> i64 {
    let per_level = 3 + floor_div(tgh as i64, 20);
    (40 + tgh as i64 + level as i64 * per_level) * 1000 + gear_hp_mhp
}

/// SPEC: canon §6.4 row 2. Natural HP regeneration after 5 s without damage:
/// `mul_bp(max_hp_mhp, max(10, 50 + floor_div(d(RES), 2)))` mHP per s.
#[must_use]
pub const fn hp_regen_mhp_per_s(max_hp_mhp: i64, res: i32) -> i64 {
    let bp = 50 + floor_div_i32(d(res), 2);
    mul_bp(max_hp_mhp, if bp < 10 { 10 } else { bp })
}

/// SPEC: canon §6.4 row 2. Delay before natural regeneration: 5 s.
pub const HP_REGEN_DELAY_TICKS: u32 = 300;

/// SPEC: canon §6.4 row 3. Healing received × `10000 + d(RES) × 50` Bp.
#[must_use]
pub const fn healing_received_bp(res: i32) -> i32 {
    10_000 + d(res) * 50
}

/// SPEC: canon §6.4 row 4. Max stamina `50 + END + floor_div(L, 2)`, in milli.
#[must_use]
pub const fn max_stamina_milli(end: i32, level: u32) -> i64 {
    (50 + end as i64 + floor_div(level as i64, 2)) * 1000
}

/// SPEC: canon §6.4 row 5. Stamina regeneration `15 + floor_div(END, 5)` per
/// s (milli per s), after 1 s without spending.
#[must_use]
pub const fn stamina_regen_milli_per_s(end: i32) -> i64 {
    (15 + floor_div(end as i64, 5)) * 1000
}

/// SPEC: canon §6.4 row 5. Delay before stamina regeneration: 1 s.
pub const STAMINA_REGEN_DELAY_TICKS: u32 = 60;

/// SPEC: canon §6.4 row 6. Sprint drain `max(4, 12 - floor_div(d(END), 10))`
/// per s (milli per s).
#[must_use]
pub const fn sprint_drain_milli_per_s(end: i32) -> i64 {
    let v = 12 - floor_div(d(end) as i64, 10);
    (if v < 4 { 4 } else { v }) * 1000
}

/// SPEC: canon §6.4 row 7. Max aether `20 + FOC + L`, in milli.
#[must_use]
pub const fn max_aether_milli(foc: i32, level: u32) -> i64 {
    (20 + foc as i64 + level as i64) * 1000
}

/// SPEC: canon §6.4 row 8. Aether regeneration `500 + ATN × 20` m-aether per
/// s, then × the §5.7 AETHER term (`aether_term_bp`).
#[must_use]
pub const fn aether_regen_milli_per_s(atn: i32, aether_term_bp: i32) -> i64 {
    mul_bp(500 + atn as i64 * 20, aether_term_bp)
}

/// SPEC: canon §6.4 row 9. Max composure `50 + RSV`, in milli.
#[must_use]
pub const fn max_composure_milli(rsv: i32) -> i64 {
    (50 + rsv as i64) * 1000
}

/// SPEC: canon §6.4 row 10. Carry capacity `25 + floor_div(PWR, 2)` kg, in grams.
#[must_use]
pub const fn carry_capacity_g(pwr: i32) -> i64 {
    (25 + floor_div(pwr as i64, 2)) * 1000
}

/// SPEC: canon §6.4 row 11. Move speed `clamp(10000 + d(AGI) × 25, 8000, 12500)` Bp.
#[must_use]
pub const fn move_speed_bp(agi: i32) -> i32 {
    clamp_i32(10_000 + d(agi) * 25, 8000, 12_500)
}

/// SPEC: canon §6.4 row 12. One scaling coefficient `c_i` (Bp, `0..=10000`)
/// on the effective sub-attribute value `x_i`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ScalingTerm {
    /// `c_i` in Bp.
    pub coef_bp: i32,
    /// Effective value of the scaled sub-attribute.
    pub value: i32,
}

/// SPEC: canon §6.4 row 12. Weapon scaling bonus
/// `max(-5000, Σ floor_div(c_i × d(x_i), 100) + governing_skill × 30)` Bp.
/// Spells never use this row.
#[must_use]
pub fn weapon_scaling_bp(terms: &[ScalingTerm], governing_skill: i32) -> i32 {
    let sum: i64 = terms
        .iter()
        .map(|t| floor_div(t.coef_bp as i64 * d(t.value) as i64, 100))
        .sum();
    let v = sum + governing_skill as i64 * 30;
    (if v < -5000 { -5000 } else { v }) as i32
}

/// SPEC: canon §6.4 row 12 schema rule: each `c_i ∈ 0..=10000` and `Σ c_i ≤ 15000`.
#[must_use]
pub fn scaling_coeffs_valid(coefs_bp: &[i32]) -> bool {
    coefs_bp.iter().all(|c| (0..=10_000).contains(c))
        && coefs_bp.iter().map(|&c| c as i64).sum::<i64>() <= 15_000
}

/// SPEC: canon §6.4 row 13. Crit chance: exactly 0 when
/// `local_axiom(e.pos, verve) = 0`, otherwise
/// `clamp(500 + d(FAT) × 10 + d(PRC) × 5 + (vrv(e) - 5) × 100 + gear, 0, 7500)` Bp.
/// NPCs pass `gear_bp = 0` (§6.5).
#[must_use]
pub const fn crit_chance_bp(fat: i32, prc: i32, local_verve: u8, vrv: u8, gear_bp: i32) -> i32 {
    if local_verve == 0 {
        return 0;
    }
    let v = 500 + d(fat) * 10 + d(prc) * 5 + verve_crit_term_bp(vrv) + gear_bp;
    clamp_i32(v, 0, 7500)
}

/// SPEC: canon §6.4 row 14. Crit damage multiplier `15000 + gear` Bp.
#[must_use]
pub const fn crit_damage_bp(gear_bp: i32) -> i32 {
    15_000 + gear_bp
}

/// SPEC: canon §6.5. NPC crit multiplier.
pub const NPC_CRIT_DAMAGE_BP: i32 = 15_000;

/// SPEC: canon §6.4 row 15. Spread `clamp(10000 - d(PRC) × 40, 5000, 14000)` Bp.
#[must_use]
pub const fn spread_bp(prc: i32) -> i32 {
    clamp_i32(10_000 - d(prc) * 40, 5000, 14_000)
}

/// SPEC: canon §6.4 row 16. Recoil `clamp(10000 - d(GRP) × 40, 5000, 14000)` Bp.
#[must_use]
pub const fn recoil_bp(grp: i32) -> i32 {
    clamp_i32(10_000 - d(grp) * 40, 5000, 14_000)
}

/// SPEC: canon §6.4 row 17. Reload, swap and use-item speed
/// `clamp(10000 + d(SLT) × 50, 7000, 15000)` Bp.
#[must_use]
pub const fn action_speed_bp(slt: i32) -> i32 {
    clamp_i32(10_000 + d(slt) * 50, 7000, 15_000)
}

/// SPEC: canon §6.4 row 17 `action_ticks = max(1, ceil_div(base_ticks × 10000, mult))`.
#[must_use]
pub const fn action_ticks(base_ticks: u32, mult_bp: i32) -> u32 {
    let t = ceil_div(base_ticks as i64 * 10_000, mult_bp as i64);
    if t < 1 { 1 } else { t as u32 }
}

/// SPEC: canon §6.4 row 18. Poise `20 + floor_div(BRN, 2)` plus armor poise,
/// in milli (poise is a `Milli` resource, canon §1.2).
#[must_use]
pub const fn poise_milli(brn: i32, armor_poise_milli: i64) -> i64 {
    (20 + floor_div(brn as i64, 2)) * 1000 + armor_poise_milli
}

/// SPEC: canon §6.4 row 19. Block stability
/// `clamp(5000 + d(BRN) × 50 + shield stability, 0, 10000)` Bp.
#[must_use]
pub const fn block_stability_bp(brn: i32, shield_stability_bp: i32) -> i32 {
    clamp_i32(5000 + d(brn) * 50 + shield_stability_bp, 0, 10_000)
}

/// SPEC: canon §6.4 row 20. Throw range `10000 + d(PWR) × 50` Bp.
#[must_use]
pub const fn throw_range_bp(pwr: i32) -> i32 {
    10_000 + d(pwr) * 50
}

/// SPEC: canon §6.4 row 21. Climb speed `10000 + d(GRP) × 50` Bp.
#[must_use]
pub const fn climb_speed_bp(grp: i32) -> i32 {
    10_000 + d(grp) * 50
}

/// SPEC: canon §6.4 rows 22-25. Status threshold `max(40, 100 + 2 × d(x))`,
/// with `x` = RES (body), TGH (nerve), RSV (mind) or ATN (arcane).
#[must_use]
pub const fn status_threshold(x: i32) -> i32 {
    let v = 100 + 2 * d(x);
    if v < 40 { 40 } else { v }
}

/// SPEC: canon §6.4 rows 22-25. Which sub-attribute sets each status group's threshold.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum StatusGroup {
    /// Row 22, RES: poisoned, diseased, irradiated, withering, corroded, exposed.
    Body,
    /// Row 23, TGH: bleeding, burning, frozen, shocked, stunned, blinded, deafened.
    Nerve,
    /// Row 24, RSV: feared, charmed, confused, sleeping, dread, berserk, taunted, hacked.
    Mind,
    /// Row 25, ATN: silenced, unraveled.
    Arcane,
}

impl StatusGroup {
    /// SPEC: canon §6.4 rows 22-25. The governing sub-attribute.
    #[must_use]
    pub const fn governing(self) -> Sub {
        match self {
            StatusGroup::Body => Sub::Resilience,
            StatusGroup::Nerve => Sub::Toughness,
            StatusGroup::Mind => Sub::Resolve,
            StatusGroup::Arcane => Sub::Attunement,
        }
    }

    /// SPEC: canon §6.4 rows 22-25. The threshold for `subs`.
    #[must_use]
    pub const fn threshold(self, subs: &SubAttrs) -> i32 {
        status_threshold(subs.get(self.governing()))
    }
}

/// SPEC: canon §6.4 row 26. Spell complexity budget `4 + floor_div(LOG, 10)` points.
#[must_use]
pub const fn spell_complexity_budget(log: i32) -> i32 {
    4 + floor_div_i32(log, 10)
}

/// SPEC: canon §6.4 row 27. Grimoire slots `2 + floor_div(MEM, 20)`.
#[must_use]
pub const fn grimoire_slots(mem: i32) -> i32 {
    2 + floor_div_i32(mem, 20)
}

/// SPEC: canon §6.4 row 28. Skill XP multiplier `10000 + d(MEM) × 50` Bp.
#[must_use]
pub const fn skill_xp_bp(mem: i32) -> i32 {
    10_000 + d(mem) * 50
}

/// SPEC: canon §6.4 row 29, before the GLYPH term:
/// `10000 + d(ATN) × 50 + school_skill × 30` Bp.
#[must_use]
pub const fn spell_potency_base_bp(atn: i32, school_skill: i32) -> i32 {
    10_000 + d(atn) * 50 + school_skill * 30
}

/// SPEC: canon §6.4 row 29 (canon-1.11). Spell potency for one glyph: the
/// base row × that glyph's §5.7 GLYPH term (`contradiction::glyph_term_bp`);
/// the row applies the GLYPH term for each glyph of the stage that produces
/// the effect (doc 06), so the caller chains one term per glyph of that stage.
#[must_use]
pub const fn spell_potency_bp(atn: i32, school_skill: i32, glyph_term_bp: i32) -> i32 {
    mul_bp_i32(spell_potency_base_bp(atn, school_skill), glyph_term_bp)
}

/// SPEC: canon §6.4 row 30. Cast interruption resistance (chance to keep
/// casting when hit) `clamp(2500 + d(FOC) × 75, 0, 9500)` Bp, keyed `rng.magic`.
#[must_use]
pub const fn cast_keep_bp(foc: i32) -> i32 {
    clamp_i32(2500 + d(foc) * 75, 0, 9500)
}

/// SPEC: canon §6.4 row 31. Dark-vision radius `2 + floor_div(SGT, 20)` m.
#[must_use]
pub const fn dark_vision_m(sgt: i32) -> i32 {
    2 + floor_div_i32(sgt, 20)
}

/// SPEC: canon §6.4 row 32. Visual detection range bonus `d(SGT) × 50` Bp.
#[must_use]
pub const fn visual_detection_bonus_bp(sgt: i32) -> i32 {
    d(sgt) * 50
}

/// SPEC: canon §6.4 row 33. Hearing sensitivity bonus `floor_div(d(HRG), 5)` dB.
#[must_use]
pub const fn hearing_bonus_db(hrg: i32) -> i32 {
    floor_div_i32(d(hrg), 5)
}

/// SPEC: canon §6.4 row 34. Footstep indicator radius `8 + floor_div(HRG, 10)` m.
#[must_use]
pub const fn footstep_radius_m(hrg: i32) -> i32 {
    8 + floor_div_i32(hrg, 10)
}

/// SPEC: canon §6.4 row 35. Passive trap and secret detection per second
/// within 3 m: `clamp(500 + d(ITU) × 50 + skill.awareness × 50, 0, 9000)` Bp.
#[must_use]
pub const fn trap_detection_bp(itu: i32, awareness: i32) -> i32 {
    clamp_i32(500 + d(itu) * 50 + awareness * 50, 0, 9000)
}

/// SPEC: canon §6.4 row 36. Curse sense on pickup `clamp(1000 + d(ITU) × 100, 0, 9000)` Bp.
#[must_use]
pub const fn curse_sense_bp(itu: i32) -> i32 {
    clamp_i32(1000 + d(itu) * 100, 0, 9000)
}

/// SPEC: canon §6.4 row 37. Buy price `clamp(10000 - d(CHM) × 30, 7000, 13000)` Bp.
#[must_use]
pub const fn buy_price_bp(chm: i32) -> i32 {
    clamp_i32(10_000 - d(chm) * 30, 7000, 13_000)
}

/// SPEC: canon §6.4 row 38. Sell share `clamp(3000 + d(CHM) × 30, 1500, 5000)` Bp.
#[must_use]
pub const fn sell_share_bp(chm: i32) -> i32 {
    clamp_i32(3000 + d(chm) * 30, 1500, 5000)
}

/// SPEC: canon §6.4 row 39. Threat generation `10000 + d(MEN) × 100` Bp.
#[must_use]
pub const fn threat_bp(men: i32) -> i32 {
    10_000 + d(men) * 100
}

/// SPEC: canon §6.4 row 40. Disguise suspicion rate
/// `clamp(10000 - d(GUI) × 100, 3000, 15000)` Bp.
#[must_use]
pub const fn disguise_bp(gui: i32) -> i32 {
    clamp_i32(10_000 - d(gui) * 100, 3000, 15_000)
}

/// SPEC: canon §6.4 row 41. Status buildup multiplier `10000 + d(FAT) × 50` Bp.
#[must_use]
pub const fn buildup_bp(fat: i32) -> i32 {
    10_000 + d(fat) * 50
}

/// SPEC: canon §6.4 row 42. Loot rarity shift `d(SER)`, which feeds §13.4.
#[must_use]
pub const fn loot_rarity_shift(ser: i32) -> i32 {
    d(ser)
}

/// SPEC: canon §6.4 row 43. Death's Door chance (Errata only, once per Folio):
/// `min(6000, mul_bp(clamp(1000 + d(PRV) × 50, 0, 4000), 10000 + (vrv(e) - 5) × 1000))` Bp.
#[must_use]
pub const fn deaths_door_bp(prv: i32, vrv: u8) -> i32 {
    let base = clamp_i32(1000 + d(prv) * 50, 0, 4000);
    let v = mul_bp_i32(base, verve_deaths_door_term_bp(vrv));
    if v > 6000 { 6000 } else { v }
}

/// SPEC: canon §6.4 row 44. Curse resistance on equip `max(0, d(PRV) × 50)` Bp.
#[must_use]
pub const fn curse_resistance_bp(prv: i32) -> i32 {
    let v = d(prv) * 50;
    if v < 0 { 0 } else { v }
}

// ---------------------------------------------------------------- Death's Door

/// SPEC: canon §1.5, §10.8. Death's Door leaves HP at 1,000 mHP.
pub const DEATHS_DOOR_HP_MHP: i64 = 1000;

/// SPEC: canon §3.3 (canon-1.11), doc 04 §12.1. Keyed `seq` reserved for the
/// Death's Door roll: `0xFFFF` on `rng.combat`.
pub const DEATHS_DOOR_SEQ: u32 = palimpsest_math::rng::seq::COMBAT_DEATHS_DOOR;

/// SPEC: canon §6.4 row 43, §6.5, §5.11. Whether a Death's Door roll is made:
/// Errata only, once per Folio, and never while `st.unmoored` lasts.
#[must_use]
pub const fn deaths_door_can_roll(is_erratum: bool, unmoored: bool, used_this_folio: bool) -> bool {
    is_erratum && !unmoored && !used_this_folio
}

/// SPEC: canon §3.3 (`rng.combat` draws Death's Door), doc 04 §12.1: one
/// `roll_bp` on `keyed_rng("rng.combat", folio, erratum_id, tick, 0xFFFF)`.
#[must_use]
pub fn roll_deaths_door(
    run_seed: u64,
    folio: u32,
    erratum_id: u64,
    tick: u64,
    chance_bp: i32,
) -> bool {
    keyed_rng(
        run_seed,
        streams::COMBAT,
        folio,
        erratum_id,
        tick,
        DEATHS_DOOR_SEQ,
    )
    .roll_bp(chance_bp)
}

// ---------------------------------------------------------------- §6.5

/// SPEC: canon §6.4. Number of master derived-stat rows.
pub const ROW_COUNT: u8 = 44;

/// SPEC: canon §6.5. The rows NPCs use (row 13 without its gear term).
pub const NPC_ROWS: [u8; 18] = [
    11, 13, 15, 16, 17, 18, 22, 23, 24, 25, 30, 31, 32, 33, 34, 35, 39, 41,
];

/// SPEC: canon §6.5. True when NPCs use §6.4 row `row`; NPCs never use rows
/// 1–10, 12, 14, 19–21, 26–29, 36–38, 40 or 42–44.
#[must_use]
pub const fn npc_uses_row(row: u8) -> bool {
    let mut i = 0;
    while i < NPC_ROWS.len() {
        if NPC_ROWS[i] == row {
            return true;
        }
        i += 1;
    }
    false
}

/// SPEC: canon §6.4. Gear and armor terms of the rows that carry them.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct GearTerms {
    /// Row 1 `gear_hp` (§13.8.2), mHP.
    pub hp_mhp: i64,
    /// Row 13 `gear` (and flat status bonuses such as `st.blessed`), Bp.
    pub crit_bp: i32,
    /// Row 14 `gear`, Bp.
    pub crit_damage_bp: i32,
    /// Row 18 armor poise, milli.
    pub armor_poise_milli: i64,
    /// Row 19 shield stability, Bp.
    pub shield_stability_bp: i32,
}

/// SPEC: canon §6.4. Inputs of an Erratum's derived stats.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ErratumInputs {
    /// Effective sub-attributes.
    pub subs: SubAttrs,
    /// Character level `L`.
    pub level: u32,
    /// Local genre at the Erratum's position (§5.8).
    pub local: LocalGenre,
    /// Has `st.inspired` (personal VERVE +2).
    pub inspired: bool,
    /// Gear and armor terms.
    pub gear: GearTerms,
    /// `skill.awareness` (row 35).
    pub awareness: i32,
}

/// SPEC: canon §6.4. An Erratum's derived stats (rows 12 and 29 depend on the
/// weapon or spell and stay functions).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub struct ErratumStats {
    pub max_hp_mhp: i64,
    pub hp_regen_mhp_per_s: i64,
    pub healing_received_bp: i32,
    pub max_stamina_milli: i64,
    pub stamina_regen_milli_per_s: i64,
    pub sprint_drain_milli_per_s: i64,
    pub max_aether_milli: i64,
    pub aether_regen_milli_per_s: i64,
    pub max_composure_milli: i64,
    pub carry_capacity_g: i64,
    pub move_speed_bp: i32,
    pub crit_chance_bp: i32,
    pub crit_damage_bp: i32,
    pub spread_bp: i32,
    pub recoil_bp: i32,
    pub action_speed_bp: i32,
    pub poise_milli: i64,
    pub block_stability_bp: i32,
    pub throw_range_bp: i32,
    pub climb_speed_bp: i32,
    /// Rows 22, 23, 24, 25: body, nerve, mind, arcane.
    pub thresholds: [i32; 4],
    pub spell_complexity_budget: i32,
    pub grimoire_slots: i32,
    pub skill_xp_bp: i32,
    pub cast_keep_bp: i32,
    pub dark_vision_m: i32,
    pub visual_detection_bonus_bp: i32,
    pub hearing_bonus_db: i32,
    pub footstep_radius_m: i32,
    pub trap_detection_bp: i32,
    pub curse_sense_bp: i32,
    pub buy_price_bp: i32,
    pub sell_share_bp: i32,
    pub threat_bp: i32,
    pub disguise_bp: i32,
    pub buildup_bp: i32,
    pub loot_rarity_shift: i32,
    pub deaths_door_bp: i32,
    pub curse_resistance_bp: i32,
}

const fn thresholds(s: &SubAttrs) -> [i32; 4] {
    [
        StatusGroup::Body.threshold(s),
        StatusGroup::Nerve.threshold(s),
        StatusGroup::Mind.threshold(s),
        StatusGroup::Arcane.threshold(s),
    ]
}

/// SPEC: canon §6.4 rows 1-11, 13-28, 30-44 for an Erratum.
#[must_use]
pub const fn derive_erratum(i: &ErratumInputs) -> ErratumStats {
    let s = &i.subs;
    let g = &i.gear;
    let v = vrv(i.local, i.inspired);
    let max_hp = max_hp_mhp(s.get(Sub::Toughness), i.level, g.hp_mhp);
    ErratumStats {
        max_hp_mhp: max_hp,
        hp_regen_mhp_per_s: hp_regen_mhp_per_s(max_hp, s.get(Sub::Resilience)),
        healing_received_bp: healing_received_bp(s.get(Sub::Resilience)),
        max_stamina_milli: max_stamina_milli(s.get(Sub::Endurance), i.level),
        stamina_regen_milli_per_s: stamina_regen_milli_per_s(s.get(Sub::Endurance)),
        sprint_drain_milli_per_s: sprint_drain_milli_per_s(s.get(Sub::Endurance)),
        max_aether_milli: max_aether_milli(s.get(Sub::Focus), i.level),
        aether_regen_milli_per_s: aether_regen_milli_per_s(
            s.get(Sub::Attunement),
            aether_regen_term_bp(i.local),
        ),
        max_composure_milli: max_composure_milli(s.get(Sub::Resolve)),
        carry_capacity_g: carry_capacity_g(s.get(Sub::Power)),
        move_speed_bp: move_speed_bp(s.get(Sub::Agility)),
        crit_chance_bp: crit_chance_bp(
            s.get(Sub::Fate),
            s.get(Sub::Precision),
            i.local.axiom(Axiom::Verve),
            v,
            g.crit_bp,
        ),
        crit_damage_bp: crit_damage_bp(g.crit_damage_bp),
        spread_bp: spread_bp(s.get(Sub::Precision)),
        recoil_bp: recoil_bp(s.get(Sub::Grip)),
        action_speed_bp: action_speed_bp(s.get(Sub::Sleight)),
        poise_milli: poise_milli(s.get(Sub::Brawn), g.armor_poise_milli),
        block_stability_bp: block_stability_bp(s.get(Sub::Brawn), g.shield_stability_bp),
        throw_range_bp: throw_range_bp(s.get(Sub::Power)),
        climb_speed_bp: climb_speed_bp(s.get(Sub::Grip)),
        thresholds: thresholds(s),
        spell_complexity_budget: spell_complexity_budget(s.get(Sub::Logic)),
        grimoire_slots: grimoire_slots(s.get(Sub::Memory)),
        skill_xp_bp: skill_xp_bp(s.get(Sub::Memory)),
        cast_keep_bp: cast_keep_bp(s.get(Sub::Focus)),
        dark_vision_m: dark_vision_m(s.get(Sub::Sight)),
        visual_detection_bonus_bp: visual_detection_bonus_bp(s.get(Sub::Sight)),
        hearing_bonus_db: hearing_bonus_db(s.get(Sub::Hearing)),
        footstep_radius_m: footstep_radius_m(s.get(Sub::Hearing)),
        trap_detection_bp: trap_detection_bp(s.get(Sub::Intuition), i.awareness),
        curse_sense_bp: curse_sense_bp(s.get(Sub::Intuition)),
        buy_price_bp: buy_price_bp(s.get(Sub::Charm)),
        sell_share_bp: sell_share_bp(s.get(Sub::Charm)),
        threat_bp: threat_bp(s.get(Sub::Menace)),
        disguise_bp: disguise_bp(s.get(Sub::Guile)),
        buildup_bp: buildup_bp(s.get(Sub::Fate)),
        loot_rarity_shift: loot_rarity_shift(s.get(Sub::Serendipity)),
        deaths_door_bp: deaths_door_bp(s.get(Sub::Providence), v),
        curse_resistance_bp: curse_resistance_bp(s.get(Sub::Providence)),
    }
}

/// SPEC: canon §6.5. Inputs of an NPC's derived stats (effective
/// sub-attributes per §13.8).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct NpcInputs {
    /// Effective sub-attributes.
    pub subs: SubAttrs,
    /// Local genre at the NPC's position.
    pub local: LocalGenre,
    /// Has `st.inspired`.
    pub inspired: bool,
    /// Row 18 armor poise, milli.
    pub armor_poise_milli: i64,
    /// `skill.awareness` (row 35), from the NPC template.
    pub awareness: i32,
}

/// SPEC: canon §6.5. The 18 rows NPCs use, and nothing else.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub struct NpcStats {
    pub move_speed_bp: i32,
    pub crit_chance_bp: i32,
    pub spread_bp: i32,
    pub recoil_bp: i32,
    pub action_speed_bp: i32,
    pub poise_milli: i64,
    /// Rows 22, 23, 24, 25: body, nerve, mind, arcane.
    pub thresholds: [i32; 4],
    pub cast_keep_bp: i32,
    pub dark_vision_m: i32,
    pub visual_detection_bonus_bp: i32,
    pub hearing_bonus_db: i32,
    pub footstep_radius_m: i32,
    pub trap_detection_bp: i32,
    pub threat_bp: i32,
    pub buildup_bp: i32,
}

/// SPEC: canon §6.5 rows 11, 13 (no gear term), 15-18, 22-25, 30-35, 39, 41.
#[must_use]
pub const fn derive_npc(i: &NpcInputs) -> NpcStats {
    let s = &i.subs;
    let v = vrv(i.local, i.inspired);
    NpcStats {
        move_speed_bp: move_speed_bp(s.get(Sub::Agility)),
        crit_chance_bp: crit_chance_bp(
            s.get(Sub::Fate),
            s.get(Sub::Precision),
            i.local.axiom(Axiom::Verve),
            v,
            0,
        ),
        spread_bp: spread_bp(s.get(Sub::Precision)),
        recoil_bp: recoil_bp(s.get(Sub::Grip)),
        action_speed_bp: action_speed_bp(s.get(Sub::Sleight)),
        poise_milli: poise_milli(s.get(Sub::Brawn), i.armor_poise_milli),
        thresholds: thresholds(s),
        cast_keep_bp: cast_keep_bp(s.get(Sub::Focus)),
        dark_vision_m: dark_vision_m(s.get(Sub::Sight)),
        visual_detection_bonus_bp: visual_detection_bonus_bp(s.get(Sub::Sight)),
        hearing_bonus_db: hearing_bonus_db(s.get(Sub::Hearing)),
        footstep_radius_m: footstep_radius_m(s.get(Sub::Hearing)),
        trap_detection_bp: trap_detection_bp(s.get(Sub::Intuition), i.awareness),
        threat_bp: threat_bp(s.get(Sub::Menace)),
        buildup_bp: buildup_bp(s.get(Sub::Fate)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contradiction::glyph_term_bp;
    use crate::genre::Genre;
    use proptest::prelude::*;

    fn baseline(local: LocalGenre) -> ErratumInputs {
        ErratumInputs {
            subs: SubAttrs::uniform(40),
            level: 1,
            local,
            inspired: false,
            gear: GearTerms::default(),
            awareness: 0,
        }
    }

    /// SPEC: canon §21 test 13. All sub-attributes 40, level 1, VERVE 5.
    #[test]
    fn canon_t13_derived_stats() {
        let st = derive_erratum(&baseline(LocalGenre::Plain(Genre::UrText)));
        assert_eq!(st.max_hp_mhp, 85_000);
        assert_eq!(st.hp_regen_mhp_per_s, 425);
        assert_eq!(st.max_stamina_milli, 90_000);
        assert_eq!(st.max_aether_milli, 61_000);
        assert_eq!(st.max_composure_milli, 90_000);
        assert_eq!(st.carry_capacity_g, 45_000);
        assert_eq!(st.crit_chance_bp, 500);
        assert_eq!(st.grimoire_slots, 4);
        assert_eq!(st.spell_complexity_budget, 8);
    }

    /// SPEC: canon §6.4 last column ("At all-40, L1, VERVE 5") for every row.
    #[test]
    fn all_rows_at_baseline() {
        let st = derive_erratum(&baseline(LocalGenre::Plain(Genre::UrText)));
        assert_eq!(st.healing_received_bp, 10_000);
        assert_eq!(st.stamina_regen_milli_per_s, 23_000);
        assert_eq!(st.sprint_drain_milli_per_s, 12_000);
        assert_eq!(st.aether_regen_milli_per_s, 1300); // 1.3 per s at aether axiom 5 / none
        assert_eq!(st.move_speed_bp, 10_000);
        assert_eq!(
            weapon_scaling_bp(
                &[ScalingTerm {
                    coef_bp: 10_000,
                    value: 40
                }],
                0
            ),
            0
        );
        assert_eq!(st.crit_damage_bp, 15_000);
        assert_eq!(
            (st.spread_bp, st.recoil_bp, st.action_speed_bp),
            (10_000, 10_000, 10_000)
        );
        assert_eq!(st.poise_milli, 40_000);
        assert_eq!(st.block_stability_bp, 5000);
        assert_eq!((st.throw_range_bp, st.climb_speed_bp), (10_000, 10_000));
        assert_eq!(st.thresholds, [100; 4]);
        assert_eq!(st.skill_xp_bp, 10_000);
        assert_eq!(spell_potency_bp(40, 0, 10_000), 10_000);
        assert_eq!(st.cast_keep_bp, 2500);
        assert_eq!(st.dark_vision_m, 4);
        assert_eq!(st.visual_detection_bonus_bp, 0);
        assert_eq!(st.hearing_bonus_db, 0);
        assert_eq!(st.footstep_radius_m, 12);
        assert_eq!(st.trap_detection_bp, 500);
        assert_eq!(st.curse_sense_bp, 1000);
        assert_eq!((st.buy_price_bp, st.sell_share_bp), (10_000, 3000));
        assert_eq!(
            (st.threat_bp, st.disguise_bp, st.buildup_bp),
            (10_000, 10_000, 10_000)
        );
        assert_eq!(st.loot_rarity_shift, 0);
        assert_eq!(st.deaths_door_bp, 1000);
        assert_eq!(st.curse_resistance_bp, 0);
    }

    /// SPEC: canon §21 test 25. FAT 100, PRC 100 and `st.blessed` (+1000 Bp)
    /// in `genre.redacted` give crit chance 0, even with `st.inspired`.
    #[test]
    fn canon_t25_zero_verve_crit() {
        let mut i = baseline(LocalGenre::Plain(Genre::Redacted));
        i.subs = i.subs.with(Sub::Fate, 100).with(Sub::Precision, 100);
        i.gear.crit_bp = 1000;
        assert_eq!(derive_erratum(&i).crit_chance_bp, 0);
        i.inspired = true;
        assert_eq!(derive_erratum(&i).crit_chance_bp, 0);
        assert_eq!(crit_chance_bp(100, 100, 0, 2, 1000), 0);
        // The same character in Cyberpunk caps at 7500? 500 + 600 + 300 + 0 + 1000 = 2400.
        i.local = LocalGenre::Plain(Genre::Cyberpunk);
        i.inspired = false;
        assert_eq!(derive_erratum(&i).crit_chance_bp, 2400);
        let npc = NpcInputs {
            subs: i.subs,
            local: LocalGenre::Plain(Genre::Redacted),
            inspired: false,
            armor_poise_milli: 0,
            awareness: 0,
        };
        assert_eq!(derive_npc(&npc).crit_chance_bp, 0);
    }

    /// SPEC: canon §21 test 26. PRV 100 at VERVE 10 gives 6000 Bp, PRV 40 at
    /// VERVE 5 gives 1000 Bp, and NPCs never roll it.
    #[test]
    fn canon_t26_deaths_door() {
        assert_eq!(deaths_door_bp(100, 10), 6000);
        assert_eq!(deaths_door_bp(40, 5), 1000);
        let mut i = baseline(LocalGenre::Plain(Genre::Wuxia));
        i.subs = i.subs.with(Sub::Providence, 100);
        assert_eq!(derive_erratum(&i).deaths_door_bp, 6000);
        assert!(!npc_uses_row(43));
        assert!(!deaths_door_can_roll(false, false, false));
        assert!(deaths_door_can_roll(true, false, false));
        assert!(!deaths_door_can_roll(true, true, false));
        assert!(!deaths_door_can_roll(true, false, true));
        // Doc 04 §14.1: PRV 37 at VERVE 5 uses 850 Bp.
        assert_eq!(deaths_door_bp(37, 5), 850);
        assert_eq!(deaths_door_bp(1, 0), 0);
    }

    /// SPEC: canon §21 test 35. Every sub-attribute 40 in `genre.cyberpunk`
    /// (aether axiom techne, TEC 9) regenerates 1,820 m-aether per second.
    #[test]
    fn canon_t35_genre_native_aether() {
        let st = derive_erratum(&baseline(LocalGenre::Plain(Genre::Cyberpunk)));
        assert_eq!(st.aether_regen_milli_per_s, 1820);
        assert_eq!(aether_regen_milli_per_s(40, 14_000), 1820);
    }

    /// Cross-check against doc 04 §14.1 / acceptance test 3 (the human
    /// Cyberpunk Netrunner at level 1, VERVE 5).
    #[test]
    fn doc04_creation_example() {
        let subs = SubAttrs::uniform(40)
            .with(Sub::Sleight, 47)
            .with(Sub::Logic, 50)
            .with(Sub::Guile, 42)
            .with(Sub::Sight, 42)
            .with(Sub::Resilience, 37)
            .with(Sub::Providence, 37)
            .with(Sub::Charm, 38)
            .with(Sub::Endurance, 38)
            .with(Sub::Agility, 44)
            .with(Sub::Hearing, 43)
            .with(Sub::Menace, 36);
        let i = ErratumInputs {
            subs,
            awareness: 10,
            ..baseline(LocalGenre::Plain(Genre::Cyberpunk))
        };
        let st = derive_erratum(&i);
        assert_eq!(st.max_hp_mhp, 85_000);
        assert_eq!(st.hp_regen_mhp_per_s, 408);
        assert_eq!(st.healing_received_bp, 9850);
        assert_eq!(st.max_stamina_milli, 88_000);
        assert_eq!(st.stamina_regen_milli_per_s, 22_000);
        assert_eq!(st.sprint_drain_milli_per_s, 13_000);
        assert_eq!(st.max_aether_milli, 61_000);
        assert_eq!(st.max_composure_milli, 90_000);
        assert_eq!(st.carry_capacity_g, 45_000);
        assert_eq!(st.move_speed_bp, 10_100);
        assert_eq!(st.crit_chance_bp, 500);
        assert_eq!(st.action_speed_bp, 10_350);
        assert_eq!(st.spell_complexity_budget, 9);
        assert_eq!(st.grimoire_slots, 4);
        assert_eq!(st.dark_vision_m, 4);
        assert_eq!(st.footstep_radius_m, 12);
        assert_eq!(st.thresholds[0], 94);
        assert_eq!((st.buy_price_bp, st.sell_share_bp), (10_060, 2940));
        assert_eq!((st.threat_bp, st.disguise_bp), (9600, 9800));
        assert_eq!(st.deaths_door_bp, 850);
        assert_eq!(st.trap_detection_bp, 1000);
        // Primaries: MGT 40, FIN 43, VIG 38, INT 43, WIL 40, PER 41, PRE 38, FOR 39.
        let p: Vec<i32> = Primary::ALL.iter().map(|&p| subs.primary(p)).collect();
        assert_eq!(p, [40, 43, 38, 43, 40, 41, 38, 39]);
    }

    /// SPEC: canon §6.3 requirement shortfall.
    #[test]
    fn requirement_shortfall_rule() {
        assert_eq!(requirement_shortfall(55, 60), 0);
        assert_eq!(requirement_multiplier_bp(55, 60), 10_000);
        assert_eq!(requirement_shortfall(55, 40), 15);
        assert_eq!(requirement_multiplier_bp(55, 40), 8500);
        assert_eq!(requirement_multiplier_bp(100, 40), 5000);
        assert_eq!(requirement_multiplier_bp(i32::MAX, 1), 5000);
    }

    /// SPEC: canon §6.4 clamps at both ends of the effective range.
    #[test]
    fn row_clamps() {
        // The lower clamp of row 11 is unreachable inside 1..=150 (9025 at AGI 1).
        assert_eq!((move_speed_bp(1), move_speed_bp(150)), (9025, 12_500));
        assert_eq!(move_speed_bp(-200), 8000);
        assert_eq!((spread_bp(150), spread_bp(1)), (5600, 11_560));
        assert_eq!(spread_bp(200), 5000);
        assert_eq!(recoil_bp(150), 5600);
        assert_eq!((action_speed_bp(1), action_speed_bp(150)), (8050, 15_000));
        assert_eq!(block_stability_bp(150, 0), 10_000);
        assert_eq!(block_stability_bp(1, -5000), 0);
        assert_eq!(status_threshold(1), 40);
        assert_eq!(status_threshold(150), 320);
        assert_eq!(cast_keep_bp(150), 9500);
        assert_eq!(cast_keep_bp(1), 0);
        assert_eq!(trap_detection_bp(150, 100), 9000);
        assert_eq!(curse_sense_bp(150), 9000);
        assert_eq!((buy_price_bp(150), buy_price_bp(1)), (7000, 11_170));
        assert_eq!((sell_share_bp(150), sell_share_bp(1)), (5000, 1830));
        assert_eq!((disguise_bp(150), disguise_bp(1)), (3000, 13_900));
        assert_eq!(curse_resistance_bp(1), 0);
        assert_eq!(curse_resistance_bp(100), 3000);
        assert_eq!(hp_regen_mhp_per_s(100_000, 1), mul_bp(100_000, 30));
        assert_eq!(sprint_drain_milli_per_s(150), 4000);
        assert_eq!(sprint_drain_milli_per_s(1), 16_000); // 12 - floor_div(-39, 10)
        assert_eq!(hearing_bonus_db(36), -1);
    }

    /// SPEC: canon §6.4 row 12 and its schema rule.
    #[test]
    fn weapon_scaling() {
        let terms = [
            ScalingTerm {
                coef_bp: 10_000,
                value: 60,
            },
            ScalingTerm {
                coef_bp: 5000,
                value: 50,
            },
        ];
        assert_eq!(weapon_scaling_bp(&terms, 20), 2000 + 500 + 600);
        let weak = [
            ScalingTerm {
                coef_bp: 10_000,
                value: 1,
            },
            ScalingTerm {
                coef_bp: 5000,
                value: 1,
            },
        ];
        assert_eq!(weapon_scaling_bp(&weak, 0), -5000);
        // floor_div of a negative product rounds toward minus infinity.
        assert_eq!(
            weapon_scaling_bp(
                &[ScalingTerm {
                    coef_bp: 333,
                    value: 39
                }],
                0
            ),
            -4
        );
        assert!(scaling_coeffs_valid(&[10_000, 5000]));
        assert!(!scaling_coeffs_valid(&[10_000, 5001]));
        assert!(!scaling_coeffs_valid(&[10_001]));
        assert!(!scaling_coeffs_valid(&[-1]));
    }

    /// SPEC: canon §6.4 rows 17 and 29.
    #[test]
    fn action_ticks_and_spell_potency() {
        assert_eq!(action_ticks(60, 10_000), 60);
        assert_eq!(action_ticks(60, 15_000), 40);
        assert_eq!(action_ticks(60, 7000), 86); // ceil(85.71)
        assert_eq!(action_ticks(0, 10_000), 1);
        let cp = LocalGenre::Plain(Genre::Cyberpunk);
        let term = glyph_term_bp(Some(Axiom::Techne), cp);
        assert_eq!(spell_potency_base_bp(60, 50), 10_000 + 1000 + 1500);
        assert_eq!(spell_potency_bp(60, 50, term), mul_bp_i32(12_500, 10_800));
    }

    /// SPEC: canon §6.5. NPC rows and Errata-only rows partition 1..=44.
    #[test]
    fn npc_row_partition() {
        let never: [u8; 26] = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 14, 19, 20, 21, 26, 27, 28, 29, 36, 37, 38, 40, 42,
            43, 44,
        ];
        for row in 1..=ROW_COUNT {
            assert_ne!(npc_uses_row(row), never.contains(&row), "row {row}");
        }
        assert!(!npc_uses_row(0) && !npc_uses_row(45));
        let npc = derive_npc(&NpcInputs {
            subs: SubAttrs::uniform(40),
            local: LocalGenre::Plain(Genre::Cyberpunk),
            inspired: false,
            armor_poise_milli: 5000,
            awareness: 0,
        });
        assert_eq!(npc.crit_chance_bp, 500);
        assert_eq!(npc.poise_milli, 45_000);
        assert_eq!(npc.thresholds, [100; 4]);
        assert_eq!(NPC_CRIT_DAMAGE_BP, 15_000);
    }

    /// SPEC: canon §6.1, §6.2 metadata, primaries and effective clamp.
    #[test]
    fn sub_metadata() {
        for (i, s) in Sub::ALL.into_iter().enumerate() {
            assert_eq!(s.index(), i);
            assert!(s.id().starts_with("sub."));
            assert_eq!(s.code().len(), 3);
            assert!(s.primary().subs().contains(&s));
        }
        assert_eq!(
            Primary::Will.subs(),
            [Sub::Focus, Sub::Resolve, Sub::Attunement]
        );
        assert_eq!(Sub::Attunement.code(), "ATN");
        assert_eq!(Primary::Fortune.id(), "attr.fortune");
        assert_eq!(effective_sub(100, 80), 150);
        assert_eq!(effective_sub(5, -20), 1);
        assert_eq!(effective_sub(40, 3), 43);
        assert_eq!(
            SubAttrs::uniform(1)
                .with(Sub::Power, 3)
                .primary(Primary::Might),
            1
        );
    }

    /// SPEC: doc 04 §12.1 Death's Door roll uses one keyed `rng.combat` draw.
    #[test]
    fn deaths_door_roll_is_keyed() {
        let a = roll_deaths_door(0x0123_4567_89AB_CDEF, 3, (3 << 40) | 5, 1000, 6000);
        let b = roll_deaths_door(0x0123_4567_89AB_CDEF, 3, (3 << 40) | 5, 1000, 6000);
        assert_eq!(a, b);
        assert!(!roll_deaths_door(1, 1, 1, 1, 0));
        assert!(roll_deaths_door(1, 1, 1, 1, 10_000));
    }

    fn arb_sub() -> impl Strategy<Value = i32> {
        SUB_EFFECTIVE_MIN..=SUB_EFFECTIVE_MAX
    }

    proptest! {
        /// SPEC: canon §6.4 clamped rows stay in range over the whole effective range.
        #[test]
        fn clamped_rows_in_range(x in arb_sub(), y in arb_sub(), skill in 0i32..=100, v in 0u8..=10, lv in 0u8..=10) {
            prop_assert!((8000..=12_500).contains(&move_speed_bp(x)));
            prop_assert!((0..=7500).contains(&crit_chance_bp(x, y, lv, v, 0)));
            prop_assert!((5000..=14_000).contains(&spread_bp(x)));
            prop_assert!((7000..=15_000).contains(&action_speed_bp(x)));
            prop_assert!((0..=9500).contains(&cast_keep_bp(x)));
            prop_assert!((0..=9000).contains(&trap_detection_bp(x, skill)));
            prop_assert!((0..=6000).contains(&deaths_door_bp(x, v)));
            prop_assert!(status_threshold(x) >= 40);
            let term = [ScalingTerm { coef_bp: 10_000, value: x }];
            prop_assert!(weapon_scaling_bp(&term, skill) >= -5000);
            prop_assert!(sprint_drain_milli_per_s(x) >= 4000);
        }

        /// SPEC: canon §6.4 monotonicity: more of the governing sub-attribute
        /// never hurts, and Max HP grows with level.
        #[test]
        fn rows_monotone(x in SUB_EFFECTIVE_MIN..SUB_EFFECTIVE_MAX, level in 1u32..200, v in 0u8..10) {
            prop_assert!(max_hp_mhp(x + 1, level, 0) >= max_hp_mhp(x, level, 0));
            prop_assert!(max_hp_mhp(x, level + 1, 0) > max_hp_mhp(x, level, 0));
            prop_assert!(move_speed_bp(x + 1) >= move_speed_bp(x));
            prop_assert!(spread_bp(x + 1) <= spread_bp(x));
            prop_assert!(deaths_door_bp(x + 1, v) >= deaths_door_bp(x, v));
            prop_assert!(deaths_door_bp(x, v + 1) >= deaths_door_bp(x, v));
            prop_assert!(crit_chance_bp(x + 1, x, 5, v, 0) >= crit_chance_bp(x, x, 5, v, 0));
            prop_assert!(hp_regen_mhp_per_s(85_000, x + 1) >= hp_regen_mhp_per_s(85_000, x));
        }

        /// SPEC: canon §6.3: the shortfall multiplier is in 5000..=10000 and
        /// never rises as the requirement rises.
        #[test]
        fn shortfall_range(req in 0i32..=200, value in 1i32..=150) {
            let m = requirement_multiplier_bp(req, value);
            prop_assert!((5000..=10_000).contains(&m));
            prop_assert!(requirement_multiplier_bp(req + 1, value) <= m);
        }

        /// SPEC: canon §6.5: an NPC's rows equal the Erratum rows for the same
        /// inputs (row 13 with no gear term).
        #[test]
        fn npc_rows_match_erratum_rows(vals in prop::array::uniform24(SUB_EFFECTIVE_MIN..=SUB_EFFECTIVE_MAX), g in 0usize..16) {
            let local = LocalGenre::Plain(Genre::ALL[g]);
            let subs = SubAttrs(vals);
            let e = derive_erratum(&ErratumInputs { subs, level: 1, local, inspired: false, gear: GearTerms::default(), awareness: 7 });
            let n = derive_npc(&NpcInputs { subs, local, inspired: false, armor_poise_milli: 0, awareness: 7 });
            prop_assert_eq!(e.move_speed_bp, n.move_speed_bp);
            prop_assert_eq!(e.crit_chance_bp, n.crit_chance_bp);
            prop_assert_eq!(e.thresholds, n.thresholds);
            prop_assert_eq!(e.trap_detection_bp, n.trap_detection_bp);
            prop_assert_eq!(e.buildup_bp, n.buildup_bp);
            prop_assert_eq!(e.poise_milli, n.poise_milli);
        }
    }
}
