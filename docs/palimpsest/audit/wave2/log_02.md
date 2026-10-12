# Wave 2 log, chapter 02

## Round 1, 2026-10-12

Triage (requests_02.json items 0-27, then inbound I1-I4, then DECISIONS rows):

| Item | Disposition | Where |
|---|---|---|
| 0 fix:13 curve JSON key | already satisfied | §11.5 uses key "curve" |
| 1 fix:16 ACTION/WHEEL_ENTRY bits, bit 25 | already satisfied | §5.5 bit table |
| 2 fix:16 svc.lobby beacon search | already satisfied | §14 Steamworks bullet |
| 3 fix:16 run summary fields, minor bar | already satisfied | §17.4 |
| 4 fix:16 MOD section sim mods only; svc.workshop | already satisfied | §11.2, §13.1, §13.2, §13.4, test 35 |
| 5 fix:16 Opus encoder | already satisfied (opus row); EOS part rejected (DECISIONS 112, 123) | §2.2 |
| 6 fix:01 run summary C-14 | already satisfied | §17.4 |
| 7 verify:16 drop EOS, Epic, crossplay | EOS/Epic already gone; leftover wording edited; flag.net.crossplay kept defined (doc 16 row 02 still names it in backticks) and marked retired, cross_chapter_request to 16 | §14, Imports, Exports, Interface 16 |
| 8 fix:11 DropLucre | already satisfied | §5.5 |
| 9 fix:15 econ Transcription, pal odds per build | econ satisfied; pal odds edit | §3.2, §3.4 pal subcommands |
| 10 fix:18 64 px layers, G1 | already satisfied | §8.3, Canon gaps |
| 11 verify:18 voice pools, web leftovers | already satisfied | §9, §11.4 |
| 12 verify:15 verify binary, 30-day replays, EOS | 30-day replays satisfied (§12.2); EOS gone; palimpsest-verify stays in palimpsest_tools: rejected, §3.2 states why (it links palimpsest_sim, which econ may not) | §3.2 |
| 13 fix:17 bits and intents, FOV | already satisfied | §5.5, §8.4 |
| 14 fix:12 citations §7.3/§14.1, bleed_on | edit | Imports, §5.5 SimConfig, §6.5 section 0, §7.2, §8.3 |
| 15 fix:06 SetCastItemTarget, Pray | already satisfied | §5.5 |
| 16 fix:03 dda max_steps, masks, ChunkView, shadow areas | already satisfied | §4.6, §6.5, §7.2 |
| 17 verify:17 FOV citation | edit | Imports, §8.4 |
| 18 verify:12 citations | edit (same as 14) | |
| 19 verify:06 ComposeSpell { slot, refs } | edit | §5.5 |
| 20 fix:02 G14 | canon_request | Canon gaps G14 |
| 21 fix:02 doc 03 mask exports | already satisfied on doc 03 (mask_at, masks, block_gen) | Interface 03 |
| 22 fix:02 code ray_sphere h==0, dda max_steps | already satisfied in code (geom.rs h<0 test; dda takes max_steps) | none |
| 23 fix:02 scratchpad note | no action (process note) | |
| 24 verify:02 crossplay retire | see item 7 | |
| 25 verify:02 playtest channel | product_question 02-Q1, default applied: channel dropped (DECISIONS 113) | §17.2, §17.3 |
| 26 verify:02 concurrency note | no action | |
| 27 verify:02 MIT licence | rejected for this chapter: doc 15's call | |
| I1 wave2:07 loot.rs Seed | code follow-up stated in §18 item 10; unresolved for orchestrator | §18 |
| I2 wave2:17 FOV citation | edit | §8.4 |
| I3 wave2:03 Climate, perf rows | Climate edit, placed in FolioView (fixed per Folio); perf rows already satisfied (§15 intro) | §7.2 |
| I4 wave2:09 folio_alerted; StealthHud.panel | edit section 12; StealthHud.panel n/a (doc 02 does not mirror doc 16 snapshot schema) | §6.5 |
| DECISIONS 126 | edit | §13.2, §13.4 |
| DECISIONS 138 | edit | §2.2, §10 |
| DECISIONS 139 | edit | §17.3 |
| DECISIONS 140 | edit | §17.4 |
| DECISIONS 144 | edit; cross_chapter_requests 14, 16 | §12.2, §12.4 |
| Leftover sweep | Epic/demo/playtest wording edited; "Plant" is doc 09's stealth intent, kept | Imports, §14, §16, §17.2 |

Code follow-ups re-checked on disk: isqrt_u64 present (fx.rs), sim and math no_std under cfg(not(test)), data modules loader, schema, mods present, ray_sphere tangent hit, dda max_steps. Remaining: loot.rs Category::Seed.

Batch 1: header, Imports, citations (doc 12 §7.3/§14.1, doc 17 §13.5), ComposeSpell payload, SimConfig.bleed_on, section 0 and 12, FolioView.climate, Fluent (138), profiles (144), mod sync (126), storefront wording, channels without playtest (113), patch-only tuning (139), telemetry free text (140). 32 edits. Lint --chapter 02 clean.
Batch 2: §18 item 10 code follow-up (loot.rs Category::Seed, ALL 31 -> 30), Exports (crossplay retired but listed; new non-ID interfaces), Interface rows 03, 09, 12, 14, 15, 16, 17, tests 54-60, three invariant rows. 15 edits. Lint --chapter 02 clean.
Final lint: --chapter 02: no findings. Full run: "IDs defined in more than one chapter: 0"; findings elsewhere are other chapters' (00 hedges optionally 1; 14 enemy.prologue_hunter; 16 aware.profile.invader, net.msg.scribe_text); none of these IDs occurs in chapter 02. Export counts unchanged: crate. 20, flag. 23, perf. 58.

Re-run check 2026-10-12: round 1 already applied and committed (e18d2f2). Header, sweep (only compliant mentions: mounts cut, no item durability, EOS dropped) and lint re-verified: --chapter 02 no findings; full run "IDs defined in more than one chapter: 0". loot.rs Category::Seed still present (lines 610, 647, 680): code follow-up open. No new edits.
