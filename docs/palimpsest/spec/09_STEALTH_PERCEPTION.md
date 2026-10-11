# 09 — Stealth and Perception
As of 2026-10-11. Status: BINDING. Conforms to canon-1.2.

This chapter owns how every simulated observer sees, hears, smells and feels the world, the per-pair detection meter and the awareness state machine, the noise catalog and its conversion into `ev.noise`, sneak attacks and takedowns, hiding, bodies, disguises, distractions, light manipulation, locks, hacking, traps, pickpocketing, co-op stealth and the genre stealth flavors. Everything here runs inside `palimpsest_sim` on integers. Detection is deterministic: it draws no random numbers. Only lockpicking, hacking, trap work, pickpocketing and passive trap and secret detection draw, all from keyed `rng.stealth`. The canon on disk at writing time is canon-1.4; every rule below was checked against it (Canon gaps G1).

## Imports

| Source | Section | What this chapter consumes |
|---|---|---|
| Canon | §1.2–§1.6 | `Fx`, `Bp`, `Bam`, `Ticks`, `Db`; two-channel `Light`; `floor_div`, `mul_bp`, `dist2`, `sqrt_fx`; integer-only content in mm, Bam, ticks |
| Canon | §1.8, §1.9 | coordinates; `ang_diff`, `atan2_bam`, `fwd`, `sin_bam`, `cos_bam` |
| Canon | §1.11 | ascending-`EntityId` iteration; phase key; no `HashMap` |
| Canon | §2.2, §2.3 | perception period 3, AI think period 6; rate accumulator; `in_combat`; `light_at` |
| Canon | §3.3, §3.4 | keyed `rng.stealth` (owner 09); trap and secret detection cadence and `seq` rule; documented draw counts |
| Canon | §4.1–§4.3 | humanoid capsule and eye heights; speeds; lean; terrain and placeables |
| Canon | §5.3–§5.8 | dependencies, Contradiction potency and failure for tools; local genre and local axioms |
| Canon | §6.4, §6.5 | rows 17, 24, 31–35, 40; NPC rows 31–35 |
| Canon | §7 | `skill.sneak`, `skill.security`, `skill.pickpocket`, `skill.hacking`, `skill.awareness` and their scope notes |
| Canon | §10.5, §10.6 | step 2.4 sneak eligibility per (attacker, target); step 2.4b flanking cone; step 5 `ignore_armor`; entry points |
| Canon | §11.1, §11.4 | statuses owned here: `st.deafened`, `st.marked`, `st.invisible`, `st.hidden`, `st.hacked`; `st.sleeping`, `st.blinded`, `st.mounted`; direct-application duration |
| Canon | §12, §12.1, §12.2 | `res.breath`; smoke and surfaces; sight concealment sums |
| Canon | §13.2 | `slot.face` masks; `slot.body`, `slot.back` |
| Canon | §15, §17.3, §17.6, §17.7, §17.9 | factions and stances; ranks; ghosts imperceptible; mounted cannot be hidden; invaders |
| Canon | §18 | `ev.noise`, `ev.awareness_changed`, `ev.damage_dealt`, `ev.block_changed`, `ev.entity_killed`, `ev.folio_entered`, `ev.mounted` |
| Doc 03 | §2.2, §3.3, §5.8, §5.15, §5.18, §7.6, §10, §11, §12.5, §14.2, §15.2 | system slots `sys.perception` (#5) and `sys.hearing` (#31); `comp.perception`; lean offset; `ev.footstep`, `ev.entity_landed`; `raycast_blocks`; `cell_conceal`; light grid and `flashes`; `emit_noise`, sound propagation, `HeardNoise`; explosion noise and flash; payload schemas; sleep query |
| Doc 04 | §4.2, §5.2, §5.3, §7.2 | check formula and DC ladder; stealth XP events; mastery perks of Sneak, Security, Pickpocket, Hacking, Awareness, Illusion, Divination; Shade and Netrunner talents; `light_step`, `fair_folk`, `wired`, `gumshoe` traits |
| Doc 05 | §1.6, §3.2, §4.9, §5.2, §6.8, §11.1, §11.4 | archetype `sneak_tier`; stance speeds; lean; armor `noise_db`; the tier values 1 = 30000, 2 = 22500, 3 = 15000 that doc 05 assumes |
| Doc 06 | §5.2, §7.1, §13, §14 | form cast noise; schools; shadow essence darkness; `glyph.ess.veil`; hushed; sight essence; `st.sleeping` awareness drop |
| Doc 08 | §3, §4, §6.3, §7 | liquid and gas conceal; respirators and their hearing penalty; shatter noise; phosphor; mind vision; invisibility potions |
| Doc 10 | §5.3, curse table | witness sight pass; `curse.loud`, `curse.beacon` |
| Doc 11 | items, affixes, manufacturers | `item.face.mask`, `item.tool.lockpicks`, `item.trinket.warning_bell`, `affix.quiet`, `affix.soft_step`, `affix.ambusher`, `perk.leg.shadowweave`, Silverbough and Midnight Avenue traits |
| Doc 12 | §2.1, §10.2, §16.3, §17 | GPV `fog`, `sound_bp`; light bands; `room.stealth`; hiding methods of secrets; `hazard.camera`, `hazard.alarm`, trap DC placement |
| Doc 13 | §2.1, §2.4, §4.2, §4.7, §5.4, §5.7, §9.1, §10.5 | template `senses`, tags, roles, ranks; blackboard `last_known`; investigate, search and call_allies actions; stalkers; hacked constructs; boss arena omniscience; hunters |
| R5 | §2, §9.2 | per-pair meter, zone rates, light table, footstep dB, hysteresis, callouts, Vigilance, darkness defense |
| R2 | §2.10, §2.11, §3.6 | Oblivion detection hysteresis and sneak perks; tumbler pity; BG3 obscurement |
| R7 | §1.4 C2, R-BA2, R-BA4 | the R5 meter as the base model; lock-health pity; engineered noise counts as stealth |

---

## 1. Conventions, schedule and state

### 1.1 Notation

- `o` is an observer, `t` (in formulas) a target or subject. The current tick is `now`.
- `d` is the current Folio depth (Canon §13.8). `d(x) = x - 40` (Canon §6.2).
- `L_s(e)` is stealth light (§2.1). `X(t)` is exposure (§3.1). `V(o, t)` is the visibility score (§3.4). `M(o, t)` is the detection meter (§5). All four are integers; `L_s` is `0..=15`, the others Bp.
- A **check** is one run of `sys.perception` for one observer (every 3 ticks, Canon §2.2). Per-second rates applied on checks use the Canon §2.3 accumulator with `k = 20`.
- "Ordinal" means the awareness ordinal of §6.1. "Below alert" means ordinal `< 5`.

### 1.2 What doc 09 decides

Everything listed in the introduction, the stealth HUD data contract (§8), and the behavior of `st.deafened`, `st.marked`, `st.invisible`, `st.hidden` and `st.hacked`. Other chapters reach it only through the functions listed under Exports.

### 1.3 Systems in the tick

| Doc 03 slot | Work done by doc 09 | Iteration |
|---|---|---|
| #5 `sys.perception` (period 3, per-entity phase) | for each due, awake, perceiving observer: candidate gathering, sight rays, sight fill, scent, breath, `st.marked`, decay, timers, sight-driven transitions, corpse and light-change discovery, camera timers, Errata reveal meters | ascending `EntityId`, then the backlog (§1.6) |
| #7 `sys.actions` (doc 05, 06, 07, 11 action code) | calls `noise_level()` (§7.1) when those chapters emit noise; lock, hack, disarm, pickpocket, takedown, hide and bluff actions run their doc 09 state machines here | ascending `EntityId` |
| #30 `sys.sound`, prelude | `convert_events()` (§7.9) turns this tick's `ev.footstep`, `ev.entity_landed`, `ev.block_changed`, `ev.entity_killed`, `ev.fire_started` into `emit_noise` requests, in event `seq` order, before doc 03 propagates | event order |
| #31 `sys.hearing` | contact from `ev.damage_dealt`, then `HeardNoise` reactions, tremor, callouts, alarms, pickpocket and takedown results, `st.hidden`, `ev.awareness_changed` flush | listener ascending `EntityId`, records in list order |

Awareness read by doc 05 at #17 `sys.damage` is the state after #5 of the same tick and after #31 of the previous tick.

### 1.4 State

`comp.perception` (doc 03 storage, doc 09 semantics) holds one `PerceptionState` per perceiving creature, gadget and device. The Folio-level `StealthWorld` is saved with the simulation (Canon §2.5) as doc 02 section `stealth`.

```rust
pub struct PerceptionState {
    pub profile: Id32,                 // aware.profile.* (§4.1)
    pub senses: u8,                    // bit 0 sight, 1 hearing, 2 breath, 3 tremor, 4 thermal, 5 scent
    pub pairs: Vec<PairAware>,         // sorted by subject EntityId; at most 16
    pub disturb: Option<Disturbance>,  // subject 0: the environment (§6.4)
    pub wary_until: Tick,              // Vigilance (§6.3)
    pub corpse: Option<(EntityId, i32)>, // body under evaluation and its discovery meter (§10.4)
    pub search: Option<SearchPlan>,    // §6.7
    pub cam_seen: i32,                 // camera sight ticks (§15.2)
    pub subdued_until: Tick,           // non-lethal takedown (§9.4)
    pub bluffs: u8, pub satisfied_until: Tick,   // disguise bluffs (§11.4)
    pub acc_decay: i64,                // shared decay accumulator for the disturbance
}
pub struct PairAware {
    pub subject: EntityId,
    pub state: u8,                     // awareness ordinal 0..=5 (§6.1)
    pub meter: i32,                    // 0..=10000
    pub suspicion: i32,                // disguise suspicion 0..=10000 (§11.2)
    pub acc: i64,                      // decay accumulator, Canon §2.3, k = 20
    pub last_gain: Tick, pub last_contact: Tick, pub state_since: Tick,
    pub lkp: Fx3, pub lkp_vel: Fx3, pub lkp_tick: Tick, pub lkp_src: u8,  // src: 0 sight, 1 sound, 2 callout, 3 damage, 4 scent, 5 marked
    pub flags: u8,                     // 0 SIGHT_PASS (latest check), 1 HEARD_THIS_TICK, 2 ARRIVED, 3 CALLED_OUT
}
pub struct Disturbance { pub pos: Fx3, pub tick: Tick, pub state: u8, pub meter: i32, pub since: Tick }
pub struct StealthWorld {
    pub locks: BTreeMap<u64, LockState>,          // key: door Gidx (lower half) or container EntityId (§13.1)
    pub hides: BTreeMap<u64, HideState>,          // hiding spots (§10.1)
    pub scent: VecDeque<ScentMark>,               // ring, capacity 4096 (§4.5)
    pub revealed: BTreeMap<u64, u8>,              // hidden objects (traps, secrets, mimics, wards) → party-slot mask
    pub reveal: BTreeMap<(EntityId, EntityId), RevealMeter>, // (erratum, stealthy NPC) (§4.8)
    pub blown: BTreeSet<(EntityId, Id32)>,        // (erratum, faction) disguises blown this Folio (§11.4)
    pub discovered: BTreeSet<EntityId>,           // bodies already reported (§10.4)
    pub alarms: BTreeMap<u64, Tick>,              // alarm source → cooldown end (§6.6)
    pub callouts: Vec<Callout>,                   // this tick's callout payloads, cleared at sys.tick_begin
    pub backlog: VecDeque<EntityId>,              // deferred perception checks (§1.6)
    pub hacked: BTreeMap<EntityId, HackState>,    // hacked devices and constructs (§13.4, §13.5)
    pub folio_alerted: bool, pub run_alerted: bool,
}
```

`folio_alerted` resets on `ev.folio_entered`; `run_alerted` never resets during a run. Both become true the first time any pair toward an Erratum reaches `aware.alert`. Canon §8's Unseen bonus reads `!folio_alerted` at exit; doc 14's `run.note.silent` reads `!run_alerted`.

### 1.5 Keyed draw allocation (`rng.stealth`)

Every draw uses `keyed_rng("rng.stealth", folio, actor, now, seq)` (Canon §3.3). Action slots start at 1000 so they never share a key with the Canon §3.3 detection rule, whose `seq` is an object index below 1000.

| `seq` | `actor` | Draws, in order | Count |
|---|---|---|---|
| object index `i` (`0..=999`) | the observer | passive trap and secret detection, Canon §3.3 and §13.6 | 1 per object in range |
| 1000 | the picker | lockpick attempt: success roll, then break roll | 2 |
| 1001 | the hacker | device or construct hack attempt: success roll | 1 |
| 1002 | the disarmer | trap disarm: success roll, then trigger-on-failure roll | 2 |
| 1003 | the thief | pickpocket or plant: success roll | 1 |
| 1004 | the picker | manual lock minigame layout: one `range(40)` per pin | `2 + tier` |
| 1005 | the picker | manual minigame pick-break roll, one per missed pin, in miss order | 0..=3 |

Perception, meters, transitions, scent, hearing, search patterns and corpse discovery draw nothing.

### 1.6 Budget

`sys.perception` casts at most `RAY_BUDGET = 1536` sight rays per tick. Each observer check gathers at most 8 subjects, nearest first by `dist2`, ties by lower `EntityId`. When the next observer's worst case (3 rays per subject) would exceed the remaining budget, that observer and every later due observer are pushed to `backlog` in ascending `EntityId`; the next tick processes the backlog first, then its own due observers. A deferred check runs with `now` of the tick it actually runs. Reference cost: 85 due observers × 2 subjects × 3 rays = 510 rays per tick in a typical Folio.

---

## 2. Light at the target

### 2.1 Two-channel light and stealth light

Each cell stores `light_sky` and `light_block` (Canon §1.2). Effective light is `max(light_sky, light_block)`; `light_at(e)` reads the cell at `e.pos + (0, 500 mm, 0)` (Canon §2.3). Spawning reads `light_block` only (Canon §12.3); stealth always reads effective light. Stealth light adds the target's own light, flashes and magical darkness:

```
fn stealth_light(e, now) -> u8:
    env   = light_at(e)                                    // includes the grid's copy of e's own emitter, capped at 10 by doc 03 §10.4
    dark  = 8 if e's light cell lies in a doc 06 shadow area (field or linger) else 0
    env   = max(0, env - dark)
    own   = max(0, max over e's lit equipment and held items of item_light(i))   // §2.3 table
    fl    = max(0, max over this tick's and the previous 2 ticks' flashes f of (f.level - taxicab(cell(e), f.cell)))
    ph    = 9 if e is phosphor-marked (doc 08) else 0
    min(15, max(env, own, fl, ph))
```

Flashes come from doc 03 §10.4's per-tick list; doc 09 copies each tick's list into a 3-tick ring so an observer whose check falls 1 or 2 ticks later still sees the muzzle flash.

**Example.** A Gothic Horror crypt corridor (`ambient_light` 2, no fixtures) has effective light 2. An Erratum holding a torch (`own` 12) has `L_s = 12`. Without the torch, a muzzle flash of level 12 in its own cell gives `L_s = 12` for 3 ticks, then 2 again. Standing 2 cells (taxicab) from an ally's flash gives `12 − 2 = 10`. Inside a doc 06 shadow field the dark corridor stays `max(0, 2 − 8) = 0`, but the torch still gives 12.

### 2.2 Light factor and bands

`LIGHT_BP[L]` for `L = 0..=15`:

| L | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Bp | 800 | 1000 | 1300 | 1700 | 2200 | 2800 | 3500 | 4300 | 5100 | 5900 | 6700 | 7500 | 8200 | 8900 | 9500 | 10000 |

Bands follow doc 12 §10.2: **dark** `L ≤ 4`, **dim** `5..=9`, **bright** `≥ 10`. The far-zone posture rule (§3.3) and every "in darkness" condition below use `L ≤ 7` or the band named.

### 2.3 Own emitted light

| Source | `item_light` |
|---|---|
| `arch.offhand.lantern` (doc 05) | 13 |
| `item.tool.torch` held or in `slot.offhand` | 12 |
| `ench.lightbearer` ranks 1, 2, 3 (doc 06) | 8, 11, 14 |
| `curse.beacon` (doc 10) | `min(15, 2 + m)` |
| doc 06 `lumen` self light | 12 |
| `aware.profile.drone` spotlight (on its own cone, §15.2) | 12 |
| a burning bearer (`st.burning`) | 12 |

Doc 03 caps grid emitters at 10, so the grid alone understates a lantern; `own` restores the true level for stealth. A lit item in the inventory but not held or worn emits nothing. Toggling a lantern or torch off is `input.interact` while holding it (6 ticks).

### 2.4 Light manipulation

Light blocks are doc 07 records with `light_emit > 0`. Doc 09 classifies them by a `lightsrc` field: `flame` (torches, candles, braziers, oil lamps), `electric` (fixtures on a room circuit) and `arcane` (glowstones, wards). Every Errata-placeable light is a `cat.placeable` (Canon §4.3).

| Action | Applies to | Ticks | Noise | Result |
|---|---|---|---|---|
| Snuff (`input.interact`) | flame | 15 | `noise.light.snuff` 20 dB | `set_block` to the unlit state, cause `grid.cause.toggled` |
| Douse | flame | — | — | any cell contact with ≥ 1 unit of `liq.water`, or a frost hit, unlights it (doc 03 §6.9 hook) |
| Relight (`input.interact` with a flame source or `dmg.fire` hit) | flame | 30 | `noise.light.snuff` 20 dB | lit state |
| Switch (`input.interact` on `block.light_switch`) | electric circuit | 6 | `noise.light.switch` 25 dB | every electric fixture of the room toggles |
| Break | any destructible fixture | weapon | block break noise of its acoustic class (§7.5) | fixture removed; light gone |
| Power cut: hack or break `block.fuse_box` | electric, whole room | §13.4 | `noise.light.power` 45 dB | every electric fixture of the room unlit for 1800 ticks, then relit unless the box is broken |

Every change goes through `set_block`, so doc 03 relights the grid within its 4,096-pop budget and both channels stay correct. Worldgen fixtures are destructible (doc 12 §2.2).

**AI reaction to darkness.** On every `ev.block_changed` whose old state emitted light and whose new state emits less by at least 4 levels, each awake sapient NPC within 16000 mm of the cell whose eye has a clear `raycast_blocks` line to the cell center gains a disturbance (§6.4) at `aware.suspicious` with position the cell. On arrival the NPC relights a flame source if its template carries `uses_items`, or walks to the room's fuse box for an electric cut; relighting takes 60 ticks. Doc 13 runs the walk; doc 09 sets the goal. Beasts, undead and constructs ignore light changes.

**Example.** A Shade snuffs one of two wall torches (`light_emit` 14) in a Mythic Fantasy hall. The guard 9 m away sees the torch cell, so its disturbance becomes suspicious at the torch: it walks over, spends 60 ticks relighting and, if it found nothing, searches (§6.2 T7). A second guard behind a wall gains nothing.

---

## 3. Visibility score

The visibility score `V(o, t)` is the share, in Bp, of a fully exposed reference target that observer `o` can see of target `t` on this check. It is the product of a target-side **exposure** and observer-side geometry, each factor one `mul_bp` in the listed order (Canon §1.3).

### 3.1 Exposure

```
X(t, o) = 10000 → × f_stance → × f_motion → × f_size → × f_armor → × f_camo → × f_invis(o) → × f_gear(o)
```

**Stance** `f_stance`, first matching row:

| Condition | Bp |
|---|---|
| `st.mounted` (Canon §17.7) | 15000 |
| `st.downed`, or a body or Remains (§10.3) | 3500 |
| eye cell submerged (doc 03 `eye_submerged`) | 3000 |
| swimming, eye above the surface | 7000 |
| climbing or mantling | 12000 |
| airborne, gliding or dodging | 12000 |
| crouched | 5500 |
| standing | 10000 |

**Motion** `f_motion`: horizontal speed `s` in mm/s from `vel`, and the gait of doc 03 §5.15 (`grid.gait.*`). NPCs use crouch when crouched, sprint when moving faster than their template `walk_mm_s`, else walk.

| Condition | Bp |
|---|---|
| `s ≤ 300` | 6000 (still) |
| `s > 7500` (gallop, knockback, dodge burst) | 16000 |
| gait crouch or crawl, or swimming or climbing | 8000 |
| gait walk | 10000 |
| gait sprint | 13000 |

**Size** `f_size = clamp(floor_div(stand_h_mm × radius_mm, 54), 2500, 40000)`. A humanoid (1800 mm, 300 mm) is exactly 10000; a 900 × 200 mm goblin-sized body is 3333; a 3600 × 900 mm troll is 40000 (clamped from 60000).

**Armor signature** `f_armor = 10000 + Σ worn armor pieces slot_weight × class_sig`, with slot weights head 1, body 4, hands 1, legs 2, feet 2 (doc 05 §11.3) and `class_sig` cloth 0, light 50, medium 100, heavy 200. Full sets: light 10500, medium 11000, heavy 12000.

**Camouflage** `f_camo`. The **outer layer** is the `slot.back` item when it carries a camo family, else the `slot.body` item. Its skin carries `camo: aware.camo.*` (default none). The **background** is the local genre's primary and secondary family (§15.1), or the doc 12 biome tileset's `camo` field when present.

| Relation | Bp |
|---|---|
| outer family ∈ {primary, secondary} | 7500 |
| no family, or no relation | 10000 |
| outer family clashes with the primary | 12500 |

Families are `aware.camo.shadow`, `.earth`, `.foliage`, `.urban`, `.pale`, `.neon`. Clashing pairs (symmetric): shadow–pale, shadow–neon, earth–pale, earth–neon, foliage–urban, foliage–neon, pale–neon.

**Invisibility** `f_invis(o)`: 10000 without `st.invisible`. With it, 1000 (Canon §11.1). Wards and lasers do not use sight, so invisibility never affects them. `f_invis = 10000` when the observer sees invisible: an Erratum with Awareness ≥ 75 within 4000 mm (doc 04), with the doc 06 Divination hook within 12000 mm.

**Gear and talents** `f_gear(o)`, every applicable factor in ascending source content-ID byte order:

| Source | Condition | Bp |
|---|---|---|
| `perk.leg.shadowweave` (doc 11) | crouched and `L_s ≤ 4` | 7000 |
| `talent.netrunner.ghost_signal` (doc 04) | observer profile camera, turret or drone | 7000 |
| `talent.shade.ghost_walk` (doc 04) | gait crouch and `s > 300` | 8000 |
| carrying a body or Remains (§10.3) | always | 15000 |

### 3.2 Sight rays, visible fraction and concealment

Three **sample points** on the target: head = its eye point, shifted by its lean offset (doc 03 §5.8, doc 05 §6.8); torso = `pos + (0, h/2, 0)`; feet = `pos + (0, 100 mm, 0)`, where `h` is the current capsule height. The ray origin is the observer's eye point; an Erratum observer's eye includes its lean. For each sample, in the order head, torso, feet:

```
fn ray(o_eye, sample, o, t) -> Option<i32>:                 // None = blocked, Some(conceal sum)
    dir  = sample - o_eye; len = sqrt_fx(dist2(o_eye, sample))
    hit  = raycast_blocks(o_eye, dir, len)                  // doc 03 §5.18
    if hit.is_some(): return None
    sum  = Σ cell_conceal(c) over cells c crossed, excluding the origin and sample cells   // Canon §12.2, doc 03 §7.6
           (thermal observers use the block and liquid terms only; crouch-only cells, doc 07 flag,
            count only when t is crouched, downed or a body)
    sum += floor_div(fog_q × to_mm(len), 8000)              // fog_q = doc 12 GPV fog of local_genre(o_eye), +1 during a weather front
    if sum >= 15: return None
    Some(sum)
```

`n` = number of `Some` results. `f_frac = [0, 4000, 7000, 10000][n]`. `c_min` = the smallest `Some` value. `f_conc = 10000 − 667 × c_min` (Canon §12.2). Other bodies never block sight. A target inside an occupied enclosed hiding spot (§10.2) has `n = 0` for everyone except the checker of that spot.

**Lean.** A standing, still Erratum leaning 9 steps past a corner exposes only its head ray: `n = 1`, so `f_frac = 4000`. At `L_s` 10, 7 m in a guard's near cone, `X = 6000` and `V = 1608`; stepping out still gives `V = 4020`, walking out `V = 6700`. The capsule never moves while leaning, so motion stays "still" and no footsteps are emitted.

**Concealment examples.** Two half-density smoke cells (conceal 2 each) and 12 m of Gothic Horror fog (`fog_q` 1, `floor_div(12000, 8000) = 1`) give `sum = 5` and `f_conc = 6665`. Three cells of tall grass (2 each, crouch-only) in front of a crouched target give `f_conc = 5998`; the same grass in front of a standing target gives 10000.

### 3.3 Zones, distance and angle

The observer's profile (§4.1) defines `near_mm`, `far_mm` and half-angles `near_half`, `far_half`, `periph_half` (Bam). Ranges grow with Canon §6.4 row 32:

```
r32    = d(SGT_o) × 50                                    // Bp, Canon §6.4 row 32
R_near = mul_bp(near_mm, 10000 + r32);  R_far = mul_bp(far_mm, 10000 + r32)
dist   = to_mm(sqrt_fx(dist2(o_eye, torso(t))))
θ_h    = ang_diff(o.yaw, atan2_bam(-(t.x - o.x), -(t.z - o.z)))                       // bearing as in doc 05 §12.2
θ_v    = ang_diff(o.pitch as Bam, atan2_bam(t_torso.y - o_eye.y, horizontal distance))
```

| Zone | Condition (`θ_v ≤ 10923` for every cone) | `f_zone` | `f_dist` |
|---|---|---|---|
| near | `dist ≤ R_near` and `θ_h ≤ near_half` | 10000 | 10000 |
| far | `dist ≤ R_far` and `θ_h ≤ far_half`, not near | 2000 | `10000 − floor_div(7000 × (dist − R_near), R_far − R_near)` |
| peripheral | `dist ≤ R_near` and `θ_h ≤ periph_half`, not near | 1000 | 10000 |
| outside | anything else | 0 | — |

Posture rules (Shadow Tactics zones, R5 §2.4): in the **far** zone a crouched, downed or submerged target with `L_eff ≤ 7` gives `f_zone = 0`; in the **peripheral** zone a target with `s ≤ 300` or gait crouch gives `f_zone = 0`. Cameras and turrets sweep `o.yaw` (§15.2); every other observer uses `comp.pos` yaw and pitch.

**Example.** A guard with SGT 45 has `r32 = 250`, `R_near = 8200`, `R_far = 24600`. A target 20000 mm away in the far cone has `f_dist = 10000 − floor_div(7000 × 11800, 16400) = 4964`.

### 3.4 The full score

```
L_eff  = stealth_light(t); if dist ≤ dv(o): L_eff = max(L_eff, 8)          // §4.3
V(o,t) = X(t, o) → × LIGHT_BP[L_eff] → × f_conc → × f_frac → × f_zone → × f_dist
```

`V` is 0 when `n = 0`, when the zone is outside, when `o` has `st.blinded` and `dist > 2000` (Canon §11.1), or when `t` is a ghost (Canon §17.6). A **sight pass** is `V(o, t) ≥ 1`; doc 13's `ai.input.target_visible` reads the latest check's sight pass.

**Worked example V1 (dark corridor).** A Shade crouch-walks at 2000 mm/s in a full light set under a shadow-family cloak in Gothic Horror (background shadow, earth) with `ghost_walk`, 6000 mm from the SGT 45 guard above, 20° (3641 Bam) off its facing, `L_s = 3`. `X = 10000 → 5500 → 4400 → 4400 → 4620 → 3465 → 2772` (stance, motion 8000, size, armor 10500, camo 7500, ghost_walk 8000). Dark vision is 4 m, so `L_eff = 3`. All three rays are clear, fog adds `floor_div(6000, 8000) = 0`, the zone is near: `V = mul_bp(2772, 1700) = 471`.

**Worked example V2.** The same Shade at 14000 mm inside the far cone: crouched with `L_eff = 3 ≤ 7`, so `f_zone = 0` and `V = 0`. Darkness plus crouching beats the far zone completely.

**Worked example V3 (bright hall).** A standing Erratum walks in a full heavy set at `L_s = 12`, 20000 mm away in the guard's far cone: `X = 12000`, `V = 12000 → 9840 → 9840 → 9840 → 1968 → mul_bp(1968, 4964) = 976`.

---

## 4. Perception by creature class

### 4.1 Perception profiles

A profile (`aware.profile.*`) holds every number of a creature class. Doc 13 templates carry `senses`; the profile supplies geometry, rates and ratings. `fill` is the meter gain per check at `V = 10000` (§5.2). `aw_base` and `sneak_base` feed the opposed ratings (§5.1). `hear_db` is the base hearing threshold. `dv_mm` adds to Canon §6.4 row 31.

| Profile | near / far mm | near / far / periph half (Bam) | fill | aw_base | sneak_base | hear_db | dv_mm | Flags |
|---|---|---|---|---|---|---|---|---|
| `aware.profile.humanoid` | 8000 / 24000 | 6372 / 10012 / 18204 | 2500 | 20 | 0 | 20 | 0 | — |
| `aware.profile.sentry` | 10000 / 32000 | 5461 / 8192 / 16384 | 3000 | 30 | 0 | 18 | 0 | — |
| `aware.profile.stalker` | 8000 / 20000 | 6372 / 10012 / 18204 | 2500 | 25 | 40 | 15 | 2000 | stealthy |
| `aware.profile.beast` | 6000 / 16000 | 8192 / 12743 / 21845 | 2000 | 25 | 20 | 15 | 2000 | ignores light changes |
| `aware.profile.hound` | 6000 / 14000 | 8192 / 12743 / 21845 | 2000 | 30 | 0 | 10 | 2000 | scent; ignores disguises; bark alarm |
| `aware.profile.undead` | 5000 / 12000 | 5461 / 8192 / 0 | 2000 | 10 | 0 | 25 | 6000 | — |
| `aware.profile.construct` | 10000 / 30000 | 4551 / 6372 / 0 | 3000 | 30 | 0 | 20 | 4000 | hackable |
| `aware.profile.flier` | 8000 / 32000 | 10923 / 16384 / 21845 | 2000 | 25 | 0 | 20 | 0 | `θ_v` limit 16384 |
| `aware.profile.swarm` | 4000 / 8000 | 32768 / 32768 / 32768 | 1500 | 10 | 0 | 20 | 4000 | — |
| `aware.profile.horror` | 0 / 0 | — | 0 | 40 | 30 | 5 | 0 | hearing only; hearing can alert; tremor |
| `aware.profile.civilian` | 6000 / 16000 | 6372 / 10012 / 18204 | 1500 | 10 | 0 | 20 | 0 | crime triggers only (§15.2) |
| `aware.profile.boss` | 16000 / 40000 | 10923 / 16384 / 24576 | 5000 | 60 | 0 | 15 | 4000 | — |
| `aware.profile.editor` | 12000 / 32000 | 8192 / 12743 / 21845 | 3500 | 50 | 0 | 15 | 8000 | ignores disguises |
| `aware.profile.camera` | 16000 / 16000 | 8192 / 8192 / 0 | timer | 0 | 0 | — | 0 (8000 at local TEC ≥ 8) | §15.2 |
| `aware.profile.turret` | 12000 / 20000 | 8192 / 8192 / 0 | 4000 | 30 | 0 | 15 | 8000 | hackable |
| `aware.profile.drone` | 8000 / 20000 | 10923 / 10923 / 0 | 2500 | 30 | 0 | 20 | 8000 | spotlight 12; hackable |
| `aware.profile.ward` | sphere 4000 | — | trip | 0 | 0 | — | — | ignores light and invisibility (§15.2) |
| `aware.profile.laser` | segment | — | trip | 0 | 0 | — | — | §15.2 |
| `aware.profile.tattler` | 6000 / 6000 | 32768 / 32768 / 0 | trip | 0 | 0 | 15 | 0 | §15.2 |
| `aware.profile.erratum` | 12000 / 48000 | 10923 / 10923 / 16384 | 2500 | skill | skill | — | 0 | reveal meters only (§4.8) |
| `aware.profile.invader` | 12000 / 64000 | 10923 / 10923 / 16384 | — | — | — | 20 | 0 | interest filter only (§14.5) |

```json
{ "id": "aware.profile.sentry", "near_mm": 10000, "far_mm": 32000,
  "near_half": 5461, "far_half": 8192, "periph_half": 16384, "theta_v": 10923,
  "fill": 3000, "aw_base": 30, "sneak_base": 0, "hear_db": 18, "dv_mm": 0,
  "step_db": 0, "flags": { "scent": 0, "hear_alerts": 0, "ignore_disguise": 0, "stealthy": 0 } }
```

NPC footstep offset `step_db`: humanoid, sentry, civilian, editor 0; stalker −10; beast and hound −5; undead +3; construct +8; boss +10; swarm −10; fliers emit wing beats instead (`noise.move.flap`, 35 dB every 30 ticks).

### 4.2 Default assignment

A doc 13 template may name `perception`. Otherwise the first matching rule wins: (1) rank boss or editor → boss; rank hunter → editor. (2) tag `editor` → editor. (3) gadget archetypes: turret → turret, drone → drone; `hazard.camera` → camera; `hazard.alarm` by skin dependency → laser (techne), ward (arcana or mythos), tattler (none in Fairy Tale and Mythic Fantasy), otherwise a tripwire segment that uses the laser rules without visibility. (4) tag `swarm` → swarm. (5) tag `eldritch` with no `sight` sense → horror. (6) role stalker → stalker; role sniper or guardian → sentry. (7) tag `construct` → construct. (8) tag `flying` → flier. (9) tag `beast` → beast. (10) tags `undead` and `mindless` → undead. (11) faction `fac.neutral` → civilian. (12) humanoid. Hounds are named explicitly by their templates.

### 4.3 Dark vision and blinding

`dv(o) = 1000 × (2 + floor_div(SGT_o, 20)) + profile.dv_mm + RowFlat(31) terms` in mm (Canon §6.4 row 31; kin beastkin, `nightborn`, `afflict.hunger` and doc 08 night-eye add to it). Within `dv(o)` the observer reads `L_eff = max(L_s, 8)`: darkness stops protecting up close. Example: SGT 40 gives 4000 mm; an undead with SGT 40 sees at light 8 out to 10000 mm.

`st.blinded` (Canon §11.1): sight only within 2000 mm. On the tick it is applied, every pair at `aware.alert` whose last contact was sight moves to `aware.lost` (§6.2 T4 with no wait), with `lkp` the subject's current position. `noise.explosion.flash` and doc 05's flash grenade apply it through nerve buildup.

### 4.4 Hearing thresholds and ambient masking

```
thr(o)  = profile.hear_db − floor_div(d(HRG_o), 5) − RowFlat(33) + 30 × [o has st.deafened] + resp_db(o)
amb(o)  = AMBIENT_DB[local_genre(o.pos)] + 10 × [weather front active in o's room]
gate    = max(thr(o), amb(o) + 3)
```

`resp_db` is doc 08's respirator penalty (cloth 0, mask 3, rebreather 5). `AMBIENT_DB` is in §15.1; a noise below `amb + 3` is inaudible (Splinter Cell rule, R5 §2.4). Example: a humanoid with HRG 40 in Steampunk (ambient 40) has `thr = 20` and `gate = 43`; in The Unwritten (ambient 0) `gate = 20`.

Genre `sound_bp` (doc 12 §2.1) scales path attenuation. `sys.hearing` reconstructs each `HeardNoise` record's path cost and rescales it, using the local genre of the noise position:

```
L0    = 10 × source_db;  cost = L0 − 10 × heard_db
L_eff = floor_div(L0 − mul_bp(cost, sound_bp), 10)
```

Example: a 45 dB step heard at 25 dB in Hardboiled Noir (13000): `cost = 200`, `L_eff = floor_div(450 − 260, 10) = 19`. In The Redaction (6000): `floor_div(450 − 120, 10) = 33`. Noises doc 03 already dropped (heard ≤ 0) stay inaudible.

### 4.5 Scent trails

Every living, non-ghost Erratum, minion and NPC with the scented flag (every creature except `construct`, `elemental`, `editor` tags) lays a **scent mark** every 30 ticks on ground: `ScentMark { cell: Gidx of its feet cell, tick, owner, s0 }`. `s0 = 10000`, × 5000 with `st.soaked`, × 15000 for kin undead; no mark while the feet cell holds ≥ 1 liquid unit or the body is airborne, swimming or mounted (the mount lays its own). The ring holds 4096 marks; the oldest is evicted. Strength at `now` is `s(m) = s0 − floor_div(s0 × (now − m.tick), 3600)`; a weather front halves the lifetime to 1800 ticks.

Scent observers (`scent` flag) on each check evaluate marks within 4000 mm of their origin, owned by a hostile, with `s(m) ≥ 1000`. The freshest such mark (largest `tick`, ties lower `owner`, then lower `cell`) sets the pair for its owner to at least `aware.suspicious` with `lkp` the mark cell and `lkp_src = 4`, and resets that pair's decay delay. Following marks one by one is the trail; sight takes over when it passes. Example: a mark laid at tick 1000 with `s0 = 10000` reads 5000 at tick 2800 and drops below 1000 after tick 4240; a soaked target's mark (`s0 = 5000`) is lost after tick 3880. Swimming across a canal breaks the trail.

### 4.6 Tremor, breath and thermal

- **Tremor** (doc 13). For each footstep or landing noise of a grounded emitter with source level ≥ 25 dB, every tremor observer within 12000 mm (origin to origin) receives a hearing record with `L_rx = L − floor_div(2 × dist_mm, 1000)`, `exact = true`, ignoring walls, ambient masking and `sound_bp`. It is processed as hearing (§5.3). A crouch-walk on grass (25 − 6 = 19 dB) never registers.
- **Breath** (doc 13, the jiangshi rule). Each check, every hostile within 6000 mm that breathes counts as a near-zone sight pass with `V = 10000`, ignoring walls and light. Breathless entities (kin construct and undead, tags `construct`, `undead`, `elemental`) and an Erratum holding its breath are undetectable. **Holding breath** (`input.hold_breath`, doc 17) drains `res.breath` at 1000 per second like submersion (doc 04) and ends when breath reaches 0 or the input is released.
- **Thermal** (doc 13) ignores gas concealment in §3.2. It also reads `L_eff = max(L_s, 8)` at any distance for targets that are not undead, construct or elemental.

### 4.7 Marking, mind vision and seeing the invisible

- **`st.marked`** (Canon §11.1, owner 09). While target `t` bears `st.marked` applied by source `S`, every observer allied with `S` (same faction, or the party for Errata sources) treats `t` as a near-zone sight pass with `f_frac = 10000`, `f_conc = 10000` and `f_zone = 10000` at any distance up to its `R_far`, and sets `lkp` to `t`'s exact position each check (`lkp_src = 5`). Errata see marked entities through walls (doc 17 renders the outline).
- **Mind vision** (doc 08). The bearer sees every creature within the radius through walls. In the simulation this counts as a sight pass for the doc 10 witness check and for reveal meters (§4.8) with `f_frac = 10000`.
- **Seeing invisible**: Awareness 75 within 4000 mm and the Divination hook within 12000 mm set `f_invis = 10000` (§3.1).

### 4.8 Errata as observers

Errata have no detection meter toward hostiles: the player sees the rendered world. Two simulation queries exist.

- **Reveal meters.** For each NPC that is stealthy (profile flag) or bears `st.invisible`, each Erratum within 48000 mm keeps `RevealMeter { meter, acc, last_gain, revealed }` in `StealthWorld.reveal`, updated on the Erratum's own perception phase (period 3). `gain = mul_bp(mul_bp(2500, V(e, n)), opp(AW_e, SR_n))` with `V` from `aware.profile.erratum`. At 10000, `revealed = true` (the client draws the full model and a marker instead of the shimmer) and the Erratum earns doc 04's 2000 Awareness milli-XP once per NPC. It stays revealed while the meter is ≥ 5000; the meter decays 1500 per second after 60 ticks without gain. An NPC is **hidden from the party** when it is stealthy or invisible and revealed to no Erratum; Awareness 100 applies `st.marked` for that Erratum to such NPCs within 6000 mm (doc 04).
- **Witness sight pass** (doc 10 §5.3): `erratum_sees(e, n)` is true when `V(e, n) ≥ 500` with the erratum profile, computed on demand on the tick of the event (3 rays).

---

## 5. The detection meter

Every observer keeps one `PairAware` per subject it has perceived (at most 16, §6.8). `M ∈ 0..=10000` measures how sure the observer is that the subject is there and hostile.

### 5.1 Opposed ratings: Awareness against Sneak

```
SR(t) = skill.sneak                                  if t is an Erratum
      = min(100, sneak_base + floor_div(3 × (d − 1), 2)) if t is an NPC with sneak_base > 0, else 0
PR(o) = skill.awareness                              if o is an Erratum
      = min(100, aw_base + floor_div(3 × (d − 1), 2))   if o is an NPC or device
opp(o, t)    = clamp(10000 + 100 × (PR(o) − SR(t)), 3000, 20000)        // Bp, sight and reveal fill
opp_db(o, t) = clamp(floor_div(SR(t) − PR(o), 10), −5, 8)                // dB added to the hearing gate for body noises
```

The depth term matches Canon §13.8's sub-attribute growth, so NPC vigilance keeps pace with Sneak growth (Canon §7.2). Examples: a Folio-4 humanoid has `PR = 20 + 4 = 24`; against Sneak 30, `opp = 9400` and `opp_db = 0`; against Sneak 0, `opp = 12400` and `opp_db = −3`. A Folio-25 humanoid (`PR = 56`) against Sneak 75: `opp = 8100`, `opp_db = 1`. Sneak 100 against a Folio-1 sentry (`PR = 30`): `opp = 3000` (clamped) and `opp_db = 7`.

### 5.2 Sight fill

On each check, for each candidate subject with a sight pass:

```
gain = profile.fill → × V(o, t) → × opp(o, t) → × STATE_MULT[state] → × (12500 if now < wary_until)
M    = min(10000, M + gain);  if gain > 0 { last_gain = now; last_contact = now; lkp = t.pos; lkp_src = 0 }
STATE_MULT: unaware 10000, curious 12500, suspicious 15000, searching 15000, lost 20000
```

Cameras use a sight timer, wards, lasers and tattlers trip (§15.2); disguised subjects in front of their outfit's faction use suspicion instead (§11.2); civilians fill only through crimes (§15.2). An alert pair keeps contact only when `V ≥ ALERT_KEEP_V = 300`.

### 5.3 Hearing fill

`sys.hearing` handles each `HeardNoise` record for a perceiving listener (doc 03 §11.5) after the §4.4 rescale:

```
gate   = max(thr(o), amb(o) + 3) + (opp_db(o, emitter) if the noise is a body noise of the emitter, §7.1)
if L_eff < gate: inaudible; stop
margin = L_eff − gate
gain   = clamp((margin + 1) × 250, 250, 5000) → × CLASS_FILL → × STATE_MULT[state] → × wary 12500
CLASS_FILL: noise.step.*, noise.move.* 4000; every other noise 10000
cap    = 10000 if profile.hear_alerts else max(M, 7499)                    // sound alone never alerts a sighted observer
M      = min(cap, M + gain); last_gain = now
```

An attributed noise (§7.7) feeds the emitter's pair. An unattributed noise feeds the disturbance (§6.4) with the same gain and no `opp_db`. Allied noises feed nothing except through callouts (§6.6).

### 5.4 Contact

Processed first in `sys.hearing`, from this tick's `ev.damage_dealt` where `target` perceives and `attacker` is an entity:

```
if pair(target, attacker).state == alert: last_contact = now; lkp = attacker.pos
elif dist(target, attacker) ≤ 3000 mm or pair had SIGHT_PASS on the latest check:  → alert, M = 10000
else: → lost, M = max(M, 9000), lkp = attacker.pos, quantized to 4000 mm when dist > 20000 mm (§6.5), lkp_src = 3
```

A hit from an unseen attacker beyond 3000 mm therefore leaves the target lost: still below alert, so the next hit is still a sneak attack (Canon §10.5 step 2.4), but the target now runs at the shot's origin. Damage with no entity attacker (hazards, falls) sets the disturbance to suspicious at the target's position. Doc 03 §5.14 push contact between a perceiver and a hostile body adds 5000 to `M` with exact `lkp` and forces at least suspicious.

### 5.5 Decay

After `DECAY_DELAY = 120` ticks without gain for a pair, each check removes `q` from `M` by the Canon §2.3 accumulator with `k = 20` at rate `R` per second: unaware and curious 1500, suspicious 1000, searching 750. Lost and alert do not decay; they run on timers (§6.2). Against a subject with Sneak ≥ 75 the rate is × 15000 (doc 04 Sneak 75: "after line of sight breaks").

Example: a curious pair at `M = 4000`, last gain at tick 1000, first decays on the first check at or after tick 1120; it removes 75 per check, so `M` falls below 1000 after 41 checks (123 ticks) at 925. Against Sneak 75 it removes 112 or 113 per check (accumulator) and falls below 1000 after 27 checks (81 ticks) at 963.

### 5.6 Worked timelines

Static geometry, checks every 3 ticks, state multipliers applied as states change:

| Case | `V` | `opp` | Curious at | Suspicious at | Alert at |
|---|---|---|---|---|---|
| V1: Shade crouch-walks 6 m in front of a Folio-4 guard, dark | 471 | 9400 | check 23 (tick 69), gain 109 | check 42 (tick 126) | check 73 (tick 219) |
| V3: heavy armor walks 20 m in the far cone, bright, Sneak 0 | 976 | 12400 | check 9 (tick 27) | check 16 (tick 48) | check 27 (tick 81) |
| Lean peek, `L` 10, 7 m, near cone, `opp` 10000 | 1608 | 10000 | check 7 (tick 21) | check 12 (tick 36) | check 20 (tick 60) |

In V1 the guard turns suspicious at tick 126 (2.1 s) and walks over; a Shade who breaks the near cone before check 42 leaves it curious and decaying.

---

## 6. Awareness state machine

### 6.1 States

| Ordinal | ID | Meaning | Behavior that doc 13 executes | `ai.input.target_aware` |
|---|---|---|---|---|
| 0 | `aware.unaware` | no knowledge | idle, patrol | 0 |
| 1 | `aware.curious` | noticed something | stops, turns its head to `lkp`, barks once (`noise.voice.bark`, sapient) | 2500 |
| 2 | `aware.suspicious` | investigating | walks to `lkp` with weapon ready; never attacks | 5000 |
| 3 | `aware.searching` | searching | runs the §6.7 search plan | 6000 |
| 4 | `aware.lost` | lost a target it was fighting | runs to `lkp`, attacks on reacquisition | 8000 |
| 5 | `aware.alert` | combat | doc 13 combat | 10000 |

"At or above `aware.suspicious`" means ordinal ≥ 2; "below alert" means ordinal ≤ 4. Doc 13's interim names map as `aware.investigating` → `aware.suspicious` and `aware.returning` → `aware.unaware` with Vigilance (Canon gaps G2).

### 6.2 Transitions

| # | From | To | Condition | Entry effect |
|---|---|---|---|---|
| T1 | unaware | curious | `M ≥ 2500` | bark; head turn |
| T2 | unaware, curious | suspicious | `M ≥ 5000`; or an investigate- or alarm-class attributed noise with margin ≥ 10 dB; or a scent mark; or a callout (§6.6); or push contact | `M = max(M, 5000)`; doc 13 investigates `lkp` |
| T3 | any of 0–4 | alert | `M` reaches 10000; or §5.4 contact; or disguise compromise (§11.4) | `M = 10000`; `alert_since = now` (doc 13 reaction delay); callout (§6.6); `folio_alerted = run_alerted = true` if the subject is an Erratum |
| T4 | alert | lost | `now − last_contact ≥ LOST_AFTER (90)`; immediately when the observer gains `st.blinded` | `lkp` = predicted position (§6.5); `M = 9000` |
| T5 | lost | alert | a sight pass with `V ≥ 300`; an attributed noise with margin ≥ 10; §5.4 contact | as T3, no new callout |
| T6 | lost | searching | within 1500 mm of `lkp`; or `now − state_since ≥ LOST_MAX (600)` | build the search plan |
| T7 | suspicious | searching | within 1500 mm of `lkp` and 120 ticks of look-around elapsed (`SUSP_LOOK`); or `now − state_since ≥ SUSP_MAX (900)` | build the search plan |
| T8 | searching | unaware | `now − state_since ≥ search_ticks` (1200; 1800 after an alarm) and `M < 2500` | `wary_until = now + 5400`; doc 13 returns to post |
| T9 | searching | suspicious | a new stimulus whose position is more than 6000 mm from the plan center | new `lkp`; a stimulus within 6000 mm instead updates `lkp` and restarts the plan |
| T10 | curious | unaware | `M < 1000` and `now − last_gain ≥ CURIOUS_HOLD (240)` | none |
| T11 | any | (pair removed) | subject dead, despawned, a ghost, out of the Folio, no longer hostile and not disguised, or pacified | `ev.awareness_changed` to unaware |
| T12 | any | unaware, `M = 0` for every pair and the disturbance | observer gains `st.sleeping` (doc 06 §14), is subdued (§9.4) or hacked (§13.5) | — |

Each check evaluates a pair in this order: removal (T11), sight fill, T3, T5, T4, T2, T1, decay, T6, T7, T8, T10. `sys.hearing` evaluates contact, hearing fill, T3, T5, T2, T1, T9. A state change emits `ev.awareness_changed { observer, subject, from, to }` immediately, with `subject = 0` for the disturbance. A pair that reached a state stays there until a listed exit fires: hysteresis comes from the thresholds being crossed one way and the exits being timers or lower thresholds.

### 6.3 Timer constants

| Constant | Ticks | Constant | Ticks |
|---|---|---|---|
| `DECAY_DELAY` | 120 | `LOST_AFTER` | 90 |
| `CURIOUS_HOLD` | 240 | `LOST_MAX` | 600 |
| `SUSP_LOOK` | 120 | `SEARCH_TICKS` | 1200 (1800 after an alarm) |
| `SUSP_MAX` | 900 | `WARY_TICKS` (Vigilance) | 5400 |
| `CALLOUT_COOLDOWN` | 300 | `RADIO_DELAY` | 30 |
| `ALARM_COOLDOWN` | 600 per source | `BARK_COOLDOWN` | 300 |

Vigilance (MGS Caution, R5 §9.2) multiplies every gain by 12500 while `now < wary_until`.

### 6.4 Disturbances

The disturbance is the observer's awareness of a place rather than of an entity: unattributed noises (thrown objects, impacts, explosions, alarms with no known subject), light changes (§2.4), discovered bodies (§10.4) and environmental damage. It uses ordinals 0–3, the same thresholds (T1, T2), the same hearing gain without `opp_db`, and exits T7, T8 and T10 with its own position. It never reaches lost or alert. An observer's overall posture is the highest ordinal among its pairs and its disturbance; doc 13 acts on the highest, ties broken by the newest `lkp_tick`, then the lower subject `EntityId`, with the disturbance as subject 0.

### 6.5 Last known position

| Source | `lkp` | `lkp_src` |
|---|---|---|
| sight pass | the subject's exact origin; `lkp_vel` = its velocity | 0 |
| attributed noise, margin ≥ 10 dB | the noise position | 1 |
| attributed noise, margin < 10 dB | `quantize(pos)`: x and z floored to a multiple of 4000 mm plus 2000, y unchanged | 1 |
| callout | the payload's `lkp` | 2 |
| damage | the attacker's origin; quantized as above when farther than 20000 mm and not sighted | 3 |
| scent | the mark cell's center | 4 |
| `st.marked` | exact origin every check | 5 |

`lkp` only moves to a stimulus at least as new as `lkp_tick`. On T4 the predicted position is `lkp + lkp_vel × 60 ticks` (one second along the last seen velocity), cut at the first cell where `raycast_blocks` from `lkp` hits. Example: last seen at (40.0, 9.0, 30.0) m moving at (4.3, 0, 0) m/s with a wall at x = 43: the prediction is 4300 mm east, cut at the wall face, x = 43.0 m.

Doc 13's blackboard copies `lkp`, `lkp_tick` and `lkp_src` each think (doc 13 §4.2).

### 6.6 Group alert propagation

- **Callout.** On T3, a sapient observer emits `noise.voice.callout` (85 dB) at its eye, unless its squad (doc 13 §4.10), or without a squad any same-faction NPC within 32000 mm, called out the same subject within `CALLOUT_COOLDOWN`. The payload `Callout { emitter, subject, lkp, tick }` rides with the noise request. Doc 13's `ai.action.call_allies` re-emits it under the same cooldown.
- **Hearing a callout.** A listener of the emitter's faction, or of a faction friendly or allied to it (Canon §15), that hears the callout above its gate: a pair for the subject below suspicious becomes suspicious with `M = max(M, 7000)` and the payload `lkp`; a pair at suspicious, searching or lost takes the newer `lkp` and restarts its state timer; alert is unchanged. Every such listener sets `wary_until = now + 5400`.
- **Radio.** When the emitter's faction carries the doc 13 `radio` flag and the local TEC at the emitter is ≥ 5, every member of its squad receives the callout at `now + 30` without a hearing check, and each receiver emits `noise.alarm.radio` (50 dB) at itself, which Errata can hear.
- **Allied combat noise.** An alarm-class noise (gunfire, explosion, scream) emitted by an allied NPC that holds some subject at alert is processed by allied listeners as a callout about that subject.
- **Alarms.** `raise_alarm(faction, pos, subject, source)` does nothing while `alarms[source] > now`; otherwise it sets the cooldown, emits the source's alarm noise, and for every awake member of the faction and of friendly or allied factions within 48000 mm of `pos`: `wary_until = now + 5400`; the pair for `subject`, when known and below searching, becomes searching at `pos` with `search_ticks = 1800`; with no subject, the disturbance becomes searching at `pos`. Doc 13's Director receives `alarm_raised(pos, faction)`. Sources: cameras, lasers, wards, tattlers, `hazard.alarm`, a hound's bark, a civilian's scream, a guard ringing a bell (`noise.alarm.bell`, a 60-tick doc 13 action at a bell or klaxon prop).
- **Bodies** (§10.4) start a disturbance search and a body callout (`noise.voice.callout` with subject 0); listeners set their disturbance to suspicious at the body.

Example: guard A at tick 5000 goes alert on Erratum E and calls out at 85 dB. Guard B 25 m away down a corridor hears it at 85 − 50 = 35 dB, above its Mythic Fantasy gate of 23, so B's pair for E becomes suspicious at A's `lkp` with `M = 7000` and Vigilance until tick 10400. B's further sight gains are × 15000 × 12500, so a glimpse of E at `V = 300` (gain `2500 × 300 × 10000 × 15000 × 12500` = 140 per check) alerts B within 22 checks.

### 6.7 Search patterns

On entry to searching (T6, T7, alarms) or a restart (T9), the observer builds a plan around center `c = lkp`:

```
P0     = c                                                     // always first, never drawn
P1     = c + unit(horizontal lkp_vel) × 6000 mm   if |lkp_vel| > 300 mm/s
RING   = for k in 0..8, for r in [4000, 8000]: c + (−sin_bam(k × 8192) × r, 0, −cos_bam(k × 8192) × r)
HIDES  = enclosed or under hiding spots (§10.1) within 10000 mm of c, ascending distance then key, at most 4
snap   = nearest walkable cell (floor with 2 air cells above) within 2000 mm horizontally and ±2 blocks,
         ascending (dist2, Gidx); drop points with none, unreachable by doc 13 paths, or duplicated
weight = 1, +2 if effective light ≤ 4, +1 if 5..=9, +3 for P1, +2 for a hiding spot
```

Doc 13's `ai.action.search` visits P0, then draws 3 points without replacement with its keyed `rng.ai` (seq 1) from the list in the order P1, RING (k, then r), HIDES, using these weights. At each point the searcher looks around for 60 ticks (4 yaw steps of 16384 Bam, 15 ticks each); at a hiding spot it performs the 60-tick check of §10.2. **Squad split:** when `n ≥ 2` squad members search centers within 2000 mm of each other for the same subject, the member with rank `i` (ascending `EntityId`) keeps P0 only if `i = 0`, ring points with `k mod n = i mod n`, and hiding spots with list index `mod n = i mod n`.

Example: center (100.5, 8.0, 60.5) m, last velocity east. P1 is (106.5, 8, 60.5). Ring point k = 0, r = 4000 is (100.5, 8, 56.5) (yaw 0 faces −Z); k = 2 (yaw 16384, facing −X), r = 8000 is (92.5, 8, 60.5). A dark ring cell has weight 3, P1 weight 4, a closet weight 3.

### 6.8 Pair bookkeeping and queries

- A pair is created on the first gain or stimulus. When 16 pairs exist, a new subject evicts the lowest of (ordinal, `M`, `last_gain`, then highest subject id) among unaware pairs; when none is unaware, the new subject is ignored this check.
- `aware_state(o, s) -> Id32`: the pair's state ID, `aware.unaware` when no pair exists. Doc 05 calls it `aware_of(target, attacker)`.
- `sneak_eligible(att, tgt, now) = tgt has st.sleeping ∨ (tgt has comp.perception ∧ ordinal(aware_state(tgt, att)) < 5)` (Canon §10.5 step 2.4). Entities without perception (Errata, invaders, containers) are eligible only while sleeping, so NPCs never sneak-attack Errata (doc 13 §5.4).
- `sight_pass(o, s)`: the `SIGHT_PASS` flag of the latest check.
- `awareness_above_unaware(e)` for doc 03 §15.2: any pair toward an Erratum or the disturbance has ordinal ≥ 1.
- **`st.hidden`** (Canon §11.1): at the end of `sys.hearing`, for each Erratum in ascending `EntityId`, apply it when the Erratum is not mounted, not a ghost, and no observer holds it at alert; remove it otherwise. It never gates sneak attacks.

```rust
fn check_pair(o: &mut Observer, p: &mut PairAware, t: &Entity, now: Tick) {
    if removed(o, t) { emit_change(o, p, AWARE_UNAWARE); drop(p); return; }        // T11
    let v = if o.senses & SIGHT != 0 { visibility(o, t) } else { 0 };
    p.flags.set(SIGHT_PASS, v >= 1);
    if v >= 1 && !disguise_path(o, t) && !camera(o) {
        let g = chain(o.profile.fill, &[v, opp(o, t), STATE_MULT[p.state], wary(o, now)]);
        if g > 0 { p.meter = min(10000, p.meter + g); p.last_gain = now; touch_lkp(p, t, SRC_SIGHT); }
    }
    if p.state == ALERT { if v >= 300 { p.last_contact = now; } }
    if p.meter >= 10000 && p.state < ALERT { set(o, p, ALERT, now); }                        // T3
    else if p.state == LOST && v >= 300 { set(o, p, ALERT, now); }                            // T5
    else if p.state == ALERT && now - p.last_contact >= 90 { set(o, p, LOST, now); p.meter = 9000; predict(p); } // T4
    else if p.state <= CURIOUS && p.meter >= 5000 { set(o, p, SUSPICIOUS, now); }            // T2
    else if p.state == UNAWARE && p.meter >= 2500 { set(o, p, CURIOUS, now); }              // T1
    decay(p, now, t);                                                                         // §5.5
    timers(o, p, now);                                                                        // T6, T7, T8, T10
}
```

---

## 7. Noise

### 7.1 Level formula

Every emitted noise has an ID (`noise.*`) and a source level in whole dB (Canon §1.2 `Db`):

```
fn noise_level(id, emitter, ctx) -> Db:
    L = BASE[id](ctx)                                  // §7.2–§7.6; ctx = gait, surface, fall_mm, archetype, mass, acoustic class
    if id is a body noise: L += armor_db(emitter)      // step, land, dodge, mantle, climb, swim
    L += Σ add_db(emitter, id)                         // the modifier table, in ascending source content-ID order
    L  = min(L, Σ caps)                                // every applicable cap, smallest wins
    max(0, L)
armor_db(e) = Σ worn pieces max(0, noise_db − floor_div(skill.armor, 25)), and 0 at Armor 100   // doc 05 §11.4, doc 04
```

**Body noises** are `noise.step.*`, `noise.move.*`, `noise.voice.*` and `noise.item.use`; only they take `opp_db` at the listener (§5.3).

| Modifier | Noises | dB |
|---|---|---|
| armor_db | body noises listed above | full set: light 3, medium 6, heavy 11 at Armor 0 |
| Sneak ≥ 25 (doc 04) | `noise.step.crouch`, `noise.step.crawl` | −3 |
| `talent.shade.soft_step` | steps, landing | −6 |
| `trait.kin.fae.light_step` | steps, landing | −3 |
| `affix.soft_step` (doc 11) | steps, landing | −X |
| Silverbough armor (any piece) | steps, landing | −6 once |
| `st.encumbered` / `st.overburdened` | steps, landing | +3 / +6 |
| `affix.quiet` | attack noises: `noise.melee.swing`, `noise.shot.*` | −X |
| Silverbough weapon | attack noises | −20 |
| `curse.loud` (doc 10) | every noise the bearer emits | +m |
| `st.erasure` (Canon §11.6) | every noise the bearer emits | +stacks |
| Midnight Avenue weapon (cap) | shots ≤ 50, swings ≤ 40 | cap |
| `talent.shade.quiet_kill`, takedowns (cap) | the victim's death cry and body fall | ≤ 24 (audible to 2 m) |

### 7.2 Movement and footsteps

| ID | Base (dB) | Cadence |
|---|---|---|
| `noise.step.crouch` | 25 + surface | doc 03 stride 500 mm |
| `noise.step.walk` | 40 + surface | stride 700 mm |
| `noise.step.sprint` | 55 + surface | stride 900 mm |
| `noise.step.crawl` | 20 + surface | stride 500 mm while downed |
| `noise.move.land` | `40 + floor_div(5 × fall_mm, 1000)` + surface | `ev.entity_landed` (fall ≥ 300 mm) |
| `noise.move.swim` | 35 | every 60 ticks swimming |
| `noise.move.climb` | 30 | every 30 ticks climbing |
| `noise.move.mantle` | 35 | each mantle |
| `noise.move.dodge` | 45 | each dodge |
| `noise.move.splash` | 50 | entering liquid with `|v.y| ≥ 3000 mm/s` |
| `noise.move.flap` | 35 | every 30 ticks for fliers |

NPC steps add the profile `step_db` (§4.1). **Surface** comes from the acoustic class of the ground block's material, overridden by `noise.surface.water` when the feet cell holds ≥ 1 liquid unit:

| Class | dB | Canon materials (doc 07 field `acoustic` overrides) |
|---|---|---|
| `noise.surface.metal` | +10 | steel, moonsilver, silver, star_metal, plasteel, titanium, scrap_iron, iron, cast_iron, hexed_silver, brass, bronze, copper, aetherium, cold_iron, orichalcum, redacted_alloy, gold_leaf |
| `noise.surface.glass` | +8 | glass, rad_glass, psi_crystal |
| `noise.surface.water` | +8 | any liquid ≥ 1 unit in the feet cell |
| `noise.surface.stone` | +6 | marble, jade, obsidian, every stone block |
| `noise.surface.ice` | +4 | `block.ice` |
| `noise.surface.bone` | +4 | bone, dragonbone, chitin |
| `noise.surface.wood` | +3 | oak, blackwood, elderwood |
| `noise.surface.synthetic` | +2 | polymer, carbon_fiber |
| `noise.surface.mud` | +2 | mud blocks |
| `noise.surface.dirt` | 0 | dirt, gravel, default |
| `noise.surface.sand` | −2 | sand |
| `noise.surface.soft` | −4 | flesh, leather, rubber |
| `noise.surface.foliage` | −4 | leaves, hay |
| `noise.surface.grass` | −6 | grass, moss blocks with grass |
| `noise.surface.paper` | −6 | vellum, paper |
| `noise.surface.cloth` | −8 | carpet, tweed, silk, nanoweave, dreamstuff |
| `noise.surface.snow` | −8 | snow layers |

Examples: walking on metal in a full heavy set at Armor 0 with `curse.loud` 2: `40 + 10 + 11 + 2 = 63` dB. A Shade crouch-walking on stone in a light set with Sneak 25 and `soft_step`: `25 + 6 + 3 − 3 − 6 = 25` dB. A 4000 mm drop onto stone in a medium set: `40 + 20 + 6 + 6 = 72` dB.

### 7.3 Weapons

Melee: `noise.melee.swing` 30 (light), 35 (heavy), +5 for two-handed archetypes; `noise.melee.hit` 55, 60 when blunt is the largest share; `noise.melee.clang` 70 (doc 05 §2.3); `noise.melee.block` 65; `noise.melee.parry` 70.

Ranged weapons emit one noise per shot; a shotgun's pellets share one noise, and a beam emits one noise every 30 ticks while firing. **Powered** skins have a non-empty dependency set (Canon §5.3, guns, railguns, lightning staves); **mundane** skins have none (bows, slings, crossbows). A **suppressor** is a weapon part with tag `suppressor` (doc 07); it applies to powered skins only and lowers the muzzle flash from 12 to 4.

| Archetype (doc 05) | `noise.shot` powered | `noise.shot.suppressed` | `noise.shot.mundane` | Flash |
|---|---|---|---|---|
| pistol | 120 | 60 | 45 | 12 / 4 / 0 |
| pistol_heavy | 130 | 68 | 50 | 12 / 4 / 0 |
| smg | 120 | 62 | 45 | 12 / 4 / 0 |
| rifle_auto | 130 | 70 | 50 | 12 / 4 / 0 |
| rifle_marksman | 135 | 72 | 45 | 12 / 4 / 0 |
| shotgun | 135 | 85 | 55 | 12 / 4 / 0 |
| sniper | 145 | 78 | 45 | 12 / 4 / 0 |
| launcher | 125 | — | 70 | 12 / — / 0 |
| beam (`noise.shot.energy`) | 70 | 50 | 50 | 10 / 4 / 0 |
| charge_rifle | 95 | 60 | 35 | 10 / 4 / 0 |
| crossbow | 70 | 50 | 40 | 6 / 4 / 0 |
| thrown | — | — | 30 | 0 |

Other weapon noises: `noise.impact.projectile` at the impact cell, 50 for bullets and bolts, 45 for arrows and thrown weapons; `noise.weapon.reload` 35; `noise.weapon.dryfire` 30 (doc 05); `noise.weapon.jam` 40 (doc 05); `noise.weapon.swap` 30; `noise.weapon.throw` 25.

Example: an unsuppressed pistol (120 dB) 30 m down a corridor in Mythic Fantasy arrives at 60 dB against a gate of 23: margin 37, gain 5000, alarm class (§7.7). Suppressed (60 dB) it is inaudible at 30 m; at 15 m it arrives at 30 dB: margin 7, gain 2000, investigate class but below T2's 10 dB margin, so the listener needs a second shot to turn curious.

### 7.4 Spells and prayers

Grimoire spells take the form's cast noise from doc 06 §5.2 with doc 06's hushed and register riders; doc 09 supplies the ID `noise.spell.<school>` of the spell's school (doc 06 §7.1). Wands, texts and NPC spells with no form noise use the school default below. Impacts use doc 06's `50 + 10 × essences`.

| ID | Default (dB) | Class | ID | Default (dB) | Class |
|---|---|---|---|---|---|
| `noise.spell.evocation` | 60 | investigate | `noise.spell.restoration` | 40 | curious |
| `noise.spell.abjuration` | 45 | curious | `noise.spell.divination` | 30 | curious |
| `noise.spell.alteration` | 40 | curious | `noise.spell.impact` | 50 + 10 × essences | investigate |
| `noise.spell.conjuration` | 55 | investigate | `noise.spell.fizzle` | 70 (doc 06) | investigate |
| `noise.spell.illusion` | 30 | curious | `noise.spell.surge` | 90 (doc 06) | alarm |
| `noise.prayer` | 50 | curious | | | |

### 7.5 World, locks, items and bodies

| ID | dB | ID | dB |
|---|---|---|---|
| `noise.block.break` | by acoustic class: metal 85, glass 80, stone 75, ice 70, bone, wood and synthetic 65, dirt, sand and mud 45, soft 40, foliage and grass 35, cloth, paper and snow 30 | `noise.block.hit` | break − 15 |
| `noise.block.place` | 40 (placeables, Canon §4.3) | `noise.block.fall` | break − 5 |
| `noise.block.erase` | 50 (Editor erasure) | `noise.node.harvest` | 50 (resource nodes) |
| `noise.door.open` | 45 wood, 55 metal; −15 when crouched | `noise.door.close` | open + 5; −15 when crouched |
| `noise.door.locked` | 40 (rattling a locked door) | `noise.lock.pick` | 25 per attempt |
| `noise.lock.snap` | 35 (a pick breaks) | `noise.lock.open` | 40 |
| `noise.lock.force` | 75 (Force check, doc 04) | `noise.hack.beep` | 30 per attempt |
| `noise.hack.fail` | 60 at the device | `noise.light.snuff` | 20 |
| `noise.light.switch` | 25 | `noise.light.power` | 45 |
| `noise.item.drop` | 35 | `noise.item.use` | 30 (drink, eat, read) |
| `noise.item.impact` | `40 + min(30, floor_div(mass_g, 100))` + surface | `noise.item.shatter_glass`, `noise.item.shatter_ceramic` | 70, 75 (doc 08) |
| `noise.body.fall` | 55 (death collapse) | `noise.body.drop` | 50 |
| `noise.body.thud` | 65 (thrown body) | `noise.hide.enter`, `noise.hide.exit` | 35 |
| `noise.takedown.lethal` | 35 | `noise.takedown.subdue` | 30 |

### 7.6 Voices, explosions, alarms, gadgets, mounts and environment

| ID | dB | ID | dB |
|---|---|---|---|
| `noise.voice.shout` | 85 (Errata emote) | `noise.voice.whistle` | 55 (Errata lure) |
| `noise.voice.callout` | 85 | `noise.voice.bark` | 50 |
| `noise.voice.scream` | 95 | `noise.voice.pain` | 60 |
| `noise.voice.death` | 70 | `noise.voice.breath` | 30 (doc 13 stalkers) |
| `noise.voice.groan` | 40 (downed, every 120 ticks) | `noise.voice.dog_bark` | 90 |
| `noise.voice.squawk` | 70 (tattlers) | `noise.explosion` | 150 (doc 03 powder keg) |
| `noise.explosion.grenade` | 140 | `noise.explosion.concussion` | 150 |
| `noise.explosion.flash` | 160 | `noise.explosion.incendiary` | 110 |
| `noise.explosion.smoke` | 55 | `noise.explosion.surface` | 100 (liquid blasts, Canon §12.1) |
| `noise.alarm` | 90 | `noise.alarm.bell` | 100 |
| `noise.alarm.radio` | 50 | `noise.trap.trigger` | 60 |
| `noise.gadget.deploy` | 45 | `noise.gadget.noisemaker` | 70 every 60 ticks for 600 ticks |
| `noise.gadget.hum` | 40 every 60 ticks (drones, turrets idle) | `noise.mount.walk` | 50 |
| `noise.mount.gallop` | 65 | `noise.mount.hover` | 60 every 30 ticks |
| `noise.mount.glide` | 30 | `noise.mount.summon` | 60 |
| `noise.env.distant` | 70 (doc 12 event) | `noise.env.debris` | 80 |
| `noise.env.steam` | 50 | `noise.env.fire` | 45 on `ev.fire_started` |

Doc 05 grenades map: frag `noise.explosion.grenade`, concussion `.concussion`, flash `.flash`, incendiary `.incendiary`, smoke `.smoke`, counter `.grenade`. Doc 03 reads levels through `noise_level` for `ExplosionSpec.noise`.

### 7.7 Attribution and AI reactions

A noise is **attributed** to its emitter when the emitter is a living entity and the ID is a body noise, a weapon-use noise (`noise.melee.swing`, `noise.shot.*`, `noise.weapon.*`), a cast (`noise.spell.<school>`, `noise.prayer`), or one of `noise.lock.*`, `noise.hack.*`, `noise.door.*`, `noise.hide.*`, `noise.takedown.*`, `noise.mount.*`, `noise.light.*`, `noise.item.drop`, `noise.body.drop`. Every other noise is unattributed: impacts, explosions, alarms, gadgets, traps, block breaks, items landing, environment. A grenade thrower is never revealed by its explosion; the guards search the blast.

| Class (`aware.react.*`) | Noises | Hostile attributed emitter | Unattributed | Allied emitter |
|---|---|---|---|---|
| `aware.react.ignore` | `noise.voice.bark`, `noise.gadget.hum`, `noise.mount.glide` | none | none | none |
| `aware.react.curious` | steps, swim, climb, mantle, dodge, flap, swing, reload, swap, dryfire, throw, item drop and use, lock pick and beep, light snuff and switch, door, hide, takedowns, breath, groan, quiet casts (≤ 45 dB defaults) | pair gain | disturbance gain | none |
| `aware.react.investigate` | landing ≥ 60 dB, splash, melee hit, clang, block, parry, impacts, item impact and shatter, block hit, break, place, fall, erase, harvest, lock snap, open and force, hack fail, light power, suppressed and mundane shots, energy shots, loud casts, spell impact, fizzle, pain, whistle, shout, squawk, trap trigger, deploy, noisemaker, smoke burst, mount noises, steam, fire, distant | pair gain; T2 when margin ≥ 10 | disturbance gain; T2 when margin ≥ 10 | none |
| `aware.react.alarm` | powered unsuppressed shots, explosions (except smoke), surge, scream, death cry, dog bark, alarms, debris | pair gain; T2 always; doc 13 runs instead of walks | disturbance becomes searching (1800 ticks) | callout rule if the ally is alert (§6.6) |
| `aware.react.callout` | `noise.voice.callout` | — | — | §6.6 |

### 7.8 Genre sound and ambient

`sound_bp` (doc 12 §2.1) rescales path cost (§4.4) and `AMBIENT_DB` (§15.1) masks quiet noises. A full heavy set walking on metal (63 dB) heard 12 m down a straight corridor arrives at 39 dB. In Mythic Fantasy (`sound_bp` 10000, ambient 20) the gate is `max(20, 23) − 2 = 21` for a listener with `PR = 24` against Sneak 10 (`opp_db = floor_div(−14, 10) = −2`): margin 18, gain `clamp(19 × 250, 250, 5000) = 4750`, × 4000 = 1900 per step, so the guard is curious after 2 steps and suspicious after 3. In Cyberpunk (`sound_bp` 12000, ambient 35) the same step arrives at `L_eff = floor_div(630 − mul_bp(240, 12000), 10) = 34` against a gate of `38 − 2 = 36`: inaudible.

### 7.9 Event conversion

`convert_events()` runs in the `sys.sound` prelude over this tick's events in `seq` order and appends requests through `emit_noise` (doc 03 §11.1):

| Event | Noise |
|---|---|
| `ev.footstep { entity, gait, material }` | `noise.step.<gait>` at the entity origin |
| `ev.entity_landed { entity, fall_mm, material }` | `noise.move.land` |
| `ev.block_changed` cause `broken`, `fell`, `placed`, `toggled` (doors), `erased` | `noise.block.break`, `.fall`, `.place`, `noise.door.open` or `.close`, `.erase` at the cell center; explosions and doc 03 causes with their own noise emit nothing here |
| `ev.entity_killed` of a creature tagged sapient or beast | `noise.voice.death` at the victim's eye and `noise.body.fall` at its origin, both capped at 24 dB after a takedown or with the killer's `quiet_kill` |
| `ev.fire_started` | `noise.env.fire` at the cell |

Weapon, spell, item, lock and voice noises are emitted directly by the acting system with `noise_level`. Doc 03 then propagates every request in order and hands `HeardNoise` records to `sys.hearing`.

---

## 8. Player feedback

The host sends each player, in its doc 16 snapshot, `StealthHud { gem: u16, hidden: bool, rings: Vec<(Fx3, u32 radius_mm, u8 class)>, marks: Vec<(EntityId, u8 state, u16 meter, u8 flags)>, steps: Vec<Fx3 dir> }` for its own Erratum. Doc 17 renders it.

### 8.1 Visibility meter (light gem)

`gem = mul_bp(X_ref, LIGHT_BP[L_s])`, where `X_ref` is §3.1's exposure with `f_invis = 1000` when invisible and without the observer-specific `ghost_signal` term. It shows as five pips: `< 500` none (unseen), `< 1500` one, `< 3000` two, `< 5000` three, `< 8000` four, else five, plus the `st.hidden` eye icon. Example: the V1 Shade has `X_ref = 2772` and `gem = mul_bp(2772, 1700) = 471`: no pips. The same Shade with a lantern (`L_s` 13): `mul_bp(2772, 8900) = 2467`, two pips.

### 8.2 Noise ring

Every noise the player's Erratum emits draws a ring for 30 ticks at the free-field radius where a reference humanoid hears it (Mark of the Ninja rule, R5 §9.2):

```
gate_ref = max(20, AMBIENT_DB[local genre] + 3)
radius_mm = floor_div(max(0, L − gate_ref) × 5_000_000, sound_bp)
```

The 2 dB per meter matches doc 03's 20 ddB face-step cost. Examples: walking on stone in a medium set (52 dB) in Mythic Fantasy: `(52 − 23) × 500 = 14500` mm. The 63 dB heavy walk in Cyberpunk: `floor_div(25 × 5_000_000, 12000) = 10416` mm. Ring color follows the reaction class; allies' rings show at half opacity.

### 8.3 Per-enemy detection indicators

- **Baseline.** For each hostile within 48000 mm whose pair toward the player gained meter in the last 20 ticks, an arc on a ring around the crosshair points toward it, its length `meter / 10000` of a quarter circle, colored by state: white curious, yellow suspicious, orange searching or lost, red alert. A red "!" shows over an observer's head for 60 ticks after T3.
- **Divination 50** (doc 04): every hostile within 30000 mm shows its state icon toward you, through walls.
- **`item.trinket.warning_bell`** (doc 11): positions of hostiles at or above suspicious toward you within 15000 mm, through walls.
- **Footstep indicators** (Canon §6.4 row 34): a directional tick for each hostile footstep heard within `(8 + floor_div(HRG, 10))` m, from `HeardNoise.arrival_dir`; none under `st.deafened`.
- **Co-op.** A red marker over an ally while any hostile holds it at alert; allies' arcs are not shown.

---

## 9. Sneak attacks and takedowns

### 9.1 Eligibility

`sneak_eligible` (§6.8) is evaluated per (attacker, target) on the impact tick (Canon §10.5 step 2.4). Other observers' states never matter: while guard G1 holds Erratum A at alert, A's hit on guard G2, whose pair for A is unaware, is still a sneak attack and draws no crit roll (Canon §21 test 38). Hits that enter the pipeline after step 2 (DoT, shock chains, environmental hazards, falls, Canon §10.6) never take the multiplier. A sneak hit sets `DmgFlags` bit 1.

### 9.2 Multiplier

```
TIER_BP = { 1: 30000, 2: 22500, 3: 15000, 4: 12500 }
sneak_mult(att, tgt, tier) =
    m = TIER_BP[tier]
    if att is an Erratum: m += 20 × skill.sneak + 5000 × [skill.sneak ≥ 50] + 5000 × [talent.shade.backstab]
    if tgt.rank ∈ {rank.boss, rank.editor, rank.hunter}: m = min(m, 15000)
    m
```

| Attack | Tier |
|---|---|
| melee and ranged archetypes | doc 05 `sneak_tier`: 1 for dagger, blade_short, fist, unarmed; 2 for blade_long, axe, mace, spear, whip, chain, pistol, pistol_heavy, rifle_marksman, sniper, charge_rifle, crossbow, thrown; 3 for blade_great, axe_great, warhammer, halberd, quarterstaff, smg, rifle_auto, shotgun, launcher, beam |
| spells: bolt, beam, chain, orb, cone | 3 |
| spells: touch | 2 |
| spells: nova, aura, field, wall, rune; throwables, bombs, flasks; gadgets, turrets, traps; hits from a mounted rider | 4 |
| minions and hacked units | their basic attack: melee 2, ranged 3 |

`affix.ambusher` and `item.exotic.quietus` add to the step-1 additive bucket on sneak hits (doc 11) and do not change `sneak_mult`. The multiplier enters Canon §10.5 at step 2.4, after precision and before elevation and flanking, and the hit draws no crit roll (Canon §3.4).

### 9.3 `ignore_armor`

A sneak hit carries `ignore_armor` (Canon §10.5 step 5) when the attacker has Sneak 100 or `talent.shade.unseen_blade`, and the target's rank is not boss, editor or hunter.

### 9.4 Takedowns

**Eligibility** (evaluated every tick while the prompt shows): attacker is an Erratum, grounded, not mounted, not carrying; target has `comp.perception`, rank minion or standard (elite: lethal only, attacker Sneak ≥ 75), not tagged `swarm`; origins within 1500 mm; attacker outside the target's frontal cone (`ang_diff(target.yaw, bearing target→attacker) > 10923`, Canon step 2.4b) unless the target has `st.sleeping`, `st.stunned` or `st.knocked_down`, or the attacker is inside an enclosed hiding spot facing it (§10.2); the target's pair for the attacker is at most suspicious.

| Kind | Input | Ticks | Result |
|---|---|---|---|
| Lethal | hold `input.heavy` 10 ticks | 45 | the target takes a hit with flags `sneak` and `takedown` that enters Canon §10.5 at step 7 as `dmg.true` equal to its current HP plus its remaining ward; an elite takes `mul_bp(max_hp, 5000)` instead and gains `st.staggered`; `noise.takedown.lethal` 35 dB; the death cry and body fall are capped at 24 dB |
| Subdue | `input.interact` | 90 | `st.sleeping` by direct application (Canon §11.4) and `subdued_until = now + 3600`; the sleep is re-applied directly on every status pulse until then; `noise.takedown.subdue` 30 dB |

During either, the target cannot move (`st.grappled` applied directly for the duration) and the attacker cannot act. The takedown cancels with no effect when either takes damage, when the target's pair for the attacker becomes alert, or when the attacker moves more than 500 mm. Observers with a sight pass on the attacker during the takedown gain 4000 meter at once. A subdued NPC wakes early on any damage (Canon `st.sleeping`) or after an ally's 60-tick wake interact, then enters searching at its own position with Vigilance. A subdue counts once per entity as a pacification defeat for doc 13 §11.1 XP. A lethal takedown kill grants doc 04's 1500 Sneak milli-XP.

### 9.5 Bosses

Ranks boss, editor and hunter: multiplier capped at 15000, no `ignore_armor`, no takedowns, no pickpocketing, immune to disguises. A boss inside its sealed arena and every hunter hold every Erratum at alert (doc 13 omniscience), so neither can be sneak-attacked after the fight starts; the opening hit on a mini-boss before its arena seals can be. Champions take full multipliers but no takedowns.

### 9.6 Worked examples

- **S1.** Doc 05's E3 dagger heavy (D1 = 25,775, flanking) by a Shade with Sneak 60 and `backstab`: `sneak_mult = 30000 + 1200 + 5000 + 5000 = 41200`. D2 = `mul_bp(mul_bp(25775, 41200), 11500) = 122,121`. Split pierce 73,273, slash 48,848; against AR 10 the DR is 233 and 486 Bp; **total 118,040 mHP**. With Sneak 0 the same hit is doc 05's 84,899.
- **S2.** Sneak 100 with `backstab`: `sneak_mult = 42000`, D2 = 124,493. Against AR 40 with armor the hit would be 109,355 (DR 912 and 1672); with `ignore_armor` it stays **124,493 mHP**.
- **S3.** The same Shade against a mini-boss before the arena seals: `min(42000, 15000) = 15000`, no `ignore_armor`.
- **S4.** A Gunslinger (Sneak 10) sniper shot, tier 2: `22500 + 200 = 22700`.

---

## 10. Hiding spots, bodies and corpse discovery

### 10.1 Hiding spot kinds

| Kind | Where | Effect |
|---|---|---|
| `aware.hide.enclosed` | closets, lockers, wardrobes, barrels, hay carts, dumpsters, sarcophagi: doc 07 props or placeables with `hide: enclosed`; capacity 1 Erratum or 2 bodies | `n = 0` for every observer except the one checking the spot |
| `aware.hide.under` | beds, tables, carts with a gap ≥ 600 mm: crouch under | `f_frac` capped at 4000; stance 3500 |
| `aware.hide.foliage` | crouch-only conceal cells: tall grass, bushes, reeds | §3.2 conceal sums |
| `aware.hide.water` | eye submerged | stance 3000 plus 2 conceal per water block (Canon §12.2) |
| `aware.hide.ledge` | standing at least 2500 mm above an observer's eye | the `θ_v ≤ 10923` cone limit excludes steep angles |
| `aware.hide.shadow` | effective light ≤ 4 | §2.2, §3.3 posture rule |
| `aware.hide.crowd` | moving at walk or slower within a crowd | §15.2 |

A mounted Erratum cannot enter any hiding spot (Canon §17.7). `HideState { kind, occupant: Option<EntityId>, bodies: [Option<EntityId>; 2] }` lives in `StealthWorld.hides`, keyed by the prop's `Gidx` or `EntityId`.

### 10.2 Enclosed spots

Entering takes 30 ticks of `input.interact` within 1500 mm and emits `noise.hide.enter`; the occupant's origin snaps to the spot center. Exiting takes 20 ticks and emits `noise.hide.exit`. The occupant sees out through the slats and may lean, but no ray reaches it. From inside, it may take down an eligible target within 1500 mm in front of the spot regardless of the target's facing ("closet grab"), in 60 ticks.

**Checking.** A searcher whose plan holds the spot, or a suspicious observer whose `lkp` lies within 1500 mm of it, opens it: 60 ticks, `noise.door.open`. An occupant is found: its pair becomes alert (`M = 10000`) and it gains `st.staggered` directly. Bodies inside are discovered (§10.4). A hound adjacent to an occupied spot for 60 ticks barks an alarm (§15.2).

### 10.3 Carrying and disposing of bodies

Corpses (`grid.ekind.corpse`), subdued NPCs and Errata Remains are picked up with `input.interact` (30 ticks). The body's mass (template mass, 75,000 g for humanoids and Remains) joins `carried_g`, so encumbrance applies; the carrier cannot attack or cast, takes the 15000 exposure term (§3.1), and drops with `input.interact` (20 ticks, `noise.body.drop`, 35 dB when crouched) or throws with `input.throw` (`noise.body.thud`). Disposal: an enclosed spot with a free body slot; liquid at least 2 blocks deep, where the body sinks and has `n = 0` for observers whose eye is out of the liquid; fire, which consumes a corpse after 600 ticks, lava after 60. Subdued NPCs drowned or burned die normally.

### 10.4 Corpse discovery

Each check, a sapient, hound or civilian observer evaluates the nearest undiscovered body within `R_far` that belonged to its own or a friendly or allied faction (civilians: any humanoid body), plus subdued allies. Its exposure is `X = 10000 → × 3500 → × 6000 → × f_size` and `V` follows §3.4 with the posture rule. `body_meter += mul_bp(profile.fill, V)`; it resets when the candidate changes or after 60 ticks without a sight pass. At 5000 the body is discovered:

- `discovered.insert(body)`; the observer's disturbance becomes searching at the body (1800 ticks) and it emits a body callout (`noise.voice.callout`, subject 0); civilians scream instead (`noise.voice.scream`, alarm class).
- Allies who hear set their disturbance to suspicious at the body and gain Vigilance; the doc 13 Director may order a bell or alarm (§6.6).
- A subdued body is woken (60 ticks) before the search starts.
- A cell holding at least 1000 ml of `liq.blood` in sight within `R_near` gives a suspicious disturbance at that cell, once per cell.

Examples: a corpse at 6 m in a humanoid's near cone at light 9: `X = 2100`, `V = mul_bp(2100, 5900) = 1239`, gain 309 per check, discovered on check 17 (51 ticks). At 14 m in the far cone at light 11: `V = 232`, gain 58, discovered on check 87 (261 ticks). At light 7 or darker in the far zone it is never discovered from there.

---

## 11. Disguises

### 11.1 Outfits and masks

An **outfit** is a `slot.body` item whose instance carries `outfit_of: fac.*` (doc 11 instance field). Sources: stripping a sapient humanoid corpse or subdued NPC of faction F (`input.interact`, 120 ticks; the item copies the template's body armor archetype), faction lockers and shops. A **mask** is any `item.face.mask` in `slot.face` (Canon §13.2); respirators are not masks (doc 08 §4.5).

| Tier | Requirement | Suspicion factor |
|---|---|---|
| `disguise.tier.full` | outfit of F and a mask | 10000 |
| `disguise.tier.partial` | outfit of F, no mask | 20000 |
| `disguise.tier.none` | anything else | — |

A disguise of F is **active** toward observer `o` when `o` is of F or of a faction friendly or allied to F, `o`'s profile does not ignore disguises (hounds, editors, bosses), the wearer is not mounted, and `(wearer, F) ∉ blown`. While active, `o` treats the wearer as a member of F for every stance check, so it gets no sight fill; every other faction treats the wearer by its own stance toward F, so a rival gang attacks a disguised Erratum.

### 11.2 Suspicion

**Enforcers** are observers of rank elite or higher, role guardian, or tagged `enforcer` by doc 13; for a partial disguise every observer within 3000 mm also counts. Each enforcer check with a sight pass:

```
V_d    = V(o, t) computed with f_stance = 10000 and f_camo = 10000      // crouching is a trigger, not cover
s_gain = 200 → × V_d → × row40(t) → × TIER_FACTOR → × wary 12500                 // Canon §6.4 row 40, doc 04 fair_folk
S      = min(10000, S + s_gain)
```

Non-enforcers gain only from triggers. `S` decays 1000 per second after 120 ticks without gain (accumulator `k = 20`). No enforcer fill happens while `now < satisfied_until`.

### 11.3 Triggers

Each trigger adds its amount on every check of every F observer with a sight pass on the wearer while it holds (one-shot triggers once per occurrence):

| ID | Condition | Amount |
|---|---|---|
| `disguise.trigger.weapon_raised` | attack windup, ADS, heavy charge or aim | +4000 per check |
| `disguise.trigger.trespass` | inside a doc 12 room tagged `room.restricted` of F | +1500 per check |
| `disguise.trigger.climb` | climbing or mantling | +1500 per check |
| `disguise.trigger.sprint` | gait sprint | +1000 per check |
| `disguise.trigger.company` | an undisguised Erratum within 3000 mm is also visible | +1000 per check |
| `disguise.trigger.crouch` | crouched and moving | +500 per check |
| `disguise.trigger.body` | carrying a body | +5000 per check |
| `disguise.trigger.crime` | lockpicking, hacking, a failed pickpocket, a takedown or snuffing a light | +6000 once |
| `disguise.trigger.attack` | any attack that hits anything | `S = 10000` |
| `disguise.trigger.strip` | changing outfit or mask | `S = 10000` |

Cameras of F never fill suspicion; a trigger they see adds 60 to their sight timer at once (§15.2).

### 11.4 Bluff and compromise

When `S ≥ 5000` a sapient observer walks to the wearer and challenges it (`noise.voice.bark`). Within 120 ticks the wearer may answer with `input.interact`: a doc 04 Deceive check (GUI + Speech) against `DC = 40 + 10 × bluffs(o)`, drawn on doc 04's keyed `rng.check`. Success: `S = 0`, `satisfied_until = now + 1800`, `bluffs += 1`. Failure or silence: compromise. **Compromise** also happens when `S` reaches 10000: the pair becomes alert (`M = 10000`, T3) and its callout carries `disguise_blown`; the observer and every F member that hears the callout add `(wearer, F)` to `blown` for the rest of the Folio.

### 11.5 Worked example

A full disguise (medium-class outfit, `f_armor` 11000), GUI 55 (`row40 = clamp(10000 − 1500, 3000, 15000) = 8500`), walking 5 m from an enforcer at light 12: `V_d = mul_bp(11000, 8200) = 9020`, `s_gain = 200 → 180 → 153` per check. The challenge comes at check 33 (99 ticks) and compromise at check 66 (198 ticks). Without a mask the gain is 306: challenge at check 17 (51 ticks), compromise at check 33 (99 ticks). Stepping into a dim alcove at light 5 cuts `V_d` to 3080 and the full-disguise gain to 51.

---

## 12. Distractions

| Distraction | Mechanism | Noise | Reaction |
|---|---|---|---|
| Thrown object | any item thrown with `input.throw` (doc 05 §9) lands | `noise.item.impact` or a shatter noise | unattributed; investigate class: a disturbance at the landing cell |
| Noisemaker | doc 07 gadget | `noise.gadget.noisemaker` every 60 ticks for 600 ticks | investigate |
| Whistle | `input.whistle` emote (doc 17), cooldown 120 ticks | `noise.voice.whistle` 55 dB | attributed investigate: pair gain, and suspicious toward the whistler's position when the margin is ≥ 10 dB |
| Shout | `input.shout` emote, cooldown 300 ticks | `noise.voice.shout` 85 dB | attributed investigate |
| Phantasm | doc 06 `summon.phantasm` | taunt and death noise per doc 06 | observers create pairs toward the decoy as a hostile |
| Food lure | a thrown `cat.food` item | item impact | beasts and hounds within 12000 mm with a sight pass or 4000 mm by scent set a disturbance at it, walk there and eat for 300 ticks, ignoring scent meanwhile |
| Light change | §2.4 | snuff, switch, power | suspicious disturbance at the fixture |
| Alarm panel test | hacking an alarm panel with the "test" option (§13.4) | its alarm noise | `raise_alarm` at the panel with no subject |

**Engineered noise is stealth play** (R7 R-BA4): a kill of an NPC whose highest-ordinal stimulus is a disturbance created within the last 600 ticks by the killer's own thrown item, noisemaker or alarm test, and whose pair for the killer is below alert, grants 750 Sneak milli-XP in addition to doc 04's events (doc 04 interface).

---

## 13. Security

### 13.1 Locks

A lock is genre-neutral; its skin decides the tool. Worldgen (doc 12) gives a lock the electronic skin when local TEC ≥ 5 and the mechanical skin otherwise, so Canon §5.7's "no electronic locks at TEC ≤ 2" always holds.

| Tier | DC (doc 04 ladder) | `lock_hp` | Pick ticks | Hack ticks | Placed at depth |
|---|---|---|---|---|---|
| `lock.t1` | 25 Easy | 20 | 60 | 80 | 1–5 |
| `lock.t2` | 40 Standard | 24 | 75 | 100 | 6–10 |
| `lock.t3` | 55 Hard | 28 | 90 | 120 | 11–15 |
| `lock.t4` | 70 Severe | 32 | 105 | 140 | 16–20 |
| `lock.t5` | 85 Heroic | 36 | 120 | 160 | 21–25 |
| `lock.t6` | 100 Legendary | 40 | 135 | 180 | 26+ |

Tier is `clamp(1 + floor_div(d − 1, 5), 1, 6)`, +1 for vaults, capped at 6. Kinds: `lock.kind.door` (`lock_hp` as listed), `lock.kind.container` (`lock_hp × 2`, R7 chest rule), `lock.kind.vault` (no pity), `lock.kind.quest` (key only, never picked). Skins: `lock.skin.mechanical` (Security, `item.tool.lockpicks`) and `lock.skin.electronic` (Hacking, the hacking deck, dependency techne). `LockState { tier, kind, skin, locked, progress: i32, key_tag: u32, fails: u8 }`. A matching key (`cat.key` with the same `key_tag`, doc 11) opens it in 20 ticks with no roll. Doc 03 asks `door_locked(gidx)` before toggling a door; a locked door refuses and emits `noise.door.locked`.

```json
{ "id": "lock.t3", "dc": 55, "lock_hp": 28, "pick_ticks": 90, "hack_ticks": 120,
  "kinds": { "lock.kind.door": 10000, "lock.kind.container": 20000, "lock.kind.vault": 0 } }
```

### 13.2 Lockpicking

```
your_tier = 1 + floor_div(skill.security, 20), +1 at Security 75 (doc 04)        // attempt only if tier ≤ your_tier
rating    = skill.security + floor_div(d(SLT), 2) + TOOL_Q[lockpicks quality]
chance    = clamp(5000 + (rating − DC) × 200, 500, 9500)
ticks     = PICK_TICKS[tier] → × 8000 with talent.shade.light_fingers
draws     = keyed rng.stealth, seq 1000: success roll, then break roll (always 2)
success   → unlocked; noise.lock.open; doc 04 Security XP 150 × DC
failure   → progress += PITY[band]; unlocked at progress ≥ lock_hp (vaults excepted); noise.lock.pick
break     → roll < break_bp: lockpicks durability −1 (doc 07), noise.lock.snap; break_bp = 2500, × 5000 at Security 25
TOOL_Q    = awful −10, poor −5, normal 0, good 3, excellent 6, masterwork 10, peerless 15
PITY      = Security 0–19: 3; 20–59: 4; 60–99: 6; 100: 8               // Barony lock health, R7 R-BA2
```

Security 100 opens non-quest mechanical locks in 60 ticks with no picks and no draws. Every attempt emits `noise.lock.pick` (25 dB). **Example:** Security 45, SLT 50, normal picks on a `lock.t3` door: `your_tier = 3`, rating 50, chance 4000, 90 ticks per attempt, break chance 1250, pity 4 per failure, so the 7th failure (28 progress) opens it at the latest.

### 13.3 Optional minigame

A per-player setting `lock_mode` (default `auto`) switches to `manual`. A manual attempt sets `2 + tier` pins in order. At start it draws one `range(40)` per pin (seq 1004) as the pin targets. For each pin a cursor runs over positions `0..=39`, one per tick from the pin's first tick, wrapping. Pressing `input.attack` sets the pin when `|cursor − target| ≤ W`, with `W = clamp(1 + floor_div(rating − DC + 40, 10), 1, 8)`. A miss, or 120 ticks without a press, draws one break roll (seq 1005, in miss order) and restarts that pin; the third miss fails the attempt (pity progress as in auto). Setting every pin unlocks with no success roll. Draws: `2 + tier` plus at most 3. Commands come from the replay log, so the minigame is deterministic. Example: rating 50 against DC 55 gives `W = 4`.

### 13.4 Electronic locks and devices (TECHNE skins)

Hacking replaces Security on electronic skins. The deck is a gadget component with dependency techne: its `TOOL_Q` bonus is multiplied by its Contradiction potency, and doc 07 rolls its failure spark (Canon §5.5).

```
your_tier = 1 + floor_div(skill.hacking, 20)
rating    = skill.hacking + floor_div(d(LOG), 2) + mul_bp(TOOL_Q[deck quality], potency)
chance    = clamp(5000 + (rating − DC) × 200, 500, 9500);  draws: seq 1001, 1 draw
ticks     = HACK_TICKS[tier] → × (10000 − 50 × skill.hacking) → × 8000 trait.origin.wired → × 7500 talent.netrunner.backdoor
range     = 2500 mm; cameras 10000 mm at Hacking 25; 15000 mm with talent.netrunner.remote_access
failure   → pity progress; noise.hack.fail at the device; the third consecutive failure on one device raises its faction's alarm
```

| Device | Success |
|---|---|
| electronic lock | unlocked |
| camera (`hazard.camera`) | looped for 1800 ticks: it sees nothing; until Folio exit at Hacking 100 |
| turret | faction `fac.errata` for 1800 ticks (Folio exit at Hacking 100); friendly fire applies |
| alarm panel | disabled until Folio exit, or tested (§12) |
| laser emitter pair | disabled until Folio exit |
| fuse box | power cut for 1800 ticks (§2.4) |
| terminal | unlocks every electronic lock in its room; doc 04 `data_thief` |

Example: Hacking 40, LOG 55, normal deck, `lock.t3`: rating 47, chance 3400, ticks `120 → 96 → 76` with `wired`.

### 13.5 Hacking constructs (`st.hacked`)

Targets: `body.construct` or tag `electronic` (Canon §11.1, doc 13 §5.7), not boss, editor or hunter (Canon §11.4). The hacker channels within 8000 mm while its eye keeps a sight pass to the target: `ticks = mul_bp(120, 10000 − 50 × skill.hacking)` with the §13.4 multipliers; damage to either or a lost sight pass cancels it. Then one draw (seq 1001):

```
DC     = {minion 25, standard 40, elite 55, champion 70} + floor_div(d, 2)
rating = §13.4 rating + 10 × [target's pair for the hacker below suspicious] + 20 × [target has st.shocked or st.stunned]
success → st.hacked by direct application: mul_bp(360, clamp(floor_div(1_000_000, thr_mind), 2500, 15000)) × rank control (Canon §11.4)
          × 15000 with talent.netrunner.puppet_master; until Folio exit at Hacking 100 (non-boss)
failure → the target's pair for the hacker becomes alert
```

At most 1 hacked unit, +1 at Hacking 75, +1 with `talent.netrunner.botnet`; a new hack past the limit ends the oldest. Example: Hacking 50, LOG 50, an unaware standard drone in Folio 10: rating 65, DC 45, chance 9000, 90 ticks; its RSV is `40 + floor_div(27, 2) = 53`, mind threshold 126, so `st.hacked` lasts `mul_bp(360, 7936) = 285` ticks.

### 13.6 Traps: detection and disarming

Hidden objects are doc 12 hidden hazards and secrets, doc 11 mimics, doc 06 runes and §15.2 wards and tripwires. **Passive detection** is Canon §6.4 row 35: each Erratum within 3000 mm rolls once per object on ticks where `(t + observer_phase) % 60 == 0` (30 with `gumshoe`), `seq` = the object's index in ascending ID order among objects in range, one draw each. **Active search:** crouched and still for at least 30 ticks raises the radius to 5000 mm and the chance × 15000 before the 9000 clamp. Doc 06's sight essence reveals objects in its area without a roll. A success sets the object's party mask in `revealed` (the whole party sees it) and grants the detector doc 04's 3000 Awareness milli-XP. NPCs never detect Errata traps; `fac.editors` members detect them at 5000 Bp under the same cadence.

**Disarm** a revealed trap within 1500 mm: mechanical and arcane with Security (lockpicks or toolkit), electronic with Hacking (deck).

```
trap_dc = min(100, 25 + 2 × d)                    // Folio 1: 27, Folio 10: 45, Folio 25: 75
rating  = skill + floor_div(d(SLT or LOG), 2) + TOOL_Q
chance  = clamp(5000 + (rating − trap_dc) × 200, 500, 9500); ticks 60 (× 8000 light_fingers)
draws   = seq 1002: success roll, then trigger roll (always 2)
success → disarmed; at Security 50 it becomes a redeployable cat.gadget (doc 07); XP 150 × DC
failure → the trap fires on the disarmer when the trigger roll < 5000
```

Example: Security 30, SLT 45 against a Folio-6 dart trap: rating 32, DC 37, chance 4000.

### 13.7 Pickpocketing

Targets: sapient humanoid NPCs with inventory (doc 13 `carry`, picked-up weapons, keys), not boss ranks, not `in_combat`, with their pair for the thief at most curious; the thief within 1500 mm and outside the frontal cone (10923 Bam). A target with `st.sleeping` or subdued is robbed with no draw.

```
rating = skill.pickpocket + floor_div(d(SLT), 2) + 10 × [ang_diff > 16384, directly behind] + 5 × [L_s ≤ 4] + 10 × [in a crowd]
DC     = 20 + PR(target) + min(30, floor_div(mass_g, 250)) + 10 × [pair curious] + 15 × [equipped item; Pickpocket 75 only]
chance = clamp(5000 + (rating − DC) × 200, 500, 9500); draw seq 1003 at completion
ticks  = min(180, 60 + floor_div(mass_g, 100)) → × 7500 at Pickpocket 25 → × 8000 light_fingers
```

Success moves the item and grants doc 04 XP. Failure: the target's pair becomes alert (suspicious with `M = 7000` at Pickpocket 100); doc 13 applies its witnessed reputation rule; a `fac.neutral` vendor turns hostile for 600 ticks and screams; a disguise takes the crime trigger. **Planting** (Pickpocket 50) uses DC − 10; a primed throwable detonates on its fuse at the target's origin. Example: Pickpocket 40, SLT 45, directly behind in darkness, a 300 g pouch on a Folio-4 guard: rating 57, DC 45, chance 7400, 47 ticks.

---

## 14. Co-op stealth

1. **Per pair.** Meters, states, `lkp` and sneak eligibility are per (observer, Erratum). One Erratum being seen never makes another seen. When an observer turns alert on one Erratum, each of its pairs toward other Errata within `R_far` gains 2500 at once (it is scanning).
2. **Party-wide outcomes.** The Unseen bonus (Canon §8) and `run.note.silent` fail when any Erratum is ever held at alert. `revealed` hidden objects are party-wide.
3. **Light and noise are shared physics.** An ally's torch lights you (§2.1); allies see your noise rings at half opacity.
4. **Downed** Errata are perceived with stance 3500 and emit `noise.voice.groan` every 120 ticks; reviving is not silent (`noise.item.use` every 60 ticks).
5. **Ghosts** cannot be perceived (Canon §17.6). Doors and switches a ghost toggles make unattributed noise, so a ghost can draw guards away; its `st.marked` marks hostiles for the party.
6. **Hiding spots** hold one Erratum. **Disguises** are per Erratum; an undisguised ally nearby is a trigger.
7. **Minions** are perceived like Errata and leave scent. Their hits use their own pair eligibility (§9.2 tier by attack).
8. Pings and voice chat never enter the simulation; emotes (`whistle`, `shout`) do.

### 14.5 Invaders

An invader (Canon §17.9) is player-controlled `fac.editors` at rank hunter: it has no meters and cannot be sneak-attacked unless asleep. To deny wallhacks, the host sends the invader's client an Erratum's transform only while `V(invader, erratum) ≥ 300` with `aware.profile.invader` on the invader's own period-3 check, or while that Erratum's noise reached the invader with `L_eff ≥ 20` within the last 60 ticks (then quantized to 4000 mm), or while the Erratum bears `st.marked` from a `fac.editors` source. NPC hunters stay omniscient (doc 13 §10.5).

---

## 15. Genre stealth flavors

### 15.1 Genre table

| Genre | `AMBIENT_DB` | Camo primary, secondary | Signature stealth features |
|---|---|---|---|
| mythic_fantasy | 20 | foliage, earth | torch-bearing patrols, hounds, ward glyphs on vaults |
| gothic_horror | 15 | shadow, earth | ambient light 2, bell towers (`noise.alarm.bell`), undead with 10 m dark vision, hounds |
| cosmic_horror | 10 | shadow, earth | sound-hunting horrors, seeing lenses (cameras), silence |
| space_opera | 30 | urban, shadow | cameras, turrets, drones, radio squads, lasers |
| cyberpunk | 35 | neon, urban | cameras, lasers, drones, crowds, radio, electronic locks |
| post_apocalypse | 20 | earth, urban | dogs, can tripwires, scavenger camps |
| weird_west | 25 | earth, pale | dogs, shotgun tripwires, townsfolk crowds |
| hardboiled_noir | 30 | shadow, urban | rain fronts, crowds, cameras, mob outfits for disguise |
| wuxia | 20 | foliage, earth | rooftop ledges (VERVE jumps), market crowds, sect sentries |
| steampunk | 40 | urban, earth | machinery masking, automatons to hack, klaxons, crowds |
| fairy_tale | 20 | foliage, pale | tattler birds, wolves (hounds), fairy-ring wards |
| mythic_antiquity | 25 | pale, earth | ambient light 7, oracle wards, temple hounds |
| oneiric | 15 | neon, pale | sleepwalker crowds, lucid wards |
| redacted | 30 | pale, urban | compliance cameras, lasers, drones, ambient light 9, agent outfits |
| unwritten | 0 | pale, pale | absolute silence (`sound_bp` 5000), no disguises against Editors |
| ur_text | 20 | earth, pale | each room uses its wedge genre's row (doc 12 §20.3) |

### 15.2 Devices and special observers

- **Cameras** (`hazard.camera`, doc 12: half-angle 8192, 16000 mm, sweep ±4096 per 360 ticks). `p = (now + phase) mod 360`; `yaw = base + (p < 180 ? −4096 + floor_div(8192 × p, 180) : 4096 − floor_div(8192 × (p − 180), 180))`, so p 0 → −4096, 90 → 0, 180 → +4096, 359 → −4050. Each check with `V ≥ 800` on a hostile (no `opp`; `ghost_signal` applies), `cam_seen += 3`; otherwise `cam_seen = max(0, cam_seen − 3)`. At 60 (doc 12's 60 ticks of sight) the camera raises its faction's alarm at the target, resets to 0 and holds its yaw on the target for 300 ticks. Examples: standing still at light 10, `V = 4020`: alarm in 60 ticks. Crouched still at light 4, `V = 726`: never. Night optics (dark vision 8000 mm) at local TEC ≥ 8.
- **Lasers** (techne skin of `hazard.alarm`): a segment up to 8000 mm between two emitters at height 300, 900 or 1500 mm. Every 3 ticks each non-ghost body capsule is tested with doc 02 `closest_seg_seg`; a capsule within its radius trips the alarm with that subject. Invisibility does not help. A crouched body (1200 mm) passes under a 1500 mm beam; a 300 mm beam can be jumped. Errata see the beam where it crosses smoke or light ≤ 9. Hacking an emitter disables the pair.
- **Tripwires** (mundane skin): laser geometry, visible only once revealed (§13.6), disarmed with Security.
- **Wards** (arcana or mythos skin): any hostile body center within 4000 mm of the glyph trips the alarm (chime, `noise.alarm`), ignoring light, invisibility and disguises. Wards are hidden objects; Security disarms them at `trap_dc`, or a doc 06 unmaking hit dispels them.
- **Dogs** (`aware.profile.hound`): scent (§4.5), hearing threshold 10, disguise-proof. While any hound pair is at least suspicious, it emits `noise.voice.dog_bark` every 120 ticks and raises its faction's alarm (600-tick cooldown). Food lures distract them (§12).
- **Sound-hunting horrors** (`aware.profile.horror`): no sight; hearing can reach alert; tremor; every `lkp` exact. Standing still and silent is safe; a noisemaker sends them running.
- **Crowds**: three or more `fac.neutral` civilians within 3000 mm of an Erratum walking or slower with no weapon raised give `f_gear × 4000` against observers farther than 6000 mm, and +10 to pickpocketing. Civilians fill their meters only from crimes (the §11.3 crime, attack, body and weapon triggers, a discovered body, an alarm-class noise); at 10000 they scream and flee (doc 13).
- **Tattlers** (birds and critters, Fairy Tale and Mythic Fantasy): an Erratum entering 6000 mm that is not crouch-walking at ≤ 2000 mm/s or still makes it squawk (`noise.voice.squawk`, unattributed) and fly off, despawning after 120 ticks.
- **Turrets** use the turret profile and fire on alert. **Drones** project their spotlight: while a drone holds a sight pass on a target, that target's `L_s` is at least 12.

---

## 16. Worked example: a two-player heist

Cyberpunk, Folio 12 (locks and devices at tier 3, DC 55; guards `PR = 20 + 16 = 36`), ambient 35. Shade A (Sneak 60, Security 45) and Netrunner B (Hacking 50, LOG 50) approach a vault room watched by one camera and two guards.

1. B hacks the camera from 10 m (Hacking 25): device DC 55, rating 55, chance 5000. It succeeds; the camera loops for 1800 ticks.
2. A crouch-walks on metal in a light set: `25 + 10 + 3 − 3 = 35` dB. Guard G1 is 8 m away: doc 03 delivers 19 dB, the Cyberpunk rescale gives `floor_div(350 − mul_bp(160, 12000), 10) = 15`, below the gate `38 + 2 = 40` (`opp_db = floor_div(60 − 36, 10) = 2`): silent.
3. G2 notices A at the edge of its far cone; A ducks into a locker (§10.2). G2 turns suspicious, finds nothing at `lkp` after 120 ticks of look-around, and searches. The locker enters its plan as a hiding-spot candidate (weight 3); when G2's three `rng.ai` draws pick other points, G2 returns to unaware with Vigilance after 1200 ticks, and when they pick the locker, A is found (§10.2).
4. A takes G1 down lethally from behind (45 ticks; G1's pair for A is unaware). The death noises are capped at 24 dB. A carries G1 into the locker (2-body capacity).
5. A (SLT 50) picks the treasury door, a `lock.kind.door` at `lock.t3`: chance 4000 per 90-tick attempt, and pity opens it by the 7th failure at the latest. Nobody reached alert: both players earn the Unseen bonus, and doc 12's heist challenge succeeds.

---

## Exports

Namespaces owned by Canon §19: `noise.`, `aware.`, `disguise.`, `lock.`.

| Namespace | IDs |
|---|---|
| `aware.` states | `aware.unaware`, `aware.curious`, `aware.suspicious`, `aware.searching`, `aware.lost`, `aware.alert` |
| `aware.profile.` | `humanoid`, `sentry`, `stalker`, `beast`, `hound`, `undead`, `construct`, `flier`, `swarm`, `horror`, `civilian`, `boss`, `editor`, `camera`, `turret`, `drone`, `ward`, `laser`, `tattler`, `erratum`, `invader` |
| `aware.camo.` | `shadow`, `earth`, `foliage`, `urban`, `pale`, `neon` |
| `aware.hide.` | `enclosed`, `under`, `foliage`, `water`, `ledge`, `shadow`, `crowd` |
| `aware.react.` | `ignore`, `curious`, `investigate`, `alarm`, `callout` |
| `noise.step.` | `crouch`, `walk`, `sprint`, `crawl` |
| `noise.move.` | `land`, `swim`, `climb`, `mantle`, `dodge`, `splash`, `flap` |
| `noise.surface.` | `metal`, `glass`, `water`, `stone`, `ice`, `bone`, `wood`, `synthetic`, `mud`, `dirt`, `sand`, `soft`, `foliage`, `grass`, `paper`, `cloth`, `snow` |
| `noise.melee.` | `swing`, `hit`, `clang`, `block`, `parry` |
| `noise.shot` and `noise.shot.` | `noise.shot`, `noise.shot.suppressed`, `noise.shot.mundane`, `noise.shot.energy` |
| `noise.weapon.`, `noise.impact.` | `reload`, `dryfire`, `jam`, `swap`, `throw`; `noise.impact.projectile` |
| `noise.spell.`, `noise.prayer` | `evocation`, `abjuration`, `alteration`, `conjuration`, `illusion`, `restoration`, `divination`, `impact`, `fizzle`, `surge`; `noise.prayer` |
| `noise.block.`, `noise.node.` | `break`, `hit`, `place`, `fall`, `erase`; `noise.node.harvest` |
| `noise.door.`, `noise.lock.`, `noise.hack.`, `noise.light.` | `open`, `close`, `locked`; `pick`, `snap`, `open`, `force`; `beep`, `fail`; `snuff`, `switch`, `power` |
| `noise.item.`, `noise.body.`, `noise.hide.`, `noise.takedown.` | `drop`, `use`, `impact`, `shatter_glass`, `shatter_ceramic`; `fall`, `drop`, `thud`; `enter`, `exit`; `lethal`, `subdue` |
| `noise.voice.` | `shout`, `whistle`, `callout`, `bark`, `scream`, `pain`, `death`, `breath`, `groan`, `dog_bark`, `squawk` |
| `noise.explosion` and `noise.explosion.` | `noise.explosion`, `grenade`, `concussion`, `flash`, `incendiary`, `smoke`, `surface` |
| `noise.alarm` and `noise.alarm.` | `noise.alarm`, `bell`, `radio` |
| `noise.trap.`, `noise.gadget.`, `noise.mount.`, `noise.env.` | `trigger`; `deploy`, `noisemaker`, `hum`; `walk`, `gallop`, `hover`, `glide`, `summon`; `distant`, `debris`, `steam`, `fire` |
| `disguise.` | tiers `disguise.tier.full`, `.partial`, `.none`; triggers `disguise.trigger.weapon_raised`, `.trespass`, `.climb`, `.sprint`, `.company`, `.crouch`, `.body`, `.crime`, `.attack`, `.strip` |
| `lock.` | tiers `lock.t1` to `lock.t6`; kinds `lock.kind.door`, `.container`, `.vault`, `.quest`; skins `lock.skin.mechanical`, `.electronic` |

Functions and data: `stealth_light`, `noise_level`, `convert_events`, `aware_state` (doc 05's `aware_of`), `sneak_eligible`, `sneak_mult(att, tgt, tier)`, `grants_ignore_armor`, `sight_pass`, `erratum_sees`, `awareness_above_unaware`, `door_locked`, `raise_alarm`, `search_plan`, `StealthHud`, `PerceptionState`, `StealthWorld`. Behavior of `st.deafened`, `st.marked`, `st.invisible`, `st.hidden`, `st.hacked` (Canon §11.1 owner 09).

## Interface assumptions

| Doc | This chapter assumes |
|---|---|
| 02 | a save section `stealth` for `StealthWorld`; `closest_seg_seg` for lasers |
| 03 | a `sys.sound` prelude hook calling `convert_events()`; `door_locked(gidx)` consulted before door toggles; `cell_conceal` split into block, liquid and gas terms for thermal rays; push-contact pairs reported from §5.14; a cause `grid.cause.erased` for Editor erasure; `ExplosionSpec.noise` levels read through `noise_level` |
| 04 | the §5.2 XP events as written plus 750 Sneak milli-XP for engineered-noise kills (§12); Deceive on `rng.check`; Row 31, 33, 34, 35 flat terms; talent and trait IDs cited here |
| 05 | calls `sneak_mult(att, tgt, tier)` (three arguments, for the boss cap); emits shot, swing, impact, reload and jam noise through `noise_level`; a `suppressor` part tag; `st.grappled` direct for takedown duration; the `IGNORE_ARMOR` flag from `grants_ignore_armor` |
| 06 | form cast noise and school; a query "is this cell inside a shadow area"; `summon.phantasm` as a hostile-perceived decoy; the Divination see-invisible hook |
| 07 | material `acoustic`; block `lightsrc` (`flame`, `electric`, `arcane`) and lit/unlit states; prop field `hide`; crouch-only conceal flag; `block.light_switch`, `block.fuse_box`; lockpick and hacking-deck durability and quality; noisemaker gadget; redeployable disarmed traps |
| 08 | phosphor marking; respirator hearing penalty; mind vision radius; blood volume per cell |
| 11 | instance field `outfit_of`; `item.tool.hack_deck`; door keys in `cat.key` with `key_tag`; tool qualities |
| 12 | lock tiers and skins at placement; `room.restricted`; biome `camo`; `hazard.alarm` skins by dependency; laser heights; hiding props in stealth rooms; one fuse box per electric room |
| 13 | template field `perception`; tag `enforcer`; faction flag `radio`; template `outfit` flag and mass; investigate and search actions using §6.5 and §6.7; relight, bell and wake behaviors; reputation event `pickpocket_caught`; Director `alarm_raised`; subdue as a pacification defeat |
| 14 | `run.note.silent` and heist challenge read `run_alerted` and `folio_alerted` |
| 16 | `StealthHud` in each player's snapshot; the §14.5 invader interest filter |
| 17 | inputs `input.hold_breath`, `input.whistle`, `input.shout`; setting `lock_mode`; HUD rendering of §8 |
| 18 | text keys and audio for every `noise.*`; HUD strings |

## Canon gaps

- **G1. Version.** The contract fixes the header to canon-1.2; the canon on disk is canon-1.4. Everything here was checked against 1.4: placeables and destructible terrain (block noises apply to destructible blocks and placeables only; `noise.node.harvest` covers resource nodes), Fieldcraft, invasions (§14.5) and the 1.3 World Laws (no stealth effect).
- **G2. Awareness IDs.** Canon names only `aware.alert` and `aware.unaware`. Doc 13 used interim `aware.returning` and `aware.investigating`, doc 14 `aware.alerted`. This chapter, as namespace owner, defines six states and maps `investigating` → `aware.suspicious`, `returning` → `aware.unaware` with Vigilance, `alerted` → `aware.alert`.
- **G3. Keyed `seq`.** Canon §3.3 calls `seq` a per-actor counter and assigns detection the object index. To avoid key collisions this chapter uses fixed action slots 1000–1005 (§1.5), as doc 05 did for `rng.combat`.
- **G4. Takedown entry point.** Canon §10.6 has no execution entry. Lethal takedowns enter at step 7 as `dmg.true` equal to HP plus ward (no crit, armor or chip floor). Requested as a §10.6 row.
- **G5. Events.** Canon §18 lacks `ev.alarm_raised`, `ev.body_discovered`, `ev.disguise_blown`, `ev.lock_opened`, `ev.object_revealed`, `ev.device_hacked`, `ev.takedown`, `ev.item_stolen`. Assumed added as doc 03 bus events.
- **G6. Stealth light.** Canon §2.3 `light_at` reads the grid only. `stealth_light` layers own light, flashes, phosphor and shadow on top for stealth alone; composure and spawning keep Canon's `light_at`.
- **G7. `sound_bp`.** Doc 12 gives `sound_bp` to doc 09 while doc 03 propagates. The rescale happens in `sys.hearing`; noises doc 03 already dropped stay dropped, so a low `sound_bp` cannot extend range past doc 03's unscaled bound.
- **G8. Sneak tiers** for spells, area effects, minions and riders are not in the canon or doc 05; defined in §9.2.
- **G9. Checks on `rng.stealth`.** Lockpicking, hacking, disarming and pickpocketing use the doc 04 check curve but Canon §3.3's `rng.stealth`; bluffing is a doc 04 Deceive check on `rng.check` (doc 04 gap 1).
- **G10. Mounted.** Canon §17.7 bars `st.hidden` for riders; this chapter also bars hiding spots, disguises and takedowns.
- **G11. Tall grass.** Canon §12.2 says "only against crouched targets"; this chapter also applies it to downed targets and bodies.
- **G12. Ghost noises.** Canon §17.6 lets ghosts open doors; their door noises are unattributed (§14).

## Acceptance tests

1. **Stealth light.** Corridor light 2: with a torch `L_s = 12`; a level-12 flash 2 cells away gives 10 for 3 ticks; inside a shadow field 0, with the torch 12; phosphor gives 9.
2. **Light table.** `LIGHT_BP[0] = 800`, `[7] = 4300`, `[15] = 10000`.
3. **Size.** `f_size` is 10000 for 1800 × 300 mm, 3333 for 900 × 200 mm and 40000 for 3600 × 900 mm.
4. **V1.** The §3.4 Shade gives `X = 2772` and `V = 471`.
5. **V2.** The same target at 14000 mm in the far cone gives `V = 0`; standing at the same light gives `V > 0`.
6. **V3.** Heavy armor, light 12, 20000 mm, SGT 45: `f_dist = 4964`, `V = 976`.
7. **Concealment.** Two conceal-2 smoke cells plus 12 m of fog 1 give `f_conc = 6665`; a sum of 15 blocks the ray.
8. **Lean.** A still, leaning Erratum at light 10, 7 m in the near cone: `n = 1`, `V = 1608`.
9. **Opposed.** `opp(24, 30) = 9400`; `opp(30, 100) = 3000`; `opp(24, 0) = 12400`; `opp_db` for SR 10 against PR 24 is −2.
10. **Timeline.** V1 at `opp` 9400 turns curious on check 23, suspicious on check 42, alert on check 73; V3 at 12400 on checks 9, 16, 27.
11. **Decay.** `M = 4000` curious, no gain: first decay at the first check at or after 120 ticks; below 1000 after 41 checks at 925; against Sneak 75 after 27 checks at 963.
12. **Hearing.** 63 dB heard at 39 dB after 12 m: Mythic Fantasy gain 1900 per step for PR 24 against Sneak 10; Cyberpunk `L_eff = 34` against gate 36, inaudible.
13. **Rescale.** A 45 dB noise heard at 25 dB: Hardboiled Noir 19 dB, The Redaction 33 dB.
14. **Gate.** Humanoid, HRG 40, Steampunk: gate 43; deafened: 50 in The Unwritten.
15. **Shots.** Pistol 120 dB at 30 m, Mythic Fantasy: gain 5000, alarm class. Suppressed (60 dB): inaudible at 30 m; at 15 m gain 2000 and no T2.
16. **Sound cap.** Unlimited footsteps raise a humanoid pair to at most 7499; a horror pair reaches 10000 and alert.
17. **Unseen hit.** A hit from 40 m with no sight pass leaves the pair lost with `M ≥ 9000` and `lkp` quantized to a 4000 mm grid center; the next hit is still sneak-eligible.
18. **Lost.** An alert pair with no contact turns lost exactly 90 ticks after its last contact; `st.blinded` turns it lost on application.
19. **Callout.** A listener 25 m away hears 35 dB: suspicious, `M = 7000`, Vigilance to `now + 5400`; at `V = 300` its gain is 140 per check and it alerts in 22 checks.
20. **Search plan.** Center (100.5, 8, 60.5), velocity east: P1 (106.5, 8, 60.5); ring k 0, r 4000 at (100.5, 8, 56.5); k 2, r 8000 at (92.5, 8, 60.5). The plan draws nothing; doc 13 draws exactly 3 `rng.ai` values.
21. **Scent.** `s0 = 10000` laid at tick 1000: 5000 at tick 2800, usable at 4240, lost at 4241; soaked (`s0 = 5000`) lost at 3881.
22. **Sneak multiplier.** Shade Sneak 60 with `backstab`, tier 1: 41200; the S1 hit totals 118,040 mHP; at Sneak 0 doc 05's 84,899; against a mini-boss 15000; S2 with `ignore_armor` 124,493.
23. **Takedown.** A lethal takedown on a standard leaves it at 0 HP with death noises at 24 dB; it is refused on ranks boss, editor, hunter and champion; damage to the attacker cancels it.
24. **Noise levels.** Heavy walk on metal with `curse.loud` 2: 63 dB; Shade crouch on stone: 25 dB; 4000 mm drop on stone in medium armor: 72 dB.
25. **Ring.** 52 dB in Mythic Fantasy: 14500 mm; 63 dB in Cyberpunk: 10416 mm.
26. **Gem.** V1 Shade: 471, no pips; with a lantern: 2467, two pips.
27. **Bodies.** 6 m, near cone, light 9: discovered on check 17; 14 m, far cone, light 11: check 87; far cone at light 7: never.
28. **Disguise.** Full, GUI 55, enforcer at 5 m, light 12: 153 per check, challenge on check 33, compromise on check 66; partial: 306, checks 17 and 33; light 5: 51.
29. **Lockpicking.** Security 45, SLT 50, `lock.t3`: chance 4000, 90 ticks, break 1250, opens by the 7th failure; each attempt draws exactly 2 values at seq 1000.
30. **Minigame.** Rating 50 against DC 55: `W = 4`; a `lock.t3` manual attempt with 1 miss draws exactly 6 values.
31. **Device hack.** Hacking 40, LOG 55, `lock.t3`, `wired`: chance 3400, 76 ticks; 1 draw at seq 1001.
32. **Construct hack.** Hacking 50, LOG 50, unaware standard drone, Folio 10: chance 9000, 90 ticks, `st.hacked` 285 ticks; a hack on a boss is rejected with no draw.
33. **Traps.** `trap_dc` 27, 45, 75 at Folios 1, 10, 25; Security 30, SLT 45 at Folio 6: chance 4000; 2 draws at seq 1002.
34. **Pickpocket.** The §13.7 example gives chance 7400 and 47 ticks; a sleeping target is robbed with no draw.
35. **Camera.** Standing still at light 10 (`V = 4020`) raises the alarm 60 ticks after first sight; crouched still at light 4 (`V = 726`) never; sweep offsets at p 0, 90, 180, 359 are −4096, 0, 4096, −4050.
36. **Lock tiers.** Depth 1, 5, 6, 10, 11, 25, 26 gives tiers 1, 1, 2, 2, 3, 5, 6; no electronic lock is placed where local TEC ≤ 4.
37. **Determinism.** Perception, hearing, search plans and corpse discovery make no draws; a lockpick attempt's draws are identical whether 0 or 50 detection rolls happened that tick for the same actor.
38. **Co-op.** Canon §21 test 38 holds. When G1 turns alert on A, G1's pair for B (within `R_far`) gains exactly 2500.
39. **Hidden.** A mounted Erratum never has `st.hidden`; an Erratum with no alert observer has it.
40. **Budget.** With 600 due rays of demand above 1536 in one tick, the deferred observers run first on the next tick in ascending `EntityId`.
41. **Invader filter.** An Erratum behind a solid wall, silent for 60 ticks and unmarked, is absent from the invader's snapshot.

## Invariants annex

```
Σ stealth: light ⊗ visibility ⊗ senses ⊗ meter ⊗ states ⊗ noise ⊗ sneak ⊗ hiding ⊗ disguise ⊗ security
W! detection := deterministic, 0 draws                         W! security|pickpocket|trap rolls := keyed rng.stealth, fixed seq
W! sneak eligibility := per (attacker, target) pair            N! other observers' states → eligibility
W! sneak hit ⇒ no crit roll                                     W! ignore_armor ⇐ Sneak 100 ∨ unseen_blade, ¬boss ranks
W! boss|editor|hunter ⇒ sneak_mult ≤ 15000 ∧ no takedown       N! NPC sneak multiplier vs Errata
W! M ∈ 0..=10000; gains chain in listed order                  N! hearing alone → M ≥ 7500 (except hear_alerts)
W! alert ⇔ M reached 10000 ∨ contact ∨ compromise              W! lost/alert decay := timers only
W! stealth_light = max(env − shadow, own, flash, phosphor)     N! stealth_light written to the light grid
W! concealment sum ≥ 15 ⇒ ray blocked                          W! f_conc = 10000 − 667 × sum
W! st.invisible ⇒ sight factor 1000 unless see-invisible       N! invisibility vs scent | tremor | breath | wards | lasers
W! mounted ⇒ ¬st.hidden ∧ ¬hide spot ∧ ¬disguise               N! ghost perceived
W! every noise has a noise.* id and a level from noise_level   W! unattributed noise → disturbance only
W! Unseen ⇔ ¬folio_alerted                                     W! electronic lock ⇒ local TEC ≥ 5
W! lock pity: progress ≥ lock_hp ⇒ open (¬vault, ¬quest)        N! HashMap in StealthWorld
```
