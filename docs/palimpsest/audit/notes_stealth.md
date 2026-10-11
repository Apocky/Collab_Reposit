# ID reconciliation: stealth cluster (owner doc 09)

**Namespaces:** `noise.`, `aware.`, `disguise.` and `lock.` (Canon §19).

**Owner chapter:** `spec/09_STEALTH_PERCEPTION.md`.

**Inputs:**
- the output of `tools/spec_lint.py --json`;
- a scan of every chapter for these prefixes, both backticked and unbackticked, which also covers JSON blocks and pseudocode.

## Inventory

**Concrete IDs.** 189 distinct concrete IDs in these namespaces are used across chapters 00, 02, 03, 04, 05, 06, 07, 08, 09, 11, 12, 13, 14, 17 and 18. They break down as follows:
- 6 awareness states;
- 21 perception profiles;
- 6 camouflage families;
- 7 hiding-spot kinds;
- 5 reaction classes;
- 119 `noise.*` IDs (17 of them surfaces);
- 3 disguise tiers;
- 10 disguise triggers;
- 6 lock tiers;
- 6 lock kinds and skins.

**Definitions.** All 189 were already defined in doc 09, either by a definition-table first cell or a JSON `"id"`. All of them are in its Exports.

**Undefined IDs.** No chapter uses a concrete ID in these namespaces that doc 09 leaves undefined. No consumer uses a misspelling or a variant spelling.

**Mapping.** `map_stealth.json` is `{}` because no rename is needed.

## Decisions

1. **Doc 07 §2.4 acoustic table** (linter: 13 `noise.surface.*` "foreign definitions", also flagged as multiply defined in 07 and 09).
   - **What it is.** Doc 07 puts each `noise.surface.*` class in the first column of its material-to-`acoustic` field table. The IDs mean exactly what doc 09 defines, so no rename applies.
   - **The problem.** The material lists disagreed. Doc 09 named things that are not doc 07 materials: dirt, gravel, hay, carpet, "mud blocks", "snow layers" and "moss blocks with grass". It also left out doc 07 materials: tin, zinc, gold, chrome, the extra woods and stones, hide, linen, web and coal.
   - **Adopted into doc 09 §7.2.** The material column now repeats doc 07 §2.4's lists verbatim, covering all 68 non-air materials. A material with no `acoustic` value defaults to dirt.
   - **Real sources for the four classes with no material.**
     - `noise.surface.mud`: a feet cell with ≥ 1 unit of `liq.mud` (doc 08).
     - `noise.surface.water`: any other liquid, the rule as before.
     - `noise.surface.grass`: a `block.tall_grass` feet cell (doc 07).
     - `noise.surface.snow`: reserved until doc 07 adds a snow-layer block, recorded as Canon gap G21.
   - **Why all 17 classes stay.** Doc 18 builds one audio set per (gait, class) and counts 17 classes.
   - **dB values.** Unchanged.
   - **New doc 09 content.** Acceptance test 55, an invariant line, and Interface assumptions rows for 07 and 08.
   - **Remaining linter flag.** It is a layout artifact in doc 07: putting the materials in the first column would remove it. Doc 07 was not edited here.
2. **Doc 13 `aware.alert` "definition"** (linter: foreign definition, and multiply defined in 00, 09 and 13).
   - The flag comes from a Director tension row in doc 13 (around line 1235) whose first cell is prose that mentions `aware.alert`. It is a use, not a definition.
   - The meaning matches doc 09 §6.1 ordinal 5. No mapping.
   - Doc 13 could reword that cell. Not edited here.
3. **Doc 12 `lock.*`** (wildcard) is used for Locked edges, the heist Treasury door, verb seals and the clearance lock. Doc 11 `loot.chest.locked` relies on "doc 09 lock". These consumers needed real lock assignments, so doc 09 §13.1 gained a "Placed locks" table:
   - `Locked(k)` and `Locked(k1, k2)` doors → `lock.kind.door`.
   - Locked chests → `lock.kind.container`.
   - The clearance door → `lock.kind.door` with `lock.skin.electronic`, placed only where local TEC ≥ 5. This keeps doc 09's invariant that an electronic lock needs local TEC ≥ 5.
   - Vault and quest kinds → only where doc 12 names them.
   - Tier is `lock_tier(d)` in every case.
   - The doc 12 `hack` verb seal is a §13.4 device (an Unlock verb at device tier `lock_tier(d)`), not a lock. Doc 12 allows the seal from TEC 3, so treating it as a lock would break the TEC ≥ 5 invariant.
   - Prayer and spell seals are not doc 09 objects.
   - **Two-key doors.** `LockState.key_tag` became `key_tags: [u32; 2]` so two-key `Locked(k1, k2)` doors work.
   - **New doc 09 content.** Acceptance test 56, an invariant line, and Interface assumptions rows for 11 and 12.
4. **Explosion noise sources.**
   - **Consumers checked.** Doc 03's `block.powder_keg`, doc 08 §5.6's nitro detonation, the doc 08 §13.2 bombs, the doc 08 §5.2 liquid blasts, and the doc 05 grenade and launcher table all use existing IDs. Their levels agree with doc 09:
     - `noise.item.shatter_glass` 70 and `noise.item.shatter_ceramic` 75;
     - `noise.explosion.surface` 100;
     - `noise.melee.clang` 70;
     - `noise.gadget.noisemaker` 70 every 60 ticks for 600 ticks;
     - `noise.weapon.jam` 40 and `noise.weapon.dryfire` 30;
     - fizzle 70 and surge 90.
   - **Doc 09 §7.6 completed.** It now names the nitro source for `noise.explosion`, the launcher blast (`.grenade`), and the doc 08 bomb and liquid-blast sources.
   - No mapping.
5. **Awareness states used by consumers.**
   - **Where.** Canon §8, §10.5, §11.1 and test 38; doc 02 music; doc 03's `ev.awareness_changed` payload; doc 04 XP and Pickpocket 100; doc 06 `glyph.cond.unaware`; doc 11 warning bell; doc 12 heist; doc 13 `ai.input.target_aware` values 0/2500/5000/6000/8000/10000; doc 14 `run.note.silent`; doc 17 awareness eye; doc 18 glyphs and barks.
   - **Result.** All of them use the six doc 09 §6.1 IDs with matching meanings and values.
   - **Interim names.** The unbackticked interim names "alerted", "investigating" and "returning" are already mapped in prose by doc 09 G2. They are not backticked, so they are not ID tokens, and no map entry applies.
6. **Patterns and non-IDs.** None of these is a concrete ID that the map tool could rewrite, so none needs a change:
   - wildcards and templates: `aware.*`, `aware.profile.*`, `lock.*`, `noise.*`, `noise.<category>.<name>`, `noise.spell.<school>`, `noise.step.<gait>`, `noise.surface.<class>` and `noise.surface.*`;
   - the bare prefix `noise.surface.` in doc 18;
   - the pseudocode field access `noise.pos` in doc 09 §4.4.

   Doc 17's `noise.<category>.<name>` pattern does not literally match the four two-part IDs (`noise.shot`, `noise.prayer`, `noise.alarm`, `noise.explosion`). Doc 09 G22 says the pattern is read to include them.
7. **Exports.** Exports already listed every ID doc 09 defines (189 of 189). No new IDs were added, so Exports needed no change.
8. **Out of scope, noted only.**
   - Doc 09 §4.2 says hounds "are named explicitly by their templates", but no doc 13 template sets `perception: aware.profile.hound`. This is a doc 13 content gap, not an ID problem.
   - The linter's `talent.*`, `trait.*` and `rng.check` undefined IDs in doc 09 belong to other clusters (doc 04 and Canon).

## Unresolved

None.
