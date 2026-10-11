# 00 — CANON: Single Source of Truth

As of 2026-10-11. Status: BINDING. Version: canon-1.5 (changelog in §23).

This document defines every shared identifier, enum, unit, constant and cross-system formula in PALIMPSEST. When any other spec document disagrees with this one, this one wins. When this one is silent, the owning document named in §19 decides. `DESIGN_BRIEF.md` explains why; this document says exactly what.

## Table of contents

1. Conventions: identifiers, numeric types, integer and fixed-point primitives, units, content encoding, ranges, coordinates, angles, text keys, entity IDs
2. Time, tick schedule, durations, sessions, replays and saves
3. Random number generation
4. World and movement constants
5. Axioms, genres, Contradiction and Anachronism
6. Attributes, sub-attributes and master derived-stat formulas
7. Skills
8. Character level and experience
9. Body plans and hit locations
10. Damage types, resistances, armor and the damage pipeline
11. Status effects and buildup
12. Resources
13. Items: categories, slots, rarity, quality, knowledge, curses, enhancement, power tables
14. Currencies
15. Factions
16. Callings, Origins and Echoes
17. Run structure, party scaling, enemy ranks and reference anchors
18. Core simulation events
19. Namespace ownership
20. Glossary
21. Acceptance tests
22. Invariants annex
23. Changelog

---

## 1. Conventions

### 1.1 Content identifiers

Every piece of content and every enum value has a string **content ID**.

- Grammar (regex): `^[a-z][a-z0-9_]*(\.[a-z0-9_]+){1,5}$`. Maximum 64 bytes, ASCII only.
- The first segment is the **namespace**. §19 assigns each namespace to one owning document.
- Content IDs are permanent once shipped. Renaming requires an alias entry in `data/aliases.json`.
- `id32 = fnv1a32(utf8(id))` is the compact form used in saves and packets. It is computed for content IDs only, never for text keys (§1.10).
- The content loader rejects any `id32` collision among base content and aliases at build time. An alias's old ID takes part in the collision check and maps to the new ID's `id32` at load.
- A mod whose content ID collides with an already loaded `id32` is rejected at mod load with error `id32_collision`.

FNV-1a constants:

| Variant | Offset basis | Prime |
|---|---|---|
| fnv1a32 | `0x811C9DC5` | `0x01000193` |
| fnv1a64 | `0xCBF29CE484222325` | `0x00000100000001B3` |

### 1.2 Numeric types

Simulation state never contains `f32` or `f64`. Rendering, audio and UI may use floats, but only for values derived from simulation state through the conversion boundary module (doc 02), never fed back.

| Type | Representation | Used for |
|---|---|---|
| `Fx` | in-house `struct Fx(i64)`, signed Q32.32 | positions, velocities, accelerations, distances, physics |
| `Milli` | `i64`, display value × 1000 | HP, damage, stamina, aether, composure, toxicity, poise, ward, skill XP |
| `Bp` | `i32` basis points, 10000 = 100% = ×1.0 | probabilities, percentage modifiers, multipliers, shares, penetration |
| `Bam` | `u16` binary angle, 65536 = one full turn | yaw, pitch, cone half-angles |
| `Ticks` | `u32` | every duration, cooldown, timer and delay |
| `Tick` | `u64` | absolute simulation time since run start |
| `Grams` | `i64` | mass |
| `Ml` | `i64` milliliters | liquid and gas volume |
| `DeciC` | `i32`, tenths of a degree Celsius | temperature |
| `Light` | two `u8` channels 0..=15: `light_sky` (genre ambient floor and sky) and `light_block` (placed and dynamic sources) | effective light = `max(light_sky, light_block)` |
| `Db` | `i32` whole decibels | sound level |
| stats | `i32` | attribute, sub-attribute and skill values |
| satiation | `i32` whole points | `res.satiation` |
| char XP | `i64` whole points | character experience |
| money | `i64` whole units | every currency |
| power tables | `i64` Bp | `P(d)`, `T(d)`, `H(d)`, `S(lvl)` (§13.8) |
| armor | `i64` | AR per body part |
| EntityId, item uid | `u64` | §1.11 |

### 1.3 Integer primitives (normative)

`palimpsest_sim` sets `overflow-checks = true` in every Cargo profile, so overflow panics in debug and release alike.

- `floor_div(a, b)` requires `b > 0` and panics otherwise. It equals `a.div_euclid(b)`, which is floor division when `b > 0`.
- `ceil_div(a, b)`, with `b > 0`, returns the smallest integer `q` with `q × b >= a`. It equals `-floor_div(-a, b)` wherever that does not overflow; implementations use a non-negating form so `a = i64::MIN` is defined.
- **Shifts.** Rust's `<<` is not covered by overflow checks. Every `x << k` in a normative formula means a checked multiplication by `2^k` that panics on overflow. `>>` on a signed integer is an arithmetic shift, which is floor division by `2^k`.
- `round_half_even(a, b)` for `b > 0`: `q = floor_div(a, b)`, `r = a - b*q`. Return `q + 1` if `2r > b`, `q` if `2r < b`, otherwise `q + (q & 1)`.
- `mul_bp(x, bp) = floor_div(x as i128 * bp as i128, 10000)`, narrowed back to the input width with a checked conversion that panics on overflow.
- Several Bp multipliers chain left to right, rounding after each step, in the exact order a formula lists them. Implementations must not reorder for speed.
- Clamp always happens last in a formula unless the formula places it elsewhere.

### 1.4 Fixed-point primitives (normative)

`Fx` is implemented in-house so no crate update can change results. Raw value `r` represents `r / 2^32`.

| Function | Definition |
|---|---|
| `Fx::from_int(n)` | `Fx((n as i64) << 32)` |
| `fx_from_milli(m)` | `Fx(floor_div((m as i128) << 32, 1000) as i64)`, checked; converts mm, mm/s and mm/s² content values |
| `add`, `sub` | raw `i64` checked add and sub |
| `mul_fx(a, b)` | `Fx(((a.raw as i128 * b.raw as i128) >> 32) as i64)`; the arithmetic shift is floor; checked narrowing |
| `div_fx(a, b)` | `Fx(floor_div((a.raw as i128) << 32, b.raw as i128) as i64)`; panics if `b.raw == 0`; for negative `b`, negate both operands first |
| `mul_bp_fx(x, bp)` | `Fx(mul_bp(x.raw, bp))` |
| `dist2(p, q) -> i128` | `Σ (p_i.raw - q_i.raw)²` over the three axes in `i128`. The result is raw Q64.64. Compare it against `(r.raw as i128)²`. Never narrow it. |
| `sqrt_fx(x)` | `Fx(isqrt_u128((x.raw as u128) << 32) as i64)`, floor; panics if `x.raw < 0` |
| `fx_floor_block(x)` | `(x.raw >> 32) as i32`, the block coordinate containing `x` |

`isqrt_u128` is the integer floor square root by Newton's method, specified in doc 02.

**Coordinate range.** Every `Fx` position, offset and direction in simulation state satisfies `|raw| < 2^62`, which keeps `dist2` and dot products inside `i128`. Folio coordinates are far inside this range.

### 1.5 Units and display units

- Units: meters, seconds, grams, milliliters, tenths of a degree Celsius, decibels. One block is 1 m on each side.
- **Display-unit rule.** Any quantity of a `Milli` resource written in this canon or any spec without an "m" prefix is in display units and is stored × 1000. Max HP 85 is stored as 85,000 mHP. A dodge cost of 20 stamina is 20,000. Composure −15 is −15,000. Death's Door sets HP to 1,000 mHP.
- `Fx` quantities shown to players, such as distances, display with `round_half_even` at the shown precision.
- `Milli` values display as `floor(x / 1000)`, except current HP, which displays as `ceil_div(x, 1000)` so a living entity never shows 0 HP. Percentages from `Bp` display with one decimal place via `round_half_even`.

### 1.6 Content numeric encoding

- Content files contain no floating-point numbers. A float in content is a schema error.
- Lengths are authored in integer millimeters, speeds in mm/s, accelerations in mm/s², angles in Bam, chances and multipliers in Bp, durations in ticks, masses in grams, volumes in ml, temperatures in DeciC.
- `u64` values such as seeds and hashes are written as `"0x"`-prefixed hexadecimal strings, because JSON tooling loses precision above 2^53.

### 1.7 Ranges

Ranges use Rust syntax. `a..=b` is inclusive and `a..b` is half-open.

### 1.8 Coordinates

- Right-handed. +Y is up. +X is east. −Z is forward (north), matching Bevy.
- Block coordinates are `i32` triples. Block `(bx, by, bz)` occupies the half-open cube `[bx, bx+1) × [by, by+1) × [bz, bz+1)`.
- A Folio's blocks occupy `x ∈ 0..=255`, `y ∈ 0..=63`, `z ∈ 0..=255`. Any block outside that box reads as `block.void_bedrock`, which is solid, opaque and unbreakable.
- Chunk coordinate `cx = floor_div(bx, 32)`, and likewise for y and z. Local coordinate `lx = bx - 32*cx`.
- Local block index inside a chunk: `index = lx + 32 * (lz + 32 * ly)`, range `0..=32767`.
- Entity positions are `Fx` triples in meters. The entity origin is the bottom-center of its collision capsule.

### 1.9 Angles

- Simulation yaw and pitch are `Bam`. Yaw 0 faces −Z. Yaw increases counter-clockwise seen from above.
- Pitch is the `i16` reinterpretation of its `Bam`, clamped to `-16000..=16000` (about ±87.9°). Positive pitch looks up.
- Bam arithmetic wraps (`wrapping_add`, `wrapping_sub`). `ang_diff(a, b) = (a.wrapping_sub(b) as i16).unsigned_abs()`, in `0..=32768`.
- Cone fields in content are half-angles in `0..=32768`.
- **Sine table.** `T` is a committed table of 16,385 `i32` values in Q2.30 for `i ∈ 0..=16384`: `T[i] = round_half_even(sin(2π·i/65536) · 2^30)`, so `T[16384] = 1073741824`. `tools/gen_trig.py` generates it once with `mpmath` at 50 digits, and the file is committed.
- `sin_bam(a)`: `q = a >> 14`, `r = a & 0x3FFF`. `q = 0 → T[r]`, `q = 1 → T[16384 - r]`, `q = 2 → -T[r]`, `q = 3 → -T[16384 - r]`.
- `cos_bam(a) = sin_bam(a.wrapping_add(16384))`.
- Forward vector in Q2.30: `fwd(yaw, pitch) = (((-sin_bam(yaw)) × cos_bam(pitch)) >> 30, sin_bam(pitch), ((-cos_bam(yaw)) × cos_bam(pitch)) >> 30)`, computed in `i64`. The negation happens before the multiply and the shift. `pitch` is the `i16` pitch reinterpreted as `Bam`; `fwd` does not clamp, because the simulation clamps pitch when it applies input.
- `atan2_bam(y, x)` uses 16-iteration CORDIC on `i64` with a committed arctangent table. Doc 02 specifies it. Its accuracy contract is ±1 Bam, which is sufficient for aim and facing.

### 1.10 Text keys

- Text keys are not content IDs. Grammar: `^loc\.[a-z0-9_.]+$`, maximum 160 bytes, no segment limit. They have no `id32`, never appear in saves or packets, and live only in localization tables (doc 18).
- Each content ID has `loc.<id>.name`, and optionally `loc.<id>.desc` and `loc.<id>.flavor`.
- A genre skin overrides with `loc.<id>.skin.<genre_suffix>.name`, where `genre_suffix` is the part after `genre.`. Example: `loc.arch.melee.blade_long.skin.wuxia.name = "Jian"`.
- Name resolution order: the local genre skin (§5.8), then the item's origin genre skin, then the archetype default name.

### 1.11 Entity IDs and item uids

- `EntityId = ((folio_index as u64) << 40) | counter`. `counter` starts at 1 at each Folio entry and is never reused within that Folio. `folio_index = 0` covers run start, the hub and anything created before Folio 1.
- Item `uid` uses the same layout with its own per-Folio counter, so items created in Folio 7 get the same uids whether or not Folios 1 to 6 were played.
- Entities and items that cross Folios keep their IDs.
- Tick-divider phases use `(EntityId & 0xFF_FFFF_FFFF) % period`.
- Every iteration that mutates simulation state, emits events, creates entities or draws random numbers runs in ascending `EntityId` order, or ascending `uid` for items.
- `HashMap` and `HashSet` are forbidden in `palimpsest_sim` state. Use `BTreeMap`, `BTreeSet` or sorted `Vec`.

---

## 2. Time, tick schedule, durations, sessions, replays and saves

### 2.1 Ticks and integration

- `TICK_HZ = 60`. One tick is 1/60 s.
- `DT` as `Fx` raw equals `71582788`, which is `floor(2^32 / 60)`. Only physics integration uses `DT`.
- Physics uses **semi-implicit Euler**: `v += a × DT`, then `p += v × DT`, each with `mul_fx`.
- Gameplay rates never multiply by `DT`. They use the accumulator rule in §2.3.
- Rendering interpolates between the last two simulation states. It never extrapolates simulation state, except the local player's client-side prediction (doc 16).

### 2.2 Tick schedule

A subsystem runs on tick `t` when `(t + phase) % period == 0`. Per-entity phases use the §1.11 phase key.

| Subsystem | Period (ticks) | Phase | Rate |
|---|---|---|---|
| Input apply, movement, collision, projectiles, melee traces | 1 | 0 | 60 Hz |
| Perception sight checks (per observer) | 3 | per-entity | 20 Hz |
| AI think (per agent) | 6 | per-entity | 10 Hz |
| Liquids cellular automaton | 6 | 1 | 10 Hz |
| Gases cellular automaton | 6 | 4 | 10 Hz |
| Fire spread | 12 | 2 | 5 Hz |
| Status pulse (DoT, HoT, buildup decay) | 15 | 0 | 4 Hz |
| Temperature diffusion | 30 | 5 | 2 Hz |
| Needs (satiation, toxicity decay, composure regeneration) | 60 | 7 | 1 Hz |
| Random block ticks | 1 | 0 | 8 random blocks per active chunk per tick, from `rng.blocktick` |
| Light relight | 1 | 0 | at most 4,096 cell updates per tick, FIFO |
| Sound propagation | on emit | same tick | event-driven |
| Network snapshot (doc 16) | 2 | 0 | 30 Hz |

Doc 03 owns the exact order of systems inside one tick, and the neighbor-visit order and budget semantics of every grid propagation.

### 2.3 Durations, rates and timers

- "N s" anywhere in the spec means exactly `N × 60` ticks.
- A status applied during tick `a` with duration `D` ticks is active on ticks `a+1 ..= a+D`. It is removed in the status-timer step at the end of tick `a+D`.
- A pulse at tick `t` affects only statuses active on `t`.
- Periodic sub-effects of a status fire on active ticks where `(t - a) % period == 0`.
- A refresh sets the expiry tick to `max(old_expiry, t + D)`.
- "After N s without X" means `t - last_X_tick >= 60 × N`. While a rate is suspended this way, its accumulator resets to 0.
- **Rate accumulator.** A per-second rate `R` (in `Milli` or whole units) applied by a subsystem with period `P` ticks uses an accumulator `acc` stored per (entity, rate), with denominator `k = 60 / P`:

```
total = R + acc
q     = floor_div(total, k)
acc   = total - k * q
apply q on this run
```

  Per-tick systems use `k = 60`. The status pulse uses `k = 4`. Needs use `k = 1`. No rate drifts.

- `in_combat(e, t)` is true when `e` dealt or took non-DoT damage within ticks `(t-300) ..= t`, or any hostile AI held awareness `alerted` toward `e` within that window.
- `light_at(e)` is the effective light of block `fx_floor_block(e.pos + (0, 0.5 m, 0))`. Stealth and composure use effective light. Spawn rules use `light_block` only (§12.3).

### 2.4 Sessions and replays

- Every save, replay and lobby handshake carries `sim_version: u32`, `content_hash: [u8; 32]` and `run_seed: u64`.
- `content_hash` is BLAKE3 over the canonical serialization of all loaded content sorted by content ID, including mods and the run's unlock set.
- Hot reload of content is refused while a run is active, except in dev builds.
- The replay log records the host's applied command per (tick, player slot), with slots `0..=3` for the party and slot 4 for an invader (§17.9): `{ tick: u64, slot: u8, move_x: i8, move_z: i8, yaw: u16, pitch: i16, buttons: u32, action_seq: u32, view_tick: u64 }`.
- Lag-compensated hit tests rewind to `view_tick`. Replays use the logged `view_tick`, so a replay reproduces exactly the hits the live host computed.

### 2.5 Saves and state hash

- A save is the canonical serialization of the entire `palimpsest_sim` state at a tick boundary, in sorted-key order, preceded by the §2.4 header. It includes every RNG state, every accumulator, buildup meters and timers, status expiry ticks, Death's Door flags, the Shock of the New set, EntityId and uid counters, the light relight FIFO, the liquid, gas, temperature and fire grids, AI state and the lattice history.
- `state_hash = BLAKE3(serialization)`. The canonical serialization is hand-written and self-delimiting (doc 02 §6.5); it does not depend on serde. Any other hash over several variable-length fields prefixes each field with its byte length as a little-endian `u32`.
- Debug builds may count draws per stream inside `palimpsest_sim` for diagnostics. Those counters never enter simulation state, saves or hashes. The host broadcasts `state_hash` every 60 ticks for desync detection (doc 16).
- Single player may save at any tick boundary. Co-op saves only at Folio entry.

---

## 3. Random number generation

### 3.1 Algorithms

All simulation randomness uses **xoshiro256++** seeded through **SplitMix64**. The project ships its own implementation. Reference:

```rust
/// SplitMix64 finalizer. Note: mix64(0) == 0.
pub fn mix64(mut z: u64) -> u64 {
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}

pub fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E3779B97F4A7C15);
    mix64(*state)
}

pub struct Rng { s: [u64; 4] }

impl Rng {
    pub fn from_seed64(seed: u64) -> Self {
        let mut sm = seed;
        Rng { s: [splitmix64(&mut sm), splitmix64(&mut sm), splitmix64(&mut sm), splitmix64(&mut sm)] }
    }
    pub fn next_u64(&mut self) -> u64 {
        let result = (self.s[0].wrapping_add(self.s[3])).rotate_left(23).wrapping_add(self.s[0]);
        let t = self.s[1] << 17;
        self.s[2] ^= self.s[0];
        self.s[3] ^= self.s[1];
        self.s[1] ^= self.s[2];
        self.s[0] ^= self.s[3];
        self.s[2] ^= t;
        self.s[3] = self.s[3].rotate_left(45);
        result
    }
    /// Uniform integer in [0, n). Lemire's method. May consume more than one u64 on rejection.
    pub fn range(&mut self, n: u64) -> u64 {
        assert!(n > 0);
        let mut m = (self.next_u64() as u128) * (n as u128);
        let mut l = m as u64;
        if l < n {
            let t = n.wrapping_neg() % n;
            while l < t {
                m = (self.next_u64() as u128) * (n as u128);
                l = m as u64;
            }
        }
        (m >> 64) as u64
    }
    /// Uniform integer in lo..=hi.
    pub fn range_incl(&mut self, lo: i64, hi: i64) -> i64 {
        assert!(lo <= hi && ((hi as i128) - (lo as i128)) < (u64::MAX as i128));
        lo + self.range((hi - lo + 1) as u64) as i64
    }
    /// Calls range(10000) exactly once regardless of chance_bp.
    pub fn roll_bp(&mut self, chance_bp: i32) -> bool {
        (self.range(10000) as i32) < chance_bp
    }
    /// Index chosen with probability proportional to weights. Calls range exactly once.
    pub fn weighted(&mut self, weights: &[u32]) -> usize {
        let total: u64 = weights.iter().map(|&w| w as u64).sum();
        assert!(total > 0);
        let mut r = self.range(total);
        for (i, &w) in weights.iter().enumerate() {
            if r < w as u64 { return i; }
            r -= w as u64;
        }
        unreachable!()
    }
    /// Fisher-Yates from the last index down.
    pub fn shuffle<T>(&mut self, v: &mut [T]) {
        for i in (1..v.len()).rev() {
            let j = self.range((i + 1) as u64) as usize;
            v.swap(i, j);
        }
    }
    /// Sum of three uniform draws in [0, n), a cheap bell curve.
    pub fn bell3(&mut self, n: u64) -> u64 { self.range(n) + self.range(n) + self.range(n) }
}
```

### 3.2 Sequential streams

Sequential streams serve host-only bulk processes whose draw order the host fully controls. A stream may be drawn only by the systems listed in its "Draws for" column.

```
stream_seed(run_seed, name, folio_index) =
    mix64(run_seed ^ mix64(fnv1a64(name) ^ mix64(folio_index as u64)))
rng = Rng::from_seed64(stream_seed(...))
```

Run-scoped streams use `folio_index = 0`. Folio indices start at 1. Per-Folio streams are derived fresh at Folio entry, so a Folio's generation depends only on the run seed, the Folio index and the Folio's genre inputs.

| Stream name | Scope | Owner | Draws for |
|---|---|---|---|
| `rng.lattice` | folio | 12 | genre candidates offered at the exit of that Folio: always exactly 3 candidates by one fixed procedure (§17.1) |
| `rng.appearance` | run | 10 | the run-start appearance shuffle (§13.6) |
| `rng.worldgen` | folio | 12 | layout, rooms, blocks, dressing |
| `rng.spawn` | folio | 13 | initial population and spawn waves |
| `rng.loot_place` | folio | 11 | contents of placed containers, shop stock, floor loot placed by worldgen |
| `rng.event` | folio | 12 | ambient and narrative events |
| `rng.ai_idle` | folio | 13 | idle and wander behavior with no actor input |
| `rng.fire` | folio | 03 | fire ignition and spread |
| `rng.blocktick` | folio | 03 | random block ticks |
| `rng.physics` | folio | 03 | debris and explosion block-break variation |

### 3.3 Keyed draws

Every random draw **caused by an entity's action** uses a keyed generator instead of a sequential stream. The key makes the draw independent of how many other draws happened that tick, so client prediction, late inputs under lag compensation, and players joining or leaving cannot shift it.

```rust
fn keyed_rng(stream: &str, folio: u32, actor: u64, tick: u64, seq: u32) -> Rng {
    Rng::from_seed64(mix64(
        stream_seed(run_seed, stream, folio as u64)
        ^ mix64(actor ^ mix64(tick ^ ((seq as u64) << 48)))
    ))
}
```

- `actor` is the acting entity's `EntityId`. `tick` is the input tick of the action. `seq` is a per-actor action counter that resets to 0 each tick.
- Loot drops use `actor` = the killed entity's `EntityId` or the container's uid, and `seq` = the receiving player's slot.

| Keyed stream | Owner | Used for |
|---|---|---|
| `rng.combat` | 05 | crits, spread and recoil jitter, Contradiction failure for every activation (docs 05, 06, 07), on-hit procs, Death's Door (04) |
| `rng.status` | 05 | only effects a doc explicitly declares as "X Bp chance to apply", and `st.confused` rotations; buildup itself never draws |
| `rng.magic` | 06 | wild surges, spell variance, cast interruption (§6.4 row 30) |
| `rng.stealth` | 09 | AI detection rolls, pickpocket, lockpicking, trap and secret detection (§6.4 row 35) |
| `rng.ident` | 10 | appraisal error, curse sensing on pickup (§6.4 row 36) |
| `rng.craft` | 07 | crafting quality |
| `rng.alchemy` | 08 | brewing variance, unknown liquid effects |
| `rng.loot` | 11 | drops from kills and opened containers, Reliquary contents, Fortune Engine reels |
| `rng.ai` | 13 | utility AI tie-breaks during an agent's think |
| `rng.progress` | 04 | talent offers and level-up choices, with `actor` = the player's EntityId |

Trap and secret detection (§6.4 row 35) rolls once per (observer, hidden object) pair on ticks where `(t + observer_phase) % 60 == 0`, with `seq` = the object's index in ascending ID order among objects in range.

### 3.4 Draw counts

A single game action draws the same number of values regardless of outcome wherever possible. Each hit draws exactly one crit roll, except sneak attacks, which draw none. Each projectile draws exactly two spread values. Any system with a variable draw count documents the count in its owning doc.

### 3.5 Commit-only offer streams

Offers a player can re-open, such as Scriptorium rewrite choices, reforge options and the faces shown on a Fortune Engine before a spin, come from the run-scoped sequential stream `rng.offer.p<slot>`, where `<slot>` is the player's party slot 0 to 3. The offer is generated once and stored. The stream advances only when the player commits. Re-opening a menu shows the stored offer and never rerolls, following Minecraft's enchanting seed.

### 3.6 Seeds

- A run seed is a `u64`. Its display form is 13 characters of Crockford Base32 in groups 4-4-5, for example `028T-5CY4-TQKFF`.
- Alphabet `0123456789ABCDEFGHJKMNPQRSTVWXYZ`, most-significant 5-bit group first. The first character carries only the top 4 bits, so it is `0` to `F`.
- Decoding strips `-` and ASCII space only, uppercases, maps `O → 0` and `I`, `L → 1`, then requires exactly 13 characters. Errors are checked in this order: length, then any `U`, then any character outside the alphabet, then a first character above `F`. Every other character is rejected.
- Daily seed: `fnv1a64("PALIMPSEST/DAILY/" + "YYYY-MM-DD")`, with the date in UTC.
- Weekly seed: `fnv1a64("PALIMPSEST/WEEKLY/" + "YYYY-Www")`, using the ISO week.
- Date strings are validated strictly: `YYYY-MM-DD` must be a real calendar date with year `0001..=9999`, and `YYYY-Www` must use an uppercase `W` and a week number no higher than that ISO year's week count. Invalid strings are rejected, never hashed.
- Paid gacha never uses this generator. Doc 15 specifies a server-side ChaCha20 generator with commit-reveal.

### 3.7 Mods

Mod scripts draw only from keyed or sequential streams named `rng.mod.<mod_id>.<name>`. WASM mod hooks run with NaN canonicalization enabled and fuel limits (doc 02).

---
## 4. World and movement constants

All values are given in content units (§1.6). Code converts them with `fx_from_milli`.

| Constant | Value | Notes |
|---|---|---|
| `BLOCK_SIZE` | 1000 mm | |
| `CHUNK_SIZE` | 32 blocks per axis | 32,768 blocks per chunk |
| `FOLIO_EXTENT` | 256 × 64 × 256 blocks (X × Y × Z) | 8 × 2 × 8 chunks; §1.8 |
| `FOLIO_TYPICAL_FOOTPRINT` | 96 to 192 blocks per horizontal side | doc 12 chooses per genre |
| `GRAVITY_BASE` | 20000 mm/s² | game-feel gravity; each genre multiplies by its `gravity_bp` |
| `TERMINAL_VELOCITY` | 50000 mm/s | |
| `ACTIVE_ACTOR_CAP` | 256 per Folio | new AI spawns beyond the cap queue FIFO |
| `PROJECTILE_CAP` | 1024 per Folio | at the cap, remove the lowest-EntityId non-player projectile; if all are player-owned, remove the lowest EntityId |
| `GROUND_ITEM_CAP` | 2048 per Folio | beyond the cap, items merge into a Lost Satchel (doc 11) |

### 4.1 Humanoid player body

| Parameter | Value |
|---|---|
| Capsule radius | 300 mm |
| Standing height | 1800 mm |
| Standing eye height | 1620 mm |
| Crouched height | 1200 mm |
| Crouched eye height | 1020 mm |
| Step-up height | 550 mm |
| Body mass | 75,000 g, plus carried mass |

### 4.2 Movement

| Movement | Speed (mm/s) |
|---|---|
| Walk | 4300 |
| Sprint | 6500 |
| Crouch walk | 2000 |
| Swim | 2400 |
| Climb (ladders, vines, rope) | 2400 |
| Backpedal | walk × 8000 Bp |
| Strafe | walk × 9500 Bp |

- Ground acceleration is 40000 mm/s². Air acceleration is 8000 mm/s².
- Backpedal applies when the angle between the input direction and facing exceeds 12288 Bam (67.5°). Strafe applies above 4096 Bam (22.5°). Otherwise the movement is forward.
- **Jump.** `JUMP_H = 1100 + 30 × min(vrv(e), 5) + 150 × max(0, vrv(e) - 5)` mm: 1100, 1250, 1400, 1550, 1700, 1850 and 2000 mm at VERVE 0, 5, 6, 7, 8, 9 and 10. Launch speed is `v0 = sqrt_fx(2 × g × JUMP_H) + g × DT / 2`, where `g` is the local gravity. With the §2.1 integrator the apex equals `JUMP_H` within 1 mm for `g` from 12000 to 20000 mm/s². Every genre can mount a 1-block ledge, and Wuxia clears 2-block walls.
- **Air jump.** At `vrv(e) >= 9`, every entity gets one air jump per airborne period, with height `mul_bp(JUMP_H, 6000)`. This is genre physics, not a component, and never Contradicts.
- **Lean.** `move.lean_l` and `move.lean_r` shift the eye 350 mm sideways over 9 ticks. The capsule stays put and the head hitbox follows the eye (doc 05).
- **Fall damage.** `fall_height = max(0, peak - land)`, measured along the current gravity axis. `peak` is the extreme origin coordinate against gravity since the entity last stood on ground, swam, climbed or had `st.levitating`. `SAFE_FALL = 4000 mm × (10000 + (vrv(e) - 5) × 400) Bp`. If `excess = fall_height - SAFE_FALL > 0`, damage is `mul_bp(max_hp_mhp, ((excess.raw as i128 × 800) >> 32) as i32)` mHP, which is 8% of Max HP per meter of excess. It is a `dmg.blunt` hit entering the damage pipeline at step 4 (§10.5). It skips armor and the chip floor and never crits. Acrobatics (doc 04) multiplies it afterwards.
- **Dodge.** A 3000 mm burst over 12 ticks costing 20 stamina. Invulnerability starts on the dodge's first tick and lasts `min(12, 8 + floor_div(max(0, AGI - 40), 15))` ticks, never past the dodge's 12th tick. A new dodge cannot start until 18 ticks after the previous one started.
- **Final move speed** is base speed, then §6.4 row 11, then each active status multiplier in ascending status ID byte order. Encumbrance is applied only through `st.encumbered` and `st.overburdened`.

### 4.3 Terrain and placeables

Building is limited (DECISIONS.md, item 16).

- **Terrain is fixed.** Worldgen blocks are indestructible unless their block definition carries the tag `destructible`. Destructible blocks are doors, crates, barrels, cracked walls that hide secrets, wooden structures, glass, ice, webs and every Errata placeable. Explosions, melee and projectiles damage only destructible blocks.
- **Placeables.** Errata place only `cat.placeable` items: barricades, bridge spans, ladders, traps, light sources and deployable cover. Doc 07 owns placement rules, anchors, block HP and costs. Placing takes ticks scaled by `skill.fieldcraft`.
- **Resource nodes.** Materials come from resource nodes (ore veins, plants, salvage piles) that are interactable props, not from digging terrain.
- Liquids, gases, fire, light and sound still simulate on the block grid (doc 03).

---

## 5. Axioms, genres, Contradiction and Anachronism

### 5.1 Axioms

| ID | Name | Governs |
|---|---|---|
| `ax.arcana` | ARCANA | how well magic works: spells, wands, enchantments, magical effects |
| `ax.techne` | TECHNE | how well technology works: firearms, energy weapons, gadgets, implants, electronics |
| `ax.mythos` | MYTHOS | how strongly the supernatural operates: gods, prayer, relics, curses, undead, fiends, the uncanny |
| `ax.verve` | VERVE | how cinematic reality is: crits, acrobatics, jump height, lucky escapes |

Each axiom is an integer `0..=10` per genre.

### 5.2 Genres

The suffix after `genre.` is the **genre suffix** used in text keys. `gravity_bp` multiplies `GRAVITY_BASE`. `ambient_light` is the simulation light floor in open areas before light sources (doc 03). Doc 12 owns the full Genre Parameter Vector for each genre and must copy these columns exactly.

`aether_axiom` is the axiom that powers native casting in that genre (§5.3, §5.7).

| # | ID | Display name | ARC | TEC | MYT | VRV | aether_axiom | gravity_bp | ambient_light | Lucre skin | Aether skin | Tier |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | `genre.mythic_fantasy` | Mythic Fantasy | 8 | 2 | 6 | 6 | arcana | 10000 | 4 | Crowns | Mana | core |
| 2 | `genre.gothic_horror` | Gothic Horror | 5 | 3 | 9 | 3 | mythos | 10000 | 2 | Shillings | Sanguine | core |
| 3 | `genre.cosmic_horror` | Cosmic Horror | 3 | 6 | 10 | 1 | mythos | 10000 | 1 | Bonds | Gnosis | core |
| 4 | `genre.space_opera` | Space Opera | 6 | 10 | 3 | 7 | arcana | 8000 | 6 | Credits | Psi | core |
| 5 | `genre.cyberpunk` | Cyberpunk | 1 | 9 | 2 | 5 | techne | 10000 | 3 | Chits | Bandwidth | core |
| 6 | `genre.post_apocalypse` | Post-Apocalypse | 0 | 6 | 5 | 6 | techne | 10000 | 5 | Tins | Static | core |
| 7 | `genre.weird_west` | Weird West | 4 | 4 | 6 | 7 | mythos | 10000 | 6 | Eagles | Hex | core |
| 8 | `genre.hardboiled_noir` | Hardboiled Noir | 0 | 5 | 1 | 2 | verve | 10000 | 2 | Bucks | Nerve | core |
| 9 | `genre.wuxia` | Wuxia | 6 | 2 | 5 | 10 | verve | 8000 | 5 | Taels | Qi | core |
| 10 | `genre.steampunk` | Steampunk | 3 | 7 | 3 | 6 | techne | 10000 | 4 | Sovereigns | Pressure | core |
| 11 | `genre.fairy_tale` | Fairy Tale | 10 | 1 | 8 | 3 | arcana | 10000 | 6 | Fairy Gold | Glamour | core |
| 12 | `genre.mythic_antiquity` | Mythic Antiquity | 6 | 2 | 10 | 8 | mythos | 10000 | 7 | Drachmae | Ichor | core |
| 13 | `genre.oneiric` | Oneiric | 9 | 5 | 8 | 9 | arcana | 6000 | 4 | Motes | Lucidity | secret |
| 14 | `genre.redacted` | The Redaction | 2 | 8 | 8 | 0 | techne | 10000 | 9 | Vouchers | Clearance | secret |
| 15 | `genre.unwritten` | The Unwritten | 0 | 0 | 0 | 0 | none | 10000 | 8 | Lucre | Potential | secret |
| 16 | `genre.ur_text` | The Ur-Text | 5 | 5 | 5 | 5 | none | 10000 | 4 | Words | Logos | secret |

Special rules:

- `genre.ur_text` is the genre of Folio 25 (§17).
- `genre.unwritten` has no shops and no Lucre drops. Its only enemies are members of `fac.editors`, who count as native there, and every ability of a `fac.editors` member counts as innate for the §5.5 override.
- `genre.oneiric` has gravity-flip volumes, which doc 12 defines.
- `genre.redacted` is the Editors' home genre.
- In any genre whose VERVE is 0 nobody crits (§6.4 row 13).
- The Contradiction overrides for these genres are in §5.5.
- **Naming rule.** No Lucre or Aether skin may match a real-world brand, a third-party game's currency or resource name, the display name of any `cur.` or `ent.` entry, or the word Ink. Doc 18 runs a name review against this rule.
- The minimum pairwise L1 distance between the axiom vectors of the 12 core genres is 6.

### 5.3 Dependencies and requirements

Every **component** of an item or ability has a dependency set `dep ⊆ {arcana, techne, mythos, verve}`. A component is the item's base function, one affix, one perk, one spell glyph, one augment or one innate ability. Dependencies come from the **genre skin**, not from the archetype, because the skin says how the thing works in its world. A longbow skin of `arch.ranged.charge_rifle` is mundane. A railgun skin of the same archetype depends on TECHNE. A lightning-staff skin depends on ARCANA.

| Component kind | Default `dep` |
|---|---|
| Mundane melee base, mundane armor base, bows, slings, thrown blades, food, raw materials, blocks | none |
| Firearms, energy weapons, gadgets, drones, turrets, implants, electronic tools, hacking, tech consumables | techne |
| Spell glyphs, wands, staves, magic texts | `{origin_genre.aether_axiom}`; `{arcana}` when the origin's aether_axiom is none |
| Enchantment affixes, grafts, supernatural alchemical effects | arcana |
| Prayers, relics, holy or unholy affixes, curse effects, undead-bane, divine favors, mutations of supernatural origin | mythos |
| Cinematic techniques: wall-run, double jump, qinggong glide, gun-kata, "rule of cool" exotic perks | verve |
| Mundane alchemical effects: healing, poison, stamina, acid, fire, smoke | none |
| Kin passive traits, tamed beasts, beast mounts | none |
| Summons | the summoning glyph's dependency |
| Mounts other than beasts | from the skin: vehicle techne, broom or carpet arcana, divine steed mythos, cloud verve |

A component's **requirement** on axis `a`:

```
Req[a] = if a ∈ dep { max(1, origin_genre.axiom[a] - 2) } else { 0 }
```

`origin_genre` is the genre in which the item was generated or last Rewritten. Innate abilities of a creature use the creature's origin genre. Worldgen and loot never roll a component with `a ∈ dep` in a genre whose `axiom[a] = 0`.

### 5.4 Tolerance

A wielder's tolerance on axis `a` is the sum below, capped at 6:

```
Tol[a] = origin_tol[a] + attune_tol + gear_tol[a] + talent_tol[a]
attune_tol = clamp(floor_div(ATN - 40, 30), 0, 3)
```

`origin_tol` comes from the character's Origin genre (§16). Sort that genre's four axioms in descending order, breaking ties in the order ARCANA, TECHNE, MYTHOS, VERVE. The first gets 2, the second and third get 1, the fourth gets 0. Every Origin totals 4.

| Origin | ARC | TEC | MYT | VRV |
|---|---|---|---|---|
| `genre.mythic_fantasy` | 2 | 0 | 1 | 1 |
| `genre.gothic_horror` | 1 | 1 | 2 | 0 |
| `genre.cosmic_horror` | 1 | 1 | 2 | 0 |
| `genre.space_opera` | 1 | 2 | 0 | 1 |
| `genre.cyberpunk` | 0 | 2 | 1 | 1 |
| `genre.post_apocalypse` | 0 | 2 | 1 | 1 |
| `genre.weird_west` | 1 | 0 | 1 | 2 |
| `genre.hardboiled_noir` | 0 | 2 | 1 | 1 |
| `genre.wuxia` | 1 | 0 | 1 | 2 |
| `genre.steampunk` | 1 | 2 | 0 | 1 |
| `genre.fairy_tale` | 2 | 0 | 1 | 1 |
| `genre.mythic_antiquity` | 1 | 0 | 2 | 1 |
| `genre.oneiric` | 2 | 0 | 1 | 1 |
| `genre.redacted` | 1 | 2 | 1 | 0 |
| `genre.unwritten` | 2 | 1 | 1 | 0 |
| `genre.ur_text` | 2 | 1 | 1 | 0 |

NPC creatures have `origin_tol = 2` on every axis of their own origin genre where that genre's axiom is at least 5, and `attune_tol = 0`.

### 5.5 Contradiction

**Overrides**, checked before the formula:

- In `genre.ur_text`, every component has `C = 0` and `E = 0`.
- In `genre.unwritten`, innate abilities (body functions, Origin traits, Echo traits) have `C = 0` and `E = 0`.
- Palimpsestic items have `C = 0` everywhere and never receive Anachronism Edge.

Otherwise, for each component, using the local axioms at the wielder's position (§5.8):

```
raw_gap[a] = max(0, Req[a] - local_axiom[a])
gap[a]     = max(0, raw_gap[a] - Tol[a])
C          = max over a of gap[a]      (0 if dep is empty)
```

| C | State | Potency (Bp) | Failure chance (Bp) |
|---|---|---|---|
| 0 | Coherent | 10000 | 0 |
| 1 | Strained | 9000 | 0 |
| 2 | Strained | 8000 | 0 |
| 3 | Contradicted | 7000 | 500 |
| 4 | Contradicted | 6000 | 1000 |
| 5 | Contradicted | 5000 | 1500 |
| 6 or more | Disconnected | 0 | inert |

- **Re-evaluation.** Contradiction is recomputed on Folio entry, on the tick the wielder's local genre changes, when the wielder changes, and when tolerance changes. Each change emits `ev.contradiction_changed`.
- **Potency** multiplies everything the component produces: damage, healing, magnitude, duration, buildup, ward, armor rating, poise and resistances it grants.
- **Per-component scope.** An affix, perk or augment contributes `mul_bp(value, its own potency)`, and contributes 0 when Disconnected. The damage pipeline uses the base-function component's potency (§10.5 step 2.5).
- **Failure** is rolled only for the base-function component, only when its failure chance is above 0, with keyed `rng.combat`. Single-shot, semi-automatic, cast and use actions roll once per activation. Weapons with a fire interval under 30 ticks roll on the first shot of each trigger hold and then once per 30 ticks of continuous fire. Doc 05 defines a failed shot as a jam. Doc 06 defines a failed spell as a fizzle that spends full aether. Doc 07 defines a failed gadget as a spark dealing 5% of the user's Max HP as `dmg.shock`.
- **Disconnected** components cannot activate and their passive bonuses are off. The item shows a torn-page icon.
- Contradiction applies to native and foreign creatures alike. A foreign enemy that arrives through genre bleed carries its own Contradiction.

### 5.6 Anachronism Edge

```
E = if C <= 5 { min(6, max over a in dep of raw_gap[a]) } else { 0 }
edge_bp = E × 400
```

- A target is **native** when `target.origin_genre == local_genre(target.pos)`.
- Against a native target, the base-function component's damage and status buildup are multiplied by `10000 + edge_bp`, up to +24%.
- **Shock of the New:** the first hit from a base-function component with `E >= 3` on each native non-boss target applies `st.staggered` regardless of poise. The simulation tracks this in a `BTreeSet<(item uid, target EntityId)>` that is cleared at Folio entry.

**Worked example.** A Cyberpunk smart pistol has `dep = {techne}` and `Req[techne] = max(1, 9 - 2) = 7`. It is carried into Fairy Tale, where TECHNE is 1, so `raw_gap = 6`. The wielder has a Cyberpunk Origin, giving `origin_tol[techne] = 2`, and ATN 60, giving `attune_tol = floor_div(20, 30) = 0`. So `Tol = 2` and `gap = 4`. The state is Contradicted: potency 6000 and failure 1000. `E = min(6, 6) = 6`, so `edge_bp = 2400`. Against a native wolf the damage factor is `0.60 × 1.24 = 0.744`, with a 10% jam chance and a guaranteed first-hit stagger.

### 5.7 Ambient axiom effects

These terms appear once each, in the formula listed. They are not applied a second time.

| Axiom | Effect | Applied in |
|---|---|---|
| AETHER | Aether regeneration × `(10000 + (local_axiom[aa] - 5) × 1000)` Bp, where `aa` is the local genre's aether_axiom; × 10000 when it is none | §6.4 row 8 |
| GLYPH | Each spell glyph's potency × `(10000 + (local_axiom[a] - 5) × 200)` Bp, where `a` is the glyph's single dependency axis; × 10000 if its dependency is empty | §6.4 row 29, per glyph |
| TECHNE | Energy-weapon and gadget recharge × `(10000 + (TEC - 5) × 1000)` Bp | doc 05, doc 07 |
| TECHNE | At TEC ≤ 2, worldgen places no electronic locks, cameras or turrets | doc 12 |
| MYTHOS | Damage × `(10000 + (MYT - 5) × 300)` Bp when the attacker or the target is tagged undead, fiend, divine or eldritch; applied once even if both are | §10.5 step 2.7 |
| MYTHOS | Curse chance on generated items × `(10000 + (MYT - 5) × 1500)` Bp | doc 10 |
| MYTHOS | Composure loss × `(10000 + (MYT - 5) × 1000)` Bp | doc 04 |
| VERVE | Crit chance term `(vrv(e) - 5) × 100` Bp | §6.4 row 13 |
| VERVE | Jump height and safe-fall height | §4.2 |
| VERVE | Death's Door chance × `(10000 + (vrv(e) - 5) × 1000)` Bp | §6.4 row 43 |

All axiom values here are local axioms at the relevant entity's position (§5.8).

### 5.8 Local genre and personal VERVE

- `local_genre(pos)` is the genre of the bleed zone containing block `fx_floor_block(pos)`, or the Folio's genre outside bleed zones. Doc 12 defines bleed zones.
- `local_axiom(pos, a)` is the axiom value of `local_genre(pos)`.
- §5.5 to §5.7 always use local values at the wielder's position, except that "native" uses the target's position.
- **Seam zones.** Doc 12 may place at most one `zone.seam` per Folio, covering 1 or 2 rooms, where two genres are stitched together. Inside a seam zone, `local_axiom(pos, a)` is the larger of the two genres' axiom values for each `a`, both genres' World Laws (§5.10) are active, entities of either genre count as native, and the Folio's genre supplies skins. This is the only place where both toolsets work at full strength.
- `vrv(e) = min(10, local_axiom(e.pos, verve) + 2 × [e has st.inspired])`. Jump, safe fall, crit chance and Death's Door use `vrv(e)`. Contradiction requirements always use the unmodified local axiom.

### 5.9 Genre identity

Doc 12 copies this table into the full Genre Parameter Vectors. Doc 07 defines every listed material. Doc 11 owns manufacturer mechanics and adds one more manufacturer per core genre plus the Errata manufacturer. Physics columns multiply the base constants of §4: `air_accel_bp` multiplies air acceleration, `fall_dmg_bp` multiplies fall damage, `ground_friction_bp` multiplies ground deceleration. `spawn_block_max` is the highest block-light level in which ambient and wave spawns may occur (§12.3).

| Genre | air_accel_bp | fall_dmg_bp | ground_friction_bp | spawn_block_max | Palette | Music register | Enemy themes | Materials |
|---|---|---|---|---|---|---|---|---|
| mythic_fantasy | 10000 | 10000 | 10000 | 0 | `#3B6E3B #C9A227 #8FB3D9` | horns, strings, choir | orcs, dragons, wyverns | `mat.steel` `mat.moonsilver` `mat.oak` `mat.dragonbone` |
| gothic_horror | 10000 | 10000 | 10000 | 7 | `#3A1F2B #7A0E14 #C8C3B4` | organ, string quartet, harpsichord | vampires, werewolves, ghouls | `mat.silver` `mat.blackwood` `mat.bone` |
| cosmic_horror | 8000 | 12500 | 10000 | 7 | `#0E2A2E #4F7F6A #B8E986` | drones, detuned strings, theremin | cultists, deep things, formless spawn | `mat.obsidian` `mat.star_metal` `mat.chitin` |
| space_opera | 12500 | 7500 | 10000 | 0 | `#0B1026 #3D7BFF #FF8A3D` | synth-orchestral, brass | boarders, xeno hives, war droids | `mat.plasteel` `mat.titanium` `mat.psi_crystal` |
| cyberpunk | 11000 | 10000 | 10000 | 0 | `#120B1F #FF2E88 #00E5FF` | darksynth, industrial | gangers, corporate security, rogue AIs | `mat.carbon_fiber` `mat.polymer` `mat.nanoweave` |
| post_apocalypse | 10000 | 10000 | 9000 | 0 | `#6B4E2E #C2A36B #E04E1B` | distorted guitar, junk percussion | raiders, mutants, scrap golems, rad-cults | `mat.scrap_iron` `mat.rubber` `mat.rad_glass` |
| weird_west | 11000 | 10000 | 10000 | 0 | `#8C4A2F #E0B776 #2F3E4E` | slide guitar, whistle | outlaws, revenants, sand wyrms | `mat.iron` `mat.leather` `mat.hexed_silver` |
| hardboiled_noir | 8000 | 12500 | 8500 | 0 | `#101010 #6E6E6E #C22B2B` | smoky jazz, muted trumpet | mobsters, crooked cops, hired killers | `mat.steel` `mat.brass` `mat.tweed` |
| wuxia | 20000 | 5000 | 10000 | 0 | `#1F3A2E #D9C9A3 #B32020` | guzheng, dizi, erhu, taiko | rival sects, jiangshi, fox spirits | `mat.jade` `mat.bronze` `mat.silk` |
| steampunk | 10000 | 10000 | 10000 | 0 | `#4A3A28 #B5793A #5FA8A0` | brass band, calliope | automata, sky pirates, clockwork beasts | `mat.brass` `mat.copper` `mat.aetherium` |
| fairy_tale | 12500 | 7500 | 10000 | 0 | `#F2D7E8 #6FBF73 #E8C547` | celesta, woodwinds, waltz | wolves, witches, trolls | `mat.cold_iron` `mat.glass` `mat.elderwood` |
| mythic_antiquity | 12500 | 10000 | 10000 | 0 | `#F0E6D2 #2E5A88 #C9792B` | lyre, aulos, chant | shades, hydras, harpies | `mat.bronze` `mat.orichalcum` `mat.marble` |
| oneiric | 15000 | 2500 | 10000 | 0 | `#2B1B4D #8E7CFF #FFC2E0` | reversed piano, choir pads | nightmares, doppelgangers, sleepwalkers | `mat.dreamstuff` |
| redacted | 6000 | 15000 | 10000 | 15 | `#F2F2F2 #1A1A1A #D7262E` | office hum, sine tones | agents, anomalies, containment drones | `mat.redacted_alloy` |
| unwritten | 10000 | 10000 | 10000 | 15 | `#FFFFFF #E6E6E6 #CCCCCC` | silence | Editors only | `mat.vellum` |
| ur_text | 10000 | 10000 | 10000 | 0 | `#1C1408 #E8D9B0 #B8862B` | every register in counterpoint | every roster | `mat.vellum` `mat.gold_leaf` |

Core manufacturers (doc 11 owns the full mechanics):

| Genre | Manufacturer IDs and signature trait |
|---|---|
| mythic_fantasy | `mfr.mythic_fantasy.deepanvil`: poise damage × 15000, attack speed × 9000. `mfr.mythic_fantasy.silverbough`: attack noise −20 dB. |
| gothic_horror | `mfr.gothic_horror.vesper_sons`: 2000 Bp of each hit converted to `dmg.radiant`. `mfr.gothic_horror.grimhallow`: instant reload for 3% Max HP. |
| cosmic_horror | `mfr.cosmic_horror.hollow_lantern`: hits apply `st.marked` for 4 s. `mfr.cosmic_horror.outer_choir`: 2500 Bp `dmg.psychic` share, +1500 Bp damage, costs 1 composure per shot. |
| space_opera | `mfr.space_opera.perihelion`: a 30-tick charged shot deals × 15000. `mfr.space_opera.umbra`: no magazine; heat; overheating locks the weapon for 90 ticks. |
| cyberpunk | `mfr.cyberpunk.ghostline`: projectiles home on `st.marked` targets at 16384 Bam/s. `mfr.cyberpunk.neon_saint`: charged shots pierce 1000 mm of blocks. |
| post_apocalypse | `mfr.post_apocalypse.rustmaw`: magazine × 20000, durability × 5000. `mfr.post_apocalypse.lastditch`: 2000 Bp chance per shot to use no ammo. |
| weird_west | `mfr.weird_west.coffin_creek`: crits ricochet 5000 Bp of their damage to the nearest other hostile within 8 m. `mfr.weird_west.seven_seals`: 2000 Bp `dmg.umbral` share with dependency mythos. |
| hardboiled_noir | `mfr.hardboiled_noir.midnight_avenue`: shot noise capped at 50 dB. `mfr.hardboiled_noir.rainslick`: × 12000 against staggered or knocked-down targets. |
| wuxia | `mfr.wuxia.jade_lotus`: thrown weapons return after 30 ticks. `mfr.wuxia.iron_crane`: a block within 6 ticks of impact reflects 5000 Bp of the damage. |
| steampunk | `mfr.steampunk.brassworth`: +200 Bp per consecutive hit, maximum +3000. `mfr.steampunk.aetherline`: every 4th hit chains 3000 Bp of its damage as `dmg.shock` to 2 targets within 5 m. |
| fairy_tale | `mfr.fairy_tale.thimbleweald`: × 13000 against targets with a larger capsule radius than the wielder. `mfr.fairy_tale.thornwood`: 1000 Bp chance per hit to apply `st.rooted`. |
| mythic_antiquity | `mfr.mythic_antiquity.hephaesteum`: no durability loss; immune to `st.disarmed`. `mfr.mythic_antiquity.argive`: +1500 Bp damage while an allied Erratum is within 3 m. |
| oneiric | `mfr.oneiric.somnambule`: projectiles pass through blocks after 10 m of flight. |
| redacted | `mfr.redacted.bureau`: never rolls Contradiction failure; the potency loss still applies. |
| ur_text | `mfr.ur_text.first_press`: Palimpsestic items only. |

### 5.10 World Laws

Each genre has 1 to 3 **World Laws** (`law.<genre_suffix>.<name>`, owned by doc 12), following Torg's World Laws. A Law is active wherever the local genre is its genre, and in seam zones that include its genre.

- A component may carry `law_bound: <law ID>`. If that Law is not active at the wielder's position, the component has `C = 6` (Disconnected), whatever the axioms say. This keeps genre signature verbs, such as qinggong or the High Noon duel, native to their genre without inflating axioms.
- A Law may carry `law_grants: [{component_class, axis, tol_bonus}]`. Each grant adds to `Tol[axis]` for components of that class before the cap of 6. Example: Steampunk's weird-science Law grants +2 TECHNE tolerance to gadgets.
- At most one Law per genre changes a canon constant. Every other Law is additive. Doc 12 lists every Law with exact effects. Required Laws include `law.wuxia.qinggong`, `law.weird_west.high_noon`, `law.steampunk.weird_science`, `law.gothic_horror.dread_escalation`, `law.cosmic_horror.dread_escalation`, `law.redacted.dread_escalation` and `law.hardboiled_noir.void_rules`.

### 5.11 Going Native and being Unmoored

- **Going Native.** Once per run, at a Colophon (§17.1), an Erratum may permanently set its Origin to the current Folio's genre. Origin tolerance is recomputed by §5.4 and a cosmetic native form is applied. Gear is not translated.
- **Unmoored.** Three Contradiction failures (jams, fizzles, sparks) by one character within 600 ticks apply `st.unmoored` for 300 ticks. While it lasts, Fortune-derived effects (Death's Door, Serendipity procs) are off, and Strained components use the C = 3 row of §5.5. It clears early on entering a Colophon or a seam zone.

---
## 6. Attributes and sub-attributes

### 6.1 Structure

Eight **primary attributes** each have three **sub-attributes**. Sub-attributes are stored. A primary is derived as `floor_div(s1 + s2 + s3, 3)` using effective sub-attribute values.

| Primary ID | Code | Sub-attribute IDs (codes) |
|---|---|---|
| `attr.might` | MGT | `sub.power` (PWR), `sub.brawn` (BRN), `sub.grip` (GRP) |
| `attr.finesse` | FIN | `sub.precision` (PRC), `sub.agility` (AGI), `sub.sleight` (SLT) |
| `attr.vigor` | VIG | `sub.toughness` (TGH), `sub.endurance` (END), `sub.resilience` (RES) |
| `attr.intellect` | INT | `sub.logic` (LOG), `sub.memory` (MEM), `sub.analysis` (ANL) |
| `attr.will` | WIL | `sub.focus` (FOC), `sub.resolve` (RSV), `sub.attunement` (ATN) |
| `attr.perception` | PER | `sub.sight` (SGT), `sub.hearing` (HRG), `sub.intuition` (ITU) |
| `attr.presence` | PRE | `sub.charm` (CHM), `sub.menace` (MEN), `sub.guile` (GUI) |
| `attr.fortune` | FOR | `sub.fate` (FAT), `sub.serendipity` (SER), `sub.providence` (PRV) |

### 6.2 Scale

- Stored (base) sub-attribute: integer `1..=100`.
- Effective sub-attribute: base plus modifiers, clamped to `1..=150`.
- Human baseline: 40. New characters start in the range `20..=60` per sub-attribute (doc 04 owns creation).
- Every formula below uses the effective value. `d(x) = x - 40`.

### 6.3 Requirements

Items may require a primary attribute, for example `req: { attr.might: 55 }`. Shortfall `s = max(0, req - value)`. While wielding with a shortfall, damage (§10.5 step 2.8), attack speed and spread recovery are each multiplied by `10000 - min(5000, s × 100)` Bp. This follows Shattered Pixel Dungeon's strength requirement.

### 6.4 Master derived-stat formulas

All doc-04-owned growth feeds these formulas, and no other document defines a competing formula for the same stat. `L` is character level. Values are effective sub-attributes. Per-second rates use the §2.3 accumulator.

| # | Stat | Formula | At all-40, L1, VERVE 5 |
|---|---|---|---|
| 1 | Max HP | `40 + TGH + L × (3 + floor_div(TGH, 20))` HP, plus `gear_hp` (§13.8.2) | 85 |
| 2 | Natural HP regeneration, after 5 s without taking damage | `mul_bp(max_hp_mhp, max(10, 50 + floor_div(d(RES), 2)))` mHP per s | 425 mHP/s (0.5% per s) |
| 3 | Healing received | × `10000 + d(RES) × 50` Bp | ×1.00 |
| 4 | Max stamina | `50 + END + floor_div(L, 2)` | 90 |
| 5 | Stamina regeneration, after 1 s without spending | `15 + floor_div(END, 5)` per s | 23 per s |
| 6 | Sprint stamina drain | `max(4, 12 - floor_div(d(END), 10))` per s | 12 per s |
| 7 | Max aether | `20 + FOC + L` | 61 |
| 8 | Aether regeneration | `500 + ATN × 20` m-aether per s, then × the AETHER term of §5.7 | 1.3 per s at aether axiom 5 |
| 9 | Max composure | `50 + RSV` | 90 |
| 10 | Carry capacity | `25 + floor_div(PWR, 2)` kg | 45 kg |
| 11 | Move speed multiplier | `clamp(10000 + d(AGI) × 25, 8000, 12500)` Bp | ×1.00 |
| 12 | Weapon scaling bonus | `max(-5000, Σ floor_div(c_i × d(x_i), 100) + governing_skill × 30)` Bp over the weapon's scaling coefficients `c_i` on sub-attributes `x_i`. Each `c_i ∈ 0..=10000` and `Σ c_i ≤ 15000`, validated by schema. Spells never use this row. | +0% at skill 0 |
| 13 | Crit chance | If `local_axiom(e.pos, verve) = 0`, exactly 0. Otherwise `clamp(500 + d(FAT) × 10 + d(PRC) × 5 + (vrv(e) - 5) × 100 + gear, 0, 7500)` Bp | 5.0% |
| 14 | Crit damage multiplier | `15000 + gear` Bp | ×1.50 |
| 15 | Spread multiplier | `clamp(10000 - d(PRC) × 40, 5000, 14000)` Bp | ×1.00 |
| 16 | Recoil multiplier | `clamp(10000 - d(GRP) × 40, 5000, 14000)` Bp | ×1.00 |
| 17 | Reload, swap and use-item speed | `clamp(10000 + d(SLT) × 50, 7000, 15000)` Bp; `action_ticks = max(1, ceil_div(base_ticks × 10000, mult))` | ×1.00 |
| 18 | Poise | `20 + floor_div(BRN, 2)` + armor poise | 40 |
| 19 | Block stability: the share of blocked damage turned into stamina loss instead of HP loss | `clamp(5000 + d(BRN) × 50 + shield stability, 0, 10000)` Bp. Stamina loss is normalized by depth (§10.7). | 50% |
| 20 | Throw range multiplier | `10000 + d(PWR) × 50` Bp | ×1.00 |
| 21 | Climb speed multiplier | `10000 + d(GRP) × 50` Bp | ×1.00 |
| 22 | Body-status threshold: poisoned, diseased, irradiated, withering, corroded, exposed | `max(40, 100 + 2 × d(RES))` | 100 |
| 23 | Nerve-status threshold: bleeding, burning, frozen, shocked, stunned, blinded, deafened | `max(40, 100 + 2 × d(TGH))` | 100 |
| 24 | Mind-status threshold: feared, charmed, confused, sleeping, dread, berserk, taunted, hacked | `max(40, 100 + 2 × d(RSV))` | 100 |
| 25 | Arcane-status threshold: silenced, unraveled | `max(40, 100 + 2 × d(ATN))` | 100 |
| 26 | Spell complexity budget | `4 + floor_div(LOG, 10)` points | 8 |
| 27 | Grimoire slots | `2 + floor_div(MEM, 20)` | 4 |
| 28 | Skill XP multiplier | `10000 + d(MEM) × 50` Bp | ×1.00 |
| 29 | Spell potency multiplier | `10000 + d(ATN) × 50 + school_skill × 30` Bp, then × the GLYPH term of §5.7 for each glyph | ×1.00 at skill 0 |
| 30 | Cast interruption resistance: chance to keep casting when hit | `clamp(2500 + d(FOC) × 75, 0, 9500)` Bp, keyed `rng.magic` | 25% |
| 31 | Dark-vision radius | `2 + floor_div(SGT, 20)` m | 4 m |
| 32 | Visual detection range bonus | `d(SGT) × 50` Bp | +0% |
| 33 | Hearing sensitivity bonus | `floor_div(d(HRG), 5)` dB | +0 dB |
| 34 | Footstep indicator radius | `8 + floor_div(HRG, 10)` m | 12 m |
| 35 | Passive trap and secret detection, per second, within 3 m | `clamp(500 + d(ITU) × 50 + skill.awareness × 50, 0, 9000)` Bp, keyed `rng.stealth` (§3.3) | 5% |
| 36 | Curse sense on pickup | `clamp(1000 + d(ITU) × 100, 0, 9000)` Bp, keyed `rng.ident` | 10% |
| 37 | Buy price multiplier | `clamp(10000 - d(CHM) × 30, 7000, 13000)` Bp | ×1.00 |
| 38 | Sell price share of base value | `clamp(3000 + d(CHM) × 30, 1500, 5000)` Bp | 30% |
| 39 | Threat generation multiplier | `10000 + d(MEN) × 100` Bp | ×1.00 |
| 40 | Disguise suspicion-rate multiplier | `clamp(10000 - d(GUI) × 100, 3000, 15000)` Bp | ×1.00 |
| 41 | Status buildup multiplier | `10000 + d(FAT) × 50` Bp | ×1.00 |
| 42 | Loot rarity shift | `d(SER)` feeds §13.4 | 0 |
| 43 | Death's Door chance, once per Folio, Errata only | `min(6000, mul_bp(clamp(1000 + d(PRV) × 50, 0, 4000), 10000 + (vrv(e) - 5) × 1000))` Bp | 10% |
| 44 | Curse resistance on equip | `max(0, d(PRV) × 50)` Bp | 0% |

### 6.5 Rows that apply to NPCs

- NPCs use rows 11, 13 (without the gear term), 15, 16, 17, 18, 22, 23, 24, 25, 30, 31, 32, 33, 34, 35, 39 and 41, computed from their effective sub-attributes (§13.8).
- NPCs never use rows 1–10, 12, 14, 19–21, 26–29, 36–38, 40 or 42–44.
- NPC HP and damage come from §13.8 and §17.3. NPC crit multiplier is 15000. NPC stamina and aether are template constants (doc 13).

---

## 7. Skills

There are 36 skills. Skills are integers `0..=100`. Mastery tiers unlock at 25 (Apprentice), 50 (Journeyman), 75 (Expert) and 100 (Master). Doc 04 owns the perks granted at each tier and the XP events table.

| ID | Name | Governing sub-attribute | Group |
|---|---|---|---|
| `skill.blade` | Blade | PRC | combat |
| `skill.hafted` | Hafted | PWR | combat |
| `skill.polearm` | Polearm | GRP | combat |
| `skill.unarmed` | Unarmed | PWR | combat |
| `skill.shield` | Shield | BRN | combat |
| `skill.marksmanship` | Marksmanship | PRC | combat |
| `skill.heavy_arms` | Heavy Arms | GRP | combat |
| `skill.throwing` | Throwing | SLT | combat |
| `skill.armor` | Armor | TGH | defense |
| `skill.acrobatics` | Acrobatics | AGI | mobility |
| `skill.athletics` | Athletics | END | mobility |
| `skill.evocation` | Evocation | ATN | magic |
| `skill.abjuration` | Abjuration | RSV | magic |
| `skill.alteration` | Alteration | FOC | magic |
| `skill.conjuration` | Conjuration | MEM | magic |
| `skill.illusion` | Illusion | GUI | magic |
| `skill.restoration` | Restoration | RES | magic |
| `skill.divination` | Divination | ITU | magic |
| `skill.spellcraft` | Spellcraft | LOG | magic |
| `skill.devotion` | Devotion | PRV | faith |
| `skill.sneak` | Sneak | AGI | stealth |
| `skill.security` | Security | SLT | stealth |
| `skill.pickpocket` | Pickpocket | GUI | stealth |
| `skill.smithing` | Smithing | BRN | craft |
| `skill.tinkering` | Tinkering | LOG | craft |
| `skill.alchemy` | Alchemy | ANL | craft |
| `skill.inscription` | Inscription | MEM | craft |
| `skill.appraisal` | Appraisal | ITU | knowledge |
| `skill.medicine` | Medicine | ANL | knowledge |
| `skill.survival` | Survival | SGT | knowledge |
| `skill.speech` | Speech | CHM | social |
| `skill.leadership` | Leadership | MEN | social |
| `skill.hacking` | Hacking | LOG | stealth |
| `skill.fieldcraft` | Fieldcraft | END | craft |
| `skill.awareness` | Awareness | HRG | stealth |
| `skill.handling` | Handling | CHM | social |

Scope notes:

- **Hacking** covers electronic locks, cameras, turrets, drones and `body.construct` NPCs (§11.1 `st.hacked`).
- **Fieldcraft** covers placing and repairing placeables, their block HP, breaking destructible blocks, setting traps and demolition charges.
- **Awareness** opposes Sneak in doc 09 and improves §6.4 row 35.
- **Handling** covers taming, minion command, mounts and vehicles.
- **Inscription** covers scrolls, runes, enchanting (`ench.`) and Rewriting.
- **Conjuration** covers all summoning, including undead.
- FAT and SER govern no skill, so they rise only through level-up allocation, mirroring Morrowind's Luck.

Weapon archetype to skill mapping is owned by doc 05. The mapping is by archetype, never by skin, so a longbow and a railgun both train Marksmanship.

### 7.1 Skill growth

- XP to go from skill `s` to `s + 1`: `need(s) = 4000 + 400 × s` milli-XP. Reaching 25 from 0 takes 220,000 milli-XP. Reaching 75 takes 1,410,000. Reaching 100 takes 2,380,000.
- Every skill XP grant is multiplied by the Skill XP multiplier (§6.4 row 28) before being added.
- **Combat skill XP is normalized by damage, never granted per hit.** A hit that deals `dmg_mhp` to a hostile with Max HP `target_max_mhp` grants `floor_div(1500 × dmg_mhp, target_max_mhp)` milli-XP to the weapon's skill (capped at 1500 per hit). Draining one standard enemy gives 1.5 skill XP whatever the weapon's fire rate. Flat per-hit combat grants are forbidden.
- **Non-combat XP is proportional to magnitude**: spell base cost, lock difficulty, item value appraised, ingredients combined. Doc 04 owns the table. Zero-effect actions grant 0.
- **Sub-attribute training.** Each skill level gained adds one training mark to the governing sub-attribute. When marks reach `T(v) = 2 + floor_div(v, 20)`, where `v` is the current base value, the base value rises by 1 and marks reset to 0. Training never raises a base value above 100.

### 7.2 Pace contract

At MEM 40, a Calling major skill used as the main damage source reaches 50 at Folio 10 ± 2 and 75 at Folio 22 ± 3. The reference supply is 40 standard-kill equivalents per Folio (§17.4). From the major-skill start of 20, reaching 75 costs 1,254 skill XP, which is about 1.5 XP per kill over 21 Folios. Doc 04 and doc 13 tune against this contract.

---

## 8. Character level and experience

- XP to go from level `L` to `L + 1`: `xp_to_next(L) = 50 × L × (L + 3)`. Level 2 needs 200 cumulative. Level 11 needs 27,500. Level 31 needs 542,500.
- There is no level cap. The design target is level 28 ± 3 on arrival at Folio 25.
- **Reference unit**: `kill_xp_std(d) = floor_div(7 × e × (e + 3), 5)`, where `e = enemy_level(d)` (§13.8). At 40 units per Folio this gives level 12 after Folio 10, level 29 after Folio 25 and level 47 after Folio 40.
- **Character XP sources**, so stealth, crafting and diplomacy builds level too:
  - Kills: `mul_bp(mul_bp(kill_xp_std(d), rank_xp_bp), 7000)`, with `rank_xp_bp` from §17.3.
  - Skill-ups: each skill level gained grants `2 × kill_xp_std(d_current)`, counting at most 8 skill levels per character per Folio.
  - Folio exit: `ev.folio_exited` grants `4 × kill_xp_std(d)`.
  - Unseen bonus: if no hostile reached `aware.alert` toward any Erratum during the Folio, the exit grants `20 × kill_xp_std(d)` more.
  - Doc 13 keeps the per-Folio total near 40 units for a combat-typical run, so the level-28 anchor holds.
- **Co-op split**: for kill XP, every party member present in the Folio gains `floor_div(kill_xp × 10000, enemy_count_bp[n])`, where `n` is party size and `enemy_count_bp` is the §17.2 table.
- Each level-up grants 4 attribute points. Each point adds +1 to the base value of any sub-attribute, with at most +2 to the same sub-attribute per level-up and never above 100.
- Each even level grants 1 talent point. Doc 04 owns talents.

---

## 9. Body plans and hit locations

Every creature has one body plan. Each body part is a simulation hitbox capsule. `share` is the part's slice of Max HP used for its **condition** pool. `hit_mult` multiplies damage landing on that part. `precision` marks parts where the weapon's precision multiplier applies. Doc 04 owns wound states and their effects.

| Body plan ID | Parts: ID (share Bp, hit_mult Bp, precision) |
|---|---|
| `body.humanoid` | `bp.head` (1500, 10000, yes), `bp.torso` (3500, 10000, no), `bp.arm_l` (1250, 8500, no), `bp.arm_r` (1250, 8500, no), `bp.leg_l` (1250, 8500, no), `bp.leg_r` (1250, 8500, no) |
| `body.quadruped` | `bp.head` (1500, 10000, yes), `bp.torso` (4500, 10000, no), `bp.leg_fl` (1000, 8500, no), `bp.leg_fr` (1000, 8500, no), `bp.leg_hl` (1000, 8500, no), `bp.leg_hr` (1000, 8500, no) |
| `body.avian` | `bp.head` (1500, 10000, yes), `bp.torso` (3500, 10000, no), `bp.wing_l` (1500, 8500, no), `bp.wing_r` (1500, 8500, no), `bp.leg_l` (1000, 8500, no), `bp.leg_r` (1000, 8500, no) |
| `body.serpentine` | `bp.head` (2000, 10000, yes), `bp.body_front` (4000, 10000, no), `bp.body_rear` (3000, 9000, no), `bp.tail` (1000, 8500, no) |
| `body.insectoid` | `bp.head` (1500, 10000, yes), `bp.thorax` (3500, 10000, no), `bp.abdomen` (2600, 10000, no), `bp.leg_1` to `bp.leg_6` (400 each, 8000, no) |
| `body.amorphous` | `bp.core` (2000, 10000, yes), `bp.mass` (8000, 10000, no) |
| `body.construct` | `bp.core` (2000, 10000, yes), `bp.chassis` (5000, 10000, no), `bp.limb_1` to `bp.limb_4` (750 each, 8500, no) |
| `body.swarm` | `bp.mass` (10000, 10000, no); area damage against a swarm is × 15000 (§10.5 step 2.10) |

- Mutations, implants and races may add parts such as `bp.tail`, `bp.arm_extra_1` or `bp.horn`. Added parts take their share from `bp.torso`, or from `bp.mass` or `bp.chassis` in plans without a torso.
- Bosses may flag extra parts as `bp.weakpoint_N` with `precision = yes`.
- Default weapon precision multiplier: ranged 15000 Bp, melee 12500 Bp. Each weapon may override (doc 05).
- **Hitbox poses.** Simulation hitbox capsules are posed only from baked fixed-point pose tables: per animation, per tick, per part, `Fx` endpoints and radius in model space, generated offline and committed as content. Simulation state stores `(animation_id, anim_tick)`. Bevy's skeletal animation never feeds the simulation. The host keeps 32 ticks of pose history for lag compensation (doc 16).
- **Part condition pools.** Each part's pool max is `mul_bp(max_hp_mhp, share)`. When Max HP changes, pools are recomputed and keep the same amount lost.

### 9.1 Severing

Severing is deterministic and draws no random numbers.

- Severable parts: limbs, tails and wings on every plan, and heads on NPCs. Errata heads never sever.
- A severable part is severed when its condition pool reaches 0 from a hit whose largest share is `dmg.slash`, or which carries the flag `sever`. Severing an NPC head also requires the attacker's governing weapon skill ≥ 75, and kills single-headed creatures.
- **Hand bindings.** `bp.arm_r` holds the active weapon slot. `bp.arm_l` holds `slot.offhand` and the second hand of two-handed weapons. `bp.arm_extra_N` grants `slot.hand_extra_N`, which holds a one-handed weapon or an off-hand item.
- **Effects.** Losing an arm drops the item in that hand, disables two-handed weapons and disables `slot.implant_arms` for that side. Losing a leg multiplies move speed by 5000 and removes sprint, jump and dodge; a crutch tool restores × 7000.
- A severed part becomes a `cat.body_part` item that rots after 7200 ticks. It can be reattached by Medicine ≥ 25 (600-tick channel), by a Scriptorium service or by a successful prayer, or replaced by an `aug.prosthetic_*` augment skinned per genre: hook, clockwork arm, cyberarm, silver hand, jade arm.
- Events: `ev.part_severed`, `ev.part_reattached`.

---
## 10. Damage types, resistances, armor and the damage pipeline

### 10.1 Damage types

`ward_mult` scales how fast each type depletes a ward (§10.5 step 7).

| ID | Category | Default color | Builds status | Armor efficiency (Bp) | ward_mult (Bp) |
|---|---|---|---|---|---|
| `dmg.slash` | physical | `#C9CED6` | `st.bleeding` | 10000 | 10000 |
| `dmg.pierce` | physical | `#E8E2C8` | `st.exposed` | 7500 | 10000 |
| `dmg.blunt` | physical | `#A68A64` | `st.stunned`; poise damage × 15000 | 5000 | 10000 |
| `dmg.fire` | elemental | `#FF6A1A` | `st.burning` | 0 | 7500 |
| `dmg.frost` | elemental | `#9BE7FF` | `st.frozen`; each hit also applies `st.chilled` | 0 | 10000 |
| `dmg.shock` | elemental | `#FFE14D` | `st.shocked` | 0 | 15000 |
| `dmg.acid` | elemental | `#9AE13B` | `st.corroded` | 0 | 7500 |
| `dmg.radiation` | elemental | `#2F6BFF` | `st.irradiated` | 0 | 10000 |
| `dmg.sonic` | elemental | `#C8B6FF` | `st.deafened`; each hit emits `ev.noise` at `60 + floor_div(d_mhp, 1000)` dB at the impact point and ends `st.frozen` like blunt | 2500 | 10000 |
| `dmg.toxin` | biological | `#6B8E23` | `st.poisoned` | 0 | 5000 |
| `dmg.radiant` | esoteric | `#FFF4C2` | `st.blinded`; × 15000 against undead and fiends (step 3a) | 0 | 10000 |
| `dmg.umbral` | esoteric | `#5B2A86` | `st.withering` | 0 | 10000 |
| `dmg.psychic` | esoteric | `#FF5FD2` | `st.dread` | 0 | 10000 |
| `dmg.arcane` | esoteric | `#3FE0D0` | `st.unraveled` | 0 | 12500 |
| `dmg.true` | true | `#FFFFFF` | none; ignores resistance and armor | 0 | 10000 |

`dmg.sonic` is elemental: Errata resistance to it caps at 7500, and armor applies to it at 2500 efficiency.

- Colors are defaults. UI also shows a unique icon per type, so color is never the only signal (doc 17).
- Genre skins rename types in text only. For example `dmg.shock` reads "Galvanic" in Steampunk. Doc 18 owns the skin names.

### 10.2 Resistances

- Each entity has `resist[t]` in Bp for every type except `dmg.true`.
- Range: −10000 (takes double) up to a cap.
- Caps: Errata 5000 for slash, pierce and blunt, and 7500 for every other type. NPCs 9000 for every type.
- Armor applies to every type with armor efficiency above 0: slash, pierce, blunt and sonic.
- An `immune: [types]` flag makes damage of those types 0 and skips their buildup.
- Sources add linearly before the cap.

### 10.3 Armor and penetration

- **Armor rating (AR)** is an `i64` per body part, summed from equipment covering that part plus natural armor, then multiplied by `st.corroded` and `st.fortified` in ascending status ID order.
- **Penetration** `pen_bp` (Bp, `0..=10000`) is the share of armor an attack ignores.
- Armor reduces only types with armor efficiency above 0, scaled by that efficiency.
- Armor is strong against many small hits and weak against few large hits, because the hit size is in the denominator.
- Errata gear AR scales with `T(ilvl)`. Enemy AR scales with `P(d)` (§13.8).

### 10.4 Source magnitude (step 0)

Every damaging source computes its base `B` in mHP before the pipeline:

```
Errata item source:  B = base_mhp → × P(src_lvl) → × rarity power (§13.3) → × quality (§13.5) → × (10000 + 600 × eff_enh) → × material_bp
Errata armor:        AR = base_ar → × T(ilvl) → × rarity power → × quality → × (10000 + 600 × eff_enh) → × mat_ar_bp
Enemy source:        B = template_dmg_mhp → × T(d) → × rank damage (§17.3)
```

Each arrow is one `mul_bp`, in that order. `src_lvl` follows §13.8.1. `material_bp` is `dmg_bp` and `mat_ar_bp` is `ar_bp` of the item's primary material (doc 07), each in `7000..=14000` with `mat.iron` at 10000. Doc 05 derives swing and recovery ticks from `mass_g`, so heavy materials hit harder and swing slower.

### 10.5 The damage pipeline

The order of steps is canonical. Doc 05 owns the details and the worked examples.

Inputs: `B`, type shares `share[t]` in Bp summing to 10000, `pen_bp`, the weapon's precision multiplier, status powers, flags.

1. **Additive bucket.** `add` = §6.4 row 12 (weapon attacks only) + `Σ mul_bp(bonus_i, potency_i)` over every "+% damage" affix, perk, talent and buff, where `potency_i` is that component's Contradiction potency. Clamp `add` to at least −9000. `D1 = mul_bp(B, 10000 + add)`.
2. **Multiplicative chain**, in this exact order, each step one `mul_bp`:
   1. spell potency (§6.4 row 29), spells and prayers only;
   2. precision multiplier, if the hit part is a precision part;
   3. crit multiplier, if the crit roll succeeded; sneak attacks never roll a crit;
   4. sneak-attack multiplier (doc 09) when, on the impact tick, the target's awareness of the attacker is below `aware.alert` or the target has `st.sleeping`. Eligibility is per (attacker, target) pair; other entities' awareness is irrelevant;
   4a. elevation: with `dy = attacker.pos.y - target.pos.y`, × 11000 if `dy >= 2500 mm`, × 9500 if `dy <= -2500 mm`; ranged maximum range also grows by `max(0, dy)`;
   4b. flanking: × 11500 when the attacker is outside the target's frontal cone (`ang_diff(target.yaw, bearing from target to attacker) > 10923` Bam) and the target is not targeting the attacker; Errata targets have no AI target, so only the cone check applies;
   5. Contradiction potency of the base-function component (§5.5);
   6. Anachronism `10000 + edge_bp` of the base-function component, if the target is native (§5.6);
   7. MYTHOS ambient (§5.7), once, if the attacker or the target is tagged undead, fiend, divine or eldritch;
   8. requirement shortfall (§6.3);
   9. the hit part's `hit_mult` (§9);
   10. 15000 if the hit is area damage and the target's body plan is `body.swarm`;
   11. target statuses in ascending status ID order: `st.exposed` `10000 + 800 × stacks`, then `st.knocked_down` 11000;
   12. attacker statuses in ascending status ID order: `st.berserk` 13000, `st.empowered` 12000, `st.weakened` 7500;
   13. difficulty multiplier (doc 14);
   14. party-size enemy damage multiplier (§17.2), when the attacker is an NPC and the target is an Erratum;
   15. friendly fire, when attacker and target are both `fac.errata` (minions included): `FF_DIRECT_BP` for weapon, spell and gadget hits, `FF_AREA_BP` for explosions, fire spread, surfaces, falling blocks and shock chains (§17.5).

   The result is `D2`.
3. **Split by type.** `d[t] = mul_bp(D2, share[t])`. The flooring remainder `D2 - Σ d[t]` goes to the type with the largest share, ties to the earlier row of §10.1.
   - **3a. Per-type target modifiers:** `d[radiant] × 15000` if the target is tagged undead or fiend. `d[blunt] × 15000` if the target has `st.frozen`, which is then removed.
4. **Resistance.** For each type, `r = clamp(resist[t], -10000, cap)`. Immune types become 0. Otherwise `d[t] = d[t] - mul_bp(d[t], r)`. `dmg.true` skips this step.
5. **Armor**, types with armor efficiency above 0 only. A hit flagged `ignore_armor` skips this step (doc 09 grants it to sneak attacks at Sneak 100):

   ```
   effAR = mul_bp(floor_div(AR_part × eff[t], 10000), 10000 - pen_bp)          // i64
   if effAR > 0:
       DR_bp = min(8500, ((effAR as i128 × 10_000_000) / (effAR as i128 × 1000 + 4 × d[t] as i128)) as i32)
       d[t]  = d[t] - mul_bp(d[t], DR_bp)
   ```

   Example: AR 100, `pen_bp` 0, a 20 HP slash hit (20,000 mHP) gives `DR_bp = floor(10^9 / 180,000) = 5555`, so the hit becomes 8,890 mHP.
6. **Chip floor.** `total = Σ d[t]`. If at least one type is not immune, `floor = floor_div(D2, 10)` for Erratum targets and `floor_div(D2, 20)` for NPC targets. If `total < floor`, the surplus `floor - total` is added to the non-immune type with the largest share, ties to the earlier §10.1 row.
   - **6b. Block** (§10.7), if the hit was blocked.
7. **Ward absorption.** The `st.shielded` pool is consumed first, then the ward generator's ward. For each type in §10.1 order with `d[t] > 0` while ward remains: `w = mul_bp(d[t], ward_mult[t])`. If `w <= ward`, then `ward -= w` and `d[t] = 0`. Otherwise `d[t] -= floor_div(ward × 10000, ward_mult[t])` and `ward = 0`, and `ev.ward_broken` is emitted. HP takes the rest.
8. **Apply.** Subtract from HP and from the hit part's condition pool. Add poise damage (impact × `S(src_lvl)`, doc 05). Add status buildup (§11.2). Resolve lethal damage (§10.8).
9. **Emit** `ev.damage_dealt` with the full per-type breakdown. The per-type values sum to the total.

### 10.6 Other entry points

| Source | Enters at | Skips | Notes |
|---|---|---|---|
| DoT pulses (§11.3) | step 4 (`dmg.true` skips 4) | armor, chip floor, crit, buildup, `st.exposed` | emits `ev.damage_dealt` with flag `dot` |
| `st.shocked` chain | step 3 with `D2 = mul_bp(HP and ward lost to the triggering hit, 2000)`, 100% `dmg.shock` | crit, chip floor, buildup, on-hit effects, further chaining | flag `no_chain` |
| Fall damage | step 4 as `dmg.blunt` (§4.2) | armor, chip floor, crit | |
| Environmental hazards not from an item | step 3 with `D2 = base × P(d)` against NPCs or `base × T(d)` against Errata | crit | doc 12 |
| Reflected damage (thorns) | step 3 | crit, buildup, reflection | |

### 10.7 Blocking

When a hit is blocked (doc 05 defines arcs and timing):

```
to_stamina_mhp   = mul_bp(total, block_stability_bp)               // §6.4 row 19
total            = total - to_stamina_mhp
stamina_loss_mhp = floor_div(to_stamina_mhp × 10000, T(d_current))
```

If stamina reaches 0, the blocker is guard-broken and gains `st.staggered`. Status buildup from a blocked hit is × 5000.

### 10.8 Lethal damage resolution

Hits in the same tick resolve in ascending (source EntityId, seq). When an entity's HP would reach 0 or below after step 7:

1. **Errata only:** if Death's Door is unused this Folio, roll §6.4 row 43 with keyed `rng.combat`. On success, HP = 1,000 mHP and the Erratum gains `st.deaths_door` for 60 ticks. Death's Door is marked used whether the roll succeeds or fails.
2. On failure, if at least one other party member is alive and not downed, the Erratum gains `st.downed` with HP 0 and `ev.player_downed` is emitted (doc 16).
3. Otherwise, emit `ev.entity_killed`. NPCs always go straight to this step. An Erratum that dies while another Erratum lives becomes a ghost (§17.6) and emits `ev.player_died` and `ev.player_ghosted`. A downed Erratum that bleeds out follows the same rule.

### 10.9 Healing

Healing amount × healing received (§6.4 row 3), then × 5000 if the target has `st.withering`, then capped at Max HP. Healing and ward amounts from spells, potions and gadgets are always stated in Bp of the recipient's Max HP, never as flat HP.

Area healing, ward and buff effects, thrown potions included, apply at full effect to every allied Erratum and minion in the area.

---

## 11. Status effects and buildup

### 11.1 Status list

Stacking rules:

- `refresh`: reapplying sets the expiry tick to `max(old_expiry, t + D)`.
- `intensity N`: each application adds one stack, up to N. All stacks share one expiry, which refreshes on each application.
- `pool`: the status holds a quantity, such as an overshield amount, instead of stacks.

Threshold groups `body`, `nerve`, `mind` and `arcane` use §6.4 rows 22 to 25. `direct` statuses are applied only directly, never by buildup. "Scaling" is the threshold group used for direct-application duration (§11.4); `—` means unscaled.

| ID | Category | Threshold group | Scaling | Stacking | Base duration | Canonical effect | Owner |
|---|---|---|---|---|---|---|---|
| `st.bleeding` | DoT | nerve | nerve | intensity 5 | 6 s | DoT rate 100 Bp per stack as `dmg.true`; a bandage or a single heal of at least 10% Max HP ends it | 05 |
| `st.burning` | DoT | nerve | nerve | refresh | 4 s | DoT rate 200 Bp as `dmg.fire`; ignites flammable blocks; water ends it | 05 |
| `st.poisoned` | DoT | body | body | intensity 5 | 8 s | DoT rate 75 Bp per stack as `dmg.toxin` | 08 |
| `st.corroded` | DoT | body | body | intensity 5 | 10 s | AR × `(10000 - 1000 × stacks)`; DoT rate 50 Bp as `dmg.acid` | 05 |
| `st.irradiated` | DoT | body | body | intensity 10 | 30 s | Max HP × `(10000 - 200 × stacks)` while active | 08 |
| `st.withering` | DoT | body | body | refresh | 8 s | healing received × 5000; DoT rate 50 Bp as `dmg.umbral` | 06 |
| `st.diseased` | debuff | body | body | refresh | 300 s | natural HP regeneration 0; stamina regeneration × 5000 | 04 |
| `st.exposed` | debuff | body | body | intensity 5 | 6 s | damage taken × `(10000 + 800 × stacks)`; each buildup fill adds one stack | 05 |
| `st.staggered` | control | poise | — | refresh | 30 ticks | cannot act or move; interrupts casting | 05 |
| `st.stunned` | control | nerve | nerve | refresh | 90 ticks | cannot act or move | 05 |
| `st.frozen` | control | nerve | nerve | refresh | 120 ticks | cannot act or move; the next blunt hit is × 15000 and ends it | 05 |
| `st.blinded` | debuff | nerve | nerve | refresh | 4 s | vision radius 2 m; spread × 20000; AI loses sight targets | 05 |
| `st.chilled` | debuff | direct | nerve | refresh | 3 s | move and attack speed × 8000 | 05 |
| `st.shocked` | control | nerve | nerve | refresh | 3 s | every 30 active ticks, a 6-tick micro-stun; each hit taken chains (§11.5) | 05 |
| `st.knocked_down` | control | direct | nerve | refresh | 60 ticks | prone; cannot act; damage taken × 11000 | 05 |
| `st.rooted` | control | direct | nerve | refresh | 2 s | cannot move; can act | 06 |
| `st.slowed` | debuff | direct | nerve | refresh | 4 s | move speed × 7000 | 05 |
| `st.disarmed` | control | direct | nerve | refresh | 3 s | active weapon unusable | 05 |
| `st.deafened` | debuff | nerve | nerve | refresh | 6 s | hearing −30 dB; no footstep indicators | 09 |
| `st.silenced` | control | arcane | arcane | refresh | 4 s | cannot cast spells | 06 |
| `st.unraveled` | debuff | arcane | arcane | refresh | 5 s | aether regeneration 0; −10 aether on application | 06 |
| `st.sleeping` | control | mind | mind | refresh | 8 s | cannot act; any damage ends it; sneak attacks always apply | 06 |
| `st.charmed` | control | mind | mind | refresh | 6 s | fights for the applier's faction | 06 |
| `st.feared` | control | mind | mind | refresh | 4 s | flees from the source; cannot attack | 06 |
| `st.confused` | control | mind | mind | refresh | 5 s | every 30 active ticks, movement input rotates by a keyed `rng.status` angle | 06 |
| `st.berserk` | control | mind | mind | refresh | 6 s | attacks the nearest entity; damage dealt × 13000 | 06 |
| `st.dread` | mental | mind | mind | refresh | 6 s | composure −15 on application; composure regeneration 0 | 06 |
| `st.madness` | mental | special | — | until composure ≥ 25% of max | none | hallucinated entities; aim sway × 20000; whispers | 04 |
| `st.weakened` | debuff | direct | — | refresh | 6 s | damage dealt × 7500 | 05 |
| `st.marked` | debuff | direct | — | refresh | 8 s | visible through walls to every hostile of the marker | 09 |
| `st.soaked` | debuff | direct | — | refresh | 10 s | shock buildup × 15000; fire buildup × 5000; ends burning | 08 |
| `st.oiled` | debuff | direct | — | refresh | 20 s | fire buildup × 20000 | 08 |
| `st.hungry` | need | special | — | while satiation ≤ 600 | none | stamina regeneration × 8000 | 04 |
| `st.starving` | need | special | — | while satiation = 0 | none | 1% Max HP lost per 5 s; no natural HP regeneration | 04 |
| `st.encumbered` | need | special | — | while `10000 < load_bp <= 12500` | none | move speed × 8500; stamina regeneration × 7500 | 04 |
| `st.overburdened` | need | special | — | while `load_bp > 12500` | none | move speed × 5000; no sprint, jump or dodge | 04 |
| `st.exhausted` | need | special | — | 2 s after stamina hits 0 | 2 s | no sprint, dodge or heavy attack | 04 |
| `st.contradicted` | indicator | special | — | while any equipped component has C ≥ 3 | none | UI only | 12 |
| `st.hasted` | buff | direct | — | refresh | 8 s | move speed × 12500; attack and cast speed × 11500 | 06 |
| `st.regenerating` | buff | direct | — | refresh | 10 s | heals at rate 200 Bp of Max HP per s | 06 |
| `st.shielded` | buff | direct | — | pool | 10 s | `res.ward` pool absorbs damage first | 06 |
| `st.invisible` | buff | direct | — | refresh | 10 s | sight detection × 1000; ends on attack | 09 |
| `st.hidden` | indicator | special | — | while no hostile holds awareness `aware.alert` or higher toward the bearer | none | UI only; never gates sneak attacks (§10.5 step 2.4) | 09 |
| `st.empowered` | buff | direct | — | refresh | 8 s | damage dealt × 12000 | 06 |
| `st.fortified` | buff | direct | — | refresh | 10 s | AR × 12000; poise × 12000 | 06 |
| `st.focused` | buff | direct | — | refresh | 10 s | spread × 7000; aim sway × 5000 | 08 |
| `st.levitating` | buff | direct | — | refresh | 10 s | gravity 0; vertical control 1500 mm/s | 06 |
| `st.blessed` | buff | direct | — | refresh | 30 s | crit chance +1000 Bp; all thresholds × 11000 | 06 |
| `st.inspired` | buff | direct | — | refresh | 10 s | personal VERVE +2 (§5.8) | 04 |
| `st.downed` | special | special | — | until revived or bled out | 30 s bleed-out | crawl only; an ally revives in 3 s | 16 |
| `st.deaths_door` | special | special | — | refresh | 60 ticks | invulnerable | 04 |
| `st.erasure` | special | direct | — | intensity 10 | until Folio exit | §11.6 | 13 |
| `st.hacked` | control | mind | mind | refresh | 6 s | applies only to `body.construct` and electronic entities; fights for the hacker's faction; constructs are immune to every other mind status | 09 |
| `st.taunted` | control | mind | mind | refresh | 4 s | must target the taunter; damage dealt to others × 7500 | 05 |
| `st.grappled` | control | direct | nerve | refresh | 3 s | cannot move; can use one-handed actions; ends when the grappler takes at least 10% of its Max HP | 05 |
| `st.polymorphed` | special | direct | arcane | refresh | 30 s | body plan and base stats replaced by a template; equipment stowed; HP ratio kept | 06 |
| `st.incubating` | special | direct | — | refresh | 7200 ticks | an affliction incubates; at expiry the affliction takes hold (§16.4) | 04 |
| `st.suffocating` | DoT | special | — | while `res.breath` = 0 | none | DoT rate 500 Bp as `dmg.true` | 04 |
| `st.mounted` | indicator | special | — | while riding | none | §17.7 | 05 |
| `st.bound` | indicator | special | — | while a minion is bound to its owner | none | §15.1 | 13 |
| `st.ghost` | special | special | — | until resurrected | none | §17.6 | 16 |
| `st.unmoored` | special | special | — | refresh | 300 ticks | §5.11 | 12 |

`load_bp = floor_div(carried_g × 10000, capacity_g)`.

### 11.2 Buildup

- Each entity has a buildup meter per status with a threshold group. Meters start at 0. Status power `p` is in threshold points (`i32`).
- A hit with status power `p` adds, each step one `mul_bp` in this order:

  ```
  p' = p × S(src_lvl)                                   // §13.8; enemies use S(d)
       × Contradiction potency of the base component
       × (10000 + edge_bp) if the target is native
       × attacker status buildup multiplier (§6.4 row 41)
       × (10000 - r)                                    // r = target's clamped resistance (§10.5 step 4) to the type carrying the status
       × 5000 if the hit was blocked
  ```

- When a meter reaches the threshold, apply the status at base duration and reset the meter to 0. For intensity statuses, each fill adds one stack.
- **Post-proc protection.** For 10 s after a proc, that status's threshold on that entity is × 15000. For bosses, editors and hunters it is × 20000 for 15 s. A re-proc restarts the timer and never compounds the multiplier.
- After 2 s without new buildup, a meter decays by 5% of its threshold per status pulse.
- Buildup never draws random numbers.

### 11.3 Damage over time

```
per_s = mul_bp(max_hp_eff_mhp, rate_bp × stacks)     // rate_bp from §11.1; max HP read at the pulse tick
per_s = mul_bp(per_s, applier_potency_snapshot)       // base-component potency when applied
per_s = mul_bp(per_s, rank_dot_bp)                    // §17.3
```

Each status pulse applies `q` from the §2.3 accumulator with `k = 4`. `q` enters the damage pipeline as §10.6 describes. Healing over time (`st.regenerating`) uses the same accumulator.

### 11.4 Direct application

Direct application ignores meters. Its duration is

```
duration = mul_bp(base, clamp(floor_div(1_000_000, thr), 2500, 15000))
```

where `thr` is the bearer's threshold for the status's scaling group. Unscaled statuses (`—`) use base duration. The result is then multiplied by the bearer's rank control multiplier (§17.3) for statuses of category `control`, and rounded up to at least 1 tick. Bosses, editors and hunters are immune to `st.charmed`, `st.sleeping`, `st.confused`, `st.hacked` and `st.polymorphed`. Their control statuses from buildup also use the rank control multiplier.

### 11.5 Shock chains

While `st.shocked` is active, each hit the bearer takes (except a `no_chain` hit) creates one chained hit. The target is the entity, other than the bearer and the original attacker, with minimum `dist2` from the bearer's origin within 4 m, ties broken by lower EntityId. The chained hit enters §10.6.

### 11.6 Erasure

- From `ERASURE_ONSET` (§17.1), every Erratum gains one stack of `st.erasure` on the same tick, every `ERASURE_STEP`.
- Each stack makes the bearer 1 dB louder (doc 09) and multiplies healing received by `10000 - 300 × stacks`.
- When the first Erratum reaches 10 stacks, one Editor hunter (`rank.hunter`) spawns for the party. Another spawns every 90 s while any Erratum holds 10 stacks.
- `st.erasure` is removed from every Erratum on `ev.folio_entered`.
- Erasure accrual and satiation decay pause in boss Folios while the boss lives.
- Hunters and every `fac.editors` member erase Errata placeables within 1500 mm of them, at one placeable block per 10 ticks.
- Each dose of the `afflict.red_ink` affliction (§16.4) lowers the hunter-spawn threshold by 1, from 10 down to a minimum of 6.

### 11.7 Multiplier order

When several statuses multiply the same stat, they apply after all non-status factors, one `mul_bp` per status, in ascending status ID byte order.

### 11.8 Player-controlled bearers

Input is never synthesized for a player. When the bearer is an Erratum controlled by a player, these rules replace the AI-facing effects:

| Status | Effect on a player-controlled bearer |
|---|---|
| `st.charmed` | outgoing damage against the applier's faction × 0; the bearer's hits on allies use × 10000 instead of the friendly-fire scale; the HUD tints allies as hostile |
| `st.berserk` | cannot block, cast, reload or use items; melee costs 0 stamina; damage dealt × 13000 |
| `st.feared` | cannot fire or cast while facing within 8192 Bam of the source; move speed × 11000 while moving away from it |
| `st.sleeping` | input disabled; ends on any damage or after an ally's 30-tick interact; duration × 5000 |
| `st.confused` | unchanged |
| `st.taunted` | the HUD marks the taunter; damage dealt to anyone else × 7500 |

---

## 12. Resources

| ID | Type | Range | Canonical rule | Owner |
|---|---|---|---|---|
| `res.hp` | Milli | `0..=max` | §6.4 rows 1–3; lethal resolution §10.8 | 04 |
| `res.stamina` | Milli | `0..=max` | spent by sprint, dodge, melee, block; §6.4 rows 4–6 | 04 |
| `res.aether` | Milli | `0..=max` | spent by spells and arcane items; displayed under the local genre's Aether skin (§5.2) | 06 |
| `res.composure` | Milli | `0..=max` | lost to horror and psychic effects; at 0 applies `st.madness` | 04 |
| `res.satiation` | i32 | `0..=2400` | −1 per second; starts at 1800 each run; `st.hungry` at ≤ 600; `st.starving` at 0 | 04 |
| `res.toxicity` | Milli | `0..=100,000` | raised by potions and stims; −500 per second; at 100,000 applies 3 stacks of `st.poisoned` every 5 s | 08 |
| `res.poise` | Milli | `0..=poise` | reduced by impact; at 0 applies `st.staggered` and refills | 05 |
| `res.ward` | Milli | `0..=pool` | the `st.shielded` pool plus the ward generator's ward (§13.2 `slot.ward`); absorbs damage before HP (§10.5 step 7) | 06 |
| `res.breath` | Milli | `0..=max` | max `10 + floor_div(END, 10)` s, so 14 s at END 40; drains 1000 per s while the eye cell holds liquid or an unbreathable gas; regenerates 5000 per s otherwise; at 0 applies `st.suffocating`; kin that need no breath are exempt; a respirator in `slot.face` makes gases breathable | 04 |
| `res.favor` | i32 | `-1000..=1000` per patron | run-scoped standing with each genre's patron (§16.5) | 06 |

- **Composure regeneration**: 1 per second while not `in_combat` and `light_at(e) >= 8`.
- **Max changes.** When a pool's maximum changes by Δ: if Δ > 0, current += Δ. Then current = `min(current, new_max)`. This applies to HP, stamina, aether, composure, poise, breath and part condition pools.

### 12.1 Surfaces and reactions

Docs 03, 05, 06 and 08 implement this one table. **Contact** means the feet cell or a body cell holds at least 100 ml of the liquid, or the eye cell holds the gas.

Per status pulse while in contact:

| Surface | Effect |
|---|---|
| `liq.water` | `st.soaked` |
| `liq.oil` | `st.oiled`; ground friction × 3000 |
| `liq.alcohol` | `st.oiled` |
| `liq.acid` | 25 `st.corroded` buildup |
| `liq.lava` | DoT rate 1000 Bp as `dmg.fire`, plus 100 `st.burning` buildup |
| `liq.red_ink` | +1 `st.erasure` per 300 ticks of contact |
| `gas.steam` | DoT rate 50 Bp as `dmg.fire` |
| `gas.miasma` | 20 `st.poisoned` buildup; unbreathable |
| `gas.smoke` | sight concealment only (§12.2) |
| `block.ice` | ground acceleration and friction × 1500 |
| `block.web` | `st.rooted` while inside |

Triggers, each emitting `ev.liquid_reaction`:

| Trigger | Result |
|---|---|
| Shock damage on water, blood or alcohol | electrifies the connected body of that liquid (at most 64 cells) for 180 ticks; contact gives 30 `st.shocked` buildup per pulse |
| Frost damage on water, blood or alcohol | becomes `block.ice` for 1200 ticks |
| Fire damage on water | becomes `gas.steam`, 1 cell per 1000 ml |
| Fire damage on oil or alcohol | ignites; a 2 m environmental `dmg.fire` blast of base 10 HP per 1000 ml, capped at 5000 ml |
| Fire damage on ice | melts back to water |
| Fire damage on web | burns out in 60 ticks |
| Fire damage on miasma | a 3 m environmental `dmg.fire` blast of base 20 HP |
| Radiant damage on red ink | becomes water |

Every blast enters the damage pipeline as an environmental hazard (§10.6).

### 12.2 Sight concealment

Every material (doc 07) and gas (doc 08) has `conceal: u8` in `0..=15`. Reference values: glass 0, tall grass 2 (only against crouched targets), leaves and foliage 3, water 2 per block, smoke 4 per cell at full density scaled linearly by density, steam 3, genre fog 1 per 8 m. A sight ray sums `conceal` over the cells it crosses, excluding both endpoint cells. At a sum of 15 or more, sight is blocked. Below that, sight detection is multiplied by `10000 - 667 × sum` Bp (doc 09).

### 12.3 Spawn light

Ambient and wave spawns (doc 13) occur only in cells whose `light_block` is at most the genre's `spawn_block_max` (§5.9). Placed light sources (`cat.placeable`) are therefore a defense, as in Minecraft.

---
## 13. Items

### 13.1 Categories

| ID | Meaning | Owner |
|---|---|---|
| `cat.weapon_melee` | melee weapons | 05, 11 |
| `cat.weapon_ranged` | every ranged weapon archetype, including bows, guns and staves | 05, 11 |
| `cat.wand` | charge-based casting items | 06, 10, 11 |
| `cat.armor` | head, body, hands, legs and feet pieces | 05, 11 |
| `cat.offhand` | shields, foci, lanterns, off-hand sidearms | 05, 11 |
| `cat.accessory` | rings, amulets, trinkets, belts, back items | 11 |
| `cat.implant` | augments installed into the body | 04, 11 |
| `cat.ammo` | ammunition | 05 |
| `cat.potion` | drinkable or throwable liquids in containers | 08 |
| `cat.text` | scroll-like single-use texts: scrolls, writs, data shards, talismans | 06, 10 |
| `cat.book` | permanent learning: glyph tomes, recipe books, lore | 06, 07 |
| `cat.food` | satiation items | 04 |
| `cat.throwable` | grenades, bombs, flasks, throwing knives | 05, 08 |
| `cat.gadget` | deployables: turrets, drones, traps, mines | 07 |
| `cat.tool` | pickaxes, lockpicks, hacking decks, rope, bandages | 07, 09 |
| `cat.material` | crafting materials | 07 |
| `cat.ingredient` | alchemical ingredients | 08 |
| `cat.container` | bottles, flasks, bags, satchels | 08, 11 |
| `cat.placeable` | barricades, bridge spans, ladders, traps, light sources, deployable cover (§4.3) | 07 |
| `cat.key` | keys and keycards of any skin | 11 |
| `cat.currency` | Lucre piles | 11 |
| `cat.quest` | narrative items | 12 |
| `cat.part` | loose weapon parts and licensed grafts that cross manufacturers | 07, 11 |
| `cat.ward_gen` | ward generators: shields of every skin, from aegis charms to kinetic barriers | 05, 11 |
| `cat.mount` | mount tokens: whistles, keys, bridles | 05, 11 |
| `cat.seed` | plantable seeds | 07, 08 |
| `cat.corpse` | corpses and Errata Remains: carry, loot, eat, raise, sacrifice | 04, 13, 16 |
| `cat.body_part` | severed body parts (§9.1) | 04, 07, 08 |

### 13.2 Equipment slots

| ID | Accepts |
|---|---|
| `slot.head`, `slot.body`, `slot.hands`, `slot.legs`, `slot.feet` | armor pieces of that kind |
| `slot.back` | cloaks, backpacks, jetpacks, wings |
| `slot.neck` | amulets |
| `slot.ring_l`, `slot.ring_r` | rings |
| `slot.belt` | belts; a belt sets the quickbar size |
| `slot.trinket` | relics and charms |
| `slot.weapon_1`, `slot.weapon_2`, `slot.weapon_3` | any melee or ranged weapon |
| `slot.offhand` | off-hand items; empty while a two-handed weapon is active |
| `slot.implant_head`, `slot.implant_torso`, `slot.implant_arms`, `slot.implant_legs` | implants |
| `slot.ward` | one ward generator: capacity `mul_bp(cap_base_mhp, T(ilvl))`; recharge starts 180 ticks after the bearer last took damage and restores 2500 Bp of capacity per second; dependency from its skin |
| `slot.face` | masks, goggles, respirators; respirators make gases breathable; masks serve disguises (doc 09) |
| `slot.mount` | one mount token (§17.7) |
| `slot.hand_extra_N` | granted by `bp.arm_extra_N` (§9.1): one one-handed weapon or off-hand item |

The quickbar has 4 slots without a belt and up to 8 with one.

### 13.3 Rarity

| ID | Name | Color | Affixes | Special | Base weight | Power (Bp) | Value multiplier |
|---|---|---|---|---|---|---|---|
| `rar.common` | Common | `#E6E6E6` | 0 | none | 7000 | 10000 | ×1 |
| `rar.uncommon` | Uncommon | `#3CC45A` | 1 | none | 2200 | 10500 | ×2 |
| `rar.rare` | Rare | `#3D8BFF` | 2 | none | 600 | 11000 | ×4 |
| `rar.epic` | Epic | `#A64DFF` | 3 | none | 150 | 11500 | ×8 |
| `rar.legendary` | Legendary | `#FF9A1F` | 3 | one legendary perk | 45 | 12500 | ×16 |
| `rar.exotic` | Exotic | `#FFD84A` | 2 | one fixed named intrinsic; at most one Exotic among `slot.weapon_1..3` and `slot.offhand`, at most one among the five armor slots, and at most one among every other slot; Palimpsestic items are exempt | 15 | 12500 | ×32 |
| `rar.palimpsestic` | Palimpsestic | animated prismatic gradient | 4 | immune to Contradiction; re-skins itself to the local genre; effective ilvl `max(ilvl, current Folio index)`, re-evaluated on `ev.folio_entered` | 0 | 13000 | ×64 |

Palimpsestic items come only from Editor bosses, Volume-completion rewards and special secret rooms (doc 11).

### 13.4 Rarity roll

For a generic drop at Folio depth `d` with the receiving player's Serendipity `SER`:

```
w[0] = 7000
w[r] = max(0, floor_div(base[r] × (10000 + (d - 1) × 400 × r + (SER - 40) × 50 × r), 10000))   for r in 1..=6
rarity = keyed rng.loot .weighted(w)
```

`base` is the base-weight column of §13.3. Each player's instanced drop uses that player's own SER. Doc 11 adds per-source shifts on top. Reference: at `d = 25`, `SER = 40` the weights are `[7000, 4312, 1752, 582, 217, 87, 0]`. At `d = 1`, `SER = 100` they are `[7000, 2860, 960, 285, 99, 37, 0]`.

### 13.5 Quality

| ID | Name | Multiplier (Bp) |
|---|---|---|
| `q.awful` | Awful | 7000 |
| `q.poor` | Poor | 8500 |
| `q.normal` | Normal | 10000 |
| `q.good` | Good | 11000 |
| `q.excellent` | Excellent | 12000 |
| `q.masterwork` | Masterwork | 13500 |
| `q.peerless` | Peerless | 15000 |

Quality multiplies an item's base stats. Found items roll quality as doc 11 specifies. Crafted items roll quality as doc 07 specifies.

### 13.6 Knowledge, curses and unknown appearances

Unknown-appearance classes:

| ID | Item types | Appearance pool examples |
|---|---|---|
| `idc.potion` | every `cat.potion` type | colors: crimson, murky green, ... |
| `idc.text` | every `cat.text` type | sigils, glyph scripts, file hashes |
| `idc.ring` | every ring type in `cat.accessory` | stones and metals |
| `idc.amulet` | every amulet type in `cat.accessory` | pendant shapes |
| `idc.wand` | every `cat.wand` type | woods and alloys |
| `idc.implant` | every `cat.implant` type | manufacturer stamps |
| `idc.relic` | every trinket type in `cat.accessory` | carvings |

**Appearance shuffle.** At run start, for each `idc` class in ascending ID byte order: `types` = all item types of that class in the run's content set, sorted by ID; `pool` = all appearance descriptors of that class, sorted by ID; `rng.appearance.shuffle(pool)`; `types[i]` gets `pool[i]`. The run's content set is the host's, fixed at run start and recorded in the save. Daily and weekly runs use the content set `unlock.daily_baseline`, which contains every base item and no account unlocks. Doc 10 guarantees `|pool| >= |types|` for every class.

An appearance descriptor is genre-neutral. The genre skin renders it: a crimson potion is a crimson flask in Fantasy, a crimson-capped injector in Cyberpunk and a crimson pill in Wuxia. Type knowledge is shared across genres and across the whole party.

An item type is type-hidden if and only if an `idc` class above lists it, and type-hidden items start at `know.unknown`. Every other item, including staves, belts, weapons, armor, tools, materials and blocks, starts at `know.classified`, because only its magic is hidden, following Barony 5.0.

| Knowledge state | Player knows |
|---|---|
| `know.unknown` | appearance only |
| `know.classified` | item type, for example "Potion of Healing" or "Longsword" |
| `know.appraised` | rarity and a value range |
| `know.identified` | everything: affixes, enhancement, curse state, charges |

| Curse state | Effect |
|---|---|
| `buc.blessed` | consumables: positive effects × 12500; equipment: +1 effective enhancement |
| `buc.uncursed` | normal |
| `buc.cursed` | cannot be unequipped until uncursed; `curse_penalty` 1..=3; carries one curse affix (doc 10) |

Genre skins rename curse states, for example Blessed / — / Hexed, or Signed / — / Corrupted.

### 13.7 Enhancement and Emendations

- Stored `enhancement` is the Emendation-driven level, `0..=15`.
- `eff_enh = clamp(enhancement + [buc = blessed] - curse_penalty, -3, 15)`. It feeds §10.4 as `10000 + 600 × eff_enh` and every other stat the same way.
- An **Emendation** is the consumable that raises enhancement, the analog of Shattered Pixel Dungeon's Scroll of Upgrade. Applying one first sets `ilvl = max(ilvl, current Folio index)`, then adds +1 enhancement. Upgrades therefore never become obsolete as the run deepens.
- An Emendation used on a cursed item instead sets `buc = uncursed`, `curse_penalty = 0`, removes the curse affix and leaves enhancement unchanged.
- `EMENDATIONS_PER_VOLUME = 3`, placed with guaranteed distribution by doc 11, so a full run yields 15.
- **Marginalia.** The enhancement cap is `15 + floor_div(d_current - 25, 5)` in Marginalia. Each Erratum receives 1 Emendation on every Marginalia `ev.folio_exited`. With the re-level rule, XP-and-upgrade numbers keep climbing as in Experienced Pixel Dungeon.

### 13.8 Item level, depth and power tables

- Item level `ilvl` is the Folio index where the item was generated: 1 to 25 for the main run, 26 or more in Marginalia. Rewriting keeps `ilvl`.
- Every table is `i64` Bp. Rows 1 to 40 are committed literal constants and are never regenerated at runtime. Rows 1 to 25 were generated in exact integer arithmetic as `floor_div(10000 × 10900^(d-1), 10000^(d-1))` for P and `floor_div(10000 × 10550^(d-1), 10000^(d-1))` for T. Rows 26 to 40 follow the recurrence.
- For `d > 40`: `P(d) = mul_bp(P(d-1), 10600)` and `T(d) = mul_bp(T(d-1), 10400)`, computed with `i128` intermediates.
- **Power depth cap.** Every table, including `S`, uses `min(d, 300)` (or `min(lvl, 300)`). Deeper Marginalia scales only through doc 14 difficulty multipliers. Depths, item levels and source levels are `u32` values of at least 1.
- `base × P(d)` and every other table product is `mul_bp` with an `i64` multiplier: `floor_div(base as i128 × P(d) as i128, 10000)`, narrowed with a check.
- `H(d) = mul_bp(P(d), 10000 + 650 × (min(d, 25) - 1) + 250 × max(0, d - 25))`.
- `S(lvl) = 10000 + 300 × (lvl - 1)`.

| Table | Used for |
|---|---|
| `P(d)` | Errata weapon and source damage (§10.4), enemy armor rating, Lucre amounts and item values |
| `T(d)` | enemy damage, Errata gear armor rating, shield block values and item ward pools |
| `H(d)` | enemy HP: `template_hp × H(d) × rank HP × party-size HP` |
| `S(lvl)` | status power and poise damage of every source (§11.2) |

| d | P(d) | T(d) | H(d) | d | P(d) | T(d) | H(d) |
|---|---|---|---|---|---|---|---|
| 1 | 10000 | 10000 | 10000 | 21 | 56044 | 29177 | 128901 |
| 2 | 10900 | 10550 | 11608 | 22 | 61088 | 30782 | 144473 |
| 3 | 11881 | 11130 | 13425 | 23 | 66586 | 32475 | 161803 |
| 4 | 12950 | 11742 | 15475 | 24 | 72578 | 34261 | 181082 |
| 5 | 14115 | 12388 | 17784 | 25 | 79110 | 36145 | 202521 |
| 6 | 15386 | 13069 | 20386 | 26 | 83856 | 37590 | 216767 |
| 7 | 16771 | 13788 | 23311 | 27 | 88887 | 39093 | 231995 |
| 8 | 18280 | 14546 | 26597 | 28 | 94220 | 40656 | 248269 |
| 9 | 19925 | 15346 | 30286 | 29 | 99873 | 42282 | 265662 |
| 10 | 21718 | 16190 | 34423 | 30 | 105865 | 43973 | 284247 |
| 11 | 23673 | 17081 | 39060 | 31 | 112216 | 45731 | 304105 |
| 12 | 25804 | 18020 | 44253 | 32 | 118948 | 47560 | 325322 |
| 13 | 28126 | 19012 | 50064 | 33 | 126084 | 49462 | 347991 |
| 14 | 30658 | 20057 | 56564 | 34 | 133649 | 51440 | 372212 |
| 15 | 33417 | 21160 | 63826 | 35 | 141667 | 53497 | 398084 |
| 16 | 36424 | 22324 | 71937 | 36 | 150167 | 55636 | 425723 |
| 17 | 39703 | 23552 | 80994 | 37 | 159177 | 57861 | 455246 |
| 18 | 43276 | 24848 | 91095 | 38 | 168727 | 60175 | 486777 |
| 19 | 47171 | 26214 | 102361 | 39 | 178850 | 62582 | 520453 |
| 20 | 51416 | 27656 | 114914 | 40 | 189581 | 65085 | 556420 |

- Enemy level: `enemy_level(d) = d + floor_div(d, 5)`.
- Enemy effective sub-attributes: template value `+ floor_div(3 × (d - 1), 2)`, capped at 150.
- `ITEM_MULT_MAX = 51870` Bp for Folios 1 to 25: the product of the best rarity power (13000), the best quality (15000), the best enhancement (19000) and the best material (14000), a ×5.187 ceiling over a Common Normal +0 iron item. Doc 05 and doc 13 tune against it.

#### 13.8.1 Source level

Every damage, ward or summon-stat magnitude is `base × P(src_lvl)` (or `T` where §13.8 says so), with `src_lvl` defined as:

| Source | `src_lvl` |
|---|---|
| Weapon, off-hand or thrown item | its `ilvl` |
| Spell, prayer or glyph | `ilvl` of the equipped casting implement (focus, wand, staff or tome); with none, `max(1, d_current - 3)` |
| Crafted or brewed potion, bomb, gadget, turret, drone, trap or mine | the Folio index where it was made, stored as its `ilvl` |
| Summon | the summoner's `src_lvl` for the summoning spell |
| Innate ability or Echo trait | `d_current` |
| Environmental hazard not from an item | `d_current` |
| Every enemy ability | `d_current` through `T(d)` |

#### 13.8.2 Marginalia armor HP

`gear_hp_mhp = Σ over equipped armor pieces of mul_bp(hp_base[slot] × 1000, max(0, T(ilvl) - T(25)))`, with `hp_base`: body 30, head 14, legs 14, hands 6, feet 6. It adds 0 in the main run, 745,143 mHP for a full set at ilvl 60 and 4,538,772 mHP at ilvl 100, which keeps Errata between 70 and 130 HP per threat unit through depth 150.

### 13.8a One material table

Blocks and items share one material table owned by doc 07, following IVAN. A block is a material plus a shape. An item component is a material plus a volume. Harvesting a resource node yields its material. An item's mass is its volume times its material density.

### 13.9 Item instance contract

Doc 19 owns the full schema. Every item instance carries at least:

| Field | Type | Meaning |
|---|---|---|
| `uid` | u64 | §1.11 |
| `template` | content ID | archetype or unique template |
| `origin_genre` | content ID | the skin's genre; drives requirements (§5.3) |
| `rarity` | content ID | §13.3 |
| `quality` | content ID | §13.5 |
| `ilvl` | i32 | §13.8 |
| `enhancement` | i32 `0..=15` | §13.7 |
| `buc` | content ID | §13.6 |
| `curse_penalty` | i8 `0..=3` | §13.6 |
| `knowledge` | content ID plus flags | §13.6; flags for rarity, enhancement, curse state, each affix, charges |
| `affixes` | list of {id, tier, roll_bp} | doc 11 |
| `parts` | list of {slot, part_id} | assembled items only (doc 07) |
| `materials` | list of material IDs | doc 07 |
| `durability` | i32 current and max | doc 07 |
| `charges` | i32 current and max | wands, gadgets |
| `stack` | i32 | stackables |
| `mass_g` | Grams | |
| `value` | i64 Lucre | base value before buy and sell multipliers; scales with `P(ilvl)` (doc 11) |
| `manufacturer` | content ID or none | `mfr.` (§5.9, doc 11) |
| `gear_tier` | i8 `1..=5` | `min(5, 1 + floor_div(errata_grade, 5))` at generation (doc 14) |
| `perk_columns` | list of {column, options (1..=3 perk IDs), chosen} | options per column by gear tier 1/1/2/2/3; choosing is free at a Scriptorium (doc 11) |
| `illuminated` | bool | cosmetic shiny variant: one keyed `rng.loot` roll at 5 Bp per Legendary-or-better drop; picking one up unlocks its skin account-wide; no stat effect |
| `finder_slot` | u8 `0..=3` | the party slot whose instanced roll created it |

---

## 14. Currencies

There is no premium currency. Real money buys fiat SKUs only (doc 15). No currency below can be bought, traded or gifted.

| ID | Name | Scope | Source | Spent on |
|---|---|---|---|---|
| `cur.lucre` | Lucre | run | drops, sales | in-run shops, services, Fortune Engines; displayed under the local genre's Lucre skin (§5.2); one shared value across genres; lost when the run ends |
| `cur.quire` | Quires | account | run score, achievements, Codex tiers | the Margin unlock tree (doc 14) |
| `cur.ink` | Ink | account | play time, run completion, Codex tiers, achievements | Inkwell pulls (doc 15) |
| `cur.vellum` | Vellum | account | Inkwell duplicates and unused spark at banner end | the Vellum Archive: past featured Echoes, Scripts and cosmetic tiers (doc 15) |

Entitlements are not currencies. A **Quill** (`ent.quill`) unlocks one Codex premium track. It is obtained by purchasing a Codex or from tier 100 of a premium Codex.

Lucre never converts into any account currency. Nothing purchasable ever converts into Ink, Quires or Vellum.

---

## 15. Factions

| ID pattern | Members | Default stance toward `fac.errata` |
|---|---|---|
| `fac.errata` | player characters, allied Errata, hirelings | allied |
| `fac.editors` | Editors and their Red Ink servants; appear in every genre | hostile; also hostile to natives carrying Contradicted items |
| `fac.native.<genre_suffix>.<group>` | sapient native factions, 2 to 4 per genre (doc 13) | per faction |
| `fac.wild.<genre_suffix>` | beasts and mindless monsters | hostile |
| `fac.margin` | hub NPCs | allied, non-combat |
| `fac.neutral` | wandering vendors and non-combatants | neutral until attacked |

Reputation per faction is an integer `-1000..=1000` and resets each run.

| Range | Stance |
|---|---|
| −1000..=−500 | hostile |
| −499..=−101 | unfriendly |
| −100..=100 | neutral |
| 101..=499 | friendly |
| 500..=1000 | allied |

### 15.1 Minions

- Minions are summons, tamed beasts and hirelings. They belong to `fac.errata` and carry `owner_slot`. Friendly-fire rules (§17.5) apply to them.
- `MINION_CAP_PER_PLAYER = min(5, 1 + floor_div(max(skill.conjuration, skill.handling), 25))`. `FOLIO_MINION_CAP = 16`, counted inside `ACTIVE_ACTOR_CAP`.
- A minion's kill grants kill XP exactly as if its owner had made it.
- Spell summons despawn on `ev.folio_exited`. Tamed beasts and hirelings follow through an exit if they are within 16 m, persist for the run, and are lost permanently when they die.
- Commands: `input.minion_follow`, `input.minion_hold`, `input.minion_attack` (doc 17). Events: `ev.minion_bound`, `ev.minion_released`.

---

## 16. Origins, Callings, Echoes, kin, afflictions and patrons

### 16.1 Origins

- **Origin**: the genre an Erratum fell out of. At creation the player picks one of the 12 core genres. Secret genres become Origin options through meta unlocks (doc 14). The Origin sets tolerance (§5.4), sub-attribute modifiers that sum to 0 and stay within ±6 each, +10 to one Origin skill, and one Origin trait (doc 04).
### 16.2 Callings and Echoes

- **Calling**: the class. Each Calling sets three major skills to 20 and three minor skills to 10. All other skills start at 0. Doc 04 owns kits and talent trees.

| ID | Identity | Major skills | Minor skills |
|---|---|---|---|
| `calling.vanguard` | heavy melee frontliner | hafted, armor, shield | athletics, leadership, smithing |
| `calling.duelist` | agile swordfighter | blade, acrobatics, speech | sneak, throwing, unarmed |
| `calling.gunslinger` | precision shooter | marksmanship, acrobatics, throwing | tinkering, speech, security |
| `calling.grenadier` | heavy weapons, explosives and sapping | heavy_arms, throwing, armor | tinkering, fieldcraft, medicine |
| `calling.spellwright` | spell designer and caster | evocation, spellcraft, alteration | abjuration, inscription, divination |
| `calling.alchemist` | brewer and bomber | alchemy, throwing, medicine | survival, restoration, appraisal |
| `calling.shade` | stealth assassin and thief | sneak, blade, security | pickpocket, acrobatics, illusion |
| `calling.artificer` | crafter, gunsmith, gadgeteer | tinkering, smithing, marksmanship | security, inscription, appraisal |
| `calling.warden` | protector and healer | shield, abjuration, restoration | polearm, devotion, leadership |
| `calling.oracle` | seer and priest | divination, devotion, conjuration | appraisal, illusion, speech |
| `calling.netrunner` | hacker and infiltrator | hacking, tinkering, sneak | security, awareness, illusion |
| `calling.beastbinder` | tamer and pack leader | handling, conjuration, survival | polearm, leadership, restoration |
| `calling.monk` | martial artist | unarmed, acrobatics, athletics | alteration, medicine, awareness |

There are 13 Callings.

- **Echo**: a pre-built Erratum. An Echo fixes a kin, an Origin, a Calling, a signature trait and cosmetics. It is acquired through the Inkwell (doc 15) or through its Recollection trial (doc 14). Launch roster: 24 Echoes, 2 per core genre. ID pattern `echo.<name>`.

### 16.3 Kin

Each Erratum has one kin. Echoes fix it. Kin passive traits have dependency none. Each kin is skinned per Origin, for example construct as golem, clockwork or android, and undead as revenant, jiangshi or ghoul. Doc 04 owns the full trait lists.

| ID | Body plan | Rules |
|---|---|---|
| `kin.human` | `body.humanoid` | no changes |
| `kin.construct` | `body.humanoid` | immune to `st.poisoned`, `st.diseased`, `st.bleeding`; no satiation, refuels from `cat.material` tagged fuel; needs no breath; resist shock −2500; potion healing × 5000; Tinkering repair heals |
| `kin.undead` | `body.humanoid` | no natural HP regeneration; `dmg.umbral` taken heals 5000 Bp of its amount instead; resist radiant −5000; immune to `st.poisoned`, `st.diseased`; needs no breath |
| `kin.beastkin` | `body.humanoid` plus `bp.tail` | unarmed attacks carry a 5000 Bp slash share; dark-vision +2 m |
| `kin.fae` | `body.humanoid` | ARCANA tolerance +1; takes × 12500 damage from `mat.cold_iron` and `mat.iron` weapons |
| `kin.mutant` | `body.humanoid` | starts with one `aug.` mutation chosen from 3 offered; +1 mutation point every 2 levels |

### 16.4 Afflictions

Afflictions (`afflict.`, owner 04) are run-long conditions with real upsides and downsides. They are contracted through body-group buildup from specific status powers, such as bites and ink. Incubation applies `st.incubating` for 7200 ticks, after which the affliction lasts for the rest of the run. Cures: a Scriptorium service for `mul_bp(500, P(d))` Lucre, or a blessed cure potion. Event: `ev.affliction_changed`.

| ID | Skins | Effects |
|---|---|---|
| `afflict.hunger` | vampirism, ghoul hunger, jiangshi | dark-vision +2 m; melee heals 1000 Bp of damage dealt; no natural HP regeneration while `light_at ≥ 12`; resist radiant −5000 |
| `afflict.beast` | lycanthropy, were-forms | once per Folio, or whenever composure is below 25%, gain `st.polymorphed` into a beast template for 30 s: melee × 15000, no items |
| `afflict.red_ink` | the Editors' forbidden ink | each dose unlocks one Marginalia power (doc 04) and lowers the hunter threshold by 1 (§11.6) |

### 16.5 Patrons and prayer

- Each genre has one patron Author, `patron.<genre_suffix>`, 16 in all, owned by doc 06.
- `res.favor` per patron is run-scoped, `-1000..=1000`. Each Erratum has a `prayer_timer` in ticks.
- Prayer resolution follows IVAN's table, with timers of +18000 ticks (pleased), +36000 (displeased), +90000 (helped anyway) and +180000 (angry). Doc 06 owns the table. Rolls use keyed `rng.magic`.
- Favor gains are × `(100 + devotion) / 100` and losses × `100 / (100 + devotion)`, with integer floor division.
- Praying to the local genre's patron applies effects × 15000.
- Prayers are components with dependency mythos whose origin genre is the patron's genre.
- Sacrifice value is `isqrt(item.value)`, doubled when the item's `origin_genre` equals the patron's genre.

---
## 17. Run structure, party scaling, enemy ranks and reference anchors

### 17.1 Run constants

| Constant | Value |
|---|---|
| `VOLUMES` | 5 |
| `FOLIOS_PER_VOLUME` | 5 |
| `BOSS_FOLIOS` | 5, 10, 15, 20, 25 |
| `FINAL_FOLIO` | 25, always `genre.ur_text` |
| `MARGINALIA_START` | 26 |
| `FOLIO_1_GENRE` | Standard mode: the host Erratum's Origin genre. Every seeded mode (Daily, Weekly, Seeded): the first candidate drawn from `rng.lattice` with `folio_index = 0`. |
| `EXIT_OFFERS` | 2; 3 when any party member has ITU ≥ 70 or carries a Lattice Lens |
| `LATTICE_CANDIDATES` | always exactly 3 drawn at the exit of each Folio, by weighted choice without replacement over eligible core genres using lattice edge weights (doc 12); the first `EXIT_OFFERS` are shown. The draw happens on the tick the exit UI opens. |
| `GENRE_REPEAT_RULE` | no genre twice in the same Volume; at most 3 appearances per run before Marginalia |
| `ERASURE_ONSET` | 420 s after Folio entry |
| `ERASURE_STEP` | 30 s; stack `k >= 1` is granted on tick `folio_entry_tick + 25200 + 1800 × (k - 1)` |
| `PARTY_MAX` | 4 |
| Loot | instanced per player |
| Target Folio duration | 180 to 420 s |
| Target full-run duration | 100 to 150 min |

Secret genres are never lattice candidates. They are reached only through secret exits (doc 12).

**Colophon.** Every Folio ends in a Colophon room (`room.colophon`, docs 12 and 14), the analog of Noita's Holy Mountain, placed before the exits. It offers one full heal, a full aether refresh, a shop, an altar of 3 boons with 1 reroll, and a Scriptorium station. Erasure accrual stops for the rest of the Folio once any Erratum enters it. Damaging its blocks marks the party for the run: an Editor hunter waits in every later Colophon, and after 3 hunter kills it becomes an elite hunter.

**Abridged mode** (`mode.abridged`, owner 14) is the shorter full-story run. It has 5 Volumes of 3 Folios each: two regular Folios plus the boss. Effective depth `d_eff(v, f) = (v - 1) × 5 + [1, 3, 5][f - 1]` feeds `P`, `T`, `H`, `S`, `ilvl` and `enemy_level`. Kill XP and loot quantity per Folio are × 16667 Bp. `EMENDATIONS_PER_VOLUME` stays 3. The final Folio is the Ur-Text at `d_eff = 25`. Target duration is 60 to 75 minutes. The 25-Folio Standard mode remains the full run.

### 17.2 Party-size tables (Bp, for party sizes 1, 2, 3, 4)

| Table | 1 | 2 | 3 | 4 |
|---|---|---|---|---|
| Enemy HP, ranks minion to champion | 10000 | 16000 | 21000 | 25500 |
| Enemy HP, ranks boss, editor, hunter | 10000 | 19000 | 28000 | 37000 |
| Enemy count (`enemy_count_bp`) | 10000 | 12500 | 15000 | 17500 |
| Enemy damage against Errata | 10000 | 11000 | 12000 | 13000 |

Enemy count per spawner is `floor_div(n_base × enemy_count_bp + carry, 10000)`, with the remainder kept as `carry` per room spawner.

### 17.3 Enemy ranks

| ID | HP (Bp) | Damage (Bp) | Kill XP (Bp) | DoT taken (Bp) | Control duration (Bp) | Notes |
|---|---|---|---|---|---|---|
| `rank.minion` | 4000 | 6000 | 3000 | 10000 | 10000 | swarm fodder |
| `rank.standard` | 10000 | 10000 | 10000 | 10000 | 10000 | |
| `rank.elite` | 30000 | 13000 | 30000 | 5000 | 7500 | one elite modifier (doc 13) |
| `rank.champion` | 60000 | 15000 | 60000 | 5000 | 5000 | one champion counter type (doc 05) |
| `rank.boss` | 200000 | 16000 | 200000 | 2500 | 2500 | genre mini-boss; immune to charmed, sleeping, confused |
| `rank.editor` | 400000 | 18000 | 400000 | 2500 | 2500 | Volume boss; same immunities |
| `rank.hunter` | 120000 | 20000 | 50000 | 2500 | 2500 | Erasure hunter; same immunities |

### 17.3a Champion counters and elite aspects

| ID | Meaning |
|---|---|
| `counter.ward_breaker` | hits deal × 30000 to the target's ward; breaking the ward applies `st.stunned` directly |
| `counter.disruptor` | applies `st.unraveled` and `st.silenced` directly for 180 ticks and stops the champion's regeneration |
| `counter.staggerer` | deals poise damage × 30000, even against poise-immune targets |
| `elite.warded` | ward equal to 50% of Max HP; fully regenerates 180 ticks after it last took damage; countered by `counter.ward_breaker` |
| `elite.surging` | attack and cast speed × 13000; heals 2% Max HP per second unless `st.unraveled` or `st.shocked`; countered by `counter.disruptor` |
| `elite.unstoppable` | immune to `st.staggered` from poise; countered by `counter.staggerer` |

Every counter type is available from at least one dependency-none source (an ammo type or a throwable) in every genre's shop and loot pool. The Folio exit preview shows the champion aspects of the next Folio. Doc 13 may add more elite modifiers; only these three are champion aspects.

### 17.4 Reference anchors

Doc 05, doc 11 and doc 13 tune all content against these Folio-1 anchors.

| Constant | Value |
|---|---|
| `REF_STD_ENEMY_HP_D1` | 60 HP for a standard-rank enemy of the reference role |
| `REF_ENEMY_HIT_D1` | 10 HP per hit before mitigation |
| `REF_ERRATA_AR_D1` | 20 AR per covered body part, full Common Normal medium armor at ilvl 1 |
| `REF_ENEMY_AR_D1` | 10 AR per body part |
| `REF_PLAYER_DPS_D1` | 30 HP per second sustained, Common Normal +0 Calling weapon, skill 20, all sub-attributes 40 |
| `REF_STANDARD_TTK_D1` | 2.0 s |
| `REF_KILLS_PER_FOLIO` | 40 standard-kill equivalents per player |

### 17.5 Friendly fire and co-op modes

| Constant or mode | Value |
|---|---|
| `FF_DIRECT_BP` | 5000: weapon, spell and gadget hits between `fac.errata` members |
| `FF_AREA_BP` | 10000: explosions, fire spread, surfaces, falling blocks and shock chains |
| `mode.ff_off` | sets the two constants to 0 and 5000 |
| `mode.ff_full` | sets both to 10000 |
| `mode.loot_shared` | one drop per kill, first pickup wins, as in Barony; instanced loot stays the default |

### 17.6 Ghosts and resurrection

- A dead Erratum whose party still has a living member becomes a ghost with `st.ghost`. A ghost has no collision, cannot deal damage or pick up items, and cannot be perceived by AI. It can open doors, press switches, ping, and apply `st.marked` (8 s duration, 600-tick cooldown).
- The body drops Remains, a `cat.corpse` item of 75,000 g.
- If an ally carries the Remains through the exit, the ghost resurrects at the next `ev.folio_entered` with 50% Max HP. Otherwise it resurrects there with 25% Max HP and loses one random equipped non-weapon item, chosen with keyed `rng.loot` (actor = the ghost's EntityId, seq 0).
- The run ends when no Erratum is alive. Events: `ev.player_died`, `ev.player_ghosted`, `ev.player_resurrected`.

### 17.7 Mounts

Mount archetypes are owned by doc 05. A mount is an entity with HP `template × H(d)`.

| Archetype | Movement | Seats |
|---|---|---|
| `arch.mount.runner` | walk 6000 mm/s; gallop 12000 mm/s, spending the mount's own stamina | 1 |
| `arch.mount.hover` | 14000 mm/s; crosses liquids | 2; the passenger can shoot |
| `arch.mount.glider` | horizontal 9000 mm/s; descent 2000 mm/s | 1 |

- Summoning a mount takes 30 ticks and needs 2600 mm of headroom. Doc 12 may tag rooms `room.no_mount`.
- A mounted Erratum has `st.mounted`, cannot be `st.hidden`, cannot place blocks, and has spread × 13000. A melee charge at 8000 mm/s or faster deals × 12500 damage and × 20000 poise damage.
- When the mount reaches 0 HP, the rider gains `st.knocked_down`. Events: `ev.mounted`, `ev.dismounted`.

### 17.8 The Margin trophies

The Margin has fixed architecture with cosmetic trophy and decoration slots per account, unlocked through Quires, achievements and the Codex. There is no free-form building in the hub. Trophies have no gameplay stats.

### 17.9 Invasions

Opt-in PvP by product-owner decision (DECISIONS.md, item 15). Docs 13, 14 and 16 own the details.

- **Opt-in.** The host sets `mode.invasion_open` at run start; the default is off. Daily, Weekly and every leaderboard-ranked run forbid it.
- **The invader** is a player from another session who joins as a Redactor: `fac.editors`, rank `rank.hunter`, scaled with `H(d)` and `T(d)` at the host's depth and the party-size tables of §17.2. The invader uses a fixed Editor kit chosen from kits unlocked on their account (doc 13), never their own run gear.
- **Window.** From Folio 3 onward, at most one invader per Folio, never in boss Folios, never earlier than 120 s after entry, never after any Erratum enters the Colophon. An invasion lasts at most 300 s.
- **Limits.** The invader cannot place or break blocks, open containers or pick up items.
- **Outcome.** If the invader reduces the last living Erratum to 0 HP, the party is Redacted instead of ending the run: every Erratum loses all Lucre and one random equipped non-weapon item (keyed `rng.loot`, actor = that Erratum, seq 0), then respawns at the Folio entrance with 50% Max HP. If the party kills the invader, every Erratum gets one guaranteed in-run Reliquary roll. Invader rewards are Quires and cosmetic Red Ink marks only.
- **Determinism.** The invader is replay slot 4 (§2.4). Invaders are the one exception to Folio-boundary drop-in: they join and leave mid-Folio.
- **Safety.** Text and voice chat with an invader require mute, block and report controls (doc 15 compliance).

---

## 18. Core simulation events

Doc 03 owns the event bus and the full payload schemas. These IDs are canonical:

| ID | Emitted when |
|---|---|
| `ev.folio_entered` | the party enters a Folio |
| `ev.folio_exited` | the party leaves through an exit |
| `ev.entity_spawned` | any entity is created |
| `ev.entity_despawned` | any entity is removed without dying |
| `ev.damage_dealt` | step 9 of §10.5, and every §10.6 entry point |
| `ev.healed` | healing applied |
| `ev.entity_killed` | lethal resolution reached step 3 of §10.8 |
| `ev.status_applied` | a status starts or gains a stack |
| `ev.status_expired` | a status ends |
| `ev.noise` | a sound is emitted into the sound grid (doc 09) |
| `ev.awareness_changed` | an AI awareness state changes (doc 09) |
| `ev.item_picked_up` | an item enters an inventory |
| `ev.item_dropped` | an item leaves an inventory onto the ground |
| `ev.item_used` | a consumable or activatable item is used |
| `ev.item_identified` | an instance or a type becomes known (doc 10) |
| `ev.item_crafted` | crafting completes (doc 07) |
| `ev.potion_brewed` | brewing completes (doc 08) |
| `ev.spell_cast` | a spell resolves (doc 06) |
| `ev.block_changed` | a block is placed, broken or transformed |
| `ev.liquid_reaction` | two liquids, or a liquid and a block, react (doc 08) |
| `ev.skill_gained` | a skill level rises |
| `ev.level_gained` | character level rises |
| `ev.contradiction_changed` | a component's Contradiction state changes |
| `ev.container_opened` | a chest or Reliquary opens (doc 11) |
| `ev.engine_spun` | a Fortune Engine resolves (doc 11) |
| `ev.reputation_changed` | faction reputation changes |
| `ev.erasure_stack` | `st.erasure` gained |
| `ev.hunter_spawned` | an Erasure hunter spawns (§11.6) |
| `ev.player_downed` | `st.downed` applied |
| `ev.player_revived` | `st.downed` removed by an ally |
| `ev.run_ended` | victory, death of the whole party, or abandonment |
| `ev.player_died` | an Erratum dies (§10.8) |
| `ev.player_ghosted` | an Erratum becomes a ghost (§17.6) |
| `ev.player_resurrected` | a ghost resurrects (§17.6) |
| `ev.ward_broken` | a ward reaches 0 (§10.5 step 7) |
| `ev.part_severed` | a body part is severed (§9.1) |
| `ev.part_reattached` | a severed part is reattached or replaced (§9.1) |
| `ev.minion_bound` | a minion joins its owner (§15.1) |
| `ev.minion_released` | a minion leaves or dies (§15.1) |
| `ev.mounted` | an Erratum mounts (§17.7) |
| `ev.dismounted` | an Erratum dismounts or is thrown (§17.7) |
| `ev.affliction_changed` | an affliction begins incubating, takes hold or is cured (§16.4) |
| `ev.invasion_started` | an invader joins (§17.9) |
| `ev.invasion_ended` | an invader leaves, dies, times out, or the party is Redacted (§17.9) |

---

## 19. Namespace ownership

| Namespace prefix | Owning document |
|---|---|
| `ax.`, `genre.` | 00 for the core table; 12 for full parameter vectors |
| `lattice.`, `room.`, `gpv.`, `biome.`, `hazard.`, `secret.`, `law.`, `zone.` | 12 |
| `attr.`, `sub.`, `skill.`, `talent.`, `body.`, `bp.`, `wound.`, `trait.`, `origin.`, `calling.`, `aug.`, `kin.`, `afflict.` | 04 |
| `dmg.`, `st.`, `res.` | 00 for the core lists; owners per §11.1 and §12 for behavior |
| `arch.melee.`, `arch.ranged.`, `arch.armor.`, `arch.offhand.`, `arch.mount.`, `arch.ward.`, `ammo.`, `move.`, `counter.` | 05 |
| `glyph.`, `spell.`, `school.`, `prayer.`, `ench.`, `summon.`, `patron.` | 06 |
| `mat.`, `q.`, `station.`, `recipe.`, `part.`, `block.`, `tooltier.`, `gadget.` | 07 (blocks shared with 03) |
| `liq.`, `gas.`, `ing.`, `alch.`, `apparatus.` | 08 |
| `noise.`, `aware.`, `disguise.`, `lock.` | 09 |
| `know.`, `idc.`, `appr.`, `buc.`, `curse.` | 10, with core values in 00 |
| `item.`, `rar.`, `affix.`, `perk.`, `mfr.`, `loot.`, `reliq.`, `engine.`, `shop.`, `cat.`, `slot.` | 11, with core values in 00 |
| `enemy.`, `ai.`, `fac.`, `boss.`, `npc.`, `director.`, `rank.`, `elite.` | 13, with core rank values in 00 |
| `run.`, `mode.`, `grade.`, `margin.`, `unlock.`, `echo.`, `score.`, `ach.` | 14; Echo acquisition in 15 |
| `cur.`, `ent.` | 00; account economy rules in 15 |
| `banner.`, `pass.`, `store.`, `juris.`, `bundle.`, `cosm.` | 15 |
| `net.`, `svc.` | 16 |
| `ui.`, `input.`, `access.` | 17 |
| `art.`, `aud.`, `nar.`, `vfx.` | 18 |
| `ev.`, `rng.` | 00 for the core lists; 03 for the event bus |
| `pillar.`, `kpi.` | 01 |
| `crate.`, `flag.`, `perf.` | 02 |
| `comp.`, `sys.`, `grid.` | 03 |
| `loc.` text keys | 18 (not content IDs, §1.10) |

---

## 20. Glossary

| Term | Meaning |
|---|---|
| Palimpsest | the world: a bottomless manuscript of overwritten stories |
| Folio | one floor of the Palimpsest, written in one genre |
| Volume | five Folios; the fifth holds an Editor boss |
| Marginalia | endless Folios after Folio 25 |
| Erratum, Errata | a player character and their kind: characters who fell out of their stories |
| Editor | an antagonist that erases Errata |
| Red Ink | the Editors' servants and their erasure pressure |
| The Margin | the meta hub between runs |
| Genre skin | the name, look, sound and dependency set a genre gives an archetype |
| Archetype | the genre-neutral mechanics of a piece of content |
| Axiom | one of ARCANA, TECHNE, MYTHOS, VERVE, 0 to 10 per genre |
| Local genre | the genre at a position: a bleed zone's genre or the Folio's genre |
| Contradiction | the penalty when a component needs more of an axiom than the local genre provides |
| Anachronism Edge | the bonus a beyond-its-time component gets against natives |
| Genre bleed | a zone where an adjacent Folio's genre leaks in (doc 12) |
| Rewrite | translating an item into the local genre's skin (doc 11) |
| Scriptorium | a Folio room for Rewriting and identification services (doc 12) |
| Emendation | the consumable that raises enhancement and re-levels the item |
| Reliquary | an earned loot box: in-run Reliquaries open with in-run keys (doc 11); account Reliquaries come from bosses, Folio completion and the Codex (doc 15) |
| Fortune Engine | an in-run capsule machine that takes Lucre; every capsule pays out an item (doc 11) |
| Inkwell | the meta gacha (doc 15) |
| Codex | the battle pass (doc 15) |
| Echo | a pre-built Erratum acquired through the Inkwell or a Recollection trial |
| Recollection | an Echo's unlock trial (doc 14) |
| Lattice | the weighted graph of genres that drives Folio order (doc 12) |
| Component | one functional part of an item or ability that carries its own dependencies |
| Keyed draw | a random draw seeded by (stream, Folio, actor, tick, seq) (§3.3) |
| Rank | an enemy's power tier: minion, standard, elite, champion, boss, editor, hunter |
| Kin | an Erratum's people: human, construct, undead, beastkin, fae, mutant |
| Patron | the Author of a genre, who answers prayers |
| Ghost | a dead Erratum still present while a party member lives |
| Remains | a dead Erratum's carryable body, which speeds resurrection |
| Quires | earned account currency for Margin unlocks |
| Ink | earned account currency for Inkwell pulls |

---
## 21. Acceptance tests

1. **xoshiro256++ vectors.** `Rng::from_seed64(0)` yields `0x53175d61490b23df`, `0x61da6f3dc380d507`, `0x5c0fdf91ec9a7bfc`, `0x02eebf8c3bbe5e1a` as its first four `next_u64` outputs.
2. **Second seed.** `Rng::from_seed64(0x0123456789ABCDEF)` yields `0xb2f2a310e96bd1c5`, `0xb54062465b950493`, `0x87aca4a9668814b0`, `0xf13d2e2448a9cffb`.
3. **Lemire range.** `Rng::from_seed64(42)` with five calls of `range(10000)` yields `8143, 3188, 9838, 7011, 7935`.
4. **FNV-1a.** `fnv1a64("rng.loot") = 0xdfec5b83d454f05e`. `fnv1a64("rng.worldgen") = 0x7f6e87856ef84a9a`. `fnv1a32("genre.cyberpunk") = 0x99b54427`. `fnv1a64("PALIMPSEST/DAILY/2026-10-11") = 0xc6f2e12ddca712ab`.
5. **mix64.** `mix64(0) = 0`. `mix64(1) = 0x5692161d100b05e5`.
6. **Stream seed.** For run seed `0x0123456789ABCDEF`: `stream_seed(run, "rng.loot", 3) = 0x3050a07115909f1c`, and that stream's first three outputs are `0x5bdc091e897b818b`, `0x53a8da29d78185d1`, `0x1fc02fe2ca3d6c88`. `stream_seed(run, "rng.worldgen", 1) = 0x276f33cc0100c222`.
7. **Keyed draw.** For the same run seed, `keyed_rng("rng.combat", 3, (3 << 40) | 5, 1000, 0)` is seeded with `0x4dcdb61bac362a8f`, and three calls of `range(10000)` yield `8822, 5948, 1792`.
8. **Keyed isolation.** The crit roll for (actor `(3 << 40) | 5`, tick 1000, seq 0) is identical whether 0 or 50 other entities drew from `rng.combat` earlier in the tick.
9. **Seed display.** Run seed `0x0123456789ABCDEF` displays as `028T-5CY4-TQKFF`. The inputs `028t 5cy4 tqkff` and `O28T5CY4TQKFF` decode to the same seed. `G28T-5CY4-TQKFF` is rejected.
10. **Rate accumulator.** 23 per second for 60 ticks at `k = 60` adds exactly 23. 7 per second for 600 ticks adds exactly 70. 850 mHP per second over 4 status pulses at `k = 4` adds exactly 850.
11. **Fixed point.** `sqrt_fx(Fx::from_int(4)) == Fx::from_int(2)`. `fx_from_milli(4300).raw == 18468359372`. `mul_fx(Fx::from_int(3), Fx::from_int(-2)) == Fx::from_int(-6)`. `div_fx(Fx::from_int(1), Fx::from_int(0))` panics.
12. **Trigonometry.** `sin_bam(0) = 0`, `sin_bam(16384) = 2^30`, `cos_bam(0) = 2^30`, `sin_bam(49152) = -2^30`.
13. **Derived stats.** All sub-attributes 40, level 1, VERVE 5: Max HP 85,000 mHP, HP regeneration 425 mHP per s, Max stamina 90, Max aether 61, Max composure 90, carry 45 kg, crit chance 500 Bp, Grimoire slots 4, complexity budget 8.
14. **Contradiction example.** The §5.6 worked example produces C = 4, potency 6000, failure 1000 and `edge_bp` 2400.
15. **Disconnection.** A Fairy Tale wand (`dep = {arcana}`, `Req = 8`) in Hardboiled Noir, held by a Fairy Tale-origin character with ATN 40, gives `raw_gap = 8`, `Tol = 2`, `C = 6`: Disconnected.
16. **Origin tolerance.** Every row of the §5.4 table sums to 4.
17. **Armor.** AR 100, `pen_bp` 0, a 20,000 mHP slash hit gives `DR_bp = 5555` and 8,890 mHP.
18. **Chip floor, NPC.** A pure-slash hit with `D2 = 10,000` mHP against an NPC with slash resistance 9000 and AR 1000: resistance leaves 1,000; armor DR is capped at 8500, leaving 150; the chip floor raises the total to 500 mHP.
19. **Chip floor, Erratum.** The same hit against an Erratum with slash resistance 9000 and AR 1000: the resistance cap of 5000 leaves 5,000; armor DR is capped at 8500, leaving 750; the chip floor raises the total to 1,000 mHP.
20. **Rarity weights.** At `d = 25`, `SER = 40`: `[7000, 4312, 1752, 582, 217, 87, 0]`. At `d = 1`, `SER = 100`: `[7000, 2860, 960, 285, 99, 37, 0]`.
21. **Power tables.** `P(25) = 79110`, `T(25) = 36145`, `H(25) = 202521`, `S(25) = 17200`, `P(41) = 200955`. Every committed row 1 to 40 matches §13.8.
22. **Skill curve.** Cumulative milli-XP from skill 0 to 25 is 220,000, and to 75 is 1,410,000.
23. **Level curve and kill XP.** Cumulative XP to reach level 11 is 27,500. Reference kill XP for a standard enemy at Folio 25 is 1,386.
24. **Jump apex.** With gravity 20000 mm/s², the simulated apex is at least 1,099 mm at VERVE 0 and between 1,999 and 2,001 mm at VERVE 10. With gravity 16000 mm/s² at VERVE 10 it is also between 1,999 and 2,001 mm.
25. **Zero-VERVE crit.** FAT 100, PRC 100 and `st.blessed` in `genre.redacted` give crit chance 0.
26. **Death's Door.** PRV 100 at VERVE 10 gives 6000 Bp. PRV 40 at VERVE 5 gives 1000 Bp. NPCs never roll it.
27. **Dodge.** AGI 150 gives 12 invulnerability ticks.
28. **Direct duration.** A stun (base 90 ticks) on a bearer with threshold 40 lasts 135 ticks. On a boss it lasts 33 ticks.
29. **ID validation.** The loader rejects `Genre.Cyberpunk`, `genre..x`, and any two content IDs with equal `fnv1a32`. The text key `loc.arch.melee.blade_long.skin.post_apocalypse.flavor` passes the text-key grammar and gets no `id32`.
30. **HP display.** A character with 400 mHP displays 1 HP.
31. **Folio isolation.** Generating Folio 7 from the same run seed, genre and bleed-neighbor genres yields byte-identical block data whether Folios 1 to 6 were played or skipped.
32. **Save and load.** Run 10,000 ticks with a fixed input log, save at tick 4,321, load and continue. The final `state_hash` equals the uninterrupted run's.
33. **No floats.** A lint step fails the build if `f32` or `f64` appears in `palimpsest_sim` outside `#[cfg(test)]` code and the documented conversion boundary module.
34. **No floats in content.** The content validator rejects any JSON number with a fraction or exponent part.
35. **Genre-native aether.** A character with every sub-attribute at 40 in `genre.cyberpunk` (aether axiom techne, TEC 9) regenerates 1,820 m-aether per second.
36. **Glyph dependency.** A spell glyph generated in Cyberpunk has `dep = {techne}` and `Req[techne] = 7`. One generated in Mythic Fantasy has `dep = {arcana}` and `Req[arcana] = 6`.
37. **Type hiding.** A generated trinket, ring and wand start at `know.unknown`. A staff, a belt and a longsword start at `know.classified`.
38. **Per-target sneak eligibility.** Erratum A is at `aware.alert` with guard G1, while guard G2's awareness of A is `aware.unaware`. A's melee hit on G2 is a sneak attack and draws no crit roll.
39. **Marginalia armor HP.** A full set of body, head, legs, hands and feet armor at ilvl 100 gives `gear_hp` of 4,538,772 mHP. The same set at ilvl 25 or lower gives 0.
40. **Genre spread.** The minimum pairwise L1 distance between the 12 core axiom vectors is 6, and every row of the §5.4 tolerance table still sums to 4.
41. **Ward depletion.** A ward of 10,000 mHP hit by 8,000 mHP of shock absorbs it fully only up to 6,666 mHP: `w = 12,000 > 10,000`, so shock damage becomes `8,000 - floor_div(10,000 × 10000, 15000) = 1,334` mHP and the ward breaks.
42. **Friendly fire.** In default mode, a rifle hit from one Erratum on another whose value before step 2.15 is 10,000 mHP leaves `D2 = 5,000`. A grenade blast with the same value before step 2.15 keeps `D2 = 10,000`.
43. **Forward vector.** `fwd(8192, 8192).x = -536870913` (negate first). `T[8192] = 759250125`.
44. **Weekly seed.** `fnv1a64("PALIMPSEST/WEEKLY/2026-W41") = 0x2e7e2bae906b8339`. The date 2026-10-11 falls in ISO week 2026-W41.

---

## 22. Invariants annex

```
Σ canon: ids ⊗ units ⊗ fixed-point ⊗ rng ⊗ formulas ⊗ ownership
W! sim.state ∈ {Fx, Milli, Bp, Bam, Ticks, i32, i64, u64}      N! f32|f64 ∈ sim.state ∪ content
W! overflow-checks = true ∀ profiles(palimpsest_sim)             N! silent wraparound
W! action-caused draw := keyed_rng(stream, folio, actor, tick, seq)
W! bulk host draw := sequential stream(owner)                    N! HashMap|HashSet ∈ sim.state
W! ∀ formula: order(bp-chain) == listed order                    N! reorder for speed
W! ∀ content id: regex ∧ fnv1a32 unique                          N! rename shipped id without alias
W! mechanics := archetype                                        W! dependency := genre skin
W! Contradiction ⊥ Palimpsestic                                  W! ur_text ⇒ C = 0 ∧ E = 0
W! Errata gear AR ∝ T(ilvl)                                      W! enemy HP ∝ H(d) ∧ enemy dmg ∝ T(d)
W! Fortune|Serendipity → in-run effects only                     N! Fortune|Serendipity → paid gacha odds | account currencies
W! Lucre ∩ account currencies = ∅                                N! real money → any randomized reward
N! premium currency                                              W! juris.paid_random_items_enabled = false
W! derived stats := §6.4 only                                    N! competing formula ∈ other docs
W! rendering reads sim                                           N! rendering → sim feedback
W! LLM text → cosmetic only                                      N! LLM output → sim state
W! glyph dep := origin_genre.aether_axiom                        W! min L1(core axiom vectors) ≥ 6
W! type-hidden ⇔ listed by an idc class                          W! sneak eligibility := per (attacker, target)
W! ∀ counter type: ∃ dep-none source ∈ every genre               N! input synthesized for a player
N! skin name ∈ {brands, third-party currency names, cur./ent. names, "Ink"}
```

---

## 23. Changelog

### canon-1.5 (2026-10-11)

Applied after implementing the Rust math crate.

- Defined checked shifts, a non-negating `ceil_div`, the simulation coordinate range, negate-first `fwd`, Fx display rounding, length-prefixed multi-field hashing, debug-only draw counters, strict seed and date decoding, the depth cap on `S`, `mul_bp` with `i64` table multipliers, and the ±1 Bam `atan2_bam` contract. Added tests 43 and 44.

### canon-1.4 (2026-10-11)

Applied after the product owner's per-system decisions (DECISIONS.md).

- Limited building: fixed terrain with `destructible` tags, `cat.placeable` replacing `cat.block`, resource nodes, `skill.fieldcraft` replacing Delving, and Margin trophies replacing the building plot.
- Opt-in PvP invasions with Redactor invaders, windows, limits, the Redacted outcome and replay slot 4.

### canon-1.3 (2026-10-11)

Applied after research R6.

- Added World Laws with law-bound components and tolerance grants, seam zones, Going Native, `st.unmoored`, and the Colophon room that ends every Folio.
- Restored the §5.9 genre identity table, which the 1.2 edit had failed to insert.

### canon-1.2 (2026-10-11)

Applied after the player-fantasy and inspiration-fidelity review, plus research R3 and R4.

- Adopted Earned Randomness, Paid Certainty: no premium currency; Quires, Ink and Vellum are earned only; the Quill is an entitlement; Fortune Engines are capsule machines.
- Added `aether_axiom` per genre so casting is native to each genre: Bandwidth runs on TECHNE in Cyberpunk, Qi on VERVE in Wuxia. Replaced the ARCANA ambient rows with AETHER and GLYPH rows.
- Changed four axiom vectors (Space Opera, Cosmic Horror, Post-Apocalypse, Fairy Tale) so core genres differ by at least 6. Renamed skins that collided with account currencies or third-party names.
- Added the genre identity table: physics multipliers, spawn light, palettes, music, enemy themes, materials and two manufacturers per genre.
- Added `cat.wand` and an exact type-hiding rule.
- Made sneak-attack eligibility per target, and added `ignore_armor`, elevation, flanking and friendly fire to the pipeline.
- Added per-type ward depletion, ward generators, champion counters and elite aspects.
- Added materials to item damage and armor, Marginalia armor HP, and Marginalia enhancement growth. Raised `ITEM_MULT_MAX` to 51870.
- Added item fields: manufacturer, gear tier, perk columns, illuminated, finder slot.
- Added severing and hand bindings, kin, afflictions, patrons and favor, breath, the surfaces and reactions table, sight concealment, two-channel light and spawn light.
- Added minions, mounts, ghosts and resurrection, co-op modes, the Margin plot and Abridged mode.
- Added skills Hacking, Delving, Awareness and Handling (36 total), Callings Netrunner, Beastbinder and Monk (13 total), damage types radiation and sonic (15 total), and statuses hacked, taunted, grappled, polymorphed, incubating, suffocating, mounted, bound and ghost.
- Added non-kill character XP sources, player-controlled status semantics and a stronger VERVE jump curve with air jumps.

### canon-1.1 (2026-10-11)

Applied after the implementability and math-balance reviews.

- Added normative integer and fixed-point primitives, an in-house `Fx`, `overflow-checks` in every profile, and `Ticks` for durations.
- Added the display-unit rule, the no-floats content encoding in mm, Bam, ticks and Bp, and Rust range notation.
- Split text keys from content IDs and defined `id32` collision handling for mods and aliases.
- Defined `EntityId` and item `uid` per Folio, phase keys, and the ascending-ID iteration rule.
- Replaced the trig table with 16,385 entries and defined folding, the forward vector and angle difference.
- Mixed stream seeds through `mix64` and added keyed draws for every action-caused roll. Assigned every canon roll to a stream. Made `rng.lattice` per-Folio with exactly 3 candidates.
- Added status active-interval semantics, the general rate accumulator, `in_combat`, `light_at`, session headers, the replay command log with `view_tick`, the save contract and `state_hash`.
- Moved movement constants to mm, mandated semi-implicit Euler, fixed jump height so every genre can mount 1 m, and defined fall damage and dodge caps.
- Made Origin tolerance fair (2/1/1/0 by sorted axioms) and slowed Attunement tolerance to every 30 points. Added overrides for Ur-Text, Unwritten and Palimpsestic items. Added local genre for bleed zones and personal VERVE.
- Made Contradiction failure base-component only, with a cadence for automatic weapons, and scoped potency per component.
- Rebuilt the damage pipeline with an explicit step 0, every multiplier placed, per-type modifiers, `pen_bp`, `i128` armor math, a 10% chip floor for Errata, blocking, lethal resolution order and other entry points.
- Separated item power `P`, threat `T`, enemy HP `H` and status power `S`. Errata armor now scales with `T`. Added source level, `ITEM_MULT_MAX`, the Emendation re-level rule and 3 Emendations per Volume.
- Reworked statuses: exposed and blinded now build through body and nerve, buildup respects resistance and `S`, post-proc protection never compounds, direct durations are clamped, DoTs use rate tables and the accumulator, shock chains cannot loop, and Erasure is fully specified.
- Rebalanced HP regeneration to scale with Max HP, normalized block stamina by depth, capped Errata physical resistance at 50%, reduced Death's Door and limited it to Errata, raised the Exotic base weight to 15, and normalized combat skill XP by damage dealt.
- Added the reference kill-XP formula, the co-op XP split, enemy ranks, party-size tables for bosses and damage, and Folio-1 reference anchors.
