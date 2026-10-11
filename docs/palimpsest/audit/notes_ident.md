# ID reconciliation: ident cluster (owner doc 10)

**Namespaces:** `know.`, `idc.`, `appr.`, `buc.` and `curse.` (Canon §19: doc 10, with core values in 00).

**Owner chapter:** `spec/10_IDENTIFICATION_APPRAISAL.md`. I edited no other chapter.

**Mapping:** `map_ident.json` is `{}`. No chapter uses a variant spelling, a synonym or an invented name in these namespaces, so nothing needs renaming.

**Inputs:**
- the output of `tools/spec_lint.py --json`, run before and after the edits;
- a scan of every chapter for these prefixes in backticks, in JSON string values and in brace forms;
- an unbackticked grep for the same prefixes;
- Canon §4.3, §10, §11.6, §13.1, §13.6, §13.7, §17.1, §17.7 and §19, and changelog entries 1.5 to 1.10.

## Inventory (IDs other chapters use)

| Family | Used by | Status |
|---|---|---|
| `know.unknown`, `.classified`, `.appraised`, `.identified` | 00, 01, 04, 06, 08, 11, 12, 13, 14, 15, 17, 18 | Canon core; same spelling everywhere |
| `know.rank.*`, `know.bestiary.*` | 10 only | defined in §12.1 and §12.4 |
| `buc.blessed`, `.uncursed`, `.cursed` | 00, 04, 07, 08, 10, 11, 13, 14, 18 | Canon core. Doc 18's `loc.buc.*.skin.*` entries are text keys, not IDs |
| `idc.` classes | 00, 04, 08, 10, 11 | Canon core |
| `idc.<class>.<name>` | 10 only | defined in §3.3 |
| `appr.use.always`, `appr.use.on_effect` | 06, 08, 10 | defined in §5.1 |
| `appr.eff.identify`, `survey`, `omen`, `unbind`, `consecrate`, `intuit`, `amnesia` | 10, 11 | defined in §5.4 |
| `appr.eff.divine_identify`, `divine_omen`, `aura`, `psychometry` | 06, 10 | linter reported them undefined; fixed (decision 3) |
| `appr.svc.*` | 10, 11 | defined in §5.9 |
| `appr.reason.*` | 03 (`appr.reason.*` in a comment), 10, 14 (`kit`) | linter reported them undefined; fixed (decision 1) |
| `appr.tier.*` | 10 | linter reported them undefined; fixed (decision 2) |
| `curse.*` plain curses | 06, 07, 08, 09, 10, 13 | defined in §7.3 |
| `curse.pact.<name>` | 00 (§13.6, canon-1.9) | was missing; now defined (decision 5) |

After the edits, chapter 10 has no undefined IDs, no foreign-namespace definitions and no retired IDs. No other chapter has an undefined ID in these namespaces. I checked all 5,477 backticked IDs in the spec, the new ones included, and none share an `id32` (fnv1a32).

## Decisions

1. **`appr.reason.*`: defined.** The Exports section listed 21 suffixes, but no table defined them, so the linter treated all 21 as undefined. The doc 03 §14.2 payload and doc 14 (`appr.reason.kit`) need real values.
   - §5 now has a closed table of reasons. Each row gives the reason's ID, the path that emits it, and its `by` value.
   - Three exported reasons were never assigned to any path, and now are: `text` covers the doc-10 text effects and the recharge text, `analysis` covers §5.6, and `craft` covers crafted items and `item.text.inscribed`.
   - **New ID `appr.reason.emendation`.** §7.6 says an Emendation sets `BUC` or `CLEAN`, and that reveal emits an event, but no reason fitted it. The total is now 22. Acceptance test 40 covers the reasons.
2. **`appr.tier.*`: defined in place.** The IDs were in the second column of the §5.5 table. The table is now keyed on the ID, with a new column giving each tier's skill range. Data is unchanged.
3. **The four divination `appr.eff.*` IDs: defined in place.** §5.10 is now keyed on the effect ID, and the glyph moved to the second column.
   - Doc 06 §5.3 agrees on the data: the omen radius `4000 + 2000 × div_tier`, one `rng.ident` draw for psychometry, and the aura on sight.
   - This also removes doc 10's first-column restatement of `glyph.ess.*`, which notes_magic.md item 4 deferred to doc 10.
   - The prose saying doc 06 "does not yet list these essences" is updated: doc 06 now lists them.
4. **Exports rewritten.** Every ID is now listed in full: 11 `know.`, 7 `idc.` classes and 176 descriptors, 5 tiers, 5 use rules, 11 effects, 6 services, 22 reasons, 3 `buc.`, 28 plain curses and 16 pacts. The descriptor lists match §3.3:
   - the counts are 32, 28, 26, 18, 18, 32 and 22;
   - no suffix appears in two classes;
   - `crimson` has sort index 9, `opal` has 18, and `rgb` is 14423100.
5. **`curse.pact.<name>`: 16 defined (new §7.8).** Canon-1.9 §13.6 gives this family to doc 10 and requires at least 16 pacts. Doc 10 still conformed to canon-1.4 and had none.
   - **The 16 pacts:** `bloodthirst` (Canon's example blade, with its exact numbers at full potency), `fury`, `executioner`, `glass`, `overcharge`, `wildfire`, `bulwark`, `thorns`, `aegis`, `titan`, `seer`, `gambler`, `miser`, `hush`, `nightmare` and `transfusion`. Each has exact units and an "Applies to" set.
   - **Pacts per category:** melee 11, ranged 10, armor and off-hand 9 each, ward generator 8, accessory and implant 7 each, wand 2, potion and text 0.
   - **Generation.** `roll_curse` still makes exactly 3 draws, so doc 11's draw counts hold. Draw A below `pact_bp = mul_bp(curse_bp, 2000)` makes the curse a pact, and draw C then picks from `eligible_pacts`.
   - **Cursed capsules.** Source `CursedCapsule` always gives a cursed pact item. It is created sealed with `RARITY` and `OMEN` set.
   - **Pact rules.** Both halves scale with Contradiction potency but not with `curse_penalty`. One affix carries both halves, so they are revealed together and uncursing removes both. No pact draws a random number.
   - **Rules updated to match:** §2.6, §2.8, §7.1, §7.6, §10.3, §11, acceptance test 27, the invariants, and the Interface assumption rows for docs 05, 06, 09, 11 and 18. New tests 41 and 42 cover pacts, and Canon gap 24 records the change.
6. **`curse.brittle`: redefined; the ID is kept.** Canon-1.10 (§4.3) removed item durability, so the old effect, "durability loss × (10000 + m)", no longer exists.
   - The curse now lowers the bearer's resistance to slash, pierce and blunt damage by `m` Bp, with unit 500.
   - Its "Applies to" set is unchanged, so the per-category curse counts in §7.1 stay at 22, 23, 18, 20, 19, 17, 17, 4, 1 and 1.
   - Doc 07's wear formula and its test 18 still cite `curse.brittle`. They go away when doc 07 removes durability, which is doc 07's edit. No rename. Canon gap 23 and test 43 record this.
7. **Other chapters define three of these IDs.** The linter flags each one as multiply defined. All three use the same spelling and agree on the data, so I adopted nothing and added no map entry:
   - doc 11 §12.3 puts `appr.svc.appraise`, `classify`, `identify` and `unbind` in the first column of its services table, with prices of 20, 40, 60 and 150 × `P(d)`;
   - doc 06 has `curse.backfire` in the first cell of a row in its RNG-use table: 1 `rng.ident` draw per zap;
   - doc 04's Exports names `idc.implant`: 27 implant-form augments, matching §3.2.

   These clear once those chapters move the IDs out of the first column, which is their own edit.
8. **Other chapters' IDs in doc 10's first columns: moved.** §6.1 (`arch.`, `alch.`), §7.6 (`alch.tx.`, `prayer.`) and §8 (`alch.potion.`) are now keyed on prose labels. This clears the items deferred to doc 10 in notes_alchemy.md, notes_archetypes.md and notes_magic.md. Canon gap 19 now states the rule.
9. **Retired IDs.**
   - `cat.mount` (canon-1.8) is removed from §2.1, §5.5 and §7.1, so the category count is now 27.
   - The backticked `cat.block` in Canon gap 6 is reworded.
10. **Data aligned with the other owners (not renames).**
    - The base values of the five doc-10 texts now match doc 11 §3.5: 20, 18, 30, 24 and 12.
    - The `idc.text` type count is now 16, including `item.text.inscribed`.
    - The `ev.item_identified` fields `descriptor`, `flags_new` and `reason` are now carried by doc 03 §14.2, so they are no longer a gap.
    - Familiarity uses an Erasure onset of 720 s (canon-1.6).
    - The header is now canon-1.10.
11. **Open conflict, not an ID.** Doc 10 §8 and test 39 set the harmful-text share at 1000 Bp. Doc 11 §3.5 weights `item.text.amnesia` at 3000 of 10000 and cites "the 3000 Bp malign share". I left this unresolved and recorded it as Canon gap 25; doc 10 or doc 11 must change.

## Unresolved

None in these namespaces.
