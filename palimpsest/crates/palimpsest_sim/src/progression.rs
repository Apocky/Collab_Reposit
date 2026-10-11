//! SPEC: canon §7-8 skills, levels, XP
//!
//! The 36 skills (§7), skill growth and sub-attribute training (§7.1),
//! character levels and every canon character-XP source (§8), the enemy-rank
//! table (§17.3), the party-size tables and co-op split (§17.2, §8), and the
//! Calling skill presets (§16.2). Pure functions over plain integers.

use crate::stats::Sub;
use palimpsest_math::int::{floor_div, floor_div_i128, mul_bp, narrow_i64};

/// SPEC: canon §7 `Group` column.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(missing_docs)]
pub enum SkillGroup {
    Combat,
    Defense,
    Mobility,
    Magic,
    Faith,
    Stealth,
    Craft,
    Knowledge,
    Social,
}

/// SPEC: canon §7. The 36 skills in table order.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(missing_docs)]
pub enum Skill {
    Blade,
    Hafted,
    Polearm,
    Unarmed,
    Shield,
    Marksmanship,
    HeavyArms,
    Throwing,
    Armor,
    Acrobatics,
    Athletics,
    Evocation,
    Abjuration,
    Alteration,
    Conjuration,
    Illusion,
    Restoration,
    Divination,
    Spellcraft,
    Devotion,
    Sneak,
    Security,
    Pickpocket,
    Smithing,
    Tinkering,
    Alchemy,
    Inscription,
    Appraisal,
    Medicine,
    Survival,
    Speech,
    Leadership,
    Hacking,
    Fieldcraft,
    Awareness,
    Handling,
}

/// SPEC: canon §7. One skill row: ID, name, governing sub-attribute, group.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SkillRow {
    /// Content ID, `skill.<name>`.
    pub id: &'static str,
    /// Display name.
    pub name: &'static str,
    /// Governing sub-attribute (receives training marks).
    pub governing: Sub,
    /// Group.
    pub group: SkillGroup,
}

const fn sk(id: &'static str, name: &'static str, governing: Sub, group: SkillGroup) -> SkillRow {
    SkillRow { id, name, governing, group }
}

use SkillGroup as G;
use Sub as S;

/// SPEC: canon §7. The skill table, indexed by [`Skill::index`].
pub static SKILLS: [SkillRow; 36] = [
    sk("skill.blade", "Blade", S::Precision, G::Combat),
    sk("skill.hafted", "Hafted", S::Power, G::Combat),
    sk("skill.polearm", "Polearm", S::Grip, G::Combat),
    sk("skill.unarmed", "Unarmed", S::Power, G::Combat),
    sk("skill.shield", "Shield", S::Brawn, G::Combat),
    sk("skill.marksmanship", "Marksmanship", S::Precision, G::Combat),
    sk("skill.heavy_arms", "Heavy Arms", S::Grip, G::Combat),
    sk("skill.throwing", "Throwing", S::Sleight, G::Combat),
    sk("skill.armor", "Armor", S::Toughness, G::Defense),
    sk("skill.acrobatics", "Acrobatics", S::Agility, G::Mobility),
    sk("skill.athletics", "Athletics", S::Endurance, G::Mobility),
    sk("skill.evocation", "Evocation", S::Attunement, G::Magic),
    sk("skill.abjuration", "Abjuration", S::Resolve, G::Magic),
    sk("skill.alteration", "Alteration", S::Focus, G::Magic),
    sk("skill.conjuration", "Conjuration", S::Memory, G::Magic),
    sk("skill.illusion", "Illusion", S::Guile, G::Magic),
    sk("skill.restoration", "Restoration", S::Resilience, G::Magic),
    sk("skill.divination", "Divination", S::Intuition, G::Magic),
    sk("skill.spellcraft", "Spellcraft", S::Logic, G::Magic),
    sk("skill.devotion", "Devotion", S::Providence, G::Faith),
    sk("skill.sneak", "Sneak", S::Agility, G::Stealth),
    sk("skill.security", "Security", S::Sleight, G::Stealth),
    sk("skill.pickpocket", "Pickpocket", S::Guile, G::Stealth),
    sk("skill.smithing", "Smithing", S::Brawn, G::Craft),
    sk("skill.tinkering", "Tinkering", S::Logic, G::Craft),
    sk("skill.alchemy", "Alchemy", S::Analysis, G::Craft),
    sk("skill.inscription", "Inscription", S::Memory, G::Craft),
    sk("skill.appraisal", "Appraisal", S::Intuition, G::Knowledge),
    sk("skill.medicine", "Medicine", S::Analysis, G::Knowledge),
    sk("skill.survival", "Survival", S::Sight, G::Knowledge),
    sk("skill.speech", "Speech", S::Charm, G::Social),
    sk("skill.leadership", "Leadership", S::Menace, G::Social),
    sk("skill.hacking", "Hacking", S::Logic, G::Stealth),
    sk("skill.fieldcraft", "Fieldcraft", S::Endurance, G::Craft),
    sk("skill.awareness", "Awareness", S::Hearing, G::Stealth),
    sk("skill.handling", "Handling", S::Charm, G::Social),
];

impl Skill {
    /// SPEC: canon §7. All skills in table order.
    pub const ALL: [Skill; 36] = [
        Skill::Blade,
        Skill::Hafted,
        Skill::Polearm,
        Skill::Unarmed,
        Skill::Shield,
        Skill::Marksmanship,
        Skill::HeavyArms,
        Skill::Throwing,
        Skill::Armor,
        Skill::Acrobatics,
        Skill::Athletics,
        Skill::Evocation,
        Skill::Abjuration,
        Skill::Alteration,
        Skill::Conjuration,
        Skill::Illusion,
        Skill::Restoration,
        Skill::Divination,
        Skill::Spellcraft,
        Skill::Devotion,
        Skill::Sneak,
        Skill::Security,
        Skill::Pickpocket,
        Skill::Smithing,
        Skill::Tinkering,
        Skill::Alchemy,
        Skill::Inscription,
        Skill::Appraisal,
        Skill::Medicine,
        Skill::Survival,
        Skill::Speech,
        Skill::Leadership,
        Skill::Hacking,
        Skill::Fieldcraft,
        Skill::Awareness,
        Skill::Handling,
    ];

    /// SPEC: canon §7. Index `0..=35` in table order.
    #[must_use]
    pub const fn index(self) -> usize {
        self as usize
    }

    /// SPEC: canon §7. The table row.
    #[must_use]
    pub const fn row(self) -> &'static SkillRow {
        &SKILLS[self.index()]
    }

    /// SPEC: canon §7. Content ID, for example `skill.blade`.
    #[must_use]
    pub const fn id(self) -> &'static str {
        self.row().id
    }

    /// SPEC: canon §7.1. Governing sub-attribute.
    #[must_use]
    pub const fn governing(self) -> Sub {
        self.row().governing
    }

    /// SPEC: canon §7. Group.
    #[must_use]
    pub const fn group(self) -> SkillGroup {
        self.row().group
    }

    /// SPEC: canon §7. The skill with content ID `id`, if any.
    #[must_use]
    pub fn from_id(id: &str) -> Option<Skill> {
        Skill::ALL.into_iter().find(|s| s.id() == id)
    }
}

// ---------------------------------------------------------------- §7.1

/// SPEC: canon §7. Skills are integers `0..=100`.
pub const SKILL_MAX: i32 = 100;

/// SPEC: canon §7. Mastery tiers: Apprentice, Journeyman, Expert, Master.
pub const MASTERY_TIERS: [i32; 4] = [25, 50, 75, 100];

/// SPEC: canon §7.1 `need(s) = 4000 + 400 × s` milli-XP to go from `s` to `s + 1`.
#[must_use]
pub const fn skill_need_milli(s: i32) -> i64 {
    4000 + 400 * s as i64
}

/// SPEC: canon §7.1, §21 test 22. Cumulative milli-XP from skill `from` to
/// skill `to` (`Σ need(s)` for `s` in `from..to`; 0 when `to <= from`).
#[must_use]
pub const fn cumulative_skill_xp_milli(from: i32, to: i32) -> i64 {
    if to <= from {
        return 0;
    }
    let n = (to - from) as i64;
    // Σ s over from..to = n × (from + to - 1) / 2, an exact integer.
    let sum_s = n * (from as i64 + to as i64 - 1) / 2;
    4000 * n + 400 * sum_s
}

/// SPEC: canon §7.1. Every skill XP grant is × the Skill XP multiplier (§6.4
/// row 28) before being added.
#[must_use]
pub const fn scaled_skill_xp_milli(milli: i64, row28_bp: i32) -> i64 {
    mul_bp(milli, row28_bp)
}

/// SPEC: canon §7.1. Per-hit cap of combat skill XP.
pub const COMBAT_XP_PER_HIT_CAP_MILLI: i64 = 1500;

/// SPEC: canon §7.1. Combat skill XP for a hit dealing `dmg_mhp` to a hostile
/// with Max HP `target_max_mhp`: `floor_div(1500 × dmg_mhp, target_max_mhp)`,
/// capped at 1500 (and 0 for zero-effect hits). Panics if `target_max_mhp <= 0`.
#[must_use]
pub const fn combat_skill_xp_milli(dmg_mhp: i64, target_max_mhp: i64) -> i64 {
    let v = floor_div_i128(1500 * dmg_mhp as i128, target_max_mhp as i128);
    if v <= 0 {
        0
    } else if v > COMBAT_XP_PER_HIT_CAP_MILLI as i128 {
        COMBAT_XP_PER_HIT_CAP_MILLI
    } else {
        v as i64
    }
}

/// SPEC: canon §7.1. A skill's value and its progress toward the next level.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct SkillTrack {
    /// Skill value `0..=100`.
    pub value: i32,
    /// Milli-XP accumulated toward `value + 1`.
    pub progress_milli: i64,
}

/// SPEC: canon §7.1 growth (doc 04 §5.1 `grant_skill_xp`): add `milli`
/// (already × row 28), level up while progress covers `need(value)`, stop at
/// 100 (where progress is 0). Returns the new track and the levels gained.
#[must_use]
pub const fn grant_skill_xp(track: SkillTrack, milli: i64) -> (SkillTrack, u32) {
    if milli <= 0 || track.value >= SKILL_MAX {
        return (track, 0);
    }
    let mut t = SkillTrack { value: track.value, progress_milli: track.progress_milli + milli };
    let mut levels = 0;
    while t.value < SKILL_MAX && t.progress_milli >= skill_need_milli(t.value) {
        t.progress_milli -= skill_need_milli(t.value);
        t.value += 1;
        levels += 1;
    }
    if t.value == SKILL_MAX {
        t.progress_milli = 0;
    }
    (t, levels)
}

/// SPEC: canon §7. Mastery tiers reached by going from `old` to `new`.
pub fn mastery_tiers_reached(old: i32, new: i32) -> impl Iterator<Item = i32> {
    MASTERY_TIERS.into_iter().filter(move |&t| old < t && t <= new)
}

/// SPEC: canon §7.1 `T(v) = 2 + floor_div(v, 20)` training marks to raise base `v`.
#[must_use]
pub const fn training_threshold(v: i32) -> i32 {
    2 + floor_div(v as i64, 20) as i32
}

/// SPEC: canon §7.1. A governing sub-attribute's base value and its marks.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Training {
    /// Stored base value `1..=100`.
    pub base: i32,
    /// Training marks `0..T(base)`.
    pub marks: i32,
}

/// SPEC: canon §7.1. One skill level gained adds a mark; at `T(v)` marks the
/// base rises by 1 (never above 100) and marks reset to 0.
#[must_use]
pub const fn add_training_mark(t: Training) -> Training {
    let marks = t.marks + 1;
    if marks >= training_threshold(t.base) {
        let base = if t.base < 100 { t.base + 1 } else { t.base };
        Training { base, marks: 0 }
    } else {
        Training { base: t.base, marks }
    }
}

// ---------------------------------------------------------------- §8

/// SPEC: canon §8 `xp_to_next(L) = 50 × L × (L + 3)`.
#[must_use]
pub const fn xp_to_next(level: u32) -> i64 {
    let l = level as i128;
    narrow_i64(50 * l * (l + 3))
}

/// SPEC: canon §8, §21 test 23. Cumulative XP to reach `level` from level 1:
/// `Σ_{l=1}^{L-1} 50 l (l + 3) = 50 n (n + 1)(n + 5) / 3` with `n = L - 1`.
#[must_use]
pub const fn cumulative_level_xp(level: u32) -> i64 {
    narrow_i64(cumulative_level_xp_wide(level))
}

/// `cumulative_level_xp` in `i128`, defined for every `u32` level.
const fn cumulative_level_xp_wide(level: u32) -> i128 {
    if level <= 1 {
        return 0;
    }
    let n = level as i128 - 1;
    floor_div_i128(50 * n * (n + 1) * (n + 5), 3)
}

/// SPEC: canon §8. The level a character with total XP `xp` has (level 1 at
/// 0 XP; no level cap).
#[must_use]
pub fn level_for_xp(xp: i64) -> u32 {
    if xp < cumulative_level_xp(2) {
        return 1;
    }
    // Largest L with cumulative_level_xp(L) <= xp; cumulative XP passes
    // i64::MAX before L = 2_000_000.
    let (mut lo, mut hi) = (2u32, 2_000_000u32);
    while lo < hi {
        let mid = lo + (hi - lo).div_ceil(2);
        if cumulative_level_xp_wide(mid) <= xp as i128 {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    lo
}

/// SPEC: canon §8. Each level-up grants 4 attribute points.
pub const ATTRIBUTE_POINTS_PER_LEVEL: u32 = 4;
/// SPEC: canon §8. At most +2 to the same sub-attribute per level-up.
pub const MAX_POINTS_PER_SUB_PER_LEVEL: u32 = 2;

/// SPEC: canon §8. Whether one more point may go into a sub-attribute with
/// stored value `base` that already took `points_this_level` from this level-up.
#[must_use]
pub const fn can_allocate_point(base: i32, points_this_level: u32) -> bool {
    base < 100 && points_this_level < MAX_POINTS_PER_SUB_PER_LEVEL
}

/// SPEC: canon §8. Each even level grants 1 talent point.
#[must_use]
pub const fn talent_points_at(level: u32) -> u32 {
    if level % 2 == 0 { 1 } else { 0 }
}

/// SPEC: canon §13.8 `enemy_level(d) = d + floor_div(d, 5)`.
#[must_use]
pub const fn enemy_level(d: u32) -> u32 {
    d + d / 5
}

/// SPEC: canon §8 `kill_xp_std(d) = floor_div(7 × e × (e + 3), 5)`, `e = enemy_level(d)`.
#[must_use]
pub const fn kill_xp_std(d: u32) -> i64 {
    let e = enemy_level(d) as i128;
    narrow_i64(floor_div_i128(7 * e * (e + 3), 5))
}

/// SPEC: canon §8. Kills pay `mul_bp(mul_bp(kill_xp_std(d), rank_xp_bp), 7000)`.
pub const KILL_XP_FACTOR_BP: i32 = 7000;

/// SPEC: canon §8 kill XP for an enemy of `rank` at depth `d`.
#[must_use]
pub const fn kill_xp(d: u32, rank: Rank) -> i64 {
    mul_bp(mul_bp(kill_xp_std(d), rank.row().kill_xp_bp), KILL_XP_FACTOR_BP)
}

/// SPEC: canon §8. Skill-up XP counts at most 8 skill levels per character per Folio.
pub const SKILL_UP_XP_PER_FOLIO_CAP: u32 = 8;

/// SPEC: canon §8. Each counted skill level gained grants `2 × kill_xp_std(d_current)`.
#[must_use]
pub const fn skill_up_xp(d_current: u32) -> i64 {
    2 * kill_xp_std(d_current)
}

/// SPEC: canon §8. `ev.folio_exited` grants `4 × kill_xp_std(d)`.
#[must_use]
pub const fn folio_exit_xp(d: u32) -> i64 {
    4 * kill_xp_std(d)
}

/// SPEC: canon §8. Unseen bonus at exit: `20 × kill_xp_std(d)` more.
#[must_use]
pub const fn unseen_bonus_xp(d: u32) -> i64 {
    20 * kill_xp_std(d)
}

// ---------------------------------------------------------------- §17.2, §17.3

/// SPEC: canon §17.3. Enemy ranks.
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

/// SPEC: canon §17.3. One rank row (all Bp).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct RankRow {
    /// Content ID, `rank.<name>`.
    pub id: &'static str,
    /// HP multiplier.
    pub hp_bp: i32,
    /// Damage multiplier.
    pub damage_bp: i32,
    /// Kill XP multiplier (`rank_xp_bp`).
    pub kill_xp_bp: i32,
    /// DoT taken multiplier.
    pub dot_taken_bp: i32,
    /// Control duration multiplier.
    pub control_bp: i32,
}

const fn rk(id: &'static str, hp_bp: i32, damage_bp: i32, kill_xp_bp: i32, dot_taken_bp: i32, control_bp: i32) -> RankRow {
    RankRow { id, hp_bp, damage_bp, kill_xp_bp, dot_taken_bp, control_bp }
}

/// SPEC: canon §17.3 table, indexed by `Rank as usize`.
pub static RANKS: [RankRow; 7] = [
    rk("rank.minion", 4000, 6000, 3000, 10_000, 10_000),
    rk("rank.standard", 10_000, 10_000, 10_000, 10_000, 10_000),
    rk("rank.elite", 30_000, 13_000, 30_000, 5000, 7500),
    rk("rank.champion", 60_000, 15_000, 60_000, 5000, 5000),
    rk("rank.boss", 200_000, 16_000, 200_000, 2500, 2500),
    rk("rank.editor", 400_000, 18_000, 400_000, 2500, 2500),
    rk("rank.hunter", 120_000, 20_000, 50_000, 2500, 2500),
];

impl Rank {
    /// SPEC: canon §17.3. All ranks in table order.
    pub const ALL: [Rank; 7] =
        [Rank::Minion, Rank::Standard, Rank::Elite, Rank::Champion, Rank::Boss, Rank::Editor, Rank::Hunter];

    /// SPEC: canon §17.3. The table row.
    #[must_use]
    pub const fn row(self) -> &'static RankRow {
        &RANKS[self as usize]
    }

    /// SPEC: canon §17.2, §17.3. Boss, editor and hunter use the boss HP row
    /// of the party table and are immune to charmed, sleeping and confused.
    #[must_use]
    pub const fn is_boss_tier(self) -> bool {
        matches!(self, Rank::Boss | Rank::Editor | Rank::Hunter)
    }

    /// SPEC: canon §17.2. The party-size HP table this rank uses.
    #[must_use]
    pub const fn party_hp_table(self) -> PartyTable {
        if self.is_boss_tier() { PartyTable::EnemyHpBoss } else { PartyTable::EnemyHp }
    }
}

/// SPEC: canon §17.2 party-size tables.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum PartyTable {
    /// Enemy HP, ranks minion to champion.
    EnemyHp,
    /// Enemy HP, ranks boss, editor, hunter.
    EnemyHpBoss,
    /// Enemy count (`enemy_count_bp`).
    EnemyCount,
    /// Enemy damage against Errata.
    EnemyDamage,
}

/// SPEC: canon §17.1 `PARTY_MAX`.
pub const PARTY_MAX: u8 = 4;

/// SPEC: canon §17.2. The Bp entry for party size `n` in `1..=4`; panics otherwise.
#[must_use]
pub const fn party_bp(table: PartyTable, n: u8) -> i32 {
    assert!(n >= 1 && n <= PARTY_MAX, "invariant: party size in 1..=4");
    let row: [i32; 4] = match table {
        PartyTable::EnemyHp => [10_000, 16_000, 21_000, 25_500],
        PartyTable::EnemyHpBoss => [10_000, 19_000, 28_000, 37_000],
        PartyTable::EnemyCount => [10_000, 12_500, 15_000, 17_500],
        PartyTable::EnemyDamage => [10_000, 11_000, 12_000, 13_000],
    };
    row[n as usize - 1]
}

/// SPEC: canon §8 co-op split: each party member present gains
/// `floor_div(kill_xp × 10000, enemy_count_bp[n])`.
#[must_use]
pub const fn coop_kill_xp(kill_xp: i64, party_size: u8) -> i64 {
    narrow_i64(floor_div_i128(kill_xp as i128 * 10_000, party_bp(PartyTable::EnemyCount, party_size) as i128))
}

/// SPEC: canon §17.2. Enemy count per spawner
/// `floor_div(n_base × enemy_count_bp + carry, 10000)`; returns the count and
/// the remainder kept as the spawner's new `carry`.
#[must_use]
pub const fn enemy_count(n_base: i64, party_size: u8, carry: i64) -> (i64, i64) {
    let total = n_base * party_bp(PartyTable::EnemyCount, party_size) as i64 + carry;
    let q = floor_div(total, 10_000);
    (q, total - 10_000 * q)
}

// ---------------------------------------------------------------- §16.2

/// SPEC: canon §16.2. Major skills start at 20.
pub const MAJOR_SKILL_START: i32 = 20;
/// SPEC: canon §16.2. Minor skills start at 10.
pub const MINOR_SKILL_START: i32 = 10;
/// SPEC: canon §16.1. The Origin adds +10 to one Origin skill.
pub const ORIGIN_SKILL_BONUS: i32 = 10;

/// SPEC: canon §16.2. The 13 Callings.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(missing_docs)]
pub enum Calling {
    Vanguard,
    Duelist,
    Gunslinger,
    Grenadier,
    Spellwright,
    Alchemist,
    Shade,
    Artificer,
    Warden,
    Oracle,
    Netrunner,
    Beastbinder,
    Monk,
}

use Skill as K;

/// SPEC: canon §16.2 table: ID, three major skills, three minor skills.
pub static CALLINGS: [(&str, [Skill; 3], [Skill; 3]); 13] = [
    ("calling.vanguard", [K::Hafted, K::Armor, K::Shield], [K::Athletics, K::Leadership, K::Smithing]),
    ("calling.duelist", [K::Blade, K::Acrobatics, K::Speech], [K::Sneak, K::Throwing, K::Unarmed]),
    ("calling.gunslinger", [K::Marksmanship, K::Acrobatics, K::Throwing], [K::Tinkering, K::Speech, K::Security]),
    ("calling.grenadier", [K::HeavyArms, K::Throwing, K::Armor], [K::Tinkering, K::Fieldcraft, K::Medicine]),
    ("calling.spellwright", [K::Evocation, K::Spellcraft, K::Alteration], [K::Abjuration, K::Inscription, K::Divination]),
    ("calling.alchemist", [K::Alchemy, K::Throwing, K::Medicine], [K::Survival, K::Restoration, K::Appraisal]),
    ("calling.shade", [K::Sneak, K::Blade, K::Security], [K::Pickpocket, K::Acrobatics, K::Illusion]),
    ("calling.artificer", [K::Tinkering, K::Smithing, K::Marksmanship], [K::Security, K::Inscription, K::Appraisal]),
    ("calling.warden", [K::Shield, K::Abjuration, K::Restoration], [K::Polearm, K::Devotion, K::Leadership]),
    ("calling.oracle", [K::Divination, K::Devotion, K::Conjuration], [K::Appraisal, K::Illusion, K::Speech]),
    ("calling.netrunner", [K::Hacking, K::Tinkering, K::Sneak], [K::Security, K::Awareness, K::Illusion]),
    ("calling.beastbinder", [K::Handling, K::Conjuration, K::Survival], [K::Polearm, K::Leadership, K::Restoration]),
    ("calling.monk", [K::Unarmed, K::Acrobatics, K::Athletics], [K::Alteration, K::Medicine, K::Awareness]),
];

impl Calling {
    /// SPEC: canon §16.2. All Callings in table order.
    pub const ALL: [Calling; 13] = [
        Calling::Vanguard,
        Calling::Duelist,
        Calling::Gunslinger,
        Calling::Grenadier,
        Calling::Spellwright,
        Calling::Alchemist,
        Calling::Shade,
        Calling::Artificer,
        Calling::Warden,
        Calling::Oracle,
        Calling::Netrunner,
        Calling::Beastbinder,
        Calling::Monk,
    ];

    /// SPEC: canon §16.2. Content ID, for example `calling.monk`.
    #[must_use]
    pub const fn id(self) -> &'static str {
        CALLINGS[self as usize].0
    }

    /// SPEC: canon §16.2. The three major skills.
    #[must_use]
    pub const fn majors(self) -> [Skill; 3] {
        CALLINGS[self as usize].1
    }

    /// SPEC: canon §16.2. The three minor skills.
    #[must_use]
    pub const fn minors(self) -> [Skill; 3] {
        CALLINGS[self as usize].2
    }

    /// SPEC: canon §16.2. Starting value of `skill`: 20 for a major, 10 for a
    /// minor, 0 otherwise (before Origin and kin bonuses).
    #[must_use]
    pub fn start_skill(self, skill: Skill) -> i32 {
        if self.majors().contains(&skill) {
            MAJOR_SKILL_START
        } else if self.minors().contains(&skill) {
            MINOR_SKILL_START
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stats::skill_xp_bp;
    use proptest::prelude::*;

    /// SPEC: canon §21 test 22. Cumulative milli-XP from skill 0 to 25 is
    /// 220,000, and to 75 is 1,410,000.
    #[test]
    fn canon_t22_skill_curve() {
        assert_eq!(cumulative_skill_xp_milli(0, 25), 220_000);
        assert_eq!(cumulative_skill_xp_milli(0, 75), 1_410_000);
        // §7.1: reaching 100 takes 2,380,000; §7.2: 20 to 75 costs 1,254 skill XP.
        assert_eq!(cumulative_skill_xp_milli(0, 100), 2_380_000);
        assert_eq!(cumulative_skill_xp_milli(20, 75), 1_254_000);
        let brute: i64 = (0..75).map(skill_need_milli).sum();
        assert_eq!(brute, 1_410_000);
    }

    /// SPEC: canon §21 test 23. Cumulative XP to reach level 11 is 27,500;
    /// reference kill XP for a standard enemy at Folio 25 is 1,386.
    #[test]
    fn canon_t23_level_curve_and_kill_xp() {
        assert_eq!(cumulative_level_xp(11), 27_500);
        assert_eq!(kill_xp_std(25), 1386);
        // §8: level 2 needs 200 cumulative, level 31 needs 542,500.
        assert_eq!(cumulative_level_xp(2), 200);
        assert_eq!(cumulative_level_xp(31), 542_500);
        assert_eq!(xp_to_next(1), 200);
        assert_eq!(xp_to_next(10), 6500);
        let brute: i64 = (1..11).map(xp_to_next).sum();
        assert_eq!(brute, 27_500);
    }

    /// SPEC: canon §8 kill XP, cross-checked with the doc 04 §6.1 table
    /// (`K(1) = 5`, `K(10) = 252`, `K(25) = 1386`).
    #[test]
    fn kill_xp_by_rank() {
        assert_eq!((kill_xp_std(1), kill_xp_std(10), kill_xp_std(25)), (5, 252, 1386));
        let table: [(Rank, [i64; 3]); 7] = [
            (Rank::Minion, [0, 52, 290]),
            (Rank::Standard, [3, 176, 970]),
            (Rank::Elite, [10, 529, 2910]),
            (Rank::Champion, [21, 1058, 5821]),
            (Rank::Boss, [70, 3528, 19_404]),
            (Rank::Editor, [140, 7056, 38_808]),
            (Rank::Hunter, [17, 882, 4851]),
        ];
        for (rank, xp) in table {
            assert_eq!([kill_xp(1, rank), kill_xp(10, rank), kill_xp(25, rank)], xp, "{rank:?}");
        }
        assert_eq!(enemy_level(25), 30);
        assert_eq!(enemy_level(4), 4);
        // Skill-ups at d = 10: 2 × 252; exit 4 ×; Unseen 20 ×.
        assert_eq!(skill_up_xp(10), 504);
        assert_eq!(folio_exit_xp(10), 1008);
        assert_eq!(unseen_bonus_xp(1), 100);
    }

    /// SPEC: canon §8 co-op split with the §17.2 enemy-count row: an elite at
    /// d = 10 in a party of 4 gives `floor_div(529 × 10000, 17500) = 302` each.
    #[test]
    fn coop_split() {
        assert_eq!(coop_kill_xp(kill_xp(10, Rank::Elite), 4), 302);
        assert_eq!(coop_kill_xp(529, 1), 529);
        assert_eq!(coop_kill_xp(529, 2), 423);
    }

    /// SPEC: canon §17.2 tables and the spawner carry rule.
    #[test]
    fn party_tables() {
        assert_eq!(party_bp(PartyTable::EnemyHp, 4), 25_500);
        assert_eq!(party_bp(PartyTable::EnemyHpBoss, 3), 28_000);
        assert_eq!(party_bp(PartyTable::EnemyCount, 2), 12_500);
        assert_eq!(party_bp(PartyTable::EnemyDamage, 1), 10_000);
        assert_eq!(Rank::Hunter.party_hp_table(), PartyTable::EnemyHpBoss);
        assert_eq!(Rank::Champion.party_hp_table(), PartyTable::EnemyHp);
        // 3 base enemies at party size 2: 3.75 → 3, carry 7500; next room 4.5 → 4.
        let (c1, carry) = enemy_count(3, 2, 0);
        assert_eq!((c1, carry), (3, 7500));
        let (c2, carry) = enemy_count(3, 2, carry);
        assert_eq!((c2, carry), (4, 5000));
    }

    /// SPEC: canon §17.2: party size outside 1..=4 is an invariant violation.
    #[test]
    #[should_panic(expected = "invariant")]
    fn party_size_zero_panics() {
        let _ = party_bp(PartyTable::EnemyCount, 0);
    }

    /// SPEC: canon §17.3 table spot checks.
    #[test]
    fn rank_table() {
        assert_eq!(Rank::Elite.row().hp_bp, 30_000);
        assert_eq!(Rank::Hunter.row().kill_xp_bp, 50_000);
        assert_eq!(Rank::Boss.row().control_bp, 2500);
        assert_eq!(Rank::Minion.row().damage_bp, 6000);
        assert!(Rank::Editor.is_boss_tier() && !Rank::Champion.is_boss_tier());
        for r in Rank::ALL {
            assert!(r.row().id.starts_with("rank."));
        }
    }

    /// SPEC: canon §7.1 combat XP normalization (doc 04 test 6: 20,000 mHP of a
    /// 60,000-mHP hostile grants 500; at MEM 50, 525).
    #[test]
    fn combat_xp() {
        assert_eq!(combat_skill_xp_milli(20_000, 60_000), 500);
        assert_eq!(scaled_skill_xp_milli(500, skill_xp_bp(50)), 525);
        assert_eq!(combat_skill_xp_milli(60_000, 60_000), 1500); // one standard enemy drained
        assert_eq!(combat_skill_xp_milli(500_000, 60_000), 1500);
        assert_eq!(combat_skill_xp_milli(0, 60_000), 0);
        assert_eq!(combat_skill_xp_milli(i64::MAX, 1), 1500);
    }

    /// SPEC: canon §7.1 growth (doc 04 test 5: Hacking 49 receiving 23,600
    /// milli-XP becomes 50; one LOG mark; at LOG 66 the fifth mark raises it to 67).
    #[test]
    fn skill_growth_and_training() {
        let (t, n) = grant_skill_xp(SkillTrack { value: 49, progress_milli: 0 }, 23_600);
        assert_eq!((t, n), (SkillTrack { value: 50, progress_milli: 0 }, 1));
        assert_eq!(mastery_tiers_reached(49, 50).collect::<Vec<_>>(), [50]);
        assert_eq!(Skill::Hacking.governing(), Sub::Logic);
        let t = add_training_mark(Training { base: 66, marks: 4 });
        assert_eq!(t, Training { base: 67, marks: 0 });
        assert_eq!(training_threshold(66), 5);
        assert_eq!(add_training_mark(Training { base: 66, marks: 0 }), Training { base: 66, marks: 1 });
        // Never above 100.
        assert_eq!(add_training_mark(Training { base: 100, marks: 6 }), Training { base: 100, marks: 0 });
        // Zero grants nothing; skill 100 stops and clears progress.
        let s = SkillTrack { value: 10, progress_milli: 7 };
        assert_eq!(grant_skill_xp(s, 0), (s, 0));
        let (top, _) = grant_skill_xp(SkillTrack { value: 99, progress_milli: 0 }, 1_000_000);
        assert_eq!(top, SkillTrack { value: 100, progress_milli: 0 });
        let (from0, n) = grant_skill_xp(SkillTrack::default(), 220_000);
        assert_eq!((from0.value, n), (25, 25));
    }

    /// SPEC: canon §8 level lookup (doc 04 test 9 values).
    #[test]
    fn level_lookup() {
        assert_eq!(level_for_xp(0), 1);
        assert_eq!(level_for_xp(199), 1);
        assert_eq!(level_for_xp(200), 2);
        assert_eq!(level_for_xp(27_500), 11);
        assert_eq!(level_for_xp(403_199), 27);
        assert_eq!(level_for_xp(403_200), 28);
        assert_eq!(level_for_xp(419_094), 28);
        assert_eq!(level_for_xp(492_999), 29);
        assert_eq!(level_for_xp(493_000), 30);
        assert_eq!(level_for_xp(542_500), 31);
        assert_eq!(level_for_xp(i64::MAX), level_for_xp(i64::MAX));
        let top = level_for_xp(i64::MAX);
        assert!(cumulative_level_xp(top) <= i64::MAX);
    }

    /// SPEC: canon §8 level-up grants.
    #[test]
    fn level_up_grants() {
        assert_eq!(ATTRIBUTE_POINTS_PER_LEVEL, 4);
        assert!(can_allocate_point(55, 1));
        assert!(!can_allocate_point(55, 2));
        assert!(!can_allocate_point(100, 0));
        assert_eq!((talent_points_at(2), talent_points_at(3)), (1, 0));
    }

    /// SPEC: canon §7 and §16.2 tables.
    #[test]
    fn skill_and_calling_tables() {
        assert_eq!(Skill::ALL.len(), 36);
        for (i, s) in Skill::ALL.into_iter().enumerate() {
            assert_eq!(s.index(), i);
            assert_eq!(Skill::from_id(s.id()), Some(s));
        }
        assert_eq!(Skill::Awareness.governing(), Sub::Hearing);
        assert_eq!(Skill::Fieldcraft.group(), SkillGroup::Craft);
        // FAT and SER govern no skill.
        assert!(Skill::ALL.iter().all(|s| s.governing() != Sub::Fate && s.governing() != Sub::Serendipity));
        assert_eq!(Calling::ALL.len(), 13);
        for c in Calling::ALL {
            let all: Vec<Skill> = c.majors().into_iter().chain(c.minors()).collect();
            for (i, a) in all.iter().enumerate() {
                assert!(!all[i + 1..].contains(a), "{} repeats {a:?}", c.id());
            }
        }
        assert_eq!(Calling::Netrunner.start_skill(Skill::Hacking), 20);
        assert_eq!(Calling::Netrunner.start_skill(Skill::Awareness), 10);
        assert_eq!(Calling::Netrunner.start_skill(Skill::Blade), 0);
        assert_eq!(Calling::Monk.id(), "calling.monk");
    }

    proptest! {
        /// SPEC: canon §7.1: `need` grows, and cumulative XP is additive over
        /// consecutive ranges.
        #[test]
        fn skill_curve_additive(a in 0i32..=100, b in 0i32..=100, c in 0i32..=100) {
            let mut v = [a, b, c];
            v.sort_unstable();
            let [a, b, c] = v;
            prop_assert_eq!(cumulative_skill_xp_milli(a, b) + cumulative_skill_xp_milli(b, c), cumulative_skill_xp_milli(a, c));
            prop_assert!(skill_need_milli(a + 1) > skill_need_milli(a));
        }

        /// SPEC: canon §7.1: granting XP conserves it: what is spent on levels
        /// plus the remaining progress equals what was put in (below 100).
        #[test]
        fn skill_grant_conserves(value in 0i32..100, prog in 0i64..4000, milli in 0i64..3_000_000) {
            let start = SkillTrack { value, progress_milli: prog };
            let (end, levels) = grant_skill_xp(start, milli);
            prop_assert_eq!(end.value - value, levels as i32);
            if end.value < SKILL_MAX {
                let spent = cumulative_skill_xp_milli(value, end.value);
                prop_assert_eq!(spent + end.progress_milli, prog + milli);
                prop_assert!(end.progress_milli < skill_need_milli(end.value));
            }
            // Split grants reach the same result as one grant.
            let half = milli / 2;
            let (mid, _) = grant_skill_xp(start, half);
            let (twice, _) = grant_skill_xp(mid, milli - half);
            prop_assert_eq!(twice, end);
        }

        /// SPEC: canon §8: cumulative XP rises strictly and `level_for_xp` inverts it.
        #[test]
        fn level_curve_inverts(xp in 0i64..2_000_000_000_000, level in 1u32..5000) {
            prop_assert!(cumulative_level_xp(level + 1) > cumulative_level_xp(level));
            prop_assert_eq!(cumulative_level_xp(level + 1) - cumulative_level_xp(level), xp_to_next(level));
            let l = level_for_xp(xp);
            prop_assert!(cumulative_level_xp(l) <= xp && xp < cumulative_level_xp(l + 1));
            prop_assert_eq!(level_for_xp(cumulative_level_xp(level)), level);
        }

        /// SPEC: canon §7.1: training never raises a base above 100 and marks
        /// stay below the threshold.
        #[test]
        fn training_bounded(base in 1i32..=100, marks_in in 0i32..8) {
            let marks = marks_in.min(training_threshold(base) - 1);
            let t = add_training_mark(Training { base, marks });
            prop_assert!(t.base <= 100 && t.base >= base);
            prop_assert!(t.marks < training_threshold(t.base));
        }

        /// SPEC: canon §8, §7.1: kill XP never decreases with depth, combat XP
        /// is in 0..=1500 and the co-op split never exceeds the solo amount.
        #[test]
        fn xp_ranges(d in 1u32..400, dmg in 0i64..10_000_000, max in 1i64..10_000_000, n in 1u8..=4) {
            prop_assert!(kill_xp_std(d + 1) >= kill_xp_std(d));
            let c = combat_skill_xp_milli(dmg, max);
            prop_assert!((0..=1500).contains(&c));
            let k = kill_xp(d, Rank::Standard);
            prop_assert!(coop_kill_xp(k, n) <= k);
        }
    }
}
