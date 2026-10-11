# 12 — World Generation and Genres
As of 2026-10-11. Status: BINDING. Conforms to canon-1.2.

This chapter owns everything that turns a run seed into a playable Folio: the full Genre Parameter Vector (GPV) of all 16 genres, the World Laws, the Genre Lattice and its exit offers, Folio types, the generation pipeline from mission graph to validated voxel blueprint, bleed and seam zones, the room catalog, secrets, hazards, events, Folio story fragments, boss arenas, Marginalia modifiers and the Abridged layout. Generation is one pure function in `crate.gen` (doc 02). The canon on disk is canon-1.4, and every rule here was checked against it, including fixed terrain, Canon §5.9 identity, World Laws, seams, `st.unmoored`, the Colophon and invasions (Canon gaps G1).

## Imports

| Source | Consumed |
|---|---|
| DESIGN_BRIEF §1–§4; DECISIONS.md 15–16 | Archetype × Skin, axioms, 16 genres, 5 × 5 Folios, Lattice exits, Marginalia; limited building; invasions |
| Canon §1–§3 | IDs, integer content, `floor_div`, `mul_bp`, Folio box, `EntityId`, `light_at`, `Rng`, `stream_seed`, the streams `rng.lattice`, `rng.worldgen`, `rng.event` (owner 12), `rng.spawn`, `rng.loot_place`, `rng.stealth` |
| Canon §4, §4.3 | extent, footprint 96–192, capsule, step-up, `SAFE_FALL`, caps; fixed terrain, `destructible`, placeables, resource nodes |
| Canon §5.2–§5.11 | genre table (copied in §3.2); `dep` and the zero-axis ban; Contradiction; TEC ≤ 2 rule; `local_genre`; seam zones; Canon §5.9 identity (copied in §2); World Laws (listed in §4); Going Native; Unmoored |
| Canon §6.4 row 35, §10.6, §11, §12 | passive detection; environmental hazards; `st.contradicted`, `st.unmoored`, Erasure; surfaces, concealment, spawn light |
| Canon §13, §15–§19 | `P`/`T`, Palimpsestic sources; factions, patrons; run constants, Colophon, Abridged, aspects, mounts, invasions; events; this chapter's namespaces |
| Doc 02 §3 | `crate.gen`, `generate_folio(input, db, par)`, `OrderedMap` |
| Doc 03 §4.7, §5, §10 | `protect` and `open_tag` bits, sky and block light channels, `gsign`, `ambient_decic` |
| Doc 08 | `liquid_viscosity_bp`, `gas_dissipation_bp`, H/F/M nodes, font basins, red-ink pools |
| Doc 11 §9.6–§9.10, §10 | chest count, floor loot, guarded spots, secret loot tables, Reliquary spots, counter caches |
| Doc 13 §2.5, §7.3, §8, §9, §10, §12 | `damage_affinity`, the aspect set, rosters, boss arenas, Director placement, territory, war fronts, dynamic bleed zones |
| Doc 14 §3, §5.3, §7, §8 | positions, Incipit, Rubric, Colophon, Interleaf, Endpaper, exit vote, room unlocks, `folio_scale_bp`, `gp.ambush_extra`, bleed-count hooks |
| R6 §7–§11; R4 §7, §14; R1 §5.9; R2 §5 | GPV, Noita modifiers, Torg Laws; WFC in rooms, Dormans cycles; Qud sultans; Minecraft light |

## 1. Vocabulary and namespaces

| Term | Exact meaning |
|---|---|
| Cell | 16 × 8 × 16 blocks; cell `(cx, cs, cz)` covers `x ∈ 16cx..16cx+16`, `y ∈ 8+8cs..16+8cs`, `z ∈ 16cz..16cz+16`; 16 × 6 × 16 cells; `cs ∈ 0..=5` is the **storey**; `y ∈ 0..=7` is foundation, `y ∈ 56..=63` roof |
| Footprint | the genre's `W × D × storeys` cells, centered: `x0 = 16 × floor_div(16 - W, 2)`, likewise `z0` |
| Tile | a WFC unit of 4 × 4 × 4 blocks; a cell holds 4 × 2 × 4 tiles |
| Room | a mission-graph node embedded as an axis-aligned box of whole cells |
| Zone | cells sharing one zone record: plain (Folio genre), bleed, seam or dynamic (§14) |
| Feet cell | an air cell with solid or slab-top support below and 2 air cells above (§15.1) |
| Path distance | breadth-first step count between feet cells on the §15.1 navigation graph × 1000 mm |
| `d`, `pos` | depth and structural position of a Folio (doc 14 §3.1) |

This chapter owns full `genre.` vectors and the namespaces `gpv.`, `lattice.`, `room.`, `biome.`, `hazard.`, `secret.`, `law.` and `zone.` (Canon §19), plus the behavior of `rng.lattice`, `rng.worldgen`, `rng.event`, `st.contradicted`, `st.unmoored` (§24.3) and `cat.quest` items placed by worldgen (§20.5).

## 2. Genre identity (Canon §5.9)

### 2.1 Canon table, copied exactly

| Genre | air_accel_bp | fall_dmg_bp | ground_friction_bp | spawn_block_max | Palette | Music register | Enemy themes | Materials |
|---|---|---|---|---|---|---|---|---|
| mythic_fantasy | 10000 | 10000 | 10000 | 0 | `#3B6E3B #C9A227 #8FB3D9` | horns, strings, choir | orcs, dragons, wyverns | `mat.steel` `mat.moonsilver` `mat.oak` `mat.dragonbone` |
| gothic_horror | 10000 | 10000 | 10000 | 7 | `#3A1F2B #7A0E14 #C8C3B4` | organ, string quartet, harpsichord | vampires, werewolves, ghouls | `mat.silver` `mat.blackwood` `mat.bone` |
| cosmic_horror | 8000 | 12500 | 10000 | 7 | `#0E2A2E #4F7F6A #B8E986` | drones, detuned strings, theremin | cultists, deep things, formless spawn | `mat.obsidian` `mat.star_metal` `mat.chitin` |
| space_opera | 12500 | 7500 | 10000 | 0 | `#0B1026 #3D7BFF #FF8A3D` | synth-orchestral, brass | boarders, xeno hives, war droids | `mat.plasteel` `mat.titanium` `mat.psi_crystal` |
| cyberpunk | 11000 | 10000 | 10000 | 0 | `#120B1F #FF2E88 #00E5FF` | darksynth, industrial | gangers, corporate security, rogue AIs | `mat.carbon_fiber` `mat.polymer` `mat.nanoweave` |
| post_apocalypse | 10000 | 10000 | 9000 | 0 | `#6B4E2E #C2A36B #E04E1B` | distorted guitar, junk percussion | raiders, mutants, scrap golems, rad-cults | `mat.scrap_iron` `mat.rubber` `mat.rad_glass` |
| weird_west | 11000 | 10000 | 10000 | 0 | `#8C4A2F #E0B776 #2F3E4E` | slide guitar, whistle | outlaws, revenants, sand wyrms | `mat.iron` `mat.leather` `mat.hexed_silver` |
| hardboiled_noir | 8000 | 12500 | 8500 | 0 | `#101010 #6E6E6E #C22B2B` | smoky jazz, muted trumpet | mobsters, crooked cops, hired killers | `mat.steel` `mat.brass` `mat.tweed` |
| wuxia | 20000 | 5000 | 10000 | 0 | `#1F3A2E #D9C9A3 #B32020` | guzheng, dizi, erhu, taiko | rival sects, jiangshi, fox spirits | `mat.jade` `mat.bronze` `mat.silk` |
| steampunk | 10000 | 10000 | 10000 | 0 | `#4A3A28 #B5793A #5FA8A0` | brass band, calliope | automata, sky pirates, clockwork beasts | `mat.brass` `mat.copper` `mat.aetherium` |
| fairy_tale | 12500 | 7500 | 10000 | 0 | `#F2D7E8 #6FBF73 #E8C547` | celesta, woodwinds, waltz | wolves, witches, trolls | `mat.cold_iron` `mat.glass` `mat.elderwood` |
| mythic_antiquity | 12500 | 10000 | 10000 | 0 | `#F0E6D2 #2E5A88 #C9792B` | lyre, aulos, chant | shades, hydras, harpies | `mat.bronze` `mat.orichalcum` `mat.marble` |
| oneiric | 15000 | 2500 | 10000 | 0 | `#2B1B4D #8E7CFF #FFC2E0` | reversed piano, choir pads | nightmares, doppelgangers, sleepwalkers | `mat.dreamstuff` |
| redacted | 6000 | 15000 | 10000 | 15 | `#F2F2F2 #1A1A1A #D7262E` | office hum, sine tones | agents, anomalies, containment drones | `mat.redacted_alloy` |
| unwritten | 10000 | 10000 | 10000 | 15 | `#FFFFFF #E6E6E6 #CCCCCC` | silence | Editors only | `mat.vellum` |
| ur_text | 10000 | 10000 | 10000 | 0 | `#1C1408 #E8D9B0 #B8862B` | every register in counterpoint | every roster | `mat.vellum` `mat.gold_leaf` |

Doc 03 applies the physics columns; `spawn_block_max` gates ambient, wave and initial spawns (Canon §12.3, doc 13 §10.2).

### 2.2 Canon manufacturers, copied exactly

| Genre | Manufacturer IDs and signature trait |
|---|---|
| mythic_fantasy | `mfr.mythic_fantasy.deepanvil`: poise damage × 15000, attack speed × 9000. `mfr.mythic_fantasy.silverbough`: attack noise −20 dB. |
| gothic_horror | `mfr.gothic_horror.vesper_sons`: 2000 Bp of each hit converted to `dmg.radiant`. `mfr.gothic_horror.grimhallow`: instant reload for 3% Max HP. |
| cosmic_horror | `mfr.cosmic_horror.hollow_lantern`: hits apply `st.marked` for 4 s. `mfr.cosmic_horror.outer_choir`: 2500 Bp `dmg.psychic` share, +1500 Bp damage, costs 1 composure per shot. |
| space_opera | `mfr.space_opera.perihelion`: a 30-tick charged shot deals × 15000. `mfr.space_opera.umbra`: no magazine; heat; overheating locks the weapon for 90 ticks. |
| cyberpunk | `mfr.cyberpunk.ghostline`: projectiles home on `st.marked` targets at 16384 Bam/s. `mfr.cyberpunk.neon_saint`: charged shots pierce 1000 mm of blocks. |
| post_apocalypse | `mfr.post_apocalypse.rustmaw`: magazine × 20000, durability × 5000. `mfr.post_apocalypse.lastditch`: 2000 Bp chance per shot to use no ammo. |
| weird_west | `mfr.weird_west.coffin_creek`: crits ricochet 5000 Bp of their damage to the nearest other hostile within 8 m. `mfr.weird_west.seven_seals`: 2000 Bp `dmg.umbral` share with dependency mythos. |
| hardboiled_noir | `mfr.hardboiled_noir.midnight_avenue`: shot noise capped at 50 dB. `mfr.hardboiled_noir.rainslick`: × 12000 against staggered or knocked-down targets. |
| wuxia | `mfr.wuxia.jade_lotus`: thrown weapons return after 30 ticks. `mfr.wuxia.iron_crane`: a block within 6 ticks of impact reflects 5000 Bp of the damage. |
| steampunk | `mfr.steampunk.brassworth`: +200 Bp per consecutive hit, maximum +3000. `mfr.steampunk.aetherline`: every 4th hit chains 3000 Bp of its damage as `dmg.shock` to 2 targets within 5 m. |
| fairy_tale | `mfr.fairy_tale.thimbleweald`: × 13000 against targets with a larger capsule radius than the wielder. `mfr.fairy_tale.thornwood`: 1000 Bp chance per hit to apply `st.rooted`. |
| mythic_antiquity | `mfr.mythic_antiquity.hephaesteum`: no durability loss; immune to `st.disarmed`. `mfr.mythic_antiquity.argive`: +1500 Bp damage while an allied Erratum is within 3 m. |
| oneiric | `mfr.oneiric.somnambule`: projectiles pass through blocks after 10 m of flight. |
| redacted | `mfr.redacted.bureau`: never rolls Contradiction failure; the potency loss still applies. |
| ur_text | `mfr.ur_text.first_press`: Palimpsestic items only. |

### 2.3 Identity extensions owned here

`fixture_level`: `light_emit` of a worldgen fixture. `protect_r = fixture_level - spawn_block_max - 1`: radius one fixture makes spawn-proof (— where `spawn_block_max = 15`). `fog`: concealment per 8 m (Canon §12.2). `ambient_decic`: temperature (doc 03). Viscosity and gas dissipation feed doc 08; `damage_affinity` adds to native NPC resistances (doc 13 §2.5).

| Genre | fixture_level | protect_r | fog | ambient_decic | liquid_viscosity_bp | gas_dissipation_bp | damage_affinity (Bp) |
|---|---|---|---|---|---|---|---|
| MF | 14 | 13 | 0 | 150 | 10000 | 10000 | arcane +1500, fire +1000 |
| GH | 12 | 4 | 1 | 80 | 9000 | 8000 | umbral +2000, radiant −1500 |
| CH | 12 | 4 | 1 | 60 | 7000 | 7000 | psychic +2500, umbral +1000 |
| SO | 15 | 14 | 0 | 210 | 10000 | 15000 | radiation +1500, shock −1000 |
| CP | 13 | 12 | 0 | 220 | 11000 | 10000 | toxin +1000, shock −1500 |
| PA | 12 | 11 | 0 | 350 | 8000 | 12000 | radiation +2500, acid +1000 |
| WW | 13 | 12 | 0 | 320 | 7000 | 13000 | fire +1000, umbral +1000 |
| HN | 12 | 11 | 1 | 120 | 12000 | 9000 | pierce +1000, psychic −1000 |
| WX | 12 | 11 | 0 | 170 | 12000 | 11000 | slash +1000, sonic −1000 |
| SP | 14 | 13 | 0 | 260 | 10000 | 8000 | fire +1500, shock −1000 |
| FT | 11 | 10 | 1 | 160 | 10000 | 10000 | arcane +2000, slash −500 |
| MA | 13 | 12 | 0 | 290 | 10000 | 12000 | radiant +1000, blunt +1000 |
| ON | 10 | 9 | 1 | 200 | 15000 | 10000 | psychic +2000, arcane +1000 |
| RD | 15 | — | 0 | 210 | 10000 | 15000 | psychic +2000, sonic +1000 |
| UW | 8 | — | 0 | 200 | 10000 | 10000 | none |
| UT | 14 | 13 | 0 | 200 | 10000 | 10000 | none |

Abbreviations in Canon §5.2 order: MF, GH, CH, SO, CP, PA, WW, HN, WX, SP, FT, MA, ON, RD (Redaction), UW (Unwritten), UT (Ur-Text).

### 2.4 Placed light as defense

1. **Sky light** (doc 03 §10.2) comes only from sky-exposed cells and rooms tagged open (`open_tag`) at the local `ambient_light`, falling 1 per step; enclosed rooms are lit by fixtures, which shape the stealth gradient.
2. **Fixtures** (`block.light_fixture`, `light_emit = fixture_level`) are `destructible` (Canon §4.3); breaking one makes its cells spawn-eligible on the next relight.
3. **Protection:** a cell at taxicab distance `k` is spawn-proof when `fixture_level - k > spawn_block_max`, i.e. `k <= protect_r`. Mythic Fantasy `14 - 0 - 1 = 13`: a fixture 2 blocks above the feet layer covers `|dx| + |dz| <= 11`, `2 × 11 × 12 + 1 = 265` cells. Gothic Horror `12 - 7 - 1 = 4` covers 13: candlelight barely holds the dark.
4. **No light is safe** in the Redaction and the Unwritten (`spawn_block_max = 15`); only `no_spawn` (§12) clears their safe rooms.
5. **Placeable light** (Canon §4.3) protects the same way; `fac.editors` members erase Errata placeables within 1500 mm (Canon §11.6).
6. **Safe rooms** carry `no_spawn` and, where `spawn_block_max <= 14`, keep every feet cell above it (§11.3).

## 3. Genre Parameter Vector (GPV)

### 3.1 Dimensions

A GPV is one content record `genre.<suffix>` with 64 dimensions. All are simulation inputs (integers or IDs) except those marked P (presentation only).

- **A, Canon §5.2 (12):** 1 `id`; 2 `display` (P); 3–6 `arc`, `tec`, `myt`, `vrv`; 7 `aether_axiom`; 8 `gravity_bp`; 9 `ambient_light`; 10–11 `lucre_skin`, `aether_skin` (P); 12 `tier`.
- **B, Canon §5.9 (9):** 13 `air_accel_bp`; 14 `fall_dmg_bp`; 15 `ground_friction_bp`; 16 `spawn_block_max`; 17 `palette` (P); 18 `music` (P); 19 `enemy_themes` (P); 20 `materials`; 21 `manufacturers`.
- **C, identity extensions (10, §2.3):** 22 `family` (P); 23 `era_tl` (P, `0..=12`); 24 `render_style` (P); 25 `fixture_level`; 26 `fog`; 27 `ambient_decic`; 28 `liquid_viscosity_bp`; 29 `gas_dissipation_bp`; 30 `damage_affinity`; 31 `patron`.
- **D, structure (19):** 32 `layout`; 33–35 `fp_w`, `fp_d`, `storeys`; 36–37 `rooms_min`, `rooms_max`; 38 `loop_bp`; 39 `vert_bp`; 40 `corridor_w`; 41 `cover_bp`; 42–43 `bright_bp`, `dark_bp`; 44 `secret_bp`; 45 `trap_bp`; 46 `liquids`; 47 `tilesets`; 48 `cycle_w`; 49 `room_bias`; 50 `node_w` (H, F, M resource-node weights).
- **E, population and play (7):** 51 `roster` and 52 `factions` (references to doc 13 §8); 53 `density_bp`; 54 `verbs` (10 × Bp: shoot, melee, cast, pray, sneak, talk, hack, craft, build, traverse); 55 `hazards`; 56 `events`; 57 `event_bp`.
- **F, bleed, Laws and story (7):** 58 `bleed_ok`; 59 `drift_bp`; 60 `bleed_in_bp`; 61 `seam_bp`; 62 `secret_exits`; 63 `laws` (§4); 64 `story_domains`.

`family`/`era_tl`/`render_style`: MF fantasy/3/painterly, GH horror/5/painterly, CH horror/6/ink, SO sf/11/pbr, CP sf/9/pbr, PA sf/8/pbr, WW historical/5/toon, HN crime/6/ink, WX historical/3/ink, SP historical/5/pbr, FT fantasy/2/toon, MA mythic/1/painterly, ON meta/6/xerox_collage, RD meta/8/pixel_dither, UW meta/0/ink, UT meta/6/painterly. `patron` is `patron.<suffix>`.

### 3.2 Block A: Canon §5.2 columns, copied exactly

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

### 3.3 Block D: structure

| # | layout | fp_w × fp_d × storeys | rooms | loop_bp | vert_bp | corr_w | cover_bp | bright_bp | dark_bp | secret_bp | trap_bp | tilesets (`biome.<suffix>.*`) |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| MF | keep | 10 × 10 × 3 | 16–20 | 1500 | 2000 | 3 | 3000 | 3500 | 2500 | 1500 | 2500 | `castle`, `cavern` |
| GH | manor | 9 × 9 × 3 | 15–19 | 2000 | 2500 | 3 | 2500 | 2000 | 5000 | 2000 | 2000 | `manor`, `catacomb` |
| CH | labyrinth | 10 × 10 × 3 | 14–18 | 2500 | 2000 | 3 | 2000 | 1500 | 6000 | 2000 | 1500 | `sunken_town`, `abyss` |
| SO | ship | 8 × 12 × 3 | 16–20 | 1500 | 2500 | 3 | 6000 | 6000 | 1000 | 1000 | 1500 | `starship`, `station` |
| CP | arcology | 8 × 8 × 4 | 15–19 | 2000 | 4000 | 5 | 6000 | 3500 | 3500 | 1500 | 2000 | `megablock`, `undercity` |
| PA | ruins | 12 × 12 × 2 | 13–17 | 1000 | 1500 | 5 | 5000 | 3000 | 1500 | 1500 | 3000 | `wasteland`, `bunker` |
| WW | frontier | 12 × 12 × 2 | 13–17 | 1000 | 1500 | 5 | 6000 | 4000 | 1000 | 1000 | 1500 | `boomtown`, `mine` |
| HN | streets | 9 × 9 × 3 | 15–19 | 2500 | 3000 | 5 | 5000 | 2500 | 5000 | 1500 | 1000 | `city_street`, `tenement` |
| WX | terraces | 10 × 10 × 3 | 14–18 | 1500 | 4000 | 3 | 2000 | 3500 | 2000 | 1500 | 2500 | `mountain_sect`, `bamboo_vale` |
| SP | foundry | 9 × 9 × 3 | 16–20 | 2000 | 3500 | 3 | 5000 | 4500 | 2000 | 1500 | 2500 | `foundry`, `airship` |
| FT | woodland | 11 × 11 × 2 | 14–18 | 2000 | 1500 | 5 | 3000 | 3000 | 2000 | 2500 | 2000 | `deep_wood`, `cottage` |
| MA | temple | 10 × 10 × 3 | 15–19 | 1500 | 2000 | 5 | 3500 | 5000 | 1000 | 2000 | 3000 | `temple`, `labyrinth` |
| ON | dream | 10 × 10 × 4 | 13–17 | 3000 | 4000 | 3 | 3000 | 3000 | 3000 | 2500 | 1500 | `dreamscape`, `nursery` |
| RD | office | 8 × 8 × 4 | 18–22 | 3000 | 2500 | 3 | 4000 | 8000 | 0 | 2000 | 2000 | `bureau`, `archive` |
| UW | blank | 12 × 12 × 2 | 10–12 | 500 | 1000 | 3 | 1000 | 2000 | 0 | 1000 | 1000 | `blank_page`, `margin` |
| UT | palimpsest | 12 × 12 × 2 | 15–19 | 2000 | 2500 | 5 | 4000 | 4000 | 2000 | 2000 | 2500 | `palimpsest`, `colophon` |

`cycle_w` (§8.2 order lock_key, hidden_shortcut, gauntlet_return, two_key_hub, flooded_detour, collapsing_bridge, patrol_loop, foreshadowed_vault): MF 20 10 10 15 10 10 10 15; GH 15 20 10 10 5 10 15 15; CH 10 25 5 10 20 10 10 10; SO 25 10 10 20 5 5 20 5; CP 20 15 5 15 5 5 25 10; PA 10 15 15 5 15 25 10 5; WW 15 10 15 5 5 25 15 10; HN 15 20 5 15 10 5 25 5; WX 10 15 20 10 15 20 5 5; SP 20 10 10 20 10 15 10 5; FT 15 20 10 10 15 10 5 15; MA 15 10 25 10 10 10 5 15; ON 10 25 10 10 15 15 5 10; RD 25 10 5 25 0 0 30 5; UW 30 20 0 0 0 30 20 0; UT 13 13 12 12 12 12 13 13.

`node_w` (H plant, F fungus, M ore; doc 08): MF 40 20 40; GH 20 50 30; CH 10 60 30; SO 10 10 80; CP 10 20 70; PA 20 20 60; WW 30 10 60; HN 20 30 50; WX 50 20 30; SP 10 20 70; FT 60 30 10; MA 40 10 50; ON 40 40 20; RD 0 20 80; UW 0 0 100; UT 34 33 33.

### 3.4 Layout styles (`gpv.layout.*`)

`fp_weights` pick footprints `[1×1, 1×2, 2×2, 2×3, 3×3]` in cells for kinds that allow a range (§16). `two_storey_bp` is the chance a kind that allows it spans 2 storeys. `open_bp` is the chance a non-safe room is tagged open, which sets doc 03 `open_tag` on its cells (sky-lit at `ambient_light`, no roof). `start_storey` is the Incipit's storey, clamped to `storeys - 1`.

| Layout (genre order) | fp_weights | two_storey_bp | open_bp | start_storey |
|---|---|---|---|---|
| `gpv.layout.keep` | 20 25 35 15 5 | 1500 | 1000 | 0 |
| `gpv.layout.manor` | 25 30 30 10 5 | 1500 | 500 | 0 |
| `gpv.layout.labyrinth` | 30 40 20 10 0 | 1000 | 0 | 0 |
| `gpv.layout.ship` | 20 35 30 15 0 | 1000 | 0 | 1 |
| `gpv.layout.arcology` | 20 30 30 15 5 | 2500 | 2500 | 0 |
| `gpv.layout.ruins` | 15 25 35 15 10 | 1000 | 6000 | 0 |
| `gpv.layout.frontier` | 20 25 35 15 5 | 500 | 7000 | 1 |
| `gpv.layout.streets` | 25 30 30 10 5 | 2000 | 5000 | 0 |
| `gpv.layout.terraces` | 20 25 30 15 10 | 2500 | 6000 | 0 |
| `gpv.layout.foundry` | 15 25 35 15 10 | 2500 | 2000 | 0 |
| `gpv.layout.woodland` | 15 25 35 15 10 | 1500 | 8000 | 0 |
| `gpv.layout.temple` | 15 20 35 20 10 | 2000 | 4000 | 0 |
| `gpv.layout.dream` | 20 25 30 15 10 | 3000 | 5000 | 1 |
| `gpv.layout.office` | 40 40 20 0 0 | 0 | 0 | 0 |
| `gpv.layout.blank` | 50 40 10 0 0 | 0 | 10000 | 0 |
| `gpv.layout.palimpsest` | 15 25 35 15 10 | 2000 | 3000 | 0 |

### 3.5 Blocks E and F: population, events, bleed and verbs

Verb weights are Bp with 10000 neutral. `hack` is 0 where TEC ≤ 2; `pray` is 0 where MYT = 0; `cast` is 0 where `aether_axiom` is none and ARC = 0. `bleed_ok` is false for RD, UW and UT, true otherwise.

| # | density_bp | event_bp | drift_bp | bleed_in_bp | seam_bp | shoot | melee | cast | pray | sneak | talk | hack | craft | build | trav |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| MF | 10000 | 2500 | 2500 | 3000 | 1500 | 6000 | 14000 | 15000 | 11000 | 9000 | 10000 | 0 | 9000 | 8000 | 9000 |
| GH | 9000 | 4000 | 2500 | 3000 | 1500 | 7000 | 11000 | 9000 | 15000 | 14000 | 9000 | 3000 | 8000 | 6000 | 8000 |
| CH | 8000 | 4500 | 3500 | 5000 | 1500 | 8000 | 7000 | 11000 | 12000 | 15000 | 9000 | 6000 | 9000 | 6000 | 9000 |
| SO | 11000 | 3000 | 2000 | 3000 | 1500 | 15000 | 9000 | 9000 | 5000 | 7000 | 10000 | 13000 | 9000 | 9000 | 11000 |
| CP | 10500 | 3000 | 2500 | 3000 | 1500 | 13000 | 9000 | 5000 | 3000 | 13000 | 10000 | 16000 | 10000 | 7000 | 11000 |
| PA | 10000 | 3000 | 3000 | 3000 | 1500 | 12000 | 11000 | 6000 | 7000 | 10000 | 8000 | 8000 | 15000 | 15000 | 10000 |
| WW | 10000 | 3000 | 2500 | 3000 | 1500 | 15000 | 10000 | 8000 | 10000 | 8000 | 10000 | 4000 | 9000 | 8000 | 11000 |
| HN | 9000 | 3500 | 2000 | 3000 | 1500 | 12000 | 9000 | 6000 | 3000 | 14000 | 16000 | 7000 | 7000 | 5000 | 9000 |
| WX | 10500 | 2500 | 2500 | 3000 | 1500 | 3000 | 16000 | 12000 | 8000 | 10000 | 9000 | 0 | 8000 | 6000 | 16000 |
| SP | 10500 | 3000 | 2500 | 3000 | 1500 | 13000 | 9000 | 7000 | 6000 | 8000 | 10000 | 9000 | 15000 | 13000 | 11000 |
| FT | 9500 | 3500 | 3000 | 4000 | 1500 | 4000 | 10000 | 16000 | 12000 | 10000 | 15000 | 0 | 10000 | 8000 | 9000 |
| MA | 11000 | 3000 | 2500 | 3000 | 1500 | 5000 | 15000 | 10000 | 16000 | 7000 | 10000 | 0 | 8000 | 7000 | 12000 |
| ON | 9500 | 5000 | 10000 | 6000 | 3000 | 8000 | 9000 | 15000 | 11000 | 9000 | 9000 | 6000 | 8000 | 10000 | 16000 |
| RD | 9000 | 4000 | 0 | 0 | 0 | 9000 | 7000 | 5000 | 8000 | 16000 | 12000 | 15000 | 8000 | 7000 | 8000 |
| UW | 6000 | 2000 | 0 | 0 | 0 | 6000 | 14000 | 0 | 0 | 12000 | 0 | 0 | 10000 | 14000 | 12000 |
| UT | 12000 | 4000 | 0 | 0 | 0 | 10000 | 10000 | 10000 | 10000 | 10000 | 10000 | 10000 | 10000 | 10000 | 10000 |

Liquids (`liquids`, `liq.<name>`, doc 08): MF water 70, oil 20, lava 10. GH water 40, blood 50, alcohol 10. CH water 60, acid 20, blood 20. SO water 30, acid 40, oil 30. CP water 40, acid 30, oil 30. PA water 30, acid 40, oil 30. WW water 50, oil 30, alcohol 20. HN water 60, alcohol 25, oil 15. WX water 80, alcohol 20. SP water 40, oil 50, lava 10. FT water 70, alcohol 20, blood 10. MA water 50, lava 30, oil 20. ON water 50, alcohol 30, acid 20. RD water 20, red_ink 60, acid 20. UW red_ink 100. UT water 40, red_ink 30, lava 30.

Story domains (§20): MF flame, oath, star, wyrm. GH blood, moon, crypt, mirror. CH deep, star, madness, tide. SO star, empire, void, machine. CP chrome, neon, data, corporation. PA rust, ash, water, radiation. WW dust, gold, hex, iron. HN rain, money, smoke, lie. WX jade, sword, river, mountain. SP steam, brass, clock, sky. FT thorn, glass, wolf, bargain. MA bronze, oracle, sea, hubris. ON sleep, mirror, flight, door. RD form, seal, silence, file. UW blank ×4. UT word ×4.

### 3.6 Example GPV record

```json
{
  "id": "genre.gothic_horror", "tier": 0,
  "arc": 5, "tec": 3, "myt": 9, "vrv": 3, "aether_axiom": "ax.mythos", "gravity_bp": 10000, "ambient_light": 2,
  "air_accel_bp": 10000, "fall_dmg_bp": 10000, "ground_friction_bp": 10000, "spawn_block_max": 7,
  "palette": [3809067, 7998996, 13157300],
  "materials": ["mat.silver", "mat.blackwood", "mat.bone"],
  "manufacturers": ["mfr.gothic_horror.vesper_sons", "mfr.gothic_horror.grimhallow"],
  "family": 1, "era_tl": 5, "render_style": 1, "fixture_level": 12, "fog": 1, "ambient_decic": 80,
  "liquid_viscosity_bp": 9000, "gas_dissipation_bp": 8000,
  "damage_affinity": [{"dmg": "dmg.umbral", "bp": 2000}, {"dmg": "dmg.radiant", "bp": -1500}],
  "patron": "patron.gothic_horror",
  "layout": "gpv.layout.manor", "fp_w": 9, "fp_d": 9, "storeys": 3, "rooms_min": 15, "rooms_max": 19,
  "loop_bp": 2000, "vert_bp": 2500, "corridor_w": 3, "cover_bp": 2500, "bright_bp": 2000, "dark_bp": 5000,
  "secret_bp": 2000, "trap_bp": 2000,
  "liquids": [{"liq": "liq.water", "w": 40}, {"liq": "liq.blood", "w": 50}, {"liq": "liq.alcohol", "w": 10}],
  "tilesets": ["biome.gothic_horror.manor", "biome.gothic_horror.catacomb"],
  "cycle_w": [15, 20, 10, 10, 5, 10, 15, 15], "node_w": [20, 50, 30],
  "room_bias": [{"room": "room.crypt", "bp": 20000}, {"room": "room.prison", "bp": 12000}, {"room": "room.garden", "bp": 5000}],
  "density_bp": 9000, "verbs": [7000, 11000, 9000, 15000, 14000, 9000, 3000, 8000, 6000, 8000],
  "hazards": ["hazard.spike_pit", "hazard.curse_glyph", "hazard.blade_trap", "hazard.falling_debris", "hazard.miasma_bog", "hazard.alarm"],
  "events": ["gpv.evt.light_failure", "gpv.evt.whispers", "gpv.evt.ambush", "gpv.evt.weather_front", "gpv.evt.plea", "gpv.evt.lore_echo"],
  "event_bp": 4000, "bleed_ok": 1, "drift_bp": 2500, "bleed_in_bp": 3000, "seam_bp": 1500,
  "secret_exits": [{"genre": "genre.oneiric", "bp": 1500}],
  "laws": ["law.gothic_horror.dread_escalation", "law.gothic_horror.sanctified_ground"],
  "story_domains": ["blood", "moon", "crypt", "mirror"]
}
```

Encodings: `tier` core 0, secret 1; `family` fantasy 0, horror 1, sf 2, historical 3, crime 4, mythic 5, meta 6; `render_style` pbr 0, painterly 1, toon 2, ink 3, pixel_dither 4, xerox_collage 5; `palette` entries are the canon `#RRGGBB` values as decimal integers; booleans are 0 or 1; `hazards` and `events` take their weights from list position (§18.2, §19.2).

### 3.7 Lints (content load fails the build)

- **L1** `fp_w × fp_d × storeys >= 8 × (rooms_max + 10)` (minimum margin: RD `256 >= 256`).
- **L2** Block A equals Canon §5.2 and Block B equals Canon §5.9 field for field.
- **L3** the verb zero rules of §3.5.
- **L4** every listed hazard and event is eligible in its genre (§18.1 `dep` and electronic flag, §19.1 `requires`).
- **L5** `protect_r >= 4` wherever `spawn_block_max <= 14`.
- **L6** `bright_bp + dark_bp <= 10000`.
- **L7** every genre lists 1 to 3 Laws, includes every Law Canon §5.10 requires of it, and at most one Law with `changes_constant = 1`.

## 4. World Laws (`law.*`, Canon §5.10)

A Law is active where the local genre is its genre and in seams that include it. Additive effects enter the Canon §10.5 step-1 bucket (potency 10000) when the attacker stands where the Law is active. No Law changes a canon constant (`changes_constant = 0`).

| ID | Kind | Exact effect |
|---|---|---|
| `law.mythic_fantasy.true_names` | additive | once the party has killed 3 NPCs of one enemy template in the Folio, Errata hits on that template add +1000 Bp |
| `law.mythic_fantasy.oathbound` | additive | an Erratum who accepts a `gpv.evt.plea` and completes it in the same Folio gains `st.blessed` directly |
| `law.gothic_horror.dread_escalation` | clock | dread meter, step 250, cap 2500 (below) |
| `law.gothic_horror.sanctified_ground` | additive | inside `room.shrine` and `secret.room.shrine`, Errata hits on undead- or fiend-tagged NPCs add +3000 Bp |
| `law.cosmic_horror.dread_escalation` | clock | dread meter, step 300, cap 3000 |
| `law.cosmic_horror.non_euclidean` | structural | 2 relink door pairs, period 3600 ticks (§9.4) |
| `law.cosmic_horror.forbidden_knowledge` | additive | reading a lore object costs the reader 5 composure (Canon §5.7 MYTHOS factor applies); the Folio story is solved at 2 distinct events instead of 3 (§20.4) |
| `law.space_opera.shipboard_systems` | structural | doors here are electronic; locked ones are hackable (doc 09) |
| `law.space_opera.hull` | structural | worldgen places at least `floor_div(rooms, 4)` `hazard.hull_breach` panels |
| `law.cyberpunk.netrun` | additive | electronic hazards, locks and doors here are hackable (doc 09); hacking a camera disables its room's cameras for 1800 ticks |
| `law.cyberpunk.corporate_response` | structural | each camera alarm makes doc 13's Director spawn a wave at once (ignoring its 900-tick spacing), at most 2 per Folio |
| `law.post_apocalypse.scavenge` | additive | destroyed crates and barrels here drop 1 material of the genre's Canon §5.9 pool (doc 07) |
| `law.post_apocalypse.scarcity` | structural | resource-node yields × 15000 (doc 07); +2 nodes per 4 rooms |
| `law.weird_west.high_noon` | binding | `law_bound` target of duel techniques (doc 05). Standoff: aiming within 1024 Bam of a hostile ≤ 20,000 mm away while both move < 100 mm for 90 ticks gives the next shot within 60 ticks +5000 Bp and a direct `st.staggered` |
| `law.hardboiled_noir.void_rules` | clock | complication rolls at a fixed 500 Bp, never escalating or resetting |
| `law.hardboiled_noir.leads` | structural | solving the Folio story (§20.4) also discovers the Folio's secret exit, if one exists |
| `law.wuxia.qinggong` | binding | `law_bound` target of qinggong techniques (wall-run, glide; docs 04, 05); no other effect |
| `law.wuxia.honor_duel` | structural | during an accepted `gpv.evt.duel_challenge`, other NPCs in the duel room never target the duelist (doc 13) |
| `law.steampunk.weird_science` | grant | `law_grants: [{component_class: gadget, axis: techne, tol_bonus: 2}]` |
| `law.steampunk.clockwork` | structural | every puzzle room here uses the lever variant (§16.3) |
| `law.fairy_tale.rule_of_three` | structural | after 2 failed attempts at a lever puzzle, the correct order glows on the levers |
| `law.fairy_tale.cold_iron` | additive | Errata hits with a weapon of primary material `mat.cold_iron` on fae-tagged NPCs add +2500 Bp |
| `law.mythic_antiquity.hubris` | structural | after 40 party kills in a Folio without a shrine prayer, the Director spawns one Nemesis (a `rank.champion` with the Folio's whole aspect set) at the anchor farthest from every Erratum; once per Folio |
| `law.mythic_antiquity.favor` | additive | each Erratum's first prayer per Folio at a `room.shrine` gains +50 favor before Canon §16.5 scaling (doc 06) |
| `law.oneiric.gravity_flip` | structural | 2..=4 gravity-flip volumes (§9.5) |
| `law.oneiric.dream_doors` | structural | 1 relink door pair, period 1800 ticks |
| `law.redacted.dread_escalation` | clock | dread meter, step 250, cap 2500 |
| `law.redacted.censorship` | additive | items picked up here show a redacted name until `know.appraised` (doc 10) |
| `law.unwritten.editors_only` | structural | roster is `fac.editors` only; no shop and no Lucre containers (Canon §5.2) |
| `law.ur_text.pure_reading` | structural | records the Canon §5.5 override `C = 0`, `E = 0` |

**Dread meter** (Cypher Horror Mode, R6 §11): Folio state `dread_bp`, 0 at entry, gains `step` on each tick with `(t - entry) % 360 == 0` while an Erratum is `in_combat`, on each tick with `(t - entry) % 7200 == 0`, and on each Erratum Contradiction failure; capped at `cap`; reset when the Colophon services open (doc 14 §3.9). **Complication rolls** on `rng.event` at `(t - entry) % 600 == 0`, `t - entry >= 600`, Colophon closed, an Erratum where the Law is active: `roll_bp(dread_bp)`; on success `weighted(light_failure 30, door_slam 25, whispers 25, ambush 20)`, then `range` over Law-active rooms holding or adjoining an Erratum (1 + 1 + 1 draws; ambush 2 more). Example: Gothic Horror in combat from entry reaches `250 × 4 = 1000` at tick 1080, a 10% roll.

## 5. Genre Lattice

### 5.1 Nodes

16 genre nodes in Canon §5.2 order (the **lattice order** of every sorted list) plus `lattice.node.margin` for `folio_index = 0`. Exits offer core nodes; secret nodes come from secret exits (§17.4) and the forced Ur-Text.

### 5.2 Core edge weights

Symmetric `u32` weights, each core pair at least 10.

| from \ to | MF | GH | CH | SO | CP | PA | WW | HN | WX | SP | FT | MA |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| MF | — | 60 | 20 | 20 | 10 | 20 | 30 | 10 | 60 | 40 | 90 | 90 |
| GH | 60 | — | 90 | 10 | 20 | 30 | 60 | 50 | 20 | 70 | 60 | 30 |
| CH | 20 | 90 | — | 50 | 40 | 50 | 40 | 70 | 10 | 40 | 20 | 40 |
| SO | 20 | 10 | 50 | — | 80 | 40 | 30 | 20 | 30 | 50 | 10 | 30 |
| CP | 10 | 20 | 40 | 80 | — | 70 | 20 | 90 | 40 | 40 | 10 | 10 |
| PA | 20 | 30 | 50 | 40 | 70 | — | 80 | 30 | 10 | 50 | 20 | 20 |
| WW | 30 | 60 | 40 | 30 | 20 | 80 | — | 50 | 30 | 60 | 30 | 30 |
| HN | 10 | 50 | 70 | 20 | 90 | 30 | 50 | — | 20 | 40 | 10 | 10 |
| WX | 60 | 20 | 10 | 30 | 40 | 10 | 30 | 20 | — | 20 | 50 | 60 |
| SP | 40 | 70 | 40 | 50 | 40 | 50 | 60 | 40 | 20 | — | 30 | 20 |
| FT | 90 | 60 | 20 | 10 | 10 | 20 | 30 | 10 | 50 | 30 | — | 50 |
| MA | 90 | 30 | 40 | 30 | 10 | 20 | 30 | 10 | 60 | 20 | 50 | — |

The raw walk visits genres 6.9% (WX) to 9.8% (GH) of the time; freshness (§5.4) flattens that.

### 5.3 Secret and virtual edges

- **Margin → core:** 100 each.
- **Core → secret:** secret-exit chances in Bp (§17.4): Oneiric from FT 2500, MF 2000, CH 2000, WX 2000, GH 1500, MA 1500; Redaction from CP 2500, HN 2500, SO 2000, SP 1500, PA 1500, CH 1500; Unwritten from every core genre 1000; Ur-Text from every core genre 800 (Marginalia only).
- **Secret → core return** (exits out of a secret Folio): Oneiric → FT 90, MF 70, CH 70, WX 60, GH 50, MA 50, others 10; Redaction → CP 90, HN 90, SO 70, SP 50, PA 50, CH 50, others 10; Unwritten and Ur-Text → every core 50.

One record per nonzero directed edge: `{ "id": "lattice.edge.cyberpunk.hardboiled_noir", "from": "genre.cyberpunk", "to": "genre.hardboiled_noir", "w": 90 }`.

### 5.4 Volume and run rules

`m` is the Folio being chosen; `history[i]` is Folio `i`'s genre.

1. **Volumes** per doc 14 §3.1; Marginalia blocks `floor_div(m - 26, 5)` for `m >= 26`.
2. **No repeat in a Volume** or Marginalia block (Canon §17.1).
3. **Run cap** for `m <= 25`: exclude core genres already present 3 times. Secret genres: once per run before Marginalia (§17.4).
4. **Never the current genre.**
5. **Freshness** `fresh_bp`: main run by count in `history`: 0 → 15000, 1 → 10000, 2 → 5000, 3+ → 2500; Marginalia by count in the last 10 entries: 0 → 15000, 1 → 10000, 2+ → 5000.
6. **Effective weight** `w(g) = edge(cur, g) × fresh_bp(g)` (`u32`, at most 1,500,000); for a secret `cur`, `edge` reads the return weights; for the Margin node it is 100.
7. **Forced:** the Folio at depth 25 (Standard 25, Abridged 15) is `genre.ur_text` (doc 14 §3.6 item 2).

**Feasibility.** Before Folio 25, rules 2 and 3 exclude at most 9 genres together (8 capped genres need 24 appearances in at most 23 Folios; with 7 capped, only 2 earlier slots are uncapped), so at least 3 stay eligible; Abridged and Marginalia exclude at most 4. The implementation asserts `eligible.len() >= 3`.

### 5.5 Exit-offer algorithm

The draw happens on the `ColophonOpen → ExitVote` (or `→ VictoryChoice`) tick (doc 14 §3.2), when the exit UI opens (Canon §17.1); exactly 3 candidates every time, even before forced Folios.

```rust
/// Pure. `n` = exiting Folio index (0 at run start).
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
    for k in 0..3 { let i = rng.weighted(&w); out[k] = elig[i]; w[i] = 0; }   // exactly 3 range() calls
    out
}
```

Doc 14 §3.6 makes doors of the first `EXIT_OFFERS`, applies the depth-25 override, appends a discovered secret exit and runs the vote.

**Worked example.** Run seed `0x0123456789ABCDEF`, host Origin Cyberpunk, so Folio 1 is Cyberpunk. At its exit `stream_seed(run, "rng.lattice", 1) = 0xb93f5f6ccba93fac`; all 11 other core genres are eligible with freshness 15000, so weights are `edge(CP, g) × 15000`: MF 150,000; GH 300,000; CH 600,000; SO 1,200,000; PA 1,050,000; WW 300,000; HN 1,350,000; WX 600,000; SP 600,000; FT 150,000; MA 150,000; total 6,450,000.

| Draw | Total | `range(total)` | Cumulative weight before the pick | Pick |
|---|---|---|---|---|
| 1 | 6,450,000 | 1,106,450 | 1,050,000 (MF + GH + CH) | Space Opera |
| 2 | 5,250,000 | 3,525,875 | 2,400,000 | Hardboiled Noir |
| 3 | 3,900,000 | 3,019,215 | 3,000,000 | Steampunk |

With `EXIT_OFFERS = 2` the doors show Space Opera and Hardboiled Noir.

### 5.6 Folio 1

- Standard: the host's Origin genre when core. A secret Origin maps to its highest return-weight core genre, ties to lattice order: Oneiric → Fairy Tale, Redaction → Cyberpunk, Unwritten and Ur-Text → Mythic Fantasy (Canon gaps G4).
- Seeded modes: the first candidate of `lattice_candidates(run_seed, 0, lattice.node.margin, [], mode)`, a uniform draw. For `0x0123456789ABCDEF` the candidates are MF, HN, SP (Folio 1 is Mythic Fantasy); for the Daily seed of 2026-10-11 (`0xc6f2e12ddca712ab`) they are WW, CH, MF.

### 5.7 Exit preview

Each door shows doc 14 §3.6 item 4 (name, palette, axioms, position, Contradiction forecast) plus **pure plan functions** of the candidate Folio, which recompute seeds without advancing live streams and match Folio `n + 1`'s generation:

| Field | Source |
|---|---|
| champion aspects with their counter icons | doc 13 §7.3 aspect set, recomputed from a temporary copy of Folio `n + 1`'s `rng.spawn` (Canon §17.3a) |
| Folio type and challenge kind | `folio_type()` (§6.1) |
| bleed: entry genre, drift genres, seam genre | `bleed_plan()` (§14.1) |
| Marginalia modifiers | `marg_mods()` (§22) |
| the active Laws | §4 for the candidate genre |
| two hazard icons | the two highest-weight hazards of the candidate GPV, ties to list order |

## 6. Folio types

### 6.1 Assignment

`lattice.folio.boss` is doc 14's Boss; `lattice.folio.standard`, `.challenge` and `.rest` are variants of doc 14's Normal and keep its Rubric, Keeper and Colophon (doc 14 §3.1, §3.4).

```rust
fn folio_type(run_seed: u64, n: u32, genre: GenreId, mode: Mode) -> FolioType {
    let pos = doc14::pos(n, mode);
    if pos == 5 { return Boss; }
    if n == 1 || genre == UNWRITTEN { return Standard; }
    let mut r = stage_rng(run_seed, n, genre, "ftype", 0);
    let w: [u32; 3] = if n >= 26 { [7000, 2000, 1000] }      // [standard, challenge, rest]
                      else if pos == 4 { [8500, 1500, 0] }    // the Interleaf is the pre-boss rest
                      else { [8000, 1500, 500] };
    let t = [Standard, Challenge, Rest][r.weighted(&w)];      // 1 weighted call
    if genre == REDACTED && t == Rest { Standard } else { t }
}
```

Example (§5.5 run): at Folio 2 Space Opera draws 3860 (standard) and Noir 9855 (rest); at Folio 3 Cyberpunk is rest and Mythic Fantasy challenge; at Folio 4 Cyberpunk is challenge.

### 6.2 Composition

The Rubric is one `weighted([5000, 3000, 2000])` call (keeper, catchwords, seal) in stage `rubric` (doc 14 §3.4); Folio 2 of §5.5 draws catchwords for Space Opera, keeper for Noir and Steampunk.

| Type | Mission graph | Fixed rooms | Population (doc 13) |
|---|---|---|---|
| standard | `max(4, mul_bp(range_incl(rooms_min, rooms_max), folio_scale_bp))` nodes | Incipit, Colophon; Interleaf at pos 4 | full Director budget |
| challenge | `range_incl(10, 14)` | Incipit, Colophon, challenge structure (§6.3); Interleaf at pos 4 | full budget |
| rest | `range_incl(6, 8)` | Incipit, campfire hub, shop, Fortune Engine hall, Colophon | flag `rest_population`: only the Rubric's Keeper or Seal waves; every other room `no_spawn` |
| boss | fixed spine (§8.2) | Incipit antechamber, Editor checkpoint, boss arena, Colophon | doc 13 §10.1 Editor-Folio squad in the checkpoint |

### 6.3 Challenge kinds

Drawn in stage `ftype` right after a challenge result (1 more weighted call). Success is judged at exit commit; each success gives every Active Erratum one extra in-run Reliquary roll (doc 11).

| ID | w | Structure | Success |
|---|---|---|---|
| `lattice.challenge.gauntlet` | 25 | 3 `room.gauntlet` on the main path | no Erratum downed in the Folio |
| `lattice.challenge.siege` | 20 | an Arena on the main path seals for 7200 ticks; doc 13 sends a wave every 900 ticks inside it | survive |
| `lattice.challenge.heist` | 20 | a locked `room.treasury` | open it with no hostile ever at `aware.alert` toward an Erratum |
| `lattice.challenge.duel` | 15 | a champion with the Folio's full aspect set waits in the room before the Colophon | kill it |
| `lattice.challenge.race` | 20 | none | commit an exit within 10800 ticks of entry |

## 7. Generation pipeline

### 7.1 Inputs and outputs

`generate_folio(input, db, par) -> FolioBlueprint` (doc 02) is pure; doc 14 §3.3 calls it at each Folio entry.

```rust
pub struct GenInput {
    pub run_seed: u64,
    pub folio_index: u32, pub d: i32, pub pos: u8, pub mode: Id32,   // doc 14 §3.1, §8
    pub genre: Id32, pub prev_genre: Option<Id32>,
    pub folio_scale_bp: i32, pub ambush_extra: u8,  // doc 14 §8.1, gp.ambush_extra
    pub bleed_mult: u8, pub bleed_add: u8,          // run.mut.wild_bleed ×2, run.marg.bleeding_genres +r
}
pub struct FolioBlueprint {
    pub header: Header,               // sim_version, content_hash, every GenInput field, attempt, blueprint_hash
    pub plan: FolioPlan,              // ftype, challenge, rubric, bleed plan, marg_mods
    pub zones: SmallVec<[ZoneRec; 6]>, pub zone_of_cell: [u8; 1536],   // 16 × 6 × 16 cells
    pub graph: MissionGraph, pub rooms: Vec<RoomRec>, pub doors: Vec<DoorRec>,
    pub chunks: Vec<ChunkBlob>,       // 128 chunks: palette_rle blocks, open_tag, protect, gsign (doc 03 §16)
    pub fixtures: Vec<FixtureRec>, pub liquids: Vec<LiquidRec>, pub hazards: Vec<HazardRec>,
    pub nodes: Vec<NodeRec>,          // resource nodes and font basins (§11.2)
    pub spawn: SpawnPlan,             // §12
    pub loot: LootSpots,              // §13
    pub rubric: RubricSetup, pub colophon: ColophonRec, pub interleaf: Option<RoomIdx>,
    pub lore: Vec<LoreRec>, pub secrets: Vec<SecretRec>, pub story: StoryFragment,
    pub flags: Flags,                 // illumination_vault, rescue, scriptorium_room, rest_population; rooms_total
}
```

`blueprint_hash` is BLAKE3 of the canonical serialization of every other field; co-op peers generate locally and compare it (doc 16). Records instantiate at `ev.folio_entered` in the order locked doors, hazards, containers, nodes, lore, secret objects (ascending index each), then doc 13 spawns; EntityIds follow. `rooms_total` counts non-secret rooms.

### 7.2 Seeds and sub-streams

All worldgen randomness derives from `rng.worldgen` through independent sub-streams, so stages, rooms and chunks may run in any order or in parallel with identical bytes.

```rust
fn derive(parent: u64, salt: u64) -> u64 { mix64(parent ^ mix64(salt)) }
fn gen_seed(run: u64, n: u32, genre: Id32) -> u64 { derive(stream_seed(run, "rng.worldgen", n as u64), genre.0 as u64) }
fn stage_rng(run: u64, n: u32, genre: Id32, stage: &str, attempt: u8) -> Rng {
    Rng::from_seed64(derive(gen_seed(run, n, genre), fnv1a64(stage) ^ attempt as u64))
}
fn item_rng(run: u64, n: u32, genre: Id32, stage: &str, i: u32, attempt: u8) -> Rng {
    Rng::from_seed64(derive(derive(gen_seed(run, n, genre), fnv1a64(stage) ^ attempt as u64), i as u64 + 1))
}
```

Example: `gen_seed(0x0123456789ABCDEF, 2, genre.space_opera) = 0x1c35d41d581f5eaa`. Plan stages (`ftype`, `rubric`, `bleed`, `marg`) always use `attempt = 0`, so previews match; structural stages use the current attempt (§15.3).

### 7.3 Stages, draws and time budget

| # | Stage | Work | Draws | p50 / p99 ms (Steam Deck, 4 threads) |
|---|---|---|---|---|
| 0 | `ftype`, `rubric`, `bleed`, `marg` | §6, §14.1, §22 | 1–2; 1; 1–6; k | < 0.1 |
| 1 | `graph` | mission graph (§8) | per rule | 1 / 3 |
| 2 | `typing` | kinds, footprints (§8.4) | 1–3 per node | 0.5 / 1 |
| 3 | `zones` | bleed and seam rooms (§14.2) | 1 + cluster | < 0.5 |
| 4 | `embed` | cells, doors, corridors, relinks, flips (§9) | 1–2 per node | 4 / 12 |
| 5 | `wfc` per room | room fill (§10) | 1 per collapse | 45 / 110 |
| 6 | `liquid`, `node`, `hazard` | §11.2–§11.4 | per placement | 3 / 6 |
| 7 | `dress`, `light` per room | §11.1, §11.3 | 1 shuffle each | 15 / 35 |
| 8 | `spawn`, `loot`, `secret`, `story` | §12, §13, §17, §20 | per record | 2 / 4 |
| 9 | rasterize (`par.map`, 128 chunks) | blocks, bits, blending (§14.4) | 0 | 20 / 45 |
| 10 | validate and repair | §15 | 0 | 15 / 40 |
| — | one attempt | | | **≤ 110 / 260** |
| — | worst case, 4 attempts and fallback | | | ≤ 900 hard cap; CI fails above |

The Folio transition (doc 17) waits for generation.

## 8. Mission graph grammar

### 8.1 Node and edge kinds

Nodes (`NodeId` = creation index): `Incipit`, `Colophon`, `Interleaf`, `Room`, `Hub`, `Key(k)`, `Service(kind)`, `Arena`, `Gauntlet`, `Treasury`, `Puzzle(p)`, `Vault`, `Campfire`, `Checkpoint`, `BossArena`, `Secret`, `SecretExit(genre)`. Edges: `Open`; `Locked(keys)` (1 or 2 keys); `OneWay(from, to)` (a door barred on the `to` side that the `from` side unbars, or a drop of at least 4 blocks); `Hidden` (§17.3); `PuzzleGate(p)`; `RubricGate` (the single edge into the Colophon, sealed by `protect` bits until the Rubric completes, doc 14 §3.4). The **main path** is the shortest Incipit→Colophon path by edge count over every non-Hidden edge, breadth-first, neighbors in ascending `NodeId`.

### 8.2 Rules

`in` = Incipit, `out` = Colophon; `RubricGate` stays the last edge into `out`.

| ID | Phase | Rewrite | Selection |
|---|---|---|---|
| `room.rule.axiom` | A | `in –Open– Room –RubricGate– out` | none |
| `room.rule.cycle_lock_key` | B | middle Room → `in–a`, `a–K1`, `K1→in` OneWay, `in–m` Locked(k1), `m–out` | `cycle_w` |
| `room.rule.cycle_hidden_shortcut` | B | `in–a–b–c–out`; `in–c` Hidden | `cycle_w` |
| `room.rule.cycle_gauntlet_return` | B | `in–Gauntlet–Treasury`, `Treasury→in` OneWay; `in–a–out` | `cycle_w` |
| `room.rule.cycle_two_key_hub` | B | `in–Hub`; `Hub–a–K1`, `K1→Hub` OneWay; `Hub–b–K2`, `K2→Hub` OneWay; `Hub–c` Locked(k1, k2); `c–out` | `cycle_w` |
| `room.rule.cycle_flooded_detour` | B | `in–w–out` with `w` flooded (§11.2); `in–a–b–out` | `cycle_w` |
| `room.rule.cycle_collapsing_bridge` | B | `in–c`, `c→x` OneWay (the bridge collapses after the first crossing), `x–out`; `in–a–b–x` | `cycle_w` |
| `room.rule.cycle_patrol_loop` | B | `in–a–b–x`; `in–c–d–x`; `x–out`; a patrol route on the loop (§12) | `cycle_w` |
| `room.rule.cycle_foreshadowed_vault` | B | `in–w–out`; `w–Treasury` Locked(k1) with a window tile on the shared face; `in–a–K1`, `K1→w` OneWay | `cycle_w` |
| `room.rule.extend` | C, w 50 | split an Open edge `u–v` into `u–n–v` | `range` |
| `room.rule.branch` | C, w 30 | dead end `u–n` on a Room or Hub with degree < 4 | `range` |
| `room.rule.minor_cycle` | C, w 20 while loops remain | `u–n–v` for Rooms `u < v`, degree < 4, hop distance 2..=4, not adjacent | `range` |
| `room.rule.lock_key` | D1 | lock an Open main-path edge off `in`; key on a reachable Room dead end, else a new branch | 2 `range` |
| `room.rule.interleaf` | D2 | pos 4: split the main-path edge at index `floor_div(len × 5000, 10000)` with an `Interleaf` | none |
| `room.rule.service` | D3 | attach each type-required service (§6.2) at main-path index `floor_div(len × frac, 10000)` (shop 5000, Fortune hall 7000, campfire 3000), clamped `1..=len-2` | none |
| `room.rule.arena` | D4 | the main-path Room nearest position 6000 Bp becomes an `Arena` (mini-boss room) | `roll_bp(3000)`; siege always |
| `room.rule.puzzle_gate` | D5 | an inner Open main-path edge becomes `PuzzleGate(p)`; `Puzzle(p)` branches from its `in` side | `roll_bp(2500)`, `range` |
| `room.rule.vault` | D6 | attach a `Vault` (Reliquary room) to a non-main-path Room | `roll_bp(5000)`, `range` |
| `room.rule.gauntlet_branch` | D7 | attach `Gauntlet–Treasury` to a Room | `roll_bp(2000)`, `range` |
| `room.rule.rescue` | D8 | the main-path Room nearest 7000 Bp hosts doc 13's stranded native (flag `rescue`) | `roll_bp(2000)` |
| `room.rule.challenge` | D9 | the challenge kind's structure (§6.3) | none |
| `room.rule.secret` | D10 | `Secret` by a Hidden edge to a node other than `in` and `out` | `range` per secret |
| `room.rule.secret_exit` | D11 | `SecretExit(g)` by a Hidden edge to the node farthest in hops from `in` (not `out`; ties to lower `NodeId`) | none |
| `room.rule.boss_spine` | boss | `in–Checkpoint–BossArena–out` | none |
| `room.rule.rest_ring` | rest | `in–Campfire`; Campfire spokes to the services, `j = range_incl(1, 3)` Rooms, and `out` | 1 `range` |

### 8.3 Algorithm

```rust
fn build_graph(r: &mut Rng, g: &Gpv, plan: &FolioPlan, n_target: u32) -> MissionGraph {
    match plan.ftype { Boss => return boss_spine(), Rest => return rest_ring(r, g), _ => {} }
    let mut G = axiom();
    apply_cycle(&mut G, CYCLES[r.weighted(&g.cycle_w)]);           // 1 weighted call
    let loops_max = floor_div(n_target * g.loop_bp, 10000);
    let n_grow = n_target - type_required_nodes(plan);
    let mut loops = 0;
    while G.non_secret_count() < n_grow {
        let mut rule = r.weighted(&[50, 30, if loops < loops_max { 20 } else { 0 }]);  // 1 call
        if !eligible(&G, rule) { rule = EXTEND; }                  // extend is always eligible
        G.apply(rule, r.range(G.targets(rule).len() as u64));       // 1 call
        if rule == MINOR_CYCLE { loops += 1; }
    }
    features_d1_to_d11(&mut G, r, plan);                            // fixed order, draws per §8.2
    G
}
```

`n_target` is the first draw of stage `graph` (§6.2 counts). Lock edges in total (major cycle included): standard `1 + [n_target >= 18]`, challenge 1, rest and boss 0. A finished standard graph has `n_target..=n_target + 8` non-secret nodes. Example: Mythic Fantasy, `n_target = 18`, `folio_scale_bp = 10000`: `loops_max = floor_div(18 × 1500, 10000) = 2`; growth stops at 18 non-secret nodes, and `lock_key` adds a second lock since `18 >= 18`.

### 8.4 Typing

Nodes in ascending `NodeId`: feature nodes map directly (`Incipit` → `room.incipit`, `Key` → `room.key_room`, the flooded room → `room.cistern`, the bridge → `room.chasm`, …); each `Room` draws a kind with §16.2 weights, skipping kinds at their maximum or outside the run's content set (1 weighted call). Every node then draws a footprint from `fp_weights` within the kind's range (1 weighted call) and, when 2 storeys are allowed and available, `roll_bp(two_storey_bp)` (1 call). Every non-safe room then rolls `roll_bp(open_bp)` for the open tag (1 call).

## 9. Spatial embedding and corridors

### 9.1 Placement

1. **Incipit:** storey `min(start_storey, storeys - 1)`, centered on the south edge: `cx = x0/16 + floor_div(W - fw, 2)`, `cz = z0/16 + D - fd`.
2. **Order:** breadth-first from the Incipit, neighbors in ascending `NodeId`; a node's parent is its first placed neighbor.
3. **Candidates** for `v` with parent `u`, sorted by `(cs, cz, cx, orientation)`: `H` = free boxes (either orientation) on `u`'s storeys sharing a full cell face with `u`; `V` = free boxes directly above or below `u` overlapping its projection; `C` = free boxes on `u`'s base storey at Chebyshev distance 2..=4 with an A* corridor route (§9.3).
4. **Choice:** the Colophon takes the `H ∪ V` (else `C`) candidate maximizing doubled-center `|Δcx| + |Δcz| + 2|Δcs|` from the Incipit, ties to the earlier index, no draw. Others: with `H` and `V` both non-empty, `V` if `roll_bp(vert_bp)` else `H`, then `range(len)`; with one non-empty, `range(len)` in it; else `range(len)` in `C`; `C` empty fails the attempt (§15.3).
5. **Non-tree edges:** a door when boxes share a face, else a corridor; no route fails the attempt.

### 9.2 Doors, connectors and protect bits

| Edge | Realization |
|---|---|
| Open | horizontal: doorway 3 wide × 3 tall, genre door skin; vertical: a 3 × 3 floor opening with a slab stair run (500 mm rises) along the lower room's wall |
| Locked(keys) | lock door (doc 09 `lock.*`) bound to its key items; destructible like every door (Canon §4.3) with doc 07 block HP by lock tier |
| OneWay | horizontal: a door barred on the `to` side; vertical: a 4-block drop. Terrain is fixed, so only a placeable ladder (Canon §4.3) climbs back |
| Hidden | a §17.3 hiding method |
| PuzzleGate(p) | a gate that opens when puzzle `p` resolves (§16.3) |
| RubricGate | the Colophon entrance; its cells carry `protect` until the Rubric completes (doc 14 §3.4) |

`protect` bits (doc 03 §4.7) are also set on exit-door volumes, on the outermost block layer of the footprint (the Folio-boundary shell) and on boss-arena doors while sealed (doc 13 §9.1). Every worldgen block is indestructible unless its block record carries `destructible` (Canon §4.3); worldgen marks doors, crates, barrels, glass, wooden structures, webs, ice, fixtures and cracked walls hiding secrets as destructible.

### 9.3 Corridors and stairs

- Route: A* over free footprint cells between the boxes' facing sides; costs 10 per horizontal step, 30 per storey change (a stair cell), +2 per turn; ties toward the lower cell index; corridors never enter room cells.
- Carving: width `corridor_w`, clear height 4 blocks (4000 mm, above the 2600 mm mount headroom of Canon §17.7). A stair cell climbs 8 blocks over 16 with alternating slab and full steps, 500 mm per step, under the 550 mm step-up (Canon §4.1).
- Corridors take the zone of their lower-`NodeId` endpoint (§14.2).

### 9.4 Relink pairs

Cosmic Horror (2 pairs, period 3600 ticks) and Oneiric (1 pair, period 1800): horizontal Open doors off the main path and outside safe rooms, chosen by `range` without replacement (1 call per door). State 0 keeps destinations; state 1 swaps each pair's destinations through portal planes (doc 03 rotates yaw by the doors' difference). State toggles at `entry + k × period`; validation covers both.

### 9.5 Gravity-flip volumes (Oneiric)

`range_incl(2, 4)` single-storey Rooms (not the first or last two main-path nodes, not safe, not zoned) are chosen by `range` without replacement; they get clear height 4 blocks, a vertically mirrored fill and `gsign = +1` (doc 03). The flip fall stays under Oneiric `SAFE_FALL = 4000 × (10000 + 4 × 400) / 10000 = 4640` mm, so a flip never hurts.

## 10. Room templates and Wave Function Collapse

### 10.1 Room templates

A template `room.tpl.*` gives a kind its shell, pins and sockets in tile coordinates; a room's tile grid is `4 fw × 2 storeys × 4 fd` (at most 12 × 4 × 12 = 576 tiles).

```json
{
  "id": "room.tpl.library_a", "kind": "room.library", "footprints": [[2, 2]], "storeys": [1, 2],
  "pins": [{"tile": [0, 0, 0], "class": "shell_corner"}, {"tile": [3, 0, 0], "class": "shell_door"}],
  "sockets": [
    {"kind": "light", "tile": [1, 0, 1], "block": [2, 3, 0]},
    {"kind": "container", "tile": [4, 0, 4], "block": [1, 0, 1]},
    {"kind": "lore", "tile": [2, 0, 5], "block": [2, 0, 2]},
    {"kind": "spawn", "tile": [5, 0, 2], "block": [2, 0, 2]},
    {"kind": "node", "tile": [6, 0, 6], "block": [1, 0, 1]}
  ],
  "fallback_fill": "room.tpl.library_a.fill0", "tags": ["room.no_mount"]
}
```

Shell classes pin every boundary tile (wall, corner, door, window, floor-only for open rooms). `fallback_fill` is an authored valid tile assignment.

### 10.2 Tilesets

Each genre has a primary and a secondary tileset (§3.3; the secondary fills its `kinds`), of 40 to 160 prototypes, each a 64-block payload of palette slots as run-length pairs `[slot, count, …]` (index `lx + 4 × (lz + 4 × ly)`), 6 face sockets (`+X, -X, +Y, -Y, +Z, -Z`) and flags (`walkable`, `cover`, `spawn`, `destructible`, `light`).

```json
{
  "id": "biome.gothic_horror.manor",
  "palette": {"1": "block.gothic_horror.wall", "2": "block.gothic_horror.floor", "3": "block.gothic_horror.trim", "4": "block.gothic_horror.shelf", "6": "block.light_fixture"},
  "tiles": [
    {"name": "floor_rug", "w": 120, "sym": "x", "faces": [3, 3, 0, 1, 3, 3], "flags": ["walkable", "spawn"], "rle": [2, 16, 0, 48]},
    {"name": "bookcase", "w": 60, "sym": "l", "faces": [5, 3, 0, 1, 3, 3], "flags": ["cover", "destructible"], "rle": [2, 16, 4, 4, 0, 12, 4, 4, 0, 12, 4, 4, 0, 12]}
  ]
}
```

Slot 0 is air. Symmetry `x` (4 identical rotations), `l` (4 distinct), `i` (2); Oneiric flip rooms add a vertical mirror. Two tiles touch when facing sockets are equal or a declared asymmetric pair. Expanded tilesets hold at most 256 variants, so a domain is a `[u64; 4]` bitset.

### 10.3 WFC algorithm

```rust
fn wfc_room(r: &mut Rng, ts: &Tileset, tpl: &RoomTemplate, dims: (u8, u8, u8), cover_bp: i32) -> Result<TileGrid, WfcFail> {
    let mut dom = vec![ts.all_variants(); n_tiles(dims)];
    apply_pins(&mut dom, tpl);
    propagate_all(&mut dom, ts)?;                         // AC-3, FIFO seeded in ascending tile index
    let mut resets = 0;
    loop {
        let Some(t) = min_options_tile(&dom) else { return Ok(collapse_to_grid(&dom)); };  // ties: lowest index
        let opts: Vec<u16> = dom[t].iter().collect();      // ascending variant index
        let w: Vec<u32> = opts.iter().map(|&v| weight(ts, v, cover_bp)).collect();
        dom[t] = Bitset::single(opts[r.weighted(&w)]);    // 1 draw per observation
        if propagate_from(&mut dom, ts, t).is_err() {
            resets += 1;
            if resets > 3 { return Err(WfcFail); }
            reset_block(&mut dom, t, 4, tpl);             // Merrell block modification
            propagate_all(&mut dom, ts)?;
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
- Rooms run on `item_rng("wfc", room_index, attempt)`, independently, through `par`.

## 11. Dressing, nodes, light, liquids and hazards

### 11.1 Dressing

Per room (`item_rng("dress", i, attempt)`): one shuffle of prop sockets, then props from the tileset list in the first `floor_div(len × 6000, 10000)` (1 weighted call each). Crates, barrels, glass and wood are `destructible`; the Colophon always has some, so desecration (doc 14 §3.9) is possible.

### 11.2 Liquids, resource nodes and basins

- **Basins:** non-safe basin sockets fill with chance 2500 Bp (cisterns and flooded rooms always), liquid by 1 weighted call over the zone genre's `liquids`, settled.
- **Flooded detour:** at least 2 blocks deep along its path with a 2-block air pocket at most every 16 blocks: under 7 s at the 2400 mm/s swim, inside the 14 s breath at END 40 (Canon §12).
- **Exclusions:** no lava, acid or red ink in safe rooms or within 8 blocks of the Incipit spawn; no liquid above the Incipit floor within 8 horizontal blocks.
- **Red-ink pools:** `range_incl(2, 4)` 3 × 3 × 1 pools in Redaction Folios; 2 per Editor arena (§21.2).
- **Font basins:** each `room.shrine`, `room.garden` and `room.cistern` holds a `liq.wellspring` basin with chance 3000 Bp and `range_incl(1, 3)` uses (doc 08).
- **Resource nodes** (Canon §4.3): `6 + floor_div(rooms, 3)` (+2 per 4 rooms under `law.post_apocalypse.scarcity`); each draws a room with free node sockets (`range`), a kind (`weighted(node_w)`: H, F, M) and a material (`range` over the zone genre's Canon §5.9 materials); doc 07 sets yields.

### 11.3 Light plan for stealth gradients

Bands use effective light (Canon §2.3): **dark** `<= 4`, **dim** `5..=9`, **bright** `>= 10`. Targets are `bright_bp` and `dark_bp` (§3.3); `room.stealth` kinds use `bright_bp / 2` and `min(10000, dark_bp × 3 / 2)`; open rooms with `ambient_light >= 5` have no dark target.

```rust
fn light_room(r: &mut Rng, room: &Room, g: &Gpv) -> Vec<Fixture> {
    let feet = room.feet_cells();                       // ascending block index
    let mut sockets = room.light_sockets();             // template + tile sockets, ascending
    r.shuffle(&mut sockets);                            // 1 shuffle (item_rng "light")
    let mut placed = vec![];
    for s in sockets {
        if share(&feet, &placed, BRIGHT) >= room.bright_bp { break; }
        let trial = with(&placed, s);
        if gain(&feet, &placed, &trial, BRIGHT) == 0 { continue; }
        if room.dark_bp > 0 && share(&feet, &trial, DARK) < room.dark_bp { continue; }
        placed = trial;
    }
    if room.is_safe() && g.spawn_block_max <= 14 { spawn_proof(&mut placed, &feet, g); }
    if room.is_stealth() { shadow_route(&mut placed, room); }
    placed
}
```

1. Light uses doc 03's propagation, sky and block channels, inside the room box; `share = floor_div(count_in_band × 10000, feet.len())`.
2. Example: a Mythic Fantasy hall with 840 feet cells needs 294 bright cells (`bright_bp` 3500), and no fixture may leave fewer than 210 dark ones (`dark_bp` 2500).
3. **Spawn-proofing** (no draws): while a feet cell has `light_block <= spawn_block_max`, add the unused socket covering the most such cells (ties to the lower block index); with none left, add a ceiling fixture above the lowest-index uncovered cell.
4. **Shadow route:** a feet-cell path with effective light `<= 6` must join the room's two lowest-index doors; otherwise remove fixtures in descending order of path cells they light (ties to the higher block index) until it exists or one fixture remains.

### 11.4 Hazards and traps

Stage `hazard`, rooms ascending. Count: gauntlet `range_incl(5, 7)`; safe rooms 0; others `floor_div(trap_bp, 10000)` plus 1 on `roll_bp(trap_bp % 10000)`. Each hazard: 1 weighted call over the zone genre's eligible `hazards` (§18), then a hazard socket or, for path traps, a `range`-chosen feet cell on the door-to-door path; 4 blocks of mutual spacing; none within 8 blocks of the Incipit spawn. Damage enters Canon §10.6 at step 3: `D2 = base × P(d)` against NPCs, `base × T(d)` against Errata.

## 12. Population interface (doc 13 §10)

Doc 13 owns budgets, packs and waves; worldgen supplies places.

- **`spawn_slots`:** `floor_div(pop_w × fw × fd × storeys × density_bp, 10000)` (§16.1 `pop`); 0 for safe, sanctuary and `no_spawn` rooms and for rooms with a feet cell within 24,000 mm path of the Incipit spawn (doc 13 §10.2, doc 14 §3.3). A 2 × 2 single-storey hall gets 17 in Space Opera, 14 in Gothic Horror.
- **Spawn cells:** feet cells flagged `spawn`, ascending index, at least `2 × spawn_slots` per room.
- **Spawn anchors** (waves, hunters, invaders): each slotted room's spawn cell nearest its center (ties to the lower index), and the center feet cell of each corridor cell ≥ 24,000 mm from the Incipit.
- **Patrol routes:** one per mission-graph cycle, the center feet cells of its rooms in cycle order.
- **Territory:** rooms with `pop >= 2` take `factions[k % F]`, `k` their order among such rooms, `factions` the zone genre's native factions by ascending ID (doc 13 §8).
- **Civic anchors:** center feet cells of shop, prison, library, garden and hireling-hall rooms (doc 13 civic packs). `no_spawn` bars packs, waves and hunters; only civic packs and doc 14's Colophon hunter may stand in such rooms.
- **Rubric setup** (doc 14 §3.4): Keeper room = the non-safe room farthest by path from the Incipit (ties to lower `NodeId`); catchwords = container or lore sockets ≥ 30,000 mm from the Incipit, taken in descending distance, each in a new room ≥ 20,000 mm from those picked, until 3 (fewer fails the attempt); Seal point = center feet cell of the largest non-safe main-path room beyond 5000 Bp of the Incipit→Colophon distance, with ≥ 80 feet cells within 4000 mm.
- **Rescue:** the rescue room (§8.2) hosts doc 13's stranded native (doc 14 `run.note.rescue`).

## 13. Loot spots (doc 11 §9–§10)

- **Chests:** `4 + range(3)` per Normal Folio, the first draw of stage `loot` (doc 11 §9.6), none in boss Folios; sockets fill treasury (locked chests), armory, key rooms, vault, then 1 weighted call over `lw` per chest.
- **Floor spots:** 36 feet cells (9 per slot 0..=3) taken in order from one shuffle of non-safe feet cells with 4-block spacing; doc 11 fills them.
- **Guarded spots:** container sockets of slotted rooms by `spawn_slots` descending, then `NodeId`, for doc 11's Volume plan.
- **Incipit caches:** 3 container sockets in the main-path room next to the Incipit (counter caches, doc 11 §9.10).
- **Reliquary spots:** vault, then treasury, then other non-safe rooms by descending `lw`; at least 4.
- **Secrets** map to doc 11 tables (§17.2). The Unwritten places no Lucre.

## 14. Bleed, seam and dynamic zones

### 14.1 Bleed plan (pure, previewed)

```rust
fn bleed_plan(run: u64, n: u32, g: GenreId, prev: Option<GenreId>, d: i32, inp: &GenInput, marg: &[MargId]) -> BleedPlan {
    if !gpv(g).bleed_ok { return BleedPlan::NONE; }
    let entry = prev.filter(|p| is_core(*p) && *p != g);
    let mut r = stage_rng(run, n, g, "bleed", 0);
    let extra = marg.contains(&MARG_BLEEDING) as u32;
    let chance = if g == ONEIRIC || extra > 0 || inp.bleed_add > 0 { 10000 }
                 else { min(8000, gpv(g).drift_bp + 500 * (vol_of_depth(d) - 1)) };
    let base = if r.roll_bp(chance) { if g == ONEIRIC { 2 } else { 1 } } else { 0 };   // 1 call
    let count = min(4, base * inp.bleed_mult as u32 + inp.bleed_add as u32 + extra);
    let cand: Vec<GenreId> = CORE.filter(|c| *c != g && Some(*c) != prev);          // lattice order
    let mut w: Vec<u32> = cand.iter().map(|c| edge(g, *c)).collect();
    let drift: Vec<GenreId> = (0..count).map(|_| { let i = r.weighted(&w); w[i] = 0; cand[i] }).collect();
    let src = drift.first().copied().or(entry);
    let seam = src.filter(|_| gpv(g).seam_bp > 0).filter(|_| r.roll_bp(gpv(g).seam_bp));  // 1 call when src exists
    BleedPlan { entry, drift, seam }
}
```

`vol_of_depth(d) = ceil_div(d, 5)` for `d <= 25`, else 6. Examples (run `0x0123456789ABCDEF`, prev MF): Folio 7 GH → entry MF, no drift, no seam; ON → entry MF, drift `[MA, GH]`; CH → entry MF, drift `[FT]`; RD → none; GH with `bleed_add = 1` → drift `[CH, HN]`. Folio 2 GH → entry MF, drift `[HN]`, seam HN; Folio 3 GH → entry MF, no drift, seam MF.

### 14.2 Zone assignment (stage `zones`)

1. **Entry zone** (`zone.bleed`): the Incipit, plus its lowest-`NodeId` neighbor Room on `roll_bp(bleed_in_bp)`.
2. **Drift zone** per drift genre: seed by `range` over eligible Rooms (not sanctuary, safe, arena, checkpoint, gravity-flip or zoned), grown breadth-first by `NodeId` to `range_incl(1, 3)` rooms.
3. **Seam** (`zone.seam`, Canon §5.8): `range_incl(1, 2)` eligible Rooms adjacent to the seam genre's zone, the first by `range`, the second its lowest-`NodeId` eligible neighbor. Inside, `local_axiom` is the per-axis maximum of both genres, both genres' Laws apply, natives of either are native, the Folio genre supplies skins, and `st.unmoored` clears.
4. Corridors take their lower-`NodeId` endpoint's zone; `zone_of_cell` stores the zone index.
5. A bleed zone uses its genre for `local_genre` (Contradiction re-evaluates on crossing), sky light, gravity, §2.1 physics, tileset, fixtures, hazards, liquids, nodes, Laws and roster; doc 13 opens a war front at its edge; a Scriptorium inside it Rewrites into the bleed genre.

### 14.3 Dynamic zones

`set_dynamic_zone(rooms, genre, until_tick)` overrides `zone_of_cell` for the listed rooms with a `zone.dynamic` record and restores it at `until_tick` (inclusive end); every override and restore re-evaluates Contradiction for entities inside. Users: doc 13's `genre_shift` boss phase (1200 ticks), `gpv.evt.genre_storm` (§19), and `room.bleed_shrine`, whose altar turns one bleed zone plain until Folio exit.

### 14.4 Blending

Wall and floor blocks within 2 blocks of a face between zones dither: with `h = mix64(gen_seed ^ (block_linear_index + 1))`, a block takes the neighbor zone's palette when `h % 10000 < [5000, 2500, 0][dist]` (`dist` 0 = touching). Zone membership stays per cell; doc 18 draws a stitched-page seam.

## 15. Validation and repair

### 15.1 Navigation model

Feet cells as §1. Moves: horizontal at level; up 1 block with 3 air above the target (Canon §4.2); down at most `floor_div(SAFE_FALL(v), 1000)` blocks, `v` the target's local VERVE (3 at VERVE 0, 4 at 10); ladders, vines, ropes; swimming in water, blood and alcohol. Lava, acid and red ink block. Locks pass with keys, puzzle gates once the puzzle room is reached, the RubricGate once the Rubric is reachable, bars per §9.2. Collapsing floors count as OneWay drops.

### 15.2 Checks

| ID | Check |
|---|---|
| V1 | every non-secret room is reachable from the Incipit under the key progression |
| V2 | each key is collected strictly before any door needing it |
| V3 | the Rubric targets are reachable without the RubricGate; the Colophon after it |
| V4 | no softlock: with each key set `K_i`, every reachable feet cell reaches the Colophon or a key outside `K_i` |
| V5 | V1–V4 hold in both relink states and after the bridge collapses |
| V6 | no hazard within 8 blocks of the Incipit spawn, no liquid above its floor within 8, no spawn cell or anchor within 24,000 mm path |
| V7 | safe rooms are `no_spawn` and, where `spawn_block_max <= 14`, lit above it |
| V8 | no electronic hazard, lock or camera where local TEC ≤ 2 (Canon §5.7) |
| V9 | no hazard, service or node whose `dep` axis has local axiom 0 (Canon §5.3) |
| V10 | chests as drawn; ≥ 36 floor spots, ≥ 4 Reliquary spots, 3 Incipit caches; spawn cells ≥ `2 × spawn_slots` |
| V11 | the Interleaf (pos 4) lies at 4000..=6000 Bp of the Incipit→Colophon path distance |
| V12 | `room.no_mount` tags match §16.4 |
| V13 | one Colophon with 5 door volumes (3 lattice, 1 secret, 1 "Close the Book" or Margins, doc 14 §3.6); secrets only via Hidden edges; ≤ 1 secret exit, ≤ 1 seam |
| V14 | at least one destructible block inside the Colophon |

### 15.3 Repair and retry

- V1–V5: carve a 1 × 2 passage along A* from the reachable frontier to the target through non-protected cells (air 1, solid 5), at most 3 carves per attempt, no draws. V11: swap the Interleaf with the main-path Room whose fraction is nearest 5000 Bp (ties to the lower `NodeId`).
- Still failing: `attempt += 1`, rerun stages 1–10 (attempts 0..=3); attempt 4 forces `rooms_min`, no minor cycles and `cycle_hidden_shortcut`; then the CI-validated authored fallback `room.tpl.fallback.<layout>` in the genre skin.
- V6–V14: fixed in place without draws: remove offending hazards, liquids, anchors or secret exits (highest index first), add safe-room light (§11.3 step 3), recompute tags.

## 16. Room catalog

### 16.1 Kinds

`fp` footprint in cells, `st` storeys, `max` per Folio, `w` typing weight (— = rules only), `pop` population weight (§12), `lw` loot weight (§13). Sanctuaries per doc 14 §3.3.

| ID | fp | st | max | w | verb | pop | lw | Tags and function |
|---|---|---|---|---|---|---|---|---|
| `room.incipit` | 1×1–2×2 | 1 | 1 | — | — | 0 | 0 | safe, sanctuary; spawn and respawn point; boss antechamber |
| `room.colophon` | 2×2 | 1 | 1 | — | — | 0 | 0 | safe, sanctuary, no_bleed; doc 14 §3.9 services, exit doors |
| `room.interleaf` | 1×2–2×2 | 1 | 1 | — | — | 0 | 0 | safe, sanctuary; Hearth, vendor |
| `room.editor_checkpoint` | 2×2 | 1 | 1 | — | — | 3 | 0 | boss approach (§21.1) |
| `room.boss_arena` | per §21.2 | 2 | 1 | — | — | 0 | 0 | no_bleed; doc 13 §9 geometry |
| `room.campfire` | 1×1–2×2 | 1 | 1 | — | — | 0 | 0 | safe; rest (§16.3) |
| `room.arena` | 3×3 | 2 | 1 | — | — | 0 | 0 | mini-boss room (doc 13) |
| `room.key_room` | 1×1–2×2 | 1 | — | — | — | 2 | 0 | key pedestal |
| `room.treasury` | 1×2 | 1 | 2 | 40 | — | 1 | 3 | no_mount; locked chests |
| `room.reliquary_vault` | 1×1–2×2 | 1 | 1 | — | — | 0 | 2 | no_mount; Reliquary spots first |
| `room.gauntlet` | 1×3 | 1 | 3 | 60 | traverse | 0 | 2 | 5–7 hazards; chest at the end |
| `room.puzzle` | 2×2 | 1 | 1 | — | — | 0 | 2 | §16.3 |
| `room.cistern` | 2×2 | 1–2 | 1 | 40 | traverse | 2 | 1 | basins always filled |
| `room.chasm` | 1×3 | 1 | 1 | 40 | traverse | 1 | 1 | bridge over `hazard.chasm` |
| `room.hall` | 2×2–3×3 | 1–2 | — | 300 | melee | 4 | 2 | combat |
| `room.guardpost` | 1×1–1×2 | 1 | — | 200 | shoot | 2 | 1 | stealth |
| `room.barracks` | 2×2 | 1 | 2 | 120 | melee | 5 | 2 | sleepers |
| `room.crossroads` | 2×2 | 1 | — | 100 | — | 2 | 1 | hub |
| `room.gallery` | 1×3 | 1 | — | 120 | sneak | 2 | 1 | stealth |
| `room.ambush_den` | 2×2 | 1 | 2 | 50 | sneak | 3 | 1 | stealth, dark |
| `room.overlook` | 2×2 | 2 | 1 | 40 | traverse | 2 | 1 | atrium |
| `room.shrine` | 1×1 | 1 | 1 | 80 | pray | 1 | 1 | patron altar (doc 06); MYT ≥ 1 |
| `room.library` | 2×2 | 1–2 | 1 | 70 | cast | 1 | 2 | no_mount; 4 lore sockets |
| `room.observatory` | 2×2 | 2 | 1 | 30 | cast | 1 | 2 | 2 lore sockets |
| `room.alchemy_lab` | 1×2 | 1 | 1 | 60 | craft | 1 | 1 | no_mount; `station.alchemy` (docs 07, 08) |
| `room.forge` | 2×2 | 1 | 1 | 60 | craft | 2 | 1 | smithing station (doc 07) |
| `room.workshop` | 2×2 | 1 | 1 | 50 | craft | 2 | 2 | tinkering station (doc 07) |
| `room.armory` | 1×2 | 1 | 1 | 70 | shoot | 2 | 3 | no_mount; racks |
| `room.prison` | 2×2 | 1 | 1 | 50 | talk | 3 | 1 | captive hireling (doc 13) |
| `room.garden` | 2×2–3×3 | 1 | 1 | 60 | craft | 1 | 1 | H nodes, food |
| `room.larder` | 1×1 | 1 | 1 | 50 | craft | 1 | 1 | no_mount; food |
| `room.crypt` | 2×2 | 1 | 1 | 60 | pray | 3 | 1 | undead roster |
| `room.server_room` | 1×2–2×2 | 1 | 1 | 40 | hack | 2 | 2 | electronic; local TEC ≥ 5 |
| `room.stable` | 2×2 | 1 | 1 | 30 | traverse | 1 | 0 | mount bay |
| `room.shop` | 1×2 | 1 | 1 | 20 | talk | 0 | 0 | safe, no_mount; vendor (doc 11) |
| `room.scriptorium` | 2×2 | 1 | 1 | 20 | — | 0 | 0 | safe, no_mount; §16.3 |
| `room.fortune_hall` | 2×2 | 1 | 1 | 30 | — | 0 | 0 | safe; 2 Fortune Engines (doc 11) |
| `room.illumination_vault` | 1×2 | 1 | 1 | 40 | — | 0 | 2 | unlock-gated; sigil-sealed (§16.3); one `loot.secret.vault` roll; flag `illumination_vault` |
| `room.hireling_hall` | 2×2 | 1 | 1 | 30 | talk | 0 | 0 | unlock-gated; doc 13 hirelings |
| `room.bleed_shrine` | 1×1 | 1 | 1 | 40 | pray | 1 | 0 | unlock-gated; needs a bleed zone; §14.3 |
| `room.duel_page` | 2×2 | 1 | 1 | 30 | melee | 0 | 1 | unlock-gated; a one-aspect champion guards a chest |

With the 8 secret rooms of §17.2 the catalog has 49 kinds.

### 16.2 Typing weight

```
w(kind, genre) = mul_bp(mul_bp(base_w, verb_bp(genre, kind.verb)), bias_bp(genre, kind))
```

`verb_bp` is 10000 for kinds without a verb, `bias_bp` defaults to 10000, and a failed requirement or a missing unlock gives 0. Examples: Cyberpunk server room `mul_bp(mul_bp(40, 16000), 20000) = 128`; Mythic Fantasy library `mul_bp(mul_bp(70, 15000), 12000) = 126`; Wuxia hall `mul_bp(300, 16000) = 480`.

Room bias (Bp): MF armory 12000, library 12000. GH crypt 20000, prison 12000, garden 5000. CH library 20000, observatory 15000, cistern 15000. SO server_room 15000, garden 3000, crypt 0. CP server_room 20000, garden 3000, shrine 5000, crypt 0. PA workshop 15000, garden 6000, library 5000. WW stable 15000, prison 12000. HN prison 15000, gallery 15000, garden 3000, crypt 3000. WX overlook 15000, garden 12000. SP workshop 20000, forge 15000. FT garden 20000, larder 15000. MA shrine 20000, crypt 12000. ON overlook 20000, observatory 15000. RD gallery 20000, server_room 15000, garden 0, stable 0, crypt 0, shrine 0. UW shop, scriptorium and fortune_hall 0. UT none.

### 16.3 Services and puzzles

- **Campfire.** Interact 300 uninterrupted ticks while not `in_combat`: heals `mul_bp(max_hp_mhp, 5000)` through Canon §10.9, refills stamina and aether, restores 25 composure; once per Erratum per campfire.
- **Scriptorium room:** the Colophon station's Rewrite and identification (doc 10, doc 11), plus perk-column choice (Canon §13.9), reattachment (Canon §9.1) and affliction cure for `mul_bp(500, P(d))` Lucre (Canon §16.4). `flags.scriptorium_room` is set when one exists.
- **Puzzles**, variant by `item_rng("puzzle", i)` `weighted([lever 40, plates 30, sigils 30])` (lever forced under `law.steampunk.clockwork`): *lever order*, 4 levers in an order from one shuffle of `[0, 1, 2, 3]`, inscribed on 2 lore objects elsewhere, a wrong pull resets and fires one `hazard.alarm` effect; *plates*, 3 plates each bearing ≥ 40,000 g on one tick (an Erratum is 75,000 g; placeables count); *sigils*, 3 sigil blocks each at `light_block >= 12` on one tick, which players solve with placeable light. Solving opens the `PuzzleGate` and the room's chest.

### 16.4 `room.no_mount`

A room carries `room.no_mount` when its kind lists it, or when a feet cell on a path between two of its doors has under 3 blocks of headroom (3000 mm; a mount needs 2600 mm, Canon §17.7). Summoning there fails; a rider entering is dismounted with `ev.dismounted` (doc 05); doc 13 cavalry spawns dismounted. Corridors are never tagged.

## 17. Secrets

### 17.1 Count

`n_secrets = floor_div(n_rooms × secret_bp, 10000)`, plus 1 on `roll_bp(n_rooms × secret_bp % 10000)` (stage `graph`, 1 call); rest and boss Folios have exactly 1; `lattice.marg.annotated` adds 2. Example: 18 Mythic Fantasy rooms give `18 × 1500 = 27000`: 2 secrets, a third at 7000 Bp.

### 17.2 Secret room kinds (1 weighted call per Secret node; each 1 × 1 cell, 1 storey)

| ID | w | Contents (doc 11 tables) |
|---|---|---|
| `secret.room.cache` | 30 | `loot.secret.cache` |
| `secret.room.shrine` | 15 | hidden patron altar; its first prayer gains +100 favor (doc 06) |
| `secret.room.reliquary` | 10 | `loot.secret.vault` |
| `secret.room.armory` | 15 | 2 racks with at least one counter source (Canon §17.3a) |
| `secret.room.editors_desk` | 5 | `loot.secret.palimpsest` (Canon §13.3); `d >= 6` |
| `secret.room.margin_note` | 15 | 1 story lore object and a cache |
| `secret.room.oubliette` | 7 | a captive hireling (doc 13) |
| `secret.room.ink_well` | 3 | a `liq.red_ink` basin granting one `afflict.red_ink` dose (Canon §16.4); `d >= 11` |

### 17.3 Hiding methods

| ID | Hides by | Found by |
|---|---|---|
| `secret.hide.cracked_wall` | a destructible wall block with a hairline-crack variant | passive detection (Canon §6.4 row 35, keyed `rng.stealth`, once per observer per 60 ticks within 3 m); a hit plays a hollow knock; breaking it |
| `secret.hide.false_wall` | a passable block rendered as wall | passive detection; walking into it |
| `secret.hide.lever` | a lever disguised as a prop opens a wall section | passive detection outlines it |
| `secret.hide.crate_stack` | the opening stands behind destructible crates and barrels | breaking or moving them |
| `secret.hide.waterfall` | the door sits behind a liquid curtain | seen when an entity crosses the curtain |
| `secret.hide.ceiling_hatch` | a hatch 4 blocks up | passive detection; a placeable ladder |
| `secret.hide.verb_lock` | a seal needing another genre's verb: hack (TEC), prayer (MYT) or spell (ARC) | that verb; detection names it |
| `secret.hide.dream_veil`, `.clearance`, `.blank_page`, `.palimpsest_wall` | secret exits | §17.4 |

Method per secret: `weighted([cracked 30, false 20, lever 15, crates 10, waterfall 5, hatch 10, verb 10])`, impossible ones 0. Revealing a secret emits `ev.secret_found` (G9).

### 17.4 Secret exits to the four secret genres

Stage `secret` tests targets in the order Oneiric, Redaction, Unwritten, Ur-Text with `roll_bp(chance)` until one succeeds (1 call per eligible target tested). A target is eligible when Folio `n + 1` is neither a boss nor depth 25, and:

| Exit | Chance | Condition | Hiding and discovery |
|---|---|---|---|
| `secret.exit.oneiric` | GPV `secret_exits` (§5.3) | `n >= 3` | `dream_veil`: invisible until an Erratum completes a campfire rest in the Folio; worldgen adds a `room.campfire` when absent |
| `secret.exit.redacted` | GPV value | `n >= 6`, local TEC ≥ 5 | `clearance`: an electronic lock (doc 09) opened by Hacking, or automatically when an Erasure hunter dies in the Folio |
| `secret.exit.unwritten` | 1000 | `n >= 8` | `blank_page`: visible and passable only for Errata with ≥ 5 `st.erasure` stacks |
| `secret.exit.ur_text` | 800 | `n >= 26` | `palimpsest_wall`: dissolves while a Palimpsestic item is within 3 m |

The room holds a dog-eared page; interacting with it **discovers** the exit, and doc 14 §3.6 appends a door to that genre after the lattice doors. Generation stays pure, so the room exists regardless of history; when the target genre already appeared this run (before Marginalia) or in the last 10 Folios (Marginalia), discovery appends no door. Every secret-exit room also holds a `loot.secret.cache`, so it always pays.

## 18. Hazards and traps

### 18.1 Archetypes

`base` is display HP (Canon §10.6); buildup scales by `S(d)`; hidden hazards use passive detection and Security disarming (doc 09); electronic ones need local TEC ≥ 3.

| ID | Trigger and cycle | Effect | dep |
|---|---|---|---|
| `hazard.dart_trap` | hidden plate | 3 darts, 25000 mm/s, 6 `dmg.pierce` each | none |
| `hazard.blade_trap` | sweep every 60 ticks | 14 `dmg.slash`, 30 bleeding | none |
| `hazard.spike_pit` | destructible lid breaks at ≥ 40,000 g | 3-block fall + 10 `dmg.pierce`, 40 bleeding | none |
| `hazard.crusher` | slam every 120 ticks, 30-tick telegraph | 25 `dmg.blunt` | none |
| `hazard.flame_jet` | on 60 of 180 ticks; 4 m cone, half-angle 2731 Bam | 8 `dmg.fire` per pulse, 60 burning | none |
| `hazard.gas_vent` | hidden plate | 6 cells of `gas.miasma` over 120 ticks | none |
| `hazard.shock_floor` | electronic; 3 × 3 live 90 of 180 ticks | 6 `dmg.shock` per pulse, 40 shocked | techne |
| `hazard.alarm` | hidden tripwire or sensor | `ev.noise` 90 dB at the source | none |
| `hazard.net_snare` | hidden tripwire | `st.rooted` direct; 3 × 3 `block.web` | none |
| `hazard.rune_ward` | hidden glyph | 3 m blast, 18 `dmg.arcane`, 50 unraveled | arcana |
| `hazard.curse_glyph` | hidden glyph | 60 dread and 40 withering buildup within 2 m | mythos |
| `hazard.camera` | electronic; cone 8192 Bam, 16 m, sweep ±4096 Bam per 360 ticks | 60 ticks of sight on an Erratum = alarm; hackable | techne |
| `hazard.collapsing_floor` | 3 × 3 destructible boards break 30 ticks after ≥ 40,000 g | drop below; OneWay in validation | none |
| `hazard.falling_debris` | `ev.noise` ≥ 80 dB within 6 m | 3 destructible rubble blocks fall, 12 `dmg.blunt` each | none |
| `hazard.lava_pool`, `.acid_pool`, `.deep_water`, `.oil_slick`, `.ink_seep` | contact | Canon §12.1; deep water ≥ 3 blocks; oil 100 ml per cell | none |
| `hazard.steam_vent` | on 60 of 240 ticks | 2 cells of `gas.steam` | none |
| `hazard.rad_zone` | 4 × 4 floor | 20 irradiated per pulse | none |
| `hazard.miasma_bog` | persistent | `gas.miasma` 3 × 3 × 2 | none |
| `hazard.web_field` | persistent | `block.web` 3 × 3 × 2 | none |
| `hazard.quicksand` | 3 × 3 mire | `st.slowed` while inside; sinks at most 1 block | none |
| `hazard.hull_breach` | destructible panel destroyed (30 damage) | 3000 mm/s² pull within 8 m for 300 ticks, then seals | techne |
| `hazard.redaction_bar` | 1 × 3 × 8 slab sliding at 2000 mm/s | conceal 15; contact applies `st.silenced` | techne |
| `hazard.chasm` | pit ≥ 6 blocks deep | fall damage | none |
| `hazard.gravity_flip` | a §9.5 flip volume | gravity reversed | none |

Examples against Errata: a dart at `d = 10` deals `mul_bp(6000, 16190) = 9714` mHP (13,030 to an NPC via `P(10)`); a crusher at `d = 15` `mul_bp(25000, 21160) = 52900`; a flame pulse at `d = 5` `mul_bp(8000, 12388) = 9910`.

### 18.2 Per-genre hazard lists (skin names)

Weights 20, 20, 15, 15, 15, 15 by position; skins are `loc.<hazard id>.skin.<genre_suffix>.name` (doc 18).

| Genre | Hazards |
|---|---|
| MF | dart_trap (arrow slit), spike_pit (oubliette), blade_trap (pendulum axe), rune_ward (warding rune), flame_jet (dragon-mouth brazier), collapsing_floor (rotten boards) |
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
| UT | rune_ward, camera, curse_glyph, flame_jet, ink_seep, collapsing_floor, each skinned by a core genre in lattice order cycling per room |

Lint L4 holds: nothing electronic in MF, WX, FT, MA, UW; no arcana hazard in PA, HN, UW; no mythos hazard in UW.

## 19. Ambient and narrative events

### 19.1 Archetypes

`requires` reads the target room's local genre; gated events need their unlock in the run's content set.

| ID | Class | Effect | requires |
|---|---|---|---|
| `gpv.evt.light_failure` | ambient | the target room's fixtures go dark for 600 ticks | — |
| `gpv.evt.weather_front` | ambient | 1800 ticks: fog +1 per 8 m, doc 09 sound attenuation × 12000 | — |
| `gpv.evt.distant_noise` | ambient | `ev.noise` 70 dB at least 2 rooms from every Erratum | — |
| `gpv.evt.tremor` | ambient | fires the room's falling debris, else drops 2 rubble blocks | — |
| `gpv.evt.whispers` | ambient | −5 composure to each Erratum there with `light_at < 8` | MYT ≥ 1 |
| `gpv.evt.boon` | ambient | `st.blessed` to each Erratum there | MYT ≥ 1 |
| `gpv.evt.lore_echo` | ambient | a lore object with an unread story event appears (§20) | — |
| `gpv.evt.editor_glimpse` | ambient | 2,000,000 ml of `liq.red_ink` seeps into the room | — |
| `gpv.evt.gravity_shift` | ambient | every gravity-flip volume toggles for 600 ticks | flip cells |
| `gpv.evt.door_slam` | ambient | the open door nearest an Erratum closes and carries `protect` for 600 ticks | — |
| `gpv.evt.wandering_vendor` | narrative | a `fac.neutral` vendor stays 1800 ticks (doc 11) | `unlock.room.wandering_scribe` |
| `gpv.evt.genre_storm` | narrative | a dynamic zone (§14.3) of the top-weight lattice neighbor covers the room 1200 ticks, moving every 400 ticks to its lowest-`NodeId` neighbor | `unlock.event.genre_storms` |
| `gpv.evt.patrol_shift` | narrative | doc 13 spawns a wave at the anchor nearest the Colophon walking toward the Incipit | — |
| `gpv.evt.ambush` | narrative | doc 13 spawns a wave on 2 spawn-eligible anchors adjacent to the party's room | — |
| `gpv.evt.reinforcements` | narrative | doc 13 spawns a wave at the qualifying anchor farthest from every Erratum | — |
| `gpv.evt.plea` | narrative | freeing a captive within 1800 ticks gives +100 reputation and a hireling offer (doc 13) | — |
| `gpv.evt.duel_challenge` | narrative | a champion (one plan aspect) duels the accepting Erratum behind a barrier, ≤ 1800 ticks | VRV ≥ 5 |
| `gpv.evt.flood` | narrative | a basin releases 20,000,000 ml of its liquid over 600 ticks | basin |
| `gpv.evt.fire_outbreak` | narrative | ignites the flammable block nearest the room center (doc 03) | — |
| `gpv.evt.lockdown` | narrative, electronic | doors of the room and its neighbors shut 900 ticks; cameras activate | TEC ≥ 3 |
| `gpv.evt.redaction_sweep` | narrative | a `hazard.redaction_bar` sweeps the target corridor once | TEC ≥ 1 |

### 19.2 Per-genre event lists (skin names)

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
| ON | gravity_shift (the world turns over), whispers (dream voices), lore_echo (memory surfacing), light_failure (the lights dim), flood (dream sea rising), genre_storm (a dream of elsewhere) |
| RD | redaction_sweep (redaction sweep), lockdown (audit), patrol_shift (enforcer rounds), distant_noise (intercom), editor_glimpse (red ink leak), whispers (compliance announcement) |
| UW | editor_glimpse (ink pools), reinforcements (Editor squad), light_failure (blank-out), distant_noise (pencil scratching), tremor (eraser pass), lore_echo (an unwritten line) |
| UT | lore_echo (every story at once), boon (the Authors' regard), editor_glimpse (the red pen), reinforcements (Editors), tremor (the page shudders), weather_front (ink weather) |

### 19.3 Scheduler (`rng.event`, sequential, host only)

```rust
// Ticks with t - entry >= 1800 and (t - entry) % 1200 == 0, while the Colophon is closed and < 6 events fired.
fn event_check(r: &mut Rng, f: &FolioState) {
    if !r.roll_bp(f.gpv.event_bp) { return; }                                   // 1 draw
    let elig = f.gpv.events.filter(|e| f.cooldown_ok(e) && requires_ok(e, f));  // list order
    if elig.is_empty() { return; }
    let e = elig[r.weighted(&weights(&elig))];                                   // 1 draw
    let rooms = target_rooms(e, f);              // rooms holding or adjoining an Erratum, ascending index
    fire(e, rooms[r.range(rooms.len() as u64) as usize], f);                     // 1 draw; ambush 2 more
}
```

Narrative events stay 2400 ticks apart; none repeats in a Folio; boss and rest Folios get none. `gp.ambush_extra` (doc 14) adds guaranteed ambushes at `entry + 5400 × k`, `k = 1..=ambush_extra`, outside the cap, 3 draws each. Complications (§4) run after the event check on shared ticks. Example: a 360 s Gothic Horror Folio runs `floor_div(21600 - 1800, 1200) + 1 = 17` checks; `stream_seed(0x0123456789ABCDEF, "rng.event", 2) = 0x9ad6720d21d175fe` first yields 2892 < 4000, so the first check succeeds.

## 20. Folio story fragments

Following Caves of Qud's sultans (R1 §5.9, R4 §7.3), stage `story` draws a life in events, rationalizes gaps afterwards, and scatters biased, contradictory retellings.

### 20.1 Cast

Two `next_u64` name seeds (hero, rival; doc 18 grammar) and 2 of the 4 `story_domains` without replacement (2 equal-weight calls). The **seat** is the lowest-`NodeId` treasury, vault or library, else the highest-`lw` room.

### 20.2 Events and rationalization

Slots origin, rise, deed, fall, erasure; erasure is always `gpv.story.erasure.scraped`. For each other slot, templates whose `requires` tags are all held are eligible, weight `10 + 20 × [template domain ∈ domains]`, 1 weighted call.

| Template | requires | grants | domains |
|---|---|---|---|
| `gpv.story.origin.foundling` | — | `orphan` | any |
| `gpv.story.origin.heir` | — | `noble`, `rival_kin` | oath, empire, jade |
| `gpv.story.origin.apprentice` | — | `trained` | machine, steam, data |
| `gpv.story.rise.oath_sworn` | `noble` or `trained` | `oath` | oath, sword |
| `gpv.story.rise.relic_found` | — | `relic` | any |
| `gpv.story.rise.pact` | `orphan` | `pact` | bargain, hex, deep |
| `gpv.story.deed.slew_beast` | `relic` or `oath` | `renown` | wyrm, wolf, sea |
| `gpv.story.deed.built_wonder` | `trained` | `renown`, `wonder` | brass, clock, chrome |
| `gpv.story.deed.stole_fire` | `pact` | `renown`, `curse` | flame, star, data |
| `gpv.story.fall.betrayed` | `rival_kin` or `renown` | `betrayed` | lie, money, blood |
| `gpv.story.fall.hubris` | `renown` | `fallen` | hubris, star, sky |
| `gpv.story.fall.cursed` | `curse` or `pact` | `fallen` | moon, madness, crypt |
| `gpv.story.bridge.met_rival` | — | `rival_kin` | any |
| `gpv.story.bridge.won_fame` | — | `renown` | any |
| `gpv.story.erasure.scraped` | — | `erased` | any |

**Rationalization.** When a slot has no eligible template, insert the bridge whose grant unblocks that slot's highest-weight blocked template (ties to the earlier row) and draw again; at most 2 bridges. Example: apprentice → relic_found → built_wonder grants `renown`, so hubris and betrayed are eligible for the fall and no bridge is needed.

### 20.3 Retellings and placement

- `L = 3 + floor_div(n_rooms, 6)` lore objects, each showing one event in one voice. Events are assigned round-robin over one shuffle of the event list, so each appears once when `L >= events`.
- Voice per object: `weighted(praise 40, slander 40, redact 20)`. Every third object in placement order is **overwritten**: hero and rival swap roles in its text, so two objects contradict each other on purpose.
- Sockets: 1 weighted call per object over lore sockets weighted by room (library 4, shrine 3, observatory 2, server room 2, crypt 2, others 1); one goes to `secret.room.margin_note` when present.

### 20.4 Reading and solving

Reading is a 60-tick interact granting Inscription XP (doc 04, magnitude `d`). Once the party has read 3 distinct events the story is **solved**: the best secret (editor's desk, then Reliquary, then §17.2 order) is revealed, doc 11's seat relic (affix themes = the 2 domains) is identified (doc 10), the Codex records it (doc 14), and `ev.objective_completed` is emitted.

### 20.5 Folio Leaf and text

One `cat.quest` item per Folio, the Folio Leaf (`item.quest.folio_leaf`, doc 11), lies at the seat; carried through an exit it records the story in the Codex. Text comes from `loc.gpv.story.*` grammars (doc 18) or the Scribe model and never enters simulation state.

```json
{ "hero_name": "0x9c1f00d2a4be7731", "rival_name": "0x13a4c0ffee0f1e2d", "domains": ["brass", "clock"],
  "events": ["gpv.story.origin.apprentice", "gpv.story.rise.relic_found", "gpv.story.deed.built_wonder", "gpv.story.fall.hubris", "gpv.story.erasure.scraped"],
  "lore": [{"room": 4, "event": 2, "voice": 0, "overwritten": 0}, {"room": 9, "event": 0, "voice": 1, "overwritten": 0}, {"room": 11, "event": 3, "voice": 2, "overwritten": 1}] }
```

## 21. Boss Folios and Editor arenas

### 21.1 Layout

`room.rule.boss_spine`: Incipit (antechamber sanctuary, doc 14 §3.7) – Editor checkpoint (≥ 24,000 mm of path on, holding doc 13 §10.1's squad) – boss arena – Colophon. The Endpaper plays in the arena; the Colophon unseals per doc 14 §3.2; Erasure pauses while the Editor lives.

### 21.2 Arena construction

Geometry is doc 13 §9.2's (the Proofreader's round reading room, the Redactor's archive, the Continuity Editor's mezzanine hall, the Abridger's platform cross over a void at least 8 m deep, the Last Word's four-quadrant ring); Marginalia reprises reuse them.

1. Box: `ceil_div(interior_m + 2, 16)` cells per axis, `ceil_div(ceiling_m + 2, 8)` storeys (Proofreader 3 × 3 × 2; Last Word 4 × 4 × 2).
2. Shell and floor from the Folio genre's primary tileset; doc 13 features are authored prefabs `room.tpl.arena_<boss>`, not WFC.
3. `bright_bp` 6000; 2 `liq.red_ink` pools (3 × 3 × 1) at the edge (doc 08).
4. Doors to the checkpoint and the Colophon, `protect` while sealed (doc 13 §9.1).
5. The mini-boss `room.arena` is a generic 3 × 3-cell, 2-storey box with `4 + floor_div(cover_bp, 1000)` 2 × 2 pillars at radius 12 blocks, pillar `k` at yaw `floor_div(65536 × k, p) + phase`, `phase = range(65536)` (1 draw); Space Opera gets 10 pillars.

## 22. Marginalia modifiers

From Folio 26 each Folio has `k = min(4, 1 + floor_div(n - 26, 10))` modifiers, drawn in stage `marg` without replacement (k weighted calls, excluded genres at 0) and previewed; they stack with doc 14's run-long marginal mutations.

| ID | w | Excluded | Effect |
|---|---|---|---|
| `lattice.marg.annotated` | 60 | — | +2 secrets |
| `lattice.marg.bleeding` | 80 | RD, UW, UT | drift certain, +1 drift genre |
| `lattice.marg.brittle` | 60 | — | every crate, barrel and wooden block is destructible at half doc 07 HP |
| `lattice.marg.champions` | 60 | — | doc 13 doubles the champion count |
| `lattice.marg.dense` | 100 | — | `density_bp` × 12500 |
| `lattice.marg.erased` | 40 | — | +3 `hazard.ink_seep` |
| `lattice.marg.flooded` | 60 | UW | basin chance 10000; +2 basins per cistern |
| `lattice.marg.sealed` | 40 | UW | no mid-Folio shop rooms; +2 chests |
| `lattice.marg.silent` | 60 | — | doc 09 sound attenuation × 7000 |
| `lattice.marg.trapped` | 80 | — | `trap_bp` × 20000 |
| `lattice.marg.unlit` | 80 | — | `bright_bp` × 5000; fixtures emit `fixture_level - 2` |
| `lattice.marg.volatile` | 60 | UW | +1 flame jet or oil slick per 4 rooms |

Examples (run `0x0123456789ABCDEF`): Folio 26 Gothic Horror `[annotated]`; Folio 36 Mythic Fantasy `[annotated, champions]`; Folio 36 Unwritten `[dense, unlit]`.

## 23. Abridged and Prologue layouts

- **Abridged** (Canon §17.1, doc 14 §3.1): Folio `n ∈ 1..=15` has positions `[1, 4, 5]` per Volume and depths 1, 3, 5, 6, 8, 10, 11, 13, 15, 16, 18, 20, 21, 23, 25. Position-4 Folios carry the Interleaf; position 5 is the boss; Folio 15 is the Ur-Text. Abridged passes `folio_scale_bp` 9000 (doc 14 §8.1). Streams and EntityIds use `n`; depth-driven values use `d`. Secret exits come only from position 1. Abridged has no Marginalia.
- **Prologue** (`mode.prologue`, doc 14): `generate_folio` returns the authored chain `room.prologue_l1` … `room.prologue_l7` with no draws.

## 24. Determinism, tests and owned statuses

### 24.1 Determinism rules

1. `generate_folio` reads only `GenInput` and `ContentDb` (with the run's content set); equal inputs give equal `blueprint_hash` on every platform (Canon §21 test 31).
2. Every draw names its sub-stream (§7.2); per-room and per-chunk work uses `item_rng`, so `par.map` order is irrelevant.
3. Lists are sorted before selection, ties to the lower index; no `HashMap` or `HashSet`.
4. Plan functions are recomputed, never cached; preview and generation share code.
5. Events, dread and complications use `rng.event` on the host only.

### 24.2 CI

- PR sweep: 50 seeds × 16 genres × 4 types pass V1–V14 within 3 attempts, fallback in fewer than 4.
- Nightly: 1000 seeds × 16 × 4; p50 > 110 ms or p99 > 260 ms on the Steam Deck runner fails.
- 32 golden `(GenInput, blueprint_hash)` pairs agree on x86_64 and aarch64 Linux, x86_64 Windows and wasm32.

### 24.3 `st.contradicted` and `st.unmoored` (owner 12)

- **`st.contradicted`** is present exactly while an equipped component of the bearer has `C >= 3` (Canon §11.1), re-evaluated on each `ev.contradiction_changed`; UI only.
- **`st.unmoored`** (Canon §5.11): each Erratum keeps the ticks of its last 3 Contradiction failures (jams, fizzles, sparks). A failure at tick `t` whose oldest kept failure is at `t - 600` or later applies the status for 300 ticks (refresh). While it lasts: Death's Door (Canon §10.8 step 1) is skipped and not marked used; the bearer's own drops roll rarity with `SER = 40` (Canon §13.4); components at `C = 1` or `2` use the `C = 3` row (potency 7000, failure 500). It ends early on the tick the bearer's origin enters a `room.colophon` cell or a `zone.seam` cell. Example: failures at ticks 1000, 1300 and 1590 apply it at 1590 until 1890; a fourth at 1700 refreshes it to 2000.

## Exports

- `genre.*`: the full GPV of all 16 genres (§2, §3).
- `law.*`: the 30 World Laws of §4.
- `zone.plain`, `zone.bleed`, `zone.seam`, `zone.dynamic` (§14).
- `lattice.node.margin`; `lattice.edge.<from_suffix>.<to_suffix>` (§5.2–§5.3); `lattice.folio.standard`, `.challenge`, `.rest`, `.boss`; `lattice.challenge.gauntlet`, `.siege`, `.heist`, `.duel`, `.race`; the 12 `lattice.marg.*` of §22.
- `gpv.layout.*` (16, §3.4); `gpv.evt.*` (21, §19.1); `gpv.story.*` (15, §20.2).
- `room.*`: the 41 kinds of §16.1 and `room.prologue_l1` … `room.prologue_l7`; tags `room.safe`, `room.sanctuary`, `room.stealth`, `room.no_bleed`, `room.no_spawn`, `room.no_mount`, `room.electronic`; templates `room.tpl.*` including `room.tpl.fallback.<layout>` and `room.tpl.arena_<boss>`; the 25 rules `room.rule.*` of §8.2.
- `biome.<suffix>.<name>`: the 32 tilesets of §3.3.
- `hazard.*`: the 28 archetypes of §18.1.
- `secret.room.*` (8), `secret.exit.*` (4), `secret.hide.*` (11) (§17).
- Requested outside this chapter's namespaces (Canon gaps G9): events `ev.secret_found`, `ev.objective_completed`; template `item.quest.folio_leaf` (doc 11).

## Interface assumptions

- **I1 (doc 03):** doc 03 §10 light (worldgen calls the same propagation); applies Canon §5.9 physics, `gsign`, relinks, `protect`, `ambient_decic`; instantiates records in §7.1 order.
- **I2 (doc 16):** peers compare `blueprint_hash`; a mismatch is a desync.
- **I3 (doc 07):** the §5.9 materials, per-genre fixture, wall, floor and trim blocks with `destructible` where §9.2 says, slabs, resource-node yields, `station.*`.
- **I4 (doc 08):** the liquids of §3.5, `liq.wellspring`, `gas.miasma`, `gas.steam`; automata start from settled worldgen liquids.
- **I5 (doc 13):** budgets over `spawn_slots`, spawn cells and anchors; a preview-recomputable aspect set (§7.3); civic packs, territory, war fronts, rescue natives, hirelings; §4 Law waves and Nemesis; `genre_shift` (§14.3); mini-boss arenas fitting `room.arena`.
- **I6 (doc 11):** fills chests, floor spots, guarded spots, Incipit caches and Reliquary spots; secret tables; the story relic; `item.quest.folio_leaf`.
- **I7 (doc 09):** `lock.*` for Locked edges and the clearance lock; disarming; camera alarms; fog and sound attenuation.
- **I8 (docs 04, 05, 06, 10):** reading XP; `law_bound` techniques; dismounting; altars and favor; censored names.
- **I9 (doc 14):** calls `generate_folio` with §7.1 inputs; owns phases, Rubric completion, doors and votes, Colophon, Interleaf, Endpaper, Codex and unlock sets.
- **I10 (docs 17, 18):** exit preview display, transition length; naming and story grammars, `loc.*` skins, seams, palettes, music.

## Canon gaps

- **G1** The contract fixes the header to canon-1.2; the canon on disk is canon-1.4, and every rule here was checked against it.
- **G2** Canon §3.2 has no sub-streams. Assumed: §7.2 derivation; previews recompute seeds without advancing live streams.
- **G3** `FOLIO_1_GENRE` may be a secret Origin. Assumed: §5.6 maps it to a core genre.
- **G4** Assumed: "lattice edge weights" means edge × freshness (§5.4).
- **G5** The repeat rule does not name secret genres. Assumed: once per run before Marginalia (§17.4).
- **G6** Canon §17.1 gives every Colophon a shop, and Canon §5.2 gives the Unwritten none. Assumed: §5.2 wins; Unwritten Colophons have no shop (doc 14 agrees).
- **G7** Canon §5.8 defines `local_genre` by bleed zones only. Assumed: inside a seam `local_genre` is the Folio genre (it "supplies skins") while `local_axiom` is the per-axis maximum.
- **G8** Canon §4.2 has no per-cell gravity sign. Assumed: Oneiric flip cells reverse Y gravity (doc 03 `gsign`).
- **G9** Canon §18 lacks `ev.secret_found` and `ev.objective_completed`, which doc 04 expects from doc 12. Assumed: emitted on secret reveal and on Rubric, challenge and story completion.
- **G10** Canon §5.7 does not list "electronic" things. Assumed: shock floors, cameras, lockdowns, the clearance lock, server rooms, Space Opera doors; doc 13 turrets inherit the rule.
- **G11** Assumed: exits into forced Folios (Standard 24, Abridged 14) still draw 3 unseen candidates.
- **G12** Doc 13 §10.1 puts the Editor squad "in the antechamber" while doc 14 §3.3 makes the antechamber a sanctuary. Assumed: the squad stands in the Editor checkpoint.
- **G13** GPV fields `fixture_level`, `fog`, `ambient_decic`, `liquid_viscosity_bp`, `gas_dissipation_bp` and `damage_affinity` are not canon; docs 03, 08 and 13 request them, and they extend Canon §5.9 without changing it.

## Acceptance tests

`R = 0x0123456789ABCDEF`; abbreviations per §2.3.

1. `stream_seed(R, "rng.lattice", 1) = 0xb93f5f6ccba93fac`; `lattice_candidates(R, 1, CP, [CP])`: totals 6,450,000 / 5,250,000 / 3,900,000, draws 1,106,450 / 3,525,875 / 3,019,215, result `[SO, HN, SP]`.
2. `stream_seed(R, "rng.lattice", 0) = 0xa487e0eae90fea26`; seeded Folio 1 candidates `[MF, HN, SP]`; Daily 2026-10-11 gives `[WW, CH, MF]`.
3. 10,000 random legal histories: 3 distinct candidates, never current, same-Volume, capped or secret; 3 `weighted` calls.
4. `gen_seed(R, 2, genre.space_opera) = 0x1c35d41d581f5eaa`; `stream_seed(R, "rng.event", 2) = 0x9ad6720d21d175fe`, first `range(10000)` 2892.
5. `folio_type`: Folio 2 SO standard (3860), HN rest (9855), SP rest; Folio 3 CP rest, MF challenge; Folio 4 CP challenge, MF and GH standard; Abridged GH Folios 1–15 boss exactly at 3, 6, 9, 12, 15.
6. Rubric at Folio 2: SO catchwords, HN keeper, SP keeper, MF keeper. Chests at Folio 2: SO 4, HN 6, SP 4, MF 4.
7. `bleed_plan` with prev MF: Folio 7 GH (MF, [], none), ON (MF, [MA, GH], none), CH (MF, [FT], none), RD none, GH with `bleed_add` 1 (MF, [CH, HN], none); Folio 2 GH (MF, [HN], seam HN); Folio 3 GH (MF, [], seam MF).
8. Marginalia `k` at 26, 35, 36, 45, 46, 56, 100 = 1, 1, 2, 2, 3, 4, 4; Folio 26 GH `[annotated]`; 36 MF `[annotated, champions]`; 36 UW `[dense, unlit]`.
9. Each preview of tests 1–2 equals its generated Folio's type, bleed plan, modifiers and aspect set.
10. `protect_r`: MF 13, GH 4, CH 4, SO 14, CP 12, PA 11, WW 12, HN 11, WX 11, SP 13, FT 10, MA 12, ON 9, UT 13; RD and UW none; flat-floor coverage MF 265, GH 13.
11. A 2 × 2 hall has 17 `spawn_slots` in SO, 14 in GH, and 0 within 24,000 mm of the Incipit spawn.
12. Hazards: dart at `d = 10` 9,714 mHP (Erratum), 13,030 (NPC); crusher at `d = 15` 52,900; flame pulse at `d = 5` 9,910.
13. Typing weights: CP server room 128, MF library 126, WX hall 480, RD garden 0, illumination vault 0 without its unlock.
14. L1–L7 pass for all 16 records; GH `spawn_block_max` 5 fails L2; dropping `law.wuxia.qinggong` fails L7.
15. Dread: GH in combat from entry holds 1000 Bp at tick 1080 and 0 once the Colophon opens; HN rolls at a constant 500.
16. `st.unmoored`: failures at 1000, 1300, 1590 apply it until 1890; another at 1700 refreshes to 2000; entering a Colophon cell ends it that tick.
17. Same `GenInput` after different histories, and 1-thread versus 4-thread `par`, give identical `blueprint_hash` (100 seeds).
18. 50 seeds × 16 genres × 4 types pass V1–V14 within 3 attempts; fallback < 4 in 3,200.
19. A key behind its own lock fails V2; a sealed pit V4; an MF camera V8; lava 5 blocks from the Incipit spawn V6; an Interleaf at 3000 Bp fails V11 until repaired.
20. Swept safe rooms carry `no_spawn`, and where `spawn_block_max <= 14` every feet cell exceeds it.
21. 2 blocks of path headroom → `room.no_mount`; 3 blocks everywhere → untagged.
22. An Oneiric flip room has clear height 4 and deals no fall damage on entry.
23. After Oneiric at Folio 4, an Oneiric exit found at Folio 9 appends no door but keeps its cache; no secret exit precedes a boss or depth 25.
24. A 360 s GH Folio runs 17 checks, at most 6 events; `ambush_extra = 1` adds an ambush at entry + 5400.
25. Collapsed WFC grids satisfy every face constraint; a contradictory tileset ends in `fallback_fill` after exactly 3 resets.
26. Nightly sweep p50 ≤ 110 ms, p99 ≤ 260 ms (Steam Deck runner), max ≤ 900 ms.

## Invariants annex

```
Σ doc12: gpv ⊗ laws ⊗ lattice ⊗ ftype ⊗ graph ⊗ embed ⊗ wfc ⊗ light ⊗ zones ⊗ validate
W! generate_folio pure(GenInput, ContentDb)            N! party size|account|clock|float → generation
W! GPV A == Canon §5.2 ∧ GPV B == Canon §5.9           W! 1..=3 Laws per genre ⊇ Canon-required
N! Law changes a canon constant (all 30)               W! Law active ⇔ local genre ∨ seam includes it
W! draw ∈ sub-stream(rng.worldgen|rng.lattice|rng.event) N! preview advances a live stream
W! lattice: 3 weighted calls per exit                  W! candidates ∉ {cur, same-Volume, capped, secret}
W! depth 25 ⇒ ur_text                                  W! boss ⇔ pos 5
W! every Folio ends Incipit → … → RubricGate → Colophon W! Interleaf ∈ 4000..=6000 Bp (pos 4)
W! key before lock ∧ no softlock ∧ Rubric reachable    W! V1..V14 before publish; attempts ≤ 4 then fallback
W! terrain indestructible unless destructible          N! digging terrain
W! safe room ⇒ no_spawn                                W! spawn eligibility reads light_block only
N! spawn cell within 24,000 mm path of the Incipit     W! chests = 4 + range(3) per Normal Folio
W! ≤ 1 seam per Folio, 1..=2 rooms                     N! static bleed in redacted|unwritten|ur_text
N! electronic thing where local TEC ≤ 2                N! dep axis with local axiom 0
W! hazard dmg := Canon §10.6                           W! room.no_mount ⇔ kind tag ∨ headroom < 3 blocks
W! secret genre ≤ 1 per run before Marginalia          W! iteration ascending; N! HashMap|HashSet
W! st.unmoored ends on Colophon or seam entry          W! story text cosmetic; N! LLM text → blueprint
```
