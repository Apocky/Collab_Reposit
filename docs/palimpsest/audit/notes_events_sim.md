# ID reconciliation: events_sim cluster (`ev.`, `comp.`, `sys.`, `grid.`)

Owner chapter: `spec/03_SIMULATION_CORE.md` (Canon §19: `comp.`, `sys.`, `grid.` belong to 03; `ev.` core names belong to 00, the bus and payload schemas to 03). Only chapter 03 was edited. Mapping: `map_events_sim.json` (3 renames).

Canon-1.8 was committed while this pass ran. It retired mounts (Canon §17.7) and dropped `ev.mounted` and `ev.dismounted` from §18, leaving 42 canon events. This pass applies that retirement to chapter 03.

## Inventory

- 231 distinct backticked IDs in the four namespaces are used across chapters 00 to 18.
- 228 were already defined in chapter 03, in §2.2 for systems, §3.2 and §3.3 for kinds and components, §4.4, §6.2, §13.1, §13.7 and Exports for the `grid.` enums, and §14.2 and §14.3 for event payloads.
- 3 were undefined: `comp.pools.aether` (06), `ev.entity_killed.damage_seq` (01) and `grid.cause` (13).
- Matches in the loose scan that are not IDs:
  - `ev.item_picked_up.from = 3` in 16 and `ev.noise.heard` in 03 are field paths inside longer code spans.
  - `grid.cause.*` and the other wildcards are family patterns.
  - `grid.trimmed()` (07) and `sys.due` / `sys.run` (02) are Rust method calls.
  - None of these needs an action.

## Mappings (consumer spelling → canonical ID)

| Old | Canonical | Used in | Why |
|---|---|---|---|
| `comp.pools.aether` | `comp.pools` | 06 Imports | A field path, not an ID. The `aether` and `aether_max` fields belong to `comp.pools` (03 §3.3). |
| `ev.entity_killed.damage_seq` | `ev.entity_killed` | 01 §1 KPI table, 01 Interface assumptions | A field path. `damage_seq` is a payload field of `ev.entity_killed`; its meaning is now defined in 03 §14.2 (see below). |
| `ev.spell_fizzled` | `ev.spell_cast` | 03 only (now removed) | Withdrawn synonym. Doc 06 §19.1 and G3 report every fizzle as `ev.spell_cast` with `fizzled = true`. Doc 18 already plays fizzles from `ev.spell_cast`. |

## Definitions added to or completed in chapter 03

1. **`grid.` enum family names** (Exports). `grid.shape`, `grid.liqi`, `grid.rt`, `grid.ekind`, `grid.move`, `grid.cause`, `grid.despawn` and `grid.gait` are now valid names for a whole enum. They are never stored values and never loaded as content. This resolves the "`grid.cause` list" reference in doc 13 Canon gap 17. That gap's claim that `erased` is missing is stale: `grid.cause.erased` exists in 03 §13.1.
2. **`grid.despawn.*`** had IDs in Exports only. §3.1 now has a table that gives each of the 9 reasons its producer.
3. **`grid.gait.*`** stride lengths now cover all four gaits. Crawl is 500 mm in `downed` (doc 09 G16), so footsteps also accumulate while downed.
4. **`grid.cause.*` producers:**
   - `toggled` now covers doc 07 §10.6 light-twin swaps (doc 09 §2.4 and doc 13 `ai.action.relight`).
   - `grew` covers doc 07 §9.7 crops.
   - `froze` covers doc 08 §5.2's frost rows for lava and molten metal.
   - Fixed terrain (§4.7, §13.1 rule 4, G14, test 36, invariants) now allows a `grid.cause.toggled` swap between lit and unlit twins. Without this, a non-destructible worldgen fixture could not be snuffed, which contradicted the cause's own producers.
5. **`comp.player`** gains `breath_held: bool`, as doc 17 asks of 03. `SetBreathHold` writes it in `sys.input_apply`, and doc 04's breath step clears it when breath reaches 0.
6. **`ev.damage_dealt`** (producer doc 05 §10.2 step 9):
   - Appended fields: `kind`, `base_mhp`, `stamina_loss_mst`, `d_armor` and `counters`, each with its meaning.
   - New `DmgFlags` bits: 13 `takedown` (doc 09), 14 `parried` and 15 `negated` (doc 05), 16 `self_cost` (doc 06 Blood Price). Bits 17 to 31 are reserved.
   - Hits that 03 queues itself are `kind` 3 with `counters` 0.
   - The JSON log example was updated to show the new fields.
7. **`ev.entity_killed.damage_seq`** (doc 01 G4): it is the `seq` of the lethal hit's `ev.damage_dealt`, which is emitted first in the same tick. A bleed-out death (doc 16 §9.4) carries `0xFFFF_FFFF`. The emission order is now a bus rule, with an invariant and test 51.
8. **`ev.spell_cast`** gains `spell_hash: u64` and `channel_ticks: Ticks` (doc 06 §19.1 and the doc 06 state table). `spell = 0` for compiled spells.
9. **`ev.item_picked_up.from`** gains code `3 trade`, and `cause = 1` exactly when `from = 3` (doc 16 G10).
10. **§14.3, registered events:**
    - The seven doc 06 rows (`ev.cast_started`, `ev.cast_interrupted`, `ev.wild_surge`, `ev.glyph_learned`, `ev.enchant_applied`, `ev.prayer_resolved`, `ev.favor_changed`) are replaced with doc 06 §19.1's producer payloads, verbatim. Their only consumer is doc 06, so no other chapter breaks.
    - `ev.entity_pacified` gains method 4 `subdued` (doc 09 §9.4), and code 1 is renamed to dialogue, as doc 13's list of pacification routes requires.
    - `ev.secret_found.kind` now points at doc 12 §17.3, indices 0 to 10. It previously cited §16, which is a stale section number.
    - The §14.3 count is now 28 (G4).
11. **`HeardNoise.req: u32`** (doc 09 Interface assumptions). This is not an ID, but `sys.hearing` needs it to read `ev.noise` sources.
12. **Exports** now has a row listing the 42 Canon §18 events: the names are canon's, and the payloads are defined in 03 §14.2. `ev.spell_fizzled` was dropped from the exports, and the mount IDs below were removed.
13. **Interface assumptions** for docs 05, 06, 09, 13, 16 and 17 now state the producer duties above. G28, tests 51 and 52, and the invariants record the reconciliation.

## Canon-1.8: retired mount IDs in chapter 03's namespaces

Removed from 03 because the game has no mounts. Chapter 03 was the only user of each.

- `comp.mount`, `comp.rider`
- `grid.move.ridden`, `grid.move.hover`, `grid.move.glide`. The `MoveMode` variants, the `select_mode` branches, `MoverParams.mount_mode`, mount slaving in `sys.movement`, and the §5.12 mount rules were removed with them.
- `grid.ekind.mount`. Kind code 2 is kept as a reserved row, so the replicated kind codes in doc 16 §6.8 are not renumbered.
- The `ev.mounted` and `ev.dismounted` payload rows. The canon retired both IDs.
- `st.mounted` was removed from §13.3 rule 5, and mounts were removed from the sleep and actor-cap rules (§15.2, §15.3), from `grid.despawn.dismissed`, and from Interface assumptions row 05.

## Unresolved

- `ev.mounted` (used in 05, 09, 16) and `ev.dismounted` (used in 05, 12, 16): retired by canon-1.8 §17.7. They have no successor ID, so nothing can map to them. The consumer chapters have to delete those references, along with `arch.mount.*`, `st.mounted` and the mount mover assumptions in doc 05 Interface assumptions.

## Other chapters' "definitions" in these namespaces (cannot edit; no rename needed)

The lint reports these because the IDs appear in another chapter's Exports section or in a first table cell. Every spelling already matches 03, so no mapping is needed.

- Doc 05 Exports names `sys.damage` and `sys.damage_late`, as the systems that call `resolve_hit`.
- Doc 13 Exports names `comp.tags` for its bit layout. Its row 259 is a table header that reads "Stage inside `sys.ai_think`".
- Doc 08 Exports names `ev.potion_brewed` and `ev.liquid_reaction` and defers to 03's payloads.
- Doc 12 Exports names `ev.secret_found` and `ev.objective_completed` ("registered by doc 03").
- Doc 16 Exports names `ev.player_*`, also as defined in 03 §14.2.
- Doc 06 §19.1 table now matches 03 verbatim.
- Doc 09 §7.9 noise-conversion table and doc 13's Director table are keyed by event.

## Discrepancies noted for other owners (no ID action here)

- Doc 06 §2.1 and its Interface assumptions say `aether_tick` runs first in `sys.actions`. Chapter 03 §2.2 runs it in `sys.resources` (26a), and 03 owns the schedule. Doc 06 should conform.
- Chapter 03 still uses `block.tag.destructible` and `block.tag.errata_placeable`, which doc 07 does not define. These are `block.` IDs and belong to doc 07's reconciler.
- Doc 07 Interface assumptions ask for block-schema fields: `break_result`, `hide`, `conceal_crouch_only` and `lightsrc`. Doc 06 asks for darkness volumes. Doc 17 asks for "step 6c". None of these is an ID in this cluster. They are left for an interface pass.
- Chapter 03's header still says it conforms to canon-1.4. Only the 1.8 mount cut was applied in this pass.
