# 02 — Technical Architecture

As of 2026-10-11. Status: BINDING. Conforms to canon-1.11.

This chapter fixes the stack, crate layout, simulation architecture, math library, determinism plan, presentation, pipelines, saves, mods, platforms, budgets, tests and coding rules. Every rule was checked against canon-1.11, including the invader replay slot 4 (Canon §2.4, §17.9), intents in the replay log and `presentation_hash` (Canon §2.4), the hand-written canonical serialization (Canon §2.5), fixed terrain with placeables and no item durability (Canon §4.3), 250 mm micro-voxels (Canon §4.4), the cut of mounts (Canon §17.7) and Margin trophies (Canon §17.8). The Rust workspace `palimpsest/` (crates `palimpsest_math`, `palimpsest_data`, `palimpsest_sim`) is the reference implementation of §3 and §4; where this chapter and that code differ, this chapter governs and the code is changed.

## Imports

| Source | Consumed |
|---|---|
| DESIGN_BRIEF §5, §8 | Rust and Bevy, Bevy-free sim, fixed point, 60 Hz, 32³ chunks, light and sound grids, sim capsules, host-authoritative co-op, data content, WASM mods, on-device language model; jurisdiction flags, offline-complete mode |
| DECISIONS.md items 5, 6, 24, 33, 37, 43, 55, 65, 73, 87, 92, 98, 112, 113, 120, 123 | Rust with Bevy 0.20 over a fixed-point sim crate; the Rust vertical slice; the account Glyph Library in the meta profile (33); 250 mm micro-voxels; JSON data packs plus sandboxed WASM; mounts cut; no item durability; Steamworks lobbies and Steam Networking Sockets with Datagram Relay for play, and our own headless service re-simulating ranked, daily and weekly replays to verify scores (73, 123); `presentation_hash` split from `content_hash`; FOV 60 to 120 degrees, default 90; PC only through 1.0: Windows, Steam Deck through Proton, no console, macOS or Linux client (98, which replaces the console plan of 6); Steam as the only storefront and the EOS SDK dropped, so no Epic Games Store build and no EOS binding (112 and 123, which replace the Epic parts of 73 and 98); no demo and no public playtest build (113); the on-device Scribe model as a 1.0 opt-in, with the template grammar at Early Access (120) |
| Canon §1.1–§1.4, §1.6 | ID grammar and `id32`, the mod ID form `x_<mod_id>`, numeric types, integer and `Fx` primitives, shifts as checked multiplications, integer-only content inside `±(2^53 − 1)`; delegation of `isqrt_u128` |
| Canon §1.8–§1.11 | coordinates, chunk index, `Bam`, sine table, `fwd`; the `atan2_bam` convention and `yaw_from_dir`, with the implementation delegated here; text keys; `EntityId`, ascending iteration, `HashMap` ban |
| Canon §2.1–§2.5 | `TICK_HZ`, `DT`, tick schedule and phases, named state boundaries, session header, replay command record with party slots `0..=3` and invader slot 4, the intent record with doc 02's `IntentKind`, `content_hash` over simulation content (§11.2 layout) and `presentation_hash`, light overrides in every light read, save contract with the invasion deferral, hand-written self-delimiting serialization, draw counters outside state, `state_hash` |
| Canon §3 | xoshiro256++, SplitMix64, `range_incl` in `i128`, sequential and keyed streams, labeled sub-streams, object-keyed draws, offer generators, mod streams, presentation generators (§3.8) |
| Canon §4, §4.3, §4.4, §9 | caps and body dimensions; fixed terrain, `block.tag.destructible`, terrain stopping every projectile, `cat.placeable`, no item durability; 250 mm micro-voxel masks for collision, rendering and destruction on the 1 m simulation grid, triplanar texturing at 64 texels per meter, render-only debris; baked hitbox pose tables |
| Canon §1.2, §2.3, §5.8 | light channels `light_sky`, `light_block`, effective = max; seam zones use Folio skins |
| Canon §13.8, §17.3, §17.4 | power tables, `ITEM_MULT_MAX`, ranks and Folio-1 reference anchors for the balance harness (§16) |
| Canon §17.7, §17.8, §17.9 | retired IDs, which leave command bit 25 unused (§5.5); Margin trophy slots in the meta profile (§12.4); invader join, slot 4 and the opt-in flag (§5.5, §14) |
| Canon §18, §19, §21 | event IDs; namespaces `crate.`, `flag.`, `perf.`; acceptance tests 1–12, 29, 31–34, 45–47 |
| R4 §0–§17 | versions read from crates.io, meshing, light texture, netcode parameters, audio split, modding, test pyramid, agent rules |
| R6 GPV blocks D–F | `render_style`, post stack, `anim_step`, `transition`, music layers (carried by doc 12 and doc 18 records, see below) |
| R7 §1.4 | conflicts C1–C9 belong to docs 05, 06, 08, 09, 11 and 13; none touches this chapter |
| `audit/canon_gaps_triage.md`, `audit/actions_02.md` | rulings G-002 to G-006, G-009 and G-199; conflicts C-10 to C-15 (this chapter changes); C-30 and C-31 (retired mount and durability references) |
| Doc 03 | §2.2 order of the 39 systems and `sys.*` IDs; §3.1–§3.3 identity, kinds and component catalog; §4.1–§4.7 grids, `Chunk` with its four cell bitsets and `fluid_gen`, `BlockState`, shapes, block registry with `skins`; §4.3–§4.4 micro-voxel masks: `shape_mask` per state, `mask_at(c)`, each chunk's custom masks (`masks` and `mask_of`) and a `block_gen` bump on every mask change; §5.18 `raycast_blocks` passing `max_steps = 3 × ceil_div(t_max.raw, 2^32) + 3` to `dda`; §6.1, §7.1, §8.1 liquid, gas and fire cells (renderer view); §10.1 light nibbles; §10.6 light overrides (box `lo..=hi`, optional sphere, `max_level`) and the dimming by 8 inside doc 06's shadow areas; §11.3–§11.5 and §14.2 `ev.noise` with `heard: [Option<(Db, Fx3)>; 5]` for slots `0..=4`; §16.1 section contents; §17 unit costs registered here as `perf.deck.sim.unit.*` |
| Doc 05 | §2.2 `seg_seg_params`, which is this chapter's `closest_seg_seg` (§4.6) with `div128`, `div256` and `I256`; bearings `atan2_bam(-dx, -dz)`; `isqrt_u64` and the `mm(Fx)` conversion (§4.1, §4.2) |
| Doc 12 | §3.1 GPV block B fields `palette`, `render_style`, `music` (presentation); §6.3 generation budgets p50 110 ms, p99 260 ms, hard cap 900 ms on 4 threads; §13 bleed plan: one entry genre plus up to 3 drift genres per Folio; `generate_folio` signature |
| Doc 13 | §4.4 table curves generated by `tools/gen_curves.py`; §4.13 A* tie-break; §4.14 host AI budget of 1.5 ms per tick; §11.2 pacing model reproduced by the bots harness |
| Doc 14 | §4.3 `run.end.abandoned_version` applied from the save's checkpoint record; §9.4 replay verification; §12 write points, suspend rules, ranked `save_seq` |
| Doc 15 | §2.2 Apocrypha rule (simulation-affecting mods earn no account rewards); §4.6 receipts (Ed25519, canonical JSON, key windows); §8.2 `juris.` records with compile-time false `juris.paid_random_items_enabled`; §10.1 service components; in-house ChaCha20 in `palimpsest_econ` |
| Doc 09 | §6.1 awareness ordinals 0 to 5 (`aware.unaware`, `aware.curious`, `aware.suspicious`, `aware.searching`, `aware.lost`, `aware.alert`) for the music level; `StealthWorld`, saved as section 12 `stealth`; conversion of footsteps, landings and actions into `ev.noise` |
| Doc 16 | §3.1 full `Transport` contract; §5.7 intent kinds and `SessionIntent`; §6.14 command-and-intent log (CIL); §10.1 `ReplayFile`; §15 host upstream split; requested flags `flag.net.shadow`, `flag.net.voice`, `flag.net.crossplay`, `flag.net.netsim` |
| Docs 04, 06, 07, 08, 09, 11, 14, 17 | the intent kinds of §5.5 with their payloads, each validated by its owner; doc 06 §13.1's `in_shadow_area` for `ViewFrame.shadow_cells`; doc 17 §12.2's command fields `ACTION` (bits 26–28) and `WHEEL_ENTRY` (bits 29–31), bit 25 always 0; doc 17 §8.4's FOV setting 60–120°; doc 17 §13.6's screen reader and `access.narrator` |
| Docs 01, 18, 19, 21 | doc 01 §3.13 signature-moment scenarios, §10.3 per-Folio KPI fields and its three bot policies; doc 18 §10.2 block layers at 64 × 64 px, §6.6 voice pools, the reveal and dark-vision presentation, `art.*` asset IDs; doc 19 §3.6 `x-patchable`, §4 validation stages and codes, the `02.patch` schema; doc 21 §3.5 task-order mapping and §4.1 module paths |

## 1. Architectural decisions

1. **One authority.** Only `Sim::step`, `Sim::restore` and `Sim::deserialize` write game state. Everything else reads output and sends inputs.
2. **The simulation is integer-only, single-threaded, `#![no_std]` with `alloc`.** `no_std` removes clocks, threads, files, environment and `HashMap` by construction. In-sim parallelism is limited to pure maps merged in index order (§5.3).
3. **Floats begin after one boundary,** the module `palimpsest_bridge::conv` (§7.5). `palimpsest_sim` has no float exception at all, stricter than Canon §21 test 33.
4. **Content is integer-only JSON** validated by JSON Schema 2020-12 (Canon §1.6; the brief also allowed RON).
5. **Pin everything:** exact versions, committed `Cargo.lock`, `=` pins for Bevy-family crates.
6. **Headless first:** every crate except the Bevy plugins builds and tests without window, GPU or audio device.
7. **Offline-complete:** every gameplay system runs without the backend (brief §8).
8. **Own the thin layers:** where no ecosystem crate supports the pinned Bevy, the project uses a Bevy-agnostic crate or in-house code behind a thin plugin (§2.4). No milestone waits on an ecosystem release.

## 2. Technology stack

### 2.1 Toolchain

| Item | Value | Justification |
|---|---|---|
| Rust | `1.99.0` stable, pinned in `rust-toolchain.toml` | Current stable on 2026-10-11 (R4 §0). Above Bevy 0.20's MSRV 1.97.1 and wasmtime 49's MSRV 1.96.0. It installs cleanly through `rust-toolchain.toml`, as the Rust slice's CI shows. |
| Edition | 2024 | Current edition; resolver 3 by default. |
| Components | `rustfmt`, `clippy` in `rust-toolchain.toml`; the coverage job adds `llvm-tools` with `rustup component add llvm-tools` | Formatting and linting everywhere; coverage only in the weekly job (§17.1). |
| Targets | `x86_64-pc-windows-msvc`: the game client and every binary; `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu`: the headless binaries (`palimpsest_server`, `palimpsest_services`, `palimpsest_bots`, `palimpsest_tools`, `xtask`) and CI; `aarch64-apple-darwin`: the determinism CI job only; `wasm32-wasip1`: the WASI determinism runner (§6.4); `wasm32-unknown-unknown`: WASM mod guests (§13) | DECISIONS.md items 98 and 112: through 1.0 the only shipped client is Windows x86_64 on Steam, and Steam Deck runs it under Proton. No console, macOS, native Linux or browser client is built. |
| Linux baseline | Ubuntu 24.04 (the CI runner image) for the headless binaries | The backend and the CI bots run them; no player installs a Linux build. |

### 2.2 Crates

Versions were read from crates.io on 2026-10-11 (R4 §16, re-checked for crates R4 omits). Every version below is the exact pin written into `[workspace.dependencies]`.

| Crate | Version | Purpose and justification |
|---|---|---|
| `bevy` | `=0.20.0` | Engine for rendering, input, UI, assets. 0.20 adds schedule randomization, panic-to-error and mesh-shader groundwork (R4 §16.3). |
| `bevy_egui` | `=0.43.1` | Debug UI in the `dev_tools` feature only. |
| `bevy_brp_mcp`, `bevy_brp_extras` | `=0.23.0` | `dev_tools`: lets a coding agent drive, query and screenshot the client. |
| `firewheel` | `=0.14.0` | Audio graph engine, Bevy-agnostic, driven by `palimpsest_audio` through a thin in-house plugin (§9). `bevy_seedling` 0.8.0 targets Bevy 0.19 only, so the project skips that wrapper instead of waiting for it. |
| `cpal` | via `firewheel` | Output backend: WASAPI, which Proton maps to the host audio server on Steam Deck. |
| `audionimbus` | `0.16.0` (`auto-install`; its `bevy` feature off) | Steam Audio 4.8.1 HRTF and reflections, Apache-2.0, called from the audio thread. |
| `bevy_rapier3d` | `=0.37.0` (Rapier `0.36.1`) | Cosmetic physics only: ragdolls, debris, loot bounces. Released 2026-10-10 for Bevy 0.20; `avian3d` 0.7.0 targets Bevy 0.19, so it is not used. |
| GPU particles | in-house `palimpsest_render::particles` (§8.7) | `bevy_hanabi` 0.19.0 targets Bevy 0.19, and the effect set (sparks, smoke, ink, blood, motes) needs one compute pass and one billboard pass. |
| `ogg`, `opus-decoder` | `0.9.2`, `0.1.1` | Pure-Rust Opus decoding (Symphonia 0.6.1 has no Opus); gated by RFC 8251 vectors in CI. |
| `opus` | `0.4.0` (libopus through `opusic-sys`, built from its bundled source) | Voice encoding in `palimpsest_audio`: 20 ms Opus frames at 24 kbit/s in every build (doc 16 §9.12); `opus-decoder` decodes them. |
| `fluent-bundle`, `unic-langid`, `fluent-langneg` | `0.16.0`, `0.9.6`, `0.14.2` | Localization with plural and gender selectors. |
| `arc-swap` | `1.9.2` | Lock-free view-frame mailbox. |
| `steamworks` | `0.13.1` | Steam build (feature `steam` of `palimpsest_platform`): lobbies, Datagram Relay, achievements, Cloud, Workshop, Input; own Bevy glue. |
| `crash-handler`, `minidumper` | `0.8.1`, `0.11.0` | Out-of-process minidumps, no third-party SDK. |
| `llama-cpp-2` | `0.1.159` | On-device flavor text, off by default (brief §5). It ships at 1.0 as an opt-in; Early Access builds leave feature `llm` off and use the template grammar (DECISIONS.md item 120). |
| `tts` | `0.26.3` | `access.narrator` speech through the Windows speech API (doc 17 §13.6). When the OS reports no voice, Settings shows the option as unavailable. Screen readers use Bevy's AccessKit integration (`bevy_a11y`), which `bevy_winit` provides on Windows without a feature flag. |
| `serde`, `serde_json`, `postcard` | `1.0.229`, `1.0.154`, `1.1.3` | Content JSON (`serde_json`); network messages, the command-and-intent log and container payload records (`postcard`, which has a documented stable wire format and `no_std`; bincode is unmaintained, R4 §12). The canonical simulation serialization (§6.5) is hand-written and uses neither (Canon §2.5). |
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
| `ed25519-dalek`, `sha2`, `hmac` | `3.0.0`, `0.11.0`, `0.13.0` | Offline receipt verification and local draw verification in `palimpsest_econ`; commit-reveal on the backend (doc 15 §4.6). All three build `no_std`. |
| `axum`, `tokio`, `sqlx` | `0.8.9`, `1.53.2`, `0.9.0` | Backend on PostgreSQL 18. |
| `proptest`, `insta`, `criterion`, `arbitrary`, `libfuzzer-sys` | `1.12.0`, `1.49.0`, `0.8.2`, `1.5.0`, `0.4.13` | Property, snapshot, benchmark, fuzz tests. |
| `binary-greedy-meshing` | `0.5.2` | Dev-dependency: oracle for the in-house mesher's face counts. |
| CLI tools | `cargo-nextest 0.9.148`, `cargo-mutants 27.1.0`, `cargo-fuzz 0.13.2`, `cargo-deny 0.20.2`, `cargo-llvm-cov 0.9.1`, `wasmtime-cli 49.0.2` | CI; the wasmtime CLI runs the `wasm32-wasip1` determinism job (§6.4). |

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
| `indexmap` ordered maps in the sim | `BTreeMap` and sorted `Vec` | Canon §1.11; insertion-ordered maps make equal states serialize differently after different histories |
| `rkyv` zero-copy baked tables | committed Rust source (`SIN_T`, `ATAN_T`) and JSON content decoded once at load | The tables are under 1 MiB; fewer binary formats are simpler to keep deterministic |
| postcard for saves and chunk blobs (R4 §12) | the hand-written canonical serialization (§6.5) for simulation state; postcard only for messages and container payload records | Canon §2.5: the canonical serialization is hand-written, self-delimiting and never depends on serde |
| `avian3d`, `bevy_seedling`, `bevy_hanabi`, `bevy_enhanced_input` (no Bevy 0.20 release on 2026-10-11) | `bevy_rapier3d =0.37.0`; `firewheel =0.14.0` with own glue; in-house particles; in-house action layer | Decision 8; no milestone waits on a port |
| `bevy-steamworks` | own thin glue over `steamworks` 0.13.1 | R4 §16.1 |
| `lightyear`, `bevy_replicon`, `ggrs` | own netcode in `palimpsest_net` (doc 16) | R4 §6.2: they replicate Bevy ECS components, while all authority lives in the Bevy-free sim |

### 2.4 Ecosystem rule

- Bevy stays at `=0.20.0` for the whole 0.20 cycle.
- No dependency is added whose newest release requires a Bevy version other than the pin.
- The only Bevy-coupled third-party crates are `bevy_egui`, `bevy_brp_mcp`, `bevy_brp_extras` and `bevy_rapier3d`. Audio, Steam, input, particles and networking use Bevy-agnostic crates or in-house code behind thin plugins.
- Migrating to Bevy 0.21 therefore touches only `palimpsest_render`, `palimpsest_audio`, `palimpsest_ui` and `palimpsest_client`, plus the four coupled crates' pins.
- Replacing a crate (for example `bevy_rapier3d` by `avian3d` once `avian3d` supports the pinned Bevy) is an ordinary pull request that must keep every client acceptance test green.

### 2.5 Bevy feature set

`bevy` uses `default-features = false` with `3d`, `ui`, `bevy_gltf`, `ktx2`, `zstd_rust`, `png`, `tonemapping_luts`, `smaa_luts`, `pbr_clustered_decals`, `multi_threaded`, `keyboard`, `mouse`, `gamepad`. Dev adds `file_watcher`, `bevy_remote`, `hotpatching`; profiling adds `trace_tracy`; CI adds `debug` (for `shuffle_seed`) and `bevy_ci_testing`; Ultra builds add `bevy_solari`, `dlss`. The client is Windows-only (DECISIONS.md item 98), so no `x11`, `wayland` or `webgpu` feature is enabled. AccessKit comes with `bevy_winit` (doc 17 §13.6). `bevy_audio` stays off; `palimpsest_audio` drives Firewheel directly.

## 3. Cargo workspace

### 3.1 Layout

```
palimpsest/
  Cargo.toml              workspace, [workspace.dependencies] with exact pins, [workspace.lints], profiles (§6.3)
  Cargo.lock              committed
  rust-toolchain.toml     channel = "1.99.0", components rustfmt and clippy
  clippy.toml             workspace defaults (§6.2)
  deny.toml               cargo-deny bans and licenses
  README.md, AGENTS.md    overview; build, test and invariant summary for coding agents (§18)
  crates/<package>/       one directory per crate of §3.2, named by its package (crates/palimpsest_math/), each with its own AGENTS.md
  content/<doc>/<ns>/     JSON content, grouped by owning spec document (§11.1)
  schemas/                generated JSON Schema 2020-12 files, committed
  assets/                 glTF, KTX2, Ogg/Opus, fonts, Fluent files (§11.4)
  tests/golden/           golden replays *.palr (§6.6)
  tests/moments/          doc 01 §3.13 signature-moment scenarios sm_NN.json (§16)
  tools/gen_trig.py       writes crates/palimpsest_math/src/trig_table.rs (SIN_T, Canon §1.9) and
                          crates/palimpsest_math/src/cordic_table.rs (ATAN_T, §4.4), mpmath at 50 digits; --check verifies
  tools/gen_curves.py     generator of doc 13 §4.4 table-curve knots (§11.5); --check verifies
  tools/gen_namespaces.py writes content/00/meta/namespaces.json from Canon §19 and §17.7 (doc 19)
  tools/validate_content.py  doc 19's Python reference validator; `pal validate` matches it case for case
  xtask/                  workspace lints and automation
```

### 3.2 Crates

Layer numbers define the dependency DAG. A crate may depend only on crates in its allow list, all of which sit in lower layers.

| ID | Package | Layer | Kind | `no_std` | Responsibility |
|---|---|---|---|---|---|
| `crate.math` | `palimpsest_math` | 0 | lib | yes | integer primitives, `I256`, `Fx`, `Fx3`, `Bam`, trig, CORDIC, `isqrt_u128`, geometry, RNG, seeds, FNV-1a, BLAKE3 wrapper, power tables |
| `crate.data` | `palimpsest_data` | 1 | lib | yes (core); `loader` feature uses std | content IDs and `Id32`, ID registry, text keys, hex `u64`, content record types, `ContentDb`, schema generation, loader, `content_hash` and `presentation_hash` |
| `crate.econ` | `palimpsest_econ` | 1 | lib | yes | doc 15's shared economy logic: in-house ChaCha20 (RFC 8439), HMAC-SHA256 draw derivation, commit-reveal verification, odds and pity math, receipt canonical JSON and Ed25519 verification, `FairStream` and doc 15's pull, open and Transcription algorithms, `PAID_RANDOM_ITEMS_ENABLED = false`; used by the client (offline receipts, the local verify button) and by `palimpsest_services`; never reads simulation state |
| `crate.gen` | `palimpsest_gen` | 2 | lib | yes | Folio generation as a pure function (doc 12) |
| `crate.ai` | `palimpsest_ai` | 2 | lib | yes | utility scoring, response curves, A* and HPA* (doc 13) |
| `crate.sim` | `palimpsest_sim` | 3 | lib | yes | world store, scheduler, every simulation system, Sim API, canonical serialization, view extraction |
| `crate.net` | `palimpsest_net` | 4 | lib | no | `Transport` trait, protocol framing, input buffers, prediction and reconciliation (doc 16), desync detection (§6.7) |
| `crate.bridge` | `palimpsest_bridge` | 4 | lib | no | view-frame mailbox, interpolation, entity mapping data, event queue, the float conversion boundary `conv` |
| `crate.mod_api` | `palimpsest_mod_api` | 4 | lib | no | WIT world, wasmtime host (`host` feature) implementing `ScriptHost`, guest SDK (`guest` feature) |
| `crate.scribe` | `palimpsest_scribe` | 4 | lib | no | on-device flavor-text jobs (`flag.scribe.enabled`, default off), template fallback; never linked into the simulation |
| `crate.render` | `palimpsest_render` | 5 | lib | no | Bevy plugin: voxel meshing, materials, lighting, viewmodel, post, particles, decals, transitions |
| `crate.audio` | `palimpsest_audio` | 5 | lib | no | Bevy plugin: voices, spatialization, occlusion, music stems |
| `crate.ui` | `palimpsest_ui` | 5 | lib | no | Bevy plugin: input action layer, HUD, menus, widgets, screen-reader nodes and `access.narrator` speech, localization, dev overlay |
| `crate.platform` | `palimpsest_platform` | 5 | lib | no | `Platform` trait, Steam (feature `steam`) and Null backends, save IO, telemetry, crash upload, receipt cache, `meta` (doc 14's meta-progression rules, headless-testable) |
| `crate.bots` | `palimpsest_bots` | 5 | lib + bin | no | headless bot players, batch runner, balance Monte Carlo harness |
| `crate.server` | `palimpsest_server` | 5 | lib + bin | no | headless dedicated host, replay runner, and the verifier that re-simulates ranked, daily and weekly replays to verify their scores (DECISIONS.md item 123) |
| `crate.client` | `palimpsest_client` | 6 | bin | no | the game executable: assembles plugins, owns the sim thread |
| `crate.services` | `palimpsest_services` | 6 | bin | no | backend: economy endpoints (doc 15), the ranked, daily and weekly verification queue, which calls `palimpsest_server`'s `verify_replay`, telemetry and crash ingest |
| `crate.tools` | `palimpsest_tools` | 6 | bins `pal`, `palimpsest-verify` | no | validate, bake, schema export, golden bless, desync diff, generator sweeps, trig check, content extraction and spec sync, catalog, odds, scenarios, balance; doc 15 §4.3's `palimpsest-verify`, a second binary here because its `--content` check regenerates Keepsakes through doc 11's item generator in `palimpsest_sim`, which `palimpsest_econ` may not link |
| `crate.xtask` | `xtask` | 6 | bin | no | dependency DAG check, float scan, size limits, SPEC reference check |

**Modules.** Module paths are part of the contract: doc 21 §4.1 cites them in its work packages, and every public item's SPEC line (§18) names its section.

| Crate | Modules |
|---|---|
| `palimpsest_math` | `int` (Canon §1.3 primitives, `I256`), `fx` (`Fx`, `isqrt_u128`, `isqrt_u64`, `sqrt_fx`), `trig` with the generated `trig_table` (`SIN_T`, `sin_bam`, `cos_bam`, `fwd`), `cordic` with the generated `cordic_table` (`ATAN_T`, `atan2_bam`, `yaw_from_dir`, `pitch_from_dir`), `geom` (`Fx3`, `IVec3`, capsules, `div128`, `div256`, `closest_seg_seg`, rays, `VoxelDda`, `dda`), `hash` (`fnv1a32`, `fnv1a64`, `id32`, `blake3_256`, `blake3_fields`), `rng` (Canon §3), `seed` (Canon §3.6), `tables` (Canon §13.8) |
| `palimpsest_data` | `ids` (content-ID grammar, `Id32`, mod segments), `registry` (collision-checked registry with aliases), `text_key`, `hex` (`"0x"` `u64` strings), `content_check` (feature `loader`; the §11.1 token pre-pass), `db` (`ContentDb`), `loader` (feature `loader`; file loading, the §11.1 validation stages, hot reload), `schema` (feature `schema`; JSON Schema export), `mods` (§13: mod folders, classification, patches, `mod_blob`, `modset_hash`), `hash` (`content_hash`, `presentation_hash`, §11.2). The slice holds `ids`, `registry`, `text_key`, `hex` and `content_check`; doc 21 WP-005, WP-006 and WP-077 add `schema`, `loader` and `mods`. |
| `palimpsest_sim` | `api` (Sim API, `StepOutput`, debug draw counters, §4.7), `serial` (§6.5), `store` (§5.2), `schedule` (§5.3), `world`, `controller`, `grids::{liquid, gas, fire, thermal, light, sound}`, `explode`, `edit`, `events`, `combat::{melee, ranged, armor}`, `character`, `body`, `magic`, `craft`, `alchemy`, `stealth`, `stealth::security`, `items`, `loot`, `ident`, `ai`, `enemies`, `enemies::boss`, `run`, `genre`. The slice modules `time`, `entity`, `genre` and `contradiction` stay; `movement` moves into `controller`, `damage` and `status` into `combat`, `stats` and `progression` into `character`, the slice `loot` into `items`, each move keeping its tests (doc 21 §4.1); the slice `sim` module splits into `api` and `serial` (doc 21 WP-010). |

### 3.3 Dependency rules

| Crate | Allowed workspace dependencies | Allowed external dependencies | Forbidden anywhere in its dependency closure |
|---|---|---|---|
| math | none | `blake3` (no default features); `serde` only through the off-by-default feature `serde`, which `palimpsest_sim`'s own `serde` feature and `palimpsest_tools` enable (§4.7) | everything else |
| data | math | `blake3`, `smallvec`; `serde` + `serde_json` + `jsonschema` + `notify` only through feature `loader`, `schemars` only through feature `schema` (record types derive `Deserialize` and `JsonSchema` under those features), so the default closure that `palimpsest_sim` sees holds no `serde` | `bevy*`, `glam`, `wgpu*`, `wasmtime*`, `fixed`, `rand*`, `postcard` |
| econ | math | `serde`, `sha2`, `hmac`, `ed25519-dalek` (all `default-features = false`) | `bevy*`, `palimpsest_sim`, `rand*`, `getrandom` |
| gen, ai | math, data | `smallvec` | same as data, plus `serde_json`, `jsonschema` |
| sim | math, data, gen, ai | `smallvec`, `bitflags`, `blake3`; `serde` only through the off-by-default feature `serde`, which derives `Serialize` and `Deserialize` on `PlayerCommand`, `PlayerIntent`, `IntentKind`, `SimEvent` and the view types for the crates that send them; the canonical serialization (§6.5) never calls it | `bevy*`, `glam`, `wgpu*`, `winit`, `wasmtime*`, `cranelift*`, `fixed`, `rand*`, `getrandom`, `rayon`, `tokio`, `steamworks`, `llama*`, `postcard` |
| net | math, data, sim (feature `serde`) | `serde`, `postcard`, `crossbeam-channel`, `renet` (feature), `thiserror` | `bevy*`, `wgpu*` |
| bridge | math, sim | `arc-swap`, `crossbeam-channel` | `bevy*`, `wgpu*` |
| mod_api | math, data, sim | `wasmtime` (feature `host`), `wit-bindgen` (feature `guest`) | `bevy*` |
| scribe | data | `llama-cpp-2` (feature `llm`), `serde_json` | `palimpsest_sim` |
| render, audio, ui | math, data, bridge | Bevy family, crates named in §2.2 for that crate | `palimpsest_sim` as a direct dependency (only through bridge re-exports), `palimpsest_net` |
| platform | math, data, net, econ | `steamworks` (feature `steam`), `directories`, `tempfile`, `ureq`, `rustls`, `ed25519-dalek`, `zstd`, `blake3`, `crash-handler`, `minidumper` | `bevy*` |
| bots, server | math, data, ai, sim, net, mod_api | `rayon` (bots), `clap`, `anyhow` | `bevy*`, `wgpu*` |
| client | every layer 0–5 crate | Bevy family | none beyond the above |
| services | math, data, econ, sim, server | `axum`, `tokio`, `sqlx`, `sha2`, `hmac`, `ed25519-dalek`, `ureq` | `bevy*`, `wgpu*` |
| tools | math, data, econ (for `pal odds` and `palimpsest-verify`, doc 15), gen, sim, bots, server | `clap`, `anyhow`, `rayon`, `dot_vox` | `bevy*` |

The rules live in content file `content/02/crate/crates.json`, one record per crate. For each package, `cargo xtask check-deps` runs `cargo tree -p <package> -e normal,build --target all --prefix none --format {p}` with default features, which resolves features for that package alone (resolver 3), so a `loader` feature enabled by the client never leaks into the closure checked for `palimpsest_sim`, and `serde` never appears in the default closure of `palimpsest_math` or `palimpsest_sim`. It then repeats the run with `--features <f>` for each key `f` of `feature_external` and allows those crates in that run only. Allow lists name direct dependencies; their own transitive dependencies are allowed unless a forbid entry matches. It fails on any direct dependency outside the allow lists and on any closure member matching a forbid entry. A name ending in `*` is a prefix match.

```json
{
  "id": "crate.sim",
  "package": "palimpsest_sim",
  "layer": 3,
  "no_std": true,
  "allow": ["crate.math", "crate.data", "crate.gen", "crate.ai"],
  "allow_external": ["smallvec", "bitflags", "blake3"],
  "feature_external": { "serde": ["serde"] },
  "forbid_closure": ["bevy*", "glam", "wgpu*", "winit", "wasmtime*", "cranelift*", "fixed", "rand*", "getrandom", "rayon", "tokio", "steamworks", "llama*", "postcard"]
}
```

### 3.4 Public API sketches

```rust
// palimpsest_math — §4
pub struct Fx(i64);  pub struct Fx3 { pub x: Fx, pub y: Fx, pub z: Fx }  pub type Bam = u16;
pub fn floor_div(a: i64, b: i64) -> i64;  pub fn mul_bp(x: i64, bp: i32) -> i64;      // Canon §1.3
pub fn isqrt_u128(n: u128) -> u128;  pub fn sqrt_fx(x: Fx) -> Fx;
pub fn isqrt_u64(n: u64) -> u64;  pub fn sin_bam(a: Bam) -> i32;  pub fn cos_bam(a: Bam) -> i32;
pub fn atan2_bam(y: i64, x: i64) -> Bam;  pub fn yaw_from_dir(dx: Fx, dz: Fx) -> Bam;  pub fn pitch_from_dir(h: Fx, dy: Fx) -> i16;
pub mod geom;  pub mod rng;  pub fn fnv1a32(b: &[u8]) -> u32;  pub fn blake3_256(parts: &[&[u8]]) -> [u8; 32];
pub fn blake3_fields(fields: &[&[u8]]) -> [u8; 32];             // each field prefixed by u32le(len), Canon §2.5
// palimpsest_data — §11
pub struct Id32(pub u32);
impl ContentDb { pub fn get<T: Record>(&self, id: Id32) -> Option<&T>; pub fn id_of(&self, s: &str) -> Option<Id32>;
                 pub fn content_hash(&self) -> [u8; 32]; pub fn presentation_hash(&self) -> [u8; 32]; }
#[cfg(feature = "loader")] pub fn load_content(roots: &[ContentRoot], unlocks: &[Id32]) -> Result<ContentDb, LoadReport>;
// palimpsest_gen, palimpsest_ai
pub fn generate_folio(input: &GenInput, db: &ContentDb, par: &dyn OrderedMap) -> FolioBlueprint;
pub fn score_actions(view: &AiView, table: &UtilityTable, out: &mut ScoredActions);
pub fn astar(grid: &dyn NavGrid, from: CellIdx, to: CellIdx, budget: u32) -> PathResult;
// palimpsest_sim — §5.5
pub trait ScriptHost { fn call(&mut self, hook: HookCall, q: &mut CommandQueue) -> HookResult; }
pub trait OrderedMap { fn map(&self, n: u32, f: &(dyn Fn(u32) -> Vec<u8> + Sync)) -> Vec<Vec<u8>>; }
// palimpsest_net — doc 16 §3.1 owns the full contract; this is its shape
pub trait Transport { fn send(&mut self, peer: PeerId, ch: Channel, bytes: &[u8]);
                      fn recv(&mut self) -> Option<(PeerId, Channel, Vec<u8>)>; }
// palimpsest_bridge — §7
impl ViewMailbox { pub fn publish(&self, out: ViewOut, events: Vec<SimEvent>, tick: u64); pub fn pair(&self) -> Arc<FramePair>;
                   pub fn chunk(&self, slot: u8) -> Arc<ChunkView>; pub fn drain(&self) -> Vec<ViewDelta>; }   // §7.1's three paths
// palimpsest_platform — §12, §14, §17.4
pub trait Platform { fn storage(&self) -> &dyn SaveStorage; fn achievements(&mut self) -> &mut dyn Achievements;
    fn transport(&mut self) -> Option<Box<dyn Transport>>; fn text_input(&mut self, r: TextInputRequest); fn is_handheld(&self) -> bool; }
pub mod meta;   // doc 14 meta-progression rules (unlocks, Bindery, Codex knowledge, achievements), no Bevy, headless-tested
// palimpsest_server, palimpsest_bots — §16
pub fn verify_replay(replay: &ReplayFile, db: &ContentDb) -> Result<VerifyReport, VerifyError>;   // `palimpsest_server host | verify | replay`
pub trait BotBrain { fn command(&mut self, obs: &BotObservation) -> PlayerCommand; }
pub fn run_batch(cfg: &BatchConfig, brains: &dyn Fn(u8) -> Box<dyn BotBrain>) -> BatchReport;   // §16 items 6–7
// palimpsest_econ — doc 15 owns behavior
pub const PAID_RANDOM_ITEMS_ENABLED: bool = false;             // doc 15 SEP-5; the flag loader rejects true
pub fn chacha20_block(key: &[u8; 32], counter: u32, nonce: &[u8; 12]) -> [u8; 64];   // RFC 8439 §2.3
pub fn verify_receipt(r: &Receipt, keys: &[PublicKeyWindow]) -> Result<(), ReceiptError>;
pub fn draw_u64(server_seed: &[u8; 32], client_seed: &[u8], k: u64) -> u64;           // HMAC-SHA256, first 8 bytes
// palimpsest_mod_api — §13
impl WasmScriptHost { pub fn load(mods: &[ModPackage], cfg: &WasmLimits) -> Result<Self, ModLoadError>; }
impl ScriptHost for WasmScriptHost { /* §13.2 */ }
// palimpsest_scribe — brief §5, R4 §8.6
pub fn submit(job: ScribeJob) -> JobId;  pub fn poll(id: JobId) -> Option<ScribeText>;   // template fallback on any failure
// palimpsest_render, palimpsest_audio, palimpsest_ui — Bevy plugins
pub struct RenderPlugin { pub preset: RenderPreset }  pub struct AudioPlugin;  pub struct UiPlugin;
pub fn build_command(look: &LookState, actions: &ActionState, tick: u64, slot: u8) -> PlayerCommand;   // §10
// palimpsest_client, palimpsest_services, palimpsest_tools — binaries
fn main() -> anyhow::Result<()>;   // client: SimThread + bridge + plugins; services: axum router; tools: `pal <subcommand>`
```

`pal` subcommands: `validate`, `schema export`, `bake-poses`, `import-vox`, `golden verify`, `golden bless`, `desync-diff`, `gen sweep`, `balance`, `trig check`, `curves gen`, `perf report`, `extract` and `spec-sync` (doc 21 WP-079), `catalog` (doc 21 WP-080), `odds` (doc 15), `scenario` (§16).

## 4. Fixed-point math library (`palimpsest_math`)

### 4.1 `Fx` and overflow policy

```rust
/// 2^32. Every "<< 32" of the canon formulas is written in this chapter's code as a checked
/// multiplication by this constant (Canon §1.3 shifts).
pub const TWO_POW_32: i128 = 4_294_967_296;
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]   // tools only (§4.7)
#[repr(transparent)]
pub struct Fx(i64);                  // raw r represents r / 2^32 (Canon §1.4)
impl Fx {
    pub const ZERO: Fx = Fx(0);
    pub const ONE: Fx = Fx(4_294_967_296);        // 2^32
    pub const HALF: Fx = Fx(2_147_483_648);       // 2^31
    pub const DT: Fx = Fx(71_582_788);            // Canon §2.1
    pub const MAX: Fx = Fx(i64::MAX);
    pub const fn raw(self) -> i64;
    pub const fn from_raw(r: i64) -> Fx;
    pub fn from_int(n: i32) -> Fx;                // Canon §1.4: Fx(n as i64 × 2^32); |n| < 2^31 cannot overflow
    pub fn from_milli(m: i64) -> Fx;              // = fx_from_milli, Canon §1.4
    pub fn mul_int(self, n: i64) -> Fx;           // checked raw * n
    pub fn div_int(self, n: i64) -> Fx;           // floor_div(raw, n), n > 0
    pub fn floor_block(self) -> i32;              // = fx_floor_block
    pub fn to_milli_floor(self) -> i64;           // floor_div(raw as i128 * 1000, 2^32), checked; doc 03's to_mm, doc 05's mm
}
impl Add, Sub, Neg, AddAssign, SubAssign for Fx;  // checked; panic on overflow
pub fn mul_fx(a: Fx, b: Fx) -> Fx;  pub fn div_fx(a: Fx, b: Fx) -> Fx;  pub fn mul_bp_fx(x: Fx, bp: i32) -> Fx;
```

- `Mul` and `Div` operator traits are not implemented for `Fx × Fx`, so every product names its flooring through `mul_fx` or `div_fx`.
- **Overflow policy.** Every operation is checked. Overflow panics in every profile (Canon §1.3). The panic is deterministic, so every peer panics on the same tick. The sim thread wraps `step` in `catch_unwind` at the binary boundary, writes a crash dump containing the last checkpoint and input log (§6.7), and ends the session. Saturating arithmetic is used only where a formula says clamp.
- **Shifts.** Rust's `<<` is not covered by overflow checks, so the code of this chapter never left-shifts a value that can lose bits: it multiplies by `TWO_POW_32` or `2^k` with `checked_mul` or with `*` under `overflow-checks = true` (§6.3), or calls `I256::shl32`, which panics on lost bits. Right shifts of signed values are arithmetic, that is floor division by `2^k`. The only wrapping left shift in the math crate is xoshiro256++'s `s[1] << 17` (Canon §3.1), which must drop bits to reproduce Canon §21 test 1 (Canon gaps G14).
- **Rounding examples.** `mul_fx(Fx(1), Fx(-1)) = Fx(-1)` because the arithmetic shift floors `-1 >> 32` to `-1`. `mul_fx(Fx(1), Fx(1)) = Fx(0)`. `div_fx(7, 2)` is raw 15032385536 (3.5). `div_fx(1, 3)` is raw 1431655765. `div_fx(-1, 3)` and `div_fx(1, -3)` are both raw -1431655766, because `div_fx` negates both operands when the divisor is negative and then floors.
- `Fx::from_milli(4300).raw() == 18468359372` (Canon §21 test 11). `Fx::from_milli(1620).raw() == 6957847019`, the standing eye height.

### 4.2 `isqrt_u128` and `sqrt_fx`

```rust
/// SPEC: 02 §4.2. Floor square root by Newton's method from above.
pub fn isqrt_u128(n: u128) -> u128 {
    if n < 2 { return n; }
    let bits = 128 - n.leading_zeros();          // significant bits, 2..=128
    let mut x: u128 = 2u128.pow((bits + 1) / 2); // x0 = 2^ceil(bits/2) >= sqrt(n); exponent <= 64, pow is checked
    loop {
        let y = (x + n / x) >> 1;                // never overflows: x <= 2^64, n / x < 2^65
        if y >= x { return x; }
        x = y;
    }
}
pub fn sqrt_fx(x: Fx) -> Fx {                    // Canon §1.4
    assert!(x.raw() >= 0);
    let n = (x.raw() as u128).checked_mul(TWO_POW_32 as u128).expect("invariant: sqrt_fx range");   // < 2^95
    Fx::from_raw(i64::try_from(isqrt_u128(n)).expect("invariant: sqrt_fx result"))                // < 2^48
}
pub fn isqrt_u64(n: u64) -> u64 { isqrt_u128(n as u128) as u64 }   // the root of a u64 is below 2^32: exact
```

- The loop runs at most 7 times for any `u128` input (measured over 300 inputs per bit length 2 to 128). A property test cross-checks the result against `u128::isqrt` from core for 100,000 random inputs.
- Examples: `isqrt_u128(99) = 9`; `isqrt_u128(2^128 - 1) = 2^64 - 1`; `isqrt_u64(2^64 - 1) = 2^32 - 1`; `sqrt_fx(Fx::from_int(2)).raw() = 6074000999` (1.4142135622); `sqrt_fx(Fx::from_int(50)).raw() = 30370004999` (7.0710678117), the jump launch term for `JUMP_H = 1250 mm` at `g = 20 m/s²` (Canon §4.2).

### 4.3 Sine and cosine

- The table `T` (Canon §1.9) is committed as `crates/palimpsest_math/src/trig_table.rs`, `pub static SIN_T: [i32; 16385]`, generated by `tools/gen_trig.py` with mpmath at 50 digits; `python3 tools/gen_trig.py --check` verifies the committed file.
- Spot values: `T[1] = 102944`, `T[2] = 205887`, `T[4096] = 410903207`, `T[8192] = 759250125`, `T[10923] = 929904853`, `T[16383] = 1073741819`, `T[16384] = 1073741824`.
- **Integrity check.** BLAKE3 of the 16,385 values serialized as little-endian `i32` (65,540 bytes) is `d2612a42910edad3b81095ad0be154d6076c5c33771916cbbea2ded40419ec11`. A unit test recomputes it.
- `sin_bam`, `cos_bam` and `fwd` are exactly Canon §1.9. A Q2.30 unit vector converts to `Fx` by `Fx::from_raw(q30 as i64 * 4)` (`|q30| <= 2^30`, so the multiply cannot overflow).
- Example: `fwd(16384, 0) = (-2^30, 0, 0)`, so yaw 16384 faces west (−X), consistent with counter-clockwise yaw seen from above.

### 4.4 `atan2_bam` (16-iteration CORDIC)

`atan2_bam(y, x)` returns the `Bam` angle of the vector `(x, y)` measured counter-clockwise from +x, in `0..=65535`; Canon §1.9 fixes this convention and `yaw_from_dir`, and this section fixes the algorithm. Internally the angle accumulates in units of `2^-16` Bam, so one full turn is `2^32`. `ATAN_T` is committed as `crates/palimpsest_math/src/cordic_table.rs`, generated by `tools/gen_trig.py`.

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
    let (mut x, mut y) = if s >= 0 { let k = 2i64.pow(s as u32); (x * k, y * k) }   // s <= 52; checked multiply
                         else      { (x >> -s, y >> -s) };                        // arithmetic shift (floor)
    // 2. Pre-rotate into the right half-plane.
    let mut z: i64 = 0;
    if x < 0 { x = -x; y = -y; z = 2_147_483_648; }          // +180 degrees (2^31)
    // 3. Vectoring: drive y to 0, accumulating the removed angle.
    for i in 0..16 {
        let (dx, dy) = (y >> i, x >> i);                      // arithmetic shifts (floor)
        if y > 0 { x += dx; y -= dy; z += ATAN_T[i]; }
        else     { x -= dx; y += dy; z -= ATAN_T[i]; }
    }
    ((z + 32_768) >> 16).rem_euclid(65_536) as u16         // round half up, wrap to 0..=65535
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

**Wide division helpers** (doc 05 §2.2 uses them too). Dot products of segment vectors stay exact as `i128` Q64.64 (`Fx3::dot_wide`); products of two such values are Q128.128 in `I256`.

```rust
/// SPEC: 02 §4.6. Ratio of two values of equal scale, as Fx. Requires d != 0; negates both when d < 0.
pub fn div128(n: i128, d: i128) -> Fx {           // floor_div(n × 2^32, d), checked multiply and narrowing
    let (n, d) = if d < 0 { (-n, -d) } else { (n, d) };
    let shifted = n.checked_mul(TWO_POW_32).expect("invariant: div128 shift");
    Fx::from_raw(i64::try_from(floor_div_i128(shifted, d)).expect("invariant: div128 range"))
}
/// Two's-complement 256-bit integer: value = hi * 2^128 + lo. The derived order (signed hi, then
/// unsigned lo) is numeric order, so comparisons use Ord (`a.cmp(&b)`, `<`, `>=`); there is no
/// inherent `cmp` method to shadow it.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct I256 { pub hi: i128, pub lo: u128 }
impl I256 {
    pub const ZERO: I256 = I256 { hi: 0, lo: 0 };
    pub fn from_i128(v: i128) -> I256;             // sign extension
    pub fn mul(a: i128, b: i128) -> I256;          // exact product: schoolbook on 64-bit limbs of |a|, |b|, then sign
    pub fn add(self, o: I256) -> I256;  pub fn sub(self, o: I256) -> I256;  pub fn neg(self) -> I256;   // panic on overflow
    pub fn shl32(self) -> I256;                    // × 2^32; panics if bits are shifted out
    pub fn sar(self, k: u32) -> I256;              // arithmetic shift: floor(self / 2^k), k in 1..=127
    pub fn is_negative(self) -> bool;
}
/// SPEC: 02 §4.6. floor(n / d) for d > 0, by restoring binary long division on magnitudes (256 iterations).
pub fn floor_div_i256(n: I256, d: I256) -> i128;  // q = |n| / d truncated; if n < 0: q = -q - (1 if remainder != 0 else 0)
pub fn div256(n: I256, d: I256) -> Fx {           // Fx(floor_div_i256(n.shl32(), d)), checked; negates both when d < 0
    let (n, d) = if d.is_negative() { (n.neg(), d.neg()) } else { (n, d) };
    Fx::from_raw(i64::try_from(floor_div_i256(n.shl32(), d)).expect("invariant: div256 range"))
}
```

- Range: every segment component below `2^41` raw (512 m) keeps `|dot_wide| < 2^84`, `|n| × 2^32 < 2^116` in `div128` and `|n| × 2^32 < 2^201` in `div256` and `ratio01`. Folio-local segments satisfy this because the Folio box is 256 m on its longest side. `div128` and `div256` panic when the quotient leaves `Fx`; callers whose ratio is clamped use `ratio01` instead.
- Examples: `div128(2 << 64, 4 << 64)` is raw 2147483648 (0.5); `div128(-1, 3)` is raw −1431655766; `div256(I256::mul(3 << 64, 1 << 64), I256::mul(4 << 64, 1 << 64))` is raw 3221225472 (0.75).

**Closest points between segments.** This is the one implementation of the closest-point problem in the simulation. Doc 05 §2.2's `seg_seg_params` is this function's `(s, t)` result. All dot products are exact; there is no epsilon. **Clamp before narrow:** every ratio is clamped to `[0, 1]` in 256-bit arithmetic before it becomes an `Fx`, by comparing numerator and denominator, so nearly parallel segments and segments a few raw units long, whose unclamped ratios leave the `Fx` range, return the clamped answer instead of panicking. For segments inside the range above, no `.expect` in this function can fire.

```rust
/// SPEC: 02 §4.6. clamp01(floor(n × 2^32 / d)) for d != 0, decided before any narrowing.
fn ratio01(n: I256, d: I256) -> Fx {
    let (n, d) = if d.is_negative() { (n.neg(), d.neg()) } else { (n, d) };
    if n <= I256::ZERO { Fx::ZERO }                                             // quotient <= 0
    else if n >= d { Fx::ONE }                                                  // quotient >= 1
    else { Fx::from_raw(i64::try_from(floor_div_i256(n.shl32(), d)).expect("invariant: 0 < q < 2^32")) }
}
/// SPEC: 02 §4.6. Parameters s, t in [0, 1] and the closest points of segments p1–q1 and p2–q2.
pub fn closest_seg_seg(p1: Fx3, q1: Fx3, p2: Fx3, q2: Fx3) -> (Fx, Fx, Fx3, Fx3) {
    let (d1, d2, r) = (q1.sub(p1), q2.sub(p2), p1.sub(p2));
    let (a, e, f) = (d1.dot_wide(d1), d2.dot_wide(d2), d2.dot_wide(r));   // i128, Q64.64
    let w = I256::from_i128;
    let (s, t) = if a == 0 && e == 0 { (Fx::ZERO, Fx::ZERO) }
    else if a == 0 { (Fx::ZERO, ratio01(w(f), w(e))) }
    else {
        let c = d1.dot_wide(r);
        if e == 0 { (ratio01(w(c).neg(), w(a)), Fx::ZERO) }
        else {
            let b = d1.dot_wide(d2);
            let denom = I256::mul(a, e).sub(I256::mul(b, b));                     // >= 0 (Cauchy–Schwarz)
            let mut s = if denom != I256::ZERO {
                ratio01(I256::mul(b, f).sub(I256::mul(c, e)), denom) } else { Fx::ZERO };
            let tn = I256::mul(b, s.raw() as i128).sar(32).add(w(f));            // b·s + f, exact; the shift floors
            let t;                                                                 // t = floor(tn × 2^32 / e), e > 0
            if tn.is_negative() { t = Fx::ZERO; s = ratio01(w(c).neg(), w(a)); }   // t < 0
            else if tn.shl32() >= I256::mul(e, TWO_POW_32 + 1) {                   // t > 1
                t = Fx::ONE; s = ratio01(w(b).sub(w(c)), w(a)); }
            else { t = Fx::from_raw(i64::try_from(floor_div_i256(tn.shl32(), w(e))).expect("invariant: 0 <= t <= 2^32")); }
            (s, t)
        }
    };
    (s, t, p1.add(d1.scale(s)), p2.add(d2.scale(t)))
}
```

Example 1: segment (0,0,0)–(2,0,0) against (1,−1,1)–(1,1,1). `a = e = 4 · 2^64`, `b = 0`, `c = −2 · 2^64`, `f = 2 · 2^64`, `denom = 16 · 2^128`, numerator `8 · 2^128`, so `s = t = 0.5` (raw 2147483648 each), closest points (1,0,0) and (1,0,1), `dist2 = 2^64` (exactly 1 m²).

Example 2 (clamped): segment (0,0,0)–(1,0,0) against (3,1,0)–(3,2,0). `a = e = 2^64`, `b = 0`, `c = −3 · 2^64`, `f = −2^64`, so the unclamped `s = 3` clamps to 1 (raw 4294967296); then `t = (b·s + f) / e = −1` clamps to 0 and `s` is recomputed as `clamp01(−c / a) = 1`. Closest points (1,0,0) and (3,1,0), `dist2 = 5 · 2^64`.

Examples 3 and 4 are the two inputs on which the earlier narrowing form (`clamp01(div256(..))`, `clamp01(div128(..))`) panicked; they are the test `closest_seg_seg_where_doc_code_panics` in `crates/palimpsest_math/src/geom.rs`.

Example 3 (nearly parallel): segment (0,0,0)–(1,0,0) against (5,1,0)–(6, raw 4294967297, 0), whose second segment rises 1 raw unit. `a = 2^64`, `e = 2^64 + 1`, `b = 2^64`, `c = −5 · 2^64`, `f = −5 · 2^64 − 2^32`, `denom = 2^64`, numerator `5 · 2^64 − 2^96`. The unclamped `s` is raw −18446744052234715136, below `i64::MIN`, so `div256` panicked with `invariant: div256 range`. `ratio01` sees a negative numerator and gives `s = 0`; then `tn = f < 0`, so `t = 0` and `s = ratio01(−c, a) = 1` because `−c ≥ a`. Result `s = Fx(4294967296)`, `t = Fx(0)`, closest points (1,0,0) and (5,1,0), `dist2 = 17 · 2^64` (313594649253062377472).

Example 4 (a point against a segment 1 raw unit long): `p1 = q1 = (0,0,0)` against (−5,0,0)–(raw −21474836479, 0, 0). `a = 0`, `e = 1`, `f = 5 · 2^32`, so `div128(f, e)` is raw `5 · 2^64`, above `i64::MAX`, and panicked with `invariant: div128 range`. `ratio01(f, e)` gives 1 because `f ≥ e`. Result `s = Fx(0)`, `t = Fx(4294967296)`, closest points (0,0,0) and (raw −21474836479, 0, 0), `dist2 = (5 · 2^32 − 1)²` (461168601799789117441).

**Capsule overlap.** `capsule_overlap(c1, c2) = dist2(cp1, cp2) <= (c1.r + c2.r).raw()²` in `i128`, with `cp1`, `cp2` from `closest_seg_seg`.

**Vertical capsule against a block.** Collision tests capsules against the occupied 250 mm micro-voxel boxes of each cell (Canon §4.4); a cell whose mask is `u64::MAX` may be tested as its one 1 m box, which gives the same answer. For an axis-aligned box `[lo, hi]` and a vertical segment at `(x, z)` spanning `[y0, y1]`, the distance is separable and exact: `dx = max(0, lo.x − x, x − hi.x)`, `dz` likewise, `dy = max(0, lo.y − y1, y0 − hi.y)`; overlap iff `dx² + dy² + dz² < r²` with `i128` raw squares. Example: origin (2.5, 1.0, 0.5), r 0.3, height 1.8 (segment y 1.3..2.5) against block (3, 1, 0): `dx = 0.5`, `dy = dz = 0`, 0.25 ≥ 0.09, no overlap; at x 2.75, `dx = 0.25`, 0.0625 < 0.09, overlap.

**Ray against capsule** (Quilez form; `rd` must be unit length from `try_normalize`). `EPS = Fx(256)`: when `a = |ba|² − (ba·rd)²` is at most raw 256 (2^-24 m²), the ray is treated as parallel to the capsule axis and only the two end spheres are tested. A tangent ray (`h == 0`) is a hit in the body path and in `ray_sphere` alike.

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
        let t = div_fx(-b - Fx::from_raw(i64::try_from(isqrt_u128(h as u128)).expect("invariant: sqrt h")), a);
        let y = baoa + mul_fx(t, bard);
        if y > Fx::ZERO && y < baba { return nonneg(t); }                              // cylinder body
        return ray_sphere(ro, rd, if y <= Fx::ZERO { pa } else { pb }, r);              // nearer cap
    }
    min_nonneg(ray_sphere(ro, rd, pa, r), ray_sphere(ro, rd, pb, r))                  // ray parallel to axis
}
fn ray_sphere(ro: Fx3, rd: Fx3, cen: Fx3, r: Fx) -> Option<Fx> {
    let oc = ro.sub(cen);
    let b = rd.dot(oc);
    let cc = ((oc.dot(oc) - mul_fx(r, r)).raw() as i128).checked_mul(TWO_POW_32).expect("invariant: ray_sphere range");
    let h = b.raw() as i128 * b.raw() as i128 - cc;                                  // Q64.64
    if h < 0 { return None; }                                                        // h == 0: tangent hit
    nonneg(-b - Fx::from_raw(i64::try_from(isqrt_u128(h as u128)).expect("invariant: sqrt h")))
}
// Helpers: clamp01(x) = x.clamp(Fx::ZERO, Fx::ONE); nonneg(t) = Some(t) if t >= 0 else None;
// min_nonneg(a, b) = the smaller of the Some values; floor_div_i128 is Canon §1.3 floor_div on i128;
// Fx::MAX = Fx(i64::MAX); Fx3 implements Index<usize> (0 = x, 1 = y, 2 = z).
```

Worked example: ray from (0, 1.0, 0) along +X against the capsule (5, 0.3, 0)–(5, 1.5, 0) with r = 0.3 m. `a = 1.44`, `b = −7.2`, `c = 35.8704`, `h = 0.186624`, `sqrt h = 0.432`, `t = (7.2 − 0.432) / 1.44 = 4.7`, `y = 0.84` inside `(0, 1.44)`: body hit at raw `t = 20186346293` (4.7000000004 m). The same ray at height 1.7 m hits the top cap at raw 20514452597 (4.7763932024 m; exact value 4.7763932023). At height 1.8 m it grazes the top cap: `ray_sphere` finds `h = 0` exactly and returns the tangent hit at raw 21474836480 (5 m). At height 2.0 m it misses. A ray from (0, 0.5, 0) along +X against the sphere at (5, 0, 0) with r = 0.5 m is likewise tangent (`h = 0`) and hits at raw 21474836480.

**Voxel traversal (Amanatides–Woo DDA).** Visits every block a ray crosses, in order; axis ties resolve x, then y, then z. Direction components with `|raw| < 256` count as 0, so `div_fx(Fx::ONE, di)` is at most raw `2^56` and cannot overflow. The traversal takes `max_steps`, the number of steps beyond the start block, and yields at most `max_steps + 1` blocks. It stops when the next crossing lies beyond `t_max` (a crossing exactly at `t_max` is still visited), after `max_steps` steps, when the ray does not move, when the next block coordinate would leave the `i32` range (the start block is still visited), or when `visit` returns `false`. Crossing times saturate at `Fx::MAX`, which no crossing reaches. Doc 03 callers pass `max_steps = 3 × ceil_div(t_max.raw(), 2^32) + 3` for a unit direction, which covers every block a ray of length `t_max` can cross (at most `ceil(t_max) + 1` planes per axis). Doc 03 §5.18 zeroes components below raw 65536 before calling, which this contract permits. The DDA's cell is 1 unit, so a traversal of the 250 mm micro-voxel grid (Canon §4.4) runs the same function with the origin and `t_max` multiplied by 4; doc 03 decides where it uses each grid.

```rust
pub struct DdaStep { pub cell: IVec3, pub t: Fx, pub face: Option<Face> }   // face entered; None for the start block
pub struct VoxelDda { cell: [i32; 3], step: [i32; 3], t_delta: [Fx; 3], t_next: [Fx; 3],
                      t_max: Fx, steps_left: u32, started: bool, done: bool }
impl VoxelDda {
    pub fn new(o: Fx3, d: Fx3, t_max: Fx, max_steps: u32) -> VoxelDda {
        let cell = [o.x.floor_block(), o.y.floor_block(), o.z.floor_block()];
        let (mut step, mut t_delta, mut t_next) = ([0i32; 3], [Fx::MAX; 3], [Fx::MAX; 3]);
        for i in 0..3 {
            let (di, frac) = (d[i], Fx::from_raw(o[i].raw() & 0xFFFF_FFFF));   // frac = o − floor(o), in [0, 1)
            if di.raw() >= 256       { step[i] = 1;  t_delta[i] = div_fx(Fx::ONE, di);  t_next[i] = div_fx(Fx::ONE - frac, di); }
            else if di.raw() <= -256 { step[i] = -1; t_delta[i] = div_fx(Fx::ONE, -di); t_next[i] = div_fx(frac, -di); }
        }
        VoxelDda { cell, step, t_delta, t_next, t_max, steps_left: max_steps, started: false, done: false }
    }
}
impl Iterator for VoxelDda {
    type Item = DdaStep;
    fn next(&mut self) -> Option<DdaStep> {
        if self.done { return None; }
        if !self.started { self.started = true; return Some(DdaStep { cell: self.cell.into(), t: Fx::ZERO, face: None }); }
        if self.steps_left == 0 { self.done = true; return None; }
        let mut ax = 0; if self.t_next[1] < self.t_next[ax] { ax = 1; } if self.t_next[2] < self.t_next[ax] { ax = 2; }
        let t = self.t_next[ax];
        if self.step[ax] == 0 || t > self.t_max { self.done = true; return None; }   // zero direction: only the start cell
        let Some(c) = self.cell[ax].checked_add(self.step[ax]) else { self.done = true; return None; };   // i32 edge
        self.cell[ax] = c;
        self.t_next[ax] = self.t_next[ax].checked_add(self.t_delta[ax]).unwrap_or(Fx::MAX);   // saturate
        self.steps_left -= 1;
        Some(DdaStep { cell: self.cell.into(), t, face: Some(Face::entered(ax, self.step[ax])) })
    }
}
pub fn dda(o: Fx3, d: Fx3, t_max: Fx, max_steps: u32, mut visit: impl FnMut(IVec3, Fx, Option<Face>) -> bool) {
    for s in VoxelDda::new(o, d, t_max, max_steps) { if !visit(s.cell, s.t, s.face) { return; } }   // false = stop
}
```

`Face::entered(axis, step)` is the face of the new block the ray enters: a step of +1 along x enters through −X, a step of −1 through +X, and likewise for y and z.

Example: origin (0.5, 1.5, 0.5), direction (0.6, 0, −0.8), `t_max` = 4 m (so doc 03 passes `max_steps = 15`), block (2, 1, −3) solid. Visits (0,1,0) at t 0; (0,1,−1) at raw 2684354559 (0.625) entering through +Z; (1,1,−1) at 0.833333 through −X; (1,1,−2) at 1.875 through +Z; (2,1,−2) at 2.5 through −X; (2,1,−3) at raw 13421772797 (3.125) through +Z, where it stops.

### 4.7 Hashes and RNG

`palimpsest_math::rng` contains Canon §3.1 verbatim, `stream_seed` and the labeled sub-streams `derive` (§3.2), `keyed_rng` and the object-keyed draws (§3.3), which take `run_seed` as an explicit first parameter, the offer generator (§3.5) and the presentation generators `nar_rng` and `pres_hash` (§3.8). `keyed_rng` panics when `seq > 0xFFFF` (Canon §3.3).

- **Save interface.** `Rng` implements `Clone`, `PartialEq` and `Debug`. `Rng::state() -> [u64; 4]` and `Rng::from_state([u64; 4]) -> Rng` are the save interface: section 1 (§6.5) writes the four state words through the hand-written serialization, and `restore` rebuilds each stream with `from_state`.
- **serde.** `palimpsest_math` depends only on `blake3`. Its optional feature `serde` is off by default; `palimpsest_sim`'s `serde` feature (for the message types of §3.3) and `palimpsest_tools` (for tool output) enable it. It derives `Serialize` and `Deserialize` for `Fx`, `Fx3`, `IVec3`, `Capsule`, `VCapsule`, `Ray`, `Face` and `Rng`. The canonical serialization never uses it (Canon §2.5), and with default features neither `palimpsest_math` nor `palimpsest_sim` has `serde` in its closure.
- **Draw counters.** Debug builds of `palimpsest_sim` count draws per stream for diagnostics in `palimpsest_sim::api`, in a table outside `World` that never enters simulation state, saves, `state_hash` or section hashes (Canon §2.5). `palimpsest_math::rng` keeps no counter. No RNG draw happens inside `#[cfg(debug_assertions)]` (§6.1).
- **Hashes.** `fnv1a32`, `fnv1a64` use Canon §1.1 constants. `blake3_256(parts)` hashes the concatenation of `parts`; `blake3_fields(fields)` prefixes each field with its length as `u32le`, the Canon §2.5 rule for any hash over several variable-length fields.

## 5. Simulation architecture (`palimpsest_sim`)

### 5.1 Decision: custom sorted-column store

The simulation uses a custom struct-of-columns store keyed by the canon `EntityId`, not an ECS crate. `bevy_ecs` would put Bevy in the sim closure (brief §5) and iterates by archetype table, not ascending ID (Canon §1.11); `hecs`, `legion` and `specs` share the ordering problem; a generational arena such as `slotmap` adds nothing because canon IDs are never reused and carry the Folio index in bits 40 and up. With sorted columns, a spawned entity's ID always exceeds every live ID (the counter only grows within a Folio, and carried entities have smaller `folio_index`), so spawns are appends, iteration is ascending by construction and lookups are binary searches. Adding a component to an older entity is a binary-search insert, which costs one `memmove` of the column tail. The canon caps bound the capped kinds at 3,328 entities (256 actors, 1,024 projectiles, 2,048 ground items).

### 5.2 Store

```rust
pub struct Column<T> { ids: Vec<EntityId>, vals: Vec<T>, dead: Vec<bool> /* parallel to ids */ }
impl<T> Column<T> {
    pub fn get(&self, id: EntityId) -> Option<&T>;             // binary search, skips dead
    pub fn get_mut(&mut self, id: EntityId) -> Option<&mut T>;
    pub fn iter(&self) -> impl Iterator<Item = (EntityId, &T)>;  // ascending EntityId, skips dead
    fn insert(&mut self, id: EntityId, v: T);   // append if id > last id; else binary-search insert;
                                                // an existing entry with the same id is overwritten and revived
    fn kill(&mut self, id: EntityId);           // sets dead; used for despawn and remove-component
    fn compact(&mut self);                      // drops dead entries, end of tick
}
pub fn join2<'a, A, B>(a: &'a Column<A>, b: &'a Column<B>) -> impl Iterator<Item = (EntityId, &'a A, &'a B)>;  // merge walk: advance the smaller id
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
- `compact()` runs once at the end of every tick (`sys.tick_end`, doc 03 §2.2).
- **Copy-on-write chunks.** `snapshot()` clones the `Arc`s, not the chunks. The first write to a chunk whose `Arc` is shared clones it through `Arc::make_mut`: at most 128 KiB of fixed arrays (blocks at 16-bit palette width 64 KiB, light 32 KiB, the four cell bitsets of doc 03 §4.7 16 KiB, thermal 16 KiB) plus the chunk's liquid and gas maps and its micro-voxel mask table (Canon §4.4). That cost lands in the writing system and is part of its worst budget (§15); at most 128 clones happen per snapshot.

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
pub static SCHEDULE: &[SystemDesc] = &[ /* the 39 systems of doc 03 §2.2 in table order; periods and phases from Canon §2.2 */ ];
```

Example: entity `(3 << 40) | 5` with AI think period 6 has phase key 5 and thinks on ticks where `(t + 5) % 6 == 0`, that is `t = 1, 7, 13, …`.

The simulation runs on one thread. The only in-sim parallelism is `OrderedMap`, injected by the host binary for worldgen and doc 03's pure batch computations: `map(n, f)` returns `f(0..n)` in index order, whatever threads computed them. The default is sequential.

### 5.4 Tick lifecycle

```rust
pub fn step(&mut self, inp: &TickInputs) -> StepOutput {
    assert_eq!(inp.tick, self.tick);                       // boundary = next tick to simulate
    self.ctx.begin(inp);                                   // commands by slot 0..=4; session intents, then intents by (slot, action_seq)
    for sys in SCHEDULE {
        if sys.due(self.tick) { (sys.run)(&mut self.world, &mut self.ctx); self.world.apply(&mut self.ctx.cmds); }
    }
    self.world.compact();
    self.tick += 1;                                        // self.tick is now the boundary b
    let at_60 = self.tick % 60 == 0;
    let hash = if self.flags.contains(SimFlags::HASH_EVERY_TICK)
                  || (self.flags.contains(SimFlags::HASH_AT_60) && at_60) { Some(self.state_hash()) } else { None };
    let snapshot = if self.flags.contains(SimFlags::SNAPSHOT_AT_60) && at_60 { Some(Box::new(self.snapshot())) } else { None };
    let view = if self.flags.contains(SimFlags::EMIT_VIEW) { Some(self.extract_view()) } else { None };
    StepOutput { tick: self.tick - 1, events: core::mem::take(&mut self.ctx.events), hash, snapshot, view,
                 diagnostics: self.ctx.take_diag() }
}
```

A state hash always describes a boundary: the state after simulating tick `b − 1`, before simulating tick `b`. The interactive host sets `SNAPSHOT_AT_60` and hashes the returned snapshot on a worker thread (§6.5), so hashing never runs on the sim thread. Headless servers and bots set `HASH_AT_60` and hash synchronously. CI sets `HASH_EVERY_TICK`.

### 5.5 Sim API

```rust
pub const SIM_VERSION: u32 = 1;
pub struct SimConfig {
    pub run_seed: u64, pub mode: Id32, pub party: [Option<PartyMember>; 4],   // the invader joins by intent (slot 4)
    pub content: Arc<ContentDb>, pub flags: SimFlags,
    pub script_host: Option<Box<dyn ScriptHost + Send>>,    // WASM mods (§13); None = no scripted effects
    pub ordered_map: Option<Box<dyn OrderedMap + Send + Sync>>,
}
bitflags! { pub struct SimFlags: u32 { const EMIT_VIEW = 1; const HASH_EVERY_TICK = 2; const HASH_AT_60 = 4; const SNAPSHOT_AT_60 = 8; } }
impl Sim {
    pub fn new(cfg: SimConfig) -> Result<Sim, SimInitError>;
    pub fn tick(&self) -> u64;
    pub fn step(&mut self, inputs: &TickInputs) -> StepOutput;
    pub fn snapshot(&self) -> SimSnapshot;               // clones columns; shares chunks through Arc
    pub fn restore(&mut self, s: &SimSnapshot);           // afterwards state_hash() == s.state_hash()
    pub fn state_hash(&self) -> StateHash;                // BLAKE3 of serialize() (Canon §2.5)
    pub fn section_hashes(&self) -> [StateHash; SECTIONS];   // SECTIONS = 13 (§6.5)
    pub fn serialize(&self, out: &mut Vec<u8>);           // canonical serialization (§6.5), the save payload
    pub fn deserialize(bytes: &[u8], cfg: SimConfig) -> Result<Sim, LoadError>;
    pub fn extract_view(&mut self) -> ViewOut;           // §7.2; a read-only pass over simulation state (doc 03 §2.2); &mut only
                                                         // for the per-chunk sent generations, view bookkeeping that serialize() never writes
}
impl SimSnapshot { pub fn serialize(&self, out: &mut Vec<u8>); pub fn state_hash(&self) -> StateHash; } // off-thread hashing
```

**Input packet.** The command is exactly the Canon §2.4 record:

```rust
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]   // messages only (§3.3); §6.5 never uses it
pub struct PlayerCommand { pub tick: u64, pub slot: u8, pub move_x: i8, pub move_z: i8, pub yaw: u16,
                           pub pitch: i16, pub buttons: u32, pub action_seq: u32, pub view_tick: u64 }
pub struct TickInputs { pub tick: u64, pub commands: [Option<PlayerCommand>; 5], pub intents: Vec<PlayerIntent> }
// slots 0..=3: party; slot 4: invader (Canon §2.4, §17.9). None = no command applied for that slot this tick.
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
| 4 | `CROUCH` | 21 | `LEAN_LEFT` (Canon §4.2 `move.lean_l`) |
| 5 | `SPRINT` | 22 | `LEAN_RIGHT` |
| 6 | `DODGE` | 23 | `PING` |
| 7 | `INTERACT` | 24 | `WALK` (slow-walk modifier) |
| 8 | `MELEE_QUICK` | 25 | none: always 0, because mounts are cut (Canon §17.7) and the mount toggle is retired |
| 9 | `CAST` | 26–28 | `ACTION` (doc 17 §12.2): 0 none; 1..=4 `input.ability_1..=4`; 5 `input.cast_cancel`; 6 and 7 are invalid |
| 10 | `THROW` | 29–31 | `WHEEL_ENTRY` (doc 17 §12.2): 0..=7, read only when `WHEEL_SELECT` is set |
| 11, 12 | `WEAPON_NEXT`, `WEAPON_PREV` | 19 and 20 together | `WHEEL_SELECT`: both set in one command selects wheel entry `WHEEL_ENTRY` (doc 17 §12.2) |
| 13–14 | `WEAPON_SELECT` (0 none, 1..=3 slot) | | |

The replay validator and doc 16 §5.8's sanitizer reject a command with bit 25 set or with `ACTION` 6 or 7; no other bit pattern is invalid. An `ACTION` press is a change to a value `k ≠ 0` from any other value; a wheel selection is a command with bits 19 and 20 both set whose predecessor did not have both set (doc 17 §12.2).

**Intents** carry discrete actions that are not button bits, on the reliable channel. Canon §2.4 logs every applied intent `{ tick, slot, action_seq, kind }` in the replay beside the commands; this chapter owns the list of `IntentKind` variants, and adding one needs no canon change. Each variant's payload and validation belong to the document in the first column; a rejected intent emits a diagnostic and changes nothing. Intents apply on their logged tick inside `sys.input_apply` (doc 03 §2.2): doc 16's session intents first, then the rest in ascending `(slot, action_seq)`.

| Owner | `IntentKind` variants |
|---|---|
| 04 | `AllocatePoint`, `PickTalent` |
| 06 | `ComposeSpell`, `Pray { patron, kind }` |
| 07 | `Craft { recipe, count }` |
| 08 | `Brew` (request of doc 08 §9.3) |
| 09 | `Hack { target, verb }`, `Steal { target, uid }`, `Plant { target, uid }` |
| 11 | `EquipItem`, `UnequipItem`, `MoveItem`, `DropItem`, `UseItemOn`, `ShopBuy`, `ShopSell`, `ShopHaggle { shop }`, `DropLucre { amount: i64 }` |
| 11, 14 | `CommitOffer` (Canon §3.5) |
| 13 | `MinionCommand { kind, minions: All \| List(Vec<EntityId>), cell, target }` |
| 14 | `ChooseExit { door }` |
| 16 | `TradePropose { to_slot }`, `TradeRespond { accept }`, `TradeSetItems { uids }`, `TradeLock { locked }`, `TradeConfirm`, `TradeCancel`, `DropItemFor { uid, qty, to_slot }`, `MarkTarget { target }`; host-only `Session(SessionIntent)` (doc 16 §5.7); host-issued on slot 4: `InvaderJoin { kit: Id32 }` and `InvaderLeave { reason: u8 }` (Canon §17.9; reasons in doc 16 §5.7) |
| 17 | `SetScribesMercy { on }`, `MapPing { cell: [i32; 3] }`, `AssignWheelEntry { index, source }`, `AssignAbilitySlot { slot, talent }`, `QuickbarAssign { index, uid }`, `SetFocusLock { uid }`, `Examine { uid }`, `SetCastItemTarget { uid }`, `SetBreathHold { on }`, `Vocalize { kind }`, `SetLockMode { manual }` |

Doc 17 sends its variants; the systems that read them are doc 06 (`AssignWheelEntry`, `SetCastItemTarget`), doc 04 (`AssignAbilitySlot`), doc 03 and doc 04 (`SetBreathHold`), doc 09 (`Vocalize`, `SetLockMode`), doc 10 (`Examine`, the §5.6 Analysis channel; `SetFocusLock`, the appraisal focus) and doc 17 itself (`SetScribesMercy`, `MapPing`, `QuickbarAssign`).

**Output.** `StepOutput { tick, events: Vec<SimEvent>, hash: Option<StateHash>, snapshot: Option<Box<SimSnapshot>>, view: Option<ViewOut>, diagnostics: Vec<Diagnostic> }`. `SimEvent { id: Id32 /* ev.* */, seq: u32, payload: EventPayload }` keeps emission order; `seq` counts from 0 each tick. Doc 03 owns payload schemas. Diagnostics (mod disabled, budget overrun, rejected intent) are informational and never read back by the simulation.

## 6. Determinism plan

### 6.1 Forbidden constructs

| Construct | Why | Enforcement |
|---|---|---|
| `f32`, `f64`, float literals in math, data core, econ, gen, ai, sim | platform and libm differences (Canon §1.2); econ's odds and pity results must match the server bit for bit | `cargo xtask float-scan` lexer, clippy float lints |
| `HashMap`, `HashSet`, any hash-ordered iteration | per-process random seeds (Canon §1.11) | `no_std` (unavailable), `disallowed-types` |
| clocks, threads, environment, filesystem | nondeterministic inputs | `no_std` |
| `rand`, `getrandom` | unseeded randomness | `forbid_closure` (§3.3) |
| `sort_unstable*`, `select_nth_unstable*` | equal-key order unspecified | `disallowed-methods`; sorts use stable sort with `EntityId` as the final key |
| `BinaryHeap` or a sort keyed without a unique final tie-break | pop order among equal keys is an implementation detail | review rule plus a property test per queue: shuffled insertion order yields the same pop order; keys end with a cell index, `EntityId` or `seq` |
| `usize`, `isize` in serialized state | 32-bit on wasm32 | `xtask serde-scan` fails on any `Canonical` impl or serialized field of type `usize` or `isize`; state uses fixed-width integers |
| serde or postcard in the canonical serialization | Canon §2.5: the serialization is hand-written and self-delimiting | `forbid_closure` lists `postcard` for `palimpsest_sim` and its default closure has no `serde` (§3.3) |
| pointer-to-integer casts, `type_name`, `TypeId` hashing | address and compiler dependence | `xtask` grep for `as *const`, `as *mut`; `disallowed-methods` |
| RNG draws inside `#[cfg(debug_assertions)]` | debug and release diverge | `xtask` grep |
| `RefCell`, `Rc`, atomics in sim | hidden mutation order | `disallowed-types` |

### 6.2 Lint configuration

```toml
# crates/{math,data,econ,gen,ai,sim}/clippy.toml
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
  { path = "core::slice::<impl [T]>::select_nth_unstable", reason = "02 §6.1" },
  { path = "core::any::type_name", reason = "02 §6.1" },
]
```

```rust
// lib.rs of math, econ, gen, ai, sim; palimpsest_data uses #![cfg_attr(not(any(test, feature = "loader")), no_std)] instead.
// Unit tests link std (proptest, catch_unwind); every non-test build is no_std.
#![cfg_attr(not(test), no_std)]
#![forbid(unsafe_code)]
#![deny(clippy::float_arithmetic, clippy::float_cmp, clippy::lossy_float_literal, clippy::cast_precision_loss,
        clippy::unwrap_used, clippy::wildcard_enum_match_arm, clippy::disallowed_types, clippy::disallowed_methods,
        clippy::too_many_lines, missing_docs)]
extern crate alloc;
```

`cargo xtask float-scan` is the authoritative check for Canon §21 test 33. It lexes every `.rs` file in the six crates of §6.1 (data with `loader` off), skips comments and string literals, removes items under `#[cfg(test)]`, and fails on the tokens `f32`, `f64`, any literal containing `.` followed by a digit, any decimal literal with an exponent, and any `f32`/`f64` suffix.

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
| `wasm32-wasip1` | `ubuntu-24.04`, run under `wasmtime-cli` 49.0.2 | `ci-det` | determinism set; 32-bit `usize` catches width bugs |
| Steam Deck (SteamOS 3, self-hosted `deck-01`), the `x86_64-pc-windows-msvc` build under Proton | self-hosted | `dist` | nightly performance gates (§15) |

The determinism set is 50 golden replays of 3,600 ticks on every pull request and 500 replays of 18,000 ticks nightly. Each job writes `hashes.json`, a sorted list of `{replay, boundary, hash}`. A final job requires all five files to be byte-identical. Only the Windows target ships a client (DECISIONS.md item 98); the Linux and macOS jobs are determinism checks, and the Linux build is also the headless verifier that the backend runs (§16, doc 16).

### 6.5 Canonical serialization and state hash

The serialization is a fixed sequence of sections. Each section is written by hand-written `put` functions in `palimpsest_sim::serial` and read back by strict `read` functions; neither uses serde or postcard (Canon §2.5). The encoding is self-delimiting:

- Integers `u8`, `i8`, `u16`, `i16`, `u32`, `i32`, `u64`, `i64` are fixed-width little-endian, two's complement when signed. `Fx` is its raw `i64`, `Id32` its `u32`, `EntityId` and item uids their `u64`, `Bam` its `u16`.
- `bool` is one byte, 0 or 1. `Option<T>` is a tag byte, 0 for `None` or 1 followed by `T`.
- A list (`Vec`, sorted `Vec`, `BTreeSet`) is `u32le(count)` followed by its elements in order. A `BTreeMap` is `u32le(count)` followed by `(key, value)` pairs in ascending key order, so equal states serialize equally whatever their history. A string is `u32le(byte length)` followed by its UTF-8 bytes. A fixed-size array writes its elements without a count.
- An enum is its `u8` variant index in declaration order, followed by that variant's fields. A struct is its fields in declaration order.
- `palette_rle` (below) is the only compressed form; its counts and run lengths are unsigned LEB128 `varint`s.
- A count above `u32::MAX` panics; no state list comes near it. Decoding rejects a tag other than those listed, a truncated value, invalid UTF-8 and trailing bytes after a section's last field.

| # | Section | Contents |
|---|---|---|
| 0 | `meta` | `SIM_VERSION`, boundary tick, run seed, mode, Folio index, `folio_entry_tick`, `EntityId` and uid counters, party and invader slots |
| 1 | `rng` | every sequential stream as `(name, Rng::state())`, ascending by name (§4.7) |
| 2 | `entities` | every component column in catalog order (doc 03 §3.3), each as `(EntityId, value)` pairs ascending; then the spawn queue |
| 3 | `items` | item instances ascending by uid |
| 4 | `blocks` | 128 chunks in slot order: `palette_rle` of `BlockState`, then the four cell bitsets (`sky_src`, `open_tag`, `protect`, `flip`), then the chunk's micro-voxel masks that differ from their shape's predefined mask (custom and chipped masks, doc 03's `mask_of`, Canon §4.4) as ascending `(Local: u16, mask: u64)` pairs; then the block-update FIFO and the other queues and maps of doc 03 §16.1 |
| 5 | `light` | per chunk `palette_rle` of the packed light bytes (doc 03 §4.3); then the relight FIFOs |
| 6 | `fluids` | per chunk the ascending `(Local, LiquidCell)` and `(Local, GasCell)` lists; active sets and run counters |
| 7 | `thermal` | per chunk the asleep byte or the `palette_rle` of the 4,096 thermal offsets; then the fire cells ascending |
| 8 | `ai` | agent memories, plans and cached paths |
| 9 | `run` | lattice history, Shock of the New set, Death's Door flags, stored offers, Erasure clocks, score state |
| 10 | `inputs` | previous buttons and last `action_seq` per slot `0..=4` |
| 11 | `mods` | per-mod enabled flag and mod-owned key-value state |
| 12 | `stealth` | doc 09's Folio-level `StealthWorld` (locks, hides, scent ring, reveal meters, alarms, backlog, hacked devices, alert flags) in its declared field order |

```
serialization = for k in 0..13: u8(k) ‖ u32le(len(S_k)) ‖ S_k
state_hash    = BLAKE3(serialization)                  // Canon §2.5
section_hash  = BLAKE3(S_k)                            // desync localization only

palette_rle(cells: &[T]):                              // T is u8 or u16; len 32,768 (blocks, light) or 4,096 (thermal)
    palette = distinct values in order of first appearance, scanning index 0..len
    if len(palette) <= 256: emit 1u8, varint(len(palette)), palette values (LE), then runs (u8 palette index, varint run length)
    else:                    emit 2u8, then runs (T value LE, varint run length)
```

An all-air chunk encodes in 8 bytes: tag 1, palette length 1, value `0u16`, then index 0 with run 32768. Doc 03 §16.1 fixes the exact field order inside sections 0, 2, 4, 5, 6 and 7, and doc 09 inside section 12; this table fixes the section order. Section 12 was appended after sections 0 to 11, so every earlier index is unchanged. Each chunk caches its encoded block and light bytes together with the generation counter (`block_gen`, `field_gen`) they were built from, so a hash re-encodes only chunks that changed.

**Cost and cadence.** The serialization of a mid-run Folio is at most 2 MiB. BLAKE3 at 1.5 GB/s on one Steam Deck core takes 1.4 ms, so the interactive host never hashes on the sim thread: at each boundary `b ≡ 0 (mod 60)` `step` returns `snapshot()` (budget 0.50 ms, `perf.deck.sim.hash_snapshot`), and a worker thread serializes and hashes it (budget 4 ms, `perf.deck.hash_ms`, result due within 10 ticks).

### 6.6 Golden replays

- A golden replay is a container (§12.1) of kind `replay`: session header (Canon §2.4), mode and party, command log, intent log, and checkpoints `(boundary, StateHash)` at 600, 1,800, 3,600 and every further 3,600.
- `pal golden verify` re-simulates and compares at each checkpoint. On a mismatch it re-runs from the last matching checkpoint with `HASH_EVERY_TICK` and reports the first diverging boundary, the differing sections, and the first differing `(column, EntityId)` or `(chunk, cell)`.
- The fast set (50 replays) covers each core genre idle and in combat, Folio generation for all 16 genres, liquid and fire stress, 4-player co-op, save and reload at tick 4,321 (Canon §21 test 32), and one WASM test mod.
- `pal golden bless` rewrites expected hashes and the header's `content_hash` and `presentation_hash` (Canon §2.4). CI rejects a golden-hash change unless the same pull request changes `SIM_VERSION` or `content/`. A pull request that changes `content/` must commit the re-blessed goldens; CI re-runs the bless and fails if the result differs from the committed files.

### 6.7 Desync detection and dump

1. Every peer holding a full replica (doc 16 decides which) hashes at boundaries `b ≡ 0 (mod 60)`; the host sends `HashReport { boundary, hash }` on the reliable channel.
2. A peer with a different hash sends `SectionHashRequest { boundary }`; the host keeps section hashes for its last 8 hash boundaries and answers with all 13.
3. Both sides write `desync_<boundary>_<role>.pald` (container kind `desync`): local serialization at `b`, both section-hash sets, the input and intent log since the last matching boundary `b0`, the serialization at `b0` (peers keep their last 2 matching snapshots), and git hash, target triple and profile. The client then requests a full resync (doc 16).
4. `pal desync-diff a.pald b.pald` prints the first differing section and inside it the first differing `(column, EntityId, field)`, `(chunk, cell, a, b)` or RNG stream; `--resim` replays from `b0` and reports the first diverging tick.

**SyncTest.** With `flag.sim.synctest_depth = N > 0`, every tick runs snapshot, N steps, restore, the same N steps, and asserts equal hashes. CI uses N = 8, which proves `snapshot` and `restore` capture all state before netcode exists.

## 7. Sim-render bridge (`palimpsest_bridge`)

### 7.1 Threads and mailbox

The host or single-player client runs the simulation on its own thread. It steps at 60 Hz from a monotonic-clock accumulator, catching up at most 4 ticks per wake. After each step with `EMIT_VIEW` the bridge publishes the step's `ViewOut` through three lossless paths:

| Path | Carries | Mechanism | Why |
|---|---|---|---|
| Entity pair | `ViewFrame` (entities, players, telegraphs) | one `ArcSwap<FramePair>`; `publish` swaps in `FramePair { prev: old.curr, curr: new }` | interpolation needs only the newest two states; skipping a frame loses nothing |
| Chunk views | per chunk slot the newest `ChunkView` | `[ArcSwap<ChunkView>; 128]`; the render world compares generations each frame | the newest state of a chunk supersedes older ones |
| Delta queue | `FolioView` at Folio entry and every tick's `events` | unbounded `crossbeam-channel` drained completely every render frame, in tick order | events and Folio changes must never be skipped when several ticks publish between two frames |

The render world loads the pair and the changed chunk views once per frame without locking. The delta queue holds at most 600 ticks of event batches: if the render world stops draining (a minimized window), the oldest event batches are dropped with a diagnostic, which only skips presentation; a `FolioView` is never dropped. A remote client feeds the same three paths from host snapshots (doc 16). The simulation never waits for rendering.

### 7.2 View frame

```rust
pub struct ViewOut { pub frame: ViewFrame, pub chunks: Vec<ChunkView>, pub folio: Option<Arc<FolioView>> }
pub struct ViewFrame {
    pub tick: u64, pub published_at_ns: u64,           // published_at is set by the bridge, not the sim
    pub entities: Vec<ViewEntity>,                     // ascending EntityId; every live entity with a visual
    pub players: [Option<ViewPlayer>; 5],              // slots 0..=3 party, 4 invader
    pub telegraphs: Vec<ViewTelegraph>,                // active enemy telegraphs (doc 13 §1 rule 3)
    pub light_overrides: Vec<LightOverride>,           // doc 03 §10.6 active overrides, insertion order; the renderer clamps with them (§8.5)
    pub shadow_cells: Vec<u32>,                        // ascending Gidx of the cells where doc 06's in_shadow_area holds; dimmed by 8 (§8.5)
}
pub struct ViewEntity { pub id: EntityId, pub visual: Id32, pub pos: Fx3, pub yaw: Bam, pub pitch: i16,
    pub anim: Id32, pub anim_tick: u32, pub origin_genre: Id32, pub hp_bp: u16 /* HP / Max HP, 0..=10000 */,
    pub flags: u32 /* Kind.flags of doc 03 §3.3: DEAD, ASLEEP, GHOST, NO_COLLIDE_BODIES, TELEPORTED; bit 8 DOWNED, bit 9 INVISIBLE */ }
pub struct ViewPlayer { pub entity: EntityId, pub pools: Pools, pub statuses: Vec<(Id32, u8, Option<Tick>)>,
    pub equipment: Vec<(Id32, u64)>, pub light_eff: u8, pub music_level: u8 /* §9 */,
    pub revealed: Vec<EntityId>,                       // doc 09 §4.8: stealthy or invisible NPCs revealed to this slot, ascending
    pub dark_vision_mm: i32 }                          // Canon §6.4 row 31 radius of this slot's Erratum (doc 18 dark vision)
pub struct ViewTelegraph { pub source: EntityId, pub shape: Id32, pub a: Fx3, pub b: Fx3, pub radius_mm: i32,
    pub start_tick: u64, pub end_tick: u64 }
pub struct ChunkView { pub slot: u8, pub block_gen: u32, pub field_gen: u32, pub fluid_gen: u32,
    pub blocks: Arc<[u16; 32768]>,                     // decoded BlockState per local index
    pub light: Arc<[u8; 32768]>,                       // doc 03 §4.3 nibbles: low light_block, high light_sky
    pub masks: Arc<[(u16, u64)]>,                      // (Local, mask) for every cell whose micro-voxel mask differs from its
                                                       // shape's predefined mask (custom and chipped), ascending Local (Canon §4.4)
    pub fluids: Arc<FluidView> }
pub struct FluidView { pub liquids: Vec<(u16, Id32, u16)>,   // (Local, liquid, units 1..=1000), one entry per component
                       pub gases: Vec<(u16, Id32, u16)>,     // (Local, gas, concentration 1..=10000)
                       pub fires: Vec<(u16, u8)> }           // (Local, intensity 1..=3)
pub struct FolioView { pub folio_index: u32, pub genre: Id32,
    pub bleed: Vec<Id32>,                              // bleed slots 1..=4 in doc 12 §13.1 order: entry, then drift
    pub seams: Vec<Id32>,                              // second genre of each seam zone, in doc 12 order, at most 7
    pub slot_of_cell: Box<[u8]> }                      // 4,194,304 entries in Gidx order: bits 0–2 bleed slot 0..=4
                                                       // (0 = Folio genre, seams too); bits 3–5 seam index + 1 (0 = no seam); bits 6–7 zero
pub enum ViewDelta { Folio(Arc<FolioView>), Events { tick: u64, events: Arc<[SimEvent]> } }   // the delta queue
```

- **Extraction rule.** `extract_view` copies an array of a chunk only when the matching generation (`block_gen`, `field_gen`, `fluid_gen`) differs from the one last sent. A changed micro-voxel mask bumps `block_gen` (doc 03). Per tick it sends at most 16 light arrays (≤ 8 µs each on Deck), 8 fluid views (≤ 10 µs each) and 2 decoded block arrays with their mask lists (≤ 60 µs each), oldest change first; the rest wait for later ticks. Chunk work is therefore at most 0.33 ms and entity extraction at most 0.27 ms, inside the 0.60 ms worst budget of `perf.deck.sim.view_extract` (§15). The generation counters are derived state and never hashed (doc 03 §4.3).
- `FolioView` is sent once, on the step that emits `ev.folio_entered`.
- `light_overrides` and `shadow_cells` carry every input of doc 03 §10.6's light read: the overrides with their optional sphere, and the cells inside doc 06 §13.1's shadow fields and `linger` areas, found by evaluating `in_shadow_area` over each active area's cells. The renderer applies that read to the light field (§8.5), so rendered light never exceeds simulation light.
- `ViewPlayer.revealed` and `dark_vision_mm` are presentation inputs for doc 18 (reveal markers instead of the shimmer, the dark-vision luminance floor); the bridge never writes them back.

### 7.3 Interpolation

- `alpha = clamp((now_ns − curr.published_at_ns) × 60 / 10^9, 0, 1)` as `f32`. Positions render at `lerp(prev.pos, curr.pos, alpha)`. The renderer never extrapolates (Canon §2.1).
- Yaw takes the short arc: `d = curr.yaw.wrapping_sub(prev.yaw) as i16`, rendered yaw `= prev.yaw + d × alpha`.
- An entity snaps to `curr` when `TELEPORTED` is set or when it moved more than 4 m in one tick.
- The local camera's yaw and pitch come from the current look state (§10), so look has no simulation latency. Its position comes from prediction (doc 16) or, on the host, from interpolation.
- Example: `curr` was published 7.67 ms ago, so `alpha = 0.46`; an entity at x = 10.0 m in `prev` and 10.1 m in `curr` renders at 10.046 m.

### 7.4 Entity mapping

The client keeps `BTreeMap<EntityId, bevy::Entity>` and merge-walks each new `curr` against the previous frame: new IDs spawn presentation from their `visual` record (doc 18), missing IDs despawn. On `ev.entity_killed` the body becomes a cosmetic ragdoll (at most 16, oldest removed first). When the simulation spawns a `grid.ekind.corpse` entity for the victim (doc 03 §3.2), a cosmetic spring keeps the ragdoll's root within 0.5 m of the corpse position, so the lootable corpse and the visible body stay together. Ragdolls and debris never feed back (Canon §22).

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
| `look_add(q, d_rad)` | `q.wrapping_add((d_rad as f64 * 2^32 / TAU).round() as i64 as u32)` on `yaw_q: u32` (2^32 per turn) | 0.01 rad → +6835653 |
| `look_to_bam(q)` | `((q as u64 + 0x8000) >> 16) as u16` (wraps 65536 to 0) | 6835653 → 104 |

The reverse functions exist only to build `PlayerCommand`; the integers they produce are what the replay logs, so determinism holds.

### 7.6 Event routing

Doc 18's `vfx.` and `aud.` records declare a `trigger`, e.g. `{ "event": "ev.damage_dealt", "dominant_type": "dmg.fire" }`, indexed at load by event `id32`. Per `SimEvent` the router picks at most one VFX and one audio record: most matching predicates (`dominant_type`, `status`, `material`, `local_genre`, `visual`), ties by ascending content ID. Effects spawn at the event position in the event's genre skin. Audio for `ev.noise` uses the heard level and arrival direction of §9; every other positional sound uses the presentation occlusion estimate of §9.

## 8. Rendering (`palimpsest_render`)

### 8.1 Frame order

1. Shadow maps. 2. Depth prepass. 3. Opaque voxels (custom pipeline: greedy micro-voxel quads). 4. Opaque entities and instanced block props. 5. Cutout voxels and foliage. 6. Sky and fog. 7. Liquid surfaces and translucent geometry, sorted back to front by chunk. 8. Clustered decals. 9. Particles, gas volumes and fire (§8.7). 10. Viewmodel. 11. Post stack. 12. Anti-aliasing and upscaling. 13. Folio transition overlay. 14. UI.

### 8.2 Voxel chunk meshing

- **Micro-voxel grid.** Rendering meshes the 250 mm micro-voxel masks of Canon §4.4 with binary greedy meshing (an in-house port of R4 §4.2). A chunk of 32³ cells is 128³ micro-voxels. The mesher works on 64 **sections** of 8 × 8 × 8 cells, 32³ micro-voxels each, with section index `sx + 4 × (sz + 4 × sy)` for `sx, sy, sz ∈ 0..=3`. Each section meshes from a 34³ padded micro-voxel copy: the section plus a 1-micro-voxel border from its neighbours across section and chunk edges, so ambient occlusion sees diagonal neighbours. Each occupancy column is one `u64` with 34 bits used.
- A cell's mask is its entry in `ChunkView.masks` when it has one, else its shape's predefined mask from doc 03's per-state tables. `door`, open `trapdoor`, `ladder`, `cross` and `fixture` cells are not meshed: they render as instanced glTF props chosen by `(type, props)` from the block's skin record, and a `cross` without a prop mesh uses two crossed cutout quads. Their masks still drive collision, light and sight in the simulation.
- Opaque, cutout and translucent faces mesh separately. Greedy merging joins faces only when the merge key `(material, AO corners, render class)` is equal, and never across a section edge, so a quad spans at most 32 micro-voxels (8 m). Translucent faces are not merged.
- Packed quad, 8 bytes, in micro-voxel units inside the chunk:

| Word | Bits | Field |
|---|---|---|
| 0 | 0–6, 7–13, 14–20 | x, y, z (0..=127) |
| 0 | 21–25, 26–30 | width − 1, height − 1 (0..=31) |
| 1 | 0–11 | render material `m = 2 × type_index + (1 for ±Y faces, else 0)`, `0..=4095` (doc 03 §4.2 type index `0..=2047`) |
| 1 | 12–19 | AO, 2 bits per corner c0..c3 |
| 1 | 20–21 | render class: 0 opaque, 1 cutout, 2 translucent |
| 1 | 22–24 | face: 0 +X, 1 −X, 2 +Y, 3 −Y, 4 +Z, 5 −Z |
| 0, 1 | rest | 0 |

  Example: the merged top face of the full cells x 3..=6, y 7, z 12..=13 (section 4) is x 12, y 31, z 48, width 16, height 8, face +Y; with material 517 (type 258, top face), AO (3, 3, 2, 1) and class opaque it packs as word0 `0x1DEC0F8C`, word1 `0x0086F205`.
- **Face frame and AO.** `(x, y, z)` is the minimum micro-voxel of the merged region; the face lies on that micro-voxel's own side (the plane `x + 1` for +X). Width runs along `u` and height along `v`: ±X faces use `(u, v) = (z, y)`, ±Y faces `(x, z)`, ±Z faces `(x, y)`. Corners are c0 `(u0, v0)`, c1 `(u1, v0)`, c2 `(u1, v1)`, c3 `(u0, v1)`. For each corner, with `s1`, `s2` the two edge neighbours and `c` the diagonal neighbour in the micro-voxel layer in front of the face (1 when that micro-voxel is occupied by a block of render class opaque), `ao = 0 if s1 = s2 = 1, else 3 − (s1 + s2 + c)`. The quad uses the triangles `[0, 1, 2, 0, 2, 3]`, or `[1, 2, 3, 1, 3, 0]` when `ao0 + ao2 < ao1 + ao3`, so the darker diagonal never splits the quad. Examples: a lone +Y face whose −X neighbour above is occupied gets AO (2, 3, 3, 2) and keeps `[0, 1, 2, 0, 2, 3]`; a lone +Y face whose only occupied neighbour above is the (−X, −Z) diagonal gets AO (2, 3, 3, 3) and uses `[1, 2, 3, 1, 3, 0]`.
- The vertex shader pulls `quads[vertex_index / 6]` and the corner from `[0, 1, 2, 0, 2, 3]` or `[1, 2, 3, 1, 3, 0]` (chosen from the quad's AO bits) at `vertex_index % 6`. No index buffer.
- Each section stores its quads grouped by face direction. A chunk draws each direction with one multi-draw-indirect call over its 64 sections. The CPU culls a direction when every face in the chunk is back-facing: +X faces are culled when `camera.x < chunk_min_x + 0.25 m`, −X faces when `camera.x > chunk_min_x + 31.75 m`, and likewise for Y and Z.
- Meshing runs on Bevy's async compute pool. A changed cell dirties its section and every section whose 1-micro-voxel border holds it; dirty sections queue by camera distance, and at most 32 section remeshes start per frame. Budget: 25 µs per section on Steam Deck (`perf.deck.mesh.section_us`). Folio load meshes all 8,192 sections (128 chunks × 64) while the transition plays: at most 204,800 µs of work, which the 3 compute workers finish in at most 68,267 µs.
- **Debris.** Removed micro-voxels emit render-only debris (Canon §4.4): when a new `ChunkView` clears bits of a cell's mask, the client spawns one 250 mm cosmetic rigid-body cube per cleared micro-voxel, at most 16 per cell change, inside the cap of 256 cosmetic rigid bodies (§15), oldest removed first. Debris never feeds back (Canon §22). Doc 18 textures the newly exposed faces.

### 8.3 Per-genre block materials

Each genre ships one `texture_2d_array` of up to 256 layers at 64 × 64 px, BC7 in KTX2 with full mip chains (7 levels, 64 px down to 1 px; 5,488 bytes per layer; doc 18 §10.2). One layer tiles 1 m: shading samples it in world space by triplanar mapping at 64 texels per meter (Canon §4.4), so ±X faces sample the fragment's `(z, y)`, ±Y faces `(x, z)` and ±Z faces `(x, y)`, a 250 mm micro-voxel face shows 16 × 16 texels, and greedy merging never changes the texture. The voxel material binds five arrays: the Folio genre (slot 0) and up to four bleed genres (slots 1 to 4, `FolioView.bleed` order). Five full arrays take 5 × 256 × 5,488 = 7,024,640 bytes, inside the 32 MiB block-array budget of `perf.deck.mem.vram` (§15). Doc 12 §13.1 yields one entry genre plus at most three drift genres (Oneiric with the Marginalia bleeding modifier), so four bleed slots always suffice. A storage buffer `layer_of[slot][m] -> u16` maps each render material to the layer of that block's skin in that genre: the block record's `skins` entry for the genre suffix, else the material default (doc 03 §4.5, doc 18 `art.block.*` names the `side` and `top` layers). The quad carries only the render material, so a genre change never remeshes. Normal and roughness data come from per-layer material parameters (doc 18), not extra textures.

### 8.4 First-person viewmodel

- World camera: horizontal FOV 90° by default (setting 60–120°; doc 17 §8.4 owns the setting), converted to vertical FOV `2·atan(tan(h/2)·H/W)`: at 16:10, 64.01° for 90°, 39.68° for 60° and 94.54° for 120°. Near plane 0.05 m.
- Viewmodel: `RenderLayers::layer(1)`, a second `Camera3d` with `order = 1`, vertical FOV 54°, near plane 0.01 m, colour kept and depth cleared to 0.0 (reverse-Z). It samples the light field at the eye, so it is exactly as dark as the player's cell.
- Full-body awareness: the local avatar renders on layer 0 with head bones scaled to zero in first person; a shadow-only copy keeps the head's shadow. Third person restores the head and puts the camera 2.5 m behind on a spring arm colliding with the voxel grid.

### 8.5 Lighting and light agreement

- Dynamic lights use Bevy's clustered forward path: at most 64 visible point or spot lights on Steam Deck (128 desktop), of which 4 cast shadows (256² cube maps). Sun and moon use 2 cascades at 1024², reaching 48 m on Deck.
- The **light field** is one `Rg8Unorm` 3D texture of 256 × 64 × 256 texels (8 MiB) covering the Folio box: R = `light_sky × 17`, G = `light_block × 17` (Canon §1.2 two-channel light). Chunks whose `ChunkView.light` changed upload as 32³ slabs of 64 KiB, at most 16 per frame, newest generation first. Before upload, each texel's two channels get doc 03 §10.6's read, exactly as every simulation read does (Canon §2.3): `min(max(0, stored − dim), m)` per channel, where `dim` is 8 for a cell in `ViewFrame.shadow_cells` and 0 otherwise, and `m` is the smallest `max_level` of the active `ViewFrame.light_overrides` entries that contain the cell (inside `lo..=hi` and, when `sphere` is set, with its centre within the sphere's radius), or 15 when none does. The slabs an override or a shadow area covers are re-uploaded on the frame it starts and on the frame it ends.
- The **bleed field** is one `Rgba8Unorm` 3D texture of the same size (16 MiB): channel `k − 1` is 255 where `FolioView.slot_of_cell` equals bleed slot `k` (`k = 1..=4`), else 0. It uploads once at Folio entry while the page turn plays.
- Shading samples the light field trilinearly at `P + 0.5 m × N`: at the centre of a face of a full cell this is exactly the air cell in front, so the face reads that cell's simulation light and blends smoothly across the face. `L = max(R, G) / 17` is the effective light (Canon §2.3 `light_at`).
- **Agreement clamp.** Ambient luminance is `ambient(L)` and the final pre-tonemap luminance is clamped to at most `cap(L)`, both interpolated between the integer entries below (content values in milli-units, exposure-normalized):

| L | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| `cap_milli` | 20 | 24 | 37 | 59 | 90 | 129 | 177 | 233 | 299 | 373 | 456 | 547 | 647 | 756 | 874 | 1000 |
| `ambient_milli` | 5 | 6 | 9 | 15 | 22 | 32 | 44 | 58 | 75 | 93 | 114 | 137 | 162 | 189 | 218 | 250 |

  `cap_milli(L) = round_half_even(20 × 225 + 980 × L², 225)`, that is `20 + 980 × L² / 225` rounded half to even; `ambient_milli = round_half_even(cap_milli × 250, 1000)`. Units are thousandths of scene-linear luminance 1.0 after the genre's fixed exposure; auto-exposure is off. The clamp scales lit RGB by `min(1, cap / Y)`, which keeps hue. Because the clamp applies after direct lighting, an unshadowed lamp behind a wall cannot brighten a cell the simulation calls dark. Emissive surfaces, particles and fire are added after the clamp, so a lamp or a pair of glowing eyes stays visible in a dark room without lighting it. Stealth darkness and visual darkness therefore agree by construction.
- **Global illumination.** The light field is the baseline indirect light. Voxel AO is baked per quad corner. SSAO is off on Deck and on at the High preset. Bevy Solari path tracing is the Ultra preset on ray-tracing GPUs (`flag.render.solari`), and its output passes through the same clamp. No hardware ray tracing on Deck (R4 §2.3).

### 8.6 Post-process stack

Order: bloom, AgX tonemapping, 32³ colour-grading LUT (KTX2, one per genre), ink outline (Sobel on depth and normals, `outline_px`), halftone or posterize, chromatic aberration, scanlines, vignette, film grain, ordered dither. Parameters come from doc 12's GPV `render_style` and `palette` and from doc 18's per-genre record `art.post.<genre_suffix>` (LUT, bloom, `outline_px`, halftone, aberration, scanlines, vignette, grain, dither, `anim_step`, transition style), all integers in milli-units. Render styles `pbr`, `painterly`, `toon`, `ink`, `pixel_dither` and `xerox_collage` are shader variants of one material. A foreign entity renders with its `origin_genre` style and animation step, so a visitor from another genre is visible at a glance (R6). Comfort settings (doc 17) disable chromatic aberration, grain, scanlines, vignette, camera shake and Dutch angles.

### 8.7 Anti-aliasing, upscaling, particles, decals

- Deck preset: native 1280 × 800, 60 fps, SMAA High. Deck Battery: 40 fps cap, render scale 0.75 plus contrast-adaptive sharpening. Desktop: TAA with sharpening, or DLSS on RTX through the Vulkan backend (`flag.render.dlss`). No frame generation.
- **Particles** (in-house, §2.2): one storage buffer of 32-byte particles `{ pos: vec3<f32>, life_left: f32, vel: vec3<f32>, packed: u32 /* palette index 8 bits, size in 4 mm steps 8 bits, sprite 8 bits, blend 8 bits */ }`: 32,768 on Deck, 131,072 on desktop. The CPU writes at most 1,024 spawn requests per frame `{ emitter: u16, count: u16, pos, dir, seed: u32 }`; a compute pass allocates slots from a ring (oldest overwritten), then integrates `vel += g × gravity_bp / 10000 × dt; vel −= vel × drag_bp / 10000 × dt; pos += vel × dt; life_left −= dt`. A second pass draws camera-facing quads by vertex pulling, premultiplied or additive by the `blend` byte, with soft depth fade against the prepass. Particles do not collide. Doc 18 `vfx.*` emitters give rate, lifetime ticks, speed mm/s, spread Bam, `gravity_bp`, `drag_bp`, size mm and palette as integers.
- **Liquids, gases, fire.** Each chunk's `FluidView` builds a translucent liquid mesh: per cell the dominant liquid (most units, ties to lower `id32`) and surface height `y + min(1000, Σ units) / 1000` m, merged greedily by equal height and liquid, with a scrolling normal map from the liquid's doc 18 record. Gas cells with concentration ≥ 500 draw one volumetric billboard per cell, opacity `conc / 10000 × 0.6`, at most 4,096 per frame nearest first. Each fire cell drives one fire emitter scaled by intensity 1 to 3.
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

For a fragment at world position `P` (metres), with `w1` to `w4` the trilinear weights from the bleed field, `n ∈ [0, 1)` from a tiling 64³ value-noise texture sampled at `P × 4`:

```
t_k   = smoothstep(0.35, 0.65, w_k + (n − 0.5) × 0.5)          k ∈ {1, 2, 3, 4}
s     = Σ t_k;  if s > 1 { t_k = t_k / s; s = 1 }               // corners where bleed zones meet
t_0   = 1 − s
albedo = Σ_{k=0..4} t_k · A[layer_of[k][m]]
ink   = any |t_k − 0.5| < 0.04 for k ≥ 1  →  albedo × 0.6      // a ragged ink line on the boundary
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

- **Engine.** `firewheel` 0.14.0 driven by `palimpsest_audio`'s own thin plugin (§2.4), with `audionimbus` (Steam Audio 4.8.1) for binaural HRTF and reflection reverb. Opus streams decode on a worker into Firewheel sample resources. Buses: master → music, sfx, ambience, voice, ui. Voice cap 64 on Deck, 128 on desktop, divided into doc 18 §6.6's pools (`aud.mix.default`), whose sizes sum to the cap. Each pool owns its voices and never borrows: a request takes a free voice of its pool; a full priority-7 pool (`voice`, `stealth`) stops its oldest voice; any other full pool replaces its quietest voice only for a strictly louder request and otherwise drops the request, so a combat sound never takes a stealth-critical voice.
- **Two acoustic systems.** The simulation's sound grid (doc 03 §11, doc 09) decides what NPCs hear and also sets loudness and occlusion, so the player hears what enemies hear. Steam Audio's distance attenuation and occlusion are off to avoid double attenuation; HRTF and reflections stay on.
- **Occlusion of `ev.noise`.** The payload (doc 03 §14.2) gives the source level `L_src` dB and, per player slot `0..=4` (slot 4, the invader, is played like any party slot), `heard[slot] = Some((L_lis, arrival_dir))`: the level at the listener's eye cell and the unit vector from that cell toward the propagation path cell up to 4 hops back. `None` means the slot did not hear it, and the sound is not played for that player. The sim's sound costs 20 ddB per face step and 28 per edge step (doc 03 §11.2), so open air loses 2 dB per metre rather than 6 dB per doubling. The audio engine therefore measures occlusion against the same metric:

```
open_cost(Δ)  = with |Δx|, |Δy|, |Δz| in cells sorted a ≥ b ≥ c, s = a + b + c, E = min(floor(s / 2), b + c):
                28 × E + 20 × (s − 2E)               // N18 open-air path cost in ddB, source cell to eye cell
att    = L_src − L_lis                                  // total attenuation, applied as gain (dB)
free   = ceil_div(open_cost(Δ), 10)                     // what open air alone would cost
extra  = max(0, att − free)                             // dB lost to walls, doors and liquid
cutoff = max(500 Hz, 20000 Hz × 10^(−extra / 40))       // low-pass
pos    = extra ≥ 6 ? eye + arrival_dir × min(d, 4 m) : source position     // d = straight-line distance
```

  `open_cost` equals a Dijkstra over open N18 neighbours for every offset within ±8 cells (checked by a unit test). Example: a 130 dB shot 10 cells down a straight corridor (`Δ = (10, 0, 0)`, `open_cost` 200) behind a closed oak door is heard at 95 dB (doc 03 §11.5): `att = 35`, `free = 20`, `extra = 15`, cutoff 8,434 Hz, and the sound appears 4 m away in the direction it arrived from, the doorway. Without the door it is heard at 110 dB: `extra = 0`, no filter, true position. An open-air diagonal `Δ = (6, 3, 2)` costs 160 ddB, is heard at 114 dB, and gives `extra = 0`.
- **Other positional sounds** (impacts routed by §7.6 without an `ev.noise`, ambience emitters, debris) use a presentation estimate on the client's `ChunkView` blocks: `n` = cells whose micro-voxel mask has at least 48 of 64 bits set (opaque, Canon §4.4) or that hold a closed door, crossed by `dda` (§4.6, with `max_steps = 3 × ceil(d) + 3` for the eye-to-source distance `d` in metres) from the listener's eye to the source, endpoints excluded; `extra = min(24, 6 × n)`; the same cutoff formula; gain from the asset's own distance curve. It has no gameplay effect.
- **Music stems.** Each genre track (doc 18) has five sample-aligned looping stems of equal length: `bed`, `pulse`, `tension`, `combat`, `apex`. Intensity level selects the audible set: 0 explore {bed}; 1 suspicion {bed, pulse}; 2 search {bed, pulse, tension}; 3 combat {bed, pulse, tension, combat}; 4 boss or hunter {all}. `ViewPlayer.music_level`, computed in `extract_view` from simulation state: 4 if an entity of `rank.boss`, `rank.editor` or `rank.hunter` (an invader included, Canon §17.9) is alive and holds `aware.alert` toward any party member; else 3 if the local player is `in_combat` (Canon §2.3); else 2 if a hostile within 32,000 mm of the local player holds awareness of doc 09 §6.1 ordinal 3 or more (`aware.searching`, `aware.lost`, `aware.alert`) toward any party member; else 1 if such a hostile holds ordinal 1 or 2 (`aware.curious`, `aware.suspicious`); else 0. The 32 m radius keeps the music from acting as a Folio-wide detector of unseen guards. Rises apply at the next bar; falls apply at a bar boundary after the lower level has held for 8 s. Stems fade in or out over one beat. Inside a bleed zone the bleed genre's track crossfades in over 2 bars at the same level.
- **Formats.** Ogg Opus, 48 kHz: effects 96 kbit/s mono; stems 128 kbit/s stereo, streamed.

## 10. Input, UI and localization

- **Input.** An in-house action layer in `palimpsest_ui` maps keyboard, mouse and gamepad (Bevy `gilrs` under Steam Input) to doc 17's `input.*` actions. Each render frame adds look deltas to the look state `yaw_q: u32` (2^32 per turn, wrapping) and `pitch_q: i32` (65,536 per Bam, clamped to `±16000 × 65536`) with `look_add` (§7.5); each sim tick builds one `PlayerCommand` (§5.5) with `yaw = look_to_bam(yaw_q)` and `pitch = ((pitch_q + 0x8000) >> 16) as i16`, and the camera renders from the same quantities. The look state never drifts, because it is never re-derived from floats. It is in-house because it must quantize into the canon command and no input crate had a Bevy 0.20 release on 2026-10-11.
- **UI.** Game UI is `bevy_ui` with custom widgets (`PalButton`, `PalList`, `PalGrid`, `PalTabs`, `PalSlider`, `PalTooltip`, `PalItemCard`, `PalModal`), gamepad-navigable through Bevy's directional focus navigation and themed per genre by doc 17's `ui.*` skins. Layout is authored at 1280 × 800 and scales by `min(w / 1280, h / 800)`; body text ≥ 12 px. Every widget publishes an AccessKit node through `bevy_a11y` (doc 17 §13.6), and `access.narrator` speaks the same strings through `tts` (§2.2). Developer tools (inspector, grid overlays including the sound field, schedule timings) use `bevy_egui` behind `dev_tools` only.
- **Localization.** Fluent files `assets/loc/<locale>/<owner_doc>.ftl`. Fluent identifiers cannot contain dots, so a text key maps to a message ID by replacing each `.` with `-` (`loc.arch.melee.blade_long.skin.wuxia.name` → `loc-arch-melee-blade_long-skin-wuxia-name`); text keys contain no `-` (Canon §1.10), so the map is reversible. Fallback: selected locale, then `en-US`. Numbers follow Canon §1.5 display rules with locale grouping. CI screenshot tests run the pseudo-locale `en-XA` (accented, 30% longer). Latin-extended and CJK fonts ship; doc 18 picks launch locales.

## 11. Content pipeline

### 11.1 Files and schemas

- Content lives in `content/<owner_doc>/<namespace>/<name>.json`. Each file is `{ "schema": "<schema id>", "records": [ … ] }`; every record has an `"id"` field (Canon §1.1).
- Rust record types in `palimpsest_data` derive `JsonSchema` (schemars 1.2.2, feature `schema`), which emits JSON Schema 2020-12 with `"$schema": "https://json-schema.org/draft/2020-12/schema"`. `pal schema export` writes `schemas/*.schema.json`; CI fails if the committed schemas differ from the generated ones.
- **Validation** runs in CI (`pal validate`) and at every load: (1) a token pre-pass rejects any number token containing `.`, `e` or `E` (Canon §1.6) and any integer outside `−(2^53 − 1) ..= 2^53 − 1`; (2) `jsonschema` 0.58.6 validates against the schema; (3) semantic checks: ID grammar, every referenced ID exists, ownership of the namespace by the file's owning document (Canon §19), no base record with a second segment starting `x_` and every mod record inside its own `x_<mod_id>` segment (Canon §1.1), no retired ID (Canon §17.7), weights and shares that must sum to 10000, `Σ c_i ≤ 15000` in weapon scaling (Canon §6.4 row 12), and each owner's declared rules, such as doc 07's `recipe_overlap` rules. `pal validate` runs doc 19 §4's stages with its error codes and matches doc 19's reference validator `tools/validate_content.py` case for case.
- **ID registry.** The loader computes `id32 = fnv1a32(id)` for every record and alias and rejects collisions (Canon §1.1); mods get error `id32_collision`.

### 11.2 Content hash and presentation hash

Canon §2.4 splits loaded content into two hashes. `content_hash` covers everything the simulation reads; peers, saves and replays must match it. `presentation_hash` covers the presentation namespaces `art.`, `aud.`, `vfx.`, `nar.` and `ui.`, the localization tables and the presentation-only mods (§13.1); replays record it, and a mismatch only warns, so co-op peers may differ in art, audio, VFX, narration, UI and localization (DECISIONS.md item 87).

```
content_hash = BLAKE3(
    "PALIMPSEST/CONTENT/1\n"
    ‖ "REC" ‖ u32le(n) ‖ for each record outside art., aud., vfx., nar., ui., in ascending id bytes:
                          u32le(len(id)) ‖ id ‖ u32le(len(body)) ‖ body
    ‖ "UNL" ‖ u32le(n) ‖ for each unlock id ascending: u32le(len) ‖ id
    ‖ "MOD" ‖ u32le(n) ‖ for each simulation-affecting mod ascending by mod id: u32le(len) ‖ mod_id ‖ u32le(len) ‖ version ‖ BLAKE3(mod blob)
    ‖ "FLG" ‖ u32le(n) ‖ for each sim-affecting flag ascending: u32le(len) ‖ id ‖ i64le(value))
presentation_hash = BLAKE3(
    "PALIMPSEST/PRESENTATION/1\n"
    ‖ "REC" ‖ u32le(n) ‖ for each record in art., aud., vfx., nar., ui., in ascending id bytes:
                          u32le(len(id)) ‖ id ‖ u32le(len(body)) ‖ body
    ‖ "LOC" ‖ u32le(n) ‖ for each file under assets/loc/ in ascending relative-path bytes: u32le(len(path)) ‖ path ‖ BLAKE3(file bytes)
    ‖ "MOD" ‖ u32le(n) ‖ for each presentation-only mod ascending by mod id: u32le(len) ‖ mod_id ‖ u32le(len) ‖ version ‖ BLAKE3(mod blob))
body = RFC 8785 canonical JSON of the record after defaults are applied (sorted keys, no whitespace)
```

Example: two records of a test schema without defaults, `{"id":"crate.sim","layer":3}` and `{"id":"crate.math","layer":0}`, with no unlocks, mods or flags give a 141-byte input and `content_hash = 92c159a0a347070d679d6edd626a8bc6938565f60204dd732ba14c2ac3592cc9`. An empty presentation set gives a 47-byte input and `presentation_hash = 596015384dbe4c15a8a14de247b103c5b2ab6640314cac114650a0a476af559b`; adding one localization file at relative path `en-US/02.ftl` with the 13 bytes `pal-test = x\n` gives a 95-byte input and `presentation_hash = 08ea33804f578a4904f58633637d4633498201421eb29186905fdf54092072d1`, while `content_hash` is unchanged.

### 11.3 Hot reload

With `flag.dev.hot_reload`, `notify` watches `content/` and `assets/`; a changed file reloads, revalidates and swaps the `Arc<ContentDb>` between frames. During an active run, release builds refuse with `content_reload_refused_in_run` (Canon §2.4); dev builds apply the change, log the new `content_hash` and mark the session replay invalid. Assets hot-reload through Bevy's file watcher in dev builds.

### 11.4 Assets

| Kind | Format | Notes |
|---|---|---|
| Meshes, skeletons, animations | glTF 2.0 binary `.glb` | Bevy loader; presentation only |
| Hitbox pose tables | JSON content, integer mm, keyframes per doc 03 §5.16 | baked from `.glb` by `pal bake-poses` (Canon §9) |
| Textures, texture arrays, LUTs | KTX2, BC7 with zstd supercompression | Bevy `ktx2`, `zstd_rust` |
| Audio | Ogg Opus 48 kHz | §9 |
| Fonts | OTF | |
| Prefab voxel structures | MagicaVoxel `.vox` → JSON content | `pal import-vox` |

Asset paths follow content IDs owned by doc 18: an ID `art.mesh.enemy.gothic_horror.ghoul` (doc 18's example mesh) resolves to `assets/art/mesh/enemy/gothic_horror/ghoul.glb`: every dot becomes a directory separator and the asset kind supplies the extension.

| ID prefix | File | Notes |
|---|---|---|
| `art.mesh.` | `.glb` | meshes, skeletons and their animations |
| `art.anim.` | none | hitbox pose tables are JSON content records (doc 03 §5.16) |
| `art.blockarray.<genre_suffix>` | `.ktx2` | one block texture array per genre (§8.3) |
| `art.tex.`, `art.lut.` | `.ktx2` | 2D textures; 32³ colour-grading LUTs |
| `art.icon.` | `.ktx2` | icon textures (doc 18) |
| `art.font.` | `.otf` | |
| `aud.` | `.ogg` (Opus) | effects and music stems (§9) |
| `vfx.`, `art.post.`, `art.block.` | none | JSON content records |
| `loc.` text keys | `assets/loc/<locale>/<owner_doc>.ftl` | §10 |

### 11.5 Generated tables

`tools/gen_curves.py` produces the knots of doc 13 §4.4 `Table` curves. Each entry of its input `tools/curves_src.json` names the curve (`curve`, a doc 13 `ai.curve.` ID that this tool only references), a function `fn` and integer `params`; with `x = 625 × i / 10000` for `i = 0..=16`, the generator writes `k[i] = clamp(round_half_even(10000 × f(x)), 0, 10000)`, computed with mpmath at 50 digits, into `content/13/ai/curve_tables.json`:

| `fn` | `params` | `f(x)` |
|---|---|---|
| `logistic` | `[x0_bp, k_centi]` | `1 / (1 + e^(−(k_centi / 100) × (x − x0_bp / 10000)))` |
| `gaussian` | `[c_bp, w_bp]` | `e^(−((x − c_bp / 10000) / (w_bp / 10000))²)` |
| `knots` | 17 integers | the knots themselves, validated to lie in `0..=10000` |

```json
[
  { "curve": "ai.curve.logistic_up", "fn": "logistic", "params": [5000, 1000] },
  { "curve": "ai.curve.bell_mid", "fn": "gaussian", "params": [5000, 2500] },
  { "curve": "ai.curve.plateau", "fn": "knots", "params": [3000, 5000, 7000, 9000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 8000, 4000, 0] }
]
```

These three entries reproduce doc 13 §4.4's shipped tables exactly: `logistic_up` gives `[67, 124, 230, 421, 759, 1330, 2227, 3486, 5000, 6514, 7773, 8670, 9241, 9579, 9770, 9876, 9933]` and `bell_mid` gives `[183, 468, 1054, 2096, 3679, 5698, 7788, 9394, 10000, 9394, 7788, 5698, 3679, 2096, 1054, 468, 183]`. CI reruns both generators (`gen_trig.py`, `gen_curves.py`) and fails if the committed outputs differ.

## 12. Save system

### 12.1 Container

Run saves, the meta profile, replays and desync dumps share one container. All integers are little-endian.

| Offset | Size | Field |
|---|---|---|
| 0 | 4 | magic `PALS` |
| 4 | 2 | `container_version` = 1 |
| 6 | 1 | kind: 1 run save, 2 meta profile, 3 replay, 4 desync dump |
| 7 | 1 | flags: bit 0 zstd, bit 1 signed |
| 8 | 4 | `sim_version` |
| 12 | 32 | `content_hash` |
| 44 | 8 | `run_seed` (0 for the meta profile) |
| 52 | 8 | boundary tick |
| 60 | 8 | uncompressed payload length |
| 68 | 8 | stored payload length |
| 76 | 32 | BLAKE3 of the stored payload |
| 108 | 4 | first 4 bytes of BLAKE3(bytes 0..108) as `u32` |
| 112 | n | payload: zstd frame of the postcard payload (below) |
| 112 + n | 64 | Ed25519 signature over bytes `0..112+n`, when flag bit 1 is set |

The header check is `u32::from_le_bytes(BLAKE3(bytes 0..108)[0..4])`. Payloads by kind, each a postcard value:

| Kind | Payload |
|---|---|
| 1 run save | `(checkpoint_version: u16, RunCheckpoint, sim: Vec<u8> /* §6.5 serialization */, log: ReplayLog)`. `RunCheckpoint` holds doc 14's score accumulators, `RunSummary`, mode, grade, Folio index, party Echo IDs and ranked state (`save_seq`, `restores`). It comes first so any later build can decode it with `postcard::take_from_bytes` without understanding the rest. |
| 2 meta profile | `(profile_version: u32, Profile)` (§12.4) |
| 3 replay | `ReplayFile` (doc 16 §10.1): session header, run ID, `RunConfig`, roster, the CIL from tick 0, checkpoints, Folio states for seeking, and the run-end record |
| 4 desync dump | §6.7 contents |

`ReplayLog` is doc 16 §6.14's command-and-intent log (`Vec<CilTick>`): per tick, the applied commands of slots `0..=4` as deltas against each slot's previous command, then the intents and session intents with their applied ticks. Decoding yields exactly the Canon §2.4 records. Run saves and replays use the same encoding.

**Loading checks**, in order: magic, header check, payload length, payload BLAKE3, signature when flag bit 1 is set, then the version checks of the kind. Kinds 1 and 3 require equal `sim_version` and `content_hash`; a replay whose recorded `presentation_hash` differs from the build's loads with a warning (Canon §2.4). Kind 2 ignores both, because the profile migrates through `profile_version` (§12.4). Kind 4 records them for diagnosis only. Each failure has its own error code (`bad_magic`, `header_corrupt`, `payload_truncated`, `payload_corrupt`, `bad_signature`, `sim_version_mismatch`, `content_mismatch`).

**Signatures.** Only `palimpsest_services` sets flag bit 1, on files it issues (a server-verified replay for leaderboard download), with doc 15's receipt keys (§4.6 key windows). The client never signs: a key shipped in the client protects nothing. Entitlements in the profile are proven by the server-signed receipts it contains (§12.4).

### 12.2 Writing

zstd level 3 for run saves and the profile, level 9 for replays. Every write goes to a temporary file in the same directory, is flushed with `fsync`, renamed over the target, and the previous file is kept as `.bak`. Steam Auto-Cloud syncs `profile/` and `runs/`. Budget: 50 ms on Steam Deck for a run save (`perf.deck.save_ms`). The client keeps the replay (kind 3) of every run it reported to doc 15's economy service for 30 days and uploads it within 7 days of a verification request; a missing upload voids the report (doc 15 §10.4).

### 12.3 Run suspend saves

Write points follow doc 14 §12.1: single player at every Folio entry, every `AUTOSAVE_PERIOD_TICKS` in non-ranked runs, on Suspend and on quit; co-op on the host at Folio entry only (Canon §2.5). A run resumes only when `sim_version` and `content_hash` match the build. Otherwise the game offers the Steam branch `prev` (previous build, kept 30 days) or "Close the Book". Close the Book never loads the simulation: the meta layer decodes the save's `RunCheckpoint` and ends the run as `run.end.abandoned_version` (doc 14 §4.3), scored and paid from that record. While slot 4 holds an invader, a single-player Suspend request is deferred and completes on the tick `ev.invasion_ended` is emitted (Canon §2.5). `SIM_VERSION` increments in any commit that can change any state hash for any input: sim code, canon formulas, RNG use or serialization layout.

### 12.4 Meta profile

The payload holds `profile_version: u32` and the account cache: unlocks, the Glyph Library (Canon §17.1 `GLYPH_LIBRARY_START`, DECISIONS.md item 33), Codex progress, Echo roster, settings references, the Margin trophy and decoration slot assignments (Canon §17.8), and every doc 15 receipt received. The online backend is authoritative; offline, the owned set is the fold of the cached receipts in `seq` order, each verified by `palimpsest_econ::verify_receipt` against the embedded key windows (doc 15 §4.6). Migrations are a chain `migrate_v{n}_to_v{n+1}`; a fixture profile of every shipped version lives in `tests/fixtures/profile_v{n}.pals`, and CI loads each one through the chain. Profiles are never migrated downward. The rules that change the profile (unlocks, Bindery nodes, the Glyph Library, Codex knowledge, achievements, Grade unlocks; doc 14) live in `palimpsest_platform::meta`, which has no Bevy dependency and is tested headless.

## 13. Modding

### 13.1 Data packs first

A mod is `mods/<mod_id>/` with `mod.json` (`id` matching `^[a-z][a-z0-9_]{2,31}$`, `version`, `api: 1`, `depends`, `load_after`), `content/`, `assets/`, and `effects.wasm` when it has scripted effects. Content files live at `mods/<mod_id>/content/<NN>/<namespace>/<name>.json` and use the base schemas and validation (doc 19). Mod content IDs use second segment `x_<mod_id>`, for example `item.x_frostpack.ice_blade` (Canon §1.1). Load order: base, then mods topologically by `load_after`, ties by `mod_id`. Mods draw only from `rng.mod.<mod_id>.<name>` streams (Canon §3.7).

- **Adding, never overriding.** A mod record whose `id32` equals any loaded `id32`, an identical ID included, is rejected with `id32_collision` (Canon §1.1). A mod changes base content only through `patches/*.json` records `{ "target": "<base id>", "op": "append", "path": "/<array field>", "value": [ … ] }`, allowed only on array fields that the base schema marks `"x-patchable": true` (loot tables, spawn lists, recipe lists, shop stock). Patches apply in mod load order and are validated like any record; the patched record enters `content_hash`. Patch records live in `mods/<mod_id>/patches/<name>.json` under doc 19's `02.patch` schema, with IDs in the mod's own `x_<mod_id>` segment. Example: a patch appending one more item line, in doc 11 §9.5's line format, to doc 11's `loot.chest.plain`, whose only list is `lines`. A mod's own items enter loot through their doc 11 §2.1 template records and pools, not through patches.

```json
{ "id": "loot.x_frostpack.extra_chest_line", "target": "loot.chest.plain", "op": "append",
  "path": "/lines", "value": [ { "kind": "item", "chance_bp": 2000, "scaled": true } ] }
```
- **Forbidden namespaces.** Mod records in `crate.`, `flag.`, `perf.`, `cur.`, `ent.`, `juris.`, `banner.`, `pass.`, `store.`, `bundle.` or `cosm.` are rejected with `mod_namespace_forbidden`, so no mod touches the economy, jurisdiction gates or budgets. A mod record that names a retired ID (Canon §17.7) fails validation like base content (doc 19 `E_RETIRED`).
- **Simulation-affecting mods.** A mod is presentation-only when it has no `effects.wasm`, no patches, and every record it adds lies in `art.`, `aud.`, `vfx.`, `nar.` or `ui.`; Fluent files and assets are presentation. Every other mod is simulation-affecting, and a run with one loaded is an Apocrypha run (doc 15 §2.2). The loader derives the class; `mod.json` cannot declare it. A presentation-only mod enters `presentation_hash` and a simulation-affecting mod enters `content_hash` (§11.2), so co-op peers may differ only in presentation-only mods (DECISIONS.md item 87).
- **Mod blob.** `mod_blob` is the concatenation, over every file under `mods/<mod_id>/` in ascending relative-path bytes with `/` separators, of `u32le(len(path)) ‖ path ‖ u64le(len(data)) ‖ data`. A mod with no files has an empty blob. `BLAKE3(mod_blob)` enters the `MOD` section (§11.2) and `modset_hash` (§13.4).

### 13.2 WASM scripted effects

```rust
let mut cfg = wasmtime::Config::new();
cfg.wasm_component_model(true);
cfg.consume_fuel(true);                    // deterministic interruption
cfg.cranelift_nan_canonicalization(true);  // identical NaN bits on every platform
cfg.wasm_relaxed_simd(false);
cfg.wasm_threads(false);
cfg.epoch_interruption(false);             // wall-clock based, never used
```

- No WASI imports: no clock, filesystem, environment or entropy. The host interface carries integers only. Floats inside a module are allowed, since WASM float arithmetic is IEEE-exact and NaN bits are canonicalized.
- Fuel: 50,000 per hook call and 100,000 per mod per tick; each hook call receives `min(50,000, fuel left for that mod this tick)`. At most 8 scripted mods load. Wasmtime fuel counts WebAssembly operators, so the same program stops at the same instruction on every platform. Eight mods at their per-tick limit must finish within 0.8 ms, the worst budget of `perf.deck.sim.mods` (§15), so Cranelift code must average at most 1 ns per fuel unit on the Deck; a criterion gate runs a fuel-bound loop to check it. Linear memory is capped at 64 MiB and the stack at 512 KiB. On exhaustion the call traps: its queued commands are discarded and the mod is disabled for the rest of the run. The disabled flag lives in serialization section 11, so every peer disables it on the same tick.
- Scripted mods load in every client build: the Windows client, including Steam Deck under Proton, which runs x86_64 code natively, so Cranelift's generated code runs unchanged. Workshop distribution is Steam Workshop (§14, DECISIONS.md item 43).
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

Within one hook call, `rng-range(name, n)` uses one keyed generator `keyed_rng("rng.mod.<mod_id>.<name>", folio, ctx.actor, ctx.tick, ctx.seq)` per stream name and draws from it sequentially. `set-block` applies as an edit by `ctx.actor` through doc 03's `set_block` with an actor cause, so cell protection (doc 03 §4.7) and the Canon §4.3 rule hold: only `destructible` blocks and placeables change, and a refused edit returns nothing and queues nothing.

### 13.4 Multiplayer mod set

`modset_hash = BLAKE3("PALIMPSEST/MODSET/1" ‖ M)`, with `M` the `MOD` section of `content_hash` (the simulation-affecting mods, §11.2) after its tag, is shown in the lobby; a `MOD` section holding one mod `frostpack` version `1.0.0` with an empty blob gives a 77-byte input and `a4cd5df9faa66d265baf53d652ff672a1e3b76f2466877f8eaa64d45ea905817`. Because `content_hash` contains that section, peers with different simulation-affecting mod sets fail the Canon §2.4 handshake; presentation-only mods may differ (DECISIONS.md item 87). Invasions require an empty mod set (doc 16). A joining player missing a Workshop mod gets a subscribe prompt; a data-only pack up to 64 MiB also transfers from the host on the reliable channel.

## 14. Platforms

- **Steamworks** (`steamworks` 0.13.1): 4-member lobbies carrying `sim_version`, `content_hash`, `modset_hash`, mode and the `mode.invasion_open` flag (Canon §17.9); invaders find open sessions through doc 16's `svc.lobby` beacon search and pass the same handshake (doc 16); Steam Networking Sockets with Datagram Relay as production `Transport` (DECISIONS.md item 123); achievements, Auto-Cloud, Workshop, rich presence, Input glyphs, on-screen keyboard, `IsSteamRunningOnSteamDeck` for presets, and doc 15's microtransaction callback.
- **One storefront.** Steam is the only store through 1.0 (DECISIONS.md item 112, which replaces the Epic Games Store build of item 73), and the EOS SDK is dropped (item 123): there is no EOS binding, no Epic Games Store or itch.io build, and no demo or public playtest build (item 113).
- **Steam Deck Verified checklist:** every screen gamepad-navigable, including first launch, seed entry and lobby; Deck glyphs everywhere; on-screen keyboard for every text field; 60 fps at 1280 × 800 in the saturated scenario (§15); text ≥ 12 px; no launcher or external account; the Windows build runs under Proton (DECISIONS.md item 98); survives suspend (single player pauses when a wake gap exceeds 1 s; co-op uses doc 16 timeouts); no anti-cheat.
- **Desktop:** Windows x86_64 with DX12 by default and Vulkan as fallback. On Steam Deck (`IsSteamRunningOnSteamDeck`) the client selects wgpu's Vulkan backend, which Proton passes to the native driver, instead of DX12 through vkd3d-proton.
- **No other platform.** Through 1.0 there is no console, Switch, native macOS, native Linux or browser client (DECISIONS.md item 98); Linux builds exist only for the headless binaries (§2.1). The `Platform` and `Transport` traits and the `no_std` simulation crates keep a later port free of simulation changes.

## 15. Performance budgets

**Simulation tick on Steam Deck.** Reference: one Zen 2 core of the self-hosted runner `deck-01`, `dist` profile, host with 4 players, scenario `perf_saturation_4p`: every canon cap saturated (256 actors, 1,024 projectiles, 2,048 ground items) plus 64 light emitters and the liquid and gas run caps of doc 03 §6.3 and §7.3, measured over 3,600 ticks. Each group budget gives the mean over its due ticks and the worst single due tick, in µs. Rows for doc 03 systems copy doc 03 §17; the AI row is doc 13 §4.14's 1.5 ms host AI budget. The four systems doc 03 §2.2 lists beyond §17's rows (`sys.crafting`, `sys.resources`, `sys.coop`, `sys.run_layer`) sit inside the `input_actions` and `bookkeeping` budgets, so all 39 systems of `SCHEDULE` are covered.

| `perf.deck.sim.` ID | Systems (doc 03 §2.2) | Due ticks `t` | Mean µs | Worst µs |
|---|---|---|---|---|
| `bookkeeping` | `tick_begin`, `interest`, `spawn_admit`, `resources`, `coop`, `status_timers`, `erasure`, `despawn`, `run_layer`, `tick_end` | every | 100 | 250 |
| `input_actions` | `input_apply`, `actions`, `crafting` | every | 300 | 700 |
| `perception` | `perception` (doc 09; period 3 per entity) | every | 400 | 800 |
| `ai` | `ai_think` with squads, Director and path service (doc 13 §4.14; period 6 per entity) | every | 1,200 | 1,500 |
| `movement` | `movement` | every | 450 | 1,200 |
| `push` | `push`, `broadphase` | every | 80 | 300 |
| `env_sample` | `env_sample` | every | 100 | 200 |
| `pose` | `pose` | every | 30 | 50 |
| `hitscan_melee` | `hitscan`, `melee` | every | 200 | 600 |
| `projectiles` | `projectiles` | every | 150 | 500 |
| `explosions` | `explosions` (at most 4 per tick) | every | 20 | 1,400 |
| `damage` | `damage`, `damage_late` | every | 300 | 800 |
| `block_updates` | `block_updates` | every | 10 | 400 |
| `random_ticks` | `random_ticks` | every | 30 | 100 |
| `light` | `light_emitters`, `light` | every | 50 | 200 |
| `sound` | `sound`, `hearing` | every | 200 | 2,000 |
| `mods` | WASM hooks inside any system (§13.2) | every | 50 | 800 |
| `view_extract` | `extract_view` (§7.2), outside the schedule | every | 300 | 600 |
| `liquids` | `liquids` | `t ≡ 5 (mod 6)` | 400 | 1,600 |
| `gases` | `gases` | `t ≡ 2 (mod 6)` | 300 | 1,500 |
| `fire` | `fire` | `t ≡ 10 (mod 12)` | 50 | 800 |
| `temperature` | `temperature` on 4 threads through `OrderedMap` (doc 03 §2.5) | `t ≡ 25 (mod 30)` | 100 | 900 |
| `surfaces_status` | `surfaces`, `status_pulse` | `t ≡ 0 (mod 15)` | 350 | 700 |
| `needs` | `needs` | `t ≡ 53 (mod 60)` | 50 | 100 |
| `net_snapshot` | host snapshot encode (doc 16), outside the schedule | `t` even | 400 | 600 |
| `hash_snapshot` | `snapshot()` for the hash worker (§6.5) | `t ≡ 59 (mod 60)` | 500 | 500 |

Totals, recomputed over one 60-tick cycle: the every-tick groups sum to 3.97 ms at their means and 12.40 ms at their worsts. The mean tick is 4.33 ms. With every group at its mean, the costliest phase is `t ≡ 59` (liquids plus hash snapshot) at 4.87 ms. If every group hit its worst on the same tick, the costliest phases are `t ≡ 2 (mod 6)` (gases plus net snapshot) and `t ≡ 59` (liquids plus hash snapshot) at 14.50 ms, still 2.17 ms inside the 16.67 ms tick period, so the sum of the group budgets proves the simulation keeps real time.

| ID | Budget |
|---|---|
| `perf.deck.sim.tick_mean` | 5.0 ms over 3,600 ticks of `perf_saturation_4p` |
| `perf.deck.sim.tick_p99` | 8.0 ms over the same ticks |
| `perf.deck.sim.tick_worst` | 14.5 ms for any single tick of `perf_saturation_4p` and of `perf_spike_4p`, which adds 4 explosions, 32,768 settled sound cells and a full liquid run on one tick |
| `perf.deck.hash_ms` | 4.0 ms on a worker thread |
| `perf.deck.gen.folio_p50_ms` | 110 ms to generate one Folio (doc 12 §6.3) on 4 threads: the sim thread and 3 compute workers |
| `perf.deck.gen.folio_p99_ms` | 260 ms, same conditions |
| `perf.deck.gen.folio_max_ms` | 900 ms hard cap including 3 retries and the fallback (doc 12 §6.3) |
| `perf.deck.load.folio_ms` | 1,500 ms from exit choice to the first frame of the new Folio, hidden by the page turn |
| `perf.deck.mem.ram` | 6,144 MiB resident: sim and checkpoints 384, content 192, CPU-side assets 1,408, render CPU 768, audio 256, UI and fonts 128, net 64, mods 256, Scribe 1,024, code and stacks 512, headroom 1,152 |
| `perf.deck.mem.vram` | 2,560 MiB: entity textures 896, block arrays 32, chunk quads 64, light and bleed fields 24, render targets 160, shadow maps 48, skinned meshes 384, particles and decals 96, UI atlases 96, LUTs and misc 48, headroom 712 |
| `perf.deck.frame.gpu` | 14.0 ms: shadows 1.4, depth prepass 0.8, voxels 2.6, entities 2.0, viewmodel 0.5, liquids and translucent 1.0, particles 0.9, decals 0.3, post 1.6, AA 0.7, UI 0.5, transition or bleed 0.4, headroom 1.2 (sum 13.9) |
| `perf.deck.net.host_kbps` | 384 kbit/s host upstream in total (doc 16 §15): with 3 clients, 128 kbit/s per peer (96 kbit/s snapshots, 32 kbit/s reliable channels); with 3 clients and an invader, 96 kbit/s per peer |
| `perf.ci.bot_tps` | ≥ 6,000 ticks per second (100× real time) per run of scenario `bot_typical`: Standard mode, 2 Pilgrim bots, doc 13 director population, `EMIT_VIEW` off, `HASH_AT_60` on, one core of the GitHub `ubuntu-24.04` x86_64 runner. That is a mean tick of at most 166 µs. A 100 to 150 minute run takes 60 to 90 s, so the 200 nightly runs (§16) cost 4.2 core-hours at the 125-minute average. |

**Unit costs** (doc 03 §17), enforced by criterion benchmarks on `deck-01`: `perf.deck.sim.unit.body_move_ns` 3,000 per body; `.push_pair_ns` 100 per pair; `.pose_ns` 100 per entity; `.ray_cell_ns` 15 per visited cell; `.projectile_ns` 400 per projectile; `.explosion_us` 350 per explosion; `.block_update_ns` 200 per pop; `.random_tick_ns` 100 per pick; `.liquid_cell_ns` 100 per cell; `.gas_cell_ns` 90 per cell; `.fire_cell_ns` 200 per burning cell; `.thermal_cell_ns` 6 per awake thermal cell; `.light_pop_ns` 50 per pop; `.sound_cell_ns` 60 per settled cell.

Budgets are content records in `content/02/perf/perf.json`; `pal perf report` compares a run's measurements against them:

```json
{ "id": "perf.deck.sim.movement", "device": "deck", "scenario": "perf_saturation_4p", "metric": "per_due_tick", "mean": 450, "worst": 1200, "unit": "us" }
```

**Deck threads (4 cores, 8 threads):** main (Bevy schedule, input, UI, extraction), render, sim (host only), 3 compute workers (meshing, decode, hashing, worldgen), real-time audio.

**Caps beyond the canon (Canon §4 caps stand):** dynamic lights 64 visible and 4 shadowed, particles 32,768, gas billboards 4,096, decals 256, ragdolls 16, cosmetic rigid bodies 256 (ragdolls and micro-voxel debris), debris cubes 16 per cell change, audio voices 64, section remeshes 32 per frame, light-field uploads 16 chunks per frame, scripted mods 8.

## 16. Testing strategy

1. **Formula unit tests** from every worked example in every chapter, written before the implementation (§18).
2. **Property tests** (`proptest`): monotonicity and bounds, no overflow over declared ranges, weight tables summing to 10000, `isqrt_u128` equals core `isqrt`, `atan2_bam` within 1 Bam, `deserialize(serialize(x)) == x`.
3. **Golden replays and SyncTest** (§6.6–§6.7): fast set per pull request, full set nightly.
4. **Determinism CI** on five targets (§6.4).
5. **Content fuzzing**: `cargo-fuzz` targets for the JSON loader, save container, replay decoder, packet decoder and mod loader, plus a schema-aware record mutator; 30 minutes per target nightly.
6. **Headless bot playtests at 100×** (`perf.ci.bot_tps`): up to 4 utility-AI "Pilgrim" bots per run (`BotBrain`, §3.4) with `EMIT_VIEW` off; 200 full runs nightly. Three `BotBrain` policies ship: combat-led, stealth-led and environment-led; doc 01's `pillar.living_page.t3` sweep runs each policy over 200 seeds per Rubric type and core genre. Failure: a panic, a desync between two replicas, or no Folio exit for 36,000 ticks (600 s, above the 420 s Folio target of Canon §17.1). Reported: death-Folio histogram, ticks per Folio, Lucre and loot value per Folio, level and main-skill value per Folio.
7. **Balance Monte Carlo** (`pal balance`), all integer and keyed-RNG, run nightly:
   - **Folio-1 anchors.** 100,000 duels of each Calling's reference weapon (one per Calling, assigned by doc 05; Common Normal +0, skill 20, all sub-attributes 40) against `enemy.role.soldier` (doc 13: 60 HP, AR 10) must give time to kill 2.0 s ± 0.3 s and DPS 30 ± 4.5 HP/s (Canon §17.4 `REF_STANDARD_TTK_D1`, `REF_PLAYER_DPS_D1`).
   - **Item ceiling.** 1,000,000 generated items per Folio depth 1 to 25 never exceed `ITEM_MULT_MAX = 51870` Bp over a Common Normal +0 iron item of the same archetype and ilvl (Canon §13.8).
   - **Depth curve.** With a Common Normal +0 weapon at `ilvl = d`, the hits needed to kill a standard enemy of depth `d` grow as `H(d) / P(d)`: 1.00, 1.26, 1.58, 1.91, 2.23, 2.56 at `d` = 1, 5, 10, 15, 20, 25 (Canon §13.8 rows). Rarity, quality, enhancement, material and skill growth close that gap (docs 05, 11). The harness takes each bot's equipped weapon on arrival at those Folios from the nightly bot runs and fails when the median time to kill a standard enemy of that depth leaves 1.0 s to 4.0 s (0.5 to 2.0 times `REF_STANDARD_TTK_D1`).
   - **Pacing.** Bot runs reproduce doc 13 §11.2's table within ±1 level per Folio, reach level 28 ± 3 on arrival at Folio 25 (Canon §8) and the main skill reaches 50 at Folio 10 ± 2 and 75 at Folio 22 ± 3 (Canon §7.2).
8. **Client checks**: headless smoke test under 16 random Bevy `shuffle_seed` values, `en-XA` screenshot tests, agent checks through `bevy_brp_mcp`.
9. **Mutation testing** weekly on math, damage, status and RNG modules.
10. **Signature-moment scenarios** (`pal scenario`): every `tests/moments/sm_NN.json` of doc 01 §3.13 runs headless in `palimpsest_sim` on every pull request; a scenario passes when each of its assertions holds on the event stream.

## 17. CI/CD, flags, channels, telemetry

### 17.1 Pipeline

- **Pull request (≤ 25 min):** fmt, `clippy -D warnings`, `cargo deny`, `xtask check-deps`, `float-scan`, `lint-size`, `spec-refs`, schema check, `pal validate`, nextest (sim suite < 60 s), golden fast set, SyncTest, determinism matrix, signature-moment scenarios, `gen_trig.py --check` and `gen_curves.py --check`, the Windows client build and the Linux headless builds.
- **Nightly:** full golden set, generator sweeps (seeds 0–9,999 per genre, doc 12 validators), bot runs, fuzzing, Steam Deck gates on every `perf.*` budget, `dist` builds to the `nightly` branch.
- **Weekly:** mutation testing, coverage.

### 17.2 Release channels

| Channel | Audience | Promotion rule |
|---|---|---|
| `dev` | team, every merge to `main` | CI green |
| `nightly` | internal Steam branch | nightly suite green |
| `playtest` | invited testers on a password-protected Steam branch; no public playtest build (DECISIONS.md item 113) | 3 consecutive green nightlies |
| `beta` | public opt-in Steam branch | 7 days on `playtest` with no P0 and crash-free sessions ≥ 99.0% |
| `stable` | default Steam branch | 7 days on `beta` with crash-free sessions ≥ 99.5% |
| `prev` | previous stable, 30 days | automatic on each stable release (§12.3) |

Transitions: a build moves one channel up when its promotion rule holds; a P0 found on any channel rolls that channel back to its previous build within 24 hours, and on `stable` the rollback target is `prev`; nothing skips a channel. Versions are `MAJOR.MINOR.PATCH`; `SIM_VERSION` is independent (§12.3).

### 17.3 Feature flags

Flags are records in `content/02/flag/flags.json`: `id`, `kind` (`bool` or `int`), `default`, `channels` (where the value may differ from the default) and `sim_affecting` (if true, the value enters `content_hash`, §11.2). Cargo features choose compiled code per build (`dev_tools`, `steam`, `llm`, `solari`, `dlss`); flags choose behavior at runtime. Doc 15's `juris.` flags use the same loader, with doc 15's extra `strict` field.

```json
{ "id": "flag.mods.wasm", "kind": "bool", "default": true, "channels": ["dev", "nightly", "playtest", "beta", "stable"], "sim_affecting": true }
```

### 17.4 Telemetry and crash reporting

- Both are **opt-in**. First launch asks for usage statistics (default off) and crash reports (`ask each time` by default, `always`, `never`); settings change them at any time.
- A minor-flagged account (doc 15 §8.4; the `minor` flag of doc 16's sign-in) is never asked and sends nothing; crash reports for it stay local (doc 16 §14).
- Identity is a random 128-bit install ID, generated locally, never derived from a Steam ID, resettable. No names, chat, IP addresses (dropped at ingest), hardware serials, advertising IDs or third-party SDKs.
- Usage data is aggregate only: session length, run summaries, `perf.*` percentiles, desync counts. A run summary holds mode, Folio reached, end cause and party size, and per Folio: genre, entry and exit ticks, the offered doors and the committed door, the maximum equipped `C` per slot at entry, a Disconnected-weapon flag per slot, committed Rewrites, and NPC kills split into systemic and other by doc 01 §10.3's rule. Doc 16 §11.7 carries the same fields in its opt-in run summary, which only the host sends. Raw events are deleted after 180 days.
- A crash report holds a minidump, the last 200 log lines and, for a simulation panic, the §6.7 dump with player names removed. Nothing uploads without consent. "Delete my data" requests deletion by install ID. Endpoints are EU-hosted by `palimpsest_services`.

## 18. Coding conventions for AI agents

1. **Size limits:** files ≤ 400 lines excluding tests, functions ≤ 60 lines, ≤ 7 parameters, cognitive complexity ≤ 25 (clippy, `xtask lint-size`).
2. **SPEC references:** every public item and test in math, data, econ, gen, ai and sim carries a doc line starting with a match of `SPEC: (\d\d|canon) §[0-9.]+( step [0-9a-z.]+)?`, for example `/// SPEC: 00 §10.5 step 5` or `/// SPEC: canon §10.5 step 5, doc 05 §12.3` (`canon` means `00`, the form the Rust slice uses); `xtask spec-refs` checks each names an existing heading.
3. **Errors:** `thiserror` enums at IO, load and network boundaries. Inside the sim a broken invariant panics via `expect("invariant: …")`; `unwrap()` is denied. Binaries may use `anyhow`.
4. **Test-first per formula:** write the spec's worked-example test, see it fail, implement, add a property test. A formula without a worked example is reported as a spec gap, never invented.
5. **No `unsafe`** in math, data, econ, gen, ai, sim, net, bridge; elsewhere each block needs a `// SAFETY:` comment and review.
6. **Newtypes** for IDs and units across module boundaries (`EntityId`, `Id32`, `Ticks`, `Milli`, `Bp`); no wildcard arm on simulation enums.
7. **AGENTS.md** at the root and per crate: commands, forbidden APIs, owning spec chapters.
8. **Task order:** workspace, CI, xtask → math (Canon §21 tests 1–12, §4) → data, schemas, ID registry, content hash → store, scheduler, Sim API, serialization, hash, golden harness, SyncTest → grids, movement, collision → damage and statuses → items, loot, crafting → perception, AI → worldgen → bridge, minimal renderer → netcode → audio, input, UI, localization → platform, saves, telemetry, `palimpsest_econ` → mods → services → Deck verification and performance. Doc 21 §3.5 maps each step to its work packages, and doc 21's milestones and work-package acceptance tests govern the schedule. The Rust vertical slice of DECISIONS.md item 24 (`palimpsest_math`, `palimpsest_data`, `palimpsest_sim`) covers the math step and the first parts of the data and simulation steps (doc 21 §2).
9. **One formula, one function:** a formula used by two chapters lives once, in the owning chapter's module, and the other chapter calls it (for example `closest_seg_seg`, §4.6, is doc 05's `seg_seg_params`).

## Exports

All IDs follow Canon §1.1 and sit in namespaces Canon §19 assigns to this chapter (`crate.`, `flag.`, `perf.`). None collides in `fnv1a32` with another export of this chapter or with any ID token in canon-1.11 or in the other spec chapters on disk (checked by `pal validate`).

- **`crate.`** (20): `crate.math`, `crate.data`, `crate.econ`, `crate.gen`, `crate.ai`, `crate.sim`, `crate.net`, `crate.bridge`, `crate.mod_api`, `crate.scribe`, `crate.render`, `crate.audio`, `crate.ui`, `crate.platform`, `crate.bots`, `crate.server`, `crate.client`, `crate.services`, `crate.tools`, `crate.xtask`. `fnv1a32("crate.sim") = 0xdc634881`.
- **`flag.`** (kind, default; `*` = sim-affecting): `flag.render.solari` (bool, false), `flag.render.dlss` (bool, false), `flag.render.ssao` (bool, false), `flag.render.preset` (int 0 deck_battery, 1 deck, 2 medium, 3 high, 4 ultra; default from hardware detection, 1 on Steam Deck), `flag.audio.hrtf` (bool, true), `flag.audio.reflections` (bool, false), `flag.net.transport` (int 0 steam_sdr, 1 renet_udp, 2 in_process; 0), `flag.net.input_redundancy` (int, 8), `flag.sim.hash_every_tick` (bool, false), `flag.sim.synctest_depth` (int, 0), `flag.mods.wasm`* (bool, true), `flag.scribe.enabled` (bool, false), `flag.telemetry.usage` (bool, false), `flag.telemetry.crash_mode` (int 0 ask, 1 always, 2 never; 0), `flag.dev.hot_reload` (bool, false), `flag.dev.tools` (bool, false), `flag.dev.desync_dump` (bool, true), `flag.dev.schedule_shuffle_seed` (int, 0), `flag.platform.steam` (bool, true; false in builds without the `steam` Cargo feature, which use the Null backend), and for doc 16: `flag.net.shadow` (int 0 off, 1 auto = on with at least 6 physical cores and not handheld, 2 on; 1), `flag.net.voice` (bool, true), `flag.net.crossplay` (bool, true; inert while Steam is the only storefront, DECISIONS.md item 112), `flag.net.netsim` (int scenario index, dev builds only; 0 = off).
- **`perf.`** (58): 26 group budgets `perf.deck.sim.bookkeeping`, `perf.deck.sim.input_actions`, `perf.deck.sim.perception`, `perf.deck.sim.ai`, `perf.deck.sim.movement`, `perf.deck.sim.push`, `perf.deck.sim.env_sample`, `perf.deck.sim.pose`, `perf.deck.sim.hitscan_melee`, `perf.deck.sim.projectiles`, `perf.deck.sim.explosions`, `perf.deck.sim.damage`, `perf.deck.sim.block_updates`, `perf.deck.sim.random_ticks`, `perf.deck.sim.light`, `perf.deck.sim.sound`, `perf.deck.sim.mods`, `perf.deck.sim.view_extract`, `perf.deck.sim.liquids`, `perf.deck.sim.gases`, `perf.deck.sim.fire`, `perf.deck.sim.temperature`, `perf.deck.sim.surfaces_status`, `perf.deck.sim.needs`, `perf.deck.sim.net_snapshot`, `perf.deck.sim.hash_snapshot`; the totals `perf.deck.sim.tick_mean`, `perf.deck.sim.tick_p99`, `perf.deck.sim.tick_worst`; 14 unit costs `perf.deck.sim.unit.body_move_ns`, `perf.deck.sim.unit.push_pair_ns`, `perf.deck.sim.unit.pose_ns`, `perf.deck.sim.unit.ray_cell_ns`, `perf.deck.sim.unit.projectile_ns`, `perf.deck.sim.unit.explosion_us`, `perf.deck.sim.unit.block_update_ns`, `perf.deck.sim.unit.random_tick_ns`, `perf.deck.sim.unit.liquid_cell_ns`, `perf.deck.sim.unit.gas_cell_ns`, `perf.deck.sim.unit.fire_cell_ns`, `perf.deck.sim.unit.thermal_cell_ns`, `perf.deck.sim.unit.light_pop_ns`, `perf.deck.sim.unit.sound_cell_ns`; and `perf.deck.hash_ms`, `perf.deck.gen.folio_p50_ms`, `perf.deck.gen.folio_p99_ms`, `perf.deck.gen.folio_max_ms`, `perf.deck.load.folio_ms`, `perf.deck.mem.ram`, `perf.deck.mem.vram`, `perf.deck.frame.gpu`, `perf.deck.frame.cpu_main`, `perf.deck.frame.cpu_render`, `perf.deck.draw.calls`, `perf.deck.mesh.section_us`, `perf.deck.save_ms`, `perf.deck.net.host_kbps`, `perf.ci.bot_tps`.
- **Non-ID interfaces:** `SIM_VERSION`, the Sim API and `SimFlags`, `TickInputs` with slots `0..=4`, button bits (bit 25 always 0, `ACTION`, `WHEEL_ENTRY`, `WHEEL_SELECT`), the `IntentKind` variants of §5.5, `ViewOut`, `ViewFrame` (with `light_overrides` and `shadow_cells`), `ViewPlayer` (with `revealed`, `dark_vision_mm`), `ChunkView` (with `masks`), `FluidView`, `FolioView` (with `seams` and the `slot_of_cell` bit layout), `ViewTelegraph`, serialization sections and the hand-written canonical encoding of §6.5, the `PALS` container, payloads by kind, `RunCheckpoint` placement, `ReplayLog` encoding and error codes, the micro-voxel section and quad formats, render materials, the light and bleed fields, `cap_milli`, `ambient_milli`, `open_cost`, WIT `palimpsest:mod@1.0.0`, mod classification, `mod_blob`, patches, the Fluent key map, `content_hash`, `presentation_hash` and `modset_hash` layouts, `TWO_POW_32`, `isqrt_u64`, `div128`, `div256`, `I256`, `closest_seg_seg`, `VoxelDda` and `dda` with `max_steps`, `Rng::state` and `Rng::from_state` as the save interface, the crate module paths of §3.2, error codes `id32_collision`, `mod_namespace_forbidden`, `content_reload_refused_in_run`.

## Interface assumptions

| Doc | Assumed to provide |
|---|---|
| 01 | §3.13 signature-moment scenarios as `tests/moments/sm_NN.json`; §10.3's systemic-kill rule for the run summary (§17.4); the `pillar.living_page.t3` sweep definition for the three bot policies (§16) |
| 03 | as imported, plus the per-tick telegraph list that doc 13 publishes, read by `extract_view` (doc 13 asks doc 03 for "a view-frame channel"; this chapter supplies `ViewTelegraph`) |
| 04 | `Pools` and `comp.stats` fields read by `ViewPlayer` and `ViewEntity.hp_bp`; reference Calling weapons and kits for the balance harness; validation of `AllocatePoint`, `PickTalent` and `AssignAbilitySlot`; the breath rules that `SetBreathHold` drives |
| 05 | `seg_seg_params` implemented as a call to `closest_seg_seg` (§4.6); hitscan and melee unit costs inside the `hitscan_melee` group budget; `isqrt_u64` and `to_milli_floor` as its `mm(Fx)` |
| 06 | validation of `ComposeSpell`, `Pray { patron, kind }`, `AssignWheelEntry` and `SetCastItemTarget`; `in_shadow_area` (§13.1), evaluated by `extract_view` for `ViewFrame.shadow_cells`; the wheel selection and `ACTION` semantics of doc 17 §12.2 |
| 07 | block records with `skins` per genre suffix; `art.block.*` side and top layers; `cat.placeable` records and their render props (Canon §4.3); the predefined micro-voxel masks and chipping of doc 07 §10.8; `Craft { recipe, count }` validation; the `recipe_overlap` rules that `pal validate` runs; arrays marked `"x-patchable": true` in recipe lists |
| 09 | the `StealthWorld` field order for section 12 and its closing sections (Exports for the `aware.` IDs used here); validation of `Hack`, `Steal`, `Plant`, `Vocalize` and `SetLockMode`; the §4.8 reveal state read into `ViewPlayer.revealed` |
| 10 | validation of `Examine` (the §5.6 Analysis channel) and `SetFocusLock` |
| 11 | validation of its intents in §5.5, including `ShopHaggle { shop }` and `DropLucre { amount: i64 }`; `loot.chest.plain`'s `lines` marked `"x-patchable": true`, which §13.1's example patches |
| 12 | GPV fields `palette`, `render_style`, `music`; per-cell zone membership mapped to `FolioView.slot_of_cell` with bleed slots in bleed-plan order (entry first, then drift); at most 4 bleed genres per Folio; seam zones report slot 0 and are listed with their second genre for `FolioView.seams`, at most 7 per Folio |
| 13 | telegraph shape IDs with their `a`, `b` and radius semantics; `tools/curves_src.json` entries for table curves, keyed by doc 13's `ai.curve.` IDs; `enemy.role.soldier` as the balance reference; the §11.2 pacing table the bots reproduce; `MinionCommand` resolution |
| 14 | `RunCheckpoint` fields and `checkpoint_version`; the mode list including `mode.invasion_open`; `ChooseExit { door }` validation; the meta-progression rules that `palimpsest_platform::meta` implements, the Glyph Library included |
| 15 | `palimpsest_econ` behaviors and test vectors (RFC 8439 §2.3.2, RFC 8032); `palimpsest-verify`'s command line and checks (§4.3), built as a `palimpsest_tools` binary (§3.2); receipt key windows; the `juris.` schema with its `strict` field, loaded by the same file loader as `flag.` records; the minor flag of §8.4; Apocrypha detection through the §13.1 classification |
| 16 | as imported; doc 16 reads 13 serialization sections; its §5.8 sanitizer rejects a command with bit 25 set or `ACTION` 6 or 7 (C-10); its opt-in run summary carries the §17.4 per-Folio fields (C-14); `svc.lobby` beacons for invader search (C-13); no EOS backend, Epic route or Epic Workshop mirror, since Steam is the only storefront and the EOS SDK is dropped (DECISIONS.md items 112 and 123), and a ruling on whether `flag.net.crossplay` retires; session intents applied before player intents |
| 17 | the `input.*` action list and bindings, `ui.*` skins, comfort and accessibility settings including the FOV setting 60–120°; §12.2's command fields; validation of `SetScribesMercy`, `MapPing` and `QuickbarAssign`; the rename of `Haggle { shop }` to `ShopHaggle { shop }` (C-15) |
| 18 | `art.mesh.*` (including `art.mesh.enemy.gothic_horror.ghoul`, the §11.4 example), `art.anim.*` pose tables with the doc 03 §5.16 schema, `art.blockarray.*` at 64 × 64 px per layer, `art.block.*`, `art.icon.*`, `art.post.<genre_suffix>`, `art.lut.*`, `aud.*`, `vfx.*` emitters with `trigger` fields, 5 stems per music track, the §6.6 voice pools, per-layer material parameters, Fluent files, launch locales; rendering of reveal markers, dark vision, seam stitches and newly exposed micro-voxel faces |
| 19 | the schema catalog matching `palimpsest_data` record types, including the `x-patchable` keyword, the `02.patch` schema and the `E_RETIRED` code |
| 21 | §3.5's mapping of §18 item 8's steps to work packages; §4.1's module paths, which §3.2's Modules table copies; assignment of acceptance tests 48–53 to work packages |

## Canon gaps

- **G14.** Canon §1.3 reads every `x << k` in a normative formula as a checked multiplication, but Canon §3.1's xoshiro256++ line `let t = self.s[1] << 17;` must drop its high bits to reproduce Canon §21 test 1. (The keyed `(seq as u64) << 48` of §3.3 is checked, which is why a `seq` above `0xFFFF` panics.) This chapter implements the xoshiro shift as `self.s[1].wrapping_shl(17)` (§4.1). Proposal: Canon §1.3's shifts bullet adds "except the bit-mixing shift `s[1] << 17` of §3.1, which wraps".

G1 to G13 are closed by canon-1.11 and the triage; their numbers are not reused.

## Acceptance tests

1. `isqrt_u128` returns 0, 1, 1, 1, 2, 9, 10, 2^32, 2^64 − 1 for inputs 0, 1, 2, 3, 4, 99, 100, 2^64, 2^128 − 1, and equals core `u128::isqrt` on 100,000 random inputs.
2. `sqrt_fx(Fx::from_int(2)).raw() == 6074000999`; `sqrt_fx(Fx::from_int(50)).raw() == 30370004999`; `sqrt_fx(Fx::from_int(4)) == Fx::from_int(2)`.
3. `mul_fx(Fx(1), Fx(-1)) == Fx(-1)`; `mul_fx(Fx(1), Fx(1)) == Fx(0)`; `div_fx(7, 2)` raw 15032385536; `div_fx(-1, 3)` and `div_fx(1, -3)` raw −1431655766; an `i64` overflow in `Fx + Fx` panics in the `release` profile.
4. `crates/palimpsest_math/src/trig_table.rs` holds `SIN_T`, and `tools/gen_trig.py --check` passes. BLAKE3 of `SIN_T` as LE `i32` is `d2612a42910edad3b81095ad0be154d6076c5c33771916cbbea2ded40419ec11`; `SIN_T[8192] == 759250125`; `SIN_T[16383] == 1073741819`.
5. For `(y, x)` = (1, 1), (3, 4), (4, 3), (7, −24), (−5, −5), (2, −1), (1, 1000000), (5, 0), (0, −3), (0, 0), `atan2_bam` returns 8192, 6712, 9672, 29808, 40960, 21220, 0, 16384, 32768, 0.
6. Over 1,000,000 pairs drawn as `next_u64() as i64` from `Rng::from_seed64(7)`, `|atan2_bam − exact| ≤ 1 Bam`.
7. `yaw_from_dir` returns 16384, 0, 49152, 8192 for directions (−1, 0), (0, −1), (1, 0), (−1, −1) m; `fwd(16384, 0) == (−2^30, 0, 0)`.
8. `Fx3::length((3, 4, 0) m) == Fx::from_int(5)` exactly.
9. `closest_seg_seg((0,0,0), (2,0,0), (1,−1,1), (1,1,1))` gives `s = t = Fx(2147483648)` and `dist2 = 2^64`; `closest_seg_seg((0,0,0), (1,0,0), (3,1,0), (3,2,0))` gives `s = Fx(4294967296)`, `t = Fx(0)` and `dist2 = 5 · 2^64`; §4.6 Example 3 returns `s = Fx(4294967296)`, `t = Fx(0)`, `dist2 = 17 · 2^64` and Example 4 returns `s = Fx(0)`, `t = Fx(4294967296)`, `dist2 = (5 · 2^32 − 1)²`, both without a panic; doc 05's `seg_seg_params` returns the same `(s, t)` for 100,000 random segment pairs, and `closest_seg_seg` never panics on 1,000,000 random pairs with components below `2^41` raw.
10. `ray_capsule` from (0, 1.0, 0) along +X against (5, 0.3, 0)–(5, 1.5, 0), r 0.3 m returns raw 20186346293; from height 1.7 m returns raw 20514452597; from height 1.8 m returns the tangent hit raw 21474836480; from height 2.0 m returns `None`. `ray_sphere` from (0, 0.5, 0) along +X against centre (5, 0, 0), r 0.5 m returns raw 21474836480.
11. The §4.6 DDA example visits (0,1,0), (0,1,−1), (1,1,−1), (1,1,−2), (2,1,−2), (2,1,−3) at raw t 0, 2684354559, 3579139414, 8053063678, 10737418242, 13421772797. From (0.5, 0.5, 0.5) along +X with `t_max = Fx::MAX`, `max_steps = 5` yields 6 blocks ending at (5, 0, 0) and `max_steps = 0` yields only the start block. From x raw `i64::MAX − 2^31` along +X, `VoxelDda` yields only the start block, x `i32::MAX`, and stops without a panic.
12. Packing x 12, y 31, z 48, width 16, height 8, face 2, material 517, AO (3, 3, 2, 1), class 0 yields `0x1DEC0F8C`, `0x0086F205`, and unpacking returns the inputs. A lone +Y face with only its −X upper neighbour occupied has AO (2, 3, 3, 2) and uses the triangles `[0, 1, 2, 0, 2, 3]`; with only its (−X, −Z) upper diagonal occupied it has AO (2, 3, 3, 3) and uses `[1, 2, 3, 1, 3, 0]`. A chunk of full cells meshes as quads no wider than 32 micro-voxels, and the binary-greedy-meshing oracle gives the same face count per section.
13. `conv`: `fx_to_f32(Fx::from_milli(1620)).to_bits() == 0x3FCF5C29`; `rad_to_bam(π/2) == 16384`; `rad_to_bam(−π/2) == 49152`; `rad_to_bam(30°) == 5461`; `stick_to_i8(0.5) == 64`.
14. The loaded light tables equal the §8.5 `cap_milli` and `ambient_milli` rows; after a chunk with `light_sky = 15` uploads, its light-field texel R reads 255, and a GPU readback of a sample at a cell centre returns exactly `level × 17`. A Folio with 4 bleed genres fills bleed-field channels R, G, B, A for slots 1 to 4 and binds 5 texture arrays of 64 × 64 px layers with 7 mip levels (5,488 bytes per layer). An active light override with `max_level` 2 over a cell whose `light_sky` and `light_block` are both 15 makes that cell's texel read 34 in R and in G; a cell in `ViewFrame.shadow_cells` with `light_sky` 15 and `light_block` 5 reads 119 in R and 0 in G; a cell inside an override's box but outside its `sphere` keeps its stored levels.
15. The page curl at `t = 0.5` maps `u = 1.0` to (0.9144, 0.15) and `u = 1.9` to (0.1, 0.3) within 10^−4.
16. `open_cost` equals a Dijkstra over open N18 neighbours (face 20, edge 28) for all 4,913 offsets within ±8 cells. The occlusion example (130 dB heard at 95 dB, `Δ = (10, 0, 0)`) yields gain −35 dB, `extra = 15` and cutoff 8,434 Hz ± 1 Hz; heard at 110 dB it yields `extra = 0`; the open diagonal `Δ = (6, 3, 2)` heard at 114 dB yields `extra = 0`.
17. The two-record `content_hash` example yields `92c159a0a347070d679d6edd626a8bc6938565f60204dd732ba14c2ac3592cc9`; reordering keys or whitespace in the source files does not change it. The empty presentation set yields `presentation_hash = 596015384dbe4c15a8a14de247b103c5b2ab6640314cac114650a0a476af559b`; adding `en-US/02.ftl` with `pal-test = x\n` yields `08ea33804f578a4904f58633637d4633498201421eb29186905fdf54092072d1` and leaves `content_hash` unchanged; adding an `art.` record changes `presentation_hash` only.
18. `loc.arch.melee.blade_long.skin.wuxia.name` maps to Fluent ID `loc-arch-melee-blade_long-skin-wuxia-name` and back.
19. The validator rejects `1.5`, `1e3`, `-0.0` and `9007199254740992`, and accepts `9007199254740991` and `"0x00000000000000FF"` in a `u64` field.
20. Save container: header length 112; flipping one payload byte gives `payload_corrupt`; flipping header byte 20 gives `header_corrupt`; a run save with a different `SIM_VERSION` gives `sim_version_mismatch`, while a meta profile with a different `SIM_VERSION` and `content_hash` loads and migrates.
21. `xtask check-deps` passes on the workspace and fails when `glam` or `postcard` is added to `palimpsest_sim` or when `palimpsest_math`'s default features bring `serde` into its closure.
22. `xtask float-scan` fails on `let x = 1.0;` in `palimpsest_sim/src` and passes on the same line inside a `#[cfg(test)]` module.
23. The determinism set yields byte-identical `hashes.json` on all five §6.4 targets.
24. SyncTest depth 8 passes over every fast golden replay; a fixture system storing state outside the serialized sections fails it.
25. A pull request changing a golden hash without touching `SIM_VERSION` or `content/` fails CI.
26. Flipping one bit of a stored sub-attribute in a client's entity column at tick 1,000 is detected at boundary 1,020; both dumps name section 2 and the affected `EntityId`.
27. In a release build with a run active, a content change returns `content_reload_refused_in_run` and `content_hash` is unchanged.
28. A WASM mod looping forever traps at the 50,000-fuel hook limit, is disabled on the same tick on x86_64 and aarch64, and both replicas keep equal state hashes. A mod whose hooks use 40,000 fuel each runs two hooks in a tick and traps in its third, which receives only the 20,000 fuel left of the 100,000 per-tick limit.
29. A WASM mod returning the bits of `0.0 / 0.0` as `u32` returns `0x7FC00000` on every target.
30. Spawning 5 entities then killing the 3rd: after `compact`, `iter` yields the other 4 in ascending `EntityId`; a sixth spawn appends after the fifth. Adding a component to entity 2 after entity 5 already has it inserts it before entity 5, and `iter` stays ascending.
31. Entity `(3 << 40) | 5` is due for AI think (period 6) on ticks 1, 7 and 13 and not on 0 or 6.
32. With telemetry at defaults, a full session makes zero requests to telemetry endpoints.
33. On `deck-01`, `perf_saturation_4p` keeps the mean tick ≤ 5.0 ms, the 99th percentile ≤ 8.0 ms and every tick ≤ 14.5 ms, each group within its §15 mean and worst; `perf_spike_4p` keeps every tick ≤ 14.5 ms; meshing takes ≤ 25 µs per section; a `bot_typical` CI run averages ≥ 6,000 ticks/s. A unit test recomputes from `content/02/perf/perf.json` that the group means give a 4.33 ms mean tick and the group worsts a 14.50 ms costliest phase, below 16.67 ms.
34. With `curr` published 7.67 ms ago, `alpha = 0.46` and an entity moving from x 10.0 m to 10.1 m renders at 10.046 m.
35. A client whose simulation-affecting mod set differs from the host's is refused at the handshake with `content_mismatch`; a client that differs from the host only in a presentation-only mod joins, and the host's replay records the host's `presentation_hash`.
36. `div128(2 << 64, 4 << 64)` is raw 2147483648; `div128(-1, 3)` is raw −1431655766; `div256(I256::mul(3 << 64, 1 << 64), I256::mul(4 << 64, 1 << 64))` is raw 3221225472; `I256::mul` matches 512-bit reference products for 100,000 random `i128` pairs. `I256` compares through derived `Ord`: `I256::from_i128(-1) < I256::ZERO`, `I256::ZERO < I256::from_i128(1)` and `I256::mul(1 << 64, 1 << 64) > I256::from_i128(i128::MAX)`.
37. Four ticks published between two render frames: the render world sees the final chunk generations and all four ticks' events in tick order, and drops nothing; 20 chunks whose light changed on one tick are sent as 16 that tick and 4 on the next.
38. A replay with commands on slot 4 (an invader) decodes from the CIL into exactly the Canon §2.4 records, and re-simulation reproduces every checkpoint hash.
39. A run save written by `SIM_VERSION` n is read by a build with `SIM_VERSION` n + 1: `RunCheckpoint` decodes, the simulation payload is never parsed, and Close the Book ends the run as `run.end.abandoned_version` with the checkpoint's score.
40. Mod classification: a mod with only `art.` records and assets is presentation-only, changes `presentation_hash` and leaves `content_hash` unchanged; adding a patch, `effects.wasm` or an `item.x_<mod_id>.*` record makes it simulation-affecting; a `flag.` or `juris.` record is rejected with `mod_namespace_forbidden`; a record reusing a base ID is rejected with `id32_collision`.
41. A mod `frostpack` version `1.0.0` holding only `mod.json` with content `{}` has a 22-byte blob with `BLAKE3 = a6e6bfb43eb8eb55d0bdd262621ba1c8412110cccb19be2a415117de63acf638`, and a `MOD` section holding that one entry gives `modset_hash = 0bac787f484526cbbae3882a76af7b24c9c2f001830afd0120ba608326c301dd`.
42. Music: a hostile at `aware.suspicious` toward the local player 31,900 mm away gives `music_level` 1; at 32,100 mm it gives 0; a living invader at `aware.alert` gives 4.
43. `look_add(0, 0.01)` gives 6835653 and `look_to_bam(6835653)` gives 104; after 1,000,000 alternating ±0.01 rad deltas `yaw_q` returns exactly to its start.
44. `xtask check-deps` passes when `palimpsest_client` enables `palimpsest_data/loader`, because `palimpsest_sim`'s closure is resolved alone; adding `serde_json` to `palimpsest_gen` fails it.
45. While slot 4 is occupied, a single-player Suspend request is deferred and completes on the tick `ev.invasion_ended` is emitted (Canon §2.5).
46. `tools/gen_trig.py` and `tools/gen_curves.py` regenerate byte-identical committed outputs.
47. The balance harness passes on the reference content: Folio-1 duels give 2.0 s ± 0.3 s and 30 ± 4.5 HP/s, no generated item exceeds `ITEM_MULT_MAX`, and the bot runs meet the §16 pacing bands.
48. Intents: a replay tick holding a `SlotDormant { slot: 2 }` session intent, `InvaderJoin { kit }` on slot 4, `ShopHaggle { shop }` on slot 1 and `SetLockMode { manual: true }` on slot 0 applies the session intent first, then the slot 0, slot 1 and slot 4 intents, and re-simulation reproduces every checkpoint hash.
49. Command bits: the replay validator rejects a command with bit 25 set, with `ACTION` 6 or with `ACTION` 7; it accepts `ACTION` 5 with bits 29–31 set to 3 and bits 19 and 20 clear, which selects no wheel entry.
50. Canonical encoding: `Some(5u32)` encodes as `01 05 00 00 00` and `None` as `00`; a `BTreeMap<u32, bool>` holding `{2: true, 1: false}` encodes as `02 00 00 00 01 00 00 00 00 02 00 00 00 01`; an `Option` tag byte 2 and a trailing byte after a section are decode errors; the default dependency closure of `palimpsest_sim` contains neither `serde` nor `postcard`.
51. Draw counters: a `dev` and a `release` build of `palimpsest_sim` give identical `state_hash` at every checkpoint of the golden fast set, and no section of the `dev` build's serialization contains a draw counter.
52. Micro-voxels: chipping one cell of a destructible block sends a `ChunkView` whose `masks` holds that cell and whose `block_gen` is bumped; the client remeshes only the sections whose padded copy holds the cell (at most 8) and spawns one debris cube per cleared micro-voxel, at most 16.
53. Seams and reveal: a Folio with one seam zone sets bits 3–5 of `slot_of_cell` to 1 on its cells and bits 0–2 to 0, and `FolioView.seams[0]` is that seam's second genre; `ViewPlayer.revealed` lists exactly the NPCs whose doc 09 §4.8 reveal meter for that slot has `revealed = true`, ascending.

## Invariants annex

```
Σ 02: stack ⊗ crates ⊗ sim ⊗ fixed-point ⊗ determinism ⊗ bridge ⊗ render ⊗ audio ⊗ content ⊗ saves ⊗ mods ⊗ platforms ⊗ budgets ⊗ tests
W! sim ∈ no_std+alloc ∧ single-thread ∧ forbid(unsafe)          N! bevy|glam|wgpu|wasmtime|rand|fixed ∈ closure(sim)
W! floats := bridge::conv only                                  N! float → sim, except via PlayerCommand integers
W! iteration := ascending EntityId | uid                        N! sort_unstable | HashMap | RefCell ∈ sim
W! Fx ops checked; overflow ⇒ panic ∀ profiles                  N! Mul|Div operator traits for Fx×Fx
W! |atan2_bam − exact| ≤ 1 Bam                                  W! isqrt_u128 == core isqrt
W! state_hash := BLAKE3(13 sections, fixed order)               W! hash boundaries ≡ 0 mod 60 (prod), every tick (CI)
W! golden hash change ⇒ SIM_VERSION|content change             W! 5-target hashes byte-identical
W! SyncTest(8) passes                                          N! state outside serialization
W! render light ≤ cap(max(light_sky, light_block))              N! visual light contradicts stealth light
W! bleed ink line ≤ 0.25 m from sim local_genre boundary        N! render extrapolates sim state
W! content: JSON ∧ schema 2020-12 ∧ |int| ≤ 2^53−1              N! float token ∈ content
W! content_hash ⊇ sim records ∪ unlocks ∪ sim mods ∪ sim-flags  N! hot reload mid-run in release
W! presentation_hash ⊇ art.|aud.|vfx.|nar.|ui. ∪ loc ∪ pres mods N! presentation record ∈ content_hash
W! save := PALS header ∧ BLAKE3 ∧ atomic rename                 N! resume with sim_version|content_hash mismatch
W! WASM: fuel ∧ NaN canon ∧ no relaxed SIMD ∧ no WASI           N! client build other than Windows x86_64 on Steam (DECISIONS 98, 112, 123)
W! telemetry & crash upload := opt-in                           N! Steam ID | IP | ad ID ∈ telemetry
W! every pub sim item has SPEC ref                              N! unwrap() ∈ sim
W! TickInputs slots 0..=3 party, 4 invader                      N! Suspend while slot 4 occupied
W! view deltas lossless: chunk views by generation, events queued   N! event dropped by a skipped frame
W! one closest_seg_seg for docs 03 and 05 (exact i128/I256)     N! epsilon in segment–segment
W! clamp before narrow: closest_seg_seg never panics           W! tangent ray (h = 0) is a hit
W! canonical serialization hand-written, self-delimiting       N! serde | postcard ∈ default closure(sim); draw counter ∈ state
W! DDA bounded by t_max ∧ max_steps ∧ i32 edge                 N! command bit 25 set | ACTION ∈ {6, 7}
W! render meshes 250 mm micro-voxels, triplanar 64 texels/m    N! render light above the doc 03 §10.6 light read
W! Σ group worst budgets at costliest phase < 16.67 ms          W! mean tick ≤ 5.0 ms on Deck
W! audio occlusion := att − open_cost (sim N18 metric)          N! inverse-square free-field term
W! mods add records ∨ append patches only                       N! mod record ∈ crate.|flag.|perf.|cur.|ent.|juris.|banner.|pass.|store.|bundle.|cosm.
W! econ: PAID_RANDOM_ITEMS_ENABLED = false                      N! client signs any container
W! run save := RunCheckpoint first, decodable by later builds   N! Close the Book loads the sim
```
