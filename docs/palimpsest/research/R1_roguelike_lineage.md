# R1 — Roguelike Lineage Dossier (Barony, Shattered PD, Experienced PD, IVAN, Caves of Qud, NetHack-ID)

As of 2026-10-11. Research input for PALIMPSEST (omni-genre 3D co-op roguelike crawler).

## Table of contents

0. Method, source aliases, evidence rules
1. Barony (Turning Wheel) — first-person real-time co-op roguelike
2. Shattered Pixel Dungeon (SPD) — the identification/upgrade/run-structure reference
3. Experienced Pixel Dungeon (ExpPD) — the "endless, numbers-go-up" fork
4. IVAN — materials, limbs, use-trained attributes, gods
5. Caves of Qud (CoQ) — attributes, penetration combat, mutations, cybernetics, liquids, tinkering, factions, history
6. NetHack identification (BUC, price-ID, engrave-ID, scroll of identify)
7. Cross-game comparison tables
8. Design implications for PALIMPSEST

---

## 0. Method, source aliases, evidence rules

Four of the five main games (Barony, SPD, ExpPD, IVAN) are open source, so most numbers below come straight from source code cloned this session, not from wikis. Code is the ground truth for those games. Wikis are used for Caves of Qud (closed source), for NetHack ID, and to cross-check Barony data that lives in non-repo JSON assets.

| Alias | Expands to (prefix URL) | Snapshot |
|---|---|---|
| `SPD-src/` | https://github.com/00-Evan/shattered-pixel-dungeon/blob/57a4e06a4caf162446d1c28caa7983f0493fecf0/core/src/main/java/com/shatteredpixel/shatteredpixeldungeon/ | v4.0.2, commit 2026-10-06 |
| `EXPD-src/` | https://github.com/TrashboxBobylev/Experienced-Pixel-Dungeon-Redone/blob/7a71df6a56b371a6b0efffcbe6210dbcfaf8611d/core/src/main/java/com/shatteredpixel/shatteredpixeldungeon/ | ExpPD-2.19.0, commit 2024-09-29 |
| `BAR-src/` | https://github.com/TurningWheel/Barony/blob/962a5ce36d10207beef7d8673876e0cebf8e76e4/ | post-5.0.2, commit 2026-02-20 |
| `IVAN-src/` | https://github.com/Attnam/ivan/blob/625767d1675073a7ecebf884a1ab0fd1d9c53baf/ | master, commit 2026-09-09 |
| `BAR-wiki/` | https://barony.wiki.gg/wiki/ | fetched 2026-10-11 |
| `COQ-wiki/` | https://wiki.cavesofqud.com/wiki/ | fetched 2026-10-11 |
| `NH-wiki/` | https://nethackwiki.com/wiki/ | fetched 2026-10-11 |

Tags: **[VERIFIED x]** = fetched or read this session (x = alias + path or full URL). **[RECALLED]** = from memory, not checked this session. Formulas copied from code are given in code font exactly as written (variable names preserved) so an implementing agent can port them literally.

---

## 1. Barony

### 1.1 State of the game in 2026

| Version | Name | Date | Key content |
|---|---|---|---|
| 4.1.0 | Life After Death | GitHub tag 20 Oct (year not shown; 2023 RECALLED) | Co-op ghosts (dead players respawn as ghosts), callout system [VERIFIED https://www.perfectly-nintendo.com/barony-switch-all-the-updates] |
| 4.2.0 | Eat My Hat | v4.2.1 tag 04 Sep (year not shown; 2024 RECALLED) | Hats/masks, Mimics, **custom seeds**, item icon redraw, skill modifiers from items [VERIFIED same] |
| 4.3.0 | Keep Your Secrets | 2024-10-31 (console), GitHub tag 12 Dec | **Compendium** (in-game encyclopedia unlocked with Lore Points from achievements), 5 slime colours, Bugbear, Gnome Thief, Bells, Daedalus Shrines, max character level raised to **255** (stats capped **248**), starvation now **5% max HP per tick** [VERIFIED same; https://github.com/TurningWheel/Barony/releases] |
| 4.3.1 | Assist Shrine | GitHub tag 07 Feb (year not shown; 2025 inferred from 4.3.0 date) | Per-player handicap shrine on floor 0 (see 1.10) [VERIFIED same] |
| 5.0.0 | Instruments of Destruction, Part One | ~2026-01-30 (PC; press published Jan 30 2026); Switch Apr 3–4 2026 | Magic rewrite into **Sorcery / Mysticism / Thaumaturgy**, ~80 new spells, Appraisal renamed **Lore**, Swimming removed, key-gated treasure rooms, Tablets, multiplicative resistances, new XP sharing [VERIFIED https://ingamenews.com/2026/01/barony-just-got-massive-facelift-and.html; https://www.perfectly-nintendo.com/barony-switch-all-the-updates] |
| 5.0.1 / 5.0.2 | hotfixes | 2026-02-18 | e.g. "Backstabs no longer work on rooted targets", Scion spell recharge cut to 5%/tier [VERIFIED https://github.com/TurningWheel/Barony/releases] |
| DLC 3 | Deserters and Disciples | with 5.0 | 5 monster races/classes: Gremlin **Sapper**, Myconid **Hermit**, Dryad **Scion**, Salamander **Paladin**, Gnome **Bard**; non-owners can join sessions [VERIFIED https://ingamenews.com/2026/02/finally-barony-50-delivers-overhaul.html] |

"Part Two" (new biomes and bosses) is announced but undated [VERIFIED ingamenews 2026/02].

### 1.2 Simulation and multiplayer model

- Real-time, first-person, **50 ticks per second** (`#define TICKS_PER_SECOND 50`) [VERIFIED BAR-src/src/game.hpp].
- `MAXPLAYERS` is **4**, or **8** when compiled with `BARONY_SUPER_MULTIPLAYER` [VERIFIED BAR-src/src/main.hpp]. Online (Steam + Epic Online Services, `eos.cpp` exists in tree), LAN, and local splitscreen [VERIFIED BAR-src/src/eos.cpp exists; crossplay details RECALLED].
- 5.0 XP sharing: party members each receive **90% (2p), 80% (3p), 70% (4p)** of kill XP, constant regardless of deaths; base kill XP **10–19**, reduced when far out-levelling the victim; allied-NPC kills give 0 [VERIFIED perfectly-nintendo 5.0 notes].
- Friendly fire (5.0) only applies to the player's outgoing damage; ally NPCs path through and shoot past players [VERIFIED same].
- Dead co-op players become **ghosts** (4.1) [VERIFIED same].
- Hunger scales with party size: 3 players ×0.8 loss rate, 4 players ×0.67 [VERIFIED BAR-wiki/Hunger].

### 1.3 The six stats (5.0 values)

| Stat | Per-point effects (5.0) | Max HP / MP per point |
|---|---|---|
| STR | +5.5% melee damage; better speed under heavy load; slight HP regen | +1 HP |
| DEX | +5.5% ranged damage; movement speed (esp. light load); spell cast speed; significant HP regen | +2 HP |
| CON | +1 natural AC; shorter debuffs; healing potions +2/pt (extra healing +4/pt); Thaumaturgy PWR +1% | +2 HP |
| INT | Sorcery PWR +2%; Mysticism/Thaumaturgy PWR +1%; magic resistance +1%; restore-magic potions +2/pt (cap INT 15) | +2 MP |
| PER | +1% arrow pierce chance and +1% pierce damage; Lore and Tinkering +1 each; explosive trap damage; touch-spell reach; slight MP regen | +2 MP |
| CHR | HP/MP restored on level-up and floor descent; sell prices; Mysticism PWR +1%; Leadership +1; charm success; significant MP regen | +1 MP |

[VERIFIED BAR-wiki/Character_Attributes_(Stats)]. Note that the in-source lookup for appraisal uses `perStatMult = 3` (PER×3 added to Lore) [VERIFIED BAR-src/src/interface/identify_and_appraise.cpp], which matches the 5.0 patch note "PER gives +3 Lore per point (previously +5)" [VERIFIED perfectly-nintendo].

Derived metastats: **AC** = CON + gear; each AC point removes 1 physical damage, **capped at 75% of the hit**; while blocking AC can remove up to **100%**. **RES** reduces magic damage by %, magic ignores AC. **PWR** scales spell damage/healing/duration by %. Weight slows movement past a STR-based threshold [VERIFIED BAR-src/lang/compendium_lang/lang_codex.json; AC base `int armor = statGetCON(stat, playerEntity);` VERIFIED BAR-src/src/entity.cpp]. 5.0 changed resistances to stack **multiplicatively** (two +30% magic-resist items = 51%) [VERIFIED perfectly-nintendo].

### 1.4 Levelling

- **100 XP per level**; every skill increase grants **+2 XP**; kills give XP by challenge [VERIFIED BAR-src lang_codex.json "xp"].
- On level-up: 3 permanent stat points. Point 1 = uniformly random stat; point 2 = class-weighted, not the same as point 1; point 3 = class-weighted, not already raised; then a **1-in-5** chance of +1 extra in a raised stat if a skill governed by it was trained [VERIFIED BAR-wiki/Character_Attributes_(Stats)].
- 5.0 HP/MP growth: class base **+2..+4 HP** and **+1..+5 MP** per level plus stat terms (STR +1 HP; DEX/CON +2 HP; CHR +1 MP; INT/PER +2 MP). On level-up current HP/MP refill by the max increase plus CHR. Descending a floor restores HP/MP equal to CHR, capped at 50% of max. Base regen **1 point / 12 s** for HP and MP, scaling with level and class [VERIFIED perfectly-nintendo 5.0].
- Level cap 255, stat cap 248 (4.3.0) [VERIFIED perfectly-nintendo].

### 1.5 Skills (use-based, 0–100)

Tiers: Novice 1–19, Basic 20–39, Skilled 40–59, Expert 60–79, Master 80–99, **Legendary 100** (each skill has a unique legendary capstone). A skill rises only when the action **succeeds in its purpose** (e.g. poison bolt must damage) [VERIFIED BAR-wiki/Skills].

| Skill (stat) | Key effects | Known level-up odds | Legendary (100) |
|---|---|---|---|
| Swords (STR) | damage variance, less degrade | non-lethal hit 1/10, lethal 1/8, crystal weapon 1/6 | +5 ATK, 5 s bleed on full charge |
| Axes / Maces / Polearms (STR) | same | — | +5 ATK and slow 3 s / 25% paralysis 1.5 s / knockback |
| Unarmed (STR) | bonus fist damage, knockback | 1/10 non-lethal, 1/8 lethal | +5 ATK, 1 s paralysis on charged backstab |
| Ranged (DEX) | variance, pierce (with PER), degrade | arrows 1/10, thrown 1/5 | ranged weapons never degrade |
| Blocking (CON) | AC while defending, passive shield AC | one level per enemy | shields never degrade |
| Stealth (DEX) | lower visibility, see in shadows while sneaking, backstab damage | max **one level per enemy** (5.0) | invisible while sneaking, ×2 backstab |
| Sorcery (INT) | PWR uses **INT+INT** | 1/4 at 0 → 1/8 at 80 (5.0) | +40% INT power, +100% DEX cast-time reduction |
| Mysticism (INT) | PWR uses **INT+CHR** | same | +40% INT/CHR power |
| Thaumaturgy (CON/CHR) | PWR uses **INT+CON** | same | +40% INT/CON power |
| Lore (PER) | appraisal speed/value limit | unidentified potion/scroll use: 25% for +1 below 20 | +25% appraisal speed, see gold values |
| Leadership (CHR) | follower count/speed, charm | recruit, followers fight | recruit Goblins/Goatmen/Insectoids |
| Trading (CHR) | prices | buying; 5.0: max 7 level-ups per shopkeeper | sell anything to any shop |
| Alchemy (PER) | potion potency, duplication, bottle salvage | drink unknown potion, brew, throw | alembics never degrade, brewed potions always +2 |
| Tinkering (PER) | lockpick, disarm, salvage scrap, repair, robot cap | lockpick, workbench, trap/robot kills | repair most items with kit |

[VERIFIED BAR-wiki/Skills; 5.0 spell-skill rules VERIFIED perfectly-nintendo]. 5.0 magic-skill rules worth copying verbatim: a spell can raise its school only up to **Tier + 20** (a Tier III spell at skill 40 caps at 60); each cast must spend a minimum MP to be eligible (**5 MP at skill 0 → 25 MP at skill 100**), extra MP raises the level-up chance up to **1/2**; fizzle chance only below skill 20; above skill 60 the school adds bonus PWR from its stats [VERIFIED perfectly-nintendo].

### 1.6 Classes and races

21 base+DLC classes with signed stat modifiers in [-3, +5] and 3–8 starting skills each. Representative rows (STR/DEX/CON/INT/PER/CHR):

| Class | Stat mods | Starting skills |
|---|---|---|
| Barbarian | 2/0/1/-2/0/-1 | Axes 50, Maces 25, Blocking 25, Unarmed 20, Alchemy 10 |
| Rogue | -1/2/0/-1/2/-1 | Stealth 50, Tinkering 40, Lore 25, Ranged 25, Swords 25, Alchemy 20 |
| Wizard | 0/-1/0/3/1/-1 | Sorcery 50, Thaumaturgy 15, Polearms 25, Lore 10, Alchemy 10 |
| Ninja | 0/2/-1/-2/1/0 | Stealth 60, Swords 60, Ranged 40 |
| Merchant | 0/-1/0/0/1/1 | Trading 50, Lore 50, Leadership 20, Axes 25, Tinkering 10, Alchemy 10 |
| Accursed (DLC) | -2/-3/-2/5/-1/0 | Sorcery 60, Stealth 40, Unarmed 40, Lore 20, Mysticism 20 |
| Hunter (DLC) | -3/1/-1/-3/3/0 | Ranged 50, Stealth 25, Lore 20, Tinkering 10 |
| Bard (DLC 2026) | -2/1/-2/0/0/3 | Lore 50, Swords 25, Leadership 15, Trading 15, Mysticism 15, Stealth 15 |

Full table: 13 base classes (Barbarian, Warrior, Healer, Rogue, Wanderer, Cleric, Merchant, Wizard, Arcanist, Joker, Sexton, Ninja, Monk) + Myths & Outcasts (Conjurer, Accursed, Mesmer, Brewer) + Legends & Pariahs (Mechanist, Punisher, Shaman, Hunter) + Deserters & Disciples (Bard, Sapper, Scion, Hermit, Paladin). The Shaman shapeshifts into Rat/Arthropod/Troll/Imp forms with their own stat rows (Troll STR +5, CON +6, DEX -4) [VERIFIED BAR-wiki/Classes].

14 playable races: Human, Automaton, Dryad, Gnome, Goatman, Goblin, Gremlin, Incubus, Insectoid, Myconid, Salamander, Skeleton, Succubus, Vampire [VERIFIED BAR-wiki/Races]. Races change resource rules, not just numbers: Vampires cast from HP when MP is empty; Automatons run on **Steam** that depends on boiler heat (cold boiler kills); Insectoids use a capped, non-regenerating **Energy** pool refilled by eating [VERIFIED BAR-src/lang/compendium_lang/lang_codex.json "mp"]. Succubus/Incubus can remove cursed items but not blessed ones; Automatons can remove anything [VERIFIED BAR-wiki/Items]. Salamanders hunger 35% faster [VERIFIED BAR-wiki/Hunger].

### 1.7 Hunger (real-time)

| Food level | State | Effect |
|---|---|---|
| > 1500 | Over-full ("sick from eating too much") | -1 STR, -1 DEX, vomit chance |
| 1000 (default start), max 1500 | normal | — |
| ≤ 250 | Hungry | none |
| ≤ 150 | Weak | -1 STR, -1 DEX |
| ≤ 50 | Starving | -2 STR, -2 DEX, -1 PER, -1 INT |
| 0 | Wasting away | Starving + periodic HP loss (5% max HP per tick since 4.3.0) |

Base loss **1 point per 30 ticks** = 1.67/s → 1000 points last **10 minutes**. Slow Digestion and boss levels: 1/120 ticks (40 min). Vampiric Aura: 1/5 ticks. Food quality vomit chance: Fresh 0%, Slightly Aged 4%, Aged 10%, Mouldy 25%, Rotten 100% [VERIFIED BAR-wiki/Hunger].

### 1.8 Items: beatitude, condition, identification (appraisal)

**Beatitude** is a signed integer (`Sint16`), shown as +N/0/−N; it feeds stats directly. Weapon attack begins `Sint32 attack = beatitude;`, armor `Sint32 armor = beatitude;` [VERIFIED BAR-src/src/items.cpp]. Random dungeon items: **50% roll beatitude uniformly in −2..+2, else 0** (`entity->skill[12] = -2 + map_rng.rand() % 5;`) → P(0)=60%, P(each of ±1, ±2)=10% [VERIFIED BAR-src/src/maps.cpp]. Cursed equipment cannot be removed once worn (race exceptions above) [VERIFIED BAR-wiki/Items].

**Condition** `enum Status { BROKEN, DECREPIT, WORN, SERVICABLE, EXCELLENT }` (0..4) [VERIFIED BAR-src/src/items.hpp]. Random spawn: `1 + map_rng.rand() % 4` (Decrepit..Excellent uniform) [VERIFIED BAR-src/src/maps.cpp]. Weapon attack adds `attack += status - 3;` (Excellent +1, Serviceable 0, Worn −1, Decrepit −2); armor pieces use e.g. `armor += std::max(1, 1 + (status - 1)); // 1-4` [VERIFIED BAR-src/src/items.cpp]. Items degrade on use; Tinkering kits and Scrolls of Repair restore [VERIFIED BAR-wiki/Items].

**Appraisal / Lore** — passive, real-time identification. Items start appraising when picked up; the game auto-selects the fastest item, and the player can lock a target. Formula (verbatim):

```cpp
int skillLVL = stats[p]->getModifiedProficiency(PRO_APPRAISAL) + statGetPER(stats[p], entity) * perStatMult; // perStatMult = 3
int value = item->type == GEM_GLASS ? 1000 : item->getGoldValue();
// fast_time = value <= table.fastTimeGold for the highest table row with skillLVL >= row.skill
appraisal_time = fastTimeAppraisal;                  // = 10 * TICKS_PER_SECOND (10 s)
if (!fast_time) appraisal_time = slow time for first appraisal_time_points row with value >= row.value;
if (cat == FOOD || cat == SCROLL || cat == POTION) {
  ratio = max(0.2, 1.0 + (-skillLVL)/100.0);
  appraisal_time = max(fastTimeAppraisal*ratio, appraisal_time*ratio); appraisal_time = max(2*TICKS_PER_SECOND, appraisal_time);
} else if (skillLVL >= 50) {
  ratio = max(0.2, 0.5 + (100 - skillLVL)/100.0); /* same clamps */ }
```
[VERIFIED BAR-src/src/interface/identify_and_appraise.cpp]. Glass gems appraise as if worth 1000 gold (they're bait). The tables load from `data/appraisal_tables.json` (not in the repo); the wiki publishes the 5.0 breakpoints:

| Lore skill | Fast (10 s) up to | Slow up to |
|---|---|---|
| 0 | 5 G | 30 G |
| 5 | 15 G | 65 G |
| 10 | 20 G | 100 G |
| 15 | 35 G | 150 G |
| 20 | 50 G | 200 G |
| 25 | 70 G | 260 G |
| 30 | 100 G | 320 G |
| 35 | 150 G | 425 G |
| 40 | 225 G | 1500 G |
| 50 | 300 G | any |

Descending a floor advances appraisal of every carried item by **25–50%** depending on Lore; above Lore 50 only items worth >300 G train the skill [VERIFIED BAR-wiki/Lore]. Design note: "anyone can see what an item is by looking at it; Lore reveals magical details" — i.e. the base type is never hidden, only beatitude/enchant/effects [VERIFIED BAR-src lang_codex.json "lore skill"].

### 1.9 Magic (5.0)

- 132 distinct spells across **Sorcery 43 / Thaumaturgy 40 / Mysticism 50**, tiered **I–VI**, with MP costs typically 1–51; 20 are **sustained** (e.g. Light 1 MP + 1 MP/15 s; Levitation 1 + 1/0.6 s; Invisible 1 + 1/s) [VERIFIED BAR-wiki/Spells]. `NUM_SPELLS = 225` IDs in code (includes monster-only) [VERIFIED BAR-src/src/magic/magic.hpp].
- Learning: spellbooks (durability), single-use **Tablets** (5.0); books and tablets generate by floor, Tier I–V across floors 0–21+ [VERIFIED perfectly-nintendo].
- Off-hand **spellbook casting**: hold a book and use block key; adds effect = INT/2; blessed books +25% per blessing. **Magic staffs** cast with no MP but degrade [VERIFIED BAR-wiki/Spells].
- Cast time is per-spell (independent of MP) and sped by PWR and DEX; Touch spells use a reticle on objects/creatures/walls/floors, reach scales with PWR and PER [VERIFIED perfectly-nintendo].
- Crafting-flavoured spells: Reshape Weapon, Alter Arrow, Forge Key, Forge Jewel, Metallurgy (transmute), Void Chest, Salvage, Lead Bolt (0 MP + 100 gold) [VERIFIED BAR-wiki/Spells].

### 1.10 Combat, stealth, run structure, assists

- Melee is charge-and-release: release after the arm "shakes" = **critical strike**; **flanking** bonus vs targets engaged with someone else; **backstab** = melee from behind on an unaware enemy, a kill is an **assassination**. **Sneaking** = holding nothing (or special ammo) in the off-hand while defending; slower, harder to detect, better darkness vision [VERIFIED BAR-src lang_codex.json].
- **Wanted** status: robbing shops makes the Merchants' Guild hostile; masks hide identity [VERIFIED same].
- Floors: 0 Minehead; 1–5 Mines; 6–10 Swamp; 11–15 Sand Labyrinth; 16–20 Ruins (boss Baron Herx, 20); 21–24 Hell (Baphomet, 24); 25 Hamlet (town hub); 26–30 Crystal Caves; 31–35 Citadel (final boss Orpheus & Erudyce, 35). Secret levels: Gnomish Mines (2), Minetown (3, 50%), Underworld (5, 18), Temple (7), Haunted Castle (8), Sokoban (11), Minotaur Maze (13), Mystic Library (16), Cockatrice Lair (28), Bram's Castle (33) [VERIFIED BAR-wiki/Dungeon_Progression].
- 5.0: treasure rooms locked by iron/bronze/silver/gold keys (≤2 per biome); wall buttons triggered by arrows/thrown items [VERIFIED perfectly-nintendo].
- **Assist Shrine** (4.3.1): spend Assist Points on items (Cloak of the Guardian 5 AP: 25% combat / 50% trap damage resist; Ring of Resolve 3 AP: +10 max HP/MP; Marigold 3 AP; Amulet of Lifesaving 4 AP). Each AP reduces leaderboard score by **10%**, capped at **95%**; ≥10 AP disables achievements and online leaderboards [VERIFIED perfectly-nintendo].

### 1.11 Why Barony matters for PALIMPSEST

It is the only shipped game in this list with the exact PALIMPSEST frame: first-person, real-time, 1–4 player co-op roguelike with use-based skills, BUC, appraisal and a 30+ floor themed descent. Its 2026 5.0 rewrite moved toward **stat-scaled spells, per-school skill caps tied to spell tier, and a Lore skill that only hides magical properties, never the base item type** — this is the current genre consensus on making ID tolerable in real-time co-op.

---

## 2. Shattered Pixel Dungeon

### 2.1 State in 2026

| Version | Released | Headline |
|---|---|---|
| v2.4 | 2024-05-15 | trinkets, alchemy rework, Duelist rework (per ExpPD 2.18 port notes) |
| v2.5 | 2024-09-11 | journal / bestiary catalogue (per ExpPD 2.19 port notes) |
| v3.0 | 2025-02-27 | **Cleric** (6th hero; spells learned through talents), subclasses Priest/Paladin, armor abilities Ascended Form/Trinity/Power of Many |
| v3.1 | 2025-06-02 | Warrior seal rework (burst shield at ≤50% HP, 150-turn cooldown) |
| v3.2 | 2025-08-04 | thrown-weapon overhaul, accuracy/evasion QoL |
| v3.3 | 2025-12-04 | **Skeleton Key** artifact (open/lock doors, spectral walls), **Cracked Spyglass** trinket, randomize-options mode, vault quest tester area |
| v4.0.0 | **2026-09-09** | Imp quest replaced by a **dwarven vault heist** (~3 floors in size, 20+ room types, enter without your gear, choose one item to keep), new **stealth AI**, 4 new enchants + 2 curses, art overhaul wave 1 |
| v4.0.2 | 2026-10-06 | current build; next is v4.1 (new items in several categories), no ETA |

[VERIFIED SPD-src/ui/changelist/v3_X_Changes.java, v4_X_Changes.java; build.gradle `appVersionName = '4.0.2'`]

### 2.2 Core hero numbers (turn-based)

```java
MAX_LEVEL = 30; STARTING_STR = 10; HP = HT = 20;
HT = 20 + 5*(lvl-1) + HTBoost;          // max HP
maxExp(lvl) = 5 + lvl * 5;              // XP needed for next level
attackSkill = 10; defenseSkill = 5;     // +1 each per level
```
[VERIFIED SPD-src/actors/hero/Hero.java]

Hit roll (verbatim logic): `acuRoll = Random.Float(acuStat); defRoll = Random.Float(defStat); hit if acuRoll >= defRoll` with multiplicative buffs (Bless ×1.25, Hex ×0.8, Daze ×0.5 on either side); magic attacks pass accuracy multiplier 2 [VERIFIED SPD-src/actors/Char.java].

### 2.3 Strength requirements and encumbrance (the cleanest "stat gate" in the genre)

```java
// weapons and armor; tier 1..5, lvl = upgrade level
STRReq(tier, lvl) = (8 + tier*2) - (int)(Math.sqrt(8*lvl + 1) - 1)/2;   // req drops by 1 at +1, +3, +6, +10, +15 ...
// thrown weapons: STRReq(tier,lvl) - 1
```
Excess requirement `enc = STRReq - STR` (if > 0): weapon accuracy `ACC / 1.5^enc`, weapon delay `× 1.2^enc`; armor evasion `/ 1.5^enc`, movement speed `/ 1.2^enc`, armor DR `−2×enc` [VERIFIED SPD-src/items/weapon/Weapon.java, items/armor/Armor.java, actors/hero/Hero.java]. Strength itself only rises via **Potions of Strength: exactly 2 per 5-floor chapter** (`posNeeded()`), so STR is a paced resource, not a grind [VERIFIED SPD-src/Dungeon.java].

### 2.4 Damage and armor formulas

```java
MeleeWeapon.min(lvl) = tier + lvl;            max(lvl) = 5*(tier+1) + lvl*(tier+1);
MissileWeapon.min(lvl) = 2*tier + lvl;        max(lvl) = 5*tier + tier*lvl;
Armor.DRMax(lvl) = tier*(2 + lvl) + augment.defenseFactor(lvl);   DRMin(lvl) = lvl;
Augment: evasion = round((2+lvl)*evasionFactor); defense = round((2+lvl)*defenceFactor)
```
[VERIFIED SPD-src/items/weapon/melee/MeleeWeapon.java, missiles/MissileWeapon.java, items/armor/Armor.java]. Example: tier-3 sword +0 = 3–20; +3 = 6–32.

### 2.5 Loot generation (deck system — copy this)

- Category weights (deck 1 / deck 2): Weapon 2/2, Armor 2/1, Missile 1/2, Wand 1/1, Ring 1/0, Artifact 0/1, Potion 8/8, Seed 1/1, Scroll 8/8, Stone 1/1, Gold 10/10. "game has two decks of 35 items for overall category probs; one deck has a ring and extra armor, the other has an artifact and extra thrown weapon". Decks decrement as drawn and reset when empty; **artifacts never reset (no duplicates)** [VERIFIED SPD-src/items/Generator.java].
- Potions and scrolls also use two alternating decks: `{0,3,2,1,2,1,1,1,1,1,1,1}` / `{0,3,2,2,1,2,1,1,1,1,1,0}` over the 12 types (index 0 = Strength / Upgrade, which are paced separately) [VERIFIED same].
- Equipment tier by chapter (floorSet 0..4), tiers 1..5: `{0,75,20,4,1} {0,25,50,20,5} {0,0,40,50,10} {0,0,20,40,40} {0,0,0,20,80}` [VERIFIED same].
- Weapon/armor upgrade level: +0 75%, +1 20%, +2 5%. Rings and wands: +0 66.67%, +1 26.67%, +2 6.67%. Artifacts always +0 [VERIFIED SPD-src/items/weapon/Weapon.java, rings/Ring.java, wands/Wand.java, artifacts/Artifact.java].
- **30% cursed** for weapons, armor, rings, wands, artifacts. Weapons 10% enchanted, armor 15% glyphed (cursed weapons/armor get a curse-enchant/glyph). Enchant/curse rolls use a pushed side-RNG "so that variance due to things like parchment scrap does not affect levelgen" [VERIFIED same files].
- Upgrade scrolls: **3 per chapter**, chance per floor = `Random.Int(5 - floorThisSet) < souLeftThisSet` (i.e. floors-left/scrolls-left); Arcane Stylus 1 per chapter; alchemy lab room on floor 3 or 4 of each chapter [VERIFIED SPD-src/Dungeon.java, levels/rooms/special/SpecialRoom.java].

### 2.6 Identification

- **Randomized appearances per run**: 12 potion colours (crimson, amber, golden, jade, turquoise, azure, indigo, magenta, bistre, charcoal, silver, ivory), 12 scroll runes (KAUNAN, SOWILO, LAGUZ, YNGVI, GYFU, RAIDO, ISAZ, MANNAZ, NAUDIZ, BERKANAN, ODAL, TIWAZ), 12 ring gems (garnet, ruby, topaz, emerald, onyx, opal, tourmaline, sapphire, amethyst, quartz, agate, diamond) mapped to 12 types each by `ItemStatusHandler` [VERIFIED SPD-src/items/potions/Potion.java, scrolls/Scroll.java, rings/Ring.java].
- **ID by use, rate-limited by XP** (the key trick):
  - Weapons: `usesToID() = 20`; Armor/Wands: `USES_TO_ID = 10`. Each use subtracts from both `usesLeftToID` and `availableUsesToID`; `availableUsesToID` starts at half and **only refills as the hero gains XP**: `availableUsesToID = min(usesToID/2, availableUsesToID + levelPercent*usesToID)` ("gains enough uses to ID over 0.5 levels"; wands refill at `USES_TO_ID/2` per level). You cannot ID gear by whacking a door 20 times.
  - Rings: `levelsToID = 1` — identify after **one hero level of XP while equipped** ("rings cannot be 'used' like other equipment, so they ID purely based on exp").
  [VERIFIED SPD-src/items/weapon/Weapon.java, armor/Armor.java, wands/Wand.java, rings/Ring.java]
- Potions/scrolls ID by drinking/reading (type becomes known for the run). Scroll of Identify reveals one item fully. **Stone of Intuition**: guess the type of one potion/scroll/ring (correct = ID). **Stone of Detect Magic**: reveals whether an unidentified item is cursed and whether it has positive magic [VERIFIED SPD assets messages/items/items.properties].
- **Shard of Oblivion** (trinket): +20% enemy loot drop rate per piece of unidentified equipment equipped or recently used (wand/thrown trackers last 50 turns), up to `level+1` items; blocks passive ID but lets you manually ID items that are "ready" [VERIFIED SPD items.properties; trinkets/ShardOfOblivion.java]. This is a reward-for-ignorance dial.

### 2.7 Upgrades and curses

- Scroll of Upgrade +1 level; upgrading clears the `cursed` flag (weakens curse), removes a **curse enchant with 33%** ("chance to remove curse is a static 33%"); a good enchant is lost when upgrading from +4/+5/+6/+7/+8 with **10/20/40/80/100%** (`Random.Float(10) < Math.pow(2, level()-4)`); "hardened" enchant loss 10..100% from +6..+10 [VERIFIED SPD-src/items/weapon/Weapon.java, scrolls/ScrollOfUpgrade.java]. Cursed gear cannot be unequipped.
- Curse rewards exist: curse infusion grants `level += 1 + level/6` while cursed [VERIFIED SPD-src/items/weapon/Weapon.java].

### 2.8 Hunger (turn-based)

`HUNGRY = 300`, `STARVING = 450`; +1 hunger per turn (Shadows buff ×1/1.5); food ration energy = 300, pasty 450, meat pie 900, berry 100. While starving: `partialDamage += target.HT/1000f` per turn (0.1% max HP/turn). Hunger pauses on boss-locked levels [VERIFIED SPD-src/actors/buffs/Hunger.java, items/food/*].

### 2.9 Classes, subclasses, talents

6 heroes × 2 subclasses × 3 armor abilities: Warrior (Berserker/Gladiator), Mage (Battlemage/Warlock), Rogue (Assassin/Freerunner), Huntress (Sniper/Warden), Duelist (Champion/Monk), Cleric (Priest/Paladin) [VERIFIED SPD-src/actors/hero/HeroClass.java, HeroSubClass.java, abilities/]. Talent tiers unlock at `tierLevelThresholds = {0, 2, 7, 13, 21, 31}`: tier 1 levels 2–6 (5 pts), tier 2 levels 7–12 (6), tier 3 levels 13–20 (8, requires subclass — from Tengu, floor 10), tier 4 levels 21–30 (10, requires armor ability — from Dwarf King, floor 20) [VERIFIED SPD-src/actors/hero/Talent.java, Hero.java].

### 2.10 Alchemy (Energy economy)

- One pot per lab room; 1-, 2- or 3-ingredient recipes plus **Energy** (currency). Alchemize value: potions and scrolls **6 energy** each, runestones **3** [VERIFIED SPD-src/items/potions/Potion.java, scrolls/Scroll.java, stones/Runestone.java].
- Energy costs: Potion→Exotic 4; Scroll→Exotic 6; Scroll→Runestone 0; seeds→potion 0; Brews: Caustic 1, Aqua 8, Blizzard 8, Shocking 10, Infernal 12; Elixirs: Honeyed Healing 2, Icy Touch 6, Aquatic Rejuvenation 6, Arcane Armor 8, Toxic Essence 8, Dragon's Blood 10, Feather Fall 10, Might 16; Spells: Wild Energy 4, Curse Infusion 6, Reclaim Trap 8, Phase Shift 10, Telekinetic Grab 10, Summon Elemental 10, Magical Infusion 12, Recycle 12, Beacon of Returning 12; Stewed meat 1–2 [VERIFIED SPD-src/items/** `cost =` lines].
- Seeds→Potion (3 seeds, verbatim rule): all same type → that potion, auto-identified; 2 distinct types → **25%** random potion else one ingredient's potion; 3 distinct → **50%** random. Healing results are re-rolled with chance `Random.Int(10) < COOKING_HP.count` (diminishing healing farms) [VERIFIED SPD-src/items/potions/Potion.java].
- Only uncursed items are valid ingredients; unidentified non-equipment ingredients are allowed [VERIFIED SPD-src/items/Recipe.java].

### 2.11 Run structure, challenges, score, seeds

- 25 floors in 5 chapters + floor 26 (Amulet): Sewers 1–4 / boss 5 Goo; Prison 6–9 / 10 Tengu; Caves 11–14 / 15 DM-300; City 16–19 / 20 Dwarf King; Halls 21–24 / 25 Yog-Dzewa. Quests: Ghost (sewers), Wandmaker (prison), Blacksmith (caves), Imp/vault (city) [VERIFIED SPD-src/levels/*.java names; boss score code in actors/mobs].
- **Challenges** are a 9-bit mask: NO_FOOD 1, NO_ARMOR 2, NO_HEALING 4, NO_HERBALISM 8, SWARM_INTELLIGENCE 16, DARKNESS 32, NO_SCROLLS 64, CHAMPION_ENEMIES 128, STRONGER_BOSSES 256 [VERIFIED SPD-src/Challenges.java]. v4.0 swarm: alert radius starts **2 tiles, grows to 12** while an enemy sees you, resets on broken sight [VERIFIED v4_X_Changes.java].
- **Score** (verbatim, v1.3+):
```java
progressScore = min(hero.lvl * deepestFloor * 65, 50_000);
treasureScore = min(goldCollected + heldItemValue, 20_000);
exploreScore  = Σ round(percentExplored_f * floorsExplored * 50);
bossScore: Goo +1000, Tengu +2000, DM-300 +3000, Dwarf King +4000, Yog +5000 + 1250*spawnersAlive; −100..−500 per avoidable hit taken
questScore: Ghost 1000, Blacksmith += favor, Imp 4000, Corpse-dust kept to floor 10 = 2000
winMultiplier = 1 (+1 if won, +0.5 if ascended);  chalMultiplier = round(1.25^activeChallenges * 20)/20;
total = (progress + treasure + explore + boss + quest) * winMultiplier * chalMultiplier;
```
[VERIFIED SPD-src/Rankings.java, actors/mobs/Goo.java, Tengu.java, DM300.java, DwarfKing.java, YogDzewa.java, npcs/Ghost.java, npcs/Imp.java]
- **Seeds**: 9 letters A–Z, base-26, `TOTAL_SEEDS = 5429503678976L` (26^9); daily runs exist with score history [VERIFIED SPD-src/utils/DungeonSeed.java, Rankings.java].

### 2.12 Stealth detection (v4.0 is directly reusable)

Per-turn notice chance (`stealth()` = 0 base + gear):
| AI state | Chance per turn |
|---|---|
| Sleeping (classic) | `1 / (distance + stealth)` |
| Wandering/Hunting (classic) | `1 / (distance/2 + stealth)` |
| Vault Sleeping | `1 / (distance + stealth)^2`, set to 0 if < 10% |
| Vault Wandering, target inside 75° forward cone and ≤ 6 tiles | `1 / (distance/2 + stealth)` |
| Vault Wandering, inside 150° cone | `1 / (distance + stealth)` |
| Vault Wandering, behind | `1 / (distance + stealth)^3`, 0 if < 10% ("1/8 at 2 tiles, 0% at 3+") |
| Vault Investigating | wandering formula, −0.33 on the first investigating turn |

New state **Investigating** sits between wandering and attacking: the enemy moves toward the noise but won't attack until it re-detects you; being attacked skips straight to retaliation; nearby enemy movement is visible through walls [VERIFIED SPD-src/actors/mobs/Mob.java; v4_X_Changes.java].

---

## 3. Experienced Pixel Dungeon (Redone)

### 3.1 Status

Fork of SPD by TrashboxBobylev. **ExpPD-2.19 (2024-09-29) is labelled "the last version of Experienced PD"**; it ports SPD 2.5.2. No release since [VERIFIED https://github.com/TrashboxBobylev/Experienced-Pixel-Dungeon-Redone/releases; EXPD-src/ui/changelist/ExpPDChanges.java]. It is a finished design artifact to mine, not a live competitor.

### 3.2 The loops

- **No level cap** (SPD's `MAX_LEVEL = 30` gate removed; `maxExp = 5 + lvl*5L` as a 64-bit long) [VERIFIED EXPD-src/actors/hero/Hero.java].
- **Upgrades from XP**: every `neededExp` total XP grants a Scroll of Upgrade while "grinding" is on (toggled by the **Fate Lock** item, default on): 100 XP at cycle 0, 200 / 1,250 / 11,750 / 75,000 / 500,000 at cycles 1–5; ×2.5 with the No-Scrolls challenge [VERIFIED EXPD-src/items/PsycheChest.java, Hero.java].
- **Cycles** (NG+ after the Amulet; up to cycle 5): depth resets to 0, quests/limited drops reset, `resetDamage = 1 + (resetDamage-1)*0.5`; monster scaling uses a virtual depth:
```java
escalatingDepth(d): cycle0 d; 1: d*2+55; 2: d*25+750; 3: d*300+12500; 4: d*3000+100000; 5: d*40000+410000
```
[VERIFIED EXPD-src/Dungeon.java]
- **Treasure bags**: each boss drops a bag (Goo bag: 2–3 Goo blobs + gold `Random.Int(400 + eDepth*60, 850 + eDepth*85)`); **Lucky Bag** = `Random.Int(0,15)` random items (0–14), value 175 each; Big Lucky Bag (more), **Elite Bag** (one uncursed item of every category), burning lucky bags gives Burnt Bags or ~10% a **Refined Lucky Bag** (≈10 lucky bags of loot, 4.5% exclusive item); **Lucky** weapon enchant makes kills drop extra loot (Elemental Strike variant 12.5% per enemy) [VERIFIED EXPD-src/items/treasurebags/*.java; assets/messages/items/items.properties; ExpPDChanges.java].
- 2.17 moved HP, damage, gold, XP, upgrades, charges to **64-bit** (max ~9.2×10^18) because players overflowed 2^31 [VERIFIED ExpPDChanges.java].
- Extras: fishing rods (equipment from fishing scales `6^(cycle+1)`), arena, Fate Lock "resets" making enemies +8% faster/+16% stronger per reset, score caps raised (progression 600k, treasure 1,000k after /1000) [VERIFIED ExpPDChanges.java].

### 3.3 What makes it addictive (analysis)

1. Every kill pays twice: XP toward levels (uncapped) and toward the next guaranteed upgrade scroll — a **visible, monotonic progress bar** that never stops.
2. Bosses become **loot piñatas** (bags) instead of gates; bags contain bags (lucky → refined) = nested variable-ratio reward.
3. Cycles give a **soft prestige** with exponential virtual-depth formulas, so numbers keep inflating while the map set stays familiar.
4. The cost: it is explicitly "grindy"; tactical identification tension evaporates once upgrades flow from XP.

---

## 4. IVAN (Iter Vehemens ad Necem)

### 4.1 Status

Last tagged release **0.59 (2021-12-23)**, which added a **Crafting** skill and custom keybinds; the Attnam community repo is still committing (latest "Crafting fixes", 2026-09-09) [VERIFIED IVAN-src/NEWS; git log]. All data is plain-text script (`Script/material.dat`, `item.dat`, `char.dat`) — an excellent model for a data-driven, agent-authored content pipeline.

### 4.2 Material system (every item = shape × material × volume)

Material record fields (defaults in brackets): `StrengthValue` (obligatory), `Density` kg/m³ (obligatory), `ConsumeType`, `PriceModifier` [0], `Emitation` (light), `NutritionValue` [10], `Flexibility` [1], `ExplosivePower`, `Acidicity`, `Hotness`, `FireResistance`, `BurnModifier` [1000], `IntelligenceRequirement` [1] (needed to recognise it), `HardenedMaterial` / `SoftenedMaterial` (upgrade/downgrade chain), `AttachedGod`, flags (`IS_METAL`, `IS_VALUABLE`, `CAN_BE_WISHED`, `IS_GOLEM_MATERIAL`, `BlockESP`…) [VERIFIED IVAN-src/Script/material.dat]. 412 `Config` entries in the file.

Core formulas (verbatim):
```cpp
material::GetWeight()        = Volume ? long(double(Volume) * GetDensity() / 1000) : 0;   // grams from cm³
material::GetRawPrice()      = GetPriceModifier() * GetWeight() / 10000;
item::GetTruePrice()         = Max(GetPrice(), GetMaterialPrice());                  // spoiled food scaled down
item::GetWeaponStrength()    = GetFormModifier() * Main->GetStrengthValue() * sqrt(Main->GetWeight());
item::GetBaseDamage()        = Max(0., sqrt(5e-5 * GetWeaponStrength()) + GetDamageBonus());
item::GetBaseMinDamage()     = int(GetBaseDamage() * 0.75);   GetBaseMaxDamage() = int(GetBaseDamage() * 1.25) + 1;
item::GetBaseToHitValue()    = int(10000. / (1000 + GetWeight()) + GetTHVBonus());
item::GetStrengthRequirement() = int(1.25e-10 * (Weight*Size)^2);
item::GetStrengthValue()     = long(GetStrengthModifier()) * Main->GetStrengthValue() / 2000;   // armor adds +Enchantment, min 0
whip/belt FormModifier       = item::GetFormModifier() * Main->GetFlexibility();
```
[VERIFIED IVAN-src/Main/Include/materia.h, Main/Source/materia.cpp, item.cpp, gear.cpp]

Representative metal ladder (StrengthValue / Density / PriceModifier / IntRequirement → hardens into):

| Material | SV | Density | Price mod | IntReq | Hardens to |
|---|---|---|---|---|---|
| Tin | 60 | 7250 | 15 | 3 | Copper |
| Copper | 70 | 9000 | 20 | 4 | Bronze |
| Bronze | 80 | 8300 | 30 | 5 | Deep bronze |
| Silver | 60 | 10500 | 1000 | 16 | Moon silver |
| Mithril | 200 | 5000 | 2000 | 26 | Galvorn |
| Star metal | 300 | 3000 | 3000 | 42 | — |
| Gold | 55 | 20000 | 1500 | 14 | Dark gold |
| Tungsten | 135 | 19200 | 500 | 17 | Iridium |
| Titanite | 355 | 4500 | 2500 | 41 | — |
| Valpurium (divine) | 400 | 3000 | 50000 | 50 | — |
| Spider silk (cloth) | 180 | 300 | 500 | 40 | Gossamer |
| Dragon hide | 160 | 2500 | 750 | 33 | Ouroboros hide |

[VERIFIED IVAN-src/Script/material.dat]. Consequence: a gold sword is heavy (high damage via sqrt(weight)) but soft (low SV) and inaccurate (to-hit falls with weight); valuables have high price but bad combat stats — material choice is a real trade-off, and the same formulas price, weigh and arm every object.

Wishing: Scroll of Wishing takes free text and can produce item+material combos ("Two %s appear…" for pairs); reading it trains Intelligence (`EditExperience(INTELLIGENCE, 600, 1 << 12)`); 9 materials are flagged un-wishable (`Base&~CAN_BE_WISHED`). Scroll of Change Material rewrites an item's material [VERIFIED IVAN-src/Main/Source/miscitem.cpp; Script/material.dat].

### 4.3 Attributes trained by use (asymptotic toward a species norm)

11 attributes: 7 base (Endurance, Perception, Intelligence, Wisdom, Willpower, Charisma, Mana) + 4 per-limb (Arm Strength, Leg Strength, Dexterity, Agility — stored on each arm/leg) [VERIFIED IVAN-src/Main/Include/ivandef.h]. Attribute = `int(Exp * EXP_DIVISOR)` with `EXP_DIVISOR 2e-8`, `EXP_MULTIPLIER 5e+7`, clamp `MIN_EXP 5e+7` .. `MAX_EXP 5e+10` (attribute 1..1000). Training call `EditExperience(Identifier, Value, Speed)`:

```cpp
if (!OldExp || !Value || (Value > 0 && OldExp >= NaturalExp*(100+Value)/100)
                       || (Value < 0 && OldExp <= NaturalExp*(100+Value)/100)) return 0;
if (IsPlayer() && !IsPlayerKind()) Speed *= 0.5;   // polymorphed player trains slower
else if (!IsPlayer())              Speed *= 1.5;   // NPCs train faster
Exp += (NaturalExp*(100+Value) - 100*OldExp) * Speed * EXP_DIVISOR;
```
[VERIFIED IVAN-src/Main/Source/char.cpp]. Meaning: an action of strength `Value` pulls the attribute toward `(100+Value)%` of the creature's natural level, with diminishing returns as it approaches; negative Values (e.g. angering a god: Wisdom −50) pull down. Leg Strength changes recompute burden; Endurance recomputes limb HP.

Weapon skills (category + specific weapon), 0–20 levels by hit count:
- Category thresholds: `0, 1500, 2000, 3000, 5000, 7500, 10000, 15000, 20000, 30000, 50000, 75000, 100000, 150000, 200000, 300000, 500000, 750000, 1000000, 1500000, 2000000`; bonus `1000 + 50*Level` (‰, ×1.0→×2.0).
- Specific-weapon thresholds `0, 500, 750, 1000, … 750000`; bonus `Level ? 1150 + 25*(Level-1) : 1000`.
- **Skill rust**: unused skills lose hits after `UnuseTick` (category L0 500000 ticks … L20 5000) by `UnusePenalty` (100 … 10000).
- 12 categories: unarmed, kicking, biting, uncategorized, small swords, large swords, blunt, axes, polearms, whips, shields, **crafting**.
[VERIFIED IVAN-src/Main/Source/wskill.cpp, Main/Include/wskill.h]

Wielded damage: `BaseDamage * sqrt(1e-13 * HitStrength) * SWeaponSkillBonus * CWeaponSkillBonus` when ArmStrength > requirement (two-handing adds the pair arm and quarters the requirement); to-hit `2e-11 * min(HitStrength,10) * skills * MoveEase * (10000/(1000+Weight)+THVBonus) * Dex*sqrt(2.5*Per)` [VERIFIED IVAN-src/Main/Source/bodypart.cpp].

### 4.4 Body parts: volume-proportional HP, scars, severing

```cpp
// organic limbs
DoubleHP = BodyPartVolume * Endurance * Endurance / 200000;
for each scar: DoubleHP *= (100 - Scar.Severity*4) / 100;
DoubleHP *= (4 - BurnLevel) / 4;
// golems / material bodies
MaxHP = (Volume * SV >> 4) * SV / 250000;
```
Humanoid volumes: head **4000 cm³**; torso `(Total-4000)*13/30`; each arm `(Total-4000)/10`; groin `/10`; each leg `(Total-4000)*2/15` [VERIFIED IVAN-src/Main/Source/bodypart.cpp, human.cpp]. A part reaching HP ≤ 0 is severed if the damage type can sever (else HP floors at 1); severed limbs drop their equipment and lie on the floor as items (can be reattached; gods may re-attach on prayer). Scar chance per hit `!(RAND_N(25 + 25*HP/MaxHP))` [VERIFIED bodypart.cpp, char.cpp, god.cpp]. Hit location: parts ranked weakest-first (SV + HP), each gets `10 * ToHit * PartVolume / (Dodge * BodyVolume)` percent [VERIFIED char.cpp `ChooseBodyPartToReceiveHit`, balance.h `GLOBAL_WEAK_BODYPART_HIT_MODIFIER 10.`].

### 4.5 Gods and prayer (complete model)

Relation −1000..+1000, prayer Timer ticks down 1 per divine tick:
| Condition | Result |
|---|---|
| Timer = 0 and `Relation >= -RAND_N(500)` | Pleased: reattach a limb / harden a body part / good effect; Timer +5000; Relation +50; Wisdom +200 training |
| Timer = 0, else | Displeased: bad effect; Timer +10000; Relation −50 |
| Timer > 0 and `Relation > RAND_N(500)` and `Timer < RAND_N(500000)` | Helps anyway: Timer +25000; Relation −75 |
| Timer > 0, else | Angry: bad effect; Timer +50000; Relation −100; if Relation < −250, 1/5 chance hostile angel |
| Good prayer, Relation > 250 and 1/20 | friendly angel; Relation > 100 and 1/20 → gift item value-balanced to [Relation/2, Relation*2], enchant +1/+2 (≥500)/+3 (=1000) |

Relation changes are scaled by Wisdom: gains `×(100+WIS)/100`, losses `×100/(100+WIS)`. Competing gods: `(Multiplier*2) − |alignmentA − alignmentB| * Multiplier`. Sacrifice value `sqrt(TruePrice)`, doubled for the item's attached god (materials carry `AttachedGod`), halved otherwise [VERIFIED IVAN-src/Main/Source/god.cpp, item.cpp].

---

## 5. Caves of Qud

### 5.1 Status

1.0 released **2024-12-05** after ~9 years of early access [VERIFIED COQ-wiki/Version_history]. 2025: patch 1.04 (May), maintenance build 2.0.210.22 (2025-07-28), "Qud 2025 Wrapped": 13 patches, 232 bugfixes, a console version announced [VERIFIED https://freeholdgames.itch.io/cavesofqud/devlog/996768/maintenance-patch-july-28-2025 via search snippet; wrap-up via search snippet]. 2026 beta builds 211.33 (2026-01-29) … 212.17 experimental (2026-03-27, debug world-seed entry) [VERIFIED COQ-wiki/Version_history].

### 5.2 Attributes

Strength, Agility, Toughness, Intelligence, Willpower, Ego. **Modifier = floor((value − 16) / 2)** [VERIFIED COQ-wiki/Attributes].

| Attribute | Exact uses |
|---|---|
| STR | melee PV + STR mod up to weapon's Strength Bonus Cap; carry = **15 × STR** lb; saves vs forced movement/stuck |
| AGI | **DV = 6 + AGI mod + bonus DV**; to-hit |
| TOU | start HP = TOU; +**1d4 + TOU mod** HP/level; bleed save `1d20 + TOU mod`; poison/illness |
| INT | skill points per level **70 (True Kin) / 50 (Mutant) + 4×(INT−10)**, retroactive; artifact examination |
| WIL | cooldowns −5% of base per point above 16 (max −80% at 32), +5% per point below; min cooldown 5 rounds; HP regen; MA |
| EGO | mental-mutation levels shift by Ego mod; trade prices; Persuasion/Proselytize |

Point-buy: 1 pt per +1 up to 18, then 2 per +1; max 24 before caste bonuses (cost to reach 11..24: 1,2,3,4,5,6,7,8,10,12,14,16,18,20). Every 6 levels from 6: +1 all attributes; every 6 levels from 3: +1 attribute point [VERIFIED COQ-wiki/Attributes, COQ-wiki/Skills].

### 5.3 Combat

- **Quickness**: base 100; each turn gain `10 × QN` action points; most actions cost 1000; leftover carries; action costs jitter ±10% [VERIFIED COQ-wiki/Quickness].
- **To-hit**: `1d20 + AGI mod + to-hit bonus` must exceed defender **DV**; natural 20 on melee = crit, cannot be dodged; ranged −5 unless Swift Reflexes; immobile/over-burdened DV = −10 (melee) / −100 (missile) [VERIFIED COQ-wiki/DV].
- **AV**: base 0; sum of armor; multiple slots of the same body-part type are **averaged** per type (half rounds to even); shield AV only on a block [VERIFIED COQ-wiki/AV].
- **Penetration**: die `d[-1..8]!` (exploding on max). Three rolls vs target number `5 − PV + AV` (a target of 5 succeeds 40%). All three fail → 0 penetrations; any success → ≥1 penetration; all three succeed → +1 and repeat with PV −2 (strength-sensitive weapons only). Each penetration adds one full set of damage dice (log shows "x2"). Melee PV = base (most weapons 4) + STR mod, capped by the weapon's Strength Bonus Cap; vibro weapons PV = target AV + 4; thrown base 5 + STR mod; player crits guarantee 1 penetration. Approx: PV−AV −4 → 22% to penetrate; 0 → 78% (0.82 avg); +4 → 99% (1.56 avg) [VERIFIED COQ-wiki/Penetration, COQ-wiki/Attributes].
- **HP regen**: `(20 + 2×(WIL mod + TOU mod)) / 100`-scaled base rate; damage interrupts natural regen for 5 turns [VERIFIED COQ-wiki/HP — page text partially flattened; treat as approximate].

### 5.4 Skills

21 trees (Acrobatics 75, Axe 50, Bow and Rifle 100, Cooking and Gathering 100, Cudgel 50, Customs and Folklore 150, Endurance 100, Heavy Weapon 100, Long Blade 100, Multiweapon Fighting 150, Persuasion 100, Physic 50, Pistol 100, Self-discipline 150, Shield 100, Short Blade 50, Single Weapon Fighting 100, Tactics 100, Tinkering 100, Wayfaring 100, Nonlinearity). Skills cost 0–400 SP with attribute gates; trainers teach via water ritual for reputation equal to SP cost, ignoring prerequisites [VERIFIED COQ-wiki/Skills].

### 5.5 Mutations and cybernetics

- **Mutation Points**: 12 at creation (more only by taking defects); +1 MP per level. New mutation = **4 MP for a choice of 3 random**; rank-up = 1 MP, new level ≤ **floor(PlayerLevel/2) + 1**, base cap 10. Physical mutations get **Rapid Advancement** (+3 levels) at player level 5 and every 10 after; mental mutations get ±Ego-mod levels (may exceed 10). Morphotypes: Chimera (1 MP, physical only), Esper (1 MP, mental only), Unstable Genome (3 MP; 33% per level-up to manifest a choice of 3). >70 mutations. Mental mutation total = **Glimmer**, which attracts esper hunters [VERIFIED COQ-wiki/Mutations].
- **Cybernetics** (True Kin): license tier starts at 2 (0 + 1 TOU if no starting implant); buy tier points with credit wedges (1¢/pt for tiers 1–8, 2¢ 9–16, 3¢ 17–24, 4¢ 25+; tier 2→24 = 46¢); implants cost 1–12 points and occupy body slots (Face, Head, Body, Back, Arm, Hands, Feet); installed/removed at **becoming nooks**; some are destroyed on removal [VERIFIED COQ-wiki/Cybernetics].

### 5.6 Anatomy and dismemberment

18 body-part types: armor slots Head, Face, Body, Worn on Back, Arm, Worn on Hands, Feet, Floating Nearby, Tread, Roots; weapon slots Hand, Missile Weapon, Thrown Weapon; non-equippable Fin, Tail, Foot, Fungal Outcrop, Icy Outcrop. Items may span slots (e.g. Back + Floating Nearby) [VERIFIED COQ-wiki/Body]. Dismemberment: penetrating axe hits **3% (one-handed) / 6% (two-handed)** with the Dismember skill; Berserk → 100% for 5 rounds; charging doubles; Serrated mod 3% without penetration; at STR 25 heads become eligible and decapitating a single-headed organic creature kills it [VERIFIED search snippets of COQ-wiki/Axe, COQ-wiki/Dismember, COQ-wiki/Serrated].

### 5.7 Liquids

Unit = **dram**; fresh water is the currency (waterskin 64 drams). Values per 64 drams: water 64, honey 128, oil 192, wine 256, lava 3200, neutron flux 64000, cloning draught 80000; weight 0.25 lb/dram (honey/sap/lava 0.5, neutron flux 2.5). Liquids mix on contact; names stack adjectives ("inky sugary bloody water"); evaporation only for puddles ≤199 drams, chance per dram per tick = volume-weighted evaporativity (oil/lava/salt 0); pure water insulates, impure conducts; acid eats containers; lava ignites non-glass containers [VERIFIED COQ-wiki/Liquids].

### 5.8 Tinkering, mods, artifact examination

- **Bits**: `<A>` power, `<B>` crystals, `<C>` metal, `<D>` electronics (common) and `<1>`..`<8>` (phasic power, flawless crystals, pure alloys, pristine electronics, nanomaterials, photonics, AI microcontrollers, metacrystals). Disassembly guarantees the rightmost (most valuable) bit, 50% each other bit [VERIFIED COQ-wiki/Bits, COQ-wiki/Tinkering].
- Skills: Tinker I/II/III (INT 19/23/29; 100/200/300 SP; one free schematic each), Disassemble, Reverse Engineer (25% learn item, 15% per mod), Repair, Scavenger, Deploy Turret, Lay Mine, Gadget Inspector (+5 examine) [VERIFIED COQ-wiki/Tinkering, COQ-wiki/Gadget_Inspector].
- **Mods**: max **3 per item** ("gigantic" is free). Apply cost = mod's bit + a bit equal to `(item tier + number of existing mods)`: flexiweaved `<2>` on tier-5 plate = `<25>`; adding wooly (`<2>`) afterwards costs `<26>`. Loot mod rarity weights Common 2000 / Uncommon 800 / Rare 210 / Rare2 50 / Rare3 3; below native tier weight = `Weight / ((ModNativeTier − ItemTier) * 5)` [VERIFIED COQ-wiki/Item_mods].
- **Artifact examination** (unknown tech = PALIMPSEST's "appraisal" model): effective rating = INT − artifact difficulty − confusion − 20 (Dystechnia) + 5 (Gadget Inspector) + 5 per telescopic monocle + 3 (powered advanced toolkit) + 2 (wrist calc) + 8 per micromanipulator array + 2..4 (comfortable chair); clamped −5..35; roll yields 0–10 successes (Action-Table style, always ≥1% chance of 0). 0 successes → item Broken (unless Sturdy); successes ≥ complexity → identified; 10 → always identified; fewer → partially identified with an intermediate name; confused examiners always *think* they broke it [VERIFIED COQ-wiki/Artifact_examination].

### 5.9 Factions, water ritual, history, temperature

- Reputation bands: ≤−600 everyone hostile; −599..−250; −249..249; 250..599 friendly (holy places open); ≥600 revered. Merchant trade Ego modifier +3/+1/0/−1/−3 across those bands [VERIFIED COQ-wiki/Reputation].
- **Water ritual** ("Your thirst is mine, my water is yours"): one-time rep +100/+50/−50/−100 by the creature's feelings toward its faction; trade gossip (+100) or secrets (+50), buy secrets (−50), recruit (`−(12×(their level − your level) + 200)`, min 50), learn skills (rep = SP cost), recipes (−50×tier/3), mental mutations (−50×cost); max 150 rep per creature; killing a water-sibling −90..−110 with all factions [VERIFIED COQ-wiki/Water_ritual].
- **Sultans**: six (five generated per world with element themes, plus fixed Resheph); life events, relics (themed mods), faction likes/hates; history shown on statues/engravings; viewing records a "secret" in the journal tradable to factions [VERIFIED COQ-wiki/Sultan].
- **Temperature** (T): ambient 25; per turn objects >5T from ambient move by max(1/50 of difference, 5T); thresholds brittle −100, freezing 0, flaming 350, vapor 10000; resistances only apply to "harmful" changes beyond 25–50T; beating flames: hands −100T, rolling −200T [VERIFIED COQ-wiki/Temperature].

---

## 6. NetHack identification (the reference vocabulary)

- **BUC**: every item but gold is blessed / uncursed / cursed. Generation rates differ by class: scrolls and potions 12.5 / 75 / 12.5%; weapons 8.64 / 78.18 / 13.18%; armor 8.72 / 78.96 / 12.31%; wands and spellbooks 2.94 / 94.12 / 2.94%; rings (non-chargeable) 0 / 90 / 10%; magic lamp 25 / 50 / 25%. Altar drop: **amber flash = blessed, black flash = cursed**, none = uncursed; priests see BUC; pets avoid cursed items; holy water steps cursed→uncursed→blessed [VERIFIED NH-wiki/BUC].
- **Price-ID** (3.6): buy price = base (+10 zm per + of enchantment) × Charisma multiplier (CHA ≤5 ×2; 6–7 ×3/2; 8–10 ×4/3; 11–15 ×1; 16–17 ×3/4; 18 ×2/3; ≥19 ×1/2) × 4/3 "sucker" (dunce cap, visible shirt, Tourist < XL15) × 4/3 for ~1 in 4 unidentified objects (fixed per object). Sell offer = 1/2 base (1/3 for suckers; ~1 in 4 shopkeepers pay 1/4 less for unidentified). Price groups, e.g. scrolls 20 (identify) / 50 / 60 / 80 / 100 / 200 / 300; potions 0 / 50 / 100 / 150 / 200 / 250 / 300; rings 100 / 150 / 200 / 300; wands 100 / 150 / 175 / 200 / 500 (death, wishing); spellbooks 100 × level; amulets 150 [VERIFIED NH-wiki/Price_identification].
- **Scroll of identify**: blessed — 1, 2, 3, 4 items or everything at 1/5 each; uncursed — 1 item 21/25, else 2 / 3 / 4 / all at 1/25 each; cursed — 1 item. Spell of identify = uncursed scroll at Unskilled/Basic, blessed at Skilled/Expert [VERIFIED NH-wiki/Identification].
- **Engrave-ID**: engrave in dust then engrave again with an unknown wand; nine wands are identified unambiguously, the rest give ambiguous messages [VERIFIED NH-wiki/Identification]. Auto-ID set (digging "gravel flies up", fire "flames", lightning "lightning arcs… blinded"); message-only set (teleportation/cancellation/make invisible: engraving vanishes; polymorph: engraving changes; slow/speed monster: bugs slow/speed; sleep/death: bugs stop moving; striking: wand fights your attempt; cold: frost/ice; magic missile: bullet holes) [RECALLED].
- Other channels: use-ID, weight-ID, sink-ID for rings (destroys the ring), monster-use, touchstone for gems, #name/call for informal labels; items with different ID states do not stack [VERIFIED NH-wiki/Identification].

---

## 7. Cross-game comparison

### 7.1 Identification approaches

| Game | What is hidden | How revealed | Anti-tedium device |
|---|---|---|---|
| NetHack | type (randomized appearance), BUC, enchant/charges | use, price groups, altar, engrave, scrolls | none (knowledge is the skill) |
| SPD | type of potions/scrolls/rings; level, curse, enchant of gear | use-count gated by XP; identify scroll; intuition/detect-magic stones | `availableUsesToID` refills only with XP; one level of XP for rings |
| Barony 5.0 | magical properties only (beatitude, enchant), never base type | passive real-time Lore timer vs gold value; +25–50% on descent | appraisal runs in background; fast/slow value bands; manual lock |
| CoQ | artifact function (unknown tech) | INT roll vs complexity, 0–10 successes, partial IDs | gear/tools/chairs add flat bonuses; NPCs ID for water |
| IVAN | material identity gated by `IntelligenceRequirement` | INT ≥ material's requirement | material names auto-reveal with INT growth |

### 7.2 Use-based growth

| Game | Model | Cap / anti-grind |
|---|---|---|
| Barony | per-skill 0–100, chance per *successful* use (1/10 hit, 1/8 kill…) | ≤1 level per enemy for Stealth/Blocking; spells cap at Tier+20; min MP per cast |
| IVAN | continuous exp toward `(100+Value)%` of natural attribute | asymptotic; rust on unused weapon skills |
| CoQ | XP levels → SP/MP/attribute points | point-buy; INT scales SP |
| SPD | XP levels → talent points; STR via paced potions | 30 levels; 2 STR potions per chapter |
| ExpPD | uncapped levels; XP → upgrade scrolls | none (by design) |

### 7.3 Attribute schemas

| Game | Primary | Sub / per-limb | Modifier formula |
|---|---|---|---|
| Barony | STR DEX CON INT PER CHR | — | linear per point (5.5% dmg, +1/+2 HP…) |
| CoQ | STR AGI TOU INT WIL EGO | — | floor((v−16)/2) |
| IVAN | END PER INT WIS WIL CHA MANA | ArmStr, LegStr, DEX, AGI per limb | multiplicative/sqrt in formulas |
| SPD | STR only (+ hidden acc/def) | — | gate formula vs gear tier |

---

## 8. Design implications for PALIMPSEST

Opinionated. "Steal" = port the formula as written; "Adapt" = keep the structure, change the numbers or the clock.

**Identification and appraisal (three layers, no tedium)**

1. **Steal Barony 5.0's rule: the base item type is never hidden; only its magic is.** A dropped rifle reads "Rifle". Its beatitude, upgrade level, affixes and curse stay unknown until appraised. This removes the worst ID tedium in a real-time co-op shooter.
2. **Consumables get NetHack/SPD randomized appearances, re-skinned per genre.** Each run seed maps 12 appearance slots to 12 true types per consumable family (potion/scroll/ring analogues). The *slot* is the identity and the label is skinned per Folio: slot 3 is "crimson phial" in high fantasy, "crimson stim-ampoule" in cyberpunk, "crimson tonic" in weird west. Knowledge therefore carries across genre bleed, and the UI shows the cross-genre aliases once one is known.
3. **Equipment ID = Barony passive timer + SPD use-count, whichever finishes first.** Timer: `skill = Lore + 3*PER`, fast band 10 s, consumables ratio `max(0.2, 1 - skill/100)`, floor 2 s, and descending a floor advances every timer by 25–50%. Use-count: weapons 20 uses, armor 10 hits taken, implants and rings 1 character level of XP while equipped, with SPD's `availableUsesToID` refilled only by XP (half the requirement banked at most). Players cannot farm ID on a wall.
4. **Artifacts and cross-genre tech use CoQ examination.** Rating = INT + tool bonuses − difficulty, clamped −5..35, roll 0–10 successes against complexity 1–10. 0 successes breaks the item, partial success reveals an intermediate name, 10 always identifies. Gadget slots and workbench "chairs" give flat bonuses, as in CoQ.
5. **Price-ID is automatic.** Use NetHack-style price groups and CHA/Trading multipliers, but the game keeps an "Appraisal Ledger" that strikes impossible candidates from every price a player has seen. The deduction stays and the bookkeeping goes.
6. **Steal SPD's Shard of Oblivion as a risk dial**: +20% loot drop per unidentified equipped or recently used item (50-turn tracker, or 25 s real-time), capped by the item's level+1. Gambling on unknown gear becomes a choice, which is the healthy cousin of loot boxes.

**BUC, curses, upgrades**

7. **Beatitude is a signed int −3..+3 added directly to the item's primary stat** (Barony `attack = beatitude + base + (status-3)`). Generation: 30% of equipment cursed (SPD), with curses carrying a negative affix. Cursed gear cannot be unequipped. An upgrade clears the cursed flag and has a 33% chance to strip the curse affix (SPD).
8. **Condition tiers** Broken/Decrepit/Worn/Serviceable/Excellent (0..4) with `+ (status-3)` to damage/AC, plus degradation on use. This feeds crafting (repair) and the material system.
9. **Pace power with SPD's chapter quotas.** A "chapter" is 5 Folios. Per chapter: 3 upgrade catalysts, 2 attribute tonics, 1 enchant-transfer stylus. The drop chance on each floor is `Random(5 - floorInChapter) < remainingThisChapter`. Above +4, a good affix risks loss at `Random.Float(10) < 2^(lvl-4)`. That gives a soft cap with no hard wall.
10. **Loot uses SPD's double-deck generator.** Two alternating 35-card category decks, per-family decks for consumables, and artifacts drawn without replacement. This sits under the gacha/loot-box layer (separate dossier) as the variance-control spine.

**Attributes, skills, bodies**

11. **Primary attributes use CoQ arithmetic; sub-attributes live on limbs (IVAN).** Six primaries baseline 16, `mod = floor((v-16)/2)`. Arm Strength and Dexterity live per arm, Leg Strength and Agility per leg. Losing an arm drops its weapon, its implant slots and its Dexterity contribution. This is the "detailed attribute/sub-attribute system" the brief asks for, and it has mechanical teeth.
12. **Train by use with IVAN's asymptotic formula verbatim**: `Exp += (Natural*(100+Value) - 100*Old) * Speed * 2e-8`, attribute = `Exp*2e-8`, clamp 1..1000. Each action declares a `Value` (sprinting a long way +5, lifting near capacity +10, reading a hard tome to INT +20) and a `Speed`. Gains slow down near the cap and come for free relative to the species norm. Set NPC Speed ×1.5, as IVAN does, so companions keep up.
13. **Skills follow Barony's 0–100 model**: level up only on success, at most one level per distinct enemy for Stealth and Blocking, spell schools capped at `tier*20 + 20`, a minimum resource spend per eligible cast, and +2 character XP per skill-up so specialists still level. Add IVAN rust only to weapon-family skills, and never below the last multiple of 20.
14. **Limb HP from IVAN**: `HP = volume * END^2 / 200000`, using humanoid volume shares (head 4000 cm³; torso 13/30, arms 1/10, groin 1/10, legs 2/15 of the rest). Each scar costs −4% × severity. Slashing damage at ≤0 HP severs and drops the limb as an item that can be re-attached by surgery, a cyber-nook or prayer. Dismember chance on penetrating heavy blades comes from CoQ: 3% one-handed, 6% two-handed, 100% while berserk.

**Combat, stealth, survival**

15. **Kinetic damage uses CoQ penetration.** Roll the exploding `d[-1..8]` three times against `5 - PV + AV`; each penetration adds one set of damage dice, and a full triple repeats at PV−2. It works for swords, bullets, lasers and monomolecular blades (PV = AV+4) across every genre. Magic and elemental damage ignore AV and use multiplicative RES% (Barony 5.0, so two +30% sources make 51%). Barony's flat AC (cap 75%) is out.
16. **Port the SPD v4.0 stealth AI to real time.** Run one detection tick every 0.5 s with SPD's per-turn chances: sleeping `1/(d+S)^2` (zeroed below 10%), wandering inside a 75° cone within 6 m `1/(d/2+S)`, inside the 150° cone `1/(d+S)`, behind `1/(d+S)^3`. Add the INVESTIGATING state (moves to the noise, doesn't attack, −0.33 on its first tick). The stealth term S comes from skill, light level, noise and gear. Barony's charge-crit, flanking and backstab/assassination complete the kit.
17. **Hunger follows Barony's real-time model**: start at 1000, maximum 1500, lose 1 point per 30 ticks at 50 Hz (10 minutes per 1000). Multiply the loss rate by ×0.8 for 3 players and ×0.67 for 4. Use the freshness vomit table (0/4/10/25/100%). Starvation costs 5% max HP per tick.

**Crafting, alchemy, materials**

18. **One material schema for voxels and items (IVAN).** Fields: Density, StrengthValue, Flexibility, PriceModifier, Hotness, Acidicity, BurnModifier, Emitation, IntelligenceRequirement, plus Hardened and Softened chains. Weight is `vol*density/1000` and price is `PriceModifier*weight/10000`. Damage is `sqrt(5e-5 * Form * SV * sqrt(weight))` × (0.75..1.25), and to-hit is `10000/(1000+weight)`. A world voxel is a material plus a volume, so mining, crafting and pricing share one table. Each genre contributes rows such as plasteel, orichalcum and ectoplasm.
19. **Alchemy runs on SPD's Energy economy plus CoQ's drams.** Alchemizing a consumable yields 6 Energy and a minor item 3. Recipes take 1–3 ingredients plus an Energy cost from 0 to 16. Seed-style brewing follows SPD: all identical gives a guaranteed result that is auto-identified, 2 distinct ingredients give 25% random, 3 distinct give 50% random, and healing yields diminish with a counter. Liquids are tracked in drams, mix by volume share, evaporate below 200 drams, and react with heat (CoQ temperature thresholds: freeze 0, flame 350, vapor 10000).
20. **Item mods use the CoQ rules**: at most 3 mods per item, cost equal to the mod's component plus a component of tier `itemTier + modsAlready`, and spawn weights 2000/800/210/50/3. This is the Borderlands/Destiny affix layer with a crafting sink built in.

**Run structure, co-op, meta**

21. **Score uses the SPD formula with caps**: progress `lvl*deepest*65` capped at 50k, treasure capped at 20k, exploration `50*floors*%`, a fixed reward per boss with −100 per avoidable hit, a win multiplier of 2 (2.5 if ascended), and `round(1.25^challenges*20)/20`. Challenges are a bitmask. Seeds are 9 letters A–Z (26^9) plus a daily seed.
22. **Co-op follows Barony 5.0**: XP share 90/80/70% for 2/3/4 players, dead players become ghosts, friendly fire applies only to your own outgoing damage, and the Assist Shrine charges −10% score per assist point (capped at −95%, achievements off at ≥10 AP).
23. **Endless mode is ExpPD-style "Cycles"** with a piecewise virtual depth (`d*2+55`, `d*25+750`, …), boss treasure bags and XP-to-upgrade conversion. Store HP, damage and currency as 64-bit (or float64 with integer display) from day one, because ExpPD had to migrate mid-life.
24. **Factions and gods**: CoQ reputation bands ±250/±600 and a water-ritual-like **Pact** per Folio, with gossip and secrets as currency. Each world gets 5 generated "Authors" plus 1 fixed (the Sultan model) whose relics carry themed mods. Prayer follows IVAN's table verbatim (relation ±1000; timers +5000/+10000/+25000/+50000; Wisdom scaling), and sacrifice value is `sqrt(price)`, doubled for the matching patron.
25. **Mutations and implants use the CoQ economy**: 12 starting points, +1 per level, 4 points buys a choice of 3, rank cap `floor(L/2)+1` up to 10. Cyber genres use a license tier bought with credits (1/2/3/4 per point by band). Biopunk and cyberpunk Folios plug straight in.

**Avoid**: NetHack's manual haggling for price-ID (automate it), Barony's pre-5.0 "appraise every pebble" (hide only magic), runaway ExpPD inflation in the main mode (keep it to Cycles), and CoQ's under-documented regen formula (define our own, closed form).
