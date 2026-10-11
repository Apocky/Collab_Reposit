# Rust slice: canon-1.8 to canon-1.11

As of 2026-10-11. The slice was written against canon-1.8. This change brings
every piece of implemented code into line with canon-1.11
(`../docs/palimpsest/spec/00_CANON.md`, §23 changelog), the editor's notes
(`../docs/palimpsest/audit/canon_1_11_notes.md`) and DECISIONS.md items 81 to
92. Only files under `palimpsest/` changed.

Gates, all passing on Rust 1.99.0:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace                                   # 558 tests (503 before the review, 465 before)
cargo test --workspace --all-features --release          # 558 tests
```

The `state_hash` of the skeleton runs did not change: canon test 32 (save and
load) and test 31 (Folio isolation) pass unchanged.

## The four known contradictions

1. **`keyed_seed` shifted `seq` without a check.** `(seq as u64) << 48` is now
   the checked multiplication by `2^48` that canon §1.3 makes of every
   normative shift: `seq` in `0..=0xFFFF` works, and a larger `seq` panics
   (`palimpsest_math::rng::keyed_seed`, private `seq_shifted`). The doc comment
   no longer calls `seq` a per-tick counter; it describes the §3.3 allocation
   rule. The canon's reserved allocations are constants in
   `palimpsest_math::rng::seq`, a test checks they never overlap, the sim
   skeleton's running counter asserts it stays in `0x0000..=0x01FF`, and
   `stats::DEATHS_DOOR_SEQ` now reads `seq::COMBAT_DEATHS_DOOR` (`0xFFFF`).
2. **`rng.run` and `rng.check` were missing.** `streams::RUN` (sequential,
   owner doc 14) and `streams::CHECK` (keyed, owner doc 04) are added, last in
   their tables as the canon orders them, so `SEQUENTIAL` and `KEYED` now hold
   11 names each. `streams::owner` returns each stream's owning document. The
   comments of `LOOT` ("Fortune Engine dispensing"), `WORLDGEN` (loot-spot
   counts and positions), `LOOT_PLACE`, `PHYSICS` (explosion block drops) and
   `FIRE` (explosion ignition) follow canon-1.11.
3. **`combat_skill_xp_milli` divided by the scaled Max HP.** It now takes the
   target's §17.2 `party_hp_bp` and divides by
   `norm_mhp = max(1, floor_div(target_max_mhp × 10000, party_hp_bp))`
   (`progression::combat_norm_mhp`), so skill XP is normalized by the enemy's
   HP before party scaling (DECISIONS 85). Doc 04 test 6 still gives 500 solo;
   a 20,000-mHP hit on a party-of-4 standard enemy now gives 500 instead of 196.
4. **`final_move_speed_mm_s` lacked `body_move_bp`.** The chain is now base,
   row 11, doc 04's `body_move_bp`, then the statuses in ascending ID order
   (`BODY_MOVE_INTACT_BP = 10000`). A severed leg takes 4,515 mm/s to 2,257;
   with `st.slowed`, 1,579.

## Every canon-1.11 row against the code

"Updated" means the code changed, "conformant" that it already matched, and
"not in slice" that the slice has no code for the rule yet.

| Row | Canon-1.11 change | Status | Where |
|---|---|---|---|
| G-001 | §3.1 `range_incl` in `i128` | conformant; test 45 added | `math::rng` |
| G-007 | §1.9 `atan2_bam` convention, `yaw_from_dir` | conformant; test 46 added, docs | `math::cordic` |
| G-008 | §1.1 mod IDs use `x_<mod_id>` | updated | `data::ids` (`validate_id_origin`, `mod_id_of`), `data::registry` (`build` rejects `x_` in base content and alias old IDs; `insert_mod_content`) |
| G-010 | §1.6 JSON integer bound | conformant (already enforced); docs cite canon §1.6 | `data::content_check` |
| G-011 | §1.5 one-decimal display of small rates | updated | `sim::display_rate_per_s` |
| G-015 | §2.2 named boundaries; §2.5 hash on `b % 60 == 0`, `b >= 60` | conformant in behavior; named helpers added and used | `time::boundary_after`, `time::is_hash_boundary` |
| G-016 | §2.4 intents in the replay log | not in slice (the skeleton applies no intents) | |
| G-017 | §2.4 `presentation_hash` split from `content_hash` | updated: replays record `presentation_hash`, mismatch only warns | `sim::ReplayLog`, `ReplayCompat`, `is_presentation_record` |
| G-018 | §2.4 `run_seed` in saves and replays, not lobby handshakes | conformant; `SessionHeader` doc | `sim::SessionHeader` |
| G-019 | §2.5 save deferred while slot 4 holds an invader | updated | `Sim::save_timing`, `SaveTiming` |
| G-020 | §2.3 light overrides clamp every light read | not in slice (no light grid) | |
| G-021 | §3.3 `seq` range and allocations | updated (contradiction 1) | `math::rng` |
| G-022 | §3.3 object-keyed draws | updated | `rng::object_keyed_rng`, `OBJECT_KEY_TICK` |
| G-023 | §3.3 `rng.check` | updated (contradiction 2) | `rng::streams` |
| G-024 | §3.3 explosion exception; `rng.physics` drops | updated (comments; no explosions in slice) | `rng::streams` |
| G-025 | §3.3 "Used for" paragraph | no code impact | |
| G-026 | §3.3 `rng.loot` Fortune Engine dispensing | updated (comment) | `rng::streams::LOOT` |
| G-027 | §3.2 labeled sub-streams `derive` | updated; test 47 added | `rng::derive`, `sub_stream_seed`, `sub_stream_rng` |
| G-028 | §3.2 `rng.worldgen`, `rng.loot_place` uses | updated (comments) | `rng::streams` |
| G-029 | §3.2 `rng.run` | updated (contradiction 2) | `rng::streams` |
| G-030 | §3.5 offer generator | updated | `rng::offer_seed`, `offer_rng`, `Rng::peek_u64` |
| G-031 | §3.6 account draws and Keepsakes | no code impact (account draws live outside the simulation) | |
| G-032 | §3.8 presentation generators | updated | `rng::nar_rng`, `rng::pres_hash` |
| G-033 | §4.4 liquid units, gas concentration | not in slice | |
| G-034 | §4.2 fall damage × `fall_dmg_bp`, then doc 04 factors by source ID | updated | `movement::FallFactor`, `apply_fall_multipliers` (asserts ascending source IDs), `fall_damage_chain_mhp` |
| G-035 | §4.2 `body_move_bp`; stance bases; armor never multiplies speed | updated (contradiction 4) | `movement::final_move_speed_mm_s` |
| G-036, G-037 | §4.3 `block.tag.destructible`; fixed terrain stops projectiles | not in slice (no block world) | |
| G-052, G-053 | §5.2 Lucre skins and naming rule ("Crowns" kept, DECISIONS 90) | conformant; now checked against the canon text | `genre::tests::genre_tables_match_canon_text` |
| G-055 | §5.3 supernatural alchemy uses the aether axiom; doc 04 grafts arcana; holy alchemy mythos; licensed grafts | updated | `contradiction::ComponentKind` (`Casting`, `Enchantment`, `Sacred` docs; new `LicensedGraft`) |
| G-056 | §5.5 failed potion; §5.11 counts failed potions | updated (docs; the window counts any failure) | `contradiction::UNMOORED_FAILURES` |
| G-059 | §5.6 Shock of the New excludes `rank.boss`, `rank.editor`, `rank.hunter` | updated: takes the target's rank | `contradiction::shock_of_the_new` |
| G-060 | §5.8 seam `local_genre(pos)` is the Folio's genre | updated: the §5.5 genre overrides and the AETHER row read `LocalGenre::genre()` | `contradiction::LocalGenre::genre`, `evaluate` |
| G-067 | §6.4 row 29 GLYPH term per glyph of the producing stage | updated (doc; the caller chains one term per glyph) | `stats::spell_potency_bp` |
| G-070 | §7.1 `norm_mhp` before party scaling (DECISIONS 85) | updated (contradiction 3) | `progression::combat_skill_xp_milli` |
| G-071 | §8 45-unit anchor | updated (constant and doc) | `progression::XP_UNITS_PER_FOLIO` |
| G-072 | §8 downed members and ghosts share kill XP (DECISIONS 81) | updated | `progression::PartyPresence`, `shares_kill_xp` |
| G-073 | §8 pacifications pay kill XP | updated (doc) | `progression::kill_xp` |
| G-080 | §9.1 Errata limbs sever only from a crippled pool (DECISIONS 84) | updated: severing implemented | `damage::severs`, `SeverInput`, `SeverPart` |
| G-081 | §9 added-part donors | not in slice (no body plans) | |
| G-084 | §10.4 attack factors before step 0, "× N" traits after | updated | `damage::apply_attack_factors`, `apply_manufacturer_trait` |
| G-085 | §10.5 step 2.12 `st.taunted` | conformant | `damage::chain_trace` |
| G-086 | §10.5 step 3a fae iron × 12500 | updated | `Hit::fae_bane`, `Defender::fae`, `per_type_target_mods`, `FAE_IRON_BP` |
| G-087 | §10.5 step 5 shield AR on blocked hits | updated: `BlockInput::shield_ar` is added to the part's AR | `damage::finish` |
| G-088 | §10.7 split-back | conformant; docs | `damage::apply_block` |
| G-089 | §10.5 step 6c Scribe's Mercy | updated | `Defender::mercy_bp`, `apply_mercy`, `mercy_reduce_mhp`, `DamageBreakdown::mercy_mhp` |
| G-090 | §10.5 step 7 per-pool `m[t]`, `counter.ward_breaker` | conformant | `damage::absorb_wards` |
| G-091 | §10.6 consumable entry | updated | `damage::resolve_consumable`, `Entry::Consumable` |
| G-092 | §10.6 lethal takedown entry | updated | `damage::resolve_takedown`, `Entry::Takedown`, flag `takedown` |
| G-093 | §10.1 blunt poise `10000 + floor_div(share_blunt, 2)` | conformant in value (doc 05's pro-rata form was equal); now written in the canon form | `damage::blunt_poise_bp`, `poise_damage_base_mp` |
| G-094 | §10.8 two hit batches | updated | `damage::HitBatch`, `HitKey::batch` |
| G-108 | §13.8.1 casting implements | updated (doc) | `damage::SourceLevel::Spell` |
| G-109 | §17.3a `counter.disruptor` exactly 180 ticks, unscaled | updated | `status::DISRUPTOR_TICKS`, `DISRUPTOR_APPLIES` |
| G-110 | §11.1 `st.downed` later downs per doc 16 §9.4 | updated: 1,800, 1,200, then 600 ticks | `damage::downed_bleed_out_ticks`, `LethalInput::prior_downs_folio` |
| G-111 | §11.1 `st.marked` wording | no code impact (the catalog holds no effect text) | |
| G-112 | §11.6 Erasure hunter: `rank.hunter` identity, `rank.elite` multipliers, no repeat spawns (DECISIONS 91) | updated | `status::Bearer::erasure_hunter`, `Bearer::ERASURE_HUNTER`, `Bearer::multiplier_rank`; `progression::multiplier_rank`, `ERASURE_HUNTER_MULTIPLIER_RANK`, `enemy_max_hp_mhp` |
| G-113 | §11.6 invaders never erase placeables | updated | `status::erases_placeables` |
| G-114 | §11.8 neutral substitute for a missing command | updated: the skeleton no longer keeps a missing slot's movement and buttons | `sim::neutral_substitute`, `buttons::HOLD_MASK`, `Sim::sys_input_apply` |
| G-119 | §12.1 gas contact at 500, respirators | not in slice | |
| G-124, G-125 | §12.2 tall grass; §12.3 initial population | not in slice | |
| G-128 | §13.1 `cat.gadget`, `cat.placeable` rows | conformant (IDs only) | `loot::Category` |
| G-129 | §13.2 ward capacity chain | updated | `loot::ward_capacity_mhp`, `ward_recharging`, `ward_recharge_per_s` |
| G-130 | §13.2 `slot.hand_extra_1..2`; §13.3 Exotic limit | updated | `loot::EquipSlot` (23 slots), `EquipSlot::exotic_group` |
| G-131 | §13.7 `ENH_CAP`; §13.9 enhancement range | updated: `damage::enhancement_bp` no longer panics above +15 (Marginalia) | `loot::enhancement_cap`, `eff_enh`, `damage::enhancement_bp` |
| G-132 | §13.8 table uses | updated (docs) | `math::tables` |
| G-133 | §13.8 `H(d)` row adds `diff_hp_bp` | updated: the enemy HP chain is implemented | `progression::enemy_max_hp_mhp` |
| G-134 | §13.9 illuminated roll keying | not in slice (no item generation) | |
| G-148, G-150 | §14 Lucre per Erratum, welcome Ink | not in slice | |
| G-159 | §16.2 kit ownership | no code impact | |
| G-161 to G-167 | §17.1 Colophons, counters, `FOLIO_1_GENRE`, `BOSS_FOLIOS`, Abridged loot | not in slice (no run layer) | |
| G-170 | §17.3 champion aspects | no code impact | |
| G-173 | §17.9 Redaction ignores downed Errata | not in slice (no invasions) | |
| G-184 | §18 events registered by doc 03 | no code impact | |
| §17.7 | retired mount and cavalry IDs | updated: see below | |

## Drift from canon-1.8 to canon-1.10 fixed on the way

- **Mounts (retired since canon-1.8, §17.7).** `st.mounted` and its
  `WhileRiding` condition are removed (`STATUS_COUNT` 62 to 61),
  `cat.mount` is removed (`Category::ALL` 32 to 31), and the §5.3 mount
  component kind (`ComponentKind::Mount`, `MountSkin`) is removed.
- **Manufacturer traits (canon-1.10, no durability).** Rustmaw now reads
  "magazine × 20000, reload time × 13000" and Hephaesteum "the wielder is
  immune to st.disarmed and st.corroded"; Somnambule follows canon-1.11.
- **Pact curses (canon-1.9).** `EnhanceState::pact_boon`; an Emendation that
  uncurses a pact item removes the boon with the curse.

## Tests added (38)

- Canon §21: `canon_t45_range_incl_extremes`, `canon_t46_yaw_from_dir`,
  `canon_t47_sub_stream`. They follow the workspace convention
  `canon_tNN_<short_name>` from `AGENTS.md`.
- Doc 17: `doc17_t02_mercy_damage`, `doc17_t03_mercy_and_wards`.
- Canon-text cross-checks, skipped when the spec is not next to the
  workspace: `genre_tables_match_canon_text` (§5.2 and §5.9, every copied
  column and every manufacturer signature) and `catalog_matches_canon_text`
  (§11.1, every row and column). The content-ID test now builds base IDs as
  base content and loads the canon's `x_frostpack` example as a mod record.
- `palimpsest_math`: `labeled_sub_streams`, `keyed_seq_upper_bound_is_valid`,
  `keyed_seq_above_range_panics`, `object_keyed_draws`,
  `reserved_seq_allocations_are_disjoint`, `offer_generator`,
  `presentation_generators`, proptest `range_incl_matches_canon_i128_reference`.
- `palimpsest_data`: `mod_segment_rule`, `mod_segment_rule_in_registry`.
- `palimpsest_sim`: `combat_xp_normalized_before_party_scaling`,
  `combat_norm_zero_party_bp_panics`, `kill_xp_recipients`,
  `erasure_hunter_multipliers`, `enemy_hp_chain`,
  `unsorted_fall_factors_panic`, `blunt_poise_for_mixed_hits`,
  `same_tick_hits_resolve_in_two_batches`, `severing_rules`,
  `step_3a_fae_iron`, `entry_consumable`, `entry_takedown`,
  `step_0_attack_factors_and_manufacturer_trait`,
  `erasure_hunter_identity_and_multipliers`, `disruptor_is_unscaled`,
  `ward_capacity_chain`, `presentation_hash_split`,
  `named_boundaries_and_hash_boundaries`, `save_timing_rules`,
  `small_rate_display`, `missing_command_gets_neutral_substitute`.
- Extended: the damage proptests now draw Mercy bearers and run the new entry
  points; `speed_composition`, `fall_damage`, `default_dep_table`,
  `anachronism_edge`, `seam_zones`, `lethal_resolution_order`, `emendations`
  and `palimpsestic_and_exotic` cover the canon-1.11 rules.

## Defaults chosen where canon-1.11 is silent (marked `GAP:`)

- Fall damage factors multiply the amount before it enters step 4.
- Hit batches: shock chains and reflected damage resolve late (doc 03 §2.2
  row 26 lists chains there), consumables and takedowns resolve with the
  actor batch. Fall damage resolves in the first batch, because doc 03 §5 and
  §5.15 queue the fall hit "for `sys.damage`" (corrected in the review below;
  this line first said fall damage counted as environmental).
- Severing applies the crippled-pool rule to every severable Erratum part,
  not only limbs.
- The §10.7 split-back remainder goes to the largest-share type among types
  that still carry damage.
- A takedown runs step 7 with the usual pool multipliers.
- Rates below 10 display units: negative rates also display in tenths.
- An empty `x_` mod segment matches no mod; the mod-namespace error code
  `mod_segment_invalid` is provisional.

## Spec issues found while implementing

See the structured summary returned with this change; the same list is
repeated here for the record.

1. §1.3 reads every `x << k` in a normative formula as a checked
   multiplication, which literally includes the §3.1 reference
   `self.s[1] << 17`; that shift must truncate (it defines xoshiro256++ and
   §21 tests 1 to 3). §1.3 should exempt the §3.1 generator code.
2. §10.6 takedown: with a generator whose doc 05 profile scales `dmg.true`
   below 10000, step 7 absorbs more than the ward's amount, so a takedown
   "equal to the target's HP plus ward" may not kill. The canon should fix
   `m[true] = 10000` for takedowns or define the HP loss directly.
3. §10.7 split-back: "the remainder to the largest-share type" can name a
   type whose `d[t]` is 0 (immune, or zeroed by resistance), unlike step 6,
   which says "non-immune". The canon should restrict it the same way.
4. §10.8 names only actor hits and "environmental, contact, surface and DoT
   hits"; shock chains, reflected damage, fall damage, consumables and
   takedowns are unclassified (doc 03 row 26 puts chains late; doc 03 §5.15
   queues fall hits for `sys.damage`, the first batch).
5. §4.2 fall damage "enters the damage pipeline at step 4 ... It is then
   multiplied by `fall_dmg_bp`": the order of the multipliers relative to
   step 4 is ambiguous.
6. §9.1 "An Erratum's limb severs only if ...": Errata tails and wings
   (mutations, §9) are not covered.
7. §10.6 consumable entry and §10.5 step 6c: whether Scribe's Mercy reduces a
   consumable's damage to its own drinker is unclear ("HP the Erratum chooses
   to pay is never reduced"); the slice applies 6c, as the entry passes
   through steps 3 to 7.
8. §8 co-op split: `n` is "party size"; whether Dormant and Departed slots,
   which are excluded from the split, still count in `n` is not stated.
9. §1.5 "per-second rates below 10 display units" does not say how negative
   rates display.
10. §5.8 and §5.5: with `local_genre(pos)` the Folio's genre in a seam, a
    seam that stitches in `genre.ur_text` or `genre.unwritten` as the other
    genre does not get their overrides; the canon could say seams never use
    secret genres.
11. §13.7 `ENH_CAP = 15 + floor_div(d_current - 25, 5)` has no depth cap,
    while §13.8 caps every table at depth 300; the canon could say whether
    `ENH_CAP` also uses `min(d, 300)`.

## Review

An adversarial review re-derived each changed rule from the canon
(canon-1.11, §23 and `audit/canon_1_11_notes.md`), DECISIONS.md items 81 to
92 and the owning chapters (docs 03, 04, 05, 08, 16, 17), then read the diff
line by line. The canon test vectors were recomputed in exact-integer Python:
test 45 (`5797906573132458142`; `3, -2, 5`), test 47 (`0x1c35d41d581f5eaa`)
and tests 6 and 7 all match. Tests 1 to 47 are all present and pass. Bp
chains run in the listed order, the new shifts are checked, and no floats,
`HashMap` or `HashSet` were added outside tests. The defects below were found
and fixed. Nothing was committed.

### Defects found and fixed

1. **Fall hits were put in the wrong §10.8 batch.** `HitBatch::of(Entry::Fall)`
   returned `Late`, a default the change chose by treating fall damage as
   environmental. Canon §10.8 defines the two batches as doc 03's
   `sys.damage` and `sys.damage_late`, and canon §2.2 makes doc 03 the owner
   of tick order. Doc 03 §5 and §5.15 queue the fall hit "for `sys.damage`"
   from `sys.movement` (#8), so it resolves in the first batch, after an
   actor hit from a lower source and before every DoT pulse. Fixed in
   `damage::HitBatch::of`, with a rewritten `GAP:` comment and an updated
   batch test, plus the new test `every_entry_has_its_doc03_batch`. The
   "Defaults chosen" bullet and spec issue 4 above are corrected.
2. **Consumable buildup skipped part of the §11.2 chain.** `resolve_consumable`
   fixed the base-component potency at 10000 and the edge at 0. Doc 08 §7
   says a consumable's `buildup` effects enter "§11.2 (× `S(src_lvl)` and the
   rest of that chain)", which includes the potency (and canon §5.5 says
   potency scales buildup) and, against a native target, `10000 + edge_bp`.
   The function now takes `potency_bp` and `edge_bp`, as `resolve_hit` does;
   the damage stays `D2 = B`. Test: `consumable_buildup_takes_potency_and_edge`.
3. **Stale `Sim::step` doc.** It still said a slot without a command "keeps its
   held state", which contradicts the canon §11.8 neutral substitute added in
   this same change. The doc now describes the substitute and the
   `HOLD_MASK` bits it keeps. The doc says a host that logs the replay (canon
   §2.4: "the replay log records it") can pass `neutral_substitute` itself and
   get the same result. The proptest `missing_commands_equal_explicit_substitutes`
   now proves that tick by tick.
4. **The severing threshold's divergence from the canon was undocumented, and
   a test comment was wrong.** `severs` reads canon §9.1's "at most 2500 Bp of
   its max" through doc 04 §8.2's floored `frac`, which admits 2,657 of 10,625
   (25.007%). The test called that pool "exactly 25%". `severs` now has a
   `GAP:` paragraph stating the reading, the test comment is corrected, and
   `AGENTS.md` lists it as a known spec tension.
5. **`enemy_source_mhp` did not say which rank to pass.** `kill_xp` and
   `enemy_max_hp_mhp` tell callers to pass the multiplier rank. The §10.4
   enemy source did not, so passing the Erasure hunter's identity rank would
   give `rank.hunter`'s 20000 damage instead of `rank.elite`'s 13000 (canon
   §11.6, §17.3). The doc is fixed. Test: `erasure_hunter_source_uses_elite_damage`.
6. **Two progression test comments made false claims.**
   - `combat_xp_normalized_before_party_scaling` said draining the
     party-scaled enemy "grants 1500 in total". It grants 3,825 across the
     party (2.55 standard kills). The comment is corrected and the 3,825 is
     asserted.
   - `xp_ranges` said "the same share of the party-scaled target" grants the
     same XP. It is the same damage that does. The comment is corrected, and
     the new property `norm_mhp_recovers_template` checks the real invariant
     for every §17.2 HP row: `norm_mhp` is the template's Max HP or one less
     (exact when solo), so equal damage earns equal skill XP within 1 milli-XP.
7. **The `keyed_rng` doc misplaced the loot example.** It gave "the receiving
   player's slot" as a running-counter range. Loot's `seq` is a fixed
   allocation (canon §3.3). The wording is corrected.
8. **The new-generator tests only restated the formulas.** The `seq = 0xFFFF`,
   object-keyed, sub-stream, offer, `nar_rng` and `pres_hash` tests compared
   each function against its own formula typed again in Rust, so they could
   not catch a formula error made in both places. `canon_1_11_generator_vectors`
   now pins every seed to values computed independently in Python.
   `stream_tables_match_canon_text` checks the §3.2 and §3.3 stream names,
   their order and their owners against the canon file.
9. **Counts and layout in `README.md`, `AGENTS.md` and this file.** The
   public-function count was 691, not 690 (the same script counts 645 at
   HEAD). The test count is now 558. `AGENTS.md` now maps doc 17 test 45. The
   README Layout still called `content/` and `schemas/` empty and listed only
   `gen_trig.py`; it now names the doc 19 content, schemas and Python
   validator that sit outside the crates.

### Tests added by the review (55; 503 → 558)

- `palimpsest_math` (9): `range_incl_top_and_point_extremes`
  (`range_incl(i64::MIN + 1, i64::MAX)` = 5797906573132458143; one-point
  ranges at both `i64` ends), `canon_1_11_generator_vectors`,
  `keyed_rng_rejects_u32_max_seq`, `object_keyed_purpose_above_range_panics`,
  `stream_tables_match_canon_text`, `canon_t46_cardinals_and_diagonals`, and
  the proptests `peek_is_next_without_advancing`,
  `keyed_seq_checked_shift_matches_plain` (seq 0..=0xFFFF) and
  `object_keys_use_folio_field`.
- `palimpsest_data` (2): proptest `mod_segment_property`, and
  `mod_records_collide_and_never_partially_register`.
- `palimpsest_sim` (44):
  - damage: `every_entry_has_its_doc03_batch`,
    `blunt_poise_rejects_share_above_10000`,
    `blunt_poise_rejects_negative_share`, `mercy_above_full_panics`,
    `enhancement_factor_bounds`, `doc17_t45_mercy_exemptions`,
    `shield_ar_only_on_blocked_hits`, `erasure_hunter_source_uses_elite_damage`,
    `takedown_edges` (pins the takedown `GAP:` with a halved true profile),
    `consumable_edges`, `consumable_buildup_takes_potency_and_edge`, and the
    proptests `prop_severing_invariants`, `prop_downed_bleed_out`,
    `prop_blunt_poise`, `prop_mercy_conserves`, `prop_step0_factors`,
    `prop_takedown_default_profile` and `prop_shield_ar_never_hurts`.
  - sim: `rate_display_extremes`, `presentation_namespace_edges`,
    `replay_log_needs_full_presentation_hash`,
    `boundary_after_last_tick_panics`, `tick_seq_stays_in_reserved_slots`
    (seq `0x01FF` is the last one, `0x0200` panics), and the proptests
    `neutral_substitute_never_presses` and
    `missing_commands_equal_explicit_substitutes`.
  - progression: the proptests `norm_mhp_recovers_template` and
    `enemy_hp_chain_order_and_monotone`, plus `multiplier_rank_is_exhaustive`,
    `rank_tables_agree_with_status` and `kill_xp_presence_is_exhaustive`.
  - movement: the proptests `body_factor_in_speed_chain` and
    `fall_chain_order`, plus `duplicate_fall_factor_source_panics`.
  - status: `bearer_multiplier_rank_is_exhaustive`,
    `erasure_hunter_durations_match_elite` and `erases_placeables_truth_table`.
  - contradiction: `seam_local_genre_is_the_folio_genre` (every seam pair)
    and `shock_of_the_new_truth_table`.
  - loot: the proptests `ward_capacity_chain_order`, `ward_recharge_boundary`
    and `pact_boon_leaves_with_the_curse`, plus `equip_slots_match_canon_text`
    and `ward_capacity_rejects_ilvl_zero`.
  - time: `hash_boundary_extremes`.

### Gates after the review

`cargo fmt --all -- --check` is clean. `cargo clippy --workspace --all-targets
-- -D warnings` is clean, and so is the `--all-features` variant.
`cargo test --workspace` passes 558 tests (data 47, math 216, sim 293,
no_floats 2). `cargo test --workspace --all-features --release` passes the
same 558, and `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` is
clean.

### Spec issues found by the review

12. §10.8 against doc 03 §12.4 and doc 05 §1.3: canon §10.8 puts
    "environmental" hits in the late batch. Doc 03 §12.4 queues every
    explosion's hits "for `sys.damage`", environmental explosions (source
    `Env`) included, and doc 05's C6 lists "the area hits of their
    explosions" in the first batch. The code follows the canon
    (`Entry::Environmental` resolves late). One of the documents needs to
    change.
13. §9.1 against doc 04 §8.2: "at most 2500 Bp of its max" is exact, while
    doc 04's `frac` floors. They disagree for pools within one `frac` step
    above a quarter, for example 2,657 of 10,625. The canon could say
    "`frac <= 2500` (doc 04 §8.2)".
14. §10.6 consumable entry against §5.5: potency "multiplies everything the
    component produces: damage, ...". But a consumable's damage enters at
    step 3 with `D2 = B`, after step 2.5's potency, and doc 08 §7's `B` chain
    has no potency term, so a Strained harmful potion deals full damage
    while its buildup is scaled. The canon should say whether `B` carries the
    potency.
