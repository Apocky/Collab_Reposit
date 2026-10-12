# Fix log: chapter 08 (Alchemy and Liquids)

As of 2026-10-11. Fixer for `spec/08_ALCHEMY_LIQUIDS.md` against DECISIONS.md, canon-1.11 (already on disk) and `audit/canon_gaps_triage.md`. Edits are made by `apply_edits.py`-style scripts that assert each anchor occurs exactly once; each batch is listed below when done.

Baseline lint (`spec_lint.py --chapter 08`): undefined `item.face.respirator`; hedges `might` ×7 (all inside the ID `alch.eff.fortify_might`, which mirrors canon `attr.might`; no hedge in prose). `--json` multiply_defined lists 08 for `slot.face`, `cat.potion`, `cat.throwable`, `cat.tool`, `cat.container`, `ev.potion_brewed`, `ev.liquid_reaction` (all named in 08's Exports).

## Plan (triage rows that name doc 08)

- G-051: header to canon-1.11; no mounts or durability in 08 (checked: none).
- G-033, G-119 (canon amendments): liquid units, gas concentration, contact 500, respirator rule.
- G-055 (canon amendment): supernatural alchemy dependency.
- G-056, G-057: failed potion and Unmoored via doc 06 `note_contradiction_failure(t)`.
- G-091: consumable self-damage entry (Canon §10.6 row).
- G-096: alchemical area hits draw and ignore the crit roll (`NO_CRIT`).
- G-021 (canon-1.11 §3.3): fixed `seq` allocation for `rng.alchemy`; potion failure roll at doc 05 §1.4 `seq` 0.
- G-024: explosions draw from `rng.physics` / `rng.fire`.
- G-036, G-192: `block.tag.destructible`, `mat.tag.acid_soluble`, `mat.tag.fat`.
- G-118, G-120 to G-123, G-142, G-143, G-205: chapter-owned; delete the gap text, keep the rule in the body.
- G-161: `genre.unwritten` has no shops.
- C-02: doc 14 now names 08 IDs directly.
- C-19: `envenom` hook in §10.3 (doc 04 must still edit its talent text).
- C-22: delete gap 18. C-26: drop "aliased"; `apparatus.respirator.mask` is the respirator.
- Exports: stop declaring `cat.*`, `slot.*`, `ev.*`; reference them under Imports.

## Done

### Batch 1 (header, Imports, §1.2, §1.3)
- Header: "Conforms to canon-1.11" (G-051).
- Imports: §3.2–§3.4 row names the fixed `rng.alchemy` allocation, doc 05 §1.4 slots, and `rng.physics`/`rng.fire` for queued explosions (G-021, G-024); §4.3/§4.4 row uses `block.tag.destructible` and liquid units / gas concentration (G-036, G-033); §5.3–§5.11 row states the canon-1.11 dependency rule and the failed potion (G-055, G-056); §10 row adds the consumable self-damage entry (G-091); §12 row names the working respirator and both contact thresholds (G-119); §13 row lists the `cat.*` and `slot.face` IDs as canon/doc 11 references and the two `ev.` IDs with doc 03 payloads (C-26 task note: cat./slot. never declared here); docs row adds doc 06's `note_contradiction_failure(t)` (G-057).
- §1.2: `LIQ_UNIT_ML`, `GAS_FULL_CONC`, `GAS_CONTACT_CONC` cite Canon §4.4 and §12.1 (G-033, G-119).
- §1.3: potion failure roll is doc 05 §1.4's `seq` 0 action-start draw; thrown alchemical area hits draw one crit each and ignore it (`NO_CRIT`, G-096); queued explosions draw from `rng.physics`/`rng.fire` (G-024); new `rng.alchemy` seq table (0 brew/bomb variance, 1 seed brew, 2 transmuter, 3 wellspring) replacing "the action seq" (G-021, canon-1.11 §3.3).

### Batch 2 (§2, §4, §5, §7)
- §2.3 spill cites Canon §4.4 (G-033). §2.5 payload `potency_bp` is the potency of the use action that created it (one evaluation per use action, see §9.9 in batch 3). §2.6 example: splash crit drawn and ignored (G-096).
- §4.2 route note, §4.3 and §5.1: a working respirator blocks every gas contact row, `inhaled` and `skin` (canon-1.11 §12.1, G-119); purify blocks `inhaled` rows only; the filter drains on any gas whose row it blocks. Earlier text blocked inhaled rows only; recorded as Canon gap 1 for owner confirmation.
- §4.3: `apparatus.respirator.mask` is the standard respirator; doc 11 §3.4 keeps loot records on the three IDs; "aliased" and `item.face.respirator` removed (C-26, G-143). Added: the filter is a refillable charge, not durability; vessels, apparatus and respirators never wear (DECISIONS 65, C-31).
- §5.3 `alch.rx.acid_block` and §5.6 nitro: `block.tag.destructible`, `mat.tag.acid_soluble`; nitro block breaks and ignition draw `rng.physics`/`rng.fire` (G-036, G-192, G-024).
- §7: Dep text restated from canon-1.11 §5.3 (supernatural alchemical effects follow the origin aether axiom, holy/unholy/curse `mythos`; `inspire` keeps `verve` as the one cinematic effect) (G-055). Damage kind: thrown splash/bomb hits are `NO_CRIT` area hits that draw and ignore one crit each (G-096); drunk/injected damage cites Canon §10.6's consumable row (G-091). `purify` note: inhaled-only immunity.

### Batch 3 (§9 to §15)
- §9.5, §12.5, §15.2, §15.4: `rng.alchemy` keys use the fixed `seq` 0, 3, 1, 2 of §1.3 instead of "start seq" (canon-1.11 §3.3, G-021). §9.7: the brew starts from doc 02's `Brew` intent on its logged tick (canon-1.11 §2.4, G-016); no action seq stored.
- §9.9: one Contradiction evaluation and at most one failure roll per use action, at doc 05 §1.4 `seq` 0 (the old "payload creation" activation had no allocated slot); "Failed potion" defined in Canon §5.5's words (consumed; only toxicity when drunk or injected, or the plain base-liquid spill when thrown; a failed coat or vaporizer load applies nothing) (G-056); every failed potion calls doc 06 §3.9 `note_contradiction_failure(t)` (G-057); the World Laws and seam-zone rules of old gap 1 moved into the body.
- §10.2, §13.2 and its worked example: `NO_CRIT`, crit drawn and ignored (G-096).
- §10.3 and §10.6: doc 04 `talent.shade.envenom` hook: +5 coat charges and `coat_bp` 10000 instead of 5000, stored on the coat (C-19). Doc 04 still has to restate the talent (cross-chapter request).
- §13.2: salt and smoke bombs in every shop and every genre's loot pool; `genre.unwritten` has no shops and gets doc 11's counter caches (G-161).
- §15.3 `alch.tx.lamp_oil`: `mat.tag.fat` (G-192).

### Batch 4 (Canon gaps, Exports, Interface assumptions)
- Canon gaps: all 20 old gaps deleted as resolved (gap 1 G-051; 2 G-033; 3 G-119; 4 G-120; 5 G-055; 6, 11, 16, 19 G-205; 7 G-142; 8 G-143 and C-26; 9 G-121; 10 G-056; 12 G-122; 13 G-091; 14 G-096; 15 G-123; 17 C-19; 18 C-22; 20 G-118). Their rules already stand in the body (§2.3, §3.1, §3.2, §3.3, §5.2, §7, §9.7, §9.9, §11.2, §12.6). One open item: respirator scope (canon-1.11 §12.1 blocks every gas row; the old gap asked for inhaled rows only).
- Exports: the sub-namespace list and the event and instance bullets no longer backtick `cat.potion`, `cat.throwable`, `cat.tool`, `cat.container`, `slot.face`, `ev.potion_brewed`, `ev.liquid_reaction`; a sentence points to Imports, where they are listed as canon/doc 11/doc 03 references (task rule: cat./slot. never declared; ev. is doc 03's). Coat record gains `coat_bp` (C-19 hook).
- Interface assumptions: doc 03 (working-respirator skip, `rng.physics`/`rng.fire`, `is_errata_placed`); doc 04 (`envenom` per C-19, pending in doc 04 §7.2); doc 05 (`NO_CRIT`, `seq` 0 failure roll); doc 06 (`note_contradiction_failure`); doc 07 (`mat.tag.*`, `block.tag.destructible`); doc 11 (salt/smoke in every shop and loot pool; respirator loot records on `apparatus.respirator.*`, no alias, C-26); doc 12 (`st.unmoored`); doc 13 (C-22 resolved, `emit_gas_volume`); doc 14 (C-02 resolved: names 08 IDs directly).

### Batch 5 (acceptance tests, invariants, follow-ups)
- Tests: 15 adds a skin-gas case for the working respirator (G-119); 18 names `seq` 0; 29 and 50 say the crit draw is ignored (`NO_CRIT`, G-096). New tests 56 (envenom, C-19), 57 (failed potion and Unmoored, G-056/G-057), 58 (fixed `rng.alchemy` seq, G-021), 59 (one Contradiction evaluation per use action).
- Invariants: dependency rule (G-055), respirator scope (G-119), `NO_CRIT` (G-096), `block.tag.destructible` (G-036), seq table, failed potion, envenom, no wear or durability (C-31), invader blood.
- §3.3/§3.4: an invader (`fac.editors`, `rank.hunter`) does not bleed red ink and leaves no red-ink death spill, so red-ink unwriting never erases placeables on an invader's behalf (canon-1.11 §11.6, §17.9; G-113).
- §11.2: overdose damage has no hit part (G-083).
- §1.3: the "no random numbers" bullet points to the explosion exception (G-024).

### Lint after
- `spec_lint.py --chapter 08`: no undefined IDs, no foreign declarations, no retired IDs, no JSON issues; hedges `might` ×7, all inside `alch.eff.fortify_might` (mirrors canon `attr.might`; not prose). `--json`: 08 no longer appears in multiply_defined.

## Requests for other chapters
- Doc 04: C-19 `envenom` is already applied on disk (its talent text and test 54 match doc 08 §10.3). Remaining: its Imports row for 08 cites §4.5 for respirators; it should cite §4.3.
- Doc 09 §11 (disguise masks): "respirators are not masks (doc 08 §4.3)", not §4.5.
- Doc 05: none needed; doc 08 now matches its `NO_CRIT` rule.
- Canon owner: confirm the respirator scope of §12.1 (every gas row vs inhaled rows only); doc 08 Canon gap 1.
- Doc 03 §6.10: add "A body whose `slot.face` holds a working doc 08 respirator (`filter > 0`, doc 08 §4.3) skips every gas row, the Canon miasma buildup and steam DoT included, and doc 08 vapor exposure (Canon §12.1); `inhaled` gas rows also skip a body with doc 08 purify immunity (doc 08 §5.1)."
- Doc 05 §9.3 bullet and §9.4 pseudocode: "Doc 08 bombs (`alch.bomb.*`) and thrown potion splashes (doc 08 §10.2) are area hits with `NO_CRIT`" and "(+ NO_CRIT for doc 08 bombs and potion splashes)", so both chapters name the same hits.

Status: all planned items done. Final lint line is in the structured summary.

## Verification

As of 2026-10-11. Adversarial verifier pass over `spec/08_ALCHEMY_LIQUIDS.md` after the fixer's batches 1 to 5. Edits were made by one Python script that asserts each anchor occurs exactly once; nothing was committed.

### Check 1: triage rows that name chapter 08

- **G-033** (liquid units, gas concentration): applied. §1.2 "`LIQ_UNIT_ML` 1,000 … Canon §4.4 grid unit of 1 litre"; "`GAS_FULL_CONC` 10,000 … 1,000,000 ml of gas"; §2.7 `place_gas_ml` "100 ml of gas is concentration 1". Pass.
- **G-051** (header, no mounts, no durability): header reads "Conforms to canon-1.11"; no mount text; §4.3 "respirators, like every vessel and apparatus in this chapter, never wear (Canon §13.9, DECISIONS 65)". Pass.
- **G-055** (supernatural alchemy dependency): §7 "`aether` for supernatural alchemical effects, whose dependency is `{origin_genre.aether_axiom}`, or `{arcana}` …; `mythos` for the holy, unholy and curse effects". Matches canon-1.11 §5.3 word for word in substance. Pass.
- **G-056** (failed potion): §9.9 "the potion is consumed, and only its toxicity (drunk or injected, §11.1) or its plain base-liquid spill (thrown, §6.3) applies". Pass.
- **G-057** (shared Unmoored counter): §9.9 "Each failed potion calls doc 06 §3.9's `note_contradiction_failure(t)` for the user once". Doc 06 line 1885 and doc 12 §24.3 agree. Pass.
- **G-091** (consumable self-damage entry): §7 "enters Canon §10.5 at step 3 with `D2 = B` and skips steps 1 and 2 … Canon §10.6's row". Canon §10.6 has the row. Pass.
- **G-096** (crit drawn and ignored): §1.3, §2.6, §7, §10.2, §13.2, tests 29 and 50. Doc 05 §1.4 `16 + i` slot agrees. Pass.
- **G-118** (overdose cadence): §11.2 "`od_count % 5 == 1`" gives the first needs tick at the cap and every fifth consecutive one. Pass.
- **G-119** (contact 500, respirator): §1.2 `GAS_CONTACT_CONC` 500; §4.3 "blocks every gas contact row, `inhaled` and `skin` alike". **Miss fixed:** the fixer kept a Canon gap asking the owner to confirm the scope, although canon-1.11 §12.1 states it and the triage decision is final (G-119 is not among the §1 rulings the owner may overrule); doc 03 §5.20 already implements "no gas contact for any contact row". Gap deleted; Canon gaps now reads "None open". **Miss fixed:** §4.3's breath bullet now names the 500 threshold, matching test 15.
- **G-120 to G-123, G-142, G-143, G-205**: the rules stand in §5.2 ("1 cell per unit"), §3.3 ("Blood has no contact row"), §2.3 (one spill cell), §3.1 (no thermal freeze for lava), §4.3 and the Exports. Old gap text is gone. Pass.
- **G-021, G-024, G-036, G-161, G-192, G-113, G-083** (rows the fixer applied that do not name 08 in Sources): §1.3 seq table; "`rng.physics` … `rng.fire`"; `block.tag.destructible`; "`genre.unwritten`, which has no shops, gets doc 11's counter caches"; `mat.tag.acid_soluble`, `mat.tag.fat`; invader blood; "it has no hit part". Pass.
- **G-109** (not applied by the fixer): canon-1.11 §17.3a makes `counter.disruptor` last "exactly 180 ticks, unscaled by §11.4". **Miss fixed:** §13.2's `alch.bomb.salt` row now says so.
- **G-197 / C-29** (Codex means only the pass): **miss fixed.** §8.5 said "The Margin codex records discovered pairs account-wide". No chapter owns such a record, the Index (doc 10 §12, doc 14 §5.5) holds item templates only, and "codex" is the pass. Replaced with "Nothing records ingredient effects across runs (DECISIONS 34) … the Index holds item templates only" (listed as unresolved in case the owner wants a hub page).
- **C-02**: doc 14 §2.4 and its Interface row for 08 name `alch.potion.*`, `alch.bomb.firebomb`, `apparatus.vessel.flask`, `apparatus.mortar`, `apparatus.alembic`. Pass.
- **C-19**: §10.3 "`charges = mul_bp(5 + floor_div(skill.alchemy, 20) + 5 × [envenom], coat_charges_bp)` … `coat_bp` (5000, or 10000 with envenom)". Doc 04 test 54 agrees (10 charges at Alchemy 0). Pass.
- **C-22**: gap 18 deleted; Interface doc 13 "`elite.miasmic`'s `emit_gas_volume(cell, gas.miasma, 1)`", which doc 13 line 780 states. Pass.
- **C-26**: no "alias" or `item.face.respirator` left; §4.3 "`apparatus.respirator.mask` is the standard respirator"; doc 11 §3.4 holds the loot records with values 15, 40, 90. Pass.
- **§4.1 stale gaps**: none listed for 08.

### Check 2: DECISIONS rows

- 16 (limited building): §8.1 "resource nodes are interactable props, never dug terrain"; vaporizer "never a placed block"; cask "carried, never placed". Pass.
- 21 (alchemy kept), 34 (re-learned every run): §8.5 per-run `ing_known`; the "Margin codex" sentence fixed above.
- 26 (softer Erasure): no 5400 or repeat spawn; §3.4 hunters other than an invader deposit 5 units. Pass.
- 15 (invasions): §3.3, §3.4, invariant "N! invader blood_liq = liq.red_ink". Pass.
- 37 (micro-voxels): §1.2 capacity rule, §5.3 "applies only when the cell is air" (test 54 slab case). Pass.
- 55 (mounts), 65 (durability), 38, 42, 46, 60, 69, 28, 33, 56: no conflicting text; `echo.mother_ilse` matches doc 14 (+1 dose on 2+, decay 300). Pass.
- 77 to 88: none contradicted. 84 (crippled-only severing) does not touch §3.3's severing spill.

### Check 3: chapter brief

- C-22, C-26: pass (above).
- `cat.*`, `slot.*`: Exports declares none; it says "this chapter references them and declares none"; Imports §13 row lists them as canon and doc 11 IDs. Pass.
- Triage amendments (dependency, failed potion and Unmoored, contact 500, liquid units and concentration): pass (above).

### Check 4: leftover grep

Hits for mount, ride, wear, breaks and plot were all benign ("ride on a puddle", "mountains", "never wear", "blocks break only when tagged", "breaks the Morrowind loop"). No hit for saddle, cavalry, durability (outside the no-durability rule), repair, broken, Delving, voice, VO, 24.99, Item Codex, Crowns, Redactor, `margin.plot`, building plot or 5400. "Margin codex" (§8.5) was the one real leftover; fixed.

### Check 5: lint

`spec_lint.py --chapter 08`: no undefined IDs, foreign definitions, retired IDs or JSON issues; `might` ×7, all inside `alch.eff.fortify_might` (not prose). `--json` `multiply_defined` is empty overall, so 08 appears nowhere. Unchanged after the edits.

### Check 6: internal consistency

- Every worked example recomputed in Python with canon §13.8 tables: §2.6 (14,115; 6,250; 8,821; 12,388), §4.2 (425), §5.2 (37,164; 42,345), §5.6 (40,000 = 28,000 + 12,000), §9.9 (540), §9.10 apprentice (9002, 3563, 630, 22,000) and master haste (16708; 3798; 12650 → 3162; power 23; 10,436; 22,100 → 13,260 → 9,945; value 93; self-zap 10,300), §9.11 (8,000), §11.1 (11,400; 9,000; 6,750), §11.2 (5,950 under 25,500), §11.3, §13.1 (59, 49, 10 charges, 29 → 32), test 56 (18 charges, 59), §13.2 firebomb (9,315), tests 4, 5, 10, 17, 25, 34, 35, 45, 47. **Miss fixed:** §13.2 said "At Folio 1 the pair deals 6,000 + 10,000" right after a Good firebomb; a Good one deals 6,600, so the sentence now names a Normal firebomb.
- §12.1 and §12.2 toxicity and value columns re-derived from §7: all match. `canon[5]` is haste. Exports count 35 + 13 + 95 + 59 + 29 + 14 + 19 + 33 + 19 = 316; every owned ID used in the body is exported. Tables checked for column counts: all well formed.
- **Miss fixed (§2.7):** `gas_emitter` listed doc 05's `grenade_smoke` (`k = 27`, `runs = 100`) as a caller, but doc 05 §9.3 releases the 27 smoke cells once through doc 03 and its Interface row for 08 says "`emit_gas` for smoke". The caller is removed and Interface doc 05 says the grenade uses no §2.7 emitter. `place_gas` listed doc 13 and `place_gas_ml` listed docs 06, 12 and 13; none of them uses those calls (doc 13 uses `emit_gas_volume`, doc 06 row lists `add_liquid`, `place_gas`, `move_gas`), so the callers columns and Interface doc 06 now match.
- **Miss fixed (Interface doc 03):** §5.1 has doc 03 skip `inhaled` rows for purify immunity, but the doc 03 row asked only for the respirator skip; the purify skip is added (doc 03 §6.10 does not state it yet).
- §7 table: "(§10.9 healing rules)" pointed to a section this chapter does not have; now "Canon §10.9".
- Header "Conforms to canon-1.11": pass. Canon gaps: "None open". Interface assumptions: each row is a current assumption on another chapter.

### Verifier edits

1. §2.7: `place_gas` callers "doc 06"; `place_gas_ml` callers "any gas amount authored in ml"; `gas_emitter` callers "this chapter (vaporizer, §10.4)".
2. Interface doc 05: `grenade_smoke` releases its smoke through doc 03 and uses no §2.7 emitter.
3. Interface doc 06: drops `place_gas_ml`.
4. Interface doc 03: adds the purify `inhaled` skip.
5. §4.3: breath drains for `gas.miasma` at `GAS_CONTACT_CONC` or more.
6. Canon gaps: respirator-scope gap deleted; "None open".
7. §8.5: "Margin codex" sentence replaced (DECISIONS 34, G-197).
8. §13.2 worked example: "a Normal firebomb and its blast deal 6,000 + 10,000".
9. §13.2 `alch.bomb.salt`: "exactly 180 ticks, unscaled by Canon §11.4" (G-109).
10. §7 `alch.eff.heal` mechanic: "Canon §10.9".

### Requests for other chapters (verifier)

- Withdrawn: "Canon owner: confirm the respirator scope". Canon-1.11 §12.1 settles it.
- Doc 03 §6.10: also skip `inhaled` gas rows and vapor exposure for a bearer with doc 08 purify immunity (doc 08 §5.1); run doc 08 gas emitters at the start of each gas run (doc 08 §2.7). The respirator half of the fixer's request is already on disk (§5.20, §6.10).
- Doc 04 Imports row for 08 and doc 09 §11 (line 1046): cite doc 08 §4.3, not §4.5 (stands from the fixer).
- Doc 05 §9.3 bullet and §9.4 pseudocode: name thrown potion splashes as `NO_CRIT` as well as bombs (stands from the fixer).
- Doc 06 Interface row for 08: the signatures are `add_liquid(c, liq, ml, slot)` and `place_gas(c, gas, conc, slot)` (doc 08 §2.7), with the instigator slot.

Lint after: `spec_lint.py --chapter 08` clean except `might` ×7 inside `alch.eff.fortify_might`; `multiply_defined` empty.
