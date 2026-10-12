# Wave-2 fixer log — chapter 15

Inputs: audit/wave2/requests_15.json (17 items), DECISIONS rows 89-129 landing in doc 15 or its systems.
Note: DECISIONS rows 130-137 are present in the working tree but lie outside this wave's binding range (1-129); not applied (see unresolved).

## Progress

## Round 1, 2026-10-12

Full pass. Inputs: requests_15.json (17 items), inbound wave2:13 and wave2:14, DECISIONS rows 89-145.

### Triage

| Item | Disposition | Where |
|---|---|---|
| fix:14 fields consumed | already satisfied (Imports doc 14 row cites them; Interface row for 14 lists only §2.3) | Imports, Interface assumptions |
| fix:14 Item Codex -> Index | already satisfied | §6.2 `pass.xp.codex_entry` |
| fix:14 base price 24.99 | already satisfied (19.99) | §1.1, §7 |
| fix:16 / fix:17 voice with non-friends muted | already satisfied | §8.5 Adult defaults, test 39 |
| verify:14 59 achievements, Ironman cosmetic | edit: totals restated for DECISIONS 141; Ironman already in §2.2 | §2.2, §2.4 |
| verify:02 palimpsest-verify crate | edit Interface row 02 | Interface assumptions |
| verify:02 MIT licence vs DECISIONS 44 | edit + product question 15-Q1 (default: source published, proprietary licence) | §4.3 |
| verify:16 idem s = req_id hex | edit | §10.3, test 36 |
| verify:16 pity receipt folio_exits, transcription | edit | §4.6 |
| verify:16 drop Epic pricing | already satisfied (no Epic text) | §7 |
| verify:18 DECISIONS 114 | already satisfied | §3.2.7, test 37 |
| fix:12 expansions + doc 12 §25 pointer | edit (pointer added; names already present) | §1.1 |
| fix:10 C-29 Index | already satisfied | §5.5, §6.2 |
| verify:06 rarity bands for glyphs | edit | §3.1.2 Rubricated row |
| fix:15 Transcription shelf | product question 15-Q2 (default: shelf kept as written) | §3.9.1 |
| fix:15 doc 14 vs 21 Echo scope | superseded by DECISIONS 114/119; Interface row 14 kept | Interface assumptions |
| fix:15 19.99 price-matrix row | product question 15-Q3 (default: authored row kept) | §7 |
| fix:15 / verify:15 Second Edition rows 100-108 | already satisfied | §2.2 Second Edition |
| verify:15 Ironman frame kind | product question 15-Q4 (default `cosm.frame.ironman`) | §2.2 |
| verify:15 rows 109-125 sync | already satisfied in doc 15; Interface rows 14, 21 stay open | Interface assumptions |
| wave2:13 six Second Edition bosses pay Editor kill, final Folio two | edit (confirmation sentence) | §2.2 Second Edition, test 43 |
| wave2:14 fused genre from FolioRecord.genre; Ironman to victory_present; two kills | edit | §5.1, §2.2 |
| DECISIONS 132 no Prologue | edit: mode row 4 retired, formula, §2.4, §5.1, tests 12, 36 | §2.2, §2.4, §5.1 |
| DECISIONS 137 first-run achievement | edit | §2.2, test 12 |
| DECISIONS 141 ~40 Steam achievements, Codex marks | edit + product question 15-Q5 (default: marks pay the grade table) | §2.2, §2.4, §6.2, §10.3 |
| DECISIONS 111/112/113/122 | already satisfied | §1.1, §3.2.3, Interface 21 |
| Sweep: EOS in invariant (end of service) | edit to plain words | Invariants annex |
| Sweep: other terms | no hits (playtest is a private build channel per doc 02 §17) | - |

Batch 1: header, Imports, §1.1 doc 12 §25 pointer, §2.2 achievement row and mode table (Prologue row retired, rows renumbered 1-5), §2.4, §5.1, §3.1.2 glyph row, §4.3 licence, §4.6 pity fields, §10.3 idem table, §6.2 achievement XP, invariant wording. 24 edits. Lint --chapter 15 clean.
Batch 2: §2.2 Second Edition Editors and fused genre, §5.1 FolioRecord.genre, tests 12 and 36 rewritten, tests 43-45 added, 3 invariants. 8 edits. Lint clean.
Batch 3: Interface assumptions rows for docs 14 and 13. 1 edit.
Final lint: `--chapter 15`: "== 15 words=28260", "IDs defined in more than one chapter: 0", no findings. Full run: only baseline findings (00 hedges optionally 1; 16 undefined aware.profile.invader); 08 hedges no longer reported; IDs defined in more than one chapter: 0.
