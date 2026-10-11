# 13 — Enemies, AI, Factions and NPCs

As of 2026-10-11. Status: BINDING. Conforms to canon-1.4.

This chapter defines every hostile and non-player actor: enemy templates and scaling, roles, the AI, rosters, bosses, the Director, XP pacing, factions and NPCs. All AI runs on the host inside `palimpsest_sim` and its helper crate `palimpsest_ai` (doc 02 §3.2).

## Imports

| Source | Consumed |
|---|---|
| Brief §1, §2, §4, §5 | Errata and Editors, archetype × skin, axioms, run shape, Folio length, deterministic fixed-point sim |
| Canon §1, §2.2, §2.3 | ID grammar, integer and `Fx` primitives, `Bam`, EntityId order and phase key, AI think period 6, durations, `in_combat`, `light_at` |
| Canon §3 | `rng.spawn`, `rng.ai_idle` (sequential); `rng.ai`, `rng.combat`, `rng.loot`, `rng.magic`, `rng.status` (keyed); draw-count rule. This chapter uses no stream outside Canon §3.2 and §3.3. |
| Canon §4, §4.3, §5.2 to §5.8 | `ACTIVE_ACTOR_CAP`, movement, jump, fall damage; fixed terrain, the `destructible` tag and `cat.placeable` Errata placeables; genres, dependencies, NPC tolerance, Contradiction, Edge, MYTHOS ambient, local genre |
| Canon §6.4, §6.5, §7 | NPC derived-stat rows; NPC stamina and aether as doc 13 constants; skill XP normalization and pace contract |
| Canon §8 | XP curve, `kill_xp_std`, XP sources, co-op split, the 40-unit target |
| Canon §9 to §12 | body plans, severing, damage types, pipeline, statuses, buildup, direct durations, Erasure §11.6, §11.8 player semantics, surfaces, spawn light |
| Canon §13.8, §15, §16.3, §17 | `P`, `T`, `H`, `S`, `enemy_level`; factions and minions; kin; run constants, Abridged kill-XP factor 16667, party tables, ranks, champion aspects, anchors, friendly fire, ghosts, mounts, invasions (§17.9) |
| Canon §18, §19 | event IDs; namespaces `enemy.`, `ai.`, `fac.`, `boss.`, `npc.`, `director.`, `rank.`, `elite.` |
| Canon-1.3 and canon-1.4 §4.3, §5.9 to §5.11, §17.1, §17.9 | `spawn_block_max` and enemy themes; World Laws and `law_bound`; Going Native and Unmoored; the Colophon and desecration; limited building; opt-in invasions by a Redactor with an account Editor kit |
| Doc 02 §2.3, §3.4 | in-house A* in `palimpsest_ai` with lower-cell-index tie-break; `score_actions`, `astar` |
| Doc 03 §2.2, §3.4, §15 | system order (`sys.spawn_admit` 3, `sys.ai_think` 6, `sys.actions` 7, `sys.erasure` 32), `comp.control`, `comp.tags`, `BodyState`, sleeping entities (§15.2), `request_spawn` and `spawn_q` (§15.3), `set_block` |
| Doc 04 §4.2, §5.2, §5.3, §6 | dialogue (Persuade, Deceive, Intimidate) and taming resolved deterministically by this chapter, with doc 04's `value` formula and verb bonuses (`courtesy`, `silver_tongue`, `beast_tongue`, Speech 50 bribe, Speech 100, Handling 75); dialogue and Tame skill XP; Handling and Leadership minion perks; XP sources with pacify (`ev.entity_pacified`); the §6.3 pacing model that replays §11.2 |
| Doc 05 §12, §14.1, §15.1, §15.2, §17.3, §19.3 | guard and poise; threat and target hysteresis; counter bits including `counter.sunder` and `counter.anchor`, and the requested `elite.armored` and `elite.phasing`; mounted combat; NPC grabs through `st.grappled` |
| Doc 06 §6.5, §12.1, §12.2, §18 | summon stats, cap replacement and AI hooks; hostile `summon.herald` and `summon.misprint`; `NPC_SPELL_SCALE_BP = 4500` and the scorer queries; `reckless` and interrupter profiles; `st.feared` flight |
| Doc 11 §9.1 to §9.8 | kill drops by `loot.drop.<rank>`, `loot_role` category bias, `no_loot`, `enemy.mimic`, `loot.chest.plain`, `loot.chest.locked`, `loot.secret.cache`, bags, Volume rewards |
| Doc 12 §3.5, §4, §5.7, §6.1 to §6.3, §8.2, §12, §14, §16, §19, §21, §22 | GPV `density_bp`; World Laws that call this chapter (`corporate_response`, `honor_duel`, `hubris` Nemesis, complication ambushes); the exit preview that recomputes this chapter's aspect set; Folio types and challenge kinds; the `room.rule.arena` and `room.rule.rescue` rules; the population interface (`spawn_slots`, spawn cells, spawn anchors, patrol routes, territory, civic anchors); bleed and dynamic zones; the room catalog; narrative events; Editor and mini-boss arenas; Marginalia modifiers |
| Doc 14 §3, §4.2, §7, §8.2a, §9, §11 | positions, Keepers, Rubrics, Editor IDs, Erasure clock, Colophon hunters, Grade parameters and difficulty chains, telegraphed hits, Abridged structure, Marginalia rotation, the Prologue's `enemy.prologue_hunter` |
| Doc 09 §2.4, §4.1 to §4.6, §6.1 to §6.7, §9.4, §11, §13.5, §13.7, §14.5 | awareness states `aware.unaware`, `aware.curious`, `aware.suspicious`, `aware.searching`, `aware.lost`, `aware.alert`; the search plan; callouts, radio and `raise_alarm`; tremor, breath, thermal and scent senses; perception profiles; disguises and outfits; subdue; hacking |
| Doc 15 §3.1.2 | Script-granted hireling types |
| Doc 16 §9.13 | invasion window, `InvaderJoin { kit }`, invader slot 4, PvP culling |
| Decision ledger 2026-10-11 items 15, 16 | opt-in PvP invasions as Editors; limited building and fixed terrain |
| R1 §2.12, §5.9; R2 §2.4, §5.3; R4 §9; R5 §2.6, §9.2; R6 §9, §10 | SPD investigate state, Qud reputation; Oblivion scaling anti-pattern, Minecraft spawn light; utility AI and HPA*; awareness machine, darkness in gunfights; genre rosters and factions |

## 1. Design rules

1. **Depth scaling only.** Enemy power scales with Folio depth `d` through `H`, `T`, `P` and `S` (Canon §13.8) and with party size (Canon §17.2). It never reads any Erratum's level, skills or gear. This avoids Oblivion's leveling wall (R2 §2.4).
2. **Roles are archetypes, roster entries are skins.** Each enemy template names one of the 16 roles in §3. The role carries the stat template and behavior set. The roster entry carries the skin, body plan, kin, tags, faction, signature ability and its axiom dependencies.
3. **Fair telegraphs.** Every enemy attack whose template damage multiplier is at least 20000 Bp has a windup of at least 30 ticks and a published telegraph. At 40000 Bp or more the windup is at least 45 ticks. Boss attacks list their exact windups in §9.
4. **No hidden cheating.** AI perceives only through doc 09 awareness, except Erasure hunters and the Prologue hunter (§10.5, §8.17), which always know the position of every living Erratum, and bosses inside their sealed arena, which always know every Erratum in the arena.
5. **Determinism.** All AI runs on the host. Every state-mutating loop runs in ascending `EntityId`. Bulk population draws use `rng.spawn`. Idle and wander draws use `rng.ai_idle`. Choices made during an agent's think use keyed `rng.ai`. Every draw count is stated.
6. **Fixed terrain.** Enemies break only doors, blocks tagged `destructible` (Canon §4.3, docs 03 and 07) and Errata placeables (`cat.placeable` blocks, which are always destructible). Worldgen terrain without the tag is never dug, erased or exploded by any enemy (brief §1, decision ledger item 16).

## 2. Enemy entity model

### 2.1 Enemy template schema

An enemy template is one JSON record in `content/13/enemy/`. Every enemy, mini-boss, Editor, hunter, summon template of doc 13, vendor and hireling uses this schema.

| Field | Type | Meaning |
|---|---|---|
| `id` | content ID | `enemy.<genre_suffix>.<name>`, `enemy.editors.<name>`, `boss.…` for bosses, `npc.…` for NPCs |
| `role` | content ID | one of the 16 `enemy.role.*` (§3); bosses use `enemy.role.boss` |
| `origin_genre` | content ID | drives NPC tolerance and component requirements (Canon §5.3, §5.4) |
| `rank` | content ID | default rank; the Director promotes standard templates to elite or champion (§7, §10) |
| `body` | content ID | Canon §9 body plan |
| `body_state` | object | doc 03 §3.4 `BodyState` in content units: `radius_mm`, `stand_mm`, `eye_mm`, `mass_g`, `kb_resist_bp`, `hitbox_scale_bp`, `flags` (PUSHABLE, COLLIDES_BODIES, FLIER, SWIMMER, CAN_MANTLE, CAN_CLIMB, EDGE_GUARD, NO_GRAVITY) |
| `kin` | content ID or null | Canon §16.3 kin (§2.4); allowed only with `body.humanoid`, because every Canon §16.3 kin is humanoid; sapience is the separate `sapient` tag |
| `mutation` | enum or null | `kin.mutant` only, required there: `regrowth`, `extra_arm`, `hide` or `gills` (§2.4); null for every other kin |
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
| `stamina_m` | i64 | NPC stamina maximum (Canon §6.5); regenerates 20,000 per second after 60 ticks without spending |
| `aether_max_mae`, `aether_regen_mae_s`, `spells` | i64, i64, list | NPC aether and authored `spell.*` IDs (doc 06 §18); regeneration default 2,000 per second |
| `block_stability_bp`, `guard_arc_bam` | Bp, Bam | NPC block stability (Canon §6.5 excludes row 19) and guard half-arc (doc 05 §12); 0 for non-blockers |
| `senses` | list of enum | `sight`, `hearing`, `breath`, `tremor`, `thermal`, `scent` (§2.4; doc 09 `senses` bits 0 to 5 in this order) |
| `perception` | content ID or null | a doc 09 `aware.profile.*` override; null uses doc 09's first-match profile rules |
| `attacks` | list of attack profiles | basic attack first, then abilities |
| `behavior` | content ID | `ai.set.*` (§4.7) |
| `loot_role` | enum | doc 11 §9.3 role bias: `brute`, `skirmisher`, `marksman`, `caster`, `support`, `swarm`, `beast`, `construct` |
| `weight` | u32 | roster weight for the Director (§10.1) |
| `flags` | object | `sapient`, `can_surrender`, `uses_items`, `weapon_pickup`, `morale_immune`, `poise_immune`, `no_loot` (doc 11 §9.1), `keeper` (doc 14 §3.4), `summoned`, `outfit` (a stripped corpse or subdued body yields a doc 09 §11.1 outfit of the template's faction), `reckless` (casts harmful area spells even when doc 06 `ally_risk > 0`), `interrupter` (prefers casters per doc 06 §18) |
| `outfit_mass_g` | Grams | mass of the outfit item; 0 when `outfit` is 0 |
| `carry` | list | spawn inventory (§5.1) |
| `mount` | content ID or null | `arch.mount.*` for cavalry (§5.6) |

Attack profile fields: `id` (`enemy.ab.*`), `kind` (`melee`, `projectile`, `hitscan`, `beam`, `area`, `lob`, `aura`, `summon`, `self`), `dmg_mult_bp`, `shares` (map `dmg.*` → Bp summing to 10000), `pen_bp`, `windup`, `active`, `recovery`, `cooldown` (all ticks), `range_mm`, `min_range_mm`, `radius_mm`, `cone_bam`, `speed_mm_s`, `impact_mp` (milli-poise), `status` (list of `{st, power}` or `{st, direct_ticks}`), `dep` (subset of the four axioms), `law_bound` (a `law.*` ID or null, Canon §5.10), `part` (the `bp.*` that performs it; a disabled or severed part disables the attack, doc 04), `unparryable`, `unblockable` (doc 05), `afflict` (`{id, power}` or null, doc 04), `telegraph` (`none`, `circle`, `cone`, `line`, `ring`, `marker`), `token` (`melee`, `ranged`, `control`, `grenade`, `none`). An attack whose `telegraph` is not `none` is flagged `telegraphed` for doc 14's scoring.

Example record:

```json
{
  "id": "enemy.space_opera.navy_trooper",
  "role": "enemy.role.soldier",
  "origin_genre": "genre.space_opera",
  "rank": "rank.standard",
  "body": "body.humanoid",
  "body_state": { "radius_mm": 300, "stand_mm": 1800, "eye_mm": 1620, "mass_g": 80000,
                  "kb_resist_bp": 0, "hitbox_scale_bp": 10000, "flags": 115 },
  "kin": "kin.human",
  "mutation": null,
  "tags": ["human", "sapient", "enforcer"],
  "faction": "fac.native.space_opera.imperial_navy",
  "hp_mhp": 60000, "dmg_mhp": 10000, "cycle_ticks": 90,
  "walk_mm_s": 3600, "run_mm_s": 5800, "nav": "ai.nav.humanoid",
  "subs": { "sub.sight": 45 }, "armor_poise_mp": 0, "ar": 10,
  "resist": { "dmg.shock": 1000 }, "immune": [], "immune_status": [],
  "stamina_m": 80000, "aether_max_mae": 0, "aether_regen_mae_s": 0, "spells": [],
  "block_stability_bp": 0, "guard_arc_bam": 0,
  "senses": ["sight", "hearing"],
  "perception": null,
  "attacks": [
    { "id": "enemy.ab.blaster_volley", "kind": "projectile", "dmg_mult_bp": 4000,
      "shares": { "dmg.fire": 5000, "dmg.pierce": 5000 }, "pen_bp": 1000,
      "windup": 18, "active": 24, "recovery": 30, "cooldown": 90,
      "range_mm": 25000, "min_range_mm": 0, "radius_mm": 0, "cone_bam": 0,
      "speed_mm_s": 60000, "impact_mp": 4000, "status": [],
      "dep": ["techne"], "law_bound": null, "part": "bp.arm_r", "unparryable": 0,
      "unblockable": 0, "afflict": null, "telegraph": "none", "token": "ranged" }
  ],
  "behavior": "ai.set.soldier",
  "loot_role": "marksman",
  "weight": 3000,
  "flags": { "sapient": 1, "can_surrender": 1, "uses_items": 1, "weapon_pickup": 1,
             "morale_immune": 0, "poise_immune": 0, "no_loot": 0, "keeper": 0, "summoned": 0,
             "outfit": 1, "reckless": 0, "interrupter": 0 },
  "outfit_mass_g": 6000,
  "carry": [ { "cat": "cat.potion", "chance_bp": 1500, "count": 1 } ],
  "mount": null
}
```

The blaster volley fires 3 bolts 12 ticks apart inside its 24-tick active window, each at `dmg_mult_bp` 4000, so one volley is 12,000 mHP of template damage. Booleans are integers 0 or 1 because content holds integers only. `flags` 115 is PUSHABLE, COLLIDES_BODIES, CAN_MANTLE, CAN_CLIMB and EDGE_GUARD (bits 0, 1, 4, 5, 6).

### 2.2 Stat scaling

For an enemy spawned or re-evaluated at depth `d` (with `d_pow = min(d, 300)`), party size `n` and rank `r`. Throughout this chapter `d` is doc 14 §3.1's depth `d(f)` of the current Folio (`d_eff` in Abridged, Canon §17.1), while per-Folio streams, EntityIds and §10.4's `volume` use the Folio index `f`:

```
hp_max_mhp   = mul_bp(mul_bp(mul_bp(mul_bp(tpl.hp_mhp, H(d)), r.hp_bp), party_hp_bp(n, r)), diff_hp_bp(r, f))   // Canon §13.8, §17.2, §17.3; doc 14 §7.3
tpl_atk_mhp  = mul_bp(tpl.dmg_mhp, atk.dmg_mult_bp)                                     // content-level attack template
B            = mul_bp(mul_bp(tpl_atk_mhp, T(d)), r.dmg_bp)                              // Canon §10.4 enemy source
AR_part      = mul_bp(tpl.ar, P(d))                                                     // every part; then statuses (Canon §10.3)
sub_eff      = min(150, tpl.subs[x] + floor_div(3 * (d - 1), 2))                         // Canon §13.8
poise_m      = (20 + floor_div(BRN_eff, 2)) * 1000 + tpl.armor_poise_mp                 // Canon §6.4 row 18
status power = p × S(d)                                                                  // Canon §11.2
```

`party_hp_bp(n, r)` reads the first row of Canon §17.2 for ranks minion to champion and the second row for boss, editor and hunter. `diff_hp_bp` is doc 14's Grade and Marginalia HP chain, 10000 at Grade 0 in the main run; doc 14 also owns the step-2.13 damage factor. The party-size damage multiplier is applied only at pipeline step 2.14 against Errata (Canon §10.5). Template HP never exceeds 400,000 mHP (doc 14 integer-safety bound). Stats are computed at spawn. An enemy that crosses a Folio boundary does not exist; enemies never leave their Folio.

**Worked example 1.** A standard `enemy.space_opera.navy_trooper` in Folio 10 with a party of 2 at Grade 0 (`diff_hp_bp` 10000): `hp_max = mul_bp(mul_bp(mul_bp(60000, 34423), 10000), 16000) = 330,460` mHP, unchanged by the final × 10000. A full-damage template hit (multiplier 10000) gives `B = mul_bp(mul_bp(10000, 16190), 10000) = 16,190` mHP, which step 2.14 raises to 17,809 before mitigation. `AR = mul_bp(10, 21718) = 21` per part. BRN is `40 + floor_div(27, 2) = 53`, so poise is 46,000 milli-poise.

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

Every kin appears in the rosters (§8): human (`navy_trooper` and most sapient soldiers), construct (`flesh_golem`, `chrome_brawler`, `mantis_assassin`), undead (`ghoul`, `vampire_courtier`, `dead_miner`, `jiangshi`, `shade_warrior`), beastkin (`orc_raider`, `orc_sapper`, `werewolf`), fae (`bridge_troll`, `thorn_knight`, `wild_hunt_rider`) and mutant (`deep_one` gills, `star_seer` and `faceless_teacher` extra_arm, `pirate_boarder`, `mutant_behemoth` and `faceless_agent` hide, `glow_zealot` and `tunnel_mole` regrowth). The content validator rejects a roster that leaves any kin unused.

Tag vocabulary (closed enum, validated by schema): `human`, `construct`, `undead`, `beastkin`, `fae`, `mutant`, `beast`, `fiend`, `divine`, `eldritch`, `swarm`, `flier`, `swimmer`, `breathless`, `burrowing`, `sapient`, `mindless`, `electronic`, `plant`, `elemental`, `editor`, `summoned`, `mounted`, `enforcer`.

| Tag | Rule |
|---|---|
| `undead`, `fiend`, `divine`, `eldritch` | Canon §5.7 MYTHOS ambient at step 2.7; `undead` and `fiend` take × 15000 radiant at step 3a |
| `mindless` | immune to `st.feared`, `st.charmed`, `st.dread`, `st.taunted`; `morale_immune` |
| `electronic` | eligible for `st.hacked` (Canon §11.1) |
| `swarm` | body plan `body.swarm`; takes × 15000 area damage (Canon §9) |
| `editor` | member of `fac.editors`; erases Errata-placed blocks (§10.6; the player-controlled invader excepted); abilities have empty `dep` (§2.6) |
| `summoned` | grants 0 kill XP (§11.1); despawns when its summoner dies |
| `sapient` | can talk, parley, surrender and witness reputation events (§12.3) |
| `breathless` | no `res.breath` drain (doc 04); set for kin construct and undead, the `gills` mutation and every `editor` |
| `enforcer` | a doc 09 §11 disguise enforcer regardless of rank and role; set on `navy_trooper`, `corp_enforcer`, `crooked_cop`, `witch_hunter`, `marshal_rifleman` and `compliance_officer` |
| `flier` | selects the doc 09 flier perception profile (doc 09 §4.2 rule 8, which must read `flier`, Canon gaps) |

Tag bits for doc 03's `comp.tags` (`u64`): 0 `undead`, 1 `fiend`, 2 `divine`, 3 `eldritch`, 4 `construct`, 5 `electronic`, 6 `flier`, 7 `swimmer`, 8 `swarm`, 9 `breathless`, 10 `human`, 11 `beastkin`, 12 `fae`, 13 `mutant`, 14 `beast`, 15 `burrowing`, 16 `sapient`, 17 `mindless`, 18 `plant`, 19 `elemental`, 20 `editor`, 21 `summoned`, 22 `mounted`, 23 `enforcer`; bits 24 to 63 are reserved and must be 0.

Senses: `sight` and `hearing` follow doc 09. `breath` detects any breathing entity within 6,000 mm through walls and replaces sight (the jiangshi rule; an Erratum holding its breath with `input.hold_breath` is undetectable to it, doc 09 §4.6). `tremor` detects footstep noise events at or above 25 dB within 12,000 mm through solid blocks. `thermal` ignores gas concealment (Canon §12.2) for sight rays. `scent` follows doc 09 scent marks (hounds, `fox_spirit`, `werewolf`, `big_bad_wolf`, `sewer_gator` and the beast companions carry it).

### 2.5 Resistance profiles

`resist[t] = role default + template value + kin and tag adjustments`, summed, then clamped to the NPC cap of 9000 at pipeline step 4 (Canon §10.2). Genre-wide affinities from the doc 12 GPV `damage_affinity` add last. Role defaults appear in §3.

### 2.6 Contradiction for NPCs

Enemy innate attacks use the template's `origin_genre` and the `dep` of each attack (Canon §5.3). Tolerance is `origin_tol = 2` on every axis where the origin genre's axiom is at least 5 (Canon §5.4); kin fae adds 1 on ARCANA. A foreign enemy in a bleed zone or in a Folio of another genre carries its own Contradiction and its own Anachronism Edge against natives (Canon §5.5, §5.6).

Example: `enemy.space_opera.navy_trooper` has `dep = {techne}`, `Req = max(1, 10 - 2) = 8`. Inside a Fairy Tale bleed zone (TEC 1), `raw_gap = 7`, `Tol = 2`, `gap = 5`, so `C = 5`: potency 5000, 1500 Bp jam chance per volley rolled on keyed `rng.combat`, and `E = 6`, so +2400 Bp against Fairy Tale natives.

Every attack of a `fac.editors` member has `dep = {}`. Editors and Red Ink servants are therefore Coherent everywhere and never receive Anachronism Edge. Every `fac.editors` attack whose profile does not name another affliction carries `afflict: {"id": "afflict.red_ink", "power": 20}` (doc 04 §10.1).

**World Laws.** An attack with `law_bound` is Disconnected (C = 6) wherever its Law is inactive (Canon §5.10): `enemy.wuxia.sect_disciple`'s `qinggong_leap` and `boss.wuxia.sword_saint`'s `qinggong_descent` carry `law.wuxia.qinggong`; `enemy.weird_west.marshal_rifleman`'s `high_noon` and `boss.weird_west.hanging_judge`'s `high_noon_duel` carry `law.weird_west.high_noon`. A disciple chasing an Erratum out of a Wuxia bleed zone loses its leap. Law tolerance grants apply to NPC components of the granted class exactly as to Errata components.

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

`enemy.role.soldier` is the reference role of Canon §17.4: 60 HP, 10 HP per hit, AR 10, so `REF_PLAYER_DPS_D1` of 30 kills it in `REF_STANDARD_TTK_D1` = 2.0 s. At Folio 1 a soldier deals 10 HP per 90 ticks, 6.67 HP per second before mitigation. Its basic attack is 100% `dmg.pierce` (armor efficiency 7500), so against a 20-AR Erratum `effAR = floor_div(20 × 7500, 10000) = 15` and each 10,000 mHP hit loses `DR = floor(150,000,000 / 55,000) = 2727` Bp: each hit lands 7,273 mHP and a lone soldier needs 12 hits to kill an 85 HP Erratum. The first lands on tick 18 and the 12th on tick `18 + 11 × 90 = 1,008` (16.8 s). A slash attacker of the same size needs 13 hits (`DR` 3333, 6,667 mHP each).

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
| caster | projectile, `dmg.arcane` 10000 (the skin sets the type), windup 45 | `enemy.ab.blink`: teleport 6000 mm to a walkable cell out of the target's frontal cone, windup 30, cooldown 600 |
| controller | projectile, `dmg.psychic` 10000 | `enemy.ab.hold`: direct `st.rooted` 2 s, windup 40, cooldown 600, token control |
| support | projectile, `dmg.radiant` 10000 | `enemy.ab.mend`: heal one ally 1500 Bp of its Max HP, windup 45, cooldown 300; `enemy.ab.bolster`: direct `st.fortified` on one ally, cooldown 900 |
| guardian | melee, `dmg.slash` 10000, windup 24 | frontal block arc 10923 Bam with `block_stability_bp` 7500 while not attacking; `enemy.ab.shield_bash`: × 12000, direct `st.staggered`, cooldown 480 |
| summoner | projectile, `dmg.umbral` 10000 | `enemy.ab.summon`: 2 swarmers of its genre roster at rank minion with tag `summoned`, windup 60, cooldown 900, at most 4 alive |
| stalker | melee, `dmg.slash` 10000, `st.bleeding` power 40 | `enemy.ab.vanish`: direct `st.invisible` 10 s, cooldown 600; `enemy.ab.ambush` adds +5000 Bp at step 1 to the first hit after `st.invisible` ends by attacking |
| artillery | lob, `dmg.blunt` 5000 + `dmg.fire` 5000, radius 2500, windup 45, telegraph circle | `enemy.ab.barrage`: 3 lobs at cells 2000 mm apart, windup 60, cooldown 1200 |
| sapper | melee, `dmg.blunt` 10000 | `enemy.ab.demo_charge`: places a charge that detonates after 120 ticks, × 25000, radius 3000 mm, breaks every destructible-tagged or Errata-placed block within 2000 mm, telegraph ring, cooldown 900 |
| flier | melee dive, `dmg.slash` 10000, windup 24 | `enemy.ab.strafe`: projectile ranged attack at 15000 mm while circling |
| cavalry | melee lance, `dmg.pierce` 10000 | `enemy.ab.lance_charge`: mounted charge per Canon §17.7, windup 30, telegraph line, cooldown 480 |

## 4. AI architecture

### 4.1 Pipeline and schedule

The AI is a utility AI (Dave Mark's infinite-axis model, R4 §9) with an action layer, a squad coordinator and a navigation service. It plugs into doc 03's system order (doc 03 §2.2): `sys.spawn_admit` (system 3) admits this chapter's `request_spawn` calls; `sys.ai_think` (system 6, owner 13) runs the four AI stages below in order and writes each agent's `comp.control`; `sys.actions` (system 7, docs 05 and 06) executes attacks and casts from that control; `sys.erasure` (system 32, owner 13) runs §10.5 and §10.6. Asleep entities (doc 03 §15.2) skip `sys.ai_think` entirely and draw nothing.

| Stage inside `sys.ai_think` | Period | Phase | Iteration order | Draws |
|---|---|---|---|---|
| 1. Squad coordinator (§4.10) | 30 | `squad_id % 30` | ascending `squad_id` | none |
| 2. Director (§10) | 30 | 11 | one instance per Folio | `rng.spawn` |
| 3. Agent think (§4.5) | 6 (Canon §2.2) | `(EntityId & 0xFF_FFFF_FFFF) % 6` | ascending EntityId | 1 keyed `rng.ai` per think that selects an action, 0 for a committed think; idle and wander targets from `rng.ai_idle` |
| 4. Path service (§4.13) | 1 | 0 | priority queue (§4.14) | none |

```rust
fn think(a: &mut Agent, w: &World, t: Tick) {               // only awake agents reach here
    update_blackboard(a, w, t);                             // §4.2, §4.8, §4.9
    if a.bb.action.is_committed() { return; }               // every §4.6 state except Idle and Moving: no draw
    let set = w.db.behavior_set(a.tpl.behavior);
    let mut scored: SmallVec<[(ActionId, i32); 24]> = SmallVec::new();
    for act in set.actions_sorted_by_id32() {               // ascending id32, so order is content-stable
        if !act.precondition(a, w) { continue; }
        let mut s = score_action(act, a, w);                // §4.5
        if Some(act.id) == a.bb.action.continuing() { s = mul_bp(s, MOMENTUM_BP); }
        if s > 0 { scored.push((act.id, s)); }
    }
    if scored.is_empty() { scored.push((AI_ACTION_IDLE, 1)); }
    let best = scored.iter().map(|x| x.1).max().unwrap();
    let thr  = core::cmp::max(1, mul_bp(best as i64, BAND_BP) as i32);
    let elig: SmallVec<[(ActionId, i32); 24]> = scored.into_iter().filter(|x| x.1 >= thr).collect();
    let weights: SmallVec<[u32; 24]> = elig.iter().map(|x| x.1 as u32).collect();
    let mut rng = keyed_rng("rng.ai", w.folio, a.id, t, 0);
    let pick = rng.weighted(&weights);                      // exactly one range() call
    start_action(a, elig[pick].0, w, t);                    // writes comp.control (§4.6)
}
```

Constants: `MOMENTUM_BP = 12500`, `BAND_BP = 9000`. The band turns near-ties into a weighted random choice, which is the "tie-break" use Canon §3.3 assigns to `rng.ai`. A think that selects a position takes its extra draws in this fixed way: search points, summon cells and the boss `classified` and `summary_judgment` picks use keyed `rng.ai` with `seq = 1`, with the draw count stated where each is defined; a wander point uses one `range` call on the sequential `rng.ai_idle` (§4.7). Cover, flank and flight cells are chosen deterministically (§4.9, §4.11, §4.12) and draw nothing. Feared, fleeing, surrendered and other committed agents draw nothing in their think.

### 4.2 Blackboard

Each agent owns one blackboard. It is simulation state and is serialized in saves (Canon §2.5).

```rust
pub struct Blackboard {
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

`update_blackboard` runs at the start of each think: it copies awareness and last-known positions from doc 09, decays threat (§4.8), applies morale changes queued since the last think (§4.9), drops a cover cell whose cover level fell to none, and drops a path whose goal moved more than 4000 mm.

### 4.3 Inputs

Every input returns `0..=10000`. Distances are `sqrt_fx` of `dist2` between origins, converted to mm by `(raw × 1000) >> 32`. **Bearing.** `bearing(a → b)` is the yaw whose forward vector (Canon §1.9, yaw 0 faces −Z) points from `a` to `b` horizontally: with `dx = b.x.raw - a.x.raw` and `dz = b.z.raw - a.z.raw`, `bearing = atan2_bam(-dx, -dz)` (Canon §1.9 `atan2_bam(y, x)`); when `dx = dz = 0` it is `a`'s own yaw.

| Input ID | Value |
|---|---|
| `ai.input.self_hp` | `floor_div(hp × 10000, hp_max)` |
| `ai.input.target_hp` | the same for the current target |
| `ai.input.target_dist` | `clamp(floor_div(dist_mm × 10000, ref_mm), 0, 10000)` with `ref_mm` from the consideration |
| `ai.input.target_visible` | 10000 if doc 09 reports sight of the target on the latest perception tick, else 0 |
| `ai.input.target_aware` | doc 09 §6.1's value for the agent's highest pair (ties to the newest `lkp_tick`, then the lower subject, the disturbance as subject 0): `aware.unaware` 0, `aware.curious` 2500, `aware.suspicious` 5000, `aware.searching` 6000, `aware.lost` 8000, `aware.alert` 10000 |
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

States: `Idle`, `Moving`, `Windup`, `Active`, `Recovery`, `Channel`, `Interrupted`, `Feared`, `Fleeing`, `Surrendered`, `Spared`, `Dead`. Rows are evaluated every tick at the start of `sys.ai_think` stage 3 for every awake agent in ascending EntityId (asleep agents keep their state), in table order; the first matching row fires. Rows conditioned on "think selects" fire only on the agent's think tick (§4.1). The timed rows (`t == phase_until`) of `Windup`, `Active` and `Recovery` are the phase changes `sys.actions` reports for the attack profile it runs (docs 05, 06).

| From | Condition | To | Side effects |
|---|---|---|---|
| any except `Dead` | death (Canon §10.8 step 3) | `Dead` | release token, path and cover cell; retract telegraph |
| `Spared` | origin inside the exit cell's room, or 1800 ticks since entering `Spared` | (despawned) | `ev.entity_despawned` |
| any except `Dead`, `Spared` | a disabling status starts: `st.staggered`, `st.stunned`, `st.frozen`, `st.knocked_down`, `st.sleeping`, `st.polymorphed` | `Interrupted` | if leaving `Windup`, `Active` or `Channel`: remaining cooldown halved, telegraph retracted, hits already resolved stay; token released |
| `Interrupted` | no disabling status remains | `Idle` (or `Surrendered` if it was surrendered) | |
| any except `Dead`, `Spared`, `Surrendered`, `Interrupted` | `st.feared` starts | `Feared` | as for `Interrupted`; `flee_from` = the status source's position |
| `Feared` | `st.feared` ended | `Idle` | |
| any except `Dead`, `Spared`, `Interrupted` | `st.charmed`, `st.hacked` or `st.berserk` starts | `Idle` | current action aborted as for `Interrupted`; from the next think the allegiance (`st.charmed`: the applier's faction; `st.hacked`: §5.7) or targeting (`st.berserk`: §4.8) rule applies |
| `Windup`, `Active` (weapon attacks) | `st.disarmed` starts | `Idle` | weapon attacks fail their precondition while it lasts |
| `Windup`, `Channel` (abilities whose `dep` contains `arcana` or the origin genre's aether axiom, and every `spell.*`) | `st.silenced` starts | `Idle` | remaining cooldown halved |
| any except `Dead`, `Spared`, `Surrendered`, `Fleeing` | morale broken (§4.9) | `Surrendered` or `Fleeing` | release token |
| `Fleeing` | `t >= flee_until` | `Idle` | `morale_bp = 5000`; doc 09 disturbance set to `aware.searching` at its own position |
| `Surrendered` | takes any damage | `Fleeing` | `flee_until = u64::MAX` (rest of the Folio) |
| `Surrendered` | an Erratum completes a 30-tick spare interact | `Spared` | faction becomes `fac.neutral`; walks to the nearest exit door; `ev.entity_pacified` (§11.1) |
| `Idle` | think selects an attack or ability | `Windup` until `t + windup` | acquire token; `cooldown_until = t + cooldown`; publish telegraph |
| `Idle` | think selects a channel action: `use_item` (45), `pick_up` (30), `eat_corpse` (120), `raise_alarm` (60), `relight` (60), `wake_ally` (60), `call_allies` (30) or an ability of kind `self` or `aura` with a channel time | `Moving` to the action's cell when farther than 1500 mm, else `Channel` until `t + channel_ticks` | |
| `Idle` | think selects a movement action | `Moving` | request path (§4.13) |
| `Idle` | think selects `idle` | `Idle` | |
| `Moving` | goal reached for a channel action | `Channel` until `t + channel_ticks` | |
| `Moving` | goal reached for a movement action, or the path service returns `NoPath` | `Idle` | release path |
| `Moving` | next think selects another action | that action's first phase | |
| `Windup` | target died, despawned or became non-hostile | `Idle` | `cooldown_until = t`; release token; retract telegraph |
| `Windup` | `t == phase_until` | `Active` until `t + active` | ranged aim locks 6 ticks before this transition |
| `Active` | `t == phase_until` | `Recovery` until `t + recovery` | |
| `Recovery` | `t == phase_until` | `Idle` | token released at `t + 30` |
| `Channel` | the action's target or cell became invalid (target dead, item taken, prop destroyed) | `Idle` | no effect |
| `Channel` | `t == phase_until` | `Idle` | the action's completion effect |

While `Feared`, `comp.control` moves at run speed along the path that maximizes distance from `flee_from` (the §4.9 flight cell search with `flee_from` as the only threat; no draw) and never attacks (doc 06 §15). While `Fleeing`, the same with every known threat. `Surrendered` and `Spared` agents never attack and are never assigned tokens or squad orders. NPCs have no composure, so `st.dread` acts only through morale (§4.9, doc 06 G10).

**Control.** `start_action` writes `comp.control` (doc 03 §3.4): `move_x` and `move_z` toward the next waypoint, `yaw` and `pitch` toward the aim point, and the attack button with the attack index. `sys.actions` runs the windup, active and recovery of the attack profile (docs 05, 06) and reports each phase change into `ActionState`, which this chapter uses for commitment. Control persists between thinks.

**Turn rates.** One think changes yaw by at most 8196 Bam (6 × 1366) out of an attack and 3276 Bam (6 × 546) during a melee windup, 1638 Bam for brutes and ranks boss and above, so sidestepping works. Ranged aim is re-aimed at each think up to the last think that starts at least 6 ticks before `Active`, then fixed.

**Reaction delay.** An agent that reached `aware.alert` on tick `a` cannot start an attack windup before `a + 18` (minion, standard), `a + 12` (elite, champion) or `a` (boss, editor, hunter).

**Telegraphs.** A telegraph is view data published in the doc 03 view frame, never an event: `{ shape: circle | cone | line | ring | marker, origin: Fx3, yaw: Bam, length_mm, radius_mm, half_angle_bam, start: Tick, impact: Tick, attacker: EntityId }`. The client draws it from `start` to `impact`.

**NPC ranged spread.** The half-angle passed to doc 05's two spread draws is the template spread (soldier 273 Bam, shooter 364, sniper 46, caster 182, artillery 0 with a 1000 mm landing scatter) × Canon §6.4 row 15 (PRC), × the darkness factor of the target cell's effective light (≤ 5: 18000, 6 to 9: 14000, 10 to 12: 11500, otherwise 10000; R5 §9.2 item 13), × 12000 when the target moves faster than 4300 mm/s.

### 4.7 Action catalog and behavior sets

Every action lists its weight and considerations in the order they multiply. `[ref]` is the input parameter.

| Action ID | Weight | Considerations | Notes |
|---|---|---|---|
| `ai.action.idle` | 1000 | none (always 10000) | stand, play idle; while the highest pair is `aware.curious` it stops and turns its head to `lkp` (doc 09 §6.1) |
| `ai.action.wander` | 2000 | `target_aware` → `binary_not` | goal = `cells[range(n)]` with one `range` call on `rng.ai_idle`, where `cells` lists the walkable cells within 8000 mm straight-line distance of `home` in ascending cell index, truncated to the first 256 (`n` its length; with `n = 0` the agent idles and draws nothing) |
| `ai.action.patrol` | 3000 | `target_aware` → `binary_not` | doc 12 patrol route |
| `ai.action.investigate` | 7000 | precondition `target_aware ∈ {5000, 8000}` (`aware.suspicious` or `aware.lost`); `target_aware` → `gate_half` | suspicious: walk to `lkp` with weapon ready, never attacks (SPD v4, R1 §2.12); lost: run to the doc 09 §6.5 predicted `lkp`; attack actions return as soon as doc 09 T5 restores `aware.alert` |
| `ai.action.search` | 6000 | precondition `target_aware` = 6000 | doc 09 §6.7 plan: visit P0, then `min(3, n)` points (`n` = drawable plan points) drawn without replacement by `min(3, n)` `weighted` calls on keyed `rng.ai` seq 1 with the plan weights, a chosen point's weight set to 0; 60 ticks of look-around at each point; squad split per doc 09 §6.7 |
| `ai.action.return` | 4000 | precondition `target_aware` = 0 and origin more than 2000 mm from `home` or off its patrol route | path home, or to the nearest patrol waypoint (doc 09 T8 "returns to post") |
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
| `ai.action.raise_alarm` | 9500 | precondition: sapient, `target_aware` = 10000, a bell or klaxon prop (doc 12 dressing) within 16000 mm of path, no `raise_alarm` by its faction within doc 09 `CALLOUT_COOLDOWN`; `allies_near` → `linear_down` | walk to the prop, 60-tick channel, then doc 09 `raise_alarm(faction, prop pos, target, bell)` |
| `ai.action.relight` | 6500 | precondition: sapient, not `construct`, `undead` or `beast`, a doc 09 §2.4 darkness disturbance at `aware.suspicious`, `target_aware` < 10000 | walk to the darkened cell (or the room's fuse box for an electric cut, doc 09 §2.4); 60-tick channel; then doc 07's lit state of the fixture is written with `set_block` and cause `grid.cause.toggled` (doc 03); needs `uses_items` for flame sources |
| `ai.action.wake_ally` | 8500 | precondition: sapient, an ally with `st.sleeping` or doc 09 subdued within 16000 mm of path, `target_aware` < 10000 | walk to it, 60-tick interact; the ally wakes and enters `aware.searching` at its own position with Vigilance (doc 09 §9.4) |

Behavior sets (`ai.set.*`) list the actions an agent scores. Every set also contains `idle`, `wander`, `patrol`, `investigate`, `search`, `return`, `call_allies`, `raise_alarm`, `relight` and `wake_ally` (the last four sapient only) and `ability` entries for each non-basic attack. The largest set, `ai.set.soldier`, scores 10 extras, 10 common actions and its abilities, inside the 24-action budget (§4.14).

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

Threat generation and decay are doc 05 §14.1: damage dealt × Canon §6.4 row 39 × attack class, blocked damage, healing of allies, taunts, and a loss of `floor_div(entry × 30, 10000)` per think. This chapter adds one rule: when the agent reaches `aware.alert` toward an entity, that entity's entry becomes at least 1000. Threat earned against a mount is credited to its rider, and threat caused by a minion to the minion (doc 05).

`ai_target(e)` runs at each think over alerted, perceivable entries (ghosts are never perceivable, Canon §17.6):

```
eff(c) = threat[c]; then × 13000 if dist(e, c) <= 3000 mm; then × 2500 if c has st.downed      // one mul_bp each
no current target:  argmax eff(c), ties to the lower EntityId
current target cur: switch to x only when eff(x) > mul_bp(eff(cur), 11000)
```

Forced targets override `ai_target` in this order: `st.taunted` forces the taunter for its duration (doc 05); a `fac.editors` member with line of sight to an Erratum bearing doc 10's `curse.red_letter` within that curse's radius targets the nearest such bearer; an agent with `st.berserk` targets the nearest entity by `dist2`, ties to the lower EntityId. Excluded candidates: ghosts, `Surrendered` and `Spared` agents, and an honor-duel duelist for everyone but its duel champion (§10.4). Example: target A holds 4000 threat at 8000 mm and B holds 4000 at 2500 mm: `eff(B) = 5200 > mul_bp(4000, 11000) = 4400`, so the agent switches to B.

### 4.9 Morale, surrender and flight

`morale_bp` starts at 10000. Changes are queued by events and applied at the next think:

| Event (within 16000 mm and seen) | Δ |
|---|---|
| allied death | −1500; −2500 if it was elite rank or higher; −3000 if it was the squad leader (the most negative applicable value only) |
| own HP first falls below 3000 Bp | −2000 (once) |
| squad reduced to half its spawn size | −1000 (once) |
| each think in combat while living allies ≥ 2 × alerted Errata | +100 |
| `enemy.ab.bolster` or `elite.rallying` pulse | +1500 |
| `st.dread` applied to the agent (NPCs have no composure, doc 06) | −2000 |

Agents tagged `construct`, `mindless`, `undead` or `editor`, ranks boss, editor and hunter, and `summoned` agents are morale-immune. Below 2500:

- a `sapient` agent with `can_surrender` and rank minion or standard whose HP is below 2500 Bp **surrenders**: it drops its held weapon, kneels and stops acting. Any damage ends the surrender and makes it flee for the rest of the Folio. An Erratum's 30-tick interact spares it (§12.3): it becomes `fac.neutral`, walks to the nearest exit and despawns, and the party receives its kill XP as a defeat (§11.1);
- every other morale-capable agent **flees** for 600 ticks (`flee_until = t + 600`), then sets morale to 5000 and enters `search`. **Flight cell:** the 16 candidates are the walkable cells nearest (by `dist2`, ties lower cell index) to the points 24000 mm from the agent at bearings `i × 4096` Bam for `i in 0..16`; among the reachable ones the agent takes the cell with the largest minimum path distance to every known threat (ties to the lower cell index) and repaths to a new flight cell when it arrives before `flee_until`. No draw.

### 4.10 Squad coordinator and attack tokens

A squad is created for every spawned pack and holds at most 8 members. Alerted agents of the same faction within 12000 mm merge into the squad with the lower `squad_id` while the total stays at most 8. The leader is the member of highest rank, ties to the lower EntityId.

Each coordinator run (period 30) does, in order:

1. **Shared knowledge.** Every member's `last_known` takes the newest entry per target from all members.
2. **Target spread.** For `m` members and `k` perceived targets, each target receives at most `ceil_div(m, k) + 1` members. Members keep their current target when it fits the cap; the rest are reassigned in ascending EntityId to the target with the fewest assigned members, ties to the lower target EntityId.
3. **Tokens.** Every potential target holds a token pool: melee 3, ranged 4, control 1, grenade 1. Requests are granted in descending action score, ties to the lower EntityId. A token is held from `Windup` to 30 ticks after `Recovery`. Elite, champion, boss, editor and hunter ranks never need tokens.
4. **Orders.** `engage` by default. `flank` to at most 2 members per target (skirmisher and stalker roles first, then lowest EntityId) when at least 3 members are engaged. `suppress` to shooters when their target holds half or full cover. `fall_back` to all members when mean squad morale is below 4000: regroup within 6000 mm of the leader. `search` when no member has a target at `aware.alert` for 300 ticks.

### 4.11 Cover from the voxel grid

**Cover map.** Each chunk keeps a cover map rebuilt within 2 ticks of any `ev.block_changed` inside it: for every walkable cell (§4.13) and each of the 8 horizontal directions, the height of solid opaque blocks in the adjacent cell (0, 1 or 2 blocks above the floor).

**Cover level** of cell `c` against threat `h`: cast two voxel DDA rays from `h`'s eye to points 900 mm and 1500 mm above `c`'s floor. A ray is blocked if it crosses a solid opaque block or accumulates concealment ≥ 15 (Canon §12.2). Both blocked: full (10000). Only the 900 mm ray blocked: half (6000). Otherwise none (0).

**Query.** `take_cover` collects up to 48 candidate cells within path distance 16000 mm that have a non-zero cover-map entry, in ascending cell index, and scores each:

```
H            = the up-to-3 known enemies with the highest threat entries, ties to the lower EntityId
w_h          = max(1, threat[h])
cover_score(c) = floor_div(Σ_{h ∈ H} level(c, h) × w_h, Σ_{h ∈ H} w_h)                 // i64; 0 when H is empty
total(c)       = mul_bp(cover_score(c), 10000 - floor_div(clamp(floor_div(path_mm(c) × 10000, 16000), 0, 10000), 2))
                 then × 5000 if another squad member holds c or a cell adjacent to c
```

The highest `total` wins, ties to the lower cell index. Each query casts at most `48 × 3 × 2 = 288` rays.

### 4.12 Flanking

A flanker's goal is a walkable cell 6000 to 10000 mm from its target with `ang_diff(target.yaw, bearing(target → cell)) > 10923` Bam, which is the Canon §10.5 step 2.4b flank cone. Candidates are the 16 points 8000 mm from the target at bearings `target.yaw + 16384 + floor_div(i × 32768, 15)` (wrapping `Bam` add) for `i in 0..16`, which spans the rear half-circle symmetrically from 90° left to 90° right; each is snapped to the nearest walkable cell (by `dist2`, ties lower cell index), and a snapped cell that fails the distance or cone test is dropped. The reachable one with the lowest path cost wins, ties to lower cell index; with none the agent keeps `engage`. No draw. Its path uses a danger overlay: cells inside the target's frontal cone (half-angle 10923 Bam) and within 12000 mm cost × 3, so flankers circle outside the target's view.

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
| locked door | key holder (a member of the door's faction, or a hacked construct at an electronic door, §5.7): opens it; `ai.nav.large` or brute role: breaks it; otherwise impassable | key holder 1000 + 1500; breaker 1000 + `floor_div(break_ticks(door) × walk_mm_s, 60)` |
| dig | burrower or sapper; destructible-tagged or Errata-placed block (hunters: §10.5) | 1000 + `floor_div(dig_ticks × walk_mm_s, 60)` |
| hazard | cell with fire, lava contact, acid or electrified liquid | × 3; lava impassable unless `immune` contains `dmg.fire` |
| light | agents with tag `undead` or `fiend` at `light_at ≥ 12` | × 4 |

`dig_ticks = ceil_div(break_ticks(material) × 10000, dig_speed_bp)`, where doc 07 provides `break_ticks` and `dig_speed_bp` is 20000 for the sapper role, 10000 for other burrowers. Diggers dig only destructible-tagged and Errata-placed blocks, and never inside a sealed arena; doors follow the door rows.

**Search.** Paths are found by A* over cells inside a chunk and by HPA* across chunks. A chunk's portals are maximal runs of walkable cells on each chunk face, per level; abstract edges between portals of one chunk store the intra-chunk A* cost. A block change marks its chunk dirty; at most 2 dirty chunks are rebuilt per tick, in FIFO order. The heuristic is octile distance in cost units (horizontal octile plus 1200 per block of vertical difference), which is admissible. Ties in the open list break by lower `f`, then lower `h`, then lower cell index (doc 02 §2.3). The cell index is `x + 256 × (z + 256 × y)`.

**Flow fields.** Swarmers and every agent of `ai.nav.small` targeting the same Erratum share one flow field: a Dijkstra integration field from the target's cell over a 64 × 64 horizontal window centered on it, all levels within ±8 blocks, using the link costs below. A field is rebuilt on the first tick on which both hold: the target's feet cell differs from the field's source cell, and at least 30 ticks have passed since the field's last build. At most 4 flow fields exist, kept in a `BTreeMap` keyed by target EntityId; a request for a fifth target falls back to A* for that agent. A field with no subscribing agent for 60 ticks is freed. An agent outside its field's window, or on a cell the field marks unreachable, also falls back to A*.

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

**Sleep.** Doc 03 §15.2 decides when an agent sleeps: inactive chunk, grounded, not `in_combat`, and unaware toward every Erratum. Sleeping agents do not think, path or draw. A squad `engage` order or any damage also wakes an agent.

At 256 active actors and period 6, on average `256 / 6 ≈ 42.7` thinks run per tick; the exact count per tick follows the EntityId phase keys. The host AI budget is 1.5 ms per tick for thinks, squads, the Director and the path service together; doc 02 owns the performance table that enforces it.

## 5. Items, spells, stealth, weapons, summons, mounts and hacking

### 5.1 Carried items

A template's `carry` list is rolled once at spawn with `rng.spawn`, in listed order, immediately after that member's §10.2 template, modifier and promotion draws. Entities tagged `summoned` and every keyed-stream spawn (§5.5) spawn with an empty `carry` and draw nothing here. Each entry draws exactly two values whatever the outcome: one `roll_bp(chance_bp)` and one `weighted` over the doc 11 list of common items of that category in the local genre (an entry naming one item ID uses `weighted` over a one-element list). Carried items are real items: `rar.common`, `q.normal`, `buc.uncursed`, `ilvl = d`, uids from the Folio item counter. Unused ones drop on the corpse's cell at death as shared world items (first pickup wins), in addition to the instanced loot roll (§9.1, doc 11).

- **Potions.** `ai.action.use_item` drinks a healing potion over 45 ticks (interruptible, §4.6). The heal follows the doc 08 potion definition in Bp of the drinker's Max HP. An Erratum that sees the drink learns the potion type to `know.classified` (monster-use identification, doc 10).
- **Throwables.** `ai.action.throw` lobs a `cat.throwable` with a 40-tick windup, a `circle` telegraph at the predicted landing cell and a grenade token. Damage and fuse come from doc 05 with `src_lvl = d`.
- **Ammo.** A shooter or sniper with an ammo entry has a magazine of 30 or 5 rounds; `reload` takes 90 ticks. Agents without ammo entries never reload.

### 5.2 Spells and abilities

Caster, controller, support and summoner templates list `spells: [spell.*]` from doc 06 in addition to innate attacks. An NPC spell is an enemy source (doc 06 §6.5): `B = mul_bp(mul_bp(mul_bp(spell.dmg_base_mhp, NPC_SPELL_SCALE_BP), T(d)), rank.dmg_bp)` with `NPC_SPELL_SCALE_BP = 4500`, so a Folio-1 standard caster's 24,000 mHP flame bolt hits for 10,800 mHP, the Canon §17.4 `REF_ENEMY_HIT_D1`. Canon §6.4 row 29 never applies to NPCs (step 2.1 is 10000). Casting spends template aether (cast cost = the spell's `base_cost`), rolls Contradiction failure for the spell's base component with keyed `rng.combat` (a failure fizzles and spends full aether, doc 06), and rolls cast interruption (row 30) with keyed `rng.magic` when hit during `Windup` or `Channel`. Spell choice uses doc 06 §18's pure queries inside the `ai.action.ability` considerations: a harmful area spell is a candidate only when `ally_risk = 0` or the template is `reckless`, and `interrupter` templates add `cast_state(target).ticks_remaining >= 12` → `binary` as a consideration.

### 5.3 Picking up weapons

Humanoid sapient templates with `weapon_pickup` and an intact `bp.arm_r` pick up `cat.weapon_melee` or `cat.weapon_ranged` items lying within 8000 mm.

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

`enemy.ab.summon` creates entities with role swarmer, rank minion and tag `summoned`: the summoner's own faction's swarmer entries in its origin genre's roster, or, when that faction has none, that roster's swarmer entries of any faction. The summon takes the summoner's faction whatever its template's faction. The template is `weighted` over those entries' W (one draw), and each summon's cell is `cells[range(n)]` over the walkable cells 1000 to 3000 mm from the summoner in ascending cell index (one `range` per summon), both on keyed `rng.ai` seq 1. Summons join the summoner's squad, grant 0 kill XP, despawn 60 ticks after their summoner dies and after 1800 ticks of existence, count toward `ACTIVE_ACTOR_CAP`, and are capped at 4 per summoner and 24 hostile summons per Folio. Elite and boss summons follow their own entries in §7 and §9.

**Hostile doc 06 summons.** A prayer that angers a patron can spawn a hostile `summon.herald`, and a wild surge a hostile `summon.misprint` (doc 06 §12.2, §16.3). This chapter runs them as enemies with doc 06's stats (HP `template × H(d) × rank HP × party HP`, damage `template × T(d) × rank damage`): the herald at `rank.elite` in faction `fac.wild.<local genre suffix>` with `ai.set.guardian` and the `flier` tag; the misprint at `rank.standard` in `fac.editors` with `ai.set.hunter` but without hunter omniscience (it perceives through doc 09), tag `editor`, empty `dep` and §10.6 block erasure. Both are XP-eligible (they are not tagged `summoned`, because no hostile caster owns them) and drop `loot.drop.<rank>` (doc 11 §9.5).

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

This section is the allied-AI contract for Canon §15.1. Doc 06 §12.1 owns spell-summon stats, duration and cap replacement; doc 04 owns the Tame check and the Handling and Leadership perks.

**Kinds.** `summon` (doc 06 `summon.*`), `tamed` (an enemy template bound by Tame), `hireling` (§13.4) and `companion` (§13.5). Every minion is in `fac.errata`, carries `owner_slot` and has `st.bound` while bound. Binding emits `ev.minion_bound`; release, death or despawn emits `ev.minion_released`. Hacked constructs (§5.7) are not minions.

**Caps.** `MINION_CAP_PER_PLAYER = min(5, 1 + floor_div(max(skill.conjuration, skill.handling), 25))` and `FOLIO_MINION_CAP = 16`, both inside `ACTIVE_ACTOR_CAP` (Canon §15.1). A spell summon at the cap releases the owner's lowest-EntityId spell summon first (doc 06 §12.1). Every other bind at a cap fails and emits nothing.

**Taming** is deterministic (doc 04 §4.2: taming is not a random check). An Erratum starts a 120-tick channel (`input.interact` held) on a creature tagged `beast`, not `summoned`, of rank minion or standard (elite only with Handling ≥ 75), while the creature has `st.sleeping` or `st.charmed`, or has HP below 3000 Bp and is not in `Windup` or `Active`. Damage to either cancels the channel. At its end the tame succeeds when

```
tame_value = skill.handling + floor_div(CHM_eff - 40, 2) + bonuses      // doc 04: courtesy +10, beast_tongue +15
tame_dc    = tier + floor_div(d, 2)     // tier: minion 10, standard 20, elite 50 (standard + 30, doc 04 Handling 75)
success    = tame_value >= tame_dc      // no draw
```

Success pacifies the creature (doc 04 §6.1 pays its kill XP once; `ev.entity_pacified`), grants Handling `floor_div(3000 × rank_xp_bp, 10000)` milli-XP (doc 04 §5.2) and binds it if the owner's cap allows; at the cap the pacified beast walks off as `fac.neutral` instead. Failure leaves the creature hostile and bars that Erratum from retrying on it until its `tame_value` rises. Example: a standard beast in Folio 12 has `tame_dc = 20 + 6 = 26`; CHM 50 and Handling 30 give `tame_value = 30 + 5 = 35`, so the tame succeeds; a starting Beastbinder (Handling 20, CHM 50, value 25) tames standard beasts through Folio 11 (`dc = 20 + 5 = 25`) and needs Handling growth beyond.

**Stats.**

```
summon:            doc 06 §12.1 (magnitudes × P(src_lvl) × spell potency)
tamed, hireling:   hp_max = mul_bp(mul_bp(tpl.hp_mhp, T(d)), 15000)        then doc 04 Leadership and Handling factors
                   B      = mul_bp(mul_bp(tpl_atk_mhp, P(d)), 10000)
                   AR     = mul_bp(tpl.ar, T(d));  subs = tpl + floor_div(3 × (d - 1), 2), capped at 150
```

Ranks, party-size tables and doc 14 difficulty chains never apply to minions. Tamed beasts, hirelings and companions re-evaluate on `ev.folio_entered`, keeping their HP ratio (Canon §12 max-change rule). Example: a tamed troll (110 HP, 18 damage, AR 15) in Folio 12 has 297,330 mHP, hits for 46,447 mHP and has AR 27.

**Profiles.** Every minion runs `ai.set.minion` with one doc 06 profile: `support`, `artillery` (keeps 6000 to 8000 mm from its target), `guardian` (stays within 4000 mm of its owner), `skirmisher`, `healer`, `decoy` or `bodyguard` (stays between its owner and the owner's nearest attacker). Tamed beasts are skirmishers. Hirelings and companions take the profile of their role: guardian → bodyguard; sniper, caster and artillery → artillery; support → healer; every other role → skirmisher.

**Targeting** (doc 06 §12.1): the `input.minion_attack` target, which is the entity under the owner's crosshair on a 48000 mm ray, until it dies or 600 ticks pass; otherwise the most recent attacker of the owner within 12000 mm; otherwise the nearest hostile within 12000 mm of the minion (minimum `dist2`, ties to the lower EntityId). Minion fire lanes treat every `fac.errata` capsule as blocking.

**Commands and leash.** `input.minion_follow` (default) keeps slot `k` (0-based bind order) 3000 mm behind the owner at bearing `owner.yaw + 32768 + (k - 2) × 5461` Bam. `input.minion_hold` holds the commanded cell. A minion more than 16000 mm from its owner drops its target and returns (doc 06 leash). A minion more than 32000 mm of path from its owner with no target for 120 ticks teleports to a free walkable cell within 3000 mm behind the owner that no alerted hostile can see, never into or out of a sealed arena unless the owner is inside.

**Owner states.** Owner downed: `guard_owner` (stay within 4000 mm, attack the nearest threat to the owner). Owner a ghost: summons despawn; tamed beasts, hirelings and companions hold at the Remains, follow any Erratum carrying them, and rebind when the ghost resurrects (Canon §17.6).

**Folio transitions, death and XP.** Spell summons despawn on `ev.folio_exited`. Tamed beasts, hirelings and companions within 16000 mm of the exit follow, persist for the run and are lost permanently on death. A minion's kill grants kill XP as if its owner made it (Canon §15.1). Friendly fire follows Canon §17.5.

## 7. Ranks, elite modifiers and champions

### 7.1 Ranks

Ranks and their multipliers are Canon §17.3. The Director's population plan (§10.1) fixes the count of each rank per Folio. Elite and champion entities are standard roster templates promoted by the Director; swarmers become elites but never champions. Bosses, Editors and hunters use their own templates.

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
| `elite.voltaic` | no | each hit adds 40 `st.shocked` power; on death direct `st.shocked` to every entity within 3000 mm of its origin, its own allies included, in ascending EntityId | 1 | — |
| `elite.thorned` | no | each melee hit it takes reflects `min(mul_bp(hp_loss, 2000), mul_bp(B1, 2500))` mHP to the attacker as a `dmg.slash` thorns hit (Canon §10.6, entering at step 3), where `hp_loss` is the HP the hit removed and `B1` is the elite's own ×1 basic-attack `B` (§2.2) | 1 | — |
| `elite.hasty` | no | move speed × 13000; `cycle`, windup and recovery × 8000, rounded up with `ceil_div`; a windup never drops below the §1 rule 3 floor of its attack (30 ticks at `dmg_mult_bp ≥ 20000`, 45 at ≥ 40000) | 1 | — |
| `elite.armored` | no | every part's AR × 25000; immune to `st.corroded` buildup; countered by `counter.sunder` (doc 05 §15.1: 5 direct `st.corroded` stacks remove the modifier for 300 ticks) | 1 | swarmer |
| `elite.shrouded` | no | gains `st.invisible` after 180 ticks without attacking or taking damage | 1 | stalker |
| `elite.brood` | no | every 600 ticks spawns 2 rank-minion swarmers of its roster, tag `summoned`, at most 4 alive | 1 | summoner, swarmer |
| `elite.splitting` | no | on death spawns 2 copies at rank minion, tag `summoned`, without modifiers | 6 | swarmer |
| `elite.mirrored` | no | the first projectile hit it takes in each 240-tick window deals it nothing and instead reflects to the projectile's owner as a Canon §10.6 reflected hit with the projectile's shares and `D2 = min(that hit's D2, mul_bp(B1, 15000))`, no crit | 6 | — |
| `elite.phasing` | no | every 240 ticks becomes intangible for 60 ticks (no capsule tests) and blinks up to 6000 mm to the walkable cell nearest the point 3000 mm behind its target (ties lower cell index, no draw) after a 30-tick `marker` telegraph; countered by `counter.anchor` (doc 05 §15.1: no phasing for 300 ticks) | 6 | cavalry |
| `elite.redlined` | no | each hit pours 1000 ml of `liq.red_ink` on the target's feet cell (Canon §12.1), at most once per 600 ticks | 6 | — |
| `elite.miasmic` | no | immune to `st.poisoned`; emits 1000 ml of `gas.miasma` into its cell every 120 ticks | 6 | — |
| `elite.rallying` | no | every 300 ticks, allies of its faction within 12000 mm gain direct `st.empowered` and +1500 morale | 11 | — |
| `elite.gravitic` | no | every 360 ticks, after a 45-tick `ring` telegraph, pulls every Erratum within 10000 mm 3000 mm toward itself over 20 ticks | 11 | — |
| `elite.vengeful` | no | on death, after a 90-tick `circle` telegraph, a 4000 mm `dmg.arcane` blast with `dmg_mult_bp` 30000 | 11 | — |

**Reflection caps.** Uncapped reflection scales with the Errata's item power `P(d)` and `ITEM_MULT_MAX`, while Errata HP scales far slower, so a 20% thorn or a full mirror would let a Folio-25 Erratum kill itself with its own charged shot. The caps tie reflection to the elite's own threat `T(d)` instead. Example: a Folio-25 elite brute has `B1 = mul_bp(mul_bp(18000, 36145), 13000) = 84,579` mHP. A 500,000 mHP melee hit on a thorned one reflects `min(100,000, 21,144) = 21,144` mHP; a 900,000 mHP charged shot on a mirrored one returns `min(900,000, 126,868) = 126,868` mHP before the shooter's own mitigation.

There are 20 elite modifiers: the 3 Canon champion aspects and 17 non-aspect modifiers.

**Selection.** When the Director promotes a template, it draws exactly one `weighted` call from `rng.spawn` over the non-aspect modifiers in ascending ID byte order, weight 1000 each, weight 0 when the Folio is below the minimum or the role is excluded. A champion from Folio 11 makes this draw after its aspect assignment. Independently, every standard spawn draws one `roll_bp(elite_promote_add_bp)` on `rng.spawn` (doc 14 Grade parameter, 0 at Grade 0; the draw happens at every Grade) and becomes an elite on success, outside the budget.

### 7.3 Champions and counters

This chapter owns a Folio's **aspect set**. It is the first draw of the Folio's `rng.spawn`, made at Folio entry before any population draw, so doc 12's exit preview (doc 12 §5.7) recomputes it from a temporary copy of Folio `n + 1`'s stream without generating that Folio. `rng.spawn`'s seed `stream_seed(run_seed, "rng.spawn", n)` does not depend on the genre, so each candidate door's preview needs only that door's genre and depth.

```rust
fn aspect_set(run_seed: u64, n: u32, g: GenreId, d: u32, marg_champions: bool) -> SmallVec<[Id32; 3]> {
    let mut r = Rng::from_seed64(stream_seed(run_seed, "rng.spawn", n as u64));   // the live stream starts identically
    let mut k = if d <= 10 { 1 } else if d <= 30 { 2 } else { 3 };
    if marg_champions { k = min(3, k + 1); }                                     // lattice.marg.champions (doc 12 §22)
    let ax = genre_axioms(g);                                                    // Canon §5.2 [ARC, TEC, MYT, VRV]
    let pool = [ELITE_SURGING, ELITE_UNSTOPPABLE, ELITE_WARDED];                  // ascending ID byte order
    let mut w: [u32; 3] = [1000 + 100 * ax.myt, 1000 + 100 * ax.vrv, 1000 + 100 * max(ax.arc, ax.tec)];
    let mut out = SmallVec::new();
    for _ in 0..k { let i = r.weighted(&w); out.push(pool[i]); w[i] = 0; }      // exactly k weighted calls
    out
}
```

Undead-, fiend- and cult-heavy high-MYTHOS genres lean to regenerating surging champions, cinematic high-VERVE genres to unstoppable ones and high-ARCANA or high-TECHNE genres to warded ones. Champion `i` of the Folio (0-based, ascending EntityId at spawn) carries aspect `set[i % len(set)]`; a Keeper champion carries `set[0]`; the doc 12 duel challenge's champion and `law.mythic_antiquity.hubris`'s Nemesis carry the whole set. Worked example: run seed `0x0123456789ABCDEF`, Folio 12, Mythic Fantasy (ARC 8, TEC 2, MYT 6, VRV 6): `stream_seed = 0x546718c15ec8e4ab`, `k = 2`, weights `[1600, 1600, 1800]`; the first call is `range(5000) = 3382`, which passes 1600 and 1600 and lands in warded; the second, over `[1600, 1600, 0]`, is `range(3200) = 722`, surging. The set is `[elite.warded, elite.surging]`, and champions 0, 1 and 2 carry warded, surging and warded.

| Aspect | Counter (doc 05 `counter.*`) | What the counter does to this champion |
|---|---|---|
| `elite.warded` | `counter.ward_breaker` | × 30000 against its ward; breaking the ward applies direct `st.stunned` |
| `elite.surging` | `counter.disruptor` | direct `st.unraveled` and `st.silenced` for 180 ticks; stops its 2% Max HP per second regeneration |
| `elite.unstoppable` | `counter.staggerer` | poise damage × 30000 even though it is immune to poise stagger |

`counter.sunder` and `counter.anchor` (doc 05 §15.1) answer the non-aspect `elite.armored` and `elite.phasing`. Every genre's shops and loot pools carry a dependency-none source of each counter type (Canon §17.3a, doc 05, doc 11). Champion AI: a surging champion with `st.unraveled` or `st.shocked` adds `ai.action.retreat` weight × 15000 until the status ends; a warded champion whose ward broke uses `take_cover` at weight × 15000 for 180 ticks.

## 8. Genre rosters

Each genre lists its factions and at least 6 enemies. Column conventions: **Name** is the suffix of `enemy.<genre_suffix>.<name>`. **Faction** is the `<group>` of `fac.native.<genre_suffix>.<group>`, or `wild` for `fac.wild.<genre_suffix>`. **Signature** is the suffix of `enemy.ab.<name>` and its effect, in addition to the role defaults of §3. **Dep** is the axiom dependency set of the basic attack and the signature (Canon §5.3). **Loot** is the template's doc 11 §9.3 `loot_role`: its kill drops roll doc 11's `loot.drop.<rank>` table (§9.5) with that role's category bias, and the `beast` role rolls `loot.trophy`. **W** is the Director weight inside its faction (§10.2). Every roster has a swarmer, which is the fallback pool for `enemy.ab.summon` and `elite.brood` (§5.5); Editor squads (§8.15) join every genre's room-faction draw (§10.2). A `fac.native` faction holds at least one `sapient` template or a vendor; packs of beasts and mindless monsters with no sapient member are `fac.wild` (Canon §15).

### 8.1 Mythic Fantasy

Factions: `orc_warband`, `kingsguard`, `mage_conclave`, wild.

| Name | Role | Body | Kin; tags | Faction | Signature | Dep | Loot | W |
|---|---|---|---|---|---|---|---|---|
| `orc_raider` | swarmer | humanoid | beastkin; sapient | orc_warband | `gang_up`: +2000 Bp step-1 bonus per other orc within 3000 mm of the target, at most +6000 | — | skirmisher | 2000 |
| `orc_sapper` | sapper | humanoid | beastkin; sapient | orc_warband | `powder_keg`: demo charge with shares blunt 5000, fire 5000 and `st.burning` power 60 | — | brute | 1000 |
| `kingsguard_knight` | guardian | humanoid | human; sapient | kingsguard | `shield_wall`: `block_stability_bp` 9000 while 2 or more kingsguard stand within 3000 mm | — | brute | 1500 |
| `conclave_battlemage` | caster | humanoid | human; sapient | mage_conclave | `firebolt`: `dmg.fire` 10000, `st.burning` power 50, ignites flammable blocks | arcana | caster | 1500 |
| `cave_troll` | brute | humanoid | —; beast | wild | `troll_regen`: regenerates 300 Bp of Max HP per second unless fire or acid damaged it within 300 ticks | — | beast | 1500 |
| `wyvern` | flier | avian | —; beast, flier | wild | `venom_stinger`: dive with `st.poisoned` power 80 | — | beast | 1000 |
| `kingsguard_lancer` | cavalry | humanoid on runner (warhorse) | human; sapient, mounted | kingsguard | `lance_charge` (§3) | — | marksman | 1500 |

### 8.2 Gothic Horror

Factions: `night_court`, `inquisition`, `hollow_village`, wild.

| Name | Role | Body | Kin; tags | Faction | Signature | Dep | Loot | W |
|---|---|---|---|---|---|---|---|---|
| `rat_swarm` | swarmer | swarm | —; beast, swarm, mindless | wild | `gnaw`: `st.bleeding` power 30 per hit | — | swarm | 2000 |
| `ghoul` | skirmisher | humanoid | undead; mindless | night_court | `corpse_feast`: `eat_corpse` heals 2500 Bp of Max HP | mythos | skirmisher | 2000 |
| `vampire_courtier` | stalker | humanoid | undead; sapient | night_court | `mist_step`: `vanish` plus a 6000 mm teleport; heals 1000 Bp of HP damage dealt | mythos | skirmisher | 1000 |
| `witch_hunter` | sniper | humanoid | human; sapient | inquisition | `blessed_bolt`: crossbow, shares pierce 7000, radiant 3000 | mythos | marksman | 1500 |
| `torch_villager` | soldier (melee) | humanoid | human; sapient | hollow_village | `torch_and_fork`: melee with `st.burning` power 40; ignites flammable blocks | — | marksman | 1500 |
| `werewolf` | brute | humanoid | beastkin; beast | wild | `moonfrenzy`: below 5000 Bp HP, direct `st.berserk` for 6 s, once | mythos | beast | 1000 |
| `flesh_golem` | guardian | humanoid | construct; mindless | night_court | `galvanic_heart`: `dmg.shock` taken heals 5000 Bp of its amount instead | arcana | construct | 1000 |

### 8.3 Cosmic Horror

Factions: `esoteric_order`, `deep_ones`, `expedition`, wild.

| Name | Role | Body | Kin; tags | Faction | Signature | Dep | Loot | W |
|---|---|---|---|---|---|---|---|---|
| `polyp_swarm` | swarmer | swarm | —; eldritch, swarm, mindless | wild | `wing_dust`: `st.blinded` power 30 per hit | mythos | swarm | 2000 |
| `cultist` | caster | humanoid | human; sapient | esoteric_order | `unspeakable_chant`: 180-tick channel; 20 `st.dread` power per status pulse to Errata within 6000 mm | mythos | caster | 2000 |
| `deep_one` | brute | humanoid | mutant (gills); eldritch, swimmer, sapient | deep_ones | `drag_below`: grab with direct `st.grappled`, dragging toward the nearest liquid cell within 8000 mm | mythos | brute | 1500 |
| `shoggoth_spawn` | summoner | amorphous | —; eldritch, mindless | wild | `proto_bud`: `summon` of 2 `polyp_swarm`; `resist[dmg.slash]` +3000 | mythos | caster | 1000 |
| `expedition_rifleman` | sniper | humanoid | human; sapient | expedition | `steady_aim`: after 120 ticks without moving, crit chance +2000 Bp | techne | marksman | 1000 |
| `star_seer` | controller | humanoid | mutant (extra_arm); eldritch, sapient | esoteric_order | `gaze_of_unmaking`: `st.confused` power 70 and composure −5 per hit | mythos | caster | 1500 |
| `lurker_below` | stalker | serpentine | —; eldritch, swimmer | wild | `undertow`: attacks from liquid cells; `st.knocked_down` direct on the first hit | mythos | skirmisher | 1000 |

### 8.4 Space Opera

Factions: `imperial_navy`, `void_pirates`, `psionic_order`, wild (the xeno hive).

| Name | Role | Body | Kin; tags | Faction | Signature | Dep | Loot | W |
|---|---|---|---|---|---|---|---|---|
| `navy_trooper` | soldier | humanoid | human; sapient | imperial_navy | `blaster_volley` (§2.1) | techne | marksman | 2000 |
| `psi_adept` | caster | humanoid | human; sapient | psionic_order | `psi_lance`: `beam`, `dmg.psychic` 10000, composure −5 per hit | arcana | caster | 1000 |
| `pirate_boarder` | skirmisher | humanoid | mutant (hide); sapient | void_pirates | `breach_jet`: 8000 mm jetpack leap; breaks one door, destructible-tagged or Errata-placed block on landing | techne | skirmisher | 1500 |
| `hive_drone` | swarmer | insectoid | —; beast, mindless | wild | `acid_spit`: projectile `dmg.acid`, `st.corroded` power 40 | — | swarm | 2000 |
| `hive_warrior` | brute | insectoid | —; beast | wild | `carapace_charge`: `charge` with AR × 15000 during the charge | — | beast | 1500 |
| `security_droid` | guardian | construct | —; construct, electronic | imperial_navy | `energy_shield`: frontal `block_stability_bp` 8000 that also blocks projectiles | techne | construct | 1000 |
| `hover_lancer` | cavalry | humanoid on hover (speeder) | human; sapient, mounted | void_pirates | `strafing_run`: passes the target at 14000 mm/s firing a 4-round burst | techne | marksman | 1000 |

### 8.5 Cyberpunk

Factions: `zaibatsu_security`, `chrome_gang`, `netrunner_collective`, wild.

| Name | Role | Body | Kin; tags | Faction | Signature | Dep | Loot | W |
|---|---|---|---|---|---|---|---|---|
| `street_punk` | swarmer | humanoid | human; sapient | chrome_gang | `switchblade_rush`: `leap` plus `st.bleeding` power 30 | — | skirmisher | 2000 |
| `corp_enforcer` | soldier | humanoid | human; sapient | zaibatsu_security | `smart_rounds`: projectiles home with 2048 Bam per tick of turn | techne | marksman | 2000 |
| `chrome_brawler` | brute | humanoid | construct; sapient, electronic | chrome_gang | `reflex_overclock`: direct `st.hasted` 4 s, cooldown 900 | techne | brute | 1500 |
| `netrunner` | controller | humanoid | human; sapient | netrunner_collective | `short_circuit`: through walls within 20000 mm; targets the nearest Erratum with any `cat.implant` (60 `st.shocked` power), else disables the nearest Errata gadget for 300 ticks | techne | caster | 1000 |
| `mantis_assassin` | stalker | humanoid | construct; sapient, electronic | chrome_gang | `optical_camo`: `vanish` with cooldown 450 | techne | skirmisher | 1000 |
| `security_drone` | flier | construct | —; construct, electronic, flier | zaibatsu_security | `taser_dart`: projectile `dmg.shock`, 40 `st.shocked` power | techne | construct | 1500 |
| `riot_mech` | artillery | construct | —; construct, electronic | zaibatsu_security | `microsalvo`: `barrage` of 6 lobs | techne | construct | 1000 |

### 8.6 Post-Apocalypse

Factions: `road_raiders`, `vault_remnant`, `glow_cult`, wild.

| Name | Role | Body | Kin; tags | Faction | Signature | Dep | Loot | W |
|---|---|---|---|---|---|---|---|---|
| `rad_roach` | swarmer | insectoid | —; beast, mindless | wild | `rad_bite`: `st.irradiated` power 30 | — | swarm | 2000 |
| `raider_scav` | soldier | humanoid | human; sapient | road_raiders | `pipe_rifle`: 1500 Bp jam chance on each shot, rolled on keyed `rng.combat` | techne | marksman | 2000 |
| `war_rig_rider` | cavalry | humanoid on runner (motorbike) | human; sapient, mounted | road_raiders | `ram`: charge × 15000 and knockback 4000 mm | techne | marksman | 1000 |
| `glow_zealot` | artillery | humanoid | mutant (regrowth); sapient | glow_cult | `rad_burst`: lob with `dmg.radiation` 10000 and `st.irradiated` power 60 | mythos | marksman | 1000 |
| `bunker_sentry` | guardian | construct | —; construct, electronic | vault_remnant | `arc_shield`: frontal `block_stability_bp` 8000; blocked hits chain 2000 Bp `dmg.shock` to the attacker | techne | construct | 1000 |
| `mutant_behemoth` | brute | humanoid | mutant (hide); beast | wild | `boulder_toss`: hurls a rubble boulder, × 20000 `dmg.blunt`, windup 45 | — | beast | 1500 |
| `tunnel_mole` | sapper | humanoid | mutant (regrowth); burrowing, sapient | vault_remnant | `burrow`: digs destructible-tagged blocks at `dig_speed_bp` 30000 and surfaces under the target | — | brute | 500 |

### 8.7 Weird West

Factions: `outlaw_gang`, `lawmen`, `railroad_company`, `hex_coven`, wild.

| Name | Role | Body | Kin; tags | Faction | Signature | Dep | Loot | W |
|---|---|---|---|---|---|---|---|---|
| `dead_miner` | swarmer | humanoid | undead; mindless | wild | `dynamite_belt`: on death a 2500 mm `dmg.fire` blast of base 12 HP after a 60-tick `circle` telegraph | — | skirmisher | 2000 |
| `outlaw_gunhand` | shooter | humanoid | human; sapient | outlaw_gang | `fan_the_hammer`: 6-round burst 4 ticks apart, × 3000 each | techne | marksman | 2000 |
| `marshal_rifleman` | sniper | humanoid | human; sapient | lawmen | `high_noon`: a 90-tick standoff telegraph; if the target is visible at its end, the shot is a guaranteed crit at × 15000 | verve | marksman | 1000 |
| `hexslinger` | caster | humanoid | human; sapient | hex_coven | `hex_bullet`: `dmg.umbral` 10000, `st.withering` power 60 | mythos | caster | 1500 |
| `railroad_automaton` | brute | construct | —; construct, electronic | railroad_company | `steam_ram`: `charge` that leaves 1000 ml of `gas.steam` in each of the last 3 cells of its charge path | techne | construct | 1500 |
| `sand_wyrm` | stalker | serpentine | —; beast, burrowing | wild | `sand_swim`: moves through `block.sand` and `block.dirt` cells as if passable (no block breaks) and surfaces under its target with direct `st.knocked_down`, cooldown 600 | — | beast | 500 |
| `posse_rider` | cavalry | humanoid on runner (horse) | human; sapient, mounted | lawmen | `lasso`: 12000 mm projectile, direct `st.grappled` and a 4000 mm pull | — | marksman | 1500 |

### 8.8 Hardboiled Noir

Factions: `syndicate`, `city_police`, `dockworkers`, wild.

| Name | Role | Body | Kin; tags | Faction | Signature | Dep | Loot | W |
|---|---|---|---|---|---|---|---|---|
| `street_tough` | swarmer | humanoid | human; sapient | syndicate | `brass_knuckles`: 20 `st.stunned` power per hit | — | skirmisher | 2000 |
| `mob_torpedo` | shooter | humanoid | human; sapient | syndicate | `drum_burst`: 8-round burst 6 ticks apart, × 2000 each | techne | marksman | 2000 |
| `leg_breaker` | brute | humanoid | human; sapient | syndicate | `kneecap`: hits on leg parts apply direct `st.slowed` 4 s | — | brute | 1500 |
| `crooked_cop` | soldier | humanoid | human; sapient | city_police | `call_backup`: once per Folio, `call_allies` that adds 2 standard `crooked_cop` at the nearest spawn anchor out of sight | techne | marksman | 1500 |
| `shadow_tail` | stalker | humanoid | human; sapient | syndicate | `garrote`: from behind, direct `st.grappled` 3 s and `st.bleeding` power 60 | — | skirmisher | 1000 |
| `dock_heavy` | guardian | humanoid | human; sapient | dockworkers | `crate_shield`: carries a crate as a frontal shield with `block_stability_bp` 8000; the crate breaks once the damage it has blocked totals 3 × the dock heavy's Max HP | — | brute | 1000 |
| `getaway_driver` | cavalry | humanoid on runner (sedan) | human; sapient, mounted | syndicate | `drive_by`: `strafing_run` with a 3-round burst | techne | marksman | 1000 |

### 8.9 Wuxia

Factions: `iron_palm_sect`, `jade_court_guard`, `lotus_assassins`, wild.

| Name | Role | Body | Kin; tags | Faction | Signature | Dep | Loot | W |
|---|---|---|---|---|---|---|---|---|
| `jiangshi` | swarmer | humanoid | undead; mindless | wild | `breath_sense`: senses `breath` only, no sight; a creature holding its breath (doc 09) is undetectable to it | mythos | skirmisher | 2000 |
| `sect_disciple` | skirmisher | humanoid | human; sapient | iron_palm_sect | `qinggong_leap`: 10000 mm leap over walls up to 3 blocks, cooldown 300 | verve | skirmisher | 2000 |
| `iron_palm_elder` | brute | humanoid | human; sapient | iron_palm_sect | `iron_palm`: × 20000 `dmg.blunt`, knockback 5000 mm, windup 36 | verve | brute | 1000 |
| `court_crossbowman` | sniper | humanoid | human; sapient | jade_court_guard | `repeating_volley`: 5 bolts 6 ticks apart, × 3000 each | — | marksman | 1500 |
| `lotus_assassin` | stalker | humanoid | human; sapient | lotus_assassins | `thousand_needles`: cone 4096 Bam, 6000 mm, `st.poisoned` power 80 | verve | skirmisher | 1000 |
| `fox_spirit` | controller | quadruped | —; fae, beast | wild | `foxfire_charm`: `st.charmed` power 60 (Errata use Canon §11.8) | arcana | beast | 1000 |
| `cavalry_general` | cavalry | humanoid on runner (horse) | human; sapient, mounted | jade_court_guard | `halberd_sweep`: arc 16384 Bam, × 15000, windup 30 | — | marksman | 1500 |

### 8.10 Steampunk

Factions: `clockwork_guild`, `sky_pirates`, `luddite_brigade`, wild.

| Name | Role | Body | Kin; tags | Faction | Signature | Dep | Loot | W |
|---|---|---|---|---|---|---|---|---|
| `clockwork_scarab` | swarmer | construct | —; construct, electronic, mindless | clockwork_guild | `wind_down`: stops for 60 ticks after every 600 ticks of activity | techne | construct | 2000 |
| `brass_automaton` | soldier | construct | —; construct, electronic | clockwork_guild | `wind_up_burst`: after 60 ticks of `reload`, its next volley is × 15000 | techne | construct | 2000 |
| `sky_aeronaut` | flier | humanoid | human; sapient, flier | sky_pirates | `grapnel_swing`: 15000 mm grapple to a solid block, then a dive attack | techne | skirmisher | 1500 |
| `galvanist` | caster | humanoid | human; sapient | clockwork_guild | `arc_coil`: `dmg.shock` 10000 that chains to 2 more targets within 4000 mm at 5000 Bp | techne | caster | 1000 |
| `luddite_wrecker` | sapper | humanoid | human; sapient | luddite_brigade | `sabot`: melee hits disable one Errata gadget, turret or drone for 600 ticks | — | brute | 1000 |
| `steam_juggernaut` | guardian | construct | —; construct, electronic | clockwork_guild | `boiler_vent`: every 300 ticks emits 1000 ml of `gas.steam` into each of the 4 orthogonally adjacent air cells at its feet level | techne | construct | 1000 |
| `sewer_gator` | brute | quadruped | —; beast, swimmer | wild | `death_roll`: grab with direct `st.grappled`, then × 25000 `dmg.slash` after 60 ticks unless the grab breaks | — | beast | 1500 |

### 8.11 Fairy Tale

Factions: `seelie_court`, `unseelie_court`, `woodcutters`, wild.

| Name | Role | Body | Kin; tags | Faction | Signature | Dep | Loot | W |
|---|---|---|---|---|---|---|---|---|
| `pixie_swarm` | swarmer | swarm | —; fae, swarm, flier | seelie_court | `glamour_dust`: `st.confused` power 30 per hit | arcana | swarm | 2000 |
| `big_bad_wolf` | skirmisher | quadruped | —; beast, sapient | unseelie_court | `huff_and_puff`: cone 5461 Bam, 8000 mm, knockback 4000 mm, breaks destructible-tagged and Errata-placed blocks in the cone, windup 45 | arcana | beast | 1500 |
| `bridge_troll` | brute | humanoid | fae; sapient | unseelie_court | `stone_at_dawn`: takes × 15000 damage while `light_at ≥ 12`; heals 2000 Bp of HP damage dealt to `st.bleeding` targets | mythos | brute | 1500 |
| `hedge_witch` | summoner | humanoid | human; sapient | unseelie_court | `gingerbread_golems`: `summon` of 2 swarmer constructs instead of the roster swarmer | arcana | caster | 1000 |
| `woodcutter` | soldier (melee) | humanoid | human; sapient | woodcutters | `cold_iron_axe`: primary material `mat.cold_iron`, so fae Errata take × 12500 (Canon §16.3) | — | marksman | 1500 |
| `thorn_knight` | guardian | humanoid | fae; sapient | seelie_court | `briar_wall`: places 3 `block.web` cells in a line in front of itself, cooldown 600 | arcana | brute | 1000 |
| `wild_hunt_rider` | cavalry | humanoid on runner (spectral steed) | fae; sapient, mounted | unseelie_court | `horn_of_the_hunt`: `area` 10000 mm, `st.feared` power 60, windup 45 | mythos | marksman | 1500 |
| `forest_wolf` | swarmer | quadruped | —; beast | wild | `pack_tactics`: +1500 Bp step-1 bonus per other `forest_wolf` within 4000 mm of its target, at most +4500; senses include `scent` | — | beast | 1500 |

### 8.12 Mythic Antiquity

Factions: `hoplite_league`, `maenad_thiasos`, `temple_oracles`, wild.

| Name | Role | Body | Kin; tags | Faction | Signature | Dep | Loot | W |
|---|---|---|---|---|---|---|---|---|
| `maenad` | swarmer | humanoid | human; sapient | maenad_thiasos | `frenzy`: direct `st.berserk` when an ally dies within 8000 mm | mythos | skirmisher | 2000 |
| `hoplite` | guardian | humanoid | human; sapient | hoplite_league | `phalanx`: +5 AR on every part per other hoplite within 2000 mm, at most +15 | — | brute | 2000 |
| `shade_warrior` | soldier (melee) | humanoid | undead; mindless | hoplite_league | `reassemble`: once, 300 ticks after death, rises at 5000 Bp HP unless the killing hit's largest share was `dmg.blunt` or `dmg.radiant`; XP and loot are paid on the final death only | mythos | marksman | 1500 |
| `harpy` | flier | avian | —; beast, flier | wild | `snatch`: grab and carry a target 3 blocks up for 60 ticks, then drop it | — | beast | 1500 |
| `oracle_priestess` | support | humanoid | human; sapient | temple_oracles | `foresight`: direct `st.blessed` on one ally; that ally sidesteps the next telegraphed attack | mythos | support | 1000 |
| `bronze_bull` | brute | quadruped | —; construct | wild | `fire_breath`: cone 5461 Bam, 6000 mm, `dmg.fire` × 3000 per 10 ticks for 60 ticks | mythos | construct | 1000 |
| `centaur_archer` | sniper | quadruped | —; sapient | maenad_thiasos | `parthian_shot`: fires while retreating at run speed | — | marksman | 1000 |

### 8.13 Oneiric (secret)

Factions: `lucid_dreamers`, `nightmare_court`, wild.

| Name | Role | Body | Kin; tags | Faction | Signature | Dep | Loot | W |
|---|---|---|---|---|---|---|---|---|
| `sleepwalker` | swarmer | humanoid | human; mindless | wild | `somnambulance`: immune to `st.sleeping`; hits add 30 `st.sleeping` power | arcana | swarm | 2500 |
| `nightmare_mare` | skirmisher | quadruped | —; fae, mindless | nightmare_court | `dread_trample`: charge with 50 `st.dread` power | mythos | beast | 2000 |
| `stair_mimic` | stalker | amorphous | —; eldritch | wild | `block_mimicry`: while idle it is drawn as a stair block and is not a sight target | arcana | skirmisher | 1500 |
| `faceless_teacher` | controller | humanoid | mutant (extra_arm); sapient | nightmare_court | `pop_quiz`: direct `st.silenced` and `st.confused`, windup 45, token control | arcana | caster | 1500 |
| `gravity_kite` | flier | avian | —; fae, flier | lucid_dreamers | `gravity_flip`: direct `st.levitating` 60 ticks on the target, then a 3000 mm slam | verve | beast | 1500 |
| `lullaby_choir` | support | humanoid | human; sapient | lucid_dreamers | `lullaby`: allies within 10000 mm regenerate 200 Bp of Max HP per second while it channels | arcana | support | 1000 |

### 8.14 The Redaction (secret)

Factions: `bureau_of_correction`, `whistleblowers` (starting reputation +150, friendly).

| Name | Role | Body | Kin; tags | Faction | Signature | Dep | Loot | W |
|---|---|---|---|---|---|---|---|---|
| `clerk` | swarmer | humanoid | human; sapient | bureau_of_correction | `form_27b`: direct `st.marked` on hit | techne | skirmisher | 2500 |
| `compliance_officer` | soldier | humanoid | human; sapient | bureau_of_correction | `cease_and_desist`: direct `st.disarmed`, windup 40, token control | techne | marksman | 2000 |
| `blackbar_sentinel` | guardian | construct | —; construct, electronic | bureau_of_correction | `redaction_field`: cells within 3000 mm have concealment 15 | techne | construct | 1500 |
| `shredder_unit` | brute | construct | —; construct, electronic | bureau_of_correction | `shred`: `dmg.slash` with `st.bleeding` power 60; destroys ground items it walks over that are not Errata-owned | techne | construct | 1500 |
| `auditor` | sniper | humanoid | human; sapient | bureau_of_correction | `audit`: `st.exposed` power 80 | mythos | marksman | 1500 |
| `faceless_agent` | stalker | humanoid | mutant (hide); sapient | bureau_of_correction | `unperson`: `vanish`; its kills leave no Remains for 300 ticks | mythos | skirmisher | 1000 |

### 8.15 The Unwritten (secret) and Editor squads

`genre.unwritten` has no native factions; its only enemies are `fac.editors` (Canon §5.2), who count as native there. These six templates are also the Editor squads that join every genre's draw. IDs are `enemy.editors.<name>`; every attack has empty `dep`.

| Name | Role | Body | Kin; tags | Faction | Signature | Loot | W |
|---|---|---|---|---|---|---|---|
| `inkling` | swarmer | amorphous | —; editor, mindless | editors | `blot`: on death pours 1000 ml `liq.red_ink` | skirmisher | 2500 |
| `strikethrough` | skirmisher | humanoid | —; editor | editors | `strike_through`: `line` dash 8000 mm, × 15000 `dmg.slash`, windup 30 | skirmisher | 2000 |
| `marginal_scribe` | caster | humanoid | —; editor | editors | `annotation`: `dmg.arcane` 10000 and direct `st.marked` | caster | 1500 |
| `footnote` | summoner | humanoid | —; editor | editors | `ibid`: summons 2 `inkling` | caster | 1500 |
| `erratum_eater` | brute | amorphous | —; editor | editors | `consume_error`: heals 500 Bp of Max HP per Errata-placed block it erases (§10.6) | brute | 1500 |
| `red_pen_lancer` | sniper | humanoid | —; editor | editors | `correction_lance`: `beam`, × 20000 `dmg.arcane`, `pen_bp` 5000, windup 60 | marksman | 1000 |

### 8.16 The Ur-Text (secret)

Factions: `first_drafts`, `glossators`, wild.

| Name | Role | Body | Kin; tags | Faction | Signature | Dep | Loot | W |
|---|---|---|---|---|---|---|---|---|
| `stray_letters` | swarmer | swarm | —; swarm, mindless | wild | `ligature`: the third hit on one target within 120 ticks applies direct `st.slowed` | — | swarm | 2000 |
| `first_draft` | soldier | humanoid | human; sapient | first_drafts | `unfinished`: renders as the soldier of the genre visited last in this run; mechanics unchanged | — | marksman | 2000 |
| `palimpsest_beast` | brute | quadruped | —; beast | wild | `overwrite`: its basic attack's damage type cycles fire, frost, shock, acid every 300 ticks | — | beast | 1500 |
| `glossator` | support | humanoid | human; sapient | glossators | `gloss`: direct `st.fortified` on one ally, cooldown 600 | arcana | support | 1500 |
| `lettered_serpent` | skirmisher | serpentine | —; beast | wild | `bind`: direct `st.rooted` on hit, cooldown 300 | — | beast | 1000 |
| `illuminator` | caster | humanoid | human; sapient | glossators | `gilded_initial`: `beam` `dmg.radiant` 10000, `st.blinded` power 50 | mythos | caster | 1000 |
| `archetype_knight` | cavalry | humanoid on runner (archetypal steed) | human; sapient, mounted | first_drafts | `lance_charge` | verve | marksman | 1000 |

### 8.17 Cross-genre templates

| ID | Role | Body | Kin; tags | Faction | Signature | Dep | Loot | W |
|---|---|---|---|---|---|---|---|---|
| `enemy.mimic` | brute, rank elite | amorphous | —; mindless | wild of the local genre | `lid_snap`: the opening Erratum takes × 15000 `dmg.slash` and direct `st.grappled` 3 s (doc 11 §9.4 mimic Reliquary; `no_loot`) | — | — | 0 |
| `enemy.editors.revisor` | brute, rank hunter | humanoid | —; editor, breathless | editors | §10.5 | — | — | 0 |
| `enemy.editors.invader` | player-controlled, rank hunter | humanoid | —; editor, breathless | editors | §10.7 | — | — | 0 |
| `enemy.prologue_hunter` | brute, rank hunter | humanoid | —; editor, breathless | editors | `slow_erasure`: the Prologue's L7 chaser (doc 14 §11) | — | — | 0 |

Weight 0 keeps these out of roster draws; doc 11, §10.5, §10.7 and doc 14's Prologue script spawn them.

**Prologue hunter.** `enemy.prologue_hunter` exists only in `mode.prologue` room L7 (doc 14 §11). Template HP 70, damage 3, run 5000 mm/s (below the Errata sprint of 6500 mm/s, Canon §4.2, so a sprinting player always escapes), walk 4300, `immune` to every damage type and `immune_status` to every status, `no_loot`, morale-immune, omniscient like an Erasure hunter, one attack `slow_erasure` (melee, reach 2000 mm, `dmg.arcane` 10000, × 10000, windup 45 with a `line` telegraph, cooldown 120). It spawns 12000 mm behind the player on L7 entry at `aware.alert`, follows the shortest path, never erases blocks, pays no XP, and despawns on the tick the player's origin enters `room.colophon`. Solo at Folio 1 a hit deals `mul_bp(mul_bp(3000, T(1)), 20000) = 6,000` mHP before armor, so it threatens without killing a careful player. Prologue L2's three "Red Ink scrawls" are `enemy.editors.inkling` at `rank.minion`.

## 9. Bosses

### 9.1 Boss rules

Notation in this section: `w/a/r` is windup, active and recovery in ticks; `×N` is `dmg_mult_bp / 10000` applied to the boss template damage, then §2.2.

- **Templates.** Mini-bosses (`rank.boss`) have template HP 80 and damage 12. Editors (`rank.editor`) have HP 100 and damage 12; the Last Word has HP 120. Every boss has `resist` 2500 to every type, `subs` 60, poise from BRN 80 plus 60 armor poise, and the Canon §11.4 immunities. Bosses are morale-immune and need no tokens.
- **Arena.** Doc 12 builds each Editor arena from the spec here (doc 12 §21.2). A mini-boss fights in the Folio's `room.arena` (doc 12 §8.2 `room.rule.arena`, §21.2 item 5: a 48 × 48 m, two-storey box with pillars); the arena features listed in §9.3 are dressing prefabs placed inside that box, and their dimensions are the interior play space. An Editor arena seals per doc 14 §3.2 (every Active, living, non-downed Erratum inside the seal volume for `TURN_HOLD_TICKS`). A mini-boss arena seals when an Erratum enters it and unseals when the boss dies, or after 600 ticks with no living Erratum inside, in which case the boss resets to full HP and phase 1. While sealed, doors lock, nav links into the arena close, and blocks inside are unbreakable except those a boss attack creates or removes. An arena changes only through the boss's own listed attacks (`final_proof`, `abridge`, `file_wall`, `gravity_flip`, `genre_shift` and the like); phase changes edit no blocks. The Editor checkpoint room shows a boss proof (doc 17 presentation): the Editor's resistances, its verse for the Folio's genre and the counters that answer its adds.
- **Phases.** A phase ends when HP falls below its threshold. The boss then performs a 90-tick phase shift: it takes no damage, loses every status, publishes a `ring` telegraph and switches its attack set. Damage beyond the threshold in the triggering hit is discarded.
- **Telegraphed flag.** Every boss attack with a telegraph is flagged `telegraphed` (doc 14 §4.2 scoring).
- **Choice.** Bosses use the §4.5 think with `ai.set.boss`: each attack in the current phase is one action of weight 10000 with considerations `cooldown_ready` → `binary` and `target_dist`[attack range] → `plateau`; the score of the attack used last is then × 5000, so bosses rarely repeat. Boss targets follow §4.8 over every Erratum in the arena.
- **Defaults.** Where a §9.2 or §9.3 attack lists no value: windup 30 ticks (45 when its multiplier is at least ×4.0, §1 rule 3), active 6, recovery 30, cooldown 300, range 20000 mm, telegraph `circle` for area attacks and `line` for every other damaging attack, shares 100% of the listed type (`dmg.arcane` for Editors when none is listed). A summon or hazard attack with no damage still publishes a `marker` telegraph at its windup. Every multi-shot attack (`rivet_gun`, `serpent_volley`, `memo_storm`, `command_pistol`, `drum_spray`) fires its rounds 6 ticks apart, so its active window is `6 × rounds` ticks.
- **Enrage.** From 18000 ticks after sealing, the boss gains +1000 Bp in its step-1 additive bucket, and another +1000 every 1800 ticks, up to +10000.
- **Erasure.** Erasure accrual pauses in Editor Folios while the Editor lives (Canon §11.6); mini-boss Folios do not pause.
- **Genre adaptation.** The boss takes its skin from the local genre. Editors also add a **verse** chosen by the local genre's dominant axiom: the highest of ARC, TEC, MYT, VRV, ties broken in that order. Dominant axioms: ARC for Mythic Fantasy, Fairy Tale, Oneiric, Unwritten and Ur-Text; TEC for Space Opera, Cyberpunk, Post-Apocalypse, Hardboiled Noir, Steampunk and The Redaction; MYT for Gothic Horror, Cosmic Horror and Mythic Antiquity; VRV for Weird West and Wuxia.
- **Loot.** Doc 11 owns every table and draw count. A mini-boss (`rank.boss`) rolls `loot.drop.boss` once per Active, living Erratum with doc 11's `drop_rng(folio, boss EntityId, tick, slot)` (doc 11 §9.5: two item lines, floor Rare, and `item.bag.boss`). An Editor rolls `loot.drop.editor` the same way (Lucre, `item.key.illumined`, `item.bag.editor` with its 1000 Bp Palimpsestic line) and then offers the Volume reward `loot.volume_reward` (doc 11 §9.8, doc 14 §3.9). This chapter adds no boss loot of its own, so Palimpsestic items stay limited to Editor bags, Volume rewards and Palimpsest vaults (Canon §13.3).

Boss HP (`hp_max`, mHP) by party size:

| Boss | Folio | 1 | 2 | 3 | 4 | Standard hit B (×1) |
|---|---|---|---|---|---|---|
| Proofreader | 5 | 7,113,600 | 13,515,840 | 19,918,080 | 26,320,320 | 26,757 |
| Censor | 10 | 13,769,200 | 26,161,480 | 38,553,760 | 50,946,040 | 34,970 |
| Continuity Editor | 15 | 25,530,400 | 48,507,760 | 71,485,120 | 94,462,480 | 45,705 |
| Abridger | 20 | 45,965,600 | 87,334,640 | 128,703,680 | 170,072,720 | 59,736 |
| Last Word | 25 | 97,210,080 | 184,699,152 | 272,188,224 | 359,677,296 | 78,073 |
| any mini-boss | 3 | 2,148,000 | 4,081,200 | 6,014,400 | 7,947,600 | 21,369 |
| any mini-boss | 23 | 25,888,480 | 49,188,112 | 72,487,744 | 95,787,376 | 62,352 |

Example: Proofreader solo, `mul_bp(mul_bp(mul_bp(100000, 17784), 400000), 10000) = 7,113,600`.

### 9.2 The five Editors

IDs follow doc 14 §3.7: `boss.editor_v1` to `boss.editor_v4` and `boss.ur_editor`, with attacks as `<boss id>.<attack>`. Every Editor attack has empty `dep` and the listed shares. Abridged, Novella, Daily and Weekly placements and the Marginalia rotation are doc 14's (§3.1, §3.7, §9.1).

**1. The Proofreader** (`boss.editor_v1`, Folio 5). Arena: round reading room, radius 20 m, ceiling 12 m, 8 unbreakable lectern pillars (1 × 1 × 3 blocks) at radius 12 m. Phases end at 6667 and 3333 Bp.

| Attack | Phases | Spec |
|---|---|---|
| `red_mark` | 1–3 | `marker` on the highest-threat Erratum; the marker follows it for 75 ticks, then fixes; impact at tick 120: `circle` r 2500, ×2.5 `dmg.arcane`; a 6, r 30, cooldown 360 |
| `strikethrough` | 1–3 | `line` 30000 × 2000 mm through its target, w 40, a 6, r 24, ×2.0 `dmg.slash`, cooldown 300 |
| `margin_note` | 2–3 | summons 4 `enemy.editors.inkling` at the wall, w 60, cooldown 900 |
| `final_proof` | 3 | pours `liq.red_ink` in a ring at the wall that advances 1000 mm inward every 300 ticks, stopping at radius 8000 mm |

Verses: ARC fires `strikethrough` as 3 lines at −2048, 0 and +2048 Bam. TEC keeps `red_mark` tracking until tick 90. MYT replaces the inklings with 2 standard soldiers of the local roster. VRV adds `pillar_dash`: a teleport to the pillar nearest its target, cooldown 600, followed at once by `strikethrough`.

**2. The Censor** (`boss.editor_v2`, Folio 10). Its display name is not "Redactor", because Canon §17.9 calls every invading player a Redactor. Arena: archive 40 × 28 m, ceiling 8 m, 6 shelf walls 2 m thick forming 5 lanes. Phases end at 6667 and 3333 Bp.

| Attack | Phases | Spec |
|---|---|---|
| `bar_sweep` | 1–3 | a black bar 24000 × 1500 × 3000 mm crosses a lane at 4000 mm/s after a 60-tick `line` telegraph; contact ×1.5 `dmg.arcane` and direct `st.silenced`; cooldown 480 |
| `gag_order` | 1–3 | `cone` 8192 Bam, 12000 mm, w 50, a 6, r 30, ×1.2 `dmg.psychic`, direct `st.silenced`, cooldown 360 |
| `classified` | 2–3 | direct `st.invisible` 300 ticks and teleport to a lane end chosen with keyed `rng.ai` seq 1, cooldown 900 |
| `blackout` | 3 | w 60 with a `marker` on every lane; effective light in the arena is 0 for 600 ticks, and two `bar_sweep` bars cross lanes back to back, each crossing still preceded by its 60-tick `line` telegraph; cooldown 1800 |

Verses: ARC bars also apply direct `st.unraveled`. TEC bars move at 6000 mm/s. MYT bars leave a 1-cell `liq.red_ink` trail. VRV follows each `classified` with a 3-hit melee combo, each ×1.0, w 18 per hit.

**3. The Continuity Editor** (`boss.editor_v3`, Folio 15). Arena: hall 36 × 36 m with a mezzanine 5 m up and 4 ramps. It records each Erratum's cell every 60 ticks in a ring of 6. Phases end at 6667 and 3333 Bp.

| Attack | Phases | Spec |
|---|---|---|
| `crossfade` | 1–3 | 5 projectiles in a 4096 Bam fan, ×0.6 `dmg.arcane` each, w 30, cooldown 120 |
| `retcon` | 1–3 | w 90 with silhouettes at each Erratum's 300-tick-old cell; at impact every Erratum in the arena moves to that cell (nearest walkable cell if it is now blocked) and the Editor heals `min(damage taken in the last 300 ticks, 500 Bp of Max HP)`; cooldown 1200 |
| `continuity_error` | 2–3 | one hostile doppelganger per living Erratum: brute template for a melee-wielding Erratum, soldier template otherwise, rank standard, tag `summoned`, drawn with the Erratum's appearance; cooldown 1800 |
| `flashback` | 3 | repeats its last 3 attacks in order with 20-tick gaps, each with its own telegraph |

Verses: ARC doppelgangers use the caster template. TEC `retcon` windup 75. MYT doppelgangers gain tag `undead` and `enemy.ab.reassemble`. VRV `flashback` repeats 4 attacks.

**4. The Abridger** (`boss.editor_v4`, Folio 20). Arena: 5 platform segments of 12 × 12 m in a cross with 4 m gaps spanned by 2 m bridges, above a void. An Erratum falling 8 m below the platforms reappears on the center segment and takes 1500 Bp of its Max HP as `dmg.true`. Phases end at 6667 and 3333 Bp.

| Attack | Phases | Spec |
|---|---|---|
| `excerpt` | 1–3 | 3 lobbed bolts at the target's cell, ×0.8 `dmg.arcane`, radius 2000, w 30, cooldown 150 |
| `abridge` | 1–3 | `ring` on one outer segment for 180 ticks, then its blocks are removed for 1800 ticks; at most 2 segments removed at once (3 in phase 3) |
| `condense` | 1–3 | pulls every Erratum 4000 mm toward itself over 30 ticks (w 45), then a `circle` r 4000 slam, w 30, ×2.5 `dmg.blunt`; cooldown 600 |
| `summary_judgment` | 2–3 | one segment glows (keyed `rng.ai` seq 1); w 240; every Erratum not on it takes ×4.0 `dmg.arcane`; cooldown 1500 |

Verses: ARC `summary_judgment` also applies direct `st.unraveled`. TEC `abridge` windup 150. MYT a returning segment brings 2 inklings. VRV the Abridger crosses gaps with a 9000 mm/s charge, w 36, ×2.0 `dmg.blunt`.

**5. The Last Word** (`boss.ur_editor`, Folio 25, `genre.ur_text`). Arena: a 48 × 48 m ring of four quadrants, each built like one earlier Editor's arena. Phases end at 7500, 5000 and 2500 Bp. Phase 1 uses `red_mark` and `strikethrough`; phase 2 `bar_sweep` and `gag_order`; phase 3 `retcon` and `continuity_error`; phase 4 `abridge` and `summary_judgment`. Every phase also uses:

- `genre_shift`: w 180 with the genre's name shown, then the arena becomes a dynamic bleed zone (doc 12) of genre `G` for 1200 ticks. `G` is the genre among Folios 1 to 24 of this run, secret genres excluded, that maximizes the sum over living Errata of the Contradiction `C` of their equipped base-function components; ties go to the lower Canon §5.2 row. Cooldown 2400 in phases 1 and 2, 1800 in phase 3, 1200 in phase 4. Inside the zone Errata Contradiction applies again (Canon §5.5 overrides only `genre.ur_text`), while the Last Word's empty-`dep` attacks stay Coherent.

With doc 14's `final_phase_extra`, the final Editor gains a fifth phase below 1000 Bp that uses all eight earlier signature attacks and `genre_shift` with cooldown 900.

### 9.3 Genre mini-bosses

A Normal standard or challenge Folio whose layout holds a `room.arena` (doc 12 §8.2 `room.rule.arena`, one `roll_bp(3000)` per Normal Folio) holds the mini-boss of the arena's zone genre there, paid from the Folio's population budget (§10.1), except in a siege challenge, whose arena hosts the siege waves (§10.4). Genres without a mini-boss (`genre.unwritten`, `genre.ur_text`) put one champion of the zone roster in the arena instead, also paid from the budget. The arena seals as §9.1 says. Killing the mini-boss is optional; it drops `loot.drop.boss` (§9.1). Each mini-boss has two phases; phase 1 ends at 5000 Bp. IDs `boss.<genre_suffix>.<name>`.

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
| `boss.oneiric.sandman` | humanoid | giant bedroom 48 × 48 m | `dream_sand` area r6000, w45, `st.sleeping` 80; `gravity_flip` w90, arena gravity reversed for 300 ticks (doc 12 §9.5 gravity-flip volume over the arena); P2 summons 2 `nightmare_mare` every 1200 ticks |
| `boss.redacted.director_of_records` | humanoid | open-plan office 40 × 30 m | `file_wall` places a 6 × 3 block wall of cabinets removed on its death; `termination_notice` marker w90, ×3.0 arcane; P2 `memo_storm` 10 projectiles ×0.3 |

Mini-boss loot is `loot.drop.boss` (§9.1). Every mini-boss attack has the `dep` of its genre's signature verb: the wyrm, countess, herald, queen, gorgon and sandman depend on their genre's `aether_axiom`, the admiral, daemon, war boss, fixer, colossus and director on techne, the judge on mythos and the sword saint on verve; they are native in their arenas, so they are Coherent unless a bleed zone overlaps the arena.

## 10. The Director

The Director (`director.profile.standard` in standard and challenge Folios; `director.profile.boss` in Editor Folios; `director.profile.rest` in rest Folios, which spawn nothing, doc 12 §6.2, doc 14 §3.1) owns population, waves, intensity pacing and Erasure hunters. It runs as stage 2 of `sys.ai_think` on period 30, phase 11, and its population and wave draws use the host-only sequential stream `rng.spawn`.

### 10.1 Folio budget and population plan

Units are milli-SKE (mSKE, standard-kill equivalents): minion 300, standard 1000, elite 3000, champion 6000, boss 20000, editor 40000, hunter 5000, which is `rank_xp_bp / 10`.

**Inputs.** Doc 12's population interface (doc 12 §12): rooms by `NodeId` with `spawn_slots`, spawn cells, spawn anchors, patrol routes, territory factions, civic anchors, the Keeper room, the Seal point, `room.arena` and the `rescue` flag; each room's zone genre (doc 12 §14); the Folio genre's GPV `density_bp` (doc 12 §3.5, already × 12500 under `lattice.marg.dense`); the Folio type and challenge kind (doc 12 §6.1, §6.3); the position `pos`, Rubric and Keeper rank (doc 14 §3.1, §3.4); doc 14 Grade parameters; Marginalia modifiers (doc 12 §22).

**Constants.** `POP_BASE_MSKE = 29000`, `BOSS_APPROACH_BP = 6000`, `MINION_SHARE_BP = 4000`, `RESERVE_PCT = 20` (50 in a Folio whose Rubric is `run.rubric.seal`), `EDITOR_ROOM_W = 1500`, `elite_share(d) = 1000 + 100 × min(d, 20)`.

```
pop        = mul_bp(POP_BASE_MSKE, density_bp)                     for a standard or challenge Folio
           = mul_bp(mul_bp(POP_BASE_MSKE, density_bp), BOSS_APPROACH_BP)   for a boss Folio (the Editor checkpoint squad)
           = 0                                                      for a rest Folio
B          = pop
mini       = 1 if the layout holds a room.arena, the Folio is standard or challenge (not siege) and B >= 20000, else 0
B         -= 20000 × mini
champions  = 0 if mini or boss Folio; else (1, × 2 under lattice.marg.champions) + champion_extra (doc 14, pos >= 3),
             each placed only while B >= 6000;                                          B -= 6000 × champions
elites     = floor_div(B × elite_share(d), 3000 × 10000);                               B -= 3000 × elites
minions    = floor_div(floor_div(B × MINION_SHARE_BP, 10000), 300);                     B -= 300 × minions
standards  = floor_div(B, 1000)                                    // a leftover below 1000 is dropped
reserve    = floor_div(minions × RESERVE_PCT, 100) minions and floor_div(standards × RESERVE_PCT, 100) standards,
             withheld from placement for waves (§10.4)
keeper     = 1 NPC flagged keeper when the Rubric is run.rubric.keeper; rank per doc 14 §3.1
             (elite at pos 1 and 2, champion at pos 3 and 4 and everywhere in Marginalia); outside B
```

Example, Folio 16 (standard, density 10000, no arena, Rubric keeper at pos 1): `B = 29000 - 6000 = 23000`; `elite_share = 2600`, elites `floor_div(59,800,000, 30,000,000) = 1`, `B = 20000`; minions `floor_div(floor_div(20000 × 4000, 10000), 300) = floor_div(8000, 300) = 26`, `B = 12200`; standards 12; reserve 5 minions and 2 standards; plus one elite Keeper. Under the Seal Rubric the same Folio reserves 13 minions and 6 standards. A Folio-10 boss Folio's checkpoint carries `mul_bp(29000, 6000) = 17400` mSKE: 1 elite, 19 minions, 8 standards, and no mini-boss, Keeper or waves. Rest Folios carry nothing.

**Why 29000 and 4000.** Doc 12 makes position 4 a standard or challenge Folio (rest weight 0 there, doc 12 §6.1), so a combat-typical run fights in four Normal Folios per Volume. With the earlier budget (30000 mSKE, minion share 6000) that run raised the main weapon skill to 50 at Folio 7 and 75 at Folio 17, outside Canon §7.2's contract (10 ± 2 and 22 ± 3), because Canon §7.1 pays 1.5 skill XP per enemy drained whatever its rank and the 48-minion Folios supplied about 59 enemies each. The budget and minion share above give about 38 enemies and 37.3 standard-kill equivalents per Folio, close to Canon §17.4's 40, and put the contract and the level anchor back in band (§11.2).

**Placement.** The spawner rooms are the rooms with `spawn_slots > 0`, in ascending `NodeId`; a boss Folio's only spawner is `room.editor_checkpoint`, and a siege challenge's arena takes no initial population. The mini-boss stands in `room.arena`, the Keeper in doc 12's Keeper room. Champions go one per room to the spawner rooms with the most `spawn_slots`, ties to the lower `NodeId`. Elites, minions and standards (after the reserve is withheld) are apportioned per rank in proportion to `spawn_slots` by largest remainder: room `r` first gets `floor_div(count × slots_r, Σ slots)`, then the leftover goes one each to the rooms with the largest remainders, ties to the lower `NodeId`.

**Party size.** Each room's per-rank counts are then multiplied by `enemy_count_bp[n]` with one carry per rank that runs across the rooms in ascending `NodeId` and on through every later wave of the Folio (Canon §17.2, §2.3). The Keeper, the mini-boss, the Editor and hunters are never count-scaled.

**Cells and squads.** A room's members take its spawn cells in ascending cell index, skipping any cell whose `light_block` exceeds the zone genre's `spawn_block_max` (Canon §5.9, §12.3); members left without a usable cell join the reserve at their rank. The members of one room, in ascending EntityId, form squads of at most 8. The first squad of a room that lies on a doc 12 patrol route takes posture `patrol` along that route; every other squad takes `guard` (hold the room). Doc 12 gives rooms within 24,000 mm of path from the Incipit no `spawn_slots` (doc 14 §3.3).

### 10.2 Population draws

At Folio entry, after doc 12's records instantiate (doc 12 §7.1), the Director derives `rng.spawn` and draws in exactly this order:

1. The aspect set (§7.3): `k` `weighted` calls, made at every Folio entry (the Prologue excepted) whatever the Folio type, so the exit preview always matches.
2. For each spawner room in ascending `NodeId` with at least one member after party scaling: one **faction draw** (below); then, in this order, per champion one template `weighted` over the faction's non-swarmer entries and, from Folio 11, one non-aspect modifier draw (§7.2); per elite one template draw over non-swarmer entries and one modifier draw; per minion one template draw over the faction's swarmer entries (over all its entries when it has no swarmer; the member still spawns at `rank.minion`); per standard one template draw over non-swarmer entries and one `roll_bp(elite_promote_add_bp)` (§7.2).
3. The Keeper: it takes the faction drawn in step 2 for its room when that room had members, and otherwise makes its own faction draw for that room (so 0 or 1 faction draw); then one template draw over non-swarmer entries, and its modifier draw (an elite always; a champion from Folio 11, after taking aspect `set[0]`).
4. The arena: a mini-boss is its genre's fixed template and draws nothing; the substitute arena champion of §9.3 makes a faction draw, a template draw and, from Folio 11, a modifier draw. A duel challenge's champion (doc 12 §6.3) and a `room.duel_page` champion make the same three draws; the first carries the whole aspect set and the second `set[0]`.
5. Civic packs: for each native faction of the Folio genre that is neutral or better toward `fac.errata`, in ascending faction ID, that has at least one roster entry: 3 template draws over its entries. Packs stand at doc 12's civic anchors, one pack per anchor in ascending cell index, cycling when packs outnumber anchors. They are outside the budget and XP-eligible only once they turn hostile (§11.1).
6. Captive hirelings: for each `room.prison` and `secret.room.oubliette` in ascending `NodeId`, one `weighted` call over the six base hireling types at weight 1000 each (§13.4).
7. The quest giver (§13.3) and the companion encounter (§13.5) draws.

**Faction draw.** One `weighted` call over the candidates in ascending faction ID byte order. The candidates come from the room's zone genre (a bleed room uses its bleed genre): every `fac.native.<g>.*` faction that holds at least one roster entry and whose party reputation at Folio entry is unfriendly or hostile, weighted by the sum of its entries' W; `fac.wild.<g>` when it holds at least one entry, weighted the same way; and `fac.editors` at `EDITOR_ROOM_W`. When the room carries a doc 12 territory faction F that is unfriendly or hostile and holds an entry, the list is `[F]` with weight `[1]`. In `genre.unwritten` the list is `[fac.editors]` (Canon §5.2). Every member of the room is drawn from the chosen faction, so no pack spawns beside a faction it is hostile to; `fac.editors` draws use the six §8.15 templates. Example (Mythic Fantasy, kingsguard friendly at +150): the candidates in order are `fac.editors` 1500, `fac.native.mythic_fantasy.mage_conclave` 1500, `fac.native.mythic_fantasy.orc_warband` 3000 (`orc_raider` 2000 + `orc_sapper` 1000) and `fac.wild.mythic_fantasy` 2500 (`cave_troll` 1500 + `wyvern` 1000), so `range(8500)` below 1500 makes the room an Editor squad, 1500 to 2999 a conclave pack, 3000 to 5999 an orc pack and 6000 to 8499 a wild pack.

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

**Initial phase** at Folio entry: `build_up` in standard and challenge Folios, `boss` in boss Folios, `exhausted` in rest Folios and the Prologue. Each Director run first applies the §10.3 decay, then the first matching exit row of the current phase, then the scripted-wave queue below.

| Phase | Reserve waves | Exit condition → next (first match) |
|---|---|---|
| `director.phase.build_up` | one wave when `t - last_wave_tick >= 900`, the reserve is not empty and the Rubric is not `run.rubric.seal` | `I ≥ 8000` → `sustain`; reserve empty and no member of a reserve or Seal wave alive → `exhausted` |
| `director.phase.sustain` | none | 300 ticks since entering → `fade` |
| `director.phase.fade` | none | `I ≤ 2000` → `relax` |
| `director.phase.relax` | none | 1200 ticks since entering → `build_up` |
| `director.phase.boss` | none; boss scripts own adds | the Editor's `ev.entity_killed` → `exhausted` |
| `director.phase.exhausted` | none | none (terminal); hunters (§10.5) and scripted waves continue |

`last_wave_tick` starts at the Folio entry tick. A **reserve wave** takes `min(reserve, 3000 + 1000 × volume)` mSKE from the reserve, where `volume = min(5, v(f))` with doc 14 §3.1's Volume `v(f)` (so 5 in Marginalia), filled with reserved standards first and reserved minions for the remainder. In a Seal Folio the reserve belongs to the Seal: its three waves (doc 14 §3.4 hold ticks 0, 600 and 1200) take `floor_div(R0, 3)` mSKE each from the reserve `R0` held at Folio entry, the third taking what is left.

**Scripted waves** fire in every phase, except in a boss Folio before the Editor dies. Every one except the Seal's is in addition to the budget and outside the reserve; all are XP-eligible:

| Source | Amount (mSKE) | Anchor (no draw unless stated) |
|---|---|---|
| Seal Rubric (above) | `floor_div(R0, 3)` from the reserve | qualifying anchor nearest the Seal point |
| `gpv.evt.patrol_shift` (doc 12 §19) | 1000 | anchor nearest the Colophon; the pack walks toward the Incipit |
| `gpv.evt.ambush`, a dread-meter `ambush` complication (doc 12 §4), each `gp.ambush_extra` ambush (doc 14) | 2000, split 1000 and 1000 | the two anchors doc 12's ambush rule picks (2 draws there) |
| `gpv.evt.reinforcements` | 3000 | qualifying anchor farthest from every Erratum |
| `lattice.challenge.siege` | 8 waves of 1500, at 0, 900, …, 6300 ticks after the arena seals | the arena's spawn cells, ascending index |
| `law.cyberpunk.corporate_response` (doc 12 §4) | `3000 + 1000 × volume`, at once on each camera alarm, at most 2 per Folio, ignoring the 900-tick spacing | qualifying anchor nearest the camera |
| `law.mythic_antiquity.hubris` Nemesis | one champion with the whole aspect set, once per Folio | anchor farthest from every Erratum |

A scripted wave's amount is filled with standards `floor_div(amount, 1000)` and then minions `floor_div(remainder, 300)`; the Nemesis is a single champion. Every wave is one pack and one squad (split at 8), scaled by `enemy_count_bp[n]` with the Director's per-rank carry, drawn as §10.2 does: one faction draw for the anchor's zone genre, then the per-member template and promotion draws.

**Reserve-wave anchors.** A qualifying anchor is a doc 12 spawn anchor whose `light_block ≤ spawn_block_max`, at path distance 24000 to 48000 mm from the nearest Erratum, with no line of sight from any Erratum's eye to the point 1000 mm above the anchor cell's floor. One `weighted` call on `rng.spawn` picks among them in ascending cell index with weight 2 for anchors behind the party (bearing from the nearest Erratum outside every Erratum's 16384 Bam frontal cone) and 1 otherwise. After a doc 09 `alarm_raised(pos, faction)` (doc 09 §6.6), the next reserve wave instead uses the qualifying anchor nearest `pos`, with no draw. With no qualifying anchor a wave waits for the next Director run. A wave spawns with its doc 09 pairs toward the Erratum nearest the anchor at `aware.suspicious` with `lkp` = that Erratum's origin. Waves of every kind wait while 224 or more actors are active, which leaves headroom under `ACTIVE_ACTOR_CAP`; hunters (§10.5) ignore this headroom.

**Duel events.** A `gpv.evt.duel_challenge` champion (doc 12 §19) is drawn on the event tick like a §10.2 step 4 champion (faction, template and, from Folio 11, modifier draws on `rng.spawn`) and carries `set[0]`; it is outside the budget and XP-eligible. While `law.wuxia.honor_duel` covers an accepted duel, every NPC in the duel room except the duel champion drops the duelist from its target list and never selects it (§4.8).

### 10.5 Erasure hunters

`sys.erasure` grants Erasure stacks on doc 14 §3.8's clock: `c = t - clock_start` (Folio entry, or the Editor's death tick in a boss Folio); stack `k` arrives at `c = 25200 + 1800 × (k - 1)` until the Folio's `colophon_entry_tick`. Each Erratum `e` has `threshold(e) = max(6, 10 - red_ink_doses(e))` (Canon §11.6, §16.4), so a threshold of 10 is reached at `c = 41,400` (690 s), 8 at 37,800 and 6 at 34,200.

- **Spawn.** On the tick the first Erratum reaches its threshold, one hunter spawns (`ev.hunter_spawned`); another spawns every 5400 ticks while any Erratum holds at least its threshold, also after Colophon entry (doc 14 §3.8). Hunters skip the Director's wave headroom check.
- **Template.** `enemy.editors.revisor`, rank hunter, role brute, behavior `ai.set.hunter`, body humanoid, tags `editor`, `breathless`, template HP 70 and damage 12, walk 4400 and run 6200 mm/s, AR 15, resist 2500 to every type, morale-immune, skinned per genre (Inquisitor in Gothic Horror, Liquidator in Cyberpunk, Revenue Agent in Hardboiled Noir; doc 18 owns the other skins). Attacks: `strike_through` (line 10000 mm, w 30, ×2.0 `dmg.slash`), `redact` (`cone` 8192 Bam 8000 mm, w 40, direct `st.silenced` and `st.marked`), `margin_lunge` (`charge`). HP takes doc 14's `hunter_hp_bp` through `diff_hp_bp`.
- **Placement.** Among doc 12 spawn anchors at least 32000 mm in straight line from every living Erratum, the one with the largest minimum path distance to the living Errata; with none, the anchor with the largest minimum straight-line distance; ties to the lower cell index; no draw.
- **Knowledge.** A hunter always knows the cell of every living Erratum (source `Omniscient`). Its target is the Erratum with the most `st.erasure` stacks, ties to the lower EntityId, re-evaluated each think.
- **Pathing.** Errata-placed blocks count as passable with cost `1000 + floor_div(10 × run_mm_s, 60)` (the 10 ticks one erasure takes, §10.6). A hunter cannot dig other blocks; it breaks locked doors at the §4.13 breaker cost.
- **Example HP,** solo at Grade 0: Folio 8 `mul_bp(mul_bp(mul_bp(70000, 26597), 120000), 10000) = 2,234,148` mHP; Folio 23 13,591,452 mHP.

**Colophon hunters** (doc 14 §3.9). When the run is `colophon_marked`, `queue_colophon_hunter(layout, kills)` spawns one `enemy.editors.revisor` inside the Colophon at Folio entry in `guard` mode. It idles until an Erratum's origin enters the Colophon or it takes damage. When `colophon_hunter_kills ≥ 3` it carries one non-aspect elite modifier from the §7.2 selection with the minimum-Folio rule ignored (one `weighted` draw on `rng.spawn`). Colophon hunters are not Erasure hunters and never count toward the cadence above.

### 10.6 Block erasure

Every `fac.editors` member, hunters included, erases blocks placed by Errata (Canon §11.6), except the player-controlled invader, which Canon §17.9 forbids to break blocks; that specific rule wins over §11.6's general one (Canon gaps). Doc 03 keeps a `placed_by_errata` bit per block, set when an Erratum, a minion or an Errata gadget places a block and cleared when the block changes for any other reason.

```
on tick t where (t + phase(m)) % 10 == 0, for each fac.editors member m in ascending EntityId:
    c = m.origin + (0, 900 mm, 0)
    cand = blocks b with placed_by_errata(b) and dist2(c, center(b)) <= (1500 mm)^2
    if cand is empty: continue
    b = argmin over cand of (dist2(c, center(b)), by, bz, bx)
    set b to air with no drop; emit ev.block_changed { cause: erased, by: m }
```

A block erased by one member is gone before the next member is processed. Example: two Errata blocks have centers 1000 mm and 1414 mm from a hunter's point `c`; it erases the 1000 mm block on its next erasure tick and the 1414 mm block 10 ticks later. A third block at 1600 mm is never erased from that spot.

### 10.7 Invading Editors

Canon §17.9 fixes opt-in invasions: one player from another session joins as a **Redactor** in `fac.editors` at `rank.hunter`, scaled by `H(d)`, `T(d)` and the §17.2 party tables at the host's depth, using a fixed Editor kit chosen from the kits unlocked on their account. Doc 16 §9.13 owns the window, matchmaking, `InvaderJoin { kit }` and transport; doc 15 owns rewards; doc 14 owns the Redacted outcome. This chapter supplies the body, the kits and the invader's place in the AI world.

- **Template** `enemy.editors.invader`: rank hunter multipliers (HP 120000, damage 20000, Canon §17.3), template HP 70 and damage 8, walk 4300 and run 6500 mm/s (the Errata speeds, Canon §4.2), AR 15, resist 2500 to every type, faction `fac.editors`, tags `editor` and `breathless`, `no_loot`, every attack with empty `dep`. HP is `mul_bp(mul_bp(mul_bp(mul_bp(70000, H(d)), 120000), party_hp_bp(n, hunter)), diff_hp_bp(hunter, f))`, the §2.2 rule with the boss-row party table. Its template damage is 8, not the NPC hunter's 12, because a human invader lands far more of its attacks than the AI does.
- **No AI.** It has no think; slot 4 drives its `comp.control` (Canon §2.4, doc 16), so Canon §11.8 governs its statuses. It is never a minion, never hacked, never perceives through doc 09 meters (doc 09 §14.5 culls what its client sees), and cannot be sneak-attacked unless asleep.
- **Limits** (Canon §17.9). It cannot place or break blocks (so it never runs §10.6), open containers or pick up items. Its kit is its only gear.
- **Entry.** It enters at the §10.5 hunter placement anchor on the `InvaderJoin` tick.
- **Allegiances.** Natives and wild creatures hold it hostile (§12.2 rule 1). `fac.editors` NPCs treat it as an ally and never target it.
- **XP.** Killing it pays rank-hunter kill XP to the party under §11.1 (doc 04 §12.3a) on top of Canon §17.9's guaranteed Reliquary roll.

**Editor kits** (`enemy.editors.kit.*`). `InvaderJoin { kit }` names one of these. `revisor` is unlocked for every account; doc 14 owns the account unlocks of the other three (Canon gaps). Each kit binds four attacks to the invader's primary, secondary and two ability inputs; every attack is an `enemy.ab.*` profile of §2.1 with empty `dep`.

| Kit | Primary | Secondary | Ability 1 | Ability 2 |
|---|---|---|---|---|
| `enemy.editors.kit.revisor` | `strike_through`: melee line 4000 mm, ×1.0 `dmg.slash`, w 12 a 6 r 18 | `redact`: cone 8192 Bam 8000 mm, direct `st.silenced` and `st.marked`, w 40, cooldown 600 | `margin_lunge`: charge 9000 mm/s up to 10000 mm, ×1.5 `dmg.blunt`, w 30, cooldown 480 | `revision`: direct `st.fortified` on itself, cooldown 1800 |
| `enemy.editors.kit.annotator` | `annotation`: projectile 40000 mm/s, ×0.8 `dmg.arcane`, w 10 a 4 r 20 | `red_mark`: `marker` on an Erratum within 30000 mm, impact at tick 90, `circle` r 2500, ×2.0 `dmg.arcane`, cooldown 600 | `footnote_blink`: teleport 8000 mm to a walkable cell, w 20, cooldown 600 | `gloss_over`: direct `st.invisible` 300 ticks, cooldown 2400 |
| `enemy.editors.kit.censor` | `bar`: hitscan 20000 mm, ×0.6 `dmg.arcane`, w 8 a 2 r 16 | `gag_order`: cone 8192 Bam 12000 mm, ×1.2 `dmg.psychic`, direct `st.silenced`, w 50, cooldown 480 | `bar_sweep`: a 12000 × 1500 × 3000 mm bar crossing at 4000 mm/s after a 60-tick `line` telegraph, ×1.5 `dmg.arcane`, cooldown 900 | `blackout`: effective light 0 within 12000 mm for 300 ticks, w 30, cooldown 2400 |
| `enemy.editors.kit.footnoter` | `ibid`: projectile 30000 mm/s, ×0.7 `dmg.arcane`, w 12 a 4 r 20 | `blot`: lob, `circle` r 2000, 1000 ml `liq.red_ink` on the landing cell, ×0.5 `dmg.arcane`, w 30, cooldown 360 | `footnote`: summons 2 `enemy.editors.inkling` at rank minion, tag `summoned`, at most 4 alive, w 60, cooldown 900 | `cross_reference`: swaps places with its nearest inkling within 16000 mm, cooldown 1200 |

Worked example: an invader at Folio 10 against a party of 4 at Grade 0 has `mul_bp(mul_bp(mul_bp(70000, 34423), 120000), 37000) = 10,698,668` mHP, and its ×1.0 `strike_through` hits for `mul_bp(mul_bp(8000, 16190), 20000) = 25,904` mHP before step 2.14 lifts it to `mul_bp(25904, 13000) = 33,675` mHP: about a fifth of a level-12 Erratum's HP, so four players who focus it win, and a lone straggler loses.

## 11. Character XP pacing

### 11.1 Sources and attribution

Character XP comes from the Canon §8 sources (kills, skill-ups, Folio exit, Unseen) and doc 04 §6.1's capped additions (pacify, discovery, identification, crafting, objectives). One unit is `kill_xp_std(d)`. This chapter fixes the kill side:

- A **defeat** is `ev.entity_killed` of an XP-eligible entity, or a pacification under doc 04 §6.1, which emits `ev.entity_pacified`: a spared surrender (§4.9), a dialogue Persuade, Intimidate or bribe that pacifies a pack (§13.7), a Tame (§6), a Hacking-100 hack (doc 09 §13.5) or a doc 09 §9.4 subdue. Pacification pays exactly the kill XP once, and that entity can never also pay kill XP.
- **XP-eligible** means not tagged `summoned`, not a mount, and either a member of a faction unfriendly or hostile toward `fac.errata` or hostile toward an Erratum at some time in the last 600 ticks. Civic packs are not eligible until they turn hostile.
- **Attribution.** A kill pays the Canon §8 formula `mul_bp(mul_bp(kill_xp_std(d), rank_xp_bp), 7000)`, split by the Canon §8 co-op rule, only when the killing blow or at least 2500 Bp of the victim's Max HP in damage within the last 600 ticks came from `fac.errata` members (Errata, minions, hacked constructs, Errata-placed hazards). Entities killed purely by other factions pay 0. Kills by minions and hacked constructs count as their owner's.

- **Abridged.** In `mode.abridged` every kill and pacification payment is then × 16667 Bp (Canon §17.1, doc 14 §8.2a), applied with one `mul_bp` after the co-op split, and `d` is the Folio's effective depth `d_eff`.

Worked example: an elite at Folio 10 (`kill_xp_std = 252`) pays `mul_bp(mul_bp(252, 30000), 7000) = 529`; in a party of 4 each member gains `floor_div(529 × 10000, 17500) = 302`. A standard at Folio 25 pays `mul_bp(1386, 7000) = 970`. In Abridged, the same elite at `d_eff = 10` pays a solo player `mul_bp(529, 16667) = 881`.

### 11.2 Tuning simulation

**Reference run** (combat-typical, reproducible by the doc 02 bots harness): solo, Standard mode, Grade 0, MEM 40, density 10000 in every genre, no events. Per Volume: position 1 a standard Folio with the keeper Rubric (elite Keeper); position 2 standard with the Seal Rubric; position 3 standard with the keeper Rubric (champion Keeper) and a `room.arena`, so it holds the mini-boss and no budget champion; position 4 standard with the keeper Rubric (champion Keeper), its most likely type and Rubric (doc 12 §6.1 gives position 4 no rest weight; doc 12 §6.2 draws keeper with weight 5000); position 5 the Editor Folio with its checkpoint squad and 20 Editor adds. Every planned enemy is defeated, including the optional mini-boss. Doc 04 §6.3's combat build supplies skill XP: the main weapon skill (start 20) gains 1.5 XP per enemy defeated (Canon §7.1 normalization), plus armor 30, shield 25, athletics 12, Origin skill 6, acrobatics 6, leadership 4 and smithing 3 XP per Folio from their doc 04 starts. Skill-ups count `min(8, ups)`; Exit pays 4 units; doc 04's capped rows pay 15 tenths of a unit per Folio (Rubric 10, half the rooms 5).

```
for f in 1..=25:
    plan  = §10.1 plan for (f, pos, type, Rubric, arena)          // counts per rank, Keeper and mini-boss included
    n     = Σ plan counts (+ 20 Editor adds in boss Folios)
    ups   = level-ups of all eight skills after adding this Folio's skill XP (need(s) = 4000 + 400 × s milli-XP)
    K     = kill_xp_std(f)
    gain  = Σ_rank count × mul_bp(mul_bp(K, rank_xp_bp), 7000) + 2 × K × min(8, ups) + 4 × K + floor_div(15 × K, 10)
    cum  += gain;  level = largest L with Σ_{i<L} 50 × i × (i + 3) <= cum
```

| Folio | pos | K | Plan (champion / elite / minion / standard) | mSKE | Enemies | Kill XP | Counted skill-ups | Folio XP | Units | Cumulative | Level after | Main skill |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 5 | 1/1/30/14 | 32,000 | 46 | 73 | 8 | 180 | 36.0 | 180 | 1 | 25 |
| 2 | 2 | 14 | 1/0/30/14 | 29,000 | 45 | 244 | 8 | 545 | 38.9 | 725 | 3 | 29 |
| 3 | 3 | 25 | 1/0/12/5 + mini | 34,600 | 19 | 588 | 8 | 1,125 | 45.0 | 1,850 | 4 | 31 |
| 4 | 4 | 39 | 2/1/26/12 | 34,800 | 41 | 913 | 8 | 1,751 | 44.9 | 3,601 | 5 | 35 |
| 5 | 5 | 75 | 0/0/23/10 + Editor | 56,900 | 54 | 2,965 | 8 | 4,577 | 61.0 | 8,178 | 7 | 39 |
| 6 | 1 | 98 | 1/2/26/12 | 31,800 | 41 | 2,157 | 8 | 4,264 | 43.5 | 12,442 | 8 | 42 |
| 7 | 2 | 123 | 1/1/26/12 | 28,800 | 40 | 2,456 | 8 | 5,100 | 41.5 | 17,542 | 9 | 45 |
| 8 | 3 | 151 | 1/0/12/5 + mini | 34,600 | 19 | 3,645 | 7 | 6,589 | 43.6 | 24,131 | 10 | 46 |
| 9 | 4 | 182 | 2/1/26/12 | 34,800 | 41 | 4,396 | 8 | 8,309 | 45.7 | 32,440 | 11 | 49 |
| 10 | 5 | 252 | 0/1/19/8 + Editor | 56,700 | 49 | 9,981 | 8 | 15,399 | 61.1 | 47,839 | 13 | 52 |
| 11 | 1 | 291 | 1/2/26/12 | 31,800 | 41 | 6,440 | 8 | 12,696 | 43.6 | 60,535 | 14 | 54 |
| 12 | 2 | 333 | 1/1/26/12 | 28,800 | 40 | 6,687 | 8 | 13,846 | 41.6 | 74,381 | 15 | 57 |
| 13 | 3 | 378 | 1/0/12/5 + mini | 34,600 | 19 | 9,147 | 6 | 15,762 | 41.7 | 90,143 | 16 | 58 |
| 14 | 4 | 425 | 2/1/26/12 | 34,800 | 41 | 10,314 | 8 | 19,451 | 45.8 | 109,594 | 17 | 60 |
| 15 | 5 | 529 | 0/1/19/8 + Editor | 56,700 | 49 | 20,972 | 8 | 32,345 | 61.1 | 141,939 | 19 | 63 |
| 16 | 1 | 585 | 1/2/26/12 | 31,800 | 41 | 12,993 | 7 | 24,400 | 41.7 | 166,339 | 20 | 65 |
| 17 | 2 | 644 | 1/2/22/10 | 28,600 | 35 | 12,878 | 6 | 24,148 | 37.5 | 190,487 | 21 | 66 |
| 18 | 3 | 705 | 1/0/12/5 + mini | 34,600 | 19 | 17,060 | 6 | 29,397 | 41.7 | 219,884 | 22 | 67 |
| 19 | 4 | 770 | 2/2/22/10 | 34,600 | 36 | 18,634 | 8 | 35,189 | 45.7 | 255,073 | 23 | 69 |
| 20 | 5 | 907 | 0/1/19/8 + Editor | 56,700 | 49 | 35,982 | 5 | 50,040 | 55.2 | 305,113 | 25 | 71 |
| 21 | 1 | 980 | 1/3/22/10 | 31,600 | 36 | 21,660 | 7 | 40,770 | 41.6 | 345,883 | 26 | 73 |
| 22 | 2 | 1,055 | 1/2/22/10 | 28,600 | 35 | 21,103 | 7 | 41,675 | 39.5 | 387,558 | 27 | 75 |
| 23 | 3 | 1,134 | 1/0/12/5 + mini | 34,600 | 19 | 27,459 | 5 | 45,036 | 39.7 | 432,594 | 28 | 75 |
| 24 | 4 | 1,215 | 2/2/22/10 | 34,600 | 36 | 29,396 | 5 | 48,228 | 39.7 | 480,822 | 29 | 77 |
| 25 | 5 | 1,386 | 0/1/19/8 + Editor | 56,700 | 49 | 54,988 | 7 | 82,015 | 59.2 | 562,837 | 31 | 79 |

Results: **level 29 on arrival at Folio 25** (480,822 XP after Folio 24; level 29 spans 446,600 to 492,999), level 13 after Folio 10, main skill 50 at Folio 10 and 75 at Folio 22, exactly Canon §7.2's centers. The kill supply averages 37.3 standard-kill equivalents and 37.6 enemies per Folio over the 25 Folios, against Canon §17.4's reference of 40; total income averages 45.5 units per Folio, slightly above Canon §8's 40-unit example (level 29 after Folio 25), so this run reaches level 29 one Folio earlier, still inside the 28 ± 3 anchor. Doc 04 §6.3 replays this plan for its stealth and crafting builds.

Sensitivity, every row inside 28 ± 3 for the level and inside Canon §7.2 for solo skill pace:

| Variant | Level on arrival at Folio 25 | Main skill 50 / 75 at Folio |
|---|---|---|
| reference | 29 | 10 / 22 |
| no `room.arena` anywhere (budget champion at position 3 instead of the mini-boss) | 29 | 9 / 20 |
| Seal Rubric instead of keeper everywhere | 29 | 10 / 23 |
| a rest Folio at position 3 of every Volume | 28 | 10 / 25 |
| one 2000 mSKE event wave (2 standards) in every standard Folio | 30 | 9 / 22 |
| doc 04 rows 0 tenths / at the 40-tenth cap | 29 / 30 | 10 / 22 |
| genre density 11000 (Space Opera, Mythic Antiquity) everywhere | 30 | 9 / 21 |
| party of 2 / 3 / 4 with §10.1 count scaling and the Canon §8 split | 28 / 28 / 28 | 15 / —, 20 / —, 23 / — |

A Folio of density 8000 (Cosmic Horror) everywhere would reach 50 at Folio 12 and miss 75 by Folio 25, but runs mix genres: the 12 core densities average 9917. In co-op the character level holds, but combat skills grow at about `enemy_count_bp[n] / n` of the solo pace, because Canon §7.1 normalizes skill XP by the target's party-scaled Max HP (Canon gaps).

## 12. Factions

### 12.1 Faction list and starting reputation

Reputation is party-shared, per faction, `-1000..=1000`, reset each run, with the Canon §15 stance bands. `fac.wild.<genre_suffix>` (16 factions) and `fac.editors` are fixed at −1000; `fac.margin` is fixed at 1000; `fac.neutral` starts at 0 and turns hostile toward an attacker for the rest of the Folio. Native factions (`fac.native.<genre_suffix>.<group>`) start at +150 (the genre's **trader faction**, which hosts vendors and quest givers), −200 or −600:

| Genre | +150 | −200 | −600 |
|---|---|---|---|
| mythic_fantasy | kingsguard | mage_conclave | orc_warband |
| gothic_horror | hollow_village | inquisition | night_court |
| cosmic_horror | expedition | — | esoteric_order, deep_ones |
| space_opera | void_pirates | psionic_order | imperial_navy |
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

That is 43 native factions, 2 to 4 per genre except The Unwritten. The xeno hive of Space Opera is `fac.wild.space_opera`, because none of its members is sapient (Canon §15).

**Radio.** Factions with doc 09's `radio` flag (callouts reach the squad by radio wherever local TEC ≥ 5, doc 09 §6.6): `space_opera.imperial_navy`, `space_opera.void_pirates`, `cyberpunk.zaibatsu_security`, `cyberpunk.netrunner_collective`, `post_apocalypse.vault_remnant`, `hardboiled_noir.city_police`, `cosmic_horror.expedition`, `redacted.bureau_of_correction`. No other faction has it.

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

Worked matrix, Mythic Fantasy at run start (rows against columns; the rule number that decides each cell in brackets):

| | `kingsguard` (+150) | `mage_conclave` (−200) | `orc_warband` (−600) | `fac.wild.mythic_fantasy` | `fac.editors` | `fac.neutral` |
|---|---|---|---|---|---|---|
| `kingsguard` | — | unfriendly [6] | hostile [6] | hostile [2] | hostile [1] | neutral [7] |
| `mage_conclave` | unfriendly [6] | — | hostile [5] | hostile [2] | hostile [1] | neutral [7] |
| `orc_warband` | hostile [6] | hostile [5] | — | hostile [2] | hostile [1] | neutral [7] |
| `fac.wild.mythic_fantasy` | hostile [2] | hostile [2] | hostile [2] | neutral [2] | hostile [1] | hostile [2] |

The matrix is symmetric and fixed for the run: NPC-to-NPC stances read starting values, never the party's current reputation. Only hostile pairs fight. Because §10.2 draws one faction per room, no pack spawns beside a hostile faction; hostile packs meet only when they move, and at bleed-zone war fronts (§12.4). A `guard` squad engages a hostile NPC only while that NPC is inside the squad's room or within 12000 mm of the squad leader, and never leaves its room to pursue another faction; `patrol` squads and waves engage hostile NPCs they perceive (doc 09) anywhere.

### 12.3 Reputation events

Each event has an ID in `fac.rep.*`. When one act matches several rows, only the first matching row in table order applies.

| ID | Event | Δ to faction F | Witness |
|---|---|---|---|
| `fac.rep.kill_vendor` | kill an F vendor or quest giver | −500; −150 unwitnessed | yes |
| `fac.rep.execute` | kill a surrendered F member | −200 | yes |
| `fac.rep.kill_member` | kill an F member | −150; −40 unwitnessed | yes |
| `fac.rep.damage_member` | damage an F member, first time per member per 600 ticks | −50 | yes |
| `fac.rep.kill_enemy_of` | kill an entity hostile to F within 24000 mm of a living F member | `+min(120, floor_div(20 × rank_xp_bp, 10000))` | yes |
| `fac.rep.spare` | spare a surrendered F member | +60 | no |
| `fac.rep.quest_done`, `fac.rep.quest_failed` | complete a quest for F / fail or abandon it | +200 / −100 | no |
| `fac.rep.pickpocket_caught` | a failed pickpocket attempt on an F member (doc 09 §13.7) | −100 | yes |
| `fac.rep.theft` | take an item from an F-owned container or an F member's inventory | −100 | yes |
| `fac.rep.trespass` | ignore a territory warning for 120 ticks | −50 | yes |
| `fac.rep.gift` | gift through dialogue, once per Folio per faction | `+min(100, isqrt(item.value))` | no |
| `fac.rep.free_captive` | free a captive of F | +150; +100 more when it answers a `gpv.evt.plea` within 1800 ticks (doc 12 §19) | no |
| `fac.rep.desecrate` | destroy an F shrine or banner block | −75 | yes |

**Witnessing.** F witnesses an event when, on its tick, a living sapient F member has awareness ≥ `aware.suspicious` toward the acting Erratum, or is within 24000 mm with a doc 09 sight pass to the event position. A witnessed delta is applied 300 ticks later; if every witness has died or been pacified by then, the unwitnessed value applies instead (0 where none is listed).

**Spillover.** After the main delta Δ, every other native faction G of the same genre changes by `spill(Δ, 5000)` if G is friendly or allied toward F, and by `-spill(Δ, 2500)` if G is hostile toward F, where `spill(Δ, bp) = sign(Δ) × floor_div(|Δ| × bp, 10000)`. Spillover never chains. Every change emits `ev.reputation_changed`.

Example: a witnessed kill of a `kingsguard_knight` moves `kingsguard` from +150 to 0 (neutral) and `orc_warband`, hostile to the kingsguard, from −600 to −563 (`spill(-150, 2500) = -37`).

### 12.4 Bleed-zone faction wars

When doc 12 places a bleed zone of genre B inside a Folio of genre A (doc 12 §14.2) and both genres have native factions, the Director opens one **war front** per bleed zone at its edge: side A is A's native faction with the highest starting reputation, side B is B's native faction with the lowest, ties to the lower faction ID; the two are hostile by §12.2 rule 3. Each side spawns one pack of 4 standards and 1 elite drawn from that faction's roster entries (from any entry of its genre, with the faction overridden, when the faction has none), already at `aware.alert` toward each other. Side A stands at the spawn anchor outside the zone nearest the zone's boundary, side B at the anchor inside the zone nearest it, ties to the lower cell index. Draws: after §10.2 step 7, zones in ascending zone ID, side A then side B, per member one template draw and, for the elite, one modifier draw, all on `rng.spawn`. Side B's 7000 mSKE come out of the reserve as far as it reaches; the rest, and all of side A's pack, are outside the budget. Both packs are count-scaled for the party (§10.1).

- While a side is engaged within 24000 mm of the seam, every Errata kill of its members gives the other side +30 in addition to §12.3.
- The war ends when one side's front members are all dead, fled or surrendered. If the winner is friendly or allied toward Errata, it leaves a container at the front that each Active Erratum opens as its own instance and that rolls doc 11's `loot.chest.locked` table (doc 11 §9.6).
- Kill XP follows §11.1: front members killed by the other side pay nothing unless Errata dealt the killing blow or 2500 Bp of their Max HP.
- Foreign enemies keep their own Contradiction and Edge (§2.6), so B's soldiers that chase across the seam into A are re-evaluated against A's axioms on that tick (Canon §5.5).

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

Each standard or challenge Folio whose genre has a trader faction (every genre but The Unwritten) rolls one `roll_bp(6000)` on `rng.spawn` at §10.2 step 7; on success an `npc.giver.<genre_suffix>` stands at the first civic anchor (ascending cell index) and offers 2 quests drawn without replacement from the seven templates by two `weighted` calls over weights 1000 (a drawn template's weight becomes 0). So the step draws 1 value, or 3 on success. Quests last until `ev.folio_exited`.

| Template | Objective | Tier |
|---|---|---|
| `npc.quest.hunt` | kill a marked elite of a faction hostile to the giver; the Director promotes one placed standard | 2 |
| `npc.quest.rescue` | free an `npc.captive` held by a hostile pack and bring it within 4000 mm of the giver | 2 |
| `npc.quest.retrieve` | bring back a `cat.quest` item placed in a room at least 48000 mm away | 1 |
| `npc.quest.deliver` | carry a `cat.quest` item to a named NPC of another non-hostile faction | 1 |
| `npc.quest.sabotage` | destroy 3 marked blocks in a hostile faction's territory | 2 |
| `npc.quest.parley` | pass a dialogue check (§13.7) with the leader of a −200 faction | 1 |
| `npc.quest.cleanse` | kill every hostile in one marked room | 3 |

Rewards: `mul_bp(reward_base[tier], P(d))` Lucre with bases 80, 120, 160; +200 reputation with the giver's faction (§12.3); per Active Erratum one roll of doc 11's `loot.chest.plain` (tier 1), `loot.chest.locked` (tier 2) or `loot.drop.champion` (tier 3), with `drop_rng(folio, giver EntityId, tick, slot)` (doc 11 §9); doc 04 §6.1's Objective row adds `5 × tier` tenths of a unit. Quests grant no character XP of their own; the Canon §8 sources apply. Example: a tier-2 quest in Folio 12 pays `mul_bp(120, 25804) = 309` Lucre.

### 13.4 Hirelings

Hirelings come from hireling brokers in trader-faction rooms (doc 12), from freed captives (doc 12 `room.prison`, `secret.room.oubliette`, `gpv.evt.plea`; a prison or oubliette captive's base type is its §10.2 step 6 draw, a plea captive is a `soldier`) and from the Hireling Hall (doc 14 `unlock.room.hireling_hall`). Each uses a §3 role template, the §6 minion stat rule and the local genre's skin.

| ID | Role | Service |
|---|---|---|
| `npc.hireling.soldier`, `.guardian`, `.caster`, `.support`, `.sniper`, `.skirmisher` | as named | base types offered by brokers and captives |
| `npc.hireling.apothecary` | support | carries 2 healing potions per Folio (doc 08) and drinks or gives one when its owner is below 3500 Bp HP |
| `npc.hireling.cartographer` | skirmisher | reveals the current Folio's room graph on its owner's map at Folio entry |
| `npc.hireling.lamplighter` | support | places one light source (`light_emit` 12) at its cell every 600 ticks, at most 6 per Folio, which denies spawns there (Canon §12.3) |
| `npc.hireling.porter` | guardian | holds up to 30,000 g of its owner's items in its own inventory, exchanged by interact |
| `npc.hireling.sellsword` | soldier | template HP × 13000 and damage × 12000 in the §6 formulas |
| `npc.hireling.<genre_suffix>_chapter` | that genre's soldier-role roster entry | 12 types, one per core genre, with that entry's signature ability |

The five named types and the 12 chapter types join the Hireling Hall pool through doc 15 Scripts (`unlock.hireling.<type>`). Hire cost `mul_bp(60, P(d))` Lucre; upkeep `mul_bp(15, P(d))` Lucre on each `ev.folio_exited`, paid automatically; an unpaid hireling leaves (`ev.minion_released`). Folio 12: hire 154, upkeep 38. A hireling accepts one weapon from its owner (the §5.3 rule without the pickup action) and returns it on release.

### 13.5 Companions and approval

Companions are named hirelings with dialogue, a personal quest and approval. One companion per core genre appears as an encounter in a standard or challenge Folio of its genre: at §10.2 step 7, after the quest-giver draws, one `roll_bp(2500)` on `rng.spawn` while that companion has not been recruited and has not died this run (no draw otherwise); on success it waits at the last civic anchor (descending cell index), or in the Incipit's adjacent room when the Folio has none, and joins on a dialogue `hire` effect. Recruiting costs nothing; a companion counts as a minion and has no upkeep.

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

**Beast companions.** `npc.companion.<genre_suffix>` (12, one per core genre) are the starting beasts of doc 14's Beastbinder kit: skirmisher role, `body.quadruped`, tag `beast`, the tamed-minion rules of §6, no dialogue and no approval. Skins: dire wolf (mythic_fantasy), grave hound (gothic_horror), abyssal hound (cosmic_horror), xeno hound (space_opera), cyber hound with tag `electronic` (cyberpunk), two-headed dog (post_apocalypse), coyote (weird_west), bulldog (hardboiled_noir), tiger cub (wuxia), clockwork hound with tag `electronic` (steampunk), fox (fairy_tale), Molossian hound (mythic_antiquity).

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

Thresholds: at ≥ 60 the loyalty perk is active; at 100 its personal quest is offered once (a tier-3 quest of the template its Likes suggest: `hunt` for valor, `rescue` for mercy, `retrieve` for greed and curiosity, `cleanse` for zeal, `sabotage` for cunning, `deliver` for order and kinship; its reward adds one doc 11 `loot.secret.cache` roll per Active Erratum); at `ev.folio_exited` with approval ≤ −50 it leaves; on reaching −100 it turns hostile at once as an elite of `fac.editors` with its role template.

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
        "check": { "kind": "persuade", "dc_base": 30 },
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
- **Checks** are deterministic (doc 04 §4.2: dialogue is not a random check) and draw nothing:

  ```
  value = skill + floor_div(sub_eff - 40, 2) + floor_div(rep, 100) + bonuses   // rep: party reputation with the speaker's faction
  dc    = dc_base + floor_div(d, 2) - (15 if the speaker's skill is 100)        // doc 04 Speech 100
  pass  = value >= dc
  ```

  Persuade uses Speech and CHM, Deceive Speech and GUI, Intimidate Leadership and MEN. `bonuses` are doc 04's `courtesy` +10 and `silver_tongue` +10. `dc_base` tiers: Easy 5, Standard 15, Hard 30, Severe 45. The UI shows `value` and `dc` before the choice. A failed choice stays unavailable on that speaker until the speaker's `value` rises. A pass grants `100 × dc` milli-XP to the skill (doc 04 §5.2). Example: Persuade with CHM 52, Speech 35 and reputation +150 at Hard tier in Folio 12: `value = 35 + 6 + 1 = 42`, `dc = 30 + 6 = 36`, a pass worth 3,600 milli-XP of Speech; with Speech 25 the value is 32 and the choice fails.
- **Effects:** `rep`, `give_item`, `take_item`, `give_lucre`, `take_lucre` (base × `P(d)`), `start_quest`, `complete_quest`, `hire`, `approval`, `set_flag` (Folio-scoped; `run.` prefix for run-scoped), `open_shop`, `pacify_pack` (the speaker's squad becomes `fac.neutral` and walks out; each XP-eligible member is a defeat, §11.1), `bribe_pack` (doc 04 Speech 50: takes `Σ mul_bp(floor_div(100 × rank_xp_bp, 10000), P(d))` Lucre over the squad's members, then `pacify_pack`), `turn_hostile`, `teach_recipe` (doc 07), `end`.
- **Parley.** An Erratum opens the leader's dialogue by interact in a hostile sapient pack when no member is in `Windup` or `Active` and the party has not damaged the pack for 300 ticks. The pack holds position (`circle`) for up to 600 ticks while the dialogue is open; the game does not pause.
- **Barks** are presentation only: client-side text keys `loc.npc.bark.<set>.<n>`, rewritten by the optional on-device Scribe model when enabled and cached by seed (brief §5). They never touch simulation state.

## Exports

All IDs below are in namespaces Canon §19 assigns to this chapter (`enemy.`, `ai.`, `fac.`, `boss.`, `npc.`, `director.`, `rank.`, `elite.`).

| Namespace | IDs defined |
|---|---|
| `enemy.role.*` | soldier, swarmer, brute, skirmisher, shooter, sniper, caster, controller, support, guardian, summoner, stalker, artillery, sapper, flier, cavalry (16), plus `enemy.role.boss` |
| `enemy.<genre_suffix>.*` | the 104 genre roster entries of §8.1 to §8.14 and §8.16 |
| `enemy.editors.*` | inkling, strikethrough, marginal_scribe, footnote, erratum_eater, red_pen_lancer, revisor, invader |
| `enemy.editors.kit.*` | revisor, annotator, censor, footnoter: the invader Editor kits of §10.7 |
| `enemy.mimic`, `enemy.prologue_hunter` | doc 11's mimic and doc 14's Prologue chaser (§8.17) |
| `enemy.ab.*` | every attack and ability named in §2.1, §3, §8, §10.5 and the §10.7 kits (boss attacks use `<boss id>.<attack>`) |
| `ai.curve.*` | linear_up, linear_down, soft_up, quad_up, quad_down, cubic_up, binary, binary_not, gate_half, gate_reach, logistic_up, logistic_down, bell_mid, plateau |
| `ai.input.*` | the 25 inputs of §4.3 |
| `ai.action.*` | the 33 actions of §4.7 |
| `ai.set.*` | the 20 behavior sets of §4.7 |
| `ai.nav.*` | small, humanoid, large, mounted, burrower, flier, swimmer |
| `fac.native.*` | the 43 native factions of §12.1 |
| `fac.rep.*` | the 14 reputation event IDs of §12.3 |
| `fac.wild.*` | one per genre, 16 |
| `boss.*` | `boss.editor_v1` to `boss.editor_v4`, `boss.ur_editor` (IDs requested by doc 14; display names the Proofreader, the Censor, the Continuity Editor, the Abridger, the Last Word) and the 14 mini-bosses `boss.<genre_suffix>.<name>` of §9.3; attacks as `<boss id>.<attack>` |
| `npc.vendor.*` | general, armorer, gunsmith, apothecary, arcanist, tinker, fence, scribe, stablemaster |
| `npc.giver.*`, `npc.captive.*` | one per genre with native factions |
| `npc.hireling.*` | soldier, guardian, caster, support, sniper, skirmisher, apothecary, cartographer, lamplighter, porter, sellsword, and `<genre_suffix>_chapter` for the 12 core genres |
| `npc.companion.*` | the 12 named companions of §13.5 and the 12 beast companions `npc.companion.<genre_suffix>` |
| `npc.margin.*` | archivist, binder, recollector, cartographer, inkwell_keeper, codex_scribe, quartermaster, drillmaster |
| `npc.quest.*` | hunt, rescue, retrieve, deliver, sabotage, parley, cleanse |
| `npc.dlg.*` | dialogue records (§13.7) |
| `director.profile.*` | standard, boss, rest |
| `director.phase.*` | build_up, sustain, fade, relax, boss, exhausted |
| `elite.*` | the 17 non-aspect modifiers of §7.2, including `elite.armored` and `elite.phasing` requested by doc 05 (the 3 aspects are canon) |
| `rank.*` | none new |

Also exported: the `comp.tags` bit layout (§2.4), the `ai_target` rule (§4.8), `aspect_set()` for doc 12's exit preview (§7.3), `queue_colophon_hunter` (§10.5), the faction `radio` list (§12.1), and constants `MOMENTUM_BP = 12500`, `BAND_BP = 9000`, `MINION_HP_BP = 15000`, `POP_BASE_MSKE = 29000`, `BOSS_APPROACH_BP = 6000`, `MINION_SHARE_BP = 4000`, `RESERVE_PCT = 20` (50 under the Seal Rubric), `EDITOR_ROOM_W = 1500`, wave interval 900 ticks, `elite_share(d) = 1000 + 100 × min(d, 20)`, token pools 3 / 4 / 1 / 1, dialogue `dc_base` tiers 5 / 15 / 30 / 45, tame tiers 10 / 20 / 50.

## Interface assumptions

| Doc | Provides |
|---|---|
| 02 | the 1.5 ms host AI budget row; the bots harness that reproduces §11.2; `tools/gen_curves.py`; `atan2_bam` |
| 03 | the §4.1 stages inside `sys.ai_think`; a `placed_by_errata` bit per block and a `grid.cause.erased` cause on `ev.block_changed`; `grid.cause.toggled` for relit fixtures; a view-frame channel for telegraphs (doc 02's `ViewTelegraph`); dynamic light override volumes (the Censor's `blackout`, the `censor` kit); `request_spawn` and `spawn_q`; sleep rules |
| 04 | the `value` bonuses of §6 and §13.7 (`courtesy`, `silver_tongue`, `beast_tongue`, Speech 50 and 100, Handling 75); `ev.entity_pacified`; Handling and Leadership factors applied after the §6 minion formulas; a replay of doc 04 §6.3 on the §11.2 plan |
| 05 | NPC action state machines that run §2.1 attack profiles; counters; NPC riders on `arch.mount.*` with `mount_hp_mhp`; NPC grabs |
| 06 | summon profiles and stats; hostile `summon.herald` and `summon.misprint` spawns handed to §5.5; NPC spell scale; scorer queries |
| 07 | `break_ticks(material)`; the `destructible` tag; doors with HP; ladder, vine and rope blocks; `block.sand`, `block.dirt`; `mat.cold_iron`; lit and unlit fixture states |
| 08 | potion heal amounts; gas emission in ml for steam, miasma and smoke |
| 09 | the six awareness states and their `ai.input.target_aware` values; the §6.7 search plan; sight-pass queries; callouts, radio and `raise_alarm` with `alarm_raised(pos, faction)` to the Director; hacking buildup; senses `breath`, `tremor`, `thermal`, `scent`; holding breath; perception profiles reading the `flier` tag; disguises reading `outfit`, `enforcer` and `radio`; subdue as a pacification |
| 10 | monster-use and witness identification; `curse.red_letter` radius |
| 11 | `loot.drop.<rank>`, `loot.trophy`, `loot.chest.plain`, `loot.chest.locked`, `loot.secret.cache`, `loot.volume_reward`, bags and `drop_rng`; mount token drops; `shop.<type>`; the reputation price factor; common item lists per category and genre |
| 12 | the §12 population interface (`spawn_slots`, spawn cells and anchors, patrol routes, territory, civic anchors, Keeper room, Seal point, rescue flag); `room.arena` from `room.rule.arena`; zone genres and static and dynamic bleed zones; Folio types and challenge kinds; narrative events and complications that call §10.4; bell, klaxon and fuse-box props; Editor and mini-boss arenas built to §9; gravity-flip volumes; the exit preview calling `aspect_set()` |
| 14 | `diff_hp_bp` and the step-2.13 factor; Grade parameters `elite_promote_add_bp`, `champion_extra`, `hunter_hp_bp`, `final_phase_extra`, `ambush_extra`; Keeper and Seal Rubrics; `RunState` fields `clock_start`, `colophon_entry_tick`, `colophon_marked`, `colophon_hunter_kills`; telegraphed-hit scoring; Marginalia rotation; Hireling Hall; the Prologue script that spawns `enemy.prologue_hunter` and three inklings; account unlock nodes for the `annotator`, `censor` and `footnoter` Editor kits; the Redacted outcome |
| 15 | Script unlocks of hireling types; invader rewards |
| 16 | host-only AI; telegraphs in snapshots; invasion netcode, `InvaderJoin { kit }` naming an `enemy.editors.kit.*` ID, and player control of `enemy.editors.invader` |
| 17 | minion command inputs; telegraph and shimmer rendering; dialogue UI showing `value` and `dc`; invader kit input bindings |
| 18 | text keys `loc.npc.dlg.*`, `loc.npc.bark.*`; enemy, boss, hunter and companion skins |

## Canon gaps

1. **Canon version.** This chapter now conforms to canon-1.4: limited building (§4.3: only `destructible` blocks and Errata placeables break) and invasions (§17.9) are applied in §1, §4.13, §10.6 and §10.7.
2. **Defeats without death.** Canon §8 pays XP only for kills. Following doc 04 §6.1, pacification pays the kill XP once and emits `ev.entity_pacified`, which Canon §18 does not list; requested for §18 (doc 03 owns the bus).
3. **Kill attribution.** Canon §8 does not say who earns XP for enemies killed by other factions or the environment. §11.1 requires the killing blow or 2500 Bp of Max HP from `fac.errata`.
4. **Summoned enemies** pay 0 kill XP (Canon §8 is silent); otherwise summoners are infinite XP sources.
5. **Champion wording.** Canon §17.3 says a champion has "one champion counter type (doc 05)" while §17.3a defines the aspects as `elite.*`. Interpreted: one aspect from §7.3's set, countered by its matching counter type.
6. **Two forty-unit targets.** Canon §17.4 sets 40 standard-kill equivalents per Folio and Canon §8 asks for "near 40 units" of total character XP per Folio, but at Canon §8's own rates 40 kill equivalents already pay 28 units, and the eight counted skill-ups, the exit and doc 04's rows add about 21 more. Both cannot hold. This chapter keeps the kill supply near Canon §17.4 (37.3) and Canon §7.2's skill pace exact, which gives 45.5 units per Folio and level 29 on arrival at Folio 25, inside the 28 ± 3 anchor (§11.2). Recommended canon wording: "near 45 units".
7. **Co-op skill pace.** With Canon §7.1 normalization by the target's party-scaled Max HP and the §17.2 count table, a member of a party of `n` gains `enemy_count_bp[n] / n` of the solo combat skill XP (4375 Bp at 4 players): the main skill reaches 50 at Folio 23 and never 75 by Folio 25, although character levels stay in band (28). Recommended canon change: normalize skill XP by `floor_div(target_max_mhp × 10000, party_hp_bp)`.
8. **Schedule.** Canon §2.2 and doc 03 §2.2 have no rows for the squad coordinator, Director and path service; they run inside `sys.ai_think` (§4.1).
9. **The Unwritten** has no native factions (Canon §5.2) although Canon §15 says 2 to 4 per genre; Canon §5.2 wins.
10. **NPC block stability.** Canon §6.5 excludes row 19 for NPCs; templates carry `block_stability_bp`.
11. **Enemy mounts.** Canon §17.7 describes Errata riders; enemy riders reuse `st.mounted` and the doc 05 §17.3 charge rules.
12. **Last Word.** `genre_shift` makes a dynamic bleed zone inside `genre.ur_text`; Canon §5.5's override is keyed to the local genre, so Contradiction applies inside the zone. This is intended and uses doc 12 §14.3's `set_dynamic_zone`.
13. **Quests** have no canon namespace; this chapter uses `npc.quest.*`.
14. **Mini-bosses.** Doc 14 §3.1 has no mini-boss slot. A mini-boss stands in doc 12's optional `room.arena` and is paid from the Folio budget (§9.3), which leaves doc 14's Keeper ranks and Rubrics unchanged.
15. **Downed targets.** Doc 16 cites "downed candidates × 2000" from an earlier draft of this chapter; this chapter adopts doc 05 §14.1's × 2500, and doc 16's summary needs × 2500.
16. **Invader and blocks.** Canon §11.6 says every `fac.editors` member erases Errata placeables; Canon §17.9 says the invader cannot break blocks. The specific rule wins: the invader never erases (§10.6). Canon §17.9 also calls every invader a "Redactor", so Editor 2's display name is now the Censor; doc 12 §21.2's "the Redactor's archive" should read "the Censor's archive".
17. **Erasure cause.** Doc 03's `grid.cause` list has no `erased`; §10.6 assumes doc 03 adds it.
18. **Companion IDs.** Doc 14 gives the Beastbinder "one tamed beast of template `npc.companion.<origin_suffix>`". §13.5 defines those 12 IDs as beast companions and gives the named approval companions `npc.companion.<name>` IDs.
19. **No `rng.check`.** Canon §3.3 lists no `rng.check` stream; doc 04 uses one for its own checks. Following doc 04 §4.2, this chapter resolves dialogue (§13.7) and taming (§6) deterministically and draws from no stream outside Canon §3.2 and §3.3.
20. **Editor kits.** Canon §17.9 says invader kits are "unlocked on their account" but names no unlock owner. §10.7 defines four kits with `revisor` always available; doc 14 owns the `unlock.` nodes for the other three.
21. **Native factions are sapient.** Canon §15 makes `fac.native.*` sapient factions and `fac.wild.*` beasts and mindless monsters. The xeno hive (no sapient member) is therefore `fac.wild.space_opera`, and the sapient `centaur_archer` and `big_bad_wolf` belong to native factions (§8).
22. **Rest population.** Doc 12 §6.2 gives rest Folios "the Rubric's Keeper or Seal waves", while doc 14 §3.1 gives rest Folios no Rubric. Doc 14 owns Rubrics, so a rest Folio spawns nothing here.
23. **Doc 09 tag name.** Doc 09 §4.2 rule 8 reads a tag `flying`; this chapter's tag is `flier` (§2.4). Doc 09 should read `flier`.
24. **Alarm event.** Doc 09's `alarm_raised(pos, faction)` reaches the Director as a call; Canon §18 has no `ev.alarm_raised` (doc 09 gap G5).

## Acceptance tests

1. **Scaling.** Standard navy trooper, Folio 10, party 2, Grade 0: `hp_max = 330,460` mHP; a ×1 hit `B = 16,190` and 17,809 after step 2.14; AR 21; poise 46,000.
2. **Elite scaling.** Elite brute, Folio 25, party 4: `hp_max = 17,042,142`; `B = 84,579`, 109,952 after step 2.14; AR 118.
3. **Counts.** Base 5, party 4: four activations spawn 8, 9, 9, 9.
4. **Curves.** `logistic_up(5000) = 5000`, `logistic_up(3500) = 1868`, `bell_mid(5833) = 8859`, `plateau(10000) = 0`, `plateau(5000) = 10000`.
5. **Utility example.** The §4.5 brute scores melee 0, advance 2592, charge 10450, retreat 151; only charge is eligible (threshold 9405).
6. **Think draws.** A think that selects an action calls `range` exactly once on keyed `rng.ai` (seq 0); a think whose agent is in any §4.6 state other than `Idle` and `Moving`, and every asleep agent, draws nothing; the choice is identical whether 0 or 100 other agents thought earlier that tick. Cover, flank and flight cells draw nothing.
7. **Telegraph validation.** The content validator rejects an attack with `dmg_mult_bp` 20000 and `windup` 29 and accepts `windup` 30; it rejects 40000 with 44. Every attack with a telegraph carries `telegraphed`.
8. **Tokens.** Five standard melee agents against one Erratum: at most 3 are ever in `Windup` or `Active` at once; an elite in the squad attacks regardless.
9. **Path cost.** Three walks, one diagonal and one jump-up cost 6814. Two equal-cost paths resolve to the one through the lower cell index on every run.
10. **Digging.** `break_ticks` 120 with a sapper gives `dig_ticks = 60`. A sapper's path never includes a natural block that lacks the destructible tag.
11. **Hacking.** A hacked `security_droid` is in `fac.errata`; a 10,000 mHP rifle hit on it from an Erratum leaves `D2 = 5000`; on expiry it returns to `imperial_navy` and gains 5000 threat toward the hacker. A hack on a boss is rejected.
12. **Minion cap.** Conjuration 60, Handling 10: cap 3. A fourth tamed bind fails and emits no `ev.minion_bound`; a fourth spell summon releases the owner's lowest-EntityId spell summon first.
13. **Minion stats and taming.** A tamed troll in Folio 12 has 297,330 mHP, hits for 46,447 mHP and has AR 27. Taming a standard beast in Folio 12 has `tame_dc = 26`; CHM 50 and Handling 30 give `tame_value = 35` and the tame succeeds with no random draw; CHM 40 and Handling 20 (value 20) fail; an elite beast is untamable below Handling 75.
14. **Weapon pickup.** A Folio-10 soldier with an Epic Good rifle (interval 30): `pickup_bp = 12650`, 6,825 mHP per shot.
15. **Elite selection.** A brute promoted in Folio 3 has 9 eligible non-aspect modifiers; in Folio 6, 14; in Folio 11, 17. Each promotion makes exactly one `weighted` call, and every standard spawn makes exactly one `roll_bp(elite_promote_add_bp)` call, also at 0.
16. **Aspects.** `aspect_set(0x0123456789ABCDEF, 12, genre.mythic_fantasy, 12, false)`: `stream_seed = 0x546718c15ec8e4ab`, draws `range(5000) = 3382` and `range(3200) = 722`, set `[elite.warded, elite.surging]`; champions 0, 1 and 2 carry warded, surging and warded. The same call for Folio 3 Cyberpunk (`k = 1`, weights `[1200, 1500, 1900]`) gives `[elite.surging]`; for Folio 31 Wuxia (`k = 3`) it gives `[elite.surging, elite.warded, elite.unstoppable]`. A preview computed from a fresh copy of the stream equals the live Folio's set.
17. **Population plan.** Folio 16, standard, density 10000, no arena, keeper Rubric at position 1: 1 champion, 1 elite plus an elite Keeper, 26 minions, 12 standards; reserve 5 minions and 2 standards (13 and 6 under the Seal Rubric). Folio 10 Editor checkpoint: 17,400 mSKE as 1 elite, 19 minions, 8 standards. A Folio with a `room.arena` (not a siege) holds the mini-boss and no budget champion. A rest Folio spawns nothing. Apportionment of 26 minions over rooms with `spawn_slots` 17, 14 and 4 (sum 35): floor shares 12, 10 and 2 with remainders 22, 14 and 34 (in 35ths); the 2 leftovers go to the 4-slot and the 17-slot rooms, giving 13, 10 and 3.
18. **Intensity.** A 25,000 mHP hit on an 85,000 mHP Erratum adds 8823; a downed Erratum sets party intensity to 10000.
19. **Hunters.** Threshold 10: first hunter at `c = 41,400`; with 2 red-ink doses on one Erratum, `c = 37,800`; then one every 5,400 ticks, also after Colophon entry. No stack and no hunter while an Editor lives. Hunter HP solo at Folio 8, Grade 0: 2,234,148.
20. **Erasure of blocks.** Errata blocks at 1000, 1414 and 1600 mm from a hunter's point `c`: the first is erased on its next erasure tick, the second 10 ticks later, the third never; natural blocks are never erased.
21. **Kill XP.** Elite at Folio 10 pays 529; each member of a 4-party gets 302. Standard at Folio 25 pays 970. A summoned inkling pays 0. In `mode.abridged` the Folio-10 elite pays a solo player 881.
22. **Attribution.** An enemy killed by another faction after Errata dealt 2000 Bp of its Max HP pays 0; after 2500 Bp it pays full kill XP.
23. **Level pacing.** The §11.2 reference run gives cumulative XP 47,839 and level 13 after Folio 10, 480,822 and level 29 on arrival at Folio 25, main skill 50 at Folio 10 and 75 at Folio 22; every §11.2 sensitivity row lands in `25..=31`.
24. **Boss HP.** `boss.editor_v1` solo 7,113,600 mHP; `boss.ur_editor` with 4 players 359,677,296 mHP; a mini-boss in Folio 3 solo 2,148,000.
25. **Boss loot.** A mini-boss kill rolls doc 11's `loot.drop.boss` once per Active, living Erratum with `drop_rng(folio, boss EntityId, tick, slot)`; an Editor kill rolls `loot.drop.editor` the same way and opens `loot.volume_reward`; this chapter makes no other loot draw for bosses.
26. **Phase shift.** A hit taking `boss.editor_v1` from 6700 to 6500 Bp stops at 6667 Bp; for 90 ticks the boss takes 0 damage and loses every status; it publishes a `ring` telegraph and switches to its phase-2 attack set; no block changes.
27. **Reputation.** A witnessed `kingsguard_knight` kill: `kingsguard` 150 → 0 and `orc_warband` −600 → −563 after 300 ticks. If both witnesses die within 300 ticks: `kingsguard` 150 → 110 and `orc_warband` −600 → −590.
28. **Dialogue.** Persuade with CHM 52, Speech 35, reputation +150, Hard tier, Folio 12: `value = 42`, `dc = 36`, pass, no random draw, 3,600 milli-XP of Speech; with Speech 25 it fails and stays unavailable on that speaker until the value rises.
29. **Approval.** Six `valor` events in one Folio for Sir Brannoc add +20, not +24; at −50 on `ev.folio_exited` he leaves with `ev.minion_released`.
30. **Morale.** A standard sapient soldier at HP 2000 Bp whose low-HP event already brought morale to 8000 sees its squad leader die (5000), then an elite ally die (2500, no surrender because the rule is below 2500), then one more ally die (1000): it surrenders.
31. **NPC Contradiction.** Navy trooper in a Fairy Tale bleed zone: C 5, potency 5000, jam 1500 Bp, E 6.
32. **Editors.** Every `fac.editors` attack has empty `dep`; in every genre its C is 0 and E is 0.
33. **Mounts.** Killing a cavalry mount applies `st.knocked_down` to the rider; killing the rider pays the rider's kill XP and the mount pays 0.
34. **Determinism.** Two hosts running the same seed and input log for 36,000 ticks of a Folio with 120 agents produce identical blackboards, paths and `state_hash`.
35. **Target hysteresis.** Current target A with 4000 threat at 8000 mm, challenger B with 4000 at 2500 mm: `eff(B) = 5200 > 4400`, so the agent switches; at 3500 mm it does not.
36. **World Laws.** A `sect_disciple` standing in a Hardboiled Noir bleed zone cannot use `qinggong_leap` (C = 6); back in Wuxia it can.
37. **Colophon hunter.** With `colophon_marked` and `colophon_hunter_kills = 3`, the Folio-entry Colophon hunter draws exactly one non-aspect modifier on `rng.spawn` and idles until an Erratum enters the Colophon.
38. **Tag bits.** A `kin.undead` ghoul has `comp.tags` bits 0 and 9 set, value 513 plus its other tags. A `navy_trooper` has bit 23 (`enforcer`) set; any template with a bit above 23 set is rejected.
39. **Reference soldier.** A Folio-1 standard soldier's 10,000 mHP pierce hits on a 20-AR, 85 HP Erratum land 7,273 mHP each; the 12th, on tick 1,008, kills it.
40. **Flank candidates.** For a target with yaw 0 the 16 candidate bearings run from 16384 (i = 0) to 49152 (i = 15) in steps of `floor_div(i × 32768, 15)`; each kept candidate has `ang_diff > 10923`.
41. **Action state machine.** An agent in `Windup` that gains `st.feared` goes to `Feared`, halves its remaining cooldown and draws nothing until `st.feared` ends; one that gains `st.charmed` returns to `Idle` and attacks the applier's hostiles from its next think; a `Surrendered` agent that takes damage goes to `Fleeing` with `flee_until = u64::MAX`; a spared one becomes `fac.neutral`, emits `ev.entity_pacified` and despawns at the exit room or after 1800 ticks.
42. **Awareness mapping.** A pair at `aware.lost` gives `target_aware = 8000`, so `investigate` runs at run speed toward the predicted `lkp`; at `aware.curious` (2500), with no darkness disturbance and no sleeping ally nearby, only `idle` scores above 0 and the agent turns its head to `lkp`; `search` with 5 drawable plan points makes exactly 3 `weighted` calls on keyed `rng.ai` seq 1.
43. **Reflection caps.** Folio-25 elite brute (`B1 = 84,579`): a 500,000 mHP melee hit on a thorned one reflects 21,144 mHP; a 900,000 mHP projectile on a mirrored one deals it 0 and reflects 126,868 mHP to the shooter.
44. **Hasty floor.** `elite.hasty` on a brute whose `charge` has windup 36 and `dmg_mult_bp` 20000 gives windup 30 (not 29); on a 12-tick windup it gives 10.
45. **Faction draw.** In Mythic Fantasy with kingsguard at +150, a non-territory room's faction draw is one `weighted([1500, 1500, 3000, 2500])` call over editors, mage_conclave, orc_warband and wild; every member of the room then belongs to the drawn faction. A room whose territory faction is hostile draws `weighted([1])`.
46. **Director phases.** A standard Folio starts in `build_up`, a boss Folio in `boss`, a rest Folio in `exhausted`. In a Seal Folio with `R0 = 9,900` mSKE the three Seal waves take 3,300 mSKE each and no reserve wave ever fires.
47. **Invader.** `enemy.editors.invader` at Folio 10 against a party of 4, Grade 0: 10,698,668 mHP; a ×1.0 `strike_through` deals 33,675 mHP after step 2.14; it never erases a block, opens a container or picks up an item; `InvaderJoin { kit }` with an ID outside `enemy.editors.kit.*` is rejected.
48. **Prologue hunter.** `enemy.prologue_hunter` takes 0 damage from every type, runs at 5000 mm/s, hits for 6,000 mHP before armor at Folio 1, pays no XP and despawns on the tick the player's origin enters `room.colophon`.
49. **Hostile summons.** A hostile `summon.herald` spawns at `rank.elite` in `fac.wild.<local genre>` and pays elite kill XP; a hostile `summon.misprint` is in `fac.editors`, erases Errata placeables within 1500 mm and perceives through doc 09.
50. **Reputation precedence.** Killing a witnessed kingsguard vendor applies only `fac.rep.kill_vendor` (−500), not `fac.rep.kill_member`; every reputation change emits `ev.reputation_changed`.
51. **Content validity.** Every JSON example in this chapter parses, contains only integers, and validates against its schema; a template with `kin` set and a body other than `body.humanoid`, or `kin.mutant` without `mutation`, is rejected.
52. **Stream discipline.** A full Folio run emits draws only from `rng.spawn`, `rng.ai_idle`, `rng.ai`, `rng.combat`, `rng.loot`, `rng.magic` and `rng.status` among this chapter's systems; no doc 13 system calls a stream outside Canon §3.2 and §3.3.

## Invariants annex

```
Σ doc13: enemies ⊗ ai ⊗ squads ⊗ nav ⊗ director ⊗ factions ⊗ npcs
W! enemy power := f(d, rank, party, doc14 grade)            N! enemy stat reads any Erratum level|skill|gear
W! hp = tpl × H(d) × rank × party_hp × diff_hp; B = tpl × T(d) × rank; AR = tpl × P(d)
W! tpl.hp_mhp <= 400000                                     N! rank|party|difficulty tables on minions
W! minion hp = tpl × T(d) × 15000; minion dmg = tpl × P(d)  W! summons := doc 06 §12.1
W! ai ⊂ host ∧ stages ⊂ sys.ai_think                        W! iteration := ascending EntityId
W! selecting think := keyed rng.ai(seq 0), exactly 1 range  N! draws in asleep|committed think
W! population|waves|promotions := rng.spawn; idle := rng.ai_idle  N! HashMap|HashSet ∈ blackboard
W! score := Π adj(c_i) × weight; select ∈ {s ≥ 0.9·best}    N! float ∈ curves|scores
W! dmg_mult ≥ 20000 ⇒ windup ≥ 30; ≥ 40000 ⇒ windup ≥ 45   N! untelegraphed big hit
W! tokens: melee 3, ranged 4, control 1, grenade 1 per target  N! token for elite|champion|boss|editor|hunter
W! threat := doc 05 §14.1; switch iff eff > 1.1 × eff(cur)  W! A* tie := (f, h, cell index)
N! path expansions > 8192 per tick
W! dig ⊂ destructible ∪ placed_by_errata                     N! dig inside a sealed arena
W! hunter knows all Errata; others perceive via doc 09       N! omniscience elsewhere
W! erase := placed_by_errata ∧ dist ≤ 1500 mm, 1 block / 10 ticks / member   N! invader erases | breaks | places blocks
W! fac.editors attack dep = ∅                                N! Contradiction|Edge on Editors
W! law_bound ∧ ¬law active ⇒ C = 6
W! aspect set := first k weighted draws of rng.spawn (§7.3)    W! champion i aspect := set[i % len]
W! population := §10.1 budget over doc 12 spawn_slots + Keeper + scripted waves   W! mini-boss ⊂ room.arena ∧ paid from budget
W! one room pack := one faction                                 W! fac.native ∋ sapient template ∨ vendor
W! POP_BASE_MSKE = 29000 ∧ MINION_SHARE_BP = 4000              N! rest Folio population
W! summoned ⇒ kill XP 0                                      W! kill XP needs Errata share ≥ 2500 Bp or killing blow
W! pacify pays kill XP once                                  W! arrival level at Folio 25 ∈ 25..=31
W! reputation ∈ -1000..=1000, party-shared, per run          N! spillover chains
W! dialogue ∧ tame := value ≥ dc, deterministic               N! random draw ∈ dialogue | tame
N! LLM text → sim state                                      N! stream ∉ Canon §3.2 ∪ §3.3
W! st.hacked ⇒ fac.errata ∧ ¬minion cap                      N! hacked boss|editor|hunter
W! boss phase shift = 90 ticks invulnerable                  W! Erasure clock := doc 14 §3.8
W! boss loot := doc 11 loot.drop.<rank> ∪ loot.volume_reward  N! doc 13-owned loot table
W! reflected dmg ≤ cap(B1)                                    W! hasty windup ≥ §1 rule-3 floor
W! committed think ⇒ 0 draws; cover|flank|flight cell ⇒ 0 draws  W! awareness inputs := doc 09 §6.1 values
```
