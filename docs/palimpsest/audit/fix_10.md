# Fix log: chapter 10 (Identification and Appraisal)

Fixer: chapter 10 fixer, 2026-10-11. Target: canon-1.11. Only `spec/10_IDENTIFICATION_APPRAISAL.md` is edited. Edits are applied by Python scripts that assert each anchor occurs exactly once.

## Baseline

- Lint `--chapter 10`: hedges {'might': 1} (the ring ID `item.ring.might` in the §3.5 worked example, not a hedge). `--json`: `cat.wand` multiply defined (00, 10, 11) because doc 10's Exports backticks it.
- Canon is already canon-1.11 on disk (G-021 seq allocation, G-022 object-keyed draws, G-025 stream uses, G-131 `ENH_CAP`, G-184 events registered by doc 03 all applied).
- Found on disk: doc 11 §13.1 stores `value` with the hidden terms (`1000 × eff_enh`, `buc_value_bp` cursed 5000 and blessed consumable 12500, the charges factor) and says doc 10 §6.1 restates it as `value_true`; doc 11 §12.2 prices with one composite `buy_trade`/`sell_trade` and `PriceObs.trade_bp`, with no "unread" or "known cursed" sale factors. Doc 10 §6 still restated a different `value` (600 per stored enhancement, no curse term) plus `worth_true`. Doc 11 owns `value` (Canon §13.9) and the price chain, so doc 10 §6, §5.8 and §10.2 are aligned to it (triage G-138: doc 10 gaps 16-18 are stale).

## Batches

### Batch 1 (header, intro, Imports) — done

- Header: Conforms to canon-1.11 (task).
- Intro: the chapter owns the wand classification rule, not the `cat.wand` ID (Canon §19: `cat.` is doc 11's with canon core values; lint multiply-defined).
- Imports: Canon §3.3 `seq` allocation and object-keyed draws (G-021, G-022), §4.3 no durability (DECISIONS 65), §13.6 pact curses and cursed capsules (DECISIONS 60), §13.7 `ENH_CAP` (G-131); `cat.` ownership; new rows for docs 07 (material `value_bp`, `block.altar`, `block.shrine_lamp`), 09 (sight checks, light, noise), 17/18/19; doc 11 row adds `zaps_total`, the cursed capsule and the composite trade factors; "Codex knowledge" → "Index and Bestiary knowledge" (doc 14 §5.5 heading, G-197).
- "Docs 07, 09, 17, 18 and 19 do not exist yet" replaced: every cited chapter exists.

### Batch 2 (§3.4, §4, §5.9, §5.12) — done

- §3.4: "vessel or mount" → "vessel or setting" (jewelry setting; avoids the retired mount sense, C-30).
- §4 heading "`cat.wand` (owned in full)" → "Wands: classification and type key"; §4.1 states the category ID is Canon §13.1's and this chapter owns only the sorting rule (Canon §19, task).
- §4.2: `arch.ranged.wand` is exported by doc 05 (triage §4.1 stale gap 11, G-139); the "Canon gaps" pointer is removed.
- §5.9: worldgen altars are doc 07's `block.altar` (doc 12 `room.shrine`, `secret.room.shrine`); the Erratum-placed altar is `block.shrine_lamp`, which sells no `appr.svc.*` (G-145, doc 07 §10.1).
- §5.12: per-tick action counter stays below `0x2000`, the range Canon §3.3 reserves (G-021), replacing the doc 06 G8 citation; the object key cites Canon §3.3 object-keyed draws and allocates purpose code 1 (G-022).

### Batch 3 (§6 value model, §5.5, §5.8, §6.4, §10.2, §12.2) — done

- §6.1 rewritten as `value_true`, a restatement of doc 11 §13.1's stored `value` (owner per Canon §13.9; doc 11 says "doc 10 §6.1 restates the same formula as `value_true`", invariant `value == doc 10 value_true`): base → P → integer rarity multiply → quality → material → `(10000 + 1000 × eff_enh)` if H has ENH → `buc_value_bp` (cursed 5000, blessed potion or text 12500) → charges factor if H has CHARGES. `worth_true` is gone. Examples recomputed by script: longsword 725, wand 251, ring 288, potion 52 (triage G-138: doc 10 gaps 16-18 stale).
- §6.2 `value_vendor` = the §6.1 formula on the `ItemView` with defaults (Common; uncursed, or cursed penalty 1 with `OMEN`; charges 0). The `unread` test and the "known cursed" sale factor are removed: doc 11 §12.2 applies one composite trade factor and no knowledge test; a known curse now prices through `buc_value_bp`. Example: longsword unappraised `value_vendor` 166, sells 49; identified 725, sells 217; wand with unknown charges 157.
- §6.3 estimate uses `value_true`; worked example recomputed (u = −6692 verified with a Python port of Canon §3.1): "≈ 420–860", "≈ 210–950", "≈ 570–790", "≈ 520–820", "≈ 670–750"; the narrowest range excludes the visible 669, which the text now says (the old "inside every range" claim no longer holds). Containment sweep (test 11) re-run: 0 failures.
- §5.8 `PriceObs` now stores `rar_mult`, `cond` (value factors after rarity) and doc 11's composite `trade_bp` (doc 11 §12.2 names `PriceObs.trade_bp`); `price_for_base` follows doc 11 §13.1 and §12.2. `candidates` lets a type with no fixed value (the draught, doc 08 §12.2) pass price filtering. Worked example recomputed: trade 13580, price 47; values 10..70 give 14, 19, 23, 31, 39, 47, 55, 71, 80, 96, 104, 105, 112; candidates healing, haste, shielding and the draught.
- §6.4: "≈ 420–860 Crowns", "725 Crowns" (DECISIONS 90 keeps "Crowns"), Ledger example "4 candidates".
- §10.2 restates doc 11's composite `buy_trade` (floor 7500) and `sell_trade` (cap 5500); the selling example is 49 unappraised and 217 identified.
- §12.2: illumination rolls per Canon §13.9 as amended by G-134 (drop or first acquisition); "value, worth" → "value".

### Batch 4 (§7, §8) — done

- §7.1: `roll_curse(rng, it, local_myt, src)` argument order now matches doc 11 §2.3's call `doc10::roll_curse(&mut sub(seed, "loot.buc"), &mut it, local_myt(req), req.source)`; the skip list matches doc 11 (shop stock, Engine faces other than the cursed capsule, Volume rewards, quest rewards).
- §7.3 `curse.brittle` (C-31, DECISIONS 65): the effect stays non-durability (bearer's slash, pierce and blunt `resist[t]` −`m` Bp, unit 500, so −1500 Bp at p = 3, the weight of a stat curse); the in-table durability clause is removed and one sentence after the table records the retirement.
- §7.8 cursed capsules (DECISIONS 60, Canon §13.6): doc 11 §11.2 now has them (face 4, displayed share 900/1000 Bp, 0 with pity armed); text cites it.
- §8 rule 1 and test 39: harmful shares stated per doc 11 §3.5 table (found potions 3000/10000, Reliquary-only potions 2700/9000, texts 1000/10000); doc 11 now meets the 1000 Bp text share, so gap 25 is resolved.

### Batch 5 (Exports, Interface assumptions) — done

- Exports: `cat.wand` no longer backticked there (Canon §19; lint multiply-defined cleared); `worth_true` and `unread` replaced by `value_true`, `value_vendor`, the estimate and `PriceObs`.
- Interface 07: material `value_bp`; durability parenthetical removed (C-31); `block.altar` and `block.shrine_lamp` (present). Interface 11: marked present on disk (roll_curse order, cursed capsule face 4, `curse.pact.miser` on fresh piles, `value` == `value_true`, composite trade with `PriceObs.trade_bp`, `appr.svc.*`, malign shares, `zaps_total`); one remaining ask about §12.2's "unknown enhancement as 0" wording. Interface 12: `block.altar` in `room.shrine` and `secret.room.shrine`. Interface 17: Lucre skin "Crowns" (DECISIONS 90).

### Batch 6 (Canon gaps, Acceptance tests, Invariants) — done

- Canon gaps: all 25 canon-1.10 gaps closed; the section now says "None open at canon-1.11" and maps each closure to its triage row (amendments G-021, G-022, G-025, G-184; already in canon G-135, G-141, G-142; chapter rulings G-047, G-068, G-137, G-140, G-145, G-194, G-197, G-207; stale per §4.1: old gaps 1, 2, 11, 16, 17, 18, 20; plus 23 (C-31), 24 (doc 11 §11.2) and 25 (doc 11 §3.5)).
- Tests: 8 (`value_true` 725, 251, 288, 52), 9 (vendor 166, sells 49/217, wand 157, OMEN ring 81, no other knowledge factor), 10 (estimate strings for V = 725), 19 (trade 13580, price 47, candidates healing, haste, shielding, draught), 23 (`block.shrine_lamp` refuses, `block.altar` performs), 28 (`value`), 33 (`eff_enh`, stored enhancement public), 43 (no durability clause).
- Invariants: `worth_true` → `value_true`; `value_true == doc 11 §13.1 stored value`.
- Durability wording reduced to the single retirement sentence after the §7.3 table (task: no item durability anywhere); no mount text remains.

### Batch 7 (cursed-capsule consistency, Scriptoria) — done

- Every "Engine faces are never cursed" rule now excepts the cursed capsule (doc 11 §11.2 face 4; DECISIONS 60): §7.6 creation row, §11 Curses row, test 27 ("faces 0 to 3 and 5"), invariant "non-capsule face". The OMEN rules for capsules (§2.6, §2.8, §5.2, §5.11, §6.2 pricing, §7.8, §10.3, §11) were checked and stand.
- §5.9 and §10.3: Scriptoria include doc 12's `room.scriptorium` and doc 11's `shop.scribe`, which offers every Scriptorium service (doc 11 §12.1).

### Batch 8 (proofreading pass) — done

- §5.9: the stale sentence about doc 11's "Identify one item" and "Uncurse one item" rows (old gaps 16-17) replaced by "doc 11 §12.3 lists the four paid services at these prices, × `gp.shop_price_bp`, floored at 1".
- §12 intro: doc 14 §5.5 is "Index and Bestiary knowledge"; doc 15 owes "Index" (C-29). §12.3: the Recollection rename cites G-197 instead of doc 14's stale gap 16 (triage §4.1).
- Interface rows: 08 (exotic values present; the draught valued per brew), 14 (Index and Bestiary thresholds), 15 (C-29 wording).
- Diff reviewed hunk by hunk against HEAD: every change is from this log.

Note for the next agent: the session scratchpad is shared by all chapter fixers; another fixer's `apply.py` (targeting doc 15) replaced this fixer's helper once, and the attempted run failed on its first anchor before writing anything. Chapter 10 scripts now live in `scratchpad/fix10_private/` with a target-file assertion.

## Lint after

- `python3 tools/spec_lint.py --chapter 10` → `== 10 words=21264; hedges: {'might': 1}`. The one "might" is the ring ID `might` (`item.ring.might`, doc 11) in the §3.5 worked example, not a hedge. No undefined IDs, no foreign-namespace declarations, no retired IDs, no JSON issues.
- `--json`: no multiply-defined ID involves doc 10 (`cat.wand` cleared).

## Requests for other chapters

- **Doc 11** §12.2: replace "unknown enhancement as 0" with "the hidden part of `eff_enh` as 0 (stored enhancement is public, doc 10 §2.1)", so `value_vendor` keeps stored `enhancement` in its `1000 × eff_enh` factor exactly as doc 10 §6.2 states.
- **Doc 15** (C-29): §5.5 last bullet "recorded in the account's Item Codex" → "recorded in the account's Index"; the `pass.xp.codex_entry` row "first account-wide Item Codex or Bestiary entry" → "first account-wide Index or Bestiary entry".
- **Doc 17** §4.3 item card row 13: "≈ 160–340 Nobles" → "≈ 160–340 Crowns" (DECISIONS 90 keeps "Crowns"; triage Q9 overruled).
- **Doc 05**: add a doc 10 Interface row (and the hooks in §10.5 terms) for the combat terms doc 10 §7.3 and §7.8 define: `curse.brittle` (slash, pierce and blunt `resist[t]` −`m` Bp before the step-4 clamp), `curse.blunted` and `curse.pact.fury`/`curse.pact.executioner` (additive bucket), `curse.dulled` and `curse.pact.nightmare` (crit chance gear term), `curse.pact.glass` (crit damage gear term), `curse.misfire` (failure chance), `curse.bloodprice`, `curse.backlash`, `curse.pact.thorns` (`dmg.true` hits at step 4), `curse.pact.bulwark` (poise), `curse.pact.bloodthirst` (heals); doc 05 currently names no `curse.` ID.

## Verification

Adversarial verifier, 2026-10-11. Edits went only into `spec/10_IDENTIFICATION_APPRAISAL.md`, applied by Python scripts that assert each anchor occurs exactly once (`scratchpad/verify10/apply_v10.py` and two follow-up scripts). The pre-verification file is kept as `scratchpad/verify10/ch10_before_verify.md`.

### 1. Triage rows that name chapter 10

| Row | Verdict | Chapter text (or fix) |
|---|---|---|
| G-021 (ruling names doc 10's counter) | applied | §5.12: "seq = that Erratum's per-tick action counter (… always below `0x2000`, the range Canon §3.3 reserves for this chapter's per-tick action counter)" |
| G-022 (10 gap 4) | applied | §5.12: "**Object** key (Canon §3.3 object-keyed draws): folio = `est_key >> 40`, actor = the item's `est_key`, tick = `u64::MAX`, seq = purpose code 1"; §6.3 `keyed_rng("rng.ident", est_key >> 40, est_key, u64::MAX, 1)`, the exact canon form |
| G-025 (10 gap 3) | applied | §5.12 table "Every `rng.ident` draw", one row per use with its draw count |
| G-047 (10 gap 8) | applied | §5.5: "The appraisal system runs inside `sys.needs` (doc 03 §2.2 system 27, …)"; no "35 systems" text remains (grep) |
| G-051 | applied | header "Conforms to canon-1.11"; canon-1.8 and canon-1.10 checked in items 3 and 4 |
| G-068 (10 gap 9) | applied | §5.5 `rate`, §5.6 `anl_bp`, §5.10 `psy_bp` are chapter formulas; no §6.4 row is claimed |
| G-131 (`ENH_CAP`) | applied | Imports: "stored enhancement and `ENH_CAP` (§13.7)"; no `0..=15` cap remains |
| G-134 | applied | §12.2: "illumination rolls only on Legendary-or-better items (Canon §13.9: at the drop, or at first acquisition for shop stock, floor loot, Engine faces and rewards; …)" |
| G-135 (10 gap 7) | applied | §4.3: "`zaps_total` … extends the Canon §13.9 contract"; §11: "`sealed = true`, doc 11 instance field" |
| G-137 (10 gap 15) | applied | §2.1: "Stored `enhancement` is therefore public" |
| G-138 (10 gaps 16-18) | applied | §6.1 restates doc 11 §13.1 as `value_true` (doc 11 on disk says the same); §6.2 "`value_vendor(view)` is the basis of every vendor price …"; §5.9 "Doc 11 §12.3 lists the four paid Scriptorium and vendor services at these prices" |
| G-139 (10 gap 11) | applied | §4.2: "Every wand is the archetype `arch.ranged.wand` (exported by doc 05, listed by doc 11 §3.1)"; doc 05 exports it (checked) |
| G-140 (10 gaps 10, 12, 13) | applied | §7.1 "The blessing chance has no rarity or source factor"; §7.6 "Severing, ghost loss and Redaction end binding (§7.2) but never uncurse"; §7.3 `curse.misfire` "rolled at the Canon §5.5 cadence even when Contradiction gives 0" |
| G-141 (10 gap 6) | applied | §2.1: mundane categories "start and stay at `know.classified` (Canon §13.6)" |
| G-142 (10 gaps 1, 2) | applied | §3.2 and §13: "29 `alch.potion.*` (doc 08)"; doc 08 §12.1–§12.2 has exactly 29 |
| G-145 (10 gap 21) | applied, sharpened | §5.9 now names the unlit twin: "doc 07's `block.shrine_lamp` or its unlit twin `block.shrine_lamp_out` … offers no `appr.svc.*`", and the drop test acts "on a worldgen `block.altar`" (doc 07 tags the snuffed lamp `altar` too, so "an altar block" let the free BUC test run at a placed altar). Test 23 and the doc 07 Interface row follow |
| G-184 (10 gap 14) | applied | §5: "The event uses the doc 03 §14.2 payload"; no `ev.` ID is added; doc 03 §14.2 carries `descriptor`, `flags_new`, `reason` (checked) |
| G-194 (10 gap 19) | applied | other owners' IDs appear only as references; lint: no foreign-namespace definitions |
| G-197 (10 gap 5) | applied | §12 "the **Index**"; §12.3 "Renamed from "Recollection", which Canon §20 and doc 14 §6 give to Echo trials (triage G-197)" |
| G-207 (10 gaps 20, 22) | applied | §5.3 witness "so the result is at most 2 ticks old"; §7.6 Prayer row "the lowest-`uid` cursed equipped item" |
| G-052 / Q9 | overruled by DECISIONS 90 | §6.4 "≈ 420–860 Crowns"; Interface 17 cites DECISIONS 90 |
| C-29 | satisfied on disk; chapter fixed | doc 15 §5.5 and the `pass.xp.codex_entry` row already say "Index", and DESIGN_BRIEF §4 now says "the Index". **Fixed** the stale §12 intro ("which DESIGN_BRIEF §4 calls the item codex; doc 15 is to write "Index"") and the doc 15 Interface row |
| C-30 | applied | no mount text (grep); §3.4 "only the vessel or setting changes"; §2.1 table has 27 categories, no `cat.mount` |
| C-31 | applied | §7.3 `curse.brittle` row: "the bearer's `resist[t]` for `dmg.slash`, `dmg.pierce` and `dmg.blunt` −`m` Bp (Canon §10.2), before the Canon §10.5 step-4 clamp", plus one sentence recording the retirement; doc 07 no longer reads `curse.brittle` (grep) |
| §4.1 stale gaps 1, 2, 11, 16, 17, 18, 20 | **fixed** | the fixer had moved them into a "Stale on disk" bullet instead of deleting them. The Canon gaps section is now one sentence, "None open at canon-1.11", with a pointer to the triage |

### 2. DECISIONS rows

- Rows that cite doc 10: 18 (SPD-style identification, base types visible: §1 rule 1), 34 (appearances re-learned each run: §2.7), 60 (cursed capsules with pact curses, shown openly: §7.8 "shows as cursed before purchase"; §10.3). All agree.
- Changed rows: 15 invasions (§2.5, an invader gets no party knowledge), 16 limited building (§12.5 decorations only; the placed altar is a placed light), 26 softer Erasure (§5.7 "`ERASURE_ONSET` = 720 s"), 55 mounts cut, 60 pacts, 65 no durability (items 3 and 4), 84 crippled-limb severing (§7.2 cites Canon §9.1 and restates no threshold), 90 "Crowns". Rows 28, 33, 37, 38, 42, 46, 56 and 69 have no text in this chapter, and none contradicts them (grep: no voice, narration, price, ending or World Law text). Rows 77 to 88: 80 matches the rename to Cross-Reference, 81 the "every party member present" Index rule, and the rest are outside this chapter. Agree.

### 3. Chapter brief

- `curse.brittle`: a non-durability effect of stat-curse weight (−1500 Bp physical resistance at `p = 3`, −1000 at `p = 2`, test 43). Pass.
- Durability: one retirement sentence after the §7.3 table; no mechanic. Pass. **Fixed** "300 s of wear" in §9, which read as item wear, to "300 s equipped".
- Mounts: 0 occurrences; the `cat.mount` row is gone. Pass.
- Stale gaps 1, 2, 11, 16, 17, 18, 20: **fixed** (item 1).
- `cat.wand`: not backticked in Exports ("the category ID itself is a Canon §13.1 core value"); lint `multiply_defined` is `{}`. Pass.
- Pacts: §7.8 has 16 pacts, `OMEN` on the cursed capsule at creation (§2.6, §7.8, §11, test 41), doc 11 face 4. Pass.

### 4. Leftover grep

- Hits that are not real: `broken_wheel` (implant stamp), "override" (matched `ride`), `5400 + 1800 × k` (familiarity ticks), "Crowns" (DECISIONS 90), the brittle retirement sentence.
- Real hits, fixed: "300 s of wear" (item 3); "item codex" (item 1).
- No hits: mount, mounted, saddle, cavalry, repair, Delving, voice, voiced, VO, 24.99, Item Codex, Redactor, `margin.plot`, building plot.

### 5. Lint

`spec_lint.py --chapter 10` before and after: no missing sections, undefined IDs, foreign-namespace definitions, retired IDs or JSON issues; hedges `{'might': 1}` is the ring ID `might` in §3.5. `--json` `multiply_defined` is `{}`.

### 6. Internal consistency

- Every worked example was recomputed in Python with the canon §3.1 generator (`scratchpad/verify10/check.py`, `shuffle.py`). The values match: `value_true` 725, 251, 288, 52; vendor 166, sells 49 and 217, wand 157, `OMEN` ring 81; `u = −6692` and the five estimate strings, with 674 as the narrowest `lo`; containment sweep 0 failures; `trade_bp` 13580, price 47 and the 13 prices; the service table, 67 and 533 sink, 177 at Grade 5; appraisal 23,600, 1,700, 14 passes, 2,210, 11, 51,600, 52, 57,150, 610, 7,080; Analysis 5000, 8250, 500, rolls 6052 and 2404; sense roll 4498; cursed identify index 3; amnesia B and D; curse chances; `pact_bp` 480; Unread Edge weights; bloodthirst and bloodprice; shuffle seed, 169 calls, amulet permutation, first descriptors and ring assignments; 183 distinct `fnv1a32`; `sort_index` of opal 18 and crimson 9.
- **Fixed** test 9's chain "`180 → 180 → 180 → 162 → 81`", which skipped the material step, to "`70 → 180 → 180 → 180 → 180 → 162 → 81`".
- **Fixed** §3.2 relic launch content "18 `item.trinket.*`" to 19: doc 11 §3.4 lists 19, including `item.trinket.lattice_lens`.
- **Fixed** §2.5 stacking, which restated doc 11 §14.2 wrongly: only damage sources need equal `ilvl`, and potions and texts merge across `ilvl`.
- **Fixed** the sealing sources against doc 11 §2.4 and §9.8: Volume rewards are created sealed, and quest and companion rewards carry `RARITY`. The changes are in §2.4, §2.6, §11 (intro, Curses row, and the closed state machine's creation trigger) and the doc 11 Interface row.
- **Fixed** §5.5: the Appraisal-25 perk was quoted as "appraised items show exact value", but doc 04's text ("an item whose `est_skill >= 25` shows its exact true value") would show `value_true` without `BUC` and so leak the curse. The chapter now states that only the flags make the value exact; doc 04 is asked to align (cross-chapter request).
- **Fixed** effects that had no implementing owner. `curse.greed` and the `curse.pact.miser` boon now act on fresh Lucre piles (doc 11 §9.1, where doc 11 applies the miser), and doc 11 is asked to add greed. `curse.leaden` and the `curse.pact.bulwark` drawback now act on `carried_g` (doc 11 §14.1); the assumption moved from the doc 07 row, since doc 07 computes no carried mass, to the doc 11 row. The doc 04 row now asks for the curse and pact stat, regeneration, healing, satiation, composure and aether terms; doc 04 names no `curse.` ID.
- Imports: the doc 11 row now cites §2.4, §9.1, §9.8 and §14.1.
- Tables: every table row has its header's cell count (script check).
- Exports: every `know.`, `idc.`, `appr.`, `buc.` and `curse.` ID used is exported. Counts match: 11, 7 + 176, 5, 5, 11, 6, 22, 3, 28, 16. `eligible_curses` and `eligible_pacts` per category recounted from §7.3 and §7.8 match §7.1.
- Canon gaps: **fixed** to open items only (none). Interface assumptions: **fixed** by removing the status notes "(present on disk)", "(present, §10.1)" and "(present)", the doc 11 "Still asked" sentence (doc 11 §12.2 already reads "the hidden part of `eff_enh` … as 0"), and the doc 15 C-29 clause (already applied).
- Header: "Conforms to canon-1.11". Pass.

### Status of the fixer's requests to other chapters

- Doc 11 §12.2 wording, doc 15 "Index" (C-29) and doc 17 "Crowns" are already on disk, so they are withdrawn.
- Doc 05 curse hooks: still open, because doc 05 names no `curse.` ID. Carried forward.
