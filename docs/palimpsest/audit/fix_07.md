# Fix log: chapter 07 (Crafting, Materials and Building)

Chapter fixer for doc 07, moving it to canon-1.11. Sources: DECISIONS.md (rows 1 to 88 and the orchestrator rows), canon-1.11, `audit/canon_gaps_triage.md` (rows naming doc 07, §4 conflicts C-23, C-24, C-25 and C-31, §4.1 stale gaps), `audit/notes_*.md`.

## Plan (status updated after each batch)

- [x] Batch 1: header, intro, Imports, §1 constants, §2 materials, §3 fae iron
- [x] Batch 2: §5 stations, §7 parts legend, §8 rewrite (no durability), §9 nodes, §14 examples
- [x] Batch 3: §10 blocks (new blocks, micro-voxels, placement, erasure), §11 gadgets
- [x] Batch 4: Exports, Interface assumptions, Canon gaps, Acceptance tests, Invariants
- [x] Lint

## Changes

### Batch 1 (done): header, intro, Imports, §1.2, §2, §3.2

- Header: "Conforms to canon-1.11" (triage G-051). Intro rewritten: owns salvage and reforging instead of durability; names canon-1.11's limited building (DECISIONS 16), no item durability (DECISIONS 65), 250 mm micro-voxels (DECISIONS 37), mounts cut (DECISIONS 55) and Margin trophies (canon §17.8).
- Imports: canon §4.3 and §4.4 row (destructible tag, projectiles stopped by fixed terrain, no durability, micro-voxels); canon §3.3 owner-added stream uses (G-025); step 3a fae iron (G-086); invaders never erase (G-113); tall grass wording (G-124); §13.1 trap categories (G-128), §13.2 ward chain (G-129), §13.8 block-HP uses (G-132), §13.8a; §17.7 retired mount IDs and §17.8 trophies replace "mounted placement limits"; doc 03 row adds micro-voxel masks, falling blocks and §13.8 `is_errata_placed`/`erase_placed` (G-050, C-24); doc 10 row drops `curse.brittle`, doc 14 row drops `run.marg.brittle_kit` (C-31).
- §1.2: deleted `DUR_BASE`, `WEAR_RATE`, `REPAIR_KIT_USE`; added `SELF_REPAIR_UNITS` = 1 (C-31; replaces the repair-kit wear that doc 04 `self_repair` spent).
- §2: toughness no longer gives durability; deleted `dur_mat_bp`; retired the brittle material tag (its only rule was the durability shatter) and removed it from the 7 CSV rows (cast_iron, obsidian, terracotta, glass, rad_glass, psi_crystal, ice); `pen_cost` read only for destructible or non-opaque cells (G-037); full-cell `conceal`/`light_filter`/`sound_atten_ddb` scaled by fill fraction (canon §4.4); snow decision: no snow material or block, `noise.surface.snow` has no carrier (G-203).
- §3.2: Erratum `kin.fae` targets use canon §10.5 step 3a × 12500 for iron and cold-iron primaries (G-086).

### Batch 2 (done): §5, §7, §8, §9, §11.2 learn flag, §14

- §8 rewritten (C-31, canon §4.3, DECISIONS 65): deleted maximum durability, wear table, condition states, item repair and the brittle shatter. New §8.1 "No item durability" (block HP is not durability; what runs out is a consumable) plus the governing-skill list that `recipe.bench.modify`, salvage and Recast still need (now total: Inscription adds wands, Tinkering covers every other item). New §8.2 "Construct self-repair supplies": a `repair_kit` tool is never consumed; each doc 04 `self_repair` use consumes `SELF_REPAIR_UNITS` = 1 cheapest carried `metal` unit (ores excluded), refused with `no_supplies` otherwise. Old §8.5 salvage is now §8.3, old §8.6 reforging is now §8.4 (Recast no longer "preserves the durability ratio"); every in-chapter reference updated.
- §5.1: workbench and forge lose "Tinkering repair" and "Smithing repair"; loom found in `room.workshop` and any other doc 12 room that lists it (drops the mount-era `room.stable` reference, DECISIONS 55).
- §7.2 legend and §7.3 good-direction list drop the `U` durability tendency letter.
- §9.3: node block records made exact (reconciler question 1 in notes_materials.md): record material per node kind, fixed Hardness (stone 6, timber 3; yields still follow the genre), nodes never burn or melt. §9.4: vein count cites doc 12 §11.2's `6 + floor_div(d, 5)` (stale gap 13, triage §4.1).
- §11.2: gadget recipes are `learn` 0 (reconciler question 7; matches the §6.6 L-flag convention).
- §14: removed durability results from §14.1 and §14.2 and pick wear from §14.3.
- §1.1: material tags 21 → 20; §1.3 and other section pointers moved to §8.3/§8.4.

### Batch 3 (done): §10 blocks, §11 gadgets

- §10.1: four new blocks so every block kind other chapters rely on has a doc 07 record (task: doc 07 owns `block.*`; canon §4.3 as amended by G-036 names rubble, pit lids and hull panels as destructible): `block.altar` (worldgen patron altar for docs 06, 10, 12; fixed terrain, `block.tag.altar`; reconciler question 2), `block.rubble` (destructible, `gravity` 1; doc 12 falling debris and tremor), `block.pit_lid` (destructible top slab; doc 12 spike pit and collapsing floor), `block.hull_panel` (destructible plasteel, hardness 1000; doc 12 hull breach). Blocks 129 → 133.
- §10.1: fixed-terrain consequences quoted from amended canon §4.3 (G-036); unlit twins keep the lit block's tags, so `block.shrine_lamp_out` stays an altar (reconciler question 3); tall grass conceals only crouched or downed targets and corpses (G-124); altar tag notes doc 10 sells `appr.svc.*` only at worldgen altars (G-145).
- §10.3: placement no longer mentions mounts (canon §17.7, DECISIONS 55); rule 5 drops the separate `placed_by_errata` bit: `placed_meta` is the one registry, read through doc 03 `is_errata_placed(c)` (G-050, C-24).
- §10.4: placeable repair restores micro-voxels (§10.8). §10.5 rule 4: invaders never erase placeables (G-113).
- New §10.8 Micro-voxels (canon §4.4, DECISIONS 37): shapes are doc 03 masks of 250 mm micro-voxels; chipping rule `voxels_left = ceil_div(n_shape × hp_left, hp_total)` couples block HP to the mask so it empties exactly at HP 0; repair refills in ascending bit index; door, trapdoor, ladder, fence, pane, cross, fixture shapes and nodes never chip; worked barricade example (59 voxels after one 5,000 hit, 6 after 11 hits).
- §11: Tinkering no longer sets repair; gadget JSON drops `dur_base`; §11.3 drops the durability check, all gadget wear and the Broken row; a gadget item never breaks (C-31).

### Batch 4 (done): Exports, Interface assumptions, Canon gaps, Acceptance tests, Invariants

- Note for any resumer: the shared scratchpad `ed.py` was overwritten by another chapter's fixer (it now targets doc 11). Doc 07 scripts live in `scratchpad/fix07/` with helper `ed07.py`; never import `ed.py`.
- §1.1 and Exports: blocks 129 → 133 (54 "other blocks" with `block.altar`, `block.rubble`, `block.pit_lid`, `block.hull_panel`); material tags 21 → 20 (brittle retired); exported functions drop `dur_max` and `wear`, add `voxels_left(cell)`.
- Interface assumptions: doc 03 row asks for C-25 (no `item.tool.torch` placement), `is_errata_placed` as the only registry, erasure excluding the invader (G-113), micro-voxel masks, §10.8 chipping, decay-safe masks, repair refill and fill-fraction scaling; doc 04 row drops Smithing 25 and `field_repair` (item repair is gone) and states the §8.2 self-repair supplies; doc 05 row reads `pen_cost` only for destructible or non-opaque cells (G-037); doc 06 row names `block.altar` and limits altar-breaking favor loss to shrine lamps; doc 09 row drops lockpick snaps and hacking-deck wear (C-31) and records the snow decision (G-203); new doc 10 row (`appr.svc.*` at `block.altar` only, G-145); doc 11 row adds the `repair_kit` tag and no durability, `U` tendency or durability gimmick (C-31); doc 12 row cites its `6 + floor_div(d, 5)` veins and asks for the four new blocks; doc 13 row drops the dirt request (C-23 done on disk) and uses `is_errata_placed` (C-24); doc 14 row drops `run.marg.brittle_kit` (C-31); docs 16–18 row: no durability display, chipped-mask rendering.
- Canon gaps: deleted gaps 1–20 except the torch (resolved per triage: G-051 gap 1; G-036 gap 2; G-192 gap 3; G-135 gap 4; G-055 gap 5; G-025 gap 6; G-204 gaps 7, 10, 12, 19; G-126 gap 8; G-129 gap 9; §4.1 stale gaps 11, 13, 16, 20; G-193 gap 15; G-128 gap 17; G-132 gap 18). Remaining open items: 1 torch placement until doc 03 applies C-25; 2 plants and doc 08 reaction products are destructible but not in canon §4.3's list (request to canon); 3 thin shapes and nodes break whole instead of chipping (request to canon §4.4).
- Acceptance tests: 3 is now "No durability"; 11 drops durability; 18 is self-repair supplies; 19 is chipping; 20 is the four new blocks; 22 drops "while mounted"; 33 adds the invader; 51 counts 20 tags and 133 blocks; new 52 node records.
- Invariants: durability replaced by salvage and micro-voxels; placement no longer names mounts; `placed_meta` is the registry; no item durability, wear or repair; self-repair consumes one metal unit; chipping invariant; invader never erases.

### Batch 5–8 (done): small alignments

- §6.4: `StartCraft` is doc 02's `Craft` intent, logged in the replay log (G-016). §10.4: block-HP tables cite canon §13.8's uses table (G-132).
- §8.2: the self-repair metal unit is reserved at channel start and consumed at completion, like crafting inputs; a cancelled channel consumes nothing.
- §11.1 rule 4 and the doc 05/06 Interface rows: `note_contradiction_failure(t)` is doc 06 §3.9's one per-character counter (G-057); doc 06 Imports row adds §3.9.

### Lint after edits

`python3 tools/spec_lint.py --chapter 07`: no missing sections, no undefined IDs, no foreign-namespace definitions, no retired IDs, no JSON issues, no hedges (words 25881). The `--json` multiply_defined map lists doc 07 only for `block.air`, `block.cover.sandbag`, `block.door.wood`, `block.masonry.marble`, `block.powder_keg`, all from doc 03 §4.5's illustrative JSON `"id"` values (request below). A scan of every chapter finds no `block.`, `mat.`, `station.`, `recipe.`, `part.`, `tooltier.` or `gadget.` ID that doc 07 does not export.

## Requests for other chapters

- **00 (canon):** §4.3 destructible list adds "plants, doc 08 reaction products" (doc 07 gap 2); §4.4 adds "A destructible cell whose shape occupies no micro-voxels or whose mask follows its props breaks whole at 0 block HP (doc 07 §10.8)" (doc 07 gap 3).
- **03:** §13.3 drop "and `item.tool.torch`" (C-25). §13.8 `sys.erasure`: every `fac.editors` member except the invader (canon §11.6 as amended by G-113), not "hunters and the invader included". §4.5 illustrative records: stop declaring `block.air`, `block.masonry.marble`, `block.door.wood`, `block.cover.sandbag`, `block.powder_keg` through JSON `"id"` (for example key them `"example_of"`), or align their values with doc 07 §2.5/§10.1 (marble `tool_tier` t0, `blast_res_milli` 4000, `heat_cond_bp` 2800, `sound_atten_ddb` 370). Implement doc 07 §10.8: predefined micro-voxel mask per §4.4 shape, removal after each damage event down to `voxels_left` (inside the damage volume nearest the impact, then nearest the impact in the cell), masks kept when a decayed `block_damage` entry is dropped, repair refill in ascending bit index, fill-fraction scaling of `conceal`, `light_filter`, `sound_atten_ddb`.
- **04:** `self_repair`: "needs a carried `cat.tool` tagged `repair_kit`, never consumed, and consumes 1 `metal` material unit (doc 07 §8.2), refused with `no_supplies` without them". Remove item-durability perks: Smithing 25 "repairs restore × 15000", the "crafted gear max durability × 20000" half of Smithing 100 (keep `material_bp` +500), Armor 25 "armor durability loss × 7500", talent `field_repair`, the smithing/tinkering "repair" XP row (keep salvage), the `crafted_durability` key; Interface row for 07 reads "`repair_kit` tag (never consumed) and self-repair supplies" instead of "tool uses".
- **06:** worldgen altars are `block.altar` (fixed terrain); the 200-favor loss for breaking an altar applies only to `block.shrine_lamp` and `block.shrine_lamp_out`.
- **09:** Imports row for doc 07 reads "lockpick, toolkit and hacking-deck quality" (no durability); close G21: doc 07 adds no snow block, so `noise.surface.snow` stays unused or is retired.
- **10:** §5.9 names `block.altar` as the worldgen altar block.
- **11:** remove the `U` tendency letter and every durability gimmick (Dunwold, Rustmaw, Glass Cobbler, Hephaesteum), `affix.durability`, `affix.self_repair`; `item.tool.toolkit` use text "construct self-repair (`repair_kit`, doc 07 §8.2) and gadgets"; Interface row for 07 drops "durability".
- **12:** name `block.altar` in `room.shrine` and `secret.room.shrine`; `block.rubble` for `hazard.falling_debris` and `gpv.evt.tremor`; `block.pit_lid` for the spike-pit lid and collapsing-floor boards; `block.hull_panel` for `hazard.hull_breach` (instead of "`block.glass_pane` hardness"); retire or reword `room.stable` ("mount bay", DECISIONS 55); close G17 (canon §4.3 amended).
- **17:** §6 disassembly row cites "doc 07 §8.3" (was §8.5); no durability display.
- **18:** skins for `block.altar`, `block.rubble`, `block.pit_lid`, `block.hull_panel`; render chipped masks per doc 07 §10.8.

## Verification

Adversarial verifier pass, 2026-10-11, against the working tree (docs 03, 04, 06, 09, 10, 13 had been edited by their own fixers meanwhile). Edits were made by a Python script that asserts each anchor occurs exactly once (`scratchpad/verify07/edit1.py` plus two small follow-ups).

### Check 1: triage rows naming doc 07

| Row | Verdict | Chapter text that satisfies it |
|---|---|---|
| G-025 (gap 6) | applied | §1.3 lists the owner-added `rng.craft` uses with draw counts ("4 per quality-rolling output", "1 `roll_bp` per node broken", crop seed return); Imports "keyed streams `rng.craft`, `rng.combat`, `rng.loot` with this chapter's owner-added uses (§3.3)" |
| G-036 (gap 2) | applied | §10.1 "As Canon §4.3 says, such a block never burns, melts, falls, washes away, decays or explodes, and changes type only through worldgen, a scripted event or a §10.6 lit and unlit twin swap"; fixtures (500), `block.rubble`, `block.pit_lid`, `block.hull_panel` destructible |
| G-037 | applied | §2.3 `pen_cost` "read only for destructible or non-opaque cells because fixed terrain stops every projectile, beam and penetration (Canon §4.3)" |
| G-050, C-24 | applied, then extended | §10.3 rule 5 "`placed_meta` is the one registry of Errata-placed cells"; **verifier fix:** crops had lost their Errata-placed marker when the `placed_by_errata` bit was dropped, so §9.7 now writes a `placed_meta` entry with a crop flag (see edits) |
| G-051 | applied | header "Conforms to canon-1.11"; canon-1.8 and canon-1.10 applied (checks 2 and 4) |
| G-055 (gap 5) | applied | §7.5 "The gimmick is one component with doc 11's Dep column"; gap deleted |
| G-057 | applied | §11.1 rule 4 "doc 06 §3.9's shared `note_contradiction_failure(t)`, the one per-character counter" |
| G-086 | applied | §3.2 rule 4 "`mat.iron` and `mat.cold_iron` primaries use Canon §10.5 step 3a (every `d[t]` × 12500)" |
| G-124 | applied | §10.1 `block.tall_grass` "conceals only crouched or downed targets and corpses (Canon §12.2)" |
| G-126 (gap 8) | applied | §10.2 "an Errata-placed web roots only entities outside `fac.errata`" |
| G-128 (gap 17) | applied | Imports "block traps under `cat.placeable`; snares, mines and charges under `cat.gadget`"; §11.2 gadget JSON `"category": "cat.gadget"` |
| G-129 (gap 9) | applied | Imports "`slot.ward` capacity chain"; gap deleted |
| G-132 (gap 18) | applied | §10.4 "Placed blocks scale with `T` ... worldgen destructibles scale with `P` against Errata weapons (Canon §13.8 uses table; doc 03 §13.2)" |
| G-135 (gap 4) | applied | gap deleted; §8.1 "No item instance carries a durability value" |
| G-145 | applied | §10.1 `block.tag.altar` "doc 10 sells `appr.svc.*` only at a worldgen `block.altar`" |
| G-192 (gap 3) | applied | §2.4 "a bare tag always means `mat.tag.<name>` (triage G-192)" |
| G-193 (gap 15) | applied | §7.2 "This chapter owns every `part.` ID" |
| G-203 | applied | §2.4 "No snow material or snow-layer block exists (triage G-203)"; doc 09 §7.2 now keeps `noise.surface.snow` for data packs, consistent |
| G-204 | applied | gaps 7, 10, 11, 12, 13, 14, 16, 19, 20 deleted |
| G-210 | applied, sharpened | intro now "the hub has trophy and decoration slots, doc 14 §5.1a's trophy alcove, instead of a building plot (Canon §17.8)" |
| C-23 | done on disk | no chapter names `block.dirt` (grep of spec/ and data/) |
| C-25 | applied; gap now stale | §6.6 "`recipe.hand.wall_torch` ... shapeless 1 `item.tool.torch` → 1 `block.torch`"; doc 03 §13.3 now reads "A held torch never places a block ... (triage C-25)", so **verifier fix:** Canon gap 1 (torch) deleted as closed |
| C-31 | applied, two leftovers fixed | §8 rewrite; **verifier fixes:** `recipe.bench.lockpicks` yielded 5 picks, a yield sized for "lockpicks 1 per pick" wear (HEAD `DUR_BASE` row) while doc 09 now consumes none, so it yields 1; doc 04 kept `field_repair` as a placeable (block HP) repair talent, which §10.4 omitted (now applied) |
| §4.1 stale gaps 11, 13, 16, 20 | applied, one body leftover fixed | gaps deleted; §9.4 cites doc 12's `6 + floor_div(d, 5)`; **verifier fix:** §2.9 still mapped pre-canon doc 12 pool names ("Doc 12 §2.4 named pool IDs") that doc 12 no longer has (its §2.4 is now placed light; GPV dimension 20 is Canon §5.9 verbatim); rewritten as "Doc 12 material pools" |

### Check 2: DECISIONS rows

| Row | Verdict |
|---|---|
| 15 invasions | agrees; **verifier fix:** §10.5 rule 4 now also states Canon §17.9's "cannot ... break blocks" (the invader never mines, breaks, places or erases a block and its hits deal no block damage); test 33 extended |
| 16 limited building | agrees: §4.3-only placeables (barricades, gate, bridges, ladders, cover, spikes and webs, torch, lantern, campfire, shrine lamp), fixed terrain, resource nodes, trophy alcove |
| 20, 35 crafting | agree: stations in rooms and the Colophon, `station.hand` 3×3 grid anywhere, carried toolkit and buildable campfire as the field kit |
| 26 softer Erasure, 91 | agree: erasure by hunters and `fac.editors` except the invader; no 5400 repeat |
| 28, 33, 38, 42, 46, 69, 90 | not in doc 07; grep finds no Echo, glyph-library, voice, price, ending, roster or "Crowns" text |
| 37 micro-voxels | agrees: §10.8 (verified against doc 03 §4.4 and §13.2 chipping, which now match it) |
| 55 mounts cut, 89 | agrees: no mount, ride, saddle or cavalry text; §5.1 loom no longer names `room.stable` |
| 56 World Laws discovered | agrees: §9.3 reads `law.post_apocalypse.scarcity` only when active, no preview claim |
| 60 pact curses | agrees: crafted items are `buc.uncursed`; Recast keeps every other field |
| 65 no durability | agrees after the two C-31 fixes above |
| 77 hub | agrees (Margin "stations" are doc 14 facilities, not crafting stations) |
| 78–83, 86, 87 | no doc 07 surface; forge reforge Lucre is the crafter's own (83) |
| 84 crippled-limb severing | agrees: doc 07 has no severing rule (butchery of parts is doc 04 §8.6) |
| 85 co-op skill XP | **verifier fix:** Interface row 04 quoted the Tinkering gadget-hit row as `floor_div(750 × dmg, max)`; doc 04 and amended Canon §7.1 use `norm` |
| 88 Somnambule | agrees: §2.3 `pen_cost` rule; `part.graft.oneiric.somnambule` carries doc 11's amended gimmick |
| 99 weather, 106 Second Edition | not landed in canon or doc 07's sources yet; nothing to align |

### Check 3: chapter brief

- No item durability: §8.1 "Items never wear, lose condition or break"; no repair station or kit for gear; the `repair_kit` tag survives only as doc 04/11's never-consumed toolkit tag for Canon §16.3's construct "Tinkering repair heals" (doc 04 line 168 agrees). Block HP stays (§10.4, §10.7).
- Limited building: fixed terrain (§2.5, §10.1), destructible tags, `cat.placeable` only (§10.3 rule 1), resource nodes mined in place (§9.1, §9.3); trophy alcove in the intro.
- Stale gaps 11, 13, 16, 20 deleted (and §2.9 body fixed).
- `recipe.hand.wall_torch` → `block.torch`: present (§6.6, §10.3 rule 1, test 22).
- 250 mm micro-voxels: §10.8.
- `block.*` ownership: a scan of every chapter and `data/` finds no `block.` ID outside doc 07's Exports (the five doc 03 sample IDs are gone: doc 03 §4.5 now declares no block ID).

### Check 4: leftover grep

`mount|ride|saddle|cavalry|durab|repair|wear|broken|breaks|Delving|voice|VO|24.99|Item Codex|Crowns|Redactor|margin.plot|building plot|5400`: every remaining hit is a statement of the removal (§8.1, test 3, invariant), construct self-repair or placeable (block HP) repair, block breaking, or the historical "Fieldcraft hooks that replaced the Delving hooks" note. The lockpick yield of 5 was the one real leftover (fixed).

### Check 5: lint

`python3 tools/spec_lint.py --chapter 07` after the edits: no missing sections, no undefined IDs, no foreign-namespace definitions, no retired IDs, no JSON issues, no hedges (words 26273). `--json` `multiply_defined`: `{}` (doc 03 dropped its sample block IDs).

### Check 6: internal consistency

- Recomputed with Python, all match: §4.3 distribution table and test 8 counts (141³ enumeration); §2.2 unit columns, flammable tag and the 20-tag set; §2.6 ores; masonry table; §3.3 bronze piece; §8.2, §8.3 examples; §9.2 mining (101, 150, 225, 84, 90, 180, 100); §9.3, §9.6, §9.7 examples; §10.3 placement (60, 47, 40); §10.4 barricade (60,000 / 108,120 / 135,150 / 216,870; 12 and 13 hits; 6 mHP left); §10.7 doors (60,000; 710,190; 36,929 mHP; 20 swings; `break_ticks` 450); §10.8 chipping (59, 6); §10.4 spikes (10,400, 52); §11.4 turret (5,589; 54,507; 68,133); test 27 recharge; §14.1–§14.3 (277, 500, 338 → 310, 29,829; 245, 386 → 402, 21,590; 101, 219, 438, 762).
- Tables: every table row has its header's column count. Exports: 69 + 10 materials, 9 roles, 20 tags, 6 tiers, 9 stations, 5 kinds, 127 recipes (equal to the recipe IDs in the body), 29 parts, 38 grafts (one per doc 11 manufacturer except the two excluded), 133 blocks (equal to the block IDs in the body), 5 block tags, 13 gadgets.
- Canon gaps now hold only open items (2); Interface rows hold no request that is already done on disk (the doc 03 sample-record clause was replaced).
- **Verifier fixes found here:** doc 04 owns the Fieldcraft 100 perk as "your demolition charges break destructible blocks in a radius 1000 mm larger", but §11.1 rule 10 and §13 also grew the blast's damage radius; now `block_power_milli` +750 (doc 03 §12.1's conversion for `demolition`), damage radius unchanged, test 44 extended. §8.1 put staves under Smithing although `recipe.bench.staff` is an Inscription recipe; staves now govern by Inscription with wands, foci and tomes. Fieldcraft XP said "500 for an ore vein or salvage pile" while the Fieldcraft 75 perk covers every mineral or salvage node; both now say mineral or salvage node. Doc 09 §13.4 says "doc 07 rolls its [hacking deck's] failure spark", which doc 07 never defined; §11.1 rule 4, §1.3 and test 38 now define the roll and the spark, leaving the attempt's outcome to doc 09. `block.cover.barrier` cells carry `hp_max` (a `placed_meta` field) and feed the §10.5 breach rule, so the row now gives them `placed_meta` outside the caps. `block.powder_keg`'s `explosive` record is written out in §10.1 instead of "exactly as doc 03 §4.5 shows" (doc 03 says doc 07 owns block values).

### Verifier edits (chapter 07)

1. Canon gaps: deleted the closed torch gap (C-25 applied in doc 03 §13.3); gap 1 now also lists furniture and switches/fuse boxes as destructible kinds Canon §4.3 omits.
2. §9.7, §1.4, §1.2, §10.3 rule 2, test 33, invariant: crops are Errata-placed through `placed_meta` with a crop flag, outside the placeable caps, erased by Editors.
3. §10.4 repair formula and Interface row 04: doc 04's `field_repair` × 15000 after the Fieldcraft 75 factor.
4. §11.1 rule 10, §13 row 100, test 44: Fieldcraft 100 demolition = `block_power_milli` +750, radius unchanged.
5. §6.6 and §4.1: `recipe.bench.lockpicks` yields 1 set (lockpicks are never consumed).
6. §8.1: staves governed by Inscription.
7. §9.3 and §13: Fieldcraft XP 500 per mineral or salvage node.
8. §10.5 rule 4, Interface row 03, test 33, invariant: the invader never mines, breaks, places or erases a block and deals no block damage.
9. §11.1 rule 4, §1.3, Interface row 09, test 38: hacking-deck failure spark.
10. §10.1: barrier cells get `placed_meta`; powder keg `explosive` record written out; intro names doc 14 §5.1a's trophy alcove.
11. §2.9 rewritten (doc 12 uses Canon §5.9 IDs); Interface row 03 sample-record clause replaced; Interface row 04 XP row uses `norm`.

### Status of the fixer's requests on disk

- 00: open (§4.3 list, now "plants, furniture, switches and fuse boxes, doc 08 reaction products"; §4.4 thin shapes).
- 03: done: §13.3 torch (C-25), §13.8 erasure without the invader, §4.5 no longer declares block IDs, chipping and refill. New: `damage_block` adds 0 for the invader's hits; §12.1 lists Fieldcraft 100's +750 block power beside `sapper` and `demolition`.
- 04: done (self_repair, Smithing 25, Armor 25, no `crafted_durability`). Withdrawn: "remove talent `field_repair`", because doc 04 now makes it a placeable-repair talent, which DECISIONS 65 allows; doc 07 applies it.
- 06, 09, 10, 13: done.
- 11: done for the U letter, durability gimmicks and toolkit text. New: optional `shop.tinker` lockpicks × 1.
- 12: open (altar, rubble, pit lid, hull panel, `room.stable`).
- 17: open (§6 disassembly still cites "doc 07 §8.5"; should be §8.3).
- 18: open (skins for the four new blocks, chipped-mask rendering).
