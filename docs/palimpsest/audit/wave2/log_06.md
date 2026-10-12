# Wave 2 log, doc 06

## Round 1, 2026-10-12

Triage (item -> disposition -> where):

| Item | Disposition | Where |
|---|---|---|
| fix:14 Glyph Library learning row | already satisfied | §8.1 table row "Glyph Library picks" |
| fix:07 altar loss only for shrine lamps | already satisfied | §16.1 paragraph after the patron table |
| verify:11 drop "Still requested" from row 11 | already satisfied | Interface row 11 carries no request |
| verify:09 shadow reader is doc 03 §10.6 | edit | §13.1 shadow row, Interface row 09, Exports functions line |
| fix:17 / wave2:17 petition 32 vs 40 | edit: 32 (m25 x8000 applies at Devotion 50: mul_bp(60, 8000) = 48, floor_div(48 x 100, 150) = 32) | §16.4 example, test 22 |
| verify:08 doc 08 signatures | already satisfied | Interface row 08 |
| verify:10 wand draw weights | edit + cross_chapter_request to 11 (step 11c uses wand_weight) | §8.4, Interface row 11 |
| fix:06 G17 daily baseline | already in chapter (§8.1, Canon gaps G17); canon_request restated with exact text | Canon gaps G17 |
| fix:06 doc 05 HitRecord Contradiction potency and Edge | cross_chapter_request to 05 (still missing in doc 05 §3) | Interface row 05 |
| fix:06 doc 03 shadow darkness | already satisfied: doc 03 §10.6 dim(c) | Interface row 03 |
| verify:06 largest block power, tool tier | product_question, default kept (largest) | §13.1 |
| verify:06 unmarked rows 04, 12, 13, 15, 16 | edit rows 04 and 12 (still requested); cross_chapter_requests to 04 and 12 | Interface rows 04, 12 |
| verify:06 intercession mid-Folio resurrection | product_question, default kept | §16.4 |
| wave2:03 homing route | edit + cross_chapter_request to 03 (lock variant, home_bam_s 8160) | §5.4, Interface row 03, tests, invariants |
| wave2:13 patron wording | edit (DECISIONS 103 names boss.author "the Author") + canon_request §16.5 and §20 | §16.1, Canon gaps G18 |
| DECISIONS 33 / G17 | confirmed: chapter text matches | §8.1 |
| DECISIONS 131 last stand | edit: Blood Price never opens the last stand | §10 row 10.2 |
| DECISIONS 132 Prologue | edit: drop "the Prologue" from §8.1; cross_chapter_request to 14 | §8.1 |
| Leftover sweep | only hit was "Prologue" (above); "Epic" is the rarity tier, "Mountain" not a mount | — |

Batch 1: header, DECISIONS paragraph (103, 131, 132), Prologue removal, shadow reader, petition 32, wand weights, homing route, patron lower case, Blood Price last stand, Interface rows 03/04/09/11/12, Exports function line: 17 edits plus 1 repair of a duplicated phrase in row 04. Lint --chapter 06 clean.
Batch 2: Canon gaps G17 exact text and new G18, tests 57 and 58, invariants line, homing constant: 4 edits; then 1 wording fix in test 57 (44 x 136 = 5984 < 6000 <= 45 x 136 = 6120). Lint clean.
Arithmetic: mul_bp(60, 8000) = 48; floor_div(48 x 100, 150) = 32; floor_div(60 x 100, 120) = 50; 136 x 60 = 8160.

Final lint: `spec_lint.py --chapter 06` -> "== 06 words=37765", "IDs defined in more than one chapter: 0", no findings. Full run: only baseline findings (00 hedges optionally 1; 16 undefined aware.profile.invader); IDs defined in more than one chapter: 0.
