# Fix log: chapter 06 (Magic and Spellcrafting)

As of 2026-10-11. Fixer for `spec/06_MAGIC_SPELLCRAFTING.md`, moving it to canon-1.11. Edits are made by Python batches that assert each anchor occurs exactly once; each insertion is confirmed by grep. This log is appended after each batch so an interrupted run can resume.

Baseline lint (`tools/spec_lint.py --chapter 06`): hedges {'could': 1}; no undefined, foreign, retired or JSON issues; no doc 06 ID in `multiply_defined`.

Inputs read: DECISIONS.md (rows 1 to 100 and the orchestrator rows), canon-1.11 (already applied on disk, see `audit/canon_1_11_notes.md`), triage rows naming doc 06 (G-021, G-035, G-057, G-067, G-105, G-106, G-107, G-108, G-116, G-117, G-127, G-145, G-184, G-187, G-206, G-216; C-10, C-18, C-28, C-30, C-31; §4.1 doc 06 G5), canon amendments touching magic (G-016, G-055, G-056, G-090, G-094), reconciler notes (`notes_magic`, `notes_events_sim`, `notes_character`, `notes_materials`, `notes_items`, `notes_ident`), and the Interface rows that docs 01, 03, 04, 05, 07, 09, 10, 11, 13, 14 and 17 address to doc 06.

## Batches

### Batch 1: header and Imports (applied, 21 edits)

- Header and the canon Imports caption now read canon-1.11 (task item 6).
- Canon Imports rows updated for the amendments that touch magic: §2.4 intents in the replay log (G-016, for `SetCastItemTarget`); §3.3 `seq` allocation with doc 06's reserved ranges (G-021); §4.2 stance bases and `body_move_bp` (G-035); §4.3 `block.tag.destructible`, fixed terrain stops projectiles, no item durability (G-036, G-037, DECISIONS 65); §5.11 failure kinds including failed potions (G-056); §6.4 row 29 per producing stage (G-067); §10.5 step 7 and §10.8 two batches (G-090, G-094); §13.8.1 casting implement (G-108); §17.1 `GLYPH_LIBRARY_START` (DECISIONS 33), §17.5 spell friendly-fire classes (G-105), §17.7 retired mounts (DECISIONS 55); §18 events registered by doc 03 §14.3 (G-184).
- The decisions sentence now names items 15, 16, 33, 55 and 65.
- Chapter Imports: new rows for docs 02 (`IntentKind`), 07 (altars, `block.conjured`, `placed_meta`) and 17 (command bits, wheel, prayer menus); doc 03 row uses doc 03's schedule (`aether_tick` in `sys.resources` 26a, notes_events_sim) and drops the per-tick `seq` reset (G-021); doc 05 row adds the three archetypes doc 05 §5.4 exports (§4.1 stale G5) and `spell_pot_bp`; doc 10 row adds the wand pacts, `identify_one_unknown` and §7.6 uncursing (doc 10's Interface row for 06); doc 11 row adds `item.tool.blank_text`; doc 14 row adds the Glyph Library (DECISIONS 33).

### Batch 2: §2 and §3 (applied, 16 edits)

- §2.1: `aether_tick` runs in `sys.resources` (doc 03 system 26a), after the tick's cast machines in `sys.actions` (7); doc 03 owns the schedule (notes_events_sim).
- §3.1: three "Canon gap G16" references now cite Canon §13.8.1 as amended (G-108).
- §3.2: C-28 applied. Instant and cast_time wheel entries cast on release (doc 17 §12.2 sets bit 9 in the selection command); channel and charge entries only set the active spell and the player holds `input.cast`; entries needing an item target only select, and doc 17 §6.5 sends `SetCastItemTarget { uid }` then the cast press. The replay bullet says the item target travels as an intent recorded by the replay log (Canon §2.4, G-016).
- §3.3: new rule fixing a compiled spell's cast type (channel by beam root, charge by `glyph.mod.charged`, else instant at `ct = 0`, cast_time at `ct >= 1`), which the wheel rule depends on.
- §3.5 row 12: "mount input" removed (DECISIONS 55, C-30).
- §3.6: Canon gap G6 sentence replaced by Canon §4.2's stance-base rule with `body_move_bp` (G-035).
- §3.7: interruption `seq` comment cites Canon §3.3's reserved range (G-021); `ev.cast_interrupted` cites doc 03 §14.3 (G-184).
- §3.8: tome and staff rows cite doc 05 §5.4 exports; stale G5 references removed (§4.1).
- §3.9: new `CastError::NoTarget` check and prose for the lore-essence item target (C-28); the misfire comment adds `curse.pact.wildfire` (doc 10 §7.8, doc 10's Interface row for 06).

### Batch 3: §5.3, §6, §8.1 (applied, 10 edits)

Note: the shared scratchpad is used by other fixers (my first `b1.py` was overwritten after it ran). My scripts now live in `scratchpad/ch06_fixer/` (`apply06.py`, `b3.py`, ...).

- §3.2, §3.9, §5.3: C-28 item target. Only `glyph.ess.appraise` and `glyph.ess.psychometry` with the `self` form need one (omen is an area effect, doc 10 §5.10). The target is the per-Erratum saved field `cast_item_target: Option<u64>`, set by the preceding `SetCastItemTarget { uid }` intent (doc 17 §6.5), applied in `sys.input_apply` before `sys.actions`; kept until the next intent, cleared when the item leaves the inventory; unset or refused target gives `NoTarget`; a target lost before resolution makes it do nothing.
- §6.2: stale request to doc 04 replaced (doc 04 §5.3 now cites the cost factor).
- §6.3: "Canon gap G2" replaced by amended Canon §6.4 row 29 (G-067).
- §6.4: `mag_chain` gains the source item's factor (blessed or cursed text, `curse.pact.wildfire` `10000 + m`, doc 10 §7.8) so the order is exact; "Canon gap G14" removed (G-107, chapter rule stands).
- §8.1: hybrid glyph learning (DECISIONS 33, Canon §17.1 `GLYPH_LIBRARY_START = 2`): the Lexicon is per run; the account Glyph Library (doc 14 §5.7) receives every glyph learned; each Erratum learns up to 2 library picks right after the run-start set (new learning-source row, source 0, origin = Origin genre, no draws); refusal rules and the baseline-run `base` offer match doc 14 §5.7. `ev.glyph_learned` cites doc 03 §14.3 (G-184).

### Batch 4: §8 to §19 body text (applied, 26 edits)

- §8.3: blessed and cursed text factors placed as the last `mag_chain` factor (one exact position, no double application to heal and ward); inscription consumes `item.tool.blank_text` (doc 11's consumable, doc 11 Interface row for 06) or scrapes an identified text; `block.tag.destructible` in the demolition row.
- §8.4: pact wands apply `curse.pact.overcharge` or `curse.pact.wildfire` on every zap by doc 10 §7.8 (doc 10's Interface row for 06).
- §8.6 and §13.1, §13.2: `destructible` written as the canon tag `block.tag.destructible` (G-036 canon wording).
- §9.4: `ench.spellstore` never holds a channel or charge spell (a single activation, so every on-use wheel entry casts on release, C-28).
- §11.2: `note_contradiction_failure(t)` named as the one per-character counter that docs 05, 07 and 08 call and doc 12 cites (G-057).
- §13.1: `in_shadow_area(cell)` query for doc 09 (doc 09's Interface row for 06).
- §13.2: wall casts place doc 07's `block.conjured` with the `placed_meta` conjured flag, removed at expiry with `grid.cause.expired` (doc 03 and doc 07 Interface rows).
- §14.1: "Canon gap G9" removed; `dispellable: bool` is this chapter's rule (G-116).
- §16.1: Grand Engineer taboo no longer reads item durability (C-31, DECISIONS 65): it is now "each failed gadget activation by the Erratum (the Canon §5.5 spark, doc 07): 20". Cosmic Horror boon names doc 10's `identify_one_unknown` with `appr.reason.prayer`. Altars defined by `block.tag.altar` (doc 07): worldgen `block.altar` (fixed terrain, never breaks) and the placed `block.shrine_lamp` and `block.shrine_lamp_out`; the 200-favor loss applies to breaking a shrine lamp (G-145, notes_materials).
- §16.2: "Canon gap G12" replaced by Canon §12 per-entity resources (G-127); `ev.favor_changed` cites doc 03 §14.3.
- §16.3: hedge "could" removed from the `r1` balance note; need (4) uncurses by doc 10 §7.6. §16.4 `prayer.absolution` cites doc 10 §7.6. §16.5 offering targets the two altar kinds.
- §18, §19.1: events cite doc 03 §14.3 (G-184); `ev.spell_cast` notes `spell_hash` and `channel_ticks` as present; note that `ev.glyph_learned` source 0 covers the library picks and `ev.favor_changed` cause 6 is a broken shrine lamp.

### Batch 5: Interface assumptions (applied, 9 rows rewritten)

- Rows 03, 05, 07, 09, 10, 11, 14 and 17 now mark what the owner already provides as "(present)" and keep only real requests under "Still requested".
- 03: `aether_tick` in `sys.resources`, intents in `sys.input_apply`, events registered in §14.3 (G-184), conjured-block expiry; still requested: shadow areas lower every light read by 8.
- 04: the §6.2 cost factor is cited (present).
- 05: archetypes exported (stale G5), stances and `spell_pot_bp` present; still requested: spell Contradiction potency and Edge (and school, `move_id`) on `HitRecord`, used at steps 2.5 and 2.6.
- 07: altar blocks, `block.conjured`, `mat.tag.arcane_reagent`, gadget spark (new taboo).
- 10: wand pacts, `identify_one_unknown`, §7.6 uncursing, no services at a shrine lamp.
- 11: `item.tool.blank_text` present; still requested: `item.text.inscribed` row names a scraped text as a source.
- 14: Glyph Library (DECISIONS 33) present.
- 17: C-28 wheel bits and `SetCastItemTarget` present; C-18 odds and gate present (stale "doc 17 uses 1,800,000 and 36000" and "W1 burning 54" text removed); still requested: the target list opens only for appraise and psychometry with `self` (omen has no item target).

### Batch 6: Exports, Canon gaps, Acceptance tests, Invariants (applied, 9 edits)

- Exports: "(Canon gap G4)" replaced by Canon §19 (G-187); new paragraph listing exported functions and state (`compile`, `note_contradiction_failure`, `in_shadow_area`, the §18 queries, `cast_item_target`). No foreign ID is declared there.
- Canon gaps: G1 to G16 deleted as closed (one closing sentence maps each to its triage row: G-206, G-067, G-184, G-187, G-105, G-127, G-035, G-021, G-116, G-117, G-106, G-107, G-108; §4.1 stale G5). One real open item kept as new G17: Canon §17.1 `GLYPH_LIBRARY_START` vs Canon §13.6 baseline runs (shared with doc 14 gap 2); assumption = the 52 `base` glyphs are offered.
- Acceptance tests: test 31 uses `block.tag.destructible` and `block.conjured`; new tests 51 (wheel release, C-28), 52 (item target, C-28), 53 (Glyph Library, DECISIONS 33), 54 (no durability, new taboo, C-31), 55 (altars, G-145), 56 (spellstore refuses channel and charge).
- Invariants: `origin_genre.` spelling (notes_character); `block.tag.destructible`; new lines for the wheel rule, item target, library picks, Lexicon vs Library, no durability, schedule placement and spellstore.

Lint after batch 6: `== 06 words=36525` with no findings (hedges {} ; no undefined, foreign, retired or JSON issues); no doc 06 ID in `multiply_defined`.

### Batch 7 and 8: consistency touches (applied, 4 edits)

- §4.2: "canon-1.5's length-prefix rule" reworded to cite Canon §2.5 without an old version number.
- §3.2: Grimoire step presses defined (bits 19 and 20; next or previous filled, unlocked slot, wrapping; press rule excludes the 19+20 wheel pair), as doc 17 §12.2 and its test 44 assume ("doc 06 counts presses this way").
- §19.2: sentence naming seq 0 and the Canon §3.3 ranges reserved for doc 06 (payload seq at most `0x42FF`) (G-021).
- §8.1: library picks are learned at doc 14 §2.2 step 9, after the run-start set and the kit (doc 14's order).

Checks: every table row has its header's column count; lint clean.

### Batch 9 (applied, 5 edits)

- §5.2 bolt and orb rows: a bolt or orb cast through a `mfr.oneiric.somnambule` implement or wand passes destructible and non-opaque blocks after 10000 mm of flight; fixed terrain still stops it (DECISIONS 88, Canon §5.9, C-27, doc 11's "spell projectiles included").
- §16.1 Grand Engineer taboo revised again so doc 06 can evaluate it without a new hook from doc 07: "each of the Erratum's Device-register spells (§10.10) that fizzles or wild-surges: 10" (supersedes batch 4's gadget-spark wording; still C-31, DECISIONS 65). Interface row 07 drops the spark mention; test 54 rewritten (10 favor, cause 2).

Lint after batch 9: clean (`== 06 words=36667`, no findings).

## Requests for other chapters (not edited here)

- **Doc 05** (§1.5 `HitRecord`, damage step 2.5 and 2.6): for spell and prayer hits carry the spell's Contradiction potency and Edge (doc 06 §11.1: maximum over the root core glyphs, or the payload stage's own core) and use them at steps 2.5 and 2.6 in place of `potency(h.item_uid, att)` and `edge_bp(h.item_uid, att)`, which read the implement; also carry the stage school (doc 06 §7.4 XP) and `move_id` = the stage's form glyph `id32`.
- **Doc 17** §6.5: the lore target list opens only for an entry whose spell has `glyph.ess.appraise` or `glyph.ess.psychometry` with the `self` form; `glyph.ess.omen` has no item target (doc 06 §5.3).
- **Doc 11** `item.text.inscribed` row: "created only by inscription (doc 06 §8.3) from `item.tool.blank_text` or a scraped identified `cat.text`".
- **Doc 03** §10.6 and its Interface row for 06: every light read inside a doc 06 shadow area (`in_shadow_area(cell)`, doc 06 §13.1) is lowered by 8, floor 0, at read time like the light overrides.
- **Canon (00)** §17.1 `GLYPH_LIBRARY_START`: add "Runs on `unlock.daily_baseline` offer the 52 glyphs of doc 06's `base` pool in place of the account library." (doc 06 G17, doc 14 gap 2).
- **Doc 10** §5.9 and gap 21: write "a `block.shrine_lamp` (doc 07; doc 06 §16.1)" for "an Erratum-placed altar (doc 06 §16)" and close gap 21 (G-145: the placed altar is the shrine lamp, a `cat.placeable` light source).
- **Doc 02** (C-15, already a triage row): `IntentKind` includes `SetCastItemTarget { uid }` and `Pray { patron, kind }`, which doc 06 §3.2, §5.3 and §16 consume.

## Final state

- Header: "Conforms to canon-1.11".
- Lint: `python3 tools/spec_lint.py --chapter 06` → `== 06 words=36667` with no findings; `--json`: no doc 06 ID in `multiply_defined`, no missing sections, no undefined, foreign, retired or JSON issues, hedges {}.
- Open canon gap: G17 only.

## Verification

As of 2026-10-11. Adversarial verifier pass over `spec/06_MAGIC_SPELLCRAFTING.md` after batch 9. Edits were made by Python scripts that assert each anchor occurs exactly once (`scratchpad/ch06_verifier/v1.py`, 18 edits; `v2.py`, 2 edits). Nothing was committed.

### Check 1: triage rows that name chapter 06

| Row | Verdict | Chapter text that satisfies it |
|---|---|---|
| G-021 (06 G8) | applied | §3.7: "`0x2000 \| (k & 0x0FFF)` … Canon §3.3 reserves 0x2000..=0x2FFF for doc 06"; §19.2: "`0x2000..=0x2FFF` (interruption), `0x3000` (`st.confused`) and `0x4000..=0x4FFF` (payloads, at most `0x42FF`) are the allocations Canon §3.3 reserves for this chapter" |
| G-035 (06 G6) | applied | §3.6: "Canon §4.2 lets a doc 05 stance base replace base speed … (row 11, then doc 04's `body_move_bp`, then statuses)" |
| G-055 | applied | §2.2: "`dep = {aether_axiom}`, or `{arcana}` when the aether axiom is `none`"; §9.2: enchantments are "a component with `dep = {arcana}`" |
| G-057 | applied | §11.2: "`note_contradiction_failure(t)` (§3.9) is the one per-character counter of these failures: docs 05 (jams), 07 (gadget sparks) and 08 (failed potions) call it, and doc 12 … cites it" |
| G-067 (06 G2) | applied | §6.3: "Canon §6.4 row 29 multiplies the GLYPH term of each glyph of the stage that produces the effect" |
| G-105 (06 G11) | **fixed** | the deleted G11 assumption had left the classification only in an Imports caption. §5.3 Polarity now reads: "every spell hit, including the hits of orbs, cones, novas, runes and chains, is a direct hit at `FF_DIRECT_BP`, while the fire, surfaces and liquids a spell leaves are area damage at `FF_AREA_BP` (triage G-105)" |
| G-106 (06 G13) | applied | §6.5: "`NPC_SPELL_SCALE_BP = 4500`"; doc 13 uses the same value |
| G-107 (06 G14) | applied | §6.4 status power: "then Canon §11.2 from `S(src_lvl)` on, unchanged, because `p` already holds `spell_pot`" |
| G-108 (06 G16) | applied | §3.1: text, on-use and prayer rows cite Canon §13.8.1; prayer: "`ilvl` of an equipped `item.trinket.holy_symbol` (doc 11), else the active implement's `ilvl`, else `max(1, d_current − 3)`" |
| G-116 (06 G9) | applied | §14.1: "Each status instance carries `dispellable: bool`, set at application" |
| G-117 (06 G10) | applied | §15 `st.dread`: "NPCs have no composure: doc 13 raises flee utility, no numeric effect" |
| G-127 (06 G12) | applied | §16.2: "`res.favor` is per Erratum and per patron … (Canon §12: resources are per entity)" |
| G-145 | applied | §16.1: "the placed `block.shrine_lamp` with its unlit twin `block.shrine_lamp_out`, a `cat.placeable` light source"; Interface 10: "no `appr.svc.*` at a `block.shrine_lamp`" (doc 10 now says the same) |
| G-184 (06 G3) | applied | §19.1: "Doc 03 §14.3 registers these events with exactly these payloads (Canon §18)" |
| G-187 (06 G4) | applied | Exports: "Polymorph templates use `summon.poly.` inside `summon.`, which Canon §19 gives this chapter" |
| G-206 (06 G1, G5, G7, G15) | applied | §3.1 step 0 uses the source item's material `dmg_bp`; §3.8 "doc 05 §5.4 exports it"; §16.3 `r1` balance note; §13 limited building |
| G-216 → C-28 | applied | §3.2: "Instant and cast_time entries cast on release …"; "Channel and charge entries only set the active spell. The player then holds `input.cast`"; §5.3: the field "holds the uid of the preceding `SetCastItemTarget { uid }` intent (doc 17 §6.5)" |
| G-051 | applied | header "Conforms to canon-1.11"; no retired mount ID; durability removed (C-31) |
| G-037 / C-27 | applied | §5.2 bolt: "a bolt that has flown 10000 mm passes destructible and non-opaque blocks without impact … fixed terrain still stops it" |
| C-10 | no change needed | §3.2 uses `ACTION` bits 26 to 28 and `WHEEL_ENTRY` bits 29 to 31 |
| C-18 | applied (doc 06 authoritative) | §16.3 `r1 = r.range(180_000)`; §16.4 `prayer_timer <= 7200`; doc 17 §10.7 now uses both |
| C-31 | applied | §16.1 Grand Engineer taboo: "each of the Erratum's Device-register spells (§10.10) that fizzles or wild-surges: 10"; test 54; invariant "N! rule reads item durability" |
| §4.1 doc 06 G5 | applied | Canon gaps holds only G17; the closing sentence records G5 as closed |

### Check 2: DECISIONS rows

- 15 invasions: Imports §17.9; Interface 13 "`enemy.editors.invader` casts no spells and is a `rank.hunter` target", matching Canon §17.9 and §17.3. Agrees.
- 16 limited building: §13.1 and §13.2 break only `block.tag.destructible`; conjured walls are temporary barricades. Agrees (block-damage units fixed under check 6).
- 26 softer Erasure: only `st.erasure` stacks (Red Ink surge, `patron.redacted`) and `summon.misprint`, a `fac.editors` member that erases placeables as Canon §11.6 allows; no repeat-spawn text, no 5400. Agrees.
- 28 Echo cast: `echo.wren_long_index`, `echo.preacher_amos`, `echo.briar_thornwood` and `echo.drowned_clerk` match doc 14 §6.3's trait texts. Agrees.
- 33 hybrid glyph learning: §8.1 Glyph Library paragraph and table row. **Fixed:** the joining rule now names doc 14 §5.7's departure merge and the Prologue and Apocrypha exceptions instead of "at run end" alone.
- 37 micro-voxels: **fixed.** Line of effect read "crosses no block with collision"; it now tests 250 mm boxes through doc 03 §5.18 `raycast_blocks`. Imports gains Canon §4.4; the decisions sentence names item 37.
- 38 text-only narration, 42 price, 46 procedural ending, 69 roster: no text here. Agrees.
- 55 mounts: §3.5 row 12 has no mount input; Imports records the retirement. Agrees.
- 56 World Laws: §10 riders are never `law_bound`; nothing reads a law preview. Agrees.
- 60 pact curses: §3.9, §6.4 and §8.4 match doc 10 §7.8 (`curse.pact.wildfire` adds `m` to the failure chance and `10000 + m` to positive magnitudes). Agrees.
- 65 durability: see C-31. Agrees.
- 77, 78, 80, 81, 82, 86: no text here. 79: glyph families come from doc 15 Scripts, which are earned; doc 14 counts the Glyph Library inside the 80 hours. 83: the Witness wrath takes carried Lucre per Erratum. 84: the chapter states no severing threshold, only reattachment. 87: names and skins never enter simulation state (§17). 88: §5.2 bolt row. All agree.
- 85 co-op skill growth: **fixed.** §7.4 used `floor_div(1500 × dmg, max)`; it now uses Canon §7.1's `norm_mhp = max(1, floor_div(target_max_mhp × 10000, party_hp_bp))`, as doc 04 §5.2 does.

### Check 3: brief

- C-28: applied (above). `SetCastItemTarget` is applied in `sys.input_apply` before `sys.actions` (§5.3), recorded by the replay log (§3.2), and tested (tests 51, 52).
- C-18: doc 06 values unchanged.
- Stale G5: deleted.
- Durability ×1: removed.
- Decision 33: §8.1, test 53, invariants. G17 stays open: Canon §17.1 still has no baseline-run exception.

### Check 4: leftover grep

Hits for `mount` (`spell.mountain_palm`, "Mountain Force", skins), `ride`/`Rider` (patron "The Rider at the Crossroads", register riders), `repair` ("Nanorepair" skin), `wear` ("wearer"), `broken` ("Unbroken Thread" skin), `breaks` (block breaking only) and `durability` (the retirement records in the decisions sentence, test 54 and the invariant) are all legitimate. No hit for saddle, cavalry, Delving, voice actor, voiced, VO, 24.99, Item Codex, Crowns, Redactor, `margin.plot`, building plot or 5400.

### Check 5: lint

`python3 tools/spec_lint.py --chapter 06`: `== 06 words=37081`, no findings. `--json`: no missing sections, no undefined, foreign, retired or JSON issues, hedges {}; `multiply_defined` is empty.

### Check 6: internal consistency

- Recomputed 130 worked numbers in Python (`scratchpad/ch06_verifier/recompute.py`): W1 to W12, Ref-A and Ref-B costs, potencies, damage and status powers, the W3 drain over 300 ticks, the W11 repayment, siphon and leech, tests 9, 10, 32, 41, 44 to 47, sacrifice and inscription. All match. The four `fnv1a32` values, the W1 `spell_hash` and the four name variants also match.
- **Fixed:** §13.1 gave block damage as `floor_div(block_power × form magnitude, 10000)` "in doc 07 block-HP points", a unit doc 07 does not have (block HP is mHP, doc 07 `block_hp`), so a force bolt dealt 6 mHP. It now calls doc 03 §13.2 `damage_block` with the hit's step-0 `B` and `block_bp = 1000 × block power` (largest power among the stage's essences), with a worked example (Ref-A's `bolt force` at Folio 3 deals 15682 to a 47524 `block.cracked_wall`, which breaks on the 4th bolt), added to test 31 and the invariants.
- **Fixed:** §16.4 said three petitions chain "and the fourth waits 60 s", and that the old gate allowed "ten" petitions of "60% of Max HP in 500 ticks, for 330 favor". Simulated with the 30-tick Winding and 20 recovery ticks: the fourth waits 3480 ticks (58 s); the old gate allowed eleven, each 4312 Bp (43%, the §16.4 table value), in 530 ticks for 363 favor.
- **Fixed:** Exports omitted `glyph_tome`, which doc 11 calls. §8.2 and the doc 02 Imports row now name `ComposeSpell { slot, refs }` (doc 02's `IntentKind`, sent by doc 17's composer) and `AssignWheelEntry`.
- **Fixed:** Interface rows. 03: the shadow-area light read is now present on disk (doc 03 §10.6 `dim(c)`), so it moved out of "Still requested"; `damage_block` added. 11: the `item.text.inscribed` row now names a scraped text (present). 08: signatures corrected to doc 08 §2.7 (`slot` parameter, `conc`). 14: "(§5.7)" made "doc 14 §5.7".
- Tables: every row has its header's column count. Exports: 195 IDs, every per-namespace count matches its list, and every owned ID used in the chapter is exported. Canon gaps holds only G17 (open). Header reads canon-1.11.

### Requests for other chapters after verification

Still open from the fixer's list: doc 05 (`HitRecord` spell Contradiction potency and Edge), doc 17 §6.5 (target list only for appraise and psychometry with `self`), Canon §17.1 (baseline-run library offer, G17). Already done on disk: doc 03 shadow-area read, doc 11 inscribed-text sources, doc 10 shrine lamp, doc 02 `SetCastItemTarget` and `Pray`. New: doc 03 §13.2 names doc 06 §13.1's `block_bp = 1000 × block power` as the spell block factor; doc 17 §10.7 writes "the fourth waits 3480 ticks (58 s)"; doc 15 §3.1.2 drops "inside their rarity bands" (glyphs have tiers, not rarity bands); doc 02 writes `ComposeSpell { slot, refs }`.
