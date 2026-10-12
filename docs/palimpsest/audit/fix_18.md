# fix_18 — chapter fixer log for spec/18_ART_AUDIO_NARRATIVE.md

As of 2026-10-11. Target: canon-1.11. Edits are made with Python scripts that assert each anchor occurs exactly once; each insertion is confirmed with grep afterwards.

Baseline: `spec_lint.py --chapter 18` clean (no undefined, foreign, retired, JSON or hedge issues); chapter 18 is in no `multiply_defined` entry.

Authority notes:
- DECISIONS 90 overrules triage Q9/G-052: canon-1.11 keeps "Crowns" with a product-owner exemption (Canon §5.2 row 1, §22). The chapter follows the canon, not the triage's "Nobles".
- DECISIONS 89 to 104 postdate the triage. Applied here where they contradict existing chapter text (90 Crowns, 91 hunter rank, 94 ink gore). 97, 99, 100 and 103 wait on owner IDs and are recorded as open items.

## Batches

(appended below as each batch lands)

### Batch 1 (done): header, Imports, retired IDs, durability, terrain, madness

- Header → "Conforms to canon-1.11"; intro paragraph names canon-1.11 sections, DECISIONS items and the triage.
- Imports: Canon §2.4/§3.8 (presentation_hash, presentation generators; G-017, G-032), §4.3 (fixed terrain, no durability; DECISIONS 65), §17.7 (mounts retired, `enemy.role.charger`), hunter rank (G-112); DECISIONS 49, 53, 55, 65, 77, 78, 87, 90, 94; doc 04 §12.4 and §13; doc 13 charger; doc 14 §4.7 and §5.6 arcs; doc 17 `access.mark_hallucinations` and §16.4.
- §2.1 rule 2: the underlayer shows only on destructible cells (Canon §4.3, §4.4); test 49 now uses a cracked `block.tag.destructible` wall and adds the fixed-terrain case.
- §3.4 silhouettes: retired riders replaced by doc 13's unmounted chargers (`ram_raider`, `posse_deputy`, `wild_huntsman`) (Canon §17.7, triage C-30, DECISIONS 55).
- §3.6: `cavalry` row ("mounted" tag) replaced by a `charger` stance rule (Canon §17.7; orchestrator row on cavalry).
- §10.2: "Mount" budget class deleted (Canon §17.7).
- §8.2: the Grand Engineer's taboo explanation no longer refers to tools wearing out; doc 06 §16.1's taboo is now a fizzling Device spell (DECISIONS 65, C-31).
- §4.5a: madness phantoms are unmarked by default (DECISIONS 49; doc 17 `access.mark_hallucinations`); the old "drawn in Narrator pencil" gave them away.

### Batch 2 (done): canon amendments, triage gap rows, Interface assumptions

- §1.1: new "Hashing" bullet: `art.`/`aud.`/`nar.`/`vfx.` records and `loc.` tables go into `presentation_hash`, never `content_hash` (G-017, Canon §2.4, DECISIONS 87). §10.1 Hash row says the same.
- §1.2: the generators are Canon §3.8's (G-032); name grammars' `Rng::from_seed64(name_seed)` recorded as the one exception (new Canon gap 1).
- §7.1: identity exemption cites Canon §5.2 (G-053); new "Product-owner exemption" paragraph for "Crowns" (DECISIONS 90, Canon §5.2 row 1, §22); review-table row and test 16 updated; "Nobles" proposal and "Until the canon renames it" sentence deleted.
- §8.2: patron display-name paragraph says doc 06 now uses "The Undersigned" (G-177).
- §8.5/§8.5a code: field names follow doc 14 §4.6/§4.7 (`deepest_folio`, per-slot `SlotEnd`).
- Canon gaps: old gaps 1–10 all closed (G-052 via DECISIONS 90, G-053, G-032, G-054, G-017, G-202, G-210, G-177/C-16, G-217/C-12, G-112/DECISIONS 91) and replaced by a summary plus 2 open items: name-seed generator vs Canon §3.8; DECISIONS 97/99/100/103 waiting on owner IDs (with a proposal for the Author vs the "no Author's face" rule).
- Interface assumptions: 01 (C-16 still pending in doc 01), 02 (C-12 still pending in doc 02), 04 (§13 present; §12.4 phantom seed should use `pres_hash`), 06 (present), 14 (rewritten to doc 14's real `RunSummary`/`FolioRecord` fields: `story_solved: Option<u64>`, `story`, `leaf_carried`, arcs), 17 (gore default `ink`, `access.mark_hallucinations`, note placement; stale G8/G26 mention removed per triage §4.1).

### Batch 3 (done): text-only Margin arcs, ink gore, Echo bodies, expansions, the Margin

- New §3.9 "The Margin" (`art.bible.margin`): the walkable hub renders with the Unwritten's palette, light, LUT and post records unchanged over the `margin` tileset; residents and trophies keep their origin style (DECISIONS 77; Canon §17.8). Exports list `art.bible.margin` and the four reused `*.unwritten` records concretely (lint).
- §5.1: non-humanoid Echoes use doc 04 §13's eye heights (1,100 / 1,000 mm) and slots: Fenrir's `slot.weapon_1` in the `jaw` socket, Jörmungandr's `slot.hand_extra_1`/`_2` in `grip_1`/`grip_2` coil tips (DECISIONS 28, Canon §16.2).
- §5.4: gore modes rewritten for DECISIONS 94: `ink` is the default (ink-colored wounds, severed parts shed script for 120 ticks and stay lootable), `full` is the red-blood toggle, `minimal` unchanged. Doc 17 still defaults to `full` (cross-chapter request).
- §6.5 VO: Margin residents drop from 1,200 voiced lines to a 160-line voiced greeting set; dialogue and arc beats are text (DECISIONS 78, doc 14 §5.6 "No beat is voiced"). Total 3,960 lines, 79,200,000 B. New "Text only, never voiced" bullet (DECISIONS 38, 78).
- §8.4: the old 150-line milestone unlock model replaced by doc 14 §5.6's 6-beat arcs, written as text exchanges `loc.margin.arc.<resident>.beat_<n>.<line>` (2–6 lines, ≤ 90 chars), with a worked binder beat; the Narrator's arc is 6 pencil notes in the Ledger. Exports list the 54 beat keys.
- §9.1: the Narrator-note `<moment>` values listed for doc 17 §16.4; notes play no audio, no subtitle, no caption (DECISIONS 38).
- §10.1: new "Folio Expansion genres" paragraph (DECISIONS 53, doc 12 §25): no reserved IDs; full per-genre family and lints; traditions for Age of Sail and Pirates and Kaiju and Mecha; no piloted vehicles (Canon §17.7); base-game counts unchanged.
- §10.2: audio total 409.2 MB, VO 79.2 MB.
- Lint after batch 3: clean.

### Batch 4 (done): acceptance tests and invariants

- Tests 52 (VO 3,960 lines, 79,200,000 B; audio 409.2 MB), 54 (jaw socket, `grip_1`), 55 (20 voiced greetings per resident, no audio for `loc.margin.arc.*`) updated.
- New tests 59 (hash split, Canon §2.4), 60 (54 arc beats as text), 61 (gore default `ink`, DECISIONS 94), 62 (unmarked madness phantoms, DECISIONS 49), 63 (mounts retired, charger silhouette), 64 (expansion invariance and palette lint).
- Invariants annex: 3 new lines (presentation_hash; no mount/vehicle/rider/item wear; gore default; phantoms unmarked; underlayer only on destructible cells; no voiced arc beat or manuscript).

### Batch 5 (done): reconciler renames

- `art.vm.ranged.sidearm` → `art.vm.ranged.pistol` (audit/map_archetypes.json: `arch.ranged.sidearm` → `arch.ranged.pistol`); Exports state that `art.vm.<archetype>` drops the `arch.` prefix.
- Checked every audit/map_*.json old ID against the chapter: no other old ID remains.
- Lint after batch 5: clean.

### Batch 6 (done): stale doc 17 and doc 02 gap references

- Doc 17 G8, G23 and G26 are deleted on disk (triage §4.1): references replaced by doc 17 §2.2, §2.4 and §10.10; the stale "interim names `investigating`/`returning`" sentence in §4.6 deleted (G-196: doc 09's six `aware.` states are used everywhere).
- Imports row for doc 02: "gap G1" replaced by the canonical serialization (§11.2), since G-017 put `presentation_hash` in Canon §2.4.

## Final state

- `python3 tools/spec_lint.py --chapter 18`: clean (no undefined, foreign-namespace, retired, JSON or hedge issues; all required sections present). Chapter 18 is in no `multiply_defined` entry.
- Required sections kept: Imports, Exports, Interface assumptions, Canon gaps (2 open items), Acceptance tests (1–64), Invariants annex.

## Requests for other chapters (not edited here)

- **00 (canon)**: §3.8 add "and `Rng::from_seed64(seed)` over a name seed the simulation already recorded (doc 12 §20.1, doc 18 §7.4)" (doc 18 Canon gap 1).
- **01**: triage C-16: "the Censor" and "the Censor's office" for `boss.editor_v2` in §1.3, the Editors diagram, Q-17 and the doc 18 Interface row; invaders stay lowercase "redactors".
- **02**: triage C-12: §8.3 texture layers 64 × 64 px, full mip chains, 5,488 B per layer, world-space triplanar at 64 texels per meter; delete G1 (Canon §2.4 now has `presentation_hash`).
- **03**: triage C-16: "the Censor's `blackout`" instead of "Redactor `blackout`".
- **04**: §12.4 seed madness phantoms with `pres_hash("art.phantom", run_seed ^ entity_id, floor_div(tick, 600))` (Canon §3.8) instead of a bare `mix64` expression.
- **13**: a `visual` field (`art.visual.*`) on every template; humanoid brutes (`leg_breaker`) at `stand_mm` ≥ 2200; Second Edition boss IDs (DECISIONS 103) and a ruling on showing the Author (doc 18 proposal: a writing hand, never a face).
- **16**: the reliable host-to-client `ScribeText { key: [u8; 32], job: Id32, subject: u64, json: ≤ 4,096 bytes }` message.
- **17**: `access.gore` default `ink` (DECISIONS 94); `full` is the red-blood toggle; update the description to "ink wounds and unravelling parts (default), red blood, or no decals".
- **12, 14, 04**: IDs for DECISIONS 99 (time of day, weather state in `FolioView`), 100 (Second Edition ending layer) and 97 (12 presets per kin, palette and marking colors) so doc 18 can key presentation records on them.
- Batch 7 (done): §6.6 mix-budget Total row realigned so the 64 / 128 voice total sits in the Voices column (table had 5 cells for 6 columns).

## Verification

Adversarial verifier, 2026-10-11. Every check re-read the chapter on disk after the fixer's batches. Fixes were applied with three Python scripts whose anchors each matched exactly once (`edit18.py`, `edit18b.py`, `edit18c.py` in the session scratchpad). Lint after: `== 18  words=23806`, no undefined, foreign, retired, JSON or hedge findings; `--json` `multiply_defined` is empty.

### 1. Triage rows that name chapter 18

| Row | Verdict | Chapter text that satisfies it |
|---|---|---|
| G-017 | PASS | §1.1 "**Hashing** (Canon §2.4, DECISIONS 87). Every record in these namespaces and every `loc.` table is hashed into `presentation_hash`, never into `content_hash` … Replays record `presentation_hash`, and a mismatch only warns."; §10.1 Hash row; test 59. |
| G-032 | PASS | §1.2 "Canon §3.8 names the two pure functions that supply every cosmetic or narrative choice"; code identical to Canon §3.8. |
| G-052, Q9 | PASS (overruled) | DECISIONS 90 keeps "Crowns"; §7.1 "**Product-owner exemption.** The canon Mythic Fantasy Lucre skin "Crowns" passes tests 1 to 4 (DECISIONS 90; Canon §5.2, §22)"; canon §5.2 row 1 still reads Crowns. No "Nobles" left. |
| G-053 | PASS | §7.1 test 1 "Exemption: a currency's identity skin, the skin equal to its own base name (Unwritten "Lucre"; Canon §5.2)", matching canon §5.2's amended naming rule. |
| G-054 | PASS | §3 "The first three swatches of every palette are Canon §5.9's, copied exactly and in order; this chapter adds four." |
| G-112, Q12 | PASS | Imports Canon §17 row "one Erasure hunter per Folio carrying `rank.hunter`"; verifier added §6.1 "**Level 4** … The softer Erasure hunter keeps that cue, because it carries `rank.hunter` for presentation under `rank.elite`'s multipliers (Canon §11.6, DECISIONS 91)". |
| G-177, C-16 | PASS | §8.2 "`loc.boss.editor_v2.name` = "The Censor" … `loc.patron.redacted.name` = "The Undersigned" (triage G-177; doc 06 §16.1 now uses it)"; §8.1 `nar.lore.redactors` "The Censor keeps an office of junior redactors". "Redactor" remains only for invaders (Canon §17.9). Docs 01 and 03 now read "the Censor" on disk, so the fixer's Interface row 01 was stale (removed). |
| G-196 | PASS | §4.6 item 4 uses doc 09's six `aware.` states; §8.8 bark sets per state. |
| G-202 | PASS | §6.5 "spoken barks are text bubbles (`loc.npc.bark.*`)", "Voiced lines are never rewritten by the Scribe"; `nar.job.bark_pack` rewrites barks. |
| G-210 | PASS | Canon gaps closure "the Margin plot is doc 14's trophy alcove (G-210)"; no building-plot text; no `margin.plot` in the chapter. |
| G-214, C-17 | PASS | §9.11 "Launch locales (15, DECISIONS item 40): `en-US` (source), `fr-FR`, … `zh-Hant`"; doc 17 §14 lists the same 15. |
| G-217, C-12 | PASS | §2.3 "64 px per m, world-space triplanar (Canon §4.4) … 64 × 64 px per layer"; §10.2 "a 64² block layer 5,488 B". Doc 02 §8.3 now ships 64 × 64 (5,488 B), so the fixer's Interface note "doc 02 still reads 32 × 32" was stale (removed). |
| C-30, C-31 (not naming 18) | PASS | No retired mount ID (lint `retired_ids` empty); no item wear (see check 4). |
| §4.1 stale gaps | PASS | No doc 17 G8, G23 or G26 reference; no `margin.plot`. |

### 2. DECISIONS rows

- **Changed rows.** Invasions (15): §4.6 rule 8, §8.1, §8.3 PASS. Limited building (16): §2.1 rule 2, §3.9 PASS. Softer Erasure (26): Imports, §4.5a PASS; no `5400`. Echo cast (28): §5.1, §8.9 PASS (Sun Wukong wuxia, Loki mythic_antiquity, Lilith gothic_horror, Fenrir `body.quadruped`, Jörmungandr `body.serpentine`, as Canon §16.2). Hybrid glyph learning (33): no conflicting text. Micro-voxels (37): §2.3, §2.1 rule 2 PASS. Text-only narration (38): §6.5 Narrator 0 lines, §9.1 PASS. USD 19.99 (42): no price in the chapter. Procedural ending (46): §8.5a PASS. Mounts cut (55): PASS. World Laws discovered (56): the chapter never previews a Law (Folio intros use story domains only). Pact curses (60): no conflicting text. No durability (65): PASS. **12+ enemies per genre (69): FIXED** — §6.5 budgeted 14 mini-bosses (one per genre); DECISIONS 69 requires 2 or more per genre, so the plan now voices 28 (840 lines), total 4,380 lines, 87,600,000 B; §10.2 audio 417.6 MB; test 52 updated. Crippled-limb severing (84): §5.4 presents severing only; no conflicting rule.
- **Rows 77–88.** 77 PASS (§3.9). 78 PASS (§8.4 arcs as text; test 60); the dialogue box is now cited as doc 17's `ui.menu.margin_dialogue`. 87 PASS (§1.1). 79–86 and 88 land elsewhere; nothing contradicts them.
- **Rows 89–117.** 90, 94 PASS. 91 **FIXED** (§6.1 Level 4 sentence). 97 **FIXED**: the fixer filed it as waiting on owner IDs, but doc 04 §2.1 ("doc 18 authors the presets") and doc 17 §5.7 delegate the presets to this chapter and the kin IDs are canon; new §5.6 defines `art.preset.<kin_suffix>.<nn>` (72), `art.preset.<kin_suffix>.colors` (6) and `art.tex.marking.<kin_suffix>.<nn>` (72), the `Appearance` triple, readability limits and presentation-only status; test 65; Exports; Interface rows 04 and 16. 98 **FIXED**: §10.1 Compress row said "(UASTC on web)"; there is no browser client. 100 **FIXED**: doc 14 §8.9 now defines `mode.second_edition` and §4.7 asks for doc 18's second ending layer, and the fixer's `select_ending` returned `short_story` for a Second Edition victory (its last Folio is not the Ur-Text); §8.5 now returns `second_edition` as the second layer after `ur_clear_ending`, keeps a forfeited wipe out of `erased_in_margin`, keys `erased` plates to Volumes 6–8, and adds the closing and Ironman lines; §8.5a, test 66, Exports and invariants follow. 99 and 103 stay open (doc 12 G19, doc 13 gap 3). **114 FIXED**: it supersedes 41, which the chapter still applied ("Early Access ships 6 genres"); the intro, Imports, §10.1 production order and test 53 now follow 114. 109–113 and 115–117 land elsewhere.

### 3. Chapter brief

Text-only narration PASS (§6.5 Narrator row "0 (600 authored notes, text only)", no VO budget; §9.1 "Notes are manuscript marginalia only"; the Scribe still writes `codex_item` flavor, §9.3). Closing manuscript PASS (§8.5a). Echo cast PASS. "the Censor" PASS. 15 locales PASS. 64 × 64 layers at 64 texels per meter and 250 mm micro-voxels PASS (§2.3). Expansions Age of Sail and Pirates, Kaiju and Mecha PASS (§10.1). Margin arcs as text PASS (§8.4, test 60).

### 4. Leftover grep

`mount`/`mounted`: retirement sentences only (Imports, §3.6 charger row, test 63, invariant) and "mountain". `ride`: "overrides" and "The Rider at the Crossroads" (doc 06's patron name, not a mount). No `saddle`, `cavalry` (except test 63's "no `cavalry` row"), `durability` (Imports and intro retirement), `repair`, `Delving`, `24.99`, `Item Codex`, `5400`, `margin.plot`, "building plot". `wear`: "When a story wears thin" (lore figure of speech). `broken`/`breaks`: `ev.ward_broken`, Gleipnir's broken links, "breaks the fleet's first article", "rough, broken" line style. `voiced`/`VO`/"voice actor": the Editors', hunters', residents' greetings, Echoes' and mini-bosses' VO and its contracts; nothing voices the Narrator, patrons, arc beats or manuscripts. "Crowns": correct (DECISIONS 90). "Redactor": invaders only. "hoofbeat" in the Weird West pulse stem is kept as a percussion texture, not a mount.

### 5. Lint

Before and after: clean for chapter 18; no `multiply_defined` entry involves it.

### 6. Internal consistency

Recomputed in Python from Canon §3.1–§3.2 and §3.8: `nar_rng` seeds `0xae27e411c755d822`, `0x57f506109a835e39`, closing-manuscript section seeds `0xe3bffbb566a8741e`, `0xf0324ad0cf4755ff`, `0xee582d0bb6b9f352` with draws 1; 0, 1; 0, 0; story draws (1, 3, 1, 1) and (0, 2); intro draws (0, 1); name draws (0, 4, 1, 2) and (1, 4, 0, 1); `fnv1a32("nar.job.folio_intro") = 0x1a087cb5`; cache key `f691c204…dc5a` over 78 bytes, seed `0x04c291f6`. Contrast: telegraph minimum 4.07 (wuxia), mythic_fantasy 4.22, gothic_horror 4.58, cyberpunk 6.52, ur_text 4.80 (old `#C8341E` 3.78); reading minimum 7.00 then 10.46; pencil minimum 5.17 (old `#6B6B6B` 3.26). Music bar lint holds for all 17 tracks; loops 2,186.7 s and music 189.4 MB; VO `4380 × 2.5 × 8000 = 87,600,000`; audio `189 + 87.6 + 63 + 17 + 61 = 417.6` MB; BC7 64² 5,488 B; pools 64 / 128; memory 256 MiB; latency 6,934 / 5,600 / 12,067 / 7,334 ms; ragdoll 3800 / 7400 / 8000 / +4500. All 66 tests are numbered contiguously; every table has a constant column count. Exports cover every `art.`, `aud.`, `nar.` and `vfx.` ID used (the only uncovered strings are `pres_hash` tags). Header reads "Conforms to canon-1.11".

**Interface assumptions** held items marked "(present)" and stale notes; doc 01 (C-16), doc 02 (C-12, `art.icon.` row, seam mask, reveal and dark vision), doc 03 (`ChunkView` masks), doc 04 (phantoms now seeded by `pres_hash("madness.phantom", …)`), docs 06, 09, 10, 11, 12, 14, 15 and doc 17 (`access.gore` now defaults to `ink`; `ui.menu.margin_dialogue` exists) all provide what was asked. The table now lists only open requests (02, 04, 07, 13, 16), and Imports gained what the removed rows recorded (doc 02 view fields, doc 07 materials, doc 09 `StealthHud.marks`, doc 12 story plan and weather front, doc 14 field lists and Second Edition flow, doc 17 dialogue box and preset picker). **Canon gaps** keep 2 open items; item 2 now drops 97 and 100 (defined here) and names doc 12 G19 and doc 13 gap 3 as the owners still missing records.

### Verifier fixes

- Intro and Imports: DECISIONS 69, 91, 97, 98, 100 and 114 added; 41 replaced by 114; Imports rows for docs 02, 04, 07 (new), 09, 12, 13, 14 and 17 extended.
- §1.1 namespaces: appearance presets.
- §5.6 (new): appearance presets (DECISIONS 97).
- §6.1: Level 4 and the hunter cue (DECISIONS 91).
- §6.5: 28 genre mini-bosses, 840 lines; total 4,380 lines, 87.6 MB (DECISIONS 69).
- §8.4: Margin dialogue box cited as `ui.menu.margin_dialogue`.
- §8.5: Second Edition layers (`ur_clear_ending`, `nar.ending.second_edition` with its Ironman line), Second Edition wipe never `erased_in_margin`, `erased` plates for Volumes 6–8; §8.5a follows.
- §10.1: "(UASTC on web)" removed (DECISIONS 98); production order follows DECISIONS 114.
- §10.2: audio 417.6 MB, VO 87.6 MB.
- Exports: presets, colors, marking masks, `nar.ending.second_edition`, `ur_clear_ending`.
- Interface assumptions: only open rows (02 pool wording, 04 and 16 `Appearance`, 07 block skins, 13 with the second mini-boss, 16 `ScribeText`).
- Canon gaps item 2 rewritten.
- Tests 52, 53 updated; tests 65 (presets) and 66 (Second Edition endings) added; 2 invariant lines.

### Requests for other chapters (verifier)

- **01, 12, 14, 15, 21**: apply DECISIONS 114 (all 12 core genres and the Standard run at Early Access; supersedes 41). Doc 01 §2.7, PAR-12, tests 24–25; doc 12 G20; doc 14 §2.3; doc 15 §3.2.7 and test 37; doc 21 §2.2–§2.3 still use 6 genres.
- **02**: §9 describes `aud.mix.default` as stealing across pools; doc 18 §6.6 pools never borrow, a full priority-7 pool stops its oldest voice, and other pools replace their quietest voice only for a louder request. Also delete the web-build leftovers ("UASTC for the web build" in the asset table, "on the web build" in the particle count; DECISIONS 98).
- **04**: §2.1 Cosmetics stores `Appearance { preset, base, marking }` (doc 18 §5.6); Going Native keeps it.
- **16**: send each slot's `Appearance` to every peer with the join data; the `ScribeText` reliable message (unchanged from the fixer's request).
- **14**: §4.7 and §8.9 may cite doc 18 §8.5's `ur_clear_ending` then `second_edition` for the second ending layer.
- **13**: unchanged from the fixer (visual field, `leg_breaker` at `stand_mm` ≥ 2200, Second Edition boss IDs and the Author ruling), plus the second mini-boss per genre that doc 18 §6.5 now budgets.
- Fixer requests now satisfied on disk: 01 and 03 (C-16), 02 (C-12, G1), 04 (phantoms via `pres_hash`), 17 (`access.gore` default `ink`).
