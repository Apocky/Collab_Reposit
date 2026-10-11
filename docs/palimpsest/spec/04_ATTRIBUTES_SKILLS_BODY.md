# 04 — Attributes, Skills, Progression and Body
As of 2026-10-11. Status: BINDING. Conforms to canon-1.2.

This document owns creation, sub-attribute growth, skills, levels, talents, the body, augments, kin, afflictions, needs, Death's Door, madness and player-controlled statuses. All of it runs in `palimpsest_sim`. Canon wins every conflict; conflicts are listed under Canon gaps.

## Imports

| Canon section | Consumed |
|---|---|
| §1.2–§1.6, §1.11 | numeric types, `floor_div`, `ceil_div`, `mul_bp`, display-unit rule, integer content, ascending-ID iteration |
| §2.2–§2.3 | Needs period 60 (phase 7), status pulse period 15, rate accumulator, `in_combat`, `light_at` |
| §3.3 | keyed `rng.progress` (offers) and `rng.combat` (Death's Door) |
| §4.1–§4.2 | humanoid capsule and speeds, jump, fall-damage hook, dodge |
| §5.3–§5.5, §5.7 | dependencies, `Req`, `Tol` (`talent_tol`, `gear_tol`), Contradiction, MYTHOS composure term |
| §6.1–§6.5 | attribute structure, scale, requirements, the 44 master rows |
| §7, §8 | 36 skills, `need(s)`, combat XP normalization, training marks, pace contract; level curve, `kill_xp_std`, XP sources, co-op split, points per level |
| §9, §9.1 | body plans, shares, condition pools, severing, hand bindings |
| §10.5–§10.9 | damage pipeline, lethal resolution, healing |
| §11, §12 | statuses owned here, buildup, direct application, player-controlled bearers; resources owned here, max-change rule |
| §13.8, §13.8.1, §15.1 | `P`, `T`, `S`, innate source level; minion cap |
| §16, §17 | Origins, Callings, Echoes, kin, afflictions; run constants, party tables, ranks, anchors, ghosts |

No other chapter exists yet; every cross-chapter dependency is listed under Interface assumptions.

---

## 1. Conventions and the modifier model

### 1.1 Notation

- `d(x) = x - 40` (Canon §6.2). "Row n" means Canon §6.4 row n.
- `d_cur` is the current Folio depth. `K(d) = kill_xp_std(d)` (Canon §8). One **unit** of character XP at depth `d` is `K(d)`. Fractional units are written in tenths: `u10` tenths of a unit give `floor_div(u10 × K(d), 10)` XP.
- Every doc-04 innate effect (Origin trait, kin trait, talent, mastery perk, Marginalia power) has dependency **none** unless its text says otherwise, and `src_lvl = d_cur` (Canon §13.8.1).
- Active abilities occupy one of four **ability slots** bound to `input.ability_1..=4` (doc 17). Cooldowns are in ticks, persist across Folios and never reset on `ev.folio_entered`.

### 1.2 Modifier kinds

Talents, perks, traits, augments, afflictions and wounds change stats only through these typed modifiers. The sim evaluates them in this fixed order:

| Kind | Meaning | Applied |
|---|---|---|
| `SubFlat(sub, n)` | adds `n` to the effective sub-attribute | before any row; effective value clamped `1..=150` (Canon §6.2) |
| `RowFlat(row, n)` | adds `n`, in the row's own unit, at the position of the row's `gear` term, inside its clamp | only rows listed "flat" below |
| `RowPost(row, bp)` | multiplies the row's final result | after the clamp, before status multipliers (Canon §11.7); only rows listed "post" |
| `DmgAdd(filter, bp)` | joins the Canon §10.5 step 1 additive bucket | per hit, scaled by its component potency |
| `TolAdd(axis, n)` | adds to `talent_tol[axis]` (Canon §5.4) | kin traits, Origin traits, talents and Marginalia powers; augments add to `gear_tol` |
| `BodyMove(bp)` | the body move factor (§8.5) | after row 11, before status multipliers |
| `Other` | a named effect with its own formula in its own text | as written |

Allowed rows: **flat** on rows 4, 7, 9, 10, 13, 14, 18, 19, 22, 23, 24, 25, 26, 27, 30, 31, 33, 34, 35, 36, 44 and on `res.breath` max (seconds). **post** on rows 2, 5, 6, 8, 15, 16, 17, 20, 21, 28, 37, 38, 40. Rows 1, 3, 11, 12, 29, 32, 39, 41, 42 and 43 take no doc-04 modifier: they change only through their inputs.

All `RowPost` factors on one row multiply from 10000 in ascending source content-ID byte order, one `mul_bp` each. The product is clamped to `5000..=20000` and then applied once. Example: `talent.gunslinger.steady_aim` (row 15 × 9000) and `aug.eye_targeting` (row 15 × 9000) give `mul_bp(mul_bp(10000, 9000), 9000) = 8100`; spread `10000` becomes `8100`.

Every numeric modifier from an augment or other dependent component is first multiplied by that component's Contradiction potency (Canon §5.5) and is 0 when Disconnected. A binary effect (an immunity, an added part, an unlocked action) is on while `C <= 5` and off when Disconnected.

---

## 2. Character creation

### 2.1 Flow

```
CreationState: ChoosePath → (Custom: Kin → Origin → Calling → PointBuy → Cosmetics → Confirm)
                          → (Echo: PickEcho → Cosmetics → Confirm)
Confirm → spawn at run start (folio_index 0) with kit, skills, subs, traits, level 1, XP 0
Back returns one step; later choices persist while still valid.
```

- **Custom** builds a character from parts. **Quick start** is Custom with the Calling's preset (§2.5) auto-filled in PointBuy; the player confirms it unchanged.
- **Echo** picks an owned Echo (doc 14/15). The Echo fixes kin, Origin, Calling, allocation, signature trait and cosmetics.
- Secret Origins (`origin.oneiric`, `origin.redacted`, `origin.unwritten`, `origin.ur_text`) appear only when the account holds their unlock (doc 14).

### 2.2 Point-buy

1. Every sub-attribute starts at **40**.
2. Add the Origin modifiers (§2.3). Each is within ±6 and each Origin row sums to 0 (Canon §16.1).
3. Spend **free points**: 10, or 14 for `kin.human` (§2.4), or 14 for `origin.unwritten`, or 18 for both.
4. Raising a sub-attribute costs 1 point per +1 while the new value is at most 50, and 2 points per +1 from 51 to 60.
5. Lowering a sub-attribute refunds 1 point per −1. At most 10 points may be refunded in total. No value may go below 25.
6. Final values must lie in `25..=60` (inside Canon's `20..=60`). Confirm is blocked while points remain unspent.
7. Skills: every skill 0; Calling majors 20 and minors 10 (Canon §16.2); the Origin skill +10; `kin.human` +5 to one skill of choice. Sums stack.

```rust
fn buy_cost(from: i32, to: i32) -> i32 {             // to > from
    (from + 1..=to).map(|v| if v <= 50 { 1 } else { 2 }).sum()
}
```

Example: raising LOG from 43 to 50 costs 7. Raising AGI from 48 to 54 costs `2 × 1 + 4 × 2 = 10`.

### 2.3 Origins

The Origin genre sets tolerance (Canon §5.4), the modifiers below, +10 to the Origin skill, and one Origin trait. IDs are `origin.<genre_suffix>`; each record names its `genre.` ID.

| Origin | Modifiers (all others 0) | Origin skill |
|---|---|---|
| `origin.mythic_fantasy` | PWR +3, ATN +3, RSV +2, PRV +2, LOG −3, SLT −2, GUI −3, ANL −2 | evocation |
| `origin.gothic_horror` | RSV +4, PRV +3, ITU +3, CHM −3, AGI −2, SER −2, END −3 | devotion |
| `origin.cosmic_horror` | ANL +4, MEM +3, HRG +3, RSV −4, CHM −3, PWR −3 | inscription |
| `origin.space_opera` | PRC +3, LOG +3, CHM +2, FAT +2, GRP −3, RES −3, ATN −2, GUI −2 | handling |
| `origin.cyberpunk` | SLT +3, LOG +3, GUI +2, SGT +2, RES −3, PRV −3, CHM −2, END −2 | hacking |
| `origin.post_apocalypse` | END +3, RES +4, SER +3, CHM −4, MEM −3, ATN −3 | survival |
| `origin.weird_west` | PRC +4, MEN +3, FAT +3, MEM −3, ANL −3, CHM −2, LOG −2 | marksmanship |
| `origin.hardboiled_noir` | ITU +4, GUI +3, TGH +3, CHM −3, FAT −3, ATN −4 | awareness |
| `origin.wuxia` | AGI +4, FOC +3, GRP +3, GUI −3, LOG −3, BRN −2, SLT −2 | acrobatics |
| `origin.steampunk` | LOG +3, ANL +3, SLT +2, BRN +2, AGI −3, ITU −3, ATN −2, FAT −2 | tinkering |
| `origin.fairy_tale` | CHM +4, SER +3, PRV +3, PWR −3, TGH −3, MEN −4 | speech |
| `origin.mythic_antiquity` | PWR +3, BRN +3, MEN +2, FAT +2, SLT −3, LOG −3, GUI −2, HRG −2 | polearm |
| `origin.oneiric` (secret) | ATN +4, ITU +3, MEM +3, END −3, TGH −3, GRP −4 | illusion |
| `origin.redacted` (secret) | ANL +3, GUI +3, RSV +2, MEM +2, CHM −3, FAT −3, SER −2, ATN −2 | security |
| `origin.unwritten` (secret) | none | player's choice |
| `origin.ur_text` (secret) | FOC +2, RSV +2, ATN +2, MEM +2, ITU +2, SER −2, GUI −2, MEN −2, SLT −2, BRN −2 | divination |

Origin traits (`trait.origin.*`, dependency none):

1. `destined` (Mythic Fantasy): killing a hostile of rank elite or higher applies `st.inspired` for 600 ticks.
2. `monster_hunter` (Gothic Horror): `DmgAdd +1500` against targets tagged undead or fiend; composure losses from undead or fiend sources × 7500.
3. `already_broken` (Cosmic Horror): all composure losses × 8000; while `st.madness`, `DmgAdd +1500` on every hit.
4. `spacer` (Space Opera): `res.breath` max +10 s; while `st.mounted`, `RowPost(15, 7700)`, cancelling the Canon §17.7 mounted spread penalty (`mul_bp(13000, 7700) = 10010`).
5. `wired` (Cyberpunk): augment total capacity +2 (§9.2); hacking action time × 8000.
6. `scavenger` (Post-Apocalypse): satiation loss × 7500 (§11.1); stale and rotten food add no `st.diseased` buildup.
7. `quickdraw` (Weird West): weapon swaps take `max(1, ceil_div(swap_ticks × 5000, 10000))` ticks; the first hit within 60 ticks after a swap completes gets `DmgAdd +2500`.
8. `gumshoe` (Hardboiled Noir): row 35 rolls on ticks where `(t + observer_phase) % 30 == 0` instead of 60, with the same `seq` rule; appraisal time × 5000 (doc 10).
9. `qinggong` (Wuxia): one extra air jump per airborne period at height `mul_bp(JUMP_H, 6000)`, in addition to Canon's VERVE-9 air jump; fall damage × 7500.
10. `gearwright` (Steampunk): your deployed `cat.gadget` entities have Max HP × 12500 and recharge × 12000.
11. `courtesy` (Fairy Tale): Speech and Handling checks +10 rating; native NPCs tagged fae or beast_sapient start one stance band friendlier toward you (Canon §15).
12. `favored` (Mythic Antiquity): favor gains × 12500 after the Canon §16.5 Devotion term; prayers to `patron.mythic_antiquity` count as prayers to the local patron (× 15000).
13. `lucid` (Oneiric): immune to `st.sleeping`; the `st.confused` rotation angle drawn by doc 06 is halved (`floor_div(angle, 2)`).
14. `cleared` (The Redaction): resist `dmg.arcane` +2500 and `dmg.psychic` +2500; composure losses from `fac.editors` sources × 5000.
15. `blank_page` (The Unwritten): the Origin skill is any one skill chosen at creation; +4 free points.
16. `first_draft` (The Ur-Text): `TolAdd(verve, 1)`; `RowPost(28, 11000)`.

### 2.4 Kin

Kin traits have dependency none (Canon §16.3). Skin text keys are `loc.kin.<kin>.skin.<genre_suffix>.name`.

**`kin.human`** (`body.humanoid`). No body or resource rule changes. `trait.kin.human.versatile`: +4 free creation points and +5 to one skill of choice at creation.

**`kin.construct`** (`body.humanoid`; a construct Erratum is not `body.construct`, so `st.hacked` never applies).
- `machine_body`: immune to `st.poisoned`, `st.diseased`, `st.bleeding`; needs no breath (`res.breath` never drains).
- `fuel_cell`: `res.satiation` is displayed as Fuel with the same range, decay and thresholds; only `cat.material` items tagged `fuel` restore it (§11.2); food is refused.
- `conductive`: resist `dmg.shock` −2500.
- `poor_chemistry`: healing from `cat.potion` items × 5000.
- `self_repair`: a 120-tick repair action consuming one repair kit (`cat.tool` tagged `repair_kit`) heals `mul_bp(max_hp_mhp, 1000 + 20 × skill.tinkering)`.
- `plated`: natural armor `mul_bp(3, T(d_cur))` on every part.

**`kin.undead`** (`body.humanoid`).
- `deathless`: no natural HP regeneration (row 2 is 0); immune to `st.poisoned`, `st.diseased`; needs no breath.
- `umbral_communion`: at Canon §10.5 step 8, the `dmg.umbral` component of a hit is not subtracted from HP or the part pool; instead the bearer heals `mul_bp(umbral_mhp, 5000)` through Canon §10.9.
- `light_averse`: resist `dmg.radiant` −5000.
- `grave_appetite`: satiation loss × 5000; only meat, `cat.body_part`, `cat.corpse` and blood restore satiation.
- `unfeeling`: composure losses from undead and fiend sources × 5000.

**`kin.beastkin`** (`body.humanoid` plus `bp.tail`, share 500 taken from `bp.torso`).
- `claws`: unarmed hits carry a 5000 Bp `dmg.slash` share; the rest keeps the unarmed profile.
- `night_eyes`: `RowFlat(31, 2)`.
- `tail_balance`: while `bp.tail` is attached, `RowFlat(18, 5)`.
- `keen_nose`: `RowFlat(33, 3)`.
- `fast_metabolism`: satiation loss × 12500.

**`kin.fae`** (`body.humanoid`).
- `glamour_born`: `TolAdd(arcana, 1)`.
- `iron_bane`: hits from an item whose primary material is `mat.iron` or `mat.cold_iron` are × 12500, applied to every type share at Canon §10.5 step 3a.
- `light_step`: fall damage × 7000; footstep noise −3 dB (doc 09).
- `fair_folk`: `RowPost(40, 8000)`.

**`kin.mutant`** (`body.humanoid`).
- `unstable_genome`: at creation, one mutation chosen from 3 offered (§9.3); +1 mutation point at every even level.
- `adaptive`: resist `dmg.radiation` +2500.
- `mutable_flesh`: augment total capacity +2, usable only by mutations.

Kin skins per Origin, in the order human, construct, undead, beastkin, fae, mutant:

- mythic_fantasy: Human, Golem, Revenant, Wolfkin, Elf-blooded, Chaos-touched.
- gothic_horror: Mortal, Automaton, Ghoul, Werekin, Changeling, Abomination.
- cosmic_horror: Survivor, Homunculus, Ghast, Tidekin, Twilight-born, Reshaped.
- space_opera: Terran, Android, Reanimant, Felinoid, Starborn, Gene-splice.
- cyberpunk: Baseline, Synth, Rezzed, Spliced, Netborn, Biohack.
- post_apocalypse: Survivor, Scrapbot, Ghoul, Mutt-kin, Wasteling, Mutant.
- weird_west: Drifter, Clockwork Deputy, Revenant, Skinwalker, Spirit-touched, Hexed.
- hardboiled_noir: Citizen, Tin Man, Stiff, Alley-cat, Moth, Freak.
- wuxia: Mortal, Bronze Warrior, Jiangshi, Fox-blooded, Xian-touched, Gu-cursed.
- steampunk: Commoner, Clockwork, Galvanized Dead, Beastman, Sylph, Experiment.
- fairy_tale: Peasant, Tin Soldier, Restless Dead, Beast-cursed, Fairy, Transformed.
- mythic_antiquity: Mortal, Bronze Automaton, Shade, Satyr, Nymph-born, Chimera-born.
- oneiric: Dreamer, Marionette, Sleepwalker, Totem-kin, Dream-fae, Shifting.
- redacted: Citizen, Unit, Archived Dead, Specimen, Anomaly, Variant.
- unwritten: Blank, Draft Engine, Erased, Sketch-beast, Doodle-fae, Scribble.
- ur_text: First Person, First Machine, First Dead, First Beast, First Fair, First Change.

### 2.5 Callings: presets and kits

Majors and minors are Canon §16.2. Each preset spends 10 points at 1 each. Every kit also holds 2 rations, 2 bandages, 1 type-identified healing potion and `mul_bp(50, P(1)) = 50` Lucre. Kit items are Common, Normal, +0, ilvl 1, `buc.uncursed`, `know.identified`, skinned to the Origin genre; doc 11 maps each role to an item ID.

- **vanguard**: preset PWR +3, TGH +3, BRN +2, RSV +2; kit: 1H hafted weapon, medium shield, heavy armor (head, body, legs).
- **duelist**: preset PRC +3, AGI +3, CHM +2, FAT +2; kit: 1H long blade, 6 throwing knives, light armor set.
- **gunslinger**: preset PRC +4, AGI +2, SLT +2, FAT +2; kit: sidearm with 60 rounds, 4 throwing blades, light armor set.
- **grenadier**: preset GRP +3, TGH +3, SLT +2, PWR +2; kit: heavy ranged weapon with 120 rounds, 4 grenades, medium armor set.
- **spellwright**: preset ATN +3, LOG +3, FOC +2, MEM +2; kit: casting focus, glyph tome (2 glyphs), light armor set.
- **alchemist**: preset ANL +4, SLT +2, RES +2, LOG +2; kit: dagger, field alembic, 3 bombs, 4 empty flasks.
- **shade**: preset AGI +4, PRC +2, SLT +2, GUI +2; kit: short blade, 5 lockpicks, light armor set, smoke bomb.
- **artificer**: preset LOG +4, BRN +2, PRC +2, ANL +2; kit: sidearm with 60 rounds, toolkit, 1 deployable turret, medium armor.
- **warden**: preset BRN +3, RES +3, RSV +2, TGH +2; kit: spear, tower shield, medium armor set, 1 healing glyph.
- **oracle**: preset ITU +3, PRV +3, MEM +2, FOC +2; kit: staff, holy symbol (prayer focus), light armor set.
- **netrunner**: preset LOG +4, AGI +2, HRG +2, SLT +2; kit: sidearm with 60 rounds, hacking deck, light armor set.
- **beastbinder**: preset CHM +3, SGT +3, MEM +2, END +2; kit: spear, 1 tamed companion beast, light armor set.
- **monk**: preset PWR +3, AGI +3, END +2, FOC +2; kit: hand wraps (unarmed archetype), cloth armor set.

### 2.6 Echo path

An Echo record (doc 14) fixes `kin`, `origin`, `calling`, a full allocation that passes §2.2 validation, a signature trait `trait.echo.<echo_name>` (doc 14 authors these inside this namespace, §Canon gaps) and a body plan, default `body.humanoid` (§13). The Echo receives its kit and the same skill starts as Custom.

### 2.7 Creation data

```json
{
  "id": "origin.cyberpunk",
  "genre": "genre.cyberpunk",
  "mods": { "sub.sleight": 3, "sub.logic": 3, "sub.guile": 2, "sub.sight": 2,
            "sub.resilience": -3, "sub.providence": -3, "sub.charm": -2, "sub.endurance": -2 },
  "origin_skill": "skill.hacking",
  "origin_skill_bonus": 10,
  "trait": "trait.origin.wired",
  "secret": 0
}
```

```json
{
  "id": "kin.beastkin",
  "body_plan": "body.humanoid",
  "added_parts": [ { "part": "bp.tail", "share_bp": 500, "from": "bp.torso", "hit_mult_bp": 8500, "precision": 0 } ],
  "traits": [ "trait.kin.beastkin.claws", "trait.kin.beastkin.night_eyes", "trait.kin.beastkin.tail_balance",
              "trait.kin.beastkin.keen_nose", "trait.kin.beastkin.fast_metabolism" ],
  "creation_free_points_bonus": 0
}
```

Validation: Origin mods sum to 0 and each is in `-6..=6`; every final sub-attribute is in `25..=60`; spent points equal the budget; refunds are at most 10.

---
## 3. Sub-attributes

Base values come from creation, level-up points (§6.4) and training marks (Canon §7.1); effective values add `SubFlat`. Each entry gives a description, governed skills, Canon rows, doc-04 uses and a tooltip. Tooltip placeholders: `{v}` effective value, `{b}` base value, `{m}` training marks and `{t}` marks needed `T(b)`; `{rN}` is row N formatted per Canon §1.5. Every tooltip ends with the shared suffix `loc.sub.common.training` = "Training {m}/{t}".

**MIGHT** (`attr.might`)

- **Power** (`sub.power`, PWR). Raw force. Governs Hafted, Unarmed. Rows 10 (carry), 20 (throw range), 12 when a weapon's coefficients name PWR. Doc 04: Force checks (§4.2). Tooltip `loc.sub.power.tooltip`: "Power {v}: carry {r10} kg, throw range ×{r20}."
- **Brawn** (`sub.brawn`, BRN). Mass and stance. Governs Shield, Smithing. Rows 18 (poise), 19 (block stability). Doc 04: Lift checks. Tooltip: "Brawn {v}: poise {r18}, block stability {r19}."
- **Grip** (`sub.grip`, GRP). Hand strength and hold. Governs Polearm, Heavy Arms. Rows 16 (recoil), 21 (climb speed). Doc 04: Climb checks. Tooltip: "Grip {v}: recoil ×{r16}, climb speed ×{r21}."

**FINESSE** (`attr.finesse`)

- **Precision** (`sub.precision`, PRC). Accuracy. Governs Blade, Marksmanship. Rows 13 (crit, 5 Bp per point), 15 (spread). Tooltip: "Precision {v}: spread ×{r15}, crit {r13}."
- **Agility** (`sub.agility`, AGI). Whole-body speed. Governs Acrobatics, Sneak. Row 11 (move speed); dodge invulnerability ticks (Canon §4.2). Doc 04: Squeeze and Balance checks. Tooltip: "Agility {v}: move ×{r11}, dodge invulnerability {iframes} ticks."
- **Sleight** (`sub.sleight`, SLT). Hand speed. Governs Throwing, Security. Row 17 (reload, swap, use). Tooltip: "Sleight {v}: reload, swap and use speed ×{r17}."

**VIGOR** (`attr.vigor`)

- **Toughness** (`sub.toughness`, TGH). Body mass and pain tolerance. Governs Armor. Rows 1 (Max HP, and HP per level), 23 (nerve threshold). Tooltip: "Toughness {v}: Max HP {r1}, +{hp_per_level} per level, nerve threshold {r23}."
- **Endurance** (`sub.endurance`, END). Stamina and breath. Governs Athletics, Delving. Rows 4, 5, 6; `res.breath` max `10 + floor_div(END, 10)` s (Canon §12). Doc 04: temperature comfort band (§11.4); Endure checks. Tooltip: "Endurance {v}: stamina {r4}, regen {r5}/s, breath {breath} s, comfort {lo}..{hi} °C."
- **Resilience** (`sub.resilience`, RES). Recovery. Governs Restoration. Rows 2, 3, 22; augment total capacity (§9.2). Tooltip: "Resilience {v}: regen {r2}/s, healing received ×{r3}, body threshold {r22}, augment capacity {augcap}."

**INTELLECT** (`attr.intellect`)

- **Logic** (`sub.logic`, LOG). Structured reasoning. Governs Spellcraft, Tinkering, Hacking. Row 26 (complexity budget). Doc 04: Decipher checks. Tooltip: "Logic {v}: spell complexity {r26}."
- **Memory** (`sub.memory`, MEM). Retention. Governs Conjuration, Inscription. Rows 27 (grimoire), 28 (skill XP multiplier). Doc 04: Recall checks. Tooltip: "Memory {v}: grimoire slots {r27}, skill XP ×{r28}."
- **Analysis** (`sub.analysis`, ANL). Diagnosis. Governs Alchemy, Medicine. No Canon row. Doc 04 rows: treatment speed term `50 × d(ANL)` (§8.4); appraisal speed `anl_speed_bp = clamp(10000 + d(ANL) × 100, 5000, 20000)` consumed by doc 10; Investigate checks. Tooltip: "Analysis {v}: treatment speed ×{treat}, appraisal speed ×{anl_speed}."

**WILL** (`attr.will`)

- **Focus** (`sub.focus`, FOC). Concentration. Governs Alteration. Rows 7 (max aether), 30 (cast interruption). Tooltip: "Focus {v}: aether {r7}, keep casting when hit {r30}."
- **Resolve** (`sub.resolve`, RSV). Nerve. Governs Abjuration. Rows 9 (max composure), 24 (mind threshold). Doc 04: Steady checks. Tooltip: "Resolve {v}: composure {r9}, mind threshold {r24}."
- **Attunement** (`sub.attunement`, ATN). Sympathy with genre law. Governs Evocation. Rows 8, 25, 29; `attune_tol = clamp(floor_div(ATN - 40, 30), 0, 3)` (Canon §5.4). Tooltip: "Attunement {v}: aether regen {r8}/s, arcane threshold {r25}, spell potency ×{r29}, tolerance +{attune_tol}."

**PERCEPTION** (`attr.perception`)

- **Sight** (`sub.sight`, SGT). Vision. Governs Survival. Rows 31 (dark-vision), 32 (visual detection). Doc 04: Spot checks. Tooltip: "Sight {v}: dark-vision {r31} m, detection +{r32}."
- **Hearing** (`sub.hearing`, HRG). Governs Awareness. Rows 33, 34. Tooltip: "Hearing {v}: hearing +{r33} dB, footstep radius {r34} m."
- **Intuition** (`sub.intuition`, ITU). Pattern sense. Governs Divination, Appraisal. Rows 35, 36; three exit offers at ITU ≥ 70 (Canon §17.1). Doc 04: Sense checks. Tooltip: "Intuition {v}: trap and secret detection {r35}/s, curse sense {r36}."

**PRESENCE** (`attr.presence`)

- **Charm** (`sub.charm`, CHM). Governs Speech, Handling. Rows 37, 38. Doc 04: Persuade and Tame checks. Tooltip: "Charm {v}: buy ×{r37}, sell {r38} of value."
- **Menace** (`sub.menace`, MEN). Governs Leadership. Row 39 (threat). Doc 04: Intimidate checks. Tooltip: "Menace {v}: threat ×{r39}."
- **Guile** (`sub.guile`, GUI). Governs Illusion, Pickpocket. Row 40 (disguise). Doc 04: Deceive checks. Tooltip: "Guile {v}: disguise suspicion ×{r40}."

**FORTUNE** (`attr.fortune`). In-run effects only; never paid odds or account currency (Canon §22).

- **Fate** (`sub.fate`, FAT). Governs no skill. Rows 13 (crit, 10 Bp per point), 41 (status buildup). Doc 04: Fortune checks. Tooltip: "Fate {v}: crit {r13}, status buildup ×{r41}."
- **Serendipity** (`sub.serendipity`, SER). Governs no skill. Row 42 (loot rarity shift, Canon §13.4). Tooltip: "Serendipity {v}: loot rarity shift {r42}."
- **Providence** (`sub.providence`, PRV). Governs Devotion. Rows 43 (Death's Door), 44 (curse resistance). Tooltip: "Providence {v}: Death's Door {r43}, curse resistance {r44}."

---

## 4. Primary attributes and checks

### 4.1 Uses

Primaries (Canon §6.1, from effective values) are used for:

1. **Item requirements** (Canon §6.3): shortfall `s` multiplies damage, attack speed and spread recovery by `10000 - min(5000, s × 100)`.
2. **Augment requirements** (§9): an augment with `req` cannot be installed below it; if a primary later falls below it, the augment counts as Strained C = 2 (potency 8000) until the requirement is met again.
3. **Talent gates** (§7.1): tier 3 needs the Calling's key primary ≥ 45 and tier 4 needs ≥ 50.
4. **Primary checks**: broad checks name a primary instead of a sub-attribute.
5. **Display tier**: ≤ 29 Poor, 30–39 Weak, 40–49 Average, 50–64 Good, 65–79 Great, 80–99 Heroic, ≥ 100 Mythic.

### 4.2 Non-combat checks

```
rating    = attr_value + (if skill named { floor_div(skill × 6, 10) } else { 0 }) + Σ situational
chance_bp = clamp(5000 + (rating - DC) × 200, 500, 9500)
success   = keyed_rng("rng.check", folio, actor, tick, seq).roll_bp(chance_bp)    // exactly 1 draw
```

`attr_value` is the named effective sub-attribute or primary. Situational modifiers: talent and trait bonuses as written; `st.exhausted` −10; `st.madness` −10; each untreated wound of tier ≥ 2 on the part the check uses (arms for Force, Lift, Climb, Squeeze; legs for Balance; head for Recall, Decipher, Investigate) −5.

Difficulty ladder:

| Tier | DC | Tier | DC |
|---|---|---|---|
| Trivial | 10 | Severe | 70 |
| Easy | 25 | Heroic | 85 |
| Standard | 40 | Legendary | 100 |
| Hard | 55 | Mythic | 120 |

A failed check against the same target (entity, item uid or block position) cannot be retried for 600 ticks unless the rating has risen. Successful Persuade, Intimidate, Deceive and Tame checks grant skill XP `100 × DC` milli-XP to the named skill (§5.2).

Check catalog (docs 12 and 13 place the targets and their DCs):

| Check | Attribute + skill | Use |
|---|---|---|
| Force | PWR | stuck doors, bars, lids; failure costs 5 stamina |
| Lift | BRN | move a block-entity heavier than carry capacity |
| Climb | GRP + athletics | climb a non-climbable surface tagged `handholds` |
| Squeeze | AGI + acrobatics | pass a gap tagged `tight` |
| Balance | AGI + acrobatics | cross a ledge or ice span tagged `balance` without falling |
| Endure | END + athletics | resist a scripted exposure event |
| Shrug | RES | resist a scripted toxic or sickening event |
| Recall | MEM + appraisal | reveal a creature's resistance row in the bestiary |
| Decipher | LOG + inscription | read ciphers and puzzle texts |
| Investigate | ANL | find a mechanism's trigger |
| Steady | RSV | resist a scripted horror event (failure: composure −10) |
| Sense | ITU + awareness | sense an ambush trigger before it fires |
| Spot | SGT + survival | see a distant marked detail |
| Persuade | CHM + speech | dialogue, pacify a sapient hostile (§6.1) |
| Intimidate | MEN + leadership | dialogue, pacify by fear |
| Deceive | GUI + speech | dialogue, bluff a disguise check (doc 09) |
| Tame | CHM + handling | tame a beast (§5.3 Handling) |
| Fortune | FAT | scripted luck events |

Example: Force with PWR 46 against a Hard bar (55): `5000 + (46 - 55) × 200 = 3200` Bp. Persuade with CHM 38 and Speech 30: `rating = 38 + 18 = 56`; against Standard 40, `5000 + 16 × 200 = 8200` Bp.

---
## 5. Skills

### 5.1 Growth engine

```rust
fn grant_skill_xp(c: &mut Erratum, s: SkillId, milli: i64) {
    if milli <= 0 { return; }                                   // zero-effect actions grant 0
    let m = mul_bp(milli, row28_with_posts(c));
    if c.skill[s] >= 100 { return; }
    c.skill_prog[s] += m;
    while c.skill[s] < 100 && c.skill_prog[s] >= need(c.skill[s]) {   // need(s) = 4000 + 400 s
        c.skill_prog[s] -= need(c.skill[s]);
        c.skill[s] += 1;
        emit(ev.skill_gained { entity: c.id, skill: s, value: c.skill[s] });
        if [25, 50, 75, 100].contains(&c.skill[s]) { grant_perk(c, perk_id(s, c.skill[s])); }
        let g = governing(s);                                   // Canon §7
        c.marks[g] += 1;
        if c.marks[g] >= 2 + floor_div(c.base[g], 20) {        // T(v), Canon §7.1
            if c.base[g] < 100 { c.base[g] += 1; }
            c.marks[g] = 0;
        }
        if c.folio_skillups_counted < 8 {                       // Canon §8
            c.folio_skillups_counted += 1;
            grant_char_xp(c, 2 * K(d_cur), XpSource::SkillUp);
        }
    }
    if c.skill[s] == 100 { c.skill_prog[s] = 0; }
}
```

- `folio_skillups_counted` resets to 0 on `ev.folio_entered`.
- A skill tome (`cat.book`, doc 06/07) grants `need(skill)` milli-XP, once per tome type per run.
- Anti-grind: the same lock, trap, device, item uid or corpse yields XP once per run; allies, neutrals, own minions and own placed blocks give 0; movement XP has the §5.2 Folio caps.

### 5.2 XP events

Values are milli-XP before row 28. `dmg` is HP actually removed after ward (Canon §10.5 step 7), `max` the recipient's Max HP in mHP, `val_n = floor_div(value × 10000, P(d_cur))` (an item's value normalized to depth 1), `cost` a spell or prayer base aether cost in display units, `DC` the check or lock difficulty (doc 09 locks use the §4.2 ladder values).

| Skill | Event | milli-XP |
|---|---|---|
| blade, hafted, polearm, unarmed, marksmanship, heavy_arms, throwing | hit on a hostile with that skill's archetype (doc 05 mapping) | `min(1500, floor_div(1500 × dmg, max))` (Canon §7.1) |
| throwing | thrown non-damaging item takes effect on ≥ 1 hostile | 300 |
| shield | hit blocked (pre-block total) | `min(1500, floor_div(3000 × blocked, own_max))`; perfect parry × 2, same cap |
| armor | hit taken on a part with AR > 0 (armor-type share after step 4) | `min(1000, floor_div(2000 × d_armor, own_max))` |
| acrobatics | dodge whose invulnerability negates ≥ 1 hit | 500 per dodge |
| acrobatics | landing with `fall_height ≥ SAFE_FALL - 1000 mm` | `min(1500, 200 + floor_div(fall_height - SAFE_FALL + 1000, 10))` |
| athletics | stamina spent on sprint, swim or climb | 30 per 1 stamina; cap 30,000 per Folio |
| evocation, abjuration, alteration, conjuration, illusion, restoration, divination | effective cast of a spell of that school | `100 × cost` |
| conjuration | hit by your spell summon or raised minion | `min(750, floor_div(750 × dmg, max))` |
| spellcraft | effective cast of a spell with `g ≥ 2` glyphs | `40 × cost × (g - 1)` |
| spellcraft | first cast of a new self-designed spell this run | `2000 × g` |
| devotion | favor gained | 100 per favor point |
| devotion | prayer answered pleased or helped | 4000 |
| devotion | effective prayer-component activation | `100 × cost` |
| sneak | each Needs tick while crouched or `st.hidden`, moving, with hostiles within 10 m below `aware.alert` | `40 × min(3, n)` |
| sneak | sneak-attack kill | 1500 |
| sneak | Folio exit that earns the Unseen bonus | 10,000 |
| security | lock picked or trap disarmed | `150 × DC` |
| pickpocket | item stolen from a hostile or neutral | `min(8000, 1500 + 40 × val_n)` |
| pickpocket | item planted | 1500 |
| smithing, tinkering, alchemy, inscription | craft or brew completed (doc 07/08 recipe) | `1000 × input_units + 2000 × recipe_tier`, then × 15000 if quality ≥ `q.excellent` |
| smithing, tinkering | repair | 500 per 1000 Bp of max durability restored |
| smithing, tinkering | salvage | 300 per output unit |
| alchemy | potion type identified by brewing or tasting | 3000 |
| inscription | Rewrite performed | `2000 + 1000 × rarity_index` (common 0 … palimpsestic 6) |
| appraisal | instance reaches `know.appraised` or `know.identified` by your action | `min(6000, 1000 + 30 × val_n)` each step |
| appraisal | type identified; curse sensed | 3000; 2000 |
| medicine | wound treated | `1500 × tier` |
| medicine | part reattached; surgery; augment installed or removed | 8000; 6000; 4000 |
| medicine | healing item applied to another Erratum | `min(1500, floor_div(1500 × healed, max))` |
| survival | butcher output unit; forage harvest; cook | 800; 500; `1000 × input_units` |
| survival | each Needs tick outside the comfort band | 20; cap 6,000 per Folio |
| speech | trade completed | `5 × val_n` of the trade total |
| speech | Persuade or Deceive success | `100 × DC` |
| speech | reputation gained | 10 per point |
| leadership | ally hit while that ally is in your aura | `min(375, floor_div(375 × dmg, max))` |
| leadership | Intimidate success; Rally removes ≥ 1 status | `100 × DC`; 1500 |
| hacking | device hacked | `150 × DC` |
| hacking | construct hacked | `floor_div(3000 × rank_xp_bp, 10000)` |
| delving | block mined; block placed | `100 × hardness` (doc 07, 1..=10); 50; combined cap 40,000 per Folio |
| awareness | trap or secret detected by row 35; hidden hostile revealed by your perception | 3000; 2000 |
| handling | hit by your tamed beast, hireling or mount | `min(750, floor_div(750 × dmg, max))` |
| handling | Tame success | `floor_div(3000 × rank_xp_bp, 10000)` |
| handling | each Needs tick mounted and moving | 10; cap 10,000 per Folio |

An "effective" cast or activation changed something: damage dealt, HP restored, ward absorbed, a status applied or removed, a summon spawned, or an object revealed. Example: a 15-aether evocation that hits one hostile grants 1,500; at MEM 50 it is `mul_bp(1500, 10500) = 1575`.

### 5.3 Skill effects and mastery perks

Mastery perks are granted automatically on reaching 25, 50, 75 and 100 and are never lost. IDs: `skill.<name>.m25`, `.m50`, `.m75`, `.m100` (144 perks). Every school skill also has the Canon row 29 term (`30 Bp` per level) and an aether cost factor `10000 - 25 × skill` that doc 06 applies to that school's spells. Every combat skill has the Canon row 12 term (`30 Bp` per level).

**Combat**

- **Blade** (PRC). 25: blade hits +15 `st.bleeding` status power. 50: parry window +3 ticks with blades. 75: blade crits apply 1 `st.exposed` stack directly. 100: blade hits `pen_bp` +2000, capped 10000.
- **Hafted** (PWR). 25: heavy attacks +20 `st.stunned` power. 50: heavy-attack stamina × 8000. 75: hafted poise damage × 12500. 100: heavy hits on a `st.staggered` target apply `st.knocked_down` directly.
- **Polearm** (GRP). 25: melee reach +300 mm. 50: an attack hitting ≥ 2 hostiles gets `DmgAdd +1000` on each. 75: hitting a hostile that moved toward you at ≥ 4300 mm/s on the impact tick applies `st.staggered`. 100: while crouched or mounted, `DmgAdd +3000`.
- **Unarmed** (PWR). Bare hands are an innate weapon: `B = mul_bp(6000, P(d_cur))` mHP, 100% `dmg.blunt`, 24-tick swing, precision 12500. 25: unarmed stamina × 7500. 50: unarmed hits +25 `st.stunned` power. 75: empty-handed blocks act as a shield with stability 3000. 100: hits on `st.staggered` or `st.stunned` targets `DmgAdd +5000`.
- **Shield** (BRN). Parry window `6 + floor_div(skill, 10)` ticks (doc 05). 25: block stamina loss × 8500. 50: shield bash applies `st.staggered` to targets of rank standard or minion. 75: block arc +4096 Bam. 100: perfect parries reflect projectiles at their shooter (Canon §10.6 reflected entry).
- **Marksmanship** (PRC). 25: aimed spread `RowPost(15, 9000)`. 50: ranged reloads `RowPost(17, 11000)`. 75: ranged precision multiplier +2500 Bp. 100: the first shot after 60 ticks without firing crits; the crit roll is still drawn and its result replaced.
- **Heavy Arms** (GRP). 25: `RowPost(16, 9000)` for heavy weapons. 50: firing removes no move speed (doc 05 firing penalty off). 75: heavy-weapon explosion radius +500 mm. 100: heavy weapons fire while sprinting.
- **Throwing** (SLT). 25: `RowPost(20, 12000)` for thrown items. 50: throw action ticks × 8000. 75: thrown hits +20 status power. 100: one input throws 2 items from the stack, 1024 Bam apart.

**Defense and mobility**

- **Armor** (TGH). Worn armor counts `mul_bp(mass_g, 10000 - 40 × skill)` toward load. 25: armor durability loss × 7500. 50: tier-1 wounds have no effect (§8.3). 75: natural armor `mul_bp(5, T(d_cur))` on every part. 100: `RowFlat(18, 15)`; armor adds no footstep noise (doc 09).
- **Acrobatics** (AGI). Fall damage × `10000 - 60 × skill` (Canon §4.2). 25: dodge cost 17 stamina instead of 20. 50: wall jump, once per airborne period, dependency verve with `Req` from the Origin genre. 75: dodge invulnerability +2 ticks, still at most 12. 100: a dodge that negates a hit restores 10 stamina.
- **Athletics** (END). Sprint, swim and climb stamina × `10000 - 30 × skill` (as `RowPost(6, ·)` for sprint). 25: `res.breath` max +4 s. 50: `RowPost(5, 11000)`. 75: `RowFlat(10, 10)`. 100: immune to `st.exhausted`.

**Magic and faith**

- **Evocation** (ATN). 25: evocation hits +10 status power. 50: evocation projectile speed × 12500. 75: +1000 Bp crit chance on evocation hits (gear term). 100: holding a cast 60 extra ticks gives `DmgAdd +5000` at 2× aether cost.
- **Abjuration** (RSV). 25: wards you grant × 11000. 50: each abjuration cast removes `st.unraveled` and `st.silenced` from its target. 75: your `st.shielded` pools last until depleted or Folio exit. 100: your ward pools use `mul_bp(ward_mult, 8000)` for every type.
- **Alteration** (FOC). 25: alteration durations × 12500. 50: `st.levitating` vertical control × 15000. 75: one self-targeted alteration cast per 600 ticks costs 0 aether. 100: Transmute: turn one block into a material of equal or lower tier (doc 07), 120-tick cast, 30 aether.
- **Conjuration** (MEM). 25: summon Max HP × 11500. 50: summon duration × 15000. 75: summons add 2500 Bp of each of your resistances, within their caps. 100: raise a humanoid corpse dead under 600 ticks as a minion until Folio exit; 50 aether, 120-tick cast, counts toward the cap.
- **Illusion** (GUI). 25: your `st.charmed`, `st.feared`, `st.confused` durations × 12500. 50: illusion casts do not end `st.invisible`. 75: mind-group status power × 12500. 100: each illusion cast grants `st.invisible` for 60 ticks.
- **Restoration** (RES). 25: healing you cause × 11000, before row 3. 50: your heals treat one untreated wound of tier ≤ 2 on the target. 75: healing beyond Max HP becomes `st.shielded` up to 1000 Bp of Max HP. 100: once per Folio, a restoration spell on a downed ally revives it at once.
- **Divination** (ITU). 25: unknown items show their `idc` class and how many types of it remain unknown. 50: you see each hostile's awareness state toward you. 75: `RowFlat(35, 1000)`. 100: on `ev.folio_entered`, every `secret.` location is marked on your map.
- **Spellcraft** (LOG). Glyphs per spell `2 + floor_div(skill, 25)` (doc 06). 25: `RowFlat(26, 1)`. 50: self-designed spells' aether × 9000. 75: `RowFlat(27, 1)`. 100: `RowFlat(26, 2)`; self-designed spells' cast ticks × 8500.
- **Devotion** (PRV). Favor terms (Canon §16.5). 25: prayer timer increments × 9000. 50: curse state of items you pick up becomes known. 75: sacrifice value × 12500. 100: once per run, one prayer resolves as pleased.

**Stealth**

- **Sneak** (AGI). Input to doc 09 detection. 25: crouched movement noise −3 dB. 50: sneak-attack multiplier +5000 Bp (doc 09). 75: awareness decay toward you after line of sight breaks × 15000 (doc 09). 100: sneak attacks carry `ignore_armor` (Canon §10.5 step 5).
- **Security** (SLT). Input to doc 09 locks. 25: lockpick break chance × 5000. 50: disarmed traps become redeployable `cat.gadget` items. 75: you may attempt locks one ladder tier above your tier. 100: non-quest mechanical locks open in 60 ticks without picks.
- **Pickpocket** (GUI). 25: steal time × 7500. 50: plant items, primed throwables included. 75: steal equipped non-weapon items. 100: a failed steal sets awareness to `aware.suspicious`, never `aware.alert`.
- **Hacking** (LOG). Hack time × `10000 - 50 × skill` (doc 09). 25: hack cameras from 10 m. 50: hacked constructs `DmgAdd +2000`. 75: two hacked units at once. 100: `st.hacked` on non-boss constructs lasts until Folio exit.
- **Awareness** (HRG). Row 35 term and doc 09's opposed check. 25: `RowFlat(34, 4)`. 50: `RowFlat(33, 5)`. 75: you see `st.invisible` entities within 4 m. 100: hostiles within 6 m that are hidden from your party gain `st.marked` for you (8 s, refreshed each Needs tick).

**Craft**

- **Smithing** (BRN). Input to doc 07 quality. 25: repairs restore × 15000. 50: a crafted weapon or armor rolling `q.normal` becomes `q.good`. 75: reforge one affix at a forge (doc 07 cost). 100: crafted gear max durability × 20000 and `material_bp` +500, capped at 14000.
- **Tinkering** (LOG). Input to doc 07 gadget quality. 25: gadgets +1 charge. 50: turret Max HP × 12500. 75: craft `cat.implant` items at a workbench (§9.3). 100: gadget recharge × 15000.
- **Alchemy** (ANL). Input to doc 08. 25: brewing 2 or more of one potion identifies its type. 50: brewed potency +1000 Bp. 75: each brew yields +1 potion. 100: potions you drink add 0 toxicity.
- **Inscription** (MEM). Input to doc 06/07 scrolls and `ench.`. 25: scribed scrolls × 12000 potency. 50: +1 enchantment slot on items you enchant. 75: Scriptorium Rewrite cost × 5000. 100: once per Folio, Rewrite without a Scriptorium.
- **Delving** (END). Dig and place speed × `10000 + 100 × skill` (doc 07). 25: mined blocks of hardness ≥ 3 yield +1 unit. 50: mine one tool tier above your tool. 75: blocks you place have × 15000 HP. 100: block placement takes 1 tick.

**Knowledge**

- **Appraisal** (ITU). Input to doc 10. 25: appraised items show exact value. 50: items reach `know.appraised` on pickup. 75: `RowFlat(36, 2000)`. 100: equipment you carry becomes `know.identified` after 600 ticks.
- **Medicine** (ANL). Treatment time (§8.4); healing items you apply × `10000 + 30 × skill`. 25: reattachment and surgery (Canon §9.1). 50: splints and salves also heal 500 Bp of Max HP. 75: one treatment treats every untreated wound on the part. 100: install and remove augments without a station.
- **Survival** (SGT). Food satiation × `10000 + 20 × skill`; comfort band widened by `10 × floor_div(skill, 10)` DeciC each side. 25: butchering +1 unit. 50: forage and cook time × 5000. 75: immune to `st.diseased` from food. 100: `st.starving` damage × 5000.

**Social**

- **Speech** (CHM). Persuade and Deceive ratings; `RowPost(37, 10000 - 10 × skill)` and `RowPost(38, 10000 + 10 × skill)`. 25: shops show one extra hidden stock item. 50: bribe a sapient non-boss hostile for `mul_bp(floor_div(100 × rank_xp_bp, 10000), P(d_cur))` Lucre; it is pacified (§6.1). 75: reputation gains × 15000. 100: pacify Persuade checks use DC − 15.
- **Leadership** (MEN). Allied Errata and minions within 10 m get `DmgAdd +5 × skill` (max +500). 25: minion Max HP × 11500. 50: active Rally removes `st.feared` and `st.dread` from allies within 12 m; cooldown 1800. 75: aura radius 15 m. 100: allies in the aura +500 Bp crit chance (gear term).
- **Handling** (CHM). Minion cap (Canon §15.1); Tame rating. 25: mount summon 15 ticks. 50: tamed beasts Max HP × 12000. 75: elites can be tamed at DC + 30. 100: any tamed `body.quadruped` beast can be ridden as `arch.mount.runner`.

---
## 6. Character XP and levels

### 6.1 Sources

Canon §8 fixes kills, skill-ups, Folio exit and Unseen. Doc 04 adds the capped non-kill sources marked "04".

| Source | XP | Owner |
|---|---|---|
| Kill | `mul_bp(mul_bp(K(d), rank_xp_bp), 7000)`, `e = enemy_level(d) = d + floor_div(d, 5)` | Canon |
| Pacify: a hostile made non-hostile for the rest of the Folio by your Persuade, Intimidate, bribe (Speech 50), Tame, or Hacking-100 hack | exactly the kill XP of that entity; once per entity; the entity can never also grant kill XP | 04 |
| Skill-up | `2 × K(d_cur)`, at most 8 per Folio | Canon |
| Folio exit | `4 × K(d)` | Canon |
| Unseen | `20 × K(d)` | Canon |
| Discovery | 10 tenths per `secret.` found (max 2 per Folio); 5 tenths when the party has visited 50% of the Folio's rooms and 5 more at 100% | 04 |
| Identification | 2 tenths per instance you identify; 5 tenths per type you identify; cap 10 tenths per Folio | 04 |
| Crafting | `3 × recipe_tier` tenths per craft or brew you complete; cap 60 tenths per Folio | 04 |
| Objective | minor 20 tenths, major 50 tenths (doc 12 tiers) | 04 |

Kill XP by rank (`K(1) = 5`, `K(10) = 252`, `K(25) = 1386`):

| Rank | d = 1 | d = 10 | d = 25 |
|---|---|---|---|
| minion (3000) | 0 | 52 | 290 |
| standard (10000) | 3 | 176 | 970 |
| elite (30000) | 10 | 529 | 2910 |
| champion (60000) | 21 | 1058 | 5821 |
| boss (200000) | 70 | 3528 | 19404 |
| editor (400000) | 140 | 7056 | 38808 |
| hunter (50000) | 17 | 882 | 4851 |

### 6.2 Co-op

- Kill and pacify XP: each party member present in the Folio, downed members and ghosts included, gains `floor_div(xp × 10000, enemy_count_bp[n])` (Canon §8, §17.2). A minion's kill counts as its owner's (Canon §15.1).
- Exit, Unseen, Discovery and Objective XP: the full amount to each present member.
- Skill-up, Identification and Crafting XP: the actor only.

Example: an elite at d = 10 in a party of 4: `floor_div(529 × 10000, 17500) = 302` XP each.

### 6.3 Pacing simulation

Model: kill supply in standard-kill equivalents, fixed skill XP per Folio at MEM 40, counted skill-ups `min(8, ups)`, Exit every Folio, extras in tenths. Level is read after Folio 24 (arrival at Folio 25).

| Build | Kill-eq | Skill XP per Folio (start) | Extras per Folio | Unseen | Avg units | Level after F10 | Level on arrival at F25 | Major 50 / 75 |
|---|---|---|---|---|---|---|---|---|
| Combat (Vanguard) | 40 | hafted 60 (20), armor 30 (20), shield 25 (20), athletics 12 (10), origin 6 (10), acrobatics 6 (0), leadership 4 (10), smithing 3 (10) | 15 | none | 47.0 | 13 | 30 | F9 / F21 |
| Combat, doc-13 trim | 34 | same | 15 | none | 42.8 | 12 | 29 | F9 / F21 |
| Stealth (Shade) | 16 | sneak 45 (20), blade 30 (20), security 30 (20), pickpocket 20 (10), acrobatics 12 (10), illusion 8 (10), awareness 8 (0), origin 6 (10) | 25 | Folios with `d % 5 ∈ {1,2,3}` | 44.5 | 12 | 29 | sneak F12 / — |
| Crafting (Artificer) | 26 | tinkering 55 (20), smithing 40 (20), marksmanship 40 (20), appraisal 15 (10), delving 15 (0), inscription 12 (10), security 8 (10), origin 6 (10) | `15 + min(60, 20 × (1 + floor_div(d - 1, 5)))` | none | 44.9 | 12 | 29 | tinkering F10 / F23 |

Combat build, per Folio (XP):

| Folio | K | Kill | Skill-ups counted | Skill-up XP | Exit | Extras | Gain | Cumulative | Level |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 5 | 140 | 8 | 80 | 20 | 7 | 247 | 247 | 2 |
| 5 | 75 | 2100 | 8 | 1200 | 300 | 112 | 3712 | 7819 | 7 |
| 10 | 252 | 7056 | 7 | 3528 | 1008 | 378 | 11970 | 47211 | 13 |
| 15 | 529 | 14812 | 7 | 7406 | 2116 | 793 | 25127 | 142217 | 19 |
| 20 | 907 | 25396 | 5 | 9070 | 3628 | 1360 | 39454 | 311650 | 25 |
| 24 | 1215 | 34020 | 5 | 12150 | 4860 | 1822 | 52852 | 512919 | 30 |

Level 30 spans 493,000 to 542,499 XP, so 512,919 is level 30. Every build lands inside 28 ± 3 and meets Canon §7.2. Canon-fixed sources alone give the combat build 45.5 units per Folio; the 34-equivalent row is the doc-13 trim (Canon gaps).

### 6.4 Level-up flow

```rust
fn grant_char_xp(c: &mut Erratum, amount: i64, src: XpSource) {
    c.xp += amount;
    while c.xp >= cum_xp(c.level + 1) {            // cum_xp(L) = Σ_{l=1}^{L-1} 50 l (l + 3)
        c.level += 1;
        emit(ev.level_gained { entity: c.id, level: c.level });
        c.pending.push(Batch { level: c.level, left: 4, per_sub: [0; 24] });
        if c.level % 2 == 0 { c.talent_points += 1; if c.kin == kin.mutant { c.mutation_points += 1; } }
        if c.level % 10 == 0 { c.note_offer = Some(marginal_note_offer(c)); }
        add_composure(c, 20_000);
        recompute_derived(c);                       // Canon §12 max-change rule
    }
}
fn allocate(c: &mut Erratum, sub: Sub) -> Result<(), Reject> {
    if c.base[sub] >= 100 { return Err(Reject::Cap); }
    let b = c.pending.iter_mut().find(|b| b.left > 0 && b.per_sub[sub] < 2).ok_or(Reject::NoBatch)?;
    b.left -= 1; b.per_sub[sub] += 1; c.base[sub] += 1;
    c.alloc_history.push((b.level, sub));
    c.pending.retain(|b| b.left > 0);
    recompute_derived(c); Ok(())
}
```

- Points may be allocated at any tick; unspent points persist for the run. The oldest batch is used first.
- **Marginal Note** (every 10th level): the pool is every tier-1 and tier-2 talent of the other 12 Calling trees that the character lacks, sorted by ID. Draw `keyed_rng("rng.progress", folio, actor, level_up_tick, 0).shuffle(pool)` (`pool.len() - 1` `range` calls) and offer the first 3. The player takes one for free or declines; the offer is stored and never rerolled. A Note talent ignores tree-point gates and does not count toward that tree.

### 6.5 Respec

At a Scriptorium (doc 12):

- **Talents**: refunds every talent point spent (not Note talents); cost `mul_bp(200 × (n + 1), P(d_cur))` Lucre, `n` = prior talent respecs this run. At d = 10: 434, then 868.
- **Attributes**: reverses `alloc_history` and restores the batches; creation values and training are untouched; cost `mul_bp(400 × (n + 1), P(d_cur))`.
- Kin, Origin, Calling and Echo choices never change.

---

## 7. Talents

### 7.1 Rules

- Each Calling tree has 12 talents: tier 1 ×4, tier 2 ×4, tier 3 ×3, tier 4 ×1. One talent point buys one talent, once.
- Gates: tier 2 needs 2 points spent in that tree; tier 3 needs 5 points, level ≥ 12 and the key primary ≥ 45; tier 4 needs 8 points, level ≥ 20, key primary ≥ 50 and its named prerequisite.
- A character buys from its own Calling tree and the general tree only.
- IDs: `talent.<calling_suffix>.<name>`, `talent.general.<name>`. "(active)" talents take an ability slot.
- Key primaries: vanguard MGT, duelist FIN, gunslinger FIN, grenadier MGT, spellwright WIL, alchemist INT, shade FIN, artificer INT, warden VIG, oracle WIL, netrunner INT, beastbinder PRE, monk FIN.

```json
{ "id": "talent.vanguard.last_bastion", "tier": 4, "tree": "calling.vanguard", "requires": ["talent.vanguard.unyielding"],
  "active": 0, "trigger": { "hp_below_bp": 2500 }, "cooldown_ticks": 5400,
  "effects": [ { "apply_status": "st.fortified", "ticks": 600 }, { "apply_status": "st.empowered", "ticks": 600 } ] }
```

### 7.2 Calling trees

**Vanguard.** T1: `heavy_hand` hafted hits `DmgAdd +1000`; `bulwark` `RowFlat(18, 10)`; `shield_wall` `RowFlat(19, 1000)` while a shield is equipped; `second_wind` when stamina reaches 0, restore 50% of max stamina, cooldown 3600. T2: `crushing_blow` heavy hafted poise damage × 13000; `taunting_roar` (active) `st.taunted` directly on hostiles within 8 m, 20 stamina, cooldown 1200; `plated` natural armor `mul_bp(4, T(d_cur))` on every part; `unyielding` immune to `st.knocked_down` while blocking. T3: `juggernaut` sprinting into a non-boss hostile applies `st.staggered`, once per target per 300 ticks; `retaliation` perfect parries reflect `mul_bp(blocked, 3000)` as thorns; `rallying_presence` allies within 10 m get `RowFlat(18, 10)` and +1 composure per s. T4: `last_bastion` (needs `unyielding`) below 2500 Bp HP gain `st.fortified` and `st.empowered` for 600 ticks, cooldown 5400.

**Duelist.** T1: `riposte` the next blade hit within 60 ticks of a parry `DmgAdd +5000`; `fleet_foot` dodge cost −3 stamina; `en_garde` +500 Bp crit chance with one-handed blades (gear term); `flourish` blade killing blows restore 10 stamina. T2: `lunge` (active) 4 m dash into a blade attack, 25 stamina, cooldown 360; `feint` a dodge that negates a hit makes your next hit within 60 ticks apply 1 `st.exposed` stack; `challenge` (active) one hostile within 20 m takes `DmgAdd +1500` from you for 600 ticks, cooldown 1800; `silver_tongue` Speech checks +10, `RowPost(37, 9500)`. T3: `blade_dance` each blade hit within 90 ticks of the previous adds `DmgAdd +300`, max 5 stacks; `uncanny_dodge` dodge invulnerability +2 ticks (cap 12); `coup_de_grace` blade hits on targets below 2000 Bp HP use the precision multiplier on any part. T4: `thousand_cuts` (needs `blade_dance`) every 5th consecutive blade hit on one target within 120 ticks applies 3 `st.bleeding` stacks directly and crits (roll drawn, result replaced).

**Gunslinger.** T1: `steady_aim` `RowPost(15, 9000)` while crouched or still; `fast_hands` `RowPost(17, 11000)`; `long_shot` `DmgAdd +1000` beyond 20 m; `trick_toss` thrown hits `DmgAdd +1500`. T2: `fan_the_hammer` the first 3 pistol shots after a reload fire at interval × 8000; `deadeye` ranged precision multiplier +2500 Bp; `dive_shot` shots during a dodge ignore movement spread; `quick_draw` swap ticks × 7000. T3: `mark_for_death` (active) `st.marked` on a target within 40 m and your hits on it `DmgAdd +1500` for 480 ticks, cooldown 1200; `ricochet` a bullet hitting a block bounces once to the nearest hostile within 6 m at × 5000; `high_noon` crits with a magazine weapon refund 1 round. T4: `dead_mans_hand` (needs `deadeye`) each kill within 300 ticks of your previous kill adds +500 Bp crit chance, max +2500, cleared 300 ticks after the last kill.

**Grenadier.** T1: `blast_padding` your own area hits on you × 5000; `bandolier` throwable stack size +2; `big_iron` `RowPost(16, 8500)` for heavy weapons; `sapper` your explosions' block damage × 15000. T2: `cook_off` grenades held ≥ 60 ticks `DmgAdd +1500`; `shrapnel` your explosions +40 `st.bleeding` power; `bracing` `RowPost(15, 7000)` for heavy weapons while crouched; `demolition` your explosions break blocks 1000 mm farther. T3: `chain_reaction` a hostile killed by your area hit explodes, 2 m, `B = mul_bp(its max HP, 1500)`, `dmg.fire`, never chains; `suppressing_fire` 4 heavy-weapon hits on one target within 60 ticks apply `st.slowed` directly; `walking_arsenal` with a heavy weapon active `RowFlat(18, 20)` and no knockback. T4: `carpet_bomb` (needs `cook_off`, active) throw 5 copies of the selected throwable in a 4096 Bam fan for the cost of 1, cooldown 3600.

**Spellwright.** T1: `efficient_casting` aether × 9000; `glyph_scholar` `RowFlat(27, 1)`; `overchannel` toggle: aether × 15000 and `DmgAdd +2000` on spells; `arcane_focus` `RowFlat(30, 1500)`. T2: `extend` spell durations × 12500; `widen` spell area radius +1000 mm; `mana_shield` while aether ≥ 25% of max, 2000 Bp of each hit's HP loss is paid 1:1 from aether instead; `quicken` cast ticks × 8500. T3: `spell_echo` every 5th effective cast costs 0 aether; `elemental_attunement` choose a damage type when learned, spell hits of it `DmgAdd +2000`; `complex_lattice` `RowFlat(26, 3)`. T4: `grand_design` (needs `complex_lattice`) `RowFlat(26, 4)`, `RowFlat(27, 1)`, and one Signature spell at aether × 7000 with `DmgAdd +1500`.

**Alchemist.** T1: `potent_brews` brewed potency +1000 Bp; `iron_stomach` toxicity gains × 7500; `quick_quaff` drink ticks × 6000; `splash_master` thrown potion radius +1000 mm. T2: `double_batch` brews yield +1; `volatile_mix` your bombs `DmgAdd +1500`; `field_medic` healing potions you drink or throw × 12000; `transmuter` 3 identical potions become 1 potion of a random type of the same rarity at an alembic (keyed `rng.alchemy`, 1 `weighted` draw). T3: `catalyst` bomb status power × 13000; `philosophers_vial` once per Folio a drunk potion is not consumed; `antivenom` immune to `st.poisoned` from your own items, resist `dmg.toxin` +2500. T4: `magnum_opus` (needs `potent_brews`, Alchemy ≥ 75) potions you brew are `buc.blessed`.

**Shade.** T1: `soft_step` footstep noise −6 dB; `backstab` sneak-attack multiplier +5000 Bp; `light_fingers` pickpocket and lockpick ticks × 8000; `nightborn` `RowFlat(31, 3)`. T2: `vanish` (active) `st.invisible` 180 ticks, cooldown 2400; `quiet_kill` sneak-attack kills emit no `ev.noise` beyond 2 m; `envenom` coat a blade with a potion so the next 5 hits apply it at potency × 5000; `escape_artist` `st.grappled` and `st.rooted` durations × 5000. T3: `shadow_mark` sneak attacks apply `st.marked` directly; `death_from_above` sneak attacks with `dy ≥ 2500` mm `DmgAdd +5000`; `ghost_walk` sight detection of you while crouch-walking × 8000 (doc 09). T4: `unseen_blade` (needs `backstab`, Sneak ≥ 50) sneak attacks carry `ignore_armor`.

**Artificer.** T1: `overclock` gadget and energy-weapon recharge × 12000; `field_repair` repair × 15000; `scrap_savant` salvage +1 unit per 3; `turret_tech` turret limit +1. T2: `modular` weapons you craft get +1 option per perk column (max 3); `drone_companion` each Folio starts with a scout drone gadget deployed (doc 07); `reinforced` armor you craft `mat_ar_bp` +500 (cap 14000); `hardlight` your gadgets' Max HP × 12500. T3: `masterwork` your crafted quality shifts one step up, max `q.peerless`; `self_destruct` your gadgets explode on destruction, 2 m, `B = mul_bp(20000, P(src_lvl))`, `dmg.fire`; `smart_rounds` ammo you craft `DmgAdd +1000`. T4: `legion` (needs `turret_tech`) turret and drone limit +2; each of your deployed gadgets gives you `DmgAdd +300`, max +1500.

**Warden.** T1: `guardian` hits on allies within 2 m that cross your front block arc strike your block instead; `mender` healing you cause × 11000; `warding` wards you create × 12000; `steadfast` `RowFlat(24, 20)`. T2: `intercept` (active) dash up to 8 m to an ally and give it `st.shielded` of 1500 Bp of its Max HP, cooldown 1200; `purge` your restoration casts remove one status of category debuff or DoT; `aegis_bash` shield bash staggers elites too; `sanctuary` while you block, allies within 4 m regain 2 composure per s. T3: `martyr` 3000 Bp of HP loss of allies within 10 m (after their step 7) is moved to you as `dmg.true` with no further mitigation; `resurgence` revive time × 5000 (doc 16 hook); `bastion` `st.shielded` you apply lasts × 15000. T4: `last_light` (needs `martyr`) once per Folio, when an ally within 15 m would be downed it instead heals 3000 Bp of its Max HP and you lose that much HP, never below 1,000 mHP.

**Oracle.** T1: `foresight` `RowFlat(35, 1000)`; `devout` favor gains × 12500; `spirit_guide` summon Max HP × 12000; `omen_reader` counts as a Lattice Lens (Canon §17.1). T2: `prophecy` once per Folio identify the type of one unknown carried item; `divine_favor` prayer timer increments × 7500; `spectral_host` Conjuration counts +25 in the minion cap formula; `glimpse` (active) hostiles within 25 m gain `st.marked` for 300 ticks, cooldown 1800. T3: `fated` `SubFlat(PRV, 10)`; `omen_ward` one hit of at least 3000 Bp of your Max HP is × 5000, once per 1800 ticks; `consecrated` your summons convert 3000 Bp of each hit's share to `dmg.radiant`. T4: `avatar_of_fate` (needs `fated`, active) for 600 ticks each of your keyed rolls draws twice and keeps the better (exactly 2 draws per roll), once per Folio.

**Netrunner.** T1: `backdoor` hack time × 7500; `ghost_signal` camera and turret detection of you × 7000; `overload` when your `st.hacked` ends on a unit it bursts, 2 m, `B = mul_bp(10000, P(d_cur))`, `dmg.shock`; `firewall` immune to `st.hacked`, resist `dmg.psychic` +1500. T2: `puppet_master` your `st.hacked` lasts × 15000; `daemon` (active) a construct or device within 15 m takes 60 `st.shocked` buildup, cooldown 600; `jammer` electronic hostiles within 6 m take 10 `st.blinded` buildup per status pulse; `remote_access` hack range 15 m. T3: `botnet` +1 simultaneous hacked unit; `feedback` a unit you hacked dying restores 10 aether; `data_thief` hacking a terminal grants `mul_bp(50, P(d_cur))` Lucre once per terminal. T4: `system_crash` (needs `daemon`, active) every construct or electronic hostile within 12 m gains `st.stunned` directly, cooldown 3600.

**Beastbinder.** T1: `pack_leader` your minions `DmgAdd +1000`; `feral_bond` tamed beasts within 8 m heal 100 Bp of Max HP per s; `tracker` hostile footprints within 30 m show for 300 ticks; `beast_tongue` Tame +15. T2: `sic_em` (active) your minions attack the target with `DmgAdd +2500` for 300 ticks, cooldown 900; `hardy_beasts` minion Max HP × 13000; `mount_bond` mount stamina × 15000 and Max HP × 12000; `shared_senses` hostiles your minions see are `st.marked` for you. T3: `alpha` Handling counts +25 in the minion cap formula; `primal_fury` (active) minions gain `st.empowered` 300 ticks, cooldown 2400; `herd_mend` healing on your minions × 15000. T4: `chimera_call` (needs `alpha`, active) fuse two minions within 3 m for 1800 ticks: summed current and max HP, damage × 13000, cooldown 5400.

**Monk.** T1: `iron_fist` unarmed `DmgAdd +1500`; `wind_step` wall-run up to 2000 mm per airborne period (dependency verve, Req from the Origin genre); `centered` composure losses × 8500; `deflect` empty-handed blocks stop projectiles in the front arc. T2: `flurry` every 3rd consecutive unarmed hit within 60 ticks strikes twice, the second × 5000; `pressure_points` unarmed hits +25 `st.stunned` power; `still_breath` `res.breath` max +10 s and `RowPost(5, 12000)` while still; `roll_with_it` fall damage × 5000. T3: `stunning_palm` (active) an unarmed strike applying `st.stunned` directly, cooldown 900; `diamond_body` immune to `st.poisoned` and `st.diseased`, `RowFlat(22, 30)`; `drunken_master` while `st.confused`, `DmgAdd +2000` and dodge invulnerability +2 ticks (cap 12). T4: `quivering_palm` (needs `stunning_palm`, active) 180 ticks after marking, the target takes `B = mul_bp(60000, P(d_cur))` `dmg.true` entering at step 3, × 5000 against ranks boss, editor and hunter, cooldown 5400.

### 7.3 General talents

G1 (no gate): `toughened` `SubFlat(TGH, 4)`; `quick_learner` `RowPost(28, 11000)`; `pack_mule` `RowFlat(10, 10)`; `lucky` `SubFlat(FAT, 5)`; `polyglot` `TolAdd(chosen axis, 1)`; `iron_will` `SubFlat(RSV, 5)`; `forager` food satiation × 12500; `hardy` comfort band +100 DeciC each side; `marathon` `RowPost(6, 8500)`. G2 (level ≥ 10): `polyglot_ii` (needs `polyglot`) `TolAdd(another axis, 1)`; `keen_senses` `SubFlat(SGT, 3)` and `SubFlat(HRG, 3)`; `field_dressing` treatment time × 5000; `scavenger` Lucre pickups × 11000; `light_sleeper` `st.sleeping` on you × 5000 after Canon §11.8.

---
## 8. Body

### 8.1 Part state

```rust
struct PartState {
    part: BpId,            // Canon §9 part ID
    max: i64,              // mul_bp(max_hp_mhp, share) (Canon §9)
    pool: i64,             // 0..=max, Milli
    tier: u8,              // 0..=4 (§8.2)
    kind: WoundKind,       // laceration | fracture | burn | trauma
    treated: bool,
    severed: bool,
    nat_ar: i64,           // natural armor from traits, perks, augments, recomputed on change
}
```

Canon §10.5 step 8 subtracts the hit's HP damage from the hit part's pool, floored at 0.

### 8.2 Wound tiers

`frac = floor_div(pool × 10000, max)`. `tier_for(frac)`: `frac > 7500` → 0; `> 5000` → 1 (`wound.bruised`); `> 2500` → 2 (`wound.wounded`); `> 0` → 3 (`wound.crippled`); `pool = 0` → 4 (`wound.disabled`). Severing is `wound.severed`.

The wound **kind** is set when the tier rises, from the category of the hit's largest type share (ties to the earlier Canon §10.1 row): slash or pierce → `wound.laceration`; blunt and fall damage → `wound.fracture`; fire, frost, acid, shock, radiation, sonic → `wound.burn`; toxin, radiant, umbral, psychic, arcane, true → `wound.trauma`.

```rust
fn on_part_damage(e: &mut Entity, p: BpId, hit: &Hit, dmg: i64) {
    let s = e.part_mut(p);
    let before = s.pool;
    s.pool = (s.pool - dmg).max(0);
    if s.pool == 0 && before > 0 && severable(e, p)
        && (largest_share(hit) == dmg.slash || hit.flags.sever)
        && head_rule(e, p, hit) { sever(e, p, hit); return; }          // Canon §9.1
    let t = tier_for(frac(s));
    if t > s.tier { s.tier = t; s.kind = kind_of(hit); s.treated = false; emit(ev.wound_changed); }
}
fn on_heal(e: &mut Entity, healed: i64) {                                 // every HP heal, after Canon §10.9
    for s in e.parts_ascending_mut().filter(|s| !s.severed) {
        let gain = floor_div(healed * s.max, e.max_hp_mhp);
        let cap = if !s.treated && s.tier >= 2 { [0, 0, 5000, 2500, 0][s.tier as usize] } else { 10000 };
        let new_pool = (s.pool + gain).min(mul_bp(s.max, cap)).max(s.pool);
        s.pool = new_pool;
        let t = tier_for(frac(s));
        if t < s.tier { s.tier = t; if t <= 1 { s.treated = false; } emit(ev.wound_changed); }
    }
}
```

An untreated wound of tier 2 or higher caps its pool at its tier until a treatment (§8.4) sets `treated`; HP still heals. Natural regeneration feeds `on_heal`.

### 8.3 Wound effects (humanoid)

Effects are modifiers with source IDs `wound.<tier>`; they stack across parts.

| Part | Tier 1 | Tier 2 | Tier 3 | Tier 4 |
|---|---|---|---|---|
| `bp.head` | aim sway × 11000 | aim sway × 13000 | aim sway × 16000; composure losses × 12500 | `RowPost(15, 15000)`, `RowFlat(31, -2)`, `RowFlat(33, -10)`; NPCs gain `st.stunned` directly once |
| `bp.torso` | `RowPost(5, 9500)` | `RowPost(5, 9000)` | `RowPost(5, 7500)`, `RowPost(6, 12500)` | `RowPost(5, 5000)`; no sprint |
| `bp.arm_l`, `bp.arm_r` | `RowPost(17, 9500)` | `RowPost(17, 9000)` | `RowPost(17, 8000)`, `RowPost(16, 12500)`, that hand's melee `DmgAdd -1500` | arm unusable as if severed, but no part item drops |
| `bp.leg_l`, `bp.leg_r` | `BodyMove 9750` | `BodyMove 9500` | `BodyMove 8000`; no sprint | `BodyMove 5000`; no sprint, jump or dodge |
| `bp.tail`, wings, extra arms | none | none | none | the part's granted functions are off |

Row 17 takes the lower of the two arm factors. Armor 50 removes all tier-1 effects. NPC templates (doc 13) map their attacks to parts: a tier-4 arm disables that arm's attacks; quadruped legs use `BodyMove` 9000 at tier 3 and 7500 at tier 4 or severed.

### 8.4 Healing, Medicine and treatment

Treatments are `cat.tool` items (doc 11 IDs) with these doc-04 profiles. Each treats the most severe untreated wound of a matching kind on one chosen part of the user or an Erratum within 2 m.

| Treatment | Kinds | Tiers | Base ticks | Medicine | Extra |
|---|---|---|---|---|---|
| bandage | laceration, trauma | 2–3 | 60 | 0 | ends `st.bleeding` (Canon §11.1) |
| splint | fracture | 2–3 | 120 | 0 | — |
| salve | burn | 2–3 | 90 | 0 | ends `st.burning` |
| surgical kit | any | 4; reattach | 600 | 25 | tier 4 → pool `mul_bp(max, 2500)`, tier 3, treated |

```
ticks  = max(15, ceil_div(base × 10000, 10000 + 100 × medicine + 50 × d(ANL)))      // treater's values
effect: treated = true; pool += mul_bp(max, 1500 + 30 × medicine), capped at max; tier recomputed as in on_heal
```

Non-DoT damage to the treater or 500 mm of movement breaks the channel and refunds the item. Example: a bandage at Medicine 0, ANL 40 takes 60 ticks and restores `mul_bp(10625, 1500) = 1593` mHP to an 85-HP arm; at Medicine 50, ANL 50, `ceil_div(600000, 15500) = 39` ticks and 3,187 mHP. Treating an arm on yourself requires the other arm at tier ≤ 3.

Other healing: Canon §10.9 applies to HP; `on_heal` then feeds pools. `st.diseased` (owner 04) builds from body-group powers: stale food 30, rotten food 60, raw meat 40, disease-tagged enemy attacks (doc 13). It lasts 300 s and ends early with a cure potion (doc 08).

### 8.5 Body move factor

`BodyMove` is the product, one `mul_bp` each in ascending part-ID order, of every leg factor (§8.3; severed leg 5000 per Canon §9.1; a crutch in `slot.offhand` turns one severed or tier-4 leg into 7000), clamped to `2500..=10000`. With both legs severed it is 2500 (crawl). It applies after row 11 and before status multipliers.

### 8.6 Severing

1. Emit `ev.part_severed`. Remove the part's hitbox. Spawn a `cat.body_part` item at the part's capsule midpoint.
2. NPC: an arm drops its held item and loses its attacks; a leg applies the leg rule; a wing removes flight; a tail removes tail attacks; a head kills single-headed creatures (Canon).
3. Erratum: 3 `st.bleeding` stacks applied directly; composure −10 (§11.3); held item and two-handed use per Canon; augments in that limb travel inside the part item and return on reattachment. With both arms gone: no weapons or hand items, interact actions take 3× ticks, and kicks are unarmed strikes at `B × 5000`.
4. Reattachment: surgical kit and Medicine ≥ 25 (600-tick channel), a Scriptorium service for `mul_bp(300, P(d_cur))` Lucre, or a prayer result that restores the body (doc 06). The part item must be under 7200 ticks old and its `owner` must match. Result: pool `mul_bp(max, 2500)`, tier 3, treated. Emit `ev.part_reattached`.
5. Resurrection from ghost (Canon §17.6) restores every severed part and clears all wounds.

Body-part item (Canon §13.1 `cat.body_part`, rots after 7200 ticks):

```json
{
  "uid": "0x0000070000000041", "template": "bp.arm_r", "source_template": "enemy.mythic_fantasy.troll",
  "owner": "0x0000070000000013", "created_tick": 151200, "mass_g": 9100, "ilvl": 7,
  "materials": [ "mat.flesh", "mat.bone" ], "tags": [ "regenerating", "beast_sapient" ], "rotten": 0
}
```

Mass share of body mass (Bp): humanoid head 700, torso 4700, each arm 650, each leg 1650; quadruped head 900, torso 5500, each leg 900; avian head 800, torso 5000, each wing 1100, each leg 1000; serpentine head 1200, body_front 4000, body_rear 3500, tail 1300; insectoid head 1000, thorax 3500, abdomen 3502, each leg 333; construct core 1500, chassis 5500, each limb 750. The troll arm above is `mul_bp(140000, 650) = 9100` g.

Uses of a body part:

- **Butcher** (Survival, 120 ticks): `floor_div(mass_g, 2000)` raw meat units (food, §11.2) and `floor_div(mass_g, 5000)` units of the part's non-flesh material (doc 07), each +1 with Survival 25. Rotten parts yield the non-flesh material only.
- **Ingredient** (doc 08): the creature's tags give the part's alchemical effect (`regenerating` → healing over time, `undead` → umbral, `fiend` → fire, `eldritch` → psychic). Rotten parts have potency × 5000.
- **Food**: raw-meat rules with satiation `min(900, floor_div(mass_g, 10))`; eating a sapient creature's part costs composure −15 unless the eater has `afflict.hunger`.
- **Graft**: a fresh arm or leg of a creature tagged `graftable` installs as the grafted skin of `aug.prosthetic_arm` or `aug.prosthetic_leg` (§9.5).
- **Value**: `mul_bp(5 × floor_div(mass_g, 1000), P(ilvl))` Lucre for selling and sacrifice.

---

## 9. Augments

### 9.1 Forms

Mutations, implants, grafts and prosthetics are one system: an **augment** archetype (`aug.*`) carries the mechanics; per-form skins set dependency (Canon §5.3) and look.

| Form | Dependency | Source |
|---|---|---|
| implant | techne | `cat.implant` items (type-hidden, `idc.implant`); crafted at Tinkering 75 |
| graft | arcana | graft items (doc 11), grafting a fresh `graftable` body part |
| mutation (natural) | none | `kin.mutant`, mutagen potions (doc 08), radiation |
| mutation (supernatural) | mythos | mythos-genre mutagens and curses (doc 08, doc 10) |
| prosthetic (mundane) | none | hook and peg items |

An installed augment is one component. `Req` uses the origin genre of its source item (Canon §5.3); a mutation granted by `kin.mutant` or radiation uses the character's Origin genre. Its Contradiction potency scales its numbers (§1.2). An augment's tolerance bonus counts as `gear_tol`.

### 9.2 Capacity

- Four regions, one per Canon slot: `slot.implant_head`, `slot.implant_torso`, `slot.implant_arms`, `slot.implant_legs`. Each region holds augments totalling at most 4 points.
- Total capacity `aug_cap = 6 + floor_div(RES, 20)` points (8 at RES 40), +2 from `trait.origin.wired`, +2 for mutations only from `trait.kin.mutant.mutable_flesh`.
- Installation is refused when it would exceed the region cap or `aug_cap`. If `aug_cap` later falls below the installed total, every augment counts as Strained C = 2 (potency 8000) until the total fits again.
- Prosthetics cost 0 points and do not count.

### 9.3 Acquisition, install and removal

- **Install** at a doc-07 augment station, or anywhere with Medicine 100: 300 ticks. Unknown implants can be installed blind; their curse state applies (doc 10).
- **Remove**: implants return as items; grafts are destroyed; mutations are removed only by the Scriptorium service "Emend the Flesh" for `mul_bp(400, P(d_cur))` Lucre.
- **Mutation points (MP)**: `kin.mutant` gains 1 at each even level; any character gains 1 when `st.irradiated` reaches 10 stacks (once per Folio). Spending 2 MP opens an offer of 3 mutations: `keyed_rng("rng.progress", folio, actor, tick, 1).shuffle(pool)` over all mutation-form augments that fit region and total capacity, sorted by ID; the first 3 are offered and the offer is stored until chosen. Spending 1 MP raises an owned mutation's rank (max 3); rank `r` multiplies its numeric magnitudes by `10000 + 5000 × (r - 1)`.
- Emit `ev.augment_changed` on install, removal, rank-up and Contradiction-driven on/off.

### 9.4 Augment list

| ID | Region, points | Effect (at rank 1, potency 10000) | Forms |
|---|---|---|---|
| `aug.eye_lowlight` | head 1 | `RowFlat(31, 3)` | implant, graft, natural mutation |
| `aug.eye_targeting` | head 2 | `RowPost(15, 9000)` | implant, graft |
| `aug.cortex_buffer` | head 2 | composure losses × 8000; `RowFlat(24, 20)` | implant, graft, natural mutation |
| `aug.audio_dampers` | head 1 | immune to `st.deafened`; resist `dmg.sonic` +2500 | implant, natural mutation |
| `aug.neural_jack` | head 2 | hack time × 8000; hack range +5 m | implant |
| `aug.third_eye` | head 2 | `RowFlat(35, 1500)`; see `st.invisible` within 6 m | graft, supernatural mutation |
| `aug.memory_lattice` | head 2 | `RowFlat(27, 1)` | implant, graft |
| `aug.gills` | head 1 | no breath drain in `liq.water`; the implant skin (rebreather) also makes gases breathable | natural mutation, graft, implant |
| `aug.dermal_plating` | torso 2 | natural armor `mul_bp(6, T(d_cur))` on every part | implant, graft, natural mutation |
| `aug.second_heart` | torso 2 | `RowPost(2, 15000)` | graft, natural mutation, implant |
| `aug.toxin_filter` | torso 1 | toxicity decay +500 per s; `RowFlat(22, 30)` | implant, natural mutation |
| `aug.adrenal_pump` | torso 2 | HP falling below 3000 Bp of max applies `st.hasted` 300 ticks; cooldown 3600 | implant, natural mutation |
| `aug.subdermal_ward` | torso 2 | on Folio entry and after 600 ticks without damage, `st.shielded` pool set to `max(current, mul_bp(max_hp, 1000))` | implant, graft, supernatural mutation |
| `aug.thermal_regulator` | torso 1 | comfort band +150 DeciC each side | implant, natural mutation |
| `aug.chitin_carapace` | torso 3 | natural armor `mul_bp(12, T(d_cur))` on torso and arms; resist slash +1000; `RowPost(40, 12000)` | natural mutation, graft |
| `aug.photosynthetic_skin` | torso 1 | +1 satiation per 2 s while `light_at ≥ 12` and not `in_combat` | natural mutation, graft |
| `aug.venom_sacs` | torso 1 | unarmed hits +40 `st.poisoned` power | natural mutation, graft |
| `aug.extra_arm` | torso 3 | adds `bp.arm_extra_1` (share 1000 from torso, hit_mult 8500) granting `slot.hand_extra_1` | natural mutation, implant, graft |
| `aug.tail` | torso 1 | adds `bp.tail` (share 500 from torso); `RowFlat(18, 5)` | natural mutation, graft |
| `aug.wings` | torso 3 | adds `bp.wing_l`, `bp.wing_r` (750 each from torso); holding jump while falling glides at −2000 mm/s vertical and up to 9000 mm/s horizontal with no fall damage on landing | natural or supernatural mutation, graft, implant (jet glider) |
| `aug.reactive_skin` | torso 2 | the first control status from buildup on you each Folio, per status, lasts × 5000 | implant, natural mutation |
| `aug.blood_battery` | torso 2 | a cast short of aether pays the rest from HP, 1 mHP per 1 m-aether, never below 1,000 mHP | graft, supernatural mutation |
| `aug.climbing_claws` | arms 1 | `RowPost(21, 15000)`; Climb checks +15 | natural mutation, implant, graft |
| `aug.myomer_bundles` | arms 2 | `RowFlat(10, 10)`; `RowPost(20, 12500)` | implant, graft, natural mutation |
| `aug.blade_arm` | arms 2 | bare-hand strikes become `B × 15000` with a 10000 Bp `dmg.slash` share | implant, natural mutation, graft |
| `aug.smartlink` | arms 1 | ranged weapons `RowPost(15, 8500)` | implant |
| `aug.static_palms` | arms 1 | melee hits +30 `st.shocked` power | implant, graft, natural mutation |
| `aug.spring_tendons` | legs 2 | one extra air jump per airborne period at `mul_bp(JUMP_H, 6000)`; fall damage × 7500 | implant, natural mutation, graft |
| `aug.sprint_servos` | legs 1 | `RowPost(6, 7000)` | implant, natural mutation |
| `aug.silent_pads` | legs 1 | footstep noise −6 dB | natural mutation, graft, implant |
| `aug.anchor_feet` | legs 2 | `RowFlat(18, 15)`; immune to knockback | implant, natural mutation |
| `aug.prosthetic_arm` | arms 0 | §9.5 | prosthetic, implant, graft, verve skin |
| `aug.prosthetic_leg` | legs 0 | §9.5 | prosthetic, implant, graft, verve skin |

### 9.5 Prosthetics

A prosthetic installs only on a severed limb and has two components:

- **Structure** (dependency none, always on): the limb exists again with its Canon share and pool. Arm: holds one-handed items and off-hand items; `RowPost(17, 8500)` for that hand; no two-handed weapons. Leg: `BodyMove 9000` for that leg; sprint, jump and dodge allowed.
- **Augment** (dependency from the skin): full function, removing the structure penalties, plus natural armor `mul_bp(10, T(d_cur))` on that limb and immunity to tiers 1 and 2 on it.

Skins: hook and peg (structure only, none), clockwork arm and leg (techne, Steampunk), cyberarm and cyberleg (techne, Cyberpunk), silver hand and leg (arcana, Mythic Fantasy), jade arm and leg (verve, Wuxia), grafted arm and leg (arcana, from a fresh `graftable` part; the graft also carries the creature's first tag effect at × 5000 potency).

When the augment component is Disconnected, the limb keeps its structure function. Example: a Cyberpunk-origin cyberarm (`Req[techne] = 7`) in Fairy Tale (TEC 1) on a Mythic Fantasy-origin character with ATN 40: `raw_gap = 6`, `Tol = 0`, `C = 6`, Disconnected, so the arm works as a hook. The same arm on a Cyberpunk-origin character has `Tol = 2`, `C = 4`, potency 6000: full function and natural armor `mul_bp(mul_bp(10, T(d)), 6000)`.

```json
{
  "id": "aug.dermal_plating", "region": "slot.implant_torso", "points": 2, "max_rank": 3,
  "effects": [ { "natural_armor_base": 6, "parts": "all" } ],
  "skins": [ { "form": "implant", "dep": ["techne"] }, { "form": "graft", "dep": ["arcana"] },
             { "form": "mutation_natural", "dep": [] } ],
  "req": { "attr.vigor": 40 }
}
```

---

## 10. Afflictions and Marginalia powers

### 10.1 Contraction

Each affliction has a buildup meter in the body threshold group (row 22). Sources carry `afflict_power` (doc 13 attacks, doc 08 items); contact with `liq.red_ink` adds 5 `afflict.red_ink` power per status pulse; `fac.editors` attacks carry 20 by default. Power follows the Canon §11.2 chain without the resistance step. Meters decay as Canon §11.2.

```
Clean --meter full--> Incubating (st.incubating, 7200 ticks; ev.affliction_changed)
Incubating --expiry--> Afflicted (ev.affliction_changed)
Incubating | Afflicted --cure--> Clean (ev.affliction_changed)
Afflicted(red_ink, doses < 4) --meter full or ink vial drunk--> doses += 1 at once
```

A full hunger or beast meter while Incubating or Afflicted only resets. Drinking a red-ink vial fills the `afflict.red_ink` meter. Cure: a Scriptorium service for `mul_bp(500, P(d))` Lucre or a blessed cure potion (Canon §16.4); 1085 Lucre at d = 10.

Example: a vampire bite with power 25 at d = 5 adds `mul_bp(25, S(5) = 11200) = 28`; at RES 40 (threshold 100) the fourth bite fills the meter (112).

### 10.2 Effects

- **`afflict.hunger`**: Canon effects (dark-vision +2 m, melee heals 1000 Bp of damage dealt, no natural regeneration while `light_at ≥ 12`, resist radiant −5000). Doc 04: food other than meat and blood gives × 5000 satiation; Feed is a 120-tick action on a corpse dead under 600 ticks, +600 satiation, once per corpse.
- **`afflict.beast`**: once per Folio by its ability-slot action, and involuntarily whenever composure is below 25% of max (then not again until 1800 ticks after the form ends), the bearer gains `st.polymorphed` for 30 s into the beast template: `body.quadruped` movement (§13), PWR 70, BRN 60, AGI 60 or its own if higher, bite `B = mul_bp(10000, P(d_cur))` (slash 7000, pierce 3000, 30-tick swing), claw `B = mul_bp(7000, P(d_cur))` slash (20 ticks). Melee × 15000 sits at Canon §10.5 step 2.12 ordered as `st.polymorphed`. No items, no casting.
- **`afflict.red_ink`**: each dose lowers the hunter threshold by 1 (Canon §11.6), applies `RowFlat(9, -5)`, and unlocks one Marginalia power of the bearer's choice among those not owned.

### 10.3 Marginalia powers

`trait.marginalia.*`, dependency none (ink sits beneath genre law), `src_lvl = d_cur`:

1. `strikethrough` (active): a hostile within 20 m takes `B = mul_bp(30000, P(d_cur))` `dmg.arcane` and `st.unraveled` directly; cooldown 1800.
2. `redaction` (active): `st.invisible` 300 ticks; cooldown 3600.
3. `rewrite_wound` (active): heal 2500 Bp of Max HP and set every wound of tier ≤ 3 to treated; gain 1 `st.erasure` stack; cooldown 3600.
4. `ink_sight`: `fac.editors` members and `st.invisible` entities within 30 m are visible to you.
5. `footnote`: `DmgAdd +2000` against `fac.editors` members.
6. `erase` (active): remove a 3 × 3 × 3 block volume centred on the aimed block within 6 m (never `block.void_bedrock`); cooldown 600.
7. `margin_step` (active): teleport up to 12 m along aim to the nearest valid standing position; cooldown 900.
8. `dictation` (active): `st.charmed` directly on a non-boss hostile; gain 1 `st.erasure` stack; cooldown 3600.

---
## 11. Needs

All needs run on the Needs subsystem (period 60, phase 7, `k = 1`) unless stated.

### 11.1 Satiation

- Range `0..=2400`, start 1800, `st.hungry` at ≤ 600, `st.starving` at 0 (Canon §12, §11.1). Decay pauses in boss Folios while the boss lives (Canon §11.6).
- Decay uses a thousandths accumulator: `R = 1000` multiplied by each personal factor (`scavenger` 7500, `grave_appetite` 5000, `fast_metabolism` 12500) in ascending source-ID order; each Needs tick `q = floor_div(R + acc, 1000)`, `acc = R + acc - 1000 q`, satiation −= q.
- While `st.starving`, every 300th tick of continuous starvation deals `mul_bp(max_hp_mhp, 100)` as `dmg.true` entering Canon §10.5 step 4 (850 mHP at 85 HP); natural regeneration is 0.
- Eating is allowed below 2400; the result is `min(2400, current + value)`.

### 11.2 Food values

`value = base × freshness`, then × `10000 + 20 × skill.survival`, then talent and trait factors, one `mul_bp` each.

| Food | Base | Rot ticks | Eat ticks | Notes |
|---|---|---|---|---|
| ration | 600 | never | 60 | |
| cooked meal | 900 | 36000 | 60 | composure +10 |
| bread, fruit, produce | 250 | 18000 | 60 | |
| raw meat | 300 | 7200 | 60 | +40 `st.diseased` power |
| raw body part | `min(900, floor_div(mass_g, 10))` | 7200 (rotten at once) | 120 | raw-meat rule; sapient: composure −15 |
| corpse (Eat action) | 1200 | 7200 from death | 600 | +80 `st.diseased` power |
| blood (`afflict.hunger` Feed) | 600 | 600 from death | 120 | |
| fuel for `kin.construct`: coal, oil flask, power cell | 600, 900, 1200 | never | 60 | constructs refuse all other food |

Freshness: age `< rot` fresh × 10000; `rot ≤ age < 2 × rot` stale × 5000 and +30 diseased power; `age ≥ 2 × rot` rotten × 2500 and +60 power. Example: a ration at Survival 50 gives `mul_bp(600, 11000) = 660`.

### 11.3 Composure

Losses are multiplied by the Canon §5.7 MYTHOS term, then by personal factors (product of the bearer's factors in ascending source-ID order, clamped `2500..=20000`).

| Loss event | Composure |
|---|---|
| first sight this Folio of an enemy template tagged eldritch, undead or fiend | minion or standard 5; elite or champion 10; boss, editor or hunter 20 |
| `st.dread` applied | 15 (Canon) |
| `dmg.psychic` HP loss | `mul_bp(psychic_mhp, 5000)` |
| an allied Erratum downed within 30 m; one dies | 10; 20 |
| own part severed | 10 |
| darkness: `light_at ≤ 2` for 10 s or more | 200 milli per Needs tick |
| `st.erasure` stack gained; hunter spawned | 2; 10 |
| eating sapient flesh without `afflict.hunger`; failed Steady check | 15; 10 |

| Gain event | Composure |
|---|---|
| not `in_combat` and `light_at ≥ 8` | 1 per s (Canon) |
| within 3 m of a block tagged `hearth` with `light_block ≥ 12`, not `in_combat` | +2 per s |
| cooked meal; kill of rank elite or higher or of an eldritch-tagged hostile | 10; 5 |
| level-up; Folio exit | 20; 25 |

Example: a 5-point loss in Cosmic Horror (MYT 10) is `mul_bp(5000, 15000) = 7500` milli; with `already_broken` it becomes `mul_bp(7500, 8000) = 6000`.

### 11.4 Temperature

`T_body` is the doc-03 temperature at block `fx_floor_block(e.pos + (0, 900 mm, 0))`. Comfort band in DeciC: `lo = 50 - 10 × floor_div(max(0, d(END)), 5) - insul_cold - bonus`, `hi = 300 + 10 × floor_div(max(0, d(END)), 5) + insul_heat + bonus`, where `insul_*` are armor fields (doc 05) and `bonus` sums `hardy`, Survival and `aug.thermal_regulator`. `st.soaked` raises `lo` by 100. With `dev` the distance outside the band:

| dev | Cold side | Hot side |
|---|---|---|
| 1..=149 | `RowPost(5, 9000)` | `RowPost(5, 9000)` |
| 150..=299 | `st.chilled` applied directly each Needs tick; `RowPost(5, 8000)` | `RowPost(5, 7500)`; composure −200 milli per Needs tick |
| ≥ 300 | as above, plus `10 × floor_div(dev - 200, 100)` `st.frozen` buildup per status pulse | as above, plus `10 × floor_div(dev - 200, 100)` `st.burning` buildup per status pulse |

Example: END 40 at −400 DeciC, no insulation: `dev = 450`: `st.chilled`, regen × 8000, 20 frozen buildup per pulse. END 60 gives `10..=340`.

### 11.5 Stamina, breath and suffocation

- Stamina costs: sprint row 6; dodge 20 (Canon §4.2); swim 5 per s; climb 4 per s; jump 0; melee and block costs per doc 05. Regeneration (row 5) starts after 60 ticks without spending. Reaching 0 applies `st.exhausted` for 120 ticks.
- `res.breath` max = `(10 + floor_div(END, 10) + Σ RowFlat breath) × 1000`. It updates every tick with `k = 60`: −1000 per s while the eye cell holds any liquid or a gas flagged unbreathable (doc 08), else +5000 per s. Exempt: `machine_body`, `deathless`, `aug.gills` in water, a respirator in `slot.face` or the rebreather skin for gases. At 0, `st.suffocating` applies (DoT 500 Bp `dmg.true`, Canon) until breath rises above 0. Example: END 40 gives 14,000; suffocation at 85 HP costs `mul_bp(85000, 500) = 4250` mHP per s.

### 11.6 Encumbrance

`carried_g` = inventory items + equipped non-armor items + worn armor counted by the Armor rule (§5.3) + a carried corpse or Remains. `capacity_g = (row 10 + RowFlat(10)) × 1000`. `load_bp` and the states are Canon §11.1. Example: capacity 45 kg with 50,000 g carried gives `load_bp = 11111`, `st.encumbered`; picking up 75,000 g Remains makes it `st.overburdened`. Armor of 12,000 g at Armor 50 counts 9,600 g.

---

## 12. Death, Death's Door, madness and player-controlled statuses

### 12.1 Death's Door

Canon §10.8 step 1. `dd_used` resets on `ev.folio_entered`. The roll is `keyed_rng("rng.combat", folio, erratum_id, tick, 0xFFFF).roll_bp(row 43)`; seq `0xFFFF` is reserved for it. Success: HP = 1,000 mHP, `st.deaths_door` 60 ticks (invulnerable), part pools unchanged. `dd_used = true` either way. A downed Erratum never rolls again until the next Folio.

### 12.2 Downed hooks (doc 16 owns `st.downed`)

- `revive_hp_mhp(reviver) = mul_bp(max_hp_mhp, 2000 + 20 × reviver.skill.medicine)`: 21,250 mHP at 85 HP with Medicine 25.
- `revive_ticks = 180`, × 5000 with `talent.warden.resurgence`.
- On `ev.player_downed`: allies within 30 m lose 10 composure. Wounds, afflictions and needs persist; natural regeneration is 0 while downed.

### 12.3 Death and resurrection

`ev.player_died` → Remains (Canon §17.6). A resurrected ghost returns with all severed parts restored, wounds cleared, composure at max, and satiation, afflictions and augments unchanged.

### 12.4 Madness

`st.madness` applies when composure reaches 0 and ends when it reaches 25% of max (Canon). Effects: aim sway × 20000 (Canon, doc 05), checks −10, and **episodes**: on each active tick where `(t - a) % 600 == 0`, draw `keyed_rng("rng.check", folio, bearer, t, 0xFFFE).weighted([2500, 2500, 1500, 1000, 2500])`:

| Index | Episode | Effect |
|---|---|---|
| 0 | Fugue | `st.confused` directly |
| 1 | Panic | `st.feared` directly, source the nearest hostile within 30 m (no effect if none) |
| 2 | Rage | `st.berserk` directly |
| 3 | Lucid insight | every hostile within 30 m gains `st.marked` for 300 ticks, visible to the bearer only |
| 4 | Whispers | audio only |

Phantoms are presentation-only, spawned by the bearer's client from `mix64(run_seed ^ entity_id ^ floor_div(tick, 600))`. They never enter sim state, never deal damage and never draw from a sim stream.

### 12.5 Player-controlled statuses (Canon §11.8)

The input layer validates commands; it never synthesizes input.

| Status | Implementation on a player-controlled Erratum |
|---|---|
| `st.charmed` | after step 2.15, `D2 = 0` against the applier's faction (no buildup, no `ev.damage_dealt`); hits on allies skip step 2.15; HUD tints allies hostile; the bearer's minions do not change sides |
| `st.berserk` | block, cast, reload and item-use commands rejected; melee stamina cost 0; × 13000 at step 2.12 |
| `st.feared` | fire and cast rejected while `ang_diff(yaw, bearing to source) ≤ 8192`; its move multiplier is 11000 while the movement direction is within 8192 Bam of the bearing away from the source, else 10000 |
| `st.sleeping` | all commands rejected; ends on damage or an ally's 30-tick interact; duration × 5000 |
| `st.confused` | Canon behavior: movement input rotated by the doc-06 angle |
| `st.taunted` | HUD marks the taunter; damage to others × 7500 |
| `st.hacked` (only a `body.construct` Echo) | as `st.charmed` toward the hacker's faction |
| `st.polymorphed` | primary input = the template's first attack, secondary = its second; equipment stowed; HP ratio kept |
| `st.stunned`, `st.frozen`, `st.staggered`, `st.knocked_down` | all action commands rejected; camera free |
| `st.grappled` | movement rejected; one-handed actions allowed |

---

## 13. Echo bodies

These base speeds replace Canon §4.2's humanoid values; row 11, `BodyMove` and statuses then apply. Jump follows Canon §4.2.

| Plan | Capsule radius / height / eye (mm) | Walk / sprint / crouch / swim / climb (mm/s) | Hands and slots | Rules |
|---|---|---|---|---|
| `body.quadruped` | 400 / 1300 / 1100 | 4800 / 8000 / 2200 / 2000 / 0 | `slot.weapon_1` only (mouth: one-handed melee or a `harness` ranged weapon); no offhand, hands or ring slots; `slot.implant_arms` = forelegs | one leg lost or tier 4: `BodyMove 7500`; two 5000; three 2500; no ladders; Climb −20 |
| `body.avian` | 300 / 1600 / 1450 | 3800 / 5500 / 1800 / 1200 / 1600 | `slot.weapon_1` held in the feet (one-handed); no offhand or hands slot | glides as `aug.wings`; a lost or tier-4 wing stops gliding |
| `body.serpentine` | 350 / 1100 / 1000 | 4300 / 6000 / 2600 / 4800 / 1800 | adds `bp.arm_extra_1` and `bp.arm_extra_2` (750 each from `bp.body_front`), giving `slot.hand_extra_1` and `_2`; two-handed weapons use both; no legs or feet slots | leg rules never apply; footstep noise −6 dB |
| `body.amorphous` | 450 / 1000 / 800 | 3500 / 5000 / 2500 / 3500 / 2400 | one pseudopod: `slot.hand_extra_1`; only `slot.body` armor | crouched height 600 mm; climbs any solid face; immune to `st.bleeding`; nothing severs; only `bp.core` is a precision part |
| `body.construct` | humanoid | humanoid | `bp.limb_1`, `bp.limb_2` bind as `bp.arm_r`, `bp.arm_l`; `bp.limb_3`, `bp.limb_4` act as legs; all humanoid slots | immune to `st.poisoned`, `st.diseased`, `st.bleeding`; no breath; `st.hacked` applies (§12.5) |
| `body.insectoid` | 350 / 1200 / 1050 | 4600 / 7000 / 2200 / 1500 / 3000 | `bp.leg_1`, `bp.leg_2` bind as arms; all humanoid hand slots | climbs any solid face; each lost walking leg (3–6) `BodyMove 8500` |

Kin traits that need a missing part are skipped.

---

## 14. Worked examples

### 14.1 Creating a character

Path Custom → `kin.human` → `origin.cyberpunk` → `calling.netrunner`.

1. Origin: SLT 43, LOG 43, GUI 42, SGT 42, RES 37, PRV 37, CHM 38, END 38; all others 40.
2. Budget 14 (10 + human 4). Refund MEN 40 → 36 (+4) for 18 points. Spend LOG 43 → 50 (7), AGI 40 → 44 (4), HRG 40 → 43 (3), SLT 43 → 47 (4). Total 18.
3. Skills: hacking 20 + 10 (Origin) = 30; tinkering 20; sneak 20; security, awareness, illusion 10; human bonus marksmanship 5.
4. Primaries: MGT 40, FIN 43, VIG 38, INT 43, WIL 40, PER 41, PRE 38, FOR 39.
5. Derived at level 1, VERVE 5: Max HP `40 + 40 + 1 × (3 + 2) = 85`; regen `max(10, 50 + floor_div(-3, 2)) = 48` Bp, so 408 mHP per s; healing received 9850; stamina `50 + 38 + 0 = 88`; stamina regen `15 + 7 = 22`; sprint drain `12 - floor_div(-2, 10) = 13`; aether 61; composure 90; carry 45 kg; move 10100; crit 500; reload 10350; complexity 9; grimoire 4; dark-vision 4 m; footstep radius 12 m; body threshold 94; buy 10060; sell 2940; threat 9600; disguise 9800; Death's Door 850; breath 13 s; tolerance `(0, 2, 1, 1)`; trap detection `500 + 0 + 10 × 50 = 1000`.

### 14.2 Level 1 to 10

Model as §6.3 with 26 kill-equivalents; skill XP per Folio hacking 50, sneak 30, tinkering 25, security 15, awareness 10, marksmanship 10, illusion 6; Unseen in Folios 1, 3, 6, 8; extras 20 tenths.

| Folio | Gain | Cumulative | Level after | Allocation at each new level | Talent | Max HP |
|---|---|---|---|---|---|---|
| 1 | 301 | 301 | 2 | L2: LOG +2, TGH +2 | L2 `backdoor` | 92 |
| 2 | 562 | 863 | 3 | L3: LOG +2, AGI +2 | — | 97 |
| 3 | 1505 | 2368 | 4 | L4: TGH +2, HRG +2 | L4 `firewall` | 104 |
| 4 | 1567 | 3935 | 5 | L5: LOG +2, SLT +2 | — | 109 |
| 5 | 3015 | 6950 | 6 | L6: AGI +2, TGH +2 | L6 `remote_access` | 116 |
| 6 | 5899 | 12849 | 8 | L7: LOG +2, END +2; L8: RES +2, TGH +2 | L8 `puppet_master` | 121, 128 |
| 7 | 4944 | 17793 | 9 | L9: LOG +2, HRG +2 | — | 133 |
| 8 | 9090 | 26883 | 10 | L10: AGI +2, ITU +2 | L10 `daemon`; Marginal Note offer | 138 |

Training marks over these Folios raise LOG by 7, AGI by 4, SLT, PRC and HRG by 2 each, and GUI by 1. At level 10: LOG 67, AGI 54, SLT 51, TGH 48, HRG 49; hacking 50 (the Folio-8 skill-up grants `skill.hacking.m50`), sneak 36, tinkering 33. Max HP at level 10 is `40 + 48 + 10 × (3 + 2) = 138`. INT is `floor_div(67 + 40 + 40, 3) = 49`, which meets the tier-3 gate of 45; tier 3 also needs level 12, so the first tier-3 talent is bought at level 12.

---
## Exports

All in namespaces Canon §19 assigns to doc 04.

| Namespace | IDs defined or given behavior here |
|---|---|
| `attr.` | the 8 Canon primaries: uses, display tiers, talent gates (§4) |
| `sub.` | the 24 Canon sub-attributes: tooltips, doc-04 uses (§3) |
| `skill.` | the 36 Canon skills: XP events, effects (§5); 144 perks `skill.<name>.m25`, `.m50`, `.m75`, `.m100` |
| `talent.` | 156 Calling talents `talent.<calling_suffix>.<name>` (§7.2) and 14 `talent.general.*` (§7.3) |
| `origin.` | 16 Origins `origin.<genre_suffix>` (§2.3) |
| `calling.` | the 13 Canon Callings: presets, kits, trees, key primaries (§2.5, §7) |
| `kin.` | the 6 Canon kin: full trait lists and skins (§2.4) |
| `trait.` | `trait.origin.*` (16), `trait.kin.<kin>.*` (human 1, construct 6, undead 5, beastkin 5, fae 4, mutant 3), `trait.marginalia.*` (8), `trait.echo.*` reserved for doc 14 |
| `aug.` | 33 augments (§9.4), including `aug.prosthetic_arm` and `aug.prosthetic_leg` |
| `afflict.` | the 3 Canon afflictions: contraction, effects, cure (§10) |
| `wound.` | tiers `wound.bruised`, `wound.wounded`, `wound.crippled`, `wound.disabled`, `wound.severed`; kinds `wound.laceration`, `wound.fracture`, `wound.burn`, `wound.trauma` |
| `body.`, `bp.` | Canon plans and parts: wound effects, mass shares, Echo body rules (§8, §13); uses of `bp.tail`, `bp.arm_extra_1`, `bp.arm_extra_2`, `bp.wing_l`, `bp.wing_r` as added parts |

Behavior is also given for the Canon statuses and resources doc 04 owns (Canon §11.1, §12).

Functions exported to other docs: `grant_skill_xp`, `grant_char_xp`, `check(attr, skill, dc)`, `revive_hp_mhp`, `revive_ticks`, `on_part_damage`, `on_heal`, `body_move_bp`, `anl_speed_bp`, `treatment_ticks`, `aug_cap`.

## Interface assumptions

| Doc | Provides |
|---|---|
| 03 | event payloads; temperature per block (DeciC); `light_at`; Needs runs after the status pulse within a tick |
| 05 | archetype → skill map; parry window, block arc, reach, aim sway, heavy attacks, firing move penalty, swap ticks; armor `insul_cold`, `insul_heat`; unarmed archetypes; crutch; `harness` weapons; weapon `sever` flags |
| 06 | spell and prayer `base_cost`, glyph school, school cost factor, glyph-count limit, body-restoring prayer result, summons, `st.confused` angle |
| 07 | body materials (`mat.flesh`, `mat.bone`, …), recipe tiers 1..=5, quality shift, block hardness 1..=10, augment station, `fuel` tag, scout drone, material tiers |
| 08 | `afflict_power` sources, mutagens, red-ink vial, cure potion, unbreathable gas flag, body-part ingredient effects, nourishment potion |
| 09 | lock and device DCs on the §4.2 ladder; use of Sneak, Awareness, Security, Pickpocket and Hacking effects; noise adjustments |
| 10 | appraisal timer using `anl_speed_bp`; per-actor identification events; curse-state reveal |
| 11 | item IDs for kit roles, food, treatments, repair kits, implants, grafts, prosthetics |
| 12 | `secret.` locations, room-visit counts, objective tiers, Scriptorium, `hearth` blocks, check targets |
| 13 | enemy tags (undead, fiend, eldritch, beast_sapient, graftable, regenerating, vampiric, disease, electronic), part-to-attack maps, body mass, attack `afflict_power`, pacify DCs |
| 14 | Echo records and body plans, secret Origin unlocks, `trait.echo.*` content |
| 16 | `st.downed` flow calling `revive_hp_mhp` and `revive_ticks`; XP delivery to present members |
| 17 | `input.ability_1..=4`; creation, level-up and talent screens |
| 18 | `loc.` keys, tooltip templates, kin skin names |

## Canon gaps

1. **Stream `rng.check`.** Non-combat checks and madness episodes need a keyed stream; Canon §3.3 lists none. Assumed: keyed stream `rng.check`, owner 04, used only by §4.2 checks and §12.4 episodes.
2. **Events.** Assumed added to Canon §18: `ev.wound_changed`, `ev.augment_changed`, `ev.pacified`, `ev.secret_found` (doc 12), `ev.objective_completed` (doc 12).
3. **Modifier placement.** Canon §6.4 names only `gear` terms. Assumed: the §1.2 registry (flat terms at the gear position, post factors after the clamp, before statuses). Canon-given effects without a pipeline position are placed here: fae `iron_bane` at §10.5 step 3a, beast-form melee × 15000 at step 2.12.
4. **Body move factor placement.** Canon §4.2 orders base, row 11, statuses, and §9.1 gives leg multipliers without a position. Assumed: `BodyMove` sits after row 11 and before statuses.
5. **Fall-damage multipliers.** Canon §4.2 names only Acrobatics. Assumed: every doc-04 fall-damage factor (fae, qinggong, `roll_with_it`, spring tendons) multiplies there, ascending source ID.
6. **`kin.human` "no changes".** Read as no body or resource rule changes; doc 04 adds creation-only bonuses (+4 points, +5 skill).
7. **`kin.construct` "no satiation".** Read as no food; `res.satiation` is displayed as Fuel with unchanged rules.
8. **`talent_tol` sources.** Assumed: kin, Origin traits, talents and Marginalia add to `talent_tol`; augments add to `gear_tol`.
9. **Implant slots.** Canon §13.2 lists four implant slots without capacity. Assumed: each slot is a region holding up to 4 augment points.
10. **Echo movement.** Canon §4.1–§4.2 give humanoid values. Assumed: §13 values replace them for non-humanoid Echo bodies.
11. **Errata limb severing.** At Canon pool sizes, an 85-HP Erratum's arm (10,625 mHP) severs from one slash-dominant hit of 10,625 mHP on that arm. Doc 04 implements Canon as written. Recommended amendment: Errata limbs sever only when the part was at tier 3 before the hit, or the hit's part damage is at least 5000 Bp of the pool's max.
12. **XP anchor.** Canon-fixed sources give the §6.3 combat build 45.5 units per Folio (skill-ups fill the cap of 8), so arrival at Folio 25 is level 30, inside 28 ± 3 but above center. Recommended: doc 13 tunes kill supply to 34 equivalents (level 29).
13. **Scope numbers.** The commission cited 32 skills, 128 perks and 10 Callings; this document follows Canon 1.2: 36 skills, 144 perks, 13 trees.
14. **Echo trait IDs.** Echo signature traits live in `trait.echo.*` (doc 04 namespace) but are authored by doc 14.

## Acceptance tests

1. **Origin rows.** Each of the 16 rows of §2.3 sums to 0 with every entry in `-6..=6`; `origin.unwritten` is all zero.
2. **Point-buy.** `buy_cost(43, 50) = 7`; `buy_cost(48, 54) = 10`. The §14.1 build spends 18 points with 4 refunded and validates. A build with any value 61, any value 24, or 11 refunded points is rejected.
3. **Creation stats.** The §14.1 character at level 1, VERVE 5: Max HP 85,000; regen 408 mHP/s; stamina 88; sprint drain 13; aether 61; composure 90; breath 13,000; Death's Door 850 Bp; move 10100; reload 10350; tolerance `(0, 2, 1, 1)`.
4. **Checks.** Force PWR 46 vs DC 55 gives 3200 Bp; Persuade CHM 38 with Speech 30 vs DC 40 gives 8200; rating 200 vs DC 10 clamps to 9500; each check makes exactly 1 `range(10000)` call on `rng.check`.
5. **Skill growth.** Hacking 49 with 0 progress receiving 23,600 milli-XP at MEM 40 becomes 50, grants `skill.hacking.m50` and adds one LOG mark; at LOG 66 the fifth mark raises LOG to 67.
6. **Combat XP.** A hit removing 20,000 mHP from a 60,000-mHP hostile grants 500 milli-XP; at MEM 50, 525. A hit on an ally grants 0.
7. **Skill-up cap.** Ten skill-ups in one Folio at d = 10 grant `8 × 504 = 4032` character XP; the counter resets on `ev.folio_entered`.
8. **Kill XP.** At d = 10: standard 176, elite 529; elite in a party of 4 gives 302 each. A minion at d = 1 gives 0.
9. **Levels.** XP 492,999 is level 29; 493,000 and 512,919 are level 30; 542,500 is level 31.
10. **Pacing.** The §6.3 model gives arrival levels 30, 29, 29, 29; hafted reaches 50 in Folio 9 and 75 in Folio 21.
11. **Allocation.** A third point into one sub-attribute from a single batch is rejected; with two pending batches, four points into LOG succeed (two per batch); a point into a sub-attribute at base 100 is rejected.
12. **Marginal Note.** At level 10 the offer holds 3 distinct tier-1 or tier-2 talents from other trees, uses `pool.len() - 1` `range` calls (95 for a 96-talent pool), and reopening shows the same offer.
13. **Respec.** At d = 10 the first talent respec costs 434 Lucre, the second 868; the first attribute respec costs 868.
14. **Talent gates.** A tier-3 talent is refused at level 11 with 5 tree points, accepted at level 12 with key primary 45, refused at key primary 44.
15. **RowPost.** Two × 9000 sources on row 15 give 8100; four × 7000 sources give 2401, clamped to 5000.
16. **Part pools.** At Max HP 85,000: head 12,750, torso 29,750, each arm and leg 10,625. Arm pools 7,970 → tier 0; 7,969 → tier 1; 5,312 → tier 2; 2,656 → tier 3; 0 → tier 4.
17. **Healing cap.** An untreated tier-2 leg at pool 4,000 receiving a heal that restores 45,000 mHP gains 5,625 but stops at 5,312. A bandage at Medicine 0 then sets treated and raises it to 6,905 (tier 1, treated cleared).
18. **Treatment time.** Bandage: 60 ticks at Medicine 0, ANL 40; 39 ticks at Medicine 50, ANL 50. Surgical kit at Medicine 25, ANL 40: 480 ticks.
19. **Severing.** A 60-HP NPC humanoid's arm (pool 7,500) hit for 8,000 mHP with a slash-dominant hit severs and spawns a `cat.body_part`; the same hit with blunt leaves it at tier 4, unsevered. An Erratum's head pool at 0 never severs.
20. **Body part.** A 140,000-g troll's arm weighs 9,100 g; butchering yields 4 meat and 1 bone, or 5 and 2 at Survival 25; it is rotten at age 7200 ticks.
21. **BodyMove.** Left leg tier 3 and right leg tier 2 give 7600; one severed leg with a crutch 7000; both legs severed 2500.
22. **Satiation.** From 1800, 1200 Needs ticks reach 600 and apply `st.hungry`; with `scavenger` the value is 900. Starving at 85 HP loses 850 mHP every 300 ticks.
23. **Food.** A ration at Survival 50 gives 660; a stale cooked meal 450.
24. **Composure.** A 5-point loss in Cosmic Horror costs 7,500 milli; with `already_broken`, 6,000. Composure 0 applies `st.madness`; reaching 22,500 at max 90 ends it.
25. **Temperature.** END 40 at −400 DeciC gives `dev = 450`: `st.chilled`, `RowPost(5, 8000)`, 20 `st.frozen` buildup per status pulse. END 60 gives band `10..=340`.
26. **Breath.** END 40: max 14,000; 14 s submerged reaches 0 and applies `st.suffocating`, costing 4,250 mHP/s at 85 HP. A `kin.construct` never drains.
27. **Encumbrance.** 50,000 g against 45 kg gives `load_bp` 11111 and `st.encumbered`; 12,000 g of armor at Armor 50 counts 9,600 g.
28. **Affliction.** Four bites of power 25 at d = 5 fill a 100 threshold (112), apply `st.incubating` for 7200 ticks, then `afflict.hunger`; the cure at d = 10 costs 1085 Lucre.
29. **Red ink.** Four doses set the hunter threshold to 6 and max composure −20; a fifth fill changes nothing.
30. **Augments.** RES 40 gives `aug_cap` 8; with 8 points installed a 1-point install is refused; if RES drops to 39 every augment runs at potency 8000. `aug.eye_lowlight` at rank 3 gives `RowFlat(31, 6)`.
31. **Prosthetic.** The §9.5 cyberarm example gives C = 6 (hook function) for the Mythic Fantasy origin and C = 4, potency 6000, for the Cyberpunk origin.
32. **Death's Door.** One `rng.combat` draw with seq `0xFFFF` per Folio at most; PRV 37 at VERVE 5 uses 850 Bp; success leaves 1,000 mHP and 60 ticks of `st.deaths_door`.
33. **Madness episodes.** One `rng.check` `weighted` call per 600 active ticks; weights sum to 10000.
34. **Charmed player.** A charmed Erratum's hit on the applier's faction yields `D2 = 0` and no buildup.
35. **Unarmed.** Bare-hand `B` at d = 10 is 13,030 mHP.
36. **Determinism.** Replaying a 10,000-tick input log reproduces all doc-04 state byte for byte.

## Invariants annex

```
Σ doc04: creation ⊗ subs ⊗ skills ⊗ levels ⊗ talents ⊗ body ⊗ augments ⊗ needs
W! sub base ∈ 1..=100 ∧ effective ∈ 1..=150          W! creation subs ∈ 25..=60 ∧ origin Σ = 0
W! skill ∈ 0..=100 ∧ growth := need(s) = 4000 + 400 s   N! flat per-hit combat skill XP
W! zero-effect action → 0 XP                            N! XP from allies | neutrals | own minions | own blocks
W! skill-up char XP counted ≤ 8 per Folio               W! level := cum_xp thresholds, no cap
W! level-up → 4 points, ≤ 2 per sub per batch           W! even level → 1 talent point
W! derived stat := canon row + registered modifiers     N! modifier on rows 1,3,11,12,29,32,39,41,42,43
W! Π RowPost(row) ∈ 5000..=20000                        W! modifier order := ascending source id
W! every random draw ∈ {rng.check, rng.progress, rng.combat, rng.alchemy} keyed
N! draw in severing | wounds | buildup | needs         N! madness phantom ∈ sim state
W! part pool ∈ 0..=max ∧ untreated tier ≥ 2 caps healing   W! Errata head never severs
W! augment numbers × own potency                         W! Disconnected prosthetic keeps structure
W! installed aug points ≤ region cap 4 ∧ ≤ aug_cap at install
W! innate doc-04 effects: dep none unless stated ∧ src_lvl = d_cur
W! Fortune subs → in-run effects only                    N! input synthesized for a player
W! Origin, kin, Calling fixed for the run                N! respec of creation points or training
W! satiation ∈ 0..=2400 ∧ breath ∈ 0..=max               W! afflict.red_ink doses ≤ 4
```
