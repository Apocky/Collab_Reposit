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

## Round 1, 2026-10-12

Resume pass. Verified batches 1-8 on disk (run_alerted 0 hits; steps 30-tick lifetime in §8.3; doc 07 Imports wording; §13.2 slip_bp, no durability; §13.4 deck spark; doc 12 §11.3 and doc 08 §4.3 citations; no shadow term; §14.5 defers to doc 16 §6.6 rule 2). All held.

| Item | Disposition | Where |
|---|---|---|
| verify:14, verify:16, fix:07, fix:12, fix:08, verify:08, fix:03, verify:07, fix:17 | already satisfied on disk | §1.4, §8.3, Imports, §2.2, §11.1, §2.1, §13.2-§13.4 |
| fix:04 (consume a lockpick) | rejected | DECISIONS 65, Canon §4.3; rng.check half satisfied (§11.4) |
| G23, G24, G25, noise.mount retirement | canon_request | Canon gaps |
| DECISIONS 99 weather / inbound wave2:03 §15.3 | edit | §15.3 keyed on doc 03 Climate.wx and wx.* IDs, outdoor = sky_open, wx_sound dropped (doc 03 sound_ddb carries sound), example and test 63 recomputed, G26 aligned to doc 03's Canon §5.12 request |
| inbound wave2:03 kit noises | edit | §7.2 slide and reel rows, §7.3 grapple, Interface row 03, test 73 |
| inbound wave2:17 mark flags | edit | §8 struct, §8.3 Mark flags, Interface 16/17, test 71, invariant |
| inbound wave2:13 Glitch hacking | edit | §13.4 comp_hack, Interface 13, Exports, test 72, invariant |
| invader filter P1 | product_question (default doc 16 rule alone) | §14.5 |
| DECISIONS 130-145 | no doc 09 subject; sweep clean (remaining hits are retirement statements and pickpocket "plant") | n/a |

- Batch 9: header row 145, §4.4, §15.3, Imports 03/12, Interface 03/12/13/16/17, §7 noise rows, §8/§8.3 flags, §13.4, G26, tests 63 and 71-73, invariants, Exports. 25 edits. Recompute: rain example L_eff = floor_div(450 - mul_bp(200,10000),10) = 25 < gate 28; fog conceal floor_div(1*16000,8000) = 2.
- Final lint --chapter 09: "== 09 words=32150 / IDs defined in more than one chapter: 0" (no findings). Full run: no new finding outside 09 (14 enemy.prologue_hunter and 16 net.msg.scribe_text belong to other lanes; 16 aware.profile.invader is baseline).
- Status: complete.
