# ID reconciliation: items cluster (doc 11)

Namespaces: `item.`, `rar.`, `affix.`, `perk.`, `mfr.`, `loot.`, `reliq.`, `engine.`, `shop.`, `cat.`, `slot.` (Canon §19, owner doc 11, core values in 00). Only `spec/11_ITEMS_LOOT_ECONOMY.md` was edited.

## Inventory

- Before: 474 distinct IDs in these namespaces were used across the spec, and 90 of them had no doc 11 definition the linter could see. Most were defined in compressed prose or brace rows (`item.ring.{...}`, `.primer_abjuration`) or were sub-generator tags.
- After: every used ID is defined in doc 11 or mapped, except the doc 02 mod example (see Unresolved). In a scratch copy, applying all `audit/map_*.json` and re-linting leaves one undefined ID in these namespaces, `item.x_frostpack.ice_blade`.
- Exports now lists all 582 IDs in full: item 147, affix 116, loot 106, perk 93, mfr 40, cat 28, slot 24, shop 11, rar 7, engine 6, reliq 4. Their `fnv1a32` values are pairwise distinct, also against canon IDs and the retired IDs.

## Renames (map_items.json, 22 entries)

| Old | Canonical | Used in | Why |
|---|---|---|---|
| item.potion.{healing, haste, aether, invisibility, shielding} | alch.potion.<same> | 14 | doc 14 kit names. Potions are doc 08 IDs. Same mapping as doc 11's earlier rebinding table |
| item.potion.fortify / antidote / clarity / focus / regeneration | alch.potion.fortitude / cure / purity / calm / honeyed_healing | 14 | same table. Effects match: fortify, cure poison, purify, composure, healing over time |
| item.throwable.knife, item.throwable.javelin | arch.ranged.thrown | 14 | thrown weapons are one archetype with knife and javelin skins (doc 11 §3.2) |
| item.throwable.frag / smoke | arch.ranged.grenade_frag / grenade_smoke | 14 | doc 05 grenades |
| item.throwable.fire_flask / poison_flask | alch.bomb.firebomb / alch.potion.toxic_gas | 14 | doc 08 bomb and aerosol potion |
| item.throwable.disruptor_flask | item.throwable.counter_disruptor | 14 | doc 11 counter grenade (Canon §17.3a) |
| item.container.flask | apparatus.vessel.flask | 14 | doc 08 vessel |
| item.food.snack | item.food.bread | 04 | doc 04's light-food row (snack, bread, fruit, produce, 300 satiation) is doc 11's bread |
| loot.drop.boss / loot.drop.editor | loot.boss.mini / loot.boss.editor | 13 | doc 13 used `loot.drop.<rank>` for the boss ranks. Doc 11 names those tables `loot.boss.*` |
| loot.mythic_fantasy.chest_common | loot.chest.plain | 02 (JSON) | the mod patch example targets a loot table that does not exist. Chests are not per genre |

Doc 11's rebinding table had the same renames. It is now a "Former names" table with the old names left unmarked, so the map tool does not rewrite it into "X → X" rows and the linter does not count it as definitions. Its `arch.` and `ammo.` rows were dropped because doc 05 §5.6 and `map_archetypes.json` own those. map_archetypes overrides doc 11 on two of them: hand_crossbow → pistol, and stone → thrown instead of `ammo.draw`.

## Defined or made explicit in doc 11

- **Sub-generator tags** (§2.3, new table): `loot.cat`, `arch`, `rarity`, `exotic`, `mfr`, `quality`, `material`, `affix`, `perk`, `legendary`, `glyph`, `spell`, `buc`, `illum`, `lucre`, `ammo`, `key`, `bundle`, plus the new `loot.book`. These are hash inputs and must not be renamed, because `fnv1a64("loot.cat")` is fixed by acceptance test 3. They are reserved in Exports. Docs 07 and 10 use `loot.perk` and `loot.buc`.
- **Pool tags** (§2.5, new table): all 12 `loot.pool.*` with their Bindery node. Doc 07's Exports listed `loot.pool.recipes_2` and `loot.pool.gadgets_2`. Doc 11 now owns them, under the same IDs.
- **Rings** (§3.4): the compressed rows are split into 15 rows, which defines `item.ring.will` (used in doc 10) and `item.ring.machine_tongue`.
- **Food** (§3.5): a table now holds ration, meal and bread. Bread satiation was corrected from 250 to doc 04's 300, and rot and eat ticks were copied from doc 04 §11.2.
- **Books** (§3.5): all 8 `item.book.primer_<school>` are defined. Doc 06's interface asked for alteration, conjuration, illusion, restoration and spellcraft. `item.book.schematic` is new (doc 07 §6.5 needs it). A new instance field `teaches` holds the glyphs or recipe. Sources: kits, a new glyph-tome line at `shop.arcanist` and a new schematic line at `shop.scribe` (Canon gaps 20). Books are not on the deck, so the §20 worked example is unchanged.
- **Keys and quest items** (§3.5): `item.key.*` and `item.quest.*` are now tables. `item.quest.folio_leaf` (doc 12 §20.5) is adopted: 50 g, value 0, Public. Doc 12 had requested it from doc 11.
- **Secrets** (§9.8): `loot.secret.{cache, vault, palimpsest}` are now a table. `loot.boss.palimpsestic` is listed in Exports.
- **Rarities** (§3.8) and **slots** (§3.9): canon §13.2 and §13.3 are restated per ID. Doc 15 had defined `rar.*` in its own odds table, and docs 06 and 08 named `slot.*` in their Exports. Those are consumer mentions and need no rename.
- **Extra hands**: `slot.hand_extra_1` (doc 04) and `slot.hand_extra_2` are defined. Doc 04 defines only `bp.arm_extra_1` and `bp.arm_extra_2`. They count in the Exotic weapon group (Canon gaps 19).
- **Retired IDs** (§3.10, new). `cat.block` and `item.relic.lattice_lens` appear only in history notes (canon changelog, doc 10 gap 6, doc 14 gap 17). Mapping them would rewrite those sentences into nonsense, so they are kept as aliases instead.
- **Respirator**: `item.face.respirator` is retired as an alias of `apparatus.respirator.mask`, following doc 08 Canon gaps 8 (see notes_alchemy). Doc 08's sentences name the old ID, so it is not mapped. §3.4 now holds loot records for doc 08's cloth, mask and rebreather, with values 15, 40 and 90 and weights 40, 40 and 20. The weights sum to the old single respirator's 100, so other accessory shares are unchanged.
- **Mod segment rule** (§1 principle 8): `x_` second segments in doc 11 namespaces belong to mods (doc 02 G5).

## Left as is (not renames)

- `cat.*`, `item.text.*`, `item.tool.*`, `item.ring.keen`, `item.food.ration` and `item.food.meal` appear as first cells of consumer tables in docs 04, 06, 07, 08, 10, 14 and 15. These are references to doc 11 IDs under the same spelling. The linter's first-cell rule calls them definitions.
- Doc 11 still keys 49 loot-record rows on other owners' IDs (`arch.*`, `alch.*`, `gadget.*`, `counter.*`), which is principle 6. Notes_archetypes and notes_alchemy flag the same issue. The fix belongs in the table layout or the linter, not in ID mapping.

## Follow-ups for other chapters

- Doc 02's mod patch uses `"path": "/entries"`. Doc 11 tables have `lines`, and mods add templates through §2.1 records.
- Doc 14: the Duelist, Gunslinger and other kits now hand out `arch.ranged.thrown` stacks. Check the quantities against the 20 stack cap. Notes_archetypes raises the same point for the sling.
- Doc 14 Gunslinger lore potion: `alch.potion.calm` (composure) replaces "focus". If a `st.focused` potion is wanted, doc 08 must add one (see notes_alchemy).
- Canon 1.9 / DECISIONS 60 (pact curses as cursed Fortune Engine capsules) changes doc 11 §11. This is a design change, not ID reconciliation, so it was not applied here.
- Doc 12 §20.5 mentions a doc 11 "seat relic" with affix themes. Doc 11 has no such template yet. This is a design gap with no ID attached.

## Unresolved

- `item.x_frostpack.ice_blade` (doc 02) and `loot.x_frostpack.add_ice_blade` (doc 02, JSON `id`): illustrative mod IDs. Base content must not define them. The linter should exempt `x_` second segments, or doc 02 should mark the example as non-content.
