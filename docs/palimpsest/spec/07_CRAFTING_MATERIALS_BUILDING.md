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
| `recipe.<group>.<name>` | recipes (§6.6, §7.2, §8.5, §11.2, §12.2) | 114 |
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
| `stations: BTreeMap<Gidx, StationState>` | Folio | tier, fuel ticks, job queue, output slots (§5.3) |
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

### 2.2 Material table

All 69 materials, one row each, with every property. The block is CSV so tools can load it directly; `+` joins multi-valued fields and `-` means none.

```csv
id,density,unit_ml,unit_g,hard,tough,sharp,cond,flam_bp,melt_decic,conceal,pen_cost,tier,band,lucre_kg,unit_lucre,value_bp,dmg_bp,ar_bp,arc,tech,genres,roles,tags
copper,8960,250,2240,3,5,3,10,0,10850,15,250,1,1..=20,4,8,8000,8000,8000,4,6,*,bl+hd+fr+pl,metal+ricochet
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
chitin,1300,1000,1300,4,5,5,1,500,-,15,80,2,4..,8,10,10000,10000,11000,3,2,CH,bl+li+pl+so,acid_soluble
hide,1100,1000,1100,1,5,0,1,2000,-,15,25,0,1..=8,4,4,6000,7000,8500,2,1,*,so,flammable+acid_soluble
leather,860,1000,860,2,5,0,1,1500,-,15,30,1,1..=20,8,6,8000,9000,10000,3,2,WW+*,so+ha,flammable+acid_soluble
linen,1500,1000,1500,1,3,0,1,6000,-,15,10,0,1..=8,3,4,6000,7000,10000,3,1,*,so,fabric+flammable+acid_soluble
tweed,1300,1000,1300,1,4,0,1,4500,-,15,12,1,1..=20,5,6,9000,7000,10500,2,2,HN,so,fabric+flammable+acid_soluble
silk,1300,1000,1300,1,6,0,1,5000,-,15,10,2,4..,12,15,16000,7500,11000,6,2,WX,so+li,fabric+flammable+acid_soluble
rubber,1100,1000,1100,2,6,0,0,4000,-,15,40,1,1..=20,6,6,8000,7000,9500,0,6,PA,so+ha,flammable+fuel
polymer,1200,1000,1200,3,6,1,0,3000,2500,15,60,2,4..,8,9,10000,9000,10000,0,9,CP+SO+RD+PA,fr+ha+so+bk,flammable+fuel
carbon_fiber,1600,1000,1600,6,8,4,7,1000,-,15,200,3,9..,25,40,18000,11500,12500,1,10,CP,fr+ha+li+pl,flammable
nanoweave,1000,1000,1000,4,9,0,1,500,-,15,150,4,14..,100,100,30000,12000,13000,2,10,CP,so,fabric
coral,1400,1000,1400,3,3,5,1,0,-,15,70,2,4..,10,14,12000,9500,9000,6,1,CH,bl+hd+fo,stone
flesh,1060,1000,1060,1,2,0,2,500,-,15,15,0,1..,1,1,2000,7000,7000,1,0,*,-,edible+fat+acid_soluble+terrain
web,1300,1000,1300,1,3,0,0,9000,-,3,1,0,1..,6,7,6000,7000,7000,4,0,*,so,flammable+acid_soluble+terrain
paper,800,1000,800,1,1,0,0,9000,-,15,8,0,1..=8,2,1,5000,7000,7000,6,1,RD+UW+*,so+fo,flammable+fuel+acid_soluble
vellum,900,1000,900,5,7,2,0,2000,-,15,200,5,20..,150,135,50000,13000,13000,10,3,UW+UT,so+fo+pl,arcane_reagent
dreamstuff,400,1000,400,4,6,6,3,1000,-,6,100,5,20..,350,140,50000,13000,12500,10,5,ON,bl+hd+fr+pl+so+fo,arcane_reagent+luminous
air,1,1000,1,0,0,0,0,0,-,0,0,0,1..,0,0,0,7000,7000,0,0,*,-,nonsolid+terrain
void,99999,250,24999,10,10,0,0,0,-,15,inf,0,1..,0,0,0,7000,7000,0,0,*,-,unbreakable+terrain
soil,1500,1000,1500,1,2,0,1,0,-,15,60,0,1..,0,0,0,7000,7000,1,0,*,bk,absorbent+tillable+terrain
sand,1600,1000,1600,1,1,0,1,0,17000,15,50,0,1..,0,0,0,7000,7000,0,1,*,bk,absorbent+terrain
ice,917,1000,917,2,2,3,2,0,10,2,20,0,1..,0,0,0,9000,7500,2,0,*,bl,brittle+ricochet+terrain
leaves,300,1000,300,1,1,0,0,7000,-,3,1,0,1..,0,0,0,7000,7000,3,0,*,-,flammable+fuel+absorbent+acid_soluble+terrain
coal,1350,1000,1350,2,2,0,3,8000,-,15,70,0,1..,1,1,3000,7000,7000,0,3,*,-,flammable+fuel
slag,2800,250,700,4,2,1,3,0,13000,15,140,0,1..,0,0,1000,8000,8000,0,2,*,bk,stone+ricochet+terrain
```

### 2.3 Column legend

`density` kg/m³; `unit_ml` and `unit_g` are the unit volume and mass (§2.5); `hard`, `tough`, `sharp`, `cond` are hardness, toughness, sharpness and conductivity `0..=10`; `flam_bp` flammability; `melt_decic` melt point (`-` never melts); `conceal` Canon §12.2; `pen_cost` doc 05 §7 wall penetration (`inf` for `mat.void`); `tier` 0 to 5; `band` is `min_d..=max_d` for loot and node placement (`min_d..` has no upper bound); `lucre_kg` value per kg and `unit_lucre` unit value, both at `P(1)`; `value_bp` doc 10 and doc 11 value factor; `dmg_bp` and `ar_bp` Canon §10.4 factors; `arc` and `tech` arcane and tech affinity `0..=10`.

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

The secondary component never draws: it is the highest-weight entry of the same procedure for the secondary role, ties to the lower ID. Example, Weird West frame at ilvl 7: `[copper 6000, hexed_silver 9000, iron 18000, steel 3000]`; the haft pool is `[birch 2000, leather 18000, oak 6000, pine 2000]`, so the secondary is `mat.leather`. In Marginalia (`ilvl >= 26`) every band test passes for materials with `max_d = 0`.

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
| `arch.melee.mace` | head 181 | haft 500 | 1799 | 1 + 2 |
| `arch.melee.warhammer` | head 375 | haft 1400 | 4001 | 4 + 1 |
| `arch.melee.spear` | blade 57 | haft 1800 | 1798 | 1 + 2 |
| `arch.melee.halberd` | blade 140 | haft 2800 | 3201 | 2 + 2 |
| `arch.melee.quarterstaff` | haft 2400 | — | 1800 | 3 |
| `arch.melee.fist` | head 60 | soft 150 | 601 | 1 + 1 |
| `arch.melee.whip` | soft 683 | haft 150 | 699 | 2 + 1 |
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
| `arch.ranged.thrown` | blade 38 | — | 299 | 2 per 3 |
| `arch.ranged.staff` | haft 2000 | focus 40 | 1530 | 2 + 1 |
| `arch.ranged.wand` | focus 400 | — | 300 | 1 |
| `arch.offhand.shield_buckler` / `_heater` / `_tower` | plate 172 / 480 / 1105 | haft 200 / 300 / 400 | 1503 / 4002 / 8996 | 1+1 / 2+1 / 5+1 |
| `arch.offhand.lantern` | frame 152 | — | 1196 | 1 + 1 glass |
| `arch.offhand.focus` | focus 800 | — | 600 | 1 |
| `arch.offhand.tome` | soft 959 | focus 100 | 899 | 1 + 1 |
| `arch.ward.aegis` / `kinetic` / `reflector` / `bulwark` | frame 34 / 147 / 96 / 372 | focus 40 / 60 / 60 / 100 | 297 / 1201 / 800 / 3002 | 1+1 / 1+1 / 1+1 / 2+1 |
| `item.tool.pick` | head 203 | haft 1200 | 2497 | 3 + 2 |
| `item.tool.toolkit` | frame 30 | haft 80 | 296 | 1 + 1 |

Armor has one component. Its volume is `ceil_div(ref_mass_g × 1000, ref_density)` with doc 05 Table 11.1b masses and reference materials cloth `mat.linen`, light `mat.leather`, medium and heavy `mat.iron`. Cells (head, body, hands, legs, feet): cloth 1/1/1/1/1, light 1/5/1/3/2, medium 1/5/1/3/1, heavy 2/8/1/4/2. Example: medium body is `ceil_div(8000000, 7870) = 1017` ml.

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

### 3.3 Armor and shields

1. **AR.** Canon §10.4 with `mat_ar_bp = ar_bp(primary)`.
2. **Poise.** `poise' = mul_bp(poise, 10000 + 400 × (toughness - ref_toughness))`, with `ref_toughness` 3 (cloth), 5 (light), 6 (medium, heavy, shields).
3. **Resistances per piece**, added before the cap (Canon §10.2): conductivity ≥ 8 gives `dmg.shock` −200; conductivity ≤ 1 with density ≤ 1600 gives `dmg.shock` +200; flammability ≥ 5000 gives `dmg.fire` −200; `silvered` gives `dmg.umbral` +300.
4. **Mass** is `vol × density`; doc 04 §11.6 counts it.

Example: a medium body piece in `mat.bronze`, ilvl 4, Common, Normal, +0: torso AR `20 → × T(4) = 23 → 23 → 23 → 23 → × 9000 = 20`; mass `floor_div(1017 × 8800, 1000) = 8949` g; poise 6 (bronze toughness 6).

## 4. Crafting quality

### 4.1 Which outputs roll

Weapons, armor, off-hands, ward generators, tools, gadgets and parts roll quality. Material units, placeables, food and other consumables are `q.normal` and draw nothing.

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
        + helpers.iter().take(2).map(|h| min(25, floor_div(*h, 4))).sum::<i32>()   // two highest helper skills
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

`rarity = rar.common`; `buc = buc.uncursed`; `ilvl` = current Folio index (`d_eff` in Abridged mode), which is also its `src_lvl` (Canon §13.8.1); `origin_genre` = the local genre at the crafter, or the Folio's genre in a seam zone; knowledge fully identified (doc 10); `finder_slot` = crafter's slot; `crafted = 1`; manufacturer per §7.4; `gear_tier` per doc 11. Weapons get perk columns 1 to `n`, where `n` is 1 at Normal or below, 2 at Good or Excellent, 3 at Masterwork, 4 at Peerless; the fourth `rng.craft` draw (`next_u64`) seeds doc 11 `roll_perks(sub(seed, "loot.perk"))`, and column 5 holds the genre trait. Other outputs ignore the fourth draw, so the count stays 4.

Skill XP per completed craft (doc 04 §5.2): `1000 × input_units + 2000 × recipe_tier`, × 15000 if quality ≥ Excellent. Process recipes have `recipe_tier` 0.

## 5. Stations

### 5.1 Station list

| ID | Functions | Tiers | Buildable | Found in | Fuel |
|---|---|---|---|---|---|
| `station.hand` | 3×3 grid; recipes marked hand; quality cap Normal | — | always available | — | no |
| `station.campfire` | cooking tier 1; smelting with melt cap 12000; charcoal | 1 | yes, `block.campfire` (a light source, Canon §4.3) | `room.campfire` service | yes |
| `station.workbench` | 3×3 grid; assembly; fabrication; Tinkering repair; salvage; graft extraction; implants (doc 04 Tinkering 75) | 1–5 | no | `room.workshop`, Colophon | no |
| `station.forge` | smelting and alloying with melt cap; 3×3 smithing grid; shields; Smithing repair; forge reforge; recast | 1–5 | no | `room.forge` | yes |
| `station.loom` | tanning, weaving, cloth and light armor | 1–5 | no | `room.workshop`, `room.stable` | no |
| `station.kitchen` | every cooking recipe | 1–5 | no | `room.larder` | yes |
| `station.alchemy` | doc 08 brewing with installed apparatus | 1–5 | no | `room.alchemy_lab` | no |
| `station.enchanting` | doc 06 §9 enchanting; ritual processes | 1–5 | no | `room.shrine`, `room.library` | no |
| `station.scriptorium` | doc 11 Rewrite and reforge; doc 10 services | 1 | no | `room.scriptorium`, Colophon | no |

Forge melt caps by tier: 16000, 18000, 20000, 32000, 40000 DeciC. A smelting, alloying or glass recipe requires `melt_cap >= melt_decic(output)`.

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
    "output": { "template": "arch.ranged.pistol", "count": 1 } }
]
```

Key kinds: `{"mat"}`, `{"role"}`, `{"tag"}`, `{"template"}`, `{"liquid", "ml"}` (drawn from held containers, doc 08). Every cell bound to one key letter must hold the same template. Station `S` with tier `t` runs a recipe when `S` is the recipe's station (a `station.hand` recipe runs anywhere; a campfire cooking recipe also runs at a kitchen; a campfire smelting or charcoal recipe also runs at a forge) and `t >= station_tier`.

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

Lookup tries recipes in ascending ID among those the crafter knows and the station runs; the first match wins. The content loader rejects two shaped recipes whose masks (or a mirror) are equal while every cell pair's accepted material sets intersect (`recipe_overlap`), so at most one recipe ever matches.

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
needed = ceil_div(ticks × count × 10000, min(30000, 10000 + 50 × skill + 1000 × (station_tier - recipe.station_tier) + 1500 × helpers))
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
| `recipe.loom.tan_leather` | loom 1 | survival 0 | 1 `mat.hide` → 1 `mat.leather` | 600 | — |
| `recipe.loom.weave_silk` | loom 2 | survival 20 | 3 web → 1 `mat.silk` | 600 | — |
| `recipe.fab.polymer` | workbench 2 | tinkering 25 | 1 rubber + 1 coal → 2 `mat.polymer` | 600 | — |
| `recipe.fab.carbon_fiber` | workbench 3 | tinkering 50 | 2 polymer + 1 coal → 1 `mat.carbon_fiber` | 900 | L |
| `recipe.fab.plasteel` | workbench 3 | tinkering 50 | 1 steel + 1 polymer → 1 `mat.plasteel` | 900 | L |
| `recipe.fab.nanoweave` | workbench 4 | tinkering 75 | 1 carbon fiber + 1 silk → 1 `mat.nanoweave` | 1200 | L r2 |

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
| `recipe.hand.torch` | hand | fieldcraft 0 | `C/W` → 4 `item.tool.torch` | 30 | 0 | |
| `recipe.hand.bandage` | hand | medicine 0 | shapeless 1 T → 3 `item.tool.bandage` | 30 | 0 | |
| `recipe.hand.barricade_wood` | hand | fieldcraft 0 | `WWW/WWW` → 4 `block.barricade.wood` | 120 | 0 | |
| `recipe.hand.barricade_stone` | hand | fieldcraft 0 | `KK/KK` → 2 `block.barricade.stone` | 120 | 0 | |
| `recipe.hand.barricade_gate` | hand | fieldcraft 10 | `WW/WW/WW` → 1 `block.barricade.gate` | 120 | 0 | |
| `recipe.forge.barricade_metal` | forge 1 | fieldcraft 10 | `MMM/M.M/MMM` → 4 `block.barricade.metal` | 300 | 1 | |
| `recipe.hand.ladder_wood` | hand | fieldcraft 0 | `W.W/WWW/W.W` → 4 `block.ladder.wood` | 60 | 0 | |
| `recipe.loom.ladder_rope` | loom 1 | fieldcraft 10 | `T.T/TTT/T.T` → 4 `block.ladder.rope` | 120 | 0 | |
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

Armor counts are head/body/hands/legs/feet; the player picks the slot. The torch, bandage and pick outputs are doc 11 templates whose instance carries the materials used.

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

Gadget assembly recipes are in §11.2, part recipes in §7.2, disassembly in §8.5 and cooking in §12.2.

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

Doc 11 §5.4 fixes the 22 loot parts and their modifiers: `part.barrel.{short,long,heavy,rifled}` → barrel, `part.feed.{extended,quick,drum}` → feed, `part.optic.{iron,scope,reflex}` → optic, `part.edge.{keen,heavy,serrated}` → edge, `part.haft.{long,light}` → haft, `part.grip.{wrapped,guarded}` → grip, `part.focus.{crystal,conduit}` → focus, `part.plate.reinforced` → plate, `part.lining.padded` → lining, `part.coil.capacitor` → coil. This chapter adds six crafted-only parts:

| ID | Slot | Modifiers |
|---|---|---|
| `part.stock.braced` | stock | recoil −1500, H +500 |
| `part.stock.folding` | stock | H −2000, recoil +1000 |
| `part.module.battery` | module | gadget duration +5000 |
| `part.module.plating` | module | gadget Max HP +3000 |
| `part.module.targeting` | module | gadget spread −3000, gadget range +2000 |
| `part.module.payload` | module | gadget damage (D) +2000 |

`recoil` multiplies doc 05 `recoil.v_bam` and `recoil.h_bam`. Letters follow doc 11 §5.1. A crafted part has `mfr.errata.marginal_press`, a rolled quality, value `mul_bp(25, P(ilvl))` and mass 300 g.

| Part recipe | Station | Skill | Variants and inputs | Ticks | Tier |
|---|---|---|---|---|---|
| `recipe.bench.part_gun` | workbench 1 | tinkering 15 | barrel (4) F1; feed (3) F1 + 1 M; optic iron F1; optic scope, reflex F1 + 1 G; stock (2) H1 | 240 | 1 |
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
2. `part.edge.keen` and `part.edge.serrated` need a `dmg.slash` share of at least 4000; haft parts need a haft secondary; plate parts need medium or heavy armor or a shield.
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
| Pick, per node broken | `10 × hardness`; × 2 when one tier above via Fieldcraft 50 |
| Toolkit | 10 per quality-rolling craft; 20 per repair |
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

Governing skill: Smithing for melee, thrown, charge rifles, armor, shields, staves and picks; Tinkering for guns, crossbows, beams, ward generators, lanterns, gadgets, toolkits and implants; Inscription for foci and tomes.

```
restore = mul_bp(dur_max, 2500) → × (10000 + 50 × skill) → × 15000 if Smithing-governed and Smithing ≥ 25
        → × 15000 with talent field_repair → × 5000 in the field
cur = min(cap, cur + restore)    // cap = dur_max at a station, mul_bp(dur_max, 7500) in the field
```

A repair consumes 1 unit of the primary material and takes 120 ticks at the matching station or 240 ticks in the field with a toolkit. Example: the 12,485 longsword at Smithing 40 restores `3121 → 3745 → 5617`. A `kin.construct` Erratum is repaired the same way with 1 metal unit: it heals `mul_bp(max_hp_mhp, 1000 + 20 × tinkering)` (Canon §16.3).

### 8.5 Salvage and disassembly

| ID | Station | Skill | Rule | Ticks |
|---|---|---|---|---|
| `recipe.disassemble.salvage` | any station, or field | governing skill 0 | doc 11 §16.4; per component `units = round_half_even(mul_bp(mass_g, 6000), unit_mass_g)`, raised to 1 when 0 and the salvaged mass is at least `floor_div(unit_mass_g, 4)`; talent `scrap_savant` adds `floor_div(units, 3)` | 0 at a station, 60 in the field |
| `recipe.disassemble.strip` | workbench or forge | 0 | removes every part intact | 60 |
| `recipe.disassemble.extract_graft` | workbench 3 | Tinkering or Smithing 50 | destroys a Rare-or-better item and yields 1 graft of its manufacturer | 600 (L r2) |

Example: a steel longsword (1,334 g steel, 60 g oak) salvages `mul_bp(1334, 6000) = 800` g of steel, `round_half_even(800, 1962) = 0`, raised to 1 because 800 ≥ 490; the oak gives 36 g, below 187, so 0.

### 8.6 Reforging

- **Forge reforge** (doc 04 Smithing 75) at a forge of tier 3 or more uses doc 11 §16.3's offer and `reforge_count`, and costs `mul_bp(doc 11 cost, 5000)` Lucre plus `2^c` units of the primary material, `c` = `reforge_count` before.
- **Recast** replaces the primary material at a forge (metal, stone, bone, glass and crystal primaries) or workbench (others) of tier ≥ the new material's tier, with governing skill ≥ `20 × new tier`. It consumes the archetype's primary cells of the new material, salvages the old primary (§8.5), keeps every other field, caps quality at the station cap, preserves the durability ratio, recomputes mass, value and stats, and increments `reforge_count` (refused at 4).

## 9. Mining and gathering

### 9.1 Tool tiers

A pick's tier is its head material's tier (minimum `tooltier.t1`); bare hands are `tooltier.t0`. Speed `TOOL_SPEED` (Bp): t0 3000, t1 10000, t2 13000, t3 16000, t4 20000, t5 25000. Class factor: a pick on stone, metal and crystal nodes, an `arch.melee.axe` or `axe_great` (tier of its primary) on timber and wooden blocks, and anything on soft nodes give 10000; any other tool gives 5000. Only resource nodes and `block.tag.destructible` blocks can be mined (Canon §4.3).

### 9.2 Mining time

```rust
fn mining_ticks(node: &Block, tool_tier: u8, class_bp: i32, fieldcraft: i32, hard_bp: i32) -> Option<u32> {
    let req = node.req_tier;                                         // §9.3
    let slow = if tool_tier >= req { 10000 }
               else if fieldcraft >= 50 && tool_tier + 1 == req { 5000 }   // doc 04 Fieldcraft 50
               else { return None };
    let t1 = mul_bp(break_ticks(node.material_hardness()), hard_bp);      // 30 × hardness; doc 12 GPV hard_bp
    let speed = mul_bp(mul_bp(TOOL_SPEED[tool_tier as usize], class_bp), slow) as i64;
    Some(max(1, ceil_div(t1 as i64 * 100_000_000, speed * (10000 + 100 * fieldcraft as i64))) as u32)
}
```

Examples: an iron vein (hardness 5) with a steel pick at Fieldcraft 30 in Mythic Fantasy takes `ceil_div(150 × 10^8, 13000 × 13000) = 89` ticks; with an iron pick at Fieldcraft 0, 150; the same in Space Opera (`hard_bp` 15000), 225; oak timber with a tier-1 axe, 90, and with a pick, 180; clay by hand, 100.

### 9.3 Resource nodes

Nodes are full-shape blocks tagged `block.tag.node` and `block.tag.destructible`. Breaking one turns the cell into `block.node_spent` (unbreakable), so the terrain keeps its shape. Yield = base + 1 if Fieldcraft ≥ 25 and hardness ≥ 3 (doc 04 Delving 25 hook) + 1 on a keyed `rng.craft` `roll_bp(1000 + 50 × fieldcraft)` (1 draw). Ore veins use hardness `max(4, metal hardness)` and `req_tier = max(1, tier - 1)`; soft piles (timber, sand, clay, salvage, bone) have `req_tier = 0`. Fieldcraft XP is `100 × hardness` per node (doc 04).

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

Genre stone / wood / salvage pair (first unit, second unit, third unit): MF granite / oak / —; GH slate / pine (blackwood from d 9) / —; CH basalt / pine / scrap_iron, scrap_iron, copper; SO basalt / bamboo / plasteel, polymer, polymer; CP concrete / bamboo / polymer, polymer, copper; PA concrete / pine / scrap_iron, scrap_iron, rubber; WW sandstone / pine / —; HN brick / oak / steel, glass, glass; WX granite / bamboo (cedar from d 4) / —; SP brick / oak / brass, brass, copper; FT granite / birch (thornwood from d 4) / —; MA marble / cedar / —; ON dreamstuff / elderwood / —; RD concrete / pine / polymer, paper, paper; UW vellum / paper / —; UT marble / oak / —.

### 9.4 Veins per genre

Doc 12 places `6 + floor_div(d_eff, 5)` veins per Folio, each 2 to 4 node blocks of one kind, drawing the kind with these weights (a node is skipped while `d_eff` is below its yield material's `min_d`):

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

Plant blocks (doc 08 H and F sources) take a 30-tick interact (× 5000 at Survival 50) and yield `1 + floor_div(survival, 50)` units of their ingredient; the cell becomes air. No draws. Survival XP 500 per harvest (doc 04).

### 9.6 Butchery

A `cat.corpse` other than Errata Remains is butchered in 240 ticks with Survival and destroyed. Flesh `min(12, floor_div(mass_g, 4000))` units, primary `min(8, floor_div(mass_g, 10000))`, secondary `min(4, floor_div(mass_g, 20000))`; each nonzero count gains 1 at Survival 25. Doc 04 §8.6 owns severed-part butchery and uses the same materials. A doc 13 template `materials` list overrides this table:

| Body plan or tag | Primary | Secondary | Flesh |
|---|---|---|---|
| `body.humanoid` | `mat.bone` | `mat.hide` if tagged beast | yes |
| `body.quadruped`, `body.avian`, `body.serpentine` | `mat.hide` | `mat.bone` | yes |
| `body.insectoid` | `mat.chitin` | — | no |
| `body.construct` | first unit of the genre salvage pair, else `mat.iron` | `mat.copper` | no |
| `body.amorphous`, `body.swarm` | — | — | no |
| tag dragon | `mat.dragonbone` | `mat.hide` | yes |
| tag undead | `mat.bone` | — | no |

Butchering a sapient corpse costs composure 15 unless the butcher has `afflict.hunger` (doc 04 §11.3).

### 9.7 Planting and farming (`cat.seed`)

- **Plant**: a 30-tick action places `block.crop` (age 0) in the air cell above a full `tillable` block. The crop is Errata-placed (owner slot; Editors erase it) and adds `crops[cell] = {seed, stage 0, next}`.
- **Grow**: `stage_ticks = ceil_div(1200 × 10000, 10000 + 50 × survival + 3000 × [liquid water within 2 taxicab cells])`. While `light_at(cell) < 8`, `next` moves 1 tick later each tick. At `next` the stage advances and the props become age 2, 4, 7 for stages 1, 2, 3 (cause `grid.cause.grew`). Random block ticks never drive crops.
- **Harvest** at stage 3 (30 ticks): `3 + floor_div(survival, 34)` units of the seed's plant (doc 08 §8.4) and the seed back at Survival ≥ 25, else on a keyed `rng.craft` `roll_bp(5000)`; the cell becomes air. Harvesting earlier returns only the seed.

Example: Survival 20 beside water in light 12: `ceil_div(12000000, 14000) = 858` ticks per stage, 2,574 to maturity, 3 units at harvest.

## 10. Blocks and building

### 10.1 Block catalog

Block tags: `block.tag.destructible`, `block.tag.errata_placeable`, `block.tag.node`, `block.tag.hearth`, `block.tag.altar`. A block without `destructible` has `hardness_milli = -1` (§2.5), so worldgen terrain never breaks.

| Block | Material | Shape | `hardness_milli` | Notes |
|---|---|---|---|---|
| `block.air`, `block.void_bedrock`, `block.reinforced` | air, void, granite | none, full, full | −1 | doc 03, doc 12 shells |
| `block.masonry.<m>` for granite, slate, basalt, sandstone, marble, brick, concrete, plasteel, dreamstuff, vellum | that material | full | −1 | genre walls (§9.3 stone list) |
| `block.soil`, `block.sand` | soil, sand | full | −1 | soil is `tillable` |
| `block.cracked_wall` | granite | full | 2000 | destructible; hides secrets |
| `block.planks` | oak | full | 3000 | destructible wooden structures |
| `block.glass_pane` | glass | pane | 1000 | destructible |
| `block.door.wood`, `block.door.iron` | oak, iron | door | 3000, 6000 | destructible |
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

Every placeable and `block.conjured` carries `errata_placeable` and `destructible`.

### 10.2 `block.ice` and `block.web`

- **Ice** forms from frost on liquids (doc 03 `timed_revert`, 1200 ticks), doc 06 frost walls and `gadget.mine_cryo`. Fields: `friction_bp` 1500 (Canon §12.1), `light_filter` 2, conceal 2, `melt_decic` 10 → 1000 units `liq.water`, ricochet. Mining it yields 1 `mat.ice`.
- **Web** forms from doc 06 bind walls, doc 13 spinners, `recipe.hand.web_trap` and worldgen dressing. It has no collision. An entity whose capsule overlaps a web cell holds `st.rooted` for as long as it overlaps, without threshold scaling, ending 1 tick after it leaves (Canon §12.1); entities tagged `web_walker` are exempt. Each tick with movement input adds 1 struggle point; at `max(20, 120 - BRN)` points the web breaks (cause `grid.cause.broken`, drops 1 `mat.web`). Its 5,000 block HP (`250 × 20`) also breaks to one 10,000 mHP melee hit (`block_bp` 5000). Fire burns it out in 60 ticks (`burn_runs` 5 at fire period 12).

### 10.3 Placement rules

Doc 03 §13.3 checks apply first. Then:

1. Only `cat.placeable` items (template = block ID) and `item.tool.torch` place blocks. The placer is not mounted, downed or a ghost, and not inside `room.colophon`.
2. The placer's live placed blocks stay under `PLACED_CAP_PER_PLAYER` and the Folio's under `PLACED_CAP_PER_FOLIO`; at a cap the placement is refused.
3. Channel `place_ticks = max(1, ceil_div(base × 10000, 10000 + 100 × fieldcraft))` with base: barricade and gate 60, bridge 40, ladder 20, cover 45, trap 30, torch 10, lantern 20, campfire 60, shrine lamp 120. It cancels like a craft channel (§6.4). Fieldcraft 100 makes every placement 1 tick.
4. Anchors: barricades, cover, traps and the campfire need a sturdy face below; torches, lanterns and shrine lamps attach to any sturdy face; ladders follow doc 03 attach; a rope ladder placed on a ledge top unrolls downward through up to 8 air cells. A bridge cell needs a horizontal neighbor that is a solid non-placed block (`span` 0) or a bridge, and takes `span = 1 + min(neighbor span)`; `span` above 5 is refused.
5. The cell gets the `placed_by_errata` bit (doc 13) and `placed_meta = {owner_slot, hp, hp_max, expires, patron, span}`. The owner can dismantle it in 30 ticks; at `hp >= mul_bp(hp_max, 5000)` the item returns, otherwise nothing.

### 10.4 Placed block HP

```
hp_max = mul_bp(mul_bp(hardness_milli × 20, T(d_place)), if fieldcraft >= 75 { 15000 } else { 10000 })   // doc 04 Delving 75 hook
```

Doc 03 §13.2 block damage to a placed cell reduces `placed_meta.hp` instead of the decaying `block_damage` map; at 0 the block breaks (cause `grid.cause.broken`) and drops nothing. A wooden barricade has 60,000 at Folio 1, 108,120 at Folio 12 (162,180 with Fieldcraft 75) and 216,870 at Folio 25, so a reference enemy melee hit (`block_bp` 5000: 5,000 at Folio 1, 18,072 at Folio 25) needs 12 hits at either depth. Repair: Fieldcraft, 60 ticks, 1 unit of the block's material, `hp += mul_bp(hp_max, mul_bp(2500, 10000 + 100 × fieldcraft))`, capped at `hp_max`.

Specifics: the gate toggles for `fac.errata` and is a locked door to everyone else; spikes hit each non-`fac.errata` entity entering the cell, at most once per 60 ticks, for `B = mul_bp(mul_bp(8000, P(d_place)), dmg_bp(mat.iron))` `dmg.pierce` with 40 `st.bleeding` power (item source, `src_lvl = d_place`); torches burn out after `TORCH_TICKS` (cause `grid.cause.decayed`); a shrine lamp stores the patron chosen at placement, one per player.

### 10.5 AI and Errata-built blocks

1. Placed solid blocks block navigation. Placed ladders give climb links to every agent (doc 13 §4.13); bridges are walkable; gates are locked doors for every non-Errata agent.
2. **Breach.** Errata barricade, cover and gate cells are passable links for ground agents with cost `1000 + floor_div(ceil_div(hp × 60, block_dps) × walk_mm_s, 60)`, `block_dps = floor_div(mul_bp(hit_mhp, 5000) × 60, attack_interval_ticks)`. An agent that reaches the link attacks the block (doc 13 `ai.action.breach`).
3. A brute charge that ends against a placed block deals `mul_bp(charge B, 20000)` block damage. Sapper charges break placed blocks within 2000 mm (doc 13). Burrowers dig them with `dig_ticks` from `break_ticks`.
4. Editors and hunters erase placed blocks (Canon §11.6, doc 13 §10.6) and path through them.
5. Hostile AI ignores Errata traps until a squad member sees one trigger; that squad then costs the cell × 3 (hazard) for the rest of the Folio.
6. Placed lights count for spawn light (Canon §12.3) and the undead and fiend light cost. Cover maps rebuild on `ev.block_changed`. AI never selects a block as a threat target.

## 11. Gadgets and deployables

### 11.1 Rules (Hacking-free)

1. **No Hacking.** No gadget action requires or reads `skill.hacking`. Tinkering sets crafting quality and repair; Fieldcraft sets deploy ticks of ground gadgets (mines, snares, charges, barrier projector); reusable gadgets deploy in `ceil_div(base × 10000, row 17)` ticks (Canon §6.4).
2. **Not hackable.** Errata gadgets are immune to `st.hacked` and every mind status. Hostile disabling effects (doc 13 `sabot`, `short_circuit`) set `disabled_until` and never change faction. An Erratum's hack of a hostile turret or drone is doc 09's and leaves it a hostile device.
3. **Dependency.** Reusable gadgets take the skin dependency `techne` in CH, SO, CP, PA, HN, SP, RD; `arcana` in MF, FT, ON; `mythos` in GH, MA, WW; none in WX, UW, UT. Consumable gadgets have dependency none everywhere.
4. **Contradiction** is evaluated at the deployer. One keyed `rng.combat` failure roll per deploy when the failure chance is above 0; a failure is a spark dealing 5% of the user's Max HP as `dmg.shock` (Canon §5.5), consumes a consumable and starts a reusable gadget's recharge. Potency multiplies damage, healing, duration and status power, never HP.
5. **Entities.** Turrets, drones, beacons, coils and decoys are `fac.errata` entities with `owner_slot`, counted in `ACTIVE_ACTOR_CAP` but not as minions; mines, snares and charges are props. Gadget hits use `FF_DIRECT_BP` (Canon §17.5) and never target `fac.errata`. Kills credit the owner; gadget hits grant Tinkering XP (doc 04).
6. **Magnitudes.** `src_lvl` = the gadget's `ilvl` (Canon §13.8.1). `B = base_mhp → P(src_lvl) → 10000 → quality → 10000 → dmg_bp(chassis)`; `hp_max = base × 1000 → T(src_lvl) → quality → modules and talents`; `AR = base → T(src_lvl) → ar_bp(chassis)`. The chassis is the primary F material.
7. **Recharge.** Per tick `progress += mul_bp(mul_bp(10000 + (TEC - 5) × 1000, 10000 + 200 × (tech_affinity - 5)), talent_bp)`, `TEC` at the owner's position (Canon §5.7), `talent_bp` = 15000 at Tinkering 100 then × 12000 with `overclock`; ready at `recharge × 10000`. A recall (30-tick interact within 3000 mm) needs half the recharge.
8. **Limits per player.** Turrets 1 (+1 `turret_tech`, +2 `legion`), drones 1 (+2 `legion`), beacons 1, coils 1, barrier 1, decoys 2, mines and snares 6 together, charges 3.

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

The first six match doc 11 §3.6 loot stats. Statuses use Canon §11.4 durations and `S(src_lvl)` power. Explosions enter doc 03 §12 as item sources.

```json
{ "id": "gadget.turret_light", "category": "cat.gadget", "reusable": 1, "deploy_ticks": 60,
  "hp_base": 40, "ar_base": 10, "duration_ticks": 1800, "recharge_ticks": 1800, "range_mm": 20000,
  "shot": { "base_mhp": 4000, "shares": { "dmg.pierce": 10000 }, "interval_ticks": 30,
            "ballistics": "arch.ranged.rifle_auto", "spread_bam": 182, "precision_bp": 15000, "impact": 3 },
  "limit_key": "turret", "dur_base": 4000 }
```

### 11.3 Deployment state machine

| State | Event | Next |
|---|---|---|
| Ready | deploy input, limit free, durability > 0, not disabled | Deploying (channel) |
| Deploying | channel cancelled (§6.4 rules) | Ready |
| Deploying | channel done, failure roll succeeds | Spark: consumable consumed, reusable → Recharging |
| Deploying | channel done, no failure | Active (entity or prop spawned, `ev.entity_spawned`) |
| Active | duration ends | Recharging (wear 500); consumables end |
| Active | HP reaches 0 | Recharging with recharge × 15000 (wear 2500); `self_destruct` talent blast (doc 04) |
| Active | owner recall | Recharging with recharge × 5000 (wear 500) |
| Recharging | progress complete | Ready |
| any | durability 0 | Broken until repaired (§8.4) |

### 11.4 Turret and drone think

```rust
fn gadget_think(g: &mut Gadget, w: &World, t: Tick) {           // every 6 ticks at the entity phase (Canon §2.2)
    if g.disabled_until > t || g.expired(t) { return; }
    let tgt = w.hostiles_within(g.pos, g.range_mm)              // ascending EntityId
        .filter(|h| w.line_of_sight(g.muzzle(), h.center()))
        .min_by_key(|h| (!h.has(ST_MARKED), dist2(g.pos, h.pos), h.id));
    if let (Some(h), 0) = (tgt, g.cooldown) {
        w.fire_projectile(g, h, g.shot);                        // 2 spread draws, keyed rng.combat, actor = g.id
        g.cooldown = g.shot.interval_ticks;
    }
}
```

Each hit draws one crit roll at `clamp(500 + (vrv - 5) × 100, 0, 7500)` Bp (0 where VERVE is 0) with multiplier 15000. Example: a Good turret with a brass chassis made in Folio 5 fires `4000 → 5646 → 5646 → 6210 → 6210 → 5589` mHP shots and has `hp_max = mul_bp(mul_bp(40000, 12388), 11000) = 54,507` mHP (68,133 at Tinkering 50).

## 12. Cooking

### 12.1 Rules

Cooking is a process at a campfire (recipes of tier 1) or kitchen (any), governed by Survival, ticks × 5000 at Survival 50 (doc 04). Outputs are doc 11 `item.food.meal` instances carrying `dish` = the recipe ID, or `item.food.ration`. Satiation, freshness and the meal's composure gain follow doc 04 §11.2. Eating a dish with a buff sets `meal_buff = (dish, t + MEAL_BUFF_TICKS)`, replacing any previous buff; the buff applies doc 04 §1.2 modifiers and ends at expiry or on `st.ghost`. Survival XP is `1000 × input_units` per cook (doc 04).

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

Doc 04's Delving perks apply to `skill.fieldcraft` (governed by END): mining and placing speed × `10000 + 100 × skill` (§9.2, §10.3); 25: +1 unit from nodes of hardness ≥ 3 (§9.3); 50: mine one tool tier above at half speed and double wear (§9.2, §8.2); 75: placed blocks × 15000 HP (§10.4); 100: placement takes 1 tick (§10.3). Fieldcraft also sets ground-gadget deploy time (§11.1) and placeable repair (§10.4). XP: `100 × hardness` per node, 50 per placement, at most 40,000 per Folio together (doc 04 §5.2); dismantled or own-broken blocks give 0.

## 14. Worked examples

Run seed `0x0123456789ABCDEF` throughout.

### 14.1 Craft a steel longsword

Folio 7, Mythic Fantasy, forge tier 2 (`1 + floor_div(6, 5)`). The crafter (EntityId `(7 << 40) | 3`) has Smithing 40, BRN 48, an iron toolkit (t1), no helpers.

1. **Steel.** `recipe.alloy.steel` queues 2 iron + 1 coal: `ticks_each = ceil_div(600 × 10000, 10000 + 50 × 40 + 0) = 500`, giving 2 `mat.steel`.
2. **Match.** The grid `B/B/H` with steel, steel, oak matches `recipe.forge.blade_long`. Forge tier 2 ≥ steel tier 2, so the material term is `15 × min(2, 0) = 0`.
3. **Channel.** `needed = ceil_div(360 × 10000, 10000 + 2000 + 1000) = 277` ticks.
4. **Quality.** `center = 200 + 100 + 30 + 0 + 0 + 0 + 8 = 338`. Keyed `rng.craft` (Folio 7, the crafter, start tick 50000, seq 0) gives `range(141)` = 116, 34, 32: noise −28, score 310, so `q.good` (cap Excellent). The fourth draw seeds two perk columns.
5. **Damage.** `14700 → × P(7) 16771 = 24653 → × 10000 → × 11000 = 27118 → × 10000 → × steel 11000 = 29829` mHP per light hit at `move_bp` 10000.
6. **Mass and timing.** `floor_div(170 × 7850, 1000) + floor_div(80 × 750, 1000) = 1334 + 60 = 1394` g; `mass_ratio_bp = 9957`, `time_bp = 9987`; light W/R stay 10/12, `charge_full` 40, light stamina `mul_bp(3000, 9978) = 2993` mst, impact 11,973 mp.
7. **Other.** Durability `10000 → 11350 → 12485`; bleed multiplier 10800; enchant capacity `floor_div(4 × 6, 4) = 6`; value `76 → 127 → 127 → 139 → 139 → × 13000 = 180`; Smithing XP `1000 × 3 + 2000 × 1 = 5000` milli-XP before row 28.

### 14.2 Assemble a pistol from parts

Folio 9, Weird West, workbench tier 2. The crafter (`(9 << 40) | 5`) has Tinkering 45, LOG 52, a steel toolkit (t2) and one helper with Tinkering 30. Components: 1 `mat.iron` frame, 1 `mat.oak` grip. Parts: `part.barrel.long` and `part.feed.quick` (Coffin Creek), `part.optic.reflex` (Ghostline), a Normal crafted `part.stock.braced` (Marginal Press).

1. **Channel.** `ceil_div(360 × 10000, 10000 + 2250 + 1000 + 1500) = 245` ticks.
2. **Quality.** `center = 200 + 112 + 30 + 10 + 15 + 7 + 12 = 386`; draws 74, 59, 93 give noise +16, score 402: `q.excellent` (cap Excellent).
3. **Manufacturer.** Coffin Creek holds 2 parts and makes class RG, so the pistol is `mfr.weird_west.coffin_creek`: tendencies D+500, M−2000, L+1000 and the crit-ricochet gimmick (dependency verve).
4. **Stats.** `base_mhp 8600 → 9030`. Magazine `round_half_even(12 × 8000, 10000) = 10`. Reload `72 → 79` (tendency) `→ 67` (quick feed). Range 60000 → 75000; falloff 18750 / 43750. Swap `18 → 20 → 21`, ADS `9 → 10 → 10`. The reflex optic is cross-manufacturer, `−800 → −600`: hip 364 → 342, ADS spread 73 → 68, bloom 64 → 60, bloom max 364 → 342. Recoil 182 / 36 → 154 / 30. Iron tech affinity 5 adds no reload tendency.
5. **Chain.** `9030 → × P(9) 19925 = 17992 → 17992 → × 12000 = 21590 → 21590 → × iron 10000 = 21590` mHP per shot. Mass `928 + 75 = 1003` g. Durability `10000 → 10000 → 12000`.

### 14.3 Mine and smelt iron

Folio 3, Mythic Fantasy. The Erratum (`(3 << 40) | 2`) has a steel pick (t2), Fieldcraft 30, Smithing 20.

1. **Mine.** An iron vein (hardness 5, req t1): `ceil_div(150 × 10^8, 13000 × 13000) = 89` ticks. At break tick 12000 the keyed bonus roll draws 9498 against 2500: no bonus. Yield `2 + 1 (Fieldcraft 25) = 3` `mat.iron.ore`; the cell becomes `block.node_spent`; pick wear 50; Fieldcraft XP 500.
2. **Smelt.** Forge tier 1 (melt cap 16000 ≥ 15380). `ticks_each = ceil_div(240 × 10000, 11000) = 219`; a batch of 3 takes 657 ticks and burns 1 coal (1200 fuel ticks). Output 3 `mat.iron`; Smithing XP 3 × 1000.

## Exports

| Namespace | IDs |
|---|---|
| `mat.` | the 69 materials of §2.2; `mat.<metal>.ore` for copper, tin, zinc, iron, silver, gold, titanium, orichalcum, star_metal, aetherium; `mat.role.{blade, head, haft, limb, frame, plate, soft, focus, block}`; `mat.tag.{absorbent, acid_soluble, alloying, arcane_reagent, brittle, edible, fabric, fae_bane, fat, flammable, fuel, luminous, metal, nonsolid, ricochet, silvered, stone, terrain, tillable, unbreakable, wood}` |
| `tooltier.` | `t0` … `t5` |
| `station.` | `hand`, `campfire`, `workbench`, `forge`, `loom`, `kitchen`, `alchemy`, `enchanting`, `scriptorium` |
| `recipe.` | `recipe.kind.{shaped, shapeless, assembly, process, disassembly}`; every recipe of §6.6, §7.2, §8.5, §11.2 and §12.2 (the 12 `r2` recipes form `loot.pool.recipes_2`) |
| `part.` | the 22 doc 11 parts; `part.stock.{braced, folding}`; `part.module.{battery, plating, targeting, payload}`; `part.graft.<genre_suffix>.<name>` for the 38 manufacturers of doc 11 §5.2 other than First Press and Marginal Press |
| `block.` | every block of §10.1, including `block.masonry.*`, `block.light_fixture.<genre_suffix>`, 23 `block.node.*`, `block.node_spent`; `block.tag.{destructible, errata_placeable, node, hearth, altar}` |
| `gadget.` | the 12 gadgets of §11.2 (the last six form `loot.pool.gadgets_2`) |

Functions exported to other docs: `pick_material`, `equivalent`, `break_ticks`, `mining_ticks`, `block_hp(cell)`, `unit_mass_g`, `salvage_units`, `dur_max`, `wear`.

## Interface assumptions

| Doc | Assumed to provide |
|---|---|
| 02 | content validation of the §2.1 and §6.2 schemas, including `recipe_overlap` |
| 03 | an optional `tags` field on block records; `sys.crafting` in the tick order (§1.4); saved `placed_meta`, `stations` and `crops` maps cleared when another cause changes the cell; calls to `doc07::mining_ticks`, `doc07::block_hp` and node drops; placed-cell damage routed to `placed_meta.hp`; gadget entity kinds |
| 04 | the Smithing, Tinkering, Survival and Delving (applied to Fieldcraft) mastery perks quoted here; Artificer talents `overclock`, `field_repair`, `scrap_savant`, `turret_tech`, `modular`, `hardlight`, `masterwork`, `self_destruct`, `legion`; an XP row granting Tinkering `min(750, floor_div(750 × dmg, max))` per gadget hit; the Fieldcraft XP row taking over Delving's; food values; meal-buff modifiers |
| 05 | `ref_mass_g` 1530 for `arch.ranged.staff` and 900 for `arch.offhand.tome`; the §3.2 bleed multiplier on `status_bp`; `pen_cost` and `ricochet` read from materials; turret projectiles using `rifle_auto` ballistics |
| 06 | `arcane_affinity`, `mat.tag.arcane_reagent`, `station.enchanting`, `block.conjured`, web and ice walls, the shrine lamp as an altar |
| 08 | `ing.seed_*` as `cat.seed`, plant tags, `liq.water` withdrawal from containers, mineral ingredient extras on nodes |
| 11 | `item.book.schematic` with `teaches`; instance fields `crafted`, `dish`, and part entries `{slot, part_id, mfr, quality}`; reusable gadgets with `stack_max` 1; graft loot; doc 05 archetype IDs replacing its older ones |
| 12 | station rooms and tiers (§5.1); vein counts and kinds (§9.4); genre stone, wood and `block.light_fixture.<suffix>`; walls without `destructible` |
| 13 | `ai.action.breach`; climb links on placed ladders; trap knowledge per squad; `disabled_until` for disabling abilities; the template `materials` list |
| 14 | `unlock.loot.recipes_2`, `unlock.loot.gadgets_2`, `run.marg.brittle_kit` rank `r`, `d_eff` |
| 16, 17, 18 | host-authoritative crafting with instanced station outputs; grid, assembly and station UI; skin names and localization keys |

## Canon gaps

1. **Version.** The contract line names canon-1.2; the canon is now 1.4. Canon-1.4 replaced Delving with `skill.fieldcraft` and limited building. Assumed: the canon-1.2 Delving hooks apply to Fieldcraft (§13), and building is the 1.4 placeable list.
2. **`destructible`.** Canon §4.3 names a block tag; doc 03's schema has none. Assumed: `block.tag.destructible` exists, and a block without it has `hardness_milli = -1`, so doc 03's unbreakable path enforces fixed terrain. Doc 12 §8's "every other wall is breakable" yields to the canon.
3. **Tags and roles as IDs.** Other docs write material tags bare (`flammable`, `absorbent`). Assumed: they mean `mat.tag.<name>`.
4. **§13.9 `parts`** lists `{slot, part_id}`. Assumed extension `{slot, part_id, mfr, quality}` plus an instance flag `crafted`.
5. **Grafts.** Canon §5.3 gives grafts dependency arcana. Assumed: that row means doc 04 body grafts; licensed grafts use their gimmick's doc 11 dependency.
6. **Stream uses.** Canon §3.3 lists `rng.craft` for crafting quality only. Proposed: add crafted perk seeds, mining bonus units and seed returns (§1.3).
7. **Meal buffs** have no status ID; assumed a per-Erratum `meal_buff` field using doc 04 modifiers.
8. **`st.rooted` in webs** is applied while overlapping, without threshold scaling (Canon §12.1 "while inside").
9. **Ward capacity** has no material term in Canon §13.2; materials change ward generators only through mass, durability and value.
10. **Archetype IDs.** Docs 05 and 11 list different melee, shield and ward IDs; Canon §19 gives `arch.` to doc 05, so this chapter uses doc 05's.
11. **Doc 12 pool IDs** predate Canon §5.9; §2.9 maps them.
12. **Gadget failure** is rolled per deploy only; turret shots are not activations.

## Acceptance tests

1. **Materials.** The table loads 69 materials; all 38 Canon §5.9 materials exist; every `dmg_bp` and `ar_bp` is in `7000..=14000`; `mat.iron` has 10000 for `dmg_bp`, `ar_bp` and `value_bp`; `mat.star_metal.dmg_bp = 14000`; `mat.redacted_alloy.ar_bp = 14000`.
2. **Units.** `unit_mass_g`: steel 1962, oak 750, leather 860, gold 4825. `unit_value`: steel 11, gold 19, iron 5. `mat.iron.ore` unit mass 1376.
3. **Durability factor.** `dur_mat_bp`: iron 10000, steel 11350, glass 5050, star_metal 13900.
4. **Block defaults.** A destructible iron block has `hardness_milli` 5000, `blast_res_milli` 5500, `heat_cond_bp` 6400; `block.masonry.granite` has −1.
5. **Pools.** `pick_material(weird_west, pistol, ilvl 7)` weights `[copper 6000, hexed_silver 9000, iron 18000, steel 3000]`; the haft secondary is `mat.leather`.
6. **Equivalence.** `equivalent(moonsilver, cyberpunk) = titanium`; `equivalent(jade, mythic_fantasy) = moonsilver`; `equivalent(dragonbone, hardboiled_noir) = dragonbone`.
7. **Quality vector.** §14.1 inputs give draws 116, 34, 32, score 310, `q.good`; §14.2 gives draws 74, 59, 93, score 402, `q.excellent`.
8. **Distribution.** Center 250, cap Good, over all 2,803,221 noise outcomes: Awful 37,820, Poor 653,710, Normal 1,407,750, Good 703,941. Center 630, cap Peerless: Excellent 88,560, Masterwork 1,601,345, Peerless 1,113,316.
9. **Caps.** Skill 100 at `station.hand` never exceeds `q.normal`; skill 24 at a tier-5 station never exceeds `q.good`.
10. **Draw count.** A quality-rolling craft draws exactly 3 `range(141)` and 1 `next_u64`; a material or placeable craft draws 0.
11. **Longsword.** §14.1 yields B 29,829 mHP, mass 1,394 g, W 10, R 12, durability 12,485, value 180, needed 277 ticks.
12. **Pistol.** §14.2 yields manufacturer Coffin Creek, magazine 10, reload 67, range 75,000, hip spread 342, recoil 154 / 30, B 21,590 mHP, 245 ticks.
13. **Parts math.** A −800 delta from another manufacturer becomes −600; a graft's "+1500 Bp" becomes 1125 and "× 15000" becomes 13750.
14. **Mining.** Iron vein: steel pick Fieldcraft 30 → 89; iron pick Fieldcraft 0 → 150; with `hard_bp` 15000 → 225. A t1 pick on a titanium vein returns `None` at Fieldcraft 49 and 280 ticks at Fieldcraft 50.
15. **Yield.** §14.3 gives 3 ore with bonus draw 9498.
16. **Smelting.** 3 iron ore at forge tier 1, Smithing 20: 219 ticks each, 657 total, 1 coal.
17. **Salvage.** A steel longsword yields 1 steel and 0 oak.
18. **Wear.** Long-blade light swing 12, SMG shot 2, sniper shot 34, 5,000 mHP mitigation at Folio 1 → 20; the swing with `curse.brittle` m 5000 → 18.
19. **Repair.** 12,485-max longsword at Smithing 40, station: +5,617; in the field: +2,808, capped at 9,363.
20. **Shatter.** An obsidian dagger reaching durability 0 is destroyed and drops its salvage.
21. **Placed HP.** Wooden barricade 60,000 at Folio 1, 108,120 at Folio 12, 162,180 at Folio 12 with Fieldcraft 75, 216,870 at Folio 25. Its HP never decays.
22. **Caps on placing.** A player's 49th live placed block is refused; a bridge cell needing span 6 is refused.
23. **Placement time.** A barricade at Fieldcraft 0 takes 60 ticks, at Fieldcraft 50 takes 40, at Fieldcraft 100 takes 1.
24. **Web.** BRN 40 breaks free after 80 movement ticks; BRN 110 after 20; a web-walker is never rooted.
25. **Crops.** Survival 20 with water and light 12: 858 ticks per stage; in light 7 the stage never advances.
26. **Turret.** Good brass turret from Folio 5: shot 5,589 mHP, HP 54,507 mHP, 68,133 at Tinkering 50.
27. **Recharge.** A tech-5 chassis in Fairy Tale (TEC 1) recharges at 6000 Bp per tick: 1800-tick recharge takes 3000 ticks.
28. **Hacking-free.** No gadget path reads `skill.hacking`; `st.hacked` applied to an Errata turret is rejected; `short_circuit` sets `disabled_until` only.
29. **Recipe overlap.** Adding a shaped recipe `B/H` with keys blade and haft beside `recipe.forge.dagger` fails `recipe_overlap`; `recipe.hand.torch` (`C/W`) coexists with the dagger.
30. **Mirror.** `B./.H` matches `recipe.forge.blade_short`.
31. **Genre legality.** In `genre.unwritten`, `recipe.bench.pistol` is refused.
32. **Butchery.** A 60,000 g quadruped corpse yields 12 flesh, 6 hide, 3 bone; 13, 7 and 4 at Survival 25.
33. **Erasure.** A hunter standing 1000 mm from a placed barricade erases it on its next erasure tick with no drop; the cell's `placed_meta` is removed.
34. **Meal buff.** Eating `hearty_stew` then `herb_bread` leaves only the bread buff, expiring 18,000 ticks after the bread.

## Invariants annex

```
Σ doc07: materials ⊗ quality ⊗ stations ⊗ recipes ⊗ parts ⊗ durability ⊗ nodes ⊗ placeables ⊗ gadgets ⊗ meals
W! ∀ mat: dmg_bp, ar_bp ∈ 7000..=14000 ∧ mat.iron = 10000           N! float ∈ material | recipe | block content
W! mass_g = Σ floor_div(vol_ml × density, 1000)                      W! material_bp := dmg_bp(primary)
W! quality draws := keyed rng.craft, exactly 4 per rolling output      N! quality draw from sequential stream
W! quality ≤ min(skill_cap, station_cap) before talent shifts          N! station.hand output > q.normal
W! ∀ shaped pair: ¬ambiguous (recipe_overlap)                         W! inputs consumed only at Done
W! stats rebuilt from base on every part change                        N! compounding part or tendency deltas
W! cross-mfr part delta × 7500 ∧ graft numbers × 7500                  N! graft on Palimpsestic | Exotic | same mfr
W! ¬destructible ⇒ hardness_milli = −1                                 N! Errata edit of fixed terrain
W! node broken ⇒ block.node_spent                                       W! placed block ⇒ placed_by_errata ∧ placed_meta
W! placed hp ∝ T(d_place) ∧ never decays                               N! placed blocks > 48 per player | 160 per Folio
W! gadget dep := genre skin ∧ failure rolled once per deploy           N! gadget reads skill.hacking | accepts st.hacked
W! turret target order := marked, dist2, EntityId                      N! gadget targets fac.errata
W! wear ≥ 1 unless dur_max = 0                                         W! brittle primary at 0 ⇒ shatter + salvage
W! iteration ascending Gidx | EntityId | content ID                    N! HashMap | HashSet ∈ doc07 state
W! one meal_buff per Erratum                                           W! Folio-scoped doc07 state cleared at ev.folio_exited
```
