# Fix log, chapter 16 (Multiplayer, Netcode and Online Services)

As of 2026-10-11. File: `spec/16_MULTIPLAYER_NETCODE_SERVICES.md` (`spec/16_NETCODE_COOP_SAVES.md` is a symlink to it). Authorities: DECISIONS.md, canon-1.11, `audit/canon_gaps_triage.md`.

Lint before: `retired_ids (2): ev.dismounted, ev.mounted`; `--json` multiply_defined lists `ev.player_ghosted`, `ev.player_resurrected`, `ev.player_revived` for 03 and 16 (doc 16's Exports named them).

## Batches applied

### Batch 1: header, retired mounts (C-30, canon-1.8 §17.7), MoverSnap (doc 03 interface)

- Header and intro: "Conforms to canon-1.11"; intro lists the canon-1.11 sections checked and records, in one sentence, that mounts (canon-1.8) and item durability (canon-1.10) appear nowhere (G-051).
- §6.4 FLAGS: bit 5 `mounted` removed, bits renumbered compactly (0 dead … 11 asleep, 12–15 zero). EXTRA: the rider/mount attach handle removed (light 4 + aux 8 = 12 bits).
- §6.8 create record: 342 → 329 bits (group bits 219); test 10 and §15 Folio-entry burst: 300 creates = 98,700 bits = 12,338 B (was 12,825 B).
- §6.6 rule 3: "the mount of a relevant rider" removed. §6.7 `w_kind`: "other creature|mount" → "other creature".
- §6.9: event row 13 (`ev.mounted`, `ev.dismounted`) deleted; item events become row 13, invasion events row 14.
- §7.1: mounted prediction removed. §7.2 `MoverSnap`: `ride` removed (doc 03 Interface row 16: "MoverSnap carries no ride field"); `speed_bp` replaced by `status_speed_bp: [Bp; 8], n_status: u8` handed unchanged to `MoverParams` (doc 03 Interface row 16: a folded product rounds differently from Canon §4.2's per-status chain); `gates` bit 18 `mounted` removed; `MoverParams` sources spelled out against doc 03 §5's struct; "mount collisions" removed.
- §9.4 downed: "or mount" removed; valid reviver: "not mounted" removed. §9.7 trade eligibility: "not mounted" removed.

### Batch 2: canon-1.11 alignment in the session, input and lag-compensation text

- §2.2 solo: saves deferred while an invader holds slot 4 (Canon §2.5, G-019); no pause while an invader is present (doc 17 P8 agrees).
- §2.3: game speed (`access.game_speed_bp`) locked to 10000 with a remote peer, invader included (doc 17 Interface row 16).
- §3.3: `pal_mods` is display-only and dropped from the Quick Join filter (DECISIONS 87, G-017); new **Lobby browser** bullet (DECISIONS 74).
- §3.8: `Hello.modset_hash` → `presentation_hash`; `Hello.platform` values 0 Steam, 1 Epic; `Welcome.presentation_hash`; refusal only on `content_hash` (which covers simulation-affecting mods); a `presentation_hash` difference shows a notice. The G7 sentence replaced by the canon rule (run_seed in the RunHeader at Commit, G-018).
- §5.4: "(Canon gaps G4)" → Canon §11.8 (G-114); `ACTION`/`WHEEL_ENTRY` are 0 in the substitute.
- §5.8 sanitizer (C-10): reserved-bit test dropped; replaces a command with `ACTION` 6 or 7 or bit 25 set (doc 17 §12.2 asks for bit 25 too); `WHEEL_ENTRY` listed with the self-bounded fields.
- §6.1 / §6.2: hash and snapshot boundaries cite Canon §2.2 and §2.5 (G-015); "(Canon gaps G5)" removed.
- §8 item 2 (C-08): catch-up steps, then the normal step against the poses of `t`, then one step per tick from `t + 1` (doc 03 §5.19).

### Batch 3: co-op rules

- §9.4 `Player` now matches doc 03 §3.3 `comp.player` (`mercy_on`, `mercy_bp`, `breath_held` from doc 17; slot 0..=4). New sentence: Canon §11.1 fixes the first down (1,800 ticks) and the 180-tick revive and gives the rest to §9.4 (G-110, "downed rules in doc 16").
- §9.4 party wipe: Redaction exception when the invader's hit drops the last Erratum alive and not downed (G-173, "Redaction while downed").
- §9.5: ghosts share kill XP, downed included; Dormant, Departed and the invader do not (Canon §8, DECISIONS 81, G-072, C-01).
- §9.6 `DropItemFor` and §9.7 trades refuse Lucre; Lucre passes only as doc 11 §17 `DropLucre` piles (Canon §14, G-148).
- §9.7 trade events carry `cause = 1` / `from = 3` (doc 03 §14.2 now has them; G-184, old G10).
- §9.13 and §12: "empty mod set" → no simulation-affecting mod (doc 14 §2.1 wording; DECISIONS 87).
- §11.8 Workshop: Epic build mirror, console clause removed (DECISIONS 98); mod match through `content_hash` only.
- Test 25: presentation-only mod difference is admitted with a notice.

### Batch 4: snapshot sections and owner/run views

- §6.8 `HUD` (C-09): explicit 16-bit mask order; new `light` byte beside `light_at` (low nibble `light_block`, high nibble `light_sky`, after light overrides, the `ChunkView.light` layout).
- §6.8 new section tag 0x06 `STEALTH`: doc 09 §8's `StealthHud` as `StealthWire` (gem, hidden, ≤ 4 rings relative to the receiver in mm, ≤ 8 marks with net handle and bearing, ≤ 4 step yaws), full when changed. Docs 09 and 17 both list "`StealthHud` in each player's snapshot" as an interface assumption on doc 16. §6.1 row, §6.5 `View.stealth`, §6.7 step 2 mandatory list updated.
- §6.10 `OwnerDelta.mover: Option<MoverParamsView>` (the doc 03 §5 `MoverParams` fields the predictor needs). `RunView` gains `rooms_visited` and `map_revealed` (doc 17 asks for the party map reveal of `item.text.mapping`; doc 14 must hold them in `RunState`).
- §6.11: the HUD light readout reads `HUD.light_at` and `HUD.light`.

### Batch 5: crossplay, voice, pings, telemetry

- DECISIONS 73 and 98: §1.1 decision 6, §3.1 relay route row, §3.5 (relay ships with the Epic Games Store build; EOS is the Epic platform layer; no EOS P2P, so every session with an Epic member is relayed), §3.6 platforms `steam` and `epic` only (no consoles), crossplay toggle limits to the player's own store, §11.2 `POST /v1/auth/epic`, link example `"epic"`.
- DECISIONS 75: §9.12 voice is Opus on every build (Steam's voice codec cannot be decoded by Epic clients); voice with strangers starts muted; new message 0x52 `net.msg.voice_mute` (`VoiceMute { muted: u8 }`) so the host forwards only unmuted talkers.
- Doc 17 request: a second `PING` within 15 ticks upgrades the previous ping to danger.
- C-14 / G-199 / G-222: §11.7 run summary carries doc 01's per-Folio fields; host-only in co-op; minor-flagged slots omitted; minors send nothing. Test 35 extended.

### Batch 6: Imports, replay header, migration preface

- Imports: DECISIONS row now 15, 16, 48, 73–76, 81, 83, 86, 87, 93, 98 (row 6 superseded by 98); canon rows updated for §1.9 `yaw_from_dir`, §2.1–§2.5 (boundaries, `presentation_hash`, `run_seed` at commit, intent records, hash boundaries, solo save deferral), §8/§10.8/§11.1/§11.8, §14 (Lucre piles), §17.9 (Redaction while downed), §18 (doc 03 registers events); doc 03 row (kind code 2 retired, `status_speed_bp`, catch-up then normal step, light overrides, 5-entry `heard`, trade codes); new rows for docs 06 (§8.3 mapping), 09 (§8, §14.5), 17 (§12.2, §13) and 01 (§10.3); doc 11 row adds `DropLucre`.
- §10.1 `ReplayFile.presentation_hash` (Canon §2.4: recorded, mismatch warns).
- §9.3 preface: DECISIONS 93 refines 76; doc 14 follows §9.3 (C-06; doc 14 line "Host migration follows doc 16 §9.3" is on disk).

### Batch 7: Interface assumptions and Canon gaps rewritten

- Interface assumptions: met requests removed (doc 03 `heard` and trade codes, doc 14 G15/G20, G-tags); added doc 02 C-10 bit table, C-13 wording, C-14 run summary, presentation-only mods out of the handshake, EOS layer, Opus encoder; doc 03 `MoverSnap` comment and invader projectile `catchup`; doc 04 predicates; doc 09 `StealthHud`; doc 14 visited rooms and map reveal; doc 17 lobby browser, voice unmute, notice, STEALTH caps.
- Canon gaps: none open; G1–G22 closed and kept reserved with their disposition (adopted by canon-1.11: G2, G3, G4, G5, G7, G10, G11, G12, G16; own rules: G6, G17, G19; resolved on disk: G1, G8, G9, G13, G14, G15, G20, G22 — triage §4.1 asked to delete G9 and G22; doc 02 changes: G18, G21). C-01's "delete G16" done.
- Open items: O1 doc 09 §14.5 invader filter vs §6.6 rule 2 (no triage ruling; §6.6 governs meanwhile); O2 DECISIONS 96 and 104 not yet specified.

### Batch 8: Exports

- 154 IDs (`net.msg.voice_mute` added, `net.msg.` 39); fnv1a32 re-checked: no collision with the 5,302 distinct ID-shaped tokens of the other chapters; example hashes verified.
- `ev.player_revived`, `ev.player_ghosted`, `ev.player_resurrected` no longer appear in Exports (they made the lint report 03/16 multiply-defined); Exports now names the emission duties and points to Imports. `HUD` light byte, `StealthWire`, `MoverParamsView`, `VoiceMute` listed as non-ID interfaces.
- Hedge "optionally" (lobby browser) reworded.
- Lint after batch 8: `== 16 words=27837`, no findings; `--json` multiply_defined has no entry with 16.

### Batch 10: acceptance tests and invariants

- Tests 57 (sanitizer, C-10), 58 (HUD light byte, C-09), 59 (STEALTH section caps and a byte vector), 60 (voice with strangers, DECISIONS 75), 61 (solo save and no pause during an invasion, Canon §2.5), 62 (invader projectile catch-up, C-08), 63 (SDR vs relay routes, DECISIONS 73), 64 (map reveal).
- Invariants for the sanitizer, voice mute, solo invasion save and no pause, relay route, presentation_hash never a refusal, STEALTH caps, per-status speed list.
- §16 scripted cases: a Steam and an Epic client finish a Folio over `net.route.relay` (doc 21 M5).

### Batches 11–13: crossplay routing made exact (DECISIONS 73)

- §3.5 **Route choice**: route fixed at lobby creation and never changed after `Commit`; Epic hosts always `svc.lobby` + relay; Steam hosts use a Steam lobby on SDR unless crossplay is on and the lobby is public; inviting an Epic player by friend code into a private or friends-only Steam lobby before `Commit` moves it to `svc.lobby`; Epic players join only `svc.lobby` lobbies; LAN and direct IP unchanged.
- §3.3: with crossplay on, Quick Join and the browser also list `svc.lobby` public lobbies, ranked by RTT to the relay node.
- §9.13 and §11.5 invasions: beacons carry `route` (`sdr` or `relay`) and `relay_node`; the ticket returns a relay connect token for relay beacons; Epic invaders see only relay beacons.
- Test 63 covers each route case.

### Batches 14–18: privacy, hex authority, params, offline, bleed-out event

- §9.13 Information: the invader's `RunView` has `doors` and `rooms_visited` empty and `map_revealed` false (they would reveal where the party is or has been).
- §6.8 `STEALTH`: never sent to the invader (doc 09 §14.5: no meters).
- §11.1: the hex forms bind every service body and every JSON example another chapter submits (C-07, "format authority").
- `net.params.default` gains `"ping_danger_ticks": 15` (the §9.12 ping window is a co-op tuning value).
- §13 offline table: relay sessions need `svc.lobby` and `svc.relay`; LAN and direct IP on either store.
- §9.4 Bleed: a bleed-out's `ev.entity_killed` carries `damage_seq = 0xFFFF_FFFF` (doc 03 Interface row 16; notes_events_sim item 7).

### Batches 19–21: wording

- Canon §4.3 import names `block.tag.destructible`; §6.9 row 3 cites doc 03 §11.5 only; §1.1 decision 6 says "Steam-only online sessions" use SDR and sessions open to Epic players use `svc.relay`; Exports collision sentence re-counted (5,309 distinct tokens of other chapters, no collision).

## Requests for other chapters

- **02** §5.5: bits 26–28 `ACTION`, 29–31 `WHEEL_ENTRY` replace "reserved, must be 0"; bit 25 reserved, always 0 (C-10, C-30).
- **02** §14 Steamworks bullet: "invaders find open sessions through doc 16's `svc.lobby` beacon search" (C-13).
- **02** §17.4: run summaries add, per Folio, genre, entry and exit ticks, offered and committed doors, the maximum equipped `C` per slot at entry, a Disconnected-weapon flag per slot, committed Rewrites, NPC kills split systemic and other; bar every minor-flagged account (C-14).
- **02** §11.2, §13.4, test 35: only simulation-affecting mods enter `content_hash` and the handshake refusal; presentation-only mods enter `presentation_hash` (Canon §2.4, DECISIONS 87). §13.2's console sentence names a doc 16 console review that no longer exists (DECISIONS 98).
- **02**: an Opus encoder crate (voice, doc 16 §9.12) and the EOS platform layer for the Epic build.
- **03** §5 SPEC comment: `MoverSnap` lists `status_speed_bp, n_status` instead of `speed_bp`. §5.19: `catchup` also for projectiles owned by the slot-4 invader.
- **14**: `RunState` keeps the party's visited room indices (not only `rooms_visited` as a count) and the current Folio's `item.text.mapping` reveal flag, which doc 16 `RunView` replicates.
- **15** §8.5 Adult defaults: voice with players who are not friends starts muted (DECISIONS 75, doc 16 §9.12).
- **17**: draw `StealthHud` from doc 16's `STEALTH` section (at most 4 rings, 8 marks, 4 steps; mark direction from `bearing`).

## Open

- O1: doc 09 §14.5 invader interest filter vs doc 16 §6.6 rule 2 (no triage ruling; §6.6 governs until one lands).
- O2: DECISIONS 96 (asynchronous traces) and 104 (streaming votes) land in doc 16 and are not yet specified.

Lint after: `== 16  words=28642` with no findings; `--json` lists no multiply-defined ID for chapter 16.

## Verification

Adversarial verifier pass, 2026-10-11. Pre-pass copy compared line by line; every edit below was made by a Python script that asserted each anchor occurs exactly once. Note: DECISIONS rows 109 to 121 were appended to DECISIONS.md at 23:16, after the fixer finished (22:41); row 112 lands in doc 16 and is applied here.

### Check 1. Triage rows that name chapter 16

| Row | Verdict | Chapter text that satisfies it |
|---|---|---|
| G-015 (16 G5) | pass | §6.1: "every boundary `b` with `b % 60 == 0` and `b >= 60` (Canon §2.5)"; §6.2: "extracts a `ReplFrame` for boundary `b = t + 1` (Canon §2.2 period 2, phase 0" |
| G-016 (16 G2) | pass | §5.7: "at the start of that tick in `(slot, action_seq)` order … The CIL and the replay record `apply_tick`"; slot-4 `InvaderJoin`/`InvaderLeave` are doc 02 intents |
| G-017 / Q11 | pass | §3.8: "`presentation_hash` is never a refusal reason"; test 25 |
| G-018 (16 G7) | pass | §3.8: "`Hello` carries no seed, because none exists before `Commit`; the lobby receives `run_seed` in the `RunHeader` the host sends at `Commit` (Canon §2.4)" |
| G-019 | pass | §2.2: "Solo saves at any tick boundary, except while an invader holds replay slot 4: a save request then completes on the tick `ev.invasion_ended` is emitted (Canon §2.5)" |
| G-051 (16 G1) | pass | header "Conforms to canon-1.11"; intro "Canon-1.8 cut mounts and canon-1.10 removed item durability, so neither appears in any rule below" |
| G-072 (16 G16) / Q1 / C-01 | **fixed** | §9.5: "A ghost shares kill XP like every member present in the Folio, downed members included (Canon §8, DECISIONS 81); Dormant and Departed slots and the invader share none". C-01 says "Doc 16 deletes G16"; the fixer had kept G16 in the closed list. Deleted (no other chapter cites doc 16 G16) |
| G-110 (16 G12) | pass | §9.4: "Canon §11.1 fixes the first down's 30 s bleed-out (1,800 ticks) and the 3 s revive (180 ticks) and gives every later down, damage shortening and revive HP to this section" |
| G-114 (16 G4) | pass | §5.4: "Canon §11.8 reads it as the absence of input, not synthesized input" |
| G-148 (16 G11) / Q3 | pass | §9.6: "for Lucre (`cat.currency`), which passes between party members only as doc 11 §17's Public `DropLucre` piles (Canon §14)"; §9.7 |
| G-153 | pass | §11.4 `POST /v1/econ/runs/report` carries doc 15 §10.4's report |
| G-173 (16 G3) | pass | §9.4 party wipe: "when the invader's hit brings the last Erratum that is alive and not downed to 0 HP, the Redaction of §9.13 resolves on that tick instead"; §9.13 Redaction; test 39 |
| G-174 / Q8 | pass | §11.5: outcome "pays the invader 10 Quires under doc 15 §2.4"; no Codex XP anywhere |
| G-175 (16 G17) | pass | §6.14 invasion sealing; Canon gaps lists G17 as an own rule |
| G-184 (16 G10) | pass | Exports: "the emission of the downed, revive, death, ghost and resurrection events whose IDs are canon's and whose payloads doc 03 §14.2 registers"; no `ev.` ID defined here |
| G-199 | pass | §11.7: "per Folio (doc 01's KPIs, triage C-14): genre, entry and exit ticks, the offered doors and the committed door, …" |
| G-209 (16 G6, G8, G13, G14, G19, G22) | **fixed** | G6 §8 item 4 and G19 §3.8 stand as own rules; G22 deleted (§4.1) |
| G-218 (16 G9) / C-08 | **fixed** | §8 item 2: "it then takes its normal step against the poses of `t`, and one step per tick from `t + 1` (doc 03 §5.19, doc 05 §2.6)"; §9.4 "crawl 800 mm/s"; G9 deleted (§4.1) |
| G-219 (16 G15) / C-06 | pass | §9.3: "hot and warm migrations continue the Folio from the live state at `L + 1`; only a cold migration … restores the Folio-entry save"; doc 14 §8.8 "Host migration follows doc 16 §9.3" is on disk |
| G-220 (16 G18) / C-13 | **fixed** | §9.13 Matchmaking through `svc.lobby` beacons; doc 02 §14 now reads "invaders find open sessions through doc 16's `svc.lobby` beacon search", so G18 moved from "doc 02 changes" to "resolved on disk" and the Interface row was dropped |
| G-221 (16 G20) / C-07 | **fixed** | §11.1: "These forms bind every request and response of every service and every JSON example in another chapter that a service accepts, such as doc 14 §9.4's ranked submission (triage C-07)"; doc 14's example is lowercase on disk. Miss: the same paragraph said "any other hex form is refused", yet §11.5 `content_hash16` (8 bytes) and §11.7 `install_id` (16 bytes) use other forms; added the rule for other fixed-length byte strings and for `client_seed_hex` |
| G-222 (16 G21) / C-14 | **fixed** | §11.7: "A minor-flagged account sends no telemetry at all, whatever the setting (§14)"; doc 02 §17.4 now carries the fields and bars minors, so G21 moved to resolved and the Interface row was dropped |
| C-09 | pass | §6.8 `HUD`: "`light` u8 (low nibble `light_block`, high nibble `light_sky` of the `light_at` cell after the active light overrides, the `ChunkView.light` layout)"; test 58 |
| C-10 | pass | §5.8: "replaces with the substitute of §5.4 a command whose `ACTION` field (bits 26–28, doc 17 §12.2) holds 6 or 7, whose bit 25 is set (reserved, always 0), or …". The bits 26–31 reserved test is gone. The bit-25 test stays: doc 02 §5.5 ("reject a command with bit 25 set or with `ACTION` 6 or 7") and doc 17 §12.2 require it (retired mount toggle, C-30) |
| §4.1 (doc 16 G9, G22) | **fixed** | both deleted from Canon gaps |

### Check 2. DECISIONS rows

- 15, 16, 48, 74, 75, 76 with 93, 81, 83, 86, 87: agree (§9.13; §6.11 "limited-building rule"; §10.1; §3.3 Lobby browser; §9.12 "Voice with strangers starts off"; §9.3 preface; §9.5; §9.6–§9.7; §11.5; §3.8).
- 6, 73, 98 and **112 (Steam only; no Epic Games Store; EOS crossplay no longer needed)**: the chapter specified an Epic build, EOS, `/v1/auth/epic`, account linking, friend codes, a crossplay setting, `svc.lobby` lobbies, relay routing and an Epic Workshop mirror. **Fixed**: Imports (rows 107, 112 added; 73 and 98 restated; R3 A2 and `flag.net.crossplay` dropped); §1.1 decision 6; §3.1 relay row; §3.3 crossplay sentence; §3.4; §3.5 rewritten as "Route choice" (every online session is a Steam lobby on SDR); §3.6 "Identity" (`steam` only; friend code and crossplay setting removed); §3.7 0x62; §3.8 `Hello.platform`; §9.3 step 1; §9.12 voice; §9.13 matchmaking; §11.1 services, keys, hosting; §11.2 Epic and link endpoints removed; §11.3; §11.5 rewritten (no lobby endpoints, no `route`/`relay_node`/`relay` fields); §11.8; §13; §14 rows; §16 scripted case; tests 26 and 63; invariants. `net.route.relay`, `svc.relay` and `svc.workshop` are kept in Exports as reserved IDs (doc 02 and doc 21 still cite them), with a sentence saying nothing uses them.
- 107 (Second Edition: every slot enters; ghosts return as at death): **fixed**: §9.5 "At the entry of Second Edition Folio 1 a ghost's Remains count as carried, so it returns as it was at death, with `GHOST_REMAINS_HP_BP` and no item lost (doc 14 §8.9, DECISIONS 107)"; §9.1 item 5 now names Second Edition Folio 1 as the next entry after `final_folio`.
- 96, 104: open item O2 (unchanged; doc 21 schedules them after Early Access, DECISIONS 118).
- Changed rows: invasions, limited building, mounts, durability: agree. Softer Erasure: §6.6 rule 3 and event row 12 assume one hunter, no repeats: agree. Echo cast: `MoverParamsView.body` carries non-humanoid bodies: agree. **Hybrid glyph learning (33): fixed**: neither the lobby pick nor `JoinCharacter` carried the 0..=2 Glyph Library picks doc 14 §2.1 and §2.5 require; added `LobbyPick { slot, erratum, kit, library_glyphs, satchel }` (0x71 also lacked the kit and the Satchel order) and `JoinCharacter.library_glyphs`. **Micro-voxels (37): fixed**: §6.11 replicated block states and crack stages only, so chipped cells (doc 03 §13.2 `ev.block_chipped`, which lists consumer 16) never reached clients and prediction collided with the wrong shape; added a mask list to `WorldEdits`, mask-aware overlay and `BlockQuery`, §6.1 row, test 65, invariant (doc 03 Interface row 16 asked for exactly this). Text-only narration, USD 19.99, procedural ending, World Laws, pact curses, 12+ enemies, Errata limbs: no text in this chapter touches them; agree.
- 77 to 88: 77 ("Invade in the Margin"), 81, 82 (no Mercy gate here; board eligibility is doc 14's), 83, 86, 87 agree; 78, 79, 80, 84, 85, 88 do not touch this chapter.

### Check 3. Brief items

C-01, C-06, C-07, C-08, C-09, C-10, C-14: see check 1. G9, G22 deleted. `ev.mounted`/`ev.dismounted`: absent (lint `retired_ids` empty). `ev.*`: Exports defines no `ev.` ID; `--json` multiply_defined has no entry with 16. Downed rules owned here (§9.4), saves deferred during an invasion (§2.2, test 61), Redaction while downed (§9.4, §9.13, test 39): present. Decisions 73–76: see check 2 (73 narrowed by 112).

### Check 4. Leftover grep

Hits: line 5 (the retirement sentence, correct), "Redactor invader" in the Canon §17.9 import (canon keeps Redactor for invaders, G-177), "amount" (2), "a trade … breaks" (2), "override" (substring). None is a real leftover. No hit for durability, repair, wear, broken items, Delving, voiced, VO, 24.99, Item Codex, Crowns, margin.plot, building plot or 5400.

### Check 5. Lint

`spec_lint.py --chapter 16`: `== 16 words=28297`, no findings (one transient `undefined_ids` hit, a backticked field path, was reworded). `--json`: `multiply_defined` is empty for every chapter; no chapter has `undefined_ids`, so the reserved IDs still resolve for docs 02 and 21.

### Check 6. Internal consistency

- Recomputed in Python: input packet 37 bytes; substitute buttons 163,856; view clamps 4984/4990/4985/4984/5000; slack EWMA 1750…900; priorities 697/1000/60/577/266; quantization; field cell `a2 08 07 00 02`; world edit `08 00 00 03`; record sizes 74 and 329 bits, 300 creates 12,338 B; bandwidth table (12,000 + 1,400 + 1,900 + 700 = 16,000; invader case 6,560 and 218; backlog 9,560; inputs 6,480 B/s, 155.5 kbit/s; voice 5,600 B/s); combat snapshot 329 B; bleed 360 and 424 ticks; revive HP 42,500/21,250/10,625/8,500 and Grade 2 39,100/19,550; interp delays 6/8/12/4; `mix64(0x75bcd15) mod 100 = 84`; token expiry 1791914400; `zz(3000)` bytes `f0 2e`; Transcription 40 − 32 = 8 < 16.
- Tables: every row has its header's column count.
- Exports: 155 IDs (`svc.err.insufficient_transcription` added), per-prefix counts match, no `fnv1a32` collision among them or with the 5,441 distinct ID-shaped tokens of the other chapters; every `net.`/`svc.` ID used is exported. Non-ID interfaces add `LobbySettings`, `LobbyPick` and the moved schema path.
- **Economy schema fixed**: §11.4 claimed "Economy error IDs map doc 15's codes one to one" but lacked doc 15's `insufficient_transcription`, and had no wire form for doc 15 §3.9.1 Transcription claims or for the `folio_exits` and `transcription` counters doc 15 §4.6 puts in `pity` receipts. Added `POST /v1/econ/archive/transcribe`, `transcription` in `GET /v1/econ/wallet`, the two receipt fields, the error ID, test 66 and the §11.1 idem-key sentence.
- **Schema path fixed** (doc 19 Interface row 16): `content/16/schema/econ_api.json` is not content (`schema` is no Canon §19 namespace); moved to `palimpsest/schemas/svc/econ_api.json` in §11.4, Exports and test 50.
- Interface assumptions now hold only open items, rechecked on disk: rows 04, 05, 10, 11, 14 are met (doc 04 §11.5 predicates and `revive_bp`; doc 05 exports `spread_offset` and the recoil step; doc 10 §2.8 `ItemView`; doc 11 §14.4 and §17; doc 14 `RunState.rooms_visited`/`map_revealed`, §8.8), and doc 03 met both remaining requests during this pass (SPEC comment lists `status_speed_bp, n_status`; `catchup` covers the slot-4 invader). Remaining: 02 (DECISIONS 112 removals, Workshop), 09 (step lifetime), 12 and 13 (`net_always`), 15 (idem key `s`, receipt field names), 17, 18.
- Canon gaps: none open; O1 and O2 are the only open items. Header: "Conforms to canon-1.11".

### Requests for other chapters added by this pass

- **01**: drop Epic crossplay (Imports row for doc 16, PAR-03, §9.2, Q-11) per DECISIONS 112; drop the `ChooseExit` intent from the doc 16 import (exits are votes by position, doc 16 §9.8).
- **02**: remove the EOS SDK row, feature `eos`, the EOS backend of `crate.platform`, §14's Epic Games Store bullet and `flag.net.crossplay`; the `opus` row's reason no longer names Epic players; §13.2's Workshop distribution goes through Steam Workshop, not `svc.workshop`; Imports row 12 and Interface row 16 drop "EOS".
- **03**: none.
- **09**: give `StealthHud.steps` (§8.3 footstep indicators) a 30-tick lifetime like rings.
- **12**, **13**: the `net_always` template flag (exit doors; Editor bosses during `BossFight` and hunters).
- **14**: confirm whether a Custom Erratum's appearance preset, palette and marking colors (doc 04 §2, DECISIONS 97) live in `ErratumChoice`, so `LobbyPick`, `JoinCharacter` and the Folio-entry state carry them.
- **15**: §10.3's `s` for Archive purchases and Transcription claims is the request's `req_id` in hex; §4.6 names the `pity` receipt fields `folio_exits` and `transcription`; §7 drops the Epic Games Store pricing mention (DECISIONS 112).
- **17**: draw stalled/migration overlays, stale entities, kick vote, lobby browser, per-member voice unmute, the cosmetic-difference notice and the `STEALTH` caps; no crossplay setting or friend codes exist.
- **18**: `loc.` keys for every `net.reject.*` and `svc.err.*` ID, including `svc.err.insufficient_transcription`.
- **21**: DECISIONS 112: drop the Epic Games Store and EOS (platform rows, WP-069, WP-101's Epic build, M5 criterion 3's Epic client over `svc.relay`, R-06, R-12's Epic review, RQ-07, G12, G16, the doc 16 Interface row, invariant) and the Workshop mirror (WP-123); `svc.relay` and `svc.workshop` are reserved and run nothing.

### Not settled

- LAN and direct-IP host migration: §9.3 sends `MigrateAnnounce` only as a Steam lobby chat message, and step 2's "its LAN address from `PartyState`" names a field `SlotSummary` does not have. This gap predates the pass (the removed `svc.lobby` event never covered direct sessions either).
- A falling block's custom mask (doc 03 §13.6) is not replicated with its entity, so a chipped block renders whole while it falls; presentation only.

Lint after verification: `== 16  words=28297` with no findings; `--json` `multiply_defined` empty.
