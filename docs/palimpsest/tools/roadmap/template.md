# 21 — Implementation Roadmap

As of 2026-10-11. Status: BINDING. Conforms to canon-1.11.

This chapter turns the specification into a build plan. It states what the Rust workspace holds today, what ships at Early Access, at 1.0 and in the first Folio Expansions, the milestones with their dates and testable exit criteria, the {{N_ALL}} work packages (WPs) that coding agents pick up, the content production plan, the engineering process every agent follows, the risk register and the dependency graph. It owns no content namespace (Canon §19) and adds no simulation state. Product-owner rows in `DECISIONS.md` bind it; where a chapter still disagrees with such a row, this chapter follows the row and lists the conflict under Canon gaps.

## Imports

| Source | Consumed |
|---|---|
| `DECISIONS.md` items 1–104 | every tier decision: 6 Early Access genres (41), USD 19.99 at Early Access and 1.0 (42), Folio Expansion order (53), roster size (69), crossplay (73), Margin hub (77), PC only through 1.0 (98), and the newest rows 81–104 (ghost XP, Mercy, Lucre drops, limb loss, co-op skill growth, invader rewards, cosmetic mods, Somnambule, chargers, Crowns, hunter identity, FOV, host migration, ink gore, movement kit, asynchronous traces, creator presets, time of day and weather, Second Edition, streaming votes) |
| `DESIGN_BRIEF.md` §1–§9 | high concept, omni-genre engine, 12 core plus 4 secret genres, run shape, technology, monetization stance, document contract |
| Canon §1–§3, §5.2, §16.2, §17, §19, §21, §22 | numeric and RNG rules agents must follow, the genre table, Origins and Echoes, run constants and `GENRE_REPEAT_RULE`, namespace ownership, acceptance tests 1–47, invariants |
| Doc 01 §2, §3.13, §4.2, §9.3, §10, §11 | the 18 pillar tests and the Early Access gate, signature moments SM-01 to SM-11, parity items PAR-11 to PAR-13, KPIs, the question default Q-19 (Q-11 is superseded by DECISIONS 98) |
| Doc 02 §2, §3, §6, §11, §15–§18 | stack and pins, the 20-crate workspace, the determinism plan, the content pipeline, performance budgets, the testing strategy, CI, channels and flags, coding conventions |
| Docs 03–18 | the sections each WP cites and every acceptance test on disk |
| Docs 19 and 20 | the schema and validation chapter and the generated content catalog; both are being written (Interface assumptions) |
| `audit/canon_gaps_triage.md` | rulings Q1–Q12, interface conflicts C-01 to C-32 and the order of work of its §5.3 |
| `palimpsest/README.md`, `palimpsest/AGENTS.md`, commit `efece38` | the state of the vertical slice (§2.1) |

## 1. Conventions

- **Work packages** are `WP-NNN`, numbered so that every dependency has a smaller number than its dependent. A WP is one merge unit of at most 3 pull requests.
- **Milestones** are `M0` to `M13`. A milestone closes on the first build of `main` on which every exit criterion of §3.2 passes.
- **Test notation.** `C n` is Canon §21 test `n`. `NN: n` is acceptance test `n` of chapter `NN`. Pillar tests use their IDs (doc 01 §2). `SM-NN` is a doc 01 §3.13 scenario.
- **Size** is in agent-days: one coding agent working one working day, including its tests, review fixes and CI time. The calendar counts working days Monday to Friday, day 0 being Monday 2026-10-12.
- **Capacity.** The plan assumes 8 concurrent agent lanes and one product owner who curates art and answers questions (RQ-10). §4.5 shows that 8 lanes finish every Early Access WP inside its milestone.
- **Tiers.** *Slice* is what the workspace holds on 2026-10-11, *EA* is Early Access, *1.0* is the full release, *Post* is after 1.0.
- All dates and times are UTC.

## 2. Scope tiers

### 2.1 The vertical slice on 2026-10-11

Committed state at `efece38`, verified by running the suite:

- **Workspace.** `palimpsest/` pins Rust 1.99.0 (edition 2024, resolver 3), turns `overflow-checks` on in the dev, release and test profiles, forbids `unsafe` code, denies `clippy::float_arithmetic` and `clippy::float_cmp`, and disallows `f32`, `f64`, `HashMap` and `HashSet` in `clippy.toml`.
- **Crates.** Three of the 20 crates of doc 02 §3.2 (Crates) exist: `palimpsest_math` (`no_std`; integer primitives, Q32.32 `Fx`, the 16,385-entry sine table, CORDIC `atan2_bam`, geometry, xoshiro256++, streams and keyed draws, FNV-1a, BLAKE3, seeds, the power tables), `palimpsest_data` (content IDs, `id32`, the collision-checked registry with aliases, text keys, hex `u64`, and the no-float JSON check behind the `loader` feature) and `palimpsest_sim` (formula modules `time`, `entity`, `genre`, `contradiction`, `stats`, `progression`, `movement`, `damage`, `status`, `loot`, and a `Sim` skeleton whose stand-in world proves save, restore and `state_hash`).
- **Tests.** 465 tests pass: 196 in `palimpsest_math`, 43 in `palimpsest_data`, 224 unit tests and 2 integration tests in `palimpsest_sim`. Canon §21 tests 1–44 are named `canon_tNN_*`; tests 31, 36 and 37 check the rule without the generators that later WPs add. Doc 02 tests 1, 5, 6, 19 and 36 are named, and the worked examples behind doc 02 tests 2–4 and 7–11 are covered.
- **CI.** `.github/workflows/palimpsest-ci.yml` runs `cargo fmt --check`, `cargo clippy -D warnings` and `cargo test` on Rust 1.99.0 for every change under `palimpsest/`.
- **Open defaults.** 18 `GAP:` comments mark choices the code made where the spec was silent.
- **Not present.** `xtask`, the Rust content loader, schemas, content files, the entity store, grids, worldgen, AI, item generation, netcode, the renderer, audio, UI, platform layers and services.
- **In progress in the working tree when this chapter was written** (uncommitted, owned by other agents): the canon-1.11 catch-up in `palimpsest_math`, `palimpsest_data` and `palimpsest_sim` (`derive`, object-keyed draws, offer generators, `nar_rng`, `pres_hash`, tests `canon_t45` to `canon_t47`), the CI workflow, the first doc 19 schemas, the namespace registry under `content/00/meta/` and the Python reference validator `tools/validate_content.py`. WP-001, WP-005 and WP-006 close when that work and the rest of their deliverables are on `main`.

### 2.2 Early Access scope

| Area | Early Access content | Binding source |
|---|---|---|
| Genres | Mythic Fantasy, Gothic Horror, Cyberpunk, Weird West, Wuxia and Steampunk as lattice genres; `genre.ur_text` as the depth-25 Folio of every full-story run; `genre.unwritten` for Unwritten only. No secret exit opens. | DECISIONS 41; §2.3 |
| Modes | `mode.abridged` (the Early Access full-story run: 15 Folios, all five Editors, the Ur-Text and the closing manuscript), `mode.novella`, `mode.daily`, `mode.weekly_gauntlet`, `mode.seeded` and `mode.custom` with the Novella and Abridged structures, and the overlays `mode.ff_off`, `mode.ff_full`, `mode.loot_shared`, `mode.invasion_open` | DECISIONS 2, 51; §2.3; RQ-01 |
| Price | base game USD 19.99, kept by Early Access owners at 1.0; Codex premium track USD 9.99; cosmetics USD 1.99 to 9.99; no premium currency | DECISIONS 1, 42, 58 |
| Co-op and PvP | 1 to 4 players; Barony-style ghosts; friendly fire 50% direct and 100% area; host migration with hot, warm and cold paths; opt-in invasions with the Redacted outcome and cosmetic-only invader rewards | DECISIONS 15, 31, 32, 50, 76, 86, 93 |
| World | fixed terrain with limited building of placeables; 250 mm micro-voxels; no mounts or vehicles; no item durability; the movement kit with a 2,000 mm mantle, sprint-slide and a grappling tool item; one time of day and one weather state per Folio | DECISIONS 16, 37, 55, 65, 95, 99 |
| Narrative and presentation | text-only Narrator in the margins; the procedural closing manuscript; ink gore with a red-blood toggle; original adaptive score; per-genre manuscript UI skins; a character creator of 12 presets per kin with palette and marking colors | DECISIONS 23, 38, 39, 46, 94, 97 |
| Meta | the walkable first-person 3D Margin hub with stations, vendors and trophy slots; resident arcs; the 91-node Bindery; 24 Echoes, each with a Recollection trial; the Glyph Library; Errata Grades 0 to 20 with Scribe's Mercy; 46 achievements | DECISIONS 12, 33, 47, 52, 77, 78, 80, 82; §2.3 (f) |
| Economy | Inkwell (160 Ink per pull, pity by pull 60, 75/25 featured split, spark at 100), account Reliquaries, the Codex with its Quill, Fortune Engines with cursed capsules, offline-complete end-of-service mode | DECISIONS 1, 57–60; brief §8 |
| Enemies | 12 or more roster templates and 2 mini-bosses per Early Access genre, elites, champions, the five Editors, run-scoped faction alliances | DECISIONS 45, 69–71 |
| Platforms and stores | Windows x86_64 on Steam and on the Epic Games Store, with crossplay through `svc.relay`; Steam Deck Verified through Proton. Linux stays a server and CI target only. | DECISIONS 6, 73, 98 |
| Languages | 15 locales: en-US, fr-FR, de-DE, es-ES, es-419, pt-BR, it-IT, pl-PL, ru-RU, uk-UA, tr-TR, ja-JP, ko-KR, zh-Hans, zh-Hant | DECISIONS 40 |
| Modding | JSON data packs and sandboxed WASM effects loaded from local folders | DECISIONS 43 |
| Replays and ranked | full deterministic replays with a killcam, Daily and Weekly boards verified by headless re-simulation | DECISIONS 48, 51 |
| Not at Early Access | the 25-Folio Standard run, Endless Marginalia, Second Edition, the other 6 core genres, Oneiric and The Redaction, secret exits, Steam Workshop, the on-device Scribe, asynchronous traces, streaming votes | §2.4, §2.5 |

### 2.3 Early Access genre-set rules

These rules bind every build whose content set holds fewer than the 12 base core genres. They change nothing once all 12 ship, because doc 12 §5.4 (Volume and run rules) proves that rules 2 to 4 then leave at least 3 eligible genres. Each is filed under Canon gaps for its owner.

**(a) Lattice relaxation cascade.** With 6 core genres, a 5-Folio Volume leaves 2 eligible genres when Folio 5 is chosen (the 4 earlier genres of the Volume are excluded), and the Abridged Folios 12 and 14 leave 1. `lattice_candidates` (doc 12 §5.5, Exit-offer algorithm) asserts at least 3, so every structure fails without a rule change. The cascade, applied inside the eligibility filter before the weights:

```
e0 = core genres passing rules 2, 3 and 4 of doc 12 §5.4
if len(e0) >= 3: return e0
e1 = core genres passing rules 2 and 4            // drop the run cap of 3 appearances
if len(e1) >= 3: return e1
e2 = core genres except the current genre and, when m >= 3, history[m - 2]
return e2                                          // len(e2) >= 4 with 6 core genres
```

Rule 5 (freshness) and the 3 `range` calls of `rng.lattice` are unchanged, so the draw count never depends on the cascade. Example: a Novella run chooses Folio 5 after Mythic Fantasy, Gothic Horror, Cyberpunk and Weird West. `e0` is {Wuxia, Steampunk}, `e1` is the same 2, and `e2` is {Mythic Fantasy, Gothic Horror, Wuxia, Steampunk}, from which the 3 candidates are drawn.

**(b) Folio 1 for an Origin outside the genre set.** Rule F1 of doc 14 §2.2 (Run initialization) uses the host's Origin genre. When that genre is not in the content set, Folio 1 is the shipped core genre with the highest doc 12 §5.2 (Core edge weights) weight from it, ties to lattice order. With the Early Access set: Cosmic Horror → Gothic Horror (90), Space Opera → Cyberpunk (80), Post-Apocalypse → Weird West (80), Hardboiled Noir → Cyberpunk (90), Fairy Tale → Mythic Fantasy (90), Mythic Antiquity → Mythic Fantasy (90).

**(c) Origins, Echoes and kits.** All 12 core Origins and all 24 Echoes ship at Early Access, so the required Echoes Loki (Mythic Antiquity) and Jörmungandr (Cosmic Horror) are playable (DECISIONS 28). Kit items carry the Origin skin (doc 14 §2.4.4, Margin Translation), so the skins of every standard, alternative, Script and Echo kit template ship in all 12 core genres. Every other per-genre skin ships with its genre. Exotics whose home genre is not shipped stay out of loot pools: 12 of the 24 of doc 11 §8.2 (Exotics (24)) ship at Early Access.

**(d) Secret genres.** No secret exit opens before 1.0. `genre.ur_text` ships complete for the final Folio; `genre.unwritten` ships as a secret genre, unlocked by its canon rules.

**(e) Pillar tests at the Early Access gate.** Doc 01 §2.7 (Pillar records and the release gate) requires all 18 tests. Their wording assumes Standard runs and 12 genres, so the Early Access gate reads them as follows; every other test reads as written.

| Test | Doc 01 wording | Early Access reading |
|---|---|---|
| `pillar.one_ruleset.t2` | all 66 core pairs | unchanged: GPV records for all 16 genres exist from M2 |
| `pillar.one_ruleset.t3` | 12 core genres, 12 display names | the 6 Early Access genres, 6 display names |
| `pillar.out_of_story.t1` | 1,000 solo Standard bot runs | 1,000 solo `mode.novella` bot runs |
| `pillar.out_of_story.t2` | 12 core genres × 13 Callings | 6 genres × 13 Callings |
| `pillar.living_page.t1` | Standard bot runs | `mode.novella` bot runs |
| `pillar.living_page.t3` | each core genre, 108 cells | each Early Access genre, 54 cells |
| `pillar.party_of_errata.t1` | Volume I clear rate in Standard Grade-0 runs | `boss.editor_v1` deaths at Folio 5 of `mode.novella` Grade-0 runs |
| `pillar.fair_page.t3` | Standard Folios 1–10 | `mode.novella` Folios 1–10 |

Novella plays Folios 1 to 15 at the Standard depth `d(f) = f` (doc 14 §8.3, Novella), so these readings keep the depth curve of the original tests.

**(f) Achievements.** 46 of the 58 records of doc 14 §10 (Achievements) register at Early Access. The other 12 need 1.0 content and register at 1.0: `ach.ur_text`, `ach.marginalia_30`, `ach.marginalia_50`, `ach.marginalia_100`, `ach.atlas`, `ach.oneiric`, `ach.redacted`, `ach.unwritten`, `ach.secret_trio`, `ach.swift_volume`, `ach.speed_win`, `ach.full_house`.

**(g) Scripts.** The 6 `bundle.script.lost_chapter_<g>` Scripts of unshipped genres stay out of every banner pool until their genre ships; the odds tables change on that update's date and are disclosed before it (doc 15 §3.11, Odds disclosure).

### 2.4 1.0 scope

1.0 adds the other 6 core genres (Cosmic Horror, Space Opera, Post-Apocalypse, Hardboiled Noir, Fairy Tale, Mythic Antiquity) in three free updates, then Oneiric, The Redaction, generated Unwritten Folios and every secret exit; `mode.standard` (the 25-Folio run), `mode.endless` with Marginalia to Folio 9999, Marginal mutations and Marginalia modifiers; the Standard structure in Seeded and Custom runs with `run.mut.short_book`; the 12 remaining achievements; 192 roster templates and 28 mini-bosses; all 24 Exotics; asynchronous traces (DECISIONS 96, delivered with Update 1); opt-in streaming votes (DECISIONS 104, delivered with Update 2); Second Edition, the post-game in which the Errata who clear the Ur-Text continue with their gear and levels through Folios of two fused genres against 5 revised Editors and the Author, with a second ending layer (DECISIONS 100–103); Steam Workshop with the `svc.workshop` mirror for Epic players; and the opt-in on-device Scribe for en-US. Abridged, Novella, Daily and Weekly continue unchanged. The platforms and the price stay as at Early Access.

### 2.5 Post-1.0

- **Folio Expansion 01, Age of Sail and Pirates**, and **Folio Expansion 02, Kaiju and Mecha** (DECISIONS 53). Each adds one genre under doc 12 §25 (Folio Expansion genres): a Canon §5.2 row with tier `core`, a vector keeping the minimum L1 distance of 6, a full GPV, 1 to 3 Laws, 2 tilesets, 6 hazards and 6 events, a 12-template roster, 2 mini-bosses, skins, 2 manufacturers, 2 Echoes and 3 Scripts, a music track and the SKU `store.sku.folio_exp_<nn>` at USD 12.99, inside DECISIONS 42's band of 9.99 to 14.99. DECISIONS 55 binds them: ships are Folio layouts with decks, not vehicles; mecha are powered-armor skins and Going Native forms; kaiju are rosters and mini-bosses; nothing is piloted.
- **Platforms.** No console, Switch 2, native macOS or native Linux port is planned (DECISIONS 98, which replaces the console plan of DECISIONS 6 and doc 02 §14). The `Platform` and `Transport` traits stay, so a later decision costs a backend, not a redesign.

### 2.6 Systems by tier

| System | Slice | Early Access | 1.0 | Post-1.0 |
|---|---|---|---|---|
| Simulation core (doc 03) | integer and fixed-point primitives, tick timers, `EntityId`, save and hash skeleton | entity store, scheduler, all six grids, controller with the DECISIONS 95 kit, explosions, block edits, event bus | unchanged | unchanged |
| Determinism and tooling (doc 02) | float and `HashMap` bans, canon tests | `xtask`, 5-target matrix (Linux verifiers must match Windows clients), golden replays, SyncTest, desync dumps, `pal` tools | unchanged | unchanged |
| World generation and genres (doc 12) | genre table, Contradiction, Anachronism Edge, World Law formulas | 6 genres, Ur-Text Folios, cascade of §2.3, bleed, seams, secret rooms, time of day and weather | 16 genres, secret exits, gravity flip, Marginalia modifiers, fused Second Edition Folios | expansion genres |
| Combat (doc 05) | damage pipeline, wards, blocking | melee, ranged, armor, counters, jams, grappling tool | unchanged | expansion skins |
| Attributes, skills, body (doc 04) | derived stats, skills, levels | full chapter, crippled-pool severing (DECISIONS 84) | unchanged | unchanged |
| Magic (doc 06) | glyph dependency rule | full chapter, Glyph Library | unchanged | expansion skins |
| Crafting and building (doc 07) | — | full chapter, placeables only | unchanged | expansion materials |
| Alchemy (doc 08) | — | full chapter | unchanged | expansion skins |
| Stealth (doc 09) | per-target sneak eligibility | full chapter | unchanged | unchanged |
| Identification (doc 10) | type-hiding rule | full chapter with pact curses | unchanged | unchanged |
| Items and loot (doc 11) | rarity, quality, enhancement, Marginalia armor HP | full chapter; 12 Exotics | 24 Exotics, Marginalia Emendations | expansion items |
| Enemies and AI (doc 13) | ranks, Erasure formulas | AI, Director, 96 roster templates, 12 mini-bosses, 5 Editors | 192 templates, 28 mini-bosses, 5 revised Editors and the Author | 12 templates and 2 mini-bosses per expansion |
| Run structure and meta (doc 14) | — | EA modes of §2.2, Margin hub, Bindery, 24 Echoes, 46 achievements, Grades | Standard, Endless, Second Edition, 58 achievements, streaming votes | expansion Echoes |
| Monetization and compliance (doc 15) | — | Inkwell, account Reliquaries, Codex, store, compliance layer, offline mode | unchanged | expansion SKUs and banners |
| Netcode, co-op, invasions (doc 16) | — | Steam, LAN, direct IP and relay sessions, EOS crossplay, host migration, invasions, replays, ranked verification | asynchronous traces, streaming votes, Workshop mirror | unchanged |
| UI, accessibility, localization (doc 17) | — | full HUD and menus, UI skins for the shipped genres, accessibility suite, 15 locales | remaining genre skins | expansion skins |
| Art, audio, narration (doc 18) | — | global assets, asset sets for the 6 genres, the Ur-Text and the Unwritten, text-only narration, closing manuscript, ink gore | 8 more genre sets, Scribe | expansion sets |
| Modding (doc 02 §13) | — | data packs and WASM from local folders | Steam Workshop and mirror | unchanged |
| Platform | — | Windows on Steam and Epic Games Store; Steam Deck through Proton | unchanged | none planned |
| Telemetry (doc 02 §17.4) | — | opt-in usage, crash and desync reports | unchanged | unchanged |
| Content pipeline (docs 19, 20) | no-float JSON check | schemas, validator, extraction, generated catalog | unchanged | unchanged |

## 3. Milestones and calendar

### 3.1 Milestone table

| Milestone | Name | Target | Work packages |
|---|---|---|---|
| M0 | Foundations | Fri 2026-11-13 | WP-001–WP-008 |
| M1 | Headless world core | Fri 2027-01-15 | WP-009–WP-021 |
| M2 | Headless rules | Fri 2027-03-12 | WP-022–WP-042, WP-079, WP-080 |
| M3 | Headless game | Fri 2027-05-14 | WP-043–WP-048, WP-081 |
| M4 | First playable client | Fri 2027-05-28 | WP-049–WP-055, WP-057, WP-060 |
| M5 | Online, platforms and economy | Fri 2027-06-25 | WP-059, WP-062–WP-076 |
| M6 | Content complete (Early Access candidate) | Fri 2027-08-06 | WP-056, WP-058, WP-061, WP-077, WP-078, WP-082–WP-099 |
| M7 | Early Access | Thu 2027-09-09 | WP-100–WP-102 |
| M8 | Update 1: Fairy Tale, Mythic Antiquity, asynchronous traces | Thu 2027-11-04 | WP-103–WP-107 |
| M9 | Update 2: Cosmic Horror, Hardboiled Noir, streaming votes | Thu 2027-12-16 | WP-108–WP-112 |
| M10 | Update 3: Space Opera, Post-Apocalypse | Thu 2028-02-10 | WP-113–WP-116 |
| M11 | 1.0 | Thu 2028-04-06 | WP-117–WP-126 |
| M12 | Folio Expansion 01: Age of Sail and Pirates | Thu 2029-01-18 | WP-127 |
| M13 | Folio Expansion 02: Kaiju and Mecha | Thu 2029-10-18 | WP-128 |

A milestone that misses its target moves every later milestone by the same number of working days. Early Access ships only when every M7 criterion passes on one candidate build: when a criterion fails, the launch moves to the next Thursday on which all pass, and no criterion is waived.

### 3.2 Exit criteria

**M0 Foundations.** (1) `cargo test --workspace` is green with Canon §21 tests 1–47 named, 45–47 included. (2) `cargo xtask check-deps`, `float-scan`, `lint-size` and `spec-refs` pass on `main` (C 33; 02: 21, 22, 44). (3) `pal schema export` and `pal validate` run in CI and pass 02: 17 and 19, C 29 and C 34 through the loader. (4) The pull-request pipeline of doc 02 §17.1 (Pipeline) runs on every pull request in at most 25 minutes. (5) `spec_lint.py --chapter 21` reports nothing.

**M1 Headless world core.** (1) Doc 03 tests 1–52 are green. (2) C 24, 27, 30 and 32 and doc 02 tests 20, 23–26, 30 and 31 are green. (3) The world-only golden replays give byte-identical `hashes.json` on all five targets of doc 02 §6.4 (Cross-platform determinism matrix), and SyncTest depth 8 passes. (4) On `deck-01`, `perf_saturation_4p` keeps the world groups (`perf.deck.sim.movement`, `perf.deck.sim.push`, `perf.deck.sim.liquids`, `perf.deck.sim.gases`, `perf.deck.sim.fire`, `perf.deck.sim.temperature`, `perf.deck.sim.light`, `perf.deck.sim.sound`, `perf.deck.sim.explosions`, `perf.deck.sim.block_updates`, `perf.deck.sim.random_ticks`) within their mean and worst budgets.

**M2 Headless rules.** (1) Every acceptance test assigned to WP-022 to WP-042 is green: docs 04, 06, 07, 08, 09 and 10 completely, doc 05 except 41 (M5) and 48 (M3), doc 11 except 28 (1.0) and 36 (M5), the doc 13 AI tests, and C 13–23, 25, 26, 28, 35–42 and 47. (2) SyncTest depth 8 passes over golden replays that exercise combat, magic, alchemy, stealth and crafting fixtures. (3) `pal extract` and `pal catalog` regenerate `spec/20_CONTENT_CATALOG.md` byte-identically from the chapter tables.

**M3 Headless game.** (1) C 1–47 are all green, test 31 comparing block data. (2) The doc 12 pull-request sweep (12: 18) passes over the 6 Early Access genres and the Ur-Text with today's 7-template rosters. (3) Doc 13 is green except 47 (M5); doc 14 is green for every test of WP-046. (4) SM-01 to SM-11 pass headless (01: 20). (5) `perf.ci.bot_tps` is at least 6,000, and 200 nightly bot runs of `mode.novella` and `mode.abridged` finish with no panic, no desync between two replicas and no Folio longer than 36,000 ticks (doc 02 §16, Testing strategy, item 6).

**M4 First playable client.** (1) A tester plays a first run with contextual tips and a `mode.abridged` run from the Lectern to the closing manuscript on Windows and on Steam Deck through Proton, with greybox art in the 6 Early Access genres. (2) Doc 02 tests 12–16, 18, 34, 37, 42 and 43, the doc 17 tests of WP-053 to WP-055 and WP-057, and the doc 18 tests of WP-050 to WP-052 are green. (3) The client smoke test passes under 16 schedule shuffle seeds. (4) SM-10 and then SM-06 are playable in the client (doc 01 §11 Q-19).

**M5 Online, platforms and economy.** (1) Docs 15 and 16 are green; doc 14 tests 16–18, 24, 26, 36, 39–41, 46, 47, 49 and 51 are green. (2) `pillar.fair_page.t1`, `pillar.fair_page.t2` and `pillar.fair_page.t3` pass. (3) A 4-player Abridged run over Steam Datagram Relay and one over LAN finish; an Epic Games Store client and a Steam client finish a Folio together over `svc.relay`; an invasion runs from beacon to Redaction and from beacon to invader death. (4) Nightly 4-player bot co-op shows 0 desyncs over 1,000 bot hours.

**M6 Content complete.** (1) The generated catalog meets every Early Access count of §5.3 and §5.4. (2) 0 keys are missing in 15 locales after the text freeze of 2027-07-09. (3) Doc 17 and doc 18 are green except the Scribe tests of WP-120; doc 02 tests 28, 29, 33, 35, 40, 41 and 47 and doc 14 test 48 are green. (4) Every `perf.deck.*` budget is green on `deck-01` and on the minimum-PC runner (PAR-13). (5) 17 of the 18 pillar tests pass on the candidate build, all but `pillar.one_ruleset.t3`. (6) The release candidate is cut.

**M7 Early Access.** (1) `release_gate(pillars, results, build, Gate::EarlyAccess)` returns true: all 18 pillar tests pass on the candidate build (doc 01 §2.7). (2) The build has passed doc 02 §17.2 (Release channels) in order: 3 consecutive green nightlies to `playtest`, 7 days on `playtest` with no P0 and crash-free sessions ≥ 99.0% to `beta`, 7 days on `beta` with crash-free sessions ≥ 99.5% to `stable`. From the M6 cut on 2027-08-06 the earliest `stable` promotion is 2027-08-23. (3) Ratings exist for the doc 15 §8.6 (Rating targets) set. (4) Every Steam Deck Verified checklist item of doc 02 §14 is green, with Proton in place of the native Linux build (DECISIONS 98). (5) The economy service is configured with `T0 = 1820448000`.

**M8, M9, M10 Updates.** For each update: the two genres meet §5.3; doc 12 test 18 sweeps the enlarged genre set; `pillar.one_ruleset.t3` is re-run with the new genres, because its earlier pass carries forward only while the genre set is unchanged (doc 01 §2.7); `release_gate` with `Gate::Later` returns true. M8 also ships asynchronous traces (WP-103) and M9 streaming votes (WP-108).

**M11 1.0.** Every chapter test is green, the 1.0-only tests included (12: 22, 23, 36; 14: 2, 7, 8, 27, 28, 44; 11: 28; 18: 20–24, 43, 44). The pillar tests pass with doc 01's original wording over 12 core genres and `mode.standard`. All 58 achievements register. A Second Edition run continues from an Ur-Text victory and replays to identical state hashes.

**M12, M13 Expansions.** The new Canon §5.2 row exists; doc 12 test 44 and Canon test 40 hold with the new vector; the expansion meets §5.3; its SKU and banner pass doc 15 tests 23–29.

### 3.3 Early Access date and `T0`

Early Access launches on **Thursday 2027-09-09** at 00:00:00 UTC. Doc 15 §3.2.3 (Schedule) defines `T0` as that instant: **`T0 = 1820448000`**. Doc 15's examples use `T0 = 1792886400` (2026-10-25), which must move to 1820448000 (Interface assumptions). With the new `T0`: banner period p001 is [2027-09-09, 2027-10-07); one week after `T0` is 1821052800; a 28-day banner ends at 1822867200. Codex `N` releases at `T0 + 84 × (N − 1)` days (doc 15 §6.6, Cadence): Codex 001 on 2027-09-09, 002 on 2027-12-02, 003 on 2028-02-24, 004 on 2028-05-18.

### 3.4 Post-Early Access cadence

Free updates arrive every 6 to 8 weeks (PAR-11): Update 1 eight weeks after launch, Update 2 six weeks later, before the year-end holidays, Update 3 and 1.0 eight weeks apart. Genre order follows lattice affinity and asset reuse: Fairy Tale and Mythic Antiquity (both 90 to Mythic Fantasy), Cosmic Horror and Hardboiled Noir, then Space Opera and Post-Apocalypse (RQ-08). From M7 on, 3 of the 8 lanes stay reserved for live fixes; post-launch content WPs use the other 5. Folio Expansions follow 1.0 at 9 to 12 months each (doc 15 §1.1).

### 3.5 Doc 02's task order

Doc 02 §18 (Coding conventions for AI agents) item 8 lists an engineering task order with labels M0 to M15 that collide with this chapter's milestones. Its steps map to WPs as follows; this chapter's labels govern (Canon gaps G9).

| Doc 02 §18 step | Work packages |
|---|---|
| workspace, CI, xtask | WP-002, WP-003, WP-004 |
| math | slice, WP-001 |
| data, schemas, ID registry, content hash | WP-005, WP-006, WP-007 |
| store, scheduler, Sim API, serialization, hash, golden harness, SyncTest | WP-009–WP-012 |
| grids, movement, collision | WP-013–WP-021 |
| damage and statuses | WP-022–WP-025 |
| items, loot, crafting | WP-031, WP-032, WP-036–WP-038 |
| perception, AI | WP-034, WP-039 |
| worldgen | WP-040–WP-043 |
| bridge, minimal renderer | WP-049–WP-051 |
| netcode | WP-062–WP-066 |
| audio, input, UI, localization | WP-052–WP-057 |
| platform, saves, telemetry, `palimpsest_econ` | WP-067, WP-072, WP-073 |
| mods | WP-077, WP-078 |
| services | WP-068, WP-074 |
| Deck verification and performance | WP-061 |

## 4. Work packages

### 4.1 Rules

- **Columns.** *Ms* is the milestone. *Owner chapter and sections* cites the chapter number and each section with its title as it stands on disk. *Crate / module* uses doc 02 §3.2's crates and the module paths below. *Depends on* lists WPs that must be merged first. *Acceptance tests* must be named and green when the WP merges. *Size* is agent-days.
- **Module paths inside `palimpsest_sim`.** `api`, `serial`, `store`, `schedule`, `world`, `controller`, `grids::{liquid, gas, fire, thermal, light, sound}`, `explode`, `edit`, `events`, `combat::{melee, ranged, armor}`, `character`, `body`, `magic`, `craft`, `alchemy`, `stealth`, `stealth::security`, `items`, `loot`, `ident`, `ai`, `enemies`, `enemies::boss`, `run`, `genre`. The slice modules `time`, `entity`, `genre` and `contradiction` stay; `movement` moves into `controller`, `damage` and `status` into `combat`, `stats` and `progression` into `character`, and the slice `loot` into `items`, each move keeping its tests.
- **Generated.** The two tables, §4.4, §4.5, §4.6 and §8 are generated from one data file by the roadmap tool, so they agree with each other.

### 4.2 Early Access work packages ({{N_EA}} WPs, {{EA_DAYS}} agent-days)

{{WP_TABLE_EA}}

### 4.3 Work packages after Early Access ({{N_LATER}} WPs)

{{WP_TABLE_LATER}}

All {{N_ALL}} WPs total {{ALL_DAYS}} agent-days.

### 4.4 Critical path

The longest size-weighted chain ends at the Early Access gate WP-102 and holds {{CRIT_N}} WPs, {{CRIT_LEN}} agent-days:

{{CRIT_PATH}}

With one agent per WP it finishes on working day {{CRIT_LEN}}, {{CRIT_END}}. With 8 lanes the gate finishes {{EA_FLOAT}} working days before launch day; that float covers the 17 calendar days of channel promotion, ratings, store review and estimate error. A slip on any critical-path WP moves M3 to M7 by the same number of working days unless the float absorbs it. The path runs through the content loader, the simulation core, combat, items, identification and loot, enemies and bosses, the run layer, the Margin and meta layer, the account economy, the store and compliance layer and release engineering. Near-critical WPs, with their slack in working days: {{NEAR_CRIT}}. The genre data and asset packs start inside M3, once WP-043 and WP-045 merge, and carry at least {{PACK_SLACK}} working days of slack.

### 4.5 Schedule check

As-soon-as-possible finish of each milestone's WPs with unlimited lanes and with 8 lanes:

{{SCHED_TABLE}}

M4's WPs finish early because they depend only on the core; M4's exit criterion (1) needs WP-046 from M3, which fixes its date. Post-launch milestones are set by the update cadence of §3.4, not by their WPs.

### 4.6 Test coverage

Every Canon §21 test and every acceptance test on disk in chapters 01 to 18 is listed by at least one WP. A retired test describes content that DECISIONS 55 or 65 cut; its owner chapter rewrites it (triage C-30, C-31), and the rewritten test joins the WP that owns the section it exercises. A test that a chapter adds later joins that WP the same way; the roadmap tool lists every unassigned test, and the triager assigns it within 2 working days.

{{COVERAGE}}

## 5. Content production plan

### 5.1 Pipeline

1. **Schema.** Doc 19 defines one JSON Schema 2020-12 file per record type. The Rust record types in `palimpsest_data` derive the schemas, `pal schema export` writes `schemas/`, and CI fails when they differ (WP-005).
2. **Extract.** `pal extract` turns every definition table and JSON example of a chapter into records under `content/<owner_doc>/<namespace>/` (WP-079).
3. **Author.** Records that no chapter lists, such as the roster templates that raise each genre to 12 and the second mini-boss, are written directly as JSON in the owner's namespace, together with their en-US Fluent keys in `assets/loc/en-US/<owner_doc>.ftl`.
4. **Validate.** `pal validate` runs the token pass, the schema pass and the semantic pass of doc 02 §11.1 (Files and schemas): ID grammar, references, Canon §19 ownership, weight sums, retired IDs and the genre-set lint of doc 18 test 53 (WP-006). Its rules match the Python reference validator `tools/validate_content.py` of doc 19 case for case.
5. **Sync.** `pal spec-sync` fails when a record listed in a chapter table differs from its JSON record.
6. **Catalog.** `pal catalog` regenerates `spec/20_CONTENT_CATALOG.md` from `content/` and reports the counts of §5.3 and §5.4 per genre (WP-080).
7. **Hash and goldens.** A content change changes `content_hash`, and the same pull request commits the re-blessed golden replays (doc 02 §6.6, Golden replays).
8. **Assets.** Presentation assets pass doc 18 §10.1 (Pipeline): author, export, bake, compress, lint, hash.
9. **Translate.** en-US keys freeze 4 weeks before each content milestone; translators deliver the 14 other locales; CI fails on a missing key or an `en-XA` overflow.

**Source of truth.** A chapter table is normative for every record it lists, and its JSON record must equal it (`pal spec-sync`). A record no chapter lists is normative in `content/`. Doc 20 is generated and never edited by hand.

### 5.2 Order of production

Doc 18 §10.1 fixes the order: every per-genre asset family for the 6 Early Access genres first, then the other 6 core genres, then the secret genres. This chapter adds the Ur-Text and Unwritten sets to the Early Access batch, because the final Folio and Unwritten need them (Canon gaps G13). Global assets ship complete at Early Access. The product owner curates every AI-assisted asset under doc 18 §10.3 (AI-assisted production, curation and licensing) in weekly batches; an uncurated asset never ships.

### 5.3 Counts per core genre

Every core genre meets these counts when it ships, at Early Access, at 1.0 or in an expansion.

| Family | Per core genre | Source | On disk 2026-10-11 |
|---|---|---|---|
| Roster templates | ≥ 12 | DECISIONS 69; doc 13 §8 (Genre rosters) | 7 (Fairy Tale 8) |
| Mini-bosses | 2 | DECISIONS 69; doc 13 §9.3 (Genre mini-bosses) | 1 |
| Factions | as doc 13 §8 lists, at least one sapient native faction | doc 13 §8 | 3 to 4 plus wild |
| World Laws | as doc 12 §4 lists | doc 12 §4 (World Laws) | Weird West 1, Cosmic Horror 3, others 2 |
| Tilesets | 2: primary of 120 prototypes, secondary of 80 | doc 12 §10.2 (Tilesets) | records pending |
| Hazards, events | 6 and 6 | doc 12 §18.2 (Per-genre hazard lists (skin names)), §19.2 (Per-genre event lists (skin names)) | 6 and 6 |
| Weapon skins | 28 archetype cells, minus `—` cells | doc 11 §4.2 (Weapon skins (28 archetypes × 16 genres)) | names on disk |
| Armor skins | 20 (5 slots × 4 classes) | doc 11 §3.3 (Armor, off-hands and ward generators) | — |
| Ward generator and implement skins | 1 noun plus focus and tome | doc 11 §4.3 (Ward generator and implement skins) | nouns on disk |
| Manufacturers | 2 | Canon §5.9; doc 11 §5.2 (Manufacturer table) | 2 |
| Genre trait perk | 1 | doc 11 §7.2 (Perk table (51 perks) and genre traits (16)) | 1 |
| Exotics | as doc 11 §8.2 lists (1 to 3) | doc 11 §8.2 | 24 in all |
| Patron Author | 1 | Canon §16.5; doc 06 §16.1 (Patrons) | 1 |
| Echoes | 2 (global at Early Access) | Canon §16.2 | 24 in all |
| Block array | 1 of 256 layers | doc 18 §10.2 (Budgets) | — |
| Art bible, palette, light rig, post, LUT, UI skin | 1 each | doc 18 §3 (Per-genre art bibles); doc 17 §2 (Genre UI skins) | records on disk |
| Going Native forms | 6, one per kin | doc 18 §5.5 (Going Native forms) | — |
| Music | 1 track of 5 stems | doc 18 §6.1 (Music system) | specified |
| Ambience and SFX | 1 set each | doc 18 §6.3 (SFX taxonomy) | — |
| Damage-type text skins | 15 | doc 18 §7.2 (Damage-type text skins) | specified |
| Name grammar | 1 | doc 18 §7.4 (Name grammars) | specified |
| en-US text keys | 201 (table below) | §5.6 | — |

### 5.4 Totals by tier

| Family | Early Access | Added by 1.0 | 1.0 total | Each expansion |
|---|---|---|---|---|
| Lattice genres | 6 | 6 | 12 | 1 |
| Non-lattice genres | Ur-Text, Unwritten | Oneiric, The Redaction, generated Unwritten | 4 | 0 |
| Roster templates | 96: 6 × 12, 12 Ur-Text, 12 Editor squads (the Unwritten roster) | 96 | 192 | 12 |
| Cross-genre templates | 4 (doc 13 §8.17) | 0 | 4 | 0 |
| New templates to author | 41 (6 × 5, 5 Ur-Text, 6 Editor squads) | 41 | 82 | 12 |
| Mini-bosses | 12 | 16 | 28 | 2 |
| Editors | 5 | 0 | 5 | 0 |
| Second Edition bosses | 0 | 6 (5 revised Editors, the Author) | 6 | 0 |
| World Laws | 12 | 18 | 30 | 1 to 3 |
| Tilesets | 14 | 18 | 32 | 2 |
| Music tracks | 8 (6 genres, Ur-Text, Unwritten) plus the Margin bed | 8 | 16 plus the Margin bed | 1 |
| Exotics | 12 | 12 | 24 | per expansion |
| Echoes | 24 | 0 | 24 | 2 |
| Achievements | 46 | 12 | 58 | per expansion |
| Bindery nodes | 91 | 0 | 91 | 0 |
| UI genre skins | 8 plus the Margin | 8 | 16 plus the Margin | 1 |

The Ur-Text and the Unwritten keep doc 13 §9.3's champion in place of a mini-boss.

### 5.5 Global content at Early Access

The 13 Calling kits with alternative and Script kits; 24 Echoes with Recollection trials and the quadruped and serpentine bodies; the 5 Editors with verses for every dominant axiom; 12 Editor-squad templates; the 4 cross-genre templates; the 89 glyphs, 27 enchantments, 19 summons and 14 prayers of doc 06; the 41 materials, 9 stations, 120 recipes and 68 parts of doc 07; the ingredients, liquids and gases of doc 08; 116 affixes, 51 perks, 16 genre traits, 26 Legendary perks and 10 Palimpsestic uniques of doc 11; the 91 Bindery nodes, 58 achievement records and 32 Margin slots of doc 14; the Margin hub and its residents; the contextual tips.

### 5.6 Localization volume

en-US keys per core genre:

| Family | Keys |
|---|---|
| Weapon skins, name and flavor | 56 |
| Armor skins | 20 |
| Ward generator noun, focus, tome | 3 |
| Manufacturers, name and tagline | 4 |
| Genre trait, name and text | 2 |
| Exotics, name, intrinsic and flavor (2 per genre on average) | 6 |
| Roster templates, name, description and Bestiary entry | 36 |
| Mini-bosses, name, title and 4 attack names | 12 |
| Factions | 4 |
| World Laws, name and text | 4 |
| Hazards and events | 12 |
| Damage-type skins | 15 |
| Lucre and Aether skins | 2 |
| Patron Author | 4 |
| Genre name, title card and door tagline | 3 |
| Echoes, name, epithet, biography, trait name, trait text and trial text | 12 |
| Going Native forms | 6 |
| **Total** | **201** |

Global record families, 2 keys each (name and text): 58 achievements, 91 Bindery nodes, 116 affixes, 51 perks, 26 Legendary perks, 10 uniques, 89 glyphs, 27 enchantments, 170 talents, 72 traits, 33 augments, 61 statuses and 13 Callings, 817 records and 1,634 keys. UI strings (doc 17) and narration grammars (doc 18 §8.7) are counted by `pal catalog`. Early Access ships 7 × 201 + 1,634 = 3,041 genre and record keys before UI and grammars, in 15 locales. Text freezes: Early Access 2027-07-09; each update 4 weeks before its release.

## 6. Engineering process for coding agents

### 6.1 Picking up a work package

1. **Check readiness.** Every WP in *Depends on* is merged on `main`.
2. **Claim.** Open a draft pull request titled `WP-NNN: <title>` from branch `wp/NNN-<slug>`. One claim per WP; a claim with no commit for 2 working days is released.
3. **Read first.** Read completely: every Canon section the WP cites or its owner chapter imports for the cited sections; the cited chapter sections; that chapter's Interface assumptions and Canon gaps rows that touch them; the DECISIONS rows named in the WP; `palimpsest/AGENTS.md` and the crate's own AGENTS.md; and every `audit/canon_gaps_triage.md` row for those sections.
4. **Write the tests first.** Turn every acceptance test in the WP row into a named, failing test before writing the implementation: `canon_tNN_<name>` for Canon tests, `docNN_tNN_<name>` for chapter tests, `pillar_<name>_tN` for pillar tests, and a scenario file `tests/moments/sm_NN.json` for SM tests. Every expected number comes from the spec text; a doubtful value is recomputed with exact integer Python, never with floats and never by running the code under test.
5. **Implement**, then add a `proptest` property for every invariant the spec states (monotonicity, bounds, round trips, no overflow over declared ranges, weight sums of 10000).
6. **Pass the gates** of §6.3, update the crate's AGENTS.md, and mark the pull request ready.

### 6.2 Determinism and numeric rules

- No `f32` or `f64` outside `palimpsest_bridge::conv` (doc 02 §7.5) and test code; no float literal in math, data, econ, gen, ai or sim.
- No `HashMap`, `HashSet`, `RefCell`, `Rc`, atomics, `sort_unstable*`, `select_nth_unstable*`, `usize` in serialized state, or randomness inside `#[cfg(debug_assertions)]` (doc 02 §6.1, Forbidden constructs).
- Iterate in ascending `EntityId` or `uid` wherever state changes, events are emitted, entities are created or numbers are drawn.
- Action-caused draws use `keyed_rng(stream, folio, actor, tick, seq)` with a `seq` from the owner's published allocation (Canon §3.3, Keyed draws); bulk host processes use their sequential stream (Canon §3.2, Sequential streams); presentation uses only `nar_rng` and `pres_hash` (Canon §3.8, Presentation generators).
- Every `Bp` chain is one `mul_bp` per factor in the listed order, clamping last unless the formula places it elsewhere (Canon §1.3, Integer primitives (normative)).
- Overflow panics in every profile. Use `i128` intermediates where the canon says so, and treat every normative `x << k` as a checked multiplication.
- Inside the simulation, `unwrap()` is denied and a broken invariant panics through `expect("invariant: …")`.
- Any commit that can change a state hash for any input increments `SIM_VERSION` and re-blesses the golden replays in the same pull request.

### 6.3 Gates

Every pull request passes, in CI and locally before review: `cargo fmt --all -- --check`; `cargo clippy --workspace --all-targets --all-features -- -D warnings`; `cargo nextest run --workspace`; `cargo xtask check-deps`, `float-scan`, `serde-scan`, `lint-size` and `spec-refs`; `pal schema export` with no diff; `pal validate`; `pal golden verify` on the fast set; SyncTest depth 8; the five-target determinism matrix; `python3 tools/gen_trig.py --check` and `python3 tools/gen_curves.py --check`; and `python3 docs/palimpsest/tools/spec_lint.py` when a document under `docs/palimpsest/` changes. Nightly adds the full golden set, generator sweeps, 200 bot runs, fuzzing and the `deck-01` performance gates; weekly adds mutation testing and coverage (doc 02 §17.1).

### 6.4 Branch, review and merge rules

- One branch per WP, `wp/NNN-<slug>`, based on `main`. At most 3 pull requests per WP, each at most 1,500 changed lines excluding generated files.
- Squash merge. The commit subject starts with `WP-NNN:`; the body lists the acceptance tests added.
- A pull request merges when CI is green and one reviewing agent other than the author approves. The product owner also approves any change to the separation wall of doc 15 §1.2, `juris.` flags, store SKUs or prices, and any edit to a binding document under `docs/palimpsest/`.
- `main` is always releasable to the `dev` channel. Nothing is force-pushed to `main`.

### 6.5 Definition of done

A WP is done when: every acceptance test in its row is named and green on all CI targets; every public item in math, data, econ, gen, ai and sim carries its `SPEC:` reference; no new `GAP:` comment lacks an intake row (§6.6); the touched systems keep their `perf.` budgets in the next nightly; its content validates and its goldens are re-blessed where needed; its deliverables exist on `main`; and the crate's AGENTS.md describes them.

### 6.6 Spec defects: the canon gap triage loop

1. **Detect.** An agent that finds the spec silent, ambiguous or self-contradictory does not invent a rule. It picks the narrowest default that lets the WP proceed, writes it in a `GAP:` paragraph of the doc comment, and appends an intake row to `docs/palimpsest/audit/slice_spec_issues.md`: WP, spec section, the question, the default used, and whether the default changes a formula.
2. **Triage.** The canon gap triager answers each intake row within 2 working days in `audit/canon_gaps_triage.md` with exactly one decision: canon amendment, chapter owns it, already in canon, or reject. A design call goes to the product owner and lands in `DECISIONS.md`.
3. **Amend.** A canon amendment raises the canon version (canon-1.12 next) and adds a Canon §21 test for every new number. Chapter fixers then apply the change and close their gap text.
4. **Close.** The implementing agent replaces its default with the ruling, deletes the `GAP:` paragraph and adds the new test.
5. **Blocking.** A WP whose default changes a formula, an RNG draw count or the serialization stops at the default and stays claimed until the ruling; every other WP merges with its default.
6. **Freeze.** After M6 the canon changes only for a P0. The canon version in force at each release is recorded in its release notes.

## 7. Risk register

| ID | Risk | Likelihood | Impact | Early-warning metric | Mitigation | Fallback that preserves the pillars |
|---|---|---|---|---|---|---|
| R-01 | Cross-platform desync from a float, iteration order or width bug | medium | high: breaks `pillar.living_page.t2`, co-op and ranked boards | any determinism-matrix mismatch or SyncTest failure on `main`; any desync in nightly co-op bot runs | `xtask` scans, the five-target matrix on every pull request, `pal desync-diff`, the `SIM_VERSION` rule | merges stop until the fix lands; ranked verification runs on the x86_64 verifier only; shadows (`flag.net.shadow`) turn off while the host simulation stays authoritative and deterministic |
| R-02 | Simulation tick over budget on Steam Deck | medium | high: PAR-13 blocks Early Access | nightly `perf.deck.sim.tick_mean` above 4.5 ms or `perf.deck.sim.tick_p99` above 7.2 ms | per-group budgets from M1, unit-cost benchmarks, the perf harness WP-021 | a Deck hosts at most 2 players while PC hosts 4; render presets drop to `deck_battery`; no simulation grid or cap is cut |
| R-03 | Lattice infeasible with 6 genres | certain until G1 is ruled | high: runs assert at a Volume's fifth Folio | doc 12 test 3 run over the Early Access genre set | the cascade of §2.3 (a) proposed for canon-1.12 before WP-041 merges | WP-041 waits for the ruling; no Early Access build ships without a canon rule; the product owner picks between the cascade and another amendment |
| R-04 | Content volume for 41 new templates, 12 mini-bosses and 8 genre asset sets | high | high: DECISIONS 69 sets a floor | the generated catalog below 50% of §5.4's Early Access counts on 2027-06-11 | archetype × skin reuse, AI-assisted production with weekly curation, genre packs start inside M3 with slack | Early Access moves by whole weeks; no genre ships below 12 templates and 2 mini-bosses; secondary tilesets drop to doc 12's floor of 40 prototypes first |
| R-05 | Bevy 0.20 or a coupled crate fails a client test | medium | medium | a client acceptance test red for 3 nightlies | doc 02 §2.4 (Ecosystem rule): thin in-house layers, exact pins | replace the crate in-house (cosmetic physics, particles); stay on 0.20 through 1.0 |
| R-06 | EOS integration has no pinned Rust binding and needs FFI | medium | medium | WP-069 not merged by 2027-06-11 | in-house binding behind feature `eos` with `// SAFETY:` review | Steam Early Access ships on date; the Epic Games Store build and crossplay ship with Update 1 |
| R-07 | New loot-box or consumer law (the EU Digital Fairness Act proposal expected in November 2026) | medium | high | a regime that restricts earned randomness or battle passes | separation wall, `juris.` flags, earned-only currencies | per-jurisdiction flags hide account Reliquaries or the Inkwell in that region; everything stays earned-only, so `pillar.fair_page` holds |
| R-08 | Balance bands fail (pacing, party parity, earn rates) | high | medium | nightly bot bands outside doc 02 §16 item 7; `kpi.coop.party_parity` above 1000 Bp in bot runs | the balance harness from M3; tuning by data only | data-only tuning patches after launch; no formula changes; Canon §17.2 stays until live KPIs report (doc 01 §11 Q-20) |
| R-09 | Spec churn while chapters are fixed concurrently | high | medium | more than 20 open intake rows, or any row older than 2 working days; any `spec_lint` finding on `main` | the triage loop of §6.6, canon freeze after M6 | WPs proceed on recorded defaults; release notes record the canon version |
| R-10 | Translation quality in 15 locales | medium | medium | linguistic QA defects per 1,000 keys above 5; any missing key after a freeze | text freezes, glossary from doc 18 §9.11, `en-XA` CI | a locale ships with its open defects listed and fixed in the next patch; no locale is dropped (DECISIONS 40) |
| R-11 | Review and curation bandwidth of one product owner | high | medium | pull-request queue age p50 above 2 working days; curation backlog above 40 assets | agent cross-review, weekly curation batches, question rounds with defaults | agents continue non-blocked WPs; critical-path WPs get first review |
| R-12 | Platform review: Deck Verified, ratings, Epic store review | medium | medium | any submission not started by 2027-07-23 | the rating dossiers of WP-075 at M5; IARC ratings | launch with every Verified checklist item green internally while Valve's review is pending |
| R-13 | Invasion griefing or cheating | medium | medium | reports per 1,000 invasions above 10; invader win rate above 7000 Bp | invader isolation and sealing (doc 16 §1.1), opt-in only | `svc.status` turns invasion beacons off globally; co-op and solo play are untouched |
| R-14 | Asset provenance (public-domain figures, AI-assisted assets) | low | high | any license-manifest lint failure | the manifest lint of WP-081, doc 18 §10.3 | the asset is replaced before release; no asset without provenance ships |
| R-15 | Online services overload at launch | medium | medium | a load test at 3 times the projected peak fails | horizontal scaling of `svc.econ` and `svc.lobby`, CDN for odds pages | offline-complete play; pulls queue as Sealed Drafts; invasions pause |
| R-16 | Scope added by new owner rows (items 94 to 104 arrived on 2026-10-11) | high | medium | any new DECISIONS row after M5 that touches Early Access | each new row gets a tier in this chapter within 2 working days | a new feature defaults to the first update after launch |

## 8. Dependency diagram

The diagram shows every WP grouped by milestone. Edges are the transitive reduction of the *Depends on* column ({{N_EDGES}} edges); thick borders mark the critical path of §4.4.

{{MERMAID}}

## 9. Questions for the product owner

Each default below is in force until answered; answers land in `DECISIONS.md`.

| # | Question | Default applied |
|---|---|---|
| RQ-01 | The 25-Folio run cannot fill 24 non-final Folios with 6 genres under `GENRE_REPEAT_RULE` (each genre would appear 4 times), and the procedural ending needs an Ur-Text victory. Is Abridged the Early Access full-story run? | yes: Abridged, Novella, Daily and Weekly at Early Access; Standard and Endless at 1.0 |
| RQ-02 | Adopt the lattice relaxation cascade of §2.3 (a) for genre sets below 12? | yes, proposed for canon-1.12 |
| RQ-03 | Early Access on Thursday 2027-09-09 and 1.0 on Thursday 2028-04-06? | yes |
| RQ-04 | All 12 Origins and 24 Echoes at Early Access, with kit skins in all 12 genres? | yes |
| RQ-05 | On-device Scribe at Early Access or 1.0? | 1.0; template text at Early Access |
| RQ-06 | Asynchronous traces (DECISIONS 96) at Early Access or later? | Update 1 |
| RQ-07 | Epic Games Store at Early Access? | yes, with crossplay; fallback Update 1 (R-06) |
| RQ-08 | Post-launch genre order? | Fairy Tale and Mythic Antiquity, then Cosmic Horror and Hardboiled Noir, then Space Opera and Post-Apocalypse |
| RQ-09 | Kaiju and Mecha without vehicles: mecha as powered armor and Going Native forms? | yes (DECISIONS 55 binds) |
| RQ-10 | Planning capacity of 8 concurrent agent lanes? | yes |
| RQ-11 | Folio Expansion price | USD 9.99 fixed (DECISIONS 122) |
| RQ-12 | Do the Ur-Text and the Unwritten need 2 mini-bosses each? | no; they keep doc 13 §9.3's champion |
| RQ-13 | A public demo before Early Access? | no; the Steam Playtest app on the `playtest` channel serves pre-launch players, and the WebGPU demo of doc 02 §14 is dropped under DECISIONS 98 |
| RQ-14 | Streaming votes (DECISIONS 104) at Early Access or later? | Update 2 |
| RQ-15 | Second Edition (DECISIONS 100–103) at Early Access or 1.0? | 1.0: it needs the Ur-Text clear, fused pairs from 12 genres and 6 new boss fights |
| RQ-16 | Time of day and weather (DECISIONS 99) at Early Access? | yes: they feed light and stealth, which ship at Early Access |

## Exports

This chapter defines no content ID and owns no Canon §19 namespace. It exports document-local labels: work packages WP-001 to WP-128, milestones M0 to M13, risks R-01 to R-16, questions RQ-01 to RQ-16 and canon gaps G1 to G16. Non-ID interfaces: the `palimpsest_sim` module paths of §4.1, the Early Access genre-set rules of §2.3, the content source-of-truth rule of §5.1, the branch and claim conventions of §6, and the requested `pal` subcommands `extract`, `spec-sync` and `catalog`.

## Canon gaps

- **G1. Lattice feasibility below 8 core genres.** Canon §17.1 (Run constants) requires exactly 3 lattice candidates under `GENRE_REPEAT_RULE`, and doc 12 §5.4 asserts at least 3 eligible genres. With the 6 Early Access genres a Volume's fifth Folio leaves 2 and Abridged Folios 12 and 14 leave 1. Proposal: canon-1.12 adds the cascade of §2.3 (a) to `LATTICE_CANDIDATES`, and doc 12 §5.5 implements it; it is inert with 12 core genres.
- **G2. Folio 1 for an unshipped Origin genre.** `FOLIO_1_GENRE` names the host's Origin genre, which can lie outside the content set. Proposal: the mapping of §2.3 (b), in Canon §17.1 and doc 12 §5.6 (Folio 1).
- **G3. Shipped genre set.** The canon has no notion of a build whose content set holds fewer than 12 core genres. Proposal: as doc 12 §25 does for expansions, a core genre outside the content set is not a lattice node.
- **G4. Pillar tests at Early Access.** Doc 01 §2 words five tests for Standard runs and 12 genres. Proposal: doc 01 adopts the readings of §2.3 (e) for the Early Access gate.
- **G5. Price.** Doc 01 §9.3 and doc 15 §1.1 list USD 24.99 at Early Access and 29.99 at 1.0; DECISIONS 42 sets 19.99 for both. This chapter uses 19.99; both chapters change.
- **G6. Early Access breadth.** Doc 01 §4.2 (PAR-12) and §11 (Q-06) put all 16 genres at Early Access; DECISIONS 41 sets 6. PAR-12's lower bound of 8 genres holds from Update 1 on.
- **G7. `T0`.** Doc 15 §3.2.3 uses `T0 = 1792886400` (2026-10-25) in its examples; the Early Access date makes it 1820448000 (§3.3).
- **G8. Erasure onset.** Doc 01 §6 (Core loops at five time scales) and its test 15 grant stack 1 at clock 25,200; Canon §17.1 (Run constants) grants it at 43,200 since canon-1.6 (DECISIONS 26), and doc 14 §3.8 (Erasure pressure timeline) now follows the canon. WPs implement the canon.
- **G9. Milestone labels.** Doc 02 §18 item 8 uses M0 to M15 for its task order and calls the slice "M0 to M4"; the slice covers the math step and parts of the data and save steps, and none of the grids. Proposal: doc 02 replaces item 8 with a pointer to §3.5 of this chapter.
- **G10. SPEC reference form.** Doc 02 §18 item 2 requires `SPEC: 00 §x.y`; the slice uses `SPEC: canon §x.y`. Ruling applied by WP-003: `xtask spec-refs` accepts both, `canon` meaning `00`, and new code uses the doc 02 form.
- **G11. Roster size.** Doc 13 §8 holds 6 to 8 templates and §9.3 one mini-boss per genre, against DECISIONS 69. The missing templates are authored as content records (§5.1 step 3) under doc 13's column rules; doc 13 gap 1 records it.
- **G12. EOS timing.** Doc 16 §3.6 (Crossplay identity) lists `epic` as a later platform; DECISIONS 73 keeps the EOS build, and this chapter ships it at Early Access.
- **G13. Early Access asset set.** Doc 18 §10.1 produces the secret genres' families by 1.0, but the final Folio and Unwritten need the Ur-Text and Unwritten families at Early Access.
- **G14. Lost Chapter Scripts.** Doc 15's launch catalog includes Scripts for genres that ship after Early Access; §2.3 (g) keeps them out of banner pools until their genre ships.
- **G15. Newest owner rows.** DECISIONS 94 to 104 (ink gore, movement kit, asynchronous traces, creator presets, PC only, time of day and weather, Second Edition, streaming votes) are not yet in their chapters, and items 99 and 101 need Canon §5 and §12 changes. This chapter assigns them: 94 to WP-050, 95 to WP-014 and WP-024, 96 to WP-103, 97 to WP-054 and WP-083, 98 to WP-060, WP-061 and WP-101, 99 to WP-043, 100 to 103 to WP-120 to WP-122, 104 to WP-108.
- **G16. Platforms.** DECISIONS 98 makes the game PC only through 1.0: Windows on Steam and Epic, the Steam Deck through Proton. Doc 01 §9.2 (Platforms) and doc 02 §2.1 (Toolchain) and §14 (Platforms) still list native Linux and macOS builds, a WebGPU demo, consoles and a native Linux build for the Deck checklist. Linux remains a CI and server target, because `svc.verify` re-simulates Windows clients' replays on Linux.

## Interface assumptions

| Doc | Assumed to provide |
|---|---|
| 01 | the Early Access readings of §2.3 (e) in §2; prices and PAR-12 updated to DECISIONS 41 and 42; §9.2 platforms updated to DECISIONS 98; §6 and test 15 at clock 43,200 |
| 02 | §2.1 and §14 reduced to the DECISIONS 98 platforms, with Linux kept for CI, servers and the verifier; §18 item 8 replaced by a pointer to §3.5; `pal extract`, `pal spec-sync` and `pal catalog` added to the `pal` subcommands of §3.4; a pinned EOS binding behind feature `eos` of `palimpsest_platform`; meta-progression rules in `palimpsest_platform::meta`, headless-testable; the module paths of §4.1 |
| 05 | the grappling tool archetype of DECISIONS 95 |
| 12 | the cascade (G1) in §5.4 and §5.5; the Folio-1 mapping (G2) in §5.6; secret exits gated by the content set; time-of-day and weather rolls (DECISIONS 99); fused Second Edition Folios (DECISIONS 101) |
| 13 | the roster templates and second mini-boss per genre authored as content (G11) |
| 14 | the Early Access mode set of §2.2; achievement registration by tier (§2.3 (f)) |
| 15 | `T0 = 1820448000` in §3.2.3 with every dependent example recomputed: the p001 end date 2027-10-07, the banner `start_utc` 1820448000 and `end_utc` 1822867200, the receipt `iat` one week after `T0` (1821052800), the Codex `release_utc`; base price USD 19.99; banner-pool gating of §2.3 (g) |
| 16 | EOS at Early Access; the asynchronous-traces service of DECISIONS 96 for Update 1 and the streaming-vote overlay of DECISIONS 104 for Update 2 |
| 17 | UI skins for the shipped genre set |
| 18 | the Ur-Text and Unwritten families in the Early Access batch; ink gore (DECISIONS 94) in the VFX language; creator presets (DECISIONS 97); time-of-day and weather presentation (DECISIONS 99) |
| 19 | a schema for every record type, the validation stages that `tools/validate_content.py` implements, and the list of rules the Rust loader must match |
| 20 | generated by `pal catalog` with per-genre counts against §5.3 and §5.4 |

## Acceptance tests

1. **Order.** In the WP tables every dependency names an existing WP with a smaller number ({{N_ALL}} rows).
2. **Ownership coverage.** Chapters 01 to 20 each appear in the owner column of at least one WP.
3. **Test coverage.** The union of the *Acceptance tests* columns holds Canon §21 tests 1–47 and every acceptance test on disk in chapters 01 to 18 except the retired tests of §4.6, recomputed from the files.
4. **Pillars.** Each of the 18 pillar tests appears in exactly one Early Access WP, and SM-01 to SM-11 in WP-048.
5. **Critical path.** The longest size-weighted path ending at WP-102 is the path of §4.4, {{CRIT_LEN}} agent-days.
6. **Schedule.** With 8 lanes, every WP of M0 to M7 finishes on or before its milestone's target (§4.5).
7. **Calendar.** 2027-09-09 is a Thursday; `T0 = 1820448000` is 2027-09-09T00:00:00Z; the M0 to M6 targets are Fridays and every release date from M7 on is a Thursday; Codex 001 to 004 fall on 2027-09-09, 2027-12-02, 2028-02-24 and 2028-05-18.
8. **Diagram.** The Mermaid node set equals the {{N_ALL}} WPs, and its edge set equals the transitive reduction of the *Depends on* column ({{N_EDGES}} edges).
9. **Cascade.** For a Novella run choosing Folio 5 after Mythic Fantasy, Gothic Horror, Cyberpunk and Weird West over the Early Access set, `e0` = {Wuxia, Steampunk} and the cascade returns {Mythic Fantasy, Gothic Horror, Wuxia, Steampunk}; for every legal history of Novella, Abridged and Daily over that set, the cascade returns at least 3 genres and `rng.lattice` makes exactly 3 `range` calls.
10. **Folio-1 mapping.** For each core genre outside the Early Access set the mapped genre is the maximum of its doc 12 §5.2 row over the Early Access genres, ties to lattice order: Cosmic Horror → Gothic Horror, Space Opera → Cyberpunk, Post-Apocalypse → Weird West, Hardboiled Noir → Cyberpunk, Fairy Tale → Mythic Fantasy, Mythic Antiquity → Mythic Fantasy.
11. **Totals.** 6 × 12 + 12 + 12 = 96 roster templates at Early Access and 16 × 12 = 192 at 1.0; 12 and 28 mini-bosses; 201 en-US keys per core genre; 817 global records and 1,634 keys; 46 + 12 = 58 achievements, equal to doc 14 test 17's count.
12. **Lint.** `python3 tools/spec_lint.py --chapter 21` reports no missing section, undefined ID, foreign definition, retired ID, JSON issue or hedge.

## Invariants annex

```
Σ 21: tiers ⊗ milestones ⊗ work packages ⊗ content plan ⊗ process ⊗ risks ⊗ graph
W! ∀ WP: deps ⊂ {WP with smaller number}                  N! merge with a listed test red or unnamed
W! EA ⇔ every M7 criterion passes on one build            N! criterion waived to keep a date
W! EA lattice genres = {MF, GH, CP, WW, WX, SP}            W! EA depth-25 Folio = genre.ur_text
W! price(EA) = price(1.0) = USD 19.99                      N! premium currency | paid randomness in any tier
W! ∀ shipped core genre: roster ≥ 12 ∧ mini-bosses = 2     N! mounts | vehicles | item durability in any tier
W! lattice cascade inert with 12 core genres               W! rng.lattice draws = 3 range calls per exit
W! chapter table = content record (spec-sync)             N! hand edit of doc 20
W! spec defect → GAP: default + intake row + triage        N! silent rule invented in code
W! state-hash change ⇒ SIM_VERSION bump ∧ goldens re-blessed
W! T0 = 1820448000                                         W! 15 locales, 0 missing keys at each release
W! client platform := Windows (Steam, Epic) ∪ Steam Deck via Proton   W! Linux ∈ CI ∧ verifier targets
W! later release: pillar.one_ruleset.t3 re-run when genres change
```
