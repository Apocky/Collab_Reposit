# 09 — Stealth and Perception
As of 2026-10-12. Status: BINDING. Conforms to canon-1.11; Canon gaps G23 to G28 carry the canon-1.12 wording this chapter already follows.

This chapter owns how every simulated observer sees, hears, smells and feels the world, the per-pair detection meter and the awareness state machine, the noise catalog and its conversion into `ev.noise`, sneak attacks and takedowns, hiding, bodies, disguises, distractions, light manipulation, locks, hacking, traps, pickpocketing, co-op stealth and the genre stealth flavors. Everything here runs inside `palimpsest_sim` on integers. Detection is deterministic: it draws no random numbers. Only lockpicking, hacking, trap work, pickpocketing, disguise bluffs and passive trap and secret detection draw, all from keyed `rng.stealth` in the `seq` range `1000..=1006` that Canon §3.3 reserves for this chapter (§1.5). Every rule below was checked against canon-1.11. Mounts are cut (DECISIONS 55, Canon §17.7), so no rule here reads a rider. Gear never wears or breaks (DECISIONS 65, Canon §4.3), so lockpicks, toolkits and hacking decks are carried tools that no attempt consumes. Alarms are recoverable, but the guards they reach search longer and stay wary until Folio exit (DECISIONS 61, §6.6). Subdue takedowns and Hacking-100 seizes are the stealth routes to pacifist play (DECISIONS 62, §9.4, §13.5). Hacking has the same optional minigame as lockpicking, and auto-resolving either one costs risk and time (DECISIONS 63, §13.3). Every sapient native faction of every genre can be impersonated (DECISIONS 64, §11.1). Hot and warm host migrations carry all stealth state live (DECISIONS 93, §1.4). Ink gore is presentation only (DECISIONS 94, §10.4). The movement kit's sprint-slide and grappling line have exposure and noise rules (DECISIONS 95, §3.1, §7.2). Asynchronous traces never enter the simulation (DECISIONS 96, §14). Time of day and weather feed light, sound, scent and footsteps (DECISIONS 99, §15.3), and Second Edition's fused Folios average the genre sound tables (DECISIONS 101, §15.1).

## Imports

| Source | Section | What this chapter consumes |
|---|---|---|
| Canon | §1.2–§1.6 | `Fx`, `Bp`, `Bam`, `Ticks`, `Db`; two-channel `Light`; `floor_div`, `mul_bp`, `dist2`, `sqrt_fx`; integer-only content in mm, Bam, ticks |
| Canon | §1.8, §1.9 | coordinates; `ang_diff`, `atan2_bam`, `fwd`, `sin_bam`, `cos_bam` |
| Canon | §1.11 | ascending-`EntityId` iteration; phase key; no `HashMap` |
| Canon | §2.2, §2.3 | perception period 3, AI think period 6; rate accumulator; `in_combat`; `light_at`, clamped by doc 03 §10.6 light overrides, as the start of `stealth_light` |
| Canon | §3.3, §3.4 | keyed `rng.stealth` (owner 09) and its reserved `seq` range `1000..=1006`; `rng.check` (owner 04), which disguise challenges never use; trap and secret detection cadence with `seq` below 1000; documented draw counts |
| Canon | §4.1–§4.4 | humanoid capsule and eye heights; speeds; lean; fixed terrain, `block.tag.destructible` and `cat.placeable` placeables; no item durability; micro-voxel cells (Canon gaps G23) |
| Canon | §5.3–§5.8 | dependencies, Contradiction potency and failure for tools; local genre and local axioms |
| Canon | §6.4, §6.5 | rows 17, 24, 31–35, 40; NPC rows 31–35 |
| Canon | §7 | `skill.sneak`, `skill.security`, `skill.pickpocket`, `skill.hacking`, `skill.awareness` and their scope notes |
| Canon | §10.5, §10.6 | step 2.4 sneak eligibility per (attacker, target); step 2.4b flanking cone; step 5 `ignore_armor`; entry points, including the lethal-takedown row |
| Canon | §11.1, §11.4, §11.6 | statuses owned here: `st.deafened`, `st.marked` (visible to the marker and its allies), `st.invisible`, `st.hidden`, `st.hacked`; `st.sleeping`, `st.blinded`; direct-application duration; the Erasure hunter keeps `rank.hunter` |
| Canon | §12, §12.1, §12.2 | `res.breath`; smoke and surfaces; sight concealment sums; tall grass only against crouched or downed targets and corpses |
| Canon | §13.2 | `slot.face` masks; `slot.body`, `slot.back` |
| Canon | §15, §17.3, §17.6, §17.7, §17.9 | factions and stances; ranks; ghosts imperceptible; mounts cut and their IDs retired; invaders |
| Canon | §18 | `ev.noise`, `ev.awareness_changed`, `ev.damage_dealt`, `ev.block_changed`, `ev.entity_killed`, `ev.folio_entered`, `ev.status_applied`; further events registered by doc 03 §14.3 |
| Doc 03 | §2.2, §3.3, §5.8, §5.12, §5.14, §5.15, §5.18, §7.6, §10, §11, §12.5, §13.4, §13.6, §13.8, §14.2, §14.3, §15.2 | system slots `sys.perception` (#5) and `sys.hearing` (#31); `comp.perception`; lean offset; `push_contacts`; downed crawl (`grid.gait.crawl`); `ev.footstep`, `ev.entity_landed`; `raycast_blocks`; `cell_conceal`; light grid, light overrides and doc 06 shadow areas inside every light read (§10.6), and `flashes`; `emit_noise`, sound propagation, `HeardNoise` with `req`; explosion noise and flash; door toggles; `grid.cause.fell`, `grid.cause.landed` and `grid.cause.erased`; payload schemas, `DmgFlags` bit 13 `takedown` and the stealth events registered in §14.3; sleep query; the `sky_src` bit (§10.2) that scopes weather (§15.3) |
| Doc 04 | §2.4, §4.2, §5.2, §5.3, §6.1, §7.2, §9.4, §10.1, §13 | check chance curve, DC ladder and the Deceive check rating; stealth XP events; mastery perks of Sneak, Security, Pickpocket, Hacking, Awareness, Illusion, Divination; Shade and Netrunner talents; `light_step`, `night_eyes`, `fair_folk`, `wired`, `gumshoe` traits; `aug.silent_pads` and the `body.serpentine` footstep rule; `res.breath` drain while holding breath; `avatar_of_fate` double draws; §6.1 pacification for subdues and Hacking-100 seizes |
| Doc 05 | §1.6, §3.2, §4.9, §5.2, §6.8, §9.3, §10.2, §10.3, §11.1, §11.4 | archetype `sneak_tier` and hands; grenade archetypes; stance speeds; lean; armor `noise_db`; the hit pipeline's call into `sneak_eligible` and `sneak_mult`; the tier values 1 = 30000, 2 = 22500, 3 = 15000 that doc 05 assumes |
| Doc 06 | §5.2, §7.1, §13, §14 | form cast noise; schools; shadow essence darkness, read through doc 03 §10.6's `light_at`; `glyph.ess.veil`; hushed; sight essence; `st.sleeping` awareness drop |
| Doc 07 | §2, §10, §11.1 | material `conceal` and `acoustic` class (no material carries `noise.surface.snow`); block records with `light_emit`, `lightsrc`, `block.tag.destructible`, `hide`, `conceal_crouch_only` and door shapes; placeables and traps as `cat.placeable`, including `block.trap.spikes`; Errata gadgets immune to `st.hacked`, and seized hostile devices outside Errata gadget limits (§11.1 rule 2); the hacking-deck failure spark (§11.1 rule 4); lockpick, toolkit and hacking-deck quality |
| Doc 08 | §3, §4.3, §6.3, §7 | liquid and gas conceal; respirators and their hearing penalty; shatter noise; phosphor; mind vision; night-eye; invisibility potions |
| Doc 10 | §5.3, §7.3, §7.8 | witness sight pass; `curse.loud`, `curse.beacon`; the pact `curse.pact.hush` |
| Doc 11 | items, affixes, manufacturers | `item.face.mask`, `item.tool.lockpicks`, `item.tool.hacking_deck`, `item.tool.toolkit`, `item.tool.torch`, `item.trinket.warning_bell`, `item.exotic.quietus`, `item.exotic.dead_drop`, `affix.quiet`, `affix.soft_step`, `affix.ambusher`, `perk.leg.shadowweave`, Silverbough and Midnight Avenue traits; `cat.key` key tags |
| Doc 12 | §2.3, §11.3, §11.4, §12, §16, §17.3, §18, §19.1, §22 | GPV `fog` (§2.3); light bands (§11.3); trap placement (§11.4); `room.restricted` with its faction (§12); `room.stealth`, `room.electronic` (§16); hiding methods of secrets (§17.3); `hazard.camera`, `hazard.alarm` (§18); `gpv.evt.weather_front` (§19.1: fog +1 per 8 m, sound attenuation × 12000) and `lattice.marg.silent` (§22: sound attenuation × 7000), both multiplying this chapter's `SOUND_BP` (§15.1); each Folio's time of day and weather state (DECISIONS 99, Canon gaps G26); a Second Edition Folio's ordered fused genre pair and its GPV fog (DECISIONS 101, Canon gaps G28); the heist Treasury door (§6.3) |
| Doc 13 | §2.1, §2.2, §2.4, §4.2, §4.7, §5.4, §5.7, §9.1, §10.5, §11.1, §12.1 | template `senses`, `perception`, the `outfit` flag and `outfit_mass_g`; the native factions `fac.native.*`; defeats and `ev.entity_pacified`; tags (closed vocabulary incl. `flier`, `breathless`, `electronic`, `enforcer`), roles, ranks; blackboard `last_known`; investigate, search (`min(3, n)` draws), call_allies, raise_alarm, relight and wake_ally actions; reaction delay; stalkers; hacked constructs; boss arena omniscience; hunters |
| Doc 14 | §3.5, §8.9, §10 | the Rubric-completion tick on which `run.note.silent` reads `folio_alerted`; Second Edition depth `25 + 2n`; `party_kills`, `party_pacified` and `ach.pacifist` |
| Doc 16 | §6.6, §6.8, §9.3 | PvP culling (rule 2) as the only invader replication rule; the `STEALTH` snapshot section and its caps; hot, warm and cold host migration |
| Doc 17 | §10.3, §10.11, §10.12, §12.1, §13 | the crosshair ray; `SetLockMode { manual }` (`access.lock_mode`); `ui.hud.lockpick`, `ui.hud.hack`, `ui.hud.pickpocket` and their `Hack`, `Steal` and `Plant` intents; `input.interact`, `input.heavy`, `input.throw`, `input.attack`, `input.ui_back`, `input.hold_breath`, `input.whistle`, `input.shout`; `access.gore` |
| DECISIONS | 55, 61–65, 93–96, 99, 101, 106 | mounts cut; recoverable alarms; pacifist play; the hacking minigame; disguises for every native faction; no durability; host migration; ink gore; movement kit; asynchronous traces; time of day and weather; fused Folios; Second Edition depth |
| R5 | §2, §9.2 | per-pair meter, zone rates, light table, footstep dB, hysteresis, callouts, Vigilance, darkness defense |
| R2 | §2.10, §2.11, §3.6 | Oblivion detection hysteresis and sneak perks; tumbler pity; BG3 obscurement |
| R7 | §1.4 C2, R-BA2, R-BA4 | the R5 meter as the base model; lock-health pity; engineered noise counts as stealth |

---

## 1. Conventions, schedule and state

### 1.1 Notation

- `o` is an observer, `t` (in formulas) a target or subject. The current tick is `now`.
- `d` is the current Folio depth (Canon §13.8). Second Edition Folio `n` has `d = 25 + 2n`, up to 55 (DECISIONS 106, doc 14 §8.9); every depth formula here stays in range there: lock tier stays 6, `trap_dc` reaches its cap of 100 at depth 38, and NPC `PR` and `SR` stay at most 100. `d(x) = x - 40` (Canon §6.2).
- `L_s(e)` is stealth light (§2.1). `X(t)` is exposure (§3.1). `V(o, t)` is the visibility score (§3.4). `M(o, t)` is the detection meter (§5). All four are integers; `L_s` is `0..=15`, the others Bp.
- A **check** is one run of `sys.perception` for one observer (every 3 ticks, Canon §2.2). Per-second rates applied on checks use the Canon §2.3 accumulator with `k = 20`.
- "Ordinal" means the awareness ordinal of §6.1. "Below alert" means ordinal `< 5`.

### 1.2 What doc 09 decides

Everything listed in the introduction, the stealth HUD data contract (§8), and the behavior of `st.deafened`, `st.marked`, `st.invisible`, `st.hidden` and `st.hacked`. Other chapters reach it only through the functions listed under Exports.

### 1.3 Systems in the tick

| Slot | Doc 03 system | Work done by doc 09 | Iteration |
|---|---|---|---|
| #5 | `sys.perception` (period 3, per-entity phase) | for each due, awake, perceiving observer: candidate gathering, sight rays, sight fill, scent, breath, `st.marked`, decay, timers, sight-driven transitions, corpse and light-change discovery, camera timers, Errata reveal meters | ascending `EntityId`, then the backlog (§1.6) |
| #7 | `sys.actions` (doc 05, 06, 07, 11 action code) | calls `noise_level()` (§7.1) when those chapters emit noise; lock, hack, disarm, pickpocket, takedown, hide and bluff actions run their doc 09 state machines here | ascending `EntityId` |
| #30 | `sys.sound`, prelude | `convert_events()` (§7.9) turns this tick's `ev.footstep`, `ev.entity_landed`, `ev.block_changed`, `ev.entity_killed`, `ev.fire_started` and sonic `ev.damage_dealt` into `emit_noise` requests, in event `seq` order, before doc 03 propagates; then copies `flashes` into `flash_ring[0]` after shifting the ring | event order |
| #31 | `sys.hearing` | (1) this tick's `ev.status_applied` for `st.blinded` (§4.3) and for `st.sleeping` and `st.hacked` (T12); (2) contact from `ev.damage_dealt` (§5.4); (3) radio messages with `due == now`; (4) `HeardNoise` and tremor records (§4.6, §5.3), callouts (§6.6) and alarms; (5) pickpocket and takedown results; (6) `st.hidden`; (7) `ev.awareness_changed` flush | steps in order; inside a step, events in `seq` order, then listener ascending `EntityId`, records in list order |

Awareness read by doc 05 at #17 `sys.damage` is the state after #5 of the same tick and after #31 of the previous tick.

### 1.4 State

`comp.perception` (doc 03 storage, doc 09 semantics) holds one `PerceptionState` per perceiving creature, gadget and device. The Folio-level `StealthWorld` is saved with the simulation (Canon §2.5) as doc 02 section `stealth`.

```rust
pub struct PerceptionState {
    pub profile: Id32,                 // aware.profile.* (§4.1)
    pub senses: u8,                    // bit 0 sight, 1 hearing, 2 breath, 3 tremor, 4 thermal, 5 scent (doc 13 order)
    pub pairs: Vec<PairAware>,         // sorted by subject EntityId; at most 16 (§6.8)
    pub disturb: Option<Disturbance>,  // subject 0: the environment (§6.4)
    pub wary_until: Tick,              // Vigilance (§6.3); WARY_FOLIO = Tick::MAX after an alarm (§6.6)
    pub corpse: Option<CorpseEval>,    // body under evaluation (§10.4)
    pub search: Option<SearchPlan>,    // §6.7; one plan per observer, for its highest-ordinal searching stimulus
    pub cam_seen: i32,                 // camera sight ticks (§15.2)
    pub cam_hold: Option<(EntityId, Tick)>, // camera yaw locked on a target until this tick (§15.2)
    pub subdued_until: Tick,           // non-lethal takedown (§9.4)
    pub bluffs: u8, pub satisfied_until: Tick,   // disguise bluffs (§11.4)
    pub challenge: Option<(EntityId, Tick)>,     // disguise challenge: wearer and answer deadline (§11.4)
    pub last_bark: Tick,               // BARK_COOLDOWN (§6.3)
    pub distracted_until: Tick,        // food lure (§12)
}
pub struct PairAware {
    pub subject: EntityId,
    pub state: u8,                     // awareness ordinal 0..=5 (§6.1)
    pub meter: i32,                    // 0..=10000
    pub suspicion: i32,                // disguise suspicion 0..=10000 (§11.2)
    pub acc: i64,                      // meter decay accumulator, Canon §2.3, k = 20
    pub s_acc: i64,                    // suspicion decay accumulator, k = 20
    pub last_gain: Tick, pub last_contact: Tick, pub state_since: Tick, pub last_s_gain: Tick,
    pub lkp: Fx3, pub lkp_vel: Fx3, pub lkp_tick: Tick, pub lkp_src: u8,  // src: 0 sight, 1 sound, 2 callout, 3 damage, 4 scent, 5 marked, 6 alarm
    pub flags: u8,                     // 0 SIGHT_PASS (latest check), 1 HEARD_THIS_TICK, 2 ARRIVED, 3 CALLED_OUT, 4 HUNTED (§9.2)
}
pub struct Disturbance { pub pos: Fx3, pub tick: Tick, pub state: u8, pub meter: i32, pub since: Tick,
                         pub last_gain: Tick, pub acc: i64 }
pub struct CorpseEval { pub body: EntityId, pub meter: i32, pub last_pass: Tick }
pub struct SearchPlan { pub subject: EntityId /* 0 = disturbance */, pub center: Fx3,
                        pub points: Vec<(Fx3, u16 /* weight */, u8 /* 0 P0, 1 P1, 2 ring, 3 hide */)>,
                        pub search_ticks: u32 /* 1200, or 1800 after an alarm */ }
pub struct ScentMark { pub cell: Gidx, pub tick: Tick, pub s0: i32 }
pub struct Trail { pub marks: [Option<ScentMark>; 120], pub head: u8, pub next_mark: Tick }   // ring, oldest overwritten (§4.5)
pub struct HackState { pub kind: u8 /* 0 loop, 1 seize, 2 disabled, 3 power_cut */, pub hacker: EntityId,
                       pub until: Tick /* Tick::MAX = until Folio exit */ }
pub struct StealthAction { pub kind: u8 /* 0 pick, 1 pick_manual, 2 hack_device, 3 hack_seize, 4 disarm, 5 steal, 6 plant,
                           7 takedown_lethal, 8 takedown_subdue, 9 hide_enter, 10 hide_exit, 11 body_pickup, 12 body_drop,
                           13 strip, 14 snuff, 15 relight, 16 bluff, 17 active_search */,
                           pub target: u64 /* EntityId or Gidx */, pub start: Tick, pub end: Tick,
                           pub origin: Fx3 /* actor position at start, for the 500 mm cancel rule */,
                           pub manual: bool /* §13.3 lock_mode at the start; true for kind 1, and for kinds 2 and 3 in manual mode */,
                           pub pins: [u8; 8], pub pin: u8, pub pin_start: Tick, pub misses: u8, pub item: u64 }
pub struct RadioMsg { pub due: Tick, pub receiver: EntityId, pub callout: Callout }
pub struct StealthWorld {
    pub locks: BTreeMap<u64, LockState>,          // key: door Gidx (lower half) or container EntityId (§13.1)
    pub hides: BTreeMap<u64, HideState>,          // hiding spots (§10.1)
    pub scent: BTreeMap<EntityId, Trail>,         // per-owner scent trails (§4.5)
    pub revealed: BTreeMap<u64, u8>,              // hidden objects (traps, secrets, mimics, wards) → party-slot mask
    pub reveal: BTreeMap<(EntityId, EntityId), RevealMeter>, // (erratum, stealthy NPC) (§4.8)
    pub blown: BTreeSet<(EntityId, Id32)>,        // (erratum, faction) disguises blown this Folio (§11.4)
    pub discovered: BTreeSet<EntityId>,           // bodies already reported (§10.4)
    pub blood_seen: BTreeSet<Gidx>,               // blood cells already reported (§10.4)
    pub alarms: BTreeMap<u64, Tick>,              // alarm source → cooldown end (§6.6)
    pub alarmed: BTreeSet<Id32>,                  // factions whose alarm went off this Folio (§6.6)
    pub callout_log: BTreeMap<(u64, EntityId), Tick>, // (squad id, or faction id32 | 1 << 63 without a squad; subject) → last callout tick (§6.6)
    pub callouts: Vec<Callout>,                   // this tick's callout payloads; cleared at sys.tick_begin
    pub radio: VecDeque<RadioMsg>,                // radio callouts in flight, ordered by (due, receiver) (§6.6)
    pub flash_ring: [Vec<(Gidx, u8)>; 3],         // flashes of the three most recent completed ticks, newest first (§2.1)
    pub backlog: VecDeque<EntityId>,              // deferred perception checks (§1.6)
    pub hacked: BTreeMap<u64, HackState>,         // hacked devices and seized units, keyed by EntityId or Gidx (§13.4, §13.5)
    pub hack_fails: BTreeMap<u64, u8>,            // consecutive failed hacks per device (§13.4)
    pub power_cuts: BTreeMap<u32, Tick>,          // doc 12 room id → relight tick (§2.4)
    pub actions: BTreeMap<EntityId, StealthAction>, // at most one running stealth action per actor
    pub emote_ready: BTreeMap<EntityId, Tick>,    // whistle and shout cooldowns (§12)
    pub folio_alerted: bool,
}
```

Every field of `StealthWorld` resets on `ev.folio_entered`. A boundary save (Canon §2.5) holds all of it, the flash ring included, so a loaded game reproduces muzzle-flash exposure and radio delays exactly. Hot and warm host migrations (doc 16 §9.3, DECISIONS 93) continue the Folio from the live simulation state, `StealthWorld` and every `comp.perception` included, so meters, alarms, scent trails and running stealth actions carry over unchanged; only a cold migration restores the Folio-entry snapshot, whose `StealthWorld` holds its Folio-entry values.

`folio_alerted` becomes true the first time any pair toward an Erratum reaches `aware.alert`, and resets on `ev.folio_entered`. Canon §8's Unseen bonus reads `!folio_alerted` at exit; doc 14's `run.note.silent` reads `!folio_alerted` on the tick the Rubric completes (doc 14 §3.5); doc 12's heist Treasury door opens only while `!folio_alerted` (doc 12 §6.3). The run-long flag of earlier drafts is retired: no rule reads alert state across Folios.

### 1.5 Keyed draw allocation (`rng.stealth`)

Every draw uses `keyed_rng("rng.stealth", folio, actor, now, seq)` (Canon §3.3). Action slots start at 1000 so they never share a key with the Canon §3.3 detection rule, whose `seq` is an object index below 1000.

| `seq` | `actor` | Draws, in order | Count |
|---|---|---|---|
| object index `i` (`0..=999`) | the observer | passive trap and secret detection, Canon §3.3 and §13.6 | 1 per object in range |
| 1000 | the picker | auto lockpick attempt (§13.2): success roll, then slip roll | 2 |
| 1001 | the hacker | auto device hack (Loop, Unlock, Disable, Test, Cut) or auto Seize (§13.4, §13.5): success roll on the channel's last tick | 1 |
| 1002 | the disarmer | trap disarm: success roll, then trigger-on-failure roll | 2 |
| 1003 | the thief | pickpocket or plant: success roll | 1 |
| 1004 | the picker or hacker | manual minigame layout for a lock, a device or a Seize (§13.3): one `range(40)` per pin | `2 + tier` |
| 1005 | — | unused: the manual minigame's pick-break roll was retired with item durability (Canon §4.3); a miss draws nothing | 0 |
| 1006 | the disguised Erratum | disguise bluff: the Deceive success roll (§11.4) | 1 |

Perception, meters, transitions, scent, hearing, search patterns and corpse discovery draw nothing. Under doc 04's `avatar_of_fate`, every **success** roll in this table (lockpick, hack, disarm, steal, bluff, passive detection) draws twice from the same keyed generator and keeps the result better for the roller; slip, trigger and layout draws are unchanged. Each such roll therefore adds exactly 1 to its row's count. A slip roll slips when its value is below `slip_bp` (§13.2), and a disarm trigger roll fires the trap when its value is below 5000; neither is a success roll. An actor runs at most one stealth action at a time (§1.4 `actions`), so a lock attempt and a hack never share a key at seq 1004. Canon §3.3 reserves `1000..=1006` on `rng.stealth` for this table and keeps detection's object-index `seq` below 1000. Disguise challenges are AI detection rolls and never draw on doc 04's `rng.check` (Canon §3.3).

### 1.6 Budget

`sys.perception` casts at most `RAY_BUDGET = 1536` sight rays per tick.

**Candidates.** A check considers as sight candidates the live, non-ghost entities `s` with all of: `s` is hostile to `o` by Canon §15 stance, or wears a disguise of `o`'s faction or of a faction friendly or allied to it (§11.1), or is the subject of an existing pair of `o`; `dist2(o_eye, torso(s)) ≤ (max(R_far(o), dv(o)) as i128)²`, with `R_far` and `dv` from §3.3 and §4.3; and, for a civilian, `s` is an Erratum. Each check gathers at most 8 candidates, nearest first by `dist2`, ties by lower `EntityId`. Every existing pair is still processed for decay, timers and removal on every check (§6.2); a pair whose subject is not among the 8 candidates gets `V = 0` that check. When the next observer's worst case (3 rays per subject) would exceed the remaining budget, that observer and every later due observer are pushed to `backlog` in ascending `EntityId`; the next tick processes the backlog first, then its own due observers. A deferred check runs with `now` of the tick it actually runs and does not change the observer's phase. Reference cost: 85 due observers × 2 subjects × 3 rays = 510 rays per tick in a typical Folio.

### 1.7 Events produced

Besides Canon §18's `ev.noise` (through doc 03's `emit_noise`) and `ev.awareness_changed` (§6.2), this chapter produces eight bus events that doc 03 §14.3 registers with their payloads. Each is emitted once per occurrence, on the tick its rule fires:

| Event | Emitted when | Payload values |
|---|---|---|
| `ev.alarm_raised` | `raise_alarm` passes its cooldown and no disabled alarm panel silences it (§6.6, §13.4) | `raiser` = the source `EntityId`, 0 for a block device; `pos`; `kind` 2 for a camera (§6.6 row 1), 1 for every other device (rows 2 to 5 and 9), 0 for a creature (rows 6 to 8) |
| `ev.body_discovered` | a body's meter reaches 5000 (§10.4) | the observer, the body, the body's origin |
| `ev.disguise_blown` | a compromise (§11.4) | the wearer and the compromising observer |
| `ev.lock_opened` | a lock opens (§13.1 to §13.4) | `method` 0 key, 1 picked (pity, a manual pick and Security 100 included), 2 hacked (auto or manual; a terminal's Unlock emits one per lock it opens), 3 forced |
| `ev.object_revealed` | an Erratum's detection or doc 06's sight essence first sets a hidden object's party mask (§13.6) | the detecting Erratum (the caster for sight essence); `kind` 1 for a doc 12 secret, 0 for every other hidden object |
| `ev.device_hacked` | a hack verb or a Seize succeeds (§13.4, §13.5) | the hacker; the device's `EntityId`, or its `Gidx` value for a block device |
| `ev.takedown` | a takedown's result applies on its last tick (§9.4) | the attacker and the target; `lethal` true for a lethal takedown |
| `ev.item_stolen` | a steal succeeds (§13.7); a plant emits nothing | the thief, the victim, the item's uid |

---

## 2. Light at the target

### 2.1 Two-channel light and stealth light

Each cell stores `light_sky` and `light_block` (Canon §1.2). Effective light is `max(light_sky, light_block)`; `light_at(e)` reads the cell at `e.pos + (0, 500 mm, 0)` (Canon §2.3). Spawning reads `light_block` only (Canon §12.3); stealth always reads effective light. Stealth light starts from `light_at`, which already carries doc 06's magical darkness (doc 03 §10.6 lowers every light read inside a shadow field or `linger` area by 8, floor 0), and adds the target's own light, flashes, phosphor and drone spotlights:

```
fn stealth_light(e, now) -> u8:
    env   = light_at(e)                                    // includes the grid's copy of e's own emitter, capped at 10 by doc 03 §10.4;
                                                           // doc 03 §10.6 shadow areas and light overrides already applied, so no second shadow term
    own   = max(0, max over e's lit equipment and held items of item_light(i))   // §2.3 table
    fl    = max(0, max over flashes f in flash_ring[0..3] of (f.level - taxicab(cell(e), f.cell)))
    ph    = 9 if e is phosphor-marked (doc 08) else 0
    spot  = 12 if any drone (aware.profile.drone, not hacked by e's faction) had SIGHT_PASS on e at its latest check else 0   // §15.2
    min(15, max(env, own, fl, ph, spot))
```

Flashes come from doc 03 §10.4's per-tick list, which the systems after #5 fill. At the end of the `sys.sound` prelude of tick `t`, the ring shifts (`[2] = [1]`, `[1] = [0]`) and `[0]` receives tick `t`'s list. A check at tick `t + 1` therefore sees the flashes of ticks `t`, `t − 1` and `t − 2`, so every observer, whose checks are exactly 3 ticks apart, sees every flash on exactly one check. `taxicab` is the sum of the absolute block-coordinate differences. Doc 03 §10.6 light overrides, such as an Editor's blackout, clamp `light_at` and therefore `env` (Canon §2.3); `own`, flashes, phosphor and drone spotlights are not cell reads and are never clamped, so a torch-bearer inside a blackout still has `L_s = 12`.

**Example.** A Gothic Horror crypt corridor (`ambient_light` 2, no fixtures) has effective light 2. An Erratum holding a torch (`own` 12) has `L_s = 12`. Without the torch, a muzzle flash of level 12 fired on tick `t` in its own cell gives `L_s = 12` on ticks `t + 1` to `t + 3`, then 2 again. Standing 2 cells (taxicab) from an ally's flash gives `12 − 2 = 10`. Inside a doc 06 shadow field `light_at` reads `max(0, 2 − 8) = 0` (doc 03 §10.6), but the torch still gives 12.

### 2.2 Light factor and bands

`LIGHT_BP[L]` for `L = 0..=15`:

| L | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Bp | 800 | 1000 | 1300 | 1700 | 2200 | 2800 | 3500 | 4300 | 5100 | 5900 | 6700 | 7500 | 8200 | 8900 | 9500 | 10000 |

Bands follow doc 12 §11.3: **dark** `L ≤ 4`, **dim** `5..=9`, **bright** `≥ 10`. The far-zone posture rule (§3.3) and every "in darkness" condition below use `L ≤ 7` or the band named.

### 2.3 Own emitted light

| # | Source | `item_light` |
|---|---|---|
| 1 | `arch.offhand.lantern` (doc 05) | 13 |
| 2 | `item.tool.torch` held or in `slot.offhand` | 12 |
| 3 | `ench.lightbearer` ranks 1, 2, 3 (doc 06) | 8, 11, 14 |
| 4 | `curse.beacon` (doc 10) | `min(15, 2 + m)` |
| 5 | doc 06 `lumen` self light | 12 |
| 6 | `aware.profile.drone` spotlight (on its own cone, §15.2) | 12 |
| 7 | a burning bearer (`st.burning`) | 12 |

Doc 03 caps grid emitters at 10, so the grid alone understates a lantern; `own` restores the true level for stealth. A lit item in the inventory but not held or worn emits nothing. Toggling a lantern or torch off is `input.interact` while holding it (6 ticks).

### 2.4 Light manipulation

Light blocks are doc 07 records with `light_emit > 0`. Doc 09 classifies them by a `lightsrc` field: `flame` (torches, candles, braziers, oil lamps), `electric` (fixtures on a room circuit) and `arcane` (glowstones, wards). Every Errata-placeable light is a `cat.placeable` (Canon §4.3).

| # | Action | Applies to | Ticks | Noise | Result |
|---|---|---|---|---|---|
| 1 | Snuff (`input.interact`) | flame | 15 | `noise.light.snuff` 20 dB | `set_block` to the unlit state, cause `grid.cause.toggled` |
| 2 | Douse | flame | — | — | any cell contact with ≥ 1 unit of `liq.water`, or a frost hit, unlights it (doc 03 §6.9 hook) |
| 3 | Relight (`input.interact` with a flame source or `dmg.fire` hit) | flame | 30 | `noise.light.snuff` 20 dB | lit state |
| 4 | Switch (`input.interact` on `block.light_switch`) | electric circuit | 6 | `noise.light.switch` 25 dB | every electric fixture of the room toggles |
| 5 | Break | any destructible fixture | weapon | block break noise of its acoustic class (§7.5) | fixture removed; light gone |
| 6 | Power cut: hack or break `block.fuse_box` | electric, whole room | §13.4 | `noise.light.power` 45 dB | every electric fixture of the room unlit; `power_cuts[room] = now + 1800`; on that tick `sys.hearing` relights each fixture through `set_block` unless the box is broken, which keeps the room dark until Folio exit |

Every change goes through `set_block`, so doc 03 relights the grid within its 4,096-pop budget and both channels stay correct. Worldgen light fixtures are destructible (Canon §4.3).

**AI reaction to darkness.** On every `ev.block_changed` whose old state emitted light and whose new state emits less by at least 4 levels, each awake sapient NPC within 16000 mm of the cell whose eye has a clear line to the cell gains a disturbance (§6.4) at `aware.suspicious` with position the cell. A clear line means `raycast_blocks` from the eye toward the cell center reports no hit before the ray enters the fixture cell; a hit on the fixture's own box counts as clear. The change is evaluated in `sys.hearing` from this tick's `ev.block_changed`, comparing `light_emit` of `old` and `new`. On arrival the NPC relights a flame source if its template carries `uses_items`, or walks to the room's fuse box for an electric cut; relighting takes 60 ticks. Doc 13 runs the walk; doc 09 sets the goal. Beasts, undead and constructs ignore light changes.

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
| `st.downed`, a body or Remains (§10.3), or lying under a `hide: under` prop (§10.1) | 3500 |
| eye cell submerged (doc 03 `eye_submerged`) | 3000 |
| swimming, eye above the surface | 7000 |
| climbing or mantling | 12000 |
| airborne, gliding, dodging, or reeling on a grappling line (DECISIONS 95) | 12000 |
| crouched, or in a sprint-slide (DECISIONS 95) | 5500 |
| standing | 10000 |

**Motion** `f_motion`: horizontal speed `s` in mm/s from `vel`, and the gait of doc 03 §5.15 (`grid.gait.*`). NPCs use crouch when crouched, sprint when moving faster than their template `walk_mm_s`, else walk. A sprint-slide counts as gait sprint.

| Condition | Bp |
|---|---|
| `s ≤ 300` | 6000 (still) |
| `s > 7500` (knockback, dodge burst) | 16000 |
| gait crouch or crawl, or swimming or climbing | 8000 |
| gait walk | 10000 |
| gait sprint | 13000 |

**Size** `f_size = clamp(floor_div(stand_h_mm × radius_mm, 54), 2500, 40000)`. A humanoid (1800 mm, 300 mm) is exactly 10000; a 900 × 200 mm goblin-sized body is 3333; a 3600 × 900 mm troll is 40000 (clamped from 60000).

**Armor signature** `f_armor = 10000 + Σ worn armor pieces slot_weight × class_sig`, with slot weights head 1, body 4, hands 1, legs 2, feet 2 (doc 05 §11.3) and `class_sig` cloth 0, light 50, medium 100, heavy 200. Full sets: light 10500, medium 11000, heavy 12000.

**Camouflage** `f_camo`. The **outer layer** is the `slot.back` item when it carries a camo family, else the `slot.body` item. Its skin carries `camo: aware.camo.*` (default none). The **background** is the local genre's primary and secondary family (§15.1, which also gives seam zones and fused Folios), or the doc 12 biome tileset's `camo` field when present. The first matching row applies, so a family in the background never counts as clashing (Weird West's earth and pale clash as a pair, yet a pale cloak there gets 7500).

| Relation | Bp |
|---|---|
| outer family ∈ {primary, secondary} | 7500 |
| no family, or no relation | 10000 |
| outer family clashes with the primary | 12500 |

Families are `aware.camo.shadow`, `.earth`, `.foliage`, `.urban`, `.pale`, `.neon`. Clashing pairs (symmetric): shadow–pale, shadow–neon, earth–pale, earth–neon, foliage–urban, foliage–neon, pale–neon.

**Invisibility** `f_invis(o)`: 10000 without `st.invisible`. With it, 1000 (Canon §11.1). Wards and lasers do not use sight, so invisibility never affects them. `f_invis = 10000` when the observer sees invisible: an Erratum with Awareness ≥ 75 within 4000 mm (doc 04), with the doc 06 Divination hook within 12000 mm.

**Gear and talents** `f_gear(o)`: every applicable row, one `mul_bp` each, in ascending row number (rows 1 to 3 are in ascending source content-ID byte order; the crowd row and the carry row follow):

| # | Source | Condition | Bp |
|---|---|---|---|
| 1 | `perk.leg.shadowweave` (doc 11) | crouched and `L_s ≤ 4` | 7000 |
| 2 | `talent.netrunner.ghost_signal` (doc 04) | observer profile camera, turret or drone | 7000 |
| 3 | `talent.shade.ghost_walk` (doc 04) | gait crouch and `s > 300` | 8000 |
| 4 | crowd (§15.2) | three or more `fac.neutral` civilians within 3000 mm, gait walk or slower or still, no weapon raised, observer farther than 6000 mm | 4000 |
| 5 | carrying a body or Remains (§10.3) | always | 15000 |

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
    sum += floor_div(fog_q × to_mm(len), 8000)              // fog_q = doc 12 GPV fog of local_genre(o_eye), +1 during a weather front,
                                                            //        + wx_fog(o_eye), the Folio weather's fog (§15.3)
    if sum >= 15: return None
    Some(sum)
```

`n` = number of `Some` results. `f_frac = [0, 4000, 7000, 10000][n]`. `c_min` = the smallest `Some` value. `f_conc = 10000 − 667 × c_min` (Canon §12.2). Other bodies never block sight. A cell's micro-voxels (Canon §4.4) block a ray only where doc 03's `raycast_blocks` meets one of that cell's boxes; a crossed cell whose boxes the ray misses adds its `cell_conceal`, which doc 03 reads through Canon §4.4's fill rule: 15, so the ray is blocked, at 48 or more micro-voxels, otherwise scaled by the fill fraction (Canon gaps G23). A target inside an occupied enclosed hiding spot (§10.2) has `n = 0` for everyone except the checker of that spot.

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

| Zone | Condition (`θ_v ≤ 10923` for every cone; 16384 for `aware.profile.flier`) | `f_zone` | `f_dist` |
|---|---|---|---|
| near | `dist ≤ R_near` and `θ_h ≤ near_half` | 10000 | 10000 |
| far | `dist ≤ R_far` and `θ_h ≤ far_half`, not near | 2000 | `10000 − floor_div(7000 × max(0, dist − R_near), R_far − R_near)` |
| peripheral | `dist ≤ R_near` and `θ_h ≤ periph_half`, not near | 1000 | 10000 |
| outside | anything else | 0 | — |

Rows are tested in table order and the first match wins, so a target inside `R_near` but between `near_half` and `far_half` is in the far zone with `f_dist = 10000`; the `max(0, ·)` keeps `f_dist` at most 10000. When `R_far = R_near` (cameras, tattlers) the far row never matches before the near row and its `f_dist` is not evaluated. Posture rules (Shadow Tactics zones, R5 §2.4): in the **far** zone a crouched, downed or submerged target with `L_eff ≤ 7` gives `f_zone = 0`; in the **peripheral** zone a target with `s ≤ 300` or gait crouch gives `f_zone = 0`. Cameras and turrets sweep `o.yaw` (§15.2); every other observer uses `comp.pos` yaw and pitch.

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
| `aware.profile.boss` | 16000 / 40000 | 10923 / 16384 / 24576 | 5000 | 60 | 0 | 15 | 4000 | ignores disguises |
| `aware.profile.editor` | 12000 / 32000 | 8192 / 12743 / 21845 | 3500 | 50 | 0 | 15 | 8000 | ignores disguises |
| `aware.profile.camera` | 16000 / 16000 | 8192 / 8192 / 0 | timer | 0 | 0 | — | 0 (8000 at local TEC ≥ 8) | §15.2 |
| `aware.profile.turret` | 12000 / 20000 | 8192 / 8192 / 0 | 4000 | 30 | 0 | 15 | 8000 | hackable |
| `aware.profile.drone` | 8000 / 20000 | 10923 / 10923 / 0 | 2500 | 30 | 0 | 20 | 8000 | spotlight 12; hackable |
| `aware.profile.ward` | sphere 4000 | — | trip | 0 | 0 | — | — | ignores light and invisibility (§15.2) |
| `aware.profile.laser` | segment | — | trip | 0 | 0 | — | — | §15.2 |
| `aware.profile.tattler` | 6000 / 6000 | 32768 / 32768 / 0 | trip | 0 | 0 | 15 | 0 | §15.2 |
| `aware.profile.erratum` | 12000 / 48000 | 10923 / 10923 / 16384 | 2500 | skill | skill | — | 0 | reveal meters only (§4.8) |

```json
{ "id": "aware.profile.sentry", "near_mm": 10000, "far_mm": 32000,
  "near_half": 5461, "far_half": 8192, "periph_half": 16384, "theta_v": 10923,
  "fill": 3000, "aw_base": 30, "sneak_base": 0, "hear_db": 18, "dv_mm": 0,
  "step_db": 0, "flags": { "scent": 0, "hear_alerts": 0, "ignore_disguise": 0, "stealthy": 0 } }
```

NPC footstep offset `step_db`: humanoid, sentry, civilian, editor 0; stalker −10; beast and hound −5; undead +3; construct +8; boss +10; swarm −10; fliers emit wing beats instead (`noise.move.flap`, 35 dB every 30 ticks).

### 4.2 Default assignment

A doc 13 template may name `perception`. Otherwise the first matching rule wins: (1) rank boss or editor → boss; rank hunter → editor. (2) tag `editor` → editor. (3) gadget archetypes: turret → turret, drone → drone; `hazard.camera` → camera; `hazard.alarm` by skin dependency → laser (techne), ward (arcana or mythos), tattler (none in Fairy Tale and Mythic Fantasy), otherwise a tripwire segment that uses the laser rules without visibility. (4) tag `swarm` → swarm. (5) tag `eldritch` with no `sight` sense → horror. (6) role stalker → stalker; role sniper or guardian → sentry. (7) tag `construct` → construct. (8) tag `flier` → flier. (9) tag `beast` → beast. (10) tags `undead` and `mindless` → undead. (11) faction `fac.neutral` → civilian. (12) humanoid. Hounds are named explicitly by their templates.

### 4.3 Dark vision and blinding

`dv(o) = 1000 × (2 + floor_div(SGT_o, 20) + Σ RowFlat(31)) + profile.dv_mm + Σ dv_mm effects` in mm (Canon §6.4 row 31). `RowFlat(31)` terms are whole meters: `trait.kin.beastkin.night_eyes` +2, `talent.shade.nightborn` +3, `aug.eye_lowlight` +3 per rank, `bp.head` tier-4 wound −2 (doc 04). `dv_mm` effects are millimeters: Canon §16.4 `afflict.hunger` +2000, doc 08 `alch.eff.night_eye` + its magnitude, doc 08 `liq.phosphor` +2000. NPCs use row 31 from their effective SGT (Canon §6.5) plus the profile term. **Devices** (profiles camera, turret, drone, ward, laser, tattler) have no sub-attributes: their `dv` is `profile.dv_mm` alone (a camera's is 8000 mm where the local TEC is at least 8, else 0), their `r32` is 0 and their `thr` is `hear_db`. Within `dv(o)` the observer reads `L_eff = max(L_s, 8)`: darkness stops protecting up close. Example: SGT 40 gives 4000 mm; an undead with SGT 40 sees at light 8 out to 10000 mm.

`st.blinded` (Canon §11.1): sight only within 2000 mm. On the tick it is applied, every pair at `aware.alert` whose last contact was sight moves to `aware.lost` (§6.2 T4 with no wait), with `lkp` the subject's current position. `noise.explosion.flash` and doc 05's flash grenade apply it through nerve buildup.

### 4.4 Hearing thresholds and ambient masking

```
thr(o)     = profile.hear_db − floor_div(d(HRG_o), 5) − Σ RowFlat(33) + 30 × [o has st.deafened] + resp_db(o)
amb(o)     = AMBIENT_DB[local_genre(o.pos)] + 10 × [weather front active in o's room] + wx_amb(o.pos)   // §15.3
gate(o, e) = max(thr(o) + od, amb(o) + 3 + max(0, od))
od         = opp_db(o, e) (§5.1) when the noise is a body noise (§7.1) of a hostile emitter e, else 0
```

`resp_db` is doc 08's respirator penalty (cloth 0, mask 3, rebreather 5). `AMBIENT_DB` is in §15.1, seam zones and fused Folios included; a noise below `amb + 3` is inaudible (Splinter Cell rule, R5 §2.4). A negative `od` (a clumsy emitter) lowers only the threshold term, so no Sneak value ever makes a noise audible under the ambient mask; a positive `od` raises both terms, so Sneak still helps in loud places. Example: a humanoid with HRG 40 in Steampunk (ambient 40) has `thr = 20` and, for a non-body noise, `gate = 43`; in The Unwritten (ambient 0) `gate = 20`. For a footstep of an emitter with `od = −3` the Steampunk gate stays 43 and the Unwritten gate falls to 17; with `od = +2` they are 45 and 22.

**Genre sound.** Path attenuation scales with

```
sound_bp_eff(pos) = SOUND_BP[local_genre(pos)]              // §15.1, owned here
                    → × 12000 while a doc 12 weather front is active in pos's room
                    → × wx_sound(pos), the Folio weather's sound term (§15.3)
                    → × 7000 in a Folio carrying lattice.marg.silent (doc 12)
```

each arrow one `mul_bp`. `sys.hearing` reconstructs each `HeardNoise` record's path cost and rescales it with `sound_bp_eff` at the noise position, which it reads with the source level through `HeardNoise.req` (doc 03 §11.5):

```
L0    = 10 × source_db;  cost = L0 − 10 × heard_db
L_eff = floor_div(L0 − mul_bp(cost, sound_bp_eff(noise.pos)), 10)
```

Example: a 45 dB step heard at 25 dB in Hardboiled Noir (13000): `cost = 200`, `L_eff = floor_div(450 − 260, 10) = 19`. In The Redaction (6000): `floor_div(450 − 120, 10) = 33`. In Mythic Fantasy during a weather front (`mul_bp(10000, 12000) = 12000`): `floor_div(450 − 240, 10) = 21`. Noises doc 03 already dropped (heard ≤ 0) stay inaudible.

### 4.5 Scent trails

**Scented creatures.** Every living, non-ghost creature is scented except those tagged `construct`, `elemental` or `editor` (doc 13 §2.2). Every Erratum is scented; kin traits that change this belong to doc 04.

**Layers.** A scented creature lays marks only when something can track it: every Erratum, every `fac.errata` minion, the invader, and every other scented creature only while at least one living `fac.errata` minion with the `scent` sense is in the Folio. A layer owns one `Trail` in `StealthWorld.scent`, created on its first mark. On ticks where `(now + (id & 0xFF_FFFF_FFFF) % 30) % 30 == 0` (its §1.11 phase with period 30), while grounded, it writes `ScentMark { cell: Gidx of its feet cell, tick: now, s0 }` at `head` and advances `head` modulo 120, overwriting the oldest mark. `s0 = 10000 → × 5000 with st.soaked → × 15000 for kin undead`, one `mul_bp` each. No mark is written while the feet cell holds ≥ 1 liquid unit or the body is airborne or swimming. 120 marks × 30 ticks = 3600 ticks, the full lifetime, so no live mark is ever overwritten. A trail is deleted when its owner dies, despawns or leaves the Folio.

**Strength** at `now` is `s(m) = s0 − floor_div(s0 × (now − m.tick), life)`, with `life = 3600`, or 1800 while, at the time of evaluation, a weather front is active in the mark cell's room or `wx_wash` holds at the mark cell (§15.3).

**Tracking.** Scent observers (`scent` sense) on each check evaluate every layer hostile to them whose origin lies within `SCENT_OWNER_RANGE = 64000` mm of the observer's origin, and in each such trail every mark whose cell center lies within 4000 mm of the observer's origin with `s(m) ≥ 1000`. The freshest such mark over all considered trails (largest `tick`, ties lower owner `EntityId`, then lower `cell`) sets the pair for its owner to at least `aware.suspicious` with `lkp` the mark cell center and `lkp_src = 4`, and sets that pair's `last_gain = now`, which restarts its decay delay. A scent observer with `now < distracted_until` (§12) skips tracking. Following marks one by one is the trail; sight takes over when it passes. Example: a mark laid at tick 1000 with `s0 = 10000` reads 5000 at tick 2800 and drops below 1000 after tick 4240; a soaked target's mark (`s0 = 5000`) is lost after tick 3880. Swimming across a canal breaks the trail.

### 4.6 Tremor, breath and thermal

- **Tremor** (doc 13). For each footstep or landing noise of a grounded emitter with source level ≥ 25 dB, every tremor observer within 12000 mm (origin to origin) receives a hearing record with `L_rx = L − floor_div(2 × dist_mm, 1000)`, `exact = true`, ignoring walls, ambient masking and `sound_bp`. It is processed as hearing (§5.3), with `sound_bp_eff` not applied. When a tremor observer also received a `HeardNoise` record for the same request, only the record with the larger level is processed. Example: a 40 dB walk on dirt 8000 mm away arrives at `40 − 16 = 24` dB through any number of walls. A crouch-walk on grass (25 − 6 = 19 dB) never registers.
- **Breath** (doc 13, the jiangshi rule). Each check, every hostile within 6000 mm that breathes counts as a near-zone sight pass with `V = 10000`, ignoring walls and light. Breathless entities (Errata of kin construct or undead, the `machine_body` and `deathless` traits of doc 04, NPCs tagged `breathless`, `construct`, `undead` or `elemental`) and an Erratum holding its breath are undetectable to it. **Holding breath** (`input.hold_breath`, doc 17) drains `res.breath` at 1000 per second like submersion (doc 04) and ends when breath reaches 0 or the input is released.
- **Thermal** (doc 13) ignores gas concealment in §3.2. It also reads `L_eff = max(L_s, 8)` at any distance for targets that are not undead, construct or elemental.

### 4.7 Marking, mind vision and seeing the invisible

- **`st.marked`** (Canon §11.1, owner 09). While target `t` bears `st.marked` applied by source `S`, `S` itself and every observer allied with `S` (same faction, or the party for Errata sources) treat `t` as a near-zone sight pass with `f_frac = 10000`, `f_conc = 10000` and `f_zone = 10000` at any distance up to their `R_far`, and set `lkp` to `t`'s exact position each check (`lkp_src = 5`). Errata see marked entities through walls (doc 17 renders the outline).
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
gain = profile.fill → × V(o, t) → × opp(o, t) → × STATE_MULT[state] → × (12500 if now < wary_until) → × (5000 if now < distracted_until)
M    = min(10000, M + gain);  if gain > 0 { last_gain = now; lkp = t.pos; lkp_vel = t.vel; lkp_tick = now; lkp_src = 0 }
if state == alert and V ≥ ALERT_KEEP_V: last_contact = now
STATE_MULT: unaware 10000, curious 12500, suspicious 15000, searching 15000, lost 20000, alert 10000
```

Cameras use a sight timer, wards, lasers and tattlers trip (§15.2); disguised subjects in front of their outfit's faction use suspicion instead (§11.2); civilians fill only through crimes (§15.2). An alert pair keeps contact only when `V ≥ ALERT_KEEP_V = 300`.

### 5.3 Hearing fill

`sys.hearing` handles each `HeardNoise` record for a perceiving listener (doc 03 §11.5) after the §4.4 rescale:

```
gate   = gate(o, emitter)                                       // §4.4; od = 0 for every non-body or unattributed noise
if L_eff < gate: inaudible; stop
margin = L_eff − gate
gain   = clamp((margin + 1) × 250, 250, 5000) → × CLASS_FILL → × STATE_MULT[state] → × wary 12500
CLASS_FILL: noise.step.*, noise.move.* 4000; every other noise 10000
cap    = 10000 if profile.hear_alerts else max(M, 7499)                    // sound alone never alerts a sighted observer
M      = min(cap, M + gain); last_gain = now
```

An attributed noise (§7.7) feeds the emitter's pair, creating it if absent. An unattributed noise feeds the disturbance (§6.4) with the same gain and `od = 0`. Allied noises feed nothing except through callouts (§6.6). `lkp` follows §6.5. `HeardNoise.req` gives `sys.hearing` the request's position, source level, emitter and callout payload (doc 03 interface).

### 5.4 Contact

Processed first in `sys.hearing`, from this tick's `ev.damage_dealt` where `target` perceives and `attacker` is an entity:

```
if pair(target, attacker).state == alert: last_contact = now; lkp = attacker.pos
elif dist(target, attacker) ≤ 3000 mm or pair had SIGHT_PASS on the latest check:  → alert, M = 10000
else: → lost, M = max(M, 9000), lkp = attacker.pos, quantized to 4000 mm when dist > 20000 mm (§6.5), lkp_src = 3
```

A hit from an unseen attacker beyond 3000 mm therefore leaves the target lost: still below alert, so the next hit is still a sneak attack (Canon §10.5 step 2.4), but the target now runs at the shot's origin. Every transition to lost from this rule sets the pair's `HUNTED` flag (§9.2). Damage with no entity attacker (hazards, falls) sets the disturbance to suspicious at the target's position. Doc 03 §5.14 push contact between a perceiver and a hostile body adds 5000 to `M` with exact `lkp` and forces at least suspicious; doc 03 reports each push pair of the tick to `sys.hearing`. `attacker` here is the hit's owner as §9.1 defines it.

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

"At or above `aware.suspicious`" means ordinal ≥ 2; "below alert" means ordinal ≤ 4.

### 6.2 Transitions

| # | From | To | Condition | Entry effect |
|---|---|---|---|---|
| T1 | unaware | curious | `M ≥ 2500` | bark; head turn |
| T2 | unaware, curious | suspicious | `M ≥ 5000`; or an investigate- or alarm-class attributed noise with margin ≥ 10 dB; or a scent mark; or a callout (§6.6); or push contact | `M = max(M, 5000)`; doc 13 investigates `lkp` |
| T3 | any of 0–4 | alert | `M` reaches 10000; or §5.4 contact; or disguise compromise (§11.4) | `M = 10000`; `state_since = now`, which doc 13's reaction delay reads; `last_contact = now`; callout (§6.6); `folio_alerted = true` if the subject is an Erratum |
| T4 | alert | lost | `now − last_contact ≥ LOST_AFTER (90)`; immediately when the observer gains `st.blinded` | `lkp` = predicted position (§6.5); `M = 9000`; set `HUNTED` |
| T5 | lost | alert | a sight pass with `V ≥ 300`; an attributed noise with margin ≥ 10; §5.4 contact | as T3, no new callout |
| T6 | lost | searching | within 1500 mm of `lkp`; or `now − state_since ≥ LOST_MAX (600)` | build the search plan (§6.7); check enclosed spots near `lkp` first (§10.2) |
| T7 | suspicious | searching | within 1500 mm of `lkp` and 120 ticks of look-around elapsed (`SUSP_LOOK`); or `now − state_since ≥ SUSP_MAX (900)` | build the search plan (§6.7); check enclosed spots near `lkp` first (§10.2) |
| T8 | searching | unaware | `now − state_since ≥ search_ticks` (1200; 1800 after an alarm) and `M < 2500` | Vigilance (§6.3); clear `HUNTED`; drop the plan; doc 13 returns to post |
| T9 | searching | suspicious | a new stimulus whose position is more than 6000 mm from the plan center | new `lkp`; a stimulus within 6000 mm instead updates `lkp`, rebuilds the plan around it and sets `state_since = now`, staying searching |
| T10 | curious | unaware | `M < 1000` and `now − last_gain ≥ CURIOUS_HOLD (240)` | none |
| T11 | any | (pair removed) | subject dead, despawned, a ghost, out of the Folio, no longer hostile and not disguised, or pacified | `ev.awareness_changed` to unaware |
| T12 | any | unaware, `M = 0` for every pair and the disturbance | observer gains `st.sleeping` (doc 06 §14), is subdued (§9.4) or gains `st.hacked` (§13.4, §13.5) | suspicion 0, `HUNTED` cleared, plan dropped; no `ev.awareness_changed` for pairs already unaware |

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
| `WARY_FOLIO` (Vigilance after an alarm) | `Tick::MAX`, until Folio exit | | |

Vigilance (MGS Caution, R5 §9.2) multiplies every gain by 12500 while `now < wary_until`. Granting Vigilance sets `wary_until = max(wary_until, now + WARY_TICKS)`, so a grant never shortens a longer one. An alarm grants it until Folio exit instead (`wary_until = WARY_FOLIO`, §6.6). An alarm is therefore recoverable: its searchers return to unaware through T8 when their 1800-tick search ends with `M < 2500`, but they stay wary for the rest of the Folio (DECISIONS 61). Perception state ends with the Folio, so `WARY_FOLIO` needs no reset.

### 6.4 Disturbances

The disturbance is the observer's awareness of a place rather than of an entity: unattributed noises (thrown objects, impacts, explosions, alarms with no known subject), light changes (§2.4), discovered bodies (§10.4) and environmental damage. It uses ordinals 0–3, the same thresholds (T1, T2), the same hearing gain with `od = 0`, its own decay (§5.5 rates by ordinal, with its `last_gain` and `acc`), and the exits T7, T8, T9 and T10 with its own position. It never reaches lost or alert. An observer's overall posture is the highest ordinal among its pairs and its disturbance; doc 13 acts on the highest, ties broken by the newest `lkp_tick`, then the lower subject `EntityId`, with the disturbance as subject 0.

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
| alarm (§6.6) | the alarm position `pos` | 6 |

`lkp` only moves to a stimulus at least as new as `lkp_tick`. On T4 the predicted position is `lkp + lkp_vel × 60 ticks` (one second along the last seen velocity), cut at the first cell where `raycast_blocks` from `lkp` hits. Example: last seen at (40.0, 9.0, 30.0) m moving at (4.3, 0, 0) m/s with a wall at x = 43: the prediction is 4300 mm east, cut at the wall face, x = 43.0 m.

Doc 13's blackboard copies `lkp`, `lkp_tick` and `lkp_src` each think (doc 13 §4.2).

### 6.6 Group alert propagation

- **Callout.** On T3, a sapient observer emits `noise.voice.callout` (85 dB) at its eye, unless `callout_log[(key, subject)] > now − CALLOUT_COOLDOWN`, where `key` is its squad id (doc 13 §4.10) or, without a squad, `faction.id32 as u64 | 1 << 63`; emitting writes `callout_log[(key, subject)] = now`. The payload `Callout { emitter, subject, lkp, tick, disguise_blown: bool }` rides with the noise request and is pushed to `StealthWorld.callouts`. Doc 13's `ai.action.call_allies` re-emits it under the same cooldown.
- **Hearing a callout.** A listener of the emitter's faction, or of a faction friendly or allied to it (Canon §15), that hears the callout above its gate: a pair for the subject below suspicious becomes suspicious with `M = max(M, 7000)` and the payload `lkp`; a pair at suspicious, searching or lost takes the newer `lkp` and restarts its state timer; alert is unchanged. Every such listener gains Vigilance (§6.3).
- **Radio.** When the emitter's faction carries the doc 13 `radio` flag and the local TEC at the emitter is ≥ 5, the callout pushes `RadioMsg { due: now + RADIO_DELAY, receiver, callout }` for every other living member of its squad in ascending receiver `EntityId`. At `due`, `sys.hearing` step 3 applies each message as a heard callout without a hearing check, and the receiver emits `noise.alarm.radio` (50 dB) at itself, which Errata can hear. A message whose receiver died or became pacified is dropped.
- **Allied combat noise.** An alarm-class noise (gunfire, explosion, scream) emitted by an allied NPC that holds some subject at alert is processed by allied listeners as a callout about that subject.
- **Alarms.** `raise_alarm(faction, pos, subject, source)` does nothing while `alarms[source] > now`; otherwise it sets `alarms[source] = now + ALARM_COOLDOWN`, emits the source's alarm noise from the table below at the source, and for every awake member of the faction and of friendly or allied factions within 48000 mm of `pos`, in ascending `EntityId`: `wary_until = WARY_FOLIO`; when `subject ≠ 0`, the pair for `subject` (created if absent) becomes searching at `pos` with `lkp_src = 6` and `search_ticks = 1800` if its ordinal is below searching; when `subject = 0`, the disturbance does the same. The faction joins `alarmed`, the call emits `ev.alarm_raised` (§1.7), and doc 13's Director receives `alarm_raised(pos, faction)`. A perceiving creature of a faction in `alarmed` that spawns later in the Folio (doc 13 reinforcements and reserve waves) starts with `wary_until = WARY_FOLIO`; `alarmed` resets with the rest of `StealthWorld` on `ev.folio_entered`. `source` is the device's `EntityId` (or `Gidx` for a block device), or the raising creature's `EntityId`. A device's faction is the faction doc 12 records when placing it, by default the Folio's primary native faction (doc 13).

| # | Source | Trigger | Alarm noise | `subject` |
|---|---|---|---|---|
| 1 | camera (§15.2) | `cam_seen` reaches 60, or a disguise trigger seen (§11.3) | `noise.alarm` 90 dB | the seen target |
| 2 | laser or tripwire, `hazard.alarm` techne and mundane skins (§15.2) | a body capsule crosses the segment | `noise.alarm` 90 dB | the crossing body |
| 3 | ward, `hazard.alarm` arcana and mythos skins (§15.2) | a hostile body center within 4000 mm | `noise.alarm` 90 dB (chime skin) | that body |
| 4 | tattler, `hazard.alarm` dependency-none skin in Fairy Tale and Mythic Fantasy (§15.2) | an Erratum enters 6000 mm, not crouch-walking at ≤ 2000 mm/s and not still | `noise.voice.squawk` 90 dB | 0 |
| 5 | alarm panel "test" (§13.4) | Hacking success with the test verb | `noise.alarm` 90 dB | 0 |
| 6 | hound (§15.2) | a hound pair at suspicious or above | `noise.voice.dog_bark` 90 dB | that pair's subject |
| 7 | civilian (§15.2) | a civilian meter reaches 10000, or a civilian discovers a body | `noise.voice.scream` 95 dB | the criminal, or 0 for a body |
| 8 | bell or klaxon (doc 13 `ai.action.raise_alarm`) | 60-tick channel at the prop | `noise.alarm.bell` 100 dB | the NPC's target |
| 9 | third consecutive failed hack on one device (§13.4) | the failure | none beyond the `noise.hack.fail` already emitted | the hacker |

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

Doc 13's `ai.action.search` visits P0, then draws `min(3, n)` points, `n` being the plan points other than P0, without replacement: `min(3, n)` `weighted` calls on its keyed `rng.ai` (seq 1) over the list in the order P1, RING (k, then r), HIDES, using these weights, a chosen point's weight set to 0 before the next call. At each point the searcher looks around for 60 ticks (4 yaw steps of 16384 Bam, 15 ticks each); at a hiding spot it performs the 60-tick check of §10.2. **Squad split:** when `q ≥ 2` squad members search centers within 2000 mm of each other for the same subject, the member with rank `i` (ascending `EntityId`, `0..q`) keeps P0 only if `i = 0`, ring points with `k mod q = i`, and hiding spots with list index `mod q = i`; its own draws then run over its kept points.

Example: center (100.5, 8.0, 60.5) m, last velocity east. P1 is (106.5, 8, 60.5). Ring point k = 0, r = 4000 is (100.5, 8, 56.5) (yaw 0 faces −Z); k = 2 (yaw 16384, facing −X), r = 8000 is (92.5, 8, 60.5). A dark ring cell has weight 3, P1 weight 4, a closet weight 3.

### 6.8 Pair bookkeeping and queries

- A pair is created on the first gain or stimulus. When 16 pairs exist, a new subject evicts the lowest of (ordinal, `M`, `last_gain`, then highest subject id) among unaware pairs; when none is unaware, the new subject is ignored this check.
- `aware_state(o, s) -> Id32`: the pair's state ID, `aware.unaware` when no pair exists. Doc 05 calls it `aware_of(target, attacker)`.
- `sneak_eligible(att, tgt, now) = tgt has st.sleeping ∨ (tgt has comp.perception ∧ ordinal(aware_state(tgt, att)) < 5)` (Canon §10.5 step 2.4). Entities without perception (Errata, invaders, containers) are eligible only while sleeping, so NPCs never sneak-attack Errata (doc 13 §5.4).
- `sight_pass(o, s)`: the `SIGHT_PASS` flag of the latest check.
- `awareness_above_unaware(e)` for doc 03 §15.2: any pair toward an Erratum or the disturbance has ordinal ≥ 1.
- **`st.hidden`** (Canon §11.1): at the end of `sys.hearing`, for each Erratum in ascending `EntityId`, apply it when the Erratum is not a ghost and no observer holds it at alert; remove it otherwise. It never gates sneak attacks.

```rust
fn check_pair(o: &mut Observer, p: &mut PairAware, t: &Entity, now: Tick) {
    if removed(o, t) { emit_change(o, p, AWARE_UNAWARE); drop(p); return; }        // T11
    let v = if o.senses & SIGHT != 0 { visibility(o, t) } else { 0 };
    p.flags.set(SIGHT_PASS, v >= 1);
    if v >= 1 && !disguise_path(o, t) && !camera(o) {
        let g = chain(o.profile.fill, &[v, opp(o, t), STATE_MULT[p.state], wary(o, now), distracted(o, now)]);
        if g > 0 { p.meter = min(10000, p.meter + g); p.last_gain = now; touch_lkp(p, t, SRC_SIGHT); }
    }
    if p.state == ALERT { if v >= 300 { p.last_contact = now; } }
    if p.meter >= 10000 && p.state < ALERT { set(o, p, ALERT, now); }                        // T3
    else if p.state == LOST && v >= 300 { set(o, p, ALERT, now); }                            // T5
    else if p.state == ALERT && now - p.last_contact >= 90 { set(o, p, LOST, now); p.meter = 9000; p.flags.set(HUNTED, true); predict(p); } // T4
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
    if id is noise.step.*, or noise.move.* other than splash and flap: L += armor_db(emitter)   // voices take none
    L += Σ add_db(emitter, id)                         // the modifier table, in ascending source content-ID order
    L  = min(L, Σ caps)                                // every applicable cap, smallest wins
    max(0, L)
armor_db(e) = Σ worn pieces max(0, noise_db − floor_div(skill.armor, 25)), and 0 at Armor 100   // doc 05 §11.4, doc 04
```

**Body noises** are `noise.step.*`, `noise.move.*`, `noise.voice.*` and `noise.item.use`; only they take `opp_db` at the listener (§5.3).

| # | Modifier | Noises | dB |
|---|---|---|---|
| 1 | armor_db | `noise.step.*`; `noise.move.*` except `noise.move.splash` and `noise.move.flap` | full set: light 3, medium 6, heavy 11 at Armor 0 |
| 2 | Sneak ≥ 25 (doc 04) | `noise.step.crouch`, `noise.step.crawl` | −3 |
| 3 | `talent.shade.soft_step` | steps, landing | −6 |
| 4 | `trait.kin.fae.light_step` | steps, landing | −3 |
| 5 | `affix.soft_step` (doc 11) | steps, landing | −X |
| 6 | Silverbough armor (any piece) | steps, landing | −6 once |
| 7 | `aug.silent_pads` (doc 04 §9.4) | steps | −6 |
| 8 | `body.serpentine` body plan (doc 04 §13, `echo.jormungandr`) | steps | −6 |
| 9 | `st.encumbered` / `st.overburdened` | steps, landing | +3 / +6 |
| 10 | `affix.quiet` | attack noises: `noise.melee.swing`, `noise.shot.*` | −X |
| 11 | Silverbough weapon | attack noises | −20 |
| 12 | `curse.loud` (doc 10) | every noise the bearer emits | +m |
| 13 | `curse.pact.hush` (doc 10 pact) | every noise the bearer emits | −m, the pact's boon magnitude (doc 10 §7.8) |
| 14 | `st.erasure` (Canon §11.6) | every noise the bearer emits | +stacks |
| 15 | Midnight Avenue weapon (cap) | shots ≤ 50, swings ≤ 40 | cap |
| 16 | `talent.shade.quiet_kill`, takedowns (cap) | the victim's death cry and body fall | ≤ 24 (audible to 2 m) |

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
| `noise.move.slide` | 45 + surface | each sprint-slide, on its first tick (DECISIONS 95) |
| `noise.move.reel` | 30 | every 30 ticks while reeling on a grappling line (DECISIONS 95) |
| `noise.move.dodge` | 45 | each dodge |
| `noise.move.splash` | 50 | entering liquid with `|v.y| ≥ 3000 mm/s` |
| `noise.move.flap` | 35 | every 30 ticks for fliers |

NPC steps add the profile `step_db` (§4.1). **Surface** is the `acoustic` class of the ground block's material (the `material` of `ev.footstep` and `ev.entity_landed`). Doc 07 §2.4 writes exactly one `acoustic` value on every material, and the material lists below repeat it verbatim; a material with no value (only `air`, never a ground block) reads `noise.surface.dirt`. Four feet-cell overrides replace the ground class, first match wins: ≥ 1 unit of `liq.mud` (doc 08) gives `noise.surface.mud`; ≥ 1 unit of any other liquid gives `noise.surface.water`; a `block.tall_grass` cell (doc 07) gives `noise.surface.grass`; a feet cell where `wx_snow` holds (§15.3: a snowy Folio weather over a sky-exposed cell) gives `noise.surface.snow`.

| Class | dB | Source: doc 07 §2.4 materials, or the override |
|---|---|---|
| `noise.surface.metal` | +10 | copper, tin, zinc, bronze, iron, cold_iron, scrap_iron, cast_iron, steel, brass, silver, hexed_silver, gold, chrome, titanium, moonsilver, plasteel, orichalcum, star_metal, aetherium, redacted_alloy, gold_leaf |
| `noise.surface.glass` | +8 | glass, rad_glass, psi_crystal |
| `noise.surface.water` | +8 | override: ≥ 1 unit of any liquid other than `liq.mud` in the feet cell |
| `noise.surface.stone` | +6 | granite, sandstone, slate, basalt, marble, obsidian, jade, brick, concrete, terracotta, coral, slag, void |
| `noise.surface.ice` | +4 | ice (`block.ice`) |
| `noise.surface.bone` | +4 | bone, dragonbone, chitin |
| `noise.surface.wood` | +3 | pine, birch, cedar, bamboo, oak, thornwood, blackwood, elderwood |
| `noise.surface.synthetic` | +2 | polymer, carbon_fiber |
| `noise.surface.mud` | +2 | override: ≥ 1 unit of `liq.mud` in the feet cell |
| `noise.surface.dirt` | 0 | soil, coal; the default for a material with no `acoustic` value |
| `noise.surface.sand` | −2 | sand |
| `noise.surface.soft` | −4 | hide, leather, rubber, flesh |
| `noise.surface.foliage` | −4 | leaves |
| `noise.surface.grass` | −6 | override: a `block.tall_grass` cell as the feet cell |
| `noise.surface.paper` | −6 | paper, vellum |
| `noise.surface.cloth` | −8 | linen, tweed, silk, nanoweave, dreamstuff, web |
| `noise.surface.snow` | −8 | override: a feet cell where `wx_snow` holds (§15.3); doc 07 defines no snow material or block (doc 07 §2.4, triage G-203), so no base-game material reads it, and data-pack materials (doc 02 modding) may |

Examples: walking on metal in a full heavy set at Armor 0 with `curse.loud` 2: `40 + 10 + 11 + 2 = 63` dB. A Shade crouch-walking on stone in a light set with Sneak 25 and `soft_step`: `25 + 6 + 3 − 3 − 6 = 25` dB. A 4000 mm drop onto stone in a medium set: `40 + 20 + 6 + 6 = 72` dB.

### 7.3 Weapons

Melee, per doc 05 archetype. `noise.melee.swing` is emitted at the attacker's eye on the first active tick of each light or heavy attack; `noise.melee.hit` at the hit position of each hit on an entity.

| Archetype (doc 05) | Hands | `noise.melee.swing` light / heavy | `noise.melee.hit` | Suppressor |
|---|---|---|---|---|
| dagger | 1 | 25 / 30 | 50 | — |
| blade_short | 1 | 30 / 35 | 55 | — |
| blade_long | 1 | 30 / 35 | 55 | — |
| blade_great | 2 | 35 / 40 | 55 | — |
| axe | 1 | 30 / 35 | 55 | — |
| axe_great | 2 | 35 / 40 | 55 | — |
| mace | 1 | 30 / 35 | 60 | — |
| warhammer | 2 | 35 / 40 | 60 | — |
| spear | 1 | 30 / 35 | 55 | — |
| halberd | 2 | 35 / 40 | 55 | — |
| quarterstaff | 2 | 35 / 40 | 60 | — |
| fist | 1 | 25 / 30 | 50 | — |
| whip | 1 | 60 / 65 (the crack) | 55 | — |
| chain | 1 | 30 / 35 | 60 | — |
| unarmed | body | 20 / 25 | 50 | — |

The rule behind the table: one-handed 30 / 35, two-handed +5, dagger and fist −5, unarmed −10, whip 60 / 65; hit 55, 60 when blunt is the largest share, 50 for dagger, fist and unarmed. No melee archetype accepts a suppressor. Other melee noises: `noise.melee.clang` 70 (doc 05 §2.3); `noise.melee.block` 65; `noise.melee.parry` 70.

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
| grenade_frag, grenade_incendiary, grenade_concussion, grenade_flash, grenade_smoke, grenade_counter | — | — | `noise.weapon.throw` 25 at release; the detonation noise of §7.6 | 0 at release; the explosion flash of doc 03 §12.5 |

Other weapon noises: `noise.impact.projectile` at the impact cell, 50 for bullets and bolts, 45 for arrows and thrown weapons; `noise.impact.grapple` 50 at the anchor point when a grappling tool's line attaches (DECISIONS 95); `noise.impact.sonic` at the hit position of every hit whose `ev.damage_dealt` `per_type` holds `dmg.sonic` above 0, at `60 + floor_div(d_sonic_mhp, 1000)` dB (Canon §10.1; a 12,000 mHP sonic share gives 72 dB); `noise.weapon.reload` 35; `noise.weapon.dryfire` 30 (doc 05); `noise.weapon.jam` 40 (doc 05); `noise.weapon.swap` 30; `noise.weapon.throw` 25.

Example: an unsuppressed pistol (120 dB) 30 m down a corridor in Mythic Fantasy arrives at 60 dB against a gate of 23: margin 37, gain 5000, alarm class (§7.7). Suppressed (60 dB) it is inaudible at 30 m; at 15 m it arrives at 30 dB: margin 7, gain 2000, investigate class but below T2's 10 dB margin, so the listener needs a second shot to turn curious.

### 7.4 Spells and prayers

Grimoire spells take the form's cast noise from doc 06 §5.2 with doc 06's hushed and register riders; doc 09 supplies the ID `noise.spell.<school>` of the spell's school (doc 06 §7.1). Wands, texts and NPC spells with no form noise use the school default below. Impacts use doc 06's `50 + 10 × essences`. A cast's reaction class (§7.7) follows its emitted level after hushed and riders, not the school: 45 dB or less is curious, above 45 dB investigate; `noise.spell.surge` is always alarm.

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
| `noise.lock.snap` | 35 (a pick slips, §13.2) | `noise.lock.open` | 40 |
| `noise.lock.force` | 75 (Force check, doc 04) | `noise.hack.beep` | 30 per attempt |
| `noise.hack.fail` | 60 at the device | `noise.light.snuff` | 20 |
| `noise.light.switch` | 25 | `noise.light.power` | 45 |
| `noise.item.drop` | 35 | `noise.item.use` | 30 (drink, eat, read) |
| `noise.item.impact` | `40 + min(30, floor_div(mass_g, 100))` + surface | `noise.item.shatter_glass`, `noise.item.shatter_ceramic` | 70, 75 (doc 08) |
| `noise.body.fall` | 55 (death collapse) | `noise.body.drop` | 50 |
| `noise.body.thud` | 65 (thrown body) | `noise.hide.enter`, `noise.hide.exit` | 35 |
| `noise.takedown.lethal` | 35 | `noise.takedown.subdue` | 30 |

### 7.6 Voices, explosions, alarms, gadgets and environment

| ID | dB | ID | dB |
|---|---|---|---|
| `noise.voice.shout` | 85 (Errata emote) | `noise.voice.whistle` | 55 (Errata lure) |
| `noise.voice.callout` | 85 | `noise.voice.bark` | 50 |
| `noise.voice.scream` | 95 | `noise.voice.pain` | 60 |
| `noise.voice.death` | 70 | `noise.voice.breath` | 30 (doc 13 stalkers) |
| `noise.voice.groan` | 40 (downed, every 120 ticks) | `noise.voice.dog_bark` | 90 |
| `noise.voice.squawk` | 90 (tattlers; doc 12 `hazard.alarm` level) | `noise.explosion` | 150 (doc 03 `block.powder_keg`, doc 08 §5.6 `liq.nitro` detonation) |
| `noise.explosion.grenade` | 140 | `noise.explosion.concussion` | 150 |
| `noise.explosion.flash` | 160 | `noise.explosion.incendiary` | 110 |
| `noise.explosion.smoke` | 55 | `noise.explosion.surface` | 100 (liquid blasts, Canon §12.1) |
| `noise.alarm` | 90 | `noise.alarm.bell` | 100 |
| `noise.alarm.radio` | 50 | `noise.trap.trigger` | 60 |
| `noise.gadget.deploy` | 45 | `noise.gadget.noisemaker` | 70 every 60 ticks for 600 ticks |
| `noise.gadget.hum` | 40 every 60 ticks (drones, turrets idle) | `noise.env.distant` | 70 (doc 12 event) |
| `noise.env.debris` | 80 | `noise.env.steam` | 50 |
| `noise.env.fire` | 45 on `ev.fire_started` | | |

Doc 05 grenades map: frag `noise.explosion.grenade`, concussion `.concussion`, flash `.flash`, incendiary `.incendiary`, smoke `.smoke`, counter `.grenade`; the launcher blast is `.grenade`. Doc 08 §13.2 bombs name their own detonation noise from this table (`.grenade`, `.incendiary`, `.smoke`, `.flash`, or `noise.item.shatter_ceramic` for bombs that only shatter); doc 08 §5.2 liquid blasts use `.surface`. Doc 03 reads levels through `noise_level` for `ExplosionSpec.noise`. Mounts are cut (Canon §17.7), so the five mount noises of earlier drafts are retired and the `noise.mount.` family is empty.

### 7.7 Attribution and AI reactions

A noise is **attributed** to its emitter when the emitter is a living entity and the ID is a body noise, a weapon-use noise (`noise.melee.swing`, `noise.shot.*`, `noise.weapon.*`), a cast (`noise.spell.<school>`, `noise.prayer`), or one of `noise.lock.*`, `noise.hack.*`, `noise.door.*`, `noise.hide.*`, `noise.takedown.*`, `noise.light.*`, `noise.item.drop`, `noise.body.drop`. Every other noise is unattributed: impacts, explosions, alarms, gadgets, traps, block breaks, items landing, environment. A grenade thrower is never revealed by its explosion; the guards search the blast.

| Class (`aware.react.*`) | Noises | Hostile attributed emitter | Unattributed | Allied emitter |
|---|---|---|---|---|
| `aware.react.ignore` | `noise.voice.bark`, `noise.gadget.hum` | none | none | none |
| `aware.react.curious` | steps, landing below 60 dB, swim, climb, mantle, dodge, slide, reel, flap, swing, reload, swap, dryfire, throw, item drop and use, lock pick and beep, light snuff and switch, door, hide, takedowns, breath, groan, casts emitted at 45 dB or less | pair gain | disturbance gain | none |
| `aware.react.investigate` | landing ≥ 60 dB, splash, melee hit, clang, block, parry, impacts (sonic included), item impact and shatter, block hit, break, place, fall, erase, harvest, lock snap, open and force, hack fail, light power, suppressed and mundane shots, energy shots, casts emitted above 45 dB, spell impact, fizzle, pain, whistle, shout, squawk, trap trigger, deploy, noisemaker, smoke burst, steam, fire, distant | pair gain; T2 when margin ≥ 10 | disturbance gain; T2 when margin ≥ 10 | none |
| `aware.react.alarm` | powered unsuppressed shots, explosions (except smoke), surge, scream, death cry, dog bark, alarms, debris | pair gain; T2 always; doc 13 runs instead of walks | disturbance becomes searching (1800 ticks) | callout rule if the ally is alert (§6.6) |
| `aware.react.callout` | `noise.voice.callout` | — | — | §6.6 |

### 7.8 Genre sound and ambient

`sound_bp_eff` (§4.4, `SOUND_BP` in §15.1) rescales path cost and `AMBIENT_DB` (§15.1) masks quiet noises. The §7.2 heavy walk on metal with `curse.loud` 2 (63 dB) heard 12 m down a straight corridor arrives at 39 dB. In Mythic Fantasy (`SOUND_BP` 10000, ambient 20) a humanoid listener with HRG 40 and `PR = 24` against Sneak 10 has `od = floor_div(−14, 10) = −2`, so its gate is `max(20 − 2, 23 + 0) = 23`: the clumsy walker cannot be heard below the ambient mask. Margin 16, gain `clamp(17 × 250, 250, 5000) = 4250`, × 4000 = 1700 per step: the guard is curious after 2 steps (3400) and suspicious after 3 (`3400 + mul_bp(1700, 12500) = 5525`). In Cyberpunk (`SOUND_BP` 12000, ambient 35) the same step arrives at `L_eff = floor_div(630 − mul_bp(240, 12000), 10) = 34` against a gate of `max(18, 38) = 38`: inaudible.

### 7.9 Event conversion

`convert_events()` runs in the `sys.sound` prelude over this tick's events in `seq` order and appends requests through `emit_noise` (doc 03 §11.1):

| Event | Noise |
|---|---|
| `ev.footstep { entity, gait, material }` | `noise.step.<gait>` at the entity origin |
| `ev.entity_landed { entity, fall_mm, material }` | `noise.move.land` |
| `ev.block_changed` cause `broken`, `mined` or `unsupported` | `noise.block.break` by the old block's acoustic class, at the cell center |
| `ev.block_changed` cause `landed` (doc 03 §13.6) | `noise.block.fall` by the new block's acoustic class |
| `ev.block_changed` cause `placed` | `noise.block.place` |
| `ev.block_changed` cause `toggled` on a door or trapdoor state | `noise.door.open` when the open bit goes 0 → 1, `noise.door.close` when it goes 1 → 0, wood or metal by the door's acoustic class, −15 dB when `actor` is crouched |
| `ev.block_changed` cause `erased` (doc 03 §13.8) | `noise.block.erase` |
| `ev.block_changed` with any other cause (`explosion`, `fire`, `fell`, `froze`, `melted`, `toggled` on a non-door, `liquid`, `connected`, `reaction`, `grew`, `spread`, `decayed`, `reverted`, `expired`, `script`, `worldgen`) | nothing: explosions, fires and light actions emit their own noise |
| `ev.damage_dealt` with `dmg.sonic` above 0 in `per_type` | `noise.impact.sonic` at `pos` |
| `ev.entity_killed` of a creature tagged sapient or beast | `noise.voice.death` at the victim's eye and `noise.body.fall` at its origin, both capped at 24 dB after a takedown or with the killer's `quiet_kill` |
| `ev.fire_started` | `noise.env.fire` at the cell |

Every converted noise takes the event's `actor` or `entity` as its emitter, except a ghost actor, whose noises are emitted with no emitter and so stay unattributed (§14 rule 5); `noise.impact.sonic` has no emitter. Weapon, spell, item, lock, light, block-hit and voice noises are emitted directly by the acting system with `noise_level`. Doc 03 then propagates every request in order and hands `HeardNoise` records to `sys.hearing`.

---

## 8. Player feedback

The host sends each player, in its doc 16 snapshot (`STEALTH` section, doc 16 §6.8), a `StealthHud` for its own Erratum. Doc 17 renders it.

```rust
pub struct StealthHud {
    pub gem: u16,                                              // §8.1
    pub hidden: bool,                                          // st.hidden (§6.8)
    pub rings: Vec<(Fx3, u32 /* radius_mm */, u8 /* class */)>, // §8.2: live rings, newest first
    pub marks: Vec<(EntityId, u8 /* state */, u16 /* meter */, u8 /* flags */)>, // §8.3
    pub steps: Vec<Fx3 /* dir */>,                             // §8.3: live footstep indicators, newest first
    pub panel: Option<StealthPanel>,                           // §8.4
}
```

### 8.1 Visibility meter (light gem)

`gem = mul_bp(X_ref, LIGHT_BP[L_s])`, where `X_ref` is §3.1's exposure with `f_invis = 1000` when invisible and without the observer-specific `ghost_signal` term. It shows as five pips: `< 500` none (unseen), `< 1500` one, `< 3000` two, `< 5000` three, `< 8000` four, else five, plus the `st.hidden` eye icon. Example: the V1 Shade has `X_ref = 2772` and `gem = mul_bp(2772, 1700) = 471`: no pips. The same Shade with a lantern (`L_s` 13): `mul_bp(2772, 8900) = 2467`, two pips.

### 8.2 Noise ring

Every noise the player's Erratum emits draws a ring, live for 30 ticks from its emission tick, at the free-field radius where a reference humanoid hears it (Mark of the Ninja rule, R5 §9.2):

```
gate_ref  = max(20, amb(pos) + 3)                         // §4.4 amb at the emitter, weather front included
radius_mm = floor_div(max(0, L − gate_ref) × 5_000_000, sound_bp_eff(pos))
```

The 2 dB per meter matches doc 03's 20 ddB face-step cost. Examples: walking on stone in a medium set (52 dB) in Mythic Fantasy: `(52 − 23) × 500 = 14500` mm. The 63 dB heavy walk in Cyberpunk: `floor_div(25 × 5_000_000, 12000) = 10416` mm. Ring color follows the reaction class; allies' rings show at half opacity.

### 8.3 Per-enemy detection indicators

- **Baseline.** For each hostile within 48000 mm whose pair toward the player gained meter in the last 20 ticks, an arc on a ring around the crosshair points toward it, its length `meter / 10000` of a quarter circle, colored by state: white curious, yellow suspicious, orange searching or lost, red alert. A red "!" shows over an observer's head for 60 ticks after T3.
- **Divination 50** (doc 04): every hostile within 30000 mm shows its state icon toward you, through walls.
- **`item.trinket.warning_bell`** (doc 11): positions of hostiles at or above suspicious toward you within 15000 mm, through walls.
- **Footstep indicators** (Canon §6.4 row 34): a directional tick for each hostile footstep heard within `(8 + floor_div(HRG, 10) + Σ RowFlat(34))` m, from `HeardNoise.arrival_dir`; none under `st.deafened`. Each indicator is live for 30 ticks from the tick the step was heard, as rings are, and `steps` lists the live ones newest first, so doc 16 §6.8's cap of 4 keeps the 4 newest.
- **Co-op.** A red marker over an ally while any hostile holds it at alert; allies' arcs are not shown.

### 8.4 Action panels

`panel` carries every number that doc 17's lock, hack and pickpocket panels show (doc 17 §10.11, §10.12: `ui.hud.lockpick`, `ui.hud.hack`, `ui.hud.pickpocket`), so the owner's client never computes a chance or a channel time itself. On each snapshot the host fills it for the Erratum's **stealth target**, the first of:

1. the target of its running `StealthAction` of kinds 0 to 6 (§1.4);
2. the first door cell, container or entity that the crosshair ray (doc 17 §10.3: 48000 mm from the eye along the view direction) meets, when it is a locked non-quest mechanical lock within 1500 mm of the eye (§13.2) while the Erratum carries lockpicks or has Security 100, a device (electronic locks included) or Seize target of §13.4 or §13.5 within its `range` while it carries a hacking deck, or a target eligible for pickpocketing under §13.7;
3. none, giving `panel = None`.

```rust
pub enum StealthPanel {
    Lock  { target: u64, tier: u8, your_tier: u8, chance: u16, slip_bp: u16, ticks: u32, progress: i32, lock_hp: i32 },
    Hack  { target: u64, tier: u8, your_tier: u8, hack_fails: u8, lines: Vec<HackLine> },
    Steal { target: EntityId, lines: Vec<StealLine> },
}
pub struct HackLine  { pub verb: u8 /* 0 Unlock, 1 Loop, 2 Seize, 3 Disable, 4 Test, 5 Cut */, pub chance: u16, pub ticks: u32 }
pub struct StealLine { pub uid: u64, pub plant: bool, pub chance: u16, pub ticks: u32 }
```

`target` is the §1.4 `locks` key for a lock and the `hacked` key for a device. `Lock` serves a mechanical lock: `chance`, `slip_bp` and `ticks` are §13.2's values for this picker (10000, 0 and 60 at Security 100), and `progress` and `lock_hp` its pity state. `Hack` serves an electronic lock, a §13.4 device or a §13.5 Seize target: one line per verb the target offers in the §13.4 table, in ascending `verb`, with that verb's auto `chance` and `ticks` (§13.5's for Seize), plus the device's `hack_fails`. `Steal` lists every item §13.7 lets the thief take, in ascending `uid`, with its chance and ticks (`chance` 10000 against a sleeping or subdued target), then, from Pickpocket 50, each of the thief's own items with `plant = true` at DC − 10. `tier` and `your_tier` let doc 17 grey lines the actor cannot attempt; the lines are filled either way. Example: the §13.2 picker aiming at the `lock.t3` door gets `Lock { tier: 3, your_tier: 5, chance: 8400, slip_bp: 1250, ticks: 90, progress: 0, lock_hp: 28 }`.

---

## 9. Sneak attacks and takedowns

### 9.1 Eligibility

`sneak_eligible` (§6.8) is evaluated per (attacker, target) on the impact tick (Canon §10.5 step 2.4). Other observers' states never matter: while guard G1 holds Erratum A at alert, A's hit on guard G2, whose pair for A is unaware, is still a sneak attack and draws no crit roll (Canon §21 test 38). Hits that enter the pipeline after step 2 (DoT, shock chains, environmental hazards, falls, Canon §10.6) never take the multiplier. A sneak hit sets `DmgFlags` bit 1.

**Attacker.** In every stealth query `att` is the hit's owner (doc 05 `HitRecord.owner`): the Erratum for its weapons, spells, thrown items, placed traps and mines; the minion, turret or drone entity for minion and gadget attacks; the seized unit for its own attacks (§13.4). Doc 05 calls `sneak_eligible(att, tgt, now)` and then `sneak_mult(att, tgt, tier)` with this owner.

### 9.2 Multiplier

```
TIER_BP = { 1: 30000, 2: 22500, 3: 15000, 4: 12500 }
sneak_mult(att, tgt, tier) =
    m = TIER_BP[tier]
    if att is an Erratum: m += 20 × skill.sneak + 5000 × [skill.sneak ≥ 50] + 5000 × [talent.shade.backstab]
    if tgt.rank ∈ {rank.boss, rank.editor, rank.hunter}: m = min(m, 15000)
    if pair(tgt, att) has HUNTED and tgt lacks st.sleeping: m = min(m, HUNTED_CAP)     // HUNTED_CAP = 15000
    m
```

**Hunted cap (anti stealth-archer rule).** `HUNTED` is set on a pair when it enters lost, through T4 or through §5.4 unseen-damage contact, and cleared by T8 and T12. A hunting target is still below alert, so Canon §10.5 step 2.4 still makes the hit a sneak attack with no crit draw, but its multiplier is at most ×1.5. Without the cap a hidden shooter keeps a wounded target in lost and repeats the full multiplier on every shot, the Skyrim stealth-archer loop: a Sneak-100 sniper with `backstab` would land ×3.45 on every shot until the target died. With the cap the full multiplier lands on the opening hit, and again only after the target's search has run out (T8). Sleeping targets ignore the cap, since Canon `st.sleeping` says sneak attacks always apply.

| Attack | Tier |
|---|---|
| melee and ranged archetypes | doc 05 `sneak_tier`: 1 for dagger, blade_short, fist, unarmed; 2 for blade_long, axe, mace, spear, whip, chain, pistol, pistol_heavy, rifle_marksman, sniper, charge_rifle, crossbow, thrown; 3 for blade_great, axe_great, warhammer, halberd, quarterstaff, smg, rifle_auto, shotgun, launcher, beam |
| spells: bolt, beam, chain, orb, cone | 3 |
| spells: touch | 2 |
| spells: nova, aura, field, wall, rune; throwables, bombs, flasks; gadgets, turrets, traps | 4 |
| minions and seized units | their basic attack: melee 2, ranged 3 |

`affix.ambusher` and `item.exotic.quietus` add to the step-1 additive bucket on sneak hits (doc 11) and do not change `sneak_mult`. The multiplier enters Canon §10.5 at step 2.4, after precision and before elevation and flanking, and the hit draws no crit roll (Canon §3.4).

### 9.3 `ignore_armor`

A sneak hit carries `ignore_armor` (Canon §10.5 step 5) when the attacker has Sneak 100 or `talent.shade.unseen_blade`, and the target's rank is not boss, editor or hunter.

### 9.4 Takedowns

**Eligibility** (evaluated every tick while the prompt shows): attacker is an Erratum, grounded, not carrying; target has `comp.perception`, rank minion or standard (elite: lethal only, attacker Sneak ≥ 75), not tagged `swarm`; origins within 1500 mm; attacker outside the target's frontal cone (`ang_diff(target.yaw, bearing target→attacker) > 10923`, Canon step 2.4b) unless the target has `st.sleeping`, `st.stunned` or `st.knocked_down`, or the attacker is inside an enclosed hiding spot facing it (§10.2); the target's pair for the attacker is at most suspicious.

| Kind | Input | Ticks | Result |
|---|---|---|---|
| Lethal | hold `input.heavy` 10 ticks | 45 | the target takes a hit with flags `sneak` and `takedown` through Canon §10.6's lethal-takedown row: it enters §10.5 at step 7 as `dmg.true` equal to its current HP plus its remaining ward and skips crit, resistance, armor and the chip floor; an elite instead takes `mul_bp(max_hp_mhp, 5000)` plus its remaining ward through the same row, so it loses half its Max HP (Canon gaps G24), and gains `st.staggered` directly; `noise.takedown.lethal` 35 dB; the death cry and body fall are capped at 24 dB |
| Subdue | hold `input.interact` 10 ticks | 90 | `st.sleeping` by direct application (Canon §11.4) and `subdued_until = now + 3600`; the sleep is re-applied directly on every status pulse until then; `noise.takedown.subdue` 30 dB |

The ticks column counts from the tick the hold completes; the result applies on the last tick. During either, the target is held: `st.grappled` is applied directly on the first tick and re-applied directly every 15 ticks (Canon §2.3 refresh), and removed when the takedown ends or cancels. The attacker cannot act, move or change weapons. The takedown cancels with no effect when either takes damage, when the target's pair for the attacker becomes alert, or when the attacker's origin moves more than 500 mm from `StealthAction.origin`. Each observer other than the target that has a sight pass on the attacker on any check during the takedown gains a flat 4000 meter at once, once per takedown. A lethal takedown hit sets `DmgFlags` bit 1 (sneak) and bit 13 (takedown, doc 03 §14.2). Every completed takedown emits `ev.takedown` (§1.7). A subdued NPC wakes early on any damage (Canon `st.sleeping`) or after an ally's 60-tick wake interact, then enters searching at its own position with Vigilance. A subdue counts once per entity as a pacification defeat for doc 13 §11.1 XP: doc 13 reads its `ev.takedown` with `lethal` false and emits `ev.entity_pacified` with method 4 (doc 03 §14.3) the first time that entity is subdued, so pacifist tracking (DECISIONS 62; docs 04 and 14) counts a subdue as non-lethal even if the NPC later wakes. A lethal takedown that kills is a kill (`ev.entity_killed`) and grants doc 04's 1500 Sneak milli-XP.

### 9.5 Bosses

Ranks boss, editor and hunter: multiplier capped at 15000, no `ignore_armor`, no takedowns, no pickpocketing, immune to disguises. A boss inside its sealed arena and every hunter hold every Erratum at alert (doc 13 omniscience), so neither can be sneak-attacked after the fight starts; the opening hit on a mini-boss before its arena seals can be. Champions take full multipliers but no takedowns.

### 9.6 Worked examples

- **S1.** Doc 05's E3 dagger heavy (D1 = 25,775, flanking) by a Shade with Sneak 60 and `backstab`: `sneak_mult = 30000 + 1200 + 5000 + 5000 = 41200`. D2 = `mul_bp(mul_bp(25775, 41200), 11500) = 122,121`. Split pierce 73,273, slash 48,848; against AR 10 the DR is 233 and 486 Bp; **total 118,040 mHP**. With Sneak 0 the same hit is doc 05's 84,899.
- **S2.** Sneak 100 with `backstab`: `sneak_mult = 42000`, D2 = 124,493. Against AR 40 with armor the hit would be 109,355 (DR 912 and 1672); with `ignore_armor` it stays **124,493 mHP**.
- **S3.** The same Shade against a mini-boss before the arena seals: `min(42000, 15000) = 15000`, no `ignore_armor`.
- **S4.** A Gunslinger (Sneak 10) sniper shot, tier 2: `22500 + 200 = 22700`.
- **S5 (hunted cap).** That Gunslinger shoots an unaware elite from 40 m: ×22700. The elite survives; it had no sight pass and the shooter is beyond 3000 mm, so §5.4 makes its pair lost with `M = 9000`, `HUNTED` set and `lkp` quantized. The second shot 60 ticks later is still a sneak attack with no crit draw, at `min(22700, 15000) = 15000`. If the elite searches for its full 1200 ticks without a sight pass and returns to unaware (T8), `HUNTED` clears and the next opening shot is ×22700 again.

---

## 10. Hiding spots, bodies and corpse discovery

### 10.1 Hiding spot kinds

| Kind | Where | Effect |
|---|---|---|
| `aware.hide.enclosed` | closets, lockers, wardrobes, barrels, hay carts, dumpsters, sarcophagi: doc 07 props or placeables with `hide: enclosed`; capacity 1 Erratum or 2 bodies | `n = 0` for every observer except the one checking the spot |
| `aware.hide.under` | beds, tables, carts: doc 07 props with `hide: under`; capacity 1 Erratum or 1 body; entered like an enclosed spot (§10.2) in 20 ticks, the occupant lying in doc 03's 600 mm downed-height capsule at the prop cell and unable to move until it exits (20 ticks) | `f_frac` capped at 4000; `f_stance` 3500 |
| `aware.hide.foliage` | crouch-only conceal cells: tall grass, bushes, reeds | §3.2 conceal sums |
| `aware.hide.water` | eye submerged | stance 3000 plus 2 conceal per water block (Canon §12.2) |
| `aware.hide.ledge` | standing at least 2500 mm above an observer's eye | the `θ_v ≤ 10923` cone limit excludes steep angles |
| `aware.hide.shadow` | effective light ≤ 4 | §2.2, §3.3 posture rule |
| `aware.hide.crowd` | moving at walk or slower within a crowd | §15.2 |

`HideState { kind, occupant: Option<EntityId>, bodies: [Option<EntityId>; 2] }` lives in `StealthWorld.hides`, keyed by the prop's `Gidx` or `EntityId`.

### 10.2 Enclosed spots

Entering takes 30 ticks of `input.interact` within 1500 mm and emits `noise.hide.enter`; the occupant's origin snaps to the spot center. Exiting takes 20 ticks and emits `noise.hide.exit`. The occupant sees out through the slats and may lean, but no ray reaches it. From inside, it may take down an eligible target within 1500 mm in front of the spot regardless of the target's facing ("closet grab"): either kind, with 60 ticks in place of the §9.4 ticks, and the victim lands in the spot's free body slot when one is free.

**Checking.** An observer opens a spot (60 ticks, `noise.door.open` at the spot) when its search plan reaches the spot; when it is suspicious and its `lkp` lies within 1500 mm of the spot; and, on entering searching through T6 or T7, for every enclosed spot whose center lies within 1500 mm of `lkp`, in ascending distance then key, before P0's look-around. An Erratum seen entering a closet is therefore found. Under spots are checked the same way with a 60-tick crouch-look and no noise. An occupant is found: its pair becomes alert (`M = 10000`) and it gains `st.staggered` directly. Bodies inside are discovered (§10.4). A hound adjacent to an occupied spot for 60 ticks barks an alarm (§15.2).

### 10.3 Carrying and disposing of bodies

Corpses (`grid.ekind.corpse`), subdued NPCs and Errata Remains are picked up with `input.interact` (30 ticks). The body's mass (template mass, 75,000 g for humanoids and Remains) joins `carried_g`, so encumbrance applies; the carrier cannot attack or cast, takes the 15000 exposure term (§3.1), and drops with `input.interact` (20 ticks, `noise.body.drop`, 35 dB when crouched) or throws with `input.throw` (`noise.body.thud`). Disposal: an enclosed spot with a free body slot; liquid at least 2 blocks deep, where the body sinks and has `n = 0` for observers whose eye is out of the liquid; fire, which consumes a corpse after 600 ticks, lava after 60. Subdued NPCs drowned or burned die normally.

### 10.4 Corpse discovery

Each check, a sapient, hound or civilian observer evaluates the nearest undiscovered body within `R_far` that belonged to its own or a friendly or allied faction (civilians: any humanoid body), plus subdued allies. Its exposure is `X = 10000 → × 3500 → × 6000 → × f_size` and `V` follows §3.4 with the posture rule. `body_meter += mul_bp(profile.fill, V)`; it resets when the candidate changes or after 60 ticks without a sight pass. At 5000 the body is discovered:

- `discovered.insert(body)` and `ev.body_discovered` (§1.7); the observer's disturbance becomes searching at the body (1800 ticks) and it emits a body callout (`noise.voice.callout`, subject 0); civilians scream instead (`noise.voice.scream`, alarm class).
- Allies who hear set their disturbance to suspicious at the body and gain Vigilance; the doc 13 Director may order a bell or alarm (§6.6).
- A subdued body is woken (60 ticks) before the search starts.
- A cell holding at least 1000 ml of `liq.blood` within `R_near` whose center a §3.2 sight ray reaches gives a suspicious disturbance at that cell, once per cell per Folio (`blood_seen`). Doc 17's `access.gore` (DECISIONS 94) is presentation only: blood renders as ink by default, as red blood under `full` and not at all under `minimal`, and this rule reads the simulated `liq.blood` under every setting.

Examples: a corpse at 6 m in a humanoid's near cone at light 9: `X = 2100`, `V = mul_bp(2100, 5900) = 1239`, gain 309 per check, discovered on check 17 (51 ticks). At 14 m in the far cone at light 11: `V = 232`, gain 58, discovered on check 87 (261 ticks). At light 7 or darker in the far zone it is never discovered from there.

---

## 11. Disguises

### 11.1 Outfits and masks

An **outfit** is a `slot.body` item whose instance carries `outfit_of: fac.*` (doc 11 instance field). Sources: stripping a corpse or subdued NPC of faction F whose doc 13 template carries the `outfit` flag (`input.interact`, 120 ticks; the item copies the template's body armor archetype and weighs its `outfit_mass_g`), faction lockers and shops. **Coverage** (DECISIONS 64): every sapient native faction of every genre, that is every `fac.native.*` faction of doc 13 §12.1, can be impersonated, so doc 13 gives at least one template of each the `outfit` flag. Only native factions have outfits: `fac.editors` observers ignore disguises, and `fac.wild.*` members are not sapient (Canon §15). A **mask** is any `item.face.mask` in `slot.face` (Canon §13.2); respirators are not masks (doc 08 §4.3).

| Tier | Requirement | Suspicion factor |
|---|---|---|
| `disguise.tier.full` | outfit of F and a mask | 10000 |
| `disguise.tier.partial` | outfit of F, no mask | 20000 |
| `disguise.tier.none` | anything else | — |

A disguise of F is **active** toward observer `o` when `o` is of F or of a faction friendly or allied to F, `o`'s profile does not ignore disguises (hounds, editors, bosses), and `(wearer, F) ∉ blown`. While active, `o` treats the wearer as a member of F for every stance check, so it gets no sight fill; every other faction treats the wearer by its own stance toward F, so a rival gang attacks a disguised Erratum.

### 11.2 Suspicion

**Enforcers** are observers of rank elite or higher, role guardian, or tagged `enforcer` by doc 13; for a partial disguise every observer within 3000 mm also counts. Each enforcer check with a sight pass:

```
V_d    = V(o, t) computed with f_stance = 10000 and f_camo = 10000      // crouching is a trigger, not cover
s_gain = 200 → × V_d → × row40(t) → × TIER_FACTOR → × wary 12500                 // Canon §6.4 row 40, doc 04 fair_folk
S      = min(10000, S + s_gain)
```

Non-enforcers gain only from triggers. `S` decays 1000 per second after 120 ticks without gain (`last_s_gain`, accumulator `s_acc`, `k = 20`). No enforcer fill happens while `now < satisfied_until`.

### 11.3 Triggers

Each trigger adds its amount on every check of every F observer with a sight pass on the wearer while it holds (one-shot triggers once per occurrence):

| ID | Condition | Amount |
|---|---|---|
| `disguise.trigger.weapon_raised` | attack windup, ADS, heavy charge or aim | +4000 per check |
| `disguise.trigger.trespass` | inside a doc 12 room tagged `room.restricted` of F | +1500 per check |
| `disguise.trigger.climb` | climbing, mantling or reeling on a grappling line | +1500 per check |
| `disguise.trigger.sprint` | gait sprint, a sprint-slide included | +1000 per check |
| `disguise.trigger.company` | an undisguised Erratum within 3000 mm is also visible | +1000 per check |
| `disguise.trigger.crouch` | crouched and moving | +500 per check |
| `disguise.trigger.body` | carrying a body | +5000 per check |
| `disguise.trigger.crime` | lockpicking, hacking, a failed pickpocket, a takedown or snuffing a light | +6000 once |
| `disguise.trigger.attack` | any attack that hits anything | `S = 10000` |
| `disguise.trigger.strip` | changing outfit or mask | `S = 10000` |

Cameras of F never fill suspicion; a trigger they see adds 60 to their sight timer at once (§15.2).

### 11.4 Bluff and compromise

When `S ≥ 5000` a sapient observer walks to the wearer and challenges it (`noise.voice.bark`, `challenge = (wearer, now + 120)`). Before the deadline the wearer may answer with `input.interact` aimed at the challenger: doc 04's Deceive check, `rating = GUI + floor_div(skill.speech × 6, 10)` plus doc 04 §4.2 situational modifiers, `chance = clamp(5000 + (rating − DC) × 200, 500, 9500)` against `DC = 40 + 10 × bluffs(o)`, one success roll on keyed `rng.stealth` with `actor` = the wearer and `seq` 1006 (Canon §3.3: a disguise challenge is an AI detection roll and never draws on `rng.check`). Example: GUI 55 and Speech 20 give `rating = 67`; the first bluff succeeds at 9500, the second (DC 50) at 8400, the third (DC 60) at 6400. Success: `S = 0`, `satisfied_until = now + 1800`, `bluffs += 1`. Failure or silence: compromise. **Compromise** also happens when `S` reaches 10000: the pair becomes alert (`M = 10000`, T3), `ev.disguise_blown` is emitted (§1.7), and its callout carries `disguise_blown`; the observer and every F member that hears the callout add `(wearer, F)` to `blown` for the rest of the Folio.

### 11.5 Worked example

A full disguise (a full medium set whose body piece is the outfit, plus a mask: `f_armor` 11000), GUI 55 (`row40 = clamp(10000 − 1500, 3000, 15000) = 8500`), walking 5 m from an enforcer at light 12: `V_d = mul_bp(11000, 8200) = 9020`, `s_gain = 200 → 180 → 153` per check. The challenge comes at check 33 (99 ticks) and compromise at check 66 (198 ticks). Without a mask the gain is 306: challenge at check 17 (51 ticks), compromise at check 33 (99 ticks). Stepping into a dim alcove at light 5 cuts `V_d` to 3080 and the full-disguise gain to 51.

---

## 12. Distractions

| Distraction | Mechanism | Noise | Reaction |
|---|---|---|---|
| Thrown object | any item thrown with `input.throw` (doc 05 §9) lands | `noise.item.impact` or a shatter noise | unattributed; investigate class: a disturbance at the landing cell |
| Noisemaker | doc 07 gadget | `noise.gadget.noisemaker` every 60 ticks for 600 ticks | investigate |
| Whistle | `input.whistle` emote (doc 17), cooldown 120 ticks in `emote_ready` | `noise.voice.whistle` 55 dB | attributed investigate: pair gain, and suspicious toward the whistler's position when the margin is ≥ 10 dB |
| Shout | `input.shout` emote, cooldown 300 ticks in `emote_ready` | `noise.voice.shout` 85 dB | attributed investigate |
| Phantasm | doc 06 `summon.phantasm` | taunt and death noise per doc 06 | observers create pairs toward the decoy as a hostile |
| Food lure | a thrown `cat.food` item | item impact | beasts and hounds within 12000 mm with a sight pass or 4000 mm by scent set a disturbance at it, walk there and eat: `distracted_until = arrival tick + 300`, during which they skip scent tracking and their sight fill is × 5000 |
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

`LOCK_DC[t]`, `PICK_TICKS[t]` and `HACK_TICKS[t]` name the DC, pick-ticks and hack-ticks columns.

Tier is `lock_tier(d) = clamp(1 + floor_div(d − 1, 5), 1, 6)`, +1 for vaults, capped at 6. Kinds: `lock.kind.door` (`lock_hp` as listed), `lock.kind.container` (`lock_hp × 2`, R7 chest rule), `lock.kind.vault` (no pity), `lock.kind.quest` (key only, never picked). Skins: `lock.skin.mechanical` (Security, `item.tool.lockpicks`) and `lock.skin.electronic` (Hacking, the hacking deck, dependency techne). Both tools are carried, never consumed, worn or broken by an attempt (Canon §4.3, DECISIONS 65). `LockState { tier, kind, skin, locked, progress: i32, key_tags: [u32; 2], fails: u8 }`, where a `key_tag` of 0 means no key. An opener holding a matching key (`cat.key` with the same `key_tag`, doc 11) for every non-zero entry of `key_tags` opens it in 20 ticks with no roll (`ev.lock_opened` method 0). Doc 03 asks `door_locked(gidx)` before toggling a door; a locked door refuses and emits `noise.door.locked`.

```json
{ "id": "lock.t3", "dc": 55, "lock_hp": 28, "pick_ticks": 90, "hack_ticks": 120,
  "kinds": { "lock.kind.door": 10000, "lock.kind.container": 20000, "lock.kind.vault": 0 } }
```

**Placed locks.** Doc 12 and doc 11 attach locks by these rules; tier is always `lock_tier(d)` (+1 for vaults) and skin follows the TEC rule above unless the row names one:

| Kind | Placement (owner) | Skin | `key_tags` |
|---|---|---|---|
| `lock.kind.door` | door of a `Locked(k)` or `Locked(k1, k2)` edge, including the heist Treasury door (doc 12 §8.1, §9.2) | TEC rule | the edge's key items: `[k, 0]` or `[k1, k2]` |
| `lock.kind.container` | every chest of kind `loot.chest.locked` (doc 11 §9.6) | TEC rule | `[0, 0]` unless doc 11 binds a key |
| `lock.kind.door` | the `clearance` secret-exit door (doc 12 §17.4; Folio genre TEC ≥ 5) | `lock.skin.electronic`; doc 12 places it only where local TEC ≥ 5 | `[0, 0]`; doc 12 also unlocks it on the tick an Erasure hunter dies in the Folio |
| `lock.kind.vault`, `lock.kind.quest` | a vault or quest door that doc 12 names | TEC rule | as doc 12 names |

The `hack` verb seal of doc 12 §17.3 (`secret.hide.verb_lock`) is a §13.4 device, not a lock: one Unlock verb at device tier `lock_tier(d)` opens it, so it never needs local TEC ≥ 5. Its `prayer` and `spell` seals are doc 12 objects with no doc 09 lock.

### 13.2 Lockpicking

```
your_tier = min(5, 2 + floor_div(skill.security, 14)) + [skill.security ≥ 75]   // doc 04 Security 75 is the +1; attempt only if lock tier ≤ your_tier
rating    = floor_div(3 × skill.security, 2) + floor_div(d(SLT), 2) + TOOL_Q[lockpicks quality]
chance    = clamp(5000 + (rating − DC) × 200, 500, 9500)
ticks     = PICK_TICKS[tier] → × 8000 with talent.shade.light_fingers; + SLIP_TICKS (30) after a slip
draws     = keyed rng.stealth, seq 1000, on the attempt's first tick: success roll, then slip roll (always 2)
slip      → slip roll < slip_bp: noise.lock.snap (35 dB) at once, and the attempt lasts SLIP_TICKS longer; slip_bp = 2500, × 5000 at Security 25
success   → unlocked at the attempt's end; noise.lock.open; ev.lock_opened (method 1); doc 04 Security XP 150 × DC
failure   → progress += PITY[band]; unlocked at progress ≥ lock_hp (vaults excepted), which counts as picked for XP and ev.lock_opened; noise.lock.pick
TOOL_Q    = awful −10, poor −5, normal 0, good 3, excellent 6, masterwork 10, peerless 15
PITY      = Security 0–19: 3; 20–59: 4; 60–99: 6; 100: 8               // Barony lock health, R7 R-BA2
```

`your_tier` by Security: 0–13 → 2, 14–27 → 3, 28–41 → 4, 42–74 → 5, 75–100 → 6. Security 100 opens non-quest mechanical locks in 60 ticks with no picks and no draws. Every attempt emits `noise.lock.pick` (25 dB). The picker must carry `item.tool.lockpicks`; the slip replaces the pick break of earlier drafts, so a bad attempt costs time and noise, never the tool (Canon §4.3). The slip is the price of auto-resolving (§13.3). **Example:** Security 45, SLT 50, normal picks on a `lock.t3` door: `your_tier = 5`, rating `67 + 5 + 0 = 72`, chance 8400, 90 ticks per attempt (120 after a slip), slip chance 1250, pity 4 per failure, so the 7th failure (28 progress) opens it at the latest.

**Starting and cancelling.** An `input.interact` press while the crosshair ray (§8.4) meets a locked non-quest mechanical lock within 1500 mm of the actor's eye (measured to the door cell center or the container origin) starts one pick in the actor's `lock_mode` (§13.3) when the actor carries `item.tool.lockpicks` and the tier is within `your_tier`. An electronic lock is a §13.4 device: within the hack `range` the press opens doc 17's hack panel, and its `Hack { target, verb }` intent with the Unlock verb starts the attempt. An actor holding keys for every non-zero `key_tags` entry opens by key instead (20 ticks), and Security 100 opens a non-quest mechanical lock in 60 ticks. While `input.interact` stays held, the tick after an auto attempt fails starts the next one. Picks, key openings, device verbs (§13.4) and disarms (§13.6) cancel with no result when the actor takes damage, starts an attack or a cast, moves its origin more than 500 mm from `StealthAction.origin`, or abandons a manual attempt (`input.ui_back`, doc 17 §10.11). A cancelled attempt adds no pity, grants no XP and leaves `hack_fails` unchanged; draws and noises already made stay made. Seizes (§13.5) and steals (§13.7) keep their own cancel rules.

**Pacing (balance fix).** Doc 04's stealth build (Security 20, then 20 skill XP per Folio at MEM 40) enters Folios 1, 6, 11, 16, 21 and 25 with Security 20, 27, 33, 39, 44 and 47. With SLT 40, 42, 45, 47, 48, 50 and picks normal, normal, good, good, excellent, excellent, against that depth's lock tier (DC 25, 40, 55, 70, 85, 85) its `your_tier` is 3, 3, 4, 4, 5, 5, always enough, and its chance is 6000, 5200, 4800, 3800, 3200, 4200. A gate of `1 + floor_div(skill, 20)` with an unweighted rating would have left the thief Calling unable to attempt any lock from Folio 11 (gate 2 against tier 3) and at the 500 floor from Folio 16; the 3/2 skill weight and the `2 + skill / 14` gate keep lockpicking a skill game for the Calling built around it. A character with Security 0 still opens only tiers 1 and 2, almost always through pity.

### 13.3 Optional minigame for locks and hacks

Each party slot's `lock_mode` (default `auto`) is the value of that slot's latest `SetLockMode { manual }` intent (doc 17 §10.11, `access.lock_mode`), which the client sends at run start or join and on every change, so the host and the replay hold it (Canon §2.4). `manual = true` switches every lockpick attempt and every hack, Seize included, to the minigame: hacking uses the same model as lockpicking (DECISIONS 63). An attempt keeps the mode it started with, recorded in `StealthAction.manual`; a `SetLockMode` during an attempt applies from the next one. A manual attempt sets `2 + tier` pins in order, where `tier` is the lock tier for a lock, the device tier of §13.4 for a device verb or for a Seize on a doc 12 camera or turret, and 1, 2, 3 or 4 for a Seize on a minion, standard, elite or champion (§13.5). At start it draws one `range(40)` per pin (seq 1004, `actor` = the picker or hacker) as the pin targets. For each pin a cursor runs over positions `0..=39`, `c = (now − pin_start) mod 40`, one position per tick from the pin's first tick, `pin_start`; before `pin_start` the cursor waits and a press does nothing. Pressing `input.attack` sets the pin when `min(|c − target|, 40 − |c − target|) ≤ W`, with `W = clamp(1 + floor_div(rating − DC + 40, 10), 1, 8)` from the auto attempt's rating and DC (§13.2, §13.4, §13.5). A miss, or 120 ticks without a press, restarts that pin with `pin_start = now + 1 + SLIP_TICKS` and emits the attempt's noise again (`noise.lock.pick`, or `noise.hack.beep` for a hack); a miss draws nothing. The third miss fails the attempt: a lockpick failure adds pity progress as in §13.2, a hack failure is a failed hack of §13.4 (pity for an electronic lock, `noise.hack.fail`, `hack_fails += 1`), and a failed Seize applies §13.5's failure. Setting every pin succeeds with no success roll and no slip roll and grants the same XP and events as an auto success. Draws: exactly `2 + tier`. The `ticks` formulas and their multipliers time auto attempts only; a manual attempt lasts as long as its pins take and keeps the auto attempt's `your_tier` gate and its range, tool, sight-pass and cancel rules. Security 100's 60-tick opening and key openings skip the minigame. Commands come from the replay log, so the minigame is deterministic. The owner's client computes the pin targets from the same keyed draws, since it holds the run seed (doc 16 `RunHeader`), so doc 17's dial needs no extra snapshot data.

**Auto-resolve has a cost** in both skills, and playing the minigame avoids it: an auto attempt's success chance is at most 9500, a failure costs the attempt's time (and for a hack a `noise.hack.fail` and a step toward the device's alarm, §13.4), and an auto pick also risks a slip (§13.2). A manual attempt draws no success roll; the player's timing alone decides it.

Example: rating 50 against DC 55 gives `W = 4`, a window of 9 of the 40 positions; the §13.2 rating 72 gives `W = 6`. A manual Loop on the §16 heist camera (device tier 3, rating 80, DC 55) has 5 pins and `W = 7`.

### 13.4 Electronic locks and devices (TECHNE skins)

Hacking replaces Security on electronic skins. The deck (`item.tool.hacking_deck`) is a gadget component with dependency techne: its `TOOL_Q` bonus is multiplied by its Contradiction potency, and doc 07 rolls its failure spark (Canon §5.5). The hacker must carry it; no hack consumes or wears it (Canon §4.3). A Disconnected deck cannot start a hack (Canon §5.5). While the deck's failure chance is above 0, doc 07 rolls its failure once as an attempt starts (doc 07 §11.1 rule 4, keyed `rng.combat`); a failure sparks the hacker and ends the attempt on that tick with no other effect: no `rng.stealth` draw at seq 1001 or 1004, no `noise.hack.beep` or `noise.hack.fail`, and `hack_fails` and pity progress unchanged. This holds for device verbs, Seizes (§13.5) and manual attempts (§13.3) alike.

```
your_tier = min(6, 2 + floor_div(skill.hacking, 14))                 // attempt only if the device tier ≤ your_tier
rating    = floor_div(3 × skill.hacking, 2) + floor_div(d(LOG), 2) + mul_bp(TOOL_Q[deck quality], potency)
DC        = LOCK_DC[device tier]; device tier = the lock's tier for an electronic lock, lock_tier(d) (§13.1) for every other device
chance    = clamp(5000 + (rating − DC) × 200, 500, 9500);  draws: seq 1001, 1 draw on the attempt's last tick (auto; manual per §13.3)
ticks     = HACK_TICKS[tier] → × (10000 − 50 × skill.hacking) → × 8000 trait.origin.wired → × 7500 talent.netrunner.backdoor
range     = 2500 mm; cameras 10000 mm at Hacking 25; every hack 15000 mm with talent.netrunner.remote_access
success   → the device row below; hack_fails[device] = 0; ev.device_hacked, plus ev.lock_opened (method 2) per lock opened; doc 04 Hacking XP 150 × DC
failure   → an electronic lock gains pity progress as in §13.2 (PITY by Hacking band); noise.hack.fail at the device;
            hack_fails[device] += 1; at 3 the device's faction raises an alarm (§6.6) with subject = the hacker and the count resets to 0
```

The hacker picks a verb with the intent `Hack { target, verb }` from doc 17's hack panel. Loop and the device verbs write a `HackState` (kinds 0, 2, 3) into `StealthWorld.hacked`; **Seize** applies Canon `st.hacked` through §13.5. Cameras, turrets, drones and constructs are the four `st.hacked` targets of this chapter.

| Device | Verbs: success effect |
|---|---|
| electronic lock | Unlock: unlocked |
| camera | (doc 12 `hazard.camera`) **Loop:** sees nothing for 1800 ticks, until Folio exit at Hacking 100. **Seize:** `st.hacked` (§13.5); while it lasts the camera raises no alarms and applies `st.marked` from the hacker (8 s, refreshed each check) to every hostile of the hacker it holds with `V ≥ 800`, so the party sees them through walls |
| turret | **Loop:** safe mode, it does not fire, for 1800 ticks, until Folio exit at Hacking 100. **Seize:** `st.hacked`: it fights for the hacker's faction (Canon §11.1), counts as `fac.errata` for friendly fire (Canon §17.5) and never counts toward doc 07's per-player gadget limits |
| drone | **Seize:** `st.hacked`, as a turret; its spotlight (§15.2) then lights only the hacker's hostiles |
| alarm panel | **Disable:** until Folio exit, devices in its room still detect and emit their alarm noise, but their `raise_alarm` calls do nothing else. **Test:** `raise_alarm` at the panel with no subject (§12) |
| laser emitter pair | **Disable:** the beam is off until Folio exit |
| fuse box | **Cut:** power cut (§2.4), `power_cuts[room] = now + 1800` |
| terminal | **Unlock:** every electronic lock in its room opens; doc 04 `data_thief` |

Loop and device verbs channel `ticks` at `range`; Seize channels as §13.5. Seize on a camera or turret placed by doc 12 uses the device-tier DC above; Seize on a drone or any unit with a doc 13 rank uses §13.5's rank DC. Example: Hacking 40, LOG 55, normal deck, `lock.t3`: rating `60 + 7 + 0 = 67`, chance 7400, ticks `120 → 96 → 76` with `wired`; 1 draw at seq 1001.

### 13.5 Seizing units (`st.hacked`)

Targets: cameras and turrets (§13.4), drones, and NPCs with body plan `body.construct` or tag `electronic` (Canon §11.1, doc 13 §5.7); never ranks boss, editor or hunter (Canon §11.4), never Errata gadgets (doc 07 §11.1), never an Erratum with `talent.netrunner.firewall`. The hacker channels within 8000 mm (15000 mm with `remote_access`) while its eye keeps a sight pass to the target: `ticks = mul_bp(120, 10000 − 50 × skill.hacking)` with the §13.4 multipliers; damage to either or a lost sight pass cancels it. Then one draw (seq 1001), or §13.3's minigame in manual mode:

```
DC     = {minion 25, standard 40, elite 55, champion 70} + floor_div(d, 2)      // device-tier DC for doc 12 cameras and turrets (§13.4)
rating = §13.4 rating + 10 × [target's pair for the hacker below suspicious] + 20 × [target has st.shocked or st.stunned]
success → st.hacked by direct application: mul_bp(360, clamp(floor_div(1_000_000, thr_mind), 2500, 15000)) × rank control (Canon §11.4)
          × 15000 with talent.netrunner.puppet_master; until Folio exit at Hacking 100 (non-boss); HackState kind 1
failure → the target's pair for the hacker becomes alert (a camera instead gains cam_seen += 60, §15.2)
```

A successful Seize emits `ev.device_hacked` (§1.7); at Hacking 100 against a non-boss it is also a pacification (doc 04 §6.1), which doc 13 reports as `ev.entity_pacified` method 3. `thr_mind` is Canon §6.4 row 24 from the target's effective RSV. A device with no doc 13 template (doc 12 cameras and turrets) has `RSV = min(150, 40 + floor_div(3 × (d − 1), 2))` and rank standard. While `st.hacked` lasts the unit's pairs reset (T12) and it perceives the hacker's hostiles as its hostiles. At most 1 seized unit, +1 at Hacking 75, +1 with `talent.netrunner.botnet`; a new Seize past the limit ends the oldest `st.hacked` it applied. Loops and device verbs do not count. Example: Hacking 50, LOG 50, an unaware standard drone in Folio 10: rating `75 + 5 + 10 = 90`, DC 45, chance 9500, 90 ticks; its RSV is `40 + floor_div(27, 2) = 53`, mind threshold 126, so `st.hacked` lasts `mul_bp(360, 7936) = 285` ticks. A Folio-12 doc 12 camera has RSV `40 + 16 = 56`, threshold 132, and stays seized `mul_bp(360, 7575) = 272` ticks.

### 13.6 Traps: detection and disarming

Hidden objects are doc 12 hidden hazards and secrets, doc 11 mimics, doc 06 runes and §15.2 wards and tripwires. **Passive detection** is Canon §6.4 row 35: each Erratum within 3000 mm rolls once per object on ticks where `(t + observer_phase) % 60 == 0`, `observer_phase = (EntityId & 0xFF_FFFF_FFFF) % 60`, `seq` = the object's index in ascending ID order among objects in range, one draw each (doc 04 `gumshoe` adds `RowFlat(35, 1000)` and never changes this cadence). **Active search** (`StealthAction` kind 17): while crouched and still for at least 30 ticks, the same detection ticks roll for objects within 5000 mm at `min(9000, mul_bp(row35, 15000))`; row 35 itself is unchanged. Doc 06's sight essence reveals objects in its area without a roll. A success sets the object's party mask in `revealed` (the whole party sees it) and grants the detector doc 04's 3000 Awareness milli-XP; the first such reveal of an object emits `ev.object_revealed` (§1.7). NPCs never detect Errata traps; `fac.editors` members detect them at 5000 Bp under the same cadence, keyed with their own `EntityId`.

**Disarm** a revealed trap within 1500 mm: mechanical and arcane with Security (lockpicks or toolkit), electronic with Hacking (deck). The tool is carried and never consumed.

```
trap_dc = min(100, 25 + 2 × d)                    // Folio 1: 27, Folio 10: 45, Folio 25: 75
rating  = floor_div(3 × skill, 2) + floor_div(d(SLT or LOG), 2) + TOOL_Q
chance  = clamp(5000 + (rating − trap_dc) × 200, 500, 9500); ticks 60
draws   = seq 1002 on the disarm's last tick: success roll, then trigger roll (always 2)
success → disarmed; XP 150 × DC; at Security 50 a worldgen spike, blade or dart trap also yields one block.trap.spikes
          cat.placeable item at ilvl = the Folio index (doc 07 §10.4, Canon §13.1); wards, alarms, tripwires and explosive traps yield nothing
failure → the trap fires on the disarmer when the trigger roll < 5000
```

Example: Security 30, SLT 45 against a Folio-6 dart trap: rating `45 + 2 = 47`, DC 37, chance 7000.

### 13.7 Pickpocketing

Targets: sapient humanoid NPCs with inventory (doc 13 `carry`, picked-up weapons, keys), not boss ranks, not `in_combat`, with their pair for the thief at most curious; the thief within 1500 mm and outside the frontal cone (10923 Bam). A target with `st.sleeping` or subdued is robbed with no draw.

**Input.** Tapping `input.interact` (released within 10 ticks) on an eligible target opens doc 17's pickpocket panel, which lists the target's stealable items with chance and ticks; choosing one sends the intent `Steal { target, uid }`, or `Plant { target, uid }` for an item of the thief's (Pickpocket 50). Holding `input.interact` 10 ticks on a takedown-eligible target starts a subdue instead (§9.4). The panel never pauses the simulation; the channel cancels when the thief leaves 1500 mm or enters the frontal cone.

```
rating = floor_div(3 × skill.pickpocket, 2) + floor_div(d(SLT), 2) + 10 × [ang_diff > 16384, directly behind] + 5 × [L_s ≤ 4] + 10 × [in a crowd]
DC     = 20 + PR(target) + min(30, floor_div(mass_g, 250)) + 10 × [pair curious] + 15 × [equipped item; Pickpocket 75 only]
chance = clamp(5000 + (rating − DC) × 200, 500, 9500); draw seq 1003 at completion
ticks  = min(180, 60 + floor_div(mass_g, 100)) → × 7500 at Pickpocket 25 → × 8000 light_fingers
```

Success moves the item, emits `ev.item_stolen` for a steal (§1.7) and grants doc 04 XP. Failure: the target's pair becomes alert (suspicious with `M = 7000` at Pickpocket 100); doc 13 applies its witnessed reputation rule; a `fac.neutral` vendor turns hostile for 600 ticks and screams; a disguise takes the crime trigger. **Planting** uses DC − 10; a primed throwable detonates on its fuse at the target's origin. Example: Pickpocket 20, SLT 45, directly behind in darkness, a 300 g pouch on a Folio-4 guard: rating `30 + 2 + 10 + 5 = 47`, DC `20 + 24 + 1 = 45`, chance 5400, 63 ticks.

---

## 14. Co-op stealth

1. **Per pair.** Meters, states, `lkp` and sneak eligibility are per (observer, Erratum). One Erratum being seen never makes another seen. When an observer turns alert on one Erratum, its pair toward every other living, non-ghost Erratum within `R_far` (created if absent) gains a flat 2500 at once, with no multipliers (it is scanning).
2. **Party-wide outcomes.** The Unseen bonus (Canon §8) fails when any Erratum is held at alert at any point in the Folio, and `run.note.silent` (doc 14 §3.5) when that happens before the Rubric completes: both read `folio_alerted` (§1.4). `revealed` hidden objects are party-wide.
3. **Light and noise are shared physics.** An ally's torch lights you (§2.1); allies see your noise rings at half opacity.
4. **Downed** Errata are perceived with stance 3500 and emit `noise.voice.groan` every 120 ticks; reviving is not silent (`noise.item.use` every 60 ticks).
5. **Ghosts** cannot be perceived (Canon §17.6). Doors and switches a ghost toggles make unattributed noise, so a ghost can draw guards away; its `st.marked` marks hostiles for the party.
6. **Hiding spots** hold one Erratum. **Disguises** are per Erratum; an undisguised ally nearby is a trigger.
7. **Minions** are perceived like Errata and leave scent. Their hits use their own pair eligibility (§9.2 tier by attack).
8. Pings, voice chat and asynchronous traces (DECISIONS 96: opt-in marginal notes and ink-stain death echoes, presentation only, doc 14 §8.1) never enter the simulation: no observer perceives a trace and no trace emits noise. Emotes (`whistle`, `shout`) do enter it.

### 14.5 Invaders

An invader (Canon §17.9) is player-controlled `fac.editors` at rank hunter: it has no meters and cannot be sneak-attacked unless asleep. Which Errata its client receives is doc 16's PvP culling alone (doc 16 §6.6 rule 2, the anti-wallhack test for both directions); this chapter applies no interest filter of its own, and the invader perception profile of earlier drafts is retired (Canon gaps, open question P1). This chapter's actions are closed to the invader, whose Editor kit binds only attacks (doc 13 §10.7): it never takes anyone down, picks, hacks, disarms, steals or plants, so it never opens a container or takes an item (Canon §17.9). NPC hunters stay omniscient (doc 13 §10.5).

---

## 15. Genre stealth flavors

### 15.1 Genre table

| Genre | `AMBIENT_DB` | `SOUND_BP` | Camo primary, secondary | Signature stealth features |
|---|---|---|---|---|
| mythic_fantasy | 20 | 10000 | foliage, earth | torch-bearing patrols, hounds, ward glyphs on vaults |
| gothic_horror | 15 | 9000 | shadow, earth | ambient light 2, bell towers (`noise.alarm.bell`), undead with 10 m dark vision, hounds |
| cosmic_horror | 10 | 8000 | shadow, earth | sound-hunting horrors, seeing lenses (cameras), silence |
| space_opera | 30 | 11000 | urban, shadow | cameras, turrets, drones, radio squads, lasers |
| cyberpunk | 35 | 12000 | neon, urban | cameras, lasers, drones, crowds, radio, electronic locks |
| post_apocalypse | 20 | 10000 | earth, urban | dogs, can tripwires, scavenger camps |
| weird_west | 25 | 9000 | earth, pale | dogs, shotgun tripwires, townsfolk crowds |
| hardboiled_noir | 30 | 13000 | shadow, urban | rain fronts, crowds, cameras, mob outfits for disguise |
| wuxia | 20 | 10000 | foliage, earth | rooftop ledges (VERVE jumps), market crowds, sect sentries |
| steampunk | 40 | 12500 | urban, earth | machinery masking, automatons to hack, klaxons, crowds |
| fairy_tale | 20 | 9500 | foliage, pale | tattler birds, wolves (hounds), fairy-ring wards |
| mythic_antiquity | 25 | 10000 | pale, earth | ambient light 7, oracle wards, temple hounds |
| oneiric | 15 | 8000 | neon, pale | sleepwalker crowds, lucid wards |
| redacted | 30 | 6000 | pale, urban | compliance cameras, lasers, drones, ambient light 9, agent outfits |
| unwritten | 0 | 5000 | pale, pale | absolute silence (`SOUND_BP` 5000), no disguises against Editors |
| ur_text | 20 | 10000 | earth, pale | every device and observer type, as doc 12 places them from every roster; no genre-specific masking |

`SOUND_BP` multiplies path attenuation (§4.4): below 10000 sound carries farther (stone crypts, eldritch hush, sterile Redaction corridors, the silent Unwritten), above 10000 the world muffles it (rain-slick Noir streets, Cyberpunk crowds, Steampunk machinery). Inside a seam zone (Canon §5.8) `AMBIENT_DB` and `SOUND_BP` take the larger of the two genres' values, the camouflage background is the Folio genre's (Canon §5.8 skins), and both genres' devices appear.

**Fused Folios** (Second Edition, DECISIONS 101; Canon gaps G28). In a Folio written in two fused genres A and B, A being the first genre of doc 12's recorded pair, every position outside bleed and seam zones reads `AMBIENT_DB = floor_div(AMBIENT_DB[A] + AMBIENT_DB[B], 2)` and `SOUND_BP = floor_div(SOUND_BP[A] + SOUND_BP[B], 2)`, as the fused axioms are averaged. Its camouflage background is A's primary and B's primary, or A's primary and A's secondary when the two primaries are equal; its `fog_q` (§3.2) is the fused Folio's GPV fog as doc 12 records it; and both genres' devices and signature features appear. Example: Cyberpunk (35, 12000) fused with Gothic Horror (15, 9000) reads ambient 25 and `SOUND_BP` 10500 with the background neon and shadow, so a shadow cloak there gets 7500.

```json
{ "genre": "genre.hardboiled_noir", "ambient_db": 30, "sound_bp": 13000,
  "camo": ["aware.camo.shadow", "aware.camo.urban"] }
```

### 15.2 Devices and special observers

- **Cameras** (`hazard.camera`, doc 12: half-angle 8192, 16000 mm, sweep ±4096 per 360 ticks). `p = (now + phase) mod 360`; `yaw = base + (p < 180 ? −4096 + floor_div(8192 × p, 180) : 4096 − floor_div(8192 × (p − 180), 180))`, so p 0 → −4096, 90 → 0, 180 → +4096, 359 → −4050. Each check with `V ≥ 800` on a hostile (no `opp`; `ghost_signal` applies), `cam_seen += 3`; otherwise `cam_seen = max(0, cam_seen − 3)`. At 60 (doc 12's 60 ticks of sight) the camera raises its faction's alarm at the target (§6.6), resets to 0 and sets `cam_hold = (target, now + 300)`, during which its yaw tracks the target's bearing instead of sweeping. A looped camera does nothing; a seized one marks for the hacker instead (§13.4). Yaw arithmetic wraps: `yaw = base.wrapping_add(offset as u16)`. Examples: standing still at light 10, `V = 4020`: alarm in 60 ticks. Crouched still at light 4, `V = 726`: never. Night optics (dark vision 8000 mm) at local TEC ≥ 8.
- **Lasers** (techne skin of `hazard.alarm`): a segment up to 8000 mm between two emitters at height 300, 900 or 1500 mm. Every 3 ticks each non-ghost body capsule is tested with doc 02 `closest_seg_seg`; a capsule within its radius trips the alarm with that subject. Invisibility does not help. A crouched body (1200 mm) passes under a 1500 mm beam; a 300 mm beam can be jumped. Errata see the beam where it crosses smoke or light ≤ 9. Hacking an emitter disables the pair.
- **Tripwires** (mundane skin): laser geometry, visible only once revealed (§13.6), disarmed with Security.
- **Wards** (arcana or mythos skin): any hostile body center within 4000 mm of the glyph trips the alarm (chime, `noise.alarm`), ignoring light, invisibility and disguises. Wards are hidden objects; Security disarms them at `trap_dc`, or a doc 06 unmaking hit dispels them.
- **Dogs** (`aware.profile.hound`): scent (§4.5), hearing threshold 10, disguise-proof. While any hound pair is at least suspicious, it emits `noise.voice.dog_bark` every 120 ticks and raises its faction's alarm (600-tick cooldown). Food lures distract them (§12).
- **Sound-hunting horrors** (`aware.profile.horror`): no sight; hearing can reach alert; tremor; every `lkp` exact. Standing still and silent is safe; a noisemaker sends them running.
- **Crowds**: three or more `fac.neutral` civilians within 3000 mm of an Erratum walking or slower with no weapon raised give `f_gear × 4000` against observers farther than 6000 mm, and +10 to pickpocketing. Civilians fill their meters only from crimes (the §11.3 crime, attack, body and weapon triggers, a discovered body, an alarm-class noise); at 10000 they scream and flee (doc 13).
- **Tattlers** (birds and critters, the dependency-none skin of `hazard.alarm` in Fairy Tale and Mythic Fantasy): on its period-3 check, an Erratum within 6000 mm that is neither still nor crouch-walking at ≤ 2000 mm/s makes it call `raise_alarm` with subject 0 (§6.6: `noise.voice.squawk` 90 dB, unattributed) and fly off, despawning after 120 ticks.
- **Turrets** use the turret profile and fire on alert. While no pair is alert a turret sweeps with the camera formula and its own phase; while one is, it tracks that subject's bearing. **Drones** project their spotlight: while a drone holds a sight pass on a target, that target's `L_s` is at least 12 for every observer (§2.1 `spot`).

### 15.3 Time of day and weather

Each Folio has one time of day and one weather state (DECISIONS 99), rolled and stored by doc 12 (Canon gaps G26). Time of day changes only the sky channel's source level (doc 03 §10.2), so it reaches stealth through `light_at` and this chapter adds no term for it: a night Folio darkens its sky-lit cells and leaves fixture-lit interiors as they are. The weather state `W` supplies five stealth terms. Each applies only where `outdoor(c)` holds, `c` being the cell `fx_floor_block(p)` of the point read and `outdoor(c)` its doc 03 §10.2 `sky_src` bit, so roofed interiors keep their genre values:

```
wx_fog(p)   = W.fog        if outdoor(c) else 0        // whole fog points, added to fog_q at the observer's eye (§3.2)
wx_sound(p) = W.sound_bp   if outdoor(c) else 10000    // Bp, one mul_bp in sound_bp_eff at the noise position (§4.4)
wx_amb(p)   = W.ambient_db if outdoor(c) else 0        // dB, added to amb at the listener's origin (§4.4)
wx_wash(p)  = W.precip == 1 and outdoor(c)             // scent life 1800 at the mark cell (§4.5)
wx_snow(p)  = W.snow == 1 and outdoor(c)               // the feet cell reads noise.surface.snow (§7.2)
```

Until doc 12 stores a weather state, `W` is none and every term is neutral (0, 10000, 0, false, false). A doc 12 weather front (`gpv.evt.weather_front`) keeps its own room terms in §3.2, §4.4 and §4.5, applied in addition, each as written there. A weather state's light, fire and surface-wetness terms belong to docs 03 and 12; they reach this chapter only through `light_at`, liquids in the feet cell (§7.2) and `ev.fire_started`. The §8.2 ring reads `amb` and `sound_bp_eff`, so it shows the weather too.

Example: a rain state with `fog` 0, `sound_bp` 11000, `ambient_db` 5 and `precip` 1 over a Mythic Fantasy courtyard. An outdoor humanoid listener with HRG 40 has ambient `20 + 5 = 25` and a gate of `max(20, 28) = 28` for an unattributed noise; a 45 dB item impact heard at 25 dB rescales to `floor_div(450 − mul_bp(200, 11000), 10) = 23`, below the gate. A scent mark laid there lasts 1800 ticks. Under the courtyard's roofed arcade none of it applies. In a snowy state (`snow` 1) an unarmored walk on outdoor soil is `40 − 8 = 32` dB.

---

## 16. Worked example: a two-player heist

Cyberpunk, Folio 12 (locks and devices at tier 3, DC 55; guards `PR = 20 + 16 = 36`), ambient 35. Shade A (Sneak 60, Security 45) and Netrunner B (Hacking 50, LOG 50) approach a vault room watched by one camera and two guards.

1. B loops the camera from 10 m (Hacking 25 range): device tier `lock_tier(12) = 3`, DC 55, rating `75 + 5 = 80`, chance 9500, one draw at seq 1001, `HACK_TICKS[3] = 120 → × 7500 = 90` ticks. It succeeds; the camera sees nothing for 1800 ticks.
2. A crouch-walks on metal in a light set: `25 + 10 + 3 − 3 = 35` dB. Guard G1 is 8 m away: doc 03 delivers 19 dB, the Cyberpunk rescale gives `floor_div(350 − mul_bp(160, 12000), 10) = 15`, below the gate `max(17 + 2, 38 + 2) = 40` (`thr = 20 − floor_div(16, 5) = 17` for the Folio-12 guard's HRG 56; `od = floor_div(60 − 36, 10) = 2`): silent.
3. G2 notices A at the edge of its far cone; A breaks line of sight and ducks into a locker 4 m from G2's `lkp` (§10.2), outside the 1500 mm first-check radius. G2 turns suspicious, finds nothing at `lkp` after 120 ticks of look-around, and searches. The locker enters its plan as a hiding-spot candidate (weight 3); when G2's three `rng.ai` draws pick other points, G2 returns to unaware with Vigilance after 1200 ticks, and when they pick the locker, A is found (§10.2).
4. Later A exits the locker (20 ticks) and takes G1 down lethally from behind (hold 10 ticks, then 45 ticks; G1's pair for A is unaware). The death noises are capped at 24 dB. A carries G1 into the locker (2-body capacity).
5. A (SLT 50) picks the treasury door, a `lock.kind.door` at `lock.t3`: `your_tier = 5`, rating 72, chance 8400 per 90-tick attempt, and pity opens it by the 7th failure at the latest. Nobody reached alert: both players earn the Unseen bonus, and doc 12's heist challenge succeeds.

---

## Exports

Namespaces owned by Canon §19: `noise.`, `aware.`, `disguise.`, `lock.`. Every ID below is defined in this chapter.

| Group | IDs |
|---|---|
| Awareness states (§6.1) | `aware.unaware`, `aware.curious`, `aware.suspicious`, `aware.searching`, `aware.lost`, `aware.alert` |
| Perception profiles (§4.1) | `aware.profile.humanoid`, `aware.profile.sentry`, `aware.profile.stalker`, `aware.profile.beast`, `aware.profile.hound`, `aware.profile.undead`, `aware.profile.construct`, `aware.profile.flier`, `aware.profile.swarm`, `aware.profile.horror`, `aware.profile.civilian`, `aware.profile.boss`, `aware.profile.editor`, `aware.profile.camera`, `aware.profile.turret`, `aware.profile.drone`, `aware.profile.ward`, `aware.profile.laser`, `aware.profile.tattler`, `aware.profile.erratum` |
| Camouflage families (§3.1) | `aware.camo.shadow`, `aware.camo.earth`, `aware.camo.foliage`, `aware.camo.urban`, `aware.camo.pale`, `aware.camo.neon` |
| Hiding spot kinds (§10.1) | `aware.hide.enclosed`, `aware.hide.under`, `aware.hide.foliage`, `aware.hide.water`, `aware.hide.ledge`, `aware.hide.shadow`, `aware.hide.crowd` |
| Reaction classes (§7.7) | `aware.react.ignore`, `aware.react.curious`, `aware.react.investigate`, `aware.react.alarm`, `aware.react.callout` |
| Footsteps and movement (§7.2) | `noise.step.crouch`, `noise.step.walk`, `noise.step.sprint`, `noise.step.crawl`, `noise.move.land`, `noise.move.swim`, `noise.move.climb`, `noise.move.mantle`, `noise.move.slide`, `noise.move.reel`, `noise.move.dodge`, `noise.move.splash`, `noise.move.flap` |
| Surfaces (§7.2) | `noise.surface.metal`, `noise.surface.glass`, `noise.surface.water`, `noise.surface.stone`, `noise.surface.ice`, `noise.surface.bone`, `noise.surface.wood`, `noise.surface.synthetic`, `noise.surface.mud`, `noise.surface.dirt`, `noise.surface.sand`, `noise.surface.soft`, `noise.surface.foliage`, `noise.surface.grass`, `noise.surface.paper`, `noise.surface.cloth`, `noise.surface.snow` |
| Melee (§7.3) | `noise.melee.swing`, `noise.melee.hit`, `noise.melee.clang`, `noise.melee.block`, `noise.melee.parry` |
| Shots and impacts (§7.3) | `noise.shot`, `noise.shot.suppressed`, `noise.shot.mundane`, `noise.shot.energy`, `noise.impact.projectile`, `noise.impact.sonic`, `noise.impact.grapple` |
| Weapon handling (§7.3) | `noise.weapon.reload`, `noise.weapon.dryfire`, `noise.weapon.jam`, `noise.weapon.swap`, `noise.weapon.throw` |
| Spells and prayers (§7.4) | `noise.spell.evocation`, `noise.spell.abjuration`, `noise.spell.alteration`, `noise.spell.conjuration`, `noise.spell.illusion`, `noise.spell.restoration`, `noise.spell.divination`, `noise.spell.impact`, `noise.spell.fizzle`, `noise.spell.surge`, `noise.prayer` |
| Blocks and nodes (§7.5) | `noise.block.break`, `noise.block.hit`, `noise.block.place`, `noise.block.fall`, `noise.block.erase`, `noise.node.harvest` |
| Doors, locks, hacking, lights (§7.5) | `noise.door.open`, `noise.door.close`, `noise.door.locked`, `noise.lock.pick`, `noise.lock.snap`, `noise.lock.open`, `noise.lock.force`, `noise.hack.beep`, `noise.hack.fail`, `noise.light.snuff`, `noise.light.switch`, `noise.light.power` |
| Items, bodies, hiding, takedowns (§7.5) | `noise.item.drop`, `noise.item.use`, `noise.item.impact`, `noise.item.shatter_glass`, `noise.item.shatter_ceramic`, `noise.body.fall`, `noise.body.drop`, `noise.body.thud`, `noise.hide.enter`, `noise.hide.exit`, `noise.takedown.lethal`, `noise.takedown.subdue` |
| Voices (§7.6) | `noise.voice.shout`, `noise.voice.whistle`, `noise.voice.callout`, `noise.voice.bark`, `noise.voice.scream`, `noise.voice.pain`, `noise.voice.death`, `noise.voice.breath`, `noise.voice.groan`, `noise.voice.dog_bark`, `noise.voice.squawk` |
| Explosions (§7.6) | `noise.explosion`, `noise.explosion.grenade`, `noise.explosion.concussion`, `noise.explosion.flash`, `noise.explosion.incendiary`, `noise.explosion.smoke`, `noise.explosion.surface` |
| Alarms, traps, gadgets, environment (§7.6) | `noise.alarm`, `noise.alarm.bell`, `noise.alarm.radio`, `noise.trap.trigger`, `noise.gadget.deploy`, `noise.gadget.noisemaker`, `noise.gadget.hum`, `noise.env.distant`, `noise.env.debris`, `noise.env.steam`, `noise.env.fire` |
| Disguise tiers (§11.1) | `disguise.tier.full`, `disguise.tier.partial`, `disguise.tier.none` |
| Disguise triggers (§11.3) | `disguise.trigger.weapon_raised`, `disguise.trigger.trespass`, `disguise.trigger.climb`, `disguise.trigger.sprint`, `disguise.trigger.company`, `disguise.trigger.crouch`, `disguise.trigger.body`, `disguise.trigger.crime`, `disguise.trigger.attack`, `disguise.trigger.strip` |
| Lock tiers (§13.1) | `lock.t1`, `lock.t2`, `lock.t3`, `lock.t4`, `lock.t5`, `lock.t6` |
| Lock kinds and skins (§13.1) | `lock.kind.door`, `lock.kind.container`, `lock.kind.vault`, `lock.kind.quest`, `lock.skin.mechanical`, `lock.skin.electronic` |

```json
{ "id": "noise.step.walk", "base_db": 40, "adds_surface": 1, "body_noise": 1, "attributed": 1,
  "react": "aware.react.curious", "stride_mm": 700 }
```

**Constants:** `RAY_BUDGET` 1536, `ALERT_KEEP_V` 300, `HUNTED_CAP` 15000, `SCENT_OWNER_RANGE` 64000 mm, `SLIP_TICKS` 30, `slip_bp` 2500, the §6.3 timers (`WARY_FOLIO` included), `LIGHT_BP`, `TIER_BP`, `TOOL_Q`, `PITY`, `LOCK_DC`, `PICK_TICKS`, `HACK_TICKS`, `AMBIENT_DB` and `SOUND_BP` (§15.1).

**Functions and data:** `stealth_light`, `noise_level`, `convert_events`, `aware_state` (doc 05's `aware_of`), `sneak_eligible`, `sneak_mult(att, tgt, tier)`, `grants_ignore_armor`, `sight_pass`, `erratum_sees`, `awareness_above_unaware`, `door_locked`, `raise_alarm`, `search_plan`, `sound_bp_eff`, `lock_tier`, `StealthHud`, `StealthPanel`, `HackLine`, `StealLine`, `PerceptionState`, `StealthWorld`, `StealthAction`, `HackState`; the weather terms `wx_fog`, `wx_sound`, `wx_amb`, `wx_wash`, `wx_snow` (§15.3). Intents consumed: `Steal`, `Plant`, `Hack { target, verb }`, and `SetLockMode { manual }` setting each slot's `lock_mode` for picks and hacks (§13.3). Events produced: the eight doc 03 §14.3 events listed in §1.7. Behavior of `st.deafened`, `st.marked`, `st.invisible`, `st.hidden`, `st.hacked` (Canon §11.1 owner 09).

## Interface assumptions

| Doc | This chapter assumes |
|---|---|
| 02 | a save section `stealth` for `StealthWorld` in its declared field order (flash ring, radio queue, per-owner scent trails and the `alarmed` set included); `closest_seg_seg` for lasers; doc 17's `SetLockMode` intent in the replay log |
| 03 | (present on disk) a `sys.sound` prelude hook calling `convert_events()`; `HeardNoise.req` (§11.5), so `sys.hearing` reads source position, source level, emitter and callout payload; `DmgFlags` bit 13 `takedown`; `grid.cause.erased` (§13.8); `ev.footstep` with gait `grid.gait.crawl` every 500 mm while downed; `door_locked(gidx)` consulted before door toggles; `cell_conceal_parts` for thermal rays; `push_contacts` (§5.14) read by `sys.hearing`; `ExplosionSpec.noise` levels read through `noise_level`; the eight §1.7 events registered in §14.3 with doc 09 as producer, with `device: u64` on `ev.device_hacked` and `kind` 0 of `ev.alarm_raised` as a creature alarm (hound, civilian, bell or klaxon); light overrides and doc 06 shadow areas applied inside every light read (§10.6); sight per Canon gaps G23: `raycast_blocks` against each cell's micro-voxel boxes (§5.18) and `conceal_at` read through the §4.4 fill rule (§7.6); the per-cell `sky_src` bit (§10.2) readable by `sys.perception`, `sys.sound` and `sys.hearing` (present); once Canon gaps G27 (doc 03 G31) lands, the sprint-slide and grappling-line states readable for §3.1, and the slide start, the line's attach point and its reel ticks reported so the moving system emits `noise.move.slide`, `noise.impact.grapple` and `noise.move.reel` through `noise_level` |
| 04 | the §5.2 XP events as written plus 750 Sneak milli-XP for engineered-noise kills (§12); the §4.2 Deceive chance drawn on `rng.stealth` seq 1006 for disguise challenges, never on `rng.check` (Canon §3.3, triage C-20); `RowFlat(31)`, `RowFlat(33)`, `RowFlat(34)`, `RowFlat(35)` terms; `light_fingers` applying to auto lockpicking and pickpocketing only; `aug.silent_pads` (§9.4) and the `body.serpentine` plan (§13) lowering footsteps by 6 dB, as §7.1 rows 7 and 8 apply them; Security 25 halving the slip chance (§13.2) in place of a lockpick break chance (triage C-31); §6.1 pacification for an entity's first subdue and for Hacking-100 seizes; talent and trait IDs cited here |
| 05 | calls `sneak_mult(att, tgt, tier)` (three arguments, for the boss and hunted caps) with `att` = the hit's owner; emits shot, swing, impact, block-hit, reload and jam noise through `noise_level`; a `suppressor` part tag for powered ranged skins only; `st.grappled` applied and refreshed directly for a takedown's duration; the `IGNORE_ARMOR` flag from `grants_ignore_armor` |
| 06 | form cast noise and school; shadow fields and `linger` areas reported by `in_shadow_area`, which doc 03 §10.6 applies inside `light_at`, so this chapter adds no shadow term of its own (§2.1); `summon.phantasm` as a hostile-perceived decoy; the Divination see-invisible hook |
| 07 | material `acoustic` class, one `noise.surface.*` value per material, which §7.2's material lists repeat verbatim (doc 07 §2.4 is consumer data for these IDs; their definitions and dB values are §7.2's); `block.tall_grass` as the grass override; block `lightsrc` (`flame`, `electric`, `arcane`) and lit/unlit states; prop field `hide` (`enclosed`, `under`); crouch-only conceal flag; `block.light_switch`, `block.fuse_box`; lockpick, toolkit and hacking-deck quality and no durability (Canon §4.3); noisemaker gadget; `block.trap.spikes` as the Security-50 disarm item of worldgen spike, blade and dart traps (§13.6, doc 07 §10.4, present); the deck's failure spark rolled as a hack starts (doc 07 §11.1 rule 4, present), after which §13.4 ends the attempt; seized hostile turrets and drones fighting for the hacker outside Errata gadget limits (doc 07 §11.1 rule 2, present); no snow material or block (doc 07 §2.4), so only §15.3's snowy-weather override yields `noise.surface.snow` |
| 08 | phosphor marking; respirator hearing penalty; mind vision radius; blood volume per cell; night-eye magnitude in mm; `liq.mud` as the mud surface override (§7.2) |
| 11 | instance field `outfit_of`; `item.face.mask`; `item.tool.lockpicks`, `item.tool.toolkit` and `item.tool.hacking_deck` as carried tools that no attempt consumes, so one of each suffices (Canon §4.3); door keys in `cat.key` with `key_tag`; tool qualities; `loot.chest.locked` chests carry a `lock.kind.container` lock (§13.1) |
| 12 | lock tiers and skins at placement, with kinds and `key_tags` per the §13.1 placed-locks table (two-key `Locked(k1, k2)` doors fill both entries; the `clearance` door only where local TEC ≥ 5); the `hack` verb seal as a §13.4 device; tag `room.restricted` with its faction (doc 12 §12, present); biome `camo`; `hazard.alarm` skins by dependency; laser heights; hiding props in stealth rooms; one fuse box per electric room; a room id per cell for power cuts and weather fronts; the faction recorded on every placed device; the `lattice.marg.silent` flag readable by `sound_bp_eff`; the per-Folio weather state of DECISIONS 99 stored with the Folio plan, carrying the five stealth fields of Canon gaps G26 (`fog`, `sound_bp`, `ambient_db`, `precip`, `snow`) that §15.3 reads (until it exists every term is neutral); a Second Edition Folio's fused pair (A, B) in a fixed recorded order and its fused GPV fog, `floor_div(fog_A + fog_B, 2)` as proposed, for §15.1; the heist Treasury door opening only while `folio_alerted` is false (doc 12 §6.3, present) |
| 13 | template field `perception`; tags `enforcer`, `flier`, `breathless`, `electronic`; faction flag `radio`; template `outfit` flag and `outfit_mass_g`, set on at least one template of every `fac.native.*` faction (DECISIONS 64, §11.1); investigate and search actions using §6.5 and §6.7 with `min(3, n)` draws; relight, bell and wake behaviors; reputation event `pickpocket_caught`; Director `alarm_raised`; subdue as a pacification defeat, emitted as `ev.entity_pacified` method 4 on an entity's first subdue (read from `ev.takedown` with `lethal` false), and a Hacking-100 Seize as method 3; each Folio's primary native faction (default device faction) |
| 14 | `run.note.silent` reads `!folio_alerted` on the tick the Rubric completes (doc 14 §3.5, present); pacifist tracking (DECISIONS 62, doc 14 §10, present) counts `party_kills` from kill credit and `party_pacified` from `ev.entity_pacified` for `ach.pacifist`: subdue takedowns (§9.4) and Hacking-100 seizes (§13.5) are this chapter's non-lethal routes, and a lethal takedown that kills is a kill |
| 16 | `StealthHud` in each player's snapshot with its caps (present), extended with the §8.4 `panel`; `steps` entries live 30 ticks, newest first (§8.3, as doc 16's interface row asks); §6.6 rule 2 as the only rule deciding which Errata an invader's client receives, so doc 16's O1 closes with no doc 09 filter (Canon gaps P1); hot and warm migrations carrying `StealthWorld` and every `comp.perception` live, and a cold one restoring the Folio-entry section 12 (§9.3, DECISIONS 93) |
| 17 | inputs `input.hold_breath`, `input.whistle`, `input.shout` as simulation intents (present); `SetLockMode { manual }` per slot (`access.lock_mode`, §10.11, present) governing lockpicks and every hack, Seize included (§13.3); the auto lock panel showing the slip chance beside the success chance (present); the manual dial for hacks as for locks (`2 + tier` pins, misses as cracks, a 30-tick pin restart, present); `ui.hud.lockpick`, `ui.hud.hack` and `ui.hud.pickpocket` (§10.11, §10.12, present) reading their numbers from `StealthHud.panel` (§8.4) and sending `Hack { target, verb }`, `Steal { target, uid }` and `Plant { target, uid }`; the §10.3 crosshair ray as the stealth target ray; `access.gore` as presentation only; HUD rendering of §8 |
| 18 | text keys and audio for every `noise.*` this chapter exports; HUD strings |

## Canon gaps

Every gap of the canon-1.4 draft is closed by canon-1.11 or by the canon gap triage: G1 (G-051), G2 (G-196), G3 (G-021), G4 (G-092), G5 (G-184: doc 03 §14.3 registers the eight events of §1.7), G6 (G-020), G7, G8, G12, G18, G20 and G21 (G-203: rulings owned here; doc 07 adds no snow block), G9 (G-023, C-20), G10 (G-172, C-30: mounts cut), G11 (G-124), G13 (G-111), G14 (doc 07 §11.1 rule 2 now agrees), G15 and G16 (triage §4.1: present in doc 03), G17 (G-069), G19 and G22 (G-194: every ID this chapter uses is defined by its owner). Seven items remain open:

- **G23. Sight through partial cells.** Canon §4.4 says a cell blocks sight when its mask holds at least 48 micro-voxels, and otherwise scales its concealment by the fill fraction. Read literally, a closed door, whose panel fills far fewer than 48 micro-voxels, would never block sight. This chapter keeps doc 03's geometric test (§3.2): a sight ray is blocked where `raycast_blocks` meets one of a cell's occupied boxes, and a crossed cell whose boxes the ray misses adds its fill-scaled `cell_conceal`. For full and empty cells both readings agree. Requested canon §4.4 wording: "A cell blocks light when `popcount(mask) >= 48`. A sight ray (doc 09) is blocked by a cell's occupied micro-voxel boxes; a crossed cell whose boxes it misses scales its concealment by the fill fraction."
- **G24. Elite takedowns.** Canon §10.6's lethal-takedown row sets the amount to the target's HP plus ward. §9.4 keeps elites alive: a lethal takedown on a `rank.elite` target deals `mul_bp(max_hp_mhp, 5000)` plus its ward through the same row and staggers it. Requested addition to the row's Notes cell: "against `rank.elite`, `mul_bp(max HP, 5000)` plus ward instead (doc 09 §9.4)".
- **G25. `in_combat` wording.** Canon §2.3 defines `in_combat` through a hostile AI that "held awareness `alerted`", an interim state name; triage G-196 made §6.1's six states the only awareness IDs, and every chapter uses them. This chapter reads `alerted` there as `aware.alert`: a subject is in combat while some hostile pair toward it is at `aware.alert` or was within the last 300 ticks. Requested canon §2.3 wording: "or any hostile AI held awareness `aware.alert` (doc 09) toward `e` within that window".
- **G26. Time of day and weather** (DECISIONS 99; doc 12 G19, doc 03 G31). Canon-1.11 has no time-of-day or weather records. §15.3 reads five stealth fields of the Folio's weather state on sky-source cells and nothing of time of day, which reaches stealth through the sky channel; until doc 12 stores a state every term is neutral. Requested canon text, a new §12.4: "**Time of day and weather.** Each Folio rolls one time of day from its genre's table and one weather state (doc 12). Time of day sets the sky channel's source level (doc 03 §10.2). A weather state's terms apply only at cells whose doc 03 `sky_src` bit is set. Its stealth fields are `fog` (whole fog points added to the genre fog, §12.2), `sound_bp` (multiplies sound path attenuation), `ambient_db` (added to the ambient mask), `precip` (0 or 1: scent lasts 1800 ticks) and `snow` (0 or 1: footsteps read `noise.surface.snow`); doc 09 §15.3 applies them. Core states: clear 0, 10000, 0, 0, 0; rain 0, 11000, 5, 1, 0; storm 1, 12000, 10, 1, 0; mist 1, 10000, 0, 0, 0; snow 1, 11000, 0, 1, 1; dust 1, 11000, 5, 0, 0." Storm matches the room-scoped weather front (fog +1, × 12000, +10 dB, scent 1800).
- **G27. Movement kit** (DECISIONS 95; doc 03 G31). Canon-1.11 §4.2 has no sprint-slide or grappling line. This chapter already rates both: a slide is stance 5500 at gait sprint and emits `noise.move.slide` (45 dB + surface) once; reeling on a line is stance 12000 and emits `noise.move.reel` (30 dB) every 30 ticks; the line's attach emits `noise.impact.grapple` (50 dB) at the anchor; the disguise sprint and climb triggers cover them. The 2,000 mm mantle needs nothing new: `noise.move.mantle` and the climbing-or-mantling stance row apply at any height. Requested canon §4.2 sentence: "A sprint-slide holds the capsule at crouch height for its duration, and reeling on a grappling line counts as airborne; their noises are doc 09's `noise.move.slide`, `noise.move.reel` and `noise.impact.grapple`."
- **G28. Fused Folios** (DECISIONS 101; doc 12 G19). Canon §5.8 gives every position one `local_genre`; a Second Edition Folio has two genres. §15.1 averages `AMBIENT_DB` and `SOUND_BP` with `floor_div(a + b, 2)`, as the fused axioms are averaged, and builds the camouflage background from both primaries, reading the pair in doc 12's recorded order. Requested canon §5.8 bullet: "**Fused Folios.** A Second Edition Folio is written in an ordered genre pair (A, B) that doc 12 records. Outside bleed and seam zones `local_axiom(pos, a) = floor_div(axiom_A + axiom_B, 2)`, both genres' rosters, World Laws and skins are active, and entities of either genre count as native; each chapter's per-genre table reads the pair as that chapter states (doc 09 §15.1)."
- **Retired mount noises.** Canon §17.7's retired-ID list does not yet name the `noise.mount.` family (walk, gallop, hover, glide, summon) that §7.6 retired. Requested addition to §17.7: "`noise.mount.` (doc 09)".

One product question is open:

- **P1. Invader replication.** Earlier drafts of §14.5 filtered which Errata an invader's client receives by the invader's own perception (`V ≥ 300`, a heard noise within 60 ticks, or a `fac.editors` mark); doc 16 §6.6 rule 2 tests mutual visibility with four rays, a 16000 mm near radius and a 30-tick grace, and no triage ruling covers the difference (doc 16 O1). Applied default: doc 16's rule alone, because doc 16 owns interest management and its rule is symmetric, tested and standard practice; this chapter's filter and its invader profile are retired. Its cost: within 16000 mm a modified invader client receives Errata hidden behind walls or in closets. Alternatives: (a) doc 16's rule alone (applied); (b) this chapter's filter for the Errata-to-invader direction and doc 16's rule for the other; (c) both tests required for the Errata-to-invader direction; (d) doc 16's rule plus one exception: an Erratum inside an occupied enclosed hiding spot (§10.2) is never sent.

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
12. **Hearing.** 63 dB heard at 39 dB after 12 m: Mythic Fantasy gate 23 (`od = −2` cannot lower the ambient mask), gain 1700 per step for PR 24 against Sneak 10, curious after 2 steps and suspicious after 3; Cyberpunk `L_eff = 34` against gate 38, inaudible.
13. **Rescale.** A 45 dB noise heard at 25 dB: Hardboiled Noir 19 dB, The Redaction 33 dB.
14. **Gate.** Humanoid, HRG 40, Steampunk: gate 43; deafened: 50 in The Unwritten. A footstep with `od = −3`: 43 in Steampunk, 17 in The Unwritten; with `od = +2`: 45 and 22. A 45 dB noise heard at 25 dB in Mythic Fantasy under a weather front rescales to 21 dB.
15. **Shots.** Pistol 120 dB at 30 m, Mythic Fantasy: gain 5000, alarm class. Suppressed (60 dB): inaudible at 30 m; at 15 m gain 2000 and no T2.
16. **Sound cap.** Unlimited footsteps raise a humanoid pair to at most 7499; a horror pair reaches 10000 and alert.
17. **Unseen hit.** A hit from 40 m with no sight pass leaves the pair lost with `M ≥ 9000` and `lkp` quantized to a 4000 mm grid center; the next hit is still sneak-eligible.
18. **Lost.** An alert pair with no contact turns lost exactly 90 ticks after its last contact; `st.blinded` turns it lost on application.
19. **Callout.** A listener 25 m away hears 35 dB: suspicious, `M = 7000`, Vigilance to `now + 5400`; at `V = 300` its gain is 140 per check and it alerts in 22 checks.
20. **Search plan.** Center (100.5, 8, 60.5), velocity east: P1 (106.5, 8, 60.5); ring k 0, r 4000 at (100.5, 8, 56.5); k 2, r 8000 at (92.5, 8, 60.5). The plan draws nothing; doc 13 draws exactly `min(3, n)` `rng.ai` values, 3 for this plan.
21. **Scent.** `s0 = 10000` laid at tick 1000: 5000 at tick 2800, usable at 4240, lost at 4241; soaked (`s0 = 5000`) lost at 3881.
22. **Sneak multiplier.** Shade Sneak 60 with `backstab`, tier 1: 41200; the S1 hit totals 118,040 mHP; at Sneak 0 doc 05's 84,899; against a mini-boss 15000; S2 with `ignore_armor` 124,493.
23. **Takedown.** A lethal takedown on a standard leaves it at 0 HP with death noises at 24 dB; it is refused on ranks boss, editor, hunter and champion; damage to the attacker cancels it. A completed takedown emits exactly one `ev.takedown`; an entity's first subdue makes doc 13 emit one `ev.entity_pacified` with method 4, and a second subdue of it emits none.
24. **Noise levels.** Heavy walk on metal with `curse.loud` 2: 63 dB; Shade crouch on stone: 25 dB; 4000 mm drop on stone in medium armor: 72 dB.
25. **Ring.** 52 dB in Mythic Fantasy: 14500 mm; 63 dB in Cyberpunk: 10416 mm.
26. **Gem.** V1 Shade: 471, no pips; with a lantern: 2467, two pips.
27. **Bodies.** 6 m, near cone, light 9: discovered on check 17; 14 m, far cone, light 11: check 87; far cone at light 7: never.
28. **Disguise.** Full, GUI 55, enforcer at 5 m, light 12: 153 per check, challenge on check 33, compromise on check 66; partial: 306, checks 17 and 33; light 5: 51.
29. **Lockpicking.** Security 45, SLT 50, normal picks, `lock.t3`: `your_tier` 5, rating 72, chance 8400, 90 ticks, slip chance 1250, opens by the 7th failure; each attempt draws exactly 2 values at seq 1000 on its first tick (3 under `avatar_of_fate`); a slipped attempt lasts 120 ticks and emits `noise.lock.snap` at 35 dB; after 50 attempts the picker still carries the same lockpicks.
30. **Minigame.** Rating 50 against DC 55: `W = 4`; rating 72: `W = 6`. With `W = 2`, cursor 39 against target 1 sets the pin (circular distance 2). A `lock.t3` manual attempt draws exactly 5 values at seq 1004 whatever its misses, and a miss at tick `t` restarts the pin with `pin_start = t + 31`. A manual Loop on a tier-3 camera draws 5 values at seq 1004 and none at seq 1001; its third miss adds 1 to `hack_fails`.
31. **Device hack.** Hacking 40, LOG 55, normal deck, `lock.t3`, `wired`: rating 67, chance 7400, 76 ticks; 1 draw at seq 1001. With a Contradicted deck whose doc 07 failure roll fails, the attempt ends on its first tick with the spark only: 0 draws at seq 1001, no `noise.hack.fail` and `hack_fails` unchanged.
32. **Construct hack.** Hacking 50, LOG 50, unaware standard drone, Folio 10: rating 90, DC 45, chance 9500, 90 ticks, `st.hacked` 285 ticks; a hack on a boss, or on an Errata gadget, is rejected with no draw.
33. **Traps.** `trap_dc` 27, 45, 75 at Folios 1, 10, 25; Security 30, SLT 45 at Folio 6: rating 47, chance 7000; 2 draws at seq 1002. Passive detection rolls on ticks with `(t + phase) % 60 == 0` with or without `gumshoe`; a successful disarm of a dart trap at Security 50 yields one `block.trap.spikes` item, and of a ward nothing.
34. **Pickpocket.** The §13.7 example (Pickpocket 20) gives rating 47, DC 45, chance 5400 and 63 ticks; a sleeping target is robbed with no draw.
35. **Camera.** Standing still at light 10 (`V = 4020`) raises the alarm 60 ticks after first sight; crouched still at light 4 (`V = 726`) never; sweep offsets at p 0, 90, 180, 359 are −4096, 0, 4096, −4050.
36. **Lock tiers.** Depth 1, 5, 6, 10, 11, 25, 26 gives tiers 1, 1, 2, 2, 3, 5, 6; no electronic lock is placed where local TEC ≤ 4.
37. **Determinism.** Perception, hearing, search plans and corpse discovery make no draws; a lockpick attempt's draws are identical whether 0 or 50 detection rolls happened that tick for the same actor.
38. **Co-op.** Canon §21 test 38 holds. When G1 turns alert on A, G1's pair for B (within `R_far`) gains exactly 2500.
39. **Hidden.** An Erratum that no observer holds at alert has `st.hidden`; one held at alert by any observer does not, and a ghost never has it.
40. **Budget.** With 600 due rays of demand above 1536 in one tick, the deferred observers run first on the next tick in ascending `EntityId`.
41. **Invader.** An invader has no pairs; `sneak_eligible(erratum, invader, now)` is false unless the invader has `st.sleeping`; its `Hack`, `Steal` and `Plant` intents and its `input.interact` on a lock start no `StealthAction`; its snapshots carry no `StealthHud`, and doc 16 §6.6 rule 2 alone decides which Errata they hold.
42. **Hunted cap.** S5: the first shot on the unaware elite uses 22700; after §5.4 leaves its pair lost, the second shot is a sneak attack with no crit draw at 15000; after T8 the next shot uses 22700 again; the same lost elite under `st.sleeping` takes 22700. Canon §21 test 38 still holds.
43. **Distance clamp.** A target 3000 mm from the SGT 45 guard with `θ_h` between `near_half` and `far_half` is in the far zone with `f_zone = 2000` and `f_dist = 10000`, never above.
44. **Scent trail.** An Erratum walking for 7200 ticks holds exactly 120 marks, the oldest between 3570 and 3599 ticks old, so no mark younger than the 3600-tick lifetime is ever overwritten; with 256 NPC layers also laying marks, its trail is unchanged. A hound 3000 mm from a mark laid 3000 ticks earlier (`s = 1667`) turns its pair suspicious with `lkp` at the mark cell center.
45. **Security gate.** `your_tier` for Security 0, 13, 14, 41, 42, 74, 75 is 2, 2, 3, 4, 5, 5, 6; for Hacking 41, 42, 56 it is 4, 5, 6. Security 33, SLT 45, good picks against `lock.t3` gives rating 54 and chance 4800.
46. **Seize.** A Folio-12 doc 12 camera has RSV 56 and mind threshold 132, and stays seized 272 ticks; while seized, a hostile of the hacker in its cone with `V ≥ 800` gains `st.marked` from the hacker on each check and no alarm is raised. A looped camera never raises an alarm while looped. A third consecutive failed hack on one device raises its faction's alarm with the hacker as subject.
47. **Flash ring.** A flash fired on tick `t` is seen by every observer on exactly one check among ticks `t + 1` to `t + 3`; saving at boundary `t + 1` and loading gives the same detection results.
48. **Bluff.** GUI 55 and Speech 20 give rating 67 and chances 9500, 8400, 6400 for `bluffs` 0, 1, 2; each answer draws exactly 1 value on `rng.stealth` at seq 1006 (2 under `avatar_of_fate`); no draw on any other stream occurs.
49. **Event conversion.** A hit with 12,000 mHP of `dmg.sonic` emits `noise.impact.sonic` at 72 dB at its `pos`. A door toggle emits `noise.door.open` or `.close`; a torch snuff (cause `toggled`, not a door) emits only the action's 20 dB `noise.light.snuff`; a falling block emits `noise.block.fall` on `landed` and nothing on `fell`.
50. **Alarm.** `raise_alarm` with a subject puts every awake faction member within 48000 mm at searching with `search_ticks` 1800, creating pairs that did not exist; a second call from the same source within 600 ticks does nothing. A tattler's alarm has subject 0 and emits `noise.voice.squawk` at 90 dB. Every member an alarm reaches gets `wary_until = Tick::MAX`; after its 1800-tick search it returns to unaware through T8 with `wary_until` still `Tick::MAX`, and a later callout's Vigilance grant leaves it unchanged; a member of the alarmed faction spawned 3000 ticks later starts with `wary_until = Tick::MAX`; each call that passes the cooldown emits one `ev.alarm_raised`.
51. **Radio.** A radio squad's callout at tick 5000 reaches its squadmates at tick 5030, each emitting `noise.alarm.radio` at 50 dB; saving at boundary 5010 and loading gives the same delivery.
52. **Takedown witnesses.** An observer with a sight pass on the attacker during a takedown gains exactly 4000 meter once; the takedown hit carries `DmgFlags` bits 1 and 13.
53. **Hiding under.** An Erratum under a `hide: under` prop has the 600 mm capsule, `f_stance` 3500 and `f_frac` at most 4000; an Erratum seen entering an enclosed spot within 1500 mm of a searcher's `lkp` is found on the searcher's T6 or T7 entry.
54. **Content lint.** Every JSON block in this chapter parses with integers only, and every ID in Exports matches the Canon §1.1 grammar and lies in `noise.`, `aware.`, `disguise.` or `lock.`.
55. **Surfaces.** For each of doc 07's 68 non-air materials the §7.2 class equals its doc 07 §2.4 `acoustic` value (granite stone, hide soft, web cloth, coal dirt, gold metal). An unarmored walk with no §7.1 modifiers on soil gives 40 dB; the same step with 1 unit of `liq.mud` in the feet cell gives 42 (mud), with 1 unit of `liq.water` 48 (water), and in a `block.tall_grass` feet cell 34 (grass). No step reads `noise.surface.snow` from a doc 07 material, because none carries it; only §15.3's snowy-weather override yields it (test 63).
56. **Placed locks.** A `Locked(k1, k2)` door at Folio 12 is `lock.kind.door` at `lock.t3` with `key_tags = [k1, k2]`: an opener holding only `k1` cannot use the key path and must pick or hack it, while holding both opens it in 20 ticks with no draw. A `loot.chest.locked` chest at the same depth has `lock_hp` 56. A `hack` verb seal at Folio 12 with local TEC 3 is hacked at device DC 55 and never becomes an electronic lock.
57. **Cross-chapter IDs.** Every concrete `noise.`, `aware.`, `disguise.` or `lock.` ID that appears in any chapter appears in this chapter's Exports.
58. **No mounts, no wear.** No rule of this chapter reads a mount or a rider, and no exported ID starts with the `noise.mount.` prefix. After 100 lockpick, hack and disarm attempts, successes and failures alike, the actor's lockpicks, toolkit and hacking deck are unchanged.
59. **Events.** An auto pick that succeeds emits exactly one `ev.lock_opened` with method 1, a key opening one with method 0; a terminal Unlock in a room with two electronic locks emits one `ev.device_hacked` and two `ev.lock_opened` with method 2; a compromise emits one `ev.disguise_blown`; a discovered body one `ev.body_discovered`; a successful plant emits no `ev.item_stolen`.
60. **Disguise coverage.** Every `fac.native.*` faction of doc 13 §12.1 has at least one template with `outfit`; an outfit stripped from it makes the disguise active toward that faction's humanoid-profile members and inactive toward `aware.profile.editor` and `aware.profile.hound` observers.
61. **Light override.** Inside a doc 03 light override with `max_level` 0, an Erratum's `env` is 0, and the same Erratum holding a torch has `L_s = 12`.
62. **Alert flags and migration.** An Erratum alerted in Folio 4 sets `folio_alerted`; on Folio 5's `ev.folio_entered` it is false and stays false while no pair reaches alert, so Folio 5's Unseen bonus and `run.note.silent` both pass. A hot host migration mid-Folio keeps every pair's meter, state, `lkp` and `wary_until`, and a running lockpick attempt ends on its original tick.
63. **Weather.** The §15.3 rain example gives an outdoor ambient of 25, a gate of 28 and a rescaled 23 dB: inaudible. A scent mark on an outdoor cell lasts 1800 ticks and one under a roof 3600. With `fog` 1, a sight ray from an outdoor eye over 16000 mm gains 2 conceal from the weather and one from an indoor eye none. Under `snow` 1 an unarmored walk on outdoor soil is 32 dB and on roofed soil 40 dB. With no weather state stored every term is neutral and tests 12 to 14 hold unchanged.
64. **Fused Folio.** Cyberpunk fused with Gothic Horror reads `AMBIENT_DB` 25 and `SOUND_BP` 10500 outside seam and bleed zones; a shadow cloak gets `f_camo` 7500 and a pale cloak 12500 (pale clashes with the primary, neon). In Weird West a pale cloak gets 7500 (first matching row).
65. **Movement kit.** A sprint-slide on stone in a full medium set at Armor 0 emits `noise.move.slide` once at `45 + 6 + 6 = 57` dB, curious class; the sliding Erratum has `f_stance` 5500 and, at 6000 mm/s, `f_motion` 13000. Reeling on a line gives `f_stance` 12000 and `noise.move.reel` at `30 + armor_db` every 30 ticks; the line attaching emits an unattributed `noise.impact.grapple` at 50 dB at the anchor. A disguised Erratum gains 1000 suspicion per check while sliding and 1500 while reeling.
66. **Panels and cancels.** The §8.4 example panel is exact; a terminal's panel holds one Unlock line, a camera's Loop then Seize, an alarm panel's Disable then Test; a sleeping pickpocket target lists every stealable item at chance 10000; with no stealth target `panel` is None. A device hack cancelled by 600 mm of movement leaves `hack_fails` unchanged and draws nothing at seq 1001; a lockpick attempt cancelled after its first tick keeps its 2 draws and adds no pity.
67. **Lock mode.** A slot whose latest intent is `SetLockMode { manual: true }` starts manual attempts; a `SetLockMode { manual: false }` sent during a manual attempt leaves that attempt manual, and the next is auto.
68. **Indicator lifetime.** A step heard at tick `t` is in `steps` on ticks `t` to `t + 29` and absent from `t + 30`; with 5 live steps the snapshot carries the 4 newest (doc 16 §6.8). Rings live the same 30 ticks.
69. **Gore and traces.** With `access.gore` at `minimal` on every client, a 1000 ml `liq.blood` cell sighted within `R_near` still gives the suspicious disturbance. A death echo or marginal note in the Folio creates no pair, no disturbance and no `HeardNoise` record.
70. **Second Edition depth.** At depth 55, `lock_tier` is 6, `trap_dc` 100 (and 99 at depth 37), and both a humanoid's and a sentry's `PR` are 100.

## Invariants annex

```
Σ stealth: light ⊗ visibility ⊗ senses ⊗ meter ⊗ states ⊗ noise ⊗ sneak ⊗ hiding ⊗ disguise ⊗ security
W! detection := deterministic, 0 draws                         W! security|pickpocket|trap rolls := keyed rng.stealth, fixed seq
W! sneak eligibility := per (attacker, target) pair            N! other observers' states → eligibility
W! sneak hit ⇒ no crit roll                                     W! ignore_armor ⇐ Sneak 100 ∨ unseen_blade, ¬boss ranks
W! boss|editor|hunter ⇒ sneak_mult ≤ 15000 ∧ no takedown       N! NPC sneak multiplier vs Errata
W! M ∈ 0..=10000; gains chain in listed order                  N! hearing alone → M ≥ 7500 (except hear_alerts)
W! alert ⇔ M reached 10000 ∨ contact ∨ compromise              W! lost/alert decay := timers only
W! stealth_light = max(env, own, flash, phosphor, spot) ≤ 15    N! stealth_light written to the light grid
W! concealment sum ≥ 15 ⇒ ray blocked                          W! f_conc = 10000 − 667 × sum
W! st.invisible ⇒ sight factor 1000 unless see-invisible       N! invisibility vs scent | tremor | breath | wards | lasers
N! mount or rider rule; no noise.mount id (Canon §17.7)        N! ghost perceived
W! every noise has a noise.* id and a level from noise_level   W! unattributed noise → disturbance only
W! Unseen ⇔ ¬folio_alerted                                     W! electronic lock ⇒ local TEC ≥ 5
W! lock pity: progress ≥ lock_hp ⇒ open (¬vault, ¬quest)        N! HashMap in StealthWorld
W! pair.HUNTED ∧ ¬st.sleeping ⇒ sneak_mult ≤ 15000              W! HUNTED ⇐ enter lost; cleared by T8 ∨ T12
W! f_dist ∈ 0..=10000                                          W! gate(o, e) ≥ amb(o) + 3
W! trail: 120 marks × 30 ticks ≥ scent life 3600               N! scent ring shared across owners
W! st.hacked targets ⊆ {camera, turret, drone, construct}      N! st.hacked on boss|editor|hunter ∨ Errata gadget
W! flash seen by each observer on exactly 1 check              W! flash_ring ∈ save
W! your_tier(sec) = min(5, 2 + sec/14) + [sec ≥ 75]            W! security|hack|disarm|steal rating := 3/2 × skill + d(sub)/2 + tool
W! bluff draw := rng.stealth seq 1006                          N! rng.check ∈ doc 09 draws
W! raise_alarm source ∈ §6.6 table ∧ cooldown 600 per source    W! SOUND_BP defined ∀ genre (§15.1)
W! convert_events: door noise ⇔ door state toggled             W! block fall noise ⇔ cause landed
W! surface(material) = doc 07 acoustic(material), then the feet-cell overrides mud, water, grass
W! key open ⇔ opener holds a key for every non-zero key_tags entry   W! used noise.|aware.|disguise.|lock. id ⊆ Exports
N! tool worn, broken or consumed by a pick, hack or disarm       W! slip ⇒ + SLIP_TICKS ∧ noise.lock.snap
W! lock_mode governs picks and hacks (Seize included)           W! manual attempt ⇒ 0 success draws ∧ exactly 2 + tier draws
W! alarm ⇒ wary_until = Tick::MAX for every member it reaches   W! Vigilance grant := max(wary_until, now + 5400)
W! every fac.native.* faction has an outfit template            W! each §1.7 event emitted once per occurrence
W! StealthWorld resets fully on ev.folio_entered               W! weather term at c ⇒ sky_src(c)
W! rings, steps live 30 ticks, newest first                     W! panel numbers := the host's §13 values
N! doc 09 interest filter on invader replication               W! fused Folio AMBIENT_DB, SOUND_BP := floor_div(a + b, 2)
N! trace perceived or audible                                   W! cancelled attempt ⇒ no pity, no XP, no hack_fails
```
