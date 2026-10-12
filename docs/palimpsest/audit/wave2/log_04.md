# Wave 2 log, chapter 04

## Round 1, 2026-10-12

Triage (item -> disposition -> where):

| # | Item | Disposition | Where |
|---|---|---|---|
| 1 | fix:14 non-humanoid Echoes, fenrir/sun_wukong/loki budget rows, ghost-XP sentence | already satisfied | §13, §2.6.1, test 37, Interface 14 |
| 2 | fix:07 self_repair supplies, durability perks removed | already satisfied (field_repair now acts on placeable block HP, which canon keeps) | §2.4, §5.3, §7.2, Interface 07 |
| 3 | fix:11 self_repair never spends a tool use | already satisfied | §2.4 |
| 4 | fix:09 Security 25 slip chance, Interface 09 | already satisfied | §5.3, Interface 09 |
| 4b | fix:09 / verify:09 pacify on first subdue | edit | §6.1 Pacify row |
| 5 | fix:18 pres_hash phantoms | already satisfied | §12.4 |
| 6 | fix:18 / verify:17 preset IDs and colors (DECISIONS 97) | already satisfied by doc 18 §5.6 (`art.` is doc 18's namespace, Canon §19); edit cites the IDs | §2.1 |
| 7 | verify:07 recipe_tier 0..=5, process recipes, mineral or salvage node | edit | §5.2, §5.3 Fieldcraft, §6.1, Interface 07 |
| 8 | verify:09 Security 25 | already satisfied | §5.3 |
| 9 | verify:18 Appearance record, kept by Going Native | edit | §2.1, §2.8 |
| 10 | fix:17 phantom marking flag | already satisfied | §12.4 |
| 11 | fix:08 / verify:08 Imports doc 08 §4.5 -> §4.3 | edit | Imports |
| 12 | verify:10 Appraisal 25 reword | edit | §5.3 |
| 12b | verify:10 curse and pact terms in the modifier model | edit | §1.2, Interface 10 |
| 13 | fix:04 canon gap 1 (leg loss, >2 legs) | canon_request (restated) | Canon gaps 1 |
| 14 | fix:04 hedge `attr.might` | already satisfied (lint clean) | lint |
| 15 | fix:04 kit value spread, no pick or torches | product_question, default: kits unchanged | 04-Q2 |
| 16 | verify:04 ally-targeting verbs per kit | product_question, default: ally verbs come from talents, kits unchanged | 04-Q3 |
| 17 | verify:04 / wave2:14 Shade "5 lockpicks" -> "lockpicks" | edit | §2.5 |
| 18 | verify:04 DECISIONS 106 depth 25 + 2n | edit + canon_request §13.8 | §6.7, Canon gaps 3 |
| 19 | wave2:03 CLIMB_ANY, sprint-slide stamina, grapple peak_y | edit | §13, §11.5 |
| 20 | wave2:14 ErratumChoice::Custom output | edit | §2.1 |
| 21 | doc 14 Monk role "light armor set" | already satisfied | §2.5 |
| 22 | DECISIONS 131, 134 last stand | edit + canon_request §10.8 + cross 05, 17; product_question 04-Q1 | §12.1a |
| 23 | DECISIONS 145 hunger only | edit | §11 |
| 24 | DECISIONS 97 presets | edit (with item 6/9) | §2.1 |
| 25 | DECISIONS 102 Second Edition carry-over | already satisfied | §6.7 |
| 26 | Leftover sweep | no hits (grep of every listed term; `run_seed` is the RNG seed) | whole chapter |

Batch 1 (edit1.py): header, Imports (doc 08 §4.3, docs 03/10/14/17/18 sections, DECISIONS rows list), §1.2 curse and pact terms, §2.1 Appearance and ErratumChoice::Custom, §2.5 Shade lockpicks, §2.8 Appearance kept, §5.2 recipe_tier 0..=5 and mineral or salvage node, §5.3 Appraisal 25 and Fieldcraft 75, §6.1 first subdue and tier 0 tenths, §6.7 depth 25 + 2n, §11 hunger only, §11.5 sprint-slide and grapple, §12.1a last stand, §13 CLIMB_ANY, Exports LAST_STAND_TICKS, Interface rows 03/05/07/10/14/17/18, Canon gaps 2 and 3, tests 57-63, invariants. 36 edits, all anchors asserted unique.
Arithmetic: tier-3 recipe, 4 units: 1000 x 4 + 2000 x 3 = 10,000 milli-XP, 3 x 3 = 9 tenths; tier 0, 4 units: 4,000 and 0 tenths. Second Edition depth 25 + 2 x 1 = 27, 25 + 2 x 3 = 31. curse.thirsting m 2500: 10000 - 2500 = 7500.
Lint --chapter 04: clean (words=33784). Full run: "IDs defined in more than one chapter: 0"; remaining findings are in other chapters (00 hedge 'optionally'; enemy.prologue_hunter and aware.profile.invader, net.msg.scribe_text in chapters edited concurrently by other lanes), none introduced by doc 04.

## Round 1, 2026-10-12 (re-run verification)

Triage: every requests_04.json item, the three inbound requests (wave2:03 CLIMB_ANY/sprint-slide/grapple, wave2:14 Shade lockpicks, wave2:14 ErratumChoice::Custom), doc 14's Monk light armor set and DECISIONS 97, 131, 134, 145 -> already satisfied on disk by the batch above (commit e18d2f2): §2.1 creation output and Cosmetics, §2.5 shade kit "lockpicks", §11 hunger only, §11.5 movement predicates, §12.1a last stand (300 ticks, solo, once per Folio, Death's Door first), §13 CLIMB_ANY, tests 57-63, invariants. Header already reads As of 2026-10-12, canon-1.11, through row 145.
Leftover sweep: hits only on retained allowed terms (placeable repair, field_repair for placeables, the retirement scope note, no-thirst/no-fatigue rule, no-slider rule). No edits.
Batch 2: 0 edits. Lint --chapter 04: clean (words=33784). Full run: "IDs defined in more than one chapter: 0"; only baseline finding remains (00 hedges {'optionally': 1}).
