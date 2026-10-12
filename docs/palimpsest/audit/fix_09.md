# Fix log: chapter 09 (Stealth and Perception)

Chapter fixer for doc 09, moving it to canon-1.11. Sources: DECISIONS.md (rows 1 to 88 and the orchestrator rows; rows 61 to 64 are doc 09's), canon-1.11 (on disk), `audit/canon_gaps_triage.md` (rows G-020, G-021, G-023, G-051, G-069, G-092, G-111, G-124, G-172, G-184, G-194, G-196, G-203; §4 conflicts C-20, C-30, C-31; §4.1 stale G15, G16), `audit/notes_stealth.md` and the other reconciler notes, and the doc 07 fix log's request for doc 09.

Edit scripts live in `scratchpad/fix09/` with helper `ed09.py` (asserts each anchor occurs exactly once, then confirms each new text is on disk). Never import another chapter's helper.

## Plan (status updated after each batch)

- [x] Batch 1: header, intro, Imports, §1 (draw table, events produced)
- [x] Batch 2: §2 to §4 (light overrides, doc 12 cites, mount rules out of stance and scent)
- [x] Batch 3: §6 and §7 (Vigilance and alarm memory, `st.hidden`, `st.marked`, `curse.pact.hush`, mount noises retired)
- [x] Batch 4: §9 to §13 (takedown entry, hiding, disguise coverage, bluff stream, slip replaces pick break, hack minigame, events)
- [x] Batch 5: Exports, Interface assumptions, Canon gaps, Acceptance tests, Invariants
- [x] Lint

## Changes

### Batch 1 (done): header, intro, Imports, §1.4, §1.5, new §1.7

- Header: "Conforms to canon-1.11" (G-051). Intro: drops the canon-1.4 sentence and Canon gaps G1; states the reserved `seq` range `1000..=1006` (G-021), mounts cut (DECISIONS 55, C-30), no item durability (DECISIONS 65, C-31), recoverable alarms with Folio-long wariness (DECISIONS 61), pacifist routes (DECISIONS 62), the hacking minigame (DECISIONS 63) and disguise coverage (DECISIONS 64).
- Imports: Canon §2.3 light overrides (G-020); §3.3 reserved range and `rng.check` (G-021, G-023); §4.1–§4.4 fixed terrain, no durability, micro-voxels; §10.6 lethal-takedown row (G-092); §11.1 drops `st.mounted` (C-30) and states the `st.marked` wording (G-111), §11.6 hunter rank (G-112); §12.2 tall grass (G-124); §17.7 mounts cut; §18 drops `ev.mounted` (C-30), adds `ev.status_applied` and doc 03 §14.3 registration (G-184). Doc 03 row adds §5.14, §13.8, §14.3, `HeardNoise.req`, `grid.cause.erased`, `DmgFlags` bit 13 (stale G15/G16 now present on disk). Doc 04 row adds §6.1 pacification. Doc 07 row: no durability (C-31), `block.trap.spikes`, `conceal_crouch_only`, no snow carrier, §11.1 rule 2 seized devices; dropped §12 (cooking). Doc 10 row adds `curse.pact.hush` (doc 10 Interface row for 09). Doc 12 row: section cites corrected per doc 12 I7 (light bands are §11.3, not §10.2), adds `room.restricted` (§12). Doc 13 row: `outfit` flag, native factions, defeats and `ev.entity_pacified`.
- §1.4: `wary_until` comment names `WARY_FOLIO`; new `StealthWorld.alarmed` faction set (DECISIONS 61 alarm memory).
- §1.5: seq 1000 is the auto pick's success and **slip** roll (the pick-break roll is gone, C-31); 1001 marked auto; 1004 now serves the manual minigame for locks and devices (DECISIONS 63); 1005 unused (retired with the pick break). Paragraph: slip instead of break, one action per actor so seq 1004 never collides, the Canon §3.3 reservation, no `rng.check` draws (G-023, C-20).
- New §1.7 Events produced: the eight doc 03 §14.3 events doc 09 produces (`ev.alarm_raised`, `ev.body_discovered`, `ev.disguise_blown`, `ev.lock_opened`, `ev.object_revealed`, `ev.device_hacked`, `ev.takedown`, `ev.item_stolen`) with emission rules and payload values (G-184 closes old G5).

### Batch 2 (done): §2.1, §2.2, §2.4, §3.1, §3.2, §4.4, §4.5, §4.7

- §2.1: light overrides clamp `light_at` and so `env`; own light, flashes, phosphor and spotlights are not cell reads and stay unclamped (G-020, Canon §2.3).
- §2.2: light bands cite doc 12 §11.3 (doc 12 I7 says §10.2 was wrong). §2.4: fixtures destructible per Canon §4.3 (doc 12 §2.2 was the manufacturer table).
- §3.1: deleted the `st.mounted` stance row (15000) and "gallop" from the fast-motion row (C-30, DECISIONS 55).
- §3.2: micro-voxel rule for sight rays: boxes met by `raycast_blocks` block, missed partial cells add fill-scaled `cell_conceal` (new open item Canon gaps G23, because Canon §4.4's "blocks sight at ≥ 48 micro-voxels" would let closed doors pass sight).
- §4.4: `HeardNoise.req` cites doc 03 §11.5 (stale G15 closed, triage §4.1).
- §4.5: scent layers drop the mount-carrying-an-Erratum case and both "mounted" conditions (C-30).
- §4.7: `st.marked` visible to the marker itself and its allies, per amended Canon §11.1 (G-111, closes old G13).

### Batch 3 (done): §6.1, §6.2, §6.3, §6.6, §6.8, §7.1, §7.5, §7.6, §7.7, §7.9

- §6.1: deleted the interim-name mapping sentence (G-196: docs 13, 17, 18 use the six states; closes old G2).
- DECISIONS 61 (recoverable alarms with memory): §6.3 defines granting Vigilance as `wary_until = max(wary_until, now + WARY_TICKS)` and adds `WARY_FOLIO = Tick::MAX`; T8 and callout listeners now "gain Vigilance" (a grant never shortens Folio-long wariness); `raise_alarm` sets `wary_until = WARY_FOLIO` for every reached member, adds the faction to `alarmed`, emits `ev.alarm_raised`, and later spawns of an alarmed faction start with `WARY_FOLIO`. The alarm stays recoverable: searchers still return to unaware through T8 after 1800 ticks.
- §6.8: `st.hidden` no longer tests "not mounted" (C-30; Canon §11.1 row has no mount clause).
- §7.1: new modifier row 11 `curse.pact.hush` (−m, doc 10 §7.8; doc 10's Interface row for 09); later rows renumbered 12 to 14 (no references to row numbers exist).
- §7.5: `noise.lock.snap` now means "a pick slips" (C-31: the pick no longer breaks).
- §7.6: heading drops "mounts"; deleted `noise.mount.walk`, `.gallop`, `.hover`, `.glide`, `.summon` rows (C-30; no other chapter uses them); one sentence records the retirement without backticked IDs. §7.7: `noise.mount.*` removed from the attributed list, the ignore row and the investigate row.
- §7.9: `erased` cause cites doc 03 §13.8 (stale G16 closed, triage §4.1).

### Batch 4 (done): §7.2 snow, §9.2, §9.4, §10.1, §10.4, §11.1, §11.4, §13.1 to §13.7

- §7.2: `noise.surface.snow` has no carrier (doc 07 §2.4 decision, G-203; doc 07's request "close G21"); kept defined for data-pack materials and doc 18's 17 audio classes.
- §9.2: deleted "hits from a mounted rider" (C-30). §9.4: takedown eligibility drops "not mounted"; the lethal takedown goes through Canon §10.6's new lethal-takedown row (G-092, closes old G4); the elite half-HP variant is a new open item (Canon gaps G24, because the canon row says "HP plus ward"); `DmgFlags` bit 13 cites doc 03 §14.2 (stale G16); `ev.takedown` emitted; subdue pacification reported by doc 13 as `ev.entity_pacified` method 4 for pacifist tracking (DECISIONS 62).
- §10.1: deleted "A mounted Erratum cannot enter any hiding spot" (C-30). §10.4: `ev.body_discovered`.
- §11.1 (DECISIONS 64): outfits come from any corpse or subdued NPC whose doc 13 template has `outfit` (no longer "humanoid" only), weighing `outfit_mass_g`; every `fac.native.*` faction must have at least one outfit template; `fac.editors` and `fac.wild.*` have none. Disguise activity drops "the wearer is not mounted" (C-30).
- §11.4: bluff stays on `rng.stealth` seq 1006, now cited to Canon §3.3's `rng.check` row (G-023, C-20; closes old G9); compromise emits `ev.disguise_blown`.
- §13.1: key opening emits `ev.lock_opened` method 0; lockpicks and decks are never consumed, worn or broken (C-31, DECISIONS 65).
- §13.2 (C-31): the break roll (lockpicks durability −1) is now a **slip roll**: below `slip_bp` (2500, × 5000 at Security 25) the attempt lasts `SLIP_TICKS` = 30 more ticks and emits `noise.lock.snap` 35 dB at once; both draws are taken on the attempt's first tick; success emits `ev.lock_opened` method 1; example "break chance 1250" is now "slip chance 1250, 120 ticks after a slip".
- §13.3 (DECISIONS 63): one `lock_mode` setting covers lockpicks and every hack verb, Seize included; pin count `2 + tier` with tier = lock tier, device tier, or 1 to 4 by Seize rank; layout on seq 1004 with actor = picker or hacker; a miss draws nothing (old seq 1005 break roll retired), restarts the pin `SLIP_TICKS` later and repeats the attempt noise; the third miss fails like an auto failure; draws exactly `2 + tier`; time multipliers apply to auto only; Security 100 and keys skip it. New paragraph states the cost of auto-resolving. New example: manual Loop on the §16 camera, 5 pins, `W = 7`.
- §13.4: the hacker must carry the deck, never consumed (C-31); success emits `ev.device_hacked` and `ev.lock_opened` method 2. §13.5: manual Seize uses the minigame; success emits `ev.device_hacked`; Hacking-100 Seize is pacification method 3.
- §13.6: `ev.object_revealed`; disarm tools never consumed; Security 50 disarm yields one `block.trap.spikes` item (doc 07's Interface row for 09). §13.7: `ev.item_stolen`.

### Batch 5 (done): Exports, Interface assumptions, Canon gaps, Acceptance tests, Invariants

- Exports: deleted the five `noise.mount.*` IDs and "mounts" from the group name (C-30); constants add `SLIP_TICKS` 30, `slip_bp` 2500, `WARY_FOLIO`; functions line adds `lock_mode` for picks and hacks and "the eight doc 03 §14.3 events listed in §1.7" (event IDs left unbackticked there so Exports declares no `ev.` ID).
- Interface assumptions: 02 adds `alarmed` and `SetLockMode` in the replay log; 03 marks the old requests present on disk (stale G15/G16) and requests the `device` field of `ev.device_hacked` as `u64` (block devices) and `kind` 0 of `ev.alarm_raised` as a creature alarm; 04 bluff on `rng.stealth` never `rng.check` (C-20), Security 25 halves the slip chance in place of a break chance (C-31), pacification; 07 no durability (C-31), `block.trap.spikes`, seized devices (doc 07 rule 2 present), no snow; 11 tools carried, never consumed, plus `item.face.mask`; 12 `room.restricted` present, and the DECISIONS 99 weather state as a future input; 13 outfit on every native faction (DECISIONS 64), `ev.entity_pacified` methods 4 and 3; 14 pacifist tracking (DECISIONS 62); 17 `lock_mode` for hacks, slip chance on the auto panel, manual hack dial, and the `noise.<category>.<name>` pattern note moved here from old G22; 18 audio for exported noises only.
- Canon gaps: all 22 old gaps closed with the triage row that closes each (G-051, G-196, G-021, G-092, G-184, G-020, G-203, G-023/C-20, G-172/C-30, G-124, G-111, doc 07 rule 2, §4.1, G-069, G-194). Two open items: **G23** sight through partial cells (Canon §4.4's 48-micro-voxel rule would let sight through closed doors; doc 09 keeps doc 03's box test; requested §4.4 wording) and **G24** elite takedowns (Canon §10.6's row says "HP plus ward"; doc 09 deals half max HP plus ward to elites; requested Notes text).
- Acceptance tests: 23 adds `ev.takedown` and pacification; 29 slip instead of break, tools still carried; 30 manual draws exactly `2 + tier`, pin restart `t + 31`, manual hack; 39 hidden without the mount clause; 50 alarm memory (`Tick::MAX`, T8 keeps it, later spawns wary, one `ev.alarm_raised`); 55 snow has no carrier; new 58 no mounts and no wear, 59 events, 60 disguise coverage, 61 light override.
- Invariants: mount line replaced by "no mount or rider rule"; added no tool wear, slip cost, `lock_mode` for picks and hacks, manual draw count, alarm memory, Vigilance grant as `max`, outfit coverage, events once per occurrence.
- Follow-ups: `ev.alarm_raised.kind`-style field references were rewritten as prose ("the `device` field of `ev.device_hacked`") because the linter reads them as IDs; trimmed duplicate mount-retirement sentences (Exports, doc 18 row) so the retirement is recorded in the intro and §7.6; `ev.lock_opened` method wording and `ev.takedown` payload completed; T8 recovery wording names `M < 2500`; §13.4 auto draw line and the manual gate.

### Lint after edits

`python3 tools/spec_lint.py --chapter 09`: no missing sections, no undefined IDs, no foreign-namespace definitions, no retired IDs, no JSON issues, no hedges (words 27,893). `--json` multiply_defined: no entry names doc 09 or a `noise.`, `aware.`, `disguise.` or `lock.` ID.

## Requests for other chapters

- **00 (canon):** §4.4 sight wording for partial cells (doc 09 G23): "A cell blocks light when `popcount(mask) >= 48`. A sight ray (doc 09) is blocked by a cell's occupied micro-voxel boxes; a crossed cell whose boxes it misses scales its concealment by the fill fraction." §10.6 lethal-takedown row Notes (doc 09 G24): "against `rank.elite`, `mul_bp(max HP, 5000)` plus ward instead (doc 09 §9.4)". §17.7: add the `noise.mount.` family (five IDs `noise.mount.walk`, `.gallop`, `.hover`, `.glide`, `.summon`) to the retired list; `tools/spec_lint.py` RETIRED_PREFIX gains `noise.mount.`.
- **03:** §14.3 `ev.device_hacked`: `device: u64 /* EntityId, or Gidx for a block device (fuse box, laser emitter) */`. `ev.alarm_raised` `kind` comment: "0 creature (hound, civilian, bell or klaxon), 1 alarm device, 2 camera"; callouts raise no `ev.alarm_raised`. Implement the doc 09 G23 sight reading in `raycast_blocks` and `cell_conceal` (box test; fill-fraction-scaled concealment for missed partial cells).
- **04:** §5.3 Security 25: "slip chance × 5000 (doc 09 §13.2)" replaces "lockpick break chance × 5000" (C-31). Interface row for 09 (C-20): "disguise Deceive checks on `rng.stealth` seq 1006"; delete the requests about the passive-detection cadence and `cat.placeable` disarmed traps (doc 09 meets both). Pacify row: a subdue counts on an entity's first subdue even though the NPC can wake (doc 09 §9.4).
- **12:** define the per-Folio weather state of DECISIONS 99 with a fog term and a sound-attenuation term in Bp; doc 09 will add them to `fog_q` and `sound_bp_eff` as it adds `gpv.evt.weather_front`'s.
- **13:** set `outfit` (and `outfit_mass_g`) on at least one template of every `fac.native.*` faction (DECISIONS 64, doc 09 §11.1, test 60). Emit `ev.entity_pacified` method 4 on an entity's first subdue, read from doc 09's `ev.takedown` with `lethal` false. Interface row for 09: "hacking buildup" should read "hacking (doc 09 §13.4, §13.5; no buildup)".
- **14:** pacifist tracking (DECISIONS 62): non-lethal defeats from `ev.entity_pacified` (doc 09 subdues method 4, Hacking-100 seizes method 3); a lethal takedown that kills is a kill.
- **17:** §10.11 `access.lock_mode` also governs hacks, Seize included (doc 09 §13.3); auto lock panel shows "Slip x.x%" from `slip_bp` (2500, 1250 at Security 25) beside the success chance; the manual dial applies to hacks (`2 + tier` pins, layout seq 1004, misses as cracks, pin restart after 30 ticks, the third crack is a failed hack); Interface row for 09 replaces "no lockpick break roll (C-31)" with "the slip roll of doc 09 §13.2 (no break, no wear)".
- **03 (citation):** line ~870 (§5.15 gaits) cites "doc 09 G16" for the crawl gait; read "doc 09 §7.2" (old G16 is closed).
- **12 (citation):** I7 note: drop "(doc 09's Canon gaps G19 asked for this tag)" and the remark about doc 09's "doc 12 §10.2" citation; doc 09 now cites §11.3 for light bands and §12 for `room.restricted`.

## Verification

Adversarial verifier pass, 2026-10-11, against the working tree (docs 03, 04, 06, 07, 10, 12, 13, 16, 17 as their own fixers left them). Edits were made by `scratchpad/verify09/edit1.py`, which asserts each of its 31 anchors occurs exactly once and confirms every new text is on disk, plus one follow-up of the same kind (G25 wording). Worked examples were recomputed by `scratchpad/verify09/recompute.py`.

### Check 1: triage rows naming doc 09

| Row | Verdict | Chapter text that satisfies it |
|---|---|---|
| G-020 (G6) | applied, then fixed | §2.1 "Doc 03 §10.6 light overrides, such as an Editor's blackout, clamp `light_at` and therefore `env` (Canon §2.3); `own`, flashes, phosphor and drone spotlights are not cell reads and are never clamped". **Verifier fix:** doc 03 §10.6 now also lowers every light read by 8 inside doc 06 shadow areas (doc 03's Interface row 09: "doc 09 adds no second shadow term"), so §2.1's own `dark = 8` term subtracted the shadow twice; it is gone, the example, the invariant and the doc 03 and 06 Imports and Interface rows now read the shadow through `light_at` |
| G-021 (G3) | applied | §1.5 "Canon §3.3 reserves `1000..=1006` on `rng.stealth` for this table and keeps detection's object-index `seq` below 1000"; Imports "its reserved `seq` range `1000..=1006`" |
| G-023, C-20 (G9) | applied | §11.4 "one success roll on keyed `rng.stealth` with `actor` = the wearer and `seq` 1006 (Canon §3.3: a disguise challenge is an AI detection roll and never draws on `rng.check`)"; Imports "`rng.check` (owner 04), which disguise challenges never use"; invariant `bluff draw := rng.stealth seq 1006`. Doc 04's side of C-20 is on disk (its row 09 reads "on `rng.stealth` seq 1006", the two met requests are gone) |
| G-051 (G1) | applied | header "Conforms to canon-1.11"; canon-1.8 and canon-1.10 applied (checks 3 and 4) |
| G-069 (G17) | applied | §13.6 "`min(9000, mul_bp(row35, 15000))`; row 35 itself is unchanged", within 5000 mm |
| G-092 (G4) | applied | §9.4 "through Canon §10.6's lethal-takedown row: it enters §10.5 at step 7 as `dmg.true` equal to its current HP plus its remaining ward and skips crit, resistance, armor and the chip floor"; the elite variant is open item G24 (pre-existing rule, canon row says HP plus ward) |
| G-111 (G13) | applied | §4.7 "`S` itself and every observer allied with `S` ... treat `t` as a near-zone sight pass" |
| G-124 (G11) | applied | §3.2 "crouch-only cells ... count only when t is crouched, downed or a body"; Imports "tall grass only against crouched or downed targets and corpses" |
| G-172, C-30 (G10) | applied | intro "Mounts are cut (DECISIONS 55, Canon §17.7), so no rule here reads a rider"; no `st.mounted`, `ev.mounted` or `noise.mount.*` ID remains (lint: no retired IDs); test 58, invariant |
| G-184 (G5) | applied | §1.7 table of the eight events, each matching doc 03 §14.3's payload (raiser/pos/kind, observer/body/pos, wearer/observer, by/lock_key/method, observer/object_key/kind, by/device u64, attacker/target/lethal, thief/victim/uid). **Verifier fix:** the `ev.alarm_raised` row now excludes calls a disabled alarm panel silences (§13.4 "their `raise_alarm` calls do nothing else") |
| G-194 (G19, G22) | applied | lint: no undefined IDs; Canon gaps closure sentence |
| G-196 (G2) | applied, residue found | docs 13, 17, 18 use the six states; Canon §2.3 `in_combat` still says "awareness `alerted`" (the only remaining interim name in spec/), and deleting old G2 dropped its mapping. **Verifier fix:** new open item G25 reads it as `aware.alert` and requests the canon wording |
| G-203 (G7, G8, G12, G18, G20, G21) | applied | §7.2 snow row "none: doc 07 defines no snow material or block (doc 07 §2.4, triage G-203)"; §15.1 `SOUND_BP`; §9.2 tiers without riders; §7.9 ghost noises unattributed; Ur-Text row in §15.1 |
| C-31 | applied | §13.1 "Both tools are carried, never consumed, worn or broken by an attempt"; §13.2 slip roll (time + 35 dB noise) in place of the break; §1.5 seq 1005 unused; §13.4 "no hack consumes or wears it" |
| §4.1 G15, G16 | applied | Canon gaps "G15 and G16 (triage §4.1: present in doc 03)"; §4.4 cites `HeardNoise.req` (doc 03 §11.5), §7.9 `grid.cause.erased` (§13.8), §9.4 `DmgFlags` bit 13 (§14.2); doc 03 no longer cites "doc 09 G16" |
| G-112, G-183 (not naming 09 in Sources) | agree | Imports Canon §11.6 "the Erasure hunter keeps `rank.hunter`"; §9.5 hunters immune to takedowns and disguises; `grid.cause.erased` and the `flier` tag present |

### Check 2: DECISIONS rows

| Row | Verdict |
|---|---|
| 15 invasions, 32, 86 | agrees: §14.5 invader is `fac.editors` at rank hunter with the perception filter. **Verifier fix:** §14.5 now closes this chapter's actions to the invader (no takedown, pick, hack, disarm, steal or plant), so it never opens a container or takes an item (Canon §17.9; its doc 13 §10.7 kit binds only attacks). The filter itself conflicts with doc 16 O1 (unresolved, below) |
| 16 limited building | agrees: §2.4 fixtures destructible (Canon §4.3), Errata lights are `cat.placeable`, `noise.block.place` for placeables, `noise.node.harvest` for nodes; `block.fuse_box` destructible per doc 07 §10.1 |
| 17, 61 alarms | agrees: §6.3 "An alarm is therefore recoverable ... they stay wary for the rest of the Folio (DECISIONS 61)"; §6.6 `WARY_FOLIO`, `alarmed`, later spawns wary, `search_ticks` 1800; test 50 |
| 26, 91 softer Erasure | agrees: one hunter, `rank.hunter` identity (§4.2 profile editor, §9.5 caps); `st.erasure` +stacks dB (§7.1); no 5400 Erasure repeat (every 5400 in the chapter is `WARY_TICKS`) |
| 28 Echo cast | **verifier fix:** doc 04 §13 gives `body.serpentine` (`echo.jormungandr`) footsteps −6 dB and doc 04 §9.4 gives `aug.silent_pads` footsteps −6 dB, but §7.1's modifier table (the complete list `noise_level` sums) had neither; both are rows 7 and 8 now (later rows renumbered; no reference to a row number exists), with Imports and Interface 04 rows |
| 33, 38, 42, 46, 56, 69, 84, 90 | not in doc 09: grep finds no glyph-library, narrator, price, ending, World Law, roster, severing or "Crowns" text |
| 37 micro-voxels | agrees: §3.2 box test plus fill-rule concealment, open item G23. **Verifier fix:** §3.2 now says the fill rule gives 15 (ray blocked) at 48 or more micro-voxels, matching doc 03 §4.4's `conceal_at` |
| 55 mounts cut, 89 | agrees (check 1, C-30) |
| 60 pact curses | agrees: §7.1 row 13 `curse.pact.hush` "−m, the pact's boon magnitude" matches doc 10 §7.8 (boon 6 dB per unit, drawback HRG −m) |
| 62 pacifist play | agrees: §9.4 subdue as `ev.entity_pacified` method 4 on the first subdue, §13.5 Hacking-100 Seize method 3 (doc 03 §14.3 codes present); Interface 13 and 14 rows. Doc 14 has no pacifist tracking text yet (request below) |
| 63 hacking minigame | agrees: §13.3 one `lock_mode` for picks and hacks, Seize included; auto costs chance ≤ 9500, failure time, `noise.hack.fail` and alarm steps, and for picks the slip. **Verifier fixes:** `StealthAction` had no field recording a hack's mode (kind 1 is `pick_manual` only), so `manual: bool` is added; the pin restart at `now + 1 + SLIP_TICKS` left `c = (now − pin_start) mod 40` undefined for presses before `pin_start`, which now do nothing; §1.5 seq 1004 names the manual Seize |
| 64 disguises | agrees: §11.1 coverage of every `fac.native.*` faction, test 60. **Verifier fix:** `aware.profile.boss` Flags said "—" though §9.5 and §11.1 make bosses ignore disguises; now "ignores disguises" |
| 65 no durability | agrees after C-31; **verifier fix:** doc 07 §11.1 rule 4 rolls the hacking-deck failure as a hack starts and says "doc 09 says what the attempt does after a spark", which doc 09 never said; §13.4 now ends the attempt on that tick with no `rng.stealth` draw, no hack noise and no `hack_fails` or pity change, and a Disconnected deck cannot start a hack (Canon §5.5); test 31 extended; Imports and Interface 07 rows |
| 77–83, 85, 87, 88 | no doc 09 surface; 81 and 85 touch only doc 04 XP, which §9.4 and §12 cite unchanged |
| 94 ink gore | presentation only: §10.4 still reads `liq.blood` (doc 08 §3.3) |
| 99 weather | agrees as far as defined: weather fronts feed `fog_q`, `sound_bp_eff`, ambient and scent life; Interface 12 asks for the per-Folio weather state |

### Check 3: chapter brief

- Mounted rules (G10; `ev.mounted`, `st.mounted` ×3): gone; only retirement sentences remain (intro, §7.6, Imports Canon §17.7, test 58, invariant).
- Durability ×3: lockpick break roll → slip roll (§13.2: +`SLIP_TICKS` 30 and `noise.lock.snap` 35 dB, `slip_bp` 2500, × 5000 at Security 25); manual seq 1005 break roll retired; deck wear gone. Tools "carried, never consumed, worn or broken". **Verifier fix:** §13.6's Security-50 yield gave a `block.trap.spikes` item on every disarm, wards and electronic traps included; doc 07 §10.4 and doc 04 §5.3 restrict it to worldgen spike, blade and dart traps at `ilvl` = the Folio index; §13.6, test 33 and Interface 07 now say so.
- C-20: bluff on `rng.stealth` seq 1006 (check 1). `rng.check` cited as Canon §3.3's (owner 04).
- Stale G15, G16: closed in the Canon gaps sentence (check 1).
- DECISIONS 61 to 64: check 2.

### Check 4: leftover grep

`mount|ride|saddle|cavalry|durab|repair|wear|broken|breaks|Delving|voice actor|voiced|VO|24.99|Item Codex|Crowns|Redactor|margin.plot|building plot|5400`: every hit is a retirement or no-wear statement (intro, Imports, §1.5 row 1005, §7.6, §13.1, §13.2, §13.4, tests 58, invariants), a block or fixture breaking (§2.4 rows 5 and 6, §7.5, §7.9), "breaks the trail / line of sight / near cone", "ties broken", `disguise_blown`, the override noun "override", or Vigilance's `WARY_TICKS` 5400 (§6.3, §6.6, test 19, invariant). No real leftover.

### Check 5: lint

`python3 tools/spec_lint.py --chapter 09` after the edits: no missing sections, no undefined IDs, no foreign-namespace definitions, no retired IDs, no JSON issues, no hedges (words 28,385). `--json` `multiply_defined`: `{}`.

### Check 6: internal consistency

- Recomputed with Python, all match: V1 (`X` 2772, `V` 471), `R_near` 8200, `R_far` 24600, `f_dist` 4964, V3 976, lean 1608 / 4020 / 6700, §5.6 timelines (23/42/73, 9/16/27, 7/12/20; unaware gain 109), decay (41 checks at 925; 27 at 963), callout gain 140 and 22 checks, disguise 9020 / 153 / 306 / 51 with checks 33/66 and 17/33, lockpick (`your_tier` 5, rating 72, chance 8400, slip 1250), `W` 4 / 6 / 7, device hack 67 / 7400 / 76, corpse 2100 / 1239 / 309 / check 17, rings 14500 and 10416, gem 471 and 2467, §7.8 hearing 4250 / 1700 / 5525 / 34, rescale 19 / 33 / 21, heist step 2 (15 dB, thr 17, `od` 2). The §2.1 example and test 1 keep their numbers after the shadow fix (doc 03 gives `light_at` = `max(0, 2 − 8)` = 0).
- Tables: every row has its header's column count. §7.9's "any other cause" list now names all 22 doc 03 §13.1 causes (`connected`, `reaction`, `reverted` and `expired` were missing).
- Exports: every backticked `noise.`, `aware.`, `disguise.` and `lock.` ID used in the chapter is exported (the one prefix-only hit is the retired `noise.mount.` family); every ID other chapters use is exported; no new owned ID was added.
- Canon gaps: three open items (G23, G24, new G25); the closure sentence only records closures, as in docs 02, 03 and 04.
- Interface assumptions: **verifier fixes:** the doc 03 row still "requested" `device: u64`, the `kind` 0 reading and the G23 sight reading, all on disk now (doc 03 §14.3, §4.4, §5.18, §7.6), so the row marks them present; the doc 17 row still asked for the `noise.<category>.<name>` reading, which doc 17's row 09 now states ("`noise.<category>` or `noise.<category>.<name>`"), so that clause is gone and the met `lock_mode` and slip-panel items are marked present; the doc 16 row now records that doc 16 does not apply §14.5's filter (doc 16 O1).
- Header: "Conforms to canon-1.11".

### Verifier edits (chapter 09)

1. §2.1, Imports (doc 03, doc 06), Interface 03 and 06, invariant: removed the second shadow subtraction; `stealth_light = max(light_at, own, flash, phosphor, spot)`.
2. §3.2: the fill rule blocks at 48 or more micro-voxels.
3. §7.1: rows 7 `aug.silent_pads` and 8 `body.serpentine` (−6 dB steps); Imports and Interface 04.
4. §7.9: complete cause list.
5. §13.3: no press before `pin_start`.
6. §13.4: Disconnected deck; spark ends the attempt with no effect; Imports and Interface 07; test 31.
7. §13.6: Security-50 yield only from worldgen spike, blade and dart traps; test 33; Interface 07.
8. §1.4: `StealthAction.manual`.
9. §1.5: seq 1004 covers a manual Seize.
10. §1.7: `ev.alarm_raised` not emitted for a call a disabled panel silences.
11. Interface 03, 16, 17 rows brought to the on-disk state.
12. §14.5: the invader takes none of this chapter's actions.
13. Canon gaps: G25 (`in_combat` "alerted").
14. §4.1: boss profile "ignores disguises".

### Requests for other chapters (updated)

The fixer's requests above stand except: the doc 03 requests (`device: u64`, `kind` comment, G23 sight in `raycast_blocks` and `cell_conceal`) and the doc 03 G16 citation are already on disk; doc 04's C-20 row change is already on disk. Added by this pass:

- **00 (canon):** §2.3 `in_combat`: "awareness `alerted`" → "awareness `aware.alert` (doc 09)" (doc 09 G25).
- **04:** Interface row 09 still describes "a lockpick break roll against `break_bp` ... where a break spends one pick from the `item.tool.lockpicks` stack", and §5.3 Security 25 still reads "lockpick break chance (doc 09 `break_bp`) × 5000"; both become doc 09 §13.2's slip chance (`slip_bp`), with no pick spent. The Shade kit's "5 lockpicks" (doc 04 §2.5, also doc 14 §2.4's kit table and doc 11's `shop.tinker` × 5) can be 1, since one set is never consumed.
- **06:** §13 shadow row "`in_shadow_area(cell)` ... (doc 09 reads it)": doc 03 §10.6 reads it; doc 09 reads the result through `light_at`.
- **10:** §5.3 Witness reads "the Erratum's most recent sight check"; Errata run no sight checks on ordinary NPCs (doc 09 §4.8). Read doc 09's `erratum_sees(e, n)`, computed on the event tick; Interface row 09 likewise.
- **16:** O1: reconcile §6.6 rule 2 with doc 09 §14.5's invader filter (owner or orchestrator ruling needed; see unresolved).
- **18:** §6.3 material class: a feet cell holding `liq.mud` selects `mud`, any other liquid `water` (doc 09 §7.2).
- **07 (optional):** §11.1 rule 4 can cite "doc 09 §13.4 ends the attempt" instead of "doc 09 says what the attempt does after a spark".
