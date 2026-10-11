# ID reconciliation: world cluster (owner doc 12)

Namespaces: `lattice.`, `room.`, `gpv.`, `biome.`, `hazard.`, `secret.`, `law.`, `zone.` (Canon §19). Owner chapter: `spec/12_WORLD_GENERATION_GENRES.md`. Inputs: `tools/spec_lint.py --json`, plus a scan of every chapter for backticked and unbackticked tokens with these prefixes.

## Inventory

- 241 distinct concrete IDs in these namespaces are used across chapters 00, 05, 06, 07, 08, 09, 11, 12, 13, 14, 16 and 18.
- 240 were already defined in doc 12. They are defined by a definition-table first cell, a JSON `"id"`, or the Exports section, and each carries full data at doc 12's level of detail.
- The one ID doc 12 did not define was `room.restricted`, used by doc 09.
- No consumer uses a misspelled or variant form of a doc 12 ID inside these namespaces.

## Decisions

1. **`room.restricted`** (doc 09 §11.3 `disguise.trigger.trespass`, §15 owner table, Canon gaps G19). **Defined in doc 12.** Doc 09 needs a real per-room tag that carries a faction ("a doc 12 room tagged `room.restricted` of F"), and no existing doc 12 ID means the same thing.
   - Territory (§12) is not a synonym. Every room with `pop >= 2` is territory, so if territory counted as restricted, a disguise would set off the trespass trigger almost everywhere.
   - Doc 12 §12 now has a "Restricted rooms" rule. The restricted kinds are armory, barracks, guardpost, key_room, prison, reliquary_vault, server_room and treasury.
   - F is the room's own territory faction. If it has none, F is the territory faction of its lowest-`NodeId` non-Hidden neighbor.
   - The rule makes no draws. Safe, sanctuary and secret rooms are never restricted, and the Unwritten has no restricted rooms.
   - The rule comes with a worked example and acceptance test 37. Doc 12 also gained a sentence after the §16.1 table, the tag in Exports, an I7 interface note and an invariant line.
   - No mapping: the ID keeps its spelling.
2. **`bridge.orphaned`** (doc 12 §20.2 worked example). This was shorthand for `gpv.story.bridge.orphaned`. **Mapped**, and fixed in place in doc 12. Doc 12 was the only place it occurred.
3. **Doc 13 namespace violations**: `gpv.evt.ambush`, `gpv.evt.patrol_shift`, `gpv.evt.reinforcements`, `lattice.challenge.siege`, `law.cyberpunk.corporate_response`, `law.mythic_antiquity.hubris` and `room.arena`.
   - The linter flags these because doc 13 puts them in the first cell of its own tables: the wave-trigger table in §10.4 and the reference-run table.
   - All seven are already defined in doc 12 with the same spelling. Doc 13's values agree with doc 12's: patrol_shift 1000 mSKE, ambush 2000, reinforcements 3000, siege 8 × 1500 every 900 ticks, and Nemesis once per Folio.
   - Nothing needed adopting and no rename applies. The fix belongs in doc 13: move these IDs out of the first column, or prefix that cell with text such as "trigger". Not edited here.
4. **Canon (00) uses** of `law.*` (Canon §5.10), `room.colophon`, `room.no_mount` and `zone.seam` match doc 12 exactly. No action.
5. **Pattern and non-ID tokens**: `room.{incipit, colophon, interleaf}` (doc 14 brace shorthand), `hazard.*`, `law.*`, `gpv.story.*`, `lattice.marg.*` (wildcards in 01, 12, 13, 14 and 18), and namespace mentions such as `secret.` (doc 04). These all refer to existing doc 12 IDs. The map tool cannot rewrite them and they need no change.
6. **Not a world ID**: `hazard.base_bp` in doc 15's invariants annex. It is an unbackticked field of a pity hazard-rate function, not a hazard archetype, and the linter ignores it. No mapping. Doc 15 could rename the field to avoid the prefix collision.
7. **Doc 12's own field accesses** in Rust and prose (`room.feet_cells`, `room.is_safe`, `room.bright_bp` and similar) are code, not IDs. Template IDs `room.tpl.library_a` and `room.tpl.library_a.fill0` are covered by the exported `room.tpl.*`. Biome IDs `biome.<suffix>.<name>` are exported as the 32 tilesets of §3.3. No action on any of these.

## Unresolved

None.

## Outside ID scope (noted, not changed)

Doc 09's §15 request row also asks doc 12 for a biome `camo` value, laser heights, one fuse box per electric room, a room id per cell and the faction recorded on placed devices. None of these is an ID in this cluster. Doc 12's linter-undefined `director.profile.*` and `loot.secret.*` belong to the doc 13 and doc 11 clusters.
