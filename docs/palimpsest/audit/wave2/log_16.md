# Wave-2 fixer log, chapter 16

## Round 1, 2026-10-12

Inputs: requests_16.json (21 items), 10 inbound requests from the wave-2 passes of docs 03, 09, 13, 14 and 17, DECISIONS rows 89-145. Baseline lint: undefined_ids aware.profile.invader.

### Triage

| Item | Disposition | Where |
|---|---|---|
| R1 fix:14 delete G16, close G15 and G20 | already satisfied | Canon gaps: G16 absent, G15 and G20 under "Resolved on disk" |
| R2 verify:14 RunView backing; trace service and audience-vote path | RunView part already satisfied (§6.10 cites doc 14 FolioRun); services: edit | §9.14, §11.10, §11.6 |
| R3 verify:02 DECISIONS 112 and 123 (EOS, auth/epic, Epic relay, crossplay identity, platform value, Epic Workshop mirror) | already satisfied; ruling on flag.net.crossplay: it retires (cross_chapter_request to 02) | §3.1, §3.5, §3.6, §3.8, §11.1, §11.8; Interface row 02 |
| R4 verify:01 trace service and streaming-vote path | edit | §9.14, §11.10 |
| R5 fix:11 delete G11, drop "not mounted" | already satisfied | Canon gaps (G11 adopted by canon), §9.7 |
| R6 fix:15 pull example window, iat, kid; voice defaults | example: edit; voice: already satisfied | §11.4; §9.12 |
| R7 fix:18 ScribeText message | edit | §3.7 (0x53 net.msg.scribe_text), §6.15 |
| R8 verify:11 cite doc 11 §2.2 keepsake field | edit | Imports row doc 11, §9.6 |
| R9 verify:09 O1 reconciliation | edit: O1 closed (doc 09 §14.5 retired its filter) | Canon gaps O1, Imports row doc 09 |
| R10 verify:18 Appearance to joiners; ScribeText | edit | §4.3 LobbyPick, §9.1 JoinCharacter, §6.10 PartyState, §10.1, §6.15 |
| R11 verify:15 pull example; EOS paths; run report ironman field | pull: edit; EOS: already satisfied; ironman: edit | §11.4 |
| R12 fix:17 C-09 light byte, C-10 sanitizer, invader no pause | already satisfied | §6.8 HUD, §5.8, §2.2, §2.3 |
| R13 fix:03 custom masks; §8 item 2 wording | already satisfied | §6.11, §8 item 2 |
| R14 verify:17 O2 | edit | §9.14, §11.10 |
| R15 verify:03 crouch_base, climb_base in OwnerDelta.mover | edit | §6.10, §7.2 |
| R16 fix:16 unresolved O1 | edit (closed) | Canon gaps O1 |
| R17 fix:16 unresolved O2 | edit (closed) | §9.14, §11.10, Canon gaps O2 |
| R18 verify:16 retire net.route.relay, svc.relay, svc.workshop once 02 and 21 drop them | kept reserved: doc 21 still names them (generated); no change | Exports |
| R19 verify:16 LAN and direct-IP migration | edit | §6.10 PartyState.direct_addrs, §9.3 detection and takeover |
| R20 verify:16 falling block custom mask | edit | §6.4 EXTRA note and §6.11 falling-block rule |
| R21 verify:16 bit-25 sanitizer | already satisfied (docs 02 §5.5, 17 §12.2 require it) | §5.8 |
| I1 wave2:17 RunView Folio quest, laws_triggered, ev.law_discovered | edit | §6.10 RunView, §6.9 row 15 |
| I2 wave2:17 O2 | edit | §9.14, §11.10 |
| I3 wave2:03 MoveState slide and grapple fields, modes 8 and 9, start_grapple correction, BlockQuery mask, Climate | edit | §7.2, §6.10 RunView.climate |
| I4 wave2:09 close O1, Imports row text | edit | Canon gaps O1, Imports |
| I5 wave2:09 Interface row 09 footstep lifetime | edit (now present; row moved to Imports) | Imports row doc 09, Interface assumptions |
| I6 wave2:09 STEALTH panel with Steal cap | edit (cap 16, highest chance, ties lower uid) | §6.8 tag 0x06, test 59 extension |
| I7 wave2:13 net_always | already satisfied | §6.6 rule 3, §9.3 |
| I8 wave2:14 AudienceResult, vote service, ranked refusal | edit | §5.7, §9.14, §11.6 |
| I9 wave2:14 appearance and display name in LobbyPick and JoinCharacter | edit | §4.3, §9.1, §6.10, §10.1 |
| I10 wave2:14 trace service | edit; "send none to the Prologue" rejected (DECISIONS 132 retires the Prologue) | §11.10 |
| DECISIONS 123 verifier replays ranked, daily and weekly | already satisfied | §10.2 |
| DECISIONS 126 Workshop auto-sync | edit; cross_chapter_request 02 §13.4 | §3.3 pal_ws, §11.8 |
| DECISIONS 129 no photo mode | edit | §10.1 |
| DECISIONS 138 Fluent localization of generated text | edit | §6.15 |
| DECISIONS 139 no server-pushed tuning | edit | §11.9 |
| DECISIONS 140 telemetry rules | edit (never free text) | §11.7 |
| DECISIONS 144 3 profiles per Steam account, Steam Cloud | edit | §11.3 |
| Leftover sweep | no hit needs a change (seed hits are run seeds; "calendar year" is key rotation) | whole chapter |

