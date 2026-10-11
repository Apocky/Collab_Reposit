# ID reconciliation: materials cluster (owner doc 07)

Namespaces: `mat.`, `q.`, `station.`, `recipe.`, `part.`, `block.`, `tooltier.`, `gadget.` (Canon §19). Owner chapter: `spec/07_CRAFTING_MATERIALS_BUILDING.md`, the only spec file edited in this pass. Inputs: `tools/spec_lint.py --json`, plus a scan of every chapter for these prefixes in backticks, JSON strings and bare prose.

## Inventory

- The spec uses 478 distinct concrete IDs in these namespaces (backticked or JSON-quoted). 145 of them appear outside doc 07, in chapters 00, 01, 03, 04, 05, 06, 08, 09, 10, 11, 12, 13, 14, 16, 17 and 18.
- 477 of the 478 were already doc 07 content with full data: a §2.2 CSV row, a §6.6, §7.2, §8.5, §11.2 or §12.2 table row, a §10.1 block row, or a pattern such as `block.masonry.<m>`. The linter could not see many of them. It reads only Exports backticks, first table cells and JSON `"id"` values. The CSV rows (`brick`, `flesh`), the brace and shorthand forms (`mat.tag.{…}`, `.stone`, `.metal`, `t0` … `t5`) and the patterns all fall outside that.
- Before this pass the linter called these doc 07 IDs undefined: 24 used by doc 18, `mat.flesh` (docs 04 and 08), `mat.coal` (doc 08), and `block.tag.destructible` and `block.tag.errata_placeable` (doc 03). It flagged 43 more inside doc 07 itself.
- `q.*`: every use, in docs 00, 04, 07, 08, 10, 11 and 13, is one of Canon §13.5's seven values. No variants.
- No consumer chapter invented its own spelling in these namespaces except doc 13's `block.dirt`. A scan of look-alike namespaces (`material.`, `node.`, `item.gadget…` and similar) found nothing.

## Decisions

1. **`block.dirt` → `block.soil` (mapped).** Doc 13 uses it twice: the `sand_wyrm` "moves through `block.sand` and `block.dirt`" and its Interface assumptions row for 07. Doc 07 has no dirt block. Its §10.1 `block.soil` (material `mat.soil`, full, not destructible, `tillable`) is the terrain block meant, and doc 07 already read it that way (old Canon gap 20). The rename is in `map_materials.json`. Doc 07's Interface assumptions row 13 and Canon gap 20 now name the old ID without backticks, so the map tool leaves doc 07 alone, and gap 20 is closed.
2. **Definitions made explicit, no renames.** Every ID that was defined only by a pattern or shorthand is now written out in full in doc 07, with exact data at the chapter's level of detail. Nothing invented new game content. The 132 ID strings spelled out for the first time anywhere in the spec are these:
   - **Ores (§2.6).** A new table for the 10 `mat.<metal>.ore`. Columns are metal, `unit_mass_g` and `unit_value` (computed with the existing §2.6 formulas: `mat.iron.ore` 1376 matches acceptance test 2), source node and smelting recipe.
   - **Roles (§2.4).** A new table for the 9 `mat.role.*`. Columns are CSV code, §6.6 recipe key letter and reference material, all taken from text already in the chapter.
   - **Tags (§2.4).** The 21 `mat.tag.*` now sit in the tag table's first column as full IDs. A sentence confirms that bare tags mean `mat.tag.<name>` (Canon gap 3).
   - **Tool tiers (§9.1).** A new table for `tooltier.t0` … `t5`. Columns are who holds the tier, `TOOL_SPEED`, `TOOL_TERM` (from §4.2) and which §9.3 nodes the tier can mine. Two rules are made explicit: no tier-0 material has the frame role, so a toolkit is always at least t1, and `TOOL_TERM` t0 means no toolkit is carried.
   - **Grafts (§7.5).** A new table lists all 38 `part.graft.<genre_suffix>.<name>`. Each is doc 11 §5.2's manufacturer ID with `mfr.` replaced by `part.graft.`, and the table gives each one's `origin_genre`. It excludes First Press and Marginal Press, as §7.5 already says.
   - **Gadget recipes (§11.2).** The 13 `recipe.bench.<gadget suffix>` IDs are now listed, along with the schema fields the table already implied. That brings the 127-recipe count to 114 tabled plus 13 listed.
   - **Blocks.**
     - Shorthand first cells in §9.3 and §10.1 (`.tin`, `.stone`, `.metal`, `.rope`, `.plate` and the rest) are expanded to full IDs.
     - A new §10.1 table covers the 5 `block.tag.*`, with each tag's meaning and the blocks that carry it.
     - A new §10.1 masonry table covers all 24 `block.masonry.<m>`. It gives the §2.5 defaults computed from §2.2: `blast_res_milli`, `heat_cond_bp`, `sound_atten_ddb`, `conceal`, `light_emit` and `acoustic`. It also gives each block's walls and floors use in doc 18 §3.4. The 24 materials match doc 18 §3.4 exactly; its sand and soil floors are `block.sand` and `block.soil`.
     - A new §10.6 table covers all 16 `block.light_fixture.<genre_suffix>` with their `_out` twins and their `lightsrc` class, applying the existing rule.
     - Totals: 129 blocks and 5 tags.
   - **Exports.** The section now lists every concrete ID: 477 in all, including Canon §13.5's seven `q.` values, re-exported with no additions.
3. **Doc 11's gadget rows (no rename).** The linter flags six "foreign definitions": `gadget.turret_light`, `drone_scout`, `mine_frag`, `trap_snare`, `beacon_mend` and `decoy`. They come from doc 11 §3.6's loot-record table, which puts doc 07 IDs in its first column. Same spelling as doc 07. Stats match §11.2:
   - turret: deploy 60, lasts 1800, base 4 HP
   - drone: 3600 ticks, 12000 mm
   - mine: 30 HP pierce, 3000 mm
   - beacon: 100 Bp per second, 4000 mm, 600 ticks
   - decoy: 8000 mm, 300 ticks

   Nothing to adopt. The fix belongs in doc 11: key those rows on a non-ID first cell. Doc 07's Interface assumptions row 11 now records that doc 11 references these IDs and does not redefine them.
4. **Doc 03's sample block records (no rename).** These are `block.air`, `block.masonry.marble`, `block.door.wood`, `block.cover.sandbag` and `block.powder_keg`. The linter reports them as defined in both 03 and 07 because doc 03 §4.5 gives them JSON `"id"` values. Doc 03 calls them illustrative and defers to doc 07 §10.1, and Canon §19 shares `block.` with doc 03, so there is no violation and no rename. Doc 03's sample values differ from doc 07's §2.5 defaults:

   | Field | Doc 03 sample | Doc 07 default |
   |---|---|---|
   | marble `blast_res_milli` | 6000 | 4000 |
   | marble `heat_cond_bp` | 2000 | 2800 |
   | marble `sound_atten_ddb` | 400 | 370 |
   | marble `tool_tier` | t1 | t0 |
   | wooden door `blast_res_milli` | 3000 | 4500 |
   | wooden door `heat_cond_bp` | 1500 | 1900 |
   | wooden door `sound_atten_ddb` | 150 | 175 |
   | sandbag `blast_res_milli` | 3500 | 1000 |

   Doc 07's Interface assumptions row 03 now says doc 07 governs and quotes the marble values. Doc 03's owner should either align its samples or ask doc 07 to add §10.1 overrides; see question 6.
5. **Layout fixes in doc 07 (no renames).** Doc 07 had put other owners' IDs in its first table cells, which the linter counts as definitions. The archetypes and stealth reconcilers both deferred this fix to doc 07, so it is done here:
   - §2.4 acoustic table: columns swapped so materials come first. This clears 13 `noise.surface.*`.
   - §3.1 volume table: now keyed by a family column. This clears 33 `arch.*`.
   - §9.6 butchery table: now keyed by a plain corpse column. This clears 8 `body.*`.

   Doc 07 now has no foreign-namespace definitions and no undefined IDs.
6. **Doc 11's pool IDs in doc 07's Exports.** The old Exports backticked `loot.pool.recipes_2` and `loot.pool.gadgets_2`, which made doc 07 a second definer of doc 11's IDs. They are now named in prose ("doc 11's `recipes_2` loot pool").
7. **Doc 12's legacy pool names (no map entries).** Doc 07 §2.9 resolves names such as `starsilver`, `ebony` and `void_glass`. No chapter uses them as `mat.` IDs, and doc 12 no longer uses them, so they have no map entries. If any reappear, `data/aliases.json` should use §2.9's resolutions.
8. **Pattern tokens.** These are covered by the explicit lists and need no change: `block.*`, `mat.*`, `block.tag.*` and `tooltier.*` (doc 03); `block.masonry.<stone>`, `block.door.*` and `block.node.*` (doc 12); `part.focus.*` (doc 06); `gadget.*` and `part.*` (doc 11); `gadget.{turret_light, drone_scout}` (doc 14); `station.*` (doc 17). Bare-prose matches such as `block.friction_bp` and `q.min` are Rust field and variable accesses, not IDs.
9. **Linter note.** `load_ownership` reads the Canon §19 cell "07 (blocks shared with 03)" as two owners for all eight prefixes, so it treats doc 03 as co-owner of `gadget.`, `mat.` and the rest. Only `block.` is shared. This needs a linter fix, not a spec fix.

## Unresolved

None. Every ID in these namespaces that any chapter uses is now either written out in doc 07 with its data or mapped (`block.dirt` → `block.soil`). After `apply_id_maps.py` runs, the linter reports no undefined IDs in these namespaces. The remaining duplicate-definition reports come only from doc 03's illustrative JSON and doc 11's loot-row layout (decisions 3 and 4).

## Questions for the product owner

The user asked for a steady supply of questions. These came up during this pass, and each one blocks a small piece of doc 07 data:

1. **Node material.** Which material do `block.node.*` cells use for the §2.5 block defaults (conceal, acoustic, heat conductance)? An ore node yields `mat.<metal>.ore`, and ores are not rows of the material table. Candidate answers: the yield's metal, or the genre stone.
2. **Worldgen altar block.** Doc 06 prayers and doc 12 `room.shrine` and `secret.room.shrine` refer to a "patron altar" block, but no block ID exists for it. Should doc 07 add one (one per genre, or one skinned block carrying `block.tag.altar`), or should shrines place `block.shrine_lamp`?
3. **Unlit twins and tags.** `block.campfire_out` keeps `hearth`. Does `block.shrine_lamp_out` keep `altar`, so a snuffed lamp is still an altar?
4. **`mat.role.block`.** This role has no consumer. Eight masonry materials lack it: titanium, coral, dreamstuff, paper, vellum, birch, thornwood and blackwood. Should the role gate which materials may form blocks, or should it be dropped?
5. **Snow.** Doc 09 reserves `noise.surface.snow` until doc 07 adds a snow-layer block (doc 09 Canon gap G21). Do you want `mat.snow` and a `block.snow_layer`, for example for Fairy Tale or Cosmic Horror?
6. **Sandbag blast resistance.** The §2.5 formula gives a sandbag `blast_res_milli` of 1000, while doc 03's sample says 3500. Should sandbags be explosion cover? If so, doc 07 §10.1 needs an override.
7. **gadgets_2 recipes.** The `r2` weapon recipes must be learned from a schematic (flag L). Are the six `gadgets_2` gadget recipes learned the same way, or known by everyone once `unlock.loot.gadgets_2` is active?
