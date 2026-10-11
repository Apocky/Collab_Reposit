# 03 — Simulation Core

As of 2026-10-11. Status: BINDING. Conforms to canon-1.1.

This chapter defines the core of `palimpsest_sim`: the order of systems inside one tick, the entity model and component catalog, voxel world storage and the block registry, the character controller and every collision primitive the simulation uses, the six environmental grids (liquids, gases, fire, temperature, light, sound), explosions, block edits, the event bus with every payload schema, activation, saves and state-hash coverage, and per-subsystem performance budgets. Every other system chapter plugs into the hooks named here. The header line above is fixed by the document contract; the canon on disk at writing time is canon-1.3, and every rule below was checked against canon-1.3 (Canon gaps G1).

## Imports

| Source | Section | What this chapter consumes |
|---|---|---|
| Canon | §1.1 | content-ID grammar, `id32 = fnv1a32` |
| Canon | §1.2–§1.6 | numeric types incl. two-channel `Light`; integer and `Fx` primitives; display units; integer-only content |
| Canon | §1.8–§1.9 | coordinates, chunk and local index, Folio box, `block.void_bedrock`; Bam, `sin_bam`, `cos_bam`, `fwd` |
| Canon | §1.11 | `EntityId` layout, phase key, ascending-ID iteration, no `HashMap` |
| Canon | §2.1–§2.5 | `DT`, semi-implicit Euler, tick schedule, rate accumulator, `in_combat`, `light_at`, replay record, save contents, `state_hash` |
| Canon | §3.1–§3.4 | xoshiro256++, `stream_seed`, `keyed_rng`; sequential streams `rng.fire`, `rng.blocktick`, `rng.physics` (owner 03); draw-count rule |
| Canon | §4–§4.2 | world constants and caps; humanoid body; speeds, accelerations, jump, air jump, lean, fall damage, dodge |
| Canon | §5.2, §5.8, §5.9 | `gravity_bp`, `ambient_light`; `local_genre`, seam zones, `vrv(e)`; physics columns `air_accel_bp`, `fall_dmg_bp`, `ground_friction_bp`, `spawn_block_max` |
| Canon | §6.4 rows 11, 21 | move-speed and climb-speed multipliers |
| Canon | §9, §9.1 | body plans, baked pose tables, 32-tick pose history, severing effects on movement |
| Canon | §10.1, §10.5, §10.6, §10.8 | damage types; pipeline entry; environmental-hazard, DoT and fall-damage entry points; same-tick hit order |
| Canon | §11.1–§11.4 | statuses that gate or modify movement and that grids apply; buildup formula; direct application |
| Canon | §12, §12.1–§12.3 | `res.breath` contact rule; the surfaces-and-reactions table; sight concealment; spawn light |
| Canon | §13.8 | `P(d)`, `T(d)`, `S(lvl)` for environmental hazards |
| Canon | §13.8a | one material table for blocks and items |
| Canon | §17.5–§17.7 | `FF_AREA_BP` applies to explosions, fire spread, surfaces, falling blocks; ghosts; mounts |
| Canon | §18, §19 | the 42 canonical events; namespaces `comp.`, `sys.`, `grid.` (03), `ev.` and `rng.` bus (03), `block.` shared with 07 |
| Doc 02 | §4.1–§4.6 | `Fx`, `Fx3`, `isqrt_u128`, `atan2_bam`, `closest_seg_seg`, `capsule_overlap`, vertical-capsule-vs-box distance, `ray_capsule`, `dda` |
| Doc 02 | §5.1–§5.5 | sorted-column store, deferred structural commands, `SystemDesc` and `due`, `step` lifecycle, `PlayerCommand` axes and button bits, intents, `SimEvent` |
| Doc 02 | §6.5, §8.5, §9 | serialization sections and `palette_rle`; light field texture and agreement clamp; audio's use of `ev.noise` |
| Docs 04–18 | — | not yet written; every needed interface is listed under Interface assumptions |

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

Systems run in exactly this order every tick. A system that is not due is skipped. After each system the scheduler applies deferred spawn and despawn commands in request order (doc 02 §5.2), so an entity created by system `k` is first iterated by system `k + 1`.

| # | System ID | Logic owner | Period, phase | Work |
|---|---|---|---|---|
| 1 | `sys.tick_begin` | 03 | 1, 0 | reset per-actor action `seq` counters to 0; open the event list; clear `flashes` and `TELEPORTED` flags; on `folio_entry_tick` emit `ev.folio_entered` |
| 2 | `sys.interest` | 03 | 1, 0 | recompute the active-chunk set and actor sleep flags (§15) |
| 3 | `sys.spawn_admit` | 03, 13 | 1, 0 | admit queued AI spawns FIFO while live AI actors < `ACTIVE_ACTOR_CAP` (§15.3) |
| 4 | `sys.input_apply` | 03, 16 | 1, 0 | copy each slot's `PlayerCommand` for `t` into `comp.control` in slot order; apply intents in `(slot, action_seq)` order; derive button presses |
| 5 | `sys.perception` | 09 | 3, per entity | sight checks for awake observers (uses §5.18 DDA, §10 light, §7.6 concealment) |
| 6 | `sys.ai_think` | 13 | 6, per entity | awake agents write `comp.control` |
| 7 | `sys.actions` | 05, 06, 07, 11 | 1, 0 | action state machines: attacks, casts, item use, interaction, mining progress (§13.2), placing (§13.3), doors; queue hits, spawn projectiles, queue explosions, request block edits, emit noise |
| 8 | `sys.movement` | 03 | 1, 0 | §5 for every body, ascending `EntityId`; mounts before riders' slaving |
| 9 | `sys.push` | 03 | 1, 0 | entity–entity separation (§5.14) |
| 10 | `sys.env_sample` | 03 | 1, 0 | per body: local genre, Contradiction re-evaluation hook on change (doc 05), contact cells, immersion, eye cell, temperature, light (§5.20) |
| 11 | `sys.broadphase` | 03 | 1, 0 | rebuild the spatial grid (§3.6) |
| 12 | `sys.pose` | 03 | 1, 0 | advance `anim_tick`, append the pose history record (§5.16) |
| 13 | `sys.hitscan` | 05 on 03 primitives | 1, 0 | lag-compensated rays (§5.17, §5.18) |
| 14 | `sys.melee` | 05 on 03 primitives | 1, 0 | melee capsule traces against hitboxes |
| 15 | `sys.projectiles` | 03, 05 | 1, 0 | §5.19 |
| 16 | `sys.explosions` | 03 | 1, 0 | at most 4 due explosions (§12) |
| 17 | `sys.damage` | 05 | 1, 0 | resolve the hit queue in ascending `(source EntityId, seq)` (Canon §10.8); fire surface triggers (§6.9) |
| 18 | `sys.block_updates` | 03 | 1, 0 | at most 2048 queued cell updates (§13.5) |
| 19 | `sys.random_ticks` | 03 | 1, 0 | 8 cells per active chunk (§13.7) |
| 20 | `sys.liquids` | 03, 08 | 6, 1 | §6 |
| 21 | `sys.gases` | 03, 08 | 6, 4 | §7 |
| 22 | `sys.fire` | 03 | 12, 2 | §8 |
| 23 | `sys.temperature` | 03 | 30, 5 | §9 |
| 24 | `sys.surfaces` | 03 (table: Canon §12.1) | 15, 0 | surface contact effects (§6.10) |
| 25 | `sys.status_pulse` | 05 | 15, 0 | DoT, HoT, buildup decay |
| 26 | `sys.damage_late` | 05 | 1, 0 | resolve hits queued after step 17 (environment, contact, DoT, chains) in ascending `(source EntityId, seq)` |
| 27 | `sys.needs` | 04 | 60, 7 | satiation, toxicity, composure, breath |
| 28 | `sys.light_emitters` | 03 | 1, 0 | sync dynamic light emitters (§10.4) |
| 29 | `sys.light` | 03 | 1, 0 | at most 4,096 relight updates (§10.5) |
| 30 | `sys.sound` | 03 | 1, 0 | propagate every `ev.noise` emitted this tick (§11) |
| 31 | `sys.hearing` | 09 | 1, 0 | consume this tick's `HeardNoise` records |
| 32 | `sys.erasure` | 13 | 1, 0 | Erasure stacks on their exact ticks (Canon §11.6) |
| 33 | `sys.status_timers` | 05 | 1, 0 | end-of-tick status expiry (Canon §2.3) |
| 34 | `sys.despawn` | 03, 11 | 1, 0 | remove dead and expired entities; death drops (doc 11); `ev.entity_despawned` |
| 35 | `sys.tick_end` | 03, 16 | 1, 0 | close the event list into `StepOutput`; column compaction (doc 02) |

Hashing at boundaries `b ≡ 0 (mod 60)` and network snapshots on even ticks happen outside the schedule (doc 02 §5.4, doc 16).

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
2. An entity spawned during system `k` exists from system `k + 1`. Its `spawn_tick` is `t`.
3. A despawn request marks the entity dead after the requesting system. Dead entities are skipped by every later system and removed by `sys.despawn`.
4. Events are visible to every system later in the same tick. No system reads an event from an earlier tick. Information that must survive a tick boundary lives in components or grids, so the event list is never part of saved state.
5. Block edits apply immediately at the call (§13.1). A later system in the same tick sees the new block.
6. Random draws: `rng.blocktick` only in `sys.random_ticks`; `rng.fire` only in `sys.fire` and fire ignition by explosions (§12.3); `rng.physics` only in `sys.explosions`. No other system in this chapter draws.

### 2.5 Parallelism

The simulation runs on one thread (doc 02 §5.3). Besides worldgen (doc 02), two pure computations may be split across worker threads through doc 02's `OrderedMap`, because their results merge in index order: the ray marches of one explosion (§12.2; all 1,352 intensity draws are taken in table order first, then rays march in parallel and merge in ray order) and the temperature diffusion of one run (§9.3, Jacobi over a snapshot, merged in thermal-chunk order). Nothing else in this chapter runs in parallel.

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

### 3.2 Entity kinds

| Kind ID | Meaning | Required components | Optional components |
|---|---|---|---|
| `grid.ekind.erratum` | a player character, alive, downed or ghost | `kind`, `pos`, `vel`, `body`, `move`, `control`, `player`, `faction`, `tags`, `pools`, `statuses`, `buildup`, `body_plan`, `pose`, `stats`, `inventory`, `equipment`, `env`, `interest` | `light_emitter`, `mining`, `rider` |
| `grid.ekind.creature` | any NPC: enemy, neutral, minion, hireling | `kind`, `pos`, `vel`, `body`, `move`, `control`, `faction`, `tags`, `pools`, `statuses`, `buildup`, `body_plan`, `pose`, `stats`, `ai`, `perception`, `env` | `inventory`, `equipment`, `light_emitter`, `interest` |
| `grid.ekind.mount` | a ridden or summoned mount (Canon §17.7) | creature set plus `mount` | `light_emitter` |
| `grid.ekind.projectile` | any moving shot, thrown item or spell bolt | `kind`, `pos`, `vel`, `projectile` | `light_emitter` |
| `grid.ekind.item_drop` | an item lying in the world | `kind`, `pos`, `vel`, `body`, `move`, `item_drop` | `lifetime` |
| `grid.ekind.falling_block` | a block in free fall | `kind`, `pos`, `vel`, `body`, `move`, `falling_block` | — |
| `grid.ekind.container` | chest, Reliquary, Fortune Engine, shop counter | `kind`, `pos`, `container` | `pools`, `body_plan`, `pose` |
| `grid.ekind.gadget` | turret, trap, mine, drone base | `kind`, `pos`, `gadget`, `faction` | `pools`, `body_plan`, `pose`, `vel`, `body`, `move`, `light_emitter` |
| `grid.ekind.zone` | an area effect: spell field, aura, trigger volume | `kind`, `pos`, `zone` | `lifetime`, `light_emitter` |
| `grid.ekind.corpse` | NPC remains kept for loot or necromancy | `kind`, `pos`, `vel`, `body`, `move`, `corpse` | `lifetime` |
| `grid.ekind.marker` | a non-physical anchor: spawn point, scripted interest anchor, exit | `kind`, `pos` | `interest`, `lifetime` |

### 3.3 Component catalog

Columns serialize in ascending component-ID byte order, which is the order of this table (doc 02 §6.5 section 2). "Semantics" names the chapter that owns the meaning of the fields; this chapter owns the storage contract.

| Component ID | Rust type | Semantics |
|---|---|---|
| `comp.ai` | `AiState` (opaque blob) | 13 |
| `comp.body` | `BodyState` | 03 |
| `comp.body_plan` | `BodyPlanState { plan: Id32, parts: Vec<PartState> }`, `PartState { part: Id32, cond: Milli, cond_max: Milli, wound: Id32, severed: bool }` | 04 |
| `comp.buildup` | `Buildup { meters: Vec<Meter> }`, `Meter { status: Id32, value: i32, last_add: Tick, protect_until: Tick, protect_bp: Bp }`, sorted by `status` | 05 |
| `comp.container` | `ContainerState { inv: Vec<u64>, opened_by: u8 /* slot bitmask */, kind: Id32 }` | 11 |
| `comp.control` | `Control` | 03 |
| `comp.corpse` | `CorpseState { template: Id32, killed_tick: Tick, looted_by: u8 }` | 13 |
| `comp.env` | `Env` | 03 |
| `comp.equipment` | `Equipment { slots: Vec<(Id32 /* slot.* */, u64 /* uid */)> }` sorted by slot ID | 11 |
| `comp.faction` | `FactionC { faction: Id32, owner_slot: u8 /* 0xFF none */ }` | 13 |
| `comp.falling_block` | `FallingBlock` | 03 |
| `comp.gadget` | `GadgetState` (opaque blob) | 07 |
| `comp.interest` | `Interest { radius_chunks: u8 }` | 03 |
| `comp.inventory` | `Inventory { uids: Vec<u64> }` ascending | 11 |
| `comp.item_drop` | `ItemDrop { uid: u64, for_slot: u8 /* 0xFF any */, pickup_tick: Tick }` | 11 |
| `comp.kind` | `Kind { kind: Id32, template: Id32, spawn_tick: Tick, owner: Option<EntityId>, flags: u32 }` | 03 |
| `comp.lifetime` | `Lifetime { end_tick: Tick }` | 03 |
| `comp.light_emitter` | `LightEmitter { level: u8, admitted: Option<Gidx> }` | 03 |
| `comp.mining` | `Mining` | 03, 07 |
| `comp.mount` | `MountState { arch: Id32, riders: [Option<EntityId>; 2], gallop: bool }` | 05 |
| `comp.move` | `MoveState` | 03 |
| `comp.perception` | `PerceptionState` (opaque blob) | 09 |
| `comp.player` | `Player { slot: u8 }` | 16 |
| `comp.pools` | `Pools { hp, hp_max, stamina, stamina_max, aether, aether_max, composure, composure_max, poise, poise_max, ward, toxicity, breath, breath_max: Milli, satiation: i32 }` | 04, 06 |
| `comp.pos` | `Pos { p: Fx3, yaw: Bam, pitch: i16 }` | 03 |
| `comp.pose` | `Pose` | 03 |
| `comp.projectile` | `Projectile` | 03, 05 |
| `comp.rider` | `Rider { mount: EntityId, seat: u8 }` | 05 |
| `comp.stats` | `Stats` (sub-attributes, skills, level; opaque here) | 04 |
| `comp.statuses` | `Statuses { list: Vec<StatusInst> }`, `StatusInst { id: Id32, applied: Tick, expiry: Option<Tick>, stacks: u8, pool: Milli, source: EntityId, potency_bp: Bp, acc: i64 }`, sorted by `id` | 05 |
| `comp.tags` | `Tags { bits: u64 }` (undead, fiend, divine, eldritch, construct, electronic, flier, swimmer, swarm, breathless, …; bit positions owned by doc 13) | 13 |
| `comp.vel` | `Vel { v: Fx3 }` | 03 |
| `comp.zone` | `ZoneState { shape: ZoneShape, effect: Id32, period: Ticks, next_pulse: Tick }` | 06 |

For Errata, `Control.prev_buttons` and `Control.action_seq` are serialized in doc 02 section 10 (`inputs`) rather than section 2; `sys.input_apply` is their only writer.

`Kind.flags` bits: 0 `DEAD`, 1 `ASLEEP`, 2 `GHOST`, 3 `NO_COLLIDE_BODIES`, 4 `TELEPORTED` (cleared at `sys.tick_begin`; read by doc 02 interpolation).

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
pub struct Projectile {
    pub owner: EntityId, pub instigator_slot: u8, pub item_uid: u64, pub payload: Id32,
    pub radius: Fx, pub gravity_bp: Bp, pub drag_bp: Bp, pub bounce_bp: Bp, pub pierce_left: u8,
    pub hits: Vec<EntityId>, pub end_tick: Tick, pub view_tick: Tick, pub age: u32, pub block_bp: Bp,
}
pub struct FallingBlock { pub state: BlockState, pub start_y: Fx, pub hit: Vec<EntityId>, pub instigator: Option<EntityId> }
pub struct Mining { pub cell: Gidx, pub state: BlockState, pub progress: u32, pub needed: u32, pub last_tick: Tick }
pub enum ZoneShape { Sphere { r: Fx }, Cylinder { r: Fx, h: Fx }, Box { half: Fx3 } }
```

Default `BodyState` for `body.humanoid` Errata: Canon §4.1 values, `kb_resist_bp` 0, `hitbox_scale_bp` 10000, flags `PUSHABLE | COLLIDES_BODIES | CAN_MANTLE | CAN_CLIMB | EDGE_GUARD`. Creatures take their values from their template (doc 13). Item drops: radius 150 mm, height 300 mm, no flags. Falling blocks: radius 490 mm, height 980 mm, no flags.

### 3.5 Mass

`mass_g(e) = base_mass_g + carried_g`. Doc 11 maintains `carried_g`. Knockback (§5.13) and push shares (§5.14) read it.

### 3.6 Spatial broadphase

- The broadphase is a sorted `Vec<(u32, EntityId)>` rebuilt by `sys.broadphase` every tick. Keys are 4 m cells: `key = (bx >> 2) + 64 * ((bz >> 2) + 64 * (by >> 2))` for block coordinates clamped into the Folio box.
- Each live entity with `body`, plus every container, gadget and zone, is inserted once per 4 m cell its AABB overlaps. A body's AABB is `[p.x − r, p.x + r] × [p.y, p.y + h] × [p.z − r, p.z + r]`, with `h` the current height. Projectiles are not inserted; they query.
- The vector is sorted by `(key, EntityId)` with a stable sort. A query takes an AABB, visits its 4 m cells in ascending key order, collects IDs, sorts them ascending and removes duplicates.
- Ghosts (`Kind.flags & GHOST`) are inserted only into a separate ghost list, so no hit test, push or AI query ever returns them.

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
    pub liquids: BTreeMap<Local, LiquidCell>,    // §6.1
    pub gases: BTreeMap<Local, GasCell>,         // §7.1
    pub thermal: Option<Box<[DeciC; 4096]>>,     // §9.1; None = every thermal cell at ambient
    pub block_gen: u32,                          // bumps on every block edit; derived, not hashed
    pub field_gen: u32,                          // bumps on light change; derived, not hashed
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
| `material` | `mat.*` | required | one material table (Canon §13.8a); supplies defaults marked "mat" |
| `shape` | `grid.shape.*` | `grid.shape.full` | §4.4 |
| `collides` | flag | 1 (0 for none, ladder, cross, fixture) | collision boxes exist |
| `hardness_milli` | i32 | mat | mining and damage hardness; −1 = unbreakable |
| `tool_tier` | `tooltier.*` | `tooltier.t0` | minimum tool tier to break (doc 07) |
| `blast_res_milli` | i32 | mat | explosion resistance (§12.2) |
| `light_filter` | 0..=15 | 15 for full and pillar, else 0 | extra light loss entering this cell; 15 = opaque |
| `light_emit` | 0..=15 | 0 | constant `light_block` emission |
| `flammability_bp` | Bp | mat | spread chance factor (§8.3) |
| `burn_runs` | u16 | mat | fire runs a burning block lasts (§8.4) |
| `burn_result` | `block.*` | `block.air` | block left after burnout |
| `ignite_decic` | DeciC | 0 | heat ignition threshold; 0 = never ignites from heat |
| `liquid_interaction` | `grid.liqi.*` | `grid.liqi.blocks` | §6.2 |
| `absorb_per_run` | u16 units | 0 | for `grid.liqi.absorbs` |
| `heat_cond_bp` | Bp | mat | thermal conductance (§9.3) |
| `heat_emit_decic`, `heat_drive_bp` | DeciC, Bp | absent | constant heat source (§9.2) |
| `melt_decic`, `melt_to`, `melt_units` | DeciC, `liq.*`, u16 | absent | heat melting into liquid (§9.5) |
| `sound_atten_ddb` | i32 tenths of a dB | mat; 400 for full solids | added when sound enters this cell (§11.2) |
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
| `skins` | map genre suffix → `art.*` | material default | presentation only; the sim never reads it |

Illustrative records (doc 07 owns final IDs and values):

```json
[
  { "id": "block.air", "material": "mat.air", "shape": "grid.shape.none", "collides": 0,
    "hardness_milli": -1, "blast_res_milli": 0, "light_filter": 0, "sound_atten_ddb": 0,
    "heat_cond_bp": 10000, "conceal": 0 },
  { "id": "block.marble_masonry", "material": "mat.marble", "shape": "grid.shape.full",
    "hardness_milli": 1500, "tool_tier": "tooltier.t1", "blast_res_milli": 6000, "light_filter": 15,
    "flammability_bp": 0, "heat_cond_bp": 2000, "sound_atten_ddb": 400, "sturdy_faces": 63,
    "skins": { "mythic_antiquity": "art.block.marble_masonry.antiquity" } },
  { "id": "block.oak_door", "material": "mat.oak", "shape": "grid.shape.door",
    "hardness_milli": 3000, "blast_res_milli": 3000, "light_filter": 15, "flammability_bp": 2500,
    "burn_runs": 50, "ignite_decic": 3000, "liquid_interaction": "grid.liqi.permeable",
    "sound_atten_ddb": 150, "attach": 1, "heat_cond_bp": 1500 },
  { "id": "block.dune_sand", "material": "mat.sand", "shape": "grid.shape.full",
    "hardness_milli": 500, "blast_res_milli": 500, "gravity": 1, "liquid_interaction": "grid.liqi.absorbs",
    "absorb_per_run": 2, "sound_atten_ddb": 300 },
  { "id": "block.powder_keg", "material": "mat.oak", "shape": "grid.shape.full", "hardness_milli": 1000,
    "flammability_bp": 6000, "burn_runs": 5,
    "explosive": { "radius_mm": 4000, "block_power_milli": 4000, "env_base_mhp": 30000,
                   "shares": { "dmg.blunt": 5000, "dmg.fire": 5000 }, "knockback_mm_s": 10000,
                   "incendiary": 1, "drop_bp": 3000, "smoke_conc": 3000, "heat_decic": 2000,
                   "noise": "noise.explosion" } }
]
```

At content load the registry is compiled into per-state tables indexed by `BlockState`: `boxes[s]` (rotated `Fx` boxes), `full[s]` (one full box), `light_filter[s]`, `gas_open[s]`, `liquid_cap[s]` (§6.2), `sound_atten[s]`, `climbable[s]`, `sturdy[s]`, `conceal[s]`. These tables are derived from content and never serialized.

### 4.6 Genre skins

The simulation stores no skin. A block renders with the skin of `local_genre` at its cell, through doc 02 §8.3's per-genre texture arrays and §8.9's bleed blend. A block carried from Cyberpunk and placed in a Fairy Tale Folio therefore renders in the Fairy Tale skin. Skins change no block behaviour.

### 4.7 Protection and open-area masks

- `protect` bits come from worldgen (doc 12): exit portals, Folio-boundary shells, boss-arena seals while sealed, and scripted props. `set_block` refuses actor-caused edits of a protected cell with `protected`; worldgen and script causes bypass it. Explosions skip protected cells.
- `open_tag` bits come from worldgen and mark cells in rooms doc 12 tags as open areas (courtyards, caverns lit by the genre's ambient). They feed the sky channel (§10.2).

---

## 5. Movement and collision

`sys.movement` processes every live, awake entity that has `move`, in ascending `EntityId`. Mounts are processed in that order like any entity; riders skip their own integration and copy the seat position afterwards (§5.12).

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

Evaluation order each tick, first match wins: downed, ridden, mantle in progress, dodge in progress, fly, swim, climb, dodge start, mantle start, jump, then ground or air from integration.

**Status gates** (Canon §11.1, §9.1): `st.staggered`, `st.stunned`, `st.frozen`, `st.knocked_down`, `st.sleeping`, `st.rooted`, `st.grappled` zero the wish velocity. `st.overburdened` and a severed leg forbid sprint, jump and dodge. `st.exhausted` forbids sprint and dodge. `st.levitating` sets gravity to 0 and gives vertical control of 1500 mm/s through JUMP (up) and CROUCH (down).

### 5.2 Wish velocity

Input axes follow doc 02 §5.5: `fwd_in = −move_z`, `right_in = move_x`, each in `−127..=127`. In Q2.30, `right = (cos_bam(yaw), 0, −sin_bam(yaw))` and `fwdh = (−sin_bam(yaw), 0, −cos_bam(yaw))`.

```
m      = isqrt(right_in² + fwd_in²)                      // 0..=179
if m == 0: wish = 0
class  = backpedal if fwd_in × 2^30 < cos_bam(12288) × m      // angle > 67.5°
         strafe    if fwd_in × 2^30 < cos_bam(4096) × m       // angle > 22.5°
         forward   otherwise
base   = 6500 if SPRINT held ∧ class = forward ∧ ¬crouched ∧ can_sprint
         2000 if crouched,  else 4300
dir_bp = 8000 (backpedal) | 9500 (strafe) | 10000 (forward); sprint uses 10000
spd    = mul_bp(base, dir_bp) → × 5000 if WALK held → × §6.4 row 11 → × 5000 for a severed leg
         (7000 with a crutch) → × each active status multiplier in ascending status-ID byte order
den    = max(m, 127)
wish   = (right_in·right + fwd_in·fwdh) × fx_mm(spd) / (den × 2^30)        // per axis, floor
```

`cos_bam(12288) = 410903207` and `cos_bam(4096) = 992008094`. Example: `move_x = 90`, `move_z = −90` gives `m = 127`; `90 × 2^30 = 96,636,764,160` is below `992008094 × 127 = 125,985,027,938` and above `410903207 × 127 = 52,184,707,289`, so the class is strafe. At AGI 60 (row 11 = 10500): `mul_bp(mul_bp(4300, 9500), 10500) = 4289` mm/s. Forward at AGI 60 gives 4515 mm/s, and with `st.slowed` 3160 mm/s.

### 5.3 Acceleration, gravity and integration

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

### 5.5 Ground detection and snap

- **Ground probe.** After movement, the body is grounded when the capsule shifted 2 mm along gravity (`GROUND_PROBE`) penetrates a box whose push normal has `n.y × (−gsign) >= GROUND_NY` (raw 3037000499, 45°). `ground_ny` is the largest such `n.y`, and `ground_state` is the state of that box's cell, ties to the first in gather order.
- **Snap.** If the body was in `ground` at the start of the tick, is not grounded now, `v.y` is zero or points along gravity, and no jump fired this tick, it sweeps 550 mm along gravity with `move_and_collide`. If that ends grounded, the snap is kept (walking down stairs and 0.5 m ledges stays on the ground). Otherwise the position reverts and the body enters `air`.
- `coyote` is set to 6 when a body leaves `ground` without jumping and decrements each tick in `air`.

### 5.6 Step-up

When a grounded body's horizontal movement records a contact with `|n.y| < GROUND_NY` (a wall), with `up_sign = −gsign`:

```
save = (p, v, contacts of the plain move)
up   = move_and_collide(e, (0, STEP_UP × up_sign, 0))           // rise up to 550 mm; ceiling may cut it
move_and_collide(e, (dx_rem, 0, dz_rem))                         // the horizontal delta the wall clipped
move_and_collide(e, (0, −(rise + GROUND_PROBE) × up_sign, 0))   // settle
accept if grounded ∧ horizontal progress² > progress² of the plain move; else restore save
```

A full 1 m block needs a jump. A 500 mm slab or stair step is climbed without one.

### 5.7 Jump and air jump

- A JUMP press (held now, not held last tick) sets `jump_buf = 6`; it decrements each tick.
- A jump fires when `jump_buf > 0`, the body is grounded or `coyote > 0`, and jumping is allowed (§5.1). It sets `v.y = −gsign × v0` with Canon §4.2 `v0 = sqrt_fx(2 × g × JUMP_H) + g × DT / 2`, clears `jump_buf` and `coyote`, and enters `air`.
- **Air jump.** At `vrv(e) >= 9`, an airborne body with `air_jumps == 0` that presses JUMP sets `v.y` to the launch speed for `mul_bp(JUMP_H, 6000)` and sets `air_jumps = 1`. Landing, swimming or climbing resets it.

| Case | `g` (mm/s²) | `JUMP_H` (mm) | `v0` raw | Simulated apex (mm) | Apex tick |
|---|---|---|---|---|---|
| VERVE 0 | 20000 | 1100 | 29205417893 | 1099.972 | 21 |
| VERVE 5 | 20000 | 1250 | 31085832879 | 1249.874 | 22 |
| VERVE 10 | 20000 | 2000 | 39131183219 | 1999.922 | 28 |
| Wuxia, VERVE 10 | 16000 | 2000 | 34932400672 | 2000.000 | 31 |
| Wuxia air jump | 16000 | 1200 | 27187601199 | 1199.874 | — |

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
require: capsule (r, h_cur) at (p.x, land_y + 1 mm, p.z) has no penetration         // room to rise
require: capsule (r, stand_h) at (q.x, land_y, q.z) has no penetration, else end crouched
         if crouch_h fits; else no mantle
total  = 8 + ceil_div(h_mm, 100) ticks
motion: ticks 1..=ceil_div(total × 6, 10) raise y linearly to land_y + 1 mm; the remaining ticks
        move x, z linearly to q; positions use floor division of the remaining distance
abort:  if the capsule penetrates any box on a mantle tick, restore the last valid position, enter air
```

A 1000 mm ledge takes 18 ticks, 1600 mm takes 24, 560 mm takes 14. A 500 mm ledge is a step-up (§5.6), and 1700 mm fails the test.

### 5.10 Swim

- `immersion_mm` is measured in the body's column: walk up from the feet cell; each cell with a liquid total of at least 990 units adds 1000; the first cell with less adds its total and ends the walk; then subtract `to_mm(p.y) − 1000 × floor_block(p.y)`, the feet's height inside the feet cell. The result is clamped to `0..=height` in mm.
- In `swim`: gravity is off. `wish = fwd3(yaw, pitch) × spd` for forward input plus strafe on `right`, with `spd = mul_bp(2400, row 11)` then × the dominant liquid's `move_bp` (doc 08: water 10000, oil 8000, lava 3000). Vertical target is +2400 mm/s with JUMP, −2400 with CROUCH, else +400 while the eye is submerged and 0 at the surface. Acceleration is 12000 mm/s² on all three axes toward the target.
- Wading (`300 <= immersion_mm < enter threshold`) multiplies ground speed by the dominant liquid's `move_bp`.
- A swimming body that presses JUMP facing a ledge runs the mantle test (§5.9).

### 5.11 Climb

- Climbable overlap: any cell with `climbable[state]` intersecting the capsule AABB expanded by 100 mm horizontally, between `p.y` and `p.y + h/2`.
- In `climb`: gravity off. `v.y = mul_bp(2400, row 21)` × sign(`fwd_in`) (forward climbs up, back climbs down, no input holds); horizontal wish is capped at 1000 mm/s; CROUCH holds. JUMP leaves the ladder with `v.y = 4000` mm/s against gravity and `v_h = −fwdh × 2000` mm/s, and sets `climb_lock = 10` ticks. At GRP 60 the climb speed is 2640 mm/s.

### 5.12 Dodge, downed, flight, ghosts, gravity flip, mounts

- **Dodge** (Canon §4.2): direction = the horizontal unit of the wish velocity, or `−fwdh` with no input. For 12 ticks the horizontal velocity is set to 15000 mm/s along it; gravity still applies. Per-tick displacement is raw 1073741820, so the dodge covers 2.99999999 m. Doc 04 spends the stamina; doc 05 owns invulnerability.
- **Downed:** height 600 mm, crawl 800 mm/s × status multipliers, no jump, sprint or dodge.
- **Fly:** gravity off; `wish = fwd3(yaw, pitch) × spd` plus strafe; acceleration 20000 mm/s² on three axes. AI fliers write yaw and pitch into `control`.
- **Ghosts** (Canon §17.6) fly at walk speed with voxel collision and `NO_COLLIDE_BODIES`; they never appear in body, hit or perception queries (§3.6).
- **Gravity flip:** inside a doc 12 flip volume `gsign = +1`. Every "up" in §5.3–§5.7 reverses, and the feet point becomes `p + (0, h, 0)`. Mantle, swim and climb are disabled under flipped gravity.
- **Mounts** (Canon §17.7): the rider's `control` drives the mount. Runner mounts use `ground`/`air` with walk 6000 mm/s and gallop 12000 mm/s (SPRINT; doc 05 spends mount stamina). Hover mounts keep their origin 500 mm above the higher of the ground top and the liquid surface below them, at up to 14000 mm/s, rising or falling at most 4000 mm/s toward that height. Glider mounts move 9000 mm/s horizontally and, while airborne, fall at exactly 2000 mm/s. After the mount moves, each rider's `p` is set to the mount's `p` plus the template seat offset rotated by the mount's yaw, and the rider's `vel` copies the mount's. Summoning checks a 2600 mm tall capsule of the mount's radius for penetration.

### 5.13 Knockback

```
apply_knockback(e, dir: unit Fx3, speed_mm_s):
    scale_bp = clamp(floor_div(75000 × 10000, mass_g(e)), 2000, 20000)
    eff      = mul_bp(mul_bp(speed_mm_s, scale_bp), 10000 − kb_resist_bp)
    v       += dir × fx_mm(eff)
    if eff >= 3000: kb_ticks = max(kb_ticks, 12); leave ground (mode air)
```

`kb_ticks` decrements each tick; while positive, control uses air acceleration. Example: 8000 mm/s on a 120 kg Erratum gives `scale_bp = 6250` and 5000 mm/s.

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

Two 75 kg bodies 500 mm apart (radii 300 mm) overlap by 100 mm and each moves 50 mm. A 300 kg ogre against a 75 kg Erratum moves 20 mm while the Erratum moves 50 mm (its share, 80 mm, is capped at 50).

### 5.15 Falls, landing and footsteps

- `peak_y` resets to `p.y` every tick the body is grounded, swimming, climbing, mantling or has `st.levitating`; otherwise it keeps the extreme against gravity (Canon §4.2).
- On the transition into `ground`, `fall_mm = to_mm(peak_y − p.y)` along gravity. If the canon fall formula gives damage, the hit is queued for `sys.damage` with `B = mul_bp(canon_damage, genre.fall_dmg_bp)` and flag `fall`; doc 04 then applies Acrobatics. When `fall_mm >= 300`, `ev.entity_landed` is emitted. Example: a 6 m fall at VERVE 5 with Max HP 85 gives `excess` raw 8589934592, 1600 Bp, and 13,600 mHP before `fall_dmg_bp`.
- **Footsteps.** While in `ground`, `stride_mm` accumulates horizontal distance in mm. When it reaches the stride length (crouch 500, walk 700, sprint 900), it drops by that length and `ev.footstep` is emitted with the gait and the ground material. Doc 09 converts footsteps and landings into `ev.noise`.

### 5.16 Hitbox capsules from pose tables

Pose tables are content (Canon §9). Each record gives, for one animation and body plan, keyframes every `key_every` ticks of every part capsule in model space (mm; origin at the entity origin; model −Z is forward). The loader expands keyframes into the canon per-tick table: for `k·K <= t < (k+1)·K`, each coordinate is `a + floor_div((b − a) × (t − k·K), K)`, and looping animations interpolate from the last key to the first.

```json
{ "id": "art.anim.humanoid.walk", "body": "body.humanoid", "length_ticks": 40, "loop": 1, "key_every": 10,
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
- It then writes `PoseRec { tick: t, p, yaw, anim, anim_tick, lean }` at `hist[head]` and advances `head` modulo 32. A rewind to `view_tick` reconstructs capsules from the record with `tick == view_tick`, or from the oldest record if `view_tick` is older than 32 ticks.

### 5.17 Geometric primitives

This chapter uses doc 02 §4.6 exactly: `closest_seg_seg`, `capsule_overlap`, `ray_capsule` (with `r` inflated by the projectile radius for thick projectiles) and `dda`. A hitscan or projectile segment hits the hitbox part whose `ray_capsule` returns the smallest `t`; ties go to the part listed first in the body plan (Canon §9).

### 5.18 Voxel raycast

`raycast_blocks(o, d, t_max)` wraps doc 02 `dda`:

1. Components of `d` with `|raw| < 65536` are set to 0 before the call, so `div_fx(ONE, d_i)` never exceeds raw `2^48`.
2. For each visited cell: a `full[state]` cell is a hit at the DDA `t` with the entered face. A cell with partial boxes runs the slab test of the ray against each box (entry `t = max` of the per-axis entry times, exit `t = min` of the exits, axis ties x, y, z); the smallest entry `t >= t_cell` is a hit with that box's entered face. A cell with no boxes continues.
3. The start cell is tested first; a ray starting inside a box hits at `t = 0` with face `None`.

Worked example: origin (5.5, 2.62, 12.5) m, `yaw = 4096`, `pitch = −2048`, so `d = fwd = ` raw (−1612031272, −837906552, −3891787748). Initial `t_next` raws are 5721583816 (x), 13649471168 (y), 2369957622 (z); `t_delta` raws 11443167632, 22015276082, 4739915244. Stone at (4, 1, 8), floor at y ≤ 0. Visits: (5,2,11) at 0.551799 m through +Z; (4,2,11) at 1.332160 through +X; (4,2,10) at 1.655396; (4,2,9) at 2.758994; (4,1,9) at 3.178015 through +Y; (4,1,8) at raw 16589703354 (3.862591 m) through +Z: a hit at (4.0503, 1.8664, 9.0).

### 5.19 Projectile sweep

`sys.projectiles` processes projectiles in ascending `EntityId`:

```
if t >= end_tick: despawn(expired); continue
v.y += −mul_fx(fx_mm(mul_bp(mul_bp(20000, genre.gravity_bp), gravity_bp)), DT)
v    = v − mul_bp_fx(v, drag_bp)                           // per tick
seg  = p → p + v × DT
pose_tick = view_tick if age == 0 ∧ owner is an Erratum else t     // lag compensation on the first tick only
world_hit = radius = 0 ? raycast_blocks(seg) : sphere_substeps(seg, radius, step = max(radius, 100 mm))
body_hits = every non-ghost hitbox part in the broadphase along seg, excluding owner and `hits`,
            tested with ray_capsule(r_part + radius) at pose_tick
take the smallest t; a body hit wins ties against a world hit
body hit:  queue the hit (doc 05); if pierce_left > 0 { pierce_left -= 1; push to hits; continue the
           remaining segment } else { despawn(consumed) }
world hit: bounce_bp > 0: reflect the velocity component on the hit axis × bounce_bp, other axes × 8000,
           place the projectile at the hit point offset by radius + 1 mm along the face normal;
           else impact (doc 05: explode, embed, splash), despawn(consumed)
age += 1
```

`sphere_substeps` tests the sphere at each substep end against gathered boxes with doc 02's box distance; the first overlapping substep is the hit, located at the previous substep position. Block damage from impacts uses `block_bp` (§13.2). `PROJECTILE_CAP` eviction follows Canon §4 at spawn time.

### 5.20 Environment sampling

`sys.env_sample` fills `comp.env` for every awake body:

- `genre = local_genre(p)` (Canon §5.8). A change triggers doc 05's Contradiction re-evaluation and `ev.contradiction_changed`.
- Contact cells: feet = cell of `p + (0, 50 mm, 0)`, body = cell of `p + (0, h/2, 0)`, eye = cell of the eye point. Each records up to 3 liquids with units, up to 2 gases with concentration, and whether the liquid is charged (§6.9).
- **Liquid contact** (Canon §12.1, "at least 100 ml"): any liquid with at least 1 unit (1 unit = 1000 ml, §6.1) in the feet or body cell.
- **Gas contact:** a gas with concentration at least 500 in the eye cell.
- `eye_submerged`: the eye cell's liquid surface `cell_y + total/1000 m` is above the eye point. Doc 04 drains `res.breath` while `eye_submerged` or while the eye-cell gas is unbreathable (doc 08 flag).
- `temp` is the thermal cell of the body point (§9). `light_sky`, `light_block` are those of the cell at `p + (0, 0.5 m, 0)` (Canon §2.3 `light_at`). `in_fire` is true when any burning cell intersects the capsule AABB expanded by 100 mm.

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
| `grid.liqi.displaced` | 1000; the first transfer into the cell breaks the block with cause `grid.cause.liquid` and its normal drops |
| `grid.liqi.absorbs` | 0; the block absorbs film from the cell above (§6.4 step 8) |

### 6.3 Activation, order and budget

- `liquid_active: BTreeSet<Gidx>` holds cells that changed or neighbor a change. At Folio entry it holds every liquid cell. A cell is processed only when its chunk is active (§15).
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
          set_block(c, freeze_block(k), grid.cause.froze); delete all liquid in c; return
      if T >= boil_decic(k): b = min(units_k, 50); units_k −= b; emit_gas(c, boil_gas(k), 20 × b)
 4  DOWN: below = c − Y; free = liquid_cap(below) − V(below)
      fl  = volume-weighted flow_bp of c = floor_div(Σ units_k × flow_bp_k, V)
      amt = min(V, free, mul_bp(FULL, fl))
      move amt into below, taking components heaviest first (density desc, id32 asc)
 5  SWAP: if V(below) = cap(below) ∧ c's heaviest density > below's lightest density:
      s = min(units of c's heaviest, units of below's lightest, mul_bp(200, min(flow_bp of both)))
      exchange s units of the two components
 6  SPREAD (only if V > FILM_MAX and below is full or blocked): V0 = V
      for n in (N4A if r even else N4B), skipping cells with cap 0:
          diff = V0 − V(n); if diff < 2: skip
          tr = floor_div(diff × fl, 50000)                       // 1/5 of the difference × flow
          if tr = 0 ∧ fl >= 5000: tr = 1                         // EQUALIZE: level the last units
          tr = min(tr, cap(n) − V(n)); move tr into n, proportional (below)
 7  EVAPORATION (V <= FILM_MAX): for each component with evap_period p > 0:
      if (r + c) % p == 0: units −= 1
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
| `flow_bp` | Bp | steps 4–6 | water 10000, oil 7000, blood 8000, honey 2500, lava 1500 |
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

### 6.6 Worked examples

A 1 × 5 channel at y 1 over solid floor, walls on both sides; water (flow 10000) 1000 units at x 2; all other cells empty.

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

`sys.damage` and `sys.damage_late` call `surface_trigger(cells, dmg_type, instigator_slot)` for every hit whose type shares include a trigger type. Trigger cells are: for a point hit, the cell of the impact point and the cell below it; for area damage, the area's center cell and its `N6` neighbors. Each trigger emits one `ev.liquid_reaction`.

| Trigger | Grid effect |
|---|---|
| shock on a conductive liquid | BFS over `N6` from each trigger cell through cells holding that liquid (≥ 1 unit), ascending visit order, at most 64 cells: `charged_until = t + 180`, `charged_slot = instigator_slot` |
| frost on water, blood or alcohol | each trigger cell that is air and holds ≥ 100 units of it becomes `block.ice` with cause `grid.cause.froze`; the cell's liquid is stored in `timed_revert[c] = (t + 1200, liq, units)` and deleted; other components are deleted |
| fire on water | each trigger cell converts `k = min(units, 27)` water units into `k` full steam cells: `emit_gas_volume(c, gas.steam, k)` |
| fire on oil or alcohol | each trigger cell ignites as a liquid fire (§8.2); one blast per hit at the impact cell: radius 2000 mm, environmental `dmg.fire`, base `10,000 mHP × min(units, 5)`; `min(units, 5)` units are consumed |
| fire on ice | `block.ice` in a trigger cell reverts now: a `timed_revert` entry restores its liquid; otherwise 1000 units of `liq.water` |
| fire on web | `block.web` ignites as a block fire (its registry `burn_runs` is 5, which is 60 ticks) |
| fire on miasma | if a trigger cell holds `gas.miasma` ≥ 1000: one blast per hit, radius 3000 mm, environmental `dmg.fire` base 20,000 mHP; the miasma in the trigger cells is removed |
| radiant on red ink | every `liq.red_ink` unit in the trigger cells becomes `liq.water` |

`timed_revert: BTreeMap<Gidx, (Tick, Id32, u16)>` is checked in `sys.block_updates`: at its tick the ice becomes air and the liquid is deposited back. Temperature never melts a cell that has a `timed_revert` entry. Blasts are explosions without block damage (`block_power_milli = 0`, §12.1) and enter the pipeline as environmental hazards with `instigator_slot`.

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

---

## 7. Gases

### 7.1 Cell record

```rust
pub struct GasCell { pub comps: [(u16 /* gas type index */, u16 /* conc 0..=10000 */); 2], pub n: u8,
                     pub owner_slot: u8, pub last_run: u32 }
```

Concentration 10000 is a cell saturated with that gas. Gases move only through cells with `gas_open[state]` (§4.5). A third gas cannot enter a cell holding two; that share stays behind. At most 65,536 gas cells exist per Folio; flow or emission that would create a cell beyond the cap is refused.

### 7.2 Gas properties consumed from doc 08

| Field | Meaning | Reference values (doc 08 owns) |
|---|---|---|
| `diffuse_bp` | share of the difference sent per face per run, over 7 | smoke 6000, steam 8000, miasma 5000 |
| `rise_bp` | share moved up (positive) or down (negative) per run | smoke 2000, steam 3000, miasma −1000 |
| `dissip_bp` | decay per run | smoke 150, steam 400, miasma 100 |
| `conceal_full` | `conceal` at concentration 10000 (Canon §12.2) | smoke 4, steam 3 |
| `unbreathable` | flag for breath (Canon §12) | miasma 1 |
| `condense_decic`, `condense_to` | below this temperature the gas becomes liquid at 1 unit per 20 conc | steam 900 → `liq.water` |

### 7.3 Gas run

Runs on `(t + 4) % 6 == 0`, with run index `floor_div(t + 4, 6)`. Active cells (any gas) in active chunks are taken in ascending `Gidx` on even runs and descending on odd runs, with the same 16,384-cell budget and `last_run` fairness as liquids (§6.3).

```
for each component k of cell c (component order):
  1 vertical: m = floor_div(conc_k × |rise_bp|, 10000) toward +Y if rise_bp > 0 else −Y,
              only if that neighbor is gas-open and holds less of k; capped at its free room to 10000
  2 diffusion: c0 = conc_k after step 1
              for n in N6 (gas-open, in bounds): diff = c0 − conc_k(n); if diff > 0:
                  f = floor_div(diff × diffuse_bp, 70000); move f to n
  3 dissipation: conc_k −= max(1, floor_div(conc_k × dissip_bp × open_mult, 10000)),
              open_mult = 4 when the cell's open_tag or sky_src bit is set, else 1
  4 condensation: if thermal T < condense_decic: deposit(c, condense_to, floor_div(conc_k, 20)); conc_k = 0
wake: changed cells and their gas-open N6 neighbors join the next active set
```

Transfers only flow from higher to lower concentration, so each face moves gas one way and the total changes only through dissipation and condensation.

**Example.** Smoke 7000 in a cell with open air on five sides and a floor below: rise moves `floor_div(7000 × 2000, 10000) = 1400` up, leaving 5600. Diffusion sends `floor_div(5600 × 6000, 70000) = 480` to each of ±X and ±Z, and `floor_div(4200 × 6000, 70000) = 360` up (the cell above now holds 1400), leaving 3320. Dissipation removes 49, leaving 3271.

### 7.4 Emission

- `emit_gas(c, gas, conc, slot)` adds up to `10000 − conc` of the cell; the excess is dropped.
- `emit_gas_volume(c, gas, k, slot)` fills `k` cells to 10000 by `N6` breadth-first search over gas-open cells from `c`, at most 27 cells.

### 7.5 Gas ignition

A gas whose doc 08 record has a blast (Canon §12.1: `gas.miasma`) ignites when fire damage triggers it (§6.9) or when a burning cell is `N6`-adjacent to a cell holding it at ≥ 1000 during a fire run (§8.3). At most 4 gas blasts resolve per fire run, in ascending cell order; the rest wait for the next run. Each blast is an explosion without block damage (§12.1), consumes the gas in its cell, and leaves a flash fire (fuel 1) there.

### 7.6 Concealment helper

`cell_conceal(c) = conceal[state] + (liquid V >= 500 ? Σ conceal_per_cell of components with ≥ 500 units, capped at 15 : 0) + Σ_gases floor_div(conceal_full × conc, 10000)`, capped at 15. Doc 09 sums it along sight rays from `raycast_blocks`, excluding both endpoint cells (Canon §12.2).

---

## 8. Fire

### 8.1 Fire cell

```rust
pub struct FireCell { pub kind: FireKind /* Block | Liquid | Flash */, pub fuel: u16 /* fire runs left */,
                      pub age: u16, pub intensity: u8 /* 1..=3 */, pub instigator: Option<EntityId> }
```

`fires: BTreeMap<Gidx, FireCell>`, at most 4,096 entries; ignition at the cap is refused. A block fire lives in the burning block's own cell, a liquid fire in the cell holding the fuel liquid.

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

Per fire run after spread, for each burning cell:

- `age += 1`; `intensity = min(3, 1 + floor_div(age, 10))`.
- Block fire: `fuel −= 1`; at 0 the block becomes `burn_result` (cause `grid.cause.fire`), an `explosive` block detonates (§12), and the fire ends with `ev.fire_out`.
- Liquid fire: consumes 5 units of the cell's `fire_fuel` liquids (proportionally); ends when none remain.
- Flash: ends after one run.
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
- Values are `DeciC`, clamped to `−2730..=15000`. `ambient(cell)` is the doc 12 Genre Parameter Vector field `ambient_decic` of the local genre at the thermal cell's lowest-corner block (default 200, 20.0 °C).
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

Example: one lava block (11000, drive 2000) in a 200 thermal cell adds `floor_div(10800 × 2000, 80000) = 270` per run.

### 9.3 Diffusion and relaxation

Runs on `(t + 5) % 30 == 0` over awake thermal chunks, Jacobi style from a snapshot:

```
cond(cell) = floor_div(Σ heat_cond_bp of its 8 blocks, 8)     // cached per thermal cell, rebuilt on edits
for each awake cell a, for each of +X, +Y, +Z with neighbor b (in bounds):
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

In each temperature run, for awake thermal cells with `|T − ambient| >= 100`, the 8 blocks are checked in ascending `Gidx`: a flammable block with `0 < ignite_decic <= T` ignites; a block with `melt_decic` and `T >= melt_decic` (and no `timed_revert` entry) becomes air and deposits `melt_units` of `melt_to` (cause `grid.cause.melted`). Liquid freezing and boiling happen in the liquid step (§6.4 step 3).

---

## 10. Light

### 10.1 Channels

Each cell stores `light_block` (low nibble) and `light_sky` (high nibble), each `0..=15` (Canon §1.2). Effective light is `max(light_sky, light_block)`. Stealth, composure and `light_at` use effective light; spawning uses `light_block` only (Canon §12.3). The renderer uploads both channels into doc 02 §8.5's field texture (R = `light_sky × 17`, G = `light_block × 17`), so visual light and simulated light agree by construction.

### 10.2 Sources

- **Sky channel.** A cell is a sky source when `light_filter[state] < 15` and either its `open_tag` bit is set or no cell above it in its column up to y 63 has `light_filter == 15`. Its source level is `ambient_light` of `local_genre(cell)` (Canon §5.2). `sky_src` bits are recomputed for a column whenever a block in it changes opacity.
- **Block channel.** `emission(c) = max(light_emit[state], dynamic emitters at c (§10.4), 12 if c is burning, light_emit of a liquid with ≥ 100 units in c)`.
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
- A level-10 emitter moving one cell costs at most 1,159 dark pops and 1,159 lit pops in open space, so four torch-bearers sprinting stay under 1,000 pops per tick.
- **Flashes.** Light shorter than 30 ticks (muzzle flash, explosion) never enters the grid. It is appended to the per-tick list `flashes: Vec<(Gidx, u8 level, EntityId)>`, which doc 09 reads in the same tick and which is cleared at `sys.tick_begin`.

### 10.5 Folio bake

At Folio entry every sky source and emitter is seeded and both queues run to empty with no budget, before `folio_entry_tick`. The resulting levels equal what an unbudgeted propagation from scratch produces, because increases alone run in that bake.

---

## 11. Sound

### 11.1 Emission

`emit_noise(emitter, noise: Id32, pos: Fx3, level_db: Db, slot: u8)` appends a request to the tick's noise list. Doc 09 owns noise IDs and levels and converts `ev.footstep`, `ev.entity_landed`, weapon, spell and block events into requests. `sys.sound` processes the list in request order and emits one `ev.noise` per request.

### 11.2 Costs

Costs are in tenths of a decibel (ddB). `L0 = 10 × level_db`.

| Step into cell `n` | Cost |
|---|---|
| face neighbor (`N18` first six) | 20 |
| edge neighbor (`N18` last twelve), allowed only when both face cells it cuts past have no `full` box | 28 |
| plus the entered cell's `sound_atten[state]` | e.g. air 0, closed oak door 150, full stone 400 |
| plus 10 when `n` holds ≥ 500 liquid units | |

The level heard at a cell is `floor_div(L0 − cost, 10)` dB.

### 11.3 Propagation

```
candidates = awake bodies with hearing (creatures with perception, every Erratum) whose eye point lies
             within floor_div(L0 × 1000, 19) mm of pos, ascending EntityId     // 19 ddB/m bounds every path
if none: emit ev.noise with no heard entries; done
Dijkstra from cell(pos): bucket queue indexed by cost 0..=L0, FIFO inside a bucket, N18 visit order;
   a cell is settled when popped; stop when every candidate's eye cell is settled, the queue is empty,
   or the next bucket exceeds L0
for each candidate: heard = floor_div(L0 − cost, 10); walk parent pointers back from its eye cell up to
   4 hops; arrival_dir = unit vector from the eye cell center to that cell's center (0 at the source)
```

### 11.4 Budget and fallback

All requests of a tick share 32,768 settled cells. When the budget runs out, every unsettled candidate of the current and later requests gets the ray estimate: `cost = floor_div(20 × d_mm, 1000) + Σ sound_atten` of the cells crossed by `raycast_blocks` between the two cells (endpoints excluded), `arrival_dir` = direction to the source, `exact = false`.

### 11.5 Outputs

- `HeardNoise { listener, emitter, noise, level_db, arrival_dir: Fx3, exact: bool }` per candidate with `level_db > 0`, appended to the tick's hearing list for `sys.hearing` (doc 09).
- `ev.noise` carries the source level and, per party slot, the heard level and arrival direction, which the audio engine uses for gain, low-pass and apparent position (doc 02 §9).

**Examples.** A 130 dB shot (`L0` 1300) reaches a listener 10 m down a straight corridor at cost 200: 110 dB. With a closed oak door on the path the cost is 350: 95 dB. A 45 dB footstep reaches 25 dB at 10 m. Candidates for the shot lie within 68,421 mm.

---

## 12. Explosions

### 12.1 Explosion record

```rust
pub struct ExplosionSpec {          // content schema (JSON integers), used by items (doc 05) and blocks (§4.5)
    pub radius_mm: i32, pub block_power_milli: i32, pub env_base_mhp: i64, pub shares: Vec<(Id32, Bp)>,
    pub knockback_mm_s: i32, pub incendiary: bool, pub drop_bp: Bp, pub smoke_conc: u16,
    pub heat_decic: i32, pub noise: Id32,
}
pub struct Explosion { pub center: Fx3, pub spec: ExplosionSpec, pub source: Source /* Item { uid, b_mhp } | Env */,
                       pub instigator: Option<EntityId>, pub due_tick: Tick }
```

`queue_explosion` appends to `explosion_q` (saved). `sys.explosions` resolves at most 4 entries with `due_tick <= t` per tick, in queue order.

### 12.2 Block damage (ray method, `rng.physics`)

- Directions: for `i, j, k in 0..16` (x outer, z inner) with at least one index equal to 0 or 15, `d = normalize(2i − 15, 2j − 15, 2k − 15)` computed as `div_fx(from_int(a), sqrt_fx(from_int(a² + b² + c²)))` per axis. There are 1,352 directions; entry 0 is raw (−2479700525, −2479700525, −2479700525) and entry 1215 (`i = 15, j = k = 7`) is raw (4276004875, −285066992, −285066992).
- Per ray, in table order, one draw: `I = floor_div(block_power_milli × (7000 + rng.physics.range(6001)), 10000)`. Then, starting at the center and stepping `d × 300 mm`:

```
while I > 0:
    s = state at the sample's cell
    if s is not air: I −= floor_div((blast_res_milli(s) + 300) × 300, 1000); if I > 0: mark the cell
    I −= 225; advance one step
```

- Marked cells are processed in ascending `Gidx`: protected or unbreakable cells are skipped; each other cell draws one `rng.physics.range(10000)` and drops its block items (doc 07) when the draw is below `drop_bp`; the block becomes air with cause `grid.cause.explosion`. An `explosive` block queues its own explosion with `due_tick = t + 10`.
- Draw count per explosion: 1,352 + the number of breakable marked cells, plus one `rng.fire` draw per breakable marked cell for incendiary explosions (§12.3). Explosions with `block_power_milli = 0` (surface blasts) skip this subsection and draw nothing.

**Example.** Power 4000, draw 3000 (`I = 4000`), ray 1215 from (10.5, 2.5, 10.5) toward a stone wall (`blast_res_milli` 6000) at x = 13. Samples 0–8 are air and leave `I = 4000 − 9 × 225 = 1975`; sample 9 lies in (13, 2, 10): `1975 − 1890 = 85 > 0`, so the cell is marked; `85 − 225 < 0` ends the ray.

### 12.3 Incendiary

For an incendiary explosion, each breakable marked cell draws `rng.fire.range(10000)`; below 3333, the flammable block directly beneath that cell, if any, ignites with the explosion's instigator.

### 12.4 Damage and knockback to entities

For every live, non-ghost body, and every container and gadget with `pools`, whose center point `q = p + (0, h/2, 0)` lies within `radius_mm` of `center`, in ascending `EntityId`:

```
dist_mm   = to_mm(dist(center, q)); R = radius_mm; R4 = floor_div(R, 4)
falloff   = 10000 if dist_mm <= R4 else 10000 − floor_div(7500 × (dist_mm − R4), R − R4)
samples   = p + (0, 100 mm, 0), q, p + (0, h − 100 mm, 0), q ± r × (horizontal perpendicular to q − center)
exposure  = 2000 × (number of samples whose raycast_blocks from center reaches them without a hit)
B_eff     = mul_bp(mul_bp(B, falloff), exposure)          // B = b_mhp (item) or env_base_mhp (environment)
queue a hit: part = the body plan's largest-share part (ties: first listed), flags area (+ env for Env),
             instigator, shares = spec.shares
apply_knockback(e, normalize(q − center + (0, 500 mm, 0)), mul_bp(mul_bp(knockback_mm_s, falloff), exposure))
```

Item drops receive knockback only. Example: `R = 4000`, `B = 40,000` mHP, target at 2500 mm with 4 of 5 samples exposed: `falloff = 10000 − floor_div(7500 × 1500, 3000) = 6250`, `B_eff = mul_bp(25000, 8000) = 20,000` mHP; a 10000 mm/s knockback becomes 5000 mm/s before mass scaling.

### 12.5 Other effects

`emit_gas(cell(center), gas.smoke, smoke_conc)`; `add_heat(cell(center), heat_decic)`; a flash of level 15 for doc 09 (§10.4); `emit_noise` with `spec.noise` (level owned by doc 09); `ev.explosion`.

---

## 13. Block edits

### 13.1 `set_block`

`set_block(c, new, cause: Id32 /* grid.cause.* */, actor: Option<EntityId>) -> Result<BlockState, EditError>` is the only way to change a block after worldgen. It refuses out-of-bounds cells, protected cells for actor causes, and states with stray prop bits. On success, in this order: write the chunk and bump `block_gen`; delete liquid above the new `liquid_cap`; end a block fire if the new block cannot burn; remove the cell's damage entry; update `heat_sources`, the thermal conductance cache and the column's `sky_src` bits; call light `on_change` for both channels; push `c` and its `N6` neighbors onto the block-update queue; emit `ev.block_changed`.

### 13.2 Breaking

- **Mining** (tool use, doc 07): `sys.actions` sets `Mining { cell, state, needed = mining_ticks(state, tool, user) }` when a mining action targets a new cell, and adds 1 to `progress` on every tick the action continues. The block breaks when `progress >= needed`, with cause `grid.cause.mined` and doc 07 drops. Progress resets when the target cell or its state changes, or after 6 ticks without the action. `mining_ticks` returning `None` means the tool tier is too low.
- **Damage** (weapons, projectiles, sonic, siege): `block_damage: BTreeMap<Gidx, (i64 accumulated, Tick last)>`. A hit adds `mul_bp(hit B in mHP, block_bp)`, where `block_bp` is the weapon's or projectile's block factor (doc 05; pickaxe 20000, hammer 10000, sword 2000, bullet 1000) and 0 if the tool tier is below `tool_tier`. The block breaks (cause `grid.cause.broken`) when the sum reaches `block_hp = hardness_milli × 20`. Entries 600 ticks old are removed in `sys.block_updates`. Example: a 1500-hardness block has `block_hp` 30,000; a 12,000 mHP sword hit adds 2,400, so 13 hits break it; a pickaxe hit of the same size adds 24,000, so 2 hits do.

### 13.3 Placing

A placement from `sys.actions` succeeds only when all hold:

1. The target comes from `raycast_blocks` from the eye along the view, `t_max` 4500 mm. The target cell is the hit cell when its block is `replaceable`, else the hit cell plus the entered face normal.
2. The target is in bounds, not protected, and holds air, a `replaceable` block, or only liquid.
3. Attach blocks need a sturdy face on the support side; other blocks need any `N6` neighbor that is not air, except `gravity` blocks.
4. No collision box of the new state penetrates any non-ghost body capsule or item drop (§5.4 test with zero tolerance).
5. The placer is not `st.mounted` (Canon §17.7) and holds the block item (doc 11 consumes it).

Props from the placer: horizontal facing from `facing_from_yaw` (stairs face away from the placer; doors, ladders and fixtures face the placer); slab and stair half from the hit point (upper half of a side face or a bottom face gives top); doors place two cells (lower, upper) and both must pass; door hinge is left. Cause `grid.cause.placed`.

### 13.4 Doors and trapdoors

INTERACT on a door or trapdoor toggles bit b2 on both halves through `set_block` with cause `grid.cause.toggled`. Opening is refused when the opened boxes would penetrate a body.

### 13.5 Block-update queue

`block_updates: VecDeque<Gidx>`, saved, at most 2,048 pops per tick. For each popped cell in order:

1. **Attachment.** An `attach` block whose support face lost its sturdy neighbor breaks with cause `grid.cause.unsupported` and drops.
2. **Gravity.** A `gravity` block whose cell below has `liquid_cap > 0` or is air becomes a falling block (§13.6).
3. **Connections.** Fence and pane props are recomputed.
4. **Wake.** Liquid cells in the cell and its `N6` neighbors join `liquid_active`; gas cells join the gas active set; the chunk's thermal grid wakes if the conductance changed.
5. **Timed reverts and damage decay** due by `t` are applied (§6.9, §13.2).

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
| `ev.noise` | `emitter: Option<EntityId>, noise: Id32, pos: Fx3, level_db: Db, heard: [Option<(Db, Fx3)>; 4]` (per party slot: level and arrival direction) | 03 (`sys.sound`) | 09, audio |
| `ev.awareness_changed` | `observer: EntityId, subject: EntityId, from: Id32, to: Id32 /* aware.* */` | 09 | 13, 17 |
| `ev.item_picked_up` | `entity: EntityId, uid: u64, from: u8 /* 0 ground, 1 container, 2 shop */, from_entity: Option<EntityId>, stack: i32` | 11 | 10, 14 |
| `ev.item_dropped` | `entity: EntityId, uid: u64, drop_entity: EntityId, pos: Fx3, stack: i32` | 11 | 13 |
| `ev.item_used` | `user: EntityId, uid: u64, template: Id32, target: Option<EntityId>, cell: Option<Gidx>` | 07, 08, 11 | 04, 10 |
| `ev.item_identified` | `by: EntityId, uid: u64 /* 0 = whole type */, template: Id32, state: Id32 /* know.* */` | 10 | 14, 17 |
| `ev.item_crafted` | `crafter: EntityId, uid: u64, recipe: Id32, station: Id32, quality: Id32` | 07 | 04, 14 |
| `ev.potion_brewed` | `brewer: EntityId, uid: u64, recipe: Id32, apparatus: Id32` | 08 | 04, 14 |
| `ev.spell_cast` | `caster: EntityId, spell: Id32, target: Option<EntityId>, target_pos: Fx3, aether_mhp: i64, fizzled: bool` | 06 | 04, 09, 13 |
| `ev.block_changed` | `cell: Gidx, old: BlockState, new: BlockState, cause: Id32 /* grid.cause.* */, actor: Option<EntityId>` | 03 (`set_block`) | 07, 09, 12, 13, 14 |
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

---

## 15. Activation and interest

### 15.1 Active chunks

`sys.interest` rebuilds `active: [bool; 128]` each tick. Every Erratum (alive, downed or ghost) activates every chunk with `|cx − ecx| <= 2` and `|cz − ecz| <= 2`, for both chunk layers. Every entity with `interest` (boss arenas while engaged, scripted events, Colophon anchors) activates chunks within its own `radius_chunks`. One Erratum activates up to 50 of the Folio's 128 chunks.

Inactive chunks keep their liquid, gas, fire and thermal state frozen: their cells stay in the active sets but are skipped until the chunk activates. Light, block updates and sound run everywhere.

### 15.2 Sleeping entities

A creature, mount, gadget, item drop or corpse is asleep (`Kind.flags & ASLEEP`) when all hold: its chunk is inactive, it is grounded or has no `move`, it is not `in_combat` (Canon §2.3), and doc 09 reports no awareness above unaware toward any Erratum. Asleep entities skip perception, AI think, movement, environment sampling, pose, surfaces and temperature effects. They still take hits, and their status timers and DoT pulses run. They wake the tick any condition fails. Projectiles and zones never sleep.

### 15.3 Actor cap

Creatures and mounts count toward `ACTIVE_ACTOR_CAP` (256; Canon §4), minions included (Canon §15.1). Doc 13 calls `request_spawn(template, pos, yaw, owner)`; when the cap is reached the request waits in `spawn_q: VecDeque<SpawnReq>` (saved) and `sys.spawn_admit` admits it, oldest first, once the count drops.

---

## 16. Save, restore and state hash

### 16.1 Coverage

Doc 02 §6.5 fixes the section order. This chapter's state maps onto it as follows; everything listed is hashed.

| Section | This chapter's contents, in this order |
|---|---|
| 0 `meta` | boundary tick, `folio_entry_tick`, entity counter |
| 1 `rng` | `rng.fire`, `rng.blocktick`, `rng.physics` states |
| 2 `entities` | every column of §3.3 in catalog order, including `Pose.hist` (32 records), `MoveState`, `Env` accumulators, `Mining`, `LightEmitter.admitted`; then `spawn_q` |
| 4 `blocks` | 128 chunks in slot order: `palette_rle` of `BlockState`, then `sky_src`, `open_tag`, `protect` bitsets; then `block_updates` FIFO, `block_damage`, `timed_revert`, `explosion_q` |
| 5 `light` | per chunk `palette_rle` of the light bytes; then `Relight.dark` and `Relight.lit` in FIFO order |
| 6 `fluids` | per chunk ascending `(Local, LiquidCell)` and `(Local, GasCell)` lists; `liquid_active`, `gas_active` ascending; liquid and gas run counters |
| 7 `thermal` | per chunk: 0 for asleep, or 1 then `palette_rle` of the 4,096 `DeciC` values as `u16` offsets `T + 2730`; then `fires` ascending |

### 16.2 Excluded derived state

Never serialized, rebuilt in this order on restore: per-state block tables (§4.5); in-memory palettes from `palette_rle`; `heat_sources` and thermal conductance caches; the dynamic-emitter map (from `admitted`); the broadphase. Also never saved: `block_gen`, `field_gen`, meshes, navigation graphs (doc 13 rebuilds them from blocks), the noise list, `flashes`, the hearing list and the event list. Rebuilding is pure, so `restore` followed by `state_hash()` reproduces the saved hash (doc 02 SyncTest with depth 8).

### 16.3 Rules

- Saves happen only at tick boundaries, so no system is mid-run and every per-tick list is empty.
- An asleep thermal chunk serializes as one byte; an awake chunk whose cells all returned to ambient serializes in full until the run that puts it to sleep.
- The light grid is saved, not recomputed, because budgeted relighting makes it path-dependent.

---

## 17. Performance budgets

Reference: one Steam Deck core (Zen 2, 3.5 GHz boost), `dist` profile, 4 Errata, 256 actors, 1,024 projectiles, 2,048 item drops. Doc 02 registers each row as a `perf.deck.sim.*` gate; criterion benchmarks enforce the per-unit costs.

| Subsystem | Unit cost | Typical per due tick | Worst per due tick |
|---|---|---|---|
| Movement and collision (§5) | ≤ 3 µs per body (12 cells, 4 passes, 7 substeps max) | 0.45 ms | 1.2 ms |
| Push and broadphase | ≤ 0.1 µs per pair | 0.08 ms | 0.3 ms |
| Pose and history | ≤ 0.1 µs per entity | 0.03 ms | 0.05 ms |
| Raycast (§5.18) | ≤ 15 ns per visited cell | — | — |
| Projectiles (§5.19) | ≤ 0.4 µs each | 0.15 ms | 0.5 ms |
| Explosions (§12) | ≤ 0.35 ms each (1,352 rays × ≤ 25 samples) | 0 | 1.4 ms (4 per tick) |
| Block updates | ≤ 0.2 µs per pop | 0.01 ms | 0.4 ms |
| Random ticks | ≤ 0.1 µs per pick | 0.03 ms | 0.1 ms |
| Liquids | ≤ 100 ns per cell | 0.4 ms | 1.6 ms (16,384 cells) |
| Gases | ≤ 90 ns per cell | 0.3 ms | 1.5 ms |
| Fire | ≤ 0.2 µs per burning cell | 0.05 ms | 0.8 ms (4,096 cells) |
| Temperature | ≤ 6 ns per awake thermal cell | 0.1 ms | 3.2 ms (all 524,288 cells awake; 0.9 ms on 4 workers) |
| Light | ≤ 50 ns per pop | 0.05 ms | 0.2 ms (4,096 pops) |
| Sound | ≤ 60 ns per settled cell | 0.15 ms | 2.0 ms (32,768 cells) |

The slow systems never share a tick (§2.3), so the worst simulated tick is the per-tick systems (≈ 4.1 ms) plus the worst single slow system (3.2 ms), 7.3 ms, inside the 16.67 ms tick. Memory for 128 chunks: blocks ≤ 4 MiB, light 4 MiB, bitsets 1.5 MiB, thermal ≤ 2 MiB, liquids and gases ≤ 3 MiB at their caps (131,072 liquid cells, 65,536 gas cells).

---

## Exports

| Namespace | IDs defined here |
|---|---|
| `comp.` | `comp.ai`, `comp.body`, `comp.body_plan`, `comp.buildup`, `comp.container`, `comp.control`, `comp.corpse`, `comp.env`, `comp.equipment`, `comp.faction`, `comp.falling_block`, `comp.gadget`, `comp.interest`, `comp.inventory`, `comp.item_drop`, `comp.kind`, `comp.lifetime`, `comp.light_emitter`, `comp.mining`, `comp.mount`, `comp.move`, `comp.perception`, `comp.player`, `comp.pools`, `comp.pos`, `comp.pose`, `comp.projectile`, `comp.rider`, `comp.stats`, `comp.statuses`, `comp.tags`, `comp.vel`, `comp.zone` |
| `sys.` | the 35 IDs of §2.2: `sys.tick_begin`, `sys.interest`, `sys.spawn_admit`, `sys.input_apply`, `sys.perception`, `sys.ai_think`, `sys.actions`, `sys.movement`, `sys.push`, `sys.env_sample`, `sys.broadphase`, `sys.pose`, `sys.hitscan`, `sys.melee`, `sys.projectiles`, `sys.explosions`, `sys.damage`, `sys.block_updates`, `sys.random_ticks`, `sys.liquids`, `sys.gases`, `sys.fire`, `sys.temperature`, `sys.surfaces`, `sys.status_pulse`, `sys.damage_late`, `sys.needs`, `sys.light_emitters`, `sys.light`, `sys.sound`, `sys.hearing`, `sys.erasure`, `sys.status_timers`, `sys.despawn`, `sys.tick_end` |
| `grid.shape.` | `none`, `full`, `pillar`, `slab`, `stairs`, `door`, `trapdoor`, `ladder`, `fence`, `pane`, `layer`, `cross`, `fixture` |
| `grid.liqi.` | `blocks`, `permeable`, `displaced`, `absorbs` |
| `grid.rt.` | `none`, `grow`, `decay`, `spread` |
| `grid.ekind.` | `erratum`, `creature`, `mount`, `projectile`, `item_drop`, `falling_block`, `container`, `gadget`, `zone`, `corpse`, `marker` |
| `grid.move.` | `ground`, `air`, `swim`, `climb`, `mantle`, `dodge`, `fly`, `downed`, `ridden`, `hover`, `glide` |
| `grid.cause.` | `worldgen`, `script`, `placed`, `mined`, `broken`, `explosion`, `fire`, `liquid`, `unsupported`, `fell`, `landed`, `melted`, `froze`, `reaction`, `toggled`, `grew`, `decayed`, `spread` |
| `grid.despawn.` | `expired`, `consumed`, `picked_up`, `merged`, `folio_exit`, `cap_evicted`, `landed`, `dismissed`, `script` |
| `grid.gait.` | `crouch`, `walk`, `sprint`, `crawl` |
| `ev.` (bus additions) | `ev.explosion`, `ev.fire_started`, `ev.fire_out`, `ev.footstep`, `ev.entity_landed` |
| `block.` (shared with 07) | `block.air` (type index 0) |
| `rng.` | none new; this chapter draws only `rng.fire`, `rng.blocktick`, `rng.physics` (Canon §3.2) |

## Interface assumptions

| Doc | This chapter assumes |
|---|---|
| 02 | the primitives, store, scheduler, `PlayerCommand` axes and bits, `SimEvent`, section order and `palette_rle` as imported; `palette_rle` also accepts the 4,096-entry thermal array; registers the §17 rows as `perf.deck.sim.*` gates |
| 04 | Errata `BodyState` values; predicates `can_sprint`, `can_dodge`, `can_jump` and stamina spending; breath drain from `Env.eye_submerged` and unbreathable eye gas; Acrobatics after `fall_dmg_bp`; crutch tool flag; the `body.*` part lists that pose tables reference |
| 05 | `queue_hit(HitReq { source, attacker, target, part, b_mhp, shares, flags, instigator_slot, pos, seq })` and the pipeline in `sys.damage` and `sys.damage_late`; calls to `surface_trigger` (§6.9); weapon and projectile `block_bp` and tool tier; projectile templates (`radius`, `gravity_bp`, `drag_bp`, `bounce_bp`, pierce, lifetime); melee traces built on doc 02 primitives; Contradiction re-evaluation called from `sys.env_sample`; `kb_resist_bp`; animation choice; mount archetypes with seat offsets; item explosion `b_mhp`; friendly-fire scaling from `instigator_slot` |
| 06 | zones, `st.levitating` control, `add_heat`, `emit_gas`, `deposit` for spells |
| 07 | every block record with the §4.5 fields; material defaults (hardness, blast resistance, flammability, conductance, conceal, sound attenuation); `mining_ticks(state, tool, user) -> Option<u32>`; `block_drops(state, cause, actor)`; ordered `tooltier.*`; block items mapping to states; `block.ice` (`friction_bp` 1500) and `block.web` (`burn_runs` 5) with Canon §12.1 behaviour |
| 08 | liquid records (§6.5) for at least `liq.water`, `liq.oil`, `liq.alcohol`, `liq.acid`, `liq.lava`, `liq.red_ink`, `liq.blood`; gas records (§7.2) for `gas.smoke`, `gas.steam`, `gas.miasma`; `alch_react` (§6.8); splash amounts for thrown containers |
| 09 | noise IDs and levels; conversion of `ev.footstep`, `ev.entity_landed` and action events into `emit_noise`; consumption of `HeardNoise`; perception through `raycast_blocks`, `cell_conceal`, light and `flashes`; the awareness query used for sleep |
| 11 | item-drop contents, pickup, Lost Satchel merging at `GROUND_ITEM_CAP`, `carried_g`, death drops in `sys.despawn` |
| 12 | worldgen chunks, `open_tag` and `protect` bits, liquid sources, gravity-flip volumes, bleed zones for `local_genre`, Genre Parameter Vector field `ambient_decic`, hazard placement |
| 13 | AI writes `control`; `request_spawn`; `comp.tags` bit layout; creature `BodyState` templates; navigation rebuilt from `ev.block_changed` |
| 16 | delivery of commands and intents per tick; lag compensation through `Pose.hist`; snapshot cadence |
| 18 | `art.anim.*` pose tables committed as content with the §5.16 schema; `art.block.*` skins |

## Canon gaps

- **G1. Version.** The contract fixes the header to canon-1.1. The canon on disk is canon-1.3; every rule here was checked against it, including the 1.2 jump curve, air jump, lean, two-channel light, surfaces, concealment, breath, mounts, ghosts and the 42-event list.
- **G2. Liquid units.** Canon types liquid volume as `Ml`. The grid stores 1-litre units (1000 ml, 1000 per cell). Canon §12.1's 100 ml contact threshold is therefore met by any non-zero unit.
- **G3. Explosion draws.** Canon §3.3 keys every action-caused draw, while §3.2 assigns explosion block-break variation to sequential `rng.physics`. Explosions resolve host-side in queue order and are never client-predicted, so this chapter uses §3.2 sequential draws, and incendiary ignition draws `rng.fire` (§3.2 "fire ignition"). The canon should name this exception in §3.3.
- **G4. New events.** Canon §19 gives `ev.` to 00 for core lists and to 03 for the bus. This chapter adds five bus events (Exports) and treats that as within its ownership.
- **G5. Two hit batches.** Canon §10.8's `(source EntityId, seq)` order holds inside `sys.damage` and inside `sys.damage_late`; environmental, contact and DoT hits of a tick resolve after weapon hits.
- **G6. Ghosts.** "A ghost has no collision" is read as no body collision and no hit or perception queries; voxel collision stays so ghosts cannot leave the Folio.
- **G7. Ambient temperature.** The canon has no per-genre temperature. This chapter reads doc 12's `ambient_decic`, default 200.
- **G8. Surface-trigger geometry.** Canon §12.1 does not say which cells a trigger touches or how much water becomes steam. This chapter fixes trigger cells, the 64-cell charge, the 100-unit freeze minimum, the 27-cell steam cap and one blast per hit (§6.9).
- **G9. Sound units.** `Db` is whole decibels; propagation runs in tenths and floors to whole dB on output.
- **G10. Sky channel.** "Open areas" (Canon §5.2) means sky-exposed or worldgen-tagged cells; the sky channel then propagates like block light.
- **G11. Pose keyframes.** Content stores keyframes every `key_every` ticks; the loader expands them to the canon per-tick table with integer interpolation.
- **G12. Serialization encodings.** Doc 02 §6.5 lists palette-RLE for fluids and thermal; sparse liquid and gas lists and `u16` thermal offsets are used (§16.1).
- **G13. Genre fall multiplier.** Canon §5.9 `fall_dmg_bp` is applied to the canon fall damage before doc 04's Acrobatics.

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
