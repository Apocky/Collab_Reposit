# 12 — World Generation and Genres
As of 2026-10-11. Status: BINDING. Conforms to canon-1.2.

This chapter owns everything that turns a run seed into a playable Folio: the genre identity table, the Genre Parameter Vector (GPV) of all 16 genres, the Genre Lattice and its exit offers, Folio types, the full generation pipeline from mission graph to validated voxel blueprint, the room catalog, secrets, hazards and traps, ambient and narrative events, procedural Folio story fragments, boss and Editor arenas, Marginalia modifiers and the Abridged layout. Generation lives in `crate.gen` (`palimpsest_gen`, doc 02) as one pure function. Nothing here uses floats in simulation state or content.

## Imports

| Source | Consumed |
|---|---|
| DESIGN_BRIEF §2–§4 | Archetype × Skin, axioms, 12 core + 4 secret genres, 5 × 5 Folios, Editor boss on Folio 5, Lattice with 2–3 previewed exits, Marginalia |
| Canon §1.1–§1.11 | ID grammar and `id32`; integer-only content; `floor_div`, `ceil_div`, `mul_bp`; Folio box and chunk index; text keys; `EntityId`/`uid`; ascending-ID iteration; no `HashMap` |
| Canon §2.2, §2.3 | tick schedule, `light_at`, seconds as ticks |
| Canon §3.1–§3.4 | `mix64`, `Rng`, `stream_seed`; streams `rng.lattice`, `rng.worldgen`, `rng.event` (owner 12), `rng.spawn`, `rng.loot_place`; keyed `rng.stealth` |
| Canon §4 | `FOLIO_EXTENT`, typical footprint 96–192, `GRAVITY_BASE`, capsule, step-up, `JUMP_H`, `SAFE_FALL`, entity caps |
| Canon §5.2–§5.8 | genre table (copied in §3.2), `dep` and the zero-axiom ban, Contradiction overrides, TEC ≤ 2 rule, `local_genre` and bleed zones delegated here |
| Canon §5.9 | genre identity table: cited by §12.3 and §13.9 but absent from the file; defined in §2 (Canon gaps G1) |
| Canon §6.4 row 35, §10.6, §11, §12 | passive detection; environmental hazard entry point; `st.contradicted`, Erasure; surfaces, concealment, spawn light |
| Canon §13.3, §13.7, §13.8, §15, §16.5 | Palimpsestic sources, Emendations, `P`/`T`/`H`; factions; patrons |
| Canon §17.1–§17.7, §18, §19 | run constants, Abridged mode, party tables, ranks, champion aspects and counters, mount headroom and `room.no_mount`; events; namespaces `lattice.`, `room.`, `gpv.`, `biome.`, `hazard.`, `secret.`, `genre.` vectors |
| Doc 02 §3.2, §3.4 | `crate.gen`, `generate_folio(input, db, par)`, `OrderedMap` |
| R6 §7–§11; R4 §7, §14; R1 §1.10, §2.5, §5.9; R2 §5.2–§5.3 | GPV blocks, weighted modifiers with exclusions, Torg zones; WFC in rooms with block modification, Dormans cycles, sweeps, 150 ms budget; Barony secrets and keyed treasure, SPD quotas, Qud sultans; Minecraft light and spawning |

## 1. Scope, vocabulary and namespaces

### 1.1 Vocabulary

| Term | Exact meaning |
|---|---|
| Cell | 16 × 8 × 16 blocks; cell `(cx, cs, cz)` covers `x ∈ 16cx..16cx+16`, `y ∈ 8+8cs..16+8cs`, `z ∈ 16cz..16cz+16`; 16 × 6 × 16 cells; `cs ∈ 0..=5` is the **storey**; `y ∈ 0..=7` is foundation, `y ∈ 56..=63` roof |
| Footprint | the genre's `W × D × storeys` cells, centered: `x0 = 16 × floor_div(16 - W, 2)`, likewise `z0` |
| Tile | a WFC unit of 4 × 4 × 4 blocks; a cell holds 4 × 2 × 4 tiles |
| Room | a node of the mission graph embedded as an axis-aligned box of whole cells |
| Zone | cells sharing one `local_genre` (Canon §5.8); a **bleed zone** differs from the Folio genre |
| Socket | an authored anchor: light, container, spawn, prop, basin, hazard, lore, door |
| SKE | standard-kill equivalent; budgets use mSKE = SKE × 1000 |
| LP | loot points, the unit of placed loot (§12) |
| `d` | depth: the Folio index, or `d_eff` in Abridged (Canon §17.1) |

### 1.2 Namespaces

This chapter owns `genre.` vectors, `gpv.`, `lattice.`, `room.`, `biome.`, `hazard.` and `secret.` (Canon §19; full list under Exports), plus the behavior of `rng.lattice`, `rng.worldgen`, `rng.event`, `st.contradicted` (§23.3) and `cat.quest` (§19.5).

## 2. Genre identity table (Canon §5.9)

Canon §12.3 and §13.9 cite a §5.9 identity table (physics multipliers, spawn light, palettes, music, enemy themes, materials, two manufacturers per genre) that the canon file lacks. This section defines it; every value is copied into the GPV (Canon gaps G1).

### 2.1 Light and physics identity

`spawn_block_max`: highest `light_block` allowing ambient and wave spawns (Canon §12.3). `fixture_level`: `light_block` emitted by a worldgen fixture. `protect_r = fixture_level - spawn_block_max - 1`: taxicab radius one fixture makes spawn-proof. `fog`: concealment per 8 m (Canon §12.2). Physics multipliers (Bp):

`fire_bp` × fire-spread chance (doc 03); `flow_bp` × liquid flow per step (docs 03, 08); `sound_bp` × per-cell sound attenuation, above 10000 sound dies faster (doc 09); `hard_bp` × block break time (doc 07); `proj_bp` × muzzle speed of projectiles launched in that local genre (doc 05).

| # | Genre | spawn_block_max | fixture_level | protect_r | fog | fire_bp | flow_bp | sound_bp | hard_bp | proj_bp |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | `genre.mythic_fantasy` | 3 | 14 | 10 | 0 | 10000 | 10000 | 10000 | 10000 | 10000 |
| 2 | `genre.gothic_horror` | 5 | 11 | 5 | 1 | 12000 | 9000 | 8000 | 10000 | 10000 |
| 3 | `genre.cosmic_horror` | 5 | 10 | 4 | 1 | 8000 | 7000 | 7000 | 12000 | 9000 |
| 4 | `genre.space_opera` | 2 | 15 | 12 | 0 | 6000 | 10000 | 10000 | 15000 | 11000 |
| 5 | `genre.cyberpunk` | 3 | 13 | 9 | 0 | 8000 | 11000 | 12000 | 13000 | 10000 |
| 6 | `genre.post_apocalypse` | 4 | 12 | 7 | 0 | 13000 | 8000 | 10000 | 8000 | 10000 |
| 7 | `genre.weird_west` | 3 | 13 | 9 | 0 | 13000 | 7000 | 9000 | 8000 | 11000 |
| 8 | `genre.hardboiled_noir` | 4 | 12 | 7 | 1 | 7000 | 12000 | 13000 | 10000 | 10000 |
| 9 | `genre.wuxia` | 3 | 12 | 8 | 0 | 10000 | 12000 | 10000 | 9000 | 9000 |
| 10 | `genre.steampunk` | 3 | 14 | 10 | 0 | 12000 | 10000 | 12000 | 12000 | 10000 |
| 11 | `genre.fairy_tale` | 4 | 11 | 6 | 1 | 11000 | 10000 | 9000 | 8000 | 8500 |
| 12 | `genre.mythic_antiquity` | 2 | 13 | 10 | 0 | 10000 | 10000 | 9000 | 11000 | 9500 |
| 13 | `genre.oneiric` | 5 | 10 | 4 | 1 | 15000 | 15000 | 8000 | 6000 | 8000 |
| 14 | `genre.redacted` | 1 | 15 | 13 | 0 | 5000 | 10000 | 6000 | 20000 | 10000 |
| 15 | `genre.unwritten` | 0 | 8 | 7 | 0 | 10000 | 10000 | 5000 | 5000 | 10000 |
| 16 | `genre.ur_text` | 4 | 14 | 9 | 0 | 10000 | 10000 | 10000 | 10000 | 10000 |

`gravity_bp` and `ambient_light` stay in Canon §5.2 and §3.2.

### 2.2 Placed light as defense

1. **Light floor.** `light_sky` = local `ambient_light` in every air cell. Stealth reads `max(light_sky, light_block)`; spawning reads `light_block` only (Canon §2.3, §12.3).
2. **Fixtures** (`block.light_fixture`, doc 07) emit `fixture_level` and are breakable; a broken fixture's cells become spawn-eligible on the next relight.
3. **Protection.** Block light falls by 1 per taxicab step (doc 03), so a cell at distance `k` is spawn-proof when `fixture_level - k > spawn_block_max`, i.e. `k <= protect_r`.
   Examples: Mythic Fantasy `protect_r = 14 - 3 - 1 = 10`; a fixture 2 blocks above the feet layer protects `|dx| + |dz| <= 8`, `2 × 8 × 9 + 1 = 145` cells. Gothic Horror `11 - 5 - 1 = 5` protects `|dx| + |dz| <= 3`, 25 cells: candlelight is a weak defense.
4. **Player light** placed by Errata (doc 07) protects the same way; `fac.editors` members erase Errata-placed blocks within 1500 mm (Canon §11.6), so an Editor unmakes a lit camp.
5. **Safe rooms** (`room.safe`) are spawn-proof on every feet cell (§10.2).

### 2.3 Presentation identity

Presentation fields never enter simulation state. Palette is four sRGB swatches: key, fill, accent, danger. Rarity colors stay global (Canon §13.3).

| # | Genre | Palette (key, fill, accent, danger) | Render style | Music register: instruments; mode; BPM |
|---|---|---|---|---|
| 1 | Mythic Fantasy | `#C9A227` `#2E5E3A` `#7A1F1F` `#E25822` | painterly | horns, harp, strings, frame drum; dorian; 70–110 |
| 2 | Gothic Horror | `#1B1B22` `#3A2E39` `#8B0000` `#C9B79C` | painterly | pipe organ, choir, cello, harpsichord; aeolian; 50–90 |
| 3 | Cosmic Horror | `#0F2A2E` `#3D6B5C` `#9FBF3B` `#2E1A47` | ink | drones, waterphone, detuned strings, low brass; atonal; 40–70 |
| 4 | Space Opera | `#0B1C3A` `#F28C28` `#3FA9F5` `#E8ECF2` | pbr | brass fanfare, synth pads, orchestra, choir; lydian; 90–130 |
| 5 | Cyberpunk | `#0A0A12` `#FF2BD6` `#00F0FF` `#F5E663` | pbr | analog synths, industrial percussion, distorted bass; phrygian; 100–140 |
| 6 | Post-Apocalypse | `#8A5A2B` `#C9A66B` `#5B6B3A` `#D94F2B` | pbr | junk percussion, slide guitar, harmonica, radio static; mixolydian; 80–120 |
| 7 | Weird West | `#E39B4A` `#7A4B2A` `#2F4F6F` `#F2D7A0` | toon | whistle, slide guitar, banjo, low choir; minor pentatonic; 70–110 |
| 8 | Hardboiled Noir | `#101010` `#6E6E6E` `#E8E8E8` `#B3001B` | ink | tenor saxophone, brushed drums, upright bass, muted trumpet; blues; 60–100 |
| 9 | Wuxia | `#2E8B57` `#E34234` `#F4F1E8` `#1C1C1C` | ink | erhu, guzheng, dizi, war drums; pentatonic; 70–150 |
| 10 | Steampunk | `#B5803A` `#6E3B1F` `#2B3A42` `#E8D5A3` | pbr | brass band, accordion, music box, whistle percussion; ionian; 90–130 |
| 11 | Fairy Tale | `#F7C6D9` `#A7D7C5` `#F9E79F` `#6C4F8C` | toon | celesta, recorder, folk fiddle, glockenspiel; ionian; 80–120 |
| 12 | Mythic Antiquity | `#F2EDE4` `#B08D57` `#C1440E` `#1F4E79` | painterly | lyre, aulos, frame drum, male chorus; dorian; 70–120 |
| 13 | Oneiric | `#6A5ACD` `#FFB6C1` `#40E0D0` `#FFD700` | xerox_collage | reversed piano, glass harmonica, choir pads, tape warble; whole-tone; 50–90 |
| 14 | The Redaction | `#F7F7F2` `#000000` `#9AA1A6` `#C8102E` | pixel_dither | HVAC hum, typewriters, sine tones, fluorescent buzz; none; 40–60 |
| 15 | The Unwritten | `#FFFFFF` `#E6E6E6` `#8C8C8C` `#C8102E` | ink | near silence, pencil scratches, single piano notes; none; 0–40 |
| 16 | The Ur-Text | `#D4C5A9` `#3B2F2F` `#C8102E` `#5C4B8A` | painterly | every core motif layered in turn; mixed; 60–120 |

### 2.4 Enemy themes, materials and manufacturers

Enemy themes guide doc 13's rosters and `fac.native.<suffix>.<group>` factions. Material pools are `mat.*` rows doc 07 must provide (Interface assumptions I3). Manufacturer IDs are proposals in doc 11's `mfr.` namespace (Canon gaps G2).

| # | Genre | Enemy themes | Material pool (weights) | Manufacturers |
|---|---|---|---|---|
| 1 | Mythic Fantasy | goblin warbands, knightly orders, wyrms, wild beasts | `mat.granite` 40, `mat.oak` 30, `mat.iron` 20, `mat.starsilver` 10 | `mfr.mythic_fantasy.anvilhold`, `mfr.mythic_fantasy.silverbough` |
| 2 | Gothic Horror | vampire courts, werebeasts, cultists, restless dead | `mat.slate` 40, `mat.ebony` 30, `mat.iron` 20, `mat.silver` 10 | `mfr.gothic_horror.hallowmere`, `mfr.gothic_horror.vesper_works` |
| 3 | Cosmic Horror | drowned congregants, aberrant geometries, possessed scholars | `mat.basalt` 40, `mat.coral` 30, `mat.bronze` 20, `mat.void_glass` 10 | `mfr.cosmic_horror.abyssal_survey`, `mfr.cosmic_horror.pale_lodge` |
| 4 | Space Opera | imperial troopers, alien broods, combat droids, void pirates | `mat.plasteel` 40, `mat.ceramic` 30, `mat.titanium` 20, `mat.quantum_glass` 10 | `mfr.space_opera.helion_yards`, `mfr.space_opera.concord_arms` |
| 5 | Cyberpunk | corp security, street gangs, drones, chrome-maddened ravagers | `mat.concrete` 40, `mat.carbon_fiber` 25, `mat.chrome` 25, `mat.neon_glass` 10 | `mfr.cyberpunk.hexacore`, `mfr.cyberpunk.zero_day` |
| 6 | Post-Apocalypse | raiders, mutants, scavenger clans, feral machines | `mat.scrap_steel` 40, `mat.concrete` 30, `mat.rubber` 15, `mat.bone` 15 | `mfr.post_apocalypse.rustwater`, `mfr.post_apocalypse.last_light` |
| 7 | Weird West | outlaws, revenants, hex-touched beasts, mining automatons | `mat.sandstone` 40, `mat.pine` 30, `mat.iron` 20, `mat.gold` 10 | `mfr.weird_west.coldiron_kettle`, `mfr.weird_west.bitterroot` |
| 8 | Hardboiled Noir | mobsters, crooked police, hired muscle, informants | `mat.brick` 40, `mat.steel` 25, `mat.mahogany` 20, `mat.glass` 15 | `mfr.hardboiled_noir.grayline`, `mfr.hardboiled_noir.boulevard` |
| 9 | Wuxia | rival sects, bandits, jiangshi, mountain beasts | `mat.cedar` 35, `mat.bamboo` 30, `mat.steel` 20, `mat.jade` 15 | `mfr.wuxia.jade_crane`, `mfr.wuxia.nine_rivers` |
| 10 | Steampunk | automatons, sky pirates, constabulary, clockwork horrors | `mat.cast_iron` 35, `mat.brass` 30, `mat.copper` 20, `mat.oak` 15 | `mfr.steampunk.brassbound`, `mfr.steampunk.halvard` |
| 11 | Fairy Tale | wolves, witches, goblins, enchanted objects | `mat.birch` 40, `mat.thornwood` 30, `mat.glass` 15, `mat.cold_iron` 15 | `mfr.fairy_tale.thimblewick`, `mfr.fairy_tale.briar_court` |
| 12 | Mythic Antiquity | hoplites, satyrs and centaurs, chimeras, shades | `mat.marble` 40, `mat.terracotta` 25, `mat.bronze` 25, `mat.orichalcum` 10 | `mfr.mythic_antiquity.hephaestine`, `mfr.mythic_antiquity.argive` |
| 13 | Oneiric | nightmares, sleepwalkers, mirror-selves, dream predators | `mat.cloud` 35, `mat.mirror` 25, `mat.featherdown` 25, `mat.dreamglass` 15 | `mfr.oneiric.somnambule`, `mfr.oneiric.lucid_loom` |
| 14 | The Redaction | Red Ink clerks, censors, enforcers, Editors | `mat.particleboard` 35, `mat.steel` 30, `mat.paper` 20, `mat.linoleum` 15 | `mfr.redacted.corrections_bureau`, `mfr.redacted.standard_issue` |
| 15 | The Unwritten | Editors only (Canon §5.2) | `mat.paper` 50, `mat.vellum` 30, `mat.graphite` 15, `mat.ink` 5 | `mfr.unwritten.blankwright`, `mfr.unwritten.first_draft` |
| 16 | The Ur-Text | every core roster plus Editors | `mat.vellum` 40, `mat.ink` 20, `mat.gold_leaf` 20, `mat.granite` 20 | `mfr.ur_text.ur_scriptorium`, `mfr.ur_text.concordance` |

## 3. Genre Parameter Vector (GPV)

### 3.1 Dimensions

A GPV is one content record `genre.<suffix>` with 59 numbered dimensions. **S** = simulation input (integer, read by `palimpsest_gen` or `palimpsest_sim`); **P** = presentation only. List fields hold IDs, never free text.

- **A, canon identity (S unless noted; Canon §5.2):** 1 `id`; 2 `display` (P, text key); 3–6 `arc`, `tec`, `myt`, `vrv` (i8 `0..=10`); 7 `aether_axiom`; 8 `gravity_bp`; 9 `ambient_light` (`light_sky` floor); 10–11 `lucre_skin`, `aether_skin` (P); 12 `tier`.
- **B, identity (§2):** 13 `family` (P enum); 14 `era_tl` (P, `0..=12`, skin era independent of TECHNE); 15 `spawn_block_max` (S, doc 13); 16 `fixture_level` (S); 17 `fog` (S, doc 09); 18–22 `fire_bp`, `flow_bp`, `sound_bp`, `hard_bp`, `proj_bp` (S, docs 03, 05, 07, 08, 09); 23 `palette` (P, 4 × RGB); 24 `render_style` (P); 25 `music` (P); 26 `enemy_themes` (P); 27 `materials` (S, `[{mat, w}]`); 28 `manufacturers` (S, 2 × `mfr.*`, doc 11); 29 `patron` (S, `patron.<suffix>`, doc 06).
- **C, structure (S, doc 12):** 30 `layout` (`gpv.layout.*`); 31–33 `fp_w`, `fp_d`, `storeys` (cells); 34–35 `rooms_min`, `rooms_max`; 36 `loop_bp`; 37 `vert_bp`; 38 `corridor_w` (blocks); 39 `cover_bp`; 40–41 `bright_bp`, `dark_bp` (§10.2); 42 `secret_bp`; 43 `trap_bp`; 44 `liquids` (`[{liq, w}]`); 45 `tilesets` (2 × `biome.*`); 46 `cycle_w` (8 × u16, §7.2); 47 `room_bias` (`[{room, bp}]`, §15.2).
- **D, population and loot (S; docs 11, 12, 13):** 48 `density_bp`; 49 `aspect_w` (3 × u16 for `elite.surging`, `elite.unstoppable`, `elite.warded`); 50 `loot_bp`; 51 `verbs` (10 × Bp: shoot, melee, cast, pray, sneak, talk, hack, craft, build, traverse); 52 `hazards` (6 × `{hazard, w}`, §17); 53 `events` (6 × `{evt, w}`, §18); 54 `event_bp`.
- **E, bleed, lattice and story (S):** 55 `drift_bp`; 56 `bleed_in_bp` (§13); 57 `secret_exits` (`[{genre, bp}]`, §16.4); 58 `laws` (`gpv.law.*`, §3.6); 59 `story_domains` (4 tags, §19).

### 3.2 Block A: canon columns (copied exactly from Canon §5.2)

| # | ID | Display name | ARC | TEC | MYT | VRV | aether_axiom | gravity_bp | ambient_light | Lucre skin | Aether skin | Tier |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | `genre.mythic_fantasy` | Mythic Fantasy | 8 | 2 | 6 | 6 | arcana | 10000 | 4 | Crowns | Mana | core |
| 2 | `genre.gothic_horror` | Gothic Horror | 5 | 3 | 9 | 3 | mythos | 10000 | 2 | Shillings | Sanguine | core |
| 3 | `genre.cosmic_horror` | Cosmic Horror | 3 | 6 | 10 | 1 | mythos | 10000 | 1 | Bonds | Gnosis | core |
| 4 | `genre.space_opera` | Space Opera | 6 | 10 | 3 | 7 | arcana | 8000 | 6 | Credits | Psi | core |
| 5 | `genre.cyberpunk` | Cyberpunk | 1 | 9 | 2 | 5 | techne | 10000 | 3 | Chits | Bandwidth | core |
| 6 | `genre.post_apocalypse` | Post-Apocalypse | 0 | 6 | 5 | 6 | techne | 10000 | 5 | Tins | Static | core |
| 7 | `genre.weird_west` | Weird West | 4 | 4 | 6 | 7 | mythos | 10000 | 6 | Eagles | Hex | core |
| 8 | `genre.hardboiled_noir` | Hardboiled Noir | 0 | 5 | 1 | 2 | verve | 10000 | 2 | Bucks | Nerve | core |
| 9 | `genre.wuxia` | Wuxia | 6 | 2 | 5 | 10 | verve | 8000 | 5 | Taels | Qi | core |
| 10 | `genre.steampunk` | Steampunk | 3 | 7 | 3 | 6 | techne | 10000 | 4 | Sovereigns | Pressure | core |
| 11 | `genre.fairy_tale` | Fairy Tale | 10 | 1 | 8 | 3 | arcana | 10000 | 6 | Fairy Gold | Glamour | core |
| 12 | `genre.mythic_antiquity` | Mythic Antiquity | 6 | 2 | 10 | 8 | mythos | 10000 | 7 | Drachmae | Ichor | core |
| 13 | `genre.oneiric` | Oneiric | 9 | 5 | 8 | 9 | arcana | 6000 | 4 | Motes | Lucidity | secret |
| 14 | `genre.redacted` | The Redaction | 2 | 8 | 8 | 0 | techne | 10000 | 9 | Vouchers | Clearance | secret |
| 15 | `genre.unwritten` | The Unwritten | 0 | 0 | 0 | 0 | none | 10000 | 8 | Lucre | Potential | secret |
| 16 | `genre.ur_text` | The Ur-Text | 5 | 5 | 5 | 5 | none | 10000 | 4 | Words | Logos | secret |

Block B (identity) is §2.1, §2.3 and §2.4 row for row. `family` and `era_tl`: Mythic Fantasy fantasy 3; Gothic Horror horror 5; Cosmic Horror horror 6; Space Opera sf 11; Cyberpunk sf 9; Post-Apocalypse sf 8; Weird West historical 5; Hardboiled Noir crime 6; Wuxia historical 3; Steampunk historical 5; Fairy Tale fantasy 2; Mythic Antiquity mythic 1; Oneiric meta 6; Redaction meta 8; Unwritten meta 0; Ur-Text meta 6. `patron` is `patron.<suffix>` for every genre.

### 3.3 Block C: structure

| # | layout | fp_w × fp_d × storeys | rooms | loop_bp | vert_bp | corr_w | cover_bp | bright_bp | dark_bp | secret_bp | trap_bp | tilesets (`biome.<suffix>.*`) |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 MF | keep | 10 × 10 × 3 | 16–20 | 1500 | 2000 | 3 | 3000 | 3500 | 2500 | 1500 | 2500 | `castle`, `cavern` |
| 2 GH | manor | 9 × 9 × 3 | 15–19 | 2000 | 2500 | 3 | 2500 | 2000 | 5000 | 2000 | 2000 | `manor`, `catacomb` |
| 3 CH | labyrinth | 10 × 10 × 3 | 14–18 | 2500 | 2000 | 3 | 2000 | 1500 | 6000 | 2000 | 1500 | `sunken_town`, `abyss` |
| 4 SO | ship | 8 × 12 × 3 | 16–20 | 1500 | 2500 | 3 | 6000 | 6000 | 0 | 1000 | 1500 | `starship`, `station` |
| 5 CP | arcology | 8 × 8 × 4 | 15–19 | 2000 | 4000 | 5 | 6000 | 3500 | 3500 | 1500 | 2000 | `megablock`, `undercity` |
| 6 PA | ruins | 12 × 12 × 2 | 13–17 | 1000 | 1500 | 5 | 5000 | 3000 | 0 | 1500 | 3000 | `wasteland`, `bunker` |
| 7 WW | frontier | 12 × 12 × 2 | 13–17 | 1000 | 1500 | 5 | 6000 | 4000 | 0 | 1000 | 1500 | `boomtown`, `mine` |
| 8 HN | streets | 9 × 9 × 3 | 15–19 | 2500 | 3000 | 5 | 5000 | 2500 | 5000 | 1500 | 1000 | `city_street`, `tenement` |
| 9 WX | terraces | 10 × 10 × 3 | 14–18 | 1500 | 4000 | 3 | 2000 | 3500 | 0 | 1500 | 2500 | `mountain_sect`, `bamboo_vale` |
| 10 SP | foundry | 9 × 9 × 3 | 16–20 | 2000 | 3500 | 3 | 5000 | 4500 | 2000 | 1500 | 2500 | `foundry`, `airship` |
| 11 FT | woodland | 11 × 11 × 2 | 14–18 | 2000 | 1500 | 5 | 3000 | 3000 | 0 | 2500 | 2000 | `deep_wood`, `cottage` |
| 12 MA | temple | 10 × 10 × 3 | 15–19 | 1500 | 2000 | 5 | 3500 | 5000 | 0 | 2000 | 3000 | `temple`, `labyrinth` |
| 13 ON | dream | 10 × 10 × 4 | 13–17 | 3000 | 4000 | 3 | 3000 | 3000 | 3000 | 2500 | 1500 | `dreamscape`, `nursery` |
| 14 RD | office | 8 × 8 × 4 | 18–22 | 3000 | 2500 | 3 | 4000 | 8000 | 0 | 2000 | 2000 | `bureau`, `archive` |
| 15 UW | blank | 12 × 12 × 2 | 10–12 | 500 | 1000 | 3 | 1000 | 2000 | 0 | 1000 | 1000 | `blank_page`, `margin` |
| 16 UT | palimpsest | 12 × 12 × 2 | 15–19 | 2000 | 2500 | 5 | 4000 | 4000 | 2000 | 2000 | 2500 | `palimpsest`, `colophon` |

`dark_bp > 0` only where `ambient_light <= 4`, because the dark band (§10.2) is effective light `<= 4`. Content lint L1: `fp_w × fp_d × storeys >= 8 × (rooms_max + 10)`. Every row passes (minimum margin: Redaction `256 >= 256`).

Major-cycle weights `cycle_w` in the order lock_key, hidden_shortcut, gauntlet_return, two_key_hub, flooded_detour, collapsing_bridge, patrol_loop, foreshadowed_vault (§7.2):

MF 20 10 10 15 10 10 10 15; GH 15 20 10 10 5 10 15 15; CH 10 25 5 10 20 10 10 10; SO 25 10 10 20 5 5 20 5; CP 20 15 5 15 5 5 25 10; PA 10 15 15 5 15 25 10 5; WW 15 10 15 5 5 25 15 10; HN 15 20 5 15 10 5 25 5; WX 10 15 20 10 15 20 5 5; SP 20 10 10 20 10 15 10 5; FT 15 20 10 10 15 10 5 15; MA 15 10 25 10 10 10 5 15; ON 10 25 10 10 15 15 5 10; RD 25 10 5 25 0 0 30 5; UW 30 20 0 0 0 30 20 0; UT 13 13 12 12 12 12 13 13.

### 3.4 Layout styles (`gpv.layout.*`)

`fp_weights` pick room footprints in cells for kinds that allow a range (§15): `[1×1, 1×2, 2×2, 2×3, 3×3]`. `two_storey_bp` is the chance a room whose kind allows it spans 2 storeys. `open_bp` is the share of rooms rendered without a roof (presentation and weather only; `light_sky` is unaffected, §2.2). `start_storey` is the Start room's storey, clamped to `storeys - 1`.

| Layout | Used by | fp_weights | two_storey_bp | open_bp | start_storey |
|---|---|---|---|---|---|
| `gpv.layout.keep` | MF | 20 25 35 15 5 | 1500 | 1000 | 0 |
| `gpv.layout.manor` | GH | 25 30 30 10 5 | 1500 | 500 | 0 |
| `gpv.layout.labyrinth` | CH | 30 40 20 10 0 | 1000 | 0 | 0 |
| `gpv.layout.ship` | SO | 20 35 30 15 0 | 1000 | 0 | 1 |
| `gpv.layout.arcology` | CP | 20 30 30 15 5 | 2500 | 2500 | 0 |
| `gpv.layout.ruins` | PA | 15 25 35 15 10 | 1000 | 6000 | 0 |
| `gpv.layout.frontier` | WW | 20 25 35 15 5 | 500 | 7000 | 1 |
| `gpv.layout.streets` | HN | 25 30 30 10 5 | 2000 | 5000 | 0 |
| `gpv.layout.terraces` | WX | 20 25 30 15 10 | 2500 | 6000 | 0 |
| `gpv.layout.foundry` | SP | 15 25 35 15 10 | 2500 | 2000 | 0 |
| `gpv.layout.woodland` | FT | 15 25 35 15 10 | 1500 | 8000 | 0 |
| `gpv.layout.temple` | MA | 15 20 35 20 10 | 2000 | 4000 | 0 |
| `gpv.layout.dream` | ON | 20 25 30 15 10 | 3000 | 5000 | 1 |
| `gpv.layout.office` | RD | 40 40 20 0 0 | 0 | 0 | 0 |
| `gpv.layout.blank` | UW | 50 40 10 0 0 | 0 | 10000 | 0 |
| `gpv.layout.palimpsest` | UT | 15 25 35 15 10 | 2000 | 3000 | 0 |

### 3.5 Block D: population, loot, verbs, events and bleed

Verb weights are Bp with 10000 as neutral. `hack` is 0 wherever TEC ≤ 2, and every verb tied to an axis is 0 where that axis is 0 (Canon §5.3), except `cast`, which follows `aether_axiom`.

| # | density_bp | aspect_w (surg, unst, ward) | loot_bp | event_bp | drift_bp | bleed_in_bp | shoot | melee | cast | pray | sneak | talk | hack | craft | build | trav |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 MF | 10000 | 30 30 40 | 10000 | 2500 | 2500 | 3000 | 6000 | 14000 | 15000 | 11000 | 9000 | 10000 | 0 | 9000 | 8000 | 9000 |
| 2 GH | 9000 | 40 20 40 | 10000 | 4000 | 2500 | 3000 | 7000 | 11000 | 9000 | 15000 | 14000 | 9000 | 3000 | 8000 | 6000 | 8000 |
| 3 CH | 8000 | 30 40 30 | 9000 | 4500 | 3500 | 5000 | 8000 | 7000 | 11000 | 12000 | 15000 | 9000 | 6000 | 9000 | 6000 | 9000 |
| 4 SO | 11000 | 20 20 60 | 10000 | 3000 | 2000 | 3000 | 15000 | 9000 | 9000 | 5000 | 7000 | 10000 | 13000 | 9000 | 9000 | 11000 |
| 5 CP | 10500 | 50 10 40 | 10000 | 3000 | 2500 | 3000 | 13000 | 9000 | 5000 | 3000 | 13000 | 10000 | 16000 | 10000 | 7000 | 11000 |
| 6 PA | 10000 | 20 60 20 | 12000 | 3000 | 3000 | 3000 | 12000 | 11000 | 6000 | 7000 | 10000 | 8000 | 8000 | 15000 | 15000 | 10000 |
| 7 WW | 10000 | 40 40 20 | 10000 | 3000 | 2500 | 3000 | 15000 | 10000 | 8000 | 10000 | 8000 | 10000 | 4000 | 9000 | 8000 | 11000 |
| 8 HN | 9000 | 30 50 20 | 9000 | 3500 | 2000 | 3000 | 12000 | 9000 | 6000 | 3000 | 14000 | 16000 | 7000 | 7000 | 5000 | 9000 |
| 9 WX | 10500 | 50 40 10 | 10000 | 2500 | 2500 | 3000 | 3000 | 16000 | 12000 | 8000 | 10000 | 9000 | 0 | 8000 | 6000 | 16000 |
| 10 SP | 10500 | 20 40 40 | 11000 | 3000 | 2500 | 3000 | 13000 | 9000 | 7000 | 6000 | 8000 | 10000 | 9000 | 15000 | 13000 | 11000 |
| 11 FT | 9500 | 40 20 40 | 10000 | 3500 | 3000 | 4000 | 4000 | 10000 | 16000 | 12000 | 10000 | 15000 | 0 | 10000 | 8000 | 9000 |
| 12 MA | 11000 | 30 50 20 | 10500 | 3000 | 2500 | 3000 | 5000 | 15000 | 10000 | 16000 | 7000 | 10000 | 0 | 8000 | 7000 | 12000 |
| 13 ON | 9500 | 40 30 30 | 11000 | 5000 | 10000 | 6000 | 8000 | 9000 | 15000 | 11000 | 9000 | 9000 | 6000 | 8000 | 10000 | 16000 |
| 14 RD | 9000 | 20 40 40 | 9000 | 4000 | 0 | 0 | 9000 | 7000 | 5000 | 8000 | 16000 | 12000 | 15000 | 8000 | 7000 | 8000 |
| 15 UW | 6000 | 34 33 33 | 8000 | 2000 | 0 | 0 | 6000 | 14000 | 0 | 0 | 12000 | 0 | 0 | 10000 | 14000 | 12000 |
| 16 UT | 12000 | 34 33 33 | 12000 | 4000 | 0 | 0 | 10000 | 10000 | 10000 | 10000 | 10000 | 10000 | 10000 | 10000 | 10000 | 10000 |

Liquid weights (`liquids`): MF water 70, oil 20, lava 10. GH water 40, blood 50, alcohol 10. CH water 60, acid 20, blood 20. SO water 30, acid 40, oil 30. CP water 40, acid 30, oil 30. PA water 30, acid 40, oil 30. WW water 50, oil 30, alcohol 20. HN water 60, alcohol 25, oil 15. WX water 80, alcohol 20. SP water 40, oil 50, lava 10. FT water 70, alcohol 20, blood 10. MA water 50, lava 30, oil 20. ON water 50, alcohol 30, acid 20. RD water 20, red_ink 60, acid 20. UW red_ink 100. UT water 40, red_ink 30, lava 30. IDs are `liq.<name>` (doc 08).

Story domains (§19): MF flame, oath, star, wyrm. GH blood, moon, crypt, mirror. CH deep, star, madness, tide. SO star, empire, void, machine. CP chrome, neon, data, corporation. PA rust, ash, water, radiation. WW dust, gold, hex, iron. HN rain, money, smoke, lie. WX jade, sword, river, mountain. SP steam, brass, clock, sky. FT thorn, glass, wolf, bargain. MA bronze, oracle, sea, hubris. ON sleep, mirror, flight, door. RD form, seal, silence, file. UW blank, blank, blank, blank. UT word, word, word, word.

Hazard and event lists are §17.2 and §18.2. Secret-exit lists are §16.4. Room bias is §15.2.

### 3.6 Structural laws (`gpv.law.*`)

Laws here change layout or population only. Combat and casting laws belong to docs 05 and 06.

| ID | Genres | Exact rule |
|---|---|---|
| `gpv.law.non_euclidean` | CH | 2 relink door pairs per Folio; each pair swaps destinations every 3600 ticks from Folio entry (§8.4) |
| `gpv.law.dream_doors` | ON | 1 relink door pair per Folio; period 1800 ticks |
| `gpv.law.gravity_flip` | ON | 2..=4 gravity-flip volumes (§8.5) |
| `gpv.law.no_bleed` | RD, UW, UT | the Folio has no bleed zones and never bleeds into other Folios |
| `gpv.law.editors_only` | UW | population roster is `fac.editors` only; no shop rooms; no Lucre containers (Canon §5.2) |
| `gpv.law.pure_reading` | UT | records the Canon §5.5 override `C = 0`, `E = 0`; worldgen places no Contradiction-dependent hazards |

### 3.7 Example GPV record

```json
{
  "id": "genre.gothic_horror",
  "tier": 0,
  "arc": 5, "tec": 3, "myt": 9, "vrv": 3,
  "aether_axiom": "ax.mythos",
  "gravity_bp": 10000, "ambient_light": 2,
  "family": 1, "era_tl": 5,
  "spawn_block_max": 5, "fixture_level": 11, "fog": 1,
  "fire_bp": 12000, "flow_bp": 9000, "sound_bp": 8000, "hard_bp": 10000, "proj_bp": 10000,
  "palette": [1776418, 3812921, 9109504, 13219740],
  "render_style": 1,
  "materials": [{"mat": "mat.slate", "w": 40}, {"mat": "mat.ebony", "w": 30}, {"mat": "mat.iron", "w": 20}, {"mat": "mat.silver", "w": 10}],
  "manufacturers": ["mfr.gothic_horror.hallowmere", "mfr.gothic_horror.vesper_works"],
  "patron": "patron.gothic_horror",
  "layout": "gpv.layout.manor", "fp_w": 9, "fp_d": 9, "storeys": 3,
  "rooms_min": 15, "rooms_max": 19, "loop_bp": 2000, "vert_bp": 2500, "corridor_w": 3,
  "cover_bp": 2500, "bright_bp": 2000, "dark_bp": 5000, "secret_bp": 2000, "trap_bp": 2000,
  "liquids": [{"liq": "liq.water", "w": 40}, {"liq": "liq.blood", "w": 50}, {"liq": "liq.alcohol", "w": 10}],
  "tilesets": ["biome.gothic_horror.manor", "biome.gothic_horror.catacomb"],
  "cycle_w": [15, 20, 10, 10, 5, 10, 15, 15],
  "room_bias": [{"room": "room.crypt", "bp": 20000}, {"room": "room.prison", "bp": 12000}, {"room": "room.garden", "bp": 5000}],
  "density_bp": 9000, "aspect_w": [40, 20, 40], "loot_bp": 10000,
  "verbs": [7000, 11000, 9000, 15000, 14000, 9000, 3000, 8000, 6000, 8000],
  "hazards": [{"hazard": "hazard.spike_pit", "w": 20}, {"hazard": "hazard.curse_glyph", "w": 20}, {"hazard": "hazard.blade_trap", "w": 15}, {"hazard": "hazard.falling_debris", "w": 15}, {"hazard": "hazard.miasma_bog", "w": 15}, {"hazard": "hazard.alarm", "w": 15}],
  "events": [{"evt": "gpv.evt.light_failure", "w": 25}, {"evt": "gpv.evt.whispers", "w": 20}, {"evt": "gpv.evt.ambush", "w": 15}, {"evt": "gpv.evt.weather_front", "w": 15}, {"evt": "gpv.evt.plea", "w": 15}, {"evt": "gpv.evt.lore_echo", "w": 10}],
  "event_bp": 4000, "drift_bp": 2500, "bleed_in_bp": 3000,
  "secret_exits": [{"genre": "genre.oneiric", "bp": 1500}],
  "laws": [],
  "story_domains": ["blood", "moon", "crypt", "mirror"]
}
```

Enum encodings: `tier` core 0, secret 1; `family` fantasy 0, horror 1, sf 2, historical 3, crime 4, mythic 5, meta 6; `render_style` pbr 0, painterly 1, toon 2, ink 3, pixel_dither 4, xerox_collage 5; `palette` entries are `0xRRGGBB` as decimal integers. The validator rejects any record whose Block A disagrees with Canon §5.2 (lint L2).

Lints run at content load and fail the build:

- **L1** footprint capacity (§3.3).
- **L2** Block A equals Canon §5.2 field for field.
- **L3** `hack = 0` where `tec <= 2`; `pray = 0` where `myt = 0`; `cast = 0` where `aether_axiom = none` and `arc = 0`.
- **L4** every hazard and event listed is eligible in that genre (§17.1 `dep` and electronic flag, §18.1 `requires`).
- **L5** `protect_r >= 4` for every genre with fixtures, so one fixture always protects a 3 × 3 doorway landing.
- **L6** `dark_bp = 0` where `ambient_light >= 5`; `bright_bp + dark_bp <= 10000`.

## 4. Genre Lattice

### 4.1 Nodes

The Lattice has 16 genre nodes (Canon §5.2 order, which is also the **lattice order** used for every sorted list below) plus one virtual source `lattice.node.margin` used only for `folio_index = 0`. Core nodes are offered by exits. Secret nodes are reached only through secret exits (§16.4) and the fixed Folio 25 (Canon §17.1).

### 4.2 Core edge weights

Weights are `u32`, symmetric, and every core pair has weight at least 10, so the core graph is complete and an exit draw never runs out of options. Abbreviations follow §3.3.

| from \ to | MF | GH | CH | SO | CP | PA | WW | HN | WX | SP | FT | MA | row sum |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| MF | — | 60 | 20 | 20 | 10 | 20 | 30 | 10 | 60 | 40 | 90 | 90 | 450 |
| GH | 60 | — | 90 | 10 | 20 | 30 | 60 | 50 | 20 | 70 | 60 | 30 | 500 |
| CH | 20 | 90 | — | 50 | 40 | 50 | 40 | 70 | 10 | 40 | 20 | 40 | 470 |
| SO | 20 | 10 | 50 | — | 80 | 40 | 30 | 20 | 30 | 50 | 10 | 30 | 370 |
| CP | 10 | 20 | 40 | 80 | — | 70 | 20 | 90 | 40 | 40 | 10 | 10 | 430 |
| PA | 20 | 30 | 50 | 40 | 70 | — | 80 | 30 | 10 | 50 | 20 | 20 | 420 |
| WW | 30 | 60 | 40 | 30 | 20 | 80 | — | 50 | 30 | 60 | 30 | 30 | 460 |
| HN | 10 | 50 | 70 | 20 | 90 | 30 | 50 | — | 20 | 40 | 10 | 10 | 400 |
| WX | 60 | 20 | 10 | 30 | 40 | 10 | 30 | 20 | — | 20 | 50 | 60 | 350 |
| SP | 40 | 70 | 40 | 50 | 40 | 50 | 60 | 40 | 20 | — | 30 | 20 | 460 |
| FT | 90 | 60 | 20 | 10 | 10 | 20 | 30 | 10 | 50 | 30 | — | 50 | 380 |
| MA | 90 | 30 | 40 | 30 | 10 | 20 | 30 | 10 | 60 | 20 | 50 | — | 390 |

The stationary distribution of the raw edge random walk ranges from 6.9% (Wuxia) to 9.8% (Gothic Horror); the freshness factor (§4.4) flattens it further.

### 4.3 Secret and virtual edges

- **Margin → core:** weight 100 to each of the 12 core genres.
- **Core → secret:** secret-exit chances in Bp (§16.4): Oneiric from FT 2500, MF 2000, CH 2000, WX 2000, GH 1500, MA 1500. Redaction from CP 2500, HN 2500, SO 2000, SP 1500, PA 1500, CH 1500. Unwritten from every core genre 1000. Ur-Text from every core genre 800 (Marginalia only).
- **Secret → core return** weights (exits out of a secret Folio): Oneiric → FT 90, MF 70, CH 70, WX 60, GH 50, MA 50, every other core 10. Redaction → CP 90, HN 90, SO 70, SP 50, PA 50, CH 50, every other core 10. Unwritten → every core 50. Ur-Text → every core 50.

One record per nonzero directed edge:

```json
{ "id": "lattice.edge.cyberpunk.hardboiled_noir", "from": "genre.cyberpunk", "to": "genre.hardboiled_noir", "w": 90 }
```

### 4.4 Volume and run rules

`m` is the Folio being chosen; `history[i]` is Folio `i`'s genre.

1. **Volumes:** Standard `ceil_div(m, 5)`, Abridged `ceil_div(m, 3)`, Marginalia blocks `floor_div(m - 26, 5)` for `m >= 26`.
2. **No repeat in a Volume** or Marginalia block (Canon §17.1).
3. **Run cap** for `m <= 25`: exclude core genres already present 3 times. Secret genres: once per run before Marginalia (§16.4).
4. **Never the current genre.**
5. **Freshness** `fresh_bp`: main run by count in `history`: 0 → 15000, 1 → 10000, 2 → 5000, 3+ → 2500; Marginalia by count in the last 10 entries: 0 → 15000, 1 → 10000, 2+ → 5000.
6. **Effective weight** `w(g) = edge(cur, g) × fresh_bp(g)` (`u32`, at most 1,500,000). For a secret `cur`, `edge` reads the §4.3 return weights; for the Margin node it is 100.
7. **Forced:** Folio 25 (Abridged 15) is `genre.ur_text`. Boss Folios otherwise draw normally.

**Feasibility.** Before Folio 25, rules 2 and 3 exclude at most 9 genres together (8 capped genres need 24 appearances in at most 23 Folios; with 7 capped, only 2 earlier slots are uncapped), so at least 3 stay eligible; Abridged and Marginalia exclude at most 4. The implementation asserts `eligible.len() >= 3`.

### 4.5 Exit-offer algorithm

The draw happens on the tick the exit UI opens (Canon §17.1): the first tick on which a living Erratum interacts with the Threshold lectern (§15.1). Exactly 3 candidates are drawn by `rng.lattice` for the exiting Folio, always, including the exits of Folio 24 (Standard) and Folio 14 (Abridged), whose candidates are drawn, stored and not shown because the next Folio is forced.

```rust
/// Pure: same inputs, same output. `n` = exiting Folio index (0 for run start).
fn lattice_candidates(run_seed: u64, n: u32, cur: Node, history: &[GenreId], mode: Mode) -> [GenreId; 3] {
    let mut rng = Rng::from_seed64(stream_seed(run_seed, "rng.lattice", n as u64));
    let m = n + 1;
    let elig: Vec<GenreId> = CORE_IN_LATTICE_ORDER.iter()
        .filter(|g| Node::Genre(**g) != cur)
        .filter(|g| !same_volume_genres(m, history, mode).contains(g))
        .filter(|g| m >= 26 || count(history, **g) < 3)
        .copied().collect();
    assert!(elig.len() >= 3);
    let mut w: Vec<u32> = elig.iter().map(|g| edge(cur, *g) * fresh_bp(*g, m, history)).collect();
    let mut out = [GenreId::NONE; 3];
    for k in 0..3 {                       // exactly 3 calls of Rng::weighted = 3 range() calls
        let i = rng.weighted(&w);
        out[k] = elig[i];
        w[i] = 0;                         // without replacement; indices stay stable
    }
    out
}
```

The first `EXIT_OFFERS` candidates are shown (Canon §17.1); if the ITU-or-Lens condition becomes true while the UI is open, the third door lights without a redraw. Candidates are saved with the lattice history (Canon §2.5).

**Worked example.** Run seed `0x0123456789ABCDEF`, host Origin Cyberpunk, so Folio 1 is Cyberpunk. At its exit `stream_seed(run, "rng.lattice", 1) = 0xb93f5f6ccba93fac`; all 11 other core genres are eligible with freshness 15000, so weights are `edge(CP, g) × 15000`: MF 150,000; GH 300,000; CH 600,000; SO 1,200,000; PA 1,050,000; WW 300,000; HN 1,350,000; WX 600,000; SP 600,000; FT 150,000; MA 150,000; total 6,450,000.

| Draw | Total | `range(total)` | Cumulative weight before the pick | Pick |
|---|---|---|---|---|
| 1 | 6,450,000 | 1,106,450 | 1,050,000 (MF + GH + CH) | Space Opera |
| 2 | 5,250,000 | 3,525,875 | 2,400,000 | Hardboiled Noir |
| 3 | 3,900,000 | 3,019,215 | 3,000,000 | Steampunk |

With `EXIT_OFFERS = 2` the doors show Space Opera and Hardboiled Noir.

### 4.6 Folio 1

- Standard mode: the host Erratum's Origin genre when it is a core genre (Canon §17.1). When the host's Origin is a secret genre, Folio 1 is the core genre with the highest return weight from that secret node, ties to the earlier lattice order: Oneiric → Fairy Tale, Redaction → Cyberpunk, Unwritten → Mythic Fantasy, Ur-Text → Mythic Fantasy (Canon gaps G4).
- Seeded modes (Daily, Weekly, Seeded): the first candidate of `lattice_candidates(run_seed, 0, lattice.node.margin, [], mode)`. Every Margin edge is 100 and every freshness is 15000, so the draw is uniform. For run seed `0x0123456789ABCDEF` the three candidates are Mythic Fantasy, Hardboiled Noir, Steampunk, so Folio 1 is Mythic Fantasy. For the Daily seed of 2026-10-11 (`0xc6f2e12ddca712ab`) they are Weird West, Cosmic Horror, Mythic Fantasy, so Folio 1 is Weird West.

### 4.7 Exit preview

Each lit door shows values from **pure plan functions** of the candidate Folio; they recompute seeds, never advance a live stream, and match what Folio `n + 1`'s generation computes (§6.2).

| Field | Source | Shown as |
|---|---|---|
| genre, display name, Lucre and Aether skin | Canon §5.2 | door art and title |
| axioms and delta | Canon §5.2; delta = candidate − current per axis | four glyphs with ▲/▼ and the signed difference |
| Contradiction forecast | Canon §5.5 evaluated client-side with the candidate's axioms for each of the viewer's equipped components | "N items Strained, M Contradicted, K Disconnected" |
| Folio type and challenge kind | `folio_type()` (§5.1) | icon |
| champion aspects | `champion_aspects()` (§4.8) | 1–3 aspect icons, each with its counter icon (Canon §17.3a) |
| bleed | `bleed_plan()` (§13.1): entry genre and drift genres | small genre sigils |
| Marginalia modifiers | `marg_mods()` (§21) | modifier icons |
| depth | `d`, item level `ilvl = d` | numerals |
| hazard family | the 2 highest-weight hazards of the candidate's GPV, ties to earlier list order | icons |
| boss marker | Folio is a boss Folio (§5.1) | crown icon; boss identity per doc 13 |

Secret exits show no preview: their door reads "?" until crossed (§16.4).

### 4.8 Champion aspect plan

```rust
fn champion_aspects(run_seed: u64, n: u32, genre: GenreId, d: i32) -> SmallVec<[EliteId; 3]> {
    let k = if d <= 10 { 1 } else if d <= 30 { 2 } else { 3 };
    let mut r = stage_rng(run_seed, n, genre, "aspects", 0);           // §6.2
    let ids = [ELITE_SURGING, ELITE_UNSTOPPABLE, ELITE_WARDED];        // ascending ID order
    let mut w = gpv(genre).aspect_w;                                     // 3 × u16
    (0..k).map(|_| { let i = r.weighted(&w); w[i] = 0; ids[i] }).collect()  // k weighted calls
}
```

Every `rank.champion` in that Folio carries one aspect of this set (I5). In the §4.5 example, Folio 2 as Space Opera has `{elite.surging}`, as Hardboiled Noir `{elite.unstoppable}`.

### 4.9 Choosing a door

A lit door is chosen on the first tick on which every living, non-downed, non-ghost Erratum stands in its 3 × 3 × 3-block trigger volume, or one Erratum has held interact on it for 180 ticks with every living Erratum inside the Threshold room. The host emits `ev.folio_exited`, appends the genre to the history and generates the next Folio; ghosts, Remains and minions follow Canon §17.6 and §15.1.

## 5. Folio types

### 5.1 Assignment

Four types exist: `lattice.folio.standard`, `lattice.folio.boss`, `lattice.folio.rest`, `lattice.folio.challenge`.

```rust
fn is_boss(n: u32, mode: Mode) -> bool {
    match mode { Mode::Abridged => n % 3 == 0, _ => if n <= 25 { n % 5 == 0 } else { (n - 25) % 5 == 0 } }
}
fn folio_type(run_seed: u64, n: u32, genre: GenreId, mode: Mode) -> FolioType {
    if is_boss(n, mode) { return Boss; }
    if n == 1 || genre == UNWRITTEN { return Standard; }
    let mut r = stage_rng(run_seed, n, genre, "ftype", 0);
    let w: [u32; 3] = match mode {                       // [standard, challenge, rest]
        Mode::Abridged => if (n - 1) % 3 == 0 { return Standard } else { [8500, 1500, 0] },
        _ if n >= 26   => [7000, 2000, 1000],
        _ if n % 5 == 4 => [4000, 0, 6000],              // the Folio before each boss
        _              => [8000, 1500, 500],
    };
    let t = [Standard, Challenge, Rest][r.weighted(&w)];  // exactly 1 weighted call
    if genre == REDACTED && t == Rest { Standard } else { t }
}
```

Worked example: for the §4.5 run at `n = 2`, Space Opera draws `range(10000) = 3860 < 8000`, so standard; Hardboiled Noir draws 9855 ≥ 9500, so rest. The Noir door therefore previews a rest Folio.

### 5.2 Composition per type

| Type | Mission-graph rooms | Required rooms | Population (`ftype_pop_bp`) | Loot target LP | Lock-key pairs | Secrets | Duration target |
|---|---|---|---|---|---|---|---|
| standard | `rooms_min..=rooms_max` | entry, threshold, shop (not UW), Scriptorium on odd `n` (50% on even `n`) | 10000 | 24 | 1, plus 1 when the target ≥ 18 | §16.1 | 180–420 s |
| challenge | 10..=14 | entry, threshold, shop, challenge rooms (§5.3) | 12500 | 30 | 1 | §16.1 | 180–300 s |
| rest | 6..=8 | entry, threshold, campfire, shop, Scriptorium, Fortune Engine hall | 0 | 8 | 0 | 1 | 90–180 s |
| boss | 4..=6 approach | entry, shop, Editor checkpoint, boss arena, threshold | 6000 (approach rooms only) | 12 plus boss drops (doc 11) | 0 | 1 | 240–420 s |

Rest Folios set `no_ambient_spawns` (doc 13) and tag every room `room.safe`; Erasure still accrues.

### 5.3 Challenge kinds

One kind per challenge Folio, drawn in stage `"ftype"` immediately after the type draw with weights below (1 more weighted call, only when the type is challenge).

| ID | Weight | Rule | Reward on success |
|---|---|---|---|
| `lattice.challenge.gauntlet` | 25 | the main path holds 3 `room.gauntlet` rooms; success = reach the Threshold with no Erratum downed | +1 Reliquary (doc 11) |
| `lattice.challenge.siege` | 20 | a `room.arena` on the main path seals for 7200 ticks while doc 13 spawns waves totalling 12000 mSKE; success = survive | +1 Reliquary, +1 rarity step on the arena chest |
| `lattice.challenge.heist` | 20 | a `room.treasury` behind a lock; success = open it with no hostile ever at `aware.alert` toward an Erratum | +1 Reliquary, treasury LP × 2 |
| `lattice.challenge.duel` | 15 | one champion with 2 aspects guards the Threshold; success = kill it | +1 Reliquary |
| `lattice.challenge.race` | 20 | success = exit within 10800 ticks of entry | +1 Reliquary |

Success is judged on `ev.folio_exited`; failure forfeits only the reward.

## 6. Generation pipeline

### 6.1 Inputs and outputs

`generate_folio(input: &GenInput, db: &ContentDb, par: &dyn OrderedMap) -> FolioBlueprint` (doc 02) is pure. It reads nothing else: no party size, no account state, no clock.

```rust
pub struct GenInput {
    pub run_seed: u64,
    pub folio_index: u32,          // stream index: 1.. (Abridged 1..=15); EntityId prefix (Canon §1.11)
    pub d: i32,                    // effective depth: folio_index, or d_eff in Abridged (Canon §17.1)
    pub mode: Id32,                // mode.* (doc 14)
    pub genre: Id32,
    pub prev_genre: Option<Id32>,  // genre of folio_index - 1; None for Folio 1
    pub loot_quota: LootQuota,     // doc 11, pure in (run_seed, folio_index, mode): emendations, reliquaries
}
pub struct FolioBlueprint {
    pub header: Header,            // sim_version, content_hash, run_seed, folio_index, d, genre, prev_genre, attempt, blueprint_hash
    pub plan: FolioPlan,           // ftype, challenge, aspects, bleed_entry, bleed_drift, marg_mods (§4.7)
    pub zone_of_cell: [u8; 1536],  // 16 × 6 × 16 cells; index into `zones`; 0 = Folio genre
    pub zones: SmallVec<[Id32; 4]>,
    pub gflip_cells: [u64; 24],    // bitset over the 1536 cells (Oneiric)
    pub graph: MissionGraph, pub rooms: Vec<RoomRec>, pub doors: Vec<DoorRec>,
    pub chunks: Vec<ChunkBlob>,    // 128 chunks (8 × 2 × 8), ascending chunk index, palette + RLE
    pub fixtures: Vec<FixtureRec>, pub liquids: Vec<LiquidRec>, pub hazards: Vec<HazardRec>,
    pub spawners: Vec<SpawnerRec>, pub containers: Vec<ContainerRec>, pub lore: Vec<LoreRec>,
    pub secrets: Vec<SecretRec>,   pub story: StoryFragment,  pub flags: Flags,
}
```

`blueprint_hash` = BLAKE3 of the canonical serialization of every other field; every co-op peer generates locally and the host compares hashes (doc 16). At `ev.folio_entered` entity records are instantiated, ascending index within each list, in the order: locked doors, hazards, containers, lore objects, secret portals; then doc 13 spawns from `spawners` (`rng.spawn`). EntityIds follow that order.

### 6.2 Seeds and sub-streams

All worldgen randomness derives from `rng.worldgen` through independent sub-streams, so stages, rooms and chunks may run in any order or in parallel with identical bytes.

```rust
fn derive(parent: u64, salt: u64) -> u64 { mix64(parent ^ mix64(salt)) }
fn gen_seed(run: u64, n: u32, genre: Id32) -> u64 {
    derive(stream_seed(run, "rng.worldgen", n as u64), genre.0 as u64)      // genre.0 = fnv1a32 id
}
fn stage_rng(run: u64, n: u32, genre: Id32, stage: &str, attempt: u8) -> Rng {
    Rng::from_seed64(derive(gen_seed(run, n, genre), fnv1a64(stage) ^ attempt as u64))
}
fn item_rng(run: u64, n: u32, genre: Id32, stage: &str, i: u32, attempt: u8) -> Rng {
    Rng::from_seed64(derive(derive(gen_seed(run, n, genre), fnv1a64(stage) ^ attempt as u64), i as u64 + 1))
}
```

Example: `gen_seed(0x0123456789ABCDEF, 2, genre.space_opera) = 0x1c35d41d581f5eaa`. Plan stages (`ftype`, `aspects`, `bleed`, `marg`) always use `attempt = 0`, so previews match; structural stages use the current attempt (§14.3).

### 6.3 Stage order, draws and budgets

| # | Stage (sub-stream) | Work | Draws | Budget p50 / p99 (Steam Deck, 4 threads) |
|---|---|---|---|---|
| 0 | `ftype`, `aspects`, `bleed`, `marg` | §5.1, §4.8, §13.1, §21 | 1–2; k; 1–3; k | < 0.1 ms |
| 1 | `graph` | mission graph (§7) | per rule, §7.3 | 1 / 3 ms |
| 2 | `typing` | room kinds and footprints (§7.4) | 1–3 per node | 0.5 / 1 ms |
| 3 | `zones` | bleed zones (§13.2) | 1 + cluster | < 0.5 ms |
| 4 | `embed` | cells, doors, corridors, relinks, gravity flips (§8) | 1–2 per node | 4 / 12 ms |
| 5 | `wfc` per room (`item_rng`, i = room index) | room fill (§9) | 1 per collapse | 45 / 110 ms |
| 6 | `liquid`, `hazard` | §10.3, §10.4 | per placement | 2 / 5 ms |
| 7 | `dress` per room | props, containers sockets (§10.1) | 1 shuffle per room | 4 / 8 ms |
| 8 | `light` per room | fixtures (§10.2) | 1 shuffle per room | 12 / 30 ms |
| 9 | `pop`, `loot`, `secret`, `story` | §11, §12, §16, §19 | per record | 2 / 4 ms |
| 10 | rasterize (`par.map` over 128 chunks) | blocks into `ChunkBlob`s; bleed blending (§13.3) | 0 (hash dither) | 20 / 45 ms |
| 11 | validate and repair (`repair`) | §14 | per repair | 15 / 40 ms |
| — | total, one attempt | | | **≤ 110 / 260 ms** |
| — | total, worst case with 3 retries and fallback | | | ≤ 900 ms hard cap (CI fails above) |

Generation runs inside the Folio transition (at least 90 ticks, doc 17), which waits for it.

## 7. Mission graph grammar

### 7.1 Node and edge kinds

Nodes, created in order (`NodeId` = creation index): `Entry`, `Threshold`, `Room`, `Hub`, `Key(k)`, `Service(kind)`, `Arena`, `Gauntlet`, `Treasury`, `Puzzle(p)`, `Vault`, `Campfire`, `Checkpoint`, `BossArena`, `Secret`, `SecretExit(genre)`. Edges: `Open`; `Locked(keys)` (1 or 2 keys); `OneWay(from, to)` (initially passable only from→to; realized as a door barred on the `to` side that the `from` side unbars, or as a drop of at least 4 blocks); `Hidden` (secret, §16.3); `PuzzleGate(p)` (opens when puzzle `p` is solved). The **main path** is the shortest Entry→Threshold path by edge count over Open, Locked, OneWay and PuzzleGate edges, breadth-first, visiting neighbors in ascending `NodeId`.

### 7.2 Rules

| ID | Phase | Rewrite (in = Entry, out = Threshold) | Selection |
|---|---|---|---|
| `room.rule.axiom` | A | `Entry –Open– Room –Open– Threshold` | none |
| `room.rule.cycle_lock_key` | B | replace the middle Room: `in–a`, `a–K1`, `K1→in` OneWay, `in–m` Locked(k1), `m–out` | `cycle_w` |
| `room.rule.cycle_hidden_shortcut` | B | `in–a–b–c–out`; `in–c` Hidden | `cycle_w` |
| `room.rule.cycle_gauntlet_return` | B | `in–Gauntlet–Treasury`, `Treasury→in` OneWay; `in–a–out` | `cycle_w` |
| `room.rule.cycle_two_key_hub` | B | `in–Hub`; `Hub–a–K1`, `K1→Hub` OneWay; `Hub–b–K2`, `K2→Hub` OneWay; `Hub–out` Locked(k1, k2) | `cycle_w` |
| `room.rule.cycle_flooded_detour` | B | `in–w–out` with `w` flooded (§10.3); `in–a–b–out` | `cycle_w` |
| `room.rule.cycle_collapsing_bridge` | B | `in–c`, `c→out` OneWay (bridge collapses after the first crossing); `in–a–b–out` | `cycle_w` |
| `room.rule.cycle_patrol_loop` | B | `in–a–b–out`; `in–c–d–out`; spawner role `patrol` on the loop | `cycle_w` |
| `room.rule.cycle_foreshadowed_vault` | B | `in–w–out`; `w–Treasury` Locked(k1) with a window tile on the shared face; `in–a–K1`, `K1→w` OneWay | `cycle_w` |
| `room.rule.extend` | C, weight 50 | split an Open edge `u–v` into `u–n–v` | `range(#eligible edges)` |
| `room.rule.branch` | C, weight 30 | add dead end `u–n` to a Room or Hub with degree < 4 | `range(#eligible nodes)` |
| `room.rule.minor_cycle` | C, weight 20 while loops remain | add `u–n–v` for Rooms `u < v`, degree < 4 each, hop distance 2..=4, not adjacent | `range(#eligible pairs)` |
| `room.rule.lock_key` | D1 | lock an Open main-path edge not touching Entry; key on a Room dead end reachable without that edge, else on a new branch | 2 `range` calls |
| `room.rule.service` | D2 | attach `Service(kind)` to the main-path node at `floor_div(len × frac, 10000)`, clamped to `1..=len-2`; `frac`: shop 5000, Scriptorium 3000, Fortune Engine hall 7000, campfire 6000 | none |
| `room.rule.arena` | D3 | the main-path Room nearest position 6000 Bp (toward the Threshold on ties) becomes `Arena` | `roll_bp(3000)`; challenge siege always |
| `room.rule.puzzle_gate` | D4 | an Open main-path edge not touching Entry or Threshold becomes `PuzzleGate(p)`; `Puzzle(p)` branches from its Entry-side endpoint | `roll_bp(2500)`, then `range` |
| `room.rule.vault` | D5 | attach `Vault` to a non-main-path Room | `roll_bp(4000)` (10000 when `loot_quota.reliquaries > 0`), then `range` |
| `room.rule.gauntlet_branch` | D6 | attach `Gauntlet–Treasury` to a Room | `roll_bp(2000)`, then `range` |
| `room.rule.challenge` | D7 | applies the challenge kind's structure (§5.3) | none |
| `room.rule.secret` | D8 | attach `Secret` by a Hidden edge to a node other than Entry and Threshold | `range` per secret |
| `room.rule.secret_exit` | D9 | attach `SecretExit(g)` by a Hidden edge to the non-Threshold node farthest (hops) from Entry, ties to lower `NodeId` | none |
| `room.rule.boss_spine` | boss | `Entry–R1–…–Rk–Checkpoint–BossArena–Threshold`, `k = range_incl(4, 6)`; shop attached to `R2` | 1 `range` |
| `room.rule.rest_ring` | rest | `Entry–Campfire`; Campfire spokes to shop, Scriptorium, Fortune Engine hall, `j = range_incl(1, 3)` Rooms and the Threshold | 1 `range` |

### 7.3 Algorithm

```rust
fn build_graph(r: &mut Rng, g: &Gpv, plan: &FolioPlan, n_target: u32) -> MissionGraph {
    match plan.ftype { Boss => return boss_spine(r, g), Rest => return rest_ring(r, g), _ => {} }
    let mut G = axiom();                                         // 3 nodes
    apply_cycle(&mut G, CYCLES[r.weighted(&g.cycle_w)]);         // 1 weighted call
    let loops_max = floor_div(n_target * g.loop_bp, 10000);
    let n_grow = n_target - required_services(plan).len() as u32;
    let mut loops = 0;
    while G.non_secret_count() < n_grow {
        let w = [50, 30, if loops < loops_max { 20 } else { 0 }];
        let mut rule = r.weighted(&w);                            // 1 call
        if !eligible(&G, rule) { rule = EXTEND; }                 // extend is always eligible
        G.apply(rule, r.range(G.targets(rule).len() as u64));     // 1 call
        if rule == MINOR_CYCLE { loops += 1; }
    }
    features_d1_to_d9(&mut G, r, plan);                           // fixed order, draws per §7.2
    G
}
```

`n_target` is the first draw of stage `graph`: standard `range_incl(rooms_min, rooms_max)`, challenge `range_incl(10, 14)`. Lock edges in total (major cycle included): standard `1 + [n_target >= 18]`, challenge 1, rest and boss 0. A finished standard graph has `n_target..=n_target + 6` non-secret nodes.

Example: Mythic Fantasy, `n_target = 18`: `loops_max = floor_div(18 × 1500, 10000) = 2`; with shop and Scriptorium required (odd `n`), growth stops at 16 non-secret nodes; `lock_key` adds a second lock since `18 >= 18`.

### 7.4 Typing

Nodes in ascending `NodeId`: feature nodes map directly (`Service(shop)` → `room.shop`, `Key` → `room.key_room`, …; the flooded room → `room.cistern`, the bridge → `room.chasm`); each `Room` draws a kind with §15.2 weights, skipping kinds at their maximum (1 weighted call). Every node then draws a footprint from `fp_weights` within the kind's range (1 weighted call) and, when 2 storeys are allowed and available, `roll_bp(two_storey_bp)` (1 call).

## 8. Spatial embedding and corridors

### 8.1 Placement

1. **Entry:** storey `min(start_storey, storeys - 1)`, centered on the south edge: `cx = x0/16 + floor_div(W - fw, 2)`, `cz = z0/16 + D - fd`.
2. **Order:** breadth-first from Entry, neighbors in ascending `NodeId`; a node's parent is its first placed neighbor.
3. **Candidates** for `v` with parent `u`, sorted by `(cs, cz, cx, orientation)`: `H` = free boxes (either orientation) on `u`'s storeys sharing a full cell face with `u`; `V` = free boxes directly above or below `u` overlapping its projection; `C` = free boxes on `u`'s base storey at Chebyshev distance 2..=4 with an A* corridor route (§8.3).
4. **Choice:** the Threshold takes the `H ∪ V` (else `C`) candidate maximizing doubled-center `|Δcx| + |Δcz| + 2|Δcs|` from Entry, ties to the earlier index, no draw. Others: with `H` and `V` both non-empty, `V` if `roll_bp(vert_bp)` else `H`, then `range(len)`; with one non-empty, `range(len)` in it; else `range(len)` in `C`; `C` empty fails the attempt (§14.3).
5. **Non-tree edges:** a door when boxes share a face, else a corridor; no route fails the attempt.

### 8.2 Doors and connectors

| Edge | Realization |
|---|---|
| Open, horizontal | doorway 3 wide × 3 tall, centered on the shared face segment, genre door skin, openable both ways |
| Open, vertical | 3 × 3 floor opening with a slab stair run along the lower room's wall (rise 500 mm per block) |
| Locked(keys) | lock door (doc 09 `lock.*`) bound to the key items; the locked room's whole shell is `block.reinforced` (unbreakable) when the locked side is a dead end |
| OneWay | horizontal: door barred on the `to` side, unbarred by interacting from the `from` side, then open both ways; vertical: a 4-block drop with no stair |
| Hidden | secret passage per hiding method (§16.3) |
| PuzzleGate(p) | portcullis-class gate (skin per genre) that opens when puzzle `p` resolves (§15.3) |

The Threshold's shell is `block.reinforced` except its doorways. Every other wall is breakable: digging around a main-path lock is a legal sequence break in a destructible world.

### 8.3 Corridors and stairs

- Route: A* over free cells of the footprint between the two boxes' facing sides. Costs: 10 per horizontal cell step, 30 per storey change (a stair cell), +2 per turn. Ties break toward the lower cell index. Corridors never enter room cells.
- Carving: width `corridor_w` (3 or 5 blocks) centered in each cell, clear height 4 blocks (4000 mm ≥ the 2600 mm mount headroom, Canon §17.7).
- A stair cell climbs one storey (8 blocks) across its 16-block length with alternating slab and full steps, so each step rises 500 mm, under the 550 mm step-up (Canon §4.1).
- Corridor cells take the zone of the room at their lower-`NodeId` endpoint (§13.2).

### 8.4 Relink pairs (`gpv.law.non_euclidean`, `gpv.law.dream_doors`)

After placement, choose `p` pairs (2 in Cosmic Horror, 1 in Oneiric) of horizontal Open doors not on the main path and not belonging to safe rooms, by `range` over the sorted eligible list (1 call per door, without replacement). In state 0 each door leads to its own neighbor; in state 1 the two doors swap destinations through portal planes (doc 03 teleports the entity with its velocity rotated by the doors' yaw difference). State toggles at `folio_entry_tick + k × period` for `k >= 1`. Validation (§14) runs in both states.

### 8.5 Gravity-flip volumes (`gpv.law.gravity_flip`, Oneiric)

`f = range_incl(2, 4)` single-storey Rooms not on the main path's first or last two nodes, not safe, not in a bleed zone, are chosen by `range` without replacement. Their interior clear height is set to 4 blocks; their WFC fill is vertically mirrored so ceilings carry walkable surfaces. Every cell of a flipped room has gravity `+Y × GRAVITY_BASE × gravity_bp` (local gravity reversed; doc 03 flips the capsule). Entering through a doorway inverts the entity; the fall across 4 blocks stays under Oneiric `SAFE_FALL = 4000 × (10000 + 4 × 400) / 10000 = 4640` mm, so a flip never deals fall damage. Flipped cells are set in `gflip_cells`.

## 9. Room templates and Wave Function Collapse

### 9.1 Room templates

A room template `room.tpl.*` gives a kind its shell, pins and sockets in tile coordinates. A room's tile grid is `4 fw × 2 storeys × 4 fd` tiles (largest 12 × 4 × 12 = 576 tiles).

```json
{
  "id": "room.tpl.library_a",
  "kind": "room.library",
  "footprints": [[2, 2]], "storeys": [1, 2],
  "pins": [ {"tile": [0, 0, 0], "class": "shell_corner"}, {"tile": [3, 0, 0], "class": "shell_door"} ],
  "sockets": [
    {"kind": "light", "tile": [1, 0, 1], "block": [2, 3, 0], "mount": 2},
    {"kind": "container", "tile": [4, 0, 4], "block": [1, 0, 1], "lp": 1},
    {"kind": "lore", "tile": [2, 0, 5], "block": [2, 0, 2]},
    {"kind": "spawn", "tile": [5, 0, 2], "block": [2, 0, 2]}
  ],
  "fallback_fill": "room.tpl.library_a.fill0",
  "tags": ["room.no_mount"]
}
```

Shell classes pin every boundary tile: wall, corner, door (the door face from §8.2), window, floor-only for open-top rooms. The template also names an authored `fallback_fill`, a complete valid tile assignment used when WFC fails.

### 9.2 Tilesets

Each genre has a primary and a secondary tileset (`biome.<suffix>.<name>`, §3.3). The primary fills rooms; the secondary fills the kinds listed in its `kinds` field (for example `biome.mythic_fantasy.cavern` fills cisterns, chasms and crypts). A tileset has 40 to 160 tile prototypes. Each prototype has a 64-block payload of palette slots stored as run-length pairs `[slot, count, …]` summing to 64, 6 face sockets and flags.

```json
{
  "id": "biome.gothic_horror.manor",
  "palette": {"1": "block.wall_primary", "2": "block.floor_primary", "3": "block.trim", "4": "block.accent", "6": "block.light_fixture"},
  "tiles": [
    {"name": "floor_rug", "w": 120, "sym": "x", "faces": [3, 3, 0, 1, 3, 3], "flags": ["walkable"], "rle": [2, 16, 0, 48]},
    {"name": "bookcase", "w": 60, "sym": "l", "faces": [5, 3, 0, 1, 3, 3], "flags": ["cover"], "rle": [2, 16, 4, 4, 0, 12, 4, 4, 0, 12, 4, 4, 0, 12]}
  ]
}
```

Payload index is `lx + 4 × (lz + 4 × ly)` (the Canon §1.8 order at tile scale). Slot 0 is air. Symmetry `x` (all 4 yaw rotations identical), `l` (4 distinct rotations), `i` (2), and a vertical mirror is generated for Oneiric flip rooms. Face socket order `+X, -X, +Y, -Y, +Z, -Z`. Two tiles may touch when their facing sockets are equal, or when they form a declared asymmetric pair (`pairs: [[a, b]]`). Rotations are expanded at content load; an expanded tileset holds at most 256 variants so a domain fits a `[u64; 4]` bitset.

### 9.3 WFC algorithm

```rust
fn wfc_room(r: &mut Rng, ts: &Tileset, tpl: &RoomTemplate, dims: (u8, u8, u8), cover_bp: i32) -> Result<TileGrid, WfcFail> {
    let mut dom = vec![ts.all_variants(); n_tiles(dims)];
    apply_pins(&mut dom, tpl);                          // shell classes and sockets
    propagate_all(&mut dom, ts)?;                        // AC-3: FIFO queue seeded in ascending tile index
    let mut resets = 0;
    loop {
        // observe: uncollapsed tile with the fewest options; ties to the lowest tile index
        let Some(t) = min_options_tile(&dom) else { return Ok(collapse_to_grid(&dom)); };
        let opts: Vec<u16> = dom[t].iter().collect();     // ascending variant index
        let w: Vec<u32> = opts.iter().map(|&v| weight(ts, v, cover_bp)).collect();
        let pick = opts[r.weighted(&w)];                   // 1 draw per observation
        dom[t] = Bitset::single(pick);
        if propagate_from(&mut dom, ts, t).is_err() {      // a tile reached 0 options
            resets += 1;
            if resets > 3 { return Err(WfcFail); }
            reset_block(&mut dom, t, 4, tpl);              // Merrell block modification: the 4×4×4-tile block around t
            propagate_all(&mut dom, ts)?;                  // re-derive domains from every collapsed tile
        }
    }
}
fn weight(ts: &Tileset, v: u16, cover_bp: i32) -> u32 {
    let w = ts.w(v) as i64;
    (if ts.has_flag(v, COVER) { mul_bp(w, 5000 + cover_bp) } else { w }) as u32
}
```

- `reset_block` returns every tile of the 4 × 4 × 4-tile block around `t` (clipped) to its pinned or full domain. On `WfcFail` the room takes `fallback_fill` with no further draws. Draws per room equal the number of observations, a declared variable count (Canon §3.4).
- Cover example: a weight-60 cover tile weighs `mul_bp(60, 11000) = 66` in Space Opera (`cover_bp` 6000) and `mul_bp(60, 7000) = 42` in Wuxia (2000).
- Rooms run on `item_rng("wfc", room_index, attempt)`, independently, through `par`. Palette slots 1–2 take the room's materials (§10.1); slots 3–6 the tileset palette.

## 10. Dressing, light, liquids and hazards

### 10.1 Dressing

Per room, on `item_rng("dress", i, attempt)`: draw the room's wall and floor materials (2 weighted calls over the zone genre's `materials`), then shuffle its prop sockets (1 shuffle) and fill the first `floor_div(len × 6000, 10000)` with props drawn from the tileset's `props` list (1 weighted call each). Props are blocks with collision; props flagged `cover` count toward `cover_bp` effects only through WFC weighting. Lore sockets and container sockets are left for §12 and §19.

### 10.2 Light plan for stealth gradients

Bands use effective light (Canon §2.3): **dark** `<= 4`, **dim** `5..=9`, **bright** `>= 10`. Targets per genre are `bright_bp` and `dark_bp` (§3.3); kinds tagged `room.stealth` use `bright_bp / 2` and `dark_bp × 3 / 2` (capped at 10000).

```rust
fn light_room(r: &mut Rng, room: &Room, g: &Gpv) -> Vec<Fixture> {
    let feet = room.feet_cells();                          // walkable cells (§14.1), ascending block index
    let mut sockets = room.light_sockets();                // template + tile sockets, ascending block index
    r.shuffle(&mut sockets);                               // 1 shuffle
    let mut placed = vec![];
    for s in sockets {
        if share(&feet, &placed, BRIGHT) >= room.bright_bp { break; }
        let trial = with(&placed, s);
        if gain(&feet, &placed, &trial, BRIGHT) == 0 { continue; }
        if room.dark_bp > 0 && share(&feet, &trial, DARK) < room.dark_bp { continue; }
        placed = trial;
    }
    if room.is_safe() { spawn_proof(&mut placed, &feet, g); }   // step 3
    if room.is_stealth() && g.ambient_light <= 6 { shadow_route(&mut placed, room); }
    placed
}
```

1. Light uses the doc 03 propagation inside the room box (I1); `share = floor_div(count_in_band × 10000, feet.len())`; a fixture emits the zone genre's `fixture_level`.
2. Example: a Mythic Fantasy hall with 840 feet cells needs 294 bright cells (`bright_bp` 3500), and no fixture may leave fewer than 210 dark cells (`dark_bp` 2500).
3. **Spawn-proofing** (safe rooms, no draws): while a feet cell has `light_block <= spawn_block_max`, add the unused socket covering the most such cells (ties to the lower block index); with no sockets left, add a ceiling fixture above the lowest-index uncovered cell.
4. **Shadow route** (stealth rooms, `ambient_light <= 6`): a feet-cell path with effective light `<= 6` must join the two lowest-index doors; otherwise remove fixtures in descending order of path cells they light (ties to the higher block index) until it exists or one fixture remains.

### 10.3 Liquids

- Non-safe rooms with basin sockets fill them with chance 2500 Bp (stage `liquid`; cisterns and flooded rooms always), liquid drawn from the zone genre's `liquids` (1 weighted call), to the rim, `cells × 1,000,000` ml, written settled.
- A flooded-detour room is at least 2 blocks deep along its path with a 2-block air pocket at most every 16 blocks: under 7 s at 2400 mm/s swim, inside the 14 s breath at END 40 (Canon §12).
- No lava, acid or red ink in safe rooms or within 8 blocks of the Entry spawn; no liquid above the Entry floor within 8 horizontal blocks.

### 10.4 Hazard and trap placement

Stage `hazard`, rooms ascending. Count: gauntlet `range_incl(5, 7)`; safe 0; others `floor_div(trap_bp, 10000)` plus 1 on `roll_bp(trap_bp % 10000)`. Each hazard: 1 weighted call over the zone genre's eligible `hazards` (§17.1), then a hazard socket or, for path traps, a `range`-chosen feet cell on the door-to-door path; 4 blocks of mutual spacing; none within 8 blocks of the Entry spawn. Damage enters Canon §10.6 at step 3: `D2 = base × P(d)` vs NPCs, `base × T(d)` vs Errata.

## 11. Population budget (doc 13 interface)

```
pop_mske = mul_bp(mul_bp(mul_bp(30000, density_bp), ftype_pop_bp), marg_pop_bp)
```

30,000 mSKE is 75% of the Canon §17.4 supply of 40 SKE; doc 13's ambient and wave spawns add the rest. `ftype_pop_bp` per §5.2; `marg_pop_bp` 12500 under `lattice.marg.dense`, else 10000. Examples: Space Opera `mul_bp(30000, 11000) = 33000`; Gothic Horror 27000; Steampunk challenge `mul_bp(mul_bp(30000, 10500), 12500) = 39375`.

`share_i = floor_div(pop_mske × pop_w_i, Σ pop_w)` with `pop_w` from §15.1; the remainder goes to the largest `pop_w`, ties to the lower room index. Example: 33000 over `[4, 2, 5, 2, 1, 3, 0, 2]` gives `[6947, 3473, 8684, 3473, 1736, 5210, 0, 3473]`; remainder 4 makes the third 8688.

`SpawnerRec { room, budget_mske, wake: asleep | guard | patrol, anchors, roster_genre, champion_slots }`; `champion_slots` = 1 in an Arena, else `floor_div(budget_mske, 15000)`. Doc 13 spends budgets at rank cost = Canon §17.3 kill-XP Bp ÷ 10 mSKE (minion 300, standard 1000, elite 3000, champion 6000), applies `enemy_count_bp` (Canon §17.2) and plan aspects (§4.8). At most 96 anchors per Folio keeps a party of 4 (`× 17500`) under `ACTIVE_ACTOR_CAP`.

## 12. Loot placement (doc 11 interface)

Container kinds and loot points: cache 1 LP, rack 2, chest 3, Reliquary 6, floor item 1. Target `lp_target = mul_bp(mul_bp(base_lp, loot_bp), marg_loot_bp)` with `base_lp` from §5.2 and `marg_loot_bp` 12500 under `lattice.marg.sealed`, else 10000.

1. Place each kind's fixed LP (§15.1 `lp`).
2. Spend the rest in stage `loot`: chests (3 LP) while at least 3 remain, then caches, each into a room with a free container socket chosen by 1 weighted call over `lw`.
3. The `loot_quota.emendations` highest-LP non-secret containers (ties to lower record index) are **guaranteed** for doc 11's Emendations (Canon §13.7).
4. Placed LP must lie in `floor_div(lp_target × 9000, 10000) ..= ceil_div(lp_target × 11000, 10000)`: `21..=27` for a standard Folio at `loot_bp` 10000.

Doc 11 rolls contents (`rng.loot_place`, keyed `rng.loot`). Unwritten Folios place no Lucre.

## 13. Bleed zones

### 13.1 Bleed plan (pure, previewed)

```rust
fn bleed_plan(run: u64, n: u32, g: GenreId, prev: Option<GenreId>, d: i32, marg: &[MargId]) -> (Option<GenreId>, Vec<GenreId>) {
    if has_law(g, NO_BLEED) { return (None, vec![]); }                         // RD, UW, UT
    let entry = prev.filter(|p| is_core(*p) && *p != g);
    let mut r = stage_rng(run, n, g, "bleed", 0);
    let extra = marg.contains(&MARG_BLEEDING) as u32;
    let chance = if g == ONEIRIC || extra > 0 { 10000 } else { min(8000, g.drift_bp + 500 * (vol_of_depth(d) - 1)) };
    let mut drift = vec![];
    if r.roll_bp(chance) {                                                       // 1 call
        let cand: Vec<GenreId> = CORE.filter(|c| *c != g && Some(*c) != prev);   // lattice order
        let mut w: Vec<u32> = cand.iter().map(|c| edge(g, *c)).collect();
        for _ in 0..(if g == ONEIRIC { 2 } else { 1 }) + extra {                 // 1 call each
            let i = r.weighted(&w); drift.push(cand[i]); w[i] = 0;
        }
    }
    (entry, drift)
}
```

`vol_of_depth(d) = ceil_div(d, 5)` for `d <= 25`, and 6 beyond. Example: at Folio 7 after Mythic Fantasy, Gothic Horror gets entry bleed Mythic Fantasy and no drift (roll failed); Oneiric gets entry Mythic Fantasy and drift `[Mythic Antiquity, Gothic Horror]`; Redaction gets none.

### 13.2 Zone assignment (stage `zones`)

1. **Entry zone:** the Entry room, plus its lowest-`NodeId` neighbor Room on `roll_bp(bleed_in_bp)`.
2. **Drift zone** per drift genre: seed = `range` over eligible Rooms (not Entry, Threshold, safe, Arena, BossArena, Checkpoint, gravity-flip, or zoned); grow breadth-first (ascending `NodeId`) to `s = range_incl(1, 3)` rooms.
3. Corridors take their lower-`NodeId` endpoint's zone; `zone_of_cell` records it.
4. A bleed zone uses its genre for `local_genre` (Contradiction re-evaluates on crossing), `light_sky`, gravity, §2.1 multipliers, tileset, materials, fixtures, hazards, liquids and roster. A Scriptorium there Rewrites into the bleed genre.

### 13.3 Blending

Wall and floor slots within 2 blocks of a face between zones dither: with `h = mix64(gen_seed ^ (block_linear_index + 1))`, a block takes the neighbor zone's material when `h % 10000 < [5000, 2500, 0][dist]` (`dist` 0 = touching). Zone membership stays per cell; doc 18 draws a stitched seam.

## 14. Validation and repair

### 14.1 Navigation model

A **feet cell** is air with solid or slab-top support below and 2 air cells above (1800 mm). Moves: horizontal at level; up 1 block with 3 air above the target (Canon §4.2); down at most `floor_div(SAFE_FALL(v), 1000)` blocks, `v` = target local VERVE (3 at VERVE 0, 4 at 10); ladders, vines, ropes; swimming in water, blood, alcohol. Lava, acid and red ink block. Locks need keys, puzzle gates open once the puzzle room is reached, bars follow §8.2.

### 14.2 Checks

| ID | Check |
|---|---|
| V1 | every non-secret room has a feet cell reachable from Entry under the key progression |
| V2 | key before lock: progressive search from Entry collects each key strictly before any door that needs it |
| V3 | the Threshold is reachable |
| V4 | no softlock: for each progression state `K_i`, every feet cell reachable with `K_i` can reach, with `K_i`, the Threshold or a key not in `K_i` |
| V5 | V1–V4 hold in both relink states (§8.4) and for the collapsed bridge (§7.2) |
| V6 | nothing hazardous within 8 blocks of the Entry spawn point; no liquid above the Entry floor within 8 horizontal blocks |
| V7 | every feet cell of every safe room has `light_block > spawn_block_max` |
| V8 | no `electronic` hazard, lock or camera in a cell whose local TEC ≤ 2 (Canon §5.7) |
| V9 | no hazard, service or container tag with a `dep` axis whose local axiom is 0 (Canon §5.3) |
| V10 | LP within tolerance (§12); spawner budgets sum exactly to `pop_mske`; anchors ≤ 96 |
| V11 | `room.no_mount` tags match §15.4 |
| V12 | exactly one Threshold; required rooms present; secret rooms reachable only through Hidden edges; at most one secret exit |

### 14.3 Repair and retry

- V1–V5: carve a 1 × 2 tunnel along A* from the reachable frontier to the target (air 1, breakable 5, reinforced impassable), at most 3 carves per attempt, no draws.
- Still failing: `attempt += 1`, rerun stages 1–11 (attempts 0..=3); attempt 4 forces `rooms_min`, no minor cycles, `cycle_hidden_shortcut`; then the CI-validated authored fallback `room.tpl.fallback.<layout>` in the genre skin.
- V6–V12: fixed in place without draws: remove offending hazards, liquids or secret exits (highest index first), add safe-room light (§10.2 step 3), recompute tags.

## 15. Room catalog

### 15.1 Kinds

`fp` is the allowed footprint range in cells, `st` storeys, `max` per Folio, `w` base weight for typing (— = placed only by rules or quotas), `pop` the population weight (§11), `lp/lw` fixed LP and loot weight (§12).

| ID | fp | st | max | w | verb | pop | lp/lw | Tags | Function |
|---|---|---|---|---|---|---|---|---|---|
| `room.entry` | 1×1–2×2 | 1 | 1 | — | — | 0 | 0/0 | safe | spawn point at its center |
| `room.threshold` | 2×2 | 1 | 1 | — | — | 0 | 0/0 | safe, no_bleed | lectern and 3 exit doors (§4.9) |
| `room.hall` | 2×2–3×3 | 1–2 | — | 300 | melee | 4 | 0/2 | — | combat |
| `room.guardpost` | 1×1–1×2 | 1 | — | 200 | shoot | 2 | 0/1 | stealth | wake `guard` |
| `room.barracks` | 2×2 | 1 | 2 | 120 | melee | 5 | 0/2 | — | wake `asleep` |
| `room.crossroads` | 2×2 | 1 | — | 100 | — | 2 | 0/1 | — | hub |
| `room.gallery` | 1×3 | 1 | — | 120 | sneak | 2 | 0/1 | stealth | columned |
| `room.ambush_den` | 2×2 | 1 | 2 | 50 | sneak | 3 | 0/1 | stealth | dark, wake `guard` |
| `room.overlook` | 2×2 | 2 | 1 | 40 | traverse | 2 | 0/1 | — | atrium |
| `room.chasm` | 1×3 | 1 | 1 | 40 | traverse | 1 | 0/1 | — | bridge over `hazard.chasm` |
| `room.cistern` | 2×2 | 1–2 | 1 | 40 | traverse | 2 | 0/1 | — | basins always filled |
| `room.arena` | 3×3 | 2 | 1 | — | — | 0 | 3/0 | — | 1 champion slot + 3000 mSKE |
| `room.key_room` | 1×1–2×2 | 1 | — | — | — | 2 | 0/0 | — | key pedestal |
| `room.treasury` | 1×2 | 1 | 2 | 40 | — | 1 | 5/0 | no_mount | |
| `room.reliquary_vault` | 1×1–2×2 | 1 | 1 | — | — | 0 | 6/0 | no_mount | in-run Reliquary (doc 11) |
| `room.shop` | 1×2 | 1 | 1 | — | talk | 0 | 0/0 | safe, no_mount | vendor (doc 11) |
| `room.scriptorium` | 2×2 | 1 | 1 | — | — | 0 | 0/0 | safe, no_mount | §15.3 |
| `room.fortune_hall` | 2×2 | 1 | 1 | — | — | 0 | 0/0 | safe | 2 Fortune Engines |
| `room.campfire` | 1×1–2×2 | 1 | 1 | — | — | 0 | 0/0 | safe | rest (§15.3) |
| `room.shrine` | 1×1 | 1 | 1 | 80 | pray | 1 | 1/1 | — | altar (doc 06); MYT ≥ 1 |
| `room.library` | 2×2 | 1–2 | 1 | 70 | cast | 1 | 2/2 | no_mount | 4 lore sockets |
| `room.observatory` | 2×2 | 2 | 1 | 30 | cast | 1 | 1/2 | — | 2 lore sockets |
| `room.alchemy_lab` | 1×2 | 1 | 1 | 60 | craft | 1 | 2/1 | no_mount | doc 08 station |
| `room.forge` | 2×2 | 1 | 1 | 60 | craft | 2 | 1/1 | — | doc 07 station |
| `room.workshop` | 2×2 | 1 | 1 | 50 | craft | 2 | 1/2 | — | doc 07 station |
| `room.armory` | 1×2 | 1 | 1 | 70 | shoot | 2 | 3/1 | no_mount | racks, counter sources |
| `room.prison` | 2×2 | 1 | 1 | 50 | talk | 3 | 0/1 | — | captive (doc 13) |
| `room.garden` | 2×2–3×3 | 1 | 1 | 60 | craft | 1 | 2/1 | — | ingredients, food |
| `room.larder` | 1×1 | 1 | 1 | 50 | craft | 1 | 2/1 | no_mount | food |
| `room.crypt` | 2×2 | 1 | 1 | 60 | pray | 3 | 2/1 | — | undead roster |
| `room.server_room` | 1×2–2×2 | 1 | 1 | 40 | hack | 2 | 1/2 | electronic | TEC ≥ 5; 2 lore sockets |
| `room.stable` | 2×2 | 1 | 1 | 30 | traverse | 1 | 1/0 | — | mount bay |
| `room.gauntlet` | 1×3 | 1 | 3 | 60 | traverse | 0 | 3/0 | — | 5–7 hazards |
| `room.puzzle` | 2×2 | 1 | 1 | — | — | 0 | 3/0 | — | §15.3 |
| `room.editor_checkpoint` | 2×2 | 1 | 1 | — | — | 0 | 0/0 | safe, no_bleed | §20.1 |
| `room.boss_arena` | 3×3 | 2 | 1 | — | — | 0 | 0/0 | no_bleed | §20.2 |

Secret rooms are §16.2. With them the catalog has 44 kinds.

### 15.2 Typing weight

```
w(kind, genre) = mul_bp(mul_bp(base_w, verb_bp(genre, kind.verb)), bias_bp(genre, kind))
```

`verb_bp` is 10000 for kinds without a verb; `bias_bp` defaults to 10000. Kinds whose requirement fails in the room's zone have weight 0. Examples: Cyberpunk server room `mul_bp(mul_bp(40, 16000), 20000) = 128`; Mythic Fantasy library `mul_bp(mul_bp(70, 15000), 12000) = 126`; Wuxia hall `mul_bp(300, 16000) = 480`.

Room bias (`room_bias`, Bp): MF armory 12000, library 12000. GH crypt 20000, prison 12000, garden 5000. CH library 20000, observatory 15000, cistern 15000. SO server_room 15000, garden 3000, crypt 0. CP server_room 20000, garden 3000, shrine 5000, crypt 0. PA workshop 15000, garden 6000, library 5000. WW stable 15000, prison 12000. HN prison 15000, gallery 15000, garden 3000, crypt 3000. WX overlook 15000, garden 12000. SP workshop 20000, forge 15000. FT garden 20000, larder 15000. MA shrine 20000, crypt 12000. ON overlook 20000, observatory 15000. RD gallery 20000, server_room 15000, garden 0, stable 0, crypt 0, shrine 0. UW every kind 10000 except shop, Scriptorium, Fortune Engine hall 0. UT none.

### 15.3 Services and puzzles

- **Campfire.** Interact for 300 uninterrupted ticks while not `in_combat`: heals 5000 Bp of Max HP (Canon §10.9), refills stamina and aether, restores 25 composure. Once per Erratum per campfire.
- **Scriptorium** (Canon glossary): Rewrite into the local genre (doc 11 prices), identification (doc 10), free perk-column choice (Canon §13.9), reattachment of severed parts (Canon §9.1), affliction cure for `mul_bp(500, P(d))` Lucre (Canon §16.4).
- **Puzzle** variants, chosen by `item_rng("puzzle", i)` weighted `[lever 40, plates 30, sigils 30]` (1 call):
  - *Lever order:* 4 levers; the order is a shuffle of `[0, 1, 2, 3]` (1 shuffle); the order is inscribed on 2 lore objects elsewhere in the Folio. A wrong pull resets all levers and fires one `hazard.alarm` effect.
  - *Plates:* 3 plates must each bear at least 40,000 g at the same tick (an Erratum is 75,000 g; any placed block of mass ≥ 40,000 g counts).
  - *Sigils:* 3 sigil blocks must each read `light_block >= 12` at the same tick; players solve it with placed light (§2.2).
  - Solving opens the `PuzzleGate` and the room's 3-LP chest.

### 15.4 `room.no_mount`

A room carries `room.no_mount` when its kind lists it, or when any feet cell on a path between two of its doors has less than 3 blocks of clear headroom (3000 mm; a mount needs 2600 mm, Canon §17.7). Summoning a mount inside a tagged room fails. A mounted Erratum that enters one is dismounted with `ev.dismounted` (doc 05). Corridors are never tagged (§8.3).

## 16. Secrets

### 16.1 Count

`n_secrets = floor_div(n_rooms × secret_bp, 10000)`, plus 1 when `roll_bp(n_rooms × secret_bp % 10000)` succeeds (stage `graph`, 1 call); rest and boss Folios have exactly 1; `lattice.marg.annotated` adds 2. Example: 18 rooms in Mythic Fantasy: `18 × 1500 = 27000`, so 2 secrets plus a third with chance 7000 Bp.

### 16.2 Secret room kinds

Drawn per Secret node in stage `typing` (1 weighted call).

| ID | w | Contents |
|---|---|---|
| `secret.room.cache` | 30 | 3 LP chest |
| `secret.room.shrine` | 15 | hidden patron altar; prayer here counts as at a shrine and its first prayer gains +100 favor (doc 06) |
| `secret.room.reliquary` | 10 | in-run Reliquary plus its in-run key (doc 11) |
| `secret.room.armory` | 15 | 2 racks (4 LP) with at least one counter source (Canon §17.3a) |
| `secret.room.editors_desk` | 5 | Editor intel lore plus a Palimpsestic-eligible container (Canon §13.3, doc 11); depth ≥ 6 |
| `secret.room.margin_note` | 15 | 1 lore object holding a story event (§19) plus a 1 LP cache |
| `secret.room.oubliette` | 7 | a captive hireling (doc 13) and 1 LP |
| `secret.room.ink_well` | 3 | a `liq.red_ink` basin from which an Erratum may drink one dose of `afflict.red_ink` (Canon §16.4); depth ≥ 11 |

### 16.3 Hiding methods

| ID | How it hides | How it is found |
|---|---|---|
| `secret.hide.cracked_wall` | a breakable wall block with a hairline-crack texture variant | passive detection (Canon §6.4 row 35, keyed `rng.stealth`, once per observer per 60 ticks within 3 m); any hit on it plays a hollow knock (deterministic); breaking it |
| `secret.hide.false_wall` | a passable block rendered as wall | passive detection; walking into it |
| `secret.hide.lever` | a lever disguised as a prop opens a wall section | passive detection reveals the lever outline |
| `secret.hide.buried` | the passage lies 2 blocks under a floor marked by one odd tile | passive detection marks the tile; digging |
| `secret.hide.waterfall` | the door sits behind a liquid curtain | seen when an entity crosses the curtain |
| `secret.hide.ceiling_hatch` | a hatch in a ceiling 4 blocks up | passive detection; climbing or building up |
| `secret.hide.verb_lock` | a seal needing a verb native to another genre: hack (TEC), prayer (MYT) or a spell (ARC) | using that verb on it; the seal names the verb when detected |
| `secret.hide.dream_veil`, `.clearance`, `.blank_page`, `.palimpsest_wall` | secret exits only | §16.4 |

Method per secret: weighted `[cracked 30, false 20, lever 15, buried 10, waterfall 5, hatch 10, verb 10]`, with methods that need absent features (no liquid, single storey) at weight 0 (1 call). Solving the Folio story reveals its most valuable secret (§19.4).

### 16.4 Secret exits to the four secret genres

Stage `secret` checks targets in the order Oneiric, Redaction, Unwritten, Ur-Text, rolling `roll_bp(chance)` for each eligible target until one succeeds (1 call per eligible target checked). A target is eligible when the next Folio `n + 1` is neither a boss nor a forced Folio and:

| Exit | Chance | Extra condition | Hiding and reveal |
|---|---|---|---|
| `secret.exit.oneiric` | the GPV `secret_exits` value (§4.3) | `n >= 3` | `secret.hide.dream_veil`: invisible until an Erratum completes a campfire rest in this Folio; worldgen adds a `room.campfire` when absent |
| `secret.exit.redacted` | GPV value | `n >= 6`, local TEC ≥ 5 | `secret.hide.clearance`: an electronic lock (doc 09) that opens on a successful Hacking attempt, or automatically when an Erasure hunter dies in this Folio |
| `secret.exit.unwritten` | 1000 | `n >= 8` | `secret.hide.blank_page`: visible and passable only for Errata holding at least 5 `st.erasure` stacks |
| `secret.exit.ur_text` | 800 | `n >= 26` | `secret.hide.palimpsest_wall`: dissolves while a Palimpsestic item is within 3 m |

- Crossing uses the §4.9 party rule. The next Folio's genre is the target; no `rng.lattice` draw happens for this Folio because its exit UI never opened.
- **Dormant portals.** Generation stays pure, so the portal is generated regardless of history. At runtime the portal is dormant when the target genre already appeared this run (before Marginalia) or in the last 10 Folios (Marginalia). Every secret-exit room also holds a 3 LP chest, so a dormant one still pays.
- Abridged mode: only from regular Folio `f = 1` (§22).

## 17. Hazards and traps

### 17.1 Archetypes

`base` is display HP for the Canon §10.6 entry point; buildup is threshold points scaled by `S(d)` (Canon §11.2, `src_lvl = d`). Hidden hazards use passive detection (§16.3) and doc 09 disarming with Security. `electronic` hazards need local TEC ≥ 3.

| ID | Kind | Trigger and cycle | Effect | dep |
|---|---|---|---|---|
| `hazard.dart_trap` | hidden | plate | 3 darts, 25000 mm/s, 6 `dmg.pierce` each | none |
| `hazard.blade_trap` | timed | sweep every 60 ticks | 14 `dmg.slash`, 30 bleeding buildup | none |
| `hazard.spike_pit` | hidden | lid breaks at ≥ 40,000 g | 3-block fall + 10 `dmg.pierce`, 40 bleeding | none |
| `hazard.crusher` | timed | slam every 120 ticks, 30-tick telegraph | 25 `dmg.blunt` | none |
| `hazard.flame_jet` | timed | on 60 of 180 ticks; 4 m cone, half-angle 2731 Bam | 8 `dmg.fire` per status pulse, 60 burning per pulse | none |
| `hazard.gas_vent` | hidden | plate | 6 cells of `gas.miasma` over 120 ticks | none |
| `hazard.shock_floor` | timed, electronic | 3 × 3, live 90 of 180 ticks | 6 `dmg.shock` per pulse, 40 shocked | techne |
| `hazard.alarm` | hidden | tripwire or sensor | `ev.noise` 90 dB at the source | none |
| `hazard.net_snare` | hidden | tripwire | `st.rooted` direct; 3 × 3 `block.web` | none |
| `hazard.rune_ward` | hidden | glyph step | 3 m blast, 18 `dmg.arcane`, 50 unraveled | arcana |
| `hazard.curse_glyph` | hidden | glyph step | 60 dread and 40 withering buildup within 2 m | mythos |
| `hazard.camera` | visible, electronic | cone 8192 Bam, 16 m, sweep ±4096 Bam per 360 ticks | 60 ticks of sight on an Erratum = alarm; hackable | techne |
| `hazard.collapsing_floor` | visible cracks | 3 × 3 breaks 30 ticks after ≥ 40,000 g | drop below; OneWay in validation | none |
| `hazard.falling_debris` | latent | `ev.noise` ≥ 80 dB within 6 m | 3 blocks fall, 12 `dmg.blunt` each | none |
| `hazard.lava_pool`, `hazard.acid_pool`, `hazard.deep_water`, `hazard.oil_slick`, `hazard.ink_seep` | visible liquids | contact | Canon §12.1; deep water ≥ 3 blocks; oil 100 ml per cell | none |
| `hazard.steam_vent` | timed | on 60 of every 240 ticks | `gas.steam`, 2 cells | none |
| `hazard.rad_zone` | visible | 4 × 4 floor area | 20 irradiated buildup per pulse | none |
| `hazard.miasma_bog` | visible | persistent | `gas.miasma` 3 × 3 × 2 cells | none |
| `hazard.web_field` | visible | persistent | `block.web` 3 × 3 × 2 | none |
| `hazard.quicksand` | visible | 3 × 3 mire | `st.slowed` while inside; sinks at most 1 block | none |
| `hazard.hull_breach` | latent | panel destroyed (30 damage) | 3000 mm/s² pull within 8 m for 300 ticks, then seals | techne |
| `hazard.redaction_bar` | timed | 1 × 3 × 8 slab slides at 2000 mm/s | conceal 15; contact applies `st.silenced` | techne |
| `hazard.chasm` | visible | pit ≥ 6 blocks deep | fall damage only | none |
| `hazard.gravity_flip` | visible | a gravity-flip volume (§8.5) | gravity reversed | none |

Worked examples against Errata (`base × T(d)`, one `mul_bp`): a dart at `d = 1` deals `mul_bp(6000, 10000) = 6000` mHP; at `d = 10`, `mul_bp(6000, 16190) = 9714` mHP, and against an NPC `mul_bp(6000, 21718) = 13030` mHP. A crusher at `d = 15` deals `mul_bp(25000, 21160) = 52900` mHP to an Erratum; a flame-jet pulse at `d = 5` deals `mul_bp(8000, 12388) = 9910` mHP.

### 17.2 Per-genre hazard lists (skin names)

Weights are 20, 20, 15, 15, 15, 15 in list order. Skin names are `loc.<hazard id>.skin.<genre_suffix>.name` (doc 18).

| Genre | Hazards |
|---|---|
| MF | dart_trap (arrow slit), spike_pit (oubliette), blade_trap (pendulum axe), rune_ward (warding rune), flame_jet (dragon-mouth brazier), collapsing_floor (rotten flagstones) |
| GH | spike_pit (open grave), curse_glyph (hex sigil), blade_trap (reaper's scythe), falling_debris (rotten rafters), miasma_bog (grave mist), alarm (bell rope) |
| CH | curse_glyph (unseen sigil), deep_water (black tide), miasma_bog (spore fog), falling_debris (crumbling vault), alarm (drowned gong), camera (seeing lens) |
| SO | shock_floor (live deck plate), camera (security optic), hull_breach (hull breach), rad_zone (reactor leak), steam_vent (coolant vent), crusher (blast door) |
| CP | camera (surveillance cam), shock_floor (live wire mesh), dart_trap (needle launcher), alarm (laser tripwire), acid_pool (chem spill), steam_vent (ruptured main) |
| PA | rad_zone (hot zone), collapsing_floor (rotted overpass), dart_trap (nail trap), acid_pool (toxic sump), oil_slick (fuel spill), spike_pit (punji pit) |
| WW | dart_trap (shotgun tripwire), quicksand (sinkmud), collapsing_floor (mine shaft), oil_slick (kerosene), falling_debris (rockfall), curse_glyph (hex fetish) |
| HN | alarm (burglar alarm), camera (watch camera), oil_slick (spilled gasoline), deep_water (harbor), collapsing_floor (rusted fire escape), shock_floor (live wire) |
| WX | dart_trap (crossbow nest), blade_trap (spinning blades), spike_pit (tiger pit), net_snare (rope net), deep_water (rapids), falling_debris (rockslide) |
| SP | steam_vent (boiler vent), crusher (piston press), flame_jet (furnace mouth), shock_floor (tesla plate), alarm (klaxon), oil_slick (machine oil) |
| FT | net_snare (briar snare), spike_pit (wolf pit), web_field (spider glade), rune_ward (fairy ring), deep_water (millpond), curse_glyph (witch's mark) |
| MA | blade_trap (labyrinth blades), spike_pit (pit of the damned), crusher (closing walls), flame_jet (temple brazier), curse_glyph (oath curse), lava_pool (smith-god's forge) |
| ON | gravity_flip (upside room), deep_water (dream sea), collapsing_floor (falling dream), rune_ward (lucid sigil), web_field (cobweb thought), falling_debris (raining furniture) |
| RD | camera (compliance optic), redaction_bar (redaction bar), alarm (audit alarm), shock_floor (live floor), crusher (shredder), ink_seep (red ink spill) |
| UW | ink_seep (red ink), chasm (blank void), collapsing_floor (erased floor), falling_debris (falling letters), spike_pit (deleted step), blade_trap (cutting line) |
| UT | rune_ward, camera, curse_glyph, flame_jet, ink_seep, collapsing_floor (each in the skin of a core genre chosen per room by the room's wedge, §20.3) |

Every list passes lint L4: no electronic hazard where TEC ≤ 2 (MF, WX, FT, MA, UW), no arcana hazard where ARC = 0 (PA, HN, UW), no mythos hazard where MYT = 0 (UW).

## 18. Ambient and narrative events

### 18.1 Archetypes

Ambient events change conditions; narrative events create encounters or choices. `requires` is checked against the target room's local genre.

| ID | Class | Effect | requires |
|---|---|---|---|
| `gpv.evt.light_failure` | ambient | the target room's fixtures go dark for 600 ticks | — |
| `gpv.evt.weather_front` | ambient | 1800 ticks: fog +1 per 8 m, `sound_bp` × 12000 | — |
| `gpv.evt.distant_noise` | ambient | `ev.noise` 70 dB ≥ 2 rooms from every Erratum | — |
| `gpv.evt.tremor` | ambient | fires the room's falling debris, else drops 2 blocks | — |
| `gpv.evt.whispers` | ambient | −5 composure to each Erratum in the target room with `light_at < 8` | MYT ≥ 1 |
| `gpv.evt.boon` | ambient | `st.blessed` to each Erratum in the target room | MYT ≥ 1 |
| `gpv.evt.lore_echo` | ambient | a lore object with an unread event appears (§19) | — |
| `gpv.evt.editor_glimpse` | ambient | 2,000,000 ml of `liq.red_ink` seeps into the target room | — |
| `gpv.evt.gravity_shift` | ambient | every gravity-flip volume toggles for 600 ticks | gflip cells exist |
| `gpv.evt.wandering_vendor` | narrative | a vendor stays 1800 ticks (doc 11) | — |
| `gpv.evt.patrol_shift` | narrative | doc 13 spawns a 1000 mSKE patrol at the Threshold walking toward Entry | — |
| `gpv.evt.ambush` | narrative | doc 13 spawns 2000 mSKE on 2 spawn-eligible cells adjacent to the party's room | — |
| `gpv.evt.reinforcements` | narrative | doc 13 spawns 3000 mSKE at a spawn-eligible cell at least 16 blocks from every Erratum | — |
| `gpv.evt.plea` | narrative | freeing a captive within 1800 ticks: +100 reputation with the first native faction, hireling offer (doc 13) | — |
| `gpv.evt.duel_challenge` | narrative | a champion (one plan aspect) duels the accepting Erratum behind a barrier, ≤ 1800 ticks | VRV ≥ 5 |
| `gpv.evt.flood` | narrative | a basin in the target room releases 20,000,000 ml of its liquid over 600 ticks | basin exists |
| `gpv.evt.fire_outbreak` | narrative | ignites the flammable block nearest the target room's center (doc 03) | — |
| `gpv.evt.lockdown` | narrative, electronic | doors of the target and adjacent rooms shut 900 ticks; cameras activate | TEC ≥ 3 |
| `gpv.evt.redaction_sweep` | narrative | a `hazard.redaction_bar` sweeps the target corridor once | TEC ≥ 1 |

### 18.2 Per-genre event lists (skin names)

Weights 25, 20, 15, 15, 15, 10 in list order.

| Genre | Events |
|---|---|
| MF | boon (patron's light), wandering_vendor (traveling tinker), ambush (goblin raid), patrol_shift (knight patrol), fire_outbreak (dragon's breath), lore_echo (minstrel's tale) |
| GH | light_failure (candles gutter), whispers (voices in the walls), ambush (the hunt), weather_front (graveyard fog), plea (captive's cry), lore_echo (the portrait speaks) |
| CH | whispers (the chorus), light_failure (lamp-eater), tremor (something moves below), distant_noise (bells under water), flood (the tide rises), editor_glimpse (ink from nowhere) |
| SO | reinforcements (dropship), lockdown (security lockdown), patrol_shift (trooper sweep), wandering_vendor (smuggler), tremor (hull impact), distant_noise (klaxon) |
| CP | lockdown (corp lockdown), reinforcements (rapid response), wandering_vendor (fixer), weather_front (acid rain), ambush (gang ambush), distant_noise (car alarm) |
| PA | weather_front (dust storm), ambush (raider ambush), wandering_vendor (caravan trader), tremor (collapse), fire_outbreak (fuel fire), plea (survivor's plea) |
| WW | duel_challenge (high noon), weather_front (dust devil), ambush (bushwhack), wandering_vendor (peddler's wagon), whispers (hex whispers), tremor (mine collapse) |
| HN | weather_front (downpour), distant_noise (siren), plea (informant), ambush (hit squad), light_failure (blackout), lore_echo (case file) |
| WX | duel_challenge (sect challenge), ambush (bandit ambush), weather_front (mountain mist), boon (master's blessing), wandering_vendor (wandering monk), lore_echo (rumor of the rivers and lakes) |
| SP | lockdown (constabulary whistle), fire_outbreak (boiler fire), tremor (engine shudder), wandering_vendor (tinker's cart), reinforcements (airship drop), distant_noise (steam whistle) |
| FT | wandering_vendor (peddler of bargains), plea (lost child), whispers (the mirror speaks), boon (godmother's gift), weather_front (enchanted mist), ambush (wolves) |
| MA | boon (omen of the gods), duel_challenge (champion's challenge), patrol_shift (phalanx march), tremor (earthquake), plea (supplicant), lore_echo (the chorus recounts) |
| ON | gravity_shift (the world turns over), whispers (dream voices), lore_echo (memory surfacing), light_failure (the lights dim), flood (dream sea rising), wandering_vendor (dream merchant) |
| RD | redaction_sweep (redaction sweep), lockdown (audit), patrol_shift (enforcer rounds), distant_noise (intercom), editor_glimpse (red ink leak), whispers (compliance announcement) |
| UW | editor_glimpse (ink pools), reinforcements (Editor squad), light_failure (blank-out), distant_noise (pencil scratching), tremor (eraser pass), lore_echo (an unwritten line) |
| UT | lore_echo (every story at once), boon (the Authors' regard), editor_glimpse (the red pen), reinforcements (Editors), tremor (the page shudders), weather_front (ink weather) |

UW lists no MYT-requiring event (its MYT is 0); `boon` and `whispers` appear only where MYT ≥ 1.

### 18.3 Scheduler (`rng.event`, sequential, host only)

```rust
// Runs on ticks t with t - entry >= 1800 and (t - entry) % 1200 == 0, while the Folio has fired < 6 events.
fn event_check(r: &mut Rng /* rng.event */, f: &FolioState) {
    if !r.roll_bp(f.gpv.event_bp) { return; }                                   // 1 draw
    let elig = f.gpv.events.filter(|e| f.cooldown_ok(e) && requires_ok(e, f));  // list order
    if elig.is_empty() { return; }
    let e = elig[r.weighted(&weights(&elig))];                                   // 1 draw
    let rooms = target_rooms(e, f);              // rooms holding or adjacent to an Erratum, ascending index
    let room = rooms[r.range(rooms.len() as u64) as usize];                      // 1 draw
    fire(e, room, f);                            // ambush draws 2 more: range over eligible cells, twice
}
```

Draws per check: 1, or 3 on a fired event (5 for ambush). Narrative events have a 2400-tick cooldown between narrative events; the same event never fires twice in a Folio. Boss arenas and rest Folios never receive events. Example: with `event_bp = 4000` (Gothic Horror), a 360 s Folio has `floor_div(21600 - 1800, 1200) + 1 = 17` checks and an expected 6.8 successes, capped at 6 fired events.

## 19. Folio story fragments

Each Folio is a page that once told a story, since scraped and overwritten. Following Caves of Qud's sultans (R1 §5.9, R4 §7.3), the generator draws a short life in events, rationalizes gaps after the fact, then scatters biased and contradictory retellings. Stage `story`.

### 19.1 Cast

`hero_name = r.next_u64()`, `rival_name = r.next_u64()` (name seeds for doc 18's genre naming grammar, 2 draws); `domains` = 2 of the genre's 4 `story_domains` without replacement (2 weighted calls, equal weights). The **seat** is the treasury, vault or library with the lowest `NodeId`, else the room with the highest fixed LP.

### 19.2 Events and rationalization

Five slots: origin, rise, deed, fall, erasure. Erasure is always `gpv.story.erasure.scraped` ("the Editors scraped this page"). For each other slot, eligible templates are those whose `requires` tags are all held; weight `10 + 20 × [template.domain ∈ domains]`; 1 weighted call.

| Template | Slot | requires | grants | domain |
|---|---|---|---|---|
| `gpv.story.origin.foundling` | origin | — | `orphan` | any |
| `gpv.story.origin.heir` | origin | — | `noble`, `rival_kin` | oath, empire, jade |
| `gpv.story.origin.apprentice` | origin | — | `trained` | machine, steam, data |
| `gpv.story.rise.oath_sworn` | rise | `noble` or `trained` | `oath` | oath, sword |
| `gpv.story.rise.relic_found` | rise | — | `relic` | any |
| `gpv.story.rise.pact` | rise | `orphan` | `pact` | bargain, hex, deep |
| `gpv.story.deed.slew_beast` | deed | `relic` or `oath` | `renown` | wyrm, wolf, sea |
| `gpv.story.deed.built_wonder` | deed | `trained` | `renown`, `wonder` | brass, clock, chrome |
| `gpv.story.deed.stole_fire` | deed | `pact` | `renown`, `curse` | flame, star, data |
| `gpv.story.fall.betrayed` | fall | `rival_kin` or `renown` | `betrayed` | lie, money, blood |
| `gpv.story.fall.hubris` | fall | `renown` | `fallen` | hubris, star, sky |
| `gpv.story.fall.cursed` | fall | `curse` or `pact` | `fallen` | moon, madness, crypt |
| `gpv.story.bridge.met_rival` | bridge | — | `rival_kin` | any |
| `gpv.story.bridge.won_fame` | bridge | — | `renown` | any |
| `gpv.story.erasure.scraped` | erasure | — | `erased` | any |

**Rationalization.** When a slot has no eligible template, insert the bridge whose grant unblocks the highest-weight blocked template of that slot (ties to the earlier table row), then draw again. At most 2 bridges per story. Example: origin `apprentice` (grants `trained`), rise `relic_found`, deed `built_wonder`; fall needs `rival_kin`, `renown`, `curse` or `pact`; `renown` is held, so `hubris` and `betrayed` are eligible and no bridge is needed.

### 19.3 Retellings and placement

- Lore objects: `L = 3 + floor_div(n_rooms, 6)`. Each shows one event in one voice. Event order: shuffle the event list (1 shuffle), assign round-robin so every event appears once when `L >= events`.
- Voice per object: weighted `praise 40, slander 40, redact 20` (1 call). Every third object in placement order is **overwritten**: hero and rival swap roles in its text, so two objects contradict each other on purpose.
- Sockets: weighted by room (library 4, shrine 3, observatory 2, server room 2, crypt 2, other kinds with lore sockets 1), 1 weighted call per object; one object goes to `secret.room.margin_note` when present.

### 19.4 Reading and solving

Reading takes a 60-tick interact and grants Inscription skill XP per doc 04's magnitude table (magnitude `d`). When the party has read objects covering at least 3 distinct events, the story is **solved**: the Folio's highest-LP secret is revealed (its hiding is removed and it shows on the map), the story relic that doc 11 places in the seat's best container becomes identified (doc 10), and the story is recorded for the Codex (doc 14). The relic's affix themes are the 2 domains.

### 19.5 Folio Leaf and text

One `cat.quest` item per Folio, the **Folio Leaf** (template `item.quest.folio_leaf`, doc 11), lies in the seat; carried through the Threshold it records the full story in the Codex. All text comes from `loc.gpv.story.*` template grammars (doc 18) or the Scribe model; text never enters simulation state.

```json
{ "hero_name": "0x9c1f00d2a4be7731", "rival_name": "0x13a4c0ffee0f1e2d", "domains": ["brass", "clock"],
  "events": ["gpv.story.origin.apprentice", "gpv.story.rise.relic_found", "gpv.story.deed.built_wonder", "gpv.story.fall.hubris", "gpv.story.erasure.scraped"],
  "lore": [{"room": 4, "event": 2, "voice": 0, "overwritten": 0}, {"room": 9, "event": 0, "voice": 1, "overwritten": 0}, {"room": 11, "event": 3, "voice": 2, "overwritten": 1}] }
```

## 20. Boss Folios and Editor arenas

### 20.1 Layout

`room.rule.boss_spine` (§7.2). The Editor checkpoint is safe and offers one campfire rest per Erratum, a Rewrite lectern, and the boss proof (the boss's resistances and counters, doc 13). Erasure pauses while the boss lives (Canon §11.6).

### 20.2 Arena generation (`room.boss_arena`, 3 × 3 cells × 2 storeys)

1. **Shape** by family: fantasy, horror and mythic use a circle of radius 22 blocks; sf, crime and meta a 44-block square; historical a cross of two 44 × 20 bars.
2. **Tiers** `t = min(3, 1 + floor_div(vert_bp, 2000))`: rings at +0, +4 and +8 blocks, 6 blocks wide, with slab ramps at the four cardinal points.
3. **Pillars** `p = 4 + floor_div(cover_bp, 1000)`, each 2 × 2 × 6 blocks at radius 12, pillar `k` at yaw `floor_div(65536 × k, p) + phase` with `phase = range(65536)` (1 draw). Example: Space Opera has `p = 10`.
4. **Hazard pockets:** the first two visible hazards of the genre list at radius 17, at `phase` and `phase + 32768`.
5. **Light:** `bright_bp = 6000`, fixtures on pillars.
6. **Phase transforms**, triggered by doc 13's boss phase change: at phase 2 (boss HP ≤ 6667 Bp) the lowest tier floods with `liq.red_ink` 500 mm deep when `t >= 2`, else 4 `hazard.ink_seep` pockets appear; at phase 3 (HP ≤ 3333 Bp) odd-index pillars collapse and their fixtures go dark, opening spawn-eligible cells for doc 13's adds.
7. The north door to the Threshold opens on `ev.entity_killed` of the boss.

### 20.3 The Ur-Text (Folio 25 Standard, Folio 15 Abridged)

The arena is a radius-22 circle cut into 12 wedges; wedge `k` spans yaw `floor_div(65536 × k, 12) ..< floor_div(65536 × (k + 1), 12)` and uses core genre `k`'s tileset and materials (lattice order). Local genre is `genre.ur_text` everywhere, so `C = 0` and `E = 0` (Canon §5.5). Approach rooms rotate through core tilesets starting at core genre `range(12)` (1 draw). There are no bleed zones.

## 21. Marginalia modifiers

From Folio 26, each Folio has `k = min(4, 1 + floor_div(n - 26, 10))` modifiers: 1 at Folios 26–35, 2 at 36–45, 3 at 46–55, 4 from 56. They are drawn in stage `marg` without replacement (k weighted calls) from the list below with excluded genres at weight 0, and they appear in the exit preview.

| ID | w | Excluded | Effect |
|---|---|---|---|
| `lattice.marg.annotated` | 60 | — | +2 secrets |
| `lattice.marg.bleeding` | 80 | RD, UW, UT | drift bleed certain, +1 drift genre |
| `lattice.marg.brittle` | 60 | — | `hard_bp` × 5000 |
| `lattice.marg.champions` | 60 | — | +1 champion aspect (max 3); doc 13 doubles champion share |
| `lattice.marg.dense` | 100 | — | `marg_pop_bp = 12500` |
| `lattice.marg.erased` | 40 | — | +3 `hazard.ink_seep` |
| `lattice.marg.flooded` | 60 | UW | basin chance 10000; +2 basins per cistern |
| `lattice.marg.sealed` | 40 | UW | no shop; `marg_loot_bp = 12500` |
| `lattice.marg.silent` | 60 | — | `sound_bp` × 7000 (sound travels farther) |
| `lattice.marg.trapped` | 80 | — | `trap_bp` × 20000 |
| `lattice.marg.unlit` | 80 | — | `bright_bp` × 5000; `spawn_block_max + 2`, capped at 15 |
| `lattice.marg.volatile` | 60 | UW | `fire_bp` × 15000; +1 flame jet or oil slick per 4 rooms |

Examples (run seed `0x0123456789ABCDEF`): Folio 26 Gothic Horror has `[annotated]`; Folio 36 Mythic Fantasy has `[annotated, champions]`; Folio 36 Unwritten has `[dense, unlit]`. Marginalia boss Folios are every `n` with `(n - 25) % 5 == 0`. Depth tables follow Canon §13.8 with `d = n`.

## 22. Abridged mode layout (Canon §17.1)

- Folio `n ∈ 1..=15`: Volume `v = ceil_div(n, 3)`, position `f = (n - 1) % 3 + 1`, `d = d_eff(v, f) = (v - 1) × 5 + [1, 3, 5][f - 1]`: depths 1, 3, 5, 6, 8, 10, 11, 13, 15, 16, 18, 20, 21, 23, 25.
- `f = 1`: standard. `f = 2`: standard 8500, challenge 1500; never rest. `f = 3`: boss. Folio 15 is the Ur-Text.
- Regular Folios use `rooms_min + 2 ..= rooms_max + 2` and always contain a shop and a Scriptorium; `f = 2` always contains a Fortune Engine hall.
- Streams, EntityIds and `folio_index` use `n`; every depth-driven quantity (power tables, `ilvl`, aspects, drift chance) uses `d`. Loot quantity × 16667 Bp and kill XP × 16667 Bp are applied by docs 11 and 13 (Canon §17.1), not by worldgen.
- Volume rule over 3-Folio Volumes; secret exits only from `f = 1`; no Marginalia layout in Abridged.

## 23. Determinism, tests and time budget

### 23.1 Determinism rules

1. `generate_folio` depends only on `GenInput` and `ContentDb`. Same inputs give the same `blueprint_hash` on every platform (Canon §21 test 31).
2. Every draw names its sub-stream (§6.2). Per-room and per-chunk work uses `item_rng`, so `par.map` order cannot change results; results merge in ascending index.
3. Lists are sorted before any `range` or `weighted` selection, by the key named at the selection site; ties go to the lower index.
4. No `HashMap` or `HashSet`; `BTreeMap`, sorted `Vec` and fixed arrays only (Canon §1.11).
5. Plan functions are recomputed, never cached across Folios; the exit preview and the next Folio's generation call the same code.
6. Event scheduling uses the sequential `rng.event` on the host only.

### 23.2 CI

- **PR sweep:** 50 seeds × 16 genres × 4 Folio types (3200 Folios); every Folio passes V1–V12 within 3 attempts, and fallback use stays under 1 per 1000.
- **Nightly sweep:** 1000 seeds × 16 × 4 (64,000 Folios) with time percentiles recorded; p50 above 110 ms or p99 above 260 ms on the reference Steam Deck runner fails the build.
- **Golden blueprints:** 32 committed `(GenInput, blueprint_hash)` pairs; any change requires a deliberate re-bless.
- **Cross-platform:** the 32 goldens on x86_64 Linux, aarch64 Linux, x86_64 Windows and wasm32 give identical hashes.

### 23.3 `st.contradicted` (owner 12)

The indicator is present on an Erratum exactly while at least one equipped component has `C >= 3` (Canon §11.1). It is re-evaluated on every `ev.contradiction_changed` for that bearer, so crossing a bleed-zone face updates it on that tick. It has no effect besides the UI.

## Exports

- `genre.*`: the full GPV of all 16 genres (§3), including the §2 identity values.
- `lattice.`: `lattice.node.margin`; `lattice.edge.<from_suffix>.<to_suffix>` for every nonzero edge of §4.2–§4.3; `lattice.folio.standard`, `.boss`, `.rest`, `.challenge`; `lattice.challenge.gauntlet`, `.siege`, `.heist`, `.duel`, `.race`; `lattice.marg.annotated`, `.bleeding`, `.brittle`, `.champions`, `.dense`, `.erased`, `.flooded`, `.sealed`, `.silent`, `.trapped`, `.unlit`, `.volatile`.
- `gpv.`: `gpv.layout.keep`, `.manor`, `.labyrinth`, `.ship`, `.arcology`, `.ruins`, `.frontier`, `.streets`, `.terraces`, `.foundry`, `.woodland`, `.temple`, `.dream`, `.office`, `.blank`, `.palimpsest`; `gpv.law.non_euclidean`, `.dream_doors`, `.gravity_flip`, `.no_bleed`, `.editors_only`, `.pure_reading`; `gpv.evt.light_failure`, `.weather_front`, `.distant_noise`, `.tremor`, `.whispers`, `.boon`, `.lore_echo`, `.editor_glimpse`, `.gravity_shift`, `.wandering_vendor`, `.patrol_shift`, `.ambush`, `.reinforcements`, `.plea`, `.duel_challenge`, `.flood`, `.fire_outbreak`, `.lockdown`, `.redaction_sweep`; the 15 `gpv.story.*` templates of §19.2.
- `room.`: the 36 kinds of §15.1; tags `room.no_mount`, `room.safe`, `room.stealth`, `room.no_bleed`, `room.electronic`; templates `room.tpl.*` including `room.tpl.fallback.<layout>`; the 23 rules `room.rule.*` of §7.2.
- `biome.`: the 32 tilesets of §3.3 (`biome.mythic_fantasy.castle` … `biome.ur_text.colophon`).
- `hazard.`: `dart_trap`, `blade_trap`, `spike_pit`, `crusher`, `flame_jet`, `gas_vent`, `shock_floor`, `alarm`, `net_snare`, `rune_ward`, `curse_glyph`, `camera`, `collapsing_floor`, `falling_debris`, `lava_pool`, `acid_pool`, `deep_water`, `oil_slick`, `ink_seep`, `steam_vent`, `rad_zone`, `miasma_bog`, `web_field`, `quicksand`, `hull_breach`, `redaction_bar`, `chasm`, `gravity_flip` (28).
- `secret.`: rooms `cache`, `shrine`, `reliquary`, `armory`, `editors_desk`, `margin_note`, `oubliette`, `ink_well`; exits `secret.exit.oneiric`, `.redacted`, `.unwritten`, `.ur_text`; hides `cracked_wall`, `false_wall`, `lever`, `buried`, `waterfall`, `ceiling_hatch`, `verb_lock`, `dream_veil`, `clearance`, `blank_page`, `palimpsest_wall`.
- Behavior owned per the canon: `rng.lattice`, `rng.worldgen` (with §6.2 sub-streams), `rng.event`, `st.contradicted`, `cat.quest` usage.
- Proposed outside this chapter's namespaces (Canon gaps G2): 32 `mfr.*` IDs (§2.4).

## Interface assumptions

- **I1 (doc 03):** block light from a level-`L` source is `L - k` at taxicab distance `k` through non-opaque cells, and worldgen calls the same function; `light_sky` = local `ambient_light` in every air cell; doc 03 applies `fire_bp`, `flow_bp`, per-cell gravity sign, relink portals, and instantiates blueprint records in §6.1 order.
- **I2 (doc 16):** peers compare `blueprint_hash` at Folio entry; a mismatch is a desync.
- **I3 (doc 07):** the `mat.*` rows of §2.4, `block.light_fixture` per genre, `block.reinforced` (unbreakable), `block.web`, slab shapes, tileset palette blocks; `hard_bp` on break time.
- **I4 (doc 08):** `liq.water`, `.blood`, `.alcohol`, `.oil`, `.acid`, `.lava`, `.red_ink`, `gas.miasma`, `gas.steam`; the automaton starts from worldgen's settled liquids.
- **I5 (doc 13):** converts spawner budgets with the §11 rank costs; gives every champion a plan aspect; honors `no_ambient_spawns`; spawns event encounters; owns captives, hirelings and boss phases (signalling phase changes for §20.2); respects `ACTIVE_ACTOR_CAP`.
- **I6 (doc 11):** `loot_quota(run_seed, folio_index, mode)` is pure; fills containers (`rng.loot_place`, keyed `rng.loot`); Emendations in guaranteed containers; the story relic; `item.quest.folio_leaf`; Reliquary keys; shop stock; the Lattice Lens; Rewrite prices.
- **I7 (doc 09):** `lock.*` for Locked edges and the clearance lock; Security disarming; camera alarms; `sound_bp` and `fog`.
- **I8 (doc 05):** `proj_bp` at launch; dismount on entering `room.no_mount`; no summoning there.
- **I9 (doc 06):** patron altars; +100 favor on a secret shrine's first prayer.
- **I10 (docs 04, 14, 17, 18):** Inscription XP for reading (04); `mode.*` and Codex records (14); exit UI, preview display, 90-tick minimum transition (17); naming and story grammars, `loc.*` skins, seams, palettes, music (18).

## Canon gaps

- **G1** Canon §12.3 and §13.9 cite §5.9 and the changelog lists its contents, but the file has no §5.9 body. Assumed: §2 is that table; the canon adopts §2.1–§2.4 verbatim.
- **G2** §5.9 requires two manufacturers per genre, but `mfr.` is doc 11's and `mat.` doc 07's. Assumed: the 32 `mfr.*` IDs of §2.4 are proposals for doc 11; material names are requests to doc 07 (I3).
- **G3** Canon §3.2 defines stream seeds but no sub-streams. Assumed: §6.2 derivation from `rng.worldgen`; previews recompute Folio `n + 1` seeds and advance no live stream.
- **G4** `FOLIO_1_GENRE` is the host's Origin, but an Origin may be secret and Ur-Text is the finale. Assumed: §4.6 maps secret Origins to a core genre.
- **G5** "lattice edge weights (doc 12)" leaves freshness open. Assumed: effective weight = edge × freshness (§4.4).
- **G6** The repeat rule does not name secret genres. Assumed: each at most once per run before Marginalia, via dormant portals (§16.4).
- **G7** The changelog puts physics multipliers in §5.9; Canon §4 names only gravity. Assumed: §2.1 multipliers are applied by docs 03–09; no Canon §4 constant changes.
- **G8** Canon §4.2 measures falls along "the current gravity axis" without a per-cell gravity sign. Assumed: Oneiric flip cells reverse gravity along Y (§8.5).
- **G9** Abridged mode is silent on Marginalia and stream indices. Assumed: streams and EntityIds use `n ∈ 1..=15`, depth uses `d_eff`, no Marginalia.
- **G10** R6 proposes `law.*` and per-axiom-maximum `zone.seam`; §19 assigns neither and §5.8 keeps `local_genre` single-valued. Assumed: `gpv.law.*`; no seams.
- **G11** Canon §5.7 does not list what is "electronic". Assumed: `hazard.shock_floor`, `hazard.camera`, `gpv.evt.lockdown`, the clearance lock, `room.server_room` (`room.electronic`); doc 13 turrets inherit the rule.
- **G12** "always exactly 3 drawn at the exit of each Folio" covers exits into forced Folios. Assumed: Folios 24 (Standard) and 14 (Abridged) draw and store 3 unseen candidates.

## Acceptance tests

`R = 0x0123456789ABCDEF`; abbreviations per §3.3.

1. `stream_seed(R, "rng.lattice", 1) = 0xb93f5f6ccba93fac`; `lattice_candidates(R, 1, CP, [CP])` has totals 6,450,000 / 5,250,000 / 3,900,000, draws 1,106,450 / 3,525,875 / 3,019,215, result `[SO, HN, SP]`.
2. `stream_seed(R, "rng.lattice", 0) = 0xa487e0eae90fea26`; seeded Folio 1 candidates `[MF, HN, SP]` → Mythic Fantasy. Daily 2026-10-11 → `[WW, CH, MF]`.
3. Over 10,000 random legal Standard, Abridged and Marginalia histories: 3 distinct candidates; never current, same-Volume, capped or secret; exactly 3 `weighted` calls.
4. `gen_seed(R, 2, genre.space_opera) = 0x1c35d41d581f5eaa`; `stream_seed(R, "rng.event", 2) = 0x9ad6720d21d175fe`, first `range(10000)` = 2892.
5. `n = 2`: SO standard (3860), HN rest (9855), SP rest (9596). `n = 4`: CP rest, MF standard. Boss at 5, 10, 15, 20, 25, 30, 35. Abridged GH `n = 1..=15`: boss at 3, 6, 9, 12, 15, others standard.
6. Aspects: `n = 2` SO `[surging]`, HN `[unstoppable]`; `n = 12` MF `[unstoppable, surging]`, CP `[warded, surging]`.
7. Bleed at `n = 7`, prev MF: GH entry MF, no drift; ON entry MF, drift `[MA, GH]`; RD none.
8. Marginalia `k` at 26, 35, 36, 45, 46, 56, 100 = 1, 1, 2, 2, 3, 4, 4; Folio 26 GH `[annotated]`; 36 MF `[annotated, champions]`; 36 UW `[dense, unlit]`.
9. For each candidate of tests 1–2, the preview plan equals the generated `plan`.
10. Abridged `d_eff`: 1, 3, 5, 6, 8, 10, 11, 13, 15, 16, 18, 20, 21, 23, 25.
11. `protect_r`: MF 10, GH 5, CH 4, SO 12, CP 9, PA 7, WW 9, HN 7, WX 8, SP 10, FT 6, MA 10, ON 4, RD 13, UW 7, UT 9; flat-floor coverage MF 145, GH 25.
12. Population: SO 33,000; GH 27,000; SP challenge 39,375 mSKE; 33,000 over `[4, 2, 5, 2, 1, 3, 0, 2]` → `[6947, 3473, 8688, 3473, 1736, 5210, 0, 3473]`.
13. Dart `d = 10`: 9,714 mHP (Erratum), 13,030 (NPC); crusher `d = 15`: 52,900; flame pulse `d = 5`: 9,910.
14. Loot target 24 accepts 21..=27, rejects 20 and 28.
15. Typing weights: CP server room 128, MF library 126, WX hall 480, RD garden 0.
16. Lints L1–L6 pass for all 16 records (RD L1 margin 0); setting GH `tec = 4` fails L2.
17. Folio 7 with the same `GenInput` after two different histories has the same `blueprint_hash`.
18. Single-threaded and 4-thread `par` give identical hashes for 100 seeds.
19. 50 seeds × 16 genres × 4 types pass V1–V12 within 3 attempts; fallback used < 4 times in 3,200.
20. Key behind its own lock fails V2; a sealed pit fails V4; a camera in a Mythic Fantasy zone fails V8; lava 5 blocks from the Entry spawn fails V6.
21. Every feet cell of every swept safe room has `light_block > spawn_block_max`.
22. 2 blocks of path headroom → `room.no_mount`; 3 blocks everywhere → untagged.
23. An Oneiric flip room has clear height 4; entering deals no fall damage.
24. Oneiric visited at Folio 4 → an Oneiric exit generated at Folio 9 is dormant with a 3 LP chest; no secret exit when `n + 1` is boss or forced.
25. A 360 s Gothic Horror Folio runs 17 event checks, fires ≤ 6 events; draws per check 1, 3, or 5 (ambush).
26. Ur-Text wedges `k = 1, 6, 11` start at 5461, 32768, 60074 Bam.
27. Collapsed WFC grids satisfy every face constraint; a contradictory tileset ends in `fallback_fill` after exactly 3 resets.
28. Nightly sweep: p50 ≤ 110 ms, p99 ≤ 260 ms (Steam Deck runner), max ≤ 900 ms.

## Invariants annex

```
Σ doc12: gpv ⊗ lattice ⊗ ftype ⊗ graph ⊗ embed ⊗ wfc ⊗ light ⊗ bleed ⊗ validate
W! generate_folio pure(GenInput, ContentDb)          N! party size|account|clock|float → generation
W! gpv Block A == Canon §5.2                          W! identity table §2 → every GPV
W! draw ∈ sub-stream(rng.worldgen | rng.lattice | rng.event) N! live stream advanced by preview
W! lattice: exactly 3 weighted calls per exit         W! candidates ∉ {cur, same-Volume, capped, secret}
W! Folio 25 = ur_text (Abridged 15)                   W! boss ⇔ is_boss(n, mode)
W! preview.plan == generated.plan                     W! champion ∈ plan.aspects
W! exit reachable ∧ key before lock ∧ no softlock     W! V1..V12 before publish
W! attempts ≤ 4 then authored fallback                N! unbounded retry
W! safe room feet: light_block > spawn_block_max      W! spawn eligibility reads light_block only
W! local_genre per cell from zone_of_cell             N! bleed in redacted|unwritten|ur_text
N! electronic thing where local TEC ≤ 2               N! dep axis with local axiom 0
W! hazard dmg := Canon §10.6 (P vs NPC, T vs Errata)  W! pop budget sums exactly
W! LP ∈ target ± 10%                                  W! anchors ≤ 96
W! room.no_mount ⇔ kind tag ∨ headroom < 3 blocks     W! gravity-flip fall < SAFE_FALL
W! secret genre ≤ 1 per run before Marginalia         W! dormant portal still pays 3 LP
W! iteration ascending (NodeId, room, record, block)  N! HashMap|HashSet in gen state
W! story text cosmetic                                N! LLM text → blueprint
```
