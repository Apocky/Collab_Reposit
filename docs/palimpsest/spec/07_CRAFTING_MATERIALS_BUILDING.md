# 07 — Crafting, Materials and Building
As of 2026-10-11. Status: BINDING. Conforms to canon-1.5.

This chapter owns the one material table (Canon §13.8a), crafting quality, stations, recipes, assembly and parts, durability, mining and gathering, blocks and Errata building, gadgets and cooking. It carries the canon-1.2 additions assigned to this chapter (every Canon §5.9 material, `cat.part` loose parts and licensed grafts, `cat.seed` farming, `block.ice`, `block.web`, the Delving hooks and Hacking-free gadgets) under canon-1.5, which limits building to placeables (Canon §4.3), replaces Delving with `skill.fieldcraft` and turns the hub plot into Margin trophies (Canon §17.8). Where an older canon text differs, canon-1.5 wins (Canon gaps 1).

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
| Canon | §5.11, §16.3, §17.1, §17.5, §17.7, §17.9, §18, §19 | Unmoored failure count, kin fae and construct, Colophon, friendly fire, mounted and invader placement limits, events, namespace ownership |
| Doc 03 | §4.2–§4.7, §12, §13 | `BlockState`, shapes, block registry schema, explosions, `set_block`, mining progress, block damage, placement checks, `timed_revert` |
| Doc 04 | §1.2, §2 (`self_repair`), §5.1–§5.3, §7, §8.6, §9.3, §11.2 | modifier kinds, skill XP events and mastery perks (Smithing, Tinkering, Fieldcraft, Survival), Artificer talents, construct self-repair, body-part butchery, augment installation, food values |
| Doc 05 | §3.2, §3.3, §5.2, §7, §11, §12, §15 | archetype stats and `ref_mass_g`, mass-driven timing, `block_bp`, wallpen, ricochet, armor and shield tables, ward generators |
| Doc 06 | §9, §13, §16 | enchant capacity from arcane affinity, conjured walls, `shape_power`, offering altars |
| Doc 08 | §5, §8.1, §8.4, §9 | liquids, ingredient schema and tags, `ing.seed_*`, `station.alchemy` apparatus |
| Doc 09 | §2.3, §2.4, §7.2, §7.3, §7.6, §13.1, §13.4, §13.6 | `lightsrc` classes and light manipulation, acoustic surface classes, the `suppressor` part rule, `noise.gadget.noisemaker`, lock tiers `lock.t1`…`lock.t6` and `door_locked`, device Seize, Security-50 trap recovery |
| Doc 10 | §2, §7 | crafted items known; `curse.brittle` |
| Doc 11 | §2.3, §3.5, §3.6, §5, §12.3, §13.1, §14, §15, §16 | generation step 7, tool and food templates (`item.tool.*`, `item.food.*`), gadget loot stats, manufacturers, classes, tendency letters and the loot placement of loose parts, shop restock, value, stacking, Rewrite, reforging, salvage |
| Doc 12 | §2.1, §2.2, §2.4, §3.3, §5 (Laws), §9.2, §11.2, §15 | `hard_bp`, `fixture_level`, genre material pools, `node_w`, `law.post_apocalypse.scarcity`, lock doors, resource-node placement, craft rooms |
| Doc 13 | §4.13, §5.7, §10.6 | navigation links (doors, climb, dig), `break_ticks`, `dig_ticks`, `sabot` and `short_circuit` disabling, Errata-block erasure |
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
| `recipe.<group>.<name>` | recipes (§6.6, §7.2, §8.5, §11.2, §12.2) | 127 |
| `part.<slot>.<name>` | loose parts (§7.2) | 29 |
| `part.graft.<genre_suffix>.<mfr_name>` | licensed grafts (§7.5) | 38 |
| `block.<…>`, `block.tag.<tag>` | blocks (§10.1) and block tags | §10.1 |
| `gadget.<name>` | gadgets and deployables (§11) | 13 |

`q.*` stays the Canon §13.5 core list. This chapter adds no quality ID.

### 1.2 Constants

| Constant | Value | Use |
|---|---|---|
| `LIGHT_DENSITY_MAX` | 1600 kg/m³ | materials at or below it come in 1000 ml units, others in 250 ml units (§2.5) |
| `UNIT_ML_DENSE`, `UNIT_ML_LIGHT` | 250, 1000 | material unit volumes |
| `DUR_BASE` | weapons, armor, shields, ward generators, lanterns 10000; tools 5000 (pick, toolkit, hacking deck, surgical kit); lockpicks 1 per pick; reusable gadgets 4000; thrown 40; wands, foci, tomes 0 (no durability) | §8.1 |
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
| `CROP_CAP_PER_PLAYER` | 4 | live crops per party slot (§9.7) |
| `REPAIR_KIT_USE` | 250 | toolkit wear spent by one doc 04 `self_repair` use (§8.2) |
| `SPARK_BP` | 500 | gadget failure spark, Bp of the user's Max HP as `dmg.shock` (Canon §5.5; §11.1) |
| `NODE_BLAST_RES` | 1000000 | `blast_res_milli` of every `block.node.*`, above any explosion's block power, so blasts never break nodes (§9.3) |

### 1.3 Random draws

| Draw | Stream | Key | Count |
|---|---|---|---|
| Crafting quality and crafted perk seed (§4) | keyed `rng.craft` | actor = crafter, tick = job start tick, seq = output index | 4 per quality-rolling output: 3 `range(141)`, 1 `next_u64`; 0 for outputs that do not roll |
| Mining bonus unit (§9.3) | keyed `rng.craft` | actor = miner, tick = break tick, seq = 0 | 1 `roll_bp` per node broken |
| Crop seed return below Survival 25 (§9.7) | keyed `rng.craft` | actor = harvester, tick = harvest tick, seq = 0 | 1 `roll_bp` |
| Gadget Contradiction failure (§11.1) | keyed `rng.combat` | actor = deployer, tick = the deploy action's start tick, seq 0 (doc 05's one action start per tick, so the key never collides with a shot) | 1 `roll_bp` per deploy while failure chance > 0, rolled when the channel completes |
| Turret, drone and coil shots (§11.4) | keyed `rng.combat` | actor = gadget EntityId, tick = firing tick, seq 0 (one shot per gadget per tick) | 2 spread per projectile and 1 crit per hit, as Canon §3.4; the coil and the flame turret draw 1 crit per target hit from the same generator, targets in ascending EntityId |
| Vein kind and size (§9.4) | `rng.worldgen` (doc 12's sequential stream, in doc 12's placement order) | — | 1 `weighted` + 1 `range_incl(2, 4)` per mineral vein |
| Salvage loose part (§8.5) | keyed `rng.loot` | doc 11 §16.4 | doc 11 |
| Forge reforge offers (§8.6) | `rng.offer.p<slot>` | doc 11 §16.3 | doc 11 |

Every other mechanic in this chapter is deterministic. The gadget spark (§11.1) draws nothing beyond its failure roll.

### 1.4 Owned simulation state (saved, Canon §2.5)

| State | Scope | Content |
|---|---|---|
| `known_recipes: BTreeSet<Id32>` | per Erratum, run | learned recipes with `learn: 1` (§6.5) |
| `craft_job: Option<CraftJob>` | per Erratum | the active channel (§6.4) |
| `meal_buff: Option<(Id32 dish, Tick expiry)>` | per Erratum | §12.1 |
| `placed_meta: BTreeMap<Gidx, PlacedMeta>` | Folio | owner slot, `hp`, `hp_max`, expiry tick (or remaining lit ticks while a torch is snuffed), patron, span, `trap_bp`, conjured flag (§10.3, §10.4) |
| `stations: BTreeMap<Gidx, StationState>` | Folio | tier, fuel ticks, job queue, output slots (§5.3) |
| `crops: BTreeMap<Gidx, CropState>` | Folio | seed, stage, next stage tick (§9.7) |
| gadget entities and props | Folio | `cooldown`, `target`, `disabled_until`, expiry (§11.3, §11.4) |
| gadget item state | per item, run | `charges`, `progress`, `target` (§11.3) |

Folio-scoped state is discarded on `ev.folio_exited`. Doc 07 runs one system, `sys.crafting` (period 1, phase 0), placed by doc 03 after `sys.actions` and before `sys.block_updates`; it advances craft channels, station queues, crops, torch expiry, gadget firing (§11.4), gadget recharge and gadget timers, each in ascending `Gidx`, `EntityId` or item `uid`.

## 2. Materials

### 2.1 Schema

```json
{
  "id": "mat.iron",
  "density": 7870, "hardness": 5, "toughness": 6, "sharpness": 5, "conductivity": 6,
  "flammability_bp": 0, "melt_decic": 15380, "conceal": 15, "pen_cost": 300,
  "tier": 1, "min_d": 1, "max_d": 20,
  "value_per_kg": 3, "value_bp": 10000, "dmg_bp": 10000, "ar_bp": 10000,
  "arcane_affinity": 2, "tech_affinity": 5, "acoustic": "noise.surface.metal",
  "genres": ["genre.weird_west"], "universal": 1,
  "roles": ["mat.role.blade", "mat.role.head", "mat.role.frame", "mat.role.plate", "mat.role.block"],
  "tags": ["mat.tag.metal", "mat.tag.ricochet", "mat.tag.fae_bane"]
}
```

Validation: `dmg_bp` and `ar_bp` in `7000..=14000`; `mat.iron` has `dmg_bp = ar_bp = value_bp = 10000`; `unit_ml`, `unit_g` and `unit_lucre` equal the §2.5 formulas; the tag `flammable` is present exactly when `flammability_bp >= 500`; every Canon §5.9 material lists each genre whose §5.9 row names it; `hardness`, `toughness`, `sharpness`, `conductivity`, `arcane_affinity`, `tech_affinity` in `0..=10`; `conceal` in `0..=15`; `melt_decic` absent means the material never melts. Every material has dependency none (Canon §5.3 raw materials).

Properties: **density** (kg/m³, equal to g/l) gives mass; **hardness** gives mining time, block hardness and tool tier; **toughness** gives durability, blast resistance and armor poise; **sharpness** gives bleed power on edged weapons; **conductivity** gives heat conductance and armor shock resistance; **arcane affinity** gives enchant capacity (doc 06 §9.1) and casting cost; **tech affinity** gives reload and gadget recharge; **flammability** feeds fire (doc 03 §8); **melt point** gates smelting stations; **value per kg** prices material units; **tier** and **band** gate loot and stations.

### 2.2 Material table

All 69 materials, one row each, with every property. The block is CSV so tools can load it directly; `+` joins multi-valued fields and `-` means none.

```csv
id,density,unit_ml,unit_g,hard,tough,sharp,cond,flam_bp,melt_decic,conceal,pen_cost,tier,band,lucre_kg,unit_lucre,value_bp,dmg_bp,ar_bp,arc,tech,genres,roles,tags
copper,8960,250,2240,3,5,3,10,0,10850,15,250,1,1..=20,4,8,8000,8000,8000,4,6,SP+*,bl+hd+fr+pl,metal+ricochet
tin,7290,250,1822,2,3,1,6,0,2320,15,200,1,1..=20,4,7,7000,7000,7000,2,4,*,-,metal+ricochet+alloying
zinc,7140,250,1785,3,3,1,6,0,4200,15,210,1,1..=20,3,5,7000,7000,7000,1,4,*,-,metal+ricochet+alloying
bronze,8800,250,2200,4,6,5,6,0,9500,15,260,1,1..=20,3,6,9000,9000,9000,4,4,WX+MA,bl+hd+fr+pl,metal+ricochet
iron,7870,250,1967,5,6,5,6,0,15380,15,300,1,1..=20,3,5,10000,10000,10000,2,5,WW+*,bl+hd+fr+pl+bk,metal+ricochet+fae_bane
cold_iron,7870,250,1967,5,6,5,6,0,15380,15,300,2,4..,8,15,12000,10500,10000,0,3,FT,bl+hd+fr+pl,metal+ricochet+fae_bane
scrap_iron,7200,250,1800,4,4,4,6,0,15000,15,260,1,1..=20,1,1,6000,9000,9000,1,5,PA,bl+hd+fr+pl+bk,metal+ricochet
cast_iron,7200,250,1800,5,3,2,5,0,11500,15,280,1,1..=20,3,5,9000,9000,10500,1,6,SP,hd+fr+pl+bk,metal+ricochet+brittle
steel,7850,250,1962,6,7,7,5,0,14250,15,350,2,4..,6,11,13000,11000,11000,2,6,MF+HN+*,bl+hd+fr+pl+li+bk,metal+ricochet
brass,8500,250,2125,4,5,3,7,0,9300,15,240,2,4..,6,12,12000,9000,9000,3,8,HN+SP,hd+fr+pl,metal+ricochet
silver,10490,250,2622,3,4,4,10,0,9620,15,240,2,4..,7,18,25000,9000,8500,8,3,GH,bl+hd+fr+fo,metal+ricochet+silvered+arcane_reagent
hexed_silver,10490,250,2622,4,5,5,9,0,9620,15,260,2,6..,8,20,30000,10500,9500,9,2,WW,bl+hd+fr+fo,metal+ricochet+silvered+arcane_reagent
gold,19300,250,4825,2,4,2,9,0,10640,15,220,2,4..,4,19,40000,7500,7000,7,5,*,hd+fo,metal+ricochet+arcane_reagent
chrome,7190,250,1797,6,6,6,6,0,19070,15,360,2,4..,10,17,14000,11500,11000,1,8,CP,bl+hd+fr+pl,metal+ricochet
titanium,4510,250,1127,7,8,6,3,0,16680,15,400,3,9..,30,33,18000,12000,12500,2,8,SO+CP,bl+hd+fr+pl+li,metal+ricochet
moonsilver,6000,250,1500,6,6,7,8,0,12000,15,330,3,9..,30,45,32000,12000,11500,10,2,MF,bl+hd+fr+pl+fo,metal+ricochet+silvered+arcane_reagent
plasteel,3500,250,875,7,8,4,4,0,24000,15,450,3,9..,25,21,18000,12000,13500,1,10,SO,fr+pl+bk,metal+ricochet
orichalcum,7000,250,1750,8,8,8,7,0,18000,15,480,4,14..,60,105,36000,13500,13000,8,6,MA,bl+hd+fr+pl+fo,metal+ricochet+arcane_reagent
star_metal,9500,250,2375,9,8,9,6,0,30000,15,600,4,14..,40,95,40000,14000,13000,6,7,CH,bl+hd+fr+pl,metal+ricochet
aetherium,3000,250,750,7,6,6,10,0,20000,15,420,4,14..,150,112,38000,12500,12000,9,10,SP,fr+fo,metal+ricochet+arcane_reagent+luminous
redacted_alloy,5000,250,1250,9,9,8,0,0,-,15,700,5,20..,120,150,50000,13500,14000,0,9,RD,bl+hd+fr+pl,metal+ricochet
gold_leaf,19300,250,4825,1,2,0,9,0,10640,15,60,5,20..,30,144,50000,7000,7000,10,2,UT,fo,metal+arcane_reagent
pine,500,1000,500,2,4,1,1,4000,-,15,30,0,1..=8,1,1,7000,8000,7000,2,1,*,ha+li+bk,wood+flammable+fuel+acid_soluble
birch,670,1000,670,2,5,1,1,3500,-,15,35,0,1..=8,1,1,7500,8500,7500,4,1,FT+*,ha+li+fo,wood+flammable+fuel+acid_soluble
cedar,380,1000,380,2,4,1,1,3000,-,15,30,1,1..=20,2,1,8500,8500,7500,3,1,WX,ha+li+bk,wood+flammable+fuel+acid_soluble
bamboo,700,1000,700,3,6,2,1,3500,-,15,35,1,1..=20,2,1,8500,9000,8000,3,2,WX,ha+li+bk,wood+flammable+fuel+acid_soluble
oak,750,1000,750,3,6,1,1,2500,-,15,40,1,1..=20,2,1,10000,10000,8500,4,2,MF+*,ha+li+fo+pl+bk,wood+flammable+fuel+acid_soluble
thornwood,900,1000,900,4,6,3,1,3000,-,15,45,2,4..,6,5,14000,10500,9000,7,1,FT,ha+li+fo,wood+flammable+fuel+acid_soluble
blackwood,1200,1000,1200,4,7,2,1,1500,-,15,60,3,9..,25,30,18000,11500,9500,7,1,GH,ha+li+fo+pl,wood+flammable+fuel+acid_soluble
elderwood,600,1000,600,3,6,1,1,2000,-,15,40,3,9..,50,30,24000,12000,9500,10,0,FT,ha+li+fo,wood+flammable+fuel+acid_soluble+arcane_reagent
granite,2700,250,675,6,5,2,2,0,12150,15,130,1,1..=20,1,1,6000,8500,8500,1,1,MF+UT+*,hd+bk,stone+ricochet
sandstone,2300,250,575,3,3,2,2,0,12000,15,90,0,1..=8,1,1,5000,7500,7500,1,0,WW+*,hd+bk,stone+ricochet
slate,2800,250,700,4,3,4,2,0,12000,15,110,1,1..=20,1,1,6000,8000,8000,2,1,GH,hd+bk,stone+ricochet
basalt,3000,250,750,6,5,2,3,0,11000,15,140,1,1..=20,1,1,6000,9000,9000,1,1,CH,hd+bk,stone+ricochet
marble,2700,250,675,4,4,2,2,0,12000,15,120,1,1..=20,3,2,9000,8000,8500,4,1,MA+*,hd+bk,stone+ricochet
obsidian,2400,250,600,5,1,10,1,0,11000,15,120,2,4..,12,7,14000,12000,7500,5,1,CH,bl+fo,stone+ricochet+brittle
jade,3300,250,825,6,7,3,1,0,-,15,150,3,9..,40,33,28000,9500,10000,8,1,WX,hd+fo+pl,stone+ricochet+arcane_reagent
brick,1900,250,475,4,4,1,2,0,-,15,150,0,1..=8,1,1,5000,7500,8000,1,1,HN+SP+*,bk,stone+ricochet
concrete,2400,250,600,5,4,1,2,0,-,15,180,0,1..=8,1,1,5000,8000,9000,0,2,CP+PA+RD+*,hd+bk,stone+ricochet
terracotta,1900,250,475,3,2,1,1,0,-,15,100,0,1..=8,2,1,6000,7500,7500,3,1,MA,hd+bk,stone+brittle
glass,2500,250,625,3,1,8,1,0,14000,0,2,1,1..=20,4,2,8000,11000,7000,3,4,FT+HN,bl+fo,brittle
rad_glass,2700,250,675,3,1,8,2,0,14000,1,3,2,4..,10,6,10000,11500,7000,1,6,PA,bl+fo,brittle+luminous
psi_crystal,2650,250,662,7,3,7,4,0,17000,2,160,4,14..,150,99,36000,12500,9000,10,8,SO,bl+fo,brittle+arcane_reagent+luminous
bone,1900,250,475,3,4,4,1,0,-,15,90,1,1..=20,2,1,7000,8500,8500,5,1,GH+*,bl+hd+li,ricochet
dragonbone,1500,1000,1500,8,8,7,1,0,-,15,300,4,14..,70,105,36000,13500,13500,8,1,MF,bl+hd+li+pl,arcane_reagent
chitin,1300,1000,1300,4,5,5,1,500,-,15,80,2,4..,8,10,10000,10000,11000,3,2,CH,bl+li+pl+so,flammable+acid_soluble
hide,1100,1000,1100,1,5,0,1,2000,-,15,25,0,1..=8,4,4,6000,7000,8500,2,1,*,so,flammable+acid_soluble
leather,860,1000,860,2,5,0,1,1500,-,15,30,1,1..=20,8,6,8000,9000,10000,3,2,WW+*,so+ha,flammable+acid_soluble
linen,1500,1000,1500,1,3,0,1,6000,-,15,10,0,1..=8,3,4,6000,7000,10000,3,1,*,so,fabric+flammable+acid_soluble
tweed,1300,1000,1300,1,4,0,1,4500,-,15,12,1,1..=20,5,6,9000,7000,10500,2,2,HN,so,fabric+flammable+acid_soluble
silk,1300,1000,1300,1,6,0,1,5000,-,15,10,2,4..,12,15,16000,7500,11000,6,2,WX,so+li,fabric+flammable+acid_soluble
rubber,1100,1000,1100,2,6,0,0,4000,-,15,40,1,1..=20,6,6,8000,7000,9500,0,6,PA,so+ha,flammable+fuel
polymer,1200,1000,1200,3,6,1,0,3000,2500,15,60,2,4..,8,9,10000,9000,10000,0,9,CP+SO+RD+PA,fr+ha+so+bk,flammable+fuel
carbon_fiber,1600,1000,1600,6,8,4,7,1000,-,15,200,3,9..,25,40,18000,11500,12500,1,10,CP,fr+ha+li+pl,flammable
nanoweave,1000,1000,1000,4,9,0,1,500,-,15,150,4,14..,100,100,30000,12000,13000,2,10,CP,so,fabric+flammable
coral,1400,1000,1400,3,3,5,1,0,-,15,70,2,4..,10,14,12000,9500,9000,6,1,CH,bl+hd+fo,stone
flesh,1060,1000,1060,1,2,0,2,500,-,15,15,0,1..,1,1,2000,7000,7000,1,0,*,-,edible+fat+flammable+acid_soluble+terrain
web,1300,1000,1300,1,3,0,0,9000,-,3,1,0,1..,6,7,6000,7000,7000,4,0,*,so,flammable+acid_soluble+terrain
paper,800,1000,800,1,1,0,0,9000,-,15,8,0,1..=8,2,1,5000,7000,7000,6,1,RD+UW+*,so+fo,flammable+fuel+acid_soluble
vellum,900,1000,900,5,7,2,0,2000,-,15,200,5,20..,150,135,50000,13000,13000,10,3,UW+UT,so+fo+pl,flammable+arcane_reagent
dreamstuff,400,1000,400,4,6,6,3,1000,-,6,100,5,20..,350,140,50000,13000,12500,10,5,ON,bl+hd+fr+pl+so+fo,flammable+arcane_reagent+luminous
air,1,1000,1,0,0,0,0,0,-,0,0,0,1..,0,0,0,7000,7000,0,0,*,-,nonsolid+terrain
void,99999,250,24999,10,10,0,0,0,-,15,-1,0,1..,0,0,0,7000,7000,0,0,*,-,unbreakable+terrain
soil,1500,1000,1500,1,2,0,1,0,-,15,60,0,1..,0,0,0,7000,7000,1,0,*,bk,absorbent+tillable+terrain
sand,1600,1000,1600,1,1,0,1,0,17000,15,50,0,1..,0,0,0,7000,7000,0,1,*,bk,absorbent+terrain
ice,917,1000,917,2,2,3,2,0,10,2,20,0,1..,0,0,0,9000,7500,2,0,*,bl,brittle+ricochet+terrain
leaves,300,1000,300,1,1,0,0,7000,-,3,1,0,1..,0,0,0,7000,7000,3,0,*,-,flammable+fuel+absorbent+acid_soluble+terrain
coal,1350,1000,1350,2,2,0,3,8000,-,15,70,0,1..,1,1,3000,7000,7000,0,3,*,-,flammable+fuel
slag,2800,250,700,4,2,1,3,0,13000,15,140,0,1..,0,0,1000,8000,8000,0,2,*,bk,stone+ricochet+terrain
```

### 2.3 Column legend

`density` kg/m³; `unit_ml` and `unit_g` are the unit volume and mass (§2.5); `hard`, `tough`, `sharp`, `cond` are hardness, toughness, sharpness and conductivity `0..=10`; `flam_bp` flammability; `melt_decic` melt point (`-` never melts); `conceal` Canon §12.2; `pen_cost` doc 05 §7 wall penetration (`-1` for `mat.void` means impenetrable; doc 05 treats any negative cost as a stop); `tier` 0 to 5; `band` is `min_d..=max_d` for loot (both ends) and node placement (lower end only, §9.4) (`min_d..` has no upper bound); `lucre_kg` value per kg and `unit_lucre` unit value, both at `P(1)`; `value_bp` doc 10 and doc 11 value factor; `dmg_bp` and `ar_bp` Canon §10.4 factors; `arc` and `tech` arcane and tech affinity `0..=10`.

Genres: MF mythic_fantasy, GH gothic_horror, CH cosmic_horror, SO space_opera, CP cyberpunk, PA post_apocalypse, WW weird_west, HN hardboiled_noir, WX wuxia, SP steampunk, FT fairy_tale, MA mythic_antiquity, ON oneiric, RD redacted, UW unwritten, UT ur_text; `*` marks a universal material (§2.7). Roles: bl blade, hd head, ha haft, li limb, fr frame, pl plate, so soft, fo focus, bk block (`mat.role.<name>`). Tags are `mat.tag.<name>`.

### 2.4 Roles and tags

Roles say which item component a material may form. Reference materials sit at 10000 for their role: `mat.iron` for blade, head, frame and plate; `mat.oak` for haft, limb and focus; `mat.leather` and `mat.linen` for soft.

| Tag | Rule |
|---|---|
| `metal`, `wood`, `stone`, `fabric` | families; `fabric` soft materials make cloth armor, other soft materials make light armor |
| `ricochet` | doc 05 §7 ricochet faces |
| `flammable` | `flammability_bp >= 500`; doc 03 fire, doc 05 and doc 06 hooks |
| `fuel` | burns in stations (§5.3); `kin.construct` refuels from it (Canon §16.3) |
| `brittle` | the item shatters at durability 0 (§8.3) |
| `alloying` | crafting input only, never a loot primary |
| `arcane_reagent` | doc 06 §9.3 enchanting reagent |
| `silvered`, `fae_bane` | banes (§3.2) |
| `luminous` | blocks of it emit `light_block` 7 |
| `edible`, `fat` | `mat.flesh` is raw meat (doc 04 §11.2) and an alchemy fat |
| `absorbent`, `acid_soluble`, `tillable` | doc 08 §2 and §5; planting (§9.7) |
| `terrain`, `nonsolid`, `unbreakable` | never loot, never a crafted primary; `air` is non-solid; `void` is unbreakable |

**Acoustic class** (doc 09 §7.2 surface and break noise). Every material has exactly one `acoustic` value, written here so doc 09 never has to guess; doc 09's table agrees on every material it names.

| `acoustic` | Materials |
|---|---|
| `noise.surface.metal` | copper, tin, zinc, bronze, iron, cold_iron, scrap_iron, cast_iron, steel, brass, silver, hexed_silver, gold, chrome, titanium, moonsilver, plasteel, orichalcum, star_metal, aetherium, redacted_alloy, gold_leaf |
| `noise.surface.glass` | glass, rad_glass, psi_crystal |
| `noise.surface.stone` | granite, sandstone, slate, basalt, marble, obsidian, jade, brick, concrete, terracotta, coral, slag, void |
| `noise.surface.ice` | ice |
| `noise.surface.bone` | bone, dragonbone, chitin |
| `noise.surface.wood` | pine, birch, cedar, bamboo, oak, thornwood, blackwood, elderwood |
| `noise.surface.synthetic` | polymer, carbon_fiber |
| `noise.surface.soft` | hide, leather, rubber, flesh |
| `noise.surface.cloth` | linen, tweed, silk, nanoweave, dreamstuff, web |
| `noise.surface.paper` | paper, vellum |
| `noise.surface.foliage` | leaves |
| `noise.surface.sand` | sand |
| `noise.surface.dirt` | soil, coal |
| none | air |

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
| `tool_tier` | `tooltier.t0`, except resource nodes, which use their §9.3 `Req` |
| `acoustic` | the material's acoustic class (§2.4) |

Doc 03 §4.5 loader rule 2 overrides three of these for every block without `block.tag.destructible`: `flammability_bp` 0, `burn_runs` 0 and no `melt_*` fields, so a fixed-terrain wall of `mat.oak` or `mat.paper` never burns. Block records may override any default; every override is listed in §10.1.

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

The secondary component never draws: it is the highest-weight entry of the same procedure for the secondary role, ties to the lower ID. Example, Weird West frame at ilvl 7: `[copper 6000, hexed_silver 9000, iron 18000, steel 3000]`; the haft pool is `[birch 2000, leather 18000, oak 6000, pine 2000]`, so the secondary is `mat.leather`. In Marginalia (`ilvl >= 26`) every band test passes for materials with `max_d = 0`. Doc 11 §20's two-entry Weird West pool (`hexed_silver` 1500, `iron` 7000) is an illustration only; the generator always runs `pick_material` as written here.

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

Each archetype has a primary and an optional secondary component. Volumes are chosen so that the archetype built from its reference materials (§2.4) weighs exactly doc 05's `ref_mass_g`, which makes doc 05 §3.3's `mass_ratio_bp` exactly 10000 for a reference-material item. `Cells` is the unit count a recipe consumes (§6.6); `cells × unit_ml` of the reference material always covers the volume.

| Archetype | Primary role, ml | Secondary role, ml | Mass at reference (g) | Cells |
|---|---|---|---|---|
| `arch.melee.dagger` | blade 47 | haft 42 | 400 | 1 + 1 |
| `arch.melee.blade_short` | blade 109 | haft 58 | 900 | 1 + 1 |
| `arch.melee.blade_long` | blade 170 | haft 84 | 1400 | 2 + 1 |
| `arch.melee.blade_great` | blade 367 | haft 150 | 3000 | 3 + 1 |
| `arch.melee.axe` | blade 133 | haft 606 | 1500 | 1 + 1 |
| `arch.melee.axe_great` | blade 292 | haft 1203 | 3200 | 2 + 2 |
| `arch.melee.mace` | head 181 | haft 502 | 1800 | 1 + 2 |
| `arch.melee.warhammer` | head 413 | haft 1000 | 4000 | 4 + 1 |
| `arch.melee.spear` | blade 57 | haft 1803 | 1800 | 1 + 2 |
| `arch.melee.halberd` | blade 217 | haft 1991 | 3200 | 2 + 2 |
| `arch.melee.quarterstaff` | haft 2400 | — | 1800 | 3 |
| `arch.melee.fist` | head 60 | soft 149 | 600 | 1 + 1 |
| `arch.melee.whip` | soft 683 | haft 151 | 700 | 2 + 1 |
| `arch.melee.chain` | head 269 | soft 97 | 2200 | 2 + 1 |
| `arch.ranged.pistol` | frame 118 | haft 97 | 1000 | 1 + 1 |
| `arch.ranged.pistol_heavy` | frame 189 | haft 151 | 1600 | 1 + 1 |
| `arch.ranged.smg` | frame 327 | haft 303 | 2800 | 2 + 1 |
| `arch.ranged.rifle_auto` | frame 343 | haft 1202 | 3600 | 2 + 2 |
| `arch.ranged.rifle_marksman` | frame 381 | haft 1603 | 4200 | 2 + 2 |
| `arch.ranged.shotgun` | frame 340 | haft 1500 | 3800 | 2 + 2 |
| `arch.ranged.sniper` | frame 636 | haft 1994 | 6500 | 3 + 2 |
| `arch.ranged.launcher` | frame 795 | haft 993 | 7000 | 4 + 1 |
| `arch.ranged.beam` | frame 553 | focus 198 | 4500 | 3 + 1 |
| `arch.ranged.charge_rifle` | limb 1967 | soft 30 | 1500 | 2 + 1 |
| `arch.ranged.crossbow` | limb 1567 | haft 3100 | 3500 | 2 + 4 |
| `arch.ranged.thrown` | blade 38 | — | 299 | 2 per 3 |
| `arch.ranged.staff` | haft 2000 | focus 40 | 1530 | 2 + 1 |
| `arch.ranged.wand` | focus 400 | — | 300 | 1 |
| `arch.offhand.shield_buckler` / `_heater` / `_tower` | plate 172 / 480 / 1105 | haft 197 / 298 / 406 | 1500 / 4000 / 9000 | 1+1 / 2+1 / 5+1 |
| `arch.offhand.lantern` | frame 151 | focus 5 (`mat.glass`) | 1200 | 1 + 1 glass |
| `arch.offhand.focus` | focus 800 | — | 600 | 1 |
| `arch.offhand.tome` | soft 960 | focus 100 | 900 | 1 + 1 |
| `arch.ward.aegis` / `kinetic` / `reflector` / `bulwark` | frame 34 / 147 / 96 / 372 | focus 44 / 59 / 60 / 98 | 300 / 1200 / 800 / 3000 | 1+1 / 1+1 / 1+1 / 2+1 |
| `item.tool.pick` | head 203 | haft 1204 | 2500 | 3 + 2 |
| `item.tool.toolkit` | frame 150 | haft 427 | 1500 | 1 + 1 |

`arch.ranged.thrown` is the one archetype whose reference mass misses doc 05's 300 g, because no whole-millilitre iron blade weighs 300 g (38 ml is 299 g, 39 ml is 306 g). Doc 05 §3.3 then gives `mass_ratio_bp` 9966, `time_bp` 9989 and `stamina_bp = impact_bp = 9983`; `time_bp` 9989 leaves every tick count up to 454 unchanged, so thrown timing is exact. `item.tool.pick` and `item.tool.toolkit` match doc 11 §3.5's template masses.

Armor has one component. Its volume is `ceil_div(ref_mass_g × 1000, ref_density)` with doc 05 Table 11.1b masses and reference materials cloth `mat.linen`, light `mat.leather`, medium and heavy `mat.iron`. Cells (head, body, hands, legs, feet): cloth 1/1/1/1/1, light 1/5/1/3/2, medium 1/5/1/3/1, heavy 2/8/1/4/2. Example: medium body is `ceil_div(8000000, 7870) = 1017` ml and weighs 8,003 g. Rounding up puts a reference armor piece at most 7 g above Table 11.1b (medium hands, 1,007 g); armor mass feeds only encumbrance (doc 04 §11.6) and never doc 05 §3.3 timing.

```
mass_g = Σ components floor_div(vol_ml × density(material), 1000)
```

### 3.2 Weapons

1. **Damage.** Canon §10.4 with `material_bp = dmg_bp(primary)`.
2. **Mass.** `mass_g` feeds doc 05 §3.3 timing, stamina, impact and recoil.
3. **Bleed.** Archetypes with a `dmg.slash` share of at least 4000 multiply their `st.bleeding` status power by `10000 + 400 × (sharpness - 5)`: obsidian 12000, steel 10800, iron 10000, oak 8400.
4. **Banes.** `silvered` primaries add `DmgAdd +2000` against targets tagged undead or fiend. `fae_bane` primaries add `DmgAdd +2500` against NPCs tagged fae; Erratum `kin.fae` targets use Canon §16.3 instead.
5. **Material tendencies**, applied with the manufacturer tendencies (§7.3): casting items (`staff`, `wand`, `focus`, `tome`, `beam`) take aether cost `C = -150 × (arcane_affinity(primary or focus) - 4)`; firearms and gadgets take reload `L = -100 × (tech_affinity(primary) - 5)` and gadget recharge rate `+200 × (tech_affinity - 5)`.
6. **Enchant capacity** uses the primary's `arcane_affinity` (doc 06 §9.1).
7. **Crafter material bonus.** A weapon crafted by a crafter with Smithing 100 stores `mat_bonus_bp = 500` at Done (doc 04 Smithing 100), and its Canon §10.4 factor becomes `material_bp = min(14000, dmg_bp(primary) + mat_bonus_bp)`. The bonus is a stored instance field, so the item keeps it after the crafter's skill changes, and Recast (§8.6) recomputes it from the recaster. The 14000 cap keeps the material slot of `ITEM_MULT_MAX` (Canon §13.8) intact.

### 3.3 Armor and shields

1. **AR.** Canon §10.4 with `mat_ar_bp = ar_bp(primary)`.
2. **Poise.** `poise' = mul_bp(poise, 10000 + 400 × (toughness - ref_toughness))`, with `ref_toughness` 3 (cloth), 5 (light), 6 (medium, heavy, shields).
3. **Resistances per piece**, added before the cap (Canon §10.2): conductivity ≥ 8 gives `dmg.shock` −200; conductivity ≤ 1 with density ≤ 1600 gives `dmg.shock` +200; flammability ≥ 5000 gives `dmg.fire` −200; `silvered` gives `dmg.umbral` +300.
4. **Mass** is `vol × density`; doc 04 §11.6 counts it.
5. **Crafter material bonus.** Crafted armor and shields store `mat_bonus_bp` = 500 if the crafter has Smithing 100, plus 500 if the crafter has the Artificer talent `reinforced` (both doc 04); `mat_ar_bp = min(14000, ar_bp(primary) + mat_bonus_bp)`. Example: steel heavy armor from a Smithing-100 crafter with `reinforced` has `mat_ar_bp = min(14000, 11000 + 1000) = 12000`.

Example: a medium body piece in `mat.bronze`, ilvl 4, Common, Normal, +0: torso AR `20 → × T(4) = 23 → 23 → 23 → 23 → × 9000 = 20`; mass `floor_div(1017 × 8800, 1000) = 8949` g; poise 6 (bronze toughness 6).

## 4. Crafting quality

### 4.1 Which outputs roll

Weapons, armor, off-hands, ward generators, tools, gadgets and parts roll quality. Material units, placeables, ammunition, food and other consumables are `q.normal` and draw nothing. An output entry with `count > 1` of a quality-rolling template (3 thrown weapons, 5 lockpicks, 2 or 3 mines) rolls once for the whole stack, so its units stack; an `output index` (the `seq` of §1.3) counts output entries, not units.

### 4.2 Roll

```rust
const TOOL_TERM: [i32; 6] = [-60, 0, 10, 20, 30, 40];             // tooltier.t0..t5 of the best toolkit carried
const Q_THRESH: [i32; 6] = [100, 200, 300, 400, 500, 650];         // Poor, Normal, Good, Excellent, Masterwork, Peerless
fn craft_quality(c: &Crafter, st: &Station, r: &Recipe, mat_tier: u8, helpers: &[i32], rng: &mut Rng) -> u8 {
    let skill = c.skill(r.skill);
    let center = 200 + floor_div(5 * skill, 2)
        + 30 * (st.tier as i32 - 1)                                 // station.hand counts as tier 1
        + TOOL_TERM[c.toolkit_tier() as usize]
        + 15 * min(2, st.tier as i32 - mat_tier as i32)             // st.tier >= mat_tier is required
        + helpers.iter().take(2).map(|h| min(25, floor_div(*h, 4))).sum::<i32>()   // helpers sorted descending: two highest skills
        + clamp(c.sub(r.skill.governing()) - 40, -20, 40);
    let noise = rng.bell3(141) as i32 - 210;                        // 3 draws, mean 0, s.d. 70.5
    let score = center + noise;
    let mut q = Q_THRESH.iter().filter(|t| score >= **t).count() as u8;   // 0 = q.awful .. 6 = q.peerless
    q = min(q, min(skill_cap(skill), station_cap(st)));
    if r.skill == SMITHING && c.skill(SMITHING) >= 50 && r.output.is_weapon_or_armor() && q == 2 { q = 3; }  // doc 04 Smithing 50
    if c.has_talent(ARTIFICER_MASTERWORK) { q = min(6, q + 1); }    // doc 04 talent; ignores caps
    q
}
```

`mat_tier` is the highest tier among the output's component materials; a station of lower tier refuses a quality-rolling recipe (placeables, food and materials are exempt). `skill_cap`: skill 0..=24 Good, 25..=49 Excellent, 50..=74 Masterwork, 75 and above Peerless. `station_cap`: `station.hand` Normal, tier 1 Good, tier 2 Excellent, tier 3 Masterwork, tiers 4 and 5 Peerless. Helpers are other Errata within `CRAFT_REACH_MM` of the station for the whole channel, not channeling and not `in_combat`; each contributes its own level in the recipe's skill.

### 4.3 Distribution table

Exact by enumeration of the 141³ noise outcomes. `attr` is `d(governing sub)`.

| Skill | Station | Toolkit | Material term | Helpers | attr | Center | Cap | Awful | Poor | Normal | Good | Excellent | Masterwork | Peerless |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0 | T1 | t1 | 0 | 0 | 0 | 200 | Good | 8.13 | 41.60 | 41.91 | 8.35 | 0 | 0 | 0 |
| 20 | T1 | t1 | 0 | 0 | 0 | 250 | Good | 1.35 | 23.32 | 50.22 | 25.11 | 0 | 0 | 0 |
| 20 | T2 | t1 | 15 | 0 | 0 | 295 | Good | 0.02 | 9.26 | 43.11 | 47.61 | 0 | 0 | 0 |
| 35 | T2 | t2 | 15 | 0 | 5 | 347 | Excellent | 0 | 1.56 | 24.45 | 50.20 | 23.79 | 0 | 0 |
| 50 | T2 | t2 | 15 | 0 | 10 | 390 | Excellent | 0 | 0.05 | 10.48 | 44.51 | 44.96 | 0 | 0 |
| 50 | T3 | t3 | 30 | 0 | 10 | 445 | Masterwork | 0 | 0 | 1.71 | 25.21 | 50.15 | 22.93 | 0 |
| 75 | T3 | t3 | 15 | 0 | 20 | 502 | Masterwork | 0 | 0 | 0 | 7.69 | 40.97 | 51.33 | 0 |
| 75 | T4 | t4 | 30 | 0 | 20 | 557 | Peerless | 0 | 0 | 0 | 0.94 | 20.74 | 68.31 | 10.02 |
| 100 | T4 | t4 | 30 | 0 | 30 | 630 | Peerless | 0 | 0 | 0 | 0 | 3.16 | 57.13 | 39.72 |
| 100 | T5 | t5 | 30 | 50 | 40 | 730 | Peerless | 0 | 0 | 0 | 0 | 0 | 13.37 | 86.63 |

Percentages, before the Smithing 50 and talent shifts. Peerless needs skill 75 and a tier-4 station; the noise spread matches the R5 RimWorld fit (σ 0.7 tier).

### 4.4 Crafted item fields

`rarity = rar.common`; `buc = buc.uncursed`; `ilvl` = current Folio index (`d_eff` in Abridged mode), which is also its `src_lvl` (Canon §13.8.1); `origin_genre` = the local genre at the crafter, or the Folio's genre in a seam zone; knowledge fully identified (doc 10); `finder_slot` = crafter's slot; `crafted = 1`; manufacturer per §7.4; `gear_tier` per doc 11. Weapons get perk columns 1 to `n`, where `n` is 1 at Normal or below, 2 at Good or Excellent, 3 at Masterwork, 4 at Peerless; with the Artificer talent `modular` (doc 04) each column offers one more option, at most 3; the fourth `rng.craft` draw (`next_u64`) seeds doc 11 `roll_perks(sub(seed, "loot.perk"))`, and column 5 holds the genre trait. Other outputs ignore the fourth draw, so the count stays 4.

Skill XP per completed craft (doc 04 §5.2): `1000 × input_units + 2000 × recipe_tier`, × 15000 if quality ≥ Excellent. Process recipes have `recipe_tier` 0. Consumable gadget recipes completed at Tinkering ≥ 25 yield one more unit, and reusable gadgets hold one more charge while their owner has Tinkering ≥ 25 (doc 04 Tinkering 25; §11.1 rule 9). Ammunition crafted by an Erratum with the Artificer talent `smart_rounds` is flagged `smart` (§6.6).

## 5. Stations

### 5.1 Station list

| ID | Functions | Tiers | Buildable | Found in | Fuel |
|---|---|---|---|---|---|
| `station.hand` | 3×3 grid; recipes marked hand; quality cap Normal | — | always available | — | no |
| `station.campfire` | cooking tier 1; smelting with melt cap 12000; charcoal | 1 | yes, `block.campfire` (a light source, Canon §4.3) | `room.campfire` service | yes |
| `station.workbench` | 3×3 grid; assembly; fabrication; ammunition; Tinkering repair; salvage; graft extraction; implant crafting (doc 04 Tinkering 75); the doc 04 §9.3 augment station (300-tick install) | 1–5 | no | `room.workshop`, Colophon | no |
| `station.forge` | smelting and alloying with melt cap; 3×3 smithing grid; shields; Smithing repair; forge reforge; recast | 1–5 | no | `room.forge` | yes |
| `station.loom` | tanning, weaving, cloth and light armor | 1–5 | no | `room.workshop`, `room.stable` | no |
| `station.kitchen` | every cooking recipe | 1–5 | no | `room.larder` | yes |
| `station.alchemy` | doc 08 brewing; five apparatus slots, one per doc 08 §9.1 kind (mortar, retort, alembic, calcinator, vaporizer); worldgen installs a mortar, retort, alembic and calcinator of the station's tier | 1–5 | no | `room.alchemy_lab` | no |
| `station.enchanting` | doc 06 §9 enchanting; ritual processes | 1–5 | no | `room.shrine`, `room.library` | no |
| `station.scriptorium` | doc 11 Rewrite and reforge; doc 10 services | 1 | no | `room.scriptorium`, Colophon | no |

Forge melt caps by tier: 16000, 18000, 20000, 32000, 40000 DeciC; the campfire's cap is 12000. A smelting, alloying or glass recipe requires `melt_cap >= melt_decic(output)`. **Pour mode:** a `recipe.smelt.*` job queued while its owner holds an empty `apparatus.vessel.crucible` (doc 08) delivers each output unit as 250 ml of `liq.molten_metal` into that crucible instead of a metal unit; the job stalls (no tick consumed) while the crucible is full.

Tier at worldgen: `clamp(1 + floor_div(d_eff - 1, 5), 1, 5)`, plus 1 (maximum 5) when doc 12's `room_bias` for that room kind in that genre is at least 15000 (Steampunk forge and workshop, Post-Apocalypse workshop). Marginalia stations are tier 5. The Colophon workbench takes the Folio's tier. Stations are world fixtures with dependency none and never Contradict.

### 5.2 Genre skins

| Genre | Workbench | Forge | Loom | Kitchen | Campfire |
|---|---|---|---|---|---|
| Mythic Fantasy | Joiner's Bench | Dwarf Forge | Weaver's Frame | Hearth Kitchen | Campfire |
| Gothic Horror | Undertaker's Bench | Iron Furnace | Tanner's Rack | Scullery | Grave Brazier |
| Cosmic Horror | Expedition Table | Abyssal Kiln | Net-Mender | Galley | Signal Fire |
| Space Opera | Fabricator Bay | Fusion Smelter | Synthweave Loom | Galley Unit | Thermal Beacon |
| Cyberpunk | Fab Station | Arc Smelter | Print Loom | Noodle Cart | Barrel Fire |
| Post-Apocalypse | Scrap Bench | Junk Furnace | Hide Rack | Can Cooker | Burn Barrel |
| Weird West | Gunsmith's Bench | Blacksmith's Forge | Tanning Rack | Chuckwagon | Cookfire |
| Hardboiled Noir | Back-Room Bench | Foundry Pot | Tailor's Table | Diner Grill | Trash Fire |
| Wuxia | Artisan's Table | Sword Furnace | Silk Loom | Wok Stove | Cooking Fire |
| Steampunk | Tinker's Bench | Steam Foundry | Mechanical Loom | Boiler Range | Coal Stove |
| Fairy Tale | Cobbler's Bench | Witch's Anvil | Spinning Wheel | Gingerbread Oven | Hearthfire |
| Mythic Antiquity | Artisan's Bench | Hephaestian Forge | Penelope's Loom | Hearth of Hestia | Altar Fire |
| Oneiric | Dream Bench | Ember of Sleep | Thread of Morning | Dream Feast | Night Light |
| The Redaction | Form Desk | Shredder Furnace | Binding Press | Break Room | Incinerator |
| The Unwritten | Margin Desk | Blank Forge | Blank Loom | Blank Table | Blank Fire |
| The Ur-Text | First Bench | First Forge | First Loom | First Table | First Fire |

`station.alchemy` skins are doc 08 §9.1's station column. Enchanting and Scriptorium skins:

| Genre | Enchanting | Scriptorium |
|---|---|---|
| Mythic Fantasy | Rune Altar | Scriptorium |
| Gothic Horror | Black Chapel Lectern | Parish Records Room |
| Cosmic Horror | Star-Chart Table | Restricted Stacks |
| Space Opera | Psi Resonator | Data Archive |
| Cyberpunk | Glitch Shrine | Data Broker Booth |
| Post-Apocalypse | Radio Altar | Library Ruin |
| Weird West | Hex Table | Land Office |
| Hardboiled Noir | Fortune-Teller's Table | Records Office |
| Wuxia | Talisman Desk | Sutra Pavilion |
| Steampunk | Aetheric Bench | Patent Office |
| Fairy Tale | Wishing Well | Storyteller's Nook |
| Mythic Antiquity | Temple Brazier | Library of Echoes |
| Oneiric | Dream Loom | Dream Diary |
| The Redaction | Classified Terminal | Archive Desk |
| The Unwritten | Blank Altar | Margin Desk |
| The Ur-Text | First Altar | First Scriptorium |

### 5.3 Station state and process queue

```rust
struct StationState { id: Id32, tier: u8, fuel_ticks: u32, fuel_slot: Option<ItemStack>,
                      queue: Vec<ProcessJob> /* <= 4 */, out: Vec<(u8 /* slot */, ItemStack)> }
struct ProcessJob { recipe: Id32, owner_slot: u8, ticks_each: u32, remaining: u32, batch_left: u16 }

fn station_tick(s: &mut StationState) {                                  // sys.crafting, ascending Gidx
    let Some(j) = s.queue.first_mut() else { return };
    if recipe(j.recipe).fueled {
        if s.fuel_ticks == 0 {
            match s.fuel_slot.take_one() { Some(u) => s.fuel_ticks = fuel_value(u), None => return } // stalled
        }
        s.fuel_ticks -= 1;
    }
    j.remaining -= 1;
    if j.remaining == 0 {
        s.out.push((j.owner_slot, recipe(j.recipe).output()));            // instanced to the owner
        j.batch_left -= 1;
        if j.batch_left == 0 { s.queue.remove(0); } else { j.remaining = j.ticks_each; }
    }
}
```

`ticks_each = ceil_div(base_ticks × 10000, min(25000, 10000 + 50 × skill + 1000 × (station.tier - recipe.station_tier)))`, with the owner's skill snapshotted when queued. Inputs are removed when the job is queued. Fuel values: `mat.coal` 1200, any `wood` 300, `mat.polymer` and `mat.rubber` 450, `mat.paper` 60, `mat.leaves` 30. A campfire whose `fuel_ticks` reaches 0 becomes `block.campfire_out` and turns back into `block.campfire` when fuel is added (cause `grid.cause.toggled`). Queued jobs and uncollected outputs are lost at `ev.folio_exited`.

### 5.4 Genre legality

A crafted output takes the local genre as `origin_genre`. The craft is refused when that genre has no skin for the output archetype (doc 11 §4) or when the skin's dependency set contains an axis whose local axiom is 0 (Canon §5.3). Ritual processes state their own axiom minimums. In `genre.unwritten` only the Colophon scriptorium exists; hand recipes still work there.

## 6. Recipes

### 6.1 Kinds

| Kind | Rule |
|---|---|
| `recipe.kind.shaped` | 3×3 grid; the trimmed grid must equal the pattern or its horizontal mirror |
| `recipe.kind.shapeless` | the grid holds exactly the listed keys and counts, in any cells |
| `recipe.kind.assembly` | named component cells plus optional part slots (§7) |
| `recipe.kind.process` | queued, timed, at a station (§5.3); never rolls quality |
| `recipe.kind.disassembly` | reverses gear into materials, parts or grafts (§8.5) |

Grid matching (§6.3) covers shaped and shapeless recipes without variants. Recipes with variants (armor slots, shields, wards, parts) and every assembly, process, disassembly and cooking recipe are chosen from the station's recipe list, so they never take part in grid matching.

### 6.2 Schema

```json
[
  { "id": "recipe.forge.blade_long", "kind": "recipe.kind.shaped",
    "station": "station.forge", "station_tier": 1, "skill": "skill.smithing", "skill_min": 10,
    "recipe_tier": 1, "ticks": 360, "pool": "base", "learn": 0, "fuel": 0,
    "pattern": ["B", "B", "H"],
    "keys": { "B": { "role": "mat.role.blade" }, "H": { "role": "mat.role.haft" } },
    "output": { "template": "arch.melee.blade_long", "count": 1, "primary": "B", "secondary": "H" } },
  { "id": "recipe.alloy.steel", "kind": "recipe.kind.process",
    "station": "station.forge", "station_tier": 2, "skill": "skill.smithing", "skill_min": 25,
    "recipe_tier": 0, "ticks": 600, "pool": "base", "learn": 0, "fuel": 1,
    "inputs": [ { "mat": "mat.iron", "count": 2 }, { "mat": "mat.coal", "count": 1 } ],
    "output": { "template": "mat.steel", "count": 2 } },
  { "id": "recipe.bench.pistol", "kind": "recipe.kind.assembly",
    "station": "station.workbench", "station_tier": 1, "skill": "skill.tinkering", "skill_min": 10,
    "recipe_tier": 1, "ticks": 360, "pool": "base", "learn": 0, "fuel": 0,
    "components": { "primary": { "role": "mat.role.frame", "count": 1 }, "secondary": { "role": "mat.role.haft", "count": 1 } },
    "slots": ["barrel", "feed", "optic", "stock", "graft"],
    "output": { "template": "arch.ranged.pistol", "count": 1 } },
  { "id": "recipe.hand.bandage", "kind": "recipe.kind.shapeless",
    "station": "station.hand", "station_tier": 0, "skill": "skill.medicine", "skill_min": 0,
    "recipe_tier": 0, "ticks": 30, "pool": "base", "learn": 0, "fuel": 0,
    "keys": { "T": { "tag": "mat.tag.fabric" } }, "counts": { "T": 1 },
    "output": { "template": "item.tool.bandage", "count": 3 } },
  { "id": "recipe.disassemble.extract_graft", "kind": "recipe.kind.disassembly",
    "station": "station.workbench", "station_tier": 3, "skill": "skill.tinkering", "skill_min": 50,
    "alt_skill": "skill.smithing", "recipe_tier": 0, "ticks": 600, "pool": "recipes_2", "learn": 1, "fuel": 0,
    "input": { "rarity_min": "rar.rare", "needs_manufacturer": 1 },
    "output": { "kind": "graft_of_input_manufacturer", "count": 1 } }
]
```

Key kinds: `{"mat"}`, `{"role"}`, `{"tag"}`, `{"template"}`, `{"liquid", "ml"}` (drawn from held containers, doc 08); process `inputs` use the same key kinds. `pool` is `base`, `recipes_2` (the recipe enters the content set with `unlock.loot.recipes_2`), `gadgets_2` (with `unlock.loot.gadgets_2`) or `craft` (base content, never in a loot pool). `alt_skill`, when present, may stand in for `skill`; the crafter uses the higher of the two. `station.hand` has a 3×3 grid like every grid station. Every cell bound to one key letter must hold the same template. Station `S` with tier `t` runs a recipe when `S` is the recipe's station (a `station.hand` recipe runs anywhere; a campfire cooking recipe also runs at a kitchen; a campfire smelting or charcoal recipe also runs at a forge) and `t >= station_tier`.

### 6.3 Matching

```rust
fn match_shaped(grid: &Grid, r: &Recipe) -> Option<BTreeMap<char, Id32>> {
    let g = grid.trimmed();                                          // drop empty outer rows and columns
    for pat in [r.pattern.clone(), r.pattern.mirrored()] {
        if g.dims() != pat.dims() { continue; }
        let mut bind = BTreeMap::new();
        let ok = g.cells().zip(pat.cells()).all(|(cell, key)| match (cell, key) {
            (None, None) => true,
            (Some(it), Some(k)) => r.keys[&k].accepts(it) && *bind.entry(k).or_insert(it.template) == it.template,
            _ => false,
        });
        if ok { return Some(bind); }
    }
    None
}
```

Lookup tries recipes in ascending ID among those the crafter knows and the station runs; the first match wins. The content loader rejects two shaped recipes whose masks (or a mirror) are equal while every cell pair's accepted material sets intersect, and two shapeless recipes runnable at one station whose key counts admit a one-to-one pairing of equal counts with intersecting accepted sets (`recipe_overlap`), so at most one recipe ever matches.

### 6.4 Craft channel state machine

| State | Event | Next |
|---|---|---|
| Idle | `StartCraft{recipe, count ≤ stack limit}` passing every check below | Channeling |
| Idle | any check fails | Idle, with the failing reason |
| Channeling | each tick, all conditions hold | Channeling, `progress += 1` |
| Channeling | `progress >= needed` | Done |
| Channeling | moved more than 1000 mm from the start point; took non-DoT damage; gained `st.staggered`, `st.stunned`, `st.frozen`, `st.knocked_down`, `st.sleeping` or `st.downed`; the station broke; cancel input | Idle; nothing consumed |
| Done | same tick | Idle: inputs consumed, quality rolled per output (seq = output index), items created, `ev.item_crafted` per item, skill XP |

Checks: recipe known (§6.5); station within `CRAFT_REACH_MM` and of sufficient tier and melt cap; `skill >= skill_min`; for quality-rolling outputs, station tier at least the highest input material tier; genre legality (§5.4); inputs present (they stay reserved while channeling).

```
needed = ceil_div(ticks × count × 10000, min(30000, 10000 + 50 × skill + 1000 × (st.tier - recipe.station_tier) + 1500 × helpers))
// st.tier: the station's tier, station.hand counting as tier 1 as in §4.2; helpers: the number counted, 0..=2
```

### 6.5 Knowledge

Every recipe in the run content set with `learn: 0` is known by every Erratum. A recipe with `learn: 1` must be learned once per run: reading a schematic book (doc 11 `item.book.schematic` with `teaches`, a 60-tick read that consumes it), an NPC `teach_recipe` effect (doc 13) or a doc 04 kit grant. `unlock.loot.recipes_2` (doc 14) adds the 12 recipes marked `r2` below to the content set and their schematics to the loot pool.

### 6.6 Catalog

Keys: B blade, D head, H haft, L limb, F frame, P plate, S soft, O focus (roles); W wood, M metal, K stone, T fabric (tags); C `mat.coal`, G `mat.glass`, Y `mat.sand`, E `mat.web`. Patterns list rows top to bottom separated by `/`; `.` is empty. Columns: station and minimum tier, skill and minimum, ticks, `recipe_tier`. Flags: `L` learn, `r2` pool `loot.pool.recipes_2`, `F` fueled.

**Process recipes** (`recipe_tier` 0, quality never rolls)

| ID | Station | Skill | Inputs → output | Ticks | Flags |
|---|---|---|---|---|---|
| `recipe.smelt.copper` | forge 1 or campfire | smithing 0 | 1 `mat.copper.ore` → 1 `mat.copper` | 200 | F |
| `recipe.smelt.tin` | forge 1 or campfire | smithing 0 | 1 `mat.tin.ore` → 1 `mat.tin` | 160 | F |
| `recipe.smelt.zinc` | forge 1 or campfire | smithing 0 | 1 `mat.zinc.ore` → 1 `mat.zinc` | 160 | F |
| `recipe.smelt.iron` | forge 1 | smithing 0 | 1 `mat.iron.ore` → 1 `mat.iron` | 240 | F |
| `recipe.smelt.silver` | forge 1 | smithing 10 | 1 `mat.silver.ore` → 1 `mat.silver` | 240 | F |
| `recipe.smelt.gold` | forge 1 | smithing 10 | 1 `mat.gold.ore` → 1 `mat.gold` | 240 | F |
| `recipe.smelt.titanium` | forge 2 | smithing 40 | 1 `mat.titanium.ore` → 1 `mat.titanium` | 480 | F |
| `recipe.smelt.orichalcum` | forge 3 | smithing 50 | 1 `mat.orichalcum.ore` → 1 `mat.orichalcum` | 600 | F |
| `recipe.smelt.aetherium` | forge 3 | smithing 50 | 1 `mat.aetherium.ore` → 1 `mat.aetherium` | 600 | F |
| `recipe.smelt.star_metal` | forge 4 | smithing 60 | 1 `mat.star_metal.ore` → 1 `mat.star_metal` | 720 | F |
| `recipe.alloy.bronze` | forge 1 | smithing 10 | 3 copper + 1 tin → 4 `mat.bronze` | 360 | F |
| `recipe.alloy.brass` | forge 1 | smithing 10 | 2 copper + 1 zinc → 3 `mat.brass` | 360 | F |
| `recipe.alloy.steel` | forge 2 | smithing 25 | 2 iron + 1 coal → 2 `mat.steel` | 600 | F |
| `recipe.alloy.cast_iron` | forge 1 | smithing 10 | 1 `mat.iron.ore` + 1 coal → 1 `mat.cast_iron` | 300 | F |
| `recipe.forge.cold_iron` | forge 2 | smithing 25 | 1 iron → 1 `mat.cold_iron` (cold-hammered, no fuel) | 600 | L |
| `recipe.ritual.hexed_silver` | enchanting 2 | inscription 20 | 1 silver + 1 bone → 1 `mat.hexed_silver`; local MYT ≥ 4 | 900 | L |
| `recipe.ritual.moonsilver` | enchanting 3 | inscription 40 | 2 silver + 1 glass → 1 `mat.moonsilver`; local ARC ≥ 6 | 1200 | L r2 |
| `recipe.kiln.glass` | forge 1 | smithing 0 | 2 sand → 1 `mat.glass` | 300 | F |
| `recipe.kiln.brick` | forge 1 | smithing 0 | 2 soil → 2 `mat.brick` | 300 | F |
| `recipe.kiln.charcoal` | campfire | survival 0 | 2 W → 1 `mat.coal` | 300 | — |
| `recipe.kiln.alch_charcoal` | campfire | survival 0 | 1 W → 2 `ing.charcoal` (doc 08 §8.2) | 300 | — |
| `recipe.loom.tan_leather` | loom 1 | survival 0 | 1 `mat.hide` → 1 `mat.leather` | 600 | — |
| `recipe.loom.weave_silk` | loom 2 | survival 20 | 3 web → 1 `mat.silk` | 600 | — |
| `recipe.fab.polymer` | workbench 2 | tinkering 25 | 1 rubber + 1 coal → 2 `mat.polymer` | 600 | — |
| `recipe.fab.carbon_fiber` | workbench 3 | tinkering 50 | 2 polymer + 1 coal → 1 `mat.carbon_fiber` | 900 | L |
| `recipe.fab.plasteel` | workbench 3 | tinkering 50 | 1 steel + 1 polymer → 1 `mat.plasteel` | 900 | L |
| `recipe.fab.nanoweave` | workbench 4 | tinkering 75 | 1 carbon fiber + 1 silk → 1 `mat.nanoweave` | 1200 | L r2 |

**Ammunition recipes** (`recipe.kind.process` at a workbench, doc 05 §8.1 families; quality never rolls; M is any `metal` material, B any blade-role material)

| ID | Station | Skill | Inputs → output | Ticks | Flags |
|---|---|---|---|---|---|
| `recipe.bench.ammo_light` | workbench 1 | tinkering 0 | 1 M + 1 coal → 24 `ammo.light` | 240 | — |
| `recipe.bench.ammo_heavy` | workbench 1 | tinkering 10 | 1 M + 1 coal → 16 `ammo.heavy` | 240 | — |
| `recipe.bench.ammo_shell` | workbench 1 | tinkering 10 | 1 M + 1 coal → 5 `ammo.shell` | 240 | — |
| `recipe.bench.ammo_draw` | workbench 1 | tinkering 0 | 1 W + 1 B → 6 `ammo.draw` | 180 | — |
| `recipe.bench.ammo_rifle` | workbench 2 | tinkering 25 | 2 M + 2 coal → 7 `ammo.rifle` | 300 | — |
| `recipe.bench.ammo_cell` | workbench 2 | tinkering 25 | 1 copper + 1 coal → 30 `ammo.cell` | 300 | — |
| `recipe.bench.ammo_ordnance` | workbench 3 | tinkering 50 | 2 M + 2 coal → 1 `ammo.ordnance` | 600 | L |

With `mat.iron` as M, every ammunition output is worth at most 1.75 times its inputs at doc 05 §8.1 values (rifle: 7 rounds = 21 Lucre against 12; light: 24 rounds = 4.8 against 6), so even at doc 11 §12.2's 5500 Bp sell cap a sale returns less than the inputs are worth (rifle 11.55 against 12); with the cheapest metal (`mat.scrap_iron`, 1 Lucre) a craft can sell for a few Lucre more than its inputs, which is bounded by node and salvage supply because shops never restock materials (doc 11 §12.3). Ammunition crafted by an Erratum with `smart_rounds` is flagged `smart`; doc 05 keeps a family's smart and plain units as two reserve counters, fires smart units first, and each smart shot carries `DmgAdd +1000` (doc 04).

**Shaped and shapeless recipes** (quality rolls for gear and tools only)

| ID | Station | Skill | Pattern or inputs → output | Ticks | Tier | Flags |
|---|---|---|---|---|---|---|
| `recipe.forge.dagger` | forge 1 | smithing 0 | `B/H` → `arch.melee.dagger` | 240 | 1 | |
| `recipe.forge.blade_short` | forge 1 | smithing 5 | `.B/H.` → `arch.melee.blade_short` | 300 | 1 | |
| `recipe.forge.blade_long` | forge 1 | smithing 10 | `B/B/H` → `arch.melee.blade_long` | 360 | 1 | |
| `recipe.forge.blade_great` | forge 2 | smithing 25 | `.B/BB/H.` → `arch.melee.blade_great` | 480 | 2 | |
| `recipe.forge.axe` | forge 1 | smithing 5 | `BH` → `arch.melee.axe` | 300 | 1 | |
| `recipe.forge.axe_great` | forge 2 | smithing 25 | `BB/.H/.H` → `arch.melee.axe_great` | 480 | 2 | L r2 |
| `recipe.forge.mace` | forge 1 | smithing 5 | `DH/.H` → `arch.melee.mace` | 300 | 1 | |
| `recipe.forge.warhammer` | forge 2 | smithing 25 | `DD/DD/.H` → `arch.melee.warhammer` | 480 | 2 | |
| `recipe.forge.spear` | forge 1 | smithing 5 | `..B/.H./H..` → `arch.melee.spear` | 300 | 1 | |
| `recipe.forge.halberd` | forge 2 | smithing 25 | `.B/BH/.H` → `arch.melee.halberd` | 480 | 2 | L r2 |
| `recipe.bench.quarterstaff` | workbench 1 | smithing 0 | `HHH` → `arch.melee.quarterstaff` | 240 | 1 | |
| `recipe.forge.fist` | forge 1 | smithing 10 | `S.D` → `arch.melee.fist` | 240 | 1 | L r2 |
| `recipe.loom.whip` | loom 1 | smithing 10 | `S./.S/H.` → `arch.melee.whip` | 300 | 1 | L r2 |
| `recipe.forge.chain` | forge 2 | smithing 25 | `DD./..S` → `arch.melee.chain` | 420 | 2 | L r2 |
| `recipe.forge.thrown` | forge 1 | throwing 0 | `B/B` → 3 `arch.ranged.thrown` | 240 | 1 | |
| `recipe.forge.pick` | forge 1 | smithing 0 | `DDD/.H./.H.` → `item.tool.pick` | 240 | 1 | |
| `recipe.bench.toolkit` | workbench 1 | tinkering 0 | shapeless 1 F + 1 H → `item.tool.toolkit` | 180 | 1 | |
| `recipe.bench.lockpicks` | workbench 1 | tinkering 0 | shapeless 1 M → 5 `item.tool.lockpicks` | 120 | 1 | |
| `recipe.bench.hacking_deck` | workbench 2 | tinkering 25 | shapeless 1 F + 2 copper + 1 G → 1 `item.tool.hacking_deck` | 360 | 2 | |
| `recipe.hand.torch` | hand | fieldcraft 0 | `C/W` → 4 `item.tool.torch` | 30 | 0 | |
| `recipe.hand.wall_torch` | hand | fieldcraft 0 | shapeless 1 `item.tool.torch` → 1 `block.torch` | 10 | 0 | |
| `recipe.hand.bandage` | hand | medicine 0 | shapeless 1 T → 3 `item.tool.bandage` | 30 | 0 | |
| `recipe.hand.barricade_wood` | hand | fieldcraft 0 | `WWW/WWW` → 4 `block.barricade.wood` | 120 | 0 | |
| `recipe.hand.barricade_stone` | hand | fieldcraft 0 | `KK/KK` → 2 `block.barricade.stone` | 120 | 0 | |
| `recipe.hand.barricade_gate` | hand | fieldcraft 10 | `WW/WW/WW` → 1 `block.barricade.gate` | 120 | 0 | |
| `recipe.forge.barricade_metal` | forge 1 | fieldcraft 10 | `MMM/M.M/MMM` → 4 `block.barricade.metal` | 300 | 1 | |
| `recipe.hand.ladder_wood` | hand | fieldcraft 0 | `W.W/WWW/W.W` → 4 `block.ladder.wood` | 60 | 0 | |
| `recipe.loom.ladder_rope` | loom 1 | fieldcraft 10 | `T.T/TTT/T.T` → 4 `block.ladder.rope` | 120 | 0 | |
| `recipe.hand.rope_ladder` | hand | fieldcraft 0 | shapeless 1 `item.tool.rope` → 1 `block.ladder.rope` | 30 | 0 | |
| `recipe.forge.ladder_metal` | forge 1 | fieldcraft 10 | `M.M/MMM/M.M` → 4 `block.ladder.metal` | 120 | 1 | |
| `recipe.hand.bridge_plank` | hand | fieldcraft 0 | `WWW/W.W` → 4 `block.bridge.plank` | 90 | 0 | |
| `recipe.forge.bridge_metal` | forge 1 | fieldcraft 10 | `MMM/M.M` → 4 `block.bridge.metal` | 180 | 1 | |
| `recipe.hand.campfire` | hand | fieldcraft 0 | `.C./WWW` → 1 `block.campfire` | 60 | 0 | |
| `recipe.bench.lantern` | workbench 1 | tinkering 0 | `.M./MGM/.M.` → 1 `block.lantern` | 180 | 1 | |
| `recipe.bench.cover_sandbag` | workbench 1 | fieldcraft 0 | shapeless 2 Y + 1 T → 2 `block.cover.sandbag` | 120 | 0 | |
| `recipe.forge.cover_plate` | forge 1 | fieldcraft 10 | `MM/MM` → 2 `block.cover.plate` | 240 | 1 | |
| `recipe.bench.trap_spikes` | workbench 1 | fieldcraft 10 | `BBB` → 2 `block.trap.spikes` | 180 | 1 | |
| `recipe.hand.web_trap` | hand | fieldcraft 0 | shapeless 3 E → 1 `block.web` | 60 | 0 | |
| `recipe.bench.shrine_lamp` | workbench 2 | inscription 10 | shapeless 1 G + 1 `arcane_reagent` + 1 M → 1 `block.shrine_lamp` | 300 | 2 | L r2 |
| `recipe.loom.armor_cloth` | loom 1 | smithing 0 | shapeless T × (1/1/1/1/1) → `arch.armor.<slot>_cloth` | 240 | 1 | |
| `recipe.loom.armor_light` | loom 1 | smithing 10 | shapeless non-fabric S × (1/5/1/3/2) → `arch.armor.<slot>_light` | 360 | 1 | |
| `recipe.forge.armor_medium` | forge 1 | smithing 15 | shapeless P × (1/5/1/3/1) → `arch.armor.<slot>_medium` | 480 | 1 | |
| `recipe.forge.armor_heavy` | forge 2 | smithing 30 | shapeless P × (2/8/1/4/2) → `arch.armor.<slot>_heavy` | 600 | 2 | |

Armor counts are head/body/hands/legs/feet; the player picks the slot. The torch, bandage, lockpick, hacking-deck, toolkit and pick outputs are doc 11 `item.tool.*` templates whose instance carries the materials used; lockpicks and the hacking deck roll quality, which sets doc 09's `TOOL_Q`. Every `block.*` output is a `cat.placeable` item whose template is that block ID (Canon §4.3). A held `item.tool.torch` is a doc 11 light tool and never places a block; `recipe.hand.wall_torch` turns it into a placeable `block.torch`.

**Assembly recipes** (components, then optional part slots of §7.1)

| ID | Station | Skill | Components → output | Ticks | Tier | Flags |
|---|---|---|---|---|---|---|
| `recipe.bench.pistol` | workbench 1 | tinkering 10 | F1 H1 → `arch.ranged.pistol` | 360 | 1 | |
| `recipe.bench.pistol_heavy` | workbench 2 | tinkering 25 | F1 H1 → `arch.ranged.pistol_heavy` | 420 | 2 | |
| `recipe.bench.smg` | workbench 2 | tinkering 25 | F2 H1 → `arch.ranged.smg` | 480 | 2 | |
| `recipe.bench.rifle_auto` | workbench 2 | tinkering 30 | F2 H2 → `arch.ranged.rifle_auto` | 540 | 2 | |
| `recipe.bench.rifle_marksman` | workbench 2 | tinkering 30 | F2 H2 → `arch.ranged.rifle_marksman` | 540 | 2 | |
| `recipe.bench.shotgun` | workbench 2 | tinkering 25 | F2 H2 → `arch.ranged.shotgun` | 480 | 2 | |
| `recipe.bench.sniper` | workbench 3 | tinkering 50 | F3 H2 → `arch.ranged.sniper` | 720 | 3 | L r2 |
| `recipe.bench.launcher` | workbench 3 | tinkering 50 | F4 H1 → `arch.ranged.launcher` | 720 | 3 | L |
| `recipe.bench.beam` | workbench 3 | tinkering 50 | F3 O1 → `arch.ranged.beam` | 720 | 3 | L |
| `recipe.bench.charge_rifle` | workbench 1 | smithing 10 | L2 S1 → `arch.ranged.charge_rifle` | 360 | 1 | |
| `recipe.bench.crossbow` | workbench 2 | tinkering 25 | L2 H4 → `arch.ranged.crossbow` | 480 | 2 | L r2 |
| `recipe.bench.staff` | workbench 1 | inscription 10 | H2 O1 → `arch.ranged.staff` | 300 | 1 | |
| `recipe.bench.focus` | enchanting 1 | inscription 10 | O1 → `arch.offhand.focus` | 240 | 1 | |
| `recipe.loom.tome` | loom 1 | inscription 10 | S1 O1 → `arch.offhand.tome` | 300 | 1 | |
| `recipe.bench.lantern_held` | workbench 1 | tinkering 0 | F1 + 1 glass → `arch.offhand.lantern` | 180 | 1 | |
| `recipe.forge.shield` | forge 1 | smithing 10 | buckler P1 H1, heater P2 H1, tower P5 H1 → `arch.offhand.shield_*` | 360 | 1 | |
| `recipe.bench.ward` | workbench 2 | tinkering 25 | aegis, kinetic, reflector F1 O1; bulwark F2 O1 → `arch.ward.*` | 480 | 2 | |
| `recipe.bench.modify` | workbench or forge 1 | item's governing skill (§8.4) | install or remove parts on any compatible item | 60 per part | 0 | |

Gadget assembly recipes (`recipe.bench.<gadget suffix>`, 13) are in §11.2, part recipes in §7.2, disassembly in §8.5 and cooking in §12.2.

## 7. Assembly and parts

### 7.1 Slots per family

| Family | Slots |
|---|---|
| Guns: `pistol`, `pistol_heavy`, `smg`, `rifle_auto`, `rifle_marksman`, `shotgun`, `sniper`, `launcher`, `beam` | barrel, feed, optic, stock, graft |
| Draw and bolt: `charge_rifle`, `crossbow` | barrel, feed, optic, stock, graft |
| Melee (all `arch.melee.*` except `unarmed`) | edge, haft, grip, graft |
| Casting: `staff`, `focus`, `tome` | focus, grip, graft |
| Shields | plate, grip, lining, graft |
| Armor | plate (medium and heavy only), lining, graft |
| Ward generators | coil, graft |
| Reusable gadgets | module_1, module_2 |

Thrown weapons, wands, lanterns, tools and consumable gadgets take no parts.

### 7.2 Part catalog

This chapter owns every `part.` ID (Canon §19). Doc 11 §5.4 mirrors the 22 loot rows (its two-letter codes, the manufacturer part lists of doc 11 §5.2 and the drop rules); the seven crafted-only rows always carry `mfr.errata.marginal_press`. Letters are doc 11 §5.1's tendency letters: D damage `base_mhp`, S attack speed, P spread, M magazine, L reload ticks, R range, H handling (swap and ADS ticks), T stamina cost, O poise damage (impact), U durability, W mass, A AR, X precision multiplier, C aether cost, V value. A "× N" modifier is the delta `N - 10000` on that stat. Every part has dependency none.

| ID | Code | Slot | Modifiers | Source |
|---|---|---|---|---|
| `part.barrel.short` | bs | barrel | H −2000, R −1500 | loot, crafted |
| `part.barrel.long` | bl | barrel | R +2500, H +1000 | loot, crafted |
| `part.barrel.heavy` | bh | barrel | D +600, S −500 | loot, crafted |
| `part.barrel.rifled` | br | barrel | P −1500 | loot, crafted |
| `part.feed.extended` | fe | feed | M +3000 | loot, crafted |
| `part.feed.quick` | fq | feed | L −1500 | loot, crafted |
| `part.feed.drum` | fd | feed | M +5000, H +2000 | loot, crafted |
| `part.optic.iron` | oi | optic | X +500 | loot, crafted |
| `part.optic.scope` | os | optic | X +1500, H +1500 | loot, crafted |
| `part.optic.reflex` | or | optic | P −800 | loot, crafted |
| `part.edge.keen` | ek | edge | pen +800 | loot, crafted |
| `part.edge.heavy` | eh | edge | O +3000, S −400 | loot, crafted |
| `part.edge.serrated` | es | edge | +20 `st.bleeding` power | loot, crafted |
| `part.haft.long` | hl | haft | reach +200 mm, S −300 | loot, crafted |
| `part.haft.light` | hg | haft | S +500, D −300 | loot, crafted |
| `part.grip.wrapped` | gw | grip | T −1200 | loot, crafted |
| `part.grip.guarded` | gg | grip | +800 Bp shield stability while blocking | loot, crafted |
| `part.focus.crystal` | fc | focus | +600 Bp spell damage | loot, crafted |
| `part.focus.conduit` | fn | focus | cast ticks × 9000 | loot, crafted |
| `part.plate.reinforced` | pr | plate | AR × 11500, W +1000 | loot, crafted |
| `part.lining.padded` | lp | lining | +400 Bp resist blunt | loot, crafted |
| `part.coil.capacitor` | cc | coil | ward capacity × 11500 | loot, crafted |
| `part.barrel.suppressor` | — | barrel | R −1000, D −200; tag `suppressor` (doc 09 §7.3: powered skins only; muzzle flash 12 → 4) | crafted |
| `part.stock.braced` | — | stock | recoil −1500, H +500 | crafted |
| `part.stock.folding` | — | stock | H −2000, recoil +1000 | crafted |
| `part.module.battery` | — | module (either module slot) | gadget duration +5000 | crafted |
| `part.module.plating` | — | module | gadget Max HP +3000 | crafted |
| `part.module.targeting` | — | module | gadget spread −3000, gadget range +2000 | crafted |
| `part.module.payload` | — | module | gadget damage (D) +2000 | crafted |

`recoil` multiplies doc 05 `recoil.v_bam` and `recoil.h_bam`. A crafted part has `mfr.errata.marginal_press`, a rolled quality, value `mul_bp(25, P(ilvl))`, mass 300 g and stack 5; parts stack only with instances of equal template, manufacturer and quality (doc 11 §14.2).

| Part recipe | Station | Skill | Variants and inputs | Ticks | Tier |
|---|---|---|---|---|---|
| `recipe.bench.part_gun` | workbench 1 | tinkering 15 | barrel short, long, heavy, rifled F1; barrel suppressor F1 + 1 S; feed (3) F1 + 1 M; optic iron F1; optic scope, reflex F1 + 1 G; stock (2) H1 | 240 | 1 |
| `recipe.forge.part_melee` | forge 1 | smithing 15 | edge (3) B1; haft (2) H1; grip wrapped S1; grip guarded P1 | 240 | 1 |
| `recipe.forge.part_armor` | forge 1 | smithing 15 | plate reinforced P2; lining padded S1 | 300 | 1 |
| `recipe.bench.part_arcane` | enchanting 1 | inscription 20 | focus crystal 1 G + 1 `arcane_reagent`; focus conduit O1 + 1 M; coil capacitor 1 copper + 1 G | 300 | 1 |
| `recipe.bench.part_module` | workbench 2 | tinkering 25 | each module F1 + 1 material with `tech_affinity >= 6` | 300 | 2 |

### 7.3 Combination formula

Stats are always rebuilt from the archetype base, so modifications never compound.

```rust
const SLOT_ORDER: [Slot; 13] = [Barrel, Feed, Optic, Stock, Edge, Haft, Grip, Focus, Plate, Lining, Coil, Module1, Module2];
fn assembled(it: &Item) -> Stats {
    let mut s = archetype_base(it.template);                       // doc 05 / doc 06 / §11
    for (k, d) in mfr_tendencies(it.manufacturer) { s.apply(k, d); }   // doc 11 §5.1
    for (k, d) in material_tendencies(it) { s.apply(k, d); }           // §3.2
    for slot in SLOT_ORDER {
        let Some(p) = it.part(slot) else { continue };
        for (k, d) in p.mods() {                                    // listed order
            let mut d = quality_scale(k, d, p.quality_bp);          // good direction × q, bad direction × (20000 − q)
            if p.mfr != it.manufacturer && p.mfr != MARGINAL_PRESS { d = toward_zero(d, CROSS_MFR_BP); }
            s.apply(k, d);
        }
    }
    s
}
// apply: tick and count stats -> max(1, round_half_even(x × (10000 + d), 10000));
//        flat stats (pen_bp, reach mm, status power, Bp bonuses, resistances) -> x + d;
//        all others -> mul_bp(x, 10000 + d).
// toward_zero(d, f) = sign(d) × mul_bp(|d|, f);  quality_scale uses toward_zero the same way.
```

Good direction is an increase for D, S, M, R, O, U, A, X, V, gadget duration, HP, range and damage, and every flat bonus; a decrease for P, L, H, T, W, C, recoil and gadget spread.

### 7.4 Manufacturer influence

A found item keeps its stamped manufacturer. A crafted item (`crafted = 1`) recomputes its manufacturer whenever its parts change: count installed non-graft parts per manufacturer, excluding the Marginal Press; the top count, ties to the lower ID, wins when it is at least 2 and that manufacturer makes the item's class (doc 11 §5.2); otherwise the item is `mfr.errata.marginal_press`. The winner's tendencies and gimmick apply exactly as at generation (doc 11 §5.1).

### 7.5 Licensed grafts

A licensed graft `part.graft.<genre_suffix>.<name>` exists for each doc 11 manufacturer except `mfr.ur_text.first_press` and `mfr.errata.marginal_press`, 38 in all, for example `part.graft.weird_west.coffin_creek`. A graft is a `cat.part` (value `mul_bp(60, P(ilvl))`, mass 300 g, stack 1) that carries its manufacturer's signature gimmick onto another manufacturer's item:

- It fills the graft slot. It is refused on Palimpsestic and Exotic items, on items whose manufacturer or `gloss_mfr` is the graft's manufacturer, and when the gimmick text has no variant for the item's class.
- The gimmick is one component with doc 11's Dep column, `origin_genre` = the graft manufacturer's genre, and its own Contradiction potency (Canon §5.5).
- Strength `GRAFT_BP`: "+N Bp" becomes `mul_bp(N, 7500)`; "× N" becomes `10000 + mul_bp(N - 10000, 7500)`; chances become `mul_bp(c, 7500)`; durations, radii, counts and non-numeric effects stay unchanged.

Example: Coffin Creek on a Rustmaw shotgun ricochets `mul_bp(5000, 7500) = 3750` Bp of a crit's `D2`; dependency verve with origin Weird West gives `Req[verve] = 5`.

### 7.6 Compatibility

1. The slot must be in the family list (§7.1); one part per slot.
2. `part.edge.keen` and `part.edge.serrated` need a `dmg.slash` share of at least 4000; haft parts need a haft secondary; plate parts need medium or heavy armor or a shield; `part.barrel.suppressor` fits the gun family only, never draw-and-bolt weapons.
3. A part with no stat present on the item, or whose every change rounds to nothing, is refused (`no_effect`).
4. Palimpsestic items take no parts.
5. Install and removal use `recipe.bench.modify`: at a workbench for guns, draw and bolt, casting, wards and gadgets; at a forge for melee, armor and shields. Removal returns the part intact. Neither changes `ilvl`, rarity, quality or affixes.

## 8. Durability, breakage, repair, salvage and reforging

### 8.1 Maximum durability

```
dur_max = DUR_BASE → × dur_mat_bp(primary) → × quality_bp → × (10000 + U) → × 20000 if made by a Smithing-100 crafter
```

Each arrow is one `mul_bp`; `U` is the manufacturer tendency (doc 11). Rustmaw's "durability × 5000" is `U = −5000`; Hephaesteum items have `dur_max = 0` and never wear. Example: a Good steel longsword has `10000 → 11350 → 12485`.

### 8.2 Wear

| Event | Wear |
|---|---|
| Melee attack performed | `max(1, ceil_div(25 × cycle, 60))`, `cycle` = windup + active + recovery (+ charge) at instance timing; × 2 when it strikes a block |
| Ranged shot or beam pulse | same formula with `cycle` = fire interval ticks, charge ticks or crank ticks |
| Throw | 1 |
| Armor piece covering a part hit | `max(1, floor_div(mitigated_mhp × 40, T(d_current)))`, `mitigated_mhp` = the step-5 reduction |
| Shield block | `max(1, floor_div(blocked_mhp × 20, T(d_current)))` |
| Ward break | 500 |
| Pick, per node broken | `10 × hardness` of the node |
| Toolkit | 10 per quality-rolling craft; 20 per repair; `REPAIR_KIT_USE` (250) per doc 04 `self_repair` use |
| Lockpicks | a doc 09 snap destroys one pick of the stack (each pick has durability 1) |
| Hacking deck | 100 per doc 09 hack attempt, success or failure |
| Reusable gadget | 500 on expiry or recall; 2500 when destroyed |

Then `wear → × (10000 + m)` for `curse.brittle` (doc 10) → `× (10000 + 2500 × r)` for `run.marg.brittle_kit` (doc 14) → `× 5000` for Dunwold; each a `mul_bp`, then `max(1, …)`. Examples: a long blade light swing (10 + 5 + 12 ticks) wears 12; an SMG shot (4 ticks) 2; a sniper shot (80 ticks) 34; a 5,000 mHP mitigation at Folio 1 wears 20.

### 8.3 Condition

| State | Durability | Effect |
|---|---|---|
| Sound | ≥ 2500 Bp of max | none |
| Worn | 1 to 2499 Bp of max | warning only |
| Broken | 0 | base function disabled; armor AR, poise and resistances count 0; tools and gadgets unusable |

An item whose primary carries `brittle` shatters at 0: it is destroyed and its salvage (§8.5) drops at the holder's feet (`ev.item_dropped`). The doc 11 Glass Cobbler shatter applies on top.

### 8.4 Repair

Governing skill: Smithing for melee, thrown, charge rifles, armor, shields, staves and picks; Tinkering for guns, crossbows, beams, ward generators, lanterns, gadgets, toolkits, hacking decks and implants; Inscription for foci and tomes. Lockpicks are never repaired.

```
restore = mul_bp(dur_max, 2500) → × (10000 + 50 × skill) → × 15000 if Smithing-governed and Smithing ≥ 25
        → × 15000 with talent field_repair → × 5000 in the field
cur = min(cap, cur + restore)    // cap = dur_max at a station, mul_bp(dur_max, 7500) in the field
```

A repair consumes 1 unit of the primary material and takes 120 ticks at the matching station or 240 ticks in the field with a toolkit. Example: the 12,485 longsword at Smithing 40 restores `3121 → 3745 → 5617`. A `kin.construct` Erratum repairs itself through doc 04's `self_repair` (Canon §16.3 "Tinkering repair heals"): one use of a `repair_kit` tool costs `REPAIR_KIT_USE` = 250 durability of the toolkit and no material, so a Normal iron toolkit (5,000) holds 20 uses; the heal of `mul_bp(max_hp_mhp, 1000 + 20 × tinkering)` is doc 04's.

### 8.5 Salvage and disassembly

| ID | Station | Skill | Rule | Ticks |
|---|---|---|---|---|
| `recipe.disassemble.salvage` | any station, or field | governing skill 0 | doc 11 §16.4; per component `units = round_half_even(mul_bp(mass_g, 6000), unit_mass_g)`, raised to 1 when 0 and the salvaged mass is at least `floor_div(unit_mass_g, 4)`; talent `scrap_savant` adds `floor_div(units, 3)` | 0 at a station, 60 in the field |
| `recipe.disassemble.strip` | workbench or forge | 0 | removes every part intact | 60 |
| `recipe.disassemble.extract_graft` | workbench 3 | Tinkering or Smithing 50 | destroys a Rare-or-better item and yields 1 graft of its manufacturer | 600 (L r2) |

Example: a steel longsword (1,334 g steel, 63 g oak) salvages `mul_bp(1334, 6000) = 800` g of steel, `round_half_even(800, 1962) = 0`, raised to 1 because 800 ≥ 490; the oak gives `mul_bp(63, 6000) = 37` g, below 187, so 0.

### 8.6 Reforging

- **Forge reforge** (doc 04 Smithing 75) at a forge of tier 3 or more uses doc 11 §16.3's offer and `reforge_count`, and costs `mul_bp(doc 11 cost, 5000)` Lucre plus `2^c` units of the primary material, `c` = `reforge_count` before.
- **Recast** replaces the primary material at a forge (metal, stone, bone, glass and crystal primaries) or workbench (others) of tier ≥ the new material's tier, with governing skill ≥ `20 × new tier`. It consumes the archetype's primary cells of the new material, salvages the old primary (§8.5), keeps every other field, caps quality at the station cap, preserves the durability ratio, recomputes mass, value and stats, and increments `reforge_count` (refused at 4).

## 9. Mining and gathering

### 9.1 Tool tiers

A pick's tier is its head material's tier (minimum `tooltier.t1`); bare hands are `tooltier.t0`. Speed `TOOL_SPEED` (Bp): t0 3000, t1 10000, t2 13000, t3 16000, t4 20000, t5 25000. Class factor: a pick on stone, metal and crystal nodes, an `arch.melee.axe` or `axe_great` (tier of its primary) on timber and wooden blocks, and anything on soft nodes give 10000; any other tool gives 5000. Every other item's tool tier is its primary material's tier (doc 05 Table 3.2d), which doc 03 §13.2 compares with a block's `tool_tier` for weapon damage to blocks. Only resource nodes and `block.tag.destructible` blocks can be mined (Canon §4.3); a resource node is an interactable prop block, mined in place and never dug out of the terrain.

### 9.2 Mining time

```rust
fn mining_ticks(node: &Block, tool_tier: u8, class_bp: i32, fieldcraft: i32, hard_bp: i32) -> Option<u32> {
    if !node.destructible() || tool_tier < node.tool_tier { return None; }   // §9.3 Req; doc 03 §13.2
    let h = floor_div(node.hardness_milli, 1000);                        // §9.3 Hardness column for nodes
    let t1 = mul_bp(30 * h, hard_bp) as i64;                             // break_ticks; doc 12 GPV hard_bp
    let speed = mul_bp(TOOL_SPEED[tool_tier as usize], class_bp) as i64;
    Some(max(1, ceil_div(t1 * 100_000_000, speed * (10000 + 50 * fieldcraft as i64))) as u32)
}
```

The Fieldcraft term `10000 + 50 × skill` is doc 04's Fieldcraft factor on melee and tool work against destructible blocks; doc 04 grants no mining-tier perk, so a tool below a node's `Req` never mines it. Examples: an iron vein (hardness 5) with a steel pick at Fieldcraft 30 in Mythic Fantasy takes `ceil_div(150 × 10^8, 13000 × 11500) = 101` ticks; with an iron pick at Fieldcraft 0, 150; the same in Space Opera (`hard_bp` 15000), 225; at Fieldcraft 75 with the steel pick, 84; oak timber with a tier-1 axe, 90, and with a pick, 180; clay by hand, 100.

### 9.3 Resource nodes

Nodes are full-shape prop blocks tagged `block.tag.node` and `block.tag.destructible`, with `hardness_milli = 1000 × Hardness`, `tool_tier = tooltier.t<Req>` and `blast_res_milli = NODE_BLAST_RES`, so explosions never break them: doc 03 §12.2 subtracts `floor_div((1000000 + 300) × 300, 1000) = 300,090` from a ray whose intensity never exceeds `mul_bp(8000, 13000) = 10,400`, so a node is never marked. Breaking one by the mining action (cause `grid.cause.mined`) or by weapon or spell damage (`grid.cause.broken`, doc 03 §13.2; doc 06 bolt removal) turns the cell into `block.node_spent` (unbreakable; doc 03 writes the record's `break_result`), so the terrain keeps its shape, and drops the node's yield to the breaking actor:

```
yield = base
      + 1   if actor's Fieldcraft >= 75 and the node is a mineral or salvage node   // doc 04 Fieldcraft 75
      + 1   if keyed rng.craft (actor, break tick, seq 0).roll_bp(1000 + 50 × fieldcraft)   // always 1 draw
yield = floor_div(yield × 15000, 10000)   when law.post_apocalypse.scarcity is active at the node (doc 12)
```

Mineral nodes are every row below except timber, sand, clay and bone; salvage nodes are `block.node.salvage`. Ore veins use hardness `max(4, metal hardness)` and `req_tier = max(1, tier - 1)`; soft piles (timber, sand, clay, salvage, bone) have `req_tier = 0`. Each node broken by an Erratum grants doc 04's Fieldcraft XP (500 milli-XP for an ore vein or salvage pile, 300 for any other destructible block, inside doc 04's 40,000 per-Folio cap). Doc 08 §8.2 lists the M-source ingredients a mineral node yields in addition to the units here. Example: base 2, Fieldcraft 30, bonus draw failed: 2 units; the same node under `law.post_apocalypse.scarcity`: `floor_div(2 × 15000, 10000) = 3`.

| Node | Yield | Base | Hardness | Req |
|---|---|---|---|---|
| `block.node.copper`, `.tin`, `.zinc`, `.silver`, `.gold` | the metal's `.ore` | 2 (gold 1) | 4 | t1 |
| `block.node.iron` | `mat.iron.ore` | 2 | 5 | t1 |
| `block.node.titanium` | `mat.titanium.ore` | 2 | 7 | t2 |
| `block.node.orichalcum`, `.star_metal`, `.aetherium` | the metal's `.ore` | 1 | 8, 9, 7 | t3 |
| `block.node.coal` | `mat.coal` | 3 | 4 | t1 |
| `block.node.obsidian`, `.coral` | the material | 2 | 5, 4 | t1 |
| `block.node.jade` | `mat.jade` | 1 | 6 | t2 |
| `block.node.psi_crystal` | `mat.psi_crystal` | 1 | 7 | t3 |
| `block.node.dreamstuff`, `.vellum` | the material | 1 | 4, 5 | t4 |
| `block.node.stone` | the genre stone | 3 | stone hardness | t1 |
| `block.node.timber` | the genre wood | 3 | wood hardness | t0 |
| `block.node.sand`, `.clay` | `mat.sand`, `mat.soil` | 2, 3 | 1 | t0 |
| `block.node.salvage` | the genre salvage pair | 3 | 3 | t0 |
| `block.node.bone` | `mat.bone` | 3 | 3 | t0 |

Genre stone / wood / salvage pair (first unit, second unit, third unit): MF granite / oak / —; GH slate / pine (blackwood from d 9) / —; CH basalt / pine / scrap_iron, scrap_iron, copper; SO basalt / bamboo / plasteel, polymer, polymer; CP concrete / bamboo / polymer, polymer, copper; PA concrete / pine / scrap_iron, scrap_iron, rubber; WW sandstone / pine / —; HN brick / oak / steel, glass, glass; WX granite / bamboo (cedar from d 4) / —; SP brick / oak / brass, brass, copper; FT granite / birch (thornwood from d 4) / —; MA marble / cedar / —; ON granite (dreamstuff from d 20) / pine (elderwood from d 9) / —; RD concrete / pine / polymer, paper, paper; UW paper (vellum from d 20) / paper / —; UT marble / oak / —. Every "from d N" gate is at or above that material's `min_d`, so a stone or timber node never yields a material before its band opens, and early Oneiric or Unwritten Folios cannot be farmed for tier-5 units. A salvage unit whose material has `min_d > d_eff` (Space Opera plasteel before d 9; Hardboiled Noir steel, Steampunk brass and Cyberpunk polymer before d 4) yields `mat.scrap_iron` instead.

### 9.4 Veins per genre

Doc 12 §11.2 owns how many mineral (M) resource nodes a Folio gets and which rooms hold them; this chapter owns what each one is. Every M node doc 12 places is one vein: its kind is one `weighted` draw over the zone genre's row below, in the listed order, after dropping metal, coal, crystal, obsidian, coral, dreamstuff and vellum kinds whose yield material has `min_d > d_eff` (stone, timber, sand, clay, salvage and bone kinds are never dropped, so every row keeps a positive total); then `range_incl(2, 4)` node blocks of that kind fill the socket's cells in ascending `Gidx`. Both draws come from doc 12's `rng.worldgen` placement sequence. This chapter asks doc 12 for at least `6 + floor_div(d_eff, 5)` veins per Folio (Canon gaps 13). Weights:

| Genre | Node weights |
|---|---|
| MF | iron 30, copper 15, tin 10, coal 15, silver 5, gold 5, stone 10, timber 10 |
| GH | iron 25, silver 15, coal 15, bone 20, stone 10, timber 15 |
| CH | iron 15, coal 10, obsidian 20, coral 20, star_metal 10, stone 15, salvage 10 |
| SO | titanium 25, salvage 30, psi_crystal 10, iron 15, stone 10, sand 10 |
| CP | salvage 40, copper 20, titanium 10, sand 10, stone 20 |
| PA | salvage 45, iron 15, copper 10, coal 10, sand 10, timber 10 |
| WW | iron 20, silver 15, gold 15, copper 10, coal 15, stone 15, timber 10 |
| HN | salvage 30, coal 15, iron 15, sand 10, clay 15, stone 15 |
| WX | copper 20, tin 15, jade 15, iron 15, stone 15, timber 20 |
| SP | coal 25, copper 15, zinc 15, iron 15, aetherium 10, clay 10, salvage 10 |
| FT | iron 20, gold 10, silver 10, sand 15, timber 25, stone 10, clay 10 |
| MA | copper 20, tin 15, orichalcum 10, gold 10, stone 25, clay 20 |
| ON | dreamstuff 40, stone 20, timber 20, gold 20 |
| RD | salvage 50, stone 30, sand 20 |
| UW | vellum 60, stone 40 |
| UT | vellum 30, gold 20, stone 20, iron 15, orichalcum 15 |

### 9.5 Harvesting

Plant blocks (doc 08 H and F sources) take a 30-tick interact (× 5000 at Survival 50) and yield `1 + floor_div(survival, 50)` units of their ingredient; the cell becomes air. No draws. Survival XP 500 per harvest (doc 04). Harvesting is an interaction, not a block break, so it needs no tool and no `tool_tier`.

### 9.6 Butchery

A `cat.corpse` other than Errata Remains is butchered in 240 ticks with Survival and destroyed. Flesh `min(12, floor_div(mass_g, 4000))` units, primary `min(8, floor_div(mass_g, 10000))`, secondary `min(4, floor_div(mass_g, 20000))`; each nonzero count gains 1 at Survival 25. Doc 04 §8.6 owns severed-part butchery and uses the same materials. A doc 13 template `materials` list overrides this table. A butchered material whose `min_d` exceeds `d_eff` (dragonbone before d 14, chitin before d 4) yields `mat.bone` instead, so early kills never hand out high-tier units:

| Body plan or tag | Primary | Secondary | Flesh |
|---|---|---|---|
| `body.humanoid` | `mat.bone` | `mat.hide` if tagged beast | yes |
| `body.quadruped`, `body.avian`, `body.serpentine` | `mat.hide` | `mat.bone` | yes |
| `body.insectoid` | `mat.chitin` | — | no |
| `body.construct` | first unit of the genre salvage pair, else `mat.iron` | `mat.copper` | no |
| `body.amorphous`, `body.swarm` | — | — | no |
| tag dragon | `mat.dragonbone` | `mat.hide` | yes |
| tag undead | `mat.bone` | — | no |

Butchering a sapient corpse costs composure 15 unless the butcher has `afflict.hunger` (doc 04 §11.3). Each output unit grants doc 04's 800 Survival milli-XP, at most 4,000 per corpse. Example: a 60,000 g quadruped at Survival 0 yields `min(12, 15) = 12` flesh, `min(8, 6) = 6` hide and `min(4, 3) = 3` bone, and grants 4,000 milli-XP (21 units, capped).

### 9.7 Planting and farming (`cat.seed`)

- **Plant**: a 30-tick action places `block.crop` (age 0) in the air cell above a full `tillable` block. The crop is Errata-placed (owner slot; Editors erase it), counts toward `CROP_CAP_PER_PLAYER` (not the placeable caps) and adds `crops[cell] = {seed, stage 0, next}`. A seed returned by a harvest is marked spent for the current Folio and cannot be planted again until the next `ev.folio_entered`, when the mark clears and its stack merges back.
- **Grow**: `stage_ticks = ceil_div(1200 × 10000, 10000 + 50 × survival + 3000 × [liquid water within 2 taxicab cells])`. While `light_at(cell) < 8`, `next` moves 1 tick later each tick. At `next` the stage advances and the props become age 2, 4, 7 for stages 1, 2, 3 (cause `grid.cause.grew`). Random block ticks never drive crops.
- **Harvest** at stage 3 (30 ticks): `3 + floor_div(survival, 34)` units of the seed's plant (doc 08 §8.4) and the seed back at Survival ≥ 25, else on a keyed `rng.craft` `roll_bp(5000)`; the cell becomes air. Harvesting earlier returns only the seed.

Example: Survival 20 beside water in light 12: `ceil_div(12000000, 14000) = 858` ticks per stage, 2,574 to maturity, 3 units at harvest.

**Balance.** Seeds never multiply (a harvest returns at most the one seed), so the limit is cycles. Without the cap and the spent mark, ten seeds beside water at Survival 20 would complete `floor_div(25200, 2574 + 60) = 9` plant-grow-harvest cycles in a 420-second Folio and yield `10 × 9 × 3 = 270` plants, several times the plant nodes doc 12 places; with them, a player harvests at most 4 crops of `3 + floor_div(survival, 34)` plants per Folio (12 units at Survival 0, 20 at Survival 100), comparable to one room of harvest nodes.

## 10. Blocks and building

### 10.1 Block catalog

Block tags: `block.tag.destructible`, `block.tag.errata_placeable`, `block.tag.node`, `block.tag.hearth`, `block.tag.altar`. A block without `destructible` has `hardness_milli = -1` (§2.5), so worldgen terrain never breaks.

| Block | Material | Shape | `hardness_milli` | Notes |
|---|---|---|---|---|
| `block.air`, `block.void_bedrock`, `block.reinforced` | air, void, granite | none, full, full | −1 | doc 03, doc 12 shells |
| `block.masonry.<m>` for every wall and floor material of doc 18 §3.4's block sets: granite, slate, basalt, sandstone, marble, brick, concrete, terracotta, coral, plasteel, titanium, polymer, scrap_iron, cast_iron, oak, pine, birch, cedar, bamboo, blackwood, thornwood, dreamstuff, paper, vellum (24) | that material | full | −1 | fixed genre walls and floors; never burn (§2.5) |
| `block.soil`, `block.sand` | soil, sand | full | −1 | soil is `tillable` |
| `block.cracked_wall` | granite | full | 2000 | destructible; hides secrets |
| `block.planks` | oak | full | 3000 | destructible wooden structures |
| `block.glass_pane` | glass | pane | 1000 | destructible |
| `block.door.wood`, `block.door.iron` | oak, iron | door | 3000, 6000 | destructible; lock and HP rules §10.7 |
| `block.light_fixture.<genre_suffix>` (16) | iron | fixture | 500 | `light_emit` = doc 12 `fixture_level` |
| `block.leaves` | leaves | cross | 0 | destructible, conceal 3 |
| `block.ice` | ice | full | 2000 | §10.2 |
| `block.web` | web | cross | 250 | §10.2 |
| `block.obsidian`, `block.basalt`, `block.slag` | obsidian, basalt, slag | full | 5000, 6000, 4000 | doc 08 reaction products; destructible |
| `block.node.*` (23), `block.node_spent` | §9.3, granite | full | §9.3, −1 | |
| `block.crop` | leaves | cross | 0 | §9.7 |
| `block.conjured` | granite | full | 2000 | doc 06 walls; renders as the local genre stone |
| `block.barricade.wood`, `.stone`, `.metal` | oak, granite, iron | full | 3000, 5000, 6000 | placeable |
| `block.barricade.gate` | oak | door | 3000 | placeable; opens only for `fac.errata` |
| `block.bridge.plank`, `.metal` | oak, iron | slab (bottom) | 2000, 4000 | placeable |
| `block.ladder.wood`, `.rope`, `.metal` | oak, linen, iron | ladder | 1000, 500, 3000 | placeable, climbable |
| `block.cover.sandbag`, `.plate` | sand, iron | full | 3000, 6000 | placeable deployable cover |
| `block.cover.barrier` | aetherium | pane, `gas_tight` 0 | 2000 | gadget-made; `light_filter` 0, conceal 2 |
| `block.trap.spikes` | iron | layer level 1 | 1500 | placeable trap |
| `block.torch`, `block.lantern` | oak, iron | fixture | 500, 2000 | light 14 (burns `TORCH_TICKS`), 15 |
| `block.campfire`, `block.campfire_out` | granite | slab (bottom) | 2000 | light 15 / 0; `hearth`; heat 1000 DeciC while lit; `station.campfire` |
| `block.shrine_lamp` | gold | fixture | 2000 | light 12; `altar` (doc 06 offering) |
| `block.torch_out`, `block.lantern_out`, `block.shrine_lamp_out`, `block.light_fixture_out.<genre_suffix>` (16) | as the lit block | as the lit block | as the lit block | unlit states, `light_emit` 0 (§10.6) |
| `block.crate` | oak | full | 2000 | destructible; doc 11 container that drops its contents when broken |
| `block.barrel` | oak | full | 2000 | destructible; `hide: enclosed` (doc 09 §10) |
| `block.powder_keg` | oak | full | 1000 | destructible; `flammability_bp` 6000, `burn_runs` 5 and the `explosive` record exactly as doc 03 §4.5 shows |
| `block.wardrobe` | oak | full | 3000 | destructible; `hide: enclosed`; skins closet, locker, sarcophagus, dumpster |
| `block.bed`, `block.table` | oak | slab (bottom) | 2000 | destructible; `hide: under` |
| `block.tall_grass` | leaves | cross | 0 | destructible; conceal 2 with `conceal_crouch_only` 1 (Canon §12.2) |
| `block.vine` | leaves | ladder | 0 | destructible; climbable (doc 13 climb link) |
| `block.light_switch` | iron | fixture | 500 | destructible; toggles its room's electric fixtures (doc 09 §2.4) |
| `block.fuse_box` | iron | fixture | 3000 | destructible; doc 09 power cut |

Every placeable and `block.conjured` carries `errata_placeable` and `destructible`. `hide` (`enclosed` or `under`) and `conceal_crouch_only` are block-record fields doc 09 reads; every block not listed with them has neither.

### 10.2 `block.ice` and `block.web`

- **Ice** forms from frost on liquids (doc 03 `timed_revert`, 1200 ticks), doc 06 frost walls and `gadget.mine_cryo`. Fields: `friction_bp` 1500 (Canon §12.1), `light_filter` 2, conceal 2, `melt_decic` 10 → 1000 units `liq.water`, ricochet. Mining it yields 1 `mat.ice`.
- **Web** forms from doc 06 bind walls, doc 13 spinners, `recipe.hand.web_trap` and worldgen dressing. It has no collision. An entity whose capsule overlaps a web cell holds `st.rooted` for as long as it overlaps (an Errata-placed web roots only entities outside `fac.errata`), without threshold scaling, ending 1 tick after it leaves (Canon §12.1); entities tagged `web_walker` are exempt. Each tick with movement input adds 1 struggle point; at `max(20, 120 - BRN)` points the web breaks (cause `grid.cause.broken`, drops 1 `mat.web`). Its 5,000 block HP (`250 × 20`) also breaks to one 10,000 mHP melee hit (`block_bp` 5000). Fire burns it out in 60 ticks (`burn_runs` 5 at fire period 12).

### 10.3 Placement rules

Doc 03 §13.3 checks apply first. Then:

1. Only `cat.placeable` items (template = block ID) place blocks (Canon §4.3); a held `item.tool.torch` never does (§6.6 `recipe.hand.wall_torch`). The placer is an Erratum in party slots 0 to 3 (the invader never places, Canon §17.9), not mounted (Canon §17.7), downed or a ghost, and not inside `room.colophon`.
2. The placer's live placed blocks stay under `PLACED_CAP_PER_PLAYER` and the Folio's under `PLACED_CAP_PER_FOLIO`; at a cap the placement is refused.
3. Channel `place_ticks = max(1, ceil_div(base × 10000, 10000 + 100 × fieldcraft))` with base: barricade and gate 60, bridge 40, ladder 20, cover 45, trap 30, torch 10, lantern 20, campfire 60, shrine lamp 120. It cancels like a craft channel (§6.4). Fieldcraft 100 makes every placement 1 tick.
4. Anchors: barricades, cover, traps and the campfire need a sturdy face below; torches, lanterns and shrine lamps attach to any sturdy face; ladders follow doc 03 attach; a rope ladder placed on a ledge top unrolls downward through up to 8 air cells. A bridge cell needs a horizontal neighbor that is a solid non-placed block (`span` 0) or a bridge, and takes `span = 1 + min(neighbor span)`; `span` above 5 is refused.
5. The cell gets the `placed_by_errata` bit (doc 13) and `placed_meta = {owner_slot, uid, hp, hp_max, expires, patron, span, trap_bp}`, where `trap_bp` is 13000 when the placer has Fieldcraft ≥ 50 (doc 04) and 10000 otherwise. Doc 04 grants 300 Fieldcraft milli-XP for the first placement of each placeable uid in a run. The owner can dismantle it in 30 ticks; at `hp >= mul_bp(hp_max, 5000)` the item returns with its original `uid`, so placing it again grants no second placement XP; otherwise nothing returns.

### 10.4 Placed block HP

```
hp_max = mul_bp(mul_bp(hardness_milli × 20, T(d_place)), if fieldcraft >= 25 { 12500 } else { 10000 })   // doc 04 Fieldcraft 25
```

Doc 03 §13.2 block damage to a placed cell reduces `placed_meta.hp` instead of the decaying `block_damage` map; at 0 the block breaks (cause `grid.cause.broken`) and drops nothing. Placed blocks scale with `T`, the enemy-damage table, because enemies are what break them; worldgen destructibles scale with `P` against Errata weapons (doc 03 §13.2). A wooden barricade has 60,000 at Folio 1, 108,120 at Folio 12 (135,150 with Fieldcraft 25) and 216,870 at Folio 25, so a reference enemy melee hit (`block_bp` 5000: `mul_bp(10000, 5000) = 5,000` at Folio 1, `mul_bp(36145, 5000) = 18,072` at Folio 25) needs 12 hits at Folio 1 and 13 at Folio 25 (12 hits leave 6 mHP). Repair: Fieldcraft, a `max(1, ceil_div(60 × 10000, 10000 + 100 × fieldcraft))`-tick channel, 1 unit of the block's material, `hp += mul_bp(mul_bp(hp_max, mul_bp(2500, 10000 + 100 × fieldcraft)), if fieldcraft >= 75 { 15000 } else { 10000 })` (doc 04 Fieldcraft 75), capped at `hp_max`; doc 04 grants 300 Fieldcraft milli-XP per 1000 Bp of `hp_max` restored.

Specifics: the gate toggles for `fac.errata` and is a locked door to everyone else; spikes hit each non-`fac.errata` entity entering the cell, at most once per 60 ticks, for `B = mul_bp(mul_bp(mul_bp(8000, P(d_place)), dmg_bp(mat.iron)), trap_bp)` `dmg.pierce` with `mul_bp(40, trap_bp)` `st.bleeding` power (item source, `src_lvl = d_place`; doc 04 Fieldcraft 50); torches burn out after `TORCH_TICKS` lit ticks and become air (cause `grid.cause.decayed`), and a snuffed torch keeps its remaining burn time until relit; a shrine lamp stores the patron chosen at placement, one per player. A worldgen mechanical trap (spike, blade or dart trap) disarmed by an Erratum with Security ≥ 50 (doc 09 §13.6) yields one `block.trap.spikes` placeable item at its `ilvl` = current Folio index; arcane wards, alarms, electronic tripwires and explosive traps yield nothing.

### 10.5 AI and Errata-built blocks

1. Placed solid blocks block navigation. Placed ladders give climb links to every agent (doc 13 §4.13); bridges are walkable; gates are locked doors for every non-Errata agent.
2. **Breach.** Errata barricade, cover and gate cells are passable links for ground agents with cost `1000 + floor_div(ceil_div(hp × 60, block_dps) × walk_mm_s, 60)`, `block_dps = floor_div(mul_bp(hit_mhp, 5000) × 60, attack_interval_ticks)`. An agent that reaches the link attacks the block (doc 13 `ai.action.breach`).
3. A brute charge that ends against a placed block deals `mul_bp(charge B, 20000)` block damage. Sapper charges break placed blocks within 2000 mm (doc 13). Burrowers dig them with `dig_ticks` from `break_ticks`.
4. Editors and hunters erase placed blocks (Canon §11.6, doc 13 §10.6) and path through them.
5. Hostile AI ignores Errata traps until a squad member sees one trigger; that squad then costs the cell × 3 (hazard) for the rest of the Folio.
6. Placed lights count for spawn light (Canon §12.3) and the undead and fiend light cost. Cover maps rebuild on `ev.block_changed`. AI never selects a block as a threat target.

### 10.6 Light sources, switches and props

Every light block has an unlit twin; snuffing, dousing, switching and relighting (doc 09 §2.4, doc 13 `ai.action.relight`) swap the pair through `set_block` with cause `grid.cause.toggled`. `lightsrc` is the doc 09 class.

| Lit block | `light_emit` | `lightsrc` | Unlit twin |
|---|---|---|---|
| `block.torch` | 14 | flame | `block.torch_out` |
| `block.lantern` | 15 | flame | `block.lantern_out` |
| `block.campfire` | 15 | flame | `block.campfire_out` (also when its fuel runs out, §5.3) |
| `block.shrine_lamp` | 12 | arcane | `block.shrine_lamp_out` |
| `block.light_fixture.<genre_suffix>` | doc 12 `fixture_level` | arcane in Oneiric, the Unwritten and the Ur-Text; electric where the genre's TECHNE ≥ 5 (Cosmic Horror, Space Opera, Cyberpunk, Post-Apocalypse, Hardboiled Noir, Steampunk, the Redaction); flame elsewhere | `block.light_fixture_out.<genre_suffix>` |

Electric fixtures belong to their room's circuit; `block.light_switch` and `block.fuse_box` act on that circuit as doc 09 §2.4 rows 4 and 6 say. Hiding props (`block.barrel`, `block.wardrobe` with `hide: enclosed`; `block.bed`, `block.table` with `hide: under`) hold the occupants doc 09 §10 allows; breaking one ejects them into the cell.

### 10.7 Doors

Worldgen doors (`block.door.wood`, `block.door.iron`, skinned per genre) and the placeable `block.barricade.gate` are the door blocks. Doc 12 §9.2 places worldgen doors and doc 09 §13.1 attaches locks (`lock.t1` to `lock.t6`).

| State | Event | Next |
|---|---|---|
| Closed | interact by an entity allowed to open it (Errata, ghosts per Canon §17.6, AI per doc 13 §4.13; the gate only for `fac.errata`) | Open (doc 03 §13.4 toggle, cause `grid.cause.toggled`) |
| Open | interact | Closed; refused while a body overlaps the closing boxes |
| Locked | interact without the key | Locked; refused |
| Locked | key holder interacts, or doc 09 pick, hack or Security 100 succeeds | Closed with the lock removed |
| Closed, Open or Locked | accumulated block damage reaches `block_hp(cell)` | Broken: the cell becomes air (cause `grid.cause.broken`), drops nothing, and its lock is gone |
| any | the cell carries doc 12 `protect` (exit doors, sealed arenas) | unchanged: protected doors take no damage |

```
door_lock_bp(cell) = 10000 + 5000 × lock_tier(cell)                     // lock_tier 0 without a doc 09 lock, else 1..=6
block_hp(cell)     = mul_bp(mul_bp(block_hp_mhp, door_lock_bp(cell)), P(d_current))   // every block without placed_meta; non-doors use 10000
break_ticks(cell)  = mul_bp(30 × floor_div(hardness_milli, 1000), door_lock_bp(cell))   // doc 13 breaker link cost
```

Examples: an unlocked wooden door at Folio 1 has `block_hp` 60,000 mHP: 10 swings of a 12,000 mHP axe (`block_bp` 5000), as doc 03 §13.2 states, or 25 swings of a long blade (`block_bp` 2000). An iron `lock.t3` door at Folio 11 has `mul_bp(mul_bp(120000, 25000), 23673) = 710,190` mHP: 20 swings of a Common Normal +0 iron mace (`mul_bp(15600, 23673) = 36,929` mHP, `block_bp` 10000) at Fieldcraft 0, and `break_ticks` 450 for doc 13's breaker cost. Breaking a lock is always possible and always slower and louder than picking it at the Security doc 09 §13.2 expects for that depth.

## 11. Gadgets and deployables

### 11.1 Rules (Hacking-free)

1. **No Hacking.** No gadget action requires or reads `skill.hacking`. Tinkering sets crafting quality, repair and the Tinkering perks below; Fieldcraft sets the deploy channel of ground gadgets (mines, snares, demolition charges, noisemakers, barrier projector) through doc 04's `max(1, ceil_div(base × 10000, 10000 + 100 × fieldcraft))`; every other gadget deploys in `max(1, ceil_div(base × 10000, row 17))` ticks (Canon §6.4 row 17).
2. **Not hackable.** Errata gadgets are immune to `st.hacked` and every mind status, and doc 09's hack verbs never target them. Hostile disabling effects (doc 13 `sabot`, `short_circuit`) set `disabled_until` and never change faction. A hostile turret or drone that an Erratum seizes with doc 09's hack gains `st.hacked` and fights for the hacker's faction for the status's duration, as Canon §11.1 says; it stays a doc 13 device, never becomes an Errata gadget, never uses these rules and never counts toward rule 8.
3. **Dependency** comes from the genre skin (Canon §5.3). Reusable gadgets take `techne` in CH, SO, CP, PA, HN, SP, RD; `arcana` in MF, FT, ON; `mythos` in GH, MA, WW; none in WX, UW, UT, where the skins are mundane mechanisms (repeating-crossbow turrets, kites, clockwork). Consumable gadgets are mundane devices (spring traps, black-powder and frost-salt mines, noisemakers) with dependency none everywhere, like the throwables of Canon §17.3a. A dependency-none gadget never Contradicts and never earns Anachronism Edge, the same trade a bow makes; that keeps the Wuxia skin from dominating.
4. **Contradiction** is evaluated at the deployer. One keyed `rng.combat` failure roll per deploy (Canon §5.5 "use action") when the failure chance is above 0. A failure is a **spark**: one hit on the user with `D2 = mul_bp(max_hp_mhp, SPARK_BP)`, 100% `dmg.shock`, entering Canon §10.5 at step 3 like the §10.6 reflected row (no crit, no buildup, no friendly-fire step, flag `no_chain`); it then calls the shared `note_contradiction_failure(t)` that counts toward `st.unmoored` (Canon §5.11; doc 05 §16.1). The spark consumes a consumable and spends a reusable gadget's charge. Example: Max HP 85 gives a 4,250 mHP shock spark before resistance. Potency multiplies damage, healing, duration and status power, never HP.
5. **Entities.** Turrets, drones, beacons, coils and decoys are `fac.errata` entities with `owner_slot`, counted in `ACTIVE_ACTOR_CAP` but not as minions; mines, snares, charges and noisemakers are props. Gadgets never target `fac.errata`. Direct gadget hits (turret, drone and coil shots) that strike an Erratum use `FF_DIRECT_BP`; gadget explosions (mines, demolition charges, the `self_destruct` blast) use `FF_AREA_BP` (Canon §10.5 step 2.15, §17.5). Kills credit the owner; gadget hits grant Tinkering XP (doc 04).
6. **Magnitudes.** `src_lvl` = the gadget's `ilvl` (Canon §13.8.1). `B = base_mhp → P(src_lvl) → rarity power → quality → (10000 + 600 × eff_enh) → dmg_bp(chassis)` (Canon §10.4; a crafted gadget is Common +0, so the third and fifth steps are 10000); `hp_max = base × 1000 → T(src_lvl) → quality → 13000 per plating module → 12500 for a turret whose owner has Tinkering ≥ 50 → 12500 with hardlight` (`part.module.plating`; doc 04 Tinkering 50 and `hardlight`); `AR = base → T(src_lvl) → ar_bp(chassis)`. Each arrow is one `mul_bp`. The chassis is the primary F material; a found gadget uses `mat.iron`.
7. **Recharge.** Per tick `progress += mul_bp(mul_bp(10000 + (TEC - 5) × 1000, 10000 + 200 × (tech_affinity - 5)), talent_bp)`, `TEC` at the owner's position (Canon §5.7, applied to every gadget whatever its dependency), `talent_bp` = 15000 at Tinkering 100 (doc 04) then × 12000 with `overclock`; a charge is restored when `progress >= target`, where `target = recharge × 10000` (× 15000 after destruction, × 5000 after a recall, §11.3).
8. **Limits per player.** Turrets 1 (+1 `turret_tech`, +2 `legion`), drones 1 (+2 `legion`), beacons 1, coils 1, barrier 1, decoys 2, noisemakers 2, mines and snares 6 together, charges 3. A deploy at a limit is refused.
9. **Charges.** A reusable gadget holds `charges.max = 1`, or 2 while its owner has Tinkering ≥ 25 (doc 04 "gadgets +1 charge"); a deploy spends one charge and the item is Ready while it holds a charge and no deployment of it is Active. A consumable gadget recipe completed by a crafter with Tinkering ≥ 25 yields one more unit (`×2` becomes 3, a single output becomes 2).
10. **Fieldcraft and talents.** Mines, snares and demolition charges deployed by an Erratum with Fieldcraft ≥ 50 deal damage and status power × 13000, stored at deploy (doc 04 Fieldcraft 50); a demolition charge deployed at Fieldcraft 100 has radius and block-breaking radius +1000 mm (doc 04 Fieldcraft 100). With `drone_companion` (doc 04), each `ev.folio_entered` spawns a `gadget.drone_scout` Active for its full duration at the owner (`src_lvl = d_current`, `q.normal`, chassis `mat.iron`, no item spent, counted toward the drone limit, gone when it ends). `self_destruct` and `legion` act exactly as doc 04 states.

### 11.2 Catalog

Recipes are workbench assembly recipes `recipe.bench.<gadget suffix>` with Tinkering (demolition: Fieldcraft); `×2` yields two consumables.

| Gadget | Kind | Deploy | HP / AR base | Lasts | Effect | Recharge | Recipe: tier, skill, inputs | Pool |
|---|---|---|---|---|---|---|---|---|
| `gadget.turret_light` | reusable | 60 | 40 / 10 | 1800 | nearest visible hostile within 20000 mm; base 4 HP pierce every 30 ticks, `rifle_auto` ballistics, spread 182 Bam, precision 15000 | 1800 | 1, 20, F2 + 1 G | base |
| `gadget.drone_scout` | reusable | 30 | 20 / 0 | 3600 | hovers 2500 mm above the owner; direct `st.marked` on hostiles in sight within 12000 mm every 60 ticks | 2400 | 1, 20, F1 + 1 G + 1 copper | base |
| `gadget.mine_frag` | consumable ×2 | 45 | — | arms 60 | hostile within 1500 mm: blast radius 3000, base 30 HP pierce, `block_power_milli` 0 | — | 1, 15, F1 + 1 coal | base |
| `gadget.trap_snare` | consumable ×2 | 30 | — | until sprung | hostile within 600 mm: direct `st.rooted` | — | 1, 10, M1 + 1 S | base |
| `gadget.beacon_mend` | reusable | 60 | 30 / 5 | 600 | allies within 4000 mm heal 100 Bp of Max HP per second | 2400 | 2, 30, F1 + 1 G + 1 `arcane_reagent` | base |
| `gadget.decoy` | consumable | 30 | 60 / 10 | 300 | direct `st.taunted` toward the decoy on hostiles within 8000 mm at deploy | — | 1, 10, 1 S + 1 W | base |
| `gadget.turret_flame` | reusable | 90 | 60 / 15 | 1200 | cone 5461 Bam, 5000 mm; every 10 ticks base 2 HP fire and 30 `st.burning` power per target | 2400 | 2, 35, F2 + 2 coal | gadgets_2 |
| `gadget.drone_combat` | reusable | 30 | 30 / 5 | 2400 | follows the owner; fires at the owner's last target within 15000 mm, base 3 HP pierce every 20 ticks | 3000 | 2, 40, F2 + 1 G + 1 copper | gadgets_2 |
| `gadget.mine_cryo` | consumable ×2 | 45 | — | arms 60 | hostile within 1500 mm: blast radius 3000, base 15 HP frost, 80 `st.frozen` power; freezes water | — | 2, 30, F1 + 1 ice | gadgets_2 |
| `gadget.barrier_projector` | reusable | 30 | — | 900 | 3 × 2 cells of `block.cover.barrier` facing the placer's view, each `hp_max = mul_bp(mul_bp(20000, T(src_lvl)), quality)` | 1800 | 2, 35, F1 + 2 G | gadgets_2 |
| `gadget.tesla_coil` | reusable | 60 | 50 / 10 | 1200 | every 60 ticks hits up to 3 hostiles within 5000 mm (nearest first) for base 6 HP shock, 30 `st.shocked` power, flag `no_chain` | 2400 | 3, 50, F1 + 2 copper | gadgets_2 |
| `gadget.demolition_charge` | consumable | 60 | — | fuse 120 | doc 03 explosion: radius 2500, `block_power_milli` 6000, base 40 HP blunt 5000 fire 5000, `drop_bp` 5000; breaks only destructible blocks | — | 1, Fieldcraft 25, M1 + 2 coal + 1 S | gadgets_2 |
| `gadget.noisemaker` | consumable ×2 | 30 | — | 600 | emits `noise.gadget.noisemaker` (70 dB, doc 09 §7.6) at its cell every 60 ticks; AI investigates it (doc 09 §12) | — | 1, 10, F1 + 1 copper | craft |

The first six match doc 11 §3.6 loot stats; the next six enter the loot pool with `unlock.loot.gadgets_2` (doc 14); `gadget.noisemaker` is crafted only and appears in no loot pool. Statuses use Canon §11.4 durations and `S(src_lvl)` power. Explosions enter doc 03 §12 as item sources.

```json
{ "id": "gadget.turret_light", "category": "cat.gadget", "reusable": 1, "deploy_ticks": 60,
  "hp_base": 40, "ar_base": 10, "duration_ticks": 1800, "recharge_ticks": 1800, "range_mm": 20000,
  "shot": { "base_mhp": 4000, "shares": { "dmg.pierce": 10000 }, "interval_ticks": 30,
            "ballistics": "arch.ranged.rifle_auto", "spread_bam": 182, "precision_bp": 15000, "impact": 3 },
  "limit_key": "turret", "dur_base": 4000 }
```

### 11.3 Deployment state machine

Per reusable gadget item: `charges` (0..=`charges.max`), `progress`, `target`. Per deployment: the spawned entity or prop. Consumables use only the rows marked C.

| State | Event | Next |
|---|---|---|
| Ready (charges ≥ 1 and no deployment of this item Active) | deploy input; rule 8 limit free; durability > 0; `disabled_until` passed | Deploying (rule 1 channel) |
| Deploying (C) | channel cancelled (§6.4 cancel rules) | Ready; nothing spent |
| Deploying (C) | channel done, failure roll succeeds | Spark (rule 4): a consumable is consumed; a reusable spends 1 charge → Ready if `charges ≥ 1`, else Recharging |
| Deploying (C) | channel done, no failure roll or roll fails | Active: entity or prop spawned (`ev.entity_spawned`, `noise.gadget.deploy`); a consumable is consumed, a reusable spends 1 charge |
| Active (C) | duration ends | a consumable ends; a reusable takes wear 500, `target = recharge × 10000` → Ready if `charges ≥ 1`, else Recharging |
| Active | HP reaches 0 | wear 2500, `target = recharge × 15000`, `self_destruct` blast if owned (doc 04) → Ready if `charges ≥ 1`, else Recharging |
| Active | owner recall (30-tick interact within 3000 mm) | wear 500, `target = recharge × 5000` → Ready if `charges ≥ 1`, else Recharging |
| Recharging (charges = 0) | `progress >= target` | `charges = 1`, `progress = 0` → Ready |
| Ready with `charges < charges.max` | `progress >= target` (progress accrues only while no deployment is Active) | `charges += 1`, `progress = 0` |
| any | durability reaches 0 | Broken until repaired (§8.4); `charges` and `progress` are kept |

A Folio exit ends every deployment and keeps each item's `charges` and `progress`; props and gadget entities never cross Folios.

### 11.4 Turret and drone think

```rust
fn gadget_tick(g: &mut Gadget, w: &World, t: Tick) {           // sys.crafting, every tick, ascending EntityId
    if g.cooldown > 0 { g.cooldown -= 1; }
    if g.disabled_until > t || g.expired(t) { return; }
    if (t + g.phase) % 6 == 0 {                                 // retarget at the Canon §2.2 AI-think phase
        g.target = w.hostiles_within(g.pos, g.range_mm)         // ascending EntityId
            .filter(|h| w.line_of_sight(g.muzzle(), h.center()))
            .min_by_key(|h| (!h.has(ST_MARKED), dist2(g.pos, h.pos), h.id))
            .map(|h| h.id);
    }
    if g.cooldown == 0 {
        if let Some(h) = g.target.filter(|&h| w.alive(h) && w.line_of_sight(g.muzzle(), w.center(h))) {
            w.fire_projectile(g, h, g.shot, keyed_rng("rng.combat", folio, g.id, t, 0));   // 2 spread draws
            g.cooldown = g.shot.interval_ticks;
        }
    }
}
```

Firing is checked every tick, so the stated intervals hold exactly (a 20-tick drone fires every 20 ticks, not every 24). `g.phase = (g.id & 0xFF_FFFF_FFFF) % 6` (Canon §1.11). The flame turret's cone pulse and the coil's arcs hit instantly with no projectile and no spread draws; each target hit draws one crit roll from that tick's generator, targets in ascending EntityId. The combat drone targets the owner's last target instead of running the search; the scout drone and the beacon never fire.

Each hit draws one crit roll at `clamp(500 + (vrv - 5) × 100, 0, 7500)` Bp (0 where VERVE is 0) with multiplier 15000. Example: a Good turret with a brass chassis made in Folio 5 fires `4000 → 5646 → 5646 → 6210 → 6210 → 5589` mHP shots and has `hp_max = mul_bp(mul_bp(40000, 12388), 11000) = 54,507` mHP (68,133 at Tinkering 50).

## 12. Cooking

### 12.1 Rules

Cooking recipes are `recipe.kind.process` jobs (§5.3 queue, fueled) at a campfire (recipes listed for the campfire) or a kitchen (any cooking recipe), governed by Survival; doc 04 Survival 50 multiplies their base ticks by 5000 before the §5.3 `ticks_each` formula. Outputs are doc 11 `item.food.meal` instances (500 g, 900 satiation) carrying `dish` = the recipe ID, or `item.food.ration` (300 g, 600 satiation, never rots); doc 04 scales satiation by the eater's `10000 + 20 × survival`. Satiation, freshness and the meal's composure gain follow doc 04 §11.2. Eating a dish with a buff sets `meal_buff = (dish, t + MEAL_BUFF_TICKS)`, replacing any previous buff; the buff applies doc 04 §1.2 modifiers and ends at expiry or on `st.ghost`. Survival XP is `1000 × input_units` per cook (doc 04).

### 12.2 Recipes and buffs

| ID | Station | Inputs → output | Ticks | Buff while active | Pool |
|---|---|---|---|---|---|
| `recipe.cook.roast` | campfire | 1 flesh → 1 meal | 120 | none | base |
| `recipe.cook.hearty_stew` | campfire | 2 flesh + 1 plant `ing.` + 250 ml `liq.water` → 2 meals | 300 | `RowPost(5, 11500)` | base |
| `recipe.cook.spiced_skewers` | campfire | 1 flesh + 1 `ing.firebloom` → 1 meal | 180 | `dmg.frost` resist +1000; `DmgAdd(melee, 300)` | base |
| `recipe.cook.strong_coffee` | campfire | 2 `ing.coffee_bean` + 250 ml water → 2 meals | 120 | `RowPost(17, 11000)` | base |
| `recipe.cook.mint_tea` | campfire | 2 `ing.mint_leaf` + 250 ml water → 2 meals | 120 | composure +15 on eating; `RowPost(6, 8500)` | base |
| `recipe.cook.mushroom_soup` | kitchen 1 | 2 `ing.glowshroom` + 250 ml water → 1 meal | 240 | `RowFlat(31, 2)` | base |
| `recipe.cook.herb_bread` | kitchen 1 | 2 `ing.sungrass` + 1 `ing.earthroot` → 2 meals | 360 | `RowPost(2, 12000)` | base |
| `recipe.cook.smoked_rations` | kitchen 1 | 3 flesh + 1 coal → 2 `item.food.ration` | 600 | none; never rots | base |
| `recipe.cook.ginseng_broth` | kitchen 2 | 1 `ing.ginseng` + 1 flesh + 250 ml water → 1 meal | 300 | `RowFlat(4, 10)` | base |
| `recipe.cook.troll_pie` | kitchen 2 | 1 `ing.troll_fat` + 2 flesh → 2 meals | 480 | `RowFlat(18, 8)` | base |
| `recipe.cook.wanderers_feast` | kitchen 3 | 2 flesh + 1 `ing.starflower` + 1 `ing.ginseng` + 250 ml water → 4 meals | 600 | `RowPost(5, 11500)` and `RowFlat(13, 300)` | r2, L |

## 13. Fieldcraft hooks (the canon-1.2 Delving hooks)

Canon-1.2 gave this chapter hooks for a Delving skill; canon-1.4 replaced Delving with `skill.fieldcraft` (governed by END, Canon §7) and doc 04 §5.3 owns its perks. Every hook this chapter exposes, and the perk or XP row that drives it:

| Fieldcraft | Effect in this chapter | Where |
|---|---|---|
| every point | placing, repairing and arming placeables take `max(1, ceil_div(base × 10000, 10000 + 100 × skill))` ticks | §10.3, §10.4, §11.1 rule 1 |
| every point | mining, and melee or tool damage to destructible blocks, × `10000 + 50 × skill` | §9.2; doc 03 §13.2 |
| 25 | placeables you place have `hp_max` × 12500 | §10.4 |
| 50 | traps (`block.trap.spikes`, mines, snares) and demolition charges you set deal damage and status power × 13000 | §10.4 `trap_bp`, §11.1 rule 10 |
| 75 | mineral and salvage nodes you break yield +1 unit; your placeable repairs restore × 15000 | §9.3, §10.4 |
| 100 | placing a placeable takes 1 tick; your demolition charges gain +1000 mm radius | §10.3, §11.1 rule 10 |

Fieldcraft XP (doc 04 §5.2): 300 per placeable uid placed (once per run), 300 per 1000 Bp of placeable `hp_max` repaired, 300 per destructible block broken, 500 per ore vein or salvage pile harvested, and trap or charge hits as doc 04 lists; at most 40,000 milli-XP per Folio together. Dismantling or breaking your own placeable grants 0. Example: Fieldcraft 30 places a 60-tick barricade in `ceil_div(600000, 13000) = 47` ticks and mines an iron vein with a steel pick in 101 ticks (§9.2).

## 14. Worked examples

Run seed `0x0123456789ABCDEF` throughout.

### 14.1 Craft a steel longsword

Folio 7, Mythic Fantasy, forge tier 2 (`1 + floor_div(6, 5)`). The crafter (EntityId `(7 << 40) | 3`) has Smithing 40, BRN 48, an iron toolkit (t1), no helpers.

1. **Steel.** `recipe.alloy.steel` queues 2 iron + 1 coal: `ticks_each = ceil_div(600 × 10000, 10000 + 50 × 40 + 0) = 500`, giving 2 `mat.steel`.
2. **Match.** The grid `B/B/H` with steel, steel, oak matches `recipe.forge.blade_long`. Forge tier 2 ≥ steel tier 2, so the material term is `15 × min(2, 0) = 0`.
3. **Channel.** `needed = ceil_div(360 × 10000, 10000 + 2000 + 1000) = 277` ticks.
4. **Quality.** `center = 200 + 100 + 30 + 0 + 0 + 0 + 8 = 338`. Keyed `rng.craft` (Folio 7, the crafter, start tick 50000, seq 0) gives `range(141)` = 116, 34, 32: noise −28, score 310, so `q.good` (cap Excellent). The fourth draw seeds two perk columns.
5. **Damage.** `14700 → × P(7) 16771 = 24653 → × 10000 → × 11000 = 27118 → × 10000 → × steel 11000 = 29829` mHP per light hit at `move_bp` 10000.
6. **Mass and timing** (doc 05 §3.3). `floor_div(170 × 7850, 1000) + floor_div(84 × 750, 1000) = 1334 + 63 = 1397` g; `mass_ratio_bp = floor_div(13,970,000, 1400) = 9978`; `time_mass_bp = 10000 + floor_div(−22 × 3000, 10000) = 9993` and the floor term `ceil_div(110,000,000, 14000) = 7858` is lower, so `time_bp = 9993`; light W/R stay 10/12 and `charge_full` 40; `stamina_bp = impact_bp = 10000 + floor_div(−22 × 5000, 10000) = 9989`, so light stamina is `mul_bp(3000, 9989) = 2996` mst and impact `mul_bp(12000, 9989) = 11,986` mp.
7. **Other.** Durability `10000 → 11350 → 12485`; bleed multiplier 10800; enchant capacity `floor_div(4 × 6, 4) = 6`; value `76 → 127 → 127 → 139 → 139 → × 13000 = 180`; Smithing XP `1000 × 3 + 2000 × 1 = 5000` milli-XP before row 28.

### 14.2 Assemble a pistol from parts

Folio 9, Weird West, workbench tier 2. The crafter (`(9 << 40) | 5`) has Tinkering 45, LOG 52, a steel toolkit (t2) and one helper with Tinkering 30; the job starts on tick 61000. Components: 1 `mat.iron` frame (118 ml), 1 `mat.oak` grip (97 ml). Parts, all `q.normal`: `part.barrel.rifled` and `part.feed.quick` stamped Coffin Creek (both on its doc 11 §5.2 part list), `part.optic.reflex` stamped Ghostline, and a crafted `part.stock.braced` (Marginal Press).

1. **Channel.** `ceil_div(360 × 10000, 10000 + 2250 + 1000 + 1500) = 245` ticks.
2. **Quality.** `center = 200 + 112 + 30 + 10 + 15 + 7 + 12 = 386`; keyed `rng.craft` (Folio 9, the crafter, tick 61000, seq 0) gives `range(141)` = 74, 59, 93: noise +16, score 402, `q.excellent` (cap Excellent from skill 45 and tier 2). The fourth draw seeds two perk columns.
3. **Manufacturer.** Coffin Creek holds 2 non-graft parts and makes class RG, so the pistol is `mfr.weird_west.coffin_creek`: tendencies D+500, M−2000, L+1000 and the crit-ricochet gimmick (dependency verve).
4. **Stats** (§7.3 order: manufacturer tendencies, material tendencies, then barrel, feed, optic, stock). `base_mhp 8600 → 9030`. Magazine `round_half_even(12 × 8000, 10000) = 10`. Reload `72 → 79` (tendency) `→ 67` (quick feed, same manufacturer, full −1500). Range 60000 and falloff 15000 / 35000 are unchanged. Spread takes the rifled barrel's −1500 in full, then the reflex optic's −800, which is cross-manufacturer and becomes `−mul_bp(800, 7500) = −600`: hip 364 → 309 → 290, ADS spread 73 → 62 → 58, bloom 64 → 54 → 50, bloom max 364 → 309 → 290. The braced stock gives recoil 182 / 36 → 154 / 30, swap `round_half_even(18 × 10500, 10000) = 19` and ADS `round_half_even(9 × 10500, 10000) = 9`. Iron tech affinity 5 adds no reload tendency.
5. **Mass.** `floor_div(118 × 7870, 1000) + floor_div(97 × 750, 1000) = 928 + 72 = 1000` g, exactly doc 05's `ref_mass_g`, so doc 05 §3.3 gives `mass_ratio_bp` 10000 and every value in step 4 stands.
6. **Chain.** `9030 → × P(9) 19925 = 17992 → 17992 → × 12000 = 21590 → 21590 → × iron 10000 = 21590` mHP per shot. Durability `10000 → 10000 → 12000`.

### 14.3 Mine and smelt iron

Folio 3, Mythic Fantasy. The Erratum (`(3 << 40) | 2`) has a steel pick (t2), Fieldcraft 30, Smithing 20.

1. **Mine.** An iron vein (hardness 5, Req t1): `ceil_div(150 × 10^8, 13000 × 11500) = 101` ticks. At break tick 12000 the keyed bonus roll (Folio 3, the miner, tick 12000, seq 0) draws 9498 against `1000 + 50 × 30 = 2500`: no bonus. Fieldcraft 30 is below the 75 perk. Yield 2 `mat.iron.ore`; the cell becomes `block.node_spent`; pick wear `10 × 5 = 50`; Fieldcraft XP 500 milli-XP (doc 04).
2. **Smelt.** Forge tier 1 (melt cap 16000 ≥ 15380). `ticks_each = ceil_div(240 × 10000, 10000 + 50 × 20) = 219`; the batch of 2 takes 438 ticks and burns 1 coal (1200 fuel ticks, 762 left in `fuel_ticks`). Output 2 `mat.iron`; Smithing XP `2 × 1000 = 2000` milli-XP before row 28.

## Exports

| Namespace | IDs |
|---|---|
| `mat.` | the 69 materials of §2.2; `mat.<metal>.ore` for copper, tin, zinc, iron, silver, gold, titanium, orichalcum, star_metal, aetherium; `mat.role.{blade, head, haft, limb, frame, plate, soft, focus, block}`; `mat.tag.{absorbent, acid_soluble, alloying, arcane_reagent, brittle, edible, fabric, fae_bane, fat, flammable, fuel, luminous, metal, nonsolid, ricochet, silvered, stone, terrain, tillable, unbreakable, wood}` |
| `tooltier.` | `t0` … `t5` |
| `station.` | `hand`, `campfire`, `workbench`, `forge`, `loom`, `kitchen`, `alchemy`, `enchanting`, `scriptorium` |
| `recipe.` | `recipe.kind.{shaped, shapeless, assembly, process, disassembly}`; the 127 recipes of §6.6 (process, ammunition, shaped and shapeless, assembly), §7.2, §8.5, §11.2 and §12.2; the 12 `r2` recipes form `loot.pool.recipes_2` |
| `part.` | the 29 parts of §7.2; `part.graft.<genre_suffix>.<name>` for the 38 manufacturers of doc 11 §5.2 other than First Press and Marginal Press |
| `block.` | every block of §10.1: `block.masonry.<m>` (24), `block.light_fixture.<genre_suffix>` and `block.light_fixture_out.<genre_suffix>` (16 each), 23 `block.node.*`, `block.node_spent`, the placeables, their unlit twins, the props (`crate`, `barrel`, `powder_keg`, `wardrobe`, `bed`, `table`), `tall_grass`, `vine`, `light_switch`, `fuse_box`; `block.tag.{destructible, errata_placeable, node, hearth, altar}` |
| `gadget.` | the 13 gadgets of §11.2: six base, six in `loot.pool.gadgets_2`, and the crafted-only `gadget.noisemaker` |
| Record fields | material `acoustic`; block `hide`, `conceal_crouch_only`, `lightsrc` (§10.6); instance fields `crafted`, `mat_bonus_bp`, `dish`, ammunition `smart`; gadget state `charges`, `progress`, `target` |

Functions exported to other docs: `pick_material`, `equivalent`, `break_ticks(material)`, `break_ticks(cell)`, `door_lock_bp(cell)`, `mining_ticks`, `block_hp(cell)`, `block_drops(state, cause, actor)`, `on_block_changed`, `unit_mass_g`, `salvage_units`, `dur_max`, `wear`.

## Interface assumptions

| Doc | Assumed to provide |
|---|---|
| 02 | content validation of the §2.1 and §6.2 schemas, including both `recipe_overlap` rules and the §2.1 validation list |
| 03 | `block.tag.*` tags on block records (§4.5); a `break_result` block field, default `block.air`, that its mining and damage breaks write (resource nodes set `block.node_spent`); `sys.crafting` in the tick order (§1.4); saved `placed_meta`, `stations` and `crops` maps cleared when another cause changes the cell; calls to `doc07::mining_ticks`, `doc07::block_hp` for every cell without `placed_meta` (doors included) and `doc07::block_drops`; placed-cell damage routed to `placed_meta.hp`; `hide`, `conceal_crouch_only` and `lightsrc` carried in the compiled block tables; its §13.3 placement accepting only `cat.placeable` items (its mention of `item.tool.torch` reads as the `block.torch` item); gadget entity kinds with the §11.3 fields |
| 04 | the Smithing (25, 50, 75, 100), Tinkering (25, 50, 75, 100), Fieldcraft (every tier) and Survival (25, 50) perks as quoted in §3, §4, §8, §9, §10, §11 and §13; Artificer talents `overclock`, `field_repair`, `scrap_savant`, `turret_tech`, `modular`, `drone_companion`, `reinforced`, `hardlight`, `masterwork`, `self_destruct`, `smart_rounds`, `legion`; `self_repair` spending `REPAIR_KIT_USE`; the XP rows quoted here (crafting, repair, salvage, Fieldcraft, Survival, a Tinkering row of `min(750, floor_div(750 × dmg, max))` per gadget hit); food values; meal-buff modifiers; augment installation at `station.workbench` |
| 05 | `ref_mass_g` equal to §3.1's reference masses (staff 1530, tome 900); the §3.2 bleed multiplier on `status_bp`; `pen_cost` (negative means stop) and `ricochet` read from materials; turret projectiles using `rifle_auto` ballistics; a smart and a plain reserve counter per ammunition family, smart fired first with `DmgAdd +1000`; the shared `note_contradiction_failure(t)` |
| 06 | `arcane_affinity`, `mat.tag.arcane_reagent`, `station.enchanting`, `block.conjured`, web and ice walls, the shrine lamp as an altar; bolt removal of nodes drops as §9.3 |
| 08 | `ing.seed_*` as `cat.seed`, plant tags, `liq.water` withdrawal from containers, M-source ingredients on mineral nodes, `ing.charcoal` from `recipe.kiln.alch_charcoal`, `apparatus.vessel.crucible` and `liq.molten_metal` for pour mode, the §9.1 apparatus kinds and alchemy-station skins |
| 09 | reading `acoustic`, `lightsrc`, the unlit twins, `hide` and `conceal_crouch_only`; the `suppressor` tag on `part.barrel.suppressor`; `noise.gadget.noisemaker` for `gadget.noisemaker`; lockpick snaps (durability 1 per pick) and hacking-deck wear 100 per attempt; `TOOL_Q` from crafted quality; Security-50 disarm yielding `block.trap.spikes` (§10.4); Seize applying `st.hacked` to hostile devices only |
| 11 | mirroring §7.2's 22 loot parts and the graft loot; `item.book.schematic` with `teaches`; instance fields `crafted`, `mat_bonus_bp`, `dish` and part entries `{slot, part_id, mfr, quality}`; reusable gadgets with `stack_max` 1 and the §11.1 charges; doc 11 §3.6's "gadgets have dependency techne" read as §11.1 rule 3's skin rows; `gadget.noisemaker` in no loot pool; `item.tool.lockpicks`, `item.tool.hacking_deck`, `item.tool.rope`, `item.tool.torch`, `item.tool.toolkit` and `item.tool.pick` templates; doc 05 archetype IDs replacing its older ones |
| 12 | station rooms and tiers (§5.1); the count and rooms of mineral (M) resource nodes, with an M weight in `node_w` (Canon gaps 13), and the §9.4 kind and size draws on `rng.worldgen`; `law.post_apocalypse.scarcity`; lock tiers on doors; genre stone, wood and `block.light_fixture.<suffix>`; walls without `destructible`; `block.masonry.<m>` for doc 18's wall and floor materials |
| 13 | `ai.action.breach`; climb links on placed ladders, rope ladders and `block.vine`; door links using `break_ticks(cell)`; trap knowledge per squad; `disabled_until` for disabling abilities; the template `materials` list; its `block.dirt` read as `block.soil` |
| 14 | `unlock.loot.recipes_2`, `unlock.loot.gadgets_2`, `run.marg.brittle_kit` rank `r`, `d_eff` |
| 16, 17, 18 | host-authoritative crafting with instanced station outputs; grid, assembly and station UI with a 3 × 3 grid at `station.hand` (doc 17's 2 × 2 hand grid cannot hold the barricade, ladder and bridge patterns); skin names and localization keys |

## Canon gaps

1. **Version.** This chapter conforms to canon-1.5. Canon-1.2 assigned it Delving hooks; canon-1.4 replaced Delving with `skill.fieldcraft`, whose perks doc 04 owns. §13 maps every hook to doc 04's Fieldcraft perks; the earlier Delving perk set (+1 node unit at 25, mining one tier above at 50, placed HP × 15000 at 75) no longer exists.
2. **`destructible`.** Canon §4.3 names a block tag. Resolved with doc 03 §4.5: `block.tag.destructible` exists, a block without it has `hardness_milli = -1`, and doc 03's loader forbids it to burn, melt, fall or explode; doc 12 §9.2 agrees.
3. **Tags and roles as IDs.** Other docs write material tags bare (`flammable`, `absorbent`). Assumed: they mean `mat.tag.<name>`.
4. **§13.9 instance fields.** Canon §13.9 lists `parts` as `{slot, part_id}` and no crafting fields. This chapter uses `{slot, part_id, mfr, quality}` plus `crafted`, `mat_bonus_bp`, `dish` and ammunition `smart`; doc 19 owns the full schema.
5. **Grafts.** Canon §5.3 gives grafts dependency arcana. Assumed: that row means doc 04 body grafts; licensed grafts use their gimmick's doc 11 dependency.
6. **Stream uses.** Canon §3.3 lists `rng.craft` for crafting quality only. Requested: add crafted perk seeds, mining bonus units and seed returns (§1.3). Vein draws stay on doc 12's `rng.worldgen`.
7. **Meal buffs** have no status ID; assumed a per-Erratum `meal_buff` field using doc 04 modifiers.
8. **`st.rooted` in webs** is applied while overlapping, without threshold scaling (Canon §12.1 "while inside"); an Errata-placed web never roots `fac.errata`.
9. **Ward capacity** has no material term in Canon §13.2; materials change ward generators only through mass, durability and value.
10. **Archetype IDs.** Canon §19 gives `arch.` to doc 05, so this chapter uses doc 05's melee, shield and ward IDs.
11. **Doc 12 pool IDs** predate Canon §5.9; §2.9 maps them.
12. **Gadget failure** is rolled per deploy only; turret shots are not activations.
13. **Resource-node count.** Canon §4.3 names resource nodes but no count. Doc 12 §11.2 places `6 + floor_div(rooms, 3)` nodes by `node_w`, whose table has no M (mineral) weight. Requested: doc 12 adds an M weight per genre so a Folio holds at least `6 + floor_div(d_eff, 5)` mineral veins; this chapter's §9.4 rules apply to every M node doc 12 places.
14. **Torch placement.** Doc 03 §13.3 lets `item.tool.torch` place blocks; Canon §4.3 allows only `cat.placeable`. The canon wins: `recipe.hand.wall_torch` turns a held torch into a placeable `block.torch`.
15. **Part namespace.** Doc 11 §5.4 lists `part.` rows; Canon §19 gives `part.` to this chapter. §7.2 is the source of truth and doc 11 mirrors it as loot.
16. **Seized devices.** The old §11.1 rule 2 left a hacked hostile turret hostile; Canon §11.1 says `st.hacked` makes it fight for the hacker. The canon wins (doc 09 gap G14 is closed).
17. **Traps in two categories.** Canon §13.1 lists traps under both `cat.gadget` and `cat.placeable`. Resolved: block traps (`block.trap.spikes`, `block.web`) are `cat.placeable`; snares, mines and charges are `cat.gadget` props.
18. **Block HP tables.** Canon §13.8 lists neither block-HP use. Placed blocks scale with `T(d_place)` against enemy damage; terrain destructibles with `P(d_current)` against Errata weapons (doc 03 G27). Requested: Canon §13.8 adds both rows.
19. **Door HP by lock tier.** Doc 12 §9.2 asks this chapter for it; §10.7 defines `door_lock_bp`.
20. **`block.dirt`.** Doc 13 names `block.dirt`; this chapter's block is `block.soil`.

## Acceptance tests

1. **Materials.** The table loads 69 materials; all 38 Canon §5.9 materials exist and each lists every genre whose §5.9 row names it (`mat.copper` lists SP); every `dmg_bp` and `ar_bp` is in `7000..=14000`; `mat.iron` has 10000 for `dmg_bp`, `ar_bp` and `value_bp`; `mat.star_metal.dmg_bp = 14000`; `mat.redacted_alloy.ar_bp = 14000`.
2. **Units.** `unit_mass_g`: steel 1962, oak 750, leather 860, gold 4825. `unit_value`: steel 11, gold 19, iron 5. `mat.iron.ore` unit mass 1376. Every row's `unit_ml`, `unit_g` and `unit_lucre` equal the §2.5 formulas.
3. **Durability factor.** `dur_mat_bp`: iron 10000, steel 11350, glass 5050, star_metal 13900.
4. **Block defaults.** A destructible iron block has `hardness_milli` 5000, `blast_res_milli` 5500, `heat_cond_bp` 6400; `block.masonry.granite` has −1; `block.masonry.oak` has `flammability_bp` 0 and `burn_runs` 0.
5. **Pools.** `pick_material(weird_west, pistol, ilvl 7)` weights `[copper 6000, hexed_silver 9000, iron 18000, steel 3000]`; the haft secondary is `mat.leather`.
6. **Equivalence.** `equivalent(moonsilver, cyberpunk) = titanium`; `equivalent(jade, mythic_fantasy) = moonsilver`; `equivalent(dragonbone, hardboiled_noir) = dragonbone`.
7. **Quality vector.** For run seed `0x0123456789ABCDEF`, keyed `rng.craft` at (Folio 7, `(7 << 40) | 3`, tick 50000, seq 0) gives `range(141)` = 116, 34, 32 and §14.1's score 310, `q.good`; at (Folio 9, `(9 << 40) | 5`, tick 61000, seq 0) it gives 74, 59, 93 and §14.2's score 402, `q.excellent`.
8. **Distribution.** Center 250, cap Good, over all 2,803,221 noise outcomes: Awful 37,820, Poor 653,710, Normal 1,407,750, Good 703,941. Center 630, cap Peerless: Excellent 88,560, Masterwork 1,601,345, Peerless 1,113,316. Every row of §4.3 matches enumeration to two decimals.
9. **Caps.** Skill 100 at `station.hand` never exceeds `q.normal`; skill 24 at a tier-5 station never exceeds `q.good`.
10. **Draw count.** A quality-rolling craft draws exactly 3 `range(141)` and 1 `next_u64`; a material, placeable or ammunition craft draws 0.
11. **Longsword.** §14.1 yields B 29,829 mHP, mass 1,397 g, `time_bp` 9993, W 10, R 12, `charge_full` 40, light stamina 2,996 mst, impact 11,986 mp, durability 12,485, value 180, needed 277 ticks.
12. **Pistol.** §14.2 yields manufacturer Coffin Creek, magazine 10, reload 67, range 60,000, hip spread 290, ADS spread 58, bloom 50 / 290, recoil 154 / 30, swap 19, ADS 9, mass 1,000 g, B 21,590 mHP, 245 ticks.
13. **Parts math.** A −800 delta from another manufacturer becomes −600; a graft's "+1500 Bp" becomes 1125 and "× 15000" becomes 13750.
14. **Mining.** Iron vein: steel pick Fieldcraft 30 → 101; iron pick Fieldcraft 0 → 150; with `hard_bp` 15000 → 225; steel pick Fieldcraft 75 → 84. A t1 pick on a titanium vein returns `None` at every Fieldcraft value.
15. **Yield.** §14.3 gives 2 ore with bonus draw 9498; the same draw at Fieldcraft 75 gives 3; 2 units under `law.post_apocalypse.scarcity` become 3.
16. **Smelting.** 2 iron ore at forge tier 1, Smithing 20: 219 ticks each, 438 total, 1 coal, 762 fuel ticks left.
17. **Salvage.** A steel longsword yields 1 steel and 0 oak.
18. **Wear.** Long-blade light swing 12, SMG shot 2, sniper shot 34, 5,000 mHP mitigation at Folio 1 → 20; the swing with `curse.brittle` m 5000 → 18; one `self_repair` use costs a toolkit 250.
19. **Repair.** 12,485-max longsword at Smithing 40, station: +5,617; in the field: +2,808, capped at 9,363.
20. **Shatter.** An obsidian dagger reaching durability 0 is destroyed and drops its salvage.
21. **Placed HP.** Wooden barricade 60,000 at Folio 1, 108,120 at Folio 12, 135,150 at Folio 12 with Fieldcraft 25, 216,870 at Folio 25. A reference enemy melee hit breaks it in 12 hits at Folio 1 and 13 at Folio 25. Its HP never decays.
22. **Caps on placing.** A player's 49th live placed block is refused; a bridge cell needing span 6 is refused; placement is refused while mounted, for the invader, and for a held `item.tool.torch`.
23. **Placement time.** A barricade at Fieldcraft 0 takes 60 ticks, at Fieldcraft 30 takes 47, at Fieldcraft 50 takes 40, at Fieldcraft 100 takes 1.
24. **Web.** BRN 40 breaks free after 80 movement ticks; BRN 110 after 20; a web-walker is never rooted; an Errata-placed web never roots an Erratum.
25. **Crops.** Survival 20 with water and light 12: 858 ticks per stage; in light 7 the stage never advances; a fifth live crop is refused; a seed returned by a harvest cannot be replanted until the next Folio.
26. **Turret.** Good brass turret from Folio 5: shot 5,589 mHP, HP 54,507 mHP, 68,133 at Tinkering 50.
27. **Recharge.** A tech-5 chassis in Fairy Tale (TEC 1) recharges at 6000 Bp per tick: 1800-tick recharge takes 3000 ticks.
28. **Hacking-free.** No gadget path reads `skill.hacking`; `st.hacked` applied to an Errata turret is rejected; `short_circuit` sets `disabled_until` only; a hostile turret seized by doc 09 gains `st.hacked`, fights for the hacker's faction and does not count toward the hacker's turret limit.
29. **Recipe overlap.** Adding a shaped recipe `B/H` with keys blade and haft beside `recipe.forge.dagger` fails `recipe_overlap`; `recipe.hand.torch` (`C/W`) coexists with the dagger; adding a workbench or hand shapeless recipe "1 M" beside `recipe.bench.lockpicks` fails (hand recipes run at every station), while a loom-only one passes.
30. **Mirror.** `B./.H` matches `recipe.forge.blade_short`.
31. **Genre legality.** In `genre.unwritten`, `recipe.bench.pistol` is refused.
32. **Butchery.** A 60,000 g quadruped corpse yields 12 flesh, 6 hide, 3 bone; 13, 7 and 4 at Survival 25.
33. **Erasure.** A hunter standing 1000 mm from a placed barricade erases it on its next erasure tick with no drop; the cell's `placed_meta` is removed.
34. **Meal buff.** Eating `hearty_stew` then `herb_bread` leaves only the bread buff, expiring 18,000 ticks after the bread.
35. **Reference masses.** Every two-component row of §3.1 built from its reference materials weighs exactly doc 05's `ref_mass_g`; `arch.ranged.thrown` weighs 299 g and keeps every doc 05 tick count.
36. **Flammable tag.** `mat.chitin`, `mat.nanoweave`, `mat.flesh`, `mat.vellum` and `mat.dreamstuff` carry `flammable`; a material carries it exactly when `flammability_bp >= 500`.
37. **Acoustic.** Every material has exactly one acoustic class; every material doc 09 §7.2 names gets the same class here.
38. **Spark.** A failed deploy by an Erratum with Max HP 85 applies a 4,250 mHP `dmg.shock` hit at Canon §10.5 step 3 and counts one Contradiction failure; three sparks within 600 ticks apply `st.unmoored`.
39. **Charges.** With Tinkering 25 the owner's turret holds 2 charges and redeploys at once after the first deployment ends; at Tinkering 24 it enters Recharging.
40. **Friendly fire.** A frag-mine blast on an Erratum uses `FF_AREA_BP` 10000; a turret bullet on an Erratum uses `FF_DIRECT_BP` 5000.
41. **Trap skill.** Spikes placed at Folio 1 by a Fieldcraft-50 Erratum deal `B` 10,400 mHP with 52 `st.bleeding` power; at Fieldcraft 49, 8,000 and 40.
42. **Doors.** An unlocked wooden door at Folio 1 has `block_hp` 60,000; an iron `lock.t3` door at Folio 11 has 710,190 and `break_ticks` 450; a door with `protect` takes no damage.
43. **Lights.** Snuffing `block.torch` writes `block.torch_out` (light 0) with cause `grid.cause.toggled`; relighting restores light 14; a torch snuffed with 600 lit ticks left burns out 600 lit ticks after relighting.
44. **Nodes.** A demolition charge (block power 8000) never breaks `block.node.iron`; a weapon that breaks one by damage yields §9.3's units to the attacker and leaves `block.node_spent`.
45. **Ammunition.** `recipe.bench.ammo_light` turns 1 `mat.iron` and 1 `mat.coal` into 24 `ammo.light`; with `smart_rounds` the stack is flagged `smart`.
46. **Smithing 100.** A steel longsword crafted at Smithing 100 has `material_bp` 11500; a star_metal one stays at 14000.
47. **Gadget cadence.** A `gadget.drone_combat` with a valid target fires on consecutive ticks 20 apart, and retargets only on its 6-tick phase.
48. **Suppressor.** `part.barrel.suppressor` installs on a pistol and is refused on a crossbow.
49. **Veins.** Each M node doc 12 places draws exactly 1 `weighted` and 1 `range_incl(2, 4)` from its placement sequence and fills that many cells with one node kind.
50. **Noisemaker.** A deployed `gadget.noisemaker` emits `noise.gadget.noisemaker` 10 times, once every 60 ticks of its 600 active ticks.

## Invariants annex

```
Σ doc07: materials ⊗ quality ⊗ stations ⊗ recipes ⊗ parts ⊗ durability ⊗ nodes ⊗ placeables ⊗ doors ⊗ gadgets ⊗ meals
W! ∀ mat: dmg_bp, ar_bp ∈ 7000..=14000 ∧ mat.iron = 10000           N! float ∈ material | recipe | block content
W! flammable ⇔ flammability_bp ≥ 500                                  W! ∀ §5.9 material: genres ⊇ its §5.9 rows
W! mass_g = Σ floor_div(vol_ml × density, 1000)                      W! ref-material mass = doc05 ref_mass_g (thrown 299)
W! material_bp := min(14000, dmg_bp(primary) + mat_bonus_bp)          N! material_bp | mat_ar_bp > 14000
W! quality draws := keyed rng.craft, exactly 4 per rolling output      N! quality draw from sequential stream
W! quality ≤ min(skill_cap, station_cap) before talent shifts          N! station.hand output > q.normal
W! ∀ shaped | shapeless pair: ¬ambiguous (recipe_overlap)             W! inputs consumed only at Done
W! stats rebuilt from base on every part change                        N! compounding part or tendency deltas
W! cross-mfr part delta × 7500 ∧ graft numbers × 7500                  N! graft on Palimpsestic | Exotic | same mfr
W! ¬destructible ⇒ hardness_milli = −1 ∧ never burns                   N! Errata edit of fixed terrain
W! node broken ⇒ block.node_spent ∧ yield → actor                      N! explosion breaks a node
W! place ⇐ cat.placeable ∧ placer ∈ party slots 0..=3                  N! mounted | invader | ghost | held torch places
W! placed block ⇒ placed_by_errata ∧ placed_meta                       N! placed blocks > 48 per player | 160 per Folio
W! placed hp ∝ T(d_place) ∧ never decays                               W! door hp = block_hp_mhp × door_lock_bp × P(d_current)
W! Fieldcraft perks := doc 04 (25 hp ×12500, 50 traps ×13000, 75 +1 unit ∧ repair ×15000, 100 1 tick)
W! gadget dep := genre skin ∧ failure rolled once per deploy           N! gadget reads skill.hacking | accepts st.hacked
W! spark ⇒ shock hit at step 3 ∧ note_contradiction_failure            W! seized hostile device ∉ Errata gadget limits
W! gadget fire check every tick ∧ retarget on 6-tick phase             N! gadget targets fac.errata
W! gadget blast ⇒ FF_AREA_BP ∧ gadget shot ⇒ FF_DIRECT_BP              W! turret target order := marked, dist2, EntityId
W! wear ≥ 1 unless dur_max = 0                                         W! brittle primary at 0 ⇒ shatter + salvage
W! iteration ascending Gidx | EntityId | content ID                    N! HashMap | HashSet ∈ doc07 state
W! one meal_buff per Erratum                                           W! Folio-scoped doc07 state cleared at ev.folio_exited
W! live crops ≤ 4 per player ∧ harvested seed spent until next Folio   N! node, salvage or butchery unit with min_d > d_eff
W! dismantled placeable keeps its uid                                  N! placement XP twice for one uid
```
