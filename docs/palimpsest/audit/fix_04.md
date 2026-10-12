# Fix log: chapter 04 (Attributes, Skills, Progression and Body)

Chapter fixer for doc 04, moving it to canon-1.11. Sources: DECISIONS.md (rows 1 to 96 and the orchestrator rows; row 84 overrides triage Q5), canon-1.11 on disk, `audit/canon_gaps_triage.md` (rows naming doc 04, §4 conflicts C-19, C-20, C-21, C-30, C-31), `audit/notes_*.md` (character, run, items, ident).

Method: Python batches in the scratchpad; each edit asserts its anchor occurs exactly once, the file is written only when the whole batch passes, and each inserted text is confirmed with grep afterwards.

Lint before: `retired_ids (2): arch.mount.runner, st.mounted`; `hedges: might 2` (both are the canonical ID `attr.might`); `idc.implant` multiply defined (00, 04, 10) because doc 04's Exports named it.

## Plan (status updated after each batch)

- [x] Batch 1: header, scope note, Imports, §2.3 `spacer`, §2.4 `self_repair`, §2.6.1 budget table
- [x] Batch 2: §4.2 checks (`seq`, Deceive on `rng.stealth`), §5.1, §5.2 XP rows, §5.3 perks
- [x] Batch 3: §6.1 hunter row, §6.3 re-run on doc 13's plan (C-21), §14.2 re-run
- [x] Batch 4: §7.2 talents (C-19 `envenom`, `field_repair`, `mount_bond`), §8 severing (DECISIONS 84), §8.4/§8.5 item IDs, §9.4 extra arms (G-130), §10.3, §12.3a, §13 Echo bodies, §15 register
- [x] Batch 5: Exports, Interface assumptions, Canon gaps, Acceptance tests, Invariants
- [x] Lint

## Pre-computed numbers (scratchpad `sim.py`, reproduces doc 13 §11.2 row for row)

- Doc 13 §11.2 plan: 37.3 standard-kill equivalents and 45.5 units per Folio over 25 Folios; combat build 480,822 XP after Folio 24 (level 29), hafted 50 at Folio 10 and 75 at Folio 22.
- Stealth build (kill share 4000, Unseen at positions 1 and 2, 30 tenths on standard Folios, 15 in boss Folios): 453,843 XP, level 29; sneak 50 at F12, blade 50 at F18; on arrival sneak 69, blade 56, security 47.
- Crafting build (kill share 6500, 40 tenths): 452,175 XP, level 29; tinkering 50 at F13, marksmanship 50 at F20; on arrival tinkering 68, smithing 57, marksmanship 54.
- Sensitivity: combat 0 tenths 29; combat 40 tenths 30; stealth Unseen on every standard Folio with 40 tenths 31 (60 tenths: 31, 591,316 XP); stealth kill share 2500 28; crafting 0 tenths 28; combat at 40 standard-kill equivalents per Folio 30.
- §14.2 Netrunner: Folio gains 257, 466, 1432, 1451, 4615, 5518, 4302, 8711; levels 2, 3, 4, 5, 7, 8, 9, 10.

## Changes

### Batch 1 (done): header, scope note, Imports, §2.3, §2.4, §2.6, §2.6.1 (20 edits, all confirmed by grep)

- Header: "Conforms to canon-1.11" (task item 6; triage G-051).
- Scope note: canon-1.8 cut mounts and canon-1.10 removed item durability (DECISIONS 55, 65; C-30, C-31); block HP of placeables and destructible blocks stays.
- Imports, canon table: §3.3 now lists `rng.check` (G-023) with `seq` 0xFFFE and `rng.combat` `seq` 0xFFFF reserved and doc 04's per-tick counters below 0x2000 (G-021); §4.1–§4.3 cite the fall-damage chain (G-034), the move-speed chain with `body_move_bp` (G-035), `block.tag.destructible` (G-036) and no item durability (C-31); §7–§8 cite `norm_mhp` normalization (G-070, DECISIONS 85), the 45-unit anchor (G-071), pacification (G-073) and the ghost/downed co-op split (G-072, DECISIONS 81); §9.1 cites crippled-limb severing (G-080 as overridden by DECISIONS 84) and added-part donors (G-081); §10.5 cites step 3a fae iron (G-086) and step 6c Mercy (G-089); §16–§17 cite kit ownership (G-159), Fenrir and Jörmungandr bodies (canon §16.2), the Erasure hunter's elite kill XP (G-112) and retired mount IDs (§17.7); new §18 row for doc 03 §14.3 registrations (G-184).
- Imports, peer table: dropped the stale "chapter 18 does not exist, chapter 07 still being written" sentence; doc 03 §14.3 registrations; doc 05 natural-weapon skins and `harness`; doc 07 row lists the sections actually read (§3.2–§3.3 material bonus, §8.2 self-repair supplies, §8.3–§8.4, §9, §10.3–§10.4, §11, §13); doc 08 §10.3 coats; doc 09 bluffs on `rng.stealth` seq 1006 (C-20); doc 14 body plans.
- §2.3 `spacer`: the `st.mounted` clause (C-30) is replaced by `RowPost(15, 8500)` while in doc 03's `Air` move mode.
- §2.4 `self_repair`: needs a never-consumed `repair_kit` tool and consumes doc 07 §8.2's `SELF_REPAIR_UNITS` metal unit, refused with `no_supplies` (C-31; matches doc 07 §8.2 and doc 11's toolkit row).
- §2.6: `echo.fenrir` is `body.quadruped` and `echo.jormungandr` `body.serpentine` (canon §16.2).
- §2.6.1: dropped the `crafted_durability` key (C-31; doc 14 re-scored `echo.ada_brassvane` with `sell_price`); replaced the retired `aldric_unsworn` example with `fenrir` (2/1/1) and added `sun_wukong` (3/1/2) and `loki` (2/1/1), recomputed from doc 14 §6.3 (notes_run follow-up; doc 14 Canon gap 22 request).

### Batch 2 (done): §4.2, §5.1, §5.2, §5.3 (22 edits, all confirmed by grep)

- §4.2: `seq` defined as the actor's per-tick check counter from 0, below 0x2000 (G-021). Deceive row: doc 09 draws the disguise bluff on keyed `rng.stealth` seq 1006, never on `rng.check` (G-023, C-20).
- §5.1: anti-grind list drops the mounted row (C-30).
- §5.2: combat-style rows now divide by `norm = norm_mhp` (canon §7.1 as amended by G-070; DECISIONS 85, triage Q6), with a party-of-4 example (153,000 mHP target, `norm` 60,000, still 500 milli-XP); the healing row uses `ally_max`. Removed the item-repair XP row (C-31); salvage pays the item's governing skill, Smithing, Tinkering or Inscription (doc 07 §8.1, §8.3). Added the Tinkering row `min(750, floor_div(750 × dmg, norm))` for turret, drone and coil hits that doc 07 §11.1 and its Interface row for doc 04 cite. Fieldcraft trap row names mines and snares (doc 07 §11.1 ground gadgets). Handling rows drop the mount hit and the "mounted and moving" row (C-30).
- §5.3: Polearm 100 drops "or mounted" (C-30). Armor 25 "armor durability loss × 7500" becomes `RowFlat(23, 10)` (C-31). Security 25 names doc 09's `break_bp`; Security 50 yields `block.trap.spikes` (doc 07 §10.4, doc 09 §13.6). Smithing 25 "repairs restore × 15000" becomes `RowFlat(19, 500)`; Smithing 100 drops "max durability × 20000" and states doc 07's `mat_bonus_bp` 500 for weapons, armor and shields (C-31, doc 07 §3.2–§3.3). Fieldcraft names `block.tag.destructible` (G-036) and "placeable repairs" at 75. Handling 25 (mount summon) becomes a 50 Bp/s out-of-combat regeneration for tamed beasts and hirelings; Handling 100 (`arch.mount.runner` riding) becomes +2000 Bp resistance to every type within caps (C-30; canon-1.8 "Handling covers taming and minion command only").

### Batch 3 (done): §6.1, §6.3, §14.2 (§6.3 replaced as one block, 4 further edits, all confirmed by grep)

- §6.3 re-run on doc 13 §11.2's current plan (C-21, G-071, triage Q7): no rest Folio at position 4, mini-boss at position 3; the harness text states the plan's 37.3 standard-kill equivalents and 45.5 units per Folio and defines `n_build`, kill share and every XP term exactly. Results: combat 480,822 (identical to doc 13 row for row), stealth 453,843, crafting 452,175 after Folio 24, all level 29 (canon §8's anchor; inside 28 ± 3); levels after Folio 10 are 13; hafted 50/75 at Folio 10/22; new per-Folio tables for all three builds; sensitivity rows 28 to 31 recomputed, with the 40-standard-kill-equivalent variant defined exactly as `40 × mul_bp(K, 7000)` per Folio; the cap sentence restated with exact XP.
- §6.1 kill XP table: the hunter row (50000) now names the invader and Colophon hunters; the Erasure hunter pays the elite row (canon §11.6, G-112; matches doc 13 §11.1).
- §14.2 Netrunner example re-run on the same plan: Folio 1 is 257 (kill `mul_bp(73, 6500) = 47`), Folio 4 is no longer a rest Folio, Folio 5 reaches level 7, Folio 8 ends at 26,752 (level 10); skills, training marks and Max HP at level 10 are unchanged.

### Batch 4 (done): §7.2, §8, §9.4, §10.3, §12.3a, §13, §15 (17 edits, all confirmed by grep)

- §7.2 Shade `envenom` (C-19, triage text verbatim): "your coats (doc 08 §10.3) get +5 charges, and the coat's buildup factor 5000 becomes 10000".
- §7.2 Artificer `field_repair` "repair × 15000" (an item repair under C-31) now multiplies placeable repairs (block HP, which stays) by 15000, one more `mul_bp` after doc 07 §10.4's Fieldcraft 75 factor. Request filed for doc 07.
- §7.2 Beastbinder `mount_bond` (mount stamina and Max HP, C-30) is replaced by `talent.beastbinder.kindred_bond`: while a tamed beast of yours is within 8 m, `RowFlat(18, 10)` and composure losses × 8500; one sentence records that it replaces the retired mount talent. §15.4 register row renamed. No other chapter, data file or code cites the old ID (repo-wide grep), so no alias is needed (canon §1.1 requires aliases only for shipped IDs).
- §8.2 `on_part_damage`: an Erratum's limb severs only when `floor_div(pool × 10000, max) <= 2500` before the hit (canon §9.1 as applied in canon-1.11; DECISIONS 84 overrides triage Q5, so there is no "single hit of 50% of the pool" clause). New "Errata limbs" paragraph with a worked example (full arm hit for 20,000 is disabled, not severed; 2,656 mHP crippled arm severs; 2,658 mHP wounded arm is disabled). §8.6 step 3 cites it.
- §8.4 treatments name doc 11's `item.tool.bandage`, `.splint`, `.salve`, `.surgical_kit`; §8.5 names `item.tool.crutch` and states that `BodyMove` is canon §4.2's `body_move_bp` (G-035).
- §9.4 `aug.extra_arm`: adds `bp.arm_extra_1`, then `bp.arm_extra_2`, refused when both exist (G-130, canon §13.2's two extra-hand slots); its share comes from the plan's donor part (G-081). §15.8 states the bound.
- §10.3 `erase` names `block.tag.destructible` (G-036).
- §12.3a: the Redaction trigger is "the last Erratum that is alive and not downed" (G-173, canon §17.9).
- §13: replaced "No launch Echo uses a non-humanoid plan" with Fenrir (`body.quadruped`) and Jörmungandr (`body.serpentine`) (canon §16.2; notes_run follow-up; doc 14 gap 22 request); natural attacks are doc 05's `arch.melee.unarmed` skinned per plan; beastkin's tail is added to a quadruped but never to a serpentine plan that already has one; kit items for missing slots go to the bag (doc 14 §2.4.2); `aug.extra_arm` is refused on the serpentine plan.

### Batch 5 (done): Exports, Interface assumptions, Canon gaps, Acceptance tests, Invariants (Interface and Canon gaps replaced as one block, 12 further edits, all confirmed by grep)

Note: the shared scratchpad root is used by other chapter fixers (another agent's `edit.py` replaced mine at 22:22, pointing at doc 08). Batches 1 to 4 had already run with the doc 04 helper and were re-verified on disk by grep before batch 5; all later scripts live in `scratchpad/fix04/` (`edit04.py`, `sim.py`, `tables.md`, `b5.py`, `b5b.py`, `b6.py`).

- Exports: removed the backticked `idc.implant` from the `aug.` row (doc 10 owns `idc.`, canon §19; it was the cause of the 00/04/10 multiply-defined flag); the Interface row for doc 10 keeps the reference. `bp.arm_extra_N (N from 1)` becomes `bp.arm_extra_1`, `bp.arm_extra_2` (G-130). The `seq` constant line now states doc 04's full allocation: per-tick check counter from 0 below 0x2000, 0xFFFE, 0xFFFF, and `rng.progress` 0, 1 and `2 + i` (G-021), without backticking foreign `rng.` IDs in Exports.
- Interface assumptions rewritten: doc 03 (§14.3 registrations; `Air` mode; `body_move_bp` per canon §4.2; fall chain) with the old "in place of its × 5000 term" request dropped because doc 03 §5.2 already applies `body_move_bp`; doc 05 (natural-weapon skins, crippled-limb rule in the severing branch, `d_armor`; the old "seq below 0xFF00" and "armor-XP line must read" requests dropped because canon §3.3 reserves the values and doc 05 §11.4 now reads doc 04's row); doc 07 (material bonus, placeable repair with `field_repair`, `metal` tag and self-repair supplies, governing skill for salvage, gadget hits; no durability); doc 08 (`envenom` on §10.3 coats, C-19); doc 09 per C-20 ("disguise Deceive checks on `rng.stealth` seq 1006"; the met `gumshoe` cadence and `cat.placeable` requests deleted) plus the lockpick-stack assumption; doc 11 (treatment, crutch, toolkit, lockpick IDs; the three Scriptorium rows now exist); doc 13 (Erasure hunter elite XP; the stale "§13.3 superseded" and "sensitivity row 28 / 28" texts replaced); doc 14 (Echo body plans; the old "doc 14 conflicts, §6.2 governs" text replaced by canon §8 and C-01).
- Canon gaps: all 20 old gaps closed and listed with the triage row and the section that now holds each rule (G-023, G-184, G-066, G-086, G-035, G-034, G-076, G-061, G-077, G-078, G-080 + DECISIONS 84, G-071 + C-21, G-063, G-079, G-058, G-097, G-083, G-081, G-082). One real open item kept: leg loss on plans with more than two legs (canon §9.1's 5000-per-leg line against §4.2's delegation to `body_move_bp` and doc 04's quadruped and insectoid factors), with proposed canon text.
- Acceptance tests: 4 (check `seq` 0 and 1; disguise answers never draw `rng.check`), 6 (party-of-4 normalization), 8 (ghosts and downed share; Erasure hunter 529, invader 882), 9 (level-29 band incl. 446,600, 452,175, 480,822), 10 (new totals, sensitivity `28..=31`, combat matches doc 13 row for row), 19 (Errata crippled-limb severing cases), 37 (Fenrir, Sun Wukong, Loki scores; retired Aldric removed); new 53 (Echo bodies and extra-arm bound), 54 (`envenom` coats), 55 (retired mechanics: `kindred_bond`, `self_repair` supplies, Smithing 25 and Armor 25 replacements).
- Invariants: crippled-limb severing, normalized combat skill XP, `bp.arm_extra_N ∈ {1, 2}`, no durability or mount rule, no bluff on `rng.check`, no duplicate kin part.

### Batch 6 (done): §6.1 pacification owner (2 edits, confirmed)

- §6.1: canon §8 now fixes pacification XP (G-073), so the Pacify row's owner is "Canon (methods: 04, 13)" and it names `ev.entity_pacified`; the lead sentence says so.

### Lint (after batch 6; final result below)

- `python3 tools/spec_lint.py --chapter 04` → `== 04 words=31804; hedges: {'might': 2}`. No undefined IDs, no foreign-namespace definitions, no retired IDs, no JSON issues. The two "might" hits are the canonical ID `attr.might` (§3 heading and §15.1 register row), not hedge words.
- `--json` multiply_defined: no entry involves doc 04 (the `idc.implant` flag is gone).

### Batches 7 to 9 (done, 4 edits, all confirmed)

- §8.1 fall damage: the old Canon gap 5 held the only statement of doc 04's fall-factor order, so the rule moved into §8.1 (canon §4.2 as amended by G-034): `fall_dmg_bp`, then `aug.spring_tendons`, `skill.acrobatics`, `talent.monk.roll_with_it`, `trait.kin.fae.light_step`, `trait.origin.qinggong` in ascending source ID, one `mul_bp` each. The leg split now covers every plan with legs (four quadruped legs for `echo.fenrir`, insectoid walking legs, construct limbs 3 and 4), remainder to the lowest part ID; test 46 is unchanged. Canon gaps closure line points to §8.1.
- §2.5 kits aligned with doc 14 §2.4.2 (doc 14 Canon gap 2 open point (a)): the Alchemist's field alembic is a mortar and an alembic (doc 08 §9.1); the Monk's "cloth armor set" is the light set doc 14 binds.

### Final lint

- `python3 tools/spec_lint.py --chapter 04` → `== 04 words=31888; hedges: {'might': 2}` (the canonical ID `attr.might`, twice). `--json`: no multiply-defined ID involves doc 04.

## Requests for other chapters (not edited here)

- **00 (canon):** §9.1 effects line: "Losing a leg of a two-legged plan multiplies move speed by 5000 and removes sprint, jump and dodge; doc 04 §8.3 and §13 set the factors for plans with more legs." (doc 04 Canon gap 1).
- **05:** add natural-weapon skins of `arch.melee.unarmed` for non-humanoid Echoes: `body.quadruped` bite, claw and pounce (`echo.fenrir`); `body.serpentine` bite, constrict and spit (`echo.jormungandr`) (canon §16.2, doc 04 §13, doc 14 §6.3).
- **07:** §10.4 placeable repair formula: after the Fieldcraft 75 factor, multiply once more by 15000 when the repairer has `talent.artificer.field_repair` (doc 04 §7.2), and add `field_repair` to the Interface row for doc 04's Artificer talents. In that Interface row, the Tinkering gadget-hit row reads `min(750, floor_div(750 × dmg, norm))` with canon §7.1's `norm_mhp`, not `max`.
- **08:** delete Canon gap 17 (`envenom`); doc 04 §7.2 now reads "your coats (doc 08 §10.3) get +5 charges, and the coat's buildup factor 5000 becomes 10000" (C-19).
- **09:** §13.3 lockpicking: replace "lockpicks durability −1 (doc 07)" with "one `item.tool.lockpicks` unit is consumed from the stack" (C-31; picks are a consumable stack, doc 11 stack 10), keeping `break_bp = 2500` and doc 04 Security 25's × 5000. In §11.4 and G9, drop "Canon §3.3 lists no `rng.check`": canon-1.11 lists `rng.check`, and the bluff stays on `rng.stealth` seq 1006 because canon §3.3 assigns disguise challenges there (G-023).
- **14:** Canon gap 22's requests to doc 04 are done (doc 04 §13 names Fenrir and Jörmungandr; §2.6.1 scores `fenrir` 2/1/1, `sun_wukong` 3/1/2, `loki` 2/1/1); close them. Canon gap 2 open point (a) is adopted in doc 04 §2.5 (field alembic = mortar + alembic; Monk light set).

## Verification

Adversarial verifier pass on the chapter as the fixer left it, against canon-1.11 on disk, `audit/canon_gaps_triage.md`, DECISIONS.md rows 1 to 108 (rows 97 to 108 were added after the fixer's pass) and the current text of the peer chapters. Edits were made with Python scripts that assert each anchor occurs exactly once (`scratchpad/verify04/edit_v1.py`, `edit_v2.py`, plus two one-anchor scripts). Numbers were recomputed with an independent script (`scratchpad/verify04/v.py` to `v4.py`) that parses doc 13 §11.2's table from disk rather than reusing the fixer's `sim.py`.

### Check 1: triage rows naming doc 04

| Row | Verdict | Chapter text that satisfies it (after this pass) |
|---|---|---|
| G-021 | applied | §4.2: "`seq` is the actor's check counter on this tick: 0 for its first check, +1 for each further check on the same tick, always below 0x2000 (Canon §3.3)"; §12.1 and §12.4 use 0xFFFF and 0xFFFE; Exports constants list the full allocation |
| G-023 | applied | Imports §3.3 row; §4.2 Deceive row: "doc 09 makes its one success draw on keyed `rng.stealth` seq 1006, never on `rng.check` (Canon §3.3)" |
| G-034 | applied | §8.1: "**Fall damage** (Canon §4.2) is multiplied by the local genre's `fall_dmg_bp`, then by each doc-04 fall factor in ascending source content-ID order, one `mul_bp` each" |
| G-035 | applied | §8.5: "It is Canon §4.2's `body_move_bp`, applied after row 11 and before status multipliers." |
| G-051 | applied | header "Conforms to canon-1.11"; scope note records canon-1.8 and canon-1.10 |
| G-058 | **fixed** | §12.1 said "`dd_used` stays false"; the ruling is "`dd_used` unchanged". Now "no draw is made and `dd_used` is unchanged"; test 32 likewise. `avatar_of_fate`: "It is Fortune-derived, so `st.unmoored` suspends it" |
| G-061 | applied | §1.2 `TolAdd` row: "kin traits, Origin traits, talents and Marginalia powers; augments add to `gear_tol`; World Law grants (Canon §5.10) add separately" |
| G-063 | applied | §2.8 steps 2 and 3 (Origin trait replaced; "Creation history stays") |
| G-066 | applied | §1.2 registry (`RowFlat` at the gear position, `RowPost` after the clamp, before statuses) |
| G-071, C-21 | applied, recomputed | §6.3 harness: "37.3 standard-kill equivalents per Folio ... 45.5 units per Folio, on Canon §8's 45-unit anchor". Independent recomputation from doc 13's table: mean mSKE 37.324, mean units 45.46, mean enemies 37.6 |
| G-072, C-01 | applied | §6.2: "each party member present in the Folio, downed members and ghosts included ... Dormant and Departed slots ... gain nothing" |
| G-073 | applied | §6.1 Pacify row, owner "Canon (methods: 04, 13)", `ev.entity_pacified` |
| G-076 | applied | §2.4 `kin.human` creation-only bonus; `fuel_cell` "displayed as Fuel with the same range, decay and thresholds" |
| G-077 | applied | §9.2 "Each region holds augments totalling at most 4 points." |
| G-078 | **fixed** | §13 said only "replace Canon §4.2's humanoid values"; the ruling replaces §4.1 to §4.2. Now "For these plans the capsule and base speeds below replace Canon §4.1's humanoid body and §4.2's base speeds" |
| G-079 | applied | §2.6 `trait.echo.<echo_suffix>`, authored by doc 14 |
| G-080 + DECISIONS 84 | applied | §8.2 code `(!e.is_erratum() \|\| crippled_before)` with `crippled_before = floor_div(s.pool * 10000, s.max) <= 2500`; "Errata limbs" paragraph: "A healthy, bruised or wounded limb never severs, however large the hit". No 50%-of-pool clause anywhere (grep). Worked example rechecked: 2,656/10,625 gives `frac` 2499, 2,658 gives 2501 |
| G-081 | **fixed** | §15.8 donor column was right, but §9.4 `aug.tail` and `aug.wings` still took their share "from torso", which a serpentine body lacks. Both now read "from the plan's donor part, §15.8" |
| G-082, G-083, G-097 | applied | §15.8 `bp.horn` and `bp.weakpoint_N` rows; §8.1 "Partless damage"; §12.5 `st.charmed` row |
| G-086 | applied | §2.4 `iron_bane` at step 3a; Imports §10.5 row |
| G-112 | applied | §6.1 hunter row: "the Erasure hunter pays the elite row (Canon §11.6)"; test 8 (529 and 882) |
| G-130 | applied | §9.4 `aug.extra_arm`; §15.8 closing sentence "no body has a third" |
| G-159 | applied | Imports §16 row; §2.5 "Doc 14 §2.4 binds each role below to an exact template ... this list fixes the roles" |
| G-173 | applied | §12.3a "the last Erratum that is alive and not downed" |
| G-184 | applied | Imports §18 row |
| C-19 | applied verbatim | §7.2: "`envenom` your coats (doc 08 §10.3) get +5 charges, and the coat's buildup factor 5000 becomes 10000"; doc 08 §10.3 matches. Test 54 now names a non-oil base, since an oil base gives × 15000 charges |
| C-20 | applied | Interface 09 row: "disguise Deceive checks on `rng.stealth` seq 1006"; the `gumshoe` cadence and `cat.placeable` requests are gone |
| C-30 | applied | no `st.mounted`, `arch.mount.*` or other retired ID (lint); `kindred_bond` replaces `mount_bond` |
| C-31 | **fixed (one miss)** | Smithing 25, Smithing 100, Armor 25, the repair XP row and `field_repair` were done. Missed: §5.3 Security 25 still multiplied "lockpick break chance (doc 09 `break_bp`)" and the Interface 09 row had a break that "spends one pick from the stack". Doc 09 §13.2 has retired the pick break: lockpicks are never consumed and Security 25 multiplies `slip_bp`. Both now follow doc 09; test 55 checks it |
| §4.1 stale gaps | none name doc 04 | — |

### Check 2: DECISIONS rows landing in doc 04 or in canon sections it elaborates

- Rows 25, 27, 28, 49, 62, 81, 84, 85: agree (§11.1 2400 satiation at 1 per s is 40 minutes; §2.1; §13; §6.1; §6.2; §8.2; §5.2). Row 49 was only half there: §12.4 now says phantoms render exactly as the native template and are client-identifiable, so `access.mark_hallucinations` (forced on under Mercy) can mark them, as doc 17 and doc 18 expect.
- Row 97 (light creator presets) was missing: §2.1 gains a Cosmetics bullet (12 presets per kin, palette and marking colors, no slider editor); Interface rows 17 and 18 updated.
- Row 102 (Second Edition carry-over) was missing: new §6.7 carries progression over, lets per-Folio counters reset as usual, and takes each Folio's depth from doc 14 (row 106 is not yet in canon §13.8, so no number is stated).
- Changed rows: invasions (§12.3a, §12.5) agree; limited building agrees (`demolition` now says "destructible blocks"); softer Erasure agrees (hunter elite XP, red ink threshold, no 5400 repeat); Echo cast agrees; hybrid glyph learning has no conflict (kits name glyph tomes only); micro-voxels, procedural ending, USD 19.99, 12+ enemies and World Laws have no doc 04 surface; text-only narration agrees (no voiced lines; Whispers is a captioned audio effect); mounts cut and no durability agree after the Security 25 fix; pact curses agree (§9.3 blind installs keep curse state, doc 10).
- Rows 77 to 88: 77, 78, 82, 83, 86 and 88 have no doc 04 surface; 79 and 80 agree with §2.1's unlock set and owned Echoes; 81, 84 and 85 agree; 87 agrees once §12.4 uses `pres_hash` (below).

### Check 3: chapter brief

- C-19, C-20, C-21: as above. Q1, DECISIONS 84, Q6 (§5.2 `norm_mhp`, party-of-4 example 153,000 / 60,000 / 500 rechecked) and Q7 (level 29 on arrival): applied.
- Fieldcraft replaces Delving: only the scope note's history sentence names Delving.
- Durability (4 sites) and mounts (`arch.mount.runner`, `st.mounted`): gone, plus the Security 25 miss above.
- `idc.*`: referenced only (§9.1 source column, Interface 10 row); not in Exports.
- Echo bodies: Fenrir `body.quadruped`, Jörmungandr `body.serpentine` in §2.6, §13, Imports, Interface 14.
- `rng.check`: cited as canon-1.11's keyed stream (Imports §3.3).

### Check 4: leftover grep

`mount`, `durab`, `wear`, `Delving` appear only in the scope note's retirement sentence and the invariant "N! item durability | mount rule". `repair` hits are the body heal `self_repair` and placeable repair (block HP stays), both correct. `broken`/`breaks` are channels and line of sight. `5400` hits are talent cooldowns. `Redactor` names the invader (canon §17.9), never the Volume II boss. No hits for ride, saddle, cavalry, voice, voiced, VO, 24.99, Item Codex, Crowns, `margin.plot`, building plot.

### Check 5: lint

`spec_lint.py --chapter 04`: no missing sections, no undefined or foreign-namespace IDs, no retired IDs, no JSON issues; `hedges: might 2` are the ID `attr.might`. `--json` multiply_defined: empty overall and nothing involving doc 04. An extra scan found no doc-04-namespace ID used anywhere that doc 04 does not define, and no foreign ID in doc 04 that its owner does not define.

### Check 6: internal consistency

- Recomputed with the independent script: combat table row for row equal to doc 13 §11.2 (480,822, level 29, hafted 50/75 at Folios 10/22, level 13 after Folio 10); stealth 453,843 and crafting 452,175 tables row for row; majors on arrival (sneak 69, blade 56, security 47; tinkering 68, smithing 57, marksmanship 54; hafted 77); first-50 Folios (sneak 12, blade 18, tinkering 13, marksmanship 20); all six sensitivity levels (29, 30, 31, 28, 28, 30); 569,486, 591,316, 3,884 and 21,830. §14.2 Netrunner gains, levels, skills and training marks (LOG +7, AGI +4, SLT, PRC and HRG +2, GUI +1) reproduce. Kill-XP table, respec, cure, unarmed, beast form, part pools, treatment and Redacted test values rechecked.
- Every table in the chapter has a separator row and a constant column count (scripted check).
- Canon §3.8 (canon-1.11) was violated: §12.4 seeded phantoms with its own `mix64` expression. Now `pres_hash("madness.phantom", run_seed ^ entity_id, floor_div(tick, 600))`; test 33 and an invariant added.
- Peer interface rows that doc 04 had stopped matching: doc 06 §7.3 now equals §5.3 (the "except Restoration 100" note was stale); doc 14 has adopted C-01 (the "asks doc 14" note was stale); doc 08 §15.4 owns the `transmuter` pool and its `seq` 2 draw ("a random type of the same rarity" contradicted it); doc 07 §3.3 gives `reinforced` to armor and shields; docs 03 and 16 call `can_sprint`, `can_jump` and `can_dodge`, which doc 04 never defined. §11.5 now defines them, Exports lists them, and test 56 covers them.
- Exports: every owned ID used resolves (lint and the extra scan); functions list completed. Canon gaps: one open item (leg loss on plans with more than two legs), still open because canon §9.1 is unchanged; the closure sentence is the house pattern. Interface assumptions hold no stale request after this pass.

### Status of the fixer's requests to other chapters

- 00 (leg-loss text in §9.1): still open.
- 05 (natural-weapon skins): done on disk (doc 05 §3.2 text on `arch.melee.unarmed`).
- 07 (`field_repair` in §10.4; Tinkering row with `norm`): done on disk.
- 08 (delete gap 17): done on disk.
- 09: superseded. Doc 09 §13.2 keeps lockpicks unconsumed and replaced the pick break with a slip, so the request to spend one pick per break is withdrawn; doc 04 now follows doc 09.
- 14 (gap 22; gap 2 point (a)): done on disk; doc 14's Canon gaps no longer name doc 04.

### Verifier requests for other chapters (not edited here)

- **00 (canon):** §9.1 leg-loss line as proposed in doc 04 Canon gap 1 (carried over). When DECISIONS 106 lands, §13.8 should state the Second Edition depth so doc 04 §6.7 and doc 14 can cite it.
- **11:** `loot.pool.implants_2` asks for "the 8 implant-form `aug.*` that doc 04 tags `implants_2`", but doc 04 has no such tag. Doc 11 owns its pools, so it should list the 8 `aug.*` IDs in the pool row itself.
- **14:** §2.4.2 Monk row, "doc 04 §2.5 role list" column: "hand wraps (unarmed archetype), light armor set"; drop the sentence "Doc 04's "cloth armor set" is the light set."
- **18:** Interface row for doc 04: drop "(§12.4 still seeds them with its own `mix64` expression)"; doc 04 §12.4 now uses `pres_hash("madness.phantom", run_seed ^ entity_id, floor_div(tick, 600))`. Author the 12 appearance presets per kin that doc 04 §2.1 and doc 17 cite (DECISIONS 97).

### Left open by the verifier

- Doc 01's Interface row asks every Calling kit to list its ally-targeting verbs (`pillar.party_of_errata.t2`). No triage or DECISIONS row covers it, and it needs a design pass on the 13 role lists.
- The Shade kit's "5 lockpicks" quantity is left over from the durability rules: doc 09 never consumes picks, so one is enough. It is left unchanged because doc 14 §2.4.2 prices the kit with it. This is a kit-balance call.
