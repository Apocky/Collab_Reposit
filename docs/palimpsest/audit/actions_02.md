# Audit actions for doc 02 (from the Rust vertical slice)

Decided by the orchestrator on 2026-10-11. The doc 02 chapter fixer applies these.

1. **closest_seg_seg**: replace the narrowing `.expect("invariant: ...")` calls with the clamp-before-narrow form in 256-bit arithmetic, as implemented in `palimpsest/crates/palimpsest_math/src/geom.rs`. Add both failing cases from that module's tests as worked examples.
2. **Shifts**: every `x << 32` in §4 code becomes a checked multiply by `2^32` (canon §1.3 shifts rule).
3. **serde**: §3.3 wins. `palimpsest_math` depends only on `blake3`, with an optional `serde` feature that is off by default and used only by tools. Canonical save serialization (§6.5) is hand-written and never uses serde. Fix §4.1 and §4.7 accordingly. `Rng::state()` and `Rng::from_state()` are the save interface.
4. **DDA bound**: the voxel DDA takes `max_steps`. Doc 03 callers use `max_steps = 3 × ceil(t_max) + 3`. The iterator yields at most `max_steps + 1` blocks and stops cleanly at the `i32` block-range edge.
5. **Tangent rays**: `ray_sphere` and the `ray_capsule` body path both treat a tangent ray (`h == 0`) as a hit.
6. **I256 ordering**: use derived `Ord` (`a.cmp(&b)`), not an inherent `cmp` method.
7. **Paths**: the trig table lives at `crates/palimpsest_math/src/trig_table.rs`.
8. **Draw counters**: §4.7's debug draw counters move to `palimpsest_sim` and never enter state, saves or hashes (canon §2.5).
9. **Toolchain note**: Rust 1.99.0 is pinned and installs cleanly through `rust-toolchain.toml`.
