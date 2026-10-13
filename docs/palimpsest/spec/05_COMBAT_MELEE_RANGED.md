# 05 — Combat: Melee, Ranged, Armor and Damage
As of 2026-10-11. Status: BINDING. Written against canon-1.12 (adversarial check pending).

This chapter owns every weapon, armor, shield, ward generator and ammunition archetype, the melee moveset, firing, ballistics, explosions, poise and stagger, blocking and parrying, threat, the doc-05 details of the canonical damage pipeline (Canon §10.5), champion counters, weapon Contradiction failures and the combat-feel parameters. Every number here is a starting value tuned against the Folio-1 anchors of Canon §17.4. Every formula is integer, every random draw is keyed, and nothing here reads a float. Gear has no durability (Canon §4.3, DECISIONS.md item 65): no weapon, armor, shield, ward generator or throwable wears or breaks; only destructible blocks and placeables have block HP (doc 07).

## Imports

| Source | Sections consumed |
|---|---|
| Canon §1 | numeric types and primitives (`Fx`, `Milli`, `Bp`, `Bam`, `Ticks`, `floor_div`, `ceil_div`, `round_half_even`, `mul_bp`, `mul_fx`, `div_fx`, `dist2`, `sqrt_fx`, `fx_from_milli`, `fx_floor_block`); checked shifts and the coordinate range (canon-1.5); display-unit rule; integer content encoding; coordinates; `sin_bam`, `cos_bam`, `fwd`, `ang_diff`, `atan2_bam`; ascending-EntityId iteration |
| Canon §2–§3 | `TICK_HZ`, `DT`, semi-implicit Euler, tick schedule, rate accumulator, `in_combat`, replay `view_tick`; keyed `rng.combat` and `rng.status` (owner 05), the `seq` range `0x0000..=0x01FF` that §3.3 reserves for §1.4 on both streams, and the §3.4 draw counts |
| Canon §4–§6 | gravity, terminal velocity, `PROJECTILE_CAP`, fixed terrain, `block.tag.destructible`, fixed terrain stopping every projectile and no item durability (§4.3), humanoid capsule, movement and the final move speed chain with doc 05 stance bases (§4.2), dodge, lean; dependencies, tolerance, Contradiction, Anachronism, Shock of the New, ambient TECHNE and MYTHOS, local genre and seam zones, `vrv(e)`, manufacturer signature traits (§5.9), World Laws and `law_bound` (§5.10), `st.unmoored` (§5.11); shortfall; §6.4 rows 4–6, 11–20, 39, 41; §6.5 |
| Canon §7, §9 | skills and combat XP; body plans, `share`, `hit_mult`, precision defaults, baked hitbox poses, 32-tick pose history, severing (an Erratum's limb only from a crippled pool, DECISIONS.md item 84) and hand bindings |
| Canon §10–§12 | damage types, armor efficiency, `ward_mult`, resistances, armor and `pen_bp`, source magnitude with doc 05's attack factors and "× N" manufacturer traits (§10.4), the pipeline (step 2.12 `st.taunted`, step 3a fae iron, step 5 shield AR on blocked hits, step 6c Scribe's Mercy, step 7 ward profiles and `counter.ward_breaker`), entry points (consumables, lethal takedowns), blocking with the per-type split (§10.7), lethal resolution in two hit batches (§10.8); statuses (doc 05 owns bleeding, burning, corroded, exposed, staggered, stunned, frozen, blinded, chilled, shocked, knocked_down, slowed, disarmed, weakened, taunted, grappled), buildup, DoT, direct application, multiplier order, player-controlled bearers; `res.stamina`, `res.poise`, `res.ward`; surfaces |
| Canon §13–§19 | categories, `slot.ward` and its capacity chain, rarity power, quality, enhancement, `P`, `T`, `H`, `S`, `src_lvl`; factions and minions; party tables, ranks, champion counters and elite aspects, reference anchors, friendly fire, ghosts, the retirement of mounts (§17.7), invasions (§17.9: the invader is replay slot 4 and is lag-compensated like an Erratum); events; namespace ownership |
| Docs 02, 03, 04, 06, 07, 08, 09, 10, 11, 12, 13, 14, 16, 17, 18 | the interfaces listed under "Interface assumptions"; DECISIONS.md rows 55, 65, 84, 88, 89, 95, 131, 134, 135, 142, 143 |

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
| C1 | #7 `sys.actions` | Apply each actor's command: at most one action start per actor per tick (§1.4), weapon state machines (§4.8, §6.1), lean target (§6.8), ADS, reload, swap, guard raise, throw release, imbue. Projectile spawns requested here are applied after #7 (doc 03 §2.2). |
| C3 | #13 `sys.melee` | Melee traces for every swing in its active ticks (§2.3), against the post-movement poses of tick `t` (rewound for player attackers, §2.6). |
| C2, C4 | #14 `sys.projectiles` | Doc 03 §5.19 moves every projectile. On its spawn tick a projectile first takes its lag-compensation catch-up steps (C2, §2.6); every projectile then takes its normal step (C4). Doc 05's hooks (§7.1) decide what each contact does. |
| C4 | #15 `sys.hitscan` | Beam pulses due this tick (§7.6). |
| C5 | #16 `sys.explosions` | At most 4 due explosions, in queue order (doc 03 §12); doc 05 supplies their parameters and builds their area hits (§9.4). |
| C6 | #17 `sys.damage` | The first of Canon §10.8's two hit batches: sort the actor hits (weapons, spells, gadgets, thrown items and the area hits of their explosions) queued for #17 by `(source EntityId, seq)` ascending and resolve each hit completely, one at a time (§10.2): pipeline steps 1–9 including poise and stagger (§12.4, §12.5), buildup (§13), counters (§15.1), severing (§19.1) and lethal resolution (Canon §10.8). A hit whose target died earlier in this C6 is discarded with no event. |
| C7 | #17, after C6 | Interrupts: every entity that gained a disabling status in C6 transitions to `Staggered` (§4.8, §6.1); swings in progress end and produce no further samples. |
| C8 | #17, after C7 | Timers: bloom and recoil recovery (§6.4, §6.5), poise reset (§12.5), ward recharge (§15.4), threat decay (§14.1), parry and riposte windows (§4.5). |
| C9 | #17, after C8 | Nothing is batched: every event is appended to doc 03's event list on the call that creates it (doc 03 §2.4), so events keep creation order. |
| — | #25 `sys.status_pulse`, #26 `sys.damage_late`, #33 `sys.status_timers` | DoT and buildup decay (Canon §11.2, §11.3); the second hit batch of Canon §10.8 in #26: environmental, contact, surface and DoT hits, and every hit queued while C6 runs (shock chains, thorns), sorted by `(source EntityId, seq)` ascending and resolved by C6's rules; status expiry. |

Each weapon and guard state machine makes at most one transition per tick, in C1, except the interrupts of C7. A state's counter `k` (or `c`) is 0 on the tick the state is entered and rises by 1 on every later tick the state persists.

### 1.4 Keyed draw allocation

Every draw in this chapter uses `keyed_rng(stream, folio, actor, tick, seq)` (Canon §3.3) with `tick = t`. The `seq` values are allocated by this table and never by a running counter, so a draw never depends on how many other draws happened that tick.

**One action start per actor per tick.** `seq` 0 is shared by every action that can start on a tick: a doc 06 cast, a quickbar item use (docs 08, 11), a throw, a doc 07 gadget deploy, and a main-hand weapon action (shot, beam pulse, charge release, melee swing, imbue). At most one of them starts per actor per tick, chosen in that priority order; the losing inputs are ignored that tick and never buffered (doc 06 §3.5 gives casts the same priority). Off-hand actions (dual-wield swings, off-hand pistol shots, the raise of a shield with a dependent skin) use `seq` 1 and may start on the same tick as a `seq` 0 action.

| `seq` | `actor` | Draws, in this order | Count |
|---|---|---|---|
| 0 | the wielder | primary action started this tick: Contradiction failure roll if one is due (§16.2); then for a shot or beam pulse: 1 recoil jitter, 2 spread values per projectile, then 1 value per per-shot chance component (`affix.frugal`, `mfr.post_apocalypse.lastditch`, doc 11) in ascending ID; for a throw: 2 spread values; for a melee swing: 1 value per per-swing chance component in ascending ID; for an imbue: nothing further | `f + 1 + 2n + s` for a shot with `n` projectiles and `s` per-shot chance components, where `f ∈ {0, 1}`; a shot that fails stops after the failure roll |
| 1 | the wielder | off-hand action started this tick, same rules as `seq` 0 | as `seq` 0 |
| 2 | — | reserved; never drawn | 0 |
| 3 | the bearer | ward-generator flicker roll (§15.4) | 1 |
| `16 + i` | the hit's `source` | the `i`-th hit with that source this tick (`i` from 0 in detection order): 1 crit roll (none for a sneak attack or an entry-point hit, §10.2; drawn and ignored for a `NO_CRIT` hit); then each "X Bp chance" on-hit component in ascending affix or perk ID order, 1 draw each. A component that applies a status draws from `rng.status` with the same key; every other component draws from `rng.combat` | `1 + procs`, or `procs` for a sneak attack or an entry-point hit |

`source` is the attacker for melee and hitscan hits and the projectile's own `EntityId` for projectile impacts and the explosions they cause. A projectile is an entity, so its keyed draws are independent of its owner's later actions. Doc 07 turrets and drones are actors too: their shots use the gadget's `EntityId` (doc 07 §11.4).

`i` is bounded by the entities one source can reach in one tick, which `ACTIVE_ACTOR_CAP`, `PARTY_MAX` and `FOLIO_MINION_CAP` keep below 300, so every doc 05 `seq` lies in `0x0000..=0x01FF`, the range Canon §3.3 reserves for this table on `rng.combat` and `rng.status`. Every other chapter's allocation on these two streams lies outside it, such as `0xFFFF` on `rng.combat` for doc 04's Death's Door (Canon §3.3).

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
    pub flags: u32,                // SEVER, GUARD_BREAK, IGNORE_ARMOR, RIPOSTE, AREA, PARRYABLE, BLOCKABLE, NO_CRIT, TAKEDOWN
    pub sneak_tier: u8,            // 1..=3 from the archetype (§3.2c, §5.2d); doc 09 reads it
    pub incoming: [Fx; 3],         // vector from the target's origin toward the attack origin; only its bearing is read (§12.2)
    pub dist_mm: i64,              // path length for falloff
    pub src_lvl: i32,              // Canon §13.8.1
    pub entry: u8,                 // Canon §10.6 entry step: 0 for weapon, spell, gadget and thrown hits; 3, 4 or 7 otherwise (§10.2)
    pub spell_pot_bp: i32,         // Canon §10.5 step 2.1: doc 06's spell_pot for spell and prayer hits, else 10000
}
```

### 1.6 Stance base speeds

Canon §4.2 makes final move speed equal base speed, then §6.4 row 11, then doc 04's `body_move_bp`, then statuses, and lets the doc 05 stance bases below replace the base speed while they last; armor never multiplies speed. When several stances apply, the lowest base speed wins; crouch walk (2000 mm/s) takes part in the same minimum.

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

### 1.7 Inputs consumed

Doc 17 owns the bindings and doc 02 §5 the bit layout. This chapter reads these logical inputs from the command's `buttons` bitfield; doc 03 #4 derives presses by comparing with the previous tick's buttons.

| Input | Bit (doc 02) | Use in this chapter |
|---|---|---|
| `input.attack` | 0 `ATTACK_PRIMARY` | light attack, trigger, draw, beam |
| `input.alt` | 1 `ATTACK_SECONDARY` | guard, ADS, off-hand attack |
| `input.reload` | 2 | reload, clear a jam; taunt while guarding with a shield (§19.2) |
| `input.jump`, `input.crouch`, `input.sprint`, `input.dodge` | 3, 4, 5, 6 | jump attack, crouch stance and underhand throw, sprint attack and hold breath, dodge cancel |
| `input.interact` | 7 | struggle while grappled (§19.3) |
| `input.heavy` | 8 `MELEE_QUICK` | heavy charge; shove or shield bash while guarding; quick melee with a ranged weapon |
| `input.throw` | 10 | cook and throw the quickbar throwable |
| `input.weapon_next`, `input.weapon_prev`, weapon select | 11, 12, 13–14 | swap weapons (§4.8, §6.1) |
| `input.lean_l`, `input.lean_r` | 21, 22 | lean (§6.8) |

Every action has a `move.` ID carried in `HitRecord.move_id` and in events: light combo steps are `move.light`, heavies `move.heavy`, a raised guard `move.guard`, aiming `move.ads`, reloading `move.reload` and clearing a jam `move.clear_jam`; the other moves are named where they are defined.

### 1.8 Move index

Every `move.` ID is listed here once with its trigger and the section that holds its exact numbers. Swing shapes and fire modes are parameters of other moves, not actions of their own.

| Move | Trigger | Rules |
|---|---|---|
| `move.light` | `input.attack` in `Idle`; one combo step per press | §4.1, Tables 3.2b and 3.2c |
| `move.heavy` | `input.heavy` charge and release | §4.2, Table 3.2b |
| `move.sprint_attack` | `input.attack` after at least 20 sprint ticks | §4.3 |
| `move.jump_attack` | `input.attack` or `input.heavy` while airborne with the §4.4 air probe | §4.4 |
| `move.riposte` | `input.attack` or `input.heavy` inside a riposte window | §4.5 |
| `move.shove` | `input.heavy` while guarding without a shield | §4.6 |
| `move.quick_melee` | `input.heavy` with a ranged weapon active | §4.7 |
| `move.guard` | `input.alt` held with no off-hand weapon | §4.8, §12 |
| `move.shield_bash` | `input.heavy` while guarding with a shield | §12.1 |
| `move.taunt` | `input.reload` while guarding with a shield | §4.8, §19.2 |
| `move.grapple` | full heavy of a fist weapon or unarmed; full chain heavy | §19.3 |
| `move.struggle` | `input.interact` while grappled | §19.3 |
| `move.ads` | `input.alt` held with a ranged weapon that allows ADS | §6.3, §6.6 |
| `move.hold_breath` | `input.sprint` while fully aimed with `ads_sway >= 27` | §6.7 |
| `move.lean_l`, `move.lean_r` | `input.lean_l`, `input.lean_r` held | §6.8 |
| `move.fire_semi`, `move.fire_auto`, `move.fire_burst`, `move.fire_bolt`, `move.fire_charge`, `move.fire_beam` | `input.attack` under the weapon's fire mode | §6.2 |
| `move.throw` | release of a thrown weapon or a quickbar throwable after a wind of at least 12 ticks | §5.3, §9.2 |
| `move.cook` | `input.throw` pressed with a throwable on the quickbar | §9.2 |
| `move.reload` | `input.reload` with a partial or empty magazine | §6.1 |
| `move.clear_jam` | `input.reload` while `Jammed` | §6.1, §16.3 |
| `move.imbue` | use of an `ammo.charge_*` counter charge | §15.3 |
| `move.quickdraw` | law-bound technique of `law.weird_west.high_noon` | §19.4 |
| `move.qinggong_strike` | law-bound technique of `law.wuxia.qinggong` | §19.4 |
| `move.shape_rl`, `move.shape_lr`, `move.shape_overhead`, `move.shape_thrust` | swing shapes used by the moves above | §2.3 |

---

## 2. Hit detection

### 2.1 Query kinds

| Query | Used by | Shape | Entity poses | Blocks |
|---|---|---|---|---|
| Melee trace | swings, bashes, shoves | blade segment swept by sub-samples (§2.3) | rewound for player attackers (party and invader, §2.6), current for NPCs | current |
| Projectile sweep | every non-hitscan projectile | doc 03 §5.19: the step segment against hitbox capsules inflated by the projectile radius (`first_body_event`) and against blocks and liquid cells (`first_world_event`) | current, except catch-up steps (§2.6) | current |
| Hitscan ray | beams (§7.6) | segment from the leaned eye to `max_range` | rewound for player attackers, current for NPCs | current |
| Area query | explosions, shockwaves | doc 03 §12.4: bodies whose center point lies within the radius, five exposure samples (§9.4) | current | current, after the explosion's own block damage |

Every query tests hitbox capsules posed from the committed fixed-point pose tables (Canon §9). A capsule is a segment `(c0, c1)` in world space plus radius `rc`. The attacker never hits its own capsules.

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

`seg_seg_params` is doc 02 §4.6's `closest_seg_seg` `(s, t)` result, shown here for reading; the simulation calls doc 02's one implementation (doc 02 rule "one formula, one function"). `div128(n, d) = Fx(floor_div(n << 32, d) as i64)` for Q64.64 numerator and positive denominator (negate both first when `d < 0`); `div256` is the same over the two-limb `I256` helper. `clamp01` clamps to `0..=Fx::from_int(1)`. Where an entry parameter of a swept sphere into a capsule is needed, it is `s_hit = max(0, s - div_fx(Fx(isqrt_u128(rr² - d²)), dist(p0, p1)))`, with `d² = dist2(pa, pb)` and `rr²` both raw Q64.64; when `dist(p0, p1) = 0`, `s_hit = 0`.

**Voxel traversal** is doc 03 §5.18's `raycast_blocks` over doc 02's `dda` (Amanatides–Woo; ties step X, then Y, then Z; partial boxes tested by slabs). The walk visits cells in strictly increasing entry parameter and stops at the first cell for which the caller's predicate returns true, or at the segment's end. Cells outside the Folio read as `block.void_bedrock` (Canon §1.8).

### 2.3 Melee traces

A swing sweeps a **blade segment** through the active ticks of its move.

- **Pivot**: `pivot = eye + (0, -200 mm, 0)`, where `eye` is the attacker's eye position including the lean offset (§6.8), recomputed on every active tick from the attacker's position on that tick. Yaw arithmetic in the shape table wraps (Canon §1.9); pitch results are clamped to `-16000..=16000`.
- **Blade**: at angles `(yaw_s, pitch_s)`, `hilt = pivot + fwd(yaw_s, pitch_s) × 300 mm` and `tip = pivot + fwd(yaw_s, pitch_s) × tip_dist`. Blade radius `blade_r` is 40 mm, or 60 mm for `move.shape_thrust`.
- **Sub-samples**: `K = 4` per active tick when `reach_mm <= 2000`, else `K = 8`. A swing with `A` active ticks has `N = A × K` samples; sample `j ∈ 0..N` belongs to active tick `floor_div(j, K)`. Its progress is `(j + 1) / N`.
- **Shapes** (aim yaw `Y` and pitch `Pt` are read once at the first active tick and frozen for the swing; `arc` is the archetype's half-angle):

| Shape | `yaw_s(j)` | `pitch_s(j)` | `tip_dist(j)` |
|---|---|---|---|
| `move.shape_rl` | `Y - arc + floor_div(2 × arc × (j + 1), N)` | `Pt` | `reach` |
| `move.shape_lr` | `Y + arc - floor_div(2 × arc × (j + 1), N)` | `Pt` | `reach` |
| `move.shape_overhead` | `Y` | `Pt + av - floor_div(2 × av × (j + 1), N)`, `av = max(arc, 5461)`, clamped to ±16000 | `reach` |
| `move.shape_thrust` | `Y` | `Pt` | `r0 + floor_div((reach - r0) × (j + 1), N)`, `r0 = mul_bp(reach, 4000)` |

- **Blocks and clang.** At each sample, traverse voxels from `hilt` to `tip` with the predicate `solid`. If a solid cell is entered at distance `s_blk` from the pivot, the effective blade ends there. For every shape except thrust, if `s_blk < mul_bp(reach, 6000)` the swing **clangs**: this sample and all later samples produce no entity hits, the active phase ends, recovery gains 8 ticks, the block takes the swing's block damage once (§7.3), and `emit_noise` fires `noise.melee.clang` (70 dB, doc 09) at the contact point. A thrust stops at the block without clanging and damages it once.
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

- `LAGCOMP_MAX_TICKS = 16` (doc 16's `LAGCOMP_MAX_REWIND` equals it). The host first clamps the command's `view_tick` per doc 16 §5.5 and §8 (at most the measured lag plus 4 ticks, never more than 16 ticks back). For a command applied at tick `t`, `rewind = min(t - view_tick, LAGCOMP_MAX_TICKS)` and `t_r = t - rewind`.
- **Who is rewound.** Every player-controlled attacker: party slots 0 to 3 and the invader in slot 4 (Canon §17.9). NPC attacks, minions and turrets never rewind.
- A player's melee samples and hitscan rays test every other entity's capsules posed at `t_r`, reconstructed from doc 03 §5.16's 32-record pose history. Blocks are never rewound. A target whose `spawn_tick` is later than `t_r` cannot be hit by a rewound test (doc 16 G6).
- A player's projectile spawned at tick `t` has `catchup = rewind` (doc 03 §5.19). On its spawn tick it takes `catchup` catch-up steps, step `i` (0-based) against poses at `t_r + i + 1`, so the last one tests the poses of `t`; it then takes its normal step against the poses of `t`, and one step per tick afterwards. Every other projectile has `catchup = 0` and takes exactly its normal step on its spawn tick. Example: fired on tick 1000 with `view_tick` 994, the projectile steps against poses 995, 996, 997, 998, 999, 1000 and then 1000 again: 7 steps on its spawn tick.
- Target **state** always reads tick `t`: guard raised, parry window, dodge invulnerability, statuses, ward, HP. Position comes from the shooter's view; defense comes from the host's present.
- Replays reproduce every lag-compensated hit from the logged `view_tick` (Canon §2.4).

### 2.7 Same-tick ordering

Hits resolve in Canon §10.8's two batches. Actor hits (weapons, spells, gadgets, thrown items and the area hits of their explosions) form the first batch, resolved in C6 in ascending `(source EntityId, seq)`. Environmental, contact, surface and DoT hits, and the hits C6 itself queues (shock chains, thorns), form the second batch, resolved in #26 `sys.damage_late` in the same order. A source that dies during C6 still resolves its remaining hits. Two hits on the same target resolve in batch order, then in queue order, so lethal resolution (Canon §10.8) sees them in that order: a weapon hit resolves before a DoT pulse of the same tick whatever their source IDs.

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

All melee archetypes have `pen_bp` 0; penetration on melee comes only from affixes and perks (doc 11). `arch.melee.unarmed` is the body function used with empty hands; `kin.beastkin` replaces its shares with blunt 5000, slash 5000 (Canon §16.3). Its `src_lvl` is `d_current` (innate) and its `material_bp`, rarity and quality factors are 10000. It is also the natural attack of every non-humanoid body: doc 14's Fenrir (`body.quadruped`: bite, claw, pounce) and Jörmungandr (`body.serpentine`: bite, constrict, spit) attack with it, and those names are skins (doc 18) that change no field of Tables 3.2a to 3.2d. A spit skin is a melee thrust at the archetype's 700 mm reach, never a projectile.

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
| whip | 700 | 14/4/16 | rl 10000, lr 10000, overhead 12500 | 4000 | 40 | 15000→22000 | 6/4/18 | 12000 | overhead; `counter.disruptor` at full; pull at full |
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
- **Whip pull.** A full whip heavy that lands (not blocked or parried) calls doc 03 `apply_knockback(target, d, 6000 mm/s)` after step 8, where `d` is the horizontal unit vector from the target toward the attacker; doc 03 scales it by the target's mass, so a 75 kg target is pulled at 6000 mm/s and a 300 kg one at 1500 mm/s.
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

`mass_g` is the instance's mass from doc 07 (volume × density) and `material_bp` its primary material's `dmg_bp` (Canon §10.4). `arch.melee.unarmed` has no instance and no reference mass, so §3.3 never applies to it (every factor is 10000). Ranged weapons apply `time_bp` to `swap_ticks` and `ads_ticks`, and multiply recoil by `clamp(10000 - floor_div((mass_ratio_bp - 10000) × 2000, 10000), 8000, 12000)`.

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

Maces and warhammers also emit a **landing shockwave**: an area hit of radius 1500 mm around the landing point with `base_mhp × 5000` blunt, impact factor 15000, never precision, excluding any entity the jump attack itself hit. It uses §9.4 step 2's target selection and exposure samples with `inner_mm = radius_mm` (no falloff); it is queued directly in C3, is not a doc 03 explosion, breaks no blocks and applies no knockback. The warhammer's jump attack carries `counter.staggerer`. The jump attack's active phase lasts until the landing tick, at most 40 ticks; that count is its `A`.

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

- **Quick melee** (`move.quick_melee`): `input.heavy` with a ranged weapon active. Base 9000 mHP, blunt 10000, reach 1100, arc 4551, `move.shape_thrust`, W 6 / A 3 / R 18, 6 stamina, impact 15, `block_bp` 1000, skill `skill.unarmed`, scaling as `arch.melee.unarmed`. It uses the held weapon's `ilvl`, rarity, quality, enhancement and material for Canon §10.4.
- **Dual wielding.** A one-handed melee weapon in the main hand and a dagger, short blade, axe, mace or fist weapon in `slot.offhand` turns `input.alt` into off-hand light attacks with their own combo index at `move_bp` 8500. The off-hand cannot block, charge a heavy or parry. A pistol, heavy pistol or thrown weapon in `slot.offhand` fires with `input.alt` from the hip only, with spread × 12500.

### 4.8 Melee state machine

States: `Holstered`, `Drawing(k)`, `Idle`, `Windup(m, k)`, `Active(m, k)`, `Recovery(m, k)`, `Charging(c)`, `GuardRaising(k)`, `Guarding(k)`, `Lowering(k)`, `Staggered`, `Disarmed`. `m` is the move and `k` the tick within the state. "Disabling status" means `st.staggered`, `st.stunned`, `st.frozen`, `st.knocked_down`, `st.sleeping` or `st.downed`, a `st.shocked` micro-stun (§13), or `st.grappled` while a two-handed weapon is active.

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
| `Idle` | `input.heavy` pressed, stamina > 0, no `st.exhausted` | `Windup(jump attack, 0)` if §4.4 holds, else `Charging(0)` |
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
| `Guarding(k)` | `input.reload` pressed with a shield, stamina ≥ 15, taunt cooldown over | `Windup(taunt, 0)` with W 20 / A 1 / R 9; the guard drops; the taunt applies on its active tick (§19.2) |
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
| `st.downed` (doc 16) | every weapon action, guard, reload, throw and item use |
| dodge in progress | every new weapon action |
| swimming | two-handed weapons cannot swing, fire or aim; one-handed ranged weapons fire at the swimming stance (§6.3) |
| climbing or mantling | every weapon action |
| doc 06 cast machine outside `Idle` | new weapon actions (doc 06 §3.5) |
| `Cooking` or throw recovery (§9.2) | main-hand weapon actions |
| `move.imbue` (§15.3) | every other weapon action until the channel ends |

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
- **Thrown weapon.** Holding `input.attack` winds for at least 12 ticks; release throws. The interval from one release to the next press is 18 ticks, so the cycle is 30 ticks. Thrown weapons become ground items where they stop (§9.2). Knife, javelin and sling-stone skins all use this archetype. A one-handed sling is the sling-stone skin: the sling itself is presentation, and its stones are the held stack, so a sling never draws from an `ammo.` family (§5.6).

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
- **Expected results** (integer pipeline, crits at 500 Bp, every swing connecting on its first active tick and every shot hitting): melee mean time to kill 1.84 s (mace) to 2.20 s (dagger); pistol 1.93 s; auto rifle 2.07 s. A ranged weapon that starts on a full magazine kills in 1.15 to 1.17 s, because one magazine holds 1.5 to 2 kills by design; sustained fire over many kills returns to the REF DPS.

### 5.6 Role names used by other chapters

Docs 04 and 14 describe Calling kits with role names. None of them is an archetype or an ammunition family of its own; each resolves to exactly one export of this chapter, and genre skins then rename it (doc 11 §4.2). Kits and templates name only the exported IDs.

| Role name | Exported ID | Why |
|---|---|---|
| sidearm | `arch.ranged.pistol` | the one-handed marksmanship sidearm and the Calling reference weapon (§5.5) |
| longarm | `arch.ranged.rifle_marksman` | the two-handed marksmanship rifle; it fires `ammo.rifle` |
| heavy automatic | `arch.ranged.rifle_auto` | the heavy_arms automatic; it fires `ammo.heavy` |
| flamer, flamethrower | `arch.ranged.beam` | doc 11's flamer skins are beam skins: fire 10000 pulses on `ammo.cell` (carry cap 300 units) |
| hand crossbow | `arch.ranged.pistol` | a crossbow held in one hand is a one-handed sidearm on `ammo.light`; `arch.ranged.crossbow` is two-handed |
| sling, sling stones | `arch.ranged.thrown` | sling-stone skin; the stones are the held stack (§5.3) |
| hafted mace | `arch.melee.mace` | one-handed hafted |
| hafted maul | `arch.melee.warhammer` | two-handed hafted |
| polearm spear | `arch.melee.spear` | one-handed polearm |
| polearm staff | `arch.melee.quarterstaff` | the melee staff; the casting staff is `arch.ranged.staff` (§5.4) |
| hand wraps, fist wraps | `arch.melee.fist` | the `skill.unarmed` weapon; bare hands are the body function `arch.melee.unarmed` |
| small shield | `arch.offhand.shield_buckler` | |
| medium shield | `arch.offhand.shield_heater` | |
| armor named class first (light head, heavy body) | `arch.armor.<slot>_<class>` | slot first, class second: `arch.armor.head_light`, `arch.armor.body_heavy` |
| fuel | `ammo.cell` | the beam family |

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
| `Ready` | throw mode (thrown weapon): `input.attack` pressed, `cool_mt <= 0`, at least one item in the held stack | `Charging(0)` (the wind) |
| `Ready` | beam mode: `input.attack` held, magazine ≥ 1000 mu | `Beaming(0)`; the first pulse fires this tick |
| `Ready` | trigger condition met (§6.2), `cool_mt <= 0`, magazine ≥ `ammo_per_shot` | the Contradiction cadence (§16.2) runs first: on a jam `Jammed` with no shot and no ammunition spent; otherwise fire; burst mode enters `Burst(1, 0)` |
| `Ready` | trigger condition met, `cool_mt <= 0`, magazine < `ammo_per_shot` | `Reloading(0)` if reserve > 0; else a dry-fire click (`emit_noise` 30 dB) and `Ready` |
| `Burst(n, k)` | `n = burst_n` or magazine < `ammo_per_shot` | `Ready` |
| `Burst(n, k)` | `k + 1 = burst_ticks` | fire with no new failure roll (§16.2); `Burst(n + 1, 0)` |
| `Charging(c)` | `input.dodge` or `input.sprint` | `Ready`; nothing spent |
| `Charging(c)` | release with `c < 15` | `Ready`; nothing spent |
| `Charging(c)` | release with `c >= 15`, or a forced release (§5.3) | the failure roll: on a misfire `Ready` with the round still loaded; otherwise fire and enter `Reloading(0)` with total 18 (the nock) when reserve > 0, else `Ready` |
| `Charging(c)`, throw mode | release with `c < 12` | `Ready`; nothing thrown |
| `Charging(c)`, throw mode | release with `c >= 12` | the failure roll (a dud still throws, §16.3); throw one item from the stack with `cool_mt = 18000`; `Ready`. The 12-tick wind and the 18-tick cooldown make the 30-tick cycle that the table's `rpm` 120 states |
| `Beaming(k)` | `input.attack` released, or magazine < 1000 mu | `Ready` |
| `Beaming(k)` | a due failure roll fails (§16.2) | `Stalled(0)` |
| `Beaming(k)` | `(k + 1) % 6 == 0` | pulse (§7.6); `Beaming(k + 1)` |
| `Stalled(k)` | `k + 1 = stall_total`: 60 for a beam stall (§16.3), 90 for the heat lock of `mfr.space_opera.umbra` (§10.5) | `Ready` |
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
atk_speed_bp: 10000, then × shortfall factor (Canon §6.3), then × the item's manufacturer factor (§10.5: Deepanvil 9000),
              then × 13000 for an NPC with elite.surging (Canon §17.3a),
              then statuses in ascending ID: st.chilled 8000, st.hasted 11500        // non-status factors first, Canon §11.7
in C1:  if trigger_met && cool_mt <= 0 { fire(); cool_mt += interval_mt }
in C8:  cool_mt = if cool_mt > 0 { cool_mt - 1000 } else { 0 }
        // a running cooldown may end slightly negative: that sub-tick carry keeps a continuous string exact;
        // a weapon whose cooldown has run out and that does not fire (idle, reloading, jammed, drawing, staggered) holds 0, so nothing banks
```

Shot `n` (0-based) of a continuous string that starts on tick `t0` fires on tick `t0 + ceil_div(n × interval_mt, 1000)`. At 900 RPM `interval_mt = 4000`, so shots land every 4 ticks. At 750 RPM `interval_mt = 4800`, so shots land on ticks 0, 5, 10, 15, 20, 24, … and shot 750 lands on tick 3,600.

| Mode | Trigger condition |
|---|---|
| `move.fire_semi` | a press; a press during cooldown is buffered 6 ticks |
| `move.fire_auto` | held |
| `move.fire_burst` | a press fires `burst_n` (default 3) shots `burst_ticks` (default 3) apart (§6.1 `Burst`); each later burst shot sets `cool_mt = interval_mt`, so the normal interval follows the last one; used only by doc 11 overrides |
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
statuses, ascending ID: st.blinded 20000, st.focused 7000
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

- ADS is allowed when the weapon has `ads.ticks > 0`, is not harness-carried, and the wielder is not sprinting, not in `SprintLock` and not swimming.
- ADS changes the stance base speed (§1.6), spread (§6.3), recoil (§6.5) and sway (§6.7). The renderer's zoom is presentation only.
- After sprint ends, the weapon cannot fire or raise ADS for `sprint_to_fire` ticks.
- Doc 04 Heavy Arms 100 exempts the heavy_arms archetypes (auto rifle, shotgun, launcher, beam) from `SprintLock`: they fire from the hip while sprinting, at Table 6.3's walking multiplier.

### 6.7 Aim sway

Sway is a deterministic Lissajous drift with no random draws:

```
amp = if ads_ticks == 0 { 9 } else { 9 + floor_div((ads_sway - 9) × ads_frac, ads_ticks) }
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

- **Wall clamp.** If any solid box lies strictly closer than 100 mm to `eye_lean` (a sphere of radius 100 mm overlapping it), `|offset_mm|` becomes `max(0, |offset_mm| - 50)` and the test repeats until it passes or the offset is 0. The clamp is recomputed every tick from `lean_k`, and `lean_k` itself keeps moving toward its target, so the eye never oscillates. This rule replaces doc 03 §5.8's point test, because Canon §4.2 gives the lean to doc 05.
- The capsule stays put. `bp.head` translates with the eye; no other part moves.
- Shots and melee pivots originate from the leaned eye. ADS works while leaning. The camera mode (first person, or the third-person toggle of DECISIONS.md item 3, doc 17) is presentation only: in both, every shot, beam and swing starts from the leaned eye along the aim of §6.5, so the simulation never reads the camera.
- Lean is unavailable while sprinting, climbing, swimming, downed or during a melee swing; while unavailable, `lean_k` moves 1 toward 0 per tick. Lean offsets by `|lean_k|`: 1 → 38 mm, 5 → 194 mm, 9 → 350 mm.
- Doc 03 §5.8 advances `lean_k` inside `sys.movement` by this rule and poses the head capsule from the clamped offset; doc 05 owns the offset formula and the wall clamp.

---

## 7. Ballistics

### 7.1 Per-tick projectile integration

Doc 03 §5.19 owns integration, geometry, catch-up and lifetime; doc 05 owns the projectile's combat fields, the four contact hooks and `after_step`, so the two chapters never disagree about where a projectile is.

```rust
pub struct ProjCombat {             // doc 03 Projectile.combat
    arch: Id32, spawn_y: Fx, traveled_mm: i64,
    pierce_left: u8,                // = the archetype's pierce count at spawn: entities it may still hit
    wallpen0: i32, wallpen_left: i32, keep_bp: i32, ricochets: u8, max_ricochets: u8,
    attack_bp: i32,                 // charge_bp or 10000
    hit_set: BTreeSet<EntityId>,    // entities already hit
    owner_hittable: bool,           // false until the first ricochet
    counters: u8, flags: u32, block_bp: i32,
}
// spawn, requested in C1 and applied after doc 03 #7:
//   p = eye_lean (start-of-tick position); v = mul_fx(dir_fx, fx_from_milli(speed_mm_s)),
//   dir_fx = fwd(shot_yaw, shot_pitch) in Q2.30 × 4 (a checked shift to Q32.32, Canon §1.3);
//   radius = fx_from_milli(radius_mm); gravity_bp = the archetype's; drag_bp = 0; bounce_bp = 0
//   (4000 for fused grenades, §9.2); end_tick = t + 600; catchup per §2.6; spawn_y = p.y;
//   keep_bp = 10000; wallpen_left = wallpen0 = wallpen; pierce_left = pierce count;
//   max_ricochets = 1 (2 with a doc 11 perk); the owner's velocity is never added.
// each step, doc 03 §5.19 applies gravity (GRAVITY_BASE × genre gravity_bp × gravity_bp, toward the local
//   gravity sign, the speed along gravity capped at TERMINAL_VELOCITY), moves along the step and calls,
//   in contact order:
fn on_body_hit(pr, target, part, point) -> Continue | Stop        // §7.2
fn on_liquid_enter(pr, cell)                                       // §7.3 "Liquids"
fn on_block_enter(pr, cell, face, point) -> Penetrate | Ricochet | Bounce | Stop   // §7.3, §7.5, §9.2
fn on_stop(pr, point)                                              // stick, detonate or vanish (§7.2, §9.2, §9.4)
fn after_step(pr, moved_mm) {                                      // doc 03 passes the distance moved this step
    pr.traveled_mm += moved_mm;
    let limit = arch.max_range_mm + max(0, mm(pr.spawn_y - pr.p.y));   // Canon §10.5 step 2.4a range rule
    if pr.traveled_mm > limit { on_stop(pr, pr.p); despawn(pr) }       // ev.entity_despawned; rockets detonate
}
```

The hit distance for range falloff is `dist_mm = traveled_mm + mm(dist(p_step, point))`, where `p_step` is the projectile's position at the start of the current step (doc 03 moves `p` only when the step ends) and `dist` is doc 02's `Fx(isqrt_u128(dist2))`. `mm(x)` converts `Fx` meters to integer mm with `floor_div(x.raw × 1000, 2^32)`. A projectile ignores its owner's capsules until `owner_hittable` is set by its first ricochet. A projectile whose `end_tick` passes is removed by doc 03; a rocket then detonates where it is.

### 7.2 Entity hits and body penetration

`on_body_hit` runs in this order:

- If the target's guard is raised and the incoming direction is inside its arc (§12.2), the hit is queued as blocked and the hook returns `Stop`.
- Otherwise it queues one `HitRecord` whose attack factors (§10.1) are `attack_bp`, range falloff at `dist_mm` (§7.1) and `keep_bp`, then sets `pierce_left -= 1`.
- **Body penetration.** If `pierce_left > 0`, it sets `keep_bp = mul_bp(keep_bp, 6000)` and returns `Continue`; doc 03 adds the target to `hit_set` and continues from the contact point. Otherwise it returns `Stop`.
- `on_stop`: arrows, bolts and thrown weapons that stop in an entity stick to it (§9.2); those that stop in a block without having pierced or ricocheted become a ground item at the contact point (a 1-unit `ammo.draw` pickup for arrows and bolts, the item itself for thrown weapons); bullets, pellets and beam pulses vanish; rockets detonate (§9.4).

Falloff:

```
falloff_bp(d) = 10000                                   if d <= start
              = min_bp                                  if d >= end
              = 10000 - floor_div((10000 - min_bp) × (d - start), end - start)   otherwise
```

Example: a pistol hit at 30,000 mm gives `10000 - floor_div(5000 × 15000, 20000) = 6250`.

### 7.3 Block penetration and block damage

`on_block_enter(pr, cell, face, point)` for a solid cell of material `m`:

1. **Penetrable cells.** A cell is penetrable only when its block carries `block.tag.destructible` (doc 03's `destructible_at(cell)`, Canon §4.3) or the block is not opaque (glass, leaves, webs, foliage). Every other worldgen block, and `block.void_bedrock`, has unbounded `pen_cost`. Fixed terrain stops every projectile, beam and penetration (Canon §4.3), so nobody kills through a wall of the level.
2. **Somnambule pass** (returns `Penetrate`). If the projectile was fired or cast through an item of `mfr.oneiric.somnambule` (Canon §5.9, DECISIONS.md item 88), the cell is penetrable and the flight distance at the contact, `traveled_mm + mm(dist(p_step, point))` (§7.1), is at least 10000 mm, the projectile continues through the cell with `wallpen_left` and `keep_bp` unchanged. It never passes fixed terrain. A charged shot of `mfr.cyberpunk.neon_saint` (doc 11) passes the first penetrable cell it enters the same way, whatever its flight distance: that cell is its 1000 mm of block (one `BLOCK_SIZE`, Canon §4), and later cells follow steps 3 to 5 (§10.5).
3. **Penetrate** (returns `Penetrate`) if the cell is penetrable and `wallpen_left >= m.pen_cost` (doc 07; reference values glass 2, leaves 1, wood 40, dirt 60, stone 120, brick 150, iron 300): `wallpen_left -= m.pen_cost`, `keep_bp = mul_bp(keep_bp, 10000 - floor_div(5000 × m.pen_cost, wallpen0))`, and the projectile continues through the cell.
4. Otherwise **ricochet** (returns `Ricochet`) if §7.5 allows; fused grenades return `Bounce` (§9.2).
5. Otherwise **stop** (returns `Stop`) at the entry point.

Hits owned by the invader (slot 4) never damage blocks (Canon §17.9). Each destructible cell the projectile enters, penetrated or not, takes block damage through doc 03 §13.2: `mul_bp(B_proj, block_bp)`, where `B_proj` is the projectile's Canon §10.4 magnitude times its current attack factors and `block_bp` comes from §5.2 (projectiles) or Table 3.2d (melee, once per swing). The tool tier is the firing weapon's (Table 3.2d note). Doc 03 decides breakage. Non-destructible cells take no damage.

Example: a sniper bullet (`wallpen0` 160) passing one destructible wood crate (`pen_cost` 40) keeps `10000 - floor_div(200000, 160) = 8750` and has 120 wallpen left. The same bullet against a fixed-terrain stone wall (`pen_cost` 120, not destructible) stops or ricochets; against a destructible cracked stone wall it penetrates with 40 wallpen left and keep `10000 - floor_div(600000, 160) = 6250`. A Somnambule auto rifle bullet (`wallpen0` 40) that meets a destructible wood crate at 12,000 mm of flight passes it with 40 wallpen and keep 10000; the same crate at 9,000 mm costs all 40 wallpen and keep `10000 - floor_div(200000, 40) = 5000`; a fixed-terrain wall stops it at any distance.

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
    continue from the contact point; owner_hittable = true
else: stop
```

Example: velocity `(100000, −20000, 0)` mm/s onto a stone floor: `4 × 20000² = 1.6e9 < 1.04e10`, so it ricochets to `(60000, 12000, 0)` mm/s with damage keep 6000.

### 7.6 Hitscan beams

Each pulse casts a ray from the leaned eye along the pulse direction (spread applied per pulse) to `max_range_mm`. The voxel walk (doc 03 `raycast_blocks`) stops at the first cell that is solid and opaque; non-opaque solids (glass, leaves) are passed and take the pulse's block damage when destructible. Capsules are tested along the ray, rewound for player attackers (§2.6). The first entity hit receives the pulse. `beam_streak` counts consecutive pulses on the same target, resetting when a pulse hits nothing or another target; from streak 11 each pulse carries `counter.disruptor`. A fire beam that hits one flammable block for 10 consecutive pulses ignites it (doc 03).

---

## 8. Ammunition

### 8.1 Families

Ammunition is family-generic: any weapon of a family fires any `ammo.` item of that family, and skins rename it (arrows, ferro slugs and lightning motes are all `ammo.draw`). Every ammunition item has an empty dependency set; Contradiction comes from the weapon.

| ID | Used by | Carry cap | Mass per unit (g) | Pickup size | Value per 100 units (Lucre, ilvl 1) | REF units per standard kill |
|---|---|---|---|---|---|---|
| `ammo.light` | pistol, SMG | 240 | 8 | 96 | 20 | pistol 7.45, SMG 16.65 |
| `ammo.heavy` | heavy pistol, auto rifle | 180 | 15 | 64 | 40 | heavy pistol 2.83, auto rifle 11.88 |
| `ammo.rifle` | marksman rifle, sniper | 60 | 25 | 28 | 300 | marksman 5.14, sniper 1.36 |
| `ammo.shell` | shotgun | 48 | 40 | 10 | 200 | 1.40 |
| `ammo.cell` | beam | 300 units | 10 | 60 | 20 | 14.93 (plus §8.4 recharge) |
| `ammo.ordnance` | launcher | 8 | 1500 | 2 | 1500 | 0.91 |
| `ammo.draw` | charge rifle, crossbow | 60 | 30 | 12 | 100 | charge rifle 1.91, crossbow 1.59 |
| `ammo.charge_breaker`, `ammo.charge_disruptor`, `ammo.charge_staggerer`, `ammo.charge_sunder`, `ammo.charge_anchor` | any weapon (§15.3) | 5 each | 100 | 1 | 1400 | — |

These seven families and five counter charges are the whole `ammo.` namespace: flamer fuel is `ammo.cell`, and sling stones are `arch.ranged.thrown` stacks, not ammunition (§5.6). Doc 11 copies these values; a stack's value is `floor_div(units × value_per_100, 100)`, scaled by `P(ilvl)` like every value (doc 11 §13.1). "REF units per standard kill" is `60,000 mHP` divided by the archetype's expected REF damage per projectile or pulse (all hits on the torso, crit expectation included).

```json
{ "id": "ammo.rifle", "category": "cat.ammo", "carry_cap": 60, "mass_g": 25, "pickup": 28, "value_per_100": 300 }
```

Ammunition mass counts toward carried mass (Canon §11.1 `load_bp`). Ammunition in a magazine stays with the weapon.

### 8.2 Pickups

An Erratum within 1000 mm of an ammunition pickup collects it automatically when its reserve for that family is below the cap: it takes `min(pickup, cap - reserve)` and the rest stays as a smaller pickup. Example: reserve 230 `ammo.light` and a 96-round pickup leave reserve 240 and an 86-round pickup.

**Coated ammunition** (doc 08 §10.3). A coat applied to ammunition covers `N = min(coat_units, magazine + reserve)` rounds of the active weapon's family: the magazine's first `min(N, magazine)` rounds, then reserve units, become coated. Coated rounds are a separate sub-stack `{family, coat}` that counts toward the family cap, loads before uncoated rounds and fires first. Each coated round's hits run the doc 08 coat hook (§10.2); for ammunition the coat's charge count is the number of coated rounds.

### 8.3 Drop need (doc 11 interface)

Doc 11 owns drop rolls (keyed `rng.loot`). For each family `f` used by the receiving player's `slot.weapon_1..3`, `need_bp(f) = 10000 - floor_div(reserve_f × 10000, cap_f)`. An ammunition drop picks its family with `weighted(need_bp)`; when every need is 0, no ammunition drops. Doc 05's target supply is one pickup per 4 standard kills (doc 11's standard-rank ammunition line at 2500 Bp). Pickup sizes are set so one pickup covers at least 4 standard kills at 75% accuracy for the hungriest archetype of each family: `pickup ≥ 4 × units_per_kill × 10000 / 7500` gives SMG 88.8 → 96, auto rifle 63.4 → 64, marksman 27.4 → 28, shotgun 7.5 → 10, charge rifle 10.2 → 12 (arrow recovery, §7.2, adds more). The launcher is deliberately ammunition-limited (2 rockets per pickup, cap 8), and the beam lives on its recharge (§8.4). The earlier sizes (32, 24, 8) starved the SMG, auto rifle and marksman rifle at REF accuracy.

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

- `input.throw` with a throwable on the quickbar enters `Cooking(c)` (`move.cook`); `c = 0` on the press tick. The fuse of a fused grenade counts from the press. Release at `c >= 12` throws (`move.throw`); recovery is 18 ticks. From the press through the end of recovery the main-hand weapon accepts no action, and a two-handed weapon is lowered (§4.8a). A throw is a `seq` 0 action start (§1.4).
- A fuse reaching 0 in the hand detonates at `eye + (0, −300 mm, 0)`.
- Thrown weapons that hit an entity stick to it and drop as ground items where it dies. Those that hit a block stop and become ground items at the contact point; walking over one returns it to its stack. Items that fall into lava or leave the Folio are destroyed.
- Fused grenades bounce: `on_block_enter` returns `Bounce` and doc 03 §5.19 reverses the normal velocity component × `bounce_bp` 4000 and multiplies the tangential components by 8000; the grenade rests when its speed falls below 500 mm/s. An entity contact (`on_body_hit` returns `Continue` without queuing a hit) stops horizontal motion. Contact grenades (incendiary) detonate on their first body or block contact.

### 9.3 Grenade archetypes

All throwables use skill `skill.throwing`, scaling SLT 5000, PRC 4000, PWR 3000, base speed 16000 mm/s, gravity 10000, radius 50 mm, precision never, and count as weapon attacks for Canon §6.4 row 12.

| Archetype | Detonation | Fuse (ticks) | `base_mhp` at center | Shares | Radius / inner (mm) / edge Bp | Impact | Status power | `block_power_milli` | Mass (g) |
|---|---|---|---|---|---|---|---|---|---|
| `arch.ranged.grenade_frag` | fuse | 150 | 48000 | pierce 6000, blunt 3000, fire 1000 | 4500 / 1000 / 2000 | 40 | default | 3000 | 400 |
| `arch.ranged.grenade_incendiary` | first contact | — | 20000 | fire 10000 | 3000 / 3000 / 10000 | 0 | burning 60 | 0 | 500 |
| `arch.ranged.grenade_concussion` | fuse | 90 | 15000 | blunt 10000 | 4000 / 1500 / 3000 | 80 | stunned 50 | 1000 | 400 |
| `arch.ranged.grenade_flash` | fuse | 90 | 2000 | radiant 5000, sonic 5000 | 6000 / 2000 / 5000 | 0 | blinded 150, deafened 150 | 0 | 300 |
| `arch.ranged.grenade_smoke` | fuse | 60 | 0 | — | releases `gas.smoke` at concentration 10000 into each of the 27 cells of the 3 × 3 × 3 cube around the center cell, ascending `Gidx`, each capped by the cell's free room (doc 03 §7); doc 08's dissipation removes it | 0 | — | 0 | 300 |
| `arch.ranged.grenade_counter` | fuse | 90 | 12000 | blunt 10000 | 3000 / 3000 / 10000 | 20 | default | 0 | 400 |

- Incendiary grenades also ignite flammable blocks within 2000 mm (doc 03) and place a burning surface. Flash blinding power is multiplied by 10000 when `ang_diff(target.yaw, bearing from target to center) <= 10923`, else by 3000.
- `grenade_counter` carries exactly one counter bit set by its item template (doc 11 `item.throwable.counter_*`), chosen from §15.1.
- Each throwable is one `cat.throwable` item; doc 08 owns alchemical flasks and bombs that reuse these archetypes with liquid payloads. Doc 08 bombs (`alch.bomb.*`) and thrown potion splashes (doc 08 §10.2) are area hits with `NO_CRIT`: they count under `skill.throwing`, draw one keyed `rng.combat` crit roll per target and ignore it, and take friendly fire `FF_AREA_BP` between Errata (Canon §17.5). Doc 04's `volatile_mix` adds +1500 Bp to their step 1 and `catalyst` multiplies their status power by 13000.
- Throwables deal no block damage on contact; their block damage is the blast's, through doc 03 §12.2 with the `block_power_milli` column (§9.4 step 1).

**Table 9.3b — explosion parameters** (doc 03 `ExplosionSpec`; `env_base_mhp` 0 and `drop_bp` 3000 for every row).

| Source | `radius_mm` | `inner_mm` | `edge_bp` | `block_power_milli` | `knockback_mm_s` | `smoke_conc` | `heat_decic` | Ignition | `noise` (doc 09) |
|---|---|---|---|---|---|---|---|---|---|
| `grenade_frag` | 4500 | 1000 | 2000 | 3000 | 8000 | 0 | 0 | none | `noise.explosion.grenade` |
| `grenade_incendiary` | 3000 | 3000 | 10000 | 0 | 0 | 0 | 400 | every flammable block within 2000 mm (`ignite_radius_mm` 2000, `ignite_bp` 10000) | `noise.explosion.incendiary` |
| `grenade_concussion` | 4000 | 1500 | 3000 | 1000 | 14000 | 0 | 0 | none | `noise.explosion.concussion` |
| `grenade_flash` | 6000 | 2000 | 5000 | 0 | 0 | 0 | 0 | none | `noise.explosion.flash` |
| `grenade_smoke` | 1500 | 1500 | 10000 | 0 | 0 | 27-cell rule above | 0 | none | `noise.explosion.smoke` |
| `grenade_counter` | 3000 | 3000 | 10000 | 0 | 4000 | 0 | 0 | none | `noise.explosion.grenade` |
| `launcher` blast | 3500 | 1000 | 2500 | 4000 | 10000 | 0 | 200 | none | `noise.explosion.grenade` |

```json
{ "id": "arch.ranged.grenade_frag", "category": "cat.throwable", "skill": "skill.throwing",
  "fuse_ticks": 150, "base_mhp": 48000, "shares": { "dmg.pierce": 6000, "dmg.blunt": 3000, "dmg.fire": 1000 },
  "radius_mm": 4500, "inner_mm": 1000, "edge_bp": 2000, "impact": 40, "block_power_milli": 3000,
  "knockback_mm_s": 8000, "drop_bp": 3000, "noise": "noise.explosion.grenade",
  "speed_mm_s": 16000, "gravity_bp": 10000, "mass_g": 400,
  "scaling": { "sub.sleight": 5000, "sub.precision": 4000, "sub.power": 3000 } }
```

### 9.4 Explosion resolution and AoE falloff

Every doc 05 explosion (grenades, launcher blasts; doc 07 gadgets and doc 08 flasks and bombs use the same path) is a doc 03 §12 explosion. Doc 05 queues it with `queue_explosion(Explosion { center, spec, source: Item { uid, b_mhp }, instigator: owner, due_tick: t })`:

- `center`: the contact point moved 50 mm back along the projectile's reverse velocity, so it lies in the cell the projectile came from; a fused grenade detonates at its own position; a grenade whose fuse ends in the hand at `eye + (0, −300 mm, 0)`.
- `b_mhp`: the archetype's `base_mhp` × attack factors 1 and 2 (§10.1). Canon §10.4 applies when the hit resolves.
- `spec`: Table 9.3b, with `shares` from Table 9.3.

`sys.explosions` (doc 03 #16) resolves at most 4 due explosions per tick in queue order; a later one waits for the next tick.

```
1. Blocks: doc 03 §12.2's ray method with the spec's block_power_milli (rng.physics, doc 03's draw count);
   only destructible blocks break (Canon §4.3). block_power_milli = 0 skips this step and draws nothing.
2. Entities: doc 03 §12.4 selects every live non-ghost body (and every container or gadget with pools)
   whose target point tp (doc 03 §12.4: the point of its largest-share part's capsule axis nearest the
   center) lies within radius_mm, in ascending EntityId, and computes
   exposure = 2000 × (exposure samples that raycast_blocks reaches from center), five samples.
   The falloff is doc 03 §12.4's curve, read with this spec's inner_mm and edge_bp:
       dist_mm    = mm(dist(center, tp))
       falloff_bp = if dist_mm <= inner_mm { 10000 }
                    else { 10000 - floor_div((10000 - edge_bp) × (dist_mm - inner_mm), radius_mm - inner_mm) }
       if exposure == 0: no hit
       B_eff = mul_bp(mul_bp(b_mhp, falloff_bp), exposure)                 // attack factor 7 (§10.1), one mul_bp each
                                                                           // (Canon §10.4); b_mhp already carries factors 1 and 2,
                                                                           // and factors 3 to 6 never apply to blasts
       push HitRecord { kind: Area, part: largest-share part (ties: first listed), base_mhp: B_eff,
                        flags: AREA (+ SEVER for a launcher blast with dist_mm <= inner_mm) (+ NO_CRIT for bombs and thrown potion splashes),
                        impact × falloff_bp × exposure, status powers × falloff_bp × exposure,
                        incoming = center − q, counters: the item's }      // q = p + (0, h/2, 0)
       apply_knockback(e, normalize(q − center + (0, 500 mm, 0)), mul_bp(knockback_mm_s, mul_bp(falloff_bp, exposure)))   // doc 03 §12.4
3. The owner and allies are included. Friendly fire applies FF_AREA_BP at step 2.15 (Canon §17.5),
   so self-damage is full by default and halved in mode.ff_off.
```

Doc 03 §12 governs explosions, their target points, exposure samples and falloff curve; doc 05 supplies `inner_mm` and `edge_bp` (triage G-049). An `ExplosionSpec` that omits the two fields takes `inner_mm = floor_div(radius_mm, 4)` and `edge_bp = 2500` (doc 03's powder keg carries them explicitly). When `inner_mm = radius_mm` the second branch is unreachable, because targets beyond `radius_mm` are never selected.

Area hits roll crits (one draw each, §1.4; a `NO_CRIT` hit draws and ignores it) and never count as precision. They take step 2.10's × 15000 against `body.swarm`. Only tower shields block area hits (§12.1). A launcher blast within its inner radius carries `SEVER`.

---

## 10. The damage pipeline in full

Canon §10.5 fixes the step order. This section adds the doc-05 inputs and rules at each step and changes nothing in the canonical order.

### 10.1 Attack factors (before step 0)

Canon §10.4 places these factors before its source-magnitude chain. A hit's `base_mhp` is the archetype's `base_mhp` (NPCs: the template's `template_dmg_mhp`) multiplied, one `mul_bp` each and in this order, by every factor that applies:

1. move factor: combo step `move_bp`, `heavy_bp` (§4.2), sprint 13000, jump (§4.4), riposte 25000, off-hand 8500;
2. `charge_bp` (charge rifle);
3. whip sweet-spot 6000 (§3.2);
4. range falloff (§7.2);
5. `keep_bp` from walls, bodies and ricochets (§7.3–§7.5);
6. cleave factor 10000 / 7500 / 5000 (§2.3);
7. area falloff, then exposure, one `mul_bp` each (§9.4).

The former mounted-charge factor is retired with mounts (Canon §17.7). The result then enters Canon §10.4 (`× P(src_lvl) × rarity × quality × enhancement × material` for Errata items; `× T(d) × rank damage` for enemies). After that chain, a manufacturer trait stated as "× N" (Canon §5.9; §10.5) multiplies `B` once, before step 1.

- **Impact** uses the move's impact factor in place of factor 1 (combo steps and heavies: their `move_bp` or `heavy_bp`; sprint attack 15000; jump attack 20000; riposte 0; off-hand 8500; `charge_bp` for the charge rifle replaces factor 2), then factors 3 to 7.
- **Status power** (§13) uses factors 1 to 7, so buildup per second stays proportional to damage per second at every range and cleave position.

### 10.2 Reference implementation

```rust
fn resolve_hit(h: &HitRecord, w: &mut World) -> Option<DamageEvent> {
    let (att, tgt) = (w.ent(h.owner), w.ent_mut(h.target));
    if tgt.dead { return None; }                        // discarded with no event (§1.3 C6)
    if h.entry != 0 { return resolve_entry_point(h, w); }   // Canon §10.6 rows: no crit draw (§10.2 "Entry points")
    // Canon §3.4: every non-sneak hit draws exactly one crit roll, whatever happens next
    let sneak = doc09::sneak_eligible(att, tgt, w.t);   // per (attacker, target) on the impact tick
    let crit = !sneak && keyed_rng("rng.combat", folio, h.source, w.t, h.seq).roll_bp(crit_chance(att))
               && !h.flags.no_crit();                   // a NO_CRIT hit draws and ignores the roll
    if tgt.has(st_deaths_door) || tgt.dodge_invulnerable(w.t) { return Some(emit_negated(h)); }  // flag negated
    // Parry and block are decided from target state at tick t (§2.6, §12.2)
    let guard = guard_outcome(h, tgt, w.t);            // None | Blocked | Parried
    if guard == Parried { apply_parry(h, w); return Some(emit_parried(h)); }    // ev.damage_dealt, every type 0, flag parried
    let b = source_magnitude(h);                        // §10.1, then Canon §10.4, then "× N" traits (§10.5)
    // Step 1: additive bucket
    let mut add = if h.is_weapon_attack() && att.is_erratum() { row12(att, h.item) } else { 0 };
    add += Σ mul_bp(bonus_i, potency_i);  add = max(add, -9000);
    let mut x = mul_bp(b, 10000 + add);
    // Step 2: chain, each one mul_bp, in canonical order
    x = mul_bp(x, h.spell_pot_bp);                                                          // 2.1 spells and prayers (doc 06); else 10000
    if !h.area() && tgt.part(h.part).precision { x = mul_bp(x, h.precision_bp); }            // 2.2
    if crit { x = mul_bp(x, crit_mult(att)); }                                               // 2.3
    if sneak { x = mul_bp(x, doc09::sneak_mult(att, tgt, h.sneak_tier)); }                  // 2.4
    x = mul_bp(x, elevation_bp(att, tgt));                                                   // 2.4a §10.3
    x = mul_bp(x, flank_bp(att, tgt));                                                       // 2.4b §10.3
    x = mul_bp(x, potency(h.item_uid, att));                                                 // 2.5
    if is_native(tgt) { x = mul_bp(x, 10000 + edge_bp(h.item_uid, att)); }                   // 2.6
    if tagged_mythic(att) || tagged_mythic(tgt) { x = mul_bp(x, 10000 + (myt - 5) * 300); } // 2.7, myt = local MYTHOS at att.pos
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
    // Steps 3–6 exactly as Canon §10.5; step 5's AR_part adds the shield's AR when guard == Blocked (§12.2)
    let mut d = split_by_type(d2, &h.shares);                  // step 3, remainder to largest share
    per_type_target_mods(&mut d, h, tgt);                       // 3a: radiant vs undead/fiend; frozen blunt; fae iron
    apply_resistance(&mut d, tgt);                              // step 4
    if !h.flags.ignore_armor() { apply_armor(&mut d, ar_for(tgt, h.part, guard), h.pen_bp); }  // step 5
    apply_chip_floor(&mut d, d2, tgt);                          // step 6
    let mut stamina_loss = 0;
    if guard == Blocked { stamina_loss = apply_block(&mut d, h, tgt); }                      // 6b, §12.3
    if tgt.is_erratum() && tgt.slot().mercy_on { apply_mercy(&mut d, tgt.slot().mercy_bp); } // 6c Scribe's Mercy (doc 17)
    let ward_lost = apply_ward(&mut d, h, tgt);                                              // step 7: per-pool m[t], §15.1, §15.4
    let total: i64 = d.iter().sum();
    apply_hp_condition_poise_buildup(h, tgt, total, guard, w);                               // step 8, §12.4, §12.5, §13
    if total + ward_lost > 0 { doc08::coat_on_hit(h, tgt, w); }                             // doc 08 §10.3: landed hits only
    shock_of_the_new(h, tgt, w);                                                             // Canon §5.6; never ranks boss, editor, hunter
    apply_counters(h, tgt, ward_lost, w);                                                    // §15.1
    check_sever(h, tgt, w);                                                                  // §19.1
    resolve_lethal(tgt, w);                                                                  // Canon §10.8
    Some(emit_damage_dealt(h, &d, ward_lost, stamina_loss))                                  // step 9
}
```

- **Step 2.12** (Canon §10.5). The attacker statuses in ascending ID order are `st.berserk` 13000, `st.empowered` 12000, `st.taunted` 7500 when the target is not the taunter, and `st.weakened` 7500.
- **Step 2.11.** `st.exposed` `10000 + 800 × stacks`, then `st.knocked_down` 11000.
- **Step 3a, fae iron** (Canon §10.5 step 3a, §16.3). Every `d[t]` × 12500 when the target is `kin.fae` and the hit's `item_uid` names an item whose primary material is `mat.iron` or `mat.cold_iron`. A hit with no `item_uid` (bare hands, NPC template attacks) never qualifies.
- **Step 5 on a blocked hit** (Canon §10.5 step 5). `AR_part` is the hit part's AR plus the blocking shield's AR (§12.1, scaled with `T(ilvl)`); a weapon guard adds 0 (§12.2).
- **Step 6b** (Canon §10.7). The blocked remainder is split back over the types as §12.3 states, so step 7 sees per-type values.
- **Step 6c, Scribe's Mercy** (Canon §10.5, doc 17). When the target is an Erratum whose slot has `mercy_on`, each `d[t] = d[t] - mul_bp(d[t], mercy_bp)` in Canon §10.1 order. It applies to every involuntary HP loss of that Erratum, entry-point hits that enter at step 3 or 4 included; HP the Erratum chooses to pay is never reduced.
- **Step 7** (Canon §10.5). The `st.shielded` pool is consumed first with `m[t] = ward_mult[t]`, then the ward generator's ward with `m[t] = mul_bp(ward_mult[t], profile[t])` (§15.4); a `counter.ward_breaker` hit multiplies `m[t]` by 30000 for every pool (§15.1). An NPC's `elite.warded` pool takes the generator's place with `m[t] = ward_mult[t]` (§15.2).
- **Entry points** (Canon §10.6). A hit with `entry > 0` is one of Canon §10.6's other entry points. It draws no crit value (triage G-096), and `resolve_entry_point` runs the pipeline from that row's entry step with that row's skips and flags, then the remaining steps of `resolve_hit` through step 9 and lethal resolution. DoT pulses and falls enter at step 4. Environmental hazards, thorns, shock chains and a consumable's damage to its own drinker or injected target enter at step 3; the consumable (doc 08) has `D2 = B` and skips steps 1 and 2, so it takes no additive bucket, crit or friendly fire. A lethal takedown (doc 09) enters at step 7 as `dmg.true` equal to the target's HP plus ward, with flag `TAKEDOWN`, and skips crit, resistance, armor and the chip floor.
- **Crit chance and multiplier** are Canon §6.4 rows 13 and 14 for Errata. NPCs use row 13 without gear and multiplier 15000.
- **Step 9.** `ev.damage_dealt` carries per-type HP damage, `ward_lost`, `stamina_loss_mst` (blocked hits), `d_armor` (doc 04 Armor XP), `crit`, `precision`, `sneak`, `blocked`, `parried`, `negated`, `kind`, `counters`, and the hit part.
- **Parried hits** emit `ev.damage_dealt` with every type 0 and `parried = true`, so docs 04 (Shield XP), 17 and 18 see the parry; they apply nothing else.
- **Negated hits** (dodge invulnerability, `st.deaths_door`) emit `ev.damage_dealt` with every type 0 and `negated = true`; doc 04's Acrobatics XP sums their `base_mhp`. Parried and negated hits draw their crit roll and no on-hit component rolls.
- **`NO_CRIT`** hits still draw their crit roll (Canon §3.4) and ignore the result. Doc 04's Marksmanship 100 ("the first shot after 60 ticks without firing crits") likewise draws and replaces the result.
- **Sonic hits** emit `ev.noise` at `60 + floor_div(d_mhp, 1000)` dB at the impact point and end `st.frozen` without the step 3a × 15000, which Canon §10.5 grants to blunt only.
- **Law terms.** Doc 12's `law.weird_west.high_noon` standoff bonus (+5000 Bp) enters step 1 as a "+% damage" term whose potency is 10000; its direct `st.staggered` applies after step 8.
- **Shock of the New** (Canon §5.6) applies to each native target whose rank is not `rank.boss`, `rank.editor` or `rank.hunter`.

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

The first hit on each native target whose rank is not `rank.boss`, `rank.editor` or `rank.hunter` applies `st.staggered` (Shock of the New, Canon §5.6). Firing rolls a 1000 Bp jam on the first shot of each trigger hold and then once per 30 ticks (§16.2).

**E5 — Armor against SMG and sniper (Folio 1, Common).** SMG D1 = 4,770 per bullet, `pen_bp` 0; sniper D1 = 43,672, `pen_bp` 5000.

| Target AR | SMG effAR / DR / per bullet | Sniper effAR / DR / per shot | SMG REF DPS | Sniper REF DPS |
|---|---|---|---|---|
| 10 | 7 / 2,684 / 3,490 | 3 / 168 / 42,939 | 30.3 | 24.0 |
| 40 | 30 / 6,112 / 1,855 | 15 / 790 / 40,222 | 16.3 | 22.5 |

Armor removes 61% of each SMG bullet at AR 40 and 8% of the sniper shot; against armored enemies the sniper overtakes the SMG.

**E6 — Blocked hit (NPC on an Erratum with a heater shield, no ward generator).** REF enemy hit `template_dmg_mhp` 10,000 slash at Folio 1, party of 2 (× 11000): D2 = 11,000. Torso AR 20 plus heater AR 25 = 45: `DR = floor(4.5 × 10^8 / 89,000) = 5056`, leaving 5,439. Block stability `5000 + 0 (BRN 40) + 2500 (heater) + 600 (Shield 20, §12.2) = 8100`: `to_stamina = mul_bp(5439, 8100) = 4,405`, **HP loss 1,034**, stamina loss `floor_div(4405 × 10000, T(1)) = 4,405` mst. Unblocked, the same hit deals 7,563. With Scribe's Mercy on at `mercy_bp` 2000, step 6c leaves the blocked hit at `1034 - mul_bp(1034, 2000) = 828` mHP; the stamina loss is unchanged.

**E7 — Warded champion and `counter.ward_breaker`.** A Folio-1 champion with template HP 60: Max HP `60,000 × H(1) × 60000 / 10000 = 360,000` mHP; `elite.warded` gives ward 180,000. A Common sniper torso shot with a breaker charge: after armor 42,939 pierce. Step 7 uses `m[pierce] = mul_bp(10000, 30000) = 30000`: `w = 128,817 <= 180,000`, so the ward drops to 51,183 and HP takes 0. Second shot: `w = 128,817 > 51,183`, HP takes `42,939 - floor_div(51,183 × 10000, 30000) = 25,878`, `ev.ward_broken` fires and the counter applies `st.stunned` directly: `mul_bp(90, clamp(floor_div(10^6, 100), 2500, 15000)) = 90` ticks, × champion control 5000 = **45 ticks**. Without the counter the first shot leaves ward 137,061 and the ward needs 5 shots.

**E8 — Frag grenade and friendly fire.** Center 48,000; radius 4,500, inner 1,000, edge 2000; every target has all five exposure samples visible (exposure 10000). An NPC whose target point (doc 03 §12.4) is 2,500 mm from the blast center: `falloff = 10000 - floor_div(8000 × 1500, 3500) = 6572`, base 31,545, D1 = 33,437, split pierce 20,063 / blunt 10,031 / fire 3,343; after AR 10, **30,717 mHP**. An allied Erratum at 3,500 mm in a full Normal medium set (AR 20, fire resistance +1000 from Table 11.1c): falloff 4286, base 20,572, D1 21,806, `FF_AREA_BP` 10000, split pierce 13,085 / blunt 6,541 / fire 2,180; resistance leaves fire 1,962; armor leaves pierce 10,171 and blunt 4,733: **16,866 mHP**. In `mode.ff_off` (5000) D2 is 10,903 and it takes 6,995.

**E9 — Elevation and flanking (pistol, 30 m, 3 m above, from the flank).** Falloff 6250: base 5,375; D1 = 5,697; elevation × 11000 → 6,266; flanking × 11500 → 7,205; AR 10 with `pen_bp` 1000 gives effAR `mul_bp(7, 9000) = 6`, DR 1,723. **Total 5,964 mHP.**


### 10.5a Curse and pact combat terms (doc 10 §7.3, §7.8)

Combat effects of equipped cursed items and pacts scale with item potency `m` (doc 10 §7.3, §7.8):
- **Resistances**: `curse.brittle` subtracts `m` Bp from the bearer's physical resistances (`dmg.slash`, `dmg.pierce`, `dmg.blunt`) before the Canon §10.5 step 4 clamp.
- **Additive bucket (step 1)**: `curse.blunted` adds `-m` Bp; `curse.pact.fury` adds `+m` Bp while bearer HP ≤ 50% Max HP; `curse.pact.executioner` adds `+m` Bp against targets below 25% Max HP.
- **Critical hits**: `curse.dulled` gear term subtracts `m` Bp from crit chance; `curse.pact.nightmare` subtracts `m` Bp from crit chance; `curse.pact.glass` adds `+m` Bp to crit damage multiplier (`crit_mult`).
- **Failure chance**: `curse.misfire` and `curse.pact.wildfire` sum their `m` Bp failure chance into the item's Contradiction activation roll.
- **Direct true damage (step 4 entry)**: `curse.bloodprice` (on weapon swing), `curse.backlash` (on hit taken) and `curse.pact.thorns` (reflected hit on attacker) queue `dmg.true` hits entering at Canon §10.6 step 4.
- **Poise**: `curse.pact.bulwark` adds `+m` mp to max poise.
- **Healing**: `curse.pact.bloodthirst` heals `m` Bp of damage dealt on lethal hits.
- **Equipment lock**: weapon switching is locked while the active weapon is bound by a curse (`buc.cursed`).

### 10.5 Manufacturer trait hooks (Canon §5.9)

Doc 11 owns every manufacturer trait's mechanics; doc 05 provides the hooks (triage G-065). Most traits need only these general hooks (doc 11 §5.1): a trait stated as "× N", such as `mfr.hardboiled_noir.rainslick` and `mfr.fairy_tale.thimbleweald`, multiplies `B` once after Canon §10.4 and before step 1 (§10.1); a "+N Bp" trait is a step 1 "+% damage" term; a trait that moves part of a hit to another type edits `HitRecord.shares` before step 3; a chance per shot or swing is a `seq` 0 component and a chance per hit an on-hit component at `16 + i` (§1.4); a status a trait applies directly applies after step 8, as counters do (§15.1); and damage a trait sends to another entity (ricochets, chains, reflections) is a Canon §10.6 entry-point hit in the second batch (§2.7). Homing (`mfr.cyberpunk.ghostline`) turns a projectile's velocity inside doc 03 §5.19's integration, and noise traits act on doc 09's levels. The traits below need a hook of their own.

| Trait | Doc 05 hook |
|---|---|
| `mfr.mythic_fantasy.deepanvil` | weapons: attack speed × 9000 joins §6.2's `atk_speed_bp` after the shortfall factor, and poise damage × 15000 joins §12.4 after the blunt factor, one `mul_bp` each; armor pieces: doc 11's +4 joins §11.2's per-piece `poise` before its chain |
| `mfr.gothic_horror.grimhallow` | a reload never enters `Reloading`: on the tick §6.1 would start one, the rounds move from reserve to magazine at once (the shotgun loads every shell), the weapon stays `Ready` and doc 11 charges the HP; when doc 11 refuses the cost, no reload starts. A melee heavy spends doc 11's HP cost in place of §4.2's stamina |
| `mfr.space_opera.perihelion` | a "× N" trait: the charged shot's × 15000 multiplies `B` once after Canon §10.4 and before step 1 (§10.1) |
| `mfr.space_opera.umbra` | the weapon has no magazine and spends no ammunition: §6.1 treats its magazine as always holding `ammo_per_shot` and never enters `Reloading`; a shot that brings doc 11's heat to 10000 moves the machine to `Stalled(0)` with `stall_total` 90 instead of `Ready`, and heat is 0 when `Stalled` ends |
| `mfr.cyberpunk.neon_saint` | §7.3 step 2: a charged shot (doc 11) passes the first penetrable cell it enters without spending penetration, whatever its flight distance; fixed terrain stops it |
| `mfr.post_apocalypse.rustmaw` | magazine × 20000, and every reload duration of the weapon (§6.1 `reload_ticks`, the shotgun's 20-tick start and 32-tick per-shell interval, the 18-tick nock) × 13000, one `mul_bp` each before `action_ticks` |
| `mfr.post_apocalypse.lastditch` | its per-shot and per-swing chance component draws at `seq` 0 in ascending ID (§1.4) |
| `mfr.wuxia.jade_lotus` | `input.throw` with such a melee weapon active and no throwable on the quickbar throws the weapon itself: a `move.throw` with §9.2's 12-tick minimum wind, 18-tick recovery and `seq` 0 action start, flying with `arch.ranged.thrown`'s Table 5.2b ballistics at §9.1's speed; its hit uses the weapon's own `base_mhp`, shares, `pen_bp`, impact and Canon §10.4 chain with move factor 10000, and it rolls Contradiction as a throw (§16.3 dud). While it is away the melee machine is `Holstered` and `input.attack` performs `arch.melee.unarmed` attacks; exactly 30 ticks after release it returns to the hand wherever it is and enters `Drawing(0)` |
| `mfr.mythic_antiquity.hephaesteum` | the bearer is immune to `st.disarmed` and `st.corroded`, buildup and direct application alike, so `counter.sunder` cannot corrode it (§15.1) |
| `mfr.oneiric.somnambule` | §7.3 step 2: after 10000 mm of flight its projectiles pass destructible and non-opaque cells without spending penetration; fixed terrain stops them |
| `mfr.redacted.bureau` | `failure_bp = 0`; the potency loss still applies (§16.1) |

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

**Insulation** (doc 04 comfort band, DeciC per piece, every slot alike): cloth `insul_cold` 10, `insul_heat` 10; light 20, 0; medium 25, −5; heavy 30, −10. A full heavy set lowers the cold bound by 150 (15.0 °C) and lowers the heat bound by 50 (5.0 °C), because doc 04 adds `insul_heat` to the upper bound.

```json
{ "id": "arch.armor.body_medium", "category": "cat.armor", "slot": "slot.body", "weight_class": 2,
  "base_ar": { "bp.torso": 20, "bp.arm_l": 8, "bp.arm_r": 8 },
  "poise": 6, "mass_g": 8000, "noise_db": 2, "insul_cold": 25, "insul_heat": -5,
  "resist": { "dmg.fire": 200, "dmg.acid": 200 } }
```

Masks, goggles and respirators for `slot.face` are not armor archetypes: doc 08 owns respirators (`apparatus.respirator.*`) and doc 11 the face accessories.

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
- Skill XP: doc 04 §5.2 owns the row. Each hostile hit taken on a part with AR > 0 grants `min(1000, floor_div(2000 × d_armor, own_max))` milli-XP to `skill.armor`, where `d_armor` is the hit's share of armor-affected types (slash, pierce, blunt, sonic) after step 4 and `own_max` the bearer's Max HP in mHP. Doc 05 reports `d_armor` in the `ev.damage_dealt` payload.

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
| `arch.offhand.tome` (§5.4) | — | — | — | — | — | 900 | — | — |

- Shield AR uses the armor chain with `T(ilvl)` (Canon §13.8 "shield block values") and is added to the hit part's AR at step 5 of a blocked hit (Canon §10.5 step 5).
- Off-hand items that cannot block (lantern, focus, tome) leave `input.alt` free for ADS with a one-handed ranged weapon. A crutch (`item.tool.crutch`, doc 11) also occupies `slot.offhand`, cannot block, and blocks two-handed weapons; doc 04 applies its leg factor.
- The lantern emits `light_block` 13 at its holder (doc 03 propagation) and cannot block. `arch.offhand.focus` is the casting focus whose casting behavior doc 06 owns; it cannot block.
- **Shield bash** (`move.shield_bash`, `input.heavy` while guarding with a shield): W 5 / A 3 / R 16, 10 stamina, blunt 10000, reach 1000, arc 4551, `block_bp` 5000, skill `skill.shield`, scaling BRN 6000, PWR 4000, GRP 2000, taunt power 40 (§19.2). Heater and tower bashes carry `counter.staggerer`.
- **Parryable projectiles.** Melee hits carry `PARRYABLE` unless an NPC template marks the move unparryable; projectile and hitscan hits carry it only against a bearer at doc 04 Shield 100, whose parry returns the hit's damage to the shooter through the Canon §10.6 thorns entry.

### 12.2 Guard outcome

The guard is effective in `Guarding` (§4.8). For a hit with incoming vector `i` (from the target toward the attack origin, the projectile's reverse velocity, or the explosion center):

```
bearing = atan2_bam(-i.x, -i.z)
in_arc  = ang_diff(target.yaw, bearing) <= guard_arc
blockable = melee | projectile | hitscan | (area && tower shield);  never DoT, surfaces, falls, unblockable NPC moves
outcome = Parried  if in_arc && blockable && PARRYABLE && k < parry_ticks
        = Blocked  if in_arc && blockable
        = None     otherwise
guard_arc   = archetype arc (+4096 at doc 04 Shield 75, shields only)
parry_ticks = archetype parry ticks + floor_div(skill.shield, 25) for bucklers and heaters (doc 04 Shield);
              + 3 for blade weapon guards at doc 04 Blade 50; the tower shield stays 0
stability_bp = clamp(5000 + d(BRN) × 50 + shield_stab, 0, 10000)       // Canon §6.4 row 19
shield_stab  = shield stability + 30 × skill.shield   (shields) | weapon block stability (Table 3.2c)
stability_bp = mul_bp(stability_bp, 5000) if the attack is a chain weapon
AR used at step 5 = AR(part) + AR(shield)  (Canon §10.5 step 5; weapon guards add 0)
```

### 12.3 Step 6b distribution

Canon §10.7 converts `to_stamina = mul_bp(total, stability_bp)` into stamina loss `floor_div(to_stamina × 10000, T(d_current))`. The remaining HP damage is split back over the types as Canon §10.7 states: `new_total = total - to_stamina`, then `d[t] = floor_div(d[t] × new_total, total)` for each type in Canon §10.1 order, with the remainder `new_total - Σ d[t]` to the largest-share type (ties to the earlier §10.1 row), so step 7 sees per-type values. A `GUARD_BREAK` hit multiplies the stamina loss by 20000. Blocked hits deal no poise damage and half buildup (Canon §11.2). When the blocker's stamina reaches 0 it is guard-broken: `st.staggered` (30 ticks) and a guard lockout of 60 ticks during which no raise starts. Stamina lost to a blocked hit counts as spending for doc 04's regeneration delay (60 ticks without spending), so a raised guard does not regenerate stamina while it is being hit. Doc 04's Shield 25 multiplies this stamina loss by 8500 after the `GUARD_BREAK` factor. NPC guards (doc 13 templates supply stamina, `block_stability_bp` and arc) follow the same rules with `T(d_current)`.

### 12.4 Poise damage (step 8)

```
pd = impact_mp                                     // impact × 1000 × §10.1 factors
pd = mul_bp(pd, S(src_lvl))                        // Canon §10.5 step 8; enemies S(d); S's argument capped at 300 (canon-1.5)
pd = mul_bp(pd, 10000 + floor_div(share_blunt, 2))  // Canon §10.1 dmg.blunt row: × 15000 for a pure blunt hit
pd = mul_bp(pd, mfr_poise_bp)                       // §10.5: Deepanvil 15000, else 10000
pd = mul_bp(pd, 30000)          if counter.staggerer
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
p_t = floor_div(POWER10[t] × base_mhp_status × share[t], 10000 × 10000)    // base_mhp after §10.1 factors 1 to 7
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

Doc-05 status behavior beyond Canon §11.1:

- `st.disarmed` puts the active weapon into `Disarmed` (§4.8, §6.1) without dropping it.
- `st.slowed` and `st.chilled` multiply stance base speed through the status step; `st.chilled` also multiplies `atk_speed_bp` (§6.2).
- `st.corroded` multiplies AR per Canon §10.3; `st.exposed` stacks at step 2.11; `st.knocked_down` applies × 11000 at step 2.11 and is a disabling status.
- `st.shocked`: on each active tick where `(t - a) % 30 == 0` the bearer is micro-stunned for 6 ticks, during which it counts as carrying a disabling status (§4.8): a swing in progress ends and both weapon machines enter `Staggered` for those ticks. Each hit it takes chains per Canon §11.5.
- `st.stunned`, `st.frozen`, `st.staggered` are disabling statuses; the next blunt hit on a frozen bearer takes × 15000 at step 3a and ends the freeze; a sonic hit ends it without the multiplier (§10.2).
- `st.burning` ignites flammable blocks in the bearer's feet and body cells through doc 03 fire on each status pulse, and ends when the bearer is in contact with `liq.water` (Canon §12.1).
- `st.blinded`: spread × 20000 (§6.3); doc 09 owns its perception effects.
- `st.bleeding` ends on a bandage or on a single heal of at least 10% of Max HP (Canon §11.1); doc 04 owns bandages.

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

Every AI think (6 ticks) each positive entry loses `max(1, floor_div(entry × 30, 10000))`; an entry that reaches 0 is removed. Target choice (doc 13 owns it, with this hysteresis): `eff(e) = threat(e)`, then × 13000 when `e` is within 3000 mm of the NPC, then × 2500 when `e` is downed, one `mul_bp` each in that order; ghosts are excluded. The NPC switches from its current target `c` to the entry with the largest `eff` (ties to the lower EntityId) only when that `eff > mul_bp(eff(c), 11000)`. A minion's threat is credited to the minion. Example: 10,000 mHP of melee damage at MEN 50 adds `mul_bp(mul_bp(10000, 11000), 12000) = 13,200`.

### 14.2 Friendly fire (Canon §17.5)

- Step 2.15 applies when attacker and target are both `fac.errata`, minions included. `FF_DIRECT_BP` covers melee, projectiles, hitscan, quick melee and bashes; `FF_AREA_BP` covers explosions, shockwaves, fire spread, surfaces, falling blocks and shock chains. Self-hits from one's own explosions are area friendly fire.
- Allies are valid melee and projectile targets and consume cleave and pierce. Friendly hits never add threat, never trigger Shock of the New, never parry or riposte, and grant no combat skill XP.
- `st.charmed` on a player uses Canon §11.8.

### 14.3 Downed and death hooks

- Canon §10.8 step 2 applies `st.downed`; doc 16 §9.4 owns the state. While downed, an Erratum crawls at 800 mm/s base (doc 03 §5.12, doc 16), keeps its capsules, and cannot attack, block, cast, reload, use items, interact, sprint, jump or dodge (§4.8a). Its bleed-out `bleed_left` starts at `max(600, 1800 − 600 × (downs_folio − 1))` ticks: 30 s for the first down in a Folio (Canon §11.1), later downs per doc 16 §9.4.
- Damage to a downed Erratum runs the full pipeline with HP held at 0; each resolved hit, DoT pulses included, shortens `bleed_left` by `ceil_div(total_mhp × 1800, max_hp_mhp)` ticks (17,000 mHP at Max HP 85,000 removes 360 ticks). At 0 remaining ticks Canon §10.8 step 3 runs.
- `ev.entity_killed` drops stuck thrown weapons and arrows as ground items (`floor_div(n, 2)` of `n` arrows survive; thrown weapons always survive) and releases grapples and taunts the entity held.

---

## 15. Champion counters, elite aspects and ward generators

### 15.1 Counter types

`counter.*` is a bitmask on a hit (`counters`), bit order as listed.

| Bit | ID | Effect on a hit carrying it |
|---|---|---|
| 0 | `counter.ward_breaker` | step 7 multiplies `m[t]` by 30000 for every ward pool of the target (Canon §10.5 step 7: `m[t]` is `ward_mult[t]` for `st.shielded` and an `elite.warded` pool, `mul_bp(ward_mult[t], profile[t])` for a generator, §15.4); if this hit breaks a pool, apply `st.stunned` directly (Canon §11.4 duration × rank control) |
| 1 | `counter.disruptor` | apply `st.unraveled` and `st.silenced` directly for exactly 180 ticks each, unscaled by Canon §11.4 (Canon §17.3a); the target's regeneration (natural, `elite.surging`, template regen) is 0 while either lasts |
| 2 | `counter.staggerer` | poise damage × 30000 (§12.4); can stagger targets immune to poise stagger |
| 3 | `counter.sunder` | apply one stack of `st.corroded` directly (direct application is not buildup, so `elite.armored`'s buildup immunity does not stop it; a `mfr.mythic_antiquity.hephaesteum` bearer stays immune, §10.5); at 5 stacks a target with `elite.armored` loses that modifier for 300 ticks |
| 4 | `counter.anchor` | apply `st.rooted` directly for 120 ticks; a target with `elite.phasing` cannot phase for 300 ticks |

Counters act on any target that a hit carrying them reaches, Errata included: NPC templates and the invader's Editor kits (doc 13) may carry counter bits, and a ward-broken Erratum is stunned like a champion. Counter effects never draw random numbers.

### 15.2 Elite aspects (Canon §17.3a) and two elite modifiers

| Aspect or modifier | Exact behavior | Counter |
|---|---|---|
| `elite.warded` (champion aspect) | NPC ward pool = `mul_bp(max_hp_mhp, 5000)`; refills fully 180 ticks after the champion last took damage; depletes per Canon §10.5 step 7 in the generator's place, with `m[t] = ward_mult[t]` | `counter.ward_breaker` |
| `elite.surging` (champion aspect) | attack and cast speed × 13000; regenerates 200 Bp of Max HP per second (accumulator `k = 60`) unless it has `st.unraveled` or `st.shocked` | `counter.disruptor` |
| `elite.unstoppable` (champion aspect) | immune to `st.staggered` and `st.knocked_down` from poise | `counter.staggerer` |
| `elite.armored` (doc 13 §7.2 elite modifier) | every part's AR × 25000; immune to `st.corroded` buildup | `counter.sunder` |
| `elite.phasing` (doc 13 §7.2 elite modifier) | every 240 ticks becomes intangible for 60 ticks (no capsule tests, so every query of §2.1 skips it) and blinks up to 6000 mm to a walkable cell behind its target after a 30-tick telegraph (doc 13) | `counter.anchor` |

**What each elite requires of the party.** Warded: deplete a ward of half the champion's Max HP before HP moves, or break it at once with ward-breaker hits (× 30000 depletion) and gain a stun. Surging: land a disruptor hit, or keep it `st.shocked`, to stop its 2% Max HP per second regeneration. Unstoppable: only staggerer hits (× 30000 poise, ignoring its immunity) stagger it. Armored: its AR is 2.5 times normal; five sunder hits strip the modifier for 300 ticks, and every sunder stack also corrodes 10% of its AR. Phasing: an anchor hit roots it for 120 ticks and blocks phasing for 300 ticks, so attacks connect.

### 15.3 Delivery channels

| Channel | Counters | Rule |
|---|---|---|
| Counter charges `ammo.charge_*` (dependency none) | each charge one type | Use action `move.imbue`, `action_ticks(45)`; imbues the active weapon: ranged for the next `max(6, magazine)` shots (beams 60 pulses), melee for the next 12 hits; a new imbue replaces the old one |
| `arch.ranged.grenade_counter` (dependency none) | one type per item template | every area hit carries it |
| Archetype innates | ward_breaker: sniper and crossbow precision hits, charge rifle full charge; disruptor: beam streak ≥ 11, whip full heavy; staggerer: launcher direct hit, warhammer and great axe full heavy, warhammer jump attack, heater and tower bash | always on |
| Perks and affixes (doc 11) | any | the item template sets `innate_counter` bits |

Doc 11 stocks at least one counter charge of every type and one `grenade_counter` of every type in every genre's loot pool and in every shop; `genre.unwritten`, which has no shops, also gets doc 11's counter caches (Canon §17.3a).

### 15.4 Ward generators (`slot.ward`, `cat.ward_gen`)

```
capacity_mhp = cap_base_mhp → × T(ilvl) → × rarity → × quality → × (10000 + 600 × eff_enh) → × potency   // Canon §13.2, no material term
recharge: starts 180 ticks after the bearer last took damage (any ev.damage_dealt with the bearer as target,
          DoT included, ward_lost or HP > 0); restores mul_bp(capacity, 2500) per second (accumulator k = 60)
flicker:  when the generator's base function has a failure chance, the tick the recharge would start draws one
          roll_bp at seq 3 (§1.4); a failure delays the start by another 180 ticks, then rolls again
```

Canon §10.5 step 7 reads the depletion profile below: for the generator pool `m[t] = mul_bp(ward_mult[t], profile[t])`; the `st.shielded` pool always uses `m[t] = ward_mult[t]`.

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

Examples: an aegis at ilvl 10 holds `mul_bp(20000, 16190) = 32,380` mHP and recharges 8,095 mHP per second. A kinetic ward of 10,000 mHP hit by 8,000 mHP of shock uses `m[shock] = mul_bp(15000, 15000) = 22500`: `w = 18,000 > 10,000`, so HP takes `8000 - floor_div(10^8, 22500) = 3,556` and the ward breaks. With the aegis profile the same hit leaves 1,334, matching Canon §21 test 41.

---

## 16. Contradiction failures: jams and clearing

### 16.1 Scope

Only the base-function component rolls (Canon §5.5). Its dependency comes from the skin, so a longbow (dependency none) never fails while a railgun skin of the same archetype can. Potency always multiplies damage, buildup, ward, AR and poise.

- **Failure chance.** `failure_bp` is the Canon §5.5 table value for the component's `C`, except that a bearer with `st.unmoored` treats a Strained component (C = 1 or 2) as C = 3 (500 Bp; Canon §5.11). An item made by `mfr.redacted.bureau` has `failure_bp = 0` and keeps its potency loss (Canon §5.9).
- **Unmoored count.** Every failure in §16.3 is a Contradiction failure for Canon §5.11: it calls the bearer's `note_contradiction_failure(t)`, doc 06's one per-character counter, which doc 06 fizzles, doc 07 sparks and doc 08 failed potions also feed (doc 12 owns `st.unmoored`; triage G-057); the third failure within 600 ticks applies `st.unmoored` for 300 ticks.
- **Disconnected** components (C ≥ 6) do nothing:

| Item | While Disconnected |
|---|---|
| melee or ranged weapon | cannot swing, fire, guard or reload; `input.attack` performs `arch.melee.unarmed` attacks and `input.alt` raises an unarmed guard |
| shield | cannot be raised; its AR is 0 |
| armor piece | AR, poise, insulation and resistances 0; mass still counts |
| ward generator | capacity 0; no recharge |
| throwable | cannot be thrown |

### 16.2 Cadence

```rust
fn failure_due(w: &mut WeaponState, mode: Mode, t: u64, trigger_held_prev: bool) -> bool {
    if w.failure_bp == 0 { return false; }
    match mode {
        Semi | Bolt | Charge | Throw | Burst | MeleeSwing | ShieldRaise => true, // once per activation
        Auto | Beam => {
            if !trigger_held_prev || t - w.last_fail_roll >= 30 { w.last_fail_roll = t; true } else { false }
        }
    }
}
// due → keyed_rng("rng.combat", folio, wielder, t, seq).roll_bp(failure_bp), the first draw at that seq (§1.4)
```

A burst rolls on its first shot only. A melee swing rolls on its Windup→Active transition. A shield whose skin has a dependency rolls on each raise at `seq` 1; a weapon guard never rolls, because the weapon's failure is its falter. A ward generator rolls as §15.4 states (`seq` 3). Example: an SMG held for 90 ticks fires on ticks 0, 4, …, 88 and rolls on ticks 0, 32 and 64; at failure 1000 Bp the chance of no jam is `0.9³ = 72.9%`.

### 16.3 Failure outcomes

| Base function | Failure | Recovery |
|---|---|---|
| Firearm, crossbow (techne skin), launcher | **Jam**: no shot, no ammunition spent; state `Jammed`; `emit_noise(40 dB)` | `input.reload` → `Clearing`, `action_ticks(45)`; ejects one round from the magazine |
| Beam | **Stall**: no pulses for 60 ticks | automatic |
| Charge rifle | **Misfire**: the draw is lost; the round stays loaded | draw again |
| Melee weapon | **Falter**: the swing produces no hits; recovery +12 ticks; stamina spent | automatic |
| Throwable: grenade, flask or thrown weapon | **Dud**: the throw happens; a grenade or flask never detonates or shatters, and a thrown weapon deals no damage and no status; the item becomes a ground item where it stops | pick it up and throw again |
| Ward generator | **Flicker**: the recharge start is delayed another 180 ticks | automatic |
| Shield with a dependent skin | **Slip**: that raise produces no guard | release and raise again |

---

## 17. Mounts (retired)

Mounts are cut (DECISIONS.md item 55, Canon §17.7): the game has no mounts or vehicles, so this chapter defines no mount archetype, move, status, input or rule. The section number is kept so that later section references stay stable.

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

After step 8, `resolve_hit` calls doc 04's `on_part_damage`, whose severing branch applies Canon §9.1: a severable hit part is severed by a hit with positive part damage whose largest share is `dmg.slash`, or which carries `SEVER`, when the part's condition pool is 0 after the hit, including a pool that was already 0. Severable parts are limbs, tails and wings on every plan and heads on NPCs; Errata heads never sever.

- **Errata sever only from a crippled pool** (DECISIONS.md item 84, Canon §9.1). An Erratum's severable part is severed only if its pool before the hit was at most 2500 Bp of its max, read as doc 04's wound tier: `floor_div(pool_before × 10000, max) <= 2500` (`wound.crippled` or worse, a pool already at 0 included). A healthy part never severs, however large the hit: a hit that empties it leaves it at `wound.disabled`, and the next qualifying hit severs it. Doc 05 applies this to every severable Erratum part, the tails and wings of Echo bodies included (Canon gap 1).
- **NPC parts** need no crippled pool. An NPC head also needs the owner's governing skill ≥ 75, and severing it kills a single-headed creature (Canon §10.8 step 3).

`SEVER` comes from great blade, great axe and halberd heavies and from launcher blasts within their inner radius. Severing draws no random numbers. The part becomes a `cat.body_part` item at its capsule's midpoint and `ev.part_severed` fires; doc 13 removes template attacks bound to the part.

- **Weapon consequences** (Canon §9.1 hand bindings). Losing `bp.arm_r` drops the active weapon as a ground item and leaves the weapon slots unusable until the arm is restored; losing `bp.arm_l` drops the `slot.offhand` item and makes two-handed weapons unusable (they enter `Staggered` and stay there); losing `bp.arm_extra_N` drops the item in `slot.hand_extra_N`.

Example: a REF NPC's arm pool is `mul_bp(60000, 1250) = 7,500` mHP. A slash-dominant hit on a full-condition arm severs it when it deals at least 7,500 mHP to that arm after `hit_mult` and mitigation; a blunt-dominant hit without `SEVER` only empties the pool. An Erratum with Max HP 80,000 has an arm pool of `mul_bp(80000, 1250) = 10,000` mHP, crippled at 2,500 or less: a slash hit that deals 12,000 mHP to the full arm empties the pool without severing it, and the next slash-dominant hit with positive part damage severs it; an arm at 2,500 mHP hit for at least 2,500 severs, while an arm at 2,501 mHP does not sever from that hit, however large.

### 19.2 `st.taunted`

- Buildup in the mind group. Sources: shield bash (power 40) and `move.taunt` (`input.reload` pressed while guarding with a shield): 30 ticks, 15 stamina, 600-tick cooldown, taunt power 60 to every hostile within 10000 mm whose torso sample `raycast_blocks` reaches from the taunter's eye, in ascending EntityId. The taunt is buildup through the Canon §11.2 chain with no hit record, no damage and no crit draw; power scales with `S(shield ilvl)`.
- Effect on NPCs: `ai_target` is forced to the taunter while active (doc 13), threat jumps per §14.1, and damage dealt to anyone else is × 7500 at step 2.12. Player-controlled bearers follow Canon §11.8.

### 19.3 `st.grappled`

- Applied directly (Canon §11.4, nerve scaling, base 3 s, × rank control) by `move.grapple` (full heavy of a fist weapon or unarmed), by a full chain heavy (entangle at reach), and by NPC grabs (doc 13). It requires a target that is not boss, editor or hunter, is not already grappled, and whose mass (body plus carried) is at most `mul_bp(grappler mass, 12500)`.
- The target cannot move and can use only one-handed actions; a two-handed weapon enters `Staggered` until release. The grappler moves at stance base 2000 mm/s; a fist or unarmed grappler's light attacks hit the target's torso automatically.
- It ends at expiry, when the grappler has taken at least 10% of its Max HP since the grapple began, when the grappler is staggered, knocked down or downed, or when the two are more than `reach + 500` mm apart.
- A grappled player presses `input.interact` to struggle (`move.struggle`): each press removes 10 ticks of remaining duration, at most once per 6 ticks. Input is never synthesized (Canon §11.8).

### 19.4 Law-bound techniques (Canon §5.10, doc 12)

Doc 12 names doc 05 as the owner of the duel and qinggong techniques that its Laws bind. Each technique is a component with `dep = {verve}` and `law_bound` set, so outside its Law's genre (or a seam zone that includes it) it is Disconnected (C = 6) and cannot start; inside, it follows Canon §5.5 like any component.

| Move | `law_bound` | Trigger | Effect |
|---|---|---|---|
| `move.quickdraw` | `law.weird_west.high_noon` | `input.attack` while a one-handed ranged weapon is holstered in a weapon slot and the active weapon is any other | swaps to that weapon and fires one hip shot on the 4th tick of `Drawing`, with spread × 5000 and the normal recoil; a `seq` 0 action start; cooldown 300 ticks |
| `move.qinggong_strike` | `law.wuxia.qinggong` | `input.attack` or `input.heavy` during a doc 03 wall-run or glide | a jump attack (§4.4) that needs no 800 mm air probe and uses `move_bp` 17000 whatever the fall height |

The High Noon standoff itself (doc 12: +5000 Bp and a direct `st.staggered` on the next shot within 60 ticks) is a Law effect, applied at step 1 and after step 8 as §10.2 states; `move.quickdraw` fired inside the standoff takes it.

---

## Exports

All IDs below sit in namespaces Canon §19 assigns to doc 05, and this list is complete: no other ID in these namespaces exists. Role names that other chapters use for kit items resolve to these IDs through §5.6.

| Namespace | IDs | Defined in |
|---|---|---|
| `arch.melee.` (15) | `arch.melee.dagger`, `arch.melee.blade_short`, `arch.melee.blade_long`, `arch.melee.blade_great`, `arch.melee.axe`, `arch.melee.axe_great`, `arch.melee.mace`, `arch.melee.warhammer`, `arch.melee.spear`, `arch.melee.halberd`, `arch.melee.quarterstaff`, `arch.melee.fist`, `arch.melee.whip`, `arch.melee.chain`, `arch.melee.unarmed` | §3 |
| `arch.ranged.` (20) | `arch.ranged.pistol`, `arch.ranged.pistol_heavy`, `arch.ranged.smg`, `arch.ranged.rifle_auto`, `arch.ranged.rifle_marksman`, `arch.ranged.shotgun`, `arch.ranged.sniper`, `arch.ranged.launcher`, `arch.ranged.beam`, `arch.ranged.charge_rifle`, `arch.ranged.crossbow`, `arch.ranged.thrown`, `arch.ranged.grenade_frag`, `arch.ranged.grenade_incendiary`, `arch.ranged.grenade_concussion`, `arch.ranged.grenade_flash`, `arch.ranged.grenade_smoke`, `arch.ranged.grenade_counter`, `arch.ranged.staff`, `arch.ranged.wand` (casting owned by doc 06) | §5, §5.4, §9.3 |
| `arch.armor.` (20) | `arch.armor.head_cloth`, `arch.armor.head_light`, `arch.armor.head_medium`, `arch.armor.head_heavy`, `arch.armor.body_cloth`, `arch.armor.body_light`, `arch.armor.body_medium`, `arch.armor.body_heavy`, `arch.armor.hands_cloth`, `arch.armor.hands_light`, `arch.armor.hands_medium`, `arch.armor.hands_heavy`, `arch.armor.legs_cloth`, `arch.armor.legs_light`, `arch.armor.legs_medium`, `arch.armor.legs_heavy`, `arch.armor.feet_cloth`, `arch.armor.feet_light`, `arch.armor.feet_medium`, `arch.armor.feet_heavy` | §11 |
| `arch.offhand.` (6) | `arch.offhand.shield_buckler`, `arch.offhand.shield_heater`, `arch.offhand.shield_tower`, `arch.offhand.lantern`, `arch.offhand.focus`, `arch.offhand.tome` (casting behavior of focus and tome owned by doc 06) | §5.4, §12.1 |
| `arch.ward.` (4) | `arch.ward.aegis`, `arch.ward.kinetic`, `arch.ward.reflector`, `arch.ward.bulwark` | §15.4 |
| `ammo.` (12) | `ammo.light`, `ammo.heavy`, `ammo.rifle`, `ammo.shell`, `ammo.cell`, `ammo.ordnance`, `ammo.draw`, `ammo.charge_breaker`, `ammo.charge_disruptor`, `ammo.charge_staggerer`, `ammo.charge_sunder`, `ammo.charge_anchor` | §8, §15.3 |
| `move.` (33) | `move.light`, `move.heavy`, `move.sprint_attack`, `move.jump_attack`, `move.riposte`, `move.shove`, `move.guard`, `move.shield_bash`, `move.quick_melee`, `move.grapple`, `move.struggle`, `move.taunt`, `move.imbue`, `move.ads`, `move.hold_breath`, `move.fire_semi`, `move.fire_auto`, `move.fire_burst`, `move.fire_bolt`, `move.fire_charge`, `move.fire_beam`, `move.throw`, `move.cook`, `move.reload`, `move.clear_jam`, `move.lean_l`, `move.lean_r`, `move.quickdraw`, `move.qinggong_strike`, `move.shape_thrust`, `move.shape_rl`, `move.shape_lr`, `move.shape_overhead` | §1.8 |
| `counter.` (5) | `counter.ward_breaker`, `counter.disruptor`, `counter.staggerer` (Canon §17.3a), `counter.sunder`, `counter.anchor` (added here) | §15.1 |

Constants exported: `LAGCOMP_MAX_TICKS = 16`; `POWER10` table (§13); recoil string reset 30 ticks; Errata poise reset 480 ticks; parry re-raise lockout 20 ticks; guard-break lockout 60 ticks; riposte window 30 ticks; firing stance 20 ticks; projectile lifetime 600 ticks; material speed floor `ceil_div(material_bp × 10000, 14000)` (§3.3); ammunition caps, pickups and values (§8.1); Calling reference weapons (§5.5).

Functions exported to other chapters: `resolve_hit` and `resolve_entry_point` (§10.2), which doc 03's damage systems #17 and #26 call for Canon §10.8's two hit batches (§1.3); the projectile hooks `on_body_hit`, `on_liquid_enter`, `on_block_enter`, `on_stop`, `after_step` (§7.1) for doc 03 §5.19; `guard_outcome` (§12.2); `failure_due` (§16.2); `spread_offset` (§6.3) and the recoil step (§6.5) for doc 16's client prediction; `threat_add` and `eff` (§14.1) for doc 13.

## Interface assumptions

| Doc | Assumed to provide |
|---|---|
| 02 | `div128`, `div256` (two-limb `I256`), `isqrt_u64`, `isqrt_u128`, `dist(p, q)` and the `mm(Fx)` conversion in the fixed-point module; `closest_seg_seg` (§4.6), which is §2.2's `seg_seg_params`; `atan2_bam` within ±1 Bam (canon-1.5); the §16 balance harness runs §5.5's duels with §5.5's starting state |
| 03 | the system order of §1.3 (`sys.actions` #7 through `sys.status_timers` #33); `solid`, `opaque`, `material_at`, `destructible_at` (true exactly for blocks tagged `block.tag.destructible`); §5.19's projectile loop calling the five hooks of §7.1 with the catch-up rule of §2.6; §12's explosion queue, ray-method block damage, target point `tp` and exposure samples, with the falloff taken from `inner_mm` and `edge_bp`, block power from the `block_power_milli` of Tables 9.3 and 9.3b, and an item blast's `b_mhp` taken before Canon §10.4 and scaled by falloff and then exposure, one `mul_bp` each, for damage, impact and status powers (§9.4, §10.1); the homing turn inside §5.19 (Projectile.home_bam_s, home_acc; target choice among st.marked hostiles); `damage_block(c, ip, mhp, shares, actor, block_bp)` with impact point `ip`; Canon §10.8's two hit batches, actor hits in #17 and then environmental, contact, surface and DoT hits, shock chains and thorns in #26, each hit carrying its Canon §10.6 entry step in `HitRecord.entry`; §13.2 block damage from `block_bp` and the weapon's tool tier; `apply_knockback`; `emit_gas`; fire ignition; `emit_noise(pos, db)`; event emission in creation order; §5.8's lean using §6.8's sphere clamp |
| 04 | stamina regeneration after 60 ticks without spending, with block stamina loss counted as spending (§12.3); `st.exhausted` blocks heavies, sprint attacks and dodges; the §5.2 XP rows (weapon skills, Shield with parries, Armor with `d_armor`, Acrobatics with negated hits); mastery hooks read by this chapter (Blade 50 parry +3, Shield parry ticks, arc +4096 and stamina × 8500, Heavy Arms 50 removes the firing stance and Heavy Arms 100 lets heavy weapons fire while sprinting, Marksmanship 100 crit replacement); `on_part_damage` with its severing branch, which severs an Erratum's part only when its pool before the hit was at `wound.crippled` or worse (DECISIONS.md item 84, §19.1); `BodyMove`, the crutch and `harness` mouth slots; the comfort band reading §11.1's insulation; Death's Door at `rng.combat` `seq` 0xFFFF; talents `volatile_mix` and `catalyst` (§9.3) |
| 06 | casting through `arch.offhand.focus`, `arch.offhand.tome`, `arch.ranged.staff` and `arch.ranged.wand` (§5.4); casts win the `seq` 0 action slot (§1.4); the `st.shielded` pool; one-handed casting while grappled; the casting stances of §1.6; `note_contradiction_failure` shared with §16.1; each spell hit's `spell_pot` in `HitRecord.spell_pot_bp` for step 2.1 |
| 07 | per material `dmg_bp`, `ar_bp`, density, `pen_cost`, `ricochet`, `flammable`, `blast_res_milli`, tier; instance `mass_g`; `block.tag.destructible` on block definitions and block HP for destructible blocks and placeables (gear has no durability, Canon §4.3); gadget deploys as `seq` 0 action starts; turret shots keyed on the gadget |
| 08 | `emit_gas` for smoke; flasks and bombs reusing `arch.ranged.grenade_*` with `NO_CRIT`; `coat_on_hit` (§10.2) and coated ammunition (§8.2); the sickened toxicity band (§6.3); `st.focused`; respirators as `apparatus.respirator.*`; a consumable's damage to its own drinker or injected target queued with `entry` 3 and `D2 = B` (Canon §10.6) |
| 10 | `curse.` and `curse.pact.` terms of §7.3 and §7.8: `curse.brittle`, `curse.blunted`, `curse.dulled`, `curse.misfire`, `curse.bloodprice`, `curse.backlash`, `curse.pact.fury`, `curse.pact.executioner`, `curse.pact.glass`, `curse.pact.nightmare`, `curse.pact.wildfire`, `curse.pact.thorns`, `curse.pact.bulwark`, `curse.pact.bloodthirst`; locked weapon switching for bound weapons |
| 09 | `sneak_eligible(att, tgt, t)`; `sneak_mult(att, tgt, tier)` with tier 1 = 30000, tier 2 = 22500, tier 3 = 15000 before doc 09's terms (E3 uses tier 1 at Sneak 0); `IGNORE_ARMOR` at Sneak 100; lethal takedowns queued with `entry` 7 and flag `TAKEDOWN` (Canon §10.6); the noise IDs and levels for shots, clang (70), block, parry, jam (40), dry fire (30) and explosions (Table 9.3b) |
| 11 | item templates and overridable fields (fire mode, `sig`, `innate_counter`, `max_ricochets`); ammunition values copied from §8.1; the standard-rank ammunition line at 2500 Bp with §8.3's need weights; counter charges and `item.throwable.counter_*` of every type in every genre's loot pool and every shop, with counter caches in `genre.unwritten` (Canon §17.3a); the manufacturer trait mechanics behind §10.5's hooks, with Somnambule limited to destructible and non-opaque blocks after 10000 mm of flight (Canon §5.9, triage C-27); per-shot and per-swing chance components drawn at `seq` 0 (§1.4); templates, loot and kit rebinding name only this chapter's exported IDs, with role names resolved by §5.6 |
| 12 | `st.unmoored`, applied through doc 06's `note_contradiction_failure` (§16.1); local gravity per genre, bleed zone and flip volume; `law.weird_west.high_noon` and `law.wuxia.qinggong` binding §19.4's techniques |
| 13 | NPC templates: `template_dmg_mhp`, impact, natural AR per part, BRN, TGH, stamina, `block_stability_bp`, guard arc, unparryable and unblockable move flags, `poise_immune`, weapon-like attack geometry, counter bits; `ai_target(e)`; forced targeting under `st.taunted`; target selection with §14.1's `eff` hysteresis; `elite.armored` and `elite.phasing` as adopted in its §7.2 |
| 14 | `difficulty_bp(att, tgt)`, 10000 at the default difficulty; Calling kits that name only this chapter's exported IDs (§5.6), with ammunition entries of the weapon's own family within its §8.1 carry cap (longarm `ammo.rifle`, flamer at most 300 `ammo.cell`, sling stones as one `arch.ranged.thrown` stack) |
| 16 | `view_tick` clamping (§5.5) and `LAGCOMP_MAX_REWIND = 16`; the 32-record pose history; prediction of recoil, sway and spread with the host's keys; `st.downed` entry, crawl 800 mm/s and `bleed_left`, accepting §14.3's shortening; the invader in slot 4; §8 item 2's catch-up steps followed by the normal step against the poses of `t` (doc 03 §5.19, §2.6) |
| 17 | the input IDs and bits of §1.7; `mercy_on` and `mercy_bp` per party slot for step 6c (§10.2); `aim_assist_bp` (§5.2c, DECISIONS 142); last stand QTE prompt and accessibility forms (DECISIONS 135); over-the-shoulder third-person camera toggle (DECISIONS 143); hit markers and damage numbers per §18; accessibility toggles |
| 18 | skin names and presentation for every archetype and event here |

## Canon gaps

Every gap this chapter raised up to canon-1.10 is closed: canon-1.11 adopted the attack factors, `st.taunted`, shield AR on blocks, the 6b split, ward profiles and capacity, the disruptor duration, the stance bases, the `seq` allocation, blunt poise, Shock of the New ranks, fixed terrain against projectiles and the downed rules, and the triage (audit/canon_gaps_triage.md) confirmed doc 05's other rulings. Three open items remain.

1. **Errata parts beyond limbs.** Canon §9.1 (canon-1.11, DECISIONS.md item 84) protects "an Erratum's limb", while the same section lists tails and wings as severable beside limbs, and Echo bodies have them (doc 04 §13). Doc 05 applies the crippled-pool rule to every severable Erratum part (§19.1). Proposed canon wording: "An Erratum's severable part (limb, tail or wing) severs only if its pool was at most 2500 Bp of its max before the hit".
2. **Hits queued while the first batch resolves.** Canon §10.8's second batch names environmental, contact, surface and DoT hits; shock chains, thorns and trait ricochets, which C6 itself queues, are in neither list. Doc 05 resolves them in the second batch (§2.7), as doc 03's #26 row does. Proposed canon wording: "then environmental, contact, surface and DoT hits and every hit queued while the first batch resolves".
3. **Movement kit (DECISIONS.md item 95).** The movement kit adds the 2,000 mm mantle, sprint-slide (doc 03 §5.23) and quickbar grappling tool (doc 03 §5.24, doc 11). Doc 05 now specifies §1.6 stance base speeds, §4.8a action gating and Table 6.3 spread multipliers for slide and grapple. Requested canon text is doc 01 G1 / doc 03 G31.
4. **Solo last stand (DECISIONS.md items 131, 134, 135).** In solo play, lethal damage opening the 300-tick last stand input window before lethal resolution. Requested canon text is doc 04 G2.

## Acceptance tests

1. **Melee calibration.** Simulating each melee archetype's light-combo cycle against the REF target (expected crit 500 Bp × 15000) reproduces Table 3.2a's REF DPS within ±0.2 HP/s, and the full-heavy cycle reproduces §3.2's heavy DPS list within ±0.2.
2. **Ranged calibration.** Each ranged archetype's REF cycle (§1.1) reproduces Table 5.2a's REF DPS within ±0.2, and the AR 40 list in §5.2 within ±0.2: launcher 26.3 and 23.0 with its 150-tick cycle, crossbow 28.0 with its 81-tick cycle, sniper 24.0 on the torso and 48.4 on the head.
3. **E1.** Long blade finisher with a successful crit against AR 10 deals 26,761 mHP (slash 22,043, pierce 4,718); without the crit 16,871.
4. **E2.** The marksman headshot deals 47,817 mHP; the torso shot 26,643.
5. **E3.** The dagger sneak heavy deals 84,899 mHP and makes no `rng.combat` draw at `seq` 16.
6. **E4.** Per-bullet results 4,453 (Coherent), 2,288 (Contradicted, non-native) and 3,048 (native wolf). The wolf gains `st.staggered` on the first hit and not on the second.
7. **E5.** SMG per bullet 3,490 at AR 10 and 1,855 at AR 40; sniper 42,939 and 40,222.
8. **E6.** The blocked hit removes 1,034 mHP and 4,405 mst; unblocked it removes 7,563 mHP; with Scribe's Mercy at `mercy_bp` 2000 the blocked hit removes 828 mHP and the same 4,405 mst.
9. **E7.** After shot 1 the ward is 51,183 and HP is unchanged; shot 2 deals 25,878 HP, emits `ev.ward_broken` and applies `st.stunned` for 45 ticks. Without the counter shot 1 leaves 137,061.
10. **E8.** Frag, all five exposure samples visible: NPC at 2,500 mm takes 30,717; the ally at 3,500 mm in a full Normal medium set takes 16,866 (fire 1,962 after its +1000 resistance); with `mode.ff_off` 6,995.
11. **E9.** The elevated flanking pistol hit at 30 m deals 5,964 mHP.
12. **Spread sample.** Run seed `0x0123456789ABCDEF`, `keyed_rng("rng.combat", 3, (3 << 40) | 5, 1000, 0)`, Coherent auto rifle, cone 546, jitter 18: jitter draw gives +14, `az` 38984, `u` 11749, `rad` 231, `dyaw` −192, `dpitch` −130.
13. **Recoil pattern.** Auto rifle with `jitter_bam` 0, 10 shots, hip, GRP 40: `rec_pitch` 670, `rec_yaw` −72.
14. **Cadence.** 750 RPM shots land on ticks 0, 5, 10, 15, 20, 24 and shot 750 on tick 3,600; 900 RPM fires every 4 ticks.
15. **Jam cadence.** An SMG with failure 1000 held 90 ticks rolls on ticks 0, 32 and 64 only.
16. **Draw counts.** A Coherent shotgun shot makes 17 draws at `seq` 0; with a failure roll due and no jam 18; a jam makes exactly 1.
17. **Mass timing.** Long blade at 700 g with material `dmg_bp` 9000: W 8, R 10, `charge_full` 34, stamina 2,250 mst, impact 9; at 700 g with `dmg_bp` 13000: `time_bp` 9286, W 9, R 11, `charge_full` 37; at 3,430 g: W 13, R 16, `charge_full` 52; active ticks stay 5. Across every material in doc 07's table and every mass from 5000 to 20000 Bp of `ref_mass_g`, `material_bp × 10000 / time_bp ≤ 14000`.
18. **Heavy charge.** Long blade release at `c = 25` gives base 28,665; at `c = 5` `heavy_bp` = 15000; at `c = 40` 24000.
19. **Falloff.** Pistol hits at 10,000, 30,000 and 40,000 mm use 10000, 6250 and 5000.
20. **Wall penetration.** A sniper bullet through one destructible wood crate keeps 8750 with wallpen 120 left; through a destructible cracked stone wall it keeps 6250 with 40 left; against a fixed-terrain stone wall it stops (non-grazing) and the wall takes no damage; a pistol bullet (wallpen 20) stops at a non-grazing stone face.
21. **Ricochet.** Velocity (100000, −20000, 0) mm/s onto stone becomes (60000, 12000, 0) with keep 6000; (100000, −80000, 0) stops.
22. **Elevation and flanking.** `dy` 2,500 → 11000; 2,499 → 10000; −2,500 → 9500. `ang_diff` 10,923 → 10000; 10,924 with the target not targeting the attacker → 11500; 10,924 with the target targeting the attacker → 10000.
23. **Lean.** `|lean_k|` 1, 5, 9 give 38, 194, 350 mm. With a wall 200 mm to the right of the eye, full right lean clamps to 100 mm and the head capsule moves 100 mm.
24. **Throw speed.** PWR 60 gives `throw_v_bp` 10488 and grenade speed 16,780 mm/s.
25. **Explosion.** Frag falloff at 1,000, 2,500 and 4,500 mm (center to the target point `tp`, doc 03 §12.4) is 10000, 6572 and 2000; at 4,501 mm no hit; with all five exposure samples blocked no hit and no crit draw; with four of five exposed the base is × 8000. A spec without `inner_mm` and `edge_bp` reproduces doc 03 §12.4's example (`R` 4000, 2500 mm, 4 of 5 samples: 20,000 mHP from 40,000).
26. **Ammo pickup.** Reserve 230 `ammo.light` plus a 96 pickup gives 240 and leaves an 86 pickup. For every family, one pickup covers at least 4 standard kills at 75% accuracy for the family's hungriest archetype, except `ammo.ordnance` and `ammo.cell` (§8.3).
27. **Energy recharge.** After 120 idle ticks a techne-skinned beam at TEC 9 regains 8,400 mu per 60 ticks; an arcana-skinned beam regains 6,000.
28. **Ward generators.** Kinetic ward 10,000 hit by 8,000 shock passes 3,556 and breaks; aegis passes 1,334. An aegis at ilvl 10 holds 32,380 and recharges 8,095 per 60 ticks after 180 ticks without damage.
29. **Poise.** Warhammer light step 1 on a REF NPC staggers (48,450 mp); a full warhammer heavy knocks down (135,660 mp). Against `elite.unstoppable` neither staggers and poise refills; with `counter.staggerer` the light hit staggers.
30. **Poise reset.** NPC max poise 40, 200, 300 gives reset delays 240, 900, 1,200 ticks; an Erratum refills 480 ticks after the last poise damage.
31. **Armor.** Full Normal medium set at ilvl 1 with Armor 20 gives armor poise 17,600 mp; a Rare Good +1 medium body at ilvl 10 gives torso AR 40.
32. **Armor speed.** Full heavy at Armor 20: sprint 5,960, swim 1,590; full medium 6,140; full heavy at Armor 100: 6,200.
33. **Status power.** Long blade step-1 light gives bleeding power 23; the fifth consecutive hit applies `st.bleeding` at threshold 100.
34. **Threat.** 10,000 mHP of melee damage at MEN 50 adds 13,200 threat.
35. **Downed.** A 17,000 mHP hit on a downed Erratum with Max HP 85,000 removes 360 bleed-out ticks; a DoT pulse also shortens it; the downed Erratum crawls at 800 mm/s base and every weapon, guard and reload input is ignored.
36. **Parry and riposte.** A melee hit inside the parry window deals 0 damage and staggers a standard attacker for 30 ticks (a champion for 15); a long blade riposte has base 36,750; a tower shield never parries.
37. **Shove.** A shove against a guarding target with 30 stamina empties it and applies `st.staggered`.
38. **Somnambule.** A `mfr.oneiric.somnambule` auto rifle bullet that meets a destructible wood crate at 12,000 mm of flight passes it with `wallpen_left` 40 and `keep_bp` 10000, and at exactly 10,000 mm likewise; at 9,000 mm it spends all 40 wallpen and keeps 5000; a fixed-terrain wall stops it at any distance and takes no damage.
39. **Disruptor.** A hit on an `elite.surging` champion applies `st.unraveled` and `st.silenced` for exactly 180 ticks; its 2% per second regeneration is 0 during them and resumes after.
40. **Friendly fire.** An ally's rifle hit with 10,000 mHP before step 2.15 gives D2 5,000 by default and 10,000 in `mode.ff_full`; the thrower's own frag at its feet applies `FF_AREA_BP` 10000.
41. **Lag compensation.** An Erratum hitscan with `view_tick = t - 20` tests poses at `t - 16`; an NPC shot tests poses at `t`; a target in `Guarding` at `t` blocks a rewound hit even though it was not guarding at `t - 16`; a target spawned at `t - 3` cannot be hit by a test rewound to `t - 10`. An Erratum projectile fired at tick 1000 with `view_tick` 994 steps against poses 995 to 1000 and then 1000 again (7 steps); an NPC projectile takes 1 step on its spawn tick. The invader (slot 4) is rewound like an Erratum.
42. **Same-tick order.** Two lethal hits on one target from sources with EntityIds 5 and 7 in one tick: source 5's hit kills; source 7's hit is discarded with no `ev.damage_dealt`. A weapon hit from source 9 and a DoT pulse from source 2 on one target in one tick: the weapon hit resolves first in #17 and the pulse in #26.
43. **Charge rifle.** Release at `c = 30` gives `charge_bp` 7000, speed 70,000 mm/s, spread factor 15000; release at `c = 14` fires nothing and spends no ammunition.
44. **Severing.** A slash-dominant hit that empties a REF NPC's arm pool (7,500 mHP) severs it and spawns a `cat.body_part`; a blunt-dominant hit without `SEVER` does not. An Erratum's full 10,000 mHP arm emptied by one 12,000 mHP slash hit is not severed and the next slash-dominant hit with positive part damage severs it; an Erratum arm at 2,500 mHP severs from a 2,500 mHP slash hit, and one at 2,501 mHP does not sever from a 30,000 mHP slash hit.
45. **Grapple mass.** A grappler of 95,000 g total can grapple a target of 118,750 g and not one of 118,751 g.
46. **Disconnected.** With C = 6, `input.attack` on a held pistol performs `arch.melee.unarmed` attacks and no shot.
47. **Determinism.** A 600-tick combat scenario with fixed inputs yields the same `state_hash` on two runs and across a save and load at tick 300.
48. **Calling duels.** With §5.5's weapons and starting state, 100,000 duels per Calling against `enemy.role.soldier` give mean time to kill 1.84 s (mace) to 2.20 s (dagger) for melee, 1.93 s (pistol) and 2.07 s (auto rifle), all inside 2.0 ± 0.3 s; a fresh-magazine pistol duel gives 1.16 s.
49. **State machine closure.** For 1,000,000 random input and status sequences, the §4.8 and §6.1 machines make at most one transition per tick (plus C7 interrupts), never leave their state sets, never fire with `cool_mt > 0`, and never start a reload while `cool_mt > 0`.
50. **Reload gating.** Holding the trigger from tick 0: the pistol fires on ticks 0, 10, …, 110 and next on 192; the launcher on 0 and 150, ignoring a reload press on tick 30; the crossbow on 0 and 81; the charge rifle released at `c = 45` on ticks 45 and 108.
51. **One action start.** A cast press, a throw press and a trigger press on the same tick start only the cast; the next tick's trigger press fires. An off-hand pistol shot and a main-hand swing may start on the same tick and use `seq` 1 and 0.
52. **Per-shot draws.** A pistol with `affix.frugal` makes 4 draws at `seq` 0 per shot (jitter, 2 spread, frugal) and 5 with a failure roll due; a jam makes 1.
53. **Crit draw count.** A parried hit, a hit on a dodging target and a `NO_CRIT` bomb hit each make exactly one `rng.combat` draw at their `16 + i` slot; a sneak hit makes none.
54. **Unmoored.** Three jams within 600 ticks apply `st.unmoored` for 300 ticks; a Strained (C = 1) pistol then rolls at 500 Bp; a `mfr.redacted.bureau` pistol at C = 4 never rolls and deals × 6000.
55. **Coated ammunition.** A coat of 12 units on a weapon with 8 rounds loaded and 50 in reserve coats the 8 magazine rounds and 4 reserve units; the first 12 shots carry the coat hook and the 13th does not.
56. **Entry points.** A consumable damage effect with `B` 10,000 mHP on its drinker enters at step 3 with `D2` 10,000, draws no `rng.combat` value and takes no additive bucket or friendly fire. A lethal takedown on an NPC with 40,000 mHP HP and a 5,000 mHP ward deals 45,000 `dmg.true` at step 7, emits `ev.ward_broken`, removes 40,000 HP, kills the NPC, carries `TAKEDOWN` and draws no crit value.
57. **Techniques.** `move.quickdraw` outside Weird West (and outside a seam zone including it) cannot start; inside it fires on the 4th draw tick with spread × 5000. `move.qinggong_strike` in Wuxia from a wall-run uses `move_bp` 17000.
58. **Threat decay.** An entry of 333 loses 1 per think and reaches 0 after 333 thinks, when it is removed; an entry of 10,000 loses 30.
59. **JSON examples.** Every JSON block in this chapter parses, contains only integers, strings, booleans, arrays and objects, and validates against the chapter's schemas.
60. **Namespace closure.** Every `arch.melee.`, `arch.ranged.`, `arch.armor.`, `arch.offhand.`, `arch.ward.`, `ammo.`, `move.` and `counter.` ID in any chapter or content file is one of the 115 IDs in Exports; the content loader rejects any other ID in these namespaces, and every kit entry's ammunition belongs to a family its kit weapon fires.
61. **Block split and Mercy.** A blocked hit with shares slash 7500 and pierce 2500 that reaches step 6b with slash 6,000 and pierce 2,001 at stability 5000 sends 4,000 to stamina and splits the remaining 4,001 into slash 3,001 (3,000 plus the remainder 1) and pierce 1,000; on an Erratum with Scribe's Mercy at `mercy_bp` 2000, step 6c leaves slash 2,401 and pierce 800.
62. **Fae iron and taunt.** A `kin.fae` target hit by an iron long blade with `D2` 10,000 (slash 8,000 and pierce 2,000 after step 3) carries slash 10,000 and pierce 2,500 into step 4; the same hit from an item of any other primary material, or from bare hands, carries 8,000 and 2,000. An attacker with `st.taunted` turns 10,000 mHP into 7,500 at step 2.12 against a target other than the taunter and keeps 10,000 against the taunter.
63. **Manufacturer hooks.** A Rustmaw pistol holds 24 rounds and reloads from empty in `action_ticks(93)` ticks (`mul_bp(72, 13000)` = 93); an Umbra weapon never enters `Reloading`, and the shot that brings its heat to 10000 enters `Stalled` for exactly 90 ticks; a Hephaesteum bearer hit by `counter.sunder` gains no `st.corroded` stack.
64. **General and added trait hooks.** A Deepanvil long blade whose light windup after §3.3 is 10 ticks winds up for `ceil_div(10 × 10000, 9000)` = 12 ticks, and a Deepanvil warhammer whose impact after §3.3 is 34 deals `mul_bp(48450, 15000)` = 72,675 mp of poise damage with light step 1 at ilvl 1; a Grimhallow pistol's reload press fills its magazine from reserve on that tick and never enters `Reloading`; a charged Neon Saint shot passes one destructible wood crate with `wallpen_left` and `keep_bp` unchanged and pays §7.3 step 3 for a second crate; a Rainslick hit on a staggered target multiplies `B` by 12000 once, before step 1.

65. **Sprint-slide and grapple combat gating (§4.8a, Table 6.3).** During a sprint-slide, hip fire and one-handed melee attacks are permitted with stance spread multiplier 15000; ADS and heavy charges are rejected. During a grappling pull, one-handed actions are permitted with airborne spread 25000 and two-handed actions are rejected.
66. **Solo last stand QTE (§12.1a).** In solo play, an Erratum failing Death's Door enters last stand for 300 ticks with HP 0; incoming hits during the window are negated; completing the 3-input ink-strike QTE restores HP to 1,000 mHP; failing it emits ev.entity_killed.
67. **Curse combat terms (§10.5a).** An equipped weapon with curse.blunted at potency m = 1000 subtracts 1000 Bp in Canon §10.5 step 1; curse.brittle at m = 1000 subtracts 1000 Bp from slash, pierce and blunt resistances before step 4; curse.dulled at m = 500 subtracts 500 Bp from crit chance.

## Invariants annex

```
Σ doc05: archetypes ⊗ moves ⊗ ballistics ⊗ pipeline-detail ⊗ poise ⊗ counters
W! sim combat state ∈ {Fx, Milli, Bp, Bam, Ticks, i64}        N! float in sim or content
W! every action draw := keyed rng.combat(seq per §1.4)         N! running seq counter
W! crit draws = 1 per hit ∧ 0 per sneak or entry-point hit     W! spread draws = 2 per projectile
W! hit order := batch (actor, then env|contact|surface|DoT) ∧ (source EntityId, seq)   N! hit on a target killed earlier in the tick
W! pipeline order := Canon §10.5                               W! attack factors ⊂ base_mhp before Canon §10.4
W! mechanics := archetype                                      N! skin changes a mechanic field
W! skill mapping := by archetype                               N! skill by skin
W! Σ scaling = 12000 ∀ base archetype                          N! Σ scaling > 15000
W! light stamina ≤ 7/s per full combo cycle                   W! REF DPS ∈ 27.0..=30.5 ∀ archetype ∖ {sniper 24.0, launcher 26.3, unarmed 22.2}
W! recoil, spread, sway ∈ sim ∧ deterministic                  N! hitstop|camera kick|markers → sim
W! lag comp: poses rewound ≤ 16 ∧ state read at t              N! blocks rewound
W! jam roll := base component ∧ Canon §5.5 cadence             N! jam roll for dependency-none skins
W! armor speed effect := stance base speed only                N! armor multiplies move speed
W! blocked hit ⇒ poise damage 0 ∧ buildup × 5000               W! parried hit ⇒ damage 0
W! champions ⊂ {warded, surging, unstoppable}                  W! ∀ counter: dep-none source ∈ every genre
W! ward gen profile ⇒ generator pool only                      N! profile on st.shielded pool
W! FF := step 2.15 ∧ self-blast = area FF                      N! friendly hit → threat | XP | Shock of the New
W! severing deterministic                                      N! Errata head severed
W! Errata sever ⇒ pool_before ≤ 2500 Bp of max (doc 04 tier)    N! item wear | breakage | durability
W! ≤ 1 action start per actor per tick at seq 0                 W! every non-sneak entry-0 hit draws 1 crit (parried, negated, NO_CRIT too)
N! projectile passes a non-destructible opaque cell             N! block damage to a non-destructible cell
W! time_bp ≥ ceil_div(material_bp × 10000, 14000)               W! one magazine pickup covers ≥ 4 REF kills at 75% (light, heavy, rifle, shell, draw)
W! explosion falloff defaults ≡ doc 03 §12.4                    W! projectile geometry := doc 03 §5.19 ∧ contact effects := doc 05 hooks
W! reload start ⇒ cool_mt ≤ 0                                   W! state machines: 1 transition per tick (+ C7 interrupts)
W! counter effects draw nothing                                 W! failure count → note_contradiction_failure (Canon §5.11)
W! entry > 0 ⇒ no crit draw ∧ Canon §10.6 skips                 W! × N manufacturer trait ⇒ once on B, before step 1
W! Somnambule pass ⇒ (destructible ∨ non-opaque) ∧ flight ≥ 10000 mm   N! fixed terrain penetrated
```
