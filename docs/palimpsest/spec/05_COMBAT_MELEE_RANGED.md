# 05 — Combat: Melee, Ranged, Armor and Damage
As of 2026-10-11. Status: BINDING. Conforms to canon-1.5.

This chapter owns every weapon, armor, shield, ward generator, ammunition and mount archetype, the melee moveset, firing, ballistics, explosions, poise and stagger, blocking and parrying, threat, the doc-05 details of the canonical damage pipeline (Canon §10.5), champion counters, weapon Contradiction failures and the combat-feel parameters. Every number here is a starting value tuned against the Folio-1 anchors of Canon §17.4. Every formula is integer, every random draw is keyed, and nothing here reads a float.

## Imports

| Source | Sections consumed |
|---|---|
| Canon §1 | numeric types and primitives (`Fx`, `Milli`, `Bp`, `Bam`, `Ticks`, `floor_div`, `ceil_div`, `round_half_even`, `mul_bp`, `mul_fx`, `div_fx`, `dist2`, `sqrt_fx`, `fx_from_milli`, `fx_floor_block`); checked shifts and the coordinate range (canon-1.5); display-unit rule; integer content encoding; coordinates; `sin_bam`, `cos_bam`, `fwd`, `ang_diff`, `atan2_bam`; ascending-EntityId iteration |
| Canon §2–§3 | `TICK_HZ`, `DT`, semi-implicit Euler, tick schedule, rate accumulator, `in_combat`, replay `view_tick`; keyed `rng.combat` and `rng.status` (owner 05) and the §3.4 draw counts |
| Canon §4–§6 | gravity, terminal velocity, `PROJECTILE_CAP`, fixed terrain and the `destructible` tag (§4.3), humanoid capsule, movement, dodge, lean; dependencies, tolerance, Contradiction, Anachronism, Shock of the New, ambient TECHNE and MYTHOS, local genre and seam zones, `vrv(e)`, manufacturer signature traits (§5.9), World Laws and `law_bound` (§5.10), `st.unmoored` (§5.11); shortfall; §6.4 rows 4–6, 11–20, 39, 41; §6.5 |
| Canon §7, §9 | skills and combat XP; body plans, `share`, `hit_mult`, precision defaults, baked hitbox poses, 32-tick pose history, severing and hand bindings |
| Canon §10–§12 | damage types, armor efficiency, `ward_mult`, resistances, armor and `pen_bp`, source magnitude, the pipeline, entry points, blocking, lethal resolution; statuses (doc 05 owns bleeding, burning, corroded, exposed, staggered, stunned, frozen, blinded, chilled, shocked, knocked_down, slowed, disarmed, weakened, taunted, grappled, mounted), buildup, DoT, direct application, multiplier order, player-controlled bearers; `res.stamina`, `res.poise`, `res.ward`; surfaces |
| Canon §13–§19 | categories, `slot.ward`, `slot.mount`, rarity power, quality, enhancement, `P`, `T`, `H`, `S`, `src_lvl`; factions and minions; party tables, ranks, champion counters and elite aspects, reference anchors, friendly fire, ghosts, mounts, invasions (§17.9: the invader is replay slot 4 and is lag-compensated like an Erratum); events; namespace ownership |
| Docs 02, 03, 04, 06, 07, 08, 09, 11, 12, 13, 14, 16, 17, 18 | the interfaces listed under "Interface assumptions" |

## 1. Conventions and the combat phase

### 1.1 Notation

- `mHP`, `mst` and `mp` are milli-units of HP, stamina and poise (Canon §1.5). Poise written without a prefix is a display value: impact 12 is stored as 12,000 mp.
- "Base" damage of an archetype is `base_mhp`, the per-hit or per-projectile magnitude at ilvl 1 for a Common, Normal, +0 item of its reference material (`mat.iron` unless stated), before any attack factor (§10.1).
- `t` is the current simulation tick. `W`, `A`, `R` are windup, active and recovery ticks.
- Reference target (REF): a standard-rank NPC at Folio 1 with AR 10 on every part (Canon §17.4), resistance 0, not native, Coherent attacker, torso hits, no crit unless stated. Reference attacker: all sub-attributes 40, level 1, governing skill 20, VERVE 5, meeting every attribute requirement (no Canon §6.3 shortfall). "REF DPS" is expected damage per second over a full combat cycle (including reloads and combo recoveries) with crit expectation folded in as `(9500 × hit + 500 × crit_hit) / 10000`.
- **REF cycles**, exactly as acceptance tests 1 and 2 compute them: light combo `Σ (W + A + R)` over the steps `+ 6` (the finisher's extra recovery); full heavy `charge_full + W + A + R`; magazine weapons `magazine × interval + reload` (the reload starts when the last shot's interval has run out, §6.1); single-load weapons with `rpm` 0 `1 + reload`; shotgun `6 × 50 + 20 + 32 × 6`; charge rifle `45 + 18`; beam `60 × 6 + 120`; thrown weapon 30.

### 1.2 What doc 05 decides

The namespaces under Exports, the archetype-to-skill mapping (Canon §7), the details of Canon §10.5, the behavior of the statuses Canon §11.1 assigns to doc 05, poise, guard arcs and timing (Canon §10.7), threat, and weapon Contradiction failures.

### 1.3 Combat phase inside a tick

Doc 03 §2.2 runs this work inside its system order; the table names the doc 03 system that hosts each step. Every step iterates entities in ascending `EntityId`. C1 runs before `sys.movement`, so an action reads the positions of the previous boundary; C2 to C9 run after movement and collision and before `sys.status_timers`.

| Step | Doc 03 system | Work |
|---|---|---|
| C1 | #7 `sys.actions` | Apply each actor's command: at most one action start per actor per tick (§1.4), weapon state machines (§4.8, §6.1), lean target (§6.8), ADS, reload, swap, guard raise, throw release, mount actions, imbue. Projectile spawns requested here are applied after #7 (doc 03 §2.2). |
| C3 | #13 `sys.melee` | Melee traces for every swing in its active ticks (§2.3), against the post-movement poses of tick `t` (rewound for player attackers, §2.6). |
| C2, C4 | #14 `sys.projectiles` | Doc 03 §5.19 moves every projectile. On its spawn tick a projectile first takes its lag-compensation catch-up steps (C2, §2.6); every projectile then takes its normal step (C4). Doc 05's hooks (§7.1) decide what each contact does. |
| C4 | #15 `sys.hitscan` | Beam pulses due this tick (§7.6). |
| C5 | #16 `sys.explosions` | At most 4 due explosions, in queue order (doc 03 §12); doc 05 supplies their parameters and builds their area hits (§9.4). |
| C6 | #17 `sys.damage` | Sort the tick's hit queue by `(source EntityId, seq)` ascending and resolve each hit completely, one at a time (§10.2): pipeline steps 1–9 including poise and stagger (§12.4, §12.5), buildup (§13), counters (§15.1), severing (§19.1) and lethal resolution (Canon §10.8). A hit whose target died earlier in this C6 is discarded with no event. |
| C7 | #17, after C6 | Interrupts: every entity that gained a disabling status in C6 transitions to `Staggered` (§4.8, §6.1); swings in progress end and produce no further samples. |
| C8 | #17, after C7 | Timers: bloom and recoil recovery (§6.4, §6.5), poise reset (§12.5), ward recharge (§15.4), threat decay (§14.1), parry and riposte windows (§4.5). |
| C9 | #17, after C8 | Nothing is batched: every event is appended to doc 03's event list on the call that creates it (doc 03 §2.4), so events keep creation order. |
| — | #25 `sys.status_pulse`, #26 `sys.damage_late`, #33 `sys.status_timers` | DoT and buildup decay (Canon §11.2, §11.3); hits queued after #17 (environment, contact, DoT, shock chains, thorns) resolved by C6's rules; status expiry. |

Each weapon, guard and mount state machine makes at most one transition per tick, in C1, except the interrupts of C7. A state's counter `k` (or `c`) is 0 on the tick the state is entered and rises by 1 on every later tick the state persists.

### 1.4 Keyed draw allocation

Every draw in this chapter uses `keyed_rng(stream, folio, actor, tick, seq)` (Canon §3.3) with `tick = t`. The `seq` values are allocated by this table and never by a running counter, so a draw never depends on how many other draws happened that tick.

**One action start per actor per tick.** `seq` 0 is shared by every action that can start on a tick: a doc 06 cast, a quickbar item use (docs 08, 11), a throw, a doc 07 gadget deploy, and a main-hand weapon action (shot, beam pulse, charge release, melee swing, imbue, mount summon). At most one of them starts per actor per tick, chosen in that priority order; the losing inputs are ignored that tick and never buffered (doc 06 §3.5 gives casts the same priority). Off-hand actions (dual-wield swings, off-hand pistol shots, the raise of a shield with a dependent skin) use `seq` 1 and may start on the same tick as a `seq` 0 action.

| `seq` | `actor` | Draws, in this order | Count |
|---|---|---|---|
| 0 | the wielder | primary action started this tick: Contradiction failure roll if one is due (§16.2); then for a shot or beam pulse: 1 recoil jitter, 2 spread values per projectile, then 1 value per per-shot chance component (`affix.frugal`, `mfr.post_apocalypse.lastditch`, doc 11) in ascending ID; for a throw: 2 spread values; for a melee swing: 1 value per per-swing chance component in ascending ID; for an imbue or a summon: nothing further | `f + 1 + 2n + s` for a shot with `n` projectiles and `s` per-shot chance components, where `f ∈ {0, 1}`; a shot that fails stops after the failure roll |
| 1 | the wielder | off-hand action started this tick, same rules as `seq` 0 | as `seq` 0 |
| 2 | the rider | mount stall roll every 300 ticks while the mount moves (§17.4) | 1 |
| 3 | the bearer | ward-generator flicker roll (§15.4) | 1 |
| `16 + i` | the hit's `source` | the `i`-th hit with that source this tick (`i` from 0 in detection order): 1 crit roll (none for a sneak attack; drawn and ignored for a `NO_CRIT` hit); then each "X Bp chance" on-hit component in ascending affix or perk ID order, 1 draw each. A component that applies a status draws from `rng.status` with the same key; every other component draws from `rng.combat` | `1 + procs`, or `procs` for a sneak attack |

`source` is the attacker for melee and hitscan hits and the projectile's own `EntityId` for projectile impacts and the explosions they cause. A projectile is an entity, so its keyed draws are independent of its owner's later actions. Doc 07 turrets and drones are actors too: their shots use the gadget's `EntityId` (doc 07 §11.4).

`i` is bounded by the entities one source can reach in one tick, which `ACTIVE_ACTOR_CAP`, `PARTY_MAX`, `FOLIO_MINION_CAP` and one mount per Erratum keep below 300, so every doc 05 `seq` is below `0x0200`. Other chapters reserve `rng.combat` slots outside that range: `0xFFFF` for Death's Door (doc 04), and doc 04 requires every doc 05 slot to stay below `0xFF00`.

### 1.5 The hit record

```rust
pub struct HitRecord {
    pub source: EntityId,          // §1.4
    pub owner: EntityId,           // wielder, thrower or NPC; equals source for melee and hitscan
    pub seq: u32,                  // 16 + i
    pub target: EntityId,
    pub part: PartId,              // hit part; area hits use the plan's largest-share part
    pub kind: HitKind,             // Melee | Projectile | Hitscan | Area | Thorns
    pub item_uid: Option<u64>,     // base-function component for Contradiction and Anachronism
    pub move_id: Id32,             // move.* that produced it
    pub base_mhp: i64,             // after attack factors (§10.1)
    pub shares: [i32; 15],         // Bp per Canon §10.1 row order, sum 10000
    pub pen_bp: i32,
    pub precision_bp: i32,
    pub impact_mp: i64,            // after move impact factor, before S(src_lvl)
    pub status_power: Vec<(Id32, i32)>, // §13, ascending status ID
    pub counters: u8,              // bit per counter.* (§15.1)
    pub flags: u32,                // SEVER, GUARD_BREAK, IGNORE_ARMOR, RIPOSTE, AREA, MOUNTED_CHARGE, PARRYABLE, BLOCKABLE, NO_CRIT
    pub sneak_tier: u8,            // 1..=3 from the archetype (§3.2c, §5.2d); doc 09 reads it
    pub incoming: [Fx; 3],         // vector from the target's origin toward the attack origin; only its bearing is read (§12.2)
    pub dist_mm: i64,              // path length for falloff
    pub src_lvl: i32,              // Canon §13.8.1
}
```

### 1.6 Stance base speeds

Canon §4.2 makes final move speed equal base speed, then §6.4 row 11, then statuses. Combat stances replace the base speed while they last. When several stances apply, the lowest base speed wins; crouch walk (2000 mm/s) takes part in the same minimum.

| Stance | Base speed (mm/s) |
|---|---|
| ADS fully or partly raised | the weapon's `ads_walk_mm_s` (§5.2) |
| Melee light windup, active or recovery | 3000 |
| Heavy charge, or charge-rifle draw | 2400 |
| Heavy windup, active or recovery | 2000 |
| Guard raised (§12.1) | 2600; tower shield 2000 |
| Lean held (§6.8) | 2600 |
| Grappler holding a target (§19.3) | 2000 |
| Firing a two-handed ranged weapon: the 20 ticks after each shot or pulse | 3000 for the heavy_arms archetypes (auto rifle, shotgun, launcher, beam); 3400 for every other two-handed ranged archetype. Doc 04 Heavy Arms 50 removes the heavy_arms row |
| Casting (doc 06 §3.6) | 2150 winding, overcasting or charging; 1720 channeling |
| Sprint, by armor weight score (§11.3) | `6500 - 20 × score_eff` |
| Swim, by armor weight score | `2400 - 30 × score_eff` |
| Downed crawl (doc 16, doc 03 §5.12) | 800 |

A mounted rider has no stance: the mount's speeds govern (§17.1).

### 1.7 Inputs consumed

Doc 17 owns the bindings and doc 02 §5 the bit layout. This chapter reads these logical inputs from the command's `buttons` bitfield; doc 03 #4 derives presses by comparing with the previous tick's buttons.

| Input | Bit (doc 02) | Use in this chapter |
|---|---|---|
| `input.attack` | 0 `ATTACK_PRIMARY` | light attack, trigger, draw, beam |
| `input.alt` | 1 `ATTACK_SECONDARY` | guard, ADS, off-hand attack |
| `input.reload` | 2 | reload, clear a jam |
| `input.jump`, `input.crouch`, `input.sprint`, `input.dodge` | 3, 4, 5, 6 | jump attack, crouch stance and underhand throw, sprint attack and hold breath, dodge cancel |
| `input.interact` | 7 | struggle while grappled (§19.3) |
| `input.heavy` | 8 `MELEE_QUICK` | heavy charge; shove or shield bash while guarding; quick melee with a ranged weapon |
| `input.throw` | 10 | cook and throw the quickbar throwable |
| `input.lean_l`, `input.lean_r` | 21, 22 | lean (§6.8) |
| `input.mount` | 25 `MOUNT_TOGGLE` | summon, mount and dismount (§17.2) |

Every action has a `move.` ID carried in `HitRecord.move_id` and in events: light combo steps are `move.light`, heavies `move.heavy`, a raised guard `move.guard`, aiming `move.ads`, reloading `move.reload` and clearing a jam `move.clear_jam`; the other moves are named where they are defined.

---

## 2. Hit detection

### 2.1 Query kinds

| Query | Used by | Shape | Entity poses | Blocks |
|---|---|---|---|---|
| Melee trace | swings, bashes, shoves, mounted charges, tramples | blade segment swept by sub-samples (§2.3) | rewound for player attackers (party and invader, §2.6), current for NPCs | current |
| Projectile sweep | every non-hitscan projectile | doc 03 §5.19: the step segment against hitbox capsules inflated by the projectile radius (`first_body_event`) and against blocks and liquid cells (`first_world_event`) | current, except catch-up steps (§2.6) | current |
| Hitscan ray | beams (§7.6) | segment from the leaned eye to `max_range` | rewound for player attackers, current for NPCs | current |
| Area query | explosions, shockwaves | doc 03 §12.4: bodies whose center point lies within the radius, five exposure samples (§9.4) | current | current, after the explosion's own block damage |

Every query tests hitbox capsules posed from the committed fixed-point pose tables (Canon §9). A capsule is a segment `(c0, c1)` in world space plus radius `rc`. The attacker never hits its own capsules, its own mount or its own rider.

### 2.2 Geometric primitives

All primitives are exact over `Fx` with `i128` intermediates. `dot(a, b)` returns raw Q64.64 as `i128`, like `dist2`.

```rust
/// Closest-approach parameters (s on P, u on Q), each in Fx 0..=1, between segments P0P1 and Q0Q1.
fn seg_seg_params(p0: V3, p1: V3, q0: V3, q1: V3) -> (Fx, Fx) {
    let d1 = p1 - p0; let d2 = q1 - q0; let r = p0 - q0;
    let a = dot(d1, d1); let e = dot(d2, d2); let f = dot(d2, r);
    if a == 0 && e == 0 { return (FX0, FX0); }
    if a == 0 { return (FX0, clamp01(div128(f, e))); }
    let c = dot(d1, r);
    if e == 0 { return (clamp01(div128(-c, a)), FX0); }
    let b = dot(d1, d2);
    let denom = a * e - b * b;                       // i256 via two-limb helper, never negative
    let mut s = if denom != 0 { clamp01(div256(b * f - c * e, denom)) } else { FX0 };
    let mut u = div128(((b * s.raw as i128) >> 32) + f, e);    // u = (b·s + f) / e
    if u < FX0 { u = FX0; s = clamp01(div128(-c, a)); }
    else if u > FX1 { u = FX1; s = clamp01(div128(b - c, a)); }
    (s, u)
}
/// Hit test of a segment of radius rb against a capsule: true when the closest distance <= rb + rc.
fn seg_capsule(p0, p1, rb, cap) -> Option<Fx /*s*/> {
    let (s, u) = seg_seg_params(p0, p1, cap.c0, cap.c1);
    let pa = lerp(p0, p1, s); let pb = lerp(cap.c0, cap.c1, u);
    let rr = (rb + cap.r).raw as i128;
    if dist2(pa, pb) <= rr * rr { Some(s) } else { None }
}
```

`div128(n, d) = Fx(floor_div(n << 32, d) as i64)` for Q64.64 numerator and positive denominator (negate both first when `d < 0`); `div256` is the same over the two-limb `i256` helper. Doc 02 ships both in the fixed-point module; `clamp01` clamps to `0..=Fx::from_int(1)`. The entry parameter of a swept sphere into a capsule is `s_hit = max(0, s - sqrt_fx(rr² - d²) / |p1 - p0|)`, computed with `sqrt_fx` and `div_fx`; when `|p1 - p0| = 0`, `s_hit = 0`.

**Voxel traversal** is Amanatides–Woo over integer block coordinates. From `p0` toward `p1`, step the axis with the smallest `t_max`; ties step X, then Y, then Z. The walk visits cells in strictly increasing entry parameter and stops at the first cell for which the caller's predicate returns true, or when the entry parameter exceeds 1. Cells outside the Folio read as `block.void_bedrock` (Canon §1.8).

### 2.3 Melee traces

A swing sweeps a **blade segment** through the active ticks of its move.

- **Pivot**: `pivot = eye + (0, -200 mm, 0)`, where `eye` is the attacker's eye position including the lean offset (§6.8), recomputed on every active tick from the attacker's position on that tick. Mounted riders use their saddle eye (§17.3). Yaw arithmetic in the shape table wraps (Canon §1.9); pitch results are clamped to `-16000..=16000`.
- **Blade**: at angles `(yaw_s, pitch_s)`, `hilt = pivot + fwd(yaw_s, pitch_s) × 300 mm` and `tip = pivot + fwd(yaw_s, pitch_s) × tip_dist`. Blade radius `blade_r` is 40 mm, or 60 mm for `move.shape_thrust`.
- **Sub-samples**: `K = 4` per active tick when `reach_mm <= 2000`, else `K = 8`. A swing with `A` active ticks has `N = A × K` samples; sample `j ∈ 0..N` belongs to active tick `floor_div(j, K)`. Its progress is `(j + 1) / N`.
- **Shapes** (aim yaw `Y` and pitch `Pt` are read once at the first active tick and frozen for the swing; `arc` is the archetype's half-angle):

| Shape | `yaw_s(j)` | `pitch_s(j)` | `tip_dist(j)` |
|---|---|---|---|
| `move.shape_rl` | `Y - arc + floor_div(2 × arc × (j + 1), N)` | `Pt` | `reach` |
| `move.shape_lr` | `Y + arc - floor_div(2 × arc × (j + 1), N)` | `Pt` | `reach` |
| `move.shape_overhead` | `Y` | `Pt + av - floor_div(2 × av × (j + 1), N)`, `av = max(arc, 5461)`, clamped to ±16000 | `reach` |
| `move.shape_thrust` | `Y` | `Pt` | `r0 + floor_div((reach - r0) × (j + 1), N)`, `r0 = mul_bp(reach, 4000)` |

- **Blocks and clang.** At each sample, traverse voxels from `hilt` to `tip` with the predicate `solid`. If a solid cell is entered at distance `s_blk` from the pivot, the effective blade ends there. For every shape except thrust, if `s_blk < mul_bp(reach, 6000)` the swing **clangs**: this sample and all later samples produce no entity hits, the active phase ends, recovery gains 8 ticks, the block takes the swing's block damage once (§7.3), and `emit_noise(point, 70)` fires. A thrust stops at the block without clanging and damages it once.
- **Entity hits.** At each sample, every capsule within `blade_r + rc` of the effective blade is a candidate. An entity already hit by this swing is skipped. New entities are ordered by sample `j`, then distance from the hilt to the closest-approach point, then `EntityId`. For each new entity the hit part is its intersecting capsule whose closest-approach point is nearest the hilt; ties prefer a precision part, then the lower part index.
- **Cleave.** Each new entity consumes one of the archetype's `cleave` slots. When the slots are spent the swing stops producing hits. The `k`-th entity hit (1-based) takes cleave factor 10000, 7500, 5000 for `k` = 1, 2, 3.
- **Allies.** Allied Errata and minions are valid candidates and consume cleave slots; their hits apply friendly fire (Canon §17.5).

### 2.4 Projectile sweeps

Doc 03 §5.19 owns the geometry: each step it tests the step segment against hitbox capsules inflated by the projectile's radius and against blocks (`raycast_blocks` at radius 0, `sphere_substeps` otherwise) and liquid cells. The earliest event by distance along the segment wins; at equal distance the body event wins over the block event; among capsules of one body the part listed first in Canon §9 wins ties. Doc 05 decides what each event does through the four hooks of §7.1. After a body event that the projectile survives (§7.2), the sweep continues from the contact point in the same step and excludes entities in the projectile's `hit_set`.

### 2.5 Hit part and precision

- Melee and projectile hits use the capsule found by the query. Hitscan uses the first capsule along the ray.
- Area hits never count as precision hits and always use the target plan's largest-share part (`bp.torso`, `bp.thorax`, `bp.body_front`, `bp.mass` or `bp.chassis`).
- A precision part (Canon §9) applies the weapon's `precision_bp` at step 2.2. Weapons inherit the Canon §9 defaults (ranged 15000, melee 12500) unless their archetype row overrides them.

### 2.6 Lag compensation hook (doc 16)

- `LAGCOMP_MAX_TICKS = 16` (doc 16's `LAGCOMP_MAX_REWIND` equals it). The host first clamps the command's `view_tick` per doc 16 §5.5 (at most the measured lag plus 4 ticks, never more than 16 ticks back). For a command applied at tick `t`, `rewind = min(t - view_tick, LAGCOMP_MAX_TICKS)` and `t_r = t - rewind`.
- **Who is rewound.** Every player-controlled attacker: party slots 0 to 3 and the invader in slot 4 (Canon §17.9). NPC attacks, minions, turrets and mounts never rewind.
- A player's melee samples and hitscan rays test every other entity's capsules posed at `t_r`, reconstructed from doc 03 §5.16's 32-record pose history. Blocks are never rewound. A target whose `spawn_tick` is later than `t_r` cannot be hit by a rewound test (doc 16 G6).
- A player's projectile spawned at tick `t` has `catchup = rewind` (doc 03 §5.19). On its spawn tick it takes `catchup` catch-up steps, step `i` (0-based) against poses at `t_r + i + 1`, so the last one tests the poses of `t`; it then takes its normal step against the poses of `t`, and one step per tick afterwards. Every other projectile has `catchup = 0` and takes exactly its normal step on its spawn tick. Example: fired on tick 1000 with `view_tick` 994, the projectile steps against poses 995, 996, 997, 998, 999, 1000 and then 1000 again: 7 steps on its spawn tick.
- Target **state** always reads tick `t`: guard raised, parry window, dodge invulnerability, statuses, ward, HP. Position comes from the shooter's view; defense comes from the host's present.
- Replays reproduce every lag-compensated hit from the logged `view_tick` (Canon §2.4).

### 2.7 Same-tick ordering

All hits detected in C3 to C5 enter one queue resolved in C6 by `(source EntityId, seq)`. A source that dies during C6 still resolves its remaining hits. Two hits on the same target from different sources resolve in queue order, so lethal resolution (Canon §10.8) sees them in that order.

---

## 3. Melee archetypes

### 3.1 Template schema

Every `arch.melee.*` record has these fields. Item templates (doc 11) and manufacturers may override any field listed as overridable in doc 11; genre skins never change a field here, only names, meshes and the dependency set (Canon §5.3).

```json
{
  "id": "arch.melee.blade_long",
  "category": "cat.weapon_melee",
  "hands": 1,
  "skill": "skill.blade",
  "ref_mass_g": 1400,
  "base_mhp": 14700,
  "shares": { "dmg.slash": 8000, "dmg.pierce": 2000 },
  "pen_bp": 0,
  "precision_bp": 12500,
  "reach_mm": 1300,
  "arc_bam": 8192,
  "cleave": 2,
  "impact": 12,
  "swap_ticks": 18,
  "block_bp": 2000,
  "light": {
    "windup": 10, "active": 5, "recovery": 12, "stamina_mst": 3000,
    "steps": [
      { "shape": "move.shape_rl", "move_bp": 10000 },
      { "shape": "move.shape_lr", "move_bp": 10500 },
      { "shape": "move.shape_overhead", "move_bp": 13000 }
    ]
  },
  "heavy": {
    "charge_full": 40, "min_bp": 15000, "full_bp": 24000,
    "windup": 6, "active": 5, "recovery": 16, "stamina_mst": 10000,
    "shape": "move.shape_overhead",
    "guard_break": false, "sever": false, "counter_at_full": 0, "grapple_at_full": false
  },
  "block": { "stability_bp": -1500, "arc_bam": 7282, "raise": 6, "parry": 5 },
  "scaling": { "sub.precision": 5000, "sub.power": 4000, "sub.agility": 3000 },
  "req": {},
  "status_bp": {},
  "sneak_tier": 2
}
```

`counter_at_full` is a bitmask over §15.1. Schema validation: `Σ scaling ≤ 15000`, each coefficient `0..=10000` (Canon §6.4 row 12); `Σ shares = 10000`; every tick field `>= 1` except `swap_ticks` of `arch.melee.unarmed` (0); `charge_full >= 20`; `cleave ∈ 1..=3`.

### 3.2 Archetype tables

**Table 3.2a — damage and identity.** `base_mhp` is per light hit at combo step `move_bp` 10000. REF DPS is the light-combo cycle against the REF target with skill 20 (`add` = 600 Bp).

| Archetype | Hands | Skill | `base_mhp` | Shares (Bp) | Precision | Cleave | Impact | REF DPS light | Requirement |
|---|---|---|---|---|---|---|---|---|---|
| `arch.melee.dagger` | 1 | blade | 10100 | pierce 6000, slash 4000 | 15000 | 1 | 6 | 27.0 | — |
| `arch.melee.blade_short` | 1 | blade | 13000 | slash 6000, pierce 4000 | 13000 | 1 | 9 | 30.0 | — |
| `arch.melee.blade_long` | 1 | blade | 14700 | slash 8000, pierce 2000 | 12500 | 2 | 12 | 30.1 | — |
| `arch.melee.blade_great` | 2 | blade | 20000 | slash 8000, blunt 2000 | 12500 | 3 | 25 | 29.0 | MGT 45 |
| `arch.melee.axe` | 1 | hafted | 16400 | slash 8500, blunt 1500 | 12500 | 2 | 14 | 30.1 | — |
| `arch.melee.axe_great` | 2 | hafted | 21600 | slash 8000, blunt 2000 | 12500 | 3 | 28 | 29.1 | MGT 50 |
| `arch.melee.mace` | 1 | hafted | 15600 | blunt 9000, pierce 1000 | 12500 | 1 | 18 | 30.2 | — |
| `arch.melee.warhammer` | 2 | hafted | 22500 | blunt 8500, pierce 1500 | 12500 | 2 | 34 | 29.1 | MGT 55 |
| `arch.melee.spear` | 1 | polearm | 15000 | pierce 9000, slash 1000 | 14000 | 1 | 12 | 30.1 | — |
| `arch.melee.halberd` | 2 | polearm | 22700 | slash 5000, pierce 3000, blunt 2000 | 12500 | 3 | 24 | 29.1 | MGT 45 |
| `arch.melee.quarterstaff` | 2 | polearm | 12100 | blunt 10000 | 12500 | 2 | 12 | 29.0 | — |
| `arch.melee.fist` | 1 | unarmed | 8200 | blunt 7000, pierce 3000 | 13000 | 1 | 7 | 27.1 | — |
| `arch.melee.whip` | 1 | polearm | 16700 | slash 10000 | 15000 | 2 | 8 | 29.1 | FIN 45 |
| `arch.melee.chain` | 1 | polearm | 17800 | blunt 7000, pierce 3000 | 12500 | 2 | 18 | 29.1 | MGT 40 |
| `arch.melee.unarmed` | body | unarmed | 6600 | blunt 10000 | 12500 | 1 | 5 | 22.2 | — |

All melee archetypes have `pen_bp` 0; penetration on melee comes only from affixes and perks (doc 11). `arch.melee.unarmed` is the body function used with empty hands; `kin.beastkin` replaces its shares with blunt 5000, slash 5000 (Canon §16.3). Its `src_lvl` is `d_current` (innate) and its `material_bp`, rarity and quality factors are 10000.

**Table 3.2b — timing and stamina** (ticks at reference mass; stamina in mst).

| Archetype | Ref mass (g) | Light W/A/R | Light steps: shape `move_bp` | Light stamina (mst) | Heavy `charge_full` | Heavy `min_bp`→`full_bp` | Heavy W/A/R | Heavy stamina (mst) | Heavy shape and flags |
|---|---|---|---|---|---|---|---|---|---|
| dagger | 400 | 6/4/8 | thrust 10000, rl 10000, thrust 10000, thrust 13000 | 2000 | 30 | 15000→22000 | 4/4/12 | 7000 | thrust |
| blade_short | 900 | 8/4/10 | rl 10000, lr 10000, thrust 12500 | 2800 | 36 | 15000→23000 | 5/4/14 | 9000 | thrust |
| blade_long | 1400 | 10/5/12 | rl 10000, lr 10500, overhead 13000 | 3000 | 40 | 15000→24000 | 6/5/16 | 10000 | overhead |
| blade_great | 3000 | 16/6/20 | rl 10000, overhead 13000 | 5000 | 50 | 16000→26000 | 8/6/24 | 16000 | overhead; guard_break; sever |
| axe | 1500 | 12/5/14 | rl 10000, lr 10000, overhead 13000 | 3800 | 42 | 15000→24000 | 6/5/18 | 12000 | overhead |
| axe_great | 3200 | 18/6/22 | rl 10000, overhead 13000 | 5000 | 54 | 16000→27000 | 9/6/26 | 18000 | overhead; guard_break; sever; `counter.staggerer` at full |
| mace | 1800 | 12/5/14 | rl 10000, lr 10000, overhead 13000 | 3800 | 42 | 15000→24000 | 6/5/18 | 12000 | overhead; guard_break |
| warhammer | 4000 | 20/6/24 | rl 10000, overhead 13000 | 6000 | 60 | 16000→28000 | 10/6/28 | 20000 | overhead; guard_break; `counter.staggerer` at full |
| spear | 1800 | 10/4/14 | thrust 10000, thrust 10000, rl 12500 | 3000 | 40 | 15000→24000 | 6/4/16 | 10000 | thrust; cleave 2 |
| halberd | 3200 | 18/6/22 | rl 10000, overhead 13000 | 5000 | 52 | 16000→26000 | 9/6/24 | 16000 | overhead; guard_break; sever |
| quarterstaff | 1800 | 9/5/11 | rl 10000, lr 10000, thrust 10000, overhead 12500 | 3000 | 36 | 15000→22000 | 5/5/14 | 9000 | overhead; guard_break |
| fist | 600 | 5/3/7 | thrust 10000 ×3, thrust 13000 | 1900 | 28 | 15000→22000 | 4/3/12 | 6000 | thrust; grapple at full (§19.3) |
| whip | 700 | 14/4/16 | rl 10000, lr 10000, overhead 12500 | 4000 | 40 | 15000→22000 | 6/4/18 | 12000 | overhead; `counter.disruptor` at full; pull 1500 mm at full |
| chain | 2200 | 14/6/16 | rl 10000, lr 10000, overhead 12500 | 4000 | 44 | 15000→25000 | 7/6/20 | 14000 | overhead; guard_break; grapple at full (entangle) |
| unarmed | 0 | 5/3/8 | thrust 10000 ×3, thrust 12000 | 2000 | 28 | 15000→20000 | 4/3/12 | 5000 | thrust; grapple at full |

Light stamina is set so a full light-combo cycle (finisher recovery included) spends at most 7 stamina per second, so a 90-stamina pool supports more than 12 s of light attacks. REF heavy-at-full DPS (charge, windup, active and recovery included) is: dagger 24.7, short blade 29.2, long blade 31.0, great blade 36.1, axe 33.3, great axe 37.8, mace 32.3, warhammer 37.9, spear 32.7, halberd 38.9, quarterstaff 27.7, fist 21.8, whip 33.1, chain 35.5, unarmed 16.8. Two-handed weapons out-damage one-handed weapons only through committed heavies.

**Table 3.2c — geometry, defense, scaling and stealth.**

| Archetype | Reach (mm) | Arc half-angle (Bam) | Scaling coefficients | Block stability / arc / raise / parry | Sneak tier | Status multipliers |
|---|---|---|---|---|---|---|
| dagger | 900 | 5461 | PRC 5000, AGI 4000, SLT 3000 | −3000 / 7282 / 6 / 7 | 1 | bleeding 15000 |
| blade_short | 1100 | 7282 | PRC 5000, AGI 4000, PWR 3000 | −1500 / 7282 / 6 / 7 | 1 | bleeding 12500 |
| blade_long | 1300 | 8192 | PRC 5000, PWR 4000, AGI 3000 | −1500 / 7282 / 6 / 5 | 2 | — |
| blade_great | 1700 | 10923 | PWR 6000, BRN 4000, PRC 2000 | 0 / 7282 / 6 / 4 | 3 | — |
| axe | 1150 | 7282 | PWR 6000, BRN 3000, GRP 3000 | −1500 / 7282 / 6 / 5 | 2 | bleeding 12500 |
| axe_great | 1600 | 9102 | PWR 7000, BRN 5000 | 0 / 7282 / 6 / 4 | 3 | bleeding 12500 |
| mace | 1100 | 6371 | PWR 6000, BRN 4000, GRP 2000 | −1500 / 7282 / 6 / 5 | 2 | stunned 12500 |
| warhammer | 1500 | 7282 | PWR 7000, BRN 5000 | 0 / 7282 / 6 / 4 | 3 | stunned 15000 |
| spear | 2200 | 5461 | GRP 5000, PRC 4000, AGI 3000 | −1500 / 7282 / 6 / 5 | 2 | exposed 15000 |
| halberd | 2300 | 9102 | GRP 5000, PWR 5000, BRN 2000 | 0 / 7282 / 6 / 4 | 3 | — |
| quarterstaff | 1800 | 8192 | AGI 5000, GRP 4000, BRN 3000 | +1000 / 7282 / 6 / 5 | 3 | stunned 12500 |
| fist | 750 | 4551 | AGI 5000, PWR 4000, BRN 3000 | −3000 / 7282 / 6 / 5 | 1 | stunned 12500 |
| whip | 3200 | 4551 | SLT 5000, PRC 5000, AGI 2000 | −3000 / 7282 / 6 / 5 | 2 | bleeding 15000 |
| chain | 1700 | 9102 | GRP 6000, PWR 4000, SLT 2000 | −1500 / 7282 / 6 / 5 | 2 | stunned 12500 |
| unarmed | 700 | 4551 | PWR 5000, AGI 4000, BRN 3000 | −3000 / 7282 / 6 / 5 | 1 | — |

Every archetype's scaling coefficients total 12000 Bp, so no archetype gains more from sub-attributes than another. Archetype-specific rules:

- **Whip sweet spot.** A whip hit whose closest-approach point lies within `mul_bp(reach, 5000)` of the pivot takes attack factor 6000.
- **Chain wrap.** When a chain-weapon hit is blocked, the blocker's block stability (Canon §6.4 row 19) is multiplied by 5000 for that hit.
- **Spear heavy** has cleave 2 and passes through the first target.
- **Fist pair.** Fist weapons in both hands alternate hands visually. They share one combo and gain no damage from the second weapon; the off-hand fist weapon adds its block stability and nothing else.

**Table 3.2d — handling and block damage.** Swap ticks scale by §3.3 `time_bp`, then Canon §6.4 row 17. `block_bp` is the share of a hit's Canon §10.4 magnitude a swing deals to a destructible block once per swing (doc 03 §13.2). A weapon's tool tier for doc 03 §13.2 is its primary material's tier (doc 07); bare hands are `tooltier.t0`.

| Archetype | Swap ticks | `block_bp` | Archetype | Swap ticks | `block_bp` |
|---|---|---|---|---|---|
| dagger | 12 | 2000 | warhammer | 30 | 10000 |
| blade_short | 18 | 2000 | spear | 18 | 2000 |
| blade_long | 18 | 2000 | halberd | 30 | 5000 |
| blade_great | 30 | 2000 | quarterstaff | 30 | 10000 |
| axe | 18 | 5000 | fist | 12 | 1000 |
| axe_great | 30 | 5000 | whip | 18 | 500 |
| mace | 18 | 10000 | chain | 18 | 10000 |
| unarmed | 0 | 500 | | | |

### 3.3 Material-driven timing from `mass_g`

Canon §10.4 makes heavy materials hit harder through `material_bp`. Doc 05 makes them swing slower:

```
mass_ratio_bp = clamp(floor_div(mass_g × 10000, ref_mass_g), 5000, 20000)
time_mass_bp  = 10000 + floor_div((mass_ratio_bp - 10000) × 3000, 10000)
time_bp       = clamp(max(time_mass_bp, ceil_div(material_bp × 10000, 14000)), 8500, 13000)
stamina_bp    = clamp(10000 + floor_div((mass_ratio_bp - 10000) × 5000, 10000), 7500, 15000)
impact_bp     = clamp(10000 + floor_div((mass_ratio_bp - 10000) × 5000, 10000), 7500, 15000)
W', R', charge_full', swap_ticks' = max(1, round_half_even(x × time_bp, 10000))   // active ticks never scale
charge_full' = max(20, charge_full')
stamina' = mul_bp(stamina, stamina_bp);  impact' = mul_bp(impact, impact_bp)
```

`mass_g` is the instance's mass from doc 07 (volume × density) and `material_bp` its primary material's `dmg_bp` (Canon §10.4). Ranged weapons apply `time_bp` to `swap_ticks` and `ads_ticks`, and multiply recoil by `clamp(10000 - floor_div((mass_ratio_bp - 10000) × 2000, 10000), 8000, 12000)`.

**Material speed floor.** `material_bp` already multiplies damage at Canon §10.4. Without the `ceil_div` term a light material with a high `dmg_bp` would multiply damage per second by up to `14000 × 10000 / 8500 = 16470` Bp, above the 14000 material slot of `ITEM_MULT_MAX` (Canon §13.8). The term keeps `material_bp × 10000 / time_bp ≤ 14000` before tick rounding, so a material's whole contribution to damage per second stays inside its canon slot. Heavy materials are unaffected, since their `time_mass_bp` already exceeds 10000.

**Example.** A long blade (`ref_mass_g` 1400) made of a light alloy with `dmg_bp` 9000 weighs 700 g: `mass_ratio_bp = 5000`, `time_mass_bp = 8500`, the floor term is `ceil_div(90,000,000, 14000) = 6429`, so `time_bp = 8500`: W = `round_half_even(85000, 10000)` = 8, R = 10, `charge_full` = 34, stamina = `mul_bp(3000, 7500)` = 2,250 mst and impact 9 (stored 9,000 mp). A 700 g long blade of a material with `dmg_bp` 13000 has the floor term `ceil_div(130,000,000, 14000) = 9286`, so `time_bp = 9286`: W 9, R 11, `charge_full` 37. The same blade in a 3,430 g dense metal gives `mass_ratio_bp = 20000` (clamped from 24500), `time_bp = 13000`, W = 13, R = 16, `charge_full` = 52.

---

## 4. Melee moveset

### 4.1 Light combo

- `input.attack` pressed in `Idle` starts the current combo step if `stamina > 0`. The step's stamina is deducted on its first windup tick and clamps at 0.
- A press during the active or recovery phase of step `k` is buffered. Step `k + 1` starts on the tick after step `k`'s recovery ends.
- With no buffered press, the combo index resets to 1 when 15 ticks pass after recovery ends. The last step (finisher) gains 6 recovery ticks and always resets the index.
- **Cancels.** `input.dodge` during recovery cancels the rest of the recovery and starts the dodge (Canon §4.2). `input.alt` from recovery tick `floor_div(R, 2)` onward cancels into a guard raise.
- Each hit's attack factor is the step's `move_bp`; its impact factor is the same value.

### 4.2 Heavy charge

- `input.heavy` pressed in `Idle` or during a buffered-combo gap enters `Charging` if `stamina > 0` and the wielder lacks `st.exhausted`. Heavy stamina is deducted on the press tick.
- `c` counts ticks held, with `c = 0` on the press tick. On release at `c`:

```
frac_bp  = clamp(floor_div((c - 10) × 10000, charge_full - 10), 0, 10000)
heavy_bp = min_bp + floor_div((full_bp - min_bp) × frac_bp, 10000)
```

- Holding to `c = charge_full + 60` auto-releases. A heavy is **full** when `frac_bp = 10000`; flags marked "at full" in Table 3.2b apply only then.
- **Hyperarmor.** Two-handed heavies multiply poise damage taken by 5000 from release through the last active tick.
- `charge_full` here is the value after §3.3 mass scaling and §6.2 attack speed, never below 20, so the divisor `charge_full - 10` is at least 10.
- Example: a long blade released at `c = 25` gives `frac_bp = floor_div(150000, 30) = 5000` and `heavy_bp = 15000 + 4500 = 19500`, so the hit's base is `mul_bp(14700, 19500) = 28,665` mHP.

### 4.3 Sprint attack (`move.sprint_attack`)

`input.attack` after at least 20 consecutive sprint ticks performs the first combo step's shape with `move_bp` 13000, impact factor 15000, double light stamina, and W+2 / A+2 / R+6. From windup start through the last active tick the attacker gains 2500 mm/s of forward velocity (a lunge). Sprint ends. The next light press continues at combo step 2.

### 4.4 Jump attack (`move.jump_attack`)

`input.attack` or `input.heavy` while airborne, with vertical velocity ≤ 0 and at least 800 mm of air below the origin (a downward voxel probe), performs `move.shape_overhead` with a 4-tick windup and an active phase that lasts until landing, at most 40 ticks. Stamina is double the light cost.

```
fall_mm  = peak_y_mm - current_y_mm at the hit tick       // peak as in Canon §4.2
move_bp  = clamp(14000 + 1000 × floor_div(fall_mm - 1000, 1000), 14000, 20000)
impact factor 20000; landing adds recovery R + 10
```

Maces and warhammers also emit a **landing shockwave**: an area query of radius 1500 mm around the landing point with `base_mhp × 5000` blunt, impact factor 15000, never precision, excluding any entity the jump attack itself hit. The warhammer's jump attack carries `counter.staggerer`.

### 4.5 Parry and riposte

Guard mechanics are in §12. A **parry** happens when a hit flagged `PARRYABLE` (every melee hit unless the NPC template marks it unparryable) arrives inside the guard's arc during the first `parry` ticks after the guard becomes effective (§12.2):

- The hit deals no damage, no buildup and no poise damage. The parrier spends no stamina.
- The attacker gains `st.staggered` by direct application: 30 ticks (unscaled, Canon §11.1), times the attacker's rank control multiplier (Canon §17.3).
- A **riposte window** opens for 30 ticks against that attacker only.

**Riposte** (`move.riposte`). The parrier's first `input.attack` or `input.heavy` inside the window performs the current weapon's first light shape with W 4, the light A and R, `move_bp` 25000, impact factor 0 and the `RIPOSTE` flag. It can hit only the parried attacker; other entities in the arc are ignored. It rolls a crit normally. Long blade riposte base: `mul_bp(14700, 25000) = 36,750` mHP.

### 4.6 Guard break

- **Stamina guard break** (Canon §10.7): a blocked hit that drives the blocker's stamina to 0 applies `st.staggered`.
- **Guard-break heavies.** When a hit flagged `GUARD_BREAK` is blocked, the blocker's stamina loss from Canon §10.7 is multiplied by 20000.
- **Shove** (`move.shove`, any melee weapon, `input.heavy` while the guard is raised): W 4, A 3, R 14, 12 stamina, no damage, impact 25, reach 1000 mm, arc 4551, `move.shape_thrust`. Against a target whose guard is raised it removes a flat 35 stamina (35,000 mst); if that empties the target's stamina, the target is guard-broken. Shove is unparryable.

### 4.7 Quick melee and dual wielding

- **Quick melee** (`move.quick_melee`): `input.heavy` with a ranged weapon active. Base 9000 mHP, blunt 10000, reach 1100, arc 4551, `move.shape_thrust`, W 6 / A 3 / R 18, 6 stamina, impact 15, skill `skill.unarmed`, scaling as `arch.melee.unarmed`. It uses the held weapon's `ilvl`, rarity, quality, enhancement and material for Canon §10.4.
- **Dual wielding.** A one-handed melee weapon in the main hand and a dagger, short blade, axe, mace or fist weapon in `slot.offhand` turns `input.alt` into off-hand light attacks with their own combo index at `move_bp` 8500. The off-hand cannot block, charge a heavy or parry. A pistol, heavy pistol or thrown weapon in `slot.offhand` fires with `input.alt` from the hip only, with spread × 12500.

### 4.8 Melee state machine

States: `Holstered`, `Drawing(k)`, `Idle`, `Windup(m, k)`, `Active(m, k)`, `Recovery(m, k)`, `Charging(c)`, `GuardRaising(k)`, `Guarding(k)`, `Lowering(k)`, `Staggered`, `Disarmed`. `m` is the move and `k` the tick within the state. "Disabling status" means `st.staggered`, `st.stunned`, `st.frozen`, `st.knocked_down`, `st.sleeping` or `st.downed`, or `st.grappled` while a two-handed weapon is active.

| From | Event (first match wins) | To |
|---|---|---|
| any except `Holstered` | a disabling status applied | `Staggered`; a swing ends with no further samples; a charge is lost (stamina not refunded); a guard drops |
| any except `Holstered`, `Staggered` | `st.disarmed` applied | `Disarmed` |
| any | the weapon is swapped out (`input.weapon_*`, doc 17) | `Holstered`; the newly selected weapon enters `Drawing(0)` |
| `Holstered` | the weapon is selected | `Drawing(0)` |
| `Drawing(k)` | `k + 1 = action_ticks(swap_ticks)` (Canon §6.4 row 17; Table 3.2d) | `Idle` |
| `Staggered` | no disabling status remains | `Disarmed` if `st.disarmed` is active, else `Idle` |
| `Disarmed` | `st.disarmed` ended | `Idle` |
| `Idle` | `input.attack` or `input.heavy` pressed while a riposte window is open (§4.5) | `Windup(riposte, 0)` |
| `Idle` | `input.alt` held, no off-hand weapon, stamina > 0, no guard lockout (§12.3) | `GuardRaising(0)` |
| `Idle` | `input.heavy` pressed, stamina > 0, no `st.exhausted` | `Charging(0)` |
| `Idle` | `input.attack` pressed, stamina > 0 | `Windup(jump attack, 0)` if §4.4 holds; else `Windup(sprint attack, 0)` if §4.3 holds; else `Windup(light step, 0)` |
| `Charging(c)` | `input.heavy` released, or `c = charge_full + 60` | `Windup(heavy, 0)` |
| `Charging(c)` | `input.dodge` | dodge (Canon §4.2); charge lost; `Idle` |
| `Windup(m, k)` | `k + 1 = W` | Contradiction failure check (§16.2): on a falter `Recovery(m, 0)` with R + 12 and no hits; otherwise `Active(m, 0)` |
| `Active(m, k)` | `k + 1 = A`, or clang | `Recovery(m, 0)` (clang adds 8 ticks) |
| `Recovery(m, k)` | `input.dodge` | dodge; `Idle` |
| `Recovery(m, k)` | `input.alt` and `k >= floor_div(R, 2)` | `GuardRaising(0)` |
| `Recovery(m, k)` | `k + 1 = R` and buffered light press | `Windup(next step, 0)` |
| `Recovery(m, k)` | `k + 1 = R` | `Idle` |
| `GuardRaising(k)` | `input.alt` released | `Idle` |
| `GuardRaising(k)` | `k + 1 = raise` | `Guarding(0)`; the parry window opens |
| `Guarding(k)` | stamina reaches 0 from a blocked hit | `Staggered` (guard broken, §12.3) |
| `Guarding(k)` | `input.attack` or `input.heavy` pressed while a riposte window is open | `Windup(riposte, 0)` |
| `Guarding(k)` | `input.heavy` pressed | `Windup(shield bash, 0)` with a shield, else `Windup(shove, 0)` |
| `Guarding(k)` | `input.attack` pressed, stamina > 0 | `Windup(light step 1, 0)`; the guard drops |
| `Guarding(k)` | `input.alt` released | `Lowering(0)` |
| `Lowering(k)` | `k + 1 = 4` | `Idle`; a new raise cannot start until 20 ticks after the previous raise started |

Every (state, event) pair not listed leaves the state unchanged and advances its counter. During a dodge's 12 ticks the machine stays in `Idle` and accepts no action input. The riposte move uses W 4 (§4.5).

### 4.8a Status and state gating

These conditions block weapon actions in both state machines (§4.8, §6.1). A blocked input is ignored, not buffered.

| Condition | Blocks |
|---|---|
| `st.exhausted` (doc 04) | heavy charge, sprint attack, dodge, sprint |
| player `st.berserk` (Canon §11.8) | guard raise, cast, reload, item use; melee costs 0 stamina |
| player `st.feared` (Canon §11.8) | firing, throwing, casting and melee swings while facing within 8192 Bam of the source |
| `st.grappled` on the bearer | movement, sprint, jump and dodge; two-handed weapons (they enter `Staggered`); one-handed actions stay legal |
| `st.downed` (doc 16) | every weapon action, guard, reload, throw, item use and mount input |
| `st.mounted` | lean and dodge; weapons outside the mount's "Rider weapons" row (§17.1) |
| dodge in progress | every new weapon action |
| swimming | two-handed weapons cannot swing, fire or aim; one-handed ranged weapons fire at the swimming stance (§6.3) |
| climbing or mantling | every weapon action |
| doc 06 cast machine outside `Idle` | new weapon actions (doc 06 §3.5) |
| `Cooking` or throw recovery (§9.2) | main-hand weapon actions |
| summon channel (§17.2) or `move.imbue` (§15.3) | every other weapon action until the channel ends |

### 4.9 Sneak attack hook (doc 09)

Sneak eligibility is evaluated per (attacker, target) on the impact tick exactly as Canon §10.5 step 2.4 states, through doc 09's `sneak_eligible(att, tgt, t)`. Doc 09 supplies the multiplier `sneak_mult(att, tgt, tier)` from the archetype's `sneak_tier` (tier 1 = 30000, 2 = 22500, 3 = 15000 before doc 09's skill and talent terms). A sneak hit draws no crit roll. When doc 09 grants `IGNORE_ARMOR` (Sneak 100), step 5 is skipped. Sneak attacks keep their move factors, so a full heavy sneak attack stacks `heavy_bp` with the sneak multiplier; the Oblivion `max(sneak, power)` rule is not used because heavy factors live in the base (§10.1) and stay below ×2.8.

---

## 5. Ranged archetypes

### 5.1 Template schema

```json
{
  "id": "arch.ranged.rifle_auto",
  "category": "cat.weapon_ranged",
  "hands": 2,
  "skill": "skill.heavy_arms",
  "ref_mass_g": 3600,
  "fire_mode": "move.fire_auto",
  "base_mhp": 5600, "pellets": 1,
  "shares": { "dmg.pierce": 10000 },
  "pen_bp": 1500, "precision_bp": 14000, "impact": 3,
  "rpm": 600, "magazine": 30, "reload_ticks": 120, "ammo": "ammo.heavy", "ammo_per_shot": 1,
  "speed_mm_s": 720000, "gravity_bp": 0, "radius_mm": 10, "max_range_mm": 100000,
  "falloff": { "start_mm": 30000, "end_mm": 60000, "min_bp": 6000 },
  "wallpen": 40, "pierce_count": 1, "ricochet": true,
  "spread": { "hip_bam": 546, "ads_bam": 91, "bloom_bam": 36, "bloom_max_bam": 455, "bloom_delay": 4, "bloom_decay_bam": 18 },
  "recoil": { "v_bam": 73, "h_bam": 36, "kneel": 8, "sig": 26214, "period": 3, "jitter_bam": 18, "delay": 8, "rate_bam": 25 },
  "ads": { "ticks": 14, "walk_mm_s": 3000, "sway_bam": 18 },
  "handling": { "swap_ticks": 30, "sprint_to_fire": 12 },
  "energy": false,
  "harness": true,
  "aim_assist_bp": 10000,
  "scaling": { "sub.grip": 6000, "sub.precision": 4000, "sub.brawn": 2000 },
  "req": {},
  "status_bp": {},
  "sneak_tier": 3,
  "innate_counter": 0
}
```

`sig` is a 16-bit pattern written in decimal (26214 = `0x6666`). Schema validation: `rpm ∈ 0..=1800`, where 0 is allowed only when `magazine = 1` (a single-load weapon with no fire interval, §6.2); `pellets ∈ 1..=16`; `speed_mm_s ∈ 0..=1_200_000` where 0 means hitscan; `Σ scaling ≤ 15000`.

### 5.2 Archetype tables

**Table 5.2a — damage, cadence and ammunition.** `base_mhp` is per projectile (per pulse for the beam). Reload is from empty; a tactical reload with at least one round left takes `ceil_div(reload × 8500, 10000)` ticks. REF DPS includes reloads.

| Archetype | Fire mode | Skill | `base_mhp` | Pellets | RPM (interval ticks) | Magazine | Reload | Ammo | Shares | `pen_bp` | Precision | Impact | REF DPS | Requirement |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| `arch.ranged.pistol` | semi | marksmanship | 8600 | 1 | 360 (10) | 12 | 72 | `ammo.light` | pierce 10000 | 1000 | 15000 | 6 | 30.2 | — |
| `arch.ranged.pistol_heavy` | semi | marksmanship | 20600 | 1 | 150 (24) | 6 | 110 | `ammo.heavy` | pierce 10000 | 2500 | 16000 | 16 | 30.0 | MGT 40 |
| `arch.ranged.smg` | auto | marksmanship | 4500 | 1 | 900 (4) | 32 | 100 | `ammo.light` | pierce 10000 | 0 | 13000 | 2 | 30.3 | — |
| `arch.ranged.rifle_auto` | auto | heavy_arms | 5600 | 1 | 600 (6) | 30 | 120 | `ammo.heavy` | pierce 10000 | 1500 | 14000 | 3 | 30.3 | — |
| `arch.ranged.rifle_marksman` | semi | marksmanship | 11600 | 1 | 240 (15) | 12 | 120 | `ammo.rifle` | pierce 10000 | 3000 | 17500 | 10 | 28.0 | — |
| `arch.ranged.shotgun` | bolt | heavy_arms | 6200 | 8 | 72 (50) | 6 | 20 + 32 per shell | `ammo.shell` | pierce 10000 | 0 | 12500 | 4 per pellet | 30.1 | MGT 40 |
| `arch.ranged.sniper` | bolt | marksmanship | 41200 | 1 | 45 (80) | 5 | 150 | `ammo.rifle` | pierce 10000 | 5000 | 20000 | 30 | 24.0 | FIN 45 |
| `arch.ranged.launcher` | semi | heavy_arms | direct 10000; blast 54000 | 1 | 60 (60) | 1 | 90 | `ammo.ordnance` | direct blunt 10000; blast pierce 2000, blunt 4000, fire 4000 | 0 | 10000 | direct 20; blast 40 | 26.3 | MGT 50 |
| `arch.ranged.beam` | beam | heavy_arms | 3700 per pulse | 1 | 600 pulses (6) | 60 units | 120 | `ammo.cell` | fire 10000 | 0 | 12500 | 1 | 30.2 | — |
| `arch.ranged.charge_rifle` | charge | marksmanship | 30000 at full charge | 1 | 0 (draw 15–45, nock 18) | 1 | 18 | `ammo.draw` | pierce 10000 | 2500 | 17500 | 18 at full | 29.9 | — |
| `arch.ranged.crossbow` | semi | marksmanship | 35700 | 1 | 0 (single load) | 1 | 80 | `ammo.draw` | pierce 10000 | 4000 | 17500 | 22 | 28.0 | — |
| `arch.ranged.thrown` | throw | throwing | 14800 | 1 | 120 (30) | the held stack | 0 | itself | pierce 7000, slash 3000 | 1500 | 15000 | 8 | 27.1 | — |

The sniper trades sustained REF DPS (24.0) for a ×2.0 precision multiplier and armor penetration: at a precision part its REF DPS is 48.4 and one headshot (86,602 mHP) kills a REF enemy. The launcher's REF DPS counts one target at the blast center; its 150-tick cycle is the 60-tick interval plus the 90-tick reload (§6.1). REF DPS against an AR 40 target: pistol 20.4, heavy pistol 25.5, SMG 16.3, auto rifle 18.1, marksman 21.4, shotgun 18.0, sniper 22.5, launcher 23.0, beam 30.2, charge rifle 26.6, crossbow 25.8, thrown 18.0. Many small hits lose most against armor (Canon §10.3).

**Table 5.2b — ballistics.** Speeds in mm/s; `gravity_bp` multiplies local gravity (Canon §4); radius and ranges in mm. Falloff is `start / end / min_bp`.

| Archetype | Speed | Gravity | Radius | Max range | Falloff | Wallpen | Pierce count | Ricochet |
|---|---|---|---|---|---|---|---|---|
| pistol | 360000 | 0 | 10 | 60000 | 15000 / 35000 / 5000 | 20 | 1 | yes |
| pistol_heavy | 420000 | 0 | 12 | 70000 | 20000 / 45000 / 5500 | 40 | 2 | yes |
| smg | 380000 | 0 | 10 | 50000 | 12000 / 30000 / 4500 | 15 | 1 | yes |
| rifle_auto | 720000 | 0 | 10 | 100000 | 30000 / 60000 / 6000 | 40 | 1 | yes |
| rifle_marksman | 850000 | 0 | 10 | 160000 | 50000 / 120000 / 7500 | 80 | 2 | yes |
| shotgun | 400000 | 0 | 15 | 40000 | 8000 / 20000 / 2000 | 5 | 1 | yes |
| sniper | 950000 | 0 | 10 | 256000 | none | 160 | 3 | yes |
| launcher | 50000 | 3000 | 80 | 120000 | none | 0 | 1 | no |
| beam | hitscan | — | 25 | 35000 | 15000 / 35000 / 4000 | 0 | 1 | no |
| charge_rifle | `40000 + floor_div(60000 × (c - 15), 30)` | 8000 | 20 | 150000 | none | 30 | 2 | no |
| crossbow | 85000 | 6000 | 20 | 120000 | none | 30 | 2 | no |
| thrown | `mul_bp(30000, throw_v_bp)` (§9.1) | 10000 | 40 | 50000 | none | 0 | 1 | no |

**Table 5.2c — accuracy, recoil and handling.** Angles in Bam; recoil columns are `v / h / kneel / sig / period / jitter`, recovery is `delay ticks / Bam per tick`.

| Archetype | Hip | ADS | Bloom per shot / max | Bloom delay / decay | Recoil | Recoil recovery | ADS ticks | ADS walk (mm/s) | ADS sway | Swap | Sprint-to-fire |
|---|---|---|---|---|---|---|---|---|---|---|---|
| pistol | 364 | 73 | 64 / 364 | 4 / 16 | 182 / 36 / 0 / 0x5A5A / 1 / 18 | 8 / 30 | 9 | 3400 | 9 | 18 | 6 |
| pistol_heavy | 455 | 55 | 182 / 546 | 6 / 20 | 546 / 91 / 0 / 0x3333 / 1 / 36 | 10 / 40 | 12 | 3200 | 18 | 24 | 8 |
| smg | 455 | 182 | 27 / 364 | 3 / 18 | 55 / 46 / 6 / 0x0F0F / 2 / 27 | 6 / 25 | 10 | 3400 | 9 | 24 | 8 |
| rifle_auto | 546 | 91 | 36 / 455 | 4 / 18 | 73 / 36 / 8 / 0x6666 / 3 / 18 | 8 / 25 | 14 | 3000 | 18 | 30 | 12 |
| rifle_marksman | 728 | 18 | 91 / 546 | 8 / 20 | 273 / 46 / 0 / 0x6969 / 1 / 18 | 16 / 35 | 18 | 2600 | 46 | 33 | 15 |
| shotgun | 1092 | 819 | 0 / 0 | — | 728 / 91 / 0 / 0x5555 / 1 / 46 | 12 / 50 | 12 | 3000 | 9 | 30 | 12 |
| sniper | 1820 | 0 | 0 / 0 | — | 910 / 0 / 0 / 0x0000 / 1 / 46 | 20 / 60 | 24 | 2000 | 91 | 40 | 20 |
| launcher | 364 | 91 | 0 / 0 | — | 546 / 0 / 0 / 0x0000 / 1 / 0 | 20 / 40 | 20 | 2200 | 46 | 45 | 20 |
| beam | 91 | 27 | 9 / 182 | 4 / 9 | 9 / 9 / 0 / 0xAAAA / 4 / 9 | 6 / 20 | 12 | 3000 | 9 | 30 | 12 |
| charge_rifle | 455 | 27 | 0 / 0 | — | 182 / 0 / 0 / 0x0000 / 1 / 18 | 10 / 30 | 15 | 2600 | 46 | 24 | 10 |
| crossbow | 364 | 18 | 0 / 0 | — | 364 / 0 / 0 / 0x0000 / 1 / 18 | 12 / 40 | 15 | 2800 | 27 | 30 | 12 |
| thrown | 182 | 91 | 0 / 0 | — | 0 / 0 / 0 / 0x0000 / 1 / 0 | — | 10 | 3400 | 0 | 12 | 6 |

Hip sway is 9 Bam for every ranged archetype. Reference masses (g): pistol 1000, heavy pistol 1600, SMG 2800, auto rifle 3600, marksman 4200, shotgun 3800, sniper 6500, launcher 7000, beam 4500, charge rifle 1500, crossbow 3500, thrown 300.

- **Hands.** Pistol, heavy pistol and thrown weapon are one-handed; every other ranged archetype is two-handed.
- **Harness** (`harness: true`; doc 04's quadruped plans carry such a weapon in the mouth): pistol, heavy pistol, SMG, auto rifle, beam and crossbow. A harness-carried weapon never aims down sights and fires at spread × 12500 (§6.3).
- **Aim assist** (`aim_assist_bp`, read by doc 17 §12.7 for controller look rate only, never by the simulation): sniper 7000, marksman rifle 8500, SMG, shotgun and beam 11000, every other archetype, melee included, 10000.
- **Block damage** (`block_bp`, doc 03 §13.2, destructible blocks only): bullets and shotgun pellets 1000, arrows and bolts (charge rifle, crossbow) 1000, beam pulses 1000, thrown weapons 2000, launcher direct hits 10000. Blasts break blocks through doc 03 §12.2 instead (§9.4).

**Table 5.2d — scaling, stealth, status and innate counters.**

| Archetype | Scaling | Sneak tier | Status multipliers | Innate counter (§15.3) |
|---|---|---|---|---|
| pistol | PRC 6000, SLT 4000, AGI 2000 | 2 | — | — |
| pistol_heavy | PRC 5000, GRP 5000, SLT 2000 | 2 | exposed 12500 | — |
| smg | GRP 5000, AGI 4000, SLT 3000 | 3 | exposed 7500 | — |
| rifle_auto | GRP 6000, PRC 4000, BRN 2000 | 3 | — | — |
| rifle_marksman | PRC 7000, SGT 3000, GRP 2000 | 2 | exposed 12500 | — |
| shotgun | GRP 5000, BRN 4000, PWR 3000 | 3 | exposed 7500 | — |
| sniper | PRC 7000, SGT 3000, FOC 2000 | 2 | exposed 15000 | `counter.ward_breaker` on precision hits |
| launcher | GRP 5000, BRN 4000, ANL 3000 | 3 | burning 12500 | `counter.staggerer` on direct hits |
| beam | FOC 5000, GRP 4000, PRC 3000 | 3 | burning 7500 | `counter.disruptor` from the 11th consecutive pulse on one target |
| charge_rifle | PRC 5000, PWR 4000, SGT 3000 | 2 | — | `counter.ward_breaker` at full charge |
| crossbow | PRC 6000, GRP 4000, SLT 2000 | 2 | exposed 12500 | `counter.ward_breaker` on precision hits |
| thrown | SLT 5000, PRC 4000, PWR 3000 | 2 | bleeding 12500 | — |

The Skill columns of Tables 3.2a and 5.2a are the Canon §7 mapping. Throwables use `skill.throwing`, shield bash and taunt use `skill.shield`, and quick melee uses `skill.unarmed`. The mapping is by archetype, never by skin.

### 5.3 Archetype-specific ranged rules

- **Charge rifle** (longbow, railgun and lightning-staff skins). Holding `input.attack` draws; `c` counts ticks from the press. Releasing at `c < 15` cancels with no ammunition spent. For `c >= 15`:

```
charge_bp        = min(10000, 4000 + floor_div(6000 × (c - 15), 30))
speed_mm_s       = min(100000, 40000 + floor_div(60000 × (c - 15), 30))
charge_spread_bp = max(10000, 20000 - floor_div(10000 × (c - 15), 30))
```

  The hit's attack factor and impact factor are `charge_bp`. From `c = 105`, ADS sway is multiplied by `min(30000, 10000 + 100 × (c - 105))`. Skins whose dependency set is empty (bows, slings) drain 6 stamina per second while `c > 45` through the Canon §2.3 accumulator; when stamina reaches 0 during that drain, the draw releases on that tick and fires normally (a forced release). After release, an 18-tick nock (`Reloading` with total 18, §6.1) loads the next round from reserve. Example: release at `c = 30` gives `charge_bp = 7000`, speed 70,000 mm/s and spread factor 15000.
- **Shotgun.** Each shell fires 8 pellets, each with its own two spread draws. Reloading inserts one shell per 32 ticks after a 20-tick start; `input.attack` with at least one shell loaded interrupts the reload and fires after a 6-tick exit.
- **Launcher.** The rocket's direct hit (if any) resolves first; the blast (§9.4, Table 9.3b: radius 3500, inner 1000, edge 2500) follows in C5 and excludes nothing. A rocket that reaches its range limit or its 600-tick age detonates where it is.
- **Beam.** While `input.attack` is held, one hitscan pulse fires on the press tick and every 6 ticks after, spending 1 cell unit each. The beam has `energy: true` (§8.4).
- **Crossbow.** Loaded state persists indefinitely; reloading is an 80-tick crank. Its `rpm` is 0, so the crank can start on the tick after the shot and the REF cycle is 81 ticks.
- **Thrown weapon.** Holding `input.attack` winds for at least 12 ticks; release throws. The interval from one release to the next press is 18 ticks, so the cycle is 30 ticks. Thrown weapons become ground items where they stop (§9.2). Knife, javelin and sling-stone skins all use this archetype (doc 11's kit rebinding maps doc 14's sling here).

### 5.4 Casting implements in doc 05 namespaces

Canon §19 gives `arch.ranged.` and `arch.offhand.` to doc 05, so these IDs are exported here. Doc 06 owns everything about casting with them (§8.5 casting fields, spell slots, bound spells).

| ID | Category | Hands | Ref mass (g) | Weapon function |
|---|---|---|---|---|
| `arch.ranged.staff` | `cat.weapon_ranged` (Canon §13.1 lists staves there) | 2 | 1530 | the `arch.melee.quarterstaff` moveset exactly: Tables 3.2a to 3.2d rows, `skill.polearm`, `block_bp` 10000, with §3.3 timing measured against `ref_mass_g` 1530 |
| `arch.ranged.wand` | `cat.wand` | 1 | 300 | none: `input.attack` casts the bound spell (doc 06 §8.4); `input.heavy` performs quick melee (§4.7) |
| `arch.offhand.tome` | `cat.offhand` | off-hand | 900 | none; cannot block or parry (§12.1) |

A staff's REF DPS is the quarterstaff's 29.0. Doc 06's `casting` record (`cost_bp`, `cast_speed_bp`, `tome_spell_slots`) is added to these three records and to `arch.offhand.focus`.

### 5.5 Calling reference weapons (doc 02 anchor duel)

Doc 02 §16 runs 100,000 duels per Calling against `enemy.role.soldier` (60 HP, AR 10) and requires time to kill 2.0 s ± 0.3 s and DPS 30 ± 4.5. Doc 05 assigns the weapons and the duel's starting state.

| Calling | Reference weapon | REF DPS | Calling | Reference weapon | REF DPS |
|---|---|---|---|---|---|
| `calling.vanguard` | `arch.melee.mace` | 30.2 | `calling.warden` | `arch.melee.spear` | 30.1 |
| `calling.duelist` | `arch.melee.blade_long` | 30.1 | `calling.oracle` | `arch.ranged.staff` | 29.0 |
| `calling.gunslinger` | `arch.ranged.pistol` | 30.2 | `calling.netrunner` | `arch.ranged.pistol` | 30.2 |
| `calling.grenadier` | `arch.ranged.rifle_auto` | 30.3 | `calling.beastbinder` | `arch.melee.spear` | 30.1 |
| `calling.alchemist` | `arch.melee.dagger` | 27.0 | `calling.monk` | `arch.melee.fist` | 27.1 |
| `calling.shade` | `arch.melee.blade_short` | 30.0 | `calling.spellwright` | doc 06's reference spell (doc 06 owns its anchor) | — |
| `calling.artificer` | `arch.ranged.pistol` | 30.2 | | | |

- **Starting state.** A melee duel starts in `Idle` with the target inside reach. A ranged duel starts at a phase drawn uniformly from `0..cycle` of the weapon's REF cycle (§1.1) with the harness's own seed, so a duel can begin mid-magazine or mid-reload; every projectile hits the torso.
- **Expected results** (integer pipeline, crits at 500 Bp): melee mean time to kill 1.84 s (mace) to 2.20 s (dagger); pistol 1.93 s; auto rifle 2.07 s. A ranged weapon that starts on a full magazine kills in 1.15 to 1.17 s, because one magazine holds 1.5 to 2 kills by design; sustained fire over many kills returns to the REF DPS.

---

## 6. Firing: modes, spread, bloom, recoil, ADS, sway and lean

### 6.1 Ranged weapon state machine

States: `Holstered`, `Drawing(k)`, `Ready`, `SprintLock(k)`, `Charging(c)`, `Beaming(k)`, `Stalled(k)`, `Burst(n, k)`, `Reloading(k)`, `Jammed`, `Clearing(k)`, `Staggered`, `Disarmed`. "Disabling status" is defined in §4.8. `total` for a reload is `action_ticks(reload_ticks)` (Canon §6.4 row 17), or `action_ticks(ceil_div(reload_ticks × 8500, 10000))` for a tactical reload with at least one round left; the shotgun and the charge rifle's nock use their own totals (§5.3).

| From | Event (first match wins) | To |
|---|---|---|
| any except `Holstered` | a disabling status applied | `Staggered`; a reload before `commit` moves nothing; a draw is cancelled with nothing spent; a beam stops |
| any except `Holstered`, `Staggered` | `st.disarmed` applied | `Disarmed` |
| any | the weapon is swapped out | `Holstered`; a reload before `commit` moves nothing; a jammed weapon stays jammed |
| `Holstered` | the weapon is selected | `Drawing(0)` |
| `Drawing(k)` | `k + 1 = action_ticks(swap_ticks)` (Canon §6.4 row 17) | `Jammed` if the weapon was holstered jammed, else `Ready` |
| `Staggered` | no disabling status remains | `Disarmed` if `st.disarmed` is active; else `Jammed` if jammed; else `Ready` |
| `Disarmed` | `st.disarmed` ended | `Jammed` if jammed, else `Ready` |
| `Ready` | `input.sprint` while moving forward | `SprintLock(0)`, held until sprint ends; no fire, no ADS |
| `SprintLock(k)` | sprint ended and `k >= sprint_to_fire` | `Ready` |
| `Ready` | `input.reload`, magazine < capacity, reserve > 0, `cool_mt <= 0` | `Reloading(0)` |
| `Ready` | charge mode: `input.attack` pressed, a round loaded | `Charging(0)` |
| `Ready` | beam mode: `input.attack` held, magazine ≥ 1000 mu | `Beaming(0)`; the first pulse fires this tick |
| `Ready` | trigger condition met (§6.2), `cool_mt <= 0`, magazine ≥ `ammo_per_shot` | the Contradiction cadence (§16.2) runs first: on a jam `Jammed` with no shot and no ammunition spent; otherwise fire; burst mode enters `Burst(1, 0)` |
| `Ready` | trigger condition met, `cool_mt <= 0`, magazine < `ammo_per_shot` | `Reloading(0)` if reserve > 0; else a dry-fire click (`emit_noise` 30 dB) and `Ready` |
| `Burst(n, k)` | `n = burst_n` or magazine < `ammo_per_shot` | `Ready` |
| `Burst(n, k)` | `k + 1 = burst_ticks` | fire with no new failure roll (§16.2); `Burst(n + 1, 0)` |
| `Charging(c)` | `input.dodge` or `input.sprint` | `Ready`; nothing spent |
| `Charging(c)` | release with `c < 15` | `Ready`; nothing spent |
| `Charging(c)` | release with `c >= 15`, or a forced release (§5.3) | the failure roll: on a misfire `Ready` with the round still loaded; otherwise fire and enter `Reloading(0)` with total 18 (the nock) when reserve > 0, else `Ready` |
| `Beaming(k)` | `input.attack` released, or magazine < 1000 mu | `Ready` |
| `Beaming(k)` | a due failure roll fails (§16.2) | `Stalled(0)` |
| `Beaming(k)` | `(k + 1) % 6 == 0` | pulse (§7.6); `Beaming(k + 1)` |
| `Stalled(k)` | `k + 1 = 60` | `Ready` |
| `Reloading(k)` | `input.sprint`, swap or `input.dodge` with `k + 1 < commit` | `Ready`; no rounds moved |
| `Reloading(k)` | shotgun: `input.attack` with at least one shell loaded | `Ready` after a 6-tick exit, then the trigger applies |
| `Reloading(k)` | `k + 1 = commit`, where `commit = mul_bp(total, 7500)` | rounds move from reserve to magazine (shotgun: one shell per 32 ticks instead, §5.3) |
| `Reloading(k)` | `k + 1 = total` | `Ready` |
| `Jammed` | `input.reload` | `Clearing(0)` |
| `Clearing(k)` | `k + 1 = action_ticks(45)` | `Ready`; one round is ejected from the magazine and lost |

Every (state, event) pair not listed leaves the state unchanged and advances its counter. A reload never starts while `cool_mt > 0`, so a magazine weapon's REF cycle is `magazine × interval + reload` and the launcher's is `60 + 90` (§1.1). A weapon in `Ready` with an empty magazine and the trigger held reloads on the first tick the interval allows.

### 6.2 Fire cadence and modes

The cadence accumulator is exact for any RPM:

```
interval_mt = if rpm == 0 { 0 } else { ceil_div(3_600_000 × 10000, rpm × atk_speed_bp) }   // milliticks
atk_speed_bp: 10000, then × shortfall factor (Canon §6.3), then × 13000 for an NPC with elite.surging (Canon §17.3a),
              then statuses in ascending ID: st.chilled 8000, st.hasted 11500        // non-status factors first, Canon §11.7
in C1:  if trigger_met && cool_mt <= 0 { fire(); cool_mt += interval_mt }
in C8:  cool_mt -= 1000
        if !trigger_held { cool_mt = max(cool_mt, 0) }              // no banking while idle
```

Shot `n` (0-based) of a continuous string that starts on tick `t0` fires on tick `t0 + ceil_div(n × interval_mt, 1000)`. At 900 RPM `interval_mt = 4000`, so shots land every 4 ticks. At 750 RPM `interval_mt = 4800`, so shots land on ticks 0, 5, 10, 15, 20, 24, … and shot 750 lands on tick 3,600.

| Mode | Trigger condition |
|---|---|
| `move.fire_semi` | a press; a press during cooldown is buffered 6 ticks |
| `move.fire_auto` | held |
| `move.fire_burst` | a press fires `burst_n` (default 3) shots `burst_ticks` (default 3) apart, then the normal interval; used only by doc 11 overrides |
| `move.fire_bolt` | a press; the interval includes the cycling action; ADS persists |
| `move.fire_charge` | release after a draw (§5.3) |
| `move.fire_beam` | held; pulses every 6 ticks |
| `move.throw` | release after a wind of at least 12 ticks |

Melee phases use the same `atk_speed_bp`: W, A, R and `charge_full` each last `max(1, ceil_div(ticks × 10000, atk_speed_bp))` ticks after the §3.3 mass scaling, and `charge_full` never drops below 20 (§4.2).

### 6.3 Spread

```
ads_frac: +1 per tick while input.alt is held and ADS is allowed, else -2 per tick; range 0..=ads_ticks
base  = if ads_ticks == 0 { hip } else { hip + floor_div((ads - hip) × ads_frac, ads_ticks) }
cone  = base + bloom
cone  = mul_bp(cone, stance_bp)                // Table 6.3
cone  = mul_bp(cone, row15_bp)                 // Canon §6.4 row 15 (PRC), after doc 04 RowPost terms
cone  = mul_bp(cone, charge_spread_bp)         // charge rifle only, else 10000
cone  = mul_bp(cone, offhand_bp)               // 12500 for off-hand or harness fire, else 10000
cone  = mul_bp(cone, tox_bp)                   // 12000 in doc 08's sickened or overdosed toxicity band, else 10000
statuses, ascending ID: st.blinded 20000, st.focused 7000, st.mounted 13000
cone  = min(cone, 8192)
```

**Table 6.3 — stance multipliers.** The ADS column applies when `ads_frac = ads_ticks`; otherwise the hip column applies.

| Stance | Hip | ADS |
|---|---|---|
| Standing still | 10000 | 10000 |
| Crouched still | 8500 | 8500 |
| Walking (horizontal speed > 500 mm/s) | 15000 | 12500 |
| Crouch-walking | 12000 | 11000 |
| Airborne | 25000 | 20000 |
| Swimming (one-handed ranged only) | 20000 | — |

**Sampling.** Each projectile draws two values from the shot's keyed generator (§1.4):

```rust
fn spread_offset(rng: &mut Rng, cone: u32) -> (i32, i32) {
    let az = rng.range(65536) as u16;                  // draw 1: azimuth
    let u  = rng.range(65536);                         // draw 2: radial sample
    let rad = ((cone as u64) * isqrt_u64(u << 16)) >> 16;      // uniform over the disk
    let dyaw   = ((rad as i64) * (cos_bam(az) as i64)) >> 30;  // arithmetic shift = floor
    let dpitch = ((rad as i64) * (sin_bam(az) as i64)) >> 30;
    (dyaw as i32, dpitch as i32)
}
shot_yaw   = aim_yaw.wrapping_add(dyaw as u16)
shot_pitch = clamp(aim_pitch + dpitch, -16000, 16000)
direction  = fwd(shot_yaw, shot_pitch)
```

No `1/cos(pitch)` correction is applied; cones above ±60° pitch are slightly narrower in yaw, which is accepted.

### 6.4 Bloom

A shot reads the current `bloom`, then sets `bloom = min(bloom + bloom_per_shot, bloom_max)`. In C8, when `t - last_shot_tick >= bloom_delay`, `bloom = max(0, bloom - mul_bp(bloom_decay, shortfall_bp))`, where `shortfall_bp` is the Canon §6.3 factor (10000 with no shortfall).

### 6.5 Recoil: deterministic pattern

Recoil is simulation state: `rec_pitch` and `rec_yaw` in Bam are added to the command's aim, so the client predicts the same view kick the host computes. The pattern is fixed per archetype (doc 11 manufacturers can replace `sig`), so players learn it; one keyed draw adds jitter.

```
on each shot, after its projectiles spawn:
  if t - last_shot_tick > 30 || (rec_pitch == 0 && rec_yaw == 0) { k = 0 }
  v = if kneel == 0 || k < kneel { v_bam } else { mul_bp(v_bam, 6000) }
  seg = (k / period) % 16
  h = if (sig >> seg) & 1 == 1 { -h_bam } else { h_bam }    // bit 1 kicks right (yaw decreases)
  h = h + (jitter_draw as i32 - jitter_bam)                  // jitter_draw = rng.range(2 × jitter_bam + 1): the shot's first
                                                             // seq-0 draw after any failure roll (§1.4), taken before the
                                                             // spread draws and applied here, after the projectiles spawn
  for x in [v, h]: x = mul_bp(x, row16_bp)                   // Canon §6.4 row 16 (GRP)
                   x = mul_bp(x, 8000) if ads_frac == ads_ticks
                   x = mul_bp(x, 8500) if crouched; mul_bp(x, 12500) if airborne
                   x = mul_bp(x, mass_recoil_bp)              // §3.3
  rec_pitch = min(rec_pitch + v, 4096)
  rec_yaw   = clamp(rec_yaw + h, -2048, 2048)
  k += 1
in C8, when t - last_shot_tick >= delay:
  each axis moves toward 0 by mul_bp(rate_bam, shortfall_bp), never overshooting
aim_yaw   = cmd.yaw.wrapping_add((rec_yaw + sway_yaw) as u16)
aim_pitch = clamp(cmd.pitch + rec_pitch + sway_pitch, -16000, 16000)
```

**Worked example.** An auto rifle (`v` 73, `h` 36, `kneel` 8, `sig` 0x6666, `period` 3) fired for 10 shots from the hip, standing, GRP 40, with recovery suppressed because the 6-tick interval is below the 8-tick delay. Ignoring jitter, pitch climbs `8 × 73 + 2 × 43 = 670` Bam (3.7°). Segments 0, 1, 2, 3 read bits 0, 1, 1, 0 of `0x6666`, so yaw moves `+108, -108, -108, +36`, a net `-72` Bam: up-left, then right, then back left.

### 6.6 ADS and sprint lock

- ADS is allowed when the weapon has `ads.ticks > 0`, is not harness-carried, and the wielder is not sprinting, not in `SprintLock`, not swimming and not the driver of `arch.mount.hover`.
- ADS changes the stance base speed (§1.6), spread (§6.3), recoil (§6.5) and sway (§6.7). The renderer's zoom is presentation only.
- After sprint ends, the weapon cannot fire or raise ADS for `sprint_to_fire` ticks.

### 6.7 Aim sway

Sway is a deterministic Lissajous drift with no random draws:

```
amp = 9 + floor_div((ads_sway - 9) × ads_frac, ads_ticks)
amp = mul_bp(amp, breath_bp)      // 2500 while holding breath, 15000 while exhaling, else 10000
statuses, ascending ID: st.focused 5000, st.madness 20000
sway_yaw   = (amp × sin_bam((t × 364) as u16)) >> 30
sway_pitch = (floor_div(amp, 2) × sin_bam((t × 728) as u16)) >> 30
```

**Hold breath** (`move.hold_breath`): `input.sprint` while fully aimed with `ads_sway >= 27`. It lasts up to 300 ticks and costs 10 stamina per second through the Canon §2.3 accumulator, then forces a 120-tick exhale.

### 6.8 Lean (`move.lean_l`, `move.lean_r`)

```
lean_k ∈ -9..=9          // negative = left
each tick: lean_l held → lean_k = max(lean_k - 1, -9); lean_r held → lean_k = min(lean_k + 1, 9);
           neither → lean_k moves 1 toward 0
offset_mm = sign(lean_k) × floor_div(350 × |lean_k|, 9)
right     = (cos_bam(yaw), 0, -sin_bam(yaw))            // Q2.30; yaw 0 gives +X
eye_lean  = eye + right × offset_mm
```

- **Wall clamp.** If any solid box lies strictly closer than 100 mm to `eye_lean` (a sphere of radius 100 mm overlapping it), `|offset_mm|` becomes `max(0, |offset_mm| - 50)` and the test repeats until it passes or the offset is 0. The clamp is recomputed every tick from `lean_k`, and `lean_k` itself keeps moving toward its target, so the eye never oscillates. This rule replaces doc 03 §5.8's point test (Canon gaps).
- The capsule stays put. `bp.head` translates with the eye; no other part moves.
- Shots and melee pivots originate from the leaned eye. ADS works while leaning.
- Lean is unavailable while sprinting, climbing, swimming, mounted, downed or during a melee swing. Lean offsets by `|lean_k|`: 1 → 38 mm, 5 → 194 mm, 9 → 350 mm.

---

## 7. Ballistics

### 7.1 Per-tick projectile integration

```rust
pub struct Projectile {
    id: EntityId, owner: EntityId, item_uid: Option<u64>, arch: Id32,
    pos: V3, vel: V3, spawn_y: Fx, traveled_mm: i64, age: u32,
    pierce_left: u8, wallpen0: i32, wallpen_left: i32, keep_bp: i32, ricochets: u8,
    attack_bp: i32,                 // charge_bp or 10000
    hit_set: BTreeSet<EntityId>,    // entities already hit
    counters: u8, flags: u32,
}
// spawn (C2): pos = eye_lean; vel = mul_fx(dir_fx, fx_from_milli(speed_mm_s)); dir_fx = Q2.30 forward << 2
// each tick (C4), ascending id:
let g = mul_bp(local_gravity_mm_s2(pos), arch.gravity_bp);          // GRAVITY_BASE × genre gravity_bp (doc 12)
vel.y = max(vel.y - mul_fx(fx_from_milli(g), DT), -fx_from_milli(TERMINAL_VELOCITY));
let p_new = pos + mul_fx(vel, DT);                                  // semi-implicit Euler (Canon §2.1)
let end = sweep_and_resolve(&mut proj, pos, p_new);                  // §7.2–§7.5; returns the stop or end point
proj.traveled_mm += mm(sqrt_fx(dist2(pos, end)));
proj.pos = end; proj.age += 1;
let limit = arch.max_range_mm + max(0, mm(proj.spawn_y - proj.pos.y));   // Canon §10.5 step 2.4a range rule
if proj.traveled_mm > limit || proj.age >= 600 { despawn(proj) }      // ev.entity_despawned
```

The owner's velocity is never added to a projectile. A projectile ignores its owner's capsules until its first ricochet. `mm(x)` converts `Fx` meters to integer mm with `floor_div(x.raw × 1000, 2^32)`.

### 7.2 Entity hits and body penetration

- The sweep reports the earliest capsule (§2.4). If the target's guard is raised and the incoming direction is inside its arc (§12.2), the hit is blocked and the projectile stops.
- The hit's attack factors (§10.1) are `attack_bp`, range falloff at `dist_mm = traveled_mm + mm(|hit point − pos|)`, and `keep_bp`.
- **Body penetration.** If `pierce_left > 1`, the projectile continues through the target from the exit side with `pierce_left -= 1` and `keep_bp = mul_bp(keep_bp, 6000)`, adding the target to `hit_set`. Otherwise it stops. Stopped arrows, bolts and thrown weapons stick (§9.2); bullets vanish.

Falloff:

```
falloff_bp(d) = 10000                                   if d <= start
              = min_bp                                  if d >= end
              = 10000 - floor_div((10000 - min_bp) × (d - start), end - start)   otherwise
```

Example: a pistol hit at 30,000 mm gives `10000 - floor_div(5000 × 15000, 20000) = 6250`.

### 7.3 Block penetration and block damage

When the sweep enters a solid cell of material `m`:

1. **Penetrate** if `wallpen_left >= m.pen_cost` (doc 07; reference values glass 2, leaves 1, wood 40, dirt 60, stone 120, brick 150, iron 300, `block.void_bedrock` unbounded): `wallpen_left -= m.pen_cost`, `keep_bp = mul_bp(keep_bp, 10000 - floor_div(5000 × m.pen_cost, wallpen0))`, and the projectile continues through the cell.
2. Otherwise **ricochet** if §7.5 allows.
3. Otherwise **stop** at the entry point.

Each cell the projectile enters, penetrated or not, takes `damage_block(cell, mul_bp(B_proj, block_bp), shares)` where `B_proj` is the projectile's Canon §10.4 magnitude times its current attack factors. `block_bp`: bullets and pellets 2500, arrows and bolts 1000, launcher direct hits 10000, thrown weapons 2500, beam pulses 1000, melee swings 5000 (once per swing). Doc 03 decides breakage.

Example: a sniper bullet (`wallpen0` 160) passing one wood block (`pen_cost` 40) keeps `10000 - floor_div(200000, 160) = 8750` and has 120 wallpen left.

**Liquids.** A projectile entering a liquid cell triggers the Canon §12.1 reaction for its largest-share elemental type once per cell. Physical projectiles continue with velocity × 3000 per liquid cell. Beams stop at the first liquid cell.

### 7.4 Projectile versus guard, ward and shields

A blocked projectile resolves through Canon §10.5 step 6b. Ward absorption (step 7) never stops or deflects a projectile; the projectile stops at the target as with any hit.

### 7.5 Ricochet

Ricochet is deterministic. For a projectile with `ricochet: true` and `ricochets < max_ricochets` (1, or 2 with a doc 11 perk) that reaches a face whose material has `ricochet: true` (doc 07: stone, brick, metal, ice):

```
vn  = the velocity component along the face's axis (raw i64)
grazing = 4 × (vn as i128)² < Σ (v_i as i128)²        // incidence more than 60° from the normal
if grazing:
    v_axis = -v_axis;  vel = mul_bp_fx(vel, 6000) per component;  keep_bp = mul_bp(keep_bp, 6000);  ricochets += 1
    continue from the contact point; the owner becomes hittable
else: stop
```

Example: velocity `(100000, −20000, 0)` mm/s onto a stone floor: `4 × 20000² = 1.6e9 < 1.04e10`, so it ricochets to `(60000, 12000, 0)` mm/s with damage keep 6000.

### 7.6 Hitscan beams

Each pulse casts a ray from the leaned eye along the pulse direction (spread applied per pulse) to `max_range_mm`. The voxel walk stops at the first cell that is solid and opaque; non-opaque solids (glass, leaves) take block damage and are passed. Capsules are tested along the ray (rewound for Errata). The first entity hit receives the pulse. `beam_streak` counts consecutive pulses on the same target, resetting when a pulse hits nothing or another target; from streak 11 each pulse carries `counter.disruptor`. A fire beam that hits one flammable block for 10 consecutive pulses ignites it (doc 03).

---

## 8. Ammunition

### 8.1 Families

Ammunition is family-generic: any weapon of a family fires any `ammo.` item of that family, and skins rename it (arrows, ferro slugs and lightning motes are all `ammo.draw`). Every ammunition item has an empty dependency set; Contradiction comes from the weapon.

| ID | Used by | Carry cap | Mass per unit (g) | Pickup size |
|---|---|---|---|---|
| `ammo.light` | pistol, SMG | 240 | 8 | 32 |
| `ammo.heavy` | heavy pistol, auto rifle | 180 | 15 | 24 |
| `ammo.rifle` | marksman rifle, sniper | 60 | 25 | 8 |
| `ammo.shell` | shotgun | 48 | 40 | 8 |
| `ammo.cell` | beam | 300 units | 10 | 60 |
| `ammo.ordnance` | launcher | 8 | 1500 | 1 |
| `ammo.draw` | charge rifle, crossbow | 60 | 30 | 10 |
| `ammo.charge_breaker`, `ammo.charge_disruptor`, `ammo.charge_staggerer`, `ammo.charge_sunder`, `ammo.charge_anchor` | any weapon (§15.3) | 5 each | 100 | 1 |

```json
{ "id": "ammo.rifle", "category": "cat.ammo", "carry_cap": 60, "mass_g": 25, "pickup": 8, "value_lucre": 3 }
```

Ammunition mass counts toward carried mass (Canon §11.1 `load_bp`). Ammunition in a magazine stays with the weapon.

### 8.2 Pickups

An Erratum within 1000 mm of an ammunition pickup collects it automatically when its reserve for that family is below the cap: it takes `min(pickup, cap - reserve)` and the rest stays as a smaller pickup. Example: reserve 230 `ammo.light` and a 32-round pickup leave reserve 240 and a 22-round pickup.

### 8.3 Drop need (doc 11 interface)

Doc 11 owns drop rolls (keyed `rng.loot`). For each family `f` used by the receiving player's `slot.weapon_1..3`, `need_bp(f) = 10000 - floor_div(reserve_f × 10000, cap_f)`. An ammunition drop picks its family with `weighted(need_bp)`; when every need is 0, no ammunition drops. Doc 05's target supply is one pickup per 4 standard kills, which keeps every weapon fed at REF DPS.

### 8.4 Energy recharge

Weapons with `energy: true` store their magazine in milli-units (`mag_mu`; each pulse spends 1,000). Starting 120 ticks after the last pulse, they regenerate without spending reserve at `R = mul_bp(6000, tec_bp)` milli-units per second through the Canon §2.3 accumulator (`k = 60`), never above capacity. `tec_bp = 10000 + (TEC - 5) × 1000` (Canon §5.7) when the skin's dependency set contains techne, else 10000. At TEC 9 a techne beam regenerates 8,400 mu (8.4 pulses) per second.

### 8.5 NPC ammunition

NPCs never track reserve. Their templates (doc 13) give magazine and reload ticks; they reload when empty.

---

## 9. Throwing, grenades and explosives

### 9.1 Throw velocity

Canon §6.4 row 20 multiplies range. Range grows with the square of speed, so speed takes the square root:

```
throw_v_bp = isqrt(row20_bp × 10000)
v          = mul_bp(base_speed, throw_v_bp)       // grenades 16000 mm/s, thrown weapons 30000 mm/s
```

Example: PWR 60 gives row 20 = 11000, `throw_v_bp = isqrt(110,000,000) = 10488`, and a grenade leaves at 16,780 mm/s. An underhand throw (`input.throw` while crouched) uses `mul_bp(v, 5000)` and a pitch 1820 Bam below aim. Throws use the throwable's spread (`hip` 182 Bam) with two draws.

### 9.2 Throw actions

- `input.throw` with a throwable on the quickbar enters `Cooking(c)` (`move.cook`); `c = 0` on the press tick. The fuse of a fused grenade counts from the press. Release at `c >= 12` throws (`move.throw`); recovery is 18 ticks. A two-handed weapon is lowered during the throw and cannot fire.
- A fuse reaching 0 in the hand detonates at `eye + (0, −300 mm, 0)`.
- Thrown weapons that hit an entity stick to it and drop as ground items where it dies. Those that hit a block stop and become ground items at the contact point; walking over one returns it to its stack. Each throw costs 1 durability (doc 07). Items that fall into lava or leave the Folio are destroyed.
- Fused grenades bounce: on a block contact the normal velocity component reverses and is multiplied by 4000 and the tangential components by 8000; the grenade rests when its speed falls below 500 mm/s. An entity contact stops horizontal motion.

### 9.3 Grenade archetypes

All throwables use skill `skill.throwing`, scaling SLT 5000, PRC 4000, PWR 3000, base speed 16000 mm/s, gravity 10000, radius 50 mm, precision never, and count as weapon attacks for Canon §6.4 row 12.

| Archetype | Detonation | Fuse (ticks) | `base_mhp` at center | Shares | Radius / inner (mm) / edge Bp | Impact | Status power | `block_bp` | Mass (g) |
|---|---|---|---|---|---|---|---|---|---|
| `arch.ranged.grenade_frag` | fuse | 150 | 48000 | pierce 6000, blunt 3000, fire 1000 | 4500 / 1000 / 2000 | 40 | default | 10000 | 400 |
| `arch.ranged.grenade_incendiary` | first contact | — | 20000 | fire 10000 | 3000 / 3000 / 10000 | 0 | burning 60 | 2500 | 500 |
| `arch.ranged.grenade_concussion` | fuse | 90 | 15000 | blunt 10000 | 4000 / 1500 / 3000 | 80 | stunned 50 | 5000 | 400 |
| `arch.ranged.grenade_flash` | fuse | 90 | 2000 | radiant 5000, sonic 5000 | 6000 / 2000 / 5000 | 0 | blinded 150, deafened 150 | 0 | 300 |
| `arch.ranged.grenade_smoke` | fuse | 60 | 0 | — | releases `gas.smoke` in the 27 cells around the center at full density for 600 ticks (doc 08) | 0 | — | 0 | 300 |
| `arch.ranged.grenade_counter` | fuse | 90 | 12000 | blunt 10000 | 3000 / 3000 / 10000 | 20 | default | 0 | 400 |

- Incendiary grenades also ignite flammable blocks within 2000 mm (doc 03) and place a burning surface. Flash blinding power is multiplied by 10000 when `ang_diff(target.yaw, bearing from target to center) <= 10923`, else by 3000.
- `grenade_counter` carries exactly one counter bit set by its item template (doc 11), chosen from §15.1.
- Each throwable is one `cat.throwable` item; doc 08 owns alchemical flasks that reuse these archetypes with liquid payloads.

```json
{ "id": "arch.ranged.grenade_frag", "category": "cat.throwable", "skill": "skill.throwing",
  "fuse_ticks": 150, "base_mhp": 48000, "shares": { "dmg.pierce": 6000, "dmg.blunt": 3000, "dmg.fire": 1000 },
  "radius_mm": 4500, "inner_mm": 1000, "edge_bp": 2000, "impact": 40, "block_bp": 10000,
  "speed_mm_s": 16000, "gravity_bp": 10000, "mass_g": 400,
  "scaling": { "sub.sleight": 5000, "sub.precision": 4000, "sub.power": 3000 } }
```

### 9.4 Explosion resolution and AoE falloff

Every explosion (grenades, launcher blasts, doc 07 gadgets, doc 08 flasks) resolves in C5 in ascending source `EntityId`:

```
1. Blocks: for every block whose center lies within radius (ascending block index):
       damage_block(b, mul_bp(mul_bp(B, falloff(dist_to_block_center)), block_bp), shares)
   Doc 03 breaks blocks and draws debris variation from rng.physics.
2. Entities, ascending EntityId, over entities with any capsule within radius:
       tp   = closest point to the center on the axis of the target's largest-share part capsule
       dist = mm(sqrt_fx(dist2(center, tp)))
       if dist > radius: skip
       if voxel walk center → tp meets a cell that is solid and opaque (after step 1): skip
       falloff_bp = if dist <= inner { 10000 }
                    else { 10000 - floor_div((10000 - edge_bp) × (dist - inner), radius - inner) }
       push HitRecord { kind: Area, part: largest-share part, flags: AREA, attack factor falloff_bp,
                        impact × falloff_bp, status powers × falloff_bp, incoming = center - tp }
3. The owner and allies are included. Friendly fire applies FF_AREA_BP at step 2.15 (Canon §17.5),
   so self-damage is full by default and halved in mode.ff_off.
```

Area hits roll crits (one draw each, §1.4) and never count as precision. They take step 2.10's × 15000 against `body.swarm`. Only tower shields block area hits (§12.1). A launcher blast within its inner radius carries `SEVER`.

---

## 10. The damage pipeline in full

Canon §10.5 fixes the step order. This section adds the doc-05 inputs and rules at each step and changes nothing in the canonical order.

### 10.1 Attack factors (before step 0)

A hit's `base_mhp` is the archetype's `base_mhp` (NPCs: the template's `template_dmg_mhp`) multiplied, one `mul_bp` each and in this order, by every factor that applies:

1. move factor: combo step `move_bp`, `heavy_bp` (§4.2), sprint 13000, jump (§4.4), riposte 25000, off-hand 8500;
2. `charge_bp` (charge rifle);
3. mounted charge 12500 (Canon §17.7; §17.3);
4. whip sweet-spot 6000 (§3.2);
5. range falloff (§7.2);
6. `keep_bp` from walls, bodies and ricochets (§7.3–§7.5);
7. cleave factor 10000 / 7500 / 5000 (§2.3);
8. area falloff (§9.4).

The result then enters Canon §10.4 (`× P(src_lvl) × rarity × quality × enhancement × material` for Errata items; `× T(d) × rank damage` for enemies). Impact uses factors 1, 2, 3 (× 20000 instead of 12500) and 8; status power uses factors 1, 2 and 8.

### 10.2 Reference implementation

```rust
fn resolve_hit(h: &HitRecord, w: &mut World) -> Option<DamageEvent> {
    let (att, tgt) = (w.ent(h.owner), w.ent_mut(h.target));
    if tgt.dead || tgt.has(st_deaths_door) || tgt.dodge_invulnerable(w.t) { return None; }
    // Parry and block are decided first from target state at tick t (§2.6, §12.2)
    let guard = guard_outcome(h, tgt, w.t);            // None | Blocked | Parried
    if guard == Parried { apply_parry(h, w); return None; }
    let b = source_magnitude(h);                        // §10.1 then Canon §10.4
    // Step 1: additive bucket
    let mut add = if h.is_weapon_attack() && att.is_erratum() { row12(att, h.item) } else { 0 };
    add += Σ mul_bp(bonus_i, potency_i);  add = max(add, -9000);
    let mut x = mul_bp(b, 10000 + add);
    // Step 2: chain, each one mul_bp, in canonical order
    let sneak = doc09::sneak_eligible(att, tgt, w.t);
    if !h.area() && tgt.part(h.part).precision { x = mul_bp(x, h.precision_bp); }            // 2.2
    if !sneak { let mut r = keyed_rng("rng.combat", folio, h.source, w.t, h.seq);
                if r.roll_bp(crit_chance(att)) { x = mul_bp(x, crit_mult(att)); } }          // 2.3
    if sneak { x = mul_bp(x, doc09::sneak_mult(att, h.sneak_tier)); }                        // 2.4
    x = mul_bp(x, elevation_bp(att, tgt));                                                   // 2.4a §10.3
    x = mul_bp(x, flank_bp(att, tgt));                                                       // 2.4b §10.3
    x = mul_bp(x, potency(h.item_uid, att));                                                 // 2.5
    if is_native(tgt) { x = mul_bp(x, 10000 + edge_bp(h.item_uid, att)); }                   // 2.6
    if tagged_mythic(att) || tagged_mythic(tgt) { x = mul_bp(x, 10000 + (myt - 5) * 300); } // 2.7
    x = mul_bp(x, shortfall_bp(att, h.item));                                                // 2.8
    x = mul_bp(x, tgt.part(h.part).hit_mult);                                                // 2.9
    if h.area() && tgt.plan == body_swarm { x = mul_bp(x, 15000); }                          // 2.10
    for s in tgt.statuses_ascending() { x = mul_bp(x, target_status_bp(s)); }                // 2.11
    for s in att.statuses_ascending() { x = mul_bp(x, attacker_status_bp(s, h.target)); }   // 2.12
    x = mul_bp(x, doc14::difficulty_bp(att, tgt));                                           // 2.13
    if att.is_npc() && tgt.is_erratum() { x = mul_bp(x, party_dmg_bp(w.party_size)); }      // 2.14
    if att.faction == fac_errata && tgt.faction == fac_errata {                              // 2.15
        x = mul_bp(x, if h.ff_area_class() { FF_AREA_BP } else { FF_DIRECT_BP }); }
    let d2 = x;
    // Steps 3–6 exactly as Canon §10.5; AR_part adds the shield's AR when guard == Blocked (§12.2)
    let mut d = split_by_type(d2, &h.shares);                  // step 3, remainder to largest share
    per_type_target_mods(&mut d, tgt);                          // 3a: radiant vs undead/fiend; frozen blunt
    apply_resistance(&mut d, tgt);                              // step 4
    if !h.flags.ignore_armor() { apply_armor(&mut d, ar_for(tgt, h.part, guard), h.pen_bp); }  // step 5
    apply_chip_floor(&mut d, d2, tgt);                          // step 6
    let mut stamina_loss = 0;
    if guard == Blocked { stamina_loss = apply_block(&mut d, h, tgt); }                      // 6b, §12.3
    let ward_lost = apply_ward(&mut d, h, tgt);                                              // step 7, §15.4
    let total: i64 = d.iter().sum();
    apply_hp_condition_poise_buildup(h, tgt, total, guard, w);                               // step 8, §12.4, §12.5, §13
    shock_of_the_new(h, tgt, w);                                                             // Canon §5.6
    apply_counters(h, tgt, ward_lost, w);                                                    // §15.1
    check_sever(h, tgt, w);                                                                  // §19.1
    resolve_lethal(tgt, w);                                                                  // Canon §10.8
    Some(emit_damage_dealt(h, &d, ward_lost, stamina_loss))                                  // step 9
}
```

- **Step 2.12 and `st.taunted`.** The attacker statuses in ascending ID order are `st.berserk` 13000, `st.empowered` 12000, `st.taunted` 7500 (only when the target is not the taunter) and `st.weakened` 7500.
- **Step 2.11.** `st.exposed` `10000 + 800 × stacks`, then `st.knocked_down` 11000.
- **Crit chance and multiplier** are Canon §6.4 rows 13 and 14 for Errata. NPCs use row 13 without gear and multiplier 15000.
- **Step 9.** `ev.damage_dealt` carries per-type HP damage, `ward_lost`, `stamina_loss_mst` (blocked hits), `crit`, `precision`, `sneak`, `blocked`, `kind`, `counters`, and the hit part.

### 10.3 Elevation and flanking

```
dy_mm = mm(att.pos.y - tgt.pos.y)                    // origins, Canon §1.8
elevation_bp = if dy_mm >= 2500 { 11000 } else if dy_mm <= -2500 { 9500 } else { 10000 }
dx, dz = att.pos.x - tgt.pos.x, att.pos.z - tgt.pos.z
bearing = atan2_bam(-dx, -dz)                        // yaw whose fwd() points from target to attacker
flank_bp = if ang_diff(tgt.yaw, bearing) > 10923
              && (tgt.is_erratum() || doc13::ai_target(tgt) != Some(att.id)) { 11500 } else { 10000 }
```

Both apply to melee, projectiles, hitscan and area hits. For projectiles and area hits, `att.pos` is the owner's position on the impact tick. Ranged maximum range grows by `max(0, dy)` through §7.1.

### 10.4 Worked examples

All examples use canon tables and integer arithmetic exactly as §10.2 runs it. "AR" is the hit part's armor.

**E1 — Crit (long blade finisher, REF target).** Base `mul_bp(14700, 13000) = 19110`; Common Normal +0 iron at ilvl 1 leaves B = 19,110. Skill 20 gives `add = 600`: D1 = 20,256. The crit roll succeeds (×15000): D2 = 30,384. Split: slash 24,308 (24,307 plus the 1 mHP remainder), pierce 6,076. Armor: slash effAR 10, `DR = floor(10^8 / 107,232) = 932`, slash 22,043; pierce effAR 7, DR 2,236, pierce 4,718. **Total 26,761 mHP** (displays 26). The same swing without the crit deals 16,871.

**E2 — Precision (marksman rifle headshot at 40 m).** Rare (11000), Good (11000), +2, ilvl 5, iron. Attacker PRC 60, SGT 50, GRP 40, Marksmanship 35: `add = floor_div(7000 × 20, 100) + floor_div(3000 × 10, 100) + 35 × 30 = 2750`. B: 11,600 → `× P(5)` 16,373 → × 11000 18,010 → × 11000 19,811 → × 11200 22,188. D1 = 28,289. Head is a precision part: × 17500 = 49,505. The Folio-5 standard target has AR `mul_bp(10, P(5)) = 14`; pierce effAR `mul_bp(floor_div(14 × 7500, 10000), 7000) = 7`; DR 341. **Total 47,817 mHP**, 44.8% of that enemy's 106,704 mHP. The same shot to the torso deals 26,643.

**E3 — Sneak attack with flanking (full heavy dagger).** Attacker PRC 50, AGI 45, SLT 40, Blade 30: `add = 500 + 200 + 0 + 900 = 1600`. Base `mul_bp(10100, 22000) = 22,220`; D1 = 25,775. The target is unaware (sneak eligible, so no crit draw); doc 09's tier-1 multiplier 30000 gives 77,325. The target is not targeting the attacker and the attacker is behind it: × 11500 = 88,923. Split: pierce 53,354, slash 35,569. Armor DR 317 (pierce) and 656 (slash). **Total 84,899 mHP**: a one-hit kill on a 60 HP REF enemy.

**E4 — Contradiction and Anachronism (Canon §5.6 smart SMG in Fairy Tale).** SMG ilvl 3, Uncommon (10500), C = 4 (potency 6000, failure 1000), E = 6 (`edge_bp` 2400). B: 4,500 → `× P(3)` 5,346 → × 10500 5,613. D1 = `mul_bp(5613, 10600)` = 5,949. Potency: 3,569. Target AR `mul_bp(10, P(3)) = 11`, pierce effAR 8.

| Case | After step 2 | DR | Per bullet |
|---|---|---|---|
| Coherent SMG (reference) | 5,949 | 2,516 | 4,453 |
| Contradicted, non-native target | 3,569 | 3,591 | 2,288 |
| Contradicted, native wolf (× 12400) | 4,425 | 3,112 | 3,048 |

The first hit on each native non-boss target applies `st.staggered` (Shock of the New). Firing rolls a 1000 Bp jam on the first shot of each trigger hold and then once per 30 ticks (§16.2).

**E5 — Armor against SMG and sniper (Folio 1, Common).** SMG D1 = 4,770 per bullet, `pen_bp` 0; sniper D1 = 43,672, `pen_bp` 5000.

| Target AR | SMG effAR / DR / per bullet | Sniper effAR / DR / per shot | SMG REF DPS | Sniper REF DPS |
|---|---|---|---|---|
| 10 | 7 / 2,684 / 3,490 | 3 / 168 / 42,939 | 30.3 | 24.0 |
| 40 | 30 / 6,112 / 1,855 | 15 / 790 / 40,222 | 16.3 | 22.5 |

Armor removes 61% of each SMG bullet at AR 40 and 8% of the sniper shot; against armored enemies the sniper overtakes the SMG.

**E6 — Blocked hit (NPC on an Erratum with a heater shield).** REF enemy hit `template_dmg_mhp` 10,000 slash at Folio 1, party of 2 (× 11000): D2 = 11,000. Torso AR 20 plus heater AR 25 = 45: `DR = floor(4.5 × 10^8 / 89,000) = 5056`, leaving 5,439. Block stability `5000 + 0 (BRN 40) + 2500 (heater) + 600 (Shield 20, §12.2) = 8100`: `to_stamina = mul_bp(5439, 8100) = 4,405`, **HP loss 1,034**, stamina loss `floor_div(4405 × 10000, T(1)) = 4,405` mst. Unblocked, the same hit deals 7,563.

**E7 — Warded champion and `counter.ward_breaker`.** A Folio-1 champion with template HP 60: Max HP `60,000 × H(1) × 60000 / 10000 = 360,000` mHP; `elite.warded` gives ward 180,000. A Common sniper torso shot with a breaker charge: after armor 42,939 pierce. Step 7 uses `ward_mult_eff = mul_bp(10000, 30000) = 30000`: `w = 128,817 <= 180,000`, so the ward drops to 51,183 and HP takes 0. Second shot: `w = 128,817 > 51,183`, HP takes `42,939 - floor_div(51,183 × 10000, 30000) = 25,878`, `ev.ward_broken` fires and the counter applies `st.stunned` directly: `mul_bp(90, clamp(floor_div(10^6, 100), 2500, 15000)) = 90` ticks, × champion control 5000 = **45 ticks**. Without the counter the first shot leaves ward 137,061 and the ward needs 5 shots.

**E8 — Frag grenade and friendly fire.** Center 48,000; radius 4,500, inner 1,000, edge 2000. An NPC at 2,500 mm: `falloff = 10000 - floor_div(8000 × 1500, 3500) = 6572`, base 31,545, D1 = 33,437, split pierce 20,063 / blunt 10,031 / fire 3,343; after AR 10, **30,717 mHP**. An allied Erratum at 3,500 mm in medium armor (AR 20): falloff 4286, base 20,572, D1 21,806, `FF_AREA_BP` 10000, after armor **17,084 mHP**; in `mode.ff_off` (5000) it takes 7,104.

**E9 — Elevation and flanking (pistol, 30 m, 3 m above, from the flank).** Falloff 6250: base 5,375; D1 = 5,697; elevation × 11000 → 6,266; flanking × 11500 → 7,205; AR 10 with `pen_bp` 1000 gives effAR `mul_bp(7, 9000) = 6`, DR 1,723. **Total 5,964 mHP.**

---

## 11. Armor

### 11.1 Archetypes

Twenty archetypes `arch.armor.<slot>_<class>` cover slots `head`, `body`, `hands`, `legs`, `feet` and classes `cloth`, `light`, `medium`, `heavy` (class values 0, 1, 2, 3). AR follows Canon §10.4 (`base_ar × T(ilvl) × rarity × quality × enhancement × mat_ar_bp`, one `mul_bp` each) per covered part.

**Table 11.1a — base AR per covered part at ilvl 1.** A full set of one class gives the same AR on every humanoid part; medium gives the canon anchor of 20 (Canon §17.4).

| Slot | Parts covered | Cloth | Light | Medium | Heavy |
|---|---|---|---|---|---|
| head | `bp.head` | 5 | 12 | 20 | 30 |
| body | `bp.torso`; each arm | 5; 2 | 12; 5 | 20; 8 | 30; 12 |
| hands | each arm | 3 | 7 | 12 | 18 |
| legs | each leg | 4 | 8 | 14 | 21 |
| feet | each leg | 1 | 4 | 6 | 9 |
| Full set, every part | | 5 | 12 | 20 | 30 |

**Table 11.1b — per piece: poise / mass (g) / noise (dB).**

| Slot | Cloth | Light | Medium | Heavy |
|---|---|---|---|---|
| head | 0 / 300 / 0 | 1 / 800 / 0 | 2 / 1800 / 1 | 4 / 3200 / 1 |
| body | 0 / 1500 / 0 | 3 / 4000 / 1 | 6 / 8000 / 2 | 12 / 14000 / 4 |
| hands | 0 / 200 / 0 | 1 / 500 / 0 | 2 / 1000 / 0 | 4 / 1800 / 1 |
| legs | 0 / 800 / 0 | 2 / 2200 / 1 | 4 / 4200 / 2 | 8 / 7000 / 3 |
| feet | 0 / 400 / 0 | 1 / 1000 / 1 | 2 / 1800 / 1 | 4 / 3000 / 2 |
| Full set | 0 / 3200 / 0 | 8 / 8500 / 3 | 16 / 16800 / 6 | 32 / 29000 / 11 |

**Table 11.1c — resistances per piece** (Bp, added linearly before the cap, Canon §10.2): cloth arcane +300, psychic +300; light frost +200, toxin +200; medium fire +200, acid +200; heavy fire +300, frost +300, shock −300. A full heavy set gives fire and frost +1500 and shock −1500.

```json
{ "id": "arch.armor.body_medium", "category": "cat.armor", "slot": "slot.body", "weight_class": 2,
  "base_ar": { "bp.torso": 20, "bp.arm_l": 8, "bp.arm_r": 8 },
  "poise": 6, "mass_g": 8000, "noise_db": 2,
  "resist": { "dmg.fire": 200, "dmg.acid": 200 } }
```

Parts added by kin or mutation (Canon §9) take `floor_div(AR(bp.torso), 2)`. Example: a medium body piece at ilvl 10, Rare, Good, +1, iron gives torso AR `20 → 32 → 35 → 38 → 40 → 40`.

### 11.2 Armor poise

```
armor_poise_mp = Σ pieces mul_bp(mul_bp(mul_bp(mul_bp(poise × 1000, S(ilvl)), quality_bp), 10000 + 50 × skill.armor), potency)
```

It feeds Canon §6.4 row 18. A full Normal medium set at ilvl 1 with Armor 20 gives 17,600 mp, so the all-40 Erratum's max poise is 57,600 mp.

### 11.3 Movement penalties

Armor never multiplies move speed (Canon §4.2). It sets the sprint and swim stance base speeds (§1.6) through a weight score:

```
score     = Σ slot_weight × class_value     // slot weights: head 1, body 4, hands 1, legs 2, feet 2; range 0..=30
score_eff = floor_div(score × (10000 - 50 × skill.armor), 10000)
sprint    = 6500 - 20 × score_eff  mm/s;   swim = 2400 - 30 × score_eff  mm/s
```

At Armor 20 a full heavy set gives `score_eff = 27`, sprint 5,960 and swim 1,590 mm/s; full medium gives 6,140 and 1,860; at Armor 100 full heavy gives 6,200 and 1,950. Mass also counts toward encumbrance (Canon §11.1).

### 11.4 Armor skill hooks

- Armor poise × `10000 + 50 × skill.armor` (§11.2) and weight score × `10000 - 50 × skill.armor` (§11.3).
- Armor noise: each piece's `noise_db` minus `floor_div(skill.armor, 25)`, minimum 0.
- Skill XP (doc 04 table): each hit taken grants `min(1000, floor_div(1000 × mitigated_mhp, max_hp_mhp))` milli-XP to `skill.armor`, where `mitigated_mhp` is the step-5 reduction. Only hostile-sourced hits count.

---

## 12. Shields, blocking, parry, poise and stagger

### 12.1 Off-hand archetypes

| Archetype | Base AR | Stability (Bp) | Arc (Bam) | Raise | Parry ticks | Mass (g) | Bash impact / `base_mhp` | Blocks area hits |
|---|---|---|---|---|---|---|---|---|
| `arch.offhand.shield_buckler` | 10 | +1000 | 8192 | 4 | 9 | 1500 | 15 / 4000 | no |
| `arch.offhand.shield_heater` | 25 | +2500 | 10923 | 6 | 6 | 4000 | 25 / 5000 | no |
| `arch.offhand.shield_tower` | 45 | +4000 | 13653 | 10 | 0 (cannot parry) | 9000 | 35 / 6000 | yes |
| `arch.offhand.lantern` | — | — | — | — | — | 1200 | — | — |
| `arch.offhand.focus` | — | — | — | — | — | 600 | — | — |

- Shield AR uses the armor chain with `T(ilvl)` (Canon §13.8 "shield block values").
- The lantern emits `light_block` 13 at its holder (doc 03 propagation) and cannot block. `arch.offhand.focus` is the casting focus whose casting behavior doc 06 owns; it cannot block.
- **Shield bash** (`move.shield_bash`, `input.heavy` while guarding with a shield): W 5 / A 3 / R 16, 10 stamina, blunt 10000, reach 1000, arc 4551, skill `skill.shield`, taunt power 40 (§19.2). Heater and tower bashes carry `counter.staggerer`.

### 12.2 Guard outcome

The guard is effective in `Guarding` (§4.8). For a hit with incoming vector `i` (from the target toward the attack origin, the projectile's reverse velocity, or the explosion center):

```
bearing = atan2_bam(-i.x, -i.z)
in_arc  = ang_diff(target.yaw, bearing) <= guard_arc
blockable = melee | projectile | hitscan | (area && tower shield);  never DoT, surfaces, falls, unblockable NPC moves
outcome = Parried  if in_arc && blockable && PARRYABLE && k < parry_ticks
        = Blocked  if in_arc && blockable
        = None     otherwise
stability_bp = clamp(5000 + d(BRN) × 50 + shield_stab, 0, 10000)       // Canon §6.4 row 19
shield_stab  = shield stability + 30 × skill.shield   (shields) | weapon block stability (Table 3.2c)
stability_bp = mul_bp(stability_bp, 5000) if the attack is a chain weapon
AR used at step 5 = AR(part) + AR(shield)  (weapon guards add 0)
```

### 12.3 Step 6b distribution

Canon §10.7 converts `to_stamina = mul_bp(total, stability_bp)` into stamina loss `floor_div(to_stamina × 10000, T(d_current))`. Doc 05 distributes the remaining HP damage over types: `new_total = total - to_stamina`, `d[t] = floor_div(d[t] × new_total, total)` per type, remainder to the largest-share type. A `GUARD_BREAK` hit multiplies the stamina loss by 20000. Blocked hits deal no poise damage and half buildup (Canon §11.2). When the blocker's stamina reaches 0 it is guard-broken: `st.staggered` (30 ticks) and no new raise for 60 ticks. NPC guards (doc 13 templates supply stamina, `block_stability_bp` and arc) follow the same rules with `T(d_current)`.

### 12.4 Poise damage (step 8)

```
pd = impact_mp                                     // impact × 1000 × §10.1 factors
pd = mul_bp(pd, S(src_lvl))                        // Canon §10.5 step 8; enemies S(d)
pd = mul_bp(pd, 10000 + mul_bp(5000, share_blunt)) // dmg.blunt "poise damage × 15000" (Canon §10.1) pro rata
pd = mul_bp(pd, 30000)          if counter.staggerer
pd = mul_bp(pd, 20000)          if MOUNTED_CHARGE
pd = mul_bp(pd, 5000)           if the target is in two-handed heavy hyperarmor (§4.2)
pd = mul_bp(pd, max(5500, poise_taken_bp))         // affixes (doc 11)
pd = 0                          if blocked or parried
```

### 12.5 Stagger and knockdown

```
poise -= pd
if poise <= 0:
    immune = target has elite.unstoppable or poise_immune (boss template flag)
    if immune && !counter.staggerer: poise = max_poise; return
    if pd >= 2 × max_poise && rank ∉ {boss, editor, hunter} && !immune:
        apply st.knocked_down directly (Canon §11.4: 60 ticks, nerve-scaled, × rank control)
    else:
        apply st.staggered directly (30 ticks × rank control)
    poise = max_poise                               // refill; overflow discarded
```

Max poise is Canon §6.4 row 18 for Errata and NPCs (NPC armor poise comes from the template), × 12000 under `st.fortified`. **Reset**: Errata poise refills fully 480 ticks after the last poise damage; NPC poise refills after `clamp(360 + floor_div(9 × (P - 80), 2), 240, 1200)` ticks, `P = floor_div(max_poise_mp, 1000)` (240 ticks at P 40, 900 at P 200).

Examples against a REF NPC (poise 40,000 mp): a warhammer light step 1 deals `mul_bp(34000, 14250) = 48,450` and staggers. A full warhammer heavy deals `mul_bp(mul_bp(34000, 28000), 14250) = 135,660 >= 80,000` and knocks down.

---

## 13. Default status power

Each hit carries a status power per type it holds (Canon §11.2 consumes it):

```
p_t = floor_div(POWER10[t] × base_mhp_status × share[t], 10000 × 10000)    // base_mhp after §10.1 factors 1, 2, 8
p_t = mul_bp(p_t, archetype status_bp for that status)                      // Tables 3.2c, 5.2d; default 10000
then Canon §11.2: × S(src_lvl) × potency × edge (native) × row 41 × (10000 - r) × 5000 if blocked
```

| Type | Status | `POWER10` | Type | Status | `POWER10` |
|---|---|---|---|---|---|
| slash | `st.bleeding` | 20 | sonic | `st.deafened` | 20 |
| pierce | `st.exposed` | 12 | toxin | `st.poisoned` | 25 |
| blunt | `st.stunned` | 12 | radiant | `st.blinded` | 15 |
| fire | `st.burning` | 25 | umbral | `st.withering` | 20 |
| frost | `st.frozen` | 15 | psychic | `st.dread` | 20 |
| shock | `st.shocked` | 20 | arcane | `st.unraveled` | 20 |
| acid | `st.corroded` | 20 | true | none | 0 |
| radiation | `st.irradiated` | 15 | | | |

Because power is proportional to hit size, fire rate never changes buildup per second: at REF DPS every archetype applies the same power per second per 100% share. Example: a long blade step-1 light hit gives bleeding power `floor_div(20 × 14700 × 8000, 10^8) = 23`, so the fifth hit fills a threshold of 100. Every `dmg.frost` hit also applies `st.chilled` directly (Canon §10.1). Explicit status powers (flash grenade, shield bash taunt) replace the default for their status.

Doc-05 status behavior beyond Canon §11.1: `st.disarmed` puts the active weapon into `Disarmed` (§4.8) without dropping it; `st.slowed` and `st.chilled` multiply stance base speed through the status step; `st.corroded` multiplies AR per Canon §10.3; `st.exposed` stacks at step 2.11.

---

## 14. Threat, friendly fire, downed and death

### 14.1 Threat

Each NPC keeps `threat: BTreeMap<EntityId, i64>`. Doc 13 selects targets from it.

| Event | Threat added to the NPC's entry for the acting Erratum or minion owner |
|---|---|
| HP and ward damage dealt to that NPC | `mul_bp(mul_bp(dmg_mhp, row39_bp), class_bp)`; class: melee 12000, ranged 10000, area 8000, DoT 5000 |
| Damage blocked from that NPC's hits | `mul_bp(mul_bp(to_stamina_mhp, row39_bp), 5000)` |
| Healing or ward granted to an ally | `mul_bp(amount_mhp, 5000)` to every NPC whose table already holds that ally |
| `st.taunted` applied | `max(table) + 1` for the taunter |

Every AI think (6 ticks) each entry loses `floor_div(entry × 30, 10000)`. Target choice (doc 13): switch when a challenger's threat exceeds the current target's × 11000, comparing entries within 3000 mm × 13000 and downed Errata × 2500; ghosts are excluded. A minion's threat is credited to the minion. Example: 10,000 mHP of melee damage at MEN 50 adds `mul_bp(mul_bp(10000, 11000), 12000) = 13,200`.

### 14.2 Friendly fire (Canon §17.5)

- Step 2.15 applies when attacker and target are both `fac.errata`, minions included. `FF_DIRECT_BP` covers melee, projectiles, hitscan, quick melee and bashes; `FF_AREA_BP` covers explosions, shockwaves, fire spread, surfaces, falling blocks and shock chains. Self-hits from one's own explosions are area friendly fire.
- Allies are valid melee and projectile targets and consume cleave and pierce. Friendly hits never add threat, never trigger Shock of the New, never parry or riposte, and grant no combat skill XP.
- `st.charmed` on a player uses Canon §11.8.

### 14.3 Downed and death hooks

- Canon §10.8 step 2 applies `st.downed`. While downed, an Erratum crawls at 600 mm/s base, cannot attack, block or use items, and keeps its capsules.
- Damage to a downed Erratum runs the full pipeline; each resolved hit shortens the remaining bleed-out by `ceil_div(total_mhp × 1800, max_hp_mhp)` ticks (17,000 mHP at Max HP 85,000 removes 360 ticks). At 0 remaining ticks Canon §10.8 step 3 runs.
- `ev.entity_killed` drops stuck thrown weapons and arrows as ground items (`floor_div(n, 2)` of `n` arrows survive; thrown weapons always survive) and releases grapples and taunts the entity held.

---

## 15. Champion counters, elite aspects and ward generators

### 15.1 Counter types

`counter.*` is a bitmask on a hit (`counters`), bit order as listed.

| Bit | ID | Effect on a hit carrying it |
|---|---|---|
| 0 | `counter.ward_breaker` | step 7 uses `ward_mult_eff[t] = mul_bp(mul_bp(ward_mult[t], profile[t]), 30000)` for every ward pool of the target (`profile` is 10000 except a generator's §15.4 profile); if this hit breaks a pool, apply `st.stunned` directly (Canon §11.4 duration × rank control) |
| 1 | `counter.disruptor` | apply `st.unraveled` and `st.silenced` directly for exactly 180 ticks each (no Canon §11.4 scaling); the target's regeneration (natural, `elite.surging`, template regen) is 0 while either lasts |
| 2 | `counter.staggerer` | poise damage × 30000 (§12.4); can stagger targets immune to poise stagger |
| 3 | `counter.sunder` | apply one stack of `st.corroded` directly, bypassing corrosion immunity; at 5 stacks a target with `elite.armored` loses that modifier for 300 ticks |
| 4 | `counter.anchor` | apply `st.rooted` directly for 120 ticks; a target with `elite.phasing` cannot phase for 300 ticks |

### 15.2 Elite aspects (Canon §17.3a) and two elite modifiers

| Aspect or modifier | Exact behavior | Counter |
|---|---|---|
| `elite.warded` (champion aspect) | NPC ward pool = `mul_bp(max_hp_mhp, 5000)`; refills fully 180 ticks after the champion last took damage; depletes per Canon §10.5 step 7 | `counter.ward_breaker` |
| `elite.surging` (champion aspect) | attack and cast speed × 13000; regenerates 200 Bp of Max HP per second (accumulator `k = 60`) unless it has `st.unraveled` or `st.shocked` | `counter.disruptor` |
| `elite.unstoppable` (champion aspect) | immune to `st.staggered` and `st.knocked_down` from poise | `counter.staggerer` |
| `elite.armored` (doc 13 elite modifier, requested) | every part's AR × 25000; immune to `st.corroded` buildup | `counter.sunder` |
| `elite.phasing` (doc 13 elite modifier, requested) | every 240 ticks becomes intangible for 60 ticks (no capsule tests) and blinks up to 6000 mm | `counter.anchor` |

### 15.3 Delivery channels

| Channel | Counters | Rule |
|---|---|---|
| Counter charges `ammo.charge_*` (dependency none) | each charge one type | Use action `move.imbue`, `action_ticks(45)`; imbues the active weapon: ranged for the next `max(6, magazine)` shots (beams 60 pulses), melee for the next 12 hits; a new imbue replaces the old one |
| `arch.ranged.grenade_counter` (dependency none) | one type per item template | every area hit carries it |
| Archetype innates | ward_breaker: sniper and crossbow precision hits, charge rifle full charge; disruptor: beam streak ≥ 11, whip full heavy; staggerer: launcher direct hit, warhammer and great axe full heavy, warhammer jump attack, heater and tower bash | always on |
| Perks and affixes (doc 11) | any | the item template sets `innate_counter` bits |

Doc 11 stocks at least one counter charge of every type and one `grenade_counter` of every type in every genre's shop and loot pool (Canon §17.3a requirement).

### 15.4 Ward generators (`slot.ward`, `cat.ward_gen`)

```
capacity_mhp = cap_base_mhp → × T(ilvl) → × rarity → × quality → × (10000 + 600 × eff_enh) → × potency
recharge: starts 180 ticks after the bearer last took damage (any ev.damage_dealt with the bearer as target,
          DoT included, ward_lost or HP > 0); restores mul_bp(capacity, 2500) per second (accumulator k = 60)
```

Step 7 for the generator pool uses `ward_mult_eff[t] = mul_bp(ward_mult[t], profile[t])`; the `st.shielded` pool always uses the canonical `ward_mult`.

| Archetype | `cap_base` (HP) | Depletion profile (Bp) | On break | Mass (g) | Hum (dB) |
|---|---|---|---|---|---|
| `arch.ward.aegis` | 20 | all 10000 | none | 300 | 0 |
| `arch.ward.kinetic` | 24 | slash, pierce, blunt 8000; shock 15000; others 10000 | 2000 mm pulse: impact 20, no damage | 1200 | 1 |
| `arch.ward.reflector` | 16 | arcane 7500; others 10000 | while ward > 0, 1000 Bp of absorbed damage returns to the attacker through the thorns entry (Canon §10.6) | 800 | 0 |
| `arch.ward.bulwark` | 36 | all 12500 | bearer gains `st.slowed` directly | 3000 | 2 |

```json
{ "id": "arch.ward.kinetic", "category": "cat.ward_gen", "slot": "slot.ward", "cap_base_mhp": 24000,
  "profile": { "dmg.slash": 8000, "dmg.pierce": 8000, "dmg.blunt": 8000, "dmg.shock": 15000 },
  "on_break_radius_mm": 2000, "on_break_impact": 20, "mass_g": 1200, "hum_db": 1 }
```

Examples: an aegis at ilvl 10 holds `mul_bp(20000, 16190) = 32,380` mHP and recharges 8,095 mHP per second. A kinetic ward of 10,000 mHP hit by 8,000 mHP of shock uses `ward_mult_eff = mul_bp(15000, 15000) = 22500`: `w = 18,000 > 10,000`, so HP takes `8000 - floor_div(10^8, 22500) = 3,556` and the ward breaks. With the aegis profile the same hit leaves 1,334, matching Canon §21 test 41.

---

## 16. Contradiction failures: jams and clearing

### 16.1 Scope

Only the base-function component rolls (Canon §5.5). Its dependency comes from the skin, so a longbow (dependency none) never fails while a railgun skin of the same archetype can. Potency always multiplies damage, buildup, ward, AR, poise and mount speed. A **Disconnected** weapon (C ≥ 6) cannot fire or swing; `input.attack` then performs `arch.melee.unarmed` attacks.

### 16.2 Cadence

```rust
fn failure_due(w: &mut WeaponState, mode: Mode, t: u64, trigger_held_prev: bool) -> bool {
    if w.failure_bp == 0 { return false; }
    match mode {
        Semi | Bolt | Charge | Throw | Burst | MeleeSwing | Summon | GuardRaise => true, // once per activation
        Auto | Beam => {
            if !trigger_held_prev || t - w.last_fail_roll >= 30 { w.last_fail_roll = t; true } else { false }
        }
    }
}
// due → keyed_rng("rng.combat", folio, wielder, t, seq).roll_bp(failure_bp), the first draw at that seq (§1.4)
```

A burst rolls on its first shot only. A melee swing rolls on its Windup→Active transition. Example: an SMG held for 90 ticks fires on ticks 0, 4, …, 88 and rolls on ticks 0, 32 and 64; at failure 1000 Bp the chance of no jam is `0.9³ = 72.9%`.

### 16.3 Failure outcomes

| Base function | Failure | Recovery |
|---|---|---|
| Firearm, crossbow (techne skin), launcher | **Jam**: no shot, no ammunition spent; state `Jammed`; `emit_noise(40 dB)` | `input.reload` → `Clearing`, `action_ticks(45)`; ejects one round from the magazine |
| Beam | **Stall**: no pulses for 60 ticks | automatic |
| Charge rifle | **Misfire**: the draw is lost; the round stays loaded | draw again |
| Melee weapon | **Falter**: the swing produces no hits; recovery +12 ticks; stamina spent | automatic |
| Throwable | **Dud**: lands and does not detonate; becomes a ground item | pick it up and throw again |
| Mount | **Stall**: speed 0 for 60 ticks (§17.4) | automatic |
| Ward generator | **Flicker**: the recharge start is delayed another 180 ticks | automatic |
| Shield with a dependent skin | **Slip**: that raise produces no guard | release and raise again |

---

## 17. Mounts (Canon §17.7)

### 17.1 Archetypes

| Field | `arch.mount.runner` | `arch.mount.hover` | `arch.mount.glider` |
|---|---|---|---|
| Body plan | `body.quadruped` | `body.construct` | `body.avian` |
| Template HP (× `H(d_current)` at summon) | 70 | 55 | 45 |
| Natural AR per part (× `T(token ilvl)`) | 10 | 15 | 5 |
| Poise | 60 | 80 | 40 |
| Speeds (mm/s) | walk 6000; gallop 12000 | 14000 | horizontal 9000; descent 2000; on ground 3000 |
| Acceleration (mm/s²) | 20000 | 15000 | 10000 |
| Turn rate (Bam per tick) | 546 walking, 273 galloping | 364 | 182 |
| Mount stamina | 100; gallop drains 20/s; regenerates 25/s after 1 s | none | 50; a flap costs 40 and adds 4000 mm of height once per airborne period |
| Jump | 1500 mm | none; step-up 1000 mm | flap |
| Saddle height (mm) | 1500 | 900 | 1300 |
| Seats | 1 | 2; the passenger can shoot | 1 |
| Liquids | swims at 2400 | hovers across | glides over |
| Rider weapons | all | driver one-handed only; passenger all | one-handed only |
| Trample at ≥ 8000 mm/s | blunt 6000 mHP, impact 40 | blunt 5000 mHP, impact 40 | none |
| Mass (g) | 450000 | 300000 | 120000 |

```json
{ "id": "arch.mount.runner", "category": "cat.mount", "body": "body.quadruped", "template_hp": 70, "natural_ar": 10,
  "poise": 60, "walk_mm_s": 6000, "gallop_mm_s": 12000, "accel_mm_s2": 20000, "turn_bam": 546, "turn_gallop_bam": 273,
  "stamina": 100, "gallop_drain": 20, "stamina_regen": 25, "jump_mm": 1500, "saddle_mm": 1500, "seats": 1,
  "trample_mhp": 6000, "trample_impact": 40, "mass_g": 450000 }
```

### 17.2 Summoning and dismounting

- `input.mount` with a token in `slot.mount` channels 30 ticks (`move.summon` rolls Contradiction at `seq` 0). It needs 2600 mm of free vertical clearance in a 600 mm-radius column at the summon point and a room without `room.no_mount`. The mount appears beside the rider, the rider mounts, `st.mounted` applies and `ev.mounted` fires.
- `input.mount` while riding dismounts in 15 ticks (`ev.dismounted`). A dismounted mount stays where it is; summoning again moves it.
- When the mount reaches 0 HP, the rider gains `st.knocked_down` and `ev.dismounted` fires. The token is inert until the next `ev.folio_entered`, when the mount returns at full HP.

### 17.3 Mounted combat

- Rider eye height is `saddle_mm + 900`. Riders' melee reach grows by 400 mm.
- `st.mounted` multiplies spread by 13000 at the status step (§6.3); a mounted Erratum cannot be `st.hidden`, cannot place blocks and cannot lean or dodge.
- **Mounted charge** (`move.mounted_charge`): any melee hit landed while the mount's horizontal speed is at least 8000 mm/s carries `MOUNTED_CHARGE`: attack factor 12500 (§10.1) and poise × 20000 (§12.4). A long blade step-1 hit rises from 14,700 to 18,375 base.
- **Couched lance**: holding `input.heavy` with a spear or halberd at ≥ 8000 mm/s keeps a thrust active (up to 180 ticks, no extra stamina) that hits the first entity at full `heavy_bp` with `MOUNTED_CHARGE`, then recovers.
- **Trample** (`move.trample`): the mount's capsule touching a non-flying entity of lower mass at ≥ 8000 mm/s makes a melee-kind hit (source the mount, owner the rider, `src_lvl` the token's ilvl) once per target per 60 ticks. Friendly fire applies.
- Rider and mount have separate capsules. Single-target hits damage whichever capsule they find; area hits test both. NPC threat earned against the mount is credited to the rider.

### 17.4 Contradiction and handling

The mount's base function takes its dependency from the skin (beasts none, vehicles techne, carpets arcana, divine steeds mythos, clouds verve). Potency multiplies every mount speed and trample damage. A stall roll (`seq` 2) runs at summon and every 300 ticks while the mount moves. `skill.handling` multiplies gallop drain by `10000 - 50 × handling` and turn rate by `10000 + 30 × handling`.

---

## 18. Combat feel (presentation only)

Everything here is derived by the renderer from simulation events and never feeds back (Canon §1.2).

| Parameter | Rule |
|---|---|
| Hitstop | The attacker's viewmodel and the target's mesh freeze for N ticks, then play the next 2N ticks at 150% speed to rejoin simulation time. N: light hit 2, heavy 4, full heavy 6, +1 on crit, +2 on kill, parry 6 (both sides), clang 4, blocked 3; ranged 0 except sniper, crossbow and launcher direct hits 2. |
| Camera kick | Visual pitch kick of `mul_bp(v_bam, 15000)` over 4 ticks, eased back over the recoil delay, on top of the simulated recoil; explosions shake with amplitude `floor_div(91 × falloff_bp, 10000)` Bam within twice the radius. |
| Hit markers | For hits owned by the local player: white normal, yellow precision, orange crit, cyan ward, gray blocked, a dented marker when step-5 DR ≥ 5000, a red cross for 20 ticks on kill. Hitscan markers are predicted locally and replaced or removed by the host's `ev.damage_dealt` within 30 ticks. |
| Damage numbers | `floor_div(total, 1000)`, or "<1" for `0 < total < 1000`; hits on one target within 6 ticks merge; color and icon of the largest-share type (Canon §10.1); crits at 125% size; 45-tick float. Doc 17 owns the accessibility toggles. |
| Audio | Doc 18 maps `ev.damage_dealt` flags, clang, parry, jam and ward break to cues. |

---

## 19. Severing, taunt and grapple

### 19.1 Severing hits (Canon §9.1)

After step 8, if the hit part is severable and its condition pool is at most 0, the part is severed when the hit's largest share is `dmg.slash` or it carries `SEVER`. NPC heads also need the owner's governing skill ≥ 75. `SEVER` comes from great blade, great axe and halberd heavies and from launcher blasts within their inner radius. The part becomes a `cat.body_part` item at its capsule's midpoint and `ev.part_severed` fires; doc 13 removes template attacks bound to the part. Example: a REF NPC's arm pool is `mul_bp(60000, 1250) = 7,500` mHP, so any slash-dominant hit dealing at least 7,500 mHP to an arm with 0 condition left after it severs that arm.

### 19.2 `st.taunted`

- Buildup in the mind group. Sources: shield bash (power 40) and `move.taunt` (`input.alt` + `input.reload` while guarding with a shield): 30 ticks, 15 stamina, 600-tick cooldown, taunt power 60 to every hostile within 10000 mm with line of sight. Power scales with `S(shield ilvl)` and the Canon §11.2 chain.
- Effect on NPCs: `ai_target` is forced to the taunter while active (doc 13), threat jumps per §14.1, and damage dealt to anyone else is × 7500 at step 2.12. Player-controlled bearers follow Canon §11.8.

### 19.3 `st.grappled`

- Applied directly (Canon §11.4, nerve scaling, base 3 s, × rank control) by `move.grapple` (full heavy of a fist weapon or unarmed), by a full chain heavy (entangle at reach), and by NPC grabs (doc 13). It requires a target that is not boss, editor or hunter, is not already grappled, and whose mass (body plus carried) is at most `mul_bp(grappler mass, 12500)`.
- The target cannot move and can use only one-handed actions; a two-handed weapon enters `Staggered` until release. The grappler moves at stance base 2000 mm/s; a fist or unarmed grappler's light attacks hit the target's torso automatically.
- It ends at expiry, when the grappler has taken at least 10% of its Max HP since the grapple began, when the grappler is staggered, knocked down or downed, or when the two are more than `reach + 500` mm apart.
- A grappled player presses `input.interact` to struggle: each press removes 10 ticks of remaining duration, at most once per 6 ticks. Input is never synthesized (Canon §11.8).

---

## Exports

All IDs below sit in namespaces Canon §19 assigns to doc 05.

| Namespace | IDs |
|---|---|
| `arch.melee.` | `dagger`, `blade_short`, `blade_long`, `blade_great`, `axe`, `axe_great`, `mace`, `warhammer`, `spear`, `halberd`, `quarterstaff`, `fist`, `whip`, `chain`, `unarmed` |
| `arch.ranged.` | `pistol`, `pistol_heavy`, `smg`, `rifle_auto`, `rifle_marksman`, `shotgun`, `sniper`, `launcher`, `beam`, `charge_rifle`, `crossbow`, `thrown`, `grenade_frag`, `grenade_incendiary`, `grenade_concussion`, `grenade_flash`, `grenade_smoke`, `grenade_counter` |
| `arch.armor.` | `head_cloth`, `head_light`, `head_medium`, `head_heavy`, `body_cloth`, `body_light`, `body_medium`, `body_heavy`, `hands_cloth`, `hands_light`, `hands_medium`, `hands_heavy`, `legs_cloth`, `legs_light`, `legs_medium`, `legs_heavy`, `feet_cloth`, `feet_light`, `feet_medium`, `feet_heavy` |
| `arch.offhand.` | `shield_buckler`, `shield_heater`, `shield_tower`, `lantern`, `focus` (casting behavior owned by doc 06) |
| `arch.ward.` | `aegis`, `kinetic`, `reflector`, `bulwark` |
| `arch.mount.` | `runner`, `hover`, `glider` (names from Canon §17.7; mechanics defined here) |
| `ammo.` | `light`, `heavy`, `rifle`, `shell`, `cell`, `ordnance`, `draw`, `charge_breaker`, `charge_disruptor`, `charge_staggerer`, `charge_sunder`, `charge_anchor` |
| `move.` | `light`, `heavy`, `sprint_attack`, `jump_attack`, `riposte`, `shove`, `guard`, `shield_bash`, `quick_melee`, `grapple`, `taunt`, `mounted_charge`, `trample`, `summon`, `imbue`, `ads`, `hold_breath`, `fire_semi`, `fire_auto`, `fire_burst`, `fire_bolt`, `fire_charge`, `fire_beam`, `throw`, `cook`, `reload`, `clear_jam`, `lean_l`, `lean_r`, `shape_thrust`, `shape_rl`, `shape_lr`, `shape_overhead` |
| `counter.` | `ward_breaker`, `disruptor`, `staggerer` (Canon §17.3a), `sunder`, `anchor` (new) |

Constants exported: `LAGCOMP_MAX_TICKS = 16`; `POWER10` table (§13); recoil string reset 30 ticks; Errata poise reset 480 ticks; parry re-raise lockout 20 ticks; riposte window 30 ticks.

## Interface assumptions

| Doc | Assumed to provide |
|---|---|
| 02 | `div128`, `div256` (two-limb `i256`), `isqrt_u64`, and the `mm(Fx)` conversion in the fixed-point module |
| 03 | the combat phase slot after movement and collision and before the status-timer step; `solid`, `opaque`, `material_at`; `damage_block(b, mhp, shares)` with breakage and `rng.physics` debris; `emit_noise(pos, db)`; fire ignition; event emission in creation order |
| 04 | stamina regeneration after 1 s without spending; `st.exhausted` blocks heavies and dodges; the XP table accepts §11.4's Armor XP; mastery perks write the hook fields of §11.4 and §12.2; Calling kits give two pickups per ammunition family used |
| 06 | `arch.offhand.focus` casting; the `st.shielded` pool; one-handed casting while grappled |
| 07 | per material `dmg_bp`, `ar_bp`, density, `pen_cost`, `ricochet`, `flammable`; instance `mass_g`; durability per throw and swing |
| 08 | `gas.smoke` emission API; alchemical flasks reusing `arch.ranged.grenade_*` with liquid payloads |
| 09 | `sneak_eligible(att, tgt, t)`; `sneak_mult(att, tier)` with tier 1 = 30000, tier 2 = 22500, tier 3 = 15000 (E3 uses tier 1); `IGNORE_ARMOR` at Sneak 100; noise propagation |
| 11 | item templates and overridable fields (fire mode, `sig`, `innate_counter`, `max_ricochets`); ammunition drops using §8.3 need weights at one pickup per 4 standard kills; counter charges and counter grenades of every type in every genre's shops and loot |
| 12 | `room.no_mount`; local gravity per genre and bleed zone |
| 13 | NPC templates: `template_dmg_mhp`, impact, natural AR per part, BRN, TGH, stamina, `block_stability_bp`, guard arc, unparryable and unblockable move flags, `poise_immune`, weapon-like attack geometry; `ai_target(e)`; forced targeting under `st.taunted`; threat-based selection with the §14.1 hysteresis; the requested modifiers `elite.armored` and `elite.phasing` |
| 14 | `difficulty_bp(att, tgt)`, 10000 at the default difficulty |
| 16 | transform and pose history of at least 32 ticks; prediction of recoil, sway and spread; `st.downed` crawl and revive flow accepting §14.3 bleed-out shortening |
| 17 | the input IDs of §1.7; hit markers and damage numbers per §18; accessibility toggles |
| 18 | skin names and presentation for every archetype and event here |

## Canon gaps

1. **Attack factors have no pipeline slot.** Canon §10.5 lists no step for combo, heavy charge, sprint, jump, riposte, charge, range falloff, penetration keep, cleave, area falloff or the Canon §17.7 mounted-charge × 12500. Assumed: they define the hit's `base_mhp` before Canon §10.4 (§10.1).
2. **`st.taunted` at step 2.12.** Canon step 2.12 lists berserk, empowered and weakened. Assumed: the Canon §11.1 effect "damage dealt to others × 7500" applies at step 2.12 in ascending status ID order.
3. **Shield AR on blocked hits.** Assumed: step 5 uses `AR_part + AR_shield`, reading Canon §13.8's "shield block values" scaling with `T` as shield AR.
4. **Step 6b per-type split.** Canon §10.7 reduces only the total. Assumed: proportional per-type scaling with the remainder to the largest-share type (§12.3).
5. **Guard-break extensions.** `GUARD_BREAK` multiplies Canon §10.7 stamina loss by 20000; shove removes a flat 35 stamina.
6. **Per-generator ward depletion.** Assumed: step 7 uses `ward_mult_eff = mul_bp(ward_mult, profile)` for the generator pool only; `counter.ward_breaker` multiplies that by 30000. With profile 10000 the result equals Canon §21 test 41.
7. **Ward capacity chain.** Canon §13.2 gives `mul_bp(cap_base_mhp, T(ilvl))`. Assumed: rarity, quality, enhancement and potency follow, as for armor.
8. **Counter durations.** Assumed: `counter.disruptor`'s 180 ticks are exact (no Canon §11.4 scaling); the ward-break stun uses Canon §11.4.
9. **Armored and Phasing.** Canon §17.3a fixes three champion aspects. Champions keep exactly those three. Doc 05 adds `counter.sunder` and `counter.anchor` and requests `elite.armored` and `elite.phasing` from doc 13 as non-champion elite modifiers.
10. **Stance base speeds.** Canon §4.2 defines base speeds but no combat stances. Assumed: §1.6 stance values and the armor sprint and swim values replace the base speed before row 11; armor never multiplies speed.
11. **Keyed `seq` allocation.** Canon §3.3 describes `seq` as a per-actor counter. Assumed: fixed slots 0, 1, 2, 3 and `16 + i` (§1.4), with projectiles as their own actors.
12. **Blunt poise factor and poise reset.** Canon §10.1 says blunt poise damage × 15000 without saying how mixed shares apply. Assumed: `10000 + share_blunt / 2`. Poise reset timings (480 ticks for Errata, the NPC formula from R5 §7.3) are not in the canon.
13. **Mount scaling depth.** Assumed: mount HP uses `H(d_current)` at summon and mount AR uses `T(token ilvl)`.
14. **Throw speed.** Canon row 20 is a range multiplier; speed uses its square root (§9.1).
15. **Default status powers** per damage type are not in the canon (§13).
16. **Downed details.** Canon §11.1 gives "crawl only" and a 30 s bleed-out. Assumed: crawl base 600 mm/s and damage shortens the bleed-out (§14.3); doc 16 confirms.
17. **Canon §5.9** (genre identity, manufacturers, `spawn_block_max`) is referenced by the canon but absent. Doc 05 reads nothing from it.
18. **Sneak and power.** R2 recommends Oblivion's `max(sneak, power)`. The canon chain multiplies sneak with everything; doc 05 keeps the canon and bounds heavy factors at ×2.8.

## Acceptance tests

1. **Melee calibration.** Simulating each melee archetype's light-combo cycle against the REF target (expected crit 500 Bp × 15000) reproduces Table 3.2a's REF DPS within ±0.2 HP/s, and the full-heavy cycle reproduces §3.2's heavy DPS list within ±0.2.
2. **Ranged calibration.** Each ranged archetype's full magazine-plus-reload cycle reproduces Table 5.2a's REF DPS within ±0.2, and the AR 40 list in §5.2 within ±0.2.
3. **E1.** Long blade finisher with a successful crit against AR 10 deals 26,761 mHP (slash 22,043, pierce 4,718); without the crit 16,871.
4. **E2.** The marksman headshot deals 47,817 mHP; the torso shot 26,643.
5. **E3.** The dagger sneak heavy deals 84,899 mHP and makes no `rng.combat` draw at `seq` 16.
6. **E4.** Per-bullet results 4,453 (Coherent), 2,288 (Contradicted, non-native) and 3,048 (native wolf). The wolf gains `st.staggered` on the first hit and not on the second.
7. **E5.** SMG per bullet 3,490 at AR 10 and 1,855 at AR 40; sniper 42,939 and 40,222.
8. **E6.** The blocked hit removes 1,034 mHP and 4,405 mst; unblocked it removes 7,563 mHP.
9. **E7.** After shot 1 the ward is 51,183 and HP is unchanged; shot 2 deals 25,878 HP, emits `ev.ward_broken` and applies `st.stunned` for 45 ticks. Without the counter shot 1 leaves 137,061.
10. **E8.** Frag: NPC at 2,500 mm takes 30,717; ally at 3,500 mm takes 17,084; with `mode.ff_off` 7,104.
11. **E9.** The elevated flanking pistol hit at 30 m deals 5,964 mHP.
12. **Spread sample.** Run seed `0x0123456789ABCDEF`, `keyed_rng("rng.combat", 3, (3 << 40) | 5, 1000, 0)`, Coherent auto rifle, cone 546, jitter 18: jitter draw gives +14, `az` 38984, `u` 11749, `rad` 231, `dyaw` −192, `dpitch` −130.
13. **Recoil pattern.** Auto rifle with `jitter_bam` 0, 10 shots, hip, GRP 40: `rec_pitch` 670, `rec_yaw` −72.
14. **Cadence.** 750 RPM shots land on ticks 0, 5, 10, 15, 20, 24 and shot 750 on tick 3,600; 900 RPM fires every 4 ticks.
15. **Jam cadence.** An SMG with failure 1000 held 90 ticks rolls on ticks 0, 32 and 64 only.
16. **Draw counts.** A Coherent shotgun shot makes 17 draws at `seq` 0; with a failure roll due and no jam 18; a jam makes exactly 1.
17. **Mass timing.** Long blade at 700 g: W 8, R 10, `charge_full` 34, stamina 2,250 mst, impact 9; at 3,430 g: W 13, R 16, `charge_full` 52; active ticks stay 5.
18. **Heavy charge.** Long blade release at `c = 25` gives base 28,665; at `c = 5` `heavy_bp` = 15000; at `c = 40` 24000.
19. **Falloff.** Pistol hits at 10,000, 30,000 and 40,000 mm use 10000, 6250 and 5000.
20. **Wall penetration.** A sniper bullet through one wood block keeps 8750 with wallpen 120 left; a pistol bullet (wallpen 20) stops at a non-grazing stone face.
21. **Ricochet.** Velocity (100000, −20000, 0) mm/s onto stone becomes (60000, 12000, 0) with keep 6000; (100000, −80000, 0) stops.
22. **Elevation and flanking.** `dy` 2,500 → 11000; 2,499 → 10000; −2,500 → 9500. `ang_diff` 10,923 → 10000; 10,924 with the target not targeting the attacker → 11500; 10,924 with the target targeting the attacker → 10000.
23. **Lean.** `|lean_k|` 1, 5, 9 give 38, 194, 350 mm. With a wall 200 mm to the right of the eye, full right lean clamps to 100 mm and the head capsule moves 100 mm.
24. **Throw speed.** PWR 60 gives `throw_v_bp` 10488 and grenade speed 16,780 mm/s.
25. **Explosion.** Frag falloff at 1,000, 2,500 and 4,500 mm is 10000, 6572 and 2000; at 4,501 mm no hit; with an opaque stone block between center and target no hit.
26. **Ammo pickup.** Reserve 230 `ammo.light` plus a 32 pickup gives 240 and leaves a 22 pickup.
27. **Energy recharge.** After 120 idle ticks a techne-skinned beam at TEC 9 regains 8,400 mu per 60 ticks; an arcana-skinned beam regains 6,000.
28. **Ward generators.** Kinetic ward 10,000 hit by 8,000 shock passes 3,556 and breaks; aegis passes 1,334. An aegis at ilvl 10 holds 32,380 and recharges 8,095 per 60 ticks after 180 ticks without damage.
29. **Poise.** Warhammer light step 1 on a REF NPC staggers (48,450 mp); a full warhammer heavy knocks down (135,660 mp). Against `elite.unstoppable` neither staggers and poise refills; with `counter.staggerer` the light hit staggers.
30. **Poise reset.** NPC max poise 40, 200, 300 gives reset delays 240, 900, 1,200 ticks; an Erratum refills 480 ticks after the last poise damage.
31. **Armor.** Full Normal medium set at ilvl 1 with Armor 20 gives armor poise 17,600 mp; a Rare Good +1 medium body at ilvl 10 gives torso AR 40.
32. **Armor speed.** Full heavy at Armor 20: sprint 5,960, swim 1,590; full medium 6,140; full heavy at Armor 100: 6,200.
33. **Status power.** Long blade step-1 light gives bleeding power 23; the fifth consecutive hit applies `st.bleeding` at threshold 100.
34. **Threat.** 10,000 mHP of melee damage at MEN 50 adds 13,200 threat.
35. **Downed.** A 17,000 mHP hit on a downed Erratum with Max HP 85,000 removes 360 bleed-out ticks.
36. **Parry and riposte.** A melee hit inside the parry window deals 0 damage and staggers a standard attacker for 30 ticks (a champion for 15); a long blade riposte has base 36,750; a tower shield never parries.
37. **Shove.** A shove against a guarding target with 30 stamina empties it and applies `st.staggered`.
38. **Mounted charge.** A long blade step-1 hit at mount speed 8,000 mm/s has base 18,375; at 7,999 mm/s 14,700.
39. **Disruptor.** A hit on an `elite.surging` champion applies `st.unraveled` and `st.silenced` for exactly 180 ticks; its 2% per second regeneration is 0 during them and resumes after.
40. **Friendly fire.** An ally's rifle hit with 10,000 mHP before step 2.15 gives D2 5,000 by default and 10,000 in `mode.ff_full`; the thrower's own frag at its feet applies `FF_AREA_BP` 10000.
41. **Lag compensation.** An Erratum hitscan with `view_tick = t - 20` tests poses at `t - 16`; an NPC shot tests poses at `t`; a target in `Guarding` at `t` blocks a rewound hit even though it was not guarding at `t - 16`.
42. **Same-tick order.** Two lethal hits on one target from sources with EntityIds 5 and 7 in one tick: source 5's hit kills; source 7's hit is discarded with no `ev.damage_dealt`.
43. **Charge rifle.** Release at `c = 30` gives `charge_bp` 7000, speed 70,000 mm/s, spread factor 15000; release at `c = 14` fires nothing and spends no ammunition.
44. **Severing.** A slash-dominant hit that empties a REF NPC's arm pool (7,500 mHP) severs it and spawns a `cat.body_part`; a blunt-dominant hit without `SEVER` does not.
45. **Grapple mass.** A grappler of 95,000 g total can grapple a target of 118,750 g and not one of 118,751 g.
46. **Disconnected.** With C = 6, `input.attack` on a held pistol performs `arch.melee.unarmed` attacks and no shot.
47. **Determinism.** A 600-tick combat scenario with fixed inputs yields the same `state_hash` on two runs and across a save and load at tick 300.

## Invariants annex

```
Σ doc05: archetypes ⊗ moves ⊗ ballistics ⊗ pipeline-detail ⊗ poise ⊗ counters ⊗ mounts
W! sim combat state ∈ {Fx, Milli, Bp, Bam, Ticks, i64}        N! float in sim or content
W! every action draw := keyed rng.combat(seq per §1.4)         N! running seq counter
W! crit draws = 1 per hit ∧ 0 per sneak hit                    W! spread draws = 2 per projectile
W! hit queue order := (source EntityId, seq)                   N! hit on a target killed earlier in C6
W! pipeline order := Canon §10.5                               W! attack factors ⊂ base_mhp before Canon §10.4
W! mechanics := archetype                                      N! skin changes a mechanic field
W! skill mapping := by archetype                               N! skill by skin
W! Σ scaling = 12000 ∀ base archetype                          N! Σ scaling > 15000
W! light stamina ≤ 7/s per full combo cycle                   W! REF DPS ∈ 30 ± 3 ∀ primary archetype
W! recoil, spread, sway ∈ sim ∧ deterministic                  N! hitstop|camera kick|markers → sim
W! lag comp: poses rewound ≤ 16 ∧ state read at t              N! blocks rewound
W! jam roll := base component ∧ Canon §5.5 cadence             N! jam roll for dependency-none skins
W! armor speed effect := stance base speed only                N! armor multiplies move speed
W! blocked hit ⇒ poise damage 0 ∧ buildup × 5000               W! parried hit ⇒ damage 0
W! champions ⊂ {warded, surging, unstoppable}                  W! ∀ counter: dep-none source ∈ every genre
W! ward gen profile ⇒ generator pool only                      N! profile on st.shielded pool
W! FF := step 2.15 ∧ self-blast = area FF                      N! friendly hit → threat | XP | Shock of the New
W! severing deterministic                                      N! Errata head severed
W! mount HP ∝ H(d) ∧ mount AR ∝ T(ilvl)                        N! mounted ∧ hidden
```
