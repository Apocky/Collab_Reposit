# Wave-2 fixer log, chapter 03

Started 2026-10-12. Baseline lint: clean (0 findings).

## Request triage (requests_03.json)

Already satisfied on disk (no edit): fix:05 #1 B_eff and two batches (§12.4, §2.2 rows 17/26); fix:16 MoverSnap comment and invader catch-up (§5, §5.19); fix:02 / verify:02 masks, mask_at, section-4 masks, max_steps; verify:05 lean (§5.8) and Source::Item comment; fix:07 torch (§13.3), erasers (§13.8), §4.5 JSON (no "id" keys), chipping (§13.2); fix:09 device_hacked, sight reading, no "G16" left; fix:18 Censor's blackout; verify:07 invader damage_block 0; fix:17 no "G22" left; fix:12 / verify:12 ev.law_discovered and masks; fix:06 shadow areas (§10.6); fix:01 trap identification (§14.2).

## Batch 1a (done)
- §12.1 item explosions: block power from doc 05 Tables 9.3/9.3b (frag 3000, concussion 1000, launcher 4000, others 0); dropped min(8000, mul_bp(4000, block_bp)); stored power = min(8000, mul_bp(base, sapper) + 750 demolition + 750 Fieldcraft-100 charge) (verify:05, verify:07).
- §12.4: impact and status powers scaled by falloff then exposure, one mul_bp each; f for knockback only; Interface row 05 reworded (fix:05).
- §13.2: doc 06 §13.1 spell block factor (verify:06).
- §6.10: respirator sentence extended; purify immunity skips inhaled rows (fix:08, verify:08).
- §2.2 row 21 and §7.3: doc 08 gas emitters run first each gas run (verify:08).
- §14.3 ev.alarm_raised kind comment: callouts raise none (fix:09).

## Batch 1b/1c (done)
- §5.19 homing defined here (Projectile.home_bam_s, home_acc; target choice, turn geometry, accumulator), replacing the undefined doc05::steer; Imports/Interface rows 05 and a new Imports row for doc 11 (verify:05, verify:03 unresolved).
- DECISIONS 128: crops removed from §2.2 row 17a, §13.1, cause table, §16.1 section 4, Interface row 07.
- DECISIONS 94: §4.6 gore mode is presentation only.
- §10.6 LightOverride.min_level (doc 13 strobe_overload interface row); acceptance 48 extended; Interface row 13.
- BlockQuery::mask added (doc 16 already reads custom masks through it).
- BodyState flag 8 CLIMB_ANY and the §5.11 rule (verify:03 unresolved, doc 04 §13).
- G29 adopted per the canon request: closed doors/trapdoors use unscaled registry light_filter, conceal, sound attenuation; §4.4, §11.2, §11.5 examples, acceptance 25 and 53 updated.

## Batch 2 (done): DECISIONS 95 movement kit (G31 part 1; fix:01, verify:01, verify:17)
- MoveMode + Slide (8), Grapple (9); MoveState.ledge_base_y, slide_t, slide_dir, last_slide_start, grapple; struct Grapple.
- §5.1 table rows, evaluation order and select_mode; ledge_base_y rule; gates end slide and grapple.
- §5.9 mantle: (550, 2000] above the feet and at most 2000 above ledge_base_y (jump adds no reach); examples 16..28 ticks.
- New §5.23 sprint-slide (CROUCH press while sprinting; 8000 mm/s start, 8000 mm/s² x friction decel, 45-tick cap; 3,799 mm on friction 10000) and §5.24 grappling tool (start_grapple; 20 m ray; 12000 mm/s pull; 120-tick cap; arrival 1000 mm; releases).
- §5.15 peak_y/footsteps, §5.21 portal shift, §2.2 row 7, §16.1 section 2, Exports, Interface rows 04/05/11/16/17/18, acceptance 11 and 52.

## Round 1, 2026-10-12

Resume pass. Every "done" claim of batches 1a, 1b/1c and 2 and of the harvest was re-checked on disk by grep (B_eff one mul_bp each in §12.4; two hit batches §2.2 rows 17/26; MoverSnap status list; mask_at, shape_mask, section-4 masks, max_steps; lean rule §5.8; Source::Item comment; homing §5.19; torch §13.3; erasers §13.8; no block JSON "id"; chipping §13.2; device_hacked u64; Censor's blackout; invader damage_block 0; no G22/G16; ev.law_discovered §14.3; respirator and purify §6.10; shadow areas §10.6; spell block factor §13.2; CLIMB_ANY §5.11; mantle 2000 mm §5.9; slide §5.23; grapple §5.24; climate §4.8 and hooks §6.10, §8.3, §10.2, §11.2). All present. Slide example recomputed (Python, Fx 32.32, DT raw 71582788, dv raw 572662304): friction 10000 -> 45 ticks, v raw 9162596992 (2133 mm/s), distance raw 16320875677 (3799 mm); friction 1500 -> 45 ticks, 5669 mm. Grapple: 19000 mm at 200 mm/tick -> 90 ticks to 1000 mm.

### Triage

| Item | Disposition | Where |
|---|---|---|
| requests 0-24 (fix/verify 05, 16, 02, 01, 07, 09, 18, 17, 12, 08, 06, verify 02/12) | already satisfied on disk | §12.1, §12.4, §2.2, §5, §5.8, §5.9, §5.19, §5.23, §13.2, §13.3, §13.8, §14.2, §14.3, §6.10, §7.3, §10.6, §4.3, §4.4, §16.1 |
| 25 G29 closed panels | chapter rule on disk (§4.4); G29 entry rewritten; canon_request | Canon gaps G29 |
| 26 G30 empty-mask shapes | chapter rule on disk (§4.4); canon_request | Canon gaps G30 |
| 27, 30 G31 movement kit and weather | body on disk; G31 entry rewritten with exact requested canon text; canon_request; tests added | Canon gaps G31, tests 58-62 |
| 28, 34 G6 ghost collision | canon_request with exact wording; entry restated | Canon gaps G6 |
| 29 doc 09 shadow term | already satisfied (doc 09 §2.1, Interface row 09) | Interface row 09 |
| 31 homing math | already satisfied | §5.19 |
| 32 CLIMB_ANY | already satisfied; doc 04 sets the flag | §5.11, Interface row 04 |
| 33 micro-voxel destruction vs whole-cell breaks | product_question default (whole-cell breaks kept); new canon gap G32 with exact wording | Canon gaps G32 |
| inbound 07 trap instigator_slot | edit | §14.2, Interface row 07, test 63 |
| inbound 17 §5.9 vs G31 | edit (one height, 2000 mm, in both; slide specified) | §5.9, G31 |
| inbound 09 slide/grapple noises | edit | §5 MoverOut, §5.23, §5.24, Interface row 09, test 62 |
| inbound 09 weather split | edit (doc 03 owns light, wet, fire, sound_ddb; doc 09 drops wx_sound, uses sky_open; precip column renamed wet) + cross_chapter_request 09 + merged canon_request | §4.8, §6.10, Interface row 09, G31 |
| inbound 13 creature crouch, mantle, knockback | edit | §3.4, §5.8, §5.9, §5.13, Interface row 13 |
| DECISIONS 99 | rules on disk; scope unified on sky_open; tests 59-61; §17 row | §4.8, §17 |
| DECISIONS 131, 134 last stand | edit: new §5.25 timing in ticks; product_question (world keeps running); cross_chapter_requests 04, 05, 16, 17, 02 | §5.25, comp.player, test 64 |
| DECISIONS 145 | no survival meter in this chapter; nothing to change | - |
| harvest: §17 budgets | edit (column tops, homing scan, slide/grapple) | §17 |
| harvest: consumers citing 1,600 mm | cross_chapter_requests 01, 05, 12, 17 | outbox |
| leftover sweep | hits are allowed text: "no item durability" (DECISIONS 65), doc 07 block repair refill (placeables keep block HP), "no crop map or planting" (128 retirement), retired mount kind code 2 (canon §17.7), 12.999 is a coordinate | - |

### Batch R1-1 (21 edits): header (As of 2026-10-12, through row 145); Imports (DECISIONS row; doc 07 §10.4; doc 09 §7.2, §7.3, §15.3; doc 13 §2.1, §4.13); §4.8 `precip` column renamed `wet` (0/1), sound weather owned here, `sky_open` the one weather scope readable by sys.perception/sys.sound/sys.hearing; §6.10 weather row; §3.4 creature crouch_h/eye_crouch from body_state; §5.8 NPC CROUCH; §5.9 creature mantle and room-to-rise cells; §5.13 single knockback entry point; MoverOut slide_started/reel and sys.movement noise emission; §5.23 slide noise; §5.24 grapple impact and reel noises; §14.2 and Interface row 07 trap instigator_slot. Lint --chapter 03: clean.
### Batch R1-2 (13 edits): comp.player last_stand_until/last_stand_used; new §5.25 last stand timing (DECISIONS 131, 134); Canon gaps G6 (wording), G29 (rewritten: adopted ahead of canon), G31 (rewritten: adopted; exact Canon §4.2, §13.1, §5.12, §19 text, merged with doc 09 G26 and doc 12 G19), new G32 (whole-cell breaks) and G33 (last stand); Interface rows 04, 05, 09, 13, 17; §17 movement, projectiles (homing scan), block updates (column tops) unit costs. Lint --chapter 03: clean.
### Batch R1-3 (2 edits): acceptance tests 58-66 (slide, grapple, climate light, climate surfaces/fire/sound, movement-kit noises, trap credit, last stand, creature body, attack knockback); invariants annex +9 lines. Exports unchanged (no new IDs; noise IDs are doc 09's).

### Final lint
`spec_lint.py --chapter 03`: "== 03 words=41867", no findings; "IDs defined in more than one chapter: 0".
Full run: 00 hedges {'optionally': 1} (baseline); 14 undefined_ids enemy.prologue_hunter and 16 undefined_ids aware.profile.invader, net.msg.scribe_text come from other chapters' concurrent passes, not from chapter 03 (chapter 03 defines and references none of them); IDs defined in more than one chapter: 0.
Status: complete.
