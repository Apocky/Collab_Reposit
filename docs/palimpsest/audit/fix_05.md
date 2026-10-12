# fix_05: chapter fixer log for spec/05_COMBAT_MELEE_RANGED.md

As of 2026-10-11. Target: canon-1.11 (already on disk), DECISIONS.md 1 to 96 and the orchestrator rows, audit/canon_gaps_triage.md.

Method: Python batches in the scratchpad; each edit asserts its anchor occurs exactly once, the file is written only when the whole batch passes, and each insertion is confirmed with grep afterwards.

Lint before: `undefined_ids (1): input.mount`; `foreign_namespace_definitions (2): sys.damage, sys.damage_late`; `retired_ids (8)`; `hedges: could 1`.

## Plan (batches)

1. Header, intro, Imports, §1 and §2: canon-1.11 header; C-30 mount removals in §1.3, §1.4, §1.5, §1.6, §1.7, §1.8, §2.1, §2.3, §2.6; G-021 seq reservation; G-035 speed chain; G-094 two hit batches in §1.3 and §2.7.
2. §4 to §9: C-30 (§4.8a, §6.3, §6.6, §6.8, §16); C-31 durability (§9.2); C-27/G-037/DECISIONS 88 Somnambule and `block.tag.destructible` in §7.3; attack-factor renumbering in §9.4; hedge in §6.2.
3. §10 to §16: G-084 attack factors without the mounted charge; G-085, G-086, G-087, G-088, G-089, G-090, G-091, G-092, G-093 in §10.2, §12, §15; G-065 manufacturer hooks as §10.5; G-059 Shock of the New ranks; G-161 counter sourcing.
4. §17 retired; §19.1 severing per DECISIONS 84.
5. Exports, Interface assumptions, Canon gaps (close resolved gaps), Acceptance tests, Invariants; reconciler notes (sys.damage in Exports, unarmed skins for Echo bodies).

## Done

### Batch 1 (header, intro, Imports, §1, §2): done, 29 edits, all confirmed by grep
- Header → "Conforms to canon-1.11" (task item 6; G-051).
- Intro: dropped "mount" from the archetypes this chapter owns; added the no-durability sentence (C-31, DECISIONS 65, canon §4.3).
- Imports: §3.3 `seq` reservation (G-021); `block.tag.destructible`, fixed terrain stops projectiles, no durability (G-036, G-037, C-31); final move speed chain (G-035); Errata crippled-limb severing (DECISIONS 84); canon-1.11 damage amendments (G-084 to G-094); removed `st.mounted` and `slot.mount` (C-30); ward capacity chain (G-129).
- §1.3: C1 drops mount actions (C-30); C6 is the first of the two hit batches and the #26 row is the second (G-094); "weapon and guard state machine".
- §1.4: mount summon and stall roll removed; `seq` 2 reserved and never drawn; bound now cites canon §3.3's `0x0000..=0x01FF` reservation (G-021).
- §1.5 `HitRecord`: dropped `MOUNTED_CHARGE`, added `TAKEDOWN`, `entry` (canon §10.6 entry step, G-091/G-092) and `spell_pot_bp` (canon step 2.1).
- §1.6: canon-1.11 speed chain with `body_move_bp` (G-035); mounted-rider sentence removed.
- §1.7 `input.mount` row and §1.8 `move.summon`, `move.mounted_charge`, `move.couched_lance`, `move.trample` rows removed (C-30; clears the undefined `input.mount`).
- §2.1, §2.3, §2.6: mount and rider mentions removed (C-30).
- §2.7: two hit batches (G-094).

### Batch 2 (§4 to §9): done, 17 edits, all confirmed by grep
- §4.8a: `st.downed` row drops mount input; `st.mounted` row removed; summon channel removed from the channel row (C-30).
- §6.1: `Stalled(k)` ends at `stall_total` (60 beam stall, 90 Umbra heat lock, §10.5 hook; G-065).
- §6.2: hedge word "could" reworded (lint).
- §6.3 `st.mounted` spread factor, §6.6 hover-driver ADS rule, §6.8 "mounted" lean rule removed (C-30); §6.8 no longer cites the deleted gap.
- §7.3: penetrable cells name `block.tag.destructible` (G-036) and "fixed terrain stops every projectile, beam and penetration" (G-037); new step 2 "Somnambule pass" (C-27, G-037, DECISIONS 88, Q4): after 10000 mm of flight, destructible and non-opaque cells are passed with `wallpen_left` and `keep_bp` unchanged, never fixed terrain; worked example added.
- §9.2: "Each throw costs 1 durability (doc 07)" removed (C-31, durability mention 1 of 2).
- §9.4: target point `tp` and the curve are doc 03 §12.4's (G-049, doc 03 governs); `B_eff` stays one `mul_bp` per attack factor (canon §10.4) and is now attack factor 7 (factors 3 to 6 never apply to blasts) after the mounted charge retired; knockback uses doc 03's `mul_bp(knockback, mul_bp(falloff, exposure))`.

### Batch 3 (§10 to §16): done, 34 edits, all confirmed by grep
- §10.1: attack factor 3 (mounted charge) dropped and factors renumbered 1 to 7 in canon §10.4's order (move, charge, sweet spot, range falloff, keep, cleave, area falloff and exposure); one retirement sentence kept (C-30, G-084); "× N" manufacturer traits multiply `B` after canon §10.4, before step 1 (G-084); impact uses factors 3 to 7, status power factors 1 to 7.
- §10.2 code: `entry != 0` → `resolve_entry_point` with no crit draw (G-091, G-092, G-096); step 2.1 `spell_pot_bp`; step 3a fae iron (G-086); step 6c Scribe's Mercy (G-089); step 7 per-pool `m[t]` (G-090).
- §10.2 bullets: step 2.12 `st.taunted` cites canon (G-085); new bullets for step 3a fae iron (G-086), step 5 shield AR on blocks (G-087), step 6b split (G-088), step 6c Mercy (G-089), step 7 ward profiles and `counter.ward_breaker` (G-090), entry points incl. consumables (G-091) and lethal takedowns (G-092); Shock of the New ranks (G-059).
- §10.4: E4 Shock of the New wording (G-059); E6 adds the 6c Mercy result 828 mHP; E7 uses `m[pierce]`; E8 measures to doc 03's target point (G-049).
- New §10.5 "Manufacturer trait hooks" (G-065; replaces gap 17): perihelion (× N), umbra (no magazine, 90-tick `Stalled` heat lock), rustmaw (magazine × 20000, reload × 13000), lastditch (`seq` 0), jade_lotus (throw action, 30-tick return), hephaesteum (immune to disarm and corrosion incl. sunder), somnambule (§7.3 step 2), bureau (`failure_bp = 0`).
- §12.1, §12.2: shield AR at step 5 cites canon (G-087). §12.3: 6b per-type split in canon §10.7's words (G-088). §12.4: blunt poise `10000 + floor_div(share_blunt, 2)` (G-093); `MOUNTED_CHARGE` poise line removed (C-30).
- §13: status power base after factors 1 to 7. §14.3: no mount input; bleed-out cites canon §11.1 and doc 16 §9.4 (G-110).
- §15.1: ward breaker in canon `m[t]` terms (G-090); disruptor unscaled per canon §17.3a (G-109); sunder vs `elite.armored` and Hephaesteum. §15.2: `elite.warded` pool takes the generator's place. §15.3: counter sourcing per canon §17.3a (G-161). §15.4: capacity chain cites canon §13.2 (G-129); profile as canon `m[t]`.
- §16.1: potency list and Disconnected table lose mount entries (C-30); Unmoored counter names doc 06's `note_contradiction_failure` fed by docs 06, 07, 08 (G-057). §16.2: `Summon` mode and mount rolls removed. §16.3: mount stall row removed.

### Batch 4 (§3.2 note, §17, §19.1): done, 4 edits, all confirmed by grep
- §17 Mounts: the whole section (three `arch.mount.*` archetypes, summoning, `st.mounted`, `ev.mounted`, `ev.dismounted`, `room.no_mount`, mounted charge, couched lance, trample, mount Contradiction) replaced by one retirement paragraph; section number kept (C-30, DECISIONS 55, canon §17.7).
- §19.1 severing: canon-1.11 §9.1 rule (positive part damage, pool 0 after the hit, pool already 0 included); Errata sever only from a crippled pool, `floor_div(pool_before × 10000, max) <= 2500` (doc 04's `wound.crippled` tier), a healthy part never severs however large the hit (DECISIONS 84, overrides triage Q5/G-080's 5000 Bp clause); applied to every severable Erratum part incl. Echo tails and wings (new Canon gap 1); Errata example with arm pool 10,000 added.
- §3.2: `arch.melee.unarmed` is the natural attack of non-humanoid Echo bodies (Fenrir bite/claw/pounce, Jörmungandr bite/constrict/spit) as skins only (notes_run.md follow-up for doc 05; doc 14 §6.3).

### Batch 5 and 6 (Exports, Interface assumptions, Canon gaps, Acceptance tests, Invariants): done, 30 edits, all confirmed by grep
- Exports: `arch.mount.` row removed; `move.` 37 → 33 (mounted_charge, couched_lance, trample, summon removed) (C-30); functions line no longer names `sys.damage`/`sys.damage_late` (doc 03's namespace; lint foreign definitions, notes_events_sim.md) and adds `resolve_entry_point`.
- Interface assumptions: 03 (`destructible_at` = `block.tag.destructible`, target point `tp`, two hit batches with `HitRecord.entry`; mount mover and gap-27 citation removed); 04 (Errata crippled-pool severing, DECISIONS 84); 06 (`spell_pot_bp`); 07 (durability removed, block HP and the destructible tag; C-31 durability mention 2 of 2); 08 (consumable entry point, G-091); 09 (takedown entry point, G-092); 11 (counter sourcing per G-161, §10.5 trait mechanics, Somnambule per C-27); 12 (`room.no_mount` removed, `st.unmoored` per G-057); 13 (cavalry mounts removed; C-30, DECISIONS 89); 17 (`mercy_on`, `mercy_bp`).
- Canon gaps: all 28 old gaps closed (G-084, G-085, G-087, G-088, G-099, G-090, G-129, G-109, G-171, G-035, G-021, G-093, G-172, G-103, G-104, G-110, G-065, G-098, G-037, G-057, G-096, G-095, G-101, G-059, G-102, G-194, G-218/C-08); two real open items kept: (1) Errata tails and wings under DECISIONS 84's "limb" wording, (2) shock chains, thorns and trait ricochets queued during the first batch.
- Acceptance tests: 8 (Mercy 828), 25 (target point `tp`), 35 (no mount input), 38 replaced (Somnambule), 42 (two batches), 44 (Errata crippled-pool severing), 56 replaced (consumable and takedown entry points), 60 (115 IDs, no `arch.mount.`), new 61 (6b split and 6c Mercy), 62 (fae iron and taunt), 63 (Rustmaw, Umbra, Hephaesteum hooks).
- Invariants: mounts removed; two-batch hit order; Errata sever rule and no durability; entry points, × N traits and Somnambule.

Lint after batch 6: `== 05 words=30716` with no undefined, foreign, retired, JSON or hedge findings; `multiply_defined` lists no ID for 05.

### Batch 7 to 9 (consistency pass): done, 7 edits, confirmed
- §1.4: entry-point hits draw no crit at their `16 + i` slot (G-096, G-091, G-092).
- §10.2: source magnitude comment names the "× N" traits; the entry-point bullet split by entry step (step 4: DoT, falls; step 3: environmental, thorns, shock chains, consumables with `D2 = B`; step 7: takedowns); Mercy applies to entry hits that enter at step 3 or 4.
- Invariants: crit-draw lines exclude entry-point hits.

## Result

Lint after: `python3 tools/spec_lint.py --chapter 05` → `== 05  words=30744` with no undefined IDs, no foreign-namespace definitions, no retired IDs, no JSON issues and no hedges; `--json` `multiply_defined` contains no entry for 05 (`sys.damage`, `sys.damage_late` cleared). Acceptance tests 1 to 63, numbered without gaps.

Checked and needing no change: doc 04 `on_part_damage` already applies DECISIONS 84 with `floor_div(pool × 10000, max) <= 2500` to every severable Erratum part, matching §19.1. Doc 11's Somnambule row already reads per C-27.

## Requests for other chapters (not edited here)

- **Doc 03** §12.4: an item explosion's damage must be `mul_bp(mul_bp(b_mhp, falloff), exposure)`, one `mul_bp` per attack factor (canon §10.4; doc 05 §9.4 factor 7); keep `mul_bp(B, f)` for `env_base_mhp` only. Interface row for 05: replace "with impact and status powers scaled by `f = mul_bp(falloff, exposure)` of §12.4 in place of doc 05 §9.4's binary line-of-sight test" with "with damage, impact and status powers scaled by falloff and then exposure, one `mul_bp` each (doc 05 §9.4, §10.1)". G19: "doc 05's distance point" → "the target point `tp`", and drop "in place of doc 05's binary line-of-sight test".
- **Doc 03** canon §10.8 batches: queue environmental-explosion hits (source `Env`) for `sys.damage_late`, item-explosion hits for `sys.damage`; rows 17 and 26 of §2.2 name the two batches.
- **Doc 08** per triage G-096: thrown splashes, area potions and bombs are `NO_CRIT` area hits (one crit value drawn per target and ignored; doc 05 §9.3, §9.4); update the damage text (~line 594), harmful area effects (~1040), bombs (~1230), the firebomb example (~1249), gap 14, test 50, the invariant and the doc 05 interface row (~1411).
- **Doc 01** §7 verb table (~line 566): delete the "Mount, dismount | `input.mount`" row (C-30).
- **Doc 11** manufacturer table: cite doc 05 §10.5 for the hooks of perihelion, umbra, rustmaw, lastditch, jade_lotus, hephaesteum, somnambule and bureau; jade_lotus throws with `input.throw` when no throwable is on the quickbar; rustmaw's reload × 13000 covers the shotgun's start and per-shell ticks and the 18-tick nock.
- **Canon (00)**: adopt doc 05 Canon gaps 1 (Errata tails and wings under item 84) and 2 (hits queued during the first batch go to the second).

## Unresolved

- DECISIONS item 95 (sprint-slide, 2,000 mm mantle, grappling tool) names doc 05, but neither canon §4 nor doc 03 defines the slide yet; doc 05 adds no slide weapon gating until doc 03 publishes the slide state.
- Bomb crits: doc 05 keeps `NO_CRIT` per triage G-096; doc 08 still lets the crit apply until it takes the request above.

## Verification

Adversarial verifier pass, 2026-10-11. I read the whole chapter against `audit/canon_gaps_triage.md`, `DECISIONS.md` rows 1 to 112, canon-1.11 (§4, §5.9, §9.1, §10, §13.8, §17.3a, §17.7, §17.9) and the doc 03, 04, 08, 09, 11, 13, 16 and 17 text each Interface row relies on. Edits were made by one Python batch that asserts each anchor occurs exactly once and writes only when the whole batch passes, then one follow-up edit to test 64. Verdicts: **OK** means applied as ruled (a quote shows where), **FIXED** means I changed the chapter, **OPEN** means it is outside this chapter and is listed under requests.

### 1. Triage rows that name chapter 05

| Row | Verdict | Chapter text that satisfies it |
|---|---|---|
| G-021 (`seq`) | OK | §1.4: "every doc 05 `seq` lies in `0x0000..=0x01FF`, the range Canon §3.3 reserves for this table on `rng.combat` and `rng.status`"; `seq` 2 "reserved; never drawn" |
| G-024 (explosion draws) | OK | §9.4 step 1: "doc 03 §12.2's ray method with the spec's block_power_milli (rng.physics, doc 03's draw count)" |
| G-025 (stream uses) | OK | §1.4 row 0 draws per-shot and per-swing chance components on `rng.combat` at `seq` 0 |
| G-035 (speed chain) | OK | §1.6: "base speed, then §6.4 row 11, then doc 04's `body_move_bp`, then statuses ... armor never multiplies speed" |
| G-036, G-037 (destructible tag, terrain) | OK | §7.3 step 1: "its block carries `block.tag.destructible` ... Fixed terrain stops every projectile, beam and penetration (Canon §4.3)" |
| G-048 (C1 before movement) | OK | §1.3: "C1 runs before `sys.movement`" |
| G-049 (doc 03 governs explosions) | OK | §9.4: "Doc 03 §12 governs explosions, their target points, exposure samples and falloff curve; doc 05 supplies `inner_mm` and `edge_bp`" |
| G-051 (header, C-30, C-31) | OK | header "Conforms to canon-1.11" |
| G-057 (Unmoored counter) | OK | §16.1: "calls the bearer's `note_contradiction_failure(t)`, doc 06's one per-character counter, which doc 06 fizzles, doc 07 sparks and doc 08 failed potions also feed" |
| G-059 (Shock of the New ranks) | OK | §10.2: "applies to each native target whose rank is not `rank.boss`, `rank.editor` or `rank.hunter`" |
| G-065 (trait hooks) | FIXED | The fixer's §10.5 said "Traits not listed act through doc 11 and the Canon §10.6 entry points alone", which is false for the other "× N" traits (`rainslick`, `thimbleweald`, which use §10.1's hook), "+N Bp" traits (step 1), share-moving traits and chance traits, and it left three traits that touch doc 05 chains with no slot: `deepanvil` (attack speed × 9000 had no place in §6.2's `atk_speed_bp`, poise × 15000 none in §12.4), `grimhallow` (an instant reload has no §6.1 path: a 1-tick `Reloading` gives `commit = mul_bp(1, 7500) = 0`, so the rounds would never move) and `neon_saint` (penetration is §7.3's, Canon §4.3). The intro now states the general hooks; new rows for the three; §6.2, §12.4 and §7.3 step 2 carry the slots; new test 64 |
| G-080 / Q5 (overridden by DECISIONS 84) | OK | see check 3 |
| G-084 (attack factors, "× N") | OK | §10.1 factors 1 to 7 in canon order and "a manufacturer trait stated as "× N" ... multiplies `B` once, before step 1"; §9.4 "attack factor 7" |
| G-085 (`st.taunted`) | OK | §10.2: "`st.taunted` 7500 when the target is not the taunter" |
| G-086 (fae iron) | OK | §10.2 step 3a bullet; test 62 |
| G-087 (shield AR) | OK | §10.2: "`AR_part` is the hit part's AR plus the blocking shield's AR"; §12.1, §12.2 |
| G-088 (6b split) | OK | §12.3: "`d[t] = floor_div(d[t] × new_total, total)` ... remainder ... to the largest-share type"; test 61 recomputed (4,000; 3,001 and 1,000) |
| G-089 (Mercy 6c) | OK | §10.2 code `apply_mercy` between 6b and step 7; E6 828 recomputed |
| G-090 (ward profiles) | OK | §10.2 step 7 bullet, §15.1 bit 0, §15.4 |
| G-091, G-092 (entry points) | OK | §10.2 "Entry points" bullet; `HitRecord.entry`; test 56 recomputed (45,000 true, ward 5,000 then HP 40,000) |
| G-093 (blunt poise) | OK | §12.4: "`10000 + floor_div(share_blunt, 2)`" |
| G-094 (two batches) | OK | §1.3 C6 and the #26 row; §2.7 |
| G-095 (sonic and frozen) | OK | §10.2 "Sonic hits ... end `st.frozen` without the step 3a × 15000" |
| G-096 (crit draws) | OK | §1.4 `16 + i` row; §10.2 `NO_CRIT` bullet; §9.3 bombs `NO_CRIT` |
| G-098 (sneak × power) | OK | §4.9: "the Oblivion `max(sneak, power)` rule is not used" |
| G-099 (guard-break extensions) | OK | §4.6 |
| G-101 (elevation range) | OK | §7.1: "`max(0, mm(pr.spawn_y - pr.p.y))`" |
| G-102 (material speed floor) | OK | §3.3 "Material speed floor" |
| G-103 (throw speed) | OK | §9.1 `isqrt(row20_bp × 10000)` |
| G-104 (status powers) | OK | §13 `POWER10` table |
| G-109 (disruptor) | OK | §15.1: "for exactly 180 ticks each, unscaled by Canon §11.4" |
| G-110 (downed) | OK | §14.3, matching doc 16 §9.4's `max(600, 1800 − 600 × (downs_folio − 1))` |
| G-129 (ward capacity) | OK | §15.4 capacity chain with no material term |
| G-139, G-206, §4.1 (staff, wand, tome) | OK | §5.4 and Exports list `arch.ranged.staff`, `arch.ranged.wand`, `arch.offhand.tome` |
| G-170, G-171 (aspects, sunder, anchor) | OK | §15.1, §15.2 |
| G-172 (mount rulings rejected) | OK | §17 retirement paragraph |
| G-194 (IDs in other namespaces) | OK | gap 27 closed; no undefined IDs (lint) |
| G-218 (spawn step, crawl) | OK | §2.6 normal step at `t`; §1.6 crawl 800 |
| C-02 | OK | §5.6 rows for hand crossbow, sling, longarm `ammo.rifle`, flamer ≤ 300 `ammo.cell` |
| C-08 | OK | §2.6; doc 16 §8 item 2 now matches, so the doc 16 Interface row was stale: FIXED (now an assumption, not a request) |
| C-27 | OK | §7.3 step 2 and the §10.5 Somnambule row |
| C-30 | OK | lint finds no retired ID; §10.1 keeps one retirement sentence, §17 one paragraph |
| C-31 | OK | intro sentence and Invariants "N! item wear \| breakage \| durability"; both durability mentions gone |

### 2. DECISIONS rows

| Row | Verdict | Note |
|---|---|---|
| 3 (camera) | FIXED | the chapter never said the camera is presentation only; §6.8 now says both camera modes aim from the leaned eye (doc 17 Q10 default) |
| 15 (invasions) | OK | §2.6 slot 4 rewound; §7.3 invader hits never damage blocks; doc 13's kits carry no doc 05 explosive, so §9.4 needs no invader rule |
| 16 (limited building) | OK | intro: "only destructible blocks and placeables have block HP" |
| 26 (softer Erasure) | OK | no 5400; `rank.hunter` excluded in §10.2, §12.5, §19.3 |
| 28 (Echo cast) | OK | §3.2 Fenrir and Jörmungandr natural attacks as skins |
| 29, 30 | OK | kept |
| 33, 38, 42, 46, 56, 60, 69 | OK | nothing in doc 05 touches glyphs, narration, price, ending, Law previews, pact curses or enemy counts (grep) |
| 37 (micro-voxels) | OK | §2.2 voxel traversal with partial boxes; §7.3 reads doc 03's `opaque` |
| 55 (mounts) | OK | §17 |
| 65 (durability) | OK | see C-31 |
| 77 to 83, 85 to 87 | OK | no doc 05 text conflicts (Mercy appears only as step 6c; friendly hits grant no skill XP, which DECISIONS 85 does not change) |
| 84 | OK | see check 3 |
| 88 | OK | §7.3 step 2, test 38 |
| 95 (movement kit) | FIXED | names doc 05 but canon-1.11 and doc 03 (G31) define no slide or grappling tool; recorded as open Canon gap 3 |

### 3. Chapter brief

- **DECISIONS 84.** OK. §19.1: "An Erratum's severable part is severed only if its pool before the hit was at most 2500 Bp of its max ... `floor_div(pool_before × 10000, max) <= 2500` ... A healthy part never severs, however large the hit". This matches doc 04's `on_part_damage` (`crippled_before`). Every other sever mention agrees: §1.3 C6, Table 3.2b flags, §9.4 launcher `SEVER`, the §19.1 example (10,000 pool, 12,000 hit not severed; 2,500 severs; 2,501 does not), test 44 and the invariant. I fixed only the citation "(Canon gaps 1)" → "(Canon gap 1)".
- **C-30.** OK (lint shows no retired IDs; attack factor 3 gone; one retirement sentence).
- **C-27/Q4/G-037.** OK; recomputed: crate at 12,000 mm passes with 40 wallpen and keep 10000; at 9,000 mm keeps 5000.
- **Durability ×2.** OK.
- **sys.damage, sys.damage_late.** OK: named only in the "Doc 03 system" column and prose; the Exports functions line no longer lists them.
- **Damage amendments.** OK, every item (see check 1).

### 4. Leftover grep

Every hit was judged in context: `mount` (Imports §17.7 citation, §10.1 retirement sentence, §17 retirement paragraph, `amount_mhp`), `durability`, `wear`, `breaks` and `broken` (no-durability statements, the invariant, guard broken, ward breaks, blocks), `ride` (override), `lance` (balance), `5400` (inside `54000`, the launcher blast). No real leftover. No hits for saddle, cavalry, repair, Delving, voiced, VO, 24.99, Item Codex, Crowns, Redactor, margin.plot or building plot.

### 5. Lint

`python3 tools/spec_lint.py --chapter 05` → `== 05  words=31343`, no findings; `--json`: no undefined, foreign, retired, JSON or hedge findings, and `multiply_defined` has no entry for 05.

### 6. Internal consistency

- Python recomputation reproduces E1 to E9 (26,761 / 16,871; 47,817 / 26,643; 84,899; 4,453 / 2,288 / 3,048; 3,490 / 1,855 / 42,939 / 40,222; 1,034 / 4,405 / 7,563 / 828; 51,183 / 25,878 / 137,061 / 45 ticks; 30,717 / 16,866 / 6,995; 5,964), and tests 17, 18, 24, 25, 28 to 34, 44, 61, 62 and 63 (Rustmaw 24 rounds, `mul_bp(72, 13000)` = 93). Test 64's 12 ticks and 72,675 mp were checked the same way, conditioned on post-§3.3 values because Deepanvil's W+1500 tendency changes mass.
- Tables: every table row has its header's cell count. Acceptance tests run 1 to 64 without gaps; Canon gaps 1 to 3.
- Exports: every `arch.melee.`, `arch.ranged.`, `arch.armor.`, `arch.offhand.`, `arch.ward.`, `ammo.`, `move.` and `counter.` ID used in the chapter is exported; counts 15 + 20 + 20 + 6 + 4 + 12 + 33 + 5 = 115 (test 60); the 33 `move.` IDs equal §1.8's index.
- Interface assumptions: FIXED the doc 16 row (its request is met on disk) and the doc 11 row (doc 11 deleted the kit-rebinding table that the parenthetical overrode). Extended the doc 03 row with what doc 05 still needs from §12 (block power from Tables 9.3 and 9.3b, `b_mhp` before Canon §10.4, falloff then exposure one `mul_bp` each) and §5.19 (ghostline homing). The §5.8 sphere-clamp entry stays: doc 03 §5.8 still uses a point test.
- Canon gaps: 1 and 2 are still open in canon-1.11 (§9.1 says "limb"; §10.8 names no queued hits). Gap 3 added.
- Header: "Conforms to canon-1.11".

### Status of the fixer's requests and unresolved items

- Resolved on disk since the fixer ran: doc 01's `input.mount` row is gone; doc 03 rows 17 and 26 name the two batches; doc 08 makes bombs and splashes `NO_CRIT` area hits (its text near lines 594 to 603 and the firebomb example), so the fixer's "Bomb crits" item is closed.
- Still open: doc 03 §12 (see requests below, extended), doc 11 citations of §10.5, canon adoption of gaps 1 and 2.

### Verifier edits (spec/05)

1. §6.2: Deepanvil attack-speed slot in `atk_speed_bp`.
2. §12.4: `mfr_poise_bp` line (Deepanvil 15000).
3. §6.8: camera mode is presentation; both modes aim from the leaned eye (DECISIONS 3).
4. §7.3 step 2: Neon Saint charged shots pass their first penetrable cell for free.
5. §10.5: intro rewritten with the general hooks; rows for `deepanvil`, `grimhallow`, `neon_saint`.
6. §19.1: "Canon gaps 1" → "Canon gap 1".
7. Interface 03: explosion block power, `b_mhp` before Canon §10.4, falloff then exposure; ghostline homing in §5.19.
8. Interface 11: stale kit-rebinding parenthetical removed.
9. Interface 16: stale request replaced by the assumption.
10. Canon gaps: "Three open items"; gap 3 (DECISIONS 95).
11. Acceptance test 64 (general and added trait hooks).

### Requests for other chapters (verifier)

- **Doc 03** §5.8: replace the lean point test ("If the eye point would lie inside a box, `lean` steps back toward 0 that tick") and the offset `350 mm × lean / 9` floored with doc 05 §6.8's rule: `offset_mm = sign(lean_k) × floor_div(350 × |lean_k|, 9)` (so −1 gives −38 mm, not −39), then the 100 mm sphere clamp lowering `|offset_mm|` by 50 until clear, with `lean_k` moving on unchanged (Canon §4.2 gives the lean to doc 05).
- **Doc 03** §12.1 and §12.4 (extends the fixer's request): `Source::Item.b_mhp` is doc 05's `base_mhp` × attack factors 1 and 2, before Canon §10.4 (doc 05 §9.4), not "Canon §10.4 magnitude"; the item hit carries `HitRecord.base_mhp = mul_bp(mul_bp(b_mhp, falloff), exposure)` and Canon §10.4 is applied when it resolves (replace "enters at step 1 with `B = B_eff`"); item block power comes from doc 05 Tables 9.3 and 9.3b `block_power_milli` (frag 3000, concussion 1000, launcher blast 4000; incendiary, flash, smoke and counter 0), so drop the `min(8000, mul_bp(4000, block_bp))` derivation and its examples (frag 4000, concussion 2000, incendiary 1000).
- **Doc 03** §5.19: add the homing step `mfr.cyberpunk.ghostline` needs (doc 11's 16384 Bam/s through the Canon §2.3 accumulator), applied to the velocity before the move.
- **Doc 11** §5.2: cite doc 05 §10.5 for `deepanvil`, `grimhallow` and `neon_saint` as well as the fixer's eight; confirm "1000 mm of blocks" as the first penetrable cell, and that a refused Grimhallow cost starts no reload.
- **Canon (00)**: when DECISIONS 95 lands in §4, adopt doc 05 Canon gap 3 with doc 03.

### Unresolved (verifier)

- Neon Saint's "1000 mm of blocks" is hooked as one cell (`BLOCK_SIZE`); a diagonal path through one cell is longer than 1000 mm. Doc 11 owns the trait and should confirm or ask for a path-length rule.
- DECISIONS 95 stays open until canon §4 and doc 03 define the slide and the grappling tool (Canon gap 3).
