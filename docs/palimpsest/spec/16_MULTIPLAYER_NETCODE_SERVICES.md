# 16 — Multiplayer, Netcode and Online Services

As of 2026-10-11. Status: BINDING. Conforms to canon-1.1.

This chapter defines how 1 to 4 players share one authoritative simulation: the three topologies, transports and NAT traversal, the wire protocol with exact packet layouts, snapshot replication with delta compression, quantization, interest management and priorities, voxel and field replication, client prediction and reconciliation, lag compensation, the co-op rules this chapter owns (`st.downed`, `st.ghost`, revive, trading, host migration, drop-in), replays, daily-score verification, desync diagnostics, and every backend service with its API. The header line is fixed by the document contract; the canon on disk is canon-1.3 and every rule below was checked against canon-1.3 (Canon gaps G1).

## Imports

| Source | Section | What this chapter consumes |
|---|---|---|
| DESIGN_BRIEF | §5, §8 | host-authoritative co-op with prediction, reconciliation and lag-compensated hits; deterministic input logs for replays and daily verification; offline-complete end-of-service mode |
| Canon | §1.1, §1.3, §1.6 | ID grammar and `id32`; `floor_div`, `ceil_div`, `mul_bp`; integer-only content, `u64` as hex strings |
| Canon | §1.8, §1.9, §1.11 | Folio box 256 × 64 × 256, chunk slot math; `Bam`, pitch clamp ±16000, `ang_diff`; `EntityId` layout, ascending-ID iteration |
| Canon | §2.1–§2.5 | 60 Hz tick, no extrapolation except local prediction; tick schedule row "Network snapshot, period 2, phase 0"; session header; replay command record with `view_tick`; save contract; `state_hash` every 60 ticks; co-op saves only at Folio entry |
| Canon | §3.2, §3.3, §3.6 | sequential and keyed streams (keyed draws make late inputs safe); seed display; daily and weekly seeds |
| Canon | §4.1, §4.2 | humanoid capsule, speeds, dodge |
| Canon | §9 | baked pose tables; 32 ticks of pose history for lag compensation |
| Canon | §10.8, §11.1 | lethal resolution steps 2–3; `st.downed` (owner 16: 30 s bleed-out, revive in 3 s), `st.ghost` (owner 16) |
| Canon | §13.6, §13.9, §14 | party-shared type knowledge; `finder_slot`; no currency can be traded or gifted |
| Canon | §17.1, §17.2, §17.5, §17.6 | `PARTY_MAX` 4, loot instanced per player; party-size tables; `FF_DIRECT_BP`, `FF_AREA_BP`, `mode.ff_off`, `mode.ff_full`, `mode.loot_shared`; ghosts and resurrection |
| Canon | §18, §19 | `ev.player_downed`, `ev.player_revived`, `ev.player_died`, `ev.player_ghosted`, `ev.player_resurrected`, `ev.run_ended`, every event replicated for presentation; namespaces `net.` and `svc.` belong to this chapter |
| Doc 02 | §2.2, §3.2–§3.4 | crates `renet` 2.0.0, `steamworks` 0.13.1, `postcard` 1.1.3, `zstd`, `blake3`, `ed25519-dalek` 3.0.0, `axum` 0.8.9, `tokio` 1.53.2, `sqlx` 0.9.0, PostgreSQL 18; `crate.net`, `crate.server`, `crate.services`; the `Transport` sketch |
| Doc 02 | §5.4, §5.5 | `Sim::step`, `snapshot`, `restore`, `serialize`, `section_hashes`; `PlayerCommand`, button bits, `PlayerIntent`, `IntentKind` |
| Doc 02 | §6.5–§6.7, §12.1, §13.4, §15, §17.3, §17.4 | 12 serialization sections and section hashes; golden replays; desync dump; `PALS` container kinds; `modset_hash`; `perf.deck.net.host_kbps` = 384 kbit/s for 3 clients; flags `flag.net.transport`, `flag.net.input_redundancy`; telemetry consent rules |
| Doc 03 | §2.2, §3.2–§3.4, §5, §5.16, §5.19, §13, §14.2, §15.1, §16 | system order (`sys.input_apply`, `sys.hitscan`, `sys.melee`, `sys.tick_end`); entity kinds, `comp.player` (semantics owned here), `Kind.flags`, `Pose.hist`; the character controller; pose-history rewind; first-tick projectile rewind; `set_block` and `ev.block_changed`; event payloads; 5 × 5 chunk activation window; save coverage |
| Doc 04 | §6.2, Restoration and Warden talents | downed members and ghosts share kill XP; instant revive at Restoration 100; Warden `resurgence` revive time × 5000 |
| Doc 10 | §2.2, §2.5, §2.8 | party-shared type knowledge; knowledge travels with traded items; host sends only `ItemView` |
| Doc 14 | §1.1, §2.1, §2.5, §3.2, §3.6, §3.10, §4.3, §8.4, §8.8, §9.4, §12 | lobby state machine; joiner rules; exit commit rule and constants; ghost restoration HP; Dormant and Departed slots, `DORMANT_GRACE_S`; ranked Daily attempts; boards and `verify`; `RANKED_RESTORE_MAX` = 2; Folio-entry save copies for host migration |
| Doc 15 | §2.3, §4, §8.5, §10, §11 | caps; commit-reveal fairness, Sealed Drafts, signed receipts; chat safety; economy service components, ledger, idempotent grants, run reports; end-of-service mode |
| R4 | §6.1–§6.4 | snapshot model, input redundancy 8, 30 Hz snapshots, 96 kbit/s snapshot budget, SDR facts (IP privacy, P2P), renet for LAN, interpolation delay 100 ms default |
| R3 | A2, C4 | crossplay on by default, cross-progression; COMMS-1, LOG-1, EOL-1 requirements |
| R7 | §4, §11 R-CO3, R-CO4 | revive health falling with difficulty; instanced currency pickups |

---

## 1. Decisions and constants

### 1.1 Decisions

1. **One authority.** Exactly one process runs the authoritative `Sim` for a session: the host. Every other process sends `PlayerCommand`s and `PlayerIntent`s and receives replicated state.
2. **Thin clients, snapshot replication.** Clients render remote state from 30 Hz delta-compressed snapshots and predict only their own mover. They regenerate each Folio's terrain locally from the seed and receive block edits as an ordered diff log, so terrain never streams in bulk.
3. **Every client keeps the Confirmed Input Log (CIL)** of the inputs the host applied, plus the latest Folio-entry state. This gives every client a complete replay, makes any client able to become host, and feeds the daily verifier.
4. **Shadow replicas are optional.** A client with spare cores runs a full `Sim` replica that consumes the CIL (the Shadow). Shadows detect desyncs live and make host migration instant.
5. **Favor the shooter** for Errata attacks, inside a rewind window of 30 ticks.
6. **Platform P2P first.** Steam builds connect through Steam Datagram Relay, so no player IP address is revealed and no game servers are rented. The dedicated server binary exists for self-hosting and for headless verification, not for live matchmaking.
7. **Offline-complete.** Every gameplay feature, including co-op over LAN and direct IP, runs without the backend (DESIGN_BRIEF §8).

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
| `LAGCOMP_MAX_REWIND` | 30 ticks | hard cap; the pose history holds 32 (Canon §9) |
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

Every constant lives in the content record `net.params.default` (`content/16/net/params.json`), loaded with the content set and covered by `content_hash`:

```json
{ "id": "net.params.default", "proto": 1, "input_redundancy": 8, "input_buffer_span": 64,
  "slack_target_min": 2, "slack_target_max": 8, "interp_min": 4, "interp_max": 12,
  "pred_ring": 128, "pred_stall_ticks": 120, "lagcomp_max_rewind": 30, "lagcomp_slop": 4,
  "snap_ring": 32, "handle_bits": 13, "mtu_payload": 1200, "snap_payload_max": 1150,
  "datagram_overhead": 48, "client_down_budget": 16000, "snap_rate": 10560, "snap_bucket": 2112,
  "cil_batch_ticks": 6, "stall_ms": 1000, "timeout_ms": 5000, "migrate_announce_ms": 10000,
  "migrate_warm_max_ms": 45000, "default_port": 27310,
  "bleed_ticks": [1800, 1200, 600], "revive_base_ticks": 180, "revive_min_ticks": 30,
  "revive_range_mm": 1500, "revive_cone_bam": 8192, "bleed_damage_ticks_per_maxhp": 900,
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

One player's client is host and runs the solo arrangement plus up to 3 remote thin clients. The host player's own slot uses `LocalInputPort` and is never predicted, so the host plays at zero network latency. A listen-host session never pauses. The host's upstream budget is 384 kbit/s for 3 clients (doc 02 `perf.deck.net.host_kbps`).

### 2.4 Dedicated headless server (`net.topo.dedicated`)

`palimpsest_server host` (crate `crate.server`) runs `HostSession` with no local slot and no rendering, and accepts up to 4 remote clients. Lobby decisions that doc 14 gives to "the host" are made by the **party leader**: the configured admin account, else the first client to connect, passed on in slot order when the leader leaves.

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

- Lobbies use `ISteamMatchmaking` with 4 members; the host is the lobby owner. Lobby data keys, all ASCII: `pal_ver` (`SIM_VERSION` decimal), `pal_proto`, `pal_ch` (first 16 hex digits of `content_hash`), `pal_mods` (first 16 hex digits of `modset_hash`, doc 02 §13.4), `pal_mode`, `pal_grade`, `pal_state` (`lobby` or `run`), `pal_folio`, `pal_ff`, `pal_loot`, `pal_join` (`open` or `closed`), `pal_host` (SteamID64), `pal_succ` (comma-separated successor SteamID64s, §9.3), `pal_pub` (`net.lobby.*` suffix).
- Lobby types: `net.lobby.private` (invite only, `k_ELobbyTypePrivate`), `net.lobby.friends` (`k_ELobbyTypeFriendsOnly`), `net.lobby.public` (`k_ELobbyTypePublic`). Default: friends.
- **Quick Join** requests the lobby list filtered by equal `pal_ver`, `pal_proto`, `pal_ch`, `pal_mods`, `pal_mode`, `pal_join = open`, `|pal_grade − own max grade| <= 2`, and the default distance filter; it then estimates ping from each lobby's host ping location (`ISteamNetworkingUtils` ping locations exchanged as lobby member data `pal_ploc`) and joins the lowest estimate under 150 ms, else creates a public lobby.
- Identity on the wire is the SteamID64; the host authenticates each client with `BeginAuthSession` on a ticket from `GetAuthSessionTicket`, and refuses with `net.reject.auth_failed` on any result other than OK.

### 3.4 Direct and LAN

- The host listens on UDP 27310 (configurable). LAN discovery: the host broadcasts a 32-byte beacon `PALB ‖ proto u16 ‖ sim_version u32 ‖ content_hash[0..8] ‖ port u16 ‖ party u8 ‖ max u8 ‖ mode id32 ‖ 6 zero bytes` to UDP 27311 once per second; clients list beacons seen in the last 3 s.
- renet runs in unsecure mode on LAN and direct IP; the host password (optional lobby setting, default empty) is verified inside `Hello` as `BLAKE3("PALIMPSEST/PW/1" ‖ host_nonce ‖ password)`, where `host_nonce` is 16 bytes sent in the renet connection challenge.
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
}
pub struct Welcome {
    pub slot: Option<u8>,            // None while waiting to drop in (§9.1)
    pub host_tick: u64, pub run: Option<RunHeader>, pub party: PartyState, pub lobby: LobbySettings,
}
pub struct RunHeader { pub run_id: u64, pub run_seed: u64, pub sim_version: u32, pub content_hash: [u8; 32],
                       pub config: RunConfig /* doc 14 */, pub folio: u32, pub start_tick: u64 }
```

The host checks, in this order, and answers `Reject` with the first failing reason: `proto` (`net.reject.proto_mismatch`), `sim_version` (`net.reject.sim_version_mismatch`), `content_hash` and `modset_hash` (`net.reject.content_mismatch`, doc 02 test 35), platform authentication (`net.reject.auth_failed`), password (`net.reject.wrong_password`), host block list (`net.reject.banned`), seats (`net.reject.party_full`), ranked eligibility (`net.reject.run_locked`, §9.1). Canon §2.4's lobby handshake carries `run_seed`; before `Commit` no seed exists, so `Hello` carries none and `RunHeader` supplies it at `Commit` (Canon gaps G7).

`run_id = mix64(host_account_id ^ mix64(run_seed ^ mix64(commit_unix_ms)))`. Example: host `0x0123456789abcdef`, seed `0xC6F2E12DDCA712AB`, `commit_unix_ms = 1791763200123` give `run_id = 0xdba5cd5ae5eebfe8`.

### 3.9 Time synchronization

Each client sends `Ping` every 500 ms. The host answers at once with `Pong`, where `host_tick` is the next tick it will simulate and `host_tick_age_us` is the time since that tick's scheduled start. The client keeps `rtt_us` as an exponentially weighted mean (`rtt += (sample − rtt) / 8`, integer µs) and `host_tick_est(now) = host_tick + (rtt/2 + host_tick_age_us + elapsed_us) / 16667`. Snapshots refine the estimate (§5.6). `rtt_ticks = ceil_div(rtt_us, 16667)`.
