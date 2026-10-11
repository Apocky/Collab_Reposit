# 18 — Art, Audio, Narrative and the Narrator
As of 2026-10-11. Status: BINDING. Conforms to canon-1.2.

This chapter was also checked line by line against the on-disk canon, which is canon-1.4 (Canon §23); nothing here contradicts 1.3 or 1.4. It owns everything the player sees, hears and reads that the simulation never reads: art direction and the per-genre art bibles, rendering style techniques built on doc 02's pipeline, animation, music and sound, the text skins and registers of every genre, the lore, and the Narrator with its optional on-device Scribe model. Every record here is presentation. `palimpsest_sim` never depends on any of it (Canon §22: rendering reads the simulation and never feeds it; language-model text is cosmetic only).

## Imports

| Source | Sections | Consumed |
|---|---|---|
| Canon | §1.1, §1.6, §1.10 | ID grammar, integer-only content, text keys `loc.<id>.skin.<genre_suffix>.name` and name resolution order |
| Canon | §2.1, §2.3 | 60 Hz ticks, "N s" = N × 60 ticks, `in_combat` |
| Canon | §3.1–§3.3 | xoshiro256++, `mix64`, `stream_seed`; reused as a pure function for presentation-only keys (§1.3) |
| Canon | §4.1, §4.2 | body capsule (radius 300 mm, height 1800 mm, eye 1620 mm), walk 4300 mm/s |
| Canon | §5.2, §5.5, §5.6, §5.9, §5.11 | genre table and skins, the skin naming rule, Contradiction states, Anachronism, genre identity palettes and music registers, Going Native, `st.unmoored` |
| Canon | §6.4 rows 17, 31, 34 | action speed, dark-vision radius, footstep indicator radius |
| Canon | §9, §9.1 | body plans, baked pose tables, severing |
| Canon | §10.1 | the 15 damage types and their default colors |
| Canon | §11.1, §11.6 | statuses, Erasure |
| Canon | §12.2 | concealment, genre fog 1 per 8 m |
| Canon | §13.3, §13.6, §13.9 | rarity colors, unknown appearances, `illuminated` |
| Canon | §14, §16.5, §17.1, §17.6, §17.9 | currency display names, patrons, run constants, ghosts, invasions |
| Canon | §18, §19, §22 | event IDs; namespaces `art.`, `aud.`, `nar.`, `vfx.` and `loc.` text keys owned here; invariants |
| Brief | §1, §2, §5 | premise, archetype × skin, runtime language model with grammar-constrained JSON, seed cache and template fallback |
| DECISIONS | items 8, 13, 14, 23 | wry literary tone, 16+ gore, optional on-device model, per-genre manuscript UI skins |
| Doc 01 | §1.3, §9.4, §9.5 | fiction premise and fiction rules, rating targets, the tone guide, adopted here as the voice bible |
| Doc 02 | §2.2, §3.2, §7.2–§7.6, §8.1–§8.10, §9, §10, §11.4, §15 | crates (`firewheel`, `audionimbus`, `bevy_rapier3d`, `llama-cpp-2` 0.1.159), `crate.scribe`, view frame, entity mapping and the 16-ragdoll cap, event routing by `trigger`, meshing, 32 × 32 px block arrays, viewmodel cameras, the light agreement clamp, post order, particles, page-turn geometry, bleed blend, audio engine, five stems and five music levels, occlusion formula, Fluent, asset path rule, RAM and VRAM budgets, `flag.scribe.enabled` (default false) |
| Doc 03 | §4.5, §4.6, §5.15, §11, §14.2 | block `skins`, sound grid costs, `ev.noise`, `ev.footstep`, `ev.entity_landed`, `ev.explosion`, `ev.fire_started`, `ev.fire_out` payloads |
| Doc 05 | hit feedback table | hitstop and markers; audio cue mapping delegated here |
| Doc 06 | §2.2, §5.7, §16.1 | casting registers, glyph skin names in five genres, the 16 patron names, boons and taboos |
| Doc 09 | §2.2, §4.8 | light bands (dark ≤ 4, dim 5–9, bright ≥ 10), reveal meters |
| Doc 10 | §3 | appearance pools, sigil atlas, `loc.idc.*` keys |
| Doc 11 | §4, §5 | weapon skin names, manufacturers |
| Doc 12 | §2.3, §3, §4, §14, §18.2, §19.2, §20 | fog, `render_style`, `era_tl`, tilesets, story domains, World Laws, bleed and seam zones, hazard and event skin names, Folio story fragments |
| Doc 13 | §8, §9, §10.5, §13.6, §13.7 | rosters and roles, mini-bosses, the five Editors and their verses, Revisor hunters, Margin NPC IDs, dialogue schema, barks |
| Doc 14 | §3.2, §3.6, §4.3, §5.1, §6.3, §11 | Folio phases, exit vote, run end states, Margin facilities, Echo roster, Prologue |
| Doc 15 | §3.1, §7.1, AP-7 | cosmetic catalog, the request for readability limits, no casino presentation |
| R4 | §2, §4.2, §7.5, §8, §10 | graphics state, meshing, authoring-time LLM pipelines, SLM models and runtimes, grammar constraints, Steam disclosure, the two acoustic systems |
| R6 | §7, §9 blocks D–G, §11 R12–R13 | genre signal channels, GPV presentation fields, foreign-things-look-foreign, lexicons |

## 1. Scope, ownership and conventions

### 1.1 What this chapter owns

| Namespace | Holds |
|---|---|
| `art.` | `art.mesh.*` (glTF), `art.anim.*` (clip names equal to doc 03 pose-table IDs), `art.rig.*`, `art.blockarray.<genre_suffix>`, `art.block.*`, `art.tex.*`, `art.lut.<genre_suffix>`, `art.font.*`, `art.post.<genre_suffix>`, `art.palette.<genre_suffix>`, `art.light.<genre_suffix>`, `art.bible.<genre_suffix>`, `art.visual.*`, `art.vm.*`, `art.silhouette.*`, `art.readability.cosmetic` |
| `vfx.` | `vfx.dmg.<type>`, `vfx.status.*`, `vfx.aware.*`, `vfx.contra.*`, `vfx.sprites.<genre_suffix>`, `vfx.trans.*`, `vfx.telegraph.*`, `vfx.ragdoll` |
| `aud.` | `aud.music.*`, `aud.sfx.*`, `aud.amb.*`, `aud.vo.*`, `aud.set.*`, `aud.matclass.*`, `aud.bus.*`, `aud.mix.default` |
| `nar.` | `nar.lore.*`, `nar.author.<genre_suffix>`, `nar.editor.*`, `nar.margin.*`, `nar.narrator`, `nar.ending.*`, `nar.grammar.*`, `nar.names.<genre_suffix>`, `nar.register.<genre_suffix>`, `nar.job.*`, `nar.scribe.*`, `nar.review.*` |
| `loc.` text keys | every localized string (Canon §1.10, §19), stored in `assets/loc/<locale>/<owner_doc>.ftl` per doc 02 §10 |

### 1.2 Presentation boundary and encodings

- No crate in `palimpsest_sim`'s dependency closure may load a record of these namespaces. Doc 02's crate rules enforce it; acceptance test 19 checks it.
- Content stays integer (Canon §1.6). A presentation scalar the renderer uses as a float is authored in milli-units with the suffix `_milli` (`bloom_milli: 250` is 0.25). Lengths are mm, durations ticks, angles Bam, chances Bp.
- Colors appear as `#RRGGBB` in this chapter's tables and as decimal integers in JSON, as doc 12 §3.6 encodes palettes (`#3A1F2B` → `3809067`).
- `art.`, `aud.`, `vfx.` and `nar.` records are part of `content_hash` today (Canon §2.4). Doc 02 gap G1 proposes splitting them into a `presentation_hash`; this chapter supports that proposal (Canon gaps, item 6).

### 1.3 Presentation randomness

Presentation never draws from a simulation stream. Two pure functions supply every cosmetic or narrative choice:

```rust
/// Narrative draws: names, grammars, Scribe sampling seeds. Same inputs, same text, on every machine.
/// `job` is always a `nar.` content ID, so it can never equal a canon `rng.` stream name.
fn nar_rng(run_seed: u64, folio: u32, job: &str, key: u64) -> Rng {
    Rng::from_seed64(mix64(stream_seed(run_seed, job, folio as u64) ^ mix64(key)))
}
/// Client-local cosmetic variation: particle seeds, sound variants, idle fidgets.
fn pres_hash(tag: &str, a: u64, b: u64) -> u64 { mix64(fnv1a64(tag) ^ mix64(a ^ mix64(b))) }
```

`stream_seed` is the Canon §3.2 formula evaluated as a pure function; no stream state exists. Example: run seed `0x0123456789ABCDEF`, Folio 4, job `nar.job.story_retelling`, key 512 gives seed `0xae27e411c755d822`. Presentation draw counts may vary with content; each family below states its count. A replay reproduces narrative text exactly because every input is in the replay header and the run history.

## 2. Art direction: the living manuscript

### 2.1 Five rules

1. **Every Folio is an illustrated page.** Each genre is drawn in one illustrative tradition (§2.2). The player walks inside the illustration; the frame (paper grain, ink edges, page turns) never lets them forget it is a book.
2. **The palimpsest shows through.** Wherever text is scraped, older text shows. A broken destructible block reveals for 600 ticks a faint under-layer: the previous Folio's genre ink pattern at 15% opacity on the exposed faces. An Editor's erasure leaves red correction marks (§4.6) instead.
3. **Ink and paper are physical.** Outlines are ink (§4.1), shading sits on paper (§4.2), Folio changes turn the page (§4.3), and genre borders are stitched or bled (§4.4).
4. **Foreign things look foreign.** Errata, their gear and bleed-zone visitors render in their origin genre's style, outline and animation step (R6 R12; doc 02 §8.6). A Mythic Fantasy knight in a Cyberpunk arcology is a painted figure in a photographed street.
5. **Readability beats style.** Damage colors (Canon §10.1), rarity colors (Canon §13.3), telegraph hatching (§4.7) and awareness glyphs (§4.6) are global and never restyled per genre beyond the limits stated. Color is never the only signal.

### 2.2 Each genre as an illustration style

`render_style` is doc 12's (§3.1) and selects the doc 02 shader variant. The tradition column directs the art team; it names historical techniques, never a living artist.

| Genre | render_style | Illustrative tradition | Line | Shading |
|---|---|---|---|---|
| Mythic Fantasy | painterly | illuminated manuscript plates and oil-glazed adventure paintings | brown iron-gall ink, tapered | soft glazes, gilt highlights |
| Gothic Horror | painterly | Victorian mezzotint and penny-dreadful frontispiece | black, heavy at the silhouette | deep low-key, candle pools |
| Cosmic Horror | ink | pulp-magazine pen-and-ink with halftone | cross-hatched, nervous | hatching density follows darkness |
| Space Opera | pbr | airbrushed chrome paperback covers | thin, only on silhouettes | glossy, rim-lit |
| Cyberpunk | pbr | neon photobash and rain-slick cel overlays | thin cyan or magenta rim | wet speculars, emissive signage |
| Post-Apocalypse | pbr | photocopied zine collage over rusted photographs | rough, broken | dusty, desaturated, high grain |
| Weird West | toon | dime-novel woodcut covers | thick, chunky | 3 hard bands, halftone shadows |
| Hardboiled Noir | ink | chiaroscuro crime-comic panels | heavy black, spotted blacks | near-monochrome, one red |
| Wuxia | ink | shan shui ink-wash scrolls | wet brush, variable width | graded wash, paper shows through |
| Steampunk | pbr | engraved technical plates of exhibition catalogues | fine engraving lines | brass and soot, warm |
| Fairy Tale | toon | storybook watercolor and gouache | soft plum ink | 3 soft bands, watercolor blooms |
| Mythic Antiquity | painterly | red-figure and black-figure pottery and fresco | black figure outlines | flat fresco tones, sun-bleached |
| Oneiric | xerox_collage | cut-paper collage and photocopied dream journals | scissor-cut edges | posterized, mismatched scales |
| The Redaction | pixel_dither | bureaucratic photocopy and 1-bit office bitmap | none, dithered | 4 levels, ordered dither |
| The Unwritten | ink | unfinished pencil underdrawing on blank paper | grey graphite, sketchy | almost none; white page |
| The Ur-Text | painterly | gold-leaf illuminated initial pages | gold and black | rich glazes, gold leaf |

### 2.3 Voxel block style and texel density

| Asset class | Texel density | Texture size rule | Filtering |
|---|---|---|---|
| Blocks (doc 02 §8.3 arrays) | 32 px per m | 32 × 32 px per face layer | nearest magnification, linear-mip minification |
| Instanced block props, fixtures, doors, placeables | 64 px per m | power of two, ≤ 256 × 256 | same |
| Creatures and native NPCs | 128 px per m | ≤ 256 × 256 albedo plus ≤ 256 × 256 ink-control map | same |
| Errata, Echoes, mini-bosses, Editors | 128 px per m body, 256 px per m head and hands | ≤ 512 × 512 each map | same |
| Viewmodel items and arms | 512 px per m | ≤ 512 × 512 each map | same |

- Nearest magnification everywhere keeps texels crisp, so the voxel world and its creatures read as one pixel-crisp object set under each genre's post stack.
- Block faces are hand-painted at 32 × 32 px with a 1 px darker rim on the side layer so greedy-merged quads still read as individual 1 m blocks: the shader draws the rim from `fract(world_pos)`, not from the merged quad's UVs.
- The **ink-control map** (RGBA8, BC7) carries R hatch mask, G outline weight, B emissive mask, A roughness. Entities ship no normal maps; per-layer material parameters give blocks their roughness (doc 02 §8.3).
- Edges of the world box read as page edges: `block.void_bedrock` (Canon §1.8) renders as the paper's deckled edge in the genre's paper color.

### 2.4 Line hierarchy

| Line | Weight (× `outline_px`) | Color |
|---|---|---|
| Entity silhouette | 1.0 | genre ink, or origin genre ink for a foreign entity |
| Block silhouette against sky or void | 1.0 | genre ink |
| Entity interior creases (normal edges) | 0.5 | genre ink at 70% |
| Block-to-block edges inside a wall | 0 (none; the painted rim covers it) | — |
| Interactables (doors, levers, containers, resource nodes) | 1.0 plus a 1 px paper-color inner line | genre ink |
| Loot on the ground | 1.0 in the rarity color (Canon §13.3) | rarity color |

## 3. Per-genre art bibles

Each genre has five records: `art.palette.<g>`, `art.light.<g>`, `art.post.<g>`, `art.bible.<g>` and `art.lut.<g>` (a 32³ KTX2 grading LUT authored from the palette). The first three swatches of every palette are Canon §5.9's, copied exactly and in order; this chapter adds four.

### 3.1 Palettes (`art.palette.<genre_suffix>`)

`key`, `fill`, `accent` are canon. `ink` draws outlines and text. `paper` is the page tone used by paper grain (§4.2), deckled edges and UI paper (doc 17). `danger` fills enemy telegraphs (§4.7). `glow` tints fixtures, motes and pickups.

| Genre | key | fill | accent | ink | paper | danger | glow |
|---|---|---|---|---|---|---|---|
| mythic_fantasy | `#3B6E3B` | `#C9A227` | `#8FB3D9` | `#2B1D12` | `#EFE2BF` | `#C0392B` | `#FFD98A` |
| gothic_horror | `#3A1F2B` | `#7A0E14` | `#C8C3B4` | `#0F0A0C` | `#D8D0BC` | `#E23B2E` | `#F2C57A` |
| cosmic_horror | `#0E2A2E` | `#4F7F6A` | `#B8E986` | `#050F10` | `#C9D3C0` | `#F05A3C` | `#9CFF8A` |
| space_opera | `#0B1026` | `#3D7BFF` | `#FF8A3D` | `#05070F` | `#E6ECF5` | `#FF3B4F` | `#8FD4FF` |
| cyberpunk | `#120B1F` | `#FF2E88` | `#00E5FF` | `#07040D` | `#E9E4F2` | `#FF5A1F` | `#00E5FF` |
| post_apocalypse | `#6B4E2E` | `#C2A36B` | `#E04E1B` | `#231A10` | `#D9C9A3` | `#FF3A1A` | `#FFB347` |
| weird_west | `#8C4A2F` | `#E0B776` | `#2F3E4E` | `#2A1A12` | `#F1DFB8` | `#C8102E` | `#FFC56B` |
| hardboiled_noir | `#101010` | `#6E6E6E` | `#C22B2B` | `#000000` | `#E8E6E1` | `#C22B2B` | `#F2E2B0` |
| wuxia | `#1F3A2E` | `#D9C9A3` | `#B32020` | `#141414` | `#EFE6D0` | `#D42A1E` | `#FFE3A3` |
| steampunk | `#4A3A28` | `#B5793A` | `#5FA8A0` | `#22180F` | `#E8DCC0` | `#E0482C` | `#FFC77D` |
| fairy_tale | `#F2D7E8` | `#6FBF73` | `#E8C547` | `#3B2A4A` | `#FBF4E4` | `#D7304B` | `#FFF2A8` |
| mythic_antiquity | `#F0E6D2` | `#2E5A88` | `#C9792B` | `#1A1410` | `#E9D3B0` | `#B8321F` | `#FFE09A` |
| oneiric | `#2B1B4D` | `#8E7CFF` | `#FFC2E0` | `#120A26` | `#F3EEFF` | `#FF4F7A` | `#C9F1FF` |
| redacted | `#F2F2F2` | `#1A1A1A` | `#D7262E` | `#000000` | `#FFFFFF` | `#D7262E` | `#FFF7D6` |
| unwritten | `#FFFFFF` | `#E6E6E6` | `#CCCCCC` | `#8A8A8A` | `#FFFFFF` | `#D7262E` | `#FFFFFF` |
| ur_text | `#1C1408` | `#E8D9B0` | `#B8862B` | `#120C04` | `#E8D9B0` | `#E04A2A` | `#F6D77A` |

Global colors, never restyled: **Red Ink** `#D7262E` (Editors, Erasure, `liq.red_ink`); **Narrator pencil** `#6B6B6B` (marginal notes); damage colors (Canon §10.1); rarity colors (Canon §13.3).

**Telegraph contrast rule.** Let `cr(a, b)` be the WCAG 2.x contrast ratio. Each genre's telegraph edge color is whichever of `ink` and `paper` contrasts more with `danger`, and that ratio must be at least 4.0. The table passes with minimum 4.07 (wuxia, paper edge). Examples: mythic_fantasy 4.22 (paper), gothic_horror 4.58 (ink), cyberpunk 6.52 (ink), ur_text 4.80 (ink). The Ur-Text danger was moved from the vermilion `#C8341E` (3.78) to `#E04A2A` to pass.

### 3.2 Lighting rig and fog (`art.light.<genre_suffix>`)

The light field (doc 02 §8.5) carries simulation light levels, not colors. The renderer colors them: `irradiance = sky_tint × R + block_tint × G`, then applies the doc 02 agreement clamp, so color never changes how dark a cell reads. The key light is a directional sun or moon that casts shadows only in `open_tag` rooms (doc 12 §3.4); its contribution passes through the same clamp.

| Genre | sky_tint | block_tint | Key | Key K | Key elevation (Bam) | key_milli | fog (doc 12) | Fog or haze color | haze_start_mm | haze_max_milli |
|---|---|---|---|---|---|---|---|---|---|---|
| mythic_fantasy | `#E8EEFF` | `#FFC98A` | sun | 5600 | 10923 | 900 | 0 | `#B9CBD9` | 64000 | 300 |
| gothic_horror | `#8C93B8` | `#FFB45E` | moon | 4100 | 5461 | 350 | 1 | `#6E6A78` | — | — |
| cosmic_horror | `#6F9A8E` | `#B8E986` | moon | 7000 | 2731 | 200 | 1 | `#2C4A48` | — | — |
| space_opera | `#AFC4FF` | `#CFE6FF` | star | 6500 | 16384 | 600 | 0 | `#0B1026` | 96000 | 200 |
| cyberpunk | `#6A4C8C` | `#FF7AD0` | none | — | — | 0 | 0 | `#3A2550` | 32000 | 450 |
| post_apocalypse | `#F2D2A0` | `#FFB060` | sun | 4800 | 8192 | 1000 | 0 | `#C9A678` | 48000 | 450 |
| weird_west | `#FFE2B0` | `#FFC070` | sun | 3800 | 4096 | 950 | 0 | `#E8C08A` | 64000 | 350 |
| hardboiled_noir | `#9AA3AD` | `#FFE6A8` | moon | 4500 | 6372 | 300 | 1 | `#4A4F55` | — | — |
| wuxia | `#E6EFE8` | `#FFD08A` | sun | 6000 | 9102 | 800 | 0 | `#DCE6DE` | 32000 | 550 |
| steampunk | `#E8D9B8` | `#FFC27A` | sun | 4500 | 7282 | 700 | 0 | `#B8A88A` | 40000 | 450 |
| fairy_tale | `#FFF0F6` | `#FFE7A0` | sun | 5200 | 9102 | 850 | 1 | `#D8E8D0` | — | — |
| mythic_antiquity | `#FFF4DC` | `#FFB868` | sun | 5800 | 14564 | 1000 | 0 | `#F4E6C8` | 80000 | 250 |
| oneiric | `#B8A8FF` | `#FFC2E0` | moon | 9000 | 1820 | 500 | 1 | `#6E5CB8` | — | — |
| redacted | `#FFFFFF` | `#F4FAFF` | none | — | — | 0 | 0 | `#FFFFFF` | 0 | 0 |
| unwritten | `#FFFFFF` | `#FFFFFF` | none | — | — | 0 | 0 | `#FFFFFF` | 0 | 0 |
| ur_text | `#F6E7C0` | `#F6D77A` | sun | 5000 | 10923 | 800 | 0 | `#E8D9B0` | 96000 | 250 |

**Fog agreement.** Genre fog is concealment (Canon §12.2: 1 point per 8 m, each point × −667 Bp of sight detection). The visual fog matches it linearly:

```
fog_opacity_milli(d_mm) = min(1000, floor_div(fog_pts × 667 × d_mm, 80000))   // fog_pts = doc 12 fog, +1 during gpv.evt.weather_front
haze_milli(d_mm)        = min(haze_max_milli, floor_div(haze_max_milli × max(0, d_mm − haze_start_mm), 64000))   // genres with fog 0
```

Example: Gothic Horror at 48,000 mm gives `floor_div(1 × 667 × 48000, 80000) = 400` milli opacity, and the simulation's 6 fog points give a detection factor of `10000 − 6 × 667 = 5998` Bp: the visible fraction (600 milli) and the simulated one (5998 Bp) agree within one 8 m step. Haze in fog-0 genres is capped at 550 milli, so a distant enemy the simulation can see is always visible. Mythic Fantasy at 96,000 mm: `floor_div(300 × 32000, 64000) = 150` milli.

### 3.3 Post-process parameters (`art.post.<genre_suffix>`)

Fields follow doc 02 §8.6's order. All values are milli-units except `outline_px` (pixels at 800 lines, scaled by §4.1), `posterize` (levels, 0 = off), `dither` (0 or 1) and `anim_step` (1, 2 or 3). `sat` and `contrast` are 1000 at neutral. `transition` is the style used when this genre is the destination (§4.3).

| Genre | outline_px | bloom | sat | contrast | grain | paper | vignette | aberration | scanlines | halftone | posterize | dither | anim_step | transition |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| mythic_fantasy | 1 | 250 | 1050 | 1000 | 0 | 250 | 200 | 0 | 0 | 0 | 0 | 0 | 1 | page_turn |
| gothic_horror | 1 | 300 | 700 | 1150 | 150 | 350 | 450 | 0 | 0 | 0 | 0 | 0 | 1 | ink_bleed |
| cosmic_horror | 2 | 150 | 600 | 1200 | 250 | 450 | 500 | 150 | 0 | 200 | 0 | 0 | 2 | ink_bleed |
| space_opera | 1 | 500 | 1150 | 1050 | 0 | 100 | 150 | 100 | 0 | 0 | 0 | 0 | 1 | iris |
| cyberpunk | 1 | 650 | 1250 | 1150 | 100 | 100 | 250 | 250 | 150 | 0 | 0 | 0 | 1 | glitch |
| post_apocalypse | 1 | 200 | 800 | 1100 | 300 | 300 | 300 | 0 | 0 | 0 | 0 | 0 | 1 | film_burn |
| weird_west | 2 | 200 | 1000 | 1100 | 200 | 400 | 300 | 0 | 0 | 300 | 0 | 0 | 2 | iris |
| hardboiled_noir | 3 | 150 | 100 | 1350 | 350 | 200 | 550 | 0 | 0 | 0 | 0 | 0 | 2 | film_burn |
| wuxia | 2 | 250 | 750 | 1050 | 0 | 500 | 200 | 0 | 0 | 0 | 0 | 0 | 1 | ink_bleed |
| steampunk | 1 | 350 | 950 | 1050 | 150 | 300 | 300 | 0 | 0 | 0 | 0 | 0 | 1 | iris |
| fairy_tale | 2 | 400 | 1150 | 950 | 0 | 400 | 250 | 0 | 0 | 0 | 0 | 0 | 2 | page_turn |
| mythic_antiquity | 1 | 300 | 1000 | 1050 | 0 | 350 | 150 | 0 | 0 | 0 | 0 | 0 | 1 | page_turn |
| oneiric | 2 | 450 | 1100 | 1000 | 200 | 300 | 350 | 300 | 0 | 400 | 6 | 0 | 3 | ink_bleed |
| redacted | 0 | 100 | 300 | 1300 | 0 | 0 | 100 | 0 | 200 | 0 | 4 | 1 | 2 | glitch |
| unwritten | 2 | 0 | 0 | 900 | 0 | 600 | 0 | 0 | 0 | 0 | 0 | 0 | 3 | page_turn |
| ur_text | 2 | 400 | 1050 | 1050 | 0 | 450 | 300 | 0 | 0 | 0 | 0 | 0 | 1 | page_turn |

- **Selective color.** Hardboiled Noir sets `keep_hue: 0` with `keep_hue_width_bam: 2731` (±15°) and `keep_sat_milli: 1000`: pixels whose hue lies within 15° of the accent red keep full saturation; everything else uses `sat` 100. Every other genre sets `keep_hue_width_bam: 0`.
- **Comfort overrides** (doc 17): aberration, scanlines, vignette and grain go to 0, and `anim_step` is forced to 1 for every entity, when the player enables the comfort preset.
- The Redaction draws no ink outlines (`outline_px` 0): its 1-bit dither is the line.

### 3.4 Bible entries (`art.bible.<genre_suffix>`)

Tilesets are doc 12's `biome.<g>.*`; materials are Canon §5.9's. Silhouette notes name doc 13 roster templates and fix the shape each must read as at 30 m (§3.6).

| Genre | Block families | Signature props | Enemy silhouettes | VFX style |
|---|---|---|---|---|
| mythic_fantasy | `castle`: dressed limestone, oak beams, banner walls; `cavern`: wet granite with glowing ore | banners, braziers, weapon racks, dragon skulls, round tables | orc_raider hunched broad swarm; kingsguard_knight tower-shield rectangle; conclave_battlemage tall hat and raised glowing hand; cave_troll long-armed giant; wyvern span twice its height; kingsguard_lancer horizontal lance line | gilded sparks, ribbon trails, gold-leaf rune circles |
| gothic_horror | `manor`: damask plaster, black oak panels, cracked marble; `catacomb`: bone niches, wet brick | candelabra, portraits whose eyes follow, coffins, iron gates, gargoyles | rat_swarm low carpet; ghoul crouched long arms; vampire_courtier tall narrow cape; witch_hunter wide hat plus crossbow line; torch_villager pitchfork and flame; werewolf hunched mass; flesh_golem lumpy with bolts | ash, blood mist, candle smoke, moths |
| cosmic_horror | `sunken_town`: barnacled clapboard, kelp-slick stone; `abyss`: wrong-angled basalt, lit fungus | tide pools, chalk circles, specimen jars, dripping idols | polyp_swarm fluttering cloud; cultist hood with raised arms; deep_one fish-backed hunch; shoggoth_spawn eye-studded blob; expedition_rifleman pith helmet and long rifle; star_seer three-armed triangle; lurker_below hump in water | tendrils, wet-ink blooms, green witchlight motes |
| space_opera | `starship`: hull plate, deck grating, bulkhead strips; `station`: glass corridors, hydroponics | consoles, cargo pods, holo-tables, viewports | navy_trooper helmet and rifle; psi_adept robe floating 200 mm up; pirate_boarder jetpack hump; hive_drone low insect; hive_warrior crescent claws; security_droid boxy shield; hover_lancer speeder line | plasma bolts, lens flares, hexagonal shield ripples |
| cyberpunk | `megablock`: wet concrete, chain-link, invented-brand neon; `undercity`: pipes, graffiti, steam | vending kiosks, noodle stalls, server racks, holo-ads with fictional logos only | street_punk crest and knife; corp_enforcer visor and rifle; chrome_brawler oversized chrome arms; netrunner trailing cables; mantis_assassin arm blades; security_drone quad-rotor; riot_mech box on legs | glitch blocks, scanline holograms, neon sparks |
| post_apocalypse | `wasteland`: cracked asphalt, sand, rebar; `bunker`: rusted steel, rad-glass | burnt car hulks, drums, scrap barricades, warning signs | rad_roach low; raider_scav spiked shoulders; war_rig_rider motorbike; glow_zealot robe with glowing chest; bunker_sentry box; mutant_behemoth asymmetric giant; tunnel_mole low snout | rust flecks, sparks, oily smoke, slow rad motes |
| weird_west | `boomtown`: weathered planks, adobe, false fronts; `mine`: timber shoring, ore carts | hitching posts, wanted posters with invented names, troughs, gallows | dead_miner hat, lantern and dynamite belt; outlaw_gunhand bandana and duster; marshal_rifleman star and long rifle; hexslinger flat hat and charms; railroad_automaton smokestack head; sand_wyrm dune hump; posse_rider mounted lasso | dust puffs, powder smoke, burnt-orange hex sigils |
| hardboiled_noir | `city_street`: wet brick, fire escapes, tungsten lamps; `tenement`: peeling paper, venetian blinds | payphones, desks with lamps, blinds that stripe light, parked sedans | street_tough flat cap; mob_torpedo fedora and drum gun; leg_breaker wide coat and bat; crooked_cop peaked cap; shadow_tail thin trench coat; dock_heavy carrying a crate; getaway_driver sedan | rain splashes, cigarette curls, flashbulb pops, white-star muzzle flashes |
| wuxia | `mountain_sect`: stone terraces, red lacquer pillars, tile roofs; `bamboo_vale`: bamboo, mist, rope bridges | incense burners, bronze bells, scroll racks, stone lanterns | jiangshi stiff outstretched arms; sect_disciple flowing sleeves; iron_palm_elder wide stance; court_crossbowman box crossbow; lotus_assassin veil and fans; fox_spirit fan of tails; cavalry_general mounted halberd | brush strokes, petals, qi swirls |
| steampunk | `foundry`: riveted iron, brick kilns, catwalks; `airship`: varnished decks, brass rails, canvas | boilers, pressure gauges, gear trains, tesla coils | clockwork_scarab domed beetle; brass_automaton barrel torso; sky_aeronaut goggles and grapnel; galvanist coil backpack; luddite_wrecker sledge; steam_juggernaut furnace body; sewer_gator low and long | steam puffs, brass sparks, gears in every explosion |
| fairy_tale | `deep_wood`: mossy giant trunks, toadstools; `cottage`: thatch, gingerbread trim, hearth stone | spinning wheels, mirrors, wells, sweets fences | pixie_swarm glitter cloud; big_bad_wolf upright snout; bridge_troll stooped giant; hedge_witch pointed hat; woodcutter axe over shoulder; thorn_knight bramble armor; wild_hunt_rider antlered rider | glitter, petals, star sparkles, watercolor blooms |
| mythic_antiquity | `temple`: marble columns, bronze doors, mosaics; `labyrinth`: limestone, carved meanders | amphorae, braziers, statues, altars, laurel trees | maenad wand and flying hair; hoplite round shield and spear; shade_warrior translucent; harpy wings and talons; oracle_priestess veil and tripod; bronze_bull horns; centaur_archer horse body and bow | golden motes, laurel leaves, red-figure flame shapes |
| oneiric | `dreamscape`: impossible stairs, cloud blocks, giant furniture; `nursery`: oversized toys, wallpaper skies | doors in midair, handless clocks, beds, music boxes | sleepwalker arms out; nightmare_mare black horse, burning mane; stair_mimic drawn as a stair block; faceless_teacher blank face and third arm; gravity_kite diamond kite; lullaby_choir robed cluster | paper cut-outs, floating letters, particles that fly back to their source |
| redacted | `bureau`: grey cubicles, ceiling panels, carpet tile; `archive`: cabinets, shelving | water coolers, shredders, rubber stamps, ceiling domes | clerk tie and clipboard; compliance_officer briefcase; blackbar_sentinel a black rectangle; shredder_unit box with teeth; auditor long lens; faceless_agent blurred face | black bars, pixel noise, wordless stamp marks |
| unwritten | `blank_page`: white blocks with a pencil grid; `margin`: ruled lines | unfinished pencil outlines of furniture | Editor squads only (§8.3) | eraser crumbs, pencil hatching, red ink |
| ur_text | `palimpsest`: blocks layered in strata of every genre's texture; `colophon`: gold-leaf lecterns | giant quills, inkwells, illuminated letters as blocks | stray_letters swarm of glyphs; first_draft wears the last visited genre's soldier; palimpsest_beast patchwork; glossator robed scribe; lettered_serpent of letters; illuminator gilded; archetype_knight | gold-leaf flakes, illuminated initials, counter-rotating rings of letters |

**Editors in any genre.** An Editor or Red Ink servant takes its mesh skin from the local genre (doc 13 §9.1) and always adds the red overlay: silhouette outline in Red Ink at 1.5 × `outline_px`, struck-through handwriting decals on its body, and `ink` render style regardless of the local genre. A correction is never part of the page it corrects.

### 3.5 Typography (`art.font.*`)

Every font is SIL Open Font License 1.1 or Apache 2.0, verified against its license file at import (§10.3). Numerals use tabular figures. Body text never drops below doc 02's 12 px at 800 lines.

| Genre | Headings | Body | Numerals | Initial capitals |
|---|---|---|---|---|
| mythic_fantasy | Cinzel | EB Garamond | EB Garamond | gold-leaf versal, 3 lines |
| gothic_horror | UnifrakturCook | IM Fell English | IM Fell English | black-letter with blood-red fill |
| cosmic_horror | Special Elite | Libre Baskerville | Special Elite | none; typed headers |
| space_opera | Orbitron | Exo 2 | Orbitron | none |
| cyberpunk | Bebas Neue | Rajdhani | Share Tech Mono | none |
| post_apocalypse | Black Ops One | Courier Prime | Courier Prime | stencil |
| weird_west | Rye | Old Standard TT | Old Standard TT | woodcut slab |
| hardboiled_noir | Playfair Display | Cutive Mono | Cutive Mono | none |
| wuxia | Ma Shan Zheng | Noto Serif and Noto Serif SC | Noto Serif | brush seal stamp |
| steampunk | Limelight | Libre Caslon Text | Libre Caslon Text | engraved roundel |
| fairy_tale | Berkshire Swash | Lora | Lora | vine-wrapped versal |
| mythic_antiquity | Marcellus | Gentium Book Plus | Marcellus | meander border |
| oneiric | Caveat | Nunito | Nunito | cut-out letter |
| redacted | Space Mono | Inter | Space Mono | none |
| unwritten | Cormorant Garamond | Cormorant Garamond | Cormorant Garamond | none |
| ur_text | Cinzel Decorative | EB Garamond | EB Garamond | full illuminated initial |

Every locale also ships Noto Sans CJK and Noto Serif CJK for Chinese, Japanese and Korean, and Atkinson Hyperlegible as the legibility font that doc 17's accessibility setting substitutes for every body face.

### 3.6 Silhouette roles (`art.silhouette.<role>`)

Every enemy reads its doc 13 role from its idle-pose LOD0 bounding box and its weapon line, measured by the asset lint (§10.2) in mm:

| Role | Rule |
|---|---|
| swarmer | height ≤ 900, or a swarm volume |
| skirmisher, stalker | height 1300–1900; stalker idle crouch ≤ 1400 |
| soldier, shooter, sniper | a weapon line ≥ 600 (soldier, shooter) or ≥ 1000 (sniper) held at chest height |
| brute | height ≥ 2200 and width ≥ 0.6 × height |
| guardian | a frontal shield or plate ≥ 900 × 1200 |
| caster, controller, support, summoner | hands raised to shoulder height or above in the cast pose, with an emissive mask on the hands or focus |
| flier | span ≥ 2 × body height |
| cavalry | mounted |
| sapper, artillery | a carried charge or launcher ≥ 400 in its largest dimension |

### 3.7 Example records

```json
[
  { "id": "art.palette.gothic_horror", "swatches": [3809067, 7998996, 13157300, 985612, 14209212, 14826286, 15910266] },
  { "id": "art.light.gothic_horror", "sky_tint": 9212856, "block_tint": 16757854, "key": 2, "key_kelvin": 4100,
    "key_elev_bam": 5461, "key_azimuth_bam": 40960, "key_milli": 350, "fog_color": 7236216,
    "haze_start_mm": 0, "haze_max_milli": 0, "shadow_soft_milli": 600 },
  { "id": "art.post.gothic_horror", "lut": "art.lut.gothic_horror", "outline_px": 1, "bloom_milli": 300,
    "sat_milli": 700, "contrast_milli": 1150, "grain_milli": 150, "paper_milli": 350, "vignette_milli": 450,
    "aberration_milli": 0, "scanline_milli": 0, "halftone_milli": 0, "posterize": 0, "dither": 0,
    "keep_hue_bam": 0, "keep_hue_width_bam": 0, "keep_sat_milli": 0, "anim_step": 1, "transition": "vfx.trans.ink_bleed" },
  { "id": "art.bible.gothic_horror", "genre": "genre.gothic_horror", "render_style": 1,
    "palette": "art.palette.gothic_horror", "light": "art.light.gothic_horror", "post": "art.post.gothic_horror",
    "blockarray": "art.blockarray.gothic_horror", "sprites": "vfx.sprites.gothic_horror",
    "fonts": ["art.font.unifraktur_cook", "art.font.im_fell_english", "art.font.im_fell_english"],
    "music": "aud.music.gothic_horror", "register": "nar.register.gothic_horror", "names": "nar.names.gothic_horror" }
]
```

`key` encodes 0 none, 1 sun, 2 moon, 3 star field. Validation: the first three palette swatches equal Canon §5.9 (acceptance test 1), and `render_style` equals doc 12 §3.1.

## 4. Rendering techniques

All passes slot into doc 02 §8.1's frame order and §8.6's post order. Shaders use floats; their parameters come from integer content.

### 4.1 Ink edges

Edges come from the Sobel pass on depth and normals (doc 02 §8.6):

```
depth_edge  = |Sobel(linear_depth)| / depth_center ≥ depth_thr_milli / 1000        // default 80
normal_edge = 1 − dot(n_center, n_mean_of_8) ≥ normal_thr_milli / 1000             // default 250
width_px    = max(1, round_half_even(outline_px × H, 800))   for outline_px > 0     // H = render height
ink         = edge_owner.foreign ? origin_genre.ink : local_genre.ink
weight      = silhouette 1.0, interior crease 0.5 (§2.4), scaled by the G channel of the ink-control map
```

A per-pixel object-ID buffer (16 bits, entity `id32` low bits) marks silhouettes, so a foreign entity's outline uses its own genre's ink and width. Example: `outline_px` 2 at 1440 lines gives `round_half_even(2880, 800) = 4` px; at 800 lines, 2 px.

**Contradiction on items** (`vfx.contra.*`, Canon §5.5): Strained (C 1–2) draws a small ink blot decal on the held item; Contradicted (C 3–5) adds fluttering glyph particles, 4 per second, in the origin genre's ink; Disconnected (C 6) flickers the item between its skin and the torn-page placeholder at 4 Hz (R6 R12). With doc 17's flicker-off setting it shows the static torn-page icon instead. A jam, fizzle or spark plays `vfx.contra.failure`: the item's outline smears for 12 ticks.

### 4.2 Paper grain

A tiling 512 × 512 paper texture `art.tex.paper.<genre_suffix>` (fibers, deckle, foxing) is sampled in screen space at `H / 800` texels per pixel and multiplies the image after grading:

```
c' = c × (1 − paper_milli / 1000 × (1 − p))          // p ∈ [0, 1] is the paper sample
```

Example: `paper_milli` 350 and `p` 0.8 give `1 − 0.35 × 0.2 = 0.93`. Film grain (`grain_milli`) is separate: per-frame noise with amplitude `grain_milli / 1000 × 0.1` in linear light, seeded by `pres_hash("art.grain", frame, 0)`.

### 4.3 Page-turn transition

Doc 02 §8.8 owns the curl geometry over 90 ticks. This chapter adds the look:

- **Front face**: the old Folio's final frame, darkened on the curl by `1 − 0.35 × sin θ` (θ from doc 02's cylinder mapping).
- **Back face**: the destination genre's paper texture, with the old frame mirrored at 12% opacity as ink bleed-through.
- **Shadow** on the new page under the curl: `0.25 × (1 − (u − c) / 0.3)` for `u ∈ [c, c + 0.3]`.
- **Hold loop**: while loading holds at `e(t) = 0.5`, the destination genre's display name writes itself in the destination heading font at 12 characters per second, then the Folio intro (§8.7) under it.

`vfx.trans.*` styles reuse the pass with other functions over the same 90 ticks: `ink_bleed` (ink spreads from the screen center by a noise threshold falling from 1 to 0), `glitch` (16-row band displacement with RGB split, posterized to 4 levels), `iris` (a circle shrinking to the screen center, then growing), `film_burn` (a burn hole from a random corner chosen by `pres_hash("vfx.trans", folio_index, 0) % 4`). The destination genre's `transition` (§3.3) chooses the style.

### 4.4 Bleed-zone blend

Doc 02 §8.9 blends block albedo and draws the ragged ink line on the boundary. This chapter adds:

- **Post, light and fog blend.** The camera samples the bleed field at its own position to get weights `t_0..t_4`. Every numeric `art.post`, `art.light` and fog field renders as `Σ t_k × value_k`; LUTs blend by the same weights. The displayed vector follows the target with a 250 ms time constant (`α = 1 − exp(−dt / 0.25)`), so a step across a border fades in about one second.
- **Seam zones** render in the Folio genre's skin (Canon §5.8). Along the faces between a seam zone and its neighbors, a stitch decal runs on the wall and floor: stitches 60 mm long and 12 mm wide every 250 mm, in the seam genre's ink, so the player sees two pages sewn together.
- **Foreign sprites.** Damage and status VFX inside a bleed zone use that zone's genre sprites (§4.5).

### 4.5 Damage-type VFX language (`vfx.dmg.<type>`)

Every damage type owns one shape motif, its Canon §10.1 default color and a unique icon (doc 17 shows the icon, so color is never the only signal). Genres swap only the sprite set (`vfx.sprites.<genre_suffix>`: a sword-slash arc is a brush stroke in Wuxia and a scanline tear in Cyberpunk); color, motif, count, timing and icon never change. Emitters use doc 02 §8.7 integer fields; counts are per hit at a full 10000 Bp share and scale linearly with the type's share, minimum 1.

| Type | Motif | Color | Count | Life (ticks) | Speed mm/s | Spread Bam | gravity_bp | drag_bp | Size mm | Decal | Icon |
|---|---|---|---|---|---|---|---|---|---|---|---|
| `dmg.slash` | crescent streak | `#C9CED6` | 18 | 12 | 6000 | 4096 | 0 | 2000 | 40 | thin cut line | crescent blade |
| `dmg.pierce` | forward puncture cone | `#E8E2C8` | 10 | 10 | 9000 | 1024 | 2000 | 1000 | 24 | round hole | arrowhead |
| `dmg.blunt` | ground ring and chunks | `#A68A64` | 14 | 18 | 3000 | 16384 | 10000 | 3000 | 60 | crack star | hammer |
| `dmg.fire` | rising tongues and embers | `#FF6A1A` | 24 | 40 | 1500 | 8192 | −3000 | 1500 | 80 | scorch | flame |
| `dmg.frost` | shards and mist | `#9BE7FF` | 16 | 30 | 2500 | 16384 | 5000 | 2500 | 50 | rime | snowflake |
| `dmg.shock` | forked arcs | `#FFE14D` | 8 | 8 | 12000 | 32768 | 0 | 0 | 30 | none | zigzag |
| `dmg.acid` | droplets and bubbles | `#9AE13B` | 20 | 36 | 2000 | 12288 | 10000 | 1000 | 40 | pitting | bubbling drop |
| `dmg.radiation` | slow motes | `#2F6BFF` | 12 | 60 | 500 | 32768 | 0 | 500 | 30 | faint glow | three-blade fan |
| `dmg.sonic` | 3 expanding rings | `#C8B6FF` | 3 | 20 | 10000 | 0 | 0 | 0 | 200 | none | wave arcs |
| `dmg.toxin` | drifting clouds | `#6B8E23` | 10 | 48 | 800 | 32768 | −500 | 3000 | 120 | stain | vial drop |
| `dmg.radiant` | rays and glare | `#FFF4C2` | 12 | 16 | 7000 | 32768 | 0 | 0 | 40 | none | sun |
| `dmg.umbral` | inward wisps | `#5B2A86` | 16 | 30 | −2000 | 32768 | 0 | 2000 | 70 | shadow stain | eclipse |
| `dmg.psychic` | kaleidoscope glyphs | `#FF5FD2` | 8 | 24 | 1500 | 32768 | 0 | 1000 | 60 | none | spiral eye |
| `dmg.arcane` | circles and triangles | `#3FE0D0` | 10 | 24 | 2000 | 32768 | 0 | 1500 | 50 | sigil burn | circled triangle |
| `dmg.true` | hairline ink stroke and paper tear | `#FFFFFF` | 6 | 12 | 4000 | 2048 | 0 | 0 | 20 | none | full stop |

```json
{ "id": "vfx.dmg.fire", "trigger": { "event": "ev.damage_dealt", "dominant_type": "dmg.fire" },
  "color": 16738842, "count": 24, "life_ticks": 40, "speed_mm_s": 1500, "spread_bam": 8192,
  "gravity_bp": -3000, "drag_bp": 1500, "size_mm": 80, "decal": "art.tex.decal.scorch",
  "icon": "art.tex.icon.dmg_fire", "sprite_slot": 3, "sound": "aud.sfx.imp.fire" }
```

The `trigger` follows doc 02 §7.6. Status VFX (`vfx.status.*`) use the manuscript's own typography over the bearer: `st.stunned` orbiting asterisks, `st.sleeping` a rising ellipsis, `st.confused` an interrobang, `st.charmed` a rose fleuron, `st.feared` a trembling outline, `st.marked` a manicule above the head plus doc 17's through-wall outline, `st.dread` an ink vignette on the bearer's screen, `st.inspired` a gold underline at the feet, `st.unmoored` desaturation with fraying paper edges on the body, `st.erasure` one struck-through line crawling over the outline per stack (the outline turns Red Ink at the bearer's hunter threshold), `st.madness` hallucinations drawn in Narrator pencil. Body emitters show `st.burning`, `st.frozen` (an ice shell), `st.shocked` (an arc every 30 active ticks), `st.bleeding` (drips) and `st.poisoned` (green breath).

### 4.6 Stealth readability

The rule: **nothing on screen may tell the player they are hidden, lit, heard or unseen when the simulation says otherwise.**

1. **Light.** Doc 02's agreement clamp makes visual darkness equal simulation darkness. Fixture and emissive colors never add luminance past the clamp.
2. **Fog.** Visual fog equals simulated concealment (§3.2).
3. **Dark vision.** Inside the local Erratum's dark-vision radius (Canon §6.4 row 31, 4 m at SGT 40) the scene gets a desaturated blue-grey luminance floor of `ambient_milli(4) = 22` from doc 02's table. It changes only the local player's image and never makes a cell look lit.
4. **Awareness glyphs** (`vfx.aware.*`) draw above an observer that holds a non-unaware pair toward a party member and that the local player can see: `suspicious` a pencil "?" filling bottom-up with the detection meter, `investigating` an amber "?", `searching` an orange "?" with a rotating sweep, `alert` a Red Ink "!", `returning` a pencil ellipsis. Glyphs never draw through walls unless the observer bears `st.marked`.
5. **Stealthy and invisible NPCs** (doc 09 §4.8) render as a refraction shimmer until revealed to the local Erratum, then as the full model with a manicule marker.
6. **Sound ripples** (doc 17 accessibility option): each `ev.noise` the local slot heard draws a ring at its apparent position (doc 02 §9) with radius `min(3000, 50 × L_lis)` mm for 30 ticks. Unheard noises draw nothing.
7. **Footsteps.** Hostile footsteps inside the footstep-indicator radius (Canon §6.4 row 34) play their sound even when the voice cap is full (§6.6).

### 4.7 Telegraphs and cosmetic readability limits

- **Telegraphs** (doc 13 `ViewTelegraph`): the shape fills with the local `danger` swatch at 45% opacity under 45° hatching 6 px apart in the telegraph edge color (§3.1), with a 3 px edge line that pulses at 4 Hz in the final 30 ticks before `end_tick`. Hatch angle and pulse are global, so a colorblind player reads timing from the pattern.
- **Cosmetic readability limits** (`art.readability.cosmetic`, requested by doc 15 §7.1) are checked by the asset lint on every `cosm.` mesh, material and effect:

| Rule | Limit |
|---|---|
| Body envelope | every vertex within the body capsule (Canon §4.1) inflated by 150 mm; back items up to 400 mm behind; nothing more than 200 mm above the head |
| Role mimicry | no cosmetic adds a shield plate, weapon line or raised-hand emissive that matches a §3.6 role rule |
| Red Ink and danger | no more than 20% of a cosmetic's albedo area within ΔE 10 of Red Ink or any genre's `danger` swatch; no hatch pattern |
| Emissive | ≤ 150 milli luminance and ≤ 5% of surface area |
| Trails and auras | ≤ 20 particles per second; off while crouched and while the wearer has `st.hidden` |
| Sound | cosmetics emit no sound into any channel that `ev.noise` uses; footstep audio comes only from materials |
| Inkwell reveals | at most 180 ticks, skippable after 30, no coin sounds, reels or near-miss frames (doc 15 AP-7) |

## 5. Animation

### 5.1 First-person viewmodel

- The viewmodel is a separate camera layer (doc 02 §8.4) and always animates on 1s, whatever the genre's `anim_step`, because aim feel outranks style.
- **Action sync.** Each action clip has an authored length `clip_ticks`. It plays at rate `clip_ticks / action_ticks`, where `action_ticks` is the simulation's duration (Canon §6.4 row 17 for reload, swap and use). Example: a 120-tick reload with SLT 60 gives `mult = 10000 + 20 × 50 = 11000` and `action_ticks = max(1, ceil_div(120 × 10000, 11000)) = 110`, so the clip plays at rate 120 / 110 ≈ 1.091 and ends on the tick the simulation finishes.
- **Motion** from `art.vm.<archetype>` (example `art.vm.ranged.sidearm`: `bob_v_mm` 6, `bob_h_mm` 10, `sway_max_mm` 25, `sway_return_ms` 120). Bob amplitude scales with `speed / 4300`, and its low point lands on each `ev.footstep` of the local player, so steps and bob agree. Sway lags look input by at most `sway_max_mm`, then springs back. Recoil kick and hitstop are doc 05's.
- **Full-body awareness**: doc 02 §8.4 renders the avatar body headless; looking down shows the legs, held off-hand items and the belt quickbar.

### 5.2 Procedural creature animation

Skeletal clips are chosen by the simulation's `(anim, anim_tick)` (Canon §9): the clip named by the `art.anim.*` ID plays at `anim_tick`. Procedural layers add on top, client-side only:

| Body plan | Layer |
|---|---|
| humanoid, quadruped, avian, construct | foot IK: a downward ray from each hip, up to 1.5 × leg length, on the client's `ChunkView` blocks; two-bone analytic IK; pelvis drops by the lowest foot's offset, at most 0.4 × leg length; quadruped body pitch = `atan2(front_feet_y − rear_feet_y, body_length)` clamped to ±20° |
| insectoid | tripod gait: legs {1, 4, 5} and {2, 3, 6} alternate; a foot steps when it is farther than 0.35 × reach from its rest point; a step lasts 8 ticks at walk |
| serpentine | 24 spine segments follow a buffer of root positions sampled every 100 mm of travel |
| amorphous | blend-shape wobble driven by a spring on velocity (frequency 3 Hz, damping 0.4) |
| swarm | 24 to 64 cosmetic boids within 1500 mm of the swarm entity's origin; count = `24 + floor_div(40 × hp_bp, 10000)`; boid seeds `pres_hash("art.swarm", entity_id, i)` |
| every plan | head look-at toward the AI target or nearest Erratum within ±60° yaw and ±30° pitch, at most 240° per second; a 12-tick additive flinch along each `ev.damage_dealt` direction |

**Readability guard.** No procedural layer may move a visible body part more than 150 mm from the pose-table capsule that the simulation uses for hits; the IK solver clamps to it. Players aim at what they see, and what they see is within 150 mm of what the simulation hits.

### 5.3 Animation step

A genre's `anim_step` (§3.3) holds skeletal poses on 2s or 3s: `t_disp = anim_tick − (anim_tick mod step)`. Root motion and position still interpolate every frame (doc 02 §7.3). Examples: `anim_tick` 37 shows pose 36 on step 2 and pose 36 on step 3; `anim_tick` 38 shows 38 on step 2 and 36 on step 3. A foreign entity uses its origin genre's step. Hitboxes always use the true `anim_tick`.

### 5.4 Render-only ragdolls (`vfx.ragdoll`)

- On `ev.entity_killed` the skinned body becomes a `bevy_rapier3d` ragdoll (doc 02: at most 16, oldest removed). Bodies per bone take the skinned pose and each bone's velocity from the last two frames, plus a kill impulse along the killing hit's direction:

```
dv_mm_s = min(8000, 2000 + floor_div(6000 × prev_hp_bp, 10000))     // prev_hp_bp = ViewEntity.hp_bp on the previous frame
explosion: dv_mm_s += floor_div(6000 × max(0, r_mm − d_mm), r_mm)    // radial, from ev.explosion
```

  Examples: a target at 3000 Bp before the killing hit flies at 3800 mm/s; one killed from full HP at 8000 mm/s.
- A ragdoll lives 1800 ticks or until doc 02 despawns its corpse, sleeps when at rest, collides only with the client's block colliders and other ragdolls, and never writes anywhere the simulation reads.
- **Severing** (Canon §9.1): the part detaches as its own rigid body cut at the joint, with an ink-cap decal on both cut faces. The `cat.body_part` item keeps its own visual record.
- **Gore modes** (doc 17 setting): `full` (blood decals and mist in deep red `#8A1010`), `ink` (blood renders as the genre's ink color), `minimal` (no decals; severing still shows the ink-cap).

## 6. Audio

### 6.1 Music system (`aud.music.<genre_suffix>`)

Each genre has one track of five sample-aligned looping stems, `bed`, `pulse`, `tension`, `combat`, `apex`, selected by doc 02 §9's music levels 0 to 4. Files are `aud.music.<g>.<stem>` (Opus, 128 kbit/s stereo, 48 kHz). Bars are whole ticks so level changes land on simulation-time bar lines:

```
bar_ticks   = beats_per_bar × 3600 / bpm        // lint: must divide exactly
bar_samples = bar_ticks × 800                   // 48 kHz / 60 Hz
loop_bars   = 64 if bar_ticks ≤ 160 else 32
```

| Track | bpm | Meter | bar_ticks | Mode, tonic | bed | pulse | tension | combat | apex |
|---|---|---|---|---|---|---|---|---|---|
| mythic_fantasy | 90 | 4/4 | 160 | dorian, D | low strings, horn pad | frame drum | choir "oo" | brass ostinato | choir and timpani |
| gothic_horror | 72 | 3/4 | 150 | harmonic minor, C | organ drone | harpsichord arpeggio | string-quartet tremolo | organ and timpani | full choir and organ |
| cosmic_horror | 60 | 5/4 | 300 | locrian, B | sub drones | heartbeat bass drum | detuned strings | theremin, brass clusters | choir cluster |
| space_opera | 120 | 4/4 | 120 | lydian, E | synth pad, strings | synth arpeggio | brass swells | orchestral percussion, brass | choir, full orchestra |
| cyberpunk | 100 | 4/4 | 144 | phrygian, F♯ | analog pad | arpeggiated bass | industrial noise | drum machine, distorted bass | lead synth |
| post_apocalypse | 96 | 4/4 | 150 | minor pentatonic, E | feedback guitar drone | junk percussion | bowed metal | distorted riff, kit | full band |
| weird_west | 72 | 6/8 | 100 | mixolydian, A | slide guitar | shaker, hoofbeat | whistle, bowed saw | banjo, snare, trumpet | choir "ah", full band |
| hardboiled_noir | 75 | 4/4 | 192 | blues minor, B♭ | rain piano | walking bass | muted trumpet | big-band stabs | full brass |
| wuxia | 90 | 4/4 | 160 | gong pentatonic, G | guzheng | woodblock | erhu | taiko, dizi | full ensemble |
| steampunk | 144 | 4/4 | 100 | mixolydian, B♭ | low brass chorale | clockwork ticks | calliope | brass-band march | brass and organ |
| fairy_tale | 135 | 3/4 | 80 | lydian, F | celesta | pizzicato waltz | woodwinds in minor | orchestral waltz | choir |
| mythic_antiquity | 80 | 4/4 | 180 | phrygian, E | lyre | frame drum | aulos | chant, war drums | full chorus |
| oneiric | 72 | 3/4 | 150 | whole-tone, C | reversed piano | music box | choir pads | reversed percussion | forward and reversed together |
| redacted | 120 | 4/4 | 120 | atonal sine clusters | office hum at 60 Hz | typewriter and clock | sine clusters | shredder rhythm, bass | fax noise, choir |
| unwritten | 60 | 4/4 | 240 | none | room tone | pencil scratching | one sine | eraser percussion | page tearing |
| ur_text | 90 | 4/4 | 160 | dorian, D | organ and choir drone | quote | quote | quote | full counterpoint |
| margin (hub) | 60 | 3/4 | 180 | major, F | quill-and-paper foley, harp | none | none | none | none |

- `silence_ratio_bp` gates the bed in quiet genres: cosmic_horror 3000, hardboiled_noir 2000, unwritten 9000, others 0; the bed drops out for that share of 8-bar phrases, chosen by `pres_hash("aud.silence", folio_index, phrase)`.
- **Ur-Text counterpoint** (Canon §5.9 "every register in counterpoint"): 12 quote stems `aud.music.ur_text.quote.<core_genre_suffix>` re-arrange each core genre's motif at 90 bpm in D dorian. Every 8 bars the Ur-Text's `pulse`, `tension` and `combat` stems play the quote of the genre at `history[k mod n]`, with `k` the phrase index and `history` the run's Folio genres 1 to 24 in order, secret genres skipped.
- **Editor overlays** `aud.music.editor.<boss_suffix>`: a 4-note Red Ink motif (scratch, scratch, stamp, rest) that plays every 4 bars at level 4 in Editor Folios, transposed to the local track's tonic. The Last Word plays all four earlier motifs in its four phases.
- **Stingers** `aud.music.sting.*` play on the next beat: `folio_enter`, `colophon_open`, `editor_phase`, `player_downed`, `run_victory`, `run_erased`.
- Level timing (doc 02 §9): a rise applies at the next bar line; a fall applies at a bar line once the lower level has held 8 s. Example: Gothic Horror with the track started at Folio entry tick 0 and a rise computed on tick 1000 applies at tick 1050 (bar 7 starts at 7 × 150).

### 6.2 Bleed crossfades

Each track keeps its own clock: position = `(t − folio_entry_tick) mod loop_ticks` of that track, so a returning genre resumes in place. Crossing into a bleed zone fades the bleed track in and the Folio track out over 2 bars of the incoming track with equal-power gains `g_in = sin(πx/2)`, `g_out = cos(πx/2)`, `x` from 0 to 1. At `x = 0.5` both are 0.7071 (−3.01 dB). Inside a seam zone both tracks hold at 0.7071 at the same level. Example: stepping into a Weird West bleed from Gothic Horror fades over 2 × 100 = 200 ticks.

### 6.3 SFX taxonomy

IDs: `aud.sfx.<family>.<subject>` with variants as numbered files; sets for skins as `aud.set.<archetype_suffix>.<genre_suffix>` (a firearm archetype's set is a crossbow twang in Fairy Tale where doc 11's skin is a crossbow).

| Family | Subject | Trigger |
|---|---|---|
| `wpn` | fire, mech, reload, swap, jam, dry, charge, overheat | doc 05 and doc 06 action events via doc 02 §7.6 |
| `imp` | damage type × material class | `ev.damage_dealt` (dominant type), projectile hits |
| `foot` | gait × material class | `ev.footstep`, `ev.entity_landed` |
| `body` | effort, pain, death, per voice set | `ev.damage_dealt`, `ev.entity_killed` |
| `mag` | cast, release, fizzle per register (doc 06 §2.2) | `ev.spell_cast` |
| `env` | liquid, gas, fire, block break and place, door, explosion | `ev.liquid_reaction`, `ev.fire_started`, `ev.fire_out`, `ev.block_changed`, `ev.explosion` |
| `status` | apply, expire per status | `ev.status_applied`, `ev.status_expired` |
| `ui` | page, quill, seal, ledger | doc 17 widgets |
| `amb` | beds per tileset (`aud.amb.<tileset>`) | Folio and zone |

Material classes `aud.matclass.<class>` map every `mat.*` ID to one of 10 classes: stone, wood, metal, earth, sand, water, flesh, glass, cloth, ice. Unmapped materials fail the lint. Variant choice: `pres_hash("aud.var", tick, entity) % n`, re-drawn once by adding 1 to the tick input if it equals the last variant played on that emitter. Each play varies pitch by `±pitch_var_cents` (default 50) and gain by `±gain_var_cdb` (default 15, i.e. 1.5 dB).

### 6.4 Occlusion from the simulation sound grid

Doc 02 §9 owns the formulas. Every sound with an `ev.noise` plays only for party slots whose `heard[slot]` is set, at the heard level, low-passed by the wall loss `extra` and placed at the apparent position, so the player hears exactly what the simulation says a listener there hears. Doc 02's example stands: a 130 dB shot 10 cells away behind a closed oak door arrives at 95 dB, filtered at 8,434 Hz, 4 m away toward the doorway. `noise.*` IDs (doc 09) map to sound records through `trigger.noise`. Sounds without an `ev.noise` use doc 02's presentation estimate and never mislead about stealth: they are quieter than any `ev.noise` of the same source class by at least 6 dB.

### 6.5 Voice-over plan

| Speaker group | Lines | Voiced | Notes |
|---|---|---|---|
| The five Editors | 700 (140 each) | yes | intros, phase lines, one line per verse per attack, kills, deaths |
| Revisor hunters | 80 | yes | one actor; per-genre `voice_fx` |
| Margin NPCs (8) | 1,200 (150 each) | yes | milestone-gated lines (§8.4) |
| The Narrator | 600 | yes | authored marginal notes; Scribe notes are text only (§9) |
| Echoes (24) | 960 (40 each) | yes | short lines and barks |
| Genre mini-bosses (14) | 420 (30 each) | yes | |
| Native NPC efforts | 1,280 (2 voice sets × 40 per genre) | non-verbal | spoken barks are text bubbles (`loc.npc.bark.*`) |
| Errata efforts | 240 (4 voice types × 60) | non-verbal | grunts, breaths, pain, death |
| **Total** | **5,480** | | about 110 MB at 2.5 s mean, 64 kbit/s mono Opus |

- Recording: 48 kHz, 24-bit, mono; delivered at −20 LUFS per line ±2, true peak ≤ −3 dBTP.
- Launch VO is English; every locale has subtitles for every voiced line.
- Per-genre `voice_fx` on Editors, hunters and barks: gothic_horror gramophone, space_opera and cyberpunk comm radio, mythic_antiquity choir-double, redacted glitch, unwritten whisper; none elsewhere.
- No synthetic or cloned human voice ships (§10.3). Voiced lines are never rewritten by the Scribe.

### 6.6 Mix budget (`aud.mix.default`)

| Bus | Gain dB | Voices (Deck / desktop) | Priority 0–7 | Ducking |
|---|---|---|---|---|
| `aud.bus.voice` | 0 | 4 / 6 | 7 | ducks music −6 dB (attack 100 ms, release 600 ms) |
| `aud.bus.sfx` stealth-critical (footsteps, callouts, doors, alarms, reloads) | 0 | 16 / 32 | 7 | never stolen |
| `aud.bus.sfx` combat and impacts | −2 | 24 / 56 | 5 | telegraph sounds duck music −3 dB |
| `aud.bus.ambience` | −10 | 6 / 10 | 2 | |
| `aud.bus.music` | −8 | 10 / 20 | 6 (streamed) | |
| `aud.bus.ui` | −6 | 4 / 4 | 4 | |
| **Total** | | **64 / 128** (doc 02 caps) | | |

- Loudness: −18 LUFS integrated ±1 and true peak ≤ −1 dBTP over the 10-minute reference capture `bot_typical` (doc 02).
- Stealing: a new voice steals the quietest voice of strictly lower priority; priority 7 voices are never stolen; when only priority 7 voices remain, the new voice of priority below 7 is dropped.
- **Memory** (doc 02's 256 MiB): pre-decoded hot set of short and stealth-critical one-shots 48 MiB (i16 PCM, 524 s), decoded LRU cache 64 MiB, compressed Opus bank for the current Folio's genres 48 MiB, music ring buffers 4 MiB (10 stems × 2 s stereo i16 = 3.66 MiB), VO buffers 2 MiB, Steam Audio HRTF and reflections 48 MiB, mixer graph 16 MiB, headroom 26 MiB.

## 7. Text skins, registers and the name review

### 7.1 The skin naming rule and its review (`nar.review.banlist`)

Canon §5.2 forbids a Lucre or Aether skin that matches a real-world brand, a third-party game's currency or resource name, the display name of any `cur.` or `ent.` entry, or the word Ink. This chapter applies the same review to every text skin it authors (damage types, glyph forms, hazards, events, names, manufacturer slogans, doc 10 implant stamps and sigils, invented signage). A name **fails** when any test holds, compared case-insensitively on whole words with a trailing plural "s" ignored:

1. It equals the display name of a `cur.` or `ent.` entry (Lucre, Quires, Ink, Vellum, Quill), or contains the standalone word "Ink". Exemption: a currency's identity skin, the skin equal to its own base name (Unwritten "Lucre"; Canon gaps, item 3).
2. It contains an entry of banlist group `brand`: trademarks, companies, products and real persons' names, screened against the trademark search export the legal reviewer attaches to each build.
3. It contains an entry of group `game_coinage`, a coinage that exists primarily as one third-party game's currency or resource: V-Bucks, Robux, Primogems, Glimmer, Bright Dust, Eddies, Nuyen, Septims, Zenny, Gil, Poké Dollars, Simoleons, Bottle Caps, Riot Points, Blood Echoes.
4. It equals an entry of group `game_premium`, the name of a purchasable premium currency in a third-party game, even when it is also a common word: Crowns, Silver, Platinum, Gems, Diamonds, Crystals, Coins, Points, Tokens.
5. It contains casino lexicon (doc 01 §9.5): jackpot, spin, bet, wager, slot, lucky streak, payout.

Common words and real or historical coin names pass otherwise. The lint `pal lint names` runs tests 1, 3, 4 and 5 automatically on every `loc.` string tagged as a name; test 2 is a human legal review recorded in the asset manifest (§10.3).

**Review of the 32 canon skins** (Canon §5.2): every Aether skin passes. Lucre skins pass except **Crowns** (mythic_fantasy), which fails test 4: "Crowns" is the purchasable currency of The Elder Scrolls Online [RECALLED]. This chapter proposes **Nobles** (the medieval English gold noble) and keeps "Crowns" until the canon decides (Canon gaps, item 2). "Lucre" for the Unwritten passes by the identity exemption; "Credits" and "Motes" pass as common words that are earned, not premium, currencies elsewhere.

### 7.2 Damage-type text skins

Text keys `loc.dmg.<type_suffix>.skin.<genre_suffix>.name` (Canon §1.10). The archetype default names are Slash, Pierce, Blunt, Fire, Frost, Shock, Acid, Radiation, Sonic, Toxin, Radiant, Umbral, Psychic, Arcane, True. Within one genre all 15 names are distinct. Steampunk's shock is "Galvanic" (Canon §10.1).

| Type | mythic_fantasy | gothic_horror | cosmic_horror | space_opera | cyberpunk | post_apocalypse | weird_west | hardboiled_noir |
|---|---|---|---|---|---|---|---|---|
| slash | Cleave | Rend | Flay | Vibro | Mono | Hack | Cut | Razor |
| pierce | Pierce | Impale | Puncture | Kinetic | Ballistic | Shot | Lead | Slug |
| blunt | Crush | Bludgeon | Concussion | Impact | Trauma | Bash | Bruise | Sap |
| fire | Flame | Pyre | Black Flame | Plasma | Thermal | Burn | Brimstone | Torch |
| frost | Rime | Grave-Chill | Void Cold | Cryo | Cryo | Freeze | Norther | Ice |
| shock | Lightning | Storm | Crawling Spark | Ion | Surge | Juice | Lightning | Live Wire |
| acid | Corrosion | Vitriol | Dissolution | Corrosive | Chemical | Acid | Lye | Battery Acid |
| radiation | Blight | Pestilence | Star-Sickness | Rad | Rad | Glow | Bad Glow | Hot |
| sonic | Thunder | Shriek | Piping | Sonic | Subsonic | Rattle | Thunderclap | Bang |
| toxin | Venom | Poison | Spore | Bio-Toxin | Nanotoxin | Rot | Snakebite | Poison |
| radiant | Holy | Sanctity | Unbearable Light | Photon | Flash | Flare | Gospel | Spotlight |
| umbral | Shadow | Unholy | Outer Dark | Dark Matter | Blackout | Grim | Hex | Shadow |
| psychic | Mind | Dread | Madness | Psionic | Neural | Shakes | Spook | Nerves |
| arcane | Arcane | Occult | Eldritch | Warp | Raw Data | Weird | Conjure | Uncanny |
| true | Doom | Fate | Inevitability | Absolute | Flatline | Last Call | Reckoning | Curtains |

| Type | wuxia | steampunk | fairy_tale | mythic_antiquity | oneiric | redacted | unwritten | ur_text |
|---|---|---|---|---|---|---|---|---|
| slash | Blade | Shear | Thorn | Sword | Paper Cut | Cut | Line | The Edge |
| pierce | Needle | Rivet | Spindle | Spear | Pin | Puncture | Point | The Point |
| blunt | Palm | Piston | Cudgel | Ram | Falling | Impact | Weight | The Weight |
| fire | Crimson Flame | Firebox | Hearth-Fire | Forge-Fire | Fever | Incineration | Heat | The Flame |
| frost | Cold Yin | Refrigerant | Snowbite | Northwind | Cold Sweat | Cold Storage | Cold | The Frost |
| shock | Heaven Lightning | Galvanic | Storm | Thunderbolt | Startle | Discharge | Spark | The Spark |
| acid | Rotting Dew | Vitriol | Gall | Hydra Bile | Melting | Solvent | Blot | The Rot |
| radiation | Fallen Star | Radium | Witchlight | Star-Fall | Glow | Contamination | Bleach | The Glow |
| sonic | Lion Roar | Resonance | Bell-Toll | War-Cry | Scream | Feedback | Noise | The Voice |
| toxin | Five Poisons | Arsenic | Poison Apple | Venom | Sick Feeling | Compound | Stain | The Bane |
| radiant | Golden Sun | Limelight | Dawn | Divine | Waking Light | Disclosure | White | The Light |
| umbral | Yin Night | Soot | Nightshade | Underworld | Deep Sleep | Redaction | Gap | The Dark |
| psychic | Heart Demon | Mesmeric | Enchantment | Mania | Nightmare | Reeducation | Doubt | The Mind |
| arcane | Pure Qi | Aetheric | Fey | Mystery | Lucid | Anomaly | Maybe | The Word |
| true | Heaven's Will | Entropy | Fate | Moira | Waking | Termination | Blank | The End |

### 7.3 Genre text registers (`nar.register.<genre_suffix>`)

Doc 01 §9.5 is the voice bible, with its sample lines. Each register record adds measurable limits that the grammar lint and the Scribe filter (§9.8) enforce. Profanity is 0 (none) or 1 (mild only), following doc 01 Q-14's default.

| Genre | Formality 0–4 | Archaism 0–4 | Profanity | Mean words per sentence | Person | Prefer | Never |
|---|---|---|---|---|---|---|---|
| mythic_fantasy | 3 | 3 | 0 | 14 | third | oath, realm, hearth, wyrm, road | okay, tech, cool |
| gothic_horror | 4 | 2 | 0 | 16 | third, polite second | sir, madam, vigil, grave, dreadful | okay, guys, awesome |
| cosmic_horror | 3 | 1 | 0 | 12 | first, as a journal | specimen, angle, measured, tide | monster, lol |
| space_opera | 2 | 0 | 0 | 10 | second, to the crew | hull, stars, sector, all hands | magic, thee |
| cyberpunk | 1 | 0 | 1 | 7 | second | chrome, corpo, signal, flatline | thee, verily |
| post_apocalypse | 1 | 0 | 1 | 8 | first plural | scrap, haul, clean water, before | archaic forms |
| weird_west | 1 | 1 | 1 | 8 | third | reckon, iron, trail, sundown | technical jargon |
| hardboiled_noir | 2 | 0 | 1 | 9 | first | rain, case, angle, dough | dame, magic words |
| wuxia | 3 | 2 | 0 | 10 | third | sect, rivers and lakes, junior brother, heaven | okay, cool |
| steampunk | 3 | 2 | 0 | 15 | third | splendid, apparatus, patent, boiler | computer |
| fairy_tale | 2 | 2 | 0 | 12 | third, as a teller | once, thrice, little, bargain | gore words, technical terms |
| mythic_antiquity | 4 | 3 | 0 | 14 | second, as an invocation | epithets, muse, bronze, fate | modern idioms |
| oneiric | 2 | 0 | 0 | 9 | second | always, again, somehow, stairs | definitely |
| redacted | 4 | 0 | 0 | 11 | passive third | pursuant, clearance, compliance, proceed | emotion words, personal names |
| unwritten | 0 | 0 | 0 | 1 | none | single words | sentences |
| ur_text | 3 | 3 | 0 | 12 | every register | word, first, beginning, page | none |

Every register also obeys: no gendered pronouns for procedural names (repeat the name or use "they"); no numbers, statistics or rules in flavor (§8.6); Editors' lines are style-guide rulings in any genre (doc 01 §1.3 rule 4).

### 7.4 Name grammars (`nar.names.<genre_suffix>`)

Doc 12 §20.1 draws `hero_name` and `rival_name` as `u64` name seeds. A name is generated with exactly 4 draws on `Rng::from_seed64(name_seed)`: `pattern = weighted([3, 1])`, `given = range(6)`, `family = range(6)`, `title = range(3)`; pattern 0 is `p0`, pattern 1 is `p1`. Titles are gender-neutral or pair with a family name. The Ur-Text first draws `range(12)` to pick a core genre's table, then the same 4 draws (5 in all).

| Genre | Given | Family or epithet | Titles | p0 / p1 |
|---|---|---|---|---|
| mythic_fantasy | Aldren, Brisa, Corwen, Elowen, Garreth, Ysmay | of Hollowmere, Stormwright, the Unbowed, Ashborne, of the Nine Wells, Greymantle | Ser, Warden, Archmage | `{g} {f}` / `{t} {g}` |
| gothic_horror | Ambrose, Cordelia, Ephraim, Honora, Lucian, Wilhelmina | Ravensworth, Ashdown, Vane, Blackmoor, Strand, Mortlake | Lord, Mother, Doctor | `{g} {f}` / `{t} {f}` |
| cosmic_horror | Abner, Constance, Josiah, Lavinia, Obed, Prudence | Harrow, Crane, Wick, Ellery, Pike, Fenwright | Professor, Brother, Captain | `{g} {f}` / `{t} {f}` |
| space_opera | Ilyra, Kade, Mirelle, Orrin, Sava, Tavish | Halcyon, Drest, Korvath, Imre, Solano, Venn | Commander, Envoy, Admiral | `{g} {f}` / `{t} {f}` |
| cyberpunk | Juno, Kai, Nyx, Ozzy, Rhee, Vex | Okafor, Lindqvist, Sato, Varga, Moreau, Petrov | Zero, Static, Halo | `{g} {f}` / `{g} "{t}" {f}` |
| post_apocalypse | Ash, Dusty, Marrow, Rook, Sparrow, Tilly | Two-Wells, Ironsides, Saltbone, Halfmile, Rustwater, Nineteen | Boss, Doc, Old | `{g} {f}` / `{t} {g}` |
| weird_west | Abigail, Cass, Eli, Hattie, Jebediah, Silas | McCready, Stroud, Ballard, Tolliver, Harlan, Vasquez | Marshal, Preacher, Widow | `{g} {f}` / `{t} {f}` |
| hardboiled_noir | Frankie, Mae, Lou, Vivian, Eddie, Rita | Kowalski, Delacroix, Moran, Petrakis, Doyle, Fontaine | Detective, Mister, Miss | `{g} {f}` / `{t} {f}` |
| wuxia | Feng, Xue, Yun, Ming, Hua, Jian | Bai, Lan, Qiu, Shen, Yan, Zhou | Elder, Young Master, Sister | `{f} {g}` / `{t} {f}` |
| steampunk | Barnaby, Cornelia, Ezra, Florence, Gideon, Honoria | Gearwright, Pennyfeather, Thistlewaite, Coalhaven, Vantwistle, Mainspring | Professor, Captain, Doctor | `{g} {f}` / `{t} {f}` |
| fairy_tale | Elsbet, Jory, Marigold, Tobin, Rosamund, Hazel | Thrice-Lost, of the Glass Hill, Applewhite, the Youngest, Briarsong, Underhill | Goody, Little, Old | `{g} {f}` / `{t} {g}` |
| mythic_antiquity | Andreia, Damon, Eudora, Leontes, Melia, Theron | Bronze-Handed, Storm-Born, the Far-Wandering, Oath-Keeper, of the Hundred Ships, Laurel-Bright | Swift, Wise, Godlike | `{g} {f}` / `{g} the {t}` |
| oneiric | You, Someone, Mother, The Child, Your Teacher, A Stranger | Who Was Always There, Of the Stairs, Upside Down, With No Face, From Before, Of the Long Hall | Dear, Little, Old | `{g} {f}` / `{t} {g}` |
| redacted | Employee, Subject, Agent, Clerk, Witness, Applicant | ████, No. 0417, (Name Withheld), Grade C, Pending, ██-7 | Former, Acting, Deputy | `{g} {f}` / `{t} {g} {f}` |
| unwritten | Blank, Nobody, Unnamed, Nothing, Untitled, Empty | Draft, Line, Page, Mark, Space, Gap | The, A, Some | `{g} {f}` / `{t} {g}` |

Example (steampunk, doc 12 §20.5's seeds): `0x9c1f00d2a4be7731` draws `(0, 4, 1, 2)` → **Gideon Pennyfeather**; `0x13a4c0ffee0f1e2d` draws `(1, 4, 0, 1)` → **Captain Gearwright**. The same hero seed gives "Garreth Stormwright" in mythic_fantasy and "Lucian Ashdown" in gothic_horror.

### 7.5 Glyph Form skin names

Doc 06 §5.7 names every glyph in five genres and assigns the other eleven here. Forms below; essences, modifiers, triggers and conditions follow the same register and are authored in `assets/loc/en-US/18.ftl` under the same review. The Unwritten has none: no glyph is generated or Rewritten there (doc 06 §2.2), so its glyphs show their origin skin.

| Form | gothic_horror | space_opera | post_apocalypse | weird_west | hardboiled_noir |
|---|---|---|---|---|---|
| bolt | Nail of Blood | Psi Dart | Zip Shot | Hex Bullet | Cheap Shot |
| orb | Clotted Orb | Focus Sphere | Junk Bomb | Cursed Coin | Tossed Bottle |
| beam | Evil Eye | Mind Lance | Arc Torch | Witch's Stare | Hard Stare |
| cone | Grave Breath | Psi Wave | Spray Rig | Spit Hex | Smoke Screen |
| touch | Corpse Touch | Mind Touch | Jumper Clamp | Hand of Glory | Sucker Punch |
| self | Within the Vein | Centering | Self-Wire | Rabbit's Foot | Second Wind |
| aura | Mourning Shroud | Psi Field | Static Halo | Dust Devil Mantle | Bad Reputation |
| field | Circle of Candles | Stasis Zone | Junk Field | Salt Circle | Turf |
| wall | Iron Gate | Barrier Plane | Scrap Wall | Bone Fence | Stonewall |
| rune | Hex Mark | Psi Mine | Tripwire Rig | Hex Sign | Setup |
| nova | Bloodburst | Mind Flare | Overload Rig | Brimstone Burst | Blowup |
| chain | Thread of Kin | Resonance Link | Daisy Wire | Hanging Rope | Domino |
| manifest | Raising | Projection | Scrapbot Build | Calling Up | Calling a Favor |

| Form | fairy_tale | mythic_antiquity | oneiric | redacted | ur_text |
|---|---|---|---|---|---|
| bolt | Wishing Dart | Far-Darting Arrow | Thrown Thought | Memo | First Letter |
| orb | Glass Bead | Thunderstone | Soap Bubble | Packet | Round Hand |
| beam | Moonbeam | Sun-Chariot Ray | Staring Too Long | Directive | Underline |
| cone | Dandelion Breath | Breath of the Winds | Sigh | Circular | Flourish |
| touch | Fairy Kiss | Anointing | Shoulder Tap | Stamp | Signature |
| self | Heart's Wish | Heroic Spirit | Pinch | Self-Assessment | First Person |
| aura | Rose Mantle | Aegis Mantle | Daydream | Clearance Badge | Illumination |
| field | Toadstool Circle | Sacred Precinct | Room That Shouldn't Be | Restricted Area | Paragraph |
| wall | Briar Hedge | Wall of Bronze | Locked Door | Cordon | Ruled Line |
| rune | Witch's Knot | Votive Mark | Note Under the Pillow | Pending Form | Initial |
| nova | Midnight Chime | Olympian Thunder | Sudden Waking | Mass Notice | Exclamation |
| chain | Silken Thread | Thread of the Fates | Déjà Vu | Chain of Custody | Ligature |
| manifest | Summoned Helper | Invocation of Heroes | Imaginary Friend | Requisition | Character Sketch |

Hazard and event skin names are doc 12 §18.2 and §19.2's parenthetical names, stored verbatim as `loc.<hazard id>.skin.<genre_suffix>.name` and `loc.<event id>.skin.<genre_suffix>.name`.

## 8. Narrative

### 8.1 Lore bible (`nar.lore.*`)

Doc 01 §1.3's premise and four fiction rules are binding. The canon of the setting, one record per entry:

| ID | Entry |
|---|---|
| `nar.lore.palimpsest` | Every story ever told lies in one bottomless manuscript, scraped thin and written over. Nobody has seen its binding. |
| `nar.lore.first_edition` | The Ur-Text is the first draft under every page: all axioms at 5, nothing contradicts. Who wrote it is the one question no text in the game answers. |
| `nar.lore.authors` | Each genre has one Author, its patron (Canon §16.5). Authors wrote their genres and then stopped; they answer prayers but never appear in person. |
| `nar.lore.scraping` | When a story wears thin it is scraped and overwritten. The scraping is weather, not malice. |
| `nar.lore.style` | The Style is the rule that every page agree with itself. It is not a being. The Editors serve it. |
| `nar.lore.errata` | Characters who slipped through the knife when their page was scraped. They remember one genre and can hold any other's tools. |
| `nar.lore.red_ink` | The Editors' correcting fluid. It erases what it touches; drunk, it lets an Erratum correct the world (doc 04 Marginalia powers) at the price of becoming visible to every Editor (Canon §11.6). |
| `nar.lore.margin` | The blank edge of the page, where Errata gather between descents and where no Editor has jurisdiction. |
| `nar.lore.marginalia` | Notes written past the last page. Beyond the Ur-Text there is only margin, and Errata may keep writing there. |

Rules for every writer: never name the Ur-Text's author; never show an Author's face; never let a genre know it is fiction (only Errata, Editors, Margin NPCs and the Narrator know); never call an Erratum chosen.

### 8.2 Patron Authors (`nar.author.<genre_suffix>`)

Names, boons and taboos are doc 06 §16.1's. This lore explains each taboo in the fiction; prayer lines are the patron's answer text when pleased.

| Author | Lore | Why the taboo | Answer when pleased |
|---|---|---|---|
| Ysolde of the Thousand Roads | wrote every quest that begins with leaving home; walks still, unseen, on the longest road | a companion's body left behind is a quest abandoned | "The road remembers you. Walk on." |
| The Vigil-Keeper | wrote the candle in the window and the thing outside it; keeps watch so the dead stay counted | a summoned servitor is a corpse taken off the count | "One more night kept. Sleep, if you can." |
| The Sleeper Under the Index | wrote knowledge that should not be read, then hid beneath the catalogue | destroying a book wakes the Sleeper with a start | "Noted. Filed. Do not look again." |
| The Admiral of the Last Fleet | wrote the oath that crews keep to each other across the dark | shooting your own crew breaks the fleet's first article | "All hands accounted for. Carry on." |
| The Sysop | wrote the network and still holds root on it | an innocent killed is noise in a clean signal | "Access granted. Don't make me regret it." |
| Mother Rust | wrote survival after the end; feeds whoever keeps walking | starving is refusing her bread | "Eat. Then walk. That's the whole sermon." |
| The Rider at the Crossroads | wrote the deal at midnight and the duel at noon | shooting a fleeing man is not a fair fight | "Fair draw. Keep your hand steady." |
| The Witness | wrote the city and saw everything in it; never testifies | a lifted wallet is a crime in front of a witness | "I saw. I'm not telling. Go." |
| The Sage of the Ninth Peak | wrote the rivers and lakes and the code of the sword | a blade in the back has no honor | "The mountain is patient. Be the mountain." |
| The Grand Engineer | wrote the patent office and every machine filed there | a tool run to ruin insults its maker | "Splendid tolerances. Mind the boiler." |
| The Godmother of Thorns | wrote every bargain with a price hidden in the third clause | a debt carried out of the story is a promise broken | "Granted. Remember the price." |
| The Laurel-Crowned | wrote heroes and their downfalls in the same hexameter | forty deaths without an offering is hubris | "Sing, and you are remembered." |
| The Dreamer Who Writes Us | dreams every page; the Errata may be its waking thoughts | waking a sleeper with a wound is a nightmare | "Shh. You were never falling." |
| The Censor | an Author who went over to the Editors and now redacts its own genre | reading a text is reading what the Censor struck | "Approved for release. This once." |
| The Blank Page | not an Author but the absence of one; resets every prayer it hears | — | "…" |
| The First Author | the voice under the Ur-Text; possibly the Style itself, possibly not | — | "In the beginning, you." |

### 8.3 The Editors (`nar.editor.*`)

Editors are diligent, never cruel (doc 01 §1.3 rule 4). They speak in style-guide rulings and proofreading marks.

| Tier | Members (doc 13) | Fiction | Visual motif | Sample line |
|---|---|---|---|---|
| The Style | none | the law of agreement; never speaks | — | — |
| Editor-in-Chief | The Last Word (`boss.ur_editor`) | issues final rulings; its phases replay every earlier Editor's method | full stop | "Final. No further revisions." |
| Volume Editors | The Proofreader, The Redactor, The Continuity Editor, The Abridger (`boss.editor_v1..v4`) | each owns one correction: mark, hide, reconcile, cut | red circle; black bar; timestamp; scissors and ellipsis | "Query: why is this character still here?" / "Withheld." / "That is not what happened." / "Too long. Cut." |
| Revisors | `enemy.editors.revisor` (Inquisitor, Auditor, Revenue Agent and other genre skins) | field agents sent when a page runs long (Canon §11.6) | genre uniform with Red Ink cuffs | "Revision pass. Hold still." |
| Deputized Redactors | `enemy.editors.invader` (players, Canon §17.9) | other Errata who took the red pen for a page | the invader kit with a red-pen sigil | none; players speak for themselves |
| Red Ink | inkling, strikethrough, marginal_scribe, footnote, erratum_eater, red_pen_lancer | the marks themselves, animate | living proof marks | non-verbal; scratching and stamping |

Each Editor has one line per verse per attack (doc 13 §9.2 dominant axiom), voiced. The Proofreader's `strikethrough`: ARC "Strike. Unattested usage.", TEC "Strike. Incorrect specification.", MYT "Strike. Unsanctioned.", VRV "Strike. Implausible."

### 8.4 The Margin cast (`nar.margin.<npc>`)

The eight doc 13 §13.6 NPCs are retired Errata who chose neither a story nor erasure. They are voiced (§6.5). Each has 150 lines, of which 30 unlock one per account milestone (doc 14 §5.1) and run count (every 5 runs).

| NPC ID | Name, origin | Look and voice | Sample line |
|---|---|---|---|
| `npc.margin.archivist` | Ottoline Gloss, Gothic Horror | widow's black, magnifier on a chain; dry, precise | "Another specimen. Do try to bring the whole thing next time." |
| `npc.margin.binder` | Tamsin Stitchwort, Fairy Tale | thimbles on every finger, a needle behind one ear; warm, brisk | "Quires in, pages out. Mind the glue." |
| `npc.margin.recollector` | Ascender, Space Opera | a courtesy android with a cracked faceplate; polite, literal | "Memory located. Shall I play it back for you?" |
| `npc.margin.cartographer` | Ines Verso, Weird West | surveyor's chain and a blank map; laconic | "Map's blank till you walk it. Then it's mine." |
| `npc.margin.inkwell_keeper` | Sepia, Cosmic Horror | ink to the elbows, eyes like wet print; soft, unhurried | "The well gives what it gives. Every chance is written on the rim." |
| `npc.margin.codex_scribe` | Brother Rubric, Mythic Fantasy | tonsure, red-and-black ink pots; cheerful pedant | "Another page done. I'll rule the next line for you." |
| `npc.margin.quartermaster` | Mags Ferrule, Post-Apocalypse | goggles, a ledger and a cart; blunt and fair | "Price is the price. Same tomorrow." |
| `npc.margin.drillmaster` | Master Kerning, Wuxia | old swordsman in plain robes; spare | "Again. Slower. Now the same, but correctly." |

Inkwell Keeper and Quartermaster lines obey doc 15: no urgency, no scarcity claims, no casino lexicon, no reference to money pressure; the Keeper always names the exact odds panel. The **Narrator** (`nar.narrator`) is the Margin's ninth resident and never seen: a former Editor who resigned, now annotating in pencil instead of red. That is why marginal notes are pencil grey `#6B6B6B` and affectionate.

### 8.5 Ur-Text endings (`nar.ending.*`)

An ending is text plus a sequence of 3 to 5 illustrated plates in the final genre's style. It is chosen on the run-end screen from `RunSummary` and the run history (doc 14 §4.3, §4.6). Cosmetic only; nothing is granted by it.

```rust
fn select_ending(s: &RunSummary, h: &[FolioRecord], slots: &[SlotEnd]) -> Id32 {
    let ur = h.last().map(|f| f.genre) == Some(genre.ur_text);
    match s.end {
        run.end.victory if !ur                       => nar.ending.short_story,
        run.end.victory if slots.iter().any(|x| x.red_ink_doses >= 3) => nar.ending.red_letter,
        run.end.victory if slots[host].went_native.is_some()          => nar.ending.adopted,
        run.end.victory if s.party_size_max >= 2 && slots.iter().all(|x| x.status_at_end == Active)
                                                     => nar.ending.anthology,
        run.end.victory                              => nar.ending.written_back,
        run.end.margin_closed                        => nar.ending.closed_in_margin,
        run.end.margin_exhausted                     => nar.ending.last_margin,
        run.end.erased if s.score_at_victory > 0     => nar.ending.erased_in_margin,
        run.end.erased                               => nar.ending.erased,       // plate text keyed by Volume 1..=5
        _                                            => nar.ending.set_aside,    // abandoned, abandoned_version, departed
    }
}
```

Committing a Margins door is not an ending: `nar.ending.into_margins` plays as an interstitial at Folio 26 entry.

| Ending | Closing line (en-US) |
|---|---|
| `written_back` | "You closed the book, and the book, for once, kept you." |
| `anthology` | "Bound together. Every one of you is in the index now." |
| `adopted` | "You chose a story that was never yours. It has decided to keep you." |
| `red_letter` | "You read the first sentence in red. The Style has a new Editor." |
| `short_story` | "Not every tale needs a Volume. This one ended where it should." |
| `into_margins` | "No Editor writes here. Mind the edges." |
| `closed_in_margin` | "Notes in the margin, signed and dated. Closed." |
| `last_margin` | "There is no more margin. Even that is a kind of ending." |
| `erased_in_margin` | "Corrected at last, far past the final page." |
| `erased` | "Struck through in Volume {volume}. The page reads cleaner. It reads worse." |
| `set_aside` | "Set aside, spine uncracked. It will keep." |

After the closing line, one epilogue line per Erratum comes from `nar.grammar.epilogue.<origin_genre_suffix>` (§8.7) with slots `name`, `calling` and `genre`, drawn with `nar_rng(run_seed, 0, "nar.job.ending_epilogue", slot)`.

### 8.6 Item flavor rules

1. Flavor is at most 140 characters in en-US and at most 2 sentences; names are at most 32 characters. Locales may expand by 30%.
2. Flavor speaks in the register of the item's skin genre (§7.3); Palimpsestic items speak in the Ur-Text register; a Rewrite re-resolves to `loc.<template>.skin.<genre_suffix>.flavor`.
3. Flavor never states a number, a stat, a rule or a curse. Below `know.identified` it describes only what the player can see; below `know.classified` it describes only the appearance descriptor (Canon §13.6).
4. Legendary perks, Exotics and Palimpsestic uniques (doc 11 §8) carry authored flavor. Other items use `nar.grammar.item.<category>` with the affix themes as slots.
5. Manufacturer slogans (`loc.<mfr id>.slogan`, one per doc 11 manufacturer) pass the §7.1 review.
6. Flavor never mocks the genre it speaks for (doc 01 §1.3 rule 2); the wink lives only in Narrator footnotes.

### 8.7 Procedural Folio story hooks and grammars (`nar.grammar.*`)

Doc 12 §20 generates the story plan; this chapter turns it into text. A grammar record is Tracery-style JSON:

```json
{ "id": "nar.grammar.story.built_wonder.steampunk", "locale": "en-US", "slots": ["hero", "rival"],
  "rules": {
    "origin_praise":  ["#hero# raised #wonder.a# over #place#, and the whole city #cheered#.",
                       "They say #hero# built #wonder.a# in #span#, with nothing but #tools#."],
    "origin_slander": ["#hero# signed #wonder.a#, but every rivet was #rival#'s.",
                       "The #wonder# that #hero# built #failed# within #span#."],
    "origin_redact":  ["#hero# built [[#wonder.a#]] at #place#. The patent was #struck#."],
    "wonder": ["difference engine", "brass orrery", "sky-dock", "clockwork cathedral"],
    "place": ["the river docks", "Exhibition Hall", "the old gasworks"],
    "cheered": ["cheered", "rang every bell", "forgot to breathe"],
    "span": ["a single winter", "nine days", "one sleepless month"],
    "tools": ["a spanner and spite", "borrowed coal", "a drafting set and a grudge"],
    "failed": ["shook itself apart", "caught fire", "ran backwards"],
    "struck": ["withdrawn", "never filed", "struck from the register"] } }
```

**Expansion.** Start at `origin_<voice>` (voice from doc 12 §20.3: praise, slander, redact). Expand depth-first, left to right. A symbol with `n ≥ 2` alternatives draws `range(n)` once per occurrence; `n = 1` and slots draw nothing. Modifiers: `.a` (article), `.s` (plural), `.capitalize`. `[[…]]` renders as a redaction bar the player can hover to read. Maximum depth 8 and maximum output 400 characters; the lint enumerates every terminal against §7.1 and the register's "Never" list. An **overwritten** lore object (doc 12 §20.3) swaps the `hero` and `rival` slot values.

**Worked example.** Run `0x0123456789ABCDEF`, Folio 4, the deed event (index 2) on lore object 0: `key = (2 << 8) | 0 = 512`, so the generator is `nar_rng(run, 4, "nar.job.story_retelling", 512)` (seed `0xae27e411c755d822`). With `hero` = Gideon Pennyfeather and voice praise, the draws are `origin_praise` 1 of 2, `wonder` 3 of 4, `span` 1 of 3, `tools` 1 of 3 (4 draws): "They say Gideon Pennyfeather built a clockwork cathedral in nine days, with nothing but borrowed coal." With key 513 and voice slander the draws are 0 of 2 and 2 of 4: "Gideon Pennyfeather signed a sky-dock, but every rivet was Captain Gearwright's." Overwritten, it reads "Captain Gearwright signed a sky-dock, but every rivet was Gideon Pennyfeather's."

**Families.** `nar.grammar.story.<template_suffix>.<genre_suffix>` (every doc 12 §20.2 template × 16 genres), `nar.grammar.folio_intro.<genre_suffix>` (fields `title` and `line`, slots `hero`, `domain1`, `domain2`, `folio`), `nar.grammar.colophon_note.any`, `nar.grammar.bark.<set>.<genre_suffix>`, `nar.grammar.item.<category>`, `nar.grammar.epilogue.<genre_suffix>`, `nar.grammar.codex.<kind>`. Every family ships in every launch locale (§9.11). A fallback grammar emits the same JSON shape as its Scribe job (§9.5), field by field in schema order, from one generator.

### 8.8 Dialogue guidelines

- A subtitle line holds at most 90 characters; a node shows at most 2 lines. Choices are at most 60 characters and begin with a verb; a check choice shows its `chance_bp` (doc 13 §13.7) as a percentage with one decimal.
- Barks (`loc.npc.bark.<set>.<n>`): 8 sets per native faction (idle ×2, suspicious ×2, alert ×2, search, lost); the same NPC barks at most once per 600 ticks, and the party hears at most one bark per 180 ticks.
- Never concatenate translated fragments; use Fluent selectors for plurals and gender of the listener only.
- Natives never know they are fiction. Editors never threaten pain; they threaten deletion. Margin NPCs never pressure purchases.
- Banned in every line, authored or generated: slurs, sexual content, real persons, real brands, real-world politics and religions by name, self-harm instructions, casino lexicon.

## 9. The Narrator and the Scribe

### 9.1 Roles

The **Narrator** is the pencil voice in the margins: authored, voiced, always on. Its notes (`loc.nar.note.<moment>.<n>`) appear on the page turn under the Folio intro, on the Colophon lectern, on firsts (first jam, first Disconnected item, first Erasure stack, first death, first Editor), in tooltip footnotes and on loading pages. The **Scribe** (`crate.scribe`, doc 02) is the optional on-device model that rewrites a fixed set of text jobs in the Narrator's and genres' voices. Every Scribe job has a template grammar producing the same JSON shape, and the template is what the player sees whenever the Scribe is off, late, unsure or unsafe.

### 9.2 Model choice (`nar.scribe.model.*`)

| ID | Model (R4 §8.1, §8.6) | File | Where | Runtime |
|---|---|---|---|---|
| `nar.scribe.model.compact` | Qwen3.5-0.8B, non-thinking mode, Q4_K_M GGUF, Apache-2.0 | 485 MB (0.8 × 10⁹ × 4.85 bits) | default on every PC and Steam Deck | `llama-cpp-2` 0.1.159; CPU, 2 threads, below-normal priority; context 1,024 tokens |
| `nar.scribe.model.rich` | Qwen3.5-2B, non-thinking, Q4_K_M GGUF, Apache-2.0 | 1,212 MB | desktop only, with ≥ 16 GiB RAM | as above; Vulkan offload when measured free VRAM ≥ 1,536 MiB, else CPU with `min(4, physical_cores − 4)` threads |

Gemma 4 E2B (Apache-2.0) is not chosen: its 4.41 GB GGUF exceeds doc 02's 1,024 MiB Scribe RAM budget. Both models ship as an optional depot downloaded only after opt-in (§9.10). The model ID used in cache keys is the file name plus the first 16 hex digits of the file's BLAKE3.

**Device gate.** On enabling and after each driver change, a 64-token benchmark measures generation and prompt rates. The Scribe runs only if generation ≥ 12 tokens/s and prompt ≥ 100 tokens/s (`nar.scribe.budget`); otherwise the setting reads "Unavailable on this device" and templates are used. R4 estimates ≤ 15 tokens/s for the 0.8B model on Deck under load; the gate turns that estimate into a measured rule.

### 9.3 Jobs (`nar.job.*`)

The Scribe runs only in non-combat states: the Margin, the lobby, `InColophon`, `ExitVote`, `VictoryChoice`, the page turn and run-end screens (doc 14 §3.2). It never runs during `Exploring`, `BossAntechamber` or `BossFight`. Jobs run one at a time in the order listed per state.

| Job | Queued | Deadline (else template) | Input tokens ≤ | Output tokens ≤ | Output fields |
|---|---|---|---|---|---|
| `nar.job.colophon_note` | entering `InColophon` | first lectern interaction | 160 | 48 | `note` ≤ 160 chars |
| `nar.job.folio_intro` | doors light, one per door in door order | the page turn's hold point for the committed door | 160 | 64 | `title` ≤ 48, `line` ≤ 200 |
| `nar.job.bark_pack` | after the intros, one per door | entry of the next Folio | 140 | 128 | `lines`: exactly 8 of `{set, text ≤ 90}` |
| `nar.job.ending_epilogue` | run end | leaving the run-end screen | 260 | 192 | `lines`: 1 to 4 strings ≤ 200 |
| `nar.job.codex_story` | opening a Folio Leaf in the Margin | none | 220 | 160 | `title` ≤ 48, `text` ≤ 600 |
| `nar.job.codex_item` | opening an item in the Index | none | 140 | 64 | `flavor` ≤ 140 |

In-run lore objects, item flavor and every voiced line always use authored or template text, so text never changes under the player's eyes mid-Folio. Hub jobs show the template at once and replace it when the Scribe finishes.

### 9.4 Prompt templates

A shared prefix is evaluated once per session and its KV cache reused:

```
You are the Scribe, a copyist inside a manuscript of overwritten stories.
Reply with one JSON object that matches OUTPUT exactly. No other text.
Never mention real people, brands, places, religions or politics. Never mention money, prices,
gambling, sex, slurs or self-harm. Never state numbers, statistics or game rules.
Use only names listed in NAMES. Never use he, she, him or her for a NAMES entry; repeat the name or use "they".
```

Job template (`nar.job.folio_intro`, template version 3):

```
TASK: Rewrite DRAFT as a page title (at most 6 words) and an opening line (at most 30 words).
VOICE: {register_display}. {register_rules}. Example of the voice: "{sample_line}"
PAGE: Folio {folio}, {genre_display}. THEMES: {domain1}, {domain2}.
NAMES: {hero}, {rival}
DRAFT: {"title": "{fallback_title}", "line": "{fallback_line}"}
OUTPUT: {"title": string up to 48 characters, "line": string up to 200 characters}
```

Slots come only from content and the run (register record, doc 01 §9.5 sample line, doc 12 story domains, §7.4 names, the fallback grammar's own output). Players' typed text, chat and account names never enter a prompt.

### 9.5 Grammar-constrained output

Each job has a JSON Schema; the runtime converts it to a GBNF grammar (R4 §8.3) and also re-validates in code, because unsupported keywords are ignored by the converter.

```json
{ "$id": "nar.job.folio_intro", "type": "object", "additionalProperties": false, "required": ["title", "line"],
  "properties": {
    "title": { "type": "string", "minLength": 3, "maxLength": 48, "pattern": "^[^\\n{}<>#\\[\\]]+$" },
    "line":  { "type": "string", "minLength": 10, "maxLength": 200, "pattern": "^[^\\n{}<>#\\[\\]]+$" } } }
```

```
root  ::= "{" ws "\"title\"" ws ":" ws str48 ws "," ws "\"line\"" ws ":" ws str200 ws "}"
str48 ::= "\"" safe{3,48} "\""
str200 ::= "\"" safe{10,200} "\""
safe  ::= [^"\\\n{}<>#\[\]]
ws    ::= [ ]?
```

Fields that refer to game content (`set` in `bark_pack`) are enums of IDs, never free strings. Sampling: `temperature_milli` 700, `top_p_milli` 900, `top_k` 40, `repeat_penalty_milli` 1100, seed = the first 4 bytes of the cache key read little-endian.

### 9.6 Caching and co-op

```
key = BLAKE3("PALIMPSEST/SCRIBE/1\n" ‖ u32le(len(model_id)) ‖ model_id ‖ u32le(template_version)
             ‖ u64le(run_seed) ‖ u32le(folio_index) ‖ u32le(fnv1a32(job_id)) ‖ u64le(subject)
             ‖ u32le(len(locale)) ‖ locale)
```

`subject` is the door index for intros and bark packs, the slot for epilogues, the template `id32` for codex items, the Folio index for codex stories, and 0 otherwise. Example: model `qwen3_5_0_8b_q4km`, template 3, run `0x0123456789ABCDEF`, Folio 7, `nar.job.folio_intro` (`fnv1a32` `0x1a087cb5`), subject 2, `en-US` hashes 78 bytes to `f691c20497bea015abc2680f1d7c866041700afb537944d18187e7ccd3a1dc5a`; the sampling seed is `0x04c291f6`. Results that pass §9.8 are stored on disk (LRU, 64 MiB). A Daily seed therefore reads identically for everyone with the same model.

In co-op the host's Scribe text is authoritative: the host sends each validated result to clients on the reliable channel (Interface assumptions, doc 16). A client shows host text only if its own `flag.scribe.enabled` is true; otherwise it shows its template (§9.10).

### 9.7 Latency budget and job state machine

`t_ms = ceil_div(in_tokens × 1000, prompt_tps) + ceil_div(out_tokens × 1000, gen_tps)`. At the gate floor (100 and 12): `folio_intro` 1,600 + 5,334 = 6,934 ms; `colophon_note` 1,600 + 4,000 = 5,600 ms; `bark_pack` 1,400 + 10,667 = 12,067 ms. A party that commits a door in 2 s therefore sees template intros; the Scribe's intros appear when the vote takes longer or the cache already holds them.

| State | Event | Next |
|---|---|---|
| `Queued` | worker free and job at the queue head | `Running` |
| `Queued` | deadline passed, or Scribe disabled | `Fallback` |
| `Running` | output complete | `Validating` |
| `Running` | deadline passed, Scribe disabled, or 2 × the estimated `t_ms` elapsed | `Fallback` (generation aborted) |
| `Validating` | §9.8 passes | `Done` (cached) |
| `Validating` | §9.8 fails, no retry used, deadline − now ≥ estimated `t_ms` | `Queued` with sampling seed + 1 |
| `Validating` | §9.8 fails otherwise | `Fallback` |

`Done` and `Fallback` are terminal. `Fallback` is never cached; the template is deterministic anyway.

### 9.8 Safety filter

```rust
fn accept(job: &Job, out: &str) -> bool {
    let v = match parse_json(out) { Ok(v) => v, Err(_) => return false };
    if !schema(job).validate(&v) { return false; }                 // lengths, patterns, enums, no extra keys
    for s in strings(&v) {
        if banlist_hit(s) { return false; }                        // §7.1 groups, casino lexicon, nar.review.banlist.safety
        if job.register.never.iter().any(|w| contains_word(s, w)) { return false; }
        if s.chars().any(|c| c.is_ascii_digit()) && !job.allows_digits { return false; }
        if proper_nouns(s).any(|n| !job.allowed_names.contains(n)) { return false; }  // capitalized words not at sentence start
        if repeated_trigram_ratio_bp(s) > 2000 { return false; }
        if !script_matches(s, job.locale) { return false; }
        if s.contains(" he ") || s.contains(" she ") /* and him, her, his, hers, case-folded */ { return false; }
    }
    true
}
```

`allowed_names` holds the job's `NAMES`, the genre lexicon (§7.3 "Prefer"), the genre and Author display names, and the display names of every content ID in the prompt. `nar.review.banlist.safety` holds slurs, sexual terms, self-harm terms and real-world political and religious names, maintained by the narrative lead. Rejections are counted locally for the settings page; text is never uploaded.

### 9.9 Fallback grammars

Every job's fallback is the matching §8.7 family, drawn with `nar_rng(run_seed, folio, job_id, subject)`. Example `nar.grammar.folio_intro.gothic_horror`:

```json
{ "id": "nar.grammar.folio_intro.gothic_horror", "locale": "en-US", "slots": ["hero", "domain1", "domain2", "folio"],
  "rules": {
    "title": ["The House of #d1n.capitalize#", "#d1a.capitalize# and #d2n.capitalize#", "Where #hero# Kept Vigil"],
    "line":  ["#hero# kept the #d1n# behind locked doors. The doors are open now.",
              "Every portrait in this house remembers #hero#, and none of them fondly."],
    "d1n": ["#domain1#"], "d2n": ["#domain2#"], "d1a": ["#domain1#-haunted"] } }
```

Story domains enter as their lexicon nouns (`blood`, `moon`, `crypt`, `mirror`). The title draws first, then the line: 2 draws.

### 9.10 Opt-out, disclosure and the cosmetic guarantee

- `flag.scribe.enabled` defaults to false (doc 01 Q-07, doc 02). Enabling it shows a disclosure page, downloads the model depot and runs the device gate. Disabling it stops jobs at once and switches every displayed Scribe text back to its template; "Delete model and cache" removes both.
- The store page declares that the game generates text during play when the option is enabled, and that pre-generated content used authoring-time AI assistance where §10.3 records it (R4 §8.5).
- The Scribe never writes simulation state, saves' simulation sections, replays, packets other than the presentation text message, or anything doc 14's `RunSummary` holds. The Codex stores story events as IDs (doc 12 §20.4) and regenerates text from the cache or template.

### 9.11 Localization

- Launch locales: `en-US` (source), `fr-FR`, `de-DE`, `es-ES`, `es-419`, `pt-BR`, `it-IT`, `pl-PL`, `ja-JP`, `ko-KR`, `zh-Hans`, `zh-Hant`.
- Every `loc.` key and every `nar.grammar.*` family ships in all 12; CI fails on a missing key, and on the pseudo-locale `en-XA` overflowing any UI box (doc 02 §10).
- The Scribe runs only for `en-US` at launch; in other locales the option is hidden and templates are used.
- Name grammars (§7.4) are not translated; titles inside them are.
