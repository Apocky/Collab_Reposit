# ID reconciliation: alchemy cluster (owner doc 08)

**Namespaces:** `liq.`, `gas.`, `ing.`, `alch.` and `apparatus.` (Canon §19).

**Owner chapter:** `spec/08_ALCHEMY_LIQUIDS.md`. No other chapter was edited.

**Mapping:** `map_alchemy.json` is `{}`. No chapter uses a variant spelling, a synonym or an invented name inside these namespaces, so there is nothing to rename.

**Inputs:**
- the output of `tools/spec_lint.py --json`, before and after the edits;
- a scan of every chapter for these prefixes in backticks, in JSON string values, in brace forms such as `alch.potion.{dragonfire, snap_freeze, corrosive_gas}`, and in relative forms such as `alch.bomb.frost`, `.acid`, `.shock`;
- an unbackticked grep for the same prefixes;
- a grep for near-miss namespaces: `potion.`, `liquid.`, `bomb.`, `vessel.`, `reagent.` and similar. None were found.

## Inventory

Doc 08 defines and exports 316 IDs. Its Exports list matches its definition tables exactly, with no `fnv1a32` collisions.

| Namespace | Count |
|---|---|
| `liq.` | 35 |
| `gas.` | 13 |
| `ing.` | 95 (72 effect ingredients, 5 modifiers, `ing.essence`, `ing.black_powder`, 16 seeds) |
| `alch.eff.` | 59 |
| `alch.potion.` | 29 |
| `alch.bomb.` | 14 |
| `alch.rx.` | 19 |
| `alch.tx.` | 33 |
| `apparatus.` tools | 5 |
| `apparatus.vessel.` | 11 |
| `apparatus.respirator.` | 3 |

84 distinct concrete IDs are used outside doc 08, counting brace and relative forms. All of them match a doc 08 ID character for character.

| ID(s) | Using chapters |
|---|---|
| `liq.water` | 00, 03, 04, 05, 06, 07, 09, 12 |
| `liq.red_ink` | 00, 01, 03, 04, 06, 12, 13, 18 |
| `liq.oil` | 00, 01, 03, 06, 12 |
| `liq.acid` | 00, 06, 12 |
| `liq.alcohol`, `liq.lava` | 00, 12 |
| `liq.blood` | 09, 12 |
| `liq.holy_water` | 06, 10 |
| `liq.nitro` | 03, 09 |
| `liq.mana` | 06 |
| `liq.molten_metal` | 07 |
| `liq.mud`, `liq.phosphor` | 09 |
| `liq.unholy_water` | 10 |
| `liq.wellspring` | 12 |
| `gas.steam` | 00, 03, 06, 11, 12, 13 |
| `gas.miasma` | 00, 03, 06, 12, 13 |
| `gas.smoke` | 00, 03, 05, 06, 13 |
| `ing.charcoal` | 07, 17 |
| `ing.coffee_bean`, `.earthroot`, `.firebloom`, `.ginseng`, `.glowshroom`, `.mint_leaf`, `.starflower`, `.sungrass`, `.troll_fat` | 07 |
| `alch.eff.restore_aether` | 06 |
| `alch.eff.night_eye` | 09 |
| `alch.potion.liquid_flame` | 01, 10, 11 |
| `alch.potion.healing`, `.frost`, `.slumber`, `.toxic_gas` | 10, 11 |
| `alch.potion.dragonfire`, `.snap_freeze`, `.corrosive_gas` | 10 (relative forms), 11 (brace) |
| `alch.potion.stamina`, `.aether`, `.calm`, `.haste`, `.invisibility`, `.levitation`, `.purity`, `.mind_vision`, `.fortitude`, `.cure`, `.night_eye`, `.draught` | 11 |
| `alch.potion.shielding`, `.adrenaline`, `.shadowmeld`, `.cleansing`, `.panacea`, `.honeyed_healing`, `.dragons_blood`, `.aquatic_rejuvenation`, `.arcane_armor` | 11 (brace) |
| `alch.bomb.firebomb`, `.frost`, `.acid`, `.shock`, `.smoke`, `.flash`, `.salt` | 11 |
| `alch.bomb.blasting`, `.incense`, `.infernal`, `.sleep`, `.stink`, `.tar` | 11 (brace) |
| `alch.tx.consecrate`, `alch.tx.uncurse` | 10 |
| `apparatus.mortar`, `apparatus.alembic` | 14 (also as the brace form `apparatus.{mortar, alembic}`) |
| `apparatus.vessel.vial`, `.flask`, `.bottle` | 11 |
| `apparatus.vessel.crucible` | 07 |

**Pattern references.** Other chapters also use pattern references. These are not IDs, and each resolves to a doc 08 table:
- `alch.*` (03, 11);
- `alch.bomb.*` (05, 11);
- `alch.potion.*` (10, 11);
- `apparatus.*` (11, 14);
- `apparatus.vessel.*` (11);
- `apparatus.respirator.*` (05);
- `liq.*` and `gas.*` (03);
- `liq.<name>` (12);
- `ing.seed_*` (07).

**Bare per-genre lists.** Doc 12 §3.5's per-genre `liquids` lists use bare suffixes (water, oil, lava, blood, alcohol, acid, red_ink). Each is a doc 08 liquid. The genre sets agree with doc 08 §3.5's worldgen sources for all seven liquids.

## Decisions

1. **`alch.tx`** (doc 10 Interface assumptions: "`alch.tx` curse-state rules as listed in §7.6"). This was the only ID in these namespaces that the linter reported as undefined.
   - It is the bare name of the transmutation sub-namespace, not a content ID. In context it means `alch.tx.consecrate` and `alch.tx.uncurse` together, so mapping it to one ID would be wrong.
   - **Decision: define the sub-namespace prefix in doc 08's Exports.** The old sentence wrote the sub-namespaces with trailing dots (`alch.tx.`), which no consumer can cite. The Exports section now has a "Sub-namespace prefixes" list giving each one bare, with its category and defining section: `alch.eff`, `alch.potion`, `alch.bomb`, `alch.rx`, `alch.tx`, `apparatus.` tools, `apparatus.vessel` and `apparatus.respirator`.
   - The list states that these prefixes are not content IDs and are not counted among the 316, so acceptance test 36 is unchanged.
   - Doc 08 already cited `alch.rx` bare in Exports. The linter now finds no undefined ID in these namespaces in any chapter.
2. **Doc 10 namespace violations**: `alch.potion.healing`, `.frost`, `.liquid_flame`, `.slumber`, `.toxic_gas`, `alch.tx.consecrate` and `alch.tx.uncurse`.
   - The linter flags these because doc 10 puts them in the first cell of three tables: its §6.1 stored-value example, its §7.6 curse-removal source table and its §8 table of dangerous unknown consumables.
   - Each ID is already defined in doc 08 with the same spelling, and doc 10's data agrees with doc 08:
     - the healing potion's base value is 30 (§12.1);
     - consecrate gives `buc.blessed` and uncurse gives `buc.uncursed` (§15.3);
     - a holy-water base gives blessed and an unholy-water base gives cursed (§9.2);
     - there are 29 potion types and 13 rows in §12.2.
   - **Decision: keep doc 08 as owner, no rename.** Nothing needed adopting. Doc 08's doc 10 interface row now states that those rows refer to doc 08 records and mint no IDs.
   - The linter will keep reporting these rows until doc 10 moves the IDs out of the first column. That is a doc 10 edit, outside this cluster's mandate.
3. **Doc 11 namespace violations**: `alch.potion.healing`, `.stamina`, `.aether`, `.calm`, `.haste`, `.invisibility`, `.levitation`, `.purity`, and `alch.bomb.firebomb`, `.frost`, `.smoke`, `.flash`, `.salt`.
   - These are first cells of doc 11 §3.5's found-potion weight table and its bomb value table.
   - Doc 11 §3 already declares that it attaches loot records to owner IDs and mints none.
   - The bomb values there equal doc 08 §13.2: firebomb, frost, acid, shock and flash 12; smoke 10; salt 14; the six pool bombs 14. The vial, flask and bottle values (1, 2, 3) equal doc 08 §6.1.
   - **Decision: keep doc 08 as owner, no rename.** Doc 08's doc 11 interface row now says these rows are loot records keyed by doc 08's IDs and that their bomb values equal §13.2. The fix in doc 11 (moving IDs out of the first column) is outside this mandate.
4. **Doc 08's false definitions of foreign IDs.** Doc 08 put `st.erasure` in a first cell of its §5.0 Canon-conformance table, and `st.blessed` and `st.confused` in first cells of its §12.5 wellspring table. The linter therefore listed doc 08 as a co-definer of those status IDs, which are owned by doc 00 and the §11.1 behavior owners.
   - **Decision: restructure without changing data.** §5.0 gains a leading row-number column (rows 1 to 18). §12.5's columns are reordered to Weight, Outcome. Rows, values and order are unchanged, so the `weighted` draw order is unchanged.
   - Doc 08 no longer appears among the definers of any `st.` ID.
5. **`item.` aliases of alchemy content (not mapped).** These IDs are in doc 11's `item.` namespace, not in this cluster's:
   - doc 14's provisional kit IDs `item.potion.{healing, haste, aether, invisibility, shielding, fortify, antidote, clarity, focus, regeneration}`;
   - `item.throwable.fire_flask` and `item.throwable.poison_flask`;
   - `item.container.flask`;
   - doc 11's `item.face.respirator`.

   Doc 11's kit rebinding table resolves them onto doc 08 IDs: `alch.potion.*`, `alch.bomb.firebomb`, `alch.potion.toxic_gas` and `apparatus.vessel.flask`. Doc 08 Canon gaps 8 already aliases `item.face.respirator` to `apparatus.respirator.mask`. No entry was added to `map_alchemy.json`, so this file cannot conflict with the items cluster's map.

## Unresolved

None. Every concrete ID in these namespaces that any chapter uses is defined in doc 08 with complete data.

## Observations for other owners (not edited)

- **Crits on doc 08 bombs and flasks.** Doc 05 §9.3, §9.4, its gaps 22 and its doc 08 interface row say doc 08 bombs and flasks are area hits with `NO_CRIT`: they draw the crit roll and ignore it. Doc 08 §7, §13.2 and Canon gaps 14 say thrown splashes and bombs are doc 05 area hits that draw one keyed `rng.combat` crit roll per target and take the rest of Canon §10.5, so the crit can apply. Both chapters agree on the draw count (acceptance tests 29 and 50 in doc 08, test 53 in doc 05). They disagree on whether the result applies. This is a rule conflict, not an ID conflict.
- **Doc 08 uses `mat.coal` and `mat.flesh`** (in `alch.tx.charcoal` and `alch.tx.lamp_oil`). Doc 07 uses both but does not define them where the linter looks. They belong to the doc 07 cluster.
- **Doc 08 still appears as a co-definer** of `cat.potion`, `cat.throwable`, `cat.tool`, `cat.container`, `slot.face`, `ev.potion_brewed` and `ev.liquid_reaction`, because its Exports section names them as the categories and events it uses or emits. This was left as is. Those namespaces belong to docs 00, 03 and 11.
- **Doc 11's kit rebinding** maps `item.potion.focus` (Gunslinger kit) to `alch.potion.calm` (composure). Doc 08 has an `alch.eff.focus` effect but no focus potion. If doc 14 wants a `st.focused` kit potion, the items cluster should raise it with doc 08.
