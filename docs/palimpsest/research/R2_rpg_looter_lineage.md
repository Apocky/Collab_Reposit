# R2: The RPG, looter and sandbox lineage (Morrowind, Oblivion, Baldur's Gate, Borderlands, Minecraft, Destiny 2)

As of 2026-10-11. Research dossier for PALIMPSEST. Author: research agent R2. Status: input to `spec/00_CANON.md` and specs 04 to 15.

## Table of contents

0. Method, sources and evidence tags
1. Morrowind (2002): open formulas for nearly everything
2. Oblivion (2006) and Oblivion Remastered (2025): detection, power-law magic, and the level-scaling failure
3. Baldur's Gate 1, 2 and 3: d20 math, surfaces, verticality, sight cones, rests and illithid powers
4. Borderlands 1 to 4: part-based guns, licensed parts, rarity, elements, Firmware, UVHM, slot machines and Vault Cards
5. Minecraft: voxel scale, ticks, light, spawning, redstone, enchanting, anvils, brewing and the 2025 to 2026 drops
6. Destiny 2: perk columns, gear tiers, Armor 3.0, Power, Portal, Rewards Pass, Eververse and the June 2026 end of live service
7. Cross-reference: best source for each PALIMPSEST system
8. Design implications for PALIMPSEST

---

## 0. Method, sources and evidence tags

- **[VERIFIED url]** means I read the page this session. For UESP (`en.uesp.net`) the HTML is behind a Cloudflare challenge. I read the same page's wikitext through the MediaWiki API (`/w/api.php?action=parse&prop=wikitext`), so the URL given is the canonical article URL. The same API route was used for bg3.wiki, minecraft.wiki, borderlands.fandom.com, destiny.fandom.com, en.wikipedia.org and the Oblivion Construction Set wiki (`cs.uesp.net`). Borderlands patch notes were read through the Zendesk help-center JSON API of `support.borderlands.com`.
- **[SEARCH url]** means I saw the claim only in a search-engine summary of that page and did not read the page itself. Treat it as medium confidence.
- **[RECALLED]** means the claim comes from memory and was not checked this session.
- Formulas are copied exactly as the source gives them. When I convert one into PALIMPSEST units (basis points, 60 Hz ticks, the 8×3 attribute scheme), I label it **PALIMPSEST adaptation**. It is then my proposal, not the original game's value.
- The OpenMW research wiki (`wiki.openmw.org`, "Research:" namespace) is a primary source for Morrowind. The OpenMW team reverse-engineered the original engine and marks each formula "Implemented / Verified".

---

## 1. Morrowind (2002)

Morrowind is the most transparent RPG in this lineage. Almost every success chance uses one shape: **(actor's rate) − (opponent's rate)**, where each rate is skill plus fractions of attributes and Luck, multiplied by a fatigue term. PALIMPSEST should use this shape as its canonical skill-check template.

### 1.1 Character structure

- 8 attributes: Strength, Intelligence, Willpower, Agility, Speed, Endurance, Personality, Luck. 27 skills. Each class has 5 Major, 5 Minor and 17 Miscellaneous skills [VERIFIED https://en.uesp.net/wiki/Morrowind:Skills].
- Base starting value of every skill is 5. Each Major skill gets +25, each Minor +10, and each skill of the chosen Specialization (Combat, Magic, Stealth) +5, plus racial bonuses. The highest possible starting skill is 50 [VERIFIED https://en.uesp.net/wiki/Morrowind:Skills].
- Governing attributes [VERIFIED https://en.uesp.net/wiki/Morrowind:Skills]:

| Attribute | Governed skills |
|---|---|
| Endurance | Heavy Armor, Medium Armor, Spear |
| Strength | Acrobatics, Armorer, Axe, Blunt Weapon, Long Blade |
| Agility | Block, Light Armor, Marksman, Sneak |
| Speed | Athletics, Hand-to-hand, Short Blade, Unarmored |
| Personality | Illusion, Mercantile, Speechcraft |
| Intelligence | Alchemy, Conjuration, Enchant, Security |
| Willpower | Alteration, Destruction, Mysticism, Restoration |
| Luck | none |

- Luck starts at 40 for everyone, or 50 if it is a favored attribute of a custom class. It governs no skill, so it never gets a level-up multiplier and rises at most +1 per level. Luck adds to success chances but never to derived pools or loot quality. "Luck has no effect on the quality or quantity of random loot" [VERIFIED https://en.uesp.net/wiki/Morrowind:Luck].

### 1.2 Use-based skill progress (exact, from the reverse-engineered engine)

```python
# OpenMW Research:Stats and Levelling, status Implemented/Verified
progress[skill] += skillGain            # skillGain = skill.gainFactor[action] from the skill record
progressRequirement = 1 + playerSkills[skill]
if skill in majorSkills:   progressRequirement *= fMajorSkillBonus
elif skill in minorSkills: progressRequirement *= fMinorSkillBonus
elif skill in miscSkills:  progressRequirement *= fMiscSkillBonus
if skill in class.specialization.skills: progressRequirement *= fSpecialSkillBonus
if int(progress[skill]) >= int(progressRequirement):
    progress[skill] = 0; playerSkills[skill] += 1
```
[VERIFIED https://wiki.openmw.org/index.php?title=Research:Stats_and_Levelling]

- GMST defaults: fMajorSkillBonus 0.75, fMinorSkillBonus 1.0, fMiscSkillBonus 1.25, fSpecialSkillBonus 0.8 [RECALLED].
- Barter gain scales with the bargain percentage: `skillGain *= int(100 * (finalPrice - initialOffer) / finalPrice)` when selling above the initial offer, and the mirror when buying [VERIFIED https://wiki.openmw.org/index.php?title=Research:Stats_and_Levelling].
- Per-action gain examples from UESP skill boxes: Alchemy "Create Potion +2.0, Use Ingredient +0.5"; Enchant "Recharge Item +5.0, Create Magic Item +5.0, Cast When Used +0.1, Cast When Strikes 0"; Sneak "Successful Pick Pocket +2.0, Avoid Notice +0.25" [VERIFIED https://en.uesp.net/wiki/Morrowind:Alchemy; https://en.uesp.net/wiki/Morrowind:Enchanting; https://en.uesp.net/wiki/Morrowind:Sneak].
- Sneak only trains within about 5 m of a creature or NPC. Time spent sneaking away from anyone is banked and paid out when you next come in range [VERIFIED https://en.uesp.net/wiki/Morrowind:Sneak].

### 1.3 Level-up and the attribute multiplier table

- 10 Major/Minor skill increases make you eligible for a level. You level when you next rest in a bed or in the wild; waiting does not count. Skill books and trainers count [VERIFIED https://en.uesp.net/wiki/Morrowind:Level].
- You raise 3 attributes. Each attribute's bonus depends on how many times skills it governs rose since the last level. Major, Minor and Misc increases all count [VERIFIED https://en.uesp.net/wiki/Morrowind:Level]:

| Governed-skill increases since last level | Attribute gain if chosen |
|---|---|
| 0 | +1 |
| 1–4 | +2 (×2) |
| 5–7 | +3 (×3) |
| 8–9 | +4 (×4) |
| ≥10 | +5 (×5) |

- Major/Minor overflow rolls over to the next level's 10-count, but multiplier counts do not roll over [VERIFIED https://en.uesp.net/wiki/Morrowind:Level].
- Health per level is +10% of current Endurance, applied after that level's Endurance gain. It is not retroactive, so Endurance taken early pays off. One decimal of hidden precision is kept [VERIFIED https://en.uesp.net/wiki/Morrowind:Level].
- Max level through normal play is 78: Majors start at 30, Minors at 15, giving 775 counting increases. Skill books raise it to 83 [VERIFIED https://en.uesp.net/wiki/Morrowind:Level].
- Known design flaw: optimal play means grinding Misc skills to engineer ×5s. The system rewards bookkeeping more than play [VERIFIED https://en.uesp.net/wiki/Morrowind:Level].

### 1.4 Derived pools

| Pool | Formula | Source |
|---|---|---|
| Starting Health | `(Strength + Endurance) / 2` | [VERIFIED https://en.uesp.net/wiki/Morrowind:Health] |
| Health per level | `+0.1 × Endurance` | same |
| Health regen while resting | `0.1 × Endurance` per hour (resting, not waiting) | same |
| Max Fatigue | `Strength + Willpower + Agility + Endurance` (max 400 unbuffed) | [VERIFIED https://en.uesp.net/wiki/Morrowind:Fatigue] |
| Fatigue regen | `2.5 + 0.02 × Endurance` per second | same |
| Max Magicka | `Intelligence × (1 + racial mod + birthsign mod)`; Breton +0.5, Altmer +1.5, Mage +0.5, Apprentice +1.5, Atronach +2.0 | [VERIFIED https://en.uesp.net/wiki/Morrowind:Magicka] |
| Magicka regen | Resting only, `0.15 × Intelligence` per hour. Atronach gets none | same |
| Drowning | Breath 20 s, then 3 HP/s | [VERIFIED https://en.uesp.net/wiki/Morrowind:Health] |

### 1.5 The universal fatigue term

```python
fatigueTerm = fFatigueBase - fFatigueMult * (1 - normalizedFatigue)
# normalizedFatigue = max(0, current/max), NOT capped above 1 (fortified fatigue helps)
# "fatigueTerm is normally 1.25 at full fatigue with default GMSTs"
```
[VERIFIED https://wiki.openmw.org/index.php?title=Research:Common_Terms]

UESP writes the same thing as `(0.75 + 0.5 × Current Fatigue / Maximum Fatigue)`, which gives 0.75 when empty and 1.25 when full. It multiplies hit, evasion, block, spell success, sneak, spot, security, enchanting and barter. Even Mercantile and jump height fall as fatigue drops [VERIFIED https://en.uesp.net/wiki/Morrowind:Fatigue].

### 1.6 Combat

All of the following are [VERIFIED https://en.uesp.net/wiki/Morrowind:Combat].

- **Hit chance** = `(Attacker Hit Rate − Defender Evasion)%`
- **Hit Rate** = `(Weapon Skill + Agility/5 + Luck/10) × (0.75 + 0.5 × CurFatigue/MaxFatigue) + Fortify Attack magnitude + Blind magnitude`. Blind adds here because of a bug.
- **Evasion** = `(Agility/5 + Luck/10) × (0.75 + 0.5 × CurFatigue/MaxFatigue) + min(Sanctuary, 100) + min(Chameleon/5, 100)`. It is 0 except Chameleon while the target is paralyzed, knocked down or knocked out. It is also 0 against ranged sneak attacks.
- **Block chance** = `(Block Rate − Attacker Hit Chance)%`. Block Rate = `(Block + Agility/5 + Luck/10) × fatigueTerm`, ×1.25 if not moving forward. Clamped to `[iBlockMinChance 10%, iBlockMaxChance 50%]`. Hits are resolved evade first, then block. Example from the page: 30% to beat evasion and 20% to beat block gives 6% clean hit, 24% shield hit (on-strike enchantments still fire) and 70% miss.
- **Damage** = `(Weapon Damage × Strength Modifier × Condition Modifier × Critical Modifier) / Armor Reduction`
  - Strength Modifier = `(Strength + 50) / 100`, so 0 STR gives ×0.5 and 100 STR gives ×1.5.
  - Condition Modifier = current / max condition.
  - Weapon Damage is not random. It is chosen inside the weapon's min–max range by how long the swing was charged. NPCs roll the charge randomly.
  - Bows and crossbows add arrow damage to bow damage. Thrown weapons count twice, as weapon and projectile.
  - Critical (unseen) multiplier: ×4 melee, ×1.5 ranged. Cast-on-Strike enchantment damage is never multiplied.
- **Hand-to-hand**: `(H2H Skill / 2) × Crit` to Fatigue. `(H2H × 0.075 × Crit) / ArmorReduction` to Health only once the target is down or paralyzed.
- **Armor Rating** per piece = `BaseAR × (ArmorSkill / 30)`. Each unarmored slot = `Unarmored² × 0.0065`.
- **Total AR** = `Chest×0.3 + (Shield+Head+Legs+Feet+RShoulder+LShoulder)×0.1 + (RHand+LHand)×0.05`
- **Armor reduction**: damage is divided by `min(1 + AR/Damage, 4)`. Damage never drops below 25% of the original. Big hits lose proportionally less: AR 33 against 50 damage takes 40% off, against 100 damage only 25%.
- Knockdown: a downed target takes +50% physical damage and has 0 evasion. 100 Agility makes you immune to knockdown.
- Spell damage ignores AR and never crits: `Damage × (1 − (Resistance − Weakness)/100)`. Resistance minus weakness of 100 or more resists the effect entirely [VERIFIED https://en.uesp.net/wiki/Morrowind:Spells].

**Worked example (hit):** Weapon Skill 50, Agility 50, Luck 40 at full fatigue gives `(50 + 10 + 4) × 1.25 = 80`. Defender Agility 40, Luck 40 at full fatigue gives `(8 + 4) × 1.25 = 15`. Hit chance is 65%.

### 1.7 Sneak (detection)

[VERIFIED https://en.uesp.net/wiki/Morrowind:Sneak]

- Check is `(Player Elusiveness − Target Spot Chance)%`. It runs every couple of seconds for idle targets and every few frames for targets actively searching.
- **Elusiveness** = `(Sneak + Agility/5 + Luck/10 − ShoeWeight) × (0.5 + Distance/500) × fatigueTerm + Chameleon`
- **Spot chance** = `(Sneak + Agility/5 + Luck/10 − Blind) × fatigueTerm × DirectionMult`. DirectionMult is 1.5 if the player is in front of or beside the target and 0.5 if behind.
- Pickpocketing is effectively capped near 56% in the original engine. A 75% cap is rolled twice, once on take and once on close. Paralyzed or knocked-down targets can be robbed freely, but witnesses still report the crime.

### 1.8 Security (locks and traps)

Chance = `(Stats Modifier × Equipment Modifier × Fatigue Modifier) − Lock Difficulty`, where Stats Modifier = `Security + Agility/5 + Luck/10` and Equipment Modifier is the lockpick or probe quality. A result of 0 or less gives "lock too complex", and the attempt still costs a use [VERIFIED https://en.uesp.net/wiki/Morrowind:Security].

### 1.9 Alchemy

[VERIFIED https://en.uesp.net/wiki/Morrowind:Alchemy]

- Effects visible per ingredient by Alchemy skill: 0–14: 0; 15–29: 1; 30–44: 2; 45–59: 3; 60+: all 4. **Hidden effects still combine.** If two ingredients share a hidden effect, the mixing screen reveals it. Experimenting is a second identification channel.
- Eating raw gives a weak dose of the first effect and trains at 1/4 the rate of brewing.
- Success = `Alchemy + Intelligence/10 + Luck/10`. Failure destroys the ingredients. Only a success gives XP.
- `BasePotionStrength = (Alchemy + Int/10 + Luck/10) × MortarQuality / (3 × EffectBaseCost)`
- `BasePotionDuration = (Alchemy + Int/10 + Luck/10) × MortarQuality / EffectBaseCost`
- Apparatus modifiers, applied to both strength and duration:

| Effect polarity | Apparatus used | Rule |
|---|---|---|
| Positive | Retort or Calcinator | + Quality |
| Positive | Retort and Calcinator | + Calcinator + 2 × Retort |
| Negative | Alembic only | ÷ (Alembic + 1) |
| Negative | Calcinator only | + round(Calcinator), min 1 |
| Negative | Alembic and Calcinator | ÷ (2 × Alembic + 3 × Calcinator) |

- Effects with magnitude only or duration only (Dispel, Invisibility, Paralyze, Water Breathing, Water Walking) multiply instead: positive with Retort or Calcinator gives ×(Q + 0.5).
- Mortar qualities: Apprentice 0.5, Journeyman 1.0, Master 1.2, Grandmaster 1.5, SecretMaster 2.0.
- Potion value = `(Alchemy + Int/10 + Luck/10) × MortarQuality`. Value ignores which effects the potion has and which other apparatus was used. Potion weight is the average of the ingredient weights, rounded down.
- Fortify Intelligence potions feed back into potion strength. This is the famous "super-potion" exploit loop.

**Worked example (PALIMPSEST-relevant shape):** Alchemy 50, Intelligence 60, Luck 40 gives a base of 60. With a Journeyman mortar (1.0) and an effect of base cost 5: strength = 60 / 15 = 4, duration = 12 s. Adding a Journeyman retort (quality assumed 1.0) gives strength 5 and duration 13.

### 1.10 Spellmaking and casting

[VERIFIED https://en.uesp.net/wiki/Morrowind:Spells; https://en.uesp.net/wiki/Morrowind:Spellmaking]

- Up to 8 effects per spell. The same effect cannot appear twice, except attribute and skill effects.
- **Built-in (auto-calc) cost per effect** = `((Min + Max) × Duration × (BaseCost/40) + Area × BaseCost/40) × (1.5 if Target)`. Round half up.
- **Custom spell cost per effect** = `(2 × AvgMagnitude × (Duration + 1) × (BaseCost/40) + Area × BaseCost/40) × (1.5 if Target)`. Round down, minimum 1. Every variable counts as at least 1.
- Consequence: damage spread over time is cheaper. 100 points for 1 s costs 50 magicka. 50 points for 2 s costs 37.
- **Gold price of spellmaking** = 7 × magicka cost. Disposition and Mercantile have no effect.
- **Cast chance** = `(SchoolSkill × 2 + Willpower/5 + Luck/10 − SpellCost − Sound) × fatigueTerm`. The spellmaker's preview assumes half fatigue. Example: 20 Destruction, 60 WIL, 50 LCK and Fire Bite (cost 6) at full fatigue give `(40 + 12 + 5 − 6) × 1.25 = 63.75%`.
- A multi-effect spell belongs to the school of the effect with the **lowest** cast chance, and it re-tags as skills change.
- Bug to avoid: cost is stored as an unsigned 16-bit integer, so a cost of 65,542 wraps to 6 while the spellmaker still charges 458,794 gold.

### 1.11 Enchanting

[VERIFIED https://en.uesp.net/wiki/Morrowind:Enchanting]

- Cost (enchantment points) per effect:
  - Cast-when-used or on-strike, self or touch: `⌊BaseCost × ((Min + Max) × Duration + Area) / 40⌋`
  - On target: `⌊1.5 × BaseCost × ((Min + Max) × Duration + Area) / 40⌋`
  - **Constant Effect**: `⌊BaseCost × ((Min + Max) × 100 + Area) / 40⌋`. Duration is treated as 100.
- **Compounding:** `C = Σ C_i × (n − i + 1)`. The first-listed effect is multiplied by n and the last by 1. Order matters, so put the expensive effect last.
- **Self-enchant success** = `(0.75 + %Fatigue/2) × (1 − 0.5 × IsConstant) × (Enchant + Int/5 + Luck/10 − 3 × EnchantPoints)`. It is checked per effect. Maxed unbuffed: 100% at 16 points or less for non-constant effects, 0% above 43 points. Constant Effect is half as likely. NPC enchanters always succeed but charge heavily.
- **Charge use per activation** = `baseCost × (1.1 − Enchant/100)`, minimum 1.
- **Passive recharge** is 1 charge every 20 s, even while the item is not carried.
- Constant Effect needs a soul of 400 or more, which only a few creatures have, in a Grand gem or Azura's Star. Item capacity is the limit, not soul size.
- Soul gem capacity: Petty 30, Lesser 60, Common 120, Greater 180, Grand 600, Azura's Star 15,000 (reusable). A soul goes into the smallest empty gem that fits.
- Item capacity runs 0–225 and is usually 1–20. The Daedric Tower Shield has 225.

### 1.12 Mysticism, Levitate and mobility

- Mysticism covers soul-trapping, teleport (Mark, Recall, Almsivi and Divine Intervention), telekinesis, absorb and reflect, and detection. Trainers suggest making a custom spell costing 1 and casting it until your magicka is empty [VERIFIED https://en.uesp.net/wiki/Morrowind:Mysticism].
- Levitate base cost is 3.0. Flight speed depends on magnitude and the Speed attribute, not Athletics. "Levitate 1 pt + Fortify Speed" is more efficient than a high-magnitude Levitate. A low-magnitude Levitate cast on an enemy works as a cheap immobilize. Melee enemies who cannot reach a levitating player flee [VERIFIED https://en.uesp.net/wiki/Morrowind:Levitate].
- Fly speed (exact):
```python
flySpeed = 0.01 * (speedAttribute + levitationBonus)
flySpeed = fMinFlySpeed + flySpeed * (fMaxFlySpeed - fMinFlySpeed)
flySpeed *= 1 - fEncumberedMoveEffect * normalizedEncumbrance
```
[VERIFIED https://wiki.openmw.org/index.php?title=Research:Movement]

- Design lesson: in Morrowind, free 3D movement breaks level geometry, and the game accepted that. PALIMPSEST's voxel world is buildable, so flight breaks nothing.

### 1.13 Barter and item value

```python
# OpenMW Research:Trading and Services (Implemented/Verified)
clampedDisposition = clamp(int(npcDisposition), 0, 100)
a = min(pcMercantile,100); b = min(0.1*pcLuck,10); c = min(0.2*pcPersonality,10)
d = min(npcMercantile,100); e = min(0.1*npcLuck,10); f = min(0.2*npcPersonality,10)
pcTerm  = (clampedDisposition - 50 + a + b + c) * pcFatigueTerm
npcTerm = (d + e + f) * npcFatigueTerm
buyTerm  = 0.01 * (100 - 0.5 * (pcTerm - npcTerm))
sellTerm = 0.01 * (50  - 0.5 * (npcTerm - pcTerm))
x = buyTerm if buying else min(buyTerm, sellTerm)
# item base price: weapons/armor × (durability/maxDurability); tools × (uses/maxUses);
# filled soul gem price = soul points × empty gem price
```
[VERIFIED https://wiki.openmw.org/index.php?title=Research:Trading_and_Services]

Item value is a fixed field on each record, scaled only by condition. Value is an authored constant, not a derived quantity. Enchanted-item value is set when it is enchanted.

### 1.14 What Morrowind teaches

1. One check template, `(rate − opposed rate) × fatigue`, used everywhere, makes a game learnable and moddable.
2. Spellmaking and enchanting with transparent cost formulas create emergent builds and exploits. Keep the freedom and bound the arithmetic.
3. The level-up multiplier rewards meta-gaming bookkeeping. Keep the idea that what you practice is what grows, and drop the grind.
4. Skill-gated knowledge (alchemy effects 15/30/45/60) plus "experiment to reveal" is exactly an identification system.

---
## 2. Oblivion (2006) and Oblivion Remastered (2025)

### 2.1 Release facts

- *The Elder Scrolls IV: Oblivion Remastered* was shadow-dropped on **April 22, 2025** for PC, Xbox Series X|S and PS5. It reached Nintendo Switch 2 on **August 11, 2026**. Developer: Virtuos. It uses **Unreal Engine 5 for rendering and UI on top of a heavily modified Gamebryo that still runs all game logic** [VERIFIED https://en.uesp.net/wiki/Oblivion:Oblivion_Remastered].
- Architectural lesson: logic and rendering split cleanly enough that a 2006 simulation runs unchanged under a 2025 renderer. PALIMPSEST's `palimpsest_sim` / Bevy split is the same idea, done on purpose.

### 2.2 Structure

- 21 skills, 7 Major. Each attribute governs 3 skills, except Luck, which governs none [VERIFIED https://en.uesp.net/wiki/Oblivion:Skills].
- **Mastery ranks**: Novice 0–24, Apprentice 25–49, Journeyman 50–74, Expert 75–99, Master 100. Every skill grants a perk at each rank. This is the direct ancestor of PALIMPSEST's 25/50/75/100 thresholds [VERIFIED https://en.uesp.net/wiki/Oblivion:Alchemy].
- **Luck as a hidden skill modifier**: `SkillModifiedByLuck = Skill + fActorLuckSkillMult × (Luck + iActorLuckSkillBase)` = `Skill + 0.4 × (Luck − 50)`. It applies to every skill except Acrobatics, Athletics and Speechcraft. The result is clamped to 0–100 and never grants mastery perks, except Armorer [VERIFIED https://en.uesp.net/wiki/Oblivion:Luck].

### 2.3 Skill XP curve

- `Base_experience = 0.25883 × skill_level^1.5`, from `FSkillUseMinor × (FSkillUseFactor × level)^FSkillUseExp` with defaults 1.25, 0.35 and 1.5 [VERIFIED https://en.uesp.net/wiki/Oblivion:Increasing_Skills].
- Multipliers: Specialization ×0.75; Major ×0.6; Specialization and Major ×0.45. Example: level 50 needs 91.51 XP for a Minor skill, 68.63 with Specialization, 54.90 as a Major, 41.16 for both [VERIFIED https://en.uesp.net/wiki/Oblivion:Increasing_Skills].
- Trainers and skill books grant exactly one full level of XP. Training is limited to 5 sessions per level [VERIFIED https://en.uesp.net/wiki/Oblivion:Leveling].

### 2.4 Original leveling and the level-scaling failure

- 10 Major-skill increases give a level. The multiplier table is identical to Morrowind's (0 → +1, 1–4 → +2, 5–7 → +3, 8–9 → +4, 10+ → +5) and counts Major and Minor increases. Surplus does not roll over [VERIFIED https://en.uesp.net/wiki/Oblivion:Leveling].
- **Health** +10% of Endurance per level [VERIFIED https://en.uesp.net/wiki/Oblivion:Leveling].
- Leveled creatures [VERIFIED https://en.uesp.net/wiki/Oblivion:Leveled_Lists]:
  - `Health = HealthMultiplier × CreatureLevel`
  - `CombatSkill = Combat + 2 × CreatureLevel`, and the same for Magic and Stealth
  - `Attack Damage = AttackMultiplier + 0.5 × CreatureLevel`
  - Random lists drop creatures more than `ILevCreaLevelDifferenceMax = 8` levels below the player.
  - From player level 15, enemy variants scale directly with player level. For example, Ogres gain 26 HP per player level [VERIFIED https://en.uesp.net/wiki/Oblivion:Leveling].
- **The Leveling Problem**: a character who levels through non-combat Majors such as Speechcraft and Mercantile meets harder enemies without getting stronger. Players report a difficulty wall at levels 10–15, and at levels 20–25 enemy health pools outgrow player damage. A player with 100 Blade and 100 Strength stops gaining damage while enemies keep scaling [VERIFIED https://en.uesp.net/wiki/Oblivion:Leveling]. **This is the canonical anti-pattern PALIMPSEST must avoid. Never scale enemies by player level.**

### 2.5 Oblivion Remastered changes

All [VERIFIED https://en.uesp.net/wiki/Oblivion:Remastered_Leveling]:

- **All skills** now feed character level through XP, as in Skyrim.
- Attribute gains are no longer tied to which skills rose. Each level gives **12 "virtues"**. One virtue gives +1 to an attribute. At most **5 virtues per attribute**, at most **3 attributes** per level, and **Luck costs 4 virtues per point** with a maximum of +1 per level.
- Endurance gains are now **retroactive** to health.
- Enemy scaling "has been adjusted". The exact new curves are not published on UESP.
- Magic-school XP now scales with spell cost: `((SpellBaseCost × [School]ExpCastMult) / 0.75) + ([School]UseValue / 1.5)`, with ExpCastMult = 0.1 for all schools. **Zero-cost spells give 0 XP.** This kills the "cast the 1-magicka spell forever" grind.
- Mercantile XP scales with the total price of items sold. Athletics trains faster while sprinting (0.1 per second against 0.05 for running).
- Per-action XP table changes, original → Remaster: Armorer 1.5 → 2.5; Athletics running 0.03 → 0.05, swimming 0.04 → 0.075; Blade hit 0.5 → 0.6; Blunt 0.5 → 0.7; Block 1.25 → 1.5; Hand-to-Hand 0.6 → 0.5; Heavy Armor hit taken 1.25 → 1.5.
- Perks were reworked to give some skills real mid-game effects. Examples: Expert Blade "20% chance to make the target Bleed for 10 s"; Expert Blunt "20% chance to Silence for 10 s"; Hand-to-Hand fatigue damage at 50%, 75% and 100% of health damage by rank; Expert Alchemy "always crafts an additional potion"; Expert Security "only one tumbler falls when a pick breaks"; Expert Sneak "harder to detect in bright environments"; Expert Mercantile "+500 gold in all shops".
- Security now gives XP for Auto-Attempt whether or not it succeeds.

### 2.6 Physical damage

All [VERIFIED https://en.uesp.net/wiki/Oblivion:The_Complete_Damage_Formula]:

```
Damage = WeaponRating × (Fatigue/MaxFatigue + 1)/2 × max(SneakMult, PowerAttackMult)
         × OpponentArmorRating × OpponentWeaponResistance
WeaponRating = BaseWeaponDamage × (0.75 + Attribute/200) × (0.2 + ModifiedSkill × 0.015)
               × (WeaponHealth/BaseWeaponHealth + 1) / 4
OpponentArmorRating = (100 − CombinedArmorRating)/100,  CombinedArmorRating ≤ 85
ArmorPiece = BaseArmorRating × (0.35 + 0.0065 × ArmorSkill) × ArmorHealth/MaxArmorHealth
H2H Health_Damage = 1 + 10.5 × (Strength/100) × (ModifiedSkill/100);  Fatigue_Damage = 1 + 0.5 × Health_Damage
Spell = Magnitude × (100 − Resist + Weakness)/100
```
- GMSTs: fDamageWeaponMult 0.5, fDamageStrengthBase 0.75, fDamageStrengthMult 0.5, fDamageSkillBase 0.2, fDamageSkillMult 1.5, fDamageWeaponConditionBase 0.5, fDamageWeaponConditionMult 0.5.
- Power attack ×2.5, or ×3 for a standing power attack at Apprentice or above. Sneak and power attack **do not stack**: the game takes the max. Otherwise the multiplier would reach ×18.
- Master Sneak makes unseen attacks ignore armor entirely (OpponentArmorRating = 1).

### 2.7 Spellmaking (power-law magnitude)

[VERIFIED https://en.uesp.net/wiki/Oblivion:Spell_Making]

```
B = BaseCost/10;  M = Magnitude^1.28;  D = Duration;  A = Area × 0.15
Cost = B × M × D × A      (any factor < 1 is treated as 1);  × 1.5 if Target
Final cost multiplier = 1.4 − 0.012 × LuckModifiedSkill        (0 skill: ×1.4; 100 skill: ×0.2)
Gold price = 3 × player's magicka cost
```
- Skill gate by **base** cost of the whole spell: under 26 needs nothing; 26–62 needs 25; 63–149 needs 50; 150–399 needs 75; 400 or more needs 100. For multi-effect spells, the school comes from the single most expensive effect. A bug checks only that one effect's cost when deciding whether you can create the spell.
- Example: Fire Damage (base 7.5) at 3 points for 12 s gives `0.75 × 3^1.28 × 12 ≈ 36.7` magicka. 6 points for 6 s gives 44.6 for the same total damage. Longer durations are cheaper.

### 2.8 Enchanting (Constant Effect)

`Effect Magnitude = BaseCost × CE_Factor × SoulLevel + fMagicCEEnchantMagOffset(5.0)`, with `CE_Factor = (Power − 5) / SoulGemNumber / BaseCost` and SoulGemNumber Petty 1 to Grand 5. Gold cost = `Magnitude × BarterFactor` [VERIFIED https://en.uesp.net/wiki/Oblivion:Enchanting]. Sigil Stones give pre-rolled enchantments whose power is set by player level.

### 2.9 Alchemy (4 effects, revealed by mastery)

[VERIFIED https://en.uesp.net/wiki/Oblivion:Alchemy]

- Visible effects per ingredient: Novice 1, Apprentice 2, Journeyman 3, Expert all 4. **Master (100) can brew from one ingredient**, which applies the first effect only. Unlike Morrowind, hidden effects **cannot** be combined.
- `Effective_Alchemy = Alchemy + 0.4 × (Luck − 50)`, clamped 0–100. Base skill and Luck only, so Fortify effects are ignored. Intelligence plays no part.
- Equivalent magicka cost with mortar only: `Effective_Alchemy + MortarPestle_Strength × 25`
- `Base_Mag = [(Effective_Alchemy + Mortar × 25) / (EffectBaseCost/10 × 4)]^(1/2.28)`, and Base_Dur = 4 × Base_Mag for most effects.
- Master equations: `Magnitude = Base_Mag × (1 + Calc_Fac×Calc + Ret_Mag_Fac×Retort − Alem_Fac×Alembic)`, and the same for Duration with Ret_Dur_Fac. Factors for most effects: Calc 0.35, Ret_Dur 1, Ret_Mag 0.5, Alem 2. Duration-only effects: Calc 0.25, Ret_Dur 0.35. Magnitude-only effects: Calc 0.3. Round half up, minimum 1.
- Calcinator and Retort together on a positive effect use `1 + 1.4×Calc + 0.5×Ret` for magnitude.
- Apparatus strength: Novice 0.1, Apprentice 0.25, Journeyman 0.5, Expert 0.75, Master 1.0.
- Bugs to avoid: a Novice Alembic can **raise** negative effects by up to 15%; an Alembic raises poisons; the weight of the first potion of a type sets the weight of every later potion of that type.

### 2.10 Detection: the reference stealth model

From the Construction Set wiki. NPCs and creatures in high processing run detection against every actor in range **every 0.3 s**, so the cost is O(n²) across actors [VERIFIED https://cs.uesp.net/wiki/Category:Detection]:

```
if distance > MaxDetectionRange and target has not attacked detector:  Detection = 0
elif target invisible or Chameleon 100%:                               Detection = −100
else Detection = fSneakBaseValue
     [+ fSneakTargetAttackBonus, one frame, first time target attacks detector]
     [+ SoundBonus, if target not swimming]
     [+ fSneakSleepBonus, if detector sleeping]
     [+ SightBonus, if detector awake and has LOS]
     +  SkillBonus
SoundBonus = max(0, fSneakSoundsMult × DistanceFactor × ( [MovementFactor if moving and tgt Sneak < Expert]
                                                          + [fSneakTargetInCombatBonus if tgt in combat] )
                    [× fSneakSoundLosMult if no LOS])
MovementFactor = (fSneakBootWeightBase + [fSneakBootWeightMult × bootWeight if tgt Sneak < Journeyman])
                 [× fSneakRunningMult if running]
SightBonus = max(0, fSneakLightMult × DistanceFactor × LightFactor × ChameleonFactor × BlindnessFactor
                    [× fSneakSwimmingLightMult if swimming])
LightFactor = fDetectionSneakLightMod + lightOnTarget(0..100) [× fDetectionNightEyeBonus if detector has Night-Eye]
ChameleonFactor = clamp(1 − Chameleon/100, 0, 1);  BlindnessFactor = clamp(1 − Blind/100, 0, 1)
SkillBonus = fSneakSkillMult × (detectorSneak × DistanceFactor − [targetSneak if sneaking and not in combat])
DistanceFactor = 1 − distance / MaxDetectionRange
MaxDetectionRange = fSneakMaxDistance [× fSneakExteriorDistanceMult if exterior]
```

Default values, each read from its own GMST page on cs.uesp.net [VERIFIED https://cs.uesp.net/wiki/FSneakLightMult and sibling GMST pages]:

| GMST | Default | GMST | Default |
|---|---|---|---|
| fSneakBaseValue | −25 | fSneakLightMult | 1.4 |
| fSneakTargetAttackBonus | 100 | fSneakSwimmingLightMult | 0.5 |
| fSneakSoundsMult | 1.6 | fDetectionSneakLightMod | −5 |
| fSneakSleepBonus | −10 | fDetectionNightEyeBonus | 3.0 |
| fSneakTargetInCombatBonus | 25 | fSneakSkillMult | 0.5 |
| fSneakSoundLosMult | 1.0 | fSneakMaxDistance | 1500 units (interior) |
| fSneakBootWeightBase | 14 | fSneakExteriorDistanceMult | 2.0 |
| fSneakBootWeightMult | 1.0 | fSneakUnseenMin | 0 |
| fSneakRunningMult | 1.3 | fSneakNoticedMin / fSneakLostMin | −20 / −20 |
| | | iAICombatMinDetection | −50 |

**Thresholds (hysteresis):** detection above 0 means seen. Above −20 means "noticed": the NPC is suspicious and says so. A hostile that is searching loses you only below −20. One already in combat loses you only below −50 [VERIFIED https://cs.uesp.net/wiki/Category:Detection]. Sound is **not** blocked by walls, which is a known flaw.

**Worked example** (interior, 500 units apart, so DistanceFactor = 0.667). The target walks while sneaking, has Sneak 50 (below Journeyman, so boots count), wears 10-weight boots and stands in light 30. The detector has Sneak 40 and has line of sight.
- Sound = 1.6 × 0.667 × (14 + 10) = 25.6
- Sight = 1.4 × 0.667 × (−5 + 30) = 23.3
- Skill = 0.5 × (40 × 0.667 − 50) = −11.7
- Total = −25 + 25.6 + 23.3 − 11.7 = **+12.2, seen**

The same target barefoot in light 5: Sound = 14.9, Sight = 0, total = **−21.8**, which is below the −20 noticed threshold, so fully hidden.

Sneak mastery perks [VERIFIED https://en.uesp.net/wiki/Oblivion:Sneak]:
- Novice: unseen attacks deal **×4** with one-handed melee and H2H and **×2** with ranged.
- Apprentice and above: **×6 / ×3**. Two-handed weapons get **×1**. Spells never get a sneak bonus.
- Journeyman: boot weight is ignored.
- Expert: movement and running are ignored.
- Master: unseen attacks ignore armor.
- XP is 0.75 per second moving undetected near an actor, and 2 per pickpocket.

### 2.11 Security (lockpicking minigame)

There are 5 tumblers. Skill slows tumbler speed. When a pick breaks, Novice drops all set tumblers, Apprentice keeps 1, Journeyman 2, Expert 3 and Master keeps all. XP is 1.5 per tumbler set and 1.5 per Auto-Attempt [VERIFIED https://en.uesp.net/wiki/Oblivion:Security]. In the Remaster, the Expert perk keeps all but one tumbler.

### 2.12 What Oblivion teaches

1. **Detection as a continuous score with hysteresis thresholds** (0 / −20 / −20 / −50) plus separate sight, sound and skill channels is the best-documented real-time stealth model available. Port it.
2. Power-law spell cost (`Magnitude^1.28`) is the right curve. It makes stacking magnitude superlinearly expensive and rewards duration and area design.
3. Fixed mastery perks at 25/50/75/100 turn a number into identity.
4. **Player-level enemy scaling plus non-combat leveling is a trap.** The Remaster's patch for it was XP from all skills plus free attribute allocation.
5. Sneak and power multipliers must use `max()`, not product.

---
## 3. Baldur's Gate 1, 2 and 3

### 3.1 BG1 and BG2 (1998, 2000)

These ran on the Infinity Engine with AD&D 2nd Edition rules. Combat was real-time with pause, and attacks resolved against THAC0 and descending AC. Abilities ran 3–18, with exceptional Strength 18/01–18/00. Parties had six members. Resting could be interrupted by spawns, and BG2 had dual-class and multiclass rules [RECALLED]. The useful lesson for PALIMPSEST is **real-time with tactical pause**, which made a party-tactics ruleset playable in real time. Barony and PALIMPSEST co-op cannot pause, so the equivalent is a short **bullet-time "Marginal Pause"** (see §8).

### 3.2 BG3 ability scores and leveling

- 6 abilities. **Modifier = ⌊(score − 10)/2⌋**, capped at +10 [VERIFIED https://bg3.wiki/wiki/Ability].
- Point-buy: 27 points. Every point up to 13 costs 1; 14 and 15 cost 2 each. Then a +2 and a +1 are added to any abilities, so the starting maximum is 17 [VERIFIED https://bg3.wiki/wiki/Character_creation].
- The 8 minimum and 15 maximum before bonuses are [VERIFIED https://bg3.wiki/wiki/Ability].
- Proficiency bonus is +2, becoming +3 at character level 5 and +4 at level 9 [VERIFIED https://bg3.wiki/wiki/Proficiency_bonus]. The level cap is 12 [VERIFIED https://bg3.wiki/wiki/Multiclassing].
- XP table, cumulative [VERIFIED https://bg3.wiki/wiki/Experience]:

| Level | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Cumulative XP | 0 | 300 | 900 | 2,700 | 6,500 | 13,000 | 21,000 | 30,000 | 42,000 | 56,000 | 76,000 | 100,000 |

- Multiclassing: every level-up picks any class. Subclass choice comes at class level 1, 2 or 3. Only the class taken at level 1 grants its full skill proficiencies. Withers can respec a character for a fee [VERIFIED https://bg3.wiki/wiki/Multiclassing].

### 3.3 Dice and advantage

- Advantage means roll 2d20 and keep the higher; disadvantage means keep the lower. Neither stacks. Having both cancels to a single die. The mean moves by ±3.325, to 13.825 or 7.175 [VERIFIED https://bg3.wiki/wiki/Advantage].
- Common sources of advantage: attacking a target that is Prone, Restrained, Blinded or Off-Balance; attacking while Hidden or Invisible; attacking from darkness with Darkvision against a target without it. Disadvantage: ranged attack while Threatened (enemy adjacent), or a target within 3 m [VERIFIED https://bg3.wiki/wiki/Advantage].
- A natural 1 auto-fails and a natural 20 auto-succeeds on attack rolls and ability checks, but not on most saves. Death saves are DC 10 [VERIFIED https://bg3.wiki/wiki/Dice_rolls].
- **Karmic Dice**, on by default, reshapes the damage distribution and forces occasional successes. The chance of a forced success rises with each failure. It is a hidden pity system on d20 rolls [VERIFIED https://bg3.wiki/wiki/Dice_rolls].

### 3.4 Verticality: high ground

[VERIFIED https://bg3.wiki/wiki/High_ground]

- Attacker **≥ 2.5 m above** the target: **+2** to hit. 2.5 m below: **−2**. This applies to all attack rolls, including melee. The Sharpshooter feat removes the penalty for ranged attacks.
- **Range extension**: most ranged abilities add the elevation difference to their horizontal range. An 18 m bow shooting 9 m down reaches 27 m horizontally. Range is checked inside a vertical cylinder. Normal effective range, beyond which the shot has disadvantage, uses true 3D distance and is **not** extended.

### 3.5 Surfaces and elemental interactions

Table excerpt [VERIFIED https://bg3.wiki/wiki/Surfaces]:

| Surface | Effect | Transforms |
|---|---|---|
| Water | Wet | Lightning → Electrified Water; Cold → Ice; Fire → Steam cloud; dipping washes off fire or poison |
| Electrified Water | 1d4 lightning per turn | |
| Ice | Prone, difficult terrain | Fire → Water |
| Grease | Prone, difficult terrain | |
| Oil, Mysterious Liquid, Unstable Blood | | Fire damage → explodes into a Fire surface |
| Alcohol | | Fire → Fire surface; Cold → Ice |
| Blood, Mind Flayer Blood | | Cold → Ice |
| Fire | Burning 1d4 per turn; weapons can be dipped (+1d4 fire) | |
| Hellfire | 6d6 fire per turn | |
| Lava | 10d6 fire per turn, difficult terrain | |
| Smokepowder | | Fire → explodes 2d6 fire + 2d6 force |
| Acid | AC −2 | |
| Web | Enwebbed, difficult terrain | |
| Spikes | 2d4 piercing per 1.5 m moved | |
| Plant Growth | Movement ×¼ | |

- Difficult terrain doubles movement cost [VERIFIED https://bg3.wiki/wiki/Surfaces].
- Design point: surfaces are a **small closed reaction graph**: substances × damage-type triggers → new substance or explosion. It is cheap to simulate and reads instantly.

### 3.6 Stealth, sight cones and witnesses

[VERIFIED https://bg3.wiki/wiki/Stealth]

- **Hide** inside a non-ally's line of sight:
  - In a Clear area or within melee range: spotted immediately.
  - **Lightly Obscured**: a Stealth check against the observer's **Passive Perception** every turn, or every 6 s in real time.
  - **Heavily Obscured**: no check needed.
  - Darkvision shifts each level one step toward Clear. Many creatures have a hidden 2 m darkvision.
  - A natural 1 on Stealth means you are spotted.
- Outside every line of sight you are never checked.
- **Sight cones** are rendered for the player while holding a key (Left Shift). Distractions such as Minor Illusion or Meow make observers turn or walk.
- **Stealth attack**: after attacking from outside all sight cones, the attacker makes a Stealth check **with disadvantage**, any advantage cancels it, against the Perception of the target and any witnesses. On success the target searches with advantage for 5 turns or 30 s, and a "torch" marker shows where the shot came from. On failure, combat starts.
- **Greater Invisibility** upkeep: each action needs a Stealth check at DC `15 + n`, where n is the number of previous successes. In Honour mode it is `15 + n²`.

### 3.7 Rest economy

[VERIFIED https://bg3.wiki/wiki/Long_rest]

- A **long rest** at camp costs **40 camp supplies** (80 on Tactician and Honour, up to 120 with custom settings). It restores all HP, spell slots, resources and both short-rest charges.
- With too few supplies you take a **partial rest**: HP and resources up to ½ of max, rounded down, and cooldowns refresh.
- Short rests: 2 per long rest.

### 3.8 Illithid powers

A separate skill tree is unlocked by **consuming mind-flayer tadpoles**, introduced through a dream visitor who urges you to "embrace your potential." Companions differ in willingness and some must be persuaded with checks at DC 12–20. The only mechanical cost is one Wisdom save, at the Astral-Touched Tadpole, to avoid Partial Ceremorphosis. The narrative cost is companion disapproval and a darker story path [VERIFIED https://bg3.wiki/wiki/Illithid_powers].

### 3.9 State in 2025 to 2026

- **Patch 8 was released April 15, 2025 as "the final major patch."** It added 12 subclasses (Arcane Archer, Bladesinging, Circle of the Stars, College of Glamour, Death Domain, Giant, Hexblade, Oath of the Crown, Shadow Magic, Swarmkeeper, Swashbuckler, Drunken Master), new spells (Booming Blade, Toll the Dead and others), Photo Mode and cross-play [VERIFIED https://bg3.wiki/wiki/Patch_8].
- The wiki lists **Hotfix 36** as the latest hotfix after Patch 8 [VERIFIED https://bg3.wiki/wiki/Patch_8].

### 3.10 What BG3 teaches

1. Show the stealth model: render sight cones on demand and mark the last known position.
2. Give elevation one simple, legible bonus (+2) plus a range extension.
3. Make the substance reaction graph small enough to memorize.
4. A tempting forbidden power tree with mostly narrative cost works as a player-choice moment.
5. Rest costs a consumable, and running short gives half a rest rather than none.

---

## 4. Borderlands 1 to 4

### 4.1 Timeline and state of Borderlands 4 (as of October 2026)

- BL4 released **September 12, 2025** on PS5, Xbox Series X|S and Windows. It is set on the planet Kairos and has seamless travel between most zones [VERIFIED https://borderlands.fandom.com/wiki/Borderlands_4]. The Switch 2 version was delayed, then "paused" indefinitely, as announced February 3, 2026 [VERIFIED same].
- Post-launch [VERIFIED https://borderlands.fandom.com/wiki/Borderlands_4; https://support.borderlands.com Version 1.10 notes]:
  - Bloomreaper raid boss, December 11, 2025.
  - Level cap 50 → **60** with Story Pack 1 and the C4SH Vault Hunter on March 26, 2026.
  - Shared Progression toggle, May 28, 2026.
  - Cross-platform saves, June 25, 2026.
  - Level cap → **70** with Version 1.10, Loveless and Story Pack 2, on **September 10, 2026**.
  - Weekly updates continue as of October 8, 2026.
- Each Vault Hunter has 3 switchable Action Skills with "augments" and "capstones", plus a passive "trait" [VERIFIED https://borderlands.fandom.com/wiki/Borderlands_4].
- Gun-count marketing has grown with each game: BL1 "17.75 million", BL2 "87 bazillion" [RECALLED], BL4 "30 billion combinations" [SEARCH https://multiplayer.it/notizie/borderlands-4-ha-30-miliardi-di-combinazioni-per-le-armi-vi-bastano.html].

### 4.2 Procedural gun generation in BL4: parts, traits and licensed parts

[VERIFIED https://borderlands.fandom.com/wiki/Licensed_Parts]

- Five part types:
  - **Traits**: the manufacturer's default unique effect.
  - **Generic** parts: ordnance and gear only.
  - **Licensed Parts**: another manufacturer's trait grafted onto any gun.
  - **Underbarrel Attachments**: alt-fire.
  - **Legendary parts**.
- **Part slots by rarity, guns only**: Common 0, Uncommon 1, Rare 2, Epic 3, Legendary 2 + 1 legendary part. A displayed 4th slot is the default trait. A higher rarity sets the maximum number of parts but does not guarantee every slot is filled.
- Underbarrel conflict rule: underbarrels cannot mix manufacturers, except the Atlas, Daedalus and Maliwan licensed variants.
- Manufacturer identity, 8 gun makers plus 3 parts-only makers:

| Manufacturer | Gun trait (identity) | As a licensed part on another gun |
|---|---|---|
| Daedalus | Always has underbarrel; secondary fire uses a different ammo type; mostly burst | Multi-Loader alt-fire using the other ammo type |
| Jakobs | Always semi-auto; **crits ricochet** to nearby enemies | Accessory: crits ricochet |
| Maliwan | Always elemental; underbarrel **switches between two elements**; small splash | Element-switch underbarrel |
| Order | Always semi-auto; **charge to fire multiple rounds** | none |
| Ripper | Full-auto after a **charge-up** | Magazine: charge before full-auto (overrides Order charge) |
| Tediore | **Reload by throwing the gun**. Variants: Combo (shoot it to boost the blast), Homing, Javelin, Legs (it runs at enemies), MIRV (4 grenades), Replicator (throws 2), Shooting (it fires on its own) | Accessory: throws a gun on reload, plus variants |
| Torgue | Always semi-auto; **gyrojets**: impact-explosive, or sticky ones that detonate on reload or mode switch, +X% per stack | Impact or Sticky magazine |
| Vladof | Always has underbarrel; **always full-auto** | none. Full-auto otherwise only via Ripper licensed parts or specific models |
| Atlas (parts only) | none | Tracker dart or grenade underbarrel; projectiles **home** on tagged enemies for 10 s or 8 s |
| CoV (parts only) | none | **Unlimited magazine that heats up and breaks**; fixing it is the reload |
| Hyperion (parts only) | none | Grip: accuracy rises with sustained fire. Gun Shield variants: Amp, Absorb (ammo), Ricochet |

- The PALIMPSEST reading: **a licensed part is literally a genre graft.** Borderlands already proves the "one gun body, foreign mechanic grafted on" grammar at scale.

### 4.3 Rarity

- BL4 tiers: Common (white), Uncommon (green), Rare (blue), Epic (purple), Legendary (orange), **Pearlescent** [VERIFIED https://borderlands.fandom.com/wiki/Rarity].
- **Phosphene** is an extremely rare **shiny variant** of a legendary or pearlescent weapon. It has no gameplay difference. Picking one up unlocks the skin for that whole model and completes a profile-wide challenge [VERIFIED https://borderlands.fandom.com/wiki/Phosphene]. This is the shiny-Pokémon pattern: a pure-collection jackpot layered over power loot.
- BL1 rarity was a hidden integer: 0–4 Common, 5–10 Uncommon, 11–15 Rare, 16–49 Epic, 50–100 three legendary shades, 101+ Pearlescent [VERIFIED https://borderlands.fandom.com/wiki/Rarity]. BL2 added E-tech, Seraph and Effervescent. BL3 added Anointments. Tiny Tina's Wonderlands added Enchantments.

### 4.4 Elemental matrix (BL4)

Effectiveness depends on health layer and **widens with difficulty** [VERIFIED https://borderlands.fandom.com/wiki/Elemental_Damage]:

| Element | Easy: Flesh / Shield / Armor | Normal | Hard |
|---|---|---|---|
| Kinetic | 100 / 100 / 100 | 100 / 100 / 100 | 100 / 100 / 100 |
| Incendiary | 120 / 90 / 90 | 150 / 75 / 75 | 175 / 50 / 50 |
| Shock | 90 / 120 / 90 | 75 / 150 / 75 | 50 / 175 / 50 |
| Corrosive | 90 / 90 / 120 | 75 / 75 / 150 | 50 / 50 / 175 |
| Radiation | 100 / 110 / 90 | 100 / 125 / 75 | 100 / 150 / 50 |
| Cryo | 100 / 90 / 110 | 100 / 75 / 125 | 100 / 50 / 150 |

Pattern: each of the three primary elements has one strong layer and two weak ones. The two secondary elements, Radiation and Cryo, are neutral on Flesh, one step better on one layer and one step worse on the other. Kinetic is always ×1.

### 4.5 Gear layers new in BL4

- **Enhancements** replace artifacts. They drop from player level 15. Their bonuses work only on guns carrying the matching manufacturer's licensed parts. Bonuses by rarity, weapon-part / stat: Common 1/0, Uncommon 1/1, Rare 1/2, Epic 1/3, Legendary 2/2. Weapon-part bonuses are fixed per model, and stat bonuses roll from a preset list [VERIFIED https://borderlands.fandom.com/wiki/Enhancement].
- **Firmware** is a set-bonus tag that can roll on Class Mods, Enhancements, Ordnance, Repkits and Shields from **player level 25**. Each equipped item with the same Firmware unlocks the next tier, **up to 3**; a 4th or 5th copy does nothing. Some sets drop only in specific places (Bruiser in Hadron Abyss) or at specific difficulties (Skillcraft at UVHM 6+, Active Fire at UVHM 7). After the story, a **Firmware Transfer Machine** moves Firmware between items of the same type once, for cash plus Eridium [VERIFIED https://borderlands.fandom.com/wiki/Firmware].
  - Example, God Killer: +10% damage to Badass enemies; −10% damage taken from Badasses; a Badass kill gives +20% damage for 60 s.
- **Repkits** are a healing item with limited charges and per-charge cooldowns. Each of the 8 manufacturers makes one common and some legendaries. They have up to 2 part slots by rarity, and the Order trait gives 2 charges [VERIFIED https://borderlands.fandom.com/wiki/Repkit].
- **Ordnance** is one shared slot for grenades and heavy weapons, with recharging ammo and cooldowns. Each manufacturer has one grenade identity: Daedalus frag that cooks, Jakobs throwing knives that crit, Maliwan plasma that sticks or self-propels, Order seeker, Ripper teleport-grenade [VERIFIED https://borderlands.fandom.com/wiki/Ordnance].

### 4.6 Ultimate Vault Hunter Mode (BL4)

UVHM has 7 ranks, each unlocked by beating set bosses or missions on the previous rank. Each rank raises the chance of licensed parts and Firmware, dedicated-drop chance and world-drop volume [VERIFIED https://borderlands.fandom.com/wiki/Ultimate_Vault_Hunter_Mode_(Borderlands_4)]:

| UVHM rank | 1 | 2 | 3 | 4 | 5 | 6 | 7 |
|---|---|---|---|---|---|---|---|
| Enemy health | +150% | +200% | +250% | +300% | +350% | +400% | +450% |
| Damage to player | +12% | +14% | +16% | +18% | +20% | +22% | +24% |
| Cash and Eridium | +20% | +25% | +30% | +35% | +40% | +45% | +50% |
| XP | +10% | +12.5% | +15% | +17.5% | +20% | +22.5% | +25% |

The page's column headers are ambiguous. The mapping above is my best reading of the wikitext.

### 4.7 Drop rates and the transparency problem

- Gearbox does not publish drop rates. A fan tracked more than 3,000 boss kills and estimated about **4% for world legendaries and about 5% per dedicated item** [SEARCH https://www.gamepro.de/artikel/borderlands-4-niedrig-drop-raten-legendaere-waffen,3440999.html].
- Patch history from official notes [VERIFIED https://support.borderlands.com, notes of 2026-01-15 and 2026-03-05]:
  - Holiday "Season's Lootings" ran up to ×5 legendary rates.
  - January 15, 2026: "permanently increasing the dedicated drop rates on bosses" and "a permanent increase to Legendary drop rate chance."
  - March 5, 2026: Pearlescent "starting drop chance remains the same" but the scaling by UVHM rank was retuned. All base-game, Invincible and Bounty Pack bosses now share **one standard dedicated-drop rate**.
- Lesson: hidden rates plus low perceived yield caused months of community pressure and repeated upward patches. Publish rates.

### 4.8 Slot machines, vending machines and Eridium

- **BL2 slot machine**, an exact integer weight table. Each pull draws 0..32,767 (15-bit) [VERIFIED https://borderlands.fandom.com/wiki/Slot_machine]:

| Roll range | Outcome | Weight /32,768 | % |
|---|---|---|---|
| 0–6,711 | White weapon | 6,712 | 20.48 |
| 6,712–11,745 | Green weapon | 5,034 | 15.36 |
| 11,746–12,249 | Blue weapon | 504 | 1.54 |
| 12,250–12,299 | Purple weapon | 50 | 0.15 |
| 12,300–12,305 | Orange weapon | 6 | 0.02 |
| 12,306–13,143 | 1× Eridium | 838 | 2.56 |
| 13,144–13,395 | 2× Eridium | 252 | 0.77 |
| 13,396–13,470 | 3× Eridium | 75 | 0.23 |
| 13,471–21,860 | Money | 8,390 | 25.60 |
| 21,861–24,377 | **Live grenade** | 2,517 | 7.68 |
| 24,378–26,055 | Skin | 1,678 | 5.12 |
| 26,056–32,767 | Nothing | 6,712 | 20.48 |

  - Payouts are per player and not split, so group play multiplies value. Price scales with loot level. The live-grenade outcome makes the gamble physically dangerous and funny.
- BL3 split slot machines by payout theme. "Loot Boxer" paid weapons by symbol triple, for example 3 bullets for a legendary [VERIFIED https://borderlands.fandom.com/wiki/Slot_machine].
- BL4: **Maurice's Black Market** vending machine relocates on the weekly reset. It sells 2 legendaries, personalized per player, and has a 30-minute cooldown after each purchase [SEARCH https://insider-gaming.com/borderlands-4-maurices-black-market-location/]. **Moxxi's Big Encore** machine resummons a harder boss for Eridium, with better drops [SEARCH https://kyberscorner.com/borderlands-4-weekly-reset-nov-6-2025/].

### 4.9 Vault Cards: a non-expiring battle pass

BL4 Bounty Packs ship a **Vault Card**: "more loot and XP via Daily and Weekly Challenges, and can be leveled up at your own pace (**no time limits or expiring rewards**)… Vault Card progression works just like leveling up: it's all driven by how much XP you're gaining." Vault Card 5 has 24 cosmetics plus 4 pieces of **rerollable gear** [VERIFIED https://support.borderlands.com Version 1.10 notes, 2026-09-10]. This is the closest AAA precedent to PALIMPSEST's non-expiring Codex.

### 4.10 What Borderlands teaches

1. Rarity sets the **number of part slots**, and manufacturer sets **behavior**.
2. Cross-manufacturer licensed parts multiply build space combinatorially while every part stays legible as one sentence.
3. Elemental matrices should be simple (strong ×1.5 / weak ×0.75) and widen with difficulty.
4. Set bonuses should be capped at 3 stacks.
5. Pure-cosmetic shiny variants are a jackpot that does not inflate power.
6. Hidden drop rates erode trust.

---

## 5. Minecraft

### 5.1 Scale and time

- 1 block = 1 m [RECALLED; universal convention].
- **Chunk = 16×16 columns.** Overworld height is **384** (y −64 to 319), so 98,304 blocks per chunk. A chunk is split into 16-tall **sections** (subchunks). Nether height is 256 on Java and 128 on Bedrock [VERIFIED https://minecraft.wiki/w/Chunk].
- **20 game ticks per second**, so 1 tick = 50 ms. A day is 24,000 ticks (20 min). The community "redstone tick" is 2 game ticks. Java picks `random_tick_speed` = 3 random blocks per section per tick; Bedrock defaults to 1 [VERIFIED https://minecraft.wiki/w/Tick].

### 5.2 Light

[VERIFIED https://minecraft.wiki/w/Light]

- Integer levels 0–15. There are two channels, **sky** and **block**. "Client light" = `max(sky, block)`.
- Block light flood-fills and loses **1 per block of taxicab distance**. A torch (14) gives 13 on adjacent floor and 12 diagonally.
- Sky light is 15 under open sky. It does **not** drop going straight down through transparent blocks. It loses 1 per step sideways or upward. It does not fall at night; time of day is applied through an internal sky-light value instead.
- Opaque blocks stop light. Some shapes are directionally opaque: slabs, stairs, farmland and dirt paths. Java light-filtering blocks such as water and leaves cut sky light by 1. Bedrock filter values differ per block.

### 5.3 Spawning

[VERIFIED https://minecraft.wiki/w/Mob_spawning]

- Overworld monsters need **block light 0** and internal sky light ≤ 7. The light check passes only if light ≤ a random value in 0–7. Nether monsters need block light ≤ 7, or ≤ 11 depending on mob. The End needs block light 0.
- Spawning happens in chunks with a player within 128 blocks of the chunk center, inside a 128-block sphere. Monsters spawn in packs offset ±5 on X and Z with a triangular distribution, and about 85% land within 5 blocks of the pack center.
- Hostile cap is **70** per player. Global cap = `mobCap × chunks / 289` (a 17×17 chunk window).
- Despawn: instant beyond 128 blocks. After 30 s with no player inside 32 blocks (10 s in light above 12), each tick has a **1/800** despawn chance, which halves the population every 27.75 s.

### 5.4 Redstone

Signal strength runs 1–15 and loses 1 per block of dust, so dust carries 15 blocks. Strength into a component does not change its behavior (a piston fires the same at 1 or 15). Repeaters re-amplify. Java processes redstone on 2-tick phases [VERIFIED https://minecraft.wiki/w/Redstone_power].

### 5.5 Crafting

The player has a 2×2 grid; a crafting table gives 3×3. Recipes are **shaped**, which is position-sensitive and can be mirrored, or **shapeless** [VERIFIED https://minecraft.wiki/w/Crafting].

### 5.6 Enchanting (exact)

[VERIFIED https://minecraft.wiki/w/Enchanting_mechanics]

- Up to **15** bookshelves count. Each must sit 2 blocks from the table horizontally, at table height or one above, with nothing blocking.
- `xpBase = 1 + randInt(7) + ⌊min(15, shelves)/2⌋ + randInt(min(15, shelves))`. randInt(n) is uniform over 0..n inclusive.
- Slot costs: top `⌊max(1, xpBase/3)⌋`; middle `⌊2·xpBase/3⌋ + 1`; bottom `⌊max(xpBase, 2·shelves)⌋`. With 15 shelves the bottom slot is **always 30**.
- Hidden power: `eCost = round((xpCost + 1 + randInt(⌊ench/4⌋) + randInt(⌊ench/4⌋)) × (1 + randTri(0.15)))`, where randTri(n) = (rand + rand − 1) × n.
- Selection loop:
  1. Pick a weighted candidate.
  2. Remove it and anything incompatible with it.
  3. Continue with probability `(eCost + 1)/50`.
  4. Halve eCost and repeat.
- A book that ends up with several enchantments loses one at random.
- The offered rolls come from a per-player **XpSeed** that changes only after you actually enchant something. Swapping items or reopening the table cannot reroll the offer.
- Slots cost 1, 2 or 3 lapis and the same number of levels [RECALLED].

### 5.7 Anvil

An operation costing more than **39 levels** is refused ("Too Expensive!"). The prior-work penalty is `2^c − 1`, where c is the number of earlier anvil uses on the item. A grindstone or grid repair clears it [VERIFIED https://minecraft.wiki/w/Anvil_mechanics].

### 5.8 Brewing

[VERIFIED https://minecraft.wiki/w/Brewing]

- One blaze powder fuels **20** brews.
- Nether wart turns water into Awkward Potion, the base. Then an effect ingredient sets the effect.
- Modifiers form a separate axis:
  - **Redstone** extends duration.
  - **Glowstone** amplifies to level II with shorter duration. There is no trade-off on instant effects.
  - **Fermented spider eye** corrupts or inverts the effect.
  - **Gunpowder** makes a splash potion with the same duration.
  - **Dragon's breath** turns splash into **lingering**, a cloud at ¼ duration.
- Lingering potions craft tipped arrows.

### 5.9 Experience levels

- XP to next level: `2L + 7` for L = 0–15; `5L − 38` for 16–30; `9L − 158` for 31+.
- Total XP: `L² + 6L` up to 16; `2.5L² − 40.5L + 360` for 17–31; `4.5L² − 162.5L + 2220` for 32+.
- On death you drop `7 × level` XP, capped at 100.

[VERIFIED https://minecraft.wiki/w/Experience]

### 5.10 2025 to 2026 updates

Since 2024 Mojang ships quarterly "game drops". In 2026 versions switched to **year-based numbering (26.x)** [VERIFIED https://minecraft.wiki/w/Game_Drop]:

| Drop | Date | Java / Bedrock | Headline |
|---|---|---|---|
| Spring to Life | 2025-03-25 | 1.21.5 / 1.21.70 | Leaf litter, wildflowers, warm and cold farm-animal variants |
| Chase the Skies | 2025-06-17 | 1.21.6 / 1.21.90 | Happy Ghast (ridable), locator bar, craftable saddle, **Vibrant Visuals** (Bedrock renderer) |
| The Copper Age | 2025-09-30 | 1.21.9 / 1.21.111 | Copper Golem (sorts chests), copper tools and armor, shelves |
| Mounts of Mayhem | 2025-12-09 | 1.21.11 / 1.21.130 | Iron spear, nautilus mounts, zombie horsemen, netherite horse armor |
| Tiny Takeover | 2026-03-24 | 26.1 / 26.10 | New baby models, craftable name tags |
| Chaos Cubed | 2026-06-16 | 26.2 / 26.30 | Sulfur springs and caves, sulfur and cinnabar blocks, Sulfur Cube mob |
| Wilderness Bound | 2026-09-15 | 26.3 / 26.50 | Dappled forest, poplar wood, abandoned camps, wool and concrete stairs, better maps |
| Fourth drop 2026 | upcoming | 26.4 / 26.70 | Ice caves, Freezing effect, snapshots from 2026-10-06 |

Vibrant Visuals brings colored point lights to light-emitting blocks under an experiment [VERIFIED https://minecraft.wiki/w/Light].

### 5.11 What Minecraft teaches

1. Integer light 0–15 with −1 per taxicab step is cheap, deterministic, legible and directly drives spawning.
2. Hard-gate monster spawns on darkness so that light becomes a buildable defense.
3. Enchanting with a hidden power, a visible cost and a seed that changes only on commit stops save-scum rerolls.
4. An exponential repair or re-forge penalty with a hard cap ends infinite optimization.
5. Potion modifiers form an orthogonal axis: duration, potency, invert, delivery.

---

## 6. Destiny 2

### 6.1 Timeline

- **The Edge of Fate**, July 15, 2025. It began Year 8, the "Year of Prophecy", and reset player Power to 10 [VERIFIED https://en.wikipedia.org/wiki/Destiny_2:_The_Edge_of_Fate].
- **Ash & Iron** major update, September 9, 2025 [VERIFIED same].
- **Renegades**, a Star Wars-themed expansion, **December 2, 2025**, which began Season "Lawless" [VERIFIED https://destiny.fandom.com/wiki/Renegades].
- **May 21, 2026**: Bungie announced that **Monument of Triumph** (Update 9.7.0, **June 9, 2026**) would be **the final content update for Destiny 2**. The planned Year 9 expansions *Shattered Cycle* and *The Alchemist* were cancelled. The decision followed Sony's **US$765 million impairment** on Bungie, partly due to *Marathon*'s poor performance [VERIFIED https://en.wikipedia.org/wiki/Destiny_2:_The_Edge_of_Fate]. The 9.7.0 notes say: "this is our final major patch… We'll still have a few maintenance patches and smaller hotfixes" [VERIFIED https://destiny.fandom.com/wiki/Destiny_2_Update_9.7.0].
- So as of October 2026, **Destiny 2 is in maintenance mode with no live-service content.** Renegades is its final expansion [VERIFIED https://destiny.fandom.com/wiki/Renegades].

### 6.2 Power and the Portal

- At Edge of Fate launch, Power was reset to 10 with a cap of 200 (equal to the old "Legacy" 2300). **Seasonal Bonus Power** gear raised it to 450, and Ash & Iron raised that to **550**. Bungie reversed its plan to make Seasonal Bonus Power expire after player feedback [VERIFIED https://en.wikipedia.org/wiki/Destiny_2:_The_Edge_of_Fate].
- The Artifact no longer gives Power. It gives bonuses per piece of equipped "New Gear" [VERIFIED same].
- The **Portal** replaced playlist navigation. You pick an activity class, add modifiers to raise difficulty, and earn better rewards [VERIFIED same].
- Monument of Triumph made the rule explicit: **"Difficulty = Grade = Reward Tier"**. Grade B gives Tier 1, B+ Tier 2, A Tier 3, A+ Tier 4, S and S+ Tier 5. Grade targets no longer scale with Power. The matchmade difficulties need Power 300 [VERIFIED https://destiny.fandom.com/wiki/Destiny_2_Update_9.7.0].
  - Example: Grade 0 gives 1× Tier 3; Grade 3 gives 2× Tier 5; Grade 5 gives 3× Tier 5.

### 6.3 Weapons: perk columns, gear tiers, crafting and Deepsight

- A legendary weapon has an intrinsic frame, then columns for barrel or sight, magazine, two trait columns, an origin trait and a masterwork. Random rolls pick one perk per column [RECALLED].
- Edge of Fate **gear tiers** [VERIFIED https://shatteredvault.com/kb/weapon-optimization/weapon-tiers/]:

| Tier | Adds |
|---|---|
| 1 | Baseline legendary |
| 2 | Enhanced left and right trait perks |
| 3 | Enhanced weapon mods; natural drops can roll **2 options per trait column** |
| 4 | Enhanced barrels and magazines |
| 5 | Enhanced origin trait; **3 options per trait column**; Tier 5 cosmetics; extra masterwork stat bonus |

- The Tier 5 masterwork bonus is +5 to all stats, or "+tier to all stats" [SEARCH https://www.bungie.net/7/en/News/Article/dev_insights_weapon_gear_tier].
- **Crafting** (patterns from Deepsight "red border" drops, then forge and level the weapon) was introduced in 2022. "So far in Edge of Fate, no new legendary weapons are craftable" [VERIFIED https://shatteredvault.com/kb/weapon-optimization/weapon-tiers/].
- Monument of Triumph: crafted and Reshape-capable weapons "can now reach the power of Gear Tier 5". **Enhanced Weapon Mods unlock at Tier 3.** Raid and dungeon chests drop Deepsight weapons or Tier 5 armor, with weekly lockouts [VERIFIED https://destiny.fandom.com/wiki/Destiny_2_Update_9.7.0].

### 6.4 Armor 3.0

- Six stats replace the old six: Mobility → **Weapons**, Resilience → **Health**, Recovery → **Class**, Discipline → **Grenade**, Intellect → **Super**, Strength → **Melee**. **Each stat runs 0–200.** Points 1–100 scale the base benefit, and **points 101–200 unlock a second, new benefit**. Each armor piece rolls points in **only 3 stats**, set by its archetype: primary, secondary and one random tertiary. Sets give **2-piece and 4-piece bonuses** [VERIFIED https://en.wikipedia.org/wiki/Destiny_2:_The_Edge_of_Fate; https://shatteredvault.com/kb/armour-buildcrafting/].
- Stat effects [VERIFIED https://shatteredvault.com/kb/armour-buildcrafting/]:

| Stat | 1–100 | 101–200 |
|---|---|---|
| Health | 0–70 heal on Orb pickup; 0–10% flinch resistance | Shields recharge 0–25% faster; 0–20 extra shield vs combatants |
| Melee | Melee cooldown and energy gain | +0–30% melee damage |
| Grenade | Grenade cooldown and energy gain | +0–65% grenade damage |
| Super | Super energy from damage | +0–45% Super damage |
| Class | Class-ability cooldown | 0–40 HP overshield on cast (0–10 in PvP) |
| Weapons | Reload and handling; +0–15% Primary/Special and +0–10% Heavy damage vs minors and majors | Bonus ammo in bricks; +0–15% / 0–10% vs bosses and Guardians |

- **Armor tier stat totals**, base (masterworked) [VERIFIED https://shatteredvault.com/kb/armour-buildcrafting/]:

| Tier | Stat total |
|---|---|
| 1 | 52–57 (67–72) |
| 2 | 58–63 (73–78) |
| 3 | 64–69 (79–84) |
| 4 | 70–75 (85–90), extra mod energy |
| 5 | 75 (90), 11 energy, plus a **Tuning slot** |

  - Tuning options are "+5 to one stat / −5 to another" or "Balanced: +1 to the three lowest stats".
  - Monument of Triumph gave Exotic armor Tier 5 behavior, retroactively.
- Archetype names, 6 at launch [RECALLED]: Gunner (Weapons/Grenade), Brawler (Melee/Health), Specialist (Class/Weapons), Grenadier (Grenade/Super), Paragon (Super/Melee), Bulwark (Health/Class). Shattered Vault now says there are 12 archetypes. That is unverified, see Open questions.

### 6.5 Monetization

- From Edge of Fate, Season Passes became **Reward Passes**. There is one per expansion and one per major update, each with free and premium tracks. Ash & Iron added a **seasonal archive** at the Tower for claiming rewards from earlier passes with a material earned from the current pass. Players who never owned a past pass can buy it with Silver [VERIFIED https://en.wikipedia.org/wiki/Destiny_2:_The_Edge_of_Fate].
- Each Reward Pass costs **1,000 Silver** and has up to **110 ranks**, with no waiting for later acts to unlock [VERIFIED https://destiny.fandom.com/wiki/Renegades].
- **Eververse** sells cosmetics for Silver. Renegades launch examples: a 2,000-Silver set and a 2,500-Silver set [SEARCH https://thenerdstash.com/all-destiny-2-renegades-eververse-cosmetics/].
- Monument of Triumph added **Legendary Marks** earned from Triumphs, which buy free ornaments and weapon engrams from vendors [VERIFIED https://destiny.fandom.com/wiki/Destiny_2_Update_9.7.0].

### 6.6 What Destiny 2 teaches

1. **Gear tier, orthogonal to rarity,** is the right lever for difficulty rewards.
2. **Perk columns with more options at higher tiers** let players choose instead of reroll.
3. **Stats with a second band past 100** make every point matter.
4. Archetype armor rolls only 3 of 6 stats, so drops are distinct.
5. Rewards that do not expire win goodwill. Expiring Bonus Power was reversed within three months.
6. **A live-service game can end.** Design for graceful end-of-service from day one.

---
## 7. Cross-reference: best source for each PALIMPSEST system

| PALIMPSEST system (spec) | Best source to port | What exactly |
|---|---|---|
| Skill check template (04) | Morrowind | `(rate − opposed rate) × fatigueTerm` (§1.5–1.8) |
| Skill XP curve (04) | Oblivion | `0.25883 × L^1.5`, with affinity multipliers (§2.3) |
| Level-up allocation (04) | Oblivion Remastered | 12 virtues, ≤5 per stat, ≤3 stats, Luck costs 4 (§2.5) |
| Anti-grind XP (04) | Oblivion Remastered | XP proportional to magnitude; zero-cost actions give 0 (§2.5) |
| Melee/ranged damage and armor (05) | Morrowind + BL4 | Hyperbolic AR with a 25% floor; health-layer matrix (§1.6, §4.4) |
| Elevation (05, 09) | BG3 | ≥2.5 m gives +2 (scaled); range extension (§3.4) |
| Spell cost and gating (06) | Oblivion + Morrowind | `Mag^1.28`, Target ×1.5, cost bands → mastery (§2.7, §1.10) |
| Multi-effect compounding (06, 07) | Morrowind enchanting | `Σ C_i × (n − i + 1)` (§1.11) |
| Alchemy (08) | Morrowind + Oblivion + Minecraft | Apparatus roles, power-law magnitude, orthogonal modifiers (§1.9, §2.9, §5.8) |
| Stealth (09) | Oblivion + BG3 | Detection score with hysteresis; visible sight cones; stealth-attack check (§2.10, §3.6) |
| Identification (10) | Morrowind + Oblivion | Properties revealed by skill bands; experimentation reveals (§1.9, §2.9) |
| Loot generation (11) | Borderlands 4 + Destiny 2 | Rarity → slots; manufacturer traits; licensed parts; gear tier → perk options (§4.2, §6.3) |
| In-run gamble (11) | Borderlands 2 | Integer weight table including a dangerous outcome (§4.8) |
| Set bonuses (11) | BL4 Firmware + D2 Armor 3.0 | 3-stack cap; 2/4-piece sets (§4.5, §6.4) |
| Difficulty → reward (13, 11) | Destiny 2 Monument | Grade = Tier mapping (§6.2) |
| Light grid and spawning (03, 12) | Minecraft | 0–15, −1 per taxicab step; spawn at block light 0 (§5.2–5.3) |
| Enchant offers (07) | Minecraft | Per-player seed that advances only on commit (§5.6) |
| Re-forge cost (07) | Minecraft anvil | `2^c − 1` penalty with a hard cap (§5.7) |
| Surfaces and reactions (03, 08) | BG3 | Substance × trigger → substance or explosion (§3.5) |
| Rest economy (02) | BG3 | Supplies per rest; half rest when short (§3.7) |
| Battle pass (15) | BL4 Vault Card + D2 archive | Non-expiring, XP-driven, retro-claimable (§4.9, §6.5) |
| End of service (15, 16) | Destiny 2 (counter-example) | Plan an offline-complete end state (§6.1) |

---

## 8. Design implications for PALIMPSEST

Each recommendation is opinionated and concrete. Numbers here are **PALIMPSEST adaptations**, not source values. Units follow the brief: sub-attributes 1–100 with baseline 40; skills 0–100 with mastery at 25/50/75/100; probabilities in basis points (bp, 0–10,000); 60 Hz ticks; 1 block = 1 m.

### 8.1 Steal exactly

1. **The universal Condition term (from Morrowind fatigueTerm).**
   `cond_bp = 7500 + 5000 × stamina_cur / stamina_max`, so 7,500 bp at empty and 12,500 bp at full. It is uncapped above when stamina is fortified. Multiply it into **every** active skill rate: hit, evade, block, cast, sneak, spot, lockpick, appraise, haggle. Players learn once that tired means worse at everything.
2. **The universal opposed-check template.**
   - `rate = skill + govSub/5 + Fate/10 + flat_bonuses`, then `× cond_bp/10,000`.
   - `chance_bp = clamp((rate_actor − rate_opposed) × 100, 500, 9500)`.
   - Use the 500 and 9,500 clamps for all random checks. Disable the clamps only for checks the spec marks as deterministic. This is Morrowind's shape with the 10%/50% block clamps generalized.
3. **The Oblivion detection model, ported.**
   - Keep the structure and the defaults: base −25; sound mult 1.6; boot base 14 + boot weight; running ×1.3; light mult 1.4; light mod −5; skill mult 0.5; thresholds seen > 0, noticed > −20, lost < −20, combat lost < −50.
   - Distance: `DistanceFactor = 1 − d / MaxRange`, with **MaxRange 21 m indoors and 42 m outdoors**. Oblivion's 1,500 units at about 70 units per meter [RECALLED] is about 21.4 m.
   - Light: `lightOnTarget_0_100 = simLight_0_15 × 20 / 3`, read from the sim light grid.
   - **Fix the flaw**: sound is read from the sim **sound grid**, which walls attenuate, instead of straight-line distance.
   - Evaluate each detector–target pair every **18 ticks** (0.3 s), staggered across ticks by `(detectorId + targetId) mod 18`.
   - Add BG3's on-demand **sight cones** and the **last-known-position marker**.
4. **Sneak-attack multipliers use `max(sneak, power)`, never a product** (Oblivion). Unaware multipliers by Stealth mastery: Novice ×2.5 for short or one-handed weapons and ×1.5 ranged; 25+ ×3.5 / ×2; 50+ ×4.5 / ×2.5; two-handed ×1.5 flat. At 100, an unaware hit ignores armor.
5. **Oblivion Remastered allocation, scaled to 24 sub-attributes.**
   - Each level gives **12 Quill points**.
   - At most **+3 to any one sub-attribute**, and at most **4 different sub-attributes** per level.
   - **FORTUNE subs (Fate, Serendipity, Providence) cost 4 points each, at most +1 per level**, mirroring Luck.
   - Health gains from VIGOR/Toughness apply **retroactively**.
6. **Character XP = total skill XP gained**, as in Oblivion Remastered and Skyrim, plus discovery XP. Skill XP to next level is `0.26 × L^1.5 × affinity`, with affinity 0.6 for the Echo's signature skills and 1.0 otherwise. **XP is proportional to meaningful magnitude**: damage dealt to hostiles, base cost of spells, value traded, lock difficulty. Zero-cost or zero-effect actions give 0 XP.
7. **Spell cost (Oblivion power law plus Morrowind conventions).**
   - Per glyph: `cost = BaseCost/10 × max(1, Mag)^1.28 × max(1, Dur)^0.9 × (1 + 0.15 × Area) × Range`, with Range = 1.0 for self or touch and **1.5 for target**.
   - Multi-glyph spells **compound in Morrowind order**: sort glyphs by cost, descending, and multiply glyph i (0-based) by `(1 + 0.25 × i)`.
   - Gate by total base cost: <26 → none; 26–62 → mastery 25; 63–149 → 50; 150–399 → 75; ≥400 → 100.
   - Use saturating u32 arithmetic. Never repeat Morrowind's 16-bit wraparound.
8. **Hyperbolic armor (Morrowind) on top of BL4 health layers.**
   - `dmg_out = max(dmg/4, dmg × dmg / (dmg + AR))`. The 25% floor stops armor from nullifying hits, and armor is strongest against many small hits.
   - Every entity has up to three layers, Flesh, Ward (shield) and Plating (armor), depleted in order Ward → Plating → Flesh.
   - The element × layer matrix widens by Volume: V1 uses BL4 Easy (120/90), V2–V3 BL4 Normal (150/75), V4–V5 and Marginalia BL4 Hard (175/50). The Kinetic analog is always ×1.
9. **BL4 part grammar for every gun and tool archetype.**
   - Rarity sets **Gloss slots**: Common 0, Uncommon 1, Rare 2, Epic 3, Legendary 2 + 1 legendary part.
   - **Each genre manufacturer has one trait sentence.** A **Gloss** is another manufacturer's trait grafted on, the licensed part.
   - **Cross-genre Glosses carry their origin genre's axiom tags.** That makes Contradiction and Anachronism (brief §2) part of loot, not just theme. A Weird-West ricochet Gloss on a Wuxia crossbow is a legible omni-genre object.
   - Keep BL4's underbarrel same-manufacturer rule as a soft rule. Make the exceptions explicit, data-driven tags.
10. **Destiny 2 gear tier, orthogonal to rarity, set by Folio grade.**
    - Folio grade B / B+ / A / A+ / S maps to Tier 1 / 2 / 3 / 4 / 5 ("Difficulty = Grade = Reward Tier").
    - Tier sets **options per perk column**: 1 / 1 / 2 / 2 / 3. The player **chooses** among rolled options at any Scriptorium. Tier also sets enhanced perks (T2+) and enhanced components (T4+). T5 weapons get an enhanced genre or origin trait and **+tier to all weapon stats at masterwork**. T5 armor and trinkets get a **Tuning slot**: +2 to one sub-attribute and −2 to another, or +1 to the piece's three lowest.
11. **Armor rolls only 3 of the 24 sub-attributes**, as in Armor 3.0: archetype primary, archetype secondary, random tertiary. Per-piece sub-attribute totals by tier (D2's 52–75 bands scaled by about 0.21 for a 1–100 scale with baseline 40): T1 9–10, T2 11–12, T3 13–14, T4 15–16, T5 16 + Tuning. Split primary / secondary / tertiary at about 50% / 30% / 20%. Five armor pieces then add at most about 40 to a primary sub-attribute, which keeps the 150 buffed cap meaningful. Sub-attributes **101–150** (the brief's buffed band) unlock a **second named benefit** per sub-attribute, as D2 does past 100, so every point matters.
12. **Firmware becomes "Refrains"**, set tags on non-weapon gear. Effects step up at **1/2/3 copies, capped at 3**, and some Refrains roll only in specific genres or at Marginalia depth. Weapon set bonuses use D2's 2-piece and 4-piece rule.
13. **Enemy scaling by depth, never by player level.**
    - `depth = (Volume − 1) × 5 + Folio`.
    - Enemy HP = `base × (1 + 0.12 × (depth − 1))`. Damage = `base × (1 + 0.06 × (depth − 1))`.
    - Marginalia continues the curve with UVHM-style rank bumps: +50% HP per Marginalia rank, +2% damage and +5% loot quantity.
    - This is the single most important lesson from Oblivion.
14. **Identification channels (Morrowind and Oblivion).**
    - Each unknown item has up to 4 hidden properties. The number revealed on pickup comes from `Appraisal + Analysis/5`: under 25 → 0, 25–49 → 1, 50–74 → 2, 75–99 → 3, 100 → all.
    - Combining or using items can reveal shared hidden properties, as in Morrowind's mixing screen. An alchemy mix that shares a hidden effect names it in the codex for the rest of the run.
15. **Alchemy apparatus roles from Morrowind, magnitude curve from Oblivion, modifiers from Minecraft.**
    - `base = (Alchemy + Logic/10 + Fate/10) × VesselQuality`. Magnitude follows `(base / (BaseCost/10 × 4))^(1/2.28)` and duration is 4 × magnitude. Then apparatus factors: Calcinator 0.35, Retort 0.5 on magnitude and 1.0 on duration, Alembic −2 on negatives only, clamped so negatives never rise.
    - Reagent modifiers form an orthogonal axis, Minecraft-style: Extend (duration ×2), Amplify (+1 rank, duration ×½), Invert, Splash (same duration), Linger (cloud, duration ×¼).
16. **Light grid exactly like Minecraft, on 32³ chunks.**
    - Integer 0–15 with sky and block channels. Block light loses 1 per taxicab step. Sky light 15 does not decay going straight down.
    - **Hostile Ink-spawns require block light ≤ a genre threshold**: 0 for most genres, ≤ 7 for Gothic and Cosmic Horror.
    - Random ticks: 1 per 16³ section per 60 Hz tick (8 per 32³ chunk). That matches Minecraft's 60 random ticks per section per second (20 TPS × 3).
17. **Offer seeds advance only on commit (Minecraft XpSeed).** Scriptorium rewrites, enchant-like offers and Fortune Engine faces draw from a per-player stream that advances only when the player commits. Re-opening a menu never rerolls.
18. **Re-forge penalty: `2^c − 1` extra ink, where c is prior re-forges, with a hard cap of 39 ink-levels** (Minecraft anvil). It ends infinite min-maxing on a single item.
19. **Fortune Engines: a BL2-style integer weight table in bp, with a dangerous outcome.** Example starting table: 2,050 white, 1,540 green, 154 blue, 15 purple, 2 orange, 330 in-run currency ×1–3, 2,560 money, **770 "Ink Blot" (a hostile spawn or explosion)**, 512 cosmetic codex page, and 2,067 nothing. Payout is per player and not split in co-op. In-run currency only, never real money.
20. **The Codex battle pass copies BL4 Vault Cards and D2's archive.**
    - Progress is XP-driven, with no timers, and Volumes never expire.
    - Past Codex Volumes are claimable through an archive.
    - 100 tiers per Volume, against D2's 110 ranks for 1,000 Silver. The premium Volume refunds at least its price in premium currency across its tiers, as the brief requires.

### 8.2 Improve on the sources

21. **Make the Morrowind multiplier idea passive.** Each skill-up adds 1 Trace to its governing sub-attribute. **Every 10 Traces convert to +1 sub-attribute**, up to the natural cap of 100. Practice still shapes the body, with no rest-time bookkeeping game.
22. **Advantage as "Edge / Snag" (BG3) for real-time skill checks.** Roll twice and keep the best or worst; it never stacks, and Edge plus Snag cancel. Sources: high ground of 2.5 m or more (ranged hit gets Edge, and range extends by the height difference, BG3 rule); a target that is unaware, prone or blinded; attacking from darkness with darkvision. Shown on the reticle.
23. **Chameleon decay (BG3 Greater Invisibility).** Concealment effects survive each hostile action on a check at DC `15 + n`, or `15 + n²` on hardcore, where n is the number of successful actions since applying.
24. **Rest = Interlude (BG3).** A full rest between Folios costs 40 Supplies, or 80 on hard. With too few Supplies you get a half rest: HP and resources to ½ of max.
25. **A forbidden tree (BG3 illithid powers) as "Marginalia powers".** It is unlocked by consuming Editor-ink fragments. The cost is narrative: Echo companions react, and the genre lattice biases toward Horror. There is no hidden stat tax.
26. **Pure-cosmetic jackpots (BL4 Phosphene).** "Illuminated" variants roll at 1 in 2,000 on Legendary or higher. They change only the skin and unlock it for the whole model in the codex. A dopamine spike with zero power inflation.
27. **Separate logic from rendering as Oblivion Remastered proved.** The sim must run under a replaced renderer unchanged, and the brief's `palimpsest_sim` must have zero Bevy imports, checked in CI.

### 8.3 Avoid

28. Player-level enemy scaling (Oblivion). Hidden drop rates (BL4: months of upward patches). Expiring power (D2 reversed it in three months). Multiplicative sneak × power stacking. Sound through walls. Integer wraparound in cost fields. Alembic-style apparatus that can worsen the thing it should improve. Binding progress or owned content to a server that can shut down: **D2's June 2026 end of live service shows that PALIMPSEST must ship an offline-complete end-of-service mode** in which Codex progress, Inkwell pulls already owned and cosmetics stay available locally, verified by signed receipts.

---

## Open questions (not verified this session)

- Morrowind GMST defaults fMajorSkillBonus 0.75, fMinorSkillBonus 1.0, fMiscSkillBonus 1.25 and fSpecialSkillBonus 0.8 are recalled. The UESP game-settings page was not reachable through the API.
- Oblivion Remastered's new enemy-scaling curves and its XP-per-level formula are not documented on the pages read.
- BL4's real drop rates are unpublished. Only a fan estimate exists (about 4% world, about 5% dedicated). BL4 enemy level scaling within zones, and whether the UVHM table column order is as I read it.
- Destiny 2: the exact Tier 5 weapon masterwork bonus (+5 is from search snippets); the current archetype count (6 at launch per Wikipedia, 12 per Shattered Vault); exact trait-column counts per weapon frame.
- Minecraft lapis cost per slot (1/2/3) is recalled.
- The Oblivion unit-to-metre ratio (about 70 units per m) is recalled.
- BG1/BG2 mechanics are recalled.
