# R5: Systems Design Reference, with exact formulas from shipped games

**As of 2026-10-11.** Research dossier R5 for PALIMPSEST, the 3D first-person, real-time, 1–4 player co-op, omni-genre roguelike.

**Scope.** This file collects exact formulas, constants and data shapes from shipped games for each required system: attributes and sub-attributes, stealth, alchemy, spellcrafting, crafting, identification, and damage/armor. Where R1 (roguelike lineage) or R2 (RPG and looter lineage) already verified a model, this file gives a short version and points to them. The new material here is mostly **read directly from source code**: OpenMW (Morrowind), Fallout 2 Community Edition, Shattered Pixel Dungeon and Arx Libertatis. §9 maps every finding onto `spec/00_CANON.md`, using its units: 60 Hz ticks, Bp (basis points, 10000 = ×1.00), mHP, `Light` 0..15 and `Db`.

---

## Table of contents

0. Method, source aliases, evidence tags
1. Attribute and sub-attribute systems
2. Stealth and AI awareness
3. Alchemy, brewing and liquids
4. Spellcrafting
5. Crafting, stations and quality
6. Identification and appraisal
7. Damage, armor, poise and hit locations
8. Cross-system formula index
9. Design implications for PALIMPSEST

---

## 0. Method, source aliases, evidence tags

**Tags**

| Tag | Meaning |
|---|---|
| `[VERIFIED <url>]` | I fetched the page this session and read the claim on it. |
| `[VERIFIED-SRC <alias>:<file>]` | I read the source file this session. The formula is transcribed from code. This is the strongest tag. |
| `[SNIPPET <url>]` | The claim appeared in a search-result extract of that URL, but I did not open the page. Medium confidence. |
| `[XREF R1 §x]` / `[XREF R2 §x]` | A sibling dossier verified the claim this run, with the URL given there. I did not re-fetch it. |
| `[COMPUTED]` | Arithmetic or a fit I ran this session on verified data. |
| `[RECALLED]` | From memory, not checked this session. Confirm before shipping a number that depends on it. |

**Source aliases** (every file was fetched this session with `curl` from raw.githubusercontent.com):

- `OMW` = `https://raw.githubusercontent.com/OpenMW/openmw/master/`. OpenMW is a re-implementation of the Morrowind engine whose stated goal is matching vanilla behavior. GMST defaults come from `apps/opencs/model/world/defaultgmsts.cpp`. Files read: `apps/openmw/mwmechanics/{spellutil,alchemy,mechanicsmanagerimp,npcstats,combat,creaturestats}.cpp`, `apps/openmw/mwclass/npc.cpp`.
- `F2CE` = `https://raw.githubusercontent.com/alexbatalov/fallout2-ce/main/src/`. A reverse-engineered re-implementation of the Fallout 2 engine. The code comments carry the original binary addresses. Files read: `stat.cc`, `skill.cc`, `combat.cc`, `combat_defs.h`.
- `SPD` = `https://raw.githubusercontent.com/00-Evan/shattered-pixel-dungeon/master/core/src/main/java/com/shatteredpixel/shatteredpixeldungeon/`. Files read: `items/weapon/Weapon.java`, `items/armor/Armor.java`, `items/rings/Ring.java`, `items/wands/Wand.java`, `items/potions/Potion.java`. `items/Item.java` and `items/Generator.java` were fetched but not cited.
- `ARX` = `https://raw.githubusercontent.com/arx/ArxLibertatis/master/src/game/magic/`. Arx Libertatis is the open-source continuation of Arx Fatalis. File read: `SpellRecognition.cpp`.

**Access failures this session.** These shape which claims are `[RECALLED]`:

- UESP (`en.`, `en.m.`, `cs.`, `ck.`) returned 403 (a Cloudflare challenge).
- Fandom wikis (Elder Scrolls, Witcher, Tarkov, Fallout mods, Kenshi, Borderlands) returned 402.
- poewiki.net and poe2wiki.net were blocked by Anubis.
- valheim.wiki.gg and escapefromtarkov.wiki.gg returned 401. fallout.wiki returned 403.
- The shared web-search budget ran out partway through the session.

As a result, Skyrim, Tarkov, Path of Exile, Kenshi, Valheim, Witcher 3, Hitman, Magicka and Ars Magica figures below are `[RECALLED]` unless marked otherwise. For Morrowind, the OpenMW code confirms R2's UESP-sourced formulas independently, so those numbers have two independent sources.

---

## 1. Attribute and sub-attribute systems

### 1.1 Seven patterns, and what each is good for

| Pattern | Exemplar | Mechanism | Strength | Failure mode |
|---|---|---|---|---|
| Point-buy primaries, then derived **secondary characteristics** that can be bought separately | GURPS 4e | HP = ST, Will = IQ, Per = IQ, FP = HT, Speed = (HT+DX)/4, each adjustable at its own point cost | "Derived, but tunable." The sub-attribute exists without double-counting. | Point-cost bookkeeping |
| Primaries feed derived stats and **seed skills from attribute sums** | Fallout 1/2 SPECIAL | `Lockpick = 10 + PE + AG` | Two-attribute skills tie stats together naturally | 1-point dump stats (CH) |
| **Governing attribute** for skills; skill-ups raise attributes at level-up | Morrowind, Oblivion | ×2..×5 level-up multipliers | Practice shapes the body | Metagaming via the multiplier |
| **Modifier bands** | D&D `floor((x−10)/2)`, Qud `floor((x−16)/2)` | Only every 2nd point matters | Readable | Odd points are dead |
| **Wide-range physiology** with species medians | Dwarf Fortress, 0–5000 | 19 attributes; mental ones mostly change speed or duration | Huge NPC variety | Opaque to players |
| **Use-trained stats with no class** | Kenshi, IVAN | XP per action, falling as the stat rises | Pure emergent identity | Grind loops (Kenshi self-beating) |
| **Attribute as gear gate plus small passive** | Path of Exile, SPD strength | Requirement gates and +x% per N points | Builds feel like puzzles | Stat-stick tax |

### 1.2 GURPS 4th edition

- Primaries ST, DX, IQ and HT, human average 10. Cost per ±1 level: ST ±10, DX ±20, IQ ±20, HT ±10 character points [RECALLED].
- Secondary characteristics [SNIPPET https://wiki.rpg.net/index.php/GURPS_Attributes]:

| Secondary | Base | Cost to adjust |
|---|---|---|
| HP | = ST | ±2 per HP |
| Will | = IQ | ±5 per point |
| Perception | = IQ | ±5 per point |
| FP | = HT | ±3 per FP |
| Basic Lift | ST²/5 lb, rounded to the nearest whole number if ≥ 10 | via ST |
| Basic Speed | (HT + DX)/4, **not rounded** | ±5 per ±0.25 |
| Basic Move | floor(Basic Speed) | ±5 per yard/s |
| Dodge | floor(Basic Speed) + 3 | [RECALLED] |

- Encumbrance levels None / Light / Medium / Heavy / Extra-Heavy apply at ≤ 1, 2, 3, 6 and 10 × BL. They multiply Move by 1, 0.8, 0.6, 0.4 and 0.2 and give Dodge −0 to −4 [RECALLED].
- Resolution is 3d6 roll-under. Critical success on 3–4 always, 5 at effective skill 15, 6 at 16 or more. Critical failure on 18 always, 17 if effective skill ≤ 15, and any roll at skill + 10 or higher [RECALLED].
- **Lesson:** a sub-attribute should be *derived by default and adjustable on its own*. CANON already derives primaries from subs (the reverse direction), which is better for a use-trained game.

### 1.3 Fallout 2 SPECIAL, from source

**Derived statistics** (`critterUpdateDerivedStats`) [VERIFIED-SRC F2CE:stat.cc]:

| Derived stat | Exact code formula |
|---|---|
| Max HP | `ST + 2×EN + 15` (base stats with trait modifiers) |
| Action Points | `AG/2 + 5` (integer division) |
| Armor Class | `AG` |
| Melee Damage | `max(ST − 5, 1)` |
| Carry Weight | `25×ST + 25` lb |
| Sequence | `2×PE` |
| Healing Rate | `max(EN/3, 1)`. The secondary guide's "0.5 × EN" [VERIFIED https://www.ludo.guide/guide/fallout-2/derived-statistics] is **wrong**; the code wins. |
| Critical Chance | `LK` % |
| Radiation Resistance | `2×EN` % |
| Poison Resistance | `5×EN` % |

HP gained per level is `2 + EN/2` [SNIPPET https://falloutmods.fandom.com/wiki/Fallout_engine_calculations].

**Skills.** `value = default + statMod × (stat1 [+ stat2]) + points`. Tagged skills count invested points twice and gain +20. The 4th tag from the Tag! perk does not get the +20. The value is capped at 300 [VERIFIED-SRC F2CE:skill.cc `skillGetValue`]. The table rows below follow Fallout 2's skill order [RECALLED order; values VERIFIED-SRC F2CE:skill.cc `gSkillDescriptions`]:

| Skill | Default | Multiplier × stats | Use-XP field |
|---|---|---|---|
| Small Guns | 5 | 4×AG | 0 |
| Big Guns | 0 | 2×AG | 0 |
| Energy Weapons | 0 | 2×AG | 0 |
| Unarmed | 30 | 2×(AG+ST) | 0 |
| Melee Weapons | 20 | 2×(AG+ST) | 0 |
| Throwing | 0 | 4×AG | 0 |
| First Aid | 0 | 2×(PE+IN) | 25 |
| Doctor | 5 | 1×(PE+IN) | 50 |
| Sneak | 5 | 3×AG | 0 |
| Lockpick | 10 | 1×(PE+AG) | 25 |
| Steal | 0 | 3×AG | 25 |
| Traps | 10 | 1×(PE+AG) | 25 |
| Science | 0 | 4×IN | 0 |
| Repair | 0 | 3×IN | 0 |
| Speech | 0 | 5×CH | 0 |
| Barter | 0 | 4×CH | 0 |
| Gambling | 0 | 5×LK | 0 |
| Outdoorsman | 0 | 2×(EN+IN) | 100 |

The use-XP column holds the XP granted on successful non-combat use. The values are verified; reading the field as XP is [RECALLED].

**Skill-point cost per +1%** (`skillsGetCost`, by current value): 1 up to 100, 2 at 101–125, 3 at 126–150, 4 at 151–175, 5 at 176–200, 6 at 201 and above [VERIFIED-SRC F2CE:skill.cc].

**Lesson:** two-stat skills plus escalating cost bands give soft caps with no hard wall. The cost band keyed to the *current value* is a reusable pattern for any point-buy layer, such as PALIMPSEST's meta upgrades.

### 1.4 Elder Scrolls: governance, use-growth and level-up (Morrowind from code)

- **Skill progress needed for the next point** [VERIFIED-SRC OMW:npcstats.cpp + defaultgmsts.cpp]:
  `req(s) = (1 + s_base) × typeFactor × specFactor`
  - typeFactor: Major 0.75 (`fMajorSkillBonus`), Minor 1.0 (`fMinorSkillBonus`), Misc 1.25 (`fMiscSkillBonus`).
  - specFactor: 0.8 (`fSpecialSkillBonus`) if the skill's specialization matches the class, otherwise 1.
- **Level-up:** `iLevelupTotal = 10` Major or Minor skill increases. For the number of governed skill increases n = 1..10, the attribute multiplier `iLevelUp{n}Mult` is `2,2,2,2,3,3,3,4,4,5`. n = 0 gives ×1 [VERIFIED-SRC OMW:npcstats.cpp `getLevelupAttributeMultiplier`, defaultgmsts.cpp].
- **Health:** `floor(0.5 × (STR + END))` at creation, then `+ END × fLevelUpHealthEndMult (0.1)` per level [VERIFIED-SRC OMW:npcstats.cpp].
- **Fatigue term**, used in nearly every check: `fFatigueBase − fFatigueMult × (1 − cur/max)` = `1.25 − 0.5 × (1 − cur/max)`. That is ×1.25 rested and ×0.75 exhausted [VERIFIED-SRC OMW:creaturestats.cpp + defaultgmsts.cpp].
- Oblivion: XP `0.25883 × L^1.5`, Luck-modified skill and mastery ranks at 25/50/75/100. Oblivion Remastered (2025) gives 12 "virtues" per level and XP from every skill [XREF R2 §2.2–2.5].
- Skyrim [RECALLED]: three pools (Health, Magicka, Stamina), +10 to one per level; choosing Stamina also gives +5 carry weight. XP to the next character level is `(L + 3) × 25`. Each skill increase grants character XP equal to the new skill level. Skill-use XP to advance is `SkillImproveMult × level^1.95 + SkillImproveOffset`, set per skill. Perk trees are gated by skill thresholds.

### 1.5 D&D 5e / 2024, BG3, Pathfinder 2e [RECALLED]

- `mod = floor((score − 10)/2)`. Proficiency bonus is +2 at levels 1–4, +3 at 5–8, +4 at 9–12, +5 at 13–16 and +6 at 17–20.
- Point buy: 27 points, scores 8–15, costing 8:0, 9:1, 10:2, 11:3, 12:4, 13:5, 14:7, 15:9. Standard array: 15/14/13/12/10/8. There are 18 skills, each keyed to one ability. The 2024 rules move the ability increase to the Background (+2/+1 or +1/+1/+1). BG3 uses this core [XREF R2 §3.2].
- PF2e: proficiency is untrained 0, trained L+2, expert L+4, master L+6, legendary L+8. Degrees of success: beating the DC by 10 or more is a critical success, missing by 10 or more is a critical failure, and a natural 20 or 1 shifts the result one degree.
- **Lesson:** rank gates (trained/expert/master) are what players remember; the numeric bonus is incidental. CANON's 25/50/75/100 mastery tiers are the same idea.

### 1.6 Caves of Qud [VERIFIED https://wiki.cavesofqud.com/wiki/Attributes]

- `mod = floor((value − 16)/2)`. Each attribute has a **signature lever**:
  - **STR** adds to melee PV. Carry capacity is 15 × STR lb. Saves against forced movement (asphalt, webs).
  - **AGI**: `DV = 6 + AGI mod + bonus DV`, plus accuracy.
  - **TOU**: starting and per-level HP, regeneration, bleeding and poison saves.
  - **WIL** adds to MA. Each point above 16 cuts ability cooldowns by 5% of base, up to 80% at WIL 32, with a 5-round minimum. Each point below 16 adds 5%.
  - **INT**: skill points per level are 70 for True Kin and 50 for Mutants, then +4 per point above 10 and −4 per point below, applied **retroactively**. Also governs artifact examination.
  - **EGO**: prices, Persuasion, and **mental mutation levels**. EGO is the only attribute that changes mutation levels.
- Growth: every attribute +1 at levels 6, 12, 18 and so on; +1 attribute point to spend at levels 3, 9, 15 and so on.
- **Lesson:** one modifier formula across all attributes, with one unique, legible lever per attribute. WIL→cooldown is the model for a mental sub-attribute that matters in real time.

### 1.7 Dwarf Fortress [VERIFIED https://dwarffortresswiki.org/index.php/Attribute]

- 19 attributes, each 0–5000.
  - 6 physical: Strength, Agility, Toughness, Endurance, Recuperation, Disease Resistance.
  - 13 mental: Analytical Ability, Focus, Willpower, Creativity, Intuition, Patience, Memory, Linguistic Ability, Spatial Sense, Musicality, Kinesthetic Sense, Empathy, Social Awareness.
- Dwarf medians: STR 1250, AGI 900, TOU 1250, END 1000, Focus 1500, Spatial Sense 1500, most others 1000–1250. The human median is always 1000.
- Seven-point range sets in creature raws, for example `[200:700:900:1000:1100:1300:2000]` (default), "+" `[450:950:1150:1250:1350:1550:2250]`, "++" `[700:1200:1400:1500:1600:1800:2500]`, "−" (Agility) `[150:600:800:900:1000:1100:1500]`.
- Description tiers change at ±250, 500, 750 and 1000 from the species median.
- **Training:** each attribute has a cost-to-improve (CTI), default 500. The counter increments every 10 ticks of relevant labor, which gives about 5000 ticks per +1 at default CTI.
- **Cap** = `max(start × ATT_CAP_PERC/100, start + ATT_RANGE.median)`, with ATT_CAP_PERC 200 by default.
- Per a Toady One quote on the page, mental attributes mostly change *duration or speed* of linked skills, not outcomes.
- How the 7-point range is sampled (equal chance per segment, uniform inside) is [RECALLED].
- **Lesson:** the cap rule `max(2 × start, start + median)` is a clean anti-grind ceiling for use-trained attributes that still honors talent.

### 1.8 Kenshi, IVAN, Path of Exile

- **Kenshi** [RECALLED; wiki unreachable]:
  - No character levels. About 25 stats on a 0–100 scale, trained only by use. XP per action falls as the stat rises, and harder opposition gives more XP.
  - Strength trains while over-encumbered, Toughness by taking damage and being knocked out, Athletics by running (faster when encumbered). Training dummies stop granting XP above a low cap.
  - Known degenerate loop: players beat their own squad to farm Toughness.
- **IVAN:** asymptotic training toward a species norm, `Exp += (Natural×(100+Value) − 100×Old) × Speed × 2e-8` [XREF R1 §4.3].
- **Path of Exile 1** [RECALLED]:
  - STR: +1 life per 2 STR, +1% melee physical damage per 5 STR.
  - DEX: +2 accuracy per DEX, +1% evasion per 5 DEX.
  - INT: +1 mana per 2 INT, +1% energy shield per 5 INT.
  - Attributes gate gear and gems through requirements.
  - Path of Exile 2's attribute bonuses differ and are unverified (see open questions).

### 1.9 Sub-attributes done well: rules extracted

1. **Every sub-attribute owns at least 1 and at most 3 mechanical levers, and at least one lever is unique to it.** Qud's WIL→cooldown and GURPS Per→sense rolls are examples. Sub-attributes that only feed one sum get ignored.
2. **Derive primaries from subs, never the reverse.** Training shapes subs, so the primary is a readable summary, like DF's descriptive tiers.
3. **Couple subs through formulas, not through ownership.** Fallout's `Lockpick = PE + AG` coupling lives in the skill formula. Every skill keeps one governing stat for UI clarity, and coupling appears in derived rows.
4. **Use-training needs an asymptote and a talent ceiling.** IVAN's asymptote and DF's `max(2 × start, start + median)` both work.
5. **Mental subs should mostly change speed, duration and information** (DF, Qud), because outcomes belong to skills and gear. CANON rows 26–36 follow this.
6. **Body-local physicals** (IVAN, DF, Qud body parts) turn injury into stat loss with real stakes.

---
## 2. Stealth and AI awareness

### 2.1 Morrowind detection, exact from code [VERIFIED-SRC OMW:mechanicsmanagerimp.cpp `awarenessCheck`; GMSTs from defaultgmsts.cpp]

```
sneakTerm = 0
if sneaking:
    sneakTerm = fSneakSkillMult(1.0)×Sneak + 0.2×AGI + 0.1×LCK + bootWeight×fSneakBootMult(−1.0)   // boots count only on the ground
distTerm  = fSneakDistanceBase(0.5) + fSneakDistanceMultiplier(0.002) × distance_units
x = sneakTerm × distTerm × fatigueTerm(self) + Chameleon;   if Invisibility > 0: x += 100
obsTerm   = obsSneak + 0.2×obsAGI + 0.1×obsLCK − obsBlind
y = obsTerm × fatigueTerm(observer) × (angle(observer facing, target) > 90° ? fSneakNoViewMult(0.5) : fSneakViewMult(1.5))
observer is aware  ⇔  roll0to99 ≥ (x − y)          // so P(spotted) ≈ clamp(1 − (x−y)/100, 0, 1)
```

**Worked example** [COMPUTED]: Sneak 50, AGI 50, LCK 40, 10-weight boots, 500 units away, full fatigue: x = (50 + 10 + 4 − 10) × 1.5 × 1.25 = 101.25. An observer with Sneak 20, AGI 40, LCK 40 has obsTerm 32. Facing the target: y = 32 × 1.25 × 1.5 = 60, x − y = 41.25, so about 58% spotted per check. From behind: y = 20, x − y = 81.25, so about 18%.

Lessons:
- Distance **multiplies the sneaker's score**: being far away makes your skill count more.
- Facing gives a ×3 swing (1.5 against 0.5).
- Light plays no part, which is Morrowind's most-criticized stealth gap.

### 2.2 Oblivion and Skyrim

- **Oblivion** has a continuous detection score built from separate **sound**, **sight**, **skill** and **base** terms. Distance factor is `1 − d/MaxRange`. The light term is `fDetectionSneakLightMod(−5) + light(0..100)` × `fSneakLightMult(1.4)`. Boots add weight-based noise. Thresholds use hysteresis: above 0 is seen, above −20 is noticed, a searching hostile loses you only below −20, and one in combat only below −50. It updates every 0.3 s. Full formula, GMST table and worked example in [XREF R2 §2.10]. The 1.4 and −25 defaults were also seen in [SNIPPET https://cs.uesp.net/wiki/Detection].
- **Skyrim** [RECALLED]:
  - The detection "eye" is closed, half-open (searching) or open (detected).
  - Inputs: light, movement speed, armor weight, distance and LOS. Muffle removes armor noise.
  - Sneak-attack multipliers: ×3 melee and ×2 ranged at base; Backstab ×6 one-handed; Assassin's Blade ×15 daggers; Deadly Aim ×3 ranged.

### 2.3 Fallout 2: light as an accuracy penalty [VERIFIED-SRC F2CE:combat.cc `attackDetermineToHit`]

For the player only, using the **target's** light intensity on a scale of 0..65536:

| Target light | Player to-hit |
|---|---|
| ≤ 26214 (≤ 40%) | −40 |
| ≤ 39321 (≤ 60%) | −25 |
| ≤ 52428 (≤ 80%) | −10 |
| above 80% | 0 |

Weapons with the Night Sight perk treat light as full. A blinded attacker takes −25, and a knocked-down or knocked-out target gives +40. Darkness therefore protects twice: AI can't see you, and if seen you're hard to hit. This mapping ports directly to PALIMPSEST's `Light 0..15`.

### 2.4 Thief (Dark Engine), Splinter Cell, Mark of the Ninja, Shadow Tactics

- **Thief (1998):**
  - Looking Glass added the **light gem** because playtesters couldn't tell whether they were hidden. It "gets brighter or dimmer depending on how visibly hidden you are" and has an audio counterpart [VERIFIED https://www.gamedeveloper.com/design/building-the-original-i-thief-s-i-revolutionary-stealth-system].
  - The gem reflects light at the player's position, with modifiers for movement and crouching. Footstep loudness depends on surface: metal and tile are loud, carpet and moss quiet. Moss arrows lay down quiet patches, and water arrows put out torches.
  - AI alertness has 4 levels (0–3) and decays over time.
  - Sound travels through the room/portal graph with door attenuation, so it goes *around* corners, not through walls.
  - Everything after the first item is [RECALLED].
- **Splinter Cell: Chaos Theory** shows a light meter and a **sound meter comparing your noise to ambient noise**. A noise below ambient is inaudible [RECALLED]. This is the cleanest rule for loud environments, such as a cyberpunk club or a steam foundry.
- **Mark of the Ninja (2012):**
  - Every noise enemies can hear is drawn as an expanding ring, and an enemy hears it only if the ring reaches them. The ring's size matches loudness.
  - The first build used one generic effect regardless of loudness, and playtesters couldn't read the AI. Matching the effect to the real noise radii fixed it at once [SNIPPET https://www.gamingnexus.com/Article/Mark-of-the-Ninja-Interview/Item3565.aspx; SNIPPET https://gameinformer.com/b/features/archive/2012/10/15/afterwords-mark-of-the-ninja.aspx].
  - Light is near-binary: in darkness, guards see you only up close [RECALLED].
- **Shadow Tactics (2016) and Desperados III (2020):**
  - The vision cone has three zones. In the **solid** near zone you are spotted in any posture. In the **striped** far zone you are hidden only if crouched. In the **dotted** zone, cover such as bushes hides you always.
  - Before an alarm, a yellow fill sweeps toward you, and you are spotted when it touches you. Night missions weaken cones, and standing in bright light makes you notable. Players can click any enemy to see its cone [SNIPPET https://www.thesixthaxis.com/2024/12/11/shadow-tactics-aikos-choice-review/; SNIPPET https://steamcommunity.com/app/418240/discussions/0/5170673756531996632].
  - The fill speed falls with distance [RECALLED].

### 2.5 Hitman, Metal Gear, Shattered PD

- **Hitman: World of Assassination** [RECALLED]:
  - Disguises grant access to zones. **Enforcers**, marked with a white dot, see through specific disguises, while others react only to illegal actions: a visible weapon, trespassing, climbing in view, lockpicking.
  - A HUD suspicion meter fills while an enforcer watches. Blend-in spots reset exposure. A found body escalates the whole level.
  - CANON already has a disguise suspicion-rate multiplier (§6.4 row 40).
- **Metal Gear Solid:**
  - In MGS1, being spotted puts the game into Alert. Evasion follows, and when its countdown reaches 0 the game returns to Infiltration [SNIPPET https://secure.cdn.us.playstation.com/manuals/classic/games/metal-gear-solid-manual-en.pdf]. The full cycle is Infiltration → Alert → Evasion → Caution → Infiltration, shown with timers on the radar [RECALLED].
  - Ground Zeroes and The Phantom Pain drop the phase clock and radar. Players read the phase from callouts and music, and a short slow-motion **Reflex** window lets you silence the spotter before the alarm [SNIPPET https://www.metalgearinformer.com/?p=9380; SNIPPET https://steamcommunity.com/app/311340/discussions/0/523890046882232991].
  - The Phantom Pain's soldiers adapt to your habits with helmets, lights and decoys [RECALLED].
- **Shattered PD v4:** per-turn detection probabilities for sleeping (`1/(d+S)^2`), wandering in a 75° cone (`1/(d/2+S)`) and behind (`1/(d+S)^3`), plus an **Investigating** state that moves to a noise without attacking [XREF R1 §2.12].

### 2.6 The convergent awareness state machine

Every modern stealth game listed above converges on this structure (synthesis):

```
UNAWARE ──stimulus──► SUSPICIOUS ──meter full or strong noise──► INVESTIGATING ──confirmed sight──► ALERT/COMBAT
   ▲                       │ decay                          │ nothing found (timer)               │ LOS lost
   │                       ▼                                ▼                                     ▼
   └──────────────── RETURNING (raised vigilance, "Caution") ◄────────────────────────────── SEARCHING (last-known position)
```

Each feature comes from a specific game:

- **Hysteresis thresholds** prevent flicker: Oblivion's 0 / −20 / −50.
- A **separate meter per observer–target pair**: Shadow Tactics' fill, Hitman's suspicion.
- **Group propagation through audible callouts**: MGS radio calls, Hitman witnesses.
- **Body discovery** forces global escalation: Hitman, Shadow Tactics.
- **Raised vigilance** after an incident: MGS Caution, Thief guards who stay jumpy.
- A **non-attacking investigate** state: SPD v4.
- **Player-visible state**: the light gem, noise rings and cone fill.

§9.2 gives exact PALIMPSEST numbers.

---

## 3. Alchemy, brewing and liquids

### 3.1 Morrowind alchemy, exact from code [VERIFIED-SRC OMW:alchemy.cpp, spellutil.cpp; GMSTs defaultgmsts.cpp]

```
f = Alchemy + 0.1×INT + 0.1×LCK                                 // getAlchemyFactor
brew fails (ingredients lost) if f < roll0to99                   // also fails if every effect was nullified
x = f × MortarQuality × fPotionStrengthMult(0.5)
magnitude = x / fPotionT1MagMult(1.5) / EffectBaseCost  = f×Q / (3×BaseCost)
duration  = x / fPotionT1DurMult(0.5) / EffectBaseCost  = f×Q / BaseCost        (seconds)
potion value = x × iAlchemyMod(2) = f × Q                       // value ignores which effects it has
round(magnitude), round(duration); effects with NoMagnitude/NoDuration use 1
```

**Apparatus** (`applyTools`, applied separately to magnitude and duration). `r` is retort quality (positive effects), `a` alembic quality (harmful effects) and `c` calcinator quality:

| Setup | Positive effect, has both magnitude and duration | Positive, single-axis | Harmful effect |
|---|---|---|---|
| Retort or alembic + calcinator | `+ (2r + c)` | `+ (2/3)(r + c) + 0.5` | `÷ (2a + 3c)` |
| Retort or alembic only | `+ r` | `+ r + 0.5` | `÷ (1 + a)` |
| Calcinator only | `+ c` | `+ c + 0.5` | `+ c`, or `+ c + 0.5` if single-axis (the calcinator *raises* poison strength) |

**Eating a raw ingredient** (`rollIngredientEffect`) [VERIFIED-SRC]:

```
x = (Alchemy + 0.2×INT + 0.1×LCK) × fatigueTerm
roll = 0..99; if roll > x: no effect
y = (roll / min(x, 100)) × 0.25 × x
duration = int(y)
magnitude = max(1, floor(0.05×y / (0.1×BaseCost)))      // if the effect also has a duration
```

Mortar qualities (Apprentice 0.5 to SecretMaster 2.0), visible effects per skill band (0, 15, 30, 45, 60 → 0..4 effects), and "hidden effects still combine" are in [XREF R2 §1.9]. The code matches UESP's `f×Q/(3×BaseCost)` exactly.

### 3.2 Oblivion and Skyrim

- **Oblivion** uses a power law, `Base_Mag = [(EffAlch + Mortar×25)/(BaseCost/10 × 4)]^(1/2.28)`, and `Base_Dur = 4 × Base_Mag`. Effects visible per ingredient: 1/2/3/4 by mastery rank. Master can brew from one ingredient [XREF R2 §2.9].
- **Skyrim** [RECALLED]:
  - Formula: `Magnitude or Duration = round(Base × 4 × (1 + 0.5 × Skill/100) × (1 + FortifyAlchemy/100) × (1 + Alchemist/100) × (1 + Physician/100) × (1 + Benefactor or Poisoner/100))`. Alchemist gives 20/40/60/80/100%. Physician gives 25% to restore effects; Benefactor and Poisoner give 25%.
  - Each ingredient has 4 effects. Eating reveals the 1st; combining reveals shared effects. A potion carries every effect shared by at least 2 of its 2–3 ingredients. Its name and value come from the most valuable effect.
  - Fortify-Alchemy gear restoration loops are a well-known exploit; PALIMPSEST must not let crafted buffs feed back into crafting without limit.

### 3.3 Shattered Pixel Dungeon field brewing [VERIFIED-SRC SPD:items/potions/Potion.java `SeedToPotion`]

- The alchemy pot takes **exactly 3 seeds** at 0 energy cost. Each seed maps to one potion:

| Seed | Potion |
|---|---|
| Blindweed | Invisibility |
| Mageroyal | Purity |
| Earthroot | Paralytic Gas |
| Fadeleaf | Mind Vision |
| Firebloom | Liquid Flame |
| Icecap | Frost |
| Rotberry | Strength |
| Sorrowmoss | Toxic Gas |
| Starflower | Experience |
| Stormvine | Levitation |
| Sungrass | Healing |
| Swiftthistle | Haste |

- Outcome by number of distinct seed types:
  - **1 type:** that potion, and it is **auto-identified**.
  - **2 types:** 25% a random potion (`Random.Int(4)==0`), otherwise the potion of a random ingredient.
  - **3 types:** 50% random.
- **Healing anti-farm:** while the result is Healing, reroll with probability `COOKING_HP.count/10`, and increment the counter each time Healing is produced.
- Alchemy energy recipes are in [XREF R1 §2.10].
- **Lesson:** brewing is also identification. Brewing a pure batch identifies the potion type, which turns ID into a crafting decision.

### 3.4 Noita: material reactions as data [VERIFIED https://noita.wiki.gg/wiki/Documentation:_Reaction; https://noita.wiki.gg/wiki/Alchemy]

**Reaction record** (`<Reaction/>` fields):

| Field | Meaning |
|---|---|
| `input_cell1`, `input_cell2` | Required inputs: a material name or `[tag]` |
| `input_cell3` | Optional third input |
| `output_cell1`, `output_cell2` | Required outputs |
| `output_cell3` | Optional third output |
| `probability` | int = percent × 100, so two implied decimals |
| `fast_reaction` | bool: takes priority over other reactions |
| `req_lifetime` | int: input must exist this long before reacting |
| `blob_radius1`, `blob_radius2` | One input pixel converts a radius of output |
| `blob_restrict_to_input1/2` | Restrict the blob to matching input pixels |
| `convert_all` | Convert every touching pixel of both inputs |
| `entity` | Spawn an entity at the reaction site |
| `direction` | none/top/bottom/left/right |
| `ExplosionConfig`, `audio_fx_volume_1`, `cosmetic_particle` | Effects on reaction |

Engine tags include `[any_liquid]` and `[any_powder]`. Material tags such as `[water]` and `[corrodible]` live in `materials.xml`.

**Seeded secret recipes:** *Lively Concoction* and *Alchemic Precursor* are each made of **3 materials picked at random per world seed**.

**Example reactions:**
- Alchemic Precursor + meat → Draught of Midas, which then converts more Precursor.
- Precursor + Lava → Gas of Midas.
- Blood + Oil + Flummoxium → Unstable Polymorphine.
- Concentrated Mana + Water → Concentrated Mana (it duplicates itself).
- Silver + Copper + Blood → Diamond.
- Diamond + Honey → Ambrosia + Poison.
- Vomit + Bone → Diminution (×10 damage taken).
- Acceleratium + Levitatium → Hastium.

Fire and toxic sludge each deal 2% of max HP per second. Sludge never kills: it stops below 2% HP.

### 3.5 Caves of Qud liquids [VERIFIED https://wiki.cavesofqud.com/wiki/Liquids]

- **Unit: drams.** A waterskin holds 64. Depth bands by total drams on a tile: puddle under 200, pool 200–1999, deep pool (swimming) over 2000.
- **Mixing:** touching liquids merge. Names stack descriptive prefixes, as in "inky sugary bloody water".
- **Evaporation:** only puddles of 199 drams or less evaporate. Each dram has a per-tick chance equal to the volume-weighted evaporability, and each component evaporates separately. 0%: asphalt, lava, wax, oil, salt. 100%: cloning draught, neutron flux.
- **Slippery puddles** of 199 drams or less (gel, ink, oil, slime, or frozen) need a save at target 10 or you slide. **Sticky** liquids (asphalt, honey, wax, sap) hold you up to 12 turns, with escape saves at 4, or 7 for asphalt.
- **Ground absorption:** liquid poured on bare ground has half its volume replaced by salt.
- **Temperature:** liquids drift toward their base temperature and heat what they touch. Lava ignites non-glass containers. Acid destroys organic containers.
- **Value scales with volume**: 64 drams of water is worth 64, and 64 of lava is worth 1600.

### 3.6 The Witcher 3, Potion Craft, Darkest Dungeon, Minecraft

- **The Witcher 3** [RECALLED]:
  - A potion is crafted **once**, then **refilled for free while meditating**, if you carry a strong alcohol such as Alcohest or Dwarven Spirit.
  - Each potion adds **toxicity** to a meter that starts at 100. Going over the threshold causes damage and screen effects.
  - Enhanced and Superior recipe tiers add doses and strength. Decoctions are long-duration, high-toxicity buffs. Oils target monster classes.
  - CANON's `res.toxicity` is this model.
- **Potion Craft: Alchemist Simulator** [RECALLED]:
  - Each ingredient is a **path drawn on a 2D map** from the current point. Effects sit at fixed map locations.
  - Grinding lengthens a path. Water pulls the point back toward the origin. Heating creates vortices. Salts rotate or nudge the point.
  - Effect strength I/II/III depends on how close the point lands to the effect icon.
  - The lesson is spatial, skill-based alchemy with no hidden RNG.
- **Darkest Dungeon curios** (used here as a "right tool" model) [VERIFIED https://darkestdungeon.wiki.gg/wiki/Curios]. Interacting by hand rolls the curio's table. Using a matching provision replaces the roll with a fixed result:
  - Eldritch Altar: Stress +25 40%, remove negative quirk 10%, nothing 50%. With Holy Water: remove negative quirk 100%.
  - Iron Maiden: loot ×2 40%, Claustrophobia 20%, Tetanus 13.3%, random disease 6.7%, nothing 20%. With Medicinal Herbs: loot ×2 100%.
  - Bas-Relief: positive quirk 66.7%, negative quirk 22.2%, disease 11.1%. With a Shovel: **Stress +100, 100%**. The "right tool" is not always benign, which keeps it from being a solved lookup.
- **Minecraft brewing** [VERIFIED https://minecraft.wiki/w/Brewing]:
  - Water + Nether Wart makes an Awkward potion, the base for effect ingredients.
  - Redstone extends duration and cannot combine with Glowstone. Glowstone raises the level and shortens duration.
  - Fermented Spider Eye corrupts (inverts) the effect. Gunpowder makes a splash potion. Dragon's Breath turns splash into lingering, which lasts ¼ as long.
  - Each operation takes 20 s (400 ticks). The stand's fuel bar holds 20 charges.
  - **Lesson:** a small set of orthogonal *modifier reagents* (extend, amplify, invert, splash, linger) gives a large, learnable recipe space.

---
## 4. Spellcrafting

### 4.1 Noita wand building: the reference model for a programmable implement

**Wand stats** [VERIFIED https://noita.wiki.gg/wiki/Wands]:

| Stat | Exact semantics |
|---|---|
| Shuffle | The deck is reordered on switch-to, on wrap, on recharge and on the first Wand Refresh per cast. |
| Spells/Cast | Draws at the start of each cast; practically 1–26. It never wraps by itself and cannot be modified. A recharge of 1 s or more guarantees Spells/Cast > 1 at generation. |
| Cast Delay | Seconds before the next cast. Every spell in a multicast group adds its delay. Spells inside triggers and timers are ignored. Applies only if the cast contains projectiles. |
| Recharge Time | Starts after the **last** spell is cast. Spells add or subtract as they cast. Swapping wands does not cancel a recharge. |
| Mana Max / Mana Charge Speed | Mana regenerates at charge speed per second, applied each frame at 1/60 of the value. Wands regenerate even when not held. |
| Capacity | Number of slots, 1–66, soft-capped at 26 (1 in 10,001 chance to exceed). |
| Spread | Degrees. Wand spread + spell spread. Negative wand spread cancels positive spell spread but adds no inaccuracy of its own. |
| Speed multiplier | Hidden, usually 0.8–1.2, rarely 0.5 or 2.0. |
| Always Cast | Prepended to every cast at 0 mana. Limited-charge spells become unlimited. Cannot be removed. |

**Timing rules:**
- **Cast delay and recharge run simultaneously; the wait is `max(castDelay, recharge)`.**
- Negative totals cancel spell-added delay. If no spell adds delay, a negative cast delay counts as 1 frame and a negative recharge as 0.
- 60 frames = 1 s.
- Starter wand ranges: mana 80–130, charge 25–40, capacity 2–3, cast delay 0.15–0.25 s, recharge 0.33–0.47 s.
- Wand tiers run 1–6, plus a rare 10 that generates as 11.

**Draw engine** [VERIFIED https://noita.wiki.gg/wiki/Expert_Guide:_Draw]:
- Three piles: **Deck → Hand → Discard**.
- *Draw* moves the top Deck spell into the Hand. If the spell lacks mana or charges, it goes to Discard and the next spell is drawn instead; if it was the last spell in the Deck, the draw is still spent.
- Modifiers draw 1. Multicasts draw N. Projectiles draw nothing unless they are triggers.
- Evaluation is **depth-first**.
- Spells modify a **shot state**, not each other. There is a base shot state per cast and one per trigger payload. Only the base state's cast delay governs the next cast.
- **Wrap** (Discard → Deck) happens when a draw finds the Deck empty, which also schedules a reload, or when Wand Refresh is cast, which does not.
- **Reload** happens at cast end if the Deck emptied during the cast. It resets recharge to base.
- Mana is capped at cast start but can exceed the cap during a cast.
- Trigger and timer payloads carry the next spell(s), fired at the projectile's position on hit or timeout.
- For trigger and timer chains, mana is charged as if every spell in the chain fires, even if the trigger never fires [VERIFIED https://noita.wiki.gg/wiki/Wands].

### 4.2 Morrowind and Oblivion: spell cost from effect parameters

**Morrowind effect cost** [VERIFIED-SRC OMW:spellutil.cpp `calcEffectCost`; matches XREF R2 §1.10]:

```
x  = 0.5×(minMag + maxMag) × 0.1×BaseCost × (durationOffset + duration)    // durationOffset = 1 for player-made spells
x += 0.05 × max(minArea, area) × BaseCost                                  // minArea = 1 for player spells
cost = x × fEffectCostMult(0.5);   × 1.5 if range = Target;   spell = Σ effects, rounded
```

This reduces to `(min+max)×(dur+1)×BaseCost/40 + area×BaseCost/40`.

**Cast chance** [VERIFIED-SRC OMW:spellutil.cpp `calcSpellBaseSuccessChance`]:

```
castChance = 2 × skill(lowest-school effect) − spellCost + 0.2×WIL + 0.1×LCK
```

The effective school is the effect whose `2×skill − effectCost` is lowest. The result is then multiplied by `fatigueTerm` [XREF R2 §1.10].

**Enchanted items:**
- Cast cost = `castCost − castCost/100 × (Enchant − 10)`, minimum 1.
- Charge = autocalc cost × {Once 1, Strike 10, Use 5, Constant 10} (`iMagicItemCharge*`).
- Recharge is `fMagicItemRechargePerSecond` = 0.05 [VERIFIED-SRC OMW:spellutil.cpp + defaultgmsts.cpp].

**Oblivion** uses a power law: `Cost = BaseCost/10 × Mag^1.28 × Dur × (Area×0.15)`, where each factor < 1 counts as 1, then ×1.5 on Target, then × `(1.4 − 0.012 × skill)` [XREF R2 §2.7]. **Mag^1.28 is the key number:** it makes large magnitude superlinear, so splitting damage into duration is cheaper.

### 4.3 Arx Fatalis: drawn runes, exact codes [VERIFIED-SRC ARX:SpellRecognition.cpp]

**Recognition algorithm:**
1. Resample the mouse stroke to **20 points**, giving 19 segments.
2. Quantize each segment's angle to **8 directions** with `round(angle/(π/4)) % 8`.
3. Compare against stored patterns with angle distance `4 − ||a−b| − 4|` and **tolerance 2**.

**Rune stroke codes** use numpad directions (8 = up, 2 = down, 4 = left, 6 = right, 7/9/1/3 = diagonals): Aam `6`, Nhi `4`, Mega `8`, Rhaa `2`, Yok `268`, Taar `626`, Kaom `41236` or `1236`, Vitae `68`, Vista `31`, Stregum `838`, Morte `62`, Cosum `6248`, Comunicatum `62426`, Movis `616`, Tempus `862686`, Folgora `93`, Spacium `4268`, Tera `926`, Cetrius `386`, Fridd `862`.

**Spells are rune sentences** (level in parentheses, from code comments):

| Spell | Runes |
|---|---|
| Magic Missile (1) | Aam Taar |
| Ignite (1) | Aam Yok |
| Douse (1) | Nhi Yok |
| Heal (2) | Mega Vitae |
| Harm (2) | Rhaa Vitae |
| Armor (2) | Mega Kaom |
| Lower Armor (2) | Rhaa Kaom |
| Fireball (3) | Aam Yok Taar |
| Ice Projectile (3) | Aam Fridd Taar |
| Speed (3) | Mega Movis |
| Levitate (5) | Mega Spacium Movis |
| Poison Projectile (5) | Aam Cetrius Taar |
| Paralyse (6) | Nhi Movis |
| Raise Dead (6) | Aam Morte Vitae |
| Lightning Strike (7) | Aam Folgora Taar |
| Fire Field (7) | Aam Yok Spacium |
| Explosion (8) | Aam Mega Morte |
| Mana Drain (8) | Stregum Movis |
| Incinerate (9) | Aam Mega Yok |
| Summon Creature (9) | Aam Vitae Tera |
| Mass Incinerate (10) | Mega Aam Mega Yok |
| Control (10) | Movis Comunicatum |
| Freeze Time (10) | Rhaa Tempus |

**Lesson:** the grammar is *verb (Aam create, Nhi negate, Mega raise, Rhaa lower) + element or noun + shape (Taar projectile, Spacium field)*. That is Ars Magica's grid in three runes, and opposite verbs give paired spells for free (Heal and Harm, Armor and Lower Armor).

### 4.4 Ars Magica, Magicka, DOS2/BG3, Spellbreak, Wizard of Legend, Dark Messiah, voice-casting [RECALLED unless noted]

- **Ars Magica (5th edition):**
  - 5 **Techniques** (Creo, Intellego, Muto, Perdo, Rego) × 10 **Forms** (Animal, Aquam, Auram, Corpus, Herbam, Ignem, Imaginem, Mentem, Terram, Vim).
  - Casting Total = Technique + Form + Stamina + aura (+ modifiers).
  - Formulaic spells succeed with no fatigue if CT ≥ level, and cost one fatigue level if CT ≥ level − 10.
  - Spontaneous magic: non-fatiguing CT/5, fatiguing (CT + stress die)/2.
  - Spell level = base effect + magnitudes for Range (Personal/Touch/Eye/Voice/Sight/Arcane Connection), Duration (Momentary/Concentration/Diameter/Sun/Ring/Moon/Year) and Target (Individual/Part/Group/Room/Structure/Boundary). Each magnitude is +5 levels, or +1 below level 5.
- **Magicka (2011):**
  - 8 elements: Water, Life, Shield, Cold, Lightning, Arcane, Earth, Fire. Up to 5 queued.
  - **Opposites cancel**: Life–Arcane, Water–Lightning, Cold–Fire, Earth–Lightning, Shield–Shield.
  - Combinations: Water + Fire = Steam; Water + Cold = Ice. Lightning cast while wet shocks the caster.
  - Cast modes: forward, area, self, weapon-imbue. Named "Magicks" are fixed element sequences.
- **Divinity: Original Sin 2:**
  - Physical and magic armor pools must be depleted before physical or magical crowd control can land [VERIFIED https://divinityoriginalsin2.wiki.fextralife.com/Combat].
  - Surfaces and clouds: fire, water, electrified water, ice, oil, poison, blood, electrified blood, steam and electrified steam. Each has *Blessed* and *Cursed* variants; blessing cleanses a cursed surface [RECALLED].
  - BG3's surface table is in [XREF R2 §3.5].
- **Spellbreak (2020, servers closed 2023):** dual elemental gauntlets (fire, frost, lightning, stone, toxic, wind) whose spells combine in the world, such as wind spreading fire or toxic gas exploding with fire.
- **Wizard of Legend (2018):** 4 equipped arcana slots (Basic, Dash, Standard, Signature). The Signature's empowered version charges through combos.
- **Dark Messiah (2006):** environment-plus-magic chains: kick into spikes, ice to make enemies slip, fire with oil.
- **Voice-casting (2025):** *Mage Arena* is a multiplayer PvP game where you **speak spell names** (for example "fireball") into proximity voice chat [RECALLED; title and mechanic unverified this session]. *In Verbis Virtus* (2015) is the earlier precedent [RECALLED]. "Mage Anyone?" could not be identified (see open questions).

---

## 5. Crafting, stations and quality

### 5.1 RimWorld: skill-based quality distribution [VERIFIED https://rimworldwiki.com/wiki/Quality]

**Tiers and multipliers** (Awful / Poor / Normal / Good / Excellent / Masterwork / Legendary):

| Stat | Multipliers |
|---|---|
| Market value | 0.5 / 0.75 / 1 / 1.25 / 1.5 / 2.5 / 5 |
| Melee damage and AP | 0.8 / 0.9 / 1 / 1.1 / 1.2 / 1.45 / 1.65 |
| Ranged accuracy | 0.8 / 0.9 / 1 / 1.1 / 1.2 / 1.35 / 1.5 |
| Ranged damage | 0.9 / 1 / 1 / 1 / 1 / 1.25 / 1.5 |
| Apparel protection | 0.6 / 0.8 / 1 / 1.15 / 1.3 / 1.45 / 1.8 |
| Deterioration | 2 / 1.5 / 1 / 0.8 / 0.6 / 0.3 / 0.1 |

**Probability by skill** (selected rows; the wiki gives all 0–20):

| Skill | Awful | Poor | Normal | Good | Excellent | Masterwork |
|---|---|---|---|---|---|---|
| 0 | 64.62 | 30.18 | 5.00 | 0.20 | 0 | 0 |
| 6 | 0.98 | 24.28 | 52.11 | 20.40 | 2.21 | 0.02 |
| 10 | 0.02 | 3.33 | 40.22 | 43.77 | 12.22 | 0.44 |
| 14 | 0 | 0.38 | 15.79 | 54.30 | 27.39 | 2.14 |
| 20 | 0 | 0.01 | 2.44 | 37.42 | 50.61 | 9.52 |

- Legendary cannot be crafted naturally. *Inspired Creativity* gives +2 tiers (no Awful or Poor, about 60% Legendary at skill 20). *Production Specialist* gives +1 tier. Both together give +3, with Good as the minimum.

**Generator model** [COMPUTED fit to the wiki table]:
- Model: `tier = clamp(floor(N(c_s, σ)), 0, 5)`, with Legendary folded into Masterwork.
- Centers: `c_s = [0.7, 1.1, 1.5, 1.8, 2.0, 2.2, 2.4, 2.6, 2.8, 2.95, 3.1, 3.25, 3.4, 3.5, 3.6, 3.7, 3.8, 3.9, 4.0, 4.1, 4.2]` for skill 0..20. This is the table I recall from decompiled `QualityUtility` [RECALLED].
- With **σ = 0.7**, the model reproduces the wiki within ≤ 5.3 percentage points in every cell. Free per-skill fits give centers that match the recalled table to within 0.06.
- My recollection of σ = 1.25 does **not** fit the wiki table. Treat 0.7 as the empirical value.

### 5.2 Dwarf Fortress item quality [VERIFIED https://dwarffortresswiki.org/index.php/Quality]

| Grade | Wrapper | Value × | Sharpness | To-hit / deflect × |
|---|---|---|---|---|
| Standard | name | 1 | 50% | 1.0 |
| Well-crafted | -name- | 1.1 | 60% | 1.2 |
| Finely-crafted | +name+ | 1.2 | 70% | 1.4 |
| Superior | \*name\* | 1.333 | 80% | 1.6 |
| Exceptional | ≡name≡ | 1.5 | 90% | 1.8 |
| Masterful | ☼name☼ | 2 | 100% | 2.0 |
| Artifact | unique name | 20 | 100% | 3.0 |

- The masterwork chance is capped at about 1/3, even at very high skill.
- Destroying, melting or stealing a masterwork gives its maker a bad thought, so a craftsman's pride is a social system.
- **Lesson:** quality affects to-hit and deflection, not just damage, and masterwork *events are recorded in legends*. PALIMPSEST can log masterworks to a run chronicle.

### 5.3 Minecraft: escalating reforge cost and enchanting [VERIFIED https://minecraft.wiki/w/Anvil_mechanics; https://minecraft.wiki/w/Enchanting_mechanics]

- **Anvil prior-work penalty** after c uses = `2^c − 1` (0, 1, 3, 7, 15, 31). When two items combine, the higher penalty carries forward.
- Operation cost = sum of both penalties, +1 to rename, +2 if the target is damaged, + Σ(enchant level × multiplier). The multiplier is per enchantment, with separate item and book values: Sharpness 1/1, Unbreaking 2/1, Fortune 4/2, Silk Touch 8/4.
- Survival refuses any operation costing over 39 levels ("Too Expensive!").
- Material repair restores 25% per unit. Combining two items adds the sacrifice's durability + 12%.
- **Enchanting table:**
  - `xpBase = 1 + randInt(0..7) + floor(min(15,b)/2) + randInt(0..min(15,b))`, where b is bookshelves (max 15).
  - Slots: top `max(1, floor(xpBase/3))`, middle `floor(2×xpBase/3) + 1`, bottom `max(xpBase, 2b)`.
  - Hidden enchant power = `round((xp + 1 + randInt(0..q) + randInt(0..q)) × (1 + (rand+rand−1)×0.15))`, with `q = floor(enchantability/4)`.
  - Weighted pick; then repeat with power halved each round. The continuation chance is keyed to power; the page's wording is garbled.

### 5.4 Valheim, Tarkov hideout, Kenshi research [RECALLED; wikis unreachable]

- **Valheim:**
  - Stations (Workbench, Forge, Black Forge, Galdr Table, Artisan Table) rise to level 5 when *station extensions* are placed within range.
  - Each item has quality levels 1–4 (some 1–3). Upgrading to quality n needs the station at level ≥ n.
  - A workbench needs a roof and has about a 20 m build radius.
- **Escape from Tarkov hideout:**
  - Module graph with levels 1–3 (some up to 4 or 6). Each level's prerequisites mix items, other modules, trader loyalty levels and skills.
  - Production crafts run in real time and keep going while offline. Fuel and filter consumables add upkeep.
- **Kenshi research:**
  - Research benches tier up. Projects cost Books, Ancient Science Books and AI cores and take time.
  - Weapon quality grades in ascending order: Rusting Junk → Rusted Blade → Mid-Grade Salvage → Old Refitted Blade → Refitted Blade → Catun No. 1/2/3 → Mk I/II/III → Edge Type 1/2/3 → Meitou (Meitou cannot be crafted).

### 5.5 Borderlands and Destiny 2

- **Borderlands 4** builds guns from parts: rarity sets the number of part slots, each manufacturer has an identity trait, and *licensed parts* graft one manufacturer's mechanic onto another's gun. Details in [XREF R2 §4.2–4.4].
- **Destiny 2 weapon crafting** [VERIFIED https://www.destinypedia.com/Weapon_crafting]:
  - **Deepsight** (red-border) drops build a **pattern**. Austringer needs 5 extractions; other weapons vary.
  - The craft is 7 steps: Frame (intrinsic) → Barrel → Magazine → Trait 1 → Trait 2 → optional Memento → Finalize.
  - The weapon **levels with kills and activity completions**. Perk options widen with level, and all perks and enhanced versions are available by **level 16**. **Level 7** enhances the intrinsic, and **level 20** raises its value.
  - Mementos give a tracker at level 1, a title at 20 and a shader at 30. Reshaping lets you swap perks later.
  - Edge of Fate (July 2025) made gear tier the main chase and stopped making new legendaries craftable [XREF R2 §6.3].

---

## 6. Identification and appraisal

### 6.1 NetHack [VERIFIED https://nethackwiki.com/wiki/Identification; https://nethackwiki.com/wiki/Price_identification]

**Scroll of identify, by BUC:**

| Scroll | 1 item | 2 items | 3 items | 4 items | Everything |
|---|---|---|---|---|---|
| Blessed | 1/5 | 1/5 | 1/5 | 1/5 | 1/5 |
| Uncursed | 21/25 | 1/25 | 1/25 | 1/25 | 1/25 |
| Cursed | always 1 | — | — | — | — |

- With positive Luck, a blessed scroll's "1" becomes 2.
- The spell of identify acts as uncursed at Unskilled or Basic and as blessed at Skilled or Expert.

**Price-ID:**
- Buy price = base × CHA factor (≤5 ×2; 6–7 ×1½; 8–10 ×1⅓; 11–15 ×1; 16–17 ×¾; 18 ×⅔; ≥19 ×½).
- A "sucker" surcharge of ×4/3 applies with a dunce cap, a visible shirt, or as a Tourist below XL 15.
- About 1 in 4 unidentified objects carries an extra ×4/3. This is fixed per object; merged stacks take the higher price, and split stacks keep it.
- Sell offer = ½ base (⅓ for suckers). About 1 in 4 shopkeepers pay ¼ less for unidentified items, fixed per shopkeeper. A shopkeeper with no gold offers 90%.
- Price groups and BUC generation rates are in [XREF R1 §6].

**Three knowledge levels:**
- **Formal:** the type gets its true name.
- **Informal with prompt:** the game asks you to name the type, for example after seeing a monster drink a potion.
- **Informal with no prompt:** the effect is recognizable, as with wand engraving or a unique price.

Per-object properties are BUC, erodeproofing, and enchantment or charges. Objects with different known properties **do not stack**.

### 6.2 Shattered Pixel Dungeon use-ID [VERIFIED-SRC SPD:items/weapon/Weapon.java, armor/Armor.java, wands/Wand.java, rings/Ring.java]

| Item | Uses to ID | Banked cap | Bank refill from hero XP (levelPercent = fraction of a level gained) |
|---|---|---|---|
| Weapon | `usesToID() = 20` hits dealt | 10 | `+ levelPercent × 20` (full refill in 0.5 levels); must be equipped (thrown weapons count) |
| Armor | 10 hits taken | 5 | `+ levelPercent × 10` |
| Wand | 10 zaps | 5 | `+ levelPercent × 5` (1 level) |
| Ring | 1 hero level gained while equipped | — | `levelsToID −= levelPercent` |

- Each use consumes `min(available, Talent.itemIDSpeedFactor)` from both the counter and the bank.
- Generation of weapons and armor: +0 75%, +1 20%, +2 5%. Then **30% cursed** (with a curse enchant or glyph), else 10% enchanted (weapons) or 15% glyphed (armor). The enchant and curse roll uses a pushed side-RNG so item variance never shifts level generation [VERIFIED-SRC Weapon.java/Armor.java `random()`].

### 6.3 Other appraisal models

- **Barony 5.0:** the base type is always visible and only magic properties are hidden. Appraisal is a timed skill [XREF R1 §1.8].
- **Caves of Qud:** INT-based artifact examination with complexity and the risk of breaking the item [XREF R1 §5.8].
- **Darkest Dungeon curios:** the right tool turns a gamble into a known result (§3.6).
- **Diablo 1/2** [RECALLED]:
  - Magic, rare, set and unique items drop unidentified. The base type and rarity color stay visible.
  - Identify with a Scroll or Tome of Identify (the tome holds up to 20 scrolls), or with Deckard Cain: for a gold fee in Diablo 1, free in Diablo 2 after his rescue.
  - Unidentified items sell for less.
- **Path of Exile** [RECALLED]:
  - Scrolls of Wisdom identify. Unidentified rares count double in the chaos vendor recipe.
  - Corrupting an unidentified item hides the outcome until it is identified.
  - The "identify or vendor blind" choice is a light gamble with a deterministic floor.
- **Wizardry I and Elminage** [RECALLED]:
  - Only the Bishop can identify in the dungeon. Success chance rises with level, roughly 10% + 5% per level.
  - A failure can curse the Bishop with the item.
  - This is the "appraisal has a price" model.

---
## 7. Damage, armor, poise and hit locations

### 7.1 Five armor models

| Model | Exemplar | Exact rule | Behavior |
|---|---|---|---|
| Flat threshold, then percent | Fallout 2 | Per bullet: `dmg = (roll + bonus) × (critMult×ammoMult) / ammoDiv / 2 × difficulty% / 100 − DT; if > 0: dmg −= dmg × DR / 100`. DR includes the ammo's DR modifier and is clamped to 0..100. Armor-bypassing hits use DT and DR at 20% [VERIFIED-SRC F2CE:combat.cc `attackComputeDamage`]. | DT erases small hits; ammo types read clearly (AP ammo = lower DR). The `/2` normalizes a crit multiplier stored in halves [RECALLED reading]. |
| Percent with a cap | Oblivion | `× (100 − AR)/100`, AR ≤ 85 [XREF R2 §2.6] | Simple; stacking hits the cap quickly. |
| Ratio, so hit size matters | Morrowind; CANON §10.5 | Morrowind: `dmg × max(fCombatArmorMinMult 0.25, dmg/(dmg + AR))` (GMST 0.25 [VERIFIED-SRC OMW:defaultgmsts.cpp]; formula [XREF R2 §1.6]). AR = Cuirass×0.3 + (Shield + Helm + Greaves + Boots + both Pauldrons)×0.1 + (both Gauntlets)×0.05 [VERIFIED-SRC OMW:npc.cpp]. | Strong against chip damage, weak against big hits. CANON's `AR/(AR + 4D)` capped at 85% is the same family, with AR effectively ÷4 and a 15% floor. |
| Ratio, PoE family | Path of Exile 1 | `DR = A/(A + 5D)`, cap 90% [RECALLED] | Same shape as CANON. |
| Dice penetration | Caves of Qud | Hit: `1d20 + AGI mod + weapon bonus > DV`. A natural 20 crits (19 with the masterwork mod), always hits and raises the STR-bonus cap by 1. Penetration: 3 rolls of an exploding d10 ranging −1..8 against `5 − PV + AV`. All three fail → 0 penetrations. Mixed → 1. All three succeed → 1 and repeat at PV − 2. Damage dice are rolled once per penetration. Vibro weapons use PV = AV + 4. Only a *player* crit guarantees one penetration [VERIFIED https://wiki.cavesofqud.com/wiki/Melee_Combat]. | Great on a turn grid; too much variance per bullet at 600 RPM. |
| Class threshold plus durability | Escape from Tarkov | [RECALLED, community-reverse-engineered]: armor class 1–6 (resistance = class × 10); penetration chance depends on bullet PEN against the class, scaled by durability %; non-penetrating hits deal blunt damage; armor loses durability = PEN × the ammo's armor-damage % × the material's destructibility (aramid 0.25 … ceramic 0.8). Since the 0.14 patch (Dec 2023), plate inserts have separate zones. | Hard gates; works because ammo is a deep economy. |

### 7.2 Hit locations

- **Fallout 2 aimed-shot penalties** [VERIFIED-SRC F2CE:combat.cc `hit_location_penalty_default` + combat_defs.h order]: Head −40, Left/Right Arm −30, Torso 0, Right/Left Leg −20, **Eyes −60**, Groin −30, Uncalled 0. Melee applies **half** the penalty.
- **Rest of the Fallout 2 to-hit pipeline** [VERIFIED-SRC F2CE:combat.cc]:
  - Ranged distance term: `dist − m×(PE − 2)`, with m = 2, or 4 for Long Range, or 5 for Scope (scoped weapons get +8 penalty under 8 hexes). Floored at −2×PE, then ×(−4), or ×(−12) when blinded.
  - −10 per creature in the line of fire.
  - −20 per point of Strength short of the weapon's requirement.
  - Accurate perk +20. Target AC subtracted. +15 against multi-hex targets.
  - The light penalties of §2.3.
  - Enemies take −20 on Easy and +20 on Hard.
  - Capped at 95.
- **VATS** (Fallout 3/4) shows per-limb hit chances from distance, cover and Perception. Crippling a limb is a separate per-limb HP pool [RECALLED].
- **Dwarf Fortress:**
  - Bodies are layered tissues: skin, fat, muscle, bone and organs.
  - Edged attacks cut layer by layer while momentum lasts; blunt attacks transmit force.
  - Contact area and material properties (shear and impact yield) decide the outcome.
  - [RECALLED; too expensive to simulate per bullet in real time, so use it only for designed boss weak points.]
- **ARMA 3:**
  - Per-selection hit points (head, face, neck, chest, diaphragm, abdomen, pelvis, arms, hands, legs). Each selection has an `armor` value and a `passThrough` fraction that sends damage to global HP, plus explosion shielding.
  - ACE3 medical adds wounds, bleeding and tourniquets [RECALLED].
  - CANON §9's `share`/`hit_mult` per part is the same shape.

### 7.3 Poise and stagger [VERIFIED https://eldenring.wiki.fextralife.com/Poise]

**Player:**
- Displayed poise is effectively poise HP, and toughness = poise/10.
- At 0 you stagger and poise refills to max; overflow damage is discarded.
- Poise **fully resets 30 s after the last hit**.
- Bull-Goat's Talisman reduces poise damage taken by 25%. Baldachin's Blessing 34%, Radiant Baldachin's 45%.

**Enemies:** stance regenerates after **6 s (80 poise) to 15 s (200 poise)** without hits. Breaking stance opens a critical.

[COMPUTED] Interpolating those two points: `t_regen = 6 + 0.075 × (P − 80)` s.

PvP stance damage per hit: Dagger 40.5, Straight Sword 110, Greatsword 228.9, Colossal Sword 504.

### 7.4 Elemental multipliers

- **Borderlands 4:** each primary element is strong against one health layer and weak against the other two, and the spread widens with difficulty. On Normal, Fire hits Flesh ×1.5 and Shields and Armor ×0.75, Shock is strong against Shields, and Corrosive against Armor. Radiation and Cryo are neutral on Flesh [XREF R2 §4.4].
- **Destiny 2** [RECALLED]:
  - An element matching a shield breaks it faster, and Champions need specific counters (Barrier, Overload, Unstoppable).
  - Subclass verbs such as scorch→ignite, jolt, freeze→shatter and suspend are status ladders, not damage multipliers.
- **Lesson:** match multipliers to a *health layer* or to *status ladders*, not to a 13×N matrix. CANON's resistance table plus statuses already does this.

---

## 8. Cross-system formula index

| Need | Best shipped formula | Tag | PALIMPSEST owner |
|---|---|---|---|
| Use-based skill progress | Morrowind `(1 + s) × type × spec` | VERIFIED-SRC | 04 |
| Use-trained attribute ceiling | DF `max(2 × start, start + median)` | VERIFIED | 04 |
| Two-stat skill seeding | Fallout `default + k × (A + B)` | VERIFIED-SRC | 04 |
| Escalating point cost by band | Fallout 1/2/3/4/5/6 per +1 at 100/125/150/175/200 | VERIFIED-SRC | 15 meta |
| Sight detection with hysteresis | Oblivion sound + sight + skill, thresholds 0/−20/−50 | XREF R2 | 09 |
| Facing and distance stealth | Morrowind ×1.5 front / ×0.5 behind; `0.5 + 0.002 × dist` | VERIFIED-SRC | 09 |
| Light → accuracy | Fallout −40/−25/−10 at ≤ 40/60/80% light | VERIFIED-SRC | 05, 09 |
| Noise legibility | Mark of the Ninja: rings at exact radius | SNIPPET | 09, 17 |
| Potency from skill | Morrowind `f × Q / (3 × BaseCost)` mag; `f × Q / BaseCost` duration | VERIFIED-SRC | 08 |
| Field brewing = ID | SPD 3-seed rule: 1 type IDs, 2 types 25%, 3 types 50% random | VERIFIED-SRC | 08, 10 |
| Reaction data schema | Noita `<Reaction>` fields | VERIFIED | 08, 03 |
| Liquid volumes and depth | Qud drams; 200 / 2000 bands | VERIFIED | 08, 03 |
| Programmable implement | Noita deck/hand/discard; `max(delay, recharge)` | VERIFIED | 06 |
| Effect cost | Morrowind `(min + max)(dur + 1)B/40 + aB/40`, ×1.5 target; Oblivion `Mag^1.28` | VERIFIED-SRC / XREF | 06 |
| Gesture input | Arx: 20 points, 8 directions, tolerance 2 | VERIFIED-SRC | 06, 17 |
| Craft quality roll | RimWorld `floor(N(c_s, 0.7))` | COMPUTED | 07 |
| Anti-reroll escalation | Minecraft `2^n − 1` prior-work penalty | VERIFIED | 07, 11 |
| Identify scroll odds | NetHack 21/25 vs 1/5 by BUC | VERIFIED | 10 |
| Use-count ID | SPD 20/10/10 uses + 1 level, bank refilled only by XP | VERIFIED-SRC | 10 |
| Price-ID | NetHack CHA table + 1-in-4 hidden surcharge | VERIFIED | 10 |
| Armor | Ratio family (Morrowind/PoE/CANON) | VERIFIED-SRC / CANON | 05 |
| Stance regeneration | Elden Ring `6 + 0.075 × (P − 80)` s | COMPUTED | 05 |
| Called-shot penalties | Fallout head −40, eyes −60, limbs −30/−20, melee ½ | VERIFIED-SRC | 05 |

---
## 9. Design implications for PALIMPSEST

These are opinionated and written in CANON units: Bp, mHP, 60 Hz ticks, perception at 20 Hz, `Light` 0..15 and `Db`. "Steal" means port as written. "Amend" means a proposed change to `00_CANON.md`. Each item names its owning document.

### 9.1 Attributes (doc 04)

1. **Amend: give `sub.analysis` (ANL) a unique lever.** In CANON §6.4 every sub-attribute has at least one row except ANL, which only governs Alchemy and Medicine. Following Qud's one-lever-per-attribute rule (§1.6), add two rows:
   - **Identification and appraisal speed** `× (10000 + d(ANL) × 100)` Bp, clamped to 5000..20000.
   - **Alchemy potency term** `floor_div(ANL, 5)` (§9.3).
2. **Amend §7.1: a talent ceiling, stolen from Dwarf Fortress.** A base sub-attribute can never be trained above `min(100, max(2 × start, start + 40))`, where 40 is the human baseline used as the median. Starts of 20, 40 and 60 cap at 60, 80 and 100. Without this, use-training levels everyone to 100.
3. **Steal DF's 7-point range sets for enemy templates.** `range: [min, a, b, median, c, d, max]`. Pick one of the six segments with probability 1/6 each, then draw uniformly inside it with keyed `rng.spawn`. Every goblin gets distinct sub-attributes, which feed CANON §6.5's NPC rows. Default set relative to median m: `[m−25, m−10, m−5, m, m+5, m+15, m+40]`.
4. **Keep a single governing sub per skill** (CANON §7). Fallout-style two-stat coupling (§1.3) already lives in the derived rows, and a second governing stat would muddy the UI.
5. **Keep mastery gates at 25/50/75/100.** As with PF2e ranks, perks at the gates are what players remember (§1.5).

### 9.2 Stealth and awareness (doc 09)

6. **Awareness is a per-pair meter** `A(o, e) ∈ 0..=10000`, updated on the observer's perception tick (period 3). States and thresholds:
   - UNAWARE at A < 2500, but a SUSPICIOUS observer only drops back below 1000 (hysteresis).
   - SUSPICIOUS at ≥ 2500: turns toward the stimulus and barks.
   - INVESTIGATING at ≥ 6000 or on an audible noise more than 15 dB over threshold. Walks to the last-known position and **does not attack** (SPD v4).
   - ALERT at 10000: combat.
   - SEARCHING for 20 s after losing line of sight, then RETURNING.
   - After an incident, **Vigilance** (MGS Caution) lasts 90 s and multiplies every gain by 12500.
   - Decay after 2 s without a stimulus: −1500 Bp/s while UNAWARE or SUSPICIOUS, −500 Bp/s while INVESTIGATING or SEARCHING. ALERT does not decay while there is line of sight.
7. **Sight gain per tick** is a `mul_bp` chain applied to the zone rate ÷ 20 (Shadow Tactics zones and Oblivion distance factor):

   | Zone | Geometry | Rate (Bp/s) | Posture rule |
   |---|---|---|---|
   | Near | ≤ 6 m × (1 + row 32), half-angle 35° | 40000 | any posture is seen |
   | Far | ≤ 24 m × (1 + row 32), half-angle 55° | 8000 × `(1 − d/R_far)` | crouch 5500, prone 3000; **both are 0 if `light_at(e)` ≤ 7** |
   | Peripheral | 55°–100° | 2000 | only if moving at walk or faster |
   | Behind | > 100° | 0 | hearing only |

   - Further factors:
     - `light_bp[L]` for L = 0..15: `[800, 1000, 1300, 1700, 2200, 2800, 3500, 4300, 5100, 5900, 6700, 7500, 8200, 8900, 9500, 10000]`.
     - motion: still 6000, walk 10000, run 13000, sprint 16000.
     - visible fraction: rays to head, torso and feet give 0, 3333, 6667 or 10000.
     - `st.invisible` 1000 (CANON).
   - The **light gem** is `light_bp × posture × motion` shown 0–100, which follows Thief's lesson that hidden state must be visible.
8. **Hearing** is event-driven on `ev.noise`:
   - Received level: `L_rx = L_src − 20·log10(max(d_m, 1)) − 0.25·d_m − occlusion`. Occlusion per crossing: solid block 25 dB, closed door 15 dB, open doorway 3 dB, liquid 10 dB.
   - Audible iff `L_rx ≥ max(25 − row 33, ambient_dB + 3)`. Noise under ambient is inaudible, as in Splinter Cell.
   - Gain = `clamp((L_rx − threshold) × 400, 500, 6000)` Bp, applied instantly. It records the last-known *position* but not the identity of the source.
   - Footsteps: crouch 25, walk 40, run 55, sprint 65 dB; landing 55 + 5 per metre fallen.
   - Surface modifiers: metal +10, stone +6, wood +3, dirt 0, grass and moss −6, carpet and snow −8, shallow water +8. Armor: heavy +8, medium +4, light +0.
   - Other sources: callout 85, door 45, melee hit 70, unsuppressed gun 140, suppressed 115, spell 60 (each genre-skinned). Room ambient: crypt 20, wind 35, rain 45, machinery 60, club or foundry 75.
9. **Mark of the Ninja rule:** draw every player-made noise as a ring at the exact free-field radius where `L_rx` = 25 dB. Solve numerically and cache per dB. Walking on stone (46 dB) gives about 8.7 m; sprinting on metal (75 dB) about 58 m; crouching on grass gives under 1 m.
10. **Propagation:** reaching ALERT emits an 85 dB callout carrying the target's position. Allies who hear it set `A = max(A, 7000)` and receive the last-known position. Seeing a downed or dead faction member starts Vigilance for the whole faction in that region and sends the observer INVESTIGATING (Hitman, Shadow Tactics).
11. **Sneak attacks** need `st.hidden`, or victim A < 6000 toward the attacker. Multipliers: melee ×3 (×4 at Sneak 50, ×5 at Sneak 100), ranged ×2 (×2.5 at Sneak 75), spells ×1.5. They never roll a crit (CANON step 2.3–2.4), which is Oblivion's `max()` lesson. This is the Morrowind/Skyrim lineage with tighter caps for co-op.
12. **Disguises (Hitman):** an NPC template lists `aware.enforcer_of: [disguise ids]`. Enforcers accrue A at the zone rate × row 40. Everyone else accrues only on an **illegal action**: a visible weapon, trespassing, lockpicking or climbing in view gives +6000 at once.
13. **Darkness also defends in gunfights** (Fallout §2.3). An NPC's spread at the player is multiplied by 18000 at target Light ≤ 5, 14000 at 6–9 and 11500 at 10–12.

### 9.3 Alchemy and liquids (doc 08)

14. **Potency, Morrowind-shaped, with no random failure.** In co-op, random failure that destroys ingredients feels like theft.
    - `f = skill.alchemy + floor_div(ANL, 5) + floor_div(FAT, 10)`.
    - Magnitude Bp = `mul_bp(base_bp, 5000 + 50 × f)` × mortar quality.
    - Mortar qualities 7000, 10000, 12000, 15000, 20000, which compress Morrowind's 0.5/1.0/1.2/1.5/2.0.
    - Duration has its own axis. Retort adds to beneficial effects, alembic divides harmful side effects, and calcinator adds to both, **including poisons** (OMW `applyTools`, §3.1).
    - If `f` is below the recipe's difficulty, the brew comes out one quality tier lower instead of failing.
15. **Ingredients carry 4 hidden effects.** Skill 0/25/50/75 reveals 1/2/3/4 (Oblivion ranks). **Hidden effects still combine** and are revealed by brewing (Morrowind). A potion carries every effect shared by at least 2 ingredients (Skyrim). Discovery is shared by the whole party.
16. **Steal SPD field brewing exactly** for the in-run "pot" in every genre (cauldron, autoclave, still):
    - 3 ingredients. 1 type gives a guaranteed result **and identifies that appearance class for the party**. 2 types give 25% random; 3 types give 50% random.
    - Healing results reroll with probability `count/10`.
    - This makes alchemy an identification tool (doc 10).
17. **Five universal modifier reagents (Minecraft), genre-skinned:**
    - Extend: duration × 16000.
    - Amplify: +1 magnitude tier, duration × 5000. Cannot combine with Extend.
    - Invert: beneficial becomes its harmful twin.
    - Aerosol: becomes a gas cloud in the gas cellular automaton.
    - Linger: becomes a surface, duration × 2500.
    - One of each per potion.
18. **Reactions use Noita's schema verbatim** as the `alch.reaction` content type: `in1, in2, in3?, out1, out2, out3?, probability_bp` (per liquids step at 10 Hz), `fast, convert_all, req_lifetime_ticks, blob_radius1/2, entity, sfx`.
    - Each run seeds **2 secret 3-material recipes**, in the style of Lively Concoction and Alchemic Precursor, from a genre-weighted pool. Lore texts hint at them.
    - Liquid depth uses Qud's bands as cell fill: < 20% is a puddle (slip and stick saves, evaporation), 20–99% is wading, 100% is swimming.
    - Mixtures are named by their top-2 components ("bloody brine").
19. **Witcher refill, limited:** up to 3 **Signature** recipes refill at a Folio rest point by spending 1 solvent each. Everything else is consumed. `res.toxicity` (CANON) is the brake, with potions at 15,000–30,000 milli each.

### 9.4 Spellcrafting (doc 06)

20. **The casting implement is a Noita wand, ported 1:1.** Noita runs at 60 frames/s, the same as CANON's tick rate, so its timings transfer directly.
    - Fields: `cast_delay_ticks, recharge_ticks, capacity, spells_per_cast, shuffle, spread_mdeg, speed_bp, always_cast[]`.
    - Deck/Hand/Discard with depth-first draw, wrap and reload exactly as in §4.1. The wait is `max(cast_delay, recharge)`.
    - Trigger and timer payloads carry the next glyph, and their aether is charged up front.
    - CANON row 26 (complexity budget) caps glyphs per sentence. Row 27 (grimoire slots) caps how many sentences can be inscribed at once.
21. **Glyph grammar from Arx and Ars Magica.**
    - Verb: Create, Negate, Raise, Lower, Perceive, Control.
    - Form: CANON damage types plus matter (water, stone, flesh, mind, light, metal).
    - Shape: projectile, touch, self, field, beam, cone.
    - Modifier: multicast N, trigger, timer, homing, spread.
    - Seal: always-cast.
    - Opposite verbs give paired spells for free (Heal and Harm). Fire↔frost and radiant↔umbral **cancel** within a sentence (Magicka). Other mixed Forms become type shares summing to 10000 (CANON §10.5).
22. **Aether cost is Oblivion's power law** with Morrowind's target surcharge: `cost = base × max(1, mag)^1.28 × max(1, dur_s) × (1 + 0.15 × area_m) × (15000 if aimed at a target)`, then × `(12500 − 50 × school_skill)` Bp (×1.25 at skill 0, ×0.75 at 100). There is no cast-failure roll. Real-time failure feels like input lag.
23. **Three input modes map to the same glyph IDs:** hotbar; **traced gestures** (Arx: 20-point resample, 8 directions, tolerance 2), which grant cast time × 7500 when traced cleanly; and **voice keywords** through on-device speech recognition (Mage Arena), which gives no numeric bonus, so speech is never mandatory.

### 9.5 Crafting (doc 07)

24. **Quality roll uses RimWorld's distribution with deterministic integers.**
    - Map skill 0..100 to `s = skill/5`, and interpolate the center `c(s)` from §5.1's table (0.7 to 4.2) in milli.
    - `z_milli = Σ_{i=1..12} rng.craft.range(1000) − 5994` (Irwin–Hall: mean 12 × 499.5 = 5994, σ = 1000.0).
    - `v = c + station_bonus + material_bonus + mul(z, 700)/1000`.
    - Tier = `clamp(floor(v/1000), 0, 5)` over `q.awful..q.masterwork`.
    - `q.peerless` comes only from Inspiration (+2 tiers) or a Specialist station (+1).
25. **Station tiers (Valheim).** Level 1..5, raised by placing extensions within 8 m. Upgrading an item to quality step n needs station level ≥ n.
26. **Anti-reroll escalation (Minecraft).** The n-th affix reroll on one item costs `base × 2^n`, and the 7th is refused ("the page will not take more ink"). This is the crafting-side guardrail that keeps loot-box-like rerolling honest.
27. **Masterwork chronicle (DF).** Every masterwork or Peerless craft logs the crafter, Folio and material to the run chronicle and the score screen.
28. **Patterns as meta, not power (Destiny Deepsight).** Five "Deepsight" extractions of one archetype unlock its pattern in the hub as a crafting *option* (skin, perk pool access), never a stat bonus.

### 9.6 Identification (doc 10)

29. **Formal ID scrolls use NetHack's odds exactly.**
    - Uncursed: 1 item 84%, 2/3/4/all 4% each.
    - Blessed: 20% each.
    - Cursed: always 1 item.
    - The spell version counts as blessed at skill 50.
30. **Item identification uses SPD's use-count exactly:** weapons 20 hits dealt, armor 10 hits taken, implements 10 casts, rings, amulets and implants 1 character level. The bank holds half the requirement and refills **only from XP**, so ID cannot be farmed on a wall.
31. **Curios, with the right tool from Darkest Dungeon.** Each Folio places 2–4 curios with roll tables. A genre-matched provision gives a fixed outcome, and **1 in 4 tool–curio pairs has a known downside**, so the table never becomes a solved lookup.
32. **Price-ID stays automatic, with no hidden per-object surcharge.** NetHack's 1-in-4 random surcharge adds noise without deduction in co-op. Use CANON rows 37–38 and the R1 Appraisal Ledger.

### 9.7 Damage, poise, hit locations (doc 05)

33. **Keep CANON's ratio armor and reject Qud dice for kinetic damage.** OpenMW's code confirms that the ratio family (Morrowind, PoE, CANON) behaves well. Qud's per-hit penetration dice suit a turn grid, but at 600 rounds per minute they turn into noise.
34. **Poise.** Errata poise resets 8 s after the last hit; Elden Ring's 30 s is tuned for a slower game. NPC stance regenerates after `clamp(6 + 0.075 × (P − 80), 4, 20)` s (§7.3). Bull-Goat-style affixes cap at −45% poise damage taken.
35. **Called shots (an optional Focus mode, VATS-like).** Fallout's table becomes aim-assist and Focus-cost multipliers: head 14000, eyes or weakpoint 16000, groin and arms 13000, legs 12000, torso 10000. Melee uses half the surcharge.

**Avoid:**
- Morrowind's light-blind stealth.
- Skyrim's Fortify-Alchemy and Restoration feedback loops (crafted buffs must never affect crafting).
- Random brew or cast failure.
- NetHack's hidden per-object price surcharge.
- Oblivion's player-level enemy scaling (R2).
- Kenshi's self-harm training: Toughness XP only from hostile-sourced damage.
- Tarkov's hard class gates in a cross-genre game, because pen and class tables do not translate across genre skins.

---

### Open questions (not verifiable this session)

- Path of Exile 1's current armour formula (`5D` or `10D`) and Path of Exile 2's per-attribute bonuses: poewiki is blocked by Anubis.
- Skyrim's alchemy and detection GMST defaults: UESP is blocked by Cloudflare.
- Tarkov's exact penetration-chance curve and material destructibility values after 2024 patches.
- Kenshi's XP curve constants and training-dummy caps.
- Mage Arena's release date and spell list; what "Mage Anyone?" refers to.
- RimWorld's true σ: the decompiled value I recall (1.25) does not fit the wiki table, and 0.7 does.
- Thief's Dark Engine visibility formula: no primary source was reached.
