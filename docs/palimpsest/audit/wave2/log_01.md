# Wave 2 log, chapter 01 (Vision, Pillars and Inspiration Ledger)

## Round 1, 2026-10-12

### Triage

| # | Item | Disposition | Where |
|---|---|---|---|
| 1 | fix:14 Erasure timing 720 s, one hunter | already satisfied | §6.3 table and paragraph; test 15 |
| 2 | fix:05 drop mount row | already satisfied | §7 (no row; retirement sentence after the table) |
| 3 | fix:02 §9.2 Windows only | already satisfied | §9.2; Q-11 answered |
| 4 | verify:14 Bindery numbers, G2 offer at every Ur-Text victory | already satisfied | §6.5 (1,960 / 5,710 / 95.2 h / 39.9 h / 8.0-19.0 weeks); §6.4; G2 |
| 5 | verify:02 Epic and EOS | already satisfied in substance; residual mentions removed in the sweep | §9.2, PAR-03, Q-11, Imports |
| 6 | verify:16 drop Epic; drop ChooseExit from the doc 16 import | edit | Imports row doc 16; §7 vote row now cites doc 14 §3.6 and doc 16 §9.8 |
| 7 | fix:15 base price 19.99 | already satisfied | §9.3 |
| 8 | fix:18 the Censor and the Censor's office | already satisfied | §1.3, Q-17 |
| 9 | verify:18 DECISIONS 114 all 12 genres at EA | already satisfied | §2.7, PAR-12, tests 22, 24, 25 |
| 10 | verify:15 Folio Expansion 9.99 | already satisfied; Q-12 question text reworded in the sweep | §9.3, Q-12, peer table |
| 11 | fix:17 drop input.mount, delete G7 | already satisfied | §7; Canon gaps preamble |
| 12 | fix:01 movement kit owner text | edit | doc 03 §5.9 (2,000 mm), §5.23, §5.24 and doc 17 §12.1 now present; §7 rows, Interface, G1 updated; doc 11 template still missing (cross-chapter 11); canon text restated as canon_request §4.2 |
| 13 | fix:01 Second Edition and traces owner text | edit | G2 updated to doc 14 §8.9 (offer, fused reporting, Ironman, forfeit); doc 12/13/16 still open |
| 14 | fix:01 doc 02 §17.4 per-Folio aggregates | already satisfied | Canon gaps preamble (C-14 on disk) |
| 15 | fix:01 block-trap identification | edit (with inbound 07 and 03) | §10.3 systemic-kill test; Interface rows 03, 07 marked present |
| 16 | verify:01 doc 14 §2.3 and doc 21 mismatch | doc 14 now satisfied on disk (§2.3 line "Early Access scope", §8.9 at EA); doc 21 still stale | Interface rows 14, 21; cross-chapter 21 |
| 17 | verify:01 doc 21 invasions and dates | cross_chapter_request | doc 21 (generated) |
| 18 | verify:01 Q-25 slide input in doc 17 tables | already satisfied on disk (doc 17 §12.1 sprint-slide entry, §12.3, §12.4 R3) | Q-25 default now cites doc 17 §12.1; Interface row 17 marked present |
| 19 | verify:01 ally verbs per Calling (Interface 04) | cross_chapter_request to 04 | Interface row 04 stays open |
| 20 | inbound wave2:07 trap payload test | edit | §10.3 Systemic kills |
| 21 | inbound wave2:03 mantle 2,000 mm, slide, grapple, trap payload | edit | §7 rows, Interface row 03, G1 |
| 22 | inbound wave2:14 G2, Interface 12 and 13, pq_se_offer | edit + product_question 01-Q1 | G2, Interface rows 12, 13 |
| 23 | doc 19 §7: bare JSON arrays | edit | §2.7 and §10.2 JSON wrapped in the doc 19 §1.2 envelope; test 23 |
| 24 | DECISIONS 126-145 coverage | edit | header, Imports; PAR-04 (144), PAR-06 (142), PAR-10 (126), Barony (145), Megabonk (136), §6.5 arcs (130), §10.1 telemetry (140), fair_page.t3 and KPI first_editor (132) |
| 25 | Leftover sweep | edit | Prologue (4 hits), Epic/EOS (4), farms, plants, 12.99 in Q-12; 24.99 and 29.99 in the peer table are third-party R3 prices of Slay the Spire 2 and Hades II, not ours, and stay |

### Batches

- Batch 1 (e1.py): header date and row-145 coverage; Imports (DECISIONS 126-145, docs 03, 07, 14, 16, 17, 19, 21); ChooseExit dropped from the doc 16 import; fair_page.t3 and kpi.run.first_editor_5runs without the Prologue; Barony hunger (145); SM-10 places lanterns; Megabonk tips (136); PAR-04 (144), PAR-06 (142), PAR-10 (126); NR-25 wording; §6.5 resident arcs (127, 130); §7 jump, slide, quickbar and vote rows; §9.2 Epic/EOS wording; §10.1 telemetry (140); §10.3 trap payload test; Q-12 and Q-25 text; Canon gaps preamble. 37 edits, all anchors asserted unique. Lint --chapter 01: clean.
- Batch 2 (e2.py): pillars.json, tests.json and kpis.json examples wrapped in the doc 19 §1.2 envelope (01.pillar, 01.pillar_test, 01.kpi) plus the envelope sentence; Interface assumptions rewritten one row per chapter with a Status column; G1 and G2 rewritten (G1 carries the requested Canon §4.2 text; G2 cites doc 14 §8.9 and product question 01-Q1); test 23 amended; tests 26-28 added; 4 invariant lines. 9 edits. No numbers changed, so no worked example needed recomputation.

### Final lint

- `spec_lint.py --chapter 01`: `== 01 words=20913`, no findings; `IDs defined in more than one chapter: 0`.
- Full run: chapter 01 clean; remaining findings belong to other lanes' in-flight edits (00 hedges optionally 1 baseline; 14 undefined enemy.prologue_hunter; 16 undefined aware.profile.invader (baseline) and net.msg.scribe_text); none references chapter 01 text. `IDs defined in more than one chapter: 0`.
