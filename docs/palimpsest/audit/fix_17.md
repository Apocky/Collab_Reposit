# Fix log: chapter 17 (UI, UX, Controls and Accessibility)

Fixer for `spec/17_UI_UX_CONTROLS_ACCESSIBILITY.md`, 2026-10-11. Target: canon-1.11 (canon on disk already reads canon-1.11).
Starting lint: `retired_ids (2): cat.mount, st.mounted`; no undefined IDs, no foreign definitions, no JSON issues, no hedges; no multiply-defined IDs involve doc 17.

Plan (in triage §5.3 order): retired IDs (C-30 mounts, C-31 durability), then C-15, C-17, C-18, then canon-1.11 alignment (G-011, G-016, G-052, G-089, G-130, G-020, World Laws §5.8), DECISIONS 38, 56, 77, 82, Q10, then gaps, Interface assumptions, questions, tests, Exports.

## Batches

### Batch 1 — retired IDs (C-30 mounts, C-31 durability; DECISIONS 55, 65; canon §17.7, §13.9) — DONE

- Removed `ui.hud.mount` (both HUD tables), the §10.2 mount panel (now a one-paragraph retirement stub, numbering kept), `input.mount` (action table, KBM `H`, pad L3 glyph layer now unbound), `input.ctx.mounted` (contexts re-ranked: margin 8, on_foot 9), `ui.tip.mount`, the `cat.mount`/`st.mounted` mentions, "mounted" from placement reasons and lean availability, doc 05 §17 from Imports, the `mount` noise segment from the §13.7 sound-viz classes. Bit 25 now has no action; `encode()` sets bits 21..=24 only.
- Exports: `ui.hud.` 32→31, `ui.tip.` 15→14, `input.` actions 71→70, `input.ctx.` 10→9. Tests 10 and 11: 24→23 and 21→20 rectangles. Test 54 uses the casting context instead of mounted. Question 36 marked answered by DECISIONS 55.
- Durability UI removed: weapon HUD warning, Large weapon note, item-card row 13 (rows renumbered 13–16, 14a), the "durability" instance field in §4.3 and the doc 10 Interface row, "repair" from §9.2 services (list now matches doc 11 §12.3 minus repair and mount mending), the lockpick "break chance" (lockpicks never wear or break). Retirement sentences kept in §4.3, §9.2, §10.11.
- Side fix: §10.11 auto-lockpick example said "Success 40.0%"; doc 09 §13.2's own example gives chance 8400 for the same inputs, so it now reads 84.0% (rating 72, DC 55).
- Side fix: sound-viz class "magic" now includes the `prayer` noise segment (`noise.prayer` is a cast, doc 09 §7.7).
- Paper doll: `slot.hand_extra_1`, `slot.hand_extra_2` (G-130, canon §13.2) replace `slot.hand_extra_N`.
- Lint after: clean (retired_ids gone).

### Batch 2 — header, C-15, C-17, C-18, G-052 — DONE

- Header now "Conforms to canon-1.11"; the intro sentence lists the canon-1.8, 1.10 and 1.11 changes this chapter was checked against (G-051).
- C-15: `Haggle { shop }` renamed `ShopHaggle { shop }` in §9.2 and the doc 02 Interface row. §9.2's price breakdown now follows doc 11 §12.2's current trade-factor chain (adds `reputation_bp`, the 7500 floor and `value_vendor`).
- C-17: §14 lists the 15 launch locales of DECISIONS 40 / doc 18 §9.11; G7 deleted; question 20 marked answered (Arabic question kept).
- C-18: §10.7 uses doc 06 §16.3's `r1 = range(180,000)` and §16.4's `prayer_timer <= 7200`. New exact example: favor 300, timer 20,000 → Helped 53.3330% (`pct4(300 × 159,999, 500 × 180,000)` = 533,330), Angry 46.6670%; timer 179,999 → 0.0000%. Added the Devotion 100 rule (first supplication per Folio uses the timer-0 lines), the petition +3600 chaining note and a "petitions open" HUD state. Tests 20 and 40 updated.
- Petition cost example now applies doc 06 §16.4's `skill.devotion.m25` × 8000: Benediction 60 is 50 at Devotion 20 and 32 at Devotion 50 (doc 06's own example says 40 at Devotion 50, which omits the m25 factor; request filed for doc 06).
- G-052: item-card range example "Crowns" → "Nobles".

### Batch 3 — canon-1.11 alignment and DECISIONS rows — DONE

- G-016/G-017, DECISIONS 87: intro states the replay log records commands and intents (Canon §2.4), `ui.*` records are presentation (`presentation_hash`, peers may differ), `input.*`/`access.*` records are content (`content_hash`), player bindings and setting values are in neither.
- Imports: DECISIONS rows 26, 38, 40, 47, 48, 49, 55, 56, 65, 75, 77, 82, 87 and triage Q2, Q10 added; canon row names small-rate display, light overrides, intent records and both hashes; pipeline row drops "added, G3"; canon §5.2–§5.10.
- G-011: P11 and §5.6 cite canon §1.5's `floor(x / 100)` tenths for per-second rates below 10 (was a G16 assumption).
- Canon §11.6/§17.1 (DECISIONS 26) — STALE NUMBERS FIXED: the Erasure clock used onset 25,200 ticks (420 s) and implied repeat hunters. Now onset 43,200 ticks (720 s), stack `k + 1` at `c = 43200 + 1800 × k`, cap 10 ("Erasure 10"), `thr = max(6, 10 − red_ink_doses)`, hunter glyph after `ev.hunter_spawned` (one hunter per Folio). `c` uses doc 14 §3.8's `clock_start`. Journal timeline and test 49 updated (39,600 / 43,200 / 59,400).
- DECISIONS 56, canon §5.8: door panels show only Laws the viewer's account has discovered; new Journal "World Laws" page; new "World Law discovery" paragraph in §7 (no placeholder for undiscovered Laws; notification on first trigger; set kept with the Index, doc 14 §5.5; per-account in co-op).
- DECISIONS 77: §11.1 states the walkable first-person 3D hub with stations, vendors and trophies. §10.8 uses doc 14 §5.1a's 32 `margin.slot.*` records (12 trophy, 8 wall, 6 floor, 3 banner, 3 light) in `margin.plot`, with locked-slot rule text; §11.1 names `margin.plot` as the arranger's facility (G2 resolved on disk).
- G-020: stealth meter bars show light after canon §2.3's light overrides clamp them.
- G-089: §13.1 cites canon §10.5 step 6c; scope restated against canon-1.11's §10.6 entry points (lethal takedown enters at step 7 and targets only NPCs, doc 09 §9.4).
- G-173: Mercy death counting names "the last Erratum alive and not downed that an invader reduces to 0 HP".
- DECISIONS 82 / Q2: "never blocks" cites DECISIONS 82 and C-04 instead of G4.
- C-03: §15's Prologue L3 sentence matches the triage ruling (harvest a node with the pick, place a bridge-span placeable).
- DECISIONS 38 naming: `access.narrator`'s Settings label is "Read menus aloud" (keeps the OS speech reader apart from doc 18's text-only Narrator). The §16.4 Narrator-notes section comes in batch 4.

### Batch 4 — new panels, narration, hallucination marking, inline gap refs — DONE

- Inline "(Canon gaps Gn)" references removed from the body (§2.2 ×2, §2.4, §12.2, §12.7, §13.9); §12.2 now cites triage C-10 and states bit 25 (retired mount toggle) has no action and stays 0; §12.7 states "no in-simulation magnetism" (was G12).
- Reconciler follow-up (notes_inputs_ui.md): doc 09 §13.4 and §13.7 call for "doc 17's hack panel" and "pickpocket panel". New §10.12 defines `ui.hud.hack` and `ui.hud.pickpocket` (open conditions, lines, chances, times, intents `Hack`, `Steal`, `Plant`, cancel reasons) and the overlay placement shared with `ui.hud.lockpick` (400 × 144 at C (0, −112); checked to clear the invasion banner and crosshair at every scale, margin and canvas of tests 10–11; hides prompt and vote). `input.ctx.menu` now also covers the two panels; P8 lists them as non-pausing HUD. §10.11 retitled "Locks".
- `access.lock_mode` transport: the client sends `SetLockMode { manual }` (new intent kind, requested of doc 02; doc 09 reads it per slot); an attempt keeps its starting mode.
- DECISIONS 38: new §16.4 "Narrator notes" (no audio, no subtitles or captions; diegetic surfaces use doc 18's pencil on doc 18's paper; firsts go to the notification list for 8 s, max 2 lines; tooltip footnotes in `muted`). New global token `pencil` `#A0A0A0` (7.28:1 on `#101010`), because doc 18's `#4D4D4D` measures 2.25:1 on the HUD backplate.
- DECISIONS 49: new `access.mark_hallucinations` (off; forced on while `mercy_on`): dashed outline and "?" on madness phantoms, "(phantom)" caption suffix, "[Whispers (madness)]".
- Exports: `ui.hud.` 31→33 (+`ui.hud.hack`, `ui.hud.pickpocket`), `access.` 50→51 (+`access.mark_hallucinations`); non-ID interfaces list the overlay placement and the `pencil` token.
- Lint after: clean.

### Batch 5 — Interface assumptions, Canon gaps, questions, tests, invariants, Imports — DONE

- Interface assumptions rewritten: gap tags replaced by triage rows (C-03, C-04, C-09, C-10, C-11, C-15, C-28, C-30, C-31, G-201); doc 02 row adds `SetLockMode { manual }` and bit 25 reserved; doc 04 row adds phantom identification; doc 09 row adds hack/pickpocket data and intents, `noise.<category>` two-part IDs, no lockpick break roll; doc 11 row adds trade factor and services without repair/mount mending; doc 12 row adds World Law triggers; doc 14 row cites §5.1a slots (defined), Erasure `clock_start` at 43,200 ticks, cap 10, one hunter, discovered Laws with the Index; doc 16 row: light after overrides, sanitizer also replaces bit 25; doc 18 row: G8 and G26 requests dropped (done on disk), Narrator notes tagged by moment.
- Canon gaps: G1–G27 closed (one closure paragraph keeps the numbers because docs 03 and 18 cite G8, G22, G23, G26). Two real open items: **G28** (canon §5.8 "the Codex then remembers it" vs the Index naming of triage G-197; proposed text) and **G29** (canon §17.1 stack schedule from `folio_entry_tick` vs §11.6 boss pause and doc 14's `clock_start`; proposed text).
- FOV row (triage Q10): Hor+ detail from old G6 moved into §13.5.
- Questions: 16, 17 answered (DECISIONS 82); 19 renamed "Read menus aloud"; 22 cites doc 14's 32 slots; new 51 (hallucination marking default) and 52 (Narrator note dwell).
- Tests: new 57–63 (hack panel, pickpocket panel, overlay placement, lock-mode intent, World Law discovery, marked hallucinations, text-only narration).
- Invariants: "bits 0..=31 full" replaced by "new sim verbs travel as intents; bit 25 = 0"; added no undiscovered Law on previews, no Narrator audio/subtitles, no mount or durability UI, Mercy marks hallucinations.
- Imports: doc 02 bits 0–24; doc 04 §12.4; doc 09 §13.2–§13.5, §13.7; doc 12 §4; doc 14 §3.8, §5.1a; doc 18 §9.1.
- Lint after: clean (an `input.mount` mention in two Interface rows was reworded to "the retired mount input").

### Batch 6 — small consistency fixes — DONE

- Imports rows updated (see batch 5 note). §10.1 aegis example now states its assumptions (Common, Normal, +0, Coherent; canon §13.2 capacity chain; doc 05 §15.4). Test 3 names an `st.shielded` pool and `ward_mult` 15000 (canon §10.5 step 7 as amended by G-090).
- §11.6 Social tab: voice with non-friends starts off (DECISIONS 75), defaults per doc 15 §8.5.
- Intro: "reserved button bits" → "button bits 26–31". Notifications row lists World Law discoveries and Narrator notes. §12.2 table gains a bit 25 row (always 0; sanitizer replaces it).
- §10.7 petition cost rewritten as `floor_div(c × 100, 100 + devotion)` with `c = mul_bp(cost, 8000)` from Devotion 25.

### Batch 7 — DECISIONS 48 replays and killcam — DONE

- No chapter specified the killcam that DECISIONS 48 lands in docs 14, 16 and 17. New §9.8: `ui.hud.killcam` (300-frame client-side ViewFrame ring buffer, presentation only; 180-tick opt-in prompt; orbit 3,000 mm around `ev.entity_killed.killer`, else the lethal `ev.damage_dealt.source`, else the victim; never pauses; co-op ghost layout after; solo run summary after) and `ui.menu.replay` (opens doc 16 §10.1 `.palr` files from `margin.ledger`; re-simulates from tick 0; refuses on `sim_version`/`content_hash`, warns on `presentation_hash`; stops on checkpoint divergence; speeds ×0.25..×8; seek via `folio_states`; per-slot or free camera; Export; ranked "View verification").
- Exports `ui.hud.` 33→34, `ui.menu.` 29→30. Question 53, tests 64–65.
- Lint after: clean.

### Batch 8 — invasion-time rules (canon §2.5 G-019, §17.9) and Unwritten shops (G-161) — DONE

- P8: nothing pauses while an invader is present (a remote peer exists; doc 16 §2.2 defines solo as zero remote peers).
- Pause menu "Save and quit": with an invader present the solo save completes on the tick `ev.invasion_ended` is emitted (canon §2.5 as amended by G-019).
- §13.9 and test 29: game speed is locked to 10000 in solo runs with `mode.invasion_open`, since an invader may join at any time.
- §9.2: `genre.unwritten` has no shops, so no shop marker or Shop screen (canon §5.2, §17.1 as amended by G-161).
- Seed entry (§11.1) uppercases ASCII letters without locale rules so tr-TR (added by C-17) never produces dotted İ; test 43 checks tr-TR.
- §3.5: HUD text color was unspecified (light skins' `ink` is dark on the `#101010` backplate); HUD text now uses the global tokens (`neutral`, resource tokens, `pencil`), skin tokens color HUD frames only.

## Requests for other chapters (not edited here)

- **00 canon**: add `input.mount`, `input.ctx.mounted`, `ui.hud.mount`, `ui.tip.mount` to §17.7's retired IDs so the linter catches stragglers (docs 01 and 05 still name `input.mount`). G28: §5.8 "the Codex then remembers it" → "the account's Index (doc 14) then remembers it". G29: §17.1 `ERASURE_STEP` "`folio_entry_tick + 43200 + 1800 × (k - 1)`" → "`clock_start + 43200 + 1800 × (k - 1)`, where `clock_start` is the entry tick, or the Editor's death tick in a boss Folio (doc 14)".
- **01**: §7 verb table drops the `input.mount` row (C-30; now an undefined ID); G7 is resolved (B in `input.ctx.casting`, doc 17 §12.4).
- **02**: §5.5 bit 25 `MOUNT_TOGGLE` becomes reserved, must be 0 (C-30); list `ACTION` 26–28 and `WHEEL_ENTRY` 29–31 (C-10); `IntentKind` adds `ShopHaggle { shop }` and doc 17's kinds (C-15) plus the new `SetLockMode { manual }` (doc 17 §10.11); FOV setting 60–120 (C-11).
- **03**: line 272 cites "doc 17 G22" (closed): cite Canon §2.4's intent log instead.
- **04**: §12.4 madness phantoms carry a client-side flag that doc 17's `access.mark_hallucinations` reads (DECISIONS 49; forced on under Mercy).
- **05**: §1.7 "Inputs consumed" drops `input.mount` (bit 25) and the mount mechanics (C-30).
- **06**: §16.4 example "Devotion 50 pays floor_div(60 × 100, 150) = 40" omits the `skill.devotion.m25` × 8000 the same paragraph applies; with it the cost is 32 (doc 17 §10.7 shows 32). Apply C-28 if not done.
- **09**: §13.2/§13.3 remove the lockpick break roll and durability (C-31); `noise.mount.*` rows (C-30); read per-slot `lock_mode` from `SetLockMode { manual }`; expose to the owner's client each targeted device's verbs, `chance`, `ticks`, `hack_fails`, and each pickpocket target's stealable items with chance and ticks; the panels are `ui.hud.hack` and `ui.hud.pickpocket` (doc 17 §10.12).
- **11**: §12.3 delete "Repair to max durability" and "Mount mending" (C-31, C-30).
- **12**: define a trigger condition per World Law and an event when a Law first fires in a Folio, naming the Errata it affected, so doc 17 §7 can record account discovery (DECISIONS 56, canon §5.8); §5.7's "the active Laws" preview field shows only Laws the viewing account has discovered.
- **14**: C-04 (drop † marks and the Mercy Grade exclusion); C-03 (Prologue L3); §3.8 Erasure onset 43,200 ticks, cap 10 stacks, one hunter per Folio with no 5,400-tick repeats (canon §11.6, §17.1; DECISIONS 26); `RunSummary` `mercy_ever_on`, `min_game_speed_bp` and the ranked rule for speeds below 10000 (G-201); keep the account's discovered-Law set with the Index (§5.5).
- **15**: §8.5 adult default "voice push-to-talk, party only" should start voice off with non-friend party members (DECISIONS 75).
- **16**: C-09 light byte (values after canon §2.3 light overrides); C-10 sanitizer replaces a command with `ACTION` 6 or 7 or bit 25 set; nothing pauses and game speed is 10000 while an invader is present.
- **18**: tag each `loc.nar.note.*` with the moment that shows it (doc 17 §16.4); citations of "doc 17 G8, G23, G26" still resolve (closure paragraph) but can point at doc 17 §2.2, §2.4, §10.10.

## Final state

- Lint: `== 17  words=30908` with no findings (no undefined, foreign, retired, JSON or hedge issues); no multiply-defined ID involves doc 17.
- §16.4: CJK locales set Narrator notes upright (§14 forbids CJK italics).

## Verification

Adversarial verifier, 2026-10-11. Every check re-read the chapter on disk after the fixer's batches; fixes were applied with one Python script whose 40 anchors each matched exactly once. Lint after: `== 17  words=32008`, no findings; no multiply-defined ID involves doc 17.

### 1. Triage rows that name chapter 17

| Row | Verdict | Chapter text that satisfies it |
|---|---|---|
| G-011 | PASS | P11: "per-second rates below 10 display units as `floor(x / 100)` tenths with one decimal"; §5.6 "673 mHP/s shows 0.6". |
| G-016 | PASS | Intro: "the replay log records both (Canon §2.4)"; doc 02's `IntentKind` table already lists doc 17's 11 kinds plus `SetLockMode`. |
| G-051 | PASS | Header "Conforms to canon-1.11"; mounts and durability removed (C-30, C-31 below). |
| G-052 | **FIXED** | The fixer applied G-052 ("Nobles"), but DECISIONS 90 overrules triage Q9 and canon §5.2 row 1 keeps "Crowns". §4.3 row 13 now reads "≈ 160–340 Crowns" in Mythic Fantasy (DECISIONS 90). |
| G-089 | PASS | §13.1 "Damage step 6c (Canon §10.5 step 6c, after step 6b and before ward absorption)", voluntary costs excluded. |
| G-115 | PASS | §12.9 "Canon §11.8: input is never synthesized beyond what the player chose". |
| G-178 | PASS | §10.4 Mark "sends `MarkTarget { target }`" (doc 02 lists it under doc 16). |
| G-197 | **FIXED** | §7 Index row said "account item codex"; now "the account's item encyclopedia (…; "Codex" names only the pass, triage G-197)". The §7 heading "Journal and Codex knowledge" is left as is (see Unresolved). |
| G-198 | PASS | §11.6 "Generated text (the Scribe toggle `flag.scribe.enabled` …)". |
| G-200 | PASS | §12.4 "Casting context … base-layer B is `input.cast_cancel`". |
| G-201 | PASS | §12.7 "adds no in-simulation magnetism"; §13.9 ranked rule; Interface row 14 "ranked eligibility excluding game speeds below 10000 (G-201)". |
| G-210, C-03 | PASS | §15 "The Prologue's room L3 … teaches harvesting a resource node with the pick lying there and placing a bridge-span placeable … it never mines terrain"; §10.8 uses doc 14's `margin.plot` (doc 14 §5.1 names it "The Trophy Alcove", so the ID is current; no building-plot text remains). |
| G-211, C-04, Q2 | PASS | §13.1 "Never blocks: achievements, Codex XP, Ink, Quires, Reliquaries, milestones, Recollections and Grade unlocks are all unaffected (DECISIONS 82 …)". |
| G-212, C-10 | PASS | §12.2 table: `ACTION` 26–28, `WHEEL_ENTRY` 29–31, "6, 7 reserved (the sanitizer rejects them)"; bit 25 row. |
| G-213, C-11, Q10 | PASS | §13.5 "60..=120 … (DECISIONS 92, triage Q10) | 90". Recomputed: 64.01° vertical, 96.03° at 16:9, 94.54° at 120, 39.68° at 60. |
| G-214, C-17 | PASS | §14 lists the 15 locales; identical to doc 18 §9.11. G7 closed in the closure paragraph. |
| G-215, C-09 | PASS | Interface row 16 "`light_sky` and `light_block`, after light overrides, in one byte". |
| G-216, C-28 | PASS | §6.5 `SetCastItemTarget { uid }`; §12.2 "a channel or charge entry is only selected, and the player holds `input.cast`". |
| C-15 | PASS | §9.2 "via `ShopHaggle { shop }`"; no `Haggle {` left. Haggle example recomputed (score 28, `haggle_bp` 1900). |
| C-18 | PASS | §10.7 "`r1 = range(180,000)`" and "the gate `prayer_timer <= 7200`"; no `1,800,000` or `36000` left. |
| C-30 | PASS | No `cat.mount`, `st.mounted`, `input.mount`, `input.ctx.mounted`, `ui.hud.mount`, `ui.tip.mount`; §10.2 is a retirement stub. |
| C-31 | PASS | §4.3 "the card has no durability row"; §9.2 "there is no repair service"; §10.11 no break chance. |
| §4.1 stale G8, G9, G20, G25, G26 | PASS | Closure paragraph: "G8, G9, G20, G25, G26: stale on disk (triage §4.1)". |

### 2. DECISIONS rows

- Changed rows: invasions (§3.2 invasion banner, §11.6, §13.9 speed lock), limited building (§6.2, §10.8), softer Erasure (§3.2 clock: onset 43,200, `+1800 × k`, cap 10, one hunter), Echo cast (**FIXED**: §4.2 paper doll now shows only the slots doc 04 §13 gives `echo.fenrir`'s quadruped and `echo.jormungandr`'s serpentine plans), hybrid glyph learning (**FIXED**: `ErratumSelect` now offers doc 14 §5.7's 2 Glyph Library picks with refusal reasons and the Daily `base` pool, which doc 14's Interface row asked of doc 17), micro-voxels (no UI conflict), text-only narration (§16.4; voiced resident greetings stay subtitled per doc 18 §8.4), USD 19.99 (no base price in the chapter; no 24.99), procedural ending (**FIXED**: §9.7 now plays doc 18 §8.5a's closing manuscript before the summary), mounts cut (PASS), World Laws (§7 discovery rule), pact curses (**FIXED**: §9.4 shows the cursed capsule as Ominous with its pact hidden and the cursed share 9.0000%/10.0000%/0.0000%, doc 11 §11.2), no durability (PASS), 12+ enemies (no count in the chapter), crippled-limb severing (no conflicting text).
- Rows 77–88: 77 PASS (§11.1); 78 **FIXED** (new Margin dialogue box `ui.menu.margin_dialogue`, which docs 14 and 18 cite as doc 17's; arc beats text only); 82 PASS; 83 **FIXED** (Drop Lucre in the Satchel footer via `DropLucre { amount }`, trade never carries Lucre); 87 PASS; 79, 80, 81, 84, 85, 86, 88 land elsewhere and nothing in the chapter contradicts them.
- Rows 89–108 that name doc 17: 90 **FIXED** (Crowns); 92 cited; 94 **FIXED** (`access.gore` default `ink`, `full` = red blood, `minimal`; question 44 answered; test 55 checks it); 97 **FIXED** (creation `Cosmetics` page: 12 presets per kin with palette and marking colors, no sliders); 98 Imports now say PC only through 1.0 with Steam Deck Verified. 95, 96, 104, 108: the owners have not specified the mechanics or services (doc 16 O2; doc 14 has no Second Edition flow; doc 03 §5.9 still mantles at most 1,600 mm and no chapter defines the sprint-slide or grappling tool), so §11.6 gains a "Later screens" paragraph with the ledger's binding rules and the Interface rows for docs 03, 14 and 16 name the dependency.

### 3. Chapter brief

C-15, C-17, C-18 PASS (above). G7, G8, G9, G20, G25, G26 closed. Mount IDs and durability UI gone. Text-only narration PASS (§16.4, test 63). World Laws discovered in play PASS (§7, §9.1, test 61). Q2 PASS. Q10 PASS. Walkable 3D Margin hub PASS (§11.1).

### 4. Leftover grep

`mount`/`mounted` hits are retirement sentences only (§10.2, §12.2 bit 25, Interface rows, invariant). No `ride`, `saddle`, `cavalry`, `Delving`, `voice actor`, `voiced` (except the new statement that no dialogue-box line is voiced), `VO`, `24.99`, `Item Codex`, `5400`. "durability", "repair", "wear", "break" hits are retirement sentences, the construct's self-repair heal, `ev.ward_broken` and text line breaks. "Redactor" names only the invader (Canon §17.9; G-177 keeps it). `margin.plot` is doc 14's alcove facility ID. "Crowns" is correct (DECISIONS 90); "Nobles" was the real leftover.

### 5. Lint

`spec_lint.py --chapter 17`: no findings before or after. `--json` `multiply_defined`: no entry involves 17.

### 6. Internal consistency

Recomputed in Python: prayer odds (76.0000%, 53.3330%/46.6670%, 0.0000%), petition costs 50 and 32, Erasure clock (3,600; 1,800; 59,400), lock 8400, hack 7400 with 96/76 ticks shown 1.6/1.3 s, pickpocket 5400 with 63 ticks shown 1.0 s, Mercy 9,136, 267 ticks, 1,334, 680, 2,550, command encodings, FOV, overlay rectangle 440,216–840,360 and its clearance of the invasion banner at Large scale with TV-safe margins. Tables well formed (column counts checked); tests 1–69 and questions 1–53 numbered contiguously; `ui.menu.` 31, `access.` 51 and every other Exports count match their lists, and every `ui.`, `input.` and `access.` ID in the body is exported. Canon gaps G28 and G29 are still open on disk (canon §5.8 still says "the Codex then remembers it"; §17.1 still keys `ERASURE_STEP` on `folio_entry_tick`).

### Verifier fixes

- §4.3 row 13: "Nobles" → "Crowns" (DECISIONS 90).
- §7 Index row: "item codex" → item encyclopedia (G-197).
- §4.2: non-humanoid Echo slots (doc 04 §13); Drop Lucre (`DropLucre { amount }`, DECISIONS 83).
- §5.7 `Cosmetics`: 12 presets per kin, no slider editor (DECISIONS 97).
- §9.2: stale "(which replace doc 11's identify and uncurse rows)" dropped; doc 11 §12.3 lists `appr.svc.*` itself.
- §9.4: cursed capsule and cursed share (DECISIONS 60); test 19 extended.
- §9.6: trades never carry Lucre.
- §9.7: closing manuscript (DECISIONS 46); test 69.
- §10.11: slip chance shown (doc 09 §13.2 replaced the break roll with a slip; the fixer dropped the break chance but showed nothing in its place); the manual dial also runs manual hacks and Seizes (doc 09 §13.3); §10.12 and the `input.ctx.lockpick` row follow.
- §11.1: Margin dialogue box `ui.menu.margin_dialogue` (DECISIONS 78, doc 18 §8.4, doc 14 §5.6); Glyph Library picks in `ErratumSelect` (DECISIONS 33, doc 14 §5.7); tests 66–67; Exports `ui.menu.` 30 → 31.
- §11.6: "Later screens" for DECISIONS 96, 104, 108.
- §13.5: `access.gore` default `ink` (DECISIONS 94); FOV row cites DECISIONS 92; question 44 answered; test 55.
- Imports: ledger rows 33, 46, 60, 78, 83, 90, 92, 94, 97, 98 (row 6 superseded by 98), open rows 95, 96, 104, 108; doc 04 §13, doc 11 §17, doc 13 §13.6, doc 14 §4.7, §5.6, §5.7, doc 18 §8.4, §8.5a.
- Interface assumptions: doc 03 (movement kit through existing bits), doc 04 (creator presets), doc 11 (`DropLucre`, cursed capsule), doc 14 (Glyph Library picks, arc beats, manuscript inputs, Second Edition flow), doc 16 (trace service and vote path), doc 18 (Margin cast lines, closing manuscript, presets).

### Further requests for other chapters

- **02**: §8.4 (line "doc 17 §8.4 owns the setting") and the Imports row both cite "doc 17 §8.4" for the FOV setting; the setting is doc 17 §13.5.
- **03**: §5.9 mantle heights up to 2,000 mm (DECISIONS 95; now `h ∈ (550, 1600]`); define the sprint-slide (doc 17 assumes a `CROUCH` press while sprinting, no new bit).
- **05, 11**: define the grappling tool item and its use (DECISIONS 95); doc 17 assumes a quickbar item.
- **04, 18**: define the 12 creator presets per kin and their IDs (DECISIONS 97).
- **14**: specify the Second Edition flow (choice at the Ur-Text clear, wipe as non-clear, Ironman reward; DECISIONS 100–108) so doc 17 can add its screens.
- **16**: specify the trace service and vote path (O2; DECISIONS 96, 104).
- **18**: line 1405 notes "doc 17 still defaults to `full`"; it now defaults to `ink`, and the Margin dialogue box exists as `ui.menu.margin_dialogue` (§11.1).
- **21**: if §7's heading is renamed to drop "Codex", update `tools/roadmap/headings.json` and WP-055 together.
