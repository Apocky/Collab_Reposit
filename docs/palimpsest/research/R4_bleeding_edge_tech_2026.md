# R4 — Bleeding-Edge Tech Dossier: engines, rendering, voxels, physics, netcode, procgen, runtime AI, tooling

As of 2026-10-11. Research input for PALIMPSEST (3D first-person co-op omni-genre voxel roguelike, Rust + Bevy per DESIGN_BRIEF §5, written mostly by an AI coding agent).

## Table of contents

0. Method, evidence legend, one-screen verdict
1. Engines and frameworks (Bevy, Godot, Unreal, Unity, Fyrox, raw wgpu)
2. Graphics state of the art (WebGPU, wgpu, mesh shaders, work graphs, GPU-driven, RT/GI, upscalers, NTC, splats)
3. Target hardware tiers (Steam Deck, Steam Machine, Steam Frame, Switch 2)
4. Voxel engines and techniques (Minecraft, Vintage Story, Veloren, Teardown, meshing, SVDAG, voxel light)
5. Physics and determinism (Rapier, Avian, Jolt, Box3D, fixed-point Rust)
6. Netcode (rollback, snapshot prediction, replication, transports, Steam/EOS, Barony model)
7. Procedural generation (WFC, model synthesis, Dormans cyclic grammars, Qud, validation, offline LLM pipelines)
8. Runtime AI: on-device small language models
9. Game AI architectures (BT, utility, GOAP, HTN) and navigation
10. Audio (Steam Audio, Firewheel, gameplay acoustics)
11. Modding (WASM components, WASI 0.3, fuel; Lua alternative)
12. Data formats, schemas, serialization
13. Platform economy services (Steam Inventory Service exact schema, backend for gacha/pass)
14. Testing, replays, automated playtesting
15. AI-agent-friendly codebase rules
16. Recommended stack with exact versions
17. Design implications for PALIMPSEST

---

## 0. Method, evidence legend, one-screen verdict

**Method.** Versions and release dates of every Rust crate were read directly from the crates.io API on 2026-10-11 (endpoint `https://crates.io/api/v1/crates/<name>`, alias **CRATES** below). Engine, platform and model facts were read from official release pages where reachable, otherwise from news coverage. Rust stable was read from `https://static.rust-lang.org/dist/channel-rust-stable.toml`.

**Evidence legend.**
- `[VERIFIED <url>]` — page fetched and read this session.
- `[VERIFIED-SEARCH <url>]` — claim seen in a search-result extract of that URL this session, page not fully fetched. Weaker; treat as provisional.
- `[CRATES]` — read from the crates.io API this session (version, date, dependency requirement, feature flags).
- `[RECALLED]` — from training memory, not re-checked this session.
- `[DERIVED]` — arithmetic on verified numbers.
- `[DESIGN]` — a recommendation of this dossier, not a fact about the world.

**One-screen verdict** (details and justifications in §16 and §17):

| Layer | Pick (exact version, 2026-10-11) | One-line reason |
|---|---|---|
| Language | Rust stable 1.99.0 (toolchain pinned via `rust-toolchain.toml`) | Strong types, exhaustive enums, cargo workspace = ideal LLM-agent substrate |
| Client engine | **Bevy 0.20.0** (released 2026-10-08), fallback Bevy 0.19.1 | Code-only, ECS, headless-testable, BRP+MCP agent hooks, schedule randomization for order bugs |
| Sim | Pure-Rust `palimpsest_sim`, no Bevy, no floats in state | Mandated by brief; survives Bevy's ~4-month breaking cadence |
| Sim math | `fixed` 1.31.0 (`FixedI64<U16>` = I48F16) + integer `isqrt` + LUT trig + u16 binary angles | Bit-exact on x86_64/aarch64/wasm |
| RNG | Own PCG32 (spec'd in canon) seeded by SplitMix64 of `(run_seed, stream_id)` | Zero crate churn; trivially exact |
| Voxel mesher | Binary greedy meshing (port of cgerikj v2), 8-byte packed quads, vertex pulling | 50–200 µs per 64³ chunk on a 2019 desktop CPU |
| Lighting | Sim light grid (flood fill) mirrored to GPU as 3D texture; optional Solari path tracing on RT-capable GPUs | Same light drives stealth and visuals |
| Cosmetic physics | `avian3d` (0.7 on Bevy 0.19; 0.8 expected for 0.20) client-only | Ragdolls/debris never feed back into the sim |
| Netcode | Own deterministic-sim prediction + rollback-to-host-snapshot, over `steamworks` 0.13.1 (SDR P2P) in production and `renet` 2.0.0 UDP for dev/LAN/tests | Barony-style listen host, IPs hidden, NAT solved by Valve |
| Audio | `bevy_seedling` (Firewheel) + `audionimbus` 0.16.0 (Steam Audio 4.8.1) | Open-source HRTF, occlusion, pathing |
| Runtime LLM | `llama-cpp-2` 0.1.159 (llama.cpp) + GBNF/JSON-schema constrained output; Qwen3.5-0.8B/2B or Gemma 4 E2B (Apache-2.0) | Flavor only, cached by seed, template fallback |
| Mods | `wasmtime` 49.0.2 components (WIT), fuel metering for deterministic mods | WASI 0.3 ratified 2026-06-11 |
| Data | RON for authored content, JSON Schema via `schemars` 1.2.2 + `jsonschema` 0.58.6, `postcard` 1.1.3 for saves/net | Validate everything in CI |
| Tests | `insta` 1.49.0, `proptest` 1.12.0, `cargo-nextest` 0.9.148, `cargo-mutants` 27.1.0, replay golden hashes | Agent gets machine-checkable truth |
| Backend (gacha/pass) | `axum` 0.8.9 + `sqlx` 0.9.0 + Postgres, `sha2` 0.11.0/`hmac` 0.13.0 commit-reveal, `ed25519-dalek` 3.0.0 receipts | Server-rolled odds, offline receipts |

---

## 1. Engines and frameworks

### 1.1 Bevy

**Release timeline.**

| Version | Date | Contributors / PRs (intro figure) | Headline items | Source |
|---|---|---|---|---|
| 0.16 | 2025-04-24 | — | ECS Relationships (`ChildOf`/`Children`, one-to-many), GPU-driven rendering on by default where supported, `no_std`, systems return `Result` | [VERIFIED https://bevy.org/news/bevy-0-16/] |
| 0.17 | 2025-09-30 | 278 / 1311 | Solari (experimental realtime path tracing, diffuse only, needs DLSS-RR denoiser), DLSS (RTX, Vulkan on Win/Linux only), subsecond hot-patching of systems, headless widgets + Feathers, buffered `Event` renamed `Message`, `Trigger<T>` renamed `On<T>` | [VERIFIED https://bevy.org/news/bevy-0-17/] |
| 0.18 | 2026-01-13 | 174 / 659 | Atmosphere affects PBR lighting, `ScatteringMedium` custom atmospheres, Solari specular/reflections, fullscreen materials, automatic directional UI navigation, cargo feature collections (`2d`, `3d`, `ui`), fly/pan dev cameras, screenshot/video plugin | [VERIFIED https://bevy.org/news/bevy-0-18/] |
| 0.19 | 2026-06-19 | 261 / 1185 | **BSN** scenes via `bsn!` macro ("composable, patchable, and dependency aware"), relationships inline in BSN with `allow_self_referential`, resources stored as components on singleton entities, remote entity reservation, contact shadows, PBR SSR, rectangular area lights, `EditableText` text input, `parley` text, `many_cubes` 49.47 ms → 18.77 ms/frame | [VERIFIED https://bevy.org/news/bevy-0-19/] |
| 0.19.1 | 2026-08-13 | — | patch | [CRATES] |
| 0.20 | 2026-10-08 | 227 / 817 | BSN syntax reworked (`@` prefix for scene refs, "should largely be nailed down"), `Ready` scene event, **initial mesh shader support** (task/mesh/fragment pipeline; not on web), Solari on macOS/Metal, ReSTIR off by default, DLSS-RR 4.5 via `dlss_wgpu`, **schedule randomization** (`ScheduleBuildSettings::shuffle_seed`, `debug` feature), `chain_weak`/`before_weak`/`after_weak`, panics in systems converted to errors (`FallbackErrorHandler`), Feathers widgets (dropdown, list view, color input) | [VERIFIED https://bevy.org/news/bevy-0-20/] |

Cadence: 0.16→0.17 5.2 months, 0.17→0.18 3.5 months, 0.18→0.19 5.2 months, 0.19→0.20 3.6 months [DERIVED]. Every minor release breaks APIs. **Editor: still not shipped** in 0.20; the 0.20 post lists building blocks only; "Assets as Entities" is work in progress with no date [VERIFIED https://bevy.org/news/bevy-0-20/].

Facts read from crates.io for `bevy 0.20.0` [CRATES]: `rust_version = 1.97.1`; `bevy_render 0.20.0` depends on `wgpu ^30` and `glam ^0.33.2`; `bevy_render 0.19.1` depends on `wgpu ^29.0.3`, `naga ^29.0.3`, `glam ^0.32.0`. Feature flags present in 0.20.0 include `3d`, `ui`, `bevy_remote`, `bevy_solari`, `dlss`, `meshlet`, `meshlet_processor`, `hotpatching`, `webgpu`, `free_camera`, `experimental_pbr_pcss`, `debug`. Rust stable on 2026-10-11 is **1.99.0** (build 2026-09-28) [VERIFIED https://static.rust-lang.org/dist/channel-rust-stable.toml].

Virtual geometry (meshlets) still lives in `bevy::pbr::experimental::meshlet`; docs list drawbacks: slower than the standard renderer for small amounts of geometry, disables MSAA, no WASM, opaque materials only [VERIFIED-SEARCH https://dev-docs.bevyengine.org/bevy/pbr/experimental/meshlet/index.html]. Irrelevant for a voxel game whose geometry is already coarse quads.

**Ecosystem readiness three days after 0.20** (dependency requirement read from crates.io) [CRATES]:

| Crate (latest) | Date | Targets Bevy | Role for PALIMPSEST |
|---|---|---|---|
| `bevy_replicon` 0.45.0 | 2026-10-11 | 0.20 | (not chosen; see §6) |
| `bevy_rapier3d` 0.37.0 | 2026-10-10 | 0.20 | alternative cosmetic physics |
| `bevy_egui` 0.43.1 | 2026-10-10 | 0.20 | debug UI |
| `bevy_kira_audio` 0.27.0 | 2026-10-09 | 0.20 | alternative audio |
| `bevy_mod_scripting` 0.22.0 | 2026-10-11 | 0.20 | (not chosen) |
| `bevy_brp_mcp` / `bevy_brp_extras` 0.23.0 | 2026-10-09 | 0.20 | agent drives the running game |
| `avian3d` 0.7.0 | 2026-06-20 | 0.19 | cosmetic physics |
| `lightyear` 0.30.1 | 2026-09-16 | 0.19 | (reference design) |
| `bevy_ggrs` 0.22.0 | 2026-06-26 | 0.19 | (not chosen) |
| `bevy-tnua` 0.32.0 | 2026-06-22 | 0.19 | (not needed: own controller in sim) |
| `bevy_enhanced_input` 0.26.0 | 2026-06-19 | 0.19 | input actions |
| `leafwing-input-manager` 0.21.0 | 2026-06-22 | 0.19 | alternative input |
| `bevy_hanabi` 0.19.0 | 2026-06-27 | 0.19 | GPU particles |
| `bevy_seedling` 0.8.0 | 2026-08-09 | 0.19 (Firewheel ^0.12) | audio graph |
| `bevy-steamworks` 0.17.0 | 2026-07-04 | 0.19 (steamworks ^0.13.1) | Steam glue |
| `bevy-inspector-egui` 0.37.0 | 2026-06-20 | 0.19 | inspector |
| `bevy_landmass` 0.12.1 | 2026-09-30 | 0.19 | (not needed) |
| `bevy_voxel_world` 0.17.0 | 2026-07-05 | 0.19 | (reference only) |
| `bevy_matchbox` 0.14.0 | 2026-02-13 | 0.18 | WebRTC P2P (not chosen) |
| `bevy_behave` 0.5.0 | 2026-02-06 | 0.18 | (not chosen) |

Port latency evidence for the last cycle: Bevy 0.19 shipped 2026-06-19; `bevy_enhanced_input` 0.26 the same day, `avian3d` 0.7 one day later, `bevy-tnua` 0.32 three days later, `lightyear` 0.28 and `bevy_ggrs` 0.22 seven days later, `bevy_hanabi` 0.19 eight days later, `bevy-steamworks` 0.17 fifteen days later, `bevy_seedling` 0.8 51 days later [CRATES, DERIVED]. Expect the 0.20 ecosystem to be complete by roughly November 2026.

**Agent-relevant Bevy features.** Bevy Remote Protocol (`bevy_remote` feature) exposes the ECS over HTTP JSON-RPC [RECALLED: introduced in 0.15]. `bevy_brp_mcp` is "an MCP server for AI coding assistants to control Bevy apps" and `bevy_brp_extras` adds "screenshot, shutdown, and keyboard input" BRP methods [VERIFIED https://github.com/natepiano/bevy_brp; CRATES description]. Combined with `hotpatching` (subsecond; systems only; binary crate only; not wasm; changing a system's parameters blocks the patch) [VERIFIED https://bevy.org/news/bevy-0-17/], an agent can launch the game, query entities, inject input, screenshot and verify.

**Console reality.** No public evidence of any Bevy title on Switch 2; Tiny Glade's developer called a Switch version "unlikely" [VERIFIED-SEARCH https://insider-gaming.com/can-you-play-tiny-glade-on-nintendo-switch/]. Console SDKs are NDA-gated; Rust console ports are done privately by studios [RECALLED]. Treat consoles as a post-launch port project.

### 1.2 Godot 4.6 / 4.7

- **4.6** released 2026-01-26 ("All about your flow"); patches 4.6.1 (2026-02-16), 4.6.2 (2026-04-01), 4.6.3 (2026-05-20) [VERIFIED-SEARCH https://godotengine.org/blog/release]. **Jolt is "the default physics engine for all new 3D projects"; existing projects aren't affected.** New Windows projects default to Direct3D 12. SSR overhauled (full/half res). Glow blends before tonemapping. AgX gets `agx_white`/`agx_contrast`. Octahedral reflection/radiance probes. New IK framework (`IKModifier3D`, `TwoBoneIK3D`, `SplineIK3D`, `FABRIK3D`, `CCDIK3D`, `JacobianIK3D`). Unique node IDs. LibGodot embedding. Delta-encoded patch PCKs. Tracy/Perfetto/Instruments profiler support [VERIFIED https://godotengine.org/releases/4.6/].
- **4.7** stable around 2026-06-19 per secondary coverage [VERIFIED-SEARCH https://app.cinevva.com/news/2026-06-12-godot-4-7-rc]. Official page: HDR output on Windows/macOS/iOS/visionOS/Linux-Wayland; `AreaLight3D`; Asset Store replaces Asset Library; Control offset transforms; GABE (Android build environment) stable; `DrawableTexture2D`; **day-one Android XR and Steam Frame support**; Vulkan subsampled images for foveation [VERIFIED https://godotengine.org/releases/4.7/].
- Verdict for PALIMPSEST: strongest open-source editor, real console path via third parties [RECALLED: W4 Games], but scene/resource files and editor-wired signals are harder for an agent to verify than pure Rust, and the brief fixes Rust + Bevy.

### 1.3 Unreal Engine 5.7 / 5.8

- **5.7** (2025-11-12): PCG framework and Substrate **production-ready**; **Nanite Foliage experimental** (Nanite Voxels, Nanite Skinning); **MegaLights beta**; Procedural Vegetation Editor experimental [VERIFIED-SEARCH https://unrealengine.com/en-US/news/unreal-engine-5-7-is-now-available].
- **5.8** (2026-06-17, State of Unreal): **MegaLights "now fully production-ready"**; **Lumen Lite** (secondary sources: about 2× faster than standard Lumen); **Mesh Terrain** experimental (3D-mesh terrain replacing heightfields); **experimental MCP server plugin** connecting LLMs to projects; 5.8 "most likely going to be the final release of Unreal Engine 5.x"; first UE6 previews "late 2027ish" [VERIFIED https://gamefromscratch.com/unreal-engine-5-8-released/]. Secondary coverage also lists Iris networking, Audio Insights, Live Link Hub and Movie Render Graph as production-ready in 5.8 [VERIFIED-SEARCH https://wnhub.io/news/engines/item-51157].
- Verdict: best AAA renderer, worst agent substrate (binary `.uasset`, Blueprints, 30–60 min clean C++ builds [RECALLED]), and Nanite/Lumen solve problems (dense static meshes, large lit worlds) that a block-voxel dungeon does not have.

### 1.4 Unity 6.x

- Unity 6.3 LTS is current LTS, supported to December 2027; Unity 6.0 LTS support ends October 2026; Unity says the Runtime Fee is cancelled for Unity 6 [VERIFIED https://unity.com/releases/unity-6].
- Secondary sources: 6.4 shipped 2026-03-19, 6.5 mid-June 2026; Unity's June 2026 forum update targets a full CoreCLR editor in 6.8; 6.5 widens `EntityId` to 8 bytes [VERIFIED-SEARCH https://discussions.unity.com/t/coreclr-scripting-and-serialization-update-june-2026/1723299].
- Verdict: viable, but Mono→CoreCLR transition is mid-flight in 2026 and scenes/prefabs are YAML with GUID cross-references that agents corrupt easily [RECALLED].

### 1.5 Fyrox 1.0

`fyrox` 1.0.0 published 2026-03-24, 1.0.1 on 2026-03-28 [CRATES]. 1.0 consolidates; adds `export-cli` for CI exports; PBR with IBL and reflection probes; **no global illumination** (planned for ~2.0) [VERIFIED-SEARCH https://fyrox.rs/blog/post/fyrox-game-engine-1-0-0/]. Smaller ecosystem than Bevy; not chosen.

### 1.6 Raw wgpu

`wgpu` 30.0.1 (2026-08-22) [CRATES]. Mesh shading exists behind `Features::EXPERIMENTAL_MESH_SHADER` with `Device::create_mesh_pipeline`, documented as possibly having "major bugs" and subject to breaking changes [VERIFIED-SEARCH https://wgpu.rs/doc/wgpu/documentation/extensions/mesh_shading/index.html]. Bevy already sits on wgpu 30, so custom render passes inside Bevy get everything raw wgpu would give, without writing windowing, input, asset loading and UI.

### 1.7 Engine scorecard for an AI-agent-built voxel roguelike

Scores 1–5, [DESIGN] judgement from the facts above.

| Criterion | Bevy 0.20 | Godot 4.7 | UE 5.8 | Unity 6.3 | Fyrox 1.0 |
|---|---|---|---|---|---|
| Everything is text/code an agent can diff | 5 | 3 | 1 | 2 | 3 |
| Compile-time correctness (types, exhaustive enums) | 5 | 2 (GDScript) / 4 (C#) | 3 | 4 | 5 |
| Headless deterministic test harness | 5 | 3 | 2 | 3 | 3 |
| Live introspection for agents (MCP) | 4 (BRP + bevy_brp_mcp) | 2 | 3 (experimental MCP) | 2 | 1 |
| Voxel suitability (custom mesh, compute) | 5 | 4 | 2 | 4 | 3 |
| API stability | 2 | 4 | 4 | 4 | 4 |
| Console path | 1 | 4 | 5 | 5 | 1 |
| Editor for artists | 1 | 5 | 5 | 5 | 4 |

---

## 2. Graphics state of the art (October 2026)

### 2.1 APIs

**WebGPU in browsers.** web.dev (2025-11-25): Chrome/Edge since 113 on Windows (D3D12), macOS, ChromeOS; Chrome 121+ on Android 12+ with Qualcomm/ARM GPUs; Firefox 141 on Windows; Firefox 145 on macOS Tahoe 26 ARM64; Safari 26 on macOS/iOS/iPadOS/visionOS 26; **Linux "still in progress"** for Chrome and Firefox [VERIFIED https://web.dev/blog/webgpu-supported-major-browsers]. Khronos (SIGGRAPH 2026 deck) adds Chromium Linux on Intel Gen12+ from 2026-01 [VERIFIED-SEARCH https://www.khronos.org/developers/linkto/webgl-and-webgpu-sigg26]. Implication: a browser demo build is feasible on Windows/macOS/Android, not a primary target; Bevy mesh shaders are explicitly "not supported on web platforms" [VERIFIED https://bevy.org/news/bevy-0-20/].

**Mesh shaders.** Bevy 0.20 has "initial base support" (optional task shader + mesh shader + fragment; integrated with pipeline cache; no `StandardMaterial` integration yet); listed use cases are meshlets, procedural grass, voxels and particles [VERIFIED https://bevy.org/news/bevy-0-20/]. Hardware: RDNA2 (Steam Deck), RDNA3 (Steam Machine), Turing+ and Ampere (Switch 2 is Ampere) all expose mesh shaders in Vulkan [RECALLED]. Not needed for v1 (vertex pulling of packed quads is enough), worth an Ultra-tier experiment.

**DX12 Work Graphs / mesh nodes.** Mesh nodes are "a new type of leaf node in work graphs" that dispatch a mesh-shader graphics pipeline instead of a compute shader; status is **public preview** (AMD day-0 driver), demonstrated at GDC 2024; no benchmark numbers on the page [VERIFIED https://gpuopen.com/learn/work_graphs_mesh_nodes/work_graphs_mesh_nodes-intro/]. AMD also ships a Vulkan experimental extension [VERIFIED-SEARCH https://gpuopen.com/learn/gpu-workgraphs-mesh-nodes-vulkan/]. No shipped game using mesh nodes was found. wgpu does not expose work graphs [RECALLED]. Verdict: ignore.

### 2.2 GPU-driven rendering, visibility buffers, meshlets, virtual geometry

- Bevy has had GPU-driven rendering (GPU culling + multi-draw indirect) for "most standard 3D mesh rendering" since 0.16, "enabled automatically on supported platforms" [VERIFIED https://bevy.org/news/bevy-0-16/]; 0.19 moved more work to the GPU (`many_cubes` 49.47 → 18.77 ms) [VERIFIED https://bevy.org/news/bevy-0-19/].
- Virtual geometry (Nanite-style meshlet DAG with a visibility buffer) is still experimental in Bevy [VERIFIED-SEARCH dev-docs meshlet]. UE 5.7 extends Nanite to foliage (experimental) [VERIFIED-SEARCH unrealengine.com 5.7 post].
- For voxel terrain the winning pattern is different: **one storage buffer of packed quads per chunk, no index buffer, vertex pulling** (`vertex_index / 6` → quad, `vertex_index % 6` → corner), drawn with one indirect draw per chunk-face-direction so back-facing directions can be culled per chunk on the CPU [DESIGN; technique RECALLED from the binary-greedy-meshing renderer, which "skips faces that point away from the camera" VERIFIED https://github.com/cgerikj/binary-greedy-meshing].

### 2.3 Ray tracing and global illumination

| Technique | Status Oct 2026 | Source |
|---|---|---|
| Bevy Solari (realtime path tracing, ReSTIR DI/GI) | Experimental since 0.17; 0.18 added specular/reflections; 0.19 temporal stability; 0.20 macOS/Metal (no denoiser there), ReSTIR optional and off by default, DLSS-RR 4.5 denoise | [VERIFIED bevy.org 0-17/0-18/0-19/0-20 posts] |
| UE MegaLights (many shadowed area lights) | Beta 5.7 → production-ready 5.8 | [VERIFIED gamefromscratch UE 5.8] |
| UE Lumen Lite | New in 5.8; ~2× faster than Lumen per coverage | [VERIFIED-SEARCH guru3d/wnhub] |
| Radiance Cascades | 3D world-space extension "Split Radiance Cascades" (Freeman & Sannikov), sparse probes in a hashmap plus ray splitting, arXiv 2607.20384 submitted 2026-07-22; no shipped 3D game confirmed | [VERIFIED-SEARCH https://arxiv.org/pdf/2607.20384] |
| Voxel cone tracing / voxel GI | Mature technique (SVOGI, CryEngine; Godot's VoxelGI) | [RECALLED] |
| AMD Radiance Caching | 0.9 technical preview in FSR SDK 2.2 | [VERIFIED-SEARCH https://www.jonpeddie.com/news/amd-fsr-powered-rendering-gets-sharper/] |

**Steam Deck and RT.** Each RDNA2 CU has a ray accelerator, but with 8 CUs "such a slim number really won't make for a great experience" [VERIFIED-SEARCH https://gamingbolt.com/steam-deck-full-specs-revealed-features-custom-rdna-2-gpu-claims-1-6-tflops-of-power]; Doom Eternal RT ran ~35 fps; Portal RTX ~30 fps only via `RADV_PERFTEST=rt` as a feasibility study; Mesa 26.0 brought "significant raytracing performance improvements" for RADV [VERIFIED-SEARCH https://frvr.com/blog/valve-helps-amd-gpus-gain-significant-ray-tracing-and-vulkan-improvements-on-steam-deck-and-more/]. Rule: **no hardware RT in the baseline renderer.**

**What a voxel game should do instead.** A block world already has an exact, cheap GI proxy: **flood-fill light** (Minecraft-style block light + sky light, 0–15 levels, BFS through transparent cells) [RECALLED]. The brief already mandates a simulation light grid for stealth; uploading that grid as an `R8G8B8A8` 3D texture per chunk (RGB colored light + sky channel) and sampling it trilinearly in the fragment shader gives soft, colored, deterministic-by-construction bounce-ish light that *is* the stealth light [DESIGN]. Solari becomes an optional "Ultra" tier on RT GPUs.

### 2.4 Upscaling and frame generation

| Vendor tech | Facts | Source |
|---|---|---|
| NVIDIA DLSS 4.5 | Announced CES 2026 (article 2026-01-06). Second-gen transformer Super Resolution "5x more compute intensive", on all RTX; RTX 20/30 pay a "high penalty" (FP16-only tensor cores); RTX 40 benefits from FP8; **6× Multi Frame Generation and Dynamic MFG are RTX 50 only**, spring 2026; RR and Reflex unchanged by 4.5 | [VERIFIED https://www.notebookcheck.net/Nvidia-DLSS-4-5-brings-2nd-gen-Transformer-and-6x-multi-frame-gen-to-RTX-50-Blackwell-RTX-20-and-RTX-30-stand-to-benefit-with-caveats.1197464.0.html] |
| DLSS in Bevy | Since 0.17: RTX only, Vulkan backend on Windows/Linux only, upscaling + AA; 0.20 adds DLSS-RR 4.5 for Solari | [VERIFIED bevy.org 0-17, 0-20] |
| AMD FSR "Redstone" | Launched December 2025 for RDNA4: ML upscaling (FSR 4), ML frame generation, Ray Regeneration (ML denoiser); FSR SDK 2.2 = Upscaling 4.1, Ray Regeneration 1.1, Frame Generation 4.0 (preview), Radiance Caching 0.9 (preview); RDNA2/RDNA3 get "FSR Upscaling (Analytical)" only | [VERIFIED-SEARCH https://www.jonpeddie.com/news/amd-fsr-powered-rendering-gets-sharper/ ; https://www.guru3d.com/story/fsr-4-redstone-amd-new-ai-upscaling-arrives-december-10-officially/] |
| Intel XeSS 3 | Driver launch January 2026, multi-frame generation 2×/3×/4× (up to 3 generated frames), works on older Arc (A770, B580), XeSS 2 games get it without code changes; XeSS 3.0 SDK released | [VERIFIED-SEARCH https://www.guru3d.com/story/intel-ships-xess-30-sdk-with-multiframe-generation-support/] |

Implication: Steam Deck (RDNA2) and Steam Machine (RDNA3) get **no ML upscaler from AMD**. PALIMPSEST's baseline AA/upscale must be engine-side TAA (Bevy has TAA, SMAA, FXAA, CAS [RECALLED]); DLSS is a bonus on RTX. Frame generation is wrong for a 60 Hz sim with latency-sensitive aiming; do not design around it.

### 2.5 Neural texture compression and Gaussian splatting

- **NTC.** GTC 2026 demo: a scene's textures from 6.5 GB to 970 MB (~85%) with visual parity; depends on DirectX **Cooperative Vectors** (Shader Model 6.9 / Agility SDK); **no shipped commercial game found**; NVIDIA-first [VERIFIED-SEARCH https://itc.ua/news/neural-texture-compression-ot-nvidia-sokrashhaet-vram-s-6-5-gb-do-970-mb-bez-poter-kachestva/amp/]. Irrelevant: a voxel game's textures are small atlases/arrays.
- **Gaussian splats.** Khronos released the **KHR_gaussian_splatting** glTF extension as a release candidate on 2026-02-03; Cesium, Babylon.js, PlayCanvas ship it; compression is outside the core extension [VERIFIED-SEARCH https://www.khronos.org/news/permalink/khronos-announces-gltf-gaussian-splatting-extension]. Use case for PALIMPSEST: none in-world (splats cannot be destroyed per block); possibly for a static "Margin" hub backdrop. Skip for v1.

---

## 3. Target hardware tiers

| Device | CPU | GPU | Memory | Other | Source |
|---|---|---|---|---|---|
| **Steam Deck** (LCD/OLED) | 4C/8T Zen 2 [RECALLED] | 8 RDNA2 CUs (512 shaders), 1.0–1.6 GHz, up to 1.6 TFLOPS FP32 | 16 GB LPDDR5 unified [RECALLED] | 1280×800 [RECALLED] | [VERIFIED-SEARCH https://gamingbolt.com/steam-deck-full-specs-revealed-features-custom-rdna-2-gpu-claims-1-6-tflops-of-power] |
| **Steam Machine** (announced 2025-11-12) | Semi-custom Zen 4, 6C/12T, up to 4.8 GHz, 30 W TDP | Semi-custom RDNA3, **28 CUs**, 2.45 GHz max sustained, 110 W TDP | 16 GB DDR5 + 8 GB GDDR6 | Valve claims 4K60 with FSR | [VERIFIED https://insider-gaming.com/steam-machine-everything-to-know-about-valves-new-system/] |
| Steam Machine price/launch | — | — | — | **$1,049 (512 GB), $1,349 (2 TB)**; +Controller bundles $1,128 / $1,428; launch June 30 (2026) via randomized reservation list | [VERIFIED https://notebookcheck.net/Steam-Machine-release-date-and-price-officially-confirmed.1326720.0.html] |
| Steam Machine FP32 | — | 28 × 64 × 2 × 2.45 GHz = **8.78 TFLOPS** single-issue (17.6 with RDNA3 dual-issue) | — | — | [DERIVED] |
| **Steam Frame** (VR) | Snapdragon 8 Gen 3 (ARM64) | Adreno | 16 GB LPDDR5X | 2160×2160 per eye, 72–120 Hz (144 experimental); x86 games via FEX | [VERIFIED-SEARCH https://vr.org/steam-frame-specs] |
| **Switch 2** | 8× Cortex-A78C (6 for games), 998 MHz docked / 1101 MHz handheld | Ampere, 1536 CUDA cores, 1007 MHz docked / 561 MHz handheld; 3.072 / 1.71 TFLOPS; RT 20 / 10 gigarays/s | 12 GB LPDDR5X (9 GB for games), 102 / 68 GB/s | DLSS confirmed in titles | [VERIFIED https://www.nintendolife.com/news/2025/05/nintendo-switch-2-final-tech-specs-have-been-confirmed] |

[DESIGN] Performance contract: **Steam Deck at 1280×800, 60 fps render, 60 Hz sim, ≤ 6 GB RAM resident, ≤ 2.5 GB VRAM-equivalent** is the floor tier; Steam Machine at 1440p60 native or 4K via TAA upscale is the "Recommended" tier; Switch 2 docked (3.07 TFLOPS) sits between Deck and Steam Machine, so a Deck-passing build is a plausible later Switch 2 port. Ship Linux-native (SteamOS) and Windows; ARM64 Linux build is cheap with Rust and avoids FEX overhead on Steam Frame flatscreen mode [DESIGN].

---

## 4. Voxel engines and techniques

### 4.1 Reference engines

| Engine | Tech facts | Source |
|---|---|---|
| Minecraft Java | Moving from OpenGL to **Vulkan** with OpenGL/Vulkan switchable during transition; renderer on a separate thread per coverage; part of the Vibrant Visuals effort | [VERIFIED-SEARCH https://minecraft.net/article/another-step-towards-vibrant-visuals-for-java-edition] |
| Minecraft (classic facts) | 16×16 columns of 16³ sections; block light and sky light 0–15 via BFS flood fill; greedy-free per-face meshing with AO | [RECALLED] |
| Vintage Story | C# on OpenTK 4.8.2, OpenGL 3.3, fork of ManicDigger engine; 32³ chunks | [VERIFIED-SEARCH https://opentk.net/resources.html ; wikipedia]; chunk size [RECALLED] |
| Veloren | Rust multiplayer voxel RPG; 0.18 released 2026-01-24 | [VERIFIED-SEARCH https://opencollective.com/veloren/events]; wgpu renderer + `specs` ECS [RECALLED] |
| Teardown | In-house engine by Dennis Gustafsson; nearly all lighting, fog, shadows and reflections use voxels + software ray tracing without special hardware; large levels ≈ 0.5 billion voxels; multiplayer update 2026-03-12, up to 12 players | [VERIFIED-SEARCH https://www.pcgameshardware.de/Teardown-Spiel-73677/Specials/PC-Game-Test-Benchmark-Voxel-Dynamic-Destruction-1394545/2/ ; https://devnplay.substack.com/p/teardowns-massive-multiplayer-update]; per-object 3D-texture volumes with 8-bit palette, 10 cm voxels [RECALLED] |

### 4.2 Meshing

**Binary greedy meshing (cgerikj v2)** [VERIFIED https://github.com/cgerikj/binary-greedy-meshing]:
- Chunk input up to 64×64×64 **including neighbor padding** (so 62³ interior when padding is 1 on each side [DERIVED]).
- Step 1: 64×64 array of `u64` occupancy columns; step 2: 62×62 `u64` face masks per face direction; culls 64 faces per bitwise op.
- Output: **8 bytes per quad** — first 4 bytes: 6-bit x, y, z, width, height; last 4 bytes include 8 bits of voxel type.
- Time: **50–200 µs per chunk**; screenshot average **74 µs single-threaded, 108 µs in a thread pool** on a Ryzen 3800X. Baked AO only in the v1.0.0 branch.
- Rust port: `binary-greedy-meshing` 0.5.2 (2026-01-07) [CRATES]. Older `block-mesh` 0.2.0 is stale (2022) [CRATES].

[DESIGN] For PALIMPSEST's mandated 32³ chunks: mesh each chunk as its own job from a **34³ padded occupancy** (the chunk plus a 1-cell border copied from the 6 face neighbours). Each column fits in one `u64` with 34 of 64 bits used, so the cgerikj bit tricks apply unchanged and the 62-interior limit is never approached. Expected cost: well under 100 µs per chunk on the Deck's Zen 2 [DESIGN estimate scaled from the VERIFIED 3800X figure; a 32³ chunk has ~1/7.3 the cells of a 62³ interior]. Greedy merging must only merge faces whose key (texture layer, 4 AO corner values) is identical, otherwise baked per-vertex AO smears across merged quads [RECALLED standard practice]. Light needs no merge key because it is sampled per fragment from the chunk light texture, not baked into vertices [DESIGN].

Packed quad format (8 bytes) [DESIGN]:

| Word | Bits | Field |
|---|---|---|
| word0 | 0–4 | x (0–31) |
| word0 | 5–9 | y (0–31) |
| word0 | 10–14 | z (0–31) |
| word0 | 15–19 | width − 1 (0–31 → 1–32) |
| word0 | 20–24 | height − 1 (0–31 → 1–32) |
| word0 | 25–27 | face (0 = +X, 1 = −X, 2 = +Y, 3 = −Y, 4 = +Z, 5 = −Z) |
| word0 | 28–31 | reserved (0) |
| word1 | 0–15 | texture-array layer (material/skin) |
| word1 | 16–23 | AO, 2 bits per corner (c0,c1,c2,c3) |
| word1 | 24–31 | reserved (0) |

The vertex shader pulls `quad = quads[vertex_index / 6]`, corner = `vertex_index % 6` mapped through the two-triangle table `[0,1,2, 0,2,3]`, derives the normal from `face`, and samples light from the chunk's 3D light texture at the face's outward cell.

### 4.3 Sparse voxel octrees / DAGs

- SVDAG = compressed sparse voxel representation supporting real-time rendering and editing; Nov 2025 CGF paper (Modisett & Billeter) encodes structure in memory location; Molenaar & Eisemann (PG 2024) edit HashDAG fully on GPU with dynamic GPU hash tables at real-time rates, whereas original HashDAG edits on CPU at only interactive rates; Eurographics 2023 adds attribute (color) compression [VERIFIED-SEARCH https://diglib.eg.org/handle/10.2312/pg20241310].
- Verdict [DESIGN]: SVDAGs pay off for billions of tiny voxels (Teardown-scale). PALIMPSEST uses 1 m blocks in Folios of a few hundred thousand to a few million blocks. A flat `[u16; 32768]` per chunk (64 KiB) with palette compression for storage is simpler, faster to edit, and trivially deterministic.

### 4.4 Voxel data layout [DESIGN, consistent with brief §5]

| Item | Value |
|---|---|
| Chunk | 32×32×32 = 32,768 cells |
| Cell | `u16` BlockId (65,536 block states incl. genre skins) + separate per-chunk arrays for light (`u8` × 4: R, G, B, sky), liquid (`u8` level + `u8` type), gas (`u8` density + `u8` type), temperature (`i8` offset), sound (`u8`) |
| Index | `i = x + 32*z + 1024*y` (y-major slabs speed light/liquid gravity passes) |
| Storage on disk/network | palette (≤ 256 entries → `u8` indices, else raw `u16`) + run-length encoding, then `postcard` |
| Dirty tracking | per-chunk `u32` generation counter; mesher re-runs when generation changes |


---

## 5. Physics and determinism

### 5.1 Engines

| Engine | Version / date | Determinism facts | Notes | Source |
|---|---|---|---|---|
| Rapier (`rapier3d`) | 0.36.1, 2026-10-10 | "Locally deterministic" by default (same machine + same Rapier + same rustc). Cross-platform deterministic with feature `enhanced-determinism`; **cannot be combined with `simd8`**; `parallel` *can* be combined and matches sequential results regardless of thread count; targets must "strictly comply to the IEEE 754-2008" standard (incl. WASM); use `ComplexField`/`RealField` trig from nalgebra, not `f32::sin` | features: `enhanced-determinism`, `parallel`, `simd8`, `block-solver`, `fem`, `serde-serialize` | [VERIFIED https://rapier.rs/docs/user_guides/rust/determinism] [CRATES] |
| `bevy_rapier3d` | 0.37.0, 2026-10-10, Bevy 0.20 | inherits Rapier | — | [CRATES] |
| Avian (`avian3d`) | 0.7.0, 2026-06-20, Bevy 0.19 | feature `enhanced-determinism` exists; README does not claim finished cross-platform determinism | ECS-native; f32/f64; joints fixed/distance/prismatic/revolute/spherical; CCD; XPBD custom constraints; physics runs in `FixedPostUpdate` since 0.5 | [VERIFIED https://github.com/avianphysics/avian] [CRATES] [VERIFIED-SEARCH docs.rs avian3d] |
| Jolt | 5.6, July 2026 (GitHub release 11 Jul; coverage 2026-07-13) | "The simulation runs deterministically"; inputs-only replication possible | Used by Horizon Forbidden West and Death Stranding 2; 5.6 adds GPU compute interface (DX12/Vulkan/Metal), WIP GPU strand hair, new friction model (15% faster, 40% less memory in Pyramid test), RISC-V RVV; Godot 4.6 default | [VERIFIED https://github.com/jrouwe/JoltPhysics] [VERIFIED https://gamefromscratch.com/jolt-physics-5-6-released/] |
| Box3D (Erin Catto) | debut ~2026-07-02, alpha | one secondary source claims cross-platform determinism (unconfirmed) | C17, forked from Box2D, partly derived from Valve's "Rubikon-Lite" (Half-Life: Alyx) with Dirk Gregorius; used in s&box and Esoterica; MIT per coverage | [VERIFIED-SEARCH https://phoronix.com/news/Box3D-Open-Source-3D-Physics] |

### 5.2 Why PALIMPSEST's sim does not use a physics engine [DESIGN, consistent with brief §5]

The brief already fixes custom swept capsules vs voxel grid inside the sim, and cosmetic physics only in the renderer. The research supports this: (a) the only engines with documented cross-platform determinism are float engines whose guarantee depends on IEEE strictness and careful trig sourcing; (b) none of them use fixed-point; (c) a first-person voxel crawler needs only capsule-vs-grid sweeps, capsule-vs-capsule hits, and ray-vs-grid DDA, all of which are ~300 lines each in integer math. Client-side Avian/Rapier then handles ragdolls, gibs, falling debris and loot-drop bounces, which never feed back.

### 5.3 Determinism rules for Rust (checklist for the canon)

1. No `f32`/`f64` in sim state or sim decisions. IEEE basic ops are reproducible, but transcendental functions differ across libm implementations; Rapier's own docs warn about `sin`/`cos` [VERIFIED rapier.rs].
2. No iteration over `std::collections::HashMap`/`HashSet` in sim logic (randomized SipHash seeds per process) [RECALLED]. Use `Vec` sorted by stable IDs, `BTreeMap`, or `indexmap` 2.14.2 [CRATES] with insertion order.
3. No wall-clock, no thread scheduling dependence: parallelism only for pure functions whose results are merged in index order (e.g. `rayon` 1.12.0 `par_iter().map().collect()` preserves order) [CRATES; RECALLED semantics].
4. Integer overflow is a desync: build sim with `overflow-checks = true` in all profiles, and use explicit `wrapping_*`/`saturating_*` where intended [RECALLED Cargo profile option].
5. Every subsystem RNG is a named stream; draw count per tick must not depend on presentation (e.g. no RNG in VFX code paths that share a stream).
6. Per-tick state hash (`xxhash-rust` 0.8.19 `xxh3_64` over a canonical serialization) logged in replays and exchanged between host and clients every 60 ticks for desync detection [CRATES; DESIGN].

### 5.4 Fixed-point in Rust

- **`fixed` 1.31.0** (2026-03-20) [CRATES]: generic `FixedI8..FixedI128<Frac>` / `FixedU*`; aliases for all integer/fraction splits (e.g. `I16F16`, `I20F12`); **`sqrt` exists; "No trigonometric functions are provided"; "No algebraic functions … no `pow`"**; wrappers `Saturating`, `Wrapping`, `Strict` (panics on overflow even in release); optional `serde` and `serde-str` features; **MSRV 1.93.0** [VERIFIED https://docs.rs/fixed/1.31.0/fixed/].
- Companion crates: `cordic` 0.1.5 (2021, stale), `fixed_trigonometry` 0.4.4 (2024), `fixed-sqrt` 0.3.0 (2025), `az` 1.3.0 for checked casts [CRATES].
- Integer square root is in std: `<int>::isqrt`, `<int>::checked_isqrt`, `<uint>::isqrt` stabilized in **Rust 1.84.0** [VERIFIED https://blog.rust-lang.org/2025/01/09/Rust-1.84.0/].

[DESIGN] Canon proposal:

| Quantity | Type | Range / resolution |
|---|---|---|
| Positions, distances (m) | `Fx = FixedI64<U16>` (I48F16) | ±1.4×10¹⁴ m, 1/65,536 m ≈ 15.3 µm |
| Velocities (m/tick) | `Fx` | same |
| Angles | `u16` binary angle (65,536 = 1 turn) | 0.0055° |
| sin/cos | 4,096-entry quarter-wave LUT of `i32` Q1.30 built at compile time from a checked-in table file | exact by construction |
| sqrt | `u64::isqrt`/`u128::isqrt` on raw bits with shift | exact |
| Damage, HP, stats | `i32` integers (HP ×100 if fractions needed) | exact |
| Probabilities | `u16` basis points 0–10,000 (brief §5) | exact |
| Multipliers | `i32` per-mille or basis points, applied as `(x * m + 5000) / 10000` with **round-half-up defined once** | exact |

Brief-mandated 60 Hz tick: 1 tick = 1/60 s; express speeds as m/s in data and convert once at load as `Fx::from_num(v) / 60` [DESIGN].

---

## 6. Netcode

### 6.1 Models

| Model | How | Fit for 1–4 player first-person co-op with destructible voxels |
|---|---|---|
| Deterministic lockstep | everyone exchanges inputs, sim advances when all inputs for tick N arrive | Bad: one laggy peer stalls everyone; input latency = worst RTT |
| Rollback (GGPO/GGRS) | predict remote inputs, roll back and re-simulate on correction | Great for 2-player fighting games with tiny state; re-simulating a voxel world with liquids for 8–15 ticks every frame is too expensive |
| Snapshot + client prediction + reconciliation (Quake/Source/Overwatch family) | host simulates; clients predict own avatar, interpolate others, reconcile to snapshots; lag-compensated hits | **Correct choice** (and what the brief mandates) |
| Pure state replication (no prediction) | host replicates components, clients render | Fine for NPC/world state, not for local movement feel |

### 6.2 Rust crate status (2026-10-11)

| Crate | Latest | Bevy | Facts | Source |
|---|---|---|---|---|
| `ggrs` / `bevy_ggrs` | 0.13.0 / 0.22.0 (2026-06-26) | 0.19 | GGPO reimagination; game supplies save/load/advance; `SyncTestSession` re-simulates every frame over `check_distance` and compares checksums; desync detection between peers; **no NAT traversal or signaling** | [CRATES] [VERIFIED-SEARCH https://docs.rs/ggrs] |
| `lightyear` | 0.30.1 (2026-09-16) | 0.19 (0.28–0.30) | Transports: UDP, WebTransport, WebSocket, **Steam** (via `aeronet`); client-server or P2P; **host-client mode**; one-line prediction + rollback; snapshot interpolation; inputs buffered per tick and resent across several frames; bandwidth cap + priority accumulation; lag compensation (predicted vs interpolated entities); lockstep/rollback with input-only replication when sim is deterministic; uses `bevy_replicon` for world replication | [VERIFIED https://github.com/cBournhonesque/lightyear] |
| `bevy_replicon` | 0.45.0 (2026-10-11) | 0.20 | server-authoritative replication; singleplayer/client/dedicated/listen server; per-client visibility; no built-in I/O; backends `bevy_replicon_renet`, `_renet2`, `_quinnet`, `aeronet_replicon`, `bevy_replicon_matchbox`; prediction only via third-party (`bevy_rewind`; `bevy_replicon_snap` unmaintained) | [VERIFIED https://github.com/simgine/bevy_replicon] |
| `renet` | 2.0.0 (2026-01-20) | Bevy-agnostic core | reliable/unreliable channels over UDP, netcode.io-style auth | [CRATES]; protocol [RECALLED] |
| `aeronet` | 0.21.0 (0.22.0-rc.1 2026-09-23) | — | transport layer used by lightyear for Steam/WebTransport | [CRATES] [VERIFIED lightyear README] |
| `matchbox_socket` | 0.14.0 (2026-02-13) | 0.18 | WebRTC P2P with signaling | [CRATES] |
| `quinn` | 0.11.12 (2026-09-14) | — | QUIC | [CRATES] |
| `steamworks` | 0.13.1 (2026-05-05) | — | Rust bindings to the Steamworks SDK | [CRATES] |

Lightyear compatibility table: 0.28–0.30 → Bevy 0.19; 0.26–0.27 → 0.18; 0.25 → 0.17; 0.20–0.24 → 0.16 [VERIFIED lightyear README]. Replicon: 0.45 → Bevy 0.20; 0.41–0.44 → 0.19; 0.38–0.40 → 0.18 [VERIFIED replicon README].

**Why PALIMPSEST should not adopt lightyear or replicon wholesale [DESIGN].** Both replicate *Bevy ECS components*. The brief puts all authoritative state in a Bevy-free `palimpsest_sim`. Prediction in PALIMPSEST is therefore "run the same deterministic sim function on the client for the local avatar", which needs: input ring buffer, snapshot ring buffer, `restore(snapshot) + resim(inputs)`. That is ~1,500 lines owned by the project, testable headlessly, and independent of Bevy's release cadence. Lightyear is the **reference design** to copy concepts from (input redundancy, priority accumulation, host-client mode, lag compensation between predicted and interpolated timelines).

### 6.3 Transports and platform relays

- **Steam Datagram Relay (SDR)**: "can be used for both peer-to-peer traffic and dedicated servers"; "IP addresses are never revealed"; traffic "authenticated, encrypted, and rate-limited"; P2P via `CreateListenSocketP2P` / `ConnectP2P` ("Steam will take care of everything else"); APIs `ISteamNetworkingSockets`, `ISteamNetworkingMessages` (address by SteamID), `ISteamNetworkingFakeUDPPort`; **the open-source GameNetworkingSockets "does not support accessing the relay network"**; pricing: "contact us … fine print" [VERIFIED https://partner.steamgames.com/doc/features/multiplayer/steamdatagramrelay]. GameNetworkingSockets v1.5 released April 2026 after v1.4.1 in June 2022 [VERIFIED-SEARCH https://phoronix.com/news/GameNetworkingSockets-1.5].
- **Epic Online Services P2P**: NAT punch-through first, automatic fallback to relay; relay not meant to act as an authoritative server [VERIFIED-SEARCH https://edgegap.com/blog/can-epic-online-services-eos-relays-allow-for-dedicated-server-or-authoritative-server]; crossplay tooling free to developers per 2022 announcement [VERIFIED-SEARCH https://gamespot.com/articles/epic-is-making-crossplay-more-feasible-with-epic-o/1100-6477256/]; EOS P2P relayed-connection incident in EU Dec 2025, maintenance windows in 2026 [VERIFIED-SEARCH https://status.epicgames.com/incidents/x518l7vkc3nc].
- **Barony model**: developer (2018) — Steam only negotiates the lobby, in-game traffic is peer-to-peer [VERIFIED-SEARCH https://steamcommunity.com/app/371970/discussions/0/1743355067116697177]; the open-source repo has CI workflows named `Linux-CI_fmod_steam` and `Linux-CI_fmod_steam_eos`, i.e. Steam and EOS builds; no dedicated server exists ("Create a dedicated server" is listed as a future idea) [VERIFIED https://github.com/TurningWheel/Barony]. Barony is 4-player listen-host co-op with Steam/EOS crossplay [RECALLED].

### 6.4 PALIMPSEST netcode parameters [DESIGN]

| Parameter | Value | Rationale |
|---|---|---|
| Topology | Listen host (one player's machine runs authoritative sim) + optional headless dedicated host binary (same code) for daily challenges | Barony-proven; the dedicated binary is free because sim is headless |
| Production transport | Steam `ISteamNetworkingMessages` over SDR (channels: 0 reliable-ordered control, 1 unreliable inputs, 2 unreliable snapshots, 3 reliable chunk deltas) | NAT + IP privacy solved; zero server cost for P2P |
| Dev/LAN/CI transport | `renet` 2.0.0 UDP; in-process `crossbeam-channel` 0.5.17 transport for tests | deterministic tests without sockets |
| Epic store build | EOS P2P behind the same `Transport` trait | Barony precedent |
| Sim tick | 60 Hz (brief) | — |
| Client → host input send | every tick, each packet carries the last **8** ticks of inputs | survives 7 consecutive drops; idea from lightyear's input redundancy |
| Host → client snapshot | every **2** ticks (30 Hz), delta vs last acked snapshot, entity priority accumulation, budget **96 kbit/s per client** | 4 players ≈ 384 kbit/s upstream for the host, fine on home broadband |
| Remote entity interpolation delay | **6 ticks (100 ms)** + adaptive +2 ticks per 5% measured loss, cap 12 | Source-engine-style 100 ms default [RECALLED cl_interp 0.1] |
| Prediction | local avatar movement, local weapon fire visuals, local ability cast start; on correction the client re-simulates **only the local avatar subsystem** (movement + collision against the latest host-confirmed voxel state), never the whole world | everything else is host-confirmed; this is what keeps resim cheap where whole-world rollback (GGRS) is not |
| Max rollback/resim | **15 ticks (250 ms)**; beyond that, hard-snap with 120 ms visual smoothing | bounds CPU |
| Lag compensation | host keeps a **20-tick (333 ms)** ring of hitbox capsules; rewinds to client's `(tick − interp_delay)` capped at 200 ms | favors the shooter in PvE co-op |
| Voxel edits | host-ordered edit log (reliable channel), each edit = `(tick, chunk_id, cell_idx, old_block, new_block, cause_id)`; clients apply optimistically for their own digging, roll back on host rejection | deterministic, compact |
| Desync check | `xxh3_64` of canonical sim state every 60 ticks; mismatch → client requests full resync of affected chunks | GGRS-style idea applied to snapshots |
| Join-in-progress | host streams chunk palette/RLE blobs, 64 KiB per tick budget | — |

---

## 7. Procedural generation

### 7.1 Wave Function Collapse and model synthesis

- WFC (mxgmn): **overlapping model** learns N×N patterns from one sample (typical N = 3); **simple tiled model** = adjacency constraint propagation over a tileset; observe = pick the cell with **lowest Shannon entropy**, collapse by pattern frequency, propagate; contradiction = a cell with zero options → the algorithm "can not continue"; contradictions are "surprisingly rare"; uses **AC-4** propagation where Merrell's model synthesis used **AC-3**; games listed: Bad North, Caves of Qud, Dead Static Drive, Townscaper, The Matrix Awakens [VERIFIED https://github.com/mxgmn/WaveFunctionCollapse].
- Paul Merrell (2021 comparison): MS and WFC are "two versions of the same algorithm"; WFC "has great difficulty generating some large textures and models. It repeatedly fails", largely due to the lowest-entropy heuristic; WFC omits **modifying in blocks** (generate a large output as overlapping sub-blocks, re-solving a block on failure instead of restarting the whole output) [VERIFIED-SEARCH https://paulmerrell.org/wp-content/uploads/2021/07/comparison.pdf].
- [DESIGN] Use WFC **only inside rooms** (≤ 24×12×24 cells) with 3D tile sets per genre skin, using Merrell-style block modification and a hard cap of 3 retries before falling back to a hand-authored room template. Never run WFC over a whole Folio.

### 7.2 Dormans cyclic generation (Unexplored)

Exact pipeline [VERIFIED https://www.boristhebrave.com/2021/04/10/dungeon-generation-in-unexplored/]:
1. Layout on a **5×5 grid of graph nodes**: place start, goal, a rough circular loop; pick a **major cycle type** (there are **24 cycle types**, e.g. `ThreeLeverHub`, hubs, the lake "cycle"); add minor cycles, dead ends, shuffle nodes.
2. Resolve: puzzles, locks, doors, enemies, room/cave/corridor typing, treasure.
3. Graph → tilemap: **double** the grid to insert corridor/barrier cells, **expand 5×**, draw rooms/corridors, set pieces, terrain noise, smoothing, vegetation.
- Scale: two generators, **~5,000 find-replace rules**, **~50 PhantomGrammar modules** for the floorplan, **40 rules** just to draw the circle; authored in Ludoscope.
- Locks/keys are non-terminals; key ∈ {item, switch, knowledge}; locks hard or soft; a key holds a special edge to its lock so shuffling keeps them paired; the same data powers an in-game "Pray For Help" progression hint.
- Cyclic generation was devised by Joris Dormans; a simpler re-implementation picks random nodes and connects them by shortest path [VERIFIED-SEARCH https://oopolo.itch.io/dungeon-of-doom/devlog/818069/from-graphs-to-gameplay-a-deep-dive-into-cyclic-dungeon-generation].

### 7.3 Caves of Qud generation lineage

Bucklew & Grinblat GDC 2019 "End-to-End Procedural Generation in Caves of Qud" (villages with histories, cultures, architecture, NPCs, quests); Bucklew's WFC map-generation talk at Roguelike Celebration 2019; Grinblat GDC 2018 "Procedurally Generating History" (5 sultans per game, events generated then rationalized after the fact) [VERIFIED-SEARCH https://www.gdcvault.com/play/1024990/Procedurally-Generating-History-in-Caves].

### 7.4 PALIMPSEST Folio generator pipeline [DESIGN]

1. **Mission graph** (Dormans-style, but data-driven and small): a typed graph grammar with ≤ 60 rules total, nodes `{Start, Exit×2–3, Room, Arena, Vault, Shrine, Station, Secret}`, edges `{Open, Locked(KeyId), OneWay, Hidden}`; 1 major cycle from a table of 8 cycle types (lock-key loop, hidden shortcut, gauntlet-and-return, two-key hub, flooded detour, collapsing bridge, patrol loop, foreshadowed vault).
2. **Graph embedding** onto a coarse 3D grid of 16×8×16-block "cells" (proposal: a Folio is at most 8×4×8 cells = 128×32×128 blocks = 4×1×4 = 16 chunks; the exact Folio size is a canon decision), using backtracking placement with max 200 attempts per node.
3. **Room fill** by WFC/model synthesis per room using the current Genre's tile set; genre bleed = tile-set mixing weights along the edge between two genres' rooms.
4. **Validation** (hard constraints, run on every generated Folio in CI for seeds 0–9,999): every Exit reachable from Start for a 1-block-wide, 2-block-tall capsule; every Locked edge's key reachable before the lock (topological check over the mission graph); no liquid source above Start within 8 blocks; loot budget within ±10% of target; generation time ≤ 150 ms on Deck CPU.
5. **Repair before reject**: dig a 1×2 corridor along the A* path when reachability fails; only reject the seed (advance to sub-seed) after 3 repair attempts.

### 7.5 LLM-assisted content pipelines (offline, build-time)

[DESIGN] Use a frontier LLM at **authoring time** (not runtime) to draft genre skins, lexicons, item names, affix flavor, monster descriptions, bark tables and Tracery-style template grammars; every output must validate against JSON Schemas generated from the Rust types (`schemars` 1.2.2 → `jsonschema` 0.58.6 in CI) [CRATES], plus lint rules (banned words, length caps, no real-person names). Generated text ships as data, so it is pre-generated content that must be disclosed on Steam (see §8.5). Text-to-3D (TRELLIS.2, MIT, 4B params, ~3 s at 512³ on H100 per vendor-adjacent coverage) is usable for prop drafts but outputs meshes, not block structures [VERIFIED-SEARCH https://www.codesota.com/news/trellis-2-3d-generation]. World models (Google Genie 3 via "Project Genie", US AI Ultra subscribers, launched 2026-01-29, 60-second sessions) are research toys, not shippable game tech [VERIFIED-SEARCH https://gigazine.net/gsc_news/en/20260130-google-project-genie].

---

## 8. Runtime AI: on-device small language models

### 8.1 Models (open weights, 2026)

| Model | Size facts | License | Source |
|---|---|---|---|
| Gemma 3n E2B / E4B | E2B loads "over 5 billion parameters" in standard execution, effective memory "1.91B" with PLE caching + parameter skipping; E4B contains E2B's parameters (nested); audio + vision + text input; **32K context** | Gemma terms [RECALLED] | [VERIFIED https://ai.google.dev/gemma/docs/3n] |
| **Gemma 4** (2026-04-02) | E2B, E4B (Per-Layer Embeddings, on-device), 26B-A4B MoE, 31B; LM Studio GGUF files: 4.41 GB / 6.33 GB / 17.99 GB / 19.89 GB | **Apache 2.0** | [VERIFIED https://simonwillison.net/b/9404] |
| **Qwen3.5 Small** (early March 2026) | dense 0.8B, 2B, 4B, 9B; native vision; hybrid thinking/non-thinking; **262K context**; Artificial Analysis Intelligence Index 9 / 16 / 27 / 32 | **Apache 2.0** | [VERIFIED https://artificialanalysis.ai/articles/qwen3-5-small-models] |
| Mistral-NeMo-Minitron (NVIDIA ACE) | inZOI "Smart Zoi" uses a ".5B" (0.5B) parameter model, "on-device", "GPU-accelerated exclusively on GeForce RTX" (2025-03-13) | NVIDIA | [VERIFIED https://www.nvidia.com/en-gb/geforce/news/nvidia-ace-naraka-bladepoint-inzoi-launch-this-month/] |

### 8.2 Runtimes

| Runtime | Version (2026-10-11) | Notes | Source |
|---|---|---|---|
| llama.cpp via `llama-cpp-2` | 0.1.159 (2026-10-07) | CPU, Vulkan, Metal, CUDA backends [RECALLED]; Vulkan generally fastest for token generation on AMD [VERIFIED-SEARCH https://github.com/ggml-org/llama.cpp/discussions/10879] | [CRATES] |
| ONNX Runtime / ORT GenAI | onnxruntime 1.31.0 (2026-10-09), onnxruntime-genai 0.17.1 (2026-09-29); Rust `ort` 2.0.0-rc.13 (still RC) | DirectML/CUDA/CoreML EPs [RECALLED] | [PyPI API] [CRATES] |
| WebLLM | `@mlc-ai/web-llm` 0.2.86 (2026-10-11) | paper: Llama-3.1-8B 41.1 tok/s in browser vs 57.7 native (71.2%), Phi-3.5-mini 71.1 vs 89.3 (79.6%) on M3 Max | [npm API] [VERIFIED-SEARCH https://arxiv.org/html/2412.15803v1] |
| NVIDIA ACE | in-game SLM runtime, RTX-only | shipped in inZOI and NARAKA PC (2025) | [VERIFIED nvidia.com article] |

No public llama.cpp benchmark for the Steam Deck was found (open question). A Framework mainboard on Vulkan ran Llama 3.2 3B Q4_K_M at ~4.47 tok/s generation [VERIFIED-SEARCH https://github.com/geerlingguy/ai-benchmarks], which bounds expectations for APUs: assume **≤ 15 tok/s for a 0.8B Q4 model on Deck while the game renders** [DESIGN estimate, must be measured].

### 8.3 Grammar-constrained output

llama-server converts an OpenAI-style `response_format` JSON Schema into a grammar per request; `/completion` takes `json_schema`; hand-written GBNF via `grammar`; supported keywords include object/array/string/number/integer/boolean/null/enum/const/required/additionalProperties/anyOf/oneOf/$ref and length/pattern constraints; **unsupported keywords are silently ignored** so outputs must still be validated in code [VERIFIED-SEARCH https://docs.liquid.ai/deployment/on-device/llama-cpp/structured-output]. The model cannot see the schema; the prompt must restate it.

### 8.4 Shipped games

inZOI (Smart Zoi) and NARAKA: BLADEPOINT PC (ACE teammate) ship on-device models, RTX-only [VERIFIED nvidia.com]. A June 2026 forecasting-market comment lists Suck Up!, Wanderfolk and Smart Zoi as the shipped LLM-NPC titles, all sub-$60, and says Ubisoft's "Teammates" is still a prototype [VERIFIED-SEARCH https://manifold.markets/BoltonBailey/will-a-major-video-game-released-in] (anonymous source, weak). Arm (Aug 2026) notes engines give "limited support for running LLMs with more than 400M parameters efficiently on mobile" [VERIFIED-SEARCH https://developer.arm.com/community/arm-community-blogs/b/mobile-graphics-and-gaming-blog/posts/first-contact-a-mobile-architecture-for-llm_2d00_powered-game-characters].

### 8.5 Steam disclosure

Valve's January 2026 revision: developers must disclose generative AI used for content that ships (artwork, sound, story, localization, store text, marketing) **and** whether the game "generates AI content during gameplay"; live-generated content requires safeguards against inappropriate/illegal output; AI tools used "behind the scenes for efficiency" (coding assistants) need no disclosure [VERIFIED https://www.notebookcheck.net/Steam-updates-AI-disclosure-form-requiring-developers-to-report-visible-and-in-game-AI-but-not-background-tools.1206103.0.html]. Live AI-generated adult sexual content is prohibited outright [VERIFIED-SEARCH https://gigazine.net/gsc_news/en/20260120-steam-updates-ai-disclosure-guidelines].

### 8.6 PALIMPSEST "Scribe" runtime text architecture [DESIGN, consistent with brief §5]

| Rule | Value |
|---|---|
| Default model | Qwen3.5-0.8B in non-thinking mode, Q4 GGUF (Apache-2.0); "Rich" option Gemma 4 E2B or Qwen3.5-2B; downloaded as optional DLC-like depot so base install stays small |
| Runtime | `llama-cpp-2` 0.1.159, Vulkan backend if VRAM headroom ≥ 1.5 GB else CPU with **2 threads max** at below-normal priority |
| When it runs | only between Folios (exit-choice screen, loading) and in the hub; never on the frame's critical path; a Folio's text jobs must finish before the Folio starts or the template output is used |
| Output | JSON constrained by a per-job schema, `maxLength` on every string, enum-only fields for anything referencing game IDs |
| Cache key | `blake3(model_id ‖ prompt_template_version ‖ run_seed ‖ folio_index ‖ job_id)`; cached on disk; identical seeds in co-op share host-generated text via the reliable channel (host generates, clients receive) |
| Safety | blocklist + length checks + schema validation; on any failure → deterministic template grammar output |
| Sim isolation | sim never reads Scribe output; text is presentation |
| Disclosure | Steam "live-generated content" checkbox + in-game toggle (default ON only if model files are installed) |

---

## 9. Game AI architectures and navigation

| Architecture | Canonical source | Strength | Use in PALIMPSEST [DESIGN] |
|---|---|---|---|
| Finite state machine | — | trivial | animation/stance layer only |
| Behavior tree | Halo 2 lineage [RECALLED] | authorable, debuggable | boss scripts |
| Utility AI (scored considerations, response curves) | Dave Mark, Infinite Axis Utility System (GDC 2010–2015) [RECALLED] | smooth, data-driven, cheap | **default for all NPCs**: per-action score = product of considerations, each a curve over an integer input, compensated for consideration count |
| GOAP | Jeff Orkin, F.E.A.R., GDC 2006 "Three States and a Plan" — FSM with only three states, A* plans actions as well as paths [VERIFIED-SEARCH https://gdcvault.com/play/1013282/Three-States-and-a-Plan] | emergent multi-step plans | stealth-search squads only (≤ 8 actions, depth ≤ 4) |
| HTN | Killzone 2 / Horizon (Guerrilla) [RECALLED] | designer-controlled plans | not needed |

Crates: `big-brain` 0.22.0 last released 2024-11-30 (stale); `bonsai-bt` 0.14.0 (2026-09-22, engine-agnostic); `dogoap` 0.5.0 (2025-05); `bevy_behave` 0.5.0 (Bevy 0.18) [CRATES]. [DESIGN] Implement utility AI inside `palimpsest_sim` (integer math, deterministic tie-break by entity ID); no crate.

Navigation: `pathfinding` 4.16.0 (2026-09-07, generic A*/Dijkstra/BFS, engine-agnostic) [CRATES]. [DESIGN] Grid A* over walkable cells (solid below, 2 air above for humanoids) with per-chunk HPA* portals, rebuilt incrementally on voxel edits (edits mark the chunk's portal graph dirty); flying and swimming use separate cost layers; path requests budgeted at 2 ms per tick on host, FIFO with priority for combat-state NPCs.

---

## 10. Audio

- **Steam Audio**: fully open source under **Apache-2.0** since 4.5.2 (Feb 2024) [VERIFIED-SEARCH https://phoronix.com/news/Steam-Audio-SDK-Fully-Open]; releases v4.7.0 (11 Aug; energy fields, impulse responses, reconstructors, faster pathing, Embree 4.4.0), v4.8.0 (26 Nov; dynamic material changes, static geometry export during gameplay), **v4.8.1 (11 Feb)** [VERIFIED https://github.com/ValveSoftware/steam-audio/releases]; years inferred 2025/2025/2026 from ordering [DERIVED].
- **`audionimbus` 0.16.0** (2026-08-08) "currently tracks Steam Audio 4.8.1", `auto-install` feature downloads platform libs, `bevy` feature runs simulations on dedicated threads, FMOD/Wwise integrations [VERIFIED https://static.crates.io/readmes/audionimbus/audionimbus-0.16.0.html].
- **Firewheel** 0.14.0 audio graph engine; **`bevy_seedling` 0.8.0** integrates it with Bevy 0.19 and requires disabling Bevy's `bevy_audio` feature [CRATES] [VERIFIED https://static.crates.io/readmes/bevy_seedling/bevy_seedling-0.8.0.html]. Alternative: `kira` 0.12.5 / `bevy_kira_audio` 0.27.0 (Bevy 0.20) [CRATES].

[DESIGN] **Two acoustic systems, never confused.** (1) *Gameplay acoustics* in the sim: integer sound-propagation flood fill over the voxel grid (each air cell costs 1, door 4, water 2, solid blocks impassable except "thin" blocks at 8), loudness in dB-like integer units decremented by cost; NPC hearing reads this grid; deterministic, drives stealth. (2) *Presentational acoustics* on the client: Steam Audio HRTF + occlusion + reflections, scene = greedy-meshed chunk geometry registered as Steam Audio dynamic/instanced meshes, rebuilt with the render mesh. The renderer may sound better than the sim "hears"; the sim is the truth.

---

## 11. Modding

- **WASI 0.3.0 ratified 2026-06-11**: async is native to components (`async func`, `stream<T>`, `future<T>` in the canonical ABI); **Wasmtime 46 ships it with component-model async on by default**; guest toolchains (Rust via `wit-bindgen`, Python, JS, C#, C, Go) still rolling out [VERIFIED https://bytecodealliance.org/articles/WASI-0.3].
- Versions: `wasmtime` 49.0.2 stable (49.0.0 on 2026-09-21; 50.0.0-rc.1 on 2026-10-05); `wit-bindgen` 0.62.0 [CRATES].
- **Interruption**: fuel "always stops the same program, given the same fuel, at the same location" but costs overhead; epochs are cheaper but wall-clock-based, so not reproducible; deterministic execution additionally requires deterministic host functions, virtualized clocks/FS, NaN canonicalization, and deterministic or disabled relaxed SIMD [VERIFIED-SEARCH https://docs.wasmtime.dev/examples-interrupting-wasm.html ; https://docs.wasmtime.dev/examples-deterministic-wasm-execution.html].
- Alternatives: `mlua` 0.12.2 (Lua/Luau), `bevy_mod_scripting` 0.22.0 (Bevy 0.20), `extism` 1.41.0 (WASM plugin framework) [CRATES].

[DESIGN] Mods = (a) pure data packs (RON validated against the same JSON Schemas as base content), (b) WASM components implementing WIT worlds such as `palimpsest:mod/item-effect`, `palimpsest:mod/folio-generator`, `palimpsest:mod/ai-consideration`. Sim-side mods run with **fuel metering** (budget e.g. 2,000,000 fuel per tick per mod, deterministic trap → mod disabled for the run), `canonicalize_nans(true)`, relaxed SIMD off, no WASI clocks/random (host provides tick and named RNG stream). Mods ship as content-addressed blobs (`blake3`) so all co-op peers verify identical mod sets before session start. No Lua: one sandbox, one ABI.

---

## 12. Data formats, schemas, serialization

| Need | Choice | Version | Why |
|---|---|---|---|
| Authored content (items, affixes, genres, recipes) | RON | `ron` 0.12.2 | Rust-native enums/structs, comments, Bevy-native [RECALLED] |
| Schema | JSON Schema generated from Rust types | `schemars` 1.2.2 | single source of truth = Rust types |
| CI validation of all content + LLM outputs | — | `jsonschema` 0.58.6 | fails the build on any invalid record |
| Saves, net messages, chunk blobs | postcard (documented stable wire format) | `postcard` 1.1.3 | compact, `no_std`, serde [RECALLED wire-format spec] |
| Fast bit-packed net payloads (optional) | bitcode | 0.6.9 | smaller than postcard for snapshots [RECALLED] |
| Zero-copy asset caches | rkyv | 0.8.18 | mmap-able baked tables |
| Cross-language service API (backend) | protobuf via prost | 0.14.4 | only at the backend boundary |
| FlatBuffers | 25.12.19 | — | not needed |
| Avoid | bincode | 3.0.0 (2025-12-16) | final release is only a README plus a lib.rs containing a compiler error: "Bincode is now unmaintained" after a doxxing/harassment incident; README recommends wincode, postcard or rkyv [VERIFIED https://static.crates.io/readmes/bincode/bincode-3.0.0.html] |

All versions [CRATES]. Every serialized blob starts with `magic: [u8;4]`, `schema_version: u16`, `content_hash: u64` (xxh3 of the content database) so replays and saves refuse to load against mismatched content [DESIGN].

---

## 13. Platform economy services (gacha, loot boxes, battle pass)

### 13.1 Steam Inventory Service — exact schema facts [VERIFIED https://partner.steamgames.com/doc/features/inventory/schema]

- Item types: `item`, `bundle`, `generator` ("randomly selects one item type from bundle"), `playtimegenerator` (granted via `ISteamInventory::TriggerItemDrop`), `tag_generator`.
- `itemdefid`: non-Workshop items must be < 1,000,000 (max 2,147,483,647).
- Bundle grammar: `bundle_def : item_recipe , { ";" , item_recipe }`, `item_recipe : item_def , [ "x" , quantity ]`; bundle `"101x1;102x5"`; generator weights need not sum to 100: `"501x90;502x9;503x1"`; bundles/generators can chain; `container_contents_generator` shows a container's possible drops.
- Exchange grammar: `"exchange":"100,101;102x5;103x3,104x3"`, tag recipes `"type:tree*3,quality:fancy"`, `"rarity:common*5"`.
- Promo rules: `owns:<appid>`, `ach:<name>`, `played:<appid>/<minutes>`, `manual`.
- Playtime drops: `drop_interval` (minutes), `use_drop_window`, `drop_window` (minutes), `drop_max_per_window` (default 1, **cap 10**), `use_drop_limit`/`drop_limit`; example "three drops per day after 90 minutes": `drop_interval 30, use_drop_window true, drop_window 1440, drop_max_per_window 3`.
- Price: `"1;USD100,EUR080"`; `price_category` e.g. `"1;VLV100"` ≈ $0.99; flags `marketable`, `tradable`, `game_only`, `hidden`, `store_hidden`, `purchase_limit`.
- Trust model: without a trusted server, clients can only trigger drops from pre-defined generators ("you can't give users specific items in this scheme"); explicit grants need a privileged key on a server that "is the master of the state of the game"; purchases via `RequestPrices` / `GetItemPrice` / `StartPurchase` / `SteamInventoryResultReady_t` or web `https://store.steampowered.com/buyitem/:appid/:itemdefid/` [VERIFIED https://partner.steamgames.com/doc/features/inventory].

### 13.2 Implication [DESIGN]

Steam Inventory generators **cannot express pity counters, commit-reveal fairness proofs, per-jurisdiction gating, or minors' blocks** that the brief §8 requires. Therefore: PALIMPSEST's own backend is the source of truth for Inkwell, Sealed Folios and Codex; Steam is used only as payment rail (microtransactions) and optionally to mirror *already-rolled* cosmetics as `item` defs with **`tradable: false`, `marketable: false`** (no secondary-market cash value from paid randomness). Backend: `axum` 0.8.9 + `sqlx` 0.9.0 + Postgres, `tokio` 1.53.2 [CRATES]. Fairness: per-player `server_seed` committed as `SHA-256(server_seed)` before draws; draw `k` uses `HMAC-SHA256(server_seed, client_seed ‖ k)` → first 8 bytes as `u64` → unbiased integer by rejection sampling into the odds table's total weight; seed revealed at rotation (`sha2` 0.11.0, `hmac` 0.13.0) [CRATES; scheme RECALLED from provably-fair practice]. Offline end-of-service receipts signed with Ed25519 (`ed25519-dalek` 3.0.0) [CRATES].

---

## 14. Testing, replays, automated playtesting

| Tool | Version | Use |
|---|---|---|
| `cargo-nextest` | 0.9.148 | parallel test runner, per-test timeouts |
| `insta` | 1.49.0 | snapshot ("golden") tests of generated Folios (ASCII slices), item rolls, tooltips |
| `proptest` | 1.12.0 | property tests: formulas monotone, no overflow, odds sum to 10,000 |
| `arbitrary` + `cargo-fuzz` | 1.5.0 / 0.13.2 | fuzz save loading, net packet decoding, mod loading |
| `cargo-mutants` | 27.1.0 | mutation testing of formula code (proves tests bite) |
| `criterion` | 0.8.2 | benchmarks with budgets (mesher < 100 µs, Folio gen < 150 ms) |
| `tracing-tracy` / `tracy-client` | 0.12.0 / 0.19.0 | frame profiling |
| Bevy schedule randomization | Bevy 0.20 `ScheduleBuildSettings::shuffle_seed` (`debug` feature); log seed; single-threaded executor reaches more orderings | client-side order-bug detection [VERIFIED https://bevy.org/news/bevy-0-20/] |

Versions [CRATES]. [DESIGN] Test pyramid for an agent-written codebase:
1. **Formula unit tests** generated from the spec's worked examples (every formula in the canon has ≥ 3 worked examples → ≥ 3 assertions).
2. **Golden replays**: `(content_hash, run_seed, input_log)` → expected `xxh3` state hash at ticks 600, 3,600, 18,000. Any diff fails CI and prints the first diverging subsystem hash.
3. **Generator sweeps**: seeds 0–9,999 per Genre through validation constraints (§7.4).
4. **Bot playtests**: headless sim + scripted "Pilgrim" bots (utility-AI players: explore, fight, loot, descend) running 200 full runs per night; metrics: death Folio histogram, time per Folio, loot value per Folio, softlocks (no progress for 6,000 ticks) = failure.
5. **Cross-platform determinism job**: run the same 50 golden replays on x86_64 Linux, aarch64 Linux, x86_64 Windows, and wasm32 (wasmtime) and require identical hashes.
6. **Live-game agent checks**: launch client with `bevy_remote`; agent uses `bevy_brp_mcp` to screenshot and query entities after scripted inputs.

---

## 15. AI-agent-friendly codebase rules

Facts: **AGENTS.md** was donated by OpenAI (with Anthropic and Block co-founding) to the Linux Foundation's **Agentic AI Foundation** in December 2025; adopted by 60,000+ open-source projects and tools (Codex, Cursor, Devin, Gemini CLI, GitHub Copilot, VS Code); MCP is an inaugural AAIF project [VERIFIED-SEARCH https://openai.com/index/agentic-ai-foundation/]. UE 5.8 ships an experimental MCP server [VERIFIED gamefromscratch]; Bevy has `bevy_brp_mcp` [VERIFIED github natepiano].

[DESIGN] Rules:
1. **Workspace of small crates** with a strict dependency DAG: `palimpsest_math` (fixed-point, RNG, hashing) → `palimpsest_data` (content types + schemas) → `palimpsest_sim` (no Bevy) → `palimpsest_net` (transport trait, prediction) → `palimpsest_client` (Bevy) / `palimpsest_host` (headless) / `palimpsest_tools` (generators, validators, bots). Enforced with `cargo deny`-style bans [RECALLED tool].
2. **Files ≤ 400 lines, functions ≤ 60 lines**; one system or one formula family per file; the file name equals the spec section ID (`spec_05_shooting_damage.rs`).
3. **Newtypes for every ID and unit** (`ItemId(u32)`, `Ticks(u32)`, `Bp(u16)`, `Meters(Fx)`); no bare `u32` crossing a module boundary.
4. **Exhaustive enums**, no `_ =>` wildcard in sim match arms (`#![deny(clippy::wildcard_enum_match_arm)]` in `palimpsest_sim`) [RECALLED lint name].
5. **No `unwrap()` in sim**; errors are typed (`thiserror` 2.0.21) [CRATES].
6. **Spec traceability**: every public sim function has a doc comment `/// SPEC: 05 §3.2` and a test named after the spec's worked example.
7. **AGENTS.md at root and per crate**: build/test commands, invariants, forbidden APIs (`HashMap` iteration, `f32` in sim, `std::time`, `rand::thread_rng`).
8. **Deterministic, fast tests**: whole `palimpsest_sim` test suite under 60 s on CI so the agent runs it every edit.
9. **Pin everything**: `rust-toolchain.toml` = `1.99.0`, `Cargo.lock` committed, `=` version pins for Bevy-family crates.

---

## 16. Recommended stack with exact versions

### 16.1 Primary matrix (Bevy 0.20 line) — versions read from crates.io on 2026-10-11

| Crate / tool | Pin | Status vs Bevy 0.20 today | Notes |
|---|---|---|---|
| Rust toolchain | `1.99.0` | ≥ Bevy MSRV 1.97.1, ≥ `fixed` MSRV 1.93.0 | `rust-toolchain.toml` |
| `bevy` | `=0.20.0` | — | features: `3d`, `ui`, `bevy_remote` (dev), `hotpatching` (dev), `dlss` (opt-in), `debug` (CI only, for `shuffle_seed`) |
| `bevy_egui` | `0.43.1` | ready | debug overlays only; game UI in `bevy_ui` |
| `bevy_brp_mcp` / `bevy_brp_extras` | `0.23.0` | ready | agent control of the running client |
| `bevy_rapier3d` | `0.37.0` | ready | cosmetic physics if Avian 0.8 is late |
| `bevy_kira_audio` | `0.27.0` | ready | fallback audio if `bevy_seedling` is late |
| `avian3d` | next release after 0.7.0 | **pending** | preferred cosmetic physics (ECS-native) |
| `bevy_seedling` + `firewheel` | next after 0.8.0 / 0.14.0 | **pending** | preferred audio graph |
| `bevy_enhanced_input` | next after 0.26.0 | **pending** | input actions/contexts |
| `bevy_hanabi` | next after 0.19.0 | **pending** | GPU particles |
| `audionimbus` | `0.16.0` (`auto-install`, `bevy`) | check `bevy` feature's Bevy version at integration | Steam Audio 4.8.1 |
| `steamworks` | `0.13.1` | Bevy-agnostic | SDR P2P, overlay, inventory, microtransactions; write own thin Bevy plugin instead of waiting for `bevy-steamworks` |
| `renet` | `2.0.0` | Bevy-agnostic | dev/LAN transport |
| `crossbeam-channel` | `0.5.17` | — | in-process transport for tests |
| `fixed` | `1.31.0` | — | sim scalars |
| `glam` | via Bevy (`^0.33.2`) | — | **client only**, never in sim |
| `indexmap` | `2.14.2` | — | ordered maps in sim |
| `smallvec` | `1.16.3` | — | — |
| `slotmap` | `1.1.1` | — | generational arenas for sim entities |
| `xxhash-rust` | `0.8.19` | — | state hashes |
| `blake3` | `1.8.7` | — | content addressing, cache keys |
| `serde` / `serde_json` | `1.0.229` / `1.0.154` | — | — |
| `ron` | `0.12.2` | — | authored content |
| `schemars` / `jsonschema` | `1.2.2` / `0.58.6` | — | schema generation + CI validation |
| `postcard` | `1.1.3` | — | saves, net, chunk blobs |
| `rkyv` | `0.8.18` | — | baked tables |
| `thiserror` | `2.0.21` | — | typed errors |
| `tracing` / `tracing-tracy` | `0.1.44` / `0.12.0` | — | logs + Tracy |
| `rayon` | `1.12.0` | — | ordered parallel generation |
| `pathfinding` | `4.16.0` | — | A*/BFS primitives |
| `binary-greedy-meshing` | `0.5.2` | — | mesher (or a 300-line in-house port) |
| `dot_vox` | `5.2.0` | — | MagicaVoxel import for prefab rooms/props |
| `wasmtime` / `wit-bindgen` | `49.0.2` / `0.62.0` | — | mod host |
| `llama-cpp-2` | `0.1.159` | — | Scribe |
| `insta` / `proptest` / `arbitrary` | `1.49.0` / `1.12.0` / `1.5.0` | — | tests |
| `cargo-nextest` / `cargo-mutants` / `cargo-fuzz` | `0.9.148` / `27.1.0` / `0.13.2` | — | CI tools |
| Backend: `axum` / `sqlx` / `tokio` | `0.8.9` / `0.9.0` / `1.53.2` | — | economy service |
| Crypto: `sha2` / `hmac` / `ed25519-dalek` | `0.11.0` / `0.13.0` / `3.0.0` | — | commit-reveal, receipts |

### 16.2 Fallback matrix (Bevy 0.19.1 line), if implementation starts before the 0.20 ports land

`bevy =0.19.1`, `avian3d 0.7.0`, `bevy_seedling 0.8.0` (Firewheel ^0.12), `bevy_enhanced_input 0.26.0`, `bevy_hanabi 0.19.0`, `bevy-inspector-egui 0.37.0`, `bevy-steamworks 0.17.0` (steamworks ^0.13.1), `lightyear 0.30.1` (reference only), `bevy_replicon 0.44.3` [CRATES]. Because the sim, net, data and tools crates do not depend on Bevy, a 0.19 → 0.20 migration touches only `palimpsest_client` [DESIGN].

### 16.3 Justification summary

1. **Bevy over Godot/Unreal/Unity**: the brief fixes Rust + Bevy; the research confirms it is the best agent substrate (all text, strong types, headless, BRP + MCP), and its weaknesses (no editor, breaking releases, no consoles) are mitigated by the Bevy-free sim and by voxel content being generated rather than hand-placed.
2. **0.20 over 0.19**: three agent-relevant features (schedule randomization, panic-to-error, settled BSN syntax) and mesh-shader groundwork; ecosystem gap closes in weeks based on the 0.19 port latencies (0–51 days).
3. **Own netcode over lightyear/replicon/GGRS**: state lives outside Bevy ECS; rollback of a whole voxel world is too costly; Steam SDR solves NAT/IP privacy at zero server cost for a listen-host model.
4. **Fixed-point sim, integer stats**: makes the spec exact and replays/daily-challenge verification bit-identical across x86_64, aarch64 (Steam Frame, future ARM PCs) and wasm.
5. **Steam Audio + own sound grid**: open-source best-in-class presentation, deterministic stealth truth.
6. **Optional on-device SLM**: Apache-2.0 0.8B–2B models are good enough for barks and lore when constrained by schema; template fallback keeps the game whole without it.

---

## 17. Design implications for PALIMPSEST

**Steal exactly**

1. **Steal Bevy 0.20 schedule randomization** for the client: CI runs the client's headless smoke test with 16 random `shuffle_seed`s and fails on any assertion or state divergence; log the seed [VERIFIED feature].
2. **Steal the cgerikj binary greedy mesher** bit layout idea (occupancy `u64` columns, per-face masks, 8-byte quads) and its budget: a chunk mesh must cost < 100 µs on Deck; enforce with a criterion benchmark gate.
3. **Steal Unexplored's lock/key representation**: keys are typed non-terminals {item, switch, knowledge} with an explicit key→lock edge carried through every graph rewrite, and reuse that graph for an in-game hint system ("Consult the Marginalia" = Pray For Help).
4. **Steal Merrell's "modifying in blocks"** for WFC room fill: re-solve only the failing block, 3 retries, then template fallback. Never restart a whole Folio.
5. **Steal Steam Inventory's grammar** for in-run Reliquary/Fortune Engine loot tables: `"501x90;502x9;503x1"` weighted-generator strings and `"rarity:common*5"` exchange recipes are compact, agent-friendly, and already familiar to players and tools. Use them as the canonical text form of every weighted table, parsed into integer weights.
6. **Steal lightyear's input redundancy and priority accumulation** (inputs resent for 8 ticks; snapshot entities ordered by accumulated priority until the per-packet byte budget is spent).
7. **Steal GGRS's SyncTest idea** at the sim level: a CI mode that every tick snapshots, restores, re-simulates N = 8 ticks, and asserts identical hashes. This proves prediction/rollback correctness before netcode exists.
8. **Steal Barony's topology**: listen host, Steam lobby + P2P, EOS build for Epic crossplay, no dedicated servers required (but keep the headless host binary for daily-challenge verification).

**Improve**

9. **Light is one system.** The sim's flood-fill light grid (required for stealth) is uploaded to the GPU as a per-chunk 3D texture and is the baseline GI. Visual light can never disagree with stealth light, which fixes the classic "I'm in the dark but the guard sees me" complaint. Solari path tracing is an Ultra-tier visual layer that must not change what the sim considers lit.
10. **Sound is two systems** (sim grid vs Steam Audio), with a debug overlay that draws the sim's sound field so players and testers can learn stealth.
11. **Genre-skin texture arrays**: each Genre ships a `texture_2d_array` of block layers at 32×32 px (pixel-art crispness, tiny memory) with the 16-bit layer index in the packed quad. Genre bleed is per-quad layer substitution decided by the sim's genre field, so no shader branching per genre.
12. **Scribe text is host-generated and replicated**, so co-op players read identical lore, and it is cached by seed so a shared daily seed reads identically for everyone who has the same model installed. Template-grammar fallback produces the same JSON shape.
13. **Fairness proofs are a feature**: show players the committed hash of their current Inkwell server seed in the UI, and reveal it on rotation with a "verify" button that recomputes their draws locally.
14. **Golden replays as the spec's acceptance tests**: every spec document's acceptance tests should include at least one `(seed, inputs) → hash` replay once implemented, so regressions are caught in milliseconds.

**Avoid**

15. **Avoid hardware ray tracing, frame generation and ML upscalers in the baseline.** Steam Deck and Steam Machine are RDNA2/RDNA3 and get no AMD ML upscaler; frame generation adds latency to a 60 Hz aim-driven game. DLSS/Solari are optional extras on RTX.
16. **Avoid Nanite-style virtual geometry, Gaussian splats, NTC and work graphs**: all solve problems a 1 m block world does not have, and several are preview-only.
17. **Avoid running a physics engine in the sim**, and avoid `glam`/`f32` types in `palimpsest_sim` entirely (enforce with a `clippy.toml` `disallowed-types` list [RECALLED lint]).
18. **Avoid whole-world rollback (GGRS) and component-replication frameworks** as the core netcode; they fight the Bevy-free deterministic sim.
19. **Avoid Steam-marketable or tradable items from any paid random source**, and avoid Steam Inventory generators for paid draws (no pity, no proofs, no jurisdiction gates).
20. **Avoid bincode** (unmaintained, final release is a compile error), stale AI crates (`big-brain` last released 2024-11), and any dependency whose latest release targets a Bevy older than the pinned one.
21. **Avoid LLM calls on the frame's critical path and LLM outputs that reference game IDs freely**: IDs only via enum-constrained schema fields, validated, else template fallback.
22. **Avoid Lua alongside WASM**: one mod sandbox (WASM components with fuel), one ABI (WIT), plus data packs.
23. **Avoid consoles before PC 1.0.** Switch 2 docked (3.07 TFLOPS, 9 GB for games) is in reach of a Deck-optimized build, but Bevy has no public console path; plan it as a funded port with an NDA'd platform layer behind the same `Platform` trait used for Steam and EOS.

**Open technical questions to resolve during implementation** (also returned in the summary): actual llama.cpp tok/s on Steam Deck under game load; Steam SDR pricing terms for P2P; Avian 0.8 / bevy_seedling for Bevy 0.20 release dates; Steam Machine availability after its June 30 launch; WASI 0.3 Rust guest toolchain readiness on stable (nightly linker needed per migration notes as of mid-2026 [VERIFIED-SEARCH https://component-model.bytecodealliance.org/design/migrating-to-p3.html]).
