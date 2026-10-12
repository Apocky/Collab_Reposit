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
