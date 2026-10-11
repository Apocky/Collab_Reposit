# 03 — Simulation Core

As of 2026-10-11. Status: BINDING. Conforms to canon-1.4.

This chapter defines the core of `palimpsest_sim`: the order of systems inside one tick, the entity model and component catalog, voxel world storage and the block registry, the character controller and every collision primitive the simulation uses, the six environmental grids (liquids, gases, fire, temperature, light, sound), explosions, block edits, the event bus with every payload schema, activation, saves and state-hash coverage, and per-subsystem performance budgets. Every other system chapter plugs into the hooks named here. Every rule below was checked against canon-1.4, including fixed terrain with `destructible` blocks and Errata placeables (Canon §4.3), Editor erasure of placeables (Canon §11.6) and the invader in replay slot 4 with its two events (Canon §2.4, §17.9), and against the interface requests that docs 02 and 04 to 17 make of this chapter as they stand on disk (Canon gaps G1).

## Imports

| Source | Section | What this chapter consumes |
|---|---|---|
| Canon | §1.1 | content-ID grammar, `id32 = fnv1a32` |
| Canon | §1.2–§1.6 | numeric types incl. two-channel `Light`; integer and `Fx` primitives; display units; integer-only content |
| Canon | §1.8–§1.9 | coordinates, chunk and local index, Folio box, `block.void_bedrock`; Bam, `sin_bam`, `cos_bam`, `fwd` |
| Canon | §1.11 | `EntityId` layout, phase key, ascending-ID iteration, no `HashMap` |
| Canon | §2.1–§2.5 | `DT`, semi-implicit Euler, tick schedule, rate accumulator, `in_combat`, `light_at`, replay record, save contents, `state_hash` |
| Canon | §3.1–§3.4 | xoshiro256++, `stream_seed`, `keyed_rng`; sequential streams `rng.fire`, `rng.blocktick`, `rng.physics` (owner 03); draw-count rule |
| Canon | §4–§4.3 | world constants and caps; humanoid body; speeds, accelerations, jump, air jump, lean, fall damage, dodge; fixed terrain, the `destructible` tag, `cat.placeable`, resource nodes |
| Canon | §5.2, §5.8, §5.9 | `gravity_bp`, `ambient_light`; `local_genre`, seam zones, `vrv(e)`; physics columns `air_accel_bp`, `fall_dmg_bp`, `ground_friction_bp`, `spawn_block_max` |
| Canon | §6.4 rows 11, 21 | move-speed and climb-speed multipliers |
| Canon | §9, §9.1 | body plans, baked pose tables, 32-tick pose history, severing effects on movement |
| Canon | §10.1, §10.5, §10.6, §10.8 | damage types; pipeline entry; environmental-hazard, DoT and fall-damage entry points; same-tick hit order |
| Canon | §11.1–§11.4, §11.6 | statuses that gate or modify movement and that grids apply; buildup formula; direct application; Editors erase Errata placeables within 1500 mm at one block per 10 ticks |
| Canon | §12, §12.1–§12.3 | `res.breath` contact rule; the surfaces-and-reactions table; sight concealment; spawn light |
| Canon | §13.8 | `P(d)`, `T(d)`, `S(lvl)` for environmental hazards |
| Canon | §13.8a | one material table for blocks and items |
| Canon | §17.5–§17.9 | `FF_AREA_BP` applies to explosions, fire spread, surfaces, falling blocks; ghosts; mounts; invasions (slot 4, invaders never place or break blocks) |
| Canon | §18, §19 | the 44 canonical events, `ev.invasion_started` and `ev.invasion_ended` included; namespaces `comp.`, `sys.`, `grid.` (03), `ev.` and `rng.` bus (03), `block.` shared with 07 |
| Doc 02 | §4.1–§4.6 | `Fx`, `Fx3`, `isqrt_u128`, `atan2_bam`, `closest_seg_seg`, `capsule_overlap`, vertical-capsule-vs-box distance, `ray_capsule`, `dda`, `div128`, `div256` |
| Doc 02 | §5.1–§5.5 | sorted-column store, deferred structural commands, `SystemDesc` and `due`, `step` lifecycle, `PlayerCommand` axes and button bits, intents, `SimEvent` |
| Doc 02 | §6.5, §7.2, §8.5, §9, §15 | serialization sections and `palette_rle`; `ChunkView` generations; light field texture and agreement clamp; audio's use of `ev.noise`; `perf.deck.sim.*` gates |
| Doc 04 | §2.3, §5.3, §7.2, §8.1, §8.5, §11.4, §11.5 | `body_move_bp(e)`, `air_jump_max(e)`, the Acrobatics 50 wall jump and Monk `wind_step` wall-run, part-state struct mirrored by `comp.body_plan`, comfort layer beside the §9.4 hazard layer, fall damage split over legs, `can_sprint`, `can_jump`, `can_dodge` |
| Doc 05 | §1.3–§1.6, §2.1–§2.7, §7, §9.4 | combat phase C1–C9, `HitRecord`, keyed `seq` table, stance base speeds, query kinds, `LAGCOMP_MAX_TICKS = 16`, projectile combat fields and block penetration, `block_bp`, ricochet, grenade explosion parameters |
| Doc 06 | §2.1, §13.1, §13.2 | `aether_tick` in the per-tick resource step, `ignite(cell)`, `add_light(pos, level, expiry)`, conjured blocks |
| Doc 07 | §1.2, §1.4 | `placed_meta` (owner slot, HP, expiry, conjured flag), `PLACED_CAP_PER_PLAYER` 48 and `PLACED_CAP_PER_FOLIO` 160, `sys.crafting`, material table, `mining_ticks`, `block_drops` |
| Doc 08 | §2.2, §3.1, §3.4, §4.2, §5.2, §5.3, §5.6 | liquid and gas records, `evap_gas`, viscosity clamp `500..=10000`, extended triggers, `alch_react`, nitro detonation |
| Doc 09 | §1.3, §1.4, §6.8, §7.9, §13.1 | system slots #5, #7, #30 prelude, #31; `PerceptionState`; `convert_events()`; `door_locked(gidx)`; `awareness_above_unaware(e)`; `noise_level` |
| Doc 10 | §5, §5.5 | appended `ev.item_identified` fields; appraisal pass inside `sys.needs` |
| Doc 12 | §2.3, §9.2, §9.4, §9.5 | GPV fields `ambient_decic`, `liquid_viscosity_bp`, `gas_dissipation_bp`; `protect`, `open_tag` and gravity-flip bits per chunk; relink door pairs |
| Doc 13 | §2.4, §4.1, §4.2, §4.6, §9.2, §10.5, §10.6 | `comp.tags` bit layout, `Blackboard` (`comp.ai`), telegraphs, Redactor `blackout`, `request_spawn`, Editor erasure |
| Doc 14 | §3.2, §3.4 | `sys.run_layer` slot, Colophon entrance `protect` bits |
| Doc 16 | §7.2, §7.3, §8, §9.4, §9.5, §9.13 | `MoverSnap`, the predictor's call of `step_mover`, `Player` (`comp.player`), `sys.coop`, 5-entry `heard`, `cause` on item events |
| Doc 17 | §13 | `mercy_on`, `mercy_bp` in `comp.player` |

---

## 1. Conventions

### 1.1 Types and indices

```rust
pub type Gidx = u32;          // global cell index = x + 256 * (z + 256 * y); in-bounds cells only; 0..4_194_304
pub type ChunkSlot = u8;      // ci = cx + 8 * (cz + 8 * cy); 0..=127 (doc 02 §5.2)
pub type Local = u16;         // lx + 32 * (lz + 32 * ly); Canon §1.8
pub struct IVec3 { pub x: i32, pub y: i32, pub z: i32 }
pub type BlockState = u16;    // §4.2
```

- Ascending `Gidx` order is ascending `(y, z, x)`. Every grid iteration described as "ascending cell order" uses it.
- `cell_of(p: Fx3) -> IVec3 = (fx_floor_block(p.x), fx_floor_block(p.y), fx_floor_block(p.z))`.
- `in_bounds(c) = 0 <= c.x <= 255 && 0 <= c.y <= 63 && 0 <= c.z <= 255`. Every read of an out-of-bounds cell returns `block.void_bedrock` with no liquid, no gas, no fire, light 0 on both channels, and temperature equal to the Folio ambient. Writes to out-of-bounds cells are refused.
- `fx_mm(m) = fx_from_milli(m)`; `to_mm(x) = x.to_milli_floor()` (doc 02 §4.1).

### 1.2 Neighbor orders

Every grid propagation in this chapter uses exactly these visit orders. Doc 02's DDA tie rule (x, then y, then z) is the same family.

| Name | Order |
|---|---|
| `N6` | +X, −X, +Y, −Y, +Z, −Z |
| `N4A` (horizontal, even runs) | +X, −X, +Z, −Z |
| `N4B` (horizontal, odd runs) | −X, +X, −Z, +Z |
| `N18` | the six of `N6`, then the 12 edges (+X+Y), (+X−Y), (−X+Y), (−X−Y), (+X+Z), (+X−Z), (−X+Z), (−X−Z), (+Y+Z), (+Y−Z), (−Y+Z), (−Y−Z) |

### 1.3 Fixed-point constants

| Name | Content value | `Fx` raw |
|---|---|---|
| `BODY_R` | 300 mm | 1288490188 |
| `STAND_H` | 1800 mm | 7730941132 |
| `CROUCH_H` | 1200 mm | 5153960755 |
| `PRONE_H` | 600 mm | 2576980377 |
| `EYE_STAND` / `EYE_CROUCH` | 1620 / 1020 mm | 6957847019 / 4380866641 |
| `STEP_UP` | 550 mm | 2362232012 |
| `SUBSTEP` | 125 mm | 536870912 |
| `SKIN` | 1 mm | 4294967 |
| `GROUND_PROBE` | 2 mm | 8589934 |
| `GROUND_NY` | cos 45° | 3037000499 (`isqrt(2^63)`) |
| `DT` | 1/60 s | 71582788 |
| ground accel per tick | 40000 mm/s² × DT | 2863311520 |
| air accel per tick | 8000 mm/s² × DT | 572662304 |

---

## 2. Tick pipeline

### 2.1 Tick numbering

- `t` is the tick being simulated. The first tick of a run is `t = 0`. The state at boundary `b` is the state after simulating tick `b − 1` and before tick `b` (doc 02 §5.4).
- A Folio's `folio_entry_tick` is the first tick simulated inside it. Folio generation, the initial light bake (§10.5) and the initial liquid activation (§6.3) happen at the boundary before that tick and are not budgeted.
- A system runs on tick `t` when `(t + phase) % period == 0` (Canon §2.2). A per-entity system runs entity `e` when `(t + (e & 0xFF_FFFF_FFFF) % period) % period == 0` (doc 02 `entity_due`).

### 2.2 System order

Systems run in exactly this order every tick. A system that is not due is skipped. After each system the scheduler applies deferred spawn and despawn commands in request order (doc 02 §5.2), so an entity created by system `k` is first iterated by the next system in the table.

| # | System ID | Logic owner | Period, phase | Work |
|---|---|---|---|---|
| 1 | `sys.tick_begin` | 03 | 1, 0 | reset per-actor action `seq` counters to 0; open the event list; clear the per-tick lists (`flashes`, `push_contacts`, noise requests, hearing records, telegraphs) and every `TELEPORTED` flag; on `folio_entry_tick` emit `ev.folio_entered` |
| 2 | `sys.interest` | 03 | 1, 0 | recompute the active-chunk set and actor sleep flags (§15) |
| 3 | `sys.spawn_admit` | 03, 13 | 1, 0 | admit queued AI spawns FIFO while live AI actors < `ACTIVE_ACTOR_CAP` (§15.3) |
| 4 | `sys.input_apply` | 03, 16 | 1, 0 | copy each slot's `PlayerCommand` for `t` into `comp.control` in slot order `0..=4` (slot 4 is the invader, Canon §2.4); apply intents in `(slot, action_seq)` order; derive button presses |
| 5 | `sys.perception` | 09 | 3, per entity | sight checks for awake observers (uses §5.18 DDA, §10 light, §7.6 concealment) |
| 6 | `sys.ai_think` | 13 | 6, per entity | awake agents run doc 13 §4.1's stages (squads, Director, path service included) and write `comp.control` |
| 7 | `sys.actions` | 05, 06, 07, 09, 11 | 1, 0 | doc 05 combat step C1: action state machines for attacks, casts and cast resolution (doc 06), item use, interaction, doors (§13.4), mining progress (§13.2), placing (§13.3), doc 09's lock, hack, takedown and hide machines; queue hits, request projectile spawns, queue explosions, request block edits, emit noise |
| 8 | `sys.movement` | 03 | 1, 0 | `step_mover` (§5) for every awake body, ascending `EntityId`; mounts before riders' slaving; portal crossings (§5.21); fall-damage hits queued |
| 9 | `sys.push` | 03 | 1, 0 | entity–entity separation (§5.14); fills `push_contacts` |
| 10 | `sys.env_sample` | 03 | 1, 0 | per body: local genre, Contradiction re-evaluation hook on change (doc 05), contact cells, immersion, eye cell, temperature, light, gravity sign (§5.20) |
| 11 | `sys.broadphase` | 03 | 1, 0 | rebuild the spatial grid (§3.6) |
| 12 | `sys.pose` | 03 | 1, 0 | advance `anim_tick`, append the pose history record (§5.16) |
| 13 | `sys.melee` | 05 on 03 primitives | 1, 0 | doc 05 step C3: melee capsule traces against hitboxes |
| 14 | `sys.projectiles` | 03, 05 | 1, 0 | doc 05 steps C2 catch-up and C4: §5.19 |
| 15 | `sys.hitscan` | 05 on 03 primitives | 1, 0 | doc 05 step C4: lag-compensated rays and beam pulses (§5.17, §5.18) |
| 16 | `sys.explosions` | 03 | 1, 0 | doc 05 step C5: at most 4 due explosions (§12) |
| 17 | `sys.damage` | 05 | 1, 0 | doc 05 steps C6 to C9: resolve the hit queue in ascending `(source EntityId, seq)` (Canon §10.8); fire surface triggers (§6.9); block damage (§13.2) |
| 17a | `sys.crafting` | 07 | 1, 0 | craft channels, station queues, crops, torch expiry, gadget timers and placed-block expiry (doc 07 §1.4) |
| 18 | `sys.block_updates` | 03 | 1, 0 | at most 2048 queued cell updates (§13.5) |
| 19 | `sys.random_ticks` | 03 | 1, 0 | 8 cells per active chunk (§13.7) |
| 20 | `sys.liquids` | 03, 08 | 6, 1 | §6 |
| 21 | `sys.gases` | 03, 08 | 6, 4 | §7 |
| 22 | `sys.fire` | 03 | 12, 2 | §8 |
| 23 | `sys.temperature` | 03 | 30, 5 | §9 |
| 24 | `sys.surfaces` | 03 (table: Canon §12.1) | 15, 0 | surface contact effects (§6.10) |
| 25 | `sys.status_pulse` | 05 | 15, 0 | DoT, HoT, buildup decay |
| 26 | `sys.damage_late` | 05 | 1, 0 | resolve hits queued after step 17 (environment, contact, DoT, chains) in ascending `(source EntityId, seq)` |
| 26a | `sys.resources` | 04, 06 | 1, 0 | per-tick regeneration that docs 04 and 06 assign to it, with the Canon §2.3 accumulator at `k = 60`, ascending `EntityId`: doc 06 `aether_tick` (§2.1) and doc 04's per-tick pool regeneration |
| 26b | `sys.coop` | 16 | 1, 0 | doc 16 §9.4 and §9.5: downed bleed-out, revive progress, ghost timers; trade validation |
| 27 | `sys.needs` | 04, 10 | 60, 7 | satiation, toxicity, composure, breath and temperature comfort (doc 04), then doc 10's appraisal and familiarity passes |
| 28 | `sys.light_emitters` | 03 | 1, 0 | sync dynamic light emitters and timed lights (§10.4) |
| 29 | `sys.light` | 03 | 1, 0 | at most 4,096 relight updates (§10.5) |
| 30 | `sys.sound` | 03, 09 | 1, 0 | prelude: doc 09 `convert_events()` (§7.9) turns this tick's events into noise requests in event `seq` order; then propagate every request (§11) |
| 31 | `sys.hearing` | 09 | 1, 0 | consume this tick's `HeardNoise` records |
| 32 | `sys.erasure` | 13 | 1, 0 | Erasure stacks on their exact ticks (Canon §11.6); Editor erasure of Errata placeables through §13.8 |
| 33 | `sys.status_timers` | 05 | 1, 0 | end-of-tick status expiry (Canon §2.3) |
| 34 | `sys.despawn` | 03, 11 | 1, 0 | remove dead and expired entities; death drops (doc 11); `ev.entity_despawned` |
| 34a | `sys.run_layer` | 14 | 1, 0 | doc 14's run-state transitions, after every death, pickup and block change of the tick |
| 35 | `sys.tick_end` | 03, 16 | 1, 0 | close the event list into `StepOutput`; column compaction (doc 02) |

A label with a letter suffix (17a, 26a, 26b, 34a) is a full system at that position. The suffix keeps the numbers that docs 09, 10, 13 and 14 cite unchanged. `SCHEDULE` (doc 02 §5.3) lists the 39 systems in table order.

Hashing at boundaries `b ≡ 0 (mod 60)`, network snapshots on even ticks, `extract_view` (doc 02 §7.2) and doc 16's `ReplFrame` extraction happen outside the schedule as read-only passes (doc 02 §5.4, doc 16).

**Doc 05 combat phase.** Doc 05 §1.3 numbers its combat work C1 to C9. They map onto this order as follows. C1 runs inside `sys.actions` (#7), before `sys.movement`, so an action reads the positions of the previous boundary. Projectile spawns requested in C1 are applied after #7. C2's lag-compensation catch-up runs on each projectile's first sweep in `sys.projectiles` (§5.19). C3 is `sys.melee` (#13). C4 is `sys.projectiles` (#14), then the beam pulses of `sys.hitscan` (#15). C5 is `sys.explosions` (#16). C6 to C9 run inside `sys.damage` (#17). C2 to C9 therefore run after movement and collision and before `sys.status_timers`, as doc 05 states. C1 runs before movement (Canon gaps G18).

### 2.3 Stagger of the slow systems

Over one 60-tick cycle the slow systems are due on these ticks `t mod 60`:

| System | Due ticks mod 60 |
|---|---|
| `sys.liquids` | 5, 11, 17, 23, 29, 35, 41, 47, 53, 59 |
| `sys.gases` | 2, 8, 14, 20, 26, 32, 38, 44, 50, 56 |
| `sys.fire` | 10, 22, 34, 46, 58 |
| `sys.surfaces`, `sys.status_pulse` | 0, 15, 30, 45 |
| `sys.temperature` | 25, 55 |
| `sys.needs` | 53 |

No two of liquids, gases, fire, temperature and the status pulse share a tick. The only coincidence is liquids with needs at `t ≡ 53`, and needs costs under 0.05 ms.

### 2.4 Visibility and ordering rules

1. Every system that mutates state iterates entities in ascending `EntityId` and cells in ascending `Gidx`, unless its section names another order.
2. An entity spawned during a system exists from the next system in the §2.2 table. Its `spawn_tick` is `t`.
3. A despawn request marks the entity dead after the requesting system. Dead entities are skipped by every later system and removed by `sys.despawn`.
4. Events are visible to every system later in the same tick. No system reads an event from an earlier tick. Information that must survive a tick boundary lives in components or grids, so the event list is never part of saved state.
5. Block edits apply immediately at the call (§13.1). A later system in the same tick sees the new block.
6. Random draws: `rng.blocktick` only in `sys.random_ticks`; `rng.fire` only in `sys.fire` and in the incendiary step of `sys.explosions` (§12.3); `rng.physics` only in `sys.explosions`. No other system in this chapter draws, and `step_mover` (§5.1) never draws.

### 2.5 Parallelism

The simulation runs on one thread (doc 02 §5.3). Besides worldgen (doc 02), two pure computations run through doc 02's `OrderedMap`, which the host binary backs with worker threads and which is sequential by default; their results merge in index order, so the thread count never changes an outcome: the ray marches of one explosion (§12.2; all 1,352 intensity draws are taken in table order first, then rays march in parallel and merge in ray order) and the temperature diffusion of one run (§9.3, Jacobi over a snapshot, merged in thermal-chunk order). Nothing else in this chapter runs in parallel.

---

## 3. Entity model

### 3.1 Identity: generational IDs and dense indices

- **`EntityId`** is the canon `u64` (Canon §1.11): bits 40–63 hold `folio_index`, which acts as the generation; bits 0–39 hold the per-Folio counter. A value is never reused within a run, because the counter only grows inside a Folio and every Folio has a new index. Saves, events, packets and component fields that reference another entity store `EntityId`, never an index.
- **Dense index.** Each component column (doc 02 §5.2) stores `(EntityId, value)` pairs sorted ascending. An entity's position in a column is its dense index. A system may cache a dense index only for the duration of one system run, because `compact()` at tick end shifts positions. A dense index never influences any outcome; iteration order is always ascending `EntityId`.
- **Allocation.** `spawn` allocates `EntityId = (folio_index << 40) | next_counter`, then increments `next_counter`. The counter resets to 1 at Folio entry. Entities that cross a Folio keep their old, smaller ID, so appends keep every column sorted.

```rust
pub fn spawn(w: &mut World, cmds: &mut Cmds, kind: EntityKind, template: Id32, owner: Option<EntityId>,
             t: Tick, comps: ComponentBundle) -> EntityId {
    let id = EntityId(((w.folio_index as u64) << 40) | w.next_counter);
    w.next_counter += 1;
    cmds.push(Cmd::Spawn { id, kind, template, owner, spawn_tick: t, comps });  // applied after this system
    id
}
pub fn despawn(cmds: &mut Cmds, id: EntityId, reason: Id32 /* grid.despawn.* */) {
    cmds.push(Cmd::Despawn { id, reason });   // sets dead bits after this system; removal in sys.despawn
}
```

`sys.despawn` emits `ev.entity_despawned` for every entity removed without dying, in ascending `EntityId`, then requests column compaction. An entity removed after `ev.entity_killed` emits no `ev.entity_despawned`.

- **Generational semantics.** The canon `EntityId` is itself the generational handle: `folio_index` is the generation and the counter is the slot. No value is reused within a run, so a stale `EntityId` held in a component, a queue, an AI blackboard or an event consumer never aliases a newer entity. Every lookup of a despawned ID returns `None`: `Column::get` skips dead entries until `compact()` removes them at tick end.
- **Dense index type.** `pub struct DenseIdx(pub u32)` is a position inside one column; `Column::index_of(id) -> Option<DenseIdx>` is a binary search over `ids`. A `DenseIdx` is valid only until the end of the system that obtained it. It is never stored in a component, a grid, an event or a save, and it never feeds an outcome.
- **Counter range.** The counter occupies bits 0–39, so a Folio can hold `2^40 − 1` allocations. `spawn` panics with `entity_counter_overflow` beyond that, which a Folio never reaches (one spawn per tick for 580 years).

Example: in Folio 3 the first spawn is `(3 << 40) | 1 = 0x0000030000000001`. If it despawns on tick 500, a component still holding `0x0000030000000001` reads `None` on tick 501 and forever after, even though Folio 3 keeps spawning entities `| 2`, `| 3` and onward.

### 3.2 Entity kinds

| Kind ID | Meaning | Required components | Optional components |
|---|---|---|---|
| `grid.ekind.erratum` | a player character, alive, downed or ghost | `kind`, `pos`, `vel`, `body`, `move`, `control`, `player`, `faction`, `tags`, `pools`, `statuses`, `buildup`, `body_plan`, `pose`, `stats`, `inventory`, `equipment`, `env`, `interest` | `light_emitter`, `mining`, `rider` |
| `grid.ekind.creature` | any NPC: enemy, neutral, minion, hireling; also the invader (Canon §17.9) | `kind`, `pos`, `vel`, `body`, `move`, `control`, `faction`, `tags`, `pools`, `statuses`, `buildup`, `body_plan`, `pose`, `stats`, `env`, and either `ai` with `perception` (every NPC) or `player` with slot 4 (the invader, doc 13 template `enemy.editors.invader`) | `inventory`, `equipment`, `light_emitter`, `interest` |
| `grid.ekind.mount` | a ridden or summoned mount (Canon §17.7) | creature set plus `mount` | `light_emitter` |
| `grid.ekind.projectile` | any moving shot, thrown item or spell bolt | `kind`, `pos`, `vel`, `projectile` | `light_emitter` |
| `grid.ekind.item_drop` | an item lying in the world | `kind`, `pos`, `vel`, `body`, `move`, `item_drop` | `lifetime` |
| `grid.ekind.falling_block` | a block in free fall | `kind`, `pos`, `vel`, `body`, `move`, `falling_block` | — |
| `grid.ekind.container` | chest, Reliquary, Fortune Engine, shop counter | `kind`, `pos`, `container` | `pools`, `body_plan`, `pose` |
| `grid.ekind.gadget` | turret, trap, mine, drone base | `kind`, `pos`, `gadget`, `faction` | `pools`, `body_plan`, `pose`, `vel`, `body`, `move`, `light_emitter` |
| `grid.ekind.zone` | an area effect: spell field, aura, trigger volume | `kind`, `pos`, `zone` | `lifetime`, `light_emitter` |
| `grid.ekind.corpse` | NPC remains kept for loot or necromancy | `kind`, `pos`, `vel`, `body`, `move`, `corpse` | `lifetime` |
| `grid.ekind.marker` | a non-physical anchor: spawn point, scripted interest anchor, exit | `kind`, `pos` | `interest`, `lifetime` |

The row order, from 0 (`erratum`) to 10 (`marker`), is the 4-bit kind code doc 16 §6.8 replicates. New kinds append after row 10 and never renumber a row.

### 3.3 Component catalog

Columns serialize in ascending component-ID byte order, which is the order of this table (doc 02 §6.5 section 2). "Semantics" names the chapter that owns the meaning of the fields; this chapter owns the storage contract.

| Component ID | Rust type | Semantics |
|---|---|---|
| `comp.ai` | `Blackboard` (doc 13 §4.2: target, last-known positions, threat, morale, squad order, cover, path, cooldowns, action state), serialized in doc 13's declared field order | 13 |
| `comp.body` | `BodyState` | 03 |
| `comp.body_plan` | `BodyPlanState { plan: Id32, parts: Vec<PartState> }` with doc 04 §8.1's `PartState { part: Id32, max: Milli, pool: Milli, tier: u8, kind: u8 /* laceration, fracture, burn, trauma */, treated: bool, severed: bool, nat_ar: i64 }` in body-plan part order; the summary wound ID is derived, never stored | 04 |
| `comp.buildup` | `Buildup { meters: Vec<Meter> }`, `Meter { status: Id32, value: i32, last_add: Tick, protect_until: Tick, protect_bp: Bp }`, sorted by `status` | 05 |
| `comp.container` | `ContainerState { inv: Vec<u64>, opened_by: u8 /* slot bitmask */, kind: Id32 }` | 11 |
| `comp.control` | `Control` | 03 |
| `comp.corpse` | `CorpseState { template: Id32, killed_tick: Tick, looted_by: u8 }` | 13 |
| `comp.env` | `Env` | 03 |
| `comp.equipment` | `Equipment { slots: Vec<(Id32 /* slot.* */, u64 /* uid */)> }` sorted by slot ID | 11 |
| `comp.faction` | `FactionC { faction: Id32, owner_slot: u8 /* 0xFF none */ }` | 13 |
| `comp.falling_block` | `FallingBlock` | 03 |
| `comp.gadget` | `GadgetState` (doc 07 §11.3 deployment state machine fields), serialized in doc 07's declared field order | 07 |
| `comp.interest` | `Interest { radius_chunks: u8 }` | 03 |
| `comp.inventory` | `Inventory { uids: Vec<u64> }` ascending | 11 |
| `comp.item_drop` | `ItemDrop { uid: u64, for_slot: u8 /* 0xFF any */, pickup_tick: Tick }` | 11 |
| `comp.kind` | `Kind { kind: Id32, template: Id32, spawn_tick: Tick, owner: Option<EntityId>, flags: u32 }` | 03 |
| `comp.lifetime` | `Lifetime { end_tick: Tick }` | 03 |
| `comp.light_emitter` | `LightEmitter { level: u8, admitted: Option<Gidx> }` | 03 |
| `comp.mining` | `Mining` | 03, 07 |
| `comp.mount` | `MountState { arch: Id32, riders: [Option<EntityId>; 2], gallop: bool }` | 05 |
| `comp.move` | `MoveState` | 03 |
| `comp.perception` | `PerceptionState` (doc 09 §1.4: profile, senses, at most 16 `PairAware` records, disturbance, timers, decay accumulator) | 09 |
| `comp.player` | `Player { slot: u8 /* 0..=4 */, downs_folio: u8, bleed_left: u16, revive_progress: u16, reviver: Option<EntityId>, downed_by: Option<EntityId>, mark_cooldown_until: Tick, trade: Option<TradeState>, mercy_on: bool, mercy_bp: Bp }` (doc 16 §9.4; `mercy_*` doc 17 §13) | 16, 17 |
| `comp.pools` | `Pools { hp, hp_max, stamina, stamina_max, aether, aether_max, composure, composure_max, poise, poise_max, ward, toxicity, breath, breath_max: Milli, satiation: i32 }` | 04, 06 |
| `comp.pos` | `Pos { p: Fx3, yaw: Bam, pitch: i16 }` | 03 |
| `comp.pose` | `Pose` | 03 |
| `comp.projectile` | `Projectile` | 03, 05 |
| `comp.rider` | `Rider { mount: EntityId, seat: u8 }` | 05 |
| `comp.stats` | `Stats` (doc 04: 24 base sub-attributes and training marks, 36 skills with milli-XP, level, char XP, talents), serialized in doc 04's declared field order | 04 |
| `comp.statuses` | `Statuses { list: Vec<StatusInst> }`, `StatusInst { id: Id32, applied: Tick, expiry: Option<Tick>, stacks: u8, pool: Milli, source: EntityId, potency_bp: Bp, acc: i64 }`, sorted by `id` | 05 |
| `comp.tags` | `Tags { bits: u64 }` (undead, fiend, divine, eldritch, construct, electronic, flier, swimmer, swarm, breathless, …; bit positions owned by doc 13) | 13 |
| `comp.vel` | `Vel { v: Fx3 }` | 03 |
| `comp.zone` | `ZoneState { shape: ZoneShape, effect: Id32, period: Ticks, next_pulse: Tick }` | 06 |

For Errata, `Control.prev_buttons` and `Control.action_seq` are serialized in doc 02 section 10 (`inputs`) rather than section 2; `sys.input_apply` is their only writer.

`Kind.flags` bits: 0 `DEAD`, 1 `ASLEEP`, 2 `GHOST`, 3 `NO_COLLIDE_BODIES`, 4 `TELEPORTED` (set by a portal crossing, §5.21, or a scripted move; cleared at `sys.tick_begin`; read by doc 02 interpolation). Bits 5–7 are reserved and 0; bits 8 and above belong to doc 02's view flags and are never stored.

### 3.4 Core component definitions

```rust
pub enum MoveMode { Ground, Air, Swim, Climb, Mantle, Dodge, Fly, Downed, Ridden, Hover, Glide }  // grid.move.*
pub struct BodyState {
    pub radius: Fx, pub stand_h: Fx, pub crouch_h: Fx, pub eye_stand: Fx, pub eye_crouch: Fx,
    pub step_up: Fx, pub base_mass_g: Grams, pub carried_g: Grams, pub kb_resist_bp: Bp,
    pub hitbox_scale_bp: Bp, pub flags: u16,  // 0 PUSHABLE, 1 COLLIDES_BODIES, 2 FLIER, 3 SWIMMER,
                                              // 4 CAN_MANTLE, 5 CAN_CLIMB, 6 EDGE_GUARD, 7 NO_GRAVITY
}
pub struct MoveState {
    pub mode: MoveMode, pub grounded: bool, pub crouched: bool, pub ground_ny: Fx,
    pub ground_state: BlockState, pub coyote: u8, pub jump_buf: u8, pub air_jumps: u8,
    pub kb_ticks: u8, pub climb_lock: u8, pub lean: i8 /* -9..=9 */,
    pub dodge_t: u8, pub dodge_dir: Fx3, pub last_dodge_start: Tick,
    pub mantle: Option<Mantle>, pub peak_y: Fx, pub stride_mm: i32, pub immersion_mm: i32,
    pub gsign: i8 /* -1 normal gravity, +1 flipped (doc 12 Oneiric volumes) */,
    pub wall_n: Fx3,           // horizontal unit normal of this tick's deepest wall contact; zero when none
    pub wall_jumped: bool,     // the doc 04 wall jump was used this airborne period
    pub wall_run_mm: i32,      // wall-run distance used this airborne period (doc 04 wind_step)
}
pub struct Mantle { pub from: Fx3, pub to: Fx3, pub total: u8, pub t: u8, pub end_crouched: bool }
pub struct Control {
    pub move_x: i8, pub move_z: i8, pub yaw: Bam, pub pitch: i16,
    pub buttons: u32, pub prev_buttons: u32, pub action_seq: u32, pub view_tick: Tick,
}
pub struct Env {
    pub genre: Id32, pub feet: CellContact, pub body: CellContact, pub eye: CellContact,
    pub eye_submerged: bool, pub temp: DeciC, pub light_sky: u8, pub light_block: u8,
    pub in_fire: bool, pub contact_acc: Vec<(Id32, i64)>,   // per-surface DoT accumulators (§6.10)
    pub red_ink_ticks: u32,                                  // contact ticks toward +1 st.erasure
}
pub struct CellContact { pub cell: Gidx, pub liquids: [(Id32, u16); 3], pub gas: [(Id32, u16); 2], pub charged: bool }
pub struct Pose { pub anim: Id32, pub anim_tick: u32, pub hist: [PoseRec; 32], pub head: u8 }
pub struct PoseRec { pub tick: Tick, pub p: Fx3, pub yaw: Bam, pub anim: Id32, pub anim_tick: u32, pub lean: i8 }
pub struct Projectile {                    // geometry and lifetime: doc 03; `combat`: doc 05 §7.1
    pub owner: EntityId, pub instigator_slot: u8, pub item_uid: u64, pub payload: Id32,
    pub radius: Fx, pub gravity_bp: Bp, pub drag_bp: Bp, pub bounce_bp: Bp,
    pub end_tick: Tick, pub view_tick: Tick, pub catchup: u8 /* 0..=16 rewind steps, doc 05 §2.6 */,
    pub age: u32, pub resting: bool,
    pub combat: ProjCombat,                // arch, spawn_y, traveled_mm, pierce_left, wallpen0, wallpen_left,
                                           // keep_bp, ricochets, attack_bp, hit_set: BTreeSet<EntityId>,
                                           // counters, flags, block_bp (doc 05 §7.1, §7.3)
}
pub struct FallingBlock { pub state: BlockState, pub start_y: Fx, pub hit: Vec<EntityId>, pub instigator: Option<EntityId> }
pub struct Mining { pub cell: Gidx, pub state: BlockState, pub progress: u32, pub needed: u32, pub last_tick: Tick }
pub enum ZoneShape { Sphere { r: Fx }, Cylinder { r: Fx, h: Fx }, Box { half: Fx3 } }
```

Default `BodyState` for `body.humanoid` Errata: Canon §4.1 values, `kb_resist_bp` 0, `hitbox_scale_bp` 10000, flags `PUSHABLE | COLLIDES_BODIES | CAN_MANTLE | CAN_CLIMB | EDGE_GUARD`. Creatures take their values from their template (doc 13). Item drops: radius 150 mm, height 300 mm, no flags. Falling blocks: radius 490 mm, height 980 mm, no flags. The content validator enforces `radius >= 100 mm` and `height >= 2 × radius` for every body template; with the thinnest collision box at 125 mm (a level-0 layer) and substeps of at most 125 mm (§5.4), no body can pass through a box between two substeps.

### 3.5 Mass

`mass_g(e) = base_mass_g + carried_g`. Doc 11 maintains `carried_g`. Knockback (§5.13) and push shares (§5.14) read it.

### 3.6 Spatial broadphase

- The broadphase is a sorted `Vec<(u32, EntityId)>`, derived state that is never saved. It is rebuilt from current positions at three points of every tick: in `sys.tick_begin` (#1) for systems #2 to #8, inside `sys.push` (#9) over pushable bodies only, and in `sys.broadphase` (#11) for systems #12 to #35. No system reads a broadphase built in an earlier tick, so a restore at any boundary reproduces every query exactly. Keys are 4 m cells: `key = (bx >> 2) + 64 * ((bz >> 2) + 64 * (by >> 2))` for block coordinates clamped into the Folio box.
- Each live entity with `body`, plus every container, gadget and zone, is inserted once per 4 m cell its AABB overlaps. A body's AABB is `[p.x − r, p.x + r] × [p.y, p.y + h] × [p.z − r, p.z + r]`, with `h` the current height. Projectiles are not inserted; they query.
- The vector is sorted by `(key, EntityId)` with a stable sort. A query takes an AABB, visits its 4 m cells in ascending key order, collects IDs, sorts them ascending and removes duplicates.
- Ghosts (`Kind.flags & GHOST`) are inserted only into a separate ghost list, so no hit test, push or AI query ever returns them.
- An entity spawned after a rebuild is absent from that broadphase until the next rebuild; a despawned entity stays in it, flagged dead, and every query skips dead IDs.
- Rebuild: one LSD radix sort on the `u128` value `((key as u128) << 64) | EntityId`, which orders exactly as `(key, EntityId)` and has no equal keys. Budget ≤ 30 ns per entry (§17).

---

## 4. World storage

### 4.1 Grids at a glance

| Grid | Resolution | Storage | Saved in doc 02 section |
|---|---|---|---|
| Blocks | 1 block | per chunk, palette bit-packed `BlockState` | 4 `blocks` |
| Light | 1 block | per chunk, `[u8; 32768]`: low nibble `light_block`, high nibble `light_sky` | 5 `light` |
| Liquids | 1 block | per chunk, sparse `BTreeMap<Local, LiquidCell>` | 6 `fluids` |
| Gases | 1 block | per chunk, sparse `BTreeMap<Local, GasCell>` | 6 `fluids` |
| Fire | 1 block | Folio-wide `BTreeMap<Gidx, FireCell>`, at most 4,096 entries | 7 `thermal` |
| Temperature | 2 × 2 × 2 blocks | per chunk, `Option<Box<[DeciC; 4096]>>` | 7 `thermal` |
| Sound | 1 block | transient per tick; never stored | — |

### 4.2 `BlockState`

`BlockState` is a `u16`: bits 0–10 are the **type index** and bits 11–15 are the **props** (`state = type | props << 11`).

- Type index 0 is always `block.air`. Every other loaded block type gets an index from 1 in ascending content-ID byte order. At most 2,047 block types load; the loader rejects more with `block_type_overflow`. The assignment depends only on the loaded content set, which `content_hash` fixes (Canon §2.4), so raw `BlockState` values may be serialized.
- Props are 5 bits whose meaning depends on the type's shape (§4.4). Bits a shape does not use must be 0; `set_block` rejects a state with stray bits (`bad_props`).
- Facing values: 0 = N (−Z), 1 = W (−X), 2 = S (+Z), 3 = E (+X). This is counter-clockwise order, so `facing_from_yaw(yaw) = ((yaw as u32 + 8192) >> 14) & 3` is the direction an entity looks.

### 4.3 Chunk and palette compression

```rust
pub struct Chunk {
    pub blocks: PalettedBlocks,
    pub light: Box<[u8; 32768]>,                 // low nibble light_block, high nibble light_sky
    pub sky_src: Box<[u64; 512]>,                // 1 bit per cell: sky-channel source (§10.2)
    pub open_tag: Box<[u64; 512]>,               // 1 bit per cell: worldgen "open area" tag (doc 12)
    pub protect: Box<[u64; 512]>,                // 1 bit per cell: refuses actor edits (§4.7)
    pub flip: Box<[u64; 512]>,                   // 1 bit per cell: gravity flipped, gsign = +1 (§4.7, doc 12 §9.5)
    pub liquids: BTreeMap<Local, LiquidCell>,    // §6.1
    pub gases: BTreeMap<Local, GasCell>,         // §7.1
    pub thermal: Option<Box<[DeciC; 4096]>>,     // §9.1; None = every thermal cell at ambient
    pub block_gen: u32,                          // bumps on every block edit; derived, not hashed
    pub field_gen: u32,                          // bumps on light change; derived, not hashed
    pub fluid_gen: u32,                          // bumps on any liquid, gas or fire change; derived, not hashed
}
pub struct PalettedBlocks {
    palette: Vec<BlockState>,   // entry i is the state stored as index i
    counts: Vec<u16>,           // occurrences of each entry; an entry with count 0 is reusable
    bits: u8,                   // 0, 1, 2, 4, 8 or 16
    words: Vec<u64>,            // 32768 * bits / 64 words; an entry never spans two words
}
```

```rust
fn get(pb: &PalettedBlocks, i: Local) -> BlockState {
    if pb.bits == 0 { return pb.palette[0]; }
    let per = 64 / pb.bits as usize;
    let w = pb.words[i as usize / per];
    let idx = (w >> ((i as usize % per) * pb.bits as usize)) & ((1u64 << pb.bits) - 1);
    pb.palette[idx as usize]
}
fn set(pb: &mut PalettedBlocks, i: Local, s: BlockState) {
    let old = get(pb, i);
    if old == s { return; }
    let k = match pb.palette.iter().position(|&x| x == s) {
        Some(k) => k,
        None => match pb.counts.iter().position(|&c| c == 0) {        // lowest reusable entry
            Some(k) => { pb.palette[k] = s; k }
            None => { pb.palette.push(s); pb.counts.push(0); pb.palette.len() - 1 }
        },
    };
    let need = bits_for(pb.palette.len());     // 1→0, 2→1, 3..=4→2, 5..=16→4, 17..=256→8, else 16
    if need > pb.bits { repack(pb, need); }    // rewrite every index at the new width
    write_index(pb, i, k);
    pb.counts[k] += 1;
    let ko = pb.palette.iter().position(|&x| x == old).unwrap_or_else(|| unreachable!());
    pb.counts[ko] -= 1;
}
```

- The palette never shrinks during play. The in-memory layout depends on edit history, so it is never serialized. The canonical serialization is doc 02's `palette_rle` over the 32,768 `BlockState` values in local-index order, which depends only on the blocks.
- Sizes: a uniform chunk stores 0 words. 2 states need 1 bit (512 words, 4 KiB). 5 to 16 states need 4 bits (2,048 words, 16 KiB). 17 to 256 states need 8 bits (4,096 words, 32 KiB). A typical dungeon chunk with 9 states uses 16 KiB.

### 4.4 Shapes, props and collision boxes

Boxes are in millimetres in cell-local coordinates, `[x0, y0, z0]–[x1, y1, z1]`, written for facing N. For facing `k`, apply `(x, z) ↦ (z, 1000 − x)` to both corners `k` times and re-sort each axis. Example: the closed door strip `[0,0,0]–[1000,1000,190]` at facing W becomes `[0,0,0]–[190,1000,1000]`.

| Shape ID | Props bits | Collision boxes | `shape_liquid_cap` (units) | Light, gas |
|---|---|---|---|---|
| `grid.shape.none` | — | none | 1000 | open |
| `grid.shape.full` | — | `[0,0,0]–[1000,1000,1000]` | 0 | registry `light_filter`; gas blocked |
| `grid.shape.pillar` | b0–1 axis (0 Y, 1 X, 2 Z) | as full | 0 | as full |
| `grid.shape.slab` | b0 half (0 bottom, 1 top) | bottom `[0,0,0]–[1000,500,1000]`; top `[0,500,0]–[1000,1000,1000]` | 500 | registry filter; gas passes |
| `grid.shape.stairs` | b0–1 facing (the high side), b2 upside-down | `[0,0,0]–[1000,500,1000]` + `[0,500,0]–[1000,1000,500]`; upside-down: `[0,500,0]–[1000,1000,1000]` + `[0,0,0]–[1000,500,500]` | 250 | registry filter; gas passes |
| `grid.shape.door` | b0–1 facing, b2 open, b3 upper half, b4 hinge right | closed `[0,0,0]–[1000,1000,190]`; open, hinge left `[0,0,0]–[190,1000,1000]`; open, hinge right `[810,0,0]–[1000,1000,1000]` | 1000 | closed: registry filter and `sound_atten_ddb`, gas blocked; open: filter 0, atten 0, gas passes |
| `grid.shape.trapdoor` | b0–1 facing (hinge side), b2 open, b3 top | closed bottom `[0,0,0]–[1000,190,1000]`; closed top `[0,810,0]–[1000,1000,1000]`; open `[0,0,0]–[1000,1000,190]` | 1000 | as door |
| `grid.shape.ladder` | b0–1 facing (away from the wall) | none; climbable | 1000 | open |
| `grid.shape.fence` | b0–3 connect N, W, S, E | post `[375,0,375]–[625,1500,625]`; per connected side the arm `[375,0,0]–[625,1500,375]` rotated to that side | 1000 | open |
| `grid.shape.pane` | b0–3 connect N, W, S, E | post `[437,0,437]–[563,1000,563]`; per side the arm `[437,0,0]–[563,1000,437]` rotated | 1000 | registry filter; gas passes unless `gas_tight` |
| `grid.shape.layer` | b0–2 level 0..=7 | `[0,0,0]–[1000, 125 × (level + 1), 1000]` | `1000 − 125 × (level + 1)` | open |
| `grid.shape.cross` | b0–2 growth age 0..=7 | none; `replaceable` | 1000 | open; registry `conceal` |
| `grid.shape.fixture` | b0–2 attach face (index into `N6`) | none | 1000 | open |

- Fence and pane connections are recomputed in `sys.block_updates` (§13.5): a side connects when the neighbor on that side is a fence or pane of the same shape, or has a sturdy face toward this cell.
- A fence is 1500 mm tall. Jump height reaches 1500 mm only at `vrv ≥ 7` (Canon §4.2), so fences contain enemies and players in low-VERVE genres.
- The `gas_tight` registry flag makes a non-full shape block gas (glass panes, sealed hatches).

### 4.5 Block registry schema

Doc 07 owns the block records and their values; this chapter owns the schema. Every field is an integer or a content-ID string (Canon §1.6). Flags are `0` or `1`.

| Field | Type | Default | Meaning |
|---|---|---|---|
| `id` | `block.*` | required | content ID |
| `material` | `mat.*` | required | one material table (Canon §13.8a); supplies defaults marked "mat" through doc 07 §2.5 |
| `tags` | list of `block.tag.*` | `[]` | doc 07 §10.1 tags; `block.tag.destructible` makes the block **destructible** (Canon §4.3), `block.tag.errata_placeable` marks a `cat.placeable` block |
| `shape` | `grid.shape.*` | `grid.shape.full` | §4.4 |
| `collides` | flag | 1 (0 for none, ladder, cross, fixture) | collision boxes exist |
| `hardness_milli` | i32 | mat: `1000 × hardness` when destructible, else −1 | mining and damage hardness; −1 = unbreakable. The integer "hardness" that docs 06 and 11 compare is `floor_div(hardness_milli, 1000)` |
| `tool_tier` | `tooltier.*` | `tooltier.t0` | minimum tool tier to break (doc 07) |
| `blast_res_milli` | i32 | mat: `500 × (hardness + toughness)` | explosion resistance (§12.2); a non-destructible block still attenuates blasts with it |
| `light_filter` | 0..=15 | 15 for full and pillar, else 0 | extra light loss entering this cell; 15 = opaque |
| `light_emit` | 0..=15 | 0 | constant `light_block` emission |
| `flammability_bp` | Bp | mat | spread chance factor (§8.3) |
| `burn_runs` | u16 | mat: `floor_div(density, 15)` when flammable, else 0 | fire runs a burning block lasts (§8.4) |
| `burn_result` | `block.*` | `block.air` | block left after burnout |
| `ignite_decic` | DeciC | 0 | heat ignition threshold; 0 = never ignites from heat |
| `liquid_interaction` | `grid.liqi.*` | `grid.liqi.blocks` | §6.2 |
| `absorb_per_run` | u16 units | 0 | for `grid.liqi.absorbs` |
| `heat_cond_bp` | Bp | mat: `1000 + 900 × conductivity` | thermal conductance (§9.3) |
| `heat_emit_decic`, `heat_drive_bp` | DeciC, Bp | absent | constant heat source (§9.2) |
| `melt_decic`, `melt_to`, `melt_units` | DeciC, `liq.*`, u16 | absent | heat melting into liquid (§9.5) |
| `sound_atten_ddb` | i32 tenths of a dB | mat: `clamp(100 + floor_div(density, 10), 100, 600)` for collidable shapes, 0 for air and non-collidable shapes | added when sound enters this cell (§11.2) |
| `friction_bp` | Bp | 10000 | ground acceleration and deceleration factor (§5.3) |
| `climbable` | flag | 1 for ladder, else 0 | §5.11 |
| `gravity` | flag | 0 | becomes a falling block when unsupported (§13.6) |
| `attach` | flag | 1 for ladder, fixture, door; else 0 | needs a sturdy support face (§13.5) |
| `sturdy_faces` | 6-bit mask in `N6` order | 63 for full and pillar; slab bottom 8 (−Y); else 0 | faces that support attachments and placement |
| `replaceable` | flag | 1 for cross and layer level 0, else 0 | placement may overwrite it (§13.3) |
| `gas_tight` | flag | 0 | §4.4 |
| `conceal` | 0..=15 | mat (Canon §12.2) | sight concealment of this cell |
| `random_tick` | object | `{"kind": "grid.rt.none"}` | §13.7 |
| `explosive` | object | absent | an `ExplosionSpec` (§12.1) triggered when this block is broken by fire, explosion or damage |
| `block_hp_mhp` | i64 | `hardness_milli × 20` | break threshold of the block-damage map (§13.2); doc 07's `block_hp(cell)` returns it for cells without `placed_meta` |
| `skins` | map genre suffix → `art.*` | material default | presentation only; the sim never reads it |

**Loader rules** (content validation fails the build):

1. `block.air` has type index 0, shape `grid.shape.none` and no tags.
2. A block without `block.tag.destructible` has `hardness_milli = -1`, and it must not set `gravity`, `flammability_bp > 0`, `melt_decic`, `liquid_interaction = grid.liqi.displaced`, `explosive`, or a `random_tick` of kind `grid.rt.decay`. Fixed terrain therefore never burns, melts, falls, washes away, decays or detonates.
3. A `grid.rt.spread` rule's `from` block must be destructible.
4. Every block carrying `block.tag.errata_placeable` also carries `block.tag.destructible` (doc 07 §10.1).
5. `explosive.block_power_milli` is in `0..=8000` and `explosive.radius_mm` in `500..=12000` (§12.1).

Illustrative records (doc 07 §10.1 owns final IDs and values):

```json
[
  { "id": "block.air", "material": "mat.air", "shape": "grid.shape.none", "collides": 0,
    "hardness_milli": -1, "blast_res_milli": 0, "light_filter": 0, "sound_atten_ddb": 0,
    "heat_cond_bp": 10000, "conceal": 0 },
  { "id": "block.masonry.marble", "material": "mat.marble", "shape": "grid.shape.full",
    "hardness_milli": -1, "tool_tier": "tooltier.t1", "blast_res_milli": 6000, "light_filter": 15,
    "flammability_bp": 0, "heat_cond_bp": 2000, "sound_atten_ddb": 400, "sturdy_faces": 63,
    "skins": { "mythic_antiquity": "art.block.masonry.marble.antiquity" } },
  { "id": "block.door.wood", "material": "mat.oak", "shape": "grid.shape.door",
    "tags": ["block.tag.destructible"],
    "hardness_milli": 3000, "blast_res_milli": 3000, "light_filter": 15, "flammability_bp": 2500,
    "burn_runs": 50, "ignite_decic": 3000, "liquid_interaction": "grid.liqi.permeable",
    "sound_atten_ddb": 150, "attach": 1, "heat_cond_bp": 1500 },
  { "id": "block.cover.sandbag", "material": "mat.sand", "shape": "grid.shape.full",
    "tags": ["block.tag.destructible", "block.tag.errata_placeable"],
    "hardness_milli": 3000, "blast_res_milli": 3500, "liquid_interaction": "grid.liqi.absorbs",
    "absorb_per_run": 2, "sound_atten_ddb": 300 },
  { "id": "block.powder_keg", "material": "mat.oak", "shape": "grid.shape.full",
    "tags": ["block.tag.destructible"], "hardness_milli": 1000,
    "flammability_bp": 6000, "burn_runs": 5,
    "explosive": { "radius_mm": 4000, "inner_mm": 1000, "edge_bp": 2500, "block_power_milli": 4000,
                   "env_base_mhp": 30000, "shares": { "dmg.blunt": 5000, "dmg.fire": 5000 },
                   "knockback_mm_s": 10000, "incendiary": 1, "ignite_radius_mm": 2000, "ignite_bp": 3333,
                   "drop_bp": 3000, "smoke_conc": 3000, "heat_decic": 2000, "noise": "noise.explosion" } }
]
```

At content load the registry is compiled into per-state tables indexed by `BlockState`: `boxes[s]` (rotated `Fx` boxes), `full[s]` (one full box), `light_filter[s]`, `gas_open[s]`, `liquid_cap[s]` (§6.2), `sound_atten[s]`, `climbable[s]`, `sturdy[s]`, `conceal[s]`, `destructible[s]`, `material[s]`. These tables are derived from content and never serialized.

**Queries** exported to docs 05, 06, 09 and 13: `state_at(c) -> BlockState`, `solid(c) = !boxes[state_at(c)].is_empty()`, `opaque(c) = light_filter[state_at(c)] == 15`, `material_at(c) -> Id32`, `destructible_at(c) -> bool`. Each reads `block.void_bedrock` outside the Folio (§1.1).

### 4.6 Genre skins

The simulation stores no skin. A block renders with the skin of `local_genre` at its cell, through doc 02 §8.3's per-genre texture arrays and §8.9's bleed blend. A block carried from Cyberpunk and placed in a Fairy Tale Folio therefore renders in the Fairy Tale skin. Skins change no block behaviour.

### 4.7 Fixed terrain, protection, open-area and flip masks

- **Fixed terrain** (Canon §4.3). A cell whose current block is not destructible keeps its type for the rest of the Folio: `set_block` refuses a change of type index with `fixed_terrain` unless the cause is `grid.cause.worldgen` or `grid.cause.script`. A change of props alone is allowed (door toggles, growth age, fence and pane connections). Mining, block damage, explosions and liquid displacement skip non-destructible cells; such cells still attenuate blasts (§12.2) and still collide.
- `protect` bits come from worldgen (doc 12 §9.2): exit portals, Folio-boundary shells, boss-arena seals while sealed, the Colophon entrance until the Rubric completes (doc 14 §3.4), and scripted props. `set_block` refuses actor-caused edits of a protected cell with `protected`; worldgen and script causes bypass it. Explosions skip protected cells. Docs 12 and 14 clear a bit through `set_protect(c, false)` with cause `grid.cause.script`.
- `open_tag` bits come from worldgen and mark cells in rooms doc 12 tags as open areas (courtyards, caverns lit by the genre's ambient). They feed the sky channel (§10.2).
- `flip` bits come from worldgen (doc 12 §9.5, Oneiric gravity-flip rooms). A body whose center cell has the bit uses `gsign = +1` (§5.12); a projectile whose cell has the bit falls upward (§5.19). Liquids, gases, falling blocks and every grid ignore the bit.

---

## 5. Movement and collision

`sys.movement` processes every live, awake entity that has `move`, in ascending `EntityId`. Mounts are processed in that order like any entity; riders skip their own integration and copy the seat position afterwards (§5.12).

**The controller is one pure function.** `sys.movement` runs `step_mover` for every body, and doc 16's client predictor runs the same function for the local Erratum (doc 16 §7.3). It reads only its arguments, draws no random numbers, emits no events and touches no other entity. Entity push (§5.14), knockback impulses (§5.13) and mount slaving are host-only steps outside it.

```rust
/// SPEC: 03 §5. MoverSnap is doc 16 §7.2's struct: tick, p, v, yaw, pitch, mv: MoveState, gates, speed_bp, genre, ride.
pub fn step_mover(m: &mut MoverSnap, c: &Control, prm: &MoverParams, q: &dyn BlockQuery) -> MoverOut;

pub struct MoverParams {
    pub body: BodyState,                        // §3.4, at the current crouch state
    pub row11_bp: Bp, pub row21_bp: Bp,         // Canon §6.4 move-speed and climb-speed multipliers
    pub body_move_bp: Bp,                       // doc 04 §8.5 legs, crutch, prosthetics; 10000 when intact
    pub status_speed_bp: [Bp; 8], pub n_status: u8,  // active move-speed status multipliers, ascending status ID
    pub walk_base: i32, pub sprint_base: i32, pub swim_base: i32,   // mm/s: 4300, 6500, 2400, or doc 05 §1.6 armor values
    pub stance_base: i32,                       // doc 05 §1.6 lowest active stance base speed; i32::MAX when none
    pub vrv: i32,                               // Canon §5.8 vrv(e) at the start of the tick
    pub can_sprint: bool, pub can_jump: bool, pub can_dodge: bool,  // doc 04 predicates (stamina, exhaustion)
    pub air_jump_max: u8,                       // doc 04 §2.3 air_jump_max(e)
    pub wall_jump: bool, pub wall_run_max_mm: i32,  // doc 04 §5.3 Acrobatics 50; doc 04 §7.2 wind_step (2000) or 0
    pub mount_mode: Option<MoveMode>,           // hover or glide for those mount templates (§5.12)
}
pub trait BlockQuery {
    fn state(&self, c: IVec3) -> BlockState;                     // block.void_bedrock outside the Folio
    fn liquid_total(&self, c: IVec3) -> u16;                     // units, §6.1
    fn dominant_liquid(&self, c: IVec3) -> Option<(Id32, Bp)>;   // most units, ties to lower id32; its move_bp
    fn has_liquid(&self, c: IVec3, liq: Id32) -> bool;           // at least 1 unit (oil friction, §5.3)
    fn genre_phys(&self, c: IVec3) -> GenrePhys;                 // gravity_bp, air_accel_bp, ground_friction_bp, fall_dmg_bp
    fn flip(&self, c: IVec3) -> bool;                            // §4.7
    fn portals(&self) -> &[PortalPair];                          // §5.21, with their current state
}
pub struct MoverOut {
    pub landed_fall_mm: Option<i32>,            // set on the tick a body enters ground (§5.15)
    pub footstep: Option<Id32>,                 // grid.gait.* when a stride completes (§5.15)
    pub ground_state: BlockState, pub teleported: bool,
}
```

`sys.movement` builds `MoverSnap` and `MoverParams` from the entity's components, calls `step_mover` with the host grid as `BlockQuery`, writes the results back, then turns `MoverOut` into effects: a fall-damage hit for `sys.damage` and `ev.entity_landed` (§5.15), `ev.footstep`, and the `TELEPORTED` flag. Doc 16 builds the same inputs from replicated state; any input the client lacks shows up as a correction, never as a different rule.

### 5.1 Movement modes

| Mode (`grid.move.*`) | Gravity | Control | Enters when | Leaves when |
|---|---|---|---|---|
| `ground` | yes | ground acceleration | grounded after integration | not grounded after snap |
| `air` | yes | air acceleration | not grounded; jump; knockback | grounded; swim; climb; mantle |
| `swim` | no (buoyancy) | 3D swim | `immersion_mm >= 1000` (crouched: 700) | `immersion_mm < 900` (crouched: 600) |
| `climb` | no | ladder axis | overlaps a climbable cell and (forward input or already climbing) and `climb_lock == 0` | no climbable overlap; jump |
| `mantle` | no | scripted | §5.9 test passes on a jump press | finished or aborted |
| `dodge` | yes | scripted 12 ticks | DODGE press on ground, allowed by doc 04 and statuses | 12th tick done |
| `fly` | no | 3D | `FLIER` creatures; ghosts | never (flags) |
| `downed` | yes | crawl 800 mm/s, height 600 mm | `st.downed` gained | `st.downed` removed and standing room exists |
| `ridden` | no | input routed to the mount | `ev.mounted` | `ev.dismounted` |
| `hover`, `glide` | special | mount archetypes | mount templates (§5.12) | — |

Evaluation order each tick, first match wins: downed, ridden, mantle in progress, dodge in progress, fly, swim, climb, dodge start, mantle start, jump, then ground or air from integration. The selection is a closed state machine: every mode is re-derived each tick from this function, so no mode can be left without a defined successor.

```
select_mode(m, c, prm, q) -> MoveMode:
    if m.gates & DOWNED:                                 return downed
    if m.ride.is_some():                                 return ridden
    if let Some(mode) = prm.mount_mode:                  return mode              // hover, glide
    if m.mv.mode == mantle ∧ m.mv.mantle.is_some():      return mantle            // until finished or aborted
    if m.mv.mode == dodge ∧ m.mv.dodge_t < 12:           return dodge
    if prm.body.flags & FLIER ∨ m.gates & GHOST:         return fly
    if m.mv.gsign == −1:
        enter = if crouched { 700 } else { 1000 };  stay = if crouched { 600 } else { 900 }
        if immersion_mm ≥ enter ∨ (m.mv.mode == swim ∧ immersion_mm ≥ stay): return swim
        if CAN_CLIMB ∧ climbable_overlap ∧ m.mv.climb_lock == 0 ∧ (fwd_in > 0 ∨ m.mv.mode == climb):
                                                         return climb
    if press(DODGE) ∧ m.mv.grounded ∧ prm.can_dodge ∧ ¬movement_gate(m) ∧ m.tick − m.mv.last_dodge_start ≥ 18:
        m.mv.dodge_t = 0; m.mv.last_dodge_start = m.tick;  return dodge
    if press(JUMP) ∧ mantle_test(m, c, prm, q):          return mantle            // §5.9 sets m.mv.mantle
    return if m.mv.grounded { ground } else { air }                                // jumps fire in §5.7
```

`press(b)` means bit `b` is held in `c.buttons` and was not in `c.prev_buttons`. `movement_gate(m)` is true when `gates` holds staggered, stunned, frozen, knocked down, sleeping, rooted or grappled (doc 16 §7.2 bits 0–6). Mode-specific steps run after selection: ground, air, dodge and downed use §5.2–§5.8 and §5.12; swim §5.10; climb §5.11; mantle §5.9; fly, hover and glide §5.12; ridden runs nothing, because the mount's step carries the rider.

**Status gates** (Canon §11.1, §9.1), read from `MoverSnap.gates`: `st.staggered`, `st.stunned`, `st.frozen`, `st.knocked_down`, `st.sleeping`, `st.rooted`, `st.grappled` zero the wish velocity. `st.overburdened` and a severed leg forbid sprint, jump and dodge. `st.exhausted` forbids sprint and dodge. `st.levitating` sets gravity to 0 and gives vertical control of 1500 mm/s through JUMP (up) and CROUCH (down). Doc 04's `can_sprint`, `can_jump` and `can_dodge` fold these with stamina, and `step_mover` reads only those three flags plus the gate bits.

### 5.2 Wish velocity

Input axes follow doc 02 §5.5: `fwd_in = −move_z`, `right_in = move_x`, each in `−127..=127`. In Q2.30, `right = (cos_bam(yaw), 0, −sin_bam(yaw))` and `fwdh = (−sin_bam(yaw), 0, −cos_bam(yaw))`.

```
m      = isqrt(right_in² + fwd_in²)                      // 0..=179
if m == 0: wish = 0
class  = backpedal if fwd_in × 2^30 < cos_bam(12288) × m      // angle > 67.5°
         strafe    if fwd_in × 2^30 < cos_bam(4096) × m       // angle > 22.5°
         forward   otherwise
sprint = SPRINT held ∧ class = forward ∧ ¬crouched ∧ can_sprint ∧ stance_base = i32::MAX
base   = sprint_base                                          if sprint
         min(if crouched { 2000 } else { walk_base }, stance_base)   otherwise   // doc 05 §1.6 minimum
dir_bp = 8000 (backpedal) | 9500 (strafe) | 10000 (forward); sprint uses 10000
spd    = mul_bp(base, dir_bp) → × 5000 if WALK held → × §6.4 row 11 → × body_move_bp (doc 04 §8.5)
         → × status_speed_bp[0], …, status_speed_bp[n_status − 1]      // one mul_bp each, ascending status ID
den    = max(m, 127)
wish   = (right_in·right + fwd_in·fwdh) × fx_mm(spd) / (den × 2^30)        // per axis, floor
```

`cos_bam(12288) = 410903207` and `cos_bam(4096) = 992008094`. A stance (aiming, guarding, leaning, swinging) cancels sprint and lowers the base by doc 05 §1.6's minimum rule. Example: `move_x = 90`, `move_z = −90` gives `m = 127`; `90 × 2^30 = 96,636,764,160` is below `992008094 × 127 = 125,985,027,938` and above `410903207 × 127 = 52,184,707,289`, so the class is strafe. At AGI 60 (row 11 = 10500): `mul_bp(mul_bp(4300, 9500), 10500) = 4289` mm/s. Forward at AGI 60 gives 4515 mm/s, and with `st.slowed` 3160 mm/s.

### 5.3 Acceleration, gravity and integration

`genre` is `q.genre_phys(cell of the body center)`: `GenrePhys { gravity_bp, air_accel_bp, ground_friction_bp, fall_dmg_bp }` of `local_genre` (Canon §5.2, §5.9).

```
g        = fx_mm(mul_bp(20000, genre.gravity_bp))                         // local genre (§5.20)
on_ground: rate = mul_bp(40000, block.friction_bp)                                       if wish ≠ 0
           rate = mul_bp(mul_bp(mul_bp(40000, block.friction_bp), genre.ground_friction_bp), oil_bp) if wish = 0
           oil_bp = 3000 while the feet cell has liq.oil contact (Canon §12.1), else 10000
in air or kb_ticks > 0: rate = mul_bp(8000, genre.air_accel_bp)
dv_max   = mul_fx(fx_mm(rate), DT)
dh       = wish_h − v_h ;  if |dh| <= dv_max { v_h = wish_h } else { v_h += dh × dv_max / |dh| }
if gravity applies: v.y += gsign × mul_fx(g, DT)
clamp |v.y| <= 50000 mm/s (Canon §4 terminal) and |v_h| <= 50000 mm/s
p_delta  = (mul_fx(v.x, DT), mul_fx(v.y, DT), mul_fx(v.z, DT));  move_and_collide(e, p_delta)
```

`block` is the ground block under the entity (`MoveState.ground_state`). Ground acceleration per tick at friction 10000 is raw 2863311520 (0.6667 m/s), so walking speed 4300 mm/s is reached on tick 7. Ice (`friction_bp` 1500) gives 0.1 m/s per tick.

### 5.4 Capsule against voxels

A body is the vertical capsule of doc 02 §4.6: segment `y0 = p.y + r` to `y1 = p.y + h − r` at `(p.x, p.z)`, radius `r`, current height `h`.

**Gather.** Cells overlapping `[p.x − r − SKIN, p.x + r + SKIN] × [p.y − SKIN, p.y + h + SKIN] × [p.z − r − SKIN, p.z + r + SKIN]`, in ascending `Gidx` order; each cell contributes `boxes[state]` offset by the cell origin, in table order. Out-of-bounds cells contribute one full box.

**Penetration against one box `[lo, hi]`.**

```
dx = max(0, lo.x − p.x, p.x − hi.x);  dz = max(0, lo.z − p.z, p.z − hi.z)
dy = max(0, lo.y − y1, y0 − hi.y)
d2 = dx² + dy² + dz²   (i128 of raws)
if d2 >= r²: none
if d2 > 0: dist = Fx(isqrt(d2)); depth = r − dist
           n = (sgn_x·dx, sgn_y·dy, sgn_z·dz) / dist, sgn = −1 when the capsule is on the low side of that axis
else:      // segment touches the box interior: push out along the cheapest axis
           candidates in order +Y: hi.y − p.y; −Y: lo.y − h − p.y; +X: hi.x + r − p.x; −X: lo.x − r − p.x;
                                 +Z: hi.z + r − p.z; −Z: lo.z − r − p.z
           pick the smallest |Δ| (ties keep the earlier candidate); depth = |Δ|, n = that axis
```

**Move and collide.**

```rust
fn move_and_collide(e: &mut Body, delta: Fx3) -> Contacts {
    let n_sub = max(1, ceil_div(max_abs_axis(delta).raw(), SUBSTEP.raw()));   // 125 mm substeps
    let mut rem = delta; let mut c = Contacts::default();
    for i in 0..n_sub {
        let step = rem.div_int(n_sub - i);            // floor per axis; the last substep takes the rest
        e.p = e.p.add(step); rem = rem.sub(step);
        for _ in 0..4 {                                // at most 4 depenetration passes per substep
            let Some((depth, n, cell)) = deepest_penetration(e) else { break };  // ties: first in gather order
            e.p = e.p.add(n.scale(depth));             // exact push: the capsule ends touching (d = r)
            c.record(n, cell);                         // ground if n.y × (−gsign) >= GROUND_NY
            let vn = e.v.dot(n); if vn < Fx::ZERO { e.v = e.v.sub(n.scale(vn)); }
            let rn = rem.dot(n); if rn < Fx::ZERO { rem = rem.sub(n.scale(rn)); }
        }
    }
    c
}
```

Worked examples (raw values computed with the canon primitives):

1. **Wall.** A standing player at `p = (10.75, 1.0, 5.5)` overlaps the block at (11, 1, 5). `dx = 0.25 m` (raw 1073741824), `dy = dz = 0`, `dist = 0.25`, depth raw 214748364, `n = (−1, 0, 0)`. The push moves `p.x` from raw 46170898432 to raw 45956150068 (10.7 m).
2. **Floor.** A player resting at `y = 1.0` on the block at y 0 falls one tick: `v.y` = raw −1431655760, `p.y` drops by raw 23860930 (5.56 mm). `dy = y0 − hi.y` = raw 1264629258, depth raw 23860930, `n = (0, 1, 0)`. The push returns `p.y` to exactly raw `1 << 32`, `v.y` is clipped to 0, and the contact is ground.

**Full body move.** `move_body` wraps `move_and_collide` with step-up (§5.6), the ground probe and snap (§5.5) and the edge guard (§5.8). Every mode that moves a capsule through the world calls it. `gdir = (0, gsign, 0)` is the gravity direction and `up = −gdir`.

```
struct Contacts { list: Vec<(Fx3 n, Gidx cell)> }       // in record order; ground and wall are derived
is_ground(n) = n.y × (−gsign) ≥ GROUND_NY               is_wall(n) = |n.y| < GROUND_NY

move_body(m, delta, prm, q, jumped_this_tick) -> Contacts:
    was_grounded = m.mv.grounded
    if was_grounded ∧ m.mv.crouched ∧ prm.body.flags & EDGE_GUARD:              // §5.8
        for axis in [x, z]:
            d = delta with only that horizontal axis kept
            save = (m.p, m.v);  move_and_collide(m, d)
            if ¬ground_within(m, STEP_UP): (m.p, m.v) = save                     // would drop off a ledge
        delta = delta with x = z = 0                                           // vertical part follows
    save = (m.p, m.v)
    c = move_and_collide(m, delta)
    if was_grounded ∧ ¬jumped_this_tick ∧ (delta.x, delta.z) ≠ 0 ∧ c has a wall contact:
        try_step_up(m, save, c, delta)                                         // §5.6; keeps the better result
    (m.mv.grounded, m.mv.ground_ny, m.mv.ground_state) = ground_probe(m)       // §5.5
    if ¬m.mv.grounded ∧ was_grounded ∧ ¬jumped_this_tick ∧ m.v · gdir ≥ 0:
        snap(m)                                                                // §5.5: 550 mm along gravity
    m.mv.wall_n = horizontal unit of the deepest wall contact in c, or zero
    c

ground_within(m, dist) = a capsule copy moved by gdir × dist with move_and_collide ends with a ground contact
```

### 5.5 Ground detection and snap

- **Ground probe.** After movement, the body is grounded when the capsule shifted 2 mm along gravity (`GROUND_PROBE`) penetrates a box whose push normal has `n.y × (−gsign) >= GROUND_NY` (raw 3037000499, 45°). `ground_ny` is the largest such `n.y`, and `ground_state` is the state of that box's cell, ties to the first in gather order.
- **Snap.** If the body was in `ground` at the start of the tick, is not grounded now, `v.y` is zero or points along gravity, and no jump fired this tick, it sweeps 550 mm along gravity with `move_and_collide`. If that ends grounded, the snap is kept (walking down stairs and 0.5 m ledges stays on the ground). Otherwise the position reverts and the body enters `air`.
- `coyote` is set to 6 when a body leaves `ground` without jumping and decrements each tick in `air`.

### 5.6 Step-up

When a grounded body's horizontal movement records a contact with `|n.y| < GROUND_NY` (a wall), with `up_sign = −gsign`:

```
try_step_up(m, save, plain, delta):
    plain_state = (m.p, m.v, plain)                                  // result of the plain move
    (m.p, m.v) = save
    y_before = m.p.y
    move_and_collide(m, (0, STEP_UP × up_sign, 0))                  // rise; a ceiling stops it early
    rise = |m.p.y − y_before|                                        // 0..=550 mm actually risen
    move_and_collide(m, (delta.x, 0, delta.z))                       // the full horizontal delta again
    move_and_collide(m, (0, −(rise + GROUND_PROBE) × up_sign, 0))   // settle back down
    progress(s) = (s.p.x − save.p.x)² + (s.p.z − save.p.z)²          // i128 of raws
    if ground_probe(m).grounded ∧ progress(m) > progress(plain_state): keep m     // stepped up
    else: (m.p, m.v) = plain_state                                   // the plain move stands
```

A full 1 m block needs a jump. A 500 mm slab or stair step is climbed without one.

### 5.7 Jump and air jump

- A JUMP press (held now, not held last tick) sets `jump_buf = 6`; it decrements each tick.
- A jump fires when `jump_buf > 0`, the body is grounded or `coyote > 0`, and `can_jump` holds (§5.1). It sets `v.y = −gsign × v0(JUMP_H)`, clears `jump_buf` and `coyote`, and enters `air`. Canon §4.2's launch speed, evaluated exactly in this order: `v0(H) = sqrt_fx(mul_fx(mul_fx(Fx::from_int(2), g), fx_mm(H))) + Fx(floor_div(mul_fx(g, DT).raw, 2))`, with `JUMP_H` from `prm.vrv`.
- **Air jump.** An airborne body that presses JUMP while `air_jumps < prm.air_jump_max` (doc 04 §2.3: one for `vrv(e) >= 9`, one for `qinggong`, one for `aug.spring_tendons`) sets `v.y = −gsign × v0(mul_bp(JUMP_H, 6000))` and adds 1 to `air_jumps`.
- **Wall jump** (doc 04 §5.3, Acrobatics 50). An airborne body with `prm.wall_jump`, `wall_jumped == false` and a nonzero `wall_n` from the previous tick that presses JUMP removes the velocity component along `−wall_n`, adds `wall_n × fx_mm(4000)` to `v`, sets `v.y = −gsign × v0(mul_bp(JUMP_H, 8000))` and sets `wall_jumped`. When both a wall jump and an air jump are available, the wall jump fires.
- **Wall-run** (doc 04 §7.2, `wind_step`). While airborne with `wall_run_mm < prm.wall_run_max_mm`, a nonzero `wall_n`, horizontal speed at least 3000 mm/s and the horizontal velocity within 45° of parallel to the wall (`2 × (v_h · wall_n)² ≤ |v_h|²` in `i128` raws), gravity is skipped, `v.y` is raised to at least 0, and the horizontal distance moved that tick, in whole mm, is added to `wall_run_mm`. It ends when any condition fails.
- Landing, entering swim or entering climb resets `air_jumps`, `wall_jumped` and `wall_run_mm` to 0.

The apex tick counts the jump tick as tick 1, because the jump tick itself integrates gravity and position (§5.3). Each value below was recomputed with the canon primitives and the §2.1 integrator.

| Case | `g` (mm/s²) | `JUMP_H` (mm) | `v0` raw | Simulated apex (mm) | Apex tick |
|---|---|---|---|---|---|
| VERVE 0 | 20000 | 1100 | 29205417893 | 1099.972 | 20 |
| VERVE 5 | 20000 | 1250 | 31085832879 | 1249.874 | 21 |
| VERVE 10 | 20000 | 2000 | 39131183219 | 1999.922 | 27 |
| Wuxia, VERVE 10 | 16000 | 2000 | 34932400672 | 2000.000 | 30 |
| Wuxia air jump from rest | 16000 | 1200 | 27187601199 | 1199.874 | 23 |

### 5.8 Crouch, lean and edge guard

- CROUCH held sets `crouched`; the height becomes `crouch_h` immediately, keeping the origin. Releasing CROUCH stands only if a capsule of `stand_h` at `p` has no penetration; otherwise the body stays crouched.
- **Lean** (Canon §4.2): `lean` moves 1 per tick toward −9 (LEAN_LEFT), +9 (LEAN_RIGHT) or 0. The eye offset is `350 mm × lean / 9` along `right`, floored in mm. If the eye point would lie inside a box, `lean` steps back toward 0 that tick. The capsule never moves; the head hitbox follows the eye (§5.16).
- **Edge guard.** A grounded, crouched body with `EDGE_GUARD` moves X and Z separately; an axis move is cancelled when the capsule at the new position, swept 550 mm along gravity, finds no ground. Crouch-walking never drops off a ledge.

### 5.9 Mantle

Mantling climbs a ledge whose top is 550 mm (exclusive) to 1600 mm (inclusive) above the feet.

```
trigger: JUMP press ∧ fwd_in > 0 ∧ CAN_MANTLE ∧ gsign = −1 ∧ mode ∈ {ground, air, swim}
f      = horizontal unit of fwdh;  q = p + f × (r + 400 mm)          // probe column 700 mm ahead
probe  = capsule (r, crouch_h) at (q.x, p.y + 1650 mm, q.z), which must start without penetration, swept down 1100 mm
require: probe lands with ground normal; ledge h = land_y − p.y ∈ (550, 1600] mm
require: the landing box's cell has a shape other than grid.shape.fence and grid.shape.pane
require: capsule (r, h_cur) at (p.x, land_y + 1 mm, p.z) has no penetration         // room to rise
require: capsule (r, stand_h) at (q.x, land_y, q.z) has no penetration, else end crouched
         if crouch_h fits; else no mantle
total  = 8 + ceil_div(h_mm, 100) ticks
motion: ticks 1..=ceil_div(total × 6, 10) raise y linearly to land_y + 1 mm; the remaining ticks
        move x, z linearly to q; positions use floor division of the remaining distance
abort:  if the capsule penetrates any box on a mantle tick, restore the last valid position, enter air
```

A 1000 mm ledge takes 18 ticks, 1600 mm takes 24, 560 mm takes 14. A 500 mm ledge is a step-up (§5.6), and 1700 mm fails the test. Fences and panes are never mantle targets, so a 1500 mm fence holds anyone below VERVE 7, whose jump apex (1400 mm at VERVE 6) stays under it.

State machine of a mantle: `Mantle.t` runs from 1 to `total`; on each tick `t` the body takes the scripted position for `t`; at `t = total` the mode re-selects (§5.1) with `grounded` from a fresh ground probe; an abort on any tick restores the last valid position and enters `air` with `v = 0`. A movement gate (§5.1) or `st.downed` on a mantle tick also aborts it.

### 5.10 Swim

- `immersion_mm` is measured in the body's column: walk up from the feet cell; each cell with a liquid total of at least 990 units adds 1000; the first cell with less adds its total and ends the walk; then subtract `to_mm(p.y) − 1000 × floor_block(p.y)`, the feet's height inside the feet cell. The result is clamped to `0..=height` in mm.
- In `swim`: gravity is off. `wish = fwd3(yaw, pitch) × spd` for forward input plus strafe on `right`, with `spd = mul_bp(swim_base, row 11)` (`swim_base` 2400, or doc 05 §1.6's armor value) then × the dominant liquid's `move_bp` (doc 08: water 10000, oil 8000, lava 3000) → × `body_move_bp` → × each status multiplier in ascending status ID, one `mul_bp` each. Vertical target is +2400 mm/s with JUMP, −2400 with CROUCH, else +400 while the eye is submerged and 0 at the surface. Acceleration is 12000 mm/s² on all three axes toward the target.
- Wading (`300 <= immersion_mm < enter threshold`) multiplies ground speed by the dominant liquid's `move_bp`.
- A swimming body that presses JUMP facing a ledge runs the mantle test (§5.9).

### 5.11 Climb

- Climbable overlap: any cell with `climbable[state]` intersecting the capsule AABB expanded by 100 mm horizontally, between `p.y` and `p.y + h/2`.
- In `climb`: gravity off. `v.y = spd_c × sign(fwd_in)` with `spd_c = mul_bp(2400, row 21)` → × each status multiplier in ascending status ID (forward climbs up, back climbs down, no input holds); horizontal wish is capped at 1000 mm/s; CROUCH holds. JUMP leaves the ladder with `v.y = 4000` mm/s against gravity and `v_h = −fwdh × 2000` mm/s, and sets `climb_lock = 10` ticks. At GRP 60 the climb speed is 2640 mm/s.

### 5.12 Dodge, downed, flight, ghosts, gravity flip, mounts

- **Dodge** (Canon §4.2): direction = the horizontal unit of the wish velocity, or `−fwdh` with no input. For 12 ticks the horizontal velocity is set to 15000 mm/s along it; gravity still applies. Per-tick displacement is raw 1073741820, so the dodge covers 2.99999999 m. Doc 04 spends the stamina; doc 05 owns invulnerability.
- **Downed:** height 600 mm, crawl 800 mm/s × status multipliers, no jump, sprint or dodge.
- **Fly:** gravity off; `wish = fwd3(yaw, pitch) × spd` plus strafe; acceleration 20000 mm/s² on three axes. AI fliers write yaw and pitch into `control`.
- **Ghosts** (Canon §17.6) fly at walk speed with voxel collision and `NO_COLLIDE_BODIES`; they never appear in body, hit or perception queries (§3.6).
- **Gravity flip:** `gsign = +1` while the cell of the body center carries the `flip` bit (§4.7, doc 12 §9.5), else −1; `step_mover` re-reads it at the start of every tick, and `peak_y` restarts at the current position on the tick the sign changes. Every "up" in §5.3–§5.7 reverses, and the feet point becomes `p + (0, h, 0)`. Mantle, swim and climb are disabled under flipped gravity.
- **Mounts** (Canon §17.7): the rider's `control` drives the mount. Runner mounts use `ground`/`air` with walk 6000 mm/s and gallop 12000 mm/s (SPRINT; doc 05 spends mount stamina). Hover mounts keep their origin 500 mm above the higher of the ground top and the liquid surface below them, at up to 14000 mm/s, rising or falling at most 4000 mm/s toward that height. Glider mounts move 9000 mm/s horizontally and, while airborne, fall at exactly 2000 mm/s. After the mount moves, each rider's `p` is set to the mount's `p` plus the template seat offset rotated by the mount's yaw, and the rider's `vel` copies the mount's. Summoning checks a 2600 mm tall capsule of the mount's radius for penetration.

### 5.13 Knockback

```
apply_knockback(e, dir: unit Fx3, speed_mm_s):
    scale_bp = clamp(floor_div(75000 × 10000, mass_g(e)), 2000, 20000)
    eff      = mul_bp(mul_bp(speed_mm_s, scale_bp), 10000 − kb_resist_bp)
    v       += dir × fx_mm(eff)
    if eff >= 3000: kb_ticks = max(kb_ticks, 12); leave ground (mode air)
```

`kb_ticks` decrements each tick; while positive, control uses air acceleration. Example: 8000 mm/s on a 120 kg Erratum gives `scale_bp = 6250` and 5000 mm/s. Knockback is host-only: explosions (§12.4) and doc 05 hits call it between movement passes, and the client predictor receives its result as a correction (doc 16 §7.2).

### 5.14 Entity–entity push

For every pair of live bodies that both have `PUSHABLE` and `COLLIDES_BODIES` and share a broadphase cell, in ascending `(lower EntityId, higher EntityId)` order:

```
d_h     = horizontal (b.p − a.p);  dist = |d_h|
overlap = a.r + b.r − dist, only if > 0 and the vertical extents overlap
n       = d_h / dist, or (1, 0, 0) when dist = 0
push_a  = −n × min(50 mm, overlap × mass_b / (mass_a + mass_b))
push_b  =  n × min(50 mm, overlap × mass_a / (mass_a + mass_b))
move_and_collide(a, push_a); move_and_collide(b, push_b)
```

Two 75 kg bodies 500 mm apart (radii 300 mm) overlap by 100 mm and each moves 50 mm. A 300 kg ogre overlapping a 75 kg Erratum by the same 100 mm moves `100 × 75 / 375 = 20` mm while the Erratum moves 50 mm (its share, 80 mm, is capped at 50).

Every pair with `overlap > 0` before the push is appended to `push_contacts: Vec<(EntityId, EntityId)>` (lower ID first, pair order), cleared at `sys.tick_begin`. Doc 04's `juggernaut` and doc 09's contact rule read it in the same tick. Pairs where either body lacks `PUSHABLE` but both have `COLLIDES_BODIES` are recorded without moving, so a sprinting Erratum against an immovable boss still registers contact.

### 5.15 Falls, landing and footsteps

- `peak_y` resets to `p.y` every tick the body is grounded, swimming, climbing, mantling or has `st.levitating`; otherwise it keeps the extreme against gravity (Canon §4.2).
- On the transition into `ground`, `fall_mm = max(0, to_mm(peak_y − p.y))` when `gsign = −1` and `max(0, to_mm(p.y − peak_y))` when `gsign = +1`. `step_mover` returns it in `MoverOut.landed_fall_mm`. If the canon fall formula gives damage, `sys.movement` queues a hit for `sys.damage` with `B = mul_bp(canon_damage, genre.fall_dmg_bp)`, type `dmg.blunt`, flag `fall`, entering Canon §10.5 at step 4; doc 04 §8.1 splits it over the legs and applies its fall factors (Acrobatics, traits) after `fall_dmg_bp`. When `fall_mm >= 300`, `ev.entity_landed` is emitted. Example: a 6 m fall at VERVE 5 with Max HP 85 gives `excess` raw 8589934592, 1600 Bp, and 13,600 mHP before `fall_dmg_bp`.
- **Footsteps.** While in `ground`, `stride_mm` accumulates horizontal distance in mm. When it reaches the stride length (crouch 500, walk 700, sprint 900), it drops by that length and `ev.footstep` is emitted with the gait and the ground material. Doc 09 converts footsteps and landings into `ev.noise`.

### 5.16 Hitbox capsules from pose tables

Pose tables are content (Canon §9). This chapter owns the record schema; doc 18 owns the `art.anim.*` IDs, which key each record through its `anim` field. Each record gives, for one animation and body plan, keyframes every `key_every` ticks of every part capsule in model space (mm; origin at the entity origin; model −Z is forward). The loader expands keyframes into the canon per-tick table: for `k·K <= t < (k+1)·K`, each coordinate is `a + floor_div((b − a) × (t − k·K), K)`, and looping animations interpolate from the last key to the first.

```json
{ "anim": "art.anim.humanoid.walk", "body": "body.humanoid", "length_ticks": 40, "loop": 1, "key_every": 10,
  "parts": [
    { "part": "bp.head",  "keys": [[0,1520,0, 0,1720,0, 120], [0,1515,-20, 0,1715,-20, 120],
                                    [0,1520,0, 0,1720,0, 120], [0,1515,20, 0,1715,20, 120]] },
    { "part": "bp.arm_r", "keys": [[250,1000,0, 250,1450,0, 70], [250,1000,-120, 250,1450,0, 70],
                                    [250,1000,0, 250,1450,0, 70], [250,1000,120, 250,1450,0, 70]] } ] }
```

Each key is `[ax, ay, az, bx, by, bz, radius]`. World transform of a model point `(x, y, z)`, with `c = cos_bam(yaw)`, `s = sin_bam(yaw)` in Q2.30 and model values scaled by `hitbox_scale_bp`:

```
wx = p.x + Fx((x·c + z·s) >> 30)      wy = p.y + y      wz = p.z + Fx((−x·s + z·c) >> 30)
```

The head capsule also shifts by the lean offset (§5.8). Example: model point (250 mm, 1000 mm, 0) at yaw 4096: `x` raw 1073741824, `wx` offset raw 992008094 (0.2310 m), `wz` offset raw −410903207 (−0.0957 m); at `p = (10, 1, 10)` the point is (10.2310, 2.0, 9.9043).

- `sys.pose` advances `anim_tick` by 1 (wrapping for looping, clamping at `length_ticks − 1` otherwise); an animation change sets `anim_tick = 0`. Doc 05 and doc 13 choose animations.
- It then writes `PoseRec { tick: t, p, yaw, anim, anim_tick, lean }` at `hist[head]` and advances `head` modulo 32.
- **Rewind** (doc 05 §2.6, doc 16 §8). For a command applied at tick `t`, `rewind = min(t − view_tick, LAGCOMP_MAX_TICKS)` with `LAGCOMP_MAX_TICKS = 16` (doc 05), and `t_r = t − rewind`. A rewound test reconstructs each target's capsules from its record with `tick == t_r`. A target whose `spawn_tick > t_r` has no such record and is skipped by that test (doc 16 G6). Because 16 < 32, the record always exists for older targets. Rewound tests do not use the broadphase: they test every live, non-ghost entity with `pose` whose rewound AABB, grown by the query's reach, overlaps the query segment's AABB, in ascending `EntityId`. The canon caps bound that scan at 256 actors plus 4 Errata.

### 5.17 Geometric primitives

This chapter uses doc 02 §4.6 exactly: `closest_seg_seg`, `capsule_overlap`, `ray_capsule` (with `r` inflated by the projectile radius for thick projectiles) and `dda`. A hitscan or projectile segment hits the hitbox part whose `ray_capsule` returns the smallest `t`; ties go to the part listed first in the body plan (Canon §9).

### 5.18 Voxel raycast

`raycast_blocks(o, d, t_max)` wraps doc 02 `dda`:

1. Components of `d` with `|raw| < 65536` are set to 0 before the call, so `div_fx(ONE, d_i)` never exceeds raw `2^48`.
2. For each visited cell: a `full[state]` cell is a hit at the DDA `t` with the entered face. A cell with partial boxes runs the slab test of the ray against each box (entry `t = max` of the per-axis entry times, exit `t = min` of the exits, axis ties x, y, z); the smallest entry `t >= t_cell` is a hit with that box's entered face. A cell with no boxes continues.
3. The start cell is tested first; a ray starting inside a box hits at `t = 0` with face `None`.

Worked example: origin (5.5, 2.62, 12.5) m, `yaw = 4096`, `pitch = −2048`, so `d = fwd = ` raw (−1612031272, −837906552, −3891787748). Initial `t_next` raws are 5721583816 (x), 13649471168 (y), 2369957622 (z); `t_delta` raws 11443167632, 22015276082, 4739915244. Stone at (4, 1, 8), floor at y ≤ 0. Visits: (5,2,11) at 0.551799 m through +Z; (4,2,11) at 1.332160 through +X; (4,2,10) at 1.655396; (4,2,9) at 2.758994; (4,1,9) at 3.178015 through +Y; (4,1,8) at raw 16589703354 (3.862591 m) through +Z: a hit at (4.0503, 1.8664, 9.0).

### 5.19 Projectile sweep

`sys.projectiles` processes projectiles in ascending `EntityId`. This chapter owns integration, geometry, catch-up and lifetime; doc 05 §7 decides what each contact does through four hooks, so the two chapters never disagree about where a projectile is.

```
sys.projectiles, for each projectile pr, ascending EntityId:
    if t >= pr.end_tick: despawn(pr, grid.despawn.expired); continue
    if pr.age == 0:                                             // Erratum catch-up (doc 05 §2.6, C2)
        for i in 0..pr.catchup: proj_step(pr, pose_tick = t − pr.catchup + i + 1)
    proj_step(pr, pose_tick = t)                                // the normal step (C4)
    pr.age += 1

proj_step(pr, pose_tick):
    if pr.resting or pr is despawned: return                    // a resting grenade waits for its fuse
    gs  = if flip(cell_of(p)) { +1 } else { −1 }                // §4.7
    gv  = mul_fx(fx_mm(mul_bp(mul_bp(20000, genre_phys(cell_of(p)).gravity_bp), pr.gravity_bp)), DT)
    v.y = if gs == −1 { max(v.y − gv, −fx_mm(50000)) } else { min(v.y + gv, fx_mm(50000)) }   // doc 05 §7.1
    v   = v − mul_bp_fx(v, pr.drag_bp) per axis                 // drag_bp 0 for every doc 05 ammo family
    a   = p;  b = p + (mul_fx(v.x, DT), mul_fx(v.y, DT), mul_fx(v.z, DT))
    loop:
        w = first_world_event(a, b, pr.radius)                  // solid-box entry or liquid-cell entry, nearest
        e = first_body_event(a, b, pr.radius, pose_tick, pr)    // nearest hitbox capsule entry
        take the smaller entry distance from a; on a tie the body event wins (doc 05 §2.4)
        nothing:            p = b; break
        body (target, part, point):
            match doc05::on_body_hit(pr, target, part, point):  // queues the HitRecord
                Continue => insert target into pr.combat.hit_set; a = point
                Stop     => doc05::on_stop(pr, point); despawn(pr, grid.despawn.consumed); return
        liquid cell c entered at point:
            doc05::on_liquid_enter(pr, c)                       // one reaction trigger per cell; speed × 3000
            a = point
        solid box of cell c entered through face f at point:
            match doc05::on_block_enter(pr, c, f, point):       // wall penetration and block damage (§13.2)
                Penetrate => a = point + unit(b − a) × 1 mm
                Ricochet  => v = doc 05 §7.5 reflection; a = point + n_f × (radius + 1 mm); b = a + remaining part of the step, reflected
                Bounce    => v.axis(f) = −mul_bp_fx(v.axis(f), pr.bounce_bp); other axes × 8000;
                             p = point + n_f × (radius + 1 mm);
                             if |v| < fx_mm(500): pr.resting = true; v = 0
                             break
                Stop      => doc05::on_stop(pr, point); despawn(pr, grid.despawn.consumed); return
    portal crossing of the segment a → p (§5.21)
    doc05::after_step(pr, moved_mm)                             // traveled_mm, range and age-600 limits (doc 05 §7.1)
```

- `first_world_event` with `radius = 0` walks `raycast_blocks` (§5.18) over `[a, b]`, reporting the first solid-box entry and every liquid cell (at least 1 unit) entered before it. With `radius > 0` it uses `sphere_substeps`: the sphere is tested at each substep end, with step `max(radius, 100 mm)`, against gathered boxes with doc 02's box distance; the first overlapping substep is the event, located at the previous substep position.
- `first_body_event` tests `ray_capsule(a, unit(b − a), c0, c1, r_part + radius)` against the hitbox capsules of every candidate posed at `pose_tick`, excluding ghosts, the projectile's `hit_set`, and the owner until doc 05 marks it hittable after a ricochet. Candidates come from the broadphase when `pose_tick = t` and from the rewound scan of §5.16 otherwise. A hit counts when its `t` is at most `|b − a|`.
- `catchup` is set at spawn: `min(t − view_tick, LAGCOMP_MAX_TICKS)` for a projectile whose owner is an Erratum, else 0. A projectile therefore advances `catchup + 1` steps on its spawn tick and one step per tick afterwards.
- `bounce_bp` is 0 for every projectile except fused grenades (doc 05 §9.2: 4000). `PROJECTILE_CAP` eviction follows Canon §4 at spawn time.

Example: an Erratum fires on tick 1000 with `view_tick` 994. `catchup = 6`, so the projectile takes catch-up steps against poses of ticks 995, 996, 997, 998, 999 and 1000, then its normal step against tick 1000's poses: 7 steps on the spawn tick, then 1 per tick.

### 5.20 Environment sampling

`sys.env_sample` fills `comp.env` for every awake body:

- `genre = local_genre(p)` (Canon §5.8). A change triggers doc 05's Contradiction re-evaluation and `ev.contradiction_changed`.
- Contact cells: feet = cell of `p + (0, 50 mm, 0)`, body = cell of `p + (0, h/2, 0)`, eye = cell of the eye point. Each records up to 3 liquids with units, up to 2 gases with concentration, and whether the liquid is charged (§6.9).
- **Liquid contact** (Canon §12.1, "at least 100 ml"): any liquid with at least 1 unit (1 unit = 1000 ml, §6.1) in the feet or body cell.
- **Gas contact:** a gas with concentration at least 500 in the eye cell.
- `eye_submerged`: the eye cell's liquid surface `cell_y + total/1000 m` is above the eye point. Doc 04 drains `res.breath` while `eye_submerged` or while the eye-cell gas is unbreathable (doc 08 flag).
- `temp` is the thermal cell of the body point (§9). `light_sky`, `light_block` are those of the cell at `p + (0, 0.5 m, 0)` (Canon §2.3 `light_at`), each clamped by an active light override (§10.6). `in_fire` is true when any burning cell intersects the capsule AABB expanded by 100 mm.

### 5.21 Portals (relink pairs)

Doc 12 §9.4 places relink door pairs in Cosmic Horror (2 pairs, period 3600 ticks) and Oneiric (1 pair, period 1800). Each pair joins two horizontal doorways through portal planes while it is in state 1.

```rust
pub struct PortalPlane { pub cell: Gidx /* lower cell of the 1 × 2 doorway */, pub facing: u8 /* 0 N, 1 W, 2 S, 3 E */ }
pub struct PortalPair { pub a: PortalPlane, pub b: PortalPlane, pub period: Ticks, pub start: Tick /* folio_entry_tick */ }
// state(t) = floor_div(t − start, period) & 1; state 0 joins nothing, state 1 links a and b
```

- **Plane.** A doorway at cell `(x, y, z)` with facing `f` has its plane through the cell center perpendicular to the facing axis: `z = z + 0.5` for facing N or S, `x = x + 0.5` for W or E. The opening spans the cell's width along the other horizontal axis and `y .. y + 2` vertically.
- **Crossing.** A body (its origin) or a projectile (its position) crosses plane `a` on a tick when the segment from its position at the start of its move to its position at the end changes sides of the plane strictly inside the opening. Pairs are tested in ascending index, and only the first crossing on a tick applies.
- **Transfer.** With `Δ = (b.facing − a.facing) & 3` quarter turns and `R` the exact rotation by `Δ × 16384` Bam about +Y (a quarter turn maps `(x, z) ↦ (z, −x)`, applied `Δ` times), the crossing body moves to `center(b) + R(p − center(a))`, where `center` is the doorway's floor center `(x + 0.5, y, z + 0.5)`. Its velocity and `wall_n` are rotated by `R`, its yaw gains `Δ × 16384`, `peak_y` shifts by the same vertical offset, and `TELEPORTED` is set. Crossing `b` maps back to `a` with `−Δ`. The rotation keeps the side: a body leaving `a`'s front exits `b`'s front.
- **Scope.** Only bodies and projectiles use portals. Light, sound, liquids, gases, temperature, fire and every perception or hit ray use physical adjacency only, so in state 1 a doorway shows the room physically behind it while leading elsewhere.

Example: pair `a` = cell (40, 2, 20) facing N, `b` = cell (100, 2, 64) facing E, `Δ = 3`. A body moving north crosses `a`'s plane `z = 20.5` and ends the tick at `(40.3, 2.0, 20.4)`, so `p − center(a) = (−0.2, 0, −0.1)`. Three quarter turns map `(x, z)` to `(z, −x)` three times: `(−0.2, −0.1) → (−0.1, 0.2) → (0.2, 0.1) → (0.1, −0.2)`. The body appears at `(100.6, 2.0, 64.3)` with its yaw increased by 49152 Bam.

### 5.22 Resting bodies

A body without control input (an item drop or a corpse) whose velocity is zero and whose ground probe finds a ground contact with `ground_ny = Fx::ONE` (a face exactly against gravity under its center) skips integration on that tick: `step_mover` returns at once with its state unchanged. Any impulse (knockback, push, a portal), any velocity change or a ground probe that fails ends the rest on that tick. The probe touches at most the 4 cells under the capsule bottom, so a resting body costs at most 0.2 µs, and the 2,048 ground items of `GROUND_ITEM_CAP` stay inside the movement budget (§17).

---

## 6. Liquids

### 6.1 Units and cell record

- One **liquid unit** is 1 litre = 1000 `Ml`. A full cell (1 m³) holds `FULL = 1000` units. Quantities authored in ml convert with `units = floor_div(ml, 1000)`; the remainder is not deposited.
- `FILM_MAX = 20` units: a cell holding at most 20 units is a film that never spreads sideways.

```rust
pub struct LiquidCell {
    pub comps: [(u16 /* liq type index */, u16 /* units */); 3], pub n: u8,  // sorted by density desc, then id32 asc
    pub source: bool,            // worldgen spring: refills to FULL of comps[0] every run
    pub owner_slot: u8,          // party slot whose action last deposited here; 0xFF none (§6.9, Canon §17.5)
    pub charged_until: Tick, pub charged_slot: u8,   // electrified body (§6.9)
    pub last_run: u32,           // liquids run that last evaluated this cell (fairness, §6.3)
}
```

A cell's total `V = Σ units ≤ liquid_cap(state)`. At most 3 liquids share a cell; a fourth liquid cannot enter a full set of 3 (that share of a transfer stays behind). At most 131,072 liquid cells exist per Folio; a transfer or deposit that would create a cell beyond the cap is refused.

### 6.2 Capacity

| `liquid_interaction` | `liquid_cap(state)` |
|---|---|
| any, when the shape is `grid.shape.none` (air) | 1000 |
| `grid.liqi.blocks` | 0 |
| `grid.liqi.permeable` | `shape_liquid_cap` (§4.4): slab 500, stairs 250, layer `1000 − 125 × (level + 1)`, others 1000 |
| `grid.liqi.displaced` | 1000; the first transfer into the cell breaks the block with cause `grid.cause.liquid` and its normal drops (destructible blocks only, §4.5 loader rule 2) |
| `grid.liqi.absorbs` | 0; the block absorbs film from the cell above (§6.4 step 8) |

### 6.3 Activation, order and budget

- `liquid_active: BTreeSet<Gidx>` holds cells that changed or neighbor a change. At Folio entry it holds every liquid cell. A cell is processed only when its chunk is active (§15). A cell in an active chunk transfers into a neighbor in an inactive chunk normally; the receiving cell joins `liquid_active` and waits until its chunk activates.
- Liquids always flow toward −Y. The gravity-flip bit (§4.7) does not affect them.
- **Run order.** Let `r` be the liquids run index (`floor_div(t + 1, 6)`). Cells are taken in ascending `y`; within a layer, even runs use ascending `(z, x)` and odd runs descending `(z, x)`. The run iterates a copy of the active set taken at run start; cells activated during the run wait for the next run, but a cell later in the copy sees volume that arrived earlier in the same run.
- **Budget.** At most 16,384 cells are evaluated per run. When the active set is larger, the 16,384 cells with the smallest `(last_run, order key)` are chosen and processed in run order; the rest stay active. A waiting cell is therefore served within `ceil(|active| / 16384)` runs.

### 6.4 Cell step

```
step_cell(c):
 1  if c.source: set comps = [(source liquid, FULL)], n = 1
 2  reactions: for each component pair (i < j), then each component with the cell's block and the block
    below, call doc 08 alch_react(...) (§6.8); apply at most 1 reaction per cell per run
 3  temperature T = thermal cell of c (§9):
      for each component k: if T <= freeze_decic(k) ∧ units_k >= 500 ∧ state(c) is air:
          timed_revert[c] = Revert { at: None, liq: k, units: units_k }   // §6.9; melting restores it
          set_block(c, freeze_block(k), grid.cause.froze); delete all liquid in c; return
      if T >= boil_decic(k): units_k −= 1; emit_gas_volume(c, boil_gas(k), 1, c.owner_slot)
                                                       // 1 unit = 1 saturated cell (§7.4), 10 units per s
 4  DOWN: below = c − Y; free = liquid_cap(below) − V(below)
      fl  = volume-weighted flow of c = floor_div(Σ units_k × flow_eff_k, V)       // flow_eff: §6.5
      amt = min(V, free, mul_bp(FULL, fl))
      move amt into below, taking components heaviest first (density desc, id32 asc)
 5  SWAP: if V(below) = cap(below) ∧ c's heaviest density > below's lightest density:
      s = min(units of c's heaviest, units of below's lightest, mul_bp(200, min(flow_eff of both)))
      exchange s units of the two components
 6  SPREAD (only if V > FILM_MAX and below is full or blocked): V0 = V
      for n in (N4A if r even else N4B), skipping cells with cap 0:
          diff = V0 − V(n); if diff < 2: skip
          tr = floor_div(diff × fl, 50000)                       // 1/5 of the difference × flow
          if tr = 0 ∧ fl >= 5000: tr = 1                         // EQUALIZE: level the last units
          tr = min(tr, cap(n) − V(n)); move tr into n, proportional (below)
 7  EVAPORATION (V <= FILM_MAX): for each component with evap_period p > 0:
      if (r + c) % p == 0: units −= 1; if doc 08 gives the liquid an evap_gas {gas, conc}:
          emit_gas(c, gas, conc, c.owner_slot)
 8  ABSORPTION (V <= FILM_MAX ∧ below has grid.liqi.absorbs): remove min(V, absorb_per_run) units proportionally
 9  delete components at 0 units, delete the cell at V = 0
10  if anything changed: insert c and every N6 neighbor with cap > 0 into the next active set
    else: drop c from the active set (settled)
```

**Proportional extraction** of `tr` units from components `u_k`: `take_k = floor_div(tr × u_k, V)` in component order; the remainder goes to the component with the most units, ties to the lower `id32`. A receiving cell whose `owner_slot` is `0xFF` takes the sender's `owner_slot`. Example: 200 units from water 600 + oil 300 takes 133 + 66 = 199, plus 1 to water: 134 water, 66 oil. The volume-weighted flow of that cell (water 10000, oil 7000) is `floor_div(600 × 10000 + 300 × 7000, 900) = 9000`.

There is no pressure: liquid never rises except through `SWAP` and sources.

### 6.5 Liquid properties consumed from doc 08

| Field | Type | Used in | Reference values (doc 08 owns) |
|---|---|---|---|
| `density_g_per_l` | i32 | layering, step 4–5 | water 1000, oil 900, lava 3100, acid 1200 |
| `flow_bp` | Bp | steps 4–6 as `flow_eff = clamp(mul_bp(flow_bp, liquid_viscosity_bp), 500, 10000)`, with doc 12 §2.3's `liquid_viscosity_bp` of the cell's local genre (default 10000) | water 10000, oil 7000, blood 8000, honey 2500, lava 1500 |
| `move_bp` | Bp | swim and wading speed (§5.10) | water 10000, oil 8000, lava 3000 |
| `evap_period` | runs | step 7; 0 = never | water 50, alcohol 10, oil and lava 0 |
| `freeze_decic`, `freeze_block` | DeciC, `block.*` | step 3 | water 0 → `block.ice` |
| `boil_decic`, `boil_gas` | DeciC, `gas.*` | step 3 | water 1000 → `gas.steam` |
| `temp_decic`, `heat_drive_bp` | DeciC, Bp | heat source (§9.2) | lava 11000, 2000 |
| `light_emit` | 0..=15 | light source (§10.2) | lava 15 |
| `fire_fuel` | flag | liquid fire (§8) | oil, alcohol 1 |
| `extinguishes` | flag | §8.5 | water 1 |
| `conductive` | flag | electrify trigger (§6.9) | water, blood, alcohol 1 |
| `conceal_per_cell` | 0..=15 | sight (§7.6) | water 2 |
| `evap_gas` | `{gas, conc}` or absent | step 7 | swamp water: `gas.miasma` 500 |

### 6.6 Worked examples

A 1 × 5 channel at y 1 over solid floor, walls on both sides, in a genre with `liquid_viscosity_bp` 10000; water (flow 10000) 1000 units at x 2; all other cells empty. The first run has an even run index `r`. The table below was recomputed by simulating §6.4 exactly.

| After run | x 0 | x 1 | x 2 | x 3 | x 4 |
|---|---|---|---|---|---|
| 1 (even, active {2}) | 0 | 200 | 600 | 200 | 0 |
| 2 (odd, order x 4, 3, 2, 1, 0) | 56 | 224 | 432 | 248 | 40 |
| 19 (settled; active set empty after this run) | 200 | 200 | 201 | 200 | 199 |

Run 2 in detail: x 3 (200) sends `floor_div(200 × 10000, 50000) = 40` to x 4; x 2 (600) sends 80 to x 1 (200) and 88 to x 3 (160); x 1 (280) sends 56 to x 0. The total stays 1000 every run. The same start with lava (flow 1500) gives 30, 940, 30 at x 1, 2, 3 after run 1. A full water cell above an empty cell moves all 1000 units down in one run; lava moves 150.

### 6.7 Sources and sinks

The automaton conserves volume. Only these operations add or remove liquid, each in ascending cell order:

| Operation | Effect |
|---|---|
| `deposit(c, liq, units, slot)` | adds up to the cell's free capacity, sets `owner_slot = slot`, activates `c`; the rest is not deposited |
| `splash(center, liq, units, slot)` | doc 08 thrown containers: deposits `max(1, floor_div(units, n))` units into each of the `n` air cells with a solid cell below within a 1-block horizontal radius of `center`, in ascending cell order, `n <= 9` |
| `drain(c, units)` | removes proportionally (doc 07 pumps, doc 08 bottles) |
| block placed into a liquid cell | the liquid above the new `liquid_cap` is deleted |
| evaporation, absorption, boiling, freezing, reactions, fire fuel use | §6.4, §8.4 |

### 6.8 Reaction hook

```rust
/// Doc 08 implements. Pure: draws nothing, reads only its arguments.
fn alch_react(a: (Id32, u16), b: ReactPartner /* Liquid(Id32, u16) | Block(BlockState) */, temp: DeciC)
    -> Option<Reaction>;
pub struct Reaction {
    pub rule: Id32,                         // alch.* (doc 08)
    pub consume_a: u16, pub consume_b: u16, // units; a consumed block is replaced by `set_block`
    pub produce: [(Id32, u16); 2],          // liquids added to the cell
    pub gas: [(Id32, u16); 2],              // emit_gas into the cell
    pub set_block: Option<BlockState>,      // for the cell (air only) or for the partner block
    pub heat_decic: i32, pub ignite: bool, pub explosion: Option<ExplosionSpec>,
}
```

`sys.liquids` applies the outcome in field order and emits `ev.liquid_reaction`. Lava meeting water is the canonical example: doc 08's rule consumes both and sets the cell to a stone block.

### 6.9 Surface triggers (Canon §12.1)

`sys.damage` and `sys.damage_late` call `surface_trigger(cells, dmg_type, instigator_slot)` for every hit whose type shares include a trigger type. Doc 05 §7.3 also calls it once per liquid cell a projectile enters, and doc 06 §13.1 for the cells a spell touches. Doc 08 §5.2 extends the rows below to every liquid by its `triggers` data and adds the fuel and mana blasts in the oil row's form; doc 08 §5.6 routes `liq.nitro` detonation into §12 explosions. Trigger cells are: for a point hit, the cell of the impact point and the cell below it; for area damage, the area's center cell and its `N6` neighbors. Each trigger emits one `ev.liquid_reaction`.

| Trigger | Grid effect |
|---|---|
| shock on a conductive liquid | BFS over `N6` from each trigger cell through cells holding that liquid (≥ 1 unit), ascending visit order, at most 64 cells: `charged_until = t + 180`, `charged_slot = instigator_slot` |
| frost on water, blood or alcohol | each trigger cell that is air and holds ≥ 100 units of it becomes `block.ice` with cause `grid.cause.froze`; the cell's liquid is stored in `timed_revert[c] = Revert { at: Some(t + 1200), liq, units }` and deleted; other components are deleted |
| fire on water | each trigger cell converts `k = min(units, 27)` water units into `k` full steam cells: `emit_gas_volume(c, gas.steam, k)` |
| fire on oil or alcohol | each trigger cell ignites as a liquid fire (§8.2); one blast per hit at the impact cell: radius 2000 mm, environmental `dmg.fire`, base `10,000 mHP × min(units, 5)`; `min(units, 5)` units are consumed |
| fire on ice | `block.ice` in a trigger cell reverts now: a `timed_revert` entry restores its liquid; otherwise the block's `melt_units` of `melt_to` (doc 07 §10.2: 1000 units of `liq.water`) |
| fire on web | `block.web` ignites as a block fire (its registry `burn_runs` is 5, which is 60 ticks) |
| fire on miasma | if a trigger cell holds `gas.miasma` ≥ 1000: one blast per hit, radius 3000 mm, environmental `dmg.fire` base 20,000 mHP; the miasma in the trigger cells is removed |
| radiant on red ink | every `liq.red_ink` unit in the trigger cells becomes `liq.water` |

`timed_revert: BTreeMap<Gidx, Revert { at: Option<Tick>, liq: Id32, units: u16 }>` records the liquid inside every ice cell made from liquid. `sys.block_updates` reverts entries whose `at` is due: the ice becomes air (cause `grid.cause.reverted`) and the stored liquid is deposited back. Temperature never melts a cell whose entry has `at = Some(_)`. A cell frozen by temperature (§6.4 step 3) has `at = None` and melts by §9.5, which deposits the stored units instead of `melt_units`. Breaking or replacing the ice by any other cause removes the entry. Freezing and melting therefore never create liquid: a freeze–thaw cycle returns exactly the units that froze, and the other components deleted at freezing stay deleted. Blasts are explosions without block damage (`block_power_milli = 0`, §12.1) and enter the pipeline as environmental hazards with `instigator_slot`.

### 6.10 Surface contact (`sys.surfaces`)

On status-pulse ticks (`t % 15 == 0`), for every awake body in ascending `EntityId`, using `comp.env` contact (§5.20), apply Canon §12.1 rows in this order: liquids in ascending `id32`, then gases in ascending `id32`, then the charged flag, then blocks:

| Contact | Applied as |
|---|---|
| status rows (`st.soaked`, `st.oiled`, `st.rooted` for web) | direct application (Canon §11.4); `st.rooted` lasts while inside |
| buildup rows (acid 25 `st.corroded`, miasma 20 `st.poisoned`, lava 100 `st.burning`, charged 30 `st.shocked`) | Canon §11.2 environmental buildup: `p × S(d_current) × (10000 − r)`, `r` the bearer's resistance to the status's carrier type |
| DoT rows (lava 1000 Bp `dmg.fire`, steam 50 Bp `dmg.fire`) | Canon §11.3 with stacks 1, potency 10000, rank DoT multiplier; `q` from the per-surface accumulator in `Env.contact_acc` with `k = 4`; queued for `sys.damage_late` with flags `dot`, `env`, `area` |
| red ink | `red_ink_ticks += 15`; at each 300, +1 `st.erasure` |
| movement rows (oil friction, ice) | §5.3, every tick |

Hits and buildup from a cell with `owner_slot ≠ 0xFF` carry that slot as instigator, so doc 05 applies `FF_AREA_BP` (Canon §17.5).

Doc 08 §3.2 and §4.2 add contact rows for every other liquid and gas, and doc 08 §5.4 adds status-driven triggers evaluated after the contact rows. They run in the same order, through the same four kinds of application, from data. Surface contact for an asleep body is skipped (§15.2).

---

## 7. Gases

### 7.1 Cell record

```rust
pub struct GasCell { pub comps: [(u16 /* gas type index */, u16 /* conc 0..=10000 */); 2], pub n: u8,
                     pub owner_slot: u8, pub last_run: u32 }
```

Concentration 10000 is a cell saturated with that gas, 1,000,000 ml (doc 08 §1.2). The concentrations of a cell's components sum to at most 10000; every transfer and emission is capped by the receiving cell's free room `10000 − Σ conc`. Gases move only through cells with `gas_open[state]` (§4.5). A third gas cannot enter a cell holding two; that share stays behind. At most 65,536 gas cells exist per Folio; flow or emission that would create a cell beyond the cap is refused.

### 7.2 Gas properties consumed from doc 08

| Field | Meaning | Reference values (doc 08 owns) |
|---|---|---|
| `diffuse_bp` | share of the difference sent per face per run, over 7 | smoke 6000, steam 8000, miasma 5000 |
| `rise_bp` | share moved up (positive) or down (negative) per run | smoke 2000, steam 3000, miasma −1000 |
| `dissip_bp` | decay per run, used as `dissip_eff = mul_bp(dissip_bp, gas_dissipation_bp)` with doc 12 §2.3's `gas_dissipation_bp` of the cell's local genre (default 10000) | smoke 150, steam 400, miasma 100 |
| `conceal_full` | `conceal` at concentration 10000 (Canon §12.2) | smoke 4, steam 3 |
| `unbreathable` | flag for breath (Canon §12) | miasma 1 |
| `condense_decic`, `condense_to` | below this temperature the gas becomes liquid at 1 unit per 10000 conc (one saturated cell per unit, the inverse of boiling, §6.4 step 3) | steam 900 → `liq.water` |

### 7.3 Gas run

Runs on `(t + 4) % 6 == 0`, with run index `floor_div(t + 4, 6)`. Active cells (any gas) in active chunks are taken in ascending `Gidx` on even runs and descending on odd runs, with the same 16,384-cell budget and `last_run` fairness as liquids (§6.3).

```
for each component k of cell c (component order):
  1 vertical: n = c + Y if rise_bp > 0 else c − Y; if n is gas-open, in bounds, can hold k
              and conc_k(n) < conc_k:
                  m = min(floor_div(conc_k × |rise_bp|, 10000), room(n)); move m to n
  2 diffusion: c0 = conc_k after step 1
              for n in N6 (gas-open, in bounds, can hold k): diff = c0 − conc_k(n); if diff > 0:
                  f = min(floor_div(diff × diffuse_bp, 70000), room(n)); move f to n
  3 dissipation: conc_k −= min(conc_k, max(1, floor_div(conc_k × dissip_eff × open_mult, 10000))),
              open_mult = 4 when the cell's open_tag or sky_src bit is set, else 1
  4 condensation: if thermal T < condense_decic:
              u = floor_div(conc_k, 10000); if u > 0: deposit(c, condense_to, u, owner_slot); conc_k = 0
room(n)  = 10000 − Σ conc of n's components;  "can hold k": n holds k already or holds fewer than 2 gases
wake: changed cells and their gas-open N6 neighbors join the next active set
```

Transfers only flow from higher to lower concentration, so each face moves gas one way and the total changes only through dissipation and condensation. Condensation removes the whole component and returns at most the units that boiling (§6.4 step 3) or a fire trigger (§6.9) turned into it, so no cycle of boiling and condensing ever creates liquid.

**Example.** Smoke 7000 in an indoor cell (no `open_tag` or `sky_src` bit, genre `gas_dissipation_bp` 10000) with open air on five sides and a floor below: rise moves `floor_div(7000 × 2000, 10000) = 1400` up, leaving 5600. Diffusion sends `floor_div(5600 × 6000, 70000) = 480` to each of ±X and ±Z, and `floor_div(4200 × 6000, 70000) = 360` up (the cell above now holds 1400), leaving 3320. Dissipation removes 49, leaving 3271.

**Example (steam balance).** Fire on a cell holding 3 units of water converts them into 3 saturated steam cells (§6.9). If those cells drift into a thermal cell below 900 DeciC before dissipating, each condenses back into `floor_div(10000, 10000) = 1` unit; a cell that has already dissipated to 9,999 yields 0. The water returned is at most the 3 units that boiled.

### 7.4 Emission

- `emit_gas(c, gas, conc, slot)` adds `min(conc, room(c))` when `c` can hold the gas; the excess is dropped. A receiving cell whose `owner_slot` is `0xFF` takes `slot`.
- `emit_gas_volume(c, gas, k, slot)` runs an `N6` breadth-first search over gas-open, in-bounds cells from `c` (neighbors in `N6` order, FIFO), visiting at most 27 cells. Each visited cell that can hold the gas receives `room(cell)` of it and counts as filled; the search stops after `k` filled cells. One liquid unit equals one filled cell (§6.4 step 3, §6.9).

### 7.5 Gas ignition

A gas whose doc 08 record has a blast (Canon §12.1: `gas.miasma`) ignites when fire damage triggers it (§6.9) or when a burning cell is `N6`-adjacent to a cell holding it at ≥ 1000 during a fire run (§8.3). At most 4 gas blasts resolve per fire run, in ascending cell order; the rest wait for the next run. Each blast is an explosion without block damage (§12.1), consumes the gas in its cell, and leaves a flash fire (fuel 1) there.

### 7.6 Concealment helper

```
cell_conceal_parts(c) -> (block, liquid, gas):
    block  = conceal[state(c)]
    liquid = if V(c) >= 500 { min(15, Σ conceal_per_cell of components with ≥ 500 units) } else { 0 }
    gas    = Σ over gas components floor_div(conceal_full × conc, 10000)
cell_conceal(c) = min(15, block + liquid + gas)
```

Doc 09 sums `cell_conceal` along sight rays from `raycast_blocks`, excluding both endpoint cells (Canon §12.2). Its `thermal` sense reads the three parts separately and drops the gas term (doc 09 §4.6). Example: a cell of 600 units of water (conceal 2) holding smoke at 5,000 (conceal 4) gives `(0, 2, 2)` and `cell_conceal` 4.

---

## 8. Fire

### 8.1 Fire cell

```rust
pub struct FireCell { pub kind: FireKind /* Block | Liquid | Flash */, pub fuel: u16 /* fire runs left */,
                      pub age: u16, pub intensity: u8 /* 1..=3 */, pub instigator: Option<EntityId> }
```

`fires: BTreeMap<Gidx, FireCell>`, at most 4,096 entries; ignition at the cap is refused. A block fire lives in the burning block's own cell, a liquid fire in the cell holding the fuel liquid.

```
ignite(c, kind, instigator) -> bool:                 // the one entry point; docs 05, 06 and 08 call it too
    refuse if out of bounds, already burning, or |fires| = 4096
    Block:  requires flammability_bp(state(c)) > 0 (so the block is destructible, §4.5 rule 2)
    Liquid: requires a fire_fuel liquid with ≥ 1 unit in c
    Flash:  requires c to be gas-open
    refuse if c or the cell above holds ≥ 10 units of an extinguishing liquid
    fires[c] = FireCell { kind, fuel: burn_runs | 1, age: 0, intensity: 1, instigator }
    emit ev.fire_started; on_change(c, block channel) (§10.3); return true
```

### 8.2 Ignition sources

All are deterministic except spread (§8.3) and incendiary explosions (§12.3).

| Source | Rule |
|---|---|
| fire damage on a flammable block | a hit with a `dmg.fire` share ≥ 2500 Bp whose impact cell or the cell below holds a block with `flammability_bp > 0` ignites it |
| fire damage on fuel liquid | §6.9 |
| heat | `sys.temperature` ignites blocks with `ignite_decic > 0` in thermal cells at or above it (§9.5) |
| lava | a liquid cell with lava ≥ 100 units ignites flammable blocks in its `N6` neighbors on fire runs |
| burning bodies | on fire runs, a grounded body with `st.burning` ignites its ground block if `flammability_bp >= 2000` (Canon §11.1) |
| explosions | §12.3 |
| doc 05 fire beam | a beam that hits the same flammable block on 10 consecutive pulses calls `ignite` (doc 05 §7.6) |
| doc 06 flame essence | `ignite` on every flammable cell a flame spell touches (doc 06 §13.1) |

New fire: `fuel = burn_runs` (block) or 1 (flash), `intensity = 1`, `age = 0`, `instigator` = the causing entity. Emits `ev.fire_started`.

### 8.3 Spread (`rng.fire`)

Fire runs on `(t + 2) % 12 == 0`. Burning cells in active chunks are processed in ascending `Gidx`, iterating the burning set as it was at the start of the run; cells ignited during the run first spread on the next run. Each draws exactly 6 values `rng.fire.range(10000)`, one per `N6` neighbor in order, whether or not the neighbor can burn.

```
for n, roll in zip(N6, draws):
    base = flammability_bp of n's block (block fire) or of n's fuel liquid (liquid fire, doc 08)
    if n or the cell above n holds an extinguishing liquid ≥ 10 units: base = 0
    dir_bp = 15000 for +Y, 4000 for −Y, 10000 sideways
    p = mul_bp(mul_bp(base, [0, 6000, 10000, 14000][intensity]), dir_bp)
    if roll < p and n is not burning: ignite n with this cell's instigator
then: miasma ≥ 1000 in an N6 neighbor → gas blast (§7.5)
```

Example: oak planks (`flammability_bp` 2500) above a fire of intensity 2 ignite with `mul_bp(mul_bp(2500, 10000), 15000) = 3750` Bp per run.

### 8.4 Burn, burnout and effects

Per fire run after spread, for each cell of the start-of-run burning set, in ascending `Gidx`:

- `age += 1`; `intensity = min(3, 1 + floor_div(age, 10))`.
- Block fire: `fuel −= 1`; at 0 the block becomes `burn_result` (cause `grid.cause.fire`), an `explosive` block detonates (§12), and the fire ends with `ev.fire_out`.
- Liquid fire: consumes 5 units of the cell's `fire_fuel` liquids (proportionally); ends when none remain.
- Flash: ends after one run.
- Burnout timing: a block fire with `fuel = n` ends on its `n`-th burn step. Its first burn step comes on the first fire run whose start-of-run set contains it, 0 to 12 ticks after ignition, and fire runs fall every 12 ticks, so it ends between `12 × (n − 1)` and `12 × n` ticks after ignition. A web (`burn_runs` 5) therefore burns out 48 to 60 ticks after ignition, which realizes Canon §12.1's 60 ticks on the fire grid (Canon gaps G16).
- **Heat:** the cell's thermal cell gains 150 DeciC, capped at 9000.
- **Smoke:** `emit_gas(cell above if gas-open else this cell, gas.smoke, 400)`.
- **Light:** a burning cell emits `light_block` 12 (§10.2).
- **Bodies:** each awake body with `Env.in_fire` gains 40 `st.burning` buildup as environmental buildup (Canon §11.2) with the fire's instigator.

### 8.5 Extinguishing

At the start of each fire run, a burning cell goes out (`ev.fire_out`) when its cell or the cell above holds at least 50 units of a liquid with `extinguishes`; 10 of those units become `emit_gas(c, gas.steam, 300)`. A block fire also ends when its block is replaced by a non-flammable one.

---

## 9. Temperature

### 9.1 Grid

- One thermal cell covers 2 × 2 × 2 blocks: `tx = x >> 1`, `ty = y >> 1`, `tz = z >> 1`. A chunk holds 16 × 16 × 16 = 4,096 thermal cells; the Folio holds 524,288.
- Values are `DeciC`, clamped to `−2730..=15000`. `ambient(cell)` is the Genre Parameter Vector field `ambient_decic` (doc 12 §2.3) of the local genre at the thermal cell's lowest-corner block (default 200, 20.0 °C).
- A chunk's `thermal` is `None` while every cell equals ambient and no source touches it (asleep). Any write wakes it by allocating the array at ambient.

### 9.2 Sources

Block and liquid sources apply first in each temperature run, in ascending `Gidx` of the source cell; the other rows apply when they happen:

| Source | Change to its thermal cell per run |
|---|---|
| block with `heat_emit_decic` (sorted list `heat_sources`, rebuilt on block edits) | `T += floor_div((target − T) × drive_bp, 80000)` |
| liquid cell with `temp_decic` and ≥ 500 units of it | same formula with the liquid's target and drive |
| burning cell | +150, applied by `sys.fire` each fire run (§8.4) |
| explosion | `+heat_decic` at the center cell, applied by `sys.explosions` (§12) |
| doc 06 and doc 13 `add_heat(cell, decic)` | added at the call |

Example: one full lava cell (`temp_decic` 11000, drive 2000) in a thermal cell at 200 adds `floor_div(10800 × 2000, 80000) = 270` per run.

### 9.3 Diffusion and relaxation

Runs on `(t + 5) % 30 == 0` over thermal cells of active chunks (§15.1), Jacobi style from a snapshot `T0` taken at run start. A pair of face-adjacent thermal cells exchanges heat when both lie in active chunks and at least one lies in an awake chunk; a cell in an asleep chunk reads `ambient` and wakes when a nonzero flux reaches it. Pairs that touch an inactive chunk exchange nothing, so an inactive chunk is insulated and keeps its state frozen. Each pair is visited once, from its lower cell along +X, +Y or +Z, in ascending thermal-cell index.

```
cond(cell) = floor_div(Σ heat_cond_bp of its 8 blocks, 8)     // cached per thermal cell, rebuilt on edits
for each qualifying pair (a, b = a + axis), axis in +X, +Y, +Z, ascending a:
    g = min(cond(a), cond(b)); d = T0[a] − T0[b]
    flux = sgn(d) × floor_div(|d| × g, 80000)
    T[a] −= flux; T[b] += flux                                  // conserves heat exactly
then for each awake cell: e = T − ambient; T −= sgn(e) × max(1, floor_div(|e| × 500, 10000)) if e ≠ 0
a chunk whose cells all equal ambient and that has no source falls asleep
```

Six faces at most `g = 10000` move at most 6/8 of the differences, so the scheme is stable. Examples: air cells at 1000 and 200 exchange `floor_div(800 × 10000, 80000) = 100`; air (10000) against stone (2000) exchanges 20; relaxation moves a cell at 1000 with ambient 200 down by 40.

### 9.4 Effects on bodies

Each temperature run, for every awake body in ascending `EntityId`, with `T = Env.temp`:

| Band | Effect |
|---|---|
| `T > 600` | environmental `dmg.fire` hazard, base `floor_div(T − 600, 20) × 100` mHP (Canon §10.6: × `T(d)` against Errata, × `P(d)` against NPCs) |
| `T >= 2500` | also 25 `st.burning` environmental buildup |
| `T <= −100` | `st.chilled` direct application |
| `T <= −300` | environmental `dmg.frost` hazard, base `floor_div(−300 − T, 20) × 100` mHP, and 20 `st.frozen` buildup |

Example: `T = 1000` gives base 2,000 mHP; at Folio 10 against an Erratum `mul_bp(2000, 16190) = 3,238` mHP per run, before resistance.

### 9.5 Effects on blocks and liquids

In each temperature run, for awake thermal cells with `|T − ambient| >= 100`, the 8 blocks are checked in ascending `Gidx`: a flammable block with `0 < ignite_decic <= T` ignites; a block with `melt_decic` and `T >= melt_decic` becomes air (cause `grid.cause.melted`) unless its `timed_revert` entry has `at = Some(_)`, and deposits the entry's stored liquid when it has an entry with `at = None`, else `melt_units` of `melt_to` (§6.9). Liquid freezing and boiling happen in the liquid step (§6.4 step 3).

---

## 10. Light

### 10.1 Channels

Each cell stores `light_block` (low nibble) and `light_sky` (high nibble), each `0..=15` (Canon §1.2). Effective light is `max(light_sky, light_block)`. Stealth, composure and `light_at` use effective light; spawning uses `light_block` only (Canon §12.3). The renderer uploads both channels into doc 02 §8.5's field texture (R = `light_sky × 17`, G = `light_block × 17`), so visual light and simulated light agree by construction.

### 10.2 Sources

- **Sky channel.** A cell is a sky source when `light_filter[state] < 15` and either its `open_tag` bit is set or no cell above it in its column up to y 63 has `light_filter == 15`. Its source level is `ambient_light` of `local_genre(cell)` (Canon §5.2). `sky_src` bits are recomputed for a column whenever a block in it changes opacity.
- **Block channel.** `emission(c) = max(light_emit[state], dynamic emitters at c (§10.4), timed lights at c (§10.4), 12 if c is burning, light_emit of a liquid with ≥ 100 units in c)`.
- **Filter.** Light entering cell `n` loses `1 + filter(n)`, where `filter(n) = light_filter[state(n)] + (1 if n holds ≥ 500 liquid units else 0)`. A cell with `light_filter == 15` receives no light. A liquid cell crossing the 500-unit threshold in a liquids run requests a relight of that cell.

### 10.3 Propagation queues

```rust
pub struct Relight { pub dark: VecDeque<(Gidx, u8 /* channel */, u8 /* old level */)>, pub lit: VecDeque<(Gidx, u8)> }
```

```
on_change(c, ch):                                   // after a block, emitter, fire, liquid or sky change at c
    old = level(c, ch); new_src = source level of c on ch (0 if none)
    if old > new_src: set level(c, ch) = new_src; dark.push((c, ch, old))
    if new_src > 0: set level(c, ch) = max(level, new_src); lit.push((c, ch))
    if filter(c) decreased: lit.push((n, ch)) for each N6 neighbor n with level > 0

pop_dark((c, ch, old)):
    for n in N6 (in bounds):
        ln = level(n, ch)
        if ln > 0 ∧ ln < old: set level(n, ch) = source level of n; dark.push((n, ch, ln))
                               if that source level > 0: lit.push((n, ch))
        else if ln >= old:     lit.push((n, ch))       // n is lit from elsewhere; let it refill
pop_lit((c, ch)):
    L = level(c, ch)
    for n in N6 (in bounds): f = filter(n); if f >= 15: continue
        cand = L − 1 − f; if cand > level(n, ch): set level(n, ch) = cand; lit.push((n, ch))
```

**Budget.** `sys.light` pops at most 4,096 entries per tick: all of `dark` first, then `lit`. Unprocessed entries stay queued in FIFO order and are saved (doc 02 section 5). A `lit` entry reads the cell's current level when popped, so stale entries are harmless. Every level change bumps the chunk's `field_gen`.

**Examples.** A torch (`light_emit` 14) placed on an open floor lights the 1,834 cells with taxicab distance ≤ 13 above the floor plane, one pop each, within one tick. A cell 3 steps away reads 11. Through one water cell (`filter` 1) the next cell reads 14 − 1 − 1 = 12, then 11. In a Mythic Fantasy courtyard (`ambient_light` 4) `light_sky` is 4, and a corridor leading off it reads 3, 2, 1, 0.

### 10.4 Dynamic emitters

- Entities with `light_emitter` (held torches, lanterns, glowing creatures, fire arrows) emit at `min(level, 10)` from the cell of their center point. At most 64 emitters are admitted per Folio: awake entities in ascending `EntityId`, skipping entities whose `lifetime` ends within 30 ticks.
- `sys.light_emitters` compares each admitted entity's current cell with `admitted`; on a change it removes the entity from the old cell's list, adds it to the new one, updates `admitted`, and calls `on_change` for both cells on the block channel. The per-chunk map `BTreeMap<Gidx, Vec<(EntityId, u8)>>` is derived from `admitted` fields and rebuilt on load.
- A level-10 emitter moving one cell along an axis in open space costs 1,150 dark pops and 1,192 lit pops (2,342 in all; recomputed by running §10.3 exactly, `on_change` on the old cell first). A sprinting bearer (6500 mm/s, 108 mm per tick) crosses a cell boundary every 9.23 ticks, so four sprinting torch-bearers average `4 × 2,342 / 9.23 = 1,015` pops per tick. When all four cross on the same tick, their 9,368 pops drain over 3 ticks under the 4,096 budget, and every cell still reaches its correct level.
- **Timed lights** (doc 06 §13.1). `add_light(c, level, expiry_tick)` adds a block-channel source of `min(level, 15)` at cell `c` until `expiry_tick`. A request lasting under 30 ticks becomes a flash instead (below). `timed_lights: BTreeMap<Gidx, Vec<(u8, Tick)>>` holds at most 64 entries in all (saved); a request beyond the cap replaces the entry with the earliest expiry. `sys.light_emitters` removes entries whose `expiry_tick <= t` in ascending `Gidx` and calls `on_change` for each touched cell.
- **Flashes.** Light shorter than 30 ticks (muzzle flash, explosion) never enters the grid. It is appended to the per-tick list `flashes: Vec<(Gidx, u8 level, EntityId)>`, which doc 09 reads in the same tick and which is cleared at `sys.tick_begin`.

### 10.5 Folio bake

At Folio entry every sky source and emitter is seeded and both queues run to empty with no budget, before `folio_entry_tick`. The resulting levels equal what an unbudgeted propagation from scratch produces, because increases alone run in that bake. Doc 12 worldgen calls the same propagation (doc 12 I1). The bake costs at most one pop per lit cell per channel: a fully sky-lit 256 × 64 × 256 box is 4,194,304 pops (≤ 210 ms at 50 ns), and a typical enclosed dungeon Folio lights under 300,000 cells (≤ 15 ms). It runs inside doc 02's Folio load budget while the page turn plays.

### 10.6 Light overrides

Doc 13 §9.2's Redactor `blackout` forces darkness without touching the grid. `light_overrides: Vec<LightOverride>` (saved, at most 8, ascending insertion) holds `LightOverride { lo: IVec3, hi: IVec3, max_level: u8, until: Tick }`. Every read of effective light, `light_sky` or `light_block` for a cell inside an active override's box returns `min(stored, max_level)`; with several overlapping overrides the smallest `max_level` wins. `sys.light_emitters` drops overrides whose `until <= t`. Overrides change no stored level and cost no relight. Spawn light (Canon §12.3) reads the clamped `light_block`. Doc 02's view frame publishes the active overrides so the renderer applies the same clamp to the light field (Interface assumptions).

---

## 11. Sound

### 11.1 Emission

`emit_noise(emitter: Option<EntityId>, noise: Id32, pos: Fx3, level_db: Db, slot: u8)` appends a request to the tick's noise list. Doc 09 owns noise IDs and levels (`noise_level`, doc 09 §7.1). Acting systems emit weapon, spell, item, lock and voice noises directly; the `sys.sound` prelude runs doc 09's `convert_events()` (§7.9), which turns this tick's `ev.footstep`, `ev.entity_landed`, `ev.block_changed`, `ev.entity_killed` and `ev.fire_started` into requests in event `seq` order. `sys.sound` then processes the list in request order and emits one `ev.noise` per request. Doc 05's shorthand `emit_noise(pos, db)` is this call with `emitter` the acting entity, `noise` the doc 05 move's noise ID and `slot` its party slot or `0xFF`.

### 11.2 Costs

Costs are in tenths of a decibel (ddB). `L0 = 10 × level_db`.

| Step into cell `n` | Cost |
|---|---|
| face neighbor (`N18` first six) | 20 |
| edge neighbor (`N18` last twelve), allowed only when both face cells it cuts past have no `full` box | 28 |
| plus the entered cell's `sound_atten[state]` | air 0; the §4.5 example door 150 while closed, 0 while open; a full block of a 3,000 g/l material 400 |
| plus 10 when `n` holds ≥ 500 liquid units | |

The level heard at a cell is `floor_div(L0 − cost, 10)` dB.

### 11.3 Propagation

```
propagate(req, budget: &mut u32):                       // budget starts at 32,768 each tick (§11.4)
    L0  = 10 × req.level_db;  src = cell_of(req.pos)
    candidates = awake bodies with hearing (creatures with perception, every player-controlled entity
                 of slots 0..=4, ghosts included) whose eye point lies within
                 floor_div(L0 × 1000, 19) + 1732 mm of req.pos, ascending EntityId
    if candidates is empty: emit ev.noise with every heard entry None; return
    dist[src] = 0; parent[src] = src; bucket[0].push_back(src)        // buckets 0..=L0, FIFO each
    'outer: for k in 0..=L0:
        while let Some(c) = bucket[k].pop_front():
            if settled(c): continue                                   // a stale, costlier entry
            settle(c); *budget −= 1
            if every candidate's eye cell is settled ∨ *budget == 0: break 'outer
            for (i, d) in N18.enumerate():                            // the §1.2 order
                n = c + d
                if ¬in_bounds(n) ∨ settled(n): continue
                if i ≥ 6 ∧ (full[state(c + d.face1)] ∨ full[state(c + d.face2)]): continue   // no corner cutting
                cost = k + (if i < 6 { 20 } else { 28 }) + sound_atten[state(n)] + (if V(n) ≥ 500 { 10 } else { 0 })
                if cost ≤ L0 ∧ (dist[n] is unset ∨ cost < dist[n]):
                    dist[n] = cost; parent[n] = c; bucket[cost].push_back(n)
    for each candidate e, ascending EntityId:
        if its eye cell x is settled: cost = dist[x]; exact = true
            walk parent pointers from x for up to 4 hops, stopping at src, to cell y
            arrival_dir = try_normalize(center(y) − center(x)), or zero when x = src
        else: the §11.4 ray estimate (exact = false), or not heard when the queue emptied first
        level_db = floor_div(L0 − cost, 10); record HeardNoise when level_db > 0
```

The candidate radius is exact: every step costs at least 19.8 ddB per metre of displacement (an edge step, 28 over 1.414 m), so a cell whose center lies farther than `L0 / 19` m from the source cell's center costs more than `L0`, and 1,732 mm covers the distance from a point to its cell center at both ends. `d.face1` and `d.face2` are the two face steps an edge step combines, for example `+X` and `+Y` for `(+X+Y)`. `dist`, `parent` and the settled set are per-request scratch maps (dense scratch arrays or `BTreeMap`, never hashed) cleared between requests; they are never saved. A cell is settled exactly once per request, so the settled count is the work.

### 11.4 Budget and fallback

All requests of a tick share 32,768 settled cells. When the budget runs out, every unsettled candidate of the current and later requests gets the ray estimate: `cost = floor_div(20 × d_mm, 1000) + Σ sound_atten` of the cells crossed by `raycast_blocks` between the two cells (endpoints excluded), `arrival_dir` = direction to the source, `exact = false`.

### 11.5 Outputs

- `HeardNoise { listener, emitter, noise, level_db, arrival_dir: Fx3, exact: bool }` per candidate with `level_db > 0`, appended to the tick's hearing list for `sys.hearing` (doc 09).
- `ev.noise` carries the source level and, for each player slot `0..=4`, the heard level and arrival direction of that slot's entity, which the audio engine uses for gain, low-pass and apparent position (doc 02 §9). Slot 4 is the invader's. Doc 16's `listener_db[slot]` is `heard[slot]`'s level.

**Examples.** A 130 dB shot (`L0` 1300) reaches a listener 10 m down a straight corridor at cost 200: 110 dB. With a closed oak door on the path the cost is 350: 95 dB. A 45 dB footstep reaches 25 dB at 10 m. Candidates for the shot lie within `68,421 + 1,732 = 70,153` mm.

---

## 12. Explosions

Explosions are a simulation-core subsystem. This chapter owns the algorithm: ordering, the ray method for blocks, exposure, falloff and knockback. Doc 05 §9.3 owns the parameters of item explosions (grenades and launchers), doc 07 §11 those of gadgets, doc 08 §5.6 those of liquid detonations, and §4.5 those of explosive blocks. Doc 05 §9.4 describes the same resolution from the combat side (Canon gaps G19).

### 12.1 Explosion record

```rust
pub struct ExplosionSpec {          // content schema (JSON integers): explosive blocks (§4.5), items, gadgets, detonations
    pub radius_mm: i32,             // 500..=12000
    pub inner_mm: i32,              // full-damage radius, 0..=radius_mm; default floor_div(radius_mm, 4)
    pub edge_bp: Bp,                // falloff factor at radius_mm; default 2500
    pub block_power_milli: i32,     // 0..=8000; 0 = no block damage
    pub env_base_mhp: i64,          // environmental base (Canon §10.6); 0 for item explosions
    pub shares: Vec<(Id32, Bp)>,    // damage type shares, sum 10000
    pub knockback_mm_s: i32, pub incendiary: bool,
    pub ignite_radius_mm: i32,      // 0..=4000; default floor_div(radius_mm, 2) when incendiary
    pub ignite_bp: Bp,              // default 10000
    pub drop_bp: Bp, pub smoke_conc: u16, pub heat_decic: i32, pub noise: Id32,
}
pub enum Source { Item { uid: u64, b_mhp: i64 /* Canon §10.4 magnitude with doc 05 attack factors */ }, Env }
pub struct Explosion { pub center: Fx3, pub spec: ExplosionSpec, pub source: Source,
                       pub source_id: EntityId /* projectile, thrower, gadget or 0 for blocks and liquids */,
                       pub instigator: Option<EntityId>, pub instigator_slot: u8, pub due_tick: Tick, pub seq: u32 }
```

- **Item explosions.** Doc 05 §9.3 archetype rows map onto the spec field for field (`radius_mm`, `inner_mm`, `edge_bp`, `shares`). Block power is `min(8000, mul_bp(4000, block_bp))` unless the item record sets `block_power_milli` itself, as doc 07's demolition charge does (6000). A frag grenade (`block_bp` 10000) gets 4000, a concussion grenade (5000) 2000, an incendiary grenade (2500) 1000, flash and counter grenades (0) no block damage. Doc 04's `sapper` multiplies the block power by 15000 and `demolition` adds 750 (1,000 mm of ray travel at 225 per 300 mm), both before the 8000 cap.
- **Ordering.** `queue_explosion` appends to `explosion_q` (saved) with a per-Folio `seq` counter; the queue holds at most 256 entries, and an append at the cap is refused (the source block still breaks, without a blast). Each tick `sys.explosions` stable-sorts the entries with `due_tick <= t` by `(due_tick, source_id with 0 last, seq)` and resolves the first 4; the rest wait for later ticks. Within a tick this is doc 05 §9.4's ascending source order.
- **Steps per explosion**, in this order: block damage (§12.2), incendiary ignition (§12.3), entity damage and knockback (§12.4), other effects (§12.5), then `ev.explosion`. The entity step therefore sees the blocks the blast just removed, as doc 05 §2.1 requires.

### 12.2 Block damage (ray method, `rng.physics`)

- Directions: for `i, j, k in 0..16` (x outer, z inner) with at least one index equal to 0 or 15, `d = normalize(2i − 15, 2j − 15, 2k − 15)` computed as `div_fx(from_int(a), sqrt_fx(from_int(a² + b² + c²)))` per axis. There are 1,352 directions; entry 0 is raw (−2479700525, −2479700525, −2479700525) and entry 1215 (`i = 15, j = k = 7`) is raw (4276004875, −285066992, −285066992).
- Per ray, in table order, one draw: `I = floor_div(block_power_milli × (7000 + rng.physics.range(6001)), 10000)`. Then, starting at the center and stepping by `s = d × 300 mm` (each axis `mul_fx(d_i, fx_mm(300))`, added to the sample position each step):

```
p = center
while I > 0:
    st = state at cell_of(p)
    if st is not air:
        I −= floor_div((blast_res_milli(st) + 300) × 300, 1000)
        if I > 0 ∧ breakable(cell_of(p)): mark the cell
    I −= 225; p = p + s
breakable(c) = destructible_at(c) ∧ ¬protected(c) ∧ hardness_milli(state(c)) ≠ −1
```

- A non-destructible block absorbs blast intensity like any other block but is never marked, so fixed terrain shields what lies behind it (Canon §4.3).
- Marked cells are processed in ascending `Gidx`: each draws one `rng.physics.range(10000)` and drops its block items (doc 07 `block_drops`) when the draw is below `drop_bp`, except cells with doc 07 `placed_meta`, which never drop (doc 07 §10.4); the block becomes air with cause `grid.cause.explosion`. An `explosive` block queues its own explosion with `due_tick = t + 10`.
- Draw count per explosion: 1,352 + the number of marked cells. Explosions with `block_power_milli = 0` skip this subsection and draw nothing from `rng.physics`.
- A ray samples at most `ceil_div(8000 × 13000 / 10000, 225) = 47` positions at the power cap, so the march is at most 63,544 samples per explosion.

**Example.** Power 4000, draw 3000 (`I = 4000`), ray 1215 from (10.5, 2.5, 10.5) toward a destructible `block.cracked_wall` (`blast_res_milli` 6000) at x = 13. Samples 0–8 are air and leave `I = 4000 − 9 × 225 = 1975`; sample 9 lies in (13, 2, 10): `1975 − 1890 = 85 > 0`, so the cell is marked; `85 − 225 < 0` ends the ray. Had the wall been `block.masonry.granite` (not destructible), the same ray would stop there and mark nothing.

### 12.3 Incendiary

For an incendiary explosion, every cell whose block is flammable, whose center lies within `ignite_radius_mm` of `center`, and for which `raycast_blocks` from `center` toward the cell center meets that cell as its first solid box, is visited in ascending `Gidx`. Each draws one `rng.fire.range(10000)` and calls `ignite(cell, Block, instigator)` (§8.1) when the draw is below `ignite_bp`. Draw count: the number of cells visited. Within the 4,000 mm cap that is at most 286, the most cell centers a 4 m ball can hold. Doc 05's incendiary grenade sets `ignite_radius_mm` 2000 and `ignite_bp` 10000, so it ignites every exposed flammable block within 2 m; the §4.5 powder keg uses 2000 and 3333.

### 12.4 Damage and knockback to entities

For every live, non-ghost entity with `body` and `pose`, every item drop, and every container and gadget with `pools`, in ascending `EntityId`:

```
tp      = the point closest to center on the axis segment of the target's largest-share part capsule,
          posed at t (doc 05 §9.4); a target without pose uses p + (0, floor_div(h, 2), 0)
dist_mm = to_mm(dist(center, tp));  if dist_mm > radius_mm: skip
falloff = 10000                                                                  if dist_mm ≤ inner_mm
          10000 − floor_div((10000 − edge_bp) × (dist_mm − inner_mm), radius_mm − inner_mm)   otherwise
perp    = try_normalize((−(tp.z − center.z), 0, tp.x − center.x)), or (1, 0, 0) when that returns None
samples = [tp, p + (0, 100 mm, 0), p + (0, h − 100 mm, 0), tp + perp × r, tp − perp × r]    // r = body radius
exposure = 2000 × (number of samples s for which raycast_blocks(center → s, t_max = |s − center|) meets no box)
if exposure == 0: skip
f       = mul_bp(falloff, exposure)
B_eff   = mul_bp(B, f)                     // B = b_mhp (item) or env_base_mhp (environment)
if the target has pools: queue a hit for sys.damage: part = largest-share part, flags area (+ env for Env),
          shares = spec.shares, instigator and instigator_slot; doc 05 scales impact and status powers by f
apply_knockback(e, unit(q − center + (0, 500 mm, 0)), mul_bp(knockback_mm_s, f))       // q = p + (0, h/2, 0)
```

- Item drops receive knockback only. The thrower and allies are included; friendly fire applies `FF_AREA_BP` at Canon §10.5 step 2.15 (Canon §17.5).
- An environmental explosion's hit enters the pipeline at step 3 with `D2 = mul_bp(B_eff, T(d))` against Errata and `mul_bp(B_eff, P(d))` against NPCs (Canon §10.6). An item explosion's hit enters at step 1 with `B = B_eff`.

Example: `R = 4000`, `inner_mm = 1000`, `edge_bp = 2500`, `B = 40,000` mHP, target point at 2500 mm with 4 of 5 samples exposed: `falloff = 10000 − floor_div(7500 × 1500, 3000) = 6250`, `f = mul_bp(6250, 8000) = 5000`, `B_eff = 20,000` mHP; a 10000 mm/s knockback becomes 5000 mm/s before mass scaling.

### 12.5 Other effects

`emit_gas(cell(center), gas.smoke, smoke_conc)`; `add_heat(cell(center), heat_decic)`; a flash of level 15 for doc 09 (§10.4); `emit_noise` with `spec.noise` at doc 09's `noise_level`; for each damage type in `shares` that has a §6.9 trigger row, one `surface_trigger` on the center cell and its `N6` neighbors; then `ev.explosion`.

---

## 13. Block edits

### 13.1 `set_block`

`set_block(c, new, cause: Id32 /* grid.cause.* */, actor: Option<EntityId>) -> Result<BlockState, EditError>` is the only way to change a block after worldgen. It checks, in this order, and returns the first failure:

1. `out_of_bounds`: `c` is outside the Folio box.
2. `bad_props`: `new` has prop bits its shape does not use (§4.2).
3. `protected`: the cell's `protect` bit is set and the cause is neither `grid.cause.worldgen` nor `grid.cause.script` (§4.7).
4. `fixed_terrain`: the current block is not destructible, the type index changes, and the cause is neither `grid.cause.worldgen` nor `grid.cause.script` (§4.7, Canon §4.3).
5. `actor_forbidden`: the actor is the invader (slot 4) or a ghost and the cause is `placed`, `mined` or `broken` (Canon §17.6, §17.9). Ghosts may toggle doors.

On success, in this order: write the chunk and bump `block_gen`; delete liquid above the new `liquid_cap` and bump `fluid_gen`; end a block fire if the new block cannot burn; remove the cell's `block_damage` entry and, unless the cause is `froze` or `reverted`, its `timed_revert` entry; call doc 07's `on_block_changed(c, old, new, cause, actor)`, which clears `placed_meta`, `stations` and `crops` entries when the cause is not doc 07's own; update `heat_sources`, the thermal conductance cache and the column's `sky_src` bits; call light `on_change` for both channels; push `c` and its `N6` neighbors onto the block-update queue; emit `ev.block_changed`.

### 13.2 Breaking

- **Mining** (tool use, doc 07 §9): `sys.actions` sets `Mining { cell, state, needed = mining_ticks(...) }` (doc 07 §9.2) when a mining action targets a new cell, and adds 1 to `progress` on every tick the action continues. The block breaks when `progress >= needed`, with cause `grid.cause.mined` and doc 07 drops. Progress resets when the target cell or its state changes, or after 6 ticks without the action. `mining_ticks` returns `None` when the tool tier is too low or the block is not destructible.
- **Damage** (weapons, projectiles, beams, spells, AI breaches): `damage_block(c, mhp, shares, actor, block_bp) -> bool` adds `amount = mul_bp(mhp, block_bp)`, where `mhp` is the hit's Canon §10.4 magnitude with its attack factors and `block_bp` is doc 05 §7.3's block factor (melee swings 5000 once per swing, bullets and pellets 2500, thrown weapons 2500, arrows and bolts 1000, beam pulses 1000, launcher direct hits 10000). The amount is 0 when the cell is not breakable (§12.2) or the source's tool tier (doc 07 §9.1) is below the block's `tool_tier`.
  - A cell with doc 07 `placed_meta` loses `amount` from `placed_meta.hp` and breaks at 0 (cause `grid.cause.broken`, no drop; doc 07 §10.4).
  - Any other cell accumulates in `block_damage: BTreeMap<Gidx, (i64 accumulated, Tick last)>` and breaks (cause `grid.cause.broken`, doc 07 drops) when the sum reaches `block_hp(c)`, which is the record's `block_hp_mhp` (§4.5, doc 07 `block_hp`). `sys.block_updates` removes entries untouched for 600 ticks.
  - The crack stage that doc 16 §6.11 replicates is `min(9, floor_div(accumulated × 10, block_hp(c)))`, or the same formula over `hp_max − hp` for placed cells.
  - Doc 11's `perk.leg.worldbreaker` multiplies `amount` by 30000 and breaks a destructible block of hardness 3 or less (`floor_div(hardness_milli, 1000) <= 3`) on any hit; doc 05 applies it before the call.
- Example: a `block.cracked_wall` (`hardness_milli` 2000) has `block_hp` 40,000. A 12,000 mHP melee swing adds 6,000, so the 7th swing breaks it; after 5 swings the crack stage is `floor_div(30,000 × 10, 40,000) = 7`. An 8,000 mHP bullet adds 2,000, so 20 bullets break it. A 40,000 mHP launcher direct hit breaks it at once.

### 13.3 Placing

Errata place only `cat.placeable` items and `item.tool.torch` (Canon §4.3, doc 07 §10.3). This section is the geometric validity check `can_place(new_state, placer)`; doc 07 §10.3 adds anchors, per-player and per-Folio caps, the Fieldcraft channel and `placed_meta`, and repeats this check on the channel's last tick. A placement succeeds only when all hold:

1. The target comes from `raycast_blocks` from the eye along the view, `t_max` 4500 mm. The target cell is the hit cell when its block is `replaceable`, else the hit cell plus the entered face normal.
2. The target is in bounds, not protected, and holds air, a destructible `replaceable` block, or only liquid.
3. Attach blocks need a sturdy face on the support side; other blocks need any `N6` neighbor that is not air, except `gravity` blocks.
4. No collision box of the new state penetrates any non-ghost body capsule or item drop (§5.4 test with zero tolerance).
5. The placer is not `st.mounted` (Canon §17.7), not downed, not a ghost and not the invader (Canon §17.9), and holds the item (doc 11 consumes it on success).

Props from the placer: horizontal facing from `facing_from_yaw` (stairs face away from the placer; doors, ladders and fixtures face the placer); slab and stair half from the hit point (upper half of a side face or a bottom face gives top); doors place two cells (lower, upper) and both must pass; door hinge is left. Cause `grid.cause.placed`.

### 13.4 Doors and trapdoors

INTERACT on a door or trapdoor toggles bit b2 on both halves through `set_block` with cause `grid.cause.toggled`. The toggle is refused when doc 09's `door_locked(gidx)` is true for the lower cell (doc 09 §13.1), when the door is `block.barricade.gate` and the actor is not `fac.errata` (doc 07 §10.4), or when the opened boxes would penetrate a body. A toggle changes props only, so worldgen doors that are not destructible still open and close. Ghosts may toggle doors (Canon §17.6).

### 13.5 Block-update queue

`block_updates: VecDeque<Gidx>`, saved, at most 2,048 pops per tick. For each popped cell in order:

1. **Attachment.** An `attach` block whose support face lost its sturdy neighbor breaks with cause `grid.cause.unsupported` and drops.
2. **Gravity.** A `gravity` block whose cell below has `liquid_cap > 0` or is air becomes a falling block (§13.6).
3. **Connections.** Fence and pane props are recomputed.
4. **Wake.** Liquid cells in the cell and its `N6` neighbors join `liquid_active`; gas cells join the gas active set; the chunk's thermal grid wakes if the conductance changed.
5. **Timed reverts and damage decay** due by `t` are applied (§6.9, §13.2); a revert uses cause `grid.cause.reverted`.

### 13.6 Falling blocks

The block becomes air (cause `grid.cause.fell`) and a `grid.ekind.falling_block` entity spawns at the cell's bottom center with the block's state and the edit's actor as instigator. It moves under §5.3 gravity with voxel collision only. While it moves down at 2000 mm/s or faster, each body it overlaps for the first time takes an environmental `dmg.blunt` hit of base `min(30,000, 3,000 × fall_m)` mHP, `fall_m` being whole metres fallen, flag `area`. On landing it places its state in the cell containing its center (cause `grid.cause.landed`); if that cell is not air or replaceable, or a body capsule occupies it, it drops as an item instead.

### 13.7 Random block ticks (`rng.blocktick`)

For each active chunk in ascending slot order, 8 picks, each drawing exactly 2 values: `i = range(32768)`, `roll = range(10000)`. The block at local index `i` runs its `random_tick`:

| Kind | Fields | Effect |
|---|---|---|
| `grid.rt.none` | — | nothing |
| `grid.rt.grow` | `chance_bp`, `max_age` | `roll < chance_bp` and age < `max_age`: age + 1 (cause `grid.cause.grew`) |
| `grid.rt.decay` | `chance_bp`, `to` | `roll < chance_bp`: becomes `to` (cause `grid.cause.decayed`) |
| `grid.rt.spread` | `chance_bp`, `from`, `to` | `roll < chance_bp` and the `N4A[roll & 3]` neighbor is `from`: that neighbor becomes `to` (cause `grid.cause.spread`) |

16 draws per active chunk per tick, whatever the outcomes.

### 13.8 Erasure of Errata placeables

`is_errata_placed(c) = placed_meta.contains_key(c)` (doc 07 §1.4) is the `placed_by_errata` bit doc 13 §10.6 reads; no second store exists. `erase_placed(c, eraser) -> bool` calls `set_block(c, block.air, grid.cause.erased, Some(eraser))` when `is_errata_placed(c)`; the block drops nothing and doc 07 removes its `placed_meta`. Callers:

- `sys.erasure` (doc 13 §10.6, Canon §11.6): each `fac.editors` member, hunters and the invader included, in ascending `EntityId`, erases at most one placed cell per 10 ticks, the one whose center is nearest its origin within 1500 mm, ties to the lower `Gidx`.
- Doc 08 §3.4: a cell holding at least 1 unit of `liq.red_ink` erases one `N6`-adjacent placed cell per 120 ticks.

Distance is `dist2` from the eraser's origin to the cell center, compared with `(1500 mm)²` inclusive. Example: a hunter with origin `(20.5, 2.0, 20.5)` beside placed barricades at cells (21, 2, 20) and (19, 2, 20) finds both centers `sqrt(1.0² + 0.5²) = 1.118` m away, so it erases the lower `Gidx` first: (19, 2, 20) is `19 + 256 × (20 + 256 × 2) = 136,211` against 136,213. Ten ticks later it erases (21, 2, 20).

---

## 14. Event bus

### 14.1 Structure and ordering

```rust
pub struct SimEvent { pub id: Id32, pub seq: u32, pub payload: EventPayload }   // doc 02 §5.5
pub struct EventBus { pub list: Vec<SimEvent>, next_seq: u32 }
impl EventBus {
    pub fn emit(&mut self, id: Id32, payload: EventPayload) {
        assert!(self.next_seq < 65_536);              // deterministic panic on a runaway producer
        self.list.push(SimEvent { id, seq: self.next_seq, payload }); self.next_seq += 1;
    }
}
```

- `seq` restarts at 0 in `sys.tick_begin`. Emission order is system order (§2.2), then the producing system's iteration order (ascending `EntityId`, `Gidx` or queue order).
- A consumer system reads `list` from the index where its previous read ended in the same tick. `sys.tick_end` moves `list` into `StepOutput` (doc 02); it is never saved (§2.4 rule 4).
- Payload references use `EntityId`, `u64` uids, `Gidx`, `Fx3` and `Id32`; `0` stands for "no ID". `slot` is a party slot, `0xFF` for none.

### 14.2 Payload schemas

Common flag word `DmgFlags: u32`: bit 0 crit, 1 sneak, 2 dot, 3 no_chain, 4 blocked, 5 area, 6 precision, 7 env, 8 fall, 9 reflected, 10 chain, 11 friendly_fire, 12 lethal.

| Event | Payload fields | Producer | Main consumers |
|---|---|---|---|
| `ev.folio_entered` | `folio_index: u32, genre: Id32, entry_tick: Tick, party: [Option<EntityId>; 4]` | 03 (`sys.tick_begin`) | 04, 05, 11, 13, 14, 16, 17 |
| `ev.folio_exited` | `folio_index: u32, exit_index: u8, next_genre: Id32` | 12, 14 | 13, 14, 16 |
| `ev.entity_spawned` | `entity: EntityId, kind: Id32, template: Id32, pos: Fx3, owner: Option<EntityId>` | 03 | 09, 13, 16 |
| `ev.entity_despawned` | `entity: EntityId, kind: Id32, reason: Id32 /* grid.despawn.* */` | 03 | 13, 16 |
| `ev.damage_dealt` | `attacker: Option<EntityId>, source: EntityId, target: EntityId, part: Id32, item_uid: u64, ability: Id32, per_type: Vec<(Id32, i64)>, total_mhp: i64, ward_mhp: i64, hp_mhp: i64, poise_mhp: i64, flags: DmgFlags, pos: Fx3, instigator_slot: u8` | 05 | 04, 09, 13, 14, 17 |
| `ev.healed` | `target: EntityId, source: Option<EntityId>, amount_mhp: i64, overheal_mhp: i64, cause: Id32` | 04, 06, 08 | 14, 17 |
| `ev.entity_killed` | `victim: EntityId, killer: Option<EntityId>, instigator_slot: u8, template: Id32, rank: Id32, pos: Fx3, damage_seq: u32` | 05 | 04, 11, 13, 14 |
| `ev.status_applied` | `target: EntityId, status: Id32, source: Option<EntityId>, stacks: u8, expiry: Option<Tick>, via: u8 /* 0 buildup, 1 direct */` | 05 | 04, 13, 17 |
| `ev.status_expired` | `target: EntityId, status: Id32, reason: u8 /* 0 timeout, 1 cleansed, 2 consumed, 3 death, 4 folio exit */` | 05 | 13, 17 |
| `ev.noise` | `emitter: Option<EntityId>, noise: Id32, pos: Fx3, level_db: Db, heard: [Option<(Db, Fx3)>; 5]` (per player slot 0..=4, slot 4 the invader: level and arrival direction) | 03 (`sys.sound`) | 09, 16, 17, audio |
| `ev.awareness_changed` | `observer: EntityId, subject: EntityId, from: Id32, to: Id32 /* aware.* */` | 09 | 13, 17 |
| `ev.item_picked_up` | `entity: EntityId, uid: u64, from: u8 /* 0 ground, 1 container, 2 shop */, from_entity: Option<EntityId>, stack: i32, cause: u8 /* 0 normal, 1 trade */` | 11, 16 | 10, 14 |
| `ev.item_dropped` | `entity: EntityId, uid: u64, drop_entity: EntityId, pos: Fx3, stack: i32, cause: u8 /* 0 normal, 1 trade */` | 11, 16 | 13 |
| `ev.item_used` | `user: EntityId, uid: u64, template: Id32, target: Option<EntityId>, cell: Option<Gidx>` | 07, 08, 11 | 04, 10 |
| `ev.item_identified` | `by: EntityId /* 0 = no entity */, uid: u64 /* 0 = whole type */, template: Id32, state: Id32 /* know.* */, descriptor: Id32 /* 0 when not type-hidden */, flags_new: u32 /* 0 for a type event */, reason: Id32 /* appr.reason.* */` | 10 | 14, 15, 17 |
| `ev.item_crafted` | `crafter: EntityId, uid: u64, recipe: Id32, station: Id32, quality: Id32` | 07 | 04, 14 |
| `ev.potion_brewed` | `brewer: EntityId, uid: u64, recipe: Id32, apparatus: Id32` | 08 | 04, 14 |
| `ev.spell_cast` | `caster: EntityId, spell: Id32, target: Option<EntityId>, target_pos: Fx3, aether_mhp: i64, fizzled: bool` | 06 | 04, 09, 13 |
| `ev.block_changed` | `cell: Gidx, old: BlockState, new: BlockState, cause: Id32 /* grid.cause.* */, actor: Option<EntityId>` | 03 (`set_block`) | 07, 09, 12, 13, 14, 16 |
| `ev.liquid_reaction` | `cell: Gidx, rule: Id32 /* alch.* or the triggering dmg.* */, a: Id32, b: Id32, result: Id32, units: u16, instigator_slot: u8` | 03 | 08, 14 |
| `ev.skill_gained` | `entity: EntityId, skill: Id32, value: i32` | 04 | 14, 17 |
| `ev.level_gained` | `entity: EntityId, level: i32` | 04 | 14, 17 |
| `ev.contradiction_changed` | `holder: EntityId, uid: u64, component: u8, old_c: u8, new_c: u8` | 05 (hook in `sys.env_sample`) | 12, 17 |
| `ev.container_opened` | `opener: EntityId, container: EntityId, kind: Id32` | 11 | 13, 14 |
| `ev.engine_spun` | `user: EntityId, engine: EntityId, cost: i64, result_uid: u64` | 11 | 14 |
| `ev.reputation_changed` | `faction: Id32, old: i32, new: i32, cause: Id32` | 13 | 17 |
| `ev.erasure_stack` | `entity: EntityId, stacks: u8` | 13 | 09, 17 |
| `ev.hunter_spawned` | `hunter: EntityId, reason: u8 /* 0 erasure, 1 colophon mark */` | 13 | 14, 17 |
| `ev.player_downed` | `entity: EntityId, slot: u8, by: Option<EntityId>` | 05 | 13, 17 |
| `ev.player_revived` | `entity: EntityId, slot: u8, reviver: EntityId` | 16 | 17 |
| `ev.run_ended` | `outcome: u8 /* 0 victory, 1 party dead, 2 abandoned */, folio_index: u32, tick: Tick` | 14 | 14, 15, 16 |
| `ev.player_died` | `entity: EntityId, slot: u8, killer: Option<EntityId>` | 05 | 14, 16, 17 |
| `ev.player_ghosted` | `entity: EntityId, slot: u8, remains_uid: u64` | 16 | 17 |
| `ev.player_resurrected` | `entity: EntityId, slot: u8, hp_bp: Bp` | 16 | 17 |
| `ev.ward_broken` | `entity: EntityId, last_type: Id32` | 05 | 06, 17 |
| `ev.part_severed` | `entity: EntityId, part: Id32, item_uid: u64, by: Option<EntityId>` | 05 | 04, 11, 17 |
| `ev.part_reattached` | `entity: EntityId, part: Id32, method: Id32` | 04 | 17 |
| `ev.minion_bound` | `minion: EntityId, owner_slot: u8` | 13 | 17 |
| `ev.minion_released` | `minion: EntityId, owner_slot: u8, reason: u8 /* 0 dismissed, 1 died, 2 folio exit */` | 13 | 17 |
| `ev.mounted` | `rider: EntityId, mount: EntityId, seat: u8` | 05 | 09, 17 |
| `ev.dismounted` | `rider: EntityId, mount: EntityId, thrown: bool` | 05 | 17 |
| `ev.affliction_changed` | `entity: EntityId, affliction: Id32, stage: u8 /* 0 incubating, 1 took hold, 2 cured */` | 04 | 17 |
| `ev.invasion_started` | `invader: EntityId, slot: u8 /* 4 */, kit: Id32, folio_index: u32` | 13, 16 | 09, 14, 15, 17 |
| `ev.invasion_ended` | `invader: EntityId, reason: u8 /* 0 left, 1 died, 2 timed out, 3 Redacted, 4 Colophon entered, 5 blocked, 6 host migration (doc 16 §9.13) */, redacted: bool, duration: Ticks` | 16 | 13, 14, 15, 17 |
| `ev.explosion` (03) | `center: Fx3, radius_mm: i32, instigator: Option<EntityId>, cells_broken: u16` | 03 | 09, presentation |
| `ev.fire_started` (03) | `cell: Gidx, kind: u8 /* 0 block, 1 liquid, 2 flash */, instigator: Option<EntityId>` | 03 | 09, presentation |
| `ev.fire_out` (03) | `cell: Gidx, reason: u8 /* 0 burnout, 1 extinguished, 2 replaced */` | 03 | presentation |
| `ev.footstep` (03) | `entity: EntityId, gait: Id32 /* grid.gait.* */, material: Id32` | 03 | 09, audio |
| `ev.entity_landed` (03) | `entity: EntityId, fall_mm: i32, material: Id32` | 03 | 04, 09, audio |

JSON log form (integers and hex strings only; `Fx3` as raw values). Cell 133645 is block (13, 2, 10), the stone the §12.2 ray breaks:

```json
{ "tick": 1000, "seq": 7, "id": "ev.block_changed",
  "payload": { "cell": 133645, "old": 37, "new": 0, "cause": "grid.cause.explosion", "actor": "0x0000030000000005" } }
```

```json
{ "tick": 1000, "seq": 3, "id": "ev.damage_dealt",
  "payload": { "attacker": "0x0000030000000005", "source": "0x0000030000000041", "target": "0x0000030000000012",
               "part": "bp.torso", "item_uid": "0x0000030000000009", "ability": 0,
               "per_type": [["dmg.blunt", 10000], ["dmg.fire", 10000]], "total_mhp": 20000, "ward_mhp": 0,
               "hp_mhp": 20000, "poise_mhp": 4000, "flags": 32, "pos": [45097156608, 8589934592, 45097156608],
               "instigator_slot": 0 } }
```

### 14.3 Bus events registered for other chapters

Canon §19 gives the `ev.` namespace to the canon core list and to this chapter for the bus. Events that other chapters emit beyond the canon list are registered here with their payloads; the requesting chapter is the producer and may append fields only through this table. `ev.entity_pacified` is the one name for the pacification event that doc 04 calls `ev.entity_pacified` and doc 13 calls `ev.pacified` (Canon gaps G21).

| Event | Payload fields | Producer | Main consumers |
|---|---|---|---|
| `ev.wound_changed` | `entity: EntityId, part: Id32, tier: u8 /* 0..=4 */, kind: u8 /* 0 laceration, 1 fracture, 2 burn, 3 trauma */, treated: bool` | 04 | 13, 17 |
| `ev.augment_changed` | `entity: EntityId, aug: Id32, change: u8 /* 0 installed, 1 removed, 2 rank up, 3 enabled, 4 disabled by Contradiction */, rank: u8` | 04 | 17 |
| `ev.entity_pacified` | `target: EntityId, by: EntityId, slot: u8, method: u8 /* 0 spared, 1 pacify check, 2 tamed, 3 hacked */` | 13 | 04, 14, 17 |
| `ev.secret_found` | `finder: EntityId, slot: u8, cell: Gidx, kind: u8 /* doc 12 §16 hiding method index */` | 12 | 04, 14, 17 |
| `ev.objective_completed` | `kind: u8 /* 0 rubric, 1 challenge, 2 story */, objective: Id32, by_slot: u8` | 12 | 14, 17 |
| `ev.cast_started` | `caster: EntityId, spell: Id32, cast_ticks: Ticks, pos: Fx3` | 06 | 09, 13, 17 |
| `ev.cast_interrupted` | `caster: EntityId, spell: Id32, cause: u8 /* 0 damage, 1 control status, 2 silenced, 3 cancelled */` | 06 | 13, 17 |
| `ev.spell_fizzled` | `caster: EntityId, spell: Id32, aether_mhp: i64` | 06 | 04, 17 |
| `ev.wild_surge` | `caster: EntityId, spell: Id32, entry: u8 /* doc 06 §11.3 table row */, param: i32` | 06 | 17 |
| `ev.glyph_learned` | `entity: EntityId, glyph: Id32, origin: Id32` | 06 | 14, 17 |
| `ev.enchant_applied` | `by: EntityId, uid: u64, ench: Id32` | 06 | 10, 17 |
| `ev.prayer_resolved` | `entity: EntityId, patron: Id32, outcome: u8 /* doc 06 prayer table row */, favor: i32` | 06 | 14, 17 |
| `ev.favor_changed` | `entity: EntityId, patron: Id32, old: i32, new: i32, cause: Id32` | 06 | 17 |
| `ev.alarm_raised` | `raiser: EntityId /* 0 for a device without an entity */, pos: Fx3, kind: u8 /* 0 callout, 1 alarm device, 2 camera */` | 09 | 13, 17 |
| `ev.body_discovered` | `observer: EntityId, body: EntityId, pos: Fx3` | 09 | 13 |
| `ev.disguise_blown` | `wearer: EntityId, observer: EntityId` | 09 | 13, 17 |
| `ev.lock_opened` | `by: EntityId, lock_key: u64 /* door Gidx or container EntityId, doc 09 §1.4 */, method: u8 /* 0 key, 1 picked, 2 hacked, 3 forced */` | 09 | 04, 13 |
| `ev.object_revealed` | `observer: EntityId, object_key: u64, kind: u8 /* 0 trap, 1 secret */` | 09 | 12, 17 |
| `ev.device_hacked` | `by: EntityId, device: EntityId` | 09 | 04, 13 |
| `ev.takedown` | `attacker: EntityId, target: EntityId, lethal: bool` | 09 | 04, 13 |
| `ev.item_stolen` | `thief: EntityId, victim: EntityId, uid: u64` | 09 | 13, 15 |
| `ev.rubric_completed` | `folio_index: u32, rubric: Id32` | 14 | 12, 17 |
| `ev.colophon_entered` | `entity: EntityId, slot: u8, first: bool /* first Erratum this Folio */` | 14 | 13, 17 |
| `ev.colophon_desecrated` | `by: EntityId, cell: Gidx` | 14 | 13, 17 |
| `ev.boon_committed` | `entity: EntityId, slot: u8, boon: Id32` | 14 | 17 |
| `ev.note_completed` | `slot: u8, note: Id32` | 14 | 17 |
| `ev.exit_committed` | `folio_index: u32, exit_index: u8, genre: Id32` | 14 | 12, 16, 17 |
| `ev.boss_engaged` | `boss: EntityId, template: Id32` | 14 | 13, 16, 17 |
| `ev.volume_completed` | `volume: u8, folio_index: u32` | 14 | 15, 17 |

---

## 15. Activation and interest

### 15.1 Active chunks

`sys.interest` rebuilds `active: [bool; 128]` each tick. Every player-controlled entity (each Erratum alive, downed or ghost, and the invader in slot 4) activates every chunk with `|cx − ecx| <= 2` and `|cz − ecz| <= 2`, for both chunk layers. Every entity with `interest` (boss arenas while engaged, scripted events, Colophon anchors) activates chunks within its own `radius_chunks`. One Erratum activates up to 50 of the Folio's 128 chunks.

Inactive chunks keep their liquid, gas, fire and thermal state frozen: their cells stay in the active sets but are skipped until the chunk activates. Light, block updates and sound run everywhere.

### 15.2 Sleeping entities

A creature, mount, gadget, item drop or corpse is asleep (`Kind.flags & ASLEEP`) when all hold: its chunk is inactive, it is grounded or has no `move`, it is not `in_combat` (Canon §2.3), and doc 09 reports no awareness above unaware toward any Erratum. Asleep entities skip perception, AI think, movement, environment sampling, animation advance, surfaces and temperature effects, and they hear nothing (§11.3). `sys.pose` still appends their unchanged history record each tick, so a lag-compensated test never finds a gap (§5.16). They still take hits, and their status timers and DoT pulses run. They wake the tick any condition fails, and a hit wakes them on the next tick. Projectiles, zones and player-controlled entities never sleep. Doc 09's `awareness_above_unaware(e)` (§6.8) is the awareness query.

### 15.3 Actor cap

Creatures and mounts count toward `ACTIVE_ACTOR_CAP` (256; Canon §4), minions included (Canon §15.1). Doc 13 calls `request_spawn(template, pos, yaw, owner)`; when the cap is reached the request waits in `spawn_q: VecDeque<SpawnReq>` (saved) and `sys.spawn_admit` admits it, oldest first, once the count drops.

---

## 16. Save, restore and state hash

### 16.1 Coverage

Doc 02 §6.5 fixes the section order. This chapter's state maps onto it as follows; everything listed is hashed.

| Section | This chapter's contents, in this order |
|---|---|
| 0 `meta` | boundary tick, `folio_entry_tick`, entity counter, explosion `seq` counter |
| 1 `rng` | `rng.fire`, `rng.blocktick`, `rng.physics` states |
| 2 `entities` | every column of §3.3 in catalog order, including `Pose.hist` (32 records), `MoveState` (with `wall_n`, `wall_jumped`, `wall_run_mm`), `Env` accumulators, `Mining`, `LightEmitter.admitted`, `Projectile.catchup` and `resting`; then `spawn_q` |
| 4 `blocks` | 128 chunks in slot order: `palette_rle` of `BlockState`, then `sky_src`, `open_tag`, `protect`, `flip` bitsets; then `block_updates` FIFO, `block_damage`, `timed_revert` (with `Revert.at`), `explosion_q`, the Folio's `PortalPair` list; then doc 07's `placed_meta`, `stations` and `crops` maps in doc 07's declared order |
| 5 `light` | per chunk `palette_rle` of the light bytes; then `Relight.dark` and `Relight.lit` in FIFO order; then `timed_lights` ascending and `light_overrides` in insertion order |
| 6 `fluids` | per chunk ascending `(Local, LiquidCell)` and `(Local, GasCell)` lists; `liquid_active`, `gas_active` ascending; liquid and gas run counters |
| 7 `thermal` | per chunk: 0 for asleep, or 1 then `palette_rle` of the 4,096 `DeciC` values as `u16` offsets `T + 2730`; then `fires` ascending |

### 16.2 Excluded derived state

Never serialized, rebuilt in this order on restore: per-state block tables (§4.5); in-memory palettes from `palette_rle`; `heat_sources` and thermal conductance caches; the dynamic-emitter map (from `admitted`); the broadphase. Also never saved: `block_gen`, `field_gen`, `fluid_gen`, meshes, navigation graphs (doc 13 rebuilds them from blocks), the per-tick lists (noise requests, `flashes`, `push_contacts`, hearing records, telegraphs), the sound scratch maps, the event list, and doc 02's view and doc 16's replication caches. The broadphase needs no restore step: `sys.tick_begin` rebuilds it before any system reads it (§3.6). Rebuilding is pure, so `restore` followed by `state_hash()` reproduces the saved hash (doc 02 SyncTest with depth 8).

### 16.3 Rules

- Saves happen only at tick boundaries, so no system is mid-run and every per-tick list is empty.
- An asleep thermal chunk serializes as one byte; an awake chunk whose cells all returned to ambient serializes in full until the run that puts it to sleep.
- The light grid is saved, not recomputed, because budgeted relighting makes it path-dependent.
- Every derived cache listed in §16.2 is a pure function of saved state, and every system that reads one rebuilds or updates it within the tick it reads it (§3.6), so `restore(b)` followed by stepping reproduces the uninterrupted run bit for bit (acceptance test 32).

---

## 17. Performance budgets

Reference: one Steam Deck core (Zen 2, 3.5 GHz boost), `dist` profile, 4 Errata, 256 actors, 1,024 projectiles, 2,048 item drops. Doc 02 registers each row as a `perf.deck.sim.*` gate; criterion benchmarks enforce the per-unit costs.

| Subsystem | Unit cost | Typical per due tick | Worst per due tick |
|---|---|---|---|
| Movement and collision (§5) | ≤ 3 µs per moving body (12 cells, 4 passes, 7 substeps max); ≤ 0.2 µs per resting body (§5.22) | 0.45 ms | 1.2 ms (260 moving bodies and 2,048 resting items: 0.78 + 0.41 ms) |
| Push and broadphase (§3.6, §5.14) | ≤ 0.1 µs per pair; ≤ 30 ns per broadphase entry, three rebuilds per tick | 0.08 ms | 0.3 ms |
| Environment sampling (§5.20) | ≤ 0.6 µs per awake body | 0.1 ms | 0.2 ms |
| Pose and history | ≤ 0.1 µs per entity | 0.03 ms | 0.05 ms |
| Raycast (§5.18) | ≤ 15 ns per visited cell | — | — |
| Projectiles (§5.19) | ≤ 0.4 µs each | 0.15 ms | 0.5 ms |
| Explosions (§12) | ≤ 0.35 ms each (1,352 rays × ≤ 47 samples at the 8,000 power cap, ≤ 5 ns per sample, 0.32 ms) | 0 | 1.4 ms (4 per tick) |
| Block updates | ≤ 0.2 µs per pop | 0.01 ms | 0.4 ms |
| Random ticks | ≤ 0.1 µs per pick | 0.03 ms | 0.1 ms |
| Liquids | ≤ 100 ns per cell | 0.4 ms | 1.6 ms (16,384 cells) |
| Gases | ≤ 90 ns per cell | 0.3 ms | 1.5 ms |
| Fire | ≤ 0.2 µs per burning cell | 0.05 ms | 0.8 ms (4,096 cells) |
| Temperature | ≤ 6 ns per awake thermal cell | 0.1 ms | 3.2 ms (all 524,288 cells awake; 0.9 ms on 4 workers) |
| Light | ≤ 50 ns per pop | 0.05 ms | 0.2 ms (4,096 pops) |
| Sound | ≤ 60 ns per settled cell | 0.15 ms | 2.0 ms (32,768 cells) |
| Folio light bake (§10.5) | ≤ 50 ns per pop | 15 ms per Folio entry | 210 ms (4,194,304 sky-lit cells), outside the tick, inside doc 02's Folio load budget |
| `step_mover` on the client predictor (doc 16 §7.3) | ≤ 3 µs per replayed tick | — | 0.36 ms (120 replayed ticks) |

The slow systems never share a tick (§2.3). This chapter's per-tick systems at their worst sum to `1.2 + 0.3 + 0.2 + 0.05 + 0.5 + 1.4 + 0.4 + 0.1 + 0.2 + 2.0 = 6.35` ms (movement, push and broadphase, environment, pose, projectiles, explosions, block updates, random ticks, light, sound). Adding the costliest slow system, temperature, gives 9.55 ms on one thread and 7.25 ms with its diffusion on 4 workers (0.9 ms). Doc 02 §15 adds every other chapter's systems and the snapshot work and shows the whole tick at most 14.50 ms inside the 16.67 ms period.

Memory for 128 chunks: blocks 2 MiB typical (4-bit palettes) and 8 MiB worst (16-bit), light 4 MiB, the four cell bitsets 2 MiB, thermal ≤ 2 MiB, liquids ≤ 6 MiB at 131,072 cells (a 32-byte `LiquidCell` plus map overhead), gases ≤ 2 MiB at 65,536 cells, fires ≤ 0.2 MiB: at most 24.2 MiB, inside doc 02's 384 MiB simulation budget.

---

## Exports

| Namespace | IDs defined here |
|---|---|
| `comp.` | `comp.ai`, `comp.body`, `comp.body_plan`, `comp.buildup`, `comp.container`, `comp.control`, `comp.corpse`, `comp.env`, `comp.equipment`, `comp.faction`, `comp.falling_block`, `comp.gadget`, `comp.interest`, `comp.inventory`, `comp.item_drop`, `comp.kind`, `comp.lifetime`, `comp.light_emitter`, `comp.mining`, `comp.mount`, `comp.move`, `comp.perception`, `comp.player`, `comp.pools`, `comp.pos`, `comp.pose`, `comp.projectile`, `comp.rider`, `comp.stats`, `comp.statuses`, `comp.tags`, `comp.vel`, `comp.zone` |
| `sys.` | the 39 systems of §2.2 in table order: `sys.tick_begin`, `sys.interest`, `sys.spawn_admit`, `sys.input_apply`, `sys.perception`, `sys.ai_think`, `sys.actions`, `sys.movement`, `sys.push`, `sys.env_sample`, `sys.broadphase`, `sys.pose`, `sys.melee`, `sys.projectiles`, `sys.hitscan`, `sys.explosions`, `sys.damage`, `sys.crafting`, `sys.block_updates`, `sys.random_ticks`, `sys.liquids`, `sys.gases`, `sys.fire`, `sys.temperature`, `sys.surfaces`, `sys.status_pulse`, `sys.damage_late`, `sys.resources`, `sys.coop`, `sys.needs`, `sys.light_emitters`, `sys.light`, `sys.sound`, `sys.hearing`, `sys.erasure`, `sys.status_timers`, `sys.despawn`, `sys.run_layer`, `sys.tick_end` |
| `grid.shape.` | `grid.shape.none`, `grid.shape.full`, `grid.shape.pillar`, `grid.shape.slab`, `grid.shape.stairs`, `grid.shape.door`, `grid.shape.trapdoor`, `grid.shape.ladder`, `grid.shape.fence`, `grid.shape.pane`, `grid.shape.layer`, `grid.shape.cross`, `grid.shape.fixture` |
| `grid.liqi.` | `grid.liqi.blocks`, `grid.liqi.permeable`, `grid.liqi.displaced`, `grid.liqi.absorbs` |
| `grid.rt.` | `grid.rt.none`, `grid.rt.grow`, `grid.rt.decay`, `grid.rt.spread` |
| `grid.ekind.` | `grid.ekind.erratum`, `grid.ekind.creature`, `grid.ekind.mount`, `grid.ekind.projectile`, `grid.ekind.item_drop`, `grid.ekind.falling_block`, `grid.ekind.container`, `grid.ekind.gadget`, `grid.ekind.zone`, `grid.ekind.corpse`, `grid.ekind.marker` (row order is the replicated kind code) |
| `grid.move.` | `grid.move.ground`, `grid.move.air`, `grid.move.swim`, `grid.move.climb`, `grid.move.mantle`, `grid.move.dodge`, `grid.move.fly`, `grid.move.downed`, `grid.move.ridden`, `grid.move.hover`, `grid.move.glide` |
| `grid.cause.` | `grid.cause.worldgen`, `grid.cause.script`, `grid.cause.placed`, `grid.cause.mined`, `grid.cause.broken`, `grid.cause.explosion`, `grid.cause.fire`, `grid.cause.liquid`, `grid.cause.unsupported`, `grid.cause.fell`, `grid.cause.landed`, `grid.cause.melted`, `grid.cause.froze`, `grid.cause.reaction`, `grid.cause.toggled`, `grid.cause.grew`, `grid.cause.decayed`, `grid.cause.spread`, `grid.cause.erased`, `grid.cause.reverted`, `grid.cause.expired` |
| `grid.despawn.` | `grid.despawn.expired`, `grid.despawn.consumed`, `grid.despawn.picked_up`, `grid.despawn.merged`, `grid.despawn.folio_exit`, `grid.despawn.cap_evicted`, `grid.despawn.landed`, `grid.despawn.dismissed`, `grid.despawn.script` |
| `grid.gait.` | `grid.gait.crouch`, `grid.gait.walk`, `grid.gait.sprint`, `grid.gait.crawl` |
| `ev.` (bus events of this chapter) | `ev.explosion`, `ev.fire_started`, `ev.fire_out`, `ev.footstep`, `ev.entity_landed` |
| `ev.` (registered for other chapters, §14.3) | `ev.wound_changed`, `ev.augment_changed`, `ev.entity_pacified`, `ev.secret_found`, `ev.objective_completed`, `ev.cast_started`, `ev.cast_interrupted`, `ev.spell_fizzled`, `ev.wild_surge`, `ev.glyph_learned`, `ev.enchant_applied`, `ev.prayer_resolved`, `ev.favor_changed`, `ev.alarm_raised`, `ev.body_discovered`, `ev.disguise_blown`, `ev.lock_opened`, `ev.object_revealed`, `ev.device_hacked`, `ev.takedown`, `ev.item_stolen`, `ev.rubric_completed`, `ev.colophon_entered`, `ev.colophon_desecrated`, `ev.boon_committed`, `ev.note_completed`, `ev.exit_committed`, `ev.boss_engaged`, `ev.volume_completed` |
| `block.` (shared with 07) | `block.air` (type index 0) |
| `rng.` | none new; this chapter draws only `rng.fire`, `rng.blocktick`, `rng.physics` (Canon §3.2; Canon gaps G17) |

Non-ID exports: `step_mover`, `MoverParams`, `BlockQuery`, `MoverOut`, `select_mode` (§5); `set_block`, `EditError`, `damage_block`, `mining` progress, `can_place`, `erase_placed`, `is_errata_placed`, `set_protect` (§4.7, §13); `state_at`, `solid`, `opaque`, `material_at`, `destructible_at`, `raycast_blocks`, `cell_conceal`, `cell_conceal_parts` (§4.5, §5.18, §7.6); `deposit`, `splash`, `drain`, `surface_trigger`, `emit_gas`, `emit_gas_volume` (§6, §7); `ignite` (§8.1); `add_heat` (§9.2); `add_light`, light overrides, `flashes` (§10); `emit_noise`, `HeardNoise` (§11); `queue_explosion`, `ExplosionSpec` (§12); `apply_knockback` (§5.13); `push_contacts` (§5.14); `request_spawn` (§15.3); `PortalPair` (§5.21); the event payload schemas of §14.

## Interface assumptions

| Doc | This chapter assumes |
|---|---|
| 02 | the primitives, store, scheduler, `PlayerCommand` axes and bits, `SimEvent`, section order and `palette_rle` as imported; `palette_rle` also accepts the 4,096-entry thermal array; `SCHEDULE` lists the 39 systems of §2.2, so doc 02 §15 adds budget rows for `sys.crafting`, `sys.resources`, `sys.coop` and `sys.run_layer`; `ChunkView` carries `fluid_gen` (§4.3); copy-on-write chunk clones grow by the fourth bitset (`flip`, 4 KiB); the view frame publishes active light overrides (§10.6) and the renderer clamps the light field with them; `extract_view` and doc 16's `ReplFrame` extraction are read-only passes outside the schedule; doc 02 §9 plays `ev.noise` for slot 4 like any party slot |
| 04 | Errata `BodyState` values; `can_sprint`, `can_jump`, `can_dodge` and stamina spending; `body_move_bp(e)` (§8.5) replacing any leg or crutch factor here; `air_jump_max(e)` (§2.3); the Acrobatics 50 wall jump and `wind_step` wall-run as components that set `MoverParams.wall_jump` and `wall_run_max_mm` (2000); breath drain from `Env.eye_submerged` and unbreathable eye gas; fall-damage leg split and fall factors after `fall_dmg_bp`; per-tick pool regeneration inside `sys.resources`; the `body.*` part lists that pose tables reference |
| 05 | `HitRecord`, `queue_hit` and the pipeline in `sys.damage` and `sys.damage_late`; combat steps C1 to C9 mapped as in §2.2 (C1 runs before movement); calls to `surface_trigger` and `damage_block` with the §13.2 `block_bp` table; projectile templates (`radius`, `gravity_bp`, `drag_bp` 0, `bounce_bp` 4000 for fused grenades, pierce, lifetime) and the four §5.19 hooks with doc 05 §7 semantics; `LAGCOMP_MAX_TICKS = 16`; melee traces on doc 02 primitives; Contradiction re-evaluation called from `sys.env_sample`; `kb_resist_bp`; animation choice; stance base speeds and armor sprint and swim bases in `MoverParams`; mount archetypes with seat offsets; item explosion parameters mapped as in §12.1, with impact and status powers scaled by `f = mul_bp(falloff, exposure)` of §12.4 in place of doc 05 §9.4's binary line-of-sight test; friendly-fire scaling from `instigator_slot` |
| 06 | `aether_tick` inside `sys.resources`; cast resolution inside `sys.actions`; `ignite`, `add_light` and `add_heat` calls; conjured blocks placed through doc 07 `placed_meta` with the conjured flag and removed at expiry with cause `grid.cause.expired`; `surface_trigger` for the cells a spell touches; `st.levitating` control |
| 07 | every block record with the §4.5 fields and `tags`; the material-to-block defaults of doc 07 §2.5; `mining_ticks` and `block_hp`; `block_drops(state, cause, actor)`; ordered `tooltier.*`; `placed_meta`, `stations` and `crops` saved in section 4 and cleared by `on_block_changed` for foreign causes; placement rules after `can_place`; `sys.crafting`; `block.ice` (`friction_bp` 1500, melts to 1000 units of water) and `block.web` (`burn_runs` 5) |
| 08 | liquid records (§6.5) and gas records (§7.2) for every `liq.` and `gas.` ID, including `evap_gas`; `alch_react` (§6.8); extended trigger rows and contact rows; `splash` amounts; nitro detonation specs with `block_power_milli` set directly (doc 08's "block-break power 2" is 2000) |
| 09 | noise IDs, `noise_level` and `convert_events()` in the `sys.sound` prelude; consumption of `HeardNoise`; perception through `raycast_blocks`, `cell_conceal_parts`, light, overrides and `flashes`; `door_locked(gidx)`; `awareness_above_unaware(e)` for sleep; reading `push_contacts` |
| 10 | the appended `ev.item_identified` fields; appraisal passes inside `sys.needs` after doc 04's needs work |
| 11 | item-drop contents, pickup, Lost Satchel merging at `GROUND_ITEM_CAP`, `carried_g`, death drops in `sys.despawn`; `perk.leg.worldbreaker` applied by doc 05 before `damage_block` |
| 12 | worldgen chunks with `open_tag`, `protect` and `flip` bits; relink pairs as `PortalPair` records; liquid sources; bleed zones for `local_genre`; GPV fields `ambient_decic`, `liquid_viscosity_bp`, `gas_dissipation_bp`; hazard placement; worldgen calls the §10 propagation for its light pass |
| 13 | AI writes `control`; `request_spawn`; `comp.tags` bit layout (doc 13 §2.4); `Blackboard` as `comp.ai`; creature `BodyState` templates with radius at least 100 mm; navigation rebuilt from `ev.block_changed`; Editor erasure through `erase_placed`; `blackout` through a light override; telegraphs published on the per-tick list read by `extract_view`; the invader template `enemy.editors.invader` carries `comp.player` slot 4 and no `comp.ai` |
| 14 | `sys.run_layer` logic; `protect` bits for the sealed Colophon entrance cleared through `set_protect` |
| 16 | delivery of commands and intents per tick for slots 0 to 4; the predictor calls `step_mover` with `MoverParams` built from replicated state and must carry the active move-speed status list (`status_speed_bp`) or the `gates` bits that imply it, because a single folded `speed_bp` rounds differently from Canon §4.2's per-status chain; `Player` semantics and `sys.coop` logic; lag compensation through `Pose.hist`; snapshot cadence |
| 17 | `mercy_on` and `mercy_bp` stored in `comp.player` |
| 18 | `art.anim.*` pose tables committed as content with the §5.16 schema, keyed by `anim`; `art.block.*` skins |

## Canon gaps

- **G1. Version.** The header conforms to canon-1.4. Every rule was checked against it: the 1.2 jump curve, air jump, lean, two-channel light, surfaces, concealment, breath, mounts and ghosts; the 1.3 World Laws, seams and Colophon; and the 1.4 fixed terrain, placeables, Editor erasure, invasions with replay slot 4, and the 44-event list including `ev.invasion_started` and `ev.invasion_ended`.
- **G2. Liquid units.** Canon types liquid volume as `Ml`. The grid stores 1-litre units (1000 ml, 1000 per cell), so `Ml = units × 1000` at every boundary. Canon §12.1's 100 ml contact threshold is therefore met by any non-zero unit.
- **G3. Explosion draws.** Canon §3.3 keys every action-caused draw, while §3.2 assigns explosion block-break variation to sequential `rng.physics` and fire ignition to `rng.fire`. Explosions resolve host-side in the §12.1 order and are never client-predicted, so this chapter uses those sequential streams for block breaks, drops and incendiary ignition. Requested: Canon §3.3 names this exception.
- **G4. Bus events.** Canon §19 gives `ev.` to 00 for core lists and to 03 for the bus. This chapter defines five bus events of its own and registers 29 events that docs 04, 06, 09, 12, 13 and 14 emit (§14.3), with payloads. Requested: Canon §18 lists them or points to §14.3.
- **G5. Two hit batches.** Canon §10.8's `(source EntityId, seq)` order holds inside `sys.damage` and inside `sys.damage_late`; environmental, contact and DoT hits of a tick resolve after weapon hits.
- **G6. Ghosts.** "A ghost has no collision" is read as no body collision and no hit, push or perception queries; voxel collision stays so ghosts cannot leave the Folio. Ghosts toggle doors (Canon §17.6) and never place or break blocks (§13.1).
- **G7. Ambient temperature.** The canon has no per-genre temperature. This chapter reads doc 12 §2.3's GPV field `ambient_decic`, default 200.
- **G8. Surface-trigger geometry.** Canon §12.1 does not say which cells a trigger touches, how much water becomes steam, or what concentration counts as gas contact. This chapter fixes trigger cells, the 64-cell charge, the 100-unit freeze minimum, the 27-cell steam cap, one blast per hit, and gas contact at concentration 500 in the eye cell (doc 08 adopts 500).
- **G9. Sound units.** `Db` is whole decibels; propagation runs in tenths and floors to whole dB on output.
- **G10. Sky channel.** "Open areas" (Canon §5.2) means sky-exposed or worldgen-tagged cells; the sky channel then propagates like block light, so `ambient_light` is the level at the open cells and falls off by 1 per step away from them.
- **G11. Pose keyframes.** Content stores keyframes every `key_every` ticks; the loader expands them to the canon per-tick table with integer interpolation.
- **G12. Serialization encodings.** Doc 02 §6.5 lists palette-RLE for fluids and thermal; sparse liquid and gas lists and `u16` thermal offsets are used (§16.1).
- **G13. Genre fall multiplier.** Canon §5.9 `fall_dmg_bp` is applied to the canon fall damage before doc 04's fall factors.
- **G14. Fixed terrain enforcement.** Canon §4.3 names a `destructible` tag but not every rule it implies. This chapter makes it `block.tag.destructible` (doc 07), sets `hardness_milli = -1` without it, refuses type changes of non-destructible cells for every cause except worldgen and script, allows prop-only changes (doors, growth, connections), and forbids non-destructible blocks from burning, melting, falling, washing away, decaying or exploding (§4.5 loader rule 2).
- **G15. Steam balance.** Canon §12.1 turns fire on water into "1 cell per 1000 ml" of steam. This chapter makes 1 liquid unit equal one saturated gas cell everywhere: boiling emits one cell per unit, and condensation returns 1 unit per 10,000 concentration. The earlier rate of 1 unit per 20 concentration let one unit of water become 500 units through fire and condensation. Freezing and melting likewise restore exactly the frozen units (`timed_revert`).
- **G16. Web burnout.** Canon §12.1's "burns out in 60 ticks" is realized as 5 fire runs, which ends 48 to 60 ticks after ignition depending on the fire phase (§8.4).
- **G17. RNG streams.** No stream beyond Canon §3.2 is needed. `rng.fire` covers spread and incendiary ignition, `rng.blocktick` random ticks, `rng.physics` explosion block intensity and drops. Liquids, gases, temperature, light, sound, falling blocks, movement and every surface rule draw nothing. Every draw is listed with its count in §8.3, §12.2, §12.3 and §13.7.
- **G18. Combat phase position.** Doc 05 §1.3 places its whole combat phase after movement. This chapter runs C1 (action state machines) in `sys.actions` before movement, so actions read the previous boundary's positions, and runs C2 to C9 after movement as doc 05 states (§2.2).
- **G19. Explosion resolution.** Docs 03 and 05 both described explosions. This chapter's algorithm governs: the ray method for blocks (block power from `block_bp` as in §12.1, not doc 05's per-block falloff), doc 05's distance point, inner radius and edge factor for falloff, and a 5-sample exposure fraction in place of doc 05's binary line-of-sight test.
- **G20. Errata-placed registry.** Doc 13 asks for a `placed_by_errata` bit per block and doc 07 keeps `placed_meta`. They are one store: `is_errata_placed(c)` reads `placed_meta` (§13.8).
- **G21. Pacification event name.** Doc 04 names `ev.entity_pacified` and doc 13 names `ev.pacified`. The bus registers `ev.entity_pacified` only (§14.3), matching `ev.entity_killed`.
- **G22. Portals.** Doc 12 §9.4 asks this chapter to apply relink door pairs. Only bodies and projectiles cross portals; every grid and every ray uses physical adjacency (§5.21).
- **G23. Darkness overrides.** Doc 13's `blackout` forces effective light to 0 in an arena. Canon §2.3's `light_at` reads the grid; this chapter applies overrides as a read-time clamp on every light read, including spawn light (§10.6).
- **G24. Mantle targets.** Canon §4.2 gives fences no special rule. Fences and panes are never mantle targets, so a 1500 mm fence contains anyone below VERVE 7 (§5.9).
- **G25. Asleep listeners.** Asleep creatures (inactive chunk, no awareness) hear nothing. A noise loud enough to matter is emitted near an Erratum, whose window keeps chunks within 2 columns active, so the case is limited to sounds that carry more than 64 m.
- **G26. System numbering.** Docs 09, 10, 13 and 14 cite system numbers. The four systems added for docs 04, 06, 07, 14 and 16 carry letter suffixes (17a, 26a, 26b, 34a) so no cited number changes.

## Acceptance tests

1. **Schedule.** For `t` in `0..60`, due ticks equal §2.3; only liquids and needs coincide, at `t ≡ 53`. `SCHEDULE` lists exactly the 35 IDs of §2.2 in order.
2. **IDs.** In Folio 3 the first two spawns get `(3 << 40) | 1` and `| 2`; every column stays ascending after 1,000 random spawns, despawns and compactions.
3. **Spawn visibility.** A projectile spawned in `sys.actions` on tick 100 is swept by `sys.projectiles` on tick 100 with `age = 0`.
4. **Palette.** A chunk holding 9 distinct states uses 4 bits and 2,048 words. Two chunks with identical blocks but different edit histories produce byte-identical `palette_rle`.
5. **Block state.** `block.air` has type index 0. `set_block` with a stray prop bit returns `bad_props`. A closed door at facing W has the box `[0,0,0]–[190,1000,1000]`.
6. **Wish.** `move_x = 90`, `move_z = −90` classifies as strafe; at AGI 60 the speeds are strafe 4289, forward 4515, forward with `st.slowed` 3160 mm/s.
7. **Acceleration.** Ground acceleration per tick is raw 2863311520; from rest, forward walking reaches 4300 mm/s on tick 7.
8. **Wall.** The §5.4 wall example ends with `p.x` raw 45956150068.
9. **Floor.** The §5.4 floor example ends with `p.y` exactly raw 4294967296, `v.y = 0`, grounded.
10. **Jump.** The §5.7 table reproduces: `v0` raws 29205417893, 31085832879, 39131183219, 34932400672; each apex within 1 mm of `JUMP_H`; the Wuxia air jump apex is 1199.874 mm.
11. **Step and mantle.** A 500 mm slab is climbed without a jump; a 1000 mm block is not. Mantles of 1000, 1600 and 560 mm take 18, 24 and 14 ticks; 1700 mm fails.
12. **Climb, dodge.** Climb speed at GRP 60 is 2640 mm/s. A dodge moves raw 12884901840 in 12 ticks.
13. **Knockback, push.** 8000 mm/s on 120 kg gives 5000 mm/s. Two 75 kg bodies 500 mm apart each move 50 mm.
14. **Fall.** A 6 m fall at VERVE 5 with Max HP 85 queues 13,600 mHP; in Wuxia (`fall_dmg_bp` 5000) 6,800 mHP.
15. **Hitbox.** Model point (250, 1000, 0) mm at yaw 4096 has world offsets raw (992008094, 4294967296, −410903207).
16. **Raycast.** The §5.18 example visits the six listed cells and hits (4, 1, 8) through +Z at raw 16589703354. A direction with an x component of raw 100 runs without panic and never steps in x.
17. **Liquids.** The §6.6 channel gives `[0,200,600,200,0]`, then `[56,224,432,248,40]`, settles after run 19 at `[200,200,201,200,199]`, and sums to 1000 after every run. Lava's run 1 gives `[0,30,940,30,0]`.
18. **Liquid details.** A full water cell over an empty cell moves 1000 units in one run; lava moves 150. Extracting 200 from water 600 and oil 300 yields 134 and 66; the cell's flow is 9000.
19. **Fairness.** With 20,000 active liquid cells every cell is evaluated within 2 runs.
20. **Gases.** The §7.3 smoke example ends at 3271, with 1760 in the cell above (1400 risen plus 360 diffused) and 480 in each horizontal neighbor before their own runs.
21. **Fire.** Every burning cell draws exactly 6 `rng.fire` values per run; 10 burning cells draw 60. The oak-plank spread chance above an intensity-2 fire is 3750 Bp.
22. **Temperature.** Air–air flux 100, air–stone flux 20, relaxation 40, one lava block 270 per run; the `T = 1000` hazard at Folio 10 against an Erratum is 3,238 mHP.
23. **Light.** A torch on an open floor lights 1,834 cells in one tick; the cell 3 steps away reads 11; through one water cell 12 then 11. Removing the torch returns every cell to 0 within one tick (1,834 dark pops, no lit pops). No tick exceeds 4,096 pops.
24. **Sky light.** A Mythic Fantasy courtyard reads `light_sky` 4 and its corridor 3, 2, 1, 0. A spawn query in the courtyard with `light_block` 0 passes `spawn_block_max` 0.
25. **Sound.** A 130 dB shot is heard at 110 dB 10 m down a corridor and 95 dB through a closed oak door; a 45 dB step at 10 m gives 25 dB; candidates lie within 68,421 mm.
26. **Explosion rays.** The table has 1,352 directions with entries 0 and 1215 as in §12.2; the worked ray marks only (13, 2, 10); draws equal 1,352 plus breakable marked cells.
27. **Explosion damage.** The §12.4 example gives falloff 6250, 20,000 mHP and 5000 mm/s knockback.
28. **Block damage.** A 1500-hardness block breaks on the 13th 12,000 mHP sword hit and the 2nd pickaxe hit; an entry untouched for 600 ticks is removed.
29. **Random ticks.** Each active chunk draws exactly 16 `rng.blocktick` values per tick.
30. **Surface triggers.** Shock on a 100-cell lake charges exactly 64 cells; frost leaves a 99-unit cell liquid and freezes a 100-unit cell; the ice reverts at `t + 1200` with its units; fire on 3 units of oil deals a blast of base 30,000 mHP.
31. **Contact.** An Erratum standing in 1 unit of `liq.water` gains `st.soaked` on the next pulse; with `gas.miasma` 400 in the eye cell nothing applies, with 500 it gains 20 poisoned buildup before scaling.
32. **Save.** SyncTest with depth 8 during liquid flow, fire spread and a relight backlog keeps every hash equal; save at tick 4,321 and reload matches the uninterrupted run (Canon §21 test 32).
33. **Events.** `seq` starts at 0 each tick and follows system order; the event list is empty at every boundary.
34. **Sleep.** A creature in an inactive chunk keeps its position for 600 ticks, takes a hit, and moves on the next tick after waking.
35. **Performance.** Every §17 unit-cost benchmark passes on the Deck runner.

## Invariants annex

```
Σ sim-core: tick order ⊗ entities ⊗ voxels ⊗ collision ⊗ grids ⊗ events ⊗ saves
W! system order := §2.2 ∀ tick                         W! due(t) := (t + phase) % period == 0
W! iterate ascending EntityId | Gidx | FIFO           N! slot/dense index → outcome
W! EntityId never reused within a run                 N! HashMap|HashSet|f32|f64 ∈ sim state
W! block.air == BlockState 0                          N! stray prop bits ∈ BlockState
W! set_block := only post-worldgen block writer       W! every set_block → ev.block_changed
W! collision := doc 02 primitives + §5.4 push         N! physics engine in sim
W! push depth exact, velocity clipped on contact      N! tunnelling: substep ≤ 125 mm
W! liquids conserve except §6.7 sinks/sources         N! liquid rises without SWAP|source
W! gases flow high → low only                         W! thermal flux antisymmetric (conserves)
W! light pops ≤ 4096/tick, dark before lit           W! light saved, never recomputed on load
W! liquids|gases ≤ 16384 cells/run, oldest first      W! block updates ≤ 2048/tick; explosions ≤ 4/tick
W! sound shares 32768 settled cells/tick              W! fallback := ray estimate, exact=false
W! rng.fire ∈ {sys.fire, incendiary}                  W! rng.physics ∈ sys.explosions only
W! rng.blocktick: 16 draws per active chunk per tick  W! fire: 6 draws per burning cell per run
N! event read across ticks                            N! event list ∈ save
W! hitboxes := baked pose tables, 32-tick history     N! render pose → sim
W! renderer light := sim light (field texture)        N! skin stored per block
W! restore → identical state_hash                     N! derived cache ∈ serialization
```
