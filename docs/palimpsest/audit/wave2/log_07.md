# Wave-2 fixer log, chapter 07

Started 2026-10-12. Inputs: requests_07.json (18 items), DECISIONS rows 89-129.

## Progress
- [x] Read requests_07.json, DECISIONS.md
- [x] Read chapter 07 in full; read doc 03 §12.2/§13.2/§13.3/§14.2, doc 09 §13.4, doc 11 §16.4, doc 12 §10.1/§11.2, doc 14 §3.1/§8.9, doc 18 §3.8, canon §4.3/§4.4/§5.8/§13.1/§13.8/§17.7
- [x] Baseline lint: 0 findings (multiply_defined already cleared)

## Triage (request -> status)
- fix:01, verify:01, verify:03 block-trap hit identity -> edit §10.4 (source 0, item_uid, ability, instigator_slot) + test 53
- fix:11, verify:11 salvage `10000 + X` (affix.engineered) -> edit §8.3 + Interface 11 + test 55
- fix:04 field_repair + norm -> already in §10.4 / Interface 04; only "as amended" wording updated to Canon §7.1 `norm_mhp`
- verify:09 rule 4 cite doc 09 §13.4 -> edit §11.1 rule 4 + Interface 09
- verify:18 skins name art.block.* -> new §10.1 Skins paragraph + Interface 18 + Imports
- fix:12 altar block -> already satisfied (`block.altar`); node socket wording aligned with doc 12 §10.1/§11.2 in §9.4
- fix:03 chipping order, refill_mask, layer rows, node explosion, break_result -> edit §9.3, §10.8, Interface 03
- fix:07 torch gap -> resolved (doc 03 §13.3 drops held torch); multiply_defined -> resolved (lint 0)
- fix:07 canon gaps 1-2 -> restated as canon_requests (block.crop removed from list)
- notes Q4 mat.role.block, Q6 sandbag -> product_questions with defaults (informational role; §2.5 value 1000 stated)
- verify:07 crop hp_max 0 -> moot (crops cut by DECISIONS 128)
- verify:07 lockpick yield / staves -> product_question, no change
- DECISIONS 128 seeds cut -> remove §9.7 farming, block.crop, crop state/caps/draw, mat.tag.tillable; tests 25, 33, 51; invariants
- DECISIONS 106 depth -> new §1.5 d_eff/d_current/d_place; §2.7, §4.4, §5.1, §10.4 + test 54
- DECISIONS 101 fused genres -> §1.5 genre paragraph + Canon gap 3 + canon_request §5.8
- DECISIONS 99 weather -> §2.4 snow note + Interface 03; no rule change
- DECISIONS 90, 93, 94, 95, 96, 97, 98, 104, 109-127, 129 -> no doc 07 text affected (checked by grep)
- [x] Batch 1 applied: DECISIONS 128 seeds cut (30 anchored edits; grep confirms only retirement mentions remain)
- [x] Batch 2 applied: §1.5 depth and genre, §2.7, §4.4, §5.1, §10.4 ilvl, Interface 14, tests 53-55, invariant (10 edits)
- [x] Batch 3 applied: requests (salvage X, node explosions/break_result, sockets, sandbag value, skins, trap-hit identity, chipping order/refill_mask, rule 4 doc 09 §13.4, Interface 03/04/09/11/12/16-18, Imports 03/11/12/18, Canon gaps 1 updated and 3-5 added) (27 edits)
- [x] Lint fix: retired IDs (cat.seed, block.crop, mat.tag.tillable) written without backticks so lint reads them as retired names, not undefined IDs; lint 0 findings
- [x] Polish: Doc 18 Imports row moved after Doc 14; §9.3 node sentence split; header date 2026-10-12
- [x] Final lint: 0 findings; hedge-word grep: none
- [x] Checked other chapters for crop/planting text: doc 03 (tick row 17a, save section 4, cause table, Interface 07), doc 08 (§8.4 seeds, seed brew, alchemize row, Interface 07), doc 10 (mundane category list), doc 11 (category tables), doc 21 (WP-032 "farming"), Rust slice `Category::Seed` -> cross_chapter_requests
## Status: complete; structured summary returned
