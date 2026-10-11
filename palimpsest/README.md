# PALIMPSEST: Rust vertical slice

This is the first executable piece of PALIMPSEST: the deterministic core that
every later system builds on, written in Rust as a Cargo workspace. It contains
no renderer, no Bevy and no networking yet. What it does contain is exact
integer math, a reproducible RNG, the canon's formulas, and a simulation
skeleton that saves, restores and hashes its state bit for bit.

The binding specification is `../docs/palimpsest/spec/00_CANON.md` (the
canon). Every public function cites the canon section it implements, and
every canon §21 acceptance test is a named Rust test (see `AGENTS.md`).

## Quick start

```sh
cd palimpsest
cargo test --workspace                                  # 465 tests; about a second once compiled
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
```

`rust-toolchain.toml` pins Rust 1.99.0 (edition 2024, resolver 3), and rustup
installs it on first use. Overflow checks are on in every profile, so an
arithmetic bug panics instead of silently desyncing.

## What is in the slice

| Crate | Spec | What it gives you |
|---|---|---|
| `palimpsest_math` | canon §1, §3, §13.8; doc 02 §4 | The numeric foundation. Integer primitives (`floor_div`, `mul_bp`, `round_half_even`, 256-bit `I256`). The in-house Q32.32 `Fx` with checked `mul_fx`, `div_fx` and `sqrt_fx`. A committed 16,385-entry sine table and CORDIC `atan2_bam`. Capsules, rays and the voxel DDA. xoshiro256++ with SplitMix64 seeding, sequential streams and keyed draws. FNV-1a and BLAKE3. Crockford seeds with daily and weekly seeds. The power tables `P`, `T`, `H` and `S`. `no_std`. |
| `palimpsest_data` | canon §1.1, §1.6, §1.10, §19; doc 02 §11 | Content IDs and `id32`, a collision-checked ID registry with aliases, `loc.` text keys, hex `u64` strings, and (feature `loader`) the no-floats JSON content checks. |
| `palimpsest_sim` | canon §2, §4-§17; docs 03, 04, 05, 11, 12 | The rules as pure functions. Tick schedule, timers and the rate accumulator. EntityIds. Genres, tolerance, Contradiction and Anachronism Edge. All 44 derived-stat rows. Skills and levels. Movement, jump, fall and dodge. The full damage pipeline with its entry points, blocking and wards. Statuses, buildup, DoT, ranks and Erasure. Rarity, quality, type hiding, enhancement and Marginalia armor HP. Also a `Sim` skeleton: command log, `step`, canonical save, `state_hash`, restore. |

The `Sim` skeleton runs a small stand-in world: wandering creatures, melee
hits, bleeding and jumps, with every non-canon number labeled `skeleton` in
`sim.rs`. That is enough to prove the determinism contracts end to end.
10,000 ticks from a fixed input log give the same `state_hash` with or without
a save and load at tick 4,321 (canon test 32). Folio 7 starts from the same
Folio-owned state whether or not Folios 1 to 6 were played (canon test 31,
adapted).

## How it maps to the spec

- **Canon §21:** all 44 acceptance tests are implemented as
  `canon_tNN_<name>` tests. `AGENTS.md` has the full map. Three of them (31,
  36, 37) test the canon rule without the generators (worldgen, glyphs, items)
  that later milestones add.
- **Canon §22 invariants:** there are no floats, no `HashMap` or `HashSet`
  and no `unsafe` code. `clippy.toml`, the workspace lints and the
  source-scanning test `tests/no_floats.rs` (canon test 33) enforce this.
  Bp chains run in canonical order, one `mul_bp` per step. Action-caused
  randomness is keyed.
- **Doc 02 (technical architecture):** the crate split and APIs follow §3.4,
  and the math follows §4. Where doc 02 and the canon differ, the canon wins.
  For example, saves use the hand-written fixed-width encoding of canon §2.5,
  not postcard.
- **Gaps:** where the canon is silent the code picks a documented default,
  marked `GAP:` in the doc comment (`grep -rn 'GAP:' crates`). Each one is an
  open question for the spec owners.

## Not in the slice yet

Rendering (Bevy), the block world and worldgen, liquids, gases, fire and
light grids, AI, perception, the content loader and content files, items and
inventory generation, netcode, and the UI. The CI workflow
(`../.github/workflows/palimpsest-ci.yml`) already gates every change to this
directory with fmt, clippy and tests on Rust 1.99.0.

## Layout

```
palimpsest/
  Cargo.toml            workspace: members, pinned dependency versions, lints, profiles
  rust-toolchain.toml   Rust 1.99.0 with clippy and rustfmt
  clippy.toml           disallowed types: f32, f64, HashMap, HashSet
  crates/
    palimpsest_math/    numeric foundation (no_std)
    palimpsest_data/    content IDs, registry, text keys, content checks
    palimpsest_sim/     rules and the Sim skeleton; tests/no_floats.rs
  tools/gen_trig.py     regenerates the committed sine and arctangent tables
  content/, schemas/    reserved for content files and JSON schemas (empty)
  AGENTS.md             build, invariants, SPEC-comment convention, canon test map
```
