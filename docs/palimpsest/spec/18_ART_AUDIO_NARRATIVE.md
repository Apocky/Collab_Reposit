# 18 — Art, Audio, Narrative and the Narrator
As of 2026-10-11. Status: BINDING. Written against canon-1.12 (adversarial check pending).

This chapter conforms to canon-1.11 (Canon §23), including §2.4's `presentation_hash`, §3.8's presentation generators, §4.3's fixed terrain and the end of item durability, §4.4's micro-voxels, the softer Erasure of §11.6 and §17.1, the public-domain Echoes of §16.2 and the mount retirement of §17.7; to DECISIONS.md items 28, 37, 38, 40, 46, 49, 53, 55, 65, 69, 77, 78, 87, 90, 91, 94, 97, 98, 100 and 114 (114 supersedes 41); to the rulings of `audit/canon_gaps_triage.md` that name it; and to the on-disk chapters 01 to 17 named in Imports. It owns everything the player sees, hears and reads that the simulation never reads: art direction and the per-genre art bibles, rendering style techniques built on doc 02's pipeline, animation, music and sound, the text skins and registers of every genre, the lore, and the Narrator with its optional on-device Scribe model. Every record here is presentation. `palimpsest_sim` never depends on any of it (Canon §22: rendering reads the simulation and never feeds it; language-model text is cosmetic only).

## Imports

|Source|Consumed|
|---|---|
|Canon §1.1, §1.6, §1.10|ID grammar, integer content, skin text keys, name resolution order|
|Canon §2, §2.4, §3.1–§3.3, §3.8|ticks and durations; `content_hash` and `presentation_hash`; RNG primitives and `stream_seed`, reused as pure functions; the presentation generators `nar_rng` and `pres_hash` (§1.2)|
|Canon §4.1–§4.4; §6.4 rows 17, 31, 34|body capsule, walk speed; fixed terrain, destructible cells (`block.tag.destructible`) and no item durability (§4.3); micro-voxels (250 mm, triplanar texturing at 64 texels per meter, render-only debris); action speed, dark-vision radius, footstep radius|
|Canon §5.2, §5.5–§5.11|genres, skins and the naming rule; Contradiction, Anachronism, palettes and music registers, Going Native, Unmoored|
|Canon §9, §10.1, §11, §12.2, §13.3, §13.6|body plans and pose tables, damage colors, statuses and Erasure, concealment and genre fog, rarity colors, appearances|
|Canon §14, §16.2, §16.3, §16.5, §17, §18, §19, §22|currency names, the 24 Echoes with the required public-domain figures (Sun Wukong, Loki, Lilith, Fenrir as `body.quadruped`, Jörmungandr as `body.serpentine`), kin, patrons, run rules (Erasure onset 720 s, one Erasure hunter per Folio carrying `rank.hunter`), the mount retirement and the unmounted `enemy.role.charger` (§17.7), events, namespaces `art.`, `aud.`, `nar.`, `vfx.` and `loc.` text keys, invariants|
|Brief §1, §2, §5; DECISIONS 8, 13, 14, 23, 28, 37, 38, 39, 40, 45, 46, 49, 53, 55, 65, 69, 77, 78, 87, 89, 90, 91, 94, 97, 98, 99, 100, 101, 103, 104, 107, 108, 111, 112, 114, 117, 118, 120, 127, 130, 132, 137, 138, 141, 143|premise, archetype × skin, runtime-model rules; tone, gore, opt-in model, UI skins; public-domain Echo cast; micro-voxels; text-only Narrator; original adaptive score; 15 launch locales; literary-bureaucrat Editors; the procedural Ur-Text ending; deceptive madness hallucinations; the first Folio Expansions (Age of Sail and Pirates, then Kaiju and Mecha); no mounts or vehicles; no item durability; 2 or more mini-bosses per genre; the walkable Margin hub; resident arcs delivered as text; cosmetic differences between co-op peers; "Crowns" kept; the Erasure hunter's `rank.hunter` identity and music cue; ink gore by default; 12 appearance presets per kin with palette and marking colors; PC only through 1.0; the Second Edition's second ending layer; all 12 core genres at Early Access (114, which supersedes 41)|
|Doc 01 §1.3, §9.5, §11|fiction rules; the tone guide and house lexicon, adopted as the voice bible; defaults Q-07 (Scribe off) and Q-14 (mild language)|
|Doc 02 §2–§3, §7–§11, §15|crates and `crate.scribe`, `bevy_rapier3d =0.37.0`, view frame (`ViewEntity.flags` INVISIBLE and GHOST bits; `ViewPlayer.revealed` and `dark_vision_mm`; `FolioView.seams`; `ChunkView` micro-voxel masks), event routing by `trigger` (at most one VFX and one audio record per event), render and audio pipelines, the light agreement clamp and `ambient_milli` table, page-turn and bleed geometry, five stems and music levels (level 4 while an alerted `rank.boss`, `rank.editor` or `rank.hunter` lives, §9), occlusion, Fluent, asset paths, BC7 texture arrays with full mip chains, memory budgets, `flag.scribe.enabled`, the canonical serialization behind both hashes (§11.2)|
|Doc 03 §4.5, §5.15–§5.16, §11, §14|block `skins`, pose tables, sound grid, event payloads: `ev.footstep {entity, gait, material}`, `ev.entity_landed {entity, fall_mm, material}`, `ev.explosion {center, radius_mm}`, `ev.fire_started`, `ev.fire_out`, `DmgFlags` bits (crit 0, sneak 1, dot 2, blocked 4, area 5, precision 6, lethal 12)|
|Docs 04, 05, 06|the creation Cosmetics step (04 §2.1); kin skin names per Origin (04 §2.4) and Going Native (04 §2.8); madness phantoms (04 §12.4); Echo bodies, eye heights and slots (04 §13); combat feel and the audio request for clang, parry, jam and ward break (05 §18); registers, the 84 glyphs and their skins (06 §5.7), patrons, taboos and prayer outcomes (06 §16.1)|
|Doc 07 §2.2|every `mat.` ID of §3.4's material sets and each material's `acoustic` override (doc 09 §7.2)|
|Docs 09, 10, 11|awareness states `aware.unaware`, `aware.curious`, `aware.suspicious`, `aware.searching`, `aware.lost`, `aware.alert` (09 §6.1), reveal meters (09 §4.8), `noise.*` IDs and acoustic classes `noise.surface.*` (09 §7), `StealthHud.marks` states and meters (09 §8); appearance descriptors, form nouns of the 12 core genres and the sigil atlas request (10 §3.3–§3.4); weapon skins, manufacturers and vendor names (11)|
|Doc 12 §2–§4, §14, §18–§20|fog, `render_style`, tilesets, Laws, bleed and seams, hazard and event names, `gpv.evt.weather_front`, story domains, fragments and the story plan JSON (§20.5)|
|Doc 13 §2.1, §3, §8–§10, §13|`body_state.stand_mm`, the 16 roles including the unmounted `enemy.role.charger`, rosters, bosses, Editors (display names the Proofreader, the Censor, the Continuity Editor, the Abridger, the Last Word), the genre mini-bosses (§9.3), the Revisor and its three named skins, Margin NPC IDs, deterministic dialogue checks and barks|
|Doc 14 §1.2, §3–§6, §8|`RunState`, `SlotStatus`, phases `Exploring`, `ColophonOpen`, `InColophon`, `ExitVote`, `BossAntechamber`, `BossFight`, `Endpaper`, `VictoryChoice`, `Ended`; exit vote, end states `run.end.*`, `RunSummary` (`mode`, `end`, `deepest_folio`, `party_size_max`, `score_at_victory`, `colophon_marked`, `ironman`; per slot `display_name`, `status_at_end`, `went_native`, `red_ink_doses`, `top_patron`) and `FolioRecord` (`index`, `genre`, `volume`, `kind`, `story`, `story_solved`, `leaf_carried`, `invasion`), the closing-manuscript inputs (§4.7), the 9 milestones, Margin facilities, the resident arcs `margin.arc.<resident>` of 6 beats (§5.6), Echoes, Novella, Abridged and the Second Edition flow (`mode.second_edition`, §8.9)|
|Doc 15 §3.1, §7.1, AP-7|cosmetics, the readability-limit request, reveal limits|
|Docs 16, 17|ghosts are never sent to an invader's client (16 §6); settings `access.gore`, `access.reduce_motion`, `access.photosensitive`, `access.sound_ripples`, `access.shimmer_bp`, `access.font`, `access.mark_hallucinations`, captions (17 §13); Narrator-note placement (17 §16.4); the Margin dialogue box `ui.menu.margin_dialogue` (17 §11.1) and the creation screen's preset picker (17 §5.7); UI tokens (17 §2.2), typography (17 §2.4) and the stealth meter's crosshair rings (17 §10.10)|
|R4 §2, §4, §7.5, §8, §10; R6 §7, §9, §11|graphics, meshing, authoring-time LLMs, small models and constraints, disclosure, the two acoustic systems; genre signals, GPV presentation fields, foreign-looks-foreign|

## 1. Scope, ownership and conventions

### 1.1 Ownership and encodings

- Namespaces: `art.` (meshes, clips, rigs, block arrays and layers, textures, icons, LUTs, fonts, palettes, lights, post stacks, bibles, visual records, viewmodels, silhouettes, native forms, appearance presets, readability limits), `vfx.` (damage, status, awareness, Contradiction, sprites, transitions, telegraphs, ragdolls), `aud.` (music, effects, ambience, voice, sets, buses, pools, mix), `nar.` (lore, Authors, Editors, Margin cast, Narrator, endings, grammars, names, registers, Scribe jobs and models, name review) and every `loc.` string, stored in `assets/loc/<locale>/<owner_doc>.ftl` (doc 02 §10).
- No crate in `palimpsest_sim`'s dependency closure loads these records (acceptance test 25).
- Content stays integer (Canon §1.6). A float the renderer uses is authored in milli-units with the suffix `_milli` (`bloom_milli: 250` is 0.25). Colors are `#RRGGBB` in tables and decimal integers in JSON, as doc 12 §3.6 encodes palettes (`#3A1F2B` → `3809067`).
- **Hashing** (Canon §2.4, DECISIONS 87). Every record in these namespaces and every `loc.` table is hashed into `presentation_hash`, never into `content_hash`, so co-op peers may differ in art, audio, VFX, narration and localization. Replays record `presentation_hash`, and a mismatch only warns.

### 1.2 Presentation randomness

Presentation never draws from a simulation stream. Canon §3.8 names the two pure functions that supply every cosmetic or narrative choice:

```rust
/// Narrative draws: names, grammars, Scribe sampling seeds. Same inputs, same text, on every machine.
/// `job` is always a `nar.` content ID, so it can never equal a canon `rng.` stream name.
fn nar_rng(run_seed: u64, folio: u32, job: &str, key: u64) -> Rng {
    Rng::from_seed64(mix64(stream_seed(run_seed, job, folio as u64) ^ mix64(key)))
}
/// Client-local cosmetic variation: particle seeds, sound variants, idle fidgets.
fn pres_hash(tag: &str, a: u64, b: u64) -> u64 { mix64(fnv1a64(tag) ^ mix64(a ^ mix64(b))) }
```

`stream_seed` is the Canon §3.2 formula evaluated as a pure function; no stream state exists. Example: run seed `0x0123456789ABCDEF`, Folio 4, job `nar.job.story_retelling`, key 512 gives seed `0xae27e411c755d822`. Presentation draw counts depend on content (a grammar draws once per multi-alternative symbol it expands); each family below states its count. A replay reproduces narrative text exactly because every input is in the replay header and the run history. Name grammars (§7.4) expand a name seed that doc 12 §20.1 already drew and recorded, with `Rng::from_seed64(name_seed)`; that expansion advances no stream (Canon gaps 1).

## 2. Art direction: the living manuscript

### 2.1 Five rules

1. **Every Folio is an illustrated page** in one illustrative tradition (§2.2); paper grain, ink edges and page turns keep the book in view.
2. **The palimpsest shows through.** When destruction removes micro-voxels from a destructible cell (Canon §4.3, §4.4; fixed terrain never loses any), every newly exposed micro-voxel face shows, for 600 ticks, the previous Folio's genre texture at 15% opacity; in Folio 1 the underlayer is the Unwritten's pencil grid. An Editor's erasure leaves red correction marks instead.
3. **Ink and paper are physical**: outlines are ink (§4.1), shading sits on paper (§4.2), Folios turn (§4.3), genre borders are stitched or bled (§4.4).
4. **Foreign things look foreign.** Errata, their gear and bleed visitors render in their origin genre's style, ink and animation step (R6 R12; doc 02 §8.6).
5. **Readability beats style.** Damage colors, rarity colors, telegraph hatching and awareness glyphs are global; color is never the only signal.

### 2.2 Each genre as an illustration style

`render_style` is doc 12's (§3.1) and selects the doc 02 shader variant. The tradition column directs the art team; it names historical techniques, never a living artist.

|Genre|render_style|Illustrative tradition|Line|Shading|
|---|---|---|---|---|
|Mythic Fantasy|painterly|illuminated manuscript plates and oil-glazed adventure paintings|brown iron-gall ink, tapered|soft glazes, gilt highlights|
|Gothic Horror|painterly|Victorian mezzotint and penny-dreadful frontispiece|black, heavy at the silhouette|deep low-key, candle pools|
|Cosmic Horror|ink|pulp-magazine pen-and-ink with halftone|cross-hatched, nervous|hatching density follows darkness|
|Space Opera|pbr|airbrushed chrome paperback covers|thin, only on silhouettes|glossy, rim-lit|
|Cyberpunk|pbr|neon photobash and rain-slick cel overlays|thin cyan or magenta rim|wet speculars, emissive signage|
|Post-Apocalypse|pbr|photocopied zine collage over rusted photographs|rough, broken|dusty, desaturated, high grain|
|Weird West|toon|dime-novel woodcut covers|thick, chunky|3 hard bands, halftone shadows|
|Hardboiled Noir|ink|chiaroscuro crime-comic panels|heavy black, spotted blacks|near-monochrome, one red|
|Wuxia|ink|shan shui ink-wash scrolls|wet brush, variable width|graded wash, paper shows through|
|Steampunk|pbr|engraved technical plates of exhibition catalogues|fine engraving lines|brass and soot, warm|
|Fairy Tale|toon|storybook watercolor and gouache|soft plum ink|3 soft bands, watercolor blooms|
|Mythic Antiquity|painterly|red-figure and black-figure pottery and fresco|black figure outlines|flat fresco tones, sun-bleached|
|Oneiric|xerox_collage|cut-paper collage and photocopied dream journals|scissor-cut edges|posterized, mismatched scales|
|The Redaction|pixel_dither|bureaucratic photocopy and 1-bit office bitmap|none, dithered|4 levels, ordered dither|
|The Unwritten|ink|unfinished pencil underdrawing on blank paper|grey graphite, sketchy|almost none; white page|
|The Ur-Text|painterly|gold-leaf illuminated initial pages|gold and black|rich glazes, gold leaf|

### 2.3 Voxel block style, texel density and lines

|Asset class|Texel density|Maps|
|---|---|---|
|Blocks and micro-voxels (doc 02 §8.3 arrays)|64 px per m, world-space triplanar (Canon §4.4)|64 × 64 px per layer, one layer tiling 1 m; a 250 mm micro-voxel face shows 16 × 16 texels|
|Block props, fixtures, doors, placeables|64 px per m|≤ 256²|
|Creatures and native NPCs|128 px per m|≤ 256² albedo plus ≤ 256² ink-control map|
|Errata, Echoes, mini-bosses, Editors|128 px per m, 256 on head and hands|≤ 512² each|
|Viewmodel items and arms|512 px per m|≤ 512² each|

- All world textures use nearest magnification and linear-mip minification, so blocks and creatures read as one crisp-texel set under each genre's post stack.
- Block faces are hand-painted with a 2 px darker rim that the shader draws from `fract(world_pos)` on faces of full 1 m cells, so greedy-merged quads still read as 1 m blocks; faces of partial cells (shaped, authored or damaged micro-voxel masks) draw a 1 px rim from `fract(world_pos × 4)` instead, so a chipped wall reads as chipped at 250 mm. Removed micro-voxels become render-only debris (Canon §4.4): each spawns one 250 mm cosmetic cube in its material's texture while doc 02's 256 cosmetic rigid bodies allow (oldest removed first), and a 4-particle dust puff of the material's mean color otherwise. `block.void_bedrock` renders as the paper's deckled edge.
- The ink-control map (BC7) carries R hatch mask, G outline weight, B emissive mask, A roughness; entities ship no normal maps.
- Line weights (× `outline_px`): entity and sky-facing block silhouettes 1.0; interior creases 0.5; block-to-block edges 0 (the rim covers them); interactables 1.0 plus a 1 px paper inner line; ground loot 1.0 in its rarity color.

## 3. Per-genre art bibles

Each genre has five records: `art.palette.<g>`, `art.light.<g>`, `art.post.<g>`, `art.bible.<g>` and `art.lut.<g>` (a 32³ KTX2 grading LUT authored from the palette). The first three swatches of every palette are Canon §5.9's, copied exactly and in order; this chapter adds four.

### 3.1 Palettes (`art.palette.<genre_suffix>`)

`key`, `fill`, `accent` are canon. `ink` draws outlines and diegetic text. `paper` is the page tone of diegetic manuscript surfaces: paper grain (§4.2), deckled edges, title cards, Lore and Journal pages and the page-turn back face. UI chrome and functional UI text use doc 17 §2.2's tokens, never these two swatches. `danger` fills enemy telegraphs (§4.7). `glow` tints fixtures, motes and pickups.

|Genre|key|fill|accent|ink|paper|danger|glow|
|---|---|---|---|---|---|---|---|
|mythic_fantasy|`#3B6E3B`|`#C9A227`|`#8FB3D9`|`#2B1D12`|`#EFE2BF`|`#C0392B`|`#FFD98A`|
|gothic_horror|`#3A1F2B`|`#7A0E14`|`#C8C3B4`|`#0F0A0C`|`#D8D0BC`|`#E23B2E`|`#F2C57A`|
|cosmic_horror|`#0E2A2E`|`#4F7F6A`|`#B8E986`|`#050F10`|`#C9D3C0`|`#F05A3C`|`#9CFF8A`|
|space_opera|`#0B1026`|`#3D7BFF`|`#FF8A3D`|`#05070F`|`#E6ECF5`|`#FF3B4F`|`#8FD4FF`|
|cyberpunk|`#120B1F`|`#FF2E88`|`#00E5FF`|`#07040D`|`#E9E4F2`|`#FF5A1F`|`#00E5FF`|
|post_apocalypse|`#6B4E2E`|`#C2A36B`|`#E04E1B`|`#231A10`|`#D9C9A3`|`#FF3A1A`|`#FFB347`|
|weird_west|`#8C4A2F`|`#E0B776`|`#2F3E4E`|`#2A1A12`|`#F1DFB8`|`#C8102E`|`#FFC56B`|
|hardboiled_noir|`#101010`|`#6E6E6E`|`#C22B2B`|`#000000`|`#E8E6E1`|`#C22B2B`|`#F2E2B0`|
|wuxia|`#1F3A2E`|`#D9C9A3`|`#B32020`|`#141414`|`#EFE6D0`|`#D42A1E`|`#FFE3A3`|
|steampunk|`#4A3A28`|`#B5793A`|`#5FA8A0`|`#22180F`|`#E8DCC0`|`#E0482C`|`#FFC77D`|
|fairy_tale|`#F2D7E8`|`#6FBF73`|`#E8C547`|`#3B2A4A`|`#FBF4E4`|`#D7304B`|`#FFF2A8`|
|mythic_antiquity|`#F0E6D2`|`#2E5A88`|`#C9792B`|`#1A1410`|`#E9D3B0`|`#B8321F`|`#FFE09A`|
|oneiric|`#2B1B4D`|`#8E7CFF`|`#FFC2E0`|`#120A26`|`#F3EEFF`|`#FF4F7A`|`#C9F1FF`|
|redacted|`#F2F2F2`|`#1A1A1A`|`#D7262E`|`#000000`|`#FFFFFF`|`#D7262E`|`#FFF7D6`|
|unwritten|`#FFFFFF`|`#E6E6E6`|`#CCCCCC`|`#595959`|`#FFFFFF`|`#D7262E`|`#FFFFFF`|
|ur_text|`#1C1408`|`#E8D9B0`|`#B8862B`|`#120C04`|`#E8D9B0`|`#E04A2A`|`#F6D77A`|

Global colors, never restyled: **Red Ink** `#D7262E` (Editors, Erasure, `liq.red_ink`); **Narrator pencil** `#4D4D4D` (marginal notes, graphite; it reads at least 5.17:1 on every genre's `paper`, minimum post_apocalypse, so text-only narration meets WCAG AA everywhere; the earlier `#6B6B6B` measured 3.26:1); damage colors (Canon §10.1); rarity colors (Canon §13.3).

**Telegraph contrast rule.** Let `cr(a, b)` be the WCAG 2.x contrast ratio. Each genre's telegraph edge color is whichever of `ink` and `paper` contrasts more with `danger`, and that ratio must be at least 4.0. The table passes with minimum 4.07 (wuxia, paper edge). Examples: mythic_fantasy 4.22 (paper), gothic_horror 4.58 (ink), cyberpunk 6.52 (ink), ur_text 4.80 (ink). The Ur-Text danger was moved from the vermilion `#C8341E` (3.78) to `#E04A2A` to pass.

**Reading contrast rule.** Lore and Journal text is `ink` on `paper`, so `cr(ink, paper) ≥ 7.0` in every genre. The table passes with minimum 7.00 (unwritten, `#595959` on `#FFFFFF`); the next lowest is post_apocalypse at 10.46. The Unwritten ink was darkened from `#8A8A8A` (3.45) to `#595959` for this rule.

### 3.2 Lighting rig and fog (`art.light.<genre_suffix>`)

The light field (doc 02 §8.5) carries simulation light levels, not colors. The renderer colors them: `irradiance = sky_tint × R + block_tint × G`, then applies the doc 02 agreement clamp, so color never changes how dark a cell reads. The key light is a directional sun or moon that casts shadows only in `open_tag` rooms (doc 12 §3.4); its contribution passes through the same clamp.

Every genre uses `key_azimuth_bam` 40960 (Canon §1.9 yaw convention); the key light points along that yaw, pitched down by its elevation.

|Genre|sky_tint|block_tint|Key|Key K|Key elevation (Bam)|key_milli|fog (doc 12)|Fog or haze color|haze_start_mm|haze_max_milli|
|---|---|---|---|---|---|---|---|---|---|---|
|mythic_fantasy|`#E8EEFF`|`#FFC98A`|sun|5600|10923|900|0|`#B9CBD9`|64000|300|
|gothic_horror|`#8C93B8`|`#FFB45E`|moon|4100|5461|350|1|`#6E6A78`|—|—|
|cosmic_horror|`#6F9A8E`|`#B8E986`|moon|7000|2731|200|1|`#2C4A48`|—|—|
|space_opera|`#AFC4FF`|`#CFE6FF`|star|6500|16384|600|0|`#0B1026`|96000|200|
|cyberpunk|`#6A4C8C`|`#FF7AD0`|none|—|—|0|0|`#3A2550`|32000|450|
|post_apocalypse|`#F2D2A0`|`#FFB060`|sun|4800|8192|1000|0|`#C9A678`|48000|450|
|weird_west|`#FFE2B0`|`#FFC070`|sun|3800|4096|950|0|`#E8C08A`|64000|350|
|hardboiled_noir|`#9AA3AD`|`#FFE6A8`|moon|4500|6372|300|1|`#4A4F55`|—|—|
|wuxia|`#E6EFE8`|`#FFD08A`|sun|6000|9102|800|0|`#DCE6DE`|32000|550|
|steampunk|`#E8D9B8`|`#FFC27A`|sun|4500|7282|700|0|`#B8A88A`|40000|450|
|fairy_tale|`#FFF0F6`|`#FFE7A0`|sun|5200|9102|850|1|`#D8E8D0`|—|—|
|mythic_antiquity|`#FFF4DC`|`#FFB868`|sun|5800|14564|1000|0|`#F4E6C8`|80000|250|
|oneiric|`#B8A8FF`|`#FFC2E0`|moon|9000|1820|500|1|`#6E5CB8`|—|—|
|redacted|`#FFFFFF`|`#F4FAFF`|none|—|—|0|0|`#FFFFFF`|0|0|
|unwritten|`#FFFFFF`|`#FFFFFF`|none|—|—|0|0|`#FFFFFF`|0|0|
|ur_text|`#F6E7C0`|`#F6D77A`|sun|5000|10923|800|0|`#E8D9B0`|96000|250|

**Fog agreement.** Genre fog is concealment (Canon §12.2: 1 point per 8 m, each point × −667 Bp of sight detection). The visual fog matches it linearly:

```
fog_pts                 = doc 12 fog of local_genre(camera) + (1 while gpv.evt.weather_front is active)
fog_opacity_milli(d_mm) = min(1000, floor_div(fog_pts × 667 × d_mm, 80000))
haze_milli(d_mm)        = min(haze_max_milli, floor_div(haze_max_milli × max(0, d_mm − haze_start_mm), 64000))
opacity_milli(d_mm)     = if fog_pts > 0 { fog_opacity_milli(d_mm) } else { haze_milli(d_mm) }
```

Haze applies only while `fog_pts = 0`; a weather front in a fog-0 genre (Cyberpunk acid rain, Post-Apocalypse dust storm) switches the genre to the fog formula with `fog_pts = 1` for its 1800 ticks. `d_mm` is the camera-to-fragment distance.

Example: Gothic Horror at 48,000 mm gives `floor_div(1 × 667 × 48000, 80000) = 400` milli opacity, and the simulation's 6 fog points give a detection factor of `10000 − 6 × 667 = 5998` Bp: the visible fraction (600 milli) and the simulated one (5998 Bp) agree within one 8 m step. At 120,000 mm fog reaches 1000 milli exactly when the simulation's 15 points block sight. Haze in fog-0 genres is capped at 550 milli, so a distant enemy the simulation can see is always visible. Mythic Fantasy at 96,000 mm: `floor_div(300 × 32000, 64000) = 150` milli.

### 3.3 Post-process parameters (`art.post.<genre_suffix>`)

Fields are doc 02 §8.6's parameters plus `sat`, `contrast`, `paper` and the selective-color triple. Passes run in doc 02 §8.6's order; `sat`, `contrast` and selective color are baked into the genre's LUT stage, and paper grain multiplies the image right after the LUT (§4.2). All values are milli-units except `outline_px` (pixels at 800 lines, scaled by §4.1), `posterize` (levels, 0 = off), `dither` (0 or 1) and `anim_step` (1, 2 or 3). `sat` and `contrast` are 1000 at neutral. `transition` is the style used when this genre is the destination (§4.3).

|Genre|outline_px|bloom|sat|contrast|grain|paper|vignette|aberration|scanlines|halftone|posterize|dither|anim_step|transition|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|mythic_fantasy|1|250|1050|1000|0|250|200|0|0|0|0|0|1|page_turn|
|gothic_horror|1|300|700|1150|150|350|450|0|0|0|0|0|1|ink_bleed|
|cosmic_horror|2|150|600|1200|250|450|500|150|0|200|0|0|2|ink_bleed|
|space_opera|1|500|1150|1050|0|100|150|100|0|0|0|0|1|iris|
|cyberpunk|1|650|1250|1150|100|100|250|250|150|0|0|0|1|glitch|
|post_apocalypse|1|200|800|1100|300|300|300|0|0|0|0|0|1|film_burn|
|weird_west|2|200|1000|1100|200|400|300|0|0|300|0|0|2|iris|
|hardboiled_noir|3|150|100|1350|350|200|550|0|0|0|0|0|2|film_burn|
|wuxia|2|250|750|1050|0|500|200|0|0|0|0|0|1|ink_bleed|
|steampunk|1|350|950|1050|150|300|300|0|0|0|0|0|1|iris|
|fairy_tale|2|400|1150|950|0|400|250|0|0|0|0|0|2|page_turn|
|mythic_antiquity|1|300|1000|1050|0|350|150|0|0|0|0|0|1|page_turn|
|oneiric|2|450|1100|1000|200|300|350|300|0|400|6|0|3|ink_bleed|
|redacted|0|100|300|1300|0|0|100|0|200|0|4|1|2|glitch|
|unwritten|2|0|0|900|0|600|0|0|0|0|0|0|3|page_turn|
|ur_text|2|400|1050|1050|0|450|300|0|0|0|0|0|1|page_turn|

- **Selective color.** Hardboiled Noir sets `keep_hue_bam: 0` with `keep_hue_width_bam: 2731` (±15°) and `keep_sat_milli: 1000`: pixels whose hue lies within 15° of the accent red keep full saturation; everything else uses `sat` 100. Every other genre sets `keep_hue_width_bam: 0`.
- **Comfort overrides** (doc 17 `access.reduce_motion`): aberration, scanlines, vignette and grain go to 0, `anim_step` is forced to 1 for every entity, and the transition becomes doc 17's 90-tick crossfade.
- The Redaction draws no ink outlines (`outline_px` 0): its 1-bit dither is the line.

### 3.4 Bible entries (`art.bible.<genre_suffix>`)

Block families are doc 12's tilesets dressed in Canon §5.9's materials. Silhouettes name doc 13 roster templates and the shape each reads as at 30 m (§3.6).

|Genre|Block families|Signature props|Key silhouettes|VFX style|
|---|---|---|---|---|
|mythic_fantasy|`castle` limestone and oak beams; `cavern` wet granite, glowing ore|banners, braziers, dragon skulls|kingsguard_knight tower-shield block; conclave_battlemage raised glowing hand; cave_troll long-armed giant; wyvern wide wings|gilded sparks, ribbon trails, gold runes|
|gothic_horror|`manor` damask plaster, black oak; `catacomb` bone niches, wet brick|candelabra, watching portraits, coffins|ghoul crouched long arms; vampire_courtier tall narrow cape; witch_hunter wide hat and crossbow; werewolf hunched mass|ash, blood mist, moths|
|cosmic_horror|`sunken_town` barnacled clapboard; `abyss` wrong-angled basalt, lit fungus|chalk circles, specimen jars, dripping idols|cultist hood and raised arms; deep_one fish-backed hunch; shoggoth_spawn eyed blob; star_seer three-armed|tendrils, wet-ink blooms, witchlight motes|
|space_opera|`starship` hull plate, deck grating; `station` glass, hydroponics|consoles, cargo pods, holo-tables|navy_trooper helmet and rifle; psi_adept floating robe; hive_warrior crescent claws; security_droid boxy shield|plasma bolts, lens flares, hex shield ripples|
|cyberpunk|`megablock` wet concrete, invented-brand neon; `undercity` pipes, graffiti|kiosks, noodle stalls, holo-ads with fictional logos|corp_enforcer visor and rifle; chrome_brawler huge chrome arms; mantis_assassin arm blades; riot_mech box on legs|glitch blocks, scanline holograms|
|post_apocalypse|`wasteland` cracked asphalt, rebar; `bunker` rusted steel, rad-glass|car hulks, drums, scrap barricades|raider_scav spiked shoulders; glow_zealot glowing chest and a rad-sling launcher; mutant_behemoth asymmetric giant; ram_raider welded car-door shield|rust flecks, oily smoke, rad motes|
|weird_west|`boomtown` weathered planks, adobe; `mine` timber shoring, ore carts|hitching posts, wanted posters with invented names, gallows|dead_miner lantern and dynamite belt; marshal_rifleman long rifle; railroad_automaton smokestack head; posse_deputy coiled lasso|dust puffs, powder smoke, hex sigils|
|hardboiled_noir|`city_street` wet brick, fire escapes; `tenement` peeling paper, blinds|payphones, desk lamps, blinds that stripe light|mob_torpedo fedora and drum gun; leg_breaker wide coat and bat; shadow_tail thin trench coat; dock_heavy carrying a crate|rain splashes, smoke curls, flashbulbs|
|wuxia|`mountain_sect` terraces, red lacquer pillars; `bamboo_vale` bamboo, rope bridges|incense burners, bronze bells, stone lanterns|jiangshi outstretched arms; sect_disciple flowing sleeves; court_crossbowman box crossbow; fox_spirit fan of tails|brush strokes, petals, qi swirls|
|steampunk|`foundry` riveted iron, kilns; `airship` varnished decks, brass rails|boilers, gauges, gear trains, tesla coils|brass_automaton barrel torso; sky_aeronaut goggles, grapnel and ornithopter wings; galvanist coil pack; steam_juggernaut furnace body|steam puffs, brass sparks, gears|
|fairy_tale|`deep_wood` mossy trunks, toadstools; `cottage` thatch, gingerbread trim|spinning wheels, mirrors, wells|big_bad_wolf oversized wolf, head above an Erratum's; bridge_troll stooped giant; hedge_witch pointed hat; wild_huntsman antlers and hunting horn|glitter, petals, watercolor blooms|
|mythic_antiquity|`temple` marble columns, mosaics; `labyrinth` limestone meanders|amphorae, braziers, statues|hoplite round shield and spear; harpy wings; oracle_priestess veil and tripod; centaur_archer horse body and bow|golden motes, laurel leaves, red-figure flames|
|oneiric|`dreamscape` impossible stairs, cloud blocks; `nursery` giant toys|doors in midair, handless clocks, music boxes|sleepwalker arms out; nightmare_mare burning mane; stair_mimic drawn as a stair; faceless_teacher third arm|paper cut-outs, floating letters, reversed particles|
|redacted|`bureau` grey cubicles, ceiling panels; `archive` cabinets|water coolers, shredders, stamps, ceiling domes|clerk clipboard; blackbar_sentinel black rectangle; shredder_unit toothed box; faceless_agent blurred face|black bars, pixel noise|
|unwritten|`blank_page` white blocks with a pencil grid; `margin` ruled lines|unfinished pencil outlines|Editor squads only (§8.3)|eraser crumbs, pencil hatching, red ink|
|ur_text|`palimpsest` strata of every genre's texture; `colophon` gold-leaf lecterns|giant quills, inkwells, letter blocks|stray_letters glyph swarm; first_draft in the last visited genre's skin; lettered_serpent of letters; illuminator gilded|gold-leaf flakes, illuminated initials|

**Editors in any genre.** An Editor or Red Ink servant takes its mesh skin from the local genre (doc 13 §9.1) and always adds the red overlay: silhouette outline in Red Ink at 1.5 × `outline_px`, struck-through handwriting decals on its body, and `ink` render style regardless of the local genre. A correction is never part of the page it corrects.

**Block material sets.** Each genre's block families are built from these doc 07 materials; the texture descriptions above name looks, and the simulated material is the `mat.` ID. Canon §5.9's signature materials appear in every set. Every listed ID exists in doc 07 §2.2, and the asset lint rejects a block skin whose material is outside its genre's set or the shared set (`mat.soil`, `mat.sand`, `mat.ice`, `mat.leaves`, `mat.web`, `mat.slag`, `mat.void`, `mat.glass`).

|Genre|Walls|Floors|Trim and fixtures|Canon §5.9 signature|
|---|---|---|---|---|
|mythic_fantasy|`mat.granite`, `mat.oak`|`mat.granite`, `mat.oak`|`mat.steel`, `mat.gold`|`mat.steel` `mat.moonsilver` `mat.oak` `mat.dragonbone`|
|gothic_horror|`mat.slate`, `mat.brick`, `mat.blackwood`|`mat.blackwood`, `mat.slate`|`mat.silver`, `mat.bone`, `mat.cast_iron`|`mat.silver` `mat.blackwood` `mat.bone`|
|cosmic_horror|`mat.basalt`, `mat.coral`|`mat.basalt`|`mat.obsidian`, `mat.chitin`|`mat.obsidian` `mat.star_metal` `mat.chitin`|
|space_opera|`mat.plasteel`, `mat.titanium`|`mat.plasteel`|`mat.polymer`, `mat.psi_crystal`|`mat.plasteel` `mat.titanium` `mat.psi_crystal`|
|cyberpunk|`mat.concrete`, `mat.polymer`|`mat.concrete`|`mat.chrome`, `mat.carbon_fiber`|`mat.carbon_fiber` `mat.polymer` `mat.nanoweave`|
|post_apocalypse|`mat.concrete`, `mat.scrap_iron`|`mat.concrete`, `mat.sand`|`mat.rubber`, `mat.rad_glass`|`mat.scrap_iron` `mat.rubber` `mat.rad_glass`|
|weird_west|`mat.sandstone`, `mat.pine`|`mat.pine`, `mat.sand`|`mat.iron`, `mat.leather`|`mat.iron` `mat.leather` `mat.hexed_silver`|
|hardboiled_noir|`mat.brick`, `mat.concrete`|`mat.pine`, `mat.concrete`|`mat.steel`, `mat.brass`, `mat.glass`|`mat.steel` `mat.brass` `mat.tweed`|
|wuxia|`mat.cedar`, `mat.bamboo`, `mat.granite`|`mat.cedar`, `mat.granite`|`mat.jade`, `mat.bronze`|`mat.jade` `mat.bronze` `mat.silk`|
|steampunk|`mat.brick`, `mat.cast_iron`|`mat.oak`, `mat.cast_iron`|`mat.brass`, `mat.copper`|`mat.brass` `mat.copper` `mat.aetherium`|
|fairy_tale|`mat.oak`, `mat.thornwood`, `mat.birch`|`mat.soil`, `mat.oak`|`mat.cold_iron`, `mat.glass`|`mat.cold_iron` `mat.glass` `mat.elderwood`|
|mythic_antiquity|`mat.marble`, `mat.terracotta`|`mat.marble`|`mat.bronze`, `mat.gold`|`mat.bronze` `mat.orichalcum` `mat.marble`|
|oneiric|`mat.dreamstuff`, `mat.paper`|`mat.dreamstuff`|`mat.glass`, `mat.brass`|`mat.dreamstuff`|
|redacted|`mat.concrete`, `mat.polymer`|`mat.concrete`|`mat.paper`, `mat.steel`|`mat.redacted_alloy`|
|unwritten|`mat.paper`|`mat.paper`|`mat.vellum`|`mat.vellum`|
|ur_text|`mat.granite`, `mat.vellum`, plus strata of every core set|`mat.vellum`|`mat.gold_leaf`, `mat.gold`|`mat.vellum` `mat.gold_leaf`|

### 3.5 Typography (`art.font.*`)

Every font is SIL Open Font License 1.1 or Apache 2.0, verified against its license file at import (§10.3). Numerals use tabular figures. Body text never drops below doc 02's 12 px at 800 lines.

|Genre|Headings|Body|Numerals|Initial capitals|
|---|---|---|---|---|
|mythic_fantasy|Cinzel|EB Garamond|EB Garamond|gold-leaf versal, 3 lines|
|gothic_horror|UnifrakturCook|IM Fell English|IM Fell English|black-letter with blood-red fill|
|cosmic_horror|Special Elite|Libre Baskerville|Special Elite|none; typed headers|
|space_opera|Orbitron|Exo 2|Orbitron|none|
|cyberpunk|Bebas Neue|Rajdhani|Share Tech Mono|none|
|post_apocalypse|Black Ops One|Courier Prime|Courier Prime|stencil|
|weird_west|Rye|Old Standard TT|Old Standard TT|woodcut slab|
|hardboiled_noir|Playfair Display|Cutive Mono|Cutive Mono|none|
|wuxia|Ma Shan Zheng|Noto Serif and Noto Serif SC|Noto Serif|brush seal stamp|
|steampunk|Limelight|Libre Caslon Text|Libre Caslon Text|engraved roundel|
|fairy_tale|Berkshire Swash|Lora|Lora|vine-wrapped versal|
|mythic_antiquity|Marcellus|Gentium Book Plus|Marcellus|meander border|
|oneiric|Caveat|Nunito|Nunito|cut-out letter|
|redacted|Space Mono|Inter|Space Mono|none|
|unwritten|Cormorant Garamond|Cormorant Garamond|Cormorant Garamond|none|
|ur_text|Cinzel Decorative|EB Garamond|EB Garamond|full illuminated initial|
|the Margin (hub)|EB Garamond|EB Garamond|EB Garamond|pencil versal, 2 lines|

Every locale also ships Noto Sans CJK and Noto Serif CJK for Chinese, Japanese and Korean, and Atkinson Hyperlegible as the legibility font that doc 17's `access.font` substitutes for every body face. Functional UI text (HUD, menus, item cards, tooltips, numbers) uses doc 17's `access.font` body font; the heading faces above are doc 17's display fonts, and the body and numeral faces set Lore and Journal pages and title cards (doc 17 §2.4). A heading face without the glyphs of the active locale falls back to Noto Serif CJK or Noto Serif. Launch locale records are `nar.locale.<tag>` (DECISIONS 40, doc 19 Canon gap G7): the BCP 47 tag is lower-cased with `-` written as `_`, so en-US is nar.locale.en_us, and the 15 launch locales each own one record.

### 3.6 Silhouette roles (`art.silhouette.<role>`)

Every enemy reads its doc 13 role (§3, 16 roles) from its idle-pose LOD0 bounding box and its weapon line, measured by the asset lint (§10.1) in mm. Inputs: `H` = the template's `body_state.stand_mm` (doc 13 §2.1); `W` = the bounding box's larger horizontal extent; `D1 ≥ D2 ≥ D3` = the box's three extents sorted. The lint first requires `|bbox height − H| ≤ 150`, so the silhouette and the hit capsules agree (§5.2 readability guard); then it applies the row of the template's role. Bosses (`enemy.role.boss`) and the Editors are exempt; their arenas and telegraphs carry the read.

|Role|Rule (every body plan unless stated)|
|---|---|
|swarmer|`H ≤ 900`, or `body.swarm`, or a hunched humanoid: `H ≤ 1700`, idle head center at least 150 ahead of the pelvis, and no weapon line ≥ 600|
|skirmisher, stalker|humanoid: `1300 ≤ H ≤ 1900`; stalker idle crouch top ≤ 1400. Other plans: the stalker's idle top ≤ 1400 (coiled serpent, mimic shape); no size rule for skirmishers|
|soldier, shooter, sniper|a weapon line ≥ 600 (soldier, shooter) or ≥ 1000 (sniper) held between hip and shoulder height in idle; melee soldiers hold a polearm, axe, fork or blade of that length|
|brute|`D1 ≥ 2200` and `D2 ≥ mul_bp(D1, 6000)`; for humanoids `D1` is `H`|
|guardian|a frontal shield, crate or plate of at least 900 wide × 1200 high, facing forward in idle|
|caster, controller, support, summoner|humanoid: hands at shoulder height or above in the cast pose, with an emissive mask on the hands or focus. Other plans: an emissive mask covering at least 1000 Bp of the front-view area during the cast pose|
|flier|avian and construct plans: span ≥ 2 × body height; humanoid fliers: a visible lift device (wings, rotors, balloon, jets) with span ≥ `mul_bp(H, 12000)`|
|charger|humanoid: a runner's split stance in idle, the lead foot at least 500 ahead of the trailing foot, with the weapon, shield or device held forward of the chest; no mount or vehicle (Canon §17.7)|
|sapper, artillery|a carried charge, launcher or digging tool ≥ 400 in its largest dimension|

Examples against doc 13 §8: `orc_raider` (swarmer, humanoid, `H` 1650, hunched) passes; `leg_breaker` (brute, humanoid) needs `H ≥ 2200` and width ≥ 1320, so doc 13 authors it at `stand_mm` 2200 (Interface assumptions); `stair_mimic` (stalker, amorphous) passes at idle top 1000; `sky_aeronaut` (flier, humanoid, `H` 1800) needs wings spanning at least 2160.

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

### 3.8 Visual and block records

`ViewEntity.visual` (doc 02 §7.2) is the `id32` of an `art.visual.*` record; the client spawns the entity's presentation from it. `art.block.*` records are the block `skins` targets of doc 03 §4.5 and fill doc 02 §8.3's `layer_of` table.

```json
[
  { "id": "art.visual.enemy.gothic_horror.ghoul", "template": "enemy.gothic_horror.ghoul",
    "mesh": "art.mesh.enemy.gothic_horror.ghoul", "rig": "art.rig.humanoid",
    "anims": ["art.anim.humanoid.idle", "art.anim.humanoid.walk", "art.anim.ghoul.claw", "art.anim.ghoul.feast"],
    "silhouette": "art.silhouette.skirmisher", "scale_milli": 1000, "voice": "aud.set.voice.ghoul",
    "lod_mm": [16000, 40000], "ragdoll": 1 },
  { "id": "art.block.marble_masonry.antiquity", "array": "art.blockarray.mythic_antiquity", "side": 12, "top": 13,
    "roughness_milli": 700, "metallic_milli": 0, "emissive_milli": 0, "rim_milli": 250, "underlayer": 1 }
]
```

Each `anims` entry is a doc 03 pose-table ID and the clip of the same name inside the `.glb`. `underlayer: 1` enables the §2.1 rule-2 reveal on that block.

- **New worldgen and hazard block skins (§3.8)**:
  - `art.block.altar.<g>`: worldgen patron altar (`block.altar`, fixed terrain), sculpted stone with genre-specific religious and patron carvings.
  - `art.block.rubble.<g>`: loose fractured blocks (`block.rubble`), jagged stone textures with dark cracks.
  - `art.block.pit_lid.<g>`: concealed top-slab floor hatch (`block.pit_lid`), matching the room's floor texture with subtle hairline seams.
  - `art.block.hull_panel.space_opera`: bolted plasteel bulkhead panel (`block.hull_panel`) with hazard chevron framing.
  - **Chipping rendering**: driven by `ev.block_chipped` (doc 03 §13.2) and `ChunkView` masks; 1 px rims rendered from `fract(world_pos * 4)` and removed micro-voxels spawn 250 mm debris cubes or 4-particle dust puffs (Canon §4.4).

### 3.9 The Margin (`art.bible.margin`)

The Margin is a walkable first-person 3D hub with fixed architecture (DECISIONS 77, doc 14 §5, Canon §17.8). It is the blank edge of the page, so `art.bible.margin` renders it with the Unwritten's records unchanged: `art.palette.unwritten`, `art.light.unwritten`, `art.lut.unwritten` and `art.post.unwritten`, over the Unwritten's `margin` tileset (§3.4) of ruled lines on `mat.paper` and `mat.vellum`. Residents, trophies and visiting Errata keep their origin genres' style, ink and animation step (§2.1 rule 4), so every story's color reads as foreign against the white page. Its typography is §3.5's Margin row, its music `aud.music.margin` (§6.1), and the Narrator's arc notes sit in the Ledger (§8.4).

## 4. Rendering techniques

All passes slot into doc 02 §8.1's frame order and §8.6's post order. Shaders use floats; their parameters come from integer content.

### 4.1 Ink edges

Edges come from the Sobel pass on depth and normals (doc 02 §8.6):

```
depth_edge  = |Sobel(linear_depth)| / depth_center ≥ depth_thr_milli / 1000        // default 80
normal_edge = 1 − dot(n_center, n_mean_of_8) ≥ normal_thr_milli / 1000             // default 250
width_px    = max(1, round_half_even(outline_px × H, 800))   for outline_px > 0     // H = render height
ink         = edge_owner.foreign ? origin_genre.ink : local_genre.ink
weight      = silhouette 1.0, interior crease 0.5 (§2.3), scaled by the G channel of the ink-control map
```

A per-pixel object-ID buffer (16 bits, entity `id32` low bits) marks silhouettes, so a foreign entity's outline uses its own genre's ink and width. Example: `outline_px` 2 at 1440 lines gives `round_half_even(2880, 800) = 4` px; at 800 lines, 2 px.

**Contradiction on items** (`vfx.contra.*`, Canon §5.5): Strained (C 1–2) draws a small ink blot decal on the held item; Contradicted (C 3–5) adds fluttering glyph particles, 4 per second, in the origin genre's ink; Disconnected (C 6) flickers the item between its skin and the torn-page placeholder at 4 Hz (R6 R12). With doc 17's `access.photosensitive` (its flicker-off setting) it shows the static torn-page icon instead. A jam, fizzle or spark plays `vfx.contra.failure`: the item's outline smears for 12 ticks.

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

- **Post, light and fog blend.** The camera samples the bleed field at its own position to get weights `t_0..t_4`. Every numeric field of `art.post.<g>`, `art.light.<g>` and the fog renders as `Σ t_k × value_k`; LUTs blend by the same weights. The displayed vector follows the target with a 250 ms time constant (`α = 1 − exp(−dt / 0.25)`), so a step across a border reaches 98% of the new look in 1 s (4 time constants).
- **Seam zones** render in the Folio genre's skin (Canon §5.8). Along the faces between a seam zone and its neighbors, a stitch decal runs on the wall and floor: stitches 60 mm long and 12 mm wide every 250 mm, in the seam genre's ink, so the player sees two pages sewn together.
- **Foreign sprites.** Damage and status VFX inside a bleed zone use that zone's genre sprites (§4.5).

### 4.5 Damage-type VFX language (`vfx.dmg.<type>`)

Every damage type owns one shape motif, its Canon §10.1 default color and a unique icon `art.icon.dmg_<type_suffix>` (doc 17 shows the icon, so color is never the only signal). Genres swap only the sprite set (`vfx.sprites.<genre_suffix>`: a sword-slash arc is a brush stroke in Wuxia and a scanline tear in Cyberpunk); color, motif, count, timing and icon never change. Emitters use doc 02 §8.7 integer fields.

**Dominant type and count.** Doc 02 §7.6 routes at most one VFX record per event. For `ev.damage_dealt` the record is the one whose `dominant_type` is the type with the largest `per_type` amount, ties to the earlier Canon §10.1 row; an event whose `per_type` amounts are all 0 (fully absorbed by ward) uses the type with the largest pipeline share instead and plays the ward ripple of §4.5a. The particle count is `max(1, floor_div(count × dom_bp, 10000))` with `dom_bp = floor_div(10000 × amount_dominant, Σ amounts)`. Example: a hit of 6,000 mHP fire and 4,000 mHP slash spawns `vfx.dmg.fire` with `dom_bp` 6000 and `floor_div(24 × 6000, 10000) = 14` particles. The other types show through the damage number's icon row (doc 05 §18), not through extra emitters.

|Record|Motif|Color|Count|Life (ticks)|Speed mm/s|Spread Bam|gravity_bp|drag_bp|Size mm|Decal|Icon|
|---|---|---|---|---|---|---|---|---|---|---|---|
|`vfx.dmg.slash`|crescent streak|`#C9CED6`|18|12|6000|4096|0|2000|40|thin cut line|crescent blade|
|`vfx.dmg.pierce`|forward puncture cone|`#E8E2C8`|10|10|9000|1024|2000|1000|24|round hole|arrowhead|
|`vfx.dmg.blunt`|ground ring and chunks|`#A68A64`|14|18|3000|16384|10000|3000|60|crack star|hammer|
|`vfx.dmg.fire`|rising tongues and embers|`#FF6A1A`|24|40|1500|8192|−3000|1500|80|scorch|flame|
|`vfx.dmg.frost`|shards and mist|`#9BE7FF`|16|30|2500|16384|5000|2500|50|rime|snowflake|
|`vfx.dmg.shock`|forked arcs|`#FFE14D`|8|8|12000|32768|0|0|30|none|zigzag|
|`vfx.dmg.acid`|droplets and bubbles|`#9AE13B`|20|36|2000|12288|10000|1000|40|pitting|bubbling drop|
|`vfx.dmg.radiation`|slow motes|`#2F6BFF`|12|60|500|32768|0|500|30|faint glow|three-blade fan|
|`vfx.dmg.sonic`|3 expanding rings|`#C8B6FF`|3|20|10000|0|0|0|200|none|wave arcs|
|`vfx.dmg.toxin`|drifting clouds|`#6B8E23`|10|48|800|32768|−500|3000|120|stain|vial drop|
|`vfx.dmg.radiant`|rays and glare|`#FFF4C2`|12|16|7000|32768|0|0|40|none|sun|
|`vfx.dmg.umbral`|inward wisps|`#5B2A86`|16|30|−2000|32768|0|2000|70|shadow stain|eclipse|
|`vfx.dmg.psychic`|kaleidoscope glyphs|`#FF5FD2`|8|24|1500|32768|0|1000|60|none|spiral eye|
|`vfx.dmg.arcane`|circles and triangles|`#3FE0D0`|10|24|2000|32768|0|1500|50|sigil burn|circled triangle|
|`vfx.dmg.true`|hairline ink stroke and paper tear|`#FFFFFF`|6|12|4000|2048|0|0|20|none|full stop|

```json
{ "id": "vfx.dmg.fire", "trigger": { "event": "ev.damage_dealt", "dominant_type": "dmg.fire" },
  "color": 16738842, "count": 24, "life_ticks": 40, "speed_mm_s": 1500, "spread_bam": 8192,
  "gravity_bp": -3000, "drag_bp": 1500, "size_mm": 80, "decal": "art.tex.decal.scorch",
  "icon": "art.icon.dmg_fire", "sprite_slot": 3 }
```

The `trigger` follows doc 02 §7.6. Each status also has a HUD glyph `art.icon.<status_suffix>` (for example `art.icon.bleeding`) that doc 17 frames by category, and each genre's Aether skin has `art.icon.aether.<genre_suffix>`.

**4.5a Ward feedback and status VFX.** `ev.ward_broken` plays `vfx.ward.break`: the ward shell shatters into 12 hexagonal paper flakes (life 30 ticks, speed 3000 mm/s, spread 32768 Bam) in `#3FE0D0`; a hit fully absorbed by ward plays `vfx.ward.ripple`, a 6-tick hex ripple at the impact point. Status VFX (`vfx.status.*`) use the manuscript's own typography over the bearer: `st.stunned` orbiting asterisks, `st.sleeping` a rising ellipsis, `st.confused` an interrobang, `st.charmed` a rose fleuron, `st.feared` a trembling outline, `st.marked` a manicule above the head plus doc 17's through-wall outline, `st.dread` an ink vignette on the bearer's screen, `st.inspired` a gold underline at the feet, `st.unmoored` desaturation with fraying paper edges on the body, `st.erasure` one struck-through line crawling over the outline per stack (the outline turns Red Ink at the bearer's hunter threshold). `st.madness` draws no marker of its own: doc 04 §12.4's madness phantoms render with the full presentation of the native template they imitate, so madness stays deceptive (DECISIONS 49), and only doc 17's `access.mark_hallucinations` (forced on under Scribe's Mercy) adds its dashed outline and "?" glyph. Body emitters show `st.burning`, `st.frozen` (an ice shell), `st.shocked` (an arc every 30 active ticks), `st.bleeding` (drips) and `st.poisoned` (green breath).

### 4.6 Stealth readability

The rule: **nothing on screen may tell the player they are hidden, lit, heard or unseen when the simulation says otherwise.**

1. **Light.** Doc 02's agreement clamp makes visual darkness equal simulation darkness. Fixture and emissive colors never add luminance past the clamp.
2. **Fog.** Visual fog equals simulated concealment (§3.2).
3. **Dark vision.** Inside the local Erratum's dark-vision radius (Canon §6.4 row 31, 4 m at SGT 40) the scene gets a desaturated blue-grey luminance floor of `ambient_milli(4) = 22` from doc 02's table. It changes only the local player's image and never makes a cell look lit.
4. **Awareness glyphs** (`vfx.aware.*`) draw above each observer listed in the local slot's `StealthHud.marks` (doc 09 §8: that observer's state and meter toward the local Erratum) whose state ordinal (doc 09 §6.1) is at least 1 and that the local player can see. Shapes and colors match doc 17 §10.10's crosshair rings, so the two never disagree: `vfx.aware.curious` (ordinal 1) a thin white "?" that fills bottom-up with the mark's `meter` toward the next threshold; `vfx.aware.suspicious` (2) a bold yellow "?"; `vfx.aware.searching` (3) an orange "?" inside a rotating sweep ring; `vfx.aware.lost` (4) a hollow orange "!" inside a rotating sweep ring; `vfx.aware.alert` (5) a solid Red Ink "!". `aware.unaware` draws nothing. Glyphs never draw through walls unless the observer bears `st.marked`.
5. **Stealthy and invisible NPCs** (doc 09 §4.8) render as a refraction shimmer at doc 17's `access.shimmer_bp` opacity (default 2500 Bp) until revealed to the local Erratum, then as the full model with a manicule marker.
6. **Sound ripples** (doc 17 accessibility option): each `ev.noise` the local slot heard draws a ring at its apparent position (doc 02 §9) with radius `min(3000, 50 × L_lis)` mm for 30 ticks. Unheard noises draw nothing.
7. **Footsteps.** Hostile footsteps inside the footstep-indicator radius (Canon §6.4 row 34) play their sound even when the voice cap is full (§6.6).
8. **Human observers (invasions, Canon §17.9).** An invader's client never receives ghosts (doc 16 §6) and draws an Erratum whose `ViewEntity.flags` has INVISIBLE as the refraction shimmer at a fixed 2500 Bp, ignoring the invader's own `access.shimmer_bp`, so no setting makes invisible Errata easier to see. Errata on the invader's client always draw their silhouette outline (weight 1.0) in their origin ink, which is the same rule as for every foreign entity (§2.1 rule 4); the §4.7 luminance rule keeps cosmetics from camouflaging them. The invader itself always wears the Editor overlay of §3.4.

### 4.7 Telegraphs and cosmetic readability limits

- **Telegraphs** (doc 13 `ViewTelegraph`): the shape fills with the local `danger` swatch at 45% opacity under 45° hatching 6 px apart in the telegraph edge color (§3.1), with a 3 px edge line that pulses at 4 Hz in the final 30 ticks before `end_tick` (2 Hz under doc 17's `access.photosensitive`, which allows nothing above 3 Hz). Hatch angle and pulse are global, so a colorblind player reads timing from the pattern.
- **Cosmetic readability limits** (`art.readability.cosmetic`, requested by doc 15 §7.1) are checked by the asset lint on every `cosm.` mesh, material and effect:

|Rule|Limit|
|---|---|
|Body envelope|every vertex within the body capsule (Canon §4.1) inflated by 150 mm; back items up to 400 mm behind; nothing more than 200 mm above the head|
|Role mimicry|no cosmetic adds a shield plate, weapon line or raised-hand emissive that matches a §3.6 role rule|
|Red Ink and danger|no more than 20% of a cosmetic's albedo area within CIEDE2000 ΔE 10 of Red Ink or any genre's `danger` swatch; no hatch pattern|
|Emissive|≤ 150 milli luminance and ≤ 5% of surface area|
|Luminance (anti-camouflage)|the area-weighted mean relative luminance of a cosmetic's albedo lies within ±150 milli of the default skin it replaces, so no cosmetic hides an Erratum in a dark genre from an invader|
|Trails and auras|≤ 20 particles per second; off while crouched and while the wearer has `st.hidden`|
|Sound|cosmetics emit no sound into any channel that `ev.noise` uses; footstep audio comes only from materials|
|Inkwell reveals|at most 180 ticks, skippable after 30, no coin sounds, reels or near-miss frames (doc 15 AP-7)|

## 5. Animation

### 5.1 First-person viewmodel

- The viewmodel is a separate camera layer (doc 02 §8.4) and always animates on 1s, whatever the genre's `anim_step`, because aim feel outranks style. The third-person toggle (DECISIONS 143) moves only the viewpoint: hit detection is identical in first and third person, and the viewmodel is hidden in third person.
- **Action sync.** Each action clip has an authored length `clip_ticks`. It plays at rate `clip_ticks / action_ticks`, where `action_ticks` is the simulation's duration (Canon §6.4 row 17 for reload, swap and use). Example: a 120-tick reload with SLT 60 gives `mult = 10000 + 20 × 50 = 11000` and `action_ticks = max(1, ceil_div(120 × 10000, 11000)) = 110`, so the clip plays at rate 120 / 110 ≈ 1.091 and ends on the tick the simulation finishes.
- **Motion** from `art.vm.<archetype>` (example `art.vm.ranged.pistol`, for doc 05's `arch.ranged.pistol`: `bob_v_mm` 6, `bob_h_mm` 10, `sway_max_mm` 25, `sway_return_ticks` 7). Bob amplitude scales with `speed / 4300`, and its low point lands on each `ev.footstep` of the local player, so steps and bob agree. Sway lags look input by at most `sway_max_mm`, then springs back. Recoil kick and hitstop are doc 05's.
- **Full-body awareness**: doc 02 §8.4 renders the avatar body headless; looking down shows the legs, held off-hand items and the belt quickbar.
- **Non-humanoid Echoes** (Canon §16.2, DECISIONS item 28): `echo.fenrir` (`body.quadruped`) shows the lower edge of the muzzle and both forepaws, and its natural-weapon actions (bite, claw, pounce) play muzzle and paw clips; `echo.jormungandr` (`body.serpentine`) shows the lower jaw and the front coil, with bite, constrict and spit clips. Both keep the eye at doc 04 §13's eye height (1,100 mm for the quadruped, 1,000 mm for the serpentine), bob from their own gait (§5.2 layers) instead of footstep bob, and use rigs `art.rig.quadruped` and `art.rig.serpentine` within the Errata budget of §10.2. Held items follow doc 04 §13: Fenrir's one `slot.weapon_1` item renders in the `jaw` socket of `art.rig.quadruped`; Jörmungandr's `slot.hand_extra_1` and `slot.hand_extra_2` items render in the `grip_1` and `grip_2` sockets of `art.rig.serpentine`, two prehensile coil tips that grow from the front coil (doc 04's `bp.arm_extra_1` and `bp.arm_extra_2`), and the viewmodel shows a grip only while it holds an item.
- **Third person** (doc 02 §8.4 spring arm): the viewmodel layer is hidden and the full avatar plays the same action clips at the same `clip_ticks / action_ticks` rate, on the genre's `anim_step` like any other entity except for the §5.3 combat exemption.

### 5.2 Procedural creature animation

Skeletal clips are chosen by the simulation's `(anim, anim_tick)` (Canon §9): the clip named by the `art.anim.*` ID plays at `anim_tick`. Procedural layers add on top, client-side only:

|Body plan|Layer|
|---|---|
|humanoid, quadruped, avian, construct|foot IK: a downward ray from each hip, up to 1.5 × leg length, on the client's `ChunkView` blocks; two-bone analytic IK; pelvis drops by the lowest foot's offset, at most 0.4 × leg length; quadruped body pitch = `atan2(front_feet_y − rear_feet_y, body_length)` clamped to ±20°|
|insectoid|tripod gait: legs {1, 4, 5} and {2, 3, 6} alternate; a foot steps when it is farther than 0.35 × reach from its rest point; a step lasts 8 ticks at walk|
|serpentine|24 spine segments follow a buffer of root positions sampled every 100 mm of travel|
|amorphous|blend-shape wobble driven by a spring on velocity: `wobble_hz_milli` 3000, `damping_milli` 400|
|swarm|24 to 64 cosmetic boids within 1500 mm of the swarm entity's origin; count = `24 + floor_div(40 × hp_bp, 10000)`; boid seeds `pres_hash("art.swarm", entity_id, i)`|
|every plan|head look-at toward the AI target or nearest Erratum within ±60° yaw and ±30° pitch, at most 240° per second; a 12-tick additive flinch along each `ev.damage_dealt` direction|

**Readability guard.** No procedural layer may move a visible body part more than 150 mm from the pose-table capsule that the simulation uses for hits; the IK solver clamps to it. Players aim at what they see, and what they see is within 150 mm of what the simulation hits.

### 5.3 Animation step

A genre's `anim_step` (§3.3) holds skeletal poses on 2s or 3s: `t_disp = anim_tick − (anim_tick mod step)`. Root motion and position still interpolate every frame (doc 02 §7.3). Examples: `anim_tick` 37 shows pose 36 on step 2 and pose 36 on step 3; `anim_tick` 38 shows 38 on step 2 and 36 on step 3. A foreign entity uses its origin genre's step. Hitboxes always use the true `anim_tick`.

**Combat exemption.** Clips tagged `combat` (every attack windup, active and recovery clip, blocks, parries, dodges and hit reactions) always play on 1s. A held pose lags the hitbox by up to 2 ticks, and a sword tip moving 10 m/s travels 333 mm in 2 ticks, which would break the 150 mm readability guard of §5.2 exactly when the player is reading an attack. Stepping therefore applies to idle, locomotion, emotes and deaths only.

### 5.4 Render-only ragdolls (`vfx.ragdoll`)

- On `ev.entity_killed` the skinned body becomes a `bevy_rapier3d` ragdoll (doc 02: at most 16, oldest removed). Bodies per bone take the skinned pose and each bone's velocity from the last two frames, plus a kill impulse along the killing hit's direction:

```
dv_mm_s = min(8000, 2000 + floor_div(6000 × prev_hp_bp, 10000))     // prev_hp_bp = ViewEntity.hp_bp on the previous frame
explosion: dv_mm_s += floor_div(6000 × max(0, r_mm − d_mm), r_mm)    // radial, from ev.explosion
```

  Examples: a target at 3000 Bp before the killing hit flies at 3800 mm/s; one killed from full HP at 8000 mm/s. The total impulse is capped at 14,000 mm/s (8,000 from the first line plus at most 6,000 from an explosion at its center).
- A ragdoll lives 1800 ticks or until doc 02 despawns its corpse, sleeps when at rest, collides only with the client's block colliders and other ragdolls, and never writes anywhere the simulation reads. When the simulation spawns a corpse entity, doc 02 §7.4's cosmetic spring keeps the ragdoll root within 500 mm of the corpse position, so the lootable corpse and the visible body stay together. Severed parts and debris count toward doc 02's cap of 256 cosmetic rigid bodies; at the cap the oldest is removed.
- **Severing** (Canon §9.1): the part detaches as its own rigid body cut at the joint, with an ink-cap decal on both cut faces. The `cat.body_part` item keeps its own visual record.
- **Gore modes** (doc 17 `access.gore`; DECISIONS 94: ink gore by default, with a toggle to red blood). `ink`, the default: wounds bleed ink, so blood decals and mist render in the local genre's `ink` swatch (a foreign entity's in its origin genre's), and a severed part unravels into script: for 120 ticks after it detaches, its cut faces shed handwritten letters as glyph particles in that ink, 4 per second, while the part itself stays visible and lootable (`cat.body_part`, Canon §9.1). `full`: blood decals and mist in deep red `#8A1010`, and no script. `minimal`: no decals, mist or script; severing still shows the ink-cap. The rating target (PEGI 16, ESRB M) is doc 15's.

### 5.5 Going Native forms (`art.native.<genre_suffix>.<kin_suffix>`)

Going Native (Canon §5.11, doc 04 §2.8) switches an Erratum's Origin to genre `g` on the interact tick. Presentation follows on the next rendered frame:

1. The Erratum's `ViewEntity.origin_genre` becomes `g`, so it renders in `g`'s `render_style`, ink, `outline_px` and `anim_step` and no longer reads as foreign inside `g` (§2.1 rule 4).
2. The costume layer `art.native.<g suffix>.<kin suffix>` replaces the body's base costume: one outfit mesh, one hair or headgear mesh and one accessory set per genre and kin, 16 × 6 = 96 records, inside the Errata budget of §10.2. Equipped gear meshes and cosmetics stay on top; gear is not translated (Canon §5.11).
3. The kin skin name switches to doc 04 §2.4's name for `g` (for example a construct goes from "Synth" to "Bronze Warrior" on Going Native in Wuxia).
4. The page-turn ink-bleed of §4.3 plays over the Erratum only, for 60 ticks, in `g`'s ink.

```json
{ "id": "art.native.wuxia.construct", "genre": "genre.wuxia", "kin": "kin.construct",
  "outfit": "art.mesh.native.wuxia.construct_outfit", "head": "art.mesh.native.wuxia.construct_head",
  "accessories": ["art.mesh.native.wuxia.jade_talisman"], "palette_bias": "art.palette.wuxia" }
```

### 5.6 Appearance presets (`art.preset.<kin_suffix>.<nn>`)

DECISIONS 97 gives the character creator 12 presets per kin with palette and marking colors, and no slider editor. Doc 04 §2.1's Cosmetics step offers them on the Custom path, doc 17 §5.7 draws the screen, and this chapter authors them.

- **Records.** For each of the 6 Canon §16.3 kin, `art.preset.<kin_suffix>.<nn>` with `nn` from `01` to `12` (72 records) fixes a head (face, hair or headgear), a build and a base costume on `art.rig.humanoid` within the Errata budget of §10.2, plus a marking mask `art.tex.marking.<kin_suffix>.<nn>`. Every preset reads as its kin: beastkin presets carry ears and the `bp.tail` mesh (Canon §16.3), constructs show plating seams, undead show pallor and stitching, fae show pointed ears, and mutants show one asymmetric growth.
- **Colors.** `art.preset.<kin_suffix>.colors` lists 8 base swatches (skin, fur or plating tones across the kin's natural range) and 8 marking swatches (paint, tattoo, fur pattern, circuit inlay, stitch or sigil colors) for all 12 of its kin's presets. An Erratum's look is `Appearance { preset: Id32, base: u8, marking: u8 }` with `base` and `marking` in `0..=7`, so each kin offers 12 × 8 × 8 = 768 looks.
- **Limits.** Preset `01` with both swatch indices 0 is the kin's default skin. Every look passes §4.7's body envelope, role mimicry and emissive rules; its base costume passes the luminance rule against preset `01`'s; and no marking swatch lies within CIEDE2000 ΔE 10 of Red Ink or of any genre's `danger` swatch. Base swatches are exempt from the luminance rule so that skin tones cover the full human range; the outline every Erratum draws on an invader's client (§4.6 rule 8) keeps a dark look visible.
- **Style.** A preset renders in its Erratum's origin style like the rest of the body (§2.1 rule 4). Going Native replaces only the base costume with its `art.native.<g>.<kin>` layer (§5.5) and keeps the head, build and colors. Echoes fix their own looks and skip the step (doc 04 §2.1).
- **Presentation only.** The choice changes no number (doc 04 §2.1) and never enters simulation state, the simulation sections of saves, the replay command log or `content_hash`; the records are hashed into `presentation_hash` (§1.1).

## 6. Audio

### 6.1 Music system (`aud.music.<genre_suffix>`)

Each genre has one track of five sample-aligned looping stems, `bed`, `pulse`, `tension`, `combat`, `apex`, selected by doc 02 §9's music levels 0 to 4. Files are `aud.music.<g>.<stem>` (Opus, 128 kbit/s stereo, 48 kHz). Bars are whole ticks so level changes land on simulation-time bar lines:

```
bar_ticks   = beats_per_bar × 3600 / bpm        // lint: must divide exactly
bar_samples = bar_ticks × 800                   // 48 kHz / 60 Hz
loop_bars   = 64 if bar_ticks ≤ 160 else 32
loop_ticks  = bar_ticks × loop_bars
```

`bpm` counts the notated beat. In compound meter the beat is the dotted quarter, so Weird West's 6/8 has `beats_per_bar` 2 and `bar_ticks = 2 × 3600 / 72 = 100`.

|Track|bpm|Meter|bar_ticks|Mode, tonic|bed|pulse|tension|combat|apex|
|---|---|---|---|---|---|---|---|---|---|
|mythic_fantasy|90|4/4|160|dorian, D|low strings, horn pad|frame drum|choir "oo"|brass ostinato|choir and timpani|
|gothic_horror|72|3/4|150|harmonic minor, C|organ drone|harpsichord arpeggio|string-quartet tremolo|organ and timpani|full choir and organ|
|cosmic_horror|60|5/4|300|locrian, B|sub drones|heartbeat bass drum|detuned strings|theremin, brass clusters|choir cluster|
|space_opera|120|4/4|120|lydian, E|synth pad, strings|synth arpeggio|brass swells|orchestral percussion, brass|choir, full orchestra|
|cyberpunk|100|4/4|144|phrygian, F♯|analog pad|arpeggiated bass|industrial noise|drum machine, distorted bass|lead synth|
|post_apocalypse|96|4/4|150|minor pentatonic, E|feedback guitar drone|junk percussion|bowed metal|distorted riff, kit|full band|
|weird_west|72|6/8|100|mixolydian, A|slide guitar|shaker, hoofbeat|whistle, bowed saw|banjo, snare, trumpet|choir "ah", full band|
|hardboiled_noir|75|4/4|192|blues minor, B♭|rain piano|walking bass|muted trumpet|big-band stabs|full brass|
|wuxia|90|4/4|160|gong pentatonic, G|guzheng|woodblock|erhu|taiko, dizi|full ensemble|
|steampunk|144|4/4|100|mixolydian, B♭|low brass chorale|clockwork ticks|calliope|brass-band march|brass and organ|
|fairy_tale|135|3/4|80|lydian, F|celesta|pizzicato waltz|woodwinds in minor|orchestral waltz|choir|
|mythic_antiquity|80|4/4|180|phrygian, E|lyre|frame drum|aulos|chant, war drums|full chorus|
|oneiric|72|3/4|150|whole-tone, C|reversed piano|music box|choir pads|reversed percussion|forward and reversed together|
|redacted|120|4/4|120|atonal sine clusters|office hum at 60 Hz|typewriter and clock|sine clusters|shredder rhythm, bass|fax noise, choir|
|unwritten|60|4/4|240|none|room tone|pencil scratching|one sine|eraser percussion|page tearing|
|ur_text|90|4/4|160|dorian, D|organ and choir drone|quote|quote|quote|full counterpoint|
|margin (hub)|60|3/4|180|major, F|quill-and-paper foley, harp|none|none|none|none|

- `silence_ratio_bp` gates the bed in quiet genres: cosmic_horror 3000, hardboiled_noir 2000, unwritten 9000, others 0. While the music level is 0, phrase `p` (8 bars counted from Folio entry, `p = floor_div(t − folio_entry_tick, 8 × bar_ticks)`, never wrapped by the loop) mutes the bed iff `pres_hash("aud.silence", folio_index, p) % 10000 < silence_ratio_bp`. At levels 1 to 4 the bed always plays. The Unwritten's 9000 realizes Canon §5.9's "silence": nine phrases in ten are room tone only at level 0.
- **Ur-Text counterpoint** (Canon §5.9 "every register in counterpoint"): each core genre has three 8-bar quote stems `aud.music.ur_text.quote.<core_genre_suffix>.<stem>`, `stem` ∈ {`pulse`, `tension`, `combat`}, re-arranging that genre's motif at 90 bpm in D dorian (36 files of 1,280 ticks). In phrase `k` (`k = floor_div(t − folio_entry_tick, 1280)`, counted from Folio entry and never wrapped by the 10,240-tick loop) the Ur-Text track replaces its own `pulse`, `tension` and `combat` stems with the quote stems of genre `history[k mod n]`, where `history` lists the genres of every Folio before the final one in order with secret genres skipped, and `n = len(history)`. Folio 1 is always a core genre (doc 14 rule F1), so `n ≥ 1`. Example: history (Wuxia, Noir, Wuxia) gives phrases Wuxia, Noir, Wuxia, Wuxia, Noir and so on.
- **Editor overlays** `aud.music.editor.<boss_suffix>`: an unpitched Red Ink motif of three hits (`a`, `b`, `stamp`, files `aud.music.editor.<boss_suffix>.<hit>`, 15 one-shots) that plays in every 4th bar at level 4 in Editor Folios, at bar offsets 0, `floor_div(bar_ticks, 4)` and `floor_div(bar_ticks, 2)`, so it fits every meter and tempo without transposition. Timbres: the Proofreader red-pen scratch and rubber stamp, the Censor marker squeak and bar thud, the Continuity Editor clock tick and date stamp, the Abridger scissor snip and guillotine slam, the Last Word all four layered. In phase `p` (1 to 4) the Last Word plays the motif of `boss.editor_v<p>`.
- **Stingers** `aud.music.sting.*` play on the next beat: `folio_enter`, `colophon_open`, `editor_phase`, `player_downed`, `run_victory`, `run_erased`.
- **Level 4** (`apex`) plays while doc 02 §9 sets it: an alerted `rank.boss`, `rank.editor` or `rank.hunter` entity, an invader included. The softer Erasure hunter keeps that cue, because it carries `rank.hunter` for presentation under `rank.elite`'s multipliers (Canon §11.6, DECISIONS 91).
- Level timing (doc 02 §9): a rise applies at the next bar line; a fall applies at a bar line once the lower level has held 8 s. Example: Gothic Horror with the track started at Folio entry tick 0 and a rise computed on tick 1000 applies at tick 1050 (bar 7 starts at 7 × 150).

### 6.2 Bleed crossfades

Each track keeps its own clock: `track_position = (t − folio_entry_tick) mod loop_ticks` of that track, so a returning genre resumes in place. The listener genre is `local_genre` (Canon §5.8) of the local camera's block, read from the bleed field. Crossing into a bleed zone fades the bleed track in and the Folio track out over 2 bars of the incoming track with equal-power gains `g_in = sin(πx/2)`, `g_out = cos(πx/2)`, `x` from 0 to 1. At `x = 0.5` both are 0.7071 (−3.01 dB). Inside a seam zone both tracks hold at 0.7071 at the same level. Example: stepping into a Weird West bleed from Gothic Horror fades over 2 × 100 = 200 ticks.

### 6.3 SFX taxonomy

IDs: `aud.sfx.<family>.<subject>` with variants as numbered files; sets for skins as `aud.set.<archetype_suffix>.<genre_suffix>` (a firearm archetype's set is a crossbow twang in Fairy Tale where doc 11's skin is a crossbow).

|Family|Subject|Trigger|
|---|---|---|
|`wpn`|fire, mech, reload, swap, jam, dry, charge, overheat|doc 05 and doc 06 action events via doc 02 §7.6|
|`imp`|damage type × material class|`ev.damage_dealt` (dominant type, §4.5), projectile hits|
|`foot`|gait × material class|the `ev.noise` of a `noise.step.*` or `noise.move.land`, joined with the same tick's `ev.footstep` or `ev.entity_landed` of the same emitter for its `material`; the `ev.footstep` itself plays nothing|
|`hit`|confirmation cues for the local attacker: crit, precision, kill, sever, stagger, ward break|`ev.damage_dealt` flags, `ev.part_severed`, `ev.ward_broken` (table below)|
|`body`|effort, pain, death, per voice set|`ev.damage_dealt`, `ev.entity_killed`|
|`mag`|cast, release, fizzle per register (doc 06 §2.2)|`ev.spell_cast`|
|`env`|liquid, gas, fire, block break and place, door, explosion|`ev.liquid_reaction`, `ev.fire_started`, `ev.fire_out`, `ev.block_changed`, `ev.explosion`|
|`status`|apply, expire per status|`ev.status_applied`, `ev.status_expired`|
|`ui`|page, quill, seal, ledger|doc 17 widgets|
|`amb`|beds per tileset (`aud.amb.<tileset>`)|Folio and zone|

**Material class.** Audio uses doc 09 §7.2's acoustic class of a material (`noise.surface.<class>`, 17 classes, with doc 07's `acoustic` override), so a footstep sounds like the material whose loudness the simulation used. Footstep and landing sets are `aud.sfx.foot.<gait>.<class>` and impact sets `aud.sfx.imp.<type_suffix>.<class>`, where `<class>` is the suffix after `noise.surface.`; a feet cell holding ≥ 1 unit of `liq.mud` selects `mud` and any other liquid selects `water` (doc 09 §7.2). An impact on an entity uses its body plan's class: `soft` for flesh plans, `metal` for `body.construct`, `paper` for `fac.editors` members. Every (gait, class) and (type, class) pair has a set; a missing one fails the lint. Variant choice: `pres_hash("aud.var", tick, entity) % n`, re-drawn once by adding 1 to the tick input if it equals the last variant played on that emitter. Each play varies pitch by `±pitch_var_cents` (default 50) and gain by `±gain_var_cdb` (default 15, i.e. 1.5 dB). Voice sets are `aud.set.voice.<name>`.

```json
[
  { "id": "aud.music.gothic_horror", "bpm": 72, "beats_per_bar": 3, "bar_ticks": 150, "loop_bars": 64,
    "tonic_pc": 0, "mode": "harmonic_minor", "silence_ratio_bp": 0,
    "stems": ["aud.music.gothic_horror.bed", "aud.music.gothic_horror.pulse", "aud.music.gothic_horror.tension",
              "aud.music.gothic_horror.combat", "aud.music.gothic_horror.apex"] },
  { "id": "aud.sfx.imp.fire.soft", "trigger": { "event": "ev.damage_dealt", "dominant_type": "dmg.fire", "material": "noise.surface.soft" },
    "bus": "aud.bus.sfx", "pool": "combat", "priority": 5, "variants": 6, "pitch_var_cents": 50, "gain_var_cdb": 15,
    "gain_cdb": -300, "max_distance_mm": 40000 },
  { "id": "aud.sfx.foot.walk.stone", "trigger": { "event": "ev.noise", "noise": "noise.step.walk", "material": "noise.surface.stone" },
    "bus": "aud.bus.sfx", "pool": "stealth", "variants": 8, "pitch_var_cents": 50, "gain_var_cdb": 15,
    "caption": "loc.aud.caption.footsteps" }
]
```

**Cue map** (doc 05 §18's request). Cues triggered by an `ev.noise` follow §6.4 and are heard by every slot the simulation says heard them. Confirmation cues (`aud.sfx.hit.*`) play only for the slot that caused the hit, non-positional on `aud.bus.ui`, because they report the player's own action and carry no stealth information.

|Record|Trigger (event or noise)|Pool|Caption key|
|---|---|---|---|
|`aud.sfx.wpn.clang`|`noise.melee.clang` (70 dB)|combat|`loc.aud.caption.clang`|
|`aud.sfx.wpn.parry`|`noise.melee.parry`|combat|`loc.aud.caption.parry`|
|`aud.sfx.wpn.block`|`noise.melee.block`|combat|`loc.aud.caption.block`|
|`aud.sfx.wpn.jam` (skinned per `aud.set.<archetype_suffix>.<g>`)|`noise.weapon.jam` (40 dB)|stealth|`loc.aud.caption.jam`|
|`aud.sfx.mag.fizzle` (per register)|`noise.spell.fizzle`|stealth|`loc.aud.caption.fizzle`|
|`aud.sfx.wpn.dry`|`noise.weapon.dryfire` (30 dB)|stealth|`loc.aud.caption.dry_fire`|
|`aud.sfx.wpn.reload`|`noise.weapon.reload`|stealth|none|
|`aud.sfx.wpn.swap`|`noise.weapon.swap`|stealth|none|
|`aud.sfx.wpn.spark`|the gadget spark's own `noise.*` (Canon §5.5, doc 07)|stealth|`loc.aud.caption.spark`|
|`aud.sfx.hit.ward_break`|`ev.ward_broken`, for the hit's owner|ui|`loc.aud.caption.ward_break`|
|`aud.sfx.env.ward_break`|`ev.ward_broken`, positional for every other slot|combat|`loc.aud.caption.ward_break`|
|`aud.sfx.hit.crit`|`ev.damage_dealt` with flag `crit`|ui|none|
|`aud.sfx.hit.precision`|`ev.damage_dealt` with flag `precision`|ui|none|
|`aud.sfx.hit.kill`|`ev.damage_dealt` with flag `lethal`|ui|none|
|`aud.sfx.hit.sever`|`ev.part_severed`|ui|none|
|`aud.sfx.hit.stagger`|`ev.status_applied` of `st.staggered` on a target the local slot hit on that tick|ui|none|

Every `noise.*` ID in doc 09 has at least one audio record (lint, acceptance tests), and every record whose sound a deaf player needs carries a `caption` text key (doc 17 §13.2).

### 6.4 Occlusion from the simulation sound grid

Doc 02 §9 owns the formulas. Every sound with an `ev.noise` plays only for party slots whose `heard[slot]` is set, at the heard level, low-passed by the wall loss `extra` and placed at the apparent position, so the player hears exactly what the simulation says a listener there hears. Doc 02's example stands: a 130 dB shot 10 cells away behind a closed oak door arrives at 95 dB, filtered at 8,434 Hz, 4 m away toward the doorway. `noise.*` IDs (doc 09) map to sound records through `trigger.noise`. Sounds without an `ev.noise` use doc 02's presentation estimate and never mislead the player on stealth: they are quieter than any `ev.noise` of the same source class by at least 6 dB.

### 6.5 Voice-over plan

|Speaker group|Lines|Voiced|Notes|
|---|---|---|---|
|The five Editors|700 (140 each)|yes|intros, phase lines, one line per verse per attack, kills, deaths|
|Revisor hunters|80|yes|one actor; per-genre `voice_fx`|
|Margin residents (8)|160 (20 each)|yes|greetings, farewells and vendor acknowledgements only; their dialogue and arc beats are text (§8.4, DECISIONS 78)|
|The Narrator|0 (600 authored notes, text only)|no|DECISIONS item 38: narration is manuscript marginalia in pencil (§9.1)|
|Patron answers|0 (64 text lines, §8.2)|no|Authors never appear in person; answers are marginalia in the Author's genre ink|
|Echoes (24)|960 (40 each)|yes|short lines and barks; Fenrir and Jörmungandr speak in a processed human voice with a creature layer, as the myths have them speak|
|Genre mini-bosses (28)|840 (30 each)|yes|2 per genre that has mini-bosses (DECISIONS 69); doc 13 §9.3 holds the first, and the Unwritten and the Ur-Text have none|
|Native NPC efforts|1,280 (2 voice sets × 40 per genre)|non-verbal|spoken barks are text bubbles (`loc.npc.bark.*`)|
|Errata efforts|240 (4 voice types × 60)|non-verbal|grunts, breaths, pain, death|
|Non-humanoid Echo efforts|120 (wolf set and serpent set × 60)|non-verbal|Fenrir and Jörmungandr growls, hisses, pain, death|
|**Total**|**4,380**||87.6 MB at a 2.5 s mean, 64 kbit/s mono Opus (`4380 × 2.5 × 8000 = 87,600,000` B)|

- Recording: 48 kHz, 24-bit, mono; delivered at −20 LUFS per line ±2, true peak ≤ −3 dBTP.
- Launch VO is English; every locale has subtitles for every voiced line.
- Per-genre `voice_fx` on Editors, hunters and barks: gothic_horror gramophone, space_opera and cyberpunk comm radio, mythic_antiquity choir-double, redacted glitch, unwritten whisper; none elsewhere.
- Text only, never voiced: Narrator notes (DECISIONS 38), patron answers (§8.2), Margin dialogue and arc beats (DECISIONS 78), closing manuscripts (§8.5a) and every Scribe text (§9).
- No synthetic or cloned human voice ships (§10.3). Voiced lines are never rewritten by the Scribe.

### 6.6 Mix budget (`aud.mix.default`)

|Pool (`pool` field)|Bus|Gain dB|Voices (Deck / desktop)|Priority 0–7|Ducking|
|---|---|---|---|---|---|
|`voice`|`aud.bus.voice`|0|4 / 6|7|ducks music −6 dB (attack 100 ms, release 600 ms)|
|`stealth`: footsteps, landings, callouts, doors, alarms, reloads, jams, fizzles, dry fire|`aud.bus.sfx`|0|16 / 32|7|none|
|`combat`: impacts, clang, parry, block, explosions, telegraph sounds|`aud.bus.sfx`|−2|20 / 52|5|telegraph sounds duck music −3 dB|
|`ambience`|`aud.bus.ambience`|−10|6 / 10|2||
|`music`: two tracks of 5 stems during a crossfade, plus Editor hits and stingers|`aud.bus.music`|−8|14 / 24|6 (stems streamed)||
|`ui`: menus and `aud.sfx.hit.*` confirmations|`aud.bus.ui`|−6|4 / 4|4||
|**Total**|||**64 / 128** (doc 02 caps)|||

- Loudness: −18 LUFS integrated ±1 and true peak ≤ −1 dBTP over a 10-minute capture of doc 02's `bot_typical` scenario played with rendering and audio on.
- **Voice allocation.** Each pool owns its voices; pools never borrow from each other, and the pool sizes sum to doc 02's caps. A request takes a free voice of its pool. When the pool is full, a priority-7 pool (`voice`, `stealth`) stops its oldest voice (earliest start, ties lower emitter `EntityId`) and plays the request; every other pool replaces its quietest voice (lowest current gain at the listener, ties oldest) only when the request's gain at the listener is strictly higher, and otherwise drops the request. A combat sound therefore never takes a stealth-critical voice.

```
fn allocate(pool: &mut Pool, req: Request) -> Option<VoiceId> {
    if let Some(v) = pool.free_voice() { return Some(pool.start(v, req)); }
    if pool.priority == 7 {
        let v = pool.voices.iter().min_by_key(|v| (v.start_frame, v.emitter)).unwrap().id;
        pool.stop(v); return Some(pool.start(v, req));
    }
    let q = pool.voices.iter().min_by_key(|v| (v.gain_milli_db, v.start_frame)).unwrap();
    if req.gain_milli_db > q.gain_milli_db { let v = q.id; pool.stop(v); Some(pool.start(v, req)) } else { None }
}
```
- **Memory** (doc 02's 256 MiB): pre-decoded hot set of short and stealth-critical one-shots 48 MiB (i16 PCM, 524 s), decoded LRU cache 64 MiB, compressed Opus bank for the current Folio's genres 48 MiB, music ring buffers 4 MiB (10 stems × 2 s stereo i16 = 3.66 MiB), VO buffers 2 MiB, Steam Audio HRTF and reflections 48 MiB, mixer graph 16 MiB, headroom 26 MiB.

## 7. Text skins, registers and the name review

### 7.1 The skin naming rule and its review (`nar.review.banlist`)

Canon §5.2 forbids a Lucre or Aether skin that matches a real-world brand, a third-party game's currency or resource name, the display name of any `cur.` or `ent.` entry, or the word Ink. This chapter applies the review to every name in scope: every genre skin name (`loc.<id>.skin.<g>.name`: damage types, glyph forms, hazards, events, archetypes, kin, Revisor skins, appearance forms, curse states), every Lucre and Aether skin, procedural names (§7.4), manufacturer names and slogans, vendor names, doc 10 implant stamps and sigils, and invented signage. Base names such as "Red Ink" or "Silver" are outside the scope of tests 1 and 4, because the canon itself uses them. A name in scope **fails** when any test holds, compared case-insensitively on whole words with a trailing plural "s" ignored:

1. It equals the display name of a `cur.` or `ent.` entry (Lucre, Quires, Ink, Vellum, Quill), or contains the standalone word "Ink". Exemption: a currency's identity skin, the skin equal to its own base name (Unwritten "Lucre"; Canon §5.2).
2. It contains an entry of banlist group `brand`: trademarks, companies, products and real persons' names. The automated seed list below runs in the lint and the Scribe filter; the full screen is the trademark search export the legal reviewer attaches to each build.
3. It contains an entry of group `game_coinage`, a coinage that exists primarily as one third-party game's currency or resource.
4. Lucre and Aether skins only: it equals an entry of group `game_premium`, the name of a purchasable premium currency in a third-party game, even when it is also a common word.
5. It contains an entry of group `casino` (doc 01 §9.5 rule 5, doc 15 AP-7). This test also runs on every other `loc.` string.
6. Diegetic text only (flavor, Narrator notes, Folio intros, Editor lines, barks, grammar terminals, Scribe output): it contains an entry of group `nondiegetic` (doc 01 §9.5 rule 2). Menus and tooltips are exempt.

**Product-owner exemption.** The canon Mythic Fantasy Lucre skin "Crowns" passes tests 1 to 4 (DECISIONS 90; Canon §5.2, §22). The exemption covers that one skin: "Crown" or "Crowns" as any other Lucre or Aether skin still fails test 4.

**Eponym allowance.** A word that is an SI unit or a generic device name derived from a person passes test 2: tesla, volt, ampere, ohm, watt, galvanic, mesmeric, gatling, derringer, bowie. So steampunk's "Galvanic" and doc 06's "Tesla Relay" pass.

Common words and real or historical coin names pass otherwise. The lint `pal lint names` runs tests 1, 3, 4, 5 and 6 and the seed half of test 2 on every name in scope; the legal half of test 2 is a human review of every `loc.` string, recorded in the asset manifest (§10.3).

**Review list of forbidden names** (`nar.review.banlist`, maintained by the narrative lead; additions need no chapter change):

|Group|Entries (whole words, case-insensitive, trailing "s" ignored)|Why|
|---|---|---|
|`account`|Lucre, Quire, Ink, Vellum, Quill|Canon §5.2: display names of `cur.` and `ent.` entries and the word Ink|
|`game_coinage`|V-Bucks, Robux, Primogem, Glimmer, Bright Dust, Eddies, Nuyen, Septim, Zenny, Gil, Poké Dollar, Simoleon, Bottle Cap, Riot Point, Blood Echo, Orundum, Stellar Jade, Apex Coin, COD Point, Shark Card|coinages that exist primarily as one third-party game's currency or resource|
|`game_premium`|Crown, Silver, Platinum, Gem, Diamond, Crystal, Coin, Point, Token, Gold Bar, Credit Pack|common words that name a purchasable premium currency in a third-party game; Lucre and Aether skins only|
|`brand` (seed)|Pinkerton, Wells Fargo, Winchester, Remington, Colt, Smith & Wesson, Glock, Beretta, Coca-Cola, Pepsi, Marlboro, Rolex, Cadillac, Harley-Davidson, Zeiss, Nokia, Sony, Tesla Motors|real brands that genre settings tempt writers to name (frontier rifles, noir cars, cyberpunk signage)|
|`casino`|jackpot, spin, bet, wager, slot machine, lucky streak, payout, gamble, high roller|doc 01 §9.5 rule 5 and doc 15 AP-7|
|`nondiegetic`|level, loot box, gacha, respawn, buff, nerf, DPS|doc 01 §9.5 rule 2 (house lexicon)|
|`safety` (`nar.review.banlist.safety`)|slurs, sexual terms, self-harm terms, real-world political and religious names|§8.8; maintained privately, never shipped in plain text (stored as BLAKE3 hashes of the lowercased word)|

```json
{ "id": "nar.review.banlist", "version": 1,
  "groups": [
    { "group": "account", "scope": "names", "entries": ["lucre", "quire", "ink", "vellum", "quill"] },
    { "group": "game_coinage", "scope": "names", "entries": ["v-bucks", "robux", "primogem", "glimmer", "bright dust", "eddies", "nuyen", "septim", "zenny", "gil", "poké dollar", "simoleon", "bottle cap", "riot point", "blood echo", "orundum", "stellar jade", "apex coin", "cod point", "shark card"] },
    { "group": "brand", "scope": "names_and_scribe", "entries": ["pinkerton", "wells fargo", "winchester", "remington", "colt", "smith & wesson", "glock", "beretta", "coca-cola", "pepsi", "marlboro", "rolex", "cadillac", "harley-davidson", "zeiss", "nokia", "sony", "tesla motors"] },
    { "group": "game_premium", "scope": "currency_skins", "entries": ["crown", "silver", "platinum", "gem", "diamond", "crystal", "coin", "point", "token", "gold bar", "credit pack"] },
    { "group": "casino", "scope": "all_loc", "entries": ["jackpot", "spin", "bet", "wager", "slot machine", "lucky streak", "payout", "gamble", "high roller"] },
    { "group": "nondiegetic", "scope": "diegetic", "entries": ["level", "loot box", "gacha", "respawn", "buff", "nerf", "dps"] } ],
  "eponym_allow": ["tesla", "volt", "ampere", "ohm", "watt", "galvanic", "mesmeric", "gatling", "derringer", "bowie"] }
```

**Review of the 32 canon skins** (Canon §5.2):

|Genre|Lucre skin|Verdict|Aether skin|Verdict|
|---|---|---|---|---|
|mythic_fantasy|Crowns|passes by the product-owner exemption (DECISIONS 90); test 4 alone would fail it, because The Elder Scrolls Online sells Crowns|Mana|passes: generic resource word|
|gothic_horror|Shillings|passes: historical coin|Sanguine|passes|
|cosmic_horror|Bonds|passes|Gnosis|passes|
|space_opera|Credits|passes: generic, earned in the games that use it|Psi|passes|
|cyberpunk|Chits|passes|Bandwidth|passes|
|post_apocalypse|Tins|passes; "Bottle Caps" would fail test 3|Static|passes|
|weird_west|Eagles|passes: historical coin|Hex|passes|
|hardboiled_noir|Bucks|passes: slang for dollars|Nerve|passes|
|wuxia|Taels|passes: historical unit|Qi|passes|
|steampunk|Sovereigns|passes: historical coin|Pressure|passes|
|fairy_tale|Fairy Gold|passes: folklore|Glamour|passes|
|mythic_antiquity|Drachmae|passes: historical coin|Ichor|passes|
|oneiric|Motes|passes: generic word|Lucidity|passes|
|redacted|Vouchers|passes|Clearance|passes|
|unwritten|Lucre|passes by the identity exemption of test 1|Potential|passes|
|ur_text|Words|passes|Logos|passes|

- **Plain Inkwell cosmetics (doc 15 §3.1, §3.4)**: 60 Plain items (20 inks `cosm.ink.inkwell_01..=20`, 15 sigils `cosm.sigil.inkwell_01..=15`, 15 titles `cosm.title.inkwell_01..=15`, 10 emotes `cosm.emote.inkwell_01..=10`). Display names follow the standard title, ink tone, sigil mark and gesture nomenclature, localized in `assets/loc/<locale>/18_art.ftl`.

### 7.2 Damage-type text skins

Text keys `loc.dmg.<type_suffix>.skin.<genre_suffix>.name` (Canon §1.10). The archetype default names are Slash, Pierce, Blunt, Fire, Frost, Shock, Acid, Radiation, Sonic, Toxin, Radiant, Umbral, Psychic, Arcane, True. Within one genre all 15 names are distinct, and none equals that genre's Lucre or Aether skin with a trailing plural "s" ignored (so Weird West umbral is "Curse", not "Hex"). Steampunk's shock is "Galvanic" (Canon §10.1).

|Type|mythic_fantasy|gothic_horror|cosmic_horror|space_opera|cyberpunk|post_apocalypse|weird_west|hardboiled_noir|
|---|---|---|---|---|---|---|---|---|
|slash|Cleave|Rend|Flay|Vibro|Mono|Hack|Cut|Razor|
|pierce|Pierce|Impale|Puncture|Kinetic|Ballistic|Shot|Lead|Slug|
|blunt|Crush|Bludgeon|Concussion|Impact|Trauma|Bash|Bruise|Sap|
|fire|Flame|Pyre|Black Flame|Plasma|Thermal|Burn|Brimstone|Torch|
|frost|Rime|Grave-Chill|Void Cold|Cryo|Cryo|Freeze|Norther|Ice|
|shock|Lightning|Storm|Crawling Spark|Ion|Surge|Juice|Lightning|Live Wire|
|acid|Corrosion|Vitriol|Dissolution|Corrosive|Chemical|Acid|Lye|Battery Acid|
|radiation|Blight|Pestilence|Star-Sickness|Rad|Rad|Glow|Bad Glow|Hot|
|sonic|Thunder|Shriek|Piping|Sonic|Subsonic|Rattle|Thunderclap|Bang|
|toxin|Venom|Poison|Spore|Bio-Toxin|Nanotoxin|Rot|Snakebite|Poison|
|radiant|Holy|Sanctity|Unbearable Light|Photon|Flash|Flare|Gospel|Spotlight|
|umbral|Shadow|Unholy|Outer Dark|Dark Matter|Blackout|Grim|Curse|Shadow|
|psychic|Mind|Dread|Madness|Psionic|Neural|Shakes|Spook|Jitters|
|arcane|Arcane|Occult|Eldritch|Warp|Raw Data|Weird|Conjure|Uncanny|
|true|Doom|Fate|Inevitability|Absolute|Flatline|Last Call|Reckoning|Curtains|

|Type|wuxia|steampunk|fairy_tale|mythic_antiquity|oneiric|redacted|unwritten|ur_text|
|---|---|---|---|---|---|---|---|---|
|slash|Blade|Shear|Thorn|Sword|Paper Cut|Cut|Line|The Edge|
|pierce|Needle|Rivet|Spindle|Spear|Pin|Puncture|Point|The Point|
|blunt|Palm|Piston|Cudgel|Ram|Falling|Impact|Weight|The Weight|
|fire|Crimson Flame|Firebox|Hearth-Fire|Forge-Fire|Fever|Incineration|Heat|The Flame|
|frost|Cold Yin|Refrigerant|Snowbite|Northwind|Cold Sweat|Cold Storage|Cold|The Frost|
|shock|Heaven Lightning|Galvanic|Storm|Thunderbolt|Startle|Discharge|Spark|The Spark|
|acid|Rotting Dew|Vitriol|Gall|Hydra Bile|Melting|Solvent|Blot|The Rot|
|radiation|Fallen Star|Radium|Witchlight|Star-Fall|Glow|Contamination|Bleach|The Glow|
|sonic|Lion Roar|Resonance|Bell-Toll|War-Cry|Scream|Feedback|Noise|The Voice|
|toxin|Five Poisons|Arsenic|Poison Apple|Venom|Sick Feeling|Compound|Stain|The Bane|
|radiant|Golden Sun|Limelight|Dawn|Divine|Waking Light|Disclosure|White|The Light|
|umbral|Yin Night|Soot|Nightshade|Underworld|Deep Sleep|Redaction|Gap|The Dark|
|psychic|Heart Demon|Mesmeric|Enchantment|Mania|Nightmare|Reeducation|Doubt|The Mind|
|arcane|Pure Qi|Aetheric|Fey|Mystery|Surreal|Anomaly|If|The Spell|
|true|Heaven's Will|Entropy|Fate|Moira|Waking|Termination|Blank|The End|

### 7.3 Genre text registers (`nar.register.<genre_suffix>`)

Doc 01 §9.5 is the voice bible, with its sample lines. Each register record adds measurable limits that the grammar lint and the Scribe filter (§9.8) enforce. Profanity is 0 (none) or 1 (mild only), following doc 01 Q-14's default.

|Genre|Formality 0–4|Archaism 0–4|Profanity|Mean words per sentence|Person|Prefer|Never|
|---|---|---|---|---|---|---|---|
|mythic_fantasy|3|3|0|14|third|oath, realm, hearth, wyrm, road|okay, tech, cool|
|gothic_horror|4|2|0|16|third, polite second|sir, madam, vigil, grave, dreadful|okay, guys, awesome|
|cosmic_horror|3|1|0|12|first, as a journal|specimen, angle, measured, tide|monster, lol|
|space_opera|2|0|0|10|second, to the crew|hull, stars, sector, all hands|magic, thee|
|cyberpunk|1|0|1|7|second|chrome, corpo, signal, flatline|thee, verily|
|post_apocalypse|1|0|1|8|first plural|scrap, haul, clean water, before|archaic forms|
|weird_west|1|1|1|8|third|reckon, iron, trail, sundown|technical jargon|
|hardboiled_noir|2|0|1|9|first|rain, case, angle, dough|dame, magic words|
|wuxia|3|2|0|10|third|sect, rivers and lakes, junior brother, heaven|okay, cool|
|steampunk|3|2|0|15|third|splendid, apparatus, patent, boiler|computer|
|fairy_tale|2|2|0|12|third, as a teller|once, thrice, little, bargain|gore words, technical terms|
|mythic_antiquity|4|3|0|14|second, as an invocation|epithets, muse, bronze, fate|modern idioms|
|oneiric|2|0|0|9|second|always, again, somehow, stairs|definitely|
|redacted|4|0|0|11|passive third|pursuant, clearance, compliance, proceed|emotion words, personal names|
|unwritten|0|0|0|1|none|single words|sentences|
|ur_text|3|3|0|12|every register|word, first, beginning, page|none|

Every register also obeys: no gendered pronouns for procedural names (repeat the name or use "they"); no numbers, statistics or rules in flavor (§8.6); Editors' lines are style-guide rulings in any genre (doc 01 §1.3 rule 4); diegetic text uses doc 01 §9.5's house lexicon (Folio, Volume, the Margin, the Colophon, erased, Emendation, Rewrite, drawing ink, unsealing a Reliquary, a Fortune Engine dispenses), enforced by test 6 of §7.1. "Mean words per sentence" is the authoring target; the grammar lint enforces its ceiling: no sentence in a genre's grammars, authored lines or Scribe output exceeds 2 × the table value in words (2 words for the Unwritten).

### 7.4 Name grammars (`nar.names.<genre_suffix>`)

Doc 12 §20.1 draws `hero_name` and `rival_name` as `u64` name seeds. A name is generated with exactly 4 draws on `Rng::from_seed64(name_seed)`: `pattern = weighted([3, 1])`, `given = range(6)`, `family = range(6)`, `title = range(3)`; pattern 0 is `p0`, pattern 1 is `p1`. Titles are gender-neutral or pair with a family name. The Ur-Text first draws `range(12)` to pick a core genre's table, indexed in Canon §5.2 row order (0 mythic_fantasy to 11 mythic_antiquity), then the same 4 draws (5 in all).

|Genre|Given|Family or epithet|Titles|p0 / p1|
|---|---|---|---|---|
|mythic_fantasy|Aldren, Brisa, Corwen, Elowen, Garreth, Ysmay|of Hollowmere, Stormwright, the Unbowed, Ashborne, of the Nine Wells, Greymantle|Ser, Warden, Archmage|`{g} {f}` / `{t} {g}`|
|gothic_horror|Ambrose, Cordelia, Ephraim, Honora, Lucian, Wilhelmina|Ravensworth, Ashdown, Vane, Blackmoor, Strand, Mortlake|Lord, Mother, Doctor|`{g} {f}` / `{t} {f}`|
|cosmic_horror|Abner, Constance, Josiah, Lavinia, Obed, Prudence|Harrow, Crane, Wick, Ellery, Pike, Fenwright|Professor, Brother, Captain|`{g} {f}` / `{t} {f}`|
|space_opera|Ilyra, Kade, Mirelle, Orrin, Sava, Tavish|Halcyon, Drest, Korvath, Imre, Solano, Venn|Commander, Envoy, Admiral|`{g} {f}` / `{t} {f}`|
|cyberpunk|Juno, Kai, Nyx, Ozzy, Rhee, Vex|Okafor, Lindqvist, Sato, Varga, Moreau, Petrov|Zero, Static, Halo|`{g} {f}` / `{g} "{t}" {f}`|
|post_apocalypse|Ash, Dusty, Marrow, Rook, Sparrow, Tilly|Two-Wells, Ironsides, Saltbone, Halfmile, Rustwater, Nineteen|Boss, Doc, Old|`{g} {f}` / `{t} {g}`|
|weird_west|Abigail, Cass, Eli, Hattie, Jebediah, Silas|McCready, Stroud, Ballard, Tolliver, Harlan, Vasquez|Marshal, Preacher, Widow|`{g} {f}` / `{t} {f}`|
|hardboiled_noir|Frankie, Mae, Lou, Vivian, Eddie, Rita|Kowalski, Delacroix, Moran, Petrakis, Doyle, Fontaine|Detective, Mister, Miss|`{g} {f}` / `{t} {f}`|
|wuxia|Feng, Xue, Yun, Ming, Hua, Jian|Bai, Lan, Qiu, Shen, Yan, Zhou|Elder, Young Master, Sister|`{f} {g}` / `{t} {f}`|
|steampunk|Barnaby, Cornelia, Ezra, Florence, Gideon, Honoria|Gearwright, Pennyfeather, Thistlewaite, Coalhaven, Vantwistle, Mainspring|Professor, Captain, Doctor|`{g} {f}` / `{t} {f}`|
|fairy_tale|Elsbet, Jory, Marigold, Tobin, Rosamund, Hazel|Thrice-Lost, of the Glass Hill, Applewhite, the Youngest, Briarsong, Underhill|Goody, Little, Old|`{g} {f}` / `{t} {g}`|
|mythic_antiquity|Andreia, Damon, Eudora, Leontes, Melia, Theron|Bronze-Handed, Storm-Born, the Far-Wandering, Oath-Keeper, of the Hundred Ships, Laurel-Bright|Swift, Wise, Godlike|`{g} {f}` / `{g} the {t}`|
|oneiric|You, Someone, Mother, The Child, Your Teacher, A Stranger|Who Was Always There, Of the Stairs, Upside Down, With No Face, From Before, Of the Long Hall|Dear, Little, Old|`{g} {f}` / `{t} {g}`|
|redacted|Employee, Subject, Agent, Clerk, Witness, Applicant|████, No. 0417, (Name Withheld), Grade C, Pending, ██-7|Former, Acting, Deputy|`{g} {f}` / `{t} {g} {f}`|
|unwritten|Blank, Nobody, Unnamed, Nothing, Untitled, Empty|Draft, Line, Page, Mark, Space, Gap|The, A, Some|`{g} {f}` / `{t} {g}`|

Example (steampunk, doc 12 §20.5's seeds): `0x9c1f00d2a4be7731` draws `(0, 4, 1, 2)` → **Gideon Pennyfeather**; `0x13a4c0ffee0f1e2d` draws `(1, 4, 0, 1)` → **Captain Gearwright**. The same hero seed gives "Garreth Stormwright" in mythic_fantasy and "Lucian Ashdown" in gothic_horror.

### 7.5 Glyph Form skin names

Doc 06 §5.7 names every glyph class in all 15 casting genres and adopts this section's Form names unchanged for the nine genres below and the Ur-Text; this chapter owns these 130 Form strings and doc 06 owns the other classes of its 84 glyphs. The Unwritten has none: no glyph is generated or Rewritten there (doc 06 §2.2), so its glyphs show their origin skin. Doc 06's acceptance test 38 checks that no two glyphs share a skin within one genre.

|Form|gothic_horror|space_opera|post_apocalypse|weird_west|hardboiled_noir|
|---|---|---|---|---|---|
|bolt|Nail of Blood|Psi Dart|Zip Shot|Hex Bullet|Cheap Shot|
|orb|Clotted Orb|Focus Sphere|Junk Bomb|Cursed Coin|Tossed Bottle|
|beam|Evil Eye|Mind Lance|Arc Torch|Witch's Stare|Hard Stare|
|cone|Grave Breath|Psi Wave|Spray Rig|Spit Hex|Smoke Screen|
|touch|Corpse Touch|Mind Touch|Jumper Clamp|Hand of Glory|Sucker Punch|
|self|Within the Vein|Centering|Self-Wire|Rabbit's Foot|Second Wind|
|aura|Mourning Shroud|Psi Field|Static Halo|Dust Devil Mantle|Bad Reputation|
|field|Circle of Candles|Stasis Zone|Junk Field|Salt Circle|Turf|
|wall|Iron Gate|Barrier Plane|Scrap Wall|Bone Fence|Stonewall|
|rune|Hex Mark|Psi Mine|Tripwire Rig|Hex Sign|Setup|
|nova|Bloodburst|Mind Flare|Overload Rig|Brimstone Burst|Blowup|
|chain|Thread of Kin|Resonance Link|Daisy Wire|Hanging Rope|Domino|
|manifest|Raising|Projection|Scrapbot Build|Calling Up|Calling a Favor|

|Form|fairy_tale|mythic_antiquity|oneiric|redacted|ur_text|
|---|---|---|---|---|---|
|bolt|Wishing Dart|Far-Darting Arrow|Thrown Thought|Memo|First Letter|
|orb|Glass Bead|Thunderstone|Soap Bubble|Packet|Round Hand|
|beam|Moonbeam|Sun-Chariot Ray|Staring Too Long|Directive|Underline|
|cone|Dandelion Breath|Breath of the Winds|Sigh|Circular|Flourish|
|touch|Fairy Kiss|Anointing|Shoulder Tap|Stamp|Signature|
|self|Heart's Wish|Heroic Spirit|Pinch|Self-Assessment|First Person|
|aura|Rose Mantle|Aegis Mantle|Daydream|Clearance Badge|Illumination|
|field|Toadstool Circle|Sacred Precinct|Room That Shouldn't Be|Restricted Area|Paragraph|
|wall|Briar Hedge|Wall of Bronze|Locked Door|Cordon|Ruled Line|
|rune|Witch's Knot|Votive Mark|Note Under the Pillow|Pending Form|Initial|
|nova|Midnight Chime|Olympian Thunder|Sudden Waking|Mass Notice|Exclamation|
|chain|Silken Thread|Thread of the Fates|Déjà Vu|Chain of Custody|Ligature|
|manifest|Summoned Helper|Invocation of Heroes|Imaginary Friend|Requisition|Character Sketch|

Hazard and event skin names are doc 12 §18.2 and §19.2's parenthetical names, stored verbatim as `loc.<hazard id>.skin.<genre_suffix>.name` and `loc.<event id>.skin.<genre_suffix>.name`.

### 7.6 Appearance forms, curse states and Editor names

**Appearance form nouns** (`loc.idc.<class>.form.<genre_suffix>`, doc 10 §3.4). The 12 core genres use doc 10 §3.4's table verbatim. This chapter adds the four secret genres:

|Genre|Potion|Text|Ring|Amulet|Wand|Implant|Relic|
|---|---|---|---|---|---|---|---|
|oneiric|glass of milk|dream-diary page|ring|music-box key|conductor's baton|sleep stitch|nightlight|
|redacted|sample vial|memo|signet|lanyard badge|pointer|tracking chip|paperweight|
|unwritten|drop|page|ring|pendant|pencil|mark|thing|
|ur_text|ampulla|leaf|ring|seal|stylus|rubric|fragment|

**Curse-state skins** (`loc.buc.blessed.skin.<g>.name`, `loc.buc.cursed.skin.<g>.name`; Canon §13.6). `buc.uncursed` displays "—" in every genre, as doc 10 shows it, and its plain name "Uncursed" appears only in filter menus.

|Genre|Blessed|Cursed|Genre|Blessed|Cursed|
|---|---|---|---|---|---|
|mythic_fantasy|Blessed|Cursed|wuxia|Auspicious|Ill-Omened|
|gothic_horror|Consecrated|Accursed|steampunk|Certified|Faulty|
|cosmic_horror|Warded|Tainted|fairy_tale|Fairy-Blessed|Bewitched|
|space_opera|Commended|Defective|mythic_antiquity|Favored|Doomed|
|cyberpunk|Signed|Corrupted|oneiric|Sweet|Nightmarish|
|post_apocalypse|Clean|Rotten|redacted|Approved|Flagged|
|weird_west|Blessed|Hexed|unwritten|Kept|Struck|
|hardboiled_noir|Lucky|Jinxed|ur_text|Fair Copy|Misprinted|

**Editor and Revisor names.** The five Editors (§8.3) and the six Editor squad templates (`inkling`, `strikethrough`, `marginal_scribe`, `footnote`, `erratum_eater`, `red_pen_lancer`) keep one name in every genre, because a correction is never part of the page it corrects. The Revisor (`enemy.editors.revisor`, doc 13 §10.5) is a field agent dressed for the page it works on, so it takes a genre title, `loc.enemy.editors.revisor.skin.<g>.name`. Doc 13 fixes three; this chapter fixes the rest:

|Genre|Revisor skin|Genre|Revisor skin|
|---|---|---|---|
|mythic_fantasy|Grey Herald|wuxia|Yamen Constable|
|gothic_horror|Inquisitor (doc 13)|steampunk|Patent Examiner|
|cosmic_horror|Asylum Orderly|fairy_tale|Huntsman|
|space_opera|Customs Inspector|mythic_antiquity|Ephor|
|cyberpunk|Liquidator (doc 13)|oneiric|Night Nurse|
|post_apocalypse|Tallyman|redacted|Case Officer|
|weird_west|Regulator|unwritten|Revisor|
|hardboiled_noir|Revenue Agent (doc 13)|ur_text|Corrector|

Every name in this section passes the §7.1 review, and no name repeats a doc 13 template display name of the same genre (doc 13's Redaction sniper is `auditor`, so no Revisor skin is "Auditor").

## 8. Narrative

### 8.1 Lore bible (`nar.lore.*`)

Doc 01 §1.3's premise and four fiction rules are binding. The canon of the setting, one record per entry:

|ID|Entry|
|---|---|
|`nar.lore.palimpsest`|Every story ever told lies in one bottomless manuscript, scraped thin and written over. Nobody has seen its binding.|
|`nar.lore.first_edition`|The Ur-Text is the first draft under every page: all axioms at 5, nothing contradicts. Who wrote it is the one question no text in the game answers.|
|`nar.lore.authors`|Each genre has one Author, its patron (Canon §16.5). Authors wrote their genres and then stopped; they answer prayers but never appear in person.|
|`nar.lore.scraping`|When a story wears thin it is scraped and overwritten. The scraping is weather, not malice.|
|`nar.lore.style`|The Style is the rule that every page agree with itself. It is not a being. The Editors serve it.|
|`nar.lore.errata`|Characters who slipped through the knife when their page was scraped. They remember one genre and can hold any other's tools.|
|`nar.lore.red_ink`|The Editors' correcting fluid. It erases what it touches; drunk, it lets an Erratum correct the world (doc 04 Marginalia powers) at the price of becoming visible to every Editor (Canon §11.6).|
|`nar.lore.margin`|The blank edge of the page, where Errata gather between descents and where no Editor has jurisdiction.|
|`nar.lore.marginalia`|Notes written past the last page. Beyond the Ur-Text there is only margin, and Errata may keep writing there.|
|`nar.lore.colophon`|The sign-off at the end of every page, where the scribe rests. Editors respect a signed page; a Colophon defaced by Errata is treated as forged, and an Editor waits at every later one (Canon §17.1).|
|`nar.lore.redactors`|The Censor keeps an office of junior redactors. When a party leaves its page open (Canon §17.9), another reader can borrow a desk and a red pen for one page.|

Rules for every writer: never name the Ur-Text's author; never show an Author's face; never let a genre know it is fiction (only Errata, Editors, Margin NPCs and the Narrator know); never call an Erratum chosen.

### 8.2 Patron Authors (`nar.author.<genre_suffix>`)

Names, boons and taboos are doc 06 §16.1's, except one display name this chapter changes (below). This lore explains each taboo in the fiction; the answer is the patron's text when a supplication resolves PLEASED (doc 06 §16.3).

|Author|Lore|Why the taboo|Answer when pleased|
|---|---|---|---|
|Ysolde of the Thousand Roads|wrote every quest that begins with leaving home; walks still, unseen, on the longest road|a companion's body left behind is a quest abandoned|"The road remembers you. Walk on."|
|The Vigil-Keeper|wrote the candle in the window and the thing outside it; keeps watch so the dead stay counted|a summoned servitor is a corpse taken off the count|"One more night kept. Sleep, if you can."|
|The Sleeper Under the Index|wrote knowledge nobody was meant to read, then hid beneath the catalogue|destroying a book wakes the Sleeper with a start|"Noted. Filed. Do not look again."|
|The Admiral of the Last Fleet|wrote the oath that crews keep to each other across the dark|shooting your own crew breaks the fleet's first article|"All hands accounted for. Carry on."|
|The Sysop|wrote the network and still holds root on it|an innocent killed is noise in a clean signal|"Access granted. Don't make me regret it."|
|Mother Rust|wrote survival after the end; feeds whoever keeps walking|starving is refusing her bread|"Eat. Then walk. That's the whole sermon."|
|The Rider at the Crossroads|wrote the deal at midnight and the duel at noon|shooting a fleeing foe is not a fair fight|"Fair draw. Keep your hand steady."|
|The Witness|wrote the city and saw everything in it; never testifies|a lifted wallet is a crime in front of a witness|"I saw. I'm not telling. Go."|
|The Sage of the Ninth Peak|wrote the rivers and lakes and the code of the sword|a blade in the back has no honor|"The mountain is patient. Be the mountain."|
|The Grand Engineer|wrote the patent office and every machine filed there|a Device that sputters or surges is shoddy work filed under the Engineer's name|"Splendid tolerances. Mind the boiler."|
|The Godmother of Thorns|wrote every bargain with a price hidden in the third clause|a debt carried out of the story is a promise broken|"Granted. Remember the price."|
|The Laurel-Crowned|wrote heroes and their downfalls in the same hexameter|forty deaths without an offering is hubris|"Sing, and you are remembered."|
|The Dreamer Who Writes Us|dreams every page; the Errata may be its waking thoughts|waking a sleeper with a wound is a nightmare|"Shh. You were never falling."|
|The Undersigned|an Author who went over to the Editors and now redacts its own genre; signs every order and is never named|reading a text is reading what the Undersigned struck|"Approved for release. This once."|
|The Blank Page|not an Author but the absence of one; resets every prayer it hears|—|"…"|
|The First Author|the voice under the Ur-Text; whether it is the Style itself is never answered|—|"In the beginning, you."|

**Display name of `patron.redacted`.** Doc 06 §16.1 first called it "The Censor", and doc 13 §9.2 gives `boss.editor_v2` the display name "The Censor". Two beings cannot share a name, and lore rule "never show an Author's face" forbids the patron being the boss the party fights. This chapter owns `loc.` strings and rules: `loc.boss.editor_v2.name` = "The Censor" (doc 13's reason stands: Canon §17.9 calls every invader a Redactor), and `loc.patron.redacted.name` = "The Undersigned" (triage G-177; doc 06 §16.1 now uses it).

**Answers by outcome** (`loc.nar.author.<g>.answer.<outcome>`, outcome ∈ `pleased`, `helped`, `displeased`, `angry`, doc 06 §16.3). Pleased is the table above.

|Author|Helped (asked again too soon, granted)|Displeased|Angry|
|---|---|---|---|
|Ysolde of the Thousand Roads|"Again so soon? Very well. One more mile."|"You left the road. The road noticed."|"Turn back, traveler. This road is closed to you."|
|The Vigil-Keeper|"The candle is low, but I will spare it."|"Something walked past the window. You let it."|"I will not keep watch for you tonight."|
|The Sleeper Under the Index|"Disturbed again. Filed under exceptions."|"Request misfiled. Consequences forwarded."|"You have woken me. Measure what follows."|
|The Admiral of the Last Fleet|"Off the books, crew member. Once."|"Log this as insubordination."|"You are struck from the muster roll."|
|The Sysop|"Rate limit exceeded. Allowing it anyway."|"Request denied. Logged."|"Session terminated. Don't reconnect."|
|Mother Rust|"Last of the bread. Make it count."|"You wasted what I gave you."|"Starve, then. See if you learn."|
|The Rider at the Crossroads|"Twice in one night. You owe me, partner."|"Dealt you a bad hand on purpose."|"Next time we meet, it's at noon."|
|The Witness|"I didn't see this. Remember that."|"I saw that. Everyone will."|"I'm testifying. Against you."|
|The Sage of the Ninth Peak|"The impatient student is still a student."|"Your stance is careless. Fall, then."|"Leave the mountain. Do not climb back."|
|The Grand Engineer|"Unscheduled maintenance. Billable."|"Shoddy work. Return to sender."|"Patent revoked. Mind the boiler."|
|The Godmother of Thorns|"A second wish costs twice. You will see."|"You forgot the third clause."|"Now you will learn what the price was."|
|The Laurel-Crowned|"Even heroes call twice. The song allows it."|"Hubris has a measure, and you broke it."|"Your name is struck from the poem."|
|The Dreamer Who Writes Us|"Wake a moment. Then sleep again."|"This dream is turning."|"Wake up. Wake up. Wake up."|
|The Undersigned|"Exception granted. Do not cite it."|"Application rejected without review."|"Your file has been closed."|
|The Blank Page|"…"|"…"|"…"|
|The First Author|"Again, from the beginning."|"Not as it was written."|"Unwritten."|

Record: `{ "id": "nar.author.weird_west", "patron": "patron.weird_west", "lore": "loc.nar.author.weird_west.lore", "taboo": "loc.nar.author.weird_west.taboo", "answers": { "pleased": "loc.nar.author.weird_west.answer.pleased", "helped": "loc.nar.author.weird_west.answer.helped", "displeased": "loc.nar.author.weird_west.answer.displeased", "angry": "loc.nar.author.weird_west.answer.angry" } }`. Answers are text only (DECISIONS item 38): they appear as a marginal note in the patron genre's `ink` and heading face, 64 lines.

### 8.3 The Editors (`nar.editor.*`)

Editors are diligent, never cruel (doc 01 §1.3 rule 4). They speak in style-guide rulings and proofreading marks.

|Tier|Members (doc 13)|Fiction|Visual motif|Sample line|
|---|---|---|---|---|
|The Style|none|the law of agreement; never speaks|—|—|
|Editor-in-Chief|The Last Word (`boss.ur_editor`)|issues final rulings; its phases replay every earlier Editor's method|full stop|"Final. No further revisions."|
|Volume Editors|The Proofreader, The Censor, The Continuity Editor, The Abridger (`boss.editor_v1..v4`)|each owns one correction: mark, hide, reconcile, cut|red circle; black bar; timestamp; scissors and ellipsis|"Query: why is this character still here?" / "Withheld." / "That is not what happened." / "Too long. Cut."|
|Revisors|`enemy.editors.revisor` (genre titles in §7.6: Inquisitor, Liquidator, Revenue Agent and 13 more)|field agents sent when a page runs long (Canon §11.6)|genre uniform with Red Ink cuffs|"Revision pass. Hold still."|
|Deputized Redactors|`enemy.editors.invader` (players, Canon §17.9)|other Errata who borrowed a desk in the Censor's office and took the red pen for a page (`nar.lore.redactors`)|the invader kit with a red-pen sigil|none; players speak for themselves|
|Red Ink|inkling, strikethrough, marginal_scribe, footnote, erratum_eater, red_pen_lancer|the marks themselves, animate|living proof marks|non-verbal; scratching and stamping|

Each Editor has one line per verse per attack (doc 13 §9.2 dominant axiom), voiced. The Proofreader's `strikethrough`: ARC "Strike. Unattested usage.", TEC "Strike. Incorrect specification.", MYT "Strike. Unsanctioned.", VRV "Strike. Implausible."

**The Author and Revised Editors (DECISIONS 103).**
In Second Edition (Volumes 6 to 8), the 5 Editors appear in revised forms with heightened visual corruption:
- Revised Proofreader (`boss.editor_v1_rev`), Censor (`boss.editor_v2_rev`), Continuity Editor (`boss.editor_v3_rev`), Abridger (`boss.editor_v4_rev`), Last Word (`boss.ur_editor_rev`) carry dark red ink halos and jagged stroke silhouettes.
- **The Author** (`boss.author`): appears strictly as a giant disembodied writing hand, holding an antique feather quill whose script carves raw glowing wounds into reality (`art.visual.boss.author`). In accordance with §8.1's core rule, the Author never shows a face or humanoid body. Voiced lines are resonant ink scrawls and quill scrapes.

### 8.4 The Margin cast (`nar.margin.<npc>`)

The eight doc 13 §13.6 NPCs are retired Errata who chose neither a story nor erasure. Their dialogue is text in doc 17's Margin dialogue box (`ui.menu.margin_dialogue`, doc 17 §11.1): each has 120 lines of facility talk, available from the start, plus a 20-line voiced greeting set (§6.5) that never carries arc content.

**Arcs** (DECISIONS 78, doc 14 §5.6). Each resident's arc `margin.arc.<resident>` has 6 beats, advanced by optional Folio quests given by the resident (DECISIONS 78, 130). Residents have fixed story arcs with no relationship, friendship, gifting or romance systems (DECISIONS 127). This chapter writes each beat as a text exchange of 2 to 6 lines, each at most 90 characters (§8.8), stored as `loc.margin.arc.<resident>.beat_<n>.<line>` with `n` from 1 to 6 and `line` counted from 0. Beats are never voiced, state no numbers (§8.6 rule 3) and grant nothing. Example, `margin.arc.binder` beat 1 (condition `first_run_end`): Tamsin Stitchwort: "You came back with the spine cracked. Good. Means it was read." / Tamsin Stitchwort: "Bring me what's left of the next one, and I'll sew you a better binding."

|Record|NPC (doc 13 §13.6)|Name, origin|Look and voice|Sample line|
|---|---|---|---|---|
|`nar.margin.archivist`|`npc.margin.archivist`|Ottoline Gloss, Gothic Horror|widow's black, magnifier on a chain; dry, precise|"Another specimen. Do try to bring the whole thing next time."|
|`nar.margin.binder`|`npc.margin.binder`|Tamsin Stitchwort, Fairy Tale|thimbles on every finger, a needle behind one ear; warm, brisk|"Quires in, pages out. Mind the glue."|
|`nar.margin.recollector`|`npc.margin.recollector`|Ascender, Space Opera|a courtesy android with a cracked faceplate; polite, literal|"Memory located. Shall I play it back for you?"|
|`nar.margin.cartographer`|`npc.margin.cartographer`|Ines Verso, Weird West|surveyor's chain and a blank map; laconic|"Map's blank till you walk it. Then it's mine."|
|`nar.margin.inkwell_keeper`|`npc.margin.inkwell_keeper`|Sepia, Cosmic Horror|ink to the elbows, eyes like wet print; soft, unhurried|"The well gives what it gives. Every chance is written on the rim."|
|`nar.margin.codex_scribe`|`npc.margin.codex_scribe`|Brother Rubric, Mythic Fantasy|tonsure, red-and-black ink pots; cheerful pedant|"Another page done. I'll rule the next line for you."|
|`nar.margin.quartermaster`|`npc.margin.quartermaster`|Mags Ferrule, Post-Apocalypse|goggles, a ledger and a cart; blunt and fair|"Price is the price. Same tomorrow."|
|`nar.margin.drillmaster`|`npc.margin.drillmaster`|Master Kerning, Wuxia|old swordsman in plain robes; spare|"Again. Slower. Now the same, but correctly."|

Inkwell Keeper and Quartermaster lines obey doc 15: no urgency, no scarcity claims, no casino lexicon, no reference to money pressure; the Keeper always names the exact odds panel. The **Narrator** (`nar.narrator`) is the Margin's ninth resident and never seen: a former Editor who resigned, now annotating in pencil instead of red. That is why marginal notes are pencil grey `#4D4D4D` and affectionate. Its arc `margin.arc.narrator` has 6 beats, each one pencil note of at most 2 sentences in the Ledger, stored as `loc.margin.arc.narrator.beat_<n>` and never voiced.

### 8.5 Ur-Text endings (`nar.ending.*`)

An ending is text plus a sequence of 3 to 5 illustrated plates in the final genre's style. It is chosen on the run-end screen from `RunSummary` and the run history (doc 14 §4.3, §4.6). Cosmetic only; nothing is granted by it.

```rust
// Ids are id32 constants of the named content IDs. `slots` is indexed by party slot 0..=3; slot 0 is the host
// (doc 14 rule F1). `h` is RunState.history; `RunSummary` and its per-slot `SlotEnd` follow doc 14 §4.6
// and §4.7.
fn select_ending(s: &RunSummary, h: &[FolioRecord], slots: &[Option<SlotEnd>; 4]) -> Id32 {
    let end = s.end;
    let final_is_ur = h.last().map(|f| f.genre) == Some(GENRE_UR_TEXT);
    let present = || slots.iter().flatten();
    if end == RUN_END_VICTORY {
        if s.mode == MODE_SECOND_EDITION { return NAR_ENDING_SECOND_EDITION; }   // second ending layer (DECISIONS 100); first layer: ur_clear_ending
        if !final_is_ur { return NAR_ENDING_SHORT_STORY; }                       // Novella, Daily, Weekly
        if present().any(|x| x.red_ink_doses >= 3) { return NAR_ENDING_RED_LETTER; }
        if slots[0].as_ref().map_or(false, |x| x.went_native.is_some()) { return NAR_ENDING_ADOPTED; }
        if s.party_size_max >= 2
            && present().filter(|x| x.status_at_end != SlotStatus::Departed)
                        .all(|x| x.status_at_end == SlotStatus::Active) { return NAR_ENDING_ANTHOLOGY; }
        return NAR_ENDING_WRITTEN_BACK;
    }
    if end == RUN_END_MARGIN_CLOSED    { return NAR_ENDING_CLOSED_IN_MARGIN; }
    if end == RUN_END_MARGIN_EXHAUSTED { return NAR_ENDING_LAST_MARGIN; }
    if end == RUN_END_ERASED {
        let endless = s.score_at_victory > 0 && s.mode != MODE_SECOND_EDITION;   // a Second Edition wipe forfeits (doc 14 §8.9)
        return if endless { NAR_ENDING_ERASED_IN_MARGIN } else { NAR_ENDING_ERASED };
    }
    NAR_ENDING_SET_ASIDE                       // run.end.abandoned, run.end.abandoned_version, run.end.departed
}
/// First layer of a Second Edition victory: the ending its Ur-Text victory earned (doc 14 §8.9).
fn ur_clear_ending(s: &RunSummary, h: &[FolioRecord], slots: &[Option<SlotEnd>; 4]) -> Id32 {
    let ur = h.iter().position(|f| f.genre == GENRE_UR_TEXT).unwrap();           // the Ur-Text Folio f_ur
    // select_ending and §8.5a read `mode` only to tell Second Edition apart; volumes() is 5 for Standard and Abridged.
    let clear = RunSummary { mode: MODE_STANDARD, end: RUN_END_VICTORY, ..s.clone() };
    select_ending(&clear, &h[..=ur], slots)
}
```

Rows are checked top to bottom, so a victory with 3 Red Ink doses on any slot is `red_letter` even if the host also went native. `erased` keys its plate text by the Volume of the last Folio in `h` (1 to 5, and 6 to 8 in Second Edition). `departed` is a per-slot end (doc 14 §4.3): the departing player sees `set_aside` while the run continues for the others.

Committing a Margins door is not an ending: `nar.ending.into_margins` plays as an interstitial at Folio 26 entry.

**Second Edition** (`mode.second_edition`, doc 14 §8.9; DECISIONS 100, 108). A Second Edition victory shows two layers: first the ending its Ur-Text victory earned, `ur_clear_ending`, with that ending's §8.5a manuscript; then the second layer `second_edition`, whose closing line is its Ironman line (`loc.nar.ending.second_edition.ironman`) when `RunSummary.ironman` is true. Every other Second Edition end forfeits the clear and goes through `select_ending` like any other run, so a wipe in Volume 7 shows only `erased` with Volume 7's plate text and no manuscript (doc 14 §4.7).

|Ending|Closing line (en-US)|
|---|---|
|`written_back`|"You closed the book, and the book, for once, kept you."|
|`anthology`|"Bound together. Every one of you is in the index now."|
|`adopted`|"You chose a story that was never yours. It has decided to keep you."|
|`red_letter`|"You read the first sentence in red. The Style has a new Editor."|
|`short_story`|"Not every tale needs a Volume. This one ended where it meant to."|
|`into_margins`|"No Editor writes here. Mind the edges."|
|`closed_in_margin`|"Notes in the margin, signed and dated. Closed."|
|`last_margin`|"There is no more margin. Even that is a kind of ending."|
|`erased_in_margin`|"Corrected at last, far past the final page."|
|`erased`|"Struck through in Volume {volume}. The page reads cleaner. It reads worse."|
|`set_aside`|"Set aside, spine uncracked. It will keep."|
|`second_edition`|"The Editors revised every page but yours. The second printing kept you." Ironman: "The second printing went to press without a single correction."|

Record: `{ "id": "nar.ending.red_letter", "line": "loc.nar.ending.red_letter.line", "plates": ["art.tex.plate.red_letter_1", "art.tex.plate.red_letter_2", "art.tex.plate.red_letter_3"] }`. After the closing line, one epilogue line per Erratum comes from `nar.grammar.epilogue.<origin_genre_suffix>` (§8.7) with slots `name`, `calling` and `genre`, drawn with `nar_rng(run_seed, 0, "nar.job.ending_epilogue", slot)`.

### 8.5a The closing manuscript (DECISIONS item 46)

The Ur-Text ending is procedural: the run-end screen assembles a closing manuscript from the run's genres, the party's Echoes or Origins, its notable choices and its solved Folio stories (including the fixed story plaque line naming the seat room and Folio Leaf, doc 12 §20.3), and renders it as illustrated pages in the Ur-Text's style. It appears for the endings that follow an Ur-Text victory (`written_back`, `anthology`, `adopted`, `red_letter`, `closed_in_margin`, `last_margin`, `erased_in_margin`); every other ending shows only its closing line and plates. After a Second Edition victory it is the first layer's manuscript, built by `closing_manuscript` over the summary and history that `ur_clear_ending` uses, and the `second_edition` layer follows it with its closing line and plates; the Second Edition Volumes get no sections yet (Canon gaps item 2). It is text and pictures only, and is regenerated from the run summary and history whenever it is opened, so the Codex stores no text.

```rust
// Section s uses nar_rng(run_seed, 0, "nar.job.closing_manuscript", s); every multi-alternative symbol draws once.
fn closing_manuscript(s: &RunSummary, h: &[FolioRecord], slots: &[Option<SlotEnd>; 4]) -> Vec<Section> {
    let party = list_join(present(slots).map(|x| x.display_name));        // Echo name, else the Erratum's name
    let mut out = vec![expand("nar.grammar.closing.title.any", 0, &[("party", &party)])];
    for v in 1..=volumes(s.mode) {                                          // 5 in Standard and Abridged
        let fs: Vec<&FolioRecord> = h.iter().filter(|f| f.volume == v && f.index <= s.deepest_folio).collect();
        let mut sec = expand("nar.grammar.closing.volume.any", v, &[
            ("party", &party),
            ("genres", &list_join(fs.iter().map(|f| genre_display(f.genre)))),   // in Folio order
            ("editor", &editor_display(v))]);                                   // the Volume's Editor, killed on victory
        if let Some(f) = fs.iter().find(|f| f.story_solved.is_some()) {        // lowest Folio index first
            sec.push_str(" ");
            sec.push_str(&retell_deed(f, (2 << 8) | 255, Voice::Praise));       // §8.7 grammar, object index 255
        }
        out.push(sec);
    }
    if s.end != RUN_END_VICTORY { out.push(marginalia_section(h, 8)); }       // Endless: up to 6 latest genres
    out.push(choices_section(s, slots, 6));                                   // at most 4 sentences
    out.push(closing_line(select_ending(s, h, slots)) + &epilogues(slots));   // §8.5, then one line per Erratum
    out
}
```

`choices_section` takes, in this fixed order, the first four kinds that apply and expands `nar.grammar.closing.choice.<kind>` once each from section 6's generator: `going_native` (any slot's `went_native`, slot `name` and `genre`), `red_ink` (any slot with `red_ink_doses ≥ 1`), `colophon_marked` (`RunState.colophon_marked`), `invasion_repelled` (any Folio whose invasion ended with the invader killed), `redacted` (any Folio whose invasion Redacted the party), `patron` (any slot's `top_patron`, the patron with the highest favor above 250 at run end, slot `author`). With none, the section is omitted. `list_join` uses the locale's list pattern (en-US: commas, then "and" before the last item, no serial comma). Every section is at most 400 characters and the whole manuscript at most 2,400 in en-US; a section that would exceed its cap drops its story sentence. The manuscript states no numbers (§8.6 rule 3) and passes the §7.1 tests like every diegetic text.

Plates: `art.tex.plate.title.ur_text` for section 0, `art.tex.plate.volume.<genre_suffix>` (16) for each Volume section, drawn in the genre of that Volume's boss Folio, and the ending's own plates for the last section.

Grammars (en-US):

```json
[
  { "id": "nar.grammar.closing.title.any", "locale": "en-US", "slots": ["party"],
    "rules": { "origin": ["The Account of #party#", "Being the Errata of #party#", "#party#, Bound and Annotated"] } },
  { "id": "nar.grammar.closing.volume.any", "locale": "en-US", "slots": ["party", "genres", "editor"],
    "rules": { "origin": ["#party# crossed #genres#. #editor_line#", "Through #genres# went #party#. #editor_line#"],
               "editor_line": ["#editor# struck at them and missed.", "#editor# ruled against them and was overruled."] } },
  { "id": "nar.grammar.closing.choice.going_native", "locale": "en-US", "slots": ["name", "genre"],
    "rules": { "origin": ["#name# stayed a while in #genre# and came out speaking it.", "#name# took the name #genre# gave and kept it."] } },
  { "id": "nar.grammar.closing.choice.colophon_marked", "locale": "en-US", "slots": [],
    "rules": { "origin": ["Somewhere a Colophon was defaced; the Editors kept count.", "A signature was forged in the margins, and noticed."] } }
]
```

**Worked example.** Run `0x0123456789ABCDEF`, Standard victory, solo `echo.sun_wukong`, Volume 1 genres Wuxia, Hardboiled Noir, Steampunk, Fairy Tale and Gothic Horror with no solved story, Going Native in Steampunk and a desecrated Colophon. Section 0 (seed `0xe3bffbb566a8741e`) draws 1 of 3: "Being the Errata of Sun Wukong". Section 1 (seed `0xf0324ad0cf4755ff`) draws 0 of 2, then 1 of 2: "Sun Wukong crossed Wuxia, Hardboiled Noir, Steampunk, Fairy Tale and Gothic Horror. The Proofreader ruled against them and was overruled." Section 6 (seed `0xee582d0bb6b9f352`) draws 0 of 2 for `going_native`, then 0 of 2 for `colophon_marked`: "Sun Wukong stayed a while in Steampunk and came out speaking it. Somewhere a Colophon was defaced; the Editors kept count." The last section is `written_back`'s closing line and one epilogue line.

### 8.6 Item flavor rules

1. Flavor is at most 140 characters in en-US and at most 2 sentences; names are at most 32 characters. Every other locale's limits are 130% of these (182 and 41 characters).
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

**Expansion.** The generator for lore object `j` showing event `i` is `nar_rng(run_seed, folio, "nar.job.story_retelling", (i << 8) | j)`. Start at `origin_<voice>` (voice from doc 12 §20.3: praise, slander, redact). Expand depth-first, left to right. A symbol with `n ≥ 2` alternatives draws `range(n)` once per occurrence; `n = 1` and slots draw nothing. Modifiers: `.a` (article), `.s` (plural), `.capitalize`. `[[…]]` renders as a redaction bar the player can hover to read. Maximum depth 8 and maximum output 400 characters; the lint enumerates every terminal against §7.1 and the register's "Never" list. An **overwritten** lore object (doc 12 §20.3) swaps the `hero` and `rival` slot values.

**Worked example.** Run `0x0123456789ABCDEF`, Folio 4, the deed event (index 2) on lore object 0: `key = (2 << 8) | 0 = 512`, so the generator is `nar_rng(run, 4, "nar.job.story_retelling", 512)` (seed `0xae27e411c755d822`). With `hero` = Gideon Pennyfeather and voice praise, the draws are `origin_praise` 1 of 2, `wonder` 3 of 4, `span` 1 of 3, `tools` 1 of 3 (4 draws): "They say Gideon Pennyfeather built a clockwork cathedral in nine days, with nothing but borrowed coal." A second lore object (index 1) showing the same deed in voice slander (a hypothetical placement; doc 12 §20.5's example puts event 0 there) uses key 513, and its draws are 0 of 2 and 2 of 4: "Gideon Pennyfeather signed a sky-dock, but every rivet was Captain Gearwright's." Overwritten, it reads "Captain Gearwright signed a sky-dock, but every rivet was Gideon Pennyfeather's."

**Families.** `nar.grammar.story.<template_suffix>.<genre_suffix>` (every doc 12 §20.2 template × 16 genres; `template_suffix` is the last segment of the `gpv.story.*` ID, which is unique, for example `built_wonder`; doc 12 §20.5's `loc.gpv.story.*` names these families, and each template's Codex title is `loc.gpv.story.<template_suffix>.title`), `nar.grammar.folio_intro.<genre_suffix>` (fields `title` and `line`, slots `hero`, `domain1`, `domain2`, `folio`), `nar.grammar.colophon_note.any`, `nar.grammar.bark.<set>.<genre_suffix>`, `nar.grammar.item.<category>`, `nar.grammar.epilogue.<genre_suffix>`, `nar.grammar.codex.<kind>`. Every family ships in every launch locale (§9.11). A fallback grammar emits the same JSON shape as its Scribe job (§9.5), field by field in schema order, from one generator.

### 8.8 Dialogue guidelines

- A subtitle line holds at most 90 characters; a node shows at most 2 lines. Choices are at most 60 characters and begin with a verb. Doc 13 §13.7's checks are deterministic, so a check choice shows its kind, `value` and `dc` instead of a chance, for example "[Persuade 42 / 36] Let us pass."
- Barks (`loc.npc.bark.<set>.<n>`): 8 sets per native faction, one per doc 09 §6.1 state plus repeats: `idle` ×2 (unaware), `curious`, `suspicious`, `searching`, `lost`, `alert` ×2. The same NPC barks at most once per 600 ticks, and the party hears at most one bark per 180 ticks. Barks never leak stealth information: a non-idle bark bubble appears only when the simulation emits a `noise.voice.*` from that NPC (doc 09) and only for slots whose `heard[slot]` is set, with the set chosen from the NPC's current awareness glyph state; an `idle` bubble appears only over an NPC the local player currently sees within 16,000 mm.
- Never concatenate translated fragments; use Fluent selectors for plurals and gender of the listener only.
- Natives never know they are fiction. Editors never threaten pain; they threaten deletion. Margin NPCs never pressure purchases.
- Banned in every line, authored or generated: slurs, sexual content, real persons, real brands, real-world politics and religions by name, self-harm instructions, casino lexicon.

### 8.9 Public-domain Echoes (Canon §16.2, DECISIONS item 28)

The 24 launch Echoes mix original characters with public-domain mythic and literary figures. In the fiction a public-domain Echo is an Erratum like any other: the oldest pages are scraped most often, so their characters fall out most often. Every figure is written and drawn only from its pre-1900 sources, never from a modern adaptation (film, comic, television, game), and never with a modern adaptation's costume, color scheme or catchphrase. The §7.1 brand test screens franchise names of those adaptations, and the asset manifest (§10.3) records the source text for every such Echo.

|Visual record|Echo (doc 14)|Origin genre|Sources|Presentation rules|
|---|---|---|---|---|
|`art.visual.echo.sun_wukong`|`echo.sun_wukong`|wuxia|*Journey to the West* (16th century)|the Monkey King of the novel: golden fillet, staff that changes size, cloud-somersault; speaks in the wuxia register with the novel's irreverence|
|`art.visual.echo.loki`|`echo.loki`|mythic_antiquity|*Poetic Edda*, *Prose Edda* (13th century)|a shape-shifting trickster with the scarred, sewn lips of the *Skáldskaparmál* tale; no horned helmet, which is a modern costume convention|
|`art.visual.echo.lilith`|`echo.lilith`|gothic_horror|medieval Jewish folklore (the *Alphabet of Ben Sira*, about 8th to 10th century) and Victorian poetry about her|a literary night-figure presented in gothic register; lines never quote scripture, never state a religious claim and never name a living religion (§8.8)|
|`art.visual.echo.fenrir`|`echo.fenrir`|mythic_fantasy|*Poetic Edda*, *Prose Edda*|a vast grey wolf with the broken links of Gleipnir at the neck; `body.quadruped`; speaks, as in the Eddas|
|`art.visual.echo.jormungandr`|`echo.jormungandr`|cosmic_horror|*Poetic Edda*, *Prose Edda*|the World Serpent drawn as a sea-dark coil with an eel's eye; `body.serpentine`; speaks seldom and in cosmic-horror register|

Each public-domain Echo has its own `art.visual.echo.<name>` record, `aud.set.voice.<name>` voice set and 40 voiced lines (§6.5), and passes the §4.7 readability limits like any cosmetic.

## 9. The Narrator and the Scribe

### 9.1 Roles

The **Narrator** is the pencil voice in the margins: authored, text-only (DECISIONS item 38: no voiced Narrator), always on. Its notes (`loc.nar.note.<moment>.<n>`) appear on the page turn under the Folio intro, on the Colophon lectern, on firsts (first jam, first Disconnected item, first Erasure stack, first death, first Editor), in tooltip footnotes and on loading pages. `<moment>` is one of `page_turn`, `colophon`, `first_jam`, `first_disconnected`, `first_erasure`, `first_death`, `first_editor`, `tooltip` and `loading`, so doc 17 §16.4 places each note by its key. Notes are manuscript marginalia only: they play no audio, enter no subtitle and carry no caption. The **Scribe** (`crate.scribe`, doc 02) is the optional on-device model that rewrites a fixed set of text jobs in the Narrator's and genres' voices. Every Scribe job has a template grammar producing the same JSON shape, and the template is what the player sees whenever the Scribe is off, late, unsure or unsafe.

### 9.2 Model choice (`nar.scribe.model.*`)

|ID|Model (R4 §8.1, §8.6)|File|Where|Runtime|
|---|---|---|---|---|
|`nar.scribe.model.compact`|Qwen3.5-0.8B, non-thinking mode, Q4_K_M GGUF, Apache-2.0|485 MB (0.8 × 10⁹ × 4.85 bits)|default on every PC and Steam Deck|`llama-cpp-2` 0.1.159; CPU, 2 threads, below-normal priority; context 1,024 tokens|
|`nar.scribe.model.rich`|Qwen3.5-2B, non-thinking, Q4_K_M GGUF, Apache-2.0|1,212 MB|desktop only, with ≥ 16 GiB RAM (it exceeds the Deck's 1,024 MiB Scribe budget)|as above; Vulkan offload when measured free VRAM ≥ 1,536 MiB, else CPU with `min(2, max(1, physical_cores − 4))` threads (R4 §8.6's 2-thread cap)|

Gemma 4 E2B (Apache-2.0) is not chosen: its 4.41 GB GGUF is over 3 times doc 02's 1,024 MiB Deck Scribe budget and the rich model's size. Both models ship as an optional depot downloaded only after opt-in (§9.10); the build fails if the compact file exceeds 512 MiB or the rich file 1,280 MiB. The `model_id` used in cache keys is the model record's `file_stem` (for example `qwen3_5_0_8b_q4km`), then `-`, then the first 16 hex digits of the GGUF file's BLAKE3; the loader recomputes the hash and refuses a file that does not match its record. The rich model is chosen only when the player selects it on a machine that passes its gate; otherwise the compact model runs.

**Device gate.** On enabling, on every game version change and whenever the GPU driver version string changes, a benchmark evaluates a fixed 128-token prompt and then generates 64 tokens under the grammar `root ::= [a-z ]{64}`, timed with the monotonic clock: `prompt_tps = floor_div(128 × 1000, prompt_ms)` and `gen_tps = floor_div(64 × 1000, gen_ms)`. The Scribe runs only if `gen_tps ≥ 12` and `prompt_tps ≥ 100` (`nar.scribe.budget`); otherwise the setting reads "Unavailable on this device" and templates are used. The measured rates feed §9.7's estimates. R4 estimates ≤ 15 tokens/s for the 0.8B model on Deck under load; the gate turns that estimate into a measured rule.

### 9.3 Jobs (`nar.job.*`)

The Scribe runs only in non-combat states. Allowed: the Margin, the lobby, the doc 14 §3.2 phases `InColophon`, `ExitVote`, `VictoryChoice` and `Ended`, and the page turn between an exit commit and the next Folio's first simulated tick. Forbidden: `Exploring`, `ColophonOpen`, `BossAntechamber`, `BossFight` and `Endpaper`; entering a forbidden phase moves every `Queued` and `Running` job to `Fallback`. Jobs run one at a time in the order listed per state. In co-op only the host runs run-content jobs (`colophon_note`, `folio_intro`, `bark_pack`, `ending_epilogue`); every client runs its own hub jobs (`codex_story`, `codex_item`).

|Job|Queued|Deadline (else template)|Input tokens ≤|Output tokens ≤|Output fields|
|---|---|---|---|---|---|
|`nar.job.colophon_note`|entering `InColophon`|first lectern interaction|160|48|`note` ≤ 160 chars|
|`nar.job.folio_intro`|doors light, one per door in door order|the page turn's hold point for the committed door|160|64|`title` ≤ 48, `line` ≤ 200|
|`nar.job.bark_pack`|after the intros, one per door, for the destination genre's primary faction|entry of the next Folio|140|128|8 strings ≤ 90 named `idle_0`, `idle_1`, `curious`, `suspicious`, `searching`, `lost`, `alert_0`, `alert_1` (§8.8)|
|`nar.job.ending_epilogue`|run end, one per present slot in slot order|leaving the run-end screen|200|64|`line` ≤ 200|
|`nar.job.codex_story`|opening a Folio Leaf in the Margin|none|220|160|`title` ≤ 48, `text` ≤ 600|
|`nar.job.codex_item`|opening an item in the Index|none|140|64|`flavor` ≤ 140|

Record: `{ "id": "nar.job.folio_intro", "schema": "nar.job.folio_intro", "template_version": 3, "fallback_family": "folio_intro", "max_in_tokens": 160, "max_out_tokens": 64, "allows_digits": 0 }`. In-run lore objects, item flavor and every voiced line always use authored or template text, so text never changes under the player's eyes mid-Folio. Hub jobs show the template at once and replace it when the Scribe finishes.

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

The other jobs' schemas follow the same pattern (`additionalProperties: false`, every listed field required, the same `pattern`, `minLength` 3 for titles and 10 for prose): `colophon_note` {`note` 10..=160}, `bark_pack` {the 8 named fields, each 3..=90}, `ending_epilogue` {`line` 10..=200}, `codex_story` {`title` 3..=48, `text` 10..=600}, `codex_item` {`flavor` 10..=140}. No output field names game content; where content matters, the field name fixes it (a bark slot), so the model never chooses an ID. Sampling: `temperature_milli` 700, `top_p_milli` 900, `top_k` 40, `repeat_penalty_milli` 1100, seed = the first 4 bytes of the cache key read little-endian.

### 9.6 Caching and co-op

```
key = BLAKE3("PALIMPSEST/SCRIBE/1\n" ‖ u32le(len(model_id)) ‖ model_id ‖ u32le(template_version)
             ‖ u64le(run_seed) ‖ u32le(folio_index) ‖ u32le(fnv1a32(job_id)) ‖ u64le(subject)
             ‖ u32le(len(locale)) ‖ locale)
```

`subject` is the door index for intros and bark packs, the slot for epilogues, the template `id32` for codex items, the Folio index for codex stories, and 0 otherwise. Example: model `qwen3_5_0_8b_q4km`, template 3, run `0x0123456789ABCDEF`, Folio 7, `nar.job.folio_intro` (`fnv1a32` `0x1a087cb5`), subject 2, `en-US` hashes 78 bytes to `f691c20497bea015abc2680f1d7c866041700afb537944d18187e7ccd3a1dc5a`; the sampling seed is `0x04c291f6`. Results that pass §9.8 are stored on disk (LRU, 64 MiB). The example's `model_id` is a test string; shipped IDs append `-` and 16 hex digits (§9.2). llama.cpp output is reproducible only for the same model file, backend and thread count, so the Scribe promises identical text for one key only on one configuration; a party always reads one text because the host's is authoritative.

In co-op the host's Scribe text is authoritative: the host sends each validated result to clients on the reliable channel (Interface assumptions, doc 16). A client shows host text only if its own `flag.scribe.enabled` is true; otherwise it shows its template (§9.10).

### 9.7 Latency budget and job state machine

`t_ms = ceil_div(in_tokens × 1000, prompt_tps) + ceil_div(out_tokens × 1000, gen_tps)`. At the gate floor (100 and 12): `folio_intro` 1,600 + 5,334 = 6,934 ms; `colophon_note` 1,600 + 4,000 = 5,600 ms; `bark_pack` 1,400 + 10,667 = 12,067 ms; `ending_epilogue` 2,000 + 5,334 = 7,334 ms per slot. A party that commits a door in 2 s therefore sees template intros; the Scribe's intros appear when the vote takes longer or the cache already holds them.

|State|Event|Next|
|---|---|---|
|(created)|cache hit for the key|`Done` (from cache)|
|(created)|cache miss, Scribe enabled and allowed in the current phase|`Queued`|
|(created)|cache miss, Scribe disabled, unavailable or forbidden in the current phase|`Fallback`|
|`Queued`|worker free and job at the queue head|`Running`|
|`Queued`|deadline passed, or Scribe disabled|`Fallback`|
|`Running`|output complete|`Validating`|
|`Running`|deadline passed, Scribe disabled, forbidden phase entered, runtime error, or 2 × the estimated `t_ms` elapsed|`Fallback` (generation aborted)|
|`Validating`|§9.8 passes|`Done` (cached)|
|`Validating`|§9.8 fails, no retry used, deadline − now ≥ estimated `t_ms`|`Queued` with sampling seed + 1|
|`Validating`|§9.8 fails otherwise|`Fallback`|

`Done` and `Fallback` are terminal. `Fallback` is never cached; the template is deterministic anyway. Validation is synchronous, so no event can interrupt it. The template text is on screen from the job's creation; `Done` swaps in the Scribe text only where §9.3 allows a swap (hub jobs at once, run-content jobs only before their text is first shown).

### 9.8 Safety filter

```rust
fn accept(job: &Job, out: &str) -> bool {
    let v = match parse_json(out) { Ok(v) => v, Err(_) => return false };
    if !schema(job).validate(&v) { return false; }                 // lengths, patterns, enums, no extra keys
    for s in strings(&v) {
        if banlist_hit(s) { return false; }                        // §7.1 groups, casino lexicon, nar.review.banlist.safety
        if job.register.never.iter().any(|w| contains_word(s, w)) { return false; }
        if s.chars().any(|c| c.is_ascii_digit()) && !job.allows_digits { return false; }
        if proper_nouns(s).any(|n| !job.allowed_names.contains(n)) { return false; }
        if repeated_trigram_ratio_bp(s) > 2000 { return false; }
        if !script_matches(s, job.locale) { return false; }
        if words(s).any(|w| GENDERED.contains(&w.to_lowercase().as_str())) { return false; }
        if sentences(s).any(|x| words(x).count() > 2 * job.register.mean_words) { return false; }
    }
    true
}
```

Definitions:

- `words(s)`: maximal runs of letters, digits, apostrophes and hyphens; `sentences(s)`: `s` split after `.`, `!`, `?` and `…`.
- `GENDERED` = he, she, him, her, his, hers, himself, herself.
- `proper_nouns(s)`: maximal runs of consecutive words that begin with an uppercase letter, excluding a word that opens a sentence or follows an opening quotation mark; "Captain Gearwright" is one run. A run passes if it equals an entry of `allowed_names` or every word of it appears in one entry.
- `repeated_trigram_ratio_bp(s)`: with `n` word trigrams of the lowercased text and `u` distinct ones, `floor_div((n − u) × 10000, n)`, and 0 when `n = 0`.
- `script_matches(s, "en-US")`: every character is U+0020..=U+007E or one of ’ ‘ “ ” … — – é è ê ï ô ç ñ ü. The Scribe runs only for `en-US` (§9.11).

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

Story domains enter as their lexicon nouns (`blood`, `moon`, `crypt`, `mirror`). The title draws first, then the line: 2 draws. Example: run `0x0123456789ABCDEF`, Folio 7, door 1, so `nar_rng(run, 7, "nar.job.folio_intro", 1)` (seed `0x57f506109a835e39`), with `hero` Lucian Ashdown and domains moon and crypt, draws title 0 of 3 and line 1 of 2: `{"title": "The House of Moon", "line": "Every portrait in this house remembers Lucian Ashdown, and none of them fondly."}`.

### 9.10 Opt-out, disclosure and the cosmetic guarantee

- `flag.scribe.enabled` defaults to false (doc 01 Q-07, doc 02). Enabling it shows a disclosure page, downloads the model depot and runs the device gate. Disabling it stops jobs at once and switches every displayed Scribe text back to its template; "Delete model and cache" removes both.
- The store page declares that the game generates text during play when the option is enabled, and that pre-generated content used authoring-time AI assistance where §10.3 records it (R4 §8.5).
- The Scribe never writes simulation state, saves' simulation sections, replays, packets other than the presentation text message, or anything doc 14's `RunSummary` holds. The Codex stores story events as IDs (doc 12 §20.4) and regenerates text from the cache or template.

### 9.11 Localization

- Launch locales (15, DECISIONS item 40): `en-US` (source), `fr-FR`, `de-DE`, `es-ES`, `es-419`, `pt-BR`, `it-IT`, `pl-PL`, `ru-RU`, `uk-UA`, `tr-TR`, `ja-JP`, `ko-KR`, `zh-Hans`, `zh-Hant`. No right-to-left script ships at launch.
- Every `loc.` key and every `nar.grammar.*` family ships in all 15; CI fails on a missing key, and on the pseudo-locale `en-XA` overflowing any UI box (doc 02 §10).
- The Scribe runs only for `en-US` at launch; in other locales the option is hidden and templates are used.
- Name grammars (§7.4) are not translated; titles inside them are.

## 10. Asset pipeline and budgets

### 10.1 Pipeline

|Stage|Input|Output|Gate (`pal` tool, CI)|
|---|---|---|---|
|Author|Blender `.blend`, MagicaVoxel `.vox`, Krita or Aseprite layers, DAW sessions|source in `art_src/` (LFS)|manifest entry exists (§10.3)|
|Export|glTF 2.0 binary; PNG; WAV 48 kHz 24-bit|`assets/art/mesh/…`, staging textures and audio|names follow doc 02 §11.4 path rule|
|Bake|`.glb` clips|hitbox pose tables (`pal bake-poses`, doc 03 §5.16)|clip name equals its `art.anim.*` ID|
|Compress|textures; audio|KTX2 BC7 with zstd; Ogg Opus|sizes within §10.2|
|Lint|every asset and `loc.` file|report|§3.6 silhouettes, §4.7 cosmetic limits, §7.1 names, §8.7 grammar lint, palette and LUT checks, license manifest|
|Hash|all content|`content_hash` over simulation records; `presentation_hash` over every `art.`, `aud.`, `nar.` and `vfx.` record and every `loc.` table (Canon §2.4, doc 02 §11.2)|committed schemas match|

**Production order** (DECISIONS 114, which supersedes item 41). Early Access ships all 12 core genres, so the 25-Folio Standard run ships there, and the 4 secret genres follow the canon's unlock rules (Canon §17.1: reached only through secret exits). Every per-genre asset family (block array, bible, palette, light, post, LUT, sprites, fonts, music track, ambience, SFX sets, grammars, damage and glyph skins) is therefore produced for the 12 core genres, the Ur-Text (every full run's final Folio) and the Unwritten (whose records the Margin reuses, §3.9) before Early Access, and for Oneiric and the Redaction before the first build whose content set holds them (doc 21 §2.2). Global assets (Errata, Echoes, Editors, Red Ink, the Margin, UI sounds) ship complete at Early Access. Each build's lint runs every per-genre check on the genres its content set holds and refuses content that references an asset of a genre outside that set.

**Folio Expansion genres** (DECISIONS 53, doc 12 §25). The first two planned are Age of Sail and Pirates, then Kaiju and Mecha. An expansion genre has no ID until the canon adds its §5.2 row, so this chapter reserves none; its records use that row's suffix, written `<exp_g>` here. Before its pack enters any content set, it ships every per-genre record of this chapter: `art.palette.<exp_g>`, `art.light.<exp_g>`, `art.post.<exp_g>`, `art.bible.<exp_g>`, `art.lut.<exp_g>`, `art.blockarray.<exp_g>`, `art.tex.paper.<exp_g>`, `art.icon.aether.<exp_g>`, `vfx.sprites.<exp_g>`, `aud.music.<exp_g>`, `nar.register.<exp_g>`, `nar.names.<exp_g>`, `nar.author.<exp_g>`, the §8.7 grammar families, its 15 damage, 13 glyph-Form, 7 appearance-form, 2 curse-state and 1 Revisor skin names, and the three Ur-Text quote stems `pulse`, `tension` and `combat` of §6.1, plus a §2.2 tradition, a §3.4 bible row and a §3.4 material set. Every rule and lint here applies to it: palette swatches 0 to 2 equal its Canon §5.9 row, the §3.1 contrast rules, the §3.5 font licenses, the §6.1 bar lint and the §7.1 review of every name. Traditions: Age of Sail and Pirates is copperplate marine engraving with sea-chart vignettes; Kaiju and Mecha is mid-century monster-film poster lithography over engineering cutaway plates. Ships, airships and mecha are architecture, enemies and bosses, never a ridden or piloted viewmodel (Canon §17.7, doc 12 §25 rule 6). The counts in this chapter (16 genres, 240 damage names, 36 quote stems) are the base game's, and rules that count the 12 base core genres keep counting them (doc 12 §25 rule 5), so §7.4's Ur-Text name draw stays `range(12)`.

### 10.2 Budgets

|Class|Tris LOD0 / LOD1 / LOD2|LOD distances|Bones|Textures|
|---|---|---|---|---|
|Errata and Echoes|6,000 / 2,400 / 800|0 / 16 / 40 m|≤ 64|2 × 512²|
|Standard enemy, native NPC|3,000 / 1,200 / 400|0 / 16 / 40 m|≤ 48 (insectoid 56)|2 × 256²|
|Mini-boss, Editor|15,000 / 6,000 / 2,000|0 / 24 / 60 m|≤ 96|2 × 512²|
|Block prop, placeable|≤ 1,500|single LOD|0 or 4|2 × 128²|
|Viewmodel weapon plus arms|4,000 + 3,000|single|≤ 40|2 × 512²|
|Micro-voxel debris cube|12|single|0|the block layer|

BC7 sizes with full mip chains to 1 × 1 (each level below 4 × 4 still takes one 16-byte block, doc 02 §8.3): 128² 21,872 B; 256² 87,408 B; 512² 349,552 B; a 64² block layer 5,488 B. Sixteen block arrays of 256 layers take `16 × 256 × 5488 = 22,478,848` B (21.44 MiB), inside doc 02 §15's 32 MiB VRAM line for block arrays; only the Folio genre and at most 4 bleed genres are resident, 5 × 256 × 5488 = 7,024,640 B. Audio totals 417.6 MB on disk: music 189 MB (16 genre tracks of 5 stems over 2,186.7 s of loops, the Margin's single 96 s bed, 36 Ur-Text quote stems of 21.3 s, 15 one-second Editor hits and 6 four-second stingers, all at 128 kbit/s, which is 16,000 B/s), VO 87.6 MB (§6.5), genre SFX 63 MB, core SFX 17 MB, ambience 61 MB. Base install ≤ 6 GiB; the Scribe depot adds 485 MB (compact) or 1,212 MB (rich). Runtime memory stays inside doc 02 §15: entity textures 896 MiB VRAM, CPU-side assets 1,408 MiB, audio 256 MiB (§6.6), Scribe 1,024 MiB.

### 10.3 AI-assisted production, curation and licensing

Every shipped asset has a manifest record:

```json
{ "id": "art.mesh.enemy.gothic_horror.ghoul", "owner": "a.artist", "reviewer": "lead.art",
  "license": "original", "source_files": ["art_src/gh/ghoul.blend"], "ai_assist": "concept",
  "ai_tools": ["concept.sketchbook"], "legal_name_review": 1, "revision": 4 }
```

|`ai_assist` value|Allowed when|
|---|---|
|`none`|always|
|`concept`|AI images used only as mood reference; the shipped asset was made by hand by `owner`|
|`draft_edited`|an AI draft of text, a texture or a prop mesh, with every shipped pixel, vertex or word reviewed and edited by `owner`; the tool's license permits commercial use; the record lists the tool|
|`generated_text`|authoring-time text drafts (R4 §7.5) that passed the §8.7 and §7.1 lints and a human edit; disclosed on the store page|
|`generated_voice`, `generated_music`|never shipped; scratch use in internal builds only|

- Prompts never name a living artist, a studio or a franchise. No asset imitates a real person's likeness or voice.
- Voice actor contracts forbid training or cloning from their recordings; music is commissioned with a buyout and stems delivered.
- Third-party assets are allowed only under CC0, CC-BY 4.0 (credited in `CREDITS.md`), SIL OFL 1.1 or Apache 2.0. CC-NC, CC-ND, GPL and "editorial use" licenses fail the lint.
- `owner` and `reviewer` are named humans and differ; the lint rejects an empty field.

## Exports

All IDs below are in this chapter's namespaces (`art.`, `aud.`, `nar.`, `vfx.`) or are `loc.` text keys (Canon §19).

- **`art.`**: `art.palette.<g>`, `art.light.<g>`, `art.post.<g>`, `art.bible.<g>`, `art.lut.<g>`, `art.blockarray.<g>`, `art.tex.paper.<g>` for the 16 genre suffixes (the Margin reuses `art.palette.unwritten`, `art.light.unwritten`, `art.lut.unwritten` and `art.post.unwritten`, §3.9); `art.tex.decal.*`; `art.tex.plate.*` (ending plates), `art.tex.plate.title.ur_text`, `art.tex.plate.volume.<g>` (16); `art.tex.sigil_atlas` (doc 10's 28 sigils); `art.idc.<class>.<name>` (doc 10's 176 descriptors); icons `art.icon.dmg_<type_suffix>` (15), `art.icon.<status_suffix>` (one per Canon §11.1 status, for example `art.icon.bleeding`), `art.icon.aether.<g>` (16); `art.font.<family>` for the §3.5 families plus `art.font.noto_sans_cjk`, `art.font.noto_serif_cjk`, `art.font.atkinson_hyperlegible`; `art.silhouette.<role_suffix>` (16, one per doc 13 role); `art.readability.cosmetic`; `art.bible.margin` (§3.9); `art.vm.<archetype>` (the archetype ID without `arch.`) including `art.vm.ranged.pistol`; `art.native.<g>.<kin_suffix>` (96); `art.preset.<kin_suffix>.<nn>` (72), `art.preset.<kin_suffix>.colors` (6) and `art.tex.marking.<kin_suffix>.<nn>` (72) (§5.6); rigs `art.rig.humanoid`, `art.rig.quadruped`, `art.rig.avian`, `art.rig.serpentine`, `art.rig.insectoid`, `art.rig.amorphous`, `art.rig.construct`; families `art.mesh.*`, `art.anim.*`, `art.block.*` (including `art.block.hull_panel.space_opera`), `art.visual.*` (including `art.visual.echo.<name>`, `art.visual.boss.author`).
- **`vfx.`**: `vfx.dmg.slash`, `vfx.dmg.pierce`, `vfx.dmg.blunt`, `vfx.dmg.fire`, `vfx.dmg.frost`, `vfx.dmg.shock`, `vfx.dmg.acid`, `vfx.dmg.radiation`, `vfx.dmg.sonic`, `vfx.dmg.toxin`, `vfx.dmg.radiant`, `vfx.dmg.umbral`, `vfx.dmg.psychic`, `vfx.dmg.arcane`, `vfx.dmg.true`; `vfx.status.<status_suffix>`; `vfx.aware.curious`, `vfx.aware.suspicious`, `vfx.aware.searching`, `vfx.aware.lost`, `vfx.aware.alert`; `vfx.contra.strained`, `vfx.contra.contradicted`, `vfx.contra.disconnected`, `vfx.contra.failure`; `vfx.ward.break`, `vfx.ward.ripple`; `vfx.debris.microvoxel`; `vfx.sprites.<g>` (16); `vfx.trans.page_turn`, `vfx.trans.ink_bleed`, `vfx.trans.glitch`, `vfx.trans.iris`, `vfx.trans.film_burn`; `vfx.telegraph.<shape>`; `vfx.ragdoll`.
- **`aud.`**: `aud.music.<g>` (16) and `aud.music.margin`; stems `aud.music.<track>.<stem>`; `aud.music.ur_text.quote.<core_g>.<stem>` (36); `aud.music.editor.<boss_suffix>` (5) with hits `aud.music.editor.<boss_suffix>.<hit>` (15); `aud.music.sting.folio_enter`, `aud.music.sting.colophon_open`, `aud.music.sting.editor_phase`, `aud.music.sting.player_downed`, `aud.music.sting.run_victory`, `aud.music.sting.run_erased`; `aud.sfx.<family>.*` for the §6.3 families (`wpn`, `imp`, `foot`, `hit`, `body`, `mag`, `env`, `status`, `ui`), including `aud.sfx.foot.<gait>.<class>` and `aud.sfx.imp.<type_suffix>.<class>` over doc 09's 17 acoustic classes; `aud.set.<archetype_suffix>.<g>`; `aud.set.voice.<name>`; `aud.amb.<tileset>`; `aud.vo.<speaker>.<line>`; `aud.bus.voice`, `aud.bus.sfx`, `aud.bus.ambience`, `aud.bus.music`, `aud.bus.ui`; `aud.mix.default`.
- **`nar.`**: `nar.lore.palimpsest`, `nar.lore.first_edition`, `nar.lore.authors`, `nar.lore.scraping`, `nar.lore.style`, `nar.lore.errata`, `nar.lore.red_ink`, `nar.lore.margin`, `nar.lore.marginalia`, `nar.lore.colophon`, `nar.lore.redactors` (11); `nar.author.<g>` (16); `nar.editor.proofreader`, `nar.editor.censor`, `nar.editor.continuity_editor`, `nar.editor.abridger`, `nar.editor.last_word`; `nar.margin.<npc_suffix>` (8); `nar.narrator`; `nar.ending.written_back`, `nar.ending.anthology`, `nar.ending.adopted`, `nar.ending.red_letter`, `nar.ending.short_story`, `nar.ending.closed_in_margin`, `nar.ending.last_margin`, `nar.ending.erased_in_margin`, `nar.ending.erased`, `nar.ending.set_aside`, `nar.ending.second_edition` (§8.5), `nar.ending.into_margins` (interstitial); `nar.grammar.*` (§8.7 families, including `nar.grammar.colophon_note.any`); `nar.names.<g>` (15; the Ur-Text reuses core tables); `nar.register.<g>` (16); `nar.job.colophon_note`, `nar.job.folio_intro`, `nar.job.bark_pack`, `nar.job.ending_epilogue`, `nar.job.codex_story`, `nar.job.codex_item`, `nar.job.story_retelling` and `nar.job.closing_manuscript` (template-only); `nar.grammar.closing.title.any`, `nar.grammar.closing.volume.any`, `nar.grammar.closing.choice.<kind>` (6 kinds); `nar.scribe.model.compact`, `nar.scribe.model.rich`, `nar.scribe.budget`; `nar.review.banlist`, `nar.review.banlist.safety`.
- **`loc.` text keys**: `loc.dmg.<type>.skin.<g>.name` (240); `loc.glyph.form.<form>.skin.<g>.name` (130 here); `loc.idc.<class>.form.<g>` for the 4 secret genres (28); `loc.buc.blessed.skin.<g>.name` and `loc.buc.cursed.skin.<g>.name` (32); `loc.enemy.editors.revisor.skin.<g>.name` (16); `loc.nar.author.<g>.answer.<outcome>` (64); `loc.boss.editor_v2.name` ("The Censor") and `loc.patron.redacted.name` ("The Undersigned"); `loc.<hazard id>.skin.<g>.name`; `loc.<event id>.skin.<g>.name`; `loc.nar.note.<moment>.<n>`; `loc.npc.bark.<set>.<n>`; `loc.npc.dlg.<name>.<node>`; `loc.margin.arc.<resident>.beat_<n>.<line>` for the 8 residents and `loc.margin.arc.narrator.beat_<n>` (54 beats, §8.4); `loc.<mfr id>.slogan`; `loc.gpv.story.*`; `loc.aud.caption.*`; `loc.idc.*` strings; every `.name`, `.desc` and `.flavor` string.
- **Functions**: `nar_rng`, `pres_hash` (§1.2), `select_ending` and `ur_clear_ending` (§8.5), `allocate` (§6.6), `accept` (§9.8).

## Interface assumptions

Only open requests are listed; what this chapter reads from chapters that already provide it is in Imports.

|Doc|Assumed to provide|
|---|---|
|02|§9's summary of `aud.mix.default` as §6.6 defines it: each pool owns its voices and never borrows; a full priority-7 pool stops its oldest voice; any other full pool replaces its quietest voice only for a louder request (doc 02 §9 still says a full mix steals the quietest voice of a lower-priority pool)|
|04|the Cosmetics step (§2.1) records the chosen `Appearance` (§5.6) with the Erratum's creation choices, and Going Native (§2.8) keeps it|
|07|block records whose `skins` (doc 03 §4.5) name the `art.block.*` records of §3.8|
|13|a `visual: art.visual.*` field on every template; every template's `body_state.stand_mm` within 150 mm of its mesh, with humanoid brutes at `stand_mm ≥ 2200` (§3.6); telegraph shape IDs; bark sets per native faction; Editor attack IDs for line timing; the second mini-boss of each genre (DECISIONS 69, doc 13 Canon gaps item 1), which §6.5 already budgets|
|16|a reliable host-to-client message carrying `ScribeText { key: [u8; 32], job: Id32, subject: u64, json: ≤ 4,096 bytes }`; each slot's `Appearance` (§5.6) in the data a joining peer receives, so every peer draws the same look|

## Canon gaps

Every gap this chapter raised against canon-1.7 is closed by canon-1.11 or by the triage (`audit/canon_gaps_triage.md`): the Unwritten's identity skin is Canon §5.2's own-currency exemption (G-053); "Crowns" stays by product-owner exemption (DECISIONS 90, which overrules G-052); `nar_rng` and `pres_hash` are Canon §3.8 (G-032); the 7-swatch palette is this chapter's (G-054); `presentation_hash` is Canon §2.4 (G-017, DECISIONS 87); spoken barks are rewritable text bubbles and voiced lines never are (G-202, §6.5); the Margin plot is doc 14's trophy alcove (G-210); the boss is the Censor and the patron the Undersigned (G-177, C-16); texture layers are 64 × 64 per Canon §4.4 (G-217, C-12); and the Erasure hunter carries `rank.hunter` with `rank.elite` multipliers (G-112, DECISIONS 91). Open items:

1. **Name-seed generator.** Canon §3.8 names `nar_rng` and `pres_hash` as the only presentation generators, while §7.4 expands doc 12 §20.1's recorded `hero_name` and `rival_name` seeds with `Rng::from_seed64(name_seed)`, which advances no stream and keeps the §7.4 and §8.7 test vectors. Proposal: Canon §3.8 adds "and `Rng::from_seed64(seed)` over a name seed the simulation already recorded (doc 12 §20.1, doc 18 §7.4)".
2. **Second Edition and presentation records (DECISIONS 99, 100, 103).** Docs 12 and 13 now define the climate tables (doc 12 §3.7), mini-boss dressings (§21.2) and the revised Editors with the Author (doc 13 §9.4). The Author is represented exclusively by a giant writing hand, quill, and handwriting (§8.3). Per-Folio weather states feed light and fog variants in §3.2 and sound in §6.3. The second ending layer is defined in §8.5.

## Acceptance tests

1. For each of the 16 genres, `art.palette.<g>.swatches[0..3]` equals Canon §5.9's three colors as decimals (gothic_horror: 3809067, 7998996, 13157300).
2. For each genre, `max(cr(danger, ink), cr(danger, paper)) ≥ 4.0`; ur_text gives 4.80 with `#E04A2A`, wuxia 4.07, cyberpunk 6.52.
3. Every `art.post.<g>.anim_step` ∈ {1, 2, 3}; hardboiled_noir has `keep_hue_bam` 0 and `keep_hue_width_bam` 2731; redacted has `outline_px` 0 and `dither` 1.
4. `fog_opacity_milli(48000)` with fog 1 is 400; with fog 0 it is 0; mythic_fantasy `haze_milli(96000)` is 150 and `haze_milli(64000)` is 0; fog 1 at 120,000 mm is 1000.
5. Outline width: `outline_px` 2 gives 4 px at 1440 lines and 2 px at 800.
6. Paper factor with `paper_milli` 350 and sample 0.8 is 0.93.
7. Viewmodel reload: clip 120 ticks, SLT 60 gives `action_ticks` 110 and playback rate 120/110.
8. `anim_step` quantization: tick 37 → 36 at steps 2 and 3; tick 38 → 38 at step 2, 36 at step 3.
9. Ragdoll impulse: previous `hp_bp` 3000 gives 3800 mm/s; 9000 gives 7400; 10000 gives 8000; an explosion at `d = 1000`, `r = 4000` adds 4500; the total never exceeds 14,000.
10. Music: every track has `beats × 3600 mod bpm = 0`; gothic_horror bar is 150 ticks and 120,000 samples; cosmic_horror loops 32 bars (300-tick bars); weird_west's 6/8 has `beats_per_bar` 2 and 100-tick bars.
11. Music level rise computed on tick 1000 in a 150-tick-bar track started at tick 0 applies at tick 1050; a fall applies only after 480 ticks at the lower level, at the next bar line.
12. Crossfade at `x = 0.5`: both gains 0.7071 ± 0.0001.
13. Every `dmg.` type has exactly one `vfx.dmg.<type_suffix>` record whose color equals Canon §10.1's default.
14. All 240 `loc.dmg.<type>.skin.<g>.name` keys exist in `en-US`; `loc.dmg.shock.skin.steampunk.name` = "Galvanic"; within each genre the 15 names are distinct.
15. All 130 glyph-form keys of §7.5 exist; no `loc.glyph.form.*.skin.unwritten.name` key exists.
16. Name review: "Crowns" as the Mythic Fantasy Lucre skin passes by the DECISIONS 90 exemption, and "Crowns" as any other Lucre or Aether skin fails test 4; "V-Bucks" fails test 3; "Ink" fails test 1; "Galvanic" and "Credits" pass; Unwritten "Lucre" passes by the Canon §5.2 exemption; "Lucky Streak Charm" fails test 5.
17. Names: steampunk seed `0x9c1f00d2a4be7731` → "Gideon Pennyfeather", draws (0, 4, 1, 2); seed `0x13a4c0ffee0f1e2d` → "Captain Gearwright", draws (1, 4, 0, 1); each name consumes exactly 4 draws.
18. `nar_rng(0x0123456789ABCDEF, 4, "nar.job.story_retelling", 512)` is seeded with `0xae27e411c755d822`; expanding §8.7's grammar with voice praise yields "They say Gideon Pennyfeather built a clockwork cathedral in nine days, with nothing but borrowed coal." after exactly 4 draws.
19. Key 513, voice slander yields "Gideon Pennyfeather signed a sky-dock, but every rivet was Captain Gearwright's." after 2 draws; the overwritten variant swaps the two names.
20. Scribe cache key of §9.6's example equals `f691c20497bea015abc2680f1d7c866041700afb537944d18187e7ccd3a1dc5a` over 78 bytes; sampling seed `0x04c291f6`.
21. Latency formula at rates 100 and 12: `folio_intro` 6,934 ms, `colophon_note` 5,600 ms, `bark_pack` 12,067 ms, `ending_epilogue` 7,334 ms.
22. Safety: an output containing "jackpot", a digit, the name "Gandalf" absent from `allowed_names`, a 49-character title, or an extra key is rejected and the template is shown.
23. Opt-out: a client with `flag.scribe.enabled = false` that receives host `ScribeText` displays its template; toggling the flag off replaces displayed Scribe text with templates within one frame.
24. No Scribe job runs while the Folio phase is `Exploring`, `ColophonOpen`, `BossAntechamber`, `BossFight` or `Endpaper`; a `folio_intro` still `Running` at the page-turn hold is aborted and its template shown; entering `ColophonOpen` moves every `Queued` job to `Fallback`.
25. Isolation: `cargo tree -p palimpsest_sim` contains neither `palimpsest_scribe`, `palimpsest_render` nor `palimpsest_audio`, and the sim never opens a file under `content/18/`; a full run's `state_hash` sequence is identical with the Scribe on and off.
26. Endings: Ur-Text victory with one slot at 3 red-ink doses → `red_letter`; with the host gone native and no doses → `adopted`; a full 3-player party alive → `anthology`; Novella victory → `short_story`; erased after victory in Endless → `erased_in_margin`.
27. Cosmetic lint rejects a mesh vertex 160 mm outside the inflated capsule, a trail of 25 particles per second, an albedo with 30% of its area within CIEDE2000 ΔE 10 of `#D7262E`, and a cosmetic whose mean albedo luminance is 120 milli over a default skin at 300 milli; it accepts 200 milli over the same skin.
28. Audio pools: the Deck pools sum to 4 + 16 + 20 + 6 + 14 + 4 = 64 and the desktop pools to 6 + 32 + 52 + 10 + 24 + 4 = 128; with the `combat` pool full of 20 voices at higher listener gain than a new combat request, the request is dropped and no `stealth` voice changes; with the `stealth` pool full, a new hostile footstep stops that pool's oldest voice and plays.
29. An `ev.noise` with `heard[slot] = None` plays no sound for that slot; the doc 02 oak-door example plays at 95 dB with an 8,434 Hz cutoff.
30. Asset manifest lint rejects `ai_assist = "generated_voice"`, an empty `reviewer`, `owner == reviewer`, and a CC-BY-NC license.
31. Every `nar.grammar.*` family exists in all 15 launch locales, and no expansion of any grammar exceeds 400 characters (exhaustive enumeration for grammars with ≤ 100,000 expansions, 10,000 sampled expansions otherwise).
32. No `loc.dmg.*.skin.<g>.name` equals genre `g`'s Lucre or Aether skin with a trailing "s" ignored; weird_west umbral is "Curse", hardboiled_noir psychic "Jitters", oneiric arcane "Surreal", ur_text arcane "The Spell".
33. Folio-intro fallback: `nar_rng(0x0123456789ABCDEF, 7, "nar.job.folio_intro", 1)` is seeded with `0x57f506109a835e39` and §9.9's grammar yields title "The House of Moon" and the line "Every portrait in this house remembers Lucian Ashdown, and none of them fondly." after exactly 2 draws.
34. Every enemy template's `visual` resolves to an `art.visual.*` record; every `anims` entry of that record exists both as a doc 03 pose table and as a clip of that name in its `.glb`; every `art.block.*` layer index is below its array's layer count (≤ 256).
35. Silhouette lint: `orc_raider` (swarmer, humanoid, `stand_mm` 1650, head 200 mm ahead of the pelvis) passes; a humanoid brute at `stand_mm` 2000 fails; any template whose idle bounding-box height differs from `stand_mm` by 200 mm fails; `sky_aeronaut` with a 2,000 mm wing span fails and with 2,200 mm passes.
36. Awareness glyphs: `StealthHud.marks` states 1 to 5 select `vfx.aware.curious`, `suspicious`, `searching`, `lost` and `alert`; state 0 draws nothing; an observer behind a wall draws nothing unless it bears `st.marked`.
37. Dominant VFX: a hit of 6,000 mHP fire and 4,000 mHP slash routes `vfx.dmg.fire` with 14 particles and no slash emitter; a hit fully absorbed by ward plays `vfx.ward.ripple`; `ev.ward_broken` plays `vfx.ward.break`.
38. Ur-Text counterpoint: with history (wuxia, hardboiled_noir, wuxia), phrases 0 to 4 play the quotes of wuxia, hardboiled_noir, wuxia, wuxia, hardboiled_noir; phrase length is 1,280 ticks.
39. Silence gate: at music level 0 in the Unwritten, phrase `p` mutes the bed iff `pres_hash("aud.silence", folio_index, p) % 10000 < 9000`; at level 1 the bed always plays.
40. Name review, further cases: "Tesla Relay" passes by the eponym allowance; "Slot Machine" fails test 5 while the UI string "Ring slot" passes; "Bottle Caps" fails test 3; "loot box" in an item flavor fails test 6 and passes in a menu string.
41. Reading contrast: `cr(ink, paper) ≥ 7.0` for all 16 genres; the Unwritten pair `#595959` on `#FFFFFF` gives 7.00.
42. Endings, further cases: `run.end.margin_closed` → `closed_in_margin`; `run.end.departed` for one slot → `set_aside` for that player while the run continues; `run.end.abandoned_version` → `set_aside`; erased before any victory with the last Folio in Volume 2 → `erased` with plate text for Volume 2.
43. Scribe states: a job whose key is in the cache goes from creation to `Done` without running; a runtime error in `Running` goes to `Fallback`; a validation failure with enough time left re-queues once with sampling seed + 1 and then falls back.
44. Safety filter, further cases: "He walked into the rain." is rejected (gendered pronoun at sentence start); "the page the page the page" has 4 word trigrams, 2 distinct, ratio 5000 Bp, and is rejected; a cyberpunk sentence of 15 words is rejected (ceiling 14).
45. Patron text: all 64 `loc.nar.author.<g>.answer.<outcome>` keys exist; `loc.patron.redacted.name` is "The Undersigned" and `loc.boss.editor_v2.name` is "The Censor"; no answer line is voiced.
46. Skin families: 16 Revisor skin keys exist and gothic_horror, cyberpunk and hardboiled_noir equal doc 13's Inquisitor, Liquidator and Revenue Agent; 32 curse-state keys exist and the Blessed and Cursed names differ in every genre; 28 secret-genre appearance form keys exist; every one passes §7.1.
47. Invader rendering: an Erratum with INVISIBLE renders at shimmer 2500 Bp on an invader's client whose `access.shimmer_bp` is 6000; a ghost is never drawn on an invader's client.
48. Combat exemption: a clip tagged `combat` at `anim_tick` 37 on a step-3 genre shows pose 37; an idle clip at the same tick shows pose 36.
49. Micro-voxels: removing one micro-voxel from a full destructible cell (a cracked wall tagged `block.tag.destructible`) shows the previous genre's texture on the newly exposed faces for 600 ticks, draws the 1 px rim from `fract(world_pos × 4)` on that cell, and spawns one debris cube while fewer than 256 cosmetic rigid bodies exist; a fixed-terrain cell never loses a micro-voxel, so it never shows the underlayer.
50. Audio coverage: every `noise.*` ID in doc 09 maps to at least one audio record; every (gait, acoustic class) and (damage type, acoustic class) set exists; every record with a `caption` field names an existing `loc.aud.caption.*` key.
51. Weather front: in Cyberpunk (fog 0) during `gpv.evt.weather_front`, `fog_pts` is 1, opacity at 48,000 mm is 400 and haze is off.
52. Budgets: `16 × 256 × 5488 = 22,478,848` B of block arrays; VO 4,380 lines at 87,600,000 B, 840 of them for 28 genre mini-bosses; music 189 MB; audio 417.6 MB; the compact model file ≤ 512 MiB.
53. Content-set lint: a content set refuses any record that references an asset of a genre outside it; the Early Access set holds the 12 core genres, the Ur-Text and the Unwritten (DECISIONS 114), and every per-genre check runs on each of them.
54. Non-humanoid Echoes: `echo.fenrir`'s viewmodel uses `art.rig.quadruped` and has no arm clips; `echo.jormungandr`'s uses `art.rig.serpentine`; Fenrir's held weapon renders at the `jaw` socket, and Jörmungandr's viewmodel shows `grip_1` only while `slot.hand_extra_1` holds an item; each of the 5 public-domain Echoes has a manifest record naming its source text, and a line containing a modern adaptation's franchise name fails test 2.
55. VO plan: no `aud.vo.narrator.*` and no `aud.vo.patron.*` record exists (DECISIONS item 38); each Margin resident has exactly 20 voiced greeting records, and no audio record references a `loc.margin.arc.*` key (DECISIONS item 78).
56. Narrator pencil: `cr(#4D4D4D, paper) ≥ 4.5` for all 16 genres, minimum 5.17 (post_apocalypse `#D9C9A3`).
57. Phrase clock: in an Ur-Text Folio entered at tick 0, tick 12,800 is phrase `k = 10` (not `10 mod 8`), so with an 11-genre history it plays the quote of `history[10]`.
58. Closing manuscript: §8.5a's worked example yields exactly the three quoted sections with section seeds `0xe3bffbb566a8741e`, `0xf0324ad0cf4755ff` and `0xee582d0bb6b9f352`; a `set_aside` ending shows no manuscript; no manuscript exceeds 2,400 characters or contains a digit.
59. Hash split: changing one `loc.` string or one swatch of an `art.palette.<g>` record changes `presentation_hash` and leaves `content_hash` unchanged, and two co-op peers whose content differs only in `art.`, `aud.`, `nar.` or `vfx.` records or `loc.` tables join the same run (Canon §2.4).
60. Margin arcs: all 54 beat keys exist in `en-US`: 8 residents × 6 beats of 2 to 6 lines, each at most 90 characters, and 6 Narrator notes of at most 2 sentences; no beat contains a digit.
62. **Second Edition Author and endings (§8.3, §8.5).** The Author boss presentation specifies art.visual.boss.author as a writing hand and quill with no face or body mesh; a Second Edition victory selects nar.ending.second_edition with the Ironman plate when no deaths occurred.

61. Gore: with `access.gore` at its default `ink`, a wound decal in a Wuxia Folio uses wuxia's `ink` `#141414`, and a severed arm sheds 8 glyph particles over its 120 ticks while staying lootable; `full` uses `#8A1010` and sheds none; `minimal` places no decal.
62. Madness: with `access.mark_hallucinations` and Mercy both off, a madness phantom renders identically to the native template it imitates; with the setting on, it also draws doc 17's dashed outline and "?".
63. Mounts retired: §3.6 has a `charger` row and no `cavalry` row; a charger whose idle lead foot is 600 mm ahead of its trailing foot passes and one at 300 mm fails; §10.2 has no mount class; no record of this chapter names a Canon §17.7 retired ID.
64. Expansions: with no expansion in the content set, every per-genre count of this chapter is unchanged (16 palettes, 240 damage names, 36 quote stems); an expansion whose `art.palette.<exp_g>` swatch 0 differs from its Canon §5.9 row fails the lint.
65. Appearance presets: each of the 6 kin has exactly 12 `art.preset.<kin_suffix>.<nn>` records and one `.colors` record of 8 base and 8 marking swatches; every one of a kin's 768 looks passes §5.6's limits against preset `01` with swatches 0 and 0; a marking swatch within ΔE 10 of `#D7262E` fails the lint; two runs that differ only in an Erratum's `Appearance` have the same `content_hash` and the same `state_hash` sequence.
66. Second Edition endings: a `mode.second_edition` victory whose Ur-Text clear had a solo host with no Red Ink dose and no Going Native shows `written_back` with its §8.5a manuscript, then `second_edition`; with `ironman` true the second layer shows the Ironman line; no Second Edition victory selects `short_story`; a Second Edition wipe that ends `run.end.erased` in Volume 7 with a stored `score_at_victory` shows only `erased` with Volume 7's plate text and no manuscript.

## Invariants annex

```
Σ 18: art ⊗ vfx ⊗ audio ⊗ text skins ⊗ lore ⊗ narrator ⊗ scribe ⊗ assets
W! presentation reads sim                                         N! art|aud|vfx|nar|loc → sim state
W! content integers (_milli, mm, ticks, Bam, Bp)                   N! float ∈ content
W! narrative draws := nar_rng(run, folio, nar.*, key)              N! presentation draw ∈ rng.* stream
W! palette[0..3] = Canon §5.9                                      W! render_style = doc 12 §3.1
W! max(cr(danger,ink), cr(danger,paper)) ≥ 4.0                     W! cr(ink, paper) ≥ 7.0 ∧ cr(pencil, paper) ≥ 4.5
W! damage color = Canon §10.1 default                              W! one VFX ∧ one audio record per event (dominant type)
W! color ∧ icon ∧ pattern per signal                               N! color as the only signal
W! visual darkness = sim darkness (doc 02 clamp)                   W! visual fog = sim concealment ± one 8 m step
N! cue says hidden|unheard when sim says seen|heard                W! foreign entity ⇒ origin style, ink, anim_step
W! awareness glyph := StealthHud.marks state (doc 09 §6.1)        N! glyph through walls unless st.marked
W! visible mesh within 150 mm of hit capsule                       N! ragdoll|IK|debris → sim
W! combat clips ∧ viewmodel on 1s                                  W! clip rate = clip_ticks / action_ticks
W! bbox height = stand_mm ± 150                                    W! block texels = 64 per m, triplanar (Canon §4.4)
W! bar_ticks ∈ ℕ ∧ level change at bar line                       W! ev.noise audible ⇔ heard[slot] set
W! pools never borrow                                              N! combat request takes a stealth voice
W! −18 LUFS ± 1, ≤ −1 dBTP                                         N! voiced Narrator | voiced patron
W! skin names pass §7.1 review                                     N! casino lexicon | brand | cur/ent name | "Ink" in skins
W! 15 distinct damage names per genre                              W! Editors speak style-guide rulings
N! gendered pronoun for procedural names                           N! flavor states numbers | rules | hidden curse
W! public-domain Echo ⇒ pre-1900 source only                       N! modern adaptation's look | name | catchphrase
W! closing manuscript := f(RunSummary, history, nar_rng)            N! stored manuscript text | digits in it
W! Scribe ⊂ {Margin, lobby, InColophon, ExitVote, VictoryChoice, Ended, page turn}
W! every Scribe job has template fallback of same JSON shape       W! schema ∧ filter ∧ deadline ⇒ else template
N! Scribe text ∈ saves.sim | replays | RunSummary                  N! voiced line rewritten by Scribe
W! scribe default off ∧ opt-in ∧ local opt-out beats host text    N! player-typed text ∈ prompt
W! asset ⇒ manifest(owner ≠ reviewer, license ∈ allowed)          N! shipped generated voice | music | living-artist prompt
W! art|aud|nar|vfx|loc ∈ presentation_hash ∧ ∉ content_hash      N! mount | vehicle | rider | item wear in any record
W! access.gore default ink; red blood only under full             N! madness phantom marked unless access.mark_hallucinations
W! underlayer reveal only on destructible cells                   N! voiced Margin arc beat | voiced closing manuscript
W! appearance preset ⇒ §4.7 limits vs kin default                 N! Appearance ∈ sim state | content_hash
W! Second Edition victory ⇒ ur_clear_ending, then second_edition  N! Second Edition victory → short_story
```
