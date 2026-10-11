# canon-1.11 editor notes

As of 2026-10-11. Input: `audit/canon_gaps_triage.md` (79 canon-amendment rows, §5.1 changelog, §5.2 tests), `DECISIONS.md` (product-owner items 1 to 88 and the orchestrator rows), `spec/00_CANON.md` at canon-1.10. Output: `spec/00_CANON.md` at canon-1.11.

Method: one Python batch per canon section range. Every edit asserted its anchor occurs exactly once before replacing, the file was written only when the whole batch passed, and each inserted text was then confirmed on disk. A final pass checked all 79 rows: every new phrase occurs exactly once and every replaced phrase is gone.

Every anchor in the triage matched the canon-1.10 text verbatim, so no row needed an adjusted anchor. Four rows were applied with adjusted text, each for the reason given on its line: G-027, G-080, G-087 and G-091.

## Rows

- G-001 → applied (§3.1 `range_incl` reference line computed in `i128`; test 45 added to §21).
- G-007 → applied (§1.9 `atan2_bam` argument convention and `yaw_from_dir`; test 46 added to §21).
- G-008 → applied (§1.1 new bullet after the `id32_collision` bullet: mod IDs use `x_<mod_id>` as the second segment).
- G-010 → applied (§1.6 JSON integer bound `-(2^53 - 1)..=2^53 - 1`).
- G-011 → applied (§1.5 one-decimal display of per-second rates below 10 display units).
- G-015 → applied (§2.2 named boundaries appended after the phase-key sentence; §2.5 hash on boundaries `b % 60 == 0`, `b >= 60`).
- G-016 → applied (§2.4 new bullet after the replay command-log bullet: intents in the replay log).
- G-017 → applied (§2.4 `content_hash` excludes presentation namespaces; `presentation_hash` added).
- G-018 → applied (§2.4 header bullet: `run_seed` in saves and replays, lobbies get it from the run header).
- G-019 → applied (§2.5 single-player save deferred while slot 4 holds an invader).
- G-020 → applied (§2.3 light overrides clamp every light read; composure, stealth and spawn sources named).
- G-021 → applied (§3.3 `seq` range, reserved allocations; trap-detection `seq` stays below 1000).
- G-022 → applied (§3.3 new "Object-keyed draws" bullet after the loot-drop bullet).
- G-023 → applied (§3.3 keyed table: `rng.check` row placed last, after `rng.progress`, which has the same owner).
- G-024 → applied (§3.3 explosion exception paragraph after the paragraph ending "cannot shift it."; §3.2 `rng.physics` row adds explosion block drops).
- G-025 → applied (§3.3 "Used for" paragraph placed directly after the keyed table, before the trap-detection paragraph).
- G-026 → applied (§3.3 `rng.loot` row: "Fortune Engine dispensing").
- G-027 → applied with adjusted text (§3.2 "Sub-streams" paragraph after the stream-seed code block). Test 47 salts with `fnv1a32("genre.space_opera")`, the `id32` that doc 12 §7.2's `gen_seed` uses, which is not the `fnv1a64(label) ^ index` form the row's text allows. The clause now reads "or another salt or a longer chain of `derive` calls that the owner documents (doc 11 `place_rng`; doc 12 §7.2, whose `gen_seed` uses the genre's `id32` as the salt)", so the rule and test 47 agree.
- G-028 → applied (§3.2 `rng.worldgen` and `rng.loot_place` "Draws for" cells).
- G-029 → applied (§3.2 `rng.run` row placed last, after `rng.physics`).
- G-030 → applied (§3.5 offer generator sentence appended).
- G-031 → applied (§3.6 account draws and Keepsake seeding replace "Paid gacha").
- G-032 → applied (new §3.8 "Presentation generators" after §3.7; text matches doc 18 §1.2's `nar_rng` and `pres_hash`).
- G-033 → applied (§4.4 liquid units of 1 litre, `Ml = units × 1000`, gas concentration `0..=10000`).
- G-034 → applied (§4.2 fall damage × `fall_dmg_bp`, then doc 04 factors in ascending source ID order).
- G-035 → applied (§4.2 final move speed adds `body_move_bp`; stance bases; armor never multiplies speed).
- G-036 → applied (§4.3 "Terrain is fixed" bullet replaced; tag is `block.tag.destructible`). Follow-on: §4.4's "Damage to a `destructible` cell" now names `block.tag.destructible` (§4.3).
- G-037 → applied (§4.3 "Projectiles and terrain" bullet placed after the terrain bullet; §5.9 Somnambule limited to destructible and non-opaque blocks; matches DECISIONS item 88).
- G-052 → applied (§5.2 row 1 Lucre skin "Nobles").
- G-053 → applied (§5.2 naming rule exempts a genre's own base currency). Follow-on: the §22 annex line now reads "cur./ent. names except its own base currency".
- G-055 → applied (§5.3 both rows replaced; note on holy, unholy and curse alchemy and licensed grafts added after the table).
- G-056 → applied (§5.5 failed potion defined; §5.11 counts failed potions).
- G-059 → applied (§5.6 Shock of the New excludes `rank.boss`, `rank.editor`, `rank.hunter`).
- G-060 → applied (§5.8 seam-zone `local_genre(pos)` is the Folio's genre).
- G-067 → applied (§6.4 row 29 GLYPH term per glyph of the producing stage).
- G-070 → applied (§7.1 `norm_mhp` before party scaling; matches DECISIONS item 85).
- G-071 → applied (§8 45-unit anchor and doc 13 supply sentence). Re-computed: 45 units per Folio gives level 12 after Folio 10, 29 on arrival at Folio 25 and 49 after Folio 40.
- G-072 → applied (§8 co-op split includes downed members and ghosts, excludes Dormant and Departed slots; matches DECISIONS item 81).
- G-073 → applied (§8 kills and pacifications; `ev.entity_pacified`).
- G-080 → applied with adjusted text (§9.1 severing rule). DECISIONS item 84 overrides triage Q5: the clause "or the hit's part damage is at least 5000 Bp of the pool's max" is dropped. The rule now reads "An Erratum's limb severs only if its pool was at most 2500 Bp of its max before the hit (doc 04 `wound.crippled` or worse); a healthy limb never severs, however large the hit (DECISIONS.md item 84)." The rest of the row (positive part damage, a pool of 0 after the hit, a pool already at 0) is applied as written.
- G-081 → applied (§9 added-part donors include `bp.body_front` and `bp.thorax`).
- G-084 → applied (§10.4 attack factors before the chain and "× N" manufacturer traits after it, inserted after "Each arrow is one `mul_bp`, in that order."). The row's trailing remark about doc 05's mounted-charge factor is a triage note, not canon text, so it was not inserted; the factor list given already omits it.
- G-085 → applied (§10.5 step 2.12 `st.taunted` 7500, in ascending status ID order).
- G-086 → applied (§10.5 step 3a fae iron × 12500).
- G-087 → applied with adjusted text (§10.5 step 5 shield AR on blocked hits). The new sentence sits after the `ignore_armor` sentence and before the code block. The colon that introduced the code block now ends the new sentence, and the earlier sentence ends with a period.
- G-088 → applied (§10.7 split-back paragraph after the block code block).
- G-089 → applied (§10.5 sub-step 6c Scribe's Mercy after 6b).
- G-090 → applied (§10.5 step 7 replaced with per-pool `m[t]`, generator profile and `counter.ward_breaker`; test 41 unchanged).
- G-091 → applied with adjusted text (§10.6 row for a consumable's damage to its drinker or injected target). The Notes cell is left empty, as in the other §10.6 rows without notes, instead of holding the word "none".
- G-092 → applied (§10.6 lethal takedown row, flag `takedown`).
- G-093 → applied (§10.1 `dmg.blunt` poise × `10000 + floor_div(share_blunt, 2)`).
- G-094 → applied (§10.8 two hit batches).
- G-108 → applied (§13.8.1 casting implement includes texts, enchanted items and holy symbols).
- G-109 → applied (§17.3a `counter.disruptor` exactly 180 ticks, unscaled by §11.4).
- G-110 → applied (§11.1 `st.downed` base duration defers later downs to doc 16 §9.4).
- G-111 → applied (§11.1 `st.marked` visible to the marker and its allies).
- G-112 → applied (§11.6 Erasure hunter keeps `rank.hunter` identity and uses `rank.elite` multipliers; this also implements orchestrator decision (b)). Follow-on: the §17.3 `rank.hunter` Notes cell points to §11.6 and to invaders (§17.9).
- G-113 → applied (§11.6 invaders never erase placeables).
- G-114 → applied (§11.8 neutral substitute command). The text directly follows "Input is never synthesized for a player." A paragraph break after it keeps the table's lead-in sentence next to the table.
- G-119 → applied (§12.1 gas contact at concentration 500 and the respirator rule; §12 `res.breath` names a working respirator and doc 08 filters).
- G-124 → applied (§12.2 tall grass conceals crouched or downed targets and corpses).
- G-125 → applied (§12.3 initial population included). Follow-on: the §5.9 definition of `spawn_block_max` now names the initial population too.
- G-128 → applied (§13.1 `cat.gadget` and `cat.placeable` rows). Follow-on: the §4.3 placeable list now says "block traps (spikes, webs)".
- G-129 → applied (§13.2 `slot.ward` capacity chain).
- G-130 → applied (§13.2 `slot.hand_extra_1` and `slot.hand_extra_2`; §13.3 Exotic limit covers them).
- G-131 → applied (§13.7 `ENH_CAP` and `eff_enh` upper bound; §13.9 `enhancement` range). The `eff_enh` anchor was widened to "curse_penalty, -3, 15)`" to make it unique; the replacement text is unchanged.
- G-132 → applied (§13.8 uses table: `P(d)` terrain break thresholds, `T(d)` Errata-placed block HP).
- G-133 → applied (§13.8 `H(d)` row adds `diff_hp_bp`).
- G-134 → applied (§13.9 `illuminated` roll keyed at drop or first acquisition).
- G-148 → applied (§14 no trading between accounts; Lucre per Erratum; dropped piles only; matches DECISIONS item 83).
- G-150 → applied (§14 `cur.ink` one-time welcome grant).
- G-159 → applied (§16.2 kit ownership split between docs 04 and 14).
- G-161 → applied (§17.1 Colophon shop absent in `genre.unwritten`; §17.3a counters in every loot pool and shop, plus doc 11's counter caches).
- G-162 → applied (§17.1 `FOLIO_1_GENRE` cell replaced).
- G-163 → applied (§17.1 `BOSS_FOLIOS` adds Marginalia Folios with `f % 5 == 0`).
- G-167 → applied (§17.1 Abridged loot quantity defined).
- G-170 → applied (§17.3 `rank.champion` notes: champion aspect and its counter type).
- G-173 → applied (§17.9 Redaction trigger ignores downed Errata).
- G-184 → applied (§18 core list, with further `ev.` IDs registered by doc 03 §14.3).

Count: 79 rows, all applied (75 as written, 4 with adjusted text), 0 skipped.

## Other edits

- **Header**: "Version: canon-1.11 (changelog in §23)."
- **§23**: canon-1.11 entry from triage §5.1 at the top. It is extended with the two orchestrator decisions, the follow-on wording, the DECISIONS item 84 override and "Added tests 45, 46 and 47." `x_<mod_id>` is in backticks so Markdown does not swallow `<mod_id>` as an HTML tag.
- **§21**: tests 45, 46 and 47 follow test 44, in order. Tests now run 1 to 47.
- **Orchestrator decision (a)**: §17.7 adds `npc.vendor.stablemaster`, `shop.stablemaster`, `ai.nav.mounted`, `ai.action.remount`, `ai.set.cavalry` and `enemy.role.cavalry` to the retired IDs. It also states that former cavalry fight on foot as `enemy.role.charger` (doc 13, namespace `enemy.`), which replaces `enemy.role.cavalry`.
- **Orchestrator decision (b)**: implemented by G-112 (§11.6) and the §17.3 `rank.hunter` note. The "no repeat spawns" sentence was already in §11.6.
- **DECISIONS items 81 to 88**: items 81, 83, 85, 87 and 88 match the applied triage defaults (G-072, G-148, G-070, G-017, G-037). Item 86 matches §17.9, which is unchanged. Item 82 is chapter-owned (C-04). Item 84 is applied as the G-080 override above.

## Verification

- All 79 rows: every new phrase occurs exactly once and every replaced phrase is gone. The only old phrase still present is "Crowns", inside the changelog line that records the rename.
- `python3 tools/spec_lint.py --chapter 00`: no JSON issues, no undefined IDs. Hedges are unchanged from canon-1.10 ("might" 2, "optionally" 1); no hedge word was added.
- Added lines contain no decimal literals. The only matches are text that existed before ("0.5 m", "×1.00", "1.5 skill XP") and the heading number "3.8".
- Table check: every table row in the canon has its header's column count (0 malformed rows, before and after).
- Test vectors: an independent Python implementation reproduces canon tests 3 and 6 and gives test 45 (`5797906573132458142`; `3, -2, 5`) and test 47 (`0x1c35d41d581f5eaa`). A scratch crate built against `palimpsest_math` (outside the repository) gives the same values, and test 46 gives `yaw_from_dir` = 0 and 16384 exactly.
- `cargo test -p palimpsest_math`: 196 passed. `cargo test -p palimpsest_data canon_spec_ids_are_collision_free`, which reads the canon: passed, so the new IDs have no `fnv1a32` collision.

## Rust code that canon-1.11 now contradicts (not changed)

- `palimpsest_math::rng::keyed_seed` computes `(seq as u64) << 48` as an unchecked shift, so a `seq` above `0xFFFF` silently loses bits. Canon §1.3 and §3.3 (G-021) require a panic. Its doc comment still calls `seq` a per-tick counter.
- `palimpsest_math::rng::streams::SEQUENTIAL` and `KEYED` lack `rng.run` and `rng.check`. The comments on `LOOT`, `WORLDGEN`, `LOOT_PLACE` and `PHYSICS` still give the canon-1.10 uses ("Fortune Engine reels" and the others).
- `palimpsest_sim::progression::combat_skill_xp_milli(dmg_mhp, target_max_mhp)` follows canon-1.10. Under G-070 its divisor must be `norm_mhp`, so callers must pass that or the function must compute it.
- `palimpsest_sim::movement::final_move_speed_mm_s` has no `body_move_bp` factor between row 11 and the statuses (G-035).
- `palimpsest_sim::movement::fall_damage_mhp` returns the damage before `fall_dmg_bp` and the doc 04 factors. That is compatible only if callers apply both in the G-034 order.
- Already consistent: `Rng::range_incl` (i128), `cordic::yaw_from_dir`, `status::ERASURE_HUNTER_STATS_RANK = Rank::Elite` with one hunter per Folio, and the damage pipeline's `st.taunted` at 2.12 and ward profiles.

## Open items for other owners

- `tools/spec_lint.py`'s `RETIRED` list does not yet hold the six newly retired IDs (orchestrator decision (a)), so the linter will not flag them in chapters.
- Doc 13 still defines `enemy.role.cavalry`, `ai.set.cavalry`, `ai.action.remount`, `ai.nav.mounted`, mounted cavalry rows and the `stablemaster` vendor. Doc 11 §12 still weights a `stablemaster` vendor in its shop draw. Both chapters need to apply §17.7.
- Doc 04 and doc 05 need to drop the "single hit of at least 50% of the pool" severing path (DECISIONS item 84).
- Triage C-06 tells doc 14 to follow doc 16's hot and warm migrations, which continue the Folio. DECISIONS item 76 says "resume from the last Folio-entry snapshot". This is chapter-owned, not a canon amendment, but the two disagree and the product owner may want to confirm which one stands.
