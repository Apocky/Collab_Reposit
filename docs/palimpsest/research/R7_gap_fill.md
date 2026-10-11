# R7: Completeness critique and gap-fill (shooting math, run and co-op scaling, real-time melee, voxel physics, Barony crafting, 2026 status)

As of 2026-10-11. Research dossier R7 for PALIMPSEST. Role: completeness critic over R1 to R6, then gap-filler. Units follow `spec/00_CANON.md`: Bp (10000 = ×1.00), 60 Hz ticks, mm, mHP.

## Table of contents

0. Method, evidence tags and access notes
1. Completeness audit of R1 to R6
   - 1.1 Coverage matrix: required systems
   - 1.2 Coverage matrix: inspirations
   - 1.3 Ranked gap list (what this file fills, what stays open)
   - 1.4 Cross-dossier conflicts the specs must resolve
2. Shooting math (the thinnest required system)
   - 2.1 Spread, accuracy pool and recoil (Borderlands 1, exact table)
   - 2.2 Damage falloff and ramp-up (Team Fortress 2)
   - 2.3 Critical and precision formulas (Borderlands 2, Borderlands 3, Warframe, TF2)
   - 2.4 Level scaling of weapon damage (Borderlands 1 and 3) and the canon P(d) cross-check
   - 2.5 Projectile ballistics (Minecraft arrows) converted to 60 Hz
   - 2.6 Proc coefficients (Risk of Rain 2)
   - 2.7 Aim assist: what could not be verified
3. Roguelite run scaling, spawn directors and item stacking (Risk of Rain 2)
4. Co-op scaling across shipped games (Deep Rock Galactic, Risk of Rain 2, Borderlands 3) versus canon §17.2
5. Real-time melee and armor (Minecraft Java)
6. Voxel world physics: breaking, explosions, fluids, structural integrity
7. Barony's missing systems: Tinkering crafting and Alembic alchemy
8. Procedural level generation numbers from a shipped roguelike (Shattered Pixel Dungeon)
9. 2026 status updates and verifications of earlier open questions
10. What remains unverified
11. Design implications for PALIMPSEST

---

## 0. Method, evidence tags and access notes

**Tags.** `[VERIFIED <url>]` means I fetched the page (or its MediaWiki wikitext through `api.php?action=parse&prop=wikitext`) this session and read the claim. `[VERIFIED-SRC <url>]` means I read the source file this session. `[COMPUTED]` means arithmetic I ran this session (Python) on verified inputs; the inputs are named. `[RECALLED]` means memory, not checked. `[PROPOSED]` marks a PALIMPSEST design value, never a fact about a shipped game. `[XREF Rn §x]` points to a sibling dossier.

**Access notes.** The shared web-search budget for this turn was exhausted before this critic pass started. All facts below therefore come from **direct fetches of known primary URLs**: wiki pages through their MediaWiki API (riskofrain2.wiki.gg, minecraft.wiki, borderlands.fandom.com, valheim.fandom.com, barony.wiki.gg, deeprockgalactic.wiki.gg, wiki.warframe.com, fortnite.fandom.com, destiny.fandom.com), wiki.teamfortress.com, raw GitHub source for Shattered Pixel Dungeon, GitHub release pages, Wikipedia and pegi.info. Destiny 2 weapon-stat pages (falloff, precision, aim assist) do not exist on the reachable wiki, and Fandom's HTML front end returned HTTP 402 to WebFetch. The API route worked.

---

## 1. Completeness audit of R1 to R6

### 1.1 Coverage matrix: required systems

Depth: 3 = exact formulas from code or primary wiki, multiple precedents; 2 = solid but missing a key sub-area; 1 = thin or mostly recalled; 0 = absent.

| Required system | R1 | R2 | R3 | R4 | R5 | R6 | Depth before R7 | Missing before R7 | Filled here |
|---|---|---|---|---|---|---|---|---|---|
| Crafting | IVAN materials, CoQ mods | MC anvil, enchanting | — | — | RimWorld, DF, MC, Valheim (recalled), D2 | Techno-Wizardry | 2 | building and structural integrity; Barony's tinkering (an inspiration's craft system) | §6.4, §7.1 |
| Alchemy | SPD energy, seeds | MW, OB, MC brewing | — | — | MW code, Noita, Qud, Witcher, DD curios | — | 3 | Barony Alembic | §7.2 |
| Shooting | Barony ranged skill | BL4 parts, elements, rarity; D2 tiers | — | lag compensation | Fallout to-hit, armor models, hit locations | Wizard with a Gun | **1** | spread and recoil model, damage falloff, crit and precision math, weapon level scaling, projectile ballistics, proc coefficients, aim assist | §2 |
| Magic and spellcrafting | Barony 5.0 | MW, OB | — | — | Noita, Arx, Ars Magica | Torg backlash | 3 | — | — |
| Attributes and sub-attributes | Barony, Qud, IVAN | MW, OB, BG3, D2 Armor 3.0 | — | — | GURPS, Fallout, DF, Kenshi, PoE | — | 3 | — | — |
| Stealth | SPD v4 | Oblivion detection, BG3 | — | sound grid | MW code, Thief, MotN, Shadow Tactics, Hitman, MGS | — | 3 | three competing models (see §1.4) | §1.4 |
| Identification and appraisal | Barony Lore, SPD, NetHack, Qud, IVAN | MW, OB | — | — | NetHack odds, SPD use-ID, Diablo | Isekai | 3 | — | — |
| Gacha | — | — | Genshin family, Endfield, hazard model | backend | — | — | 3 | — | — |
| Loot boxes | ExpPD bags | BL2 slot machine | Overwatch counters | Steam Inventory grammar | DD curios | DCC boxes | 3 | in-run gamble precedents with escalating cost | §3.6 |
| Battle passes | — | BL4 Vault Card, D2 | Fortnite, CoD, Apex, Marvel Rivals, Helldivers, Halo | — | — | Fortnite seasons | 2 | XP per tier, 2026 Fortnite structure | §9 |
| Omni-genre structure | — | — | — | — | — | Torg code, Cypher, Noita, 20 genres | 3 | — | — |
| 3D tech | — | Oblivion Remastered split | — | Bevy 0.20, voxels, GI | — | Spider-Verse | 3 | — | — |
| Co-op netcode | Barony | — | co-op norms | full parameter table | — | — | 3 | co-op **gameplay** scaling (enemy HP, count, damage per player) from shipped games | §4 |
| Procgen | — | MC light and spawn | — | WFC, Dormans, Qud | — | Noita bleed modifiers | 2 | per-floor room, mob, item and trap counts from a shipped roguelike | §8 |
| Roguelike run structure | SPD, Barony, ExpPD | — | Nightreign, run lengths | — | — | DCC timers | 2 | a closed-form run difficulty curve; spawn budget director | §3 |

### 1.2 Coverage matrix: inspirations

| Inspiration | Covered by | Depth before R7 | Gap | Filled here |
|---|---|---|---|---|
| Barony | R1 (source code, 5.0) | 3 | Tinkering crafting, Alembic alchemy, 2026 status of "Part Two" | §7, §9 |
| Shattered Pixel Dungeon | R1, R5 (source) | 3 | level generation numbers | §8 |
| Experienced Pixel Dungeon | R1 | 2 | 2026 status | §9 |
| Oblivion | R2, R5 | 3 | Remastered enemy-scaling curve (still unpublished) | — |
| Morrowind | R2, R5 (OpenMW code) | 3 | — | — |
| Baldur's Gate | R2 (BG3 verified; BG1/BG2 **recalled**) | 2 | BG1/BG2 basics unverified | §9 |
| Caves of Qud | R1, R5, R6 | 3 | 2026 status | §9 |
| IVAN | R1 (source) | 3 | — | — |
| Borderlands | R2 (BL4) | 2 | gun-stat math (accuracy, crit, level scaling), co-op loot modes | §2, §4 |
| Minecraft | R2 (light, spawn, enchant, anvil, brewing, XP) | 2 | block breaking, explosions, fluids, melee cooldown, armor, arrows | §5, §6 |
| Destiny 2 | R2 (Armor 3.0, tiers, Portal, end of service) | 2 | weapon falloff, precision multipliers, aim assist | **not fillable this session** (§2.7, §10) |

### 1.3 Ranked gap list

| Rank | Gap | Why it matters | Status after R7 |
|---|---|---|---|
| 1 | Shooting math | Doc 05 owns shooting. Canon already references spread, recoil, precision and "two spread draws per projectile" but no dossier gives a shipped spread/recoil/falloff model | **Filled** with BL1, TF2, BL2/BL3, Warframe, Minecraft and RoR2 exact rules (§2) |
| 2 | Run difficulty curve and spawn budget | Roguelite pacing in real time needs a director, not only depth tables | **Filled** with RoR2 coefficient, Director credits, elite costs (§3) |
| 3 | Co-op gameplay scaling | Canon §17.2 has no cited precedent | **Filled** with DRG hazard tables and RoR2; canon cross-checked (§4) |
| 4 | Voxel breaking, explosions, fluids, structural integrity | The brief demands a destructible and buildable Minecraft-like world | **Filled** with Minecraft, Valheim (from game code via wiki module), 7 Days to Die (§6) |
| 5 | Real-time melee timing and armor | Canon's pipeline has no swing-cooldown rule | **Filled** with Minecraft Java cooldown and armor formulas (§5) |
| 6 | Barony crafting and alchemy | Barony is the closest precedent and its craft systems were absent | **Filled** (§7) |
| 7 | Level-gen counts | Folio generator needs room, mob, item and trap budgets | **Filled** with SPD source (§8) |
| 8 | 2026 status checks | Several "as of 2026" claims were recalled or single-source | **Filled** for Barony, SPD, ExpPD, Qud, PEGI, Fortnite, BG1/BG2 (§9) |
| 9 | Destiny 2 weapon stats and aim assist | Controller play on Steam Deck needs aim assist | **Open** (§2.7, §10) |

### 1.4 Cross-dossier conflicts the specs must resolve

These are contradictions between sibling recommendations. Each spec must pick one, and the brief and canon win any tie.

| # | Conflict | Dossiers | Resolution recommended |
|---|---|---|---|
| C1 | Kinetic damage model: R1 #15 says CoQ penetration dice; R5 #33 rejects it and keeps canon ratio armor | R1, R5, canon §10.5 | **Canon ratio armor wins.** Canon §10.5 step 5 already fixes it. R1 #15 is void. |
| C2 | Stealth: R1 #16 ports SPD v4 per-tick probabilities; R2 #3 ports Oblivion's continuous score; R5 #6–#13 builds a per-pair meter with Shadow Tactics zones | R1, R2, R5 | **R5's meter is the base** (in canon units, has every state). Import Oblivion's 0/−20/−50 hysteresis as R5 already does, and use SPD's cone-dependent rates only as the zone-rate source. |
| C3 | Spell cost exponent on duration: R2 #7 `Dur^0.9`; R5 #22 `Dur^1` | R2, R5 | Oblivion's verified formula uses `Dur^1` [XREF R2 §2.7]. **Use `Dur^1`.** The 0.9 is an unsourced tweak. |
| C4 | Alchemy potency curve: R2 #15 Oblivion power law `^(1/2.28)`; R5 #14 Morrowind linear `5000 + 50f` | R2, R5 | **Pick R5's linear version.** It is integer-friendly and has no random failure. Keep Oblivion's 1/2/3/4 effect reveal by mastery. |
| C5 | Gacha naming: R3 calls the gacha "Scriptorium"; the brief names the gacha **Inkwell** and the rewriting room **Scriptorium** | R3, brief §2.4, §8 | **Brief wins:** Inkwell = gacha. R3's numbers carry over unchanged. |
| C6 | Fortune Engine table: R2 #19 proposes a BL2-style table with 2,067 Bp "nothing" and 770 Bp "Ink Blot" hostile spawn | R2, brief §8 | Brief §8: "Every capsule pays out an item. They have no wagering, no payout multipliers and no casino presentation." **The "nothing" row is forbidden.** A hostile-spawn row is allowed only if an item still pays out. |
| C7 | Enemy scaling: R2 #13 `HP × (1 + 0.12 × (depth−1))`; canon uses `H(d)` | R2, canon §13.8 | **Canon wins.** R2's curve was a placeholder. |
| C8 | Hunger clock: R1 #17 quotes Barony's 1 point per 30 ticks at **50 Hz** | R1, canon §2 | At 60 Hz the same real-time rate is **1 point per 36 ticks** (1.667 per second). |
| C9 | Enchant reroll cap: R2 #18 "hard cap 39 ink-levels"; R5 #26 "7th reroll refused" | R2, R5 | Both are Minecraft-derived. **Use R5's count cap (7th refused)**, which is legible, and drop the level-sum cap. |

---

## 2. Shooting math (the thinnest required system)

### 2.1 Spread, accuracy pool and recoil (Borderlands 1, exact table)

Borderlands models hip-fire inaccuracy as a **static cone plus a dynamic pool** [VERIFIED https://borderlands.fandom.com/wiki/Accuracy]:

- "The full angle of the firing cone (far side to far side) equals *spread* + *accuracyPool* when firing from the hip, and just *spread* when aiming down the sights."
- Card accuracy maps to spread: `spread = (100 − weapon card accuracy) / 12` degrees of full aperture. "An Accuracy of 100 means the weapon has 0 spread."
- The pool: "*AccuracyPool* starts at *accuracyMinimum*. Whenever the character fires a shot, it increases by *recoil*, up to a maximum of *accuracyMaximum*. If the character stops firing, after a delay of *accuracyIdleRegenDelay*, it begins to decrease at a rate of *accuracyRegenRate* until the character fires again or it reaches *accuracyMinimum*."
- `accuracyIdleRegenDelay` = **0.2 s for all weapons**.
- Scopes multiply spread only when aiming down sights. Scope sway is roughly proportional to the pool. Screen shake and muzzle climb are proportional to recoil.

**Base values by weapon type** (degrees; regen in degrees per second) [VERIFIED same page, values sourced there from the community Gear Calculator]:

| Type | Subtype | Spread | Hip min | Hip max | Hip regen | Sight min | Sight max | Sight regen | Recoil |
|---|---|---|---|---|---|---|---|---|---|
| Assault rifle | Combat rifle | 1.30 | 1.50 | 12.00 | −8.00 | 1.00 | 8.00 | −6.00 | 3.50 |
| Assault rifle | Support machine gun | 2.40 | 2.00 | 14.00 | −8.00 | 1.00 | 8.00 | −6.00 | 2.20 |
| Pistol | Machine pistol | 2.00 | 2.00 | 12.00 | −8.00 | 1.00 | 8.00 | −6.00 | 1.50 |
| Pistol | Repeater | 2.00 | 1.50 | 8.00 | −12.00 | 1.00 | 4.00 | −12.00 | 2.00 |
| Pistol | Revolver | 1.00 | 1.50 | 12.00 | −7.00 | 1.00 | 8.00 | −6.00 | 3.00 |
| Rocket launcher | — | 1.00 | 2.00 | 12.00 | −8.00 | 2.00 | 8.00 | −8.00 | 6.00 |
| Shotgun | Assault | 7.00 | 2.00 | 12.00 | −8.00 | 1.00 | 12.00 | −8.00 | 7.00 |
| Shotgun | Combat | 6.50 | 2.00 | 12.00 | −8.00 | 1.00 | 12.00 | −8.00 | 8.00 |
| SMG | — | 2.00 | 2.00 | 10.00 | −10.00 | 0.00 | 8.00 | −6.00 | 1.50 |
| Sniper | Pump-action | 0.40 | 5.00 | 12.00 | −6.00 | 0.00 | 12.00 | −6.00 | 15.00 |
| Sniper | Semi-automatic | 0.60 | 5.00 | 12.00 | −6.00 | 0.00 | 12.00 | −2.00 | 12.00 |

Stock modifiers replace the pool min/max modifiers: none −20%, stock1 −30%, stock2 −40%, stock3 −70%, stock4 −100%, stock5 −140% [VERIFIED same]. The page notes shotguns have a built-in +400% to sight AccuracyMax and semi-auto snipers +70% regen rate.

**Why this is the right model for PALIMPSEST.** It separates *weapon identity* (spread, min, max, regen, recoil), *part modifiers* (stock, barrel, scope) and *player skill* (sub-attribute multipliers), which maps 1:1 onto canon §6.4 rows 15 (spread × PRC) and 16 (recoil × GRP). The pool is a single integer state per wielded weapon, which is deterministic and cheap.

### 2.2 Damage falloff and ramp-up (Team Fortress 2)

[VERIFIED https://wiki.teamfortress.com/wiki/Damage]

- Standard: `Base × Splash × Resistance/Vulnerability × Distance modifier`.
- **Distance modifier** runs from **1.5 at 0 HU** through **1.0 at 512 HU** to **0.5 at 1024 HU or farther**, "following a cubic curve".
- Random spread: with `tf_damage_disablespread 0`, distance is randomly shifted by **±102.4 HU**, about 15% damage variance. Official servers default to `tf_damage_disablespread 1`.
- Projectiles use attacker–target distance **at the moment of impact**.
- Exceptions: Rocket Launcher ramp-up **125%**; several weapons **120%**; Scout primaries **175%** at 0 HU. **No falloff** (modifier fixed at 1): melee, Flare Gun, grenade launchers, Sniper Rifles, Huntsman. Crusader's Crossbow: base damage *rises* with distance.
- Ramp-up over time: flamethrowers start at **50%** and reach 100% after **0.9 s** on the same target; miniguns **50% → 100% over 1 s** of spin-up.
- Splash: full damage at center, "roughly half" at the edge; a direct hit ignores splash falloff.
- **Crits ×3**; mini-crits ×1.35. **Crits and mini-crits receive no falloff beyond 512 HU**: "Lost damage is offset, so damage stays constant out to range."
- Backstab: 2× the victim's current health, always crit, so 6×.

Unit conversion: 1 Hammer unit = 0.75 inch = 19.05 mm [RECALLED], so 512 HU ≈ 9.75 m and 1024 HU ≈ 19.5 m.

### 2.3 Critical and precision formulas

**Borderlands 2** [VERIFIED https://borderlands.fandom.com/wiki/Critical_hit_(Borderlands_2)]:

```
CritDamage = Damage × Multiplier × (100% + BaseBonus) / (100% + CritPenalty)
```

| Weapon class | Multiplier | BaseBonus | CritPenalty |
|---|---|---|---|
| Shotguns, pistols, SMGs | ×2 | 0% | 0% (SMG "—") |
| Jakobs shotguns / ARs | ×2.3 | 0% | — |
| Jakobs pistols | ×2.5 | 0% | — |
| Assault rifles | ×2 | 0% | **20%** |
| E-Tech assault rifles | ×2 | 0% | 70% |
| Sniper rifles | ×2 | **100%** | 0% |
| Jakobs sniper rifles | ×2 | **160%** | — |
| E-Tech sniper rifles | ×2 | 100% | 100% |

- Skill, class-mod and Badass-Rank crit bonuses all add to BaseBonus. Bonus elemental damage is applied **after** the crit and is never multiplied.
- Example from the page: Zer0 with +20% from Headsh0t and a 150-damage Jakobs sniper: `150 × 2 × (1 + 1.6 + 0.2) = 840`. The same Zer0 with a Vladof assault rifle: `150 × 2 × 1.2 / 1.2 = 300`.
- Melee crits are a flat ×2 and ignore weapon or skill crit bonuses.

**Borderlands 3** [VERIFIED https://borderlands.fandom.com/wiki/Critical_hit_(Borderlands_3)]:

```
CritDMG = NormalDMG × 2 × (100% + Add1 + Add2 + …) × (100% + Mult1) × (100% + Mult2) × …
```

- Additive bucket: skills, class mods, Guardian Rank, anointments.
- Multiplicative: sniper rifles **+20%**, the weapon card crit bonus, **Jakobs +10%**, **Hyperion +5%**.
- Worked example on the page: a Hyperion sniper with 1000 damage, +21% card, +6.35% Guardian Rank and +12% skill gives `1000 × 2 × 1.1835 × 1.20 × 1.21 × 1.05 = 3609`.
- "Vault Hunters don't have a crit spot, so enemies can't do critical hits on players." Melee and grenades cannot crit.

**Warframe crit tiers** (crit chance above 100%) [VERIFIED https://wiki.warframe.com/w/Critical_Hit]:

- `Total Critical Chance = Base × (1 + Σ relative bonuses) + Σ absolute bonuses`.
- Tiers: chance ≤ 0% never crits; 0–100% chance of a tier-1 (yellow) crit; 100–200% every hit at least tier 1 with a chance of tier 2 (orange); and so on without limit (red from tier 3).
- `Critical Tier Multiplier = 1 + Critical Tier × (Total Critical Damage Multiplier − 1)`.
- `Average Damage Multiplier = 1 + Total Critical Chance × (Total Critical Damage Multiplier − 1)`. The page's example: a Paris at 30% chance and ×2.0 averages ×1.3; with the two basic crit mods (75% and ×4.4) it averages ×3.55.
- Headshot crits: `Headshot Multi × (1 + Tier × (2 × Total Crit Damage − 1))`. The headshot multiplier "is 3.0x in almost all cases".
- Design note from the page, quoting the developers, as of **version 35.0.9: enemies cannot inflict critical hits**, because a head hit is "outside of player control" and the damage spikes were unreadable.

**Minecraft melee crit** is ×1.5 and needs the attack cooldown at **84.8% or higher** [VERIFIED https://minecraft.wiki/w/Damage] (§5).

### 2.4 Level scaling of weapon damage and the canon P(d) cross-check

- **Borderlands 3:** "Weapon damage scales at 1.09^n, where n is the Mayhem Level (up to ~237% at Mayhem 10), which is the same as level scaling (that means a weapon at Lv 72 M10 has the damage of a weapon at Lv 82)." [VERIFIED https://borderlands.fandom.com/wiki/Mayhem_Mode]. So BL3 weapon damage grows **×1.09 per character level**.
- **Canon cross-check:** canon §13.8 generates `P(d) = 10000 × 10900^(d−1) / 10000^(d−1)`, also **×1.09 per depth**. PALIMPSEST's weapon power curve is therefore identical to Borderlands 3's per-level curve, a shipped and widely tested looter value `[COMPUTED comparison]`.
- **Borderlands 1** level-difference multiplier on all damage [VERIFIED https://borderlands.fandom.com/wiki/Damage]: attacker − defender level = +6 or more ×2; +5 ×1.6; +4 ×1.4; +3 ×1.3; +2 ×1.2; +1 ×1.1; 0 ×1.0; −1 ×0.8; −2 ×0.6; −3 ×0.4; −4 ×0.2; −5 ×0.1. PALIMPSEST scales by depth, not by player level, so this table should **not** be used between Errata and enemies. It is a useful precedent only for enemy-versus-summon fights where levels differ.

### 2.5 Projectile ballistics (Minecraft arrows) converted to 60 Hz

[VERIFIED https://minecraft.wiki/w/Arrow]

- Damage tag 2; with Power, `2.5 + Power/2`. Hit damage = `ceil(damage tag × speed)`. Critical arrows (full bow draw, or any crossbow) add a random bonus.
- Starting speed in blocks per tick: bow 0–3 by draw; crossbow **3.15**; dispenser 1.1. Inaccuracy: bow and crossbow 1, dispenser 6. Random spread per axis `±0.0172275 × inaccuracy`.
- Java movement per 20 Hz tick: **gravity 0.05** subtracted from Y velocity; **drag 0.99** in air and **0.6** in water; terminal velocity 5 blocks per tick. Order: position, then drag, then gravity.

Conversion to PALIMPSEST (1 block = 1 m) `[COMPUTED]`:

| Quantity | Minecraft (20 Hz) | Real units | PALIMPSEST per 60 Hz tick |
|---|---|---|---|
| Gravity on arrows | 0.05 block/tick² | **20 m/s²** | 5.556 mm/tick² |
| Air drag | ×0.99 per tick | — | ×0.99666 per tick (`0.99^(1/3)`) |
| Water drag | ×0.6 per tick | — | ×0.84343 per tick |
| Full-draw bow speed | 3 blocks/tick | **60 m/s** | 1000 mm/tick |
| Crossbow speed | 3.15 blocks/tick | 63 m/s | 1050 mm/tick |

Arrow gravity (20 m/s²) is about 2× real gravity, which keeps arcs visible at game ranges.

### 2.6 Proc coefficients (Risk of Rain 2)

[VERIFIED https://riskofrain2.wiki.gg/wiki/Proc_Coefficient]

- Definition: "the factor by which an item's effect, trigger chance, duration, or any combination of the three is scaled".
- Example: Tri-Tip Dagger has a 10% (+10% per stack) chance to bleed. Bleed deals 240% damage over time, and its duration is `3 × proc coefficient` seconds. An Auto-Nailgun (0.6) gets a 6% chance per Dagger and a 1.8 s bleed. A Supercharged Railgun (3.0) gets a 9 s bleed, and four Daggers guarantee it.
- **Proc coefficients do not affect critical strike chance.**
- Skill values: Commando Double Tap 1.0; Phase Blast 0.5 × 8; Huntress Flurry 0.7 × 3; Arrow Rain 0.2 × 19; Captain Vulcan Shotgun 0.75 × 8; MUL-T Nailgun 0.6; Railgunner Supercharged 3.0.
- Item values: Ukulele 0.2; Fireworks 0.2 × 8; Sentient Meat Hook 0.33; Razorwire 0.5; Frost Relic 0.2; Sticky Bomb **0.0**.
- **Chain breakers:** Bleed, Burn and Collapse have proc coefficient **0.0**, so they "will never activate other effects".

### 2.7 Aim assist: what could not be verified

No primary or wiki source for Destiny 2, Halo or Apex aim-assist parameters was reachable this session (Destiny wiki has no weapon-stat pages; Apex wiki has no aim-assist page). The three-part structure, (1) **slowdown/friction** of look sensitivity near a target, (2) **magnetism** that bends the projectile path toward the target inside a small cone, and (3) optional **snap** on ADS, is [RECALLED] from industry practice. Every aim-assist number in §11 is therefore `[PROPOSED]`.

---

## 3. Roguelite run scaling, spawn directors and item stacking (Risk of Rain 2)

Risk of Rain 2 is the reference 3D co-op roguelite shooter: 1–4 players, real-time, item stacking and a time-driven difficulty curve.

### 3.1 Difficulty coefficient

[VERIFIED https://riskofrain2.wiki.gg/wiki/Difficulty]

```
playerFactor = 1 + 0.3 × (playerCount − 1)
timeFactor   = 0.0506 × difficultyValue × playerCount^0.2
stageFactor  = 1.15^stagesCompleted
coeff        = (playerFactor + timeInMinutes × timeFactor) × stageFactor
enemyLevel   = 1 + (coeff − playerFactor) / 0.33
```

- difficultyValue: Drizzle 1, Rainstorm 2, Monsoon 3. playerCount includes disconnected players. Time increments every second.
- Each enemy level adds **+30% health and +20% damage** over Logbook base values.
- Drizzle: time ×0.5, +70 armor, regen ×1.5. Monsoon: time ×1.5, regen ×0.6.
- Special boss scaling: `enemyHP × round((1 + coeff/2.5) × livingPlayers × 10) / 10`; `enemyDamage × round((1 + coeff/30) × 10) / 10`.

Worked values (Rainstorm) `[COMPUTED]`:

| Players | Minutes | Stages done | coeff | Enemy level | HP multiplier (1 + 0.3 × (level − 1)) |
|---|---|---|---|---|---|
| 1 | 10 | 2 | 2.661 | 6.0 | ×2.5 |
| 1 | 30 | 5 | 8.118 | 22.6 | ×7.3 |
| 4 | 10 | 2 | 4.279 | 8.2 | ×3.1 |
| 4 | 30 | 5 | 11.879 | 31.2 | ×10.0 |
| 4 | 60 | 10 | 40.10 | 116.8 | ×35.5 |

**Lesson:** time spent, not only depth, drives threat. This is RoR2's anti-stall device. PALIMPSEST's Erasure clock (canon `ERASURE_ONSET` 420 s, stacks every 30 s) is the same idea applied per Folio instead of per run.

### 3.2 Directors (spawn budgets)

[VERIFIED https://riskofrain2.wiki.gg/wiki/Directors]

- **Scene Director, interactables:** a stage base (Distant Roost 220, Siphoned Forest 300, Verdant Falls 280, Abyssal Depths 400), **+50% per additional player** (Abyssal Depths gives 400 / 600 / 800 / 1000 for 1–4 players).
- **Scene Director, monsters:** stage base × difficulty coefficient, **not** scaled by player count (except through the coefficient). Example: 100 × 2.3 = 230. The first stage uses the base.
- **Combat Directors** (continuous): `creditsPerSecond = creditMultiplier × (1 + 0.4 × coeff) × (playerCount + 1) / 2`. creditMultiplier: 0.75 for the Fast and Slow directors, 2.0 for the Teleporter director.
- Spawn costs: Beetle 8, Lemurian 11, Hermit Crab 25, Clay Templar 125, Greater Wisp 200, Scavenger 2000.
- **Elite cost multipliers:** Tier 1 ×6 (×3.5 with Artifact of Honor); Tier 2 ×36 (stage 6 onward); Lunar ×6. Example: a Tier-1 Beetle costs 8 × 6 = 48.

### 3.3 Item stacking formulas

[VERIFIED https://riskofrain2.wiki.gg/wiki/Item_Stacking]

| Stacking type | Formula | Example |
|---|---|---|
| Linear | `f(x) = 1 + a·x` | Soldier's Syringe +15% attack speed per stack: 10 stacks = +150% |
| Hyperbolic | `f(x) = 1 − 1/(1 + a·x)` | Tougher Times, a = 0.15: 13.04% at 1 stack, 60% at 10 stacks |
| Reciprocal | `f(x) = a/x`, or `a/(x + 1)` so the first stack counts | cooldown-style items |
| Exponential | `f(x) = a^x` | Shaped Glass doubles damage per stack: 2 stacks ×4 |
| Special | Bandolier `1 − 1/(1 + x)^0.33`; Dream Crucible `1 − 0.9^x` | — |

Hyperbolic values `[COMPUTED]`: a = 0.15 gives 13.04% / 42.86% / 60.00% / 75.00% at 1 / 5 / 10 / 20 stacks. It approaches 100% but never reaches it, which makes it the correct curve for any stacking avoidance, block or dodge chance.

### 3.4 Luck

[VERIFIED https://riskofrain2.wiki.gg/wiki/Luck]

- Each 57 Leaf Clover is +1 Luck. "All random effects are rolled +1 (+1 per stack) times for a favorable outcome."
- `y = 1 − (1 − p)^(x + 1)`, where p is the proc chance and x the Clover count. With t rolls: `y = 1 − ((1 − p)^(x + 1))^t`. Inverse: `x = log(1 − y)/log(1 − p) − 1`.
- Negative Luck (Purity, −1 each) cancels Clovers one for one. At −2 Luck "a proc must succeed three times in a row".
- **Not affected by Luck:** chest spawns and contents (world seed), Shrine of Chance, Artifact Key drop, Sacrifice drops, Tougher Times.
- Values `[COMPUTED]`: p = 10% gives 19.0% / 27.1% / 34.4% at Luck 1 / 2 / 3; p = 25% gives 43.8% / 57.8% / 68.4%.

### 3.5 Chests and multishops

[VERIFIED https://riskofrain2.wiki.gg/wiki/Chest; https://riskofrain2.wiki.gg/wiki/Chests]

- Small Chest base $25: Common 79.2%, Uncommon 19.8%, Legendary 0.99%. Large Chest base $50: Uncommon 80%, Legendary 20%.
- **Multishop Terminal:** 3 terminals, "One item will always be visible, while the others have a 20% to be hidden." Buying one **locks the other two** (unless the buyer carries Executive Card). Costs match Small or Large Chests by tier.
- The commonly quoted cost scaling `cost = base × coeff^1.25` was **not** found on the fetched pages [RECALLED].

### 3.6 Shrine of Chance (the in-run gamble precedent)

[VERIFIED https://riskofrain2.wiki.gg/wiki/Shrine_of_Chance]

- Outcomes per use: Nothing 45%, Common 36%, Uncommon 9%, Equipment 9%, Legendary 1%. Success is therefore 55%.
- "The base cost is $17, increasing by 40% with each activation." At most **two** successful payouts per shrine.
- The success chance is **not** affected by Luck.

Under the brief, PALIMPSEST's Fortune Engines must always pay out, so the 45% "Nothing" row cannot be copied. The **escalating price (+40% per use)** and the **payout cap per machine** can be.

---
## 4. Co-op scaling across shipped games versus canon §17.2

### 4.1 Deep Rock Galactic (4-player co-op, Hazard 1–5)

[VERIFIED https://deeprockgalactic.wiki.gg/wiki/Hazard_Level (redirects to "Difficulty Scaling")]

Values are listed as solo / 2 / 3 / 4 players where they vary.

| Hazard | Bonus | Enemy damage | Attack rate | Projectile speed | Move speed | Friendly fire | Environmental dmg | Revive health | Max HP regen | Point Extraction scaler |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 Low Risk | +25% | 0.50 / 0.50 / 0.50 / 0.50 | 0.80 | 1.00 | 0.80 | 0.10 | 0.75 | 0.60 | 0.15 | 0.70 |
| 2 Challenging | +50% | 0.70 / 0.80 / 0.90 / 1.00 | 0.90 | 1.00 | 0.90 | 0.20 | 1.00 | 0.50 | 0.15 | 0.80 |
| 3 Dangerous | +75% | 1.20 / 1.30 / 1.40 / 1.50 | 1.00 | 1.00 | 1.00 | 0.30 | 1.00 | 0.40 | 0.15 | 0.90 |
| 4 Extreme | +100% | 2.00 / 2.15 / 2.30 / 2.50 | 1.25 | 1.30 | 1.00 | 0.40 | 1.33 | 0.20 | 0.15 | 1.00 |
| 5 Lethal | +133% | 2.80 / 3.00 / 3.20 / 3.40 | 1.50 | 1.70 | 1.15 | 0.70 | 2.00 | 0.10 | 0.10 | 1.10 |

**Resistance scaling.** `Damage Dealt To Enemy = Base Damage / Enemy Damage Resistance`, equivalently `Effective Enemy Health = Base Health × Enemy Damage Resistance`.

| Hazard | Enemy count | Normal | Large | Extra large | Elite |
|---|---|---|---|---|---|
| 1 | 0.15 / 0.25 / 0.45 / 0.65 | 0.70 | 0.45 / 0.55 / 0.70 / 0.85 | 0.20 / 0.40 / 0.60 / 0.80 | 0.60 / 0.75 / 0.85 / 1.10 |
| 2 | 0.25 / 0.35 / 0.65 / 0.80 | 1.00 | 0.65 / 0.75 / 0.90 / 1.00 | 0.30 / 0.40 / 0.70 / 1.00 | 0.90 / 1.10 / 1.10 / 1.30 |
| 3 | 0.50 / 0.50 / 0.80 / 1.10 | 1.10 | 0.80 / 0.90 / 1.00 / 1.10 | 0.50 / 0.55 / 0.90 / 1.20 | 0.90 / 1.10 / 1.30 / 1.30 |
| 4 | 0.75 / 0.75 / 1.15 / 1.35 | 1.20 | 1.00 / 1.00 / 1.20 / 1.30 | 0.70 / 0.75 / 1.00 / 1.60 | 1.10 / 1.10 / 1.30 / 1.50 |
| 5 | 0.85 / 0.85 / 1.25 / 1.50 | 1.20 | 1.20 / 1.20 / 1.40 / 1.50 | 0.75 / 0.80 / 1.20 / 1.70 | 1.30 / 1.30 / 1.50 / 1.65 |

- **Normal enemies do not scale with player count at all**; the extra threat comes from enemy count. Large, boss-class and elite enemies scale per player.
- "There is only a small increase of enemy spawns between Solo and 2 Player to compensate for the loss of Bosco" (the solo drone companion).
- Friendly fire **scales with difficulty**, from 10% to 70%.
- Revive health falls from 60% to 10% of max as Hazard rises.

### 4.2 Risk of Rain 2 player scaling (summary of §3)

- Combat credits `× (playerCount + 1)/2`: ×1.0 / 1.5 / 2.0 / 2.5 for 1–4 players.
- Time scaling `× playerCount^0.2`: ×1.00 / 1.15 / 1.25 / 1.32.
- Interactables +50% per extra player. Special bosses: HP × living players.
- Gold from kills and barrels "is shared between players" [VERIFIED https://riskofrain2.wiki.gg/wiki/Gold].

### 4.3 Borderlands 3 loot and level modes

[VERIFIED https://borderlands.fandom.com/wiki/Borderlands_3]

- **Cooperation** mode: "Each player has their own loot instance (i.e. two players can loot the same loot chest and obtain different gear). Character levels are intuitively scaled (weapons inflict a damage percentage which is then applied to the enemy)."
- **Coopetition**: shared loot, the default of BL1, BL2 and the Pre-Sequel; higher-level characters hold an advantage.
- Borderlands 4: 4-player online co-op, 2-player split-screen on consoles, cross-platform [VERIFIED https://borderlands.fandom.com/wiki/Borderlands_4].

### 4.4 Canon §17.2 against the precedents

Threat budget = per-enemy HP multiplier × enemy-count multiplier, normalized to solo `[COMPUTED from the verified tables and canon §17.2]`.

| Players | Canon §17.2 budget (HP minion–champion × count) | Canon per player | DRG Hazard 4 normal enemies (count × resistance) | DRG per player | RoR2 combat credits | RoR2 per player | Canon boss HP | Canon boss HP per player |
|---|---|---|---|---|---|---|---|---|
| 1 | 1.000 | 1.000 | 1.00 | 1.00 | 1.0 | 1.00 | 1.0 | 1.00 |
| 2 | 2.000 | 1.000 | 1.00 | 0.50 | 1.5 | 0.75 | 1.9 | 0.95 |
| 3 | 3.150 | 1.050 | 1.53 | 0.51 | 2.0 | 0.67 | 2.8 | 0.93 |
| 4 | 4.463 | 1.116 | 1.80 | 0.45 | 2.5 | 0.63 | 3.7 | 0.93 |

Readings:

1. **Canon boss HP scaling matches RoR2.** RoR2 scales special bosses by living players (×n). Canon boss HP is 1.0 / 1.9 / 2.8 / 3.7 (0.93–0.95 per player).
2. **Canon enemy damage scaling matches DRG.** Canon 1.0 / 1.1 / 1.2 / 1.3 equals DRG Hazard 3's relative curve (1.20 / 1.30 / 1.40 / 1.50 normalized: 1.00 / 1.08 / 1.17 / 1.25).
3. **Canon's trash-mob budget is the outlier.** At 4 players it gives every player 1.12× the solo threat, while DRG gives 0.45× and RoR2 about 0.63×. Both shipped co-op games deliberately make each player face *less* than a solo player, because co-op adds revives, crossfire and focus fire. See §11 R-CO1.

---

## 5. Real-time melee and armor (Minecraft Java)

### 5.1 Attack cooldown

[VERIFIED https://minecraft.wiki/w/Damage]

- "The damage multiplier is then `0.2 + ((t + 0.5) / T) ^ 2 * 0.8`, restricted to the range 0.2 – 1."
- `T = 20 / attack_speed` ticks (20 Hz), where `t` is ticks since the last attack or item switch.
- Sharpness, Smite and Bane of Arthropods scale with the same timing but **without** the squaring step.
- Crit: weapon base damage, plus effects, ×1.5 if critical, then enchantment damage. A crit needs "the attack cooldown of the weapon is at 84.8% or higher".

Worked values `[COMPUTED]`: a sword (attack speed 1.6, T = 12.5 ticks) swung 2 ticks after the last swing deals ×0.23; at 5 ticks ×0.36; at 12 ticks ×1.0. Spam-clicking deals about a fifth of full damage.

### 5.2 Armor and toughness

[VERIFIED https://minecraft.wiki/w/Armor]

```
reduction% = min(20, max(armorPoints / 5, armorPoints − 4 × damage / (min(toughness, 20) + 8))) / 25 × 100
```

- Capped at 80%. Armor attribute capped at 30, toughness at 20. "The total damage reduction of armor cannot drop off below 20% of its maximum."
- Enchantment Protection Factor: Protection I–IV give EPF 1–4; all applicable EPFs are summed to at most 20; reduction = `EPF / 25`.

Against canon §10.5 step 5 `[COMPUTED]`:

| Hit size | Minecraft diamond (20 armor, 8 toughness) | Minecraft netherite (20, 12) | Canon AR 100 (DR = AR/(AR + 4D), cap 85%) |
|---|---|---|---|
| Small (MC 4 HP; canon 5 HP) | 76.0% | 76.8% | 83.3% |
| Medium (MC 10; canon 20 HP) | 70.0% | 72.0% | 55.6% |
| Large (MC 20; canon 50 HP) | 60.0% | 64.0% | 33.3% |

Both shrink armor's effect on large hits. Minecraft adds **toughness**, a second stat that slows that decay. Canon has the equivalent through `pen_bp` on the attacker's side but nothing on the defender's side.

---

## 6. Voxel world physics: breaking, explosions, fluids, structural integrity

### 6.1 Block breaking

[VERIFIED https://minecraft.wiki/w/Breaking]

- Base: the block's hardness, ×1.5 if the tool can harvest it, ×5 if it cannot.
- Tool speed (correct tool only): Wood 2, Stone 4, Copper 5, Iron 6, Diamond 8, Netherite 9, Golden 12. Otherwise speed 1.
- Efficiency adds `level² + 1` to tool speed. Haste adds +20% speed per level (Java).
- Head underwater without Aqua Affinity: ×5 time. Not on the ground: another ×5.
- `damage per tick = speed / hardness / 30` with a harvesting tool, `/ 100` otherwise. If damage ≥ 1, the block breaks instantly. Otherwise `ticks = ceil(1 / damage)` at 20 Hz.

Check against known values `[COMPUTED; known values RECALLED]`: stone (hardness 1.5) with a stone pickaxe is `ceil(30 × 1.5 / 4) = 12` ticks = 0.60 s; with diamond 6 ticks = 0.30 s; by hand `ceil(100 × 1.5)` = 150 ticks = 7.5 s; obsidian (50) with diamond 188 ticks = 9.4 s. All four match the familiar in-game times.

At 60 Hz the same real time is `ticks60 = ceil(90 × hardness / speed)` with a harvesting tool and `ceil(300 × hardness / speed)` without.

### 6.2 Explosions

[VERIFIED https://minecraft.wiki/w/Explosion]

1. Rays go from the center toward the outer points of a 16 × 16 × 16 grid: `16³ − 14³ = 1352` rays `[COMPUTED]`.
2. Each ray starts with intensity `power × U(0.7, 1.3)`.
3. Repeat: at the current position, if the block is not air, subtract `(blast_resistance + 0.3) × 0.3`. If intensity is still above 0 and the block is destructible, add it to the destruction list. Step 0.3 blocks. Subtract **0.22500001**. Stop when intensity ≤ 0.
4. Shuffle the list. Fire-causing explosions try to ignite about one third of the destroyed blocks, sticking only above opaque blocks.

Entity damage: `impact = (1 − distance / (2 × power)) × exposure`; `damage = 7 × (2 × power) × (impact² + impact)/2 + 1`. Easy `min(damage/2 + 1, damage)`, Hard ×1.5. Powers: TNT 4, creeper 3, charged creeper 6, bed in the wrong dimension 5, end crystal 6, wither 7, fireball 1.

Derived `[COMPUTED]`: maximum reach through air is `1.3 × power × 0.3 / 0.225`, which is 6.9 m for TNT. Maximum entity damage at the center is `14 × power + 1`, which is 57 HP for TNT.

### 6.3 Fluids

[VERIFIED https://minecraft.wiki/w/Fluid]

- Source level 0. Water and Nether lava add 1 level per flowing block up to 7. Overworld lava adds 2 up to 6. Falling fluid (Java) is level 8.
- Spread speed: water 1 block per 5 ticks (4 blocks/s); Nether lava 1 per 10 ticks; Overworld lava 1 per 30 ticks.
- Reach on flat ground: water about 7 blocks (a diamond 15 blocks point to point); Overworld lava about 3.
- Drop search: flow prefers the shortest route to a drop within **5** blocks (water, Nether lava) or **3** (Overworld lava).
- Infinite source (Java): an enterable cell with **at least two horizontally adjacent water sources and a non-flowable block below** becomes a source. Lava does not do this by default.

### 6.4 Structural integrity

**Valheim** (support values from `WearNTear.cs`, published in the wiki's data module) [VERIFIED https://valheim.fandom.com/wiki/Building_stability; https://valheim.fandom.com/wiki/Module:Damage/data]:

```
Distance   = sqrt((h_child/2 + h_parent/2)² + (w_child/2 + w_parent/2)²) + 0.1      // 0.1 m "fudge"
AngledLoss = interpolate(VerticalLoss, HorizontalLoss) by the angle to the parent's support point
Support    = parent_support × (1 − AngledLoss × Distance)
MutualSupport (two parents > 100° apart, both ≥ 0.05 m below) =
           (S_a × (1 − VerticalLoss × d_a) + S_b × (1 − VerticalLoss × d_b)) / 2
final support = max over every single and mutual support; break if support ≤ MinSupport
grounded (touching terrain, rocks or trees) → support = MaxSupport
```

| Material | MinSupport | MaxSupport | VerticalLoss | HorizontalLoss |
|---|---|---|---|---|
| Wood | 10 | 100 | 1/8 | 1/5 |
| Hard wood (core wood) | 10 | 140 | 1/10 | 1/6 |
| Stone | 100 | 1000 | 1/8 | 1/1 |
| Iron | 20 | 1500 | 1/13 | 1/13 |
| Black marble | 100 | 1500 | 1/8 | 1/2 |
| Ashstone (Grausten) | 100 | 2000 | 1/10 | 1/3 |
| Ancient | 100 | 5000 | 1/15 | 1/4 |

- Support recalculates iteratively: "every 0.5 seconds … A maximum of 50 components are recalculated every 0.5 seconds."
- The wiki states that wood and stone poles reach **16 m**, core-wood poles **24 m** and wood-iron poles **50 m** [VERIFIED https://valheim.fandom.com/wiki/Building].
- **Correction to the wiki text** `[COMPUTED]`: the page's angle formula, read literally, applies HorizontalLoss to a straight vertical stack, and its worked example uses 0.2 for wood poles. With VerticalLoss applied vertically, 2 m poles reproduce the stated heights exactly: wood `100 × (1 − 0.125 × 2.1)^7 > 10` and the 8th fails, giving 16 m; core wood 24 m; iron 50 m. With HorizontalLoss the wood tower would stop at 10 m. **Implement VerticalLoss for vertical neighbors.**

Behavior on a 1 m voxel grid (Distance = 1.1) `[COMPUTED]`:

| Material | Blocks stackable on a grounded block | Longest horizontal cantilever |
|---|---|---|
| Wood | 15 | 9 |
| Hard wood | 22 | 13 |
| Stone | 15 | **0** (horizontal loss 1/1 kills any overhang) |
| Iron | 48 | 48 |
| Black marble | 18 | 3 |
| Ashstone | 25 | 6 |

**7 Days to Die** [VERIFIED https://7daystodie.wiki.gg/wiki/Structural_Integrity]: blocks on bedrock, and blocks on a vertically supported load-bearing block, are vertically supported, and "Vertically supported blocks are always stable, regardless of material, shape, or the mass above them." Each block has **Mass** and a **Horizontal Support** limit per side face (Wood 40, Concrete 120 in the page's examples). A series attached to one face is checked against that face's limit, and a series attached to several faces splits its mass across them. A non-vertically-supported block's limit applies to its own mass plus everything attached. Every placement or removal triggers recalculation, and newly unstable blocks collapse immediately.

---

## 7. Barony's missing systems: Tinkering crafting and Alembic alchemy

### 7.1 Tinkering

[VERIFIED https://barony.wiki.gg/wiki/Tinkering]

- **Tinkering kit:** three tabs, Crafting, Salvage and Repair. "Every use of the tinkering kit is associated with a 5% chance to degrade." An Excellent kit lasts about 100 uses. A broken kit can repair itself given enough scrap.
- **Salvage** destroys items for **metal scrap** (most armor, weapons, tools and mechanisms) or **magic scrap** (magical, blessed or cursed items).
- **Recipes** (metal : magic scrap, gated by Tinkering skill plus PER):

| Output | Metal | Magic | Gate | Tier |
|---|---|---|---|---|
| Beartrap | 12 | 0 | 0 | Novice |
| Empty bottle | 2 | 2 | 0 | Novice |
| Glasses | 8 | 4 | 0 | Novice |
| Lockpick | 2 | 0 | 0 | Novice |
| Noisemaker | 8 | 1 | 0 | Novice |
| Dummybot | 8 | 4 | 20 | Basic |
| Gyrobot | 16 | 12 | 20 | Basic |
| Lantern | 8 | 4 | 20 | Basic |
| Freeze trap | 8 | 12 | 20 | Basic |
| Sleep trap | 4 | 8 | 40 | Basic |
| Flame trap | 8 | 12 | 40 | Skilled |
| Teleportation trap | 4 | 8 | 40 | Skilled |
| Sentrybot | 16 | 8 | 40 | Skilled |
| Spellbot | 8 | 16 | 60 | Expert |
| Tech goggles | 10 | 40 | 60 | Expert |
| Backpack | 20 | 4 | 80 | Master |

- **Robot quality by skill** (Sentrybot): Simple at skill 40 (level 3, 50 HP, AC 0, ATK 16, "Hold Aim"); Complex 60 (level 5, 75 HP, AC 3, ATK 20, "Look at"); Intricate 80 (level 10, 125 HP, AC 6, ATK 24, "Attack"); Artisan 100 (level 15, 150 HP, AC 9, ATK 28). Dummybots "can train weapon skill up to 20".
- **Lockpicking:** door and chest success `skill/2` %. Disarming a trap needs skill 40, succeeds at `skill` %, degrades the pick 10% (25% on failure) and fires an arrow on failure. Sabotaging an Automaton works only from behind (180° cone), at 0/50/66/100% for skill 0/20/40/60.
- **Lock health (a deterministic pity on lockpicking):** each attempt deals lock "damage" by skill (3 at 0–19, 4–5 at 20–59, 6–11 at 60–99, 8–17 at 100). Doors open at 20 cumulative damage and chests at 40, whatever the success rolls. At skill 0 a door needs 7 attempts (2 picks) and a chest 14 attempts (3 picks).
- **Skill-gain odds:** unlocking a chest 100% (cap 60, once per lock); failing 10%; door 100% (cap 40); salvaging an item worth 4+ scrap 50% (cap 60); crafting items costing 5+ scrap 10% (cap 100); enemy killed while investigating a noisemaker 20%, or 47% if it was unaware of the player.

### 7.2 Alembic alchemy

[VERIFIED https://barony.wiki.gg/wiki/Alchemy]

- Ingredients are **potions**. Each potion type has a hidden **use**: Base, Secondary or No unique use. Base + Secondary gives a fixed product. Anything else unlisted gives Sickness.
- Learning uses: drink it (risky); brew it (a non-Sickness, non-explosion result reveals one ingredient's use, the left one if both are unknown); or level Alchemy (fixed reveals: Juice and Booze at 20, Acid at 40, Invisibility and Polymorph at 60; a random use every 5th level otherwise).
- **Water duplication:** any potion + water (except Acid and Polymorph) duplicates it, starting at **50% and +10% per 20 Alchemy**. On failure the potion becomes water, and an empty bottle is recovered with chance from 50% at skill 0 to 100% at skill 100.
- **Decanting** stacks two qualities of one potion type at 100%.
- **Polymorph** turns any potion, water included, into a random potion other than firestorm, icestorm, thunderstorm, polymorph or water.
- **Acid** + correct secondary gives unstable Firestorm, Icestorm or Thunderstorm. Acid + water or + another Base causes an explosion that always degrades the Alembic.
- Alembic degrade chance: **1/40** per brew, **1/10** with acid, **100%** on explosion, **0** at Legendary.
- Legendary (100): Alembics never degrade, and brewed potions get **+2 blessing**.
- Skill-gain chances: brew 50%; identify a use 100%; learn a new recipe 100%; hit a target with a thrown potion 25%; drink a known potion 16.66%; duplicate 5%.

---

## 8. Procedural level generation numbers (Shattered Pixel Dungeon)

All from source [VERIFIED-SRC https://github.com/00-Evan/shattered-pixel-dungeon/blob/master/core/src/main/java/com/shatteredpixel/shatteredpixeldungeon/levels/RegularLevel.java; Level.java; SewerLevel.java; PrisonLevel.java; CavesLevel.java; CityLevel.java; HallsLevel.java]. `Random.chances(w)` picks index i with weight w[i].

| Chapter | Standard rooms | Max (LARGE) | Special rooms | Max |
|---|---|---|---|---|
| Sewers | `4 + chances{1,3,1}` (4–6, mode 5) | 6 | `1 + chances{1,4}` (1–2) | 2 |
| Prison | `5 + chances{1,1}` (5–6) | 6 | `1 + chances{1,3,1}` (1–3) | 3 |
| Caves | `6 + chances{2,1}` (6–7) | 7 | `2 + chances{4,1}` (2–3) | 3 |
| City | `6 + chances{1,3,1}` (6–8) | 8 | `2 + chances{2,1}` (2–3) | 3 |
| Halls | `8 + chances{2,1}` (8–9) | 9 | `2 + chances{1,1}` (2–3) | 3 |

- LARGE feeling: standard rooms `ceil(×1.5)`, specials +1, mobs `ceil(×1.33)`, items +2. A shop room is added when the floor rolls one. Secret rooms come from `SecretRoom.secretsForFloor(depth)` (+1 on SECRETS floors).
- **Builder:** 50% `LoopBuilder.setLoopShape(2, U(0, 0.65), U(0, 0.50))`; 50% `FigureEightBuilder.setLoopShape(2, U(0.3, 0.8), 0)`.
- **Mobs:** `mobLimit = 3 + depth % 5 + Random.Int(3)`. Depth 1 starts with 8 mobs and has a respawn limit of 0 (10 once the Amulet is obtained). From depth 2, each room has a 1-in-4 chance to receive a second mob.
- **Items:** `3 + chances{6,3,1}`, so 3/4/5 items at 60/30/10%.
- **Traps:** `Random.NormalIntRange(2, 3 + depth/5)`.
- **Feelings** (depth > 1): `Random.Int(14)`: 7 named outcomes at 1/14 each (CHASM, WATER, GRASS, DARK with view distance ×5/8, LARGE with an extra food item, TRAPS, SECRETS), otherwise NONE, so 50% of floors get a feeling, about 7.14% each.

---
## 9. 2026 status updates and verifications of earlier open questions

| Item | Finding | Effect on earlier dossiers | Tag |
|---|---|---|---|
| Barony 5.0 "Part Two" | GitHub's newest release is **V5.0.2** (marked Latest, 18 Feb). No "Part Two" or 5.1 release is listed. | R1 §1.1 "announced but undated" still holds on 2026-10-11 | [VERIFIED https://github.com/TurningWheel/Barony/releases] |
| Shattered Pixel Dungeon | v4.0.2 on **2026-10-07**, v4.0.1 2026-10-03, v4.0.0 2026-09-09, v3.3.8 2026-03-23, v3.2.5 2025-09-30 | Confirms R1 and R3 (R1 lists the 4.0.2 commit as 2026-10-06; the release is dated 10-07) | [VERIFIED https://github.com/00-Evan/shattered-pixel-dungeon/releases] |
| Experienced Pixel Dungeon | ExpPD-2.19 is "Latest" and described as "the last version of Experienced PD". The repository is **archived and read-only**. | Confirms R1 §3.1 | [VERIFIED https://github.com/TrashboxBobylev/Experienced-Pixel-Dungeon-Redone/releases] |
| Caves of Qud | 2026 builds: 211.33 (2026-01-29), 211.35 (02-16), 211.36 (03-01), 212.17 experimental (03-27). Content is quality of life and modding (UTF-8 mod XML, new XML file types, custom hotbar abilities, faster pathfinding). No new story arc is listed. | Updates R1 §5.1 | [VERIFIED https://wiki.cavesofqud.com/wiki/Version_history] |
| PEGI and paid random items | PEGI news dated **2026-10-06**: "digital-only games that are currently rated PEGI 3, 7 or 12 and feature Paid random items" "will have their age rating updated to PEGI 16 (with the descriptor Paid random items)". Updates begin **October 2026**. New interactive-risk categories were introduced in **June 2026**, and new submissions are assessed on "purchases of in-game content", "communication features" and "elements that incentivise players to continue playing". For new submissions the page says PEGI will assess "to what extent" paid random items require a higher rating; it does not state a fixed minimum. | **Confirms** R3's unconfirmed October re-rating. **Nuances** R3's "minimum PEGI 16 for new submissions", which came from press coverage. MON-0 (no paid random items) remains correct. | [VERIFIED https://pegi.info/news/pegi-updates-age-ratings-existing-games-paid-random-items] |
| EU Digital Fairness Act | Wikipedia (last edited 2026-02-01): consultation 2025-07-17 to 2025-10-24; proposal planned for "the third quarter of 2026"; not yet published | Consistent with R3 (expected November 2026, not published on 2026-10-11) | [VERIFIED https://en.wikipedia.org/wiki/Digital_Fairness_Act] |
| Fortnite battle pass, 2026 | 800 V-Bucks (cut from 1,000 at **Chapter 7 Season 2**, when total V-Bucks obtainable fell from 1,500 to 800 and bonus-reward V-Bucks were removed). 100 rewards: 85 cosmetics plus 800 V-Bucks. Free pass includes 300 V-Bucks. Levels sell for **150 V-Bucks each** up to level 200; a Level Up Quest Pack costs 1,800 V-Bucks for 25+ levels. Mini-seasons have **50** levels. Chapter 7 Season 3 re-locked pages behind levels in order. XP per level is not stated on the wiki pages. | Adds detail to R3 B5. PALIMPSEST's no-tier-skip rule differs deliberately from Fortnite's paid levels. | [VERIFIED https://fortnite.fandom.com/wiki/Battle_Pass; https://fortnite.fandom.com/wiki/Season_XP] |
| Baldur's Gate (1998) | NA release 1998-12-21; first Infinity Engine game; modified AD&D 2nd edition; "pausable real-time" with optional auto-pause; party of up to 6 (player + 5); 6-player online multiplayer; fog of war; reputation affects prices, companions and quests; ~2.8 million sold by 2015; Enhanced Edition 2012 | Replaces R2 §3.1 [RECALLED] items | [VERIFIED https://en.wikipedia.org/wiki/Baldur%27s_Gate_(video_game)] |
| Baldur's Gate II (2000) | Windows NA 2000-09-21; AD&D 2e; party of up to 6; up to 6 human players; kits per class; class-based strongholds; Throne of Bhaal (June 2001) added a higher XP cap, high-level abilities and the Wild Mage; Enhanced Edition 2013 | Replaces R2 §3.1 [RECALLED] items | [VERIFIED https://en.wikipedia.org/wiki/Baldur%27s_Gate_II:_Shadows_of_Amn] |
| Borderlands co-op loot | BL3 "Cooperation" = instanced loot plus level-normalized damage; "Coopetition" = shared loot | Confirms R3 D3's instanced-loot assumption ([RECALLED] there) and canon §17.1 "Loot: instanced per player" | [VERIFIED https://borderlands.fandom.com/wiki/Borderlands_3] |

---

## 10. What remains unverified

1. **Destiny 2 weapon numbers:** damage falloff by Range stat, precision multipliers per archetype and aim-assist cone values. The reachable wiki has no such pages, and Fandom HTML returned 402.
2. **Any shipped aim-assist parameters** (Destiny, Halo, Apex, Call of Duty). Every aim-assist value in §11 is `[PROPOSED]`.
3. **Risk of Rain 2 chest cost scaling** (`base × coeff^1.25`) was not found on the fetched pages. Treat it as [RECALLED].
4. **Hammer unit to metre conversion** (1 HU = 19.05 mm) is [RECALLED].
5. **Fortnite XP per battle-pass level in 2026** is not stated on the wiki pages.
6. **7 Days to Die per-block mass and support values** beyond the page's two examples (Wood 40, Concrete 120 horizontal support).
7. **Minecraft bow draw curve** (draw time to arrow speed) and the exact crit-arrow bonus. The Arrow page's notation was ambiguous.
8. **Barony "Part Two" release date.** The GitHub release list shows none as of today. A Steam-only release without a GitHub tag cannot be ruled out.
9. Carried over from earlier dossiers and still open: Oblivion Remastered enemy-scaling curves; the Destiny 2 armor archetype count (6 or 12); PoE armour constants; Kenshi XP curve.

---

## 11. Design implications for PALIMPSEST

Opinionated, in canon units (Bp, 60 Hz ticks, mm, mdeg = millidegrees). "Steal" means port the shipped rule. "Amend" means a proposed canon change. Owning spec documents are named in brackets.

### Shooting (doc 05)

- **R-SH1. Steal the Borderlands 1 accuracy pool exactly.**
  - Each weapon archetype carries two parameter sets, hip and ADS: `spread_mdeg, pool_min_mdeg, pool_max_mdeg, pool_regen_mdeg_per_s, recoil_mdeg`, plus `regen_delay_ticks = 12` (BL1's 0.2 s).
  - Cone full angle = `spread + pool` from the hip, `spread` alone when aiming.
  - On each shot: `pool = min(pool_max, pool + mul_bp(recoil, row16))`.
  - Each tick, once `ticks_since_shot >= 12`: `pool = max(pool_min, pool - regen/60)`.
  - Spread is `× row 15` (PRC).
  - Seed the archetype table from §2.1 × 1000. For example, an assault rifle has spread 1300 mdeg, hip pool 1500..12000, regen 8000 mdeg/s and recoil 3500 mdeg.
  - Sample uniformly inside the cone with canon's two spread draws: `r = (cone/2) × sqrt(u1)` using integer `isqrt` on fixed point, `θ = u2 × 65536` Bam. Stocks, barrels and scopes are Gloss parts that modify these five fields, as in BL1's part table.
- **R-SH2. Use linear falloff with a floor, and steal TF2's crit exemption.**
  - Per weapon: `falloff_start_mm, falloff_end_mm, falloff_floor_bp`, with linear interpolation. Measure distance on the impact tick, as TF2 does for projectiles.
  - `[PROPOSED]` defaults: pistol 12000/24000 mm, floor 5000; SMG 10000/20000, floor 5000; rifle 30000/60000, floor 6000; shotgun pellet 6000/15000, floor 3000. Snipers, rockets, grenades, bows and melee have no falloff (TF2's no-falloff classes).
  - No close-range ramp-up above 10000. TF2's 150% is tuned for PvP.
  - **Precision hits ignore falloff beyond the midpoint** (TF2: crits keep full damage past 512 HU). That gives the Gunslinger Calling a skill-expressive identity at no cost.
- **R-SH3. Amend: crit chance may exceed 10000 Bp, with Warframe tiers.**
  - `tier = cc / 10000 + (roll < cc % 10000 ? 1 : 0)`. It is still one draw per hit, as canon §3.4 requires.
  - `crit_mult = 10000 + tier × (crit_dmg_bp - 10000)`. Average = `10000 + mul_bp(cc, crit_dmg_bp - 10000)`.
  - **NPCs never crit Errata.** Warframe removed enemy crits in 35.0.9 and BL3 gives players no crit spot, in both cases because the spikes were unreadable.
  - Crit damage uses BL3's structure: one additive bucket (skills, talents, Refrains) times separate multiplicative manufacturer and archetype bonuses. Avoid BL2's divisor-penalty form, which even its wiki says confuses players.
- **R-SH4. Precision multiplier overrides.** Keep canon defaults (ranged 15000, melee 12500). `[PROPOSED]` archetype overrides: sniper 25000, revolver and hand cannon 20000, shotgun pellet 12500, beam 10000. Precedents run ×2 (BL base) to ×3 (TF2 sniper crit, Warframe heads).
- **R-SH5. Steal RoR2 proc coefficients as `proc_bp` on every hit source.**
  - Default 10000. Pellets 5000–7500 each. Beam ticks 2000. Rockets 10000. Charged sniper shots 20000–30000.
  - On-hit chances and on-hit durations are `× proc_bp`.
  - **Every damage tick produced by a status, surface, reaction, shock chain or item proc has `proc_bp = 0`.** That is the chain breaker; without it, co-op item stacks recurse.
  - Proc coefficients never scale crit chance (RoR2).
- **R-SH6. Projectile constants (from Minecraft).**
  - Arrows and bolts: gravity 5.556 mm/tick² (20 m/s²); drag `× 9967` Bp per tick in air and `× 8434` in liquid; full-draw bow 1000 mm/tick (60 m/s); crossbow 1050 mm/tick.
  - Bullets in genres with TECHNE ≥ 6 are hitscan out to 120 m, then become projectiles. GPV `projectile_speed_bp` and `projectile_drop_bp` (R6) retune per genre.
- **R-SH7. Aim assist, controller only `[PROPOSED]`.**
  - Slowdown: look sensitivity × 6000 while the reticle is within 2.5° of a hostile hitbox at ≤ 40 m.
  - Magnetism: bend the shot up to 1000 mdeg toward the nearest precision part inside that cone.
  - Both scale with the weapon's `aim_assist_bp`. A settings slider covers 0–100%. Never applied to mouse input.
  - It is presentation-adjacent but changes hit results, so it runs inside the sim on the input's aim vector, and the input log records `input_device`.

### Run pacing, loot and stacking (docs 11, 12, 14)

- **R-RS1. Use a RoR2-style credit director for spawns. Do not double-scale HP.**
  - Enemy HP and damage already scale by `H(d)` and `T(d)`, so the director scales **count only**: `credits_per_s = base_rate × (10000 + 2000 × erasure_stacks)/10000 × enemy_count_bp/10000`.
  - Spawn cost = template cost × rank multiplier. Elite rank ×6 and champion ×36, the RoR2 tier-1 and tier-2 elite multipliers.
  - Unspent credits carry over. A Folio-start "scene" budget is placed once.
- **R-RS2. Lint stacking curves (doc 11).** Only four curves are legal:
  - **linear** for rates (attack speed, move speed, reload);
  - **hyperbolic** for every avoidance or ignore chance, `10000 − 10000 × 10000 / (10000 + a × n)`;
  - **reciprocal** for cooldowns;
  - **exponential** only for curse-type trade-offs (Shaped Glass).
  - CI fails any affix or Refrain that stacks a chance linearly.
- **R-RS3. Serendipity as RoR2 Luck, closed form and one draw.** `p' = 10000 − (10000 − p)^(L+1) / 10000^L` for L ≥ 0, and `p^(|L|+1) / 10000^|L|` for L < 0, with `i128` intermediates. Then one draw against `p'`, so the draw count stays fixed. Never applied to Reliquary contents, Inkwell pulls or Fortune Engine rarity, which matches the brief and RoR2's exclusions of chests and shrines.
- **R-RS4. Fortune Engine pricing and table (RoR2 Shrine of Chance, made always-paying).**
  - Price × 14000 per use; at most 3 payouts per machine per Folio.
  - Rarity table = RoR2's success-conditional odds: Common 6546, Uncommon 1636, Equipment 1636, Legendary 182 Bp.
  - Every pull pays an item, per brief §8. This replaces R2 #19's table (conflict C6).
- **R-RS5. Colophon shop = RoR2 Multishop, per player.** 3 pedestals; one is always visible and each other is hidden with 2000 Bp chance; buying one locks the other two for that player only, since loot is instanced.
- **R-RS6. Folio Feelings from SPD, orthogonal to genre.**
  - Draw with `rng.lattice`, keyed by Folio. 5000 Bp none; otherwise one of 7 at 714 Bp each:
    - Chasm (vertical shafts)
    - Flooded
    - Overgrown
    - Dark (sight radius × 6250, SPD's ×5/8)
    - Large (standard rooms × 15000, +1 special, mobs × 13300, +2 item drops)
    - Trapped
    - Secretive (+1 secret)
  - A Folio gets at most 1 Feeling plus 1 bleed modifier (R6 R5).
- **R-RS7. Room, mob, item and trap budgets per Volume (SPD chapters → Volumes).**
  - Standard rooms: 4–6, 5–6, 6–7, 6–8, 8–9, with SPD's weights.
  - Special rooms: 1–2, 1–3, 2–3, 2–3, 2–3.
  - Floor items: 3/4/5 at 6000/3000/1000 Bp, per player (instanced).
  - Traps: `NormalIntRange(2, 3 + Volume)`.
  - Mob cap: `3 + Folio + rng(0..2)` before party scaling.
  - Default layout builder: 50% loop, 50% figure-eight. This is the Dormans major-cycle default (R4 §7.4) when no genre grammar overrides it.

### Co-op scaling (canon §17.2, doc 14)

- **R-CO1. Amend §17.2 trash-mob scaling so per-player threat falls below 1.0** (DRG 0.45 and RoR2 about 0.63 at 4 players; canon is currently 1.12).
  - Enemy HP for ranks minion to champion: 10000 / 14000 / 17500 / 20000.
  - `enemy_count_bp`: 10000 / 12500 / 15000 / 16500.
  - Budget 1.00 / 1.75 / 2.63 / 3.30, so per player 1.00 / 0.875 / 0.875 / 0.825 `[COMPUTED]`.
  - Keep boss HP (it already matches RoR2's ×living players) and enemy damage (it already matches DRG Hazard 3).
- **R-CO2. Friendly fire scales with difficulty (DRG).** `FF_DIRECT_BP` by difficulty tier 1–5: 1000 / 2000 / 3000 / 4000 / 7000. `FF_AREA_BP` stays at 10000 (explosions and surfaces are the readable co-op hazard).
- **R-CO3. Revive health falls with difficulty (DRG).** Revived HP: 6000 / 5000 / 4000 / 2000 / 1000 Bp of max.
- **R-CO4. Instance currency pickups as well as items** (BL3 Cooperation). Each Erratum sees and collects its own Lucre drop. RoR2's shared gold works in RoR2 only because its chests are single-claim.
- **R-CO5. Solo gets a familiar, not a discount** (DRG's Bosco). A solo Erratum receives a minor drone or familiar (genre-skinned, revive-capable once per Folio), so solo and duo budgets can stay close, as DRG's do.

### Melee and armor (doc 05)

- **R-ME1. Steal the Minecraft swing cooldown at 60 Hz, in integers.**
  - `T = ceil(60 / attack_speed)` ticks.
  - `mult_bp = min(10000, 2000 + 8000 × (2t + 3)² / (2T)²)`, where t is ticks since the last swing.
  - Affix "+element on hit" damage uses the unsquared ramp `2000 + 8000 × (2t + 3) / (2T)`.
  - A charged crit or Barony-style "shake" release needs `mult_bp >= 8480`, Minecraft's 84.8% crit gate.
  - This ends click-spam without a stamina cost per swing.
- **R-ME2. Amend: add defender `toughness` (Minecraft armor toughness).** In canon §10.5 step 5, replace `4 × d[t]` with `4 × d[t] × 8 / (8 + min(toughness, 20))`. Toughness 0 is today's canon. Toughness 20 makes large hits erode armor 3.5× more slowly. It is the heavy-armor identity knob, and `pen_bp` is its counter.

### Voxel world (docs 03, 07)

- **R-VX1. Breaking time (Minecraft), at 60 Hz.**
  - `ticks = ceil(90 × hardness_milli / tool_speed_milli)` with a harvesting tool; `ceil(300 × hardness_milli / 1000)` without.
  - Tool speeds by material tier: wood 2000, stone 4000, bronze 5000, iron 6000, steel 8000, mithril or plasteel 9000, gold 12000 (fast, but low durability).
  - An Efficiency-style affix adds `(level² + 1) × 1000`.
  - Time × 5 while submerged, × 5 while airborne.
  - `[PROPOSED]` derive `hardness_milli = SV × 25` from IVAN StrengthValue, which makes copper SV 70 ≈ 1750, about Minecraft stone.
- **R-VX2. Explosions use Minecraft's 1352-ray march, in fixed point.**
  - Step 300 mm.
  - Per step: decay 225 milli-units, plus `(blast_res + 300) × 300 / 1000` for a non-air cell.
  - Ray start = `power × (7000 + rng(0..6000))/10000`, from keyed `rng.fire` with the ray index, so 1352 draws per explosion.
  - Entity damage: `7 × 2P × (i² + i)/2 + 1`.
  - At most 4 explosions are processed per tick; the rest queue in event order.
  - Blast resistance is a column in the shared material table.
- **R-VX3. Fluids (Minecraft levels, Qud volumes).**
  - Cell fill in eighths. Water spreads one cell per 15 ticks (4 m/s) with level +1 per cell up to 7. Lava spreads one cell per 90 ticks with +2 per cell up to 6.
  - Drop search 5 cells for water and 3 for lava. The infinite-source rule applies to water only.
  - GPV `liquid_viscosity_bp` scales the spread interval.
- **R-VX4. Structural integrity: Valheim support propagation on the 1 m grid.**
  - Constants: Distance 1100 mm; VerticalLoss for ±Y neighbors (wiki errata, §6.4); HorizontalLoss for lateral neighbors; mutual support for two opposite lateral parents.
  - Material columns `support_min, support_max, loss_v_bp, loss_h_bp`, for example wood 10/100/1250/2000, stone 100/1000/1250/10000, iron 20/1500/769/769.
  - **7DTD shortcut:** a cell in an unbroken vertical column down to a grounded cell is always stable (O(1)).
  - Recompute by BFS from the edited cell in ascending cell index, at most 512 cells per tick, with the rest carried to the next tick.
  - Collapse spawns sim-side falling-block entities that deal impact damage through `FF_AREA_BP`.
  - Emergent consequence: stone cannot cantilever at all (0 blocks) and wood reaches 9 blocks, so arches, pillars and iron beams become real building decisions.

### Barony systems (docs 07, 08, 10)

- **R-BA1. Tinkering is the in-run crafting spine.**
  - Two scrap currencies, `mat.scrap_common` and `mat.scrap_arcane`, genre-skinned (scrap, bits, salvage, aether-shards).
  - Salvage anywhere with a kit. Kit degrade 500 Bp per use.
  - Recipes gated by `skill.tinkering + d(PER sub)`. Port Barony's 16-recipe ladder (§7.1) as the content seed.
  - Robot or turret quality tiers Simple/Complex/Intricate/Artisan at skill 40/60/80/100, each unlocking one command.
- **R-BA2. Generalize Barony's lock health as an anti-frustration rule.** Every per-attempt chance in the sim (lockpicking, hacking, disarming, persuasion, examination) also accumulates deterministic progress: lock HP 20 for doors and 40 for chests, damage per attempt 3 / 4–5 / 6–11 / 8–17 by skill band 0–19 / 20–59 / 60–99 / 100. Success is guaranteed once progress reaches the lock HP. This is gacha pity applied to skill checks.
- **R-BA3. The Alembic grid sits beside SPD's 3-seed pot.**
  - Potion × potion recipes (Base × Secondary) give the "brew from what you found" layer.
  - Water duplication `5000 + 1000 × floor(skill/20)` Bp, capped at 10000; failure turns the potion into water.
  - Bottle recovery `5000 + 50 × skill`.
  - Alembic degrade 250 Bp per brew, 1000 with the acid-class reagent.
  - Fixed use reveals at alchemy 20, 40 and 60.
  - `[PROPOSED]` co-op guard: at most 3 duplications per potion type per Folio per player, so 4 players cannot multiply one healing potion without limit.
- **R-BA4. Engineered noise counts as stealth play.** Barony grants Tinkering XP for kills of enemies "investigating noisemaker" at 47% if they were unaware of the player. Doc 09 should grant Sneak or Tinkering progress for kills set up through deliberately generated noise events.

### Process

- **R-PR1.** Each owning spec must record its resolution of conflicts C1–C9 (§1.4) in its Imports section.
- **R-PR2.** Doc 05 must cite §2.1–§2.6 for every shooting default, and mark the R-SH7 aim-assist numbers as playtest-tuned, since no shipped value was verifiable.
