# 02 — Technical Architecture

As of 2026-10-11. Status: BINDING. Conforms to canon-1.1.

This chapter fixes the stack, crate layout, simulation architecture, math library, determinism plan, presentation, pipelines, saves, mods, platforms, budgets, tests and coding rules. The canon file on disk is canon-1.3; every rule was also checked against it (Canon gaps G1).

## Imports

| Source | Consumed |
|---|---|
| DESIGN_BRIEF §5, §8 | Rust and Bevy, Bevy-free sim, fixed point, 60 Hz, 32³ chunks, light and sound grids, sim capsules, host-authoritative co-op, data content, WASM mods, on-device language model; jurisdiction flags, offline-complete mode |
| Canon §1.1–§1.4, §1.6 | ID grammar and `id32`, numeric types, integer and `Fx` primitives, integer-only content; delegation of `isqrt_u128` |
| Canon §1.8–§1.11 | coordinates, chunk index, `Bam`, sine table, `fwd`; delegation of `atan2_bam`; text keys; `EntityId`, ascending iteration, `HashMap` ban |
| Canon §2.1–§2.5 | `TICK_HZ`, `DT`, tick schedule and phases, session header, replay command record, save contract, `state_hash` |
| Canon §3 | xoshiro256++, SplitMix64, sequential and keyed streams, mod streams |
| Canon §4, §9 | caps and body dimensions; baked hitbox pose tables |
| Canon-1.3 §1.2, §2.3, §5.8 | light channels `light_sky`, `light_block`, effective = max; seam zones use Folio skins |
| Canon §18, §19, §21 | event IDs; namespaces `crate.`, `flag.`, `perf.`; acceptance tests 1–12, 29, 31–34 |
| R4 §0–§16 | versions read from crates.io, meshing, light texture, netcode parameters, audio split, modding, test pyramid |
| R6 GPV blocks D–F | `render_style`, `post`, `grade`, `anim_step`, `transition`, music layers |
| Docs 03, 04, 07, 09, 12–19 | not yet written; see Interface assumptions |

## 1. Architectural decisions

1. **One authority.** Only `Sim::step`, `Sim::restore` and `Sim::deserialize` write game state. Everything else reads output and sends inputs.
2. **The simulation is integer-only, single-threaded, `#![no_std]` with `alloc`.** `no_std` removes clocks, threads, files, environment and `HashMap` by construction. In-sim parallelism is limited to pure maps merged in index order (§5.3).
3. **Floats begin after one boundary,** the module `palimpsest_bridge::conv` (§7.5). `palimpsest_sim` has no float exception at all, stricter than Canon §21 test 33.
4. **Content is integer-only JSON** validated by JSON Schema 2020-12 (Canon §1.6; the brief also allowed RON).
5. **Pin everything:** exact versions, committed `Cargo.lock`, `=` pins for Bevy-family crates.
6. **Headless first:** every crate except the Bevy plugins builds and tests without window, GPU or audio device.
7. **Offline-complete:** every gameplay system runs without the backend (brief §8).

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

Versions were read from crates.io on 2026-10-11 (R4 §16, re-checked for crates R4 omits). "Pending" means the latest release targets Bevy 0.19; §2.4 decides.

| Crate | Version | Purpose and justification |
|---|---|---|
| `bevy` | `=0.20.0` | Engine for rendering, input, UI, assets. 0.20 adds schedule randomization, panic-to-error and mesh-shader groundwork (R4 §16.3). |
| `bevy_egui` | `=0.43.1` | Debug UI in the `dev_tools` feature only. |
| `bevy_brp_mcp`, `bevy_brp_extras` | `=0.23.0` | `dev_tools`: lets a coding agent drive, query and screenshot the client. |
| `bevy_seedling` + `firewheel` | pending (latest 0.8.0 / 0.14.0) | Audio graph with a Web Audio backend. Fallback `bevy_kira_audio =0.27.0`. |
| `audionimbus` | `0.16.0` | Steam Audio 4.8.1 HRTF and reflections, Apache-2.0. |
| `avian3d` | pending (latest 0.7.0) | Cosmetic physics only. Fallback `bevy_rapier3d =0.37.0`. |
| `bevy_hanabi` | pending (latest 0.19.0) | GPU particles. Fallback: in-house CPU billboards (§8.7). |
| `ogg`, `opus-decoder` | `0.9.2`, `0.1.1` | Pure-Rust Opus decoding (Symphonia 0.6.1 has no Opus); gated by RFC 8251 vectors in CI. |
| `fluent-bundle`, `unic-langid`, `fluent-langneg` | `0.16.0`, `0.9.6`, `0.14.2` | Localization with plural and gender selectors. |
| `arc-swap` | `1.9.2` | Lock-free view-frame mailbox. |
| `steamworks` | `0.13.1` | Lobbies, Datagram Relay, achievements, Cloud, Workshop, Input; own Bevy glue. |
| `crash-handler`, `minidumper` | `0.8.1`, `0.11.0` | Out-of-process minidumps, no third-party SDK. |
| `llama-cpp-2` | `0.1.159` | On-device flavor text, off by default (brief §5). |
| `serde`, `serde_json`, `postcard` | `1.0.229`, `1.0.154`, `1.1.3` | Serialization. postcard has a documented stable wire format and `no_std`; bincode is unmaintained (R4 §12). |
| `zstd`, `blake3` | `0.14.2`, `1.8.7` | Save compression; `state_hash`, `content_hash`, checksums (Canon §2.4–§2.5). |
| `schemars`, `jsonschema` | `1.2.2`, `0.58.6` | JSON Schema 2020-12 generated from Rust types; validation at build and load. |
| `thiserror`, `smallvec`, `bitflags` | `2.0.21`, `1.16.3`, `2.13.2` | Typed errors, inline lists, button masks. |
| `tracing`, `tracing-subscriber`, `tracing-tracy`, `tracy-client` | `0.1.44`, `0.3.23`, `0.12.0`, `0.19.0` | Logging and profiling. |
| `rayon`, `crossbeam-channel` | `1.12.0`, `0.5.17` | Ordered parallel maps outside the sim; job queues and in-process transport. |
| `notify`, `notify-debouncer-full` | `8.2.0`, `0.7.0` | Content hot reload in dev builds. |
| `renet` | `2.0.0` | LAN, dev and CI transport. |
| `wasmtime` | `=49.0.2` (`component-model`, `cranelift`, `pulley`) | Deterministic mod host with fuel and NaN canonicalization. |
| `wit-bindgen`, `dot_vox` | `0.62.0`, `5.2.0` | Mod guest SDK; MagicaVoxel prefab import. |
| `clap`, `anyhow` | `4.6.7`, `1.0.104` | Binaries only. |
| `directories`, `tempfile`, `fs-err` | `6.0.0`, `3.27.0`, `3.3.2` | Save paths, atomic writes, readable IO errors. |
| `ureq`, `rustls` | `3.4.2`, `0.23.45` | Telemetry and crash upload on a worker thread. |
| `ed25519-dalek`, `sha2`, `hmac` | `3.0.0`, `0.11.0`, `0.13.0` | Offline receipts; commit-reveal on the backend (doc 15). |
| `axum`, `tokio`, `sqlx` | `0.8.9`, `1.53.2`, `0.9.0` | Backend on PostgreSQL 18. |
| `proptest`, `insta`, `criterion`, `arbitrary`, `libfuzzer-sys` | `1.12.0`, `1.49.0`, `0.8.2`, `1.5.0`, `0.4.13` | Property, snapshot, benchmark, fuzz tests. |
| `binary-greedy-meshing` | `0.5.2` | Dev-dependency: oracle for the in-house mesher's face counts. |
| CLI tools | `cargo-nextest 0.9.148`, `cargo-mutants 27.1.0`, `cargo-fuzz 0.13.2`, `cargo-deny 0.20.2`, `cargo-llvm-cov 0.9.1` | CI. |

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

Bevy stays at `=0.20.0` for the whole 0.20 cycle. When the milestone that first needs a pending crate starts (§18 item 8), the primary is used if it has a release requiring `bevy ^0.20`; otherwise the fallback is used for the rest of the cycle. Migrating to Bevy 0.21 touches only `palimpsest_render`, `palimpsest_audio`, `palimpsest_ui` and `palimpsest_client`.

### 2.5 Bevy feature set

`bevy` uses `default-features = false` with `3d`, `ui`, `bevy_gltf`, `ktx2`, `zstd_rust`, `png`, `tonemapping_luts`, `smaa_luts`, `pbr_clustered_decals`, `multi_threaded`, `keyboard`, `mouse`, `gamepad`, `x11`, `wayland`. Dev adds `file_watcher`, `bevy_remote`, `hotpatching`; profiling adds `trace_tracy`; CI adds `debug` (for `shuffle_seed`) and `bevy_ci_testing`; web adds `webgpu`; Ultra builds on Windows and Linux add `bevy_solari`, `dlss`. `bevy_audio` stays off; `bevy_seedling` replaces it.

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
  assets/                 glTF, KTX2, Ogg/Opus, fonts, Fluent files (§11.4)
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
| `crate.scribe` | `palimpsest_scribe` | 4 | lib | no | on-device flavor-text jobs (`flag.scribe.enabled`, default off), template fallback; never linked into the simulation |
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
// palimpsest_math — §4
pub struct Fx(i64);  pub struct Fx3 { pub x: Fx, pub y: Fx, pub z: Fx }  pub type Bam = u16;
pub fn floor_div(a: i64, b: i64) -> i64;  pub fn mul_bp(x: i64, bp: i32) -> i64;      // Canon §1.3
pub fn isqrt_u128(n: u128) -> u128;  pub fn sqrt_fx(x: Fx) -> Fx;
pub fn sin_bam(a: Bam) -> i32;  pub fn cos_bam(a: Bam) -> i32;  pub fn atan2_bam(y: i64, x: i64) -> Bam;
pub mod geom;  pub mod rng;  pub fn fnv1a32(b: &[u8]) -> u32;  pub fn blake3_256(parts: &[&[u8]]) -> [u8; 32];
// palimpsest_data — §11
pub struct Id32(pub u32);
impl ContentDb { pub fn get<T: Record>(&self, id: Id32) -> Option<&T>; pub fn id_of(&self, s: &str) -> Option<Id32>;
                 pub fn content_hash(&self) -> [u8; 32]; }
#[cfg(feature = "loader")] pub fn load_content(roots: &[ContentRoot], unlocks: &[Id32]) -> Result<ContentDb, LoadReport>;
// palimpsest_gen, palimpsest_ai
pub fn generate_folio(input: &GenInput, db: &ContentDb, par: &dyn OrderedMap) -> FolioBlueprint;
pub fn score_actions(view: &AiView, table: &UtilityTable, out: &mut ScoredActions);
pub fn astar(grid: &dyn NavGrid, from: CellIdx, to: CellIdx, budget: u32) -> PathResult;
// palimpsest_sim — §5.5
pub trait ScriptHost { fn call(&mut self, hook: HookCall, q: &mut CommandQueue) -> HookResult; }
pub trait OrderedMap { fn map(&self, n: u32, f: &(dyn Fn(u32) -> Vec<u8> + Sync)) -> Vec<Vec<u8>>; }
// palimpsest_net — doc 16 owns behavior
pub trait Transport { fn send(&mut self, peer: PeerId, ch: Channel, bytes: &[u8]);
                      fn recv(&mut self) -> Option<(PeerId, Channel, Vec<u8>)>; }
// palimpsest_bridge — §7
impl ViewMailbox { pub fn publish(&self, f: Arc<ViewFrame>); pub fn pair(&self) -> FramePair; }
// palimpsest_platform — §12, §14, §17.4
pub trait Platform { fn storage(&self) -> &dyn SaveStorage; fn achievements(&mut self) -> &mut dyn Achievements;
    fn transport(&mut self) -> Option<Box<dyn Transport>>; fn text_input(&mut self, r: TextInputRequest); fn is_handheld(&self) -> bool; }
// palimpsest_server, palimpsest_bots — §16
pub fn verify_replay(replay: &ReplayFile, db: &ContentDb) -> Result<VerifyReport, VerifyError>;
pub trait BotBrain { fn command(&mut self, obs: &BotObservation) -> PlayerCommand; }
```

## 4. Fixed-point math library (`palimpsest_math`)

### 4.1 `Fx` and overflow policy

```rust
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Serialize, Deserialize)]
#[repr(transparent)]
pub struct Fx(i64);                  // raw r represents r / 2^32 (Canon §1.4)
impl Fx {
    pub const ZERO: Fx = Fx(0);
    pub const ONE: Fx = Fx(1 << 32);
    pub const DT: Fx = Fx(71_582_788);            // Canon §2.1
    pub const fn raw(self) -> i64;
    pub const fn from_raw(r: i64) -> Fx;
    pub fn from_int(n: i32) -> Fx;                // Canon §1.4
    pub fn from_milli(m: i64) -> Fx;              // = fx_from_milli, Canon §1.4
    pub fn mul_int(self, n: i64) -> Fx;           // checked raw * n
    pub fn div_int(self, n: i64) -> Fx;           // floor_div(raw, n), n > 0
    pub fn floor_block(self) -> i32;              // = fx_floor_block
    pub fn to_milli_floor(self) -> i64;           // floor_div(raw as i128 * 1000, 2^32), checked
}
impl Add, Sub, Neg, AddAssign, SubAssign for Fx;  // checked; panic on overflow
pub fn mul_fx(a: Fx, b: Fx) -> Fx;  pub fn div_fx(a: Fx, b: Fx) -> Fx;  pub fn mul_bp_fx(x: Fx, bp: i32) -> Fx;
```

- `Mul` and `Div` operator traits are not implemented for `Fx × Fx`, so every product names its flooring through `mul_fx` or `div_fx`.
- **Overflow policy.** Every operation is checked. Overflow panics in every profile (Canon §1.3). The panic is deterministic, so every peer panics on the same tick. The sim thread wraps `step` in `catch_unwind` at the binary boundary, writes a crash dump containing the last checkpoint and input log (§6.7), and ends the session. Saturating arithmetic is used only where a formula says clamp.
- **Rounding examples.** `mul_fx(Fx(1), Fx(-1)) = Fx(-1)` because the arithmetic shift floors `-1 >> 32` to `-1`. `mul_fx(Fx(1), Fx(1)) = Fx(0)`. `div_fx(7, 2)` is raw 15032385536 (3.5). `div_fx(1, 3)` is raw 1431655765. `div_fx(-1, 3)` and `div_fx(1, -3)` are both raw -1431655766, because `div_fx` negates both operands when the divisor is negative and then floors.
- `Fx::from_milli(4300).raw() == 18468359372` (Canon §21 test 11). `Fx::from_milli(1620).raw() == 6957847019`, the standing eye height.

### 4.2 `isqrt_u128` and `sqrt_fx`

```rust
/// SPEC: 02 §4.2. Floor square root by Newton's method from above.
pub fn isqrt_u128(n: u128) -> u128 {
    if n < 2 { return n; }
    let bits = 128 - n.leading_zeros();          // significant bits, 2..=128
    let mut x: u128 = 1u128 << ((bits + 1) / 2); // x0 = 2^ceil(bits/2) >= sqrt(n)
    loop {
        let y = (x + n / x) >> 1;                // never overflows: x <= 2^64, n / x < 2^65
        if y >= x { return x; }
        x = y;
    }
}
pub fn sqrt_fx(x: Fx) -> Fx {                    // Canon §1.4
    assert!(x.raw() >= 0);
    Fx::from_raw(isqrt_u128((x.raw() as u128) << 32) as i64)
}
```

- The loop runs at most 7 times for any `u128` input (measured over 300 inputs per bit length 2 to 128). A property test cross-checks the result against `u128::isqrt` from core for 100,000 random inputs.
- Examples: `isqrt_u128(99) = 9`; `isqrt_u128(2^128 - 1) = 2^64 - 1`; `sqrt_fx(Fx::from_int(2)).raw() = 6074000999` (1.4142135622); `sqrt_fx(Fx::from_int(50)).raw() = 30370004999` (7.0710678117), the jump launch term for `JUMP_H = 1250 mm` at `g = 20 m/s²` (Canon §4.2).

### 4.3 Sine and cosine

- The table `T` (Canon §1.9) is committed as `crates/math/src/trig_table.rs`, `pub static SIN_T: [i32; 16385]`, generated by `tools/gen_trig.py` with mpmath at 50 digits.
- Spot values: `T[1] = 102944`, `T[2] = 205887`, `T[4096] = 410903207`, `T[8192] = 759250125`, `T[10923] = 929904853`, `T[16383] = 1073741819`, `T[16384] = 1073741824`.
- **Integrity check.** BLAKE3 of the 16,385 values serialized as little-endian `i32` (65,540 bytes) is `d2612a42910edad3b81095ad0be154d6076c5c33771916cbbea2ded40419ec11`. A unit test recomputes it.
- `sin_bam`, `cos_bam` and `fwd` are exactly Canon §1.9. A Q2.30 unit vector converts to `Fx` by `Fx::from_raw((q30 as i64) << 2)`.
- Example: `fwd(16384, 0) = (-2^30, 0, 0)`, so yaw 16384 faces west (−X), consistent with counter-clockwise yaw seen from above.

### 4.4 `atan2_bam` (16-iteration CORDIC)

`atan2_bam(y, x)` returns the `Bam` angle of the vector `(x, y)` measured counter-clockwise from +x, in `0..=65535`. Internally the angle accumulates in units of `2^-16` Bam, so one full turn is `2^32`.

```rust
/// round_half_even(atan(2^-i) / (2*pi) * 2^32), i = 0..=15, generated by tools/gen_trig.py
pub const ATAN_T: [i64; 16] = [536870912, 316933406, 167458907, 85004756, 42667331, 21354465,
    10679838, 5340245, 2670163, 1335087, 667544, 333772, 166886, 83443, 41722, 20861];

/// SPEC: 02 §4.4
pub fn atan2_bam(y: i64, x: i64) -> Bam {
    if y == 0 { return if x >= 0 { 0 } else { 32768 }; }     // (0, 0) returns 0
    if x == 0 { return if y > 0 { 16384 } else { 49152 }; }
    // 1. Normalize so max(|x|, |y|) lies in [2^52, 2^53).
    let m = x.unsigned_abs().max(y.unsigned_abs());
    let s = m.leading_zeros() as i32 - 11;
    let (mut x, mut y) = if s >= 0 { (((x as i128) << s) as i64, ((y as i128) << s) as i64) }
                         else      { (((x as i128) >> -s) as i64, ((y as i128) >> -s) as i64) };
    // 2. Pre-rotate into the right half-plane.
    let mut z: i64 = 0;
    if x < 0 { x = -x; y = -y; z = 1 << 31; }                // +180 degrees
    // 3. Vectoring: drive y to 0, accumulating the removed angle.
    for i in 0..16 {
        let (dx, dy) = (y >> i, x >> i);                      // arithmetic shifts (floor)
        if y > 0 { x += dx; y -= dy; z += ATAN_T[i]; }
        else     { x -= dx; y += dy; z -= ATAN_T[i]; }
    }
    ((z + (1 << 15)) >> 16) as u16                          // round half up, wrap to 0..=65535
}
pub fn yaw_from_dir(dx: Fx, dz: Fx) -> Bam { atan2_bam(-dx.raw(), -dz.raw()) }   // Canon §1.9 yaw
pub fn pitch_from_dir(h: Fx, dy: Fx) -> i16 { (atan2_bam(dy.raw(), h.raw()) as i16).clamp(-16000, 16000) }
```

- After normalization the CORDIC gain (1.6468) keeps `|x|, |y| < 2^55`, far from `i64` overflow.
- **Accuracy.** Against the exact angle, the error is at most 0.818 Bam over 300,000 random inputs at magnitudes from 2^10 to 2^62, and at most 0.788 Bam over every integer pair in `-40..=40`. The contract is `|error| <= 1 Bam`.
- Examples: `atan2_bam(1, 1) = 8192`; `atan2_bam(3, 4) = 6712` (exact 6711.96); `atan2_bam(4, 3) = 9672`; `atan2_bam(7, -24) = 29808`; `atan2_bam(-5, -5) = 40960`; `atan2_bam(2, -1) = 21220`; `atan2_bam(1, 1000000) = 0`.
- Yaw examples: `yaw_from_dir(-1 m, 0) = 16384` (west), `yaw_from_dir(0, -1 m) = 0` (north), `yaw_from_dir(1 m, 0) = 49152` (east), `yaw_from_dir(-1 m, -1 m) = 8192` (north-west).

### 4.5 `Fx3`

```rust
impl Fx3 {
    pub fn add(self, o: Fx3) -> Fx3;  pub fn sub(self, o: Fx3) -> Fx3;  pub fn neg(self) -> Fx3;
    pub fn scale(self, s: Fx) -> Fx3;                 // mul_fx per axis
    pub fn dot_wide(self, o: Fx3) -> i128;            // Σ a.raw * b.raw, Q64.64, never narrowed
    pub fn dot(self, o: Fx3) -> Fx;                   // Fx((dot_wide >> 32) as i64), checked
    pub fn cross(self, o: Fx3) -> Fx3;                // mul_fx terms, x then y then z
    pub fn length(self) -> Fx;                        // Fx(isqrt_u128(dot_wide as u128))
    pub fn try_normalize(self) -> Option<Fx3>;        // None if length.raw < 256; else div_fx per axis
}
pub fn dist2(p: Fx3, q: Fx3) -> i128;                 // Canon §1.4
pub fn dist(p: Fx3, q: Fx3) -> Fx { Fx::from_raw(isqrt_u128(dist2(p, q) as u128) as i64) }
```

Because `dot_wide` is Q64.64, its integer square root is directly Q32.32. Example: `length((3 m, 4 m, 0))` has `dot_wide = 25 · 2^64` and returns exactly `5 · 2^32`.

### 4.6 Capsules, rays and the voxel grid

`Capsule { a: Fx3, b: Fx3, r: Fx }` is a segment with radius. The canon movement capsule is vertical: `VCapsule { origin, radius, height }` is the segment from `origin + (0, r, 0)` to `origin + (0, height - r, 0)` (Canon §1.8, §4.1). Hitbox capsules come from the baked pose tables (Canon §9). Doc 03 builds the character controller and sweeps on these primitives.

**Closest points between segments.** `EPS = Fx(256)` (2^-24 m²). Scalars are `Fx`; `denom` and `num` are `i128` products of raws.

```rust
pub fn closest_seg_seg(p1: Fx3, q1: Fx3, p2: Fx3, q2: Fx3) -> (Fx, Fx, Fx3, Fx3) {
    let (d1, d2, r) = (q1.sub(p1), q2.sub(p2), p1.sub(p2));
    let (a, e, f) = (d1.dot(d1), d2.dot(d2), d2.dot(r));
    let (s, t);
    if a <= EPS && e <= EPS { s = Fx::ZERO; t = Fx::ZERO; }
    else if a <= EPS { s = Fx::ZERO; t = clamp01(div_fx(f, e)); }
    else {
        let c = d1.dot(r);
        if e <= EPS { t = Fx::ZERO; s = clamp01(div_fx(-c, a)); }
        else {
            let b = d1.dot(d2);
            let denom = a.raw() as i128 * e.raw() as i128 - b.raw() as i128 * b.raw() as i128;
            let mut s0 = if denom > 0 {
                let num = b.raw() as i128 * f.raw() as i128 - c.raw() as i128 * e.raw() as i128;
                clamp01(Fx::from_raw(floor_div_i128(num << 32, denom) as i64))
            } else { Fx::ZERO };
            let tnom = mul_fx(b, s0) + f;
            if tnom < Fx::ZERO      { t = Fx::ZERO; s0 = clamp01(div_fx(-c, a)); }
            else if tnom > e        { t = Fx::ONE;  s0 = clamp01(div_fx(b - c, a)); }
            else                    { t = div_fx(tnom, e); }
            s = s0;
        }
    }
    (s, t, p1.add(d1.scale(s)), p2.add(d2.scale(t)))
}
```

Example 1: segment (0,0,0)–(2,0,0) against (1,−1,1)–(1,1,1). `a = e = 4`, `b = 0`, `c = −2`, `f = 2`, `denom = 16`, `num = 8`, so `s = t = 0.5` (raw 2147483648 each), closest points (1,0,0) and (1,0,1), `dist2 = 2^64` (exactly 1 m²).

**Capsule overlap.** `capsule_overlap(c1, c2) = dist2(cp1, cp2) <= (c1.r + c2.r).raw()²` in `i128`, with `cp1`, `cp2` from `closest_seg_seg`.

**Vertical capsule against a block.** For an axis-aligned box `[lo, hi]` and a vertical segment at `(x, z)` spanning `[y0, y1]`, the distance is separable and exact: `dx = max(0, lo.x − x, x − hi.x)`, `dz` likewise, `dy = max(0, lo.y − y1, y0 − hi.y)`; overlap iff `dx² + dy² + dz² < r²` with `i128` raw squares. Example: origin (2.5, 1.0, 0.5), r 0.3, height 1.8 (segment y 1.3..2.5) against block (3, 1, 0): `dx = 0.5`, `dy = dz = 0`, 0.25 ≥ 0.09, no overlap; at x 2.75, `dx = 0.25`, 0.0625 < 0.09, overlap.

**Ray against capsule** (Quilez form; `rd` must be unit length from `try_normalize`):

```rust
pub fn ray_capsule(ro: Fx3, rd: Fx3, pa: Fx3, pb: Fx3, r: Fx) -> Option<Fx> {
    let (ba, oa) = (pb.sub(pa), ro.sub(pa));
    let (baba, bard, baoa, rdoa, oaoa) = (ba.dot(ba), ba.dot(rd), ba.dot(oa), rd.dot(oa), oa.dot(oa));
    let a = baba - mul_fx(bard, bard);
    let b = mul_fx(baba, rdoa) - mul_fx(baoa, bard);
    let c = mul_fx(baba, oaoa) - mul_fx(baoa, baoa) - mul_fx(mul_fx(r, r), baba);
    if a > EPS {
        let h = b.raw() as i128 * b.raw() as i128 - a.raw() as i128 * c.raw() as i128;   // Q64.64
        if h < 0 { return None; }
        let t = div_fx(-b - Fx::from_raw(isqrt_u128(h as u128) as i64), a);
        let y = baoa + mul_fx(t, bard);
        if y > Fx::ZERO && y < baba { return nonneg(t); }                              // cylinder body
        return ray_sphere(ro, rd, if y <= Fx::ZERO { pa } else { pb }, r);              // nearer cap
    }
    min_nonneg(ray_sphere(ro, rd, pa, r), ray_sphere(ro, rd, pb, r))                  // ray parallel to axis
}
fn ray_sphere(ro: Fx3, rd: Fx3, cen: Fx3, r: Fx) -> Option<Fx> {
    let oc = ro.sub(cen);
    let b = rd.dot(oc);
    let h = b.raw() as i128 * b.raw() as i128 - (((oc.dot(oc) - mul_fx(r, r)).raw() as i128) << 32);
    if h <= 0 { return None; }
    nonneg(-b - Fx::from_raw(isqrt_u128(h as u128) as i64))
}
// Helpers: clamp01(x) = x.clamp(Fx::ZERO, Fx::ONE); nonneg(t) = Some(t) if t >= 0 else None;
// min_nonneg(a, b) = the smaller of the Some values; floor_div_i128 is Canon §1.3 floor_div on i128;
// Fx::MAX = Fx(i64::MAX); Fx3 implements Index<usize> (0 = x, 1 = y, 2 = z).
```

Worked example: ray from (0, 1.0, 0) along +X against the capsule (5, 0.3, 0)–(5, 1.5, 0) with r = 0.3 m. `a = 1.44`, `b = −7.2`, `c = 35.8704`, `h = 0.186624`, `sqrt h = 0.432`, `t = (7.2 − 0.432) / 1.44 = 4.7`, `y = 0.84` inside `(0, 1.44)`: body hit at raw `t = 20186346293` (4.7000000004 m). The same ray at height 1.7 m hits the top cap at raw 20514452597 (4.7763932024 m; exact value 4.7763932023). At height 2.0 m it misses.

**Voxel traversal (Amanatides–Woo DDA).** Visits every block a ray crosses, in order; axis ties resolve x, then y, then z. Direction components with `|raw| < 256` count as 0, so `div_fx(Fx::ONE, di)` cannot overflow.

```rust
pub fn dda(o: Fx3, d: Fx3, t_max: Fx, mut visit: impl FnMut(IVec3, Fx, Option<Face>) -> bool) {
    let mut cell = [o.x.floor_block(), o.y.floor_block(), o.z.floor_block()];
    let (mut step, mut t_delta, mut t_next) = ([0i32; 3], [Fx::MAX; 3], [Fx::MAX; 3]);
    for i in 0..3 {
        let (oi, di) = (o[i], d[i]);
        if di > Fx::ZERO { step[i] = 1;  t_delta[i] = div_fx(Fx::ONE, di);
                           t_next[i] = div_fx(Fx::from_int(cell[i] + 1) - oi, di); }
        if di < Fx::ZERO { step[i] = -1; t_delta[i] = div_fx(Fx::ONE, -di);
                           t_next[i] = div_fx(oi - Fx::from_int(cell[i]), -di); }
    }
    if !visit(cell.into(), Fx::ZERO, None) { return; }
    loop {
        let mut ax = 0; if t_next[1] < t_next[ax] { ax = 1; } if t_next[2] < t_next[ax] { ax = 2; }
        let t = t_next[ax];
        if t > t_max { return; }
        cell[ax] += step[ax];
        t_next[ax] = t_next[ax] + t_delta[ax];
        if !visit(cell.into(), t, Some(Face::entered(ax, step[ax]))) { return; }   // false = stop
    }
}
```

Example: origin (0.5, 1.5, 0.5), direction (0.6, 0, −0.8), `t_max` = 4 m, block (2, 1, −3) solid. Visits (0,1,0) at t 0; (0,1,−1) at raw 2684354559 (0.625) entering through +Z; (1,1,−1) at 0.833333 through −X; (1,1,−2) at 1.875 through +Z; (2,1,−2) at 2.5 through −X; (2,1,−3) at raw 13421772797 (3.125) through +Z, where it stops.

### 4.7 Hashes and RNG

`palimpsest_math::rng` contains Canon §3.1 verbatim, `stream_seed` (§3.2) and `keyed_rng` (§3.3), which takes `run_seed` as an explicit first parameter. `Rng` implements `Clone`, `Serialize` and `Deserialize`, so its four state words are part of every snapshot. Debug builds count draws per stream in a side table that is never serialized. `fnv1a32`, `fnv1a64` use Canon §1.1 constants. `blake3_256(parts)` hashes the concatenation of `parts`.

## 5. Simulation architecture (`palimpsest_sim`)

### 5.1 Decision: custom sorted-column store

The simulation uses a custom struct-of-columns store keyed by the canon `EntityId`, not an ECS crate. `bevy_ecs` would put Bevy in the sim closure (brief §5) and iterates by archetype table, not ascending ID (Canon §1.11); `hecs`, `legion` and `specs` share the ordering problem; a generational arena such as `slotmap` adds nothing because canon IDs are never reused and carry the Folio index in bits 40 and up. With sorted columns, new IDs always exceed every live ID (the counter only grows within a Folio, and carried entities have smaller `folio_index`), so inserts are appends, iteration is ascending by construction and lookups are binary searches. The canon caps bound the capped kinds at 3,328 entities (256 actors, 1,024 projectiles, 2,048 ground items).

### 5.2 Store

```rust
pub struct Column<T> { ids: Vec<EntityId>, vals: Vec<T>, dead: Vec<u64> /* bitset over indices */ }
impl<T> Column<T> {
    pub fn get(&self, id: EntityId) -> Option<&T>;             // binary search, skips dead
    pub fn get_mut(&mut self, id: EntityId) -> Option<&mut T>;
    pub fn iter(&self) -> impl Iterator<Item = (EntityId, &T)>;  // ascending EntityId
    fn push(&mut self, id: EntityId, v: T);                     // asserts id > last id
    fn kill(&mut self, id: EntityId);                           // sets dead bit
    fn compact(&mut self);                                      // retain live, end of tick
}
pub fn join2<'a, A, B>(a: &'a Column<A>, b: &'a Column<B>) -> impl Iterator<Item = (EntityId, &'a A, &'a B)>;
pub fn join2_mut<'a, A, B>(a: &'a mut Column<A>, b: &'a Column<B>) -> impl Iterator<Item = (EntityId, &'a mut A, &'a B)>;
pub struct World {                // field list generated by macro from doc 03's component catalog
    pub pos: Column<Pos>, pub vel: Column<Vel>, pub body: Column<BodyState>, /* … */
    pub items: ItemStore,         // same column design keyed by item uid (Canon §1.11)
    pub grid: FolioGrid,          // chunked voxel and field grids
    next_entity: u64, next_uid: u64, folio_index: u32,
}
pub struct FolioGrid { chunks: [Arc<Chunk>; 128] }   // copy-on-write through Arc::make_mut
```

- **Chunk index.** The Folio box (Canon §1.8) holds 8 × 2 × 8 chunks. Chunk slot `ci = cx + 8 * (cz + 8 * cy)`, mirroring the canon block index. Doc 03 owns `Chunk`'s fields; this chapter owns the `Arc` copy-on-write wrapper and the order used by serialization.
- **Structural changes are deferred.** Systems mutate component values through `join*_mut` and request spawn, despawn, add-component and remove-component through `ctx.cmds`. `spawn` allocates the `EntityId` immediately (Canon §1.11), so the ID order equals request order. The scheduler applies the queue after each system, in request order.
- `compact()` runs once at the end of every tick.

### 5.3 Scheduling

```rust
pub struct SystemDesc { pub id: Id32 /* sys.* (doc 03) */, pub period: u32, pub phase: Phase, pub run: fn(&mut World, &mut Ctx) }
pub enum Phase { Fixed(u32), PerEntity }
impl SystemDesc {
    pub fn due(&self, t: u64) -> bool {
        match self.phase { Phase::Fixed(p) => (t + p as u64) % self.period as u64 == 0, Phase::PerEntity => true }
    }
}
pub fn entity_due(e: EntityId, t: u64, period: u32) -> bool {                 // Canon §1.11 phase key
    (t + (e.0 & 0xFF_FFFF_FFFF) % period as u64) % period as u64 == 0
}
pub static SCHEDULE: &[SystemDesc] = &[ /* doc 03 order; periods and phases from Canon §2.2 */ ];
```

Example: entity `(3 << 40) | 5` with AI think period 6 has phase key 5 and thinks on ticks where `(t + 5) % 6 == 0`, that is `t = 1, 7, 13, …`.

The simulation runs on one thread. The only in-sim parallelism is `OrderedMap`, injected by the host binary for worldgen and doc 03's pure batch computations: `map(n, f)` returns `f(0..n)` in index order, whatever threads computed them. The default is sequential.

### 5.4 Tick lifecycle

```rust
pub fn step(&mut self, inp: &TickInputs) -> StepOutput {
    assert_eq!(inp.tick, self.tick);                       // boundary = next tick to simulate
    self.ctx.begin(inp);                                   // commands by slot; intents sorted by (slot, action_seq)
    for sys in SCHEDULE {
        if sys.due(self.tick) { (sys.run)(&mut self.world, &mut self.ctx); self.world.apply(&mut self.ctx.cmds); }
    }
    self.world.compact();
    self.tick += 1;
    let hash = if self.flags.contains(SimFlags::HASH_EVERY_TICK)
                  || (self.flags.contains(SimFlags::HASH_AT_60) && self.tick % 60 == 0) { Some(self.state_hash()) } else { None };
    if self.flags.contains(SimFlags::EMIT_VIEW) { self.extract_view(&mut self.view); }
    StepOutput { tick: self.tick - 1, events: core::mem::take(&mut self.ctx.events), hash, diagnostics: self.ctx.take_diag() }
}
```

A state hash always describes a boundary: the state after simulating tick `b − 1`, before simulating tick `b`. Production hosts hash at boundaries `b ≡ 0 (mod 60)` (Canon §2.5) asynchronously (§6.5); CI uses `HASH_EVERY_TICK`.

### 5.5 Sim API

```rust
pub const SIM_VERSION: u32 = 1;
pub struct SimConfig {
    pub run_seed: u64, pub mode: Id32, pub party: [Option<PartyMember>; 4],
    pub content: Arc<ContentDb>, pub flags: SimFlags,
    pub script_host: Option<Box<dyn ScriptHost + Send>>,    // WASM mods (§13); None = no scripted effects
    pub ordered_map: Option<Box<dyn OrderedMap + Send + Sync>>,
}
bitflags! { pub struct SimFlags: u32 { const EMIT_VIEW = 1; const HASH_EVERY_TICK = 2; const HASH_AT_60 = 4; } }
impl Sim {
    pub fn new(cfg: SimConfig) -> Result<Sim, SimInitError>;
    pub fn tick(&self) -> u64;
    pub fn step(&mut self, inputs: &TickInputs) -> StepOutput;
    pub fn snapshot(&self) -> SimSnapshot;               // clones columns; shares chunks through Arc
    pub fn restore(&mut self, s: &SimSnapshot);           // afterwards state_hash() == s.state_hash()
    pub fn state_hash(&self) -> StateHash;                // BLAKE3 of serialize() (Canon §2.5)
    pub fn section_hashes(&self) -> [StateHash; SECTIONS];
    pub fn serialize(&self, out: &mut Vec<u8>);           // canonical serialization (§6.5), the save payload
    pub fn deserialize(bytes: &[u8], cfg: SimConfig) -> Result<Sim, LoadError>;
    pub fn extract_view(&self, out: &mut ViewFrame);      // §7.2
}
impl SimSnapshot { pub fn serialize(&self, out: &mut Vec<u8>); pub fn state_hash(&self) -> StateHash; } // off-thread hashing
```

**Input packet.** The command is exactly the Canon §2.4 record:

```rust
#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerCommand { pub tick: u64, pub slot: u8, pub move_x: i8, pub move_z: i8, pub yaw: u16,
                           pub pitch: i16, pub buttons: u32, pub action_seq: u32, pub view_tick: u64 }
pub struct TickInputs { pub tick: u64, pub commands: [Option<PlayerCommand>; 4], pub intents: Vec<PlayerIntent> }
pub struct PlayerIntent { pub tick: u64, pub slot: u8, pub action_seq: u32, pub kind: IntentKind }
```

- `(move_x, move_z)` is in the player's yaw frame with canon axes: at yaw 0, `(0, -127)` is full forward (−Z) and `(127, 0)` is full right (+X). The sim maps `-128` to `-127`. The sim clamps `pitch` to `-16000..=16000` (Canon §1.9).
- `buttons` carries held state. The sim derives presses by comparing with the slot's previous buttons, which are part of simulation state.

| Bit | Name | Bit | Name |
|---|---|---|---|
| 0 | `ATTACK_PRIMARY` | 15–17 | `QUICKBAR_INDEX` (0..=7) |
| 1 | `ATTACK_SECONDARY` (aim, block or alt fire per doc 05) | 18 | `QUICKBAR_USE` |
| 2 | `RELOAD` | 19 | `GRIMOIRE_NEXT` |
| 3 | `JUMP` | 20 | `GRIMOIRE_PREV` |
| 4 | `CROUCH` | 21 | `LEAN_LEFT` (canon-1.2 `move.lean_l`) |
| 5 | `SPRINT` | 22 | `LEAN_RIGHT` |
| 6 | `DODGE` | 23 | `PING` |
| 7 | `INTERACT` | 24 | `WALK` (slow-walk modifier) |
| 8 | `MELEE_QUICK` | 25 | `MOUNT_TOGGLE` |
| 9 | `CAST` | 26–31 | reserved, must be 0; the replay validator rejects set bits |
| 10 | `THROW` | | |
| 11, 12 | `WEAPON_NEXT`, `WEAPON_PREV` | | |
| 13–14 | `WEAPON_SELECT` (0 none, 1..=3 slot) | | |

- **Intents** carry discrete menu actions on the reliable channel: `EquipItem`, `UnequipItem`, `MoveItem`, `DropItem`, `UseItemOn`, `Craft`, `Brew`, `ComposeSpell`, `ChooseExit`, `ShopBuy`, `ShopSell`, `CommitOffer` (Canon §3.5), `AllocatePoint`, `PickTalent`, `MinionCommand`, `Pray`. The owning document of each system defines payload validation. Intents apply at the start of their tick in `(slot, action_seq)` order and are logged in replays beside the commands (Canon gaps G4).

**Output.** `StepOutput { tick, events: Vec<SimEvent>, hash: Option<StateHash>, diagnostics: Vec<Diagnostic> }`. `SimEvent { id: Id32 /* ev.* */, seq: u32, payload: EventPayload }` keeps emission order; `seq` counts from 0 each tick. Doc 03 owns payload schemas. Diagnostics (mod disabled, budget overrun, rejected intent) are informational and never read back by the simulation.

## 6. Determinism plan

### 6.1 Forbidden constructs

| Construct | Why | Enforcement |
|---|---|---|
| `f32`, `f64`, float literals in math, data core, gen, ai, sim | platform and libm differences (Canon §1.2) | `cargo xtask float-scan` lexer, clippy float lints |
| `HashMap`, `HashSet`, any hash-ordered iteration | per-process random seeds (Canon §1.11) | `no_std` (unavailable), `disallowed-types` |
| clocks, threads, environment, filesystem | nondeterministic inputs | `no_std` |
| `rand`, `getrandom` | unseeded randomness | `forbid_closure` (§3.3) |
| `sort_unstable*` | equal-key order unspecified | `disallowed-methods`; sorts use stable sort with `EntityId` as the final key |
| `usize`, `isize` in serialized state | 32-bit on wasm32 | `xtask serde-scan`; state uses fixed-width integers |
| pointer-to-integer casts, `type_name`, `TypeId` hashing | address and compiler dependence | `xtask` grep for `as *const`, `as *mut`; `disallowed-methods` |
| RNG draws inside `#[cfg(debug_assertions)]` | debug and release diverge | `xtask` grep |
| `RefCell`, `Rc`, atomics in sim | hidden mutation order | `disallowed-types` |

### 6.2 Lint configuration

```toml
# crates/{math,data,gen,ai,sim}/clippy.toml
too-many-lines-threshold = 60
too-many-arguments-threshold = 7
cognitive-complexity-threshold = 25
disallowed-types = [
  { path = "std::collections::HashMap", reason = "Canon §1.11: use BTreeMap or sorted Vec" },
  { path = "std::collections::HashSet", reason = "Canon §1.11: use BTreeSet or sorted Vec" },
  { path = "hashbrown::HashMap", reason = "Canon §1.11" },
  { path = "core::cell::RefCell", reason = "02 §6.1: hidden mutation order" },
  { path = "alloc::rc::Rc", reason = "02 §6.1: reference entities by EntityId" },
  { path = "core::sync::atomic::AtomicU64", reason = "02 §6.1: the simulation is single-threaded" },
]
disallowed-methods = [
  { path = "core::slice::<impl [T]>::sort_unstable", reason = "02 §6.1" },
  { path = "core::slice::<impl [T]>::sort_unstable_by", reason = "02 §6.1" },
  { path = "core::slice::<impl [T]>::sort_unstable_by_key", reason = "02 §6.1" },
  { path = "core::any::type_name", reason = "02 §6.1" },
]
```

```rust
// lib.rs of math, gen, ai, sim (data core identical without no_std when `loader` is on)
#![no_std]
#![forbid(unsafe_code)]
#![deny(clippy::float_arithmetic, clippy::float_cmp, clippy::lossy_float_literal, clippy::cast_precision_loss,
        clippy::unwrap_used, clippy::wildcard_enum_match_arm, clippy::disallowed_types, clippy::disallowed_methods,
        clippy::too_many_lines, missing_docs)]
extern crate alloc;
```

`cargo xtask float-scan` is the authoritative check for Canon §21 test 33. It lexes every `.rs` file in the five crates, skips comments and string literals, removes items under `#[cfg(test)]`, and fails on the tokens `f32`, `f64`, any literal containing `.` followed by a digit, any decimal literal with an exponent, and any `f32`/`f64` suffix.

### 6.3 Cargo profiles

```toml
[profile.dev]
opt-level = 1
overflow-checks = true
[profile.release]
opt-level = 3
overflow-checks = true          # every package; Canon §1.3 requires it for the sim
lto = "thin"
panic = "unwind"                # the sim thread boundary uses catch_unwind
debug = "line-tables-only"
[profile.dist]                  # shipped builds
inherits = "release"
lto = "fat"
codegen-units = 1
[profile.ci-det]                # determinism matrix
inherits = "release"
debug-assertions = true
[profile.release.package.palimpsest_sim]
overflow-checks = true          # explicit, survives any future workspace default change
```

No build uses `-C target-cpu=native` or fast-math flags.

### 6.4 Cross-platform determinism matrix

| Target | Runner | Profile | Workload |
|---|---|---|---|
| `x86_64-unknown-linux-gnu` | GitHub `ubuntu-24.04` | `dev`, `ci-det` | unit, property, golden fast set, determinism set |
| `aarch64-unknown-linux-gnu` | GitHub `ubuntu-24.04-arm` | `ci-det` | determinism set |
| `x86_64-pc-windows-msvc` | GitHub `windows-2025` | `ci-det` | determinism set |
| `aarch64-apple-darwin` | GitHub `macos-15` | `ci-det` | determinism set |
| `wasm32-wasip1` | `ubuntu-24.04`, run under the wasmtime CLI | `ci-det` | determinism set; 32-bit `usize` catches width bugs |
| Steam Deck (SteamOS 3, self-hosted `deck-01`) | self-hosted | `dist` | nightly performance gates (§15) |

The determinism set is 50 golden replays of 3,600 ticks on every pull request and 500 replays of 18,000 ticks nightly. Each job writes `hashes.json`, a sorted list of `{replay, boundary, hash}`. A final job requires all five files to be byte-identical.

### 6.5 Canonical serialization and state hash

The serialization is a fixed sequence of sections. Each section is the postcard encoding of its struct; maps are `BTreeMap`s, so keys serialize in sorted order (Canon §2.5).

| # | Section | Contents |
|---|---|---|
| 0 | `meta` | `SIM_VERSION`, boundary tick, run seed, mode, Folio index, `EntityId` and uid counters, party slots |
| 1 | `rng` | every sequential stream's `Rng` state, keyed by stream name |
| 2 | `entities` | every component column in catalog order, each as `(EntityId, value)` pairs ascending |
| 3 | `items` | item instances ascending by uid |
| 4 | `blocks` | 128 chunks in slot order, palette-RLE encoded |
| 5 | `light` | packed light byte per chunk (doc 03 §4.1), palette-RLE; the relight FIFO |
| 6 | `fluids` | liquid and gas cells, ascending per chunk |
| 7 | `thermal` | fire cells and temperature grids |
| 8 | `ai` | agent memories, plans and cached paths |
| 9 | `run` | lattice history, Shock of the New set, Death's Door flags, stored offers, Erasure clocks, score state |
| 10 | `inputs` | previous buttons and last `action_seq` per slot |
| 11 | `mods` | per-mod enabled flag and mod-owned key-value state |

```
serialization = for k in 0..12: u8(k) ‖ u32le(len(S_k)) ‖ S_k
state_hash    = BLAKE3(serialization)                  // Canon §2.5
section_hash  = BLAKE3(S_k)                            // desync localization only

palette_rle(cells: &[T; 32768]):                       // T is u8 or u16
    palette = distinct values in order of first appearance, scanning index 0..=32767
    if len(palette) <= 256: emit 1u8, varint(len(palette)), palette values (LE), then runs (u8 palette index, varint run length)
    else:                    emit 2u8, then runs (T value LE, varint run length)
```

An all-air chunk encodes in 8 bytes: tag 1, palette length 1, value `0u16`, then index 0 with run 32768. Each chunk caches its encoded bytes together with the generation counter they were built from, so a hash re-encodes only chunks that changed.

**Cost and cadence.** The serialization budget for a mid-run Folio is 2 MiB. BLAKE3 at 1.5 GB/s on one Steam Deck core takes 1.4 ms, so the host never hashes on the sim thread: at each boundary `b ≡ 0 (mod 60)` it takes `snapshot()` (budget 0.15 ms) and a worker thread serializes and hashes it (budget 4 ms, result due within 10 ticks).

### 6.6 Golden replays

- A golden replay is a container (§12.1) of kind `replay`: session header (Canon §2.4), mode and party, command log, intent log, and checkpoints `(boundary, StateHash)` at 600, 1,800, 3,600 and every further 3,600.
- `pal golden verify` re-simulates and compares at each checkpoint. On a mismatch it re-runs from the last matching checkpoint with `HASH_EVERY_TICK` and reports the first diverging boundary, the differing sections, and the first differing `(column, EntityId)` or `(chunk, cell)`.
- The fast set (50 replays) covers each core genre idle and in combat, Folio generation for all 16 genres, liquid and fire stress, 4-player co-op, save and reload at tick 4,321 (Canon §21 test 32), and one WASM test mod.
- `pal golden bless` rewrites expected hashes; CI rejects a golden-hash change unless the same pull request changes `SIM_VERSION` or `content/`.

### 6.7 Desync detection and dump

1. Every peer holding a full replica (doc 16 decides which) hashes at boundaries `b ≡ 0 (mod 60)`; the host sends `HashReport { boundary, hash }` on the reliable channel.
2. A peer with a different hash sends `SectionHashRequest { boundary }`; the host keeps section hashes for its last 8 hash boundaries and answers with all 12.
3. Both sides write `desync_<boundary>_<role>.pald` (container kind `desync`): local serialization at `b`, both section-hash sets, the input and intent log since the last matching boundary `b0`, the serialization at `b0` (peers keep their last 2 matching snapshots), and git hash, target triple and profile. The client then requests a full resync (doc 16).
4. `pal desync-diff a.pald b.pald` prints the first differing section and inside it the first differing `(column, EntityId, field)`, `(chunk, cell, a, b)` or RNG stream; `--resim` replays from `b0` and reports the first diverging tick.

**SyncTest.** With `flag.sim.synctest_depth = N > 0`, every tick runs snapshot, N steps, restore, the same N steps, and asserts equal hashes. CI uses N = 8, which proves `snapshot` and `restore` capture all state before netcode exists.

## 7. Sim-render bridge (`palimpsest_bridge`)

### 7.1 Threads and mailbox

The host or single-player client runs the simulation on its own thread. It steps at 60 Hz from a monotonic-clock accumulator, catching up at most 4 ticks per wake. After each step with `EMIT_VIEW` it publishes `Arc<ViewFrame>` to the `ViewMailbox`. The mailbox is a double buffer held in one `ArcSwap<FramePair>`: `publish` swaps in `FramePair { prev: old.curr, curr: new }`, so the render world loads a consistent pair once per frame without locking. A remote client feeds frames reconstructed from host snapshots (doc 16) into the same mailbox. The simulation never waits for rendering.

### 7.2 View frame

```rust
pub struct ViewFrame {
    pub tick: u64, pub published_at_ns: u64,           // published_at is set by the bridge, not the sim
    pub entities: Vec<ViewEntity>,                     // ascending EntityId
    pub chunk_updates: Vec<(u8, Arc<Chunk>)>,          // chunks whose block generation changed
    pub field_dirty: Vec<u8>,                          // chunks whose light or bleed weights changed
    pub players: [Option<ViewPlayer>; 4],              // HUD resources, statuses, equipment
    pub events: Arc<[SimEvent]>,
}
pub struct ViewEntity { pub id: EntityId, pub visual: Id32, pub pos: Fx3, pub yaw: Bam, pub pitch: i16,
    pub anim: Id32, pub anim_tick: u32, pub origin_genre: Id32, pub flags: u32 /* TELEPORTED, HIDDEN, DOWNED, GHOST, … */ }
```

### 7.3 Interpolation

- `alpha = clamp((now_ns − curr.published_at_ns) × 60 / 10^9, 0, 1)` as `f32`. Positions render at `lerp(prev.pos, curr.pos, alpha)`. The renderer never extrapolates (Canon §2.1).
- Yaw takes the short arc: `d = curr.yaw.wrapping_sub(prev.yaw) as i16`, rendered yaw `= prev.yaw + d × alpha`.
- An entity snaps to `curr` when `TELEPORTED` is set or when it moved more than 4 m in one tick.
- The local camera's yaw and pitch come from the current input state, so look has no simulation latency. Its position comes from prediction (doc 16) or, on the host, from interpolation.
- Example: `curr` was published 7.67 ms ago, so `alpha = 0.46`; an entity at x = 10.0 m in `prev` and 10.1 m in `curr` renders at 10.046 m.

### 7.4 Entity mapping

The client keeps `BTreeMap<EntityId, bevy::Entity>` and merge-walks each new `curr` against the previous frame: new IDs spawn presentation from their `visual` record (doc 18), missing IDs despawn. On `ev.entity_killed` the body becomes a cosmetic ragdoll (at most 16, oldest removed first). Ragdolls and debris never feed back (Canon §22).

### 7.5 Conversion boundary `palimpsest_bridge::conv`

This module is the only place where simulation values become floats.

| Function | Definition | Example |
|---|---|---|
| `fx_to_f32(x)` | `((x.raw() as f64) * 2^-32) as f32` | `Fx::from_milli(1620)` → `1.62f32` (bits `0x3FCF5C29`) |
| `fx3_to_f32(v)` | per axis | |
| `bam_to_rad(a)` | `a as f32 * (TAU / 65536.0)` | 16384 → 1.5707964 |
| `pitch_to_rad(p)` | `p as f32 * (TAU / 65536.0)` | −16000 → −1.5339808 |
| `light_to_unorm(l)` | `l * 17` | 15 → 255 |
| `bp_to_f32(bp)` | `bp as f32 / 10000.0` | |
| `rad_to_bam(r)` | `((r / TAU * 65536.0).round() as i64).rem_euclid(65536) as u16` | π/2 → 16384; −π/2 → 49152; 30° → 5461 |
| `stick_to_i8(v)` | `(v.clamp(-1.0, 1.0) * 127.0).round() as i8` | 0.5 → 64; −1.0 → −127 |

The reverse functions exist only to build `PlayerCommand`; the integers they produce are what the replay logs, so determinism holds.

### 7.6 Event routing

Doc 18's `vfx.` and `aud.` records declare a `trigger`, e.g. `{ "event": "ev.damage_dealt", "dominant_type": "dmg.fire" }`, indexed at load by event `id32`. Per `SimEvent` the router picks at most one VFX and one audio record: most matching predicates (`dominant_type`, `status`, `material`, `local_genre`, `visual`), ties by ascending content ID. Effects spawn at the event position in the event's genre skin.

## 8. Rendering (`palimpsest_render`)

### 8.1 Frame order

1. Shadow maps. 2. Depth prepass. 3. Opaque voxels (custom pipeline). 4. Opaque entities. 5. Cutout voxels and foliage. 6. Sky and fog. 7. Liquids and translucent geometry, sorted back to front by chunk. 8. Clustered decals. 9. Particles. 10. Viewmodel. 11. Post stack. 12. Anti-aliasing and upscaling. 13. Folio transition overlay. 14. UI.

### 8.2 Voxel chunk meshing

- An in-house port of binary greedy meshing (R4 §4.2) meshes each 32³ chunk from a 34³ padded copy: the chunk plus a 1-cell border from all 26 neighbors, so ambient occlusion sees diagonal neighbors. Each occupancy column is one `u64` with 34 bits used.
- Opaque, cutout and translucent faces mesh separately. Greedy merging joins faces only when the merge key `(material, AO corners, render class)` is equal. Translucent faces are not merged.
- Packed quad, 8 bytes:

| Word | Bits | Field |
|---|---|---|
| 0 | 0–4, 5–9, 10–14 | x, y, z (0..=31) |
| 0 | 15–19, 20–24 | width − 1, height − 1 |
| 0 | 25–27 | face: 0 +X, 1 −X, 2 +Y, 3 −Y, 4 +Z, 5 −Z |
| 1 | 0–11 | material index (0..=4095, from doc 07's block registry) |
| 1 | 12–19 | AO, 2 bits per corner c0..c3 |
| 1 | 20–21 | render class: 0 opaque, 1 cutout, 2 translucent |
| 0, 1 | rest | 0 |

  Example: x 3, y 7, z 12, width 4, height 2, face +Y, material 517, AO (3, 3, 2, 1), opaque → word0 `0x0411B0E3`, word1 `0x0006F205`.
- The vertex shader pulls `quads[vertex_index / 6]` and the corner from `[0, 1, 2, 0, 2, 3][vertex_index % 6]`. No index buffer.
- Each chunk stores six face-direction ranges, each drawn by one indirect draw. The CPU culls a direction range when every face in it is back-facing: +X faces are culled when `camera.x < chunk_min_x + 1`, −X faces when `camera.x > chunk_max_x`, and likewise for Y and Z.
- Meshing runs on Bevy's async compute pool. Dirty chunks queue by camera distance; at most 8 remeshes start per frame. Budget: 100 µs per chunk on Steam Deck (`perf.deck.mesh.chunk_us`). Folio load meshes all 128 chunks while the transition plays.

### 8.3 Per-genre block materials

Each genre ships one `texture_2d_array` of up to 256 layers at 32 × 32 px, BC7 in KTX2 with 6 mip levels (1,392 bytes per layer). The voxel material binds three arrays, for the Folio genre (slot 0) and up to two bleed genres (slots 1 and 2), plus a storage buffer `layer_of[slot][material] -> u16`. The quad carries only the material index, so a genre change never remeshes. Normal and roughness data come from per-layer material parameters (doc 18), not extra textures.

### 8.4 First-person viewmodel

- World camera: horizontal FOV 90° by default (setting 70–110°), converted to vertical FOV `2·atan(tan(h/2)·H/W)`: 64.01° at 16:10. Near plane 0.05 m.
- Viewmodel: `RenderLayers::layer(1)`, a second `Camera3d` with `order = 1`, vertical FOV 54°, near plane 0.01 m, colour kept and depth cleared to 0.0 (reverse-Z). It samples the field texture at the eye, so it is exactly as dark as the player's cell.
- Full-body awareness: the local avatar renders on layer 0 with head bones scaled to zero in first person; a shadow-only copy keeps the head's shadow. Third person restores the head and puts the camera 2.5 m behind on a spring arm colliding with the voxel grid.

### 8.5 Lighting and light agreement

- Dynamic lights use Bevy's clustered forward path: at most 64 visible point or spot lights on Steam Deck (128 desktop), of which 4 cast shadows (256² cube maps). Sun and moon use 2 cascades at 1024², reaching 48 m on Deck.
- The **field texture** is one `Rgba8Unorm` 3D texture of 256 × 64 × 256 texels (16 MiB) covering the Folio box: R = `light_sky × 17`, G = `light_block × 17` (canon-1.2 two-channel light), B and A = 255 where the block's skin genre is bleed slot 1 or 2, else 0. Dirty chunks upload as 32³ slabs of 128 KiB, at most 8 per frame.
- Shading samples the field trilinearly at `P + 0.5 m × N`: at a face centre this is exactly the air cell in front, so the face reads that cell's simulation light and blends smoothly across the face. `L = max(R, G) / 17` is the effective light (Canon §2.3 `light_at`).
- **Agreement clamp.** Ambient luminance is `ambient(L)` and the final pre-tonemap luminance is clamped to at most `cap(L)`, both interpolated between the integer entries below (content values in milli-units, exposure-normalized):

| L | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| `cap_milli` | 20 | 24 | 37 | 59 | 90 | 129 | 177 | 233 | 299 | 373 | 456 | 547 | 647 | 756 | 874 | 1000 |
| `ambient_milli` | 5 | 6 | 9 | 15 | 22 | 32 | 44 | 58 | 75 | 93 | 114 | 137 | 162 | 189 | 218 | 250 |

  `cap_milli(L) = round_half_even(20 + 980 × L² / 225)`; `ambient_milli = round_half_even(cap_milli × 250 / 1000)`. Because the clamp applies after direct lighting, an unshadowed lamp behind a wall cannot brighten a cell the simulation calls dark. Stealth darkness and visual darkness therefore agree by construction.
- **Global illumination.** The field texture is the baseline indirect light. Voxel AO is baked per quad corner. SSAO is off on Deck and on at the High preset. Bevy Solari path tracing is the Ultra preset on ray-tracing GPUs (`flag.render.solari`), and its output passes through the same clamp. No hardware ray tracing on Deck (R4 §2.3).

### 8.6 Post-process stack

Order: bloom, AgX tonemapping, 32³ colour-grading LUT (KTX2, one per genre), ink outline (Sobel on depth and normals, `outline_px`), halftone or posterize, chromatic aberration, scanlines, vignette, film grain, ordered dither. Parameters come from the genre's visual block (`grade`, `post`, `render_style`; R6, doc 12, doc 18) as integers in milli-units. Render styles `pbr`, `painterly`, `toon`, `ink`, `pixel_dither` and `xerox_collage` are shader variants of one material. A foreign entity renders with its `origin_genre` style and animation step, so a visitor from another genre is visible at a glance (R6). Comfort settings (doc 17) disable chromatic aberration, grain, scanlines, vignette, camera shake and Dutch angles.

### 8.7 Anti-aliasing, upscaling, particles, decals

- Deck preset: native 1280 × 800, 60 fps, SMAA High. Deck Battery: 40 fps cap, render scale 0.75 plus contrast-adaptive sharpening. Desktop: TAA with sharpening, or DLSS on RTX (Vulkan, Windows and Linux; `flag.render.dlss`). No frame generation.
- Particles: `bevy_hanabi`, 32,768 live on Deck, 131,072 on desktop; CPU fallback 4,096 billboards.
- Decals: Bevy clustered decals, 256 live, block-anchored and removed on that block's `ev.block_changed`; oldest evicted first.

### 8.8 Folio transition (page turn)

The final frame of the old Folio is captured to a texture and drawn on a 64 × 36 vertex page mesh with page coordinate `u ∈ [0, W]`, `W = 2.0`, spine at `u = 0` (screen x = `u − 1`). Over 90 ticks the curl line moves from `c = W` to `c = −πR` with `R = 0.15` and smoothstep easing `e(t) = 3t² − 2t³`:

```
c(t) = W − (W + πR) × e(t)
u ≤ c:                       (u, 0)                                   // still flat
θ = (u − c)/R < π:           (c + R sin θ, R (1 − cos θ))             // on the cylinder
otherwise:                   (c − (u − c − πR), 2R)                   // turned over
```

Example at `t = 0.5`: `c = 0.7644`; the point `u = 1.0` maps to (0.9144, 0.15) and `u = 1.9` maps to (0.1, 0.3). If the new Folio is not ready when `e(t)` reaches 0.5, the page holds there with an ink-writing loop until loading completes. Other GPV transition styles (ink-bleed, glitch, iris, film-burn) reuse the pass with a different vertex and fragment function; page-turn is the default.

### 8.9 Genre-bleed blend

For a fragment at world position `P` (metres), with `w1`, `w2` the trilinear bleed weights from the field texture, `n ∈ [0, 1)` from a tiling 64³ value-noise texture sampled at `P × 4`:

```
t_k   = smoothstep(0.35, 0.65, w_k + (n − 0.5) × 0.5)          k ∈ {1, 2}
t_0   = max(0, 1 − t_1 − t_2)
albedo = t_0·A[layer_of[0][m]] + t_1·A[layer_of[1][m]] + t_2·A[layer_of[2][m]]
ink   = any |t_k − 0.5| < 0.04  →  albedo × 0.6                 // a ragged ink line on the boundary
```

Examples: `w1 = 0.5`, `n = 0.5` gives `t1 = 0.5` (on the ink line); `w1 = 0.75`, `n = 0.2` gives input 0.6 and `t1 = 0.9259`. Weights are 0 or 1 per block and texel centres sit at block centres, so the blend spans half a block on each side of the true boundary and the noise keeps the ink line within 0.25 m of the boundary the simulation uses for `local_genre` (Canon §5.8).

### 8.10 Steam Deck targets

| Metric | Target |
|---|---|
| Resolution, frame rate | 1280 × 800, 60 fps locked |
| GPU frame time | ≤ 14.0 ms (`perf.deck.frame.gpu`) |
| Main-thread CPU | ≤ 6.0 ms (`perf.deck.frame.cpu_main`) |
| Render-thread CPU | ≤ 5.0 ms (`perf.deck.frame.cpu_render`) |
| Draw calls, main view | ≤ 1,200 (`perf.deck.draw.calls`) |
| VRAM | ≤ 2,560 MiB (`perf.deck.mem.vram`) |

## 9. Audio (`palimpsest_audio`)

- **Engine.** `bevy_seedling` (Firewheel) with `audionimbus` (Steam Audio 4.8.1) for binaural HRTF and reflection reverb; fallback per §2.4. Buses: master → music, sfx, ambience, voice, ui. Voice cap 64 on Deck, 128 on desktop; a new voice steals the quietest lower-priority voice.
- **Two acoustic systems.** The simulation's sound grid (doc 09) decides what NPCs hear and also sets loudness and occlusion, so the player hears what enemies hear. Steam Audio's own occlusion is off to avoid double attenuation.
- **Occlusion.** For each `ev.noise`, the payload gives the source level `L_src` dB and, per player slot, the propagated level `L_lis` at the listener cell and the arrival direction of the last propagation hop (Interface assumptions). With straight-line distance `d` metres:

```
att    = L_src − L_lis                                  // total attenuation, applied as gain
free   = 20 × log10(max(d, 1))
extra  = max(0, att − free)                             // dB lost to walls and doors
cutoff = max(500 Hz, 20000 Hz × 10^(−extra / 40))       // low-pass
pos    = extra ≥ 6 ? listener + arrival_dir × min(d, 4 m) : source position
```

  Example: `L_src = 80`, `L_lis = 44`, `d = 10 m` → `att = 36`, `free = 20`, `extra = 16`, cutoff 7,962 Hz, and the sound appears to come from the doorway it arrived through.
- **Music stems.** Each genre track (doc 18) has five sample-aligned looping stems of equal length: `bed`, `pulse`, `tension`, `combat`, `apex`. Intensity level selects the audible set: 0 explore {bed}; 1 suspicion {bed, pulse}; 2 search {bed, pulse, tension}; 3 combat {bed, pulse, tension, combat}; 4 boss or hunter {all}. Level = 4 if a boss or Erasure hunter is alive and alerted, else 3 if the local player is `in_combat` (Canon §2.3), else 2 if any hostile holds awareness at search level or above toward a party member, else 1 if any hostile is suspicious, else 0. Rises apply at the next bar; falls apply at a bar boundary after the lower level has held for 8 s. Stems fade in or out over one beat. Inside a bleed zone the bleed genre's track crossfades in over 2 bars at the same level.
- **Formats.** Ogg Opus, 48 kHz: effects 96 kbit/s mono; stems 128 kbit/s stereo, streamed.

## 10. Input, UI and localization

- **Input.** An in-house action layer in `palimpsest_ui` maps keyboard, mouse and gamepad (Bevy `gilrs` under Steam Input) to doc 17's `input.*` actions. Each render frame accumulates look deltas into absolute yaw and pitch (`f32`); each sim tick builds one `PlayerCommand` (§5.5) through `conv` (§7.5). It is in-house because it must quantize into the canon command and no input crate had a Bevy 0.20 release on 2026-10-11.
- **UI.** Game UI is `bevy_ui` with custom widgets (`PalButton`, `PalList`, `PalGrid`, `PalTabs`, `PalSlider`, `PalTooltip`, `PalItemCard`, `PalModal`), gamepad-navigable through Bevy's directional focus navigation and themed per genre by doc 17's `ui.*` skins. Layout is authored at 1280 × 800 and scales by `min(w / 1280, h / 800)`; body text ≥ 12 px. Developer tools (inspector, grid overlays including the sound field, schedule timings) use `bevy_egui` behind `dev_tools` only.
- **Localization.** Fluent files `assets/loc/<locale>/<owner_doc>.ftl`. Fluent identifiers cannot contain dots, so a text key maps to a message ID by replacing each `.` with `-` (`loc.arch.melee.blade_long.skin.wuxia.name` → `loc-arch-melee-blade_long-skin-wuxia-name`); text keys contain no `-` (Canon §1.10), so the map is reversible. Fallback: selected locale, then `en-US`. Numbers follow Canon §1.5 display rules with locale grouping. CI screenshot tests run the pseudo-locale `en-XA` (accented, 30% longer). Latin-extended and CJK fonts ship; doc 18 picks launch locales.

## 11. Content pipeline

### 11.1 Files and schemas

- Content lives in `content/<owner_doc>/<namespace>/<name>.json`. Each file is `{ "schema": "<schema id>", "records": [ … ] }`; every record has an `"id"` field (Canon §1.1).
- Rust record types in `palimpsest_data` derive `JsonSchema` (schemars 1.2.2), which emits JSON Schema 2020-12. `pal schema export` writes `schemas/*.schema.json`; CI fails if the committed schemas differ from the generated ones.
- **Validation** runs in CI (`pal validate`) and at every load: (1) a token pre-pass rejects any number token containing `.`, `e` or `E` (Canon §1.6) and any integer outside `−(2^53 − 1) ..= 2^53 − 1`; (2) `jsonschema` 0.58.6 validates against the schema; (3) semantic checks: ID grammar, every referenced ID exists, ownership of the namespace by the file's owning document (Canon §19), weights and shares that must sum to 10000, `Σ c_i ≤ 15000` in weapon scaling (Canon §6.4 row 12).
- **ID registry.** The loader computes `id32 = fnv1a32(id)` for every record and alias and rejects collisions (Canon §1.1); mods get error `id32_collision`.

### 11.2 Content hash

```
content_hash = BLAKE3(
    "PALIMPSEST/CONTENT/1\n"
    ‖ "REC" ‖ u32le(n) ‖ for each record in ascending id bytes: u32le(len(id)) ‖ id ‖ u32le(len(body)) ‖ body
    ‖ "UNL" ‖ u32le(n) ‖ for each unlock id ascending: u32le(len) ‖ id
    ‖ "MOD" ‖ u32le(n) ‖ for each mod ascending by mod id: u32le(len) ‖ mod_id ‖ u32le(len) ‖ version ‖ BLAKE3(mod blob)
    ‖ "FLG" ‖ u32le(n) ‖ for each sim-affecting flag ascending: u32le(len) ‖ id ‖ i64le(value))
body = RFC 8785 canonical JSON of the record after defaults are applied (sorted keys, no whitespace)
```

Example: records `{"id":"crate.sim","layer":3}` and `{"id":"crate.math","layer":0}` with no unlocks, mods or flags give a 141-byte input and `content_hash = 92c159a0a347070d679d6edd626a8bc6938565f60204dd732ba14c2ac3592cc9`.

### 11.3 Hot reload

With `flag.dev.hot_reload`, `notify` watches `content/` and `assets/`; a changed file reloads, revalidates and swaps the `Arc<ContentDb>` between frames. During an active run, release builds refuse with `content_reload_refused_in_run` (Canon §2.4); dev builds apply the change, log the new `content_hash` and mark the session replay invalid. Assets hot-reload through Bevy's file watcher in dev builds.

### 11.4 Assets

| Kind | Format | Notes |
|---|---|---|
| Meshes, skeletons, animations | glTF 2.0 binary `.glb` | Bevy loader; presentation only |
| Hitbox pose tables | JSON content, integer mm | baked from `.glb` by `pal bake-poses` (Canon §9) |
| Textures, texture arrays, LUTs | KTX2, BC7 with zstd supercompression; UASTC for the web build | Bevy `ktx2`, `zstd_rust` |
| Audio | Ogg Opus 48 kHz | §9 |
| Fonts | OTF | |
| Prefab voxel structures | MagicaVoxel `.vox` → JSON content | `pal import-vox` |

Asset paths follow content IDs owned by doc 18: an ID `art.mesh.enemy.wolf_dire` resolves to `assets/art/mesh/enemy/wolf_dire.glb`: every dot becomes a directory separator and the asset kind supplies the extension.

## 12. Save system

### 12.1 Container

Run saves, the meta profile, replays and desync dumps share one container. All integers are little-endian.

| Offset | Size | Field |
|---|---|---|
| 0 | 4 | magic `PALS` |
| 4 | 2 | `container_version` = 1 |
| 6 | 1 | kind: 1 run suspend, 2 meta profile, 3 replay, 4 desync dump |
| 7 | 1 | flags: bit 0 zstd, bit 1 signed |
| 8 | 4 | `sim_version` |
| 12 | 32 | `content_hash` |
| 44 | 8 | `run_seed` (0 for the meta profile) |
| 52 | 8 | boundary tick |
| 60 | 8 | uncompressed payload length |
| 68 | 8 | stored payload length |
| 76 | 32 | BLAKE3 of the stored payload |
| 108 | 4 | first 4 bytes of BLAKE3(bytes 0..108) as `u32` |
| 112 | n | payload: zstd frame of the postcard payload (§6.5 serialization for run saves) |
| 112 + n | 64 | Ed25519 signature over bytes `0..112+n`, when flag bit 1 is set |

Loading checks, in order: magic, header check, payload length, payload BLAKE3, signature, then `sim_version` and `content_hash`. Each failure has its own error code (`bad_magic`, `header_corrupt`, `payload_truncated`, `payload_corrupt`, `bad_signature`, `sim_version_mismatch`, `content_mismatch`).

### 12.2 Writing

zstd level 3 for run saves and the profile, level 9 for replays. Every write goes to a temporary file in the same directory, is flushed with `fsync`, renamed over the target, and the previous file is kept as `.bak`. Steam Auto-Cloud syncs `profile/` and `runs/`. Budget: 50 ms on Steam Deck for a run save (`perf.deck.save_ms`).

### 12.3 Run suspend saves

Single player saves at the quit boundary; co-op saves at Folio entry on the host (Canon §2.5). A run resumes only when `sim_version` and `content_hash` match the build. Otherwise the game offers the Steam branch `prev` (previous build, kept 30 days) or "Close the Book": `ev.run_ended` with reason `abandoned_version`, rewarded by doc 14's rules for a run ending at that Folio. `SIM_VERSION` increments in any commit that can change any state hash for any input: sim code, canon formulas, RNG use or serialization layout.

### 12.4 Meta profile

The payload holds `profile_version: u32` and the account cache: unlocks, Codex progress, Echo roster, settings references, and the Margin plot (canon-1.2 §17.8, 64 × 32 × 64 blocks, palette-RLE). The online backend is authoritative; offline, signed receipts (doc 15) are verified with `ed25519-dalek`. Migrations are a chain `migrate_v{n}_to_v{n+1}`; a fixture profile of every shipped version lives in `tests/fixtures/profile_v{n}.pals`, and CI loads each one through the chain. Profiles are never migrated downward.

## 13. Modding

### 13.1 Data packs first

A mod is `mods/<mod_id>/` with `mod.json` (`id` matching `^[a-z][a-z0-9_]{2,31}$`, `version`, `api: 1`, `depends`, `load_after`), `content/`, `assets/`, and `effects.wasm` when it has scripted effects. Content uses the base schemas and validation. Mod content IDs use second segment `x_<mod_id>`, e.g. `item.x_frostpack.ice_blade` (Canon gaps G5). Load order: base, then mods topologically by `load_after`, ties by `mod_id`. Mods draw only from `rng.mod.<mod_id>.<name>` streams (Canon §3.7).

### 13.2 WASM scripted effects

```rust
let mut cfg = wasmtime::Config::new();
cfg.wasm_component_model(true);
cfg.consume_fuel(true);                    // deterministic interruption
cfg.cranelift_nan_canonicalization(true);  // identical NaN bits on every platform
cfg.wasm_relaxed_simd(false);
cfg.wasm_threads(false);
cfg.epoch_interruption(false);             // wall-clock based, never used
// consoles: cfg.target("pulley64") — interpreter, no JIT
```

- No WASI imports: no clock, filesystem, environment or entropy. The host interface carries integers only. Floats inside a module are allowed, since WASM float arithmetic is IEEE-exact and NaN bits are canonicalized.
- Fuel: 1,000,000 per hook call and 4,000,000 per mod per tick. Linear memory is capped at 64 MiB and the stack at 512 KiB. On exhaustion the call traps: its queued commands are discarded and the mod is disabled for the rest of the run. The disabled flag lives in serialization section 11, so every peer disables it on the same tick.
- Mod commands enter the same pipelines as base content, for example damage through Canon §10.5.

### 13.3 Mod API surface (WIT)

```wit
package palimpsest:mod@1.0.0;
interface types {
  type entity-id = u64;  type id32 = u32;
  record fx3 { x: s64, y: s64, z: s64 }                        // Fx raws
  record ctx { tick: u64, folio: u32, actor: entity-id, target: option<entity-id>, item-uid: option<u64>, seq: u32 }
  variant command { deal-damage(tuple<entity-id, id32, s64>), apply-status(tuple<entity-id, id32, u32>),
                    heal-bp(tuple<entity-id, s32>), spawn(tuple<id32, fx3>), emit-noise(tuple<fx3, s32>),
                    set-block(tuple<s32, s32, s32, id32>), set-state(tuple<u32, list<u8>>) }
}
interface host {
  use types.{entity-id, id32, fx3, command};
  rng-range: func(stream: string, n: u64) -> u64;              // keyed by the current ctx (Canon §3.3)
  entity-pos: func(e: entity-id) -> option<fx3>;
  entity-stat: func(e: entity-id, stat: id32) -> option<s64>;
  block-at: func(x: s32, y: s32, z: s32) -> id32;
  entities-in-radius: func(c: fx3, r: s64, max: u32) -> list<entity-id>;   // ascending EntityId
  get-state: func(key: u32) -> option<list<u8>>;
  queue: func(cmd: command);
}
interface effects {
  use types.{ctx};
  on-activate: func(c: ctx, effect: u32, params: list<s64>);
  on-hit: func(c: ctx, effect: u32, params: list<s64>, dealt-mhp: s64);
  on-status-pulse: func(c: ctx, status: u32, stacks: u32);
  ai-consideration: func(c: ctx, consideration: u32) -> s32;  // Bp score
}
world palimpsest-mod { import host; export effects; }
```

Within one hook call, `rng-range(name, n)` uses one keyed generator `keyed_rng("rng.mod.<mod_id>.<name>", folio, ctx.actor, ctx.tick, ctx.seq)` per stream name and draws from it sequentially.

### 13.4 Multiplayer mod set

`modset_hash = BLAKE3("PALIMPSEST/MODSET/1" ‖ M)`, with `M` the §11.2 `MOD` section after its tag, is shown in the lobby; one mod `frostpack` version `1.0.0` with an empty blob gives a 77-byte input and `a4cd5df9faa66d265baf53d652ff672a1e3b76f2466877f8eaa64d45ea905817`. Because `content_hash` contains the `MOD` section (§11.2), peers with different mod sets fail the Canon §2.4 handshake. A joining player missing a Workshop mod gets a subscribe prompt; a data-only pack up to 64 MiB also transfers from the host on the reliable channel.

## 14. Platforms

- **Steamworks** (`steamworks` 0.13.1): 4-member lobbies carrying `sim_version`, `content_hash`, `modset_hash` and mode; Datagram Relay as production `Transport`; achievements, Auto-Cloud, Workshop, rich presence, Input glyphs, on-screen keyboard, `IsSteamRunningOnSteamDeck` for presets, and doc 15's microtransaction callback.
- **Steam Deck Verified checklist:** every screen gamepad-navigable, including first launch, seed entry and lobby; Deck glyphs everywhere; on-screen keyboard for every text field; 60 fps at 1280 × 800 in the saturated scenario (§15); text ≥ 12 px; no launcher or external account; native Linux build; survives suspend (single player pauses when a wake gap exceeds 1 s; co-op uses doc 16 timeouts); no anti-cheat.
- **Desktop:** Linux x86_64 and aarch64 (Steam Frame flatscreen) in Steam Linux Runtime 3.0; Windows x86_64 with DX12 by default, Vulkan fallback; macOS 14+ on Apple silicon with Metal, notarized.
- **Consoles (Switch 2, PS5, Xbox Series)** are a post-1.0 port behind the same `Platform` and `Transport` traits. Fixed constraints: a private wgpu backend per console; no JIT, so WASM mods run on Pulley or are off; Switch 2 budget 7.5 GB total (9 GB available) with Scribe off and asset pools halved; saves through platform storage with suspend-safe writes; the `no_std` simulation crates need no OS services; certification items (suspend, user switching, network loss) map onto existing pause and Folio-entry save points.
- **WebGPU demo:** `wasm32-unknown-unknown` with Bevy `webgpu`; single player, Volume 1, no mods, Scribe or Steam; the sim steps on the main thread, at most 4 ticks per animation frame; Firewheel Web Audio; settings in `localStorage`, no suspend saves; download ≤ 120 MB; Chromium 113+ (Windows, macOS, ChromeOS), Safari 26, Firefox 141+ on Windows (R4 §2.1).

## 15. Performance budgets

**Simulation tick on Steam Deck** (one Zen 2 core, host, 4 players, every canon cap saturated: scenario `perf_saturation_4p`), in ms. Every tick: input and movement 0.45, collision 0.70, projectiles 0.60, melee 0.20, damage and events 0.35, perception 0.55, AI think 0.90, pathfinding 0.80, block ticks 0.05, relight 0.45, sound 0.30, view extraction 0.30 (sum 5.65). On their Canon §2.2 phase ticks: liquids 1.20, gases 0.80, fire 0.40, status pulse 0.35, temperature 1.10, needs 0.10, net snapshot 0.40, hash snapshot 0.15. The canon phases never put two expensive periodic systems on one tick, so the worst tick is `t ≡ 53 (mod 60)` (liquids plus needs) at 6.95 ms and the mean is 6.28 ms.

| ID | Budget |
|---|---|
| `perf.deck.sim.tick_worst` | 8.0 ms, any single tick |
| `perf.deck.sim.tick_mean` | 6.3 ms over 3,600 ticks |
| `perf.deck.snapshot_ms` | 0.15 ms for `snapshot()` |
| `perf.deck.hash_ms` | 4.0 ms on a worker thread |
| `perf.deck.gen.folio_ms` | 150 ms to generate one Folio (R4 §7.4), using 3 worker threads |
| `perf.deck.load.folio_ms` | 1,500 ms from exit choice to the first frame of the new Folio, hidden by the page turn |
| `perf.deck.mem.ram` | 6,144 MiB resident: sim and checkpoints 384, content 192, CPU-side assets 1,408, render CPU 768, audio 256, UI and fonts 128, net 64, mods 256, Scribe 1,024, code and stacks 512, headroom 1,152 |
| `perf.deck.mem.vram` | 2,560 MiB: entity textures 896, block arrays 32, chunk quads 64, field texture 16, render targets 160, shadow maps 48, skinned meshes 384, particles and decals 96, UI atlases 96, LUTs and misc 48, headroom 720 |
| `perf.deck.frame.gpu` | 14.0 ms: shadows 1.4, depth prepass 0.8, voxels 2.6, entities 2.0, viewmodel 0.5, liquids and translucent 1.0, particles 0.9, decals 0.3, post 1.6, AA 0.7, UI 0.5, transition or bleed 0.4, headroom 1.2 (sum 13.9) |
| `perf.deck.net.host_kbps` | 384 kbit/s upstream for 3 clients (R4 §6.4) |
| `perf.ci.bot_tps` | ≥ 6,000 ticks per second per headless bot run on a CI core (100× real time) |

**Deck threads (4 cores, 8 threads):** main (Bevy schedule, input, UI, extraction), render, sim (host only), 3 compute workers (meshing, decode, hashing, worldgen), real-time audio.

**Caps beyond the canon (Canon §4 caps stand):** dynamic lights 64 visible and 4 shadowed, particles 32,768, decals 256, ragdolls 16, cosmetic rigid bodies 256, audio voices 64, remeshes 8 per frame, field uploads 8 chunks per frame.

## 16. Testing strategy

1. **Formula unit tests** from every worked example in every chapter, written before the implementation (§18).
2. **Property tests** (`proptest`): monotonicity and bounds, no overflow over declared ranges, weight tables summing to 10000, `isqrt_u128` equals core `isqrt`, `atan2_bam` within 1 Bam, `deserialize(serialize(x)) == x`.
3. **Golden replays and SyncTest** (§6.6–§6.7): fast set per pull request, full set nightly.
4. **Determinism CI** on five targets (§6.4).
5. **Content fuzzing**: `cargo-fuzz` targets for the JSON loader, save container, replay decoder, packet decoder and mod loader, plus a schema-aware record mutator; 30 minutes per target nightly.
6. **Headless bot playtests at 100×**: up to 4 utility-AI "Pilgrim" bots per run with `EMIT_VIEW` off; 200 full runs nightly. Failure: a panic, a desync between two replicas, or no Folio progress for 36,000 ticks.
7. **Balance Monte Carlo** (`pal balance`): 100,000 keyed-RNG duels of each Folio-1 Calling against the reference standard enemy must give time to kill 2.0 s ± 0.3 s and DPS 30 ± 4.5 HP/s (Canon §17.4); bot-run level and pace statistics are checked against Canon §7.2 and §8.
8. **Client checks**: headless smoke test under 16 random Bevy `shuffle_seed` values, `en-XA` screenshot tests, agent checks through `bevy_brp_mcp`.
9. **Mutation testing** weekly on math, damage, status and RNG modules.

## 17. CI/CD, flags, channels, telemetry

### 17.1 Pipeline

- **Pull request (≤ 25 min):** fmt, `clippy -D warnings`, `cargo deny`, `xtask check-deps`, `float-scan`, `lint-size`, `spec-refs`, schema check, `pal validate`, nextest (sim suite < 60 s), golden fast set, SyncTest, determinism matrix, Linux and Windows client builds.
- **Nightly:** full golden set, generator sweeps (seeds 0–9,999 per genre, doc 12 validators), bot runs, fuzzing, Steam Deck gates on every `perf.*` budget, `dist` builds to the `nightly` branch.
- **Weekly:** mutation testing, coverage.

### 17.2 Release channels

| Channel | Audience | Promotion rule |
|---|---|---|
| `dev` | team, every merge to `main` | CI green |
| `nightly` | internal Steam branch | nightly suite green |
| `playtest` | Steam Playtest app | 3 consecutive green nightlies |
| `beta` | public opt-in Steam branch | 7 days on `playtest` with no P0 and crash-free sessions ≥ 99.0% |
| `stable` | default Steam branch | 7 days on `beta` with crash-free sessions ≥ 99.5% |
| `prev` | previous stable, 30 days | automatic on each stable release (§12.3) |

Versions are `MAJOR.MINOR.PATCH`; `SIM_VERSION` is independent (§12.3).

### 17.3 Feature flags

Flags are records in `content/02/flag/flags.json`: `id`, `kind` (`bool` or `int`), `default`, `channels` (where the value may differ from the default) and `sim_affecting` (if true, the value enters `content_hash`, §11.2). Cargo features choose compiled code per build (`dev_tools`, `steam`, `llm`, `solari`, `dlss`); flags choose behavior at runtime. Doc 15's `juris.` flags use the same loader.

```json
{ "id": "flag.mods.wasm", "kind": "bool", "default": true, "channels": ["dev", "nightly", "playtest", "beta", "stable"], "sim_affecting": true }
```

### 17.4 Telemetry and crash reporting

- Both are **opt-in**. First launch asks for usage statistics (default off) and crash reports (`ask each time` by default, `always`, `never`); settings change them at any time.
- Identity is a random 128-bit install ID, generated locally, never derived from a Steam ID, resettable. No names, chat, IP addresses (dropped at ingest), hardware serials, advertising IDs or third-party SDKs.
- Usage data is aggregate only: session length, run summaries (mode, Folio reached, end cause, party size), `perf.*` percentiles, desync counts. Raw events are deleted after 180 days.
- A crash report holds a minidump, the last 200 log lines and, for a simulation panic, the §6.7 dump with player names removed. Nothing uploads without consent. "Delete my data" requests deletion by install ID. Endpoints are EU-hosted by `palimpsest_services`.

## 18. Coding conventions for AI agents

1. **Size limits:** files ≤ 400 lines excluding tests, functions ≤ 60 lines, ≤ 7 parameters, cognitive complexity ≤ 25 (clippy, `xtask lint-size`).
2. **SPEC references:** every public item and test in math, data, gen, ai and sim carries a doc line matching `SPEC: \d\d §[0-9.]+( step [0-9a-z.]+)?`, for example `/// SPEC: 00 §10.5 step 5`; `xtask spec-refs` checks each names an existing heading.
3. **Errors:** `thiserror` enums at IO, load and network boundaries. Inside the sim a broken invariant panics via `expect("invariant: …")`; `unwrap()` is denied. Binaries may use `anyhow`.
4. **Test-first per formula:** write the spec's worked-example test, see it fail, implement, add a property test. A formula without a worked example is reported as a spec gap, never invented.
5. **No `unsafe`** in math, data, gen, ai, sim, net, bridge; elsewhere each block needs a `// SAFETY:` comment and review.
6. **Newtypes** for IDs and units across module boundaries (`EntityId`, `Id32`, `Ticks`, `Milli`, `Bp`); no wildcard arm on simulation enums.
7. **AGENTS.md** at the root and per crate: commands, forbidden APIs, owning spec chapters.
8. **Task order:** M0 workspace, CI, xtask → M1 math (Canon §21 tests 1–12, §4) → M2 data, schemas, ID registry, content hash → M3 store, scheduler, Sim API, serialization, hash, golden harness, SyncTest → M4 grids, movement, collision → M5 damage and statuses → M6 items, loot, crafting → M7 perception, AI → M8 worldgen → M9 bridge, minimal renderer → M10 netcode → M11 audio, input, UI, localization → M12 platform, saves, telemetry → M13 mods → M14 services → M15 Deck verification and performance. Each milestone ends with its acceptance tests green.

## Exports

All IDs follow Canon §1.1 and sit in namespaces Canon §19 assigns to this chapter. No new ID collides in `fnv1a32` with any of the 539 ID tokens in canon-1.3.

- **`crate.`** (19): `crate.math`, `crate.data`, `crate.gen`, `crate.ai`, `crate.sim`, `crate.net`, `crate.bridge`, `crate.mod_api`, `crate.scribe`, `crate.render`, `crate.audio`, `crate.ui`, `crate.platform`, `crate.bots`, `crate.server`, `crate.client`, `crate.services`, `crate.tools`, `crate.xtask`. `fnv1a32("crate.sim") = 0xdc634881`.
- **`flag.`** (kind, default; `*` = sim-affecting): `flag.render.solari` (bool, false), `flag.render.dlss` (bool, false), `flag.render.ssao` (bool, false), `flag.render.preset` (int 0 deck_battery, 1 deck, 2 medium, 3 high, 4 ultra; default from hardware detection, 1 on Steam Deck), `flag.audio.hrtf` (bool, true), `flag.audio.reflections` (bool, false), `flag.net.transport` (int 0 steam_sdr, 1 renet_udp, 2 in_process; 0), `flag.net.input_redundancy` (int, 8), `flag.sim.hash_every_tick` (bool, false), `flag.sim.synctest_depth` (int, 0), `flag.mods.wasm`* (bool, true), `flag.scribe.enabled` (bool, false), `flag.telemetry.usage` (bool, false), `flag.telemetry.crash_mode` (int 0 ask, 1 always, 2 never; 0), `flag.dev.hot_reload` (bool, false), `flag.dev.tools` (bool, false), `flag.dev.desync_dump` (bool, true), `flag.dev.schedule_shuffle_seed` (int, 0), `flag.platform.steam` (bool, true).
- **`perf.`**: `perf.deck.sim.tick_worst`, `perf.deck.sim.tick_mean`, `perf.deck.snapshot_ms`, `perf.deck.hash_ms`, `perf.deck.gen.folio_ms`, `perf.deck.load.folio_ms`, `perf.deck.mem.ram`, `perf.deck.mem.vram`, `perf.deck.frame.gpu`, `perf.deck.frame.cpu_main`, `perf.deck.frame.cpu_render`, `perf.deck.draw.calls`, `perf.deck.mesh.chunk_us`, `perf.deck.save_ms`, `perf.deck.net.host_kbps`, `perf.ci.bot_tps`.
- **Non-ID interfaces:** `SIM_VERSION`, the Sim API, button bits, `IntentKind`, serialization sections, the `PALS` container and error codes, the quad format, the field texture, `cap_milli`, `ambient_milli`, WIT `palimpsest:mod@1.0.0`, the Fluent key map, `content_hash` and `modset_hash` layouts.

## Interface assumptions

| Doc | Assumed to provide |
|---|---|
| 03 | the component catalog (`comp.*`) that generates `World`; `SCHEDULE` order (`sys.*`); `Chunk` fields and grid propagation; event payload schemas, including per-slot `listener_db: [i32; 4]` and `arrival_dir: [u8; 4]` (26-neighbour index, 255 = direct) on `ev.noise`; the character controller and sweeps built on §4.6 |
| 04 | hitbox pose table records (Canon §9) in integer mm, baked by `pal bake-poses` |
| 07 | the block registry mapping each block skin to a material index `0..=4095` |
| 09 | sound-grid dB semantics; awareness levels named for music intensity (suspicious, search, alert) |
| 12 | the visual and audio GPV blocks (`render_style`, `grade`, `post`, `transition`, music), at most 2 bleed genres per Folio, `local_genre` per block, generator validators |
| 13 | utility tables and A* tie-break rules implemented in `palimpsest_ai` |
| 14 | the `ev.run_ended` reason `abandoned_version` and its rewards; the mode list |
| 15 | service endpoints, receipt format (Ed25519), `juris.` flags |
| 16 | which peers hold full replicas, channels, snapshots, prediction, resync, the lobby handshake |
| 17 | the `input.*` action list and bindings, `ui.*` skins, comfort and accessibility settings |
| 18 | `art.`, `aud.`, `vfx.` records with `trigger` fields, 5 stems per music track, LUTs, per-layer material parameters, Fluent files, launch locales |
| 19 | the schema catalog matching `palimpsest_data` record types |

## Canon gaps

- **G1.** The contract fixes the header at canon-1.1; the canon file is canon-1.3. Checked against it: the field texture carries both light channels, lean bits exist, new events and World Laws need no architecture change, and seam zones render in the Folio genre's skin, so bleed weights there are 0.
- **G2.** Canon §1.9 omits the `atan2_bam` argument convention. Assumed: angle of `(x, y)` counter-clockwise from +x; `yaw_from_dir(dx, dz) = atan2_bam(−dx, −dz)`.
- **G3.** Canon §2.5 says "every 60 ticks" without naming ticks. Assumed: boundaries `b ≡ 0 (mod 60)`, `b ≥ 60`, hashing the state before tick `b` runs.
- **G4.** Canon §2.4 logs only commands. Replays and saves also log `PlayerIntent { tick, slot, action_seq, kind }`; Canon §2.4 should list it.
- **G5.** No canon rule namespaces mod content IDs. Assumed: second segment `x_<mod_id>`, prefix `x_` reserved for mods.
- **G6.** Canon §1.6 does not bound JSON integers. Assumed: `|n| ≤ 2^53 − 1`, larger values as hex strings.
- **G7.** Canon §2.4 gives no `content_hash` byte layout; §11.2 defines it and folds in sim-affecting flags.
- **G8.** Canon §21 test 33 permits a float module inside the sim; this chapter puts it in `palimpsest_bridge` (stricter, compatible).
- **G9.** Telemetry event kinds are enum variants, not content IDs; if IDs are needed, Canon §19 should give doc 02 a `tel.` namespace.
- **G10.** Canon §1.11 IDs are never reused, so the store is non-generational although the computed brief suggested generations; R4's `fixed`, PCG32, RON and xxh3 yield to the canon (§2.3).

## Acceptance tests

1. `isqrt_u128` returns 0, 1, 1, 1, 2, 9, 10, 2^32, 2^64 − 1 for inputs 0, 1, 2, 3, 4, 99, 100, 2^64, 2^128 − 1, and equals core `u128::isqrt` on 100,000 random inputs.
2. `sqrt_fx(Fx::from_int(2)).raw() == 6074000999`; `sqrt_fx(Fx::from_int(50)).raw() == 30370004999`; `sqrt_fx(Fx::from_int(4)) == Fx::from_int(2)`.
3. `mul_fx(Fx(1), Fx(-1)) == Fx(-1)`; `mul_fx(Fx(1), Fx(1)) == Fx(0)`; `div_fx(7, 2)` raw 15032385536; `div_fx(-1, 3)` and `div_fx(1, -3)` raw −1431655766; an `i64` overflow in `Fx + Fx` panics in the `release` profile.
4. BLAKE3 of `SIN_T` as LE `i32` is `d2612a42910edad3b81095ad0be154d6076c5c33771916cbbea2ded40419ec11`; `SIN_T[8192] == 759250125`; `SIN_T[16383] == 1073741819`.
5. For `(y, x)` = (1, 1), (3, 4), (4, 3), (7, −24), (−5, −5), (2, −1), (1, 1000000), (5, 0), (0, −3), (0, 0), `atan2_bam` returns 8192, 6712, 9672, 29808, 40960, 21220, 0, 16384, 32768, 0.
6. Over 1,000,000 pairs drawn as `next_u64() as i64` from `Rng::from_seed64(7)`, `|atan2_bam − exact| ≤ 1 Bam`.
7. `yaw_from_dir` returns 16384, 0, 49152, 8192 for directions (−1, 0), (0, −1), (1, 0), (−1, −1) m; `fwd(16384, 0) == (−2^30, 0, 0)`.
8. `Fx3::length((3, 4, 0) m) == Fx::from_int(5)` exactly.
9. `closest_seg_seg((0,0,0), (2,0,0), (1,−1,1), (1,1,1))` gives `s = t = Fx(2147483648)` and `dist2 = 2^64`.
10. `ray_capsule` from (0, 1.0, 0) along +X against (5, 0.3, 0)–(5, 1.5, 0), r 0.3 m returns raw 20186346293; from height 1.7 m returns raw 20514452597; from height 2.0 m returns `None`.
11. The §4.6 DDA example visits (0,1,0), (0,1,−1), (1,1,−1), (1,1,−2), (2,1,−2), (2,1,−3) at raw t 0, 2684354559, 3579139414, 8053063678, 10737418242, 13421772797.
12. Packing x 3, y 7, z 12, w 4, h 2, face 2, material 517, AO (3,3,2,1), class 0 yields `0x0411B0E3`, `0x0006F205`, and unpacking returns the inputs.
13. `conv`: `fx_to_f32(Fx::from_milli(1620)).to_bits() == 0x3FCF5C29`; `rad_to_bam(π/2) == 16384`; `rad_to_bam(−π/2) == 49152`; `rad_to_bam(30°) == 5461`; `stick_to_i8(0.5) == 64`.
14. The loaded light tables equal the §8.5 `cap_milli` and `ambient_milli` rows; after a chunk with `light_sky = 15` uploads, its texel R reads 255, and a GPU readback of a sample at a cell centre returns exactly `level × 17`.
15. The page curl at `t = 0.5` maps `u = 1.0` to (0.9144, 0.15) and `u = 1.9` to (0.1, 0.3) within 10^−4.
16. The occlusion example (80 dB, 44 dB, 10 m) yields gain −36 dB and cutoff 7,962 Hz ± 1 Hz.
17. The two-record `content_hash` example yields `92c159a0a347070d679d6edd626a8bc6938565f60204dd732ba14c2ac3592cc9`; reordering keys or whitespace in the source files does not change it.
18. `loc.arch.melee.blade_long.skin.wuxia.name` maps to Fluent ID `loc-arch-melee-blade_long-skin-wuxia-name` and back.
19. The validator rejects `1.5`, `1e3`, `-0.0` and `9007199254740992`, and accepts `9007199254740991` and `"0x00000000000000FF"` in a `u64` field.
20. Save container: header length 112; flipping one payload byte gives `payload_corrupt`; flipping header byte 20 gives `header_corrupt`; a different `SIM_VERSION` gives `sim_version_mismatch`.
21. `xtask check-deps` passes on the workspace and fails when `glam` is added to `palimpsest_sim`.
22. `xtask float-scan` fails on `let x = 1.0;` in `palimpsest_sim/src` and passes on the same line inside a `#[cfg(test)]` module.
23. The determinism set yields byte-identical `hashes.json` on all five §6.4 targets.
24. SyncTest depth 8 passes over every fast golden replay; a fixture system storing state outside the serialized sections fails it.
25. A pull request changing a golden hash without touching `SIM_VERSION` or `content/` fails CI.
26. Flipping one bit of a stored sub-attribute in a client's entity column at tick 1,000 is detected at boundary 1,020; both dumps name section 2 and the affected `EntityId`.
27. In a release build with a run active, a content change returns `content_reload_refused_in_run` and `content_hash` is unchanged.
28. A WASM mod looping forever traps at the 1,000,000-fuel limit, is disabled on the same tick on x86_64 and aarch64, and both replicas keep equal state hashes.
29. A WASM mod returning the bits of `0.0 / 0.0` as `u32` returns `0x7FC00000` on every target.
30. Spawning 5 entities then killing the 3rd: after `compact`, `iter` yields the other 4 in ascending `EntityId`; a sixth spawn appends after the fifth.
31. Entity `(3 << 40) | 5` is due for AI think (period 6) on ticks 1, 7 and 13 and not on 0 or 6.
32. With telemetry at defaults, a full session makes zero requests to telemetry endpoints.
33. On `deck-01`, `perf_saturation_4p` keeps every tick ≤ 8.0 ms and the mean ≤ 6.3 ms; meshing takes ≤ 100 µs per chunk; a CI bot run averages ≥ 6,000 ticks/s.
34. With `curr` published 7.67 ms ago, `alpha = 0.46` and an entity moving from x 10.0 m to 10.1 m renders at 10.046 m.
35. A client whose mod set differs from the host's is refused at the handshake with `content_mismatch`.

## Invariants annex

```
Σ 02: stack ⊗ crates ⊗ sim ⊗ fixed-point ⊗ determinism ⊗ bridge ⊗ render ⊗ audio ⊗ content ⊗ saves ⊗ mods ⊗ platforms ⊗ budgets ⊗ tests
W! sim ∈ no_std+alloc ∧ single-thread ∧ forbid(unsafe)          N! bevy|glam|wgpu|wasmtime|rand|fixed ∈ closure(sim)
W! floats := bridge::conv only                                  N! float → sim, except via PlayerCommand integers
W! iteration := ascending EntityId | uid                        N! sort_unstable | HashMap | RefCell ∈ sim
W! Fx ops checked; overflow ⇒ panic ∀ profiles                  N! Mul|Div operator traits for Fx×Fx
W! |atan2_bam − exact| ≤ 1 Bam                                  W! isqrt_u128 == core isqrt
W! state_hash := BLAKE3(12 sections, fixed order)               W! hash boundaries ≡ 0 mod 60 (prod), every tick (CI)
W! golden hash change ⇒ SIM_VERSION|content change             W! 5-target hashes byte-identical
W! SyncTest(8) passes                                          N! state outside serialization
W! render light ≤ cap(max(light_sky, light_block))              N! visual light contradicts stealth light
W! bleed ink line ≤ 0.25 m from sim local_genre boundary        N! render extrapolates sim state
W! content: JSON ∧ schema 2020-12 ∧ |int| ≤ 2^53−1              N! float token ∈ content
W! content_hash ⊇ records ∪ unlocks ∪ mods ∪ sim-flags          N! hot reload mid-run in release
W! save := PALS header ∧ BLAKE3 ∧ atomic rename                 N! resume with sim_version|content_hash mismatch
W! WASM: fuel ∧ NaN canon ∧ no relaxed SIMD ∧ no WASI           N! mod JIT on consoles
W! telemetry & crash upload := opt-in                           N! Steam ID | IP | ad ID ∈ telemetry
W! every pub sim item has SPEC ref                              N! unwrap() ∈ sim
```
