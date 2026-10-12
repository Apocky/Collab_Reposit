# Wave 2 log, chapter 19 (Data Schemas)

## Round 1, 2026-10-12

### Triage

| Item | Disposition | Where |
|---|---|---|
| fix:02 `x-patchable` on loot-table `lines` | edit | §3.6, §5 row `11.loot_table`, test 14 |
| fix:01 `01.pillar_test` fields | edit | §5 row `01.pillar_test` |
| verify:11 `11.template_loot` ID scheme | edit (scheme `loot.tpl.<template ID, dots as underscores>`, `template` field by x-idref) + cross_chapter_request to 11 + product_question 19-Q1 | §1.2, §5 row `11.template_loot`, test 13 |
| wave2:13 doc 13 template and ability fields and validation rules | edit | §5.1 (new), test 17 |
| conflict: doc 11 loot records duplicate template IDs | already satisfied on disk (doc 11 §2.1 names `11.template_loot`); ID scheme now given | Interface row 11 |
| conflict: doc 16 `content/16/schema/` path | already satisfied on disk (doc 16 test 50 uses `palimpsest/schemas/svc/econ_api.json`); §1.3 now states that subdirectories of `schemas/` hold non-content schemas | §1.3, Interface row 16 |
| conflict: doc 01 bare arrays without envelope | cross_chapter_request to 01 (doc 01 §2.7 and §10.2 still show bare arrays) | Interface row 01 |
| conflict: `block.air` claimed by 03 and 07 | already satisfied on disk (doc 03 Imports take `block.air` from doc 07; doc 07 §10.1 defines it) | Interface row 03 |
| DECISIONS 128 seeds cut | edit (catalog row `08.ingredient`) + canon_request §17.7 retirement sentence | §5 |
| DECISIONS 132 Prologue cut | edit (catalog row `12.room_kind`) + canon_request §17.7 retirement sentence | §5 |
| DECISIONS 141 achievements | edit (row `14.achievement` capped at 40 records; proposed row `14.codex_mark`) + cross_chapter_request to 14 + product_questions 19-Q2, 19-Q3 | §5, test 16 |
| DECISIONS 55, 65, 89 | already satisfied (Interface rows 05, 11; §1.4 retired-ID rule) | §1.4, Interface |
| Canon gaps G1-G11 | canon_request each | Canon gaps |
| G7 locales | canon_request + cross_chapter_request to 18 (doc 18 still has no locale records) | Canon gaps |
| Leftover sweep | hits: "seeds" (08.ingredient row), "prologue rooms" (12.room_kind row); others none | §5 |

### Batches

- Batch 1 (e1.py): header, Imports (7 rows), §1.2 template loot ID scheme, §1.3 schemas subdirectories, §3.6 use on loot-table `lines`, §5 rows 01.pillar_test, 08.ingredient, 11.template_loot, 11.loot_table, 12.room_kind, 14.achievement, new 14.codex_mark, new §5.1 (doc 13 fields and rules), Exports row count 215 -> 216 (counted by script), Interface rows 01, 02, 03, 11, 13, 14 (new), 16, 18. 20 edits. Lint ch19: clean before batch 2.
- Batch 2 (e2.py): Canon gap G12, acceptance tests 13-18, invariants. 3 edits. Lint ch19: 4 undefined_ids (example IDs, retired IDs backticked).
- Batch 3 (e3.py): example template switched to `arch.melee.spear` (defined by doc 05), derived loot ID and retired IDs written without backticks. 5 edits. Lint ch19: clean.
- 19 SCHEMA-FRAGMENT placeholders kept (grep count 19).

### Final lint

`spec_lint.py --chapter 19`: `== 19 words=12575` with no findings; `== IDs defined in more than one chapter: 0`.
Full run: IDs defined in more than one chapter: 0. Findings outside 19 (00 hedges optionally 1; 14 enemy.prologue_hunter; 16 aware.profile.invader, net.msg.scribe_text) come from those chapters' own concurrent edits; doc 19 introduces none.

### Round 1 re-run, 2026-10-12

Re-dispatched with the same task. Chapter 19 already carries every triage disposition above (commit e18d2f2); header names row 145; 19 SCHEMA-FRAGMENT placeholders present; leftover sweep re-grepped with no new hits. 0 edits. Lint --chapter 19: clean (words=12575). Full run: IDs defined in more than one chapter: 0; only finding outside 19 is the chapter 00 baseline hedge. Outbox unchanged.
