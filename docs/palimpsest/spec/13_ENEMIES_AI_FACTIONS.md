# 13 — Enemies, AI, Factions and NPCs

As of 2026-10-11. Status: BINDING. Conforms to canon-1.2.

This chapter defines every hostile and non-player actor: enemy templates and scaling, roles, the AI, rosters, bosses, the Director, XP pacing, factions and NPCs. All AI runs on the host inside `palimpsest_sim` and its helper crate `palimpsest_ai` (doc 02 §3.2).

## Imports

| Source | Consumed |
|---|---|
| Brief §1, §2, §4, §5 | Errata and Editors, archetype × skin, axioms, run shape, Folio length, deterministic fixed-point sim |
| Canon §1, §2.2, §2.3 | ID grammar, integer and `Fx` primitives, `Bam`, EntityId order and phase key, AI think period 6, durations, `in_combat`, `light_at` |
| Canon §3 | `rng.spawn`, `rng.ai_idle` (sequential); `rng.ai`, `rng.combat`, `rng.loot`, `rng.magic` (keyed); draw-count rule |
| Canon §4, §5.2 to §5.8 | `ACTIVE_ACTOR_CAP`, movement, jump, fall damage; genres, dependencies, NPC tolerance, Contradiction, Edge, MYTHOS ambient, local genre |
| Canon §6.4, §6.5, §7 | NPC derived-stat rows; NPC stamina and aether as doc 13 constants; skill XP normalization and pace contract |
| Canon §8 | XP curve, `kill_xp_std`, XP sources, co-op split, the 40-unit target |
| Canon §9 to §12 | body plans, severing, damage types, pipeline, statuses, buildup, direct durations, Erasure §11.6, §11.8 player semantics, surfaces, spawn light |
| Canon §13.8, §15, §16.3, §17 | `P`, `T`, `H`, `S`, `enemy_level`; factions and minions; kin; run constants, party tables, ranks, champion aspects, anchors, friendly fire, ghosts, mounts |
| Canon §18, §19 | event IDs; namespaces `enemy.`, `ai.`, `fac.`, `boss.`, `npc.`, `director.`, `rank.`, `elite.` |
| Doc 02 §2.3, §3.4 | in-house A* in `palimpsest_ai` with lower-cell-index tie-break; `score_actions`, `astar` |
| R1 §2.12, §5.9; R2 §2.4, §5.3; R4 §9; R5 §2.6, §9.2; R6 §9, §10 | SPD investigate state, Qud reputation; Oblivion scaling anti-pattern, Minecraft spawn light; utility AI and HPA*; awareness machine, darkness in gunfights; genre rosters and factions |

## 1. Design rules

1. **Depth scaling only.** Enemy power scales with Folio depth `d` through `H`, `T`, `P` and `S` (Canon §13.8) and with party size (Canon §17.2). It never reads any Erratum's level, skills or gear. This avoids Oblivion's leveling wall (R2 §2.4).
2. **Roles are archetypes, roster entries are skins.** Each enemy template names one of the 16 roles in §3. The role carries the stat template and behavior set. The roster entry carries the skin, body plan, kin, tags, faction, signature ability and its axiom dependencies.
3. **Fair telegraphs.** Every enemy attack whose template damage multiplier is at least 20000 Bp has a windup of at least 30 ticks and a published telegraph. At 40000 Bp or more the windup is at least 45 ticks. Boss attacks list their exact windups in §9.
4. **No hidden cheating.** AI perceives only through doc 09 awareness, except Erasure hunters (§10.5), which always know the position of every living Erratum, and bosses inside their sealed arena, which always know every Erratum in the arena.
5. **Determinism.** All AI runs on the host. Every state-mutating loop runs in ascending `EntityId`. Bulk population draws use `rng.spawn`. Idle and wander draws use `rng.ai_idle`. Choices made during an agent's think use keyed `rng.ai`. Every draw count is stated.

## 2. Enemy entity model

### 2.1 Enemy template schema

An enemy template is one JSON record in `content/13/enemy/`. Every enemy, mini-boss, Editor, hunter, summon template of doc 13, vendor and hireling uses this schema.

| Field | Type | Meaning |
|---|---|---|
| `id` | content ID | `enemy.<genre_suffix>.<name>`, `enemy.editors.<name>`, `boss.…` for bosses, `npc.…` for NPCs |
| `role` | content ID | one of the 16 `enemy.role.*` (§3); bosses use `enemy.role.boss` |
| `origin_genre` | content ID | drives NPC tolerance and component requirements (Canon §5.3, §5.4) |
| `rank` | content ID | default rank; the Director may promote standard templates to elite or champion (§7) |
| `body` | content ID | Canon §9 body plan |
| `kin` | content ID or null | Canon §16.3 kin for humanoid sapients (§2.4) |
| `tags` | list of enum | §2.4 tag vocabulary |
| `faction` | content ID | `fac.*` (§12) |
| `hp_mhp`, `dmg_mhp` | i64 | template HP and template damage per standard hit at Folio 1, rank standard |
| `cycle_ticks` | u32 | ticks between the starts of two basic attacks |
| `walk_mm_s`, `run_mm_s` | i32 | base move speeds before Canon §6.4 row 11 |
| `nav` | content ID | `ai.nav.*` class (§4.13) |
| `subs` | map `sub.*` → i32 | template sub-attributes; omitted ones are 40 |
| `armor_poise_mp` | i64 | milli-poise added to Canon §6.4 row 18 |
| `ar` | i64 | natural AR per body part at `P(1)` |
| `resist` | map `dmg.*` → Bp | before kin and tag adjustments |
| `immune`, `immune_status` | lists | damage types and statuses with no effect |
| `stamina_m`, `aether_m` | i64 | NPC pool maxima (Canon §6.5); regeneration is 20,000 stamina and 2,000 aether per second for every NPC |
| `block_bp` | Bp | block stability for NPC blockers (Canon §6.5 excludes row 19); 0 for non-blockers |
| `senses` | list of enum | `sight`, `hearing`, `breath`, `tremor`, `thermal` (§2.4) |
| `attacks` | list of attack profiles | basic attack first, then abilities |
| `behavior` | content ID | `ai.set.*` (§4.7) |
| `loot` | content ID | `loot.drop.*` reference (doc 11) |
| `weight` | u32 | roster weight for the Director (§10.1) |
| `flags` | object | `sapient`, `can_surrender`, `uses_items`, `weapon_pickup`, `morale_immune`, `diggable_nav`, `summoned` |
| `carry` | list | spawn inventory (§5.1) |
| `mount` | content ID or null | `arch.mount.*` for cavalry (§5.6) |

Attack profile fields: `id` (`enemy.ab.*`), `kind` (`melee`, `projectile`, `hitscan`, `beam`, `area`, `lob`, `aura`, `summon`, `self`), `dmg_mult_bp`, `shares` (map `dmg.*` → Bp summing to 10000), `pen_bp`, `windup`, `active`, `recovery`, `cooldown` (all ticks), `range_mm`, `min_range_mm`, `radius_mm`, `cone_bam`, `speed_mm_s`, `impact_mp` (milli-poise), `status` (list of `{st, power}` or `{st, direct_ticks}`), `dep` (subset of the four axioms), `telegraph` (`none`, `circle`, `cone`, `line`, `ring`, `marker`), `token` (`melee`, `ranged`, `control`, `grenade`, `none`).

Example record:

```json
{
  "id": "enemy.space_opera.navy_trooper",
  "role": "enemy.role.soldier",
  "origin_genre": "genre.space_opera",
  "rank": "rank.standard",
  "body": "body.humanoid",
  "kin": "kin.human",
  "tags": ["human", "sapient"],
  "faction": "fac.native.space_opera.imperial_navy",
  "hp_mhp": 60000, "dmg_mhp": 10000, "cycle_ticks": 90,
  "walk_mm_s": 3600, "run_mm_s": 5800, "nav": "ai.nav.humanoid",
  "subs": { "sub.sight": 45 }, "armor_poise_mp": 0, "ar": 10,
  "resist": { "dmg.shock": 1000 }, "immune": [], "immune_status": [],
  "stamina_m": 80000, "aether_m": 0, "block_bp": 0,
  "senses": ["sight", "hearing"],
  "attacks": [
    { "id": "enemy.ab.blaster_volley", "kind": "projectile", "dmg_mult_bp": 4000,
      "shares": { "dmg.fire": 5000, "dmg.pierce": 5000 }, "pen_bp": 1000,
      "windup": 18, "active": 24, "recovery": 30, "cooldown": 90,
      "range_mm": 25000, "min_range_mm": 0, "radius_mm": 0, "cone_bam": 0,
      "speed_mm_s": 60000, "impact_mp": 4000, "status": [],
      "dep": ["techne"], "telegraph": "none", "token": "ranged" }
  ],
  "behavior": "ai.set.soldier",
  "loot": "loot.drop.armed",
  "weight": 3000,
  "flags": { "sapient": 1, "can_surrender": 1, "uses_items": 1, "weapon_pickup": 1,
             "morale_immune": 0, "diggable_nav": 0, "summoned": 0 },
  "carry": [ { "cat": "cat.potion", "chance_bp": 1500, "count": 1 } ],
  "mount": null
}
```

The blaster volley fires 3 bolts 12 ticks apart inside its 24-tick active window, each at `dmg_mult_bp` 4000, so one volley is 12,000 mHP of template damage. Booleans are integers 0 or 1 because content holds integers only.

### 2.2 Stat scaling

For an enemy spawned or re-evaluated in Folio `d` (with `d_pow = min(d, 300)`), party size `n` and rank `r`:

```
hp_max_mhp   = mul_bp(mul_bp(mul_bp(tpl.hp_mhp, H(d)), r.hp_bp), party_hp_bp(n, r))      // Canon §13.8, §17.2, §17.3
tpl_atk_mhp  = mul_bp(tpl.dmg_mhp, atk.dmg_mult_bp)                                     // content-level attack template
B            = mul_bp(mul_bp(tpl_atk_mhp, T(d)), r.dmg_bp)                              // Canon §10.4 enemy source
AR_part      = mul_bp(tpl.ar, P(d))                                                     // every part; then statuses (Canon §10.3)
sub_eff      = min(150, tpl.subs[x] + floor_div(3 * (d - 1), 2))                         // Canon §13.8
poise_m      = (20 + floor_div(BRN_eff, 2)) * 1000 + tpl.armor_poise_mp                 // Canon §6.4 row 18
status power = p × S(d)                                                                  // Canon §11.2
```

`party_hp_bp(n, r)` reads the first row of Canon §17.2 for ranks minion to champion and the second row for boss, editor and hunter. The party-size damage multiplier is applied only at pipeline step 2.14 against Errata (Canon §10.5). Stats are computed at spawn. An enemy that crosses a Folio boundary does not exist; enemies never leave their Folio.

**Worked example 1.** A standard `enemy.space_opera.navy_trooper` in Folio 10 with a party of 2: `hp_max = mul_bp(mul_bp(mul_bp(60000, 34423), 10000), 16000) = 330,460` mHP. A full-damage template hit (multiplier 10000) gives `B = mul_bp(mul_bp(10000, 16190), 10000) = 16,190` mHP, which step 2.14 raises to 17,809 before mitigation. `AR = mul_bp(10, 21718) = 21` per part. BRN is `40 + floor_div(27, 2) = 53`, so poise is 46,000 milli-poise.

**Worked example 2.** An elite brute (template 110,000 mHP, 18,000 mHP) in Folio 25 with a party of 4: `hp_max = mul_bp(mul_bp(mul_bp(110000, 202521), 30000), 25500) = 17,042,142` mHP. Its basic hit is `mul_bp(mul_bp(18000, 36145), 13000) = 84,579` mHP, and 109,952 after step 2.14. `AR = mul_bp(15, 79110) = 118`.

### 2.3 Counts per spawner

Every spawner except boss, editor and hunter spawners multiplies its base count by `enemy_count_bp[n]` with a per-spawner carry (Canon §17.2); the Director counts as one spawner per rank per Folio (§10.1). This includes elite and champion spawners, so per-player kill XP after the co-op split equals the solo value. Example: a spawner with base 5 in a party of 4 spawns 8, 9, 9 and 9 on its first four activations (`5 × 17500 = 87500`, carries 7500, 5000, 2500, 0).

### 2.4 Kin and creature tags

Humanoid sapient enemies carry a kin. Kin rules for enemies are the Canon §16.3 rules with the Errata-only clauses removed.

| Kin | Enemy rules | Tags it adds |
|---|---|---|
| `kin.human` | none | `human` |
| `kin.construct` | immune to `st.poisoned`, `st.diseased`, `st.bleeding`; `resist[dmg.shock] -= 2500`; no breath; eligible for `st.hacked` only when tagged `electronic` (`body.construct` entities are always eligible, Canon §11.1); immune to every other mind-group status | `construct`, plus `electronic` when its basic attack depends on techne |
| `kin.undead` | no natural HP regeneration; `dmg.umbral` taken heals 5000 Bp of its amount; `resist[dmg.radiant] -= 5000`; immune to `st.poisoned`, `st.diseased`; no breath | `undead` |
| `kin.beastkin` | basic melee carries a 5000 Bp `dmg.slash` share; dark-vision +2 m | `beastkin` |
| `kin.fae` | takes × 12500 damage from weapons whose primary material is `mat.cold_iron` or `mat.iron`; `origin_tol[arcana] += 1` | `fae` |
| `kin.mutant` | one mutation from the template's `mutation` field: `regrowth` (natural HP regeneration 100 Bp of Max HP per second after 5 s without damage), `extra_arm` (adds `bp.arm_extra_1`), `hide` (`ar × 15000`) or `gills` (no breath) | `mutant` |

Tag vocabulary (closed enum, validated by schema): `human`, `construct`, `undead`, `beastkin`, `fae`, `mutant`, `beast`, `fiend`, `divine`, `eldritch`, `swarm`, `flying`, `aquatic`, `burrowing`, `sapient`, `mindless`, `electronic`, `plant`, `elemental`, `editor`, `summoned`, `mounted`.

| Tag | Rule |
|---|---|
| `undead`, `fiend`, `divine`, `eldritch` | Canon §5.7 MYTHOS ambient at step 2.7; `undead` and `fiend` take × 15000 radiant at step 3a |
| `mindless` | immune to `st.feared`, `st.charmed`, `st.dread`, `st.taunted`; `morale_immune` |
| `electronic` | eligible for `st.hacked` (Canon §11.1) |
| `swarm` | body plan `body.swarm`; takes × 15000 area damage (Canon §9) |
| `editor` | member of `fac.editors`; erases Errata-placed blocks (§10.6); abilities have empty `dep` (§2.6) |
| `summoned` | grants 0 kill XP (§11.1); despawns when its summoner dies |
| `sapient` | can talk, parley, surrender and witness reputation events (§12.3) |

Senses: `sight` and `hearing` follow doc 09. `breath` detects any breathing entity within 6,000 mm through walls and replaces sight (the jiangshi rule). `tremor` detects footstep noise events at or above 25 dB within 12,000 mm through solid blocks. `thermal` ignores gas concealment (Canon §12.2) for sight rays.

### 2.5 Resistance profiles

`resist[t] = role default + template value + kin and tag adjustments`, summed, then clamped to the NPC cap of 9000 at pipeline step 4 (Canon §10.2). Genre-wide affinities from the doc 12 GPV `damage_affinity` add last. Role defaults appear in §3.

### 2.6 Contradiction for NPCs

Enemy innate attacks use the template's `origin_genre` and the `dep` of each attack (Canon §5.3). Tolerance is `origin_tol = 2` on every axis where the origin genre's axiom is at least 5 (Canon §5.4); kin fae adds 1 on ARCANA. A foreign enemy in a bleed zone or in a Folio of another genre carries its own Contradiction and its own Anachronism Edge against natives (Canon §5.5, §5.6).

Example: `enemy.space_opera.navy_trooper` has `dep = {techne}`, `Req = max(1, 10 - 2) = 8`. Inside a Fairy Tale bleed zone (TEC 1), `raw_gap = 7`, `Tol = 2`, `gap = 5`, so `C = 5`: potency 5000, 1500 Bp jam chance per volley rolled on keyed `rng.combat`, and `E = 6`, so +2400 Bp against Fairy Tale natives.

Every attack of a `fac.editors` member has `dep = {}`. Editors and Red Ink servants are therefore Coherent everywhere and never receive Anachronism Edge.

## 3. Roles

Sixteen roles cover R6's eight roster roles and the extra jobs a voxel shooter needs. Values are Folio-1, rank-standard templates. HP and damage are display units (stored × 1000). `cycle` is ticks between basic attacks. AR is per body part before `P(d)`. Poise uses Canon §6.4 row 18 with the listed BRN and armor poise.

| Role ID | Default rank | HP | Dmg | Cycle | Range mm | Walk / run mm/s | Nav | BRN + armor poise = poise | AR | Role resist (Bp) | SGT / HRG / ITU | Stamina / aether | Impact | Behavior set |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| `enemy.role.soldier` | standard | 60 | 10 | 90 | 25000 | 3600 / 5800 | humanoid | 40 + 0 = 40 | 10 | none | 45 / 40 / 40 | 80 / 0 | 10 | `ai.set.soldier` |
| `enemy.role.swarmer` | minion | 40 | 6 | 60 | 1800 | 4800 / 7000 | small | 20 + 0 = 30 | 0 | none | 35 / 50 / 30 | 60 / 0 | 4 | `ai.set.swarmer` |
| `enemy.role.brute` | standard | 110 | 18 | 120 | 2500 | 3400 / 5200 | humanoid | 60 + 30 = 80 | 15 | blunt 2000 | 35 / 40 / 35 | 120 / 0 | 30 | `ai.set.brute` |
| `enemy.role.skirmisher` | standard | 50 | 8 | 50 | 2000 | 4800 / 7200 | humanoid | 35 + 0 = 37 | 6 | none | 45 / 45 / 45 | 100 / 0 | 8 | `ai.set.skirmisher` |
| `enemy.role.shooter` | standard | 55 | 4 per round, 4-round burst | 120 | 30000 | 3600 / 5800 | humanoid | 40 + 10 = 50 | 10 | none | 45 / 40 / 40 | 80 / 0 | 3 | `ai.set.shooter` |
| `enemy.role.sniper` | standard | 45 | 22 | 240 | 60000 | 3400 / 5600 | humanoid | 35 + 0 = 37 | 5 | none | 60 / 35 / 45 | 70 / 0 | 15 | `ai.set.sniper` |
| `enemy.role.caster` | standard | 45 | 14 | 150 | 30000 | 3400 / 5600 | humanoid | 30 + 0 = 35 | 4 | arcane 3000 | 40 / 40 / 50 | 60 / 80 | 8 | `ai.set.caster` |
| `enemy.role.controller` | standard | 50 | 6, status power 60 | 180 | 20000 | 3400 / 5600 | humanoid | 35 + 0 = 37 | 6 | psychic 2000 | 45 / 45 / 50 | 70 / 80 | 5 | `ai.set.controller` |
| `enemy.role.support` | standard | 50 | 6 | 120 | 15000 | 3600 / 5800 | humanoid | 35 + 0 = 37 | 8 | radiant 2000 | 40 / 45 / 50 | 70 / 100 | 5 | `ai.set.support` |
| `enemy.role.guardian` | standard | 90 | 10 | 100 | 2500 | 3200 / 5000 | humanoid | 60 + 40 = 90 | 25 | slash 2000, pierce 2000 | 40 / 40 / 40 | 140 / 0 | 20 | `ai.set.guardian` |
| `enemy.role.summoner` | standard | 55 | 8 | 150 | 25000 | 3400 / 5600 | humanoid | 30 + 0 = 35 | 4 | umbral 2000 | 40 / 40 / 50 | 60 / 120 | 6 | `ai.set.summoner` |
| `enemy.role.stalker` | standard | 45 | 20 | 90 | 2000 | 4400 / 7000 | humanoid | 35 + 0 = 37 | 5 | none | 50 / 55 / 50 | 100 / 40 | 12 | `ai.set.stalker` |
| `enemy.role.artillery` | standard | 60 | 16, radius 2500 | 240 | 40000, min 6000 | 3200 / 5000 | humanoid | 45 + 10 = 52 | 12 | fire 2000 | 45 / 35 / 40 | 80 / 0 | 20 | `ai.set.artillery` |
| `enemy.role.sapper` | standard | 55 | 10; charge 25, radius 3000 | 90 | 2000 | 3600 / 5800 | burrower | 50 + 0 = 45 | 10 | blunt 2000 | 35 / 50 / 40 | 120 / 0 | 15 | `ai.set.sapper` |
| `enemy.role.flier` | standard | 40 | 9 | 90 | 2000 melee, 15000 ranged | fly 6000 / 9000 | flier | 30 + 0 = 35 | 4 | none | 55 / 40 / 40 | 100 / 0 | 6 | `ai.set.flier` |
| `enemy.role.cavalry` | standard | 60 rider, 80 mount | 12; charge × 12500 | 90 | 3000 | mount 6000 / 12000 | mounted | 45 + 20 = 62 | 12 | none | 45 / 40 / 40 | 100 / 0 | 25 | `ai.set.cavalry` |

`enemy.role.soldier` is the reference role of Canon §17.4: 60 HP, 10 HP per hit, AR 10, so `REF_PLAYER_DPS_D1` of 30 kills it in `REF_STANDARD_TTK_D1` = 2.0 s. At Folio 1 a soldier deals 10 HP per 90 ticks, 6.67 HP per second before mitigation. Against a 20-AR Erratum each 10,000 mHP hit loses `DR = floor(200,000,000 / 60,000) = 3333` Bp, so each hit lands 6,667 mHP and a lone soldier needs 13 hits to kill an 85 HP Erratum: the first lands on tick 18 and the 13th on tick 1,098 (18.3 s).

Roster entries override `nav` where the body demands it: `ai.nav.large` for `cave_troll`, `hive_warrior`, `mutant_behemoth`, `railroad_automaton`, `steam_juggernaut`, `riot_mech`, `bronze_bull` and `palimpsest_beast`; `ai.nav.swimmer` for `lurker_below`.

Role attack defaults, used when a roster entry does not override them:

| Role | Basic attack | Role ability (all roles have it unless the roster overrides) |
|---|---|---|
| soldier | projectile, `dmg.pierce` 10000, windup 18 | `enemy.ab.grenade_toss`: lob, × 15000, radius 2500 mm, windup 40, cooldown 900, token grenade, needs a `cat.throwable` in `carry` |
| swarmer | melee, `dmg.slash` 10000, windup 12 | `enemy.ab.leap`: closes 4000 mm over 20 ticks, cooldown 300 |
| brute | melee arc 8192 Bam, `dmg.blunt` 10000, windup 30 | `enemy.ab.charge`: run at 9000 mm/s up to 12000 mm, × 20000, knockback 3000 mm, windup 36, telegraph line, cooldown 600 |
| skirmisher | melee, `dmg.slash` 10000, windup 10 | `enemy.ab.sidestep`: dodge 3000 mm over 12 ticks, 12 invulnerable ticks, 20 stamina, cooldown 180 |
| shooter | hitscan burst, `dmg.pierce` 10000, 4 rounds 8 ticks apart, windup 20 | `enemy.ab.suppress`: 3 s of fire at a cover cell, × 2500 per round, applies 30 `st.exposed` power per round, token ranged |
| sniper | hitscan, `dmg.pierce` 10000, `pen_bp` 3000, windup 60 with a visible aim beam | `enemy.ab.relocate`: moves to the best cover cell at least 10000 mm away after each shot |
| caster | projectile, `dmg.arcane` 10000 (skin may change the type), windup 45 | `enemy.ab.blink`: teleport 6000 mm to a walkable cell out of the target's frontal cone, windup 30, cooldown 600 |
| controller | projectile, `dmg.psychic` 10000 | `enemy.ab.hold`: direct `st.rooted` 2 s, windup 40, cooldown 600, token control |
| support | projectile, `dmg.radiant` 10000 | `enemy.ab.mend`: heal one ally 1500 Bp of its Max HP, windup 45, cooldown 300; `enemy.ab.bolster`: direct `st.fortified` on one ally, cooldown 900 |
| guardian | melee, `dmg.slash` 10000, windup 24 | frontal block arc 10923 Bam with `block_bp` 7500 while not attacking; `enemy.ab.shield_bash`: × 12000, direct `st.staggered`, cooldown 480 |
| summoner | projectile, `dmg.umbral` 10000 | `enemy.ab.summon`: 2 swarmers of its genre roster at rank minion with tag `summoned`, windup 60, cooldown 900, at most 4 alive |
| stalker | melee, `dmg.slash` 10000, `st.bleeding` power 40 | `enemy.ab.vanish`: direct `st.invisible` 10 s, cooldown 600; `enemy.ab.ambush` adds +5000 Bp at step 1 to the first hit after `st.invisible` ends by attacking |
| artillery | lob, `dmg.blunt` 5000 + `dmg.fire` 5000, radius 2500, windup 45, telegraph circle | `enemy.ab.barrage`: 3 lobs at cells 2000 mm apart, windup 60, cooldown 1200 |
| sapper | melee, `dmg.blunt` 10000 | `enemy.ab.demo_charge`: places a charge that detonates after 120 ticks, × 25000, radius 3000 mm, breaks every breakable block within 2000 mm, telegraph ring, cooldown 900 |
| flier | melee dive, `dmg.slash` 10000, windup 24 | `enemy.ab.strafe`: projectile ranged attack at 15000 mm while circling |
| cavalry | melee lance, `dmg.pierce` 10000 | `enemy.ab.lance_charge`: mounted charge per Canon §17.7, windup 30, telegraph line, cooldown 480 |

## 4. AI architecture

### 4.1 Pipeline and schedule

The AI is a utility AI (Dave Mark's infinite-axis model, R4 §9) with an action-execution layer, a squad coordinator and a navigation service. Doc 03 owns the order of systems inside a tick. This chapter requires that, inside one tick, perception (doc 09) runs before squads, squads before the Director, the Director before thinks, thinks before the path service, and the path service before action execution.

| Stage | Period | Phase | Iteration order | Draws |
|---|---|---|---|---|
| Squad coordinator (§4.10) | 30 | `squad_id % 30` | ascending `squad_id` | none |
| Director (§10) | 30 | 11 | one instance per Folio | `rng.spawn` |
| AI think (§4.5) | 6 (Canon §2.2) | `(EntityId & 0xFF_FFFF_FFFF) % 6` | ascending EntityId | 1 keyed `rng.ai` per think that selects an action; 0 for dormant or committed thinks; idle and wander targets from `rng.ai_idle` |
| Path service (§4.13) | 1 | 0 | priority queue (§4.14) | none |
| Action execution (§4.6) | 1 | 0 | ascending EntityId | `rng.combat` for attacks (doc 05) |

```rust
fn think(a: &mut Agent, w: &World, t: Tick) {
    if a.bb.dormant {
        if !wake_check(a, w, t) { return; }          // §4.14
        a.bb.dormant = false;
    }
    update_blackboard(a, w, t);                       // §4.2, §4.8, §4.9
    if a.bb.action.is_committed() { return; }         // Windup, Active, Recovery, Channel (§4.6)
    let set = w.db.behavior_set(a.tpl.behavior);
    let mut scored: SmallVec<[(ActionId, i32); 24]> = SmallVec::new();
    for act in set.actions_sorted_by_id32() {         // ascending id32, so order is content-stable
        if !act.precondition(a, w) { continue; }
        let mut s = score_action(act, a, w);          // §4.5
        if Some(act.id) == a.bb.action.continuing() { s = mul_bp(s, MOMENTUM_BP); }
        if s > 0 { scored.push((act.id, s)); }
    }
    if scored.is_empty() { scored.push((AI_ACTION_IDLE, 1)); }
    let best = scored.iter().map(|x| x.1).max().unwrap();
    let thr  = core::cmp::max(1, mul_bp(best as i64, BAND_BP) as i32);
    let elig: SmallVec<[(ActionId, i32); 24]> = scored.into_iter().filter(|x| x.1 >= thr).collect();
    let weights: SmallVec<[u32; 24]> = elig.iter().map(|x| x.1 as u32).collect();
    let mut rng = keyed_rng("rng.ai", w.folio, a.id, t, 0);
    let pick = rng.weighted(&weights);                // exactly one range() call
    start_action(a, elig[pick].0, w, t);
}
```

Constants: `MOMENTUM_BP = 12500`, `BAND_BP = 9000`. The band turns near-ties into a weighted random choice, which is the "tie-break" use Canon §3.3 assigns to `rng.ai`. A think that selects a position (cover cell, flank cell, wander point) takes the extra draws from keyed `rng.ai` with `seq = 1` in combat states and from `rng.ai_idle` otherwise, one `range` call per positional choice.

### 4.2 Blackboard

Each agent owns one blackboard. It is simulation state and is serialized in saves (Canon §2.5).

```rust
pub struct Blackboard {
    pub dormant: bool,
    pub target: Option<EntityId>,
    pub target_since: Tick,
    pub last_known: BTreeMap<EntityId, LastKnown>,   // cell, tick, source: Sight | Sound | Callout | Omniscient
    pub threat: BTreeMap<EntityId, i64>,             // §4.8
    pub morale_bp: i32,                              // §4.9, 0..=10000
    pub squad: Option<u32>,
    pub order: SquadOrder,                           // §4.10
    pub token: Option<(EntityId, TokenKind)>,
    pub cover: Option<(CellIdx, CoverLevel)>,        // §4.11
    pub path: Option<PathHandle>,
    pub path_goal: Option<CellIdx>,
    pub repath_at: Tick,
    pub cooldown_until: SmallVec<[Tick; 8]>,         // per attack index
    pub action: ActionState,                         // §4.6
    pub last_damaged: Tick,
    pub last_attacked: Tick,
    pub alert_since: Tick,                           // reaction delay
    pub home: CellIdx,
    pub patrol: Option<(PatrolId, u16)>,
    pub owner: Option<EntityId>,                     // minions, hacked constructs (§5.7, §6)
    pub command: MinionCommand,                      // Follow | Hold(CellIdx) | Attack(EntityId)
    pub surrendered: bool,
    pub flee_until: Tick,
    pub held_item: Option<u64>,                      // weapon pickup (§5.3)
    pub mount_link: Option<EntityId>,                // rider <-> mount (§5.6)
}
```

`update_blackboard` runs at the start of each non-dormant think: it copies awareness and last-known positions from doc 09, decays threat (§4.8), applies morale changes queued since the last think (§4.9), drops a cover cell whose cover level fell to none, and drops a path whose goal moved more than 4000 mm.

### 4.3 Inputs

Every input returns `0..=10000`. Distances are `sqrt_fx` of `dist2` between origins, converted to mm by `(raw × 1000) >> 32`.

| Input ID | Value |
|---|---|
| `ai.input.self_hp` | `floor_div(hp × 10000, hp_max)` |
| `ai.input.target_hp` | the same for the current target |
| `ai.input.target_dist` | `clamp(floor_div(dist_mm × 10000, ref_mm), 0, 10000)` with `ref_mm` from the consideration |
| `ai.input.target_visible` | 10000 if doc 09 reports sight of the target on the latest perception tick, else 0 |
| `ai.input.target_aware` | the agent's awareness of the target: unaware 0, returning 1000, suspicious 2500, investigating 5000, searching 6000, alert 10000 |
| `ai.input.cooldown_ready` | 10000 if `t >= cooldown_until[attack]`, else 0 |
| `ai.input.has_token` | 10000 if the agent holds, or the squad would grant now, a token of the action's kind (§4.10), else 0 |
| `ai.input.in_cover` | current cover level against the target: none 0, half 6000, full 10000 |
| `ai.input.cover_gain` | `max(0, best_candidate_cover_score - current_cover_score)` (§4.11) |
| `ai.input.allies_near` | `min(10000, 2000 × allies within 12000 mm)` |
| `ai.input.morale` | `morale_bp` |
| `ai.input.threat_share` | `floor_div(threat[target] × 10000, Σ threat)`; 10000 when the table holds only the target |
| `ai.input.ammo` | carried ammo over magazine size for agents with an ammo `carry` entry, else 10000 |
| `ai.input.aether` | `floor_div(aether × 10000, aether_max)`; 10000 for agents with no aether pool |
| `ai.input.ally_low_hp` | `10000 - min(hp ratio of allies within 15000 mm)`; 0 with no ally |
| `ai.input.since_damaged` | `clamp(floor_div((t - last_damaged) × 10000, ref_ticks), 0, 10000)` |
| `ai.input.flank_angle` | `floor_div(ang_diff(target.yaw, bearing(target → agent)) × 10000, 32768)` |
| `ai.input.fire_lane` | 10000 if no allied capsule intersects the segment from the agent's muzzle point to the target's torso, else 0 |
| `ai.input.charge_lane` | 10000 if every cell of the straight walk line of the charge length is walkable at the agent's nav class |
| `ai.input.corpse_near` | 10000 if a `cat.corpse` lies within 8000 mm on a reachable cell, else 0 |
| `ai.input.item_value` | pickup score of the best reachable ground weapon within 8000 mm (§5.3) |
| `ai.input.owner_dist` | minions only: `clamp(floor_div(dist_mm × 10000, 16000), 0, 10000)` |
| `ai.input.light_here` | `floor_div(light_at(agent) × 10000, 15)` |
| `ai.input.squad_order` | 10000 if the squad order matches the action's order tag, else 0 |
| `ai.input.carrying` | 10000 if the agent carries an item usable by the action, else 0 |

### 4.4 Response curves

Every consideration maps one input through one curve. Curves are integer-only. `invert` replaces `x` with `10000 - x` before evaluation. Output is clamped to `0..=10000`.

```rust
pub enum Curve {
    Linear { m: i32, b: i32 },                 // y = b + floor_div(m × x, 10000)
    Poly   { k: u8, m: i32, b: i32 },          // p = x; repeat k-1 times: p = mul_bp(p, x); y = b + mul_bp(p, m); k in 1..=4
    Step   { t: i32, lo: i32, hi: i32 },       // y = if x >= t { hi } else { lo }
    Table  { knots: [i32; 17] },               // knots at x = 0, 625, ..., 10000
}
fn eval_table(k: &[i32; 17], x: i32) -> i32 {
    let i = (x / 625) as usize; let r = x % 625;
    if i >= 16 { return k[16]; }
    k[i] + floor_div(((k[i + 1] - k[i]) * r) as i64, 625) as i32
}
```

Table knots are generated offline by `tools/gen_curves.py` and committed. Shipped curves:

| Curve ID | Definition |
|---|---|
| `ai.curve.linear_up` | Linear m 10000, b 0 |
| `ai.curve.linear_down` | Linear m −10000, b 10000 |
| `ai.curve.soft_up` | Linear m 7500, b 2500 (never zero) |
| `ai.curve.quad_up` | Poly k 2, m 10000, b 0 |
| `ai.curve.quad_down` | Poly k 2, m 10000, b 0, invert |
| `ai.curve.cubic_up` | Poly k 3, m 10000, b 0 |
| `ai.curve.binary` | Step t 1, lo 0, hi 10000 |
| `ai.curve.binary_not` | Step t 1, lo 0, hi 10000, invert (10000 only when x = 0) |
| `ai.curve.gate_half` | Step t 5000, lo 0, hi 10000 |
| `ai.curve.gate_reach` | Step t 5000, lo 0, hi 10000, invert (10000 when x ≤ 5000) |
| `ai.curve.logistic_up` | Table `[67, 124, 230, 421, 759, 1330, 2227, 3486, 5000, 6514, 7773, 8670, 9241, 9579, 9770, 9876, 9933]`, which is `10000 / (1 + e^(-10(x/10000 - 0.5)))` rounded half-even |
| `ai.curve.logistic_down` | the same table, invert |
| `ai.curve.bell_mid` | Table `[183, 468, 1054, 2096, 3679, 5698, 7788, 9394, 10000, 9394, 7788, 5698, 3679, 2096, 1054, 468, 183]`, which is `10000 × e^(-((x/10000 - 0.5) / 0.25)^2)` |
| `ai.curve.plateau` | Table `[3000, 5000, 7000, 9000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 8000, 4000, 0]` (ranged attacks: best between 25% and 81% of max range) |

Worked values: `logistic_up(5000) = 5000`; `logistic_up(3500)`: `i = 5`, `r = 375`, `1330 + floor_div(897 × 375, 625) = 1868`; `bell_mid(5833)`: `i = 9`, `r = 208`, `9394 + floor_div(-1606 × 208, 625) = 8859`.

### 4.5 Scoring and selection

An action's score is the product of its considerations with Dave Mark's compensation factor, then its weight.

```
n       = number of considerations (1..=8)
mod_bp  = 10000 - floor_div(10000, n)
adj(c)  = c + mul_bp(mul_bp(10000 - c, mod_bp), c)
score   = 10000; for each c in listed order: if c == 0 { return 0 }; score = mul_bp(score, adj(c))
score   = mul_bp(score, weight_bp)
```

Compensation keeps actions with many considerations competitive: with `n = 3`, `adj(5000) = 6666`; with `n = 1`, `adj(c) = c`.

**Worked example.** A brute at 7,000 mm from its target, target visible, melee token available, HP 70%, charge off cooldown, clear charge lane. Its set offers four actions.

| Action | Weight | Considerations (input → curve) | Values | Score |
|---|---|---|---|---|
| `ai.action.melee` | 10000 | `target_dist`[ref 5000] → `gate_reach`; `has_token` → `binary`; `cooldown_ready` → `binary` | `x = 10000` → 0 | 0 |
| `ai.action.advance` | 9000 | `target_dist`[ref 20000] → `logistic_up`; `has_token` → `binary`; `target_visible` → `binary` | 1868, 10000, 10000; `adj(1868) = 2880` | 2,592 |
| `ai.action.charge` | 12000 | `target_dist`[ref 12000] → `bell_mid`; `cooldown_ready` → `binary`; `charge_lane` → `binary`; `self_hp` → `soft_up` | 8859, 10000, 10000, 7750; adj 9616 and 9057 | 10,450 |
| `ai.action.retreat` | 8000 | `self_hp` → `quad_down`; `morale` (9000) → `linear_down` | 900, 1000; adj 1309 and 1450 | 151 |

`best = 10450`, threshold `mul_bp(10450, 9000) = 9405`, so only the charge is eligible. The think draws one `weighted([10450])` call from keyed `rng.ai` (seq 0), which returns index 0, and the brute starts its charge windup.

### 4.6 Action execution layer

Each agent holds one `ActionState { id, phase, phase_until, target, aim_cell, attack_index, started }`.

| From | Condition | To | Side effects |
|---|---|---|---|
| `Idle` | think selects an attack or ability | `Windup` until `t + windup` | acquire token; `cooldown_until = t + cooldown`; publish telegraph |
| `Idle` | think selects a movement action | `Moving` | request path (§4.13) |
| `Moving` | goal reached or path fails | `Idle` | release path |
| `Moving` | next think selects another action | that action's first phase | |
| `Windup` | `t == phase_until` | `Active` until `t + active` | ranged aim locks 6 ticks before this transition |
| `Windup` | target died or despawned | `Idle` | `cooldown_until = t`; release token; retract telegraph |
| `Windup`, `Active`, `Channel` | an interrupting status starts | `Interrupted` | remaining cooldown halved; telegraph retracted; hits already resolved stay |
| `Active` | `t == phase_until` | `Recovery` until `t + recovery` | |
| `Recovery` | `t == phase_until` | `Idle` | token released at `t + 30` |
| `Interrupted` | no interrupting status remains | `Idle` | |
| any | morale broken (§4.9) | `Fleeing` or `Surrendered` | release token |
| any | death | `Dead` | |

Interrupting statuses: `st.staggered`, `st.stunned`, `st.frozen`, `st.knocked_down`, `st.sleeping`, `st.polymorphed`, `st.charmed`, `st.feared`, `st.berserk`, `st.hacked`, and `st.disarmed` for weapon attacks only. `st.silenced` interrupts `Windup` and `Channel` of abilities whose `dep` contains the genre's aether axiom or arcana.

**Turn rates.** Out of an attack an agent turns at most 1366 Bam per tick. During `Windup` of a melee attack it turns at most 546 Bam per tick (brutes and every rank boss or above 273 Bam), so sidestepping works. Ranged aim tracks the target until 6 ticks before `Active` and is then fixed.

**Reaction delay.** An agent that reached `aware.alert` on tick `a` cannot start an attack windup before `a + 18` (minion, standard), `a + 12` (elite, champion) or `a` (boss, editor, hunter).

**Telegraphs.** A telegraph is view data published in the doc 03 view frame, never an event: `{ shape: circle | cone | line | ring | marker, origin: Fx3, yaw: Bam, length_mm, radius_mm, half_angle_bam, start: Tick, impact: Tick, attacker: EntityId }`. The client draws it from `start` to `impact`.

**NPC ranged spread.** The half-angle passed to doc 05's two spread draws is the template spread (soldier 273 Bam, shooter 364, sniper 46, caster 182, artillery 0 with a 1000 mm landing scatter) × Canon §6.4 row 15 (PRC), × the darkness factor of the target cell's effective light (≤ 5: 18000, 6 to 9: 14000, 10 to 12: 11500, otherwise 10000; R5 §9.2 item 13), × 12000 when the target moves faster than 4300 mm/s.

### 4.7 Action catalog and behavior sets

Every action lists its weight and considerations in the order they multiply. `[ref]` is the input parameter.

| Action ID | Weight | Considerations | Notes |
|---|---|---|---|
| `ai.action.idle` | 1000 | none (always 10000) | stand, play idle |
| `ai.action.wander` | 2000 | `target_aware` → `binary_not` | point within 8000 mm of `home` from `rng.ai_idle` |
| `ai.action.patrol` | 3000 | `target_aware` → `binary_not` | doc 12 patrol route |
| `ai.action.investigate` | 7000 | `target_aware` → `gate_half` (≥ 5000) and `< 10000` precondition | walk to last-known cell; never attacks (SPD v4, R1 §2.12) |
| `ai.action.search` | 6000 | `target_aware` = 6000 precondition | visit 3 cells within 10000 mm of last-known, chosen by `rng.ai` seq 1 |
| `ai.action.return` | 4000 | `target_aware` = 1000 precondition | path home |
| `ai.action.melee` | 10000 | `target_dist`[2 × reach] → `gate_reach`; `has_token` → `binary`; `cooldown_ready` → `binary`; `target_visible` → `binary` | basic attack |
| `ai.action.shoot` | 10000 | `target_dist`[range] → `plateau`; `fire_lane` → `binary`; `target_visible` → `binary`; `has_token` → `binary`; `cooldown_ready` → `binary` | basic ranged attack |
| `ai.action.ability` | per ability, default 11000 | `cooldown_ready` → `binary`; `target_dist`[range] → `plateau`; `has_token` → `binary`; ability-specific extra | one entry per non-basic attack |
| `ai.action.advance` | 9000 | `target_dist`[20000] → `logistic_up`; `has_token` → `binary`; `target_visible` → `binary` | path toward attack range |
| `ai.action.circle` | 6000 | `has_token` → `binary_not`; `target_visible` → `binary` | hold a ring 6000 to 10000 mm from the target |
| `ai.action.charge` | 12000 | `target_dist`[12000] → `bell_mid`; `cooldown_ready` → `binary`; `charge_lane` → `binary`; `self_hp` → `soft_up` | brutes, cavalry |
| `ai.action.take_cover` | 8000 | `in_cover` → `linear_down`; `cover_gain` → `quad_up`; `since_damaged`[180] → `logistic_down` | §4.11 |
| `ai.action.flank` | 8500 | `squad_order` → `binary`; `flank_angle` → `linear_down` | §4.12 |
| `ai.action.suppress` | 9000 | `squad_order` → `binary`; `in_cover` (target's) → `gate_half` | shooters |
| `ai.action.retreat` | 8000 | `self_hp` → `quad_down`; `morale` → `linear_down` | back away to 15000 mm |
| `ai.action.reload` | 9000 | `ammo` → `quad_down` | 90 ticks |
| `ai.action.use_item` | 11000 | `self_hp` → `logistic_down`; `carrying` → `binary` | §5.1 |
| `ai.action.throw` | 9500 | `carrying` → `binary`; `target_dist`[20000] → `plateau`; `has_token` (grenade) → `binary` | §5.1 |
| `ai.action.heal_ally` | 10500 | `ally_low_hp` → `logistic_up`; `cooldown_ready` → `binary`; `aether` → `soft_up` | support |
| `ai.action.buff_ally` | 8000 | `allies_near` → `linear_up`; `cooldown_ready` → `binary` | support |
| `ai.action.summon` | 9500 | `cooldown_ready` → `binary`; `allies_near` → `linear_down` | summoner |
| `ai.action.call_allies` | 9000 | `target_aware` → `gate_half`; `allies_near` → `soft_up` | 30-tick shout; emits the doc 09 callout |
| `ai.action.pick_up` | 8500 | `item_value` → `linear_up`; `since_damaged`[120] → `linear_up` | §5.3 |
| `ai.action.dig` | 7000 | precondition: path blocked by a breakable block | §4.13 |
| `ai.action.vanish` | 9000 | `target_visible` → `binary`; `cooldown_ready` → `binary`; `light_here` → `linear_down` | stalker |
| `ai.action.eat_corpse` | 8000 | `corpse_near` → `binary`; `self_hp` → `quad_down` | ghouls, beasts: heals 2500 Bp of Max HP over 120 ticks |
| `ai.action.follow_owner` | 7000 | `owner_dist` → `logistic_up` | minions (§6) |
| `ai.action.guard_owner` | 9000 | owner downed or under attack precondition | minions |
| `ai.action.remount` | 9000 | dismounted cavalry whose mount lives within 8000 mm | §5.6 |

Behavior sets (`ai.set.*`) list the actions an agent may score. Every set also contains `idle`, `wander`, `patrol`, `investigate`, `search`, `return`, `call_allies` (sapient only) and `ability` entries for each non-basic attack.

| Set | Extra actions |
|---|---|
| `ai.set.soldier` | shoot, advance, circle, take_cover, flank, retreat, reload, use_item, throw, pick_up |
| `ai.set.swarmer` | melee, advance, eat_corpse |
| `ai.set.brute` | melee, advance, charge, pick_up |
| `ai.set.skirmisher` | melee, advance, circle, flank, retreat |
| `ai.set.shooter` | shoot, suppress, take_cover, advance, circle, reload, throw |
| `ai.set.sniper` | shoot, take_cover, retreat, reload |
| `ai.set.caster` | shoot, take_cover, retreat, use_item |
| `ai.set.controller` | shoot, take_cover, circle, retreat |
| `ai.set.support` | shoot, heal_ally, buff_ally, take_cover, retreat, use_item |
| `ai.set.guardian` | melee, advance, circle, pick_up |
| `ai.set.summoner` | shoot, summon, take_cover, retreat |
| `ai.set.stalker` | melee, vanish, flank, retreat |
| `ai.set.artillery` | shoot, take_cover, retreat, throw |
| `ai.set.sapper` | melee, dig, advance, throw |
| `ai.set.flier` | melee, shoot, advance, circle, retreat |
| `ai.set.cavalry` | melee, charge, advance, circle, remount |
| `ai.set.minion` | follow_owner, guard_owner, melee, shoot, advance, take_cover, retreat (§6) |
| `ai.set.vendor` | call_allies, retreat; becomes `ai.set.guardian` when hostile (§13.2) |
| `ai.set.hunter` | melee, ability, advance, charge (§10.5) |
| `ai.set.boss` | scripted by the boss phase table (§9.1) |

### 4.8 Target selection and threat

Threat entries exist only for entities the agent holds at `aware.alert` or has taken damage from.

```
on damage taken from e:   threat[e] += floor_div(dmg_mhp × threat_mult(e), 10000)        // §6.4 row 39 for e
on e healing a hostile-to-agent entity within 24000 mm while visible: threat[e] += floor_div(heal_mhp, 2)
on alert toward e:        threat[e] = max(threat[e], 1000)
each think:               threat[e] -= floor_div(threat[e], 100)                          // 1% per think
```

Candidate score for each alerted, perceivable candidate `c` (ghosts are never perceivable, Canon §17.6):

```
s = mul_bp(threat_share(c), 5000) + mul_bp(10000 - dist_norm(c, 24000), 3000) + mul_bp(10000 - hp_ratio(c), 1000)
s += 1000 if c == current target
s = mul_bp(s, 7000) if c is a minion;  s = mul_bp(s, 2000) if c has st.downed
```

The agent switches target only when the best candidate scores above `mul_bp(current_score, 12000)`. Ties go to the lower EntityId. `st.taunted` forces the taunter as target for its duration. An agent with `st.berserk` targets the nearest entity by `dist2`, ties to lower EntityId.

### 4.9 Morale, surrender and flight

`morale_bp` starts at 10000. Changes are queued by events and applied at the next think:

| Event (within 16000 mm and seen) | Δ |
|---|---|
| allied death | −1500; −3000 if it was the squad leader; −2500 if it was elite rank or higher |
| own HP first falls below 3000 Bp | −2000 (once) |
| squad reduced to half its spawn size | −1000 (once) |
| each think in combat while living allies ≥ 2 × alerted Errata | +100 |
| `enemy.ab.bolster` or `elite.rallying` pulse | +1500 |

Agents tagged `construct`, `mindless`, `undead` or `editor`, ranks boss, editor and hunter, and `summoned` agents are morale-immune. Below 2500:

- a `sapient` agent with `can_surrender` and rank minion or standard whose HP is below 2500 Bp **surrenders**: it drops its held weapon, kneels and stops acting. Any damage ends the surrender and makes it flee for the rest of the Folio. An Erratum's 30-tick interact spares it (§12.3): it becomes `fac.neutral`, walks to the nearest exit and despawns, and the party receives its kill XP as a defeat (§11.1);
- every other morale-capable agent **flees** for 600 ticks to the reachable cell that maximizes the minimum path distance to all threats, among 16 candidate cells sampled with keyed `rng.ai` seq 1 inside 32000 mm, then sets morale to 5000 and enters `search`.

### 4.10 Squad coordinator and attack tokens

A squad is created for every spawned pack and holds at most 8 members. Alerted agents of the same faction within 12000 mm merge into the squad with the lower `squad_id` while the total stays at most 8. The leader is the member of highest rank, ties to the lower EntityId.

Each coordinator run (period 30) does, in order:

1. **Shared knowledge.** Every member's `last_known` takes the newest entry per target from all members.
2. **Target spread.** For `m` members and `k` perceived targets, each target receives at most `ceil_div(m, k) + 1` members. Members keep their current target when it fits the cap; the rest are reassigned in ascending EntityId to the target with the fewest assigned members, ties to the lower target EntityId.
3. **Tokens.** Every potential target holds a token pool: melee 3, ranged 4, control 1, grenade 1. Requests are granted in descending action score, ties to the lower EntityId. A token is held from `Windup` to 30 ticks after `Recovery`. Elite, champion, boss, editor and hunter ranks never need tokens. Doc 14 difficulty may add to the melee and ranged pools.
4. **Orders.** `engage` by default. `flank` to at most 2 members per target (skirmisher and stalker roles first, then lowest EntityId) when at least 3 members are engaged. `suppress` to shooters when their target holds half or full cover. `fall_back` to all members when mean squad morale is below 4000: regroup within 6000 mm of the leader. `search` when no member has a target at `aware.alert` for 300 ticks.

### 4.11 Cover from the voxel grid

**Cover map.** Each chunk keeps a cover map rebuilt within 2 ticks of any `ev.block_changed` inside it: for every walkable cell (§4.13) and each of the 8 horizontal directions, the height of solid opaque blocks in the adjacent cell (0, 1 or 2 blocks above the floor).

**Cover level** of cell `c` against threat `h`: cast two voxel DDA rays from `h`'s eye to points 900 mm and 1500 mm above `c`'s floor. A ray is blocked if it crosses a solid opaque block or accumulates concealment ≥ 15 (Canon §12.2). Both blocked: full (10000). Only the 900 mm ray blocked: half (6000). Otherwise none (0).

**Query.** `take_cover` collects up to 48 candidate cells within path distance 16000 mm that have a non-zero cover-map entry, in ascending cell index, and scores each:

```
cover_score(c) = Σ over the up-to-3 highest-threat known enemies h: level(c, h) × w_h / Σ w_h   (w = threat, ties lower EntityId)
total(c)       = mul_bp(cover_score(c), 10000 - floor_div(clamp(floor_div(path_mm(c) × 10000, 16000), 0, 10000), 2))
                 then × 5000 if another squad member holds c or a cell adjacent to c
```

The highest `total` wins, ties to the lower cell index. Each query casts at most `48 × 3 × 2 = 288` rays.

### 4.12 Flanking

A flanker's goal is a walkable cell 6000 to 10000 mm from its target with `ang_diff(target.yaw, bearing(target → cell)) > 10923` Bam, which is the Canon §10.5 step 2.4b flank cone. Candidates are 16 cells on that ring at bearings `target.yaw + 16384 + i × 2048` for `i in 0..16`, snapped to the nearest walkable cell; the reachable one with the lowest path cost wins, ties to lower cell index. Its path uses a danger overlay: cells inside the target's frontal cone (half-angle 10923 Bam) and within 12000 mm cost × 3, so flankers circle outside the target's view.

### 4.13 Navigation

**Cells.** A cell is the 1 m air block a creature's feet occupy. Nav classes:

| Nav ID | Walkable cell rule | Moves |
|---|---|---|
| `ai.nav.small` | solid floor below, 1 passable cell of headroom | walk, jump-up, drop, climb |
| `ai.nav.humanoid` | floor below, 2 passable cells | walk, jump-up, drop, climb, door |
| `ai.nav.large` | floor below a 2 × 2 footprint, 3 passable cells | walk, jump-up, drop, door (breaks wooden doors) |
| `ai.nav.mounted` | floor below, 3 passable cells | walk, jump-up 1 block, drop ≤ 3 |
| `ai.nav.burrower` | humanoid rule | humanoid moves plus dig |
| `ai.nav.flier` | any passable air cell with 1 passable cell above | 26-connected flight |
| `ai.nav.swimmer` | liquid cell (water, blood, oil) | 26-connected inside liquid |

A cell is passable when its block is air, a non-solid plant, an open door, a liquid other than lava, or `block.web` for web-immune creatures.

**Links and costs** (cost unit: mm of level walking):

| Link | Rule | Cost |
|---|---|---|
| walk | 8 horizontal neighbors at the same level; a diagonal needs both orthogonal neighbors walkable | 1000; diagonal 1414 |
| jump-up | +1 block, with headroom above the origin | 2400 |
| gap jump | 2-cell horizontal gap at the same level; skirmisher, stalker, swarmer roles, or any role where `vrv ≥ 6` | 3000 |
| drop | down 1 to 3 blocks | 1200 per block |
| deep drop | down 4 or more blocks; only when the drop height is below `SAFE_FALL` (Canon §4.2) at the agent's position | 1200 per block |
| climb | ladder, vine or rope blocks (doc 07) | 2000 per block |
| door | closed unlocked door | 1000 + 1500 |
| locked door | key holder: + 1500; `ai.nav.large` or brute role: + door break time × walk speed; otherwise impassable | |
| dig | burrower or sapper, breakable block (§10.6 for hunters) | 1000 + `dig_ticks × walk_mm_s / 60` |
| hazard | cell with fire, lava contact, acid or electrified liquid | × 3; lava impassable unless `immune` contains `dmg.fire` |
| light | agents with tag `undead` or `fiend` at `light_at ≥ 12` | × 4 |

`dig_ticks = ceil_div(break_ticks(material) × 10000, dig_speed_bp)`, where doc 07 provides `break_ticks` and `dig_speed_bp` is 20000 for the sapper role, 10000 for other burrowers. Diggers never dig `block.void_bedrock`, doors or blocks inside a boss arena.

**Search.** Paths are found by A* over cells inside a chunk and by HPA* across chunks. A chunk's portals are maximal runs of walkable cells on each chunk face, per level; abstract edges between portals of one chunk store the intra-chunk A* cost. A block change marks its chunk dirty; at most 2 dirty chunks are rebuilt per tick, in FIFO order. The heuristic is octile distance in cost units (horizontal octile plus 1200 per block of vertical difference), which is admissible. Ties in the open list break by lower `f`, then lower `h`, then lower cell index (doc 02 §2.3). The cell index is `x + 256 × (z + 256 × y)`.

**Flow fields.** Swarmers and every agent of `ai.nav.small` targeting the same Erratum share one flow field: a Dijkstra integration field from the target's cell over a 64 × 64 horizontal window centered on it, all levels within ±8 blocks. It is rebuilt when the target changes cell or every 30 ticks, whichever comes later. At most 4 flow fields exist; a fifth request uses the field of the nearest existing target or falls back to A*.

**Following.** An agent steers to the next waypoint at its move speed. It requests a new path when its goal moved more than 4000 mm, when it has made less than 500 mm of progress in 30 ticks, or when the path crosses a changed chunk.

### 4.14 Think budget and level of detail

| Budget | Value | Overflow rule |
|---|---|---|
| Actions scored per think | 24 | content validation rejects larger sets |
| Considerations per action | 8 | validation error |
| A* node expansions per request | 2048 local, 512 abstract | returns `NoPath`; the agent picks `circle` or `return` |
| A* node expansions per tick, all requests | 8192 | remaining requests wait for the next tick |
| Cover queries per tick | 16 | later queries wait |
| Flow-field rebuilds per tick | 1 | later rebuilds wait |
| Cover-map chunk rebuilds per tick | 2 | FIFO |

The path queue orders requests by (priority, request tick, EntityId). Priority 0: agents at `aware.alert` and minions in combat. Priority 1: investigate and search. Priority 2: everything else.

**Dormancy.** An agent with no Erratum or minion within 48000 mm and awareness `unaware` toward everything is dormant. A dormant think only runs `wake_check`: it wakes when an Erratum or minion comes within 48000 mm, when doc 09 delivers an audible noise to it, when its squad orders `engage` or when it takes damage. Dormant thinks draw nothing.

At 256 active actors and period 6, about 43 thinks run per tick. The host AI budget is 1.5 ms per tick for thinks, squads, the Director and the path service together; doc 02 owns the performance table that enforces it.

## 5. Items, spells, stealth, weapons, summons, mounts and hacking

### 5.1 Carried items

A template's `carry` list is rolled once at spawn with `rng.spawn`, in listed order. Each entry draws exactly two values whatever the outcome: one `roll_bp(chance_bp)` and one `weighted` over the doc 11 list of common items of that category in the local genre (an entry naming one item ID uses `weighted` over a one-element list). Carried items are real items: `rar.common`, `q.normal`, `buc.uncursed`, `ilvl = d`, uids from the Folio item counter. Unused ones drop on the corpse's cell at death as shared world items (first pickup wins), in addition to the instanced loot roll (§9.1, doc 11).

- **Potions.** `ai.action.use_item` drinks a healing potion over 45 ticks (interruptible, §4.6). The heal follows the doc 08 potion definition in Bp of the drinker's Max HP. An Erratum that sees the drink learns the potion type to `know.classified` (monster-use identification, doc 10).
- **Throwables.** `ai.action.throw` lobs a `cat.throwable` with a 40-tick windup, a `circle` telegraph at the predicted landing cell and a grenade token. Damage and fuse come from doc 05 with `src_lvl = d`.
- **Ammo.** A shooter or sniper with an ammo entry has a magazine of 30 or 5 rounds; `reload` takes 90 ticks. Agents without ammo entries never reload.

### 5.2 Spells and abilities

Caster, controller, support and summoner templates may list `spells: [spell.*]` from doc 06 in addition to innate attacks. An NPC spell is an enemy source: `B = template_dmg_mhp × T(d) × rank damage` with the spell's NPC damage multiplier from doc 06. Canon §6.4 row 29 never applies to NPCs. Casting spends template aether, rolls Contradiction failure for the spell's base component with keyed `rng.combat` (a failure fizzles and spends full aether, doc 06), and rolls cast interruption (row 30) with keyed `rng.magic` when hit during `Windup` or `Channel`.

### 5.3 Picking up weapons

Humanoid sapient templates with `weapon_pickup` and an intact `bp.arm_r` may pick up `cat.weapon_melee` or `cat.weapon_ranged` items lying within 8000 mm.

```
pickup_bp   = clamp(mul_bp(rarity_power_bp, quality_bp), 8000, 15000)     // Canon §13.3, §13.5
item_value  = clamp(floor_div((pickup_bp - held_pickup_bp) × 10000, 7000), 0, 10000)   // held template weapon = 10000
interval_bp = clamp(floor_div(item_interval_ticks × 10000, tpl.cycle_ticks), 1000, 30000)
tpl_atk_mhp = mul_bp(mul_bp(tpl.dmg_mhp, pickup_bp), interval_bp)        // then × T(d) × rank damage (§2.2)
```

The picked weapon replaces the basic attack: shares, `pen_bp`, range, interval, magazine and status powers come from the item's doc 05 archetype and skin. Its components keep their own `dep` and `origin_genre`; the NPC's tolerance applies (§2.6). Pickup takes 30 ticks. The item keeps its uid and always drops when the holder dies or is disarmed. A disarmed agent adds +5000 to `item_value` for its own dropped weapon. Agents never pick up `cat.quest`, Palimpsestic or Exotic items.

Example: a Folio-10 soldier (cycle 90) picks up an Epic Good rifle with a 30-tick interval: `pickup_bp = mul_bp(11500, 11000) = 12650`, `interval_bp = 3333`, `tpl_atk = mul_bp(mul_bp(10000, 12650), 3333) = 4216`, so each shot is `mul_bp(4216, 16190) = 6825` mHP and three shots per 90 ticks give 20,475 mHP against its own weapon's 16,190.

### 5.4 Stealth enemies

The stalker loop is: `vanish` (direct `st.invisible`, 10 s, ends on attack, Canon §11.1) → path through cells with `light_at ≤ 7` (cells above cost × 4 for this role) → approach the target from outside its frontal cone → attack with `enemy.ab.ambush` (+5000 Bp in the step-1 additive bucket, plus the Canon step 2.4b flank multiplier when it applies). While invisible a stalker moves at crouch speed 2000 mm/s with 25 dB footsteps and emits a 30 dB breath noise every 60 ticks, so the Canon §6.4 row 34 footstep indicator and doc 09 hearing can reveal it. Clients render an invisible enemy as a refraction shimmer: fully visible within 2000 mm, otherwise at the doc 17 shimmer opacity. Enemies never receive the doc 09 sneak-attack multiplier against Errata, because Errata have no awareness meter (Canon §10.5 step 2.4).

### 5.5 Enemy summons

`enemy.ab.summon` creates entities from the summoner's genre roster with role swarmer, rank minion and tag `summoned`, at walkable cells 1000 to 3000 mm from the summoner chosen with keyed `rng.ai` seq 1 (one `range` per summon). Summons join the summoner's squad, grant 0 kill XP, despawn 60 ticks after their summoner dies and after 1800 ticks of existence, count toward `ACTIVE_ACTOR_CAP`, and are capped at 4 per summoner and 24 hostile summons per Folio. Elite and boss summons follow their own entries in §7 and §9.

### 5.6 Mounted enemies

A cavalry template spawns two linked entities: the rider (role cavalry) and a mount built from the template's `mount` archetype (`arch.mount.runner` or `arch.mount.hover`, doc 05) with `hp_max = mount_hp_mhp × H(d) × rider rank HP × party HP` (mount template HP 80, body `body.quadruped` for beasts and `body.construct` for vehicles). The rider has `st.mounted`, uses the mount's movement speeds (Canon §17.7) and nav class `ai.nav.mounted`, and both entities keep separate hitboxes. A melee charge at 8000 mm/s or more deals × 12500 damage and × 20000 poise damage (Canon §17.7).

- **Mount dies:** the rider gains `st.knocked_down` (Canon §17.7) and continues on foot with `ai.set.cavalry` minus `charge`; it remounts a riderless mount of its faction within 8000 mm in 30 ticks.
- **Rider dies:** a beast mount flees for 600 ticks and then joins `fac.wild.<genre_suffix>`; a vehicle mount stops and becomes inert. The mount drops its `cat.mount` token with chance 1500 Bp, one keyed `rng.loot` roll per player (actor = mount EntityId, seq = slot).
- **XP:** the rider pays its rank's kill XP; the mount pays 0.
- In rooms tagged `room.no_mount` (doc 12) cavalry spawns dismounted, without a mount entity.

### 5.7 Constructs and `st.hacked`

Only `body.construct` and `electronic` entities can gain `st.hacked` (Canon §11.1); bosses, editors and hunters are immune (Canon §11.4). When it starts on construct `x`, hacked by Erratum `h`:

1. `x` aborts its action (`Interrupted`), releases tokens and leaves its squad. `bb.owner = h`, `bb.command = Attack`, and its faction becomes `fac.errata` for the duration, so friendly fire (Canon §10.5 step 2.15) applies between `x` and Errata.
2. `x` runs `ai.set.minion`. Its target is the entity `h` damaged most recently within 180 ticks, otherwise the hostile to `fac.errata` nearest to `x` within 16000 mm. It does not count toward `MINION_CAP_PER_PLAYER` or `FOLIO_MINION_CAP`.
3. Its former allies hold it hostile: their awareness toward `x` is set to `aware.alert` and it enters their target lists.
4. A hacked construct guarding a locked electronic door opens it as a key holder.
5. On expiry `x` returns to its original faction, rejoins the nearest squad of that faction within 12000 mm, sets awareness toward `h` to `aware.alert` and adds 5000 threat to `h`. Post-proc protection follows Canon §11.2.
6. Kill XP: if `x` dies while hacked, the party receives its kill XP as a defeat. Kills made by `x` count as Errata-side kills for attribution (§11.1).

## 6. The minion contract

This section is the full allied-AI contract for Canon §15.1.

**Kinds.** `summon` (doc 06 `summon.*` templates), `tamed` (an enemy template bound by Handling), `hireling` (§13.4) and `companion` (§13.5). Every minion is in `fac.errata`, carries `owner_slot` and has `st.bound` while bound. Binding emits `ev.minion_bound`; release, death or despawn emits `ev.minion_released`.

**Caps.** `MINION_CAP_PER_PLAYER = min(5, 1 + floor_div(max(skill.conjuration, skill.handling), 25))`, `FOLIO_MINION_CAP = 16`, both inside `ACTIVE_ACTOR_CAP` (Canon §15.1). A bind that would exceed a cap fails and emits nothing. Hacked constructs are not minions.

**Taming.** An Erratum taming a creature tagged `beast` of rank minion or standard, not `summoned`, channels an interact for 120 ticks while the creature has `st.sleeping` or `st.charmed`, or has HP below 3000 Bp and is not in `Windup` or `Active`. The bind succeeds when `skill.handling ≥ 10 + 2 × d + (15 if rank standard else 0)`, with no draw. Example: a standard beast in Folio 12 needs Handling 49.

**Stats.**

```
summon:            every magnitude = base × P(src_lvl)                       (Canon §13.8.1, doc 06)
tamed, hireling:   hp_max = mul_bp(mul_bp(tpl.hp_mhp, T(d)), 15000)
                   B      = mul_bp(mul_bp(tpl_atk_mhp, P(d)), 10000)
                   AR     = mul_bp(tpl.ar, T(d));  subs = tpl + floor_div(3 × (d - 1), 2), capped at 150
```

Ranks and party-size tables never apply to minions. Tamed beasts and hirelings re-evaluate on `ev.folio_entered`, keeping their HP ratio (Canon §12 max-change rule). Example: a tamed troll (110 HP, 18 damage, AR 15) in Folio 12 has 297,330 mHP, hits for 46,447 mHP and has AR 27.

**Commands** (`input.minion_follow`, `input.minion_hold`, `input.minion_attack`, doc 17):

| Command | Behavior |
|---|---|
| Follow (default) | keep slot `k` (0-based bind order) at 2500 mm behind the owner, bearing `owner.yaw + 32768 + (k - 2) × 5461` Bam; engage hostiles that attacked the owner or a minion of the owner in the last 180 ticks, or that are alert toward the party within 8000 mm of the owner |
| Hold | stay on the commanded cell; engage hostiles within 12000 mm of it |
| Attack | attack the entity under the owner's crosshair (48000 mm ray, doc 05 hitscan rules) until it dies or 600 ticks pass, then Follow |

**Targeting.** Minions use §4.8 with two changes: candidates are hostiles within 16000 mm of the owner that are alert toward an Erratum or a minion, and candidates that damaged the owner within 180 ticks get +3000. Minion fire lanes treat every `fac.errata` capsule as blocking.

**Leash.** A minion more than 32000 mm of path from its owner, with no target for 120 ticks, teleports to a free walkable cell within 3000 mm behind the owner that no alerted hostile can see. It never teleports into or out of a sealed boss arena unless the owner is inside.

**Owner states.** Owner downed: `guard_owner` (stay within 4000 mm, attack the nearest threat to the owner). Owner a ghost: summons despawn; tamed beasts, hirelings and companions hold at the Remains, follow any Erratum carrying them, and rebind when the ghost resurrects (Canon §17.6).

**Folio transitions, death and XP.** Spell summons despawn on `ev.folio_exited`. Tamed beasts, hirelings and companions within 16000 mm of the exit follow, persist for the run and are lost permanently on death. A minion's kill grants kill XP as if its owner made it (Canon §15.1). Friendly fire follows Canon §17.5.

## 7. Ranks, elite modifiers and champions

### 7.1 Ranks

Ranks and their multipliers are Canon §17.3. The Director's population plan (§10.1) fixes the count of each rank per Folio. Elite and champion entities are standard roster templates promoted by the Director; swarmers may become elites but never champions. Bosses, Editors and hunters use their own templates.

### 7.2 Elite modifiers

Every elite carries one non-aspect modifier. Every champion carries one champion aspect and, from Folio 11, one non-aspect modifier. Modifiers are innate with empty `dep`, so they never Contradict. Durations and radii are exact; damage multipliers apply to the carrier's template damage and then follow §2.2.

| ID | Aspect | Effect | Min Folio | Excluded roles |
|---|---|---|---|---|
| `elite.warded` | yes | Canon §17.3a; countered by `counter.ward_breaker` | 3 | — |
| `elite.surging` | yes | Canon §17.3a; countered by `counter.disruptor` | 3 | — |
| `elite.unstoppable` | yes | Canon §17.3a; countered by `counter.staggerer` | 3 | — |
| `elite.vampiric` | no | heals 1500 Bp of the HP damage it deals | 1 | — |
| `elite.molten` | no | immune to `st.burning`; attack shares scaled by 7000 and `dmg.fire` 3000 added; on death a 3000 mm environmental `dmg.fire` blast of base 15 HP (Canon §10.6) | 1 | — |
| `elite.glacial` | no | immune to `st.frozen`; Errata within 4000 mm gain direct `st.chilled` each status pulse | 1 | — |
| `elite.voltaic` | no | each hit adds 40 `st.shocked` power; on death direct `st.shocked` to every entity within 3000 mm | 1 | — |
| `elite.thorned` | no | reflects 2000 Bp of melee HP damage taken as `dmg.slash` thorns (Canon §10.6) | 1 | — |
| `elite.hasty` | no | move speed × 13000; `cycle`, windup and recovery × 8000, rounded up | 1 | — |
| `elite.juggernaut` | no | AR × 20000; poise × 20000; move × 8500; immune to `st.knocked_down` | 1 | swarmer |
| `elite.shrouded` | no | gains `st.invisible` after 180 ticks without attacking or taking damage | 1 | stalker |
| `elite.brood` | no | every 600 ticks spawns 2 rank-minion swarmers of its roster, tag `summoned`, at most 4 alive | 1 | summoner, swarmer |
| `elite.splitting` | no | on death spawns 2 copies at rank minion, tag `summoned`, without modifiers | 6 | swarmer |
| `elite.mirrored` | no | the first projectile hit each 240 ticks is reflected to its source with the same damage and no crit | 6 | — |
| `elite.blinking` | no | every 300 ticks, when its target is visible and farther than 6000 mm, teleports to a walkable cell 2000 to 3000 mm behind the target after a 30-tick `marker` telegraph | 6 | cavalry |
| `elite.redlined` | no | each hit pours 1000 ml of `liq.red_ink` on the target's feet cell (Canon §12.1), at most once per 600 ticks | 6 | — |
| `elite.miasmic` | no | immune to `st.poisoned`; emits 1000 ml of `gas.miasma` into its cell every 120 ticks | 6 | — |
| `elite.rallying` | no | every 300 ticks, allies of its faction within 12000 mm gain direct `st.empowered` and +1500 morale | 11 | — |
| `elite.gravitic` | no | every 360 ticks, after a 45-tick `ring` telegraph, pulls every Erratum within 10000 mm 3000 mm toward itself over 20 ticks | 11 | — |
| `elite.vengeful` | no | on death, after a 90-tick `circle` telegraph, a 4000 mm `dmg.arcane` blast with `dmg_mult_bp` 30000 | 11 | — |

**Selection.** When the Director promotes a template, it draws exactly one `weighted` call from `rng.spawn` over the non-aspect modifiers in ascending ID byte order, weight 1000 each, weight 0 when the Folio is below the minimum or the role is excluded. A champion from Folio 11 makes this draw after its aspect assignment.

### 7.3 Champions and counters

Each Folio has an **aspect set**, fixed as the first operation on that Folio's `rng.spawn` stream: shuffle `[elite.surging, elite.unstoppable, elite.warded]` (ascending ID byte order) with `Rng::shuffle`, which calls `range` exactly twice, and keep the first `A(f)` entries: `A = 0` for Folios 1 and 2 (the shuffle still runs), 1 for Folios 3 to 10, 2 for Folios 11 to 20, 3 from Folio 21. The exit preview of the previous Folio derives a temporary copy of the same stream and runs the same procedure, so the preview and the Folio always agree, and the aspect set is independent of the genre chosen. Champion `i` of the Folio (0-based, ascending EntityId at spawn) takes aspect `set[i % A]`.

| Aspect | Counter (doc 05 `counter.*`) | What the counter does to this champion |
|---|---|---|
| `elite.warded` | `counter.ward_breaker` | × 30000 against its ward; breaking the ward applies direct `st.stunned` |
| `elite.surging` | `counter.disruptor` | direct `st.unraveled` and `st.silenced` for 180 ticks; stops its 2% Max HP per second regeneration |
| `elite.unstoppable` | `counter.staggerer` | poise damage × 30000 even though it is immune to poise stagger |

Every genre's shops and loot pools carry a dependency-none source of each counter type (Canon §17.3a, doc 05, doc 11). Champion AI: a surging champion with `st.unraveled` or `st.shocked` adds `ai.action.retreat` weight × 15000 until the status ends; a warded champion whose ward broke uses `take_cover` at weight × 15000 for 180 ticks.

## 8. Genre rosters

Each genre lists its factions and at least 6 enemies. Column conventions: **Name** is the suffix of `enemy.<genre_suffix>.<name>`. **Faction** is the `<group>` of `fac.native.<genre_suffix>.<group>`, or `wild` for `fac.wild.<genre_suffix>`. **Signature** is the suffix of `enemy.ab.<name>` and its effect, in addition to the role defaults of §3. **Dep** is the axiom dependency set of the basic attack and the signature (Canon §5.3). **Loot** is the suffix of `loot.drop.<pool>` (doc 11). **W** is the Director weight. Every roster has a swarmer, which is the pool for `enemy.ab.summon` and `elite.brood`; Editor squads (§8.15) join every genre's draw at weight 600.

### 8.1 Mythic Fantasy

Factions: `goblin_warrens`, `kingsguard`, `mage_conclave`, wild.

| Name | Role | Body | Kin; tags | Faction | Signature | Dep | Loot | W |
|---|---|---|---|---|---|---|---|---|
| `goblin_cutter` | swarmer | humanoid | beastkin; sapient | goblin_warrens | `gang_up`: +2000 Bp step-1 bonus per other goblin within 3000 mm of the target, at most +6000 | — | armed | 2000 |
| `goblin_sapper` | sapper | humanoid | beastkin; sapient | goblin_warrens | `powder_keg`: demo charge with shares blunt 5000, fire 5000 and `st.burning` power 60 | — | armed | 1000 |
| `kingsguard_knight` | guardian | humanoid | human; sapient | kingsguard | `shield_wall`: `block_bp` 9000 while 2 or more kingsguard stand within 3000 mm | — | armed | 1500 |
| `conclave_battlemage` | caster | humanoid | human; sapient | mage_conclave | `firebolt`: `dmg.fire` 10000, `st.burning` power 50, ignites flammable blocks | arcana | caster | 1500 |
| `cave_troll` | brute | humanoid | —; beast | wild | `troll_regen`: regenerates 300 Bp of Max HP per second unless fire or acid damaged it within 300 ticks | — | beast | 1500 |
| `wyvern` | flier | avian | —; beast, flying | wild | `venom_stinger`: dive with `st.poisoned` power 80 | — | beast | 1000 |
| `kingsguard_lancer` | cavalry | humanoid on runner (warhorse) | human; sapient, mounted | kingsguard | `lance_charge` (§3) | — | armed | 1500 |

### 8.2 Gothic Horror

Factions: `night_court`, `inquisition`, `hollow_village`, wild.

| Name | Role | Body | Kin; tags | Faction | Signature | Dep | Loot | W |
|---|---|---|---|---|---|---|---|---|
| `rat_swarm` | swarmer | swarm | —; beast, swarm, mindless | wild | `gnaw`: `st.bleeding` power 30 per hit | — | swarm | 2000 |
| `ghoul` | skirmisher | humanoid | undead; mindless | night_court | `corpse_feast`: `eat_corpse` heals 2500 Bp of Max HP | mythos | undead | 2000 |
| `vampire_courtier` | stalker | humanoid | undead; sapient | night_court | `mist_step`: `vanish` plus a 6000 mm teleport; heals 1000 Bp of HP damage dealt | mythos | undead | 1000 |
| `witch_hunter` | sniper | humanoid | human; sapient | inquisition | `blessed_bolt`: crossbow, shares pierce 7000, radiant 3000 | mythos | armed | 1500 |
| `torch_villager` | soldier (melee) | humanoid | human; sapient | hollow_village | `torch_and_fork`: melee with `st.burning` power 40; ignites flammable blocks | — | armed | 1500 |
| `werewolf` | brute | humanoid | beastkin; beast | wild | `moonfrenzy`: below 5000 Bp HP, direct `st.berserk` for 6 s, once | mythos | beast | 1000 |
| `flesh_golem` | guardian | humanoid | construct; mindless | night_court | `galvanic_heart`: `dmg.shock` taken heals 5000 Bp of its amount instead | arcana | construct | 1000 |

### 8.3 Cosmic Horror

Factions: `esoteric_order`, `deep_ones`, `expedition`, wild.

| Name | Role | Body | Kin; tags | Faction | Signature | Dep | Loot | W |
|---|---|---|---|---|---|---|---|---|
| `polyp_swarm` | swarmer | swarm | —; eldritch, swarm, mindless | wild | `wing_dust`: `st.blinded` power 30 per hit | mythos | swarm | 2000 |
| `cultist` | caster | humanoid | human; sapient | esoteric_order | `unspeakable_chant`: 180-tick channel; 20 `st.dread` power per status pulse to Errata within 6000 mm | mythos | caster | 2000 |
| `deep_one` | brute | humanoid | mutant (gills); eldritch, aquatic, sapient | deep_ones | `drag_below`: grab with direct `st.grappled`, dragging toward the nearest liquid cell within 8000 mm | mythos | eldritch | 1500 |
| `shoggoth_spawn` | summoner | amorphous | —; eldritch, mindless | wild | `proto_bud`: `summon` of 2 `polyp_swarm`; `resist[dmg.slash]` +3000 | mythos | eldritch | 1000 |
| `expedition_rifleman` | sniper | humanoid | human; sapient | expedition | `steady_aim`: after 120 ticks without moving, crit chance +2000 Bp | techne | armed | 1000 |
| `star_seer` | controller | humanoid | mutant (extra_arm); eldritch, sapient | esoteric_order | `gaze_of_unmaking`: `st.confused` power 70 and composure −5 per hit | mythos | eldritch | 1500 |
| `lurker_below` | stalker | serpentine | —; eldritch, aquatic | wild | `undertow`: attacks from liquid cells; `st.knocked_down` direct on the first hit | mythos | eldritch | 1000 |

### 8.4 Space Opera

Factions: `imperial_navy`, `void_pirates`, `hive_swarm`, `psionic_order`, wild.

| Name | Role | Body | Kin; tags | Faction | Signature | Dep | Loot | W |
|---|---|---|---|---|---|---|---|---|
| `navy_trooper` | soldier | humanoid | human; sapient | imperial_navy | `blaster_volley` (§2.1) | techne | armed | 2000 |
| `psi_adept` | caster | humanoid | human; sapient | psionic_order | `psi_lance`: `beam`, `dmg.psychic` 10000, composure −5 per hit | arcana | caster | 1000 |
| `pirate_boarder` | skirmisher | humanoid | mutant (hide); sapient | void_pirates | `breach_jet`: 8000 mm jetpack leap; breaks one door or breakable block on landing | techne | armed | 1500 |
| `hive_drone` | swarmer | insectoid | —; beast, mindless | hive_swarm | `acid_spit`: projectile `dmg.acid`, `st.corroded` power 40 | — | swarm | 2000 |
| `hive_warrior` | brute | insectoid | —; beast | hive_swarm | `carapace_charge`: `charge` with AR × 15000 during the charge | — | beast | 1500 |
| `security_droid` | guardian | construct | —; construct, electronic | imperial_navy | `energy_shield`: frontal `block_bp` 8000 that also blocks projectiles | techne | construct | 1000 |
| `hover_lancer` | cavalry | humanoid on hover (speeder) | human; sapient, mounted | void_pirates | `strafing_run`: passes the target at 14000 mm/s firing a 4-round burst | techne | armed | 1000 |

### 8.5 Cyberpunk

Factions: `zaibatsu_security`, `chrome_gang`, `netrunner_collective`, wild.

| Name | Role | Body | Kin; tags | Faction | Signature | Dep | Loot | W |
|---|---|---|---|---|---|---|---|---|
| `street_punk` | swarmer | humanoid | human; sapient | chrome_gang | `switchblade_rush`: `leap` plus `st.bleeding` power 30 | — | armed | 2000 |
| `corp_enforcer` | soldier | humanoid | human; sapient | zaibatsu_security | `smart_rounds`: projectiles home with 2048 Bam per tick of turn | techne | armed | 2000 |
| `chrome_brawler` | brute | humanoid | construct; sapient, electronic | chrome_gang | `reflex_overclock`: direct `st.hasted` 4 s, cooldown 900 | techne | armed | 1500 |
| `netrunner` | controller | humanoid | human; sapient | netrunner_collective | `short_circuit`: through walls within 20000 mm, 60 `st.shocked` power to an Erratum with any `cat.implant`, or disables one Errata gadget for 300 ticks | techne | caster | 1000 |
| `mantis_assassin` | stalker | humanoid | construct; sapient, electronic | chrome_gang | `optical_camo`: `vanish` with cooldown 450 | techne | armed | 1000 |
| `security_drone` | flier | construct | —; construct, electronic, flying | zaibatsu_security | `taser_dart`: projectile `dmg.shock`, 40 `st.shocked` power | techne | construct | 1500 |
| `riot_mech` | artillery | construct | —; construct, electronic | zaibatsu_security | `microsalvo`: `barrage` of 6 lobs | techne | construct | 1000 |

### 8.6 Post-Apocalypse

Factions: `road_raiders`, `vault_remnant`, `glow_cult`, wild.

| Name | Role | Body | Kin; tags | Faction | Signature | Dep | Loot | W |
|---|---|---|---|---|---|---|---|---|
| `rad_roach` | swarmer | insectoid | —; beast, mindless | wild | `rad_bite`: `st.irradiated` power 30 | — | swarm | 2000 |
| `raider_scav` | soldier | humanoid | human; sapient | road_raiders | `pipe_rifle`: 1500 Bp jam chance on each shot, rolled on keyed `rng.combat` | techne | armed | 2000 |
| `war_rig_rider` | cavalry | humanoid on runner (motorbike) | human; sapient, mounted | road_raiders | `ram`: charge × 15000 and knockback 4000 mm | techne | armed | 1000 |
| `glow_zealot` | artillery | humanoid | mutant (regrowth); sapient | glow_cult | `rad_burst`: lob with `dmg.radiation` 10000 and `st.irradiated` power 60 | mythos | caster | 1000 |
| `bunker_sentry` | guardian | construct | —; construct, electronic | vault_remnant | `arc_shield`: frontal `block_bp` 8000; blocked hits chain 2000 Bp `dmg.shock` to the attacker | techne | construct | 1000 |
| `mutant_behemoth` | brute | humanoid | mutant (hide); beast | wild | `boulder_toss`: digs a block and throws it, × 20000 `dmg.blunt`, windup 45 | — | beast | 1500 |
| `tunnel_mole` | sapper | humanoid | mutant (regrowth); burrowing, sapient | vault_remnant | `burrow`: digs at `dig_speed_bp` 30000 and surfaces under the target | — | armed | 500 |

### 8.7 Weird West

Factions: `outlaw_gang`, `lawmen`, `railroad_company`, `hex_coven`, wild.

| Name | Role | Body | Kin; tags | Faction | Signature | Dep | Loot | W |
|---|---|---|---|---|---|---|---|---|
| `dead_miner` | swarmer | humanoid | undead; mindless | wild | `dynamite_belt`: on death a 2500 mm `dmg.fire` blast of base 12 HP after a 60-tick `circle` telegraph | — | undead | 2000 |
| `outlaw_gunhand` | shooter | humanoid | human; sapient | outlaw_gang | `fan_the_hammer`: 6-round burst 4 ticks apart, × 3000 each | techne | armed | 2000 |
| `marshal_rifleman` | sniper | humanoid | human; sapient | lawmen | `high_noon`: a 90-tick standoff telegraph; if the target is visible at its end, the shot is a guaranteed crit at × 15000 | verve | armed | 1000 |
| `hexslinger` | caster | humanoid | human; sapient | hex_coven | `hex_bullet`: `dmg.umbral` 10000, `st.withering` power 60 | mythos | caster | 1500 |
| `railroad_automaton` | brute | construct | —; construct, electronic | railroad_company | `steam_ram`: `charge` that leaves 3 cells of `gas.steam` | techne | construct | 1500 |
| `thunderbird` | flier | avian | —; beast, divine, flying | wild | `thunderclap`: `area` 4000 mm `dmg.shock`, 40 `st.shocked` power, windup 45 | mythos | beast | 500 |
| `posse_rider` | cavalry | humanoid on runner (horse) | human; sapient, mounted | lawmen | `lasso`: 12000 mm projectile, direct `st.grappled` and a 4000 mm pull | — | armed | 1500 |

### 8.8 Hardboiled Noir

Factions: `syndicate`, `city_police`, `dockworkers`, wild.

| Name | Role | Body | Kin; tags | Faction | Signature | Dep | Loot | W |
|---|---|---|---|---|---|---|---|---|
| `street_tough` | swarmer | humanoid | human; sapient | syndicate | `brass_knuckles`: 20 `st.stunned` power per hit | — | armed | 2000 |
| `mob_torpedo` | shooter | humanoid | human; sapient | syndicate | `drum_burst`: 8-round burst 6 ticks apart, × 2000 each | techne | armed | 2000 |
| `leg_breaker` | brute | humanoid | human; sapient | syndicate | `kneecap`: hits on leg parts apply direct `st.slowed` 4 s | — | armed | 1500 |
| `crooked_cop` | soldier | humanoid | human; sapient | city_police | `call_backup`: once per Folio, `call_allies` that adds 2 standard `crooked_cop` at the nearest spawn anchor out of sight | techne | armed | 1500 |
| `shadow_tail` | stalker | humanoid | human; sapient | syndicate | `garrote`: from behind, direct `st.grappled` 3 s and `st.bleeding` power 60 | — | armed | 1000 |
| `dock_heavy` | guardian | humanoid | human; sapient | dockworkers | `crate_shield`: carries a crate as a frontal shield with `block_bp` 8000 that breaks after absorbing 3 × its Max HP | — | armed | 1000 |
| `getaway_driver` | cavalry | humanoid on runner (sedan) | human; sapient, mounted | syndicate | `drive_by`: `strafing_run` with a 3-round burst | techne | armed | 1000 |

### 8.9 Wuxia

Factions: `iron_palm_sect`, `jade_court_guard`, `lotus_assassins`, wild.

| Name | Role | Body | Kin; tags | Faction | Signature | Dep | Loot | W |
|---|---|---|---|---|---|---|---|---|
| `jiangshi` | swarmer | humanoid | undead; mindless | wild | `breath_sense`: senses `breath` only, no sight; a creature holding its breath (doc 09) is undetectable to it | mythos | undead | 2000 |
| `sect_disciple` | skirmisher | humanoid | human; sapient | iron_palm_sect | `qinggong_leap`: 10000 mm leap over walls up to 3 blocks, cooldown 300 | verve | armed | 2000 |
| `iron_palm_elder` | brute | humanoid | human; sapient | iron_palm_sect | `iron_palm`: × 20000 `dmg.blunt`, knockback 5000 mm, windup 36 | verve | armed | 1000 |
| `court_crossbowman` | sniper | humanoid | human; sapient | jade_court_guard | `repeating_volley`: 5 bolts 6 ticks apart, × 3000 each | — | armed | 1500 |
| `lotus_assassin` | stalker | humanoid | human; sapient | lotus_assassins | `thousand_needles`: cone 4096 Bam, 6000 mm, `st.poisoned` power 80 | verve | armed | 1000 |
| `fox_spirit` | controller | quadruped | —; fae, beast | wild | `foxfire_charm`: `st.charmed` power 60 (Errata use Canon §11.8) | arcana | beast | 1000 |
| `cavalry_general` | cavalry | humanoid on runner (horse) | human; sapient, mounted | jade_court_guard | `halberd_sweep`: arc 16384 Bam, × 15000, windup 30 | — | armed | 1500 |

### 8.10 Steampunk

Factions: `clockwork_guild`, `sky_pirates`, `luddite_brigade`, wild.

| Name | Role | Body | Kin; tags | Faction | Signature | Dep | Loot | W |
|---|---|---|---|---|---|---|---|---|
| `clockwork_scarab` | swarmer | construct | —; construct, electronic, mindless | clockwork_guild | `wind_down`: stops for 60 ticks after every 600 ticks of activity | techne | construct | 2000 |
| `brass_automaton` | soldier | construct | —; construct, electronic | clockwork_guild | `wind_up_burst`: after 60 ticks of `reload`, its next volley is × 15000 | techne | construct | 2000 |
| `sky_aeronaut` | flier | humanoid | human; sapient, flying | sky_pirates | `grapnel_swing`: 15000 mm grapple to a solid block, then a dive attack | techne | armed | 1500 |
| `galvanist` | caster | humanoid | human; sapient | clockwork_guild | `arc_coil`: `dmg.shock` 10000 that chains to 2 more targets within 4000 mm at 5000 Bp | techne | caster | 1000 |
| `luddite_wrecker` | sapper | humanoid | human; sapient | luddite_brigade | `sabot`: melee hits disable one Errata gadget, turret or drone for 600 ticks | — | armed | 1000 |
| `steam_juggernaut` | guardian | construct | —; construct, electronic | clockwork_guild | `boiler_vent`: emits 4 cells of `gas.steam` around itself every 300 ticks | techne | construct | 1000 |
| `sewer_gator` | brute | quadruped | —; beast, aquatic | wild | `death_roll`: grab with direct `st.grappled`, then × 25000 `dmg.slash` after 60 ticks unless the grab breaks | — | beast | 1500 |

### 8.11 Fairy Tale

Factions: `seelie_court`, `unseelie_court`, `woodcutters`, wild.

| Name | Role | Body | Kin; tags | Faction | Signature | Dep | Loot | W |
|---|---|---|---|---|---|---|---|---|
| `pixie_swarm` | swarmer | swarm | —; fae, swarm, flying | seelie_court | `glamour_dust`: `st.confused` power 30 per hit | arcana | swarm | 2000 |
| `big_bad_wolf` | skirmisher | quadruped | —; beast, sapient | wild | `huff_and_puff`: cone 5461 Bam, 8000 mm, knockback 4000 mm, breaks wooden blocks, windup 45 | arcana | beast | 1500 |
| `redcap` | brute | humanoid | fae; sapient | unseelie_court | `bloody_cap`: heals 2000 Bp of HP damage dealt to `st.bleeding` targets | mythos | armed | 1500 |
| `hedge_witch` | summoner | humanoid | human; sapient | unseelie_court | `gingerbread_golems`: `summon` of 2 swarmer constructs instead of the roster swarmer | arcana | caster | 1000 |
| `woodcutter` | soldier (melee) | humanoid | human; sapient | woodcutters | `cold_iron_axe`: primary material `mat.cold_iron`, so fae Errata take × 12500 (Canon §16.3) | — | armed | 1500 |
| `thorn_knight` | guardian | humanoid | fae; sapient | seelie_court | `briar_wall`: places 3 `block.web` cells in a line in front of itself, cooldown 600 | arcana | armed | 1000 |
| `wild_hunt_rider` | cavalry | humanoid on runner (spectral steed) | fae; sapient, mounted | unseelie_court | `horn_of_the_hunt`: `area` 10000 mm, `st.feared` power 60, windup 45 | mythos | armed | 1500 |

### 8.12 Mythic Antiquity

Factions: `hoplite_league`, `maenad_thiasos`, `temple_oracles`, wild.

| Name | Role | Body | Kin; tags | Faction | Signature | Dep | Loot | W |
|---|---|---|---|---|---|---|---|---|
| `maenad` | swarmer | humanoid | human; sapient | maenad_thiasos | `frenzy`: direct `st.berserk` when an ally dies within 8000 mm | mythos | armed | 2000 |
| `hoplite` | guardian | humanoid | human; sapient | hoplite_league | `phalanx`: +5 AR per other hoplite within 2000 mm | — | armed | 2000 |
| `spartoi` | soldier (melee) | humanoid | undead; mindless | hoplite_league | `reassemble`: once, 300 ticks after death, rises at 5000 Bp HP unless the killing hit was mostly `dmg.blunt` or `dmg.radiant`; XP is paid on the final death | mythos | undead | 1500 |
| `harpy` | flier | avian | —; beast, flying | wild | `snatch`: grab and carry a target 3 blocks up for 60 ticks, then drop it | — | beast | 1500 |
| `oracle_priestess` | support | humanoid | human; sapient | temple_oracles | `foresight`: direct `st.blessed` on one ally; that ally sidesteps the next telegraphed attack | mythos | caster | 1000 |
| `bronze_bull` | brute | quadruped | —; construct | wild | `fire_breath`: cone 5461 Bam, 6000 mm, `dmg.fire` × 3000 per 10 ticks for 60 ticks | mythos | construct | 1000 |
| `centaur_archer` | sniper | quadruped | —; sapient | wild | `parthian_shot`: fires while retreating at run speed | — | armed | 1000 |

### 8.13 Oneiric (secret)

Factions: `lucid_dreamers`, `nightmare_court`, wild.

| Name | Role | Body | Kin; tags | Faction | Signature | Dep | Loot | W |
|---|---|---|---|---|---|---|---|---|
| `sleepwalker` | swarmer | humanoid | human; mindless | wild | `somnambulance`: immune to `st.sleeping`; hits add 30 `st.sleeping` power | arcana | swarm | 2500 |
| `nightmare_mare` | skirmisher | quadruped | —; fae, mindless | nightmare_court | `dread_trample`: charge with 50 `st.dread` power | mythos | beast | 2000 |
| `stair_mimic` | stalker | amorphous | —; eldritch | wild | `block_mimicry`: while idle it is drawn as a stair block and is not a sight target | arcana | eldritch | 1500 |
| `faceless_teacher` | controller | humanoid | mutant (extra_arm); sapient | nightmare_court | `pop_quiz`: direct `st.silenced` and `st.confused`, windup 45, token control | arcana | caster | 1500 |
| `gravity_kite` | flier | avian | —; fae, flying | lucid_dreamers | `gravity_flip`: direct `st.levitating` 60 ticks on the target, then a 3000 mm slam | verve | beast | 1500 |
| `lullaby_choir` | support | humanoid | human; sapient | lucid_dreamers | `lullaby`: allies within 10000 mm regenerate 200 Bp of Max HP per second while it channels | arcana | caster | 1000 |

### 8.14 The Redaction (secret)

Factions: `bureau_of_correction`, `whistleblowers` (starting reputation +150, friendly).

| Name | Role | Body | Kin; tags | Faction | Signature | Dep | Loot | W |
|---|---|---|---|---|---|---|---|---|
| `clerk` | swarmer | humanoid | human; sapient | bureau_of_correction | `form_27b`: direct `st.marked` on hit | techne | armed | 2500 |
| `compliance_officer` | soldier | humanoid | human; sapient | bureau_of_correction | `cease_and_desist`: direct `st.disarmed`, windup 40, token control | techne | armed | 2000 |
| `blackbar_sentinel` | guardian | construct | —; construct, electronic | bureau_of_correction | `redaction_field`: cells within 3000 mm have concealment 15 | techne | construct | 1500 |
| `shredder_unit` | brute | construct | —; construct, electronic | bureau_of_correction | `shred`: `dmg.slash` with `st.bleeding` power 60; destroys ground items it walks over that are not Errata-owned | techne | construct | 1500 |
| `auditor` | sniper | humanoid | human; sapient | bureau_of_correction | `audit`: `st.exposed` power 80 | mythos | armed | 1500 |
| `faceless_agent` | stalker | humanoid | mutant (hide); sapient | bureau_of_correction | `unperson`: `vanish`; its kills leave no Remains for 300 ticks | mythos | armed | 1000 |

### 8.15 The Unwritten (secret) and Editor squads

`genre.unwritten` has no native factions; its only enemies are `fac.editors` (Canon §5.2), who count as native there. These six templates are also the Editor squads that join every genre's draw. IDs are `enemy.editors.<name>`; every attack has empty `dep`.

| Name | Role | Body | Kin; tags | Faction | Signature | Loot | W |
|---|---|---|---|---|---|---|---|
| `inkling` | swarmer | amorphous | —; editor, mindless | editors | `blot`: on death pours 1000 ml `liq.red_ink` | editor | 2500 |
| `strikethrough` | skirmisher | humanoid | —; editor | editors | `strike_through`: `line` dash 8000 mm, × 15000 `dmg.slash`, windup 30 | editor | 2000 |
| `marginal_scribe` | caster | humanoid | —; editor | editors | `annotation`: `dmg.arcane` 10000 and direct `st.marked` | editor | 1500 |
| `footnote` | summoner | humanoid | —; editor | editors | `ibid`: summons 2 `inkling` | editor | 1500 |
| `erratum_eater` | brute | amorphous | —; editor | editors | `consume_error`: heals 500 Bp of Max HP per Errata-placed block it erases (§10.6) | editor | 1500 |
| `red_pen_lancer` | sniper | humanoid | —; editor | editors | `correction_lance`: `beam`, × 20000 `dmg.arcane`, `pen_bp` 5000, windup 60 | editor | 1000 |

### 8.16 The Ur-Text (secret)

Factions: `first_drafts`, `glossators`, wild.

| Name | Role | Body | Kin; tags | Faction | Signature | Dep | Loot | W |
|---|---|---|---|---|---|---|---|---|
| `stray_letters` | swarmer | swarm | —; swarm, mindless | wild | `ligature`: the third hit on one target within 120 ticks applies direct `st.slowed` | — | swarm | 2000 |
| `first_draft` | soldier | humanoid | human; sapient | first_drafts | `unfinished`: renders as the soldier of the genre visited last in this run; mechanics unchanged | — | armed | 2000 |
| `palimpsest_beast` | brute | quadruped | —; beast | wild | `overwrite`: its basic attack's damage type cycles fire, frost, shock, acid every 300 ticks | — | beast | 1500 |
| `glossator` | support | humanoid | human; sapient | glossators | `gloss`: direct `st.fortified` on one ally, cooldown 600 | arcana | caster | 1500 |
| `lettered_serpent` | skirmisher | serpentine | —; beast | wild | `bind`: direct `st.rooted` on hit, cooldown 300 | — | beast | 1000 |
| `illuminator` | caster | humanoid | human; sapient | glossators | `gilded_initial`: `beam` `dmg.radiant` 10000, `st.blinded` power 50 | mythos | caster | 1000 |
| `archetype_knight` | cavalry | humanoid on runner (archetypal steed) | human; sapient, mounted | first_drafts | `lance_charge` | verve | armed | 1000 |

## 9. Bosses

### 9.1 Boss rules

Notation in this section: `w/a/r` is windup, active and recovery in ticks; `×N` is `dmg_mult_bp / 10000` applied to the boss template damage, then §2.2.

- **Templates.** Mini-bosses (`rank.boss`) have template HP 80 and damage 12. Editors (`rank.editor`) have HP 100 and damage 12; the Last Word has HP 120. Every boss has `resist` 2500 to every type, `subs` 60, poise from BRN 80 plus 60 armor poise, and the Canon §11.4 immunities. Bosses are morale-immune and need no tokens.
- **Arena.** Doc 12 builds each arena from the spec here. When an Erratum enters the arena volume, its doors lock and nav links into it close (the arena is sealed). It unseals when the boss dies, or after 600 ticks with no living Erratum inside, in which case the boss resets to full HP and phase 1. Blocks inside a sealed arena are unbreakable except those a boss attack creates or removes.
- **Phases.** A phase ends when HP falls below its threshold. The boss then performs a 90-tick phase shift: it takes no damage, loses every status, publishes a `ring` telegraph and switches its attack set. Damage beyond the threshold in the triggering hit is discarded.
- **Choice.** Bosses use the §4.5 think with `ai.set.boss`: each attack in the current phase is one action of weight 10000 with considerations `cooldown_ready` → `binary` and `target_dist`[attack range] → `plateau`; the score of the attack used last is then × 5000, so bosses rarely repeat.
- **Enrage.** From 18000 ticks after sealing, the boss gains +1000 Bp in its step-1 additive bucket, and another +1000 every 1800 ticks, up to +10000.
- **Erasure.** Erasure accrual pauses in Editor Folios while the Editor lives (Canon §11.6); mini-boss Folios do not pause.
- **Genre adaptation.** The boss takes its skin from the local genre. Editors also add a **verse** chosen by the local genre's dominant axiom: the highest of ARC, TEC, MYT, VRV, ties broken in that order. Dominant axioms: ARC for Mythic Fantasy, Fairy Tale, Oneiric, Unwritten and Ur-Text; TEC for Space Opera, Cyberpunk, Post-Apocalypse, Hardboiled Noir, Steampunk and The Redaction; MYT for Gothic Horror, Cosmic Horror and Mythic Antiquity; VRV for Weird West and Wuxia.
- **Loot.** Per player, keyed `rng.loot` with actor = boss EntityId and seq = party slot. Mini-boss: 2 drops from `loot.boss.mini`, minimum rarity `rar.rare`. Editor of Volume `v`: the first draw is `roll_bp(palimpsestic_bp[v])` with `palimpsestic_bp = [2500, 4375, 6250, 8125, 10000]`; on success one item from `loot.boss.palimpsestic`; then 3 drops from `loot.boss.editor`, the first at minimum rarity `rar.legendary`. Doc 11 owns the tables and the remaining draw counts.

Boss HP (`hp_max`, mHP) by party size:

| Boss | Folio | 1 | 2 | 3 | 4 | Standard hit B (×1) |
|---|---|---|---|---|---|---|
| Proofreader | 5 | 7,113,600 | 13,515,840 | 19,918,080 | 26,320,320 | 26,757 |
| Redactor | 10 | 13,769,200 | 26,161,480 | 38,553,760 | 50,946,040 | 34,970 |
| Continuity Editor | 15 | 25,530,400 | 48,507,760 | 71,485,120 | 94,462,480 | 45,705 |
| Abridger | 20 | 45,965,600 | 87,334,640 | 128,703,680 | 170,072,720 | 59,736 |
| Last Word | 25 | 97,210,080 | 184,699,152 | 272,188,224 | 359,677,296 | 78,073 |
| any mini-boss | 3 | 2,148,000 | 4,081,200 | 6,014,400 | 7,947,600 | 21,369 |
| any mini-boss | 23 | 25,888,480 | 49,188,112 | 72,487,744 | 95,787,376 | 62,352 |

Example: Proofreader solo, `mul_bp(mul_bp(mul_bp(100000, 17784), 400000), 10000) = 7,113,600`.

### 9.2 The five Editors

IDs `boss.editor.<name>`. Every Editor attack has empty `dep` and shares as listed.

**1. The Proofreader** (`boss.editor.proofreader`, Folio 5). Arena: round reading room, radius 20 m, ceiling 12 m, 8 unbreakable lectern pillars (1 × 1 × 3 blocks) at radius 12 m. Phases end at 6600 and 3300 Bp.

| Attack | Phases | Spec |
|---|---|---|
| `red_mark` | 1–3 | `marker` on the highest-threat Erratum; the marker follows it for 75 ticks, then fixes; impact at tick 120: `circle` r 2500, ×2.5 `dmg.arcane`; a 6, r 30, cooldown 360 |
| `strikethrough` | 1–3 | `line` 30000 × 2000 mm through its target, w 40, a 6, r 24, ×2.0 `dmg.slash`, cooldown 300 |
| `margin_note` | 2–3 | summons 4 `enemy.editors.inkling` at the wall, w 60, cooldown 900 |
| `final_proof` | 3 | pours `liq.red_ink` in a ring at the wall that advances 1000 mm inward every 300 ticks, stopping at radius 8000 mm |

Verses: ARC fires `strikethrough` as 3 lines at −2048, 0 and +2048 Bam. TEC keeps `red_mark` tracking until tick 90. MYT replaces the inklings with 2 standard soldiers of the local roster. VRV adds `pillar_dash`: a teleport to the pillar nearest its target, cooldown 600, followed at once by `strikethrough`.

**2. The Redactor** (`boss.editor.redactor`, Folio 10). Arena: archive 40 × 28 m, ceiling 8 m, 6 shelf walls 2 m thick forming 5 lanes. Phases end at 6600 and 3300 Bp.

| Attack | Phases | Spec |
|---|---|---|
| `bar_sweep` | 1–3 | a black bar 24000 × 1500 × 3000 mm crosses a lane at 4000 mm/s after a 60-tick `line` telegraph; contact ×1.5 `dmg.arcane` and direct `st.silenced`; cooldown 480 |
| `gag_order` | 1–3 | `cone` 8192 Bam, 12000 mm, w 50, a 6, r 30, ×1.2 `dmg.psychic`, direct `st.silenced`, cooldown 360 |
| `classified` | 2–3 | direct `st.invisible` 300 ticks and teleport to a lane end chosen with keyed `rng.ai` seq 1, cooldown 900 |
| `blackout` | 3 | effective light in the arena is 0 for 600 ticks, and two bars sweep continuously; cooldown 1800 |

Verses: ARC bars also apply direct `st.unraveled`. TEC bars move at 6000 mm/s. MYT bars leave a 1-cell `liq.red_ink` trail. VRV follows each `classified` with a 3-hit melee combo, each ×1.0, w 18 per hit.

**3. The Continuity Editor** (`boss.editor.continuity`, Folio 15). Arena: hall 36 × 36 m with a mezzanine 5 m up and 4 ramps. It records each Erratum's cell every 60 ticks in a ring of 6. Phases end at 6600 and 3300 Bp.

| Attack | Phases | Spec |
|---|---|---|
| `crossfade` | 1–3 | 5 projectiles in a 4096 Bam fan, ×0.6 `dmg.arcane` each, w 30, cooldown 120 |
| `retcon` | 1–3 | w 90 with silhouettes at each Erratum's 300-tick-old cell; at impact every Erratum in the arena moves to that cell (nearest walkable cell if it is now blocked) and the Editor heals `min(damage taken in the last 300 ticks, 500 Bp of Max HP)`; cooldown 1200 |
| `continuity_error` | 2–3 | one hostile doppelganger per living Erratum: brute template for a melee-wielding Erratum, soldier template otherwise, rank standard, tag `summoned`, drawn with the Erratum's appearance; cooldown 1800 |
| `flashback` | 3 | repeats its last 3 attacks in order with 20-tick gaps, each with its own telegraph |

Verses: ARC doppelgangers use the caster template. TEC `retcon` windup 75. MYT doppelgangers gain tag `undead` and `enemy.ab.reassemble`. VRV `flashback` repeats 4 attacks.

**4. The Abridger** (`boss.editor.abridger`, Folio 20). Arena: 5 platform segments of 12 × 12 m in a cross with 4 m gaps spanned by 2 m bridges, above a void. An Erratum falling 8 m below the platforms reappears on the center segment and takes 1500 Bp of its Max HP as `dmg.true`. Phases end at 6600 and 3300 Bp.

| Attack | Phases | Spec |
|---|---|---|
| `excerpt` | 1–3 | 3 lobbed bolts at the target's cell, ×0.8 `dmg.arcane`, radius 2000, w 30, cooldown 150 |
| `abridge` | 1–3 | `ring` on one outer segment for 180 ticks, then its blocks are removed for 1800 ticks; at most 2 segments removed at once (3 in phase 3) |
| `condense` | 1–3 | pulls every Erratum 4000 mm toward itself over 30 ticks (w 45), then a `circle` r 4000 slam, w 30, ×2.5 `dmg.blunt`; cooldown 600 |
| `summary_judgment` | 2–3 | one segment glows (keyed `rng.ai` seq 1); w 240; every Erratum not on it takes ×4.0 `dmg.arcane`; cooldown 1500 |

Verses: ARC `summary_judgment` also applies direct `st.unraveled`. TEC `abridge` windup 150. MYT a returning segment brings 2 inklings. VRV the Abridger crosses gaps with a 9000 mm/s charge, w 36, ×2.0 `dmg.blunt`.

**5. The Last Word** (`boss.editor.last_word`, Folio 25, `genre.ur_text`). Arena: a 48 × 48 m ring of four quadrants, each built like one earlier Editor's arena. Phases end at 7500, 5000 and 2500 Bp. Phase 1 uses `red_mark` and `strikethrough`; phase 2 `bar_sweep` and `gag_order`; phase 3 `retcon` and `continuity_error`; phase 4 `abridge` and `summary_judgment`. Every phase also uses:

- `genre_shift`: w 180 with the genre's name shown, then the arena becomes a dynamic bleed zone (doc 12) of genre `G` for 1200 ticks. `G` is the genre among Folios 1 to 24 of this run, secret genres excluded, that maximizes the sum over living Errata of the Contradiction `C` of their equipped base-function components; ties go to the lower Canon §5.2 row. Cooldown 2400 in phases 1 and 2, 1800 in phase 3, 1200 in phase 4. Inside the zone Errata Contradiction applies again (Canon §5.5 overrides only `genre.ur_text`), while the Last Word's empty-`dep` attacks stay Coherent.

### 9.3 Genre mini-bosses

A mini-boss guards the exit of every Folio `f` with `f % 5 == 3` (3, 8, 13, 18, 23, and every such Marginalia Folio), using the mini-boss of that Folio's genre. Each has two phases; phase 1 ends at 5000 Bp. IDs `boss.<genre_suffix>.<name>`.

| ID | Body | Arena | Attacks (phase 2 additions marked P2) |
|---|---|---|---|
| `boss.mythic_fantasy.ember_wyrm` | avian | collapsed cathedral 32 × 32 × 16 m, 4 pillars | `flame_breath` cone 5461 Bam 9000 mm, w60 a90 r40, ×0.4 fire per 10 ticks, `st.burning` 60; `tail_sweep` rear arc 16384 Bam 4000 mm, w24, ×1.5 blunt, knockback 3000; P2 takes flight and adds `firebomb` 3 circles r3000 on Errata, w75, ×2.0 fire |
| `boss.gothic_horror.blood_countess` | humanoid, undead | ballroom 30 × 30 m, 6 coffins | `blood_tide` ring 0 to 12000 mm over 60 ticks, w45, ×1.5 umbral, `st.withering` 60; `bat_form` 180 ticks invisible then a claw behind the target, marker w30, ×2.0 slash; P2 `thrall_call` summons 3 `ghoul` |
| `boss.cosmic_horror.drowned_herald` | serpentine, eldritch | flooded crypt 28 × 28 m, 1 m water | 4 tentacles `bp.weakpoint_1..4`, share 1000 each, precision; `tentacle_slam` line 12000 mm, w36, ×2.0 blunt; `maddening_hymn` area 20000 mm, w60, `st.dread` 80, composure −10; P2 `abyss_pull` drags every Erratum 3000 mm toward it, w45 |
| `boss.space_opera.dread_admiral` | humanoid | bridge deck 36 × 24 m | `orbital_strike` 3 markers r3000, w90, ×3.0 fire; `command_pistol` 3 shots ×0.5, w18; P2 `shield_drones`: 2 construct drones (rank minion, electronic), each granting a `st.shielded` pool of 2000 Bp of its Max HP while alive |
| `boss.cyberpunk.ice_daemon` | construct, electronic | server vault 28 × 28 m, 6 terminals | `black_ice` beam 20000 mm, w45 a60, ×0.3 shock per 10 ticks, `st.shocked` 40; `firewall` ring r8000, w45, ×2.0 fire; P2 activates 2 terminals as turrets (rank minion) every 900 ticks |
| `boss.post_apocalypse.war_boss` | humanoid on a runner war rig | dust arena 48 × 48 m | mounted (§5.6) with rig HP 5000 Bp of its own; `rig_ram` line charge at 16000 mm/s, w45, ×3.0; `flamethrower` cone 5461 Bam, w30 a90, ×0.3 fire per 10 ticks; P2 (or when the rig dies) dismounted, adds `chain_swing` arc 16384 Bam, w36, ×2.0 |
| `boss.weird_west.hanging_judge` | humanoid, undead | gallows square 32 × 32 m | `high_noon_duel` 120-tick marker on each Erratum in turn, a guaranteed crit ×3.0 shot if that Erratum is visible at the end; `verdict` line through the gallows, w45, ×2.0 umbral; P2 `dead_mans_posse` summons 3 `dead_miner` |
| `boss.hardboiled_noir.the_fixer` | humanoid | penthouse 30 × 20 m, glass walls, 8 tables | uses `take_cover` at weight 15000; `drum_spray` 12 rounds ×0.25, w24; `smoke_bomb` 25 cells of `gas.smoke` then relocates; 2 `leg_breaker` at the start and 2 more at P2 |
| `boss.wuxia.sword_saint` | humanoid | bamboo courtyard 28 × 28 m | `thousand_blades` 8 radiating lines, w45, ×1.5 slash; `parry_stance` 90 ticks in which melee hits deal 0 and trigger a ×1.5 riposte (ranged hits land); P2 `qinggong_descent` leap to a circle r3000, w60, ×2.5 |
| `boss.steampunk.brass_colossus` | construct, electronic | foundry 36 × 36 m, 4 vents | rear boiler `bp.weakpoint_1`, share 1500, precision; `hammerfall` circle r4000, w60, ×3.0 blunt; `rivet_gun` 6 rounds ×0.4; P2 vents emit `gas.steam` for 300 ticks every 900 |
| `boss.fairy_tale.wicked_queen` | humanoid, fae | mirror hall 30 × 30 m, 6 breakable mirrors | `mirror_images` 3 decoys with 1 HP each, which copy her telegraphs without damage; `poisoned_apple` lob ×1.0 and `st.poisoned` 100; P2 `curse_of_sleep` cone 8192 Bam, w60, `st.sleeping` 100 |
| `boss.mythic_antiquity.gorgon_matriarch` | serpentine | temple ruins 32 × 32 m, columns | `petrifying_gaze` w60: every Erratum whose facing is within 8192 Bam of the bearing to her gains direct `st.frozen`; `serpent_volley` 6 projectiles ×0.5 toxin, `st.poisoned` 40; P2 `tail_crush` arc, w30, ×2.0 |
| `boss.oneiric.sandman` | humanoid | giant bedroom 48 × 48 m | `dream_sand` area r6000, w45, `st.sleeping` 80; `gravity_flip` w90, arena gravity reversed for 300 ticks (doc 12 volume); P2 summons 2 `nightmare_mare` every 1200 ticks |
| `boss.redacted.director_of_records` | humanoid | open-plan office 40 × 30 m | `file_wall` places a 6 × 3 block wall of cabinets removed on its death; `termination_notice` marker w90, ×3.0 arcane; P2 `memo_storm` 10 projectiles ×0.3 |

Mini-boss loot is `loot.boss.mini` (§9.1). Every mini-boss attack has the `dep` of its genre's signature verb: the wyrm, countess, herald, queen, gorgon and sandman depend on their genre's `aether_axiom`, the admiral, daemon, war boss, fixer, colossus and director on techne, the judge on mythos and the sword saint on verve; they are native in their arenas, so they are Coherent unless a bleed zone overlaps the arena.

## 10. The Director

The Director (`director.profile.standard`; `director.profile.boss` in Editor Folios) owns population, waves, intensity pacing and Erasure hunters. It runs on period 30, phase 11.

### 10.1 Folio budget and population plan

Budgets are in deci-SKE (tenths of a standard-kill equivalent): minion 3, standard 10, elite 30, champion 60, boss 200, editor 400, hunter 50, matching `rank_xp_bp / 1000`. For party size 1, with `f` the Folio index (Marginalia uses the same rules with `f ≥ 26`):

```
budget(f)    = 380 for every non-Editor Folio
champions(f) = 0 if f <= 2; 1 if f <= 10; 2 if f <= 25; 3 if f >= 26
elites(f)    = 1 + floor_div(f, 8), at most 4
mini-boss    = 1 if f % 5 == 3, which also sets champions = 0 and elites = floor_div(elites, 2)
rem          = budget - 200·mini - 60·champions - 30·elites
minions      = floor_div(floor_div(rem × 30, 100), 3);   rem -= 3 × minions
standards    = floor_div(rem, 10)                          // leftover below 10 is dropped
Editor Folio (f % 5 == 0, f <= 25): 1 Editor, 1 elite and 3 standards in the antechamber; Marginalia f % 5 == 0 hosts an Editor reprise: index ((f/5 - 1) % 4) + 1,
                                    1 Proofreader, 2 Redactor, 3 Continuity Editor, 4 Abridger, same antechamber
```

Example, Folio 16: elites 3, champions 2, `rem = 380 - 120 - 90 = 170`, minions `floor_div(51, 3) = 17`, `rem = 119`, standards 11. The initial population takes `floor_div(65 × count, 100)` of the party-1 minion and standard counts plus every elite and champion; the rest is the **wave reserve**. Each pack and each wave then multiplies its per-rank counts by `enemy_count_bp[n]` (§2.3); the Director is one spawner per rank for the whole Folio, so its carry persists across packs and waves in creation order.

### 10.2 Population draws and placement

At Folio entry the Director derives `rng.spawn` for the Folio and performs, in order: the champion aspect shuffle (§7.3, 2 calls); one `weighted` template draw per champion from non-swarmer roster entries, plus its non-aspect modifier draw from Folio 11; one template draw and one modifier draw per elite; one `weighted` draw per minion slot over swarmer entries; one per standard slot over non-swarmer entries; then pack assembly and one `weighted` room draw per pack. Roster draws include only templates whose faction is unfriendly or hostile toward `fac.errata` at Folio entry, plus the six Editor-squad templates at weight 600 each.

**Packs.** `K = max(1, ceil_div(standards_initial, 3))` packs. Standards are dealt round-robin into packs 0..K, then minions, then elite `i` into pack `i % K` as leader, then champion `j` into pack `(j + elites) % K`.

**Rooms.** Each pack draws a room with weights equal to the room's remaining `spawn_slots` (doc 12), excluding the entry room and any room within 24000 mm of the entry. Members take that room's spawn cells in ascending cell index, skipping cells whose `light_block` exceeds the genre's `spawn_block_max` (Canon §12.3).

Native factions that are neutral or better toward Errata get one **civic pack** each (3 standards drawn from that faction's roster entries; none when it has no entry), placed by doc 12 at vendor and quest rooms, outside the budget and XP-eligible only if they turn hostile.

### 10.3 Intensity

Each living Erratum has `intensity_bp` in `0..=10000`:

| Trigger | Change |
|---|---|
| non-DoT damage taken from a hostile | `+ floor_div(dmg_mhp × 30000, max_hp_mhp)` |
| DoT damage taken | half of the above |
| a hostile dies within 12000 mm | `+ floor_div(400 × rank_xp_bp, 10000)`: minion 120, standard 400, elite 1200, champion 2400 |
| a control-category status applied to it | +1500 |
| `ev.player_downed` | set to 10000 |
| each Director run with no hostile at `aware.alert` toward it | −300 |
| each Director run otherwise | −50 |

All clamp to `0..=10000`. Party intensity `I` is the maximum over living Errata. Example: a 25 HP hit on an 85 HP Erratum adds `floor_div(25000 × 30000, 85000) = 8823`.

### 10.4 Phase machine and waves

| Phase | Waves | Exit condition → next |
|---|---|---|
| `director.phase.build_up` | one wave whenever 900 ticks have passed since the last wave and the reserve is not empty | `I ≥ 8000` → sustain; reserve empty and no wave alive → exhausted |
| `director.phase.sustain` | none | 300 ticks elapsed → fade |
| `director.phase.fade` | none | `I ≤ 2000` → relax |
| `director.phase.relax` | none | 1200 ticks elapsed → build_up |
| `director.phase.boss` | none; boss scripts own adds | Editor dies → exhausted |
| `director.phase.exhausted` | none | terminal; hunters continue (§10.5) |

A wave takes `min(reserve, 30 + 10 × volume)` deci-SKE from the reserve, where `volume = ceil_div(f, 5)` (Marginalia uses 5), filled as standards first and minions with the remainder, drawn as in §10.2. It spawns as one pack at a wave anchor: a doc 12 spawn anchor whose `light_block ≤ spawn_block_max`, with path distance 24000 to 48000 mm from the nearest Erratum and no line of sight from any Erratum's eye to the anchor's cell 1000 mm above the floor. One `weighted` draw from `rng.spawn` picks among qualifying anchors with weight 2 for anchors behind the party (bearing outside every Erratum's 16384 Bam frontal cone) and 1 otherwise. With no qualifying anchor the wave waits. The wave starts at `aware.investigating` toward the last-known party position. Waves pause while 224 or more actors are active, which leaves headroom under `ACTIVE_ACTOR_CAP`.

### 10.5 Erasure hunters

Each Erratum `e` has `threshold(e) = max(6, 10 - red_ink_doses(e))` (Canon §11.6, §16.4). Stack `k` arrives at `folio_entry_tick + 25200 + 1800 × (k - 1)` (Canon §17.1), so a threshold of 10 is reached 41,400 ticks (690 s) after entry, 8 at 37,800 ticks and 6 at 34,200 ticks.

- **Spawn.** On the tick the first Erratum reaches its threshold, one hunter spawns (`ev.hunter_spawned`), and another every 5400 ticks while any Erratum holds at least its threshold. Hunters skip the Director's wave headroom check.
- **Template.** `enemy.editors.revisor`, rank hunter, role brute, behavior `ai.set.hunter`, body humanoid, tags `editor`, template HP 70 and damage 12, walk 4400 and run 6200 mm/s, AR 15, resist 2500 to every type, morale-immune, skinned per genre (Inquisitor in Gothic Horror, Auditor in Cyberpunk, Revenue Agent in Hardboiled Noir). Attacks: `strike_through` (line 10000 mm, w 30, ×2.0 `dmg.slash`), `redact` (`cone` 8192 Bam 8000 mm, w 40, direct `st.silenced` and `st.marked`), `margin_lunge` (`charge`).
- **Placement.** The spawn anchor with the greatest path distance from every Erratum among anchors at least 32000 mm away in straight line; ties to the lower cell index; no draw.
- **Knowledge.** A hunter always knows the cell of every living Erratum (source `Omniscient`). Its target is the Erratum with the most `st.erasure` stacks, ties to the lower EntityId, re-evaluated each think.
- **Pathing.** Errata-placed blocks count as passable with cost `1000 + 10 × run_mm_s / 60`, because the hunter erases them (§10.6). A hunter cannot dig natural blocks; locked doors cost their break time.
- **Example HP,** solo: Folio 8 `mul_bp(mul_bp(mul_bp(70000, 26597), 120000), 10000) = 2,234,148` mHP; Folio 23 13,591,452 mHP.

### 10.6 Block erasure

Every `fac.editors` member, hunters included, erases blocks placed by Errata (Canon §11.6). Doc 03 keeps a `placed_by_errata` bit per block, set when an Erratum, a minion or an Errata gadget places a block and cleared when the block changes for any other reason.

```
on tick t where (t + phase(m)) % 10 == 0, for each fac.editors member m in ascending EntityId:
    c = m.origin + (0, 900 mm, 0)
    cand = blocks b with placed_by_errata(b) and dist2(c, center(b)) <= (1500 mm)^2
    if cand is empty: continue
    b = argmin over cand of (dist2(c, center(b)), by, bz, bx)
    set b to air with no drop; emit ev.block_changed { cause: erased, by: m }
```

A block erased by one member is gone before the next member is processed. Example: two Errata blocks have centers 1000 mm and 1414 mm from a hunter's point `c`; it erases the 1000 mm block on its next erasure tick and the 1414 mm block 10 ticks later. A third block at 1600 mm is never erased from that spot.

## 11. Character XP pacing

### 11.1 Sources and attribution

Character XP comes only from the Canon §8 sources: kills, skill-ups (at most 8 counted per character per Folio), Folio exit (4 units) and the Unseen bonus (20 units). One unit is `kill_xp_std(d)`. This chapter fixes how kills are counted:

- A **defeat** is `ev.entity_killed` of an XP-eligible entity, a spared surrender (§4.9) or a pacified pack member (§13.7). XP-eligible means not tagged `summoned`, not a mount, and either a member of a faction unfriendly or hostile toward `fac.errata` or hostile toward an Erratum at some time in the last 600 ticks. Civic packs of neutral or better factions are not XP-eligible until they turn hostile.
- A defeat pays the Canon §8 kill formula `mul_bp(mul_bp(kill_xp_std(d), rank_xp_bp), 7000)`, split by Canon §8 co-op rule, only when the killing blow or at least 2500 Bp of the victim's Max HP in damage came from `fac.errata` members (Errata, minions, hacked constructs, Errata-placed hazards) within the last 600 ticks. Entities killed purely by other factions pay 0.
- Kills by minions and hacked constructs count as their owner's (Canon §15.1).

Worked example: an elite in Folio 10 (`kill_xp_std = 252`) pays `mul_bp(mul_bp(252, 30000), 7000) = 529`; in a party of 4 each member gains `floor_div(529 × 10000, 17500) = 302`. A standard in Folio 25 pays `mul_bp(1386, 7000) = 970`.

### 11.2 Tuning simulation

Simulation model (reproducible by the bots harness, doc 02): solo, Standard mode, every planned enemy defeated; skill XP of the main Calling skill at 1.5 per enemy drained (Canon §7.1) with 20 Editor adds per Editor fight; 3 further skill levels per Folio from other skills; counted skill-ups `min(8, main + 3)`; MEM 40.

| Folio | `kill_xp_std` | Plan (champion / elite / minion / standard / boss) | Kill XP | Counted skill-ups | Exit | Folio XP | Units | Cumulative | Level after | Main skill |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 5 | 0/1/35/24/0 | 82 | 8 | 20 | 182 | 36.4 | 182 | 1 | 26 |
| 2 | 14 | 0/1/35/24/0 | 315 | 8 | 56 | 595 | 42.5 | 777 | 3 | 32 |
| 3 | 25 | 0/0/18/12/mini | 626 | 6 | 100 | 1026 | 41.0 | 1803 | 4 | 35 |
| 4 | 39 | 1/1/29/20/0 | 987 | 7 | 156 | 1689 | 43.3 | 3492 | 5 | 39 |
| 5 | 75 | 0/1/0/3/Editor | 2413 | 5 | 300 | 3463 | 46.2 | 6955 | 6 | 41 |
| 6 | 98 | 1/1/29/20/0 | 2556 | 6 | 392 | 4124 | 42.1 | 11079 | 7 | 44 |
| 7 | 123 | 1/1/29/20/0 | 3219 | 7 | 492 | 5433 | 44.2 | 16512 | 9 | 48 |
| 8 | 151 | 0/1/15/10/mini | 3946 | 5 | 604 | 6060 | 40.1 | 22572 | 10 | 50 |
| 9 | 182 | 1/2/26/18/0 | 4776 | 5 | 728 | 7324 | 40.2 | 29896 | 11 | 52 |
| 10 | 252 | 0/1/0/3/Editor | 8113 | 5 | 1008 | 11641 | 46.2 | 41537 | 12 | 54 |
| 11 | 291 | 2/2/20/14/0 | 7708 | 5 | 1164 | 11782 | 40.5 | 53319 | 13 | 56 |
| 12 | 333 | 2/2/20/14/0 | 8836 | 5 | 1332 | 13498 | 40.5 | 66817 | 15 | 58 |
| 13 | 378 | 0/1/15/10/mini | 9910 | 5 | 1512 | 15202 | 40.2 | 82019 | 16 | 60 |
| 14 | 425 | 2/2/20/14/0 | 11272 | 5 | 1700 | 17222 | 40.5 | 99241 | 17 | 62 |
| 15 | 529 | 0/1/0/3/Editor | 17032 | 4 | 2116 | 23380 | 44.2 | 122621 | 18 | 63 |
| 16 | 585 | 2/3/17/11/0 | 15171 | 5 | 2340 | 23361 | 39.9 | 145982 | 19 | 65 |
| 17 | 644 | 2/3/17/11/0 | 16709 | 4 | 2576 | 24437 | 37.9 | 170419 | 20 | 66 |
| 18 | 705 | 0/1/15/10/mini | 18485 | 5 | 2820 | 28355 | 40.2 | 198774 | 21 | 68 |
| 19 | 770 | 2/3/17/11/0 | 19985 | 4 | 3080 | 29225 | 38.0 | 227999 | 23 | 69 |
| 20 | 907 | 0/1/0/3/Editor | 29202 | 4 | 3628 | 40086 | 44.2 | 268085 | 24 | 70 |
| 21 | 980 | 2/3/17/11/0 | 25437 | 5 | 3920 | 39157 | 40.0 | 307242 | 25 | 72 |
| 22 | 1055 | 2/3/17/11/0 | 27382 | 4 | 4220 | 40042 | 38.0 | 347284 | 26 | 73 |
| 23 | 1134 | 0/1/15/10/mini | 29757 | 5 | 4536 | 45633 | 40.2 | 392917 | 27 | 75 |
| 24 | 1215 | 2/4/14/9/0 | 31616 | 4 | 4860 | 46196 | 38.0 | 439113 | 28 | 76 |
| 25 | 1386 | 0/1/0/3/Editor | 44628 | 4 | 5544 | 61260 | 44.2 | 500373 | 30 | 77 |

Results: **level 28 on arrival at Folio 25** (after Folio 24), level 12 after Folio 10, main skill 50 at Folio 8 and 75 at Folio 23 (Canon §7.2: 10 ± 2 and 22 ± 3). Sensitivity, all within 28 ± 3:

| Variant | Level on arrival at Folio 25 |
|---|---|
| 1 or 5 other skill levels per Folio | 27 / 29 |
| budget 400 instead of 380 | 29 |
| stealth run: 25% of non-boss kills, Unseen bonus in every non-boss Folio, 4 counted skill-ups | 28 |
| stealth on every second non-boss Folio | 26 |
| party of 2 / 3 / 4 with §2.3 count scaling and the Canon §8 split | 28 / 27 / 27 |

## 12. Factions

### 12.1 Faction list and starting reputation

Reputation is party-shared, per faction, `-1000..=1000`, reset each run, with the Canon §15 stance bands. `fac.wild.<genre_suffix>` (16 factions) and `fac.editors` are fixed at −1000; `fac.margin` is fixed at 1000; `fac.neutral` starts at 0 and turns hostile toward an attacker for the rest of the Folio. Native factions (`fac.native.<genre_suffix>.<group>`) start at +150 (the genre's **trader faction**, which hosts vendors and quest givers), −200 or −600:

| Genre | +150 | −200 | −600 |
|---|---|---|---|
| mythic_fantasy | kingsguard | mage_conclave | goblin_warrens |
| gothic_horror | hollow_village | inquisition | night_court |
| cosmic_horror | expedition | — | esoteric_order, deep_ones |
| space_opera | void_pirates | imperial_navy, psionic_order | hive_swarm |
| cyberpunk | netrunner_collective | zaibatsu_security | chrome_gang |
| post_apocalypse | vault_remnant | glow_cult | road_raiders |
| weird_west | railroad_company | lawmen | outlaw_gang, hex_coven |
| hardboiled_noir | dockworkers | city_police | syndicate |
| wuxia | iron_palm_sect | jade_court_guard | lotus_assassins |
| steampunk | sky_pirates | luddite_brigade | clockwork_guild |
| fairy_tale | woodcutters | seelie_court | unseelie_court |
| mythic_antiquity | temple_oracles | hoplite_league | maenad_thiasos |
| oneiric | lucid_dreamers | — | nightmare_court |
| redacted | whistleblowers | — | bureau_of_correction |
| unwritten | none (Canon §5.2) | — | — |
| ur_text | glossators | — | first_drafts |

That is 44 native factions, 2 to 4 per genre except The Unwritten.

### 12.2 Stances and the hostility matrix

Stance toward Errata drives AI: **hostile** attacks on sight; **unfriendly** barks a warning when an Erratum enters faction territory (doc 12 room tag) or stays within 4000 mm of a member for 180 ticks, and becomes hostile toward that Erratum 120 ticks later unless it leaves; **neutral** ignores Errata unless attacked; **friendly** trades and gives quests; **allied** members within 16000 mm join fights against anything attacking an Erratum.

NPC-to-NPC stances, checked in this order (first match wins):

1. Anything vs `fac.editors`: hostile.
2. `fac.wild.X` vs any native faction or `fac.neutral`: hostile; `fac.wild.X` vs `fac.wild.X`: neutral; vs `fac.wild.Y`: hostile.
3. Native factions of different genres: hostile (foreign hostility).
4. Listed pairs: allied `cosmic_horror.esoteric_order`–`deep_ones`; friendly `weird_west.hex_coven`–`outlaw_gang`, `hardboiled_noir.syndicate`–`city_police`, `space_opera.imperial_navy`–`psionic_order`; unfriendly `ur_text.first_drafts`–`glossators`.
5. Same genre, both starting below −100: hostile if their starting values differ, otherwise unfriendly.
6. Same genre, one trader faction: hostile toward the −600 factions, unfriendly toward the −200 factions.
7. Otherwise neutral.

Only hostile pairs fight. The Director keeps hostile packs of different factions at least 32000 mm apart at placement, except at bleed-zone war fronts (§12.4).

### 12.3 Reputation events

| Event | Δ to faction F | Witness |
|---|---|---|
| kill a member | −150; −40 unwitnessed | yes |
| damage a member, first time per member per 600 ticks | −50 | yes |
| kill an entity hostile to F within 24000 mm of a living F member | `+min(120, floor_div(20 × rank_xp_bp, 10000))` | yes |
| spare a surrendered member | +60 | no |
| execute a surrendered member | −200 | yes |
| complete a quest for F / fail or abandon it | +200 / −100 | no |
| theft from F (pickpocket, F-owned container) | −100 | yes |
| ignore a territory warning for 120 ticks | −50 | yes |
| gift through dialogue, once per Folio per faction | `+min(100, isqrt(item.value))` | no |
| free a captive of F | +150 | no |
| destroy an F shrine or banner block | −75 | yes |
| kill an F vendor | −500; −150 unwitnessed | yes |

**Witnessing.** F witnesses an event when, on its tick, a living sapient F member has awareness ≥ `aware.suspicious` toward the acting Erratum, or is within 24000 mm with a doc 09 sight pass to the event position. A witnessed delta is applied 300 ticks later; if every witness has died or been pacified by then, the unwitnessed value applies instead (0 where none is listed).

**Spillover.** After the main delta Δ, every other native faction G of the same genre changes by `spill(Δ, 5000)` if G is friendly or allied toward F, and by `-spill(Δ, 2500)` if G is hostile toward F, where `spill(Δ, bp) = sign(Δ) × floor_div(|Δ| × bp, 10000)`. Spillover never chains. Every change emits `ev.reputation_changed`.

Example: a witnessed kill of a `kingsguard_knight` moves `kingsguard` from +150 to 0 (neutral) and `goblin_warrens`, hostile to the kingsguard, from −600 to −563 (`spill(-150, 2500) = -37`).

### 12.4 Bleed-zone faction wars

When doc 12 places a bleed zone of genre B inside a Folio of genre A and both genres have native factions, the Director opens a **war front** at the seam: side A is A's native faction with the highest starting reputation, side B is B's native faction with the lowest. Each side spawns one pack of 4 standards and 1 elite drawn from that faction's roster entries (any roster entry of its genre with a faction override when the faction has none), already at `aware.alert` toward each other. Side B's pack is paid from the wave reserve (70 deci-SKE). Side A's pack is outside the budget when its faction is friendly or allied toward Errata.

- While a side is engaged within 24000 mm of the seam, every Errata kill of its members gives the other side +30 in addition to §12.3.
- The war ends when one side's front members are all dead, fled or surrendered. If the winner is friendly or allied toward Errata, it leaves a `loot.war_spoils` container at the seam, instanced per player (doc 11).
- Kill XP follows §11.1: front members killed by the other side pay nothing unless Errata dealt the killing blow or 2500 Bp of their Max HP.
- Foreign enemies keep their own Contradiction and Edge (§2.6), so B's soldiers that chase across the seam into A may weaken or gain Edge.

## 13. NPCs

### 13.1 NPC kinds

| Kind | ID pattern | Faction | AI |
|---|---|---|---|
| Vendor | `npc.vendor.<type>` | the genre's trader faction, or `fac.neutral` for wanderers | `ai.set.vendor` |
| Quest giver | `npc.giver.<genre_suffix>` | trader faction | `ai.set.vendor` |
| Captive | `npc.captive.<genre_suffix>` | trader faction | follows its rescuer like a Follow minion without counting toward caps; despawns at the exit |
| Hireling | `npc.hireling.<role>` | `fac.errata` once hired | `ai.set.minion` |
| Companion | `npc.companion.<name>` | `fac.errata` once recruited | `ai.set.minion` |
| Margin NPC | `npc.margin.<name>` | `fac.margin` | non-combat schedule |

Vendors and givers use the enemy schema (§2.1) with role guardian, rank elite and `sapient`, so attacking one is costly; captives use role soldier at rank minion.

### 13.2 Vendors

Vendor types: `general`, `armorer`, `gunsmith`, `apothecary`, `arcanist`, `tinker`, `fence`, `scribe` (Scriptorium services), `stablemaster` (mount tokens). Each references a doc 11 shop (`shop.<type>`) and is skinned by genre. Doc 11 applies a reputation price factor after Canon §6.4 rows 37 and 38: allied 9000, friendly 9500, neutral 10000, unfriendly 11500 on purchases; hostile factions refuse service. A vendor that takes damage turns hostile toward the attacker with its faction (§12.3) and calls allies. A killed vendor drops one item per player from its unsold stock, keyed `rng.loot` (actor = vendor EntityId, seq = slot).

### 13.3 Quest givers and quests

Each non-boss Folio has a quest giver with chance 6000 Bp, rolled with `rng.spawn` after population placement; a giver offers 2 quests drawn without replacement from the seven templates by one `weighted` call each over weights 1000. Quests last until `ev.folio_exited`.

| Template | Objective | Tier |
|---|---|---|
| `npc.quest.hunt` | kill a marked elite of a faction hostile to the giver; the Director promotes one placed standard | 2 |
| `npc.quest.rescue` | free an `npc.captive` held by a hostile pack and bring it within 4000 mm of the giver | 2 |
| `npc.quest.retrieve` | bring back a `cat.quest` item placed in a room at least 48000 mm away | 1 |
| `npc.quest.deliver` | carry a `cat.quest` item to a named NPC of another non-hostile faction | 1 |
| `npc.quest.sabotage` | destroy 3 marked blocks in a hostile faction's territory | 2 |
| `npc.quest.parley` | pass a dialogue check (§13.7) with the leader of a −200 faction | 1 |
| `npc.quest.cleanse` | kill every hostile in one marked room | 3 |

Rewards: `mul_bp(reward_base[tier], P(d))` Lucre with bases 80, 120, 160; +200 reputation with the giver's faction (§12.3); one item from `loot.quest.t<tier>` per player (doc 11). Quests grant no character XP of their own; the Canon §8 sources apply. Example: a tier-2 quest in Folio 12 pays `mul_bp(120, 25804) = 309` Lucre.

### 13.4 Hirelings

Hireling brokers appear in trader-faction rooms (doc 12). Hire roles: soldier, guardian, caster, support, sniper, skirmisher, each using the role template of §3 and the minion stat rule of §6, skinned in the local genre. Hire cost `mul_bp(60, P(d))` Lucre; upkeep `mul_bp(15, P(d))` Lucre per Erratum-owner on each `ev.folio_exited`, paid automatically; an unpaid hireling leaves (`ev.minion_released`). Folio 12: hire 154, upkeep 38. A hireling accepts one weapon from its owner (the §5.3 rule, without the pickup action) and returns it on release.

### 13.5 Companions and approval

Companions are named hirelings with dialogue, a personal quest and approval. One companion per core genre appears as an encounter with chance 2500 Bp in a Folio of its genre (rolled with `rng.spawn`) while not yet recruited this run. Recruiting costs nothing; a companion counts as a minion and has no upkeep.

| ID | Genre | Role | Likes | Dislikes | Loyalty perk (approval ≥ 60) |
|---|---|---|---|---|---|
| `npc.companion.sir_brannoc` | mythic_fantasy | guardian | valor, order | cunning | its `shield_wall` also protects its owner within 3000 mm |
| `npc.companion.mother_vesper` | gothic_horror | support | zeal, mercy | greed | `mend` also targets its owner, cooldown 300 |
| `npc.companion.dr_halloway` | cosmic_horror | controller | curiosity, cunning | zeal | owner composure loss × 8000 within 8000 mm |
| `npc.companion.ensign_okafor` | space_opera | soldier | order, valor | cunning | `call_allies` gives the party direct `st.focused` |
| `npc.companion.glitch` | cyberpunk | controller | cunning, greed | order | owner's hacking buildup on constructs × 12500 within 12000 mm |
| `npc.companion.rust_mae` | post_apocalypse | sapper | greed, kinship | mercy | breaking a container yields one extra `cat.material` |
| `npc.companion.deputy_cole` | weird_west | sniper | order, mercy | greed | `high_noon` on any target its owner damaged |
| `npc.companion.iris_kane` | hardboiled_noir | stalker | cunning, curiosity | zeal | marks the nearest hidden hostile with `st.marked` every 600 ticks |
| `npc.companion.lan_feiyu` | wuxia | skirmisher | valor, mercy | greed | `qinggong_leap` carries its owner when both stand within 1500 mm |
| `npc.companion.gearwright` | steampunk | artillery | curiosity, greed | zeal | owner gadgets recharge × 11500 |
| `npc.companion.bramble` | fairy_tale | summoner | kinship, curiosity | order | summons gain +2000 Bp Max HP |
| `npc.companion.kallias` | mythic_antiquity | guardian | valor, zeal | cunning | `phalanx` AR bonus applies to its owner within 2000 mm |

Approval is `-100..=100`, starting at 20. Each event tagged with a value the companion likes adds +4 (a boss or Editor kill adds +10 to `valor`); each event tagged with a disliked value adds −4. Net change per companion per Folio is clamped to ±20.

| Value | Events |
|---|---|
| mercy | spare a surrender (+); execute a surrender (−) |
| greed | sell items, spin a Fortune Engine, open a Reliquary (+) |
| zeal | pray to a patron, kill an `undead` or `fiend` (+) |
| cunning | earn the Unseen bonus, kill with a sneak attack (+) |
| valor | kill an elite, champion, mini-boss or Editor (+) |
| order | complete a quest (+); witnessed theft (−) |
| curiosity | identify an item, enter a secret room (+) |
| kinship | complete a quest for the companion's genre's trader faction (+); kill a native of its genre (−) |

Thresholds: at ≥ 60 the loyalty perk is active; at 100 its personal quest is offered once (reward `loot.companion.<name>`, doc 11); at `ev.folio_exited` with approval ≤ −50 it leaves; on reaching −100 it turns hostile at once as an elite of `fac.editors` with its role template.

### 13.6 Margin NPCs

All are `fac.margin`, invulnerable and non-combat, walking a fixed 3-station schedule in the hub.

| ID | Service | Owner of the service |
|---|---|---|
| `npc.margin.archivist` | bestiary and item codex | 14 |
| `npc.margin.binder` | Quire unlock tree | 14 |
| `npc.margin.recollector` | Echo Recollection trials | 14 |
| `npc.margin.cartographer` | run history, seeds, daily and weekly runs | 14 |
| `npc.margin.inkwell_keeper` | Inkwell | 15 |
| `npc.margin.codex_scribe` | Codex pass | 15 |
| `npc.margin.quartermaster` | store of deterministic goods | 15 |
| `npc.margin.drillmaster` | training yard: spawns any template from the account's bestiary at Folio-1 stats and a chosen rank; no XP, loot or skill XP | 13 |

### 13.7 Dialogue system

A dialogue is one record `npc.dlg.<name>` with local node keys. Text uses text keys `loc.npc.dlg.<name>.<node>` (doc 18).

```json
{
  "id": "npc.dlg.kingsguard_sergeant",
  "start": "greet",
  "nodes": {
    "greet": { "text": "loc.npc.dlg.kingsguard_sergeant.greet", "choices": [
      { "text": "loc.npc.dlg.kingsguard_sergeant.c_pass",
        "check": { "skill": "skill.speech", "sub": "sub.charm", "dc_base": 30 },
        "effects": [ { "op": "pacify_pack" } ], "next": "end", "fail_next": "refuse" },
      { "text": "loc.npc.dlg.kingsguard_sergeant.c_bribe",
        "cond": [ { "op": "lucre_at_least", "base": 50 } ],
        "effects": [ { "op": "take_lucre", "base": 50 }, { "op": "rep", "fac": "fac.native.mythic_fantasy.kingsguard", "delta": 30 } ],
        "next": "end" } ] },
    "refuse": { "text": "loc.npc.dlg.kingsguard_sergeant.refuse", "choices": [], "effects": [ { "op": "turn_hostile" } ] }
  }
}
```

- **Conditions:** `rep_at_least`, `rep_below`, `has_item`, `lucre_at_least` (base × `P(d)`), `skill_at_least`, `origin_is`, `kin_is`, `companion_present`, `flag_set`, `axiom_at_least` (local), `quest_state`, `not_in_combat`.
- **Checks are deterministic:** `value = skill + floor_div(sub_eff - 40, 2) + floor_div(rep(F), 100)`, `dc = dc_base + floor_div(d, 2)`, pass when `value >= dc`. The UI shows `value` and `dc` before the choice. Pairs: persuade (`skill.speech`, `sub.charm`), deceive (`skill.speech`, `sub.guile`), intimidate (`skill.leadership`, `sub.menace`). A pass grants non-combat skill XP with magnitude `dc` (doc 04). Example: Speech 35, CHM 52, reputation +150 gives `35 + 6 + 1 = 42` against `30 + 6 = 36` in Folio 12: pass.
- **Effects:** `rep`, `give_item`, `take_item`, `give_lucre`, `take_lucre` (base × `P(d)`), `start_quest`, `complete_quest`, `hire`, `approval`, `set_flag` (Folio-scoped; `run.` prefix for run-scoped), `open_shop`, `pacify_pack` (the speaker's squad becomes `fac.neutral` and walks out; each XP-eligible member is a defeat, §11.1), `turn_hostile`, `teach_recipe` (doc 07), `end`.
- **Parley.** An Erratum may open the leader's dialogue in a hostile sapient pack when no member is in `Windup` or `Active` and the party has not damaged the pack for 300 ticks. The pack holds position (`circle`) for up to 600 ticks while the dialogue is open; the game does not pause.
- **Barks** are presentation only: client-side text keys `loc.npc.bark.<set>.<n>`, optionally rewritten by the Scribe model and cached by seed (brief §5). They never touch simulation state.

## Exports

All IDs below are in namespaces Canon §19 assigns to this chapter (`enemy.`, `ai.`, `fac.`, `boss.`, `npc.`, `director.`, `rank.`, `elite.`).

| Namespace | IDs defined |
|---|---|
| `enemy.role.*` | soldier, swarmer, brute, skirmisher, shooter, sniper, caster, controller, support, guardian, summoner, stalker, artillery, sapper, flier, cavalry (16), plus `enemy.role.boss` |
| `enemy.<genre_suffix>.*` | the 103 genre roster entries of §8.1 to §8.14 and §8.16 |
| `enemy.editors.*` | inkling, strikethrough, marginal_scribe, footnote, erratum_eater, red_pen_lancer, revisor |
| `enemy.ab.*` | every attack and ability named in §2.1, §3, §8 and §10.5 |
| `ai.curve.*` | linear_up, linear_down, soft_up, quad_up, quad_down, cubic_up, binary, binary_not, gate_half, gate_reach, logistic_up, logistic_down, bell_mid, plateau |
| `ai.input.*` | the 25 inputs of §4.3 |
| `ai.action.*` | the 30 actions of §4.7 |
| `ai.set.*` | the 20 behavior sets of §4.7 |
| `ai.nav.*` | small, humanoid, large, mounted, burrower, flier, swimmer |
| `fac.native.*` | the 44 native factions of §12.1 |
| `fac.wild.*` | one per genre, 16 |
| `boss.editor.*` | proofreader, redactor, continuity, abridger, last_word; their attacks as `boss.editor.<name>.<attack>` |
| `boss.<genre_suffix>.*` | the 14 mini-bosses of §9.3; their attacks as `boss.<genre_suffix>.<name>.<attack>` |
| `npc.vendor.*` | general, armorer, gunsmith, apothecary, arcanist, tinker, fence, scribe, stablemaster |
| `npc.giver.*`, `npc.captive.*` | one per genre with native factions |
| `npc.hireling.*` | soldier, guardian, caster, support, sniper, skirmisher |
| `npc.companion.*` | the 12 companions of §13.5 |
| `npc.margin.*` | archivist, binder, recollector, cartographer, inkwell_keeper, codex_scribe, quartermaster, drillmaster |
| `npc.quest.*` | hunt, rescue, retrieve, deliver, sabotage, parley, cleanse |
| `npc.dlg.*` | dialogue records (§13.7) |
| `director.profile.*` | standard, boss |
| `director.phase.*` | build_up, sustain, fade, relax, boss, exhausted |
| `elite.*` | the 17 non-aspect modifiers of §7.2 (the 3 aspects are canon) |
| `rank.*` | none new |

Constants: `MOMENTUM_BP = 12500`, `BAND_BP = 9000`, budget 380 deci-SKE, wave interval 900 ticks, hunter interval 5400 ticks, erasure radius 1500 mm and period 10 ticks (canon), token pools 3 / 4 / 1 / 1, `MINION_HP_BP = 15000`.

## Interface assumptions

| Doc | Provides |
|---|---|
| 02 | the 1.5 ms host AI budget row; the bots harness that reproduces §11.2; `tools/gen_curves.py` |
| 03 | tick order of §4.1; a `placed_by_errata` bit per block; `ev.block_changed` with `cause: erased`; a view-frame channel for telegraphs and summons; dynamic light override volumes (Redactor `blackout`) |
| 04 | non-combat skill XP for dialogue checks with magnitude `dc`; holding breath |
| 05 | `counter.*` sources; weapon archetype profiles used by §5.3; `arch.mount.*` accepting NPC riders and `mount_hp_mhp`; spread from a half-angle; homing, knockback and grapple mechanics |
| 06 | `summon.*` templates and magnitudes; NPC damage multipliers for `spell.*`; fizzle rules |
| 07 | `break_ticks(material)`; door blocks with HP; ladder, vine and rope blocks; `mat.cold_iron` |
| 08 | potion heal amounts; gas emission in ml for steam, miasma and smoke |
| 09 | awareness states `aware.unaware`, `aware.returning`, `aware.suspicious`, `aware.investigating`, `aware.searching`, `aware.alert`; sight-pass queries; callouts; hacking buildup; senses `breath`, `tremor`, `thermal` |
| 10 | monster-use identification to `know.classified` |
| 11 | `loot.drop.{armed, beast, caster, construct, undead, eldritch, swarm, editor, mount}`, `loot.boss.{mini, editor, palimpsestic}`, `loot.quest.t1..t3`, `loot.war_spoils`, `loot.companion.<name>`; `shop.<type>`; the reputation price factor; common item lists per category and genre |
| 12 | GPV `spawn_block_max`, `damage_affinity`, roster and faction fields copied from this chapter; rooms with `spawn_slots`, territory tags, `room.no_mount`; spawn anchors; patrol routes; boss arenas to §9; static and dynamic bleed zones; gravity-flip volumes |
| 14 | difficulty multiplier (step 2.13) and token additions; Marginalia acceptance of Editor reprises; Abridged-mode scaling |
| 16 | host-only AI; telegraphs in snapshots |
| 17 | minion command inputs; telegraph and shimmer rendering; dialogue UI showing `value` and `dc` |
| 18 | text keys `loc.npc.dlg.*`, `loc.npc.bark.*`; enemy, boss and hunter skins |

## Canon gaps

1. **§5.9 is missing.** Canon §12.3 and §13.9 cite §5.9 (`spawn_block_max`, manufacturers), but canon-1.2 has no §5.9. Assumed: doc 12's GPV supplies `spawn_block_max` per genre, 0 by default and 7 for Gothic Horror and Cosmic Horror (R2 §8.1 item 16).
2. **Defeats without death.** Canon §8 pays XP only for kills and has no event for a spared or pacified enemy. Assumed: §11.1 defeats pay the kill formula at the sparing interact or at `pacify_pack`; doc 03 adds an event `ev.entity_pacified`, and until it exists the enemy emits `ev.entity_despawned` when it leaves.
3. **Kill attribution.** Canon §8 does not say who earns XP for enemies killed by other factions or the environment. §11.1 requires 2500 Bp of Max HP or the killing blow from `fac.errata`.
4. **Summoned enemies** pay 0 kill XP (Canon §8 is silent); without this, summoners are infinite XP sources.
5. **Champion wording.** Canon §17.3 says a champion has "one champion counter type (doc 05)" while §17.3a defines the aspects as `elite.*`. Interpreted: one aspect from §17.3a, countered by its matching counter type.
6. **Reference kills.** `REF_KILLS_PER_FOLIO = 40` is read as about 40 XP units per Folio; the spawn budget is 38 standard-kill equivalents, which yields 38 to 46 units per Folio with exit and skill-up XP (§11.2).
7. **Co-op skill pace.** With Canon §7.1 normalization by the target's Max HP and the §17.2 HP and count tables, a member of a 4-player party gains about 4375 Bp of the solo combat skill XP (main skill 50 instead of 77 at Folio 25 in the §11.2 model). Character levels stay in band (27). Recommended canon change: normalize skill XP by `floor_div(target_max_mhp × 10000, party_hp_bp)`.
8. **Schedule.** Canon §2.2 has no rows for the squad coordinator and the Director; this chapter uses period 30 with phases `squad_id % 30` and 11.
9. **Marginalia bosses.** Canon §17.1 lists boss Folios only to 25; §10.1 adds Editor reprises at every Marginalia Folio divisible by 5.
10. **The Unwritten** has no native factions (Canon §5.2) although Canon §15 says 2 to 4 per genre; Canon §5.2 wins.
11. **NPC block stability.** Canon §6.5 excludes row 19 for NPCs; templates carry `block_bp`.
12. **Enemy mounts.** Canon §17.7 describes Errata riders; enemy riders reuse `st.mounted` and the §17.7 charge rule.
13. **Last Word.** `genre_shift` makes a dynamic bleed zone inside `genre.ur_text`; because Canon §5.5's override is keyed to the local genre, Contradiction applies inside the zone. This is intended and needs doc 12's dynamic bleed-zone hook.
14. **Quests** have no canon namespace; this chapter uses `npc.quest.*`.

## Acceptance tests

1. **Scaling.** Standard navy trooper, Folio 10, party 2: `hp_max = 330,460` mHP; a ×1 hit `B = 16,190` and 17,809 after step 2.14; AR 21; poise 46,000.
2. **Elite scaling.** Elite brute, Folio 25, party 4: `hp_max = 17,042,142`; `B = 84,579`, 109,952 after step 2.14; AR 118.
3. **Counts.** Base 5, party 4: four activations spawn 8, 9, 9, 9.
4. **Sub-attributes.** Template 40 at Folio 25 is 76; NPC move multiplier at AGI 76 is 10900.
5. **Curves.** `logistic_up(5000) = 5000`, `logistic_up(3500) = 1868`, `bell_mid(5833) = 8859`, `plateau(10000) = 0`, `plateau(5000) = 10000`.
6. **Compensation.** `adj(5000)` is 6666 with `n = 3` and 5000 with `n = 1`.
7. **Utility example.** The §4.5 brute scores melee 0, advance 2592, charge 10450, retreat 151; only charge is eligible (threshold 9405).
8. **Think draws.** A think that selects an action calls `range` exactly once on keyed `rng.ai` (seq 0); a dormant think, or one whose agent is in `Windup`, `Active`, `Recovery` or `Channel`, draws nothing; the choice is identical whether 0 or 100 other agents thought earlier that tick.
9. **Telegraph validation.** The content validator rejects an attack with `dmg_mult_bp` 20000 and `windup` 29 and accepts `windup` 30; it rejects 40000 with 44.
10. **Reaction delay.** A standard agent alerted on tick 1000 starts no attack windup before tick 1018; an elite none before 1012.
11. **Tokens.** Five standard melee agents against one Erratum: at most 3 are ever in `Windup` or `Active` at once; an elite added to the squad attacks regardless.
12. **Cover.** A one-block wall between threat and cell gives half cover (6000); a two-block wall gives full (10000).
13. **Path cost.** Three walks, one diagonal and one jump-up cost 6814. Two equal-cost paths resolve to the one through the lower cell index on every run.
14. **Digging.** `break_ticks` 120 with a sapper gives `dig_ticks = 60`.
15. **Hacking.** A hacked `security_droid` is in `fac.errata`; a 10,000 mHP rifle hit on it from an Erratum leaves `D2 = 5000`; on expiry it returns to `imperial_navy` and gains 5000 threat toward the hacker. A hack on a boss is rejected.
16. **Minion cap.** Conjuration 60, Handling 10: cap 3. A fourth bind fails and emits no `ev.minion_bound`.
17. **Minion stats.** A tamed troll in Folio 12 has 297,330 mHP, hits for 46,447 mHP and has AR 27; taming a standard beast in Folio 12 needs Handling 49, and Handling 48 fails with no draw.
18. **Weapon pickup.** A Folio-10 soldier with an Epic Good rifle (interval 30): `pickup_bp = 12650`, 6,825 mHP per shot.
19. **Elite selection.** A brute promoted in Folio 3 has 9 eligible non-aspect modifiers; in Folio 6, 14; in Folio 11, 17. Each promotion makes exactly one `weighted` call.
20. **Aspect sets.** Run seed `0x0123456789ABCDEF`: Folio 3 → `[elite.warded]`; Folio 11 → `[elite.surging, elite.warded]`; Folio 21 → `[elite.unstoppable, elite.warded, elite.surging]`. The exit preview of Folio 2 shows Folio 3's set. Each shuffle calls `range` twice.
21. **Population plan.** Folio 1: 1 elite, 35 minions, 24 standards. Folio 16: 2 champions, 3 elites, 17 minions, 11 standards. Folio 23: 1 mini-boss, 1 elite, 15 minions, 10 standards. Editor Folios: Editor, 1 elite, 3 standards.
22. **Intensity.** A 25,000 mHP hit on an 85,000 mHP Erratum adds 8823; a downed Erratum sets party intensity to 10000.
23. **Hunters.** Threshold 10: first hunter on `folio_entry_tick + 41,400`; with 2 red-ink doses on one Erratum, `+ 37,800`; then one every 5,400 ticks. Hunter HP solo at Folio 8: 2,234,148. No hunter spawns in an Editor Folio while the Editor lives.
24. **Erasure of blocks.** Errata blocks at 1000, 1414 and 1600 mm from a hunter's point `c`: the first is erased on its next erasure tick, the second 10 ticks later, the third never; natural blocks are never erased.
25. **Kill XP.** Elite at Folio 10 pays 529; each member of a 4-party gets 302. Standard at Folio 25 pays 970. A summoned inkling pays 0.
26. **Attribution.** An enemy killed by another faction after Errata dealt 2000 Bp of its Max HP pays 0; after 2500 Bp it pays full kill XP.
27. **Level pacing.** The §11.2 model gives level 12 after Folio 10, level 28 on arrival at Folio 25, main skill 50 at Folio 8 and 75 at Folio 23.
28. **Boss HP.** Proofreader solo 7,113,600 mHP; Last Word with 4 players 359,677,296 mHP; a mini-boss in Folio 3 solo 2,148,000.
29. **Palimpsestic roll.** A Volume-2 Editor's first loot draw per player is `roll_bp(4375)` on keyed `rng.loot` with seq = slot.
30. **Phase shift.** A hit taking a Proofreader from 6700 to 6500 Bp stops at 6600 Bp; for 90 ticks the boss takes 0 damage.
31. **Reputation.** A witnessed `kingsguard_knight` kill: `kingsguard` 150 → 0 and `goblin_warrens` −600 → −563 after 300 ticks. If both witnesses die within 300 ticks: `kingsguard` 150 → 110 and `goblin_warrens` −600 → −590.
32. **Dialogue.** Speech 35, CHM 52, reputation +150, `dc_base` 30, Folio 12: value 42, DC 36, pass, with no random draw.
33. **Approval.** Six `valor` events in one Folio for Sir Brannoc add +20, not +24; at −50 on `ev.folio_exited` he leaves with `ev.minion_released`.
34. **Economy.** Folio 12: hireling 154 Lucre, upkeep 38; tier-2 quest reward 309.
35. **Morale.** A standard sapient soldier at HP 2000 Bp whose low-HP event already brought morale to 8000 sees its squad leader die (5000), then an elite ally die (2500, no surrender because the rule is below 2500), then one more ally die (1000): it surrenders.
36. **NPC Contradiction.** Navy trooper in a Fairy Tale bleed zone: C 5, potency 5000, jam 1500 Bp, E 6.
37. **Editors.** Every `fac.editors` attack has empty `dep`; in every genre its C is 0 and E is 0.
38. **Mounts.** Killing a cavalry mount applies `st.knocked_down` to the rider; killing the rider pays the rider's kill XP and the mount pays 0.
39. **Reference duel.** A Folio-1 soldier against an 85 HP, 20 AR Erratum lands 6,667 mHP per hit and kills on its 13th hit, tick 1,098.
40. **Determinism.** Two hosts running the same seed and input log for 36,000 ticks of a Folio with 120 agents produce identical blackboards, paths and `state_hash`.

## Invariants annex

```
Σ doc13: enemies ⊗ ai ⊗ squads ⊗ nav ⊗ director ⊗ factions ⊗ npcs
W! enemy power := f(d, rank, party)                         N! enemy stat reads any Erratum level|skill|gear
W! hp = tpl × H(d) × rank × party_hp; B = tpl × T(d) × rank; AR = tpl × P(d)
W! minion hp = tpl × T(d) × 15000; minion dmg = tpl × P(d)   N! rank|party tables on minions
W! ai ⊂ host                                                 W! iteration := ascending EntityId
W! selecting think := keyed rng.ai(seq 0), exactly 1 range  N! draws in dormant|committed think
W! population|waves := rng.spawn; idle := rng.ai_idle         N! HashMap|HashSet ∈ blackboard
W! score := Π adj(c_i) × weight; select ∈ {s ≥ 0.9·best}      N! float ∈ curves|scores
W! dmg_mult ≥ 20000 ⇒ windup ≥ 30; ≥ 40000 ⇒ windup ≥ 45     N! untelegraphed big hit
W! tokens: melee 3, ranged 4, control 1, grenade 1 per target N! token for elite|champion|boss|editor|hunter
W! A* tie := (f, h, cell index)                               N! path expansions > 8192 per tick
W! dig ∌ void_bedrock | doors | sealed arena blocks
W! hunter knows all Errata; others perceive via doc 09       N! omniscience elsewhere
W! erase := placed_by_errata ∧ dist ≤ 1500 mm, 1 block / 10 ticks / member
W! fac.editors attack dep = ∅                                 N! Contradiction|Edge on Editors
W! champion aspect ∈ §17.3a ∧ counter ∈ every genre (dep none)
W! aspect set := first 2 draws of rng.spawn(f)                W! preview(f) == Folio(f) aspects
W! summoned ⇒ kill XP 0                                       W! kill XP needs Errata share ≥ 2500 Bp or killing blow
W! XP sources == Canon §8                                     W! arrival level at Folio 25 ∈ 25..=31
W! reputation ∈ -1000..=1000, party-shared, per run           N! spillover chains
W! dialogue checks deterministic                              N! LLM text → sim state
W! st.hacked ⇒ fac.errata ∧ ¬minion cap                       N! hacked boss|editor|hunter
W! boss phase shift = 90 ticks invulnerable                   W! Erasure paused while Editor lives
N! Palimpsestic drop outside Editor | Volume reward | secret room
```
