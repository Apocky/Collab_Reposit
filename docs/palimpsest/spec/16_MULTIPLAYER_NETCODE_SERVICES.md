# 16 — Multiplayer, Netcode and Online Services

As of 2026-10-12. Status: BINDING. Conforms to canon-1.11. Applies DECISIONS.md through row 145.

This chapter defines how 1 to 4 players share one authoritative simulation: the three topologies, transports and NAT traversal, the wire protocol with exact packet layouts, snapshot replication with delta compression, quantization, interest management and priorities, voxel and field replication, client prediction and reconciliation, lag compensation, the co-op rules this chapter owns (`st.downed`, `st.ghost`, revive, trading, host migration, drop-in), replays, daily-score verification, desync diagnostics, and every backend service with its API. Every rule below was checked against canon-1.11, including §1.3 (checked shifts, non-negating `ceil_div`), §2.2 (named state boundaries), §2.4 (intents in the replay log, `presentation_hash`, `run_seed` from the run header), §2.5 (length-prefixed multi-field hashing; solo saves deferred during an invasion), §3.6 (strict seed and date decoding), §4.3 (fixed terrain, placeables), §11.1 and §11.8 (downed rules owned here, substituted commands), §14 (Lucre between party members only as dropped piles) and §17.9 (invasions, Redaction while downed). Canon-1.8 cut mounts and canon-1.10 removed item durability, so neither appears in any rule below.

## Imports

| Source | Section | What this chapter consumes |
|---|---|---|
| DESIGN_BRIEF | §5, §8 | host-authoritative co-op with prediction, reconciliation and lag-compensated hits; deterministic input logs for replays and daily verification; offline-complete end-of-service mode |
| DECISIONS | 2026-10-11 rows 15, 16, 48, 73–76, 81, 83, 86, 87, 93, 96, 97, 98, 104, 107, 112, 123, 126, 129, 138, 139, 140, 144 | opt-in PvP invasions where a player invades as an Editor (15); limited building, so only placeables and destructible-tagged blocks change (16); shareable deterministic replays (48); Steam P2P over Datagram Relay and a headless host for verification (73; its EOS build for Epic crossplay is withdrawn by 112); friends plus a public lobby browser with mute, block, report and kick votes (74); pings, text and push-to-talk voice, voice with strangers off by default (75); host migration from the Folio-entry snapshot (76), refined so hot and warm migrations continue the Folio and only a cold one restores that snapshot (93); downed members and ghosts share kill XP (81); Lucre between players only as dropped piles inside one run (83); cosmetic-only invader rewards (86); peers may differ in presentation content (87); PC only through 1.0: Windows and the Steam Deck through Proton, no console port (98); Steam the only storefront, with no Epic Games Store build and no EOS (112); every slot enters Second Edition and a ghost returns there as it was at death (107); opt-in asynchronous traces (96) and opt-in streaming audience votes (104), specified in §11.10 and §9.14; light character presets whose `Appearance` is presentation data (97); Steam Networking Sockets for play and our own verifier re-simulating ranked, Daily and Weekly replays, with the EOS SDK dropped (123); Steam Workshop with co-op peers auto-syncing the host's required mod set (126); replays without a photo mode (129); Project Fluent formatting of generated text in each client's own locale (138); balance changes only in patches, never pushed by a server (139); opt-in per-run aggregate telemetry, never from minor-flagged accounts, never free text (140); 3 account profiles per Steam account, each with one suspended run, synced through Steam Cloud (144) |
| Canon | §1.1, §1.3, §1.6 | ID grammar and `id32`; `floor_div`, non-negating `ceil_div`, checked shifts (`>>` is floor division by `2^k`), `mul_bp`; integer-only content, `u64` as hex strings |
| Canon | §1.8, §1.9, §1.11 | Folio box 256 × 64 × 256, chunk slot math; `Bam`, pitch clamp ±16000, `ang_diff`, `yaw_from_dir`; `EntityId` layout, ascending-ID iteration |
| Canon | §2.1–§2.5 | 60 Hz tick, no extrapolation except local prediction; named state boundaries and the tick schedule row "Network snapshot, period 2, phase 0" (boundary `t + 1` for every even `t`); session header with `content_hash`, `presentation_hash` (a mismatch only warns) and `run_seed` delivered in the run header at commit; replay command record with `view_tick` and the applied-intent record; save contract; `state_hash` at every boundary `b` with `b % 60 == 0` and `b >= 60`; solo saves deferred while an invader holds slot 4; co-op saves only at Folio entry |
| Canon | §2.5, §3.2, §3.3, §3.6 | length-prefixed multi-field hashing; sequential and keyed streams (keyed draws make late inputs safe); seed display; daily and weekly seeds with strict date validation |
| Canon | §4.1, §4.2 | humanoid capsule, speeds, dodge |
| Canon | §9 | baked pose tables; 32 ticks of pose history for lag compensation |
| Canon | §8, §10.8, §11.1, §11.8 | co-op kill XP shared by every member present, downed members and ghosts included, Dormant and Departed slots excluded; lethal resolution steps 2–3; `st.downed` (owner 16: 30 s bleed-out for the first down in a Folio, later downs, damage shortening and revive HP per §9.4; revive in 3 s), `st.ghost` (owner 16); the missing-command substitute read as the absence of input |
| Canon | §13.6, §13.9, §14 | party-shared type knowledge; `finder_slot`; no currency bought, or traded or gifted between accounts; Lucre per Erratum, passing between party members only as dropped piles |
| Canon | §4.3 | terrain fixed except blocks tagged `block.tag.destructible` and `cat.placeable` items |
| Canon | §17.1, §17.2, §17.5, §17.6 | `PARTY_MAX` 4, loot instanced per player; party-size tables; `FF_DIRECT_BP`, `FF_AREA_BP`, `mode.ff_off`, `mode.ff_full`, `mode.loot_shared`; ghosts and resurrection |
| Canon | §17.9, §2.4 | invasions: `mode.invasion_open` (off by default, forbidden in Daily, Weekly and ranked runs), Redactor invader as `rank.hunter` with an account Editor kit, window (Folio 3 onward, one per Folio, no boss Folios, not before 120 s, not after a Colophon entry, at most 300 s), invader limits, the Redacted outcome when the invader drops the last Erratum that is alive and not downed, replay slot 4, mid-Folio join |
| Canon | §18, §19 | `ev.player_downed`, `ev.player_revived`, `ev.player_died`, `ev.player_ghosted`, `ev.player_resurrected`, `ev.run_ended`, `ev.invasion_started`, `ev.invasion_ended`, every event replicated for presentation; doc 03 owns the bus and registers every further `ev.` ID with its payload; namespaces `net.` and `svc.` belong to this chapter |
| Doc 02 | §2.2, §3.2–§3.4 | crates `renet` 2.0.0, `steamworks` 0.13.1, `postcard` 1.1.3, `zstd`, `blake3`, `ed25519-dalek` 3.0.0, `axum` 0.8.9, `tokio` 1.53.2, `sqlx` 0.9.0, PostgreSQL 18; `crate.net`, `crate.server`, `crate.services`; the `Transport` sketch |
| Doc 02 | §5.4, §5.5 | `Sim::step`, `snapshot`, `restore`, `serialize`, `section_hashes`; `PlayerCommand`, button bits, `PlayerIntent`, `IntentKind` |
| Doc 02 | §6.5–§6.7, §12.1, §13.4, §15, §17.3, §17.4 | 13 serialization sections (0 to 12, section 12 = stealth) and section hashes; golden replays; desync dump; `PALS` container kinds; `modset_hash`; `perf.deck.net.host_kbps` = 384 kbit/s for 3 clients; `perf.ci.bot_tps` ≥ 6,000 ticks per second; Deck sim mean tick 4.33 ms; flags `flag.net.transport`, `flag.net.input_redundancy`, `flag.net.shadow`, `flag.net.voice`, `flag.net.netsim`; telemetry consent rules |
| Doc 03 | §2.2, §3.2–§3.4, §4.3, §4.4, §4.8, §5, §5.16, §5.19, §5.24, §10, §11.5, §13, §14.2, §15.1, §16 | system order (`sys.input_apply`, `sys.hitscan`, `sys.melee`, `sys.coop`, `sys.tick_end`); entity kinds (the row index is the replicated kind code; code 2 is retired), `comp.player` (semantics owned here except doc 17's Mercy and breath fields), `Kind.flags`, `Pose.hist`; the pure controller `step_mover` with `MoverParams` and its `status_speed_bp` list; pose-history rewind; projectile catch-up steps, then the normal step on the spawn tick; light channels and light overrides; `ev.noise` with 5 `heard` entries; `set_block`, `set_block_mask`, `ev.block_changed`, `ev.block_chipped` and the per-chunk custom micro-voxel masks; event payloads, including `from = 3` and `cause = 1` on traded items' events; 5 × 5 chunk activation window; save coverage; `MoveState` with the slide and grapple fields and `MoveMode` codes 8 `slide` and 9 `grapple`; `start_grapple` as a host step outside `step_mover`; `Climate { tod, wx }` in section 0 (§4.8, §16.1); `ev.law_discovered { law, slots }` (§14.3); falling blocks (§13.6) |
| Doc 04 | §6.2, Restoration and Warden talents | downed members and ghosts share kill XP; instant revive at Restoration 100; Warden `resurgence` revive time × 5000 |
| Doc 05 | §2.6, §14.3 | `LAGCOMP_MAX_TICKS = 16`, target state read at the present tick, projectile catch-up steps; downed Errata keep capsules and cannot attack, block or use items; bleed-out shortening `ceil_div(total_mhp × 1800, max_hp_mhp)` |
| Doc 11 | §2.2, §14.4, §17 | item `visibility` `Instanced(slot)` or `Public`; the `keepsake` field on item instances, which `DropItemFor` and trades refuse; ground items destroyed at Folio exit; per-slot rolls with `seq = slot`; Lucre between party members only as Public `DropLucre` piles |
| Doc 06 | §8.3 | `item.text.mapping` reveals every room outline, door and exit of the current Folio to the party |
| Doc 09 | §8, §8.3, §8.4, §14.5 | `StealthHud` for each player's own Erratum, with each footstep indicator's 30-tick lifetime from the tick the step was heard, newest first (§8.3), and the `panel: Option<StealthPanel>` with `HackLine` and `StealLine` (§8.4); no doc 09 interest filter: §6.6 rule 2 alone (doc 09 §14.5, P1) |
| Doc 17 | §7, §11.6, §12.2, §13 | command fields `ACTION` (bits 26–28, values 0 to 5) and `WHEEL_ENTRY` (bits 29–31), bit 25 always 0; ping kinds; `access.game_speed_bp`; `mercy_on`, `mercy_bp` and `breath_held` in `comp.player`; the Current Folio page and World Law visibility rebuilt from `RunView` (§7); streamer-safe mode for traces and votes (§11.6) |
| Doc 01 | §10.3 | the per-Folio KPI aggregates of the opt-in run summary and the systemic-kill rule |
| Doc 12 | §4.1, §6.1, §20.4–§20.7 | `laws_triggered` slot masks and the Law discovery rule; `FolioBlueprint`, `blueprint_hash` compared by every co-op peer; the Folio quest (distinct story events read, `story_solved`, `item.quest.folio_leaf`) |
| Doc 13 | target scoring, §10.5, §10.7 | downed candidates × 2500 (doc 05 §14.1), ghosts never perceivable; template `enemy.editors.invader` (scaled by the party's `n`), no AI think, entry at the §10.5 hunter placement anchor; invader kits `enemy.editors.kit.*` |
| Doc 10 | §2.2, §2.5, §2.8 | party-shared type knowledge; knowledge travels with traded items; host sends only `ItemView` |
| Doc 14 | §1.1, §1.2, §1.3, §2.1, §2.5, §3.2, §3.3, §3.6, §3.10, §4.3, §7, §8.4, §8.8, §9.4, §12 | lobby state machine (a lobby whose host leaves disbands); joiner rules; `enter_folio`; phase row 1 (no Active Erratum alive and not downed ends the run, downed ones die first); exit commit rule and constants; ghost restoration HP (`GHOST_REMAINS_HP_BP` 5000, `GHOST_BARE_HP_BP` 2500); `SlotStatus`; Grades 0 to 20; `Appearance` and display names as presentation data (§1.3); audience votes and asynchronous traces (§8.1); Dormant and Departed slots, `DORMANT_GRACE_S`; ranked Daily attempts; boards, submission JSON and `verify`; `RankedState` with a 32-byte `ranked_token`; `RANKED_RESTORE_MAX` = 2; Folio-entry save copies for host migration |
| Doc 15 | §2.2–§2.4, §3.4, §3.9.1, §3.13, §4, §5.3, §6.2, §6.5, §7, §8.2, §8.4, §8.5, §10, §11 | mode rules and caps; invader Quires (`INVASION_QUIRES` 5, `INVASION_REDACT_QUIRES` 5, `INVASION_MIN_TICKS` 3,600, 60 per 24 h); pull and Reliquary algorithms and records; the `transcription` counter and Transcription claims; commit-reveal epochs, Sealed Drafts, signed receipts and their canonical hex forms; Codex XP and switching; store, purchase flow and refunds; `juris.flag.invasion_minor_default`, `juris.flag.minor_chat_default`; minor spend caps; chat safety including invader rules; economy service components, ledger, idem keys (§10.3), the run report schema and `run_id` from the host's OS CSPRNG (§10.4); end-of-service mode |
| Doc 18 | §5.6, §9.6 | `Appearance { preset, base, marking }`; the host's validated Scribe text carried by `ScribeText`; the marginal-note grammar used by traces |
| Doc 02 | §10, §12.2, §13.4 | Fluent message formatting; Steam Auto-Cloud of `profile/` and `runs/`; Workshop mod set and data-pack transfer |
| R4 | §6.1–§6.4 | snapshot model, input redundancy 8, 30 Hz snapshots, 96 kbit/s snapshot budget, SDR facts (IP privacy, P2P), renet for LAN, interpolation delay 100 ms default |
| R3 | C4 | COMMS-1, LOG-1, EOL-1 requirements |
| R7 | §4, §11 R-CO3, R-CO4 | revive health falling with difficulty; instanced currency pickups |

---

## 1. Decisions and constants

### 1.1 Decisions

1. **One authority.** Exactly one process runs the authoritative `Sim` for a session: the host. Every other process sends `PlayerCommand`s and `PlayerIntent`s and receives replicated state.
2. **Thin clients, snapshot replication.** Clients render remote state from 30 Hz delta-compressed snapshots and predict only their own mover. They regenerate each Folio's terrain locally from the seed and receive block edits as an ordered diff log, so terrain never streams in bulk.
3. **Every client keeps the Confirmed Input Log (CIL)** of the inputs the host applied, plus the latest Folio-entry state. This gives every client a complete replay, makes any client able to become host, and feeds the daily verifier.
4. **Shadow replicas are a configurable option** (`flag.net.shadow`, default auto: on with at least 6 physical cores and not handheld). A Shadow is a full `Sim` replica on a client that consumes the CIL. Shadows detect desyncs live and make host migration instant.
5. **Favor the shooter** for Errata and invader attacks, inside doc 05's rewind cap of 16 ticks (`LAGCOMP_MAX_TICKS`, doc 05 §2.6).
6. **Platform P2P first.** Every online session connects through Steam Datagram Relay (§3.5), so no player IP address is revealed and no game servers are rented; Steam is the only storefront (DECISIONS 112). The dedicated server binary exists for self-hosting and for headless verification, not for live matchmaking.
7. **Offline-complete.** Every gameplay feature, including co-op over LAN and direct IP, runs without the backend (DESIGN_BRIEF §8). Invasions need `svc.lobby` beacons and tickets and are the one online-only gameplay feature; ranked submission is a service, not gameplay.
8. **Opt-in invasions** (DESIGN_BRIEF §1, DECISIONS #15). A run whose host enables invasions accepts one invading player at a time from another session, who plays an Editor in slot 4. Canon §17.9 fixes the rules; this chapter owns the session, matchmaking, visibility and transport details, doc 13 the invader's body, doc 15 its rewards.
9. **Party trust, invader isolation.** Every party member holds the run seed and the CIL, so a modified party client can derive anything the simulation knows. Inside a party, interest management and instanced replication are bandwidth and presentation measures, not secrecy; co-op cheating harms only that party's own game. The invader is the only adversary: it never receives the CIL, and while it is present party clients never receive its commands (§6.14 sealing), so PvP culling (§6.6) holds against both sides.

### 1.2 Constants

| Constant | Value | Meaning |
|---|---|---|
| `NET_PROTO_VERSION` | 1 | wire protocol version, independent of `SIM_VERSION` |
| `SNAP_PERIOD` | 2 ticks | Canon §2.2; snapshot boundaries `b = t + 1` for every even `t` |
| `INPUT_REDUNDANCY` | 8 | commands per input packet (`flag.net.input_redundancy`, doc 02) |
| `INPUT_BUFFER_SPAN` | 64 ticks | host keeps commands for ticks `next ..< next + 64` |
| `SLACK_TARGET_MIN`, `SLACK_TARGET_MAX` | 2, 8 ticks | input arrival lead the client aims for |
| `INTERP_MIN`, `INTERP_MAX` | 4, 12 ticks | remote-entity interpolation delay range |
| `PRED_RING` | 128 ticks | prediction history |
| `PRED_STALL_TICKS` | 120 | prediction stops after this many ticks without an authoritative mover state |
| `LAGCOMP_MAX_REWIND` | 16 ticks | equals doc 05 `LAGCOMP_MAX_TICKS`; the pose history holds 32 (Canon §9) |
| `LAGCOMP_SLOP` | 4 ticks | added to the measured lag before clamping `view_tick` |
| `SNAP_RING` | 32 snapshots | baselines kept by host and client (64 ticks) |
| `KEYFRAME_GAP` | 64 ticks | a snapshot whose boundary is at least this far past the acknowledged baseline is replaced by a keyframe (§6.5) |
| `NET_HANDLE_BITS` | 13 | per-client entity handles `0..=8190`; 8191 means none |
| `MTU_PAYLOAD` | 1,200 bytes | largest unreliable message; snapshots stop at 1,150 |
| `RELIABLE_MSG_MAX` | 262,144 bytes | larger payloads are split into 65,536-byte parts |
| `DATAGRAM_OVERHEAD` | 48 bytes | fixed transport overhead counted per datagram in every budget |
| `CLIENT_DOWN_BUDGET` | 16,000 B/s (128 kbit/s) | per client, all channels, overhead included; on a listen host `min(16000, floor_div(48000, remote_peers))` |
| `HOST_UP_BUDGET` | 48,000 B/s (384 kbit/s) | listen host total; dedicated servers have no total cap |
| `SNAP_RATE` | `CLIENT_DOWN_BUDGET − 5,440` = 10,560 B/s payload by default | snapshot token refill (4,000 B/s of reliable channels and 1,440 B/s of snapshot datagram overhead removed); 352 B per snapshot; lowered by `CIL_BACKLOG_PLAY_RATE` while that client's CIL backlog streams in game (§6.14) |
| `SNAP_BUCKET` | `6 × floor_div(SNAP_RATE, 30)` = 2,112 B by default | snapshot token capacity |
| `CIL_BATCH_TICKS` | 6 | CIL message cadence (10 Hz) |
| `CIL_BACKLOG_WAIT_RATE`, `CIL_BACKLOG_PLAY_RATE` | 8,000 B/s, 1,000 B/s | CIL backlog stream while the client is not in game, and while it is (§6.14) |
| `PARTY_STATE_PERIOD` | 30 ticks | `PartyState` cadence, 2 Hz (§6.10) |
| `NET_STALL_MS` | 1,000 ms | silence before the "connection problem" overlay |
| `NET_TIMEOUT_MS` | 5,000 ms | silence before a peer is declared lost |
| `MIGRATE_ANNOUNCE_MS` | 10,000 ms | per successor rank before the next successor tries |
| `MIGRATE_WARM_MAX_MS` | 45,000 ms | estimated re-simulation time above which migration is cold |
| `MIGRATE_PROBE_TICKS` | 600 | re-simulation probe after each Folio entry that measures `measured_tick_us` (§9.3) |
| `MIGRATE_TICK_US_DEFAULT` | 4,000 µs | `measured_tick_us` before the first probe completes |
| `JOIN_WAIT_MAX` | none | a joiner waits for the next Folio entry however long it takes |
| `DEFAULT_PORT` | UDP 27310 | renet direct and dedicated |
| `INVADER_SLOT` | 4 | the slot index an invader's commands use |
| `INVASION_MAX_TICKS` | 18,000 (300 s) | longest invasion (Canon §17.9) |
| `INVASION_FIRST_FOLIO`, `INVASION_ENTRY_DELAY` | Folio 3, 7,200 ticks | earliest Folio and earliest tick after entry (Canon §17.9) |
| `PVS_PERIOD` | 6 ticks | visibility culling cadence for PvP pairs (§6.6) |
| `PVS_GRACE_TICKS` | 30 | an unseen hostile player stays replicated this long |
| `BEACON_REFRESH_S`, `BEACON_TTL_S` | 30 s, 45 s | invasion beacon refresh period and expiry at `svc.lobby` (§9.13, §11.5) |
| `VOICE_STREAMS_MAX` | 2 | voice talkers forwarded to one receiver at once (§9.12) |
| `VOICE_QUEUE_MAX` | 8,192 bytes | a peer's transport send queue above which voice frames to it are dropped |
| `KICK_VOTE_TICKS` | 1,800 | kick vote duration (§9.2) |
| `REVIVE_HP_BASE_BP`, `REVIVE_HP_STEP_BP`, `REVIVE_HP_MIN_BP` | 5000, 200, 1000 | revive HP curve over Grade and repeated downs (§9.4) |
| `TRADE_CONFIRM_TICKS` | 1,800 | longest wait in `both_locked` for both confirmations (§9.7) |

Every constant of the table above and every co-op tuning value of §9 lives in the content record `net.params.default` (`content/16/net/params.json`), loaded with the content set and covered by `content_hash`. Service-side limits (§11: rate limits, token lifetimes, retention) live in the service configuration instead, because they never touch a simulation or a session:

```json
{ "id": "net.params.default", "proto": 1, "input_redundancy": 8, "input_buffer_span": 64,
  "slack_target_min": 2, "slack_target_max": 8, "interp_min": 4, "interp_max": 12,
  "pred_ring": 128, "pred_stall_ticks": 120, "lagcomp_max_rewind": 16, "lagcomp_slop": 4,
  "snap_ring": 32, "keyframe_gap": 64, "handle_bits": 13, "mtu_payload": 1200, "snap_payload_max": 1150,
  "datagram_overhead": 48, "client_down_budget": 16000, "host_up_budget": 48000, "reliable_share": 4000,
  "cil_batch_ticks": 6, "cil_backlog_wait_rate": 8000, "cil_backlog_play_rate": 1000, "party_state_period": 30,
  "stall_ms": 1000, "timeout_ms": 5000, "migrate_announce_ms": 10000,
  "migrate_warm_max_ms": 45000, "migrate_probe_ticks": 600, "migrate_tick_us_default": 4000,
  "default_port": 27310, "invader_slot": 4, "invasion_max_ticks": 18000, "invasion_first_folio": 3, "invasion_entry_delay": 7200,
  "pvs_period": 6, "pvs_grace_ticks": 30, "pvs_near_mm": 16000, "beacon_refresh_s": 30, "beacon_ttl_s": 45,
  "bleed_ticks": [1800, 1200, 600], "revive_base_ticks": 180, "revive_min_ticks": 30,
  "revive_range_mm": 1500, "revive_cone_bam": 8192, "bleed_damage_ticks_per_maxhp": 1800,
  "revive_hp_base_bp": 5000, "revive_hp_step_bp": 200, "revive_hp_min_bp": 1000, "revive_hp_halvings_max": 3,
  "reliable_msg_max": 262144, "voice_streams_max": 2, "voice_queue_max": 8192, "kick_vote_ticks": 1800,
  "trade_range_mm": 4000, "trade_break_mm": 6000, "trade_max_items": 8, "trade_accept_ticks": 600,
  "trade_confirm_ticks": 1800, "ghost_mark_range_mm": 30000, "voice_full_mm": 5000, "voice_zero_mm": 30000,
  "chat_max_bytes": 200, "chat_burst": 5, "chat_window_ticks": 600, "ping_danger_ticks": 15 }
```

---

## 2. Topologies

### 2.1 Roles

| Role | Runs | Holds | Sends |
|---|---|---|---|
| Host | the authoritative `Sim` at real time | full state, every client's input buffer, per-client replication views | snapshots, keyframes, world edits, CIL, owner, party and run state, hash reports |
| Thin client | prediction of its own mover, interpolation, presentation grids | client world (blocks, presentation fields), entity views, CIL, latest Folio-entry state | commands, intents, acks |
| Shadow | a full `Sim` replica one CIL batch behind the host | full state | nothing extra; compares hash reports |
| Verifier | a headless `Sim` re-simulating a replay | full state | a verdict |

### 2.2 Solo in-process (`net.topo.solo`)

Single player is a host with zero remote peers. The client process runs `HostSession` on the sim thread (doc 02 §7.1). The local slot's commands enter through `LocalInputPort`, a lock-free queue that bypasses serialization; no snapshot is built for the local player, whose renderer reads the `ViewMailbox` directly. The local player is not predicted: its camera position interpolates (doc 02 §7.3) and its look comes from current input, so position latency is one tick plus interpolation. Solo saves at any tick boundary, except while an invader holds replay slot 4: a save request then completes on the tick `ev.invasion_ended` is emitted (Canon §2.5). Solo pauses when a menu opens or a wake gap exceeds 1 s (doc 02 §14), except while an invader is present: the session then has a remote peer and follows §2.3, so it never pauses. Solo has no downed state (Canon §10.8 step 2 needs another member alive and not downed).

### 2.3 Listen host (`net.topo.listen`)

One player's client is host and runs the solo arrangement plus up to 3 remote thin clients and, in an invasion, one invader (§9.13). The host player's own slot uses `LocalInputPort` and is never predicted, so the host plays at zero network latency. A listen-host session never pauses, and while it has a remote peer, an invader included, game speed (doc 17 `access.game_speed_bp`) is locked to 10000. The host's total upstream is capped at `HOST_UP_BUDGET` = 48,000 B/s (384 kbit/s, doc 02 `perf.deck.net.host_kbps`), divided among its remote peers as §15 states. Relayed voice is outside this cap and is dropped first under congestion (§9.12).

### 2.4 Dedicated headless server (`net.topo.dedicated`)

`palimpsest_server host` (crate `crate.server`) runs `HostSession` with no local slot and no rendering, and accepts up to 4 remote party clients plus one invader. Lobby decisions that doc 14 gives to "the host" are made by the **party leader**: the configured admin account, else the first client to connect, passed on in slot order when the leader leaves.

```
palimpsest_server host --port 27310 --max-players 4 --password-file pw.txt --save-dir ./runs \
                  --admin 0x0123456789abcdef --content ./content --mods ./mods
palimpsest_server verify --replay run.palr --report verdict.json
palimpsest_server replay --replay run.palr --hash-every-tick --until 36000
```

Dedicated servers use `renet` UDP (§3.4) at launch. Their sim thread uses the same 60 Hz accumulator as the client (doc 02 §7.1). A dedicated server writes the co-op Folio-entry save itself and needs no host migration while it runs; when its process ends, the clients migrate exactly as from a listen host (§9.3), and the successor becomes a listen host.

### 2.5 Threads on a networked peer

| Thread | Host | Thin client |
|---|---|---|
| sim | `Sim::step` at 60 Hz; `ReplFrame` extraction on snapshot ticks (budget 0.40 ms, doc 02 §15) | Shadow `Sim` when enabled |
| net | transport pump, per-client encoding, input buffers, CIL batching | transport pump, decoding, prediction, interpolation buffers |
| main / render | Bevy (listen host only) | Bevy |
| worker | hashing (doc 02 §6.5), zstd of Folio-entry state, CIL backlog compression | Folio generation, section-hash verification, the migration probe and CIL replay for warm migration (§9.3) |

---

## 3. Transport, NAT traversal and identity

### 3.1 Transport trait

Doc 02 sketches the trait; this is its full contract, implemented in `palimpsest_net` and `palimpsest_platform`:

```rust
pub type PeerId = u32;                               // transport-local, never on the wire
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Channel { Control = 0, Input = 1, Snapshot = 2, World = 3, Cil = 4, Voice = 5 }
pub enum TransportEvent { Connected(PeerId, PeerIdentity), Disconnected(PeerId, DisconnectReason) }
pub trait Transport: Send {
    fn listen(&mut self, cfg: &ListenCfg) -> Result<(), NetError>;
    fn connect(&mut self, target: &ConnectTarget) -> Result<PeerId, NetError>;
    fn send(&mut self, peer: PeerId, ch: Channel, bytes: &[u8]);      // never blocks; drops when the queue is full
    fn recv(&mut self) -> Option<(PeerId, Channel, Vec<u8>)>;
    fn poll_event(&mut self) -> Option<TransportEvent>;
    fn disconnect(&mut self, peer: PeerId, reason: DisconnectReason);
    fn stats(&self, peer: PeerId) -> LinkStats;   // rtt_us, loss_bp, send_queue_bytes, out_bps, in_bps
    fn flush(&mut self);                           // called once per net-thread wake
}
```

Implementations and their routes:

| Route ID | Transport | NAT traversal | IP privacy | Used for |
|---|---|---|---|---|
| `net.route.sdr` | Steam `ISteamNetworkingSockets` P2P (`CreateListenSocketP2P`, `ConnectP2P` by SteamID, virtual port 0) through Steam Datagram Relay | relay, handled by Steam | yes | every Steam session (`flag.net.transport = 0`) |
| `net.route.direct` | `renet` 2.0.0 over UDP, port 27310 | none: LAN, or a forwarded port | no | LAN, direct IP, dedicated servers (`flag.net.transport = 1`) |
| `net.route.relay` | none: retired before shipping (§3.5, DECISIONS 112) | — | — | no session; the ID stays reserved |
| `net.route.in_process` | `crossbeam-channel` pairs | none | n/a | tests, bots, CI (`flag.net.transport = 2`) |

### 3.2 Channels and lanes

| ID | Channel | Delivery | Direction | Lane priority (0 highest), weight | Contents |
|---|---|---|---|---|---|
| `net.ch.control` | 0 | reliable, ordered | both | 1, 1 | handshake, intents, owner, party and run state, hash reports, chat, session |
| `net.ch.input` | 1 | unreliable | client → host | 0, 1 | input packets |
| `net.ch.snapshot` | 2 | unreliable | host → client | 0, 1 | snapshots |
| `net.ch.world` | 3 | reliable, ordered | host → client | 2, 3 | world edits, keyframes, Folio-entry state, chunk data, full state |
| `net.ch.cil` | 4 | reliable, ordered | host → client | 2, 1 | Confirmed Input Log, backlog and unseal messages (§6.14) |
| `net.ch.voice` | 5 | unreliable | both | 3, 1 | voice frames; lowest priority, dropped first (§9.12) |

On Steam each channel is one lane configured with `ConfigureConnectionLanes` using the priorities and weights above, and messages are sent with `k_nSteamNetworkingSend_Reliable` or `k_nSteamNetworkingSend_UnreliableNoNagle`. On renet each channel is one renet channel: `ReliableOrdered` for 0, 3, 4 and `Unreliable` for 1, 2, 5. The connection send rate is set to minimum 131,072 and maximum 524,288 bytes per second (Steam `SendRateMin`, `SendRateMax`); this chapter's budgets keep the average below 16,000 B/s per client.

**Reliable framing.** Every message on channels 0, 3 and 4 starts with `rseq: u32` (little-endian), counting from 0 per channel per connection. The receiver processes messages strictly in `rseq` order and buffers gaps, so correctness never depends on a transport's ordering guarantee. Payloads over `RELIABLE_MSG_MAX` are split into parts of 65,536 bytes, each carrying `part: u16, parts: u16` after the message type.

### 3.3 Steam (production)

- Lobbies use `ISteamMatchmaking` with 4 members; the host is the lobby owner. Lobby data keys, all ASCII: `pal_ver` (`SIM_VERSION` decimal), `pal_proto`, `pal_ch` (first 16 hex digits of `content_hash`), `pal_mods` (first 16 hex digits of `modset_hash`, doc 02 §13.4; shown in the lobby, never a filter, because presentation-only mods may differ between peers, DECISIONS 87), `pal_mode`, `pal_grade`, `pal_state` (`lobby` or `run`), `pal_folio`, `pal_ff`, `pal_loot`, `pal_join` (`open` or `closed`), `pal_host` (SteamID64), `pal_succ` (comma-separated successor SteamID64s, §9.3), `pal_pub` (`net.lobby.*` suffix), `pal_inv` (`open` or `closed`, §9.13; informational only, because invaders find sessions through `svc.lobby` beacons), `pal_ws` (the Steam Workshop `PublishedFileId`s of the host's required mods as comma-separated decimals in ascending order, at most 32, §11.8), `pal_aud` (`on` or `off`, §9.14). Member data: `pal_ploc` (the member's Steam ping location).
- Lobby types: `net.lobby.private` (invite only, `k_ELobbyTypePrivate`), `net.lobby.friends` (`k_ELobbyTypeFriendsOnly`), `net.lobby.public` (`k_ELobbyTypePublic`). Default: friends.
- **Quick Join** requests the lobby list filtered by equal `pal_ver`, `pal_proto`, `pal_ch`, `pal_mode`, `pal_join = open`, `|pal_grade − own max grade| <= 2`, and the default distance filter; it then estimates ping from each lobby's host ping location (`ISteamNetworkingUtils` ping locations exchanged as lobby member data `pal_ploc`) and joins the lowest estimate under 150 ms, else creates a public lobby.
- **Lobby browser** (DECISIONS 74). The player's friends' joinable lobbies come first, from the Steam friends list; below them the browser lists `net.lobby.public` lobbies with equal `pal_ver`, `pal_proto`, `pal_ch` and `pal_join = open`, plus the `pal_mode` and `pal_grade` filters the player sets, sorted by ascending ping estimate (ties to the lower `pal_host`), 50 per page. Each row shows mode, Grade, member count, `pal_state`, `pal_folio` and the estimate. Lobbies hosted by or holding a blocked player are hidden (§3.6). Mute, block and report are on every member row, and kick votes follow §9.2.
- Identity on the wire is the SteamID64; the host authenticates each client with `BeginAuthSession` on a ticket from `GetAuthSessionTicket`, and refuses with `net.reject.auth_failed` on any result other than OK.

### 3.4 Direct and LAN

- The host listens on UDP 27310 (configurable). LAN discovery: the host broadcasts a 32-byte beacon `PALB ‖ proto u16 ‖ sim_version u32 ‖ content_hash[0..8] ‖ port u16 ‖ party u8 ‖ max u8 ‖ mode id32 ‖ 6 zero bytes` to UDP 27311 once per second; clients list beacons seen in the last 3 s.
- renet runs in unsecure mode on LAN and direct IP; the host password (a lobby setting, default empty) is verified inside `Hello` as `BLAKE3("PALIMPSEST/PW/1" ‖ host_nonce ‖ le32(len(password)) ‖ password)` (the variable-length field carries Canon §2.5's length prefix), where `host_nonce` is the 16 random bytes of the host's `Challenge` message (§3.8), drawn from the host's OS CSPRNG per connection.
- No hole punching is attempted. A direct connection that has not completed its handshake after 5,000 ms fails with a message telling the host to forward UDP 27310 or use a Steam session.

### 3.5 Route choice

Every online session is a Steam lobby (§3.3) on `net.route.sdr`, because Steam is the only storefront (DECISIONS 112) and no other platform ships through 1.0 (DECISIONS 98); the route is fixed when the lobby is created. Invaders connect through SDR as well (§9.13). LAN and direct IP sessions use `net.route.direct` (§3.4); tests, bots and CI use `net.route.in_process`. `net.route.relay` and its service `svc.relay`, which carried sessions with Epic Games Store members, are retired before shipping: no session uses the route, no relay node runs, and both IDs stay reserved (Exports).

### 3.6 Identity

- **Account.** `account_id: u64` from the backend OS CSPRNG at account creation, never derived from a platform ID; it is the `acct` of doc 15 receipts.
- **Platform identity.** A table maps `(platform, platform_user_id)` to one `account_id`; `steam` is the only platform, because Steam is the only storefront and no port ships through 1.0 (DECISIONS 98, 112).
- **Display.** In-session names are the platform persona name, sent in `Hello`, at most 32 UTF-8 bytes after truncation at a character boundary.
- **Blocks.** Steam blocks and account blocks both apply: a blocked player's lobby is hidden, the player cannot join one's lobby, and voice and text from them are muted.

### 3.7 Message envelope

Every transport message is `msg_type: u8` followed by the body; on reliable channels `rseq: u32` comes first, before `msg_type`. Bodies of control messages are the `postcard` encoding (doc 02) of the Rust struct named in the table. Input packets, snapshots, world edits and CIL batches use the hand-written layouts of §5.1, §6.8, §6.11 and §6.14. All fixed-width integers are little-endian. `varint` is unsigned LEB128; `zz` is zigzag (`(n << 1) ^ (n >> 63)`) followed by LEB128. A wire tick is the low 32 bits of a `Tick`; the receiver reconstructs the `u64` nearest to its own current tick:

```rust
fn reconstruct_tick(wire: u32, near: u64) -> u64 {
    let base = near & !0xFFFF_FFFFu64;
    let mut best = base | wire as u64;                          // candidate in near's 2^32 window
    for c in [base.checked_sub(1 << 32), base.checked_add(1 << 32)].into_iter().flatten() {
        let c = c | wire as u64;
        let (dc, db) = (c.abs_diff(near), best.abs_diff(near));
        if dc < db || (dc == db && c < best) { best = c; }      // ties go to the smaller tick
    }
    best
}
```

| Type | ID | Channel | Direction | Body |
|---|---|---|---|---|
| 0x01 | `net.msg.hello` | 0 | C → H | `Hello` |
| 0x02 | `net.msg.welcome` | 0 | H → C | `Welcome` |
| 0x03 | `net.msg.reject` | 0 | H → C | `Reject { reason: Id32 }` |
| 0x04 | `net.msg.ping` | 0 | both | `Ping { sent_us: u64 }` |
| 0x05 | `net.msg.pong` | 0 | both | `Pong { echo_us: u64, host_tick: u64, host_tick_age_us: u32 }` |
| 0x06 | `net.msg.bye` | 0 | both | `Bye { reason: Id32 }` |
| 0x07 | `net.msg.challenge` | 0 | H → C | `Challenge { nonce: [u8; 16] }`, the host's first message on every connection |
| 0x10 | `net.msg.input` | 1 | C → H | §5.1 |
| 0x11 | `net.msg.intent` | 0 | C → H | `IntentMsg { tick: u32, action_seq: u32, kind: IntentKind }` |
| 0x20 | `net.msg.snapshot` | 2 | H → C | §6.8 |
| 0x21 | `net.msg.keyframe` | 3 | H → C | §6.13 |
| 0x22 | `net.msg.world_edits` | 3 | H → C | §6.11 |
| 0x23 | `net.msg.folio_enter` | 0 | H → C | `FolioEnter`, §6.13 |
| 0x24 | `net.msg.folio_state` | 3 | H → C | zstd of serialization sections 0, 1, 2, 3, 8, 9, 10, 11, 12 (doc 02 §6.5) at the Folio-entry boundary |
| 0x25 | `net.msg.chunk_data` | 3 | H → C | `ChunkData { first_slot: u8, sections: zstd bytes }` |
| 0x26 | `net.msg.folio_ready` | 0 | C → H | `FolioReady { folio: u32, gen_ok: bool }` |
| 0x30 | `net.msg.cil` | 4 | H → C | §6.14 |
| 0x31 | `net.msg.hash_report` | 0 | H → C | `HashReport { boundary: u64, hash: [u8; 32] }` |
| 0x32 | `net.msg.section_hash_request` | 0 | C → H | `{ boundary: u64 }` |
| 0x33 | `net.msg.section_hashes` | 0 | H → C | `{ boundary: u64, hashes: [[u8; 32]; 13] }` |
| 0x34 | `net.msg.full_state_request` | 0 | C → H | `{ reason: u8 }` |
| 0x35 | `net.msg.full_state` | 3 | H → C | `{ boundary: u64, part: u16, parts: u16, bytes }` (zstd of the full serialization) |
| 0x36 | `net.msg.cil_unseal` | 4 | H → C | slot-4 records of a finished invasion, §6.14 |
| 0x37 | `net.msg.cil_backlog` | 4 | H → C | `{ first_tick: u64, ticks: u16, zstd bytes }` of missing CIL ticks, §6.14 |
| 0x40 | `net.msg.owner_delta` | 0 | H → C | §6.10 |
| 0x41 | `net.msg.party_state` | 0 | H → C | §6.10 |
| 0x42 | `net.msg.run_state` | 0 | H → C | `RunView`, §6.10 |
| 0x43 | `net.msg.knowledge_delta` | 0 | H → C | §9.9 |
| 0x50 | `net.msg.chat` | 0 | both | `Chat { from_slot: u8, text: String }`, ≤ 200 bytes |
| 0x51 | `net.msg.voice` | 5 | both | `{ from_slot: u8, seq: u16, frame: bytes }` |
| 0x52 | `net.msg.voice_mute` | 0 | C → H | `VoiceMute { muted: u8 }`: bit `s` set means the sender mutes slot `s`'s voice (§9.12) |
| 0x53 | `net.msg.scribe_text` | 0 | H → C | `ScribeText { key: [u8; 32], job: Id32, subject: u64, json: String }`, `json` at most 4,096 bytes (§6.15) |
| 0x60 | `net.msg.join_request` | 0 | C → H | `JoinRequest { character: JoinCharacter }` |
| 0x61 | `net.msg.join_ready` | 0 | H → C | `{ folio: u32, slot: u8 }` |
| 0x62 | `net.msg.migrate_announce` | lobby, or 0 on `net.route.direct` | S → all | `MigrateAnnounce`, §9.3, sent as a Steam lobby chat message, or after `Challenge` on a direct connection |
| 0x63 | `net.msg.kick_vote` | 0 | both | `{ target_slot: u8, yes: bool }` |

### 3.8 Handshake

```rust
pub struct Hello {
    pub proto: u16, pub sim_version: u32, pub content_hash: [u8; 32], pub presentation_hash: [u8; 32],
    pub build: u32, pub account_id: u64, pub platform: u8 /* 0 Steam, the only store (DECISIONS 112) */, pub persona: String,
    pub password_proof: Option<[u8; 32]>, pub ping_location: String,       // Steam ping location, else empty
    pub shadow_capable: bool, pub handheld: bool, pub minor: bool,          // minor: doc 15 §8.4 flag
    pub migrating_from_tick: Option<u64>, pub resume_slot: Option<u8>,
    pub migrating_cil_hash: Option<[u8; 32]>,   // BLAKE3 of the postcard CIL up to migrating_from_tick (§9.3)
    pub role: u8,                    // 0 party member, 1 invader (§9.13)
    pub invasion_ticket: Option<(u64, [u8; 64])>,  // (expiry Unix s, svc.lobby Ed25519 signature), role 1 only
}
pub struct Welcome {
    pub slot: Option<u8>,            // None while waiting to drop in (§9.1)
    pub host_tick: u64, pub run: Option<RunHeader>, pub party: PartyState, pub lobby: LobbySettings,
    pub presentation_hash: [u8; 32], // the host's; a difference only shows a notice (Canon §2.4)
}
pub struct RunHeader { pub run_id: u64, pub run_seed: u64, pub sim_version: u32, pub content_hash: [u8; 32],
                       pub config: RunConfig /* doc 14 */, pub folio: u32, pub start_tick: u64 }
```

On transport `Connected` the host sends `Challenge` before anything else; the client answers with `Hello`, and disconnects when no `Challenge` arrives within 5,000 ms. The host checks `Hello`, in this order, and answers `Reject` with the first failing reason: `proto` (`net.reject.proto_mismatch`), `sim_version` (`net.reject.sim_version_mismatch`), `content_hash` (`net.reject.content_mismatch`; it covers the simulation-affecting mods, Canon §2.4, doc 02 test 35), platform authentication and, for role 1, the invasion ticket (`net.reject.auth_failed`), password (`net.reject.wrong_password`), host block list and party blocks (`net.reject.banned`), for role 1 the invasion window and the free invader slot (`net.reject.invasion_closed`, §9.13), party seats for role 0 (`net.reject.party_full`), ranked eligibility (`net.reject.run_locked`, §9.1). `presentation_hash` is never a refusal reason: peers may differ in art, audio, VFX, narration, UI and localization (Canon §2.4, DECISIONS 87), and a client whose own `presentation_hash` differs from `Welcome.presentation_hash` shows a notice. `Hello` carries no seed, because none exists before `Commit`; the lobby receives `run_seed` in the `RunHeader` the host sends at `Commit` (Canon §2.4).

`run_id` is the `u64` the host draws from its OS CSPRNG at `Commit` (doc 15 §10.4). It never feeds simulation randomness, is stored in doc 14's `RunState`, travels in `RunHeader`, survives host migration unchanged, and keys run reports, replays, ranked submissions, invasion beacons and tickets.

`Bye` reasons: `net.reject.host_left` (the host quit during a run; party clients migrate, §9.3), `net.reject.session_ended` (Abandon, run end or an invasion's end), `net.reject.kicked`, `net.reject.timeout` (handshake or session silence), `net.reject.protocol_violation`.

### 3.9 Time synchronization

Each client sends `Ping` every 500 ms. The host answers at once with `Pong`, where `host_tick` is the next tick it will simulate and `host_tick_age_us` is the time since that tick's scheduled start. The client keeps `rtt_us` as an exponentially weighted mean (`rtt += (sample − rtt) / 8`, integer µs, truncating toward zero) and `host_tick_est(now) = host_tick + (rtt/2 + host_tick_age_us + elapsed_us) / 16667`, the host's current tick. Derived values: `rtt_ticks = ceil_div(rtt_us, 16667)` and `one_way_ticks = ceil_div(rtt_us, 33334)`. Example: RTT 100,000 µs gives `rtt_ticks` 6 and `one_way_ticks` 3. Snapshot arrival drives a second clock, `snap_tick_est`, used only for interpolation (§7.5).

---

## 4. Session state machines

### 4.1 Client

"Host lost" means `NET_TIMEOUT_MS` of silence from the host or a transport `Disconnected` from it. Every row below is evaluated in table order; an event a state does not list is ignored in that state.

| State | Entered when | Event | Next |
|---|---|---|---|
| `net.cstate.disconnected` | start; `Bye` with a terminal reason; `Reject`; migration failed | player joins a lobby, enters an address, uses Quick Join, or picks an invasion beacon (§9.13) | `connecting` |
| `net.cstate.connecting` | transport connect issued | transport `Connected` | `handshaking` |
| | | connect error, or 5,000 ms | `disconnected` |
| `net.cstate.handshaking` | transport connected; waits for `Challenge`, then sends `Hello` | `Welcome` with a slot and no run | `lobby` |
| | | `Welcome` with a run and `slot = None` | `awaiting_folio` |
| | | `Welcome` answering `migrating_from_tick` | `loading` (keyframe pending) |
| | | `Welcome` with a run and slot 4 (invader) | `loading` (Folio generation, edit backlog and keyframe pending, §9.13) |
| | | `Reject`; no `Challenge` within 5,000 ms; no `Welcome` within 5,000 ms after `Hello`, or within `eta_ms + NET_TIMEOUT_MS` when migrating (§9.3) | `disconnected` |
| `net.cstate.lobby` | | `RunHeader` (doc 14 `Commit`) | `loading` |
| | | `Bye`, or host lost | `disconnected`; a lobby whose host leaves disbands (doc 14 §2.1) |
| `net.cstate.awaiting_folio` | drop-in or rejoin pending (§9.1) | `JoinReady` then `FolioEnter` for that Folio | `loading` |
| | | host lost, or `Bye { net.reject.host_left }` | `migrating` (never a successor, §9.3) |
| | | any other `Bye` | `disconnected` |
| `net.cstate.loading` | `FolioEnter` received, or a migration or invader `Welcome` | blueprint hash equal and `Keyframe` decoded | `playing` (sends `FolioReady`) |
| | | blueprint hash differs | stays; requests chunk data (§6.13) |
| | | host lost, or `Bye { net.reject.host_left }` | `migrating` (party member) or `disconnected` (invader) |
| | | any other `Bye` | `disconnected` |
| `net.cstate.playing` | | no snapshot for `NET_STALL_MS` | `stalled` |
| | | `FolioEnter` for the next Folio | `loading` |
| | | `ev.run_ended` in `RunView` | `lobby` |
| | | host lost, or `Bye { net.reject.host_left }` | `migrating` (party member) or `disconnected` (invader) |
| | | any other `Bye` | `disconnected` |
| `net.cstate.stalled` | overlay shown; prediction frozen at `PRED_STALL_TICKS` | snapshot arrives | `playing` |
| | | host lost, or `Bye { net.reject.host_left }` | `migrating` (party member) or `disconnected` (invader) |
| | | any other `Bye` | `disconnected` |
| `net.cstate.migrating` | §9.3; an invader never enters it | this client claimed the session as successor and finished catch-up | host role (§9.3) |
| | | `MigrateAnnounce` received from another successor | `connecting` to the announced host, then `Hello { migrating_from_tick }` |
| | | the wait of §9.3 expired with no `MigrateAnnounce` | `disconnected`; the Folio-entry state is kept for resume (doc 14 §12.2) |

### 4.2 Host, per peer

| State | Entered when | Event | Next |
|---|---|---|---|
| `net.peer.pending` | transport `Connected`; `Challenge` sent | valid `Hello`, run not started | `lobby` |
| | | valid `Hello`, run started, party member | `waiting` |
| | | valid `Hello` with `migrating_from_tick` (this host is a successor) | held until catch-up ends, then `loading` |
| | | valid `Hello`, role invader, window open and invader slot free (§9.13) | `loading` |
| | | invalid `Hello`, or 5,000 ms without one | `Reject`, closed |
| `net.peer.lobby` | | `Commit` | `loading` |
| | | `Bye`, `NET_TIMEOUT_MS` of silence, or transport `Disconnected` | closed; doc 14 §2.1 returns that slot to `ErratumSelect` |
| `net.peer.waiting` | drop-in or rejoin pending | `JoinRequest` held at the exit-commit tick of the current Folio | `loading` at the next Folio entry |
| | | `Bye`, silence, or `Disconnected` | closed; a pending `SlotJoin` is withdrawn; a resuming slot stays `Dormant` |
| `net.peer.loading` | `FolioEnter`, migration `Welcome` or invader `Welcome` sent | `FolioReady` | `in_game` |
| | | silence for `NET_TIMEOUT_MS`, or `Disconnected` | `lost` (party member); closed with `InvaderLeave { reason: 0 }` (invader) |
| `net.peer.in_game` | | silence for `NET_TIMEOUT_MS`, or transport `Disconnected` | `lost` (party member); closed with `InvaderLeave { reason: 0 }` (invader) |
| | | `Bye` | closed; slot `Departed` (doc 14 §8.8); invader: `InvaderLeave { reason: 0 }` |
| | | kick (§9.2) | `Bye { net.reject.kicked }`, closed; slot `Departed` |
| | | `InvaderLeave` applied (invader peer) | `Bye { net.reject.session_ended }`, closed |
| `net.peer.lost` | `SlotDormant` session intent at the next tick | reconnect with `resume_slot` | `waiting` |
| | | `DORMANT_GRACE_S` (300 s) elapses | closed; `SlotDeparted` |

While a peer is `loading` or `lost`, its slot's commands are substitutes (§5.4), so its Erratum stands still. The Incipit is a sanctuary (doc 14 §3.3), so a loading Erratum is safe.

### 4.3 Lobby messages

Doc 14 §2.1 owns the lobby states; the network carries them with four control messages:

| Type | ID | Direction | Body |
|---|---|---|---|
| 0x70 | `net.msg.lobby_settings` | H → C | `LobbySettings` on every change |
| 0x71 | `net.msg.lobby_pick` | C → H | `LobbyPick` on every change |
| 0x72 | `net.msg.lobby_ready` | C → H | `{ ready: bool }` |
| 0x73 | `net.msg.run_header` | H → C | `RunHeader` at `Commit` |

```rust
pub struct LobbySettings {
    pub privacy: Id32,          // net.lobby.private | friends | public
    pub audience: bool,         // mode.audience_votes, default false (§9.14)
    pub join_open: bool,        // drop-in allowed during the run (default true)
    pub ff: Id32,               // 0 = default, else mode.ff_off | mode.ff_full (Canon §17.5)
    pub loot: Id32,             // 0 = instanced, else mode.loot_shared
    pub invasions: bool,        // default false (§9.13)
    pub voice: Id32,            // net.voice.off | net.voice.party | net.voice.proximity
    pub password_set: bool, pub mode: Id32, pub grade: u8, pub mutators: Vec<Id32>, pub seed: Option<u64>,
}
pub struct LobbyPick {                                   // one player's choices, validated by doc 14 §2.1
    pub slot: u8, pub erratum: ErratumChoice /* doc 14 §1.2 */, pub kit: Id32 /* run.kit.*, doc 14 §2.4 */,
    pub library_glyphs: Vec<Id32>,                       // 0..=2 distinct Glyph Library picks (doc 14 §5.7, DECISIONS 33)
    pub satchel: Vec<Keepsake>,                          // the RunSetup Satchel order, doc 15 §5.5, at most 5
    pub appearance: Option<Appearance>,                  // doc 18 §5.6; None for an Echo; presentation only (doc 14 §1.3)
    pub name: String,                                    // display name, at most 32 UTF-8 bytes; presentation only
}
```

`appearance` and `name` in `LobbyPick` and `JoinCharacter` are presentation data (doc 14 §1.3, DECISIONS 97): the host checks `appearance` against doc 18 §5.6 (a preset of the Erratum's kin, `base` and `marking` in `0..=7`, `None` for an Echo, else the pick is refused), relays both in `PartyState.looks`, and the replay file stores them for the viewer (§10.1); neither enters simulation state or `state_hash`.

---

## 5. Input path

### 5.1 Command production and the input packet

Each client tick (period from §5.6) the client samples input, builds the `PlayerCommand` for its predicted tick `P` (doc 02 §5.5), stores it in the prediction ring, runs the predictor one step (§7.3) and sends an input packet holding the newest `count = min(8, commands produced)` commands, oldest first. `view_tick = floor(render_tick)` at the moment of sampling (§7.5). `action_seq` is the sequence number of the newest intent the client has issued at or before `P`.

| Offset | Size | Field |
|---|---|---|
| 0 | 1 | `msg_type` = 0x10 |
| 1 | 4 | `newest_tick: u32` (wire tick of the last command) |
| 5 | 1 | `count: u8` in `1..=8`; the commands cover ticks `newest − count + 1 ..= newest` with no gap |
| 6 | 4 | `snap_ack: u32`, wire tick of the newest snapshot or keyframe fully decoded; 0 = none |
| 10 | 1 | `interp_delay: u8`, the client's current interpolation delay in ticks |
| 11 | 1 | `flags`: bit 0 Shadow running, bit 1 stalled; bits 2–7 zero |
| 12 | n | commands, oldest first |

The first command is written in full: `move_x i8, move_z i8, yaw u16, pitch i16, buttons u32, action_seq varint, view_lag u8`, where `view_lag = min(255, tick − view_tick)`. Each later command starts with a mask byte (bit 0 move pair, 1 yaw, 2 pitch, 3 buttons, 4 action_seq, 5 view_lag, 6–7 zero) followed by the changed fields in bit order, with `action_seq` written as the varint of its increase over the previous command.

**Worked example.** Ticks 1198 to 1200, newest snapshot decoded at boundary 1185, interpolation delay 6. Tick 1198: full forward (`move_z = −127`), yaw 16384, pitch −300, `SPRINT` held (`0x20`), `action_seq` 41, view tick 1184. Tick 1199: yaw 16400. Tick 1200: yaw 16416, pitch −296, `ATTACK_PRIMARY | SPRINT` (`0x21`), an intent issued (`action_seq` 42). The packet is 37 bytes:

```
10 b0 04 00 00 03 a1 04 00 00 06 00            header (12 bytes)
00 81 00 40 d4 fe 20 00 00 00 29 0e            tick 1198, full (12 bytes)
02 10 40                                       tick 1199: mask yaw, 16400
1e 20 40 d8 fe 21 00 00 00 01                  tick 1200: mask yaw|pitch|buttons|seq
```

In steady state a packet carries 8 commands; the budget of §15 assumes 60 bytes (7 deltas of 5 bytes after a 13-byte first command).

### 5.2 Redundancy and acknowledgement

Every command travels in 8 consecutive packets, so up to 7 consecutive lost packets lose nothing. Snapshots acknowledge implicitly: each snapshot header carries `input_ack_tick`, the last tick the host applied for that slot. The client never resends older commands; a command that arrives after its tick was simulated is late (§5.3).

### 5.3 Host input buffer (jitter buffer)

```rust
pub struct SlotInput {
    buf: BTreeMap<u64, PlayerCommand>,       // ticks in next ..< next + INPUT_BUFFER_SPAN
    last_applied: PlayerCommand,
    last_intent_seq: u32,
    slack_hist: [i16; 120], slack_head: u8,  // arrival slack of each packet's newest command
    late_packets: u32, substituted: u32,
}
fn on_input_packet(si: &mut SlotInput, p: &InputPacket, next: u64) {
    let newest = reconstruct_tick(p.newest_tick, next);
    for (i, c) in p.commands.iter().enumerate() {            // oldest first
        let t = newest + 1 + i as u64 - p.count as u64;
        if t >= next && t < next + INPUT_BUFFER_SPAN { si.buf.entry(t).or_insert(sanitize(c, t)); }
    }
    if newest < next { si.late_packets += 1; }
    let slack = (newest as i64 - next as i64).clamp(-127, 127) as i16;
    si.slack_hist[si.slack_head as usize] = slack; si.slack_head = (si.slack_head + 1) % 120;
}
fn take(si: &mut SlotInput, t: u64) -> (PlayerCommand, bool /* real */) {
    while si.buf.first_key_value().map_or(false, |(&k, _)| k < t) { si.buf.pop_first(); }
    let (c, real) = match si.buf.remove(&t) {
        Some(c) => (c, true),
        None => { si.substituted += 1; (substitute(&si.last_applied, t), false) }
    };
    let c = clamp_view_tick(c, t, &lag_estimate(si));          // §5.5
    si.last_applied = c;
    (c, real)
}
```

`next` is the tick the host simulates next. **Slack** is how many ticks early the newest command arrived. The first packet of a slot fills all 120 history entries with its slack. The host reports, per slot, in every snapshot header: the newest slack, and `target_slack = clamp(2 + ceil_div(spread, 2), 2, 8)`, where `spread = s[113] − s[6]` over the sorted 120-entry history (the 5th-to-95th percentile spread). Examples: spread 0 → 2; spread 3 → 4; spread 6 → 5; spread 20 → 8.

### 5.4 Missing input

When no command for tick `t` is buffered, the host applies the neutral substitute:

```rust
const HOLD_MASK: u32 = (1 << 4) | (1 << 24) | (0b111 << 15);    // CROUCH, WALK, QUICKBAR_INDEX
fn substitute(last: &PlayerCommand, t: u64) -> PlayerCommand {
    PlayerCommand { tick: t, slot: last.slot, move_x: 0, move_z: 0, yaw: last.yaw, pitch: last.pitch,
                    buttons: last.buttons & HOLD_MASK, action_seq: last.action_seq, view_tick: t }
}
```

The substitute keeps stance and the selected quickbar index, releases every action button, moves nowhere and never rewinds. It contains no press edge, because every bit it holds was already held, so it never starts an action; Canon §11.8 reads it as the absence of input, not synthesized input. `ACTION` and `WHEEL_ENTRY` (bits 26–31, doc 17 §12.2) are 0 in the substitute, which is a release and never a press. The replay log records the substitute, since Canon §2.4 logs the applied command.

### 5.5 `view_tick` clamp

```rust
fn clamp_view_tick(mut c: PlayerCommand, t: u64, est: &LagEstimate) -> PlayerCommand {
    // est.rtt_ticks = ceil_div(rtt_us, 16667) from the transport, refreshed once per 60 ticks
    let lag = est.rtt_ticks + est.interp_delay as u64 + est.target_slack as u64;
    let max_rewind = core::cmp::min(LAGCOMP_MAX_REWIND, lag + LAGCOMP_SLOP);
    c.view_tick = c.view_tick.clamp(t.saturating_sub(max_rewind), t);
    c
}
```

| RTT | rtt_ticks | interp | target slack | lag | max rewind | requested `view_tick` at `t = 5000` | applied |
|---|---|---|---|---|---|---|---|
| 100 ms | 6 | 6 | 2 | 14 | 16 | 4982 | 4984 |
| 50 ms | 3 | 6 | 2 | 11 | 15 | 4990 | 4990 |
| 50 ms | 3 | 6 | 2 | 11 | 15 | 4970 | 4985 |
| 400 ms | 24 | 10 | 4 | 38 | 16 | 4960 | 4984 |
| 20 ms | 2 | 4 | 2 | 8 | 12 | 5003 | 5000 |

A client cannot claim more rewind than its measured lag plus 4 ticks, and never more than 16. Doc 05 §2.6 then computes `rewind = t − view_tick`.

### 5.6 Client tick-rate controller

The client keeps `slack_ewma` in milli-ticks and updates it on each snapshot with integer division truncating toward zero: `slack_ewma += (slack × 1000 − slack_ewma) / 8`. With `err = slack_ewma − target_slack × 1000`:

| Condition | Client tick period |
|---|---|
| `err < −1000` (commands arrive too late) | 15,873 µs (63 Hz) |
| `err > 2000` (commands arrive too early) | 17,544 µs (57 Hz) |
| otherwise | 16,667 µs |
| `|slack − target_slack| > 30` | hard reset: `P = floor(host_tick_est) + one_way_ticks + target_slack` (§3.9), the prediction ring is cleared and `slack_ewma = target_slack × 1000` |

Example: `target_slack = 2`, `slack_ewma = 2000`, then six snapshots report slack 0: the average falls 1750, 1532, 1341, 1174, 1028, 900; at the sixth, `err = −1100`, so the client switches to 63 Hz and gains 3 ticks of lead per second. Hard-reset example: `host_tick_est` 5000.4, RTT 100 ms (`one_way_ticks` 3), `target_slack` 2 give `P = 5005`; with a 50 ms one-way delay the command for 5005 reaches the host 3 ticks later, at host tick 5003, 2 ticks early.

### 5.7 Intents

- Intents (doc 02 §5.5) travel as `IntentMsg` on the reliable control channel, tagged with the predicted tick at which the player issued them.
- The host applies an intent at `apply_tick = max(reconstruct_tick(intent.tick, next), next)`, at the start of that tick in `(slot, action_seq)` order. An intent tagged later than `next + INPUT_BUFFER_SPAN` counts one violation (§5.8) and applies at `next`. The CIL and the replay record `apply_tick`.
- An intent with `action_seq <= last_intent_seq` is a duplicate and is dropped. A slot may apply at most 8 intents per tick; the rest wait for the next tick in order.
- The owning document validates each payload; a rejected intent emits a diagnostic and the owner sees the rejection in its next `OwnerDelta`.
- This chapter adds intent kinds: `TradePropose { to_slot }`, `TradeRespond { accept }`, `TradeSetItems { uids }`, `TradeLock { locked }`, `TradeConfirm`, `TradeCancel` (§9.7), `DropItemFor { uid, qty, to_slot }` (§9.6), `MarkTarget { target }` (ghosts, §9.5), and the host-only `Session(SessionIntent)`:

```rust
pub enum SessionIntent {                        // logged with slot = affected slot, applied before player intents
    SlotJoin { slot: u8, character: JoinCharacter },  // at a Folio-entry boundary only (§9.1)
    SlotDormant { slot: u8 }, SlotResume { slot: u8 }, SlotDeparted { slot: u8 },
    HostChanged { from_slot: u8, to_slot: u8 },     // 0xFF = dedicated server; no simulation effect
    FolioRestart { folio: u32 },                     // cold migration (§9.3); counts as a ranked restore
    AudienceResult { kind: u8, choice: u8 },         // §9.14; kind 0 door, 1 boon, 2 Chaos; effects doc 14 §8.1
}
```

Invaders use doc 02's host-issued slot-4 intents `InvaderJoin { kit: Id32 }` and `InvaderLeave { reason: u8 }`; this chapter fixes the reasons: 0 left or disconnected, 1 invader died, 2 time limit, 3 Redacted, 4 Colophon entered, 5 blocked, 6 host migration (§9.13).

### 5.8 Sanitization and protocol violations

`sanitize` maps `move_x`, `move_z` of −128 to −127, clamps `pitch` to ±16000, and replaces with the substitute of §5.4 a command whose `ACTION` field (bits 26–28, doc 17 §12.2) holds 6 or 7, whose bit 25 is set (reserved, always 0), or whose `action_seq` is lower than the slot's last applied `action_seq`. `ACTION` values 0 to 5 are valid. `WEAPON_SELECT` (2 bits), `QUICKBAR_INDEX` (3 bits) and `WHEEL_ENTRY` (bits 29–31, 0 to 7) cannot leave their ranges, and `view_lag` is at most 255, so they need no check. Each replacement, malformed packet, oversized intent (> 4,096 bytes) or intent beyond 120 per second counts one violation. More than 600 violations within 3,600 ticks disconnect the peer with `net.reject.protocol_violation`.

---

## 6. Replication

### 6.1 What travels where

| State | Path | Rate | Reliability |
|---|---|---|---|
| Remote entity presentation (position, rotation, animation, flags, health bar, visible statuses, equipment visuals) | snapshot `ENTITIES` section | 30 Hz, priority-limited | unreliable, delta vs acked baseline |
| Own mover, exact (prediction base) | snapshot `OWNER` section | 30 Hz | unreliable, delta |
| Own pools, exact (HUD) | snapshot `HUD` section | 30 Hz | unreliable, delta |
| Own stealth feedback (doc 09 §8 `StealthHud`) | snapshot `STEALTH` section | 30 Hz on change | unreliable, delta |
| Presentation events | snapshot `EVENTS` section | 30 Hz | resent until acked or 30 ticks old |
| Liquids, gases, fire (presentation) | snapshot `FIELDS` section | as dirty, priority-limited | unreliable, delta |
| Blocks, crack stages and custom micro-voxel masks | `WorldEdits` | per tick with edits | reliable, ordered |
| Inventory, equipment, stats, skills, statuses with timers, Lucre, offers, trade | `OwnerDelta` | ≤ 10 Hz on change | reliable |
| Party summaries | `PartyState` | 2 Hz (every 30 ticks); roster on change | reliable |
| Folio phase, Rubric, notes, vote holds, Erasure clock, visited rooms, map reveal, score | `RunView` | on change | reliable |
| Type knowledge | `KnowledgeDelta` | on change | reliable |
| Applied inputs | CIL; backlog; unseal after an invasion | 10 Hz; §6.14 rates | reliable |
| State hashes | `HashReport` | every boundary `b` with `b % 60 == 0` and `b >= 60` (Canon §2.5) | reliable |

The client never receives another player's inventory, unknown item properties (doc 10 §2.8) or any entity outside the relevance rules of §6.6.

### 6.2 Snapshot timing

For every even tick `t`, after `Sim::step(t)` returns, the sim thread extracts a `ReplFrame` for boundary `b = t + 1` (Canon §2.2 period 2, phase 0; budget 0.40 ms): quantized rows for every live entity, the presentation pages of chunks whose fields changed, and the tick's presentation events. The net thread encodes one snapshot per in-game client from that frame and sends it at once. Snapshot boundaries are therefore odd: 1, 3, 5, …, the boundaries `t + 1` for even `t` that Canon §2.2 names.

### 6.3 Net handles and content indices

- Each client has a table of 8,191 entity handles. A handle is allocated, lowest free first, when an entity becomes relevant to that client, and freed when a snapshot carrying its removal is acknowledged. A handle is never reused while an unacknowledged snapshot mentions it.
- A **content index** `cidx(kind, id)` is the rank of a record among all loaded records of its kind (visuals, animations, statuses, liquids, gases, block causes), sorted by content-ID bytes. Equal `content_hash` guarantees equal indices on every peer, so packets carry 16-bit indices instead of 32-bit `id32`s.

### 6.4 Replicated entity state and quantization

| Bit | Group | Encoding | Quantization and reconstruction |
|---|---|---|---|
| 0 | POS | 1 bit `abs`; abs: x 18, y 16, z 18 bits; delta: per axis a 2-bit class `c` and a two's-complement value of `[0, 6, 11, 16][c]` bits | `q = clamp(to_mm(p), 0, 262143)` for x and z, `0..=65535` for y; delta from the baseline's quantized value; abs when any axis moved more than 32,767 mm |
| 1 | ROT | yaw 12 bits, pitch 10 bits | `yaw_q = yaw >> 4`, shown as `(yaw_q << 4) | 8`; `pitch_q = (pitch + 16000) >> 5`, shown as `pitch_q × 32 − 15984` |
| 2 | ANIM | animation `cidx` 16 bits, start tick 16 bits | the client derives `anim_tick(b) = b − start` (low 16 bits), wrapped or clamped by the pose table |
| 3 | FLAGS | 16 bits | 0 dead, 1 downed, 2 ghost, 3 teleported, 4 invisible, 5 hostile to the receiver, 6 marked by the receiver's party, 7 burning, 8 Contradicted, 9 boss, 10 speaking, 11 asleep, 12–15 zero |
| 4 | HEALTH | hp 8 bits, ward 4 bits | `hp_q = ceil_div(hp × 255, hp_max)` (0 only at 0 HP); `ward_q = min(15, ceil_div(ward × 15, hp_max))` |
| 5 | STATUS | count 4 bits, then per status: Canon §11.1 row index 7 bits, stacks 4 bits | at most 15, in Canon §11.1 row order; stacks capped at 15 |
| 6 | EQUIP | 4 × 16-bit visual `cidx` (main hand, off-hand, head, body; 0xFFFF none), 4 illuminated bits | |
| 7 | EXTRA | light 4 bits, aux 8 bits | aux: containers, the opened-by slot mask; zones, radius in 250 mm units |

Examples: position (12.3456 m, 3.0 m, 200.0004 m) → (12345, 3000, 200000); yaw 16400 → 1025 → shown 16408; pitch −300 → 490 → shown −304; HP 42,500 of 85,000 → 128; HP 400 of 85,000 → 2; ward 10,000 with Max HP 85,000 → 2. Quantization affects presentation only: the predictor's own state arrives exact in `OWNER` (§7.2), and no client decision reads quantized values back into the simulation.

### 6.5 Delta compression and baselines

```rust
struct View { ents: BTreeMap<u16, EntRepl>, pages: BTreeMap<u8, FieldPage>, mover: Option<MoverSnap>, hud: Option<Hud>,
              stealth: Option<StealthWire> }
struct ClientRepl {
    views: BTreeMap<u32, Arc<View>>,   // by boundary; at most SNAP_RING
    acked: u32,                        // newest boundary the client acknowledged
    handles: HandleTable, acc: BTreeMap<EntityId, i64>, page_acc: [i64; 128], bucket: i64,
}
```

- The host encodes snapshot `b` against `views[acked]` and stores `views[b] = views[acked] ⊕ records written`. Entities and pages not written in `b` carry over unchanged from the baseline.
- `snap_ack` in an input packet acknowledges boundary `k`; if `k > acked` and `views` holds `k`, then `acked = k` and every view older than `k` is dropped.
- The client decodes snapshot `b` only when it holds the view at `baseline_tick`, then stores `views[b]` and acknowledges `b` in its next input packet. It buffers at most 32 undecodable snapshots.
- When `b − acked >= KEYFRAME_GAP` (64 ticks), or when the host no longer holds `views[acked]`, the host sends a `Keyframe` (§6.13) on the reliable world channel instead of snapshot `b` and uses it as the baseline of the following snapshots. The bound keeps `views[acked]` plus every newer view inside `SNAP_RING`: at `b − acked = 62` the host holds the views of `acked, acked + 2, …, b`, exactly 32.
- An entity changes in a snapshot only when its current `EntRepl` differs from the baseline's; unchanged groups are omitted by the mask, and an entity with no changed group writes nothing.

### 6.6 Interest management

Each client's **view origin** is its Erratum; a ghost spectating an ally uses the ally; a loading client uses the Incipit. An entity `e` is relevant to client `c` when the first matching rule says so:

1. **Never** an item drop with visibility `Instanced(s)` and `s ≠ c.slot` (doc 11 §14.4); never a ghost to an invader's client.
2. **PvP culling** (§9.13): an invader and an Erratum are mutually relevant only while visible. On every tick with `t % PVS_PERIOD == 0` the host tests each such pair with four rays, from each side's eye to the other's eye and to the other's origin + 900 mm; the pair is visible when the origins are at most `pvs_near_mm` (16,000 mm) apart or any ray reaches its end without crossing a `full` block (doc 03 §5.18). `last_seen` is the newest test tick that found the pair visible; at a test tick with `t − last_seen > PVS_GRACE_TICKS` the pair is culled, and the next visible test restores it. A target carrying `st.marked` applied by the viewer's side is never culled, because Canon §11.1 makes it visible through walls to that side.
3. **Always**: every Erratum; every entity with `owner_slot = c.slot`; every template flagged `net_always` (Editor bosses during `BossFight`, hunters, exit doors).
4. **Window**: with column distance `D = max(|cx − ocx|, |cz − ocz|)` between `e`'s chunk column and the view origin's, `e` becomes relevant at `D <= 2` (the doc 03 §15.1 activation window) and stops being relevant only at `D >= 4`, so an entity that drifts to `D = 3` stays.
5. **Marked**: `e` carries `st.marked` applied by any member of `c`'s party.

Field pages use rule 4. World edits are global. Events use §6.9.

### 6.7 Priority accumulator and budget

```
w_kind: erratum 1000, boss|editor|hunter 900, projectile 700, hostile creature 600, falling block 600,
        other creature 400, gadget|zone 300, field page 300, container 200, item drop|corpse 100, marker 50
d_mm    = horizontal distance from the view origin to e's origin, whole mm
dist_bp = clamp(10000 − floor_div(d_mm × 9000, 80000), 1000, 10000)
view_bp = 15000 if ang_diff(view_yaw, bearing to e) <= 10923 (60°) else 10000
prio    = mul_bp(mul_bp(w_kind, dist_bp), view_bp)
```

| Entity | Distance | In view | `dist_bp` | `prio` |
|---|---|---|---|---|
| hostile creature | 20 m | yes | 7750 | 697 |
| Erratum | 0 m | no | 10000 | 1000 |
| hostile creature | 85 m | no | 1000 | 60 |
| projectile | 40 m | yes | 5500 | 577 |
| gadget | 10 m | no | 8875 | 266 |

Per snapshot:

1. `refill = floor_div(SNAP_RATE, 30)` (352 by default); `bucket = min(bucket + refill, 6 × refill)`; `budget = min(1150, max(0, bucket))` bytes of payload.
2. Mandatory, in order: header, `OWNER`, `HUD`, `STEALTH`, events concerning the receiver's own slot (at most 128 bytes). These are written even when they exceed `budget`.
3. Candidates: relevant entities whose state differs from the baseline, creates and removals (`acc += 100000` once when they first become pending), and dirty field pages. Each pending candidate adds its `prio` to `acc`.
4. Candidates are sorted by `acc` descending, ties by ascending handle (pages after entities, by chunk slot). Records are written in that order while they fit; a record that does not fit is skipped, and writing stops after 8 consecutive skips. Remaining events fill the leftover bytes.
5. Every written candidate resets `acc` to 0. `bucket −= bytes written`, never below `−6 × refill`.

### 6.8 Snapshot packet

| Offset | Size | Field |
|---|---|---|
| 0 | 1 | `msg_type` = 0x20 |
| 1 | 4 | `snap_tick: u32`, boundary `b` |
| 5 | 4 | `baseline_tick: u32` |
| 9 | 4 | `input_ack_tick: u32`, the last tick applied for the receiver's slot (`b − 1`) |
| 13 | 1 | `input_flags`: bit 0 the command for `b − 1` was real, bit 1 its `view_tick` was clamped |
| 14 | 1 | `slack: i8` (§5.3) |
| 15 | 1 | `target_slack: u8` |
| 16 | … | sections: `tag: u8`, `len: varint`, body; until the end of the message |

| Tag | Section | Body |
|---|---|---|
| 0x01 | `OWNER` | postcard of `MoverDelta { mask: u16, fields }`: each changed `MoverSnap` field (§7.2); `Fx` raws as `zz` varints of the difference from the baseline value |
| 0x02 | `HUD` | `mask: u16`, one bit per field in this order from bit 0, then the changed fields in the same order: hp, hp_max, stamina, stamina_max, aether, aether_max, composure, composure_max, ward, breath, poise, toxicity, each a `zz` varint of the difference in `Milli`; satiation `zz`; `light_at` u8; `light` u8 (low nibble `light_block`, high nibble `light_sky` of the `light_at` cell after the active light overrides, the `ChunkView.light` layout); `bleed_left` u16 while downed |
| 0x03 | `ENTITIES` | `count: varint`, then a bit stream (LSB first) of records in ascending handle order, padded to a byte |
| 0x04 | `FIELDS` | §6.12 |
| 0x05 | `EVENTS` | `count: varint`, then a bit stream of §6.9 records |
| 0x06 | `STEALTH` | doc 09 §8's `StealthHud` for the receiver's Erratum at boundary `b` as `StealthWire`, written in full when it differs from the baseline view's and omitted otherwise: `gem: u16`; `hidden: u8`; `ring_count: u8` (at most 4, newest first), per ring `dx`, `dy`, `dz` as `zz` varints of whole mm from the receiver's own position (`to_mm` of `MoverSnap.p`), `radius_mm` varint, `class: u8`; `mark_count: u8` (at most 8, highest `meter` first, ties to the lower observer `EntityId`), per mark the observer's net `handle: u16` (8191 when it is not relevant to the receiver), `bearing: u16` (`yaw_from_dir(ox − rx, oz − rz)` over the observer's and the receiver's origins, Canon §1.9), `state: u8`, `meter: u16`, `flags: u8`; `step_count: u8` (at most 4, newest first, each kept 30 ticks from the tick the step was heard, doc 09 §8.3), per step `yaw_from_dir(dir.x, dir.z)` as `u16`; then `panel: u8` (0 none, 1 `Lock`, 2 `Hack`, 3 `Steal`) and doc 09 §8.4's panel body: `Lock` as `target: u64, tier: u8, your_tier: u8, chance: u16, slip_bp: u16, ticks: u32, progress: i32, lock_hp: i32` (28 bytes); `Hack` as `target: u64, tier: u8, your_tier: u8, hack_fails: u8, line_count: u8` (at most 6, ascending `verb`), per line `verb: u8, chance: u16, ticks: u32`; `Steal` as `target: u64` (the `EntityId`), `line_count: u8` (at most 16, highest `chance` first, ties to the lower `uid`), per line `uid: u64, plant: u8, chance: u16, ticks: u32`. The panel travels inside the section, so it is written only when the whole `StealthWire` differs from the baseline view's. An invader's snapshots never carry it, because the invader has no meters (doc 09 §14.5) |

Entity record: `handle` 13 bits, `op` 2 bits (0 update, 1 create, 2 remove). An update follows with an 8-bit group mask and the groups of §6.4. A create carries `entity_id` 64 bits, `kind` 4 bits (doc 03 §3.2 row index), `owner_slot` 3 bits (7 = none), visual `cidx` 16 bits, then a mask with every group the kind uses and those groups in full. A remove carries nothing more. A walking creature's update is 13 + 2 + 8 + 29 (POS delta: abs bit, two 13-bit axes, one empty axis) + 22 (ROT) = 74 bits. A create of a kind that uses all 8 groups, with no statuses, is 110 header bits (13 + 2 + 64 + 4 + 3 + 16 + 8) plus 219 group bits (POS 53, ROT 22, ANIM 32, FLAGS 16, HEALTH 12, STATUS 4, EQUIP 68, EXTRA 12) = 329 bits.

### 6.9 Presentation events

An event record is `kind` 6 bits (row of the table below), `age` 6 bits (`b − event tick`), host `seq` 16 bits (the event's `SimEvent.seq`), then the payload. The host keeps each client's outbox until a snapshot carrying the record is acknowledged or the record is 30 ticks old; the client discards duplicates by `(tick, seq)` and plays each event when its interpolation time reaches the event tick.

| Row | Event | Sent to | Payload bits |
|---|---|---|---|
| 0 | `ev.damage_dealt` | the attacker's and target's clients; window clients when the target is an Erratum | target 13, attacker 13, display amount 16 (`min(65535, ceil_div(total_mhp, 1000))`), dominant type 4, flags 6 (crit, precision, sneak, blocked, ward, lethal) |
| 1 | `ev.entity_killed` | window | entity 13, killer 13 |
| 2 | `ev.explosion` (03) | window, or the slot's heard level is at least 0 dB | absolute position 52, radius 8 (250 mm units) |
| 3 | `ev.noise` | the slot's `heard[slot]` level is at least 0 dB (doc 03 §11.5) | absolute position 52, dB 8, noise `cidx` 16 |
| 4 | `ev.footstep` (03) | window | entity 13, gait 2, material `cidx` 12 |
| 5 | `ev.status_applied` | window | entity 13, status row 7, stacks 4 |
| 6 | `ev.spell_cast` | window | caster 13, spell visual `cidx` 16 |
| 7 | `ev.fire_started`, `ev.fire_out` (03) | window | `Gidx` 22, kind 2 |
| 8 | `ev.liquid_reaction` | window | `Gidx` 22, kind 4 |
| 9 | `ev.ward_broken` | window | entity 13 |
| 10 | `ev.part_severed` | window | entity 13, part row 4 |
| 11 | `ev.player_downed`, `ev.player_revived`, `ev.player_died`, `ev.player_ghosted`, `ev.player_resurrected` | all clients | event variant 3, slot 3, entity 13 |
| 12 | `ev.hunter_spawned` | all clients | entity 13 |
| 13 | `ev.item_picked_up`, `ev.item_dropped` | window, Public items only | entity 13, item visual `cidx` 16, variant 1 |
| 14 | `ev.invasion_started`, `ev.invasion_ended` | all clients, invader included | variant 1, invader entity 13, reason 3 |
| 15 | `ev.law_discovered` (03) | the clients of the slots set in `slots`; never the invader | law `id32` 32, slots 5 |

Events that only the owner needs (`ev.skill_gained`, `ev.level_gained`, `ev.contradiction_changed`, `ev.item_identified`, `ev.engine_spun`, `ev.erasure_stack`, `ev.affliction_changed`, `ev.container_opened`) travel inside `OwnerDelta`. `ev.block_changed` travels as world edits. `ev.run_ended` travels in `RunView`.

### 6.10 Owner, party and run state

- `OwnerDelta` (postcard), at most every 6 ticks, sent at once after an intent resolves: `{ tick, items: Vec<ItemView> /* upserts, doc 10 */, removed: Vec<u64>, equipment, quickbar, stats, skills /* value and milli-XP */, level, xp, statuses: Vec<StatusView /* id, stacks, expiry */>, lucre, offers, trade: Option<TradeView>, mover: Option<MoverParamsView>, rejected: Vec<u32 /* action_seq */>, owner_events: Vec<NetOwnerEvent> }`. The first `OwnerDelta` after Folio entry is complete. `MoverParamsView` carries, whenever one changes, the doc 03 §5 `MoverParams` fields that follow stats, gear, body and local genre: `body`, `row11_bp`, `row21_bp`, `body_move_bp`, `walk_base`, `sprint_base`, `swim_base`, `crouch_base`, `climb_base`, `stance_base`, `vrv`, `air_jump_max`, `wall_jump`, `wall_run_max_mm` (§7.2; doc 03 §5 carries `crouch_base` and `climb_base`, which Echo plans set per doc 04 §13).
- `PartyState` every `PARTY_STATE_PERIOD` (30) ticks. Allies' live health bars come from the 30 Hz `HEALTH` group of their entity records (§6.4); `PartyState` carries the slower roster data.
- `RunView` on change. It is the network view of doc 14's simulation `RunState` and `FolioRun`, never the state itself. A joining, resuming or migrating client receives a complete `RunView` before its first snapshot, so it rebuilds doc 17 §7's Current Folio page, the Folio quest and World Law visibility from it alone.

```rust
pub struct PartyState { pub tick: u64, pub slots: [Option<SlotSummary>; 5], pub successors: Vec<u8>,   // §9.3
    pub roster: Option<Vec<(u8 /* slot */, u64 /* account_id */, String /* persona */)>>,       // only when changed
    pub looks: Option<Vec<(u8 /* slot */, Option<Appearance> /* doc 18 §5.6 */, String /* display name */)>>, // only when changed; presentation (§4.3)
    pub direct_addrs: Option<Vec<(u8 /* slot */, [u8; 16] /* IPv6, IPv4-mapped */, u16 /* port */)>> } // net.route.direct only, when changed (§9.3)
pub struct SlotSummary { pub status: u8,   // doc 14 SlotStatus 0 Active, 1 Ghost, 2 Dormant, 3 Departed; 4 Waiting, 5 Invader
    pub level: u16, pub hp_bp: u16, pub ward_bp: u16, pub bleed_left: u16, pub ping_ms: u16,
    pub flags: u8 }                        // bit 0 Shadow running, bit 1 handheld, bit 2 CIL complete from tick 0
pub struct RunView { pub tick: u64, pub folio: u32, pub genre: Id32, pub phase: u8 /* doc 14 FolioPhase index */,
    pub rubric: Vec<(Id32, u32, u32)> /* objective, progress, goal */, pub notes: Vec<(Id32, u8 /* done */)>,
    pub doors: Vec<(u16 /* hold */, u8 /* voters */)>, pub erasure_next_tick: Option<u64>,
    pub rooms_visited: Vec<u16> /* doc 12 room indices the party has visited, ascending (doc 14 RunState) */,
    pub map_revealed: bool /* item.text.mapping read in this Folio: every room outline, door and exit, doc 06 */,
    pub climate: (Id32, Id32) /* doc 03 §4.8 Climate { tod, wx } of the current Folio */,
    pub laws_triggered: Vec<(Id32, u8 /* slot mask */)> /* doc 12 §4.1, ascending law ID */,
    pub story_read: u8 /* distinct story events the party has read, doc 12 §20.4 */,
    pub story_solved: Option<u64> /* doc 12 §20.6, doc 14 FolioRecord */,
    pub leaf: u8 /* item.quest.folio_leaf: 0 not taken, 1 on the ground, 2 + s held by slot s, 7 carried out */,
    pub score: i64, pub end: Option<RunEndRecord> /* doc 14 §4.4 */ }
```

### 6.11 Voxel diffs

All block changes, crack stages and custom micro-voxel masks (doc 03 §4.3, §4.4) replicate to every client, inside or outside interest, because terrain is shared state and the limited-building rule (DECISIONS #16) keeps edits sparse.

| Offset | Size | Field |
|---|---|---|
| 0 | 4 | `rseq: u32` (reliable framing) |
| 4 | 1 | `msg_type` = 0x22 |
| 5 | 4 | `tick: u32` |
| 9 | 1 | `part`: bit 7 set when more parts of the same tick follow |
| 10 | v | `edit_count: varint`, then per edit in `ev.block_changed` emission order: `zz(gidx − prev_gidx)` (prev starts at 0), `state: u16`, `cause: u8` (`cidx` of `grid.cause.*`) |
| … | v | `crack_count: varint`, then per cell whose crack stage changed: `zz(gidx − prev_gidx)`, `stage: u8` in `0..=9` = `floor_div(accumulated × 10, block_hp)` (doc 03 §13.2); 24,000 accumulated of 30,000 is stage 8 |
| … | v | `mask_count: varint`, then per cell, in ascending `Gidx`, whose custom mask at the end of the tick differs from the one the client holds after applying this message's edits (each edit clears its cell's custom mask, as `set_block` does, doc 03 §13.1): `zz(gidx − prev_gidx)` (prev starts at 0), `mask: u64` little-endian, the cell's custom mask, 0 when it has none so its state's predefined mask applies. Chipping (`ev.block_chipped`, doc 03 §13.2), `set_block_mask` and every other mask write of doc 03 §4.3 produce these records |

| … | v | `fall_count: varint`, then per falling block doc 03 §13.6 created on this tick, in ascending `EntityId`: `entity_id: u64`, `mask: u64`, the custom mask its cell held when it began to fall (0 = none); the client draws that falling entity with the mask's shape |

A part holds at most 4,096 entries in each of its four lists. The client applies a tick's edits, then its masks. It keeps a **pending overlay** `BTreeMap<Gidx, (BlockState, u64)>` holding, for each cell a received batch changed whose tick its render timeline has not reached, the cell's state and custom mask after that batch (0 = none); prediction collision reads the overlay first and the render grid second, through doc 03's `mask_at` (§4.4), and the renderer applies a batch to the render grid when `render_tick >= tick`. Example: an edit at `Gidx` 2,099,204 to `block.air` (state 0) by cause index 3 after a previous edit at 2,099,200 encodes as `08 00 00 03`. After a batch reaches the render grid, the client runs doc 03's relight on its presentation light grid with the same 4,096-update budget per tick; presentation light is never compared with the host's, and the HUD light readout reads `HUD.light_at` and `HUD.light` (doc 17's stealth meter).

### 6.12 Field pages

A field page is the presentation state of one chunk's non-empty fluid and fire cells. Presentation cell: `liq_level = min(15, ceil_div(total_units × 15, 1000))`, `liq_type` = the component with the most units (ties to the lower type index), `gas_conc = min(15, ceil_div(conc × 15, 10000))` of the strongest gas (concentration 5,000 → 8), `gas_type`, `fire` = the `FireCell` intensity (0 to 3), `charged` = the liquid is electrified (doc 03 §6.9).

`FIELDS` body: `page_count: varint`; per page: `slot: u8`, `full: u8` (1 = all non-empty cells, 0 = changes since the baseline page), `cell_count: varint`; per cell in ascending `Local`: `local_delta: varint` (from the previous `Local`, first from −1), `rec: u16` (bits 0–3 liquid level, 4–7 gas concentration, 8–9 fire, 10 charged), then `liq_type: u8` when the liquid level is above 0 and `gas_type: u8` when the gas concentration is above 0. In a delta page, `rec = 0` means the cell became empty. Example: a cell at `Local` 1,057 holding 437 units of the liquid with type index 2 encodes, as the first cell of a page, `a2 08 07 00 02` (`liq_level = ceil_div(6555, 1000) = 7`).

### 6.13 Folio entry and keyframes

At each Folio-entry boundary the host sends, in this order:

1. `FolioEnter` (control): `{ folio, genre, gen: GenInput /* doc 12 */, entry_tick, blueprint_hash: [u8; 32], section_hashes: [[u8; 32]; 13], slots: Vec<(u8, SlotStatus)> }`.
2. `FolioState` (world): zstd of serialization sections 0, 1, 2, 3, 8, 9, 10, 11 and 12 (doc 02 §6.5; 12 is doc 09's `StealthWorld`) at the entry boundary. Every client stores it with the regenerated sections 4–7 (blocks, light, fluids, thermal) as its Folio-entry save copy (doc 14 §12.1).
3. `Keyframe` (world): the client's full relevant view at the entry boundary (every relevant entity as a create record, every field page in the window as a full page, `OWNER`, `HUD`), in the snapshot section format, with `baseline_tick = 0`.

The client generates the Folio with doc 12's pure generator, compares `blueprint_hash`, rebuilds sections 4–7 at the entry boundary and compares their section hashes. On any mismatch it sends `FolioReady { gen_ok: false }`, the host sends `ChunkData` (zstd of sections 4–7), and the client counts one generation desync in telemetry (doc 12 I2). Generation runs on a worker thread within doc 02's 1,500 ms load budget.

### 6.14 Confirmed Input Log

The CIL is the exact `TickInputs` the host applied, the same data as the replay log.

| Offset | Size | Field |
|---|---|---|
| 0 | 4 | `rseq: u32` |
| 4 | 1 | `msg_type` = 0x30 |
| 5 | 4 | `first_tick: u32` |
| 9 | 1 | `ticks: u8` (6 in steady state; up to 255 after a send stall; backlogs use `net.msg.cil_backlog`) |
| 10 | … | per tick: `slot_mask: u8` (bits 0–4); per slot in the mask: one command as an input-packet delta against that slot's previous CIL command, with mask bit 6 = substitute and bit 7 = full; then `intent_count: varint` and that many `(slot: u8, apply_tick delta: varint, action_seq: varint, postcard IntentKind)` records, session intents first |

A slot's first command after a `SlotJoin`, a resume, the start of a stream, or the end of an invasion seal is written full.

**Backlog.** A party client missing CIL ticks (a drop-in joiner, a client resuming from Dormant, or a client whose CIL prefix hash failed after a migration, §9.3) receives them as `net.msg.cil_backlog` messages: `first_tick: u64`, `ticks: u16` (at most 3,600), then a zstd frame (level 3) of those ticks in the per-tick encoding above, split into 65,536-byte reliable parts. The stream runs at `CIL_BACKLOG_WAIT_RATE` (8,000 B/s) while the client is not in game (waiting for its Folio, or on the run-end screen) and at `CIL_BACKLOG_PLAY_RATE` (1,000 B/s) while it plays, during which that client's `SNAP_RATE` is lowered by 1,000 B/s so its total stays within `CLIENT_DOWN_BUDGET`. At the replay budget of 2 MB per hour (§10.1), one hour of backlog takes 250 s while waiting. A client writes its replay file only once its CIL is complete, and ranks below every complete client as a successor (§9.3).

**Invasion sealing.** From the tick of `InvaderJoin` through the tick of `InvaderLeave`, CIL messages to party clients carry no slot-4 command (slot_mask bit 4 clear) and no slot-4 intent. The host keeps those records and, in the first CIL batch after `InvaderLeave`, sends `net.msg.cil_unseal`: `rseq: u32`, `msg_type` 0x36, `first_tick: u32`, `ticks: u16`, then per tick the slot-4 command (first one full) and `intent_count: varint` with the slot-4 intents. Each client merges them into its CIL, so its replay is complete. A Shadow stops at the boundary of the `InvaderJoin` tick and catches up from the merged CIL. The invader's own client never receives the CIL. A party client therefore never learns the invader's inputs while the invader is present, which is what makes PvP culling binding (§6.6, decision 9); the cost is that host loss during an invasion migrates cold (§9.3).

**Cold-restart truncation.** A cold migration (§9.3) restarts the current Folio from its entry tick `E`. Every client deletes the CIL records of ticks after `E` and the command records of tick `E`, keeps the session intents of tick `E` (the `SlotJoin`s already inside the Folio-entry save), and the new host appends `FolioRestart { folio }` to tick `E`'s session intents. Tick `E` is then simulated again with new commands, so a verifier re-simulating from tick 0 sees one consistent history.

### 6.15 Text on the wire and Scribe text

Packets carry text keys, content IDs, seeds and integers, never rendered generated text (DECISIONS 138): every generated name, title or description travels as the IDs and seeds that produce it, and each client formats it with Project Fluent messages and their gender, number and case selectors in its own locale (doc 02 §10, doc 18). The only strings on the wire are text a player typed (chat, persona, display name) and `ScribeText`.

`ScribeText` (0x53, reliable control) carries the host's validated Scribe output (doc 18 §9.6): `key` is doc 18's cache key, `job` the `nar.job.*` ID, `subject` the entity or record it names, `json` the validated result of at most 4,096 bytes. The host sends each `key` once to every in-game party client when the job completes, and again to a client that joins, resumes or follows a migration, for every key still displayed in the current Folio. A client stores it by `key` and displays it in place of the template text; a client with `flag.scribe.enabled = false` displays its own template (doc 18). `ScribeText` is presentation only and never enters simulation state, the CIL or `state_hash`; the invader receives none.

---

## 7. Client prediction and reconciliation

### 7.1 Scope

| Predicted exactly (corrected by reconciliation) | Predicted cosmetically (replaced by host results) | Never predicted |
|---|---|---|
| the client's own mover in every doc 03 mode: ground, air, crouch, lean, dodge, mantle, swim, climb, downed crawl, ghost flight | weapon fire: muzzle, tracer, recoil kick, spread and ammo count, using keyed `rng.combat` with the host's key (actor, tick, `seq`), so predicted spread equals the host's (Canon §3.3, doc 05 draw table); melee swing; cast start and Winding (doc 06); thrown and spell projectile stand-ins; block placement and breaking stand-ins; impact decals | damage, hit markers and numbers, deaths, statuses, pickups, doors, AI, other players |

A cosmetic projectile stand-in disappears when the host projectile with the same owner and spawn tick arrives, or after `rtt_ticks + 12` ticks. A predicted block edit is held in the pending overlay and reverted if no host edit for that cell arrives within `rtt_ticks + 12` ticks.

### 7.2 Mover state

```rust
pub struct MoverSnap {                 // exact; compared with ==
    pub tick: u64,                     // boundary
    pub p: Fx3, pub v: Fx3, pub yaw: Bam, pub pitch: i16,
    pub mv: MoveState,                 // doc 03 §3.4, every field
    pub gates: u32,                    // movement-relevant statuses active, one bit per row below
    pub status_speed_bp: [Bp; 8], pub n_status: u8,  // active move-speed status multipliers, ascending status ID; unused entries 0
    pub genre: Id32,
}
```

Stamina is not part of `MoverSnap`. The predictor permits a dodge while the latest authoritative stamina (`HUD`) minus the dodges it predicted since then is at least 20,000 (Canon §4.2), and sprint while that value is above 0; a wrong permit is corrected like any other divergence. `gates`, `status_speed_bp` and `n_status` are copied from the newest authoritative state into every predicted step. The predictor hands the status list to `MoverParams.status_speed_bp` unchanged, so `step_mover` applies one `mul_bp` per status in ascending status ID order exactly as the host does (Canon §4.2); a folded product would round differently and is never used.

`gates` bits: 0 staggered, 1 stunned, 2 frozen, 3 knocked down, 4 sleeping, 5 rooted, 6 grappled, 7 overburdened, 8 exhausted, 9 levitating, 10 downed, 11 ghost, 12 leg severed, 13 crutch, 14 chilled, 15 slowed, 16 hasted, 17 encumbered; bits 18–31 zero.

The predictor calls doc 03's pure controller `step_mover(&mut MoverSnap, &Control, &MoverParams, &dyn BlockQuery) -> MoverOut`, the same function `sys.movement` runs for Errata on the host (doc 03 §5). `MoverParams` (doc 03 §5) comes from `OwnerDelta.mover` (§6.10: `body`, `row11_bp`, `row21_bp`, `body_move_bp`, the walk, sprint, swim and stance base speeds, `vrv`, `air_jump_max`, `wall_jump`, `wall_run_max_mm`), from `MoverSnap` (`status_speed_bp`, `n_status`), and from doc 04's `can_sprint`, `can_jump` and `can_dodge` predicates evaluated on the predicted stamina above and the `gates` bits. `BlockQuery` reads states and custom masks from the pending overlay, then the render grid (§6.11), then the blueprint's gravity-flip and zone data, and its `genre_phys` gives the Folio's genre physics. Entity push and knockback are host-only and reach the client as corrections. `MoverSnap.mv` carries every doc 03 §3.4 `MoveState` field, including `ledge_base_y: Fx`, `slide_t: u8`, `slide_dir: Fx3`, `last_slide_start: Tick` and `grapple: Option<Grapple { anchor: Fx3, cell: Gidx, t: u8, blocked: bool }>`, and `MoveMode` codes 8 `slide` and 9 `grapple`; `MoverDelta` lists each of them as one mask field. `start_grapple` (doc 03 §5.24) is a host step outside `step_mover`, so a grapple start reaches the client as a correction. `BlockQuery::mask(c)` returns the replicated custom mask of the pending overlay, else of the render grid, else the predefined mask of the cell's state, and `u64::MAX` outside the Folio box (doc 03 §5, §5.24).

### 7.3 Algorithm

```rust
pub struct Predictor { ring: [Option<PredRec>; 128], base: MoverSnap, cur: MoverSnap, next: u64 }
struct PredRec { tick: u64, cmd: PlayerCommand, after: MoverSnap }    // `after` = boundary tick + 1

fn predict_tick(pr: &mut Predictor, cmd: PlayerCommand, env: &Env) {
    if pr.next - pr.base.tick >= PRED_STALL_TICKS { return; }          // stalled: no new commands (§4.1)
    let mut m = pr.cur; step_mover(&mut m, &control_of(&cmd), &env.params, &env.blocks);
    pr.ring[(pr.next % 128) as usize] = Some(PredRec { tick: pr.next, cmd, after: m });
    pr.cur = m; pr.next += 1;
}
fn on_authoritative(pr: &mut Predictor, auth: MoverSnap, env: &Env, smooth: &mut Smoothing) {
    if auth.tick <= pr.base.tick { return; }                           // older than what we hold
    pr.base = auth;
    if auth.tick >= pr.next { pr.cur = auth; pr.next = auth.tick; return; }   // host is ahead: adopt
    let mine = pr.ring[((auth.tick - 1) % 128) as usize].filter(|r| r.tick == auth.tick - 1);
    if mine.map_or(false, |r| r.after == auth) { return; }             // exact agreement: nothing to do
    let before = pr.cur.p;
    let mut m = auth;
    for t in auth.tick..pr.next {                                      // replay unconfirmed commands
        let r = pr.ring[(t % 128) as usize].as_mut().expect("invariant: ring covers base..next");
        step_mover(&mut m, &control_of(&r.cmd), &env.params, &env.blocks);
        r.after = m;
    }
    pr.cur = m;
    smooth.add(before - m.p);                                          // §7.4
}
```

Agreement is exact equality of the whole `MoverSnap`, so any divergence (a push, knockback, a block another player placed, a status) triggers a correction, and a correction replays at most `PRED_STALL_TICKS` steps (≤ 120 × 3 µs = 0.36 ms, doc 03 §17).

### 7.4 Smoothing

The renderer draws the predicted position plus a visual offset `o`. A correction adds `before − after` to `o`; every rendered tick multiplies `o` by 0.85 (half-life 4.3 ticks, presentation floats only). If `|o| >= 2,000 mm`, or the `TELEPORTED` flag is set, `o = 0` (snap). The camera follows the drawn position, so small corrections never jolt the view.

### 7.5 Remote entities: interpolation and its jitter buffer

- **Snapshot clock.** On each snapshot arrival at local monotonic time `a_us`, `sample = b × 16667 − a_us` (`i64` µs). The first sample sets `off_us = sample`; afterwards `dev = sample − off_us` is recorded and then `off_us += dev / 16` (truncating toward zero), except that `|dev| > 250,000` resets `off_us = sample`. `snap_tick_est(now) = (now_us + off_us) / 16667`, kept in µs precision, is the boundary of the newest snapshot the client should hold now.
- `render_tick = snap_tick_est − interp_delay`. The delay is measured from snapshot arrival, not from the host's current tick, so it absorbs jitter and loss only; the one-way delay is already inside `snap_tick_est`. A command for `P = host_tick_est + one_way_ticks + slack` sampled at `render_tick` therefore has `P − view_tick = rtt_ticks + slack + interp_delay` in steady state, which is exactly the `lag` §5.5 permits. Example: RTT 100 ms (`rtt_ticks` 6, `one_way_ticks` 3), `target_slack` 2, `interp_delay` 6, `host_tick_est` 5000.0 at sampling: `P = 5005`, `snap_tick_est` = 4997.0, `render_tick` = 4991.0, `view_tick = 4991`, and the host rewinds `5005 − 4991 = 14` ticks.
- Each remote entity keeps the samples `(snap_tick, EntRepl)` of the snapshots that wrote it; the renderer interpolates between the two samples that bracket `render_tick` (doc 02 §7.3 rules for yaw and teleports).
- `interp_delay = clamp(4 + ceil_div(2 × jitter_us, 16667) + 2 × floor_div(loss_bp, 500), 4, 12)` ticks, recomputed every 60 ticks and moved by at most 1 tick per 60 ticks. `jitter_us = s[284] − s[15]` over the sorted last 300 `dev` values (the 5th-to-95th percentile spread); `loss_bp = floor_div(missing × 10000, 300)` over the last 300 expected snapshot boundaries. Examples: jitter 12 ms, loss 1% → 6; jitter 12 ms, loss 6% → 8; jitter 40 ms, loss 16% → 12; a clean LAN → 4.
- Nothing extrapolates (Canon §2.1). When `render_tick` passes an entity's newest sample, the entity holds that sample; after 6 ticks without a new sample it is drawn as stale (desaturated, doc 17) until one arrives.
- Events play at `render_tick`; world edits apply at `render_tick` (§6.11).

---

## 8. Lag compensation

The host applies doc 05 §2.6 with the clamped `view_tick` of §5.5:

1. **Who rewinds.** Hitscan rays and melee samples of Errata and of the invader (§9.13) test every other entity's hitbox capsules posed at `t_r = t − rewind`, `rewind = min(t − view_tick, 16)`. NPC attacks, explosions, area effects, surfaces and shock chains never rewind.
2. **Projectiles.** An Erratum's or invader's projectile spawned at `t` takes `rewind` catch-up steps, step `i` (0-based) against the poses of `t_r + i + 1`, so the last tests the poses of `t`; it then takes its normal step against the poses of `t`, and one step per tick from `t + 1` (doc 03 §5.19, doc 05 §2.6).
3. **What rewinds.** Only target transforms and poses, from `Pose.hist` (Canon §9, doc 03 §5.16). Blocks, the shooter's own position, target state (guard, parry window, dodge invulnerability, statuses, ward, HP, alive or dead) always read tick `t`.
4. **Missing history.** A target whose `spawn_tick > t_r` is not hittable by that rewound test. A target that died or despawned before `t` is never hit.
5. **Replays** reproduce every rewound hit from the logged, already clamped `view_tick` (Canon §2.4).

Example: at `t = 5000` a client with RTT 100 ms, whose steady-state `view_tick` is 4986 (§7.5), claims `view_tick` 4982; §5.5 applies 4984, so `rewind = 16` and the ray tests poses recorded at tick 4984.

---

## 9. Co-op rules

### 9.1 Drop-in at Folio boundaries

1. A player joins a run in progress when `join_open` is true and a seat is free (party slots 0–3). The host answers `Welcome { slot: None }`; the client enters `awaiting_folio`, receives the CIL backlog, and creates or picks an Erratum on the join screen (doc 14 §2.5 values).
2. The client sends `JoinRequest { character }`. If it is held at the exit-commit tick of the current Folio (doc 14 §3.6), the host enqueues `SessionIntent::SlotJoin` for the next Folio-entry boundary; otherwise the join waits one more Folio.

   ```rust
   pub struct JoinCharacter { pub erratum: ErratumChoice /* doc 14 §1.2 */, pub kit: Id32 /* run.kit.* */,
       pub alloc: Vec<(Id32 /* sub.* */, u8)>,      // doc 14 §2.5 join-level points, ascending sub ID
       pub library_glyphs: Vec<Id32>,                 // doc 14 §2.5, §5.7: 0..=2 distinct Glyph Library picks, learned at the join
       pub satchel: Vec<Keepsake>,                    // doc 15 §5.5, at most 5, materialized from the join Folio on
       pub appearance: Option<Appearance>,            // doc 18 §5.6; None for an Echo; presentation only (§4.3)
       pub name: String }                             // display name, at most 32 UTF-8 bytes; presentation only
   ```
3. `enter_folio` (doc 14 §3.3) consumes pending `SlotJoin` intents in ascending slot order before the Folio-entry save, so the joiner is in the save, in `n` (§9.10) and in the CIL.
4. **Ranked runs.** A joiner needs a ranked token for the run (§11.6); for `mode.daily` the token consumes the joiner's attempt for that UTC date, and a joiner whose attempt is used is refused with `net.reject.run_locked`. The board entry uses the largest party size the run reached.
5. No `SlotJoin` takes effect at the entry of the run's final Folio (`final_folio`, doc 14 §3.1); such a joiner waits for the next Folio entry (Marginalia, or Second Edition Folio 1, doc 14 §8.9) or the run's end.

### 9.2 Leaving, Dormant and reconnect

- **Transient loss.** Up to `NET_TIMEOUT_MS` of silence, the slot stays in the world on substitute commands and nothing else happens.
- **Dormant.** At timeout the host enqueues `SlotDormant` for the next tick; doc 14 §8.8 then removes the Erratum from the world with its inventory. Kill XP and loot rolls skip Dormant slots.
- **Reconnect.** A Dormant player reconnects with `resume_slot`, waits in `awaiting_folio`, and returns through `SlotResume` at the next Folio entry. After `DORMANT_GRACE_S` (300 s wall clock) the slot becomes `Departed` (`SlotDeparted`).
- **Steam Deck suspend** looks like transient loss, then Dormant (doc 02 §14).
- **Kick.** The host kicks with `SlotDeparted` and `Bye { reason: net.reject.kicked }`. In public lobbies any member starts a kick vote, and the starter's vote counts as yes; one vote runs at a time. With `m` Active members other than the target, it passes when at least `max(2, m − 1)` of them vote yes within `KICK_VOTE_TICKS` (1,800): 2 of 3 in a full party, 2 of 2 in a party of 3, and never in a party of 2, where only the host can kick. A kicked account cannot rejoin that session.

### 9.3 Host migration

DECISIONS 93 refines 76: hot and warm migrations continue the Folio from the live state at `L + 1`; only a cold migration, when no party client can rebuild the current state, restores the Folio-entry save and restarts the Folio from its entry tick `E`. Doc 14 follows this section (triage C-06).

**Successor list.** Every 300 ticks the host ranks the other Active party clients and publishes the order in `PartyState.successors` and the lobby key `pal_succ`. Rank keys, compared in order: (1) Shadow running and in sync with the last `HashReport`; (2) CIL complete from tick 0 (§6.14); (3) lower mean estimated RTT to the other members (Steam ping locations, else RTT to the host); (4) not handheld (`Hello.handheld`); (5) lower slot. Invaders, Dormant slots and clients in `awaiting_folio` are never successors.

**Detection.** A party client enters `migrating` when the host's connection reports `Disconnected`, after `NET_TIMEOUT_MS` of silence, or on `Bye { net.reject.host_left }`. The successor at rank `k` (0-based) waits `k × MIGRATE_ANNOUNCE_MS` without seeing a `MigrateAnnounce` before claiming the session. A client with no rank waits until `(number of successors) × MIGRATE_ANNOUNCE_MS + NET_TIMEOUT_MS` has passed without a `MigrateAnnounce`, then disconnects. On `net.route.direct` there is no lobby to carry the announcement: each successor's address is its `PartyState.direct_addrs` entry (the address and port the host observed for that client's connection, IPv4 as IPv4-mapped IPv6), every party client other than the claiming successor connects to the successors in rank order, trying each for `MIGRATE_ANNOUNCE_MS`, and the claiming successor sends `MigrateAnnounce` as the first control message after `Challenge` on each such connection.

**Catch-up.** Let `L` be the claiming successor's last complete CIL tick and `E` the current Folio's `folio_entry_tick`. The first matching row decides the mode:

| Mode | Condition | Procedure | Resume tick |
|---|---|---|---|
| `net.migrate.cold` | the successor's CIL lacks sealed slot-4 records at or before `L`: an invasion was in progress, or its `cil_unseal` had not arrived (§6.14) | restore the Folio-entry save; truncate the CIL (§6.14) | `E` |
| `net.migrate.hot` | a Shadow is running | step the Shadow to boundary `L + 1` | `L + 1` |
| `net.migrate.warm` | `(L + 1 − E) × measured_tick_us ≤ MIGRATE_WARM_MAX_MS × 1000` | `Sim::deserialize` the Folio-entry save copy, then step every CIL tick to `L + 1` on the worker thread, comparing `HashReport`s along the way | `L + 1` |
| `net.migrate.cold` | otherwise, or any hash mismatch during warm catch-up | restore the Folio-entry save; truncate the CIL (§6.14) | `E` |

**`measured_tick_us`.** A Shadow measures its own mean step time. Every other party client runs a **probe** after each Folio entry: once its CIL covers the first boundary `p ≥ E + MIGRATE_PROBE_TICKS` with `p ≡ 0 (mod 60)`, its worker thread deserializes the Folio-entry save copy, steps to `p`, and sets `measured_tick_us = floor_div(wall_us, p − E)`. It also compares the result with `HashReport(p)`; a mismatch writes the doc 02 §6.7 dump and counts a desync in telemetry. Before the first probe completes, `measured_tick_us = MIGRATE_TICK_US_DEFAULT` (4,000). A probe re-simulates 600 to 659 ticks on one worker per Folio, at most 0.11 s at 6,000 ticks per second.

At 6,000 ticks per second a 300 s Folio (18,000 ticks) catches up in 3 s; at 500 ticks per second the same Folio needs 36 s, still warm; at 160 ticks per second it needs 112.5 s and migration is cold.

**Takeover.**

1. The claiming successor posts `MigrateAnnounce` at once, before catching up: as a Steam lobby chat message on `net.route.sdr`, and on `net.route.direct` as the first control message after `Challenge` on every incoming connection (Detection).
2. It opens a listen socket (`net.route.sdr`: `CreateListenSocketP2P`; direct: `renet` on the port of its `PartyState.direct_addrs` entry, which is the port it used toward the old host, else `DEFAULT_PORT`), becomes the Steam lobby owner (the current Steam owner calls `SetLobbyOwner(successor)` when it sees `pal_succ`) and sets `pal_host`.
3. It catches up, holding every `Hello` that arrives meanwhile.
4. It enqueues `HostChanged { from_slot, to_slot }` and `SlotDormant { from_slot }` for the resume tick, answers each held `Hello` with `Welcome`, `OwnerDelta` and `Keyframe` (after a cold migration, a full `FolioEnter` sequence instead, §6.13), and resumes stepping at real time.
5. Every other party client, on `MigrateAnnounce`, truncates its CIL to `L_c = min(L_own, L)`, connects and sends `Hello { migrating_from_tick: L_c, migrating_cil_hash, resume_slot }` with the hash of its CIL up to `L_c`, under a handshake deadline of `eta_ms + NET_TIMEOUT_MS`. The new host compares that hash with its own CIL hash up to `L_c`: on a match it sends the CIL ticks `L_c + 1 ..= L` that the client lacks; on a mismatch the client discards its CIL and receives the new host's from tick 0 as a backlog (§6.14). A client that does not reconnect by its deadline becomes Dormant on the new host.
6. An invader is dropped with `InvaderLeave { reason: 6 }` (§9.13).

```rust
pub struct MigrateAnnounce { pub new_host_slot: u8, pub new_host: u64 /* SteamID64, else account_id */,
    pub mode: Id32 /* net.migrate.* */, pub cil_tick: u64 /* L */, pub resume_tick: u64,
    pub cil_hash: [u8; 32] /* BLAKE3 of the postcard CIL up to L */, pub eta_ms: u32 }
```

`eta_ms` is 0 for hot, the warm estimate `(L + 1 − E) × measured_tick_us / 1000` rounded up, and 3,000 for cold.

Inputs the old host applied after `L` are lost: at most `CIL_BATCH_TICKS` plus the one-way delay in ticks, which is 9 ticks at RTT 100 ms.

### 9.4 Downed and revive (owner of `st.downed`)

```rust
pub struct Player {                    // comp.player (doc 03 §3.3); semantics owned here except the doc 17 fields
    pub slot: u8 /* 0..=4 */, pub downs_folio: u8, pub bleed_left: u16, pub revive_progress: u16,
    pub reviver: Option<EntityId>, pub downed_by: Option<EntityId>,
    pub mark_cooldown_until: Tick, pub trade: Option<TradeState>,
    pub mercy_on: bool, pub mercy_bp: Bp,  // doc 17 §13, Scribe's Mercy
    pub breath_held: bool,               // doc 17, written by SetBreathHold
}
```

Canon §11.1 fixes the first down's 30 s bleed-out (1,800 ticks) and the 3 s revive (180 ticks) and gives every later down, damage shortening and revive HP to this section.

- **Entry** (Canon §10.8 step 2): HP 0, `st.downed` without expiry, `downs_folio += 1`, `bleed_left = max(600, 1800 − 600 × (downs_folio − 1))`: 1,800, 1,200, then 600 ticks. Movement mode `downed`, crawl 800 mm/s (doc 03 §5.1). The Erratum keeps its capsules and cannot attack, block, cast, reload, use items, interact, sprint, jump or dodge (doc 05 §14.3); it can look, crawl, ping and chat.
- **Damage while downed** runs the full pipeline; HP stays 0, and each resolved hit shortens `bleed_left` by `ceil_div(total_mhp × 1800, max_hp_mhp)` (doc 05 §14.3; 17,000 mHP at Max HP 85,000 removes 360 ticks). DoT pulses count as hits.
- **Bleed.** `sys.coop` (doc 03 §2.2 row 26b) decrements `bleed_left` by 1 each tick on which no valid reviver acts. At 0, Canon §10.8 step 3 runs (no new Death's Door roll); when it emits `ev.entity_killed`, that event carries `damage_seq = 0xFFFF_FFFF`, because no hit caused the death (doc 03 §14.2).
- **Valid reviver** on tick `t`: an Erratum that is alive, not downed, not a ghost, free of `st.staggered`, `st.stunned`, `st.frozen`, `st.knocked_down`, `st.sleeping` and `st.grappled`, holds `INTERACT`, has its origin within 1,500 mm of the downed origin, and faces it within 8,192 Bam. The lowest-`EntityId` valid reviver is the active one; several revivers do not stack.
- **Progress.** `revive_progress += 1` on every tick with a valid reviver, and resets to 0 on a tick without one. Revival completes when `revive_progress >= revive_ticks = max(30, mul_bp(180, reviver.revive_bp))`, where `revive_bp` is the product of the reviver's talent multipliers (doc 04; Warden `resurgence` 5000 gives 90 ticks). Doc 04's Restoration-100 spell completes revival at once.
- **Completion.** `st.downed` is removed and HP is set (not healed, so Canon §10.9 does not apply) to `mul_bp(max_hp_mhp, revive_hp_bp)`:

  ```
  revive_base_bp = max(REVIVE_HP_MIN_BP, REVIVE_HP_BASE_BP − REVIVE_HP_STEP_BP × grade)    // max(1000, 5000 − 200 × grade); grade 0..=20 (doc 14 §7)
  revive_hp_bp   = max(REVIVE_HP_MIN_BP, revive_base_bp >> min(downs_folio − 1, 3))       // downs_folio >= 1 while downed
  ```

  At Max HP 85,000: Grade 0 gives 5000, 2500, 1250 and then 1000 Bp for the first to fourth down in one Folio (42,500, 21,250, 10,625 and 8,500 mHP); Grade 2 gives 4600 (39,100 mHP) and then 2300 (19,550); Grade 10 gives 3000; Grade 20 gives 1000. The base follows R7 R-CO3's DRG curve, 50% at the easiest Grade falling to 10% at the hardest (R7's 60% tier lies below Grade 0 and is not used). The halving per repeated down is a balance fix: natural regeneration (Canon §6.4 row 2) already restores 50% of Max HP in 100 s out of combat, so a first revive at 50% is no shortcut, but a flat 50% let a pair trade 3-second revives for unlimited half refills in combat. With halving, the fourth and later revives of a Folio give 10% (8.5 HP at Folio 1), which survives exactly one reference enemy hit: 10 HP before mitigation (Canon §17.4 `REF_ENEMY_HIT_D1`), 11 HP at party size 2 (Canon §17.2), 7.563 HP after the reference 20 AR (`DR_bp` 3125, Canon §10.5 step 5). Poise refills. `ev.player_revived { entity, slot, reviver }` is emitted.
- **Folio exit.** A downed Erratum leaves with the party (doc 14 §3.6) and enters the next Folio revived at `revive_hp_bp`, computed with the old Folio's `downs_folio`. `downs_folio` resets at Folio entry.
- **Party wipe.** When no Active Erratum is alive and not downed at the end of a tick, doc 14 §3.2 row 1 kills every downed Erratum (`ev.entity_killed`) and ends the run, so nobody waits out a bleed timer that no ally can stop. Exception (Canon §17.9): when the invader's hit brings the last Erratum that is alive and not downed to 0 HP, the Redaction of §9.13 resolves on that tick instead, and no Erratum dies.
- Downed, revive and bleed draw no random numbers.

### 9.5 Ghosts (owner of `st.ghost`)

Canon §17.6 governs; this section fixes the remaining details.

- The Erratum entity stays in the world with `Kind.flags & GHOST`, flies at walk speed with voxel collision only (doc 03 §5.12), is excluded from every hit, push and perception query, and is replicated to its own party only.
- Allowed actions: `INTERACT` on doors, switches and levers; `PING`; `MarkTarget { target }`, which applies `st.marked` for 480 ticks to a non-ghost entity whose origin is within 30,000 mm and visible by doc 03 §5.18 from the ghost's eye, at most once per 600 ticks (`mark_cooldown_until`).
- Remains (`cat.corpse`, 75,000 g) drop at the death position. At the next `ev.folio_entered` the ghost resurrects with doc 14's `GHOST_REMAINS_HP_BP` (5000) when an ally carried the Remains through the exit, else with `GHOST_BARE_HP_BP` (2500) and the loss of one equipped non-weapon item: one keyed `rng.loot` draw (folio = the new Folio, actor = the ghost's `EntityId`, tick = its `folio_entry_tick`, seq 0), `range(n)` over the eligible items sorted by uid, and 0 draws when `n = 0`. No ghost resurrects at an Endpaper (doc 14 §3.10). At the entry of Second Edition Folio 1 a ghost's Remains count as carried, so it returns as it was at death, with `GHOST_REMAINS_HP_BP` and no item lost (doc 14 §8.9, DECISIONS 107).
- A ghost shares kill XP like every member present in the Folio, downed members included (Canon §8, DECISIONS 81); Dormant and Departed slots and the invader share none.
- Doc 14 §3.10 owns the slot status, XP and Remains rules; this section owns the ghost entity's movement, actions and replication.

### 9.6 Instanced loot

- Every kill, container, Reliquary, bag, Engine and shop shelf rolls per Active, non-ghost slot with `seq = slot` (Canon §3.3, doc 11 §17); Dormant and ghost slots receive nothing. Lucre piles are instanced like items (R7 R-CO4).
- An `Instanced(s)` item is never sent to another client (§6.6 rule 1), so instanced loot is private on the wire, not merely hidden.
- `DropItem` makes an item `Public` (doc 11 §14.4). `DropItemFor { uid, qty, to_slot }` drops it as `Instanced(to_slot)` at the dropper's feet, visible only to that player; it fails for a Departed, Dormant or ghost target, for quest items, for Keepsakes (doc 11 §2.2 `keepsake` field) and for Lucre (`cat.currency`), which passes between party members only as doc 11 §17's Public `DropLucre` piles (Canon §14).
- `mode.loot_shared` follows doc 11 §17; pickups that collide in one tick resolve in ascending picker `EntityId`.

### 9.7 Trading

A trade swaps items between two Errata atomically. Currencies are never traded: Lucre passes between party members only as doc 11 §17's `DropLucre` piles (Canon §14).

| State (`net.trade.*`) | Event | Next |
|---|---|---|
| `idle` | A sends `TradePropose { to_slot: B }`, both are eligible and both are `idle` | `proposed` |
| `proposed` | B sends `TradeRespond { accept: true }` within `trade_accept_ticks` (600) | `open` |
| | B declines, 600 ticks pass, or eligibility is lost | `idle` |
| `open` | either side sends `TradeSetItems` (at most 8 uids, all tradeable) | `open`, both locks cleared |
| | one side sends `TradeLock { locked: true }` | `locked` |
| `locked` | the other side sends `TradeLock { locked: true }` | `both_locked` |
| | any `TradeSetItems`, or the locked side sends `TradeLock { locked: false }` | `open` |
| `both_locked` | `TradeConfirm` from one side sets that side's confirm flag | `both_locked` |
| | the tick on which both confirm flags are set, and the capacity rule holds | `executed` on that tick, then `idle` |
| | the tick on which both confirm flags are set, and the capacity rule fails | `open`, locks and confirm flags cleared; each side's `OwnerDelta.rejected` lists the confirming `action_seq` |
| | any `TradeSetItems` or `TradeLock { locked: false }` | `open`, confirm flags cleared |
| | `TRADE_CONFIRM_TICKS` (1,800) in `both_locked` without both confirmations | `idle` |
| any state but `idle` | `TradeCancel`; eligibility lost; an offered item leaves its offerer's backpack (used, dropped, equipped or destroyed); Folio exit | `idle` |

**Capacity rule.** Each side's backpack, after removing its own offered items, must have room for the incoming items under doc 11's inventory rules. Carried mass may exceed capacity; encumbrance then applies (Canon §11.1).

**Eligibility,** checked on every tick of a trade: both alive, not downed, not ghosts, not `in_combat` (Canon §2.3), origins within 4,000 mm (a trade in progress breaks beyond 6,000 mm). **Tradeable items:** in the offerer's backpack (equipped items cannot be offered, so a cursed equipped item never can), not `cat.quest`, not a Keepsake, not `cat.currency`. **Execution:** remove every offered uid from its giver in ascending uid order, then add them to the receivers in the same order; instance knowledge travels with each item (doc 10 §2.5); `finder_slot` is unchanged. The trade emits, per item, `ev.item_dropped` with `cause = 1` for the giver and `ev.item_picked_up` with `from = 3` and `cause = 1` for the receiver (doc 03 §14.2). Trading draws no random numbers.

### 9.8 Exit voting

Doc 14 §3.6 owns the commit rule (`TURN_HOLD_TICKS` 120, `EXIT_MAJORITY_TICKS` 3,600, `EXIT_ANY_TICKS` 7,200). Votes are positions inside door volumes, so they travel as ordinary movement inputs; there is no vote message. The host replicates each door's hold counter and voter count in `RunView`. Ghosts, Dormant slots and invaders are never voters. A host migration keeps the counters, which are simulation state.

### 9.9 Shared identification knowledge

Type knowledge is party-wide (Canon §13.6, doc 10 §2.2). The host sends `KnowledgeDelta { tick, types: Vec<(Id32 /* type key */, Id32 /* descriptor */, u8 /* state */)>, excluded: Vec<(Id32, Id32)> }` to every party client on change, and the full table in the first message after a join. Instance knowledge travels with the item's `ItemView`. Invaders receive no party knowledge.

### 9.10 Scaling

`n` is the number of Active slots after ghost restoration at Folio entry (doc 14 §3.3); it drives Canon §17.2 for the whole Folio. A slot that goes Dormant mid-Folio changes nothing until the next entry; a joiner counts from the Folio it joins. Invaders never count toward `n`.

### 9.11 Friendly fire options

The lobby's `ff` setting selects the default constants, `mode.ff_off` (direct 0, area 5000) or `mode.ff_full` (both 10000) (Canon §17.5). Ranked modes force the default. The setting is part of `RunConfig` and therefore of the replay.

### 9.12 Chat, voice and pings

- Text chat: at most 200 bytes, 5 messages per 10 s per sender, filtered and reportable per doc 15 §8.5. The host relays chat; it is never stored by any service.
- Voice: every session sends Opus frames of 20 ms at 24 kbit/s (60 B of voice plus a 4 B header) as `net.msg.voice`, relayed by the host. Voice is outside `CLIENT_DOWN_BUDGET` and `HOST_UP_BUDGET` and is best effort: the host forwards at most `VOICE_STREAMS_MAX` (2) talkers to each receiver, the two unmuted talkers (below) whose current talk spurt started earliest (ties to the lower slot), and drops any voice frame to a peer whose transport send queue holds more than `VOICE_QUEUE_MAX` (8,192) bytes, so game traffic never waits behind voice. Modes `net.voice.off`, `net.voice.party` (default; full volume for the party), `net.voice.proximity` (full volume within 5,000 mm, linear to silence at 30,000 mm from the speaker's origin). Defaults follow doc 15 §8.5: adults push-to-talk, party only; minor-flagged accounts follow `juris.flag.minor_chat_default` (0 `friends_only`: free text and voice with friends only, quick chat with others; 1 `quick_ping_only`: pings only). **Voice with strangers starts off** (DECISIONS 75): when a member joins, each client mutes that member's voice unless the member is on its Steam friends list, and the player unmutes per member from the scoreboard (doc 17); text and pings are unaffected. Each client sends `VoiceMute { muted }` on every change of its mask, and the host forwards no voice frame from a talker the receiver has muted. Every speaker can be muted per session and blocked per account.
- World pings are the `PING` button (doc 02 bit 23), handled by the simulation, so they appear in replays. A `PING` press within 15 ticks after the same slot's previous `PING` press upgrades that previous ping to a danger ping instead of placing a new one (doc 17 ping kinds).

### 9.13 Invasions (network and session rules)

Canon §17.9 fixes what an invasion is; this section fixes how it runs.

- **Opt-in.** The lobby's `invasions` setting puts `mode.invasion_open` into `RunConfig` at run start (default off). Daily, Weekly and every leaderboard-ranked run refuse it. It also requires every party member online, no simulation-affecting mod loaded (doc 14 §2.1; the invader's equal `content_hash` then excludes one on its side too), and every member's account permitting invasions (doc 15 `juris.flag.invasion_minor_default`).
- **Window.** The host keeps its beacon (and the informational `pal_inv = open`) only while all hold: `folio_index >= 3`; the Folio is not a boss Folio; `t − folio_entry_tick >= 7,200`; no Erratum has entered `room.colophon`; no invader is present; no invasion has started in this Folio. The window closes on the tick any condition fails.
- **Matchmaking** through `svc.lobby` beacons (§11.5), so private and friends-only parties are findable without exposing their party lobby and no client needs a second Steam lobby. While the window is open the host keeps a beacon with `PUT /v1/invasions/beacons/{run_id}`, refreshed every `BEACON_REFRESH_S` (30 s); the service drops a beacon not refreshed for `BEACON_TTL_S` (45 s), and the host deletes it on the tick the window closes. An invader chooses Invade in the Margin, lists beacons with equal `sim_version`, `proto` and content hash prefix and `|grade − invader max grade| <= 2`, estimates ping from each beacon's Steam ping location, requests an `invasion_ticket` for the lowest estimate under 150 ms and connects by the beacon's SteamID through SDR (§3.5). `svc.lobby` refuses the ticket for accounts with invasions disabled and for accounts that a party member blocked within 30 days (doc 15 §8.5), and hides such beacons from them.
- **Joining mid-Folio,** the one exception to Folio-boundary drop-in: the invader receives `RunHeader` for generation, the world-edit backlog since the Folio entry, and a `Keyframe`. The host enqueues `InvaderJoin { kit }` on slot 4 with the Editor kit the invader picked from its account's kits; doc 13 spawns `enemy.editors.invader`; the entity enters at doc 13 §10.5's hunter placement anchor on the `InvaderJoin` tick, and `ev.invasion_started` is emitted.
- **End.** `InvaderLeave { reason }` with reason 0 (left or disconnected), 1 (invader died; the party's guaranteed Reliquary roll follows Canon §17.9), 2 (`INVASION_MAX_TICKS` since entry), 3 (Redacted), 4 (an Erratum entered `room.colophon`), 5 (a party member blocked the invader) or 6 (host migration). Each emits `ev.invasion_ended`; the invader's entity despawns at the end of that tick and its connection closes.
- **Redaction.** When the invader's hit would bring the last Erratum that is alive and not downed to 0 HP, doc 14 resolves Canon §17.9's Redaction on that tick instead of Canon §10.8 steps 2 and 3; every downed Erratum loses `st.downed` without `ev.player_revived`, and the invasion ends with reason 3.
- **Information.** The invader receives snapshots under PvP culling, `RunView` with `doors`, `rooms_visited` and `laws_triggered` empty, `map_revealed` false, `story_read` 0, `story_solved` none and `leaf` 0 (each would reveal where the party is or has been), no `ev.law_discovered`, no `ScribeText`, and slot numbers and personas from `PartyState`; never the CIL, `KnowledgeDelta`, another player's `OwnerDelta`, or Errata inventories. It receives `run_seed` in `RunHeader` because it regenerates the Folio's terrain; the seed reveals the layout and the keys of keyed draws but no Erratum's position or input. Party clients never receive the invader's commands while it is present (§6.14 sealing). It is never a successor, a voter or part of `n`.
- **Host loss.** The invasion ends with reason 6 and the migration is cold (§9.3), because no party client holds the sealed slot-4 commands; the Folio restarts from its entry. Invasion runs are never ranked, so the restore costs time only.
- **Chat** (doc 15 §8.5): voice with the invader starts off for every account; minor-flagged accounts exchange only pings and emotes with it; every channel carries mute, block and report, and a block ends the invasion with reason 5.
- **Outcome report.** At `ev.invasion_ended` the host and the invader each post `POST /v1/invasions/outcome` (§11.5); `svc.econ` pays doc 15 §2.4 invader Quires from the shorter reported length and treats the party as Redacted only when both reports say so. After a host loss, the new host posts reason 6 with the start tick of the `ev.invasion_started` it received, and the invader's client posts reason 6 when it loses the host.
- **Visibility and hits.** PvP culling (§6.6 rule 2) applies in both directions; lag compensation (§8) applies to both sides.

---

## 10. Replays, daily verification and desync diagnostics

### 10.1 Replay files

Every party client writes `replays/<run_id>.palr` at run end and on Suspend, once its CIL is complete (§6.14): a doc 02 §12.1 container of kind 3, zstd level 9, holding

```rust
pub struct ReplayFile {
    pub header: SessionHeader,                      // Canon §2.4
    pub presentation_hash: [u8; 32],                // Canon §2.4: recorded; a mismatch at playback only warns
    pub run_id: u64, pub config: RunConfig, pub roster: Vec<(u8, u64 /* account_id */, String)>,
    pub cil: Vec<CilTick>,                          // tick 0 to the last tick, exactly §6.14 content
    pub checkpoints: Vec<(u64, [u8; 32])>,          // HashReport boundaries: every 3,600, plus each Folio entry
    pub folio_states: Vec<(u32, Vec<u8>)>,          // §6.13 FolioState per Folio, for seeking
    pub end: Option<RunEndRecord>,                  // doc 14 §4.4
}
```

The run report's `input_log_blake3` (doc 15 §10.4) is BLAKE3 of the postcard encoding of `cil`, so it does not depend on how the CIL was batched on the wire. The budget is 2 MB per hour of 4-player play after zstd; the raw CIL is 22 bytes per tick (a slot mask, four 5-byte command deltas and an intent count), 4,752,000 bytes per hour before zstd. The viewer re-simulates locally with `EMIT_VIEW`, seeks by restoring the nearest Folio state, plays at 1/4× to 8×, and offers a free camera and each player's view.

### 10.2 Daily and ranked score verification

1. At `Commit`, the host requests a ranked token for the party (§11.6). The service computes the Daily or Weekly seed itself (Canon §3.6), checks and consumes each member's attempt (doc 14 §8.4) and signs the token.
2. At run end, the host uploads the replay and the doc 14 §9.4 submission. Every other member's client calls `GET /v1/leaderboards/runs/{run_id}` 600 s after run end and uploads its own copy when the state is `none`. A further complete submission for the same `run_id` returns the first submission's ID and is not re-verified.
3. A `svc.verify` worker runs `palimpsest_server verify`: it checks the token, `sim_version` and `content_hash` against released builds, re-simulates the whole CIL from tick 0 (doc 14 `verify`), compares every checkpoint, the final `state_hash` and the score, and requires the BLAKE3 of the CIL to equal the `input_log_blake3` of at least one run report from a different party member (doc 15 §10.4) when the party had 2 or more accounts.
4. Verdicts: accept, or reject with `svc.err.replay_mismatch`, `svc.err.token_invalid`, `svc.err.sim_version_unknown` or `svc.err.content_hash_unknown`. 95% of submissions are verified within 600 s. The doc 14 §9.4 example Daily (`end_tick` 99,000) costs 16.5 s of one core at 6,000 ticks per second (doc 02 `perf.ci.bot_tps`).

### 10.3 Desync diagnostics

| Detector | Compares | On mismatch |
|---|---|---|
| Shadow client | its state hash at each `HashReport` boundary | doc 02 §6.7: `SectionHashRequest`, dump `desync_<b>_shadow.pald`, then `FullStateRequest`; the host serializes at its next hash boundary and streams `FullState` at ≤ 64 KiB per tick; the Shadow restores it and continues from the CIL; presentation never pauses |
| Every client at Folio entry | `blueprint_hash` and sections 4–7 | §6.13 chunk data; telemetry counter `gen_desync` |
| Non-Shadow client probe (§9.3) | its state hash at the first boundary `p ≥ E + 600`, `p ≡ 0 (mod 60)` | doc 02 §6.7 dump; telemetry counter `desync`; no gameplay effect |
| Warm migration | hashes along the catch-up | cold migration (§9.3) |
| Verifier | checkpoints | reject and keep the dump for 30 days |
| CI | golden replays, SyncTest, 4-player bot runs over NetSim (§16) | build fails |

With crash-report consent, a client uploads its dump to `svc.crash`; player names are replaced by slot numbers (doc 02 §17.4).

---

## 11. Backend services

### 11.1 Services and technology

| ID | Responsibility |
|---|---|
| `svc.auth` | sign-in through Steam tickets, access and refresh tokens, the platform identity table (§3.6) |
| `svc.profile` | account profile (unlocks, Codex, Echo roster, settings references, Margin trophies), export and deletion |
| `svc.econ` | doc 15 economy (`econ-api`, `econ-db`, `econ-verify`, `econ-platform`, `econ-signer`) |
| `svc.lobby` | invasion beacons, tickets and outcome matching |
| `svc.relay` | none: retired before shipping with `net.route.relay` (§3.5, DECISIONS 112); no node runs and the ID stays reserved |
| `svc.leaderboard` | ranked tokens, submissions, boards |
| `svc.verify` | headless replay re-simulation workers (`palimpsest_server verify`) |
| `svc.telemetry` | opt-in usage batches (doc 02 §17.4) |
| `svc.crash` | opt-in minidumps and desync dumps |
| `svc.cloud` | account cloud saves (§11.3) |
| `svc.workshop` | none: a mod mirror for a store other than Steam, which DECISIONS 98 and 112 rule out; nothing runs and the ID stays reserved (§11.8) |
| `svc.status` | build gates, maintenance notices |
| `svc.moderation` | chat and player reports, account blocks |

| Concern | Choice |
|---|---|
| Language and API | Rust; `axum` 0.8.9 on `tokio` 1.53.2; `rustls` 0.23.45; JSON over HTTPS; one binary `palimpsest_services` with a role flag per service (doc 02 `crate.services`) |
| Database | PostgreSQL 18 through `sqlx` 0.9.0; serializable isolation for `svc.econ` and `svc.leaderboard`; doc 15 §10.2's ledger tables in schema `econ`; continuous WAL archiving to object storage with point-in-time recovery (recovery point 5 minutes, recovery time 1 hour) and a nightly logical dump kept 35 days |
| Jobs | a Postgres job table polled with `FOR UPDATE SKIP LOCKED` (verification, exports, deletions); no separate broker |
| Objects | S3-compatible object storage in `eu-central` with server-side encryption: replays, dumps, cloud saves, exports |
| Keys | KMS (doc 15 §4.6): receipt keys `rk_`, token keys `tk_`, ranked keys `lk_`, invasion keys `ik_`; one key per calendar year, public keys embedded in the client |
| Hosting | containers in a managed Kubernetes cluster in `eu-central` across 2 zones; a CDN serves `/v1/econ/odds/*` and `/audit/*` |
| Observability | `tracing` logs and Prometheus metrics, EU-hosted; IP addresses truncated to /24 (IPv4) or /48 (IPv6) at ingest |

**API conventions.** Base path `/v1`. `Authorization: Bearer <access token>` on every endpoint except the public ones. JSON numbers are integers only; `u64` values are `"0x"` + 16 lowercase hex digits; 32-byte hashes, seeds and keys are 64 lowercase hex digits without a prefix; Ed25519 signatures are 128 lowercase hex digits (doc 15 §4.6 and §10.4 forms); every other fixed-length byte string, such as the 8-byte `content_hash16` prefix (§11.5) and the 16-byte `install_id` (§11.7), is 2 lowercase hex digits per byte without a prefix; `client_seed_hex` is 2 to 128 lowercase hex digits (§11.4); any other hex form is refused with `svc.err.bad_request`; times are Unix seconds. These forms bind every request and response of every service and every JSON example in another chapter that a service accepts, such as doc 14 §9.4's ranked submission (triage C-07). Every write except a run report and the `/v1/auth/*` calls carries `req_id`, a client-random `u64`: the service stores `(account_id, method, path, req_id) → (SHA-256 of the body, response)` for 7 days, returns the stored response for a repeat with the same body and `svc.err.idem_conflict` for a different body, so retries are safe. Ledger postings additionally use doc 15 §10.3's `idem_key`, whose `s` for an Archive purchase or a Transcription claim is this `req_id` in its hex form; a run report's idempotency is doc 15 §10.3 row `run` (`s = run_id`) alone. Errors: an HTTP status plus `{ "error": "<svc.err.* ID>", "retry_after_s": 0 }`. Rate limits per account: 10 reads per second (burst 30), 2 economy writes per second (burst 10); excess returns 429 with `svc.err.rate_limited`.

### 11.2 Accounts and authentication

- `POST /v1/auth/steam` `{ "ticket": "<hex of GetAuthTicketForWebApi("palimpsest-auth")>", "build": 1042 }` → `{ "account_id": "0x0123456789abcdef", "access_token": "pal1....", "expires_in": 3600, "refresh_token": "<64 hex>", "created": false, "region": "juris.region.eu", "minor": false }`. The service validates the ticket with the Steam Web API, creates the account on first sign-in, and applies doc 15 §8.1 region resolution.
- `POST /v1/auth/refresh` `{ "refresh_token": "…" }` → a new pair. Refresh tokens last 30 days and rotate on every use; reuse of a rotated token revokes the whole token family.
- Access token: `pal1.` ‖ base64url(canonical JSON `{"acct","exp","iat","kid","plat","scope"}`) ‖ `.` ‖ base64url(Ed25519 signature over `"PALIMPSEST/TOKEN/v1" ‖ 0x00 ‖ payload`).

### 11.3 Profiles and cloud saves

- `GET /v1/profile` → `{ "version": 37, "profile": { … } }`; `PUT /v1/profile/settings` with `If-Match: <version>` writes only client-owned settings. Unlocks, Codex, Echoes and balances are written only by `svc.econ` and `svc.profile` grant paths.
- `GET /v1/cloud/{key}` and `PUT /v1/cloud/{key}` with `If-Match` ETags, keys `profile` and `run` (doc 14's run save file), at most 16 MiB per object and 64 MiB per account. A `PUT` with a stale ETag returns 409 `svc.err.cloud_conflict`; the client keeps the save with the higher `save_seq`, then the higher boundary tick (doc 14 §12.3). Steam Auto-Cloud (doc 02 §12.2) also syncs the local save files; `svc.cloud` keeps the account's copy.

### 11.4 Economy API (`svc.econ`)

Doc 15 owns every rule; this section fixes the wire schemas. Types: `u64h` is `"0x"` + 16 lowercase hex digits; `h32` is 64 lowercase hex digits; `sigh` is 128 lowercase hex digits; `id` is a content ID; `int` is a JSON integer; `bool` is `true` or `false`; `T?` may be `null`; `[T]` is an array. Every object is closed (no other keys). `palimpsest/schemas/svc/econ_api.json` holds one JSON Schema per request and response, generated from these tables, and both the service and the client validate against it. It describes the service API, not content, so it lies outside `content/` and is not a doc 19 record schema.

Shared objects:

| Object | Fields |
|---|---|
| `Wallet` | `ink`, `quire`, `vellum`, `reliquary`, `quill`: int; `debt`: [{`asset`: `"INK"` \| `"QUIRE"` \| `"VELLUM"` \| `"RELIQUARY"`, `amount`: int}] (doc 15 §10.2 `econ_debt`) |
| `Caps` | `ink_play_24h_used`, `ink_play_24h_cap` (4000), `codex_xp_24h_used`, `codex_xp_24h_cap` (200000), `invasion_quires_24h_used`, `invasion_quires_24h_cap` (60): int (doc 15 §2.3) |
| `Receipt` | doc 15 §4.6: `v` int, `kind` string, `rid` u64h, `acct` u64h, `item` id?, `qty` int, `src` id?, `seq` int, `iat` int, `kid` string; plus `sig` sigh over doc 15's signed message. `wallet` receipts add `balances` (a `Wallet` without `debt`); `pity` receipts add `pity` [`Pity`], `spark` [{`banner` id, `points` int}], `reliquary_counters` {`c5` int, `c20` int}, `folio_exits` int (the lifetime count of qualified Folio exits, doc 15 §5.1) and `transcription` int (doc 15 §3.9.1) |
| `Pity` | `family` id; `top_since`, `mid_since` int; `top_guarantee`, `mid_guarantee` bool (doc 15 §3.4) |
| `Epoch` | `id` u64h; `banner` id; `period` int; `state` `"committed"` \| `"active"` \| `"revealed"`; `commitment` h32; `config_hash` h32; `next_nonce` int; `client_seed_hex` string? (2 to 128 lowercase hex digits, set once bound); `server_seed` h32? (set once revealed) (doc 15 §4.1) |
| `PullResult` | `nonce`, `u_tier`, `u_feat`, `idx` int; `tier` id (`banner.tier.*`); `entry` id; `featured` bool (featured or rate-up); `grant` {`kind` `"new"` \| `"fold"` \| `"vellum"`, `item` id, `fold` int, `vellum` int} (doc 15 §3.8) |
| `Grant` | exactly one of `asset` (`"INK"` \| `"QUIRE"` \| `"VELLUM"` \| `"RELIQUARY"` \| `"QUILL"`) or `item` (id), plus `qty` int |

Endpoints (errors are `svc.err.` IDs; every endpoint can also return `unauthorized`, `rate_limited`, `maintenance`, `version_too_old` and `internal`):

| Method and path | Request body | Response body | Endpoint errors |
|---|---|---|---|
| `GET /v1/econ/wallet` | none | {`wallet` Wallet, `caps` Caps, `transcription` int (doc 15 §3.9.1), `receipt` Receipt (kind `wallet`)} | |
| `GET /v1/econ/banners` | none | {`banners` [doc 15 §3.13 banner record plus `state` `"scheduled"` \| `"live"` \| `"ended"`], `pity` [Pity], `spark` [{`banner` id, `points` int}], `epochs` [Epoch]} | |
| `GET /v1/econ/odds/{banner_id}` (public) | none | {`banner` id, `rows` [{`entry` id, `tier` id, `base_pct4` int, `consolidated_pct4` int}]}, doc 15 §3.11 values in units of 0.0001% | `not_found` |
| `POST /v1/econ/fairness/epoch` | {`req_id` u64h, `banner` id} | {`epoch` Epoch, `pity_receipt` Receipt (kind `pity`)}: the open epoch, or a new `committed` one | `banner_ended`, `banner_not_started`, `requires_sku` |
| `POST /v1/econ/fairness/reveal` | {`req_id` u64h, `epoch` u64h} | {`epoch` Epoch} in state `revealed` | `epoch_mismatch` |
| `POST /v1/econ/fairness/client_seed` | {`req_id` u64h, `mode` `"fresh"` \| `"pinned"`, `client_seed_hex` string?} (required for `pinned`) | {`revealed` [Epoch]}, every `active` epoch revealed by the change | `bad_request` |
| `GET /v1/econ/fairness/epochs/{epoch_id}` | none | Epoch | `not_found` |
| `POST /v1/econ/inkwell/pull` | {`req_id` u64h, `banner` id, `count` 1 \| 10, `expect_cost` int, `epoch` u64h, `commitment` h32, `client_seed_hex` string?, `beginner_target` id?}; `client_seed_hex` is present exactly when the epoch is `committed` | {`epoch` {`id` u64h, `commitment` h32, `nonce_first` int, `nonce_last` int}, `results` [PullResult], `wallet` Wallet, `pity_after` Pity, `spark_after` int, `receipts` [Receipt]} | `insufficient_ink`, `banner_ended`, `banner_not_started`, `requires_sku`, `beginner_target_required`, `beginner_closed`, `epoch_mismatch`, `cost_changed` |
| `POST /v1/econ/reliquary/open` | {`req_id` u64h, `reliquaries` [u64h] (1 to 10 unopened instance IDs), `epoch` u64h, `commitment` h32, `client_seed_hex` string?} | {`results` [{`nonce` int, `reliquary` u64h, `u_rarity` int, `u_upgrade` int?, `rarity` id, `item_seed` u64h, `keepsake` {`vault_id` u64h, `item` object}}], `counters_after` {`c5` int, `c20` int}, `wallet` Wallet, `receipts` [Receipt]}; `item` is the Canon §13.9 instance in doc 19's schema with `uid` 0; `u_upgrade` is null when draw 2 was the discarded `uniform(1)` | `vault_full`, `not_found`, `epoch_mismatch`, `forbidden` (`juris.flag.minor_inkwell_block`) |
| `POST /v1/econ/archive/purchase` | {`req_id` u64h, `entry` id, `shelf` `"unlock"` \| `"fold"`, `expect_cost` int} | {`grant` Grant, `wallet` Wallet, `receipts` [Receipt]} | `insufficient_vellum`, `requires_sku`, `cost_changed`, `not_found` |
| `POST /v1/econ/archive/transcribe` | {`req_id` u64h, `entry` id (a Script, `bundle.script.<name>`), `expect_cost` int} | {`grant` Grant, `transcription_after` int, `receipts` [Receipt]}: the doc 15 §3.9.1 claim, costing 16 for a Rubricated and 32 for an Illuminated Script; it touches no pity, spark, Fold or Vellum | `insufficient_transcription`, `requires_sku`, `cost_changed`, `not_found` (not a Script, or outside the build's content set), `forbidden` (already owned) |
| `GET /v1/econ/codex` | none | {`active` id, `codexes` [{`pass` id, `xp` int, `tier` int, `premium` bool}], `reserve` int, `coda_xp` int, `quills` int} | |
| `POST /v1/econ/codex/activate` | {`req_id` u64h, `pass` id} | {`active` id} (doc 15 §6.5 switching) | `not_found` |
| `POST /v1/econ/codex/redeem_quill` | {`req_id` u64h, `pass` id} | {`pass` id, `premium` true, `grants` [Grant], `wallet` Wallet, `receipts` [Receipt]}, including late-purchase grants (doc 15 §6.5) | `bad_request` (no Quill held, or track already owned) |
| `POST /v1/econ/runs/report` | the doc 15 §10.4 run report, without `req_id` | {`state` `"accepted"` \| `"held_for_host"` \| `"held_for_review"`, `grants` [Grant], `codex` {`pass` id, `xp_base` int, `xp_bonus` int, `xp_after` int, `tier_after` int, `reserve_after` int}, `wallet` Wallet, `caps` Caps, `receipts` [Receipt], `verify_sampled` bool} | `report_malformed`, `sim_version_unknown`, `content_mismatch`, `host_mismatch` |
| `POST /v1/econ/drafts/sync` | {`req_id` u64h, `reports` [run report], `drafts` [{`draft_id` u64h, `banner` id, `client_seed_hex` string, `count` int, `queued_at` int}]} | {`reports` [{`run_id` u64h, `state` string, `error` string?}], `drafts` [{`draft_id` u64h, `state` `"resolved"` \| `"cancelled"`, `results` [PullResult], `error` string?}], `wallet` Wallet}; reports first, then drafts in queue order (doc 15 §4.5) | per entry |
| `GET /v1/econ/receipts?since_seq=N` | none | {`receipts` [Receipt], `next_since_seq` int}, at most 500 per page | |
| `GET /v1/econ/history/pulls?banner=&cursor=` | none | {`rows` [doc 15 §3.4 `PullRecord` fields plus `utc` int], `cursor` string?}, 50 rows per page | |
| `GET /v1/econ/store/catalog` | none | {`currency` string (ISO 4217), `skus` [{`sku` id, `price_minor` int, `owned` bool, `bundle` id, `requires_sku` id?}]} (doc 15 §7) | |
| `POST /v1/econ/store/purchase/init` | {`req_id` u64h, `sku` id, `platform` `"steam"`} | {`order_id` u64h, `platform_params` {`steam_orderid` u64h}} | `purchase_locked`, `minor_cap_tx`, `minor_cap_month`, `minor_store_blocked`, `forbidden` (already owned) |
| `POST /v1/econ/store/purchase/finalize` | {`req_id` u64h, `order_id` u64h} | {`state` `"granted"` \| `"failed"`, `grants` [Grant], `receipts` [Receipt]} | `platform_verify_failed` |
| `POST /v1/econ/store/refund` | {`req_id` u64h, `order_id` u64h, `reason` `"policy"` \| `"withdrawal"` \| `"minor"`} | {`state` `"refunded"` \| `"refused"`, `revocations` [Grant], `receipts` [Receipt]} (doc 15 §7.4) | `forbidden` (outside the window) |
| `GET /audit/{YYYY-MM-DD}` (public) | none | {`date` string, `root` h32, `leaves` int, `kid` string, `sig` sigh} (doc 15 §4.4) | `not_found` |

Pull, request and response (doc 15 acceptance test 8 and §3.14 example F: `server_seed` bytes `00..1f`, client seed ASCII `errata`, nonce 0, `u_tier` 344, `u_feat` 3861, rate-up Rubricated, `uniform(3) = 1`; the receipt's `iat` lies inside `banner.echo.p001`'s window `[1792886400, 1795305600)`; signatures elided):

```json
{ "req_id": "0x5f1e00000000002a", "banner": "banner.echo.p001", "count": 1, "expect_cost": 160,
  "epoch": "0x0000000000000011",
  "commitment": "630dcd2966c4336691125448bbb25b4ff412a49c732db2c8abc1b8581bd710dd",
  "client_seed_hex": "657272617461", "beginner_target": null }
```

```json
{ "epoch": { "id": "0x0000000000000011", "commitment": "630dcd2966c4336691125448bbb25b4ff412a49c732db2c8abc1b8581bd710dd",
             "nonce_first": 0, "nonce_last": 0 },
  "results": [ { "nonce": 0, "u_tier": 344, "u_feat": 3861, "idx": 1, "tier": "banner.tier.rubricated",
                 "entry": "echo.lan_ninth_peak", "featured": true,
                 "grant": { "kind": "new", "item": "echo.lan_ninth_peak", "fold": 0, "vellum": 0 } } ],
  "wallet": { "ink": 2240, "quire": 310, "vellum": 12, "reliquary": 1, "quill": 0, "debt": [] },
  "pity_after": { "family": "banner.family.echo", "top_since": 1, "mid_since": 0,
                  "top_guarantee": false, "mid_guarantee": false },
  "spark_after": 1,
  "receipts": [ { "acct": "0x0123456789abcdef", "iat": 1792890000, "item": "echo.lan_ninth_peak", "kid": "rk_2026a",
                  "kind": "grant", "qty": 1, "rid": "0x00000000000003ea", "seq": 8, "src": "banner.echo.p001", "v": 1,
                  "sig": "<128 hex digits>" } ] }
```

The wallet held 2,400 Ink before the pull. Run report: the request body is doc 15 §10.4's example verbatim (a solo Standard death in Folio 4, `run_id` `0x00000000075bcd15`). For an account whose Codex XP, catch-up reserve and 24-hour windows were all 0 and whose wallet held 2,240 Ink and 310 Quires, the response is:

```json
{ "state": "accepted",
  "grants": [ { "asset": "INK", "qty": 95 }, { "asset": "QUIRE", "qty": 14 } ],
  "codex": { "pass": "pass.codex_001", "xp_base": 5025, "xp_bonus": 0, "xp_after": 5025, "tier_after": 0, "reserve_after": 0 },
  "wallet": { "ink": 2335, "quire": 324, "vellum": 12, "reliquary": 1, "quill": 0, "debt": [] },
  "caps": { "ink_play_24h_used": 95, "ink_play_24h_cap": 4000, "codex_xp_24h_used": 5025, "codex_xp_24h_cap": 200000,
            "invasion_quires_24h_used": 0, "invasion_quires_24h_cap": 60 },
  "receipts": [], "verify_sampled": false }
```

`verify_sampled` is true when `mix64(run_id) mod 100 < 5`, when the report reaches 90% of any cap, or when the account has `econ_review = true` (doc 15 §10.4); for `0x00000000075bcd15`, `mix64` mod 100 is 84. A co-op report that arrives before the host's returns `held_for_host` and posts nothing until the host's report matches or 7 days pass (doc 15 §10.4 step 5). Economy error IDs map doc 15's codes one to one: `svc.err.insufficient_ink`, `svc.err.insufficient_vellum`, `svc.err.insufficient_transcription`, `svc.err.banner_ended`, `svc.err.banner_not_started`, `svc.err.requires_sku`, `svc.err.beginner_target_required`, `svc.err.beginner_closed`, `svc.err.epoch_mismatch`, `svc.err.vault_full`, `svc.err.cost_changed` (when `expect_cost` differs), `svc.err.idem_conflict` (same `req_id`, different body), `svc.err.report_malformed`, `svc.err.content_mismatch`, `svc.err.host_mismatch` (header or `state_hash` disagrees with the host's report), `svc.err.purchase_locked`, `svc.err.minor_cap_tx`, `svc.err.minor_cap_month`, `svc.err.minor_store_blocked`, `svc.err.platform_verify_failed`. Caps never produce an error: they clip grants (doc 15 §2.3).

### 11.5 Invasion beacons and tickets

- Every online session uses a Steam lobby (§3.3, §3.5); `svc.lobby` serves only invasion beacons, tickets and outcomes.
- Invasion beacons (§9.13): `PUT /v1/invasions/beacons/{run_id}` `{ "req_id": "0x…", "sim_version": 1, "proto": 1, "content_hash16": "9f00000000000000", "grade": 4, "host_steamid": "0x0110000100000001", "ping_location": "<Steam ping location>", "accounts": ["0x0123456789abcdef"] }` → `{ "ttl_s": 45 }`; the caller's account must be in `accounts`, and a beacon not refreshed within 45 s disappears. `DELETE /v1/invasions/beacons/{run_id}` → `{}`. `GET /v1/invasions/beacons?sim_version=1&proto=1&content_hash16=…&grade_min=2&grade_max=6&cursor=` → `{ "beacons": [ { "run_id": "0x…", "grade": 4, "party_size": 2, "host_steamid": "0x…", "ping_location": "…" } ], "cursor": null }`, at most 50 per page, never listing a beacon whose party blocked the caller or a beacon to an account with invasions disabled.
- `POST /v1/invasions/ticket` `{ "req_id", "host_run_id": "0x…" }` → `{ "exp": 1791763260, "sig": "<128 hex>" }`, an Ed25519 signature by an `ik_` key over `"PALIMPSEST/INVADE/v1" ‖ 0x00 ‖ le64(invader account_id) ‖ le64(host_run_id) ‖ le64(exp)`, valid for 60 s. Refusals: `svc.err.not_found` (no live beacon), `svc.err.forbidden` (invasions disabled for the account, or a party member blocked it within 30 days).
- `POST /v1/invasions/outcome` `{ "req_id", "host_run_id": "0x…", "invader": "0x…", "start_tick": 41000, "end_tick": 53000, "reason": 3 }` from the host and from the invader → `{ "matched": true }`. Both reports must arrive within 900 s of each other; a report without its pair grants nothing. The example (12,000 ticks, Redacted) pays the invader 10 Quires under doc 15 §2.4.

### 11.6 Leaderboards and ranked tokens

- `POST /v1/ranked/tokens` `{ "req_id", "mode": "mode.daily", "period": "2026-10-11", "run_id": "0x00000000075bcd15", "accounts": ["0x…", "0x…"], "sim_version": 1, "content_hash": "<h32>" }` → `{ "run_seed": "0xc6f2e12ddca712ab", "token": "<h32>", "exp": 1791914400, "sig": "<sigh>" }`. `token` is 32 bytes from the service's OS CSPRNG, stored server-side with the request fields, `run_seed` and `exp`; it is doc 14's `RankedState.ranked_token`. `sig` is an Ed25519 signature by an `lk_` key over `"PALIMPSEST/RANKED/v1" ‖ 0x00 ‖ token ‖ le64(run_id) ‖ le64(run_seed) ‖ le64(exp)`, so every party member verifies the host's token offline against the embedded public keys. A token lives 48 hours: issued at 2026-10-11 18:00:00 UTC (1791741600), it expires at 1791914400. `period` is the UTC date for `mode.daily`, the ISO week for `mode.weekly_gauntlet`, and absent for other ranked modes; it is validated as strictly as Canon §3.6 requires (a real calendar date, an uppercase `W`, a week the ISO year has) and must be the current UTC date or ISO week, else `svc.err.bad_request`. A drop-in joiner of a ranked run requests a token with the run's `run_id` and its own account only; for `mode.daily` this consumes that account's attempt. Refusals: `svc.err.attempt_used`, `svc.err.sim_version_unknown`, `svc.err.content_hash_unknown`, and `svc.err.forbidden` for a run with `mode.invasion_open`.
- `POST /v1/leaderboards/submissions` with the doc 14 §9.4 JSON → `{ "submission_id", "upload_url" }`; the client `PUT`s the replay to `upload_url` (valid 900 s); `GET /v1/leaderboards/submissions/{id}` → `{ "state": "queued" | "accepted" | "rejected", "error": null }`.
- `GET /v1/leaderboards/runs/{run_id}` → `{ "state": "none" | "queued" | "accepted" | "rejected", "submission_id": "0x…" | null }`, used by non-host members (§10.2).
- `GET /v1/leaderboards/{board}?period=&party_size=&cursor=` → 50 entries per page with rank, party personas, score, `end_tick`, and a replay link for the top 100.

### 11.7 Telemetry and crash reports

Both are opt-in and keyed by the random install ID (doc 02 §17.4). `POST /v1/telemetry/batch` `{ "install_id": "<32 hex>", "build": 1042, "events": [ … ] }`, at most 64 KiB, every 300 s while enabled; a disabled client sends nothing and keeps nothing, and the endpoint drops IP addresses at ingest (doc 02 §17.4). Network counters in each session summary: median and 95th-percentile RTT, loss, corrections per minute, 99th-percentile correction distance, substituted commands, late packets, snapshot budget overruns, desyncs, migrations by mode. Each run summary carries doc 02 §17.4's mode, Folio reached, end cause and party size and, per Folio (doc 01's KPIs, triage C-14): genre, entry and exit ticks, the offered doors and the committed door, the maximum equipped `C` per slot at entry, a Disconnected-weapon flag per slot, committed Rewrites, and NPC kills split into systemic (doc 01 §10.3's rule) and other. In co-op only the host's client sends the run summary, because only the host holds every slot's state, and it omits the per-slot entries of every slot whose account is minor-flagged (`Hello.minor`). A minor-flagged account sends no telemetry at all, whatever the setting (§14). `POST /v1/crash` takes a minidump or desync dump of at most 8 MiB.

### 11.8 Workshop

Steam Workshop (doc 02 §13.4) is the only mod channel, because Steam is the only storefront (DECISIONS 112); `svc.workshop` runs nothing (§11.1). A session's simulation-affecting mods must match exactly, through `content_hash` (Canon §2.4); presentation-only mods may differ between peers (DECISIONS 87).

### 11.9 Status and versioning

`GET /v1/status` (public) → `{ "min_build": 1042, "sim_versions": [1], "maintenance": null, "notice_key": null }`. A client below `min_build` keeps every offline feature and refuses online ones with `svc.err.version_too_old`.

---

## 12. Anti-cheat boundaries

PALIMPSEST ships no kernel or client anti-cheat (doc 02 §14, Steam Deck Verified). Integrity rests on authority, re-simulation and bounded rewards.

| Boundary | Trusted | Defense |
|---|---|---|
| Client → host | nothing | commands sanitized (§5.8); `view_tick` clamped to measured lag (§5.5); intents validated by their owning document; rate limits and violation kicks |
| Host → party clients | presentation data, not secrets (decision 9) | instanced items, other inventories, unknown item properties and out-of-interest entities are never replicated (§6.6, doc 10 §2.8), but every party member holds the run seed and the CIL and can derive them with a modified client; inside a party this protects bandwidth and honest presentation only |
| Host → invader, and party → invader | nothing beyond its own view | the invader gets no CIL, knowledge or inventories, and snapshots under PvP culling; party clients get no slot-4 commands while it is present (§6.14 sealing), so no client of either side, modified or not, receives a culled opponent's position; only the host itself holds both, which is why a cheating host affects only its own session; the invader holds the run seed for terrain only |
| Host's own session | the host, for its own party | a host that edits its process changes only its party's game; its ranked submission fails re-simulation, and its run reports earn at most doc 15's per-run and 24-hour caps |
| Ranked boards | nothing until verified | full re-simulation of the CIL from tick 0 and cross-checked input-log hashes (§10.2) |
| Economy | platform tickets, server ledger | doc 15 caps, idempotent grants, 5% re-simulation sampling, co-op run reports matched against the host's report (`host_mismatch`, doc 15 §10.4) |
| Invasions (PvP) | nothing | signed tickets, no simulation-affecting mod on either side (equal `content_hash`), PvP culling, lag-compensation cap of 16 ticks, reports to `svc.moderation`; statistical flags (aim-snap rate, reaction time) go to human review only; a cheating host affects only invasions into its own session, whose invader rewards doc 15 caps at 60 Quires per 24 hours |

Input automation inside one's own co-op session is not detected. Sanctions are per account and online only: hidden from boards, barred from invasions, or both; offline play is never restricted.

---

## 13. Offline mode

| Feature | Offline behavior |
|---|---|
| Solo, every mode except ranked submission | full; Daily and Weekly seeds are computed locally and play as practice |
| Co-op | LAN and direct IP over `net.route.direct`; Steam P2P works while Steam itself is online |
| Account economy | doc 15 §4.5 Sealed Drafts and queued run reports; end-of-service rules (doc 15 §11.3) after shutdown |
| Ownership | signed receipts (doc 15 §4.6) and platform ownership checks |
| Replays | written and viewable locally; uploaded later only on request |
| Leaderboards, invasions, cloud saves, telemetry, crash upload | unavailable; telemetry is not queued |

A session never waits on a service: service calls run on the net or worker thread with a 10 s timeout, the simulation never blocks on one, and a failed call affects only its own feature.

---

## 14. Privacy and data retention

| Data | Purpose | Legal basis (GDPR Art. 6(1)) | Retention |
|---|---|---|---|
| Account ID, the linked SteamID, region, minor flag | sign-in, ownership, compliance | (b) contract | until deletion + 30 days |
| Ledger, receipts, audit log | economy and fairness proofs | (c) legal obligation, (b) | 5 years, pseudonymous (doc 15 §4.4); daily roots forever |
| Profile and cloud saves | progression | (b) | until deletion; accounts inactive 3 years are notified, then deleted 90 days later |
| Accepted ranked replays and entries | boards | (b) | while on a board; top-1,000 per board period forever, persona replaced by "Erased Erratum" after account deletion |
| Rejected or unranked submissions | anti-cheat | (f) legitimate interest | 30 days |
| Telemetry | balance and stability | (a) consent | raw 180 days (doc 02 §17.4), aggregates forever |
| Crash and desync dumps | stability | (a) consent | 90 days |
| Moderation reports with the last 100 text lines held by the reporter (doc 15 §8.5) | safety | (f) | 365 days |
| Invasion beacons: account IDs, SteamID, Steam ping location | invasion matchmaking | (b) | until the window closes or 45 s after the last refresh (§9.13) |
| Service logs | security | (f) | 14 days, IPs truncated (§11.1) |
| Chat, voice, game traffic | none | — | never stored by any service |

`GET /v1/account/export` returns a JSON archive of every row above within 30 days. `DELETE /v1/account` starts a 30-day grace period, then removes the SteamID link, profile, cloud saves and non-board replays, and pseudonymizes what retention requires. Minor-flagged accounts never send telemetry, whatever the setting. No advertising identifiers or third-party analytics exist anywhere.

---

## 15. Bandwidth budgets

Budgets count message bytes plus `DATAGRAM_OVERHEAD` (48 bytes) per datagram.

| Flow | Per client | Notes |
|---|---|---|
| Snapshots, host → client | 12,000 B/s (96 kbit/s) | 30 datagrams/s; 10,560 B/s payload; mean 352 B per snapshot. Combat estimate 329 B: 16-byte header; `ENTITIES` with 24 walking updates at 74 bits (226 B with framing); `EVENTS` with 6 `ev.damage_dealt` records at 80 bits (63 B); `OWNER` 16 B; `HUD` 8 B. Calm estimate 99 B (6 updates, no events) |
| World edits | 1,400 B/s average | keyframes and Folio state are exempt and occur at Folio entry |
| CIL | 1,900 B/s | 10 messages/s of 142 B (10-byte header plus 6 ticks × 22 B) plus 48 B overhead each |
| Control (owner, party, run, hashes, knowledge, chat) | 700 B/s | `PartyState` of 70 B with 4 Active slots and no roster change, at 2 Hz (236 B/s with overhead), `HashReport` of 40 B at 1 Hz (88 B/s), and `OwnerDelta`, `RunView`, `KnowledgeDelta` and chat within the remaining 376 B/s average |
| **Down total** | **16,000 B/s (128 kbit/s)** | listen host with 3 clients: 48,000 B/s = 384 kbit/s up (doc 02 `perf.deck.net.host_kbps`); with 3 clients and an invader each peer gets `floor_div(48000, 4) = 12,000` B/s, so `SNAP_RATE = 6,560` and refill 218 B; dedicated with 4 clients and an invader: 80,000 B/s |
| CIL backlog | 8,000 B/s while not in game; 1,000 B/s in game | the in-game rate comes out of that client's `SNAP_RATE` (9,560 B/s meanwhile), so the total stays 16,000 B/s |
| Inputs, client → host | 6,480 B/s (51.8 kbit/s) | 60 packets/s of 60 B; host down from 3 clients 155.5 kbit/s |
| Voice while talking | 5,600 B/s per forwarded talker | 50 frames/s of 64 B (60 B at 24 kbit/s plus a 4 B header) plus overhead; outside the game budgets; at most 2 talkers per receiver, so a listen host's worst case is 3 × 2 × 5,600 = 33,600 B/s up, dropped first under congestion (§9.12) |
| Folio entry burst | ≤ 64 KiB per tick | `FolioState`, `Keyframe` (12,338 B of entity records for 300 creates at 329 bits), `ChunkData` only on mismatch |

---

## 16. Network test plan

**NetSim.** A link conditioner inside `net.route.in_process` delays, drops, duplicates and caps datagrams per direction, with `rng = Rng::from_seed64(mix64(scenario_seed ^ link_id))` (outside the simulation) and exactly 4 calls per datagram whatever the outcome:

```rust
fn condition(l: &mut Link, dg: Datagram, now_ms: u64) {
    l.burst = if l.burst { !l.rng.roll_bp(l.sc.burst_leave_bp) } else { l.rng.roll_bp(l.sc.burst_enter_bp) };  // call 1
    let loss = l.rng.roll_bp(l.sc.loss_bp);                                                                  // call 2
    let dup  = l.rng.roll_bp(l.sc.dup_bp);                                                                   // call 3
    let at   = now_ms + l.sc.rtt_ms / 2 + l.rng.range(l.sc.jitter_ms as u64 + 1);                           // call 4
    if l.burst || loss { return; }                          // in the burst state every datagram is lost
    l.enqueue(at, dg.clone());                              // delivery queue ordered by (at, enqueue order)
    if dup { l.enqueue(at + 1, dg); }
    // cap: a token bucket of cap B/s with capacity cap / 10 drains the delivery queue in order;
    // datagrams waiting beyond 65,536 queued bytes are tail-dropped
}
```

| Scenario | RTT ms | One-way jitter ms | Loss Bp | Burst enter / leave Bp | Duplicate Bp | Cap B/s |
|---|---|---|---|---|---|---|
| `lan` | 1 | 0 | 0 | 0 / 10000 | 0 | none |
| `good` | 40 | 4 | 10 | 0 / 10000 | 0 | none |
| `typical` | 100 | 10 | 100 | 50 / 3000 | 10 | none |
| `poor` | 200 | 30 | 300 | 100 / 2000 | 50 | 64,000 |
| `bad` | 350 | 60 | 800 | 200 / 1500 | 100 | 32,000 |

In the burst state every datagram is lost. **Nightly runs:** 4 Pilgrim bots (doc 02 §16) for 18,000 ticks per scenario, one with a Shadow. Pass criteria:

| Metric | lan, good | typical | poor | bad |
|---|---|---|---|---|
| Desyncs, CIL differences between clients, failed `verify` | 0 | 0 | 0 | 0 |
| Substituted commands | ≤ 0.5% | ≤ 2% | ≤ 8% | reported only |
| Corrections per minute, excluding knockback and push | ≤ 2 | ≤ 2 | ≤ 6 | reported only |
| 99th-percentile correction distance | ≤ 100 mm | ≤ 250 mm | ≤ 600 mm | reported only |
| Entity-frames held past their newest sample | ≤ 0.5% | ≤ 1% | ≤ 5% | reported only |
| Snapshot payload over 1,150 B, or a 600-tick window over 16,000 B/s | 0 | 0 | 0 | 0 |

**Scripted cases:** host killed at a random tick (hot, warm and cold modes each resume with equal hashes); host killed during an invasion (cold, invasion reason 6, sealed slot-4 records never reach a party client); a client Dormant for 20 s then resuming at the next Folio entry; a drop-in at Folio 3 with a full CIL backlog; a trade under 8% loss; an invasion with PvP culling; a generation mismatch forced by a corrupted chunk; a kick vote in a full party. **Soak:** a dedicated server cycles 4-bot runs for 24 h with flat memory. **Live:** a weekly playtest over Steam Datagram Relay between `eu-central` and `us-east` and between `eu-central` and `ap-southeast`. **Services:** 10,000 concurrent simulated clients; `POST /v1/econ/inkwell/pull` 99th-percentile latency ≤ 300 ms; zero ledger invariant violations. **Fuzzing:** every decoder of §5.1, §6.8, §6.11, §6.12 and §6.14 (doc 02 §16 item 5).

---

## Exports

All 155 IDs below follow Canon §1.1, sit in `net.` and `svc.` (Canon §19, doc 16), and have no `fnv1a32` collision among themselves or with any ID-shaped token of the other spec documents on disk (5,441 distinct tokens at the last check, 2026-10-11) (`fnv1a32("net.ch.snapshot") = 0xe092c335`, `fnv1a32("svc.econ") = 0x71dea0e0`). `net.route.relay`, `svc.relay` and `svc.workshop` are reserved: Steam is the only storefront (DECISIONS 112) and no other platform ships through 1.0 (DECISIONS 98), so no session or service uses them (§3.5, §11.1).

| Prefix | IDs |
|---|---|
| `net.params.` (1) | `net.params.default` |
| `net.topo.` (3) | `net.topo.solo`, `net.topo.listen`, `net.topo.dedicated` |
| `net.route.` (4) | `net.route.sdr`, `net.route.direct`, `net.route.relay`, `net.route.in_process` |
| `net.ch.` (6) | `net.ch.control`, `net.ch.input`, `net.ch.snapshot`, `net.ch.world`, `net.ch.cil`, `net.ch.voice` |
| `net.msg.` (40) | `net.msg.hello`, `net.msg.welcome`, `net.msg.reject`, `net.msg.ping`, `net.msg.pong`, `net.msg.bye`, `net.msg.input`, `net.msg.intent`, `net.msg.snapshot`, `net.msg.keyframe`, `net.msg.world_edits`, `net.msg.folio_enter`, `net.msg.folio_state`, `net.msg.chunk_data`, `net.msg.folio_ready`, `net.msg.cil`, `net.msg.hash_report`, `net.msg.section_hash_request`, `net.msg.section_hashes`, `net.msg.full_state_request`, `net.msg.full_state`, `net.msg.owner_delta`, `net.msg.party_state`, `net.msg.run_state`, `net.msg.knowledge_delta`, `net.msg.chat`, `net.msg.voice`, `net.msg.join_request`, `net.msg.join_ready`, `net.msg.migrate_announce`, `net.msg.kick_vote`, `net.msg.lobby_settings`, `net.msg.lobby_pick`, `net.msg.lobby_ready`, `net.msg.run_header`, `net.msg.challenge`, `net.msg.cil_unseal`, `net.msg.cil_backlog`, `net.msg.voice_mute`, `net.msg.scribe_text` |
| `net.reject.` (14) | `net.reject.proto_mismatch`, `net.reject.sim_version_mismatch`, `net.reject.content_mismatch`, `net.reject.auth_failed`, `net.reject.wrong_password`, `net.reject.banned`, `net.reject.party_full`, `net.reject.run_locked`, `net.reject.protocol_violation`, `net.reject.kicked`, `net.reject.timeout`, `net.reject.invasion_closed`, `net.reject.host_left`, `net.reject.session_ended` |
| `net.cstate.` (9) | `net.cstate.disconnected`, `net.cstate.connecting`, `net.cstate.handshaking`, `net.cstate.lobby`, `net.cstate.awaiting_folio`, `net.cstate.loading`, `net.cstate.playing`, `net.cstate.stalled`, `net.cstate.migrating` |
| `net.peer.` (6) | `net.peer.pending`, `net.peer.lobby`, `net.peer.waiting`, `net.peer.loading`, `net.peer.in_game`, `net.peer.lost` |
| `net.migrate.` (3) | `net.migrate.hot`, `net.migrate.warm`, `net.migrate.cold` |
| `net.trade.` (6) | `net.trade.idle`, `net.trade.proposed`, `net.trade.open`, `net.trade.locked`, `net.trade.both_locked`, `net.trade.executed` |
| `net.lobby.` (3) | `net.lobby.private`, `net.lobby.friends`, `net.lobby.public` |
| `net.voice.` (3) | `net.voice.off`, `net.voice.party`, `net.voice.proximity` |
| `net.session.` (6) | `net.session.slot_join`, `net.session.slot_dormant`, `net.session.slot_resume`, `net.session.slot_departed`, `net.session.host_changed`, `net.session.folio_restart` |
| `net.netsim.` (5) | `net.netsim.lan`, `net.netsim.good`, `net.netsim.typical`, `net.netsim.poor`, `net.netsim.bad` |
| `svc.` (13) | `svc.auth`, `svc.profile`, `svc.econ`, `svc.lobby`, `svc.relay`, `svc.leaderboard`, `svc.verify`, `svc.telemetry`, `svc.crash`, `svc.cloud`, `svc.workshop`, `svc.status`, `svc.moderation` |
| `svc.err.` (34) | `svc.err.bad_request`, `svc.err.unauthorized`, `svc.err.forbidden`, `svc.err.not_found`, `svc.err.rate_limited`, `svc.err.version_too_old`, `svc.err.maintenance`, `svc.err.internal`, `svc.err.insufficient_ink`, `svc.err.insufficient_vellum`, `svc.err.insufficient_transcription`, `svc.err.banner_ended`, `svc.err.banner_not_started`, `svc.err.requires_sku`, `svc.err.beginner_target_required`, `svc.err.beginner_closed`, `svc.err.epoch_mismatch`, `svc.err.vault_full`, `svc.err.cost_changed`, `svc.err.idem_conflict`, `svc.err.report_malformed`, `svc.err.content_mismatch`, `svc.err.host_mismatch`, `svc.err.purchase_locked`, `svc.err.minor_cap_tx`, `svc.err.minor_cap_month`, `svc.err.minor_store_blocked`, `svc.err.platform_verify_failed`, `svc.err.cloud_conflict`, `svc.err.attempt_used`, `svc.err.token_invalid`, `svc.err.replay_mismatch`, `svc.err.sim_version_unknown`, `svc.err.content_hash_unknown` |

Non-ID interfaces: the `Transport` trait, `reconstruct_tick`, the message table of §3.7 and every layout in §5.1, §6.8 (with the `HUD` light byte and `StealthWire`), §6.11, §6.12, §6.14; `MoverSnap` and `MoverParamsView`; `Player` (the semantics of doc 03's `player` component); `SessionIntent`; `LobbySettings`, `LobbyPick` and `JoinCharacter`; `MigrateAnnounce`; `VoiceMute`; `PartyState`, `SlotSummary`, `RunView`; the intent kinds of §5.7; `ReplayFile`; the HTTP API of §11 with the economy schemas of §11.4 (`palimpsest/schemas/svc/econ_api.json`); the content record `net.params.default`; the behavior of `st.downed` and `st.ghost` (Canon §11.1, owner 16) and the emission of the downed, revive, death, ghost and resurrection events whose IDs are canon's and whose payloads doc 03 §14.2 registers (Imports).

## Interface assumptions

Only open items are listed; every other dependency is met on disk and recorded under Imports.

| Doc | Assumed to provide |
|---|---|
| 02 | no Epic Games Store build (DECISIONS 112): the EOS SDK row, feature `eos` and the EOS backend of `crate.platform`, §14's Epic Games Store bullet, `flag.net.crossplay` and the `opus` row's reason naming Epic players removed; §13.2's Workshop distribution through Steam Workshop, because `svc.workshop` runs nothing (§11.1, §11.8) |
| 09 | `StealthHud.steps` holding the hostile footsteps heard in the last 30 ticks, as rings do (doc 09 §8.3 gives the footstep indicator no lifetime), so §6.8's cap keeps the 4 newest |
| 12 | the `net_always` flag on exit doors (§6.6 rule 3) |
| 13 | `net_always` on Editor bosses during `BossFight` and on hunters (§6.6 rule 3) |
| 15 | §10.3's idempotency source `s` for an Archive purchase and for a Transcription claim being the request's `req_id` in its hex form (§11.1), where §10.3 says "client request UUID"; §4.6's `pity` receipt fields named `folio_exits` and `transcription`, as §11.4 writes them |
| 17 | stalled and migration overlays, stale-entity rendering, the kick vote, the lobby browser, voice settings with per-member voice unmute, the cosmetic-difference notice; drawing the `STEALTH` section with its caps of 4 rings, 8 marks and 4 steps |
| 18 | `loc.` keys for every `net.reject.*` and `svc.err.*` ID |

## Canon gaps

None is open against canon-1.11. The closed numbers below stay reserved, because other chapters cite this chapter's gap numbers:

- Adopted by canon-1.11: G2 (intents in the replay log, §2.4), G3 (Redaction while downed, §17.9), G4 (the substitute is not synthesized input, §11.8), G5 (named state boundaries, §2.2), G7 (`run_seed` in the run header, §2.4), G10 (events registered by doc 03, §18; the trade codes are in doc 03 §14.2), G11 (Lucre between party members only as dropped piles, §14) and G12 (downed rules in §9.4, §11.1).
- This chapter's own rules (triage G-175, G-209): G6 (§8 item 4), G17 (§6.14 sealing) and G19 (§3.8 `run_id`, §11.1 and §11.4 error and `req_id` mapping).
- Resolved on disk: G1 (header), G8, G13, G14, G15 and G20 (doc 14 follows §9.3 and §11.1), G18 (doc 02 §14 names the `svc.lobby` beacon search, triage C-13) and G21 (doc 02 §17.4 carries the per-Folio run summary and bars every minor-flagged account, triage C-14).

Open items, which are not canon gaps:

- **O1. Invader interest filter.** Doc 09 §14.5 sends the invader's client an Erratum's transform only while the invader perceives it (`V ≥ 300` with invader awareness), heard it within the last 60 ticks (position quantized to 4,000 mm) or a `fac.editors` source marked it; §6.6 rule 2 here uses the symmetric 4-ray visibility test with a 16,000 mm near radius and a 30-tick grace. The triage has no ruling on the difference. Until one lands, §6.6 rule 2 governs replication in both directions and doc 09's filter is not applied.
- **O2. DECISIONS 96 and 104.** Asynchronous traces (opt-in marginal notes and ink-stain death echoes in other players' Folios of the same seed or genre) and opt-in streaming audience votes both land in this chapter. Neither the trace service nor the vote path is specified yet; doc 21 schedules traces for Update 1 and votes for Update 2.

## Acceptance tests

1. **Input packet.** Encoding the §5.1 example yields exactly the 37 bytes shown; decoding them yields ticks 1198, 1199, 1200 with yaws 16384, 16400, 16416, `action_seq` 41, 41, 42 and `view_tick` 1184, 1185, 1186.
2. **Wire ticks.** A receiver at tick `0x1_0000_0005` reconstructs wire tick `0xFFFFFFFE` as `0x0_FFFF_FFFE`.
3. **Substitute.** Last applied buttons `ATTACK_PRIMARY | CROUCH | SPRINT` with `QUICKBAR_INDEX` 5 give a substitute with buttons 163,856, move (0, 0), the same yaw and pitch, and `view_tick = t`; the simulation derives no press from it.
4. **View clamp.** The five rows of the §5.5 table reproduce: 4982 → 4984, 4990 → 4990, 4970 → 4985, 4960 → 4984, 5003 → 5000.
5. **Target slack.** Spreads 0, 3, 6, 20 give 2, 4, 5, 8.
6. **Tick controller.** From `slack_ewma = 2000`, target 2, six snapshots with slack 0 give 1750, 1532, 1341, 1174, 1028, 900, and the period becomes 15,873 µs at the sixth.
7. **Snapshot cadence.** Over ticks 0 to 9 the host emits snapshots for boundaries 1, 3, 5, 7, 9 and no others.
8. **Priority.** The five §6.7 rows give 697, 1000, 60, 577, 266.
9. **Quantization.** (12.3456 m, 3.0 m, 200.0004 m) → (12345, 3000, 200000); yaw 16400 → 1025 → 16408; pitch −300 → 490 → −304; HP 42,500 of 85,000 → 128; HP 400 of 85,000 → 2; pitch ±16000 quantize to 1000 and 0.
10. **Record sizes.** A walking update (two 11-bit axes, rotation) is 74 bits; a create with all 8 groups and no statuses is 329 bits; 300 such creates take 12,338 bytes (98,700 bits padded to a byte).
11. **World edit.** The §6.11 edit encodes as `08 00 00 03`; a 5,000-edit tick splits into parts of 4,096 and 904 with the more-parts bit set on the first.
12. **Field cell.** The §6.12 cell encodes as `a2 08 07 00 02`; the same cell becoming empty in a delta page encodes `a2 08 00 00`.
13. **Keyframe rule.** With `acked = 1001`, a snapshot at boundary 1063 is encoded against 1001 while the host holds 32 views, and boundary 1065 triggers a `Keyframe`.
14. **Handles.** A handle removed in snapshot 2001 is not reallocated before the client acknowledges 2001, then is the lowest free handle.
15. **Relevance.** An `Instanced(2)` item 1 m from slot 1's Erratum never appears in slot 1's snapshots; an Erratum 200 m away always does; an entity entering chunk distance 3 stays relevant and leaves at distance 4.
16. **PvP culling.** An invader 20 m away behind a wall of `full` blocks, last seen at test tick 600, stays relevant at test ticks 606 to 630 and is culled at 636; at 15 m it stays relevant through the wall; carrying `st.marked` applied by an Erratum, it is never culled for the party.
17. **Bleed.** Successive downs in one Folio start at 1800, 1200, 600, 600 ticks. With Max HP 85,000 a 17,000 mHP hit removes 360 ticks and a 20,000 mHP hit removes 424.
18. **Revive.** Default revive takes 180 ticks; with `revive_bp` 5000, 90; with 1000, 30. Releasing `INTERACT` at progress 179 resets progress to 0 and bleeding resumes on that tick.
19. **Revive HP.** At Max HP 85,000 and Grade 2, the first, second and fourth revives in one Folio set 39,100, 19,550 and 8,500 mHP; Grade 0's first revive sets 42,500; Grade 20 always sets 8,500; a target with `st.withering` still gets 39,100 at Grade 2; a downed Erratum carried through the exit after its second down enters the next Folio at 19,550.
20. **Ghost loss.** A bare resurrection with 0 eligible items draws nothing from `rng.loot`; with 3 eligible items it draws exactly one `range(3)`.
21. **Trade.** A Lucre pile, a Keepsake, a quest item and an equipped cursed item are each refused; a trade breaks when origins reach 6,001 mm; an executed trade moves uids in ascending order and leaves `finder_slot` unchanged.
22. **Migration mode.** With a Shadow: hot. Without, 18,000 ticks at 500 ticks/s (36 s): warm. At 160 ticks/s (112.5 s): cold, and a ranked run's `restores` increases by 1. During an invasion: cold even with a Shadow. Before any probe completes, `measured_tick_us` is 4,000, so 11,250 ticks since entry is the warm limit.
23. **Successors.** Slot 1 (handheld, no Shadow, 40 ms), slot 2 (Shadow in sync, 90 ms), slot 3 (desktop, no Shadow, 30 ms), all with complete CILs, rank as 2, 3, 1; with slot 3's backlog incomplete they rank 2, 1, 3.
24. **Migration continuity.** In NetSim `typical`, killing the host at a random tick resumes warm play with the successor's state hash at the resume boundary equal to an uninterrupted replay of the truncated CIL.
25. **Handshake.** A client with a different simulation-affecting mod set is refused with `net.reject.content_mismatch`, while one that differs from the host only in a presentation-only mod is admitted and shows the cosmetic-difference notice; a fifth party member with `net.reject.party_full`; a client failing both proto and content checks gets `net.reject.proto_mismatch`.
26. **Run identity.** A hot, a warm and a cold migration each keep `run_id` unchanged (§3.8).
27. **Replay verification.** The verifier accepts an untouched 4-player replay and rejects it with `svc.err.replay_mismatch` after one changed yaw at tick 5,000.
28. **Ranked token.** A second token request for the same account and UTC date returns `svc.err.attempt_used`.
29. **Idempotency.** Repeating a pull with the same `req_id` and body returns the first response and posts one journal transaction; the same `req_id` with a different body returns `svc.err.idem_conflict`.
30. **Run report.** The doc 15 §10.4 example report returns `accepted` with grants of 95 Ink and 14 Quires, `codex.xp_base` 5,025 and `verify_sampled` false (`mix64(0x00000000075bcd15) mod 100 = 84`).
31. **Invasion ticket.** A ticket verifies over the 20-byte domain, separator and three little-endian `u64`s; it is refused 61 s after issue and refused for a different `host_run_id`.
32. **NetSim determinism.** The same scenario seed delivers the identical datagram sequence on two runs, with exactly 4 draws per datagram.
33. **Late targets.** A target spawned at tick 4990 is not hit by a rewound ray applied at `t = 5000` with `t_r = 4984`.
34. **Instanced Lucre.** A kill in a 3-player party with one Dormant slot produces Lucre piles for the 2 Active slots only, each sent only to its owner.
35. **Telemetry.** With telemetry at its default, and for any minor-flagged account, a full online session makes zero requests to `/v1/telemetry/batch`. An opted-in host of a 3-player party with one minor-flagged member sends one run summary whose per-Folio entries hold the §11.7 fields for the other 2 slots only; the non-host members send no run summary.
36. **Budgets.** In NetSim `typical`, no snapshot payload exceeds 1,150 bytes and no 600-tick window exceeds 16,000 B/s per client. A listen host with 4 remote peers gives each 12,000 B/s, `SNAP_RATE` 6,560 and refill 218, and its total upstream, voice excluded, stays at or below 48,000 B/s.
37. **Invasion window.** With `mode.invasion_open`: Folio 2 at entry + 9,000 ticks is closed; Folio 3 at entry + 7,199 is closed and at entry + 7,200 is open; a boss Folio is closed; the tick after any Erratum enters `room.colophon` is closed; after one invasion has started in a Folio, that Folio stays closed. A Daily or ranked lobby refuses `mode.invasion_open`.
38. **Invasion end.** An invader present for 18,000 ticks receives `InvaderLeave { reason: 2 }` on that tick and `ev.invasion_ended` is emitted; outcome reports of 12,000 ticks with reason 3 from both host and invader pay 10 Quires (doc 15 §2.4), and a lone report pays nothing.
39. **Redaction and downed.** With two Errata downed and the third alive, an invader hit that brings the third to 0 HP triggers the Redaction instead of a run end, and both downed Errata lose `st.downed` without `ev.player_revived`.
40. **Wire tick windows.** `reconstruct_tick(5, 0xFFFF_FFFE)` returns `0x1_0000_0005`; `reconstruct_tick(7, 3)` returns 7.
41. **Lag budget.** In an in-process session with a fixed 50 ms one-way delay and no jitter or loss, after 600 ticks every applied command has `t − view_tick` between 11 and 14 (`rtt_ticks` 6, slack 2 to 4, `interp_delay` 4, minus at most 1 tick of fractional clock phase) and no `view_tick` is clamped. With RTT 100 ms, `target_slack` 2 and `interp_delay` 6 the steady-state rewind is 14 (§7.5 example).
42. **Hard reset.** `host_tick_est` 5000.4, RTT 100 ms and `target_slack` 2 give `P = 5005`.
43. **Cold-restart log.** Folio 8 entered at `E = 90,000` with one `SlotJoin`, then a cold migration with `L = 95,000`: the CIL holds no command record at any tick ≥ 90,000 before the restart, tick 90,000 holds the `SlotJoin` followed by `FolioRestart`, `restores` rises by 1, and re-simulating the final CIL from tick 0 reproduces every later `HashReport` of the resumed session.
44. **Invasion sealing.** For an invasion from tick 41,000 to 53,000, no CIL message to a party client sets slot_mask bit 4 for those ticks; `cil_unseal` covering ticks 41,000 to 53,000 follows the first batch after 53,000; afterwards every client's CIL hash equals the host's, and a Shadow paused at 41,000 matches the next `HashReport`.
45. **Invasion host loss.** Killing the host during an invasion gives `net.migrate.cold` even with a Shadow and `InvaderLeave { reason: 6 }`, and no party client ever held a slot-4 command of the sealed ticks.
46. **Trade closure.** In `both_locked`, the offerer drinking an offered potion returns the trade to `idle`; both confirming while a receiver's backpack lacks room returns it to `open` with the confirming `action_seq` in each `OwnerDelta.rejected`; 1,800 ticks in `both_locked` with one confirmation return it to `idle`.
47. **Kick vote.** With 4 Active members, a vote against slot 3 passes with 2 yes votes (the starter's included) within 1,800 ticks; with 3 members it needs both others; with 2 members no vote passes.
48. **Voice.** With 4 talkers each receiver gets frames from exactly the 2 earliest talk spurts; a peer whose send queue holds 8,193 bytes receives no voice frame while its snapshots continue.
49. **Backlog.** A 2,000,000-byte compressed backlog to a waiting joiner completes in 250 s; one still incomplete at Folio entry streams at 1,000 B/s with that client's `SNAP_RATE` at 9,560 until it completes; the client writes no replay file before then.
50. **Economy schemas.** Every request and response example of §11.4 validates against `palimpsest/schemas/svc/econ_api.json`; a pull whose `commitment` is uppercase is refused with `svc.err.bad_request`; a pull on a `committed` epoch without `client_seed_hex` is refused with `svc.err.epoch_mismatch` and posts nothing.
51. **Pull vector.** The §11.4 pull request on a fresh Echo family returns `u_tier` 344, `u_feat` 3861, `idx` 1, `echo.lan_ninth_peak`, `pity_after` (1, 0, false, false), `spark_after` 1, and debits 160 Ink (doc 15 acceptance test 8).
52. **Beacons.** A beacon not refreshed for 45 s is absent from the listing; a beacon whose party blocked the caller is never listed to it; the host deletes its beacon on the tick the window closes, and a ticket requested afterwards is refused with `svc.err.not_found`.
53. **Ranked token signature.** The §11.6 signature verifies over the 20-byte domain `PALIMPSEST/RANKED/v1`, the separator, the 32-byte token and three little-endian `u64`s; changing `run_seed` makes it fail.
54. **Party wipe.** In a 2-player party with slot 0 downed, a lethal hit on slot 1 makes slot 1 a ghost, then kills slot 0 (`ev.entity_killed`) and ends the run on the same tick, with no bleed wait.
55. **Challenge.** A `password_proof` computed over any nonce other than the connection's `Challenge` nonce is refused with `net.reject.wrong_password`; a client that receives no `Challenge` within 5,000 ms disconnects.
56. **State machines closed.** For every state of §4.1, §4.2 and §9.7 and every event listed anywhere in those tables, a table-driven test finds exactly one next state (or the documented "ignored").
57. **Sanitizer.** A command with `ACTION` 6, `ACTION` 7 or bit 25 set is replaced by the §5.4 substitute and counts one violation each; a command with `ACTION` 5 and `WHEEL_ENTRY` 7 is applied unchanged.
58. **HUD light byte.** A receiver whose `light_at` cell holds `light_sky` 4 and `light_block` 11 under no light override gets `HUD.light = 0x4b` and `light_at` 11; inside an override with `max_level` 3 it gets `0x33` and `light_at` 3.
59. **Stealth section.** A `StealthHud` with 6 rings, 10 marks and 5 steps produces a `STEALTH` section with the 4 newest rings, the 8 marks of highest `meter` and the 4 newest steps; an unchanged `StealthHud` writes no `STEALTH` section; a ring 3,000 mm along +X from the receiver encodes `dx` as `zz(3000)` = 6000, bytes `f0 2e`.
60. **Voice with strangers.** In a 4-player public lobby where slot 2 is on slot 0's friends list and slots 1 and 3 are not, slot 0's client sends `VoiceMute { muted: 0b1010 }` on joining, and while slots 1, 2 and 3 all talk the host forwards slot 0 only slot 2's voice frames.
61. **Solo save during an invasion.** A solo host with `mode.invasion_open` that requests Save and quit while an invader is present saves on the tick `ev.invasion_ended` is emitted and not before; opening a menu during the invasion does not pause the simulation.
62. **Projectile catch-up.** An invader's projectile fired at `t = 1000` with applied `view_tick` 994 takes 7 steps on tick 1000, against the poses of ticks 995 to 1000 and then 1000 again, and 1 step per tick afterwards.
63. **Routes.** A Steam host's private, friends-only and public lobbies all run on `net.route.sdr`, and an invader joins through SDR; a LAN or direct-IP session runs on `net.route.direct`; no session opens a `net.route.relay` connection (§3.5).
64. **Map reveal.** After a party member reads `item.text.mapping`, the next `RunView` every party client receives has `map_revealed = true`, and the first `RunView` of the next Folio has `map_revealed = false`.
65. **Chip mask.** A hit that chips a destructible full block without breaking it yields, for that tick, a `WorldEdits` message with no edit, one crack record and one mask record whose `mask` equals the `mask` field of that tick's `ev.block_chipped`; afterwards the client's `mask_at` for that cell equals the host's, and a predicted capsule collides with the chipped shape exactly as the host's does. A later plain `set_block` of the cell sends one edit and no mask record, and the client then holds no custom mask for it.
66. **Transcription claim.** With `transcription` 40, `POST /v1/econ/archive/transcribe` for `bundle.script.lost_chapter_wuxia` with `expect_cost` 32 grants it and returns `transcription_after` 8; a claim of `bundle.script.kit_monk` with `expect_cost` 16 then returns `svc.err.insufficient_transcription` and posts nothing (doc 15 §3.9.1).

## Invariants annex

```
Σ 16: authority ⊗ thin clients ⊗ CIL ⊗ snapshots ⊗ prediction ⊗ lag-comp ⊗ co-op ⊗ migration ⊗ services ⊗ privacy
W! one authoritative Sim per session                       N! client state → host sim, except commands and intents
W! applied command logged (real | substitute)              N! substitute with movement | press edge
W! view_tick ∈ [t − min(16, lag + 4), t]                    N! rewind of blocks | target state | NPC attacks
W! snapshot boundaries b = t + 1, t even                   W! delta vs acked baseline | keyframe when b − acked ≥ 64
W! snapshot payload ≤ 1150 B ∧ client down ≤ 16000 B/s     N! Instanced(s) item → client ≠ s
W! predicted mover == host mover (exact) or corrected      N! extrapolation of remote entities
W! blocks global ∧ reliable ordered                        W! fields | entities interest-managed
W! every party client completes CIL from tick 0 (backlog) ∧ holds Folio-entry state   W! blueprint_hash checked at every Folio entry
W! drop-in at Folio entry (invader: only exception)        N! invader ∈ {voters, successors, n}
W! migration ∈ {hot, warm, cold}; cold ⇒ restores += 1     N! migration that skips hash checks
W! downed, revive, trade draw 0 random numbers             W! ghost item loss: keyed rng.loot, ≤ 1 draw
N! currency in trades                                      W! trade atomic, ascending uid
W! ranked ⇒ server re-simulation from tick 0               N! ranked accept without matching hashes
W! service writes idempotent by req_id | doc 15 idem_key   N! telemetry without consent | from minors
N! chat | voice | game traffic stored by services          N! IP address in logs beyond truncation
W! offline: every gameplay system runs                     N! session waits on a service call
W! invasion ⇒ mode.invasion_open ∧ Canon §17.9 window       N! invasion in Daily | Weekly | ranked runs
N! slot-4 command → party client while invader present      W! host loss during invasion ⇒ cold migration
W! render_tick := snap_tick_est − interp_delay              W! hard reset P := host_tick_est + one_way_ticks + target_slack
W! revive_hp_bp ≥ 1000 ∧ halves per repeated down           W! cold ⇒ CIL cut to E ∧ FolioRestart at E
N! voice ∈ game budgets                                     W! voice dropped before game traffic
W! run_id := host OS CSPRNG                                 W! every economy body ⊨ econ_api.json ∧ canonical hex
W! ∀ state ∈ §4.1 ∪ §4.2 ∪ §9.7: every event → one next      N! replay file written before CIL complete
W! sanitize: ACTION ∈ 0..=5 ∧ bit 25 = 0, else substitute   N! voice frame from a talker → a receiver that muted it
W! solo save while invader present := tick of ev.invasion_ended   N! pause while an invader is present
W! online session ⇒ Steam lobby ∧ net.route.sdr             N! presentation_hash mismatch → Reject
W! STEALTH ≤ 4 rings ∧ ≤ 8 marks ∧ ≤ 4 steps               W! MoverSnap carries the per-status speed list, never a product
W! WorldEdits: edits, then cracks, then custom masks        N! net.route.relay | svc.relay | svc.workshop in use
```
