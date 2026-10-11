# ID reconciliation: inputs_ui (`ui.`, `input.`, `access.`)

Owner: doc 17 (Canon §19). Run on 2026-10-11 against the spec on disk, including concurrent edits to docs 05, 06 and 09.

## Inventory

- 309 distinct backticked IDs in the three namespaces appear across the spec. Every one is listed in doc 17's Exports, and doc 17 uses all 309 itself.
- Users outside doc 17: 00 (`input.minion_*`), 01 (the §7 verb table: 46 `input.*` IDs, `access.scribes_mercy`, `access.game_speed_bp`), 04 (`input.attack`, `input.hold_breath`), 05 (15 `input.*` IDs), 06 (`input.cast`, `input.cast_cancel`, `input.cast_wheel`, `input.pray`, `input.minion_*`), 09 (`input.interact`, `input.attack`, `input.heavy`, `input.throw`, `input.hold_breath`, `input.whistle`, `input.shout`), 10 (`input.examine`), 12 (`input.interact`), 13 (`input.interact`, `input.hold_breath`, `input.minion_*`), 14 (`access.scribes_mercy`), 18 (`access.font`, `access.gore`, `access.photosensitive`, `access.reduce_motion`, `access.shimmer_bp`, `access.sound_ripples`).
- Every one of these consumer IDs is spelled exactly as doc 17 defines it, and every consumer meaning matches doc 17's definition. No synonym, variant spelling or invented name was found. Range forms (`input.ability_1..=4`, `input.weapon_1..=3`, `input.quickbar_1..=8`, `input.weapon_*`) expand to exported IDs.
- `ai.input.*` (doc 13 §4.3 utility inputs, also cited by doc 09) belongs to the `ai.` namespace (doc 13), not to `input.`. It was left alone.

## Mapping

`map_inputs_ui.json` is `{}`. No rename is needed.

## Namespace violations

- **Doc 05 §1.7 "Inputs consumed"** puts 15 `input.*` IDs in the first column of a table (`input.attack`, `alt`, `reload`, `jump`, `crouch`, `sprint`, `dodge`, `interact`, `heavy`, `throw`, `weapon_next`, `weapon_prev`, `lean_l`, `lean_r`, `mount`), so the linter reads them as definitions. Their bit numbers (0–8, 10–14, 21, 22, 25) and names match doc 17 §12.1 exactly, so there is nothing to adopt and nothing to map. The flag clears only when doc 05 changes that table, for example by making the bit number the first column. Doc 17 cannot fix this.

## Changes to doc 17

Every exported ID already had content. These edits give the IDs that had only a name, or only prose, exact data in table rows or tagged sections:

1. **`input.ctx.*` (10).** §12.1's prose list became a table with each context's rank, activation condition and the bindings it changes. The prose's specificity order left out `margin`; it is now rank 9, between `mounted` and `on_foot`. The Margin is never in a run, so it only competes with `menu` and `wheel`. Its one binding change is the existing §11.1 rule that `input.menu_book` opens `ui.menu.margin_index`.
2. **`input.cfg.*` (12).** A new §12.10 table lists values, defaults and rules for each setting. All values come from §12.3, §12.4 and §12.6. Two defaults were implicit and are now stated: `aim_assist_gyro` off and `invert_y` off.
3. **`access.*` (12 settings without tabulated values).** A new §13.11 covers `colorblind_mode`, `world_filter`, `ui_scale_bp`, `text_scale_bp`, `font`, `high_contrast`, `consistent_skin`, `lean_toggle`, `narrator`, `sound_viz`, `photosensitive` and `game_speed_bp`. Stated defaults were copied over. Implicit ones were made explicit: `none`, off, and 10000 for both scales, as §2.4's worked examples and §12.5's Deck override of 11000 imply. All 50 `access.*` IDs now have a table row or JSON record.
4. **`ui.tip.*` (15).** Before this change only `ui.tip.contradiction` had a record. A §15 table now gives each tip a trigger and glyphs. Every trigger event exists in Canon §18 or the doc 03 bus: `ev.contradiction_changed`, `ev.erasure_stack`, `ev.colophon_entered`, `ev.ward_broken`, `ev.item_picked_up`, `ev.minion_bound`, `ev.player_ghosted`, `ev.affliction_changed`, `ev.favor_changed`, `ev.awareness_changed`. Three triggers are client conditions: `ui.icon.edge` first drawn, the stealth meter first shown, and an unknown item type picked up. `ui.tip.scribes_mercy` keeps its existing rule (first death) and text. §10.6 used to say the affliction tip shows "the first time per run", which contradicted §15's one show per account. It now names `ui.tip.affliction` and drops "per run".
5. **`ui.menu.character`, `ui.menu.boon_altar`, `ui.menu.margin_index`** appeared only in Exports. The ID is now added to the sections that already describe them: §5.1 Character sheet (the Book's Character tab), §9.5 Boon altar and §11.1 Margin index.
6. **Acceptance tests 54–56** cover context precedence, fresh-profile defaults and one-show tips.
7. **Not ID changes, but consumer corrections whose data belongs to another chapter.** §6.1 now uses doc 07's 3 × 3 grid at `station.hand` instead of 2 × 2, as doc 07's Interface assumptions row for doc 17 requests. The §6.4 composer example now shows W1 burning power 64 instead of 54, with the Evocation 25 hook, matching doc 06 §6.7 and doc 06's Interface assumptions row for doc 17.

The Exports section was already complete: 309 IDs, and every group count matches its list. These edits add no new IDs, so it is unchanged. `spec_lint.py` reports doc 17 clean: no undefined IDs, no foreign definitions, no JSON issues, no hedges.

## Unresolved

None among IDs.

## Follow-ups outside this cluster's ID scope

- Doc 09 §13.4 and §13.7 call for "doc 17's hack panel" (`Hack { target, verb }`) and "doc 17's pickpocket panel" (`Steal`, `Plant`). Doc 17 defines neither and gives neither an ID. Both need a HUD panel design (placement, input context, list contents) that goes beyond ID reconciliation. Suggested IDs when designed: `ui.hud.hack`, `ui.hud.pickpocket`.
- `ui.hud.lockpick`, `ui.hud.placement` and the wheels have content but no §3.2-style rectangle, because they are contextual overlays. `ui.hud.lockpick` has no stated geometry at all.
- Doc 01 G7 and its §7 row for `input.cast_cancel` still say the gamepad cast cancel is "requested". Doc 17 §12.4 now binds it: B in `input.ctx.casting`.
- Doc 14 §7.4 and §10 still block Grades and † achievements under `access.scribes_mercy`. Doc 17 G4 asks doc 14 to drop both.
