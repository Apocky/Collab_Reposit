# Wave 2 log, chapter 08

## Round 1, 2026-10-12

### Triage
|Item|Disposition|Where|
|---|---|---|
|fix:05 G-096 NO_CRIT ignored crit draw|already satisfied|§1.3, §7, §10.2, §13.2 firebomb example, tests 29/50, invariant, doc 05 row; Canon gaps none open|
|fix:11 C-26 item.face.respirator; cat./slot. out of Exports|already satisfied|§4.3, doc 11 row, Imports Canon row; no such ID remains|
|fix:04 delete gap 17 envenom|already satisfied|Canon gaps none open; §10.3, test 56|
|fix:08 respirator scope|rejected: chapter conforms to canon-1.11 §12.1 (every gas row blocked)|§4.3, invariant|
|fix:08 'might' hedge|edit: backticked 4 bare `fortify_might` table cells|§8.2|
|verify:08 Margin codex|already satisfied (DECISIONS 34)|§8.5|
|verify:08 hazard drink entry|rejected: canon §10.6 environmental entry stands; text unchanged|§3.2|
|verify:08 reaction products destructible|already satisfied by doc 07 (block table, destructible; doc 07 Canon gap 1)|doc 07|
|wave2:07 seeds (DECISIONS 128)|edit + product_question 08-Q1 (seed brew retired, not re-keyed)|§1.3, §8.4, §9.11, §12.4, §15, §15.1-15.3, Imports, Exports, Interface, tests 30/31/58, invariants|
|DECISIONS 145 hunger only|edit|§3.2 rule, waterskin skin name, doc 04 row, test 31, invariant|
|Leftover sweep|clean except the above (plantations = harvest biome; ing.lotus_seed is an ingredient)|whole chapter|

### Batches
- e1: header, seeds retirement, seq 1 reserved, hedges, thirst rule, exports 316 -> 299, tests 30/31 replaced, invariants; 32 edits; lint 08 clean.
- e2: Imports DECISIONS row, doc 04 interface row; 2 edits; lint 08 clean; full run: duplicates 0, baseline findings in 00 and 16 unchanged, 08 hedges gone.

### Final lint
```
== 08  words=22270
== IDs defined in more than one chapter: 0
```
