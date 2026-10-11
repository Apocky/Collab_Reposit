# 16 — Multiplayer, Netcode and Online Services

As of 2026-10-11. Status: BINDING. Conforms to canon-1.1.

This chapter defines how 1 to 4 players share one authoritative simulation: the three topologies, transports and NAT traversal, the wire protocol with exact packet layouts, snapshot replication with delta compression, quantization, interest management and priorities, voxel and field replication, client prediction and reconciliation, lag compensation, the co-op rules this chapter owns (`st.downed`, `st.ghost`, revive, trading, host migration, drop-in), replays, daily-score verification, desync diagnostics, and every backend service with its API. The header line is fixed by the document contract; the canon on disk is canon-1.4 and every rule below was checked against canon-1.4, including §4.3 (fixed terrain, placeables) and §17.9 (invasions) (Canon gaps G1).

## Imports

| Source | Section | What this chapter consumes |
|---|---|---|
| DESIGN_BRIEF | §5, §8 | host-authoritative co-op with prediction, reconciliation and lag-compensated hits; deterministic input logs for replays and daily verification; offline-complete end-of-service mode |
| DECISIONS | 2026-10-11 rows 6, 15, 16 | PC and Steam Deck first; opt-in PvP invasions where a player invades as an Editor; limited building (only placeables and destructible-tagged blocks change) |
| Canon | §1.1, §1.3, §1.6 | ID grammar and `id32`; `floor_div`, `ceil_div`, `mul_bp`; integer-only content, `u64` as hex strings |
| Canon | §1.8, §1.9, §1.11 | Folio box 256 × 64 × 256, chunk slot math; `Bam`, pitch clamp ±16000, `ang_diff`; `EntityId` layout, ascending-ID iteration |
| Canon | §2.1–§2.5 | 60 Hz tick, no extrapolation except local prediction; tick schedule row "Network snapshot, period 2, phase 0"; session header; replay command record with `view_tick`; save contract; `state_hash` every 60 ticks; co-op saves only at Folio entry |
| Canon | §3.2, §3.3, §3.6 | sequential and keyed streams (keyed draws make late inputs safe); seed display; daily and weekly seeds |
| Canon | §4.1, §4.2 | humanoid capsule, speeds, dodge |
| Canon | §9 | baked pose tables; 32 ticks of pose history for lag compensation |
| Canon | §10.8, §11.1 | lethal resolution steps 2–3; `st.downed` (owner 16: 30 s bleed-out, revive in 3 s), `st.ghost` (owner 16) |
| Canon | §13.6, §13.9, §14 | party-shared type knowledge; `finder_slot`; no currency can be traded or gifted |
| Canon | §4.3 | terrain fixed except `destructible` blocks and `cat.placeable` items |
| Canon | §17.1, §17.2, §17.5, §17.6 | `PARTY_MAX` 4, loot instanced per player; party-size tables; `FF_DIRECT_BP`, `FF_AREA_BP`, `mode.ff_off`, `mode.ff_full`, `mode.loot_shared`; ghosts and resurrection |
| Canon | §17.9, §2.4 | invasions: `mode.invasion_open` (off by default, forbidden in Daily, Weekly and ranked runs), Redactor invader as `rank.hunter` with an account Editor kit, window (Folio 3 onward, one per Folio, no boss Folios, not before 120 s, not after a Colophon entry, at most 300 s), invader limits, the Redacted outcome, replay slot 4, mid-Folio join |
| Canon | §18, §19 | `ev.player_downed`, `ev.player_revived`, `ev.player_died`, `ev.player_ghosted`, `ev.player_resurrected`, `ev.run_ended`, `ev.invasion_started`, `ev.invasion_ended`, every event replicated for presentation; namespaces `net.` and `svc.` belong to this chapter |
| Doc 02 | §2.2, §3.2–§3.4 | crates `renet` 2.0.0, `steamworks` 0.13.1, `postcard` 1.1.3, `zstd`, `blake3`, `ed25519-dalek` 3.0.0, `axum` 0.8.9, `tokio` 1.53.2, `sqlx` 0.9.0, PostgreSQL 18; `crate.net`, `crate.server`, `crate.services`; the `Transport` sketch |
| Doc 02 | §5.4, §5.5 | `Sim::step`, `snapshot`, `restore`, `serialize`, `section_hashes`; `PlayerCommand`, button bits, `PlayerIntent`, `IntentKind` |
| Doc 02 | §6.5–§6.7, §12.1, §13.4, §15, §17.3, §17.4 | 12 serialization sections and section hashes; golden replays; desync dump; `PALS` container kinds; `modset_hash`; `perf.deck.net.host_kbps` = 384 kbit/s for 3 clients; flags `flag.net.transport`, `flag.net.input_redundancy`; telemetry consent rules |
| Doc 03 | §2.2, §3.2–§3.4, §5, §5.16, §5.19, §13, §14.2, §15.1, §16 | system order (`sys.input_apply`, `sys.hitscan`, `sys.melee`, `sys.tick_end`); entity kinds, `comp.player` (semantics owned here), `Kind.flags`, `Pose.hist`; the character controller; pose-history rewind; first-tick projectile rewind; `set_block` and `ev.block_changed`; event payloads; 5 × 5 chunk activation window; save coverage |
| Doc 04 | §6.2, Restoration and Warden talents | downed members and ghosts share kill XP; instant revive at Restoration 100; Warden `resurgence` revive time × 5000 |
| Doc 05 | §2.6, §14.3 | `LAGCOMP_MAX_TICKS = 16`, target state read at the present tick, projectile catch-up steps; downed Errata keep capsules and cannot attack, block or use items; bleed-out shortening `ceil_div(total_mhp × 1800, max_hp_mhp)` |
| Doc 11 | §2.2, §14.4, §17 | item `visibility` `Instanced(slot)` or `Public`; ground items destroyed at Folio exit; per-slot rolls with `seq = slot` |
| Doc 12 | §6.1 | `FolioBlueprint`, `blueprint_hash` compared by every co-op peer |
| Doc 13 | target scoring, §10.7 | downed candidates × 2000, ghosts never perceivable; template `enemy.editors.invader`, no AI think, entry at the hunter placement anchor |
| Doc 10 | §2.2, §2.5, §2.8 | party-shared type knowledge; knowledge travels with traded items; host sends only `ItemView` |
| Doc 14 | §1.1, §2.1, §2.5, §3.2, §3.6, §3.10, §4.3, §8.4, §8.8, §9.4, §12 | lobby state machine; joiner rules; exit commit rule and constants; ghost restoration HP; Dormant and Departed slots, `DORMANT_GRACE_S`; ranked Daily attempts; boards and `verify`; `RANKED_RESTORE_MAX` = 2; Folio-entry save copies for host migration |
| Doc 15 | §2.3, §2.4, §4, §8.2, §8.5, §10, §11 | caps; invader Quires (`INVASION_QUIRES`, `INVASION_MIN_TICKS`); commit-reveal fairness, Sealed Drafts, signed receipts; `juris.flag.invasion_minor_default`; chat safety including invader rules; economy service components, ledger, idempotent grants, run reports; end-of-service mode |
| R4 | §6.1–§6.4 | snapshot model, input redundancy 8, 30 Hz snapshots, 96 kbit/s snapshot budget, SDR facts (IP privacy, P2P), renet for LAN, interpolation delay 100 ms default |
| R3 | A2, C4 | crossplay on by default, cross-progression; COMMS-1, LOG-1, EOL-1 requirements |
| R7 | §4, §11 R-CO3, R-CO4 | revive health falling with difficulty; instanced currency pickups |

---

## 1. Decisions and constants

### 1.1 Decisions

1. **One authority.** Exactly one process runs the authoritative `Sim` for a session: the host. Every other process sends `PlayerCommand`s and `PlayerIntent`s and receives replicated state.
2. **Thin clients, snapshot replication.** Clients render remote state from 30 Hz delta-compressed snapshots and predict only their own mover. They regenerate each Folio's terrain locally from the seed and receive block edits as an ordered diff log, so terrain never streams in bulk.
3. **Every client keeps the Confirmed Input Log (CIL)** of the inputs the host applied, plus the latest Folio-entry state. This gives every client a complete replay, makes any client able to become host, and feeds the daily verifier.
4. **Shadow replicas are a configurable option** (`flag.net.shadow`, default auto: on with at least 6 physical cores and not handheld). A Shadow is a full `Sim` replica on a client that consumes the CIL. Shadows detect desyncs live and make host migration instant.
5. **Favor the shooter** for Errata and invader attacks, inside doc 05's rewind cap of 16 ticks (`LAGCOMP_MAX_TICKS`, doc 05 §2.6).
6. **Platform P2P first.** Steam builds connect through Steam Datagram Relay, so no player IP address is revealed and no game servers are rented. The dedicated server binary exists for self-hosting and for headless verification, not for live matchmaking.
7. **Offline-complete.** Every gameplay feature, including co-op over LAN and direct IP, runs without the backend (DESIGN_BRIEF §8). Invasions need matchmaking and are the one online-only feature.
8. **Opt-in invasions** (DESIGN_BRIEF §1, DECISIONS #15). A run whose host enables invasions accepts one invading player at a time from another session, who plays an Editor in slot 4. Canon §17.9 fixes the rules; this chapter owns the session, matchmaking, visibility and transport details, doc 13 the invader's body, doc 15 its rewards.

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
| `NET_HANDLE_BITS` | 13 | per-client entity handles `0..=8190`; 8191 means none |
| `MTU_PAYLOAD` | 1,200 bytes | largest unreliable message; snapshots stop at 1,150 |
| `RELIABLE_MSG_MAX` | 262,144 bytes | larger payloads are split into 65,536-byte parts |
| `DATAGRAM_OVERHEAD` | 48 bytes | fixed transport overhead counted per datagram in every budget |
| `CLIENT_DOWN_BUDGET` | 16,000 B/s (128 kbit/s) | per client, all channels, overhead included |
| `SNAP_RATE` | 10,560 B/s payload | snapshot token refill; 352 B per snapshot on average |
| `SNAP_BUCKET` | 2,112 B | snapshot token capacity (6 average snapshots) |
| `CIL_BATCH_TICKS` | 6 | CIL message cadence (10 Hz) |
| `NET_STALL_MS` | 1,000 ms | silence before the "connection problem" overlay |
| `NET_TIMEOUT_MS` | 5,000 ms | silence before a peer is declared lost |
| `MIGRATE_ANNOUNCE_MS` | 10,000 ms | per successor rank before the next successor tries |
| `MIGRATE_WARM_MAX_MS` | 45,000 ms | estimated re-simulation time above which migration is cold |
| `JOIN_WAIT_MAX` | none | a joiner waits for the next Folio entry however long it takes |
| `DEFAULT_PORT` | UDP 27310 | renet direct and dedicated |
| `INVADER_SLOT` | 4 | the slot index an invader's commands use |
| `INVASION_MAX_TICKS` | 18,000 (300 s) | longest invasion (Canon §17.9) |
| `INVASION_FIRST_FOLIO`, `INVASION_ENTRY_DELAY` | Folio 3, 7,200 ticks | earliest Folio and earliest tick after entry (Canon §17.9) |
| `PVS_PERIOD` | 6 ticks | visibility culling cadence for PvP pairs (§6.6) |
| `PVS_GRACE_TICKS` | 30 | an unseen hostile player stays replicated this long |

Every numeric constant of this chapter lives in the content record `net.params.default` (`content/16/net/params.json`), loaded with the content set and covered by `content_hash`:

```json
{ "id": "net.params.default", "proto": 1, "input_redundancy": 8, "input_buffer_span": 64,
  "slack_target_min": 2, "slack_target_max": 8, "interp_min": 4, "interp_max": 12,
  "pred_ring": 128, "pred_stall_ticks": 120, "lagcomp_max_rewind": 16, "lagcomp_slop": 4,
  "snap_ring": 32, "handle_bits": 13, "mtu_payload": 1200, "snap_payload_max": 1150,
  "datagram_overhead": 48, "client_down_budget": 16000, "snap_rate": 10560, "snap_bucket": 2112,
  "cil_batch_ticks": 6, "stall_ms": 1000, "timeout_ms": 5000, "migrate_announce_ms": 10000,
  "migrate_warm_max_ms": 45000, "default_port": 27310, "invader_slot": 4, "invasion_max_ticks": 18000, "invasion_first_folio": 3, "invasion_entry_delay": 7200,
  "pvs_period": 6, "pvs_grace_ticks": 30, "pvs_near_mm": 16000,
  "bleed_ticks": [1800, 1200, 600], "revive_base_ticks": 180, "revive_min_ticks": 30,
  "revive_range_mm": 1500, "revive_cone_bam": 8192, "bleed_damage_ticks_per_maxhp": 1800,
  "revive_hp_base_bp": 5000, "revive_hp_step_bp": 150, "revive_hp_min_bp": 2000, "reliable_msg_max": 262144,
  "trade_range_mm": 4000, "trade_break_mm": 6000, "trade_max_items": 8, "trade_accept_ticks": 600 }
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

Single player is a host with zero remote peers. The client process runs `HostSession` on the sim thread (doc 02 §7.1). The local slot's commands enter through `LocalInputPort`, a lock-free queue that bypasses serialization; no snapshot is built for the local player, whose renderer reads the `ViewMailbox` directly. The local player is not predicted: its camera position interpolates (doc 02 §7.3) and its look comes from current input, so position latency is one tick plus interpolation. Solo saves at any tick boundary (Canon §2.5), pauses when a menu opens or a wake gap exceeds 1 s (doc 02 §14), and has no downed state (Canon §10.8 step 2 needs another living member).

### 2.3 Listen host (`net.topo.listen`)

One player's client is host and runs the solo arrangement plus up to 3 remote thin clients and, in an invasion, one invader (§9.13). The host player's own slot uses `LocalInputPort` and is never predicted, so the host plays at zero network latency. A listen-host session never pauses. The host's upstream budget is 384 kbit/s for 3 clients (doc 02 `perf.deck.net.host_kbps`).

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
| worker | hashing (doc 02 §6.5), zstd of Folio-entry state | Folio generation, section-hash verification, CIL replay for warm migration |

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
| `net.route.relay` | `renet` netcode secure mode to a `svc.relay` node, which forwards to the host's own relay session | relay | yes | online sessions with a non-Steam member (post-1.0 crossplay) |
| `net.route.in_process` | `crossbeam-channel` pairs | none | n/a | tests, bots, CI (`flag.net.transport = 2`) |

### 3.2 Channels and lanes

| ID | Channel | Delivery | Direction | Lane priority (0 highest), weight | Contents |
|---|---|---|---|---|---|
| `net.ch.control` | 0 | reliable, ordered | both | 1, 1 | handshake, intents, owner, party and run state, hash reports, chat, session |
| `net.ch.input` | 1 | unreliable | client → host | 0, 1 | input packets |
| `net.ch.snapshot` | 2 | unreliable | host → client | 0, 1 | snapshots |
| `net.ch.world` | 3 | reliable, ordered | host → client | 2, 3 | world edits, keyframes, Folio-entry state, chunk data, full state |
| `net.ch.cil` | 4 | reliable, ordered | host → client | 2, 1 | Confirmed Input Log |
| `net.ch.voice` | 5 | unreliable | both | 1, 1 | voice frames |

On Steam each channel is one lane configured with `ConfigureConnectionLanes` using the priorities and weights above, and messages are sent with `k_nSteamNetworkingSend_Reliable` or `k_nSteamNetworkingSend_UnreliableNoNagle`. On renet each channel is one renet channel: `ReliableOrdered` for 0, 3, 4 and `Unreliable` for 1, 2, 5. The connection send rate is set to minimum 131,072 and maximum 524,288 bytes per second (Steam `SendRateMin`, `SendRateMax`); this chapter's budgets keep the average below 16,000 B/s per client.

**Reliable framing.** Every message on channels 0, 3 and 4 starts with `rseq: u32` (little-endian), counting from 0 per channel per connection. The receiver processes messages strictly in `rseq` order and buffers gaps, so correctness never depends on a transport's ordering guarantee. Payloads over `RELIABLE_MSG_MAX` are split into parts of 65,536 bytes, each carrying `part: u16, parts: u16` after the message type.

### 3.3 Steam (production)

- Lobbies use `ISteamMatchmaking` with 4 members; the host is the lobby owner. Lobby data keys, all ASCII: `pal_ver` (`SIM_VERSION` decimal), `pal_proto`, `pal_ch` (first 16 hex digits of `content_hash`), `pal_mods` (first 16 hex digits of `modset_hash`, doc 02 §13.4), `pal_mode`, `pal_grade`, `pal_state` (`lobby` or `run`), `pal_folio`, `pal_ff`, `pal_loot`, `pal_join` (`open` or `closed`), `pal_host` (SteamID64), `pal_succ` (comma-separated successor SteamID64s, §9.3), `pal_pub` (`net.lobby.*` suffix), `pal_inv` (`open` or `closed`, §9.13), `pal_kind` (`party`, or `beacon` for an invasion beacon), `pal_run` (`run_id`, beacons only). Member data: `pal_ploc` (the member's Steam ping location).
- Lobby types: `net.lobby.private` (invite only, `k_ELobbyTypePrivate`), `net.lobby.friends` (`k_ELobbyTypeFriendsOnly`), `net.lobby.public` (`k_ELobbyTypePublic`). Default: friends.
- **Quick Join** requests the lobby list filtered by equal `pal_ver`, `pal_proto`, `pal_ch`, `pal_mods`, `pal_mode`, `pal_join = open`, `|pal_grade − own max grade| <= 2`, and the default distance filter; it then estimates ping from each lobby's host ping location (`ISteamNetworkingUtils` ping locations exchanged as lobby member data `pal_ploc`) and joins the lowest estimate under 150 ms, else creates a public lobby.
- Identity on the wire is the SteamID64; the host authenticates each client with `BeginAuthSession` on a ticket from `GetAuthSessionTicket`, and refuses with `net.reject.auth_failed` on any result other than OK.

### 3.4 Direct and LAN

- The host listens on UDP 27310 (configurable). LAN discovery: the host broadcasts a 32-byte beacon `PALB ‖ proto u16 ‖ sim_version u32 ‖ content_hash[0..8] ‖ port u16 ‖ party u8 ‖ max u8 ‖ mode id32 ‖ 6 zero bytes` to UDP 27311 once per second; clients list beacons seen in the last 3 s.
- renet runs in unsecure mode on LAN and direct IP; the host password (a lobby setting, default empty) is verified inside `Hello` as `BLAKE3("PALIMPSEST/PW/1" ‖ host_nonce ‖ password)`, where `host_nonce` is 16 bytes sent in the renet connection challenge.
- No hole punching is attempted. A direct connection that has not completed its handshake after 5,000 ms fails with a message telling the host to forward UDP 27310 or use a Steam or relay session.

### 3.5 Relay (`svc.relay`)

Sessions with any non-Steam member use `net.route.relay`, always relayed, never peer-to-peer, so no IP address is exposed. The host creates a relay session through `svc.lobby` and receives a session ID; every member, the host included, receives a netcode connect token (`svc.lobby` signs tokens with the relay key; token lifetime 30 s, timeout 10 s). The relay forwards datagrams between the host's connection and each client's connection without inspecting payloads. Relay nodes run in `eu-central`, `us-east` and `ap-southeast`; the lobby picks the node with the lowest RTT from the host. Launch builds are Steam-only, so the relay ships with the first non-Steam platform.

### 3.6 Crossplay identity

- **Account.** `account_id: u64` from the backend OS CSPRNG at account creation, never derived from a platform ID; it is the `acct` of doc 15 receipts.
- **Linked identities.** A table maps `(platform, platform_user_id)` to one `account_id`. Platforms: `steam`, later `psn`, `xbl`, `nsw`, `epic`. Linking a second platform to an existing account requires signing in on both within 600 s with a one-time 8-character link code.
- **Display.** In-session names are the platform persona name, sent in `Hello`, at most 32 UTF-8 bytes after truncation at a character boundary. Cross-platform friends use a **friend code**: the low 40 bits of `mix64(account_id)` as 8 Crockford Base32 characters (Canon §3.6 alphabet), most significant first, shown `XXXX-XXXX`. Example: `account_id 0x0123456789abcdef` → `mix64 = 0xb2c058e4ebb5112c` → low 40 bits `0xe4ebb5112c` → `WKNV-A49C`.
- **Crossplay setting.** On by default (R3 A2); turning it off restricts Quick Join and invitations to the player's own platform family.
- **Blocks.** Platform blocks and account blocks both apply: a blocked player's lobby is hidden, the player cannot join one's lobby, and voice and text from them are muted.

### 3.7 Message envelope

Every transport message is `msg_type: u8` followed by the body; on reliable channels `rseq: u32` comes first, before `msg_type`. Bodies of control messages are the `postcard` encoding (doc 02) of the Rust struct named in the table. Input packets, snapshots, world edits and CIL batches use the hand-written layouts of §5.1, §6.8, §6.11 and §6.14. All fixed-width integers are little-endian. `varint` is unsigned LEB128; `zz` is zigzag (`(n << 1) ^ (n >> 63)`) followed by LEB128. A wire tick is the low 32 bits of a `Tick`; the receiver reconstructs the `u64` nearest to its own current tick.

| Type | ID | Channel | Direction | Body |
|---|---|---|---|---|
| 0x01 | `net.msg.hello` | 0 | C → H | `Hello` |
| 0x02 | `net.msg.welcome` | 0 | H → C | `Welcome` |
| 0x03 | `net.msg.reject` | 0 | H → C | `Reject { reason: Id32 }` |
| 0x04 | `net.msg.ping` | 0 | both | `Ping { sent_us: u64 }` |
| 0x05 | `net.msg.pong` | 0 | both | `Pong { echo_us: u64, host_tick: u64, host_tick_age_us: u32 }` |
| 0x06 | `net.msg.bye` | 0 | both | `Bye { reason: Id32 }` |
| 0x10 | `net.msg.input` | 1 | C → H | §5.1 |
| 0x11 | `net.msg.intent` | 0 | C → H | `IntentMsg { tick: u32, action_seq: u32, kind: IntentKind }` |
| 0x20 | `net.msg.snapshot` | 2 | H → C | §6.8 |
| 0x21 | `net.msg.keyframe` | 3 | H → C | §6.13 |
| 0x22 | `net.msg.world_edits` | 3 | H → C | §6.11 |
| 0x23 | `net.msg.folio_enter` | 0 | H → C | `FolioEnter`, §6.13 |
| 0x24 | `net.msg.folio_state` | 3 | H → C | zstd of serialization sections 0, 1, 2, 3, 8, 9, 10, 11 at the Folio-entry boundary |
| 0x25 | `net.msg.chunk_data` | 3 | H → C | `ChunkData { first_slot: u8, sections: zstd bytes }` |
| 0x26 | `net.msg.folio_ready` | 0 | C → H | `FolioReady { folio: u32, gen_ok: bool }` |
| 0x30 | `net.msg.cil` | 4 | H → C | §6.14 |
| 0x31 | `net.msg.hash_report` | 0 | H → C | `HashReport { boundary: u64, hash: [u8; 32] }` |
| 0x32 | `net.msg.section_hash_request` | 0 | C → H | `{ boundary: u64 }` |
| 0x33 | `net.msg.section_hashes` | 0 | H → C | `{ boundary: u64, hashes: [[u8; 32]; 12] }` |
| 0x34 | `net.msg.full_state_request` | 0 | C → H | `{ reason: u8 }` |
| 0x35 | `net.msg.full_state` | 3 | H → C | `{ boundary: u64, part: u16, parts: u16, bytes }` (zstd of the full serialization) |
| 0x40 | `net.msg.owner_delta` | 0 | H → C | §6.10 |
| 0x41 | `net.msg.party_state` | 0 | H → C | §6.10 |
| 0x42 | `net.msg.run_state` | 0 | H → C | §6.10 |
| 0x43 | `net.msg.knowledge_delta` | 0 | H → C | §9.9 |
| 0x50 | `net.msg.chat` | 0 | both | `Chat { from_slot: u8, text: String }`, ≤ 200 bytes |
| 0x51 | `net.msg.voice` | 5 | both | `{ from_slot: u8, seq: u16, frame: bytes }` |
| 0x60 | `net.msg.join_request` | 0 | C → H | `JoinRequest { character: JoinCharacter }` |
| 0x61 | `net.msg.join_ready` | 0 | H → C | `{ folio: u32, slot: u8 }` |
| 0x62 | `net.msg.migrate_announce` | lobby | S → all | `MigrateAnnounce`, §9.3 |
| 0x63 | `net.msg.kick_vote` | 0 | both | `{ target_slot: u8, yes: bool }` |

### 3.8 Handshake

```rust
pub struct Hello {
    pub proto: u16, pub sim_version: u32, pub content_hash: [u8; 32], pub modset_hash: [u8; 32],
    pub build: u32, pub account_id: u64, pub platform: u8, pub persona: String,
    pub password_proof: Option<[u8; 32]>, pub ping_location: String,       // Steam ping location, else empty
    pub shadow_capable: bool, pub minor: bool,                               // minor: doc 15 §8.4 flag
    pub migrating_from_tick: Option<u64>, pub resume_slot: Option<u8>,
    pub role: u8,                    // 0 party member, 1 invader (§9.13)
    pub invasion_ticket: Option<(u64, [u8; 64])>,  // (expiry Unix s, svc.lobby Ed25519 signature), role 1 only
}
pub struct Welcome {
    pub slot: Option<u8>,            // None while waiting to drop in (§9.1)
    pub host_tick: u64, pub run: Option<RunHeader>, pub party: PartyState, pub lobby: LobbySettings,
}
pub struct RunHeader { pub run_id: u64, pub run_seed: u64, pub sim_version: u32, pub content_hash: [u8; 32],
                       pub config: RunConfig /* doc 14 */, pub folio: u32, pub start_tick: u64 }
```

The host checks, in this order, and answers `Reject` with the first failing reason: `proto` (`net.reject.proto_mismatch`), `sim_version` (`net.reject.sim_version_mismatch`), `content_hash` and `modset_hash` (`net.reject.content_mismatch`, doc 02 test 35), platform authentication and, for role 1, the invasion ticket (`net.reject.auth_failed`), password (`net.reject.wrong_password`), host block list (`net.reject.banned`), seats (`net.reject.party_full`), ranked eligibility (`net.reject.run_locked`, §9.1). Canon §2.4's lobby handshake carries `run_seed`; before `Commit` no seed exists, so `Hello` carries none and `RunHeader` supplies it at `Commit` (Canon gaps G7).

`run_id = mix64(host_account_id ^ mix64(run_seed ^ mix64(commit_unix_ms)))`. Example: host `0x0123456789abcdef`, seed `0xC6F2E12DDCA712AB`, `commit_unix_ms = 1791763200123` give `run_id = 0xdba5cd5ae5eebfe8`.

### 3.9 Time synchronization

Each client sends `Ping` every 500 ms. The host answers at once with `Pong`, where `host_tick` is the next tick it will simulate and `host_tick_age_us` is the time since that tick's scheduled start. The client keeps `rtt_us` as an exponentially weighted mean (`rtt += (sample − rtt) / 8`, integer µs) and `host_tick_est(now) = host_tick + (rtt/2 + host_tick_age_us + elapsed_us) / 16667`. Snapshots refine the estimate (§5.6). `rtt_ticks = ceil_div(rtt_us, 16667)`.

---

## 4. Session state machines

### 4.1 Client

| State | Entered when | Event | Next |
|---|---|---|---|
| `net.cstate.disconnected` | start; `Bye`; `Reject`; migration failed | player joins a lobby, enters an address, or uses Quick Join | `connecting` |
| `net.cstate.connecting` | transport connect issued | transport `Connected` | `handshaking` |
| | | connect error, or 5,000 ms | `disconnected` |
| `net.cstate.handshaking` | `Hello` sent | `Welcome` with a slot and no run | `lobby` |
| | | `Welcome` with a run and `slot = None` | `awaiting_folio` |
| | | `Welcome` with `migrating_from_tick` accepted | `loading` (keyframe pending) |
| | | `Welcome` with a run and slot 4 (invader) | `loading` (edit backlog and keyframe pending) |
| | | `Reject`, or 5,000 ms | `disconnected` |
| `net.cstate.lobby` | | `RunHeader` (doc 14 `Commit`) | `loading` |
| `net.cstate.awaiting_folio` | drop-in or rejoin pending (§9.1) | `JoinReady` then `FolioEnter` for that Folio | `loading` |
| `net.cstate.loading` | `FolioEnter` received | blueprint hash equal, `Keyframe` decoded | `playing` (sends `FolioReady`) |
| | | blueprint hash differs | stays; requests chunk data (§6.13) |
| `net.cstate.playing` | | no snapshot for `NET_STALL_MS` | `stalled` |
| | | `FolioEnter` for the next Folio | `loading` |
| | | `ev.run_ended` in `RunState` | `lobby` |
| `net.cstate.stalled` | overlay shown, prediction frozen at `PRED_STALL_TICKS` | snapshot arrives | `playing` |
| | | `NET_TIMEOUT_MS` since the last host message, or transport `Disconnected` from the host | `migrating` |
| `net.cstate.migrating` | §9.3 | this client is the chosen successor and finished catch-up | host role (§9.3) |
| | | `MigrateAnnounce` received, connected, `Welcome` accepted | `loading` |
| | | every successor timed out | `disconnected`; the Folio-entry state is kept for resume (doc 14 §12.2) |

### 4.2 Host, per peer

| State | Entered when | Event | Next |
|---|---|---|---|
| `net.peer.pending` | transport `Connected` | valid `Hello`, run not started | `lobby` |
| | | valid `Hello`, run started, party member | `waiting` |
| | | valid `Hello` with `migrating_from_tick` | `loading` |
| | | valid `Hello`, role invader, invasion slot free (§9.13) | `loading` |
| | | invalid `Hello`, or 5,000 ms | `Reject`, closed |
| `net.peer.lobby` | | `Commit` | `loading` |
| `net.peer.waiting` | drop-in or rejoin pending | `JoinRequest` held at the exit-commit tick of the current Folio | `loading` at the next Folio entry |
| `net.peer.loading` | `FolioEnter` sent | `FolioReady` | `in_game` |
| `net.peer.in_game` | | silence for `NET_TIMEOUT_MS`, or transport `Disconnected` | `lost` |
| | | `Bye` | closed; slot `Departed` (doc 14 §8.8) |
| `net.peer.lost` | `SlotDormant` session intent at the next tick | reconnect with `resume_slot` | `waiting` |
| | | `DORMANT_GRACE_S` (300 s) elapses | closed; `SlotDeparted` |

While a peer is `loading` or `lost`, its slot's commands are substitutes (§5.4), so its Erratum stands still. The Incipit is a sanctuary (doc 14 §3.3), so a loading Erratum is safe.

### 4.3 Lobby messages

Doc 14 §2.1 owns the lobby states; the network carries them with four control messages:

| Type | ID | Direction | Body |
|---|---|---|---|
| 0x70 | `net.msg.lobby_settings` | H → C | `LobbySettings` on every change |
| 0x71 | `net.msg.lobby_pick` | C → H | `{ slot: u8, erratum: ErratumChoice }` (doc 14 `ErratumSelect`) |
| 0x72 | `net.msg.lobby_ready` | C → H | `{ ready: bool }` |
| 0x73 | `net.msg.run_header` | H → C | `RunHeader` at `Commit` |

```rust
pub struct LobbySettings {
    pub privacy: Id32,          // net.lobby.private | friends | public
    pub join_open: bool,        // drop-in allowed during the run (default true)
    pub ff: Id32,               // 0 = default, else mode.ff_off | mode.ff_full (Canon §17.5)
    pub loot: Id32,             // 0 = instanced, else mode.loot_shared
    pub invasions: bool,        // default false (§9.13)
    pub voice: Id32,            // net.voice.off | net.voice.party | net.voice.proximity
    pub password_set: bool, pub mode: Id32, pub grade: u8, pub mutators: Vec<Id32>, pub seed: Option<u64>,
}
```

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

`next` is the tick the host simulates next. **Slack** is how many ticks early the newest command arrived. The host reports, per slot, in every snapshot header: the newest slack, and `target_slack = clamp(2 + ceil_div(spread, 2), 2, 8)`, where `spread = s[113] − s[6]` over the sorted 120-entry history (the 5th-to-95th percentile spread). Examples: spread 0 → 2; spread 3 → 4; spread 6 → 5; spread 20 → 8.

### 5.4 Missing input

When no command for tick `t` is buffered, the host applies the neutral substitute:

```rust
const HOLD_MASK: u32 = (1 << 4) | (1 << 24) | (0b111 << 15);    // CROUCH, WALK, QUICKBAR_INDEX
fn substitute(last: &PlayerCommand, t: u64) -> PlayerCommand {
    PlayerCommand { tick: t, slot: last.slot, move_x: 0, move_z: 0, yaw: last.yaw, pitch: last.pitch,
                    buttons: last.buttons & HOLD_MASK, action_seq: last.action_seq, view_tick: t }
}
```

The substitute keeps stance and the selected quickbar index, releases every action button, moves nowhere and never rewinds. It contains no press edge, because every bit it holds was already held, so it never starts an action (Canon gaps G4). The replay log records the substitute, since Canon §2.4 logs the applied command.

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
| `|slack − target_slack| > 30` | hard reset: `P = host_tick_est + target_slack + rtt_ticks`, the prediction ring is cleared and `slack_ewma = target_slack × 1000` |

Example: `target_slack = 2`, `slack_ewma = 2000`, then six snapshots report slack 0: the average falls 1750, 1532, 1341, 1174, 1028, 900; at the sixth, `err = −1100`, so the client switches to 63 Hz and gains 3 ticks of lead per second.

### 5.7 Intents

- Intents (doc 02 §5.5) travel as `IntentMsg` on the reliable control channel, tagged with the predicted tick at which the player issued them.
- The host applies an intent at `apply_tick = max(intent.tick, next)`, at the start of that tick in `(slot, action_seq)` order. The CIL and the replay record `apply_tick`.
- An intent with `action_seq <= last_intent_seq` is a duplicate and is dropped. A slot may apply at most 8 intents per tick; the rest wait for the next tick in order.
- The owning document validates each payload; a rejected intent emits a diagnostic and the owner sees the rejection in its next `OwnerDelta`.
- This chapter adds intent kinds: `TradePropose { to_slot }`, `TradeRespond { accept }`, `TradeSetItems { uids }`, `TradeLock { locked }`, `TradeConfirm`, `TradeCancel` (§9.7), `DropItemFor { uid, qty, to_slot }` (§9.6), `MarkTarget { target }` (ghosts, §9.5), and the host-only `Session(SessionIntent)`:

```rust
pub enum SessionIntent {                        // logged with slot = affected slot, applied before player intents
    SlotJoin { slot: u8, character: JoinCharacter },  // at a Folio-entry boundary only (§9.1)
    SlotDormant { slot: u8 }, SlotResume { slot: u8 }, SlotDeparted { slot: u8 },
    HostChanged { from_slot: u8, to_slot: u8 },     // 0xFF = dedicated server; no simulation effect
    FolioRestart { folio: u32 },                     // cold migration (§9.3); counts as a ranked restore
}
```

Invaders use doc 02's host-issued slot-4 intents `InvaderJoin { kit: Id32 }` and `InvaderLeave { reason: u8 }`; this chapter fixes the reasons: 0 left or disconnected, 1 invader died, 2 time limit, 3 Redacted, 4 Colophon entered, 5 blocked, 6 host migration (§9.13).

### 5.8 Sanitization and protocol violations

`sanitize` maps `move_x`, `move_z` of −128 to −127, clamps `pitch` to ±16000, and replaces a command whose reserved button bits 26–31 are set, whose `WEAPON_SELECT` exceeds 3 or whose `action_seq` decreased, with the substitute of §5.4. Each replacement, malformed packet, oversized intent (> 4,096 bytes) or intent beyond 120 per second counts one violation. More than 600 violations within 3,600 ticks disconnect the peer with `net.reject.protocol_violation`.

---

## 6. Replication

### 6.1 What travels where

| State | Path | Rate | Reliability |
|---|---|---|---|
| Remote entity presentation (position, rotation, animation, flags, health bar, visible statuses, equipment visuals) | snapshot `ENTITIES` section | 30 Hz, priority-limited | unreliable, delta vs acked baseline |
| Own mover, exact (prediction base) | snapshot `OWNER` section | 30 Hz | unreliable, delta |
| Own pools, exact (HUD) | snapshot `HUD` section | 30 Hz | unreliable, delta |
| Presentation events | snapshot `EVENTS` section | 30 Hz | resent until acked or 30 ticks old |
| Liquids, gases, fire (presentation) | snapshot `FIELDS` section | as dirty, priority-limited | unreliable, delta |
| Blocks and crack stages | `WorldEdits` | per tick with edits | reliable, ordered |
| Inventory, equipment, stats, skills, statuses with timers, Lucre, offers, trade | `OwnerDelta` | ≤ 10 Hz on change | reliable |
| Party summaries | `PartyState` | 5 Hz | reliable |
| Folio phase, Rubric, notes, vote holds, Erasure clock, score | `RunState` | on change | reliable |
| Type knowledge | `KnowledgeDelta` | on change | reliable |
| Applied inputs | CIL | 10 Hz | reliable |
| State hashes | `HashReport` | every 60 ticks | reliable |

The client never receives another player's inventory, unknown item properties (doc 10 §2.8) or any entity outside the relevance rules of §6.6.

### 6.2 Snapshot timing

For every even tick `t`, after `Sim::step(t)` returns, the sim thread extracts a `ReplFrame` for boundary `b = t + 1` (Canon §2.2 period 2, phase 0; budget 0.40 ms): quantized rows for every live entity, the presentation pages of chunks whose fields changed, and the tick's presentation events. The net thread encodes one snapshot per in-game client from that frame and sends it at once. Snapshot boundaries are therefore odd: 1, 3, 5, … (Canon gaps G5).

### 6.3 Net handles and content indices

- Each client has a table of 8,191 entity handles. A handle is allocated, lowest free first, when an entity becomes relevant to that client, and freed when a snapshot carrying its removal is acknowledged. A handle is never reused while an unacknowledged snapshot mentions it.
- A **content index** `cidx(kind, id)` is the rank of a record among all loaded records of its kind (visuals, animations, statuses, liquids, gases, block causes), sorted by content-ID bytes. Equal `content_hash` guarantees equal indices on every peer, so packets carry 16-bit indices instead of 32-bit `id32`s.

### 6.4 Replicated entity state and quantization

| Bit | Group | Encoding | Quantization and reconstruction |
|---|---|---|---|
| 0 | POS | 1 bit `abs`; abs: x 18, y 16, z 18 bits; delta: per axis a 2-bit class `c` and a two's-complement value of `[0, 6, 11, 16][c]` bits | `q = clamp(to_mm(p), 0, 262143)` for x and z, `0..=65535` for y; delta from the baseline's quantized value; abs when any axis moved more than 32,767 mm |
| 1 | ROT | yaw 12 bits, pitch 10 bits | `yaw_q = yaw >> 4`, shown as `(yaw_q << 4) | 8`; `pitch_q = (pitch + 16000) >> 5`, shown as `pitch_q × 32 − 15984` |
| 2 | ANIM | animation `cidx` 16 bits, start tick 16 bits | the client derives `anim_tick(b) = b − start` (low 16 bits), wrapped or clamped by the pose table |
| 3 | FLAGS | 16 bits | 0 dead, 1 downed, 2 ghost, 3 teleported, 4 invisible, 5 mounted, 6 hostile to the receiver, 7 marked by the receiver's party, 8 burning, 9 Contradicted, 10 boss, 11 speaking, 12 asleep, 13–15 zero |
| 4 | HEALTH | hp 8 bits, ward 4 bits | `hp_q = ceil_div(hp × 255, hp_max)` (0 only at 0 HP); `ward_q = min(15, ceil_div(ward × 15, hp_max))` |
| 5 | STATUS | count 4 bits, then per status: Canon §11.1 row index 7 bits, stacks 4 bits | at most 15, in Canon §11.1 row order; stacks capped at 15 |
| 6 | EQUIP | 4 × 16-bit visual `cidx` (main hand, off-hand, head, body; 0xFFFF none), 4 illuminated bits | |
| 7 | EXTRA | light 4 bits, attach handle 13 bits (rider ↔ mount, 8191 none), aux 8 bits | aux: containers, the opened-by slot mask; zones, radius in 250 mm units |

Examples: position (12.3456 m, 3.0 m, 200.0004 m) → (12345, 3000, 200000); yaw 16400 → 1025 → shown 16408; pitch −300 → 490 → shown −304; HP 42,500 of 85,000 → 128; HP 400 of 85,000 → 2. Quantization affects presentation only: the predictor's own state arrives exact in `OWNER` (§7.2), and no client decision reads quantized values back into the simulation.

### 6.5 Delta compression and baselines

```rust
struct View { ents: BTreeMap<u16, EntRepl>, pages: BTreeMap<u8, FieldPage>, mover: Option<MoverSnap>, hud: Option<Hud> }
struct ClientRepl {
    views: BTreeMap<u32, Arc<View>>,   // by boundary; at most SNAP_RING
    acked: u32,                        // newest boundary the client acknowledged
    handles: HandleTable, acc: BTreeMap<EntityId, i64>, page_acc: [i64; 128], bucket: i64,
}
```

- The host encodes snapshot `b` against `views[acked]` and stores `views[b] = views[acked] ⊕ records written`. Entities and pages not written in `b` carry over unchanged from the baseline.
- `snap_ack` in an input packet acknowledges boundary `k`; if `k > acked` and `views` holds `k`, then `acked = k` and every view older than `k` is dropped.
- The client decodes snapshot `b` only when it holds the view at `baseline_tick`, then stores `views[b]` and acknowledges `b` in its next input packet. It buffers at most 32 undecodable snapshots.
- When `b − acked > 64` ticks, or when the host no longer holds `views[acked]`, the host sends a `Keyframe` (§6.13) on the reliable world channel and uses it as the baseline of the following snapshots.
- An entity changes in a snapshot only when its current `EntRepl` differs from the baseline's; unchanged groups are omitted by the mask, and an entity with no changed group writes nothing.

### 6.6 Interest management

Each client's **view origin** is its Erratum; a ghost spectating an ally uses the ally; a loading client uses the Incipit. An entity `e` is relevant to client `c` when the first matching rule says so:

1. **Never** an item drop with visibility `Instanced(s)` and `s ≠ c.slot` (doc 11 §14.4).
2. **PvP culling** (§9.13): an invader and an Erratum are mutually relevant only while visible: every `PVS_PERIOD` ticks the host tests each such pair; the pair is visible when the distance between origins is at most 16,000 mm, or when either of two rays from the viewer's eye (to the target's eye, and to its origin + 900 mm) reaches the target without crossing a `full` block (doc 03 §5.18). A pair unseen for more than `PVS_GRACE_TICKS` is culled.
3. **Always**: every Erratum; every entity with `owner_slot = c.slot`; every template flagged `net_always` (Editor bosses during `BossFight`, hunters, exit doors); the mount of a relevant rider.
4. **Window**: `e` is in a chunk column with `|cx − ocx| <= 2` and `|cz − ocz| <= 2`, the doc 03 §15.1 activation window. An entity leaves only when the distance exceeds 3 columns.
5. **Marked**: `e` carries `st.marked` applied by any member of `c`'s party.

Field pages use rule 4. World edits are global. Events use §6.9.

### 6.7 Priority accumulator and budget

```
w_kind: erratum 1000, boss|editor|hunter 900, projectile 700, hostile creature 600, falling block 600,
        other creature|mount 400, gadget|zone 300, field page 300, container 200, item drop|corpse 100, marker 50
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

1. `bucket = min(bucket + 352, 2112)`; `budget = min(1150, max(0, bucket))` bytes of payload.
2. Mandatory, in order: header, `OWNER`, `HUD`, events concerning the receiver's own slot (at most 128 bytes). These are written even when they exceed `budget`.
3. Candidates: relevant entities whose state differs from the baseline, creates and removals (`acc += 100000` once when they first become pending), and dirty field pages. Each pending candidate adds its `prio` to `acc`.
4. Candidates are sorted by `acc` descending, ties by ascending handle (pages after entities, by chunk slot). Records are written in that order while they fit; a record that does not fit is skipped, and writing stops after 8 consecutive skips. Remaining events fill the leftover bytes.
5. Every written candidate resets `acc` to 0. `bucket −= bytes written`, never below −2112.

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
| 0x02 | `HUD` | `mask: u16`, then changed pools as `zz` varints of the difference in `Milli`: hp, hp_max, stamina, stamina_max, aether, aether_max, composure, composure_max, ward, breath, poise, toxicity; satiation `zz`; `light_at` u8; `bleed_left` u16 while downed |
| 0x03 | `ENTITIES` | `count: varint`, then a bit stream (LSB first) of records in ascending handle order, padded to a byte |
| 0x04 | `FIELDS` | §6.12 |
| 0x05 | `EVENTS` | `count: varint`, then a bit stream of §6.9 records |

Entity record: `handle` 13 bits, `op` 2 bits (0 update, 1 create, 2 remove). An update follows with an 8-bit group mask and the groups of §6.4. A create carries `entity_id` 64 bits, `kind` 4 bits (doc 03 §3.2 row index), `owner_slot` 3 bits (7 = none), visual `cidx` 16 bits, then a mask with every group the kind uses and those groups in full. A remove carries nothing more. A walking creature's update is 13 + 2 + 8 + 29 (POS delta: abs bit, two 13-bit axes, one empty axis) + 22 (ROT) = 74 bits; a create is 334 bits.

### 6.9 Presentation events

An event record is `kind` 6 bits (row of the table below), `age` 6 bits (`b − event tick`), host `seq` 16 bits (the event's `SimEvent.seq`), then the payload. The host keeps each client's outbox until a snapshot carrying the record is acknowledged or the record is 30 ticks old; the client discards duplicates by `(tick, seq)` and plays each event when its interpolation time reaches the event tick.

| Row | Event | Sent to | Payload bits |
|---|---|---|---|
| 0 | `ev.damage_dealt` | the attacker's and target's clients; window clients when the target is an Erratum | target 13, attacker 13, display amount 16 (`ceil_div(total_mhp, 1000)`), dominant type 4, flags 6 (crit, precision, sneak, blocked, ward, lethal) |
| 1 | `ev.entity_killed` | window | entity 13, killer 13 |
| 2 | `ev.explosion` (03) | window, or `listener_db[slot] >= 0` | absolute position 52, radius 8 (250 mm units) |
| 3 | `ev.noise` | `listener_db[slot] >= 0` (doc 03 payload) | absolute position 52, dB 8, noise `cidx` 16 |
| 4 | `ev.footstep` (03) | window | entity 13, gait 2, material `cidx` 12 |
| 5 | `ev.status_applied` | window | entity 13, status row 7, stacks 4 |
| 6 | `ev.spell_cast` | window | caster 13, spell visual `cidx` 16 |
| 7 | `ev.fire_started`, `ev.fire_out` (03) | window | `Gidx` 22, kind 2 |
| 8 | `ev.liquid_reaction` | window | `Gidx` 22, kind 4 |
| 9 | `ev.ward_broken` | window | entity 13 |
| 10 | `ev.part_severed` | window | entity 13, part row 4 |
| 11 | `ev.player_downed`, `ev.player_revived`, `ev.player_died`, `ev.player_ghosted`, `ev.player_resurrected` | all clients | event variant 3, slot 3, entity 13 |
| 12 | `ev.hunter_spawned` | all clients | entity 13 |
| 13 | `ev.mounted`, `ev.dismounted` | window | rider 13, mount 13, variant 1 |
| 14 | `ev.item_picked_up`, `ev.item_dropped` | window, Public items only | entity 13, item visual `cidx` 16, variant 1 |
| 15 | `ev.invasion_started`, `ev.invasion_ended` | all clients, invader included | variant 1, invader entity 13, reason 3 |

Events that only the owner needs (`ev.skill_gained`, `ev.level_gained`, `ev.contradiction_changed`, `ev.item_identified`, `ev.engine_spun`, `ev.erasure_stack`, `ev.affliction_changed`, `ev.container_opened`) travel inside `OwnerDelta`. `ev.block_changed` travels as world edits. `ev.run_ended` travels in `RunState`.

### 6.10 Owner, party and run state

- `OwnerDelta` (postcard), at most every 6 ticks, sent at once after an intent resolves: `{ tick, items: Vec<ItemView> /* upserts, doc 10 */, removed: Vec<u64>, equipment, quickbar, stats, skills /* value and milli-XP */, level, xp, statuses: Vec<StatusView /* id, stacks, expiry */>, lucre, offers, trade: Option<TradeView>, rejected: Vec<u32 /* action_seq */>, owner_events: Vec<NetOwnerEvent> }`. The first `OwnerDelta` after Folio entry is complete.
- `PartyState` every 12 ticks: per slot `{ status: SlotStatus /* doc 14, plus Waiting and Invader */, persona, level, hp_bp, ward_bp, bleed_left, ping_ms, shadow: bool }` and the ordered `successors: Vec<u8>` (§9.3).
- `RunState` on change: Folio index and genre, doc 14 §3.2 phase, Rubric progress, marginal notes, the per-door hold counters of `ExitVote`, Erasure clock, displayed score accumulators, run end.

### 6.11 Voxel diffs

All block changes replicate to every client, inside or outside interest, because terrain is shared state and the limited-building rule (DECISIONS #16) keeps edits sparse.

| Offset | Size | Field |
|---|---|---|
| 0 | 4 | `rseq: u32` (reliable framing) |
| 4 | 1 | `msg_type` = 0x22 |
| 5 | 4 | `tick: u32` |
| 9 | 1 | `part`: bit 7 set when more parts of the same tick follow |
| 10 | v | `edit_count: varint`, then per edit in `ev.block_changed` emission order: `zz(gidx − prev_gidx)` (prev starts at 0), `state: u16`, `cause: u8` (`cidx` of `grid.cause.*`) |
| … | v | `crack_count: varint`, then per cell whose crack stage changed: `zz(gidx − prev_gidx)`, `stage: u8` in `0..=9` = `floor_div(accumulated × 10, block_hp)` (doc 03 §13.2) |

A part holds at most 4,096 edits. The client keeps a **pending overlay** `BTreeMap<Gidx, BlockState>` of received edits whose tick its render timeline has not reached; prediction collision reads the overlay first and the render grid second, and the renderer applies a batch to the render grid when `render_tick >= tick`. Example: an edit at `Gidx` 2,099,204 to `block.air` (state 0) by cause index 3 after a previous edit at 2,099,200 encodes as `08 00 00 03`. After a batch reaches the render grid, the client runs doc 03's relight on its presentation light grid with the same 4,096-update budget per tick; presentation light is never compared with the host's, and the HUD light gem reads `HUD.light_at`.

### 6.12 Field pages

A field page is the presentation state of one chunk's non-empty fluid and fire cells. Presentation cell: `liq_level = min(15, ceil_div(total_units × 15, 1000))`, `liq_type` = the component with the most units (ties to the lower type index), `gas_conc = min(15, ceil_div(conc × 15, 10000))` of the strongest gas, `gas_type`, `fire` = the `FireCell` intensity (0 to 3), `charged` = the liquid is electrified (doc 03 §6.9).

`FIELDS` body: `page_count: varint`; per page: `slot: u8`, `full: u8` (1 = all non-empty cells, 0 = changes since the baseline page), `cell_count: varint`; per cell in ascending `Local`: `local_delta: varint` (from the previous `Local`, first from −1), `rec: u16` (bits 0–3 liquid level, 4–7 gas concentration, 8–9 fire, 10 charged), then `liq_type: u8` when the liquid level is above 0 and `gas_type: u8` when the gas concentration is above 0. In a delta page, `rec = 0` means the cell became empty. Example: a cell at `Local` 1,057 holding 437 units of the liquid with type index 2 encodes, as the first cell of a page, `a2 08 07 00 02` (`liq_level = ceil_div(6555, 1000) = 7`).

### 6.13 Folio entry and keyframes

At each Folio-entry boundary the host sends, in this order:

1. `FolioEnter` (control): `{ folio, genre, gen: GenInput /* doc 12 */, entry_tick, blueprint_hash: [u8; 32], section_hashes: [[u8; 32]; 12], slots: Vec<(u8, SlotStatus)> }`.
2. `FolioState` (world): zstd of serialization sections 0, 1, 2, 3, 8, 9, 10, 11 at the entry boundary. Every client stores it with the regenerated sections 4–7 as its Folio-entry save copy (doc 14 §12.1).
3. `Keyframe` (world): the client's full relevant view at the entry boundary (every relevant entity as a create record, every field page in the window as a full page, `OWNER`, `HUD`), in the snapshot section format, with `baseline_tick = 0`.

The client generates the Folio with doc 12's pure generator, compares `blueprint_hash`, rebuilds sections 4–7 at the entry boundary and compares their section hashes. On any mismatch it sends `FolioReady { gen_ok: false }`, the host sends `ChunkData` (zstd of sections 4–7), and the client counts one generation desync in telemetry (doc 12 I2). Generation runs on a worker thread within doc 02's 1,500 ms load budget.

### 6.14 Confirmed Input Log

The CIL is the exact `TickInputs` the host applied, the same data as the replay log.

| Offset | Size | Field |
|---|---|---|
| 0 | 4 | `rseq: u32` |
| 4 | 1 | `msg_type` = 0x30 |
| 5 | 4 | `first_tick: u32` |
| 9 | 1 | `ticks: u8` (6 in steady state; up to 255 in a backlog) |
| 10 | … | per tick: `slot_mask: u8` (bits 0–4); per slot in the mask: one command as an input-packet delta against that slot's previous CIL command, with mask bit 6 = substitute and bit 7 = full; then `intent_count: varint` and that many `(slot: u8, apply_tick delta: varint, action_seq: varint, postcard IntentKind)` records, session intents first |

A slot's first command after a `SlotJoin`, a resume, or the start of a stream is written full. Clients that join late receive the backlog from tick 0 at 8,000 B/s while they wait for their Folio, so every party member ends the run with the complete log.

---

## 7. Client prediction and reconciliation

### 7.1 Scope

| Predicted exactly (corrected by reconciliation) | Predicted cosmetically (replaced by host results) | Never predicted |
|---|---|---|
| the client's own mover in every doc 03 mode: ground, air, crouch, lean, dodge, mantle, swim, climb, downed crawl, ghost flight; when mounted, the mount's mover with the rider at its seat | weapon fire: muzzle, tracer, recoil kick, spread and ammo count, using keyed `rng.combat` with the host's key (actor, tick, `seq`), so predicted spread equals the host's (Canon §3.3, doc 05 draw table); melee swing; cast start; thrown and spell projectile stand-ins; block placement and breaking stand-ins; impact decals | damage, hit markers and numbers, deaths, statuses, pickups, doors, AI, other players |

A cosmetic projectile stand-in disappears when the host projectile with the same owner and spawn tick arrives, or after `rtt_ticks + 12` ticks. A predicted block edit is held in the pending overlay and reverted if no host edit for that cell arrives within `rtt_ticks + 12` ticks.

### 7.2 Mover state

```rust
pub struct MoverSnap {                 // exact; compared with ==
    pub tick: u64,                     // boundary
    pub p: Fx3, pub v: Fx3, pub yaw: Bam, pub pitch: i16,
    pub mv: MoveState,                 // doc 03 §3.4, every field
    pub gates: u32,                    // movement-relevant statuses active, one bit per row below
    pub speed_bp: Bp,                  // product of active move-speed status multipliers (Canon §4.2 order)
    pub genre: Id32, pub ride: Option<EntityId>,
}
```

Stamina is not part of `MoverSnap`. The predictor permits a dodge while the latest authoritative stamina (`HUD`) minus the dodges it predicted since then is at least 20,000 (Canon §4.2), and sprint while that value is above 0; a wrong permit is corrected like any other divergence. `gates` and `speed_bp` are copied from the newest authoritative state into every predicted step.

`gates` bits: 0 staggered, 1 stunned, 2 frozen, 3 knocked down, 4 sleeping, 5 rooted, 6 grappled, 7 overburdened, 8 exhausted, 9 levitating, 10 downed, 11 ghost, 12 leg severed, 13 crutch, 14 chilled, 15 slowed, 16 hasted, 17 encumbered, 18 mounted.

The predictor calls doc 03's pure controller `step_mover(&mut MoverSnap, &Control, &MoverParams, &dyn BlockQuery) -> MoverOut`, the same function `sys.movement` runs for Errata on the host (Interface assumptions, doc 03). `MoverParams` comes from `OwnerDelta` (row 11 move-speed multiplier, body dimensions, carried mass) and the Folio's genre physics. `BlockQuery` reads the pending overlay, then the render grid, then the blueprint's gravity-flip and zone data. Entity push, knockback and mount collisions are host-only and reach the client as corrections.

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

- `render_tick = host_tick_est − interp_delay` in µs precision. Each remote entity keeps the samples `(snap_tick, EntRepl)` of the snapshots that wrote it; the renderer interpolates between the two samples that bracket `render_tick` (doc 02 §7.3 rules for yaw and teleports).
- `interp_delay = clamp(4 + ceil_div(2 × jitter_us, 16667) + 2 × floor_div(loss_bp, 500), 4, 12)` ticks, recomputed every 60 ticks from snapshot arrival jitter (5th-to-95th percentile spread of inter-arrival deviation) and loss over the last 300 snapshots, and moved by at most 1 tick per 60 ticks. Examples: jitter 12 ms, loss 1% → 6; jitter 12 ms, loss 6% → 8; jitter 40 ms, loss 16% → 12; a clean LAN → 4.
- Nothing extrapolates (Canon §2.1). When `render_tick` passes an entity's newest sample, the entity holds that sample; after 6 ticks without a new sample it is drawn as stale (desaturated, doc 17) until one arrives.
- Events play at `render_tick`; world edits apply at `render_tick` (§6.11).

---

## 8. Lag compensation

The host applies doc 05 §2.6 with the clamped `view_tick` of §5.5:

1. **Who rewinds.** Hitscan rays and melee samples of Errata and of the invader (§9.13) test every other entity's hitbox capsules posed at `t_r = t − rewind`, `rewind = min(t − view_tick, 16)`. NPC attacks, explosions, area effects, surfaces and shock chains never rewind.
2. **Projectiles.** An Erratum's or invader's projectile spawned at `t` integrates `rewind` catch-up steps against poses `t_r + i + 1`, then continues at `t + 1` against current poses (doc 05 §2.6).
3. **What rewinds.** Only target transforms and poses, from `Pose.hist` (Canon §9, doc 03 §5.16). Blocks, the shooter's own position, target state (guard, parry window, dodge invulnerability, statuses, ward, HP, alive or dead) always read tick `t`.
4. **Missing history.** A target whose `spawn_tick > t_r` is not hittable by that rewound test. A target that died or despawned before `t` is never hit.
5. **Replays** reproduce every rewound hit from the logged, already clamped `view_tick` (Canon §2.4).

Example: at `t = 5000` a client with RTT 100 ms fires with `view_tick` 4982; §5.5 applies 4984, so `rewind = 16` and the ray tests poses recorded at tick 4984.

---

## 9. Co-op rules

### 9.1 Drop-in at Folio boundaries

1. A player joins a run in progress when `join_open` is true and a seat is free (party slots 0–3). The host answers `Welcome { slot: None }`; the client enters `awaiting_folio`, receives the CIL backlog, and creates or picks an Erratum on the join screen (doc 14 §2.5 values).
2. The client sends `JoinRequest { character }`. If it is held at the exit-commit tick of the current Folio (doc 14 §3.6), the host enqueues `SessionIntent::SlotJoin` for the next Folio-entry boundary; otherwise the join waits one more Folio.
3. `enter_folio` (doc 14 §3.3) consumes pending `SlotJoin` intents in ascending slot order before the Folio-entry save, so the joiner is in the save, in `n` (§9.10) and in the CIL.
4. **Ranked runs.** A joiner needs a ranked token for the run (§11.6); for `mode.daily` the token consumes the joiner's attempt for that UTC date, and a joiner whose attempt is used is refused with `net.reject.run_locked`. The board entry uses the largest party size the run reached.
5. No `SlotJoin` takes effect at the entry of the run's final Folio (`final_folio`, doc 14 §3.1); such a joiner waits for Marginalia or the run's end.

### 9.2 Leaving, Dormant and reconnect

- **Transient loss.** Up to `NET_TIMEOUT_MS` of silence, the slot stays in the world on substitute commands and nothing else happens.
- **Dormant.** At timeout the host enqueues `SlotDormant` for the next tick; doc 14 §8.8 then removes the Erratum from the world with its inventory. Kill XP and loot rolls skip Dormant slots.
- **Reconnect.** A Dormant player reconnects with `resume_slot`, waits in `awaiting_folio`, and returns through `SlotResume` at the next Folio entry. After `DORMANT_GRACE_S` (300 s wall clock) the slot becomes `Departed` (`SlotDeparted`).
- **Steam Deck suspend** looks like transient loss, then Dormant (doc 02 §14).
- **Kick.** The host kicks with `SlotDeparted` and `Bye { reason: net.reject.kicked }`. In public lobbies any member starts a kick vote; with `m` Active members other than the target, it passes when at least `max(2, m − 1)` of them vote yes within 1,800 ticks (2 of 3 in a full party). A kicked account cannot rejoin that session.

### 9.3 Host migration

**Successor list.** Every 300 ticks the host ranks the other Active party clients and publishes the order in `PartyState.successors` and the lobby key `pal_succ`. Rank keys, compared in order: (1) Shadow running and in sync with the last `HashReport`; (2) lower mean estimated RTT to the other members (Steam ping locations, else RTT to the host); (3) not handheld; (4) lower slot. Invaders and Dormant slots are never successors.

**Detection.** A client enters `migrating` when the host's connection reports `Disconnected` or after `NET_TIMEOUT_MS` of silence. The successor at rank `k` (0-based) waits `k × MIGRATE_ANNOUNCE_MS` without seeing a `MigrateAnnounce` before acting.

**Catch-up.** Let `L` be the successor's last complete CIL tick.

| Mode | Condition | Procedure |
|---|---|---|
| `net.migrate.hot` | a Shadow is running | step the Shadow to boundary `L + 1` |
| `net.migrate.warm` | no Shadow; estimated time `(L + 1 − entry_tick) × measured_tick_us ≤ MIGRATE_WARM_MAX_MS` | `Sim::deserialize` the Folio-entry save copy, then step every CIL tick to `L + 1` on the worker thread, comparing `HashReport`s along the way |
| `net.migrate.cold` | otherwise, or any hash mismatch during warm catch-up | restore the Folio-entry save; log `FolioRestart`; the Folio replays from its entry, and ranked runs count one restore (doc 14 §12.3) |

At 6,000 ticks per second a 300 s Folio (18,000 ticks) catches up in 3 s; at 500 ticks per second the same Folio needs 36 s, still warm; at 160 ticks per second it needs 112.5 s and migration is cold.

**Takeover.**

1. The successor opens a listen socket (`net.route.sdr`: `CreateListenSocketP2P`; direct: its LAN address from `PartyState`), becomes the Steam lobby owner (the current Steam owner calls `SetLobbyOwner(successor)` when it sees `pal_succ`) and sets `pal_host`.
2. It posts `MigrateAnnounce { new_host_slot, resume_tick: L + 1, mode, cil_hash }` as a Steam lobby chat message (relay sessions: through `svc.lobby`).
3. It enqueues `HostChanged { from_slot, to_slot }` and `SlotDormant { from_slot }` for tick `L + 1` and resumes stepping at real time.
4. The other clients truncate their CIL to `L`, reconnect with `Hello { migrating_from_tick: L }`, and receive `Welcome`, `OwnerDelta` and `Keyframe` (no `FolioEnter`); they keep their Folio-entry save copies. A client that does not reconnect within `NET_TIMEOUT_MS` becomes Dormant on the new host.
5. An invader is dropped (§9.13).

Inputs the old host applied after `L` are lost: at most `CIL_BATCH_TICKS` plus the one-way delay in ticks, which is 9 ticks at RTT 100 ms.

### 9.4 Downed and revive (owner of `st.downed`)

```rust
pub struct Player {                    // comp.player, semantics owned here (doc 03 §3.3)
    pub slot: u8, pub downs_folio: u8, pub bleed_left: u16, pub revive_progress: u16,
    pub reviver: Option<EntityId>, pub downed_by: Option<EntityId>,
    pub mark_cooldown_until: Tick, pub trade: Option<TradeState>,
}
```

- **Entry** (Canon §10.8 step 2): HP 0, `st.downed` without expiry, `downs_folio += 1`, `bleed_left = max(600, 1800 − 600 × (downs_folio − 1))`: 1,800, 1,200, then 600 ticks. Movement mode `downed`, crawl 800 mm/s (doc 03 §5.1). The Erratum keeps its capsules and cannot attack, block, cast, reload, use items, interact, sprint, jump, dodge or mount (doc 05 §14.3); it can look, crawl, ping and chat.
- **Damage while downed** runs the full pipeline; HP stays 0, and each resolved hit shortens `bleed_left` by `ceil_div(total_mhp × 1800, max_hp_mhp)` (doc 05 §14.3; 17,000 mHP at Max HP 85,000 removes 360 ticks). DoT pulses count as hits.
- **Bleed.** `sys.coop` (Interface assumptions, doc 03) decrements `bleed_left` by 1 each tick on which no valid reviver acts. At 0, Canon §10.8 step 3 runs (no new Death's Door roll).
- **Valid reviver** on tick `t`: an Erratum that is alive, not downed, not a ghost, not mounted, free of `st.staggered`, `st.stunned`, `st.frozen`, `st.knocked_down`, `st.sleeping` and `st.grappled`, holds `INTERACT`, has its origin within 1,500 mm of the downed origin, and faces it within 8,192 Bam. The lowest-`EntityId` valid reviver is the active one; several revivers do not stack.
- **Progress.** `revive_progress += 1` on every tick with a valid reviver, and resets to 0 on a tick without one. Revival completes when `revive_progress >= revive_ticks = max(30, mul_bp(180, reviver.revive_bp))`, where `revive_bp` is the product of the reviver's talent multipliers (doc 04; Warden `resurgence` 5000 gives 90 ticks). Doc 04's Restoration-100 spell completes revival at once.
- **Completion.** `st.downed` is removed, HP is set (not healed, so Canon §10.9 does not apply) to `mul_bp(max_hp_mhp, revive_hp_bp)` with `revive_hp_bp = max(2000, 5000 − 150 × grade)` (R7 R-CO3): Grade 0 → 5000 (42,500 mHP at Max HP 85,000), Grade 2 → 4700 (39,950), Grade 10 → 3500, Grade 20 → 2000. Poise refills. `ev.player_revived { entity, slot, reviver }` is emitted.
- **Folio exit.** A downed Erratum leaves with the party (doc 14 §3.6) and enters the next Folio revived at `revive_hp_bp`. `downs_folio` resets at Folio entry.
- Downed, revive and bleed draw no random numbers.

### 9.5 Ghosts (owner of `st.ghost`)

Canon §17.6 governs; this section fixes the remaining details.

- The Erratum entity stays in the world with `Kind.flags & GHOST`, flies at walk speed with voxel collision only (doc 03 §5.12), is excluded from every hit, push and perception query, and is replicated to its own party only.
- Allowed actions: `INTERACT` on doors, switches and levers; `PING`; `MarkTarget { target }`, which applies `st.marked` for 480 ticks to a non-ghost entity whose origin is within 30,000 mm and visible by doc 03 §5.18 from the ghost's eye, at most once per 600 ticks (`mark_cooldown_until`).
- Remains (`cat.corpse`, 75,000 g) drop at the death position. At the next `ev.folio_entered` the ghost resurrects with `GHOST_REMAINS_HP_BP` (5000) when an ally carried the Remains through the exit, else with `GHOST_BARE_HP_BP` (2500) and the loss of one equipped non-weapon item: one keyed `rng.loot` draw (folio = the new Folio, actor = the ghost's `EntityId`, tick = its `folio_entry_tick`, seq 0), `range(n)` over the eligible items sorted by uid, and 0 draws when `n = 0`. Endpaper restoration follows doc 14 §3.10.
- Doc 14 §3.10's "the entity leaves the world" is read as leaving the physical world (Canon gaps G8).

### 9.6 Instanced loot

- Every kill, container, Reliquary, bag, Engine and shop shelf rolls per Active, non-ghost slot with `seq = slot` (Canon §3.3, doc 11 §17); Dormant and ghost slots receive nothing. Lucre piles are instanced like items (R7 R-CO4).
- An `Instanced(s)` item is never sent to another client (§6.6 rule 1), so instanced loot is private on the wire, not merely hidden.
- `DropItem` makes an item `Public` (doc 11 §14.4). `DropItemFor { uid, qty, to_slot }` drops it as `Instanced(to_slot)` at the dropper's feet, visible only to that player; it fails for a Departed, Dormant or ghost target, for quest items and for Keepsakes.
- `mode.loot_shared` follows doc 11 §17; pickups that collide in one tick resolve in ascending picker `EntityId`.

### 9.7 Trading

A trade swaps items between two Errata atomically. Currencies are never traded (Canon §14).

| State (`net.trade.*`) | Entered when | Event | Next |
|---|---|---|---|
| `idle` | | A sends `TradePropose { to_slot: B }` and both are eligible | `proposed` |
| `proposed` | | B sends `TradeRespond { accept: true }` within 600 ticks | `open` |
| | | decline, 600 ticks, or eligibility lost | `idle` |
| `open` | | either side sends `TradeSetItems` (≤ 8 uids) | `open`, both locks cleared |
| | | one side sends `TradeLock { locked: true }` | `locked` |
| `locked` | one side locked | the other side locks | `both_locked` |
| | | any `TradeSetItems` or unlock | `open` |
| `both_locked` | | both send `TradeConfirm` | `executed` on that tick, then `idle` |
| any | | `TradeCancel`, eligibility lost, or Folio exit | `idle` |

**Eligibility,** checked on every tick of a trade: both alive, not downed, not ghosts, not mounted, not `in_combat` (Canon §2.3), origins within 4,000 mm (a trade in progress breaks beyond 6,000 mm). **Tradeable items:** in the offerer's backpack, not equipped while cursed, not `cat.quest`, not a Keepsake, not `cat.currency`. **Execution:** remove every offered uid from its giver in ascending uid order, then add them to the receivers in the same order; instance knowledge travels with each item (doc 10 §2.5); `finder_slot` is unchanged. The trade emits `ev.item_dropped` for the giver and `ev.item_picked_up` for the receiver per item (Canon gaps G10). Trading draws no random numbers.

### 9.8 Exit voting

Doc 14 §3.6 owns the commit rule (`TURN_HOLD_TICKS` 120, `EXIT_MAJORITY_TICKS` 3,600, `EXIT_ANY_TICKS` 7,200). Votes are positions inside door volumes, so they travel as ordinary movement inputs; there is no vote message. The host replicates each door's hold counter and voter count in `RunState`. Ghosts, Dormant slots and invaders are never voters. A host migration keeps the counters, which are simulation state.

### 9.9 Shared identification knowledge

Type knowledge is party-wide (Canon §13.6, doc 10 §2.2). The host sends `KnowledgeDelta { tick, types: Vec<(Id32 /* type key */, Id32 /* descriptor */, u8 /* state */)>, excluded: Vec<(Id32, Id32)> }` to every party client on change, and the full table in the first message after a join. Instance knowledge travels with the item's `ItemView`. Invaders receive no party knowledge.

### 9.10 Scaling

`n` is the number of Active slots after ghost restoration at Folio entry (doc 14 §3.3); it drives Canon §17.2 for the whole Folio. A slot that goes Dormant mid-Folio changes nothing until the next entry; a joiner counts from the Folio it joins. Invaders never count toward `n`.

### 9.11 Friendly fire options

The lobby's `ff` setting selects the default constants, `mode.ff_off` (direct 0, area 5000) or `mode.ff_full` (both 10000) (Canon §17.5). Ranked modes force the default. The setting is part of `RunConfig` and therefore of the replay.

### 9.12 Chat, voice and pings

- Text chat: at most 200 bytes, 5 messages per 10 s per sender, filtered and reportable per doc 15 §8.5. The host relays chat; it is never stored by any service.
- Voice: Steam builds use the Steam voice API, 24 kbit/s per talker, as `net.msg.voice` frames. Modes `net.voice.off`, `net.voice.party` (default; full volume for the party), `net.voice.proximity` (full volume within 5,000 mm, linear to silence at 30,000 mm from the speaker's origin). Minor-flagged accounts default to off and hear friends only (R3 COMMS-1). Every speaker can be muted per session and blocked per account.
- World pings are the `PING` button (doc 02 bit 23), handled by the simulation, so they appear in replays.

### 9.13 Invasions (network and session rules)

Canon §17.9 fixes what an invasion is; this section fixes how it runs.

- **Opt-in.** The lobby's `invasions` setting puts `mode.invasion_open` into `RunConfig` at run start (default off). Daily, Weekly and every leaderboard-ranked run refuse it. It also requires every party member online, an empty mod set, and every member's account permitting invasions (doc 15 `juris.flag.invasion_minor_default`).
- **Window.** The host advertises `pal_inv = open` only while all hold: `folio_index >= 3`; the Folio is not a boss Folio; `t − folio_entry_tick >= 7,200`; no Erratum has entered `room.colophon`; no invader is present; no invasion has started in this Folio. The flag closes on the tick any condition fails.
- **Matchmaking.** An invader chooses Invade in the Margin; while the window is open, a Steam host also owns a separate public **invasion beacon** lobby (1 member, data `pal_kind = beacon`, `pal_ver`, `pal_proto`, `pal_ch`, `pal_grade`, `pal_run` = `run_id`, `pal_host`, `pal_ploc`) and leaves it the tick the window closes, so private and friends-only parties are findable without exposing their party lobby. Invaders search beacons with equal `pal_ver`, `pal_proto`, `pal_ch` and `|pal_grade − invader grade| <= 2`, and connect by `pal_host` to the lowest estimated ping under 150 ms. `svc.lobby` signs the `invasion_ticket` (§11.5); it refuses accounts with invasions disabled and accounts that a member of that party blocked within 30 days (doc 15 §8.5).
- **Joining mid-Folio,** the one exception to Folio-boundary drop-in: the invader receives `RunHeader` for generation, the world-edit backlog since the Folio entry, and a `Keyframe`. The host enqueues `InvaderJoin { kit }` on slot 4 with the Editor kit the invader picked from its account's kits; doc 13 spawns `enemy.editors.invader`; the entity enters at doc 13's hunter placement anchor, and `ev.invasion_started` is emitted.
- **End.** `InvaderLeave { reason }` with reason 0 (left or disconnected), 1 (invader died; the party's guaranteed Reliquary roll follows Canon §17.9), 2 (`INVASION_MAX_TICKS` since entry), 3 (Redacted), 4 (an Erratum entered `room.colophon`), 5 (a party member blocked the invader) or 6 (host migration). Each emits `ev.invasion_ended`; the invader's entity despawns at the end of that tick and its connection closes.
- **Redaction.** When the invader's hit would bring the last Erratum that is alive and not downed to 0 HP, doc 14 resolves Canon §17.9's Redaction on that tick instead of Canon §10.8 steps 2 and 3; every downed Erratum loses `st.downed` without `ev.player_revived`, and the invasion ends with reason 3.
- **Information.** The invader receives snapshots under PvP culling, `RunState`, and slot numbers and personas from `PartyState`; never the CIL, `KnowledgeDelta`, another player's `OwnerDelta`, or Errata inventories. It is never a successor, a voter or part of `n`.
- **Chat** (doc 15 §8.5): voice with the invader starts off for every account; minor-flagged accounts exchange only pings and emotes with it; every channel carries mute, block and report, and a block ends the invasion with reason 5.
- **Outcome report.** At `ev.invasion_ended` the host and the invader each post `POST /v1/invasions/outcome` (§11.5); `svc.econ` pays doc 15 §2.4 invader Quires from the shorter reported length and treats the party as Redacted only when both reports say so.
- **Visibility and hits.** PvP culling (§6.6 rule 2) applies in both directions; lag compensation (§8) applies to both sides.

---

## 10. Replays, daily verification and desync diagnostics

### 10.1 Replay files

Every party client writes `replays/<run_id>.palr` at run end and on Suspend: a doc 02 §12.1 container of kind 3, zstd level 9, holding

```rust
pub struct ReplayFile {
    pub header: SessionHeader,                      // Canon §2.4
    pub run_id: u64, pub config: RunConfig, pub roster: Vec<(u8, u64 /* account_id */, String)>,
    pub cil: Vec<CilTick>,                          // tick 0 to the last tick, exactly §6.14 content
    pub checkpoints: Vec<(u64, [u8; 32])>,          // HashReport boundaries: every 3,600, plus each Folio entry
    pub folio_states: Vec<(u32, Vec<u8>)>,          // §6.13 FolioState per Folio, for seeking
    pub end: Option<RunEndRecord>,                  // doc 14 §4.4
}
```

`input_log_hash` (doc 15 §10.4) is BLAKE3 of the postcard encoding of `cil`, so it does not depend on how the CIL was batched on the wire. The budget is 2 MB per hour of 4-player play (the raw CIL averages 21 bytes per tick, 4.5 MB per hour before zstd). The viewer re-simulates locally with `EMIT_VIEW`, seeks by restoring the nearest Folio state, plays at 1/4× to 8×, and offers a free camera and each player's view.

### 10.2 Daily and ranked score verification

1. At `Commit`, the host requests a ranked token for the party (§11.6). The service computes the Daily or Weekly seed itself (Canon §3.6), checks and consumes each member's attempt (doc 14 §8.4) and signs the token.
2. At run end, the host uploads the replay and the doc 14 §9.4 submission; if no upload arrives within 600 s, any other member's client uploads its own copy. Duplicate submissions per `run_id` are ignored.
3. A `svc.verify` worker runs `palimpsest_server verify`: it checks the token, `sim_version` and `content_hash` against released builds, re-simulates the whole CIL from tick 0 (doc 14 `verify`), compares every checkpoint, the final `state_hash` and the score, and requires the BLAKE3 of the CIL to equal the `input_log_hash` of at least one run report from a different party member (doc 15 §10.4) when the party had 2 or more accounts.
4. Verdicts: accept, or reject with `svc.err.replay_mismatch`, `svc.err.token_invalid`, `svc.err.sim_version_unknown` or `svc.err.content_hash_unknown`. 95% of submissions are verified within 600 s. A Daily of 90,000 ticks costs 15 s of one core at 6,000 ticks per second.

### 10.3 Desync diagnostics

| Detector | Compares | On mismatch |
|---|---|---|
| Shadow client | its state hash at each `HashReport` boundary | doc 02 §6.7: `SectionHashRequest`, dump `desync_<b>_shadow.pald`, then `FullStateRequest`; the host serializes at its next hash boundary and streams `FullState` at ≤ 64 KiB per tick; the Shadow restores it and continues from the CIL; presentation never pauses |
| Every client at Folio entry | `blueprint_hash` and sections 4–7 | §6.13 chunk data; telemetry counter `gen_desync` |
| Warm migration | hashes along the catch-up | cold migration (§9.3) |
| Verifier | checkpoints | reject and keep the dump for 30 days |
| CI | golden replays, SyncTest, 4-player bot runs over NetSim (§16) | build fails |

With crash-report consent, a client uploads its dump to `svc.crash`; player names are replaced by slot numbers (doc 02 §17.4).

---

## 11. Backend services

### 11.1 Services and technology

| ID | Responsibility |
|---|---|
| `svc.auth` | sign-in through platform tickets, access and refresh tokens, identity linking |
| `svc.profile` | account profile (unlocks, Codex, Echo roster, settings references, Margin trophies), export and deletion |
| `svc.econ` | doc 15 economy (`econ-api`, `econ-db`, `econ-verify`, `econ-platform`, `econ-signer`) |
| `svc.lobby` | non-Steam lobbies, relay session setup, invasion tickets |
| `svc.relay` | UDP relays for `net.route.relay` |
| `svc.leaderboard` | ranked tokens, submissions, boards |
| `svc.verify` | headless replay re-simulation workers (`palimpsest_server verify`) |
| `svc.telemetry` | opt-in usage batches (doc 02 §17.4) |
| `svc.crash` | opt-in minidumps and desync dumps |
| `svc.cloud` | cross-platform cloud saves |
| `svc.workshop` | mod mirror for non-Steam platforms (post-1.0) |
| `svc.status` | build gates, maintenance notices |
| `svc.moderation` | chat and player reports, account blocks |

| Concern | Choice |
|---|---|
| Language and API | Rust; `axum` 0.8.9 on `tokio` 1.53.2; `rustls` 0.23.45; JSON over HTTPS; one binary `palimpsest_services` with a role flag per service (doc 02 `crate.services`) |
| Database | PostgreSQL 18 through `sqlx` 0.9.0; serializable isolation for `svc.econ` and `svc.leaderboard` |
| Jobs | a Postgres job table polled with `FOR UPDATE SKIP LOCKED` (verification, exports, deletions); no separate broker |
| Objects | S3-compatible object storage in `eu-central` with server-side encryption: replays, dumps, cloud saves, exports |
| Keys | KMS (doc 15 §4.6): receipt keys `rk_`, token keys `tk_`, ranked keys `lk_`, invasion keys `ik_`, relay keys; one key per calendar year, public keys embedded in the client |
| Hosting | containers in a managed Kubernetes cluster in `eu-central` across 2 zones; relays on UDP hosts in `eu-central`, `us-east`, `ap-southeast`; a CDN serves `/v1/econ/odds/*` and `/audit/*` |
| Observability | `tracing` logs and Prometheus metrics, EU-hosted; IP addresses truncated to /24 (IPv4) or /48 (IPv6) at ingest |

**API conventions.** Base path `/v1`. `Authorization: Bearer <access token>` on every endpoint except the public ones. Integers only; `u64` values as `"0x"` + 16 lowercase hex digits (doc 15 canonical form); times in Unix seconds. Every write carries `req_id` (a client-random `u64`); the service derives doc 15's `idem_key = SHA-256(canonical_json({source_kind, source_id: req_id, account_id}))`, so retries are safe. Errors: an HTTP status plus `{ "error": "<svc.err.* ID>", "retry_after_s": 0 }`. Rate limits per account: 10 reads per second (burst 30), 2 economy writes per second (burst 10); excess returns 429 with `svc.err.rate_limited`.

### 11.2 Accounts and authentication

- `POST /v1/auth/steam` `{ "ticket": "<hex of GetAuthTicketForWebApi("palimpsest-auth")>", "build": 1042 }` → `{ "account_id": "0x0123456789abcdef", "access_token": "pal1....", "expires_in": 3600, "refresh_token": "<64 hex>", "created": false, "region": "juris.region.eu", "minor": false }`. The service validates the ticket with the Steam Web API, creates the account on first sign-in, and applies doc 15 §8.1 region resolution.
- `POST /v1/auth/refresh` `{ "refresh_token": "…" }` → a new pair. Refresh tokens last 30 days and rotate on every use; reuse of a rotated token revokes the whole token family.
- `POST /v1/auth/link/begin` → `{ "code": "7KQ2-M9TX", "expires_in": 600 }`; `POST /v1/auth/link/complete` `{ "code": "7KQ2-M9TX", "ticket": "…", "platform": "psn" }` → `{ "account_id": "0x…" }`.
- Access token: `pal1.` ‖ base64url(canonical JSON `{"acct","exp","iat","kid","plat","scope"}`) ‖ `.` ‖ base64url(Ed25519 signature over `"PALIMPSEST/TOKEN/v1" ‖ 0x00 ‖ payload`).

### 11.3 Profiles and cloud saves

- `GET /v1/profile` → `{ "version": 37, "profile": { … } }`; `PUT /v1/profile/settings` with `If-Match: <version>` writes only client-owned settings. Unlocks, Codex, Echoes and balances are written only by `svc.econ` and `svc.profile` grant paths.
- `GET /v1/cloud/{key}` and `PUT /v1/cloud/{key}` with `If-Match` ETags, keys `profile` and `run` (doc 14 `run.sav`), at most 16 MiB per object and 64 MiB per account. A `PUT` with a stale ETag returns 409 `svc.err.cloud_conflict`; the client keeps the save with the higher `save_seq`, then the higher boundary tick (doc 14 §12.3). Steam builds also keep Steam Auto-Cloud (doc 02 §12.2); `svc.cloud` adds cross-platform saves.

### 11.4 Economy API (`svc.econ`)

Doc 15 owns every rule; these are the endpoints.

| Method and path | Request | Response |
|---|---|---|
| `GET /v1/econ/wallet` | none | balances, rolling caps, signed `wallet` receipt |
| `GET /v1/econ/banners` | none | active banner records (doc 15 §3.13), own pity and spark per family, open epoch commitments |
| `GET /v1/econ/odds/{banner_id}` (public) | none | doc 15 §3.11 odds table |
| `POST /v1/econ/inkwell/pull` | `req_id`, `banner`, `count` (1 or 10), `expect_cost` | results per nonce, wallet, pity after, receipts |
| `POST /v1/econ/fairness/client_seed` | `req_id`, `banner`, `client_seed_hex` | the revealed epoch (`server_seed`, `commitment`) and the new commitment |
| `GET /v1/econ/fairness/epochs/{epoch_id}` | none | `commitment`, state, `server_seed` once revealed |
| `POST /v1/econ/reliquary/open` | `req_id`, `count` | results, counters after, Keepsake records |
| `POST /v1/econ/archive/purchase` | `req_id`, `entry`, `expect_cost` | grant, wallet, receipt |
| `GET /v1/econ/codex` | none | track states, tier, XP, catch-up reserve |
| `POST /v1/econ/codex/redeem_quill` | `req_id`, `pass` | track unlocked, receipt |
| `POST /v1/econ/runs/report` | doc 15 §10.4 run report | grants, Codex XP, wallet, receipts |
| `POST /v1/econ/drafts/sync` | queued Sealed Drafts and run reports (doc 15 §4.5) | one resolution per draft |
| `GET /v1/econ/receipts?since_seq=N` | none | receipts with `seq > N` |
| `GET /v1/econ/history/pulls?banner=&cursor=` | none | pull records (doc 15 §3.12) |
| `GET /v1/econ/store/catalog` | none | SKUs with regional fiat prices (doc 15 §7) |
| `POST /v1/econ/store/purchase/init` | `req_id`, `sku`, `platform` | `order_id` and platform parameters |
| `POST /v1/econ/store/purchase/finalize` | `req_id`, `order_id` | grants and receipts after server-to-server verification |
| `POST /v1/econ/store/refund` | `req_id`, `order_id` | revocations and receipts (doc 15 §7.4) |
| `GET /audit/{YYYY-MM-DD}` (public) | none | signed daily Merkle root (doc 15 §4.4) |

Pull, request and response (doc 15 acceptance test 8: client seed `errata`, nonce 0, `u_tier` 344, `u_feat` 3861, rate-up Rubricated index 1):

```json
{ "req_id": "0x5f1e00000000002a", "banner": "banner.echo.p001", "count": 1, "expect_cost": 160 }
```

```json
{ "epoch": { "id": "0x0000000000000011", "commitment": "630dcd2966c4336691125448bbb25b4ff412a49c732db2c8abc1b8581bd710dd",
             "nonce_first": 0, "nonce_last": 0 },
  "results": [ { "nonce": 0, "tier": "banner.tier.rubricated", "entry": "echo.slot_cy_b", "featured": true,
                 "grant": { "kind": "echo", "item": "echo.slot_cy_b", "qty": 1 }, "vellum": 0 } ],
  "wallet": { "ink": 2240, "quire": 310, "vellum": 12, "reliquary": 1, "quill": 0 },
  "pity_after": { "family": "banner.family.echo", "top_since": 1, "mid_since": 0, "top_guarantee": false,
                  "mid_guarantee": false, "spark": 1 },
  "receipts": [ { "acct": "0x0123456789abcdef", "iat": 1791763200, "item": "echo.slot_cy_b", "kid": "rk_2026a",
                  "kind": "grant", "qty": 1, "rid": "0x00000000000003ea", "seq": 8, "src": "banner.echo.p001", "v": 1,
                  "sig": "<128 hex>" } ] }
```

Run report, request and response (the reference failed run of doc 15: 400 Ink, 60 Quires):

```json
{ "req_id": "0x5f1e00000000002b", "run_id": "0xdba5cd5ae5eebfe8", "slot": 0, "sim_version": 1,
  "content_hash": "0x9f00000000000000000000000000000000000000000000000000000000000001",
  "run_seed": "0xc6f2e12ddca712ab", "mode": "mode.standard", "grade": 2, "outcome": "run.end.erased",
  "score": 3000, "active_ticks": 216000, "party_size_max": 2,
  "folio_exits": [ { "folio": 1, "genre": "genre.cyberpunk" }, { "folio": 2, "genre": "genre.wuxia" } ],
  "editor_kills": 0, "kills": [ { "rank": "rank.standard", "party_size": 2, "count": 61 } ],
  "types_identified": 6, "crafts": 2, "revives": 1, "apocrypha": false,
  "input_log_hash": "0x1b00000000000000000000000000000000000000000000000000000000000004",
  "final_state_hash": "0x1a00000000000000000000000000000000000000000000000000000000000002" }
```

```json
{ "grants": [ { "asset": "INK", "qty": 400 }, { "asset": "QUIRE", "qty": 60 } ], "codex_xp": 20000,
  "wallet": { "ink": 2640, "quire": 370, "vellum": 12, "reliquary": 1, "quill": 0 },
  "caps": { "ink_play_24h_used": 400, "ink_play_24h_cap": 4000, "codex_xp_24h_used": 20000, "codex_xp_24h_cap": 200000 },
  "receipts": [], "sampled_for_verify": false }
```

`sampled_for_verify` is true when `fnv1a64(run_id) mod 100 < 5` or the report reaches 90% of a cap (doc 15 §10.4). Economy error IDs: `svc.err.insufficient_ink`, `svc.err.insufficient_vellum`, `svc.err.banner_ended`, `svc.err.banner_not_started`, `svc.err.cap_reached`, `svc.err.cost_changed` (when `expect_cost` differs), `svc.err.idem_conflict` (same `req_id`, different body), `svc.err.report_mismatch` (header or `final_state_hash` disagrees with the host's report), `svc.err.purchase_locked`, `svc.err.platform_verify_failed`.

### 11.5 Lobbies, relay and invasion tickets

- Steam sessions use Steam lobbies (§3.3); `svc.lobby` is not involved except for invasion tickets.
- Non-Steam sessions: `POST /v1/lobbies` `{ "req_id", "settings": LobbySettings, "region_rtt_ms": [31, 104, 212] }` → `{ "lobby_id", "relay": "eu-central", "host_token": "<base64 netcode token>" }`; `GET /v1/lobbies?mode=&grade=&cursor=` lists public lobbies; `POST /v1/lobbies/{id}/join` → `{ "relay", "token" }`; `GET /v1/lobbies/{id}/events` (WebSocket) carries membership changes and `MigrateAnnounce`.
- `POST /v1/invasions/ticket` `{ "req_id", "host_run_id": "0x…" }` → `{ "exp": 1791763260, "sig": "<128 hex>" }`, an Ed25519 signature by an `ik_` key over `"PALIMPSEST/INVADE/v1" ‖ 0x00 ‖ le64(invader account_id) ‖ le64(host_run_id) ‖ le64(exp)`, valid for 60 s.
- `POST /v1/invasions/outcome` `{ "req_id", "host_run_id": "0x…", "invader": "0x…", "start_tick": 41000, "end_tick": 53000, "reason": 3 }` from the host and from the invader → `{ "matched": true }`. Both reports must arrive within 900 s of each other; a report without its pair grants nothing. The example (12,000 ticks, Redacted) pays the invader 10 Quires under doc 15 §2.4.

### 11.6 Leaderboards and ranked tokens

- `POST /v1/ranked/tokens` `{ "req_id", "mode": "mode.daily", "period": "2026-10-11", "accounts": ["0x…", "0x…"], "sim_version": 1, "content_hash": "0x…" }` → `{ "run_seed": "0xc6f2e12ddca712ab", "token": "<base64>", "exp": 1791936000 }`. The token is canonical JSON of the request fields plus `run_seed` and `exp`, signed by an `lk_` key; it lives 48 hours. `period` is the UTC date for `mode.daily`, the ISO week for `mode.weekly_gauntlet`, and absent for other ranked modes. A drop-in joiner of a ranked run requests a token with the run's `run_id` and its own account only; for `mode.daily` this consumes that account's attempt. Refusals: `svc.err.attempt_used`, `svc.err.sim_version_unknown`, `svc.err.content_hash_unknown`, and `svc.err.forbidden` for a run with `mode.invasion_open`.
- `POST /v1/leaderboards/submissions` with the doc 14 §9.4 JSON → `{ "submission_id", "upload_url" }`; the client `PUT`s the replay to `upload_url` (valid 900 s); `GET /v1/leaderboards/submissions/{id}` → `{ "state": "queued" | "accepted" | "rejected", "error": null }`.
- `GET /v1/leaderboards/{board}?period=&party_size=&cursor=` → 50 entries per page with rank, party personas, score, `end_tick`, and a replay link for the top 100.

### 11.7 Telemetry and crash reports

Both are opt-in and keyed by the random install ID (doc 02 §17.4). `POST /v1/telemetry/batch` `{ "install_id": "<32 hex>", "build": 1042, "events": [ … ] }`, at most 64 KiB, every 300 s while enabled; a disabled client sends nothing and keeps nothing. Network counters in each session summary: median and 95th-percentile RTT, loss, corrections per minute, 99th-percentile correction distance, substituted commands, late packets, snapshot budget overruns, desyncs, migrations by mode. `POST /v1/crash` takes a minidump or desync dump of at most 8 MiB.

### 11.8 Workshop

Steam builds use Steam Workshop (doc 02 §13.4). For the first non-Steam platform, `svc.workshop` mirrors items whose authors opt in; each upload passes `pal validate`, a content-only check for console targets (no WASM), and manual review before listing. A session's mod set must match exactly (doc 02 `modset_hash`).

### 11.9 Status and versioning

`GET /v1/status` (public) → `{ "min_build": 1042, "sim_versions": [1], "maintenance": null, "notice_key": null }`. A client below `min_build` keeps every offline feature and refuses online ones with `svc.err.version_too_old`.

---

## 12. Anti-cheat boundaries

PALIMPSEST ships no kernel or client anti-cheat (doc 02 §14, Steam Deck Verified). Integrity rests on authority, re-simulation and bounded rewards.

| Boundary | Trusted | Defense |
|---|---|---|
| Client → host | nothing | commands sanitized (§5.8); `view_tick` clamped to measured lag (§5.5); intents validated by their owning document; rate limits and violation kicks |
| Host → clients | only what the client may know | instanced items, other inventories, unknown item properties, culled PvP players and out-of-interest entities never leave the host (§6.6, doc 10 §2.8) |
| Host's own session | the host, for its own party | a host that edits its process changes only its party's game; its ranked submission fails re-simulation, and its run reports earn at most doc 15's per-run and 24-hour caps |
| Ranked boards | nothing until verified | full re-simulation of the CIL from tick 0 and cross-checked input-log hashes (§10.2) |
| Economy | platform tickets, server ledger | doc 15 caps, idempotent grants, 5% re-simulation sampling, run reports matched against the host's `final_state_hash` |
| Invasions (PvP) | nothing | signed tickets, empty mod set, equal `content_hash`, PvP culling, lag-compensation cap of 16 ticks, reports to `svc.moderation`; statistical flags (aim-snap rate, reaction time) go to human review only |

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

A session never waits on a service: every service call happens in the Margin or at run end, on the net or worker thread, with a 10 s timeout.

---

## 14. Privacy and data retention

| Data | Purpose | Legal basis (GDPR Art. 6(1)) | Retention |
|---|---|---|---|
| Account ID, linked platform IDs, region, minor flag | sign-in, ownership, compliance | (b) contract | until deletion + 30 days |
| Ledger, receipts, audit log | economy and fairness proofs | (c) legal obligation, (b) | 5 years, pseudonymous (doc 15 §4.4); daily roots forever |
| Profile and cloud saves | progression | (b) | until deletion; accounts inactive 3 years are notified, then deleted 90 days later |
| Accepted ranked replays and entries | boards | (b) | while on a board; top-1,000 per board period forever, persona replaced by "Erased Erratum" after account deletion |
| Rejected or unranked submissions | anti-cheat | (f) legitimate interest | 30 days |
| Telemetry | balance and stability | (a) consent | raw 180 days (doc 02 §17.4), aggregates forever |
| Crash and desync dumps | stability | (a) consent | 90 days |
| Moderation reports with the last 50 chat lines | safety | (f) | 365 days |
| Service logs | security | (f) | 14 days, IPs truncated (§11.1) |
| Chat, voice, game traffic | none | — | never stored by any service |

`GET /v1/account/export` returns a JSON archive of every row above within 30 days. `DELETE /v1/account` starts a 30-day grace period, then removes identity links, profile, cloud saves and non-board replays, and pseudonymizes what retention requires. Minor-flagged accounts never send telemetry, whatever the setting. No advertising identifiers or third-party analytics exist anywhere.

---

## 15. Bandwidth budgets

Budgets count message bytes plus `DATAGRAM_OVERHEAD` (48 bytes) per datagram.

| Flow | Per client | Notes |
|---|---|---|
| Snapshots, host → client | 12,000 B/s (96 kbit/s) | 30 datagrams/s; 10,560 B/s payload; mean 352 B per snapshot; combat estimate 304 B (24 entity updates at 74 bits, 6 events, owner, HUD, header); calm 87 B |
| World edits | 1,500 B/s average | keyframes and Folio state are exempt and occur at Folio entry |
| CIL | 1,800 B/s | 10 messages/s of 132 B (6 ticks × 21 B + 6 B) |
| Control (owner, party, run, hashes, chat) | 700 B/s | |
| **Down total** | **16,000 B/s (128 kbit/s)** | listen host with 3 clients: 384 kbit/s up (doc 02 `perf.deck.net.host_kbps`); dedicated with 4: 512 kbit/s |
| Inputs, client → host | 6,480 B/s (51.8 kbit/s) | 60 packets/s of 60 B; host down from 3 clients 155.5 kbit/s |
| Voice while talking | 5,600 B/s | 50 frames/s of 64 B (60 B at 24 kbit/s plus a 4 B header) |
| Folio entry burst | ≤ 64 KiB per tick | `FolioState`, `Keyframe` (≈ 12.5 KB for 300 entities), `ChunkData` only on mismatch |

---

## 16. Network test plan

**NetSim.** A link conditioner inside `net.route.in_process` delays, drops, duplicates and caps datagrams per direction. Per datagram it draws, in order, from `Rng::from_seed64(mix64(scenario_seed ^ link_id))` (outside the simulation): 1 burst-state transition `roll_bp`, 1 loss `roll_bp`, 1 duplicate `roll_bp`, 1 jitter `range(J + 1)` ms: 4 draws always.

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

**Scripted cases:** host killed at a random tick (hot, warm and cold modes each resume with equal hashes); a client Dormant for 20 s then resuming at the next Folio entry; a drop-in at Folio 3; a trade under 8% loss; an invasion with PvP culling; a generation mismatch forced by a corrupted chunk. **Soak:** a dedicated server cycles 4-bot runs for 24 h with flat memory. **Live:** a weekly playtest over Steam Datagram Relay between `eu-central` and `us-east` and between `eu-central` and `ap-southeast`. **Services:** 10,000 concurrent simulated clients; `POST /v1/econ/inkwell/pull` 99th-percentile latency ≤ 300 ms; zero ledger invariant violations. **Fuzzing:** every decoder of §5.1, §6.8, §6.11, §6.12 and §6.14 (doc 02 §16 item 5).

---

## Exports

All 138 IDs below follow Canon §1.1, sit in `net.` and `svc.` (Canon §19, doc 16), and have no `fnv1a32` collision among themselves or with the 2,749 ID tokens of the other spec documents on disk (`fnv1a32("net.ch.snapshot") = 0xe092c335`, `fnv1a32("svc.econ") = 0x71dea0e0`).

| Prefix | IDs |
|---|---|
| `net.params.` | `default` |
| `net.topo.` | `solo`, `listen`, `dedicated` |
| `net.route.` | `sdr`, `direct`, `relay`, `in_process` |
| `net.ch.` | `control`, `input`, `snapshot`, `world`, `cil`, `voice` |
| `net.msg.` | `hello`, `welcome`, `reject`, `ping`, `pong`, `bye`, `input`, `intent`, `snapshot`, `keyframe`, `world_edits`, `folio_enter`, `folio_state`, `chunk_data`, `folio_ready`, `cil`, `hash_report`, `section_hash_request`, `section_hashes`, `full_state_request`, `full_state`, `owner_delta`, `party_state`, `run_state`, `knowledge_delta`, `chat`, `voice`, `join_request`, `join_ready`, `migrate_announce`, `kick_vote`, `lobby_settings`, `lobby_pick`, `lobby_ready`, `run_header` |
| `net.reject.` | `proto_mismatch`, `sim_version_mismatch`, `content_mismatch`, `auth_failed`, `wrong_password`, `banned`, `party_full`, `run_locked`, `protocol_violation`, `kicked`, `timeout` |
| `net.cstate.` | `disconnected`, `connecting`, `handshaking`, `lobby`, `awaiting_folio`, `loading`, `playing`, `stalled`, `migrating` |
| `net.peer.` | `pending`, `lobby`, `waiting`, `loading`, `in_game`, `lost` |
| `net.migrate.` | `hot`, `warm`, `cold` |
| `net.trade.` | `idle`, `proposed`, `open`, `locked`, `both_locked`, `executed` |
| `net.lobby.` | `private`, `friends`, `public` |
| `net.voice.` | `off`, `party`, `proximity` |
| `net.session.` | `slot_join`, `slot_dormant`, `slot_resume`, `slot_departed`, `host_changed`, `folio_restart` |
| `net.netsim.` | `lan`, `good`, `typical`, `poor`, `bad` |
| `svc.` | `auth`, `profile`, `econ`, `lobby`, `relay`, `leaderboard`, `verify`, `telemetry`, `crash`, `cloud`, `workshop`, `status`, `moderation` |
| `svc.err.` | `bad_request`, `unauthorized`, `forbidden`, `not_found`, `rate_limited`, `version_too_old`, `maintenance`, `internal`, `insufficient_ink`, `insufficient_vellum`, `banner_ended`, `banner_not_started`, `cap_reached`, `cost_changed`, `idem_conflict`, `report_mismatch`, `purchase_locked`, `platform_verify_failed`, `cloud_conflict`, `attempt_used`, `token_invalid`, `replay_mismatch`, `sim_version_unknown`, `content_hash_unknown` |

Non-ID interfaces: the `Transport` trait, the message table of §3.7 and every layout in §5.1, §6.8, §6.11, §6.12, §6.14; `MoverSnap`; `Player` (`comp.player` semantics); `SessionIntent`; the intent kinds of §5.7; `ReplayFile`; the HTTP API of §11; behavior of `st.downed` and `st.ghost` and the payloads of `ev.player_revived`, `ev.player_ghosted`, `ev.player_resurrected` (doc 03 §14.2).

## Interface assumptions

| Doc | Assumed to provide |
|---|---|
| 02 | §3.1's `Transport` trait replacing the sketch; the intent kinds of §5.7 in `IntentKind` (doc 02 already has 5 command slots and `InvaderJoin`, `InvaderLeave`); flags `flag.net.shadow` (int: 0 off, 1 auto = on with ≥ 6 physical cores and not handheld, 2 on; default 1), `flag.net.voice` (bool, true), `flag.net.crossplay` (bool, true), `flag.net.netsim` (int scenario index, dev builds only, 0 = off); container kind 3 holding `ReplayFile`; `palimpsest_server` subcommands `host`, `verify`, `replay` |
| 03 | the pure controller `step_mover` that `sys.movement` runs for Errata; system `sys.coop` (logic owner 16, period 1, phase 0) between `sys.damage_late` and `sys.needs`, running §9.4 and §9.5 and trade validation; `ReplFrame` extraction as a read-only pass; `ev.noise.listener_db` with 5 entries; `ev.item_dropped` and `ev.item_picked_up` payloads carrying `cause: u8` (0 normal, 1 trade) |
| 04 | `revive_bp` per Erratum (product of talent multipliers, 10000 default); the Restoration-100 revive calls §9.4 completion |
| 05 | §2.6 reading the already clamped `view_tick`; a client-callable cosmetic fire model (interval, recoil, spread from keyed `rng.combat`) |
| 10 | `ItemView` encoding; `KnowledgeDelta` contents |
| 11 | `DropItemFor` producing `Instanced(to_slot)`; a Keepsake flag on item instances; loot rolls skipping Dormant and ghost slots |
| 12 | `generate_folio` as a pure function returning `blueprint_hash`; the `net_always` flag on exit doors |
| 13 | `net_always` on Editor bosses during `BossFight` and on hunters; invader character templates and behavior under Canon §17.9 |
| 14 | slot-status effects of each `SessionIntent`; `SlotJoin` consumed inside `enter_folio`; `FolioRestart` counted in `restores`; invasion rewards and the unranked-board rule |
| 15 | endpoint semantics of §11.4; the `minor` flag; Keepsakes untradeable |
| 17 | stalled and migration overlays, stale-entity rendering, trade window, kick vote, voice and crossplay settings, friend codes, ping display |
| 18 | `loc.` keys for every `net.reject.*` and `svc.err.*` ID |

## Canon gaps

- **G1.** The contract fixes the header at canon-1.1; the canon on disk is canon-1.4, and this chapter was checked against it.
- **G2.** Canon §2.4 logs only commands. The CIL and replays also log intents and session intents with their applied ticks (doc 02 G4). Requested: add both to Canon §2.4.
- **G3.** Canon §17.9 says the party is Redacted when the invader "reduces the last living Erratum to 0 HP" without relating it to `st.downed`. Assumed: the last Erratum alive and not downed; downed Errata lose `st.downed` without `ev.player_revived` when the party respawns; ghosts during a Redaction are doc 14's. Canon §17.9 gives no reason codes for `ev.invasion_ended`; §9.13 defines 0 to 6.
- **G4.** Canon §22 forbids synthesized input. The §5.4 substitute contains no movement and no press edge, so it starts nothing; this chapter reads it as the absence of input. Requested: say so in Canon §11.8.
- **G5.** Canon §2.2 gives the snapshot period and phase without naming the boundary. Assumed: boundary `b = t + 1` for every even `t`, so snapshot boundaries are odd.
- **G6.** Canon §9 keeps 32 ticks of pose history; doc 05 caps rewind at 16. Added: a target whose `spawn_tick` is later than `t_r` cannot be hit by that rewound test.
- **G7.** Canon §2.4 puts `run_seed` in the lobby handshake, but no seed exists before `Commit`; `RunHeader` carries it then.
- **G8.** Doc 14 §3.10 says a dead Erratum's entity "leaves the world"; Canon §17.6 gives ghosts doors, pings and marks in the world. The canon governs; doc 14's text needs to read "leaves the physical world".
- **G9.** Doc 03 §5.19 rewinds Errata projectiles on their first tick only; doc 05 §2.6 integrates catch-up steps. This chapter follows doc 05 and asks doc 03 to align. Doc 03 gives downed crawl 800 mm/s and doc 05 600 mm/s; as owner of `st.downed` this chapter fixes 800 (doc 03, the controller's owner).
- **G10.** Canon §18 has no trade event. Assumed: each traded item emits `ev.item_dropped` and `ev.item_picked_up` with `cause = 1`.
- **G11.** Canon §14 says no currency can be traded or gifted; doc 11 §17 lets players pass Lucre piles by dropping them. This chapter's trade and `DropItemFor` refuse `cat.currency`; doc 11's Public Lucre drops conflict with the canon and need a ruling.
- **G12.** Canon §11.1 fixes a 30 s bleed-out and a 3 s revive. Added: 1,200 and 600 ticks for the second and later downs in a Folio, damage shortening (doc 05 §14.3), talent multipliers, and `revive_hp_bp = max(2000, 5000 − 150 × grade)`.
- **G13.** `flag.` belongs to doc 02; the flags this chapter needs are requested in Interface assumptions.
- **G14.** Doc 02 §15 budgets 384 kbit/s of host upstream and describes it as 96 kbit/s per remote peer (R4's snapshot figure). This chapter keeps the 384 kbit/s total for 3 peers and splits each peer's 128 kbit/s into 96 kbit/s of snapshots and 32 kbit/s of reliable channels; doc 02's wording needs that split.

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
10. **Record sizes.** A walking update (two 11-bit axes, rotation) is 74 bits; a full create is 334 bits.
11. **World edit.** The §6.11 edit encodes as `08 00 00 03`; a 5,000-edit tick splits into parts of 4,096 and 904 with the more-parts bit set on the first.
12. **Field cell.** The §6.12 cell encodes as `a2 08 07 00 02`; the same cell becoming empty in a delta page encodes `a2 08 00 00`.
13. **Keyframe rule.** With `acked = 1001`, a snapshot at boundary 1065 is encoded against 1001, and boundary 1067 triggers a `Keyframe`.
14. **Handles.** A handle removed in snapshot 2001 is not reallocated before the client acknowledges 2001, then is the lowest free handle.
15. **Relevance.** An `Instanced(2)` item 1 m from slot 1's Erratum never appears in slot 1's snapshots; an Erratum 200 m away always does; an entity entering chunk distance 3 stays relevant and leaves at distance 4.
16. **PvP culling.** An invader 20 m away behind a wall of `full` blocks, unseen for 31 ticks, is removed from every Erratum client's view; at 15 m it stays relevant through the wall.
17. **Bleed.** Successive downs in one Folio start at 1800, 1200, 600, 600 ticks. With Max HP 85,000 a 17,000 mHP hit removes 360 ticks and a 20,000 mHP hit removes 424.
18. **Revive.** Default revive takes 180 ticks; with `revive_bp` 5000, 90; with 1000, 30. Releasing `INTERACT` at progress 179 resets progress to 0 and bleeding resumes on that tick.
19. **Revive HP.** At Max HP 85,000: Grade 2 gives 39,950 mHP, Grade 20 gives 17,000; a target with `st.withering` still gets 39,950 at Grade 2.
20. **Ghost loss.** A bare resurrection with 0 eligible items draws nothing from `rng.loot`; with 3 eligible items it draws exactly one `range(3)`.
21. **Trade.** A Lucre pile, a Keepsake, a quest item and an equipped cursed item are each refused; a trade breaks when origins reach 6,001 mm; an executed trade moves uids in ascending order and leaves `finder_slot` unchanged.
22. **Migration mode.** With a Shadow: hot. Without, 18,000 ticks at 500 ticks/s (36 s): warm. At 160 ticks/s (112.5 s): cold, and a ranked run's `restores` increases by 1.
23. **Successors.** Slot 1 (handheld, no Shadow, 40 ms), slot 2 (Shadow in sync, 90 ms), slot 3 (desktop, no Shadow, 30 ms) rank as 2, 3, 1.
24. **Migration continuity.** In NetSim `typical`, killing the host at a random tick resumes warm play with the successor's state hash at the resume boundary equal to an uninterrupted replay of the truncated CIL.
25. **Handshake.** A client with a different mod set is refused with `net.reject.content_mismatch`; a fifth party member with `net.reject.party_full`; a client failing both proto and content checks gets `net.reject.proto_mismatch`.
26. **Identity.** Account `0x0123456789abcdef` has friend code `WKNV-A49C`; the §3.8 inputs give `run_id = 0xdba5cd5ae5eebfe8`.
27. **Replay verification.** The verifier accepts an untouched 4-player replay and rejects it with `svc.err.replay_mismatch` after one changed yaw at tick 5,000.
28. **Ranked token.** A second token request for the same account and UTC date returns `svc.err.attempt_used`.
29. **Idempotency.** Repeating a pull with the same `req_id` and body returns the first response and posts one journal transaction; the same `req_id` with a different body returns `svc.err.idem_conflict`.
30. **Run report.** The §11.4 example grants 400 Ink and 60 Quires.
31. **Invasion ticket.** A ticket verifies over the 20-byte domain, separator and three little-endian `u64`s; it is refused 61 s after issue and refused for a different `host_run_id`.
32. **NetSim determinism.** The same scenario seed delivers the identical datagram sequence on two runs, with exactly 4 draws per datagram.
33. **Late targets.** A target spawned at tick 4990 is not hit by a rewound ray applied at `t = 5000` with `t_r = 4984`.
34. **Instanced Lucre.** A kill in a 3-player party with one Dormant slot produces Lucre piles for the 2 Active slots only, each sent only to its owner.
35. **Telemetry.** With telemetry at its default, and for any minor-flagged account, a full online session makes zero requests to `/v1/telemetry/batch`.
36. **Budgets.** In NetSim `typical`, no snapshot payload exceeds 1,150 bytes and no 600-tick window exceeds 16,000 B/s per client.
37. **Invasion window.** With `mode.invasion_open`: Folio 2 at entry + 9,000 ticks is closed; Folio 3 at entry + 7,199 is closed and at entry + 7,200 is open; a boss Folio is closed; the tick after any Erratum enters `room.colophon` is closed; after one invasion has started in a Folio, that Folio stays closed. A Daily or ranked lobby refuses `mode.invasion_open`.
38. **Invasion end.** An invader present for 18,000 ticks receives `InvaderLeave { reason: 2 }` on that tick and `ev.invasion_ended` is emitted; outcome reports of 12,000 ticks with reason 3 from both host and invader pay 10 Quires (doc 15 §2.4), and a lone report pays nothing.
39. **Redaction and downed.** With two Errata downed and the third alive, an invader hit that brings the third to 0 HP triggers the Redaction instead of a run end, and both downed Errata lose `st.downed` without `ev.player_revived`.

## Invariants annex

```
Σ 16: authority ⊗ thin clients ⊗ CIL ⊗ snapshots ⊗ prediction ⊗ lag-comp ⊗ co-op ⊗ migration ⊗ services ⊗ privacy
W! one authoritative Sim per session                       N! client state → host sim, except commands and intents
W! applied command logged (real | substitute)              N! substitute with movement | press edge
W! view_tick ∈ [t − min(16, lag + 4), t]                    N! rewind of blocks | target state | NPC attacks
W! snapshot boundaries b = t + 1, t even                   W! delta vs acked baseline | keyframe when b − acked > 64
W! snapshot payload ≤ 1150 B ∧ client down ≤ 16000 B/s     N! Instanced(s) item → client ≠ s
W! predicted mover == host mover (exact) or corrected      N! extrapolation of remote entities
W! blocks global ∧ reliable ordered                        W! fields | entities interest-managed
W! every client holds CIL from tick 0 ∧ Folio-entry state  W! blueprint_hash checked at every Folio entry
W! drop-in at Folio entry (invader: only exception)        N! invader ∈ {voters, successors, n}
W! migration ∈ {hot, warm, cold}; cold ⇒ restores += 1     N! migration that skips hash checks
W! downed, revive, trade draw 0 random numbers             W! ghost item loss: keyed rng.loot, ≤ 1 draw
N! currency in trades                                      W! trade atomic, ascending uid
W! ranked ⇒ server re-simulation from tick 0               N! ranked accept without matching hashes
W! service writes idempotent by req_id                     N! telemetry without consent | from minors
N! chat | voice | game traffic stored by services          N! IP address in logs beyond truncation
W! offline: every gameplay system runs                     N! session waits on a service call
W! invasion ⇒ mode.invasion_open ∧ Canon §17.9 window       N! invasion in Daily | Weekly | ranked runs
```
