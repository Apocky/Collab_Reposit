//! SPEC: canon §5.1-5.2, §5.4, §5.9 genre table and tolerance
//!
//! The four axioms, the 16-genre table of canon §5.2 (axioms, `aether_axiom`,
//! `gravity_bp`, `ambient_light`, Lucre and Aether skins, tier), the genre
//! identity columns of canon §5.9 (physics multipliers, spawn light, palette,
//! music, enemy themes, materials, manufacturers) and the Origin tolerance of
//! canon §5.4 derived from a genre's axioms by the sorted-axiom rule.
//!
//! Everything here is `const` data; nothing allocates and nothing draws.

use palimpsest_math::{fnv1a32, mul_bp};

/// SPEC: canon §5.1. The four axioms, in the canon tie-break order
/// ARCANA, TECHNE, MYTHOS, VERVE (canon §5.4). The discriminant is the index
/// used by every `[_; 4]` per-axis array in this crate.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Axiom {
    /// `ax.arcana`: how well magic works.
    Arcana = 0,
    /// `ax.techne`: how well technology works.
    Techne = 1,
    /// `ax.mythos`: how strongly the supernatural operates.
    Mythos = 2,
    /// `ax.verve`: how cinematic reality is.
    Verve = 3,
}

impl Axiom {
    /// SPEC: canon §5.1. All axioms in index (and tie-break) order.
    pub const ALL: [Axiom; 4] = [Axiom::Arcana, Axiom::Techne, Axiom::Mythos, Axiom::Verve];

    /// SPEC: canon §5.1. Index `0..=3` into per-axis arrays.
    #[must_use]
    pub const fn index(self) -> usize {
        self as usize
    }

    /// SPEC: canon §5.1. The axiom with index `i`, if `i <= 3`.
    #[must_use]
    pub const fn from_index(i: usize) -> Option<Axiom> {
        match i {
            0 => Some(Axiom::Arcana),
            1 => Some(Axiom::Techne),
            2 => Some(Axiom::Mythos),
            3 => Some(Axiom::Verve),
            _ => None,
        }
    }

    /// SPEC: canon §5.1. Content ID, for example `ax.arcana`.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Axiom::Arcana => "ax.arcana",
            Axiom::Techne => "ax.techne",
            Axiom::Mythos => "ax.mythos",
            Axiom::Verve => "ax.verve",
        }
    }

    /// SPEC: canon §5.1. Display name, for example `ARCANA`.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Axiom::Arcana => "ARCANA",
            Axiom::Techne => "TECHNE",
            Axiom::Mythos => "MYTHOS",
            Axiom::Verve => "VERVE",
        }
    }
}

/// SPEC: canon §5.1. Highest axiom value; every axiom is `0..=10` per genre.
pub const AXIOM_MAX: u8 = 10;

/// SPEC: canon §5.1. One axiom value per axis, indexed by [`Axiom::index`]
/// (ARC, TEC, MYT, VRV), each `0..=10`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Axioms(pub [u8; 4]);

impl Axioms {
    /// SPEC: canon §5.1. The value on axis `a`.
    #[must_use]
    pub const fn get(self, a: Axiom) -> u8 {
        self.0[a.index()]
    }

    /// SPEC: canon §5.2 "minimum pairwise L1 distance". `Σ |self[a] - o[a]|`.
    #[must_use]
    pub const fn l1_distance(self, o: Axioms) -> u32 {
        let mut i = 0;
        let mut sum = 0u32;
        while i < 4 {
            sum += self.0[i].abs_diff(o.0[i]) as u32;
            i += 1;
        }
        sum
    }

    /// SPEC: canon §5.8 seam zones: the larger of the two genres' values on
    /// each axis.
    #[must_use]
    pub const fn max_each(self, o: Axioms) -> Axioms {
        let mut out = [0u8; 4];
        let mut i = 0;
        while i < 4 {
            out[i] = if self.0[i] >= o.0[i] {
                self.0[i]
            } else {
                o.0[i]
            };
            i += 1;
        }
        Axioms(out)
    }
}

/// SPEC: canon §5.2 `Tier` column.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Tier {
    /// One of the 12 core genres: lattice candidates and launch Origins.
    Core,
    /// A secret genre, reached only through secret exits (canon §17.1).
    Secret,
}

/// SPEC: canon §5.9 manufacturer row: content ID and signature trait text
/// (doc 11 owns the mechanics).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Manufacturer {
    /// `mfr.<genre_suffix>.<name>`.
    pub id: &'static str,
    /// Signature trait as written in the canon table.
    pub signature: &'static str,
}

/// SPEC: canon §5.2 and §5.9. One genre row: the §5.2 columns followed by the
/// §5.9 identity columns. Doc 12 copies these columns exactly.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct GenreRow {
    /// Content ID, `genre.<suffix>`.
    pub id: &'static str,
    /// Display name.
    pub display_name: &'static str,
    /// ARC, TEC, MYT, VRV.
    pub axioms: Axioms,
    /// The axiom that powers native casting; `None` is the canon's `none`.
    pub aether_axiom: Option<Axiom>,
    /// Multiplies `GRAVITY_BASE` (canon §4).
    pub gravity_bp: i32,
    /// Simulation light floor in open areas, `0..=15`.
    pub ambient_light: u8,
    /// Display skin of `cur.lucre`.
    pub lucre_skin: &'static str,
    /// Display skin of aether.
    pub aether_skin: &'static str,
    /// Core or secret.
    pub tier: Tier,
    /// §5.9: multiplies air acceleration.
    pub air_accel_bp: i32,
    /// §5.9: multiplies fall damage.
    pub fall_dmg_bp: i32,
    /// §5.9: multiplies ground deceleration.
    pub ground_friction_bp: i32,
    /// §5.9: highest block-light level in which ambient spawns, wave spawns and
    /// the initial population may occur (§12.3).
    pub spawn_block_max: u8,
    /// §5.9 palette as `0xRRGGBB`.
    pub palette: [u32; 3],
    /// §5.9 music register.
    pub music: &'static str,
    /// §5.9 enemy themes.
    pub enemy_themes: &'static str,
    /// §5.9 material IDs (doc 07 defines each).
    pub materials: &'static [&'static str],
    /// §5.9 core manufacturers (doc 11 adds more).
    pub manufacturers: &'static [Manufacturer],
}

/// SPEC: canon §5.2. The 16 genres in table order (`#` 1 to 16).
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Genre {
    /// `genre.mythic_fantasy` (#1).
    MythicFantasy = 0,
    /// `genre.gothic_horror` (#2).
    GothicHorror = 1,
    /// `genre.cosmic_horror` (#3).
    CosmicHorror = 2,
    /// `genre.space_opera` (#4).
    SpaceOpera = 3,
    /// `genre.cyberpunk` (#5).
    Cyberpunk = 4,
    /// `genre.post_apocalypse` (#6).
    PostApocalypse = 5,
    /// `genre.weird_west` (#7).
    WeirdWest = 6,
    /// `genre.hardboiled_noir` (#8).
    HardboiledNoir = 7,
    /// `genre.wuxia` (#9).
    Wuxia = 8,
    /// `genre.steampunk` (#10).
    Steampunk = 9,
    /// `genre.fairy_tale` (#11).
    FairyTale = 10,
    /// `genre.mythic_antiquity` (#12).
    MythicAntiquity = 11,
    /// `genre.oneiric` (#13, secret).
    Oneiric = 12,
    /// `genre.redacted` (#14, secret; the Editors' home genre).
    Redacted = 13,
    /// `genre.unwritten` (#15, secret).
    Unwritten = 14,
    /// `genre.ur_text` (#16, secret; always Folio 25).
    UrText = 15,
}

impl Genre {
    /// SPEC: canon §5.2. All 16 genres in table order.
    pub const ALL: [Genre; 16] = [
        Genre::MythicFantasy,
        Genre::GothicHorror,
        Genre::CosmicHorror,
        Genre::SpaceOpera,
        Genre::Cyberpunk,
        Genre::PostApocalypse,
        Genre::WeirdWest,
        Genre::HardboiledNoir,
        Genre::Wuxia,
        Genre::Steampunk,
        Genre::FairyTale,
        Genre::MythicAntiquity,
        Genre::Oneiric,
        Genre::Redacted,
        Genre::Unwritten,
        Genre::UrText,
    ];

    /// SPEC: canon §5.2. The 12 core genres (rows 1 to 12).
    pub const CORE: [Genre; 12] = [
        Genre::MythicFantasy,
        Genre::GothicHorror,
        Genre::CosmicHorror,
        Genre::SpaceOpera,
        Genre::Cyberpunk,
        Genre::PostApocalypse,
        Genre::WeirdWest,
        Genre::HardboiledNoir,
        Genre::Wuxia,
        Genre::Steampunk,
        Genre::FairyTale,
        Genre::MythicAntiquity,
    ];

    /// SPEC: canon §5.2. Zero-based table index (`#` minus 1).
    #[must_use]
    pub const fn index(self) -> usize {
        self as usize
    }

    /// SPEC: canon §5.2. The full table row of this genre.
    #[must_use]
    pub const fn row(self) -> &'static GenreRow {
        &GENRES[self.index()]
    }

    /// SPEC: canon §5.2. Content ID, for example `genre.cyberpunk`.
    #[must_use]
    pub const fn id(self) -> &'static str {
        self.row().id
    }

    /// SPEC: canon §1.10, §5.2. The genre suffix (the part after `genre.`)
    /// used in skin text keys.
    #[must_use]
    pub fn suffix(self) -> &'static str {
        let id = self.id();
        id.strip_prefix("genre.").unwrap_or(id)
    }

    /// SPEC: canon §1.1 `id32 = fnv1a32(utf8(id))`.
    #[must_use]
    pub const fn id32(self) -> u32 {
        fnv1a32(self.id().as_bytes())
    }

    /// SPEC: canon §5.2. The genre with content ID `id`, if any.
    #[must_use]
    pub fn from_id(id: &str) -> Option<Genre> {
        Genre::ALL.into_iter().find(|g| g.id() == id)
    }

    /// SPEC: canon §5.2. ARC, TEC, MYT, VRV.
    #[must_use]
    pub const fn axioms(self) -> Axioms {
        self.row().axioms
    }

    /// SPEC: canon §5.2. This genre's value on axis `a`.
    #[must_use]
    pub const fn axiom(self, a: Axiom) -> u8 {
        self.row().axioms.get(a)
    }

    /// SPEC: canon §5.2 `aether_axiom` (`None` is `none`).
    #[must_use]
    pub const fn aether_axiom(self) -> Option<Axiom> {
        self.row().aether_axiom
    }

    /// SPEC: canon §5.2 `gravity_bp`.
    #[must_use]
    pub const fn gravity_bp(self) -> i32 {
        self.row().gravity_bp
    }

    /// SPEC: canon §5.2 `ambient_light`.
    #[must_use]
    pub const fn ambient_light(self) -> u8 {
        self.row().ambient_light
    }

    /// SPEC: canon §5.2 `Tier` column: true for the 12 core genres.
    #[must_use]
    pub const fn is_core(self) -> bool {
        matches!(self.row().tier, Tier::Core)
    }

    /// SPEC: canon §4, §5.2. Local gravity in mm/s²:
    /// `mul_bp(GRAVITY_BASE, gravity_bp)`.
    #[must_use]
    pub const fn gravity_mm_s2(self) -> i64 {
        mul_bp(GRAVITY_BASE_MM_S2, self.gravity_bp())
    }

    /// SPEC: canon §5.4 Origin tolerance by the sorted-axiom rule: sort the
    /// four axioms in descending order, ties broken ARCANA, TECHNE, MYTHOS,
    /// VERVE; the first gets 2, the second and third 1, the fourth 0.
    #[must_use]
    pub const fn origin_tol(self) -> [u8; 4] {
        const BY_RANK: [u8; 4] = [2, 1, 1, 0];
        let ax = self.axioms().0;
        let mut out = [0u8; 4];
        let mut i = 0;
        while i < 4 {
            // rank = number of axes sorted before axis i.
            let mut rank = 0;
            let mut j = 0;
            while j < 4 {
                if ax[j] > ax[i] || (ax[j] == ax[i] && j < i) {
                    rank += 1;
                }
                j += 1;
            }
            out[i] = BY_RANK[rank];
            i += 1;
        }
        out
    }

    /// SPEC: canon §5.4. NPC creatures have `origin_tol = 2` on every axis of
    /// their own origin genre where that genre's axiom is at least 5.
    #[must_use]
    pub const fn npc_origin_tol(self) -> [u8; 4] {
        let ax = self.axioms().0;
        let mut out = [0u8; 4];
        let mut i = 0;
        while i < 4 {
            if ax[i] >= NPC_TOL_MIN_AXIOM {
                out[i] = 2;
            }
            i += 1;
        }
        out
    }
}

/// SPEC: canon §4 `GRAVITY_BASE` = 20000 mm/s².
pub const GRAVITY_BASE_MM_S2: i64 = 20_000;

/// SPEC: canon §5.4. NPCs get origin tolerance on axes at or above this value.
pub const NPC_TOL_MIN_AXIOM: u8 = 5;

/// SPEC: canon §5.2. The minimum pairwise L1 distance between the axiom
/// vectors of the 12 core genres.
pub const MIN_CORE_L1_DISTANCE: u32 = 6;

/// SPEC: canon §5.2, §21 test 40. Minimum pairwise L1 distance over
/// [`Genre::CORE`], computed from the table.
#[must_use]
pub fn min_core_l1_distance() -> u32 {
    let mut best = u32::MAX;
    for (i, a) in Genre::CORE.iter().enumerate() {
        for b in &Genre::CORE[i + 1..] {
            best = best.min(a.axioms().l1_distance(b.axioms()));
        }
    }
    best
}

const fn m(id: &'static str, signature: &'static str) -> Manufacturer {
    Manufacturer { id, signature }
}

const A: Option<Axiom> = Some(Axiom::Arcana);
const T: Option<Axiom> = Some(Axiom::Techne);
const MY: Option<Axiom> = Some(Axiom::Mythos);
const V: Option<Axiom> = Some(Axiom::Verve);

/// SPEC: canon §5.2 and §5.9. The genre table in `#` order, indexed by
/// [`Genre::index`].
pub static GENRES: [GenreRow; 16] = [
    GenreRow {
        id: "genre.mythic_fantasy",
        display_name: "Mythic Fantasy",
        axioms: Axioms([8, 2, 6, 6]),
        aether_axiom: A,
        gravity_bp: 10000,
        ambient_light: 4,
        lucre_skin: "Crowns",
        aether_skin: "Mana",
        tier: Tier::Core,
        air_accel_bp: 10000,
        fall_dmg_bp: 10000,
        ground_friction_bp: 10000,
        spawn_block_max: 0,
        palette: [0x3B6E3B, 0xC9A227, 0x8FB3D9],
        music: "horns, strings, choir",
        enemy_themes: "orcs, dragons, wyverns",
        materials: &["mat.steel", "mat.moonsilver", "mat.oak", "mat.dragonbone"],
        manufacturers: &[
            m(
                "mfr.mythic_fantasy.deepanvil",
                "poise damage × 15000, attack speed × 9000",
            ),
            m("mfr.mythic_fantasy.silverbough", "attack noise −20 dB"),
        ],
    },
    GenreRow {
        id: "genre.gothic_horror",
        display_name: "Gothic Horror",
        axioms: Axioms([5, 3, 9, 3]),
        aether_axiom: MY,
        gravity_bp: 10000,
        ambient_light: 2,
        lucre_skin: "Shillings",
        aether_skin: "Sanguine",
        tier: Tier::Core,
        air_accel_bp: 10000,
        fall_dmg_bp: 10000,
        ground_friction_bp: 10000,
        spawn_block_max: 7,
        palette: [0x3A1F2B, 0x7A0E14, 0xC8C3B4],
        music: "organ, string quartet, harpsichord",
        enemy_themes: "vampires, werewolves, ghouls",
        materials: &["mat.silver", "mat.blackwood", "mat.bone"],
        manufacturers: &[
            m(
                "mfr.gothic_horror.vesper_sons",
                "2000 Bp of each hit converted to dmg.radiant",
            ),
            m(
                "mfr.gothic_horror.grimhallow",
                "instant reload for 3% Max HP",
            ),
        ],
    },
    GenreRow {
        id: "genre.cosmic_horror",
        display_name: "Cosmic Horror",
        axioms: Axioms([3, 6, 10, 1]),
        aether_axiom: MY,
        gravity_bp: 10000,
        ambient_light: 1,
        lucre_skin: "Bonds",
        aether_skin: "Gnosis",
        tier: Tier::Core,
        air_accel_bp: 8000,
        fall_dmg_bp: 12500,
        ground_friction_bp: 10000,
        spawn_block_max: 7,
        palette: [0x0E2A2E, 0x4F7F6A, 0xB8E986],
        music: "drones, detuned strings, theremin",
        enemy_themes: "cultists, deep things, formless spawn",
        materials: &["mat.obsidian", "mat.star_metal", "mat.chitin"],
        manufacturers: &[
            m(
                "mfr.cosmic_horror.hollow_lantern",
                "hits apply st.marked for 4 s",
            ),
            m(
                "mfr.cosmic_horror.outer_choir",
                "2500 Bp dmg.psychic share, +1500 Bp damage, costs 1 composure per shot",
            ),
        ],
    },
    GenreRow {
        id: "genre.space_opera",
        display_name: "Space Opera",
        axioms: Axioms([6, 10, 3, 7]),
        aether_axiom: A,
        gravity_bp: 8000,
        ambient_light: 6,
        lucre_skin: "Credits",
        aether_skin: "Psi",
        tier: Tier::Core,
        air_accel_bp: 12500,
        fall_dmg_bp: 7500,
        ground_friction_bp: 10000,
        spawn_block_max: 0,
        palette: [0x0B1026, 0x3D7BFF, 0xFF8A3D],
        music: "synth-orchestral, brass",
        enemy_themes: "boarders, xeno hives, war droids",
        materials: &["mat.plasteel", "mat.titanium", "mat.psi_crystal"],
        manufacturers: &[
            m(
                "mfr.space_opera.perihelion",
                "a 30-tick charged shot deals × 15000",
            ),
            m(
                "mfr.space_opera.umbra",
                "no magazine; heat; overheating locks the weapon for 90 ticks",
            ),
        ],
    },
    GenreRow {
        id: "genre.cyberpunk",
        display_name: "Cyberpunk",
        axioms: Axioms([1, 9, 2, 5]),
        aether_axiom: T,
        gravity_bp: 10000,
        ambient_light: 3,
        lucre_skin: "Chits",
        aether_skin: "Bandwidth",
        tier: Tier::Core,
        air_accel_bp: 11000,
        fall_dmg_bp: 10000,
        ground_friction_bp: 10000,
        spawn_block_max: 0,
        palette: [0x120B1F, 0xFF2E88, 0x00E5FF],
        music: "darksynth, industrial",
        enemy_themes: "gangers, corporate security, rogue AIs",
        materials: &["mat.carbon_fiber", "mat.polymer", "mat.nanoweave"],
        manufacturers: &[
            m(
                "mfr.cyberpunk.ghostline",
                "projectiles home on st.marked targets at 16384 Bam/s",
            ),
            m(
                "mfr.cyberpunk.neon_saint",
                "charged shots pierce 1000 mm of blocks",
            ),
        ],
    },
    GenreRow {
        id: "genre.post_apocalypse",
        display_name: "Post-Apocalypse",
        axioms: Axioms([0, 6, 5, 6]),
        aether_axiom: T,
        gravity_bp: 10000,
        ambient_light: 5,
        lucre_skin: "Tins",
        aether_skin: "Static",
        tier: Tier::Core,
        air_accel_bp: 10000,
        fall_dmg_bp: 10000,
        ground_friction_bp: 9000,
        spawn_block_max: 0,
        palette: [0x6B4E2E, 0xC2A36B, 0xE04E1B],
        music: "distorted guitar, junk percussion",
        enemy_themes: "raiders, mutants, scrap golems, rad-cults",
        materials: &["mat.scrap_iron", "mat.rubber", "mat.rad_glass"],
        manufacturers: &[
            m(
                "mfr.post_apocalypse.rustmaw",
                "magazine × 20000, reload time × 13000",
            ),
            m(
                "mfr.post_apocalypse.lastditch",
                "2000 Bp chance per shot to use no ammo",
            ),
        ],
    },
    GenreRow {
        id: "genre.weird_west",
        display_name: "Weird West",
        axioms: Axioms([4, 4, 6, 7]),
        aether_axiom: MY,
        gravity_bp: 10000,
        ambient_light: 6,
        lucre_skin: "Eagles",
        aether_skin: "Hex",
        tier: Tier::Core,
        air_accel_bp: 11000,
        fall_dmg_bp: 10000,
        ground_friction_bp: 10000,
        spawn_block_max: 0,
        palette: [0x8C4A2F, 0xE0B776, 0x2F3E4E],
        music: "slide guitar, whistle",
        enemy_themes: "outlaws, revenants, sand wyrms",
        materials: &["mat.iron", "mat.leather", "mat.hexed_silver"],
        manufacturers: &[
            m(
                "mfr.weird_west.coffin_creek",
                "crits ricochet 5000 Bp of their damage to the nearest other hostile within 8 m",
            ),
            m(
                "mfr.weird_west.seven_seals",
                "2000 Bp dmg.umbral share with dependency mythos",
            ),
        ],
    },
    GenreRow {
        id: "genre.hardboiled_noir",
        display_name: "Hardboiled Noir",
        axioms: Axioms([0, 5, 1, 2]),
        aether_axiom: V,
        gravity_bp: 10000,
        ambient_light: 2,
        lucre_skin: "Bucks",
        aether_skin: "Nerve",
        tier: Tier::Core,
        air_accel_bp: 8000,
        fall_dmg_bp: 12500,
        ground_friction_bp: 8500,
        spawn_block_max: 0,
        palette: [0x101010, 0x6E6E6E, 0xC22B2B],
        music: "smoky jazz, muted trumpet",
        enemy_themes: "mobsters, crooked cops, hired killers",
        materials: &["mat.steel", "mat.brass", "mat.tweed"],
        manufacturers: &[
            m(
                "mfr.hardboiled_noir.midnight_avenue",
                "shot noise capped at 50 dB",
            ),
            m(
                "mfr.hardboiled_noir.rainslick",
                "× 12000 against staggered or knocked-down targets",
            ),
        ],
    },
    GenreRow {
        id: "genre.wuxia",
        display_name: "Wuxia",
        axioms: Axioms([6, 2, 5, 10]),
        aether_axiom: V,
        gravity_bp: 8000,
        ambient_light: 5,
        lucre_skin: "Taels",
        aether_skin: "Qi",
        tier: Tier::Core,
        air_accel_bp: 20000,
        fall_dmg_bp: 5000,
        ground_friction_bp: 10000,
        spawn_block_max: 0,
        palette: [0x1F3A2E, 0xD9C9A3, 0xB32020],
        music: "guzheng, dizi, erhu, taiko",
        enemy_themes: "rival sects, jiangshi, fox spirits",
        materials: &["mat.jade", "mat.bronze", "mat.silk"],
        manufacturers: &[
            m(
                "mfr.wuxia.jade_lotus",
                "thrown weapons return after 30 ticks",
            ),
            m(
                "mfr.wuxia.iron_crane",
                "a block within 6 ticks of impact reflects 5000 Bp of the damage",
            ),
        ],
    },
    GenreRow {
        id: "genre.steampunk",
        display_name: "Steampunk",
        axioms: Axioms([3, 7, 3, 6]),
        aether_axiom: T,
        gravity_bp: 10000,
        ambient_light: 4,
        lucre_skin: "Sovereigns",
        aether_skin: "Pressure",
        tier: Tier::Core,
        air_accel_bp: 10000,
        fall_dmg_bp: 10000,
        ground_friction_bp: 10000,
        spawn_block_max: 0,
        palette: [0x4A3A28, 0xB5793A, 0x5FA8A0],
        music: "brass band, calliope",
        enemy_themes: "automata, sky pirates, clockwork beasts",
        materials: &["mat.brass", "mat.copper", "mat.aetherium"],
        manufacturers: &[
            m(
                "mfr.steampunk.brassworth",
                "+200 Bp per consecutive hit, maximum +3000",
            ),
            m(
                "mfr.steampunk.aetherline",
                "every 4th hit chains 3000 Bp of its damage as dmg.shock to 2 targets within 5 m",
            ),
        ],
    },
    GenreRow {
        id: "genre.fairy_tale",
        display_name: "Fairy Tale",
        axioms: Axioms([10, 1, 8, 3]),
        aether_axiom: A,
        gravity_bp: 10000,
        ambient_light: 6,
        lucre_skin: "Fairy Gold",
        aether_skin: "Glamour",
        tier: Tier::Core,
        air_accel_bp: 12500,
        fall_dmg_bp: 7500,
        ground_friction_bp: 10000,
        spawn_block_max: 0,
        palette: [0xF2D7E8, 0x6FBF73, 0xE8C547],
        music: "celesta, woodwinds, waltz",
        enemy_themes: "wolves, witches, trolls",
        materials: &["mat.cold_iron", "mat.glass", "mat.elderwood"],
        manufacturers: &[
            m(
                "mfr.fairy_tale.thimbleweald",
                "× 13000 against targets with a larger capsule radius than the wielder",
            ),
            m(
                "mfr.fairy_tale.thornwood",
                "1000 Bp chance per hit to apply st.rooted",
            ),
        ],
    },
    GenreRow {
        id: "genre.mythic_antiquity",
        display_name: "Mythic Antiquity",
        axioms: Axioms([6, 2, 10, 8]),
        aether_axiom: MY,
        gravity_bp: 10000,
        ambient_light: 7,
        lucre_skin: "Drachmae",
        aether_skin: "Ichor",
        tier: Tier::Core,
        air_accel_bp: 12500,
        fall_dmg_bp: 10000,
        ground_friction_bp: 10000,
        spawn_block_max: 0,
        palette: [0xF0E6D2, 0x2E5A88, 0xC9792B],
        music: "lyre, aulos, chant",
        enemy_themes: "shades, hydras, harpies",
        materials: &["mat.bronze", "mat.orichalcum", "mat.marble"],
        manufacturers: &[
            m(
                "mfr.mythic_antiquity.hephaesteum",
                "the wielder is immune to st.disarmed and st.corroded",
            ),
            m(
                "mfr.mythic_antiquity.argive",
                "+1500 Bp damage while an allied Erratum is within 3 m",
            ),
        ],
    },
    GenreRow {
        id: "genre.oneiric",
        display_name: "Oneiric",
        axioms: Axioms([9, 5, 8, 9]),
        aether_axiom: A,
        gravity_bp: 6000,
        ambient_light: 4,
        lucre_skin: "Motes",
        aether_skin: "Lucidity",
        tier: Tier::Secret,
        air_accel_bp: 15000,
        fall_dmg_bp: 2500,
        ground_friction_bp: 10000,
        spawn_block_max: 0,
        palette: [0x2B1B4D, 0x8E7CFF, 0xFFC2E0],
        music: "reversed piano, choir pads",
        enemy_themes: "nightmares, doppelgangers, sleepwalkers",
        materials: &["mat.dreamstuff"],
        manufacturers: &[m(
            "mfr.oneiric.somnambule",
            "after 10 m of flight, projectiles pass through destructible and non-opaque blocks without spending penetration (§4.3)",
        )],
    },
    GenreRow {
        id: "genre.redacted",
        display_name: "The Redaction",
        axioms: Axioms([2, 8, 8, 0]),
        aether_axiom: T,
        gravity_bp: 10000,
        ambient_light: 9,
        lucre_skin: "Vouchers",
        aether_skin: "Clearance",
        tier: Tier::Secret,
        air_accel_bp: 6000,
        fall_dmg_bp: 15000,
        ground_friction_bp: 10000,
        spawn_block_max: 15,
        palette: [0xF2F2F2, 0x1A1A1A, 0xD7262E],
        music: "office hum, sine tones",
        enemy_themes: "agents, anomalies, containment drones",
        materials: &["mat.redacted_alloy"],
        manufacturers: &[m(
            "mfr.redacted.bureau",
            "never rolls Contradiction failure; the potency loss still applies",
        )],
    },
    GenreRow {
        id: "genre.unwritten",
        display_name: "The Unwritten",
        axioms: Axioms([0, 0, 0, 0]),
        aether_axiom: None,
        gravity_bp: 10000,
        ambient_light: 8,
        lucre_skin: "Lucre",
        aether_skin: "Potential",
        tier: Tier::Secret,
        air_accel_bp: 10000,
        fall_dmg_bp: 10000,
        ground_friction_bp: 10000,
        spawn_block_max: 15,
        palette: [0xFFFFFF, 0xE6E6E6, 0xCCCCCC],
        music: "silence",
        enemy_themes: "Editors only",
        materials: &["mat.vellum"],
        manufacturers: &[],
    },
    GenreRow {
        id: "genre.ur_text",
        display_name: "The Ur-Text",
        axioms: Axioms([5, 5, 5, 5]),
        aether_axiom: None,
        gravity_bp: 10000,
        ambient_light: 4,
        lucre_skin: "Words",
        aether_skin: "Logos",
        tier: Tier::Secret,
        air_accel_bp: 10000,
        fall_dmg_bp: 10000,
        ground_friction_bp: 10000,
        spawn_block_max: 0,
        palette: [0x1C1408, 0xE8D9B0, 0xB8862B],
        music: "every register in counterpoint",
        enemy_themes: "every roster",
        materials: &["mat.vellum", "mat.gold_leaf"],
        manufacturers: &[m("mfr.ur_text.first_press", "Palimpsestic items only")],
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    /// The canon §5.4 Origin tolerance table, copied literally (ARC, TEC, MYT, VRV).
    const CANON_ORIGIN_TOL: [[u8; 4]; 16] = [
        [2, 0, 1, 1],
        [1, 1, 2, 0],
        [1, 1, 2, 0],
        [1, 2, 0, 1],
        [0, 2, 1, 1],
        [0, 2, 1, 1],
        [1, 0, 1, 2],
        [0, 2, 1, 1],
        [1, 0, 1, 2],
        [1, 2, 0, 1],
        [2, 0, 1, 1],
        [1, 0, 2, 1],
        [2, 0, 1, 1],
        [1, 2, 1, 0],
        [2, 1, 1, 0],
        [2, 1, 1, 0],
    ];

    /// SPEC: canon §5.4. The sorted-axiom rule reproduces every row of the table.
    #[test]
    fn origin_tol_rule_matches_canon_table() {
        for g in Genre::ALL {
            assert_eq!(g.origin_tol(), CANON_ORIGIN_TOL[g.index()], "{}", g.id());
        }
    }

    /// SPEC: canon §21 test 16. Every row of the §5.4 table sums to 4.
    #[test]
    fn canon_t16_origin_tolerance_sums() {
        for row in CANON_ORIGIN_TOL {
            assert_eq!(row.iter().map(|&v| u32::from(v)).sum::<u32>(), 4);
        }
        for g in Genre::ALL {
            assert_eq!(g.origin_tol().iter().map(|&v| u32::from(v)).sum::<u32>(), 4);
        }
    }

    /// SPEC: canon §21 test 40. Minimum pairwise L1 distance of the 12 core
    /// axiom vectors is 6, and every tolerance row still sums to 4.
    #[test]
    fn canon_t40_genre_spread() {
        assert_eq!(min_core_l1_distance(), MIN_CORE_L1_DISTANCE);
        assert_eq!(min_core_l1_distance(), 6);
        for g in Genre::ALL {
            assert_eq!(g.origin_tol().iter().map(|&v| u32::from(v)).sum::<u32>(), 4);
        }
    }

    /// SPEC: canon §5.2 table columns, spot-checked against the canon text.
    #[test]
    fn genre_table_columns() {
        let cp = Genre::Cyberpunk;
        assert_eq!(cp.id(), "genre.cyberpunk");
        assert_eq!(cp.suffix(), "cyberpunk");
        assert_eq!(cp.axioms(), Axioms([1, 9, 2, 5]));
        assert_eq!(cp.aether_axiom(), Some(Axiom::Techne));
        assert_eq!(cp.id32(), 0x99b5_4427); // canon §21 test 4
        assert_eq!(Genre::Wuxia.gravity_bp(), 8000);
        assert_eq!(Genre::Wuxia.gravity_mm_s2(), 16_000);
        assert_eq!(Genre::Oneiric.gravity_mm_s2(), 12_000);
        assert_eq!(Genre::Redacted.axiom(Axiom::Verve), 0);
        assert_eq!(Genre::Redacted.ambient_light(), 9);
        assert_eq!(Genre::Unwritten.aether_axiom(), None);
        assert_eq!(Genre::UrText.aether_axiom(), None);
        assert_eq!(Genre::FairyTale.row().lucre_skin, "Fairy Gold");
        assert_eq!(Genre::HardboiledNoir.row().aether_skin, "Nerve");
        assert_eq!(Genre::Wuxia.row().air_accel_bp, 20_000);
        assert_eq!(Genre::Oneiric.row().fall_dmg_bp, 2500);
        assert_eq!(Genre::PostApocalypse.row().ground_friction_bp, 9000);
        assert_eq!(Genre::Redacted.row().spawn_block_max, 15);
        assert_eq!(
            Genre::MythicFantasy.row().palette,
            [0x3B6E3B, 0xC9A227, 0x8FB3D9]
        );
        assert_eq!(
            Genre::UrText.row().manufacturers[0].id,
            "mfr.ur_text.first_press"
        );
        assert!(Genre::Unwritten.row().manufacturers.is_empty());
    }

    /// Cells of a Markdown table row (`| a | b |` gives `["a", "b"]`).
    fn cells(line: &str) -> Vec<&str> {
        let t = line.trim();
        let inner = t.strip_prefix('|').and_then(|r| r.strip_suffix('|'));
        inner.map_or_else(Vec::new, |r| r.split('|').map(str::trim).collect())
    }

    /// SPEC: canon §5.2 and §5.9 (canon-1.11): every column this crate copies,
    /// manufacturer signatures included (backticks and the final period
    /// dropped), equals the canon text. Skipped when the spec is not next to
    /// the workspace (a standalone export).
    #[test]
    fn genre_tables_match_canon_text() {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../docs/palimpsest/spec/00_CANON.md"
        );
        let Ok(canon) = std::fs::read_to_string(path) else {
            std::eprintln!("skipped: {path} not found");
            return;
        };
        let lines: Vec<&str> = canon.lines().collect();
        let mut s52 = 0;
        let mut s59 = 0;
        let mut mfr = 0;
        for line in &lines {
            let c = cells(line);
            // §5.2: | # | `genre.x` | Display | ARC | TEC | MYT | VRV | aether | gravity | light | Lucre | Aether | Tier |
            if c.len() == 13 && c[1].starts_with("`genre.") {
                let id = c[1].trim_matches('`');
                let g = Genre::from_id(id).expect("canon genre is in the table");
                let r = g.row();
                assert_eq!(c[0], (g.index() + 1).to_string(), "{id} #");
                assert_eq!(c[2], r.display_name, "{id}");
                for (i, a) in Axiom::ALL.into_iter().enumerate() {
                    assert_eq!(c[3 + i], r.axioms.get(a).to_string(), "{id} {a:?}");
                }
                let aether = r.aether_axiom.map_or("none", |a| match a {
                    Axiom::Arcana => "arcana",
                    Axiom::Techne => "techne",
                    Axiom::Mythos => "mythos",
                    Axiom::Verve => "verve",
                });
                assert_eq!(c[7], aether, "{id}");
                assert_eq!(c[8], r.gravity_bp.to_string(), "{id}");
                assert_eq!(c[9], r.ambient_light.to_string(), "{id}");
                assert_eq!(c[10], r.lucre_skin, "{id}");
                assert_eq!(c[11], r.aether_skin, "{id}");
                let tier = if r.tier == Tier::Core {
                    "core"
                } else {
                    "secret"
                };
                assert_eq!(c[12], tier, "{id}");
                s52 += 1;
            }
            // §5.9 identity: | suffix | air | fall | friction | spawn | palette | music | themes | materials |
            if c.len() == 9 && c[5].starts_with("`#") {
                let g = Genre::ALL
                    .into_iter()
                    .find(|g| g.suffix() == c[0])
                    .expect("canon §5.9 suffix");
                let r = g.row();
                assert_eq!(c[1], r.air_accel_bp.to_string(), "{}", c[0]);
                assert_eq!(c[2], r.fall_dmg_bp.to_string(), "{}", c[0]);
                assert_eq!(c[3], r.ground_friction_bp.to_string(), "{}", c[0]);
                assert_eq!(c[4], r.spawn_block_max.to_string(), "{}", c[0]);
                let palette: Vec<String> = r.palette.iter().map(|p| format!("#{p:06X}")).collect();
                assert_eq!(c[5].trim_matches('`'), palette.join(" "), "{}", c[0]);
                assert_eq!(c[6], r.music, "{}", c[0]);
                assert_eq!(c[7], r.enemy_themes, "{}", c[0]);
                let mats: Vec<&str> = c[8]
                    .split_whitespace()
                    .map(|m| m.trim_matches('`'))
                    .collect();
                assert_eq!(mats, r.materials, "{}", c[0]);
                s59 += 1;
            }
            // §5.9 manufacturers: | suffix | `mfr.x.y`: trait. `mfr.x.z`: trait. |
            if c.len() == 2 && c[1].starts_with("`mfr.") {
                let g = Genre::ALL
                    .into_iter()
                    .find(|g| g.suffix() == c[0])
                    .expect("canon manufacturer suffix");
                let parsed: Vec<(String, String)> = c[1]
                    .split("`mfr.")
                    .skip(1)
                    .map(|piece| {
                        let (name, rest) = piece.split_once("`: ").expect("`mfr.x`: trait");
                        let text = rest.trim().trim_end_matches('.').replace('`', "");
                        (format!("mfr.{name}"), text)
                    })
                    .collect();
                let code: Vec<(String, String)> = g
                    .row()
                    .manufacturers
                    .iter()
                    .map(|m| (m.id.to_string(), m.signature.to_string()))
                    .collect();
                assert_eq!(parsed, code, "{}", c[0]);
                mfr += 1;
            }
        }
        assert_eq!((s52, s59), (16, 16));
        assert_eq!(mfr, 15, "12 core rows plus oneiric, redacted and ur_text");
    }

    /// SPEC: canon §5.2 structure: 16 rows, 12 core, ids unique with the
    /// `genre.` prefix, axioms in `0..=10`, the core genres have exactly two
    /// manufacturers each, and `from_id` round-trips.
    #[test]
    fn genre_table_structure() {
        for (i, g) in Genre::ALL.into_iter().enumerate() {
            assert_eq!(g.index(), i);
            assert!(g.id().starts_with("genre."));
            assert_eq!(Genre::from_id(g.id()), Some(g));
            assert!(g.axioms().0.iter().all(|&v| v <= AXIOM_MAX));
            assert!(g.ambient_light() <= 15 && g.row().spawn_block_max <= 15);
            assert_eq!(g.is_core(), i < 12);
            if g.is_core() {
                assert_eq!(g.row().manufacturers.len(), 2, "{}", g.id());
            }
            for mf in g.row().manufacturers {
                assert!(mf.id.starts_with(&["mfr.", g.suffix(), "."].concat()));
            }
            for other in &Genre::ALL[i + 1..] {
                assert_ne!(g.id32(), other.id32());
            }
        }
        assert_eq!(Genre::from_id("genre.nope"), None);
        assert_eq!(Genre::CORE.len(), 12);
        assert!(Genre::CORE.iter().all(|g| g.is_core()));
    }

    /// SPEC: canon §5.4 NPC rule: tolerance 2 where the axiom is at least 5.
    #[test]
    fn npc_origin_tol_rule() {
        assert_eq!(Genre::Cyberpunk.npc_origin_tol(), [0, 2, 0, 2]);
        assert_eq!(Genre::UrText.npc_origin_tol(), [2, 2, 2, 2]);
        assert_eq!(Genre::Unwritten.npc_origin_tol(), [0, 0, 0, 0]);
        assert_eq!(Genre::GothicHorror.npc_origin_tol(), [2, 0, 2, 0]);
    }

    /// SPEC: canon §5.1 axiom metadata round trip.
    #[test]
    fn axiom_metadata() {
        for (i, a) in Axiom::ALL.into_iter().enumerate() {
            assert_eq!(a.index(), i);
            assert_eq!(Axiom::from_index(i), Some(a));
            assert!(a.id().starts_with("ax."));
        }
        assert_eq!(Axiom::from_index(4), None);
        assert_eq!(Axiom::Verve.name(), "VERVE");
    }

    /// SPEC: canon §5.8 seam rule on axiom vectors and §5.2 L1 metric.
    #[test]
    fn axioms_helpers() {
        let a = Genre::Cyberpunk.axioms();
        let b = Genre::FairyTale.axioms();
        assert_eq!(a.max_each(b), Axioms([10, 9, 8, 5]));
        assert_eq!(a.l1_distance(b), 9 + 8 + 6 + 2);
        assert_eq!(a.l1_distance(a), 0);
    }
}
