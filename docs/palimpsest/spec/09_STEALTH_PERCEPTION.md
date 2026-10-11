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
| Doc 05 | §1.6, §3.2, §4.9, §5.2, §6.8, §11.1, §11.4 | archetype `sneak_tier`; stance speeds; lean; armor `noise_db`; quick tiers 1 = 30000, 2 = 22500, 3 = 15000 |
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

Sight, hearing, scent, tremor, breath and thermal perception for every NPC, gadget and device; Errata reveal of stealthy NPCs; the detection meter and every awareness transition; noise IDs and levels and the conversion of movement, weapon, spell, block and item events into `emit_noise` requests; AI reactions to noise; the sneak-attack multiplier and the `ignore_armor` grant; takedowns; hiding spots, bodies and corpse discovery; disguises; distractions and light manipulation; locks, lockpicking, electronic locks, device and construct hacking, trap detection and disarming, pickpocketing; co-op stealth; the stealth HUD data contract; the genre stealth flavors; the behavior of `st.deafened`, `st.marked`, `st.invisible`, `st.hidden` and `st.hacked`.

### 1.3 Systems in the tick

| Doc 03 slot | Work done by doc 09 | Iteration |
|---|---|---|
| #5 `sys.perception` (period 3, per-entity phase) | for each due, awake, perceiving observer: candidate gathering, sight rays, sight fill, scent, breath, `st.marked`, decay, timers, sight-driven transitions, corpse and light-change discovery, camera timers, Errata reveal meters | ascending `EntityId`, then the backlog (§1.6) |
| #7 `sys.actions` (doc 05, 06, 07, 11 action code) | calls `noise_level()` (§7.1) when those chapters emit noise; lock, hack, disarm, pickpocket, takedown, hide and bluff actions run their doc 09 state machines here | ascending `EntityId` |
| #30 `sys.sound`, prelude | `convert_events()` (§7.9) turns this tick's `ev.footstep`, `ev.entity_landed`, `ev.block_changed`, `ev.explosion`, `ev.spell_cast`, `ev.entity_killed` into `emit_noise` requests, in event `seq` order, before doc 03 propagates | event order |
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
    pub lkp: Fx3, pub lkp_tick: Tick, pub lkp_src: u8,  // 0 sight, 1 sound, 2 callout, 3 damage, 4 scent, 5 marked
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

**Invisibility** `f_invis(o)`: 10000 without `st.invisible`. With it, `profile.invis_bp` of the observer (1000 for most, Canon §11.1; 4000 for `aware.profile.editor`; 10000 for wards and lasers, which do not use light). `f_invis = 10000` when the observer sees invisible: an Erratum with Awareness ≥ 75 within 4000 mm (doc 04), with the doc 06 Divination hook within 12000 mm.

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
r32    = floor_div(d(SGT_o) × 50, 1)                     // Bp
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
| `aware.profile.editor` | 12000 / 32000 | 8192 / 12743 / 21845 | 3500 | 50 | 0 | 15 | 8000 | invis 4000; ignores disguises |
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
  "invis_bp": 1000, "step_db": 0, "flags": { "scent": 0, "hear_alerts": 0, "ignore_disguise": 0, "stealthy": 0 } }
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

Scent observers (`scent` flag) on each check consider marks within 4000 mm of their origin, owned by a hostile, with `s(m) ≥ 1000`. The freshest such mark (largest `tick`, ties lower `owner`, then lower `cell`) sets the pair for its owner to at least `aware.suspicious` with `lkp` the mark cell and `lkp_src = 4`, and resets that pair's decay delay. Following marks one by one is the trail; sight takes over when it passes. Example: a mark laid at tick 1000 with `s0 = 10000` reads 5000 at tick 2800 and drops below 1000 after tick 4240; a soaked target's mark (`s0 = 5000`) is lost after tick 3880. Swimming across a canal breaks the trail.

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

