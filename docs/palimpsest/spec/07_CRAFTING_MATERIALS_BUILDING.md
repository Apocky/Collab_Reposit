# 07 — Crafting, Materials and Building
As of 2026-10-11. Status: BINDING. Conforms to canon-1.2.

This chapter owns the one material table (Canon §13.8a), crafting quality, stations, recipes, assembly and parts, durability, mining and gathering, blocks and Errata building, gadgets and cooking. It was also checked against canon-1.4: limited building (Canon §4.3), `skill.fieldcraft` in place of Delving, and Margin trophies. Where 1.2 and 1.4 differ, 1.4 wins (Canon gaps 1).

## Imports

| Source | Sections | Consumed |
|---|---|---|
| Canon | §1.1–§1.6, §1.11 | ID grammar, `id32`, numeric types, `mul_bp`, `floor_div`, `ceil_div`, `round_half_even`, display-unit rule, integer content, ascending-ID iteration |
| Canon | §2.2, §2.3 | tick schedule, rate accumulator, `in_combat`, `light_at` |
| Canon | §3.1–§3.5 | `Rng`, `bell3`, keyed streams `rng.craft`, `rng.combat`, `rng.loot`, offer streams |
| Canon | §4.3 | fixed terrain, `destructible`, `cat.placeable`, resource nodes, Fieldcraft placement |
| Canon | §5.3–§5.7, §5.9 | dependencies, Contradiction, gadget spark failure, TECHNE recharge term, genre materials and manufacturers |
| Canon | §6.4 rows 10, 12, 13, 17, 28 | carry capacity, weapon scaling, crit chance, action speed, skill XP multiplier |
| Canon | §7, §9.1, §10.3–§10.5, §11.1, §11.4, §11.6 | skills, severed parts, armor and source magnitude (`material_bp`, `mat_ar_bp`), damage pipeline, statuses, direct application, Editor erasure |
| Canon | §12.1–§12.3 | surfaces (`block.ice`, `block.web`), `conceal`, spawn light |
| Canon | §13.1, §13.3, §13.5, §13.7, §13.8, §13.8.1, §13.9 | categories, rarity, quality, enhancement, `P`, `T`, `S`, `src_lvl`, item instance contract |
| Canon | §16.3, §17.1, §17.5, §18, §19 | kin fae and construct, Colophon, friendly fire, events, namespace ownership |
| Doc 03 | §4.2–§4.7, §12, §13 | `BlockState`, shapes, block registry schema, explosions, `set_block`, mining progress, block damage, placement checks, `timed_revert` |
| Doc 04 | §1.2, §5.1–§5.3, §7, §8.6, §11.2 | modifier kinds, skill XP events and mastery perks, Artificer talents, body-part butchery, food values |
| Doc 05 | §3.2, §3.3, §5.2, §7, §11, §12, §15 | archetype stats and `ref_mass_g`, mass-driven timing, `block_bp`, wallpen, ricochet, armor and shield tables, ward generators |
| Doc 06 | §9, §13, §16 | enchant capacity from arcane affinity, conjured walls, `shape_power`, offering altars |
| Doc 08 | §5, §8.1, §8.4, §9 | liquids, ingredient schema and tags, `ing.seed_*`, `station.alchemy` apparatus |
| Doc 10 | §2, §7 | crafted items known; `curse.brittle` |
| Doc 11 | §2.3, §3.6, §5, §13.1, §14, §15, §16 | generation step 7, gadget loot stats, manufacturers and the 22 loose parts, value, stacking, Rewrite, reforging, salvage |
| Doc 12 | §2.1, §2.2, §2.4, §15 | `hard_bp`, `fixture_level`, genre material pools, craft rooms |
| Doc 13 | §4.13, §10.6 | navigation links, `break_ticks`, `dig_ticks`, Errata-block erasure |
| Doc 14 | §5, §9.3 | `unlock.loot.recipes_2`, `unlock.loot.gadgets_2`, `run.marg.brittle_kit`, `d_eff` |

## 1. Conventions, constants and owned state

### 1.1 Owned ID forms (Canon §19: `mat.`, `q.`, `station.`, `recipe.`, `part.`, `block.`, `tooltier.`, `gadget.`)

| Form | Meaning | Count |
|---|---|---|
| `mat.<name>` | a material (§2) | 69 |
| `mat.<metal>.ore` | raw ore of a smeltable metal (§2.6) | 10 |
| `mat.role.<role>` | material role enum: `blade`, `head`, `haft`, `limb`, `frame`, `plate`, `soft`, `focus`, `block` | 9 |
| `mat.tag.<tag>` | material tag enum (§2.4) | 21 |
| `tooltier.t0` … `tooltier.t5` | tool tiers (§9.1) | 6 |
| `station.<name>` | crafting stations (§5) | 9 |
| `recipe.kind.<kind>` | `shaped`, `shapeless`, `assembly`, `process`, `disassembly` | 5 |
| `recipe.<group>.<name>` | recipes (§6.6) | 96 |
| `part.<slot>.<name>` | loose parts (§7.2) | 28 |
| `part.graft.<genre_suffix>.<mfr_name>` | licensed grafts (§7.5) | 38 |
| `block.<…>`, `block.tag.<tag>` | blocks (§10.1) and block tags | §10.1 |
| `gadget.<name>` | gadgets and deployables (§11) | 12 |

`q.*` stays the Canon §13.5 core list. This chapter adds no quality ID.

### 1.2 Constants

| Constant | Value | Use |
|---|---|---|
| `LIGHT_DENSITY_MAX` | 1600 kg/m³ | materials at or below it come in 1000 ml units, others in 250 ml units (§2.5) |
| `UNIT_ML_DENSE`, `UNIT_ML_LIGHT` | 250, 1000 | material unit volumes |
| `DUR_BASE` | weapons, armor, shields, ward generators, lanterns 10000; tools 5000; reusable gadgets 4000; thrown 40; wands, foci, tomes 0 (no durability) | §8.1 |
| `WEAR_RATE` | 25 durability per second of attack cycle | §8.2 |
| `CROSS_MFR_BP` | 7500 | part delta on an item of another manufacturer (§7.4) |
| `GRAFT_BP` | 7500 | licensed graft gimmick strength (§7.5) |
| `CRAFT_REACH_MM` | 3000 | crafter to station distance |
| `STATION_QUEUE_MAX` | 4 | process jobs per station |
| `HELPERS_MAX` | 2 | crafting helpers counted |
| `PLACED_CAP_PER_PLAYER` | 48 | live Errata-placed blocks per party slot |
| `PLACED_CAP_PER_FOLIO` | 160 | live Errata-placed blocks per Folio |
| `MEAL_BUFF_TICKS` | 18000 | meal buff duration (§12) |
| `TORCH_TICKS` | 18000 | placed torch burn time |
| `CROP_STAGE_TICKS` | 1200 | base ticks per crop stage (§9.7) |

### 1.3 Random draws

| Draw | Stream | Key | Count |
|---|---|---|---|
| Crafting quality and crafted perk seed (§4) | keyed `rng.craft` | actor = crafter, tick = job start tick, seq = output index | 4 per quality-rolling output: 3 `range(141)`, 1 `next_u64`; 0 for outputs that do not roll |
| Mining bonus unit (§9.3) | keyed `rng.craft` | actor = miner, tick = break tick, seq = 0 | 1 `roll_bp` per node broken |
| Crop seed return below Survival 25 (§9.7) | keyed `rng.craft` | actor = harvester, tick = harvest tick, seq = 0 | 1 `roll_bp` |
| Gadget Contradiction failure (§11.1) | keyed `rng.combat` | actor = deployer, tick, seq | 1 `roll_bp` per deploy while failure chance > 0 |
| Turret and drone shots (§11.4) | keyed `rng.combat` | actor = gadget EntityId, tick, seq = shot index that tick | 2 spread + 1 crit per projectile hit, as Canon §3.4 |
| Salvage loose part (§8.5) | keyed `rng.loot` | doc 11 §16.4 | doc 11 |
| Forge reforge offers (§8.6) | `rng.offer.p<slot>` | doc 11 §16.3 | doc 11 |

Every other mechanic in this chapter is deterministic.

### 1.4 Owned simulation state (saved, Canon §2.5)

| State | Scope | Content |
|---|---|---|
| `known_recipes: BTreeSet<Id32>` | per Erratum, run | learned recipes with `learn: 1` (§6.5) |
| `craft_job: Option<CraftJob>` | per Erratum | the active channel (§6.4) |
| `meal_buff: Option<(Id32 dish, Tick expiry)>` | per Erratum | §12.1 |
| `placed_meta: BTreeMap<Gidx, PlacedMeta>` | Folio | owner slot, `hp`, `hp_max`, expiry tick, patron, span, conjured flag (§10.4) |
| `stations: BTreeMap<Gidx, StationState>` | Folio | tier, fuel ticks, job queue, output slots (§5.4) |
| `crops: BTreeMap<Gidx, CropState>` | Folio | seed, stage, next stage tick (§9.7) |
| gadget entities | Folio | §11.3 |

Folio-scoped state is discarded on `ev.folio_exited`. Doc 07 runs one system, `sys.crafting` (period 1, phase 0), placed by doc 03 after `sys.actions` and before `sys.block_updates`; it advances craft channels, station queues, crops, torch expiry and gadget timers, each in ascending `Gidx` or `EntityId`.

## 2. Materials

### 2.1 Schema

```json
{
  "id": "mat.iron",
  "density": 7870, "hardness": 5, "toughness": 6, "sharpness": 5, "conductivity": 6,
  "flammability_bp": 0, "melt_decic": 15380, "conceal": 15, "pen_cost": 300,
  "tier": 1, "min_d": 1, "max_d": 20,
  "value_per_kg": 3, "value_bp": 10000, "dmg_bp": 10000, "ar_bp": 10000,
  "arcane_affinity": 2, "tech_affinity": 5,
  "genres": ["genre.weird_west"], "universal": 1,
  "roles": ["mat.role.blade", "mat.role.head", "mat.role.frame", "mat.role.plate", "mat.role.block"],
  "tags": ["mat.tag.metal", "mat.tag.ricochet", "mat.tag.fae_bane"]
}
```

Validation: `dmg_bp` and `ar_bp` in `7000..=14000`; `mat.iron` has `dmg_bp = ar_bp = value_bp = 10000`; `hardness`, `toughness`, `sharpness`, `conductivity`, `arcane_affinity`, `tech_affinity` in `0..=10`; `conceal` in `0..=15`; `melt_decic` absent means the material never melts. Every material has dependency none (Canon §5.3 raw materials).

Properties: **density** (kg/m³, equal to g/l) gives mass; **hardness** gives mining time, block hardness and tool tier; **toughness** gives durability, blast resistance and armor poise; **sharpness** gives bleed power on edged weapons; **conductivity** gives heat conductance and armor shock resistance; **arcane affinity** gives enchant capacity (doc 06 §9.1) and casting cost; **tech affinity** gives reload and gadget recharge; **flammability** feeds fire (doc 03 §8); **melt point** gates smelting stations; **value per kg** prices material units; **tier** and **band** gate loot and stations.

### 2.2 Table A: physical properties

`Unit ml / g` is the unit volume and unit mass (§2.5). `pen_cost` feeds doc 05 §7 wall penetration.

| `mat.` | ρ kg/m³ | Unit ml / g | Hard | Tough | Sharp | Cond | Flam Bp | Melt DeciC | Conceal | pen_cost |
|---|---|---|---|---|---|---|---|---|---|---|
| copper | 8960 | 250 / 2240 | 3 | 5 | 3 | 10 | 0 | 10850 | 15 | 250 |
| tin | 7290 | 250 / 1822 | 2 | 3 | 1 | 6 | 0 | 2320 | 15 | 200 |
| zinc | 7140 | 250 / 1785 | 3 | 3 | 1 | 6 | 0 | 4200 | 15 | 210 |
| bronze | 8800 | 250 / 2200 | 4 | 6 | 5 | 6 | 0 | 9500 | 15 | 260 |
| iron | 7870 | 250 / 1967 | 5 | 6 | 5 | 6 | 0 | 15380 | 15 | 300 |
| cold_iron | 7870 | 250 / 1967 | 5 | 6 | 5 | 6 | 0 | 15380 | 15 | 300 |
| scrap_iron | 7200 | 250 / 1800 | 4 | 4 | 4 | 6 | 0 | 15000 | 15 | 260 |
| cast_iron | 7200 | 250 / 1800 | 5 | 3 | 2 | 5 | 0 | 11500 | 15 | 280 |
| steel | 7850 | 250 / 1962 | 6 | 7 | 7 | 5 | 0 | 14250 | 15 | 350 |
| brass | 8500 | 250 / 2125 | 4 | 5 | 3 | 7 | 0 | 9300 | 15 | 240 |
| silver | 10490 | 250 / 2622 | 3 | 4 | 4 | 10 | 0 | 9620 | 15 | 240 |
| hexed_silver | 10490 | 250 / 2622 | 4 | 5 | 5 | 9 | 0 | 9620 | 15 | 260 |
| gold | 19300 | 250 / 4825 | 2 | 4 | 2 | 9 | 0 | 10640 | 15 | 220 |
| chrome | 7190 | 250 / 1797 | 6 | 6 | 6 | 6 | 0 | 19070 | 15 | 360 |
| titanium | 4510 | 250 / 1127 | 7 | 8 | 6 | 3 | 0 | 16680 | 15 | 400 |
| moonsilver | 6000 | 250 / 1500 | 6 | 6 | 7 | 8 | 0 | 12000 | 15 | 330 |
| plasteel | 3500 | 250 / 875 | 7 | 8 | 4 | 4 | 0 | 24000 | 15 | 450 |
| orichalcum | 7000 | 250 / 1750 | 8 | 8 | 8 | 7 | 0 | 18000 | 15 | 480 |
| star_metal | 9500 | 250 / 2375 | 9 | 8 | 9 | 6 | 0 | 30000 | 15 | 600 |
| aetherium | 3000 | 250 / 750 | 7 | 6 | 6 | 10 | 0 | 20000 | 15 | 420 |
| redacted_alloy | 5000 | 250 / 1250 | 9 | 9 | 8 | 0 | 0 | — | 15 | 700 |
| gold_leaf | 19300 | 250 / 4825 | 1 | 2 | 0 | 9 | 0 | 10640 | 15 | 60 |
| pine | 500 | 1000 / 500 | 2 | 4 | 1 | 1 | 4000 | — | 15 | 30 |
| birch | 670 | 1000 / 670 | 2 | 5 | 1 | 1 | 3500 | — | 15 | 35 |
| cedar | 380 | 1000 / 380 | 2 | 4 | 1 | 1 | 3000 | — | 15 | 30 |
| bamboo | 700 | 1000 / 700 | 3 | 6 | 2 | 1 | 3500 | — | 15 | 35 |
| oak | 750 | 1000 / 750 | 3 | 6 | 1 | 1 | 2500 | — | 15 | 40 |
| thornwood | 900 | 1000 / 900 | 4 | 6 | 3 | 1 | 3000 | — | 15 | 45 |
| blackwood | 1200 | 1000 / 1200 | 4 | 7 | 2 | 1 | 1500 | — | 15 | 60 |
| elderwood | 600 | 1000 / 600 | 3 | 6 | 1 | 1 | 2000 | — | 15 | 40 |
| granite | 2700 | 250 / 675 | 6 | 5 | 2 | 2 | 0 | 12150 | 15 | 130 |
| sandstone | 2300 | 250 / 575 | 3 | 3 | 2 | 2 | 0 | 12000 | 15 | 90 |
| slate | 2800 | 250 / 700 | 4 | 3 | 4 | 2 | 0 | 12000 | 15 | 110 |
| basalt | 3000 | 250 / 750 | 6 | 5 | 2 | 3 | 0 | 11000 | 15 | 140 |
| marble | 2700 | 250 / 675 | 4 | 4 | 2 | 2 | 0 | 12000 | 15 | 120 |
| obsidian | 2400 | 250 / 600 | 5 | 1 | 10 | 1 | 0 | 11000 | 15 | 120 |
| jade | 3300 | 250 / 825 | 6 | 7 | 3 | 1 | 0 | — | 15 | 150 |
| brick | 1900 | 250 / 475 | 4 | 4 | 1 | 2 | 0 | — | 15 | 150 |
| concrete | 2400 | 250 / 600 | 5 | 4 | 1 | 2 | 0 | — | 15 | 180 |
| terracotta | 1900 | 250 / 475 | 3 | 2 | 1 | 1 | 0 | — | 15 | 100 |
| glass | 2500 | 250 / 625 | 3 | 1 | 8 | 1 | 0 | 14000 | 0 | 2 |
| rad_glass | 2700 | 250 / 675 | 3 | 1 | 8 | 2 | 0 | 14000 | 1 | 3 |
| psi_crystal | 2650 | 250 / 662 | 7 | 3 | 7 | 4 | 0 | 17000 | 2 | 160 |
| bone | 1900 | 250 / 475 | 3 | 4 | 4 | 1 | 0 | — | 15 | 90 |
| dragonbone | 1500 | 1000 / 1500 | 8 | 8 | 7 | 1 | 0 | — | 15 | 300 |
| chitin | 1300 | 1000 / 1300 | 4 | 5 | 5 | 1 | 500 | — | 15 | 80 |
| hide | 1100 | 1000 / 1100 | 1 | 5 | 0 | 1 | 2000 | — | 15 | 25 |
| leather | 860 | 1000 / 860 | 2 | 5 | 0 | 1 | 1500 | — | 15 | 30 |
| linen | 1500 | 1000 / 1500 | 1 | 3 | 0 | 1 | 6000 | — | 15 | 10 |
| tweed | 1300 | 1000 / 1300 | 1 | 4 | 0 | 1 | 4500 | — | 15 | 12 |
| silk | 1300 | 1000 / 1300 | 1 | 6 | 0 | 1 | 5000 | — | 15 | 10 |
| rubber | 1100 | 1000 / 1100 | 2 | 6 | 0 | 0 | 4000 | — | 15 | 40 |
| polymer | 1200 | 1000 / 1200 | 3 | 6 | 1 | 0 | 3000 | 2500 | 15 | 60 |
| carbon_fiber | 1600 | 1000 / 1600 | 6 | 8 | 4 | 7 | 1000 | — | 15 | 200 |
| nanoweave | 1000 | 1000 / 1000 | 4 | 9 | 0 | 1 | 500 | — | 15 | 150 |
| coral | 1400 | 1000 / 1400 | 3 | 3 | 5 | 1 | 0 | — | 15 | 70 |
| flesh | 1060 | 1000 / 1060 | 1 | 2 | 0 | 2 | 500 | — | 15 | 15 |
| web | 1300 | 1000 / 1300 | 1 | 3 | 0 | 0 | 9000 | — | 3 | 1 |
| paper | 800 | 1000 / 800 | 1 | 1 | 0 | 0 | 9000 | — | 15 | 8 |
| vellum | 900 | 1000 / 900 | 5 | 7 | 2 | 0 | 2000 | — | 15 | 200 |
| dreamstuff | 400 | 1000 / 400 | 4 | 6 | 6 | 3 | 1000 | — | 6 | 100 |
| air | 1 | 1000 / 1 | 0 | 0 | 0 | 0 | 0 | — | 0 | 0 |
| void | 99999 | 250 / 24999 | 10 | 10 | 0 | 0 | 0 | — | 15 | ∞ |
| soil | 1500 | 1000 / 1500 | 1 | 2 | 0 | 1 | 0 | — | 15 | 60 |
| sand | 1600 | 1000 / 1600 | 1 | 1 | 0 | 1 | 0 | 17000 | 15 | 50 |
| ice | 917 | 1000 / 917 | 2 | 2 | 3 | 2 | 0 | 10 | 2 | 20 |
| leaves | 300 | 1000 / 300 | 1 | 1 | 0 | 0 | 7000 | — | 3 | 1 |
| coal | 1350 | 1000 / 1350 | 2 | 2 | 0 | 3 | 8000 | — | 15 | 70 |
| slag | 2800 | 250 / 700 | 4 | 2 | 1 | 3 | 0 | 13000 | 15 | 140 |

### 2.3 Table B: gameplay properties

`Band` is `min_d..=max_d` for loot and node placement (`min_d..` has no upper bound). `Unit Lucre` is the unit value at `P(1)` (§2.5). Genres: MF mythic_fantasy, GH gothic_horror, CH cosmic_horror, SO space_opera, CP cyberpunk, PA post_apocalypse, WW weird_west, HN hardboiled_noir, WX wuxia, SP steampunk, FT fairy_tale, MA mythic_antiquity, ON oneiric, RD redacted, UW unwritten, UT ur_text; `*` marks a universal material (§2.7). Roles: bl blade, hd head, ha haft, li limb, fr frame, pl plate, so soft, fo focus, bk block. Tags are written without the `mat.tag.` prefix.

| `mat.` | Tier | Band | Lucre/kg | Unit Lucre | value_bp | dmg_bp | ar_bp | Arc | Tech | Genres | Roles | Tags |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| copper | 1 | 1..=20 | 4 | 8 | 8000 | 8000 | 8000 | 4 | 6 | * | bl hd fr pl | metal ricochet |
| tin | 1 | 1..=20 | 4 | 7 | 7000 | 7000 | 7000 | 2 | 4 | * | — | metal ricochet alloying |
| zinc | 1 | 1..=20 | 3 | 5 | 7000 | 7000 | 7000 | 1 | 4 | * | — | metal ricochet alloying |
| bronze | 1 | 1..=20 | 3 | 6 | 9000 | 9000 | 9000 | 4 | 4 | WX MA | bl hd fr pl | metal ricochet |
| iron | 1 | 1..=20 | 3 | 5 | 10000 | 10000 | 10000 | 2 | 5 | WW * | bl hd fr pl bk | metal ricochet fae_bane |
| cold_iron | 2 | 4.. | 8 | 15 | 12000 | 10500 | 10000 | 0 | 3 | FT | bl hd fr pl | metal ricochet fae_bane |
| scrap_iron | 1 | 1..=20 | 1 | 1 | 6000 | 9000 | 9000 | 1 | 5 | PA | bl hd fr pl bk | metal ricochet |
| cast_iron | 1 | 1..=20 | 3 | 5 | 9000 | 9000 | 10500 | 1 | 6 | SP | hd fr pl bk | metal ricochet brittle |
| steel | 2 | 4.. | 6 | 11 | 13000 | 11000 | 11000 | 2 | 6 | MF HN * | bl hd fr pl li bk | metal ricochet |
| brass | 2 | 4.. | 6 | 12 | 12000 | 9000 | 9000 | 3 | 8 | HN SP | hd fr pl | metal ricochet |
| silver | 2 | 4.. | 7 | 18 | 25000 | 9000 | 8500 | 8 | 3 | GH | bl hd fr fo | metal ricochet silvered arcane_reagent |
| hexed_silver | 2 | 6.. | 8 | 20 | 30000 | 10500 | 9500 | 9 | 2 | WW | bl hd fr fo | metal ricochet silvered arcane_reagent |
| gold | 2 | 4.. | 4 | 19 | 40000 | 7500 | 7000 | 7 | 5 | * | hd fo | metal ricochet arcane_reagent |
| chrome | 2 | 4.. | 10 | 17 | 14000 | 11500 | 11000 | 1 | 8 | CP | bl hd fr pl | metal ricochet |
| titanium | 3 | 9.. | 30 | 33 | 18000 | 12000 | 12500 | 2 | 8 | SO CP | bl hd fr pl li | metal ricochet |
| moonsilver | 3 | 9.. | 30 | 45 | 32000 | 12000 | 11500 | 10 | 2 | MF | bl hd fr pl fo | metal ricochet silvered arcane_reagent |
| plasteel | 3 | 9.. | 25 | 21 | 18000 | 12000 | 13500 | 1 | 10 | SO | fr pl bk | metal ricochet |
| orichalcum | 4 | 14.. | 60 | 105 | 36000 | 13500 | 13000 | 8 | 6 | MA | bl hd fr pl fo | metal ricochet arcane_reagent |
| star_metal | 4 | 14.. | 40 | 95 | 40000 | 14000 | 13000 | 6 | 7 | CH | bl hd fr pl | metal ricochet |
| aetherium | 4 | 14.. | 150 | 112 | 38000 | 12500 | 12000 | 9 | 10 | SP | fr fo | metal ricochet arcane_reagent luminous |
| redacted_alloy | 5 | 20.. | 120 | 150 | 50000 | 13500 | 14000 | 0 | 9 | RD | bl hd fr pl | metal ricochet |
| gold_leaf | 5 | 20.. | 30 | 144 | 50000 | 7000 | 7000 | 10 | 2 | UT | fo | metal arcane_reagent |
| pine | 0 | 1..=8 | 1 | 1 | 7000 | 8000 | 7000 | 2 | 1 | * | ha li bk | wood flammable fuel acid_soluble |
| birch | 0 | 1..=8 | 1 | 1 | 7500 | 8500 | 7500 | 4 | 1 | FT * | ha li fo | wood flammable fuel acid_soluble |
| cedar | 1 | 1..=20 | 2 | 1 | 8500 | 8500 | 7500 | 3 | 1 | WX | ha li bk | wood flammable fuel acid_soluble |
| bamboo | 1 | 1..=20 | 2 | 1 | 8500 | 9000 | 8000 | 3 | 2 | WX | ha li bk | wood flammable fuel acid_soluble |
| oak | 1 | 1..=20 | 2 | 1 | 10000 | 10000 | 8500 | 4 | 2 | MF * | ha li fo pl bk | wood flammable fuel acid_soluble |
| thornwood | 2 | 4.. | 6 | 5 | 14000 | 10500 | 9000 | 7 | 1 | FT | ha li fo | wood flammable fuel acid_soluble |
| blackwood | 3 | 9.. | 25 | 30 | 18000 | 11500 | 9500 | 7 | 1 | GH | ha li fo pl | wood flammable fuel acid_soluble |
| elderwood | 3 | 9.. | 50 | 30 | 24000 | 12000 | 9500 | 10 | 0 | FT | ha li fo | wood flammable fuel acid_soluble arcane_reagent |
| granite | 1 | 1..=20 | 1 | 1 | 6000 | 8500 | 8500 | 1 | 1 | MF UT * | hd bk | stone ricochet |
| sandstone | 0 | 1..=8 | 1 | 1 | 5000 | 7500 | 7500 | 1 | 0 | WW * | hd bk | stone ricochet |
| slate | 1 | 1..=20 | 1 | 1 | 6000 | 8000 | 8000 | 2 | 1 | GH | hd bk | stone ricochet |
| basalt | 1 | 1..=20 | 1 | 1 | 6000 | 9000 | 9000 | 1 | 1 | CH | hd bk | stone ricochet |
| marble | 1 | 1..=20 | 3 | 2 | 9000 | 8000 | 8500 | 4 | 1 | MA * | hd bk | stone ricochet |
| obsidian | 2 | 4.. | 12 | 7 | 14000 | 12000 | 7500 | 5 | 1 | CH | bl fo | stone ricochet brittle |
| jade | 3 | 9.. | 40 | 33 | 28000 | 9500 | 10000 | 8 | 1 | WX | hd fo pl | stone ricochet arcane_reagent |
| brick | 0 | 1..=8 | 1 | 1 | 5000 | 7500 | 8000 | 1 | 1 | HN SP * | bk | stone ricochet |
| concrete | 0 | 1..=8 | 1 | 1 | 5000 | 8000 | 9000 | 0 | 2 | CP PA RD * | hd bk | stone ricochet |
| terracotta | 0 | 1..=8 | 2 | 1 | 6000 | 7500 | 7500 | 3 | 1 | MA | hd bk | stone brittle |
| glass | 1 | 1..=20 | 4 | 2 | 8000 | 11000 | 7000 | 3 | 4 | FT HN | bl fo | brittle |
| rad_glass | 2 | 4.. | 10 | 6 | 10000 | 11500 | 7000 | 1 | 6 | PA | bl fo | brittle luminous |
| psi_crystal | 4 | 14.. | 150 | 99 | 36000 | 12500 | 9000 | 10 | 8 | SO | bl fo | brittle arcane_reagent luminous |
| bone | 1 | 1..=20 | 2 | 1 | 7000 | 8500 | 8500 | 5 | 1 | GH * | bl hd ha li | ricochet |
| dragonbone | 4 | 14.. | 70 | 105 | 36000 | 13500 | 13500 | 8 | 1 | MF | bl hd ha li pl | arcane_reagent |
| chitin | 2 | 4.. | 8 | 10 | 10000 | 10000 | 11000 | 3 | 2 | CH | bl li pl so | acid_soluble |
| hide | 0 | 1..=8 | 4 | 4 | 6000 | 7000 | 8500 | 2 | 1 | * | so | flammable acid_soluble |
| leather | 1 | 1..=20 | 8 | 6 | 8000 | 9000 | 10000 | 3 | 2 | WW * | so ha | flammable acid_soluble |
| linen | 0 | 1..=8 | 3 | 4 | 6000 | 7000 | 10000 | 3 | 1 | * | so | fabric flammable acid_soluble |
| tweed | 1 | 1..=20 | 5 | 6 | 9000 | 7000 | 10500 | 2 | 2 | HN | so | fabric flammable acid_soluble |
| silk | 2 | 4.. | 12 | 15 | 16000 | 7500 | 11000 | 6 | 2 | WX | so li | fabric flammable acid_soluble |
| rubber | 1 | 1..=20 | 6 | 6 | 8000 | 7000 | 9500 | 0 | 6 | PA | so ha | flammable fuel |
| polymer | 2 | 4.. | 8 | 9 | 10000 | 9000 | 10000 | 0 | 9 | CP SO RD PA | fr ha so bk | flammable fuel |
| carbon_fiber | 3 | 9.. | 25 | 40 | 18000 | 11500 | 12500 | 1 | 10 | CP | fr ha li pl | flammable |
| nanoweave | 4 | 14.. | 100 | 100 | 30000 | 12000 | 13000 | 2 | 10 | CP | so | fabric |
| coral | 2 | 4.. | 10 | 14 | 12000 | 9500 | 9000 | 6 | 1 | CH | bl hd fo | stone |
| flesh | 0 | 1.. | 1 | 1 | 2000 | 7000 | 7000 | 1 | 0 | * | — | edible fat acid_soluble terrain |
| web | 0 | 1.. | 6 | 7 | 6000 | 7000 | 7000 | 4 | 0 | * | so | flammable acid_soluble terrain |
| paper | 0 | 1..=8 | 2 | 1 | 5000 | 7000 | 7000 | 6 | 1 | RD UW * | so fo | flammable fuel acid_soluble |
| vellum | 5 | 20.. | 150 | 135 | 50000 | 13000 | 13000 | 10 | 3 | UW UT | so fo pl | arcane_reagent |
| dreamstuff | 5 | 20.. | 350 | 140 | 50000 | 13000 | 12500 | 10 | 5 | ON | bl hd fr pl so fo | arcane_reagent luminous |
| air | 0 | 1.. | 0 | 0 | 0 | 7000 | 7000 | 0 | 0 | * | — | nonsolid terrain |
| void | 0 | 1.. | 0 | 0 | 0 | 7000 | 7000 | 0 | 0 | * | — | unbreakable terrain |
| soil | 0 | 1.. | 0 | 0 | 0 | 7000 | 7000 | 1 | 0 | * | bk | absorbent tillable terrain |
| sand | 0 | 1.. | 0 | 0 | 0 | 7000 | 7000 | 0 | 1 | * | bk | absorbent terrain |
| ice | 0 | 1.. | 0 | 0 | 0 | 9000 | 7500 | 2 | 0 | * | bl | brittle ricochet terrain |
| leaves | 0 | 1.. | 0 | 0 | 0 | 7000 | 7000 | 3 | 0 | * | — | flammable fuel absorbent acid_soluble terrain |
| coal | 0 | 1.. | 1 | 1 | 3000 | 7000 | 7000 | 0 | 3 | * | — | flammable fuel |
| slag | 0 | 1.. | 0 | 0 | 1000 | 8000 | 8000 | 0 | 2 | * | bk | stone ricochet terrain |

### 2.4 Roles and tags

Roles say which item component a material may form. Reference materials sit at 10000 for their role: `mat.iron` for blade, head, frame and plate; `mat.oak` for haft, limb and focus; `mat.leather` and `mat.linen` for soft.

| Tag | Rule |
|---|---|
| `metal`, `wood`, `stone`, `fabric` | families; `fabric` soft materials make cloth armor, other soft materials make light armor |
| `ricochet` | doc 05 §7 ricochet faces |
| `flammable` | `flammability_bp >= 500`; doc 03 fire, doc 05 and doc 06 hooks |
| `fuel` | burns in stations (§5.4); `kin.construct` refuels from it (Canon §16.3) |
| `brittle` | the item shatters at durability 0 (§8.3) |
| `alloying` | crafting input only, never a loot primary |
| `arcane_reagent` | doc 06 §9.3 enchanting reagent |
| `silvered`, `fae_bane` | banes (§3.2) |
| `luminous` | blocks of it emit `light_block` 7 |
| `edible`, `fat` | `mat.flesh` is raw meat (doc 04 §11.2) and an alchemy fat |
| `absorbent`, `acid_soluble`, `tillable` | doc 08 §2 and §5; planting (§9.7) |
| `terrain`, `nonsolid`, `unbreakable` | never loot, never a crafted primary; `air` is non-solid; `void` is unbreakable |

### 2.5 Units, value and block defaults

```
unit_ml(m)     = if m.density <= 1600 { 1000 } else { 250 }
unit_mass_g(m) = floor_div(m.density × unit_ml(m), 1000)
unit_value(m)  = if m.value_per_kg == 0 { 0 } else { max(1, floor_div(m.value_per_kg × unit_mass_g(m), 1000)) }   // Lucre at P(1); doc 11 scales by P(ilvl)
dur_mat_bp(m)  = 2500 + 750 × toughness + 600 × hardness          // mat.iron 10000
break_ticks(m) = 30 × hardness                                     // doc 13 §4.13 input; mining base (§9.2)
```

A material unit is a `cat.material` stack item whose `template` is the material ID; `stack_max` 20; it never rolls quality or rarity. Example: `mat.steel` has `unit_ml = 250`, `unit_mass_g = 1962`, `unit_value = floor_div(6 × 1962, 1000) = 11`.

Block registry defaults that doc 03 §4.5 marks "mat":

| Field | Default from the material |
|---|---|
| `hardness_milli` | `1000 × hardness` if the block carries `block.tag.destructible`, else `-1` (unbreakable; Canon §4.3 fixed terrain) |
| `blast_res_milli` | `500 × (hardness + toughness)` |
| `flammability_bp` | `flammability_bp` |
| `burn_runs` | `floor_div(density, 15)` when flammable, else 0 |
| `heat_cond_bp` | `1000 + 900 × conductivity` |
| `sound_atten_ddb` | `clamp(100 + floor_div(density, 10), 100, 600)` |
| `conceal` | `conceal` |
| `light_emit` | 7 if `luminous`, else 0 |

### 2.6 Raw ores

Ten metals come from ore: `mat.copper.ore`, `mat.tin.ore`, `mat.zinc.ore`, `mat.iron.ore`, `mat.silver.ore`, `mat.gold.ore`, `mat.titanium.ore`, `mat.orichalcum.ore`, `mat.star_metal.ore`, `mat.aetherium.ore`. An ore unit has `unit_mass_g = mul_bp(unit_mass_g(metal), 7000)` and `unit_value = max(1, mul_bp(unit_value(metal), 4000))`, stacks to 20, and is valid only as a process input. One ore unit smelts into one metal unit (§6.6).

### 2.7 Loot material pools

Doc 11 step 7 calls `pick_material(genre, tmpl, rng)` once per item (1 `weighted` draw).

```rust
const TIER_W: [u32; 6] = [2000, 6000, 3000, 1200, 400, 150];
fn pick_material(g: GenreId, tmpl: &Template, ilvl: i32, rng: &mut Rng) -> MatId {
    let role = tmpl.primary_role;                                   // §3.1
    let mut ids = Vec::new(); let mut w = Vec::new();
    for m in MATERIALS.iter() {                                     // ascending content ID
        if !m.roles.contains(role) || m.has_tag(TERRAIN) || m.has_tag(ALLOYING) { continue; }
        if ilvl < m.min_d || (m.max_d > 0 && ilvl > m.max_d) { continue; }
        let k = if m.genres.contains(g) { 3 } else if m.universal { 1 } else { 0 };
        if k > 0 { ids.push(m.id); w.push(TIER_W[m.tier as usize] * k); }
    }
    if ids.is_empty() { return role.reference(); }                  // iron, oak or leather; no draw
    ids[rng.weighted(&w)]
}
```

The secondary component never draws: it is the highest-weight entry of the same procedure for the secondary role, ties to the lower ID. Example, Weird West frame at ilvl 7: `[copper 6000, hexed_silver 9000, iron 18000, steel 3000]`; the haft pool is `[birch 2000, bone 6000, leather 18000, oak 6000, pine 2000]`, so the secondary is `mat.leather`. In Marginalia (`ilvl >= 26`) every band test passes for materials with `max_d = 0`.

### 2.8 Material equivalence for Rewrite

Doc 11 §15 calls `equivalent(m, g)` for each material of a rewritten item. Material tier never changes (Brief §2.4).

```rust
fn equivalent(m: &Mat, g: GenreId) -> MatId {
    if m.genres.contains(g) || m.universal { return m.id; }
    for scope in [Native, Universal] {
        let best = MATERIALS.iter()                                  // ascending ID
            .filter(|c| !c.has_tag(TERRAIN) && c.tier == m.tier && c.shares_role(m) && c.in_scope(scope, g))
            .min_by_key(|c| ((c.dmg_bp - m.dmg_bp).abs() + (c.ar_bp - m.ar_bp).abs(), c.id));
        if let Some(c) = best { return c.id; }
    }
    m.id
}
```

Examples: `mat.moonsilver` into Cyberpunk becomes `mat.titanium` (distance 1000, ahead of carbon fiber at 1500); `mat.jade` into Mythic Fantasy becomes `mat.moonsilver`; `mat.dragonbone` into Hardboiled Noir stays `mat.dragonbone` (no tier-4 candidate).

### 2.9 Doc 12 pool reconciliation

Doc 12 §2.4 named pool IDs before Canon §5.9 existed. Canon §5.9 wins. Doc 12 IDs that this table lacks resolve as follows: `starsilver` → `moonsilver`; `ebony` → `blackwood`; `void_glass` → `obsidian`; `ceramic` → `terracotta`; `quantum_glass` → `psi_crystal`; `neon_glass`, `mirror` → `glass`; `scrap_steel` → `scrap_iron`; `mahogany` → `oak`; `cloud`, `dreamglass` → `dreamstuff`; `featherdown` → `silk`; `particleboard` → `pine`; `linoleum` → `polymer`; `graphite` → `coal`; `ink` → `vellum`. The other doc 12 names (`granite`, `slate`, `basalt`, `coral`, `concrete`, `chrome`, `sandstone`, `pine`, `gold`, `brick`, `cedar`, `bamboo`, `cast_iron`, `birch`, `thornwood`, `terracotta`, `paper`) exist here.

## 3. Stat derivation from materials

### 3.1 Component volumes

Each archetype has a primary and an optional secondary component. Volumes give the doc 05 `ref_mass_g` with reference materials. `Cells` is the unit count a recipe consumes (§6.6).

| Archetype | Primary role, ml | Secondary role, ml | Mass at reference (g) | Cells |
|---|---|---|---|---|
| `arch.melee.dagger` | blade 47 | haft 40 | 399 | 1 + 1 |
| `arch.melee.blade_short` | blade 109 | haft 60 | 902 | 1 + 1 |
| `arch.melee.blade_long` | blade 170 | haft 80 | 1397 | 2 + 1 |
| `arch.melee.blade_great` | blade 367 | haft 150 | 3000 | 3 + 1 |
| `arch.melee.axe` | blade 133 | haft 600 | 1496 | 1 + 1 |
| `arch.melee.axe_great` | blade 292 | haft 1200 | 3198 | 2 + 2 |
| `arch.melee.mace` | head 181 | haft 500 | 1799 | 1 + 1 |
| `arch.melee.warhammer` | head 375 | haft 1400 | 4001 | 2 + 2 |
| `arch.melee.spear` | blade 57 | haft 1800 | 1798 | 1 + 2 |
| `arch.melee.halberd` | blade 140 | haft 2800 | 3201 | 1 + 3 |
| `arch.melee.quarterstaff` | haft 2295 | head 10 | 1799 | 3 + 1 |
| `arch.melee.fist` | head 60 | soft 150 | 601 | 1 + 1 |
| `arch.melee.whip` | soft 683 | haft 150 | 699 | 1 + 1 |
| `arch.melee.chain` | head 269 | soft 100 | 2203 | 2 + 1 |
| `arch.ranged.pistol` | frame 118 | haft 100 | 1003 | 1 + 1 |
| `arch.ranged.pistol_heavy` | frame 189 | haft 150 | 1599 | 1 + 1 |
| `arch.ranged.smg` | frame 327 | haft 300 | 2798 | 2 + 1 |
| `arch.ranged.rifle_auto` | frame 343 | haft 1200 | 3599 | 2 + 2 |
| `arch.ranged.rifle_marksman` | frame 381 | haft 1600 | 4198 | 2 + 2 |
| `arch.ranged.shotgun` | frame 340 | haft 1500 | 3800 | 2 + 2 |
| `arch.ranged.sniper` | frame 635 | haft 2000 | 6497 | 3 + 2 |
| `arch.ranged.launcher` | frame 794 | haft 1000 | 6998 | 4 + 1 |
| `arch.ranged.beam` | frame 553 | focus 200 | 4502 | 3 + 1 |
| `arch.ranged.charge_rifle` | limb 1966 | soft 30 | 1499 | 2 + 1 |
| `arch.ranged.crossbow` | limb 1567 | haft 3100 | 3500 | 2 + 4 |
| `arch.ranged.thrown` | blade 38 | — | 299 | 1 per 3 |
| `arch.ranged.staff` | haft 2000 | focus 40 | 1530 | 2 + 1 |
| `arch.ranged.wand` | focus 400 | — | 300 | 1 |
| `arch.offhand.shield_buckler` / `_heater` / `_tower` | plate 172 / 480 / 1105 | haft 200 / 300 / 400 | 1503 / 4002 / 8996 | 1+1 / 2+1 / 5+1 |
| `arch.offhand.lantern` | frame 152 | — | 1196 | 1 + 1 glass |
| `arch.offhand.focus` | focus 800 | — | 600 | 1 |
| `arch.offhand.tome` | soft 959 | focus 100 | 899 | 1 + 1 |
| `arch.ward.aegis` / `kinetic` / `reflector` / `bulwark` | frame 34 / 147 / 96 / 372 | focus 40 / 60 / 60 / 100 | 297 / 1201 / 800 / 3002 | 1+1 / 1+1 / 1+1 / 2+1 |
| `item.tool.pick` | head 203 | haft 1200 | 2497 | 1 + 2 |
| `item.tool.toolkit` | frame 30 | haft 80 | 296 | 1 + 1 |

Armor has one component. Its volume is `ceil_div(ref_mass_g × 1000, ref_density)` with doc 05 Table 11.1b masses and reference materials cloth `mat.linen`, light `mat.leather`, medium and heavy `mat.iron`. Cells (head, body, hands, legs, feet): cloth 1/1/1/1/1, light 1/5/1/3/2, medium 1/5/1/3/1, heavy 2/8/1/4/2. Example: medium body is `ceil_div(8000000, 7870) = 1017` ml.

```
mass_g = Σ components floor_div(vol_ml × density(material), 1000)
```

### 3.2 Weapons

1. **Damage.** Canon §10.4 with `material_bp = dmg_bp(primary)`.
2. **Mass.** `mass_g` feeds doc 05 §3.3 timing, stamina, impact and recoil.
3. **Bleed.** Archetypes with a `dmg.slash` share of at least 4000 multiply their `st.bleeding` status power by `10000 + 400 × (sharpness - 5)`: obsidian 14000, steel 10800, iron 10000, oak 8400.
4. **Banes.** `silvered` primaries add `DmgAdd +2000` against targets tagged undead or fiend. `fae_bane` primaries add `DmgAdd +2500` against NPCs tagged fae; Erratum `kin.fae` targets use Canon §16.3 instead.
5. **Material tendencies**, applied with the manufacturer tendencies (§7.3): casting items (`staff`, `wand`, `focus`, `tome`, `beam`) take aether cost `C = -150 × (arcane_affinity(primary or focus) - 4)`; firearms and gadgets take reload `L = -100 × (tech_affinity(primary) - 5)` and gadget recharge rate `+200 × (tech_affinity - 5)`.
6. **Enchant capacity** uses the primary's `arcane_affinity` (doc 06 §9.1).

### 3.3 Armor and shields

1. **AR.** Canon §10.4 with `mat_ar_bp = ar_bp(primary)`.
2. **Poise.** `poise' = mul_bp(poise, 10000 + 400 × (toughness - ref_toughness))`, with `ref_toughness` 3 (cloth), 5 (light), 6 (medium, heavy, shields).
3. **Resistances per piece**, added before the cap (Canon §10.2): conductivity ≥ 8 gives `dmg.shock` −200; conductivity ≤ 1 with density ≤ 1600 gives `dmg.shock` +200; flammability ≥ 5000 gives `dmg.fire` −200; `silvered` gives `dmg.umbral` +300.
4. **Mass** is `vol × density`; doc 04 §11.6 counts it.

Example: a medium body piece in `mat.bronze`, ilvl 4, Common, Normal, +0: torso AR `20 → × T(4) = 23 → 23 → 23 → 23 → × 9000 = 20`; mass `floor_div(1017 × 8800, 1000) = 8949` g; poise 6 (bronze toughness 6).
