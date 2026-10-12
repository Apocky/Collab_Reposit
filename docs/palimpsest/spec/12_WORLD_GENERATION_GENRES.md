# 12 — World Generation and Genres
As of 2026-10-11. Status: BINDING. Conforms to canon-1.11.

This chapter owns everything that turns a run seed into a playable Folio: the full Genre Parameter Vector (GPV) of all 16 genres, the World Laws, the Genre Lattice and its exit offers, Folio types, the generation pipeline from mission graph to validated voxel blueprint, bleed and seam zones, the room catalog, secrets, hazards, events, Folio story fragments with the optional Folio quest and the story records the procedural ending reads, boss arenas, Marginalia modifiers, the Abridged layout and the way Folio Expansion genres join generation. Generation is one pure function in `crate.gen` (doc 02). Every rule here was checked against canon-1.11, including fixed terrain with `block.tag.destructible`, 250 mm micro-voxels, Canon §5.9 identity, spawn light, World Laws and their discovery in play, seams, `st.unmoored`, the Colophon, Abridged mode, invasions, the cut of mounts (canon-1.8) and the removal of item durability (canon-1.10).

## Imports

| Source | Consumed |
|---|---|
| DESIGN_BRIEF §1–§4; DECISIONS.md 15, 16, 26, 37, 46, 53, 54, 55, 56, 65, 72, 95, 99, 100, 101, 103, 105, 106, 109, 110, 114, 115, 116, 117, 121, 130, 132 | Archetype × Skin, axioms, 16 genres, 5 × 5 Folios, Lattice exits, Marginalia; invasions; limited building; softer Erasure (onset 720 s, one hunter per Folio); 250 mm micro-voxels; the procedural ending assembled from Folio story fragments; the first two Folio Expansions (Age of Sail and Pirates, Kaiju and Mecha); genre bleed in 1 Folio in 10 once an account has finished 3 runs; no mounts or vehicles; World Laws discovered in play; no item durability; prominent Folio stories with an optional Folio quest |
| Canon §1–§3 | IDs, integer content, `floor_div`, `mul_bp`, Folio box, `EntityId`, `light_at`, `Rng`, `stream_seed`, the streams `rng.lattice`, `rng.worldgen`, `rng.event` (owner 12), `rng.spawn`, `rng.loot_place`, `rng.stealth`; labeled sub-streams (§3.2) |
| Canon §4, §4.3, §4.4 | extent, footprint 96–192, capsule, step-up, `SAFE_FALL`, caps; fixed terrain, `block.tag.destructible`, terrain stopping every projectile, placeables, resource nodes, no item durability; 250 mm micro-voxel masks and shapes on the 1 m simulation grid |
| Canon §5.2–§5.11 | genre table (copied in §3.2); `dep` and the zero-axis ban; Contradiction; TEC ≤ 2 rule; `local_genre` (bleed zones, and the Folio's genre inside a seam); Law discovery; seam zones; Canon §5.9 identity (copied in §2); World Laws (listed in §4); Going Native; Unmoored |
| Canon §6.4 row 35, §10.6, §11, §12 | passive detection; environmental hazards; `st.contradicted`, `st.unmoored`, Erasure; surfaces, concealment, spawn light |
| Canon §13, §15–§19 | `P`/`T`, Palimpsestic sources; factions, patrons; run constants, Colophon, Abridged, aspects, the retired mount IDs (§17.7), invasions; events; this chapter's namespaces |
| Doc 02 §3, §8, perf table | `crate.gen`, `generate_folio(input, db, par)`, `OrderedMap`; `FolioView.bleed` holds at most 4 bleed slots (entry plus drift); `perf.deck.gen.folio_*` budgets |
| Doc 03 §4.4, §4.7, §5, §10, §14.3 | block shapes, `protect`, `open_tag` and `flip` bits, sky and block light channels and their propagation (§10.3), `gsign`, `ambient_decic`, `climbable` ladders; the registered events `ev.secret_found` and `ev.objective_completed`, which this chapter emits |
| Doc 06 §3.9 | `note_contradiction_failure(t)`, the one per-character Contradiction failure counter (§4 dread, §24.3 Unmoored) |
| Doc 07 §9.3, §9.4, §10.1 | resource-node blocks `block.node.*` and the per-genre vein weights; `block.light_fixture.<genre_suffix>`, `block.masonry.<stone>`, `block.torch`, `block.door.*`, `block.cracked_wall`, `block.planks`, the worldgen patron altar `block.altar`, `block.rubble`, `block.pit_lid` and `block.hull_panel`; placed-block rules; mining time reads GPV `hard_bp` |
| Doc 08 §3, §8.1, §12.5 | `liquid_viscosity_bp`, `gas_dissipation_bp`, H (plant) and F (fungus) harvest nodes by biome, `liq.blood`, `liq.wellspring` font basins, red-ink pools |
| Doc 09 §2.2, §2.4 | light bands, light manipulation, `lock.*`, disarming |
| Doc 11 §9.6–§9.11, §10, §11.1, §12.1 | chest kinds, floor loot (9 per slot, 15 in Abridged), guarded spots, secret loot tables `loot.secret.*`, Reliquary spots, counter caches, Fortune Engine and vendor counts per Folio type, `item.tool.torch`; `item.quest.folio_leaf` (§3.5) |
| Doc 13 §2.5, §6, §7.3, §8, §9, §10, §12, §13 | `damage_affinity`, the aspect set, rosters (`enemy.editors` for the Unwritten), boss arenas, Director placement, territory, war fronts, dynamic bleed zones; captives `npc.captive.<genre_suffix>` and `fac.rep.free_captive`; taming of `beast` creatures |
| Doc 14 §3, §5.3, §7, §8, §9 | positions, Incipit, Rubric (none in rest and boss Folios), Colophon, Interleaf, Endpaper, exit vote and door order, room unlocks, `folio_scale_bp`, `gp.ambush_extra`, bleed-count hooks (`run.mut.wild_bleed`, `run.marg.bleeding_genres`), Marginalia boss positions; `FolioRecord`, run history and run ends (§1.2, §4.3); the account's Index and discovered Laws (§5.5); the Prologue's rooms and zones (§11); content sets and Script unlocks `unlock.room.lost_chapter_<genre_suffix>` (§2.3) |
| Doc 15 §1.1, §3.1.2 | Folio Expansion SKUs `store.sku.folio_exp_<nn>`, one genre each; the Scripts that grant the Lost Chapter rooms |
| Doc 18 §7, §8.5a, §8.7 | name grammars for story seeds, story retelling grammars per §20.2 template, hazard and event skin names; the closing manuscript built from solved Folio stories |
| Doc 21 §2.3 | the Early Access rules for content sets below the 12 base core genres (Canon gaps G20) |
| R6 §7–§11; R4 §7, §14; R1 §5.9; R2 §5 | GPV, Noita modifiers, Torg Laws; WFC in rooms, Dormans cycles; Qud sultans; Minecraft light |

## 1. Vocabulary and namespaces

| Term | Exact meaning |
|---|---|
| Cell | 16 × 8 × 16 blocks; cell `(cx, cs, cz)` covers `x ∈ 16cx..16cx+16`, `y ∈ 8+8cs..16+8cs`, `z ∈ 16cz..16cz+16`; 16 × 6 × 16 cells; `cs ∈ 0..=5` is the **storey**; `y ∈ 0..=7` is foundation, `y ∈ 56..=63` roof |
| Footprint | the genre's `W × D × storeys` cells, centered: `x0 = 16 × floor_div(16 - W, 2)`, likewise `z0` |
| Tile | a WFC unit of 4 × 4 × 4 blocks; a cell holds 4 × 2 × 4 tiles |
| Room | a mission-graph node embedded as an axis-aligned box of whole cells |
| Zone | cells sharing one zone record: plain (Folio genre), bleed, seam or dynamic (§14) |
| Feet cell | an open cell (micro-voxel mask 0, or a doc 03 shape without collision: ladder, fixture, cross) with solid or slab-top support below and 2 open cells above (§10.4, §15.1); solid support is a cell whose top micro-voxel layer (`my = 3`) is fully occupied |
| Micro-voxel | a 250 mm cube; each 1 m cell's `u64` occupancy mask holds 4 × 4 × 4 of them (Canon §4.4). The 1 m cell stays the generation, zone, light, liquid, navigation and spawn grid |
| Path distance | breadth-first step count between feet cells on the §15.1 navigation graph × 1000 mm |
| `d`, `pos` | depth and structural position of a Folio (doc 14 §3.1); `d` is a `u32` of at least 1 (Canon §13.8) |
| `n` | the Folio index that keys streams and `EntityId`s (Canon §1.11, §3.2); equal to `d` except in Abridged |
| Block linear index | `bx + 256 × (bz + 256 × by)` for a block of the Folio box, range `0..=4194303` (the doc 02 `Gidx` order) |
| Pit depth | every worldgen pit (spike pit, collapsing floor) is 2 blocks deep, which every Erratum leaves with a jump and a mantle: the VERVE-0 apex of 1100 mm plus doc 03 §5.9's 1600 mm mantle reach is 2700 mm (§18.1) |
| `rooms_total` | the count of non-secret mission-graph nodes after stage `graph` (Incipit and Colophon included) |

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
| post_apocalypse | `mfr.post_apocalypse.rustmaw`: magazine × 20000, reload time × 13000. `mfr.post_apocalypse.lastditch`: 2000 Bp chance per shot to use no ammo. |
| weird_west | `mfr.weird_west.coffin_creek`: crits ricochet 5000 Bp of their damage to the nearest other hostile within 8 m. `mfr.weird_west.seven_seals`: 2000 Bp `dmg.umbral` share with dependency mythos. |
| hardboiled_noir | `mfr.hardboiled_noir.midnight_avenue`: shot noise capped at 50 dB. `mfr.hardboiled_noir.rainslick`: × 12000 against staggered or knocked-down targets. |
| wuxia | `mfr.wuxia.jade_lotus`: thrown weapons return after 30 ticks. `mfr.wuxia.iron_crane`: a block within 6 ticks of impact reflects 5000 Bp of the damage. |
| steampunk | `mfr.steampunk.brassworth`: +200 Bp per consecutive hit, maximum +3000. `mfr.steampunk.aetherline`: every 4th hit chains 3000 Bp of its damage as `dmg.shock` to 2 targets within 5 m. |
| fairy_tale | `mfr.fairy_tale.thimbleweald`: × 13000 against targets with a larger capsule radius than the wielder. `mfr.fairy_tale.thornwood`: 1000 Bp chance per hit to apply `st.rooted`. |
| mythic_antiquity | `mfr.mythic_antiquity.hephaesteum`: the wielder is immune to `st.disarmed` and `st.corroded`. `mfr.mythic_antiquity.argive`: +1500 Bp damage while an allied Erratum is within 3 m. |
| oneiric | `mfr.oneiric.somnambule`: after 10 m of flight, projectiles pass through destructible and non-opaque blocks without spending penetration (§4.3). |
| redacted | `mfr.redacted.bureau`: never rolls Contradiction failure; the potency loss still applies. |
| ur_text | `mfr.ur_text.first_press`: Palimpsestic items only. |

### 2.3 Identity extensions owned here

`fixture_level`: `light_emit` of the genre's worldgen fixture block `block.light_fixture.<genre_suffix>` (doc 07 §10.1). `protect_r = fixture_level - spawn_block_max - 1`: the taxicab radius one fixture makes spawn-proof (— where `spawn_block_max = 15`). `fog`: concealment per 8 m (Canon §12.2). `ambient_decic`: temperature (doc 03). Viscosity and gas dissipation feed doc 08; `damage_affinity` adds to native NPC resistances (doc 13 §2.5). `hard_bp` multiplies doc 07 §9.2 mining time of resource nodes (alloys and concrete mine slower, rot and dream-stuff faster).

| Genre | fixture_level | protect_r | fog | ambient_decic | liquid_viscosity_bp | gas_dissipation_bp | damage_affinity (Bp) | hard_bp |
|---|---|---|---|---|---|---|---|---|
| MF | 14 | 13 | 0 | 150 | 10000 | 10000 | arcane +1500, fire +1000 | 10000 |
| GH | 12 | 4 | 1 | 80 | 9000 | 8000 | umbral +2000, radiant −1500 | 10000 |
| CH | 12 | 4 | 1 | 60 | 7000 | 7000 | psychic +2500, umbral +1000 | 12000 |
| SO | 15 | 14 | 0 | 210 | 10000 | 15000 | radiation +1500, shock −1000 | 15000 |
| CP | 13 | 12 | 0 | 220 | 11000 | 10000 | toxin +1000, shock −1500 | 13000 |
| PA | 12 | 11 | 0 | 350 | 8000 | 12000 | radiation +2500, acid +1000 | 9000 |
| WW | 13 | 12 | 0 | 320 | 7000 | 13000 | fire +1000, umbral +1000 | 10000 |
| HN | 12 | 11 | 1 | 120 | 12000 | 9000 | pierce +1000, psychic −1000 | 11000 |
| WX | 12 | 11 | 0 | 170 | 12000 | 11000 | slash +1000, sonic −1000 | 10000 |
| SP | 14 | 13 | 0 | 260 | 10000 | 8000 | fire +1500, shock −1000 | 12000 |
| FT | 11 | 10 | 1 | 160 | 10000 | 10000 | arcane +2000, slash −500 | 9000 |
| MA | 13 | 12 | 0 | 290 | 10000 | 12000 | radiant +1000, blunt +1000 | 11000 |
| ON | 10 | 9 | 1 | 200 | 15000 | 10000 | psychic +2000, arcane +1000 | 8000 |
| RD | 15 | — | 0 | 210 | 10000 | 15000 | psychic +2000, sonic +1000 | 15000 |
| UW | 8 | — | 0 | 200 | 10000 | 10000 | none | 10000 |
| UT | 14 | 13 | 0 | 200 | 10000 | 10000 | none | 10000 |

Abbreviations in Canon §5.2 order: MF, GH, CH, SO, CP, PA, WW, HN, WX, SP, FT, MA, ON, RD (Redaction), UW (Unwritten), UT (Ur-Text).

### 2.4 Placed light as defense

1. **Sky light** (doc 03 §10.2) comes only from sky-exposed cells and rooms tagged open (`open_tag`) at the local `ambient_light`, falling 1 per step; enclosed rooms are lit by fixtures, which shape the stealth gradient.
2. **Fixtures** (`block.light_fixture.<genre_suffix>`, `light_emit = fixture_level` of the zone genre, doc 07 §10.1) carry `block.tag.destructible` (Canon §4.3); breaking or snuffing one (doc 09 §2.4) makes its cells spawn-eligible as soon as doc 03's relight lowers them to `spawn_block_max` or below.
3. **Protection:** a cell at taxicab distance `k` is spawn-proof when `fixture_level - k > spawn_block_max`, i.e. `k <= protect_r`. Mythic Fantasy `14 - 0 - 1 = 13`: a fixture 2 blocks above the feet layer covers `|dx| + |dz| <= 11`, `2 × 11 × 12 + 1 = 265` cells. Gothic Horror `12 - 7 - 1 = 4` covers 13: candlelight barely holds the dark.
4. **No light is safe** in the Redaction and the Unwritten (`spawn_block_max = 15`); only `no_spawn` (§12) clears their safe rooms.
5. **Placeable light** (Canon §4.3, §12.3) protects the same way with its own `light_emit`: `block.torch` 14, `block.lantern` 15, `block.campfire` 15, `block.shrine_lamp` 12 (doc 07 §10.1). A torch therefore protects taxicab radius `14 - spawn_block_max - 1`: 13 in every genre with `spawn_block_max` 0 and 6 in Gothic Horror and Cosmic Horror; a lantern 14 and 7. Every `fac.editors` member except an invader erases Errata placeables within 1500 mm (Canon §11.6, §17.9), so light is no defense against Editor squads and hunters, and none at all in the Redaction and the Unwritten. Held lights (doc 03 §10.4 dynamic emitters, capped at 10) never protect where `spawn_block_max >= 10`, and protect radius `9 - spawn_block_max` elsewhere: 9 where it is 0, 2 in Gothic and Cosmic Horror.
6. **Safe rooms** carry `no_spawn` and, where `spawn_block_max <= 14`, keep every feet cell above it (§11.3).
7. **Balance.** Light is a strong defense in the ten core genres with `spawn_block_max` 0 (a single torch clears a 2 × 2-cell hall's floor within radius 13) and a weak one in the two horror genres, where the dark is the genre's identity (Canon §5.9). Light removes spawn cells and wave anchors only (doc 13 §10.1 skips lit spawn cells, and a qualifying wave anchor needs `light_block <= spawn_block_max`, doc 13 §10.4); it never moves the Keeper, the mini-boss, Erasure hunters (doc 13 §10.5 picks hunter anchors by distance alone) or invaders, and members that already spawned walk into lit cells freely. Torch spam is bounded by doc 07's `PLACED_CAP_PER_PLAYER`, `PLACED_CAP_PER_FOLIO` and `TORCH_TICKS` burn-out, and the initial population spawns at Folio entry before any Erratum can place light. At depth the reference numbers hold their shape: a Folio-25 crusher deals `mul_bp(25000, 36145) = 90,362` mHP before armor, 34% of a level-28 Erratum's 268 HP (TGH 60), against 29% of 85 HP at Folio 1 (§18.1).

## 3. Genre Parameter Vector (GPV)

### 3.1 Dimensions

A GPV is one content record `genre.<suffix>` with 65 dimensions. All are simulation inputs (integers or IDs) except those marked P (presentation only).

- **A, Canon §5.2 (12):** 1 `id`; 2 `display` (P); 3–6 `arc`, `tec`, `myt`, `vrv`; 7 `aether_axiom`; 8 `gravity_bp`; 9 `ambient_light`; 10–11 `lucre_skin`, `aether_skin` (P); 12 `tier`.
- **B, Canon §5.9 (9):** 13 `air_accel_bp`; 14 `fall_dmg_bp`; 15 `ground_friction_bp`; 16 `spawn_block_max`; 17 `palette` (P); 18 `music` (P); 19 `enemy_themes` (P); 20 `materials`; 21 `manufacturers`.
- **C, identity extensions (11, §2.3):** 22 `family` (P); 23 `era_tl` (P, `0..=12`); 24 `render_style` (P); 25 `fixture_level`; 26 `fog`; 27 `ambient_decic`; 28 `liquid_viscosity_bp`; 29 `gas_dissipation_bp`; 30 `damage_affinity`; 31 `patron`; 32 `hard_bp`.
- **D, structure (19):** 33 `layout`; 34–36 `fp_w`, `fp_d`, `storeys`; 37–38 `rooms_min`, `rooms_max`; 39 `loop_bp`; 40 `vert_bp`; 41 `corridor_w`; 42 `cover_bp`; 43–44 `bright_bp`, `dark_bp`; 45 `secret_bp`; 46 `trap_bp`; 47 `liquids`; 48 `tilesets`; 49 `cycle_w`; 50 `room_bias`; 51 `node_w` (H plant and F fungus harvest-node weights, doc 08).
- **E, population and play (7):** 52 `roster` (the template prefix `enemy.<suffix>`, doc 13 §8) and 53 `factions` (the native factions `fac.native.<suffix>.<group>` of doc 13 §12.1 in ascending ID; empty for the Unwritten); 54 `density_bp`; 55 `verbs` (10 × Bp: shoot, melee, cast, pray, sneak, talk, hack, craft, build, traverse); 56 `hazards`; 57 `events`; 58 `event_bp`.
- **F, bleed, Laws and story (7):** 59 `bleed_ok`; 60 `drift_bp`; 61 `bleed_in_bp`; 62 `seam_bp`; 63 `secret_exits`; 64 `laws` (§4); 65 `story_domains`.

**Where each genre's values live.** Every dimension is given for all 16 genres in exactly one table or list of this chapter; the content record copies it field for field, and lint L2 compares Blocks A and B against the canon.

| Dimensions | Values for all 16 genres |
|---|---|
| 1–12 | §3.2 (Canon §5.2 verbatim) |
| 13–20 | §2.1 (Canon §5.9 verbatim) |
| 21 | §2.2 (Canon §5.9 verbatim); `genre.unwritten` has `[]` |
| 22–24, 31 | the paragraph below |
| 25–30, 32 | §2.3 |
| 33–36, 37–38, 39–46, 48 | §3.3 |
| 47, 65 | §3.5 lists |
| 49, 51 | §3.3 lists |
| 50 | §16.2 room-bias list |
| 52–53 | `enemy.<suffix>`; doc 13 §12.1 faction table |
| 54, 58, 55, 59–62 | §3.5 table |
| 56 | §18.2 |
| 57 | §19.2 |
| 63 | §5.3 (core genres); `[]` for the four secret genres, which never hold a secret exit (§17.4) |
| 64 | §4 |

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

`node_w` (H plant, F fungus; doc 08 harvest nodes, §11.2): MF 40 20; GH 20 50; CH 10 60; SO 10 10; CP 10 20; PA 20 20; WW 30 10; HN 20 30; WX 50 20; SP 10 20; FT 60 30; MA 40 10; ON 40 40; RD 0 20; UW 0 0; UT 34 33. Mineral (M) sources are the doc 07 ore veins of §11.2, whose kinds come from doc 07 §9.4, so `node_w` carries no M weight. The Unwritten places no harvest nodes.

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

Verb weights are Bp with 10000 neutral. `hack` is 0 where TEC ≤ 2; `pray` is 0 where MYT = 0; `cast` is 0 where `aether_axiom` is none and ARC = 0. `bleed_ok` is false for RD, UW and UT, true otherwise. `drift_bp` is the chance that a Folio of the genre bleeds once bleed is on (§14.1); the 12 core values sum to 12,000, a mean of exactly 1000 Bp, so 1 Folio in 10 bleeds (DECISIONS 54), and the Oneiric's 10000 makes every Oneiric Folio bleed.

| # | density_bp | event_bp | drift_bp | bleed_in_bp | seam_bp | shoot | melee | cast | pray | sneak | talk | hack | craft | build | trav |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| MF | 10000 | 2500 | 1000 | 3000 | 1500 | 6000 | 14000 | 15000 | 11000 | 9000 | 10000 | 0 | 9000 | 8000 | 9000 |
| GH | 9000 | 4000 | 1000 | 3000 | 1500 | 7000 | 11000 | 9000 | 15000 | 14000 | 9000 | 3000 | 8000 | 6000 | 8000 |
| CH | 8000 | 4500 | 1300 | 5000 | 1500 | 8000 | 7000 | 11000 | 12000 | 15000 | 9000 | 6000 | 9000 | 6000 | 9000 |
| SO | 11000 | 3000 | 800 | 3000 | 1500 | 15000 | 9000 | 9000 | 5000 | 7000 | 10000 | 13000 | 9000 | 9000 | 11000 |
| CP | 10500 | 3000 | 1000 | 3000 | 1500 | 13000 | 9000 | 5000 | 3000 | 13000 | 10000 | 16000 | 10000 | 7000 | 11000 |
| PA | 10000 | 3000 | 1100 | 3000 | 1500 | 12000 | 11000 | 6000 | 7000 | 10000 | 8000 | 8000 | 15000 | 15000 | 10000 |
| WW | 10000 | 3000 | 1000 | 3000 | 1500 | 15000 | 10000 | 8000 | 10000 | 8000 | 10000 | 4000 | 9000 | 8000 | 11000 |
| HN | 9000 | 3500 | 800 | 3000 | 1500 | 12000 | 9000 | 6000 | 3000 | 14000 | 16000 | 7000 | 7000 | 5000 | 9000 |
| WX | 10500 | 2500 | 1000 | 3000 | 1500 | 3000 | 16000 | 12000 | 8000 | 10000 | 9000 | 0 | 8000 | 6000 | 16000 |
| SP | 10500 | 3000 | 1000 | 3000 | 1500 | 13000 | 9000 | 7000 | 6000 | 8000 | 10000 | 9000 | 15000 | 13000 | 11000 |
| FT | 9500 | 3500 | 1000 | 4000 | 1500 | 4000 | 10000 | 16000 | 12000 | 10000 | 15000 | 0 | 10000 | 8000 | 9000 |
| MA | 11000 | 3000 | 1000 | 3000 | 1500 | 5000 | 15000 | 10000 | 16000 | 7000 | 10000 | 0 | 8000 | 7000 | 12000 |
| ON | 9500 | 5000 | 10000 | 6000 | 3000 | 8000 | 9000 | 15000 | 11000 | 9000 | 9000 | 6000 | 8000 | 10000 | 16000 |
| RD | 9000 | 4000 | 0 | 0 | 0 | 9000 | 7000 | 5000 | 8000 | 16000 | 12000 | 15000 | 8000 | 7000 | 8000 |
| UW | 6000 | 2000 | 0 | 0 | 0 | 6000 | 14000 | 0 | 0 | 12000 | 0 | 0 | 10000 | 14000 | 12000 |
| UT | 12000 | 4000 | 0 | 0 | 0 | 10000 | 10000 | 10000 | 10000 | 10000 | 10000 | 10000 | 10000 | 10000 | 10000 |

Liquids (`liquids`, `liq.<name>`, doc 08): MF water 70, oil 20, lava 10. GH water 40, blood 50, alcohol 10. CH water 60, acid 20, blood 20. SO water 30, acid 40, oil 30. CP water 40, acid 30, oil 30. PA water 30, acid 40, oil 30. WW water 50, oil 30, alcohol 20. HN water 60, alcohol 25, oil 15. WX water 80, alcohol 20. SP water 40, oil 50, lava 10. FT water 70, alcohol 20, blood 10. MA water 50, lava 30, oil 20. ON water 50, alcohol 30, acid 20. RD water 20, red_ink 60, acid 20. UW red_ink 100. UT water 40, red_ink 30, lava 30.

Story domains (§20): MF flame, oath, star, wyrm. GH blood, moon, crypt, mirror. CH deep, star, madness, tide. SO star, empire, void, machine. CP chrome, neon, data, corporation. PA rust, ash, water, radiation. WW dust, gold, hex, iron. HN rain, money, smoke, lie. WX jade, sword, river, mountain. SP steam, brass, clock, sky. FT thorn, glass, wolf, bargain. MA bronze, oracle, sea, hubris. ON sleep, mirror, flight, door. RD form, seal, silence, file. UW blank ×4. UT word ×4.

### 3.6 Example GPV record

```json
{
  "id": "genre.gothic_horror", "display": "Gothic Horror", "tier": 0,
  "arc": 5, "tec": 3, "myt": 9, "vrv": 3, "aether_axiom": "ax.mythos", "gravity_bp": 10000, "ambient_light": 2,
  "lucre_skin": "Shillings", "aether_skin": "Sanguine",
  "air_accel_bp": 10000, "fall_dmg_bp": 10000, "ground_friction_bp": 10000, "spawn_block_max": 7,
  "palette": [3809067, 7998996, 13157300],
  "music": ["organ", "string quartet", "harpsichord"], "enemy_themes": ["vampires", "werewolves", "ghouls"],
  "materials": ["mat.silver", "mat.blackwood", "mat.bone"],
  "manufacturers": ["mfr.gothic_horror.vesper_sons", "mfr.gothic_horror.grimhallow"],
  "family": 1, "era_tl": 5, "render_style": 1, "fixture_level": 12, "fog": 1, "ambient_decic": 80,
  "liquid_viscosity_bp": 9000, "gas_dissipation_bp": 8000,
  "damage_affinity": [{"dmg": "dmg.umbral", "bp": 2000}, {"dmg": "dmg.radiant", "bp": -1500}],
  "patron": "patron.gothic_horror", "hard_bp": 10000,
  "layout": "gpv.layout.manor", "fp_w": 9, "fp_d": 9, "storeys": 3, "rooms_min": 15, "rooms_max": 19,
  "loop_bp": 2000, "vert_bp": 2500, "corridor_w": 3, "cover_bp": 2500, "bright_bp": 2000, "dark_bp": 5000,
  "secret_bp": 2000, "trap_bp": 2000,
  "liquids": [{"liq": "liq.water", "w": 40}, {"liq": "liq.blood", "w": 50}, {"liq": "liq.alcohol", "w": 10}],
  "tilesets": ["biome.gothic_horror.manor", "biome.gothic_horror.catacomb"],
  "cycle_w": [15, 20, 10, 10, 5, 10, 15, 15],
  "room_bias": [{"room": "room.crypt", "bp": 20000}, {"room": "room.prison", "bp": 12000}, {"room": "room.garden", "bp": 5000}],
  "node_w": [20, 50],
  "roster": "enemy.gothic_horror",
  "factions": ["fac.native.gothic_horror.hollow_village", "fac.native.gothic_horror.inquisition", "fac.native.gothic_horror.night_court"],
  "density_bp": 9000, "verbs": [7000, 11000, 9000, 15000, 14000, 9000, 3000, 8000, 6000, 8000],
  "hazards": ["hazard.spike_pit", "hazard.curse_glyph", "hazard.blade_trap", "hazard.falling_debris", "hazard.miasma_bog", "hazard.alarm"],
  "events": ["gpv.evt.light_failure", "gpv.evt.whispers", "gpv.evt.ambush", "gpv.evt.weather_front", "gpv.evt.plea", "gpv.evt.lore_echo"],
  "event_bp": 4000, "bleed_ok": 1, "drift_bp": 1000, "bleed_in_bp": 3000, "seam_bp": 1500,
  "secret_exits": [{"genre": "genre.oneiric", "bp": 1500}],
  "laws": ["law.gothic_horror.dread_escalation", "law.gothic_horror.sanctified_ground"],
  "story_domains": ["blood", "moon", "crypt", "mirror"]
}
```

Encodings: `tier` core 0, secret 1; `family` fantasy 0, horror 1, sf 2, historical 3, crime 4, mythic 5, meta 6; `render_style` pbr 0, painterly 1, toon 2, ink 3, pixel_dither 4, xerox_collage 5; `palette` entries are the canon `#RRGGBB` values as decimal integers; booleans are 0 or 1; `hazards` and `events` take their weights from list position (§18.2, §19.2); `display`, `lucre_skin`, `aether_skin`, `music` and `enemy_themes` are the canon strings, which doc 18 localizes through `loc.genre.<suffix>.*`. The record has one key per dimension, 65 in all.

### 3.7 Lints (content load fails the build)

- **L1** `fp_w × fp_d × storeys >= 8 × (rooms_max + 10)` (minimum margin: RD `256 >= 256`).
- **L2** Block A equals Canon §5.2 and Block B equals Canon §5.9 field for field.
- **L3** the verb zero rules of §3.5.
- **L4** every listed hazard and event is eligible in its genre (§18.1 `dep` and electronic flag, §19.1 `requires`).
- **L5** `protect_r >= 4` wherever `spawn_block_max <= 14`.
- **L6** `bright_bp + dark_bp <= 10000`.
- **L7** every genre lists 1 to 3 Laws, includes every Law Canon §5.10 requires of it, and at most one Law with `changes_constant = 1`.
- **L8** every tileset prototype has weight `w >= 1`; the expanded tileset (rotations and mirrors, §10.2) has at most 256 variants; every pinned shell class of every template has at least one variant; every `fallback_fill` satisfies every face constraint; no prototype puts a custom micro-voxel mask in a cell that its `walkable` flag makes a feet, support or headroom cell (§10.4).
- **L9** `cycle_w` sums to more than 0; `node_w` sums to more than 0 in every genre except the Unwritten; `liquids` weights sum to more than 0; `secret_exits` is empty for the four secret genres.
- **L10** every room template's footprint list lies inside its kind's §16.1 `fp` range and storeys inside `st`; every kind that typing can draw has, for every footprint and storey count typing can give it, at least one template without `unlock` whose `genres` admit every genre (§10.1).


### 3.7 Climate: Time of day and weather (DECISIONS 99, doc 03 §4.8)

Each Folio rolls one time of day and one weather state at plan stage 0 on sub-stream `rng.worldgen` ("climate", 0), fixed for the Folio.
- **Time of day**: `tod.dawn`, `tod.day`, `tod.dusk`, `tod.night`. Weights per genre:
  - Default: dawn 2500, day 4000, dusk 2500, night 1000.
  - Gothic Horror, Cosmic Horror, Noir: dawn 1000, day 1000, dusk 3000, night 5000.
  - Oneiric: dawn 2500, day 2500, dusk 2500, night 2500.
- **Weather state**: `wx.clear`, `wx.rain`, `wx.storm`, `wx.fog`, `wx.snow`, `wx.dust`. Each state feeds light, sound, surfaces and fire (doc 03 §4.8) and stealth (doc 09 §15.3: fog points and ambient dB):
  - clear: fog 0, sound_ddb 0, amb 0 dB
  - rain: fog 0, sound_ddb 2, amb +5 dB, wet (st.soaked)
  - storm: fog +1, sound_ddb 4, amb +10 dB, wet
  - fog: fog +1, sound_ddb 0, amb 0 dB
  - snow: fog +1, sound_ddb 2, amb 0 dB, noise.surface.snow
  - dust: fog +1, sound_ddb 3, amb +5 dB
- `FolioPlan` and doc 02's `FolioView` expose `Climate { tod: Id32, wx: Id32 }`, passed to doc 03 `set_climate(tod, wx)` before the light bake (§10.5).

## 4. World Laws (`law.*`, Canon §5.10)

A Law is active where the local genre is its genre and in seams that include it, from Folio entry, whether or not it has been discovered (§4.1). Additive effects enter the Canon §10.5 step-1 bucket (potency 10000) when the attacker stands where the Law is active. No Law changes a canon constant (`changes_constant = 0`).

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
| `law.space_opera.hull` | structural | after stage `hazard`, while the Space Opera zones hold fewer than `floor_div(rooms_total, 4)` `hazard.hull_breach` panels, one panel goes to the first free hazard socket of the lowest-`NodeId` non-safe Space Opera room without one; no draws |
| `law.cyberpunk.netrun` | additive | electronic hazards, locks and doors here are hackable (doc 09); hacking a camera disables its room's cameras for 1800 ticks |
| `law.cyberpunk.corporate_response` | structural | each camera alarm makes doc 13's Director spawn a wave at once (ignoring its 900-tick spacing), at most 2 per Folio |
| `law.post_apocalypse.scavenge` | additive | destroyed crates and barrels here drop 1 material of the genre's Canon §5.9 pool (doc 07) |
| `law.post_apocalypse.scarcity` | structural | resource-node yields × 15000 (doc 07); `2 × floor_div(rooms_total, 4)` extra ore veins (§11.2) |
| `law.weird_west.high_noon` | binding | `law_bound` target of duel techniques (doc 05). Standoff: aiming within 1024 Bam of a hostile ≤ 20,000 mm away while both move < 100 mm for 90 ticks gives the next shot within 60 ticks +5000 Bp and a direct `st.staggered` |
| `law.hardboiled_noir.void_rules` | clock | keeps no meter: its complication checks roll a constant `dread_bp = 500`, which never escalates or resets |
| `law.hardboiled_noir.leads` | structural | solving the Folio story (§20.4) also discovers the Folio's secret exit, if one exists |
| `law.wuxia.qinggong` | binding | `law_bound` target of qinggong techniques (wall-run, glide; docs 04, 05); no other effect |
| `law.wuxia.honor_duel` | structural | during an accepted `gpv.evt.duel_challenge`, other NPCs in the duel room never target the duelist (doc 13) |
| `law.steampunk.weird_science` | grant | `law_grants: [{component_class: gadget, axis: techne, tol_bonus: 2}]` |
| `law.steampunk.clockwork` | structural | every puzzle room here uses the lever variant (§16.3) |
| `law.fairy_tale.rule_of_three` | structural | after 2 failed attempts at a lever puzzle, the correct order glows on the levers |
| `law.fairy_tale.cold_iron` | additive | Errata hits with a weapon of primary material `mat.cold_iron` on fae-tagged NPCs add +2500 Bp |
| `law.mythic_antiquity.hubris` | structural | on the tick the party's 40th kill in the Folio lands, if no Erratum has yet prayed at a `room.shrine` or `secret.room.shrine` altar in this Folio (doc 06 "made at a shrine"), the Director spawns one Nemesis (a `rank.champion` with the Folio's whole aspect set) at the anchor farthest from every Erratum; once per Folio. Doc 13 §10.1's budget yields 38 enemies in its reference Folio, so a party that clears that and the Folio's waves without praying meets the Nemesis near the end |
| `law.mythic_antiquity.favor` | additive | each Erratum's first prayer per Folio at a `room.shrine` gains +50 favor before Canon §16.5 scaling (doc 06) |
| `law.oneiric.gravity_flip` | structural | 2..=4 gravity-flip volumes (§9.5) |
| `law.oneiric.dream_doors` | structural | 1 relink door pair, period 1800 ticks |
| `law.redacted.dread_escalation` | clock | dread meter, step 250, cap 2500 |
| `law.redacted.censorship` | additive | items picked up here show a redacted name until `know.appraised` (doc 10) |
| `law.unwritten.editors_only` | structural | roster is `fac.editors` only; no shop and no Lucre containers (Canon §5.2) |
| `law.ur_text.pure_reading` | structural | records the Canon §5.5 override `C = 0`, `E = 0` |

**Dread meter** (Cypher Horror Mode, R6 §11). Folio state `dread_bp: i32`, 0 at Folio entry, one per Folio. The **governing clock Law** is the Folio genre's clock Law when it has one, else the first clock Law among the bleed zones in zone order (entry, then drift in draw order, §14.1); a Folio with none keeps no meter and makes no complication checks. On each tick `t > entry` it gains `step` once if `(t - entry) % 360 == 0` and any Active Erratum is `in_combat` (Canon §2.3), once more if `(t - entry) % 7200 == 0`, and once per Contradiction failure (jam, fizzle, spark or failed potion, Canon §5.5, §5.11: each call of doc 06's `note_contradiction_failure(t)`) by an Erratum standing where the Law is active; after all gains of the tick, `dread_bp = min(dread_bp, cap)`. When the Colophon services open (doc 14 §3.9) it resets to 0 and gains nothing more in the Folio. `step` and `cap` are the governing Law's. When the governing Law is `law.hardboiled_noir.void_rules`, the meter is never kept and `dread_bp` reads 500 at every check.

**Complication checks.** On every tick with `t - entry >= 600` and `(t - entry) % 600 == 0`, while the Colophon services are closed and at least one Active Erratum stands where any clock Law is active, the host draws on `rng.event` after the §19.3 event check of the same tick:

```rust
fn complication_check(r: &mut Rng, f: &FolioState) {
    if !r.roll_bp(f.dread_bp()) { return; }                                  // 1 draw
    let k = r.weighted(&[30, 25, 25, 20]);                                   // 1 draw: light_failure, door_slam, whispers, ambush
    let rooms = f.law_active_rooms_holding_or_adjoining_errata();            // ascending NodeId, never empty here
    let room = rooms[r.range(rooms.len() as u64) as usize];                  // 1 draw
    if k == 3 { ambush_anchors(r, f, room); }                                // 0 or 2 draws (§19.1 gpv.evt.ambush)
    fire_complication(k, room, f);
}
```

A check draws 1, 3 or 5 values. `whispers` needs MYT ≥ 1 in the room, which every clock genre has (GH 9, CH 10, HN 1, RD 8). Example: Gothic Horror with an Erratum `in_combat` on every 360-tick mark: `dread_bp` is 250 at `entry + 360`, 750 at `entry + 1080` and 1000 at `entry + 1440`; the check at `entry + 600` rolls `roll_bp(250)`, the one at `entry + 1800` rolls `roll_bp(1250)` (5 gains), and the meter reaches its cap of 2500 at `entry + 3600` (10 gains). A complication fires on average once per `600 / 0.125 = 4800` ticks at 1250 Bp, and once per 2400 ticks at the cap.

### 4.1 Discovery in play (Canon §5.8, DECISIONS 56)

A Law's effects apply from Folio entry whether or not anyone has discovered it; discovery changes only what players are shown. A Law is **discovered** for an account once it triggers while that account's player holds an Active Erratum in the run. Until then, the exit preview (§5.7), the HUD's Law list and the Folio header (doc 17) omit it, with no placeholder; from then on, the account's discovered Laws (doc 14 §5.5) remember it and every later run shows it. Canon §5.8 and DECISIONS 56 call this record the Codex; doc 14 §5.5 keeps that name for the battle pass and names the encyclopedia the Index, so this chapter says "discovered Laws" and records them in the Index.

- **Trigger record.** The run state (doc 14 `RunState`) keeps `laws_triggered: BTreeMap<Id32, u8>`, saved and hashed with the rest of run state and never cleared during the run. On the first tick a Law's trigger below holds, the host inserts the Law's ID with the bitmask of the slots that hold an Active Erratum on that tick, and doc 03 emits `ev.law_discovered` (payload `{ law: Id32, slots: u8 }`, doc 03 §14.3). Later triggers of the same Law change nothing. Triggers are evaluated only where the Law is active.
- **Who sees it.** A viewer's previews and HUD show a Law that was among the viewer's account's discovered Laws at run start or whose `laws_triggered` slot mask holds the viewer's slot. At run end, and when a player departs (doc 14 `run.end.departed`), doc 14 §5.5 adds to that player's account's discovered Laws every Law whose slot mask includes the player's slot.
- **Determinism.** Triggers read simulation state only, so `laws_triggered` is the same for every account; only the presentation differs between a fresh account and a veteran one.

| Law | Discovered on the first tick in the run that |
|---|---|
| `law.mythic_fantasy.true_names` | an Errata hit receives its +1000 Bp term |
| `law.mythic_fantasy.oathbound` | it applies `st.blessed` to an Erratum |
| `law.gothic_horror.dread_escalation` | `dread_bp` first rises above 0 with it as the governing clock Law |
| `law.gothic_horror.sanctified_ground` | an Errata hit receives its +3000 Bp term |
| `law.cosmic_horror.dread_escalation` | `dread_bp` first rises above 0 with it as the governing clock Law |
| `law.cosmic_horror.non_euclidean` | an Erratum crosses one of its relink doors while the pair is in state 1 (§9.4) |
| `law.cosmic_horror.forbidden_knowledge` | an Erratum pays its composure cost for reading a lore object |
| `law.space_opera.shipboard_systems` | an Erratum opens or hacks a door it makes electronic |
| `law.space_opera.hull` | a `hazard.hull_breach` panel breaks where it is active |
| `law.cyberpunk.netrun` | an Erratum completes a hack of an electronic hazard, lock or door where it is active (doc 09) |
| `law.cyberpunk.corporate_response` | it makes doc 13's Director spawn a wave |
| `law.post_apocalypse.scavenge` | a destroyed crate or barrel drops its material under it |
| `law.post_apocalypse.scarcity` | an Erratum breaks a resource node whose yield it multiplies |
| `law.weird_west.high_noon` | a standoff completes for an Erratum, or any entity activates a component bound to it |
| `law.hardboiled_noir.void_rules` | a complication fires with it as the governing clock Law |
| `law.hardboiled_noir.leads` | the Folio story is solved in a Folio whose genre is Hardboiled Noir |
| `law.wuxia.qinggong` | any entity activates a component bound to it in a room holding an Erratum |
| `law.wuxia.honor_duel` | an Erratum accepts a `gpv.evt.duel_challenge` where it is active |
| `law.steampunk.weird_science` | an Erratum activates a gadget (doc 07) where it is active |
| `law.steampunk.clockwork` | an Erratum pulls a puzzle lever where it is active |
| `law.fairy_tale.rule_of_three` | the correct lever order glows |
| `law.fairy_tale.cold_iron` | an Errata hit receives its +2500 Bp term |
| `law.mythic_antiquity.hubris` | the Nemesis spawns |
| `law.mythic_antiquity.favor` | an Erratum gains its +50 favor |
| `law.oneiric.gravity_flip` | an Erratum's origin stands in a cell carrying the `flip` bit |
| `law.oneiric.dream_doors` | an Erratum crosses one of its relink doors while the pair is in state 1 (§9.4) |
| `law.redacted.dread_escalation` | `dread_bp` first rises above 0 with it as the governing clock Law |
| `law.redacted.censorship` | an Erratum picks up an item whose name it redacts |
| `law.unwritten.editors_only` | a Folio of `genre.unwritten` is entered (`ev.folio_entered`) |
| `law.ur_text.pure_reading` | a Folio of `genre.ur_text` is entered (`ev.folio_entered`) |

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
6. **Effective weight** (Canon §17.1's "lattice edge weights") `w(g) = edge(cur, g) × fresh_bp(g)` (`u32`, at most 1,500,000); for a secret `cur`, `edge` reads the return weights; for the Margin node it is 100.
7. **Forced:** the Folio at depth 25 (Standard 25, Abridged 15) is `genre.ur_text` (doc 14 §3.6 item 2).

**Feasibility.** Choosing Folio `m <= 24` in Standard, rules 2 to 4 exclude at most 9 of the 12 core genres: `c` capped genres use `3c` of the `m - 1` earlier slots, and only the other `m - 1 - 3c` slots can hold an uncapped same-Volume genre (or, at position 1, the uncapped current genre). The maximum of `c + min(pos - 1, m - 1 - 3c) + [pos = 1 and m - 1 - 3c >= 1]` over `c` is 9, reached at `m = 20` (`c = 5` uses 15 of the 19 earlier slots, and the 4 earlier Folios of Volume 4 are the 4 uncapped ones: `5 + 4`) and at `m = 24` (`c = 6` with 3 uncapped same-Volume genres, or `c = 7` with 2). Abridged excludes at most 5 (at `m = 12` and `m = 14`), and Marginalia at most 4 (same block only, no cap). So at least 3 genres always stay eligible; the implementation asserts `eligible.len() >= 3`, and acceptance test 3 checks the bound by exhaustive enumeration.

### 5.5 Exit-offer algorithm

The draw happens on the `InColophon → ExitVote` (or `→ VictoryChoice`) tick, the first lectern interaction (doc 14 §3.2, §3.6), when the exit UI opens (Canon §17.1); exactly 3 candidates every time, even before forced Folios. The function reads only its arguments, so the host and every client compute the same doors.

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

- Unseeded runs (Standard, Abridged and every other mode without a typed or Canon §3.6 seed; Canon §17.1 `FOLIO_1_GENRE`): the host Erratum's Origin genre when core. A secret Origin maps to its highest return-weight core genre, ties to lattice order: Oneiric → Fairy Tale, Redaction → Cyberpunk, Unwritten and Ur-Text → Mythic Fantasy.
- Seeded runs (Daily, Weekly and any run with a typed seed): the first candidate of `lattice_candidates(run_seed, 0, lattice.node.margin, [], mode)`, a uniform draw. For `0x0123456789ABCDEF` the candidates are MF, HN, SP (Folio 1 is Mythic Fantasy); for the Daily seed of 2026-10-11 (`0xc6f2e12ddca712ab`) they are WW, CH, MF.

### 5.7 Exit preview

Each door shows doc 14 §3.6 item 4 (name, palette, axioms, position, Contradiction forecast) plus the fields below (doc 17 §9.1 draws them). The plan fields are **pure plan functions** of the candidate Folio, which recompute seeds without advancing live streams and match Folio `n + 1`'s generation exactly (acceptance test 9). A door hidden by `run.mut.blind_exits` (doc 14 §8.7) shows none of them until it commits.

| Field | Source |
|---|---|
| champion aspects with their counter icons | doc 13 §7.3 aspect set, recomputed from a temporary copy of Folio `n + 1`'s `rng.spawn` (Canon §17.3a) |
| Folio type and challenge kind | `folio_type()` (§6.1) |
| bleed: entry genre, drift genres, seam genre | `bleed_plan()` (§14.1) |
| Marginalia modifiers | `marg_mods()` (§22) |
| the candidate genre's Laws that the viewer has discovered (§4.1); undiscovered Laws are omitted, with no placeholder | §4 |
| two hazard icons | the two highest-weight hazards of the candidate GPV, ties to list order |
| Lucre and Aether skins | GPV dimensions 10–11 of the candidate genre |
| depth and position | doc 14 §3.1 `d(n + 1)` and `pos(n + 1)`; a boss crown when the type is boss, and "Binding" for a pos-4 Folio's doors |
| secret door | a discovered secret exit door (§17.4) shows "?" and only its genre's palette |

The champion aspects are recomputed for every door, because the aspect draw reads the candidate genre's axioms (doc 13 §7.3); the stream seed does not depend on the genre. The preview fields cost at most `3 × (2 + 5 + 4 + 3) = 42` draws on temporary generators per exit opening (at most 3 lattice doors; type 1–2, bleed 0–5, Marginalia `k <= 4`, aspects `k <= 3`), all discarded.


### 5.8 Second Edition fused lattice (DECISIONS 100, 101, 105, 106, 117, 118)

Second Edition is 15 Folios in 3 Volumes of fused genres (Volumes 6, 7, 8; depth `d = 25 + 2n` for Folio `n = 1..=15`).
- **Fused pairs**: each Folio is written in an ordered pair of core genres `(genre_A, genre_B)`. `FolioRecord.genre` records the genre of lower Canon §5.2 row and `fused` the other (DECISIONS 101, doc 14 §8.9).
- **Lattice rule**: the Volume no-repeat rule applies per fused pair (DECISIONS 117). A Volume never repeats the same pair, but an individual genre may appear in two different pairs within a Volume.
- **Axioms**: local axioms outside bleed and seam zones are `floor_div(axiom_A + axiom_B, 2)` per axis (Canon §5.8). Both genres' rosters, World Laws and skins are active, and entities of either genre count as native.
- **Bosses**: Volumes 6 and 7 end on revised Editors (`n = 5` and `n = 10`), Volume 8 on `boss.ur_editor_rev` followed by `boss.author` at `n = 15` (doc 13 §9.4).
- **Exempt draw**: the draw immediately before the forced Ur-Text Folio is exempt from Volume and run-cap rules (DECISIONS 115).

## 6. Folio types

### 6.1 Assignment

`lattice.folio.boss` is doc 14's Boss; `lattice.folio.standard`, `lattice.folio.challenge` and `lattice.folio.rest` are variants of doc 14's Normal (doc 14 §3.1). Standard and challenge Folios keep doc 14's Rubric, Keeper, marginal notes and Colophon; a rest Folio has no Rubric, no Keeper, no marginal notes and no Interleaf, and its Colophon entrance is open from entry (doc 14 §3.1, §3.2).

```rust
fn folio_type(run_seed: u64, n: u32, genre: GenreId, mode: Mode) -> (FolioType, Option<ChallengeKind>) {
    let pos = doc14::pos(n, mode);                            // Standard, Novella, Daily, Endless: n - 5 × (v - 1); Abridged [1, 4, 5]
    if pos == 5 { return (Boss, None); }                      // boss Folios 5, 10, …; Abridged 3, 6, …; Marginalia n % 5 == 0
    if n == 1 || genre == UNWRITTEN { return (Standard, None); }
    let mut r = stage_rng(run_seed, n, genre, "ftype", 0);
    let w: [u32; 3] = if n >= 26 { [7000, 2000, 1000] }      // [standard, challenge, rest]
                      else if pos == 4 { [8500, 1500, 0] }    // the Interleaf is the pre-boss rest
                      else { [8000, 1500, 500] };
    let mut t = [Standard, Challenge, Rest][r.weighted(&w)];  // 1 weighted call
    if genre == REDACTED && t == Rest { t = Standard; }      // no light makes a rest safe there (spawn_block_max 15)
    let ch = if t == Challenge {                              // 1 more weighted call, §6.3 order
        Some([Gauntlet, Siege, Heist, Duel, Race][r.weighted(&[25, 20, 20, 15, 20])])
    } else { None };
    (t, ch)
}
```

The function draws 0, 1 or 2 values. Example (§5.5 run, `range(10000)` values in brackets): at Folio 2 Space Opera is standard (3860), Hardboiled Noir rest (9855), Steampunk rest (9596) and Mythic Fantasy standard (7853); at Folio 3 Cyberpunk is rest (9633) and Mythic Fantasy a siege challenge (8993); at Folio 4 (pos 4) Cyberpunk is a gauntlet challenge (9480), Mythic Fantasy (774) and Gothic Horror (4365) standard. Over a Standard run, the 14 Folios at positions 1 to 3 among Folios 2 to 24 expect `14 × 0.05 = 0.7` rest Folios, and the 19 non-boss Folios among 2 to 24 expect `19 × 0.15 = 2.85` challenges.

### 6.2 Composition

**Rubric.** Stage `rubric` draws one `weighted([5000, 3000, 2000])` over keeper, catchwords, seal (doc 14 §3.4) for standard and challenge Folios and nothing for rest and boss Folios. At Folio 2 of §5.5, Space Opera draws catchwords (5080) and Mythic Fantasy keeper (1931); Hardboiled Noir and Steampunk are rest Folios and draw no Rubric.

`folio_scale_bp` (doc 14 §8.1: Standard 10000, Abridged 9000, Novella and Daily 8000) scales the drawn node count of standard and challenge Folios: `n_target = max(4, mul_bp(base, folio_scale_bp))`, `base` being the first draw of stage `graph`.

| Type | Mission graph `base` | Fixed rooms (rules) | Population (doc 13 §10) |
|---|---|---|---|
| standard | `range_incl(rooms_min, rooms_max)` | Incipit, Colophon, one `room.shop` (none in the Unwritten); Interleaf at pos 4 | `director.profile.standard`: full budget |
| challenge | `range_incl(10, 14)` | Incipit, Colophon, one `room.shop` (none in the Unwritten), the challenge structure (§6.3); Interleaf at pos 4 | full budget |
| rest | none: `rest_ring` builds 5 fixed nodes plus `j = range_incl(1, 3)` Rooms, 6 to 8 in all, unscaled | Incipit, `room.campfire` hub, `room.shop`, `room.fortune_hall`, Colophon | `director.profile.rest`: no initial population, no ambient or scripted waves; every room has `spawn_slots = 0`, while spawn cells and anchors of non-safe rooms stay for Erasure hunters and invaders |
| boss | none: fixed spine (§8.2) | Incipit antechamber, Editor checkpoint, boss arena, Colophon | `director.profile.boss`: doc 13 §10.1 checkpoint squad |

The `room.shop` of a standard or challenge Folio has 2 vendor sockets and 2 Fortune Engine sockets: doc 11 §12.1 puts vendor [1] in socket 0 and, when its roll succeeds, vendor [2] in socket 1 (an unused socket holds an empty stall prop), and doc 11 §11.1 puts the brass Engine in engine socket 0 and the silver Engine or its variant in socket 1 when placed. The rest Folio's `room.shop` uses vendor socket 0 only, and its `room.fortune_hall` holds the brass and silver Engines. The Interleaf's vendor is doc 11's `shop.general` [3]. Boss Folios hold only the Colophon shop.

`type_required_nodes(plan)` is the number of nodes the D phase of §8.2 adds by rule, which growth (§8.3) leaves room for: `[pos == 4] + [genre != unwritten and not lattice.marg.sealed]` (Interleaf, shop) for standard and challenge Folios, plus, for challenge Folios, the structure's nodes (gauntlet 3, heist 2, siege, duel and race 0).

### 6.3 Challenge kinds

Drawn in stage `ftype` right after a challenge result (§6.1). Success is judged at exit commit and emits `ev.objective_completed` with kind 1 (doc 03 §14.3); each success gives every Active Erratum one in-run Reliquary roll (doc 11 §9.12). A challenge never replaces the Rubric; both run.

| ID | w | Structure (rule `room.rule.challenge`, §8.2) | Success |
|---|---|---|---|
| `lattice.challenge.gauntlet` | 25 | 3 `room.gauntlet` nodes, inserted one at a time by splitting the non-RubricGate main-path edge whose start fraction is nearest 2500, 5000 and then 7500 Bp (ties to the lower index), recomputing the main path after each split | no `ev.player_downed` and no Erratum `ev.entity_killed` in the Folio |
| `lattice.challenge.siege` | 20 | `room.rule.arena` always fires; the Arena seals for 7200 ticks when the first Erratum enters it; doc 13 sends 8 waves of 1500 mSKE, one every 900 ticks (doc 13 §10.4) | every siege-wave member is dead on the tick the seal ends |
| `lattice.challenge.heist` | 20 | a `Treasury` attached to the main-path Room nearest fraction 6000 Bp by a `Locked(k)` edge, its `Key` on a dead end branching from the main-path Room nearest 3000 Bp; this is the Folio's one lock (§8.3) | the Treasury door opens while no hostile has ever held `aware.alert` toward any Erratum in the Folio |
| `lattice.challenge.duel` | 15 | a champion carrying the Folio's whole aspect set waits in the main-path Room adjacent to the Colophon (doc 13 §10.2 step 4) | the champion is dead |
| `lattice.challenge.race` | 20 | none | an exit commits within 10800 ticks of entry |

## 7. Generation pipeline

### 7.1 Inputs and outputs

`generate_folio(input, db, par) -> FolioBlueprint` (doc 02) is pure; doc 14 §3.3 calls it at each Folio entry.

```rust
pub struct GenInput {
    pub run_seed: u64,
    pub folio_index: u32, pub d: u32, pub pos: u8, pub mode: Id32,   // doc 14 §3.1, §8; d >= 1 (Canon §13.8)
    pub genre: Id32, pub prev_genre: Option<Id32>,
    pub folio_scale_bp: i32, pub ambush_extra: u8,  // doc 14 §8.1, gp.ambush_extra
    pub bleed_mult: u8, pub bleed_add: u8,          // run.mut.wild_bleed: 2 (else 1); run.marg.bleeding_genres: its rank r (else 0)
    pub bleed_on: bool,                             // §14.1 newcomer gate, fixed at Commit in the run header (doc 14)
}
pub struct FolioBlueprint {
    pub header: Header,               // sim_version, content_hash, every GenInput field, attempt, blueprint_hash
    pub plan: FolioPlan,              // ftype, challenge, rubric, bleed plan, marg_mods
    pub zones: SmallVec<[ZoneRec; 6]>, pub zone_of_cell: [u8; 1536],   // 16 × 6 × 16 cells
    pub graph: MissionGraph, pub rooms: Vec<RoomRec>, pub doors: Vec<DoorRec>,
    pub chunks: Vec<ChunkBlob>,       // 128 chunks: palette_rle blocks with shapes, custom-mask table (§10.4), open_tag, protect, flip bits (doc 03 §4.7)
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
| 0 | `ftype`, `rubric`, `bleed`, `marg` | §6, §14.1, §22 | 0–2; 0–1; 0–5; `k <= 4` | < 0.1 |
| 1 | `graph` | mission graph (§8) | per rule | 1 / 3 |
| 2 | `typing` | kinds, footprints (§8.4) | 1–3 per node | 0.5 / 1 |
| 3 | `zones` | bleed and seam rooms (§14.2) | 1 + cluster | < 0.5 |
| 4 | `embed` | cells, doors, corridors, relinks, flips (§9) | 1–2 per node | 4 / 12 |
| 5 | `wfc` per room | template choice, room fill (§10) | 0–1 for the template, then 1 per observation | 45 / 110 |
| 6 | `liquid`, `node`, `hazard` | §11.2–§11.4 | per placement | 3 / 6 |
| 7 | `dress`, `light` per room | §11.1, §11.3 | 1 shuffle each | 15 / 35 |
| 8 | `spawn`, `loot`, `secret`, `story` | §12, §13, §17, §20 | per record | 2 / 4 |
| 9 | rasterize (`par.map`, 128 chunks) | blocks, bits, blending (§14.4) | 0 | 20 / 45 |
| 10 | validate and repair | §15 | 0 | 15 / 40 |
| — | one attempt | | | **≤ 110 / 260** |
| — | worst case: attempts 0 to 3 (§15.3) and the authored fallback | | | ≤ 900 (doc 02 `perf.deck.gen.folio_max_ms`); the nightly sweep fails above it |

Stages run in table order; stage `k` reads only the outputs of stages before it, and the per-room stages 5 and 7 and the per-chunk stage 9 run through `par.map` in ascending index (doc 02 `OrderedMap`). Timing never changes output: the 900 ms figure is a CI performance gate, not a runtime cut-off. The host calls `generate_folio` on the exit-commit tick (doc 14 §3.6) and each client on receiving the commit; the page-turn transition (doc 17) lasts until the blueprint exists and its `blueprint_hash` matches the host's (doc 16).

## 8. Mission graph grammar

### 8.1 Node and edge kinds

Nodes (`NodeId` = creation index): `Incipit`, `Colophon`, `Interleaf`, `Room`, `Hub`, `Key(k)`, `Service(kind)`, `Arena`, `Gauntlet`, `Treasury`, `Puzzle(p)`, `Vault`, `Campfire`, `Checkpoint`, `BossArena`, `Secret`, `SecretExit(genre)`. Edges: `Open`; `Locked(keys)` (1 or 2 keys); `OneWay(from, to)` (a horizontal door barred on the `to` side, which the `from` side unbars, §9.2); `Hidden` (§17.3); `PuzzleGate(p)`; `RubricGate` (the single edge into the Colophon, sealed by `protect` bits until the Rubric completes, doc 14 §3.4). The **main path** is the shortest Incipit→Colophon path by edge count over every non-Hidden edge (a OneWay edge only from `from` to `to`; Locked and PuzzleGate edges count as passable), breadth-first, neighbors in ascending `NodeId`; it is recomputed after every rule that changes the graph. Its `len` is its edge count, its node `i` (0 = `in`) has **fraction** `floor_div(i × 10000, len)`, and its last edge is always the RubricGate.

### 8.2 Rules

`in` = Incipit, `out` = Colophon. A phase-B rule replaces the axiom's middle Room and both its edges; the single edge into `out` that each rewrite writes is the `RubricGate`, so `out` always has degree 1. Lower-case letters are new `Room` nodes, created in the order written. Every rule's candidate list is sorted (edges by `(min NodeId, max NodeId)`, nodes by `NodeId`, pairs lexicographically) before its `range` draw, and a `range` is drawn only when its list is non-empty.

| ID | Phase | Rewrite | Selection |
|---|---|---|---|
| `room.rule.axiom` | A | `in –Open– Room –RubricGate– out` | none |
| `room.rule.cycle_lock_key` | B | middle Room → `in–a`, `a–K1`, `K1→in` OneWay, `in–m` Locked(k1), `m–out` | `cycle_w` |
| `room.rule.cycle_hidden_shortcut` | B | `in–a–b–c–out`; `in–c` Hidden | `cycle_w` |
| `room.rule.cycle_gauntlet_return` | B | `in–Gauntlet–Treasury`, `Treasury→in` OneWay; `in–a–out` | `cycle_w` |
| `room.rule.cycle_two_key_hub` | B | `in–Hub`; `Hub–a–K1`, `K1→Hub` OneWay; `Hub–b–K2`, `K2→Hub` OneWay; `Hub–c` Locked(k1, k2); `c–out` | `cycle_w` |
| `room.rule.cycle_flooded_detour` | B | `in–w–x` with `w` flooded (§11.2); `in–a–b–x`; `x–out` | `cycle_w` |
| `room.rule.cycle_collapsing_bridge` | B | `in–c`, `c→x` OneWay (the bridge collapses after the first crossing), `x–out`; `in–a–b–x` | `cycle_w` |
| `room.rule.cycle_patrol_loop` | B | `in–a–b–x`; `in–c–d–x`; `x–out`; a patrol route on the loop (§12) | `cycle_w` |
| `room.rule.cycle_foreshadowed_vault` | B | `in–w–out`; `w–Treasury` Locked(k1) with a window tile on the shared face; `in–a–K1`, `K1→w` OneWay | `cycle_w` |
| `room.rule.extend` | C, w 50 | split an `Open` edge `u–v` (never the RubricGate) into `u–n–v` | `range` over Open edges |
| `room.rule.branch` | C, w 30 | dead end `u–n` on a Room or Hub with degree < 4 | `range` over those nodes |
| `room.rule.minor_cycle` | C, w 20 while loops remain | `u–n–v` for Rooms `u < v`, both of degree < 4, hop distance 2..=4 | `range` over those pairs |
| `room.rule.lock_key` | D1 | applied while the graph holds fewer `Locked` edges than the lock target (§8.3): an Open main-path edge becomes `Locked(k)`; its key goes on a Room dead end reachable from `in` without crossing any Locked, PuzzleGate or OneWay edge (that Room becomes a `Key`), else on a new `Key` branch of a reachable Room or Hub of degree < 4, else of `in` | 2 `range`: the edge among Open main-path edges other than the first, then the key spot among the dead ends, or among the branch anchors when no dead end exists |
| `room.rule.interleaf` | D2 | pos 4, standard and challenge: split main-path edge `min(floor_div(len × 5000, 10000), len - 2)` (0-based, `len` edges, the RubricGate last) with an `Interleaf` | none |
| `room.rule.service` | D3 | standard and challenge: attach the `room.shop` Service as a dead end of main-path node `clamp(floor_div(len × 5000, 10000), 1, len - 1)`; when the Oneiric secret exit is planned (§17.4) and the graph has no Campfire, also attach a `Campfire` dead end at main-path node `clamp(floor_div(len × 3000, 10000), 1, len - 1)` | none |
| `room.rule.arena` | D4 | standard and challenge: the main-path Room whose index fraction `floor_div(i × 10000, len)` is nearest 6000 (ties to the lower index) becomes an `Arena` (mini-boss room) | 1 `roll_bp(3000)`, always drawn; a siege challenge converts regardless of the roll |
| `room.rule.puzzle_gate` | D5 | an Open main-path edge other than the first and the last becomes `PuzzleGate(p)`; `Puzzle(p)` is a dead end of its `in`-side node | `roll_bp(2500)`, then `range` on success |
| `room.rule.vault` | D6 | attach a `Vault` (Reliquary room) as a dead end of a non-main-path Room of degree < 4, else of a main-path Room of degree < 4 | `roll_bp(5000)`, then `range` on success |
| `room.rule.gauntlet_branch` | D7 | attach `Gauntlet–Treasury` as a branch of a Room of degree < 4; skipped when the graph already holds a Treasury or the challenge is a gauntlet | `roll_bp(2000)`, always drawn, then `range` on success |
| `room.rule.rescue` | D8 | standard and challenge: the main-path Room nearest fraction 7000 (ties to the lower index) hosts doc 13's stranded native (flag `rescue`) | `roll_bp(2000)` |
| `room.rule.challenge` | D9 | the challenge kind's structure (§6.3) | none |
| `room.rule.secret` | D10 | each of `n_secrets` (§17.1) `Secret` nodes joins by a `Hidden` edge a non-secret node other than `in` and `out` of degree < 4 (any non-secret node other than `in` and `out` when none has degree < 4) | `range` per secret |
| `room.rule.secret_exit` | D11 | when §17.4 planned a secret exit to genre `g`: `SecretExit(g)` by a `Hidden` edge to the non-secret node farthest in hops from `in` (not `out`; ties to the lower `NodeId`) | none |
| `room.rule.boss_spine` | boss | `in–Checkpoint–BossArena–out` | none |
| `room.rule.rest_ring` | rest | `in–Campfire`; spokes `Campfire–Service(shop)`, `Campfire–Service(fortune_hall)`, `Campfire–r_1` … `Campfire–r_j` with `j = range_incl(1, 3)`, and `Campfire–out` (the RubricGate, unsealed from entry) | 1 `range` |

### 8.3 Algorithm

Stage `graph` uses `stage_rng(run, n, genre, "graph", attempt)` and draws in exactly this order:

```rust
fn build_graph(r: &mut Rng, g: &Gpv, plan: &FolioPlan, scale_bp: i32, attempt: u8) -> MissionGraph {
    match plan.ftype {
        Boss => { let mut G = boss_spine(); features_d10_d11(&mut G, r, plan, 1); return G; }   // exactly 1 secret
        Rest => { let mut G = rest_ring(r); features_d10_d11(&mut G, r, plan, 1); return G; }  // 1 range, then 1 secret
        _ => {}
    }
    let (lo, hi) = if plan.ftype == Challenge { (10, 14) } else { (g.rooms_min, g.rooms_max) };
    let base = if attempt == 3 { lo } else { r.range_incl(lo, hi) };  // 1 draw (none at the forced attempt 3, §15.3)
    let n_target = max(4, mul_bp(base, scale_bp));
    let n_secrets = secret_count(r, g, n_target, plan);                // 1 roll_bp (§17.1)
    let mut G = axiom();
    let cycle = if attempt == 3 { HIDDEN_SHORTCUT } else { CYCLES[r.weighted(&g.cycle_w)] }; // 1 weighted call (0 at attempt 3)
    apply_cycle(&mut G, cycle);
    let loops_max = if attempt == 3 { 0 } else { floor_div(n_target * g.loop_bp, 10000) };
    let n_grow = n_target.saturating_sub(type_required_nodes(plan));
    let mut loops = 0;
    while G.non_secret_count() < n_grow {
        let mut rule = r.weighted(&[50, 30, if loops < loops_max { 20 } else { 0 }]);   // 1 call
        if G.targets(rule).is_empty() { rule = EXTEND; }               // extend always has an Open edge
        G.apply(rule, r.range(G.targets(rule).len() as u64));          // 1 call
        if rule == MINOR_CYCLE { loops += 1; }
    }
    features_d1_to_d9(&mut G, r, plan, lock_target(plan, n_target));   // §8.2 order and draws
    features_d10_d11(&mut G, r, plan, n_secrets);
    G
}
fn lock_target(plan: &FolioPlan, n_target: u32) -> u32 {
    match (plan.ftype, plan.challenge) {
        (Standard, _) => 1 + (n_target >= 18) as u32,
        (Challenge, Some(Heist)) => 0,                                 // the heist's own lock (§6.3) is the one lock
        (Challenge, _) => 1,
        _ => 0,
    }
}
```

- D1 counts every `Locked` edge already in the graph, the major cycle's included, so a `cycle_lock_key`, `cycle_two_key_hub` or `cycle_foreshadowed_vault` Folio adds at most one more lock. When D1 finds no candidate edge (an Open main-path edge other than the first), it first splits the first main-path edge with a new Room (no draw) and then draws.
- Growth stops at `n_grow` non-secret nodes, or at once when the axiom plus the major cycle already reach it (the largest cycle, `two_key_hub`, has 8 nodes). D1 to D9 then add the `type_required_nodes` (Interleaf, shop, challenge structure) and at most 8 optional nodes: 2 key branches, D1's split, a Puzzle, a Vault, a Gauntlet and a Treasury, and an Oneiric Campfire. So a finished standard or challenge graph has between `n_target` and `max(n_target, 8 + type_required_nodes) + 8` non-secret nodes, which L1's margin of 10 rooms per footprint covers; `rooms_total` is that count.
- Example: Mythic Fantasy, pos 2, `base = 18`, `folio_scale_bp = 10000`: `n_target = 18`, `n_secrets = 2 + [roll_bp(7000)]`, `loops_max = floor_div(18 × 1500, 10000) = 2`, `n_grow = 18 - 1 = 17` (the shop), growth stops at 17 non-secret nodes, D3 adds the shop (18), and D1 adds locks until 2 exist, since `18 >= 18`. In Abridged (`folio_scale_bp = 9000`) the same draw gives `n_target = mul_bp(18, 9000) = 16` and one lock.

### 8.4 Typing

Stage `typing` (`stage_rng(…, "typing", attempt)`) visits nodes in ascending `NodeId`.

1. **Kind.** Feature nodes map directly: `Incipit` → `room.incipit`, `Colophon` → `room.colophon`, `Interleaf` → `room.interleaf`, `Key` → `room.key_room`, `Hub` → `room.crossroads`, `Service(shop)` → `room.shop`, `Service(fortune_hall)` → `room.fortune_hall`, `Campfire` → `room.campfire`, `Arena` → `room.arena`, `Gauntlet` → `room.gauntlet`, `Treasury` → `room.treasury`, `Puzzle` → `room.puzzle`, `Vault` → `room.reliquary_vault`, `Checkpoint` → `room.editor_checkpoint`, `BossArena` → `room.boss_arena`, the flooded detour's `w` → `room.cistern`, the collapsing bridge's `c` → `room.chasm`; `Secret` and `SecretExit` nodes are 1 × 1-cell, 1-storey boxes typed in stage `secret` (§17). Every other `Room` draws its kind with one `weighted` call over the §16.1 kinds in table order with the §16.2 weights, a kind at its per-Folio `max` or absent from the run's content set weighing 0.
2. **Footprint.** One `weighted` call over `fp_weights` (§3.4) restricted to the footprints in the kind's `fp` range, in the order `[1×1, 1×2, 2×2, 2×3, 3×3]`; when the restricted weights sum to 0, each footprint in range weighs 1. A kind with a single footprint still makes the call. Orientation of non-square footprints is chosen in embedding (§9.1).
3. **Storeys.** Kinds with `st` 1 take 1 storey and kinds with `st` 2 take 2, without a draw; a `1–2` kind draws `roll_bp(two_storey_bp)` when the genre has `storeys >= 2`, else takes 1 storey without a draw.
4. **Open tag.** Every room that is not safe, not secret and not `room.boss_arena` draws `roll_bp(open_bp)`; on success its cells get doc 03's `open_tag` (sky-lit at the zone genre's `ambient_light`, no roof).

A typed Room therefore draws 3 or 4 values, a feature node 1 to 3. Example: with no unlocks and no kind at its maximum, a Cyberpunk Room's kind draw sees server room weight 128 (§16.2) out of 1,819 for the 25 always-available kinds (hall 270, guardpost 260, gallery 156, …, crypt 0), so 703 Rooms in 10,000 (`floor_div(128 × 10000, 1819)`) become server rooms.

## 9. Spatial embedding and corridors

### 9.1 Placement

Stage `embed` uses `stage_rng(…, "embed", attempt)`; Secret and SecretExit nodes embed like any node, as 1 × 1 × 1 boxes.

1. **Incipit:** storey `min(start_storey, storeys - 1)`, centered on the south edge: `cx = x0/16 + floor_div(W - fw, 2)`, `cz = z0/16 + D - fd`.
2. **Order:** breadth-first from the Incipit, neighbors in ascending `NodeId`; a node's parent is its first placed neighbor.
3. **Candidates** for `v` with parent `u`, sorted by `(cs, cz, cx, orientation)`: `H` = free boxes (either orientation) on `u`'s storeys sharing a full cell face with `u`; `V` = free boxes directly above or below `u` overlapping its projection; `C` = free boxes on `u`'s base storey at Chebyshev distance 2..=4 with an A* corridor route (§9.3).
4. **Choice:** the Colophon takes the `H ∪ V` (else `C`) candidate maximizing doubled-center `|Δcx| + |Δcz| + 2|Δcs|` from the Incipit, ties to the earlier index, no draw. Others: with `H` and `V` both non-empty, `V` if `roll_bp(vert_bp)` else `H`, then `range(len)`; with one non-empty, `range(len)` in it; else `range(len)` in `C`; `C` empty fails the attempt (§15.3).
5. **Non-tree edges:** a door when boxes share a face, else a corridor; no route fails the attempt.
6. **Arena boxes** (`room.arena` 3 × 3 × 2, `room.boss_arena` per §21.2) may use storeys up to `min(5, start_storey + 2)` even where the genre's `storeys` is smaller; every other box stays inside `fp_w × fp_d × storeys`.

Example: Mythic Fantasy (`W = D = 10`, `x0 = z0 = 16 × floor_div(16 - 10, 2) = 48`) puts a 1 × 1 Incipit at `cx = 3 + floor_div(10 - 1, 2) = 7`, `cz = 3 + 10 - 1 = 12`, storey 0: blocks `x ∈ 112..128`, `z ∈ 192..208`, the southernmost row of the 160 × 160-block footprint `48..208`.

### 9.2 Doors, connectors and protect bits

| Edge | Realization |
|---|---|
| Open | horizontal: doorway 3 wide × 3 tall, genre door skin; vertical: a 3 × 3 floor opening with a slab stair run (500 mm rises) along the lower room's wall |
| Locked(keys) | lock door (doc 09 `lock.*`) bound to its key items; destructible like every door (Canon §4.3) with doc 07 block HP by lock tier |
| OneWay | always horizontal: a door with a bar on the `to` side. An Erratum on the `from` side unbars it with a 30-tick `input.interact`, after which it is an Open door for the rest of the Folio; from the `to` side it is a locked door without a lock (no key, no lockpick). Like every door it is `destructible` (Canon §4.3), so breaking it from the `to` side also opens it; validation (§15) ignores that and treats the edge as one-way. Vertical one-way drops are not used: a VERVE-10 Erratum with an air jump, a wall jump and a mantle reaches `2000 + 1200 + 1600 + 1600 = 6400` mm (Canon §4.2, doc 03 §5.7, §5.9), so no fixed drop height is both harmless at VERVE 0 (`SAFE_FALL` 3200 mm) and unclimbable at VERVE 10 |
| Hidden | a §17.3 hiding method |
| PuzzleGate(p) | a gate that opens when puzzle `p` resolves (§16.3) |
| RubricGate | the Colophon entrance; its cells carry `protect` until the Rubric completes (doc 14 §3.4) |

`protect` bits (doc 03 §4.7) are also set on exit-door volumes, on the outermost block layer of the footprint (the Folio-boundary shell) and on boss-arena doors while sealed (doc 13 §9.1). Every worldgen block is indestructible unless its block record carries `block.tag.destructible` (Canon §4.3, doc 07). Worldgen places destructible blocks only as doors, crates, barrels, cracked walls hiding secrets, wooden structures, glass, ice, webs, light fixtures, falling-debris rubble, pit lids, hull panels and resource-node blocks (doc 07 §9.3: a broken node turns into its unbreakable spent block, so no wall opens); no structural wall or floor is destructible, and fixed terrain stops every projectile (Canon §4.3).

### 9.3 Corridors and stairs

- Route: A* over free footprint cells between the boxes' facing sides; costs 10 per horizontal step, 30 per storey change (a stair cell), +2 per turn; ties toward the lower cell index; corridors never enter room cells.
- Carving: width `corridor_w`, clear height 4 blocks (4000 mm). A stair cell climbs 8 blocks over 16 with alternating slab and full steps, 500 mm per step, under the 550 mm step-up (Canon §4.1).
- Corridors take the zone of their lower-`NodeId` endpoint (§14.2).

### 9.4 Relink pairs

Cosmic Horror (2 pairs, period 3600 ticks) and Oneiric (1 pair, period 1800): horizontal Open doors off the main path and outside safe rooms, chosen by `range` without replacement (1 call per door). State 0 keeps destinations; state 1 swaps each pair's destinations through portal planes (doc 03 rotates yaw by the doors' difference). State toggles at `entry + k × period`; validation covers both.

### 9.5 Gravity-flip volumes (Oneiric)

`k = range_incl(2, 4)` single-storey Rooms are chosen by `range` without replacement (1 + k draws) among Rooms that are not among the first or last two main-path nodes, not safe, not in a bleed or seam zone and with no neighbor in one, in ascending `NodeId`; with fewer candidates than `k`, all candidates flip. A flip room gets clear height 4 blocks, doors 3 wide and 4 tall (so a body walking on the ceiling passes), a vertically mirrored WFC fill and the doc 03 §4.7 `flip` bit on every cell of its interior, so bodies there use `gsign = +1`. The longest fall a flip room allows is its 4000 mm clear height, either inside it or leaving it upside down into a normal room, which stays under Oneiric `SAFE_FALL = mul_bp(4000, 10000 + (9 - 5) × 400) = 4640` mm (Canon §4.2), so a flip never hurts; Oneiric's `fall_dmg_bp` 2500 would quarter it anyway.

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
    {"kind": "node", "tile": [6, 0, 6], "block": [0, 0, 1], "axis": "x"}
  ],
  "fallback_fill": "room.tpl.library_a.fill0", "tags": []
}
```

Shell classes pin every boundary tile (wall, corner, door, window, floor-only for open rooms). `fallback_fill` is an authored valid tile assignment.

**Template choice.** In stage `wfc`, before its first observation, each room draws its template with one `range` over its candidates in ascending ID: the templates of its kind whose `footprints` hold its footprint (either orientation) and whose `storeys` hold its storey count, whose optional `genres` list is absent or holds the room's zone genre, and whose optional `unlock` is absent or in the run's content set. With one candidate there is no draw; L10 guarantees at least one. A node socket (`"kind": "node"`) is the strip of 4 floor-level cells from `block` along `axis` inside its tile, standing against a wall and never in a wall cell.

**Lost Chapter templates.** `unlock.room.lost_chapter_<genre_suffix>` (doc 14 §2.3, granted by a doc 15 Script) adds 3 templates for that core genre: `room.tpl.lost_chapter_<genre_suffix>_hall` (kind `room.hall`), `room.tpl.lost_chapter_<genre_suffix>_crossroads` (`room.crossroads`) and `room.tpl.lost_chapter_<genre_suffix>_gallery` (`room.gallery`), each with `"genres": ["genre.<genre_suffix>"]` and `"unlock"` set to that unlock. They only join the candidate lists above: typing weights, kind counts and every draw outside the template choice and fill of a room that can take one stay unchanged.

### 10.2 Tilesets

Each genre has a primary and a secondary tileset (§3.3). The secondary's record lists the room kinds it fills in `kinds` (Gothic Horror's catacomb: crypt, cistern, prison, chasm, larder, ambush_den); the primary fills every other kind, and a corridor or a secret room takes the tileset of the room it adjoins with the lower `NodeId`. Each tileset holds 40 to 160 prototypes, each a 64-block payload of palette slots as run-length pairs `[slot, count, …]` (index `lx + 4 × (lz + 4 × ly)`), 6 face sockets (`+X, -X, +Y, -Y, +Z, -Z`) and flags (`walkable`, `cover`, `spawn`, `destructible`, `light`).

```json
{
  "id": "biome.gothic_horror.manor", "role": "primary", "kinds": [],
  "palette": {"1": "block.masonry.slate", "4": "block.planks", "6": "block.light_fixture.gothic_horror"},
  "tiles": [
    {"name": "floor_rug", "w": 120, "sym": "x", "faces": [3, 3, 0, 1, 3, 3], "flags": ["walkable", "spawn"], "rle": [1, 16, 0, 48]},
    {"name": "bookcase", "w": 60, "sym": "l", "faces": [5, 3, 0, 1, 3, 3], "flags": ["cover", "destructible"], "rle": [1, 16, 4, 4, 0, 12, 4, 4, 0, 12, 4, 4, 0, 12]}
  ]
}
```

Slot 0 is air; every other slot names a doc 07 §10.1 block (genre walls and floors are `block.masonry.<stone>` of the genre stone of doc 07 §9.3, never destructible; dressing uses destructible blocks). Example: `floor_rug` is 16 floor blocks (`ly = 0`) then 48 air; `bookcase` is a floor layer and three layers each holding one 4-block shelf row (`lz = 0`) and 12 air, 64 blocks either way. Symmetry `x` (4 identical rotations), `l` (4 distinct), `i` (2); Oneiric flip rooms add a vertical mirror. Two tiles touch when facing sockets are equal or a declared asymmetric pair. Expanded tilesets hold at most 256 variants, so a domain is a `[u64; 4]` bitset.

### 10.3 WFC algorithm

```rust
fn wfc_room(r: &mut Rng, ts: &Tileset, tpl: &RoomTemplate, dims: (u8, u8, u8), cover_bp: i32) -> Result<TileGrid, WfcFail> {
    let mut dom = vec![ts.all_variants(); n_tiles(dims)];
    apply_pins(&mut dom, tpl);
    propagate_all(&mut dom, ts)?;                         // AC-3, FIFO seeded in ascending tile index
    let mut resets = 0;
    loop {
        let Some(t) = min_options_tile(&dom) else { return Ok(collapse_to_grid(&dom)); };  // fewest options >= 2; ties: lowest index
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
    (if ts.has_flag(v, COVER) { max(1, mul_bp(w, 5000 + cover_bp)) } else { w }) as u32   // never 0 (L8: w >= 1)
}
```

- `min_options_tile` picks the uncollapsed tile (2 or more options) with the fewest options, ties to the lowest tile index `tx + tw × (tz + td × ty)`; this is Merrell's minimum-count choice, not Shannon entropy, so it needs no floats. A domain of 0 options is a contradiction, and `propagate_*` returns `Err` on the first one.
- `reset_block` returns every tile of the 4 × 4 × 4-tile block whose corner is `(floor_div(tx, 4) × 4, floor_div(ty, 4) × 4, floor_div(tz, 4) × 4)`, clipped to the room, to its pinned or full domain. On `WfcFail` (a contradiction after the third reset, or one during the initial or a post-reset `propagate_all`) the room takes `fallback_fill` with no further draws. Draws per room equal the template draw (0 or 1, §10.1) plus the number of observations, a declared variable count (Canon §3.4); a 576-tile room makes at most `576 + 3 × 64 = 768` observations, because each of the 3 resets reopens at most one 64-tile block.
- Cover example: a weight-60 cover tile weighs `mul_bp(60, 11000) = 66` in Space Opera (`cover_bp` 6000) and `mul_bp(60, 7000) = 42` in Wuxia (2000).
- Rooms run on `item_rng("wfc", room_index, attempt)`, independently, through `par`.

### 10.4 Micro-voxel shapes (Canon §4.4, DECISIONS 37)

Worldgen writes every block as a block type with a shape. A tile prototype's palette slot names a doc 07 block and, when it is not the block's default shape, a shape: one of doc 03's predefined shapes (full, pillar, slab, stairs, door, trapdoor, ladder, fence, pane, layer, cross, fixture) or an authored custom mask, a `u64` over the cell's 4 × 4 × 4 micro-voxels of 250 mm with bit `mx + 4 × (mz + 4 × my)`.

1. **Where custom masks go.** Custom masks are authored detail in wall, ceiling and prop cells: carved stone, rubble edges, furniture, broken beams. A custom mask stands only on a block whose shape chips (full, pillar, slab, stairs or layer) and is never 0 (doc 03 §4.4). Every feet cell, its support cell and the 2 cells above it use predefined shapes only, so §15.1's navigation reads shapes and V12 holds by construction; lint L8 rejects a prototype that breaks this.
2. **Rotation and mirroring.** A prototype's custom masks rotate and mirror with it (§10.2 symmetry, the Oneiric vertical mirror included); expanding a tileset precomputes the transformed masks.
3. **Rasterizing.** Stage 9 writes each block's shape. Each chunk's custom masks go into its custom-mask table in ascending first-use block linear index (§1), equal masks sharing one entry, and a block stores its table index. No draw.
4. **Grid.** The 1 m cell stays the generation, zone, light, liquid, navigation and spawn grid. Stage `light` reads each cell's shape from its collapsed tile, so its evaluation matches the rasterized chunk; a cell blocks light and sight when `popcount(mask) >= 48` and is otherwise scaled by its fill fraction (Canon §4.4).
5. **Destruction.** Damage to a destructible worldgen block (§9.2) removes its micro-voxels nearest the impact first (Canon §4.4); fixed terrain never loses one. Damage thresholds are doc 03's and doc 07's (Canon §13.8), halved under `lattice.marg.brittle` (§22).

## 11. Dressing, nodes, light, liquids and hazards

### 11.1 Dressing

Each tileset lists props `{block, w, socket_class}`; a prop is a doc 07 block or a multi-block prefab of destructible blocks (crates, barrels, shelves, glass, planks). Per room, on `item_rng(…, "dress", i, attempt)`: one `shuffle` of the room's prop sockets in ascending block index (`len - 1` draws), then the first `floor_div(len × 6000, 10000)` sockets each take one prop by one `weighted` call over the props of that socket's class (a class with no props leaves the socket empty, no draw). Prop sockets never lie on a door-to-door path, so dressing cannot block navigation. The Colophon template additionally places 2 fixed `block.planks` crates (no draw), so V14 and desecration (doc 14 §3.9) are always possible.

### 11.2 Liquids, resource nodes and basins

Stages `liquid` and `node` (`stage_rng`, rooms in ascending `NodeId`):

- **Basins.** A basin socket is a 3 × 3 × 2 floor depression. Each basin socket of a non-safe room draws `roll_bp(2500)` (10000 under `lattice.marg.flooded`, §22); cisterns and the flooded detour's room fill every basin without a roll. A filled basin draws one `weighted` call over the zone genre's `liquids` and is filled to its brim with settled liquid. Safe rooms hold no liquid except font basins.
- **Flooded detour:** at least 2 blocks deep along its door-to-door path with a 2-block air pocket at most every 16 blocks: 16 blocks at the 2400 mm/s swim take `ceil_div(16000 × 60, 2400) = 400` ticks (6.7 s), inside the 14 s breath at END 40 (Canon §12).
- **Exclusions:** no lava, acid or red ink in safe rooms or within 8 blocks of the Incipit spawn; no liquid above the Incipit floor within 8 horizontal blocks. An excluded liquid draw is redrawn among the remaining liquids of the list (one more `weighted` call); with none left the basin stays empty.
- **Red-ink pools:** `range_incl(2, 4)` 3 × 3 × 1 pools in Redaction Folios, each in a non-safe room chosen by `range`; 2 per Editor arena (§21.2), no draw.
- **Font basins** (doc 08 §12.5): each `room.shrine`, `room.garden` and `room.cistern` draws `roll_bp(3000)`; on success one of its basin sockets (the lowest index) holds `liq.wellspring` with `range_incl(1, 3)` uses.
- **Ore veins** (doc 07 §9.4, Canon §4.3): `n_veins = 6 + floor_div(d, 5)`, plus `2 × floor_div(rooms_total, 4)` under `law.post_apocalypse.scarcity`. Each vein draws a non-safe room with a free node socket (`range` over such rooms), a kind (`weighted` over the zone genre's doc 07 §9.4 weights, a kind whose yield's `min_d` exceeds `d` weighing 0) and a size `range_incl(2, 4)`: `size` node blocks (doc 07 §9.3 full-shape prop blocks) fill the first `size` cells of the room's lowest-index free node socket in ascending block index (doc 07 §9.4). A node socket stands against a wall and never replaces a wall cell, so resource nodes are props and terrain stays fixed (Canon §4.3). Veins stop when no room has a free node socket. Example: depth 12 places `6 + 2 = 8` veins; a 16-room Post-Apocalypse Folio places `8 + 2 × 4 = 16`.
- **Harvest nodes** (doc 08 §8.1 H and F): `n_harvest = 4 + floor_div(rooms_total, 4)` when `node_w` sums above 0 (never in the Unwritten). Each draws a room with a free node socket (`range`) and a class (`weighted(node_w)`: plant or fungus) and takes the first cell of that room's lowest-index free node socket; doc 08 fills it with an ingredient of that class by the zone genre's biome tags. Every `room.garden` also gets 2 plant nodes on its first free node sockets, no draw.

### 11.3 Light plan for stealth gradients

Bands use effective light (Canon §2.3, doc 09 §2.2): **dark** `<= 4`, **dim** `5..=9`, **bright** `>= 10`. A room's targets are the zone genre's `bright_bp` and `dark_bp` (§3.3; `bright_bp × 5000` under `lattice.marg.unlit`); kinds tagged `stealth` use `floor_div(bright_bp, 2)` and `min(10000, floor_div(dark_bp × 3, 2))`; the dark target is 0 for open rooms with `ambient_light >= 5` and for any room whose dark share before its first fixture is already below the target (sky light). Stage `light` runs per room on `item_rng(…, "light", i, attempt)`, after `dress`, and evaluates light inside the room box alone (doc 03 §10.3 propagation from the room's own sky cells and fixtures; neighbors and corridors are ignored), so rooms are independent and run through `par.map`. Light from outside can only raise levels, so a safe room proofed in its box stays proofed after the Folio bake (doc 03 §10.5), which V7 checks.

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

Stage `hazard` (`stage_rng(…, "hazard", attempt)`), rooms in ascending `NodeId`:

1. **Count.** `room.gauntlet`: `range_incl(5, 7)` (1 draw). Safe rooms, secret rooms, `room.boss_arena` and `room.arena`: 0, no draw. Every other room: `floor_div(t, 10000) + [roll_bp(t % 10000)]` with `t = trap_bp` of the zone genre (`× 20000` under `lattice.marg.trapped`), 1 draw. Gothic Horror (`trap_bp` 2000) therefore gives a non-gauntlet room 0 or 1 hazards at 20%.
2. **Kind.** Per hazard, one `weighted` call over the zone genre's §18.2 list with its position weights, a hazard ineligible by §18.1 (`dep` axis at local axiom 0, electronic below local TEC 3, structural) weighing 0.
3. **Spot.** *Path traps* (`dart_trap`, `spike_pit`, `gas_vent`, `alarm`, `net_snare`, `rune_ward`, `curse_glyph`, `collapsing_floor`, `quicksand`) take a feet cell on the room's door-to-door path; every other hazard takes a free hazard socket. One `range` over the candidates that keep 4 blocks of spacing from every placed hazard and 8 blocks from the Incipit spawn; with no candidate the hazard is dropped and the room moves on.
4. **Structural hazards** are never drawn here: `hazard.chasm` is the trench of every `room.chasm` and `hazard.gravity_flip` is a §9.5 flip room. They stay in the Unwritten's and the Oneiric's §18.2 lists for skins and preview icons.

Damage enters Canon §10.6 at step 3 with `D2 = base × P(d)` against NPCs and `base × T(d)` against Errata (`src_lvl = d`, Canon §13.8.1), never crits, and builds statuses with `S(d)`. "Per pulse" in §18.1 means each status-pulse tick (period 15, phase 0, Canon §2.2) on which the entity's capsule overlaps the hazard's active volume, so a hazard active for 60 ticks hits 4 times.

## 12. Population interface (doc 13 §10)

Doc 13 owns budgets, packs and waves; worldgen supplies places. Stage `spawn` makes no draws.

- **`spawn_slots`:** `floor_div(pop_w × fw × fd × storeys × density_bp, 10000)` with `pop_w` from §16.1 and the Folio genre's `density_bp` (× 12500 under `lattice.marg.dense`). It is 0 in every rest Folio and for safe, sanctuary, secret and `no_spawn` rooms and rooms with a feet cell within 24,000 mm of path from the Incipit spawn (doc 13 §10.1, doc 14 §3.3). Example: a 2 × 2 single-storey hall (`pop_w` 4) gets `floor_div(4 × 2 × 2 × 1 × 11000, 10000) = 17` in Space Opera and `floor_div(16 × 9000, 10000) = 14` in Gothic Horror.
- **Spawn cells:** a room's feet cells whose tile carries the `spawn` flag and whose `light_block` in the room's §11.3 evaluation is at or below the zone genre's `spawn_block_max`, ascending index (the Folio bake can only light more cells, and doc 13 §10.1 skips those at spawn time). When a room has fewer than `2 × spawn_slots` of them, `spawn_slots` drops to `floor_div(cells, 2)`, so V10 holds by construction.
- **Spawn anchors** (waves, hunters, invaders): for every non-safe, non-secret room with a feet cell at least 24,000 mm of path from the Incipit spawn, its spawn cell nearest the room's center (any feet cell when it has no spawn cell; ties to the lower index); plus the center feet cell of every corridor cell at least 24,000 mm from the Incipit spawn. Rest Folios keep their anchors for Erasure hunters and invaders. Doc 13 §10.4 filters them into qualifying anchors by light, distance and sight.
- **Ambush anchors** (`gpv.evt.ambush` and the dread `ambush` complication): the qualifying anchors (doc 13 §10.4) in rooms or corridors within 2 graph hops of the target room, ascending cell index. With 2 or more, two `range` draws on `rng.event` pick them without replacement (`range(len)`, then `range(len - 1)`); with 1, both packs share it and nothing is drawn; with 0, the ambush becomes doc 13's reserve-wave rule and `rng.event` draws nothing more.
- **Patrol routes:** one per mission-graph cycle, the center feet cells of its rooms in cycle order.
- **Territory:** rooms with `pop >= 2` take `factions[k % F]`, `k` their order among such rooms, `factions` the zone genre's native factions by ascending ID (doc 13 §12.1); none in the Unwritten.
- **Restricted rooms** (`room.restricted`, read by doc 09 §11.3 `disguise.trigger.trespass`): a room of kind `room.armory`, `room.barracks`, `room.guardpost`, `room.key_room`, `room.prison`, `room.reliquary_vault`, `room.server_room` or `room.treasury` carries `room.restricted` of faction F, where F is its own territory faction, else the territory faction of its lowest-`NodeId` neighbor by a non-Hidden mission-graph edge that has one; with neither it carries no restriction. No draws. Safe, sanctuary and secret rooms are never restricted, and the Unwritten has none (no territory). The blueprint stores F per room beside its territory faction. Example: Hardboiled Noir's factions in ascending ID are `city_police`, `dockworkers`, `syndicate`, so an armory that is the first room with `pop >= 2` is restricted to the city police (`fac.native.<genre_suffix>.<group>` with group `city_police`, doc 13 §12.1), and a treasury (`pop` 1) whose lowest-`NodeId` territory neighbor is that armory is restricted to the same faction.
- **Civic anchors:** center feet cells of shop, prison, library, garden and hireling-hall rooms (doc 13 civic packs). `no_spawn` bars packs, waves and hunters; only civic packs and doc 14's Colophon hunter may stand in such rooms.
- **Vendor and Engine sockets** (§6.2): the shop room's 2 vendor sockets and 2 Engine sockets, the rest Folio's Engine hall's 2 Engine sockets, the Interleaf's vendor socket and the Colophon's shop and Engine sockets (doc 11 §11.1, §12.1); the Unwritten's Colophon has no shop (Canon §17.1), so doc 11 leaves its vendor sockets empty.
- **Rubric setup** (doc 14 §3.4, standard and challenge Folios only): Keeper room = the non-safe room farthest by path from the Incipit (ties to the lower `NodeId`); catchwords = container or lore sockets at least 30,000 mm of path from the Incipit, taken in descending distance (ties to the lower index), each in a room with no earlier pick and at least 20,000 mm of path from every earlier pick, until 3 (fewer fails the attempt); Seal point = center feet cell of the non-safe main-path room with the most feet cells among those beyond 5000 Bp of the Incipit→Colophon path distance and with at least 80 feet cells within 4000 mm of that center (none fails the attempt).
- **Rescue:** the rescue room (§8.2) hosts doc 13's stranded native (doc 14 `run.note.rescue`).

## 13. Loot spots (doc 11 §9–§12)

Stage `loot` (`stage_rng(…, "loot", attempt)`) places spots; doc 11 fills them from `rng.loot_place` and keyed `rng.loot`.

- **Chests:** `c = 4 + range(3)` in every standard, challenge and rest Folio (the first draw of the stage; mean 5, the figure doc 11 §9.6's Lucre table assumes), and `mul_bp(4 + range(3), 16667)` = 6, 8 or 10 in Abridged, whose loot quantity (worldgen chests and floor-loot spots) is × 16667 per Folio (Canon §17.1); none in boss Folios. Chests fill container sockets: first one per Treasury (a `loot.chest.locked`), then one each in the armory, the key rooms and the Reliquary vault in ascending `NodeId`, then every remaining chest draws a room by one `weighted` call over non-safe rooms with a free container socket, weight `lw` (§16.1), and takes its lowest free socket. A chest finding no socket stands on its room's free feet cell nearest the center. Doc 11 makes chests in a Treasury or behind a `Locked` edge locked chests and omits Lucre lines in the Unwritten (Canon §5.2).
- **Floor spots:** 9 per slot 0..=3, 36 in all (15 per slot, 60 in all, in Abridged; none in the Unwritten): the feet cells of non-safe, non-secret rooms in ascending index take one `shuffle`, then are taken greedily in shuffled order, skipping any cell within 4 blocks (Chebyshev) of a taken one; spot `k` belongs to slot `floor_div(k, 9)` (`floor_div(k, 15)` in Abridged).
- **Guarded spots:** container sockets of rooms with `spawn_slots > 0`, by `spawn_slots` descending, then `NodeId`, then socket index, for doc 11 §9.9's Volume plan; in a rest Folio, where no room has slots, the container sockets of non-safe rooms by descending `lw`, then `NodeId`.
- **Incipit caches:** the first 3 free container sockets of the main-path rooms after the Incipit, in main-path order (counter caches, doc 11 §9.10).
- **Reliquary spots:** container sockets of the Reliquary vault, then the Treasuries, then other non-safe rooms by descending `lw` (ties to the lower `NodeId`); at least 4 per standard, challenge or rest Folio (doc 11 §10 places at most 4). A boss Folio's illumined Reliquary and gilt Engine stand in its Colophon.
- **Secrets** map to doc 11 tables (§17.2).
- **Story relic spot:** the story seat's (§20.1) lowest free container socket, else its free feet cell nearest the center; no draw. Doc 11 fills it with the story relic, whose affix themes follow the story's 2 domains, and solving the story identifies it (§20.4).

## 14. Bleed, seam and dynamic zones

### 14.1 Bleed plan (pure, previewed)

```rust
fn bleed_plan(run: u64, n: u32, g: GenreId, prev: Option<GenreId>, inp: &GenInput, marg: &[MargId]) -> BleedPlan {
    if !gpv(g).bleed_ok || !inp.bleed_on { return BleedPlan::NONE; }                     // newcomer gate: no draws
    let mut r = stage_rng(run, n, g, "bleed", 0);
    let extra = marg.contains(&MARG_BLEEDING) as u32;
    let forced = g == ONEIRIC || extra > 0 || inp.bleed_add > 0;
    if !r.roll_bp(if forced { 10000 } else { gpv(g).drift_bp }) { return BleedPlan::NONE; }   // 1 call
    let entry = prev.filter(|p| is_core(*p) && *p != g);
    let base = if g == ONEIRIC { 2 } else { 1 };
    let count = min(3, base * inp.bleed_mult as u32 + inp.bleed_add as u32 + extra);   // entry + 3 drift = doc 02's 4 bleed slots
    let cand: Vec<GenreId> = CORE.filter(|c| *c != g && Some(*c) != prev);          // lattice order
    let mut w: Vec<u32> = cand.iter().map(|c| edge(g, *c)).collect();
    let drift: Vec<GenreId> = (0..count).map(|_| { let i = r.weighted(&w); w[i] = 0; cand[i] }).collect();
    let seam = r.roll_bp(gpv(g).seam_bp).then_some(drift[0]);                       // 1 call; count >= 1
    BleedPlan { entry, drift, seam }
}
```

**When a Folio bleeds** (DECISIONS 54). `bleed_on` (`GenInput`, §7.1) is the newcomer gate. Doc 14 sets it at Commit: 1 when the run is seeded (Daily, Weekly or a typed seed, so every player of one seed meets the same Folios) or when the host account has ended at least 3 runs (doc 14 §4.3 party run ends, in any mode except `mode.prologue`); 0 otherwise. It is stored in the run header, so saves, replays, co-op peers and previews read the value fixed at Commit. Once bleed is on, a core-genre Folio bleeds with chance `drift_bp` (§3.5), and the 12 core values average exactly 1000 Bp: 1 Folio in 10. The Oneiric always bleeds; the Redaction, the Unwritten and the Ur-Text (`bleed_ok = 0`) never do. The opt-in modifiers `lattice.marg.bleeding` (§22), `run.mut.wild_bleed` and `run.marg.bleeding_genres` (doc 14) force the bleed or add drift genres, but none bypasses the gate.

A Folio that does not bleed has no entry zone, no drift zone and no seam: every cell is the Folio's genre outside §14.3's dynamic zones. A bleeding Folio has the entry zone (the previous genre at the Incipit, when that genre is core and differs from this one), `count` drift genres and at most one seam, whose source is the first drift genre. `edge(g, c)` reads §5.2 for a core `g` and the §5.3 return weights for the Oneiric. The plan draws nothing when gated, 1 `roll_bp` when the Folio does not bleed, and 1 + `count` + 1 values when it does: 0 to 5 in all. A drift genre never repeats the entry genre, and the cap of 3 keeps the entry zone plus drift zones within doc 02's 4 bleed slots. Examples (run `0x0123456789ABCDEF`, `bleed_on = 1`, prev MF, the bleed roll's `range(10000)` in brackets): Folio 7 GH (8487) → none; ON → entry MF, drift `[MA, GH]`, no seam; CH (3626, against 1300) → none; RD → none, with no draw; GH with `bleed_add = 1` → entry MF, drift `[CH, HN]`, no seam. Folio 2 GH (2208) and Folio 3 GH (8697) → none. Folio 8 CH (711) → entry MF, drift `[GH]`, no seam; Folio 11 WW (977) → entry MF, drift `[HN]`, seam HN; Folio 17 FT (67) → entry MF, drift `[GH]`, seam GH.

### 14.2 Zone assignment (stage `zones`)

Stage `zones` (`stage_rng(…, "zones", attempt)`) works on the typed graph before embedding. Zone records are indexed 0 (plain, the Folio genre), then the entry zone, then drift zones in draw order, then the seam: at most 6 (§7.1). **Eligible** Rooms are typed Rooms that are not safe, sanctuary, `room.arena`, `room.editor_checkpoint`, `room.boss_arena` or tagged `no_bleed`, and not already zoned.

1. **Entry zone** (`zone.bleed`, when the plan has an entry genre): the Incipit, plus its lowest-`NodeId` eligible neighbor on `roll_bp(bleed_in_bp)` (1 draw).
2. **Drift zone** per drift genre, in plan order: the seed Room by `range` over eligible Rooms (1 draw), then a size `range_incl(1, 3)` (1 draw); the zone grows breadth-first from the seed through eligible neighbors in ascending `NodeId` until it holds `size` rooms or no eligible neighbor remains.
3. **Seam** (`zone.seam`, Canon §5.8, when the plan has a seam genre): the candidates are eligible Rooms adjacent to the seam source's zone (the first drift zone, else the entry zone). With none, there is no seam and no draw. Otherwise `k = range_incl(1, 2)` (1 draw), the first seam room by `range` over the candidates (1 draw), and, when `k = 2`, the second is the first room's lowest-`NodeId` eligible neighbor (none: the seam has 1 room). Inside, `local_axiom` is the per-axis maximum of both genres, both genres' Laws apply, natives of either genre are native, the Folio genre supplies skins (`local_genre` is the Folio genre, Canon §5.8), and `st.unmoored` ends (§24.3).
4. Corridors take their lower-`NodeId` endpoint's zone; `zone_of_cell` stores the zone index of every cell, corridors and walls included.
5. A bleed zone uses its genre for `local_genre` (Contradiction re-evaluates on crossing, Canon §5.5), sky light, gravity, §2.1 physics, `spawn_block_max`, tileset, fixtures, hazards, liquids, nodes, Laws, events and roster; doc 13 §12.4 opens a war front at its edge; a Scriptorium inside it Rewrites into the bleed genre (doc 11 §15).

Example: Folio 17 Fairy Tale after Mythic Fantasy (§14.1: entry MF, drift `[GH]`, seam GH): the Incipit and, at 4000 Bp (Fairy Tale's `bleed_in_bp`), its lowest-`NodeId` eligible neighbor are Mythic Fantasy; 1 to 3 Rooms are Gothic Horror; 1 or 2 Rooms next to the Gothic zone are a Fairy-Tale/Gothic-Horror seam where Gothic dread and the Fairy Tale rule of three both run and each axis reads `max(FT, GH)`: ARC 10, TEC 3, MYT 9, VRV 3.

### 14.3 Dynamic zones

`set_dynamic_zone(rooms, genre, until_tick)` overrides `zone_of_cell` for the listed rooms with a `zone.dynamic` record during ticks `t <= until_tick` and restores the stored zone at the end of tick `until_tick`; every override and restore re-evaluates Contradiction for entities inside (Canon §5.5). Dynamic zones are simulation state (doc 03), not blueprint records, and at most 2 are active at once; a third request replaces the one with the earliest `until_tick`. Users: doc 13's `genre_shift` boss phase (1200 ticks), `gpv.evt.genre_storm` (§19), and `room.bleed_shrine`, whose altar (60-tick interact) turns the bleed zone nearest by path (ties to the lower zone index) plain until Folio exit.

### 14.4 Blending

Wall and floor blocks at Chebyshev distance `dist ∈ {0, 1}` from a face between cells of two zones dither; `dist` 0 is the block layer touching the face. For block linear index `b` (§1), `h = mix64(gen_seed ^ (b + 1))`, and the block takes the neighbor zone tileset's block for the same palette slot when `h % 10000 < [5000, 2500][dist]`. No draw is made. Zone membership stays per cell, so blending is cosmetic; doc 18 draws a stitched-page seam along the face.

## 15. Validation and repair

### 15.1 Navigation model

Nodes are feet cells (§1). Edges, each also listing what it needs:

- **Walk:** to a horizontally adjacent feet cell at the same level.
- **Climb up:** 1 block (step-up or jump) when the target has 2 air cells above it; 2 blocks (jump and mantle, doc 03 §5.9: the VERVE-0 apex 1100 mm plus 1600 mm reach) when the target has 2 air cells above it and the source column has 4. Climbable blocks (doc 03 `climbable`) and slab stairs link every level they span.
- **Drop:** to the feet cell below an adjacent air column. A **safe drop** falls at most `floor_div(SAFE_FALL(v), 1000)` blocks, `v` the target's local VERVE (Canon §4.2: 3 blocks at VERVE 0, 4 at VERVE 10); any longer drop is a **hard drop**, which costs HP but is still possible.
- **Swim:** through water, blood, oil, alcohol and every other liquid except lava, acid and red ink, which block; a swim segment longer than 16 blocks without a 2-block air pocket blocks (breath, §11.2).
- **Doors:** Open doors pass; `Locked(keys)` passes when every key is held; a OneWay door passes only from its `from` side; a `PuzzleGate` passes once its puzzle room is reached (§16.3 guarantees solvability); the `RubricGate` passes once every Rubric target is reached (always in rest Folios); `Hidden` edges pass for V4 only (a player who found the secret may use it) and never for V1–V3; relink doors follow the state being checked.

The **safe move set** omits hard drops; the **full move set** includes them. Collapsing floors and spike pits are 2-block pits (§1) and need no special case.

### 15.2 Checks

The key-progression search runs per variant: both relink states (§9.4) as static graphs, and, when the collapsing-bridge cycle exists, the bridge intact and collapsed (4 variants at most).

```rust
fn progression(nav: &Nav) -> Vec<Snapshot> {             // Snapshot = (keys held, puzzles reached, rubric done, reach set)
    let (mut keys, mut puzzles, mut rubric) = (KeySet::EMPTY, PuzzleSet::EMPTY, nav.rest_folio);
    let mut snaps = vec![];
    loop {
        let reach = nav.bfs_safe(nav.incipit_spawn, keys, puzzles, rubric);    // ascending cell index
        snaps.push(Snapshot { keys, puzzles, rubric, reach: reach.clone() });
        let k2 = keys | nav.keys_on(&reach);
        let p2 = puzzles | nav.puzzles_reached(&reach);
        let r2 = rubric || nav.rubric_targets().iter().all(|c| reach.contains(c));
        if (k2, p2, r2) == (keys, puzzles, rubric) { return snaps; }
        keys = k2; puzzles = p2; rubric = r2;
    }
}
```

| ID | Check |
|---|---|
| V1 | the last snapshot's reach set holds a feet cell of every non-secret room |
| V2 | for each `Locked` door, the snapshot that first holds all its keys reaches every key pedestal without crossing that door (a key never sits behind its own lock) |
| V3 | the Rubric targets lie in a snapshot taken while `rubric` was false (reachable without the RubricGate), and the Colophon's feet cells lie in the last snapshot |
| V4 | no softlock: for each snapshot `i`, every feet cell reachable from the Incipit spawn by the full move set with `keys_i` and `puzzles_i` lies in the set `G_i` of cells that reach, by the safe move set, a Colophon feet cell or a key pedestal outside `keys_i`; `G_i` is one reverse breadth-first search from those targets |
| V5 | V1–V4 hold in every variant |
| V6 | no hazard within 8 blocks of the Incipit spawn, no liquid above its floor within 8, no spawn cell or anchor within 24,000 mm path |
| V7 | safe rooms are `no_spawn` and, where `spawn_block_max <= 14`, every feet cell's `light_block` exceeds it after the Folio bake |
| V8 | no electronic feature of §18.3 where local TEC ≤ 2 (Canon §5.7) |
| V9 | no hazard, service or node whose `dep` axis has local axiom 0 (Canon §5.3) |
| V10 | chests as drawn (§13); 36 floor spots (60 in Abridged, 0 in the Unwritten); at least 4 Reliquary spots in standard, challenge and rest Folios; 3 Incipit caches; every room holds at least `2 × spawn_slots` spawn cells |
| V11 | the Interleaf (pos 4) lies at 4000..=6000 Bp of the Incipit→Colophon safe path distance |
| V12 | every feet cell, its support cell and the 2 cells above it use only doc 03's predefined shapes (§10.4); custom micro-voxel masks stand only in wall, ceiling and prop cells |
| V13 | exactly one Colophon with 5 door volumes (up to 3 lattice doors, 1 secret door, 1 "Close the Book" door, doc 14 §3.6); every Secret and SecretExit node joins by a `Hidden` edge only; at most 1 secret exit, and none in a secret-genre Folio; at most 1 seam |
| V14 | at least one destructible block inside the Colophon (the 2 fixed crates, §11.1) |
| V15 | every puzzle room carries its solution aids (§16.3): the lever-order lore objects lie in rooms of an earlier snapshot than the PuzzleGate's far side; plate pairs lie within 8,000 mm of path; 3 `item.tool.torch` ground items and a sturdy face within 1 block of every sigil |
| V16 | the vendor and Engine sockets of §6.2 exist: 2 and 2 in a standard or challenge Folio's `room.shop` (none in the Unwritten), 1 vendor socket in the rest shop and 2 Engine sockets in the rest Engine hall; the Incipit holds the story plaque (§20.3) |

Example (V4): a key pedestal on a ledge reachable only by a 5-block hard drop into a sealed cellar with no way up fails V4, because the cellar's cells are in the full reach set of snapshot 0 but reach neither the Colophon nor a new key by safe moves.

### 15.3 Repair and retry

- **V1–V5:** carve a passage 1 block wide and 2 tall along A* from the reachable frontier to the nearest unreached target (costs: air 1, non-protected solid 5; never through liquid or `protect` cells; ties toward the lower cell index); a carved cell over air gets a floor block of the genre stone. At most 3 carves per attempt, no draws; the checks rerun after each carve.
- **V11:** swap the Interleaf with the main-path Room whose path fraction is nearest 5000 Bp (ties to the lower `NodeId`), then recheck.
- **Retry:** still failing, `attempt += 1` and stages 1–10 rerun on the attempt's sub-streams (§7.2). Attempts 0, 1 and 2 are normal. Attempt 3 is forced simple: `base` = the lower bound without a draw, no minor cycles, `cycle_hidden_shortcut`, and no `PuzzleGate`, relink or gravity-flip room. If attempt 3 fails, the Folio uses the CI-validated authored fallback `room.tpl.fallback.<layout>` in the genre's tilesets; it holds a slot for every fixed room of §6.2, its type activates the slots it needs, and it makes no draws. Plan stages never rerun, so the preview stays true.
- **V6–V16:** fixed in place without draws: remove offending hazards, liquids, anchors or secret exits (highest index first), add safe-room light (§11.3 step 3), recompute tags and spawn slots.

## 16. Room catalog

### 16.1 Kinds

`fp` footprint in cells, `st` storeys, `max` per Folio, `w` typing (spawn) weight that a mission-graph Room uses to become this kind (— = placed by §8.2 rules only), `verb` the GPV verb that scales `w` (§16.2), `pop` population weight (§12), `lw` loot weight (§13). Sanctuaries per doc 14 §3.3. Safe rooms carry `no_spawn`.

| ID | fp | st | max | w | verb | pop | lw | Tags and function |
|---|---|---|---|---|---|---|---|---|
| `room.incipit` | 1×1–2×2 | 1 | 1 | — | — | 0 | 0 | safe, sanctuary; spawn and respawn point; boss antechamber |
| `room.colophon` | 2×2 | 1 | 1 | — | — | 0 | 0 | safe, sanctuary, no_bleed; doc 14 §3.9 services, 5 exit-door volumes, 2 fixed destructible crates |
| `room.interleaf` | 1×2–2×2 | 1 | 1 | — | — | 0 | 0 | safe, sanctuary; Hearth, vendor |
| `room.editor_checkpoint` | 2×2 | 1 | 1 | — | — | 3 | 0 | boss approach (§21.1) |
| `room.boss_arena` | per §21.2 | 2 | 1 | — | — | 0 | 0 | no_bleed; doc 13 §9 geometry |
| `room.campfire` | 1×1–2×2 | 1 | 1 | — | — | 0 | 0 | safe; rest (§16.3); the rest Folio's hub and the Oneiric secret exit's trigger |
| `room.arena` | 3×3 | 2 | 1 | — | — | 0 | 0 | mini-boss room (doc 13), paid from the Folio's population budget |
| `room.key_room` | 1×1–2×2 | 1 | — | — | — | 2 | 0 | key pedestal |
| `room.treasury` | 1×2 | 1 | 2 | 40 | — | 1 | 3 | locked chests |
| `room.reliquary_vault` | 1×1–2×2 | 1 | 1 | — | — | 0 | 2 | Reliquary spots first |
| `room.gauntlet` | 1×3 | 1 | 4 | 60 | traverse | 0 | 2 | 5–7 hazards; chest at the end |
| `room.puzzle` | 2×2 | 1 | 1 | — | — | 0 | 2 | §16.3 |
| `room.cistern` | 2×2 | 1–2 | 1 | 40 | traverse | 2 | 1 | basins always filled |
| `room.chasm` | 1×3 | 1 | 1 | 40 | traverse | 1 | 1 | a trench 5 wide and 6 deep along the long axis (`hazard.chasm`) under a 3-wide destructible plank bridge; a 12-step slab stair climbs from the trench floor to the ledge at the lower-`NodeId` door |
| `room.hall` | 2×2–3×3 | 1–2 | — | 300 | melee | 4 | 2 | combat |
| `room.guardpost` | 1×1–1×2 | 1 | — | 200 | shoot | 2 | 1 | stealth |
| `room.barracks` | 2×2 | 1 | 2 | 120 | melee | 5 | 2 | sleepers |
| `room.crossroads` | 2×2 | 1 | — | 100 | — | 2 | 1 | hub |
| `room.gallery` | 1×3 | 1 | — | 120 | sneak | 2 | 1 | stealth |
| `room.ambush_den` | 2×2 | 1 | 2 | 50 | sneak | 3 | 1 | stealth, dark |
| `room.overlook` | 2×2 | 2 | 1 | 40 | traverse | 2 | 1 | atrium |
| `room.shrine` | 1×1 | 1 | 1 | 80 | pray | 1 | 1 | patron altar `block.altar` of the zone genre (docs 06, 07) and `station.enchanting`; local MYT ≥ 1 |
| `room.library` | 2×2 | 1–2 | 1 | 70 | cast | 1 | 2 | 4 lore sockets; `station.enchanting` |
| `room.observatory` | 2×2 | 2 | 1 | 30 | cast | 1 | 2 | 2 lore sockets |
| `room.alchemy_lab` | 1×2 | 1 | 1 | 60 | craft | 1 | 1 | `station.alchemy` (docs 07, 08) |
| `room.forge` | 2×2 | 1 | 1 | 60 | craft | 2 | 1 | `station.forge` (doc 07 §5.1) |
| `room.workshop` | 2×2 | 1 | 1 | 50 | craft | 2 | 2 | `station.workbench` and `station.loom` (doc 07 §5.1) |
| `room.armory` | 1×2 | 1 | 1 | 70 | shoot | 2 | 3 | racks |
| `room.prison` | 2×2 | 1 | 1 | 50 | talk | 3 | 1 | captive hireling (doc 13) |
| `room.garden` | 2×2–3×3 | 1 | 1 | 60 | craft | 1 | 1 | H nodes, food |
| `room.larder` | 1×1 | 1 | 1 | 50 | craft | 1 | 1 | food; `station.kitchen` |
| `room.crypt` | 2×2 | 1 | 1 | 60 | pray | 3 | 1 | undead roster |
| `room.server_room` | 1×2–2×2 | 1 | 1 | 40 | hack | 2 | 2 | electronic; local TEC ≥ 5 |
| `room.stable` | 2×2 | 1 | 1 | 30 | traverse | 1 | 0 | stalls, tack and fodder as dressing only; `station.loom` |
| `room.shop` | 1×2 | 1 | 1 | — | — | 0 | 0 | safe; 2 vendor and 2 Fortune Engine sockets (§6.2, doc 11 §11.1, §12.1); placed by `room.rule.service` and `room.rule.rest_ring` |
| `room.scriptorium` | 2×2 | 1 | 1 | 20 | — | 0 | 0 | safe; `station.scriptorium`; §16.3 |
| `room.fortune_hall` | 2×2 | 1 | 1 | — | — | 0 | 0 | safe; 2 Fortune Engine sockets for the rest Folio's brass and silver Engines (doc 11 §11.1); placed by `room.rule.rest_ring` |
| `room.illumination_vault` | 1×2 | 1 | 1 | 40 | — | 0 | 2 | unlock-gated; sigil-sealed (§16.3); one `loot.secret.vault` roll; flag `illumination_vault` |
| `room.hireling_hall` | 2×2 | 1 | 1 | 30 | talk | 0 | 0 | unlock-gated; doc 13 hirelings |
| `room.bleed_shrine` | 1×1 | 1 | 1 | 40 | pray | 1 | 0 | unlock-gated; needs a bleed zone; §14.3 |
| `room.duel_page` | 2×2 | 1 | 1 | 30 | melee | 0 | 1 | unlock-gated; a one-aspect champion guards a chest |

Mounts are cut (Canon §17.7), so no room carries a mount tag and `room.stable` is dressing only. With the 8 secret rooms of §17.2 the catalog has 49 kinds. Typing can draw 29 of the 41 above: 25 always and 4 behind unlocks. The tag `room.restricted` is not a kind property: §12 sets it on armories, barracks, guardposts, key rooms, prisons, Reliquary vaults, server rooms and treasuries that hold or adjoin a territory faction.

### 16.2 Typing weight

```
w(kind, genre) = mul_bp(mul_bp(base_w, verb_bp(genre, kind.verb)), bias_bp(genre, kind))
```

`verb_bp` is 10000 for kinds without a verb, `bias_bp` defaults to 10000, and a failed requirement or a missing unlock gives 0. Examples: Cyberpunk server room `mul_bp(mul_bp(40, 16000), 20000) = 128`; Mythic Fantasy library `mul_bp(mul_bp(70, 15000), 12000) = 126`; Wuxia hall `mul_bp(300, 16000) = 480`.

Room bias (Bp): MF armory 12000, library 12000. GH crypt 20000, prison 12000, garden 5000. CH library 20000, observatory 15000, cistern 15000. SO server_room 15000, garden 3000, crypt 0. CP server_room 20000, garden 3000, shrine 5000, crypt 0. PA workshop 15000, garden 6000, library 5000. WW stable 15000, prison 12000. HN prison 15000, gallery 15000, garden 3000, crypt 3000. WX overlook 15000, garden 12000. SP workshop 20000, forge 15000. FT garden 20000, larder 15000. MA shrine 20000, crypt 12000. ON overlook 20000, observatory 15000. RD gallery 20000, server_room 15000, garden 0, stable 0, crypt 0, shrine 0. UW scriptorium 0 (the shop and Engine-hall rules skip the Unwritten, which has no shops, Canon §5.2). UT none.

### 16.3 Services and puzzles

- **Campfire.** A 300-tick `input.interact` channel by an Erratum not `in_combat`; it cancels if the Erratum takes damage, attacks, casts or its origin moves more than 500 mm. On completion: heal `mul_bp(max_hp_mhp, 5000)` through Canon §10.9, stamina and aether set to max, composure +25,000 m (capped at max); once per Erratum per campfire. It does not stop Erasure. The first completed rest in a Folio reveals an Oneiric secret exit's `dream_veil` (§17.4). Example: Max HP 85 at all-40 and 2 Erasure stacks heals `mul_bp(85000, 5000) = 42500`, then × 9400 for Erasure (Canon §11.6) = 39,950 mHP.
- **Scriptorium room:** the Colophon station's Rewrite and identification (doc 10, doc 11 §15), plus perk-column choice (Canon §13.9), reattachment (Canon §9.1) and affliction cure for `mul_bp(500, P(d))` Lucre (Canon §16.4: 1,085 Lucre at `d = 10`). `flags.scriptorium_room` is set when one exists.
- **Puzzles.** Each `room.puzzle` `i` draws its variant by one `weighted([40, 30, 30])` (lever, plates, sigils) on its own sub-stream `item_rng(…, "puzzle", i, attempt)`; the draw is made even under `law.steampunk.clockwork`, which then forces lever. Solving opens the `PuzzleGate` and unlocks the room's chest. Every variant is solvable by one solo Erratum with only what the Folio provides (V15):
  - *Lever order:* 4 levers; the order is one `shuffle` of `[0, 1, 2, 3]` (3 draws). The order is inscribed on 2 lore objects placed by `range` over the lore sockets of rooms reachable without crossing the PuzzleGate (2 draws, without replacement). A wrong pull resets the sequence and fires one `hazard.alarm` effect (`ev.noise` 90 dB at that lever). Under `law.fairy_tale.rule_of_three`, after 2 failed attempts the correct order glows.
  - *Plates:* 3 pressure plates from the template's plate sockets, each pair within 8,000 mm of path. A plate is **latched** for 300 ticks after the last tick on which entities, ground items and placed blocks on it weigh at least 40,000 g in total (an Erratum alone is 75,000 g, Canon §4.1); the puzzle solves on the first tick all 3 are latched. Solo check: two moves of at most 8,000 mm at the 4300 mm/s walk take `2 × ceil_div(8000 × 60, 4300) = 2 × 112 = 224` ticks, under 300.
  - *Sigils:* 3 sigil blocks; the puzzle solves on the first tick all 3 cells have `light_block >= 12`. The room's fixtures avoid every socket within taxicab distance `fixture_level - 11` of a sigil, so the sigils start below 12, and stage `loot` places 3 shared `item.tool.torch` ground items on the room's feet cells nearest its lower-`NodeId` door. Every sigil has a sturdy face within 1 block, where a placed `block.torch` (light 14, doc 07 §10.1) lights it at `14 - 1 = 13`.

## 17. Secrets

### 17.1 Count

For standard and challenge Folios `n_secrets = floor_div(n_target × secret_bp, 10000) + [roll_bp((n_target × secret_bp) % 10000)]`, the second draw of stage `graph` (§8.3); rest and boss Folios have exactly 1 without a draw; `lattice.marg.annotated` adds 2. Example: 18 Mythic Fantasy rooms give `18 × 1500 = 27000`: 2 secrets, a third at 7000 Bp, 2.7 expected. Across the 12 core genres at mid-range room counts a standard Folio expects 1.5 (Weird West) to 4.0 (Fairy Tale) secrets, 2.70 on average.

### 17.2 Secret room kinds

Stage `secret` (`stage_rng(…, "secret", attempt)`) types each `Secret` node in ascending `NodeId` with one `weighted` call over the kinds below in table order; a kind whose condition fails weighs 0. Each is a 1 × 1-cell, 1-storey box.

| ID | w | Condition | Contents (doc 11 tables) |
|---|---|---|---|
| `secret.room.cache` | 30 | — | `loot.secret.cache` |
| `secret.room.shrine` | 15 | local MYT ≥ 1 | hidden patron altar (`block.altar`); its first prayer gains +100 favor before scaling (doc 06) |
| `secret.room.reliquary` | 10 | not the Unwritten | `loot.secret.vault` |
| `secret.room.armory` | 15 | — | 2 racks with at least one counter source (Canon §17.3a) |
| `secret.room.editors_desk` | 2 | `d >= 6` | `loot.secret.palimpsest` (Canon §13.3) |
| `secret.room.margin_note` | 15 | — | 1 story lore object (§20.3) and a `loot.secret.cache` |
| `secret.room.oubliette` | 7 | not the Unwritten | a captive hireling (doc 13 §10.2 step 6) |
| `secret.room.ink_well` | 3 | `d >= 11` | a `liq.red_ink` basin; drinking from it grants one `afflict.red_ink` dose (Canon §16.4) |

**Balance.** A Standard run's Folios 6 to 24 hold an expected `16 × 2.70 + 3 × 1 = 46.3` secrets (16 non-boss Folios, 3 boss Folios). At weight 2 of 97 that is 0.95 Editor's desks per run if every secret is found, each paying one Palimpsestic unique per Active Erratum, beside doc 11 §9.8's 3.125 from Editors and at most 1.25 from Volume rewards per player. The earlier weight of 5 gave 2.31 per run, which nearly doubled the Editor supply for a thorough explorer, so the weight is 2.

### 17.3 Hiding methods

`ev.secret_found` (doc 03 §14.3) carries the hiding method's row index in this table as its `kind` (doc 03's event table), 0 to 10.

| # | ID | Hides by | Found by |
|---|---|---|---|
| 0 | `secret.hide.cracked_wall` | a destructible `block.cracked_wall` with a hairline-crack variant | passive detection (Canon §6.4 row 35, keyed `rng.stealth`, once per observer per 60 ticks within 3 m, Canon §3.3) outlines it and a hit plays a hollow knock; breaking it opens the way |
| 1 | `secret.hide.false_wall` | a block without collision rendered as the room's wall | passive detection; walking into it |
| 2 | `secret.hide.lever` | a lever disguised as a dressing prop opens a 3 × 3 wall section | passive detection outlines it; pulling it (30-tick interact) |
| 3 | `secret.hide.crate_stack` | the opening stands behind destructible crates and barrels | breaking them |
| 4 | `secret.hide.waterfall` | the opening sits behind a 1-block liquid curtain of `liq.water` | seen when any entity crosses the curtain; walking through it |
| 5 | `secret.hide.ceiling_hatch` | a hatch in the ceiling 4 blocks above the neighbor room's floor | passive detection; a placeable ladder or any climb to it |
| 6 | `secret.hide.verb_lock` | a seal needing a verb (below) | performing that verb at the seal; passive detection names the verb |
| 7–10 | `secret.hide.dream_veil`, `secret.hide.clearance`, `secret.hide.blank_page`, `secret.hide.palimpsest_wall` | secret exits only | §17.4 |

**Choice.** A Secret whose box embedding put directly above its neighbor (§9.1 `V`) uses `ceiling_hatch` without a draw. Every other Secret draws `weighted([30, 20, 15, 10, 5, 0, 10])` over methods 0 to 6, with `waterfall` at 0 when the zone genre's `liquids` lack `liq.water` and `verb_lock` at 0 when no verb qualifies. **Verb:** among hack (local TEC ≥ 3; an electronic seal opened by doc 09 Hacking at the Folio's lock tier), prayer (local MYT ≥ 1; any doc 06 prayer within 3000 mm) and spell (local ARC ≥ 1 or a non-`none` aether axiom; any spell hit on the seal), the one whose axis has the lowest local value, ties in the order hack, prayer, spell. Example: in Mythic Fantasy (ARC 8, TEC 2, MYT 6) hack fails its minimum and prayer (6) beats spell (8), so the seal wants a prayer; in Cyberpunk (ARC 1, TEC 9, MYT 2) spell (1) wins.

Revealing a secret (the first Erratum detecting, opening or passing it) emits `ev.secret_found` once per secret.

### 17.4 Secret exits to the four secret genres

Secret exits exist only in core-genre Folios. The plan function `secret_exit_plan(run, n, genre, d, mode)` draws on `stage_rng(run, n, genre, "secret_exit", 0)` in stage 0, so `room.rule.service` (D3) and `room.rule.secret_exit` (D11) can use it. It tests the targets in the order Oneiric, Redaction, Unwritten, Ur-Text, each eligible target with one `roll_bp(chance)`, and stops at the first success (0 to 4 draws). A target is eligible when its chance is above 0, the next Folio is neither a boss (`pos(n + 1) != 5`) nor at depth 25, the build ships the target genre's Folios (doc 14 §2.3; doc 21 §2.3 (d) ships none before 1.0), and its condition holds:

| Exit | Chance | Condition | Hiding and discovery |
|---|---|---|---|
| `secret.exit.oneiric` | GPV `secret_exits` (§5.3; 0 when absent) | `d >= 3` | `dream_veil`: invisible and impassable until an Erratum completes a campfire rest in the Folio (§16.3); D3 adds a `room.campfire` when the graph has none |
| `secret.exit.redacted` | GPV `secret_exits` (0 when absent) | `d >= 6`, Folio genre TEC ≥ 5 | `clearance`: an electronic lock (doc 09) opened by Hacking, or unlocked on the tick an Erasure hunter dies in the Folio |
| `secret.exit.unwritten` | 1000 | `d >= 8` | `blank_page`: visible and passable only for Errata holding at least 5 `st.erasure` stacks (stack 5 lands 840 s after entry, doc 14 §3.8) |
| `secret.exit.ur_text` | 800 | `d >= 26` | `palimpsest_wall`: has no collision while a Palimpsestic item is within 3000 mm of it |

The SecretExit room holds a dog-eared page; a 60-tick interact with it **discovers** the exit, and doc 14 §3.6 appends a door to that genre after the lattice doors, shown as "?". Generation stays pure, so the room exists regardless of history; when the target genre already appeared this run (before Marginalia) or in the last 10 Folios (Marginalia), discovery appends no door. Every SecretExit room also holds a `loot.secret.cache`, so finding it always pays. Example: a Standard Fairy Tale Folio at `d = 8` (the next Folio is at pos 4) tests Oneiric at 2500 Bp, then Unwritten at 1000 Bp (Redaction has chance 0 there and is skipped without a draw), so it plans an exit with probability `2500 + floor_div(7500 × 1000, 10000) = 3250` Bp. Folio 9 plans none, because Folio 10 is a boss. Vectors for run `R = 0x0123456789ABCDEF` (`range(10000)` values in brackets): Fairy Tale Folio 8 tests Oneiric (8593) and Unwritten (9259) and plans none; Mythic Fantasy Folio 8 tests Oneiric (7395, against 2000) and then plans an Unwritten exit (680 < 1000); Hardboiled Noir Folio 7 plans a Redaction exit (706 < 2500); Gothic Horror Folio 5, a boss Folio before the pos-1 Folio 6, plans an Oneiric exit (968 < 1500).

## 18. Hazards and traps

### 18.1 Archetypes

`base` is display HP (Canon §10.6), so 6 means 6,000 mHP before `T(d)` or `P(d)`; buildup values are status powers scaled by `S(d)`. Hidden hazards (plates, tripwires, glyphs, lids) are hidden objects for passive detection (Canon §6.4 row 35) and are disarmed with Security (doc 09). "E" marks electronic hazards, which need local TEC ≥ 3 (Canon §5.7; §18.3 lists every electronic feature); `dep` follows Canon §5.3 and bars a hazard where that axis is 0. Rearm: a plate or tripwire rearms 60 ticks after it fires; a glyph, lid or snare fires once.

| ID | Trigger and cycle | Effect | dep |
|---|---|---|---|
| `hazard.dart_trap` | hidden plate | 3 darts at 25000 mm/s from the nearest wall, 6 `dmg.pierce` each | none |
| `hazard.blade_trap` | sweeps every 60 ticks along a 4-block arc | 14 `dmg.slash`, 30 bleeding | none |
| `hazard.spike_pit` | a hidden `block.pit_lid` over a 1 × 1 pit 2 blocks deep; it breaks under ≥ 40,000 g | 10 `dmg.pierce`, 40 bleeding; the 2000 mm fall is under every `SAFE_FALL`; climb out by jump and mantle (§1) | none |
| `hazard.crusher` | slams every 120 ticks after a 30-tick telegraph | 25 `dmg.blunt` | none |
| `hazard.flame_jet` | active on ticks 0..60 of every 180; 4 m cone, half-angle 2731 Bam | 6 `dmg.fire` per pulse, 60 burning | none |
| `hazard.gas_vent` | hidden plate | 6 cells of `gas.miasma` over 120 ticks | none |
| `hazard.shock_floor` | E; 3 × 3 floor live on ticks 0..90 of every 180 | 6 `dmg.shock` per pulse, 40 shocked | techne |
| `hazard.alarm` | hidden tripwire or sensor | `ev.noise` 90 dB at the source; rearms after 600 ticks | none |
| `hazard.net_snare` | hidden tripwire | `st.rooted` directly; 3 × 3 `block.web` around the cell | none |
| `hazard.rune_ward` | hidden glyph | 3 m blast, 18 `dmg.arcane`, 50 unraveled | arcana |
| `hazard.curse_glyph` | hidden glyph | 60 dread and 40 withering buildup to every entity within 2 m | mythos |
| `hazard.camera` | E; cone half-angle 8192 Bam, 16 m, sweeping ±4096 Bam over 360 ticks | 60 consecutive ticks of sight on an Erratum fire the `hazard.alarm` effect at the camera and doc 09's `alarm_raised`; hackable | techne |
| `hazard.collapsing_floor` | 3 × 3 `block.pit_lid` boards over a 3 × 3 pit 2 blocks deep; they break 30 ticks after bearing ≥ 40,000 g | drop 2 blocks; climb out by jump and mantle (§1) | none |
| `hazard.falling_debris` | an `ev.noise` of ≥ 80 dB within 6 m | 3 `block.rubble` blocks fall, 12 `dmg.blunt` each | none |
| `hazard.lava_pool` | contact | a 3 × 3 × 1 basin of `liq.lava` (Canon §12.1) | none |
| `hazard.acid_pool` | contact | a 3 × 3 × 1 basin of `liq.acid` | none |
| `hazard.deep_water` | contact | a 3 × 3 basin of `liq.water` 3 blocks deep (swim, breath) | none |
| `hazard.oil_slick` | contact | 3 × 3 floor cells of `liq.oil`, 1,000 ml each (contact needs 100 ml) | none |
| `hazard.ink_seep` | contact | 3 × 3 floor cells of `liq.red_ink`, 1,000 ml each | none |
| `hazard.steam_vent` | active on ticks 0..60 of every 240 | 2 cells of `gas.steam` | none |
| `hazard.rad_zone` | a 4 × 4 floor area, always on | 20 irradiated per pulse | none |
| `hazard.miasma_bog` | persistent | `gas.miasma` filling 3 × 3 × 2 cells | none |
| `hazard.web_field` | persistent | `block.web` filling 3 × 3 × 2 cells | none |
| `hazard.quicksand` | a 3 × 3 mire | `st.slowed` while inside; the body sinks at most 1 block | none |
| `hazard.hull_breach` | a `block.hull_panel` (doc 07 §10.1, replacing glass pane hardness) breaks | a pull of 3000 mm/s² toward the panel on every body within 8 m for 300 ticks, then the panel reseals | techne |
| `hazard.redaction_bar` | a 1 × 3 × 8 slab sliding at 2000 mm/s along its corridor, reversing at the ends | conceal 15; contact applies `st.silenced` directly | techne |
| `hazard.chasm` | structural: the trench of `room.chasm`, 6 blocks deep | fall damage (Canon §4.2); the trench stair leads back up | none |
| `hazard.gravity_flip` | structural: a §9.5 flip room | gravity reversed | none |

Examples against Errata: a dart at `d = 10` deals `mul_bp(6000, 16190) = 9714` mHP (13,030 to an NPC via `P(10) = 21718`); a crusher at `d = 15` `mul_bp(25000, 21160) = 52900`; a flame pulse at `d = 5` `mul_bp(6000, 12388) = 7432`.

**Balance against Canon §17.4** (Folio 1, an Erratum with 85 HP and `REF_ERRATA_AR_D1` 20, Canon §10.5 step 5): a crusher slam `effAR = floor_div(20 × 5000, 10000) = 10`, `DR = floor_div(10^8, 10,000 + 100,000) = 909`, leaves 22,728 mHP (27% of 85 HP, 2.3 times `REF_ENEMY_HIT_D1`), after a 30-tick telegraph; three darts leave `3 × 3,693 = 11,079` mHP (13%); a flame jet's full 60-tick window is 4 pulses of 6,000 fire (no armor) plus a burning proc, 24,000 + 6,800 mHP (36%), while crossing the cone's far end (`2 × 4000 × tan 15° = 2,144` mm wide) at the 4300 mm/s walk takes 30 ticks, 2 pulses (12,000 mHP, 14%, plus the proc). The flame pulse was 8 HP, which made a full window 45% of a Folio-1 Erratum's HP, more than a telegraphed crusher; 6 HP puts it beside the crusher. A chasm fall at VERVE 5 costs `excess = 6000 - 4000 = 2000` mm, 16% of Max HP (Canon §4.2), times the genre's `fall_dmg_bp`. Against NPCs the same hazards scale with `P(d)` while NPC HP scales with `H(d)` (Canon §13.8), so luring enemies into traps is strong early (a crusher's 25 HP is 42% of a 60-HP standard at Folio 1 before armor) and fades with depth (`mul_bp(25000, 79110) = 197,775` mHP against that standard's `mul_bp(60000, 202521) = 1,215,126` at Folio 25, 16%).

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
| UT | rune_ward, camera, curse_glyph, flame_jet, ink_seep, collapsing_floor; the `k`-th room (ascending `NodeId`) holding Ur-Text hazards skins them as core genre `k % 12` in lattice order |

Lint L4 holds: nothing electronic in MF, WX, FT, MA, UW; no arcana hazard in PA, HN, UW; no mythos hazard in UW. `chasm` (UW) and `gravity_flip` (ON) are structural (§11.4), so those two genres draw from 5 hazards each in stage `hazard`, with the remaining position weights 20, 15, 15, 15, 15 (UW) and 20, 15, 15, 15, 15 (ON); every other genre draws from all 6.

### 18.3 Electronic features (Canon §5.7)

Canon §5.7 bars electronic locks, cameras and turrets where the local TEC is 2 or less. This list is every worldgen feature that counts as electronic. Each placement rule gives it weight 0 or skips it where the local TEC is 2 or less (typing §8.4 and §16.2, hazards §11.4, seals §17.3, secret exits §17.4, event `requires` §19.1), and V8 rejects any that remains: `hazard.shock_floor`, `hazard.camera`, `gpv.evt.lockdown`, the `clearance` lock of `secret.exit.redacted`, the `hack` verb seal of §17.3, `room.server_room`, every electronic `lock.*` door (doc 09), and the doors that `law.space_opera.shipboard_systems` makes electronic. Doc 13's turrets inherit the rule.

## 19. Ambient and narrative events

### 19.1 Archetypes

`requires` reads the target room's local genre; gated events need their unlock in the run's content set. **Target rooms** are the rooms holding or adjoining an Erratum (by mission-graph edge) that meet `requires`, ascending `NodeId`, except where the row says otherwise; an event with no target room is ineligible.

| ID | Class | Effect | requires |
|---|---|---|---|
| `gpv.evt.light_failure` | ambient | the target room's fixtures go dark for 600 ticks | — |
| `gpv.evt.weather_front` | ambient | 1800 ticks: fog +1 per 8 m, doc 09 sound attenuation × 12000 | — |
| `gpv.evt.distant_noise` | ambient | `ev.noise` 70 dB at the target room's center; its target rooms are those at least 2 graph hops from every Erratum's room | — |
| `gpv.evt.tremor` | ambient | fires the room's falling debris, else drops 2 `block.rubble` | — |
| `gpv.evt.whispers` | ambient | −5 composure (× the Canon §5.7 MYTHOS composure factor, doc 04) to each Erratum there with `light_at < 8` | MYT ≥ 1 |
| `gpv.evt.boon` | ambient | `st.blessed` applied directly (30 s) to each Erratum there | MYT ≥ 1 |
| `gpv.evt.lore_echo` | ambient | a lore object with an unread story event appears (§20) | — |
| `gpv.evt.editor_glimpse` | ambient | 2,000,000 ml (2 full cells) of `liq.red_ink` appear at the room's center floor and spread by doc 03's liquid rules | — |
| `gpv.evt.gravity_shift` | ambient | every gravity-flip volume toggles for 600 ticks | flip cells |
| `gpv.evt.door_slam` | ambient | the open door nearest an Erratum closes and carries `protect` for 600 ticks | — |
| `gpv.evt.wandering_vendor` | narrative | a `fac.neutral` vendor stays 1800 ticks (doc 11) | `unlock.room.wandering_scribe` |
| `gpv.evt.genre_storm` | narrative | a dynamic zone (§14.3) of the Folio genre's top-weight lattice neighbor (§5.2, or §5.3 return weights; ties to lattice order; Cyberpunk → Hardboiled Noir) covers the target room for 1200 ticks, moving every 400 ticks to that room's lowest-`NodeId` non-safe neighbor | `unlock.event.genre_storms` |
| `gpv.evt.patrol_shift` | narrative | doc 13 spawns a 1000-mSKE wave at the spawn anchor nearest the Colophon, walking toward the Incipit | — |
| `gpv.evt.ambush` | narrative | doc 13 spawns a 2000-mSKE wave split over the 2 ambush anchors of the target room (§12; 0 or 2 more draws) | — |
| `gpv.evt.reinforcements` | narrative | doc 13 spawns a 3000-mSKE wave at the qualifying anchor farthest from every Erratum | — |
| `gpv.evt.plea` | narrative | a bound `npc.captive.<genre_suffix>` of the zone genre (doc 13 §13.1, trader faction) appears in the target room; freeing it (60-tick interact) within 1800 ticks pays doc 13's `fac.rep.free_captive` with its plea bonus (+150, then +100 more) and a `soldier` hireling offer (doc 13 §13.4); it fulfils `law.mythic_fantasy.oathbound` | not the Unwritten |
| `gpv.evt.duel_challenge` | narrative | a champion carrying the aspect set's first aspect (doc 13 §10.4) challenges the party; the first Erratum to accept (interact) duels it behind a barrier of `protect` cells for at most 1800 ticks | VRV ≥ 5 |
| `gpv.evt.flood` | narrative | the target room's lowest-index filled basin releases 20,000,000 ml of its liquid over 600 ticks | a filled basin in the room |
| `gpv.evt.fire_outbreak` | narrative | ignites the flammable block nearest the room center (doc 03) | — |
| `gpv.evt.lockdown` | narrative, electronic | the doors of the target room and its neighbors shut and carry `protect` for 900 ticks; cameras there activate (hackable, doc 09) | TEC ≥ 3 |
| `gpv.evt.redaction_sweep` | narrative | a `hazard.redaction_bar` sweeps the lowest-index corridor adjoining the target room once | TEC ≥ 1; the room has a corridor |

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

The host draws every event on the Folio's sequential `rng.event` stream (Canon §3.2). Standard and challenge Folios only: boss and rest Folios get no events, complications or guaranteed ambushes. On a tick that qualifies for several, the order is guaranteed ambush, event check, complication check (§4).

```rust
// Event checks: ticks with t - entry >= 1800 and (t - entry) % 1200 == 0,
// while the Colophon services are closed (doc 14 §3.9) and fewer than 6 events have fired.
fn event_check(r: &mut Rng, f: &FolioState) {
    if !r.roll_bp(f.gpv.event_bp) { return; }                                   // 1 draw
    let elig = f.gpv.events.filter(|e| f.cooldown_ok(e) && !f.target_rooms(e).is_empty());   // list order
    if elig.is_empty() { return; }
    let e = elig[r.weighted(&position_weights(&elig))];                           // 1 draw
    let rooms = f.target_rooms(e);                                                // ascending NodeId
    let room = rooms[r.range(rooms.len() as u64) as usize];                       // 1 draw
    if e == AMBUSH { ambush_anchors(r, f, room); }                                // 0 or 2 draws (§12)
    fire(e, room, f);
}
```

`cooldown_ok`: an event that has fired in this Folio never fires again in it, and a narrative event needs 2400 ticks since the last narrative event. `position_weights` keeps each eligible event's §19.2 list-position weight (25, 20, 15, 15, 15, 10). `gp.ambush_extra` (doc 14) adds guaranteed ambushes at `entry + 5400 × k`, `k = 1..=ambush_extra`, outside the cap of 6 and the cooldowns: each draws 1 target-room `range` and the 0 or 2 anchor draws. A check therefore draws 1, 3 or 5 values.

Example: a 360 s Folio runs `floor_div(21600 - 1800, 1200) + 1 = 17` checks (entry + 1800, …, entry + 21000) if nobody enters the Colophon earlier, and fires at most 6 events. For run `0x0123456789ABCDEF`, Folio 2 has `stream_seed(R, "rng.event", 2) = 0x9ad6720d21d175fe`, whose first `range(10000)` is 2892. In a Space Opera Folio 2 (`event_bp` 3000, no clock Law) that value is the first check's roll at entry + 1800 and fires an event; in a Gothic Horror Folio 2 the same value is instead the complication roll at entry + 600 (`roll_bp(250)` with an Erratum in combat since entry + 360, which fails).

## 20. Folio story fragments

Following Caves of Qud's sultan histories (R1 §5.9, R4 §7.3: events are generated first and rationalized after the fact), stage `story` draws a life in events, then explains its contradictions with bridging events, and scatters biased, contradictory retellings through the Folio. Every Folio type has a story; it draws on `stage_rng(…, "story", attempt)` in the order below. Stories are prominent (DECISIONS 72): each Folio presents its story at the Incipit (§20.3 story plaque), offers it as an optional Folio quest (§20.6), and keeps it as a fragment of the run's procedural ending (§20.7).

### 20.1 Cast

Draws 1–4: the hero's and the rival's name seeds as two `next_u64` values (doc 18 §7 name grammars), then 2 of the GPV's 4 `story_domains` without replacement (`range(4)`, then `range(3)` over the rest). The **seat** is the lowest-`NodeId` treasury, Reliquary vault or library, else the non-safe room with the highest `lw` (ties to the lower `NodeId`).

### 20.2 Events and rationalization

The life has slots origin, rise, deed and fall, then the fixed `gpv.story.erasure.scraped`. Draws 5–8: each slot draws one template with one `weighted` call over **all** of the slot's templates in table order, weight `10 + 20 × [one of the template's domains is a drawn domain]` (`any` never counts as a match). The draw ignores `requires`. Then **rationalization**: when the drawn template's `requires` is not met by the tags held so far, the bridge granting the first tag of that `requires` is inserted before it, with no draw. A story therefore has 5 to 8 events (at most one bridge per slot after the origin, which has no `requires`) and always makes exactly 8 draws in §20.1–§20.2.

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
| `gpv.story.bridge.ennobled` | — | `noble` | any |
| `gpv.story.bridge.orphaned` | — | `orphan` | any |
| `gpv.story.bridge.found_relic` | — | `relic` | any |
| `gpv.story.bridge.apprenticed` | — | `trained` | any |
| `gpv.story.bridge.made_pact` | — | `pact` | any |
| `gpv.story.bridge.was_cursed` | — | `curse` | any |
| `gpv.story.erasure.scraped` | — | `erased` | any |

Every first `requires` tag (`noble`, `orphan`, `relic`, `trained`, `pact`, `rival_kin`, `renown`, `curse`) has exactly one bridge, so rationalization always succeeds.

**Worked example** (Steampunk, domains `[brass, clock]`). Origin weights `[10, 10, 10]` (total 30), rise `[10, 10, 10]`, deed `[10, 30, 10]` (built_wonder matches brass and clock; total 50), fall `[10, 10, 10]`. With `range` values 25, 23, 20 and 5: origin 25 ≥ 20 → apprentice (`trained`); rise 23 ≥ 20 → pact, whose `orphan` is missing, so `gpv.story.bridge.orphaned` goes first; deed 20 lies in 10..40 → built_wonder (`trained` held; grants `renown`, `wonder`); fall 5 → betrayed (`renown` held). The life reads: an apprentice, later orphaned, made a pact, built a wonder, was betrayed, and was scraped from the page: 6 events. With values 25, 15, 20, 15 instead it is apprentice, relic_found, built_wonder, hubris, scraped, the 5-event record of §20.5. Vector: run `R`, Folio 2, Gothic Horror, attempt 0 draws hero seed `0x417db148d4394087`, rival seed `0x630efb60831bfd3c`, domains `[moon, blood]` and slot values 27 of 30 (apprentice), 21 of 30 (pact, bridged by orphaned), 2 of 30 (slew_beast, bridged by found_relic) and 47 of 70 (fall weights `[30, 10, 30]`: cursed): 7 events, apprentice, orphaned, pact, found_relic, slew_beast, cursed, scraped.

### 20.3 Retellings and placement

- `L = 3 + floor_div(rooms_total, 6)` lore objects (6 at 18 rooms). Draws: one `shuffle` of the event indices `0..E` (`E - 1` draws), and object `k` shows event `perm[k % E]`, so each event appears once when `L >= E`.
- Voice per object `k`: one `weighted([40, 40, 20])` over praise, slander, redact. Object `k` is **overwritten** when `k % 3 == 2`: hero and rival swap roles in its text (doc 18 §8.7), so two objects contradict each other on purpose.
- Socket per object `k`: object 0 takes the lore socket of `secret.room.margin_note` when one exists (no draw); every other object makes one `weighted` call over the free lore sockets in ascending block index, weighted by room kind (library 4, shrine 3, observatory 2, server room 2, crypt 2, every other non-safe room 1, safe rooms 0), and a used socket weighs 0 afterwards. With no free socket left, the remaining objects are dropped. Order: the shuffle first, then for `k = 0, 1, …` the voice draw and then the socket draw, so a story makes `8 + (E - 1) + L + (L - [margin note])` draws in all (25 for `E = 6`, `L = 6` without a margin note).
- **Story plaque.** Every Incipit template has one fixed plaque socket on the wall facing the spawn point. In every Folio it holds one extra lore object, the plaque, which shows event 0 (the origin) in voice praise, never overwritten, followed by doc 18's fixed line naming the seat room and the Folio Leaf. The plaque makes no draw, is not counted in `L`, and reading it counts as reading event 0 (§20.4).

### 20.4 Reading and solving

Reading is a 60-tick `input.interact` granting Inscription XP (doc 04, magnitude `d`). Once the party has read 3 distinct events (2 under `law.cosmic_horror.forbidden_knowledge`; bridges and the plaque's event 0 count) the story is **solved**: the best secret (editor's desk, then Reliquary, then the §17.2 table order, ties to the lower `NodeId`) is revealed to the party with `ev.secret_found`; the story relic in the seat's relic spot (§13, filled by doc 11) becomes `know.identified` (doc 10 `identify_instance`, reason `appr.reason.story`); doc 14's `FolioRecord` stores the solve tick as `story_solved` (§20.7); `ev.objective_completed` is emitted with kind 2 (doc 03 §14.3); and under `law.hardboiled_noir.leads` the Folio's SecretExit room, if any, is revealed too.

### 20.5 Folio Leaf and text

One `cat.quest` item per Folio, the Folio Leaf (`item.quest.folio_leaf`, doc 11; Canon §13.1 makes `cat.quest` this chapter's), lies at the seat as a shared ground item. On the exit-commit tick (doc 14 §3.6), a Folio Leaf in an Active Erratum's inventory is consumed and sets the Folio record's `leaf_carried`. At run end, and at a `run.end.departed` for that slot, doc 14 §5.5 adds the story to the Index of every present account in the party when the story was solved or its Leaf carried out, so carrying the Leaf records even an unsolved story; doc 18 renders the story when it is opened in the Margin (`nar.job.codex_story`). A Leaf left behind is lost with the Folio. Text comes from doc 18's `nar.grammar.story.<template_suffix>.<genre_suffix>` grammars (doc 18 §8.7) or the Scribe model, with story titles `loc.gpv.story.<template_suffix>.title`, is cached by seed, and never enters simulation state. The blueprint stores only the record below: seeds as `"0x"` hex strings (Canon §1.6), template IDs, and per lore object its room `NodeId`, event index, voice (0 praise, 1 slander, 2 redact) and overwritten flag.

```json
{ "hero_name": "0x9c1f00d2a4be7731", "rival_name": "0x13a4c0ffee0f1e2d", "domains": ["brass", "clock"],
  "events": ["gpv.story.origin.apprentice", "gpv.story.rise.relic_found", "gpv.story.deed.built_wonder", "gpv.story.fall.hubris", "gpv.story.erasure.scraped"],
  "lore": [{"room": 4, "event": 2, "voice": 0, "overwritten": 0}, {"room": 9, "event": 0, "voice": 1, "overwritten": 0}, {"room": 11, "event": 3, "voice": 2, "overwritten": 1}] }
```

### 20.6 The Folio quest (DECISIONS 72)

Every Folio offers one optional quest with two steps: **solve the story** (§20.4) and **carry the Folio Leaf out** (§20.5). Either step may come first, and the quest is complete when both have happened. It never gates the Rubric, the Colophon or an exit, has no timer, and lapses when the party leaves the Folio with a step unfinished. Doc 17 lists it from Folio entry, under the Rubric, with the state of each step. Solving pays §20.4's revelations and fixes the story in the run's ending (§20.7); either step records the story in the party's Index, and carrying the Leaf out is the only way to record a story left unsolved (§20.5).

### 20.7 Story fragments in the closing manuscript (DECISIONS 46)

Each Folio's story is a fragment of the run's procedural ending. For every Folio entered, doc 14's `FolioRecord` stores the blueprint's story record of §20.5 (at Folio entry), `story_solved` (the tick the story was solved, else none) and `leaf_carried`. Doc 18 §8.5a assembles the closing manuscript from these records together with the run's genres, the party's Echoes or Origins and its notable choices: per Volume it retells the lowest-index solved story in voice praise. An unsolved story adds nothing. The record holds seeds and template IDs only, so the ending regenerates its text whenever it is opened and stores none.

## 21. Boss Folios and Editor arenas

### 21.1 Layout

`room.rule.boss_spine`: Incipit (antechamber sanctuary, doc 14 §3.7) – Editor checkpoint – boss arena – Colophon. The Checkpoint embeds only from §9.1's corridor candidates `C` at Chebyshev distance 3 or 4 cells from the Incipit, so at least 32 blocks of corridor separate them and every checkpoint feet cell lies beyond doc 14's 24,000 mm spawn exclusion; it holds doc 13 §10.1's squad and the boss proof (doc 13 §9.1). The arena follows the Checkpoint as an `H` candidate (a shared door) when one fits, else through a corridor. The Endpaper plays in the arena; the Colophon unseals per doc 14 §3.2; Erasure pauses while the Editor lives (Canon §11.6). Boss Folios have no Rubric, events or complications, and exactly 1 secret.

### 21.2 Arena construction

Geometry is doc 13 §9.2's; Marginalia reprises reuse it (doc 14 §9.1 rotation).

1. **Box:** `ceil_div(interior_m + 2, 16)` cells per horizontal axis and `ceil_div(height_m + 2, 8)` storeys, where `height_m` is the interior height (the void depth included). The box may rise above the genre's `storeys` (§9.1 item 6).

| Editor | doc 13 §9.2 interior | `height_m` | Box (cells, x × z × storeys) |
|---|---|---|---|
| `boss.editor_v1` the Proofreader | round, radius 20 m: 40 × 40 m | 12 (ceiling) | `ceil_div(42, 16) = 3` × 3 × `ceil_div(14, 8) = 2` |
| `boss.editor_v2` the Censor | archive 40 × 28 m | 8 (ceiling) | 3 × `ceil_div(30, 16) = 2` × `ceil_div(10, 8) = 2` |
| `boss.editor_v3` the Continuity Editor | hall 36 × 36 m, mezzanine at 5 m | 12 | `ceil_div(38, 16) = 3` × 3 × 2 |
| `boss.editor_v4` the Abridger | cross of 5 platforms of 12 × 12 m with 4 m gaps: `3 × 12 + 2 × 4 = 44` m | 18: a 10 m void below the platforms (the doc 13 fall trigger is 8 m down) and 8 m above | `ceil_div(46, 16) = 3` × 3 × `ceil_div(20, 8) = 3` |
| `boss.ur_editor` the Last Word | ring of four quadrants, 48 × 48 m | 12 | `ceil_div(50, 16) = 4` × 4 × 2 |

2. **Fill:** shell and floor from the Folio genre's primary tileset; doc 13's features (lectern pillars, shelf walls, mezzanine and ramps, platforms and bridges, quadrants) are authored prefabs `room.tpl.arena_<boss suffix>` placed without WFC or draws.
3. **Light and ink:** `bright_bp` 6000 through §11.3 (dark target 0); 2 `liq.red_ink` pools (3 × 3 × 1) on the floor cells nearest the two arena corners away from the doors (doc 08).
4. **Doors** to the Checkpoint and the Colophon carry `protect` while the arena is sealed (doc 13 §9.1, doc 14 §3.2).
5. **Mini-boss arena.** `room.arena` is a generic 3 × 3-cell, 2-storey box (48 × 48 blocks outside, 46 × 46 inside) with `p = 4 + floor_div(cover_bp, 1000)` pillars of 2 × 2 blocks, full interior height, at radius 12 blocks from the arena's center block. Pillar `k` has yaw `y_k = (floor_div(65536 × k, p) + phase) mod 65536`, `phase = range(65536)` (1 draw on `item_rng(…, "arena", node, attempt)`), and center offset `(round_half_even(-12 × sin_bam(y_k), 2^30), round_half_even(-12 × cos_bam(y_k), 2^30))` blocks (Canon §1.3, §1.9); its 2 × 2 footprint spans `center_x - 1 ..= center_x` by `center_z - 1 ..= center_z`. Example: `p = 4`, `phase = 0` puts pillars at yaw 0, 16384, 32768 and 49152, offsets `(0, -12)`, `(-12, 0)`, `(0, 12)` and `(12, 0)`: north, west, south and east. Space Opera (`cover_bp` 6000) gets 10 pillars 36° apart (chord 7.4 blocks), Wuxia 6, the Unwritten 5. Doc 13 §9.3's dressing prefabs (cathedral, war camp, ballroom, moorland graveyard, flooded crypt, drowned observatory, bridge deck, hive chamber, server vault, nightclub, dust arena, reactor hall, gallows square, dry wash, penthouse, rooftop, bamboo courtyard, moon pavilion, brass foundry, airship deck, gingerbread cottage, giant's kitchen, labyrinth, marsh temple, dream bedroom, exam hall, office, containment wing) go in after the pillars and skip pillar blocks; prefab extents above 46 m are clipped to 46 m.

## 22. Marginalia modifiers

From Folio 26 each Folio has `k = min(4, 1 + floor_div(n - 26, 10))` modifiers: 1 at Folios 26–35, 2 at 36–45, 3 at 46–55, 4 from 56. The plan function `marg_mods(run, n, genre, bleed_on)` draws them on `stage_rng(run, n, genre, "marg", 0)` with `k` `weighted` calls over the table in order, setting each picked weight and every weight excluded for the genre to 0 (without replacement); the exit preview shows them (§5.7). They stack with doc 14 §9.3's run-long Marginal mutations and apply in boss Folios too, where some have nothing to act on.

| ID | w | Excluded | Effect |
|---|---|---|---|
| `lattice.marg.annotated` | 60 | — | `n_secrets` + 2 (§17.1) |
| `lattice.marg.bleeding` | 80 | RD, UW, UT; every Folio while `bleed_on` is 0 | drift chance 10000 and one extra drift genre (`extra = 1` in §14.1), within the cap of 3 |
| `lattice.marg.brittle` | 60 | — | every worldgen `destructible` block (doors, crates, barrels, planks, glass, cracked walls) has half its doc 07 block HP |
| `lattice.marg.champions` | 60 | — | doc 13 doubles the budget champions and adds one aspect to the aspect set, at most 3 (doc 13 §7.3, §10.1) |
| `lattice.marg.dense` | 100 | — | `density_bp` × 12500 |
| `lattice.marg.erased` | 40 | — | after stage `hazard`, 3 more `hazard.ink_seep`, each in a non-safe room by `range` on a free hazard socket (3 draws) |
| `lattice.marg.flooded` | 60 | UW | every basin socket of a non-safe room fills (chance 10000); each `room.cistern` turns 2 more free floor areas into filled basins, no draw |
| `lattice.marg.sealed` | 40 | UW | D3 places no `room.shop`, so the Folio has no mid-Folio vendors or Fortune Engines (the Colophon and Interleaf keep theirs); chests `c + 2` |
| `lattice.marg.silent` | 60 | — | sound carries farther: doc 09's per-block sound attenuation × 7000 |
| `lattice.marg.trapped` | 80 | — | `trap_bp` × 20000 |
| `lattice.marg.unlit` | 80 | — | `bright_bp` × 5000; worldgen fixtures emit `fixture_level - 2`, so `protect_r` drops by 2 (Gothic Horror 4 → 2); safe rooms are still spawn-proofed (§11.3 step 3) |
| `lattice.marg.volatile` | 60 | UW | after stage `hazard`, `floor_div(rooms_total, 4)` more hazards, each one `weighted([1, 1])` (`flame_jet`, `oil_slick`) and one `range` over non-safe rooms with a free hazard socket (2 draws each) |

Examples (run `0x0123456789ABCDEF`, `bleed_on = 1`): Folio 26 Gothic Horror `[annotated]`; Folio 36 Mythic Fantasy `[annotated, champions]`; Folio 36 Unwritten `[dense, unlit]`. With `bleed_on = 0`, Folio 36 Mythic Fantasy draws `[annotated, dense]`.

## 23. Abridged and Prologue layouts

**Abridged** (`mode.abridged`, Canon §17.1, doc 14 §3.1, §8.2a) keeps every rule of this chapter and changes only these inputs and counts:

| `n` | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Volume | 1 | 1 | 1 | 2 | 2 | 2 | 3 | 3 | 3 | 4 | 4 | 4 | 5 | 5 | 5 |
| `pos` | 1 | 4 | 5 | 1 | 4 | 5 | 1 | 4 | 5 | 1 | 4 | 5 | 1 | 4 | 5 |
| `d` | 1 | 3 | 5 | 6 | 8 | 10 | 11 | 13 | 15 | 16 | 18 | 20 | 21 | 23 | 25 |

- Streams, sub-streams and `EntityId`s use `n`; every depth-driven value (`P`, `T`, `H`, `S`, Law and secret-exit thresholds, vein count) uses `d`.
- `folio_type`: `n = 1` is standard, `pos = 5` is the boss (Folios 3, 6, 9, 12, 15), and `pos = 4` Folios draw `[8500, 1500, 0]`, so they are standard or challenge and always carry the Interleaf. Folio 15 is the Ur-Text (§5.4 rule 7), and the exit draw at Folio 14 still makes its 3 unseen candidates (Canon §17.1 `LATTICE_CANDIDATES`).
- Room counts: `n_target = max(4, mul_bp(base, 9000))`, so Mythic Fantasy plans 14 to 18 Rooms instead of 16 to 20.
- Loot quantity per Folio × 16667 Bp (Canon §17.1): chests `mul_bp(4 + range(3), 16667)` = 6, 8 or 10, and 60 floor spots (15 per slot, doc 11 §9.6). Reliquaries, Fortune Engines and vendor stock are unchanged (Canon §17.1).
- Secret exits follow §17.4 with `d`: a pos-1 Folio and a boss Folio before Folio 15 can hold one (the next Folio is never a boss); a pos-4 Folio cannot.
- The lattice's Volume rule spans 3 Folios, the run cap of 3 appearances still applies, and at most 5 genres are ever excluded (§5.4). Abridged has no Marginalia.
- Expected time: 15 Folios at 240 to 300 s for doc 14's 60 to 75 minutes; the 9000 room scale keeps 90% of the drawn rooms, rounded down to whole rooms (18 → 16).

**Prologue** (`mode.prologue`, doc 14 §11): `generate_folio` returns the authored chain `room.prologue_l1` … `room.prologue_l7` with no draws and no Marginalia, events, secrets or bleed plan. Its zones are authored: L1 and L2 form a bleed zone of the chosen Origin's genre, which doc 14 passes as `prev_genre` (the Prologue has no earlier Folio), L5 and L6 a bleed zone of `genre.fairy_tale`, and L3, L4 and L7 stay `genre.unwritten`; these are the only bleed zones an Unwritten Folio ever holds. L3 holds a `block.node.timber` node, an `item.tool.pick`, 3 `block.ladder.wood` and 2 `block.bridge.plank` cat.placeable items, a 3,000 mm drop onto solid ground and a 2,000 mm gap (doc 14 §11, Canon §4.3). Note: mode.prologue is retired by DECISIONS.md item 132; the layout is preserved for reference and contextual tutorials.

## 24. Determinism, tests and owned statuses

### 24.1 Determinism rules

1. `generate_folio` reads only `GenInput` and `ContentDb` (with the run's content set); equal inputs give equal `blueprint_hash` on every platform (Canon §21 test 31). Party size, wall-clock time, thread count and floats never reach it; account state reaches it only as the `bleed_on` bit fixed at Commit (§14.1).
2. Every draw names its sub-stream (§7.2): stage streams `ftype`, `rubric`, `bleed`, `marg`, `secret_exit` (attempt 0 always), `graph`, `typing`, `zones`, `embed`, `liquid`, `node`, `hazard`, `secret`, `story`, `loot`, and per-item streams `wfc`, `dress`, `light`, `puzzle`, `arena`. Per-room and per-chunk work uses `item_rng`, so `par.map` order is irrelevant.
3. Lists are sorted before selection, ties to the lower index; no `HashMap` or `HashSet` (Canon §1.11).
4. Plan functions (`folio_type`, Rubric, `bleed_plan`, `marg_mods`, `secret_exit_plan`, and doc 13's aspect set) are recomputed, never cached; preview and generation share code.
5. Events, dread and complications use the sequential `rng.event` on the host only; loot contents use `rng.loot_place` and keyed `rng.loot` (doc 11); population uses `rng.spawn` (doc 13). Worldgen never draws from them.
6. Variable draw counts (WFC observations, shuffles, rejections) are declared per stage (§7.3), as Canon §3.4 requires.

### 24.2 CI

- **Unit vectors:** acceptance tests 1–2, 4–8, 15, 16, 29, 30, 36, 37 and 38 run as plain unit tests against the Python-checked values in this chapter.
- **PR sweep:** 50 seeds × 16 genres × 4 types (64 per seed, 3,200 in all) pass V1–V16; at least 3,184 (99.5%) pass by attempt 2, and fewer than 4 need the fallback.
- **Nightly:** 1,000 seeds × 16 × 4; the Steam Deck runner fails the build when p50 exceeds 110 ms, p99 260 ms or any single Folio 900 ms (doc 02 `perf.deck.gen.*`).
- **Golden hashes:** 32 `(GenInput, blueprint_hash)` pairs agree on the five targets of doc 02 §6.4: x86_64 Windows, x86_64 and aarch64 Linux, aarch64 macOS and wasm32-wasip1.

### 24.3 `st.contradicted` and `st.unmoored` (owner 12)

- **`st.contradicted`** is present exactly while an equipped component of the bearer has `C >= 3` (Canon §11.1), re-evaluated on each `ev.contradiction_changed`; UI only.
- **`st.unmoored`** (Canon §5.11): each Erratum keeps the ticks of its last 3 Contradiction failures (jams, fizzles, sparks, failed potions), the newest included, in doc 06's per-character counter: docs 05, 06, 07 and 08 report every failure through `note_contradiction_failure(t)`. A failure at tick `t` whose oldest kept failure is at `t - 600` or later applies the status for 300 ticks (refresh). While it lasts, Fortune-derived effects are off: no Death's Door roll (Canon §10.8 step 1) and `dd_used` unchanged, and doc 04's `avatar_of_fate` suspended; the bearer's own drops roll rarity with an effective `SER = 40` (Canon §13.4) and a Fortune Engine SER shift of 0 (doc 11); components at `C = 1` or `2` use the `C = 3` row (potency 7000, failure 500). It ends early on the tick the bearer's origin enters a `room.colophon` cell or a `zone.seam` cell. Example: failures at ticks 1000, 1300 and 1590 apply it at 1590 until 1890; a fourth at 1700 refreshes it to 2000.

## 25. Folio Expansion genres (DECISIONS 53)

Each Folio Expansion (doc 15 §1.1, `store.sku.folio_exp_<nn>`) adds one genre. The first two planned are **Age of Sail and Pirates** (expansion 01: ship decks, ports and sea caves, with deep water and boarding) and **Kaiju and Mecha** (expansion 02: ruined city blocks under giant creatures and machines). This section fixes how an expansion genre enters world generation; its rows below are written as `genre.<exp_genre_suffix>`.

1. **Canon first.** An expansion genre exists once the canon gains its §5.2 row (axioms, `aether_axiom`, gravity, ambient light, Lucre and Aether skins, tier `core`), its §5.9 identity and manufacturer rows and its required §5.10 Laws (Canon gaps G18). This chapter reserves no expansion ID before then.
2. **Record.** The expansion ships the genre's full 65-dimension GPV, which passes lints L1 to L10 with `drift_bp = 1000`, so the 1-in-10 bleed rate holds (§14.1); 1 to 3 Laws, each with a §4.1 discovery trigger; a primary and a secondary tileset; 6-entry hazard and event lists built from §18.1 and §19.1 archetypes (a new archetype joins this chapter's tables first); 4 story domains with doc 18 grammars for every §20.2 template; liquids, `node_w`, `cycle_w` and room bias; doc 07 vein weights, a doc 13 roster and native factions; and 3 Lost Chapter templates (§10.1) behind the expansion's own Script.
3. **Content set.** An expansion genre is a lattice node only in a run whose content set includes its pack (doc 14 §2.3); `content_hash` covers the content set, so co-op peers agree (Canon §2.4). Daily and Weekly runs use the baseline content set (doc 14 §8.1) and never include it.
4. **Lattice.** Lattice order appends expansion genres after `genre.ur_text` in expansion number order, and `CORE_IN_LATTICE_ORDER` lists the 12 base core genres followed by the installed expansion genres. The expansion supplies a symmetric edge of at least 10 to each base core genre and each earlier expansion genre (§5.2), its `secret_exits` chances (§5.3) and a return weight from each secret genre (10 unless it names another). The run cap, the Volume rule and freshness apply unchanged (§5.4); exclusions do not grow with the number of genres, so at least 3 genres stay eligible.
5. **Invariance.** With no expansion in the content set, every rule, draw and vector of this chapter is unchanged. Rules that count the 12 base core genres keep counting them: the Ur-Text hazard skins cycle `k % 12` over the base core genres (§18.2), and §5.6's secret-Origin mapping is untouched.
6. **No vehicles.** Ships and airships are Folio architecture and set dressing; mecha and kaiju are architecture, enemies and mini-bosses (doc 21 §2.5); no Erratum steers or pilots one (Canon §17.7). A kaiju-scale mini-boss fights in §21.2's `room.arena` (46 × 46 blocks inside, 2 storeys), so no expansion needs a larger box.

## Exports

- `genre.*`: the full 65-dimension GPV of all 16 genres (§2, §3), including the non-canon fields `fixture_level`, `fog`, `ambient_decic`, `liquid_viscosity_bp`, `gas_dissipation_bp`, `damage_affinity`, `hard_bp` (§2.3), which extend Canon §5.9 without changing it; `drift_bp` as the per-genre bleed chance (§3.5, §14.1); the rules by which a Folio Expansion genre joins generation (§25).
- `law.*`: the 30 World Laws of §4, among them every Law Canon §5.10 requires, each with its discovery trigger (§4.1).
- `zone.plain`, `zone.bleed`, `zone.seam`, `zone.dynamic` (§14).
- `lattice.node.margin`; `lattice.edge.<from_suffix>.<to_suffix>` (§5.2–§5.3); `lattice.folio.standard`, `lattice.folio.challenge`, `lattice.folio.rest`, `lattice.folio.boss` (§6.1); `lattice.challenge.gauntlet`, `lattice.challenge.siege`, `lattice.challenge.heist`, `lattice.challenge.duel`, `lattice.challenge.race` (§6.3); the 12 `lattice.marg.*` of §22.
- `gpv.layout.*` (16, §3.4); `gpv.evt.*` (21, §19.1); `gpv.story.*` (21, §20.2: 12 slot templates, 8 bridges, 1 erasure).
- `room.*`: the 41 kinds of §16.1 and `room.prologue_l1` … `room.prologue_l7`; tags `room.safe`, `room.sanctuary`, `room.stealth`, `room.no_bleed`, `room.no_spawn`, `room.electronic`, `room.restricted` (with its faction, §12); templates `room.tpl.*` including `room.tpl.fallback.<layout>`, `room.tpl.arena_<boss suffix>` and the Lost Chapter templates `room.tpl.lost_chapter_<genre_suffix>_hall`, `room.tpl.lost_chapter_<genre_suffix>_crossroads` and `room.tpl.lost_chapter_<genre_suffix>_gallery` (§10.1); the 25 rules `room.rule.*` of §8.2.
- `biome.<suffix>.<name>`: the 32 tilesets of §3.3, with their micro-voxel shapes and custom masks (§10.4).
- `hazard.*`: the 28 archetypes of §18.1, `hazard.chasm` and `hazard.gravity_flip` structural; the electronic-feature list of §18.3.
- `secret.room.*` (8) and `secret.exit.*` (4) (§17.2, §17.4); the 11 hiding methods with the `kind` index each carries in doc 03's secret-found event (§17.3): `secret.hide.cracked_wall` 0, `secret.hide.false_wall` 1, `secret.hide.lever` 2, `secret.hide.crate_stack` 3, `secret.hide.waterfall` 4, `secret.hide.ceiling_hatch` 5, `secret.hide.verb_lock` 6, `secret.hide.dream_veil` 7, `secret.hide.clearance` 8, `secret.hide.blank_page` 9, `secret.hide.palimpsest_wall` 10.
- Owned statuses' behavior: `st.contradicted`, `st.unmoored` (§24.3).
- Quest items placed by worldgen (the quest category is this chapter's, Canon §13.1): the Folio Leaf's placement and consumption, the story plaque and the optional Folio quest (§20.3, §20.5, §20.6); the story record that doc 14's `FolioRecord` keeps and doc 18's closing manuscript reads (§20.7).
- `GenInput` and `FolioBlueprint` (§7.1), including the newcomer bleed gate `bleed_on` (§14.1).

## Interface assumptions

- **I1 (doc 03):** worldgen calls doc 03 §10.3's light propagation for the light plan and the bake; doc 03 applies Canon §5.9 physics per local genre, the `flip` bit (`gsign`), relink portals, `protect`, `open_tag`, `ambient_decic`, block shapes and the per-chunk custom-mask tables (§10.4), and instantiates records in §7.1 order; dynamic zones (§14.3) are doc 03 state with at most 2 active. This chapter emits `ev.secret_found` (hiding-method index, §17.3) and `ev.objective_completed` (kind 0 Rubric, 1 challenge, 2 story) with doc 03 §14.3's payloads. Requested: doc 03 registers a law-discovery event with payload `{ law: Id32, slots: u8 }`, emitted once per Law per run on its first trigger (§4.1).
- **I2 (doc 02, doc 16):** `generate_folio` signature and `OrderedMap`; `FolioView.bleed` holds the entry zone plus at most 3 drift zones (§14.1 caps drift at 3); peers compare `blueprint_hash`, and a mismatch is a desync; budgets `perf.deck.gen.folio_*` (§7.3); the run header carries `bleed_on` (§14.1). Doc 02's citations of "doc 12 §6.3" and "§13.1" mean §7.3 and §14.1.
- **I3 (doc 07):** the Canon §5.9 materials; `block.masonry.<stone>` genre walls and floors, `block.light_fixture.<genre_suffix>` with `light_emit = fixture_level`, `block.planks`, `block.door.*`, `block.cracked_wall`, `block.glass_pane`, `block.torch`, the patron altar `block.altar` (never breaks), `block.rubble`, `block.pit_lid` and `block.hull_panel` (§16.1, §17.2, §18.1, §19.1); `block.tag.destructible` on exactly the worldgen blocks of §9.2; the 23 `block.node.*` as full-shape prop blocks that turn into unbreakable spent blocks, the §9.4 vein weights, and the vein count `6 + floor_div(d, 5)` that §11.2 places into node-socket strips (doc 07 §9.4's "the socket's cells"); mining time × GPV `hard_bp`; station tiers from `room_bias`.
- **I4 (doc 08):** the liquids of §3.5 (`liq.blood` included), `liq.wellspring` fonts with 1 to 3 uses, `gas.miasma`, `gas.steam`; harvest-node ingredients for the H and F classes by biome (mineral ingredients come from doc 07 veins); automata start from settled worldgen liquids; `liquid_viscosity_bp` clamps to `500..=10000` there.
- **I5 (doc 13):** budgets over `spawn_slots`, spawn cells and anchors; the aspect set as the first `rng.spawn` draws, recomputable for the preview (§5.7); `director.profile.rest` spawns nothing; the checkpoint squad; civic packs, territory, war fronts, rescue natives, hirelings; plea captives `npc.captive.<genre_suffix>` paying `fac.rep.free_captive` (§19.1); the Unwritten's roster `enemy.editors`; Law waves and the Nemesis; ambush waves on §12's ambush anchors; `genre_shift` (§14.3); mini-boss prefabs fit the 46 × 46-block `room.arena` interior, paid from the population budget; `lattice.marg.champions` doubles budget champions and adds one aspect.
- **I6 (doc 11):** fills chests, floor spots (9 per slot, 15 in Abridged), guarded spots, Incipit caches and Reliquary spots; puts vendors and Engines in the §6.2 sockets (none when `lattice.marg.sealed` removes the shop); omits Lucre lines in the Unwritten; secret tables `loot.secret.cache`, `loot.secret.vault`, `loot.secret.palimpsest`; `item.quest.folio_leaf` (doc 11 §3.5); `item.tool.torch` ground items in sigil puzzle rooms. Requested: a story-relic fill for §13's story relic spot (one item whose affix themes follow the story's 2 domains), which doc 11 does not define yet.
- **I7 (doc 09):** `lock.*` for Locked edges, verb seals and the clearance lock; disarming hidden hazards; camera alarms and `alarm_raised`; light bands (§11.3), fog and sound attenuation; `disguise.trigger.trespass` reads §12's `room.restricted` and its faction; per-Folio weather fog and sound attenuation.
- **I8 (docs 04, 05, 06, 10):** reading XP; `law_bound` techniques and the High Noon standoff term; altars, prayers within 3000 mm and favor; censored names; identification of the story relic; doc 06's `note_contradiction_failure(t)` as the one failure counter for dread and Unmoored (§4, §24.3).
- **I9 (doc 14):** calls `generate_folio` with §7.1 inputs at each Folio entry, with `bleed_on` fixed at Commit in the run header (§14.1); owns phases, Rubric completion, doors, the vote, the Colophon, Interleaf, Endpaper, the Index, discovered Laws and unlock sets; a rest Folio has no Rubric (§6.2); keeps `RunState.laws_triggered` and merges it into each account's discovered Laws at run end and on departure (§4.1); keeps the story record, `story_solved` and `leaf_carried` in `FolioRecord` (§20.7) and adds a solved or carried story to the Index (§20.5). Requested: in the Prologue, doc 14 passes the chosen Origin's genre as `prev_genre` (§23).
- **I10 (docs 17, 18):** the door panel shows every §5.7 field, with only the viewer's discovered Laws; the HUD Law list and Folio header omit undiscovered Laws (§4.1); the Folio quest appears under the Rubric (§20.6); the transition waits for the blueprint; doc 18 ships `nar.grammar.story.<template_suffix>.<genre_suffix>` for all 21 §20.2 templates, the story plaque's fixed line (§20.3), name grammars, `loc.*` skins for hazards and events, seams, palettes and music, and builds the closing manuscript from §20.7's records.
- **I11 (doc 15):** each Folio Expansion pack carries one genre with everything §25 lists; the Lost Chapter Scripts grant §10.1's templates through doc 14's `unlock.room.lost_chapter_<genre_suffix>`.

## Canon gaps

Every gap this chapter raised against canon-1.5 (G1 to G17) is closed by canon-1.11 or by a ruling of `audit/canon_gaps_triage.md`, and its text is removed: G1 (G-051), G2 (G-027), G3 (G-162), G4 (G-165), G5 (G-166), G6 (G-161), G7 (G-060), G8 (G-039), G9 (G-184), G10 (G-064, now §18.3), G11 (G-164), G12 (G-226), G13 (G-041), G14 (G-167), G15 (G-125), G16 (G-028) and G17 (G-036). Three gaps are open:

- **G18** Canon §5.2, §5.9 and §5.10 cover exactly 16 genres. A Folio Expansion genre (DECISIONS 53: Age of Sail and Pirates, then Kaiju and Mecha) needs canon rows before its GPV can be written: §5.2 axioms, `aether_axiom`, gravity, ambient light, skins and tier; a §5.4 Origin tolerance row if it can be an Origin; §5.9 identity and manufacturers; §5.10 required Laws. Assumed: each expansion ships a canon amendment with those rows, and §25 governs how the genre then joins generation.
- **G19** DECISIONS 99 (each Folio rolls one time of day from its genre's table and one weather state, which feed light, sound, surfaces and fire) and DECISIONS 100, 101 and 105 (Second Edition after the first Ur-Text clear: a rewritten lattice, 15 Folios in 3 Volumes, each Folio written in two fused genres with both rosters, Laws and skins and averaged axioms) name this chapter, but canon-1.11 has no time-of-day or weather tables and no fused-genre rule: Canon §5.8 gives every position one `local_genre`, and only a seam merges two genres' axioms, by maximum. Until the canon carries them this chapter generates neither, and no rule or vector here depends on them. Proposal: the canon adds per-genre time-of-day weights and weather states with their light, fog, sound-attenuation, surface and fire terms in Bp (Canon §5.9, §12; docs 03, 09 and 18 apply them) and a fused-Folio rule (both genres' rosters, Laws and skins, axioms `floor_div(a + b, 2)` per axis); this chapter then draws time of day and weather in plan stage 0 on their own `stage_rng` sub-streams at attempt 0, so previews show them and no existing draw moves, stores them in `FolioPlan` and doc 02's `FolioView`, and builds the Second Edition lattice and fused Folios on §14.2's seam machinery, with doc 14 owning the mode.
- **G20** Content sets at Early Access: DECISIONS.md item 114 supersedes item 41; all 12 core genres ship from Early Access, and strict lattice rules hold without relaxation (DECISIONS 109, 110). §5.4's feasibility bound assumes 12 core genres: with 6, a Volume's fifth Folio leaves 2 eligible genres and Abridged Folios 12 and 14 leave 1, so `lattice_candidates` would fail its assertion, and §5.6 has no Folio 1 for an Origin whose genre the build does not ship. Doc 21 §2.3 (a) and (b) propose, for canon-1.12, a relaxation cascade inside the eligibility filter (inert with 12 genres, with the 3 `range` calls unchanged) and a mapping to the shipped core genre with the highest §5.2 edge from the Origin, ties to lattice order. Rule F1 Origin fallback (DECISIONS 116): when the host's Origin genre is not shipped in the current build, Folio 1 uses the nearest shipped genre by lattice edge weight (ties resolved by ascending genre ID).

## Acceptance tests

`R = 0x0123456789ABCDEF`; abbreviations per §2.3. Every number below was recomputed with an independent implementation of Canon §3.

1. `stream_seed(R, "rng.lattice", 1) = 0xb93f5f6ccba93fac`; `lattice_candidates(R, 1, CP, [CP])`: totals 6,450,000 / 5,250,000 / 3,900,000, draws 1,106,450 / 3,525,875 / 3,019,215, result `[SO, HN, SP]`.
2. `stream_seed(R, "rng.lattice", 0) = 0xa487e0eae90fea26`; seeded Folio 1 candidates `[MF, HN, SP]`; Daily 2026-10-11 (`0xc6f2e12ddca712ab`) gives `[WW, CH, MF]`.
3. Exhaustive enumeration over `c` (capped genres) and position shows at most 9 excluded core genres for Standard `m <= 24`, 5 for Abridged and 4 for Marginalia; 10,000 random legal histories always yield 3 distinct candidates, never current, same-Volume, capped or secret, with exactly 3 `weighted` calls.
4. `gen_seed(R, 2, genre.space_opera) = 0x1c35d41d581f5eaa`; `stream_seed(R, "rng.event", 2) = 0x9ad6720d21d175fe`, first `range(10000)` 2892.
5. `folio_type`: Folio 2 SO standard (3860), HN rest (9855), SP rest (9596), MF standard (7853); Folio 3 CP rest (9633), MF siege challenge (8993); Folio 4 CP gauntlet challenge (9480), MF (774) and GH (4365) standard; Abridged GH Folios 1–15 are bosses exactly at 3, 6, 9, 12, 15.
6. Rubric at Folio 2: SO catchwords (5080), MF keeper (1931); the rest Folios HN and SP draw none. Chests at Folio 2 (stage `loot`, attempt 0): SO 4, HN 6, SP 4, MF 4; in Abridged the same draws give 6, 10, 6, 6.
7. `bleed_plan` with prev MF and `bleed_on = 1`: Folio 7 GH none (roll 8487), ON (MF, [MA, GH], none), CH none (3626 against 1300), RD none with no draw, GH with `bleed_add` 1 (MF, [CH, HN], none); Folio 2 GH none (2208); Folio 3 GH none (8697); Folio 8 CH (MF, [GH], none); Folio 11 WW (MF, [HN], seam HN); Folio 17 FT (MF, [GH], seam GH). With `bleed_on = 0` every plan is none and draws nothing. No input yields more than 3 drift genres.
8. Marginalia `k` at 26, 35, 36, 45, 46, 56, 100 = 1, 1, 2, 2, 3, 4, 4; with `bleed_on = 1` Folio 26 GH `[annotated]`, 36 MF `[annotated, champions]`, 36 UW `[dense, unlit]`; with `bleed_on = 0`, 36 MF `[annotated, dense]`.
9. Each preview of tests 1–2 equals its generated Folio's type, bleed plan, modifiers and aspect set.
10. `protect_r`: MF 13, GH 4, CH 4, SO 14, CP 12, PA 11, WW 12, HN 11, WX 11, SP 13, FT 10, MA 12, ON 9, UT 13; RD and UW none; flat-floor coverage of a fixture 2 blocks up: MF 265 cells, GH 13.
11. A 2 × 2 hall has 17 `spawn_slots` in SO, 14 in GH, 0 in a rest Folio, and 0 within 24,000 mm of the Incipit spawn.
12. Hazards: dart at `d = 10` 9,714 mHP (Erratum), 13,030 (NPC); crusher at `d = 15` 52,900; flame pulse at `d = 5` 7,432; at Folio 1 against AR 20 a crusher leaves 22,728 mHP and three darts 11,079.
13. Typing weights: CP server room 128, MF library 126, WX hall 480, RD garden 0, illumination vault 0 without its unlock; CP's 25 always-available kinds total 1,819.
14. L1–L10 pass for all 16 records; GH `spawn_block_max` 5 fails L2; dropping `law.wuxia.qinggong` fails L7; a 300-prototype tileset with symmetry `l` fails L8.
15. Dread (GH, an Erratum in combat on every 360-tick mark): 250 at `entry + 360`, 1000 at `entry + 1440`, the check at `entry + 1800` rolls `roll_bp(1250)`, the cap 2500 at `entry + 3600`, and 0 once the Colophon services open; a Hardboiled Noir Folio rolls a constant 500.
16. `st.unmoored`: failures at 1000, 1300, 1590 apply it until 1890; another at 1700 refreshes to 2000; entering a Colophon or seam cell ends it that tick; Death's Door is not marked used while it lasts.
17. Same `GenInput` after different histories, and 1-thread versus 4-thread `par`, give identical `blueprint_hash` (100 seeds).
18. The PR sweep of 3,200 Folios passes V1–V16, at least 3,184 by attempt 2, fewer than 4 with the fallback.
19. A key behind its own lock fails V2; a sealed cellar under a 5-block hard drop fails V4; an MF camera fails V8; lava 5 blocks from the Incipit spawn fails V6; an Interleaf at 3000 Bp fails V11 until repaired; a sigil room without torches fails V15.
20. Safe rooms carry `no_spawn`, and where `spawn_block_max <= 14` every feet cell's `light_block` exceeds it after the bake.
21. Shapes: a tileset prototype whose feet-cell support is a custom micro-voxel mask fails V12; a bookcase prototype with a custom mask in its shelf cells passes; a slab-stair step rises 500 mm, two 250 mm micro-voxel layers, under the 550 mm step-up.
22. An Oneiric flip room has clear height 4, doors 4 tall, no zoned neighbor, and deals no fall damage on entry or exit (4000 mm < 4640 mm).
23. After Oneiric at Folio 4, an Oneiric exit discovered at Folio 8 appends no door but keeps its cache; Folios 4, 9, 14, 19 and 24 plan no secret exit; MF Folio 8 plans Unwritten (7395, 680), HN Folio 7 Redaction (706), FT Folio 8 none; in a build that does not ship the Oneiric's Folios, MF Folio 8 tests only the Unwritten (7395 against 1000) and plans none.
24. A 360 s Folio runs 17 event checks and fires at most 6 events; SO Folio 2's first check fires (2892 < 3000); in GH Folio 2 the same value is the failed complication roll at `entry + 600`; `ambush_extra = 1` adds an ambush at `entry + 5400` outside the cap.
25. Collapsed WFC grids satisfy every face constraint; a tileset whose every observation contradicts ends in `fallback_fill` after exactly 3 resets, with no draws after the failure.
26. The nightly sweep holds p50 ≤ 110 ms, p99 ≤ 260 ms and max ≤ 900 ms on the Steam Deck runner.
27. The §3.6 record parses as JSON with no floats and has 65 keys; L2 compares its Block A and B fields to Canon §5.2 and §5.9.
28. No OneWay edge is vertical; every spike pit and collapsing-floor pit is 2 blocks deep and V4 passes with an Erratum dropped into it at VERVE 0.
29. Story vector: `R`, Folio 2, GH, attempt 0 gives hero seed `0x417db148d4394087`, rival `0x630efb60831bfd3c`, domains `[moon, blood]` and 7 events: apprentice, orphaned, pact, found_relic, slew_beast, cursed, scraped; every story makes exactly 8 draws before its retellings.
30. Mini-boss pillars with `p = 4`, `phase = 0` sit at offsets `(0, -12)`, `(-12, 0)`, `(0, 12)`, `(12, 0)`; Space Opera places 10.
31. Editor arena boxes: Proofreader 3 × 3 × 2, Censor 3 × 2 × 2, Continuity Editor 3 × 3 × 2, Abridger 3 × 3 × 3, Last Word 4 × 4 × 2.
32. Plates: an Erratum walking 8,000 mm twice reaches the third plate 224 ticks after the first, inside the 300-tick latch, so a solo player solves it.
33. Veins: `d = 12` places 8; a 16-room Post-Apocalypse Folio at `d = 12` places 16; harvest nodes number `4 + floor_div(rooms_total, 4)` and are 0 in the Unwritten.
34. Secret kinds: `secret.room.editors_desk` weighs 2 and 0 below `d = 6`; `secret.room.ink_well` 0 below `d = 11`; the verb seal wants a prayer in Mythic Fantasy and a spell in Cyberpunk.
35. A Folio-2 Mythic Fantasy campfire rest at 2 Erasure stacks heals 39,950 mHP of an 85-HP Erratum; a Scriptorium cure at `d = 10` costs 1,085 Lucre.
36. Secret-exit probability for a Standard Fairy Tale Folio at `d = 8` is 3250 Bp (Oneiric 2500, then Unwritten 1000 of the rest); the plan draws at most 2 values there.
37. Restricted rooms: in Hardboiled Noir the first room with `pop >= 2`, an armory, is `room.restricted` of the city police native faction, and a treasury whose lowest-`NodeId` territory neighbor is that armory carries the same faction; a hall is never restricted; an Unwritten Folio has no restricted room; stage `spawn` still makes no draws.
38. Bleed frequency and gate (DECISIONS 54): the 12 core `drift_bp` values sum to 12,000; run seeds `mix64(i)` for `i` in 1..=4000, Folios 1..=24 and the 12 core genres (1,152,000 plans, `bleed_on = 1`, no modifiers) bleed in exactly 114,723 (9.96%); with `bleed_on = 0` none bleeds and the bleed stage draws nothing; a Daily run bleeds whatever the host account's run count.
39. Law discovery (DECISIONS 56): in a Gothic Horror Folio with an Erratum `in_combat` from `entry + 300`, `law.gothic_horror.dread_escalation` enters `laws_triggered` at `entry + 360` with the slot mask of the Active Errata; a fresh account's door panel and HUD omit it before that tick and show it from that tick on, and its next run's previews show it from the account's discovered Laws (doc 14 §5.5); `law.gothic_horror.sanctified_ground`, never triggered, stays hidden; the run's `state_hash` values are identical for a fresh account and a veteran one; every Law of §4 has exactly one row in §4.1.
40. Folio quest: every Incipit holds the story plaque (event 0, praise, not overwritten, no draw), so test 29's story still makes exactly 8 draws before its retellings and §20.3's draw count is unchanged; reading the plaque and lore objects showing events 1 and 2 solves the story; a Leaf held by an Active Erratum on the exit-commit tick is consumed and sets `leaf_carried`; the Rubric, the Colophon and the exits open with the quest unfinished.
41. Ending fragments: a run whose Volume 2 has solved stories in Folios 7 and 9 has Folio 7's story retold in doc 18's Volume 2 section; an unsolved story contributes nothing; each `FolioRecord` story record equals the blueprint's after the Folio is regenerated from its `GenInput`.
42. Template choice: a room with one candidate template makes no template draw; a Lost Chapter template never appears without its unlock or outside its genre; with `unlock.room.lost_chapter_wuxia` in the content set, a Wuxia hall's candidates include the Wuxia Lost Chapter hall template, and the typing weights of test 13 are unchanged.
43. Resource nodes and micro-voxels: a vein of size 3 fills the first 3 cells of its room's lowest-index free node socket; no node block stands in a wall cell, so breaking every node of a Folio opens no wall; every chunk's custom-mask table lists each distinct mask once, in ascending first-use block index, and rasterizing twice gives identical chunks.
44. Expansion invariance: with no expansion in the content set, tests 1 to 43 give the same values; with a test pack that adds one core-tier genre with edge 50 to every base core genre, `lattice_candidates` returns 3 distinct eligible genres for 10,000 random legal histories, and Ur-Text hazard skins still cycle over the 12 base core genres.

## Invariants annex

```
Σ doc12: gpv ⊗ laws ⊗ lattice ⊗ ftype ⊗ graph ⊗ embed ⊗ wfc ⊗ light ⊗ zones ⊗ validate ⊗ story
W! generate_folio pure(GenInput, ContentDb)              N! party size|clock|threads|float → generation
W! account state → generation only as bleed_on           W! bleed ⇔ bleed_on ∧ bleed_ok ∧ roll_bp(drift_bp or forced 10000)
W! mean of the 12 core drift_bp = 1000                   W! Law effects from entry, discovered or not
N! undiscovered Law in preview|HUD                       W! one discovery trigger per Law; laws_triggered account-independent
W! GPV A == Canon §5.2 ∧ GPV B == Canon §5.9             W! GPV has 65 dims for all 16 genres
W! 1..=3 Laws per genre ⊇ Canon-required                 N! Law changes a canon constant (all 30)
W! Law active ⇔ local genre ∨ seam includes it           W! draw ∈ named sub-stream(rng.worldgen|rng.lattice|rng.event)
N! preview advances a live stream                        W! lattice: exactly 3 weighted calls per exit
W! candidates ∉ {cur, same-Volume, capped, secret}       W! depth 25 ⇒ ur_text      W! boss ⇔ pos 5
W! rest ⇒ no Rubric ∧ no spawns ∧ no events              W! Interleaf ∈ 4000..=6000 Bp (pos 4, standard|challenge)
W! every Folio: Incipit → … → RubricGate → Colophon      W! out has degree 1
W! key before lock ∧ no softlock ∧ Rubric reachable      W! V1..V16 before publish; attempts 0..=3 then fallback
W! terrain indestructible unless block.tag.destructible  N! digging terrain       N! vertical OneWay
W! node blocks fill socket strips, never wall cells      N! custom micro-voxel mask in a walkable cell
W! pit depth 2 ⇒ climbable at VERVE 0                    W! safe room ⇒ no_spawn ∧ (sbm ≤ 14 ⇒ lit above sbm)
W! spawn eligibility reads light_block only              N! spawn cell within 24,000 mm path of the Incipit
W! chests = 4 + range(3) (Abridged × 16667) per non-boss Folio
W! ≤ 1 seam per Folio, 1..=2 rooms                       W! drift ≤ 3 ∧ entry + drift ≤ 4 bleed slots
N! drawn bleed in redacted|unwritten|ur_text             N! electronic thing where local TEC ≤ 2
W! Prologue zones authored, never drawn
N! dep axis with local axiom 0                           W! hazard dmg := Canon §10.6 ∧ src_lvl = d
W! walkable cells use doc 03 shapes only                 W! secret genre ≤ 1 per run before Marginalia
W! room.restricted(F) ⇔ restricted kind ∧ F = (own territory, else lowest-NodeId territory neighbor)   N! restricted safe|secret room
N! secret exit in a secret-genre Folio ∨ before a boss ∨ before depth 25 ∨ to an unshipped genre
W! every puzzle solo-solvable with Folio contents        W! story: 8 draws, 5..=8 events, rationalized
W! iteration ascending; N! HashMap|HashSet               W! st.unmoored ends on Colophon or seam entry
W! story text cosmetic                                   N! LLM text → blueprint
W! every Incipit holds the story plaque                  N! Folio quest gates Rubric|Colophon|exit
N! Lost Chapter template without its unlock              N! expansion genre without canon rows
N! vehicle or piloted mecha                              W! no expansion in content set ⇒ every vector unchanged
```
