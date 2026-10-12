# Wave-2 fixer log, chapter 17

Started 2026-10-12. Inputs: requests_17.json (27 items), DECISIONS rows 89-129.

## Progress
- [x] Read chapter 17, requests_17.json (27 items), DECISIONS 89-129, and the owner sections they cite (doc 02 §2.2; doc 03 §5.9, G31, ev.law_discovered; doc 06 §5.3, §16.4; doc 07 §8.3; doc 09 §8, §13.2-§13.5; doc 12 §4.1, §5.7, §20.4-§20.6; doc 14 §2.1, §5.1, §5.5-§5.7, §8.9, Canon gaps; doc 15 §3.9, §3.9.1, §4.1, §4.3, §8.5, §8.6; doc 16 §3.3, §4.1, §6.8, §9.2, §9.3, §9.12, O2; doc 18 §5.4)
- [x] Baseline lint: clean (0 findings)

## Triage (request -> action)
- fix:14 G2/G4/G21 already closed in Canon gaps; G28/G29 wording identical to doc 14 gaps 6 and 3; arc beats already in §11.1 -> add `ui.menu.dramatis` with Glyph Library picks
- fix:16/verify:16 -> STEALTH caps and `bearing` in §10.10; ping_danger_ticks 15 in §12.3/§12.4; new §11.9 online sessions (lobby browser, mod sync, kick vote, voice unmute, stalled/migration overlays, cosmetic notice); stale entities already §16.1
- fix:02 -> §13.6 cites tts 0.26.3 and AccessKit via bevy_winit
- fix:01/verify:01 -> sprint-slide in §12.1, §12.3, §12.4
- verify:14 -> new §11.10 Second Edition (door, stakes, Ironman), Interface row 14 closed; Second Edition at EA per DECISIONS 118
- fix:07/verify:07 -> doc 07 §8.3
- fix:09/verify:09 -> §10.11 rewrite (hacks, seq 1004, 30-tick restart, third crack = failed hack, deck spark); Interface row 09 slip wording
- fix:15/verify:15 -> new §11.7 Vellum Archive with Transcription shelf, §11.8 Fairness panel, Ironman frame; voice-muted non-friends; gore default already ink
- fix:18 -> §13.5 gore row wording
- fix:12/verify:12 -> §7 discovery rule per doc 12 §4.1; Folio header Law glyphs; Folio quest under the Rubric
- fix:06/verify:06 -> §6.5 appraise/psychometry only; §10.7 3480 ticks (58 s)
- fix:10 Crowns -> already satisfied
- verify:03 -> Interface row 03 mantle 1,600 mm until doc 03 G31
- unresolved carry-overs: G28/G29 -> canon_requests; Law discovery inputs now on disk (doc 12 §4.1, doc 03 ev.law_discovered, doc 14 §5.5); petition 32 vs 40 still open -> doc 06; input.mount gone from docs 01, 05; §7 heading renamed to "Index" -> doc 21 refresh
- DECISIONS: 93 migration overlay; 112 no crossplay/friend codes; 118 SE at EA, traces/votes/invasions post-EA; 120 Scribe tab at 1.0; 126 Workshop mod sync; 127 no resident relationships; 129 no photo mode; 98 Windows screen readers only

## Edits
- [x] Batch 1 (20 edits): header DECISIONS line; Imports rows (DECISIONS, doc 02 §2.2/§13.4, docs 03/07 §8.3/12 §4.1 §20.4-§20.6/14 §8.9/15 §3.9 §4.1 §4.3/16); §3.2 and §3.4 Folio header Law glyphs; §6.1 doc 07 §8.3; §6.5 appraise/psychometry only; §10.7 3480 ticks (58 s); §13.5 gore wording; §13.6 tts 0.26.3 and AccessKit via bevy_winit, Windows readers only; §15 L3 per doc 14 §11
- [x] Batch 2 (11 edits): §7 heading renamed "Journal and Index knowledge"; Run log quest outcome; §7 discovery rule per doc 12 §4.1 (run-start set or laws_triggered mask; toast on ev.law_discovered; account set at run end); Folio quest under the Rubric; §9.1 Second Edition door pointer; §9.7 second ending layer; §9.8 no photo mode; §10.10 STEALTH caps, flags bit 0, bearing field; §10.11 rewrite (hacks and Seize, seq 1004, W, pin restart at now + 1 + SLIP_TICKS, third crack outcomes, deck spark)
- [x] Batch 3 (13 edits; scripts now in scratchpad/ch17/ because the scratchpad is shared): §11.1 Idle "Join a party", ErratumSelect uses `ui.menu.dramatis`, Dramatis paragraph with Glyph Library picks, RunSetup invasion toggle hidden before invasions ship, resident box without relationships (127); §11.2 Fairness tab; §11.3 Fairness button; §11.6 Abandon in Second Edition, Social tab (voice mutes, no crossplay), Generated text hidden before 1.0 (120), scoreboard portrait frame, voice-muted glyph, Unmute/Kick; Later screens now traces and votes only; new §11.7 Vellum Archive + Transcription shelf, §11.8 Fairness panel, §11.9 Online sessions (lobby browser, mod sync 126, cosmetic notice, voice, kick vote, stalled, migration 93), §11.10 Second Edition at EA (door order, stakes, HUD, summary, Ironman mark and frame)
- [x] Batch 4 (8 edits): §2.5 `ui.icon.ironman`; §10.12 net_status shares the overlay rectangle; §12.1 Movement kit paragraph (mantle 1,600 mm until doc 03 G31, sprint-slide = crouch press while sprinting, grappling tool as quickbar item); §12.3 sprint-slide row and ping within 15 ticks; §12.4 R3 slide and D-pad ping; §12.9 slide toggle note and ping_danger_ticks
- [x] Batch 5 (14 edits): Exports (ui.hud 35 with net_status; ui.menu 36 with dramatis, vellum_archive, fairness, lobby_browser, mod_sync; ui.icon 62 with ironman; non-ID: slide rule, stakes lines); Interface rows 02 (tts/AccessKit present, FOV cite, Workshop auto-sync), 03 (mantle 1,600 mm until G31, slide, ev.law_discovered), 09 (slip wording, hacks, seq 1004, deck spark, flags bit 0), 12 (discovery present, Folio quest, fused pair), 14 (SE present, EA per 118), 15 (Transcription, Fairness, Ironman, voice), 16 (STEALTH caps, ping 15, RunView quest and laws_triggered, network items present)
- [x] Batch 6 (3 edits): product questions 54-58; acceptance tests 70-80; invariants (photo mode/crossplay/friend codes, stakes, arc bearing, slide, Law visibility)
- Lint after batch 6: 1 hedge ("could") -> fixing
- [x] Batch 7 (1 edit): kick-vote sentence without "could"
- [x] Batch 8 (3 edits): ModeSelect excludes Second Edition; dropped row 117 from §11.10 heading and Imports (lands in docs 12, 14 only)
- Lint: clean
- [x] Batch 9 (2 edits): deck-spark wording (doc 07 §11.1 rule 4); World Laws page uses the viewer rule
- Lint: clean. All 27 requests and DECISIONS 89-129 rows naming doc 17 handled; preparing structured summary (canon_requests G28, G29; cross-chapter requests to 02, 03, 06, 09, 12, 14, 16, 21)
