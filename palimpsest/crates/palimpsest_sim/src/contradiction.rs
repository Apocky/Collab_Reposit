//! SPEC: canon §5.3-5.8, §5.10-5.11
//!
//! Dependencies and requirements (§5.3), tolerance (§5.4), Contradiction with
//! its overrides, potency and failure table (§5.5), Anachronism Edge and the
//! Shock of the New (§5.6), the ambient axiom terms (§5.7), local genre, seam
//! zones and personal VERVE (§5.8), World Laws as data (§5.10) and the
//! Unmoored rule (§5.11). Everything is a pure function of typed inputs.

use crate::genre::{Axiom, Axioms, Genre};
use palimpsest_math::int::{clamp_i64, floor_div, mul_bp};
use palimpsest_math::{Rng, fnv1a32};

// ---------------------------------------------------------------- §5.3

/// SPEC: canon §5.3. A dependency set `dep ⊆ {arcana, techne, mythos, verve}`;
/// bit `i` is the axiom with [`Axiom::index`] `i`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DepSet(u8);

impl DepSet {
    /// SPEC: canon §5.3. The empty set (dependency `none`).
    pub const NONE: DepSet = DepSet(0);

    /// SPEC: canon §5.3. `{a}`.
    #[must_use]
    pub const fn single(a: Axiom) -> DepSet {
        DepSet(1 << a.index())
    }

    /// SPEC: canon §5.3. `self ∪ {a}`.
    #[must_use]
    pub const fn with(self, a: Axiom) -> DepSet {
        DepSet(self.0 | (1 << a.index()))
    }

    /// SPEC: canon §5.3. `a ∈ self`.
    #[must_use]
    pub const fn contains(self, a: Axiom) -> bool {
        self.0 & (1 << a.index()) != 0
    }

    /// SPEC: canon §5.3. True for dependency `none`.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// SPEC: canon §5.3. Number of axes in the set.
    #[must_use]
    pub const fn len(self) -> u32 {
        self.0.count_ones()
    }

    /// SPEC: canon §5.7 GLYPH row: the single dependency axis, if the set has
    /// exactly one axis.
    #[must_use]
    pub const fn only(self) -> Option<Axiom> {
        if self.0.count_ones() == 1 {
            Axiom::from_index(self.0.trailing_zeros() as usize)
        } else {
            None
        }
    }

    /// SPEC: canon §5.3. The axes of the set in index order.
    pub fn axes(self) -> impl Iterator<Item = Axiom> {
        Axiom::ALL.into_iter().filter(move |&a| self.contains(a))
    }
}

/// SPEC: canon §5.3 "Mounts other than beasts": the dependency comes from the skin.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum MountSkin {
    /// A vehicle: techne.
    Vehicle,
    /// A broom or carpet: arcana.
    BroomOrCarpet,
    /// A divine steed: mythos.
    DivineSteed,
    /// A cloud: verve.
    Cloud,
}

/// SPEC: canon §5.3 component kinds of the default-dependency table. The kind
/// comes from the genre skin, never from the archetype.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ComponentKind {
    /// Mundane melee and armor bases, bows, slings, thrown blades, food, raw
    /// materials, blocks: none.
    Mundane,
    /// Firearms, energy weapons, gadgets, drones, turrets, implants,
    /// electronic tools, hacking, tech consumables: techne.
    Tech,
    /// Spell glyphs, wands, staves, magic texts: the origin genre's
    /// `aether_axiom`, or arcana when it is none.
    Casting,
    /// Enchantment affixes, grafts, supernatural alchemical effects: arcana.
    Enchantment,
    /// Prayers, relics, holy or unholy affixes, curse effects, undead-bane,
    /// divine favors, mutations of supernatural origin: mythos.
    Sacred,
    /// Cinematic techniques (wall-run, double jump, qinggong glide, gun-kata,
    /// "rule of cool" exotic perks): verve.
    Cinematic,
    /// Mundane alchemical effects (healing, poison, stamina, acid, fire,
    /// smoke): none.
    MundaneAlchemy,
    /// Kin passive traits, tamed beasts, beast mounts: none.
    KinOrBeast,
    /// Summons: the summoning glyph's dependency.
    Summon {
        /// Dependency of the summoning glyph.
        glyph_dep: DepSet,
    },
    /// Mounts other than beasts: from the skin.
    Mount(MountSkin),
}

/// SPEC: canon §5.3 default `dep` of a component kind skinned in `origin`.
#[must_use]
pub const fn default_dep(kind: ComponentKind, origin: Genre) -> DepSet {
    match kind {
        ComponentKind::Mundane | ComponentKind::MundaneAlchemy | ComponentKind::KinOrBeast => {
            DepSet::NONE
        }
        ComponentKind::Tech => DepSet::single(Axiom::Techne),
        ComponentKind::Casting => match origin.aether_axiom() {
            Some(a) => DepSet::single(a),
            None => DepSet::single(Axiom::Arcana),
        },
        ComponentKind::Enchantment => DepSet::single(Axiom::Arcana),
        ComponentKind::Sacred => DepSet::single(Axiom::Mythos),
        ComponentKind::Cinematic => DepSet::single(Axiom::Verve),
        ComponentKind::Summon { glyph_dep } => glyph_dep,
        ComponentKind::Mount(skin) => DepSet::single(match skin {
            MountSkin::Vehicle => Axiom::Techne,
            MountSkin::BroomOrCarpet => Axiom::Arcana,
            MountSkin::DivineSteed => Axiom::Mythos,
            MountSkin::Cloud => Axiom::Verve,
        }),
    }
}

/// SPEC: canon §5.3 `Req[a] = if a ∈ dep { max(1, origin_genre.axiom[a] - 2) } else { 0 }`.
#[must_use]
pub const fn requirement(dep: DepSet, origin: Genre) -> [u8; 4] {
    let mut out = [0u8; 4];
    let mut i = 0;
    while i < 4 {
        if dep.0 & (1 << i) != 0 {
            let v = origin.axioms().0[i].saturating_sub(2);
            out[i] = if v < 1 { 1 } else { v };
        }
        i += 1;
    }
    out
}

/// SPEC: canon §5.3. Worldgen and loot never roll a component with `a ∈ dep`
/// in a genre whose `axiom[a] = 0`; this returns false in that case.
#[must_use]
pub const fn component_rollable(dep: DepSet, genre: Genre) -> bool {
    let mut i = 0;
    while i < 4 {
        if dep.0 & (1 << i) != 0 && genre.axioms().0[i] == 0 {
            return false;
        }
        i += 1;
    }
    true
}

// ---------------------------------------------------------------- §5.4

/// SPEC: canon §5.4. Tolerance on each axis is capped at 6.
pub const TOL_CAP: u8 = 6;

/// SPEC: canon §5.4 `attune_tol = clamp(floor_div(ATN - 40, 30), 0, 3)`.
#[must_use]
pub const fn attune_tol(atn: i32) -> u8 {
    clamp_i64(floor_div(atn as i64 - 40, 30), 0, 3) as u8
}

/// SPEC: canon §5.4. The terms of `Tol[a] = origin_tol[a] + attune_tol +
/// gear_tol[a] + talent_tol[a]` before World Law grants and the cap.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct TolInputs {
    /// `origin_tol` from the Origin genre ([`Genre::origin_tol`]).
    pub origin: [u8; 4],
    /// `attune_tol` ([`attune_tol`]); added on every axis.
    pub attune: u8,
    /// `gear_tol` per axis.
    pub gear: [u8; 4],
    /// `talent_tol` per axis.
    pub talent: [u8; 4],
}

impl TolInputs {
    /// SPEC: canon §5.4. An Erratum's terms from its Origin genre and ATN.
    #[must_use]
    pub const fn erratum(origin: Genre, atn: i32, gear: [u8; 4], talent: [u8; 4]) -> TolInputs {
        TolInputs {
            origin: origin.origin_tol(),
            attune: attune_tol(atn),
            gear,
            talent,
        }
    }

    /// SPEC: canon §5.4. An NPC creature: `origin_tol = 2` on every axis of its
    /// own origin genre at or above 5, and `attune_tol = 0`.
    #[must_use]
    pub const fn npc(origin: Genre) -> TolInputs {
        TolInputs {
            origin: origin.npc_origin_tol(),
            attune: 0,
            gear: [0; 4],
            talent: [0; 4],
        }
    }

    /// SPEC: canon §5.4, §5.10. `Tol[a]`, with the World Law grants for this
    /// component's class added before the cap of 6.
    #[must_use]
    pub const fn total(&self, law_bonus: [u8; 4]) -> [u8; 4] {
        let mut out = [0u8; 4];
        let mut i = 0;
        while i < 4 {
            let sum = self.origin[i] as u32
                + self.attune as u32
                + self.gear[i] as u32
                + self.talent[i] as u32
                + law_bonus[i] as u32;
            out[i] = if sum > TOL_CAP as u32 {
                TOL_CAP
            } else {
                sum as u8
            };
            i += 1;
        }
        out
    }
}

// ---------------------------------------------------------------- §5.8

/// SPEC: canon §5.8 `local_genre(pos)`: one genre, or a seam zone stitching
/// the Folio's genre to another genre.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum LocalGenre {
    /// A bleed zone's genre, or the Folio's genre outside bleed zones.
    Plain(Genre),
    /// A `zone.seam`: per-axis maximum axioms, both genres' Laws active,
    /// either genre native, skins from the Folio's genre.
    Seam {
        /// The Folio's genre (supplies skins).
        folio: Genre,
        /// The genre stitched in.
        other: Genre,
    },
}

impl LocalGenre {
    /// SPEC: canon §5.8 `local_axiom(pos, ·)` for all four axes; in a seam,
    /// the larger of the two genres' values on each axis.
    #[must_use]
    pub const fn axioms(self) -> Axioms {
        match self {
            LocalGenre::Plain(g) => g.axioms(),
            LocalGenre::Seam { folio, other } => folio.axioms().max_each(other.axioms()),
        }
    }

    /// SPEC: canon §5.8 `local_axiom(pos, a)`.
    #[must_use]
    pub const fn axiom(self, a: Axiom) -> u8 {
        self.axioms().get(a)
    }

    /// SPEC: canon §5.8, §5.10. True when `g` is the local genre or one of the
    /// two seam genres (so its World Laws are active here).
    #[must_use]
    pub const fn includes(self, g: Genre) -> bool {
        match self {
            LocalGenre::Plain(h) => h as u8 == g as u8,
            LocalGenre::Seam { folio, other } => folio as u8 == g as u8 || other as u8 == g as u8,
        }
    }

    /// SPEC: canon §5.6, §5.8. A target is native when its origin genre is the
    /// local genre at its position; in a seam, entities of either genre are native.
    #[must_use]
    pub const fn is_native(self, origin: Genre) -> bool {
        self.includes(origin)
    }

    /// SPEC: canon §5.8, §1.10. The genre that supplies skins (the Folio's
    /// genre in a seam).
    #[must_use]
    pub const fn skin_genre(self) -> Genre {
        match self {
            LocalGenre::Plain(g) => g,
            LocalGenre::Seam { folio, .. } => folio,
        }
    }

    /// SPEC: canon §5.7 AETHER row: "the local genre's aether_axiom". In a
    /// seam this crate uses the Folio genre's (canon is silent; see the
    /// module's reported spec issues).
    #[must_use]
    pub const fn aether_axiom(self) -> Option<Axiom> {
        self.skin_genre().aether_axiom()
    }
}

/// SPEC: canon §5.8 `vrv(e) = min(10, local_axiom(e.pos, verve) + 2 × [e has st.inspired])`.
#[must_use]
pub const fn vrv(local: LocalGenre, inspired: bool) -> u8 {
    let v = local.axiom(Axiom::Verve) + if inspired { 2 } else { 0 };
    if v > 10 { 10 } else { v }
}

// ---------------------------------------------------------------- §5.10

/// SPEC: canon §5.10, §1.1. A World Law's content ID as `id32`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LawId(pub u32);

impl LawId {
    /// SPEC: canon §1.1 `id32 = fnv1a32(utf8(id))` of a `law.<genre_suffix>.<name>` ID.
    #[must_use]
    pub const fn from_id(id: &str) -> LawId {
        LawId(fnv1a32(id.as_bytes()))
    }
}

/// SPEC: canon §5.10 `component_class` of a `law_grants` entry, as `id32` of
/// its name (for example `gadget`). Doc 12 owns the class vocabulary.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ComponentClass(pub u32);

impl ComponentClass {
    /// SPEC: canon §5.10. The class named `name`.
    #[must_use]
    pub const fn from_name(name: &str) -> ComponentClass {
        ComponentClass(fnv1a32(name.as_bytes()))
    }
}

/// SPEC: canon §5.10 `law_grants: [{component_class, axis, tol_bonus}]` entry.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LawGrant {
    /// Components of this class receive the bonus.
    pub class: ComponentClass,
    /// Axis whose tolerance rises.
    pub axis: Axiom,
    /// Added to `Tol[axis]` before the cap of 6.
    pub tol_bonus: u8,
}

/// SPEC: canon §5.10. A World Law as data: ID, genre and tolerance grants.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct WorldLaw<'a> {
    /// `law.<genre_suffix>.<name>` as `id32`.
    pub id: LawId,
    /// The Law is active where this genre is local (or in a seam including it).
    pub genre: Genre,
    /// `law_grants`; empty for Laws without grants.
    pub grants: &'a [LawGrant],
}

/// SPEC: canon §5.10 `law_bound: <law ID>` target: the Law's ID and genre.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LawRef {
    /// The Law's `id32`.
    pub id: LawId,
    /// The Law's genre.
    pub genre: Genre,
}

/// SPEC: canon §5.10. A Law is active wherever the local genre is its genre,
/// and in seam zones that include its genre.
#[must_use]
pub const fn law_active(law_genre: Genre, local: LocalGenre) -> bool {
    local.includes(law_genre)
}

/// SPEC: canon §5.10, §1.1. The genre of a `law.<genre_suffix>.<name>` ID.
#[must_use]
pub fn law_genre_from_id(id: &str) -> Option<Genre> {
    let rest = id.strip_prefix("law.")?;
    let (suffix, name) = rest.split_once('.')?;
    if name.is_empty() {
        return None;
    }
    Genre::ALL.into_iter().find(|g| g.suffix() == suffix)
}

/// SPEC: canon §5.10. Sum of `tol_bonus` per axis over every grant of every
/// active Law whose `component_class` is `class` (saturating; the cap of 6 is
/// applied by [`TolInputs::total`]).
#[must_use]
pub fn law_tol_bonus(laws: &[WorldLaw<'_>], local: LocalGenre, class: ComponentClass) -> [u8; 4] {
    let mut out = [0u8; 4];
    for law in laws.iter().filter(|l| law_active(l.genre, local)) {
        for g in law.grants.iter().filter(|g| g.class == class) {
            let slot = &mut out[g.axis.index()];
            *slot = slot.saturating_add(g.tol_bonus);
        }
    }
    out
}

/// SPEC: canon §5.10. The Laws the canon requires doc 12 to define.
pub const REQUIRED_LAWS: [&str; 7] = [
    "law.wuxia.qinggong",
    "law.weird_west.high_noon",
    "law.steampunk.weird_science",
    "law.gothic_horror.dread_escalation",
    "law.cosmic_horror.dread_escalation",
    "law.redacted.dread_escalation",
    "law.hardboiled_noir.void_rules",
];

/// SPEC: canon §5.10 example grant: Steampunk's weird-science Law grants +2
/// TECHNE tolerance to gadgets.
pub const WEIRD_SCIENCE_GRANTS: [LawGrant; 1] = [LawGrant {
    class: ComponentClass::from_name("gadget"),
    axis: Axiom::Techne,
    tol_bonus: 2,
}];

/// SPEC: canon §5.10 `law.steampunk.weird_science` as data.
pub const LAW_STEAMPUNK_WEIRD_SCIENCE: WorldLaw<'static> = WorldLaw {
    id: LawId::from_id("law.steampunk.weird_science"),
    genre: Genre::Steampunk,
    grants: &WEIRD_SCIENCE_GRANTS,
};

// ---------------------------------------------------------------- §5.5, §5.6

/// SPEC: canon §5.3, §5.5, §5.10. One component of an item or ability, as the
/// Contradiction check sees it.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Component {
    /// Dependency set from the genre skin.
    pub dep: DepSet,
    /// Genre where the item was generated or last Rewritten; a creature's
    /// origin genre for innate abilities.
    pub origin: Genre,
    /// Class for World Law grants; `None` receives no grant.
    pub class: Option<ComponentClass>,
    /// `law_bound`: Disconnected wherever that Law is inactive.
    pub law_bound: Option<LawRef>,
    /// Innate ability (body function, Origin trait, Echo trait).
    pub innate: bool,
    /// Part of a Palimpsestic item.
    pub palimpsestic: bool,
}

impl Component {
    /// SPEC: canon §5.3. A plain component: no class, Law binding, innate or
    /// Palimpsestic flag.
    #[must_use]
    pub const fn new(dep: DepSet, origin: Genre) -> Component {
        Component {
            dep,
            origin,
            class: None,
            law_bound: None,
            innate: false,
            palimpsestic: false,
        }
    }
}

/// SPEC: canon §5.5 `State` column.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ContradictionState {
    /// C = 0.
    Coherent,
    /// C = 1 or 2.
    Strained,
    /// C = 3 to 5.
    Contradicted,
    /// C >= 6: cannot activate, passive bonuses off, torn-page icon.
    Disconnected,
}

/// SPEC: canon §5.5. One row of the potency and failure table.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ContradictionRow {
    /// State of the row.
    pub state: ContradictionState,
    /// Multiplies everything the component produces.
    pub potency_bp: i32,
    /// Failure chance per activation of the base-function component; 0 when
    /// none is rolled. A Disconnected component is inert (never activates).
    pub failure_bp: i32,
}

/// SPEC: canon §5.5 table for Contradiction `c`, with the §5.11 Unmoored rule:
/// while `st.unmoored` lasts, Strained components use the C = 3 row.
#[must_use]
pub const fn contradiction_row(c: u8, unmoored: bool) -> ContradictionRow {
    let c = if unmoored && (c == 1 || c == 2) { 3 } else { c };
    let (state, potency_bp, failure_bp) = match c {
        0 => (ContradictionState::Coherent, 10_000, 0),
        1 => (ContradictionState::Strained, 9000, 0),
        2 => (ContradictionState::Strained, 8000, 0),
        3 => (ContradictionState::Contradicted, 7000, 500),
        4 => (ContradictionState::Contradicted, 6000, 1000),
        5 => (ContradictionState::Contradicted, 5000, 1500),
        _ => (ContradictionState::Disconnected, 0, 0),
    };
    ContradictionRow {
        state,
        potency_bp,
        failure_bp,
    }
}

/// SPEC: canon §5.5 overrides, checked before the formula.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ContradictionBasis {
    /// `genre.ur_text`: every component has C = 0 and E = 0.
    UrText,
    /// `genre.unwritten`: innate abilities have C = 0 and E = 0.
    UnwrittenInnate,
    /// Palimpsestic items: C = 0 everywhere, never Edge.
    Palimpsestic,
    /// §5.10: the component's `law_bound` Law is inactive: C = 6.
    LawInactive,
    /// The §5.5 formula.
    Formula,
}

/// SPEC: canon §5.5, §5.6. Result of the Contradiction check for one component.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Contradiction {
    /// `C`.
    pub c: u8,
    /// Anachronism Edge `E`, `0..=6`.
    pub e: u8,
    /// `raw_gap[a] = max(0, Req[a] - local_axiom[a])` (zero under an override).
    pub raw_gap: [u8; 4],
    /// `gap[a] = max(0, raw_gap[a] - Tol[a])` (zero under an override).
    pub gap: [u8; 4],
    /// Which rule decided `C`.
    pub basis: ContradictionBasis,
}

impl Contradiction {
    /// SPEC: canon §5.5, §5.11. The table row (potency and failure) for `C`.
    #[must_use]
    pub const fn row(&self, unmoored: bool) -> ContradictionRow {
        contradiction_row(self.c, unmoored)
    }

    /// SPEC: canon §5.6 `edge_bp = E × 400`.
    #[must_use]
    pub const fn edge_bp(&self) -> i32 {
        edge_bp(self.e)
    }
}

/// SPEC: canon §5.6. Anachronism Edge is at most 6 (+24%).
pub const EDGE_MAX: u8 = 6;

/// SPEC: canon §5.6 `edge_bp = E × 400`.
#[must_use]
pub const fn edge_bp(e: u8) -> i32 {
    e as i32 * 400
}

/// SPEC: canon §5.6. Damage and buildup factor of the base-function component:
/// `10000 + edge_bp` against a native target, 10000 otherwise.
#[must_use]
pub const fn edge_multiplier_bp(e: u8, target_native: bool) -> i32 {
    if target_native {
        10_000 + edge_bp(e)
    } else {
        10_000
    }
}

/// SPEC: canon §5.6 Shock of the New: the first hit from a base-function
/// component with `E >= 3` on each native non-boss target applies
/// `st.staggered`. `first_hit` means the `(item uid, target)` pair is not yet
/// in the Folio's tracking set.
#[must_use]
pub const fn shock_of_the_new(
    e: u8,
    target_native: bool,
    target_is_boss: bool,
    first_hit: bool,
) -> bool {
    e >= 3 && target_native && !target_is_boss && first_hit
}

/// SPEC: canon §5.5 formula given final tolerances (World Law grants and the
/// cap already applied), after the overrides and the §5.10 Law binding.
#[must_use]
pub fn evaluate(component: &Component, local: LocalGenre, tol: [u8; 4]) -> Contradiction {
    let zero = |basis| Contradiction {
        c: 0,
        e: 0,
        raw_gap: [0; 4],
        gap: [0; 4],
        basis,
    };
    if local.includes(Genre::UrText) {
        return zero(ContradictionBasis::UrText);
    }
    if component.innate && local.includes(Genre::Unwritten) {
        return zero(ContradictionBasis::UnwrittenInnate);
    }
    if component.palimpsestic {
        return zero(ContradictionBasis::Palimpsestic);
    }
    let req = requirement(component.dep, component.origin);
    let ax = local.axioms().0;
    let mut raw_gap = [0u8; 4];
    let mut gap = [0u8; 4];
    for i in 0..4 {
        raw_gap[i] = req[i].saturating_sub(ax[i]);
        gap[i] = raw_gap[i].saturating_sub(tol[i]);
    }
    if let Some(law) = component.law_bound
        && !law_active(law.genre, local)
    {
        return Contradiction {
            c: 6,
            e: 0,
            raw_gap,
            gap,
            basis: ContradictionBasis::LawInactive,
        };
    }
    let c = gap.iter().copied().max().unwrap_or(0);
    let max_raw = component
        .dep
        .axes()
        .map(|a| raw_gap[a.index()])
        .max()
        .unwrap_or(0);
    let e = if c <= 5 { max_raw.min(EDGE_MAX) } else { 0 };
    Contradiction {
        c,
        e,
        raw_gap,
        gap,
        basis: ContradictionBasis::Formula,
    }
}

/// SPEC: canon §5.4, §5.5, §5.10. Full check for a wielder: tolerance terms
/// plus the grants of the active `laws` for the component's class, capped at 6.
#[must_use]
pub fn evaluate_for(
    component: &Component,
    local: LocalGenre,
    tol: &TolInputs,
    laws: &[WorldLaw<'_>],
) -> Contradiction {
    let bonus = match component.class {
        Some(class) => law_tol_bonus(laws, local, class),
        None => [0; 4],
    };
    evaluate(component, local, tol.total(bonus))
}

/// SPEC: canon §5.5 per-component scope: an affix, perk or augment
/// contributes `mul_bp(value, its own potency)` (0 when Disconnected).
#[must_use]
pub const fn apply_potency(value: i64, potency_bp: i32) -> i64 {
    mul_bp(value, potency_bp)
}

/// SPEC: canon §5.5. Failure is rolled only when its chance is above 0, with
/// keyed `rng.combat` (one `range(10000)` draw); otherwise nothing is drawn.
pub fn roll_failure(rng: &mut Rng, row: &ContradictionRow) -> bool {
    row.failure_bp > 0 && rng.roll_bp(row.failure_bp)
}

/// SPEC: canon §5.5. Weapons with a fire interval under this many ticks roll
/// failure on the first shot of each trigger hold and then once per this
/// many ticks of continuous fire.
pub const FAILURE_CADENCE_TICKS: u32 = 30;

/// SPEC: canon §5.5 failure cadence. Single-shot, semi-automatic, cast and use
/// actions (interval at or above 30 ticks) roll every activation; faster
/// weapons roll on the first shot of a hold, then when at least 30 ticks of
/// continuous fire have passed since the last roll.
#[must_use]
pub const fn failure_roll_due(
    fire_interval_ticks: u32,
    first_shot_of_hold: bool,
    ticks_since_last_roll: u32,
) -> bool {
    fire_interval_ticks >= FAILURE_CADENCE_TICKS
        || first_shot_of_hold
        || ticks_since_last_roll >= FAILURE_CADENCE_TICKS
}

// ---------------------------------------------------------------- §5.11

/// SPEC: canon §5.11. Failures that trigger `st.unmoored`.
pub const UNMOORED_FAILURES: usize = 3;
/// SPEC: canon §5.11. Window, in ticks, in which those failures must fall.
pub const UNMOORED_WINDOW_TICKS: u64 = 600;
/// SPEC: canon §5.11. Duration of `st.unmoored` in ticks.
pub const UNMOORED_DURATION_TICKS: u32 = 300;

/// SPEC: canon §5.11. True when at least three of one character's
/// Contradiction failure ticks lie in `(now - 600) ..= now`.
#[must_use]
pub fn unmoored_triggered(failure_ticks: &[u64], now: u64) -> bool {
    let lo = now.saturating_sub(UNMOORED_WINDOW_TICKS);
    failure_ticks
        .iter()
        .filter(|&&t| t >= lo && t <= now)
        .count()
        >= UNMOORED_FAILURES
}

// ---------------------------------------------------------------- §5.7

/// SPEC: canon §5.7 AETHER term in Bp: `10000 + (local_axiom[aa] - 5) × 1000`
/// where `aa` is the local genre's `aether_axiom`; 10000 when it is none.
#[must_use]
pub const fn aether_regen_term_bp(local: LocalGenre) -> i32 {
    match local.aether_axiom() {
        Some(a) => 10_000 + (local.axiom(a) as i32 - 5) * 1000,
        None => 10_000,
    }
}

/// SPEC: canon §5.7 GLYPH term in Bp: `10000 + (local_axiom[a] - 5) × 200`
/// for the glyph's single dependency axis `a`; 10000 if its dependency is empty.
#[must_use]
pub const fn glyph_term_bp(glyph_axis: Option<Axiom>, local: LocalGenre) -> i32 {
    match glyph_axis {
        Some(a) => 10_000 + (local.axiom(a) as i32 - 5) * 200,
        None => 10_000,
    }
}

/// SPEC: canon §5.7 TECHNE recharge term: `10000 + (TEC - 5) × 1000` Bp.
#[must_use]
pub const fn techne_recharge_term_bp(local: LocalGenre) -> i32 {
    10_000 + (local.axiom(Axiom::Techne) as i32 - 5) * 1000
}

/// SPEC: canon §5.7. Worldgen places electronic locks, cameras and turrets
/// only where TECHNE is above 2.
#[must_use]
pub const fn electronics_allowed(genre: Genre) -> bool {
    genre.axiom(Axiom::Techne) > 2
}

/// SPEC: canon §5.7 MYTHOS damage term: `10000 + (MYT - 5) × 300` Bp when the
/// attacker or the target is tagged undead, fiend, divine or eldritch (applied
/// once even if both are); 10000 otherwise.
#[must_use]
pub const fn mythos_damage_term_bp(local: LocalGenre, tagged: bool) -> i32 {
    if tagged {
        10_000 + (local.axiom(Axiom::Mythos) as i32 - 5) * 300
    } else {
        10_000
    }
}

/// SPEC: canon §5.7 MYTHOS curse-chance term: `10000 + (MYT - 5) × 1500` Bp.
#[must_use]
pub const fn mythos_curse_chance_term_bp(local: LocalGenre) -> i32 {
    10_000 + (local.axiom(Axiom::Mythos) as i32 - 5) * 1500
}

/// SPEC: canon §5.7 MYTHOS composure-loss term: `10000 + (MYT - 5) × 1000` Bp.
#[must_use]
pub const fn mythos_composure_loss_term_bp(local: LocalGenre) -> i32 {
    10_000 + (local.axiom(Axiom::Mythos) as i32 - 5) * 1000
}

/// SPEC: canon §5.7 VERVE crit term `(vrv(e) - 5) × 100` Bp (§6.4 row 13).
#[must_use]
pub const fn verve_crit_term_bp(vrv: u8) -> i32 {
    (vrv as i32 - 5) * 100
}

/// SPEC: canon §5.7 VERVE Death's Door factor `10000 + (vrv(e) - 5) × 1000` Bp
/// (§6.4 row 43).
#[must_use]
pub const fn verve_deaths_door_term_bp(vrv: u8) -> i32 {
    10_000 + (vrv as i32 - 5) * 1000
}

#[cfg(test)]
mod tests {
    use super::*;
    use palimpsest_math::Rng;
    use proptest::prelude::*;

    fn smart_pistol() -> Component {
        Component::new(DepSet::single(Axiom::Techne), Genre::Cyberpunk)
    }

    /// SPEC: canon §21 test 14 (the §5.6 worked example): a Cyberpunk smart
    /// pistol in Fairy Tale, Cyberpunk Origin, ATN 60: C = 4, potency 6000,
    /// failure 1000, `edge_bp` 2400; damage factor against a native is 0.744.
    #[test]
    fn canon_t14_contradiction_example() {
        let pistol = smart_pistol();
        assert_eq!(requirement(pistol.dep, pistol.origin), [0, 7, 0, 0]);
        let tol = TolInputs::erratum(Genre::Cyberpunk, 60, [0; 4], [0; 4]);
        assert_eq!(tol.attune, 0);
        assert_eq!(tol.total([0; 4])[Axiom::Techne.index()], 2);
        let local = LocalGenre::Plain(Genre::FairyTale);
        let k = evaluate_for(&pistol, local, &tol, &[]);
        assert_eq!(k.raw_gap[Axiom::Techne.index()], 6);
        assert_eq!(k.gap[Axiom::Techne.index()], 4);
        assert_eq!(k.c, 4);
        assert_eq!(k.basis, ContradictionBasis::Formula);
        let row = k.row(false);
        assert_eq!(row.state, ContradictionState::Contradicted);
        assert_eq!(row.potency_bp, 6000);
        assert_eq!(row.failure_bp, 1000);
        assert_eq!(k.e, 6);
        assert_eq!(k.edge_bp(), 2400);
        // A native wolf: 0.60 × 1.24 = 0.744.
        assert!(local.is_native(Genre::FairyTale));
        let factor = mul_bp(
            mul_bp(10_000, row.potency_bp),
            edge_multiplier_bp(k.e, true),
        );
        assert_eq!(factor, 7440);
        assert!(shock_of_the_new(k.e, true, false, true));
    }

    /// SPEC: canon §21 test 15: a Fairy Tale wand (`dep = {arcana}`, `Req = 8`)
    /// in Hardboiled Noir, Fairy Tale Origin, ATN 40: `raw_gap = 8`,
    /// `Tol = 2`, `C = 6`, Disconnected.
    #[test]
    fn canon_t15_disconnection() {
        let dep = default_dep(ComponentKind::Casting, Genre::FairyTale);
        assert_eq!(dep, DepSet::single(Axiom::Arcana));
        let wand = Component::new(dep, Genre::FairyTale);
        assert_eq!(requirement(dep, Genre::FairyTale)[0], 8);
        let tol = TolInputs::erratum(Genre::FairyTale, 40, [0; 4], [0; 4]);
        assert_eq!(tol.total([0; 4])[0], 2);
        let k = evaluate_for(&wand, LocalGenre::Plain(Genre::HardboiledNoir), &tol, &[]);
        assert_eq!(k.raw_gap[0], 8);
        assert_eq!(k.c, 6);
        assert_eq!(k.e, 0);
        let row = k.row(false);
        assert_eq!(row.state, ContradictionState::Disconnected);
        assert_eq!(row.potency_bp, 0);
        assert_eq!(apply_potency(5000, row.potency_bp), 0);
    }

    /// SPEC: canon §21 test 36: a glyph generated in Cyberpunk has
    /// `dep = {techne}`, `Req[techne] = 7`; in Mythic Fantasy `dep = {arcana}`,
    /// `Req[arcana] = 6`.
    #[test]
    fn canon_t36_glyph_dependency() {
        let cp = default_dep(ComponentKind::Casting, Genre::Cyberpunk);
        assert_eq!(cp, DepSet::single(Axiom::Techne));
        assert_eq!(requirement(cp, Genre::Cyberpunk), [0, 7, 0, 0]);
        let mf = default_dep(ComponentKind::Casting, Genre::MythicFantasy);
        assert_eq!(mf, DepSet::single(Axiom::Arcana));
        assert_eq!(requirement(mf, Genre::MythicFantasy), [6, 0, 0, 0]);
        // aether_axiom none falls back to arcana; Wuxia casts on VERVE.
        assert_eq!(
            default_dep(ComponentKind::Casting, Genre::UrText),
            DepSet::single(Axiom::Arcana)
        );
        assert_eq!(
            default_dep(ComponentKind::Casting, Genre::Wuxia),
            DepSet::single(Axiom::Verve)
        );
    }

    /// SPEC: canon §5.3 default dependency table.
    #[test]
    fn default_dep_table() {
        let g = Genre::Steampunk;
        assert!(default_dep(ComponentKind::Mundane, g).is_empty());
        assert!(default_dep(ComponentKind::MundaneAlchemy, g).is_empty());
        assert!(default_dep(ComponentKind::KinOrBeast, g).is_empty());
        assert_eq!(
            default_dep(ComponentKind::Tech, g).only(),
            Some(Axiom::Techne)
        );
        assert_eq!(
            default_dep(ComponentKind::Enchantment, g).only(),
            Some(Axiom::Arcana)
        );
        assert_eq!(
            default_dep(ComponentKind::Sacred, g).only(),
            Some(Axiom::Mythos)
        );
        assert_eq!(
            default_dep(ComponentKind::Cinematic, g).only(),
            Some(Axiom::Verve)
        );
        let glyph = DepSet::single(Axiom::Mythos);
        assert_eq!(
            default_dep(ComponentKind::Summon { glyph_dep: glyph }, g),
            glyph
        );
        let mounts = [
            (MountSkin::Vehicle, Axiom::Techne),
            (MountSkin::BroomOrCarpet, Axiom::Arcana),
            (MountSkin::DivineSteed, Axiom::Mythos),
            (MountSkin::Cloud, Axiom::Verve),
        ];
        for (skin, axis) in mounts {
            assert_eq!(
                default_dep(ComponentKind::Mount(skin), g).only(),
                Some(axis)
            );
        }
    }

    /// SPEC: canon §5.3 requirement floor of 1 and the worldgen roll rule.
    #[test]
    fn requirement_floor_and_rollable() {
        let tech = DepSet::single(Axiom::Techne);
        assert_eq!(requirement(tech, Genre::FairyTale), [0, 1, 0, 0]); // max(1, 1 - 2)
        assert_eq!(requirement(tech, Genre::SpaceOpera), [0, 8, 0, 0]);
        assert_eq!(requirement(DepSet::NONE, Genre::SpaceOpera), [0; 4]);
        let both = DepSet::single(Axiom::Arcana).with(Axiom::Techne);
        assert_eq!(requirement(both, Genre::Oneiric), [7, 3, 0, 0]);
        assert!(!component_rollable(
            DepSet::single(Axiom::Arcana),
            Genre::HardboiledNoir
        ));
        assert!(component_rollable(
            DepSet::single(Axiom::Techne),
            Genre::HardboiledNoir
        ));
        assert!(component_rollable(DepSet::NONE, Genre::Unwritten));
        assert!(!component_rollable(
            DepSet::single(Axiom::Verve),
            Genre::Redacted
        ));
    }

    /// SPEC: canon §5.4 `attune_tol = clamp(floor_div(ATN - 40, 30), 0, 3)`.
    #[test]
    fn attune_tol_values() {
        assert_eq!(attune_tol(1), 0);
        assert_eq!(attune_tol(40), 0);
        assert_eq!(attune_tol(60), 0);
        assert_eq!(attune_tol(69), 0);
        assert_eq!(attune_tol(70), 1);
        assert_eq!(attune_tol(100), 2);
        assert_eq!(attune_tol(130), 3);
        assert_eq!(attune_tol(150), 3);
    }

    /// SPEC: canon §5.4 sum and cap of 6; §5.10 grants count before the cap.
    #[test]
    fn tolerance_cap() {
        let t = TolInputs {
            origin: [2, 0, 1, 1],
            attune: 3,
            gear: [1, 0, 0, 0],
            talent: [1, 0, 0, 0],
        };
        assert_eq!(t.total([0; 4]), [6, 3, 4, 4]);
        assert_eq!(t.total([0, 2, 0, 9]), [6, 5, 4, 6]);
        let npc = TolInputs::npc(Genre::Cyberpunk);
        assert_eq!(npc.total([0; 4]), [0, 2, 0, 2]);
    }

    /// SPEC: canon §5.5 table and the §5.11 Unmoored rule.
    #[test]
    fn potency_failure_table() {
        let expect = [
            (10_000, 0),
            (9000, 0),
            (8000, 0),
            (7000, 500),
            (6000, 1000),
            (5000, 1500),
            (0, 0),
            (0, 0),
        ];
        for (c, (p, f)) in expect.into_iter().enumerate() {
            let row = contradiction_row(c as u8, false);
            assert_eq!((row.potency_bp, row.failure_bp), (p, f), "C = {c}");
        }
        assert_eq!(
            contradiction_row(1, false).state,
            ContradictionState::Strained
        );
        assert_eq!(
            contradiction_row(6, false).state,
            ContradictionState::Disconnected
        );
        assert_eq!(
            contradiction_row(9, false).state,
            ContradictionState::Disconnected
        );
        // Unmoored: Strained uses the C = 3 row; others unchanged.
        assert_eq!(contradiction_row(1, true), contradiction_row(3, false));
        assert_eq!(contradiction_row(2, true), contradiction_row(3, false));
        assert_eq!(contradiction_row(0, true), contradiction_row(0, false));
        assert_eq!(contradiction_row(4, true), contradiction_row(4, false));
    }

    /// SPEC: canon §5.5 overrides: Ur-Text, Unwritten innate, Palimpsestic.
    #[test]
    fn overrides() {
        let tol = TolInputs::default();
        let pistol = smart_pistol();
        let k = evaluate_for(&pistol, LocalGenre::Plain(Genre::UrText), &tol, &[]);
        assert_eq!((k.c, k.e, k.basis), (0, 0, ContradictionBasis::UrText));
        // Unwritten: only innate abilities are exempt.
        let k = evaluate_for(&pistol, LocalGenre::Plain(Genre::Unwritten), &tol, &[]);
        assert_eq!((k.c, k.e), (7, 0));
        let innate = Component {
            innate: true,
            ..pistol
        };
        let k = evaluate_for(&innate, LocalGenre::Plain(Genre::Unwritten), &tol, &[]);
        assert_eq!(
            (k.c, k.e, k.basis),
            (0, 0, ContradictionBasis::UnwrittenInnate)
        );
        // Palimpsestic: C = 0 everywhere and never Edge.
        let pal = Component {
            palimpsestic: true,
            ..pistol
        };
        for g in Genre::ALL {
            let k = evaluate_for(&pal, LocalGenre::Plain(g), &tol, &[]);
            assert_eq!((k.c, k.e), (0, 0));
        }
    }

    /// SPEC: canon §5.10 `law_bound`: Disconnected where the Law is inactive,
    /// whatever the axioms say; active in its genre and in seams including it.
    #[test]
    fn law_bound_components() {
        let qinggong = LawRef {
            id: LawId::from_id("law.wuxia.qinggong"),
            genre: Genre::Wuxia,
        };
        let tech = Component {
            law_bound: Some(qinggong),
            ..Component::new(DepSet::single(Axiom::Verve), Genre::Wuxia)
        };
        let full = TolInputs {
            attune: 6,
            ..TolInputs::default()
        };
        let k = evaluate_for(&tech, LocalGenre::Plain(Genre::MythicAntiquity), &full, &[]);
        assert_eq!((k.c, k.e, k.basis), (6, 0, ContradictionBasis::LawInactive));
        let k = evaluate_for(&tech, LocalGenre::Plain(Genre::Wuxia), &full, &[]);
        assert_eq!((k.c, k.basis), (0, ContradictionBasis::Formula));
        let seam = LocalGenre::Seam {
            folio: Genre::Steampunk,
            other: Genre::Wuxia,
        };
        assert_eq!(evaluate_for(&tech, seam, &full, &[]).c, 0);
        assert_eq!(law_genre_from_id("law.wuxia.qinggong"), Some(Genre::Wuxia));
        assert_eq!(
            law_genre_from_id("law.hardboiled_noir.void_rules"),
            Some(Genre::HardboiledNoir)
        );
        assert_eq!(law_genre_from_id("law.nowhere.x"), None);
        assert_eq!(law_genre_from_id("law.wuxia"), None);
        for id in REQUIRED_LAWS {
            assert!(law_genre_from_id(id).is_some(), "{id}");
        }
    }

    /// SPEC: canon §5.10 `law_grants`: Steampunk's weird science grants +2
    /// TECHNE tolerance to gadgets, before the cap, only where active.
    #[test]
    fn law_grants() {
        let laws = [LAW_STEAMPUNK_WEIRD_SCIENCE];
        let gadget = ComponentClass::from_name("gadget");
        let rifle = ComponentClass::from_name("firearm");
        let sp = LocalGenre::Plain(Genre::Steampunk);
        assert_eq!(law_tol_bonus(&laws, sp, gadget), [0, 2, 0, 0]);
        assert_eq!(law_tol_bonus(&laws, sp, rifle), [0; 4]);
        assert_eq!(
            law_tol_bonus(&laws, LocalGenre::Plain(Genre::Wuxia), gadget),
            [0; 4]
        );
        let seam = LocalGenre::Seam {
            folio: Genre::Wuxia,
            other: Genre::Steampunk,
        };
        assert_eq!(law_tol_bonus(&laws, seam, gadget), [0, 2, 0, 0]);
        // A Space Opera gadget (Req[techne] = 8) in Steampunk (TEC 7) with a
        // Fairy Tale Origin (techne tolerance 0): raw_gap 1; the grant clears it.
        let mut drone = Component::new(DepSet::single(Axiom::Techne), Genre::SpaceOpera);
        let tol = TolInputs::erratum(Genre::FairyTale, 40, [0; 4], [0; 4]);
        assert_eq!(evaluate_for(&drone, sp, &tol, &laws).c, 1);
        drone.class = Some(gadget);
        let k = evaluate_for(&drone, sp, &tol, &laws);
        assert_eq!((k.c, k.e), (0, 1));
    }

    /// SPEC: canon §5.8 seam zones: per-axis maximum, both genres native,
    /// Folio genre supplies skins.
    #[test]
    fn seam_zones() {
        let seam = LocalGenre::Seam {
            folio: Genre::Cyberpunk,
            other: Genre::FairyTale,
        };
        assert_eq!(seam.axioms(), Axioms([10, 9, 8, 5]));
        assert!(seam.is_native(Genre::Cyberpunk) && seam.is_native(Genre::FairyTale));
        assert!(!seam.is_native(Genre::Wuxia));
        assert_eq!(seam.skin_genre(), Genre::Cyberpunk);
        // Both toolsets work at full strength: the pistol is Coherent.
        let tol = TolInputs::default();
        let k = evaluate_for(&smart_pistol(), seam, &tol, &[]);
        assert_eq!((k.c, k.e), (0, 0));
        assert!(!LocalGenre::Plain(Genre::FairyTale).is_native(Genre::Cyberpunk));
    }

    /// SPEC: canon §5.8 `vrv(e)`.
    #[test]
    fn personal_verve() {
        assert_eq!(vrv(LocalGenre::Plain(Genre::Cyberpunk), false), 5);
        assert_eq!(vrv(LocalGenre::Plain(Genre::Cyberpunk), true), 7);
        assert_eq!(vrv(LocalGenre::Plain(Genre::Wuxia), true), 10);
        assert_eq!(vrv(LocalGenre::Plain(Genre::Oneiric), true), 10);
        assert_eq!(vrv(LocalGenre::Plain(Genre::Redacted), true), 2);
    }

    /// SPEC: canon §5.6 Edge, multiplier and Shock of the New conditions.
    #[test]
    fn anachronism_edge() {
        assert_eq!(edge_bp(0), 0);
        assert_eq!(edge_bp(6), 2400);
        assert_eq!(edge_multiplier_bp(6, true), 12_400);
        assert_eq!(edge_multiplier_bp(6, false), 10_000);
        assert!(shock_of_the_new(3, true, false, true));
        assert!(!shock_of_the_new(2, true, false, true));
        assert!(!shock_of_the_new(6, false, false, true));
        assert!(!shock_of_the_new(6, true, true, true));
        assert!(!shock_of_the_new(6, true, false, false));
        // Edge is capped at 6 even when raw_gap is 8.
        let wand = Component::new(DepSet::single(Axiom::Arcana), Genre::FairyTale);
        let tol = TolInputs {
            attune: 3,
            origin: [2, 0, 0, 0],
            ..TolInputs::default()
        };
        let k = evaluate_for(&wand, LocalGenre::Plain(Genre::HardboiledNoir), &tol, &[]);
        assert_eq!((k.c, k.e), (3, 6));
    }

    /// SPEC: canon §5.5: failure draws exactly one value when its chance is
    /// above 0 and none otherwise.
    #[test]
    fn failure_roll_draw_count() {
        let mut a = Rng::from_seed64(9);
        let before = a.clone();
        assert!(!roll_failure(&mut a, &contradiction_row(2, false)));
        assert_eq!(a, before);
        let _ = roll_failure(&mut a, &contradiction_row(4, false));
        let mut b = before.clone();
        let _ = b.range(10_000);
        assert_eq!(a, b);
        assert!(!roll_failure(&mut a, &contradiction_row(6, false)));
        assert_eq!(a, b);
    }

    /// SPEC: canon §5.5 failure cadence for automatic weapons.
    #[test]
    fn failure_cadence() {
        assert!(failure_roll_due(30, false, 0));
        assert!(failure_roll_due(45, false, 0));
        assert!(failure_roll_due(6, true, 0));
        assert!(!failure_roll_due(6, false, 24));
        assert!(failure_roll_due(6, false, 30));
    }

    /// SPEC: canon §5.11 three failures within 600 ticks.
    #[test]
    fn unmoored_window() {
        assert!(unmoored_triggered(&[400, 700, 1000], 1000));
        assert!(!unmoored_triggered(&[399, 700, 1000], 1000));
        assert!(!unmoored_triggered(&[900, 1000], 1000));
        assert!(unmoored_triggered(&[0, 1, 2], 10));
    }

    /// SPEC: canon §5.7 ambient axiom terms, with §21 test 35's AETHER term
    /// (Cyberpunk, TEC 9: 14000) and doc 04's composure example (Cosmic
    /// Horror: a 5-point loss costs 7,500 milli).
    #[test]
    fn ambient_terms() {
        let cp = LocalGenre::Plain(Genre::Cyberpunk);
        assert_eq!(aether_regen_term_bp(cp), 14_000);
        assert_eq!(
            aether_regen_term_bp(LocalGenre::Plain(Genre::UrText)),
            10_000
        );
        assert_eq!(
            aether_regen_term_bp(LocalGenre::Plain(Genre::HardboiledNoir)),
            7000
        );
        assert_eq!(glyph_term_bp(Some(Axiom::Techne), cp), 10_800);
        assert_eq!(glyph_term_bp(None, cp), 10_000);
        assert_eq!(
            techne_recharge_term_bp(LocalGenre::Plain(Genre::FairyTale)),
            6000
        );
        let ch = LocalGenre::Plain(Genre::CosmicHorror);
        assert_eq!(mythos_damage_term_bp(ch, true), 11_500);
        assert_eq!(mythos_damage_term_bp(ch, false), 10_000);
        assert_eq!(mythos_curse_chance_term_bp(ch), 17_500);
        assert_eq!(mul_bp(5000, mythos_composure_loss_term_bp(ch)), 7500);
        assert_eq!(verve_crit_term_bp(0), -500);
        assert_eq!(verve_deaths_door_term_bp(10), 15_000);
        assert!(!electronics_allowed(Genre::FairyTale));
        assert!(!electronics_allowed(Genre::MythicFantasy));
        assert!(electronics_allowed(Genre::GothicHorror));
    }

    /// SPEC: canon §5.3 dependency set helpers.
    #[test]
    fn depset_helpers() {
        let d = DepSet::single(Axiom::Mythos).with(Axiom::Arcana);
        assert_eq!(d.len(), 2);
        assert_eq!(d.only(), None);
        assert_eq!(d.axes().collect::<Vec<_>>(), [Axiom::Arcana, Axiom::Mythos]);
        assert!(d.contains(Axiom::Arcana) && !d.contains(Axiom::Verve));
        assert_eq!(DepSet::NONE.only(), None);
        assert!(DepSet::NONE.is_empty());
    }

    fn arb_genre() -> impl Strategy<Value = Genre> {
        (0usize..16).prop_map(|i| Genre::ALL[i])
    }

    fn arb_local() -> impl Strategy<Value = LocalGenre> {
        (arb_genre(), arb_genre(), any::<bool>()).prop_map(|(a, b, seam)| {
            if seam {
                LocalGenre::Seam { folio: a, other: b }
            } else {
                LocalGenre::Plain(a)
            }
        })
    }

    proptest! {
        /// SPEC: canon §5.5 invariants: raising any tolerance never raises C
        /// or changes E; C = 0 iff every raw_gap is within tolerance; E <= 6;
        /// potency never rises with C.
        #[test]
        fn contradiction_monotone_in_tolerance(
            bits in 0u8..16, origin in arb_genre(), local in arb_local(),
            tol in prop::array::uniform4(0u8..=6), axis in 0usize..4,
        ) {
            let comp = Component::new(DepSet(bits), origin);
            let k = evaluate(&comp, local, tol);
            let mut more = tol;
            more[axis] = (more[axis] + 1).min(TOL_CAP);
            let k2 = evaluate(&comp, local, more);
            prop_assert!(k2.c <= k.c);
            prop_assert!(k.e <= EDGE_MAX);
            if k.c <= 5 && k2.c <= 5 { prop_assert_eq!(k.e, k2.e); }
            if k.basis == ContradictionBasis::Formula {
                let within = (0..4).all(|i| k.raw_gap[i] <= tol[i]);
                prop_assert_eq!(k.c == 0, within);
            }
            prop_assert!(contradiction_row(k2.c, false).potency_bp >= contradiction_row(k.c, false).potency_bp);
        }

        /// SPEC: canon §5.5 table: potency never rises and failure never falls
        /// as C grows from 0 to 5; Unmoored never raises potency.
        #[test]
        fn table_monotone(c in 0u8..6, unmoored in any::<bool>()) {
            let a = contradiction_row(c, unmoored);
            let b = contradiction_row(c + 1, unmoored);
            prop_assert!(b.potency_bp <= a.potency_bp);
            if b.state != ContradictionState::Disconnected {
                prop_assert!(b.failure_bp >= a.failure_bp);
            }
            prop_assert!(contradiction_row(c, true).potency_bp <= contradiction_row(c, false).potency_bp);
        }

        /// SPEC: canon §5.8: a seam is never worse than either genre alone.
        #[test]
        fn seam_never_worse(a in arb_genre(), b in arb_genre(), bits in 0u8..16, origin in arb_genre()) {
            let comp = Component::new(DepSet(bits), origin);
            let seam = evaluate(&comp, LocalGenre::Seam { folio: a, other: b }, [0; 4]);
            let alone = evaluate(&comp, LocalGenre::Plain(a), [0; 4]);
            prop_assert!(seam.c <= alone.c);
            prop_assert_eq!(evaluate(&comp, LocalGenre::Plain(a), [0; 4]), alone);
        }

        /// SPEC: canon §5.4: tolerance is capped at 6 and attune_tol is in 0..=3
        /// and non-decreasing in ATN.
        #[test]
        fn tolerance_ranges(atn in 1i32..=150, g in arb_genre(), gear in prop::array::uniform4(0u8..=6)) {
            let t = TolInputs::erratum(g, atn, gear, gear);
            prop_assert!(t.total(gear).iter().all(|&v| v <= TOL_CAP));
            prop_assert!(attune_tol(atn) <= 3);
            prop_assert!(attune_tol(atn + 1) >= attune_tol(atn));
        }
    }
}
