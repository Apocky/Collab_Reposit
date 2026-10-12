# Fix log: chapter 12 (World Generation and Genres)

Chapter fixer for `spec/12_WORLD_GENERATION_GENRES.md`, against canon-1.11 (already on disk) and DECISIONS 1 to 92.
Edits are made with Python scripts that assert each anchor occurs exactly once; each batch is confirmed with grep.

## Plan (batches)

1. Retired IDs and durability (C-30, C-31; DECISIONS 55, 65).
2. Header, intro, Imports, stale gap references and canon-1.11 wording (G-051 and the triage rows naming doc 12).
3. Genre bleed at 1 Folio in 10 after 3 finished runs (DECISIONS 54).
4. World Laws discovered in play (DECISIONS 56, Canon §5.8).
5. Prominent Folio stories, optional quest, ending fragments (DECISIONS 72, 46).
6. Micro-voxels, limited building, resource nodes, Lost Chapter templates (DECISIONS 16, 37; docs 14, 15 requests).
7. Folio Expansion genres (DECISIONS 53).
8. Exports, Imports, Canon gaps, Interface assumptions, tests, invariants; lint.

## Done

(appended per batch below)

### Batch 1: retired IDs and durability (C-30, C-31; DECISIONS 55, 65)

- §2.2 manufacturer rows now copy canon-1.11 §5.9 exactly: Rustmaw "reload time × 13000" (was durability × 5000), Hephaesteum "immune to `st.disarmed` and `st.corroded`" (was no durability loss), Somnambule limited to destructible and non-opaque blocks (G-037). Durability ×2 gone.
- Mounts ×9 gone: `room.no_mount` dropped from the §10.1 template JSON, the §16.1 kind tags (8 kinds), Exports, I8, test 21, the invariant line and V12; §16.4 (`room.no_mount` rule with `ev.dismounted` and doc 13 cavalry) deleted; §9.3 corridor note on the 2600 mm mount headroom deleted; `room.stable` "mount bay" is now dressing only. One retirement sentence after the §16.1 table (Canon §17.7).
- V12 reused for a micro-voxel shape check (batch 6 rule §10.4); test 21 now tests V12.
- Imports: "mounts" now reads "the retired mount IDs (§17.7)".

### Batch 2: header, intro, Imports, stale gap references, canon-1.11 wording

- Header "Conforms to canon-1.11" (G-051); intro lists the canon-1.11 checks (destructible tag, micro-voxels, Law discovery, mounts cut, no durability) instead of citing G1.
- Imports: DECISIONS 15, 16, 37, 46, 53, 54, 55, 56, 65, 72; Canon §3.2 sub-streams (G-027), §4.4 micro-voxels, `block.tag.destructible` (G-036), seam `local_genre` (G-060), Law discovery; doc 03 §4.4 shapes and §14.3 registered events (G-184); new doc 06 §3.9 row (`note_contradiction_failure`, G-057); doc 11 §3.5 `item.quest.folio_leaf`; doc 13 §6, §13 captives, `enemy.editors`; doc 14 FolioRecord, run ends, §5.5 Codex, §2.3 Lost Chapter unlocks; new doc 15 row (expansion SKUs, Scripts); doc 18 §8.5a.
- §5.6 Folio 1 rewritten to canon-1.11 `FOLIO_1_GENRE` wording (G-162); "(Canon gaps G3)" removed.
- "(G9)" ×3 now cite doc 03 §14.3 (G-184); "(G7)" now Canon §5.8 (G-060); "(G10)" replaced by the new §18.3 electronic-features list (G-064, chapter owns it) and V8 points to it; "(G11)" now Canon §17.1 `LATTICE_CANDIDATES` (G-164); Exports "(G13)" removed (G-041).
- Abridged loot (G-167): §13 and §23 say loot quantity is worldgen chests and floor spots; Reliquaries, Engines and vendor stock unchanged.
- §2.4 fixtures carry `block.tag.destructible`; §9.2 destructible list matches canon §4.3 (light fixtures, rubble, pit lids, hull panels added; nodes become spent blocks) (G-036, G-037).
- §2.4 placeable erasure excludes invaders (G-113, canon §11.6).
- Dread meter and §24.3 Unmoored count failed potions and use doc 06's `note_contradiction_failure` (G-056, G-057); Unmoored's Fortune-off effects follow G-058 (no Death's Door roll, `dd_used` unchanged, `avatar_of_fate` suspended, SER 40, Engine SER shift 0).
- §5.4 rule 6 names Canon §17.1's "lattice edge weights" (G-165). `room.arena` paid from the population budget (G-181).
- `gpv.evt.plea` captive is doc 13's `npc.captive.<genre_suffix>` (trader faction) paying `fac.rep.free_captive` and a `soldier` hireling offer (notes_enemies out-of-scope item).

### Batch 3: genre bleed in 1 Folio in 10 after 3 finished runs (DECISIONS 54)

- New `GenInput.bleed_on` (§7.1): set by doc 14 at Commit, 1 for seeded runs (Daily, Weekly, typed seed) or when the host account has ended at least 3 runs (any party run end of doc 14 §4.3, Prologue excluded); stored in the run header. §24.1 rule 1 notes it is the only account state that reaches generation.
- §14.1 `bleed_plan` rewritten: gated by `bleed_on`; one `roll_bp(drift_bp)` decides whether the Folio bleeds at all (forced to 10000 for the Oneiric and the bleed modifiers); a non-bleeding Folio has no entry zone, drift or seam. Volume ramp (`vol_of_depth`, +500 per Volume, cap 8000) and the `d` parameter removed. Draw order after the roll is unchanged, so drift picks of forced plans keep their old values.
- `drift_bp` (GPV 60) re-scaled so the 12 core values sum to 12,000 (mean exactly 1000 Bp): MF 1000, GH 1000, CH 1300, SO 800, CP 1000, PA 1100, WW 1000, HN 800, WX 1000, SP 1000, FT 1000, MA 1000; ON stays 10000. §3.6 JSON GH 2500 → 1000.
- Examples and test 7 recomputed with a Python port of canon §3 verified against this chapter's vectors (gen_seed, lattice and event stream seeds, ftype 3860, all old bleed examples reproduced before the change): Folio 7 GH none (8487), ON (MF, [MA, GH]), CH none (3626), GH bleed_add 1 (MF, [CH, HN]), Folio 2 GH none (2208), Folio 3 GH none (8697), Folio 8 CH (MF, [GH]), Folio 11 WW (MF, [HN], seam HN), Folio 17 FT (MF, [GH], seam GH). §14.2 example moved to Folio 17 FT (seam axes ARC 10, TEC 3, MYT 9, VRV 3).
- §22: `lattice.marg.bleeding` excluded while `bleed_on` is 0; `marg_mods` takes `bleed_on`; examples recomputed (unchanged with bleed on; Folio 36 MF gives [annotated, dense] with bleed off). Test 8 updated. Preview draw count "bleed 0–5".
- Frequency vector for new test 38: run seeds mix64(i), i in 1..=4000, Folios 1..=24, 12 core genres: 114,723 of 1,152,000 plans bleed (9.96%).

### Batch 4: World Laws discovered in play (DECISIONS 56, Canon §5.8)

- New §4.1 "Discovery in play": Laws apply from entry whether discovered or not; undiscovered Laws are omitted (no placeholder) from the exit preview, HUD Law list and Folio header; discovered = in the account Codex at run start or in the run's `laws_triggered`.
- `laws_triggered: BTreeMap<Id32, u8>` (Law → slot mask of Active Errata on the trigger tick) lives in doc 14's `RunState` (requested), saved and hashed; doc 03 emits a law-discovery event `{ law, slots }` (requested; not backticked until doc 03 registers its ID); the meta layer merges into each account's Codex (doc 14 §5.5) at run end and on departure.
- A trigger table gives exactly one discovery trigger for each of the 30 Laws.
- §4 intro and §5.7 preview row updated; test 39 added.

### Batch 5: prominent stories, optional Folio quest, ending fragments (DECISIONS 72, 46)

- §20 intro states the prominence rule. §20.3 adds the **story plaque**: a fixed Incipit lore object showing event 0 in praise, no draw, outside `L`, counting as event 0 for solving; V16 checks it. Story draw counts and test 29 unchanged.
- §20.4: solving now identifies the story relic in a new §13 **story relic spot** (seat's lowest free container socket, no draw; doc 11 fills it, requested, see notes_items "seat relic" gap) and stores `story_solved` in doc 14's `FolioRecord` (requested).
- §20.5: the Leaf is consumed on the exit-commit tick when an Active Erratum holds it, sets `leaf_carried`, and the story enters every party account's Codex at run end (doc 14 §5.5; doc 18 `nar.job.codex_story`); Codex titles `loc.gpv.story.<template_suffix>.title` (matches doc 18 §8.7).
- New §20.6 Folio quest (two optional steps, never gates anything, doc 17 lists it) and §20.7 story fragments feeding doc 18 §8.5a's closing manuscript (per Volume, lowest-index solved story).
- Tests 40 and 41 added.

### Batch 6: micro-voxels, limited building, resource nodes, Lost Chapter templates (DECISIONS 16, 37)

- §1: feet cell defined on micro-voxel masks (open = mask 0 or a non-colliding doc 03 shape; solid support = top layer `my = 3` full); new "Micro-voxel" vocabulary row (Canon §4.4).
- New §10.4 Micro-voxel shapes: palette slots name a doc 03 predefined shape or a custom `u64` mask; custom masks only in wall, ceiling and prop cells (walkable cells use predefined shapes, V12 and lint L8); masks rotate/mirror with prototypes; per-chunk custom-mask table in ascending first-use order; light reads shapes; destruction removes micro-voxels of destructible blocks only. §7.1 `chunks` comment lists shapes and the mask table.
- Resource nodes (Canon §4.3, doc 07 §9.3–§9.4): node sockets are 4-cell floor strips against a wall (template JSON gains `axis`), veins fill the first `size` cells of the room's lowest-index free socket (aligned with doc 07 §9.4 "fill the socket's cells"), so no vein replaces a wall cell; harvest nodes take a socket's first cell.
- Template choice (previously unspecified): one `range` per room over fitting templates (optional `genres` and `unlock` fields), 0 draws with one candidate; stage table and §10.3 draw counts updated; L10 strengthened.
- Lost Chapter templates: `unlock.room.lost_chapter_<genre_suffix>` adds 3 templates (hall, crossroads, gallery) for its genre, answering docs 14 §2.3 and 15 §3.1.2 ("3 room variants in genre g's room pool").
- Tests 42, 43 added (test 21 covers V12).

### Batch 7: Folio Expansion genres (DECISIONS 53)

- New §25: the first two planned expansions are Age of Sail and Pirates (01) and Kaiju and Mecha (02). Rules: canon rows first (new Canon gaps G18; no expansion ID reserved here, pattern `genre.<exp_genre_suffix>`), full GPV and assets list (with `drift_bp = 1000` so 1-in-10 bleed holds), content-set gating (never in Daily/Weekly baseline), lattice order appends expansion genres after the Ur-Text with edges ≥ 10 to every core genre, invariance of every vector without an expansion, and no vehicles (ships and mecha are architecture, enemies and bosses; Canon §17.7). Test 44 added.

### Batch 8: Exports, Interface assumptions, Canon gaps, invariants; lint

- Exports: removed the "Requested outside this chapter's namespaces (G9)" bullet that declared `ev.secret_found`, `ev.objective_completed` and `item.quest.folio_leaf` (they now sit in Imports as doc 03 §14.3 and doc 11 §3.5 references; canon §19); the hiding-method bullet and quest-item bullet no longer backtick `ev.secret_found` or `cat.quest` (both were counted as foreign declarations by the multiply_defined map). Added `drift_bp`, §25, Law discovery triggers, Lost Chapter templates, micro-voxel tilesets, §18.3, the quest and story records, `GenInput.bleed_on`.
- Interface assumptions: I1 adds shapes, the emitted events and the requested law-discovery event; I2 run header `bleed_on`; I3 destructible tag, node-socket strips; I5 captives, `enemy.editors`, arena budget; I6 requested story-relic fill; I8 `note_contradiction_failure`; I9 requested `bleed_on`, `laws_triggered`, FolioRecord story fields, Codex Laws and stories; I10 discovered-Laws-only display, Folio quest, plaque line, closing manuscript; new I11 (doc 15).
- Canon gaps: G1–G17 all closed (mapped to G-051, G-027, G-162, G-165, G-166, G-161, G-060, G-039, G-184, G-064, G-164, G-226, G-041, G-167, G-125, G-028, G-036) and deleted; one open gap G18 (expansion genres need canon §5.2/§5.4/§5.9/§5.10 rows).
- Invariants annex: account state only via `bleed_on`; bleed rule and mean 1000; Law discovery; node sockets; custom masks; plaque and quest; Lost Chapter unlock; expansion rules; destructible tag wording.
- Lint after batch 8: `== 12 words=34087` with no findings; chapter 12 appears in no multiply_defined entry.

### Final checks

- Copied canon tables (§2.1 = canon §5.9 identity, §2.2 = canon §5.9 manufacturers, §3.2 = canon §5.2) diffed programmatically against canon-1.11: equal ("Crowns" kept per DECISIONS 90).
- Small follow-ups: §4.1 discovery is per Active slot (slot mask) for both the Codex and in-run display; §14.1 non-bleeding Folios are single-genre outside §14.3 dynamic zones; test 38 joins the CI unit-vector list.
- Lint: `python3 tools/spec_lint.py --chapter 12` → `== 12  words=34098` with no findings; `--json` multiply_defined has no entry naming chapter 12 (was `ev.objective_completed`, `ev.secret_found`, `item.quest.folio_leaf`, then transiently `cat.quest`).

## Requests for other chapters

- **03**: register a law-discovery event (suggested `ev.law_discovered`, payload `{ law: Id32, slots: u8 }`, producer 12, consumers 14 and 17), emitted once per Law per run on its first trigger (doc 12 §4.1); doc 12 will name it once registered. Give each §4.4 predefined shape its 64-bit micro-voxel mask and store doc 12's per-chunk custom-mask tables (doc 12 §10.4, Canon §4.4).
- **14**: set `bleed_on` at Commit and store it in the run header (1 for Daily, Weekly and typed-seed runs, or when the host account has at least 3 party run ends of §4.3 outside `mode.prologue`; else 0) and pass it in doc 12's `GenInput`; add `laws_triggered: BTreeMap<Id32, u8>` to `RunState` and merge it into each account's §5.5 Codex knowledge at run end and on `run.end.departed`; add `story` (doc 12 §20.5 record), `story_solved: Option<u64>` and `leaf_carried: bool` to `FolioRecord`; add Laws and Folio Leaf stories to §5.5; update the stale "doc 12 conforms to canon-1.2" note and the doc 12 §4.5/§4.6/§4.9/§5.x citations to §5.5, §5.6, §6.1–§6.3.
- **17**: door panel and HUD show only the viewer's discovered Laws (doc 12 §4.1), with a discovery toast on the law-discovery event; list the optional Folio quest (solve the story, carry the Leaf) under the Rubric (doc 12 §20.6).
- **18**: ship the story plaque's fixed line naming the seat room and the Folio Leaf (doc 12 §20.3); §8.5a already reads `story_solved`.
- **11**: define the story-relic fill for doc 12 §13's story relic spot (one item, affix themes from the story's 2 domains, identified when the story is solved); §3.5 `item.quest.folio_leaf` row: consumed on the exit-commit tick when an Active Erratum holds it, setting `FolioRecord.leaf_carried`.
- **07**: define the worldgen patron-altar block doc 12 places in `room.shrine` and `secret.room.shrine` (open question in notes_materials.md item 2); confirm §9.4's "the socket's cells" means doc 12 §10.1's 4-cell node-socket strip.
- **05**: drop the doc 12 Interface row's `room.no_mount` dismount rule (C-30); doc 12 no longer has the tag.
- **09**: the citation "doc 12 §10.2" for light bands should read §11.3.
- **02**: citations "doc 12 §6.3" and "§13.1" should read §7.3 and §14.1; the run header carries `bleed_on`.
- **15**: §1.1 can name the first two Folio Expansions (Age of Sail and Pirates, Kaiju and Mecha; DECISIONS 53) and point to doc 12 §25 for what each genre pack carries.

## Verification

Adversarial verifier pass over the fixer's result. Edits were made with Python scripts that assert each anchor occurs exactly once. Vectors were recomputed with a fresh, independent Python port of Canon §3 (xoshiro256++, SplitMix64, Lemire `range`, FNV-1a), checked first against `gen_seed(R, 2, SO) = 0x1c35d41d581f5eaa` and `stream_seed(R, "rng.event", 2)` → 2892.

### Check 1: triage rows that name chapter 12

| Row | Verdict | Chapter text that satisfies it (or fix) |
|---|---|---|
| G-027 (12 G2) | applied | §7.2 "`fn derive(parent: u64, salt: u64) -> u64 { mix64(parent ^ mix64(salt)) }`"; "All worldgen randomness derives from `rng.worldgen` through independent sub-streams" |
| G-028 (12 G16) | applied | §13 "Stage `loot` (`stage_rng(…, "loot", attempt)`) places spots; doc 11 fills them from `rng.loot_place` and keyed `rng.loot`" |
| G-036 (12 G17) | applied | §9.2 "Every worldgen block is indestructible unless its block record carries `block.tag.destructible` (Canon §4.3, doc 07)", with light fixtures, rubble, pit lids and hull panels in the list; §2.4 item 2 fixtures carry the tag. Verifier added the doc 07 records `block.rubble`, `block.pit_lid`, `block.hull_panel` (fix below) |
| G-039 (12 G8) | applied | §9.5 "the doc 03 §4.7 `flip` bit on every cell of its interior, so bodies there use `gsign = +1`" |
| G-041 (12 G13) | applied | Exports "the non-canon fields `fixture_level`, `fog`, … `hard_bp` (§2.3), which extend Canon §5.9 without changing it" |
| G-051 (12 G1) | applied | header "Conforms to canon-1.11."; C-30 and C-31 applied (below) |
| G-057 | applied | §24.3 "docs 05, 06, 07 and 08 report every failure through `note_contradiction_failure(t)`" |
| G-060 (12 G7) | applied | §14.2 item 3 "the Folio genre supplies skins (`local_genre` is the Folio genre, Canon §5.8)" |
| G-064 (12 G10) | applied | §18.3 list; V8 "no electronic feature of §18.3 where local TEC ≤ 2"; "Doc 13's turrets inherit the rule" |
| G-125 (12 G15) | applied | §2.1 "`spawn_block_max` gates ambient, wave and initial spawns (Canon §12.3, doc 13 §10.2)" |
| G-161 (12 G6) | applied, tightened | §6.2 "one `room.shop` (none in the Unwritten)"; §13 counter caches in every Folio. Verifier added to §12 that the Unwritten's Colophon has no shop (Canon §17.1) |
| G-162 (12 G3) | applied | §5.6 "Unseeded runs (Standard, Abridged and every other mode without a typed or Canon §3.6 seed; Canon §17.1 `FOLIO_1_GENRE`)…" |
| G-164 (12 G11) | applied | §23 "the exit draw at Folio 14 still makes its 3 unseen candidates (Canon §17.1 `LATTICE_CANDIDATES`)" |
| G-165 (12 G4) | applied | §5.4 rule 6 "**Effective weight** (Canon §17.1's "lattice edge weights") `w(g) = edge(cur, g) × fresh_bp(g)`" |
| G-166 (12 G5) | applied | §5.4 rule 3 "Secret genres: once per run before Marginalia (§17.4)" |
| G-167 (12 G14) | applied | §13 "loot quantity (worldgen chests and floor-loot spots) is × 16667 per Folio (Canon §17.1)"; §23 "Reliquaries, Fortune Engines and vendor stock are unchanged" |
| G-169 | applied | §11.1 "2 fixed `block.planks` crates (no draw), so V14 and desecration (doc 14 §3.9) are always possible" |
| G-181 | applied | §16.1 `room.arena` "paid from the Folio's population budget" |
| G-182, G-183 | consistent | `law.ur_text.pure_reading` is active only where the local genre is the Ur-Text, so §14.3's `genre_shift` lifts it; §6.2 rest row "`director.profile.rest`: no initial population" |
| G-184 (12 G9) | applied | Imports doc 03 "§14.3 … the registered events `ev.secret_found` and `ev.objective_completed`"; doc 03 lines 1793–1794 register both with the payloads this chapter uses |
| G-204, G-208, G-226 | applied | §11.2 vein count `6 + floor_div(d, 5)`; doc 14 owns §3.6 commit paths; G12 closed |
| G-113 | applied | §2.4 item 5 "Every `fac.editors` member except an invader erases Errata placeables within 1500 mm" (verifier fixed the lowercase sentence start) |
| G-037 / DECISIONS 88 | applied | §2.2 Somnambule row equals canon §5.9 (programmatic diff of §2.1, §2.2 and §3.2 against canon: equal) |
| C-30 | applied | no retired ID in the chapter (lint `retired_ids` empty); §16.1 "Mounts are cut (Canon §17.7), so no room carries a mount tag" |
| C-31 | applied | no item wear, breaking or repair; Rustmaw "reload time × 13000", Hephaesteum "`st.disarmed` and `st.corroded`" |
| §4.1 stale (doc 07 gaps; doc 13 "Redactor's archive"; `margin.plot`) | applied | §11.2 veins; §21.2 "`boss.editor_v2` the Censor"; no `margin.plot` or building-plot text |

### Check 2: DECISIONS rows

- 15, 16, 37, 46, 53, 54, 55, 56, 65, 72: the chapter agrees (§2.4 invaders, §9.2 and §10.4 fixed terrain and placeables, §10.4 micro-voxels, §20.7 ending fragments, §25 expansions, §14.1 bleed gate, §16.1 mounts note, §4.1 discovery, no durability, §20 prominent stories and quest).
- 26 (softer Erasure): **miss fixed.** §17.4 still said `blank_page` opens at 5 stacks "from 540 s" (the old 420 s onset); canon-1.11 onset 720 s puts stack 5 at 840 s (doc 14 §3.8 table). DECISIONS 26 added to Imports.
- 56 vs doc 14 §5.5: **miss fixed.** Doc 14 names the account encyclopedia the Index and keeps "Codex" for the battle pass (G-197); the chapter said "account Codex" in §4.1, §20.5, §20.6, test 39, Imports and I9. Now "discovered Laws" (doc 14 §5.5) and "the Index", with one sentence noting that Canon §5.8 and DECISIONS 56 say "Codex". §20.5 and §20.6 now match doc 14 §5.5's rule (a story joins the Index of every present account when solved or its Leaf is carried; carrying is the only way to record an unsolved story).
- 28, 33, 38, 42, 60, 69, 84: nothing in the chapter conflicts (no voiced lines, prices, glyph rules, curses, roster counts or limb rules here). 77–88: consistent (87: story text never enters simulation state and `content_hash` covers the content set; 88: Somnambule copy exact; 81, 82, 83, 85, 86 do not touch generation).
- 99, 100, 101, 105 (time of day and weather; Second Edition) cite doc 12 but are not in canon-1.11: **miss fixed** by a new open Canon gap G19 with a proposal (the fixer's brief stopped at DECISIONS 92). Docs 09 and 18 asked this chapter for those IDs; they wait on the canon.
- 41 (6 Early Access genres): doc 21 §2.3 assumes this chapter gates secret exits by the shipped content and adopts the lattice cascade. Secret-exit gate added to §17.4 (inert with a full content set; test 23 vector added); cascade and Folio-1 mapping recorded as open Canon gap G20 pending canon-1.12.

### Check 3: chapter brief

- `room.no_mount` ×9, `ev.dismounted`, durability ×2: gone (grep and lint).
- World Laws discovered in play (56): §4.1 with one trigger per Law (30 rows).
- Procedural ending from story fragments (46) and prominent stories with optional quest (72): §20 intro, §20.3 plaque, §20.6, §20.7; doc 18 §8.5a reads "the lowest Folio index first" solved story per Volume, as §20.7 says.
- Genre bleed 1 in 10 after 3 runs (54): §14.1 and §3.5; recomputed (below).
- `ev.*` and `item.*` reference only: Exports holds no `ev.`, `item.` or `cat.` ID; multiply_defined has no chapter-12 entry.
- Seam `local_genre` amendment: §14.2 item 3.
- Expansions (53): §25. Verifier aligned item 6 with doc 21 §2.5 (kaiju and mecha are enemies and mini-bosses; a kaiju mini-boss uses `room.arena`, not the Editor box rule).
- 250 mm micro-voxels, limited building, resource nodes: §1, §10.4, §11.2. Verifier added doc 03 §4.4's rule that a custom mask stands only on a full, pillar, slab, stairs or layer block (doc 03's request in fix_03).

### Check 4: leftover grep

Every hit judged in context: "mount" only in "mountain" and the §16.1 retirement sentence and Imports; "breaks"/"broken" only for blocks, nodes, panels and lids; "repair" only for §15 validation repair; "Crowns" is the canon Lucre skin kept by DECISIONS 90; "5400" is `gp.ambush_extra`'s ambush interval, not an Erasure repeat; "Codex" remains only in the §4.1 note on canon wording and the doc 18 ID `nar.job.codex_story`. No "Delving", "voiced", "VO", "24.99", "Item Codex", "Redactor", `margin.plot` or "building plot".

### Check 5: lint

`python3 tools/spec_lint.py --chapter 12` → `== 12  words=34737`, no findings; `--json` multiply_defined is empty overall and names no chapter-12 ID. New references (`block.altar`, `block.rubble`, `block.pit_lid`, `block.hull_panel`, `block.node.timber`, `block.ladder.wood`, `block.bridge.plank`, `item.tool.pick`, `appr.reason.story`) all resolve to their owners.

### Check 6: internal consistency

- Recomputed and equal: §14.1 bleed examples (GH 8487 none; ON entry MF, drift [MA, GH]; CH 3626 none; GH `bleed_add` 1 → [CH, HN]; Folio 2 GH 2208; Folio 3 GH 8697; Folio 8 CH 711 → [GH]; Folio 11 WW 977 → [HN] seam HN; Folio 17 FT 67 → [GH] seam GH); test 38's 114,723 of 1,152,000 (9.96%); `drift_bp` core sum 12,000; §22 examples with `bleed_on` 1 and 0; Marginalia `k` list; §5.5, §5.6 lattice vectors; §6.1 Folio types; §17.4 secret-exit draws (and the new test 23 vector 7395); §14.2 seam axes ARC 10, TEC 3, MYT 9, VRV 3; §8.4 Cyberpunk typing total 1,819 and 703 in 10,000; 41 kinds, 25 rules, 30 Laws, 28 hazards, 21 events; preview cost 42 draws.
- Tables: every table's rows match its header width (script check).
- Exports: every owned ID used is exported explicitly or by a counted family.
- Stale items fixed: I3 (doc 07 no longer cites old sections), I7 (doc 09 has no G19 and now cites §11.3), I9 (doc 14 now holds `bleed_on`, `laws_triggered`, `FolioRecord.story`, `story_solved`, `leaf_carried` and the §5.5 rules, and no longer cites the old sections or a "Threshold room"). I2's doc 02 note stays (doc 02 still cites §13.1).
- §23 Prologue: doc 14 §11 puts L1–L2 in a bleed zone of the chosen Origin and L5–L6 in Fairy Tale, while the chapter said the Prologue has no bleed and the annex forbade bleed in the Unwritten. Now the zones are authored (Origin passed as `prev_genre`, requested of doc 14), L3's contents follow doc 14's request, and the annex reads "N! drawn bleed …" plus "W! Prologue zones authored".
- §24.2 golden hashes now name doc 02 §6.4's five determinism targets (macOS was missing).
- Doc 07 block records named per doc 07's request: `block.altar` in `room.shrine` and `secret.room.shrine`, `block.rubble` (falling debris, tremor), `block.pit_lid` (spike pit, collapsing floor), `block.hull_panel` (hull breach, replacing "`block.glass_pane` hardness"). §20.4 names doc 10's `appr.reason.story`.

### Stale requests in this log

The list above predates other fixers' work: doc 03 already gives every shape its mask and stores per-chunk custom-mask tables (doc 03 §4.3, §4.4); doc 07 already defines `block.altar`; doc 09 already cites §11.3; doc 14 already has `bleed_on`, `laws_triggered`, the `FolioRecord` story fields and the §5.5 rules; doc 05 no longer has `room.no_mount`; doc 15 §1.1 already names both expansions. Still open: doc 03's law-discovery event, doc 11's story-relic fill, doc 17's Folio quest line, doc 18's plaque line, doc 02's §13.1 citation, and (new) doc 14 passing the Prologue's Origin as `prev_genre`.

