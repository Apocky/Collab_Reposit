# 08 — Alchemy and Liquids
As of 2026-10-11. Status: BINDING. Conforms to canon-1.2.

Owns liquids, gases, ingredients, alchemical effects, apparatus, potions, bombs, transmutations, the liquid and gas automata transfer rules, the full Canon §12.1 table, `res.toxicity`, and the behavior of `st.poisoned`, `st.irradiated`, `st.soaked`, `st.oiled` and `st.focused`.

## Imports

|From|Section|What this chapter consumes|
|---|---|---|
|Canon|§1.1–§1.11|ID grammar, `id32`, text keys; numeric types (`Milli`, `Bp`, `Ticks`, `Ml`, `DeciC`, `Grams`); `floor_div`, `mul_bp` chains; display-unit rule; integer content; block coordinates; ascending-ID iteration; no `HashMap`|
|Canon|§2.2, §2.3|tick schedule (liquids, gases, fire, status pulse, temperature, needs); status active interval; rate accumulator|
|Canon|§3.3, §3.4|keyed `rng.alchemy` and `rng.combat`; draw-count rule|
|Canon|§4.1, §4.2|eye height, swim speed, `SAFE_FALL`|
|Canon|§5.3–§5.8|dependencies (mundane vs supernatural alchemy), `Req`, tolerance, Contradiction, Anachronism Edge, local genre|
|Canon|§6.4, §7|rows 1–3, 17, 20, 22–25, 41; `skill.alchemy` (ANL), tiers 25/50/75/100, magnitude-based non-combat XP|
|Canon|§10, §11|damage types, resistance, item source chain (§10.4), environmental entry (§10.6), healing in Bp of Max HP (§10.9); status list, buildup, DoT, direct durations, Erasure|
|Canon|§12, §12.1, §12.2|`res.toxicity` (owner 08), `res.breath`, respirators, surfaces and reactions, concealment|
|Canon|§13, §16, §17.5, §18, §19|item categories, quality, `idc.potion`, knowledge, `buc`, `P`/`T`/`S`, source level, item contract; kin, afflictions, blessed cure; `FF_AREA_BP`; events; namespaces `liq.`, `gas.`, `ing.`, `alch.`, `apparatus.`|
|Docs 03–18|see Interface assumptions|grids and tick order (03), skills and afflictions (04), throwing and hits (05), holy water (06), materials, blocks and stations (07), stealth (09), identification (10), loot (11), worldgen (12), enemies (13)|

---

## 1. Conventions and constants

### 1.1 Design pillars

1. **Liquids are matter.** Integer milliliters per cell that flow, mix, freeze, boil, burn, conduct and react by data; a thrown potion joins the same automaton as a lake.
2. **Alchemy is identification.** Effects reveal by skill, tasting and brewing; brewing a canonical type identifies it (Shattered Pixel Dungeon, Morrowind).
3. **No random ruin.** No failure roll destroys ingredients; one variance draw moves quality by at most one tier.
4. **One ruleset, many skins.** Elixir, stim, chem, dan pill, fairy dew and ambrosia share one archetype; skins never change numbers.
5. **Toxicity is the brake.** `res.toxicity`, diminishing returns and overdose bound consumable spam.

### 1.2 Global constants

|Constant|Value|Meaning|
|---|---|---|
|`LIQ_UNIT_ML`|1,000|doc 03 §6.1 grid unit; a full cell holds 1,000 units|
|`GAS_FULL_CONC`|10,000|doc 03 §7.1 concentration of a saturated cell (1,000,000 ml of gas)|
|`GAS_CONTACT_CONC`|500|doc 03 §5.20: an eye cell holds a gas at this concentration or more|
|`DOSE_ML`|250|volume of one standard dose (flask)|
|`PAYLOAD_MAX_ACTIVE`|512|payload records alive per Folio (§2.5)|
|`LINGER_RADIUS_MM`, `LINGER_TICKS`|1,500; 1,200|Linger surface patch (§10.6)|
|`VAPOR_RADIUS_MM`, `VAPOR_TICKS`|4,000; 600|vapor cloud patch (§10.4)|
|`DR_WINDOW`|1,800 ticks|diminishing-returns window per (recipient, effect) (§11.3)|
|`TOX_CAP`|100,000|Canon §12 `res.toxicity` maximum|

### 1.3 Determinism

- Liquid and gas behavior, reactions, triggers and contact are integer rules that draw **no** random numbers (doc 03 runs them; fire spread alone uses `rng.fire`, doc 03 §8.3).
- Random draws in this chapter come only from keyed `rng.alchemy` (brewing variance §9.5, seed brewing §15.2, wellspring drinks §12.5) and keyed `rng.combat` (Contradiction failure of potions, §9.9). Each draw site states its draw count.
- Mutating iterations run in ascending `EntityId`, `uid` or payload ID.

---

## 2. Liquids in the world grid

### 2.1 Division of labor with doc 03

- **Doc 03 §6 owns the grid**: the cell record (at most 3 liquids per cell, `u16` units), capacity, activation, run order and budget, the cell step (reactions hook, freeze, boil, fall, swap, spread, evaporation, absorption), `deposit`, `splash`, `drain`, contact sampling (§5.20: contact is at least 1 unit in the feet or body cell; `eye_submerged`), swim and wade speeds, surface triggers for the Canon §12.1 rows (§6.9), contact application (§6.10), gases (§7), fire (§8) and temperature (§9).
- **This chapter owns the content**: every `liq.` and `gas.` record with the fields doc 03 §6.5 and §7.2 consume (§3.1, §4.2), every contact and drink row beyond the Canon ones (§3.2, §4.2), the extended trigger rows (§5.2), `alch_react` (§5.3), unit conversion for vessels (§2.3), payload patches (§2.5) and everything alchemical.

### 2.2 Liquid record

```json
{
  "id": "liq.oil", "group": "oily", "color": "#8C6D1F",
  "density_g_per_l": 900, "flow_bp": 7000, "move_bp": 8000, "evap_period": 0,
  "freeze_decic": null, "freeze_block": null, "boil_decic": null, "boil_gas": null,
  "temp_decic": 150, "heat_drive_bp": 0, "light_emit": 0,
  "fire_fuel": true, "flammability_bp": 3000, "ignite_blast_mhp_per_unit": 10000,
  "extinguishes": false, "conductive": false, "conceal_per_cell": 3,
  "evap_gas": null, "dep": [], "vessel_flag": null,
  "contact": [{ "kind": "status_direct", "status": "st.oiled" }, { "kind": "friction_bp", "value": 3000 }],
  "drink": [{ "kind": "toxicity", "milli": 10000 }],
  "triggers": { "fire": "ignite", "frost": null, "shock": null },
  "brew_base": { "mag_bp": 9000, "dur_bp": 15000, "tox_bp": 9000, "coat_charges_bp": 15000 }
}
```

`group` ∈ `aq` (aqueous), `oily`, `gel`, `molten`, `metal`, `cryo`, `ether`; groups drive naming and reaction tags only. `evap_gas` (`{gas, conc}`) is emitted into the cell each time doc 03 step 7 evaporates a unit of this liquid. Doc 12's `liquid_viscosity_bp` (default 10000) multiplies `flow_bp`, clamped to `500..=10000`.

### 2.3 Units, vessels and the grid

- Quantities in this chapter are authored in ml (Canon §1.6). Vessels hold exact ml as item state (§6).
- **Into the grid** from a vessel or item (pour, spill, splash, bleeding): `ceil_div(ml, LIQ_UNIT_ML)` units, so every dose leaves a visible puddle. World quantities authored by docs 06, 12 and 13 use doc 03's `floor_div` conversion.
- **Out of the grid** into a vessel: filling drains `ceil_div(room_ml, 1000)` units (doc 03 `drain`) and the vessel keeps `min(room_ml, drained × 1000)` ml by the doc 03 proportional split. Filling a flask from a puddle therefore costs a whole unit, and pouring it back returns one unit.
- **Mixtures.** Doc 03 averages flow and layers by density. Every other rule tests each component on its own: contact, conductivity, flammability and reactions. A cell's display name is its largest component's name, prefixed by the adjective (`loc.<liq>.adj`) of the second largest when that holds at least 10% of the total ("bloody water"). Drinking a mixture applies each component's drink row scaled by its share (§3.2). A brewing base must be a single component.

### 2.4 Burning liquids

Doc 03 §8 runs liquid fires: a cell holding a `fire_fuel` liquid ignites from §5.2 triggers or spread (chance from `flammability_bp`), burns 5 units of fuel per fire run, emits smoke and heat, and gives `st.burning` buildup to bodies in it. Values: oil 3000, alcohol 5000, fuel 8000, tar 2000, sap 1000. Ignition blasts per unit are in §5.2.

### 2.5 Payload patches

A payload lets a potion's effects ride on a puddle (Linger, §10.6) or a cloud (Aerosol and vaporizers, §10.4). The grid is untouched; the payload is a patch around an origin.

```rust
pub struct Payload {
    pub id: u16,                                   // 1..=65535 from a per-Folio counter, never 0
    pub effects: ArrayVec<PayloadEffect, 4>,       // {eff: u32, mag: i64, dur: u32, potency_bp: i32}
    pub src_lvl: i32, pub quality_bp: i32, pub buc: u32,
    pub creator: u64, pub origin_genre: u32,
    pub route: PayloadRoute,                       // Surface { liq: u32 } or Vapor
    pub origin: Gidx, pub radius_mm: i32,          // LINGER_RADIUS_MM or VAPOR_RADIUS_MM
    pub expires_tick: u64,                         // t + LINGER_TICKS or t + VAPOR_TICKS
}
```

- Stored in `BTreeMap<u16, Payload>` per Folio; creating one beyond `PAYLOAD_MAX_ACTIVE` first expires the lowest ID.
- **Surface exposure**: the entity has contact with the patch's `liq` (doc 03 §5.20) and its feet cell center lies within `radius_mm` of `origin`. **Vapor exposure**: the eye cell holds `gas.alch_vapor` at `GAS_CONTACT_CONC` or more and lies within `radius_mm`. When several patches qualify, the one with the nearest origin applies (`dist2`, ties to lower ID).
- `potency_bp` is the Contradiction potency snapshot at creation (§9.9), like Canon §11.3's applier snapshot.

### 2.6 Worked example: a thrown liquid-flame flask

A `alch.potion.liquid_flame` flask (250 ml oil base) shatters on open floor on Folio 5. Its spill enters the grid as `ceil_div(250, 1000) = 1` unit through doc 03 `splash`, which puts `max(1, floor_div(1, 9)) = 1` unit of oil in each of the 9 floor cells around the impact. The impact is a fire trigger (§5.2): doc 03 blasts the impact cell for `10,000 × min(1, 5) = 10,000` mHP base, which an Erratum within 2 m takes as `D2 = mul_bp(10000, T(5) = 12388) = 12,388` mHP before resistance, and consumes that cell's unit. The 8 other oily cells then catch through fire spread (`flammability_bp` 3000). Separately, the potion's own splash applies `alch.eff.burn` to everyone within 2000 mm (§10.2).

---

## 3. Liquid catalog

### 3.1 Physical properties

35 liquids. Columns follow the §2.3 schema. "Freezes" and "Boils" give the threshold in DeciC and the product. Flammable modes: `ignite` (burns at the listed ml per liquids step), `explosive` (§5.2 detonation). Vessel flags: `corrosive` destroys leather, wood and polymer vessels; `molten` destroys every vessel except `apparatus.vessel.crucible`; `cryogenic` cracks glass and ceramic (§6.3). `conceal_full` is the Canon §12.2 conceal of a full cell, scaled linearly by fill: `conceal = floor_div(conceal_full × total, CELL_CAP_ML)`.

|ID|Grp|Dens g/L|spread_bp|film_ml|fall_bp|Cond|Flammable (burn ml/step)|Temp dC|Freezes|Boils|Color|Light|Conceal|Evap ml/step|Evap gas (ml per ml)|Dep|Vessel flag|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|`liq.water`|aq|1000|10000|2000|10000|yes|none|150|0>ice|1000>gas.steam|#3A7BD5|0|2|2|-|none|-|
|`liq.brine`|aq|1025|10000|2000|10000|yes|none|150|-210>ice|1010>gas.steam|#4E8FA8|0|2|2|-|none|-|
|`liq.oil`|oily|900|6000|4000|8000|no|ignite 400|150|-|-|#8C6D1F|0|3|0|-|none|-|
|`liq.alcohol`|aq|790|10000|2000|10000|yes|ignite 600|150|-|780>gas.fuel_vapor|#E8E4D0|0|1|4|-|none|-|
|`liq.acid`|aq|1300|9000|2000|10000|yes|none|150|-300>ice|3300>gas.caustic|#9AE13B|0|2|1|gas.caustic 100|none|corrosive|
|`liq.lava`|molten|2600|2500|50000|3000|no|none|12000|7000>block.basalt|-|#FF5A00|15|15|0|-|none|molten|
|`liq.red_ink`|aq|1100|8000|3000|10000|no|none|150|-|-|#C8102E|2|12|0|-|none|-|
|`liq.blood`|aq|1060|8000|3000|9000|yes|none|370|-5>ice|1000>gas.steam|#8A0303|0|4|1|-|none|-|
|`liq.sepia`|aq|1050|9000|2000|10000|no|none|150|-20>ice|1000>gas.steam|#3B2A1A|0|10|1|-|none|-|
|`liq.honey`|aq|1420|1500|20000|2500|no|none|150|-|-|#E8A317|0|6|0|-|none|-|
|`liq.slime`|gel|1100|3000|10000|5000|no|none|150|-50>ice|-|#7FD957|1|5|1|-|none|-|
|`liq.tar`|oily|1200|1000|30000|2000|no|ignite 150|150|-|-|#1C1C1C|0|15|0|-|none|-|
|`liq.mud`|aq|1700|3000|15000|5000|yes|none|150|-10>ice|-|#6B4F2A|0|8|1|-|none|-|
|`liq.wine`|aq|990|10000|2000|10000|yes|none|150|-50>ice|900>gas.steam|#6D0E2A|0|4|2|-|none|-|
|`liq.milk`|aq|1030|9500|2000|10000|yes|none|50|-5>ice|1000>gas.steam|#F2EFE6|0|6|2|-|none|-|
|`liq.holy_water`|aq|1000|10000|2000|10000|yes|none|150|0>ice|1000>gas.incense|#CFE8FF|3|2|1|-|mythos|-|
|`liq.unholy_water`|aq|1000|10000|2000|10000|yes|none|50|0>ice|1000>gas.miasma|#3A2F4A|0|4|1|-|mythos|-|
|`liq.quicksilver`|metal|13534|7000|1000|10000|yes|none|150|-|-|#C0C6CC|0|15|0|-|none|-|
|`liq.molten_metal`|molten|7000|3000|30000|4000|yes|none|15000|12000>block.slag|-|#FFB347|14|15|0|-|none|molten|
|`liq.coolant`|cryo|810|10000|2000|10000|no|none|-1960|-|-1950>gas.cold_fog|#BEEFFF|0|2|20|gas.cold_fog 500|none|cryogenic|
|`liq.ichor`|aq|1050|8000|3000|9000|no|none|370|-|-|#FFD700|6|6|0|-|mythos|-|
|`liq.ectoplasm`|ether|600|5000|5000|3000|no|none|50|-|-|#B8FFE0|5|3|3|-|mythos|-|
|`liq.venom`|aq|1050|9000|1000|10000|yes|none|150|-20>ice|-|#9C2DB5|0|4|1|-|none|-|
|`liq.bile`|aq|1100|7000|3000|9000|yes|none|300|-30>ice|1000>gas.miasma|#5C6B2A|0|6|2|gas.miasma 200|none|-|
|`liq.swamp_water`|aq|1010|9000|2000|10000|yes|none|200|0>ice|1000>gas.steam|#4A5D23|0|4|2|gas.miasma 50|none|-|
|`liq.nitro`|oily|1600|8000|1000|10000|no|explosive|150|-|-|#F5F0C8|0|2|0|-|none|-|
|`liq.fuel`|oily|740|10000|2000|10000|no|ignite 800|150|-|950>gas.fuel_vapor|#D4B483|0|2|6|gas.fuel_vapor 300|none|-|
|`liq.mana`|ether|900|10000|2000|10000|no|none|150|-|-|#3FE0D0|9|3|3|-|arcana|-|
|`liq.nanite_slurry`|aq|1800|6000|5000|8000|yes|none|150|-|-|#7A8B99|1|8|0|-|techne|-|
|`liq.rad_sludge`|aq|1400|4000|8000|6000|yes|none|300|-|-|#7CFC00|8|8|1|gas.fallout 100|none|-|
|`liq.phosphor`|aq|1000|9000|2000|10000|yes|none|150|0>ice|1000>gas.steam|#C6FF6B|10|3|2|-|none|-|
|`liq.sap`|gel|1150|2000|15000|3000|no|ignite 100|150|-|-|#C08A2D|0|6|0|-|none|-|
|`liq.hemolymph`|aq|1040|9000|2000|10000|yes|none|200|-10>ice|1000>gas.steam|#7FBF3F|0|4|1|-|none|-|
|`liq.wellspring`|aq|1000|10000|2000|10000|yes|none|120|0>ice|1000>gas.steam|#9F7FFF|4|2|0|-|none|-|
|`liq.nectar`|aq|1300|3000|8000|5000|no|none|150|-|-|#FFB7D5|3|6|1|-|arcana|-|

Additional flags: `liq.coolant` has `evap_any_volume = true`. Ignition blasts (§5.2): `liq.oil` and `liq.alcohol` 10 HP per 1000 ml (Canon §12.1), `liq.fuel` 15 HP per 1000 ml; `liq.tar` and `liq.sap` burn without a blast. Liquids with `light > 0` are `light_block` sources (doc 03): full `light` when the cell holds at least 100,000 ml, `light − 4` (minimum 0) from 1,000 ml, none below.

### 3.2 Contact and drink rows

Contact rows apply per status pulse (§5.1). Drink rows are per 250 ml; other amounts scale numbers by `floor_div(ml × value, 250)`, and removal rules need at least 125 ml. "Hazard N HP" enters Canon §10.6 at step 3 with `D2 = mul_bp(N × 1000, T(d))` against Errata and `P(d)` against NPCs. Buildup is environmental (§5.1). "Refused": the drink action is not offered.

|Liquid|Contact (per status pulse)|Drink (per 250 ml)|
|---|---|---|
|`liq.water`|`st.soaked` (Canon)|toxicity −2,000 (at most once per 600 ticks per drinker)|
|`liq.brine`|`st.soaked`|toxicity +3,000|
|`liq.oil`|`st.oiled`; ground friction × 3000 (Canon)|toxicity +10,000|
|`liq.alcohol`|`st.oiled` (Canon)|composure +1000 Bp of max; 20 `st.confused` buildup; toxicity +15,000|
|`liq.acid`|25 `st.corroded` buildup (Canon)|hazard 15 HP `dmg.acid`; toxicity +5,000|
|`liq.lava`|DoT rate 1000 Bp as `dmg.fire`, plus 100 `st.burning` buildup (Canon)|refused|
|`liq.red_ink`|+1 `st.erasure` per 300 ticks of contact (Canon; §3.4)|one `afflict.red_ink` dose; toxicity +5,000|
|`liq.blood`|none|toxicity +5,000, satiation +50; with `afflict.hunger`: heal 500 Bp, satiation +200, no toxicity|
|`liq.sepia`|ends `st.invisible`|toxicity +4,000|
|`liq.honey`|`st.slowed`|heal 300 Bp; satiation +150|
|`liq.slime`|ground friction × 4000|toxicity +6,000; satiation +50|
|`liq.tar`|`st.slowed`|refused|
|`liq.mud`|`st.slowed`; ends `st.burning`|toxicity +4,000; satiation +20|
|`liq.wine`|`st.soaked`|composure +1500 Bp of max; toxicity +6,000; satiation +50|
|`liq.milk`|`st.soaked`|satiation +100; removes 1 stack of `st.poisoned`|
|`liq.holy_water`|`st.soaked`; bearers tagged undead or fiend: DoT rate 300 Bp as `dmg.radiant`|ends `st.withering` and `st.dread`; undead or fiend drinker: hazard 20 HP `dmg.radiant`|
|`liq.unholy_water`|`st.soaked`; bearers not tagged undead or fiend: 15 `st.withering` buildup; undead or fiend: `st.regenerating`|living drinker: 40 `st.withering` buildup; undead or fiend: heal 1500 Bp|
|`liq.quicksilver`|10 `st.poisoned` buildup|60 `st.poisoned` buildup; toxicity +10,000|
|`liq.molten_metal`|DoT rate 1500 Bp as `dmg.fire`, plus 150 `st.burning` buildup|refused|
|`liq.coolant`|`st.chilled`; 30 `st.frozen` buildup; DoT rate 200 Bp as `dmg.frost`|refused|
|`liq.ichor`|bearers tagged divine: `st.regenerating`|heal 1500 Bp; toxicity +5,000|
|`liq.ectoplasm`|15 `st.dread` buildup|`st.invisible` for 120 ticks; 30 `st.dread` buildup|
|`liq.venom`|none (harmless on skin)|toxicity +5,000 (harmless orally; dangerous only on weapons, §13.1)|
|`liq.bile`|15 `st.diseased` buildup|40 `st.diseased` buildup; toxicity +8,000|
|`liq.swamp_water`|`st.soaked`; 5 `st.diseased` buildup|20 `st.diseased` buildup; toxicity +3,000|
|`liq.nitro`|none|stamina +3000 Bp of max; toxicity +20,000|
|`liq.fuel`|`st.oiled`|toxicity +20,000|
|`liq.mana`|aether +500 m-aether|aether +2500 Bp of max; toxicity +8,000|
|`liq.nanite_slurry`|`body.construct` or `kin.construct`: heal at rate 100 Bp of Max HP per s (accumulator `k = 4`); others: 10 `st.exposed` buildup|constructs: heal 2000 Bp; others: heal 1000 Bp, toxicity +12,000|
|`liq.rad_sludge`|20 `st.irradiated` buildup|60 `st.irradiated` buildup; toxicity +15,000|
|`liq.phosphor`|bearer counts as lit: `light_at(e) = max(light_at(e), 9)` for stealth until 600 ticks after the last contact pulse|dark-vision +2000 mm for 1800 ticks; toxicity +5,000|
|`liq.sap`|`st.slowed`|heal 200 Bp; satiation +80|
|`liq.hemolymph`|none|toxicity +3,000; satiation +40|
|`liq.wellspring`|`st.soaked`|one wellspring roll (§12.5)|
|`liq.nectar`|none|aether +1500 Bp of max; satiation +100|

Rows of liquids with a dependency in §3.1 are environmental, like every surface: they always apply at potency 10000 and never Contradict.

### 3.3 `liq.blood` and bleeding

- **Blood liquid by body.** Each creature template has `blood_liq`, default `liq.blood`. Defaults by tag: `body.construct` → `liq.oil`; `body.insectoid` → `liq.hemolymph`; `body.amorphous` → `liq.slime`; tag undead → `liq.bile`; tag divine → `liq.ichor`; tag eldritch → `liq.ectoplasm`; tag plant → `liq.sap`; `fac.editors` members → `liq.red_ink`; swarms and spirits → none. Errata use their kin: human, beastkin and mutant → `liq.blood`; fae → `liq.nectar`; construct → `liq.oil`; undead → `liq.bile`.
- **Bleeding drip.** On each status pulse an entity with `st.bleeding` adds `25 × stacks` ml of its `blood_liq` to its feet cell.
- **Death spill.** On `ev.entity_killed`, the corpse adds `min(20000, floor_div(body_mass_g, 50))` ml of `blood_liq` to its feet cell. A 75,000 g humanoid spills 1,500 ml.
- **Severing** (Canon §9.1) adds 500 ml at the part's position.
- Canon §12.1 makes blood conductive and freezable. Blood has no contact row. It dries at 1 ml per step (evaporation, no gas).
- Blood is a brewing base (§9.2) and the drink of `afflict.hunger` bearers.

### 3.4 `liq.red_ink`

- **Sources.** Every `fac.editors` member bleeds red ink. Hunters (`rank.hunter`) spill 5,000 ml on death; Editor bosses spill 50,000 ml. Doc 12 places red-ink pools in `genre.redacted` and in Editor arenas.
- **Erasure contact.** Each entity keeps `red_ink_contact_ticks: u32`. On every status pulse in contact, add 15; when it reaches 300, subtract 300 and grant one `st.erasure` stack (emit `ev.erasure_stack`). After 600 ticks without contact it resets to 0. `fac.editors` members are immune.
- **Unwriting.** A cell holding at least 1,000 ml of red ink erases one adjacent (6-neighbor) block that an Erratum placed, every 120 ticks per cell, lowest local index first (the same rule Canon §11.6 gives Editors).
- **Cleansing.** `dmg.radiant` on red ink turns it to water (Canon §12.1). Holy water reacts with it 1:1 into water (§5.3).
- Red ink never evaporates, never freezes, never boils, does not conduct and does not burn. It is the base liquid of forbidden draughts (§9.2, §14).

---

## 4. Gases

### 4.1 Gas cell state

```rust
pub struct GasComp { pub gas: u32 /* id32 of gas.* */, pub payload: u16, pub ml: i64 }
pub struct GasCell { pub comps: ArrayVec<GasComp, 3> }   // sorted by (gas, payload)
```

- Gas room in a cell: `CELL_CAP_ML − total_liquid(cell) − Σ gas ml`. A cell full of liquid holds no gas; solid cells hold none.
- **Density** of a component: `density_bp = floor_div(ml × 10000, CELL_CAP_ML)`.
- **Holds** (contact and breathing): the eye cell's component has at least `GAS_CONTACT_ML`.
- **Concealment** (Canon §12.2): `conceal(cell) = min(15, Σ floor_div(conceal_full × ml, CELL_CAP_ML))` over gas components, plus the liquid conceal of §3.1. Canon gives smoke 4 at full density scaled linearly and steam 3; this chapter scales every gas linearly.

### 4.2 Gas catalog

Route: `inhaled` effects are blocked by a working respirator (§4.5) or purify immunity; `skin` effects are not; `sight` only conceals. Buoyancy moves `buoy_bp` of the component up (rise) or down (sink) per step. Decay per step: `max(decay_min_ml, mul_bp(ml, decay_bp))`, then × `gas_dissipation_bp`.

|ID|Route|Conceal|Buoy|buoy_bp|diffuse_bp|decay_bp|decay_min_ml|Breathable|Contact (per status pulse while held)|Fire trigger|Color|
|---|---|---|---|---|---|---|---|---|---|---|---|
|`gas.steam`|skin|3|rise|5000|8000|300|2000|yes|DoT rate 50 Bp as `dmg.fire` (Canon)|none|#E6EEF2|
|`gas.miasma`|inhaled|1|none|0|6000|50|500|no (Canon)|20 `st.poisoned` buildup (Canon)|3 m blast, base 20 HP `dmg.fire` (Canon); cell converts to 50% `gas.smoke`|#6B8E23|
|`gas.smoke`|sight|4|rise|3000|7000|60|200|yes|sight concealment only (Canon)|none|#5A5A5A|
|`gas.fuel_vapor`|inhaled|0|sink|3000|5000|80|500|yes|none|2 m blast, base `floor_div(ml, 25000)` HP `dmg.fire` (40 HP at full density); consumed|#D9CFA0|
|`gas.caustic`|skin|1|sink|3000|5000|100|1000|yes|15 `st.corroded` buildup|none|#B5E655|
|`gas.soporific`|inhaled|1|none|0|5000|120|1000|yes|25 `st.sleeping` buildup|none|#C9B8E8|
|`gas.cold_fog`|skin|2|sink|4000|6000|200|2000|yes|`st.chilled`; 10 `st.frozen` buildup|consumed, becomes 50% `gas.steam`|#DDF6FF|
|`gas.spore`|inhaled|2|none|0|5000|60|500|yes|15 `st.diseased` buildup|consumed, becomes 50% `gas.smoke`|#A89F68|
|`gas.laughing`|inhaled|1|none|0|6000|120|1000|yes|20 `st.confused` buildup|none|#F2C6DE|
|`gas.terror`|inhaled|1|none|0|6000|120|1000|yes|20 `st.feared` buildup|none|#4B3B5C|
|`gas.incense`|inhaled|2|rise|2000|5000|80|500|yes|tagged undead or fiend: DoT rate 100 Bp as `dmg.radiant`; others: composure +1 per s (accumulator `k = 4`), ignoring the Canon §12 light and combat conditions|none|#F5E6C4|
|`gas.fallout`|inhaled|1|sink|2000|4000|30|300|yes|10 `st.irradiated` buildup|none|#9DB36B|
|`gas.alch_vapor`|inhaled|1|none|0|6000|100|1000|yes|payload exposure (§10.4)|none|payload color|

Gas rows are environmental and never Contradict. Fire triggers on gases are §5.2 triggers.

### 4.3 The gases step

Runs on ticks where `(t + 4) % 6 == 0`. Each phase is double-buffered and processes each gas component independently (partial pressures), clipping into the target's room as in §2.4.

```rust
fn gases_step(t: Tick) {
    for each cell, component c:                                   // phase 1: decay
        c.ml -= min(c.ml, mul_bp(max(decay_min_ml(c), mul_bp(c.ml, decay_bp(c))), gas_dissipation_bp));
    for each cell, component c with buoy != none:                 // phase 2: buoyancy
        target = cell above (rise) or below (sink); if !solid(target):
            req = mul_bp(c.ml, buoy_bp(c));                        // clipped by target room
    for each cell a, component c, each of 6 neighbors n, not solid:  // phase 3: diffusion
        if c.ml(a) > c.ml(n): req = mul_bp(floor_div(c.ml(a) - c.ml(n), 7), diffuse_bp(c));
    cleanup: delete components < GAS_TRACE_ML; expire payloads (§2.9); enforce MAX_GAS_COMPONENTS
}
```

A new gas component entering a cell that already has 3 components is refused (it stays in its source), with the same ordering rule as liquids.

### 4.4 Gas injection

`inject(cell, gas, payload, ml)` fills the origin cell to its room, then visits non-solid cells breadth-first over 6-neighbors (doc 03 order), up first when the gas rises and down first when it sinks, filling each to its room, for at most 27 cells. Any remainder is discarded. Sources: boiling (§2.6), evaporation gases, fire triggers on water (§5.2), burning cells (§2.7), doc 03 burning blocks (50,000 ml `gas.smoke` per burning block per fire step), bombs (§13), aerosol potions (§10.4) and enemy abilities (doc 13).

**Worked example.** Fire hits a cell holding 3,000 ml of water: all 3,000 ml boils into 3,000,000 ml of steam, filling the origin cell, the cell above, then the next visited cell. An 85 HP Erratum whose eye cell is full of steam loses `mul_bp(85000, 50) = 425` mHP per second; that cell's conceal is 3.

### 4.5 Breathing and respirators

- `res.breath` (Canon §12, owner 04) drains while the eye is submerged (§2.2) or the eye cell holds an unbreathable gas (`gas.miasma`).
- **Respirators** are `slot.face` items, archetype `apparatus.respirator.*`:

|ID|Filter (ticks)|Mass g|Hearing|Dep|Skins|
|---|---|---|---|---|---|
|`apparatus.respirator.cloth`|1800|200|0 dB|none|plague-doctor beak (gothic_horror), herb veil (fairy_tale), rag mask (post_apocalypse), silk face-cloth (wuxia)|
|`apparatus.respirator.mask`|3600|600|−3 dB|none|gas mask (steampunk, weird_west, hardboiled_noir), filter hood (cosmic_horror)|
|`apparatus.respirator.rebreather`|7200|900|−5 dB|techne|rebreather (cyberpunk, space_opera), breath-gill (redacted)|

- A respirator with `filter > 0` makes every gas breathable (Canon §12) and blocks every `inhaled` contact row and inhaled payload exposure. It does not block `skin` rows (steam, caustic, cold fog), sight concealment, or liquid contact.
- The filter loses 1 per tick while the eye cell holds any `inhaled` gas, and 2 per tick for a rebreather while the eye is submerged. At 0 the respirator stops working. Using `ing.charcoal` (1 unit; 2 for a rebreather, 60 ticks) refills it to full.
- The rebreather also stops `res.breath` from draining while the eye is submerged and `filter > 0`.
- Respirators are not disguise masks (doc 09). `alch.eff.purify` (§7) grants the inhaled immunity without a respirator.

---

## 5. Surfaces and reactions (Canon §12.1, implemented and extended)

Docs 03, 05, 06 and 08 implement this one table. Every row of Canon §12.1 appears here unchanged and is marked (Canon). Every trigger that changes a cell emits `ev.liquid_reaction { cell, trigger, inputs, outputs }`.

### 5.1 Contact evaluation

On each status pulse (Canon §2.2), in ascending `EntityId`, for each entity: collect the distinct liquid components (`CONTACT_ML` or more) over its feet and body cells and the gas components held by its eye cell; apply each component's contact row once, in ascending `(id32, payload)`; then apply cell-state rows; then status triggers (§5.5).

|Cell state or block|Contact effect|
|---|---|
|liquid component|its §3.2 contact row|
|gas component|its §4.2 contact row; `inhaled` rows apply only to a bearer without a working respirator or purify immunity|
|`block.ice` or frozen film|ground acceleration and friction × 1500 (Canon)|
|`block.web`|`st.rooted` while inside (Canon)|
|electrified cell|30 `st.shocked` buildup (Canon)|
|burning cell|80 `st.burning` buildup|
|payload component|payload exposure (§10.6)|

- DoT rows use Canon §11.3 with `applier_potency_snapshot = 10000` and the bearer's `rank_dot_bp`; they enter at step 4 with flag `dot`.
- Buildup rows are environmental: `p' = mul_bp(mul_bp(p, S(d_current)), 10000 − r)`, where `r` is the bearer's clamped resistance to the damage type that builds the status (Canon §10.1: toxin for poisoned, acid for corroded, fire for burning, shock for shocked, frost for frozen, radiation for irradiated, umbral for withering, psychic for dread; mind statuses without a type use `r = 0`). Immune types skip the row.
- Direct statuses in contact rows (`st.soaked`, `st.oiled`, `st.slowed`, `st.chilled`, `st.regenerating`) use their Canon base duration with Canon §11.4 scaling, refreshed each pulse.

### 5.2 Damage-type triggers

A hit triggers type `X` when its type share for `X` is at least 2500 Bp (DoT pulses never trigger). **Trigger cells:** for a hit on an entity, its feet cell; for an impact on a block or the ground, the non-solid cell on the attacker's side of the struck face; for area damage, every non-solid cell whose center lies within the area radius, at most 125 cells, nearest first by `dist2`, ties to lower cell order.

|Trigger|Target (component in a trigger cell)|Result|
|---|---|---|
|Shock|`liq.water`, `liq.blood`, `liq.alcohol` (Canon) and every other conductive liquid (§3.1)|electrifies the connected body (§2.8): at most 64 cells for 180 ticks; contact gives 30 `st.shocked` buildup per pulse (Canon)|
|Shock|`liq.nitro`|detonation (§5.6)|
|Frost|`liq.water`, `liq.blood`, `liq.alcohol` (Canon)|the cell becomes `block.ice` for 1200 ticks (Canon), by the deep or film rule of §2.8|
|Frost|every other liquid with a freeze entry in §3.1 except `liq.brine`|as the row above|
|Frost|`liq.lava` or `liq.molten_metal`, cell total at most 50,000 ml|becomes `block.basalt` or `block.slag` (permanent)|
|Frost|`gas.steam`|condenses: the component is removed and 1 ml of `liq.water` per 1,000 ml is added to the cell|
|Fire|`liq.water` (Canon)|becomes `gas.steam`, 1 cell per 1000 ml (Canon): up to `STEAM_CONVERT_CAP_ML` boils at `STEAM_PER_ML`|
|Fire|every other non-flammable aqueous liquid with a boil entry|its boil gas at the same rate (`liq.holy_water` → `gas.incense`, `liq.unholy_water` and `liq.bile` → `gas.miasma`, `liq.acid` → `gas.caustic`, the rest → `gas.steam`)|
|Fire|`liq.oil` or `liq.alcohol` (Canon)|ignites; a 2 m environmental `dmg.fire` blast of base 10 HP per 1000 ml, capped at 5000 ml (Canon); the cell then burns (§2.7)|
|Fire|`liq.fuel`|ignites; 2 m blast of base 15 HP per 1000 ml, capped at 5000 ml; burns|
|Fire|`liq.tar`, `liq.sap`|ignites without a blast; burns|
|Fire|`liq.nitro`|detonation (§5.6)|
|Fire|`liq.coolant`|all coolant in the cell flashes to `gas.cold_fog` at 500 ml per ml|
|Fire|`liq.ectoplasm`|removed|
|Fire|`block.ice` or frozen film (Canon: ice)|melts back to water (Canon); an ice record restores its stored liquid (§2.8)|
|Fire|`block.web` (Canon)|burns out in 60 ticks|
|Fire|`gas.miasma` (Canon)|a 3 m environmental `dmg.fire` blast of base 20 HP (Canon); the component becomes 50% `gas.smoke`|
|Fire|`gas.fuel_vapor`|2 m blast of base `floor_div(ml, 25000)` HP; the component is consumed|
|Fire|`gas.spore`, `gas.cold_fog`|§4.2 fire column|
|Radiant|`liq.red_ink` (Canon)|becomes water (equal volume)|
|Radiant|`liq.unholy_water`|becomes water|
|Radiant|`liq.ectoplasm`, `gas.spore`|removed|
|Umbral|`liq.holy_water`|becomes water|
|Arcane|`liq.mana`|arcane surge: 2 m environmental `dmg.arcane` blast of base 8 HP per 1000 ml, capped at 5000 ml; the mana is consumed|
|Blunt, pierce or slash with `D2 >= 1000` mHP; any sonic|`liq.nitro`|detonation (§5.6)|

Every blast above enters the damage pipeline as an environmental hazard (Canon §12.1, §10.6).

### 5.3 Liquid and liquid reactions (`alch.rx.*`)

Phase A, per cell, in ascending `priority` then ascending `id32`. `units = min(floor_div(in1, r1), floor_div(in2, r2), max_units)`; when `units >= 1`, consume `units × r` of each non-catalyst input and add every output × `units`. A `[aq_boil]` input matches any non-flammable aqueous liquid with a boil entry, and its gas output is that liquid's boil gas. Lava or molten metal sharing a cell with at least 100 ml of a flammable component is a fire trigger on that cell every step.

```json
{ "id": "alch.rx.water_lava", "priority": 10,
  "in": [{ "tag": "aq_boil", "ratio_ml": 1000 }, { "liq": "liq.lava", "ratio_ml": 50000 }],
  "max_units": 20,
  "out": [{ "gas": "boil_gas_of_in1", "ml": 1000000 }, { "block_if_consumed": "block.obsidian", "min_ml_of_in2": 50000 }] }
```

|ID|Pri|Inputs (ratio ml)|Max units|Outputs per unit|
|---|---|---|---|---|
|`alch.rx.water_lava`|10|`[aq_boil]` 1000 + lava 50,000|20|1,000,000 ml boil gas; the cell becomes `block.obsidian` when at least 50,000 ml lava was consumed this step|
|`alch.rx.coolant_lava`|11|coolant 2000 + lava 50,000|20|cell becomes `block.obsidian`; 500,000 ml `gas.steam`|
|`alch.rx.metal_water`|12|molten_metal 10,000 + `[aq_boil]` 1000|20|cell becomes `block.slag`; 1,000,000 ml boil gas|
|`alch.rx.coolant_water`|13|coolant 1000 + water 4000 (catalyst)|1|frost trigger on the cell (§5.2); 500,000 ml `gas.cold_fog`|
|`alch.rx.holy_red_ink`|20|holy_water 1 + red_ink 1|10,000|2 ml water|
|`alch.rx.holy_unholy`|21|holy_water 1 + unholy_water 1|10,000|2 ml water|
|`alch.rx.ichor_unholy`|22|ichor 1 + unholy_water 1|10,000|2 ml holy_water|
|`alch.rx.holy_ectoplasm`|23|holy_water 1000 + ectoplasm 1000|20|500,000 ml `gas.incense`|
|`alch.rx.red_ink_blood`|30|red_ink 1000 (catalyst) + blood 1000|20|none (blood is erased; red ink unchanged)|
|`alch.rx.bile_water`|40|bile 1000 + water 4000|20|5000 ml swamp_water|
|`alch.rx.acid_slime`|41|acid 1000 + slime 1000|20|2000 ml water|
|`alch.rx.acid_milk`|42|acid 1000 + milk 1000|20|2000 ml slime|
|`alch.rx.venom_milk`|43|venom 1000 + milk 1000|20|1000 ml milk|
|`alch.rx.honey_alcohol`|44|honey 1000 + alcohol 1000|20|2000 ml wine|
|`alch.rx.mana_water`|45|mana 1000 + water 1000|20|2000 ml water (mana is lost when diluted)|
|`alch.rx.nanite_rad`|46|nanite_slurry 1000 (catalyst) + rad_sludge 1000|20|1000 ml water|

Oily, metal and cryo liquids do not react with water except as listed; they share the cell as separate components. When a reaction solidifies its cell, its gas outputs inject starting at the cell above.

**Worked example.** A cell holds 6,000 ml water and 120,000 ml lava. `units = min(floor_div(6000, 1000), floor_div(120000, 50000), 20) = 2`. The step consumes 2,000 ml water and 100,000 ml lava, injects 2,000,000 ml steam, and the cell becomes `block.obsidian` because 100,000 ≥ 50,000; phase D pushes the remaining 4,000 ml water and 20,000 ml lava to the cell above.

### 5.4 Liquid and block reactions

Phase A, after liquid reactions, for each cell holding the listed component, against its 4 side neighbors and the block below:

|Liquid (min ml)|Block condition|Result per step|
|---|---|---|
|`liq.acid` (1,000)|material tag `acid_soluble` (doc 07: wood, cloth, paper, flesh, leaves)|1 block damage; 200 ml acid consumed per damaged block|
|`liq.acid` (100)|`block.web`|web removed; 100 ml consumed|
|`liq.lava`, `liq.molten_metal` (1,000)|material tag `flammable`|fire trigger on the block (doc 03 ignition)|
|`liq.lava`, `liq.molten_metal` (1,000)|`block.ice`|melts (§2.8)|
|any aqueous (1,000)|a burning block (doc 03)|extinguished; 500 ml consumed; 500,000 ml `gas.steam` injected|
|`liq.red_ink` (1,000)|a block placed by an Erratum|erased every 120 ticks (§3.4)|
|any aqueous|material tag `absorbent` below|100 ml absorbed (§2.6)|

### 5.5 Status-driven triggers

Evaluated after contact rows, per entity:

- `st.burning` bearer with at least 100 ml of a flammable component in its feet or body cells: fire trigger on that cell.
- `st.oiled` bearer gaining `st.burning`: `st.oiled` ends and the feet cell takes a fire trigger.
- `st.shocked` bearer in contact with a conductive component: each micro-stun (every 30 active ticks, Canon §11.1) is a shock trigger on its feet cell.
- `st.soaked` (applied by aqueous contact) ends `st.burning` (Canon §11.1).

### 5.6 Blast resolution and detonation

- Blasts queue during a step and resolve at its end in ascending origin-cell order.
- Each blast hits every entity whose capsule center lies within its radius and has line of effect (doc 05). It enters Canon §10.6 at step 3 with `D2 = mul_bp(base_mhp, T(d_current))` against Errata and `mul_bp(base_mhp, P(d_current))` against NPCs; crits are skipped.
- **Merge rule.** Within one step, an entity takes only the largest queued liquid or gas blast (largest `base_mhp`, ties to lower origin cell), then ignores liquid and gas blasts for `BLAST_IMMUNITY_TICKS`.
- Each blast is a fire trigger (or arcane trigger for an arcane surge) on every cell within its radius on the next step, which chains ignitions and miasma blasts at one ring per step.
- **Detonation** (`liq.nitro`): radius 3000 mm, base 300 HP per 1000 ml capped at 1000 ml, shares `dmg.blunt` 7000 and `dmg.fire` 3000, block-break power `1 + floor_div(ml, 250)` (doc 03 explosion rules), all nitro in the cell is consumed, and the blast is a shock trigger on every nitro cell within radius.

**Worked example.** Fire hits a 2,500 ml oil puddle on Folio 5. Base = `10 × 2500 = 25,000` mHP. An Erratum within 2 m takes `D2 = mul_bp(25000, T(5) = 12388) = 30,970` mHP before resistance; an NPC takes `mul_bp(25000, P(5) = 14115) = 35,287` mHP. The cell then burns 400 ml per step for 7 steps (6 × 400 + 100), injecting 50,000 ml smoke per step.

---

## 6. Containers

### 6.1 Vessels (`cat.container`)

A filled vessel holding a brewed or found potion is a `cat.potion` (or, for grenade shells, `cat.throwable`) item; an empty or raw-filled vessel is `cat.container`. Vessels hold up to 4 components, like cells. Dose volume = capacity ÷ doses.

|ID|Capacity ml|Doses|Empty mass g|Material|Shatters when thrown|Splash radius mm|Deliveries|Skins|
|---|---|---|---|---|---|---|---|---|
|`apparatus.vessel.vial`|100|1|60|glass|yes|1000|drink, throw, coat|phial, ampul-vial, pill-gourd|
|`apparatus.vessel.flask`|250|1|150|glass|yes|2000|drink, throw, coat|flask, stim-cartridge, pill jar, dewdrop bead|
|`apparatus.vessel.bottle`|750|3|400|glass|yes|2500|drink, throw|bottle, canteen-jet, wine jar|
|`apparatus.vessel.ampoule`|50|1|30|polymer|no throw|0|inject, drink|auto-injector (cyberpunk), hypo (space_opera), acupuncture needle (wuxia), bee-sting lancet (fairy_tale)|
|`apparatus.vessel.grenade_shell`|300|1|200|ceramic|yes|3000|throw|clay pot, thunder pellet, tin grenade, glass orb|
|`apparatus.vessel.waterskin`|2000|8|250|leather|no|0|drink, pour|waterskin, hydration pack, gourd|
|`apparatus.vessel.canteen`|1000|4|300|metal|no|0|drink, pour|canteen, flask-tin|
|`apparatus.vessel.bucket`|10000|0|1500|wood|no|0|pour, dump|bucket, pail, jerrycan|
|`apparatus.vessel.crucible`|1000|0|1200|ceramic|no|0|pour|crucible, ladle|
|`apparatus.vessel.dewar`|500|2|800|metal|no|0|drink, pour|dewar, cryo-flask|
|`apparatus.vessel.cask`|50000|0|15000|wood|no|0|pour; placeable as a block entity|cask, drum, amphora|

Mass of a filled vessel: `empty_mass + Σ floor_div(c.ml × density_g_per_l, 1000)`. A water flask weighs `150 + 250 = 400` g.

### 6.2 Filling, pouring and compatibility

- **Fill** (interact with a liquid cell, 30 ticks): takes `min(room, total(cell))` from the cell by the §2.4 component split. Filling from a burning or electrified cell is refused.
- **Pour** (30 ticks): moves the contents to the cell in front (or the feet cell) and lets the automaton take over. **Dump** (buckets and casks, 20 ticks): splits the contents over the 3 × 3 cells centered in front, `floor_div(v, 9)` each, remainder to the center.
- **Compatibility.** A vessel holding a component whose flag conflicts with its material is destroyed and spills (§6.3): `corrosive` in leather, wood or polymer after 600 ticks; `molten` in anything but a crucible after 60 ticks (and the item ignites if its holder is not fire-immune: one fire trigger on the holder's feet cell); `cryogenic` in glass or ceramic after 300 ticks.
- A crucible holds lava or molten metal for pouring but is never drunk from.

### 6.3 Shatter

- Glass and ceramic vessels shatter: when a thrown one impacts anything; when one falls freely at least 4000 mm; when one lying on the ground (`ev.item_dropped`) takes any hit, area damage or blast; by the compatibility rule. Carried vessels never shatter from hits, so co-op inventories are safe.
- Shatter emits `ev.noise` at 70 dB (glass) or 75 dB (ceramic) at the impact cell.
- **Spill distribution.** Contents `v` go to the impact cell (`c0`) and its 4 horizontal neighbors: each non-solid neighbor gets `floor_div(v × 1250, 10000)` (split by §2.4), and `c0` gets the rest. If `c0` is solid, the cell on the thrower's side of the struck face is `c0`. Example: a 250 ml flask gives each of 4 open neighbors 31 ml and keeps 126 ml in `c0`, so only `c0` reaches `CONTACT_ML`.
- Aerosol potions inject their gas instead of spilling (§10.4). Linger potions spill their base liquid with the payload attached (§10.6).

---

## 7. Alchemical effects (`alch.eff.*`)

59 effects. **Pol**: B beneficial, H harmful. **Dep** (Canon §5.3): `none` for mundane effects; `aether` for supernatural effects, whose dependency is `{origin_genre.aether_axiom}` (arcana when that is none), mirroring the spell-glyph rule, so an invisibility dan pill brewed in Wuxia depends on VERVE and an invisibility stim brewed in Cyberpunk on TECHNE; `mythos` and `verve` are fixed. **Kind** and **Unit**:

- `instant`: applied once. `bp_maxhp` is Bp of the recipient's Max HP (Canon §10.9); `bp_pool` is Bp of that pool's max; `milli_tox` is `Milli` toxicity; `points` are whole satiation or sub-attribute points.
- `status`: applies the named Canon status directly with the brewed duration as its `base` (Canon §11.4 scaling applies only to scaled statuses). `ward` sets the `st.shielded` pool to its magnitude in Bp of Max HP.
- `timed`: a modifier held for the duration (mm of dark-vision, mind-vision radius, `SAFE_FALL` bonus; Bp of resistance, added before the resistance cap; sub-attribute points, within the buffed cap 150).
- `buildup`: status power `p` in threshold points, entering Canon §11.2 (× `S(src_lvl)` and the rest of that chain).
- `direct`: a `direct` Canon status applied with the brewed duration as `base`.
- `damage`: `base_mhp` of a hit; at use it follows Canon §10.4 (`× P(src_lvl) → × rarity power (Common 10000) → × quality → × 10000 → × 10000`) and enters the pipeline as an item hit that never crits. Its status power (listed in Mechanic) is scaled by the same per-effect chain as `buildup` effects.
- `special`: `red_dose` fills the `afflict.red_ink` meter (one dose, doc 04 §10.1); it exists only on forbidden draughts (§9.2). `mutate` grants one doc 04 mutation point; a brew whose primary effect is `mutate` is the mutagen potion doc 04 names, and one whose primary is `satiate` is its nourishment potion.

**Twin** is the effect that the Invert modifier substitutes (§9.4); pairs are symmetric. **Tox** is the toxicity this effect adds per dose (§11). **Value** is Lucre at `P(1)` per effect (§12.1). Caps bound the final magnitude and duration after every multiplier, before Contradiction potency and blessing.

|Effect|Pol|Dep|Kind|Unit|Base mag|Base dur|Mag cap|Dur cap|Tox milli|Value|Twin|Mechanic|
|---|---|---|---|---|---|---|---|---|---|---|---|---|
|`alch.eff.heal`|B|none|instant|bp_maxhp|3000|—|9000|—|12000|30|`harm`|res.hp +mag (§10.9 healing rules)|
|`alch.eff.restore_stamina`|B|none|instant|bp_pool|5000|—|10000|—|5000|12|`drain_stamina`|res.stamina +mag of max|
|`alch.eff.restore_aether`|B|aether|instant|bp_pool|4000|—|10000|—|9000|25|`unravel`|res.aether +mag of max|
|`alch.eff.restore_composure`|B|none|instant|bp_pool|3000|—|9000|—|7000|20|`dread`|res.composure +mag of max|
|`alch.eff.regenerate`|B|none|status|—|—|600|—|2400|10000|30|`wither`|st.regenerating for dur|
|`alch.eff.cure`|B|none|instant|—|—|—|—|—|6000|25|`disease`|ends st.poisoned, st.diseased, st.bleeding, st.incubating; blessed: cures 1 affliction|
|`alch.eff.detox`|B|none|instant|milli_tox|20000|—|60000|—|—|15|—|res.toxicity -mag; removes up to 3 st.irradiated stacks|
|`alch.eff.purify`|B|none|instant+timed|mm_radius|3000|1200|6000|4800|3000|20|—|removes gas within radius; inhaled-route immunity for dur|
|`alch.eff.satiate`|B|none|instant|points|300|—|900|—|1000|6|—|res.satiation +mag|
|`alch.eff.water_breathing`|B|aether|timed|—|—|1800|—|7200|4000|15|—|res.breath does not drain for dur|
|`alch.eff.haste`|B|none|status|—|—|900|—|3600|12000|30|`slow`|st.hasted for dur|
|`alch.eff.fortify`|B|none|status|—|—|1200|—|4800|9000|25|`expose`|st.fortified for dur|
|`alch.eff.empower`|B|none|status|—|—|900|—|3600|14000|35|`weaken`|st.empowered for dur|
|`alch.eff.focus`|B|none|status|—|—|1200|—|4800|6000|20|`confuse`|st.focused for dur|
|`alch.eff.ward`|B|aether|status|bp_maxhp|2500|600|7500|2400|9000|30|—|st.shielded pool = mag of Max HP|
|`alch.eff.invisibility`|B|aether|status|—|—|900|—|3600|10000|35|—|st.invisible for dur|
|`alch.eff.levitate`|B|aether|status|—|—|900|—|3600|8000|25|—|st.levitating for dur|
|`alch.eff.featherfall`|B|aether|timed|mm|8000|1800|24000|7200|2000|10|—|SAFE_FALL + mag mm for dur|
|`alch.eff.night_eye`|B|none|timed|mm|6000|3600|18000|14400|3000|10|`blind`|dark-vision radius + mag mm|
|`alch.eff.mind_vision`|B|aether|timed|mm|20000|1200|60000|4800|6000|25|—|see creatures through walls within mag mm|
|`alch.eff.resist_fire`|B|none|timed|bp|2500|3600|7500|14400|5000|15|—|resist[dmg.fire] + mag|
|`alch.eff.resist_frost`|B|none|timed|bp|2500|3600|7500|14400|5000|15|—|resist[dmg.frost] + mag|
|`alch.eff.resist_shock`|B|none|timed|bp|2500|3600|7500|14400|5000|15|—|resist[dmg.shock] + mag|
|`alch.eff.resist_toxin`|B|none|timed|bp|2500|3600|7500|14400|5000|15|`poison`|resist[dmg.toxin] + mag|
|`alch.eff.resist_acid`|B|none|timed|bp|2500|3600|7500|14400|5000|15|—|resist[dmg.acid] + mag|
|`alch.eff.resist_radiation`|B|none|timed|bp|2500|3600|7500|14400|5000|15|—|resist[dmg.radiation] + mag|
|`alch.eff.resist_umbral`|B|mythos|timed|bp|2500|3600|7500|14400|5000|20|—|resist[dmg.umbral] + mag|
|`alch.eff.fortify_might`|B|none|timed|points|10|3600|30|14400|8000|25|—|PWR, BRN, GRP + mag|
|`alch.eff.fortify_finesse`|B|none|timed|points|10|3600|30|14400|8000|25|—|PRC, AGI, SLT + mag|
|`alch.eff.fortify_vigor`|B|none|timed|points|10|3600|30|14400|8000|25|—|TGH, END, RES + mag|
|`alch.eff.fortify_intellect`|B|none|timed|points|10|3600|30|14400|8000|20|—|LOG, MEM, ANL + mag|
|`alch.eff.fortify_perception`|B|none|timed|points|10|3600|30|14400|8000|20|—|SGT, HRG, ITU + mag|
|`alch.eff.inspire`|B|verve|status|—|—|1200|—|4800|6000|30|`fear`|st.inspired for dur|
|`alch.eff.bless`|B|mythos|status|—|—|1800|—|7200|4000|35|—|st.blessed for dur|
|`alch.eff.berserk`|B|none|status|—|—|600|—|2400|15000|15|—|st.berserk on self for dur|
|`alch.eff.poison`|H|none|buildup|p|60|—|180|—|—|15|`resist_toxin`|st.poisoned buildup|
|`alch.eff.harm`|H|none|damage|mhp|15000|—|45000|—|—|20|`heal`|dmg.toxin hit|
|`alch.eff.burn`|H|none|damage|mhp|10000|—|30000|—|—|20|—|dmg.fire hit + st.burning p 50|
|`alch.eff.freeze`|H|none|damage|mhp|10000|—|30000|—|—|20|—|dmg.frost hit + st.frozen p 50|
|`alch.eff.shock`|H|none|damage|mhp|10000|—|30000|—|—|20|—|dmg.shock hit + st.shocked p 50|
|`alch.eff.corrode`|H|none|damage|mhp|8000|—|24000|—|—|20|—|dmg.acid hit + st.corroded p 40|
|`alch.eff.paralyze`|H|none|buildup|p|50|—|150|—|—|25|—|st.stunned buildup|
|`alch.eff.sleep`|H|none|buildup|p|60|—|180|—|—|20|—|st.sleeping buildup|
|`alch.eff.blind`|H|none|buildup|p|60|—|180|—|—|15|`night_eye`|st.blinded buildup|
|`alch.eff.silence`|H|aether|buildup|p|60|—|180|—|—|25|—|st.silenced buildup|
|`alch.eff.unravel`|H|aether|buildup|p|60|—|180|—|—|20|`restore_aether`|st.unraveled buildup|
|`alch.eff.weaken`|H|none|direct|—|—|360|—|1440|—|20|`empower`|st.weakened direct|
|`alch.eff.slow`|H|none|direct|—|—|240|—|960|—|15|`haste`|st.slowed direct|
|`alch.eff.drain_stamina`|H|none|instant|bp_pool|3000|—|9000|—|—|10|`restore_stamina`|res.stamina -mag of max|
|`alch.eff.confuse`|H|none|buildup|p|60|—|180|—|—|20|`focus`|st.confused buildup|
|`alch.eff.fear`|H|none|buildup|p|60|—|180|—|—|20|`inspire`|st.feared buildup|
|`alch.eff.dread`|H|mythos|buildup|p|60|—|180|—|—|20|`restore_composure`|st.dread buildup|
|`alch.eff.wither`|H|mythos|buildup|p|50|—|150|—|—|20|`regenerate`|st.withering buildup|
|`alch.eff.disease`|H|none|buildup|p|50|—|150|—|—|15|`cure`|st.diseased buildup|
|`alch.eff.irradiate`|H|none|buildup|p|40|—|120|—|—|15|—|st.irradiated buildup|
|`alch.eff.expose`|H|none|buildup|p|50|—|150|—|—|20|`fortify`|st.exposed buildup|
|`alch.eff.charm`|H|aether|buildup|p|50|—|150|—|—|40|—|st.charmed buildup|
|`alch.eff.mutate`|B|none|instant|points|1|—|1|—|20000|40|—|+1 doc 04 mutation point (natural form; supernatural form when the origin genre has MYTHOS >= 8)|
|`alch.eff.red_dose`|H|none|special|doses|1|—|1|—|5000|0|—|fills the afflict.red_ink meter: one dose (doc 04)|

Effect application notes:

- `cure` ends `st.poisoned` (all stacks), `st.diseased`, `st.bleeding` and `st.incubating`, and clears their buildup meters. A blessed `cure` also cures one affliction (§12.6).
- `purify` removes every gas component in cells whose centers lie within its magnitude of the drinker's eye, then grants inhaled immunity (as a working respirator, §4.5) for its duration.
- `mind_vision` reveals every creature within its radius through walls to the bearer (doc 09 renders it).
- `berserk` uses the Canon §11.8 player-controlled rules.
- Drinking a harmful potion applies its harmful effects to the drinker. A thrown or coated harmful effect applies to targets (§10).

---

## 8. Ingredients (`ing.*`, `cat.ingredient`)

### 8.1 Schema

```json
{
  "id": "ing.mender_root",
  "category": "cat.ingredient",
  "effects": ["alch.eff.heal", "alch.eff.cure", "alch.eff.regenerate", "alch.eff.slow"],
  "potency_bp": 11000,
  "mass_g": 40,
  "value": 6,
  "tags": ["plant"],
  "sources": [{ "kind": "harvest", "biome_tag": "forest", "weight": 100 }],
  "skins": { "genre.weird_west": "loc.ing.mender_root.skin.weird_west.name" },
  "stack_max": 20
}
```

- Every effect-bearing ingredient has exactly 4 distinct effects in a fixed order. Order matters for reveal (§8.5) and for single-ingredient brewing (§9.3).
- `value` is Lucre at `P(1)`; doc 11 scales it by `P(ilvl)`. Ingredients have dependency none (raw materials, Canon §5.3); the effects they produce carry the §7 dependencies.
- Sources: H harvest a plant block, F harvest a fungus block, M mine an ore block, D enemy drop by creature tag (doc 13 tables), B butcher a `cat.corpse` or `cat.body_part` (Survival, doc 04), S shop, W worldgen container (doc 11). Doc 12 places H, F and M nodes by biome.

### 8.2 Catalog (72 effect ingredients)

Effect columns give the `alch.eff.` suffix.

|ID|Effect 1|Effect 2|Effect 3|Effect 4|Potency Bp|Mass g|Value|Sources|Tags|Skin examples|
|---|---|---|---|---|---|---|---|---|---|---|
|`ing.aether_crystal`|restore_aether|ward|levitate|unravel|13000|30|20|M mine: arcane genres|mineral|space_opera: Psi-crystal; cyberpunk: Bandwidth shard|
|`ing.amber`|resist_shock|regenerate|focus|ward|12000|20|12|M mine; D tag beast|mineral|mythic_antiquity: Electrum amber; cyberpunk: Insulgel|
|`ing.angel_feather`|bless|featherfall|heal|resist_umbral|13000|5|25|D tag divine; W shrines|monster holy|mythic_antiquity: Nike's feather; fairy_tale: Godmother's plume|
|`ing.ash_of_errata`|inspire|weaken|fortify_finesse|resist_radiation|12000|5|16|D fac.editors; ruins|monster|redacted: Expunged ash; ur_text: Margin ash|
|`ing.basilisk_eye`|paralyze|fortify|blind|mind_vision|13000|30|22|D tag basilisk, gorgon|monster|mythic_antiquity: Gorgon eye; space_opera: Petrifier lens|
|`ing.blessed_salt`|purify|bless|resist_umbral|silence|11000|20|10|S temple shops; W shrines|mineral holy|gothic_horror: Consecrated salt; weird_west: Mission salt|
|`ing.blindweed`|invisibility|blind|confuse|night_eye|10000|10|6|H weed: ruins|plant|hardboiled_noir: Shade-weed; fairy_tale: Hide-me-not|
|`ing.bloodmoss`|heal|restore_stamina|resist_frost|disease|10000|20|4|H moss: every genre|plant|gothic_horror: Grave-moss; cyberpunk: Hemo-lichen culture|
|`ing.bombardier_gland`|corrode|burn|expose|resist_acid|11000|10|7|D tag beetle, insect|animal|steampunk: Boiler-beetle gland; wuxia: Fire-beetle sac|
|`ing.brimstone`|burn|fear|wither|resist_frost|10000|60|4|M mine: volcanic|mineral|mythic_fantasy: Hellstone; space_opera: Sulfur ore|
|`ing.charcoal`|purify|detox|resist_toxin|blind|8000|30|1|S shop; crafted from wood (doc 07)|mineral filter|post_apocalypse: Filter char; hardboiled_noir: Carbon|
|`ing.cinnabar`|fortify_vigor|regenerate|poison|inspire|12000|40|10|M mine: wuxia, antiquity|mineral|wuxia: Cinnabar (dan sand); mythic_antiquity: Dragon's-blood ore|
|`ing.circuit_dust`|focus|fortify_intellect|shock|confuse|10000|10|6|D tag construct; M scrap|tech|space_opera: Quantum filings; steampunk: Brass filings|
|`ing.clockwork_spring`|haste|fortify_finesse|shock|slow|10000|40|8|D tag clockwork; M scrap|tech|steampunk: Mainspring; wuxia: Mechanism coil|
|`ing.coffee_bean`|focus|restore_stamina|fortify_perception|drain_stamina|9000|10|3|S shop; H plantations|plant|hardboiled_noir: Joe; cyberpunk: Caf-pellet|
|`ing.demon_horn`|empower|berserk|fear|wither|12000|120|18|D tag fiend|monster|gothic_horror: Devil's horn; wuxia: Yaoguai horn|
|`ing.dreamfoil`|sleep|restore_composure|mind_vision|confuse|10000|10|5|H flower: oneiric, fairy_tale|plant|oneiric: Somnifoil; wuxia: Drowsy-cloud herb|
|`ing.earthroot`|fortify|paralyze|resist_acid|satiate|10000|40|5|H root: caves|plant|mythic_antiquity: Gaia root; post_apocalypse: Ironroot|
|`ing.ectoplasm_wisp`|invisibility|levitate|dread|unravel|12000|1|12|D tag ghost, spirit|monster|gothic_horror: Spirit residue; cosmic_horror: Ectoplasm|
|`ing.eldritch_eye`|mind_vision|dread|fortify_intellect|mutate|12000|40|18|D tag eldritch|monster|cosmic_horror: Eye of the Outer; space_opera: Xeno-ocellus|
|`ing.eye_of_newt`|mind_vision|night_eye|fortify_perception|confuse|10000|5|4|D tag amphibian; B|animal|fairy_tale: Newt's eye; cyberpunk: Optic jelly|
|`ing.fadeleaf`|mind_vision|invisibility|slow|fortify_perception|10000|10|6|H shrub: forest|plant|cosmic_horror: Veil-leaf; cyberpunk: Ghost-fern|
|`ing.fairy_wing`|levitate|charm|featherfall|confuse|11000|1|15|D tag fae; H|monster|fairy_tale: Pixie wing; oneiric: Moth-dream wing|
|`ing.firebloom`|burn|resist_frost|empower|blind|10000|15|5|H flower: volcanic|plant|wuxia: Flame-lotus; post_apocalypse: Burnflower|
|`ing.foxglove`|harm|haste|fortify_vigor|paralyze|10000|10|4|H flower: woods|plant|gothic_horror: Dead-men's-bells; steampunk: Digitalis leaf|
|`ing.frost_salt`|freeze|resist_fire|focus|slow|10000|30|5|M mine: tundra|mineral|space_opera: Cryo-salt; fairy_tale: Winter salt|
|`ing.ghoul_tongue`|paralyze|disease|satiate|wither|10000|40|5|D tag ghoul, undead|monster|gothic_horror: Ghast tongue; wuxia: Jiangshi tongue|
|`ing.ginseng`|restore_stamina|fortify_vigor|regenerate|restore_aether|12000|25|9|H root: wuxia, fantasy|plant|wuxia: Thousand-year ginseng; space_opera: Vita-root|
|`ing.glowshroom`|night_eye|resist_radiation|irradiate|focus|10000|15|4|F mushroom: deep caves|fungus|post_apocalypse: Glowing fungus; space_opera: Lumen-cap|
|`ing.hagberry`|charm|detox|weaken|night_eye|10000|10|6|H bush: fairy_tale, gothic|plant|fairy_tale: Witch's berry; weird_west: Hag's chokeberry|
|`ing.hemlock`|paralyze|poison|harm|silence|11000|15|7|H umbel: riverbanks|plant|mythic_antiquity: Socrates' cup; weird_west: Water hemlock|
|`ing.icecap`|freeze|resist_fire|slow|focus|10000|15|5|H flower: tundra, ice caves|plant|cosmic_horror: Rime-bloom; space_opera: Cryo-moss|
|`ing.ink_lotus`|weaken|silence|restore_aether|fortify_intellect|11000|10|10|H water plant: Scriptorium pools|plant|wuxia: Calligrapher's lotus; redacted: Correction lily|
|`ing.inkcap`|blind|mind_vision|dread|unravel|10000|15|5|F mushroom: libraries, manuscripts|fungus|cosmic_horror: Ink-cap of Yith; gothic_horror: Shaggy mane|
|`ing.kraken_ink`|blind|water_breathing|invisibility|dread|11000|50|12|D tag cephalopod|monster|cosmic_horror: Deep-one ink; steampunk: Squid ink|
|`ing.lich_dust`|unravel|wither|drain_stamina|resist_umbral|12000|10|16|D tag lich, undead caster|monster|gothic_horror: Bone-mage ash; wuxia: Corpse-sage powder|
|`ing.lodestone`|resist_shock|shock|slow|fortify|10000|80|6|M mine|mineral|steampunk: Magnetite; cyberpunk: Ferrite core|
|`ing.lotus_seed`|restore_composure|purify|focus|water_breathing|11000|10|8|H water plant: wuxia, antiquity|plant|wuxia: Snow lotus seed; mythic_antiquity: Lotus-eater seed|
|`ing.mageroyal`|purify|restore_aether|cure|silence|10000|10|6|H flower: arcane biomes|plant|fairy_tale: Queen's lace; space_opera: Filter-reed|
|`ing.mandrake`|sleep|fear|fortify_intellect|restore_aether|12000|60|12|H root: graveyards|plant|fairy_tale: Screaming root; oneiric: Dream-root|
|`ing.mender_root`|heal|cure|regenerate|slow|11000|40|6|H root: forest biomes|plant|weird_west: Snakeroot; space_opera: Medi-tuber|
|`ing.mint_leaf`|restore_stamina|detox|focus|resist_fire|9000|5|2|H herb: every genre|plant|cyberpunk: Menthol cartridge; fairy_tale: Pennyroyal|
|`ing.mutagen`|mutate|irradiate|regenerate|fortify_might|11000|30|10|D tag mutant; W labs|tech|post_apocalypse: Mutant serum; cosmic_horror: Shoggoth cell|
|`ing.nanopaste`|heal|ward|fortify|unravel|12000|20|14|S shop; D tag construct|tech|cyberpunk: Medi-nanite paste; steampunk: Restorative grease|
|`ing.nightshade`|poison|sleep|night_eye|harm|10000|10|5|H berry: shade|plant|gothic_horror: Belladonna; cyberpunk: Black-berry synth|
|`ing.ooze_core`|corrode|resist_acid|slow|satiate|10000|200|5|D tag ooze, amorphous|monster|post_apocalypse: Sludge heart; space_opera: Protoplasm nucleus|
|`ing.peach_of_immortality`|regenerate|heal|inspire|fortify_vigor|14000|150|45|H tree: wuxia; Reliquary|plant|wuxia: Immortal peach; mythic_antiquity: Golden apple|
|`ing.peyote_flesh`|mind_vision|restore_stamina|confuse|fear|10000|30|5|H cactus: deserts|plant|weird_west: Peyote button; space_opera: Psi-cactus|
|`ing.phoenix_feather`|regenerate|resist_fire|burn|bless|14000|5|40|D tag phoenix; Reliquary|monster|wuxia: Vermilion bird plume; fairy_tale: Firebird feather|
|`ing.pitchblende`|irradiate|night_eye|resist_radiation|harm|11000|100|8|M mine: post_apocalypse, space_opera|mineral|post_apocalypse: Hot rock; space_opera: Isotope ore|
|`ing.poppy_tear`|sleep|restore_composure|slow|heal|10000|5|6|H flower: fields|plant|hardboiled_noir: Laudanum gum; wuxia: Dream poppy|
|`ing.redcap`|restore_stamina|poison|berserk|fortify_might|10000|20|3|F mushroom: caves|fungus|fairy_tale: Fly agaric; post_apocalypse: Rad-cap|
|`ing.rotberry`|fortify_might|disease|satiate|empower|10000|20|4|H bush: ruins|plant|post_apocalypse: Mutberry; gothic_horror: Grave-berry|
|`ing.salamander_tail`|resist_fire|regenerate|burn|water_breathing|11000|40|9|D tag salamander, amphibian|animal|mythic_fantasy: Salamander tail; cyberpunk: Regen-graft tissue|
|`ing.saltpeter`|burn|purify|resist_frost|expose|9000|50|3|M mine; S shop|mineral|steampunk: Nitre; wuxia: Fire-saltpetre|
|`ing.serpent_venom`|poison|paralyze|resist_toxin|expose|11000|10|8|D tag serpent|animal|weird_west: Rattler venom; mythic_antiquity: Asp venom|
|`ing.siren_scale`|charm|water_breathing|restore_composure|confuse|12000|10|15|D tag siren, merfolk|monster|mythic_antiquity: Siren scale; space_opera: Lure-scale|
|`ing.sorrowmoss`|poison|dread|disease|resist_toxin|10000|15|4|H moss: swamps|plant|gothic_horror: Widow's moss; cyberpunk: Toxo-lichen|
|`ing.spider_gland`|paralyze|poison|slow|night_eye|10000|10|6|D tag spider|animal|cyberpunk: Neuro-venom sac; fairy_tale: Ettercap gland|
|`ing.spore_sac`|disease|confuse|poison|resist_toxin|9000|20|3|D tag spore; F|fungus|post_apocalypse: Mold sac; space_opera: Xeno-spore pod|
|`ing.starflower`|restore_aether|inspire|bless|fortify_perception|12000|10|10|H flower: high places|plant|mythic_antiquity: Asphodel; space_opera: Nova-bloom|
|`ing.stim_gland`|haste|empower|restore_stamina|berserk|12000|20|12|D tag construct, cyborg; S|tech|cyberpunk: Adrenal booster; space_opera: Combat gland|
|`ing.stormvine`|levitate|shock|haste|resist_shock|10000|20|6|H vine: storm biomes, steampunk|plant|steampunk: Galvanic creeper; space_opera: Ion-vine|
|`ing.sungrass`|heal|regenerate|satiate|resist_radiation|10000|15|4|H grass: fantasy, fairy_tale, antiquity|plant|wuxia: Sun-blade grass; post_apocalypse: Green-stem|
|`ing.swiftthistle`|haste|restore_stamina|featherfall|confuse|10000|10|5|H thistle: plains|plant|cyberpunk: Kick-weed; weird_west: Jackrabbit thistle|
|`ing.troll_fat`|regenerate|heal|fortify_vigor|slow|12000|80|12|D tag troll, brute|animal fat|fairy_tale: Ogre lard; post_apocalypse: Mutant fat|
|`ing.unicorn_hair`|cure|purify|heal|bless|14000|1|40|D tag unicorn; Reliquary|monster holy|fairy_tale: Unicorn mane; wuxia: Qilin hair|
|`ing.vampire_dust`|wither|invisibility|regenerate|dread|12000|10|14|D tag vampire|monster|gothic_horror: Nosferatu ash; cyberpunk: Night-gland powder|
|`ing.vitriol`|corrode|purify|expose|resist_acid|10000|50|5|M mine; S shop|mineral|steampunk: Oil of vitriol crystal; cyberpunk: Etchant salt|
|`ing.wolf_heart`|berserk|fortify_might|haste|wither|11000|300|8|D tag wolf, were|animal|weird_west: Coyote heart; wuxia: Tiger heart|
|`ing.wolfsbane`|poison|resist_umbral|fear|paralyze|10000|15|6|H flower: mountains|plant|gothic_horror: Monkshood; wuxia: Aconite of the wolf|
|`ing.wyrm_scale`|resist_fire|fortify|empower|burn|13000|60|25|D tag dragon, reptile|animal|wuxia: Jiao scale; space_opera: Salamander plate|

Every brewable effect appears on at least 2 ingredients.

### 8.3 Modifier reagents and special ingredients

Modifiers carry no effects and do not count toward the 2 to 4 effect ingredients of a brew (§9.3).

|ID|Role|Rule|Mass g|Value|Sources|Skins|
|---|---|---|---|---|---|---|
|`ing.quicksalt`|Extend|every duration × 16000; toxicity × 11000; not with Amplify|10|8|M, S|time-salt, nano-stabilizer, slow-ember sand|
|`ing.bright_dust`|Amplify|every magnitude × 13000, every duration × 6000; toxicity × 12000; not with Extend|5|10|M, S|glowstone dust, overclock powder, gold-cloud dust|
|`ing.black_gall`|Invert|each effect with a twin becomes its twin (§9.4)|10|6|D tag insect, S|oak gall, null-code shard, reversal gall|
|`ing.puffball`|Aerosol|the potion becomes an aerosol (§10.4); not with Linger|15|5|F|puffball spore, atomizer cell, smoke-gourd|
|`ing.gum_resin`|Linger|the potion's spill carries its payload (§10.6); not with Aerosol|15|5|H tree|pine gum, adhesive polymer, frankincense|
|`ing.essence`|transmutation currency (§15)|stack 999|1|1|alchemizing (§15.1)|quintessence, reagent credits, refined qi, ichor salt|
|`ing.black_powder`|bomb charge (§13.2)|no brewing use|20|3|§15 recipe, S|gunpowder, flash compound, fire medicine|

### 8.4 Seeds (`cat.seed`)

Seeds are planted (doc 07) to grow their plant, or brewed three at a time (§15.2). Each maps to one canonical potion.

|Seed ID|Grows|Seed brew potion|
|---|---|---|
|`ing.seed_sungrass`|`ing.sungrass`|`alch.potion.healing`|
|`ing.seed_ginseng`|`ing.ginseng`|`alch.potion.stamina`|
|`ing.seed_starflower`|`ing.starflower`|`alch.potion.aether`|
|`ing.seed_poppy`|`ing.poppy_tear`|`alch.potion.calm`|
|`ing.seed_swiftthistle`|`ing.swiftthistle`|`alch.potion.haste`|
|`ing.seed_blindweed`|`ing.blindweed`|`alch.potion.invisibility`|
|`ing.seed_stormvine`|`ing.stormvine`|`alch.potion.levitation`|
|`ing.seed_mageroyal`|`ing.mageroyal`|`alch.potion.purity`|
|`ing.seed_fadeleaf`|`ing.fadeleaf`|`alch.potion.mind_vision`|
|`ing.seed_earthroot`|`ing.earthroot`|`alch.potion.fortitude`|
|`ing.seed_mender_root`|`ing.mender_root`|`alch.potion.cure`|
|`ing.seed_glowshroom`|`ing.glowshroom`|`alch.potion.night_eye`|
|`ing.seed_firebloom`|`ing.firebloom`|`alch.potion.liquid_flame`|
|`ing.seed_icecap`|`ing.icecap`|`alch.potion.frost`|
|`ing.seed_sorrowmoss`|`ing.sorrowmoss`|`alch.potion.toxic_gas`|
|`ing.seed_dreamfoil`|`ing.dreamfoil`|`alch.potion.slumber`|

### 8.5 Effect reveal

Knowledge is per run and shared by the party: `ing_known: BTreeMap<u32 /* ing id32 */, u8 /* bit k = effect k known */>`, saved with the run.

- **By skill tier** (personal, not written to the record): a player sees effect `k` (0-based) of every ingredient when `skill.alchemy >= [0, 25, 50, 75][k]`. Skill 0 sees the first effect, 25 the first two, 50 three, 75 all four (Oblivion ranks).
- **By tasting** (`input` Taste, 45 ticks, Canon §6.4 row 17 applies): consumes one unit; sets the lowest unknown bit (the two lowest at Alchemy ≥ 50); applies that ingredient's first effect to the taster with magnitude, duration and status power × 2500; adds 2,000 toxicity. With all bits known, tasting applies the first effect and reveals nothing.
- **By brewing**: on completion, every effect shared by at least two contributing ingredients (including effects beyond the 4-effect cap) becomes known on each ingredient that carries it (Morrowind: hidden effects still combine). Before brewing, the preview shows known shared effects by name and the count of unknown shared effects as "N unknown".
- Each newly set bit emits `ev.item_identified { kind: ingredient_effect }`. The Margin codex records discovered pairs account-wide for reading in the hub only; it never sets in-run bits.

---

## 9. Brewing

### 9.1 Apparatus (`apparatus.*`, `cat.tool`)

Each apparatus has a tier 1 to 5. A brewer uses the best tier of each kind among apparatus in their inventory or installed in the `station.alchemy` (doc 07) they brew at. A mortar is required.

|ID|Tier values (T1..T5)|Applies to|Mass g (T1..T5)|
|---|---|---|---|
|`apparatus.mortar`|`mortar_bp` 9000, 10000, 11000, 12000, 13000|quality score (§9.5)|800, 1000, 1200, 1400, 1600|
|`apparatus.retort`|`retort_bp = 10000 + 1000 × tier`|durations of beneficial effects|1500 + 300 × tier|
|`apparatus.alembic`|`alembic_bp = 10000 − 1500 × tier`; `alembic_tox_bp = 10000 − 800 × tier`|magnitudes of harmful effects in a potion whose primary effect is beneficial; that potion's toxicity|1500 + 300 × tier|
|`apparatus.calcinator`|`calc_bp = 10000 + 500 × tier`|magnitudes of every effect, harmful ones included (Morrowind)|2000 + 300 × tier|
|`apparatus.vaporizer`|none|vaporize delivery (§10.4)|1200|

Genre skins (mortar / retort / alembic / calcinator / station): Mythic Fantasy mortar and pestle / retort / alembic / calcinator / alchemist's bench; Gothic Horror apothecary mortar / distilling coil / cucurbit / athanor / apothecary; Space Opera homogenizer / synth-coil / filter column / plasma kiln / med-fab; Cyberpunk grinder / centrifuge / dialysis filter / reactor block / chem-lab; Post-Apocalypse hand mill / radiator still / mask filter / scrap kiln / cook shack; Wuxia grinding stone / condensing jar / cinnabar sieve / pill furnace / alchemy hall; Steampunk grinder / pressure still / condenser / boiler-kiln / laboratory engine; Fairy Tale acorn mortar / dew-spinner / petal sieve / ember-hearth / witch's cauldron; Cosmic Horror bone mortar / vitreous coil / grey alembic / star-kiln / laboratory; Weird West pestle bowl / copper still / cloth strainer / tin oven / medicine wagon; Hardboiled Noir pill crusher / bathtub coil / coffee filter / hot plate / back-room lab; Mythic Antiquity marble mortar / ambix / kerotakis / furnace / pharmakeion. Secret genres use the Mythic Fantasy names.

### 9.2 Base liquids

The base is the liquid every dose is made of. It must be a single component with `brew_base` data.

|Base|mag_bp|dur_bp|tox_bp|Special|
|---|---|---|---|---|
|`liq.water`|10000|10000|10000|default|
|`liq.alcohol`|10000|13000|13000|spills flammable alcohol (tincture)|
|`liq.oil`|9000|15000|9000|coat charges × 15000 (§10.3); spills oil|
|`liq.honey`|9500|10000|7000|each dose also gives satiation +100 when drunk (oxymel)|
|`liq.wine`|9500|11000|8000|tonic|
|`liq.milk`|9000|10000|6000|gentle|
|`liq.blood`|12000|8000|12000|blood rite|
|`liq.holy_water`|10000|10000|10000|result is `buc.blessed`|
|`liq.unholy_water`|10000|10000|10000|harmful magnitudes × 13000 (applied after `mag_bp`); result is `buc.cursed`|
|`liq.mana`|10000|10000|10000|magnitudes of `aether`-dep effects × 13000|
|`liq.nanite_slurry`|12000|7000|11000|stim base|
|`liq.ichor`|12500|10000|12000|ambrosia base|
|`liq.nectar`|11000|12000|8000|fairy-dew base|
|`liq.red_ink`|15000|15000|10000|adds `alch.eff.red_dose` as a fifth, uncapped effect: a **forbidden draught**|

### 9.3 Inputs and validation

A brew request is `{ ingredients: 2..=4 distinct effect ingredients (1 at skill.alchemy 100), modifiers: 0..=3 modifier reagents, base: vessel or cell holding the base liquid, vessel_kind, doses }`.

- At most one of Extend and Amplify; at most one of Aerosol and Linger; at most one Invert. Grenade shells cannot take Aerosol or Linger (a shell already shatters into a splash).
- `doses <= batch` (§9.7); the brewer holds `doses` empty vessels of `vessel_kind` (or one bottle per 3 doses) and at least `doses ×` the vessel's dose volume (§6.1) of base liquid.
- One unit of each ingredient and modifier is consumed per brew, regardless of `doses`.
- A single-ingredient brew (skill 100) yields only that ingredient's first effect with `n = 1`.

### 9.4 Shared-effect rule and selection

```rust
fn select_effects(ings: &[Ingredient], invert: bool) -> Vec<Selected> {
    // n(e) = number of ingredients carrying e; pot(e) = floor_div(Σ potency of those, n(e))
    let mut s: Vec<Selected> = all e with n(e) >= 2 (or the single ingredient's effect 1);
    if invert {
        for x in s.iter_mut() { if let Some(t) = twin(x.eff) { x.eff = t; } }
        dedupe: for equal effects keep the entry with larger n, ties to the earlier entry;
    }
    s.sort_by(|a, b| b.n.cmp(&a.n).then(b.value.cmp(&a.value)).then(a.id32.cmp(&b.id32)));
    s.truncate(4);
    s   // s[0] is the primary effect; the potion's polarity is s[0]'s polarity
}
```

`shared_bp` by `n`: 1 → 8000, 2 → 10000, 3 → 11500, 4 → 12500. A brew with no selected effect is a **murky draught** (§9.8).

### 9.5 Quality

```
f        = skill.alchemy + floor_div(ANL_base, 5) + floor_div(FAT_base, 10)    // base sub-attributes; fortify effects never count
skill_bp = 8000 + 50 × f                                                       // f in 0..=130
v        = keyed rng.alchemy (actor = brewer, tick = brew start tick, seq).range_incl(-500, 500)   // exactly 1 draw per brew
q_score  = mul_bp(skill_bp, mortar_bp) + v
```

|q_score|Quality (Canon §13.5)|
|---|---|
|< 7500|`q.awful` 7000|
|7500..=8999|`q.poor` 8500|
|9000..=10999|`q.normal` 10000|
|11000..=12499|`q.good` 11000|
|12500..=13999|`q.excellent` 12000|
|14000..=15999|`q.masterwork` 13500|
|>= 16000|`q.peerless` 15000|

Using base sub-attributes breaks the Morrowind fortify-intelligence loop: no potion can raise its own successor's power.

### 9.6 Magnitude, duration and toxicity

For each selected effect `e`, every arrow is one `mul_bp`, in this order:

```
mag(e)  = base_mag(e) → pot(e) → shared_bp(n) → q_bp → calc_bp
          → alembic_bp            if e is harmful and the primary effect is beneficial
          → base.mag_bp → base special (unholy harmful 13000; mana aether-dep 13000)
          → 13000                 if Amplify
          → min(mag_cap(e))
dur(e)  = base_dur(e) → pot(e) → shared_bp(n) → q_bp
          → retort_bp             if e is beneficial
          → base.dur_bp
          → 16000 if Extend | 6000 if Amplify
          → min(dur_cap(e))
```

- For `damage` effects, `mag(e)` omits the `q_bp` arrow; quality enters at use through Canon §10.4. Their status power and every `buildup` power follow the full `mag` chain with `base_mag = p`.
- Effects without a magnitude or duration skip that chain.
- **Toxicity per dose** (stored on the item):

```
tox = Σ tox(e) over selected effects (plus 5000 for red_dose)
      → base.tox_bp → alembic_tox_bp (if an alembic was used) → 11000 if Extend → 12000 if Amplify
      → 12500 if the vessel is an ampoule
```

- **Value** (Lucre at `P(1)`, doc 11 scales): `mul_bp(Σ value(e), q_bp)`.
- The potion's `ilvl` is the Folio index where it was brewed (Canon §13.8.1); `origin_genre` is the local genre at the brewer at completion.

### 9.7 Batch, time and the brewing state machine

```
batch = 1 + [at a station.alchemy] + [skill.alchemy >= 50] + [skill.alchemy >= 100]     // 1..=4 doses
brew_ticks = 150 (field) or 120 (station), then Canon §6.4 row 17
```

|State|Event|Next state|Effect|
|---|---|---|---|
|Idle|`input` Brew with a valid request (§9.3)|Brewing|store request; record `start_tick`; draw nothing yet|
|Brewing|`t = start_tick + brew_ticks`|Complete|draw `v` (§9.5); consume inputs; create `doses` items; reveal effects (§8.5); emit `ev.potion_brewed`, `ev.item_crafted`|
|Brewing|non-DoT damage taken, move more than 1000 mm from start, cancel input, or an input item leaves the inventory|Idle|nothing consumed, nothing drawn|
|Complete|next tick|Idle|grant XP (§9.11)|

At most one brewer uses a station at a time; a second brewer's request is refused while it is Brewing. The quality is shown only at completion, after the inputs are consumed, so cancelling and restarting reveals nothing to reroll against.

### 9.8 Failure

- No random failure destroys ingredients. The only variance is `v` (±500 on `q_score`, at most one tier).
- **Murky draught**: a brew with no selected effect still completes, consumes its inputs and yields `doses` of `alch.potion.draught` with no effects, toxicity 3,000 per dose, value 0. It reveals nothing.
- Use-time failure comes only from Contradiction (§9.9).

### 9.9 Contradiction of potions

Each selected effect with a non-empty dependency is a component (Canon §5.3) with `Req[a] = max(1, origin_genre.axiom[a] − 2)`. The primary effect is the base-function component.

- On every activation (drink, throw, inject, coat, vapor release, payload creation), Contradiction is evaluated at the user's position (Canon §5.5). Each component's potency multiplies its magnitude, duration and status power; Disconnected components do nothing.
- If the primary effect's failure chance is above 0, roll it once with keyed `rng.combat` (Canon §5.5). On failure the potion is consumed and nothing applies except its toxicity (drink) or its plain base-liquid spill (throw).
- A harmful component hitting a native target gets Anachronism Edge on its damage and buildup (Canon §5.6).

**Worked example.** An invisibility potion brewed in Mythic Fantasy (aether axiom arcana, ARC 8) has `dep = {arcana}`, `Req = 6`. Drunk in Hardboiled Noir (ARC 0) by a Fantasy-origin Erratum with ATN 40: `raw_gap = 6`, `Tol = 2`, `C = 4`: potency 6000, failure 1000 Bp. A 900-tick brew lasts `mul_bp(900, 6000) = 540` ticks if the 10% failure roll misses.

### 9.10 Worked examples

**Apprentice healing.** A new `calling.alchemist` (Alchemy 20, ANL 46, FAT 40) field-brews `ing.bloodmoss` (10000), `ing.sungrass` (10000) and `ing.mender_root` (11000) on water in flasks with a T1 mortar. `f = 20 + 9 + 4 = 33`, `skill_bp = 9650`, `mul_bp(9650, 9000) = 8685`; with `v = +317`, `q_score = 9002`: Normal. Shared: `heal` (n 3), `regenerate` (n 2, sungrass and mender root). `heal`: `pot = floor_div(31000, 3) = 10333`; `3000 → 3099 → 3563 → 3563` (shared 11500, quality 10000), so 3563 Bp (35.6% of Max HP). `regenerate`: `pot = 10500`; `600 → 630` ticks. Toxicity `12000 + 10000 = 22000`. Batch 1. It is classified `alch.potion.healing` (§12.3).

**Master haste.** Alchemy 100, ANL 80, FAT 60 at a station with mortar T4, retort T5, alembic T5, calcinator T3, on alcohol: `ing.stim_gland` (12000), `ing.swiftthistle`, `ing.stormvine`, `ing.clockwork_spring` (10000 each). `f = 122`, `skill_bp = 14100`, `mul_bp(14100, 12000) = 16920`; with `v = −212`, `q_score = 16708`: Peerless (15000). Shared: `haste` (n 4, pot 10500), then `shock` (n 2, value 20) before `restore_stamina` (n 2, value 12).
- `haste` duration: `900 → 945 → 1181 → 1771 → 2656 (retort 15000) → 3452 (alcohol 13000)`, under the 3600 cap: 3452 ticks.
- `shock` (harmful side effect): `base_mhp 10000 → 10000 → 10000 → 11500 (calc) → 2875 (alembic 2500) → 2875`. Its shocked power `50 → 50 → 50 → 75 → 86 → 21 → 21`. The drinker zaps themself: at `ilvl 10` the hit is `mul_bp(mul_bp(2875, P(10) = 21718), 15000) = 9364` mHP before resistance.
- `restore_stamina`: `5000 → 5500 → 5500 → 8250 → 9487 → 9487` Bp.
- Toxicity: `(12000 + 5000) → 22100 (alcohol) → 13260 (alembic 6000)`. Batch `1 + 1 + 1 + 1 = 4` doses. Value `mul_bp(62, 15000) = 93`.

### 9.11 Alchemy XP basis

Canon §7.1 gives non-combat XP proportional to magnitude and doc 04 owns the table. This chapter supplies its inputs: a completed brew reports `brew_basis = Σ n(e)` over selected effects and `new_reveals`; tasting reports `new_reveals`. Proposed default for doc 04: `600 × brew_basis + 1000 × new_reveals` milli-XP per brew, `250 × new_reveals` per taste; a murky draught and a fully known taste grant 0.

---

## 10. Delivery

Every delivery emits `ev.item_used` and evaluates Contradiction once (§9.9). `blessed` multiplies beneficial magnitudes and durations by 12500 (Canon §13.6); `cursed` multiplies them by 7500 and toxicity by 15000. Diminishing returns (§11.3) apply per recipient.

### 10.1 Drink

- 45 ticks (Canon §6.4 row 17). On completion: apply every selected effect to the drinker in list order, then add toxicity (§11.1). A bottle, waterskin or canteen loses one dose.
- **Drinking from a cell** (kneel, 60 ticks): takes 250 ml by the §2.4 split and applies each component's §3.2 drink row scaled by its share. Wellspring components roll §12.5.
- `kin.construct` drinkers heal × 5000 (Canon §16.3).

### 10.2 Throw and splash

- Doc 05 owns the throw (range × Canon §6.4 row 20) and the impact. A shattering vessel (§6.3) applies a **splash** centered on the impact point, radius by vessel (§6.1), to every entity whose capsule center is within the radius with line of effect:
  - beneficial effects apply at full effect to every `fac.errata` member and minion in the area (Canon §10.9), and to nobody else;
  - harmful effects apply to every entity in the area; damage effects are area hits (`FF_AREA_BP` between Errata, Canon §17.5), and buildup on `fac.errata` targets is × `FF_AREA_BP` as well.
- Then the contents spill into the automaton (§6.3), so a liquid-flame potion's oil base burns and a frost potion's water freezes. An Aerosol potion injects instead (§10.4); a Linger potion's spill carries its payload (§10.6).
- Unknown potions identify by throwing only as §12.4 says.

### 10.3 Coat

- Apply a potion (or vial) whose effects include at least one harmful effect to the active melee weapon, or to up to `10 + floor_div(skill.alchemy, 10)` units of the loaded ammunition stack (doc 05 splits the stack). 90 ticks, one dose.
- The coat stores the harmful effects only, with their status powers, the potion's `src_lvl`, quality, potency snapshot and `charges = 5 + floor_div(skill.alchemy, 20)` (× `coat_charges_bp` 15000 for oil bases), and `expires = t + 3600`.
- Each landed hit (Canon §10.5 step 8 reached with any HP or ward loss) spends one charge and adds, for each coated effect, buildup `mul_bp(p(e), 5000)` of its status through Canon §11.2 (× `S(src_lvl)`, potency, edge, attacker buildup multiplier, resistance, block). A damage effect uses its status power as `p`; `harm` counts as `st.poisoned` power `floor_div(base_mhp, 250)` (60 at base). Coats never add damage.
- A coat ends when charges reach 0, at `expires`, when the bearer swims or wades in an aqueous liquid, or when a new coat is applied.
- **Flaming dip.** Dipping a melee weapon (30 ticks) into a burning liquid cell gives it a `flaming` coat for 600 ticks: each landed hit adds 40 `st.burning` buildup.

### 10.4 Vaporize and aerosols

- An **Aerosol** potion shattering, or any potion placed in a lit `apparatus.vaporizer` (60 ticks to load; the vaporizer is placed at the user's feet), creates a payload with `route = Vapor` and injects 4,000,000 ml of `gas.alch_vapor` carrying it (the vaporizer releases 400,000 ml per gases step for 10 steps). `alch.potion.toxic_gas`, `slumber` and `corrosive_gas` inject their named gas instead and carry no payload.
- **Vapor exposure** (eye cell holds the component, no working respirator or purify immunity): the first exposure per (entity, payload) applies the beneficial effects at magnitude and duration × 6000 (to `fac.errata` members and minions only); every status pulse in contact applies each harmful buildup effect at `mul_bp(p, 2000)` and each harmful damage effect as DoT rate 50 Bp of its type. Exposures are tracked in `BTreeSet<(EntityId, u16)>`, pruned when the payload expires.

### 10.5 Inject

- Ampoules (`apparatus.vessel.ampoule`) are injected, never thrown. Self-injection takes 20 ticks and works while `st.grappled` (a one-handed action) or `st.silenced`. `st.berserk` bearers cannot inject (Canon §11.8 forbids item use).
- Injecting another entity takes 30 ticks within 1500 mm: allies any time; an enemy only if it has `st.sleeping`, `st.stunned`, `st.frozen`, `st.knocked_down` or `st.grappled`, or the injector is sneak-attack eligible against it (Canon §10.5 step 2.4). Every effect applies at full magnitude to the injected entity, harmful buildup as a direct buildup application of `p` (no coat 5000 factor).
- Toxicity of an ampoule dose is × 12500 (§9.6).

### 10.6 Linger surfaces

- A Linger potion's spill creates a payload with `route = Surface` that lives 1,200 ticks on the spilled base-liquid components (§2.9).
- Contact with a payload component (`CONTACT_ML`): the first exposure per (entity, payload) applies beneficial effects at × 5000 to `fac.errata` members and minions; each status pulse applies harmful buildup at `mul_bp(p, 2500)` and harmful damage effects as DoT rate 50 Bp of their type.
- Dipping a weapon (30 ticks) into a cell holding at least 1,000 ml of a payload component coats it as §10.3 with that payload's harmful effects and 3 charges.

---

## 11. Toxicity, overdose and diminishing returns

### 11.1 Gaining toxicity

```
applied = tox_item → 15000 if cursed → (10000 − 25 × skill.alchemy of the drinker) → kin_bp
res.toxicity = min(TOX_CAP, res.toxicity + applied)
```

`kin_bp`: construct 5000, undead 5000, others 10000. Thrown, coated, vapor and surface effects never add toxicity to anyone. Tasting adds 2,000; raw drinks add their §3.2 values (these skip the skill factor).

### 11.2 Bands and overdose

|`res.toxicity`|Band|Effect (non-status factors, Canon §11.7)|
|---|---|---|
|0..=49,999|clean|none|
|50,000..=74,999|queasy|stamina regeneration × 8500|
|75,000..=99,999|sickened|stamina regeneration × 8500; beneficial magnitudes and durations from newly drunk or injected doses × 7500; spread × 12000|
|100,000|overdosed|as sickened, plus Canon §12: 3 stacks of `st.poisoned` every 5 s (needs tick, on ticks where `(t − overdose_start) % 300 == 0`)|

- **Decay**: −500 per second on the needs tick (Canon §12), accumulator `k = 1`.
- **Overdose damage**: if `res.toxicity + applied > TOX_CAP`, the excess `x` deals `mul_bp(max_hp_mhp, floor_div(x, 10))` mHP of `dmg.toxin` entering Canon §10.6 at step 4 (resistance applies; constructs take it as `dmg.shock`). Example: an 85 HP Erratum at 95,000 drinks a dose that applies 11,400: excess 6,400 deals `mul_bp(85000, 640) = 5,440` mHP and toxicity sits at 100,000.
- `alch.eff.detox` and water (−2,000) lower toxicity; nothing else does.

### 11.3 Diminishing returns

Per (recipient, effect ID): `dr_count` and `dr_last_tick`. When an effect from any potion, vapor, surface or injection applies at tick `t`: if `t − dr_last_tick < DR_WINDOW` then `dr_count += 1` else `dr_count = 1`; `dr_last_tick = t`. Magnitude and duration × `[10000, 7500, 5000, 3000][min(dr_count, 4) − 1]`, after every other factor. Harmful buildup is exempt (Canon post-proc protection already bounds it).

Example: three Normal healing potions (3000 Bp) drunk 10 s apart heal 3000, 2250 and 1500 Bp; a fourth within 30 s of the third heals 900 Bp. Each applies 11,400 toxicity at Alchemy 20 (`mul_bp(12000, 9500)`); four doses total 45,600, and decay of 500 per second removes that in 92 needs ticks (91 ticks remove 45,500).

---

## 12. Potion types, unknown potions and identification

### 12.1 Canonical potions (`alch.potion.*`, `cat.potion`, `idc.potion`)

Found potions are `q.normal` unless doc 11 rolls otherwise and carry these effects at base values × quality. Value is `Σ value(e)` × quality at `P(1)`.

|ID|Effects (base values)|Base|Value|
|---|---|---|---|
|`alch.potion.healing`|heal 3000 Bp|water|30|
|`alch.potion.stamina`|restore_stamina 5000 Bp|water|12|
|`alch.potion.aether`|restore_aether 4000 Bp|water|25|
|`alch.potion.calm`|restore_composure 3000 Bp|wine|20|
|`alch.potion.haste`|haste 900 ticks|water|30|
|`alch.potion.invisibility`|invisibility 900 ticks|water|35|
|`alch.potion.levitation`|levitate 900 ticks|water|25|
|`alch.potion.purity`|purify 3000 mm, 1200 ticks|water|20|
|`alch.potion.mind_vision`|mind_vision 20000 mm, 1200 ticks|water|25|
|`alch.potion.fortitude`|fortify 1200 ticks|water|25|
|`alch.potion.cure`|cure|water|25|
|`alch.potion.night_eye`|night_eye 6000 mm, 3600 ticks|water|10|
|`alch.potion.liquid_flame`|burn 10 HP, burning p 50|oil|20|
|`alch.potion.frost`|freeze 10 HP, frozen p 50|water|20|
|`alch.potion.toxic_gas`|poison p 60; shatters into 4,000,000 ml `gas.miasma`|water|15|
|`alch.potion.slumber`|sleep p 60; shatters into 4,000,000 ml `gas.soporific`|water|20|

### 12.2 Exotics, elixirs and the draught

Created only by transmutation (§15) or found in Reliquaries (doc 11).

|ID|Effects|Made from (§15)|
|---|---|---|
|`alch.potion.shielding`|ward 4000 Bp, 900 ticks|healing|
|`alch.potion.adrenaline`|empower 900 ticks; haste 600 ticks|stamina|
|`alch.potion.shadowmeld`|invisibility 1500 ticks; night_eye 6000 mm, 1500 ticks|invisibility|
|`alch.potion.dragonfire`|burn 20 HP, burning p 100; base fuel|liquid_flame|
|`alch.potion.snap_freeze`|freeze 15 HP, frozen p 100; slow 240 ticks|frost|
|`alch.potion.corrosive_gas`|corrode 8 HP; shatters into 4,000,000 ml `gas.caustic`|toxic_gas|
|`alch.potion.cleansing`|purify 6000 mm, 2400 ticks; cure; detox 20000|purity|
|`alch.potion.panacea`|cure; detox 40000; regenerate 900 ticks|cure|
|`alch.potion.honeyed_healing`|heal 2000 Bp; regenerate 900 ticks; satiate 300; base honey|healing + honey|
|`alch.potion.dragons_blood`|resist_fire 5000 Bp, 3600 ticks; empower 900 ticks|liquid_flame + wyrm scale|
|`alch.potion.aquatic_rejuvenation`|water_breathing 3600 ticks; regenerate 1800 ticks|healing + siren scale|
|`alch.potion.arcane_armor`|ward 3000 Bp, 1200 ticks; resist_umbral 2500 Bp, 3600 ticks|aether + amber|
|`alch.potion.draught`|whatever was brewed (custom brews and murky draughts)|brewing (§12.3)|

29 `cat.potion` types in all. Doc 10 must supply at least 32 `idc.potion` appearance descriptors.

### 12.3 Classifying a brew

A brewed potion takes the type of the first matching row; its instance stores the brewed effects, which override the type's defaults.

|Primary effect (and condition)|Type|
|---|---|
|poison with Aerosol|`alch.potion.toxic_gas` (injects `gas.miasma` instead of `gas.alch_vapor`)|
|sleep with Aerosol|`alch.potion.slumber`|
|the defining effect of one of the other 14 canonical potions, no Aerosol|that potion|
|anything else|`alch.potion.draught`, named by the primary effect ("Draught of Paralysis"); with `red_dose` it is a forbidden draught|

Brewed items start at `know.identified`. Brewing a canonical type identifies that type for the party (`ev.item_identified`).

### 12.4 Unknown potions (doc 10 integration)

- Every `cat.potion` type is in `idc.potion` (Canon §13.6); type knowledge is shared across genres and the party.
- **Drinking** an unknown potion classifies its type and identifies the instance.
- **Throwing** an unknown potion classifies its type if its splash applied an effect to any entity or its spill or gas caused an `ev.liquid_reaction`; otherwise the type stays unknown.
- **Alchemy appraisal hooks** for doc 10: at Alchemy 25 a held unknown potion shows its polarity (beneficial or harmful); at 50 also whether it is an aerosol; at 75 also its primary effect's kind (`instant`, `status`, `buildup`, `damage`).
- **Seed brewing** (§15.2) of one seed type identifies the produced type.
- **Price identification** uses the §12.1 values (doc 10, doc 11).

### 12.5 Wellsprings: unknown liquid effects

Doc 12 places font basins holding `liq.wellspring` with 1 to 3 uses. Each drink of at least 125 ml of wellspring (from a basin, a cell or a vessel) draws exactly 1 value: keyed `rng.alchemy` (actor = drinker, tick = drink completion tick, seq) `.weighted(w)`:

|Outcome|Weight|
|---|---|
|heal 3000 Bp|2000|
|aether +5000 Bp of max|1500|
|`st.blessed` 1800 ticks|1000|
|the carried unknown potion type with the lowest `id32` becomes classified|1000|
|2 stacks of `st.poisoned` (direct)|1200|
|60 `st.confused` buildup|800|
|one hostile from the local genre's swarm role spawns 3000 mm away (doc 13)|500|
|toxicity −20,000|1000|
|nothing|500|
|the font dries: its remaining uses become 0 and its liquid becomes `liq.water`|500|

Total weight 10000. A basin use is spent per drink; at 0 uses the basin's liquid becomes `liq.water`.

### 12.6 Cure potions and afflictions

- `alch.eff.cure` (in `alch.potion.cure`, `cleansing`, `panacea` and any brew) ends `st.poisoned`, `st.diseased`, `st.bleeding` and `st.incubating`; ending `st.incubating` aborts the incubating affliction before it takes hold (doc 04 emits `ev.affliction_changed`).
- A **blessed** cure cures one affliction that has taken hold, the one contracted most recently (ties to lower `id32`): Canon §16.4's "blessed cure potion". For `afflict.red_ink` it removes every dose and restores the hunter threshold to 10.
- Ways to bless: brew on `liq.holy_water` (§9.2), or consecrate (§15). Doc 06 altars and doc 11 loot supply holy water.

---

## 13. Poisons, bombs and alchemical grenades

### 13.1 Poisons

A poison is any brew whose effects are all harmful. Poisons are thrown (§10.2), coated (§10.3), injected (§10.5), vaporized (§10.4) or lingered (§10.6). Drinking one applies it to the drinker. These two-ingredient reference recipes produce only harmful effects; recipe books (doc 07 `recipe.`) teach them, and the brewed item is `alch.potion.draught`:

|Reference name|Ingredients|Effects|
|---|---|---|
|Viper's Kiss|`ing.serpent_venom` + `ing.hemlock`|paralyze, poison|
|Socrates' Cup|`ing.hemlock` + `ing.nightshade`|harm, poison|
|Heartstopper|`ing.foxglove` + `ing.hemlock`|paralyze, harm|
|Hellbrand|`ing.bombardier_gland` + `ing.saltpeter`|burn, expose|
|Grave Terror|`ing.brimstone` + `ing.demon_horn`|fear, wither|
|Galvanic Lockjaw|`ing.clockwork_spring` + `ing.lodestone`|shock, slow|
|Ink of Despair|`ing.inkcap` + `ing.kraken_ink`|blind, dread|
|Siren's Lullaby|`ing.fairy_wing` + `ing.siren_scale`|charm, confuse|

**Worked example (Viper's Kiss on oil).** Alchemy 40, ANL 50, FAT 40, T2 mortar: `f = 54`, `skill_bp = 10700`, `q_score = 10700 + v`; with `v = −200`, 10500: Normal. Both effects have `n = 2`, `pot = 11000`; paralyze (value 25) is primary. Poison power `60 → 66 → 66 → 66 → 59` (oil `mag_bp` 9000); paralyze `50 → 55 → 55 → 55 → 49`. Coated on a sword: `charges = mul_bp(5 + 2, 15000) = 10`. Each landed hit adds poison buildup `mul_bp(59, 5000) = 29` before Canon §11.2; at `ilvl 5` that becomes `mul_bp(29, S(5) = 11200) = 32` against a target with 0 toxin resistance and FAT-40 attacker, so 4 hits fill a 100 threshold (with no decay between hits).

### 13.2 Bombs (`alch.bomb.*`, `cat.throwable`)

Crafted with the §9.7 state machine, quality by §9.5, `ilvl` = Folio where made. Bombs shatter on impact (§6.3). Damage follows Canon §10.4 (`base_mhp → P(src_lvl) → 10000 → quality → 10000 → 10000`) as a non-crit area hit (`FF_AREA_BP`); status power follows Canon §11.2.

|ID|Recipe (shell = `apparatus.vessel.grenade_shell`)|Radius mm|Base damage, shares|Status power|Release|
|---|---|---|---|---|---|
|`alch.bomb.firebomb`|shell + 300 ml oil, alcohol or fuel|2000|6 HP fire|burning 40|spills the fill; the impact is a fire trigger on it (§5.2)|
|`alch.bomb.frost`|shell + 300 ml water + `ing.frost_salt`|2500|8 HP frost|frozen 60|spills water; frost trigger|
|`alch.bomb.acid`|shell + 300 ml acid|2000|6 HP acid|corroded 50|spills acid|
|`alch.bomb.shock`|shell + 300 ml brine + `ing.lodestone`|2500|8 HP shock|shocked 60|spills brine; shock trigger (electrifies)|
|`alch.bomb.blasting`|shell + 100 ml nitro + `ing.black_powder`|3000|40 HP: blunt 7000, fire 3000|none|block-break power 2 (doc 03); ignites flammable cells|
|`alch.bomb.smoke`|shell + `ing.black_powder` + `ing.charcoal`|0|none|none|8,000,000 ml `gas.smoke`|
|`alch.bomb.stink`|shell + 300 ml bile|0|none|none|4,000,000 ml `gas.miasma`; spills bile|
|`alch.bomb.sleep`|shell + `ing.dreamfoil` + `ing.poppy_tear`|0|none|none|4,000,000 ml `gas.soporific`|
|`alch.bomb.flash`|shell + 100 ml phosphor + `ing.black_powder`|8000|none|blinded 80 to entities with line of sight whose facing is within 16384 Bam of the burst|`light_block` 15 at the impact cell for 30 ticks|
|`alch.bomb.tar`|shell + 300 ml tar + `ing.puffball`|0|none|none|the tar foams to 3,000 ml, dumped over 3 × 3 cells (§6.2)|
|`alch.bomb.salt`|shell + 2 × `ing.blessed_salt`|2500|none|applies `st.unraveled` and `st.silenced` directly for 180 ticks: a dependency-none `counter.disruptor` source (Canon §17.3a)|none|
|`alch.bomb.incense`|shell + `ing.gum_resin` + `ing.blessed_salt`|0|none|none|4,000,000 ml `gas.incense`|
|`alch.bomb.infernal`|§15 recipe|3000|8 HP fire|burning 60|4,000,000 ml `gas.miasma`, then a fire trigger on it at the impact (Canon miasma blast)|
|`alch.bomb.grenade`|shell + any potion (30 ticks)|3000|the potion's effects as a splash (§10.2)|per potion|spills 300 ml of the potion's base|

**Worked example.** A Good firebomb made on Folio 5: `B = 6000 → mul_bp(6000, 14115) = 8469 → 8469 → mul_bp(8469, 11000) = 9315` mHP of fire, before resistance, to each target within 2 m; its spilled oil then ignites with its own Canon blast (300 ml: base 3,000 mHP), which is a separate, environmental source.

Doc 11 stocks `alch.bomb.salt` and `alch.bomb.smoke` in every genre's shops and loot (Canon §17.3a requires a dependency-none disruptor source everywhere).

---

## 14. Genre skins

Skins change names, meshes, animations and the dependency of `aether` effects (§7), never numbers.

|Genre|Potion class name|Delivery fiction|Typical base|Bomb skin|
|---|---|---|---|---|
|Mythic Fantasy|elixir, potion|glass flask|water, alcohol|alchemist's fire|
|Gothic Horror|tonic|apothecary bottle|wine, blood|vial of holy fire|
|Cosmic Horror|tincture, draught|stoppered specimen jar|sepia-black water|eldritch flask|
|Space Opera|serum, hypo|hypospray|nanite slurry|plasma canister|
|Cyberpunk|stim|auto-injector, inhaler|nanite slurry, alcohol|chem-grenade|
|Post-Apocalypse|chem|jet inhaler, syringe, bottle|water, fuel|molotov|
|Weird West|snake-oil tonic|medicine bottle|alcohol|dynamite flask|
|Hardboiled Noir|nostrum, pills, hooch|pill tin, hip flask|alcohol|bottle bomb|
|Wuxia|dan pill|pill swallowed; thrown pills burst into their essence|water, honey|thunder pellet|
|Steampunk|patent tonic|brass vial|alcohol, oil|clockwork grenade|
|Fairy Tale|fairy dew|dewdrop bead, acorn cap|nectar|witch's bottle|
|Mythic Antiquity|ambrosia, nectar draught|kylix, amphora|ichor, honey|Greek-fire pot|
|Oneiric|dream-draught|soap bubble|nectar|dream-burst|
|The Redaction|approved compound|numbered ampoule|water|containment canister|
|The Unwritten|blank draught|unlabeled glass|water|blank shell|
|The Ur-Text|word-draught|ink drop|water|full stop|

**Forbidden draughts** are any brew on `liq.red_ink` in any genre: skinned in red glass with a struck-through label, magnitudes and durations × 15000, and one `afflict.red_ink` dose per drink. They are never generated as loot (red ink comes only from Editors and their pools, §3.4).

---

## 15. Transmutation (`alch.tx.*`)

The Shattered Pixel Dungeon alchemy-pot analog. Every recipe needs a `station.alchemy` (doc 07), takes 60 ticks (§9.7 state machine and interruptions), consumes its inputs and `ing.essence`, and yields identified output with quality equal to the best input potion's quality (Normal when none) and `ilvl = max(input ilvl, current Folio index)`.

### 15.1 Alchemize

At a station, 30 ticks per item: `cat.potion` → 6 `ing.essence`; `cat.text` → 6; `alch.bomb.*` → 4; `cat.seed` → 2; `cat.ingredient` (except essence) → 1; `cat.food` → 1. Nothing else can be alchemized. Unknown items are accepted and stay unknown.

### 15.2 Seed brewing (`alch.tx.seed_brew`)

Three `cat.seed` items, 0 essence, keyed `rng.alchemy` (actor = brewer, tick = start tick, seq). `canon` is the 16 §12.1 IDs sorted by ID byte order: aether, calm, cure, fortitude, frost, haste, healing, invisibility, levitation, liquid_flame, mind_vision, night_eye, purity, slumber, stamina, toxic_gas (indices 0 to 15).

```rust
let k = distinct seed types among the 3;
let d1 = rng.range(10000); let d2 = rng.range(3); let d3 = rng.range(16);   // always 3 draws
let mut out = if k == 1 { seed_potion(seeds[0]) }
              else if d1 < (if k == 2 { 2500 } else { 5000 }) { canon[d3] }
              else { seed_potion(seeds[d2]) };                              // seeds sorted by id32
if out == HEALING {
    let d4 = rng.range(10000);                                             // 4th draw
    if d4 < min(10000, party.seed_heal_count * 1000) { out = canon[rng.range(16)]; }   // 5th draw
    if out == HEALING { party.seed_heal_count += 1; }
}
identify(out) only if k == 1;  // otherwise the potion is created unknown
```

Draw count: 3, 4 or 5. `seed_heal_count` is per run and per party (SPD's healing anti-farm).

**Worked example.** Two `ing.seed_sungrass` and one `ing.seed_icecap`: `k = 2`. With `d1 = 1873 < 2500`, the result is `canon[d3]`; with `d3 = 5` it is `alch.potion.haste`, created unknown. With `d1 = 6000` and `d2 = 2` (third sorted seed), it is that seed's potion.

### 15.3 Recipes

|ID|Inputs|Essence|Output|
|---|---|---|---|
|`alch.tx.seed_brew`|3 seeds|0|§15.2|
|`alch.tx.exotic_shielding`|`alch.potion.healing`|4|`alch.potion.shielding`|
|`alch.tx.exotic_adrenaline`|`alch.potion.stamina`|4|`alch.potion.adrenaline`|
|`alch.tx.exotic_shadowmeld`|`alch.potion.invisibility`|4|`alch.potion.shadowmeld`|
|`alch.tx.exotic_dragonfire`|`alch.potion.liquid_flame`|4|`alch.potion.dragonfire`|
|`alch.tx.exotic_snap_freeze`|`alch.potion.frost`|4|`alch.potion.snap_freeze`|
|`alch.tx.exotic_corrosive_gas`|`alch.potion.toxic_gas`|4|`alch.potion.corrosive_gas`|
|`alch.tx.exotic_cleansing`|`alch.potion.purity`|4|`alch.potion.cleansing`|
|`alch.tx.exotic_panacea`|`alch.potion.cure`|6|`alch.potion.panacea`|
|`alch.tx.elixir_honeyed_healing`|`alch.potion.healing` + 250 ml honey|2|`alch.potion.honeyed_healing`|
|`alch.tx.elixir_dragons_blood`|`alch.potion.liquid_flame` + `ing.wyrm_scale`|8|`alch.potion.dragons_blood`|
|`alch.tx.elixir_aquatic`|`alch.potion.healing` + `ing.siren_scale`|6|`alch.potion.aquatic_rejuvenation`|
|`alch.tx.elixir_arcane_armor`|`alch.potion.aether` + `ing.amber`|8|`alch.potion.arcane_armor`|
|`alch.tx.consecrate`|any `cat.potion` + 250 ml holy water|2|the same potion, `buc.blessed`|
|`alch.tx.uncurse`|a cursed `cat.potion` + `ing.blessed_salt`|1|the same potion, `buc.uncursed`|
|`alch.tx.holy_water`|250 ml water + `ing.blessed_salt`|1|250 ml `liq.holy_water`|
|`alch.tx.distill`|1500 ml wine; a retort is required|1|750 ml `liq.alcohol`|
|`alch.tx.nitro`|250 ml acid + 250 ml oil + `ing.saltpeter`|3|100 ml `liq.nitro` in a vial|
|`alch.tx.mana`|250 ml water + `ing.aether_crystal`|3|250 ml `liq.mana`|
|`alch.tx.coolant`|500 ml water + `ing.frost_salt`; a dewar is required|2|500 ml `liq.coolant` in the dewar|
|`alch.tx.lamp_oil`|1 ingredient tagged `fat`|1|250 ml `liq.oil`|
|`alch.tx.black_powder`|`ing.saltpeter` + `ing.charcoal` + `ing.brimstone`|0|3 `ing.black_powder`|
|`alch.tx.concentrate`|3 brewed potions of one type, one quality, identical effects|6|1 of them at the next quality tier; each magnitude and duration `x` becomes `min(cap, floor_div(x × new_q, old_q))`|
|`alch.tx.transmute_ingredient`|2 units of one effect ingredient|1|1 unit of the next ingredient in ascending ID byte order whose first effect equals this one's first effect (wrapping); refused when none exists|
|`alch.tx.infernal_bomb`|`alch.potion.toxic_gas` + `alch.potion.liquid_flame` + a grenade shell|4|`alch.bomb.infernal`|

---

## 16. Owned status and resource behavior

Canon §11.1 fixes each status's numbers; this chapter fixes the rest of their behavior.

- **`st.poisoned`**: built by `dmg.toxin` hits, poison effects, `gas.miasma`, quicksilver and overdose. Immune: `kin.construct`, `kin.undead`, and NPC templates doc 13 marks immune. Removed by `cure` (all stacks), milk (1 stack per 250 ml) and expiry.
- **`st.irradiated`**: built by `dmg.radiation`, irradiate effects, `liq.rad_sludge` and `gas.fallout`. Its Max HP factor uses the Canon §12 max-change rule on gain and loss of stacks. `detox` removes up to 3 stacks.
- **`st.soaked`**: applied by the contact rows that list it. Its shock × 15000 and fire × 5000 multiply buildup after the Canon §11.2 chain, in ascending status ID order (Canon §11.7). It ends `st.burning` on application; a `st.burning` buildup fill while soaked ends `st.soaked` instead and applies no burning.
- **`st.oiled`**: applied by oil, alcohol and fuel contact; fire buildup × 20000 as above. When the bearer gains `st.burning`, `st.oiled` ends and its feet cell takes a fire trigger (§5.5). Friction × 3000 belongs to the oil surface, not the status.
- **`st.focused`**: applied by `alch.eff.focus`; spread × 7000 and aim sway × 5000 (doc 05 consumes).
- **`res.toxicity`**: §11.

---

## Exports

Content IDs, all in namespaces Canon §19 assigns to doc 08 (`liq.`, `gas.`, `ing.`, `alch.`, `apparatus.`):

- **Liquids (35)**: `liq.acid`, `liq.alcohol`, `liq.bile`, `liq.blood`, `liq.brine`, `liq.coolant`, `liq.ectoplasm`, `liq.fuel`, `liq.hemolymph`, `liq.holy_water`, `liq.honey`, `liq.ichor`, `liq.lava`, `liq.mana`, `liq.milk`, `liq.molten_metal`, `liq.mud`, `liq.nanite_slurry`, `liq.nectar`, `liq.nitro`, `liq.oil`, `liq.phosphor`, `liq.quicksilver`, `liq.rad_sludge`, `liq.red_ink`, `liq.sap`, `liq.sepia`, `liq.slime`, `liq.swamp_water`, `liq.tar`, `liq.unholy_water`, `liq.venom`, `liq.water`, `liq.wellspring`, `liq.wine`.
- **Gases (13)**: `gas.alch_vapor`, `gas.caustic`, `gas.cold_fog`, `gas.fallout`, `gas.fuel_vapor`, `gas.incense`, `gas.laughing`, `gas.miasma`, `gas.smoke`, `gas.soporific`, `gas.spore`, `gas.steam`, `gas.terror`.
- **Ingredients (72 effect ingredients, 5 modifier reagents, `ing.essence`, `ing.black_powder`, 16 seeds)**: `ing.aether_crystal`, `ing.amber`, `ing.angel_feather`, `ing.ash_of_errata`, `ing.basilisk_eye`, `ing.black_gall`, `ing.black_powder`, `ing.blessed_salt`, `ing.blindweed`, `ing.bloodmoss`, `ing.bombardier_gland`, `ing.bright_dust`, `ing.brimstone`, `ing.charcoal`, `ing.cinnabar`, `ing.circuit_dust`, `ing.clockwork_spring`, `ing.coffee_bean`, `ing.demon_horn`, `ing.dreamfoil`, `ing.earthroot`, `ing.ectoplasm_wisp`, `ing.eldritch_eye`, `ing.essence`, `ing.eye_of_newt`, `ing.fadeleaf`, `ing.fairy_wing`, `ing.firebloom`, `ing.foxglove`, `ing.frost_salt`, `ing.ghoul_tongue`, `ing.ginseng`, `ing.glowshroom`, `ing.gum_resin`, `ing.hagberry`, `ing.hemlock`, `ing.icecap`, `ing.ink_lotus`, `ing.inkcap`, `ing.kraken_ink`, `ing.lich_dust`, `ing.lodestone`, `ing.lotus_seed`, `ing.mageroyal`, `ing.mandrake`, `ing.mender_root`, `ing.mint_leaf`, `ing.mutagen`, `ing.nanopaste`, `ing.nightshade`, `ing.ooze_core`, `ing.peach_of_immortality`, `ing.peyote_flesh`, `ing.phoenix_feather`, `ing.pitchblende`, `ing.poppy_tear`, `ing.puffball`, `ing.quicksalt`, `ing.redcap`, `ing.rotberry`, `ing.salamander_tail`, `ing.saltpeter`, `ing.seed_blindweed`, `ing.seed_dreamfoil`, `ing.seed_earthroot`, `ing.seed_fadeleaf`, `ing.seed_firebloom`, `ing.seed_ginseng`, `ing.seed_glowshroom`, `ing.seed_icecap`, `ing.seed_mageroyal`, `ing.seed_mender_root`, `ing.seed_poppy`, `ing.seed_sorrowmoss`, `ing.seed_starflower`, `ing.seed_stormvine`, `ing.seed_sungrass`, `ing.seed_swiftthistle`, `ing.serpent_venom`, `ing.siren_scale`, `ing.sorrowmoss`, `ing.spider_gland`, `ing.spore_sac`, `ing.starflower`, `ing.stim_gland`, `ing.stormvine`, `ing.sungrass`, `ing.swiftthistle`, `ing.troll_fat`, `ing.unicorn_hair`, `ing.vampire_dust`, `ing.vitriol`, `ing.wolf_heart`, `ing.wolfsbane`, `ing.wyrm_scale`.
- **Alchemical effects (58)**: `alch.eff.berserk`, `alch.eff.bless`, `alch.eff.blind`, `alch.eff.burn`, `alch.eff.charm`, `alch.eff.confuse`, `alch.eff.corrode`, `alch.eff.cure`, `alch.eff.detox`, `alch.eff.disease`, `alch.eff.drain_stamina`, `alch.eff.dread`, `alch.eff.empower`, `alch.eff.expose`, `alch.eff.fear`, `alch.eff.featherfall`, `alch.eff.focus`, `alch.eff.fortify`, `alch.eff.fortify_finesse`, `alch.eff.fortify_intellect`, `alch.eff.fortify_might`, `alch.eff.fortify_perception`, `alch.eff.fortify_vigor`, `alch.eff.freeze`, `alch.eff.harm`, `alch.eff.haste`, `alch.eff.heal`, `alch.eff.inspire`, `alch.eff.invisibility`, `alch.eff.irradiate`, `alch.eff.levitate`, `alch.eff.mind_vision`, `alch.eff.night_eye`, `alch.eff.paralyze`, `alch.eff.poison`, `alch.eff.purify`, `alch.eff.red_dose`, `alch.eff.regenerate`, `alch.eff.resist_acid`, `alch.eff.resist_fire`, `alch.eff.resist_frost`, `alch.eff.resist_radiation`, `alch.eff.resist_shock`, `alch.eff.resist_toxin`, `alch.eff.resist_umbral`, `alch.eff.restore_aether`, `alch.eff.restore_composure`, `alch.eff.restore_stamina`, `alch.eff.satiate`, `alch.eff.shock`, `alch.eff.silence`, `alch.eff.sleep`, `alch.eff.slow`, `alch.eff.unravel`, `alch.eff.ward`, `alch.eff.water_breathing`, `alch.eff.weaken`, `alch.eff.wither`.
- **Potion types (29)**: `alch.potion.adrenaline`, `alch.potion.aether`, `alch.potion.aquatic_rejuvenation`, `alch.potion.arcane_armor`, `alch.potion.calm`, `alch.potion.cleansing`, `alch.potion.corrosive_gas`, `alch.potion.cure`, `alch.potion.dragonfire`, `alch.potion.dragons_blood`, `alch.potion.draught`, `alch.potion.fortitude`, `alch.potion.frost`, `alch.potion.haste`, `alch.potion.healing`, `alch.potion.honeyed_healing`, `alch.potion.invisibility`, `alch.potion.levitation`, `alch.potion.liquid_flame`, `alch.potion.mind_vision`, `alch.potion.night_eye`, `alch.potion.panacea`, `alch.potion.purity`, `alch.potion.shadowmeld`, `alch.potion.shielding`, `alch.potion.slumber`, `alch.potion.snap_freeze`, `alch.potion.stamina`, `alch.potion.toxic_gas`.
- **Bombs (14)**: `alch.bomb.acid`, `alch.bomb.blasting`, `alch.bomb.firebomb`, `alch.bomb.flash`, `alch.bomb.frost`, `alch.bomb.grenade`, `alch.bomb.incense`, `alch.bomb.infernal`, `alch.bomb.salt`, `alch.bomb.shock`, `alch.bomb.sleep`, `alch.bomb.smoke`, `alch.bomb.stink`, `alch.bomb.tar`.
- **Reactions (16)**: `alch.rx.acid_milk`, `alch.rx.acid_slime`, `alch.rx.bile_water`, `alch.rx.coolant_lava`, `alch.rx.coolant_water`, `alch.rx.holy_ectoplasm`, `alch.rx.holy_red_ink`, `alch.rx.holy_unholy`, `alch.rx.honey_alcohol`, `alch.rx.ichor_unholy`, `alch.rx.mana_water`, `alch.rx.metal_water`, `alch.rx.nanite_rad`, `alch.rx.red_ink_blood`, `alch.rx.venom_milk`, `alch.rx.water_lava`.
- **Transmutations (25)**: `alch.tx.black_powder`, `alch.tx.concentrate`, `alch.tx.consecrate`, `alch.tx.coolant`, `alch.tx.distill`, `alch.tx.elixir_aquatic`, `alch.tx.elixir_arcane_armor`, `alch.tx.elixir_dragons_blood`, `alch.tx.elixir_honeyed_healing`, `alch.tx.exotic_adrenaline`, `alch.tx.exotic_cleansing`, `alch.tx.exotic_corrosive_gas`, `alch.tx.exotic_dragonfire`, `alch.tx.exotic_panacea`, `alch.tx.exotic_shadowmeld`, `alch.tx.exotic_shielding`, `alch.tx.exotic_snap_freeze`, `alch.tx.holy_water`, `alch.tx.infernal_bomb`, `alch.tx.lamp_oil`, `alch.tx.mana`, `alch.tx.nitro`, `alch.tx.seed_brew`, `alch.tx.transmute_ingredient`, `alch.tx.uncurse`.
- **Apparatus, vessels and respirators (19)**: `apparatus.alembic`, `apparatus.calcinator`, `apparatus.mortar`, `apparatus.respirator.cloth`, `apparatus.respirator.mask`, `apparatus.respirator.rebreather`, `apparatus.retort`, `apparatus.vaporizer`, `apparatus.vessel.ampoule`, `apparatus.vessel.bottle`, `apparatus.vessel.bucket`, `apparatus.vessel.canteen`, `apparatus.vessel.cask`, `apparatus.vessel.crucible`, `apparatus.vessel.dewar`, `apparatus.vessel.flask`, `apparatus.vessel.grenade_shell`, `apparatus.vessel.vial`, `apparatus.vessel.waterskin`.

Sub-namespaces of `alch.`: `alch.eff.` effects, `alch.potion.` potion item templates (`cat.potion`), `alch.bomb.` bomb item templates (`cat.throwable`), `alch.rx.` reactions, `alch.tx.` transmutations. Sub-namespaces of `apparatus.`: tools (`cat.tool`), `apparatus.vessel.` (`cat.container`), `apparatus.respirator.` (`slot.face`).

Rules, data and interfaces:

- Constants of §1.2; liquid schema (§2.3), payload record (§2.9), ice record (§2.8), gas cell (§4.1).
- Functions for other docs: `fill_height_mm(cell)`, `surface_height_above_feet(e)`, `eye_submerged(e)`, `is_flammable(cell)`, `is_conductive(cell)`, `inject(cell, gas, payload, ml)`, `spill(cell, comps)`, `fire_trigger(cell)`, `shock_trigger(cell)`, `frost_trigger(cell)`, `radiant_trigger(cell)`, `conceal_liquid_gas(cell)`, `light_emission(cell)`.
- `ev.liquid_reaction` payload (doc 03 bus): `{ cell: (i32, i32, i32), trigger: u8, rx: u32 /* alch.rx id32 or 0 */, inputs: [(u32, i64)], outputs: [(u32, i64)], block: u32 /* 0 if none */ }`.
- `ev.potion_brewed` payload: `{ brewer: u64, items: [u64], effects: [u32], quality: u32, doses: u8, reveals: u8 }`.
- Item instance extension (doc 19 schema): `alch: { base: u32, effects: [{eff, mag, dur, n}], tox_milli: i64, mods: u8 bitset (extend, amplify, invert, aerosol, linger), forbidden: bool }` on `cat.potion`, `alch.bomb.*` and coated weapons (`coat: {effects, charges, expires, src_lvl, quality_bp, potency}`); `filter: u32` on respirators; doses as `charges` on multi-dose vessels.
- Per-entity state: `red_ink_contact_ticks`, `dr: BTreeMap<u32, (u8, u64)>`, contact accumulators; per-party run state: `ing_known`, `seed_heal_count`.
- Behavior of `res.toxicity`, `st.poisoned`, `st.irradiated`, `st.soaked`, `st.oiled`, `st.focused` (§16).

## Interface assumptions

- **Doc 03**: stores liquid, gas and ice-record state per chunk; runs `liquids_step` and `gases_step` on the Canon §2.2 schedule over its active cells; gives a deterministic 6-neighbor order (BFS) and per-step budgets, truncating in ascending `(cy, cz, cx, local index)` and carrying the rest over; provides `temp_at(cell)`, heat sources, `ignite_block`, `is_burning(block)`, smoke via `inject`, block placement with `ev.block_changed`, explosion block breaking, and the event bus.
- **Doc 04**: Alchemy tier perks match §8.5 and §9.7 (reveals at 25/50/75, batch at 50 and 100, single-ingredient brewing at 100, two reveals per taste at 50); the XP table consumes §9.11; `add_affliction_dose` and `cure_affliction`; kin immunities; satiation; Survival butchering.
- **Doc 05**: throw and impact for vessels and bombs; explosion line of effect; the on-hit hook for coats; ammo stack splitting; wading × 7000 and swimming (§2.2); spread and aim sway for `st.focused` and the sickened band; `counter.disruptor`.
- **Doc 06**: holy water from altars and prayers; `st.regenerating`, `st.shielded`, `st.invisible`, `st.levitating`, `st.blessed`, `st.hasted`, `st.empowered`, `st.fortified` behavior.
- **Doc 07**: material tags `absorbent`, `acid_soluble`, `flammable`, `fat`; block HP and damage; blocks `block.ice`, `block.web`, `block.obsidian`, `block.basalt`, `block.slag`; `station.alchemy` with apparatus slots; recipe books (`recipe.`) teaching §13.1 poisons; seed planting; charcoal from wood.
- **Doc 09**: consumes liquid and gas conceal, `light_at` override from phosphor, mind-vision reveal, shatter noise, respirator hearing penalty.
- **Doc 10**: at least 32 `idc.potion` appearance descriptors; knowledge transitions on drink, throw, brew and seed brew (§12.4); alchemy appraisal hooks; price identification from §12.1 values; BUC generation rates for found potions.
- **Doc 11**: loot and shop tables for every exported item; values × `P(ilvl)`; `alch.bomb.salt` and `alch.bomb.smoke` in every genre's shop and loot pool.
- **Doc 12**: GPV fields `liquid_viscosity_bp` and `gas_dissipation_bp` (default 10000); placement of liquid pools, font basins (1 to 3 uses), red-ink pools, alchemy labs, and harvest and ore nodes.
- **Doc 13**: creature `blood_liq` and tags (undead, fiend, divine, eldritch, plant, construct); ingredient drop tables by tag; enemy gas abilities using `inject`; AI avoidance of harmful surfaces and gases.
- **Doc 16**: replicates liquid, gas and payload state as chunk deltas.
- **Doc 17**: brewing UI with preview (known effects plus "N unknown"), toxicity meter with bands, respirator filter gauge.
- **Doc 18**: text keys for every exported ID, liquid adjectives `loc.<liq>.adj`, genre skin names of §14.

## Canon gaps

1. **Gas contact threshold.** Canon §12.1 gives no volume for "the eye cell holds the gas". Assumed: `GAS_CONTACT_ML = 50,000`.
2. **Gas conceal scaling.** Canon §12.2 scales smoke linearly by density and lists steam as 3. Assumed: every gas, steam included, scales linearly by density from its full-cell value.
3. **Frost on shallow liquid.** A 1 m³ `block.ice` from a 100 ml puddle would wall corridors. Assumed: cells under 200,000 ml become a frozen film with the `block.ice` contact row for the same 1200 ticks; deeper cells become `block.ice` as Canon §12.1 says.
4. **Ice melting.** Canon says fire melts ice "back to water". Assumed: ice created from a liquid stores and restores that liquid (water in the Canon case); natural `block.ice` melts into 917,000 ml of water.
5. **Fire on water, quantity.** Canon gives the ratio (1 cell per 1000 ml) but no cap. Assumed: at most `STEAM_CONVERT_CAP_ML = 5,000` ml per trigger cell per trigger.
6. **Oil and alcohol ignition.** Canon omits cells, consumption and repetition. Assumed: one blast per igniting cell, then burning (§2.7); an entity takes at most one liquid or gas blast per step and none for 30 ticks after (§5.6).
7. **Trigger threshold.** Canon does not say how much of a damage type triggers §12.1. Assumed: a type share of at least 2500 Bp; DoT pulses never trigger.
8. **Surface buildup source level.** Canon §11.2 scales hit buildup by `S(src_lvl)`; surfaces have no hit. Assumed: surfaces are environmental (`src_lvl = d_current`, Canon §13.8.1), with resistance and no attacker multipliers.
9. **Supernatural alchemy dependency.** Canon §5.3 lists supernatural alchemical effects as `arcana` by default, and says dependencies come from the skin. Assumed: they follow the spell-glyph rule, `{origin_genre.aether_axiom}` (arcana when none), so dan pills depend on VERVE and stims on TECHNE. Mythos and verve effects stay fixed.
10. **Cursed consumables.** Canon §13.6 defines blessed consumables (× 12500) only. Assumed: cursed potions have beneficial effects × 7500 and toxicity × 15000, and carry no curse affix.
11. **Uncursed cure and incubation.** Canon §16.4 names a blessed cure potion for afflictions. Assumed: uncursed `cure` also ends `st.incubating` (aborting the affliction before it takes hold); only a blessed cure removes an affliction that has taken hold.
12. **Item template namespaces.** Canon §13.9 allows any template ID; `item.` belongs to doc 11. Assumed: potions use `alch.potion.`, bombs `alch.bomb.`, apparatus, vessels and respirators `apparatus.`, since `cat.potion`, `cat.ingredient` and `cat.container` are doc 08's.
13. **Respirator archetype.** Canon §13.2 lets `slot.face` hold respirators but no doc owns their archetype; armor archetypes are doc 05's. Assumed: `apparatus.respirator.*`, because their function is gas filtering. Doc 05 aliases them if it claims them.
14. **Blood contact.** Canon §12.1 names blood in triggers but gives it no contact row. Assumed: none.
15. **Brewing and Contradiction.** Canon §5.5 lists Contradiction failure for docs 05, 06 and 07 activations. Assumed: potion use is an activation of the same kind, rolled with keyed `rng.combat` for the primary effect only.
16. **Body cells.** Canon §12.1 says "a body cell" without defining it. Assumed: every cell the entity's feet-to-eye segment passes through (§2.2).

## Acceptance tests

1. **Bucket spill.** 10,000 ml `liq.water` poured into one floor cell with 4 open floor neighbors, 15 °C, viscosity 10000: after liquids step 1 the center holds 3,598 and each neighbor 1,598; after step 6 the center holds 2,000, each neighbor 1,985, diagonals 0, total 9,940 (§2.10).
2. **Conservation.** 10,000 ml `liq.oil` (no evaporation) in the same layout: step 1 moves 720 ml to each neighbor; the cell total over the 5 cells is exactly 10,000 after each of 100 steps.
3. **Order independence.** Running any liquids or gases step with the active-cell list reversed yields a byte-identical grid.
4. **Component split.** Moving 500 ml from a cell holding water 700, blood 200, oil 101 moves water 351, blood 99, oil 50.
5. **Mixture parameters.** 6,000 ml water plus 2,000 ml honey give `spread = 7875`, `film = 6500`.
6. **Contact threshold.** 99 ml of water in the feet cell applies nothing on the next status pulse; 100 ml applies `st.soaked`.
7. **Frost, deep and shallow.** A frost hit on a cell with 300,000 ml water makes `block.ice` with an ice record; 1,200 ticks later the cell holds 300,000 ml water again. A frost hit on 150 ml of blood makes a frozen film whose contact multiplies ground acceleration and friction by 1500.
8. **Fire on water.** A fire hit on 3,000 ml water leaves 0 ml and injects 3,000,000 ml `gas.steam` into 3 cells. On 8,000 ml it boils 5,000 and leaves 3,000.
9. **Oil ignition.** Fire on 2,500 ml oil on Folio 5: an Erratum in 2 m takes `D2 = 30,970` mHP at step 3, an NPC 35,287. On 6,000 ml the base is capped at 50,000 mHP.
10. **Blast merge.** Two oil cells 1 m apart ignite in the same step with bases 25,000 and 10,000; an Erratum inside both radii takes only the 25,000 blast; a miasma blast resolved within the next 30 ticks does not hit it; one resolved 36 ticks later does.
11. **Electrify cap.** A straight line of 100 cells holding 1,000 ml water each, shock hit at cell 0: cells 0 to 63 have `electrified_until = t + 180`, cell 64 is untouched, and an entity standing in cell 10 gains `mul_bp(30, S(d))` `st.shocked` buildup per pulse (0 shock resistance).
12. **Lava and water.** A cell with 6,000 ml water and 120,000 ml lava becomes `block.obsidian` in one liquids step, injects 2,000,000 ml steam, and pushes 4,000 ml water and 20,000 ml lava into the cell above.
13. **Holy water and red ink.** 3,000 ml holy water and 1,000 ml red ink in one cell become 2,000 ml water and 2,000 ml holy water in one step. Radiant damage on a cell of 1,000 ml red ink leaves 1,000 ml water.
14. **Red ink erasure.** An Erratum standing in red ink for 600 consecutive ticks gains exactly 2 `st.erasure` stacks and 2 `ev.erasure_stack` events; a `fac.editors` member gains none.
15. **Gas contact and respirator.** Eye cell with 49,999 ml `gas.miasma`: no buildup and no breath drain. With 50,000 ml: `mul_bp(20, S(d))` `st.poisoned` buildup per pulse (0 toxin resistance) and breath drains. With a cloth respirator at filter 1,800: no buildup, no drain, filter 0 after 1,800 ticks, and buildup resumes on the next pulse.
16. **Steam DoT.** An 85 HP Erratum with 0 fire resistance whose eye cell holds full steam for 60 ticks loses exactly 425 mHP over 4 pulses.
17. **Splash.** A thrown 250 ml flask leaves 126 ml in the impact cell and 31 ml in each open neighbor.
18. **Quality boundary.** Alchemy 20, ANL 46, FAT 40, T1 mortar: `q_score = 8685 + v`; `v = 317` gives 9002 (`q.normal`), `v = 314` gives 8999 (`q.poor`). The brew draws exactly one `rng.alchemy` value.
19. **Apprentice healing.** §9.10's brew yields 1 `alch.potion.healing` with heal 3563 Bp, regenerate 630 ticks, toxicity 22,000, and the party gains type knowledge of `alch.potion.healing`.
20. **Master haste.** §9.10's brew yields 4 doses: haste 3452 ticks, shock `base_mhp` 2875 and shocked power 21, restore stamina 9487 Bp, toxicity 13,260, value 93; the effect order is haste, shock, restore stamina.
21. **Murky draught.** `ing.mint_leaf` + `ing.hemlock` share nothing: the brew yields `alch.potion.draught` with no effects, toxicity 3,000, value 0, and reveals nothing.
22. **Invert.** `ing.bloodmoss` + `ing.sungrass` + `ing.black_gall` yields one effect, `alch.eff.harm` with `n = 2`.
23. **Cancelled brew.** A brew interrupted by non-DoT damage at `start + 60` consumes nothing and draws nothing.
24. **Reveal.** At Alchemy 24 a player sees 1 effect per ingredient, at 25 two. Tasting `ing.mender_root` at Alchemy 10 with nothing known sets bit 0, then bit 1 on the second taste; at Alchemy 50 the first taste sets bits 0 and 1. Each taste adds 2,000 toxicity.
25. **Toxicity and overdose.** At Alchemy 20 a healing potion applies 11,400 toxicity. At 95,000 toxicity, drinking it deals 5,440 mHP `dmg.toxin` to an 85 HP Erratum and leaves toxicity at 100,000; at 100,000 the bearer gains 3 `st.poisoned` stacks every 300 ticks.
26. **Diminishing returns.** Four Normal healing potions drunk 600 ticks apart heal 3000, 2250, 1500 and 900 Bp of Max HP.
27. **Contradiction.** The §9.9 invisibility potion drunk in Hardboiled Noir by a Fantasy-origin Erratum with ATN 40 has C = 4, rolls one keyed `rng.combat` failure value at 1000 Bp, and on success lasts 540 ticks.
28. **Coat.** Viper's Kiss on oil (§13.1) gives 10 charges; each landed hit at `ilvl 5` adds 32 poison buildup against 0 resistance, so the 4th hit applies one `st.poisoned` stack at threshold 100.
29. **Firebomb.** A Good firebomb made on Folio 5 deals 9,315 mHP fire before resistance.
30. **Seed brew.** Three `ing.seed_sungrass` with `seed_heal_count = 0` yield an identified `alch.potion.healing` after exactly 4 draws and set the count to 1; the same brew with count 1 rerolls when the 4th draw is below 1000 (5 draws).
31. **Seed index.** `canon[5]` is `alch.potion.haste`.
32. **Wellspring.** The §12.5 weights sum to 10,000 and each qualifying drink draws exactly one value.
33. **Cure.** An uncursed cure ends `st.incubating` and the affliction never takes hold. A blessed cure on a bearer with 3 `afflict.red_ink` doses removes all of them and the hunter threshold returns to 10.
34. **Bleeding and death spill.** A bearer with 3 `st.bleeding` stacks adds 75 ml of its blood liquid per pulse; a 75,000 g humanoid corpse adds 1,500 ml.
35. **Vessel mass.** A water flask weighs 400 g; an alcohol bottle of 750 ml weighs `400 + 592 = 992` g.
36. **ID hygiene.** Every exported ID matches the Canon §1.1 grammar, is at most 64 bytes, and the 304 exported IDs have distinct `fnv1a32` values (`fnv1a32("liq.water") = 0x2d2df52c`).
37. **Integer content.** Every liquid, gas, ingredient, effect, reaction, transmutation and apparatus record passes the Canon integer-only content validator.

## Invariants annex

```
Σ 08: liquids ⊗ gases ⊗ reactions ⊗ ingredients ⊗ brewing ⊗ delivery ⊗ toxicity
W! liquid volume ∈ Ml (i64) ∧ 0 ≤ total(cell) ≤ CELL_CAP_ML        N! f32|f64 in liquid, gas or alchemy state
W! spread ∧ fall ∧ diffusion := double-buffered (Jacobi)            N! result depends on cell visit order
W! Σ volume conserved by flow; changes only by react|boil|evap|burn|absorb|trace|displace-overflow
W! automata ∧ reactions ∧ contact draw 0 random values              W! brew draws exactly 1 keyed rng.alchemy value
W! ≤ 4 liquid comps/cell ∧ ≤ 3 gas comps/cell                       N! component cap exceeded by any transfer
W! contact := comp ≥ 100 ml in feet|body cell ∨ gas ≥ 50,000 ml in eye cell
W! Canon §12.1 rows implemented unchanged                          N! canon surface or trigger altered
W! entity takes ≤ 1 liquid|gas blast per step, then 30 ticks immune
W! electrify ≤ 64 cells for 180 ticks                               W! frost ice lasts 1200 ticks then reverts
W! shared effect ⇔ n(e) ≥ 2 (n = 1 only at skill 100)               W! ≤ 4 selected effects (+ red_dose on red ink)
W! quality uses base ANL, FAT                                       N! fortify effects feed brewing power
N! random failure destroys ingredients                             W! interrupted brew consumes nothing
W! healing ∧ ward ∈ Bp of recipient Max HP                         W! damage effects follow Canon §10.4 chain
W! supernatural effect dep := {origin.aether_axiom} | mythos | verve  W! mundane effect dep = ∅
W! beneficial splash → fac.errata only                              W! harmful area effects × FF_AREA_BP on Errata
W! res.toxicity ∈ 0..=100,000 ∧ −500/s                              W! excess over cap → toxin damage
W! DR per (recipient, effect) within 1800 ticks                     N! thrown|coated|vapor effects add toxicity
W! brewed items know.identified ∧ brewing canonical type ⇒ type identified
W! seed brew of 1 type identifies; mixed types create unknown       W! respirator blocks inhaled rows only
N! carried vessels shatter from hits                                N! red ink appears in generated loot
W! iteration order := ascending EntityId | uid | cell order         N! HashMap|HashSet in 08 state
```
