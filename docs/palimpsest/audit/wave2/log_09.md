# Wave-2 fixer log, chapter 09

Started 2026-10-12. Inputs: requests_09.json (18 items), DECISIONS rows 89-129.

## Progress
- [x] Read chapter 09, requests_09.json, DECISIONS.md, canon §2.3, §4.4, §10.6
- [x] Baseline lint: 0 findings
- [x] Triage of requests (status before edits):
  - verify:14 run.note.silent -> !folio_alerted: TODO (run_alerted has no consumer left; retire it)
  - verify:16 steps 30-tick lifetime: TODO
  - fix:07 Imports doc 07 wording: TODO (drop "with no durability"); G21 already closed (snow kept, see weather)
  - fix:04 consume a lockpick unit: NOT APPLIED (contradicts DECISIONS 65, C-31 and doc 04's current Security 25 text, which uses slip_bp); G9 rng.check sentence already gone
  - verify:07 deck spark: already satisfied (§13.4)
  - fix:17 noise.mount rows, break roll, seq 1005: already satisfied; SetLockMode + panel exposure: TODO
  - fix:12 doc 12 §11.3 citation: already satisfied
  - fix:08 / verify:08 doc 08 §4.3: TODO
  - fix:03 shadow term: already satisfied; invariant wording: TODO (env)
  - G23, G24, G25: canon_requests (repeat); noise.mount §17.7 retirement: canon_requests (repeat)
  - DECISIONS 99 weather: TODO (§15.3 consumption + canon proposal G26)
  - invader filter vs doc 16: TODO (default: doc 16 §6.6 rule 2 governs; product question)
  - doc 13 outfit coverage: cross-chapter request (doc 13 has no per-template outfit flags yet)
  - DECISIONS 93/94/95/96/101/106: TODO short rules
- [x] Batch 1 applied: header, §1.4 (run_alerted retired, folio_alerted readers, host migration DECISIONS 93), T3, §11.1 doc 08 §4.3, Imports doc 07, §14 rule 2, Interface row 14, stealth_light invariant
- [x] Batch 2 applied: §1.1 Second Edition depth, slide/grapple stance+motion+noises (DECISIONS 95), camo first-match, weather hooks wx_* in §3.2/§4.4/§4.5/§7.2, snow override, armor_db scope, disguise triggers, Exports
  (note: scratchpad root is shared with other fixers; my scripts live in scratchpad/w2_09/)
- [x] Batch 3 applied: StealthHud struct, ring/steps 30-tick lifetime, new §8.4 action panels (StealthPanel), §10.4 ink gore note, §13.2 start/cancel rules, §13.3 SetLockMode per slot + client pin targets, hack/disarm draw timing, §14 rule 8 traces, §14.5 invader defers to doc 16 §6.6 rule 2 (profile retired), test 41
- [x] Batch 4/5 applied: §15.1 fused Folios, new §15.3 time of day and weather (wx_* terms on sky_src cells)
- [ ] Next: Imports rows, Interface rows, Exports functions, Canon gaps G26-G28 + P1, tests 55 and new 62+, invariants, intro sentence; then lint
- [x] Batch 6-8 applied: intro sentence, §13.2 electronic locks via hack panel, Imports (doc 03 sky_src, doc 12 weather/fused, new rows doc 14, 16, 17, DECISIONS), Interface rows 03, 07, 12, 16, 17, Exports functions, Canon gaps G26-G28 + mount-noise retirement + P1, tests 55, 62-70, invariants
