# 02 — Technical Architecture

As of 2026-10-11. Status: BINDING. Conforms to canon-1.1.

This chapter fixes the technology stack, the Cargo workspace, the deterministic simulation architecture, the fixed-point math library, the determinism plan, the simulation-to-presentation bridge, rendering, audio, input, UI, localization, the content pipeline, saves, modding, platforms, performance budgets, testing, CI/CD, telemetry and the coding rules an implementing agent follows. The canon file on disk at writing time is canon-1.2; every rule below was also checked against it (Canon gaps G1).

## Imports

| Source | Section | Consumed |
|---|---|---|
| DESIGN_BRIEF | §5 | Rust, Bevy, Bevy-free `palimpsest_sim`, fixed point, 60 Hz tick, 32³ chunks, light and sound grids, sim-side capsules, host-authoritative co-op, data-driven content, WASM mods, optional on-device language model |
| DESIGN_BRIEF | §8 | jurisdiction flag layer, offline-complete end-of-service mode |
| Canon | §1.1 | content ID grammar, `id32 = fnv1a32`, collision rules, aliases |
| Canon | §1.2–§1.4 | numeric types, `floor_div`, `mul_bp`, `Fx` primitives, `dist2`, `sqrt_fx`; delegation of `isqrt_u128` to this chapter |
| Canon | §1.6 | integer-only content, `"0x"` hex strings for `u64` |
| Canon | §1.8–§1.9 | coordinates, chunk index, `Bam`, sine table `T`, `sin_bam`, `cos_bam`, `fwd`, `ang_diff`; delegation of `atan2_bam` to this chapter |
| Canon | §1.10 | text keys `loc.*` |
| Canon | §1.11 | `EntityId` layout, ascending-ID iteration, `HashMap`/`HashSet` ban |
| Canon | §2.1–§2.5 | `TICK_HZ`, `DT`, semi-implicit Euler, tick schedule, session header, replay command record, save contract, `state_hash = BLAKE3(serialization)`, 60-tick broadcast |
| Canon | §3 | xoshiro256++, SplitMix64, `stream_seed`, `keyed_rng`, stream table, mod streams |
| Canon | §4 | `ACTIVE_ACTOR_CAP`, `PROJECTILE_CAP`, `GROUND_ITEM_CAP`, body dimensions |
| Canon | §9 | baked fixed-point hitbox pose tables, 32-tick pose history |
| Canon-1.2 | §1.2 `Light` row, §2.3 `light_at` | two light channels `light_sky`, `light_block`; effective light = max |
| Canon | §18 | event IDs |
| Canon | §19 | namespaces `crate.`, `flag.`, `perf.` owned by this chapter |
| Canon | §21 | acceptance tests 1–12, 29, 31–34 |
| R4 | §0–§6, §10–§16 | versions read from crates.io on 2026-10-11, meshing, light texture idea, netcode parameters, audio split, modding, test pyramid |
| R6 | GPV blocks D–F | presentation fields: `render_style`, `post`, `grade`, `anim_step`, `transition`, music layers |
| Docs 03, 09, 12–19 | — | not yet written; every needed interface is listed under Interface assumptions |

## 1. Architectural decisions

1. **One authority.** `palimpsest_sim` owns all game state. Every other crate reads simulation output and sends inputs. Nothing writes simulation state except `Sim::step`, `Sim::restore` and `Sim::deserialize`.
2. **The simulation is integer-only, single-threaded and `#![no_std]` with `alloc`.** `no_std` removes `std::time`, `std::thread`, `std::fs`, `std::env` and `std::collections::HashMap` by construction. Parallelism exists only outside the simulation, or inside it as pure maps merged in index order (§5.6).
3. **Floats begin after one boundary.** The only code that converts simulation values to `f32` is the module `palimpsest_bridge::conv` (§7.5). `palimpsest_sim` contains no float exception at all, which is stricter than Canon §21 test 33 requires.
4. **Content is data.** All content is JSON validated by JSON Schema 2020-12 (§11). The brief allows RON; the canon chooses integer-only JSON (Canon §1.6), and this chapter follows the canon.
5. **Pin everything.** Exact versions, a committed `Cargo.lock`, and `=` pins for every Bevy-family crate.
6. **Headless first.** Every crate except the Bevy plugin crates builds and tests without a window, GPU or audio device.
7. **Offline-complete.** Every gameplay system runs without the backend (brief §8). Network services are optional enhancements.

## 2. Technology stack

### 2.1 Toolchain

| Item | Value | Justification |
|---|---|---|
| Rust | `1.99.0` stable, pinned in `rust-toolchain.toml` | Current stable on 2026-10-11 (R4 §0). Above Bevy 0.20's MSRV 1.97.1 and wasmtime 49's MSRV 1.96.0. |
| Edition | 2024 | Current edition; resolver 3 by default. |
| Components | `rustfmt`, `clippy`, `llvm-tools` | Formatting, linting, coverage. |
| Targets | `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu`, `x86_64-pc-windows-msvc`, `aarch64-apple-darwin`, `wasm32-unknown-unknown`, `wasm32-wasip1` | Shipping platforms, the web demo, and the WASI determinism runner (§6.4). |
| Linux baseline | Steam Linux Runtime 3.0 ("sniper") container | Valve's supported runtime; glibc baseline fixed by Valve. |

### 2.2 Crates

Every version below was read from crates.io on 2026-10-11 (R4 §16 plus a re-check for crates R4 does not list). "Pending" means the latest release targets Bevy 0.19; §2.4 decides.

| Crate | Version | Used in | Purpose and justification |
|---|---|---|---|
| `bevy` | `=0.20.0` | client crates | Engine for rendering, input, UI, assets. 0.20 adds schedule randomization, panic-to-error and mesh-shader groundwork (R4 §16.3). |
| `bevy_egui` | `=0.43.1` | `palimpsest_ui` and client, `dev_tools` feature only | Immediate-mode debug UI; never compiled into release builds. |
| `bevy_brp_mcp`, `bevy_brp_extras` | `=0.23.0` | client `dev_tools` | Lets a coding agent drive, query and screenshot the running client. |
| `bevy_seedling` + `firewheel` | first release requiring Bevy 0.20 (pending; latest 0.8.0 / 0.14.0) | `palimpsest_audio` | Audio graph with web backend (`firewheel-web-audio`) and HRTF node support. |
| `bevy_kira_audio` | `=0.27.0` | fallback for `palimpsest_audio` | Ready for Bevy 0.20. |
| `audionimbus` | `0.16.0` (`auto-install`) | `palimpsest_audio` | Steam Audio 4.8.1 binaural rendering and reflections, Apache-2.0. |
| `avian3d` | first release requiring Bevy 0.20 (pending; latest 0.7.0) | `palimpsest_render` | Cosmetic physics only: ragdolls, gibs, debris. ECS-native. |
| `bevy_rapier3d` | `=0.37.0` | fallback for cosmetic physics | Ready for Bevy 0.20. |
| `bevy_hanabi` | first release requiring Bevy 0.20 (pending; latest 0.19.0) | `palimpsest_render` | GPU particles. Fallback: in-house CPU billboard particles (§8.9). |
| `ogg` + `opus-decoder` | `0.9.2` + `0.1.1` | `palimpsest_audio` | Pure-Rust Ogg/Opus decoding; Symphonia 0.6.1 has no Opus codec. Gated by RFC 8251 conformance vectors in CI. |
| `fluent-bundle`, `unic-langid`, `fluent-langneg` | `0.16.0`, `0.9.6`, `0.14.2` | `palimpsest_ui` | Fluent localization with plural and gender selectors. |
| `arc-swap` | `1.9.2` | `palimpsest_bridge` | Lock-free publication of view frames. |
| `steamworks` | `0.13.1` | `palimpsest_platform` | Lobbies, Steam Datagram Relay, achievements, Cloud, Workshop, Input glyphs. Own thin Bevy glue instead of waiting for `bevy-steamworks`. |
| `crash-handler` + `minidumper` | `0.8.1` + `0.11.0` | client, server | Out-of-process minidumps without a third-party SDK. |
| `llama-cpp-2` | `0.1.159` | `palimpsest_scribe` | Optional on-device flavor text (brief §5), grammar-constrained JSON. |
| `serde`, `serde_json` | `1.0.229`, `1.0.154` | data, tools | Serialization; JSON content. |
| `postcard` | `1.1.3` | sim, net, saves | Documented stable wire format; `no_std`. bincode is unmaintained (R4 §12). |
| `zstd` | `0.14.2` | saves, replays | Save and replay compression. |
| `blake3` | `1.8.7` | math, data, saves | `state_hash`, `content_hash`, checksums (Canon §2.4–§2.5). `no_std` capable. |
| `schemars` | `1.2.2` | data, tools | JSON Schema 2020-12 generated from Rust types. |
| `jsonschema` | `0.58.6` | data loader, tools | Validation of every content file at build and load. |
| `thiserror` | `2.0.21` | all library crates | Typed errors. |
| `smallvec`, `bitflags` | `1.16.3`, `2.13.2` | sim, data | Inline small lists; button masks. |
| `tracing`, `tracing-subscriber`, `tracing-tracy`, `tracy-client` | `0.1.44`, `0.3.23`, `0.12.0`, `0.19.0` | all binaries | Logging and frame profiling. |
| `rayon` | `1.12.0` | tools, client asset baking | Ordered parallel maps outside the simulation. |
| `notify`, `notify-debouncer-full` | `8.2.0`, `0.7.0` | data loader `hot_reload` feature | Content hot reload in dev builds. |
| `crossbeam-channel` | `0.5.17` | net tests, bridge | In-process transport and job queues. |
| `renet` | `2.0.0` | net `transport_udp` feature | LAN, dev and CI transport. |
| `wasmtime` | `=49.0.2` (`component-model`, `cranelift`, `pulley`; no WASI) | `palimpsest_mod_api` host | Deterministic mod execution with fuel and NaN canonicalization. |
| `wit-bindgen` | `0.62.0` | mod guest SDK | Guest bindings for the WIT world (§13.3). |
| `dot_vox` | `5.2.0` | tools | MagicaVoxel import for prefab rooms and props. |
| `clap`, `anyhow` | `4.6.7`, `1.0.104` | tools, server, services binaries only | CLI parsing; binary-level error reporting. |
| `directories`, `tempfile`, `fs-err` | `6.0.0`, `3.27.0`, `3.3.2` | platform, saves | Save paths, atomic writes, readable IO errors. |
| `ureq` + `rustls` | `3.4.2` + `0.23.45` | platform | Blocking HTTPS for telemetry and crash upload on a worker thread. |
| `ed25519-dalek` | `3.0.0` | platform, services | Offline entitlement receipts (brief §8). |
| `sha2`, `hmac` | `0.11.0`, `0.13.0` | services | Commit-reveal for server-rolled randomness (doc 15). |
| `axum`, `tokio`, `sqlx` | `0.8.9`, `1.53.2`, `0.9.0` | services | Backend HTTP, async runtime, PostgreSQL 18 access. |
| `proptest`, `insta`, `criterion`, `arbitrary`, `libfuzzer-sys` | `1.12.0`, `1.49.0`, `0.8.2`, `1.5.0`, `0.4.13` | dev-dependencies | Property, snapshot, benchmark and fuzz tests. |
| `binary-greedy-meshing` | `0.5.2` | render dev-dependency | Test oracle for the in-house mesher's face counts (§8.2). |
| CLI tools | `cargo-nextest 0.9.148`, `cargo-mutants 27.1.0`, `cargo-fuzz 0.13.2`, `cargo-deny 0.20.2`, `cargo-llvm-cov 0.9.1` | CI | Test runner, mutation testing, fuzzing, dependency policy, coverage. |

### 2.3 Where R4 is overridden

| R4 recommendation | Decision | Reason |
|---|---|---|
| `fixed` crate `I48F16` | in-house `Fx(i64)` Q32.32 | Canon §1.2, §1.4 |
| own PCG32 | xoshiro256++ seeded by SplitMix64 | Canon §3.1 |
| RON content | JSON + JSON Schema 2020-12 | Canon §1.6 |
| `xxh3_64` state hash | BLAKE3 over the canonical serialization | Canon §2.5 |
| `slotmap` generational arena | sorted columns keyed by canon `EntityId` | Canon §1.11 IDs are never reused, so generations add nothing (§5.2) |
| `pathfinding` crate | in-house A* in `palimpsest_ai` | Exact tie-break by lower cell index is part of determinism |
| 4,096-entry quarter-wave LUT | 16,385-entry table `T` | Canon §1.9 |

### 2.4 Pending-crate rule

Bevy stays at `=0.20.0` for the whole 0.20 cycle. When the milestone that first needs a pending crate starts (§18.5), the primary crate is used if it has a release whose `bevy` requirement is `^0.20`; otherwise the listed fallback is used for the rest of the 0.20 cycle. Migration to Bevy 0.21 is a dedicated task that touches only `palimpsest_render`, `palimpsest_audio`, `palimpsest_ui`, `palimpsest_client` and `palimpsest_platform`.

### 2.5 Bevy feature set

`bevy` is declared with `default-features = false` and these features: `3d`, `ui`, `bevy_gltf`, `ktx2`, `zstd_rust`, `png`, `tonemapping_luts`, `smaa_luts`, `pbr_clustered_decals`, `multi_threaded`, `keyboard`, `mouse`, `gamepad`, `x11`, `wayland`. Dev builds add `file_watcher`, `bevy_remote`, `hotpatching`. Profiling builds add `trace_tracy`. CI builds add `debug` (for `ScheduleBuildSettings::shuffle_seed`) and `bevy_ci_testing`. The web demo adds `webgpu`. Ultra builds for Windows and Linux add `bevy_solari` and `dlss`. `bevy_audio` is never enabled because `bevy_seedling` replaces it.

## 3. Cargo workspace

### 3.1 Layout

```
palimpsest/
  Cargo.toml              workspace, [workspace.dependencies] with exact pins
  Cargo.lock              committed
  rust-toolchain.toml     channel = "1.99.0"
  clippy.toml             workspace defaults (§6.2)
  deny.toml               cargo-deny bans and licenses
  AGENTS.md               build, test and invariant summary for coding agents (§18)
  crates/<crate>/         one directory per crate in §3.2, each with its own AGENTS.md
  content/<doc>/<ns>/     JSON content, grouped by owning spec document (§11.1)
  schemas/                generated JSON Schema 2020-12 files, committed
  assets/                 glTF, KTX2, Ogg/Opus, fonts, Fluent files (§11.7)
  tests/golden/           golden replays *.palr (§6.6)
  tools/gen_trig.py       generator of the sine table (Canon §1.9)
  xtask/                  workspace lints and automation
```

### 3.2 Crates

Layer numbers define the dependency DAG. A crate may depend only on crates in its allow list, all of which sit in lower layers.

| ID | Package | Layer | Kind | `no_std` | Responsibility |
|---|---|---|---|---|---|
| `crate.math` | `palimpsest_math` | 0 | lib | yes | integer primitives, `Fx`, `Fx3`, `Bam`, trig, CORDIC, `isqrt_u128`, geometry, RNG, FNV-1a, BLAKE3 wrapper |
| `crate.data` | `palimpsest_data` | 1 | lib | yes (core); `loader` feature uses std | content record types, `ContentDb`, ID registry, schema generation, loader, `content_hash` |
| `crate.gen` | `palimpsest_gen` | 2 | lib | yes | Folio generation as a pure function (doc 12) |
| `crate.ai` | `palimpsest_ai` | 2 | lib | yes | utility scoring, response curves, A* and HPA* (doc 13) |
| `crate.sim` | `palimpsest_sim` | 3 | lib | yes | world store, scheduler, every simulation system, Sim API, canonical serialization, view extraction |
| `crate.net` | `palimpsest_net` | 4 | lib | no | `Transport` trait, protocol framing, input buffers, prediction and reconciliation (doc 16), desync detection (§6.7) |
| `crate.bridge` | `palimpsest_bridge` | 4 | lib | no | view-frame mailbox, interpolation, entity mapping data, event queue, the float conversion boundary `conv` |
| `crate.mod_api` | `palimpsest_mod_api` | 4 | lib | no | WIT world, wasmtime host (`host` feature) implementing `ScriptHost`, guest SDK (`guest` feature) |
| `crate.scribe` | `palimpsest_scribe` | 4 | lib | no | optional on-device flavor-text jobs, template fallback; never linked into the simulation |
| `crate.render` | `palimpsest_render` | 5 | lib | no | Bevy plugin: voxel meshing, materials, lighting, viewmodel, post, particles, decals, transitions |
| `crate.audio` | `palimpsest_audio` | 5 | lib | no | Bevy plugin: voices, spatialization, occlusion, music stems |
| `crate.ui` | `palimpsest_ui` | 5 | lib | no | Bevy plugin: input action layer, HUD, menus, widgets, localization, dev overlay |
| `crate.platform` | `palimpsest_platform` | 5 | lib | no | `Platform` trait, Steam and Null backends, save IO, telemetry, crash upload, receipts |
| `crate.bots` | `palimpsest_bots` | 5 | lib + bin | no | headless bot players, batch runner, balance Monte Carlo harness |
| `crate.server` | `palimpsest_server` | 5 | lib + bin | no | headless dedicated host, replay runner, daily-score verifier |
| `crate.client` | `palimpsest_client` | 6 | bin | no | the game executable: assembles plugins, owns the sim thread |
| `crate.services` | `palimpsest_services` | 6 | bin | no | backend: economy endpoints (doc 15), daily verification queue, telemetry and crash ingest |
| `crate.tools` | `palimpsest_tools` | 6 | bin `pal` | no | validate, bake, schema export, golden bless, desync diff, generator sweeps, trig check |
| `crate.xtask` | `xtask` | 6 | bin | no | dependency DAG check, float scan, size limits, SPEC reference check |

### 3.3 Dependency rules

| Crate | Allowed workspace dependencies | Allowed external dependencies | Forbidden anywhere in its dependency closure |
|---|---|---|---|
| math | none | `blake3` (no default features) | everything else |
| data | math | `serde`, `postcard`, `blake3`, `smallvec`, `schemars` (feature `schema`), `serde_json` + `jsonschema` + `notify` (feature `loader`) | `bevy*`, `glam`, `wgpu*`, `wasmtime*`, `fixed`, `rand*` |
| gen, ai | math, data | `smallvec` | same as data, plus `serde_json`, `jsonschema` |
| sim | math, data, gen, ai | `serde`, `postcard`, `smallvec`, `bitflags`, `blake3` | `bevy*`, `glam`, `wgpu*`, `winit`, `wasmtime*`, `cranelift*`, `fixed`, `rand*`, `getrandom`, `rayon`, `tokio`, `steamworks`, `llama*` |
| net | math, data, sim | `postcard`, `crossbeam-channel`, `renet` (feature), `thiserror` | `bevy*`, `wgpu*` |
| bridge | math, sim | `arc-swap`, `crossbeam-channel` | `bevy*`, `wgpu*` |
| mod_api | math, data, sim | `wasmtime` (feature `host`), `wit-bindgen` (feature `guest`) | `bevy*` |
| scribe | data | `llama-cpp-2` (feature `llm`), `serde_json` | `palimpsest_sim` |
| render, audio, ui | math, data, bridge | Bevy family, crates named in §2.2 for that crate | `palimpsest_sim` as a direct dependency (only through bridge re-exports), `palimpsest_net` |
| platform | math, data, net | `steamworks` (feature `steam`), `directories`, `tempfile`, `ureq`, `rustls`, `ed25519-dalek`, `zstd`, `blake3`, `crash-handler`, `minidumper` | `bevy*` |
| bots, server | math, data, ai, sim, net, mod_api | `rayon` (bots), `clap`, `anyhow` | `bevy*`, `wgpu*` |
| client | every layer 0–5 crate | Bevy family | none beyond the above |
| services | math, data, sim, server | `axum`, `tokio`, `sqlx`, `sha2`, `hmac`, `ed25519-dalek` | `bevy*`, `wgpu*` |
| tools | math, data, gen, sim, bots, server | `clap`, `anyhow`, `rayon`, `dot_vox` | `bevy*` |

The rules live in content file `content/02/crate/crates.json`, one record per crate. `cargo xtask check-deps` reads `cargo metadata --format-version 1`, computes each package's full dependency closure, and fails on any violation. A name ending in `*` is a prefix match.

```json
{
  "id": "crate.sim",
  "package": "palimpsest_sim",
  "layer": 3,
  "no_std": true,
  "allow": ["crate.math", "crate.data", "crate.gen", "crate.ai"],
  "allow_external": ["serde", "postcard", "smallvec", "bitflags", "blake3"],
  "forbid_closure": ["bevy*", "glam", "wgpu*", "winit", "wasmtime*", "cranelift*", "fixed", "rand*", "getrandom", "rayon", "tokio", "steamworks", "llama*"]
}
```

### 3.4 Public API sketches

```rust
// ---- palimpsest_math ----
pub struct Fx(i64);                       // Canon §1.4; §4.1
pub struct Fx3 { pub x: Fx, pub y: Fx, pub z: Fx }
pub type Bam = u16;
pub fn floor_div(a: i64, b: i64) -> i64;  // Canon §1.3
pub fn mul_bp(x: i64, bp: i32) -> i64;
pub fn isqrt_u128(n: u128) -> u128;       // §4.2
pub fn sqrt_fx(x: Fx) -> Fx;
pub fn sin_bam(a: Bam) -> i32;            // Q2.30, Canon §1.9
pub fn cos_bam(a: Bam) -> i32;
pub fn atan2_bam(y: i64, x: i64) -> Bam;  // §4.4
pub fn yaw_from_dir(dx: Fx, dz: Fx) -> Bam;
pub mod geom { /* §4.6: closest_seg_seg, ray_capsule, capsule_capsule, vcapsule_aabb, Dda */ }
pub mod rng { pub struct Rng; pub fn stream_seed(..) -> u64; pub fn keyed_rng(..) -> Rng; } // Canon §3
pub fn fnv1a32(b: &[u8]) -> u32;
pub fn fnv1a64(b: &[u8]) -> u64;
pub fn blake3_256(parts: &[&[u8]]) -> [u8; 32];

// ---- palimpsest_data ----
pub struct Id32(pub u32);
pub struct ContentDb { /* typed tables in BTreeMap<Id32, Record> */ }
impl ContentDb {
    pub fn get<T: Record>(&self, id: Id32) -> Option<&T>;
    pub fn id_of(&self, s: &str) -> Option<Id32>;
    pub fn content_hash(&self) -> [u8; 32];              // §11.5
}
#[cfg(feature = "loader")]
pub fn load_content(roots: &[ContentRoot], unlocks: &[Id32]) -> Result<ContentDb, LoadReport>;

// ---- palimpsest_gen ----
pub fn generate_folio(input: &GenInput, db: &ContentDb, par: &dyn OrderedMap) -> FolioBlueprint;

// ---- palimpsest_ai ----
pub fn score_actions(ctx: &AiView, table: &UtilityTable, out: &mut ScoredActions);
pub fn astar(grid: &dyn NavGrid, from: CellIdx, to: CellIdx, budget: u32) -> PathResult;

// ---- palimpsest_sim ----  (§5.5 in full)
pub struct Sim;  pub struct SimConfig;  pub struct TickInputs;  pub struct StepOutput;
pub struct SimSnapshot;  pub struct StateHash(pub [u8; 32]);  pub struct ViewFrame;
pub trait ScriptHost { fn call(&mut self, hook: HookCall, q: &mut CommandQueue) -> HookResult; }
pub trait OrderedMap { fn map(&self, n: u32, f: &(dyn Fn(u32) -> ChunkBlob + Sync)) -> Vec<ChunkBlob>; }

// ---- palimpsest_net ----
pub trait Transport { fn send(&mut self, peer: PeerId, ch: Channel, bytes: &[u8]);
                      fn recv(&mut self) -> Option<(PeerId, Channel, Vec<u8>)>; }
pub struct HostSession;  pub struct ClientSession;   // doc 16 owns behavior
pub struct DesyncMonitor;                            // §6.7

// ---- palimpsest_bridge ----
pub struct ViewMailbox;                        // §7.1
impl ViewMailbox { pub fn publish(&self, f: Arc<ViewFrame>); pub fn pair(&self) -> FramePair; }
pub mod conv { pub fn fx_to_f32(x: Fx) -> f32; pub fn fx3_to_f32(v: Fx3) -> [f32; 3];
               pub fn bam_to_rad(a: Bam) -> f32; pub fn rad_to_bam(r: f32) -> Bam; }

// ---- palimpsest_platform ----
pub trait Platform { fn user(&self) -> UserInfo; fn storage(&self) -> &dyn SaveStorage;
    fn achievements(&mut self) -> &mut dyn Achievements; fn transport(&mut self) -> Option<Box<dyn Transport>>;
    fn text_input(&mut self, req: TextInputRequest); fn is_handheld(&self) -> bool; }

// ---- palimpsest_server ----
pub fn verify_replay(replay: &ReplayFile, db: &ContentDb) -> Result<VerifyReport, VerifyError>;

// ---- palimpsest_bots ----
pub trait BotBrain { fn command(&mut self, obs: &BotObservation) -> PlayerCommand; }
pub fn run_batch(cfg: &BatchConfig) -> BatchReport;
```

