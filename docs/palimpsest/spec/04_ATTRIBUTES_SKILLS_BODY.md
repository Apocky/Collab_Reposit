# 04 — Attributes, Skills, Progression and Body
As of 2026-10-12. Status: BINDING. Conforms to canon-1.11. Applies DECISIONS.md through row 145.

This document owns creation, sub-attribute growth, skills, levels, talents, the body, augments, kin, afflictions, needs, Death's Door, madness and player-controlled statuses. All of it runs in `palimpsest_sim`. Canon wins every conflict; conflicts are listed under Canon gaps.

Scope note. The original commission named 32 skills, 128 perks and 10 Callings. Canon 1.2 raised these to 36 skills, 144 mastery perks and 13 Callings, and canon 1.4 replaced Delving with Fieldcraft; this document follows the canon everywhere. Canon-1.8 cut mounts and canon-1.10 removed item durability (DECISIONS.md items 55 and 65), so no rule here reads a mount or the wear of an item; placeables and destructible blocks keep their block HP. Where a peer chapter defines a mechanic that a doc-04 perk, talent or XP row modifies (spells in doc 06, brewing in doc 08, appraisal in doc 10, taming and dialogue in doc 13, downing in doc 16), this document states the effect in that chapter's terms so both texts agree.

## Imports

| Canon section | Consumed |
|---|---|
| §1.2–§1.6, §1.11 | numeric types, `floor_div`, `ceil_div`, `mul_bp`, display-unit rule, integer content, ascending-ID iteration |
| §2.2–§2.3 | Needs period 60 (phase 7), status pulse period 15, rate accumulator, `in_combat`, `light_at` |
| §3.3 | keyed `rng.progress` (offers), `rng.combat` (Death's Door, `seq` 0xFFFF reserved) and `rng.check` (non-combat checks and madness episodes, `seq` 0xFFFE reserved); the `seq` allocation rule, under which doc 04's per-tick counters stay below 0x2000; disguise challenges drawn on `rng.stealth` (doc 09) |
| §4.1–§4.3 | humanoid capsule and speeds, jump, dodge; the fall-damage chain (`fall_dmg_bp`, then doc 04's factors in ascending source ID) and the final move-speed chain (row 11, then `body_move_bp`, then statuses); fixed terrain, `block.tag.destructible`, `cat.placeable` and its block HP, resource nodes; no item durability |
| §5.3–§5.5, §5.7 | dependencies, `Req`, `Tol` (`talent_tol`, `gear_tol`), Contradiction, MYTHOS composure term |
| §5.10–§5.11 | World Law `law_grants` (added to `Tol` before the cap of 6), Going Native at a Colophon, `st.unmoored` (Fortune-derived effects off) |
| §6.1–§6.5 | attribute structure, scale, requirements, the 44 master rows |
| §7, §8 | 36 skills, `need(s)`, combat XP normalized by `norm_mhp` before party scaling, training marks, pace contract; level curve, `kill_xp_std`, the 45-unit anchor, kill and pacification XP, co-op split with downed members and ghosts, points per level |
| §9, §9.1 | body plans, shares, condition pools, added-part donors; severing (an Erratum's limb only from a crippled pool, DECISIONS.md item 84), hand bindings |
| §10.5–§10.9 | damage pipeline (step 3a `kin.fae` iron × 12500, step 6c Scribe's Mercy), lethal resolution, healing |
| §11, §12 | statuses owned here, buildup, direct application, player-controlled bearers; resources owned here, max-change rule |
| §13.8, §13.8.1, §15.1 | `P`, `T`, `S`, innate source level; minion cap |
| §16, §17 | Origins, Callings (doc 04 owns kit roles, presets, kit balance and talent trees), Echoes (`echo.fenrir` on `body.quadruped`, `echo.jormungandr` on `body.serpentine`), kin, afflictions; run constants, the Colophon, `mode.abridged` kill-XP factor 16667, party tables, ranks (the Erasure hunter pays `rank.elite` kill XP, §11.6), anchors, ghosts, retired mount IDs (§17.7), invasions and the Redacted outcome (§17.9) |
| §18 | core events; a further event exists once doc 03 §14.3 registers it (`ev.wound_changed`, `ev.augment_changed`, `ev.entity_pacified`, `ev.secret_found`) |

Peer chapters consumed, checked against the files on disk on 2026-10-11. What this document needs from them is listed under Interface assumptions.

| Doc | Sections | Consumed |
|---|---|---|
| 03 | §2.2, §3.3, §3.4, §5.1–§5.2, §5.11, §5.15, §5.20, §5.23–§5.24, §9.4, §14 | system order (`sys.needs`, period 60, phase 7, runs satiation, toxicity, composure and breath), `comp.body_plan`, `comp.pools`, `comp.stats`, the wish-velocity chain, `ev.entity_landed`, `Env.temp`, `eye_submerged`, the temperature hazard layer, the event bus and the events it registers for doc 04 (§14.3) |
| 05 | §1.4, §3.2, §11.3–§11.4, §12.1–§12.2, §14.3, §19 | keyed `seq` allocation, `arch.melee.unarmed` and its natural-weapon skins for non-humanoid Echoes, `harness` weapons, armor weight score, shield parry ticks and `shield_stab`, damage while downed, the severing hook |
| 06 | §6.2, §7.1–§7.4, §12.3, §14.1, §16.3 | `cost_skill`, glyph tier gates, magic mastery hooks and XP inputs, `summon.poly.beast`, dispellability, supplication needs |
| 07 | §2.4–§2.5, §3.2–§3.3, §4.2, §4.4, §5.1, §8.1–§8.4, §9, §10.3–§10.4, §11, §13 | material tags (`fuel`, `edible`, `metal`), the crafter material bonus (Smithing 100), quality hooks (Smithing 50, `masterwork`), craft XP inputs, the workbench implant hook, `self_repair` supplies (`SELF_REPAIR_UNITS`), salvage and forge reforge (Smithing 75), mining and node yields, placeable placement, block HP and repair, gadgets and their Tinkering hooks |
| 08 | §4.3, §8.5, §9.3–§9.7, §9.11, §10.3, §11.1, §15.4 | respirators and rebreathers, weapon coats (`envenom`), ingredient reveal tiers, brew batch, single-ingredient brewing, alchemy XP basis, toxicity skill factor, the `transmuter` hook |
| 09 | §4.6, §8.3, §9.2–§9.4, §11.4, §13.2–§13.7 | holding breath, awareness icons, sneak multiplier and takedowns, disguise bluffs (doc 04's Deceive rating and chance, drawn by doc 09 on `rng.stealth` seq 1006), lock tiers, hacking, traps, pickpocketing |
| 10 | §5.5–§5.6, §6.3, §7.3, §7.8 | appraisal tiers and rate, Examine, the tier-1 reveal set, curse and pact terms |
| 11 | §3.5, §12.2–§12.3 | food, tool and treatment item IDs, haggling, Scriptorium service prices |
| 13 | §4.9, §6, §11, §13.3, §13.7, invader | surrender, minion contract and taming threshold, defeats and the kill-XP reference plan, quest tiers, deterministic dialogue checks, the Redactor invader |
| 14 | §1.2–§1.3, §2.4–§2.5, §3.1, §3.4–§3.5, §3.9, §6.3 | kit templates, mid-run joiners, Rubric, marginal notes, Colophon services, Echo roster, body plans and signature traits |
| 15 | §3.8 | Fold variants B and C of Echo traits |
| 16 | §9.4–§9.6 | `st.downed` revive progress and revived HP, ghosts, instanced loot and Dormant slots |
| 17 | §5.2–§5.7 | allocation, skill, talent and ability-slot screens; the Cosmetics page; breakdown tooltips calling `explain_row` |
| 18 | §5.6 | `art.preset.<kin_suffix>.<nn>` (12 per kin), `art.preset.<kin_suffix>.colors` and `Appearance { preset, base, marking }` |

DECISIONS.md rows consumed: 49 (phantom marking), 55 (no mounts), 65 (no item durability), 84 (Errata severing), 89 (`enemy.role.charger`), 97 (creator presets), 102 and 106 (Second Edition carry-over and depth), 107 (ghosts), 131 and 134 (last stand), 145 (hunger only).

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
| `TolAdd(axis, n)` | adds to `talent_tol[axis]` (Canon §5.4) | kin traits, Origin traits, talents and Marginalia powers; augments add to `gear_tol`; World Law grants (Canon §5.10) add separately; the cap of 6 applies to the whole sum |
| `BodyMove(bp)` | the body move factor (§8.5) | after row 11, before status multipliers |
| `Other` | a named effect with its own formula in its own text | as written |

Allowed rows: **flat** on rows 4, 7, 9, 10, 13, 14, 18, 19, 22, 23, 24, 25, 26, 27, 30, 31, 33, 34, 35, 36, 44 and on `res.breath` max (seconds). **post** on rows 2, 5, 6, 8, 15, 16, 17, 20, 21, 28, 37, 38, 40. Rows 1, 3, 11, 12, 29, 32, 39, 41, 42 and 43 take no doc-04 modifier: they change only through their inputs.

All `RowPost` factors on one row multiply from 10000 in ascending source content-ID byte order, one `mul_bp` each. The product is clamped to `5000..=20000` and then applied once. Example: `talent.gunslinger.steady_aim` (row 15 × 9000) and `aug.eye_targeting` (row 15 × 9000) give `mul_bp(mul_bp(10000, 9000), 9000) = 8100`; spread `10000` becomes `8100`.

**Explanations.** `explain_row(entity, row) -> Explanation` is a pure function of simulation state that doc 17 §5.6 renders: the row's formula text key, every input value, each intermediate result in evaluation order (Canon §1.3 rounding), every `SubFlat`, `RowFlat` and `RowPost` with its source ID and potency, the status multipliers in Canon §11.7 order, and the final value. It draws no random numbers and writes nothing.

Every numeric modifier from an augment or other dependent component is first multiplied by that component's Contradiction potency (Canon §5.5) and is 0 when Disconnected. A binary effect (an immunity, an added part, an unlocked action) is on while `C <= 5` and off when Disconnected.

**Curse and pact terms (doc 10 §7.3, §7.8).** Doc 10's curses and pacts enter this model as follows, each scaled by the item's potency as doc 10 states. Sub-attribute penalties and bonuses are `SubFlat` terms, applied before the Canon §6.2 clamp: `curse.fumbling`, `curse.feeble`, `curse.sluggish`, `curse.frail`, `curse.addled`, `curse.myopic`, `curse.muffled`, `curse.ill_starred`, and `curse.pact.titan`, `curse.pact.seer`, `curse.pact.gambler`, `curse.pact.glass`, `curse.pact.aegis`, `curse.pact.bulwark`, `curse.pact.hush`. The following are `Other` terms with doc 10's formulas, applied after every doc-04 modifier of the same quantity and outside the `RowPost` clamp: natural HP regeneration (`curse.thirsting`, `curse.pact.transfusion`), healing received at Canon §10.9 (`curse.pact.thorns`, `curse.pact.miser`), satiation (`curse.ravenous`, §11.1), composure regeneration and max composure (`curse.dread`, `curse.pact.nightmare`, §11.3), aether regeneration (`curse.siphon`, `curse.pact.transfusion`) and the HP drain of `curse.pact.bloodthirst`, which is HP loss through the Canon §2.3 accumulator and never a hit. `explain_row` lists each of them with its source ID.

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
- **Cosmetics.** On the Custom path the player picks one of the chosen kin's 12 appearance presets with palette and marking colors; there is no slider editor (DECISIONS.md item 97; doc 18 authors the presets and doc 17 draws the screen). On either path the step also applies owned cosmetics. Nothing chosen here changes a number. The Custom look is recorded with the Erratum's creation choices as doc 18 §5.6's `Appearance { preset: Id32, base: u8, marking: u8 }`, where `preset` is one of `art.preset.<kin_suffix>.01` to `.12` and `base` and `marking` index the 8 base and 8 marking swatches of `art.preset.<kin_suffix>.colors` (`0..=7`); an Echo's look is fixed by its record. The look and the display name are presentation data (doc 14 §1.3): they never enter simulation state.
- **Creation output.** The Custom path outputs doc 14 §1.2's `ErratumChoice::Custom { kin, origin, calling, subs: [u8; 24], human_skill: Option<Id32> }`: `subs` holds the §2.2 final sub-attribute values in Canon §6.1 table order, and `human_skill` is `kin.human`'s +5 skill (`None` for every other kin). Doc 04 validates it by §2.7 before Commit or `SlotJoin`. The `Appearance` and the name travel beside it, outside it.
- The Calling and Origin lists show only entries in the run's unlock set (doc 14 `unlock.calling.*`, `unlock.origin.*`). Secret Origins (`origin.oneiric`, `origin.redacted`, `origin.unwritten`, `origin.ur_text`) need `unlock.origin.<genre_suffix>`. Daily and weekly runs offer every Calling and every core Origin (doc 14 `unlock.daily_baseline`).
- `CreationState` has no other transitions. Confirm runs the §2.7 validation; a failure keeps the state at PointBuy (Custom) or PickEcho (Echo) and names the failing rule.
- Mid-run joiners use the same flow and then §6.6. Going Native (§2.8) is the only later change to a creation choice.

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
4. `spacer` (Space Opera): `res.breath` max +10 s; while in doc 03's `Air` move mode, `RowPost(15, 8500)`.
5. `wired` (Cyberpunk): augment total capacity +2 (§9.2); hacking action time × 8000.
6. `scavenger` (Post-Apocalypse): satiation loss × 7500 (§11.1); stale and rotten food add no `st.diseased` buildup.
7. `quickdraw` (Weird West): weapon swaps take `max(1, ceil_div(swap_ticks × 5000, 10000))` ticks; the first hit within 60 ticks after a swap completes gets `DmgAdd +2500`.
8. `gumshoe` (Hardboiled Noir): `RowFlat(35, 1000)`; appraisal work `need` × 5000 (doc 10 §5.5). The row-35 roll keeps Canon §3.3's cadence of once per 60 ticks.
9. `qinggong` (Wuxia): one extra air jump per airborne period at height `mul_bp(JUMP_H, 6000)`, in addition to Canon's VERVE-9 air jump; fall damage × 7500. Air jumps are counted by `air_jump_max(e) = [vrv(e) >= 9] + [qinggong] + [aug.spring_tendons is on]`, which doc 03 §5.7 compares with its `air_jumps` counter; every air jump uses `mul_bp(JUMP_H, 6000)`.
10. `gearwright` (Steampunk): your deployed `cat.gadget` entities have Max HP × 12500 and recharge × 12000.
11. `courtesy` (Fairy Tale): doc 13 §13.7 dialogue checks add +10 to `value`; Handling counts +10 toward doc 13 §6 taming thresholds; at run start every `fac.native.fairy_tale.*` faction gains +100 reputation (party-shared, Canon §15; applied once per run however many party members carry `courtesy`).
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
- `self_repair`: a 120-tick channel (broken by non-DoT damage or 500 mm of movement, as in §8.4) that needs a carried `cat.tool` tagged `repair_kit` (doc 11 `item.tool.toolkit`), which is never consumed, and consumes doc 07 §8.2's `SELF_REPAIR_UNITS` (1) `metal` material unit; without either the channel is refused (`no_supplies`, doc 07 §8.2). It heals `mul_bp(max_hp_mhp, 1000 + 20 × skill.tinkering)` through Canon §10.9. It also treats the most severe untreated wound of any kind on one chosen part, as a bandage would (§8.4). This is Canon §16.3's "Tinkering repair heals". Example: Tinkering 20 at Max HP 85 heals `mul_bp(85000, 1400) = 11,900` mHP.
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

Majors and minors are Canon §16.2. Each preset spends 10 points at 1 each. Points beyond 10 (`kin.human`, `origin.unwritten`) go +1 each to the preset's listed sub-attributes in listed order, wrapping around, so a human Vanguard preset is PWR +4, TGH +4, BRN +3, RSV +3. No preset value exceeds 50 for any kin, Origin and Calling (the largest is ANL 49 for a human Cosmic Horror Alchemist: 40 + 4 + 5), so every preset point costs 1. Every Erratum also carries the universal kit (doc 14 `run.kit.universal`): 2 rations, 2 bandages, 1 type-identified healing potion and `mul_bp(50, P(1)) = 50` Lucre. Kit items are Common, Normal, +0, ilvl 1, `buc.uncursed`, `know.identified`, skinned to the Origin genre. Doc 14 §2.4 binds each role below to an exact template (`run.kit.<calling_suffix>`) and owns alt kits; this list fixes the roles.

- **vanguard**: preset PWR +3, TGH +3, BRN +2, RSV +2; kit: 1H hafted weapon, medium shield, heavy armor (head, body, legs).
- **duelist**: preset PRC +3, AGI +3, CHM +2, FAT +2; kit: 1H long blade, 6 throwing knives, light armor set.
- **gunslinger**: preset PRC +4, AGI +2, SLT +2, FAT +2; kit: sidearm with 60 rounds, 4 throwing blades, light armor set.
- **grenadier**: preset GRP +3, TGH +3, SLT +2, PWR +2; kit: heavy ranged weapon with 120 rounds, 4 grenades, medium armor set.
- **spellwright**: preset ATN +3, LOG +3, FOC +2, MEM +2; kit: casting focus, glyph tome (2 glyphs), light armor set.
- **alchemist**: preset ANL +4, SLT +2, RES +2, LOG +2; kit: dagger, field alembic (a mortar and an alembic, both of which doc 08 §9.1 needs to brew), 3 bombs, 4 empty flasks.
- **shade**: preset AGI +4, PRC +2, SLT +2, GUI +2; kit: short blade, lockpicks, light armor set, smoke bomb.
- **artificer**: preset LOG +4, BRN +2, PRC +2, ANL +2; kit: sidearm with 60 rounds, toolkit, 1 deployable turret, medium armor.
- **warden**: preset BRN +3, RES +3, RSV +2, TGH +2; kit: spear, tower shield, medium armor set, 1 healing glyph.
- **oracle**: preset ITU +3, PRV +3, MEM +2, FOC +2; kit: staff, holy symbol (prayer focus), light armor set.
- **netrunner**: preset LOG +4, AGI +2, HRG +2, SLT +2; kit: sidearm with 60 rounds, hacking deck, light armor set.
- **beastbinder**: preset CHM +3, SGT +3, MEM +2, END +2; kit: spear, 1 tamed companion beast, light armor set.
- **monk**: preset PWR +3, AGI +3, END +2, FOC +2; kit: hand wraps (unarmed archetype), light armor set.

### 2.6 Echo path

An Echo record (doc 14 §6.3) fixes `kin`, `origin`, `calling`, a signature trait `trait.echo.<echo_suffix>` with variants `trait.echo.<echo_suffix>_b` and `_c` (doc 14 authors them inside this namespace; doc 15 Folds 2 and 5 unlock the variants) and a body plan, default `body.humanoid`; `echo.fenrir` is `body.quadruped` and `echo.jormungandr` is `body.serpentine` (Canon §16.2, §13). Its creation preset is fixed by rule, so no allocation is authored: the Echo's sub-attributes are the Quick start build of its kin, Origin and Calling (§2.2, §2.5), its skills follow §2.2 step 7, and a `kin.human` Echo's +5 skill goes to the Calling's first listed major. The Echo receives its Calling's kit (doc 14 §2.4) and plays exactly like a Custom character from there.

#### 2.6.1 Trait power-budget validator

Every `trait.echo.*` record, each variant separately, lists 1 to 3 **terms**. Each term names a stat key from the registry below, a kind, an optional `when` condition and a polarity (`benefit` or `drawback`). The content validator computes:

```
magnitude_bp(term) = match kind {
    mul(m)       => |m - 10000|,                    // a Bp factor
    add_bp(a)    => |a|,                            // a Bp addend
    flat(n, ref) => floor_div(|n| × 10000, ref),    // ref = the stat at all-40, level 1, VERVE 5 (Canon §21 test 13), or the canon constant it replaces
    count(n)     => 5000 × |n|,                     // slots, charges, doses, stacks, yields
    unlock       => 5000,                           // a capability the base game lacks
}
scope(term)  = narrow if the term has a `when` condition (light band, HP band, a status present, a damage-type subset), else the registry scope
weight       = global 3 | broad 2 | narrow 1
points(term) = ceil_div(magnitude_bp × weight, 2500)
benefit = Σ points of benefit terms;  drawback = Σ points of drawback terms;  net = benefit - drawback
valid ⇔ 1 ≤ #terms ≤ 3 ∧ benefit ≤ 6 ∧ drawback ≥ 1 ∧ -1 ≤ net ≤ 3 ∧ each polarity agrees with its key's better direction
```

Variants B and C of one Echo must have the same `net` as variant A (doc 15's "equal budget"). Origin and kin traits are not budgeted.

| Scope | Stat keys (better direction is up unless marked ↓) |
|---|---|
| global | `max_hp`, `max_stamina`, `move_speed`, `crit_chance`, `damage_dealt`, `damage_taken` ↓, `healing_received`, `skill_xp` |
| broad | `max_aether`, `max_composure`, `composure_loss` ↓, `poise`, `block_stability`, `reload_speed`, `swap_speed`, `dodge_distance`, `dodge_cost` ↓, `damage_dealt.<skill>`, `crit_damage.<skill>`, `damage_taken.<damage type>` ↓, `spell_potency.<school>`, `spell_cost.<school>` ↓, `potion_potency`, `potion_duration`, `brew_yield`, `gadget_charges`, `gadget_recharge`, `tool_time` ↓ (lockpicking and hacking), `grimoire_slots`, `complexity_budget`, `minion_max_hp`, `buy_price` ↓, `sell_price` |
| narrow | every other key, including `crafting_time` ↓, `block_yield`, `disguise_rate` ↓, `swim_speed`, `food_satiation`, `toxicity_decay`, `breath`, `status_buildup.<status>.<skill>`, `throwable.<class>`, `range.<weapon class>`, `reload_speed.<weapon class>`, `self_damage.<source>` ↓, `prayer_healing`, `exit_preview` |

Worked examples. All 24 doc 14 variant-A traits pass; the acceptance test recomputes every one.

| Echo | Terms → points | benefit / drawback / net |
|---|---|---|
| `fenrir` | `damage_dealt.unarmed` add_bp 2000, broad → 2; `buy_price` mul 11000, broad → 1 | 2 / 1 / 1 |
| `sun_wukong` | `damage_dealt.polearm` add_bp 1500, broad → 2; `move_speed` add_bp 500, global → 1; `damage_taken` add_bp 500, global → 1 | 3 / 1 / 2 |
| `loki` | `spell_potency.illusion` add_bp 2000, broad → 2; `prayer_healing` mul 7500, narrow → 1 | 2 / 1 / 1 |
| `nix_seventeen` | `tool_time` mul 6000, broad → 4; `max_hp` mul 9500, global → 1 | 4 / 1 / 3 |
| `tallow` | `food_satiation` add_bp 2500, narrow → 1; `potion_duration` add_bp −1500, broad → 2 | 1 / 2 / −1 |
| `lan_ninth_peak` | `dodge_distance` flat +1000 of 3000 → 3333, broad → 3; `dodge_cost` flat +6 of 20 → 3000, broad → 3 | 3 / 3 / 0 |
| `kallias_twice_drowned` | `swim_speed` mul 15000, narrow → 2; `breath` unlock, narrow → 2; `damage_taken.fire` add_bp 1000, broad → 1 | 4 / 1 / 3 |

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

### 2.8 Going Native (Canon §5.11)

At a Colophon, once per run per Erratum (doc 14 §3.9, `going_native_used`), the player may set its Origin to the genre `g` of that Colophon's Folio. The change applies on the interact tick, in this order:

1. `origin := origin.<g suffix>`. `origin_tol` is recomputed from the Canon §5.4 table and every component's Contradiction is re-evaluated, emitting `ev.contradiction_changed` per change.
2. The Origin trait is replaced by `g`'s Origin trait (§2.3). Trait state (cooldowns, timers, once-per-Folio flags) resets. `blank_page` grants nothing after creation, because its effects are creation choices.
3. Creation history stays: the §2.3 sub-attribute modifiers, the +10 Origin skill bonus and every allocation are unchanged.
4. Doc-04 components whose `Req` reads "the Origin genre" (Acrobatics 50 wall jump, Monk `wind_step`, mutations granted by `kin.mutant` or radiation) use `g` from this tick.
5. Kin skins and the cosmetic native form switch to `g` (doc 18). The Erratum's `Appearance` (§2.1) is kept unchanged.

Example: a Cyberpunk-origin Netrunner Going Native in a Wuxia Colophon gets `origin_tol` (1, 0, 1, 2) instead of (0, 2, 1, 1) and `qinggong` instead of `wired`; its SLT, LOG, GUI and SGT creation bonuses and its Hacking +10 stay. If installed augments now exceed `aug_cap` because `wired` is gone, §9.2's Strained rule applies.

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
- **Endurance** (`sub.endurance`, END). Stamina and breath. Governs Athletics, Fieldcraft. Rows 4, 5, 6; `res.breath` max `10 + floor_div(END, 10)` s (Canon §12). Doc 04: temperature comfort band (§11.4); Endure checks. Tooltip: "Endurance {v}: stamina {r4}, regen {r5}/s, breath {breath} s, comfort {lo}..{hi} °C."
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

- **Charm** (`sub.charm`, CHM). Governs Speech, Handling. Rows 37, 38. Doc 13 §13.7 Persuade `value`. Tooltip: "Charm {v}: buy ×{r37}, sell {r38} of value."
- **Menace** (`sub.menace`, MEN). Governs Leadership. Row 39 (threat). Doc 13 §13.7 Intimidate `value`. Tooltip: "Menace {v}: threat ×{r39}."
- **Guile** (`sub.guile`, GUI). Governs Illusion, Pickpocket. Row 40 (disguise). Doc 13 §13.7 Deceive `value`; disguise Deceive checks (§4.2). Tooltip: "Guile {v}: disguise suspicion ×{r40}."

**FORTUNE** (`attr.fortune`). In-run effects only; never paid odds or account currency (Canon §22).

- **Fate** (`sub.fate`, FAT). Governs no skill. Rows 13 (crit, 10 Bp per point), 41 (status buildup). Doc 04: Fortune checks. Tooltip: "Fate {v}: crit {r13}, status buildup ×{r41}."
- **Serendipity** (`sub.serendipity`, SER). Governs no skill. Row 42 (loot rarity shift, Canon §13.4). Tooltip: "Serendipity {v}: loot rarity shift {r42}."
- **Providence** (`sub.providence`, PRV). Governs Devotion. Rows 43 (Death's Door), 44 (curse resistance). Tooltip: "Providence {v}: Death's Door {r43}, curse resistance {r44}."

Row 12 reads whichever sub-attributes a weapon's scaling coefficients name (doc 05 §3.2, §5.2), so every sub-attribute can feed weapon damage. Other chapters read sub-attributes inside their own mechanics, never as competing derived stats: doc 08 §9.5 brew quality uses base ANL and FAT; doc 10 §5.5 and §5.6 appraisal rate and Examine use ITU and ANL; doc 11 §12.2 haggling and doc 13 §13.7 dialogue use effective CHM, GUI and MEN; Canon §4.2 dodge invulnerability uses AGI. Tooltips show those values too when the item or dialogue is on screen (doc 17).

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
success   = keyed_rng("rng.check", folio, actor, tick, seq).roll_bp(chance_bp)    // exactly 1 draw (2 under avatar_of_fate, §7.2)
```

`attr_value` is the named effective sub-attribute or primary. `seq` is the actor's check counter on this tick: 0 for its first check, +1 for each further check on the same tick, always below 0x2000 (Canon §3.3). Situational modifiers: talent and trait bonuses as written; `st.exhausted` −10; `st.madness` −10; each untreated wound of tier ≥ 2 on the part the check uses (arms for Force, Lift, Climb, Squeeze; legs for Balance; head for Recall, Decipher, Investigate) −5.

Difficulty ladder:

| Tier | DC | Tier | DC |
|---|---|---|---|
| Trivial | 10 | Severe | 70 |
| Easy | 25 | Heroic | 85 |
| Standard | 40 | Legendary | 100 |
| Hard | 55 | Mythic | 120 |

A failed check against the same target (entity, item uid or block position) cannot be retried by the same actor for 600 ticks unless the rating has risen. **Persistence:** each failure adds 1000 Bp to that actor's next `chance_bp` against that target, at most +3000, added before the clamp; a success clears it. The bonus is stored per (actor, target) in Folio state and cleared on `ev.folio_entered`. It changes no draw count. A successful check whose catalog row names a skill grants that skill `100 × DC` milli-XP (§5.2).

Dialogue and taming are not random checks. Doc 13 §13.7 resolves Persuade, Deceive and Intimidate choices deterministically (`value = skill + floor_div(sub_eff - 40, 2) + floor_div(rep, 100)` against `dc`, pass when `value >= dc`), and doc 13 §6 resolves taming by a Handling threshold. Doc-04 bonuses that name those verbs (`courtesy`, `silver_tongue`, `beast_tongue`, Speech 100, Handling 75) add to doc 13's `value`, `dc` or Handling count as their text says, and a passed dialogue check grants `100 × dc` milli-XP to its skill (§5.2).

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
| Deceive | GUI + speech | answer a disguise challenge (doc 09 §11.4, `DC = 40 + 10 × bluffs`); doc 09 makes its one success draw on keyed `rng.stealth` seq 1006, never on `rng.check` (Canon §3.3); dialogue Deceive choices use doc 13 §13.7 instead |
| Fortune | FAT | scripted luck events |

Example: Force with PWR 46 against a Hard bar (55): `5000 + (46 - 55) × 200 = 3200` Bp; after two failures on the same bar it is `3200 + 2000 = 5200` Bp. Climb with GRP 38 and Athletics 30: `rating = 38 + floor_div(180, 10) = 56`; against Standard 40, `5000 + 16 × 200 = 8200` Bp.

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
- Anti-grind: the same lock, trap, device, item uid or corpse yields XP once per run for each row; allies, neutrals, own minions and own placeables give 0 combat XP; movement, landing, stealth-movement and exposure rows have the §5.2 Folio caps, which reset on `ev.folio_entered`.

### 5.2 XP events

Values are milli-XP before row 28. `dmg` is HP actually removed after ward (Canon §10.5 step 7), `norm` the hostile target's party-normalized Max HP `norm_mhp = max(1, floor_div(target_max_mhp × 10000, party_hp_bp))`, where `party_hp_bp` is the Canon §17.2 enemy HP multiplier applied to that target (10000 solo; Canon §7.1, DECISIONS.md item 85), `own_max` the actor's own Max HP in mHP, `ally_max` the healed Erratum's Max HP in mHP, `val_n = floor_div(value × 10000, P(d_cur))` (an item's value normalized to depth 1), `base_cost` a spell's skill-independent cost in m-aether (doc 06 §6.1), `DC` the check or lock difficulty (doc 09 locks use the §4.2 ladder values). Every combat row is the Canon §7.1 damage-normalized formula; no row pays a flat amount per hit.

| Skill | Event | milli-XP |
|---|---|---|
| blade, hafted, polearm, unarmed, marksmanship, heavy_arms, throwing | hit on a hostile with that skill's archetype (doc 05 mapping) | `min(1500, floor_div(1500 × dmg, norm))` (Canon §7.1) |
| throwing | thrown non-damaging item takes effect on ≥ 1 hostile | 300, once per item uid |
| shield | hostile hit blocked or parried (pre-block total `blocked`) | `min(1500, floor_div(3000 × blocked, own_max))`; a parry counts `2 × blocked`, same cap |
| armor | hostile hit taken on a part with AR > 0 (`d_armor` = the armor-type share after step 4) | `min(1000, floor_div(2000 × d_armor, own_max))` |
| acrobatics | dodge whose invulnerability negates ≥ 1 hostile hit (`avoided` = Σ `base_mhp` of the negated hit records, doc 05 §1.5) | `min(1500, floor_div(3000 × avoided, own_max))` |
| acrobatics | `ev.entity_landed` with `fall_mm ≥ SAFE_FALL_mm - 1000` | `min(1500, 200 + floor_div(fall_mm - SAFE_FALL_mm + 1000, 10))`; cap 15,000 per Folio |
| athletics | stamina spent on sprint, swim or climb | 30 per 1 stamina; cap 30,000 per Folio |
| evocation, abjuration, alteration, conjuration, illusion, restoration, divination | spell, wand, text or summon hit on a hostile; the stage's school earns it, summons earn Conjuration (doc 06 §7.4) | `min(1500, floor_div(1500 × dmg, norm))` |
| same schools | effective non-damaging root resolution | `floor_div(base_cost, 20)` |
| spellcraft | any effective root resolution | 100 per modifier, trigger and condition glyph in the spell, at most 500 |
| spellcraft | first effective cast this run of a newly compiled spell with `g` glyphs | `2000 × g`; at most 3 per Folio |
| devotion | supplication answered pleased or helped; petition; sacrifice (doc 06 §16) | 2000; `20 × favor_cost` before Devotion scaling; `10 × favor_gained` |
| any | fizzle or wild surge | 0 |
| sneak | each Needs tick while crouched or `st.hidden`, moving, with `n` hostiles within 10 m below `aware.alert` | `40 × min(3, n)`; cap 24,000 per Folio |
| sneak | sneak-attack kill or lethal takedown (doc 09 §9.4); Folio exit that earns the Unseen bonus | 1500; 10,000 |
| sneak | kill of a target below `aware.alert` toward you that your own thrown item, noisemaker or alarm test lured within the last 600 ticks (doc 09 §12) | 750, in addition to other rows |
| security | lock picked or trap disarmed | `150 × DC` |
| pickpocket | item stolen from a hostile or neutral; item planted | `min(8000, 1500 + 40 × val_n)`; 1500 |
| smithing, tinkering, inscription | craft completed (doc 07 recipe, `recipe_tier` 0..=5; process recipes are tier 0 and pay `1000 × input_units`, doc 07 §4.4, §14.3) | `1000 × input_units + 2000 × recipe_tier`, × 15000 if quality ≥ `q.excellent` |
| alchemy | brew completed (doc 08 §9.11 `brew_basis`, `new_reveals`); murky draught | `600 × brew_basis + 1000 × new_reveals`; 0 |
| alchemy | ingredient tasted; potion type identified by brewing or tasting | `250 × new_reveals`; 3000 |
| smithing, tinkering, inscription | salvage, to the item's governing skill (doc 07 §8.1, §8.3) | 300 per output unit |
| tinkering | hit on a hostile by your turret, drone or coil (doc 07 §11.1) | `min(750, floor_div(750 × dmg, norm))` |
| inscription | text inscribed; enchantment applied (doc 06 §7.4); Rewrite performed; lore object read (doc 12) | `floor_div(base_cost, 10)`; `2000 × points`; `2000 + 1000 × rarity_index` (common 0 … palimpsestic 6); `200 × d_cur` |
| appraisal | doc 10 §5.5 appraisal completion or §5.6 Examine success on an item | `min(6000, 1000 + 30 × val_n)` |
| appraisal | type identified; curse sensed on pickup | 3000; 2000 |
| medicine | wound treated | `1500 × tier`; cap 20,000 per Folio |
| medicine | part reattached; surgery; augment installed or removed | 8000; 6000; 4000 |
| medicine | healing item applied to another Erratum | `min(1500, floor_div(1500 × healed, ally_max))` |
| survival | butcher output unit (at most 4,000 per corpse or part); forage harvest; cook | 800; 500; `1000 × input_units`; combined cap 30,000 per Folio |
| survival | each Needs tick outside the comfort band (§11.4) | 20; cap 6,000 per Folio |
| speech | shop trade completed (each item uid once per run) | `5 × val_n` |
| speech | haggle with `score >= 0` (doc 11 §12.2, once per shop per Folio) | `100 × (20 + min(60, d_cur))` |
| speech, leadership | passed dialogue check (doc 13 §13.7: Persuade and Deceive train Speech, Intimidate trains Leadership); successful disguise Deceive check (§4.2) | `100 × dc`; `100 × DC` |
| speech | reputation gained | 10 per point |
| leadership | ally hit on a hostile while that ally is in your aura | `min(375, floor_div(375 × dmg, norm))` |
| leadership | Rally removes ≥ 1 status | 1500 |
| hacking | device hacked; construct hacked | `150 × DC`; `floor_div(3000 × rank_xp_bp, 10000)` |
| fieldcraft | placeable placed (each placeable uid once per run); placeable repaired; destructible block broken by your melee, tool or charge; mineral or salvage node harvested (doc 07 §9.3; Canon §4.3) | 300; 300 per 1000 Bp of its max block HP restored; 300; 500; combined cap 40,000 per Folio |
| fieldcraft | trap, mine, snare or demolition charge you set hits a hostile; a non-damaging trap or snare you set takes effect on a hostile | `min(1500, floor_div(1500 × dmg, norm))`; 300 |
| awareness | trap or secret detected by row 35; hidden hostile revealed by your perception | 3000; 2000 |
| handling | hit on a hostile by your tamed beast or hireling | `min(1500, floor_div(1500 × dmg, norm))` |
| handling | Tame success (doc 13 §6) | `floor_div(3000 × rank_xp_bp, 10000)` |

An "effective" root resolution changed something other than HP damage: HP restored, `res.ward` raised, a status applied or removed, a summon spawned, a block or object changed, or an object revealed. Damage pays only through the damage rows, never twice. `SAFE_FALL_mm` is Canon §4.2's `SAFE_FALL` in mm at the landing tick.

Examples: a frost bolt dealing 20,000 mHP to a 60,000-mHP hostile grants Evocation 500 and, with 2 modifier glyphs, Spellcraft 200. A non-damaging alteration with `base_cost` 15,000 m-aether grants `floor_div(15000, 20) = 750`, or `mul_bp(750, 10500) = 787` at MEM 50. A dodge that negates a hit with `base_mhp` 10,000 at Max HP 85 grants `floor_div(30,000,000, 85,000) = 352`. Crouch-walking past three unaware guards earns 120 per second and stops at 24,000 for the Folio. In a party of 4 a standard hostile of that template has Max HP `mul_bp(60000, 25500) = 153,000` mHP (Canon §17.2) and `norm = floor_div(153000 × 10000, 25500) = 60,000`, so the same 20,000-mHP frost bolt still grants Evocation 500.

### 5.3 Skill effects and mastery perks

Mastery perks are granted automatically on reaching 25, 50, 75 and 100 and are never lost. IDs: `skill.<name>.m25`, `.m50`, `.m75`, `.m100` (144 perks). Every school skill also has the Canon row 29 term (`30 Bp` per level), doc 06 §6.2's cost factor `12500 - 50 × cost_skill` with `cost_skill = floor_div(3 × school_skill + skill.spellcraft, 4)`, and doc 06's glyph tier gates (tiers 1, 2, 3, 4 need 0, 25, 50, 75). Every combat skill has the Canon row 12 term (`30 Bp` per level). Where a perk changes another chapter's mechanic, its text here matches that chapter's hook table (doc 06 §7.3, doc 08 §8.5 and §9.7, doc 10 §5.5); this document owns the perk IDs and grants them.

**Combat**

- **Blade** (PRC). 25: blade hits +15 `st.bleeding` status power. 50: parry window +3 ticks with blades. 75: blade crits apply 1 `st.exposed` stack directly. 100: blade hits `pen_bp` +2000, capped 10000.
- **Hafted** (PWR). 25: heavy attacks +20 `st.stunned` power. 50: heavy-attack stamina × 8000. 75: hafted poise damage × 12500. 100: heavy hits on a `st.staggered` target apply `st.knocked_down` directly.
- **Polearm** (GRP). 25: melee reach +300 mm. 50: an attack hitting ≥ 2 hostiles gets `DmgAdd +1000` on each. 75: a hit on a hostile whose velocity component toward you is at least 4300 mm/s on the impact tick applies `st.staggered` directly. 100: while crouched, `DmgAdd +3000`.
- **Unarmed** (PWR). Bare hands use doc 05's `arch.melee.unarmed` (base 6600 mHP, `dmg.blunt` 10000, precision 12500, `src_lvl = d_cur`), so `B = mul_bp(6600, P(d_cur))` before attack factors. 25: unarmed stamina × 7500. 50: unarmed hits +25 `st.stunned` power. 75: empty-handed blocks act as a shield with stability 3000. 100: hits on `st.staggered` or `st.stunned` targets `DmgAdd +5000`.
- **Shield** (BRN). Block stability `+30 × skill` Bp through doc 05 §12.2's `shield_stab`; parry window = the shield archetype's parry ticks (doc 05 §12.1) + `floor_div(skill, 25)`; tower shields stay at 0. 25: block stamina loss × 8500. 50: shield bash applies `st.staggered` to targets of rank standard or minion. 75: block arc +4096 Bam. 100: perfect parries reflect projectiles at their shooter (Canon §10.6 reflected entry).
- **Marksmanship** (PRC). 25: aimed spread `RowPost(15, 9000)`. 50: ranged reloads `RowPost(17, 11000)`. 75: ranged precision multiplier +2500 Bp. 100: the first shot after 60 ticks without firing crits; the crit roll is still drawn and its result replaced.
- **Heavy Arms** (GRP). 25: `RowPost(16, 9000)` for heavy weapons. 50: firing removes no move speed (doc 05 firing penalty off). 75: heavy-weapon explosion radius +500 mm. 100: heavy weapons fire while sprinting.
- **Throwing** (SLT). 25: `RowPost(20, 12000)` for thrown items. 50: throw action ticks × 8000. 75: thrown hits +20 status power. 100: one input throws 2 items from the stack, 1024 Bam apart.

**Defense and mobility**

- **Armor** (TGH). Worn armor counts `mul_bp(mass_g, 10000 - 40 × skill)` toward load. 25: `RowFlat(23, 10)`. 50: tier-1 wounds have no effect (§8.3). 75: natural armor `mul_bp(5, T(d_cur))` on every part. 100: `RowFlat(18, 15)`; armor adds no footstep noise (doc 09).
- **Acrobatics** (AGI). Fall damage × `10000 - 60 × skill` (Canon §4.2). 25: dodge cost −3 stamina (§11.5). 50: wall jump, once per airborne period, dependency verve with `Req` from the Origin genre. 75: dodge invulnerability +2 ticks, still at most 12. 100: a dodge that negates a hit restores 10 stamina.
- **Athletics** (END). Sprint, swim and climb stamina × `10000 - 30 × skill` (as `RowPost(6, ·)` for sprint). 25: `res.breath` max +4 s. 50: `RowPost(5, 11000)`. 75: `RowFlat(10, 10)`. 100: immune to `st.exhausted`.

**Magic and faith**

These ten perk sets are doc 06 §7.3's hook table; doc 06 owns the mechanics they modify.

- **Evocation** (ATN). 25: damage-essence status power × 12000. 50: the charge factor caps at 17500, reached at `c = 90`. 75: chain forms +1 jump. 100: orb, nova and rune radius × 12500.
- **Abjuration** (RSV). Dispel count `2 + floor_div(skill, 25)` (doc 06 §14.1). 25: ward magnitude × 12000. 50: dispel count +1. 75: counterspell silence lasts 180 ticks instead of 60. 100: a ward cast also applies `st.fortified` for 300 ticks.
- **Alteration** (FOC). Shape power `3 + floor_div(skill, 25)` (doc 06 §13.2). 25: helpful direct-application durations × 12500. 50: shape power +2. 75: polymorph break threshold 4000 Bp of Max HP instead of 2000. 100: swiftness and levitate glyph costs × 5000.
- **Conjuration** (MEM). Summon duration × `10000 + 50 × skill`; Canon §15.1 minion cap. 25: summon duration × 15000. 50: summon Max HP × 12500. 75: summons resist every type +1500 Bp, within their caps. 100: summon attacks carry status power × 20000.
- **Illusion** (GUI). 25: veil duration × 15000. 50: control status power × 12500. 75: targets you charm deal damage × 12000. 100: phantasm spawns 2 copies.
- **Restoration** (RES). 25: heal magnitude × 11000, before row 3. 50: mend also ends one DoT status (the lowest status ID). 75: healing beyond Max HP becomes `st.shielded` up to 1000 Bp of Max HP. 100: a mend that reaches a downed ally completes its revival at once under doc 16 §9.4 (HP set to doc 16's `revive_hp_bp`), once per ally per Folio.
- **Divination** (ITU). 25: sight radius × 15000. 50: the keen modifier costs 0 (doc 06), and every hostile within 30000 mm shows its awareness-state icon toward you through walls (doc 09 §8.3). 75: condition glyphs cost 0 complexity. 100: you see `st.invisible` entities within 12000 mm.
- **Spellcraft** (LOG). A 25% weight in every spell's `cost_skill`; tier gates of triggers, conditions and Spellcraft modifiers. 25: modifier cost addends × 8000. 50: depth surcharge −1 (doc 06 §4.4). 75: trigger `max_fires` +1. 100: a stage holds 4 modifiers.
- **Devotion** (PRV). Favor terms (Canon §16.5); prayer potency is row 29 with Devotion as the school skill (doc 06). 25: petition favor cost × 8000. 50: a supplication answered pleased resolves 2 needs. 75: prayers to non-local patrons apply × 12500. 100: the first supplication each Folio ignores the prayer timer.

**Stealth**

- **Sneak** (AGI). Input to doc 09 detection. 25: crouched movement noise −3 dB. 50: sneak-attack multiplier +5000 Bp (doc 09). 75: awareness decay toward you after line of sight breaks × 15000 (doc 09). 100: sneak attacks carry `ignore_armor` (Canon §10.5 step 5).
- **Security** (SLT). Input to doc 09 locks. 25: lockpick slip chance (doc 09 §13.2 `slip_bp`) × 5000. 50: a disarmed worldgen mechanical trap yields one redeployable `cat.placeable` item, `block.trap.spikes` (Canon §4.3, doc 07 §10.4, doc 09 §13.6). 75: your lock tier (doc 09 §13.2 `your_tier`) +1, so you may attempt locks one ladder tier higher. 100: non-quest mechanical locks open in 60 ticks without picks.
- **Pickpocket** (GUI). 25: steal time × 7500. 50: plant items, primed throwables included. 75: steal equipped non-weapon items. 100: a failed steal sets awareness to `aware.suspicious`, never `aware.alert`.
- **Hacking** (LOG). Hack time × `10000 - 50 × skill` (doc 09). 25: hack cameras from 10 m. 50: hacked constructs `DmgAdd +2000`. 75: two hacked units at once. 100: `st.hacked` on non-boss constructs lasts until Folio exit.
- **Awareness** (HRG). Row 35 term and doc 09's opposed check. 25: `RowFlat(34, 4)`. 50: `RowFlat(33, 5)`. 75: you see `st.invisible` entities within 4 m. 100: hostiles within 6 m that are hidden from your party gain `st.marked` for you (8 s, refreshed each Needs tick).

**Craft**

- **Smithing** (BRN). Input to doc 07 quality. 25: `RowFlat(19, 500)`. 50: a crafted weapon or armor rolling `q.normal` becomes `q.good`. 75: reforge one affix at a forge (doc 07 cost). 100: weapons, armor and shields you craft store `mat_bonus_bp` 500, so their `material_bp` or `mat_ar_bp` is +500, capped at 14000 (doc 07 §3.2, §3.3).
- **Tinkering** (LOG). Input to doc 07 gadget quality. 25: gadgets +1 charge. 50: turret Max HP × 12500. 75: craft `cat.implant` items at a workbench (§9.3). 100: gadget recharge × 15000.
- **Alchemy** (ANL). Doc 08 inputs: brew quality `f` (§9.5), toxicity taken × `10000 - 25 × skill` (§11.1), coat charges (§10.3). 25: you see effect 2 of every ingredient (doc 08 §8.5). 50: you see effect 3; a taste reveals two effects; brew batch +1 (doc 08 §9.7). 75: you see all four effects. 100: brew batch +1 more; single-ingredient brewing (doc 08 §9.3).
- **Inscription** (MEM). Enchantment tier gates 0, 25, 50 and text complexity limit `4 + floor_div(skill, 10)` (doc 06 §8.3, §9.3); Rewrite XP (§5.2). Perks are doc 06 §7.3's: 25: +1 enchantment capacity point per item. 50: text inscription aether × 7500. 75: 4 enchantments per item. 100: on-use enchantments +1 use per Folio.
- **Fieldcraft** (END). Placing, repairing and arming placeables (Canon §4.3) takes `max(1, ceil_div(base_ticks × 10000, 10000 + 100 × skill))` ticks (doc 07 sets `base_ticks`); your melee and tool damage to blocks tagged `block.tag.destructible` × `10000 + 50 × skill`. 25: placeables you place have block HP × 12500. 50: traps and demolition charges you set deal damage and status power × 13000. 75: mineral and salvage nodes you harvest (doc 07 §9.3) yield +1 unit, and your placeable repairs restore × 15000 (doc 07 §10.4). 100: placing a placeable takes 1 tick, and your demolition charges break destructible blocks in a radius 1000 mm larger. Example: a 120-tick barricade at Fieldcraft 50 takes `ceil_div(1,200,000, 15,000) = 80` ticks.

**Knowledge**

- **Appraisal** (ITU). Doc 10 §5.5 appraisal tier and rate `max(500, 1000 + 30 × skill + 10 × d(ITU))` milli-work per second × `anl_speed_bp`. 25: your appraisal completions apply doc 10's tier-1 reveal set, which makes the value line exact (doc 10 §5.5, §6.3); the value never shows before the reveal set that includes the curse state. 50: the tier-0 reveal set applies on your pickup with no work. 75: `RowFlat(36, 2000)`. 100: every equipment item you carry for 600 consecutive ticks gets every required knowledge flag and, if type-hidden, its type.
- **Medicine** (ANL). Treatment time (§8.4); healing items you apply × `10000 + 30 × skill`. 25: reattachment and surgery (Canon §9.1). 50: splints and salves also heal 500 Bp of Max HP. 75: one treatment treats every untreated wound on the part. 100: install and remove augments without a station.
- **Survival** (SGT). Food satiation × `10000 + 20 × skill`; comfort band widened by `10 × floor_div(skill, 10)` DeciC each side. 25: butchering +1 unit. 50: forage and cook time × 5000. 75: immune to `st.diseased` from food. 100: `st.starving` damage × 5000.

**Social**

- **Speech** (CHM). The skill term of doc 13 §13.7 Persuade and Deceive `value`; disguise Deceive rating; `RowPost(37, 10000 - 10 × skill)` and `RowPost(38, 10000 + 10 × skill)`. 25: shops show one extra hidden stock item. 50: bribe a sapient non-boss hostile for `mul_bp(floor_div(100 × rank_xp_bp, 10000), P(d_cur))` Lucre; it is pacified (§6.1). 75: reputation gains × 15000. 100: doc 13 dialogue checks you make use `dc - 15`.
- **Leadership** (MEN). The skill term of doc 13 §13.7 Intimidate `value`. Allied Errata and minions within 10 m get `DmgAdd +5 × skill` (max +500). 25: minion Max HP × 11500. 50: active Rally removes `st.feared` and `st.dread` from allies within 12 m; cooldown 1800. 75: aura radius 15 m. 100: allies in the aura +500 Bp crit chance (gear term).
- **Handling** (CHM). Minion cap (Canon §15.1); doc 13 §6 taming threshold. 25: your tamed beasts and hirelings regain 50 Bp of their Max HP per s while you are not `in_combat`. 50: tamed beasts Max HP × 12000. 75: beasts of rank elite can be tamed; doc 13 §6's threshold adds +30 for them. 100: your tamed beasts and hirelings resist every damage type +2000 Bp, within their caps.

---
## 6. Character XP and levels

### 6.1 Sources

Canon §8 fixes kills and pacifications, skill-ups, Folio exit and Unseen; doc 04 lists the pacification methods with doc 13 and adds the capped non-kill sources marked "04".

| Source | XP | Owner |
|---|---|---|
| Kill | `mul_bp(mul_bp(K(d), rank_xp_bp), 7000)`, `e = enemy_level(d) = d + floor_div(d, 5)` | Canon |
| Pacify: a hostile made non-hostile for the rest of the Folio by a spared surrender (doc 13 §4.9), a subdue takedown (doc 09 §9.4; it counts on the entity's first subdue, even though the NPC can wake later), a dialogue Persuade, Intimidate or bribe (doc 13 §13.7; Speech 50), a Tame (doc 13 §6) or a Hacking-100 hack | exactly the kill XP of that entity, paid as a doc 13 §11.1 defeat; once per entity; the entity can never also grant kill XP; `ev.entity_pacified` instead of `ev.entity_killed` | Canon (methods: 04, 13) |
| Skill-up | `2 × K(d_cur)`, at most 8 per Folio | Canon |
| Folio exit | `4 × K(d)` | Canon |
| Unseen | `20 × K(d)` | Canon |
| Discovery | 10 tenths per `secret.` found (doc 12; max 2 per Folio); 5 tenths when the party has visited 50% of the Folio's rooms and 5 more at 100% | 04 |
| Identification | 2 tenths per instance you bring to `know.identified`; 5 tenths per type you identify; cap 10 tenths per Folio | 04 |
| Crafting | `3 × recipe_tier` tenths per craft you complete (doc 07, tier 0..=5; a tier-0 process recipe pays 0 tenths); a completed brew counts as tier `min(5, brew_basis)` (doc 08 §9.11); cap 40 tenths per Folio | 04 |
| Objective | the Rubric completed, or the Editor killed in a boss Folio (doc 14 §3.4): 10 tenths; each completed marginal note (doc 14 §3.5): 5 tenths; each completed quest (doc 13 §13.3, tier 1..=3): `5 × tier` tenths | 04 |

The four doc-04 rows (Discovery, Identification, Crafting, Objective) share one cap of **40 tenths per character per Folio**; the counter resets on `ev.folio_entered`. They pay on the tick of their event, except marginal notes, which doc 14 checks at exit commit. In `mode.abridged` each doc-04 grant is × 16667 Bp after the cap, the factor Canon §17.1 applies to kill XP; Canon-fixed sources keep their Canon values. Example: the Rubric at d = 10 pays `floor_div(10 × 252, 10) = 252`, or `mul_bp(252, 16667) = 420` in Abridged.

Kill XP by rank (`K(1) = 5`, `K(10) = 252`, `K(25) = 1386`):

| Rank | d = 1 | d = 10 | d = 25 |
|---|---|---|---|
| minion (3000) | 0 | 52 | 290 |
| standard (10000) | 3 | 176 | 970 |
| elite (30000) | 10 | 529 | 2910 |
| champion (60000) | 21 | 1058 | 5821 |
| boss (200000) | 70 | 3528 | 19404 |
| editor (400000) | 140 | 7056 | 38808 |
| hunter (50000): the invader and a Colophon hunter; the Erasure hunter pays the elite row (Canon §11.6) | 17 | 882 | 4851 |

### 6.2 Co-op

- Kill and pacify XP: each party member present in the Folio, downed members and ghosts included, gains `floor_div(xp × 10000, enemy_count_bp[n])` (Canon §8, §17.2). Dormant and Departed slots (doc 14, doc 16 §9.6) gain nothing. A minion's kill counts as its owner's (Canon §15.1).
- Exit, Unseen, Discovery and Objective XP: the full amount to each present member, ghosts included, Dormant and Departed slots excluded.
- Skill-up, Identification and Crafting XP: the actor only.

Example: an elite at d = 10 in a party of 4: `floor_div(529 × 10000, 17500) = 302` XP each.

### 6.3 Pacing simulation

**Harness.** The doc 02 bots harness reproduces these tables. Solo, Standard mode, Grade 0, row 28 held at 10000 (MEM 40) for the whole run, and doc 13 §11.2's reference plan: per Volume, positions 1 to 4 are standard Folios (position 3 holds the arena mini-boss) and position 5 is the Editor Folio; the plan fixes every Folio's count per rank, its enemy count `n` (Keeper, mini-boss, Editor and its 20 adds included) and its kill XP `kill_plan`. Over the 25 Folios the plan supplies 37.3 standard-kill equivalents per Folio against Canon §17.4's 40, and the combat build's total income averages 45.5 units per Folio, on Canon §8's 45-unit anchor. Each build has a kill share `ks`: on a standard Folio it defeats `n_build = floor_div(n × ks, 10000)` enemies and earns `mul_bp(kill_plan, ks)` kill XP; in a boss Folio it defeats all `n` and earns `kill_plan`. Per Folio, each skill gains its fixed XP plus its per-enemy XP times `n_build`, plus the standard-Folio and Unseen terms listed, all in milli-XP against `need(s)`. Skill-ups are counted `min(8, ups)` and pay `2 × K` each; Exit pays `4 × K`, Unseen `20 × K`, and the doc-04 rows `floor_div(tenths × K, 10)`. The level is read after Folio 24, which is arrival at Folio 25.

| Build | Kill share on standard Folios | Skills: start, then XP per Folio (fixed) or per enemy defeated | Unseen | Doc-04 tenths per Folio | Level after F10 | Level on arrival at F25 | Major skill 50 / 75 |
|---|---|---|---|---|---|---|---|
| Combat (Vanguard) | 10000 | hafted 20: 1.5 per enemy; armor 20: 30; shield 20: 25; athletics 10: 12; Origin skill 10: 6; acrobatics 0: 6; leadership 10: 4; smithing 10: 3 | never | 15 (Rubric 10, half the rooms 5) | 13 | 29 | hafted F10 / F22 |
| Stealth (Shade) | 4000 | blade 20: 1.5 per enemy; sneak 20: 24, plus 1.5 per enemy on standard Folios, plus 10 when Unseen; security 20: 20; pickpocket 10: 12; acrobatics 10: 8; illusion 10: 6; awareness 0: 8; Origin skill 10: 6 | standard Folios at positions 1 and 2 | 30 on standard Folios (Rubric 10, one secret 10, half the rooms 5, `run.note.silent` 5), 15 in boss Folios | 13 | 29 | sneak F12 / —; blade F18 / — |
| Crafting (Artificer) | 6500 | marksmanship 20: 1.0 per enemy; tinkering 20: 30, plus 0.5 per enemy; smithing 20: 30; appraisal 10: 15; fieldcraft 0: 15; inscription 10: 10; security 10: 8; Origin skill 10: 6 | never | 40 (the cap) | 13 | 29 | tinkering F13 / —; marksmanship F20 / — |

Arrival XP: combat 480,822, stealth 453,843, crafting 452,175; level 29 spans 446,600 to 492,999 XP, which is Canon §8's anchor and inside the 28 ± 3 target. Canon §7.2's pace contract binds the combat build, whose main skill meets it exactly (50 at Folio 10, 75 at Folio 22). The stealth and crafting builds defeat fewer enemies by design, so the contract does not bind them; on arrival at Folio 25 their majors stand at sneak 69, blade 56 and security 47 (stealth) and tinkering 68, smithing 57 and marksmanship 54 (crafting); the combat build's hafted stands at 77.

Combat build, per Folio (XP). This is doc 13 §11.2's reference run row for row.

| Folio | Position | Type | K | Kill | Counted skill-ups | Skill-up XP | Exit | Doc-04 rows | Gain | Cumulative | Level | Hafted |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 1 | standard | 5 | 73 | 8 | 80 | 20 | 7 | 180 | 180 | 1 | 25 |
| 2 | 2 | standard | 14 | 244 | 8 | 224 | 56 | 21 | 545 | 725 | 3 | 29 |
| 3 | 3 | standard | 25 | 588 | 8 | 400 | 100 | 37 | 1125 | 1850 | 4 | 31 |
| 4 | 4 | standard | 39 | 913 | 8 | 624 | 156 | 58 | 1751 | 3601 | 5 | 35 |
| 5 | 5 | boss | 75 | 2965 | 8 | 1200 | 300 | 112 | 4577 | 8178 | 7 | 39 |
| 6 | 1 | standard | 98 | 2157 | 8 | 1568 | 392 | 147 | 4264 | 12442 | 8 | 42 |
| 7 | 2 | standard | 123 | 2456 | 8 | 1968 | 492 | 184 | 5100 | 17542 | 9 | 45 |
| 8 | 3 | standard | 151 | 3645 | 7 | 2114 | 604 | 226 | 6589 | 24131 | 10 | 46 |
| 9 | 4 | standard | 182 | 4396 | 8 | 2912 | 728 | 273 | 8309 | 32440 | 11 | 49 |
| 10 | 5 | boss | 252 | 9981 | 8 | 4032 | 1008 | 378 | 15399 | 47839 | 13 | 52 |
| 11 | 1 | standard | 291 | 6440 | 8 | 4656 | 1164 | 436 | 12696 | 60535 | 14 | 54 |
| 12 | 2 | standard | 333 | 6687 | 8 | 5328 | 1332 | 499 | 13846 | 74381 | 15 | 57 |
| 13 | 3 | standard | 378 | 9147 | 6 | 4536 | 1512 | 567 | 15762 | 90143 | 16 | 58 |
| 14 | 4 | standard | 425 | 10314 | 8 | 6800 | 1700 | 637 | 19451 | 109594 | 17 | 60 |
| 15 | 5 | boss | 529 | 20972 | 8 | 8464 | 2116 | 793 | 32345 | 141939 | 19 | 63 |
| 16 | 1 | standard | 585 | 12993 | 7 | 8190 | 2340 | 877 | 24400 | 166339 | 20 | 65 |
| 17 | 2 | standard | 644 | 12878 | 6 | 7728 | 2576 | 966 | 24148 | 190487 | 21 | 66 |
| 18 | 3 | standard | 705 | 17060 | 6 | 8460 | 2820 | 1057 | 29397 | 219884 | 22 | 67 |
| 19 | 4 | standard | 770 | 18634 | 8 | 12320 | 3080 | 1155 | 35189 | 255073 | 23 | 69 |
| 20 | 5 | boss | 907 | 35982 | 5 | 9070 | 3628 | 1360 | 50040 | 305113 | 25 | 71 |
| 21 | 1 | standard | 980 | 21660 | 7 | 13720 | 3920 | 1470 | 40770 | 345883 | 26 | 73 |
| 22 | 2 | standard | 1055 | 21103 | 7 | 14770 | 4220 | 1582 | 41675 | 387558 | 27 | 75 |
| 23 | 3 | standard | 1134 | 27459 | 5 | 11340 | 4536 | 1701 | 45036 | 432594 | 28 | 75 |
| 24 | 4 | standard | 1215 | 29396 | 5 | 12150 | 4860 | 1822 | 48228 | 480822 | 29 | 77 |

Stealth and crafting builds, per Folio (XP):

| Folio | Stealth kill | Stealth Unseen | Stealth gain | Stealth cumulative | Stealth level | Crafting kill | Crafting gain | Crafting cumulative | Crafting level |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 29 | 100 | 244 | 244 | 2 | 47 | 167 | 167 | 1 |
| 2 | 97 | 280 | 699 | 943 | 3 | 158 | 494 | 661 | 2 |
| 3 | 235 | 0 | 810 | 1753 | 4 | 382 | 982 | 1643 | 4 |
| 4 | 365 | 0 | 1262 | 3015 | 5 | 593 | 1529 | 3172 | 5 |
| 5 | 2965 | 0 | 4577 | 7592 | 6 | 2965 | 4765 | 7937 | 7 |
| 6 | 862 | 1960 | 5076 | 12668 | 8 | 1402 | 3754 | 11691 | 8 |
| 7 | 982 | 2460 | 6271 | 18939 | 9 | 1596 | 4548 | 16239 | 9 |
| 8 | 1458 | 0 | 4931 | 23870 | 10 | 2369 | 5993 | 22232 | 10 |
| 9 | 1758 | 0 | 5944 | 29814 | 11 | 2857 | 7225 | 29457 | 11 |
| 10 | 9981 | 0 | 15399 | 45213 | 13 | 9981 | 16029 | 45486 | 13 |
| 11 | 2576 | 5820 | 15089 | 60302 | 14 | 4186 | 11170 | 56656 | 14 |
| 12 | 2674 | 6660 | 16993 | 77295 | 15 | 4346 | 11672 | 68328 | 15 |
| 13 | 3658 | 0 | 12352 | 89647 | 16 | 5945 | 15017 | 83345 | 16 |
| 14 | 4125 | 0 | 13050 | 102697 | 17 | 6704 | 16904 | 100249 | 17 |
| 15 | 20972 | 0 | 32345 | 135042 | 19 | 20972 | 33668 | 133917 | 19 |
| 16 | 5197 | 11700 | 29182 | 164224 | 20 | 8445 | 20145 | 154062 | 20 |
| 17 | 5151 | 12880 | 32843 | 197067 | 21 | 8370 | 23826 | 177888 | 21 |
| 18 | 6824 | 0 | 18809 | 215876 | 22 | 11089 | 26599 | 204487 | 22 |
| 19 | 7453 | 0 | 25163 | 241039 | 23 | 12112 | 29052 | 233539 | 23 |
| 20 | 35982 | 0 | 51854 | 292893 | 25 | 35982 | 57750 | 291289 | 25 |
| 21 | 8664 | 19600 | 50804 | 343697 | 26 | 14079 | 37599 | 328888 | 26 |
| 22 | 8441 | 21100 | 45366 | 389063 | 27 | 13716 | 34816 | 363704 | 27 |
| 23 | 10983 | 0 | 34797 | 423860 | 28 | 17848 | 45064 | 408768 | 28 |
| 24 | 11758 | 0 | 29983 | 453843 | 29 | 19107 | 43407 | 452175 | 29 |

Sensitivity, every row inside 28 ± 3:

| Variant | Level on arrival at F25 |
|---|---|
| combat without doc-04 rows | 29 |
| combat with doc-04 rows at the 40-tenth cap | 30 |
| stealth with Unseen on every standard Folio and the 40-tenth cap | 31 |
| stealth with a 2500 kill share | 28 |
| crafting without doc-04 rows | 28 |
| combat with every Folio's kill XP set to Canon §17.4's 40 standard-kill equivalents, `40 × mul_bp(K, 7000)` | 30 |

The stealth extreme is the top of the band: it arrives with 569,486 XP. Without the 40-tenth cap (60 tenths) it would arrive with 591,316 XP, still level 31 and 3,884 XP short of level 32 (595,200); the cap keeps it 21,830 XP lower.

### 6.4 Level-up flow

```rust
fn grant_char_xp(c: &mut Erratum, amount: i64, src: XpSource) {
    c.xp += amount;
    while c.xp >= cum_xp(c.level + 1) {            // cum_xp(L) = Σ_{l=1}^{L-1} 50 l (l + 3)
        c.level += 1;
        emit(ev.level_gained { entity: c.id, level: c.level });
        c.pending.push(Batch { level: c.level, left: 4, per_sub: [0; 24] });
        if c.level % 2 == 0 { c.talent_points += 1; if c.kin == kin.mutant { c.mutation_points += 1; } }
        if c.level % 10 == 0 { c.interp_offer = Some(interpolation_offer(c, 0)); }
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
- **Interpolation** (every 10th level; the name keeps it apart from doc 14's marginal notes): the pool is every tier-1 and tier-2 talent of the other 12 Calling trees that the character lacks, sorted by ID. Draw `keyed_rng("rng.progress", folio, actor, level_up_tick, seq).shuffle(pool)` (`pool.len() - 1` `range` calls; `seq` 0 for a level-up, `2 + i` for a joiner's `i`-th offer, §6.6) and offer the first 3. The player takes one for free or declines; the offer is stored and never rerolled. An Interpolated talent ignores tree-point gates and does not count toward that tree.

### 6.5 Respec

At a Scriptorium (doc 12):

- **Talents**: refunds every talent point spent (not Interpolated talents); cost `mul_bp(200 × (n + 1), P(d_cur))` Lucre, `n` = prior talent respecs this run. At d = 10: 434, then 868.
- **Attributes**: reverses `alloc_history` and restores the batches; creation values and training are untouched; cost `mul_bp(400 × (n + 1), P(d_cur))`.
- Kin, Calling and Echo choices never change. The Origin changes only by Going Native (§2.8).

### 6.6 Mid-run joiners (doc 14 §2.5)

A joiner enters at `join_level` with XP `cum_xp(join_level)` and:

- receives `join_level - 1` allocation batches of 4 points, one per level `2..=join_level`, spent in the join screen under §6.4's rules (each batch keeps its own limit of 2 per sub-attribute);
- receives `floor_div(join_level, 2)` talent points, and as many mutation points if `kin.mutant`;
- receives one Interpolation offer per multiple of 10 up to `join_level`, the `i`-th (from 0) drawn at the join tick with `seq = 2 + i`;
- takes doc 14's skill values (majors `20 + min(30, 2 × (d - 1))`, minors `10 + min(15, d - 1)`), then adds the Origin +10 and `kin.human` +5; every mastery perk at or below the resulting values is granted; these raises add no training marks and no character XP;
- starts the Folio with `folio_skillups_counted = 0` and every doc-04 Folio cap at 0.

Example: a human joining at level 9 in Folio 8 gets 8 batches (32 points), 4 talent points, majors 34 and minors 17, plus its Origin and kin bonuses.

### 6.7 Second Edition

Second Edition (DECISIONS.md item 102; doc 14 owns the mode) continues the characters that cleared the Ur-Text. Their level, XP, skills and skill progress, training marks, sub-attributes, unspent points, talents, Interpolations, augments, mutation points and afflictions carry over unchanged, and the level curve has no cap (Canon §8). Per-Folio counters reset on `ev.folio_entered` as in any Folio, and every depth-keyed formula here (`K(d)`, `P`, `T`, `S`) reads depth `25 + 2 × n` for Second Edition Folio `n` (DECISIONS.md item 106; Canon gaps 3). Example: Second Edition Folio 3 reads depth 31. Ghosts enter as docs 14 and 16 rule (DECISIONS.md item 107).

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

**Vanguard.** T1: `heavy_hand` hafted hits `DmgAdd +1000`; `bulwark` `RowFlat(18, 10)`; `shield_wall` `RowFlat(19, 1000)` while a shield is equipped; `second_wind` when stamina reaches 0, restore 50% of max stamina, cooldown 3600. T2: `crushing_blow` heavy hafted poise damage × 13000; `taunting_roar` (active) `st.taunted` directly on hostiles within 8 m, 20 stamina, cooldown 1200; `plated` natural armor `mul_bp(4, T(d_cur))` on every part; `unyielding` immune to `st.knocked_down` while blocking. T3: `juggernaut` while you sprint, the first tick your capsule touches a non-boss hostile's capsule (doc 03 §5.14) applies `st.staggered` directly to it, once per target per 300 ticks; `retaliation` perfect parries reflect `mul_bp(blocked, 3000)` as thorns; `rallying_presence` allies within 10 m get `RowFlat(18, 10)` and +1 composure per s. T4: `last_bastion` (needs `unyielding`) below 2500 Bp HP gain `st.fortified` and `st.empowered` for 600 ticks, cooldown 5400.

**Duelist.** T1: `riposte` the next blade hit within 60 ticks of a parry `DmgAdd +5000`; `fleet_foot` dodge cost −3 stamina (§11.5); `en_garde` +500 Bp crit chance with one-handed blades (gear term); `flourish` blade killing blows restore 10 stamina. T2: `lunge` (active) 4 m dash into a blade attack, 25 stamina, cooldown 360; `feint` a dodge that negates a hit makes your next hit within 60 ticks apply 1 `st.exposed` stack; `challenge` (active) one hostile within 20 m takes `DmgAdd +1500` from you for 600 ticks, cooldown 1800; `silver_tongue` doc 13 dialogue `value` +10 and disguise Deceive rating +10, `RowPost(37, 9500)`. T3: `blade_dance` each blade hit within 90 ticks of the previous adds `DmgAdd +300`, max 5 stacks; `uncanny_dodge` dodge invulnerability +2 ticks (cap 12); `coup_de_grace` blade hits on targets below 2000 Bp HP use the precision multiplier on any part. T4: `thousand_cuts` (needs `blade_dance`) every 5th consecutive blade hit on one target within 120 ticks applies 3 `st.bleeding` stacks directly and crits (roll drawn, result replaced).

**Gunslinger.** T1: `steady_aim` `RowPost(15, 9000)` while crouched or still; `fast_hands` `RowPost(17, 11000)`; `long_shot` `DmgAdd +1000` beyond 20 m; `trick_toss` thrown hits `DmgAdd +1500`. T2: `fan_the_hammer` the first 3 pistol shots after a reload fire at interval × 8000; `deadeye` ranged precision multiplier +2500 Bp; `dive_shot` shots during a dodge ignore movement spread; `quick_draw` swap ticks × 7000. T3: `mark_for_death` (active) `st.marked` on a target within 40 m and your hits on it `DmgAdd +1500` for 480 ticks, cooldown 1200; `ricochet` a bullet hitting a block bounces once to the nearest hostile within 6 m at × 5000; `high_noon` crits with a magazine weapon refund 1 round. T4: `dead_mans_hand` (needs `deadeye`) each kill within 300 ticks of your previous kill adds +500 Bp crit chance, max +2500, cleared 300 ticks after the last kill.

**Grenadier.** T1: `blast_padding` your own area hits on you × 5000; `bandolier` throwable stack size +2; `big_iron` `RowPost(16, 8500)` for heavy weapons; `sapper` your explosions' block damage × 15000. T2: `cook_off` grenades held ≥ 60 ticks `DmgAdd +1500`; `shrapnel` your explosions +40 `st.bleeding` power; `bracing` `RowPost(15, 7000)` for heavy weapons while crouched; `demolition` your explosions break destructible blocks 1000 mm farther. T3: `chain_reaction` a hostile killed by your area hit explodes, 2 m, `B = mul_bp(its max HP, 1500)`, `dmg.fire`, never chains; `suppressing_fire` 4 heavy-weapon hits on one target within 60 ticks apply `st.slowed` directly; `walking_arsenal` with a heavy weapon active `RowFlat(18, 20)` and no knockback. T4: `carpet_bomb` (needs `cook_off`, active) throw 5 copies of the selected throwable in a 4096 Bam fan for the cost of 1, cooldown 3600.

**Spellwright.** T1: `efficient_casting` aether × 9000; `glyph_scholar` `RowFlat(27, 1)`; `overchannel` toggle: aether × 15000 and `DmgAdd +2000` on spells; `arcane_focus` `RowFlat(30, 1500)`. T2: `extend` spell durations × 12500; `widen` spell area radius +1000 mm; `mana_shield` while aether ≥ 25% of max, 2000 Bp of each hit's HP loss is paid 1:1 from aether instead; `quicken` cast ticks × 8500. T3: `spell_echo` every 5th effective cast costs 0 aether; `elemental_attunement` choose a damage type when learned, spell hits of it `DmgAdd +2000`; `complex_lattice` `RowFlat(26, 3)`. T4: `grand_design` (needs `complex_lattice`) `RowFlat(26, 4)`, `RowFlat(27, 1)`, and one Signature spell at aether × 7000 with `DmgAdd +1500`.

**Alchemist.** T1: `potent_brews` brewed potency +1000 Bp; `iron_stomach` toxicity gains × 7500; `quick_quaff` drink ticks × 6000; `splash_master` thrown potion radius +1000 mm. T2: `double_batch` brews yield +1; `volatile_mix` your bombs `DmgAdd +1500`; `field_medic` healing potions you drink or throw × 12000; `transmuter` 3 potions of one identified type become 1 potion of another type at an alembic (doc 08 §15.4 sets the pool and makes 1 `weighted` draw on keyed `rng.alchemy`, `seq` 2). T3: `catalyst` bomb status power × 13000; `philosophers_vial` once per Folio a drunk potion is not consumed; `antivenom` immune to `st.poisoned` from your own items, resist `dmg.toxin` +2500. T4: `magnum_opus` (needs `potent_brews`, Alchemy ≥ 75) potions you brew are `buc.blessed`.

**Shade.** T1: `soft_step` footstep noise −6 dB; `backstab` sneak-attack multiplier +5000 Bp; `light_fingers` pickpocket and lockpick ticks × 8000; `nightborn` `RowFlat(31, 3)`. T2: `vanish` (active) `st.invisible` 180 ticks, cooldown 2400; `quiet_kill` sneak-attack kills emit no `ev.noise` beyond 2 m; `envenom` your coats (doc 08 §10.3) get +5 charges, and the coat's buildup factor 5000 becomes 10000; `escape_artist` `st.grappled` and `st.rooted` durations × 5000. T3: `shadow_mark` sneak attacks apply `st.marked` directly; `death_from_above` sneak attacks with `dy ≥ 2500` mm `DmgAdd +5000`; `ghost_walk` sight detection of you while crouch-walking × 8000 (doc 09). T4: `unseen_blade` (needs `backstab`, Sneak ≥ 50) sneak attacks carry `ignore_armor`.

**Artificer.** T1: `overclock` gadget and energy-weapon recharge × 12000; `field_repair` your placeable repairs restore × 15000 (one more `mul_bp` in doc 07 §10.4's repair formula, after the Fieldcraft 75 factor); `scrap_savant` salvage +1 unit per 3; `turret_tech` turret limit +1. T2: `modular` weapons you craft get +1 option per perk column (max 3); `drone_companion` each Folio starts with a scout drone gadget deployed (doc 07); `reinforced` armor and shields you craft store `mat_bonus_bp` +500, so `mat_ar_bp` +500 (cap 14000, doc 07 §3.3); `hardlight` your gadgets' Max HP × 12500. T3: `masterwork` your crafted quality shifts one step up, max `q.peerless`; `self_destruct` your gadgets explode on destruction, 2 m, `B = mul_bp(20000, P(src_lvl))`, `dmg.fire`; `smart_rounds` ammo you craft `DmgAdd +1000`. T4: `legion` (needs `turret_tech`) turret and drone limit +2; each of your deployed gadgets gives you `DmgAdd +300`, max +1500.

**Warden.** T1: `guardian` hits on allies within 2 m that cross your front block arc strike your block instead; `mender` healing you cause × 11000; `warding` wards you create × 12000; `steadfast` `RowFlat(24, 20)`. T2: `intercept` (active) dash up to 8 m to an ally and give it `st.shielded` of 1500 Bp of its Max HP, cooldown 1200; `purge` your restoration casts remove one status of category debuff or DoT; `aegis_bash` shield bash staggers elites too; `sanctuary` while you block, allies within 4 m regain 2 composure per s. T3: `martyr` 3000 Bp of HP loss of allies within 10 m (after their step 7) is moved to you as `dmg.true` with no further mitigation; moved loss is never moved again, so two Wardens cannot pass it back and forth; `resurgence` revive time × 5000 (doc 16 hook); `bastion` `st.shielded` you apply lasts × 15000. T4: `last_light` (needs `martyr`) once per Folio, when an ally within 15 m would be downed it instead heals 3000 Bp of its Max HP and you lose that much HP, never below 1,000 mHP.

**Oracle.** T1: `foresight` `RowFlat(35, 1000)`; `devout` favor gains × 12500; `spirit_guide` summon Max HP × 12000; `omen_reader` counts as a Lattice Lens (Canon §17.1). T2: `prophecy` once per Folio identify the type of one unknown carried item; `divine_favor` prayer timer increments × 7500; `spectral_host` Conjuration counts +25 in the minion cap formula; `glimpse` (active) hostiles within 25 m gain `st.marked` for 300 ticks, cooldown 1800. T3: `fated` `SubFlat(PRV, 10)`; `omen_ward` one hit of at least 3000 Bp of your Max HP is × 5000, once per 1800 ticks; `consecrated` your summons convert 3000 Bp of each hit's share to `dmg.radiant`. T4: `avatar_of_fate` (needs `fated`, active, once per Folio) for 600 ticks: (1) every success roll you make on keyed `rng.ident`, `rng.check` or `rng.stealth` draws twice from the same keyed generator (exactly 2 draws per roll, as doc 10 §5.12 lists for its rolls) and keeps the result better for you; (2) on `rng.combat` and `rng.status`, where Canon §3.4 fixes one crit draw per hit, it instead grants `RowFlat(13, 2000)` inside the row's 7500 cap and makes every "X Bp chance" proc you roll use `min(10000, mul_bp(X, 15000))`, with draw counts unchanged. It is Fortune-derived, so `st.unmoored` suspends it (Canon §5.11).

**Netrunner.** T1: `backdoor` hack time × 7500; `ghost_signal` camera and turret detection of you × 7000; `overload` when your `st.hacked` ends on a unit it bursts, 2 m, `B = mul_bp(10000, P(d_cur))`, `dmg.shock`; `firewall` immune to `st.hacked`, resist `dmg.psychic` +1500. T2: `puppet_master` your `st.hacked` lasts × 15000; `daemon` (active) a construct or device within 15 m takes 60 `st.shocked` buildup, cooldown 600; `jammer` electronic hostiles within 6 m take 10 `st.blinded` buildup per status pulse; `remote_access` hack range 15 m. T3: `botnet` +1 simultaneous hacked unit; `feedback` a unit you hacked dying restores 10 aether; `data_thief` hacking a terminal grants `mul_bp(50, P(d_cur))` Lucre once per terminal. T4: `system_crash` (needs `daemon`, active) every construct or electronic hostile within 12 m gains `st.stunned` directly, cooldown 3600.

**Beastbinder.** T1: `pack_leader` your minions `DmgAdd +1000`; `feral_bond` tamed beasts within 8 m heal 100 Bp of Max HP per s; `tracker` hostile footprints within 30 m show for 300 ticks; `beast_tongue` Handling counts +15 toward doc 13 §6 taming thresholds. T2: `sic_em` (active) your minions attack the target with `DmgAdd +2500` for 300 ticks, cooldown 900; `hardy_beasts` minion Max HP × 13000; `kindred_bond` (replaces the mount talent that canon-1.8 retired) while one of your tamed beasts is within 8 m, `RowFlat(18, 10)` and your composure losses × 8500; `shared_senses` hostiles your minions see are `st.marked` for you. T3: `alpha` Handling counts +25 in the minion cap formula; `primal_fury` (active) minions gain `st.empowered` 300 ticks, cooldown 2400; `herd_mend` healing on your minions × 15000. T4: `chimera_call` (needs `alpha`, active) fuse two minions within 3 m for 1800 ticks: summed current and max HP, damage × 13000, cooldown 5400.

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

Canon §10.5 step 8 subtracts the hit's HP damage from the hit part's pool, floored at 0. Doc 03 §3.3 stores this struct as `comp.body_plan`: its `cond` and `cond_max` are `pool` and `max`, and its summary field `wound` is derived (`wound.severed`, else the tier's ID, or none at tier 0).

- **Partless damage.** DoT pulses, `st.suffocating`, starvation, toxicity overdose and every other hit with no hit part reduce HP only.
- **Fall damage** (Canon §4.2) is multiplied by the local genre's `fall_dmg_bp`, then by each doc-04 fall factor in ascending source content-ID order, one `mul_bp` each: `aug.spring_tendons` 7500, `skill.acrobatics` `10000 - 60 × skill`, `talent.monk.roll_with_it` 5000, `trait.kin.fae.light_step` 7000, `trait.origin.qinggong` 7500. The result `d` splits evenly over the plan's leg parts, each taking `floor_div(d, n)` and the lowest part ID also the remainder: `bp.leg_l` and `bp.leg_r` (humanoid, avian), the four quadruped legs, `bp.leg_3` to `bp.leg_6` (insectoid) and `bp.limb_3` and `bp.limb_4` (construct). Plans without legs put it on their largest-share part. Area hits use the largest-share part (doc 05 §1.5).
- **Set operations.** Death's Door (HP set to 1,000 mHP) and doc 16 revival leave part pools unchanged. The Colophon full heal (doc 14 §3.9) and ghost resurrection set every non-severed pool to `max`, tier 0, `treated = false`; resurrection also restores severed parts (§8.6). The Colophon heal does not restore severed parts.

### 8.2 Wound tiers

`frac = floor_div(pool × 10000, max)`. `tier_for(frac)`: `frac > 7500` → 0; `> 5000` → 1 (`wound.bruised`); `> 2500` → 2 (`wound.wounded`); otherwise `pool > 0` → 3 (`wound.crippled`), so a pool too small to register in `frac` is still tier 3; `pool = 0` → 4 (`wound.disabled`). Severing is `wound.severed`.

The wound **kind** is set when the tier rises, from the category of the hit's largest type share (ties to the earlier Canon §10.1 row): slash or pierce → `wound.laceration`; blunt and fall damage → `wound.fracture`; fire, frost, acid, shock, radiation, sonic → `wound.burn`; toxin, radiant, umbral, psychic, arcane, true → `wound.trauma`.

```rust
fn on_part_damage(e: &mut Entity, p: BpId, hit: &Hit, amount: i64) {   // amount = the hit's HP loss after step 7
    let s = e.part_mut(p);
    let crippled_before = floor_div(s.pool * 10000, s.max) <= 2500;    // wound.crippled or wound.disabled before the hit
    s.pool = (s.pool - amount).max(0);
    if s.pool == 0 && amount > 0 && severable(e, p)     // a pool already at 0 severs on the next slash-dominant hit (doc 05 §19.1)
        && (largest_share(hit) == dmg.slash || hit.flags.sever)
        && (!e.is_erratum() || crippled_before)                        // Canon §9.1, DECISIONS.md item 84
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

**Errata limbs.** An Erratum's limb severs only when its pool was at most 2500 Bp of its max before the hit, which is `wound.crippled` or `wound.disabled` (Canon §9.1, DECISIONS.md item 84). A healthy, bruised or wounded limb never severs, however large the hit: it is left at pool 0 (`wound.disabled`), and only a later slash-dominant or `sever` hit with positive part damage severs it. Example: an 85-HP Erratum's full arm (10,625 mHP) takes a 20,000-mHP slash hit and is disabled, not severed; an arm at 2,656 mHP (`frac` 2499, crippled) severs from a 3,000-mHP slash hit; an arm at 2,658 mHP (`frac` 2501, wounded) takes the same hit and is disabled.

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

Other plans map their parts onto these rows:

| Plan | Part → row above |
|---|---|
| `body.quadruped` | `bp.head` → head; `bp.torso` → torso; each leg → leg row with factors 9750, 9500, 9000 at tiers 1 to 3; tier-4 or severed legs use §13's count rule (Echo) or 7500 each (NPC) |
| `body.avian` | head and torso as humanoid; `bp.wing_l`, `bp.wing_r` → wings row; legs → leg row |
| `body.serpentine` | `bp.head` → head; `bp.body_front` → torso; `bp.body_rear` → one leg row (it is the locomotion part); `bp.tail` → tail row |
| `body.insectoid` | `bp.head` → head; `bp.thorax`, `bp.abdomen` → torso; `bp.leg_1`, `bp.leg_2` → arm rows when bound as arms (§13); `bp.leg_3` to `bp.leg_6` → `BodyMove 8500` each at tier 4 or severed, no effect below tier 4 |
| `body.amorphous` | `bp.core` → head; `bp.mass` → torso |
| `body.construct` | `bp.core` → head; `bp.chassis` → torso; `bp.limb_1`, `bp.limb_2` → arm rows; `bp.limb_3`, `bp.limb_4` → leg rows |
| `body.swarm` | `bp.mass` → no wound effects |

### 8.4 Healing, Medicine and treatment

Treatments are `cat.tool` items (doc 11 IDs) with these doc-04 profiles. Each treats the most severe untreated wound of a matching kind on one chosen part of the user or an Erratum within 2 m.

| Treatment | Kinds | Tiers | Base ticks | Medicine | Extra |
|---|---|---|---|---|---|
| bandage (`item.tool.bandage`) | laceration, trauma | 2–3 | 60 | 0 | ends `st.bleeding` (Canon §11.1) |
| splint (`item.tool.splint`) | fracture | 2–3 | 120 | 0 | — |
| salve (`item.tool.salve`) | burn | 2–3 | 90 | 0 | ends `st.burning` |
| surgical kit (`item.tool.surgical_kit`) | any | 4; reattach | 600 | 25 | tier 4 → pool `mul_bp(max, 2500)`, tier 3, treated |

```
ticks  = max(15, ceil_div(base × 10000, 10000 + 100 × medicine + 50 × d(ANL)))      // treater's values
effect: treated = true; pool += mul_bp(max, 1500 + 30 × medicine), capped at max; tier recomputed as in on_heal
```

Non-DoT damage to the treater or 500 mm of movement breaks the channel and refunds the item. Example: a bandage at Medicine 0, ANL 40 takes 60 ticks and restores `mul_bp(10625, 1500) = 1593` mHP to the 10,625-mHP arm pool of an 85-HP Erratum; at Medicine 50, ANL 50, `ceil_div(600000, 15500) = 39` ticks and 3,187 mHP. Treating an arm on yourself requires the other arm at tier ≤ 3.

Other healing: Canon §10.9 applies to HP; `on_heal` then feeds pools. `st.diseased` (owner 04) builds from body-group powers: stale food 30, rotten food 60, raw meat 40, disease-tagged enemy attacks (doc 13). It lasts 300 s and ends early with a cure potion (doc 08).

### 8.5 Body move factor

`BodyMove` is the product, one `mul_bp` each in ascending part-ID order, of every leg factor (§8.3; severed leg 5000 per Canon §9.1; a crutch (`item.tool.crutch`, doc 11) in `slot.offhand` turns one severed or tier-4 leg into 7000), clamped to `2500..=10000`. With both legs severed it is 2500 (crawl). It is Canon §4.2's `body_move_bp`, applied after row 11 and before status multipliers.

### 8.6 Severing

1. Severing runs inside `on_part_damage` (§8.2), which doc 05 §19.1 calls after step 8 of each hit. Emit `ev.part_severed`. Remove the part's hitbox. Spawn a `cat.body_part` item at the part's capsule midpoint.
2. NPC: an arm drops its held item and loses its attacks; a leg applies the leg rule; a wing removes flight; a tail removes tail attacks; a head kills single-headed creatures (Canon).
3. Erratum (a limb severs only from `wound.crippled` or `wound.disabled`, §8.2): 3 `st.bleeding` stacks applied directly; composure −10 (§11.3); held item and two-handed use per Canon; augments in that limb travel inside the part item and return on reattachment. With both arms gone: no weapons or hand items, interact actions take 3× ticks, and kicks are unarmed strikes at `B × 5000`.
4. Reattachment: surgical kit and Medicine ≥ 25 (600-tick channel), the Scriptorium service for `mul_bp(200, P(d_cur))` Lucre (doc 11 §12.3; 434 at d = 10), or a supplication that resolves the severed-part need (doc 06 §16.3). The part item must be under 7200 ticks old and its `owner` must match. Result: pool `mul_bp(max, 2500)`, tier 3, treated. Emit `ev.part_reattached`.
5. Resurrection from ghost (Canon §17.6) restores every severed part and clears all wounds.

Body-part item (Canon §13.1 `cat.body_part`, rots after 7200 ticks):

```json
{
  "uid": "0x0000070000000041", "template": "bp.arm_r", "source_template": "enemy.mythic_fantasy.cave_troll",
  "owner": "0x0000070000000013", "created_tick": 151200, "mass_g": 9100, "ilvl": 7,
  "materials": [ "mat.flesh", "mat.bone" ], "tags": [ "regenerating", "beast_sapient" ], "rotten": 0
}
```

Mass share of body mass (Bp): humanoid head 700, torso 4700, each arm 650, each leg 1650; quadruped head 900, torso 5500, each leg 900; avian head 800, torso 5000, each wing 1100, each leg 1000; serpentine head 1200, body_front 4000, body_rear 3500, tail 1300; insectoid head 1000, thorax 3500, abdomen 3502, each leg 333; construct core 1500, chassis 5500, each limb 750. An added part (§15.8) has a mass share equal to its share, taken from its donor part's mass share: a beastkin's `bp.tail` is 500 and its torso 4200. The troll arm above is `mul_bp(140000, 650) = 9100` g.

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
- **Mutation points (MP)**: `kin.mutant` gains 1 at each even level; any character gains 1 when `st.irradiated` reaches 10 stacks (once per Folio). Spending 2 MP opens an offer of 3 mutations: `keyed_rng("rng.progress", folio, actor, tick, 1).shuffle(pool)` over all mutation-form augments that fit region and total capacity, sorted by ID; the first 3 are offered and the offer is stored until chosen. Spending 1 MP raises an owned mutation's rank (max 3); rank `r` multiplies its numeric magnitudes by `10000 + 5000 × (r - 1)`. The `kin.mutant` creation offer (§2.4) is this draw at `folio = 0`, `tick = 0` with no MP spent. At most one mutation offer opens per character per tick, and a new offer cannot open while a stored one is unchosen, so two offers never share a key.
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
| `aug.extra_arm` | torso 3 | adds `bp.arm_extra_1`, or `bp.arm_extra_2` when the body already has `bp.arm_extra_1` (share 1000 from the plan's donor part, §15.8; hit_mult 8500), granting `slot.hand_extra_1` or `slot.hand_extra_2`; refused when the body has both (Canon §13.2) | natural mutation, implant, graft |
| `aug.tail` | torso 1 | adds `bp.tail` (share 500 from the plan's donor part, §15.8); `RowFlat(18, 5)`; refused when the body already has `bp.tail` | natural mutation, graft |
| `aug.wings` | torso 3 | adds `bp.wing_l`, `bp.wing_r` (750 each from the plan's donor part, §15.8); holding jump while falling glides at −2000 mm/s vertical and up to 9000 mm/s horizontal with no fall damage on landing; refused on `body.avian` | natural or supernatural mutation, graft, implant (jet glider) |
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
Clean --meter full, or add_affliction_dose (red-ink vial)--> Incubating   (incubate_until = t + 7200; st.incubating refreshed to it; ev.affliction_changed)
Incubating --t == incubate_until--> Afflicted                             (red ink: doses = 1, one Marginalia power; ev.affliction_changed)
Incubating --cure, or st.incubating removed early (doc 08 cure potion)--> Clean   (ev.affliction_changed)
Afflicted --cure--> Clean                                                 (red ink: doses = 0, every power and the composure and threshold changes removed; ev.affliction_changed)
Afflicted(red_ink, doses < 4) --meter full or add_affliction_dose--> Afflicted(doses + 1), one more power (ev.affliction_changed)
any other full meter (Incubating, Afflicted hunger or beast, red ink at 4 doses) --> no change; the meter resets
```

Each affliction keeps its own state, meter and `incubate_until`. `st.incubating` is present while any affliction is Incubating; removing it early aborts every incubating affliction. Doc 08 calls `add_affliction_dose(e, afflict.red_ink)` for a red-ink vial and `cure_affliction(e, id)` for cures. Cure: a Scriptorium service for `mul_bp(500, P(d))` Lucre or a blessed cure potion (Canon §16.4); 1085 Lucre at d = 10.

Example: a vampire bite with power 25 at d = 5 adds `mul_bp(25, S(5) = 11200) = 28`; at RES 40 (threshold 100) the fourth bite fills the meter (112).

### 10.2 Effects

- **`afflict.hunger`**: Canon effects (dark-vision +2 m, melee heals 1000 Bp of damage dealt, no natural regeneration while `light_at ≥ 12`, resist radiant −5000). Doc 04: food other than meat and blood gives × 5000 satiation; Feed is a 120-tick action on a corpse dead under 600 ticks, +600 satiation, once per corpse.
- **`afflict.beast`**: once per Folio by its ability-slot action, and involuntarily whenever composure is below 25% of max (then not again until 1800 ticks after the form ends), the bearer gains `st.polymorphed` for 1800 ticks (30 s, unscaled) as doc 06 §12.3's `summon.poly.beast`: quadruped, capsule 400 / 1200 mm, move 7500 mm/s, jump 1700 mm, one attack at 2200 mm reach every 40 ticks with `B = mul_bp(mul_bp(20000, P(d_cur)), 15000)`. That `B` already carries Canon §16.4's melee × 15000, so no pipeline step adds it again. No items, no casting. Doc 06's damage-taken break (2000 Bp of Max HP) does not end an affliction form: it ends at expiry, when the bearer is downed, or on a cure, and it is not dispellable (doc 06 §14.1). Example: at d = 10 the attack is `mul_bp(43436, 15000) = 65,154` mHP before attack factors.
- **`afflict.red_ink`**: each dose lowers the hunter threshold by 1 (Canon §11.6), applies `RowFlat(9, -5)`, and unlocks one Marginalia power of the bearer's choice among those not owned.

### 10.3 Marginalia powers

`trait.marginalia.*`, dependency none (ink sits beneath genre law), `src_lvl = d_cur`:

1. `strikethrough` (active): a hostile within 20 m takes `B = mul_bp(30000, P(d_cur))` `dmg.arcane` and `st.unraveled` directly; cooldown 1800.
2. `redaction` (active): `st.invisible` 300 ticks; cooldown 3600.
3. `rewrite_wound` (active): heal 2500 Bp of Max HP and set every wound of tier ≤ 3 to treated; gain 1 `st.erasure` stack; cooldown 3600.
4. `ink_sight`: `fac.editors` members and `st.invisible` entities within 30 m are visible to you.
5. `footnote`: `DmgAdd +2000` against `fac.editors` members.
6. `erase` (active): every block tagged `block.tag.destructible` and every placeable in a 3 × 3 × 3 volume centred on the aimed block within 6 m is removed with no drops; fixed terrain is untouched (Canon §4.3); cooldown 600.
7. `margin_step` (active): teleport up to 12 m along aim to the nearest valid standing position; cooldown 900.
8. `dictation` (active): `st.charmed` directly on a non-boss hostile; gain 1 `st.erasure` stack; cooldown 3600.

---
## 11. Needs

All needs run on the Needs subsystem (period 60, phase 7, `k = 1`) unless stated. Satiation (hunger) is the only survival meter: there is no thirst meter and no fatigue meter (DECISIONS.md item 145). Composure, breath and temperature comfort are the other needs below.

### 11.1 Satiation

- Range `0..=2400`, start 1800, `st.hungry` at ≤ 600, `st.starving` at 0 (Canon §12, §11.1). Decay pauses in boss Folios while the boss lives (Canon §11.6).
- Decay uses a thousandths accumulator: `R = 1000` multiplied by each personal factor (`scavenger` 7500, `grave_appetite` 5000, `fast_metabolism` 12500) in ascending source-ID order; each Needs tick `q = floor_div(R + acc, 1000)`, `acc = R + acc - 1000 q`, satiation −= q.
- While `st.starving`, every 300th tick of continuous starvation deals `mul_bp(max_hp_mhp, 100)` as `dmg.true` entering Canon §10.5 step 4 (850 mHP at 85 HP); natural regeneration is 0.
- Eating is allowed below 2400; the result is `min(2400, current + value)`.

### 11.2 Food values

`value = base × freshness`, then × `10000 + 20 × skill.survival`, then talent and trait factors, one `mul_bp` each.

| Food | Base | Rot ticks | Eat ticks | Notes |
|---|---|---|---|---|
| ration (`item.food.ration`) | 600 | never | 60 | |
| meal (`item.food.meal`; also any cooked dish) | 900 | 36000 | 60 | composure +10 |
| snack (`item.food.bread`), bread, fruit, produce | 300 | 18000 | 60 | |
| raw meat | 300 | 7200 | 60 | +40 `st.diseased` power |
| raw body part | `min(900, floor_div(mass_g, 10))` | 7200 (rotten at once) | 120 | raw-meat rule; sapient: composure −15 |
| corpse (Eat action) | 1200 | 7200 from death | 600 | +80 `st.diseased` power |
| blood (`afflict.hunger` Feed) | 600 | 600 from death | 120 | |
| fuel for `kin.construct`: coal, oil flask, power cell | 600, 900, 1200 | never | 60 | constructs refuse all other food |

Freshness: age `< rot` fresh × 10000; `rot ≤ age < 2 × rot` stale × 5000 and +30 diseased power; `age ≥ 2 × rot` rotten × 2500 and +60 power. Example: a ration at Survival 50 gives `mul_bp(600, 11000) = 660`.

### 11.3 Composure

Losses are multiplied by the Canon §5.7 MYTHOS term, then by personal factors (product of the bearer's factors in ascending source-ID order, clamped `2500..=20000`). Personal factors come from traits, talents, augments and gear, for example doc 11's `perk.leg.cold_blood` (× 7000) and `item.exotic.veil_of_vespers` (× 5000). Example: `perk.leg.cold_blood` with `trait.origin.already_broken` gives `mul_bp(mul_bp(10000, 7000), 8000) = 5600`.

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
| a loss another chapter states (doc 06 Composure Toll register, doc 08 effects, doc 13 attacks) | as stated, in milli; it takes this section's multipliers |
| comfort band, cold or hot side (§11.4) | 100 to 300 milli per Needs tick |

| Gain event | Composure |
|---|---|
| not `in_combat` and `light_at ≥ 8` | 1 per s (Canon) |
| within 3 m of a block tagged `hearth` with `light_block ≥ 12`, not `in_combat` | +2 per s |
| cooked meal; kill of rank elite or higher or of an eldritch-tagged hostile | 10; 5 |
| level-up; Folio exit | 20; 25 |

Example: a 5-point loss in Cosmic Horror (MYT 10) is `mul_bp(5000, 15000) = 7500` milli; with `already_broken` it becomes `mul_bp(7500, 8000) = 6000`.

### 11.4 Temperature

Two layers act on a body.

- **Hazard layer (doc 03 §9.4).** On each temperature run (period 30) a body at `Env.temp > 600` takes environmental `dmg.fire`, at `>= 2500` also `st.burning` buildup, at `<= -100` gains `st.chilled`, and at `<= -300` takes environmental `dmg.frost` and 20 `st.frozen` buildup. Resistances apply. This chapter never applies those effects a second time.
- **Comfort layer (this chapter).** Evaluated on each Needs tick from `T = Env.temp`, the thermal cell of the body point (doc 03 §5.20). The comfort band in DeciC is `lo = 50 - 10 × floor_div(max(0, d(END)), 5) - insul_cold - bonus` and `hi = 300 + 10 × floor_div(max(0, d(END)), 5) + insul_heat + bonus`, where `insul_*` are armor fields (doc 05) and `bonus` sums `hardy` (100), Survival (`10 × floor_div(skill, 10)`) and `aug.thermal_regulator` (150). `st.soaked` raises `lo` by 100. `dev` is the distance outside the band, 0 inside.

| dev | Cold side | Hot side |
|---|---|---|
| 0 | none | none |
| 1..=149 | `RowPost(5, 9000)` | `RowPost(5, 9000)` |
| 150..=299 | `RowPost(5, 8000)`; composure −100 milli per Needs tick | `RowPost(5, 7500)`; composure −200 milli per Needs tick |
| >= 300 | `RowPost(5, 7000)`, `RowPost(2, 5000)`; composure −200 milli per Needs tick | `RowPost(5, 6000)`, `RowPost(2, 5000)`; composure −300 milli per Needs tick |

Example: END 40 at −400 DeciC with no insulation: `lo = 50`, `dev = 450`, so stamina regeneration × 7000, HP regeneration × 5000 and −200 milli composure per second; doc 03 separately applies `st.chilled`, frost damage and 20 `st.frozen` buildup per temperature run. END 60 gives the band `10..=340`.

### 11.5 Stamina, breath and suffocation

- Stamina costs: sprint row 6; dodge `max(10, 20 - reductions)` (Canon §4.2's 20 minus Acrobatics 25's 3 and `fleet_foot`'s 3); swim 5 per s; climb 4 per s; jump 0; melee and block costs per doc 05. Regeneration (row 5) starts after 60 ticks without spending. Reaching 0 applies `st.exhausted` for 120 ticks.
- **Movement predicates.** `can_sprint(e)`, `can_jump(e)` and `can_dodge(e)` (doc 03's `MoverParams`, doc 16's client predictor) are true unless something removes the action: `st.overburdened` removes all three and `st.exhausted` removes sprint and dodge (Canon §11.1); stamina 0 removes sprint and dodge; a §8.3 wound effect or a §13 plan rule removes what it names (on a humanoid, a leg at tier 3 removes sprint, a leg at tier 4 removes sprint, jump and dodge, and a torso at tier 4 removes sprint); and a severed leg of a two-legged plan without a prosthetic (§9.5) removes all three (Canon §9.1). Jump costs no stamina, so stamina never removes it. The sprint-slide (doc 03 §5.23) costs no stamina and is allowed only while `can_sprint(e)` is true. A grapple pull (doc 03 §5.24) resets `peak_y` every tick, so no fall damage accrues during it.
- `res.breath` max = `(10 + floor_div(END, 10) + Σ RowFlat breath) × 1000`. It updates on each Needs tick (doc 03 §2.2 `sys.needs`, `k = 1`): −1000 while doc 03's `eye_submerged` is true, the eye cell holds a gas flagged unbreathable (doc 08), or the player holds `input.hold_breath` (doc 09 §4.6), else +5000, clamped to `0..=max`. Exempt from draining: `machine_body`, `deathless`, `aug.gills` while submerged in `liq.water` (its implant skin also makes gases breathable), a doc 08 respirator with `filter > 0` for gases, and a doc 08 rebreather with `filter > 0` while submerged. When breath is 0 after the update, `st.suffocating` applies (DoT 500 Bp `dmg.true`, Canon); the first Needs tick that ends with breath above 0 removes it. Example: END 40 gives 14,000 and reaches 0 on the 14th submerged Needs tick; suffocation at 85 HP costs `mul_bp(85000, 500) = 4250` mHP per s; the first Needs tick in air raises breath to 5,000 and removes it.

### 11.6 Encumbrance

`carried_g` = inventory items + equipped non-armor items + worn armor counted by the Armor rule (§5.3) + a carried corpse or Remains. `capacity_g = (row 10 + RowFlat(10)) × 1000`. `load_bp` and the states are Canon §11.1. Example: capacity 45 kg with 50,000 g carried gives `load_bp = 11111`, `st.encumbered`; picking up 75,000 g Remains makes it `st.overburdened`. Armor of 12,000 g at Armor 50 counts 9,600 g.

---

## 12. Death, Death's Door, madness and player-controlled statuses

### 12.1 Death's Door

Canon §10.8 step 1. `dd_used` resets on `ev.folio_entered`. The roll is `keyed_rng("rng.combat", folio, erratum_id, tick, 0xFFFF).roll_bp(row 43)`; seq `0xFFFF` is reserved for it. Success: HP = 1,000 mHP, `st.deaths_door` 60 ticks (invulnerable), part pools unchanged. `dd_used = true` either way. A downed Erratum never rolls again until the next Folio. While the Erratum has `st.unmoored`, Death's Door is off (Canon §5.11): no draw is made and `dd_used` is unchanged.

### 12.1a Last stand (DECISIONS.md items 131, 134)

In solo play (a party with one Erratum slot), when lethal damage reaches an Erratum and Death's Door does not prevent it (the roll failed, Death's Door was already used this Folio, or `st.unmoored` made no draw), the Erratum enters a **last stand** instead of Canon §10.8 step 3, provided `ls_used` is false. `ls_used` resets on `ev.folio_entered` and is set when the last stand opens, so it opens at most once per Folio. The last stand lasts `LAST_STAND_TICKS = 300` ticks: HP stays 0, the Erratum takes no damage and gains no buildup, every command except the last-stand inputs is rejected, and the world keeps running. Doc 05 owns the input (three prompted ink-strike inputs in order, DECISIONS.md item 135) and doc 17 its prompt and accessibility forms. On success, HP = 1,000 mHP, part pools are unchanged and play resumes on the next tick. On failure, or when the 300 ticks run out, Canon §10.8 step 3 applies (the Erasure scene plays); when the killer is an invader, §12.3a applies instead. In a party of two or more no last stand opens. The last stand draws no random number.

### 12.2 Downed hooks (doc 16 owns `st.downed`)

- Doc 16 §9.4 owns revive progress and the revived HP (`revive_hp_bp`). This chapter exports `revive_bp(reviver)`, the product (one `mul_bp` each, ascending source-ID order, from 10000) of the reviver's doc-04 revive factors. The only factor at launch is `talent.warden.resurgence` (5000), so doc 16's `revive_ticks = max(30, mul_bp(180, revive_bp))` is 180, or 90 with `resurgence`.
- Restoration 100 completes a revival at once (§5.3).
- On `ev.player_downed`: allies within 30 m lose 10 composure. Wounds, afflictions and needs persist; natural regeneration is 0 while downed.

### 12.3 Death and resurrection

`ev.player_died` → Remains (Canon §17.6). While the Erratum is a ghost its needs are frozen: `sys.needs` skips it, so satiation, toxicity, composure and breath keep their values and their accumulators do not advance. A resurrected ghost returns with every severed part restored, every part pool at max (tier 0), composure at max, and satiation, toxicity, afflictions and augments unchanged; its HP is doc 16 §9.5's resurrection value.

### 12.3a Redacted (Canon §17.9)

When an invader reduces the last Erratum that is alive and not downed to 0 HP (Canon §17.9), Death's Door is rolled first as usual (§12.1); if it fails, the party is Redacted instead of the run ending. Every Erratum, ghosts included, respawns at the Folio entrance with HP set to `mul_bp(max_hp_mhp, 5000)`. Doc-04 state on respawn: a ghost is resurrected (§12.3 part rules); a living Erratum keeps severed parts severed and sets every other part pool to `max(pool, mul_bp(max, 5000))`, recomputing tiers as `on_heal` does; satiation, toxicity, composure, afflictions, augments, `dd_used` and every XP counter are unchanged. Killing the invader pays rank-hunter kill XP under doc 13 §11.1.

### 12.4 Madness

`st.madness` applies when composure reaches 0 and ends when it reaches 25% of max (Canon). Effects: aim sway × 20000 (Canon, doc 05), checks −10, and **episodes**: on each active tick where `(t - a) % 600 == 0`, draw `keyed_rng("rng.check", folio, bearer, t, 0xFFFE).weighted([2500, 2500, 1500, 1000, 2500])`:

| Index | Episode | Effect |
|---|---|---|
| 0 | Fugue | `st.confused` directly |
| 1 | Panic | `st.feared` directly, source the nearest hostile within 30 m (no effect if none) |
| 2 | Rage | `st.berserk` directly |
| 3 | Lucid insight | every hostile within 30 m gains `st.marked` for 300 ticks, visible to the bearer only |
| 4 | Whispers | audio only |

Phantoms are presentation-only, spawned by the bearer's client from Canon §3.8's `pres_hash("madness.phantom", run_seed ^ entity_id, floor_div(tick, 600))`. They never enter sim state, never deal damage and never draw from a sim stream. Unmarked, a phantom looks and sounds exactly like the native template it imitates (doc 18), so madness stays deceptive; the client knows which entities are phantoms, so doc 17's `access.mark_hallucinations`, forced on under Scribe's Mercy, can mark them (DECISIONS.md item 49).

### 12.5 Player-controlled statuses (Canon §11.8)

The input layer validates commands; it never synthesizes input. The same table governs an invader (Canon §17.9, doc 13), with "allies" meaning `fac.editors` members.

| Status | Implementation on a player-controlled Erratum |
|---|---|
| `st.charmed` | after step 2.15, `D2 = 0` against the applier's faction; that hit's buildup and on-hit effects are × 0 and `ev.damage_dealt` reports total 0; hits on allies skip step 2.15 (× 10000); HUD tints allies hostile; the bearer's minions do not change sides |
| `st.berserk` | block, cast, reload and item-use commands rejected; melee stamina cost 0; × 13000 at step 2.12 |
| `st.feared` | fire and cast rejected while `ang_diff(yaw, bearing to source) ≤ 8192`; its move multiplier is 11000 while the movement direction is within 8192 Bam of the bearing away from the source, else 10000 |
| `st.sleeping` | all commands rejected; ends on damage or an ally's 30-tick interact; duration × 5000 |
| `st.confused` | Canon behavior: movement input rotated by the doc-06 angle |
| `st.taunted` | HUD marks the taunter; damage to others × 7500 |
| `st.hacked` (only a `body.construct` Echo) | as `st.charmed` toward the hacker's faction |
| `st.polymorphed` | `input.attack` performs the template's attack (every doc 06 §12.3 template has at most one); every other action input is rejected; equipment stowed; HP ratio kept |
| `st.stunned`, `st.frozen`, `st.staggered`, `st.knocked_down` | all action commands rejected; camera free |
| `st.grappled` | movement rejected; one-handed actions allowed |

---

## 13. Echo bodies

For these plans the capsule and base speeds below replace Canon §4.1's humanoid body and §4.2's base speeds; row 11, `BodyMove` and statuses then apply. Jump follows Canon §4.2.

| Plan | Capsule radius / height / eye (mm) | Walk / sprint / crouch / swim / climb (mm/s) | Hands and slots | Rules |
|---|---|---|---|---|
| `body.quadruped` | 400 / 1300 / 1100 | 4800 / 8000 / 2200 / 2000 / 0 | `slot.weapon_1` only (mouth: one-handed melee or a `harness` ranged weapon); no offhand, hands or ring slots; `slot.implant_arms` = forelegs | one leg lost or tier 4: `BodyMove 7500`; two 5000; three 2500; no ladders; Climb −20 |
| `body.avian` | 300 / 1600 / 1450 | 3800 / 5500 / 1800 / 1200 / 1600 | `slot.weapon_1` held in the feet (one-handed); no offhand or hands slot | glides as `aug.wings`; a lost or tier-4 wing stops gliding |
| `body.serpentine` | 350 / 1100 / 1000 | 4300 / 6000 / 2600 / 4800 / 1800 | adds `bp.arm_extra_1` and `bp.arm_extra_2` (750 each from `bp.body_front`), giving `slot.hand_extra_1` and `_2`; two-handed weapons use both; no legs or feet slots | leg rules never apply; footstep noise −6 dB |
| `body.amorphous` | 450 / 1000 / 800 | 3500 / 5000 / 2500 / 3500 / 2400 | one pseudopod: `slot.hand_extra_1`; only `slot.body` armor | crouched height 600 mm; climbs any solid face (`BodyState` flag 8 `CLIMB_ANY`, doc 03 §3.4, §5.11); immune to `st.bleeding`; nothing severs; only `bp.core` is a precision part |
| `body.construct` | humanoid | humanoid | `bp.limb_1`, `bp.limb_2` bind as `bp.arm_r`, `bp.arm_l`; `bp.limb_3`, `bp.limb_4` act as legs; all humanoid slots | immune to `st.poisoned`, `st.diseased`, `st.bleeding`; no breath; `st.hacked` applies (§12.5) |
| `body.insectoid` | 350 / 1200 / 1050 | 4600 / 7000 / 2200 / 1500 / 3000 | `bp.leg_1`, `bp.leg_2` bind as arms; all humanoid hand slots | climbs any solid face (`CLIMB_ANY`); each lost walking leg (3–6) `BodyMove 8500` |

Kin traits that need a missing part are skipped, and a kin never adds a part the plan already has: `kin.beastkin` adds `bp.tail` (share 500 from `bp.torso`) to `body.quadruped` but not to `body.serpentine`. Two launch Echoes use these plans (Canon §16.2, doc 14 §6.3): `echo.fenrir` is `body.quadruped` and `echo.jormungandr` is `body.serpentine`. Their natural attacks are doc 05's `arch.melee.unarmed` skinned per plan (quadruped: bite, claw and pounce; serpentine: bite, constrict and spit), so the Unarmed skill and its perks, `claws` and the Monk talents apply to them as to bare hands; on a quadruped the Monk's hand wraps go in `slot.weapon_1` (the mouth). A kit item for a slot the plan lacks goes to the bag (doc 14 §2.4.2). The serpentine plan's two extra arms use both extra-arm parts, so `aug.extra_arm` is refused for it. A non-humanoid Echo's wound effects follow the §8.3 mapping table. Doc 04 sets `BodyState` flag 8 `CLIMB_ANY` on every Erratum whose plan is `body.amorphous` or `body.insectoid`; doc 03 §5.11 then treats every collision box overlapping the capsule AABB expanded by 100 mm as climbable (between `p.y + STEP_UP` and `p.y + h` to enter a climb, `p.y` to `p.y + h` to stay in it).

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

Model as §6.3 on doc 13 §11.2's plan with a 6500 kill share on standard Folios; skill XP per Folio hacking 50, sneak 30, tinkering 25, security 15, awareness 10, marksmanship 10, illusion 6; Unseen in Folios 1, 3, 6 and 8; doc-04 rows 20 tenths per Folio. Folio 1 in full: `K = 5`, kill `mul_bp(73, 6500) = 47`, 8 counted skill-ups × 10 = 80, Exit 20, Unseen 100, doc-04 rows 10, total 257.

| Folio | Gain | Cumulative | Level after | Allocation at each new level | Talent | Max HP |
|---|---|---|---|---|---|---|
| 1 | 257 | 257 | 2 | L2: LOG +2, TGH +2 | L2 `backdoor` | 92 |
| 2 | 466 | 723 | 3 | L3: LOG +2, AGI +2 | — | 97 |
| 3 | 1432 | 2155 | 4 | L4: TGH +2, HRG +2 | L4 `firewall` | 104 |
| 4 | 1451 | 3606 | 5 | L5: LOG +2, SLT +2 | — | 109 |
| 5 | 4615 | 8221 | 7 | L6: AGI +2, TGH +2; L7: LOG +2, END +2 | L6 `remote_access` | 116, 121 |
| 6 | 5518 | 13739 | 8 | L8: RES +2, TGH +2 | L8 `puppet_master` | 128 |
| 7 | 4302 | 18041 | 9 | L9: LOG +2, HRG +2 | — | 133 |
| 8 | 8711 | 26752 | 10 | L10: AGI +2, ITU +2 | L10 `daemon`; Interpolation offer | 138 |

Training marks over these Folios raise LOG by 7, AGI by 4, SLT, PRC and HRG by 2 each, and GUI by 1. At level 10: LOG 67, AGI 54, SLT 51, TGH 48, HRG 49; hacking 50 (the Folio-8 skill-up grants `skill.hacking.m50`), sneak 36, tinkering 33. Max HP at level 10 is `40 + 48 + 10 × (3 + 2) = 138`. INT is `floor_div(67 + 40 + 40, 3) = 49`, which meets the tier-3 gate of 45; tier 3 also needs level 12, so the first tier-3 talent is bought at level 12.

---

## 15. ID register

Every ID this document defines, one row each, with its exact structural data. Effects are not repeated: each row names the section that states them, and the ID's last segment is the name used there. The augment list (§9.4) and the Origin table (§2.3) are already registers and are not repeated. Consumers cite these IDs exactly; no other spelling is valid.

### 15.1 Primary attributes and sub-attributes

| ID | Code | Kind | Sub-attributes, or parent primary | Governed skills (Canon §7) | Canon §6.4 rows | Key primary of (§7.1) |
|---|---|---|---|---|---|---|
| `attr.might` | MGT | primary | `sub.power`, `sub.brawn`, `sub.grip` | hafted, unarmed, shield, smithing, polearm, heavy_arms | through its sub-attributes | vanguard, grenadier |
| `attr.finesse` | FIN | primary | `sub.precision`, `sub.agility`, `sub.sleight` | blade, marksmanship, acrobatics, sneak, throwing, security | through its sub-attributes | duelist, gunslinger, shade, monk |
| `attr.vigor` | VIG | primary | `sub.toughness`, `sub.endurance`, `sub.resilience` | armor, athletics, fieldcraft, restoration | through its sub-attributes | warden |
| `attr.intellect` | INT | primary | `sub.logic`, `sub.memory`, `sub.analysis` | spellcraft, tinkering, hacking, conjuration, inscription, alchemy, medicine | through its sub-attributes | alchemist, artificer, netrunner |
| `attr.will` | WIL | primary | `sub.focus`, `sub.resolve`, `sub.attunement` | alteration, abjuration, evocation | through its sub-attributes | spellwright, oracle |
| `attr.perception` | PER | primary | `sub.sight`, `sub.hearing`, `sub.intuition` | survival, awareness, divination, appraisal | through its sub-attributes | none |
| `attr.presence` | PRE | primary | `sub.charm`, `sub.menace`, `sub.guile` | speech, handling, leadership, illusion, pickpocket | through its sub-attributes | beastbinder |
| `attr.fortune` | FOR | primary | `sub.fate`, `sub.serendipity`, `sub.providence` | devotion | through its sub-attributes | none |
| `sub.power` | PWR | sub-attribute | MGT | hafted, unarmed | 10, 20 | — |
| `sub.brawn` | BRN | sub-attribute | MGT | shield, smithing | 18, 19 | — |
| `sub.grip` | GRP | sub-attribute | MGT | polearm, heavy_arms | 16, 21 | — |
| `sub.precision` | PRC | sub-attribute | FIN | blade, marksmanship | 13, 15 | — |
| `sub.agility` | AGI | sub-attribute | FIN | acrobatics, sneak | 11 | — |
| `sub.sleight` | SLT | sub-attribute | FIN | throwing, security | 17 | — |
| `sub.toughness` | TGH | sub-attribute | VIG | armor | 1, 23 | — |
| `sub.endurance` | END | sub-attribute | VIG | athletics, fieldcraft | 4, 5, 6 | — |
| `sub.resilience` | RES | sub-attribute | VIG | restoration | 2, 3, 22 | — |
| `sub.logic` | LOG | sub-attribute | INT | spellcraft, tinkering, hacking | 26 | — |
| `sub.memory` | MEM | sub-attribute | INT | conjuration, inscription | 27, 28 | — |
| `sub.analysis` | ANL | sub-attribute | INT | alchemy, medicine | none | — |
| `sub.focus` | FOC | sub-attribute | WIL | alteration | 7, 30 | — |
| `sub.resolve` | RSV | sub-attribute | WIL | abjuration | 9, 24 | — |
| `sub.attunement` | ATN | sub-attribute | WIL | evocation | 8, 25, 29 | — |
| `sub.sight` | SGT | sub-attribute | PER | survival | 31, 32 | — |
| `sub.hearing` | HRG | sub-attribute | PER | awareness | 33, 34 | — |
| `sub.intuition` | ITU | sub-attribute | PER | divination, appraisal | 35, 36 | — |
| `sub.charm` | CHM | sub-attribute | PRE | speech, handling | 37, 38 | — |
| `sub.menace` | MEN | sub-attribute | PRE | leadership | 39 | — |
| `sub.guile` | GUI | sub-attribute | PRE | illusion, pickpocket | 40 | — |
| `sub.fate` | FAT | sub-attribute | FOR | none | 13, 41 | — |
| `sub.serendipity` | SER | sub-attribute | FOR | none | 42 | — |
| `sub.providence` | PRV | sub-attribute | FOR | devotion | 43, 44 | — |

Row 12 also reads any sub-attribute a weapon's scaling coefficients name (§3). Uses, checks and tooltips: §3 and §4.

### 15.2 Skills and mastery perks

Skills, in Canon §7 order. XP events are §5.2; skill-level effects are §5.3.

| ID | Name | Governing | Group |
|---|---|---|---|
| `skill.blade` | Blade | PRC | combat |
| `skill.hafted` | Hafted | PWR | combat |
| `skill.polearm` | Polearm | GRP | combat |
| `skill.unarmed` | Unarmed | PWR | combat |
| `skill.shield` | Shield | BRN | combat |
| `skill.marksmanship` | Marksmanship | PRC | combat |
| `skill.heavy_arms` | Heavy Arms | GRP | combat |
| `skill.throwing` | Throwing | SLT | combat |
| `skill.armor` | Armor | TGH | defense |
| `skill.acrobatics` | Acrobatics | AGI | mobility |
| `skill.athletics` | Athletics | END | mobility |
| `skill.evocation` | Evocation | ATN | magic |
| `skill.abjuration` | Abjuration | RSV | magic |
| `skill.alteration` | Alteration | FOC | magic |
| `skill.conjuration` | Conjuration | MEM | magic |
| `skill.illusion` | Illusion | GUI | magic |
| `skill.restoration` | Restoration | RES | magic |
| `skill.divination` | Divination | ITU | magic |
| `skill.spellcraft` | Spellcraft | LOG | magic |
| `skill.devotion` | Devotion | PRV | faith |
| `skill.sneak` | Sneak | AGI | stealth |
| `skill.security` | Security | SLT | stealth |
| `skill.pickpocket` | Pickpocket | GUI | stealth |
| `skill.smithing` | Smithing | BRN | craft |
| `skill.tinkering` | Tinkering | LOG | craft |
| `skill.alchemy` | Alchemy | ANL | craft |
| `skill.inscription` | Inscription | MEM | craft |
| `skill.appraisal` | Appraisal | ITU | knowledge |
| `skill.medicine` | Medicine | ANL | knowledge |
| `skill.survival` | Survival | SGT | knowledge |
| `skill.speech` | Speech | CHM | social |
| `skill.leadership` | Leadership | MEN | social |
| `skill.hacking` | Hacking | LOG | stealth |
| `skill.fieldcraft` | Fieldcraft | END | craft |
| `skill.awareness` | Awareness | HRG | stealth |
| `skill.handling` | Handling | CHM | social |

Mastery perks: four per skill, 144 in all, granted on reaching 25 (Apprentice), 50 (Journeyman), 75 (Expert) and 100 (Master) and never lost (§5.1). Each perk's effect is the text after its tier number in that skill's §5.3 entry; the ten magic and faith skills' perks are also doc 06 §7.3's hook rows.

| Perk IDs (25, 50, 75, 100) | Skill | Effects |
|---|---|---|
| `skill.blade.m25`, `skill.blade.m50`, `skill.blade.m75`, `skill.blade.m100` | `skill.blade` | §5.3 Blade |
| `skill.hafted.m25`, `skill.hafted.m50`, `skill.hafted.m75`, `skill.hafted.m100` | `skill.hafted` | §5.3 Hafted |
| `skill.polearm.m25`, `skill.polearm.m50`, `skill.polearm.m75`, `skill.polearm.m100` | `skill.polearm` | §5.3 Polearm |
| `skill.unarmed.m25`, `skill.unarmed.m50`, `skill.unarmed.m75`, `skill.unarmed.m100` | `skill.unarmed` | §5.3 Unarmed |
| `skill.shield.m25`, `skill.shield.m50`, `skill.shield.m75`, `skill.shield.m100` | `skill.shield` | §5.3 Shield |
| `skill.marksmanship.m25`, `skill.marksmanship.m50`, `skill.marksmanship.m75`, `skill.marksmanship.m100` | `skill.marksmanship` | §5.3 Marksmanship |
| `skill.heavy_arms.m25`, `skill.heavy_arms.m50`, `skill.heavy_arms.m75`, `skill.heavy_arms.m100` | `skill.heavy_arms` | §5.3 Heavy Arms |
| `skill.throwing.m25`, `skill.throwing.m50`, `skill.throwing.m75`, `skill.throwing.m100` | `skill.throwing` | §5.3 Throwing |
| `skill.armor.m25`, `skill.armor.m50`, `skill.armor.m75`, `skill.armor.m100` | `skill.armor` | §5.3 Armor |
| `skill.acrobatics.m25`, `skill.acrobatics.m50`, `skill.acrobatics.m75`, `skill.acrobatics.m100` | `skill.acrobatics` | §5.3 Acrobatics |
| `skill.athletics.m25`, `skill.athletics.m50`, `skill.athletics.m75`, `skill.athletics.m100` | `skill.athletics` | §5.3 Athletics |
| `skill.evocation.m25`, `skill.evocation.m50`, `skill.evocation.m75`, `skill.evocation.m100` | `skill.evocation` | §5.3 Evocation |
| `skill.abjuration.m25`, `skill.abjuration.m50`, `skill.abjuration.m75`, `skill.abjuration.m100` | `skill.abjuration` | §5.3 Abjuration |
| `skill.alteration.m25`, `skill.alteration.m50`, `skill.alteration.m75`, `skill.alteration.m100` | `skill.alteration` | §5.3 Alteration |
| `skill.conjuration.m25`, `skill.conjuration.m50`, `skill.conjuration.m75`, `skill.conjuration.m100` | `skill.conjuration` | §5.3 Conjuration |
| `skill.illusion.m25`, `skill.illusion.m50`, `skill.illusion.m75`, `skill.illusion.m100` | `skill.illusion` | §5.3 Illusion |
| `skill.restoration.m25`, `skill.restoration.m50`, `skill.restoration.m75`, `skill.restoration.m100` | `skill.restoration` | §5.3 Restoration |
| `skill.divination.m25`, `skill.divination.m50`, `skill.divination.m75`, `skill.divination.m100` | `skill.divination` | §5.3 Divination |
| `skill.spellcraft.m25`, `skill.spellcraft.m50`, `skill.spellcraft.m75`, `skill.spellcraft.m100` | `skill.spellcraft` | §5.3 Spellcraft |
| `skill.devotion.m25`, `skill.devotion.m50`, `skill.devotion.m75`, `skill.devotion.m100` | `skill.devotion` | §5.3 Devotion |
| `skill.sneak.m25`, `skill.sneak.m50`, `skill.sneak.m75`, `skill.sneak.m100` | `skill.sneak` | §5.3 Sneak |
| `skill.security.m25`, `skill.security.m50`, `skill.security.m75`, `skill.security.m100` | `skill.security` | §5.3 Security |
| `skill.pickpocket.m25`, `skill.pickpocket.m50`, `skill.pickpocket.m75`, `skill.pickpocket.m100` | `skill.pickpocket` | §5.3 Pickpocket |
| `skill.smithing.m25`, `skill.smithing.m50`, `skill.smithing.m75`, `skill.smithing.m100` | `skill.smithing` | §5.3 Smithing |
| `skill.tinkering.m25`, `skill.tinkering.m50`, `skill.tinkering.m75`, `skill.tinkering.m100` | `skill.tinkering` | §5.3 Tinkering |
| `skill.alchemy.m25`, `skill.alchemy.m50`, `skill.alchemy.m75`, `skill.alchemy.m100` | `skill.alchemy` | §5.3 Alchemy |
| `skill.inscription.m25`, `skill.inscription.m50`, `skill.inscription.m75`, `skill.inscription.m100` | `skill.inscription` | §5.3 Inscription |
| `skill.appraisal.m25`, `skill.appraisal.m50`, `skill.appraisal.m75`, `skill.appraisal.m100` | `skill.appraisal` | §5.3 Appraisal |
| `skill.medicine.m25`, `skill.medicine.m50`, `skill.medicine.m75`, `skill.medicine.m100` | `skill.medicine` | §5.3 Medicine |
| `skill.survival.m25`, `skill.survival.m50`, `skill.survival.m75`, `skill.survival.m100` | `skill.survival` | §5.3 Survival |
| `skill.speech.m25`, `skill.speech.m50`, `skill.speech.m75`, `skill.speech.m100` | `skill.speech` | §5.3 Speech |
| `skill.leadership.m25`, `skill.leadership.m50`, `skill.leadership.m75`, `skill.leadership.m100` | `skill.leadership` | §5.3 Leadership |
| `skill.hacking.m25`, `skill.hacking.m50`, `skill.hacking.m75`, `skill.hacking.m100` | `skill.hacking` | §5.3 Hacking |
| `skill.fieldcraft.m25`, `skill.fieldcraft.m50`, `skill.fieldcraft.m75`, `skill.fieldcraft.m100` | `skill.fieldcraft` | §5.3 Fieldcraft |
| `skill.awareness.m25`, `skill.awareness.m50`, `skill.awareness.m75`, `skill.awareness.m100` | `skill.awareness` | §5.3 Awareness |
| `skill.handling.m25`, `skill.handling.m50`, `skill.handling.m75`, `skill.handling.m100` | `skill.handling` | §5.3 Handling |

### 15.3 Callings

| ID | Key primary | Majors, 20 (Canon §16.2) | Minors, 10 (Canon §16.2) | Quick start preset (§2.5) | Talent tree |
|---|---|---|---|---|---|
| `calling.vanguard` | MGT | hafted, armor, shield | athletics, leadership, smithing | PWR +3, TGH +3, BRN +2, RSV +2 | `talent.vanguard.*` (§7.2, §15.4) |
| `calling.duelist` | FIN | blade, acrobatics, speech | sneak, throwing, unarmed | PRC +3, AGI +3, CHM +2, FAT +2 | `talent.duelist.*` (§7.2, §15.4) |
| `calling.gunslinger` | FIN | marksmanship, acrobatics, throwing | tinkering, speech, security | PRC +4, AGI +2, SLT +2, FAT +2 | `talent.gunslinger.*` (§7.2, §15.4) |
| `calling.grenadier` | MGT | heavy_arms, throwing, armor | tinkering, fieldcraft, medicine | GRP +3, TGH +3, SLT +2, PWR +2 | `talent.grenadier.*` (§7.2, §15.4) |
| `calling.spellwright` | WIL | evocation, spellcraft, alteration | abjuration, inscription, divination | ATN +3, LOG +3, FOC +2, MEM +2 | `talent.spellwright.*` (§7.2, §15.4) |
| `calling.alchemist` | INT | alchemy, throwing, medicine | survival, restoration, appraisal | ANL +4, SLT +2, RES +2, LOG +2 | `talent.alchemist.*` (§7.2, §15.4) |
| `calling.shade` | FIN | sneak, blade, security | pickpocket, acrobatics, illusion | AGI +4, PRC +2, SLT +2, GUI +2 | `talent.shade.*` (§7.2, §15.4) |
| `calling.artificer` | INT | tinkering, smithing, marksmanship | security, inscription, appraisal | LOG +4, BRN +2, PRC +2, ANL +2 | `talent.artificer.*` (§7.2, §15.4) |
| `calling.warden` | VIG | shield, abjuration, restoration | polearm, devotion, leadership | BRN +3, RES +3, RSV +2, TGH +2 | `talent.warden.*` (§7.2, §15.4) |
| `calling.oracle` | WIL | divination, devotion, conjuration | appraisal, illusion, speech | ITU +3, PRV +3, MEM +2, FOC +2 | `talent.oracle.*` (§7.2, §15.4) |
| `calling.netrunner` | INT | hacking, tinkering, sneak | security, awareness, illusion | LOG +4, AGI +2, HRG +2, SLT +2 | `talent.netrunner.*` (§7.2, §15.4) |
| `calling.beastbinder` | PRE | handling, conjuration, survival | polearm, leadership, restoration | CHM +3, SGT +3, MEM +2, END +2 | `talent.beastbinder.*` (§7.2, §15.4) |
| `calling.monk` | FIN | unarmed, acrobatics, athletics | alteration, medicine, awareness | PWR +3, AGI +3, END +2, FOC +2 | `talent.monk.*` (§7.2, §15.4) |

Kit roles are §2.5; doc 14 §2.4 binds each to its `run.` kit template.

### 15.4 Talents

170 talents: 12 per Calling tree (tier 1 ×4, tier 2 ×4, tier 3 ×3, tier 4 ×1) and 14 general. Tier gates are §7.1; general tier 2 needs level 10. Kind: **passive**; **active** takes an ability slot (§1.1); **toggle** is switched on and off without a slot. Effects are the §7.2 or §7.3 entry of the same name.

| ID | Tree | Tier | Kind | Requires |
|---|---|---|---|---|
| `talent.vanguard.heavy_hand` | `calling.vanguard` | 1 | passive | — |
| `talent.vanguard.bulwark` | `calling.vanguard` | 1 | passive | — |
| `talent.vanguard.shield_wall` | `calling.vanguard` | 1 | passive | — |
| `talent.vanguard.second_wind` | `calling.vanguard` | 1 | passive | — |
| `talent.vanguard.crushing_blow` | `calling.vanguard` | 2 | passive | — |
| `talent.vanguard.taunting_roar` | `calling.vanguard` | 2 | active | — |
| `talent.vanguard.plated` | `calling.vanguard` | 2 | passive | — |
| `talent.vanguard.unyielding` | `calling.vanguard` | 2 | passive | — |
| `talent.vanguard.juggernaut` | `calling.vanguard` | 3 | passive | — |
| `talent.vanguard.retaliation` | `calling.vanguard` | 3 | passive | — |
| `talent.vanguard.rallying_presence` | `calling.vanguard` | 3 | passive | — |
| `talent.vanguard.last_bastion` | `calling.vanguard` | 4 | passive | `talent.vanguard.unyielding` |
| `talent.duelist.riposte` | `calling.duelist` | 1 | passive | — |
| `talent.duelist.fleet_foot` | `calling.duelist` | 1 | passive | — |
| `talent.duelist.en_garde` | `calling.duelist` | 1 | passive | — |
| `talent.duelist.flourish` | `calling.duelist` | 1 | passive | — |
| `talent.duelist.lunge` | `calling.duelist` | 2 | active | — |
| `talent.duelist.feint` | `calling.duelist` | 2 | passive | — |
| `talent.duelist.challenge` | `calling.duelist` | 2 | active | — |
| `talent.duelist.silver_tongue` | `calling.duelist` | 2 | passive | — |
| `talent.duelist.blade_dance` | `calling.duelist` | 3 | passive | — |
| `talent.duelist.uncanny_dodge` | `calling.duelist` | 3 | passive | — |
| `talent.duelist.coup_de_grace` | `calling.duelist` | 3 | passive | — |
| `talent.duelist.thousand_cuts` | `calling.duelist` | 4 | passive | `talent.duelist.blade_dance` |
| `talent.gunslinger.steady_aim` | `calling.gunslinger` | 1 | passive | — |
| `talent.gunslinger.fast_hands` | `calling.gunslinger` | 1 | passive | — |
| `talent.gunslinger.long_shot` | `calling.gunslinger` | 1 | passive | — |
| `talent.gunslinger.trick_toss` | `calling.gunslinger` | 1 | passive | — |
| `talent.gunslinger.fan_the_hammer` | `calling.gunslinger` | 2 | passive | — |
| `talent.gunslinger.deadeye` | `calling.gunslinger` | 2 | passive | — |
| `talent.gunslinger.dive_shot` | `calling.gunslinger` | 2 | passive | — |
| `talent.gunslinger.quick_draw` | `calling.gunslinger` | 2 | passive | — |
| `talent.gunslinger.mark_for_death` | `calling.gunslinger` | 3 | active | — |
| `talent.gunslinger.ricochet` | `calling.gunslinger` | 3 | passive | — |
| `talent.gunslinger.high_noon` | `calling.gunslinger` | 3 | passive | — |
| `talent.gunslinger.dead_mans_hand` | `calling.gunslinger` | 4 | passive | `talent.gunslinger.deadeye` |
| `talent.grenadier.blast_padding` | `calling.grenadier` | 1 | passive | — |
| `talent.grenadier.bandolier` | `calling.grenadier` | 1 | passive | — |
| `talent.grenadier.big_iron` | `calling.grenadier` | 1 | passive | — |
| `talent.grenadier.sapper` | `calling.grenadier` | 1 | passive | — |
| `talent.grenadier.cook_off` | `calling.grenadier` | 2 | passive | — |
| `talent.grenadier.shrapnel` | `calling.grenadier` | 2 | passive | — |
| `talent.grenadier.bracing` | `calling.grenadier` | 2 | passive | — |
| `talent.grenadier.demolition` | `calling.grenadier` | 2 | passive | — |
| `talent.grenadier.chain_reaction` | `calling.grenadier` | 3 | passive | — |
| `talent.grenadier.suppressing_fire` | `calling.grenadier` | 3 | passive | — |
| `talent.grenadier.walking_arsenal` | `calling.grenadier` | 3 | passive | — |
| `talent.grenadier.carpet_bomb` | `calling.grenadier` | 4 | active | `talent.grenadier.cook_off` |
| `talent.spellwright.efficient_casting` | `calling.spellwright` | 1 | passive | — |
| `talent.spellwright.glyph_scholar` | `calling.spellwright` | 1 | passive | — |
| `talent.spellwright.overchannel` | `calling.spellwright` | 1 | toggle | — |
| `talent.spellwright.arcane_focus` | `calling.spellwright` | 1 | passive | — |
| `talent.spellwright.extend` | `calling.spellwright` | 2 | passive | — |
| `talent.spellwright.widen` | `calling.spellwright` | 2 | passive | — |
| `talent.spellwright.mana_shield` | `calling.spellwright` | 2 | passive | — |
| `talent.spellwright.quicken` | `calling.spellwright` | 2 | passive | — |
| `talent.spellwright.spell_echo` | `calling.spellwright` | 3 | passive | — |
| `talent.spellwright.elemental_attunement` | `calling.spellwright` | 3 | passive | — |
| `talent.spellwright.complex_lattice` | `calling.spellwright` | 3 | passive | — |
| `talent.spellwright.grand_design` | `calling.spellwright` | 4 | passive | `talent.spellwright.complex_lattice` |
| `talent.alchemist.potent_brews` | `calling.alchemist` | 1 | passive | — |
| `talent.alchemist.iron_stomach` | `calling.alchemist` | 1 | passive | — |
| `talent.alchemist.quick_quaff` | `calling.alchemist` | 1 | passive | — |
| `talent.alchemist.splash_master` | `calling.alchemist` | 1 | passive | — |
| `talent.alchemist.double_batch` | `calling.alchemist` | 2 | passive | — |
| `talent.alchemist.volatile_mix` | `calling.alchemist` | 2 | passive | — |
| `talent.alchemist.field_medic` | `calling.alchemist` | 2 | passive | — |
| `talent.alchemist.transmuter` | `calling.alchemist` | 2 | passive | — |
| `talent.alchemist.catalyst` | `calling.alchemist` | 3 | passive | — |
| `talent.alchemist.philosophers_vial` | `calling.alchemist` | 3 | passive | — |
| `talent.alchemist.antivenom` | `calling.alchemist` | 3 | passive | — |
| `talent.alchemist.magnum_opus` | `calling.alchemist` | 4 | passive | `talent.alchemist.potent_brews`, `skill.alchemy` ≥ 75 |
| `talent.shade.soft_step` | `calling.shade` | 1 | passive | — |
| `talent.shade.backstab` | `calling.shade` | 1 | passive | — |
| `talent.shade.light_fingers` | `calling.shade` | 1 | passive | — |
| `talent.shade.nightborn` | `calling.shade` | 1 | passive | — |
| `talent.shade.vanish` | `calling.shade` | 2 | active | — |
| `talent.shade.quiet_kill` | `calling.shade` | 2 | passive | — |
| `talent.shade.envenom` | `calling.shade` | 2 | passive | — |
| `talent.shade.escape_artist` | `calling.shade` | 2 | passive | — |
| `talent.shade.shadow_mark` | `calling.shade` | 3 | passive | — |
| `talent.shade.death_from_above` | `calling.shade` | 3 | passive | — |
| `talent.shade.ghost_walk` | `calling.shade` | 3 | passive | — |
| `talent.shade.unseen_blade` | `calling.shade` | 4 | passive | `talent.shade.backstab`, `skill.sneak` ≥ 50 |
| `talent.artificer.overclock` | `calling.artificer` | 1 | passive | — |
| `talent.artificer.field_repair` | `calling.artificer` | 1 | passive | — |
| `talent.artificer.scrap_savant` | `calling.artificer` | 1 | passive | — |
| `talent.artificer.turret_tech` | `calling.artificer` | 1 | passive | — |
| `talent.artificer.modular` | `calling.artificer` | 2 | passive | — |
| `talent.artificer.drone_companion` | `calling.artificer` | 2 | passive | — |
| `talent.artificer.reinforced` | `calling.artificer` | 2 | passive | — |
| `talent.artificer.hardlight` | `calling.artificer` | 2 | passive | — |
| `talent.artificer.masterwork` | `calling.artificer` | 3 | passive | — |
| `talent.artificer.self_destruct` | `calling.artificer` | 3 | passive | — |
| `talent.artificer.smart_rounds` | `calling.artificer` | 3 | passive | — |
| `talent.artificer.legion` | `calling.artificer` | 4 | passive | `talent.artificer.turret_tech` |
| `talent.warden.guardian` | `calling.warden` | 1 | passive | — |
| `talent.warden.mender` | `calling.warden` | 1 | passive | — |
| `talent.warden.warding` | `calling.warden` | 1 | passive | — |
| `talent.warden.steadfast` | `calling.warden` | 1 | passive | — |
| `talent.warden.intercept` | `calling.warden` | 2 | active | — |
| `talent.warden.purge` | `calling.warden` | 2 | passive | — |
| `talent.warden.aegis_bash` | `calling.warden` | 2 | passive | — |
| `talent.warden.sanctuary` | `calling.warden` | 2 | passive | — |
| `talent.warden.martyr` | `calling.warden` | 3 | passive | — |
| `talent.warden.resurgence` | `calling.warden` | 3 | passive | — |
| `talent.warden.bastion` | `calling.warden` | 3 | passive | — |
| `talent.warden.last_light` | `calling.warden` | 4 | passive | `talent.warden.martyr` |
| `talent.oracle.foresight` | `calling.oracle` | 1 | passive | — |
| `talent.oracle.devout` | `calling.oracle` | 1 | passive | — |
| `talent.oracle.spirit_guide` | `calling.oracle` | 1 | passive | — |
| `talent.oracle.omen_reader` | `calling.oracle` | 1 | passive | — |
| `talent.oracle.prophecy` | `calling.oracle` | 2 | passive | — |
| `talent.oracle.divine_favor` | `calling.oracle` | 2 | passive | — |
| `talent.oracle.spectral_host` | `calling.oracle` | 2 | passive | — |
| `talent.oracle.glimpse` | `calling.oracle` | 2 | active | — |
| `talent.oracle.fated` | `calling.oracle` | 3 | passive | — |
| `talent.oracle.omen_ward` | `calling.oracle` | 3 | passive | — |
| `talent.oracle.consecrated` | `calling.oracle` | 3 | passive | — |
| `talent.oracle.avatar_of_fate` | `calling.oracle` | 4 | active, once per Folio | `talent.oracle.fated` |
| `talent.netrunner.backdoor` | `calling.netrunner` | 1 | passive | — |
| `talent.netrunner.ghost_signal` | `calling.netrunner` | 1 | passive | — |
| `talent.netrunner.overload` | `calling.netrunner` | 1 | passive | — |
| `talent.netrunner.firewall` | `calling.netrunner` | 1 | passive | — |
| `talent.netrunner.puppet_master` | `calling.netrunner` | 2 | passive | — |
| `talent.netrunner.daemon` | `calling.netrunner` | 2 | active | — |
| `talent.netrunner.jammer` | `calling.netrunner` | 2 | passive | — |
| `talent.netrunner.remote_access` | `calling.netrunner` | 2 | passive | — |
| `talent.netrunner.botnet` | `calling.netrunner` | 3 | passive | — |
| `talent.netrunner.feedback` | `calling.netrunner` | 3 | passive | — |
| `talent.netrunner.data_thief` | `calling.netrunner` | 3 | passive | — |
| `talent.netrunner.system_crash` | `calling.netrunner` | 4 | active | `talent.netrunner.daemon` |
| `talent.beastbinder.pack_leader` | `calling.beastbinder` | 1 | passive | — |
| `talent.beastbinder.feral_bond` | `calling.beastbinder` | 1 | passive | — |
| `talent.beastbinder.tracker` | `calling.beastbinder` | 1 | passive | — |
| `talent.beastbinder.beast_tongue` | `calling.beastbinder` | 1 | passive | — |
| `talent.beastbinder.sic_em` | `calling.beastbinder` | 2 | active | — |
| `talent.beastbinder.hardy_beasts` | `calling.beastbinder` | 2 | passive | — |
| `talent.beastbinder.kindred_bond` | `calling.beastbinder` | 2 | passive | — |
| `talent.beastbinder.shared_senses` | `calling.beastbinder` | 2 | passive | — |
| `talent.beastbinder.alpha` | `calling.beastbinder` | 3 | passive | — |
| `talent.beastbinder.primal_fury` | `calling.beastbinder` | 3 | active | — |
| `talent.beastbinder.herd_mend` | `calling.beastbinder` | 3 | passive | — |
| `talent.beastbinder.chimera_call` | `calling.beastbinder` | 4 | active | `talent.beastbinder.alpha` |
| `talent.monk.iron_fist` | `calling.monk` | 1 | passive | — |
| `talent.monk.wind_step` | `calling.monk` | 1 | passive | — |
| `talent.monk.centered` | `calling.monk` | 1 | passive | — |
| `talent.monk.deflect` | `calling.monk` | 1 | passive | — |
| `talent.monk.flurry` | `calling.monk` | 2 | passive | — |
| `talent.monk.pressure_points` | `calling.monk` | 2 | passive | — |
| `talent.monk.still_breath` | `calling.monk` | 2 | passive | — |
| `talent.monk.roll_with_it` | `calling.monk` | 2 | passive | — |
| `talent.monk.stunning_palm` | `calling.monk` | 3 | active | — |
| `talent.monk.diamond_body` | `calling.monk` | 3 | passive | — |
| `talent.monk.drunken_master` | `calling.monk` | 3 | passive | — |
| `talent.monk.quivering_palm` | `calling.monk` | 4 | active | `talent.monk.stunning_palm` |
| `talent.general.toughened` | general | 1 | passive | — |
| `talent.general.quick_learner` | general | 1 | passive | — |
| `talent.general.pack_mule` | general | 1 | passive | — |
| `talent.general.lucky` | general | 1 | passive | — |
| `talent.general.polyglot` | general | 1 | passive | — |
| `talent.general.iron_will` | general | 1 | passive | — |
| `talent.general.forager` | general | 1 | passive | — |
| `talent.general.hardy` | general | 1 | passive | — |
| `talent.general.marathon` | general | 1 | passive | — |
| `talent.general.polyglot_ii` | general | 2 | passive | `talent.general.polyglot` |
| `talent.general.keen_senses` | general | 2 | passive | — |
| `talent.general.field_dressing` | general | 2 | passive | — |
| `talent.general.scavenger` | general | 2 | passive | — |
| `talent.general.light_sleeper` | general | 2 | passive | — |

### 15.5 Kin

| ID | Body plan | Added parts | Creation bonus | Traits (§15.6) |
|---|---|---|---|---|
| `kin.human` | `body.humanoid` | none | +4 free points; +5 to one skill | `trait.kin.human.versatile` |
| `kin.construct` | `body.humanoid` | none | none | `trait.kin.construct.machine_body`, `trait.kin.construct.fuel_cell`, `trait.kin.construct.conductive`, `trait.kin.construct.poor_chemistry`, `trait.kin.construct.self_repair`, `trait.kin.construct.plated` |
| `kin.undead` | `body.humanoid` | none | none | `trait.kin.undead.deathless`, `trait.kin.undead.umbral_communion`, `trait.kin.undead.light_averse`, `trait.kin.undead.grave_appetite`, `trait.kin.undead.unfeeling` |
| `kin.beastkin` | `body.humanoid` | `bp.tail` (share 500 from `bp.torso`, hit_mult 8500, precision no) | none | `trait.kin.beastkin.claws`, `trait.kin.beastkin.night_eyes`, `trait.kin.beastkin.tail_balance`, `trait.kin.beastkin.keen_nose`, `trait.kin.beastkin.fast_metabolism` |
| `kin.fae` | `body.humanoid` | none | none | `trait.kin.fae.glamour_born`, `trait.kin.fae.iron_bane`, `trait.kin.fae.light_step`, `trait.kin.fae.fair_folk` |
| `kin.mutant` | `body.humanoid` | none | one mutation from 3 offered (§9.3) | `trait.kin.mutant.unstable_genome`, `trait.kin.mutant.adaptive`, `trait.kin.mutant.mutable_flesh` |

Doc 13 §2.4 applies kin to sapient enemies with the Errata-only clauses removed; the IDs are these.

### 15.6 Traits

Origin, kin and Marginalia traits have dependency none and `src_lvl = d_cur` (§1.1). Kind: **passive**, **active** (ability slot), **channel** (an action, not a slot) or **creation** (applies at creation only).

| ID | Bearer | Kind | Effect |
|---|---|---|---|
| `trait.origin.destined` | `origin.mythic_fantasy` | passive | §2.3 item 1 |
| `trait.origin.monster_hunter` | `origin.gothic_horror` | passive | §2.3 item 2 |
| `trait.origin.already_broken` | `origin.cosmic_horror` | passive | §2.3 item 3 |
| `trait.origin.spacer` | `origin.space_opera` | passive | §2.3 item 4 |
| `trait.origin.wired` | `origin.cyberpunk` | passive | §2.3 item 5 |
| `trait.origin.scavenger` | `origin.post_apocalypse` | passive | §2.3 item 6 |
| `trait.origin.quickdraw` | `origin.weird_west` | passive | §2.3 item 7 |
| `trait.origin.gumshoe` | `origin.hardboiled_noir` | passive | §2.3 item 8 |
| `trait.origin.qinggong` | `origin.wuxia` | passive | §2.3 item 9 |
| `trait.origin.gearwright` | `origin.steampunk` | passive | §2.3 item 10 |
| `trait.origin.courtesy` | `origin.fairy_tale` | passive | §2.3 item 11 |
| `trait.origin.favored` | `origin.mythic_antiquity` | passive | §2.3 item 12 |
| `trait.origin.lucid` | `origin.oneiric` | passive | §2.3 item 13 |
| `trait.origin.cleared` | `origin.redacted` | passive | §2.3 item 14 |
| `trait.origin.blank_page` | `origin.unwritten` | creation | §2.3 item 15 |
| `trait.origin.first_draft` | `origin.ur_text` | passive | §2.3 item 16 |
| `trait.kin.human.versatile` | `kin.human` | creation | §2.4 `kin.human` |
| `trait.kin.construct.machine_body` | `kin.construct` | passive | §2.4 `kin.construct` |
| `trait.kin.construct.fuel_cell` | `kin.construct` | passive | §2.4 `kin.construct` |
| `trait.kin.construct.conductive` | `kin.construct` | passive | §2.4 `kin.construct` |
| `trait.kin.construct.poor_chemistry` | `kin.construct` | passive | §2.4 `kin.construct` |
| `trait.kin.construct.self_repair` | `kin.construct` | channel | §2.4 `kin.construct` |
| `trait.kin.construct.plated` | `kin.construct` | passive | §2.4 `kin.construct` |
| `trait.kin.undead.deathless` | `kin.undead` | passive | §2.4 `kin.undead` |
| `trait.kin.undead.umbral_communion` | `kin.undead` | passive | §2.4 `kin.undead` |
| `trait.kin.undead.light_averse` | `kin.undead` | passive | §2.4 `kin.undead` |
| `trait.kin.undead.grave_appetite` | `kin.undead` | passive | §2.4 `kin.undead` |
| `trait.kin.undead.unfeeling` | `kin.undead` | passive | §2.4 `kin.undead` |
| `trait.kin.beastkin.claws` | `kin.beastkin` | passive | §2.4 `kin.beastkin` |
| `trait.kin.beastkin.night_eyes` | `kin.beastkin` | passive | §2.4 `kin.beastkin` |
| `trait.kin.beastkin.tail_balance` | `kin.beastkin` | passive | §2.4 `kin.beastkin` |
| `trait.kin.beastkin.keen_nose` | `kin.beastkin` | passive | §2.4 `kin.beastkin` |
| `trait.kin.beastkin.fast_metabolism` | `kin.beastkin` | passive | §2.4 `kin.beastkin` |
| `trait.kin.fae.glamour_born` | `kin.fae` | passive | §2.4 `kin.fae` |
| `trait.kin.fae.iron_bane` | `kin.fae` | passive | §2.4 `kin.fae` |
| `trait.kin.fae.light_step` | `kin.fae` | passive | §2.4 `kin.fae` |
| `trait.kin.fae.fair_folk` | `kin.fae` | passive | §2.4 `kin.fae` |
| `trait.kin.mutant.unstable_genome` | `kin.mutant` | creation and passive | §2.4 `kin.mutant` |
| `trait.kin.mutant.adaptive` | `kin.mutant` | passive | §2.4 `kin.mutant` |
| `trait.kin.mutant.mutable_flesh` | `kin.mutant` | passive | §2.4 `kin.mutant` |
| `trait.marginalia.strikethrough` | `afflict.red_ink` dose | active | §10.3 item 1 |
| `trait.marginalia.redaction` | `afflict.red_ink` dose | active | §10.3 item 2 |
| `trait.marginalia.rewrite_wound` | `afflict.red_ink` dose | active | §10.3 item 3 |
| `trait.marginalia.ink_sight` | `afflict.red_ink` dose | passive | §10.3 item 4 |
| `trait.marginalia.footnote` | `afflict.red_ink` dose | passive | §10.3 item 5 |
| `trait.marginalia.erase` | `afflict.red_ink` dose | active | §10.3 item 6 |
| `trait.marginalia.margin_step` | `afflict.red_ink` dose | active | §10.3 item 7 |
| `trait.marginalia.dictation` | `afflict.red_ink` dose | active | §10.3 item 8 |

Echo signature traits: variant A of each of the 24 launch Echoes. Doc 14 §6.3 authors each effect as §2.6.1 terms, in the row of the Echo with the same suffix; §2.6.1 validates it. Variants B and C are `trait.echo.<suffix>_b` and `trait.echo.<suffix>_c` (doc 15 Folds 2 and 5) and must have variant A's `net`.

| ID | Echo | Effect |
|---|---|---|
| `trait.echo.fenrir` | `echo.fenrir` | doc 14 §6.3 |
| `trait.echo.wren_long_index` | `echo.wren_long_index` | doc 14 §6.3 |
| `trait.echo.lilith` | `echo.lilith` | doc 14 §6.3 |
| `trait.echo.mother_ilse` | `echo.mother_ilse` | doc 14 §6.3 |
| `trait.echo.jormungandr` | `echo.jormungandr` | doc 14 §6.3 |
| `trait.echo.drowned_clerk` | `echo.drowned_clerk` | doc 14 §6.3 |
| `trait.echo.ostra_vey` | `echo.ostra_vey` | doc 14 §6.3 |
| `trait.echo.kestrel_unit` | `echo.kestrel_unit` | doc 14 §6.3 |
| `trait.echo.nix_seventeen` | `echo.nix_seventeen` | doc 14 §6.3 |
| `trait.echo.ronin_valdez` | `echo.ronin_valdez` | doc 14 §6.3 |
| `trait.echo.ma_rust` | `echo.ma_rust` | doc 14 §6.3 |
| `trait.echo.tallow` | `echo.tallow` | doc 14 §6.3 |
| `trait.echo.josephine_crowe` | `echo.josephine_crowe` | doc 14 §6.3 |
| `trait.echo.preacher_amos` | `echo.preacher_amos` | doc 14 §6.3 |
| `trait.echo.dex_halloran` | `echo.dex_halloran` | doc 14 §6.3 |
| `trait.echo.vera_lux` | `echo.vera_lux` | doc 14 §6.3 |
| `trait.echo.lan_ninth_peak` | `echo.lan_ninth_peak` | doc 14 §6.3 |
| `trait.echo.sun_wukong` | `echo.sun_wukong` | doc 14 §6.3 |
| `trait.echo.ada_brassvane` | `echo.ada_brassvane` | doc 14 §6.3 |
| `trait.echo.colonel_wexley` | `echo.colonel_wexley` | doc 14 §6.3 |
| `trait.echo.briar_thornwood` | `echo.briar_thornwood` | doc 14 §6.3 |
| `trait.echo.wolf_in_wool` | `echo.wolf_in_wool` | doc 14 §6.3 |
| `trait.echo.kallias_twice_drowned` | `echo.kallias_twice_drowned` | doc 14 §6.3 |
| `trait.echo.loki` | `echo.loki` | doc 14 §6.3 |

### 15.7 Afflictions and wounds

| ID | Skins (Canon §16.4) | Contraction | Effects |
|---|---|---|---|
| `afflict.hunger` | vampirism, ghoul hunger, jiangshi | §10.1 meter, then Incubating 7200 ticks | §10.2 |
| `afflict.beast` | lycanthropy, were-forms | §10.1 meter, then Incubating 7200 ticks | §10.2; form `summon.poly.beast` (doc 06 §12.3) |
| `afflict.red_ink` | the Editors' forbidden ink | §10.1 meter or `add_affliction_dose`; up to 4 doses | §10.2; one `trait.marginalia.*` power per dose (§10.3) |

| ID | Class | Rule (§8.2) |
|---|---|---|
| `wound.bruised` | tier 1 | `frac` in `5001..=7500` |
| `wound.wounded` | tier 2 | `frac` in `2501..=5000` |
| `wound.crippled` | tier 3 | `frac <= 2500` and `pool > 0` |
| `wound.disabled` | tier 4 | `pool = 0`, not severed |
| `wound.severed` | severed | the part is severed (§8.6) |
| `wound.laceration` | kind | largest share slash or pierce |
| `wound.fracture` | kind | largest share blunt, or fall damage |
| `wound.burn` | kind | largest share fire, frost, acid, shock, radiation or sonic |
| `wound.trauma` | kind | largest share toxin, radiant, umbral, psychic, arcane or true |

A tier ID is also the source ID of that tier's §8.3 modifiers.

### 15.8 Body plans and parts

| ID | Parts (Canon §9) | Donor of added parts | Base speeds and slots |
|---|---|---|---|
| `body.humanoid` | `bp.head`, `bp.torso`, `bp.arm_l`, `bp.arm_r`, `bp.leg_l`, `bp.leg_r` | `bp.torso` | Canon §4.1–§4.2 |
| `body.quadruped` | `bp.head`, `bp.torso`, `bp.leg_fl`, `bp.leg_fr`, `bp.leg_hl`, `bp.leg_hr` | `bp.torso` | §13 |
| `body.avian` | `bp.head`, `bp.torso`, `bp.wing_l`, `bp.wing_r`, `bp.leg_l`, `bp.leg_r` | `bp.torso` | §13 |
| `body.serpentine` | `bp.head`, `bp.body_front`, `bp.body_rear`, `bp.tail` | `bp.body_front` | §13 |
| `body.insectoid` | `bp.head`, `bp.thorax`, `bp.abdomen`, `bp.leg_1` to `bp.leg_6` | `bp.thorax` | §13 |
| `body.amorphous` | `bp.core`, `bp.mass` | `bp.mass` | §13 |
| `body.construct` | `bp.core`, `bp.chassis`, `bp.limb_1` to `bp.limb_4` | `bp.chassis` | §13 |
| `body.swarm` | `bp.mass` | `bp.mass` | NPC only (doc 13) |

Parts. Share, hit_mult and precision are Canon §9 for native parts; an added part takes its share from its plan's donor part. Mass share is the part's slice of body mass for body-part items (§8.6); an added part's mass share equals its share and is taken from the donor's mass share. Severable follows Canon §9.1: limbs, tails and wings on every plan, heads on NPCs only. The wound row is the §8.3 row the part uses.

| ID | Share Bp by plan | hit_mult Bp | Precision | Severable | Mass share Bp | Wound row (§8.3) |
|---|---|---|---|---|---|---|
| `bp.head` | humanoid 1500, quadruped 1500, avian 1500, serpentine 2000, insectoid 1500 | 10000 | yes | NPCs only | humanoid 700, quadruped 900, avian 800, serpentine 1200, insectoid 1000 | head |
| `bp.torso` | humanoid 3500, quadruped 4500, avian 3500 | 10000 | no | no | humanoid 4700, quadruped 5500, avian 5000 | torso |
| `bp.arm_l` | humanoid 1250 | 8500 | no | yes | 650 | arm |
| `bp.arm_r` | humanoid 1250 | 8500 | no | yes | 650 | arm |
| `bp.leg_l` | humanoid 1250, avian 1000 | 8500 | no | yes | humanoid 1650, avian 1000 | leg |
| `bp.leg_r` | humanoid 1250, avian 1000 | 8500 | no | yes | humanoid 1650, avian 1000 | leg |
| `bp.leg_fl` | quadruped 1000 | 8500 | no | yes | 900 | leg, quadruped factors (§8.3) |
| `bp.leg_fr` | quadruped 1000 | 8500 | no | yes | 900 | leg, quadruped factors (§8.3) |
| `bp.leg_hl` | quadruped 1000 | 8500 | no | yes | 900 | leg, quadruped factors (§8.3) |
| `bp.leg_hr` | quadruped 1000 | 8500 | no | yes | 900 | leg, quadruped factors (§8.3) |
| `bp.wing_l` | avian 1500; added by `aug.wings` 750 | 8500 | no | yes | avian 1100; added 750 | wings |
| `bp.wing_r` | avian 1500; added by `aug.wings` 750 | 8500 | no | yes | avian 1100; added 750 | wings |
| `bp.body_front` | serpentine 4000 | 10000 | no | no | 4000 | torso |
| `bp.body_rear` | serpentine 3000 | 9000 | no | no | 3500 | one leg (locomotion part) |
| `bp.tail` | serpentine 1000; added by `kin.beastkin` or `aug.tail` 500 | 8500 | no | yes | serpentine 1300; added 500 | tail |
| `bp.thorax` | insectoid 3500 | 10000 | no | no | 3500 | torso |
| `bp.abdomen` | insectoid 2600 | 10000 | no | no | 3502 | torso |
| `bp.leg_1` | insectoid 400 | 8000 | no | yes | 333 | arm when bound as an arm (§13) |
| `bp.leg_2` | insectoid 400 | 8000 | no | yes | 333 | arm when bound as an arm (§13) |
| `bp.leg_3` | insectoid 400 | 8000 | no | yes | 333 | `BodyMove 8500` at tier 4 or severed; none below tier 4 |
| `bp.leg_4` | insectoid 400 | 8000 | no | yes | 333 | `BodyMove 8500` at tier 4 or severed; none below tier 4 |
| `bp.leg_5` | insectoid 400 | 8000 | no | yes | 333 | `BodyMove 8500` at tier 4 or severed; none below tier 4 |
| `bp.leg_6` | insectoid 400 | 8000 | no | yes | 333 | `BodyMove 8500` at tier 4 or severed; none below tier 4 |
| `bp.core` | amorphous 2000, construct 2000 | 10000 | yes | no | construct 1500; amorphous none (nothing severs, §13) | head |
| `bp.mass` | amorphous 8000, swarm 10000 | 10000 | no | no | none (never severs) | amorphous torso; swarm none |
| `bp.chassis` | construct 5000 | 10000 | no | no | 5500 | torso |
| `bp.limb_1` | construct 750 | 8500 | no | yes | 750 | arm |
| `bp.limb_2` | construct 750 | 8500 | no | yes | 750 | arm |
| `bp.limb_3` | construct 750 | 8500 | no | yes | 750 | leg |
| `bp.limb_4` | construct 750 | 8500 | no | yes | 750 | leg |
| `bp.arm_extra_1` | added: `aug.extra_arm` 1000; serpentine Echo 750 (§13) | 8500 | no | yes | equal to its share | extra arms |
| `bp.arm_extra_2` | added: `aug.extra_arm` 1000; serpentine Echo 750 (§13) | 8500 | no | yes | equal to its share | extra arms |
| `bp.horn` | added: 500, by an NPC template or an augment that names it | 8500 | no | no | 500 | extra parts (tail, wings, extra arms row) |
| `bp.weakpoint_1` | added: the boss template's share (doc 13) | 10000 | yes | no | equal to its share | its mapped attacks off at tier 4 (NPC part-to-attack map) |
| `bp.weakpoint_2` | added: the boss template's share (doc 13) | 10000 | yes | no | equal to its share | its mapped attacks off at tier 4 (NPC part-to-attack map) |
| `bp.weakpoint_3` | added: the boss template's share (doc 13) | 10000 | yes | no | equal to its share | its mapped attacks off at tier 4 (NPC part-to-attack map) |
| `bp.weakpoint_4` | added: the boss template's share (doc 13) | 10000 | yes | no | equal to its share | its mapped attacks off at tier 4 (NPC part-to-attack map) |

`bp.arm_extra_1` is added first and `bp.arm_extra_2` second; they grant `slot.hand_extra_1` and `slot.hand_extra_2` (Canon §9.1, §13.2), and no body has a third. Boss templates use `bp.weakpoint_1` to `bp.weakpoint_4`.

---
## Exports

All in namespaces Canon §19 assigns to doc 04. §15 lists every ID with its structural data; §2.3 (Origins) and §9.4 (augments) are registers in place. An ID in these namespaces that is not listed here is invalid.

| Namespace | IDs defined or given behavior here |
|---|---|
| `attr.` | the 8 Canon primaries (§15.1): uses, display tiers, talent gates (§4) |
| `sub.` | the 24 Canon sub-attributes (§15.1): tooltips, doc-04 uses (§3) |
| `skill.` | the 36 Canon skills (§15.2): XP events (§5.2), effects (§5.3); 144 mastery perks `skill.<name>.m25`, `.m50`, `.m75`, `.m100` (§5.3, §15.2) |
| `talent.` | 156 Calling talents `talent.<calling_suffix>.<name>` (§7.2) and 14 `talent.general.<name>` (§7.3), each with tier, kind and prerequisite in §15.4 |
| `origin.` | 16 Origins `origin.<genre_suffix>` (§2.3) |
| `calling.` | the 13 Canon Callings (§15.3): presets, kits, trees, key primaries (§2.5, §7) |
| `kin.` | the 6 Canon kin (§15.5): full trait lists and skins (§2.4) |
| `trait.` | `trait.origin.*` (16), `trait.kin.<kin>.*` (human 1, construct 6, undead 5, beastkin 5, fae 4, mutant 3) and `trait.marginalia.*` (8), all in §15.6; the 24 Echo signature traits `trait.echo.<suffix>` (§15.6), whose effects doc 14 §6.3 authors and §2.6.1 validates, with variants `trait.echo.<suffix>_b` and `_c` (doc 15 Folds 2 and 5) |
| `aug.` | 33 augments (§9.4), including `aug.prosthetic_arm` and `aug.prosthetic_leg`; 27 have an implant form, which doc 10's implant appearance class counts (Interface assumptions) |
| `afflict.` | the 3 Canon afflictions `afflict.hunger`, `afflict.beast`, `afflict.red_ink` (§15.7): contraction, effects, cure (§10) |
| `wound.` | tiers `wound.bruised`, `wound.wounded`, `wound.crippled`, `wound.disabled`, `wound.severed`; kinds `wound.laceration`, `wound.fracture`, `wound.burn`, `wound.trauma` (§8.2, §15.7) |
| `body.`, `bp.` | the 8 Canon plans and their 30 native parts (§15.8): wound effects, mass shares, Echo body rules (§8, §13); added parts `bp.tail`, `bp.wing_l`, `bp.wing_r`, `bp.arm_extra_1`, `bp.arm_extra_2`, `bp.horn` and boss parts `bp.weakpoint_1` to `bp.weakpoint_4`, with donor parts per plan (§15.8) |

Behavior is also given for the Canon statuses and resources doc 04 owns (Canon §11.1, §12).

Functions exported to other docs: `grant_skill_xp`, `grant_char_xp`, `check(attr, skill, dc)`, `revive_bp(reviver)` (doc 16), `on_part_damage` with its severing branch (doc 05 §19.1), `on_heal`, `body_move_bp` (doc 03 §5.2), `anl_speed_bp` (doc 10), `treatment_ticks`, `aug_cap`, `trait_budget(trait) -> (benefit, drawback, net)` (content validator, docs 14 and 15), `going_native(e, genre)` (doc 14 Colophon), `add_affliction_dose(e, afflict)` and `cure_affliction(e, afflict)` (doc 08, doc 11 Scriptorium), `explain_row(entity, row)` (doc 17), `air_jump_max(e)` (doc 03 §5.7), `can_sprint(e)`, `can_jump(e)` and `can_dodge(e)` (§11.5; doc 03, doc 16).

Constants exported: `DOC04_ROW_CAP_TENTHS = 40` (§6.1), `LAST_STAND_TICKS = 300` (§12.1a), `CHECK_PERSIST_BP = 1000` and its cap 3000 (§4.2). Keyed `seq` allocation (Canon §3.3): checks use the actor's per-tick counter from 0, below 0x2000 (§4.2); madness episodes 0xFFFE and Death's Door 0xFFFF (§12.1, §12.4); progress offers 0 for a level-up Interpolation, 1 for a mutation offer and `2 + i` for a joiner's `i`-th Interpolation (§6.4, §6.6, §9.3).

## Interface assumptions

| Doc | Provides, or must match |
|---|---|
| 02 | the bots harness that replays §6.3 on doc 13 §11.2's plan; `isqrt` and fixed-point helpers |
| 03 | event payloads, including the §14.3 registrations of `ev.wound_changed` and `ev.augment_changed` (producer 04), `ev.entity_pacified` (13) and `ev.secret_found` (12); `Env.temp`, `eye_submerged`, `light_at`; the `Air` move mode (`spacer`); §5.7 allows an air jump while `air_jumps < air_jump_max(e)` and supports the Acrobatics 50 wall jump and the Monk `wind_step` wall-run; `sys.needs` after the status pulse; §5.2 applies `body_move_bp(e)` once, after row 11 and before statuses (Canon §4.2); the fall-damage hit multiplied by `fall_dmg_bp` before doc 04's fall factors (Canon §4.2); `comp.body_plan` mirrors §8.1; §9.4's hazard layer is the only source of temperature damage and temperature statuses; `MoverParams` reads §11.5's `can_sprint`, `can_jump` and `can_dodge`; present: §3.4 flag 8 `CLIMB_ANY` with §5.11's climb test (§13), the sprint-slide gated by `can_sprint` at no stamina cost (§5.23) and `peak_y` reset while grappling (§5.24) |
| 05 | archetype → skill map; `arch.melee.unarmed`, with natural-weapon skins for non-humanoid Echoes (quadruped bite, claw and pounce; serpentine bite, constrict and spit); shield parry ticks and `shield_stab`, to which Shield adds `floor_div(skill, 25)` parry ticks; block arcs; aim sway; heavy attacks; firing move penalty; swap ticks; armor `insul_cold`, `insul_heat`; `harness` weapons; `SEVER` flags; §19.1 calls `on_part_damage`, whose severing branch holds the Errata crippled-limb rule (§8.2); `d_armor` in `ev.damage_dealt` for §5.2's Armor row; keyed `seq` values inside Canon §3.3's `rng.combat` reservation, apart from Death's Door's 0xFFFF; requested: the §12.1a last-stand input (three prompted ink-strike inputs in order within `LAST_STAND_TICKS`), reporting success or failure to §12.1a and drawing no random number |
| 06 | `base_cost`, `cost_skill`, tier gates, the §7.3 hook table (identical to §5.3 here), `summon.poly.beast` (the beast form ignores the damage-taken break), supplication needs, the `st.confused` angle, summons |
| 07 | body materials (`mat.flesh`, `mat.bone`, …), recipe tiers 0..=5 (process recipes tier 0), quality shift, the crafter material bonus (`mat_bonus_bp` 500 at Smithing 100), placeable `base_ticks`, block HP and the §10.4 repair formula, which applies the Fieldcraft 75 factor and then × 15000 for `talent.artificer.field_repair`; mineral and salvage nodes and their yields (§9.3; Fieldcraft 75 adds +1 unit); the augment station; `fuel`, `metal` and `repair_kit` tags; `self_repair` supplies (§8.2: a never-consumed `repair_kit` tool and `SELF_REPAIR_UNITS` metal units); each item's governing skill for salvage XP (§8.1); turret, drone and coil hits for the Tinkering XP row; scout drone; material tiers; no item durability |
| 08 | `afflict_power` sources; mutagens; `red_dose` calling `add_affliction_dose`; cure potions and the `st.incubating` abort calling `cure_affliction`; the unbreathable gas flag; respirator and rebreather filters; ingredient reveal tiers, batch and single-ingredient brewing as §5.3 states; `talent.alchemist.double_batch` and `echo.mother_ilse` add +1 to its batch, past its 4-dose ceiling; §10.3 coats, to which `envenom` adds 5 charges and whose buildup factor it raises from 5000 to 10000 |
| 09 | lock and device DCs on the §4.2 ladder; Sneak, Awareness, Security, Pickpocket and Hacking effects as §5.3 states; noise adjustments; disguise Deceive checks on `rng.stealth` seq 1006 with §4.2's rating and chance; a lockpick slip roll against `slip_bp`, which Security 25 multiplies by 5000; lockpicks, toolkits and hacking decks are carried tools that no attempt consumes, wears or breaks (Canon §4.3, doc 09 §13.2); subdue takedowns as pacification; `input.hold_breath` |
| 10 | appraisal using `anl_speed_bp`; appraisal completion and Examine success events for §5.2; curse-state reveal; the tier-1 reveal set that Appraisal 25 applies (§6.3); present: the §7.3 and §7.8 curse and pact terms entering §1.2 as `SubFlat` and `Other` terms; `idc.implant` covers the 27 implant-form augments |
| 11 | item IDs for food, kits and treatments (`item.tool.bandage`, `item.tool.splint`, `item.tool.salve`, `item.tool.surgical_kit`, `item.tool.crutch`), `item.tool.toolkit` tagged `repair_kit` and never consumed, `item.tool.lockpicks`, implants, grafts and prosthetics; Scriptorium prices (reattach 200, cure 500 and "Emend the Flesh" 400, each × `P(d)`); haggling `score` |
| 12 | `secret.` locations, room-visit counts, `hearth` blocks, check targets, lore objects |
| 13 | enemy tags (undead, fiend, eldritch, beast, beast_sapient, graftable, regenerating, disease, electronic); part-to-attack maps; body mass; attack `afflict_power`; deterministic dialogue (§13.7) and taming (§6) with doc-04 bonuses added; §11.1 defeats including pacification, with the Erasure hunter paying `rank.elite` kill XP; the §11.2 plan, which §6.3 replays; quests paying only §6.1's Objective tenths (§13.3); the Redactor invader's statuses follow §12.5 |
| 14 | kit templates (§2.4); joiner values (§2.5) completed by §6.6 here; Rubric and marginal notes; the Colophon full heal and Going Native calling `going_native`; Echo traits written as §2.6.1 terms; Echo body plans (`echo.fenrir` `body.quadruped`, `echo.jormungandr` `body.serpentine`) with §13's slot and kin-part rules; co-op kill XP per Canon §8, where downed members and ghosts share it; Second Edition carrying the character over at depth `25 + 2 × n` for its Folio `n` (§6.7, DECISIONS.md items 102 and 106); present: §1.2's `ErratumChoice::Custom` built from §2.1's output, with `Appearance` and the name outside it (§1.3) |
| 15 | Fold variants B and C of each Echo trait with the same `net` (§2.6.1) |
| 16 | `st.downed` flow, `revive_hp_bp`, consumption of `revive_bp`; ghosts; Dormant and Departed slots; the client predictor calling §11.5's `can_sprint`, `can_jump` and `can_dodge` |
| 17 | `input.ability_1..=4` and `AssignAbilitySlot`; creation, level-up, talent, join and Going Native screens, with §2.1's preset picker in Cosmetics; breakdown tooltips from `explain_row`; `access.mark_hallucinations` marking §12.4's madness phantoms, forced on under Scribe's Mercy; present: §5.7's Cosmetics page with the 12 presets per kin; requested: the §12.1a last-stand prompt and its accessibility forms, a single hold or automatic success (DECISIONS.md item 135) |
| 18 | `loc.` keys, tooltip templates, kin skin names, native forms; the 12 appearance presets per kin with palette and marking colors, present as §5.6's `art.preset.<kin_suffix>.<nn>`, `art.preset.<kin_suffix>.colors` and `Appearance` (§2.1, DECISIONS.md item 97); madness phantoms drawn as the native templates they imitate (§12.4) |

## Canon gaps

Every gap this chapter listed against canon-1.4 is closed by canon-1.11 or by a ruling of `audit/canon_gaps_triage.md`, and the rule now stands in the section named: `rng.check` (G-023; §4.2, §12.4), events (G-184; doc 03 §14.3), modifier placement and fae iron (G-066, G-086; §1.2, §2.4), the body move factor (G-035; §8.5), fall-damage order (G-034; §8.1), the `kin.human` and `kin.construct` readings (G-076; §2.4), `talent_tol` sources (G-061; §1.2), implant capacity (G-077; §9.2), Echo movement (G-078; §13), Errata limb severing and a pool already at 0 (G-080 with DECISIONS.md item 84; §8.2), the XP anchor (G-071, C-21; §6.3), Going Native (G-063; §2.8), Echo trait IDs (G-079; §2.6), Death's Door while Unmoored (G-058; §12.1), charmed players (G-097; §12.5), partless damage (G-083; §8.1), added-part donors (G-081; §15.8) and unspecified added parts (G-082; §15.8). Three items are open:

1. **Leg loss on plans with more than two legs.** Canon §9.1's effects line says losing a leg multiplies move speed by 5000 and removes sprint, jump and dodge, which fits two-legged plans. Canon §4.2 routes leg factors through doc 04's `body_move_bp`. Assumed until the canon rules: a quadruped Echo with one leg lost or at tier 4 moves at `BodyMove 7500` (5000 with two, 2500 with three, §13), NPC quadruped legs use 7500 each at tier 4 or severed (§8.3), and an insectoid uses 8500 per lost walking leg; on these plans losing one leg removes no sprint, jump or dodge. Proposed canon text for §9.1: "Losing a leg of a two-legged plan multiplies move speed by 5000 and removes sprint, jump and dodge; doc 04 §8.3 and §13 set the factors for plans with more legs."
2. **Last stand (DECISIONS.md items 131, 134).** Canon §10.8 has no last stand. Applied here in §12.1a. Proposed canon text, a new step after §10.8 step 2: "2a. **Errata in solo play only:** if Death's Door did not prevent the lethal damage and the last stand is unused this Folio, the Erratum enters a last stand of 300 ticks (doc 04 §12.1a) with HP 0, taking no damage. On success its HP = 1,000 mHP; on failure step 3 applies."
3. **Second Edition depth (DECISIONS.md item 106).** Canon §13.8 has no Second Edition depth. Applied here in §6.7. Proposed canon text, a new bullet in §13.8: "Second Edition Folio `n` (n ≥ 1) uses depth `25 + 2n` in the P, T, H and S tables."

## Acceptance tests

1. **Origin rows.** Each of the 16 rows of §2.3 sums to 0 with every entry in `-6..=6`; `origin.unwritten` is all zero.
2. **Point-buy.** `buy_cost(43, 50) = 7`; `buy_cost(48, 54) = 10`. The §14.1 build spends 18 points with 4 refunded and validates. A build with any value 61, any value 24, or 11 refunded points is rejected. The human Vanguard Quick start preset is PWR +4, TGH +4, BRN +3, RSV +3; every (kin, Origin, Calling) Quick start build validates and its largest value is 49.
3. **Creation stats.** The §14.1 character at level 1, VERVE 5: Max HP 85,000; regen 408 mHP/s; stamina 88; sprint drain 13; aether 61; composure 90; breath 13,000; Death's Door 850 Bp; move 10100; reload 10350; tolerance `(0, 2, 1, 1)`.
4. **Checks.** Force PWR 46 vs DC 55 gives 3200 Bp, and 5200 after two failures on the same target; a success clears the bonus. Climb GRP 38 with Athletics 30 vs DC 40 gives 8200; rating 200 vs DC 10 clamps to 9500; outside `avatar_of_fate` each check makes exactly 1 `range(10000)` call on `rng.check`, with or without the persistence bonus; two checks by one actor on one tick use `seq` 0 and 1. A doc 13 dialogue choice makes no draw, and a disguise Deceive answer makes no `rng.check` draw (doc 09 draws it on `rng.stealth` seq 1006).
5. **Skill growth.** Hacking 49 with 0 progress receiving 23,600 milli-XP at MEM 40 becomes 50, grants `skill.hacking.m50` and adds one LOG mark; at LOG 66 the fifth mark raises LOG to 67.
6. **Combat XP.** A hit removing 20,000 mHP from a 60,000-mHP hostile grants 500 milli-XP; at MEM 50, 525. In a party of 4 the same template has Max HP 153,000 and `norm` 60,000, so the same hit still grants 500. A hit on an ally grants 0.
7. **Skill-up cap.** Ten skill-ups in one Folio at d = 10 grant `8 × 504 = 4032` character XP; the counter resets on `ev.folio_entered`.
8. **Kill XP.** At d = 10: standard 176, elite 529; elite in a party of 4 gives 302 each, ghosts and downed members included. A minion at d = 1 gives 0. At d = 10 the Erasure hunter pays 529 (the elite row) and an invader 882.
9. **Levels.** XP 403,199 is level 27; 403,200 and 446,599 are level 28; 446,600, 452,175, 480,822 and 492,999 are level 29; 493,000 is level 30; 542,500 is level 31.
10. **Pacing.** Replaying §6.3 on doc 13 §11.2's plan gives cumulative XP 480,822 (combat), 453,843 (stealth) and 452,175 (crafting) after Folio 24, all level 29; the combat build's per-Folio gains equal doc 13 §11.2's Folio XP column; hafted reaches 50 in Folio 10 and 75 in Folio 22; every sensitivity row lies in `28..=31`.
11. **Allocation.** A third point into one sub-attribute from a single batch is rejected; with two pending batches, four points into LOG succeed (two per batch); a point into a sub-attribute at base 100 is rejected.
12. **Interpolation.** At level 10 the offer holds 3 distinct tier-1 or tier-2 talents from other trees, uses `pool.len() - 1` `range` calls (95 for a 96-talent pool) with `seq` 0, and reopening shows the same offer.
13. **Respec.** At d = 10 the first talent respec costs 434 Lucre, the second 868; the first attribute respec costs 868.
14. **Talent gates.** A tier-3 talent is refused at level 11 with 5 tree points, accepted at level 12 with key primary 45, refused at key primary 44.
15. **RowPost.** Two × 9000 sources on row 15 give 8100; four × 7000 sources give 2401, clamped to 5000.
16. **Part pools.** At Max HP 85,000: head 12,750, torso 29,750, each arm and leg 10,625. Arm pools 7,970 → tier 0; 7,969 → tier 1; 5,312 → tier 2; 2,656 → tier 3; 0 → tier 4.
17. **Healing cap.** An untreated tier-2 leg at pool 4,000 receiving a heal that restores 45,000 mHP gains 5,625 but stops at 5,312. A bandage at Medicine 0 then sets treated and raises it to 6,905 (tier 1, treated cleared).
18. **Treatment time.** Bandage: 60 ticks at Medicine 0, ANL 40; 39 ticks at Medicine 50, ANL 50. Surgical kit at Medicine 25, ANL 40: 480 ticks.
19. **Severing.** A 60-HP NPC humanoid's arm (pool 7,500) hit for 8,000 mHP with a slash-dominant hit severs and spawns a `cat.body_part`; the same hit with blunt leaves it at tier 4, unsevered, and a later slash-dominant hit dealing 1 mHP to it severs it. An Erratum's head pool at 0 never severs. An 85-HP Erratum's full arm (10,625 mHP) hit for 20,000 mHP by a slash-dominant hit ends at pool 0, `wound.disabled`, unsevered, and a later slash-dominant hit dealing 1 mHP severs it; an Erratum's arm at 2,656 mHP severs from a 3,000-mHP slash-dominant hit, and one at 2,658 mHP does not.
20. **Body part.** A 140,000-g troll's arm weighs 9,100 g; butchering yields 4 meat and 1 bone, or 5 and 2 at Survival 25; it is rotten at age 7200 ticks.
21. **BodyMove.** Left leg tier 3 and right leg tier 2 give 7600; one severed leg with a crutch 7000; both legs severed 2500.
22. **Satiation.** From 1800, 1200 Needs ticks reach 600 and apply `st.hungry`; with `scavenger` the value is 900. Starving at 85 HP loses 850 mHP every 300 ticks.
23. **Food.** A ration at Survival 50 gives 660; a stale meal 450; a fresh snack at Survival 0 gives 300.
24. **Composure.** A 5-point loss in Cosmic Horror costs 7,500 milli; with `already_broken`, 6,000. Composure 0 applies `st.madness`; reaching 22,500 at max 90 ends it.
25. **Temperature.** END 40 at −400 DeciC gives `dev = 450`: `RowPost(5, 7000)`, `RowPost(2, 5000)` and −200 milli composure per Needs tick from doc 04, while doc 04 applies no `st.chilled`, no `st.frozen` buildup and no damage (doc 03 §9.4 does). END 60 gives band `10..=340`.
26. **Breath.** END 40: max 14,000; the 14th submerged Needs tick reaches 0 and applies `st.suffocating`, costing 4,250 mHP/s at 85 HP; the first Needs tick in air raises breath to 5,000 and removes the status. A `kin.construct` never drains; a doc 08 rebreather with `filter > 0` stops the submerged drain.
27. **Encumbrance.** 50,000 g against 45 kg gives `load_bp` 11111 and `st.encumbered`; 12,000 g of armor at Armor 50 counts 9,600 g.
28. **Affliction.** Four bites of power 25 at d = 5 fill a 100 threshold (112), apply `st.incubating` for 7200 ticks, then `afflict.hunger`; the cure at d = 10 costs 1085 Lucre. A doc 08 cure potion drunk during incubation returns the affliction to Clean with one `ev.affliction_changed`.
29. **Red ink.** Four doses set the hunter threshold to 6 and max composure −20; a fifth fill changes nothing.
30. **Augments.** RES 40 gives `aug_cap` 8; with 8 points installed a 1-point install is refused; if RES drops to 39 every augment runs at potency 8000. `aug.eye_lowlight` at rank 3 gives `RowFlat(31, 6)`.
31. **Prosthetic.** The §9.5 cyberarm example gives C = 6 (hook function) for the Mythic Fantasy origin and C = 4, potency 6000, for the Cyberpunk origin.
32. **Death's Door.** One `rng.combat` draw with seq `0xFFFF` per Folio at most; PRV 37 at VERVE 5 uses 850 Bp; success leaves 1,000 mHP and 60 ticks of `st.deaths_door`. Under `st.unmoored` lethal damage makes no draw and `dd_used` is unchanged.
33. **Madness episodes.** One `rng.check` `weighted` call per 600 active ticks; weights sum to 10000. Two clients rendering the same bearer on the same tick of the same run spawn the same phantoms from `pres_hash`, and no simulation stream advances.
34. **Charmed player.** A charmed Erratum's hit on the applier's faction yields `D2 = 0`, no buildup, and one `ev.damage_dealt` with total 0.
35. **Unarmed.** Bare-hand `B` at d = 10 is `mul_bp(6600, 21718) = 14,333` mHP (doc 05 `arch.melee.unarmed`).
36. **Determinism.** Replaying a 10,000-tick input log reproduces all doc-04 state byte for byte.
37. **Trait budget.** All 24 doc 14 variant-A Echo traits pass §2.6.1; `fenrir` scores 2 / 1 / 1, `sun_wukong` 3 / 1 / 2, `loki` 2 / 1 / 1, `nix_seventeen` 4 / 1 / 3, `tallow` 1 / 2 / −1, `lan_ninth_peak` 3 / 3 / 0 and `kallias_twice_drowned` 4 / 1 / 3. A trait with benefit 7, a trait with no drawback, a trait with 4 terms, and a variant B whose `net` differs from variant A are each rejected.
38. **Going Native.** A Cyberpunk-origin character Going Native in a Wuxia Colophon gets `origin_tol` (1, 0, 1, 2) and `trait.origin.qinggong` in place of `wired`; its creation modifiers and Hacking +10 are unchanged; a second attempt in the same run is refused.
39. **Joiner.** A joiner at `join_level` 9 gets 8 batches (32 points) and 4 talent points; at `join_level` 20 it gets two Interpolation offers drawn with `seq` 2 and 3.
40. **XP caps.** Crouch-walking near 3 unaware hostiles for 600 Needs ticks grants exactly 24,000 Sneak milli-XP; doc-04 rows stop paying at 40 tenths in one Folio and resume after `ev.folio_entered`; in Abridged the Rubric at d = 10 pays 420.
41. **Dodge XP.** A dodge that negates a hit with `base_mhp` 10,000 at Max HP 85 grants 352 Acrobatics milli-XP; landing XP stops at 15,000 per Folio.
42. **Spell XP.** A frost bolt removing 20,000 mHP from a 60,000-mHP hostile grants Evocation 500 and, with 2 modifier glyphs, Spellcraft 200; a non-damaging alteration with `base_cost` 15,000 grants 750, or 787 at MEM 50; a fizzle grants 0.
43. **Beast form.** At d = 10 the `afflict.beast` form attacks for 65,154 mHP before attack factors; taking 3000 Bp of Max HP in damage does not end it; it ends after 1800 ticks.
44. **Colophon heal.** After the full heal every non-severed part pool equals its max at tier 0, and a severed arm stays severed.
45. **Composure factors.** `perk.leg.cold_blood` with `already_broken` multiplies composure losses by 5600; a 5-point loss in Cosmic Horror becomes `mul_bp(7500, 5600) = 4200` milli.
46. **Partless damage.** A 4,250-mHP suffocation pulse changes no part pool; a 13,600-mHP fall hit at Max HP 85 puts 6,800 on each leg (pools 3,825, tier 2).
47. **Shield parry.** A heater shield (6 parry ticks) at Shield 50 parries for 8 ticks; a tower shield stays at 0.
48. **Avatar of fate.** During `avatar_of_fate` a 1000 Bp proc rolls at 1500 Bp and crit chance gains 2000 Bp within the 7500 cap; every hit still draws exactly one crit roll; a §4.2 check makes exactly 2 `range(10000)` calls and succeeds when either is below `chance_bp`.
49. **Fieldcraft.** A placeable with `base_ticks` 120 takes 80 ticks at Fieldcraft 50 and 1 tick at Fieldcraft 100; breaking your own placeable grants 0 XP; the Marginalia power `erase` removes destructible blocks and placeables in its volume and leaves fixed terrain unchanged.
50. **Redacted.** An Erratum at Max HP 85 with a leg pool at 1,000 of 10,625 and a severed arm respawns after a Redaction with HP 42,500, that leg at 5,312 (tier 2) and the arm still severed; its `dd_used` is unchanged.
51. **ID register.** Every ID in the doc-04 namespaces (Canon §19) that any chapter cites resolves to a row of §15, §2.3 or §9.4. §15 holds 8 primaries, 24 sub-attributes, 36 skills, 144 perks, 13 Callings, 170 talents, 6 kin, 72 traits (16 Origin, 24 kin, 8 Marginalia, 24 Echo), 3 afflictions, 9 wound IDs, 8 body plans and 37 parts.
52. **Tier floor.** An arm pool of 1 mHP out of 10,625 has `frac = 0` and is tier 3 (`wound.crippled`), not tier 4. A beastkin's severed `bp.tail` has mass share 500 and its torso 4200.
53. **Echo bodies.** `echo.fenrir` builds `body.quadruped` plus `bp.tail` (share 500, leaving `bp.torso` 4000) and holds weapons only in `slot.weapon_1`; `echo.jormungandr` builds `body.serpentine` with exactly one `bp.tail`, plus `bp.arm_extra_1` and `bp.arm_extra_2`, and refuses `aug.extra_arm`. A humanoid installs `aug.extra_arm` twice for `bp.arm_extra_1` and `bp.arm_extra_2`; a third install is refused.
54. **Coats.** At Alchemy 0, with a potion that has no oil base (doc 08 §10.3), a Shade with `envenom` coats a blade with 10 charges and each landed hit adds `mul_bp(p, 10000)` buildup; without the talent the coat has 5 charges and adds `mul_bp(p, 5000)`.
55. **Retired mechanics.** `talent.beastbinder.kindred_bond` gives `RowFlat(18, 10)` and composure losses × 8500 while a tamed beast of its owner is within 8,000 mm, and nothing otherwise. A `kin.construct` Erratum's `self_repair` with an `item.tool.toolkit` and one `mat.scrap_iron` consumes the scrap iron and keeps the toolkit; a second use is refused with `no_supplies`. Smithing 25 gives `RowFlat(19, 500)` and Armor 25 `RowFlat(23, 10)`. Security 25 halves doc 09's `slip_bp` from 2500 to 1250, and no lockpick attempt changes the picker's `item.tool.lockpicks`.
56. **Movement predicates.** A humanoid Erratum with stamina 30, intact legs and no statuses has `can_sprint`, `can_jump` and `can_dodge` true; at stamina 0 sprint and dodge are false and jump is true; under `st.overburdened` all three are false; with one leg at tier 3 only sprint is false; with one leg severed and no prosthetic all three are false; `echo.fenrir` with one severed leg keeps all three at `BodyMove` 7500.
57. **Last stand (§12.1a).** A solo Erratum whose Death's Door roll fails enters a last stand with HP 0 for 300 ticks and takes no damage during it; success leaves 1,000 mHP; failure or timeout emits `ev.entity_killed`. A second lethal hit in the same Folio after a success opens no last stand. In a party of 2 the failed roll applies `st.downed` and no last stand opens. The last stand makes no `rng.*` draw.
58. **Needs (§11).** An Erratum has exactly one survival meter, `res.satiation`; no thirst or fatigue resource or status exists in doc 04.
59. **Second Edition depth (§6.7).** Second Edition Folio 1 reads depth 27 and Folio 3 depth 31 in `K(d)`, `P`, `T` and `S`.
60. **Craft XP tier 0 (§5.2, §6.1).** A tier-0 process recipe with 4 input units grants 4,000 milli-XP to its skill and 0 character-XP tenths; a tier-3 recipe with 4 units grants 10,000 and 9 tenths.
61. **Creation output (§2.1).** A Custom human Vanguard Quick start yields `ErratumChoice::Custom` with 24 `subs` in Canon §6.1 order and `human_skill` set; a non-human yields `None`; two runs differing only in `Appearance` or name have equal `state_hash` sequences; Going Native keeps `Appearance`.
62. **Climb any face (§13).** `body.amorphous` and `body.insectoid` Errata carry `CLIMB_ANY`; a humanoid does not. A sprint-slide spends 0 stamina and is refused while `can_sprint` is false; a grapple pull ending 6 m above its start deals no fall damage.
63. **Curse terms (§1.2).** `curse.frail` lowers its effective sub-attribute before the Canon §6.2 clamp; `curse.thirsting` at full potency (m 2500) multiplies natural regeneration by 7500 after every `RowPost` on row 2 and outside its clamp; `curse.pact.bloodthirst` HP drain emits no `ev.damage_dealt`.

## Invariants annex

```
Σ doc04: creation ⊗ subs ⊗ skills ⊗ levels ⊗ talents ⊗ body ⊗ augments ⊗ needs
W! sub base ∈ 1..=100 ∧ effective ∈ 1..=150          W! creation subs ∈ 25..=60 ∧ origin Σ = 0
W! skill ∈ 0..=100 ∧ growth := need(s) = 4000 + 400 s   N! flat per-hit combat skill XP
W! zero-effect action → 0 XP                            N! XP from allies | neutrals | own minions | own placeables
N! doc-04 effect breaks non-destructible terrain        W! Redacted respawn keeps severed parts severed
W! skill-up char XP counted ≤ 8 per Folio               W! level := cum_xp thresholds, no cap
W! doc-04 XP rows Σ ≤ 40 tenths per character per Folio W! per-Folio XP caps reset on ev.folio_entered
W! level-up → 4 points, ≤ 2 per sub per batch           W! even level → 1 talent point
W! derived stat := canon row + registered modifiers     N! modifier on rows 1,3,11,12,29,32,39,41,42,43
W! Π RowPost(row) ∈ 5000..=20000                        W! modifier order := ascending source id
W! every random draw ∈ {rng.check, rng.progress, rng.combat, rng.alchemy} keyed
N! draw in severing | wounds | buildup | needs | check persistence | dialogue | taming
N! madness phantom ∈ sim state                          N! extra combat draw from avatar_of_fate
W! part pool ∈ 0..=max ∧ untreated tier ≥ 2 caps healing   W! Errata head never severs
W! partless damage → HP only                            W! temperature damage | statuses := doc 03 §9.4 only
W! augment numbers × own potency                         W! Disconnected prosthetic keeps structure
W! installed aug points ≤ region cap 4 ∧ ≤ aug_cap at install
W! innate doc-04 effects: dep none unless stated ∧ src_lvl = d_cur
W! trait.echo ∈ budget(1..=3 terms, benefit ≤ 6, drawback ≥ 1, net ∈ -1..=3)   W! variants B, C: net = net(A)
W! Fortune subs → in-run effects only                    N! input synthesized for a player
N! Death's Door roll while st.unmoored                  W! ghost needs frozen
W! kin, Calling fixed for the run ∧ Origin changes ≤ 1 (Going Native)
N! respec of creation points or training               W! affliction form := doc 06 summon.poly.beast
W! satiation ∈ 0..=2400 ∧ breath ∈ 0..=max               W! afflict.red_ink doses ≤ 4
W! Erratum limb severs ⇒ frac(pool before hit) ≤ 2500  W! combat skill XP := min(1500, floor_div(1500 × dmg, norm_mhp))
W! bp.arm_extra_N ∈ {1, 2}                              N! item durability | mount rule in doc 04
N! disguise bluff drawn on rng.check                    W! kin adds no part its plan already has
W! madness phantom seed := pres_hash (Canon §3.8)        W! stamina = 0 ⇒ ¬can_sprint ∧ ¬can_dodge
W! last stand ⇒ solo ∧ Death's Door failed ∧ ≤ 1 per Folio   N! random draw in last stand
N! thirst | fatigue meter                               W! Appearance ∉ sim state ∧ kept by Going Native
W! Second Edition Folio n depth := 25 + 2n              W! CLIMB_ANY ⇔ plan ∈ {body.amorphous, body.insectoid}
```
