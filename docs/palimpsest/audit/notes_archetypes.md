# ID reconciliation: archetypes cluster (owner doc 05)

Namespaces: `arch.melee.`, `arch.ranged.`, `arch.armor.`, `arch.offhand.`, `arch.ward.`, `arch.mount.`, `ammo.`, `move.`, `counter.` (Canon §19). Only `spec/05_COMBAT_MELEE_RANGED.md` was edited. Mapping: `map_archetypes.json` (35 renames).

## Inventory

Every ID in these namespaces that any chapter uses is either one of doc 05's 122 exported IDs or one of 23 non-canonical spellings. All 23 come from doc 14's Calling kits (§2.4.2, the kit JSON, the L5 tutorial and the doc 05 import row), and doc 11's kit-rebinding table repeats them:

- 5 melee: hafted_mace, hafted_maul, polearm_spear, polearm_staff, fist_wraps.
- 6 ranged: sidearm, longarm, heavy_auto, flamer, hand_crossbow, sling.
- 2 off-hand: shield_small, shield_medium.
- 2 ammunition: fuel, stone.
- 8 class-first armor IDs: light_head, light_body, light_hands, light_legs, light_feet, heavy_head, heavy_body, heavy_legs.

The map adds the 12 other class-first armor spellings (all medium_* and cloth_* forms, plus heavy_hands and heavy_feet) to cover doc 14's "medium set" prose and doc 11's `<class>_<slot>` pattern, for 35 renames in total. Hits for `grid.move.*` (doc 03) and `noise.move.*` (docs 09, 18) contain the substring `move.` but sit in other namespaces, so they are out of scope.

## Changes to doc 05

1. **Exports.** The table now lists every ID in full (`move.light`, not `light`), with counts and the defining section: 15 melee, 20 ranged, 20 armor, 6 off-hand, 4 ward, 3 mount, 12 ammunition, 37 moves and 5 counters, 122 in all. The linter reads only full backticked IDs, so the old short-name list left all 21 action moves "undefined".
2. **§1.8 Move index (new).** Each `move.` ID has one row with its trigger and the section that holds its numbers. The 21 action moves (`move.light`, `move.heavy`, `move.guard` and so on) used to be defined only in prose.
3. **§5.6 Role names (new).** This table resolves the kit role names used by docs 04 and 14 (sidearm, longarm, heavy automatic, flamer, hand crossbow, sling, hafted mace or maul, polearm spear or staff, hand wraps, small or medium shield, class-first armor names, fuel) to exported IDs. Retired IDs are left out of backticks so the map tool does not rewrite this table.
4. **§5.3 thrown weapon.** A one-handed sling is the sling-stone skin of `arch.ranged.thrown`. Its stones are the held stack, not an `ammo.` family. This replaces the pointer to doc 11's rebinding table.
5. **§8.1.** The seven ammunition families and five counter charges are the whole `ammo.` namespace. Fuel is `ammo.cell`, and stones are thrown-weapon stacks.
6. **Interface assumptions, rows 11 and 14.** Kits and templates name only exported IDs. §5.6 replaces two rows of doc 11's rebinding table. Doc 14's kit ammunition must match the weapon's family and stay within the carry cap.
7. **Acceptance test 60 (new).** Namespace closure: these namespaces contain only the 122 exported IDs, and kit ammunition must fit the kit weapon.

## Mapping decisions (old → canonical)

| Old (doc 14 kits, doc 11 table) | Canonical | Reason |
|---|---|---|
| arch.melee.hafted_mace | arch.melee.mace | Same weapon: one-handed hafted, the Vanguard reference weapon |
| arch.melee.hafted_maul | arch.melee.warhammer | Two-handed hafted |
| arch.melee.polearm_spear | arch.melee.spear | Same weapon |
| arch.melee.polearm_staff | arch.melee.quarterstaff | The Monk's melee staff. The casting staff is arch.ranged.staff |
| arch.melee.fist_wraps | arch.melee.fist | The hand-wraps item is the skill.unarmed weapon. Bare hands are arch.melee.unarmed |
| arch.ranged.sidearm | arch.ranged.pistol | The Calling reference sidearm (§5.5) |
| arch.ranged.longarm | arch.ranged.rifle_marksman | The two-handed marksmanship rifle for the Gunslinger alt kit |
| arch.ranged.heavy_auto | arch.ranged.rifle_auto | The Grenadier reference weapon; fires ammo.heavy |
| arch.ranged.flamer | arch.ranged.beam | Doc 11's flamer skins (Fuel-Can Flamer, Boiler Flamer, Flamethrower) are beam skins: fire 10000, ammo.cell |
| arch.ranged.hand_crossbow | arch.ranged.pistol | Overrides doc 11's choice of crossbow. A hand crossbow is one-handed, and arch.ranged.crossbow is two-handed. The kit already supplies ammo.light, the pistol's family, so the Shade alt kit works with no further edit |
| arch.ranged.sling | arch.ranged.thrown | Matches doc 11 and doc 05 §5.3. One-handed, so the Beastbinder alt kit's buckler can still be used (a two-handed weapon empties slot.offhand, Canon §13.2) |
| arch.offhand.shield_small | arch.offhand.shield_buckler | Size match |
| arch.offhand.shield_medium | arch.offhand.shield_heater | Size match |
| ammo.fuel | ammo.cell | The beam's ammunition family |
| ammo.stone | arch.ranged.thrown | Overrides doc 11's choice of ammo.draw. With the sling mapped to the thrown archetype, the stones are the thrown stack itself. Under doc 11's choice, ammo.draw would sit unused in the bag |
| arch.armor.{cloth,light,medium,heavy}_{head,body,hands,legs,feet} (20) | arch.armor.{head,body,hands,legs,feet}_{cloth,light,medium,heavy} | Doc 05 orders slot before class. Doc 14 used light_* and heavy_*, and its prose and line 1540 imply medium_* as well |

No ID needed a new definition: every consumer spelling names content doc 05 already defines.

## Namespace violations reported by the linter (no adoption needed)

- Doc 07 (§3.1 volume table, §6 recipes), doc 10 (§6 worth example) and doc 11 (§3.2 and §3.3 value tables, throwables, the counters table) put doc 05 IDs in the first column of their own data tables. The linter counts that as a definition. Every one of these IDs is defined and exported by doc 05, so nothing needs adopting. Those chapters (or the linter's first-cell rule) must treat them as references, for example by keying rows on the owner's ID in a second column. These violations remain after the map is applied. They are structural, not ID mismatches.
- After the map is applied, doc 11's kit-rebinding table maps the canonical IDs onto themselves, and its shorthand cells (`.hafted_maul` and the like) are not rewritten. Doc 11 should delete the `arch.*` and `ammo.*` rows of that table.

## Follow-ups for other owners (data, not IDs)

- Doc 14 line 1540 (doc 05 import row) uses brace forms `arch.melee.{hafted_mace, ...}`, `arch.ranged.{sidearm, ...}`, `arch.offhand.{shield_small, shield_medium, ...}`, `arch.armor.{light,medium,heavy}_{head,...}` and `ammo.{light, heavy, fuel, stone}`. The map tool cannot rewrite these, so doc 14 must update them by hand. Doc 14 §2.4.2 also says the medium set uses "the same five slots with `medium_`". After the map it should read `_medium`.
- Doc 14 Gunslinger alt kit: the longarm (now arch.ranged.rifle_marksman) fires `ammo.rifle`, so `ammo.heavy` ×30 should become `ammo.rifle` ×30.
- Doc 14 Grenadier alt kit: `ammo.cell` ×600 exceeds the 300-unit carry cap (§8.1). It should be at most 300.
- Doc 14 Alchemist and Beastbinder alt kits: the sling and its 40 stones form one arch.ranged.thrown stack of 41. Doc 11 caps thrown stacks at 20, so doc 14 should set a quantity of at most 20 or split the stack.
- Doc 14's Calling-part values for the changed alt kits should be recomputed from doc 11's values.

## Unresolved

None.
