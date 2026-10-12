# Fix log: chapter 14 (Run Structure and Meta Progression)

Fixer pass against canon-1.11, DECISIONS 1 to 88 and audit/canon_gaps_triage.md. Edits are made by scripted
exact-anchor replacement in `spec/14_RUN_STRUCTURE_PROGRESSION.md` only. Each batch below is appended when done.

Baseline lint before edits: `== 14 words=27631; undefined_ids (1): rng.worldgen.weighted`.

## Plan (batches)

1. Header, Imports, §1 constants and run state.
2. §2 start flow: Glyph Library picks, rule F1, kit ownership, kit IDs and values (C-02).
3. §3 Folio flow: Rubric sub-stream, Erasure (canon-1.6/1.11), boons (C-05), ghosts (C-01), Redaction, run-layer events.
4. §4 score and payout: Lucre per Erratum, hunter cap, RunSummary fields, closing-manuscript inputs (DECISIONS 46).
5. §5 Margin: 3D hub (77), resident arcs (78), 80-hour pace (79), invader-kit grants, Glyph Library (33), stale plot text.
6. §6 to §12: Recollection bound (80), Mercy (C-04, Q2), invasions (Q8), migration (C-06), hex (C-07), L3 (C-03), achievements.
7. Exports, Interface assumptions, Canon gaps, Acceptance tests, Invariants.

## Done

### Batch 1 (header, Imports, §1)

- Header now "Conforms to canon-1.11" (G-051).
- Imports: canon §2.4/§2.5 (G-016 to G-019), §3 (`rng.run` G-029, sub-streams G-027, offer generator G-030), §8 (45-unit anchor G-071/Q7; ghost XP G-072/C-01), §11.6 (canon-1.6 Erasure, G-112), §13.8 (`diff_hp_bp`, G-133), §14 (Lucre per Erratum, G-148), §16.2 (kit ownership, G-159), §17.1 (G-161 to G-163, `GLYPH_LIBRARY_START`), §17.9 (G-173, Q8), §18 (G-184); doc rows for 03 (§14.3, `sys.run_layer` 34a), 05/06/07/08/11 sections, 12 (renumbered sections), 13, 16 (migration, hex), 17 (Mercy), new rows doc 18 and DECISIONS.
- §1.1: Erasure UI cues moved to 600 s and 690 s (onset 720 s); `HUNTER_SCORE_CAP_PER_FOLIO = 1` (one hunter per Folio); new `Q_MIX`, `UNLOCK_PACE_HOURS`, `ARC_BEATS` (DECISIONS 78, 79).
- §1.2: `RunState.run_id`, `RunState.satchels` (doc 15 request), `SlotState.went_native`, `active_ticks`, `last_input_tick` (doc 15), `library_glyphs`, `glyphs_learned` (DECISIONS 33), `FolioRecord.story_solved`, `invasion` (doc 18 request, DECISIONS 46).

### Batch 2 (§2 start flow and kits)

- §2.1: ErratumSelect takes 0..=2 Glyph Library picks (DECISIONS 33, canon §17.1 `GLYPH_LIBRARY_START`); RunSetup takes the doc 15 Satchel; Commit draws `run_id` and sends the run header with `run_seed` (G-018, doc 16 §11.6).
- §2.2: new step 9 learns the library picks (no draws); rule F1 rewritten to amended canon `FOLIO_1_GENRE` (G-162) and doc 12 §5.6 (renumbered section).
- §2.4: kit ownership per amended canon §16.2 (G-159); owner IDs only (C-02): armor suffixes `_medium`/`_cloth`, thrown stacks as weapons, stack caps (thrown 20, throwables and potions 10, ammo carry caps).
- Kit values recomputed from doc 11 §3 (thrown 6 per unit), doc 08 (healing 30, toxic gas 15, purity 20, firebomb 12, mortar T1 10, alembic T1 15, flask 2) and doc 05 §8.1 ammo: universal 100; standard Calling parts 206, 192, 186, 314, 172, 109, 342, 248, 274, 240, 222, 138, 136; kit totals 209 to 442.
- Alt kits fixed per notes_archetypes and C-02: Gunslinger longarm fires `ammo.rifle` ×12 (and drops head and hands armor to stay within 500 Bp, +107 Bp); Grenadier `ammo.cell` ×300 (cap); sling stones are one `arch.ranged.thrown` stack (Alchemist ×5, Beastbinder ×4); every difference recomputed (all within 500 Bp).
- Script-kit differences recomputed: 0, +312, +215, −64, +232, −184, 0, −81, +218, 0, +90, 0, +147.
- Kit JSON: Gunslinger thrown entry to `slot.weapon_2`, `consumable_entries: []` (thrown weapons are not consumables); alt record removes entries 0, 1, 2, 3, 5 and adds `ammo.rifle` ×12.
- §2.5 joiners: Glyph Library picks and Satchel rows.

### Batch 3 (§3 Folio flow)

- Doc 12 citations renumbered per doc 12 I9 (§4.5 → §5.5, §4.6 → §5.6, §5.1–§5.3 → §6.1–§6.3; §4.9 is now this chapter's own interact-hold path, G-208).
- §3.1: Marginalia boss Folios cited to amended canon `BOSS_FOLIOS` (G-163). §3.2: Redaction trigger "alive and not downed" (G-173); run layer is doc 03 row 34a `sys.run_layer` (G-047, stale gap 19).
- §3.3: `enter_folio` applies pending `SlotJoin`/`SlotResume` (doc 16 request) and materializes Satchels at pos 1 (doc 15 request).
- §3.4: Rubric draw now on doc 12's labeled sub-stream `stage_rng(run_seed, f, genre, "rubric", 0)` (canon-1.11 §3.2, G-027); removes the undefined `rng.worldgen.weighted`.
- §3.5: `run.note.excavate` counts 20 destructible cells not placed by Errata (fixed terrain, canon §4.3; old gap 26).
- §3.8: Erasure rewritten to canon-1.6/1.11 (DECISIONS 26, G-112): onset 720 s, stack k at 43,200 + 1,800(k−1), cap 10 at 990 s, exactly one elite-strength hunter per Folio at `thr`, no repeats; table and example redone (Colophon at c = 48,000 holds 3 stacks).
- §3.9: boon generator cites canon §3.5 (G-030) with `context_uid = f` (new Canon gap 1); `run.boon.deep_pockets` on fresh piles only (C-05, G-224); desecration emits `ev.colophon_desecrated`; Colophon hunters are `enemy.editors.revisor`.
- §3.10: ghosts share kill XP (C-01, G-072, DECISIONS 81); new Redaction resolution including downed and ghost slots (doc 16 request, canon §17.9).
- New §3.11: emission rules for the eight run-layer events doc 03 §14.3 registers (G-184; closes old gap 6).

### Batch 4 (§4 score and payout)

- §4.1 `lucre_norm`: dropped piles do not count (Lucre per Erratum, G-148).
- §4.4 meta-layer steps: arc counters, Glyph Library merge, history entry with `RunState.history`, closing manuscript.
- §4.5 kept: recorded Folio stories, Glyph Library, resident arc beats.
- §4.6 `RunSummary`: lowercase `run_seed`, `run_id`, `mercy_ever_on`, `min_game_speed_bp` (doc 17 request, G-201), `colophon_marked`, `score_at_victory`; slot fields requested by doc 15 (§6.2 counters) and doc 18 (ending inputs), `glyphs_learned`.
- New §4.7 closing-manuscript inputs (DECISIONS 46; doc 18 §8.5a): `story_solved`, `invasion` codes, party, notable choices.

### Batch 5 (§5 Margin)

- §5 intro: walkable first-person 3D hub with stations, vendors (residents) and trophies (DECISIONS 77).
- §5.1: Index and Bestiary are "encyclopedia", not "codex" (G-197); `margin.plot` displays as "The Trophy Alcove" (§4.1 stale plot text, G-210); alcove light slot is a "ceiling fixture".
- New §5.3a: `unlock.invader.annotator`, `.censor`, `.footnoter` as free milestone grants for doc 13's Editor kits (G-176, doc 13 gap 20 and its Interface row); invading advances nothing on the invader's account (Q8, DECISIONS 86).
- New §5.4a unlock-pace model (DECISIONS 79): 73 non-cosmetic nodes 7,070 Quires = 70.0 h at `Q_MIX`; Echoes 84.2 h by Recollection alone, at most 66.0 h with earned Ink; pace contract ≤ 85.0 h.
- §5.5 renamed "Index and Bestiary knowledge"; stories recorded from doc 12 §20 (doc 12 asks "the Codex records the story (doc 14)").
- New §5.6 resident arcs (DECISIONS 78): 9 arcs `margin.arc.*` (8 doc 13 residents plus the Narrator), 6 beats each, at most one beat per arc per run end, text only, conditions never read Ink or purchases.
- New §5.7 Glyph Library (DECISIONS 33, canon §17.1): record, joining, picks (baseline runs pick from doc 06's base pool; new Canon gap 2), learning, horizontal rule.

### Batch 6 (§6 to §12)

- §6.1: every Echo has a bounded Recollection trial, validator rejects an Echo without one (DECISIONS 80); Mercy never pauses a trial (DECISIONS 82).
- §6.2/§6.3: `blocks` step now counts destructible cells not placed by Errata at 1 ch each (reference 100 per hour); `echo.ma_rust` trial "100 blocks" keeps 470 ch; its trait names destructible blocks (old gap 26). Retired Echo aliasing note moved from old gap 22 into §6.3.
- §7.3: `diff_hp_bp` cited to canon §13.8 (G-133). §7.4: Mercy removed from the Grade rule (C-04, Q2, DECISIONS 82).
- §8.1 invasions: `FolioRecord.invasion`, kit choice (§5.3a), invader rewards Quires and Red Ink marks only, invasion is not a run of the invader's account (Q8, G-174, DECISIONS 86).
- §8.4: doc 12 §5.6 citation; Daily Errata take 2 glyph picks from doc 06's base pool.
- §8.8: host migration per doc 16 §9.3 (C-06, G-219); new session-intent bullet (doc 16 request: SlotJoin, SlotDormant, SlotResume, SlotDeparted, HostChanged, FolioRestart).
- §9.3: `run.marg.brittle_kit` row no longer mentions durability (C-31). §9.4: eligibility uses `mercy_ever_on` and `min_game_speed_bp` (G-201); submission JSON in doc 16 §11.1 hex forms with `req_id` and `run_id` (C-07, G-221).
- §10: every † mark and the Mercy sentence removed (C-04); `scribes_mercy_ok` dropped from the record; `ach.builder` counts `cat.placeable` placements; `ach.excavator` 1,000 destructible cells not placed by Errata.
- §11: natural Erasure after 720 s; L3 rewritten (C-03, G-210): harvest `block.node.timber` with `item.tool.pick`, place 3 `block.ladder.wood` and 2 `block.bridge.plank`; 3,000 mm drop kept.
- §12: warm and cold migration copies; deferred Suspend cites canon §2.5 (G-019); `presentation_hash` warns only (G-017); cold migration increments `restores` (doc 16 request).

### Batch 7 (Exports, Interface assumptions, Canon gaps)

- Exports: 441 IDs (was 429): `margin.` 66 (+9 `margin.arc.*`), `unlock.` 144 (+3 `unlock.invader.*`); fnv1a32 of the 12 new IDs checked against every backticked or quoted ID in spec/ (no collision). Constants and functions lines updated.
- Interface assumptions rewritten with full owner IDs (no brace forms, no former names: C-02): doc 03 row 34a and §14.3; doc 04 kit notes moved here from old gap 2; doc 05 exported archetypes and ammo caps; doc 06 Glyph Library row; doc 07 timber node, ladder and plank placeables; doc 08 values; doc 11 values, stack caps, deep pockets, dropped piles; doc 12 renumbered sections, Rubric sub-stream, stories; doc 13 single hunter, Editor kits, residents; doc 15 requested counters; doc 16 session intents, migration, hex; doc 17 Mercy, arcs, manuscript; doc 18 arc text and endings.
- Canon gaps: all 26 old items removed (resolved by canon-1.11 amendments, already in canon, chapter-owned rulings now in the text, or stale per §4.1: gaps 16, 17, 19 and the plot text). Two real open items remain: (1) `context_uid` for a room fixture (boon altar), (2) Glyph Library picks in baseline runs.

### Batch 8 (acceptance tests, invariants, follow-ups)

- Tests updated: 2 (hunters one per Folio), 4 (canon-1.6 Erasure: 43,200, 59,400, 1,133,200, Colophon at 48,000, Red Ink 52,200), 9 (new kit values and caps), 18 (Echo without trial rejected), 19 (441 IDs), 22 (dropped piles), 24 (cold migration restore), 37 (L3 node and placeables), 38 (Script-kit differences), 40 (Redaction details).
- New tests 43 to 51: ghost XP (C-01), Mercy (C-04), Rubric sub-stream, Glyph Library, resident arcs, unlock pace, invader kits, run-layer events, submission hex (C-07).
- Invariants: Erasure cap and single hunter, ghost XP split, Mercy only off the boards, every Echo trial bounded, pace ≤ 85 h, arcs text only and never Ink-gated, Glyph Library, L3 placeables, invader rewards.
- §9.3: `lattice.marg.*` and `run.marg.*` add, `lattice.marg.*` first (moved from old gap 21).
- §8.2a: Abridged loot quantity definition per amended canon §17.1 (G-167).
- §1.2 `FolioRecord`: `doors_offered`, `door_taken`, `rewrites` for doc 01's KPI request (G-199).
- §3.10: ghost marks through doc 17's `MarkTarget` intent (G-178).
- §12.1: checkpoint record is doc 02's `RunCheckpoint` with `checkpoint_version = 1` and its fields (doc 02 request).
- Pace table: Ink row "at most 66.1" (13,204 ch = 66.02 h).
- Lint after edits: `== 14 words=31297` with no undefined IDs, foreign definitions, retired IDs, JSON issues or hedges.

### Batch 9 (alignment with concurrent doc 12 and doc 17 text)

- New Canon gap 3: boss-Folio Erasure clock keyed to `clock_start` (doc 17 G29 proposes the same canon text).
- §1.2: `FolioRecord.story` (doc 12 §20.5 record), `story_solved: Option<u64>` (solve tick, per doc 12 §20.7), `leaf_carried`; `RunState.laws_triggered` (doc 12 §4) and `bleed_on` (doc 12 §14, DECISIONS 54). This replaces batch 1's `story_solved: Option<(Id32, u64, u64)>`; doc 18 is asked to read the deed and seeds from `FolioRecord.story`.
- §2.1 Commit fixes `bleed_on`; §4.4 and §4.5 add Index stories and discovered World Laws; §5.5 gains Stories (solved or Leaf carried) and World Laws (DECISIONS 56) bullets; §4.7 stories row; Interface rows for docs 12 and 18 updated.

### Batch 10 (other binding decisions)

- §2.3: Early Access scope note (DECISIONS 41): six shipped core genres; unshipped-genre records absent; "each of the 12 core genres" counts and core-genre draws use shipped genres; numbers and test vectors are 1.0 values.
- §5.1: the Ledger keeps shareable deterministic replays with the killcam (DECISIONS 48).
- Not changed (reported as unresolved): DECISIONS 62 pacifist achievements; adding one would change the totals docs 01 and 15 cite.

## Requests for other chapters

- 00: canon wording for doc 14 Canon gaps 1 (`context_uid` of a room fixture = Folio index), 2 (baseline runs pick library glyphs from doc 06's `base` pool) and 3 (`ERASURE_STEP` keyed to `clock_start`).
- 01: §6.3 Erasure table and the paragraph after it still show 420 s onset and repeating hunters; use canon-1.6/1.11 (stack 1 at 43,200, cues at 36,000 and 41,400, stack 10 and the single hunter at 59,400; Colophon by tick 43,199 means no stack; Colophon entry stops stacks, and a hunter only spawns if a threshold is reached before it).
- 04: §13 drop "No launch Echo (doc 14 §6.3) uses a non-humanoid plan"; §2.6.1 budget table and test 37 replace the retired `aldric_unsworn` row with `fenrir` 2/1/1 and add `sun_wukong` 3/1/2 and `loki` 2/1/1; Interface row for doc 14 drops the ghost-XP conflict sentence (resolved, C-01).
- 05: Interface row for doc 14 is satisfied (owner IDs, `ammo.rifle`, `ammo.cell` ≤ 300, one thrown stack); call the step-13 factor `m13` (doc 14 §7.3) instead of `difficulty_bp(att, tgt)`.
- 06: add a §8.1 learning-source row "Glyph Library (doc 14 §5.7): 0..=2 picks, origin = Origin genre (`genre.ur_text` for an Unwritten Origin), learned after the run-start set at run start or join, source 0".
- 10: §12.3 and Canon gaps 5 cite "doc 14 Canon gap 16", which is deleted; cite triage G-197 or drop the citation.
- 11: delete the doc-14 rows of the former-name table (C-02); the doc 14 Interface row clause about former names can go; Canon gap 17 (deep pockets) is resolved by doc 14 §3.9 (C-05).
- 12: §17 `secret.exit.unwritten` "5 st.erasure stacks (from 540 s)" becomes "from 840 s" (c = 50,400); write "Index" for the account encyclopedia (lines on Law discovery and the Folio Leaf, G-197); §23 Prologue L3 contents: `block.node.timber`, `item.tool.pick`, 3 `block.ladder.wood` and 2 `block.bridge.plank` items, a 3,000 mm drop and a 2,000 mm gap.
- 13: delete Canon gap 2 (doc 14 §3.8 now follows canon) and gap 20 (doc 14 §5.3a defines the Editor-kit unlocks).
- 15: the requested doc 14 fields now exist (`run_id`, `satchels`, `active_ticks`, `last_input_tick`, per-slot counters); §2.8 "Item Codex" → "Index" (C-29); base game price 24.99 vs DECISIONS 42 (19.99).
- 16: delete G16 (C-01) and close G15 and G20 (C-06, C-07 applied in doc 14).
- 17: close G2, G4, G21 (applied in doc 14) and keep G29 aligned with doc 14 Canon gap 3; show §5.6 arc beats as text and the §5.7 glyph picks in the Dramatis.
- 18: §8.4 arc beats are text only (DECISIONS 78), 6 per `margin.arc.<resident>` keyed to doc 14 §5.6; §8.5a and the doc 14 Interface row read the deed and name seeds from `FolioRecord.story` and treat `story_solved` as the solve tick (`Option<u64>`), as doc 12 §20.7 defines it.

## Verification

Adversarial verifier pass over the fixer's result, against `audit/canon_gaps_triage.md`, `DECISIONS.md` rows 1 to 108 and canon-1.11. Edits were made by Python scripts that assert each anchor occurs exactly once; nothing was committed.

### V1. Triage rows that name chapter 14

| Row | Verdict | Chapter text that satisfies it (or fix) |
|---|---|---|
| G-029 | applied | §3.3 `Rng::from_seed64(stream_seed(run.run_seed, "rng.run", f as u64))`; §8.4 "draws from `rng.run` with `folio_index = 0`"; Imports "this chapter owns `rng.run`" |
| G-030 | applied | §3.9 "`o = offer_rng(slot, f as u64, "boon")` ... A **commit** advances the stream by exactly one `next_u64`"; room-fixture `context_uid` kept open as Canon gaps item 1 (canon §3.5 still says uid or EntityId) |
| G-047 | applied | §3.2 "The run layer is doc 03 §2.2's system 34a `sys.run_layer`" |
| G-072, C-01 | applied | §3.10 "It shares kill XP like every member present in the Folio (Canon §8: downed members and ghosts included, Dormant and Departed slots excluded; doc 04 §6.2)"; test 43 recomputed: `floor_div(1000 × 10000, 12500) = 800` |
| G-079 | applied | §6.3 "Traits are `trait.echo.<suffix>` ... in doc 04's `trait.` namespace, authored here" |
| G-100 | applied | §7.3 "Canon §10.5 step 13 multiplies `D` by one factor `m13`" |
| G-133 | applied | §7.3 "`template_hp × H(d) × rank HP × party-size HP × diff_hp_bp` ... (Canon §13.8 `H(d)` row)"; HP example recomputed (18,175,344 mHP) |
| G-142, C-02 | applied | §2.4 binds only owner IDs; every non-doc-14 ID in the chapter is defined by its owner (scripted check against canon §19 owners); all 13 standard, 13 alt and 13 Script kit values and differences recomputed from doc 11 §3, doc 08 §6.1/§9.1/§12.1 and doc 05 §8.1 (`value_per_100`): 206, 192, 186, 314, 172, 109, 342, 248, 274, 240, 222, 138, 136; alt 0, +208, +107, +63, 0, −92, +409, −162, 0, −334, −91, +144, 0; Script 0, +312, +215, −64, +232, −184, 0, −81, +218, 0, +90, 0, +147 |
| G-144 | applied | §2.4.3 `item.trinket.lattice_lens` |
| G-154, G-155 | applied | §4.6 "Doc 15 owns the Quire earn rate"; Account currencies `cur.quire` (Bindery) |
| G-159 | applied | §2.4 "Canon §16.2 gives doc 04 the kit roles, presets, kit balance and talents, and gives this chapter the kit records (`run.kit.*`)" |
| G-160 | applied | §6.3 Inkwell binding table |
| G-161 | applied | §3.9 Shop: "Absent in `genre.unwritten` Folios" |
| G-162 | applied | §2.2 rule F1 matches canon `FOLIO_1_GENRE` word for word in substance |
| G-163 | applied | §3.1 "every Marginalia Folio with `f % 5 == 0` is a boss Folio (Canon §17.1 `BOSS_FOLIOS`" |
| G-164 | applied | §3.6 item 2 "the draw still happens (stream consistency)" |
| G-167 | applied | §8.2a "loot quantity means worldgen chests and floor-loot spots" |
| G-168, G-169 | applied | §3.1 Abridged row; §3.9 Colophon, desecration and boons |
| G-173 | applied | §3.2 row 1 and §3.10 Redaction "the last Erratum that is alive and not downed" |
| G-174, G-176 | applied | §5.3a invader-kit grants; §8.1 "Its rewards are doc 15's Quires and Red Ink marks only" |
| G-179 | applied | §4.5 Ink from doc 15 over `RunSummary` (doc 15 owns the `boss.ur_editor` rule) |
| G-180 | applied | §9.2 overflow bound recomputed: 4,021,273,828,026,032,000 < `i64::MAX`; damage 81,715,654,144,800 |
| G-181 | no change needed | no mini-boss slot in this chapter; doc 12's `room.arena` |
| G-184 | applied | §3.11 table; all eight events are registered in doc 03 §14.3 with producer 14 (checked) |
| G-197 | applied | §5.5 "The item encyclopedia is the Index ... "Codex" names only the battle pass". Canon §5.8 still says "the Codex then remembers" a World Law: **added** Canon gaps item 6 |
| G-201 | applied | §4.6 `min_game_speed_bp`; §9.4 "no tick with `access.game_speed_bp` below 10000" |
| G-208 | applied | §4.3 `run.end.abandoned_version`; §3.6 interact-hold path. **Fixed** test 30, which said "every voter" where the code reads every living Erratum |
| G-210, C-03 | applied, then tightened | §5.1a alcove; §11 L3 harvests `block.node.timber` with `item.tool.pick` and places `block.ladder.wood` and `block.bridge.plank`; 3,000 mm drop kept (canon `SAFE_FALL` at VERVE 0 is 3,200 mm). **Fixed**: L3's leave condition now requires the harvest, as test 37 already asserted |
| G-211, C-04 | applied | no `†` left (HEAD had 16); §7.4 "Scribe's Mercy ... never blocks a Grade unlock"; §10 "never disables an achievement" |
| G-219, C-06 | applied | §8.8 "Host migration follows doc 16 §9.3: hot and warm migrations continue the Folio; only a cold migration restores the Folio-entry save and restarts the Folio" |
| G-221, C-07 | applied | §9.4 JSON: `"run_seed": "0xc6f2e12ddca712ab"`, 32-byte values 64 lowercase hex without prefix |
| G-224, C-05 | applied | §3.9 "every fresh Lucre pile the bearer picks up × 12500 (doc 11 §9.1) ... sales and dropped piles are excluded" |
| G-051 | applied | header "Conforms to canon-1.11"; no mount text; no item durability |
| C-29 | not this chapter | doc 15 writes "Index"; this chapter has no "Item Codex" |
| C-31 | applied | §9.3 `run.marg.brittle_kit` is a step-13 damage factor with no durability hook; no wear, repair or item breaking anywhere |
| §4.1 stale | applied | old gaps 13, 16, 17, 19 and the building-plot text are gone; `margin.plot` survives only as the trophy alcove's ID (doc 17 §11 uses it) |
| Canon amendments the chapter elaborates (G-016 to G-019, G-027, G-071, G-112, G-148) | applied | Imports rows and §2.1 Commit (`run_seed` in the run header), §12.2 (deferred Suspend, `presentation_hash` warns), §3.4 Rubric sub-stream `stage_rng(run_seed, f, genre, "rubric", 0)` (matches doc 12 §7.2's `stage_rng`), Imports canon §8 45-unit anchor, §3.8 hunter rank, §4.1 dropped piles |

### V2. DECISIONS rows that land in doc 14

- Changed rows: 15 invasions (§8.1 overlay), 16 limited building (§5.1a, L3, `ach.builder` on `cat.placeable`), 26 softer Erasure (§3.8 table recomputed: stack 1 at 43,200, stack 10 and the one hunter at 59,400, Colophon at 48,000 holds 3, Red Ink 4 doses gives 52,200, boss Folio stack 1 at 1,133,200), 28 Echo cast (§6.3), 33 Glyph Library (§5.7), 37 micro-voxels (counts are of broken cells; chipping is not a break), 38 text-only narration (arc beats and Narrator notes are text; Echo voice lines stay doc 18's, which voices them), 42 price (absent), 46 procedural ending (§4.7), 55 mounts (absent), 56 World Laws (§5.5), 65 no durability (absent), 69 and 84 (not this chapter): all agree.
- Rows 77 to 88: 77, 78, 80, 81, 82, 86, 87 agree. **79 fixed**: §5.4a now counts the 36 launch Scripts (Transcription, 768 exits, 76.8 hours at 10 Folios per hour) and its pace contract covers the Transcription rate, as doc 15 §2.8.3 requests; `margin.vellum_archive` hosts the Transcription shelf.
- **62 fixed** (fixer left it open; doc 09 asks for it): `FolioRecord.party_kills` and `party_pacified` track pacifist play, §10 defines a kill and a pacification, new `ach.pacifist` (grade 2). Totals now 59 achievements (21/18/14/6), 1,960 Quires, 6,440 Ink; §5.1a, §5.4, §5.4a, tests 17 and 52 updated.
- 93 host migration paths: agree (§8.8, §12.1, §12.3).
- **96, 100 to 108 fixed** (rows the fixer's 1–88 scope missed; docs 01 G2, 13 gap 3 and 17 §11.6 wait on this chapter): new §8.9 Second Edition (offer at every Ur-Text victory of Standard or Abridged, `mode.second_edition`, Folio `f_ur + n` at `d = 25 + 2n` in Volumes 6 to 8, every slot enters, ghosts return without item loss, not Marginalia, every non-victory end forfeits the clear, Ironman flag and board mark); §3.1, §3.2, §3.3, §4.2, §4.3, §4.6, §4.7, §8.1 mode row, §8.2a, §9.1, §9.2 and §9.4 follow it; §8.1 "Later overlays" fixes the run-layer rules of traces (presentation only) and audience votes (unranked, refused in Daily, Weekly, Prologue and ranked lobbies, host intents in the replay); Canon gaps items 4 and 5.
- Other rows citing doc 14 (2, 12, 27, 41, 47, 48, 51, 52, 54) agree.

### V3. Chapter-specific brief

C-01 to C-07, Q2, Q7, Q8, stale gaps, decisions 77 to 80, durability, `rng.run`, the Rubric sub-stream, namespace ownership, the procedural ending and the Glyph Library: all verified as in V1 and V2. Daily and Weekly draws recomputed from the canon §3.1 reference in Python: stream seeds `0x6c9f436c0c107704` and `0x54f2e7d7bdbce9f5`, draws 3, 8, 1, 5, 4, 7, 6, 1, 4 and 2, 0, 3; `PROLOGUE_SEED = 0x39d13f78bea6b81d`.

### V4. Leftover grep

`mount`, `ride`, `saddle`, `cavalry`, `durability`, `repair`, `wear`, `Delving`, `VO`, `24.99`, `Item Codex`, `Crowns`, `building plot`: no hits. Judged hits: `broken` in §2.4.4 ("would arrive broken", read as item damage) **reworded** to "with a Contradiction above 2 in the Folio-1 genre"; `breaks` (§3.9 desecration) and "broken destructible cells" are block HP and stay; `voice` (§6.3, Interface 18) is doc 18's voiced Echo lines, not the Narrator; `Redactor` names the invader only, which canon §17.9 keeps (G-177); `margin.plot` is the alcove's ID; `5400` is the alcove's z coordinate, not an Erasure tick.

### V5. Lint

`spec_lint.py --chapter 14`: no undefined IDs, foreign definitions, retired IDs, JSON issues or hedges, before and after these edits; `--json` `multiply_defined` is empty. A stricter scripted check (every ID the chapter uses in another namespace is defined by that namespace's canon §19 owner) passes.

### V6. Internal consistency

- Score examples A, B, C, D, E and R, presence examples, Grade-20 cap examples and the mutator chain (64,529) recomputed in Python: all match. **Fixed** test 2's wording: its `base = 105985` needs 3 Erasure hunters in all (B's one plus 2 in Marginalia), not 3 new ones. **Fixed** §8.1's `run.mut.short_book` paragraph, whose 400 and 303 Quires ignored the §4.6 per-Folio cap: at 10000 or 5000 the run meets the 8,000 cap (160 Quires, 192 per hour); 121 Quires at 2000 is unchanged.
- Power tables, Marginalia table, `DIFF_CAP_BP` at Folio 467, joiner Lucre 91, note Lucre 65 and `lucre_norm` 55 recomputed: match. Bindery 91 nodes 7,670 Quires and shelf sums, 73 non-cosmetic nodes 7,070 (70.0 h at `Q_MIX = 101`), Recollection Σ 16,839 ch (84.2 h over 2) and every trial's `est_ch` recomputed: match.
- Tables well formed (scripted pipe count). Exports: 443 IDs after adding `mode.second_edition` and `ach.pacifist` (fnv1a32 0xf8f64089 and 0xd3198e9e, no collision with the 5,653 ID tokens in spec/); every owned ID the chapter uses is exported.
- **Added** `FolioRun` (`RunState.cur`) to §1.2: `clock_start`, `colophon_entry_tick`, `rooms_visited` (room-index set), `map_revealed`, vote, offers and per-Folio masks, which docs 13, 16 (§6.10 `RunView`) and 17 read; `run.note.silent` now reads doc 09's `folio_alerted` on the Rubric-completion tick (doc 09 says it reads `run_alerted`, which never resets and would make the note unreachable after one alert).
- Smaller fixes: Interface row 04 ("no standard kit carries a pick"; Script kits do carry torches), Imports doc 16 sections (§5.7 session intents, §6.10, §9.13), §12.3's `FolioRestart` citation, "Canon gaps, item N" references, "store purchases" in the arc rule.
- Canon gaps: items 1 to 3 re-checked against canon-1.11 (still open); items 4 to 6 added. Interface assumptions hold no resolved-conflict text. Header reads "Conforms to canon-1.11".
- Lint after verification: `== 14 words=33885`, no findings.
