# 00 — CANON: Single Source of Truth

As of 2026-10-11. Status: BINDING. Version: canon-1.0.

This document defines every shared identifier, enum, unit, constant and cross-system formula in PALIMPSEST. When any other spec document disagrees with this one, this one wins. When this one is silent, the owning document named in §18 decides. The orchestrator's `DESIGN_BRIEF.md` explains why; this document says exactly what.

## Table of contents

1. Conventions: identifiers, numeric types, rounding, units, coordinates, angles, text keys
2. Time and tick schedule
3. Random number generation
4. World and movement constants
5. Axioms, genres, Contradiction and Anachronism
6. Attributes, sub-attributes and master derived-stat formulas
7. Skills
8. Character level
9. Body plans and hit locations
10. Damage types, resistances, armor and the damage pipeline
11. Status effects and buildup
12. Resources
13. Items: categories, slots, rarity, quality, knowledge, curses, enhancement, power tables
14. Currencies
15. Factions
16. Callings, Origins and Echoes
17. Run structure constants
18. Core simulation events
19. Namespace ownership
20. Glossary
21. Acceptance tests
22. Invariants annex

---

## 1. Conventions

### 1.1 Identifiers

Every piece of content and every enum value has a string ID.

- Grammar (regex): `^[a-z][a-z0-9_]*(\.[a-z0-9_]+){1,5}$`. Maximum 64 bytes, ASCII only.
- The first segment is the **namespace**. §19 assigns each namespace to one owning document.
- IDs are permanent once shipped. Renaming requires an alias entry in `data/aliases.json`.
- The compact network and save form of an ID is `id32 = fnv1a32(utf8(id))`. The content loader must fail the build on any `id32` collision.
- Examples: `sub.precision`, `genre.cyberpunk`, `dmg.fire`, `st.bleeding`, `arch.ranged.rifle`, `mat.steel`.

FNV-1a constants:

| Variant | Offset basis | Prime |
|---|---|---|
| fnv1a32 | `0x811C9DC5` | `0x01000193` |
| fnv1a64 | `0xCBF29CE484222325` | `0x00000100000001B3` |

### 1.2 Numeric types

Simulation state never contains `f32` or `f64`. Rendering, audio and UI may use floats, but only for values derived from simulation state, never fed back.

| Type | Representation | Used for |
|---|---|---|
| `Fx` | signed Q32.32 fixed point, `fixed::types::I32F32` (raw `i64`) | positions, velocities, accelerations, distances, durations in seconds, physics |
| `Milli` | `i64`, value × 1000 | HP, damage, stamina, aether, composure, satiation, toxicity, XP |
| `Bp` | `i32` basis points, 10000 = 100% = ×1.0 | probabilities, percentage modifiers, multipliers, shares |
| `Bam` | `u16` binary angle, 65536 = one full turn | yaw, pitch, cone angles in the simulation |
| `Grams` | `i64` | mass |
| `Ml` | `i64` milliliters | liquid and gas volume |
| `DeciC` | `i32`, tenths of a degree Celsius | temperature |
| `Light` | `u8` 0..=15 | simulation light level |
| `Db` | `i32` whole decibels | sound level |
| `Tick` | `u64` | simulation time since run start |
| stats | `i32` | attribute, sub-attribute and skill values |
| money | `i64` | every currency, whole units |

### 1.3 Rounding rules

- `floor_div(a, b)` is Euclidean floor division. Every integer division in a game formula uses it unless a formula says `ceil_div` or `round_half_even`.
- `mul_bp(x, bp) = floor_div(x as i128 * bp as i128, 10000)`, narrowed back to the input width with a debug assertion that it fits.
- Several bp multipliers chain left to right, rounding after each step, in the exact order a formula lists them. Implementations must not reorder for speed.
- Clamp always happens last in a formula unless the formula places it elsewhere.
- **Per-second rates** become per-tick amounts with a remainder accumulator, so no rate drifts. For a rate `R` (per second, in Milli or integer units) and accumulator `acc` stored per (entity, rate):

```
total = R + acc
q     = floor_div(total, 60)
acc   = total - 60 * q
apply q this tick
```

- **Display rounding**: `Milli` values display as `floor(x / 1000)`, except current HP, which displays as `ceil(x / 1000)` so a living entity never shows 0 HP. Percentages from `Bp` display with one decimal place using `round_half_even`.

### 1.4 Units

Meters, seconds, grams, milliliters, degrees Celsius in tenths, decibels. One block is 1 m on each side. Speeds are m/s. Accelerations are m/s².

### 1.5 Coordinates

- Right-handed. +Y is up. +X is east. −Z is forward (north), matching Bevy.
- Block coordinates are `i32` triples. Block `(bx, by, bz)` occupies the half-open cube `[bx, bx+1) × [by, by+1) × [bz, bz+1)`.
- Chunk coordinate `cx = floor_div(bx, 32)`, and likewise for y and z. Local coordinate `lx = bx - 32*cx`.
- Local block index inside a chunk: `index = lx + 32 * (lz + 32 * ly)`, range 0..32767.
- Entity positions are `Fx` triples in meters. The entity origin is the bottom-center of its collision capsule.

### 1.6 Angles

- Simulation yaw and pitch are `Bam`. Yaw 0 faces −Z. Yaw increases counter-clockwise seen from above.
- Pitch is stored as `i16` reinterpretation of the `Bam` range and is clamped to `[-16000, 16000]`, about ±87.9°.
- `sin_bam` and `cos_bam` read a committed quarter-wave table of 16384 `i32` entries in Q2.30. Entry `i` equals `round_half_even(sin(2π·i/65536) · 2^30)`. `tools/gen_trig.py` generates it once with `mpmath` at 50 digits, and the file is committed. There is no interpolation at runtime.
- `atan2_bam(y, x)` uses 16-iteration CORDIC on `i64` with a committed arctangent table. Doc 02 specifies the function.
- `sqrt_fx` is an integer square root on the raw `i128` value, rounded down. Doc 02 specifies it.

### 1.7 Text keys

- Display text is never hard-coded. Each ID has `loc.<id>.name`, and optionally `loc.<id>.desc` and `loc.<id>.flavor`.
- A genre skin overrides with `loc.<id>.skin.<genre_suffix>.name`, where `genre_suffix` is the part after `genre.`, for example `loc.arch.melee.blade_long.skin.wuxia.name = "Jian"`.
- Name resolution order: active Folio genre skin, then the item's origin genre skin, then the archetype default name.

---

## 2. Time and tick schedule

- `TICK_HZ = 60`. One tick is 1/60 s.
- `DT` as `Fx` raw equals `71582788`, which is `floor(2^32 / 60)`. Physics integrates with `DT`. Gameplay rates use the accumulator rule in §1.3 instead of multiplying by `DT`.
- Rendering interpolates between the last two simulation states. It never extrapolates simulation state, except for client-side prediction of the local player as doc 16 specifies.

### 2.1 Tick schedule

A subsystem runs on tick `t` when `(t + phase) % period == 0`. For per-entity phases, `entity_index` is the entity's dense slot index (doc 03).

| Subsystem | Period (ticks) | Phase | Rate |
|---|---|---|---|
| Input apply, movement, collision, projectiles, melee traces | 1 | 0 | 60 Hz |
| Perception sight checks (per observer) | 3 | `entity_index % 3` | 20 Hz |
| AI think (per agent) | 6 | `entity_index % 6` | 10 Hz |
| Liquids cellular automaton | 6 | 1 | 10 Hz |
| Gases cellular automaton | 6 | 4 | 10 Hz |
| Fire spread | 12 | 2 | 5 Hz |
| Status pulse (DoT, HoT, buildup decay) | 15 | 0 | 4 Hz |
| Temperature diffusion | 30 | 5 | 2 Hz |
| Needs (satiation, toxicity decay, composure regen) | 60 | 7 | 1 Hz |
| Light relight | 1 | 0 | ≤ 4096 cell updates per tick, FIFO |
| Sound propagation | on emit | same tick | event-driven |
| Network snapshot (doc 16) | 2 | 0 | 30 Hz |

Doc 03 owns the exact order of systems inside one tick.

---

## 3. Random number generation

### 3.1 Algorithms

All simulation randomness uses **xoshiro256++** seeded through **SplitMix64**. The project ships its own implementation so no crate update can change sequences. Reference implementation:

```rust
pub fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E3779B97F4A7C15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
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
    /// Uniform integer in [0, n). Lemire's method. n must be > 0.
    pub fn range(&mut self, n: u64) -> u64 {
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
    /// Uniform integer in [lo, hi], inclusive.
    pub fn range_incl(&mut self, lo: i64, hi: i64) -> i64 {
        lo + self.range((hi - lo + 1) as u64) as i64
    }
    /// True with probability chance_bp / 10000. Always draws exactly once.
    pub fn roll_bp(&mut self, chance_bp: i32) -> bool {
        (self.range(10000) as i32) < chance_bp
    }
    /// Index chosen with probability proportional to weights. Draws exactly once.
    /// Panics if the total weight is 0.
    pub fn weighted(&mut self, weights: &[u32]) -> usize {
        let total: u64 = weights.iter().map(|&w| w as u64).sum();
        let mut r = self.range(total);
        for (i, &w) in weights.iter().enumerate() {
            if r < w as u64 { return i; }
            r -= w as u64;
        }
        unreachable!()
    }
    /// Fisher-Yates shuffle from the last index down.
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

`roll_bp` draws exactly once even when `chance_bp` is 0 or ≥ 10000. This keeps stream positions independent of the chance value.

### 3.2 Streams

Each subsystem owns one named stream and never draws from another subsystem's stream. Per-Folio streams are re-derived at each Folio entry, so the layout of Folio 7 depends only on the run seed, never on what happened in Folio 6.

```
stream_seed(run_seed, stream_name, folio_index) =
    run_seed
    ^ fnv1a64(stream_name)
    ^ (folio_index as u64).wrapping_mul(0x9E3779B97F4A7C15)
rng = Rng::from_seed64(stream_seed(...))
```

Run-scoped streams use `folio_index = 0`. Folio indices start at 1.

| Stream name | Scope | Owner | Draws for |
|---|---|---|---|
| `rng.lattice` | run | 12 | genre lattice walk, exit offers |
| `rng.appearance` | run | 10 | shuffling unknown-item appearances |
| `rng.worldgen` | folio | 12 | layout, rooms, blocks, dressing |
| `rng.spawn` | folio | 13 | initial and wave population |
| `rng.loot` | folio | 11 | drops, chests, Reliquaries, shops |
| `rng.engine` | folio | 11 | Fortune Engine reels |
| `rng.combat` | folio | 05 | crits, failure rolls, procs, spread seeds |
| `rng.status` | folio | 05 | status chances |
| `rng.ai` | folio | 13 | utility AI tie-breaks, idle behavior |
| `rng.stealth` | folio | 09 | pickpocket and lock rolls |
| `rng.craft` | folio | 07 | crafting quality rolls |
| `rng.alchemy` | folio | 08 | brewing variance, unknown liquid effects |
| `rng.magic` | folio | 06 | wild-magic surges, spell variance |
| `rng.ident` | folio | 10 | appraisal error, curse sensing |
| `rng.event` | folio | 12 | ambient and narrative events |
| `rng.progress` | run | 04 | talent offers, level-up choices |
| `rng.fire` | folio | 03 | fire ignition and spread |
| `rng.physics` | folio | 03 | debris, explosion block-break variation |
| `rng.offer.p<slot>` | run | 11 | per-player offer rolls (Scriptorium rewrites, reforge options, Fortune Engine faces); advances only when the player commits (§3.3b) |

Rendering and audio use a separate non-simulation generator, `rng.cosmetic`, which may be any fast PRNG. It must never influence simulation state.

### 3.3 Draw order

Within one tick, systems draw in the order doc 03 fixes. Within one system, entities are processed in ascending `EntityId` order. Any `HashMap` iteration that leads to a draw is forbidden. Use `BTreeMap` or sorted vectors.

### 3.3a Fixed draw counts

A single game action must draw the same number of values from a stream regardless of outcome, wherever practical. Rule: each attack hit draws exactly one crit roll, except sneak attacks, which draw none. Each projectile draws exactly two spread values. Systems with variable draw counts must document the count in their owning doc so replays and server verification stay exact.

### 3.3b Commit-only offer streams

Offers a player can re-open, such as Scriptorium rewrite choices, reforge options and the faces shown on a Fortune Engine before a spin, are generated from `rng.offer.p<slot>`, where `<slot>` is the player's party slot 0 to 3. The offer is generated once and stored. The stream advances only when the player commits to the offer. Re-opening a menu shows the stored offer and never rerolls, following Minecraft's enchanting seed.

### 3.4 Seeds

- A run seed is a `u64`. Its display form is 13 characters of Crockford Base32 in groups 4-4-5, for example `7KQ2-M9XD-PAH3T`.
- Daily seed: `fnv1a64("PALIMPSEST/DAILY/" + "YYYY-MM-DD")`, with the date in UTC.
- Weekly seed: `fnv1a64("PALIMPSEST/WEEKLY/" + "YYYY-Www")`, using the ISO week.
- Paid gacha never uses this generator. Doc 15 specifies a server-side ChaCha20 generator with commit-reveal.

---

## 4. World and movement constants

| Constant | Value | Notes |
|---|---|---|
| `BLOCK_SIZE` | 1 m | |
| `CHUNK_SIZE` | 32 blocks per axis | 32,768 blocks per chunk |
| `FOLIO_MAX_EXTENT` | 256 × 64 × 256 blocks (X × Y × Z) | 8 × 2 × 8 chunks |
| `FOLIO_TYPICAL_FOOTPRINT` | 96 to 192 blocks per horizontal side | doc 12 picks per genre |
| `GRAVITY` | 20.0 m/s² × genre `gravity_bp` | game-feel gravity, not Earth |
| `TERMINAL_VELOCITY` | 50.0 m/s | |
| `ACTIVE_ACTOR_CAP` | 256 per Folio | AI-driven entities simulated |
| `PROJECTILE_CAP` | 1024 per Folio | oldest non-player projectile is removed first |
| `GROUND_ITEM_CAP` | 2048 per Folio | beyond cap, items merge into a Lost Satchel |

### 4.1 Humanoid player body

| Parameter | Value |
|---|---|
| Capsule radius | 0.30 m |
| Standing height | 1.80 m |
| Standing eye height | 1.62 m |
| Crouched height | 1.20 m |
| Crouched eye height | 1.02 m |
| Step-up height | 0.55 m |
| Mass | 75,000 g plus carried mass |

### 4.2 Movement

| Movement | Speed (m/s) |
|---|---|
| Walk | 4.3 |
| Sprint | 6.5 |
| Crouch walk | 2.0 |
| Swim | 2.4 |
| Climb (ladders, vines, rope) | 2.4 |
| Backpedal | walk × 0.80 |
| Strafe | walk × 0.95 |

- Ground acceleration is 40 m/s². Air acceleration is 8 m/s².
- Jump apex height: `JUMP_H = 1.25 m × (10000 + (VRV - 5) × 400) / 10000`, where VRV is the Folio's VERVE axiom. Launch speed is `sqrt_fx(2 × g × JUMP_H)`.
- Safe fall height: `SAFE_FALL = 4.0 m × the same Verve factor`. Fall damage equals `(fall_height - SAFE_FALL) × 800 bp of MaxHP per meter`. Acrobatics reduces it as doc 04 specifies.
- Dodge: a 3.0 m burst over 12 ticks. It costs 20 stamina. Invulnerability starts on the dodge's first tick and lasts `8 + floor(max(0, AGI - 40) / 15)` ticks, so 8 ticks at AGI 40 and 12 ticks at AGI 100. A new dodge cannot start until 18 ticks after the previous one started.
- Final move speed is base speed × the move-speed multiplier in §6.4 × encumbrance × status multipliers, applied in that order.

---
## 5. Axioms, genres, Contradiction and Anachronism

### 5.1 Axioms

| ID | Name | Governs |
|---|---|---|
| `ax.arcana` | ARCANA | how well magic works: spells, wands, enchantments, magical effects |
| `ax.techne` | TECHNE | how well technology works: firearms, energy weapons, gadgets, implants, electronics |
| `ax.mythos` | MYTHOS | how strongly the supernatural operates: gods, prayer, relics, curses, undead, fiends, the uncanny |
| `ax.verve` | VERVE | how cinematic reality is: crits, acrobatics, jump height, lucky escapes |

Each axiom is an integer from 0 to 10 per genre.

### 5.2 Genres

The suffix after `genre.` is the **genre suffix** used in text keys. `gravity_bp` multiplies `GRAVITY`. `ambient_light` is the simulation light floor in open areas before light sources (doc 03). Doc 12 owns the full Genre Parameter Vector for each genre. The columns below are canonical and doc 12 must copy them exactly.

| # | ID | Display name | ARC | TEC | MYT | VRV | gravity_bp | ambient_light | Lucre skin | Aether skin | Tier |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | `genre.mythic_fantasy` | Mythic Fantasy | 8 | 2 | 6 | 6 | 10000 | 4 | Crowns | Mana | core |
| 2 | `genre.gothic_horror` | Gothic Horror | 5 | 3 | 9 | 3 | 10000 | 2 | Shillings | Sanguine | core |
| 3 | `genre.cosmic_horror` | Cosmic Horror | 6 | 4 | 10 | 1 | 10000 | 1 | Dollars | Lore | core |
| 4 | `genre.space_opera` | Space Opera | 3 | 10 | 3 | 7 | 8000 | 6 | Credits | Psi | core |
| 5 | `genre.cyberpunk` | Cyberpunk | 1 | 9 | 2 | 5 | 10000 | 3 | Cred | Bandwidth | core |
| 6 | `genre.post_apocalypse` | Post-Apocalypse | 1 | 6 | 2 | 4 | 10000 | 5 | Scrip | Flux | core |
| 7 | `genre.weird_west` | Weird West | 4 | 4 | 6 | 7 | 10000 | 6 | Bits | Hex | core |
| 8 | `genre.hardboiled_noir` | Hardboiled Noir | 0 | 5 | 1 | 2 | 10000 | 2 | Bucks | Nerve | core |
| 9 | `genre.wuxia` | Wuxia | 6 | 2 | 5 | 10 | 8000 | 5 | Taels | Qi | core |
| 10 | `genre.steampunk` | Steampunk | 3 | 7 | 3 | 6 | 10000 | 4 | Sovereigns | Pressure | core |
| 11 | `genre.fairy_tale` | Fairy Tale | 10 | 1 | 7 | 5 | 10000 | 6 | Fairy Gold | Glamour | core |
| 12 | `genre.mythic_antiquity` | Mythic Antiquity | 6 | 2 | 10 | 8 | 10000 | 7 | Drachmae | Ichor | core |
| 13 | `genre.oneiric` | Oneiric | 9 | 5 | 8 | 9 | 6000 | 4 | Motes | Lucidity | secret |
| 14 | `genre.redacted` | The Redaction | 2 | 8 | 8 | 0 | 10000 | 9 | Vouchers | Clearance | secret |
| 15 | `genre.unwritten` | The Unwritten | 0 | 0 | 0 | 0 | 10000 | 8 | none | Ink | secret |
| 16 | `genre.ur_text` | The Ur-Text | 5 | 5 | 5 | 5 | 10000 | 4 | Words | Ink | secret |

Special rules:

- `genre.ur_text` is the genre of Folio 25. No component suffers Contradiction there, and no Anachronism Edge applies there.
- `genre.unwritten` has no shops, no Lucre drops and no native enemies except Editors. Only Palimpsestic items, mundane components and innate abilities work fully.
- `genre.oneiric` has gravity-flip volumes, which doc 12 defines.
- `genre.redacted` is the Editors' home genre. It has no crits for anyone, because VERVE is 0 and the crit floor in §6.4 applies.

### 5.3 Dependencies and requirements

Every **component** of an item or ability has a dependency set `dep ⊆ {arcana, techne, mythos, verve}`. A component is the item's base function, one affix, one perk, one spell glyph, or one innate ability. Dependencies come from the **genre skin**, not from the archetype, because the skin says how the thing works in its world. A longbow skin of `arch.ranged.rifle` is mundane. A plasma-rifle skin of the same archetype depends on TECHNE. A staff-of-bolts skin depends on ARCANA.

| Component kind | Default `dep` |
|---|---|
| Mundane melee base, mundane armor base, bows, slings, thrown blades, food, raw materials, blocks | none |
| Firearms, energy weapons, gadgets, drones, turrets, implants, electronic tools, hacking, tech consumables | techne |
| Spells, wands, staves, magic texts, enchantment affixes, supernatural alchemical effects | arcana |
| Prayers, relics, holy or unholy affixes, curse effects, undead-bane, divine favors | mythos |
| Cinematic techniques: wall-run, double jump, qinggong glide, gun-kata, "rule of cool" exotic perks | verve |
| Mundane alchemical effects: healing, poison, stamina, acid, fire, smoke | none |

A component's **requirement** on axis `a`:

```
Req[a] = if a ∈ dep { max(1, origin_genre.axiom[a] - 2) } else { 0 }
```

`origin_genre` is the genre in which the item was generated or last Rewritten. Innate abilities of a creature use the creature's origin genre.

### 5.4 Tolerance

A wielder's tolerance on axis `a` is the sum below, capped at 6:

```
Tol[a] = origin_tol[a] + attune_tol + gear_tol[a] + talent_tol[a]
origin_tol[a] = 2 if errata_origin.axiom[a] >= 7, else 1 if >= 5, else 0
attune_tol    = clamp(floor_div(ATN - 40, 20), 0, 3)
```

`errata_origin` is the character's Origin genre (§16). NPC creatures have `origin_tol = 2` on every axis of their own origin genre where that genre's axiom is ≥ 5, and `attune_tol = 0`.

### 5.5 Contradiction

For each component, evaluated whenever the Folio changes, the wielder changes, or tolerance changes:

```
raw_gap[a] = max(0, Req[a] - folio.axiom[a])
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

- **Potency** multiplies everything the component produces: damage, healing, magnitude, duration, buildup, shield strength.
- **Failure** is rolled on `rng.combat` once per activation: per shot, per cast, per gadget use. Doc 05 defines a failed gun shot as a jam. Doc 06 defines a failed spell as a fizzle that spends full aether. Doc 07 defines a failed gadget as a spark that deals 5% of the user's MaxHP as `dmg.shock`.
- **Disconnected** components cannot activate. Their passive bonuses are off. The item shows a torn-page icon.
- Palimpsestic items (§13.3) ignore Contradiction entirely.
- Contradiction applies to native and foreign creatures alike. A foreign enemy that arrives through genre bleed carries its own Contradiction.

### 5.6 Anachronism Edge

```
E = if C <= 5 { min(6, max over a in dep of raw_gap[a]) } else { 0 }
edge_bp = E * 400
```

- Against a **native** target, meaning `target.origin_genre == folio.genre`, the component's damage and status buildup are multiplied by `10000 + edge_bp`. That is up to +24%.
- **Shock of the New:** the first hit from a component with `E >= 3` on each native non-boss target applies `st.staggered` regardless of poise.
- Anachronism never applies in `genre.ur_text`.

Worked example. A Cyberpunk smart pistol has `dep = {techne}` and `Req[techne] = max(1, 9 - 2) = 7`. It is carried into Fairy Tale, where TECHNE is 1, so `raw_gap = 6`. The wielder has a Cyberpunk origin, giving `origin_tol = 2`, and ATN 60, giving `attune_tol = 1`, so `Tol = 3` and `gap = 3`. The state is Contradicted: potency 7000 and failure 500. `E = min(6, 6) = 6`, so `edge_bp = 2400`. Against a native wolf, the damage factor is `0.70 × 1.24 = 0.868`, with a 5% jam chance and a guaranteed first-hit stagger.

### 5.7 Ambient axiom effects

These apply to everyone in the Folio, in addition to Contradiction.

| Axiom | Effect |
|---|---|
| ARCANA | Aether regeneration × `(10000 + (ARC - 5) × 1000)` Bp. Spell potency × `(10000 + (ARC - 5) × 200)` Bp. |
| TECHNE | Energy-weapon and gadget recharge × `(10000 + (TEC - 5) × 1000)` Bp. At TEC ≤ 2, worldgen places no electronic locks, cameras or turrets. |
| MYTHOS | Damage dealt by and to entities tagged undead, fiend, divine or eldritch × `(10000 + (MYT - 5) × 300)` Bp. Curse chance on generated items × `(10000 + (MYT - 5) × 1500)` Bp. Composure loss × `(10000 + (MYT - 5) × 1000)` Bp. |
| VERVE | Crit chance + `(VRV - 5) × 100` Bp. Jump height and safe-fall height × `(10000 + (VRV - 5) × 400)` Bp. Death's Door chance × `(10000 + (VRV - 5) × 1000)` Bp. |

---

## 6. Attributes and sub-attributes

### 6.1 Structure

Eight **primary attributes** each have three **sub-attributes**. Sub-attributes are stored. A primary is derived as `floor((s1 + s2 + s3) / 3)` using effective sub-attribute values.

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

- Stored (base) sub-attribute: integer 1..100.
- Effective sub-attribute: base plus modifiers, clamped to 1..150.
- Human baseline: 40. New characters start in the range 20..60 per sub-attribute (doc 04 owns creation).
- Every formula below uses the effective value. `d(x) = x - 40`.

### 6.3 Requirements

Items may require a primary attribute, for example `req: { attr.might: 55 }`. Shortfall `s = max(0, req - value)`. While wielding with a shortfall, damage, attack speed and spread recovery are each multiplied by `10000 - min(5000, s × 100)` Bp. Five points short costs 5%. Fifty or more points short costs 50%. This follows Shattered Pixel Dungeon's strength requirement.

### 6.4 Master derived-stat formulas

All doc-04-owned growth must feed these formulas, and no other document may define a competing formula for the same stat. `L` is character level. Values are effective sub-attributes. "Per second" rates use the §1.3 accumulator.

| # | Stat | Formula | At all-40, L1 |
|---|---|---|---|
| 1 | Max HP | `40 + TGH + L × (3 + floor(TGH / 20))` HP | 85 |
| 2 | Natural HP regeneration (after 5 s without taking damage) | `200 + RES × 10` mHP per s | 0.6 HP/s |
| 3 | Healing received | × `10000 + d(RES) × 50` Bp | ×1.00 |
| 4 | Max stamina | `50 + END + floor(L / 2)` | 90 |
| 5 | Stamina regeneration (after 1.0 s without spending) | `15 + floor(END / 5)` per s | 23/s |
| 6 | Sprint stamina drain | `max(4, 12 - floor_div(d(END), 10))` per s | 12/s |
| 7 | Max aether | `20 + FOC + L` | 61 |
| 8 | Aether regeneration | `500 + ATN × 20` m-aether per s, × ARCANA ambient | 1.3/s |
| 9 | Max composure | `50 + RSV` | 90 |
| 10 | Carry capacity | `25 + floor(PWR / 2)` kg | 45 kg |
| 11 | Move speed multiplier | `clamp(10000 + d(AGI) × 25, 8000, 12500)` Bp | ×1.00 |
| 12 | Weapon scaling bonus | `max(-5000, Σ floor_div(c_i × d(x_i), 100))` Bp, over the weapon's scaling coefficients `c_i` (Bp) on sub-attributes `x_i` | +0% |
| 13 | Crit chance | `clamp(500 + d(FAT) × 10 + d(PRC) × 5 + (VRV - 5) × 100 + gear, 0, 7500)` Bp | 5.0% at VRV 5 |
| 14 | Crit damage multiplier | `15000 + gear` Bp | ×1.50 |
| 15 | Spread multiplier | `clamp(10000 - d(PRC) × 40, 5000, 14000)` Bp | ×1.00 |
| 16 | Recoil multiplier | `clamp(10000 - d(GRP) × 40, 5000, 14000)` Bp | ×1.00 |
| 17 | Reload, swap and use-item speed | `clamp(10000 + d(SLT) × 50, 7000, 15000)` Bp; action duration is divided by this | ×1.00 |
| 18 | Poise | `20 + floor(BRN / 2) + armor poise` | 40 |
| 19 | Block stability (share of blocked damage turned into stamina loss instead of HP loss) | `clamp(5000 + d(BRN) × 50 + shield stability, 0, 10000)` Bp | 50% |
| 20 | Throw range multiplier | `10000 + d(PWR) × 50` Bp | ×1.00 |
| 21 | Climb speed multiplier | `10000 + d(GRP) × 50` Bp | ×1.00 |
| 22 | Body-status threshold: poisoned, diseased, irradiated, withering, corroded | `max(40, 100 + 2 × d(RES))` | 100 |
| 23 | Nerve-status threshold: bleeding, burning, frozen, shocked, stunned | `max(40, 100 + 2 × d(TGH))` | 100 |
| 24 | Mind-status threshold: feared, charmed, confused, sleeping, dread, berserk | `max(40, 100 + 2 × d(RSV))` | 100 |
| 25 | Arcane-status threshold: silenced, unraveled | `max(40, 100 + 2 × d(ATN))` | 100 |
| 26 | Spell complexity budget | `4 + floor(LOG / 10)` points | 8 |
| 27 | Grimoire slots | `2 + floor(MEM / 20)` | 4 |
| 28 | Skill XP multiplier | `10000 + d(MEM) × 50` Bp | ×1.00 |
| 29 | Spell potency multiplier | `10000 + d(ATN) × 50 + school_skill × 50` Bp, then × ARCANA ambient | ×1.00 at skill 0 |
| 30 | Cast interruption resistance (chance to keep casting when hit) | `clamp(2500 + d(FOC) × 75, 0, 9500)` Bp | 25% |
| 31 | Dark-vision radius | `2 + floor(SGT / 20)` m | 4 m |
| 32 | Visual detection range bonus | `d(SGT) × 50` Bp | +0% |
| 33 | Hearing sensitivity bonus | `floor_div(d(HRG), 5)` dB | +0 dB |
| 34 | Footstep indicator radius | `8 + floor(HRG / 10)` m | 12 m |
| 35 | Passive trap and secret detection (per second, within 3 m) | `clamp(500 + d(ITU) × 50, 0, 9000)` Bp | 5% |
| 36 | Curse sense on pickup | `clamp(1000 + d(ITU) × 100, 0, 9000)` Bp | 10% |
| 37 | Buy price multiplier | `clamp(10000 - d(CHM) × 30, 7000, 13000)` Bp | ×1.00 |
| 38 | Sell price share of base value | `clamp(3000 + d(CHM) × 30, 1500, 5000)` Bp | 30% |
| 39 | Threat generation multiplier | `10000 + d(MEN) × 100` Bp | ×1.00 |
| 40 | Disguise suspicion-rate multiplier | `clamp(10000 - d(GUI) × 100, 3000, 15000)` Bp | ×1.00 |
| 41 | Status proc chance multiplier | `10000 + d(FAT) × 50` Bp | ×1.00 |
| 42 | Loot rarity shift | `d(SER)` feeds §13.4 | 0 |
| 43 | Death's Door chance (once per Folio) | `clamp(1000 + d(PRV) × 100, 0, 7000)` Bp, then × VERVE ambient | 10% |
| 44 | Curse resistance on equip | `max(0, d(PRV) × 50)` Bp | 0% |

**Death's Door**: when damage would reduce HP to 0 or below and the character has not used Death's Door in this Folio, roll the chance on `rng.combat`. On success, set HP to 1 and grant 60 ticks of `st.deaths_door`, which makes the character invulnerable. The roll happens once per Folio whether it succeeds or not.

---

## 7. Skills

Skills are integers 0..100. Mastery tiers unlock at 25 (Apprentice), 50 (Journeyman), 75 (Expert) and 100 (Master). Doc 04 owns the perks granted at each tier and the XP events table.

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

Weapon archetype to skill mapping is owned by doc 05. The mapping is by archetype, never by skin, so a longbow and a rifle both train Marksmanship.

### 7.1 Skill growth

- XP to go from skill `s` to `s + 1`: `need(s) = 4000 + 400 × s` milli-XP. Reaching 25 from 0 takes 220,000 milli-XP. Reaching 75 takes 1,410,000. Reaching 100 takes 2,380,000.
- Every skill XP grant is multiplied by the Skill XP multiplier (§6.4 row 28) before being added.
- **Sub-attribute training**: each skill level gained adds one training mark to the governing sub-attribute. When marks reach `T(v) = 2 + floor(v / 20)`, where `v` is the current base value, the base value rises by 1 and marks reset to 0. Base values above 100 are never reached by training.

---

## 8. Character level

- XP to go from level `L` to `L + 1`: `xp_to_next(L) = 50 × L × (L + 3)`. Level 2 needs 200. Level 11 needs 27,500 cumulative. Level 31 needs 542,500 cumulative.
- There is no level cap. The design target is level 28 ± 3 on arrival at Folio 25 (doc 13 tunes kill XP to hit this).
- Each level-up grants 4 attribute points. Each point adds +1 to the base value of any sub-attribute, with at most +2 to the same sub-attribute per level-up and never above 100.
- Each even level grants 1 talent point. Doc 04 owns talents.

---
## 9. Body plans and hit locations

Every creature has one body plan. Each body part is a simulation hitbox capsule (doc 03 owns hitbox placement). `share` is the part's slice of Max HP used for its **condition** pool. `hit_mult` multiplies damage landing on that part. `precision` marks parts where the weapon's precision multiplier applies. Doc 04 owns wound states and their effects.

| Body plan ID | Parts: ID (share Bp, hit_mult Bp, precision) |
|---|---|
| `body.humanoid` | `bp.head` (1500, 10000, yes), `bp.torso` (3500, 10000, no), `bp.arm_l` (1250, 8500, no), `bp.arm_r` (1250, 8500, no), `bp.leg_l` (1250, 8500, no), `bp.leg_r` (1250, 8500, no) |
| `body.quadruped` | `bp.head` (1500, 10000, yes), `bp.torso` (4500, 10000, no), `bp.leg_fl` (1000, 8500, no), `bp.leg_fr` (1000, 8500, no), `bp.leg_hl` (1000, 8500, no), `bp.leg_hr` (1000, 8500, no) |
| `body.avian` | `bp.head` (1500, 10000, yes), `bp.torso` (3500, 10000, no), `bp.wing_l` (1500, 8500, no), `bp.wing_r` (1500, 8500, no), `bp.leg_l` (1000, 8500, no), `bp.leg_r` (1000, 8500, no) |
| `body.serpentine` | `bp.head` (2000, 10000, yes), `bp.body_front` (4000, 10000, no), `bp.body_rear` (3000, 9000, no), `bp.tail` (1000, 8500, no) |
| `body.insectoid` | `bp.head` (1500, 10000, yes), `bp.thorax` (3500, 10000, no), `bp.abdomen` (2600, 10000, no), `bp.leg_1` to `bp.leg_6` (400 each, 8000, no) |
| `body.amorphous` | `bp.core` (2000, 10000, yes), `bp.mass` (8000, 10000, no) |
| `body.construct` | `bp.core` (2000, 10000, yes), `bp.chassis` (5000, 10000, no), `bp.limb_1` to `bp.limb_4` (750 each, 8500, no) |
| `body.swarm` | `bp.mass` (10000, 10000, no). Area damage against a swarm is × 15000. |

- Mutations, implants and races may add parts such as `bp.tail`, `bp.arm_extra_1` or `bp.horn`. Added parts take their share proportionally from `bp.torso`, or from `bp.mass` or `bp.chassis` in plans without a torso.
- Bosses may flag extra parts as `bp.weakpoint_N` with `precision = yes`.
- Default weapon precision multiplier: ranged 15000 Bp, melee 12500 Bp. Each weapon may override (doc 05).

---

## 10. Damage types, resistances, armor and the damage pipeline

### 10.1 Damage types

| ID | Category | Default color | Builds status | Armor efficiency (Bp) |
|---|---|---|---|---|
| `dmg.slash` | physical | `#C9CED6` | `st.bleeding` | 10000 |
| `dmg.pierce` | physical | `#E8E2C8` | `st.exposed` | 7500 |
| `dmg.blunt` | physical | `#A68A64` | `st.stunned` (and poise damage ×1.5) | 5000 |
| `dmg.fire` | elemental | `#FF6A1A` | `st.burning` | 0 |
| `dmg.frost` | elemental | `#9BE7FF` | `st.frozen` (each hit also applies `st.chilled`) | 0 |
| `dmg.shock` | elemental | `#FFE14D` | `st.shocked` | 0 |
| `dmg.acid` | elemental | `#9AE13B` | `st.corroded` | 0 |
| `dmg.toxin` | biological | `#6B8E23` | `st.poisoned` | 0 |
| `dmg.radiant` | esoteric | `#FFF4C2` | `st.blinded` (vs undead, fiend: × 15000 damage) | 0 |
| `dmg.umbral` | esoteric | `#5B2A86` | `st.withering` | 0 |
| `dmg.psychic` | esoteric | `#FF5FD2` | `st.dread` | 0 |
| `dmg.arcane` | esoteric | `#3FE0D0` | `st.unraveled` | 0 |
| `dmg.true` | true | `#FFFFFF` | none; ignores resistance and armor | 0 |

- Colors are defaults. UI must also show a unique icon per type, so color is never the only signal (doc 17).
- Genre skins rename types in text only, for example `dmg.shock` is "Galvanic" in Steampunk. Doc 18 owns the skin names.

### 10.2 Resistances

- Each entity has `resist[t]` in Bp for every type except `dmg.true`.
- Range: −10000 (takes double) up to a cap of 7500 for player characters and 9000 for NPCs. An `immune: [types]` flag makes damage of that type 0 and skips buildup.
- Sources add linearly before the cap.

### 10.3 Armor

- **Armor rating (AR)** is an integer per body part, summed from equipment covering that part plus natural armor.
- **Penetration (PEN)** is an integer on each attack.
- Armor reduces physical types only, scaled by the type's armor efficiency.
- Armor is strong against many small hits and weak against few large hits, because the hit size is in the denominator.

### 10.4 The damage pipeline

Doc 05 owns the details and worked examples. The order of steps is canonical.

Inputs: base damage `B` in Milli, already including item power `P(ilvl)`, quality and enhancement. Type shares `share[t]` in Bp summing to 10000. Penetration `PEN`. Precision multiplier. Status powers. Flags.

1. **Additive bucket.** `add` = weapon scaling bonus (§6.4 row 12) + every "+% damage" affix + talents + buffs, all in Bp. `add` is clamped to at least −9000. `D1 = mul_bp(B, 10000 + add)`.
2. **Multiplicative chain**, in this exact order, each step `mul_bp`:
   1. precision multiplier, if the hit part is a precision part;
   2. crit multiplier, if the crit roll succeeded and the hit is not a sneak attack;
   3. sneak-attack multiplier (doc 09), if applicable. A sneak attack never also crits: it skips the crit roll entirely, which keeps the `rng.combat` draw count fixed per hit as described in §3.3a.
   4. Contradiction potency (§5.5);
   5. Anachronism, `10000 + edge_bp`, if the target is native (§5.6);
   6. MYTHOS ambient, if the attacker or target carries a qualifying tag (§5.7);
   7. part `hit_mult` (§9);
   8. target `st.exposed`, `10000 + 800 × stacks`;
   9. attacker `st.weakened`, `7500`, and attacker `st.empowered`, `12000`;
   10. difficulty and party-size damage multipliers (docs 14 and 16).

   The result is `D2`.
3. **Split by type.** `d[t] = mul_bp(D2, share[t])`. The flooring remainder `D2 - Σ d[t]` is added to the type with the largest share, with ties going to the earlier row of §10.1.
4. **Resistance.** For each type, `r = clamp(resist[t], -10000, cap)`. If the target is immune, `d[t] = 0`. Otherwise `d[t] = d[t] - mul_bp(d[t], r)`.
5. **Armor**, physical types only. `effAR = max(0, floor_div(AR_part × eff[t], 10000) - PEN)`. If `effAR > 0`:

   ```
   DR_bp = min(8500, floor_div(effAR × 10_000_000, effAR × 1000 + 4 × d[t]))
   d[t]  = d[t] - mul_bp(d[t], DR_bp)
   ```

   Example: AR 100, PEN 0, a 20 HP slash hit (20,000 mHP). `DR_bp = floor(10^9 / 180,000) = 5555`, so the hit becomes 8,890 mHP.
6. **Chip floor.** `total = Σ d[t]`. If at least one type was not immune, `total = max(total, floor_div(D2, 20))`. Every non-immune hit deals at least 5% of `D2`.
7. **Absorption.** `res.ward` (overshield) absorbs first, then HP takes the rest.
8. **Apply.** Subtract from HP. Subtract the same amount from the hit part's condition pool. Add poise damage (doc 05). Add status buildup (§11.2).
9. **Emit** `ev.damage_dealt` with the full breakdown, and `ev.entity_killed` if HP reached 0 and Death's Door did not trigger.

Healing is not damage. Healing amount × healing received (§6.4 row 3), then × `st.withering` (× 5000 when present), then capped at Max HP.

---

## 11. Status effects and buildup

### 11.1 Status list

Stacking rules:

- `refresh`: reapplying sets remaining duration to `max(remaining, new)`.
- `intensity N`: each application adds one stack, up to N stacks. All stacks share one duration, which refreshes on each application.
- `pool`: the status holds a quantity, such as an overshield amount, instead of stacks.

Thresholds: body, nerve, mind and arcane groups use §6.4 rows 22 to 25. `direct` means the status is only applied directly, never through buildup.

| ID | Category | Threshold group | Stacking | Base duration | Canonical effect | Owner |
|---|---|---|---|---|---|---|
| `st.bleeding` | DoT | nerve | intensity 5 | 6 s | 1.0% Max HP per s per stack as `dmg.true`; bandage or healing ≥ 10% Max HP ends it | 05 |
| `st.burning` | DoT | nerve | refresh | 4 s | 2.0% Max HP per s as `dmg.fire`; ignites flammable blocks; water ends it | 05 |
| `st.poisoned` | DoT | body | intensity 5 | 8 s | 0.75% Max HP per s per stack as `dmg.toxin` | 08 |
| `st.corroded` | DoT | body | intensity 5 | 10 s | AR × (10000 − 1000 × stacks); 0.5% Max HP per s as `dmg.acid` | 05 |
| `st.irradiated` | DoT | body | intensity 10 | 30 s | Max HP × (10000 − 200 × stacks) while active | 08 |
| `st.withering` | DoT | body | refresh | 8 s | healing received × 5000; 0.5% Max HP per s as `dmg.umbral` | 06 |
| `st.diseased` | debuff | body | refresh | 300 s | natural HP regen 0; stamina regen × 5000 | 04 |
| `st.staggered` | control | poise | refresh | 30 ticks | cannot act or move; interrupts casting | 05 |
| `st.stunned` | control | nerve | refresh | 90 ticks | cannot act or move | 05 |
| `st.frozen` | control | nerve | refresh | 120 ticks | cannot act or move; the next blunt hit deals × 15000 and ends it | 05 |
| `st.chilled` | debuff | direct | refresh | 3 s | move and attack speed × 8000 | 05 |
| `st.shocked` | control | nerve | refresh | 3 s | every 30 ticks a 6-tick micro-stun; each hit taken chains 20% of its damage as `dmg.shock` to the nearest other entity within 4 m | 05 |
| `st.knocked_down` | control | direct | refresh | 60 ticks | prone; cannot act; takes × 11000 damage | 05 |
| `st.rooted` | control | direct | refresh | 2 s | cannot move; can act | 06 |
| `st.slowed` | debuff | direct | refresh | 4 s | move speed × 7000 | 05 |
| `st.blinded` | debuff | direct | refresh | 4 s | vision radius 2 m; spread × 20000; AI loses sight targets | 05 |
| `st.deafened` | debuff | direct | refresh | 6 s | hearing −30 dB; no footstep indicators | 09 |
| `st.silenced` | control | arcane | refresh | 4 s | cannot cast spells | 06 |
| `st.disarmed` | control | direct | refresh | 3 s | active weapon unusable | 05 |
| `st.sleeping` | control | mind | refresh | 8 s | cannot act; any damage ends it; sneak attacks always apply | 06 |
| `st.charmed` | control | mind | refresh | 6 s | fights for the applier's faction | 06 |
| `st.feared` | control | mind | refresh | 4 s | flees from the source; cannot attack | 06 |
| `st.confused` | control | mind | refresh | 5 s | movement input rotated by a random angle from `rng.status` every 30 ticks | 06 |
| `st.berserk` | control | mind | refresh | 6 s | attacks the nearest entity; damage dealt × 13000 | 06 |
| `st.dread` | mental | mind | refresh | 6 s | composure −15 on application; composure regen 0 | 06 |
| `st.madness` | mental | special | until composure ≥ 25% | none | hallucinated entities; aim sway × 20000; whispers | 04 |
| `st.exposed` | debuff | direct | intensity 5 | 6 s | damage taken × (10000 + 800 × stacks) | 05 |
| `st.weakened` | debuff | direct | refresh | 6 s | damage dealt × 7500 | 05 |
| `st.marked` | debuff | direct | refresh | 8 s | visible through walls to all hostiles of the marker | 09 |
| `st.soaked` | debuff | direct | refresh | 10 s | shock buildup × 15000; fire buildup × 5000; ends burning | 08 |
| `st.oiled` | debuff | direct | refresh | 20 s | fire buildup × 20000 | 08 |
| `st.hungry` | need | special | while satiation ≤ 600 | none | stamina regen × 8000 | 04 |
| `st.starving` | need | special | while satiation = 0 | none | 1% Max HP lost per 5 s; no natural HP regen | 04 |
| `st.encumbered` | need | special | while load 100–125% | none | move speed × 8500; stamina regen × 7500 | 04 |
| `st.overburdened` | need | special | while load > 125% | none | move speed × 5000; no sprint, jump or dodge | 04 |
| `st.exhausted` | need | special | 2 s after stamina hits 0 | 2 s | no sprint, dodge or heavy attack | 04 |
| `st.unraveled` | debuff | arcane | refresh | 5 s | aether regen 0; −10 aether on application | 06 |
| `st.contradicted` | indicator | special | while any equipped component has C ≥ 3 | none | UI only | 12 |
| `st.hasted` | buff | direct | refresh | 8 s | move speed × 12500; attack and cast speed × 11500 | 06 |
| `st.regenerating` | buff | direct | refresh | 10 s | heals 2% Max HP per s | 06 |
| `st.shielded` | buff | direct | pool | 10 s | `res.ward` pool absorbs damage first | 06 |
| `st.invisible` | buff | direct | refresh | 10 s | sight detection × 1000; ends on attack | 09 |
| `st.hidden` | indicator | special | while undetected by every hostile | none | enables sneak attacks | 09 |
| `st.empowered` | buff | direct | refresh | 8 s | damage dealt × 12000 | 06 |
| `st.fortified` | buff | direct | refresh | 10 s | AR × 12000; poise × 12000 | 06 |
| `st.focused` | buff | direct | refresh | 10 s | spread × 7000; aim sway × 5000 | 08 |
| `st.levitating` | buff | direct | refresh | 10 s | gravity 0; vertical control 1.5 m/s | 06 |
| `st.blessed` | buff | direct | refresh | 30 s | crit chance +1000 Bp; all thresholds × 11000 | 06 |
| `st.inspired` | buff | direct | refresh | 10 s | effective VERVE +2 for this character, capped at 10 | 04 |
| `st.downed` | special | special | until revived or bled out | 30 s bleed-out | crawl only; allies revive in 3 s | 16 |
| `st.deaths_door` | special | special | refresh | 60 ticks | invulnerable | 04 |
| `st.erasure` | special | direct | intensity 10 | 120 s | at 10 stacks an Editor hunter spawns on the bearer | 13 |

Percentages "of Max HP" are of the bearer's Max HP. DoTs tick on the status pulse (every 15 ticks), dealing a quarter of the per-second amount per pulse, computed with `floor_div`. Bosses take DoT percentages × 2500, so 1% becomes 0.25%.

### 11.2 Buildup

- Each entity has a buildup meter per status that uses a threshold group. Meters start at 0.
- A hit with status power `p` adds `p' = p` × Contradiction potency × `(10000 + edge_bp)` against native targets × the attacker's status proc multiplier (§6.4 row 41), each step `mul_bp`, in that order. Buildup from blocked hits is × 5000.
- When a meter reaches its threshold, apply the status at base duration and reset the meter to 0. For the next 10 s that status's threshold is × 15000 on that entity.
- After 2 s without new buildup, a meter decays by 5% of its threshold per status pulse.
- **Direct application** ignores meters. Its duration is scaled by `max(2500, floor_div(10000 × 100, threshold))` Bp of base duration, so a target with threshold 200 suffers half duration. Bosses take direct control effects at × 2500 duration and are immune to `st.charmed`, `st.sleeping` and `st.confused`.

---

## 12. Resources

| ID | Type | Range | Canonical rule | Owner |
|---|---|---|---|---|
| `res.hp` | Milli | 0..Max HP | §6.4 rows 1–3; death at 0 | 04 |
| `res.stamina` | Milli | 0..Max stamina | spent by sprint, dodge, melee, block; §6.4 rows 4–6 | 04 |
| `res.aether` | Milli | 0..Max aether | spent by spells and arcane items; displayed under the genre's Aether skin (§5.2) | 06 |
| `res.composure` | Milli | 0..Max composure | lost to horror and psychic effects; at 0 applies `st.madness` | 04 |
| `res.satiation` | integer | 0..2400 | −1 per second; starts at 1800 each run; `st.hungry` at ≤ 600; `st.starving` at 0 | 04 |
| `res.toxicity` | Milli | 0..100 | raised by potions and stims; −0.5 per second; at 100 applies 3 stacks of `st.poisoned` every 5 s | 08 |
| `res.poise` | Milli | 0..Poise | reduced by impact; at 0 applies `st.staggered` and refills | 05 |
| `res.ward` | Milli | 0..pool | overshield; absorbs damage before HP | 06 |

Composure regenerates 1 per second while out of combat for 5 s and standing on a cell with light ≥ 8.

---
## 13. Items

### 13.1 Categories

| ID | Meaning | Owner |
|---|---|---|
| `cat.weapon_melee` | melee weapons | 05, 11 |
| `cat.weapon_ranged` | every ranged weapon archetype, including bows, guns, wands and staves | 05, 11 |
| `cat.armor` | head, body, hands, legs, feet pieces | 05, 11 |
| `cat.offhand` | shields, foci, lanterns, off-hand sidearms | 05, 11 |
| `cat.accessory` | rings, amulets, trinkets, belts, back items | 11 |
| `cat.implant` | cybernetic and grafted augments | 04, 11 |
| `cat.ammo` | ammunition | 05 |
| `cat.potion` | drinkable or throwable liquids in containers | 08 |
| `cat.text` | scroll-like single-use texts (scrolls, writs, data shards, talismans) | 06, 10 |
| `cat.book` | permanent learning: glyph tomes, recipe books, lore | 06, 07 |
| `cat.food` | satiation items | 04 |
| `cat.throwable` | grenades, bombs, flasks, throwing knives | 05, 08 |
| `cat.gadget` | deployables: turrets, drones, traps, mines | 07 |
| `cat.tool` | pickaxes, lockpicks, hacking decks, rope, bandages | 07, 09 |
| `cat.material` | crafting materials | 07 |
| `cat.ingredient` | alchemical ingredients | 08 |
| `cat.container` | bottles, flasks, bags, satchels | 08, 11 |
| `cat.block` | placeable blocks | 03, 07 |
| `cat.key` | keys and keycards of any skin | 11 |
| `cat.currency` | Lucre piles | 11 |
| `cat.quest` | narrative items | 12 |

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
| `slot.offhand` | off-hand items; empty when a two-handed weapon is active |
| `slot.implant_head`, `slot.implant_torso`, `slot.implant_arms`, `slot.implant_legs` | implants |

The quickbar has 4 slots without a belt and up to 8 with one.

### 13.3 Rarity

| ID | Name | Color | Affixes | Special | Base weight | Power (Bp) | Value multiplier |
|---|---|---|---|---|---|---|---|
| `rar.common` | Common | `#E6E6E6` | 0 | none | 7000 | 10000 | ×1 |
| `rar.uncommon` | Uncommon | `#3CC45A` | 1 | none | 2200 | 10500 | ×2 |
| `rar.rare` | Rare | `#3D8BFF` | 2 | none | 600 | 11000 | ×4 |
| `rar.epic` | Epic | `#A64DFF` | 3 | none | 150 | 11500 | ×8 |
| `rar.legendary` | Legendary | `#FF9A1F` | 3 | one legendary perk | 45 | 12500 | ×16 |
| `rar.exotic` | Exotic | `#FFD84A` | 2 | one fixed named intrinsic; one per type equipped at a time | 5 | 12500 | ×32 |
| `rar.palimpsestic` | Palimpsestic | animated prismatic gradient | 4 | immune to Contradiction; re-skins itself to every Folio | 0 | 13000 | ×64 |

Palimpsestic items come only from Editor bosses, Volume-completion rewards and special secret rooms (doc 11).

### 13.4 Rarity roll

For a generic drop at Folio depth `d` with the finder's Serendipity `SER`:

```
w[0] = 7000
w[r] = max(0, floor_div(base[r] × (10000 + (d - 1) × 400 × r + (SER - 40) × 50 × r), 10000))   for r = 1..6
rarity = rng.loot.weighted(w)
```

`base` is the base-weight column of §13.3. In co-op, each player's instanced drop uses that player's own SER. Doc 11 adds per-source shifts on top. Reference values: at `d = 25`, `SER = 40`, the weights are `[7000, 4312, 1752, 582, 217, 29, 0]`.

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

Unknown-appearance classes. In each run, `rng.appearance` shuffles the appearance pool of each class onto that class's item types (doc 10):

| ID | Item types | Appearance pool examples |
|---|---|---|
| `idc.potion` | every potion type | colors: crimson, murky green, ... |
| `idc.text` | every scroll-like text type | sigils, glyph scripts, file hashes |
| `idc.ring` | every ring type | stones and metals |
| `idc.amulet` | every amulet type | pendant shapes |
| `idc.wand` | every wand and staff type | woods and alloys |
| `idc.implant` | every implant type | manufacturer stamps |
| `idc.relic` | every trinket type | carvings |

An appearance descriptor is genre-neutral. The genre skin renders it: a crimson potion is a crimson flask in Fantasy, a crimson-capped injector in Cyberpunk and a crimson pill in Wuxia. Type knowledge is shared across genres and across the whole party.

Weapons, armor, off-hands, accessories other than rings and amulets, tools, materials and blocks are never type-hidden. They start at `know.classified`, because only their magic is hidden, following Barony 5.0. Only the seven unknown-appearance classes above start at `know.unknown`.

Instance knowledge states, in order:

| ID | Player knows |
|---|---|
| `know.unknown` | appearance only |
| `know.classified` | item type, for example "Potion of Healing" or "Longsword" |
| `know.appraised` | rarity and a value range |
| `know.identified` | everything: affixes, enhancement, curse state, charges |

Curse states:

| ID | Effect |
|---|---|
| `buc.blessed` | positive effects × 12500 for consumables; equipment gains +1 enhancement |
| `buc.uncursed` | normal |
| `buc.cursed` | cannot be unequipped until uncursed; enhancement −1 to −3; carries one curse affix (doc 10) |

Genre skins rename curse states, for example Blessed / — / Hexed, or Signed / — / Corrupted.

### 13.7 Enhancement

- Range −3 to +15. The consumable that raises it is an **Emendation** (doc 11), the analog of Shattered Pixel Dungeon's Scroll of Upgrade.
- Each level multiplies base stats by `10000 + 600 × n` Bp, so +10 is ×1.6 and −2 is ×0.88.
- An Emendation used on a cursed item removes the curse instead of adding a level.

### 13.8 Item level and power tables

- Item level `ilvl` equals the Folio index where the item was generated, 1 to 25 for the main run and 26 or more in Marginalia.
- **P(d)**, item and enemy-HP power, in Bp. Used for weapon damage, armor rating and enemy HP.
- **T(d)**, threat, in Bp. Used for enemy damage. It grows at the pace of player Max HP so incoming damage stays fair.
- Beyond the table: `P(d) = floor_div(P(d-1) × 10600, 10000)` and `T(d) = floor_div(T(d-1) × 10400, 10000)`.

| d | P(d) | T(d) | d | P(d) | T(d) |
|---|---|---|---|---|---|
| 1 | 10000 | 10000 | 21 | 56044 | 29177 |
| 2 | 10900 | 10550 | 22 | 61088 | 30782 |
| 3 | 11881 | 11130 | 23 | 66586 | 32475 |
| 4 | 12950 | 11742 | 24 | 72578 | 34261 |
| 5 | 14115 | 12388 | 25 | 79110 | 36145 |
| 6 | 15386 | 13069 | 26 | 83856 | 37590 |
| 7 | 16771 | 13788 | 27 | 88887 | 39093 |
| 8 | 18280 | 14546 | 28 | 94220 | 40656 |
| 9 | 19925 | 15346 | 29 | 99873 | 42282 |
| 10 | 21718 | 16190 | 30 | 105865 | 43973 |
| 11 | 23673 | 17081 | 31 | 112216 | 45731 |
| 12 | 25804 | 18020 | 32 | 118948 | 47560 |
| 13 | 28126 | 19012 | 33 | 126084 | 49462 |
| 14 | 30658 | 20057 | 34 | 133649 | 51440 |
| 15 | 33417 | 21160 | 35 | 141667 | 53497 |
| 16 | 36424 | 22324 | 36 | 150167 | 55636 |
| 17 | 39703 | 23552 | 37 | 159177 | 57861 |
| 18 | 43276 | 24848 | 38 | 168727 | 60175 |
| 19 | 47171 | 26214 | 39 | 178850 | 62582 |
| 20 | 51416 | 27656 | 40 | 189581 | 65085 |

- Enemy level: `enemy_level(d) = d + floor_div(d, 5)`.
- Enemy effective sub-attributes: template value `+ floor_div(3 × (d - 1), 2)`, capped at 150.

### 13.8a One material table

Blocks and items share one material table owned by doc 07, following IVAN. A block is a material plus a shape. An item component is a material plus a volume. Mining a block yields that block's material, and an item's mass is its volume times its material density.

### 13.9 Item instance contract

Doc 19 owns the full schema. Every item instance carries at least:

| Field | Type | Meaning |
|---|---|---|
| `uid` | u64 | unique within the run, assigned in creation order |
| `template` | ID | archetype or unique template |
| `origin_genre` | ID | the skin's genre; drives requirements (§5.3) |
| `rarity` | ID | §13.3 |
| `quality` | ID | §13.5 |
| `ilvl` | i32 | §13.8 |
| `enhancement` | i32 | §13.7 |
| `buc` | ID | §13.6 |
| `knowledge` | ID plus flags | §13.6; flags for rarity, enhancement, curse state, each affix, charges |
| `affixes` | list of {id, tier, roll_bp} | doc 11 |
| `parts` | list of {slot, part_id} | assembled items only (doc 07) |
| `materials` | list of material IDs | doc 07 |
| `durability` | i32 current and max | doc 07 |
| `charges` | i32 current and max | wands, gadgets |
| `stack` | i32 | stackables |
| `mass_g` | Grams | |
| `value` | i64 Lucre | base value before buy and sell multipliers |

---

## 14. Currencies

| ID | Name | Scope | Source | Notes |
|---|---|---|---|---|
| `cur.lucre` | Lucre | run | drops, sales | displayed under the genre's Lucre skin (§5.2); one shared value across genres; lost when the run ends |
| `cur.ink` | Ink | account | run score, Codex pages, achievements | soft meta currency (doc 14) |
| `cur.gilt` | Gilt | account | real-money purchase; small amounts from Codex | premium currency (doc 15) |
| `cur.quill` | Quill | account | Codex, events, exchange | one standard Inkwell pull |
| `cur.gilded_quill` | Gilded Quill | account | Gilt exchange, Codex | one featured-banner Inkwell pull |
| `cur.vellum` | Vellum | account | duplicate Echoes and cosmetics | spends in the Vellum exchange (doc 15) |

Lucre never converts into any account currency. In-run Reliquaries and Fortune Engines use only Lucre and in-run keys.

---

## 15. Factions

| ID pattern | Members | Default stance toward `fac.errata` |
|---|---|---|
| `fac.errata` | player characters, allied Errata, hirelings | allied |
| `fac.editors` | Editors and their Red Ink servants; appear in every genre | hostile; hostile to natives that are carrying Contradicted items |
| `fac.native.<genre_suffix>.<group>` | sapient native factions, 2 to 4 per genre (doc 13) | per faction |
| `fac.wild.<genre_suffix>` | beasts and mindless monsters | hostile |
| `fac.margin` | hub NPCs | allied, non-combat |
| `fac.neutral` | wandering vendors and non-combatants | neutral until attacked |

Reputation per faction is an integer from −1000 to +1000 and resets each run.

| Range | Stance |
|---|---|
| −1000 to −500 | hostile |
| −499 to −101 | unfriendly |
| −100 to +100 | neutral |
| +101 to +499 | friendly |
| +500 to +1000 | allied |

---

## 16. Callings, Origins and Echoes

- **Origin**: the genre an Erratum fell out of. At creation the player picks one of the 12 core genres. Secret genres become Origin options through meta unlocks (doc 14). The Origin sets tolerance (§5.4), sub-attribute modifiers that sum to 0 and stay within ±6 each, +10 to one Origin skill, and one Origin trait (doc 04).
- **Calling**: the class. Each Calling sets three major skills to 20 and three minor skills to 10. All other skills start at 0. Doc 04 owns kits and talent trees.

| ID | Identity | Major skills | Minor skills |
|---|---|---|---|
| `calling.vanguard` | heavy melee frontliner | hafted, armor, shield | athletics, leadership, smithing |
| `calling.duelist` | agile swordfighter | blade, acrobatics, unarmed | sneak, speech, throwing |
| `calling.gunslinger` | precision shooter | marksmanship, acrobatics, throwing | tinkering, speech, security |
| `calling.grenadier` | heavy weapons and explosives | heavy_arms, throwing, armor | tinkering, athletics, medicine |
| `calling.spellwright` | spell designer and caster | evocation, spellcraft, alteration | abjuration, inscription, divination |
| `calling.alchemist` | brewer and bomber | alchemy, throwing, medicine | survival, restoration, appraisal |
| `calling.shade` | stealth assassin and thief | sneak, blade, security | pickpocket, acrobatics, illusion |
| `calling.artificer` | crafter, gunsmith, gadgeteer | tinkering, smithing, marksmanship | security, inscription, appraisal |
| `calling.warden` | protector and healer | shield, abjuration, restoration | polearm, devotion, leadership |
| `calling.oracle` | seer, priest, summoner | divination, devotion, conjuration | appraisal, illusion, speech |

- **Echo**: a pre-built Erratum. An Echo fixes an Origin, a Calling, a signature trait and cosmetics. It is acquired through the Inkwell (doc 15) or through its Recollection trial (doc 14). Launch roster: 24 Echoes, 2 per core genre. ID pattern `echo.<name>`.

---

## 17. Run structure constants

| Constant | Value |
|---|---|
| `VOLUMES` | 5 |
| `FOLIOS_PER_VOLUME` | 5 |
| `BOSS_FOLIOS` | 5, 10, 15, 20, 25 |
| `FINAL_FOLIO` | 25, always `genre.ur_text` |
| `MARGINALIA_START` | 26 |
| `EXIT_OFFERS` | 2; 3 when any party member has ITU ≥ 70 or carries a Lattice Lens |
| `GENRE_REPEAT_RULE` | no genre twice in the same Volume; at most 3 appearances per run before Marginalia |
| `ERASURE_ONSET` | 600 s after Folio entry |
| `ERASURE_STEP` | every 60 s after onset, every Erratum gains 1 stack of `st.erasure` |
| `PARTY_MAX` | 4 |
| Enemy HP by party size 1..4 (Bp) | 10000, 16000, 21000, 25500 |
| Enemy count by party size 1..4 (Bp) | 10000, 12500, 15000, 17500 |
| Loot | instanced per player |
| Target Folio duration | 180 to 420 s |
| Target full-run duration | 100 to 150 min |

---

## 18. Core simulation events

Doc 03 owns the event bus and the full payload schemas. These IDs are canonical:

| ID | Emitted when |
|---|---|
| `ev.folio_entered` | the party enters a Folio |
| `ev.folio_exited` | the party leaves through an exit |
| `ev.entity_spawned` | any entity is created |
| `ev.entity_despawned` | any entity is removed without dying |
| `ev.damage_dealt` | step 9 of §10.4 |
| `ev.healed` | healing applied |
| `ev.entity_killed` | HP reached 0 |
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
| `ev.liquid_reaction` | two liquids or a liquid and a block react (doc 08) |
| `ev.skill_gained` | a skill level rises |
| `ev.level_gained` | character level rises |
| `ev.contradiction_changed` | a component's Contradiction state changes |
| `ev.container_opened` | a chest or Reliquary opens (doc 11) |
| `ev.engine_spun` | a Fortune Engine resolves (doc 11) |
| `ev.reputation_changed` | faction reputation changes |
| `ev.erasure_stack` | `st.erasure` gained |
| `ev.player_downed` | `st.downed` applied |
| `ev.player_revived` | `st.downed` removed by an ally |
| `ev.run_ended` | victory, death of the whole party, or abandonment |

---

## 19. Namespace ownership

| Namespace prefix | Owning document |
|---|---|
| `ax.`, `genre.` | 00 (core table), 12 (full parameter vectors) |
| `lattice.`, `room.`, `gpv.`, `biome.`, `hazard.`, `secret.` | 12 |
| `attr.`, `sub.`, `skill.`, `talent.`, `body.`, `bp.`, `wound.`, `trait.`, `origin.`, `calling.`, `mut.` | 04 |
| `dmg.`, `st.`, `res.` | 00 (core), 05 (combat statuses), with owners per §11.1 and §12 |
| `arch.melee.`, `arch.ranged.`, `arch.armor.`, `arch.offhand.`, `ammo.`, `move.` | 05 |
| `glyph.`, `spell.`, `school.`, `prayer.`, `ench.` | 06 |
| `mat.`, `q.`, `station.`, `recipe.`, `part.`, `block.`, `tooltier.` | 07 (blocks shared with 03) |
| `liq.`, `ing.`, `alch.`, `apparatus.` | 08 |
| `noise.`, `aware.`, `disguise.`, `lock.` | 09 |
| `know.`, `idc.`, `appr.`, `buc.`, `curse.` | 10 (core values in 00) |
| `item.`, `rar.`, `affix.`, `perk.`, `mfr.`, `loot.`, `reliq.`, `engine.`, `shop.`, `cat.`, `slot.` | 11 (core values in 00) |
| `enemy.`, `ai.`, `fac.`, `boss.`, `npc.`, `director.` | 13 |
| `run.`, `mode.`, `grade.`, `margin.`, `unlock.`, `echo.`, `score.` | 14 (Echo acquisition in 15) |
| `cur.` | 00; account economy rules in 15 |
| `banner.`, `pass.`, `store.`, `juris.`, `bundle.`, `cosm.` | 15 |
| `net.`, `svc.` | 16 |
| `ui.`, `input.`, `access.` | 17 |
| `art.`, `aud.`, `nar.`, `vfx.` | 18 |
| `ev.`, `rng.` | 00 (core), 03 (bus) |
| `loc.` | 18 |

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
| Contradiction | the penalty when a component needs more of an axiom than the Folio provides |
| Anachronism Edge | the bonus a beyond-its-time component gets against natives |
| Genre bleed | a zone where an adjacent Folio's genre leaks in (doc 12) |
| Rewrite | translating an item into the local genre's skin (doc 11) |
| Scriptorium | a Folio room for Rewriting and identification services (doc 12) |
| Emendation | the consumable that raises enhancement |
| Reliquary | an in-run locked loot box opened with in-run keys (doc 11) |
| Fortune Engine | an in-run slot-machine shrine that takes Lucre (doc 11) |
| Inkwell | the meta gacha (doc 15) |
| Sealed Folio | a cosmetic paid loot box (doc 15) |
| Codex | the battle pass (doc 15) |
| Echo | a pre-built Erratum acquired through the Inkwell or a Recollection trial |
| Recollection | an Echo's unlock trial (doc 14) |
| Lattice | the weighted graph of genres that drives Folio order (doc 12) |
| Component | one functional part of an item or ability that carries its own dependencies |

---

## 21. Acceptance tests

1. **xoshiro256++ vectors.** `Rng::from_seed64(0)` yields `0x53175d61490b23df`, `0x61da6f3dc380d507`, `0x5c0fdf91ec9a7bfc`, `0x02eebf8c3bbe5e1a` as its first four `next_u64` outputs.
2. **Second seed.** `Rng::from_seed64(0x0123456789ABCDEF)` yields `0xb2f2a310e96bd1c5`, `0xb54062465b950493`, `0x87aca4a9668814b0`, `0xf13d2e2448a9cffb`.
3. **Lemire range.** `Rng::from_seed64(42)` with five calls of `range(10000)` yields `8143, 3188, 9838, 7011, 7935`.
4. **FNV-1a.** `fnv1a64("rng.loot") = 0xdfec5b83d454f05e`. `fnv1a64("rng.worldgen") = 0x7f6e87856ef84a9a`. `fnv1a32("genre.cyberpunk") = 0x99b54427`. `fnv1a64("PALIMPSEST/DAILY/2026-10-11") = 0xc6f2e12ddca712ab`.
5. **Stream seed.** For run seed `0x0123456789ABCDEF`, `stream_seed(run, "rng.loot", 3) = 0x046973c82020498e`, and that stream's first three outputs are `0x168178f67218a16a`, `0xc4972121be652360`, `0x53480f71d2b62988`.
6. **Seed display.** Run seed `0x0123456789ABCDEF` displays as `028T-5CY4-TQKFF`.
7. **Rate accumulator.** Applying 23 per second for 60 ticks adds exactly 23. Applying 7 per second for 600 ticks adds exactly 70.
8. **Derived stats.** A character with every sub-attribute at 40, level 1, in a VERVE 5 Folio, has Max HP 85, Max stamina 90, Max aether 61, Max composure 90, carry 45 kg, crit chance 500 Bp, Grimoire slots 4, complexity budget 8.
9. **Contradiction example.** The §5.6 worked example produces C = 3, potency 7000, failure 500 and `edge_bp` 2400.
10. **Disconnection.** A Fairy Tale wand (`dep = {arcana}`, `Req = 8`) in Hardboiled Noir held by a Fairy Tale-origin, ATN 40 character gives `raw_gap = 8`, `Tol = 2`, `C = 6`: Disconnected.
11. **Armor example.** AR 100, PEN 0, 20,000 mHP slash gives `DR_bp = 5555` and 8,890 mHP.
12. **Chip floor.** A pure-slash hit with `D2 = 10,000` mHP against an NPC with slash resistance 9000 and AR 1000 on the hit part: resistance leaves 1,000; armor DR is capped at 8500, leaving 150; the chip floor raises the total to 500 mHP.
13. **Rarity weights.** At `d = 25` and `SER = 40`, §13.4 yields `[7000, 4312, 1752, 582, 217, 29, 0]`. At `d = 1`, `SER = 100`, it yields `[7000, 2860, 960, 285, 99, 12, 0]`.
14. **Power tables.** `P(25) = 79110`, `T(25) = 36145`, `P(41) = floor_div(189581 × 10600, 10000) = 200955`.
15. **Skill curve.** Cumulative milli-XP from 0 to 25 is 220,000, to 75 is 1,410,000.
16. **Level curve.** Cumulative XP to reach level 11 is 27,500.
17. **ID validation.** The loader rejects `Genre.Cyberpunk`, `genre..x`, and any two IDs with equal `fnv1a32`.
18. **HP display.** A character with 400 mHP displays 1 HP.
19. **Stream isolation.** Generating Folio 7 with the same run seed yields byte-identical block data whether Folios 1 to 6 were played or skipped.
20. **No floats.** A lint step fails the build if `f32` or `f64` appears in `palimpsest_sim` outside `#[cfg(test)]` code and the documented conversion boundary module.

---

## 22. Invariants annex

```
Σ canon: ids ⊗ units ⊗ formulas ⊗ rng ⊗ ownership
W! sim.state ∈ {Fx, Milli, Bp, Bam, i32, i64, u64}        N! f32|f64 ∈ sim.state
W! ∀ draw: stream == owner(system)                         N! HashMap-iteration → draw
W! ∀ formula: order(bp-chain) == listed order              N! reorder for speed
W! ∀ content: id ~ regex ∧ fnv1a32 unique                  N! rename shipped id without alias
W! mechanics := archetype                                  W! dependency := genre skin
W! Contradiction ⊥ Palimpsestic                            W! ur_text ⇒ C = 0 ∧ edge = 0
W! Fortune|Serendipity → in-run loot only                  N! Fortune|Serendipity → paid gacha odds
W! Lucre ∩ account currencies = ∅                          N! real money → in-run Reliquary|Fortune Engine
W! derived stats := §6.4 only                              N! competing formula ∈ other docs
W! rendering reads sim                                     N! rendering → sim feedback
W! LLM text → cosmetic only                                N! LLM output → sim state
```
