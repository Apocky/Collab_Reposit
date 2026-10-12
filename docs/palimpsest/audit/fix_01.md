# Fix log: doc 01 (Vision, Pillars and Inspiration Ledger)

Chapter fixer for `spec/01_VISION_AND_PILLARS.md`, 2026-10-11. Authorities: DECISIONS.md (rows 1 to 108 and the orchestrator rows; rows 89 to 108 postdate the triage and are applied where they contradict chapter text, as the other fixers did), canon-1.11 (every "canon amendment" row of `audit/canon_gaps_triage.md` treated as in force), then the triage's other rows and §4 conflicts.

Edits are made by Python scripts in the session scratchpad (`fix01/`) that assert each anchor occurs exactly once; the pre-edit chapter is saved there as `fix01/01_orig.md`.

Lint before: `== 01 words=16754; undefined_ids (1): input.mount; retired_ids (1): st.mounted`. Not in the multiply_defined map.

## Plan (batches)

1. Header, Imports.
2. C-16 (the Censor), C-30 (mount row), G7 / G-200 (`input.cast_cancel` gamepad binding), §7 verbs (DECISIONS 95).
3. Decisions 26 (Erasure), 41 (6 EA genres), 42 (prices), 38, 46, 77 to 80, 93, 94, 98, 100 to 108 across §1, §4, §5, §6, §8, §9.
4. Canon-1.11 amendments touching signature moments and loops (limb protection, ENH_CAP, Marginalia bosses, XP anchor, illuminated rolls, liquids, rng.run).
5. §10 KPIs (C-32 env wording), §11 questions answered by DECISIONS.
6. Exports, Interface assumptions, Canon gaps, Acceptance tests, Invariants.

## Done

(appended after each batch)

### Batch 1: header, intro, Imports (applied)

- Header "Conforms to canon-1.11" (triage G-051). Intro states the canon-1.11 check, mounts cut (canon-1.8, DECISIONS 55), no item durability (canon-1.10, DECISIONS 65), DECISIONS 26, 41, 98.
- Imports: DECISIONS row widened to rows 1–108 with the rows this chapter reflects; canon rows add §3.2–§3.4 (`rng.run`), §4.4 micro-voxels, §7.1 normalized skill XP (G-070), the 45-unit XP anchor (G-071), §9.1 crippled-only Errata severing (DECISIONS 84), spawn light incl. initial population (G-125), §13.1 trap categories (G-128), `ENH_CAP` (G-131), `ERASURE_ONSET`/`ERASURE_STEP`, §17.7 retired mounts, §17.8 trophies; doc 03 row cites `StatusInst.env` and C-32; doc 04 §8.2/§8.4 limbs; doc 13 display names (the Censor, C-16); doc 14 §2.3, §3.8, §4.7, §5.4a, §5.6, §8.4; doc 15 prices and §2.8.3; doc 16 §3.6 Steam/Epic, §9.3 host migration (DECISIONS 93), §11.7 per-Folio aggregates; doc 17 cast cancel in `input.ctx.casting`; new rows for doc 18, doc 21 and the triage.

### Batch 2: §1 fiction, §6.4 diagram, §7 verbs (applied)

- C-16 / G-177: `boss.editor_v2` is "the Censor" in the §1.3 premise and the §6.4 diagram; the redactors row reads "the Censor's office"; invaders stay lowercase "redactors" (Canon §17.9 keeps "Redactor" for the role; doc 18 `nar.lore.redactors`). Q-17 and the doc 18 interface row follow in later batches.
- §1.2 / §1.3 / premise: closing manuscript (DECISIONS 46, doc 14 §4.7, doc 18 §8.5a); new "Second Edition" fiction row (DECISIONS 100–108, owners docs 12, 13, 14, 18 at 1.0); the Margin row is the walkable 3D hub with resident arcs and the trophy alcove (DECISIONS 77, 78, Canon §17.8); Red Ink row names one Erasure hunter with `rank.hunter` identity and `rank.elite` multipliers (Canon §11.6, DECISIONS 91); Marginalia line no longer says "no Editor has authority" (Marginalia has Editor boss Folios, doc 14 §9.1).
- §7: retired "Mount, dismount" row (`input.mount`, `st.mounted`) deleted and replaced by one sentence recording the retirement (C-30, Canon §17.7, DECISIONS 55); `input.cast_cancel` gamepad binding is B in `input.ctx.casting` (G-200, stale G7); jump row adds the ledge mantle up to 2,000 mm (DECISIONS 95); new "Sprint-slide" row marked requested from doc 17 (DECISIONS 95); quickbar row adds the grappling tool item (DECISIONS 95); new hold-breath / whistle / shout row copied from doc 17 §12.1, §12.3, §12.4; gamepad column of the Book-menu row corrected to doc 17 §12.4 (View, View hold 15 frames, Book tabs, Book tabs, LB+Menu hold, Menu) and the trade row's gamepad input is View; camera is LB+R3 tap.

### Batch 3: §2.7 pillar records (applied)

- New pillar test record `content/01/pillar/tests.json` (`id`, `pillar`, `index`, `carry_forward`), because doc 19 §5 registers `01.pillar_test` at that path with no doc 01 definition; `carry_forward` is true only for `pillar.one_ruleset.t3`, matching `release_gate`.
- New "Shipped-content readings": tests over core genres run over the shipped core genres, tests over Standard runs use `mode.novella` until `mode.standard` ships; the Early Access reading table copies doc 21 §2.3 (e) (doc 21 G4 and its doc 01 request; DECISIONS 41). The 1.0 gate reads every test as written (doc 21 §3.2 M11).

### Batch 4: §3 ledger, §4 competitive bar (applied)

- §3.1 Barony rejects item wear and breaking (DECISIONS 65; Canon §4.3 block HP kept).
- SM-07 (§3.7 and §3.13): the severed arm must already be crippled (pool ≤ 2500 Bp of max) before the hit; a second run with a full arm pool asserts `wound.disabled` and no `ev.part_severed` (Canon §9.1 amended, DECISIONS 84); the Alchemist carries `item.tool.surgical_kit` (doc 04 §8.4, doc 11).
- §3.10 Minecraft: `block.tag.destructible` (G-036) and 250 mm micro-voxels (DECISIONS 37, Canon §4.4). SM-10 also asserts no initial-population spawns (G-125, Canon §12.3).
- §4.1: Hades row cites doc 02 §12.2 for Auto-Cloud and doc 16 §3.6 account sync; Marathon row "6 genres at EA, 16 by 1.0" (DECISIONS 41); PEAK row adds pings and strangers' voice off by default (DECISIONS 75).
- §4.2: PAR-02 host migration per DECISIONS 93 / doc 16 §9.3; PAR-03 Steam and Epic through Datagram Relay and an EOS build (DECISIONS 73); PAR-05 drops the ungrounded "Steam Machine 10-foot UI" and names the Windows build under Proton (DECISIONS 98); PAR-06 FOV 60–120, default 90 (DECISIONS 92); PAR-12 rewritten for 6 EA genres (DECISIONS 41, doc 21 §2.2–§2.4) with the Abridged bound `ceil_div(14, 3) = 5` and the Standard bound `ceil_div(24, 3) = 8` (doc 21 G6).

### Batch 5: §4.2 note, §5 novelty register (applied)

- §4.2 closing note: Abridged tells the full story at EA; Standard joins at 1.0 (doc 21 §2.2, §2.4).
- NR-02 Shock of the New excludes `rank.boss`, `rank.editor`, `rank.hunter` (G-059); NR-08 counts failed potions (G-056); NR-09 shows only Laws the account has discovered (DECISIONS 56, Canon §5.8); NR-11 micro-voxels (DECISIONS 37); NR-22 onset 720 s, one hunter, no repeat spawns (DECISIONS 26, Canon §11.6, §17.1); NR-23 optional and text only (DECISIONS 14, 38); NR-26 cites DECISIONS 80.
- New NR-27 closing manuscript (DECISIONS 46), NR-28 Second Edition (DECISIONS 100–108), NR-29 asynchronous traces (DECISIONS 96).

### Batch 6: §6 loops (applied)

- §6.3 Erasure table rebuilt on canon-1.11: warnings at 36,000 and 41,400 (doc 14 `ERASURE_WARN_*`), stack 1 at 43,200 (`ERASURE_ONSET` 720 s), stack 10 at 59,400 with the Folio's one hunter and no repeat spawns (DECISIONS 26, Canon §11.6, §17.1, doc 14 §3.8; doc 21 G8). Boss-Folio clock start, Red Ink threshold example (stack 6 at 52,200) and Colophon rules restated; "reaches the Colophon by tick 43,199" replaces 25,199.
- §6.4: level 29 on arrival at Folio 25 at 45 units per Folio, inside 28 ± 3 (G-071); Abridged is the EA full-story run; Second Edition paragraph from DECISIONS 100–108 (depth `25 + 2n`, 15 Folios in 3 Volumes, revised Editors, the Author, every slot enters, ghosts return, wipe forfeits the clear, Ironman reward).
- §6.5: the 80-hour unlock pace (DECISIONS 79; doc 14 §5.4a: 7,070 Quires / `Q_MIX` 101 = 70.0 h; Echoes 84.2 h by Recollection alone, at most 66.1 h with earned Ink), Recollection trials for every Echo (DECISIONS 80), resident arcs as text (DECISIONS 78).

### Batch 7: §8 sessions, §9 audience to tone (applied)

- §8: SS-2, SS-3, SS-4 need `mode.standard` (1.0); SS-1 states the exact 9 `rng.run` draws of doc 14 §8.4 with `folio_index = 0` (canon-1.11 `rng.run`, G-029), Origins over shipped core genres (`range(6)` at EA, doc 14 §2.3).
- §9.2 Platforms rewritten for DECISIONS 98 (PC only: Windows on Steam and Epic, Deck through Proton; no console, Switch, native macOS/Linux or browser client; Linux only for headless binaries) and DECISIONS 73 (doc 21 G16).
- §9.3: base game USD 19.99 at EA and 1.0 (DECISIONS 42; doc 21 G5); Folio Expansion 12.99 inside the 9.99–14.99 band, first two per DECISIONS 53.
- §9.4: ink gore by default with a red-blood toggle, PEGI 16 / ESRB M targets (DECISIONS 94, doc 15 §8.6).
- §9.5: narration text only, never voiced (DECISIONS 38); rule 6 adds ink gore; "DECISIONS #n" normalized to "DECISIONS n"; Enshrouded peer row no longer argues about ports (DECISIONS 98).

### Batch 8: §10 KPIs (applied)

- §10.1: per-genre KPIs run over the shipped core genres; new "Shipped modes" rule: KPIs over `mode.standard` report `Insufficient` until 1.0, except `kpi.coop.party_parity`, which reads Grade-0 `mode.novella` runs (matches the §2.7 reading and doc 21 §2.3 (e)).
- §10.3 `kpi.coop.party_parity` names `boss.editor_v1` deaths and the Novella fallback; `kpi.pillar.systemic_kill_share` owners add 07 (block traps).
- C-32 / G-185: the systemic-kill paragraph now states doc 03's `env` rule word for word in substance (Canon §10.6 hazards, §12.1 surface/trigger/reaction hits, environmental explosions and falling blocks, DoT pulses of an instance whose `StatusInst.env` is set by the creating/refreshing/stacking application); a DoT pulse is systemic exactly when it carries `env`; trap sources are `cat.placeable` block traps and `cat.gadget` snares, mines and charges (G-128); `damage_seq = 0xFFFF_FFFF` (bleed-out, doc 03 G28) is not systemic. Worked example 5 and acceptance test 11 reworded to match.

### Batch 9: §11 questions, premise wording (applied)

- Premise: Marginalia line restored to "where no Editor has authority" (matches doc 18 `nar.ending.into_margins`) with "though the Editors still give chase" (doc 14 §9.1 Marginalia Editors).
- Answered questions moved, numbers kept, to a second table: Q-02 (DECISIONS 46; doc 18 §8.5 endings), Q-06 (DECISIONS 41), Q-08 (DECISIONS 26), Q-11 (DECISIONS 98; doc 21 notes Q-11 superseded), Q-12 (DECISIONS 42 band; doc 15 12.99), Q-17 (triage C-16, G-177).
- Q-10 default: Standard from 1.0, Abridged at EA. Q-13 default: ink gore with a presentation-only red-blood toggle (DECISIONS 94). New Q-25: sprint-slide input (DECISIONS 95), default crouch while sprinting unless doc 17 binds an action.

### Batch 10–12: Canon gaps, Exports, Interface assumptions, Acceptance tests, Invariants (applied)

- Canon gaps: old G1–G8 closed with their triage rows (G-051; G-177/C-16; G-210/C-03; G-185/C-32; G-199/C-14; G-200 = stale G7 per §4.1; G-178) in one summary paragraph. New open items: G1 movement kit of DECISIONS 95 not yet in Canon §4 / doc 03 (mantle stops at 1,600 mm) / doc 11 (no grappling tool) / doc 17 (no slide binding); G2 Second Edition and asynchronous traces have no owner text, and the Second Edition gating (first clear vs every clear) is open for doc 14.
- Exports: canon-1.11 collision statement; test-record file named; labels `NR-01`…`NR-29`, `Q-01`…`Q-25`, `SS-2a`.
- Interface assumptions: 02 (Windows-only client; §17.4 per-Folio fields and minor bar, C-14, not yet on disk); 03 (present: `damage_seq`, `env` per C-32; requested: block-trap hit identification, 2,000 mm mantle, slide); 04 (present: crippled-only severing, surgical kit); 07 (block-trap identification); 10/11 (grappling tool item); 12 (discovered Laws only; fused Second Edition Folios); 14 (present run-history fields; Daily `rng.run` draws; Second Edition rules); 16 (present on disk); 17 (present incl. cast cancel, no mount input; requested slide binding); 18 (present: the Censor, `nar.lore.redactors`; text-only Narrator; closing manuscript); new 21 row (EA readings mirrored in §2.7).
- Acceptance tests (numbering kept, doc 21 cites them): 12 adds the 9 `rng.run` draws; 15 rebuilt on canon-1.11 Erasure; 19 novelty register is exactly 29 entries; 21 bans `input.mount` and §17.7 IDs; 22 Standard bound 8 and Abridged bound 5; new 24 (price 19.99, no console/macOS/Linux client, 6 EA genres, no gear wear) and 25 (18 test records, `carry_forward`, EA cell counts 54/78 vs 108/156).
- Invariants: shipped-content readings, no mount verb or gear wear, Erasure onset 720 s with one hunter.

### Batch 13: proofreading pass (applied)

- Lint fixes: the `damage_seq` field and the mount input are no longer written as back-quoted pseudo-IDs (`ev.entity_killed.damage_seq`, `input.mount` were flagged undefined).
- §3.2 SPD: "every genre skin" (not "all 16"). §6.6: level-29 note (225,000 / 274,000 Max HP, same 7 / 8 hits) beside the level-28 design target (G-071). §9.3: Fortune Engine cursed capsules with pact curses shown openly (DECISIONS 60; doc 11 §11.2, doc 10 §7.8).
- Lint after: `== 01 words=…` with no undefined, foreign, retired, JSON or hedge issues; chapter 01 absent from the multiply_defined map. All JSON blocks parse with integers only.

## Requests for other chapters

- **00 (canon):** §4.2 states the DECISIONS 95 movement kit (ledge mantle up to 2,000 mm, sprint-slide, grappling tool as an item outside the base kit).
- **03:** §5.9 mantle bound to 2,000 mm (now `h ∈ (550, 1600]`); define the sprint-slide motion; with doc 07, make an `ev.damage_dealt` from an Errata-placed block trap (`block.trap.spikes`) identify the trap, for doc 01 §10.3.
- **07:** block-trap hits identifiable (as above).
- **11:** a genre-skinned grappling tool item record (DECISIONS 95).
- **17:** a binding for the sprint-slide (doc 01 Q-25 default: `input.crouch` pressed while sprinting); doc 01 §7 copies it.
- **02:** apply C-14 to §17.4 (per-Folio aggregates, bar every minor-flagged account); doc 16 §11.7 already does.
- **14:** Second Edition rules (DECISIONS 100–108), including whether the choice comes at every Ur-Text clear or only after the account's first.
- **19:** `01.pillar_test` fields are now defined in doc 01 §2.7 (`id`, `pillar`, `index`, `carry_forward`).
- **21:** doc 01 now mirrors §2.3 (e) in §2.7, prices (19.99), PAR-12, §9.2 and the Erasure clock; doc 21 G4, G5, G6, G8, G16 and its doc 01 row can close. Test numbers 1–23 are unchanged; 24 and 25 are new.
- **18:** its Interface row for doc 01 (C-16) is satisfied.

## Status

All batches applied. Lint after: `== 01  words=19845`, no issues; not in multiply_defined.

## Verification

Adversarial verifier pass, 2026-10-11. Only `spec/01_VISION_AND_PILLARS.md` and this log were edited, every edit through a Python script that asserts each anchor occurs exactly once (scratchpad `v01/`, pre-pass copy `v01/01_pre_verify.md`). Line numbers are those after the pass.

**New since the fixer ran.** DECISIONS.md gained rows 109 to 125 (commit `40d2963`, 23:21, after this chapter's last batch at 22:57). Row 114 supersedes row 41, on which the fixer built §2.7's Early Access readings, PAR-12, §6.4, §8, §10.1, Q-06, Q-10 and tests 22, 24, 25. Rows 112 and 123 remove the Epic Games Store and EOS that §9.2, PAR-03, the Hades and ARC Raiders rows, Q-11 and the doc 16 Imports row still described; row 122 sets Folio Expansions at USD 9.99 (the chapter said 12.99); row 118 puts the Second Edition at Early Access and invasions, traces and streaming votes in updates before 1.0. Docs 02, 14, 15, 16 and 17 also changed on disk after the fixer (doc 02 §17.4 now carries the C-14 fields, doc 14 §8.9 specifies the Second Edition and §5.4 re-totals the Bindery, doc 16 is Steam-only, doc 17 names the slide input). The chapter was brought to all of these, as the doc 02, 15 and 18 verifiers did for their chapters.

### Check 1: triage rows that name chapter 01

| Row | Verdict | Chapter text that satisfies it |
|---|---|---|
| G-051 | applied | l. 2: "Conforms to canon-1.11."; intro l. 4 records canon-1.8 (mounts cut) and canon-1.10 (no item durability) |
| G-177 (reject) / C-16 | applied | §1.3 l. 59: "junior Editors on loan from the Censor's office"; premise: "the Censor (`boss.editor_v2`, Folio 10)", "The Censor keeps an office of junior redactors"; §6.4 diagram "Censor"; lexicon "PvP invader \| a redactor"; Q-17 answered (l. 860); Interface row 18 removed because doc 18 has `loc.boss.editor_v2.name` = "The Censor" and `nar.lore.redactors` on disk (Imports l. 31). "Redactor" appears only as Canon §17.9's role name (l. 59) and in Q-17's question |
| G-178 | applied | §7 l. 590: "Ghost: mark a target \| `input.attack` on an entity in the ghost context \| … `MarkTarget`" |
| G-185 / C-32 | applied, wording re-checked | §10.3 l. 812 restates doc 03 §14's `DmgFlags` sentence ("every Canon §10.6 environmental hazard hit, every Canon §12.1 surface, trigger or reaction hit, every environmental explosion and falling-block hit, and every DoT pulse of a status instance whose `env` is set") and doc 03 §3.3's `StatusInst.env` rule (created, last refreshed or last stacked by an `env` hit or a grid contact). Matches doc 03 on disk (l. 278, 1715, test 56) |
| G-190 | applied | Exports `kpi.` (27) names the seven `kpi.econ.*` IDs doc 15 requested (plus `kpi.econ.quires_per_hour_p50`); doc 15 §12's "Doc 01 ID" column lists the same eight with equal bands and modes |
| G-199 / C-14 | applied, then closed | was "doc 02 §17.4 follows … not yet on disk"; doc 02 §17.4 now carries the per-Folio fields and bars minor-flagged accounts. Canon-gaps closure now reads "doc 02 §17.4 and doc 16 §11.7 carry them on disk"; Interface row 02 removed; Imports doc 02 row cites §17.4 |
| G-200 / stale G7 (§4.1) | applied | §7 l. 581: "B while casting (`input.ctx.casting`; a second B within 18 frames dodges)", equal to doc 17 §12.1 and §12.4; G7 exists only as "the stale former G7" in the closure paragraph |
| G-210 / C-03 | applied | closure paragraph: trophy alcove and Prologue room L3 harvesting a node and placing ladders and planks; doc 14 §11 L3 and doc 12 l. 1458 confirm on disk |
| C-30 (`st.mounted` ×1) | applied | lint `retired_ids` empty; §7 l. 599: "Mounts are cut (DECISIONS 55, Canon §17.7): no verb mounts or dismounts, and the retired mount toggle, button bit 25, stays 0" |
| G-029 (`rng.run`, canon-1.11 §3.2) | applied, then fixed | SS-1 l. 617 cites "exactly the 9 draws of doc 14 §8.4 from `rng.run` (Canon §3.2) with `folio_index = 0`". **Miss fixed:** the Origin draws said "over the shipped core genres (`range(6)` at Early Access)"; with DECISIONS 114 they are `range(12)` each, as doc 14 §8.4 writes |

### Check 2: DECISIONS rows

| Row | Verdict |
|---|---|
| 15 invasions | agree (NR-21, §4.3 item 6, PAR-02); **fixed** for 118 and 125: NR-21 and PAR-02 now say invasions arrive in an update before 1.0 |
| 16 limited building | agree: §3.1, §3.10 (`block.tag.destructible`), §7 placement row, §1.3 trophy alcove, Enshrouded row; new Imports row for doc 07 §10.1, §10.4 |
| 26 softer Erasure | agree: §6.3 table l. 494 (stack 10 at 59,400, one hunter), NR-22, Q-08, test 15, invariant; recomputed against doc 14 §3.8 |
| 28, 119 Echo cast | agree: no Echo is named; "24 Echoes" in §6.5 |
| 33 hybrid glyphs | **added**: §6.5 now states the Glyph Library and the 2 run-start picks (doc 14 §5.7) |
| 37 micro-voxels, 38 text-only, 46 procedural ending, 55 mounts, 56 Laws, 60 pact curses, 65 durability | agree: §3.10 and NR-11; §9.5 and NR-23; §1.2, §1.3, NR-27, Q-02; §7 note and test 21; NR-09; §9.3 l. 658; §3.1 and test 24 |
| 42 USD 19.99 | agree: §9.3 l. 652; test 24 |
| 69 12+ enemies per genre | agree: no roster count is stated |
| 77–80 | agree: §1.3 l. 61 (walkable first-person 3D hub, residents' text arcs, trophy alcove); §6.5 (6-beat arcs at run ends; 80 hours; Recollection trial for every Echo); NR-26 |
| 81–83, 86–88 | agree or not applicable: no claim on ghost XP, Lucre transfers, invader rewards, presentation_hash or Somnambule |
| 84 | agree: SM-07 l. 350 (crippled pool at most 2500 Bp, second run at full pool gives `wound.disabled` and no `ev.part_severed`), equal to Canon §9.1 and doc 04 §8.2, §8.6 (citation §8.6 added) |
| 85 | agree: NR-25 equals Canon §7.1 |
| 92, 93, 94, 95, 96, 98 | agree: PAR-06; PAR-02; §9.4 (**fixed** to name doc 17's actual setting `access.gore`, `ink` default, `full`, `minimal`, doc 17 §13.5; Q-13 likewise); §7 (mantle row now says doc 03 §5.9 still stops at 1,600 mm); NR-29; §9.2 |
| 100–108 | agree after fix: §6.4 now follows doc 14 §8.9 (offer at every Ur-Text victory of `mode.standard` or `mode.abridged`, the first included; a wipe or an Abandon forfeits) |
| 109, 110 | **fixed**: PAR-12 said "Builds with fewer than 12 core genres use doc 21 §2.3 (a)'s lattice relaxation"; now 12 genres keep doc 12 §5.4's bound of 3 eligible genres, no relaxation; test 22 |
| 111, 113 | agree / **added**: no calendar date appears; §9.2 states no demo or public playtest build |
| 112, 123 | **fixed**: §9.2, PAR-03, ARC Raiders and Hades rows, Q-11, Imports DECISIONS and doc 16 rows, new invariant; `flag.net.crossplay` no longer cited (doc 16 asks doc 02 to drop it) |
| 114 (supersedes 41) | **fixed**: intro, §2.7 (readings table replaced by an Early Access gate rule that reads all 18 tests as written), PAR-12, Marathon row, §4.2 note, §6.4, §8 shapes and SS-1, §10.1 (the "Shipped modes" rule deleted), §10.3 party parity, Q-06, Q-10, §11 intro, tests 22, 24, 25, invariant |
| 117, 118 | **fixed**: §1.3, §6.4, NR-28 (Second Edition at Early Access, no repeated fused pair in a Volume), NR-29 |
| 120 | **fixed**: NR-23 says the Scribe ships at 1.0 and Early Access shows the template grammar |
| 121, 122 | **fixed**: §9.3 Folio Expansion USD 9.99 (was 12.99) with worn-armor mechs; Risk of Rain row; Q-12; test 24 |

### Check 3: chapter brief

| Item | Verdict |
|---|---|
| C-16, invaders lowercase "redactors" | pass (check 1) |
| stale G7 deleted | pass |
| retired `st.mounted` | pass |
| `rng.run` per canon-1.11 | pass after the SS-1 fix |
| 19.99 (42), text-only (38), procedural ending (46), limited building (16), micro-voxels (37), no durability (65), invasions (15), hub (77), arcs (78), 80 hours (79), Recollection trials (80) | pass (check 2) |
| 6 genres at Early Access (41) | superseded by DECISIONS 114, which the product owner added after the brief was written; the chapter follows 114 |
| C-32 wording equals doc 03's env rule | pass (check 1) |

### Check 4: leftover grep

Hits for the listed terms after the pass: "mounts"/"mount" (retirement records l. 4, 11, 17, 599, tests 21 and 24, invariant), "durability" (retirement records), "Mountain" (Noita's Holy Mountain), "Redactor" (Canon §17.9's role name, Q-17's question), "wear"/"breaks"/"repair"/"broken" (§3.1 rejection of item wear, test 24, invariant "fair_page broken by both"), "voiced" ("never voiced"), "24.99" (Slay the Spire 2's price in the peer table), "5400" (worked example 3's party-parity mean). No real leftover. Extra greps: "Epic", "EOS", "crossplay", "12.99", "range(6)", "6 genres", "shipped", "1.0" (all fixed where stale).

### Check 5: lint

`python3 tools/spec_lint.py --chapter 01` → `== 01  words=19978`, no findings; `--json`: chapter 01 has empty `missing_sections`, `undefined_ids`, `foreign_namespace_definitions`, `retired_ids`, `json_issues`, `hedges`; `multiply_defined` is empty. All three JSON blocks parse with integers only.

### Check 6: internal consistency

- **Recomputed** (Python, exact integers): §6.1 TTK 191 (190 ticks deal 206,321); §6.2 spawner 7 / carry 5000 / 8; dodge 8 and 10; §6.6 TTK 120, 191, 117, 308, 132, 111, 60; hits to down 9, 11, 9, 11, 7, 8 and the level-29 note (225,000 and 274,000 mHP, 7 and 8 hits); workloads 10000, 10000, 10500, 11156; §6.5 3.92 and 8 weeks; §8 budgets 1,500, 7,800, 4,350, 9,000, 1,560, 3,120; SM-03 cap 20 and 39 Emendations; §10.4 examples 300/410, 3250, 5400/5525/1075, 1000, 1 per 1,000 h; `fnv1a32` of the six pillar IDs and the three `kpi.` IDs in tests 1 and 2; no `fnv1a32` collision of the 51 owned IDs with any back-quoted ID in `spec/*.md`.
- **Miss fixed:** doc 14 §5.4 now pays 1,960 Quires from achievements, leaving 5,710: §6.5 reads `5710 / 60 = 95.2` and `5710 / 143 = 39.9` hours and 8.0 to 19.0 weeks; the Quire-rate paragraph `5710 / 50 = 114.2` and `5710 / 160 = 35.7`; Q-22 "40-to-95-hour". §6.5 also adds the 36 Scripts' 76.8 hours (64.0 at 12 exits per hour), equal to doc 14 §5.4a and doc 15 §2.8.3.
- **Tables:** column counts consistent; the Verbs row of the `syntax_delta` table had unescaped pipes inside a code span, which split the GitHub table cell; now `\|`.
- **§7 bindings** re-checked cell by cell against doc 17 §12.1, §12.3, §12.4: the throw cell now reads "LB+RB (hold to cook)"; the sprint-slide row carries doc 17's chosen input (`input.crouch` while sprinting), so test 21 no longer needs a "requested" exception.
- **Exports:** 24 `pillar.` and 27 `kpi.` IDs, all used, none missing; "shipped-content readings" renamed "the Early Access gate rule".
- **Interface assumptions** now hold open items only (intro sentence added): rows 02, 06, 08, 09, 15, 16 (telemetry), 18 and the "present on disk" parts of 03, 04, 14, 17 were removed after checking each on disk, with Imports widened to cite them (doc 02 §16 bots and scenario runner, doc 04 §1 ability slots and §8.6, doc 07, doc 13 §4.7 and §8, doc 17 §13.5). Open rows: 03, 04, 07, 11, 12, 13, 14, 16, 17, 21.
- **Canon gaps:** G1 updated (doc 03 G31's measurement question; doc 17's slide input); G2 rewritten: doc 14 §8.9 settled the offer question the fixer had left open, so G2 keeps only the canon fused-genre rule, docs 12 and 13's Second Edition records and doc 16's trace service. Header "Conforms to canon-1.11" present.

### Requests for other chapters (verifier)

- **14:** §2.3 Early Access scope per DECISIONS 114 (12 core genres; the Daily's Origins `range(12)` at Early Access) and §8.1, §8.9 per DECISIONS 118 (`mode.second_edition` from Early Access).
- **21:** apply DECISIONS 109 to 125: §2.2 scope table (12 genres, `mode.standard` and Second Edition at Early Access; invasions, traces and votes before 1.0; Steam only; no demo), retire §2.3 (e)'s Early Access readings and G4, G6, G16, re-word M4, M11 and WP-100, and update its doc 01 Interface row: doc 01 no longer has reduced Early Access readings, PAR-12 has no lattice relaxation, and the Folio Expansion is USD 9.99.
- **17:** put the sprint-slide (`input.crouch` pressed while sprinting) in §12's action list or binding tables, not only in the Interface row for doc 03.
- **03, 07, 11, 12, 13, 16:** unchanged from the fixer's list (block-trap hit identification, mantle and slide, grappling tool, fused Folios, revised Editors and the Author, trace service).
