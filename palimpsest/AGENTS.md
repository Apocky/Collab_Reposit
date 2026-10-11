# AGENTS.md: working on the PALIMPSEST Rust slice

This file is for anyone, human or agent, who changes code in this workspace.
Read it before your first edit.

## Binding specification

- **Canon:** `../docs/palimpsest/spec/00_CANON.md` is BINDING. When any other
  document disagrees with it, the canon wins. When it is silent, the owning
  document named in canon §19 decides. The code follows **canon-1.11**;
  `CHANGELOG_canon_1_11.md` lists every change made to reach it and every
  canon-1.11 rule that is not in the slice yet.
- **Architecture:** `../docs/palimpsest/spec/02_TECH_ARCHITECTURE.md` gives
  the API sketches (§3.4) and the math details (§4). It loses to the canon on
  any conflict.
- **Simulation core:** `../docs/palimpsest/spec/03_SIMULATION_CORE.md` (tick
  order, components), plus docs 04, 05, 11 and 12 for the systems in the slice.
- Read the canon section you implement **completely** before you code. Every
  number in a test comes from the canon text, never from running the code.

## Build and test

The toolchain is pinned to **Rust 1.99.0** by `rust-toolchain.toml`, with
edition 2024 and resolver 3. Run everything from this directory
(`palimpsest/`).

```sh
cargo fmt --all                                               # format
cargo fmt --all -- --check                                    # CI format gate
cargo build --workspace
cargo test --workspace                                        # 558 tests
cargo test --workspace --all-features --release               # release, serde and loader features
cargo clippy --workspace --all-targets -- -D warnings         # CI lint gate
cargo clippy --workspace --all-targets --all-features -- -D warnings
python3 tools/gen_trig.py --check                             # committed trig tables (needs mpmath)
```

To work on one crate, use `cargo test -p palimpsest_sim` (or `palimpsest_math`
or `palimpsest_data`). Run one canon test with
`cargo test -p palimpsest_sim canon_t17`.

CI runs the same gates in `../.github/workflows/palimpsest-ci.yml` on every
push and pull request that touches `palimpsest/**`.

## Crate map

| Crate | Role | Depends on |
|---|---|---|
| `palimpsest_math` | `no_std`. Integer primitives, `Fx` Q32.32, trig and CORDIC, geometry, RNG, hashes, seeds, power tables. Optional feature `serde`. | `blake3` (and `serde` behind the feature) |
| `palimpsest_data` | `no_std` + `alloc` core: content IDs, `id32`, the `x_<mod_id>` mod namespace, the ID registry, text keys, hex `u64`. Feature `loader` (off by default) adds std, `serde_json` and the JSON content checks. | `palimpsest_math` |
| `palimpsest_sim` | The deterministic simulation: formulas from canon §2 and §4 to §17, and the `Sim` skeleton with save, restore and `state_hash`. | `palimpsest_math`, `blake3` |

| Module | Canon / doc sections | Contents |
|---|---|---|
| `math::int` | §1.3 | `floor_div`, non-negating `ceil_div`, `round_half_even`, `mul_bp` (+ `_wide`, `_chain`), clamps, `I256` |
| `math::fx` | §1.4, doc 02 §4.1-4.2 | `Fx`, `fx_from_milli`, `mul_fx`, `div_fx`, `sqrt_fx`, `isqrt_u128` |
| `math::trig`, `math::trig_table` | §1.9, doc 02 §4.3 | 16,385-entry sine table (generated), `sin_bam`, `cos_bam`, negate-first `fwd`, `ang_diff` |
| `math::cordic`, `math::cordic_table` | §1.9, doc 02 §4.4 | `atan2_bam` (angle of `(x, y)` from +x, ±1 Bam), `yaw_from_dir(dx, dz) = atan2_bam(-dx, -dz)`, `pitch_from_dir` |
| `math::geom` | doc 02 §4.5-4.6 | `Fx3`, `dist2`, capsules, segment-segment, ray casts, Amanatides-Woo voxel DDA, semi-implicit Euler |
| `math::hash` | §1.1, §2.4-2.5 | `fnv1a32`, `fnv1a64`, `id32`, `blake3_256` (concatenation), `blake3_fields` (length-prefixed fields) |
| `math::rng` | §3.1-3.5, §3.8 | xoshiro256++, SplitMix64, `mix64`, `stream_seed`, sub-streams (`derive`), `keyed_rng` with checked `seq` and the `seq` allocations, `object_keyed_rng`, offer generators, `nar_rng` and `pres_hash`, the stream name registry with owners |
| `math::seed` | §3.6 | Crockford seed display and decode, daily and weekly seeds, ISO weeks |
| `math::tables` | §13.8 | Literal rows 1-40 of `P`, `T`, `H`; `S`; depth cap 300; the canon-1.11 uses of each table |
| `data::ids`, `data::registry` | §1.1, §19 | Content-ID grammar, `Id32`, the `x_<mod_id>` mod namespace, collision-free registry with aliases and mod records |
| `data::text_key` | §1.10 | `loc.` text keys (no `id32`) |
| `data::hex` | §1.6 | `"0x"` hexadecimal `u64` strings |
| `data::content_check` | §1.6, doc 02 §11.1 | No-float JSON content validation with the `±(2^53 - 1)` integer bound (feature `loader`) |
| `sim::time` | §2.2-2.3, §2.5 | Tick schedule and phases, named boundaries and hash boundaries, status timers, the rate accumulator, `in_combat` windows |
| `sim::entity` | §1.11 | `EntityId` and item `uid` per Folio, phase keys |
| `sim::genre` | §5.1-5.2, §5.4, §5.9 | Genre table, Origin tolerance, genre identity rows |
| `sim::contradiction` | §5.3-5.8, §5.10-5.11 | Requirements, tolerance, Contradiction, Anachronism Edge and Shock of the New ranks, ambient terms, local genre (seams), World Laws, Unmoored |
| `sim::stats` | §6 | All 44 derived-stat rows and the NPC row set |
| `sim::progression` | §7-8, §13.8, §17.2-17.3 | Skills, skill XP normalized before party scaling, training marks, level curve, kill XP, co-op recipients, Erasure-hunter multipliers, enemy HP chain |
| `sim::movement` | §4.2 | Speeds and direction classes, final speed with `body_move_bp`, jump and air jump, apex simulation, the fall-damage chain, dodge |
| `sim::damage` | §9.1, §10, §17.2, §17.5 | Source magnitude (step 0), the full §10.5 pipeline with steps 3a, 6b and 6c, the §10.6 entry points, blocking, wards, severing, hit batches, lethal resolution |
| `sim::status` | §11, §17.3, §17.3a | Status table, buildup chain, post-proc protection, DoT, direct duration, ranks, Erasure and its hunter, `counter.disruptor`, shock chains |
| `sim::loot` | §13.1-13.8 | Rarity table and roll, quality, equipment slots and the Exotic limit, the ward capacity chain, type hiding and knowledge, `ENH_CAP`, Emendations and pact curses, Marginalia armor HP |
| `sim::sim` | §1.5, §2.4-2.5, §11.8, doc 02 §5.5, §6.5 | `Sim` skeleton: commands and neutral substitutes, `step`, canonical serialization, `state_hash`, save timing, save and restore, replay log with `presentation_hash` |

## Invariants (never break these)

1. **No floats.** `f32` and `f64` never appear in non-test code (canon §1.2,
   §22). `clippy.toml` disallows the types, the workspace lints deny float
   arithmetic, and `crates/palimpsest_sim/tests/no_floats.rs` (canon test 33)
   scans the source of all three crates. Test code may use floats only to
   cross-check, never to define an expected value.
2. **No `HashMap` or `HashSet`.** Use `BTreeMap`, `BTreeSet` or a sorted
   `Vec` (canon §1.11).
3. **No `unsafe`.** Every crate has `#![forbid(unsafe_code)]`.
4. **Overflow panics in every profile.** `overflow-checks = true` for dev,
   release and test. Use `i128` intermediates wherever the canon says so (for
   example `mul_bp`, armor `DR_bp`, `dist2`). Rust's `<<` is not overflow
   checked, so canon §1.3 reads every normative `x << k` as a checked
   multiplication. Only shift when the range proves no bit can be lost, and
   say so in a comment.
5. **Bp chains run left to right in the listed order**, one `mul_bp` per
   step and never reordered (canon §1.3). Clamp happens last unless the
   formula places it elsewhere. Status multipliers come after all
   non-status factors, in ascending status ID order (canon §11.7).
6. **Randomness:** draws caused by an entity's action use
   `keyed_rng(stream, folio, actor, tick, seq)`, with `seq` in `0..=0xFFFF`
   taken from a published allocation (`math::rng::seq`); a running counter
   stays inside its reserved range. Values fixed per object use
   `object_keyed_rng`. Bulk host processes use the sequential stream that
   owns them, or a labeled sub-stream of it (canon §3.2-3.3). Every hit draws
   exactly one crit roll, and sneak attacks draw none (canon §3.4).
7. **Iteration order:** anything that mutates state, emits events, creates
   entities or draws randomness runs in ascending `EntityId` (or `uid`) order.
8. **Saves:** the canonical serialization is hand-written and self-delimiting.
   It never depends on serde (canon §2.5). Any other hash over several
   variable-length fields uses `blake3_fields`, which length-prefixes each one.
9. **Generated tables are never edited by hand.** `trig_table.rs` and
   `cordic_table.rs` come from `python3 tools/gen_trig.py` (mpmath, 50
   digits); use `--check` to verify them.

## The SPEC-comment convention

- Every public function's doc comment **starts with** `SPEC: canon §x.y`, adding
  the owning doc when relevant, for example
  `/// SPEC: canon §10.5 step 5, doc 05 §12.3. ...`. All 691 public functions
  follow this today, and public types and constants do the same. Keep it
  that way.
- Where the canon is silent or ambiguous and the code picks a default, the doc
  comment carries a `GAP:` paragraph that states the choice. List them with
  `grep -rn 'GAP:' crates`. Every GAP is an open question for the spec owners.
  Do not resolve one silently in code.
- Open question: doc 02 §18 item 2 plans an `xtask spec-refs` that matches
  `SPEC: \d\d §[0-9.]+` (for example `SPEC: 00 §1.1`). The slice uses
  `SPEC: canon §...` as its task instructions require. One form will need to
  be chosen before that xtask lands.

## Test conventions

- A canon §21 acceptance test `N` is a Rust test named `canon_tNN_<short_name>`
  (two digits). A doc 02 acceptance test is `doc02_tNN_<short_name>`.
- Every formula you implement gets unit tests with the canon's exact numbers.
  Recompute a value you doubt with exact integer Python, never with floats.
- Where the canon states an invariant (monotonicity, ranges, round trips,
  determinism, no drift), add a `proptest` property.
- Proptest regression files under `proptest-regressions/` are committed.

## Canon §21 acceptance-test map

Paths are `crate::module::tests::name` unless stated otherwise.

| # | Test | Rust test |
|---|---|---|
| 1 | xoshiro256++ vectors | `palimpsest_math::rng::tests::canon_t01_xoshiro_vectors` |
| 2 | Second seed | `palimpsest_math::rng::tests::canon_t02_second_seed` |
| 3 | Lemire range | `palimpsest_math::rng::tests::canon_t03_lemire_range` |
| 4 | FNV-1a | `palimpsest_math::hash::tests::canon_t04_fnv1a` |
| 5 | mix64 | `palimpsest_math::rng::tests::canon_t05_mix64` |
| 6 | Stream seed | `palimpsest_math::rng::tests::canon_t06_stream_seed` |
| 7 | Keyed draw | `palimpsest_math::rng::tests::canon_t07_keyed_draw` |
| 8 | Keyed isolation | `palimpsest_math::rng::tests::canon_t08_keyed_isolation` (+ proptest `keyed_isolation_property`) |
| 9 | Seed display | `palimpsest_math::seed::tests::canon_t09_seed_display` |
| 10 | Rate accumulator | `palimpsest_sim::time::tests::canon_t10_rate_accumulator` |
| 11 | Fixed point | `palimpsest_math::fx::tests::canon_t11_fixed_point`, `canon_t11_div_fx_zero_panics` |
| 12 | Trigonometry | `palimpsest_math::trig::tests::canon_t12_trigonometry` |
| 13 | Derived stats | `palimpsest_sim::stats::tests::canon_t13_derived_stats` |
| 14 | Contradiction example | `palimpsest_sim::contradiction::tests::canon_t14_contradiction_example` |
| 15 | Disconnection | `palimpsest_sim::contradiction::tests::canon_t15_disconnection` |
| 16 | Origin tolerance | `palimpsest_sim::genre::tests::canon_t16_origin_tolerance_sums` |
| 17 | Armor | `palimpsest_sim::damage::tests::canon_t17_armor` |
| 18 | Chip floor, NPC | `palimpsest_sim::damage::tests::canon_t18_chip_floor_npc` |
| 19 | Chip floor, Erratum | `palimpsest_sim::damage::tests::canon_t19_chip_floor_erratum` |
| 20 | Rarity weights | `palimpsest_sim::loot::tests::canon_t20_rarity_weights` |
| 21 | Power tables | `palimpsest_math::tables::tests::canon_t21_power_tables` |
| 22 | Skill curve | `palimpsest_sim::progression::tests::canon_t22_skill_curve` |
| 23 | Level curve and kill XP | `palimpsest_sim::progression::tests::canon_t23_level_curve_and_kill_xp` |
| 24 | Jump apex | `palimpsest_sim::movement::tests::canon_t24_jump_apex` |
| 25 | Zero-VERVE crit | `palimpsest_sim::stats::tests::canon_t25_zero_verve_crit` |
| 26 | Death's Door | `palimpsest_sim::stats::tests::canon_t26_deaths_door` |
| 27 | Dodge | `palimpsest_sim::movement::tests::canon_t27_dodge` |
| 28 | Direct duration | `palimpsest_sim::status::tests::canon_t28_direct_duration` |
| 29 | ID validation | `palimpsest_data::canon_tests::canon_t29_rejects_bad_id_grammar`, `canon_t29_rejects_equal_fnv1a32`, `canon_t29_text_key_has_no_id32`. There is no loader yet (milestone M2), so these run parse, then check, then `IdRegistry::build`. |
| 30 | HP display | `palimpsest_sim::sim::tests::canon_t30_hp_display` |
| 31 | Folio isolation | `palimpsest_sim::sim::tests::canon_t31_folio_isolation`, **adapted**. The slice has no worldgen or block data, so the test compares the Folio-owned state instead: the per-Folio stream states, the EntityId and uid counters, and the entities created in that Folio. Byte-identical block data needs worldgen (doc 12). |
| 32 | Save and load | `palimpsest_sim::sim::tests::canon_t32_save_and_load` |
| 33 | No floats | `palimpsest_sim` integration test `tests/no_floats.rs::canon_t33_no_floats` (plus `scanner_self_test`). The slice has no conversion-boundary module, so no file is exempt. |
| 34 | No floats in content | `palimpsest_data::canon_tests::canon_t34_rejects_fraction_or_exponent` |
| 35 | Genre-native aether | `palimpsest_sim::stats::tests::canon_t35_genre_native_aether` |
| 36 | Glyph dependency | `palimpsest_sim::contradiction::tests::canon_t36_glyph_dependency`. This tests the dependency and requirement rule; glyph generation (doc 06) is out of slice scope. |
| 37 | Type hiding | `palimpsest_sim::loot::tests::canon_t37_type_hiding`. This tests the §13.6 starting-knowledge rule; item generation (doc 11) is out of slice scope. |
| 38 | Per-target sneak eligibility | `palimpsest_sim::damage::tests::canon_t38_sneak_eligibility_per_target` |
| 39 | Marginalia armor HP | `palimpsest_sim::loot::tests::canon_t39_marginalia_armor_hp` |
| 40 | Genre spread | `palimpsest_sim::genre::tests::canon_t40_genre_spread` |
| 41 | Ward depletion | `palimpsest_sim::damage::tests::canon_t41_ward_depletion` |
| 42 | Friendly fire | `palimpsest_sim::damage::tests::canon_t42_friendly_fire` |
| 43 | Forward vector | `palimpsest_math::trig::tests::canon_t43_forward_vector` |
| 44 | Weekly seed | `palimpsest_math::seed::tests::canon_t44_weekly_seed` |
| 45 | range_incl extremes | `palimpsest_math::rng::tests::canon_t45_range_incl_extremes` |
| 46 | Yaw from direction | `palimpsest_math::cordic::tests::canon_t46_yaw_from_dir` |
| 47 | Sub-stream | `palimpsest_math::rng::tests::canon_t47_sub_stream` |

All 47 are implemented, and none is out of slice scope. Tests 31, 36 and 37
exercise the canon rule without the generators (worldgen, glyphs and items)
that later milestones add. When those land, extend these tests rather than
writing new ones.

Doc 02 acceptance tests in the slice: `doc02_t01_isqrt_values`,
`doc02_t05_atan2_values`, `doc02_t06_million_random_pairs_within_one_bam`,
`doc02_t19_validator_vectors` and `doc02_t36_i256_mul_reference`. The worked
examples behind doc 02 tests 2-4 and 7-11 are covered by the `sqrt_fx`,
`mul_fx`, `div_fx`, table-hash, `fwd`, `closest_seg_seg`, `ray_capsule` and
`dda_worked_example` tests. Doc 02 test 46 (not canon test 46) is
`tools/gen_trig.py --check`. Doc 17 tests 2, 3 and 45 (Scribe's Mercy) are
`palimpsest_sim::damage::tests::doc17_t02_mercy_damage`,
`doc17_t03_mercy_and_wards` and `doc17_t45_mercy_exemptions`.

## Known spec tensions the code currently resolves by default

- **Test 39 vs §13.8.2:** read literally, the formula floors each armor piece
  (giving 4,538,770 mHP). The canon's 4,538,772 needs one floor over the sum,
  and `gear_hp_mhp` does that. `gear_hp_piece_mhp` keeps the per-piece reading.
- **§4.2 direction classes:** pure sideways input (90°) exceeds 12288 Bam, so
  it is backpedal (× 8000), as the canon is written.
- **§1.3 shifts vs §3.1 hashing:** xoshiro's `s[1] << 17` drops bits by
  design (it defines the generator and §21 tests 1 to 3), so it is not a
  checked multiplication. The keyed `(seq as u64) << 48` is checked since
  canon-1.11: a `seq` above `0xFFFF` panics (canon §3.3).
- **§10.7 split-back remainder:** the remainder goes to the largest-share
  type among the types that still carry damage, never to an immune type
  (marked `GAP:` in `apply_block`).
- **§10.8 hit batches:** the canon names only actor hits and
  "environmental, contact, surface and DoT hits". The other entries follow
  doc 03, which canon §2.2 makes the owner of tick order: a fall hit is
  queued by `sys.movement` for `sys.damage`, so it resolves in the first
  batch; shock chains and thorns, queued while `sys.damage` runs, resolve in
  `sys.damage_late` (marked `GAP:` in `HitBatch::of`).
- **§9.1 vs doc 04 §8.2:** "at most 2500 Bp of its max" is read through doc
  04's floored `frac`, which admits a pool a hair above an exact quarter
  (marked `GAP:` in `severs`).

Every other default is marked `GAP:` in the code.
