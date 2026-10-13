# Work-package data for spec/21_IMPLEMENTATION_ROADMAP.md.
# Each WP: id, title, ms (milestone), tier, owner [(doc, [sections])], crate, deps, deliv, tests, size.
# tests: dict with keys 'C' (canon §21 numbers), 'NN' (chapter test numbers), 'P' (pillar test suffixes), 'SM' (numbers), 'X' (free text checks)

R = lambda a, b: list(range(a, b + 1))

WPS = []
def wp(id, title, ms, tier, owner, crate, deps, deliv, tests, size):
    WPS.append(dict(id=id, title=title, ms=ms, tier=tier, owner=owner, crate=crate, deps=deps,
                    deliv=deliv, tests=tests, size=size))

# ---------------- M0 Foundations ----------------
wp(1, "Canon-1.11 catch-up in the math crate", "M0", "EA",
   [("00", ["1.9", "3.1", "3.2", "3.3", "3.5", "3.8"])], "palimpsest_math::rng, ::cordic", [],
   "`derive` sub-streams, `seq` range check, object-keyed draws, offer generator, `nar_rng` and `pres_hash`; named tests `canon_t45`..`canon_t47`; every slice test stays green",
   {"C": R(1, 12) + [21, 43, 44, 45, 46, 47], "02": R(1, 11) + [36]}, 2)
wp(2, "Workspace skeleton for the 20 crates", "M0", "EA",
   [("02", ["3.1", "3.2", "3.3", "18"])], "workspace; all `crates/*`", [],
   "empty crates with allow lists, `deny.toml`, per-crate AGENTS.md, the crate records file under `content/02/crate/`",
   {"02": [21]}, 3)
wp(3, "xtask lint suite", "M0", "EA",
   [("02", ["6.1", "6.2", "18"])], "xtask", [2],
   "`check-deps`, `float-scan`, `serde-scan`, `lint-size`, `spec-refs` (accepts `SPEC: 00 §` and `SPEC: canon §`), grep rules of §6.1",
   {"C": [33], "02": [21, 22, 44]}, 4)
wp(4, "CI pipeline: pull request, nightly, weekly", "M0", "EA",
   [("02", ["6.4", "16", "17.1"])], "`.github/workflows/`", [3],
   "PR job (fmt, clippy, deny, xtask, schema check, validate, nextest, golden fast set, SyncTest, 5-target matrix), nightly and weekly jobs, `deck-01` runner, spec lint job for `docs/palimpsest`",
   {"X": ["CI dry run on a no-op PR completes in ≤ 25 min"]}, 4)
wp(5, "Doc 19 schema set", "M0", "EA",
   [("02", ["11.1"]), ("19", [])], "palimpsest_data::schema; palimpsest_tools (`pal schema export`)", [2],
   "Rust record types for every content namespace, JSON Schema 2020-12 export with `x-patchable`, committed `schemas/`, CI diff check",
   {"02": [19]}, 8)
wp(6, "Content loader and validator `pal validate`", "M0", "EA",
   [("02", ["11.1", "11.2", "11.3"]), ("00", ["1.1", "1.6", "2.4", "17.7"]), ("19", [])],
   "palimpsest_data::loader; palimpsest_tools::validate", [5],
   "token pre-pass, schema pass, semantic pass (grammar, references, §19 ownership, weight sums, aliases, `id32` collisions, retired IDs, genre-set lint), `ContentDb`, `content_hash` and `presentation_hash`",
   {"C": [29, 34], "02": [17, 19, 27], "18": [53, 59, 60, 61, 62, 63, 64, 65, 66]}, 8)
wp(7, "Flag, perf and crate records; `pal perf report`", "M0", "EA",
   [("02", ["15", "17.3"]), ("15", ["8.2"])], "palimpsest_data; palimpsest_tools", [6],
   "flag and perf record loaders, the jurisdiction flag hook, budget arithmetic check",
   {"X": ["02 test 33, unit clause: group means give 4.33 ms and worsts 14.50 ms"]}, 3)
wp(8, "Generated tables in CI", "M0", "EA",
   [("02", ["11.5"])], "tools/gen_trig.py; tools/gen_curves.py", [4],
   "`gen_curves.py` with `curves_src.json`, both generators checked byte-identical in CI",
   {"02": [46]}, 2)

# ---------------- M1 Headless world core ----------------
wp(9, "Entity store and scheduler", "M1", "EA",
   [("02", ["5.1", "5.2", "5.3"]), ("03", ["2", "3"])], "palimpsest_sim::store, ::schedule", [1, 6],
   "sorted-column store keyed by `EntityId`, deferred commands, phases, `entity_due`, `OrderedMap`",
   {"02": [30, 31], "03": [1, 2, 3]}, 6)
wp(10, "Sim API and canonical serialization", "M1", "EA",
   [("02", ["5.4", "5.5", "6.5"]), ("03", ["16"]), ("00", ["2.2", "2.4", "2.5"])], "palimpsest_sim::api, ::serial", [9],
   "`TickInputs` slots 0..=4, commands and intents, `StepOutput`, 13 sections, `state_hash` boundaries, snapshot and restore; the slice `Sim` skeleton is retired",
   {"C": [30, 32], "03": [32, 39]}, 8)
wp(11, "Save container, golden replays, SyncTest, desync dump", "M1", "EA",
   [("02", ["6.6", "6.7", "12.1", "12.2"])], "palimpsest_tools (`pal golden`, `pal desync-diff`); palimpsest_sim::serial", [10, 4],
   "`PALS` container, golden fast and full sets, `HASH_EVERY_TICK` bisection, SyncTest depth 8, dump files",
   {"02": [20, 24, 25, 26]}, 7)
wp(12, "Five-target determinism matrix", "M1", "EA",
   [("02", ["6.4"])], "CI", [11],
   "hashes.json jobs on x86_64 Linux, aarch64 Linux, Windows, macOS and wasm32-wasip1",
   {"02": [23], "P": ["living_page.t2"]}, 3)
wp(13, "Voxel world storage and block registry", "M1", "EA",
   [("03", ["4"]), ("00", ["4.3", "4.4"]), ("07", ["10.1"])], "palimpsest_sim::world", [9, 6],
   "`Chunk`, `BlockState`, palette compression, registry with genre skins, shapes and micro-voxels, fixed terrain and protect masks",
   {"03": [4, 5, 36]}, 7)
wp(14, "Character controller and collision", "M1", "EA",
   [("03", ["5"]), ("00", ["4.1", "4.2"])], "palimpsest_sim::controller (absorbs ::movement)", [13],
   "wish velocity, integration, capsule against voxels, step-up, jumps, mantle up to 2,000 mm and sprint-slide (DECISIONS 95), swim, climb, dodge, knockback, push, falls, raycast, resting bodies",
   {"C": [24, 27], "03": R(6, 14) + [16, 38, 47, 50]}, 10)
wp(15, "Hitboxes, projectile sweep, portals, sampling", "M1", "EA",
   [("03", ["3.6", "5.16", "5.19", "5.20", "5.21"])], "palimpsest_sim::controller", [14],
   "pose-table hitbox capsules, broadphase, projectile sweep with the lag-compensation hook, relink portals, environment sampling",
   {"03": [15, 40, 49]}, 5)
wp(16, "Liquids and surfaces", "M1", "EA",
   [("03", ["6"]), ("08", ["2"])], "palimpsest_sim::grids::liquid", [13],
   "liquid automaton, capacity, activation and budget, sources and sinks, reaction hook, surface triggers and contact",
   {"03": [17, 18, 19, 30, 31, 37, 46], "08": [1, 2, 3]}, 8)
wp(17, "Gases, fire and temperature", "M1", "EA",
   [("03", ["7", "8", "9"])], "palimpsest_sim::grids::{gas, fire, thermal}", [16],
   "gas run, emission and ignition, fire cells and spread on `rng.fire`, thermal diffusion and effects",
   {"03": [20, 21, 22, 42]}, 8)
wp(18, "Light and sound grids", "M1", "EA",
   [("03", ["10", "11"]), ("00", ["2.3", "12.3"])], "palimpsest_sim::grids::{light, sound}", [13],
   "two light channels, propagation queues, Folio bake, light overrides, sound emission and propagation with fallback",
   {"03": [23, 24, 25, 48]}, 7)
wp(19, "Explosions and block edits", "M1", "EA",
   [("03", ["12", "13"])], "palimpsest_sim::explode, ::edit", [14, 17],
   "explosion queue and rays on `rng.physics`, incendiary, entity damage records, `set_block`, breaking, placing, doors, block-update queue, falling blocks, random ticks, Erasure of placeables",
   {"03": [26, 27, 28, 29, 41, 43, 44]}, 6)
wp(20, "Event bus, activation and actor cap", "M1", "EA",
   [("03", ["14", "15"]), ("00", ["18"])], "palimpsest_sim::events", [10],
   "event bus with payload schemas, active chunks, sleeping entities, actor cap",
   {"03": [33, 34, 45, 51, 52]}, 4)
wp(21, "Simulation performance harness", "M1", "EA",
   [("02", ["15"]), ("03", ["17"])], "palimpsest_bots (scenarios); `benches/`", [15, 18, 19, 20, 7],
   "criterion unit costs, `perf_saturation_4p` and `perf_spike_4p` for the world groups, nightly gate on `deck-01`",
   {"03": [35]}, 5)

# ---------------- M2 Headless rules ----------------
wp(22, "Damage pipeline in the tick", "M2", "EA",
   [("05", ["1", "2", "10", "13", "14"]), ("00", ["10", "11"])], "palimpsest_sim::combat (absorbs ::damage, ::status)", [15, 20],
   "hit records, attack factors before step 1, same-tick order, keyed draw allocation, status power, threat, friendly fire, downed hooks",
   {"C": [17, 18, 19, 25, 26, 28, 38, 41, 42], "05": R(3, 11) + [16, 22, 33, 34, 40, 42, 47, 53, 56, 58]}, 8)
wp(23, "Melee", "M2", "EA",
   [("05", ["3", "4", "12"])], "palimpsest_sim::combat::melee", [22],
   "melee archetypes, moveset and state machine, parry and riposte, guard break, blocking, poise and stagger",
   {"05": [1, 17, 18, 29, 30, 36, 37, 49, 51, 61]}, 8)
wp(24, "Ranged, ballistics, ammunition and throwing", "M2", "EA",
   [("05", ["5", "6", "7", "8", "9"])], "palimpsest_sim::combat::ranged", [22, 19],
   "ranged archetypes, firing state machine, spread, bloom, recoil, ADS, sway, lean, projectiles, penetration, ricochet, ammunition, grenades, explosion resolution, the genre-skinned grappling tool (DECISIONS 95)",
   {"05": [2, 12, 13, 14, 15, 19, 20, 21, 23, 24, 25, 26, 27, 38, 43, 50, 52, 55]}, 10)
wp(25, "Armor, wards, counters, jams, severing hits", "M2", "EA",
   [("05", ["11", "15", "16", "19"])], "palimpsest_sim::combat::armor", [23, 24],
   "armor and armor poise, ward generators, champion counters and elite aspects, Contradiction jams, severing hits, taunt, grapple, law-bound techniques",
   {"05": [28, 31, 32, 35, 39, 44, 45, 46, 54, 57, 59, 60, 62]}, 7)
wp(26, "Character creation, attributes, Origins, kin, Callings", "M2", "EA",
   [("04", ["1", "2", "3", "4", "13", "15"]), ("00", ["6", "16"])], "palimpsest_sim::character (absorbs ::stats)", [10, 6],
   "point-buy, Origins, kin, Calling presets, Echo path, checks, Echo bodies, the ID register",
   {"C": [13, 16, 35], "04": [1, 2, 3, 4, 37, 51, 52, 53]}, 5)
wp(27, "Skills, levels, talents, joiners", "M2", "EA",
   [("04", ["5", "6", "7"]), ("00", ["7", "8"])], "palimpsest_sim::character (absorbs ::progression)", [26, 22],
   "use-based growth, XP events, character XP with ghost sharing (DECISIONS 81) and HP-normalized skill XP (DECISIONS 85), talents, respec, mid-run joiners",
   {"C": [22, 23], "04": R(5, 15) + [39, 40, 41, 42, 48]}, 6)
wp(28, "Body, survival and player-controlled statuses", "M2", "EA",
   [("04", ["2.8", "8", "9", "10", "11", "12"])], "palimpsest_sim::body", [25, 27, 17],
   "part pools, wounds, severing with the crippled-pool rule (DECISIONS 84), augments, prosthetics, afflictions, needs, Death's Door, madness, Going Native",
   {"04": R(16, 36) + [38, 43, 44, 45, 46, 47, 49, 50, 55]}, 9)
wp(29, "Magic core: aether, casting, grammar, glyphs", "M2", "EA",
   [("06", ["2", "3", "4", "5", "6", "7", "11"])], "palimpsest_sim::magic", [27],
   "genre-native aether, cast state machine, glyph grammar and compiler, glyph catalog, cost and potency, schools, spell Contradiction",
   {"C": [36], "06": R(1, 15) + [30, 32, 33, 34, 35, 36, 37, 51, 52]}, 10)
wp(30, "Magic extended: Glyph Library, enchanting, summons, prayer", "M2", "EA",
   [("06", ["8", "9", "10", "12", "13", "14", "15", "16", "17", "18", "19"])], "palimpsest_sim::magic", [29, 19],
   "Grimoire and the account Glyph Library (DECISIONS 33), texts, wands, enchanting, summons, polymorph, dispel, prayer and Devotion, name generator, AI hooks",
   {"06": R(16, 29) + [31] + R(38, 50) + R(53, 56)}, 9)
wp(31, "Materials, quality, stations, recipes, assembly", "M2", "EA",
   [("07", ["1", "2", "3", "4", "5", "6", "7"])], "palimpsest_sim::craft", [26, 13],
   "one material table, stat derivation, quality rolls, stations, recipes, parts and licensed grafts",
   {"07": R(1, 13) + [29, 30, 31, 35, 46, 51]}, 8)
wp(32, "Salvage, gathering, limited building, gadgets, cooking", "M2", "EA",
   [("07", ["8.3", "8.4", "9", "10", "11", "12", "13"])], "palimpsest_sim::craft", [31, 19, 25],
   "salvage and reforging, mining and nodes, farming, placeables only (DECISIONS 16), gadgets and deployables, cooking, Fieldcraft",
   {"07": R(14, 28) + R(32, 45) + R(47, 50) + [52]}, 8)
wp(33, "Alchemy and liquids catalog", "M2", "EA",
   [("08", ["3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16"]), ("00", ["12.1"])],
   "palimpsest_sim::alchemy", [17, 31, 24, 27],
   "liquid and gas catalogs, the full surfaces and reactions table, containers, effects, ingredients, brewing, delivery, toxicity, potions, bombs, transmutation",
   {"08": R(4, 59), "04": [54]}, 12)
wp(34, "Stealth and perception", "M2", "EA",
   [("09", ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "14", "15"])], "palimpsest_sim::stealth", [18, 25],
   "stealth light, visibility, perception classes, detection meter, awareness machine, noise, sneak attacks, takedowns, hiding, bodies, disguises, distractions, co-op stealth",
   {"C": [38], "09": R(1, 28) + R(37, 44) + R(47, 55)}, 10)
wp(35, "Security: locks, hacking, traps, alarms", "M2", "EA",
   [("09", ["13"])], "palimpsest_sim::stealth::security", [34, 32],
   "locks and lockpicking, hacking minigame with auto-resolve (DECISIONS 63), traps, pickpocketing, cameras, alarms (DECISIONS 61), radio",
   {"09": R(29, 36) + [45, 46, 56, 57]}, 6)
wp(36, "Items: templates, generation, affixes, perks, inventory", "M2", "EA",
   [("11", ["1", "2", "3", "4", "5", "6", "7", "8", "14", "18", "19"])], "palimpsest_sim::items (absorbs ::loot)", [25, 31],
   "generation pipeline, archetypes, genre skins, manufacturers, affixes, switchable perk columns (DECISIONS 68), Legendaries, Exotics, uniques, inventory with weight (DECISIONS 66), Item Power",
   {"C": [20, 21, 37, 39], "05": [63], "11": R(1, 6) + R(12, 16) + [25, 26, 27, 30, 32, 33, 34, 35, 38, 40, 44]}, 10)
wp(37, "Identification, appraisal, curses and pacts", "M2", "EA",
   [("10", ["2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13"]), ("00", ["13.6"])], "palimpsest_sim::ident", [36],
   "knowledge model, appearance shuffle, `cat.wand`, identification methods, value model, curses, blessings and pact curses (DECISIONS 60), seals, Index and Bestiary",
   {"10": R(1, 43), "P": ["every_find_a_question.t1"]}, 9)
wp(38, "Loot sources and the in-run economy", "M2", "EA",
   [("11", ["9", "10", "11", "12", "13", "15", "16", "17"])], "palimpsest_sim::loot", [37],
   "kill drops, deck, bad-luck protection, rank tables, chests, bags, Volume plan, placement generators, in-run Reliquaries, Fortune Engines with cursed capsules, shops, Lucre, Rewrite (DECISIONS 67), Emendations, co-op loot",
   {"11": R(7, 11) + R(17, 24) + [29, 31, 37, 39, 41, 42, 43], "P": ["party_of_errata.t3", "every_find_a_question.t2"]}, 10)
wp(39, "AI core", "M2", "EA",
   [("13", ["4"])], "palimpsest_ai; palimpsest_sim::ai", [15, 22, 8],
   "blackboard, inputs, response curves, scoring, action layer, behavior sets, threat, morale, squads, cover, flanking, A* and HPA*, think budget",
   {"13": R(4, 10) + [34, 35] + R(40, 44) + [52]}, 12)
wp(40, "GPV, genres and World Laws", "M2", "EA",
   [("12", ["2", "3", "4", "24.3"]), ("00", ["5"])], "palimpsest_data (GPV records); palimpsest_sim::genre", [6],
   "GPV records for all 16 genres, lints L1–L10, World Laws with discovery state (DECISIONS 56), `st.contradicted`, `st.unmoored`",
   {"C": [14, 15, 40], "12": [14, 15, 16, 27, 39], "01": [5], "P": ["one_ruleset.t2"]}, 5)
wp(41, "Lattice, exits, Folio types, sub-streams, bleed plan", "M2", "EA",
   [("12", ["5", "6", "7", "14.1"]), ("00", ["3.2", "17.1"])], "palimpsest_gen::lattice", [40, 1],
   "pure `lattice_candidates` with the EA relaxation cascade (§2.3), Folio types, stage seeds, bleed plan, exit previews",
   {"C": [47], "12": R(1, 9) + [38, 44]}, 5)
wp(42, "Layout: mission graph, embedding, WFC", "M2", "EA",
   [("12", ["8", "9", "10"])], "palimpsest_gen::layout", [41, 13],
   "mission graph grammar, spatial embedding, corridors and stairs, relink pairs, room templates, tilesets, WFC",
   {"12": [17, 21, 25, 28, 42]}, 10)

# ---------------- M3 Headless game ----------------
wp(43, "Folio finishing, validation, secrets, stories, arenas", "M3", "EA",
   [("12", ["11", "12", "13", "14", "15", "16", "17", "18", "19", "20", "21", "23"])], "palimpsest_gen::finish", [42, 16, 17, 18],
   "dressing, nodes, light plan, hazards, population and loot-spot interfaces, zones, validation and repair, room services and puzzles, secret rooms, events, story fragments, boss arenas, Abridged layouts, the per-Folio time-of-day and weather rolls that feed light, sound, surfaces and fire (DECISIONS 99)",
   {"C": [31], "12": [10, 11, 12, 13, 18, 19, 20, 24, 26] + R(29, 35) + [37, 40, 41, 43]}, 12)
wp(44, "Enemies, Director, factions and NPCs", "M3", "EA",
   [("13", ["2", "3", "5", "6", "7", "10", "12", "13"]), ("00", ["11.6", "15", "17.3"])], "palimpsest_sim::enemies", [39, 25, 38, 34, 43],
   "templates and scaling, roles, ranks, elites and champions, minions, enemy items and spells, Director, softer Erasure hunters (DECISIONS 26), factions and run-scoped reputation (DECISIONS 71), vendors, quests, hirelings and tamed beasts (DECISIONS 36), dialogue",
   {"13": [1, 2, 3] + R(11, 22) + R(27, 31) + [33] + R(36, 39) + [45, 46] + R(48, 51) + [53, 54, 55]}, 12)
wp(45, "Bosses: five Editors and mini-bosses", "M3", "EA",
   [("13", ["9"]), ("12", ["21"])], "palimpsest_sim::enemies::boss", [44],
   "Editor templates with verses (DECISIONS 45), mini-boss framework for 2 per genre (DECISIONS 69), phases, enrage, arena construction",
   {"13": [24, 25, 26, 32]}, 7)
wp(46, "Run layer for the EA modes", "M3", "EA",
   [("14", ["1", "2", "3", "4", "7", "8.1", "8.2a", "8.3", "8.4", "8.5", "8.6", "8.7", "8.8", "11", "12"])], "palimpsest_sim::run", [45, 41, 36, 28],
   "`RunState`, lobby machine, content sets, kits and Margin Translation, Folio phases, Rubrics, notes, exit votes, boss gates, Erasure timeline, rests, ghosts, score, Grades, the EA modes and overlays, save and suspend",
   {"14": [1, 3, 4, 5, 6] + R(9, 15) + [20, 21, 22, 23, 25] + R(29, 35) + [37, 43, 45, 50], "01": [12, 13, 14, 15], "P": ["every_find_a_question.t3"]}, 12)
wp(47, "Bots and the balance harness", "M3", "EA",
   [("02", ["16"]), ("13", ["11"])], "palimpsest_bots", [46, 30, 33, 35],
   "Pilgrim bots with combat-led, stealth-led and environment-led policies, `run_batch`, `pal balance`, `bot_typical` throughput",
   {"02": [47], "05": [48], "13": [23], "X": ["`perf.ci.bot_tps` ≥ 6,000 ticks per second"]}, 8)
wp(48, "Signature moments, pillar registry, KPI functions", "M3", "EA",
   [("01", ["2.7", "3.13", "10"])], "palimpsest_bots::moments; palimpsest_tools::gate", [47],
   "SM-01 to SM-11 scenario files and runner, pillar records, `release_gate`, `prefer`, KPI `evaluate` and `aggregate`",
   {"01": [1, 2, 3, 4, 6, 7, 8, 9, 10, 11, 16, 19, 20, 22, 23], "SM": R(1, 11)}, 6)

# ---------------- M4 First playable client ----------------
wp(49, "Sim-render bridge", "M4", "EA",
   [("02", ["7"])], "palimpsest_bridge", [10, 20],
   "view mailbox, view frame, interpolation, entity mapping, the float boundary `conv`, event routing",
   {"02": [13, 34, 37]}, 5)
wp(50, "Renderer core", "M4", "EA",
   [("02", ["8.1", "8.2", "8.3", "8.5", "8.6", "8.7"]), ("18", ["2", "4.1", "4.2", "4.5", "4.6"])], "palimpsest_render", [49, 13],
   "frame order, micro-voxel chunk meshing, per-genre block materials, light agreement, post stack, anti-aliasing, particles, decals, ink edges, paper grain, damage VFX, ink gore with the red-blood toggle (DECISIONS 94)",
   {"02": [12, 14], "18": [4, 5, 6, 13, 37, 49, 51]}, 12)
wp(51, "Viewmodel, cameras, animation, transitions", "M4", "EA",
   [("02", ["8.4", "8.8", "8.9"]), ("18", ["4.3", "4.4", "4.7", "5"])], "palimpsest_render", [50],
   "first-person viewmodel with full-body awareness and the third-person toggle (DECISIONS 3), procedural creature animation, anim step, render-only ragdolls, Going Native forms, page turn, bleed blend",
   {"02": [15], "18": [3, 7, 8, 9, 47, 48, 54]}, 9)
wp(52, "Audio", "M4", "EA",
   [("02", ["9"]), ("18", ["6"])], "palimpsest_audio", [49, 18],
   "Firewheel plugin, spatialization, occlusion from the sound grid, music stems and levels, bleed crossfades, mix",
   {"02": [16, 42], "18": [10, 11, 12, 28, 29, 38, 39, 50, 55, 57]}, 7)
wp(53, "Input action layer and controls", "M4", "EA",
   [("17", ["12"]), ("02", ["10"])], "palimpsest_ui::input", [10],
   "action list, command encoding, KB/M, gamepad and Deck presets, gyro, aim assist, rebinding, toggle and hold, contexts",
   {"02": [43], "17": [7, 8, 26, 27, 28, 35, 42, 44, 47, 48, 50, 52, 54], "01": [21]}, 6)
wp(54, "UI framework, genre skins, HUD, item card, character", "M4", "EA",
   [("17", ["1", "2", "3", "4", "5"])], "palimpsest_ui", [53, 49],
   "skin model and tokens, HUD elements, Book menu, satchel and paper doll, item card, character sheet, the preset character creator (12 presets per kin with palette and marking colors, DECISIONS 97)",
   {"17": [9, 10, 12, 13, 14, 23, 41, 49, 51], "18": [36], "P": ["out_of_story.t3"]}, 8)
wp(55, "Making, journal, map, run-flow and system screens", "M4", "EA",
   [("17", ["6", "7", "8", "9", "10", "15", "16"])], "palimpsest_ui", [54],
   "crafting, placement mode, alchemy, spell composer, Grimoire wheel, journal, map, exit choice, shop, Reliquary and Engine panels, Colophon, trade, run summary, ghost and downed HUDs, onboarding, feedback",
   {"17": [15, 18, 19, 20, 24, 25, 37, 38, 39, 40, 43, 46, 55, 56, 57, 58, 59, 60, 61]}, 8)
wp(56, "Accessibility and Scribe's Mercy", "M6", "EA",
   [("17", ["13"])], "palimpsest_ui::access", [55],
   "Scribe's Mercy (never blocks achievements, DECISIONS 82), subtitles, color-blind support, text and contrast, FOV, screen reader, sound visualization, photosensitivity, game speed, madness markers (DECISIONS 49)",
   {"17": [1, 2, 3, 4, 5, 6, 11, 29, 30, 32, 33, 34, 36, 45, 53, 62]}, 6)
wp(57, "Localization pipeline for 15 locales", "M4", "EA",
   [("17", ["14"]), ("18", ["9.11"]), ("02", ["10"])], "palimpsest_ui::loc", [54],
   "Fluent loading and key map, fallback chain, `en-XA` overflow screenshots, font coverage, CJK line breaking",
   {"02": [18], "17": [31]}, 4)
wp(58, "Text-only narration, endings and grammars", "M6", "EA",
   [("18", ["1.2", "7", "8", "9.9"])], "palimpsest_scribe (template engine); palimpsest_ui::narration", [57, 46, 1],
   "marginalia narration (DECISIONS 38), lore, Editors' lines, Margin cast arcs (DECISIONS 78), endings and the closing manuscript (DECISIONS 46), item flavor, story and name grammars, name review, casino-lexicon lint",
   {"18": [14, 15, 16, 17, 18, 19, 25, 26, 31, 32, 33, 40, 42, 45, 46, 56, 58], "17": [63], "01": [18]}, 8)
wp(59, "Walkable Margin hub and meta progression", "M5", "EA",
   [("14", ["5", "6", "10"]), ("17", ["11"])], "palimpsest_platform::meta; palimpsest_ui::margin; palimpsest_render", [55, 46, 30],
   "first-person 3D Margin (DECISIONS 77), facilities and milestones, trophy alcove, Bindery, Codex knowledge, Echo roster and Recollection trials (DECISIONS 80), achievements, Grade unlocks, Inkwell, Vault, Codex and Store screens",
   {"14": [16, 17, 18, 26, 41, 46, 47], "17": [16, 17, 21, 22]}, 10)
wp(60, "Client executable", "M4", "EA",
   [("02", ["3.2", "7.1", "14"])], "palimpsest_client", [50, 52, 54],
   "the Windows x86_64 executable (DECISIONS 98): plugin assembly, sim thread, first launch, settings, suspend handling, client smoke test under 16 schedule shuffle seeds",
   {"X": ["doc 02 §16 item 8 smoke test passes"]}, 5)
wp(61, "Steam Deck and PC performance, Deck Verified checklist", "M6", "EA",
   [("02", ["8.10", "14", "15"]), ("18", ["10.2"])], "palimpsest_render; palimpsest_client", [60, 21],
   "render presets, Deck targets under Proton (DECISIONS 98), memory and VRAM budgets, every `perf.deck.*` budget on `deck-01`, minimum-PC gate (PAR-13), the Deck Verified checklist",
   {"02": [33], "18": [52]}, 6)

# ---------------- M5 Online ----------------
wp(62, "Transport, protocol and input path", "M5", "EA",
   [("16", ["1", "2", "3.1", "3.2", "3.4", "3.7", "3.8", "3.9", "4", "5"])], "palimpsest_net", [10],
   "`Transport` trait, channels, envelope, handshake and challenge, time sync, session machines, input packets, redundancy, jitter buffer, `view_tick` clamp, tick controller, intents, sanitization",
   {"16": R(1, 6) + [25, 40, 55, 56]}, 8)
wp(63, "Snapshot replication", "M5", "EA",
   [("16", ["6"])], "palimpsest_net", [62, 20, 13],
   "snapshots, quantization, deltas, interest and priority, presentation events, voxel diffs, field pages, Folio-entry keyframes, the Confirmed Input Log",
   {"16": R(7, 15) + [17]}, 9)
wp(64, "Prediction, reconciliation, lag compensation", "M5", "EA",
   [("16", ["7", "8"]), ("05", ["2.6"])], "palimpsest_net", [63, 24],
   "own-mover prediction, reconciliation and smoothing, remote interpolation, lag-compensated hits",
   {"05": [41], "16": [33, 41, 42]}, 6)
wp(65, "Co-op rules", "M5", "EA",
   [("16", ["9.1", "9.2", "9.3", "9.4", "9.5", "9.6", "9.7", "9.8", "9.9", "9.10", "9.11", "9.12"]), ("00", ["17.5", "17.6"])], "palimpsest_net; palimpsest_sim::run", [64, 46],
   "drop-in at Folio entry, Dormant and reconnect, host migration (DECISIONS 76), downed and revive, ghosts (DECISIONS 50), instanced loot, trading, exit voting, shared knowledge, friendly fire options, chat, voice and pings (DECISIONS 74, 75)",
   {"16": R(18, 24) + [34, 43, 46, 47, 48, 49, 54]}, 9)
wp(66, "Replays, ranked verification, killcam", "M5", "EA",
   [("16", ["10"]), ("17", ["9.7"])], "palimpsest_server; palimpsest_tools; palimpsest_ui", [63, 11, 55],
   "replay files, daily and ranked verification, desync diagnostics, the replay viewer and killcam (DECISIONS 48)",
   {"02": [38], "14": [24, 51], "16": [27, 28, 53], "17": [64, 65]}, 5)
wp(67, "Platform layer and Steam backend", "M5", "EA",
   [("02", ["12", "14"]), ("16", ["3.3"])], "palimpsest_platform (feature `steam`)", [62, 11],
   "`Platform` trait, Null and Steam backends, SDR transport, lobbies, achievements, Auto-Cloud, rich presence, Input glyphs, on-screen keyboard, purchase callback, atomic save IO, crash handler",
   {"02": [20, 39, 45], "14": [36]}, 8)
wp(68, "Backend services", "M5", "EA",
   [("16", ["11", "12", "13", "14"])], "palimpsest_services", [66, 7],
   "`svc.auth`, `svc.profile`, `svc.lobby`, `svc.relay`, `svc.leaderboard`, `svc.verify`, `svc.telemetry`, `svc.crash`, `svc.cloud`, `svc.status`, `svc.moderation`; Postgres, KMS keys, observability, retention",
   {"16": [29, 30, 50]}, 12)
wp(69, "EOS backend and crossplay identity", "M5", "EA",
   [("16", ["3.5", "3.6"])], "palimpsest_platform (feature `eos`)", [67, 68],
   "Epic Online Services sign-in and achievements, relay route for mixed sessions, account linking, friend codes (DECISIONS 73)",
   {"16": [26]}, 7)
wp(70, "Invasions", "M5", "EA",
   [("16", ["9.13"]), ("13", ["10.7"]), ("14", ["8.1"]), ("00", ["17.9"])], "palimpsest_net; palimpsest_sim::enemies; palimpsest_services", [65, 68, 45],
   "opt-in overlay, beacons and tickets, invader kits, slot 4, PvP culling, sealing, window and end rules, Redaction, host loss, cosmetic-only rewards (DECISIONS 15, 32, 86)",
   {"11": [36], "13": [47], "14": [40, 49], "16": [16, 31, 37, 38, 39, 44, 45, 52]}, 7)
wp(71, "Network test plan and bandwidth budgets", "M5", "EA",
   [("16", ["15", "16"])], "palimpsest_net (NetSim)", [65],
   "NetSim scenarios, bandwidth budget checks, 4-player nightly co-op bot runs with desync counting",
   {"16": [32, 36]}, 4)
wp(72, "Telemetry and crash ingest", "M5", "EA",
   [("02", ["17.4"]), ("01", ["10"])], "palimpsest_platform; palimpsest_services", [68],
   "opt-in consent flow, run summaries with the per-Folio aggregates of doc 01, minidump and desync-dump upload, deletion by install ID",
   {"02": [32], "16": [35]}, 4)
wp(73, "Economy core crate", "M5", "EA",
   [("15", ["4"]), ("02", ["3.4"])], "palimpsest_econ", [1, 3],
   "ChaCha20, HMAC draw derivation, commit-reveal verification, odds and pity math, Ed25519 receipts, `palimpsest-verify`, `PAID_RANDOM_ITEMS_ENABLED = false`",
   {"15": [7, 8, 9, 10, 30, 31]}, 6)
wp(74, "Account economy: Inkwell, Reliquaries, Codex", "M5", "EA",
   [("15", ["2", "3", "5", "6", "10"])], "palimpsest_econ; palimpsest_services (`svc.econ`)", [73, 68, 59],
   "currencies and earn rates, Inkwell with pity, split and spark (DECISIONS 57), account Reliquaries (DECISIONS 59), Codex with the Quill (DECISIONS 58), Echo binding, ledger, run reports",
   {"15": R(1, 6) + R(11, 22) + [32, 33], "16": [51], "14": [39], "P": ["fair_page.t2"]}, 12)
wp(75, "Store, compliance and ratings package", "M5", "EA",
   [("15", ["1", "7", "8", "9", "12"])], "palimpsest_econ; palimpsest_services; palimpsest_ui::store", [74, 67],
   "fiat SKUs with the base game at USD 19.99 (DECISIONS 42), jurisdiction flags, minor caps, chat safety, anti-predatory rules, guardrail KPIs, rating dossiers",
   {"15": R(23, 29), "01": [17], "P": ["fair_page.t1"]}, 8)
wp(76, "Offline-complete and end-of-service mode", "M5", "EA",
   [("15", ["11"]), ("16", ["13"])], "palimpsest_econ; palimpsest_platform", [74, 65],
   "offline receipts, Sealed Drafts, the Retrospective banner, sunset build switches",
   {"15": [34, 35], "P": ["fair_page.t3"]}, 4)

# ---------------- M6 Content complete ----------------
wp(77, "Mod data packs", "M6", "EA",
   [("02", ["13.1", "13.4"])], "palimpsest_data::mods", [6, 62],
   "mod folders, classification, append-only patches, forbidden namespaces, `mod_blob`, `modset_hash`, handshake refusal",
   {"02": [35, 40, 41]}, 4)
wp(78, "WASM scripted effects", "M6", "EA",
   [("02", ["13.2", "13.3"]), ("00", ["3.7"])], "palimpsest_mod_api", [77, 22],
   "wasmtime host with fuel and NaN canonicalization, the WIT world, guest SDK, `ScriptHost` calls inside the sim",
   {"02": [28, 29]}, 7)
wp(79, "Content extraction and spec sync", "M2", "EA",
   [("19", []), ("00", ["5.2", "11.1", "17"])], "palimpsest_tools (`pal extract`, `pal spec-sync`)", [6],
   "chapter definition tables and canon tables to JSON, the sync check that chapter tables equal content records",
   {"14": [19], "X": ["every chapter table record equals its JSON record"]}, 5)
wp(80, "Content catalog generator for doc 20", "M2", "EA",
   [("20", []), ("18", ["10.1"])], "palimpsest_tools (`pal catalog`)", [79],
   "`spec/20_CONTENT_CATALOG.md` generated from `content/`, per-genre counts against §5.3 of this chapter, the EA genre-set report",
   {"X": ["catalog regeneration is byte-identical"]}, 4)
wp(81, "Asset pipeline tools and lints", "M3", "EA",
   [("18", ["10"]), ("02", ["11.4"])], "palimpsest_tools (`pal bake-poses`, `pal import-vox`)", [6],
   "pose baking, `.vox` import, asset manifest and license lint, silhouette, palette, contrast and LUT checks",
   {"18": [1, 2, 27, 30, 34, 35, 41]}, 5)
wp(82, "Global gameplay content for EA", "M6", "EA",
   [("04", ["15"]), ("06", ["5", "8.6"]), ("07", ["2", "6.6"]), ("08", ["8"]), ("11", ["6", "7", "8"]), ("13", ["8.15", "8.17", "9.2"]), ("14", ["2.4", "5.3", "6.3", "10"])],
   "`content/`", [79, 36, 30, 33, 44, 46],
   "13 Calling kits with alt and Script kits, 24 Echoes and their trials, 12 Editor-squad templates, cross-genre templates, affixes, perks, Legendaries, uniques, glyphs, ingredients, recipes, materials, 91 Bindery nodes, 58 achievement records",
   {"14": [38, 42, 48], "P": ["party_of_errata.t2"]}, 15)
wp(83, "Global presentation assets", "M6", "EA",
   [("18", ["3.5", "5.5", "8.3", "8.4", "8.9", "10.2"])], "`assets/`", [81, 51],
   "Errata bodies with 72 creator presets (12 per kin), 13 Calling looks, 24 Echoes including the quadruped and serpentine bodies, 5 Editors and squads, Red Ink, the Margin hub, UI sounds and fonts, Ur-Text quote stems",
   {"X": ["doc 18 per-asset budgets and lints pass for every global asset"]}, 20)

GENRES_EA = [("mythic_fantasy", "Mythic Fantasy", "8.1"), ("gothic_horror", "Gothic Horror", "8.2"),
             ("cyberpunk", "Cyberpunk", "8.5"), ("weird_west", "Weird West", "8.7"),
             ("wuxia", "Wuxia", "8.9"), ("steampunk", "Steampunk", "8.10")]
n = 84
data_ids = {}
for suf, name, sec in GENRES_EA:
    wp(n, f"Genre pack data: {name}", "M6", "EA",
       [("12", ["3", "4", "18.2", "19.2", "20"]), ("13", [sec, "9.3"]), ("11", ["4.2", "5.2"])], f"`content/*/` records for `genre.{suf}`",
       [80, 36, 43, 45],
       "≥ 12 roster templates, 2 mini-bosses, GPV, World Laws, 2 tilesets, 6 hazards, 6 events, story grammar, weapon and armor skins, 2 manufacturers, alchemy and appearance skins, en-US keys (§5.3)",
       {"12": [18], "X": [f"§5.3 counts for {name}"]}, 10)
    data_ids[suf] = n; n += 1
wp(n, "Genre pack data: The Ur-Text and Unwritten", "M6", "EA",
   [("12", ["21", "23"]), ("13", ["8.16"]), ("14", ["11"])], "`content/*/` records for `genre.ur_text`, `genre.unwritten`",
   [80, 36, 43, 45],
   "Ur-Text GPV, Law, tilesets and a 12-template roster; the Unwritten's seven rooms",
   {"X": ["§5.3 counts for the Ur-Text"]}, 8)
data_ids["ur_text"] = n; n += 1
asset_ids = {}
for suf, name, sec in GENRES_EA:
    wp(n, f"Genre pack assets: {name}", "M6", "EA",
       [("18", ["3", "5.5", "6.1", "6.3", "7.4"]), ("17", ["2"])], "`assets/`", [data_ids[suf], 81, 51, 52],
       "block array, art bible, palette, light rig, post, LUT, UI skin, meshes for the roster and mini-bosses, Going Native forms, time-of-day and weather sets, music track of 5 stems, ambience, SFX set",
       {"18": [34], "X": [f"doc 18 lints pass for {name}"]}, 18)
    asset_ids[suf] = n; n += 1
wp(n, "Assets: The Ur-Text and Unwritten", "M6", "EA",
   [("18", ["3", "6.1", "8.5a"])], "`assets/`", [data_ids["ur_text"], 81, 51, 52],
   "Ur-Text block array, palette, post, music and quote stems, closing-manuscript plates; the Unwritten's Unwritten set",
   {"X": ["doc 18 lints pass for the Ur-Text and Unwritten sets"]}, 12)
asset_ids["ur_text"] = n; n += 1
assert n == 98, n
wp(98, "Localization production, EA", "M6", "EA",
   [("17", ["14"]), ("18", ["9.11"])], "`assets/loc/`", [57, 82] + list(data_ids.values()),
   "14 translations of every EA key, linguistic QA, `en-XA` pass, text freeze at the M6 gate",
   {"X": ["0 missing keys in 15 locales"]}, 10)
wp(99, "EA balance and bot pillar sweeps", "M6", "EA",
   [("02", ["16"]), ("01", ["2", "6.6"]), ("13", ["11.2"]), ("15", ["2.7", "2.8"]), ("14", ["5.4"])], "palimpsest_bots; `content/`", [47, 74] + list(data_ids.values()),
   "Folio-1 anchors, depth curve, pacing bands, party parity, Quire and Ink rates, Abridged and Novella lengths, the EA reading of the bot pillar tests (§2.3)",
   {"02": [47], "P": ["one_ruleset.t1", "out_of_story.t1", "out_of_story.t2", "living_page.t1", "living_page.t3", "party_of_errata.t1"]}, 10)
wp(100, "Human playtests and blind genre recognition", "M7", "EA",
   [("01", ["2.1"])], "palimpsest_client (test flag)", [61, 65] + list(asset_ids.values()),
   "≥ 20 fresh testers over the 6 EA genres with labels hidden, Deck sessions, co-op nights, P0 and P1 triage",
   {"P": ["one_ruleset.t3"]}, 4)
wp(101, "Release engineering for EA", "M7", "EA",
   [("02", ["17.2"]), ("15", ["8.6", "3.2"])], "CI; store back ends", [75, 61, 69, 98],
   "Steam app with its Windows depot, Epic Games Store build, channels `dev` to `stable`, store pages, Deck Verified submission under Proton, ratings, accessibility questionnaire, Codex 001 and banner p001 at `T0`",
   {"X": ["`T0 = 1820448000` in the economy service configuration"]}, 8)
wp(102, "EA release gate", "M7", "EA",
   [("01", ["2.7"]), ("02", ["17.2"])], "palimpsest_tools::gate", [99, 100, 101, 76, 78, 70, 71, 72, 56, 58, 48, 83] + list(asset_ids.values()),
   "18 pillar results on the candidate build, performance gate, channel promotion record, offline test, compliance audit",
   {"X": ["`release_gate(Gate::EarlyAccess)` returns true on the candidate build"]}, 3)

# ---------------- Post-EA updates and 1.0 ----------------
def genre_pair(ms, a, b, n):
    ids = []
    for suf, name, sec in (a, b):
        wp(n, f"Genre pack data: {name}", ms, "1.0",
           [("12", ["3", "4", "18.2", "19.2", "20"]), ("13", [sec, "9.3"]), ("11", ["4.2", "5.2"])], f"`content/*/` records for `genre.{suf}`",
           [102], "the §5.3 counts for one core genre", {"12": [18], "X": [f"§5.3 counts for {name}"]}, 10)
        ids.append(n); n += 1
    for k, (suf, name, sec) in enumerate((a, b)):
        wp(n, f"Genre pack assets: {name}", ms, "1.0",
           [("18", ["3", "5.5", "6.1", "6.3", "7.4"]), ("17", ["2"])], "`assets/`", [ids[k]],
           "the per-genre asset families of §5.3, time-of-day and weather sets", {"18": [34], "X": [f"doc 18 lints pass for {name}"]}, 18)
        ids.append(n); n += 1
    return ids, n

wp(103, "Asynchronous traces", "M8", "1.0",
   [("16", ["10", "11"]), ("17", ["11.6"])], "palimpsest_services; palimpsest_ui; palimpsest_scribe", [102],
   "opt-in grammar-built marginal notes and replayable ink-stain death echoes in other players' Folios of the same seed or genre (DECISIONS 96), moderation and reporting",
   {"X": ["a note left in a seeded Folio appears for a second account on the same seed; a reported note is hidden"]}, 10)
upd_ids = {}
upd_ids["M8"], n = genre_pair("M8", ("fairy_tale", "Fairy Tale", "8.11"), ("mythic_antiquity", "Mythic Antiquity", "8.12"), 104)
assert n == 108
wp(108, "Streaming audience votes", "M9", "1.0",
   [("16", ["11"]), ("17", ["11.6"])], "palimpsest_services; palimpsest_ui", [102],
   "opt-in Twitch and YouTube votes on doors, boons and a Chaos mutator, never in ranked or daily runs, streamer-safe mode (DECISIONS 104)",
   {"X": ["a vote result enters the run as a host intent and replays identically; ranked and daily lobbies refuse the overlay"]}, 8)
upd_ids["M9"], n = genre_pair("M9", ("cosmic_horror", "Cosmic Horror", "8.3"), ("hardboiled_noir", "Hardboiled Noir", "8.8"), 109)
assert n == 113
upd_ids["M10"], n = genre_pair("M10", ("space_opera", "Space Opera", "8.4"), ("post_apocalypse", "Post-Apocalypse", "8.6"), 113)
assert n == 117
wp(117, "Secret genres: Oneiric, The Redaction, generated Unwritten, secret exits (data)", "M11", "1.0",
   [("12", ["9.5", "17.4"]), ("13", ["8.13", "8.14", "8.15"])], "palimpsest_gen; `content/`", [102],
   "gravity-flip volumes, secret-exit chances, three secret rosters at ≥ 12 templates, 4 mini-bosses",
   {"12": [22, 23, 36]}, 12)
wp(118, "Secret genres: assets", "M11", "1.0",
   [("18", ["3", "6.1"])], "`assets/`", [117], "Oneiric, Redaction and generated Unwritten asset families", {"X": ["doc 18 lints pass for the secret sets"]}, 18)
wp(119, "Standard run, Endless Marginalia and boards", "M11", "1.0",
   [("14", ["8.2", "9"]), ("12", ["22"]), ("11", ["16.2"]), ("00", ["13.7", "13.8"])], "palimpsest_sim::run; palimpsest_gen", [102] + upd_ids["M10"],
   "`mode.standard`, `mode.endless`, Marginal mutations, Marginalia modifiers and Emendations, Standard and Endless boards, `run.mut.short_book`, the 12 remaining achievements",
   {"14": [2, 7, 8, 27, 28, 44], "11": [28]}, 8)
wp(120, "Second Edition: fused-genre Folios and the rewritten lattice", "M11", "1.0",
   [("12", ["5", "14"]), ("00", ["5.8"])], "palimpsest_gen; palimpsest_sim::genre", [119, 117],
   "Folios written in two fused genres with both rosters, Laws and skins and averaged axioms, built on seams and bleed; remixed genre pairings after the first Ur-Text clear (DECISIONS 100, 101)",
   {"X": ["a fused Folio's local axioms equal doc 12's average of its two genres; its preview equals its generation"]}, 12)
wp(121, "Second Edition: revised Editors and the Author", "M11", "1.0",
   [("13", ["9"]), ("18", ["8.3"])], "palimpsest_sim::enemies::boss; `content/`; `assets/`", [120],
   "revised phases and mechanics for the 5 Editors and a new final boss, the Author, with their art, music cues and lines (DECISIONS 103)",
   {"X": ["each revised Editor passes doc 13 test 26's phase-shift rule; the Author's arena seals and unseals per doc 13 §9.1"]}, 20)
wp(122, "Second Edition: carry-over, run flow and second ending layer", "M11", "1.0",
   [("14", ["3", "4"]), ("04", ["6"]), ("11", ["2"]), ("18", ["8.5a"])], "palimpsest_sim::run; palimpsest_scribe", [121],
   "the Errata who clear the Ur-Text continue with gear and levels against scaled enemies; the second ending layer of the closing manuscript (DECISIONS 100, 102)",
   {"X": ["a Second Edition run resumes from the Ur-Text victory state and replays to the same hashes"]}, 8)
wp(123, "Steam Workshop and the Workshop mirror", "M11", "1.0",
   [("02", ["13", "14"]), ("16", ["11.8"])], "palimpsest_platform; palimpsest_services", [102],
   "Workshop upload and subscribe (DECISIONS 43), the `svc.workshop` mirror for Epic players",
   {"X": ["a Workshop data pack installs, classifies and joins a co-op session"]}, 5)
wp(124, "On-device Scribe", "M11", "1.0",
   [("18", ["9"]), ("02", ["3.2"])], "palimpsest_scribe (feature `llm`)", [102],
   "model packaging, jobs, prompt templates, grammar-constrained JSON, cache, latency machine, safety filter, disclosure (DECISIONS 14)",
   {"18": [20, 21, 22, 23, 24, 43, 44]}, 7)
wp(125, "Localization production, 1.0", "M11", "1.0",
   [("17", ["14"])], "`assets/loc/`", [117, 122] + upd_ids["M10"], "translations of every 1.0 key", {"X": ["0 missing keys in 15 locales"]}, 8)
wp(126, "1.0 balance and pillar gate", "M11", "1.0",
   [("01", ["2.7"])], "palimpsest_bots", [103, 108, 118, 119, 122, 123, 124, 125] + [upd_ids[m][2] for m in upd_ids] + [upd_ids[m][3] for m in upd_ids],
   "Standard-run pillar sweeps over 12 genres, blind recognition for every added genre", {"X": ["`release_gate(Gate::Later)` returns true"]}, 6)
wp(127, "Expansion 01: Age of Sail and Pirates", "M12", "Post",
   [("00", ["5.2"]), ("12", ["25"]), ("15", ["1.1"])], "`content/`; `assets/`", [126],
   "a new genre row, GPV, Laws, roster ≥ 12, 2 mini-bosses, skins, 2 manufacturers, 2 Echoes, 3 Scripts, SKU, expansion banner; ships are Folio layouts, never vehicles",
   {"12": [44], "X": ["canon test 40 extended to the new vector"]}, 40)
wp(128, "Expansion 02: Kaiju and Mecha", "M13", "Post",
   [("00", ["5.2"]), ("12", ["25"]), ("15", ["1.1"])], "`content/`; `assets/`", [127],
   "a new genre row with the same package; mecha are powered-armor skins and Going Native forms, kaiju are rosters and mini-bosses, nothing is piloted",
   {"12": [44], "X": ["canon test 40 extended to the new vector"]}, 40)

MILESTONES = [
    ("M0", "Foundations", "2026-11-13"),
    ("M1", "Headless world core", "2027-01-15"),
    ("M2", "Headless rules", "2027-03-12"),
    ("M3", "Headless game", "2027-05-14"),
    ("M4", "First playable client", "2027-05-28"),
    ("M5", "Online, platforms and economy", "2027-06-25"),
    ("M6", "Content complete (EA candidate)", "2027-08-06"),
    ("M7", "Early Access", "2027-09-09"),
    ("M8", "Update 1", "2027-11-04"),
    ("M9", "Update 2", "2027-12-16"),
    ("M10", "Update 3", "2028-02-10"),
    ("M11", "1.0", "2028-04-06"),
    ("M12", "Folio Expansion 01", "2029-01-18"),
    ("M13", "Folio Expansion 02", "2029-10-18"),
]

RETIRED = {}


# Acceptance tests added after the first mapping: merged into the first work package that already covers each chapter.
_EXTRA_TESTS = {40: {'01': [24, 25, 26, 27, 28]}, 1: {'02': [48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60]}, 9: {'03': [53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66]}, 26: {'04': [56, 57, 58, 59, 60, 61, 62, 63]}, 22: {'05': [64]}, 29: {'06': [57, 58]}, 31: {'07': [53, 54, 55]}, 34: {'09': [58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73]}, 37: {'10': [44]}, 36: {'11': [45, 46, 47, 48, 49, 50, 51, 52, 53, 54]}, 39: {'13': [56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73]}, 46: {'14': [52, 53, 54, 55, 56, 57, 58]}, 73: {'15': [36, 37, 38, 39, 40, 41, 42, 43, 44, 45]}, 62: {'16': [57, 58, 59, 60, 61, 62, 63, 64, 65, 66]}, 53: {'17': [66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80]}}
for _w in WPS:
    for _c, _nums in _EXTRA_TESTS.get(_w['id'], {}).items():
        _w['tests'][_c] = sorted(set(_w['tests'].get(_c, [])) | set(_nums))

_EXTRA_TESTS_CANON = {1: {'C': [48, 49, 50]}}
for _w in WPS:
    for _c, _nums in _EXTRA_TESTS_CANON.get(_w['id'], {}).items():
        _w['tests'][_c] = sorted(set(_w['tests'].get(_c, [])) | set(_nums))

_EXTRA_TESTS_RENUM = {22: {'05': [65, 66, 67]}, 6: {'18': [67]}}
for _w in WPS:
    for _c, _nums in _EXTRA_TESTS_RENUM.get(_w['id'], {}).items():
        _w['tests'][_c] = sorted(set(_w['tests'].get(_c, [])) | set(_nums))
