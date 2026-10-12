# Fix log: doc 02 (Technical Architecture)

Chapter fixer for `spec/02_TECH_ARCHITECTURE.md`, 2026-10-11. Authorities: DECISIONS.md rows 1 to 104 and the orchestrator rows, canon-1.11 (every "canon amendment" row of `audit/canon_gaps_triage.md` treated as in force), then the triage's other rows and §4 conflicts. Reference implementation: the Rust workspace at `palimpsest/` (crates `palimpsest_math`, `palimpsest_data`, `palimpsest_sim`).

Edits are made by scripts in the session scratchpad (`fix02/`) that assert each anchor occurs exactly once; the pre-edit chapter is saved there as `fix02/02_orig.md`.

## Plan (batches)

1. Header, intro, Imports.
2. §2 stack: toolchain targets (DECISIONS 98: Windows-only client), crates (`tts`, EOS binding, serialization roles), Bevy features.
3. §3 workspace: layout paths, crate and module lists matching the workspace and doc 21 §4.1, dependency rules (serde feature), API sketches, `pal` subcommands.
4. §4 math: actions_02 items 1 to 8.
5. §5 sim: chunk clone sizes, button bits (C-10, C-30), intents (C-15 and every owner's requests).
6. §6 determinism: hand-written canonical encoding (Canon §2.5), matrix, goldens.
7. §7 bridge: view data requested by docs 03 and 18, micro-voxel masks.
8. §8 rendering: micro-voxel meshing (Canon §4.4), 64 × 64 layers (C-12), FOV (C-11), light overrides.
9. §9 to §11: audio, input and UI crates, content hash and `presentation_hash` (Canon §2.4), asset example ID, curve generator input.
10. §12 to §14: saves, mods, platforms (C-13, DECISIONS 73, 98).
11. §15 to §18: budgets, tests, telemetry (C-14), conventions.
12. Exports, Interface assumptions, Canon gaps, Acceptance tests, Invariants.

## Done

(appended after each batch)

### Batch 1: header, intro, Imports (applied)

- Header now "Conforms to canon-1.11" (triage G-051); intro lists the canon-1.11 rules checked and names the Rust workspace as the reference implementation.
- Imports: new DECISIONS row (5, 6, 24, 37, 43, 55, 65, 73, 87, 92, 98); canon rows updated for the amended §1.1 mod IDs, §1.6 integer bound, §1.9 `atan2_bam` convention, §2.2 boundaries, §2.4 intents and `presentation_hash`, §2.5 hand-written serialization and invasion save deferral, §3 sub-streams and §3.8, §4.3 and §4.4 micro-voxels, §17.7 mounts, tests 45 to 47; new triage row; doc 03 row now cites `heard` with 5 slots, the fourth bitset, `fluid_gen` and light overrides (doc 03 on disk); the stale "Docs 04, 07, 17–19 not yet written" row replaced by rows for the intent owners and docs 01, 18, 19, 21.

### Batch 2: §2 stack (applied)

- §2.1: Rust 1.99.0 noted as installing cleanly through `rust-toolchain.toml` (actions_02 item 9); components match the workspace's `rust-toolchain.toml` (`rustfmt`, `clippy`), coverage adds `llvm-tools`; targets rewritten for DECISIONS 98 (Windows x86_64 is the only client; Linux targets build headless binaries; macOS is determinism CI only; no browser build); Linux baseline is the CI image for headless binaries.
- §2.2: `cpal` WASAPI only; EOS binding row behind feature `eos` (DECISIONS 73, doc 21 Interface row); `tts` 0.26.3 for `access.narrator` and AccessKit through `bevy_winit` (doc 17 Interface row; version read from crates.io on 2026-10-11); serde/postcard roles restated so the simulation serialization is hand-written (actions_02 item 3, Canon §2.5).
- §2.3: new override row for R4 §12's postcard saves; the `rkyv` row no longer calls postcard the single format.
- §2.5: `x11`, `wayland`, `webgpu` and the Linux Ultra build removed (DECISIONS 98).

### Batch 3: §3 workspace (applied)

- §3.1 layout matches the workspace: `crates/<package>/` (for example `crates/palimpsest_math/`), toolchain components, README, `tools/gen_trig.py` writing `crates/palimpsest_math/src/trig_table.rs` and `cordic_table.rs` (actions_02 item 7), `tools/gen_namespaces.py`, `tools/validate_content.py`, `tests/moments/`.
- §3.2: crate responsibilities updated (data hashes, platform `steam`/`eos`/`meta`, ui screen reader, tools subcommands); new **Modules** table listing the workspace's real modules for `palimpsest_math` and `palimpsest_data` and doc 21 §4.1's module paths for `palimpsest_sim`, with the slice-module moves (doc 21 Interface row).
- §3.3: `serde` only behind an off-by-default `serde` feature for math (tools only) and sim (types that cross the network); `postcard` removed from and forbidden in sim; net may use sim's `serde` feature; check-deps semantics made exact (default features, `feature_external`, direct allow lists); JSON record updated (actions_02 item 3).
- §3.4: `isqrt_u64` (doc 05 Interface row), `yaw_from_dir`, `pitch_from_dir`, `blake3_fields`, `presentation_hash`, `palimpsest_platform::meta` (doc 21); `pal` gains `extract`, `spec-sync`, `catalog` (doc 21), `odds` (doc 15), `scenario` (doc 01).

### Batch 4: §4 math (applied; actions_02 items 1 to 8)

- §4.1: `TWO_POW_32` constant; `Fx` derives `Hash` and gets serde derives only behind the `serde` feature; `ONE`, `HALF`, `MAX` as literals; new **Shifts** bullet: every `x << 32` in §4 code is a checked multiply (item 2, Canon §1.3), with xoshiro's `s[1] << 17` named as the one wrapping shift (new canon gap G1).
- §4.2: `2u128.pow(..)` start value; `sqrt_fx` with `checked_mul(TWO_POW_32)` and checked narrowing; `isqrt_u64` added (doc 05 Interface row).
- §4.3: trig table path `crates/palimpsest_math/src/trig_table.rs` (item 7); Q2.30 conversion as `* 4`.
- §4.4: convention now cited to Canon §1.9 (G-007); `ATAN_T` path `cordic_table.rs`; normalization and `z` without left shifts; final wrap with `rem_euclid`.
- §4.6: `div128` with checked multiply; `I256` derives `Ord` (`a.cmp(&b)`), no inherent `cmp`, gains `from_i128` and `sar` (item 6); `closest_seg_seg` rewritten clamp-before-narrow with `ratio01` (item 1, G-002) plus Examples 3 and 4, the two panicking inputs from `geom.rs` test `closest_seg_seg_where_doc_code_panics`; collision against 250 mm micro-voxel boxes (Canon §4.4); tangent rays (`h == 0`) hit in `ray_sphere` and the body path, with the 1.8 m grazing example and a sphere example, both verified by integer emulation (item 5, G-005); voxel DDA is the `VoxelDda` iterator taking `max_steps`, yielding at most `max_steps + 1` blocks, stopping at the `i32` edge, with doc 03's `max_steps = 3 × ceil(t_max) + 3` (item 4, G-004), and `dda` gains `max_steps`.
- §4.7: `Rng::state()` / `Rng::from_state()` as the save interface; `serde` feature off by default, tools only (item 3, G-003); debug draw counters live in `palimpsest_sim::api` and never enter state, saves or hashes (item 8, Canon §2.5); sub-streams, object-keyed draws, offer and presentation generators listed; `blake3_fields`.
- Code note for the slice (task #8, not this chapter's file): `geom.rs` `ray_sphere` still returns `None` for `h == 0`, and `dda` lacks the `max_steps` parameter.

### Batch 5: §5 simulation (applied)

- §5.2: copy-on-write clone is at most 128 KiB (doc 03's fourth bitset `flip`, doc 03 Interface row) plus the micro-voxel mask table (Canon §4.4).
- §5.3: `SCHEDULE` names the 39 systems of doc 03 §2.2 (doc 03 Interface row).
- §5.4 and §5.5: session intents apply first; `extract_view` documented as a read-only pass whose `&mut` touches only view bookkeeping (doc 03 Interface row).
- Button table: bit 25 `MOUNT_TOGGLE` retired and always 0 (C-30, Canon §17.7); bits 26–28 `ACTION` and 29–31 `WHEEL_ENTRY`, bits 19+20 `WHEEL_SELECT`, "reserved" dropped (C-10); the validator and doc 16's sanitizer reject bit 25 or `ACTION` 6 and 7.
- Intents: "Canon gaps G4" replaced by Canon §2.4 (G-016); full `IntentKind` table by owner, adding `ShopHaggle { shop }` and doc 17's kinds (C-15), `DropLucre { amount: i64 }` (doc 11), `Hack`, `Steal`, `Plant` (doc 09), `SetLockMode` (doc 17), `Pray { patron, kind }` (doc 06), `Craft { recipe, count }` (doc 07), `MinionCommand` payload (doc 17/13), `ChooseExit { door }` (doc 14), doc 16's trade, `DropItemFor`, `MarkTarget`, `Session`.

### Batch 6: §6 determinism (applied)

- §6.1: `serde-scan` defined against `Canonical` impls; new row forbidding serde and postcard in the canonical serialization (Canon §2.5).
- §6.2: `#![cfg_attr(not(test), no_std)]`, matching the workspace's `lib.rs` (tests link std).
- §6.4: Deck row is the Windows build under Proton; Linux and macOS jobs stated as determinism checks only (DECISIONS 98).
- §6.5: postcard sections replaced by the hand-written self-delimiting encoding (little-endian fixed-width integers, 0/1 bool and option tags, `u32le` counts, ascending `BTreeMap` keys, `u8` enum indices, LEB128 only inside `palette_rle`), which is what `palimpsest_sim/src/sim.rs` `Canonical` already does (actions_02 item 3, Canon §2.5); section 1 uses `Rng::state()`; section 4 lists the four bitsets and the micro-voxel mask list (Canon §4.4).
- §6.6: golden bless also rewrites `presentation_hash` (Canon §2.4).

### Batch 7: §7 bridge (applied)

- `ViewFrame.light_overrides` (doc 03 §10.6; doc 03 Interface row; Canon §2.3 light-override amendment G-020).
- `ViewPlayer.revealed` and `dark_vision_mm` (doc 18 Interface row: doc 09 §4.8 reveal state, Canon §6.4 row 31).
- `ChunkView.masks`: micro-voxel masks that differ from the shape's predefined mask (Canon §4.4); a mask change bumps `block_gen`.
- `FolioView.seams` plus seam bits 3–5 in `slot_of_cell` (doc 18 Interface row: seam cells and each seam's second genre).

### Batch 8: §8 rendering (applied)

- §8.1 and §8.2: meshing rewritten for 250 mm micro-voxels (Canon §4.4, DECISIONS 37): 64 sections of 32³ micro-voxels per chunk, 34³ padded copies, masks from `ChunkView.masks` or the shape's predefined mask, prop shapes unmeshed; detail quads deleted; new packed-quad layout (x, y, z 7 bits; face moved to word 1) with a recomputed example (`0x1DEC0F8C`, `0x0086F205`); AO on micro-voxel neighbours; the AO triangle example corrected (the old example (2, 3, 3, 2) does not flip, since 5 is not below 5; the flip example is now (2, 3, 3, 3)); culling planes at 0.25 m and 31.75 m; remesh per section, `perf.deck.mesh.chunk_us` (100 µs per chunk) replaced by `perf.deck.mesh.section_us` (25 µs per section; no other chapter cites the old ID); render-only debris (Canon §4.4).
- §8.3: 64 × 64 px layers, full mip chains, 5,488 bytes per layer, world-space triplanar at 64 texels per meter (C-12, G-217, doc 18 Interface row); VRAM check 7,024,640 bytes for five arrays.
- §8.4: FOV setting 60–120°, default 90° (C-11, DECISIONS 92), with the vertical FOV at both ends.
- §8.5: the light field is clamped by `ViewFrame.light_overrides` before upload (doc 03 §10.6, Canon §2.3); the face-centre claim limited to full-cell faces.
- §8.7: DLSS no longer mentions a Linux build (DECISIONS 98).

### Batch 9: §9 to §11 (applied)

- §9: doc 18 §6.6 pools named as the concrete form of the voice-stealing rule (doc 18 Interface row); `ev.noise` heard for slots `0..=4` (doc 03); presentation occlusion counts cells with a mask of at least 48 bits (Canon §4.4).
- §10: AccessKit nodes and `access.narrator` through `tts` (doc 17 Interface row).
- §11.1: semantic checks add the `x_<mod_id>` rule (Canon §1.1, G-008), retired IDs (Canon §17.7), owner rules such as doc 07's `recipe_overlap` (doc 07 Interface row); `pal validate` follows doc 19 §4 and matches `tools/validate_content.py` (doc 19 Interface row).
- §11.2: `content_hash` now excludes the presentation namespaces and counts only simulation-affecting mods; new `presentation_hash` layout over presentation records, `assets/loc/` files and presentation-only mods (Canon §2.4 amendment G-017, DECISIONS 87); the existing 141-byte example still holds (both records are `crate.`); new examples verified with Python `blake3` (47-byte empty input `596015…559b`; one localization file, 95 bytes, `08ea33…72d1`).
- §11.4: undefined `art.mesh.enemy.wolf_dire` replaced by doc 18's `art.mesh.enemy.gothic_horror.ghoul`; `art.icon.` `.ktx2` row (doc 18 Interface row).
- §11.5: the generator input uses `"curve"` instead of `"id"`, so this chapter no longer declares doc 13's `ai.curve.*` IDs (lint multiply_defined: `ai.curve.logistic_up`, `bell_mid`, `plateau`).

### Batch 10: §12 to §14 (applied)

- §12.1: a replay with a different `presentation_hash` loads with a warning (Canon §2.4).
- §12.2: reported-run replays kept 30 days by `palimpsest_services` (doc 15 Interface row).
- §12.3: "Canon gaps G11" replaced by Canon §2.5 (G-019).
- §12.4: meta-progression rules in `palimpsest_platform::meta`, headless (doc 21 Interface row).
- §13.1: mod file paths per doc 19; "Canon gaps G5" replaced by Canon §1.1 (G-008); the patch example now appends a doc 11 §9.5 line to `loot.chest.plain` through `/lines` with ID `loot.x_frostpack.extra_chest_line` (notes_items follow-up and doc 11 Interface row; the old `/entries` path and `item`/`w` value did not exist in doc 11); retired IDs rejected in mods; presentation-only mods enter `presentation_hash` (DECISIONS 87).
- §13.2: the console sentence replaced: scripted mods load in every client build, including Steam Deck under Proton (DECISIONS 98).
- §13.4: `modset_hash` covers the simulation-affecting `MOD` section; presentation-only mods may differ; invasions need an empty mod set (doc 16).
- §14: invaders find sessions through doc 16's `svc.lobby` beacon search (C-13, G-220); Epic Games Store bullet (DECISIONS 73); Deck checklist names the Windows build under Proton; desktop is Windows only, Vulkan backend on Deck; console and WebGPU bullets replaced by one "No other platform" bullet (DECISIONS 98).

### Batch 11: §15 to §18 (applied)

- §15: `sys.crafting`, `sys.resources`, `sys.coop`, `sys.run_layer` placed inside the `input_actions` and `bookkeeping` groups with unchanged budgets, so totals (3.97, 12.40, 4.33, 4.87, 14.50 ms, re-verified by script) stand and all 39 systems are covered (doc 03 Interface row); caps updated for debris and section remeshes.
- §16: three bot policies for `pillar.living_page.t3` and a signature-moment scenario runner (doc 01 Interface row).
- §17.1: PR pipeline runs scenarios and the generator checks; Windows client and Linux headless builds (DECISIONS 98).
- §17.3: `eos` Cargo feature; doc 15's `strict` field on `juris.` records (doc 15 Interface row).
- §17.4: every minor-flagged account is barred (C-14, G-222); per-Folio run-summary fields (C-14, G-199).
- §18 item 2: `SPEC:` regex accepts `canon` (resolves the open question in the workspace's AGENTS.md; doc 21 WP-003). Item 8: milestone labels M0–M15 removed; doc 21 §3.5 and its milestones govern (doc 21 Interface row).

### Batch 12 and 13: closing sections (applied)

- Exports: canon-1.11 in the collision sentence; `perf.deck.mesh.chunk_us` replaced by `perf.deck.mesh.section_us` (count stays 58); `flag.platform.steam` false in the EOS build; Non-ID interfaces list the new view fields, intents, encoding, `presentation_hash`, `VoxelDda`, `TWO_POW_32`, `isqrt_u64`, module paths.
- Interface assumptions rewritten: rows for docs 01, 06, 10, 11, 21 added; doc 03 row drops the two requests doc 03 has met (`heard` with 5 slots, `fluid_gen`) and adds micro-voxel masks and the DDA `max_steps`; every intent owner validates its variants; doc 16 row carries C-10, C-13, C-14; doc 17 row carries the `ShopHaggle` rename (C-15); doc 18 row carries the example mesh ID, 64 × 64 layers, voice pools, reveal and seams.
- Canon gaps: G1 to G13 deleted as closed (triage G-007, G-008, G-010, G-012 to G-017, G-019 adopted or already in canon; G-009 chapter-owned), with a one-paragraph closure note; one real open item kept as new G14 (Canon §1.3 checked shifts vs Canon §3.1's wrapping `s[1] << 17`).
- Acceptance tests: 4 (trig table path), 9 (Examples 3 and 4, no panic), 10 (tangent hits), 11 (`max_steps`, `i32` edge), 12 (new quad words; the AO triangle claim corrected), 14 (64 × 64 layers, light override clamp), 17 (`presentation_hash` vectors), 21 (`postcard`, `serde` closure), 33 (25 µs per section), 36 (derived `Ord`), 40, 41, 45 updated; new tests 48 (intent order), 49 (bits 25 to 31), 50 (canonical encoding bytes), 51 (draw counters outside state), 52 (micro-voxel remesh and debris), 53 (seams and reveal).
- Invariants: content/presentation hash split, clamp-before-narrow, tangent hit, hand-written serialization, bounded DDA, bit 25, micro-voxel rendering, light override clamp; the console line replaced by the Windows-only client (DECISIONS 98).
- Lint after: `== 02 words=27698` with no findings; `multiply_defined` empty.

### Batch 14 to 24: consistency pass (applied)

- Imports: the Canon §17.7 row records the retirement only (C-30).
- §3.3: `palimpsest_data` takes `serde` only through `loader` (and `schemars` through `schema`) and forbids `postcard`, so `palimpsest_sim`'s default closure really holds no `serde` (needed by tests 21 and 50); math's `serde` feature is enabled by sim's `serde` feature and by tools (sim's message types hold `Fx3`); tools may depend on econ for `pal odds` (doc 15 Interface row).
- §4.6: the no-panic claim limited to the stated coordinate range; §9: the audio DDA passes `max_steps`.
- §8.2: Folio-load meshing stated as 204,800 µs of work, at most 68,267 µs on 3 workers (exact integers).
- §2.2: `opus` 0.4.0 (libopus through `opusic-sys`) for voice encoding, because doc 16 §9.12 now sends Opus voice in every build (doc 16 Interface row; version read from crates.io, released 2026-08-23).
- Test 14 wording made exact; test 35 restated for simulation-affecting mods (doc 16 Interface row, DECISIONS 87); §17.4 cites doc 16 §11.7.
- Invariant wording: `serde | postcard ∈ default closure(sim)`.

## Status

All planned batches are applied. Lint after: `python3 tools/spec_lint.py --chapter 02` prints `== 02 words=27847` with no findings; the `--json` `multiply_defined` map is empty. `perf.deck.mesh.section_us` has no `fnv1a32` collision with any ID token in the spec.

## Requests for other chapters

- **00 (canon):** G14. Canon §1.3's shifts bullet should add "except the bit-mixing shift `s[1] << 17` of §3.1, which wraps", since that shift must drop bits to reproduce Canon §21 test 1.
- **03:** for Canon §4.4 rendering and collision: a predefined `u64` micro-voxel mask for every shape and state in the per-state tables; a `mask_at(c) -> u64` query; each chunk's list of masks that differ from the shape's mask (custom and chipped), serialized in section 4 directly after the four cell bitsets as ascending `(Local: u16, mask: u64)` pairs, and listed in §16.1's row 4; a `block_gen` bump on every mask change; `raycast_blocks` passing `max_steps = 3 × ceil_div(t_max.raw(), 2^32) + 3` to `dda`. Doc 03's Interface row for doc 02 is otherwise met (39 systems budgeted, `fluid_gen`, the `flip` bitset in clone sizes, light overrides in the view frame, `extract_view` read-only, slot 4 noise) and can be marked so.
- **11 (with 19):** mark the `lines` array of `loot.` table records `"x-patchable": true`, which doc 02 §13.1's patch example appends to.
- **21:** doc 02 now has 53 acceptance tests (48 intent order, 49 command bits 25 to 31, 50 canonical encoding bytes, 51 draw counters outside state, 52 micro-voxel remesh and debris, 53 seams and reveal); update the count row "doc 02 | 47 | 47" and assign 48 to 51 to the store/Sim API/serialization WPs (WP-009 to WP-012), 52 to the renderer WP (WP-050) and 53 to the bridge WP (WP-049). Close G16: doc 02 §2.1 and §14 now follow DECISIONS 98. WP-012's macOS job stays a determinism-only target.
- **01:** §9.2 still lists Linux, macOS, a WebGPU demo and console ports; DECISIONS 98 makes the client Windows-only through 1.0 (Steam and Epic, Steam Deck through Proton). Q-11 (macOS timing) is moot.
- **17:** §13.6's "(doc 02 picks the crate)" can name `tts` 0.26.3 (doc 02 §2.2).
- **Rust slice (task #8, not a chapter):** `crates/palimpsest_math/src/geom.rs` `ray_sphere` must return a hit when `h == 0` (it returns `None`), and `dda` must take `max_steps` like `VoxelDda::new`.

## Verification

Adversarial verifier, 2026-10-11. Edits were made by scripts in the session scratchpad (`verify02/edit1.py` to `edit6.py`) that assert each anchor occurs exactly once; the chapter as the fixer left it is saved as `verify02/02_before_verify.md`. DECISIONS.md gained rows 109 to 125 after the fixer finished (file times 23:16 and 23:21 against the fixer's 22:52); rows 112, 113, 120 and 123 bear on this chapter and are applied below.

### Check 1: triage rows that name chapter 02

| Row | Verdict | Chapter text that satisfies it |
|---|---|---|
| G-002 | pass | §4.6 "**Clamp before narrow:** every ratio is clamped to `[0, 1]` in 256-bit arithmetic before it becomes an `Fx`"; `ratio01`; Examples 3 and 4 are the inputs of `closest_seg_seg_where_doc_code_panics` in `crates/palimpsest_math/src/geom.rs` (checked against the test: `(6, raw 2^32 + 1, 0)` and `raw (-5 << 32) + 1`) |
| G-003 | **fixed** | §3.3 and §4.7 were right ("Its optional feature `serde` is off by default"; "`Rng::state()` and `Rng::from_state()` are the save interface"), but §5.5's `PlayerCommand` still derived `Serialize, Deserialize` unconditionally inside `palimpsest_sim`. Now `#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]`. §4.7's derive list now matches the workspace (adds `Capsule`, `VCapsule`). |
| G-004 | pass | §4.6 "The traversal takes `max_steps` ... yields at most `max_steps + 1` blocks ... when the next block coordinate would leave the `i32` range"; "Doc 03 callers pass `max_steps = 3 × ceil_div(t_max.raw(), 2^32) + 3`" |
| G-005 | pass | §4.6 "A tangent ray (`h == 0`) is a hit in the body path and in `ray_sphere` alike."; `if h < 0 { return None; } // h == 0: tangent hit` |
| G-006 | pass | §4.6 `#[derive(..., PartialOrd, Ord, ...)] pub struct I256` and "there is no inherent `cmp` method"; §4.3 "`crates/palimpsest_math/src/trig_table.rs`" |
| G-007 | pass | §4.4 "Canon §1.9 fixes this convention and `yaw_from_dir`, and this section fixes the algorithm." |
| G-008 | pass | §11.1 "every mod record inside its own `x_<mod_id>` segment (Canon §1.1)"; §13.1 "`item.x_frostpack.ice_blade` (Canon §1.1)", the canon's own §1.1 example |
| G-009 | pass | §13.1 "A mod changes base content only through `patches/*.json` records ... allowed only on array fields that the base schema marks `"x-patchable": true`" |
| G-010 | pass | §11.1 "any integer outside `−(2^53 − 1) ..= 2^53 − 1`" |
| G-012, G-013, G-014 | pass | the old G8 to G10 and G13 are gone from Canon gaps |
| G-015 | pass | §5.4 "A state hash always describes a boundary: the state after simulating tick `b − 1`, before simulating tick `b`." |
| G-016 | pass | §5.5 "Canon §2.4 logs every applied intent `{ tick, slot, action_seq, kind }` ... this chapter owns the list of `IntentKind` variants" |
| G-017 | pass | §11.2 `content_hash` and `presentation_hash` layouts; vectors recomputed (Check 6) |
| G-018 | pass | §14 lobbies carry "`sim_version`, `content_hash`, `modset_hash`, mode and the `mode.invasion_open` flag", no `run_seed` |
| G-019 | pass | §12.3 "While slot 4 holds an invader, a single-player Suspend request is deferred and completes on the tick `ev.invasion_ended` is emitted (Canon §2.5)." |
| G-020 | **fixed** | The renderer clamp covered override boxes only. Doc 03 §10.6 on disk now adds an optional `sphere` and the dimming by 8 inside doc 06's shadow areas, and its Interface row asks doc 02 to publish both. Added `ViewFrame.shadow_cells`, a §7.2 bullet, the full read `min(max(0, stored − dim), m)` in §8.5, two clauses in test 14, Imports, Exports, Interface rows for 03 and 06, and the invariant. |
| G-046 | pass | §6.5 rows 6 and 7: sparse `(Local, LiquidCell)` and `(Local, GasCell)` lists; `palette_rle` of the 4,096 thermal offsets |
| G-051, C-30, C-31 | pass | header "Conforms to canon-1.11"; bit 25 "none: always 0, because mounts are cut"; "no item durability" |
| G-199, C-14, G-222 | pass | §17.4 per-Folio fields and "A minor-flagged account ... is never asked and sends nothing" |
| G-217, C-12 | pass | §8.3 "64 × 64 px, BC7 in KTX2 with full mip chains (7 levels ...; 5,488 bytes per layer ...)", triplanar at 64 texels per meter |
| G-220, C-13 | pass | §14 "invaders find open sessions through doc 16's `svc.lobby` beacon search" |
| G-227 | pass | §3.2 `crate.econ` = `palimpsest_econ` |
| C-10 | pass | §5.5 table: `ACTION` 26–28, `WHEEL_ENTRY` 29–31, `WHEEL_SELECT`; no "reserved" |
| C-11, Q10 | pass | §8.4 "horizontal FOV 90° by default (setting 60–120°" |
| C-15 | pass | §5.5 `ShopHaggle { shop }` and doc 17's 10 kinds plus `SetLockMode` |
| §4.1 stale gaps | pass | none names doc 02; G1 to G13 are deleted (the closing note is cut to one numbering line) |

### Check 2: DECISIONS rows

- Rows landing in doc 02: 5, 6 (superseded by 98), 24, 37, 43, 87, 92 and 98 pass. **112** (Steam only) and **123** (Steam networking plus our own verifier, EOS SDK dropped) were contradicted: the chapter had an EOS binding row, an `eos` feature, an Epic Games Store bullet, an EOS-build flag note and an EOS row in the doc 16 Interface assumption. Fixed in Imports, §2.1, §2.2 (EOS row deleted, `opus` rationale), §3.2 (`crate.platform`, `crate.server`, `crate.services`), §14 (new "One storefront" bullet; Steam Networking Sockets), §17.3, Exports (`flag.platform.steam`; `flag.net.crossplay` kept but marked inert until doc 16 rules), Interface row 16 and the invariant. **120**: `llama-cpp-2` row now says it ships at 1.0 as an opt-in and Early Access uses the template grammar.
- Row **113** (no demo, no public playtest build; lands in docs 15 and 21) was contradicted by §17.2's `playtest` channel on the "Steam Playtest app"; it is now an invite-only, password-protected Steam branch.
- Row 98 leftovers: "32,768 on Deck and on the web build" (§8.7) and "UASTC for the web build" (§11.4) removed.
- Row 43: §13.2 sent Workshop distribution through `svc.workshop`, which doc 16 defines as the mod mirror for non-Steam platforms; now Steam Workshop.
- Changed rows: invasions (slot 4, `InvaderJoin`, `mode.invasion_open`, empty mod set) pass; limited building (§13.3 `set-block` changes only destructible blocks and placeables; §12.4 Margin trophy slots) pass; softer Erasure (no repeat-spawn text; `rank.hunter` music per row 91) pass; Echo cast, USD 19.99, procedural ending, World Laws, pact curses, 12+ enemies, Errata limbs: no text in this chapter; text-only narration: no voiced narration (`access.narrator` is doc 17's accessibility speech); micro-voxels pass; mounts and durability pass. **Hybrid glyph learning (33)**: the meta profile did not list the account Glyph Library; §12.4 now does (Canon §17.1 `GLYPH_LIBRARY_START`), and the doc 14 Interface row names it.
- Rows 77 to 88: 83 matches `DropLucre { amount: i64 }`; 87 matches §11.2 and §13.4; 88 matches the imported "terrain stopping every projectile"; the rest have no text here. Row 90 (keep "Crowns"): no Crowns text in this chapter.

### Check 3: actions_02 and the brief

- Items 1 to 9 pass (item 1 and 3 as in Check 1; item 2: no `<< 32` remains in §4 code, only `<< 64` in example literals; item 4 to 8 as quoted; item 9: §2.1 toolchain row).
- Module lists: `palimpsest_math` matches the workspace's `lib.rs`. **Fixed** `palimpsest_data`: the chapter named a module `load`, while doc 21 WP-005, WP-006 and WP-077 cite `palimpsest_data::schema`, `::loader` and `::mods`; the row now lists `loader`, `schema` and `mods` and says which modules the slice already has. The `sim` split into `api` and `serial` is now attributed to doc 21 WP-010 (doc 21 §4.1 does not say it).
- `ai.curve.*` referenced only (`"curve"` key); `art.mesh.enemy.gothic_horror.ghoul` exists in doc 18 §3.8 and §10.1; `item.x_frostpack.ice_blade` is the canon's own `x_<mod_id>` example. 250 mm micro-voxels pass.
- Interface rows other chapters address to doc 02, re-read on disk: **doc 15** now assumes `palimpsest-verify` built from `palimpsest_econ` and *clients* keeping reported-run replays 30 days (§10.4). §12.2 had the services keeping replays "reported by another player"; corrected to doc 15 §10.4's client rule. `palimpsest-verify` is now a second binary of `palimpsest_tools` (it links `palimpsest_econ`; its `--content` check needs doc 11's item generator in `palimpsest_sim`, which econ may not link), and econ's row adds `FairStream` and the pull, open and Transcription algorithms. **Doc 18** asks §9 to summarize §6.6 as written (pools never borrow; priority-7 pools stop the oldest voice; other pools replace the quietest only for a louder request); §9 corrected. **Doc 03**'s row is now fully met (the shadow areas were the one gap). The doc 14 Interface row's "reads as `run.end.abandoned_version`" workaround was stale (doc 14 §12.2 now says `run.end.abandoned_version`) and is removed; the doc 03 Interface row's micro-voxel and `raycast_blocks` requests are met on disk (doc 03 §4.3, §4.4, §5.18, §16.1) and moved into Imports.

### Check 4: leftover grep

Hits for mount, durability, Epic and EOS are now only retirement records (Imports, intro, bit 25, §14 "One storefront", Interface row 16). "broken" (§18 item 3) is "a broken invariant", not an item. No hits for ride, saddle, cavalry, repair, wear, breaks, Delving, voice actor, voiced, VO, 24.99, Item Codex, Crowns, Redactor, `margin.plot`, building plot or 5400. "web build" (2 hits) fixed.

### Check 5: lint

`python3 tools/spec_lint.py --chapter 02` prints `== 02 words=28231` with no findings; `--json` `multiply_defined` is `{}`.

### Check 6: internal consistency

Recomputed with Python (`verify02/check.py`, Python `blake3` 1.0.11): `content_hash` 141 bytes `92c159…2cc9`; `presentation_hash` 47 bytes `596015…559b` and 95 bytes `08ea33…72d1`; `modset_hash` 77 bytes `a4cd5d…5817`; test 41 blob 22 bytes `a6e6bf…f638` and `0bac78…01dd`; quad words `0x1DEC0F8C`, `0x0086F205`; FOV 64.01°, 39.68°, 94.54°; BC7 5,488 bytes per layer and 7,024,640 bytes for five arrays; `closest_seg_seg` Examples 1 to 4 (including the unclamped raw `s` −18446744052234715136, `dist2` 313594649253062377472 and 461168601799789117441); `ray_capsule` raws 20186346293, 20514452597, 21474836480 (`h = 0`), miss at 2.0 m; `ray_sphere` tangent 21474836480; DDA raws 2684354559, 3579139414, 8053063678, 10737418242, 13421772797 with `max_steps = 15`; budget totals 3.97, 12.40, 4.33, 4.87, 14.50 ms; test 14's new shadow value (15 − 8) × 17 = 119; test 50 byte strings. All match. Every table has a separator row and a constant cell count. Exports list every `crate.`, `flag.` and `perf.` ID the body uses. Canon gaps holds only G14 (Canon §1.3 still has no exception for `s[1] << 17`) plus one numbering line. Header: "Conforms to canon-1.11".

### Status of the fixer's requests

- 03: met on disk; no change needed. Doc 03's Interface row for doc 02 can be marked met.
- 11 (with 19): met on disk (doc 11 principle 8 marks `lines` `"x-patchable": true`).
- 21, 01, 17 and the Rust slice: still open, plus the new requests below.

### New requests for other chapters

- **16:** DECISIONS 112 and 123: drop the EOS backend, `/v1/auth/epic`, `net.route.relay` for Epic members, §3.6 crossplay identity, the Epic value of `platform: u8` and the Epic Workshop mirror; rule on whether `flag.net.crossplay` retires (doc 02 keeps it, marked inert); drop "an Epic Online Services platform layer" from the Interface row for doc 02; cite rows 112 and 123 instead of 73's Epic build.
- **21:** DECISIONS 112 and 123: drop "a pinned EOS binding behind feature `eos`" from the doc 02 Interface row and any EOS work package; G16 and RQ-13 still name Epic and the Steam Playtest app (row 113). The test-count row "doc 02 | 47 | 47" should read 53.
- **01:** §9.2, PAR-03, Q-11 and the doc 16 Imports row still describe Epic Games Store crossplay (DECISIONS 112, 123).
- **15:** `palimpsest-verify` is a `palimpsest_tools` binary that links `palimpsest_econ` (doc 02 §3.2), because `--content` regenerates Keepsakes through doc 11's generator in `palimpsest_sim`; its MIT licence beside the proprietary crates (DECISIONS 44) is doc 15's to settle.
