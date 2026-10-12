# Fix log: chapter 13 (Enemies, AI, Factions and NPCs)

As of 2026-10-11. Chapter fixer for `spec/13_ENEMIES_AI_FACTIONS.md`, conforming the chapter to canon-1.11.

Inputs read: DECISIONS.md (items 1 to 88 and the orchestrator rows), `spec/00_CANON.md` (already at canon-1.11 on disk), `audit/canon_gaps_triage.md` (every row naming doc 13, §4 and §4.1), `audit/notes_enemies.md` and the doc 13 lines of the other `notes_*.md`, and the lint before edits (`--chapter 13`: 5 retired IDs, `arch.mount.hover`, `arch.mount.runner`, `cat.mount`, `room.no_mount`, `st.mounted`; `--json` multiply_defined: only `ai.curve.logistic_up`, `bell_mid`, `plateau` shared with doc 02).

## Plan (status updated after each batch)

1. Header and Imports to canon-1.11.
2. Mounts cut (DECISIONS 55, canon §17.7, C-30, orchestrator row): `enemy.role.cavalry` → `enemy.role.charger`, `ai.set.cavalry` → `ai.set.charger`; delete `ai.nav.mounted`, `ai.action.remount`, the `mount` schema field, `mount_link`, the `mounted` tag (bit 22 reserved), `npc.vendor.stablemaster`, §5.6 (one-sentence retirement note), mount XP and threat rules, the War Boss rig, test 33; convert the 8 cavalry roster entries.
3. Erasure hunter (DECISIONS 26, canon §11.6 and §17.1, G-112, triage Q12, orchestrator row): one per Folio, `rank.hunter` identity and immunities, `rank.elite` multipliers and party row, no 5400-tick repeat; Erasure clock to canon §17.1's 43200-tick onset.
4. C-23 (`block.soil`), C-24 (`is_errata_placed`, `erase_placed`).
5. Canon-1.11 alignment: G-070 (co-op skill normalization; recomputed §11.2 co-op row), G-071 (45-unit anchor), G-073 (pacification in canon §8), G-109 (disruptor duration), G-113 (invader never erases), G-023 (`rng.check` exists; doc 13 stays drawless), G-007 (`yaw_from_dir`), canon §3.3 seq allocation.
6. Canon gaps: delete every resolved gap (triage rows G-023, G-047, G-051, G-070, G-071, G-073, G-074, G-075, G-113, G-156, G-157, G-158, G-170, G-172, G-176, G-177, G-181, G-182, G-183, G-184, G-191, G-195; §4.1 stale gaps 15, 17, 22, 23 and the "Redactor's archive" request), keep only real open items.
7. Exports, Interface assumptions, Acceptance tests and Invariants to match.
8. Lint.

## Batches done

### Batch 1 (done): header, Imports, schema, tags, seq allocation
- Header → "Conforms to canon-1.11" (G-051).
- Imports: canon §3 row notes the §3.3 seq-allocation rule and that `rng.check` (G-023, canon-1.11 §3.3, owner 04) is never drawn here; canon §17 row names the mount retirement and `enemy.role.charger` (§17.7, orchestrator row); the canon-1.3 row now covers canon-1.11 (§7.1 normalization G-070, §8 pacification and 45 units G-071/G-073, §11.6 hunter G-112, §17.1 Erasure clock, §17.7 retired IDs, invader never erases G-113); doc 03 row adds `apply_knockback` (§5.13) and `is_errata_placed`/`erase_placed` (§13.8, C-24); doc 05 row drops "§17.3 mounted combat" (C-30) and adds the §1.4 keyed slots; decision row cites items 15, 16, 26, 55, 65, 69, 85 and the orchestrator rows.
- §2.1: removed the `mount` schema field and `"mount": null` from the JSON example (C-30).
- §2.4: `mounted` tag removed from the closed vocabulary; tag bit 22 reserved with a one-clause retirement note; `flier` row drops the stale "doc 09 must read `flier`" (G-183, §4.1 gap 23); kin list names `wild_huntsman` (renamed, batch 2). Test 38 rejects bit 22.
- §4.3 bearing written as canon §1.9's `yaw_from_dir(dx, dz)` (G-007).
- §4.1: new **Seq allocation** paragraph (canon-1.11 §3.3: rng.ai seq 0 and 1 only; combat/magic/status slots from docs 05, 06; loot seq = slot).
- §4.6: "doc 06 G10" → "doc 06" (G-117 closed).

### Batch 2 (done): mounts cut, cavalry → charger (DECISIONS 55, canon §17.7, C-30, G-172, orchestrator row)
- §3: `enemy.role.cavalry` → `enemy.role.charger` (standard, HP 70, damage 12, cycle 90, reach 3000, walk/run 4000/7000, `humanoid` nav, BRN 50 + 20 armor poise = 65, AR 12, stamina 120, impact 25, `ai.set.charger`). New paragraph defines the niche and a worked hit (15,000 mHP at Folio 1). Role ability `enemy.ab.lance_charge` respecified on foot: w30 `line` telegraph, 10000 mm/s run up to 16000 mm (≤ 96 active ticks), × 12500, `impact_mp` 50000 (the old × 20000 poise), one hit per touched hostile with `apply_knockback(target, dir, 4000)` (doc 03 §5.13), continues past bodies, stops at solid block, closed door or sealed arena boundary; recovery 45, cooldown 480.
- §4.2 removed `mount_link`; §4.7 deleted `ai.action.remount` (34 → 33 actions), `ai.set.cavalry` → `ai.set.charger` (melee, charge, advance, circle), charge notes "brutes, chargers", breach nav list drops `mounted`; §4.8 drops the mount-threat rule; §4.13 deleted `ai.nav.mounted` (7 → 6 nav classes).
- §5 title drops "mounts"; §5.6 replaced by a one-sentence retirement note (orchestrator instruction).
- §7.2 `elite.phasing` no longer excludes a role (its only exclusion was cavalry).
- §8 roster, 8 entries converted to unmounted chargers (body humanoid, `mounted` tag removed). Renamed where the old name described a mount or vehicle: `hover_lancer` → `jet_lancer`, `war_rig_rider` → `ram_raider`, `posse_rider` → `posse_deputy`, `getaway_driver` → `getaway_gunman`, `cavalry_general` → `jade_general`, `wild_hunt_rider` → `wild_huntsman`; kept `kingsguard_lancer` and `archetype_knight`. Signature `enemy.ab.drive_by` → `enemy.ab.running_fire`; `strafing_run` and `ram` restated as foot charges; `ram` dep techne → none (no motorbike). Loot roles: lancers, ram raider, general, huntsman, knight → `brute`; jet lancer, deputy, gunman stay `marksman`.
- §9.3 War Boss fights on foot in a ram harness: `rig_ram` 12000 mm/s up to 24000 mm with the charger trample; P2 no longer depends on a rig dying.
- §11.1 XP-eligible drops "not a mount"; §13.2 drops `stablemaster`: 10 vendor types.
- Exports: role list, 6 roster lists, `enemy.ab.*` (drive_by → running_fire, still 131), `ai.action.*` 33, `ai.nav.*` 6, `npc.vendor.*` 10. Interface 05 and 11 drop mount riders and mount tokens; 11 now says 10 vendor types. Test 33 replaced by a charger test; test 55 says 10 shops. New invariant line for chargers and no mounts.

### Batch 3 (done): Erasure hunter, C-24, XP attribution
- §10.5 (DECISIONS 26, canon §11.6 and §17.1, G-112, triage Q12, orchestrator row): the clock now follows canon §17.1 (`c = 43200 + 1800 × (k - 1)`, at most 10 stacks; threshold 10 at 59,400, 8 at 55,800, 6 at 52,200), replacing doc 14's stale 25200 onset. Spawn: exactly one hunter per Folio, no 5400-tick repeat, none if the party reaches the Colophon first. New **Rank** bullet: `rank.hunter` identity, immunities, post-proc protection and presentation; `rank.elite` multipliers and the minion-to-champion party row; doc 14's hunter-scoped `diff_hp_bp` factors still apply (they key on the rank ID). New example: 558,537 mHP solo at Folio 8, 1,424,269 with 4 players, 3,397,863 solo at Folio 23, 45,383 mHP hit, 317 XP. Colophon hunters keep full `rank.hunter` multipliers and the boss row (canon §17.1, §17.3) and never count as the Erasure hunter.
- §2.2 party row and §10.1 mSKE units name the Erasure hunter's elite values; §11.1 new **Hunters** bullet (elite kill XP for the Erasure hunter; `rank.hunter` for Colophon hunters and the invader); §8.17 revisor row notes the elite multipliers.
- §10.6 (C-24): uses doc 03 §13.8's `is_errata_placed(c)` and `erase_placed(b, m)`; the separate `placed_by_errata` bit request is gone; the invader exception now cites canon §11.6 directly (G-113).
- §10.7 kit unlocks cite doc 14's `unlock.` nodes instead of "Canon gaps" (G-176).
- Test 19 rewritten for the new clock, single hunter and elite numbers; invariants: Erasure clock := canon §17.1, at most one Erasure hunter per Folio, `is_errata_placed` replaces `placed_by_errata`.
- Interface 03 row: `is_errata_placed`/`erase_placed` and `apply_knockback`; Interface 08 row cites doc 08 §2.7 instead of doc 08's gap 18 (C-22 deletes that gap).

### Batch 4 (done): canon-1.11 XP alignment, Canon gaps
- §11.2 (G-071): results paragraph now reads 45.5 units "on Canon §8's 45-unit anchor"; (G-070, DECISIONS 85): the co-op sensitivity row was recomputed with canon-1.11 §7.1 normalization (each member deals an equal damage share; count scaling with per-Folio carry; Keeper, mini-boss, Editor and adds not count-scaled): level 28 / 28 / 28, main skill 50/75 at Folios 10/24, 10/23, 9/22 for parties of 2/3/4 (all inside canon §7.2). A script reproducing the solo reference table row for row was used. The co-op paragraph gives the per-member factor `floor_div(party_hp_bp × enemy_count_bp[n], 10000 × n)` = 10000, 10500, 11156 Bp. Test 23 adds the co-op values.
- §11.1 and §6 (G-073): pacification is a canon §8 source; defeat bullet names `ev.entity_pacified` instead of `ev.entity_killed`; co-op split notes downed members and ghosts (G-072).
- §7.3 (G-109): disruptor applies its statuses for exactly 180 ticks, unscaled by canon §11.4.
- Canon gaps rewritten: all 26 earlier gaps closed (1 G-051; 2 G-073/G-184; 3 G-074; 4 G-075; 5 G-170; 6 G-071; 7 G-070; 8 G-047; 9 G-156; 10 G-158; 11 orchestrator row/C-30; 12 G-182; 13 G-191; 14 G-181; 15, 17, 22, 23 G-183 and §4.1; 16 G-113/G-177 and the stale "Redactor's archive" request; 18 G-195; 19 G-023; 20 G-176; 21 G-157; 24 G-184; 25 and 26 were records of adopted requests). Open now: roster size (DECISIONS 69), doc 14's stale Erasure clock, and the two unconfirmed defaults (reconciler Q4, Q5).

### Batch 5 (done): follow-ups
- C-23: `block.soil` was already on disk in the `sand_wyrm` row and the doc 07 Interface row; no `block.dirt` remains. Nothing to change.
- §8 `jet_lancer` `strafing_run` and `getaway_gunman` `running_fire` restated exactly (active at most 52 ticks for the jet charge; hitscan burst from the first tick within 6000 mm).
- Imports doc 14 row: doc 14 supplies `clock_start` and `colophon_entry_tick`; the schedule is canon §17.1's. Interface 14 row asks for an Erasure schedule matching canon §17.1 and §11.6.
- Invariant: boss loot is doc 11's `loot.boss.mini` / `loot.boss.editor` plus `loot.volume_reward` (matches §9.1; notes_items rename).
- Lint after batches 1 to 4: `--chapter 13` clean (0 retired, 0 undefined, 0 foreign definitions, 0 JSON issues, 0 hedges). `--json` multiply_defined lists only `ai.curve.logistic_up`, `bell_mid`, `plateau` (doc 02's curve-generator JSON declares IDs in doc 13's `ai.` namespace; doc 02 must change).

### Batch 6 (done): last alignments
- Rule 6 and the canon §4 import row name the canonical tag `block.tag.destructible` (canon-1.11 §4.3); "destructible-tagged" keeps its meaning.
- §7.3 counter sourcing follows canon-1.11 §17.3a (G-161): every loot pool and every shop, plus doc 11's counter caches in `genre.unwritten`.
- Test 52 adds "never draws `rng.check`" and "keyed `rng.ai` seq 0 or 1".

## Final lint
`python3 tools/spec_lint.py --chapter 13`: `== 13  words=36882`, no findings. `--json`: chapter 13 has no undefined IDs, foreign definitions, retired IDs, JSON issues or hedges; multiply_defined involving 13 is only doc 02's `ai.curve.*` (request below).

## Requests for other chapters
- **Doc 02**: its curve-generator JSON declares `ai.curve.logistic_up`, `ai.curve.bell_mid` and `ai.curve.plateau` as `"id"` values in doc 13's `ai.` namespace (canon §19). Rename the JSON key (for example `"curve"`) or cite doc 13 §4.4 so the IDs are referenced, not declared.
- **Doc 11**: canon §17.7 retires `shop.stablemaster`; delete the shop, its vendor-draw weight (Σ becomes 17), its "mount mending" service row and the mount-token loot lines; doc 13 now has 10 `npc.vendor.<type>` IDs.
- **Doc 12**: §8 / line "doc 13 cavalry spawns dismounted" with `room.no_mount` must go (C-30; cavalry no longer exists). The `gpv.evt.plea` captive is "a bound `fac.neutral` captive (doc 13 hireling template)"; doc 13 §13.1 makes captives `npc.captive.<genre_suffix>` of the genre's trader faction and §12.3's `fac.rep.free_captive` pays that faction, so doc 12 should read "a bound `npc.captive.<genre_suffix>` of the zone genre's trader faction (doc 13 §13.1)".
- **Doc 14**: §3.8 Erasure clock must follow canon §17.1 and §11.6: stack `k` at `c = 43200 + 1800 × (k - 1)` (tenth stack 59,400, 990 s), one Erasure hunter per Folio with no 5400-tick repeat; its timeline table (rows 420 s to 870 s) and example need the same change. The meta counter for `ach.hunter` keys on the hunter spawned by §3.8, which is still `rank.hunter`.
- **Doc 18**: art notes use the old cavalry names: `war_rig_rider` motorbike → `ram_raider` with a welded car-door ram shield (on foot); `posse_rider` → `posse_deputy` (lasso); `wild_hunt_rider` → `wild_huntsman` (antlers). Also rename skins for `hover_lancer` → `jet_lancer`, `getaway_driver` → `getaway_gunman`, `cavalry_general` → `jade_general` if any skin text names them.
- **Doc 05**: `enemy.ab.lance_charge` is now an on-foot charge (doc 13 §3); doc 05's NPC action state machine runs it like any attack profile; nothing else is needed once doc 05 drops its §17 mount sections (C-30).

## Verification

As of 2026-10-11. Adversarial verifier for chapter 13. Every check below was made against the files on disk after the fixer's batches; edits made by the verifier are listed at the end.

### Check 1: triage rows naming chapter 13

| Row | Verdict | Chapter text that satisfies it |
|---|---|---|
| G-007 | applied | §4.3: "`bearing = yaw_from_dir(dx, dz) = atan2_bam(-dx, -dz)` (Canon §1.9)" |
| G-021 | applied | §4.1: "On keyed `rng.ai` this chapter uses seq 0 for the action pick and seq 1 for position and target picks, and no other seq." |
| G-023 (gap 19) | applied | Imports, Canon §3 row: "never draws Canon §3.3's `rng.check` (doc 04): its dialogue and taming are deterministic"; test 52. |
| G-036 | applied | §1 rule 6: "blocks tagged `block.tag.destructible`" |
| G-047 (gap 8) | applied | §4.1: "`sys.ai_think` (system 6, owner 13) runs the four AI stages below" |
| G-051 (gap 1) | applied | Header: "Conforms to canon-1.11." Mounts (C-30) and durability (C-31) checked under checks 2 and 4. |
| G-064 | consistent | Turrets in §9.3 (`ice_daemon` P2) are `electronic` constructs; no doc 13 rule contradicts doc 12's TEC ≤ 2 list. |
| G-070 (gap 7) | applied | §11.2: "a member of a party of `n` gains `floor_div(party_hp_bp × enemy_count_bp[n], 10000 × n)` Bp of the solo combat skill XP: 10000 at 2 players, 10500 at 3 and 11156 at 4". Recomputed with an independent script: co-op row 28 / 28 / 28, skill 10 / 24, 10 / 23, 9 / 22, as stated. |
| G-071 (gap 6) | applied after fix | §11.2: "on Canon §8's 45-unit anchor". **Miss fixed:** the Imports row for Canon §8 still read "the 40-unit target". |
| G-072 | applied | §11.1: "downed members and ghosts share it; Dormant and Departed slots do not" |
| G-073 (gap 2) | applied | §11.1: "a pacification under Canon §8 and doc 04 §6.1, which emits `ev.entity_pacified` instead of `ev.entity_killed`" |
| G-074 (gap 3), G-075 (gap 4) | applied | §11.1 attribution (killing blow or 2500 Bp); §2.4 `summoned` "grants 0 kill XP" |
| G-106 | applied | §5.2: "`NPC_SPELL_SCALE_BP = 4500`" |
| G-109 | applied | §7.3: "for exactly 180 ticks, unscaled by Canon §11.4" |
| G-112, Q12 | applied | §10.5 Rank bullet: "carries `rank.hunter` for identity, the Canon §11.4 and §17.3 immunities, post-proc protection and presentation, but uses `rank.elite`'s multipliers" |
| G-113 (gap 16) | applied | §10.6: "Every `fac.editors` member, hunters included, erases blocks placed by Errata, except the player-controlled invader (Canon §11.6, §17.9)." |
| G-117 | applied | §4.6: "NPCs have no composure, so `st.dread` acts only through morale" |
| G-125 | applied | §10.1: "skipping any cell whose `light_block` exceeds the zone genre's `spawn_block_max` (Canon §5.9, §12.3)" |
| G-133 | applied | §2.2 `hp_max_mhp` chain ends with `diff_hp_bp(r, f)` |
| G-156 (gap 9), G-157 (gap 21), G-158 (gap 10) | applied | §12.1 "unwritten: none (Canon §5.2)"; "The xeno hive of Space Opera is `fac.wild.space_opera`"; §2.1 `block_stability_bp` |
| G-161 | applied | §7.3: "every shop carry a dependency-none source of each counter type, and `genre.unwritten`, which has no shops, also gets doc 11's counter caches" |
| G-170 (gap 5) | applied | §7.2: "Every champion carries one champion aspect" |
| G-172 (gap 11), C-30 | applied | §3 charger row; §5.6 "Mounts are cut (Canon §17.7, DECISIONS.md item 55): no enemy rides, spawns or drops a mount, and the former cavalry fight on foot as `enemy.role.charger` (§3)." |
| G-176 (gap 20) | applied | §10.7: "doc 14 owns the account unlocks of the other three through its `unlock.` nodes" |
| G-177 (gap 16), C-16 | applied | §9.2: "**2. The Censor** (`boss.editor_v2`, Folio 10)"; Interface 03 "the Censor's `blackout`" |
| G-181 (gap 14), G-182 (gap 12) | applied | §9.3 "paid from the Folio's population budget"; §9.2 "Inside the zone Errata Contradiction applies again (Canon §5.5 overrides only `genre.ur_text`)" |
| G-183 (gaps 15, 17, 22, 23) | applied | §4.8 "× 2500 if c has st.downed"; §10.6 `grid.cause.erased`; §10 rest profile "0: no initial population"; §2.4 `flier` row. Doc 03, 09 and 16 confirmed on disk. |
| G-184 (gaps 2, 24) | applied | No `ev.` request remains; lint finds no undefined ID. |
| G-191 (gap 13), G-195 (gap 18), G-196, G-226 | applied | `npc.quest.*` (§13.3); `npc.companion.<genre_suffix>` and `npc.companion.<name>` (§13.5); the six `aware.` states (§4.3); Editor squads in §10.2 |
| C-21 | consistent | §11.2 still supplies 37.3 kill equivalents and 45.5 units (recomputed: 37.324, 45.46). |
| C-22 | applied | §7.2 `elite.miasmic`: "`emit_gas_volume(cell, gas.miasma, 1)` (doc 08 §2.7)" |
| C-23 | applied | §8.7 `sand_wyrm`: "`block.sand` and `block.soil` cells"; Interface 07 "`block.sand`, `block.soil`"; no `block.dirt`. |
| C-24 | applied | §10.6: "A block is Errata-placed while doc 03's `is_errata_placed(c)` holds"; pseudocode `erase_placed(b, m)`; doc 03 §13.8 defines both. No `placed_by_errata` remains. |
| §4.1 stale gaps | applied | Gaps 15, 17, 22, 23 and the "Redactor's archive" request are gone; doc 12 no longer contains "Redactor". |

### Check 2: DECISIONS rows

- Changed rows: 15 invasions (§10.7) agree; 16 limited building (§1 rule 6, §10.6) agree, no building plot; 26 softer Erasure (§10.5: onset 43,200 ticks = 720 s, one hunter, no 5400-tick repeat) agree; 28 Echo cast, 33 glyph learning, 42 price, 46 ending, 56 World Laws, 60 pact curses: nothing in doc 13 contradicts them; 37 micro-voxels: doc 13 navigates and spawns on the 1 m grid, as Canon §4.4 keeps it; 38 text-only narration: barks are text keys, no voiced line; 55 mounts and 89 charger agree; 65 durability: no wear, break or repair rule (the dock heavy's crate is an NPC ability, not an item); 69 roster: **miss fixed**, §8 still said "at least 6 enemies"; 84 Errata limbs: doc 13 states no Errata severing rule and imports Canon §9.
- Rows 77 to 88: 77 (walkable hub, §13.6 3-station schedule), 81 (§11.1 ghosts), 85 (§11.2 co-op), 86 (§10.7 leaves invader rewards to doc 15) agree; 78, 79, 80, 82, 83, 84, 87, 88 do not land in doc 13 and nothing contradicts them.
- Rows 89 to 108: 89 and 91 agree (91's music cue is covered by "presentation"); 90 (Crowns) has no doc 13 text. **Miss recorded:** rows 100, 101, 103, 105 and 106 (Second Edition) name doc 13 and the chapter was silent; canon has no Second Edition section yet, so this is now open Canon gap 3.

### Check 3: chapter brief

- C-23, C-24: applied (check 1).
- Cavalry to charger: `enemy.role.charger` (§3), `ai.set.charger` (§4.7); `enemy.role.cavalry`, `ai.set.cavalry`, `ai.nav.mounted`, `ai.action.remount`, `npc.vendor.stablemaster`, `shop.stablemaster` appear nowhere in the chapter; §5.6 is one sentence; the 8 entries are chargers (`kingsguard_lancer`, `jet_lancer`, `ram_raider`, `posse_deputy`, `getaway_gunman`, `jade_general`, `wild_huntsman`, `archetype_knight`), none mounted. Lance charge recomputed: 15,000 mHP; 16,000 mm at 10,000 mm/s = 96 ticks; jet lancer 12,000 mm at 14,000 mm/s ≤ 52 ticks.
- Erasure: one elite-strength hunter, `rank.hunter` identity and immunities, `rank.elite` multipliers, no repeat. Doc 14 §3.8 on disk now matches (stack 1 at 43,200, no repeat), so the fixer's open gap "Doc 14's Erasure clock" was stale: **removed**, and the Interface 14 row that cited it was rewritten.
- Stale gaps deleted; "the Censor" used; namespaces owned per Canon §19 (`enemy.`, `ai.`, `fac.`, `boss.`, `npc.`, `director.`, `rank.`, `elite.`); `rng.check` cited as Canon §3.3's stream.

### Check 4: leftover grep

Hits judged: "mount"/"cavalry" lines 18, 20, 32, 185, 228, 698 to 700, 1770, 1814 are retirement records (kept). "Redactor" lines 20, 1106, 1310 name invaders or explain the Censor's name (kept). "breaks"/"broken" hits are blocks, doors, morale and an NPC crate shield (kept). "5400" was only in the stale gap (removed). **Miss fixed:** §13.6 `npc.margin.archivist` "bestiary and item codex" (G-197, C-29) now reads "the Bestiary and the Index". No hit for durability, repair, wear, saddle, Delving, voice, VO, 24.99, Crowns, `margin.plot` or building plot.

### Check 5: lint

`python3 tools/spec_lint.py --chapter 13`: no findings before or after the verifier's edits. `--json`: `multiply_defined` is empty (doc 02 now keys its curves as `"curve"`), chapter 13 has no undefined IDs, foreign definitions, retired IDs, JSON issues or hedges.

### Check 6: internal consistency

- Recomputed in Python from Canon §13.8's tables: §2.2 examples 1 and 2, the charger hit, the boss HP table (all 7 rows, 4 party sizes, B column), the hunter (558,537; 1,424,269; 3,397,863; 45,383; 317 XP), the Colophon hunter (2,234,148), the invader (10,698,668; 25,904; 33,675), kill XP (529, 302, 970, 881), the tamed troll, weapon pickup, reflection caps, breach, curves, the population plans, the Erasure stack ticks and 4 stacks at `c = 50,000`. All match.
- §11.2: the solo reference table reproduces row for row (plan, enemies, kill XP, skill-ups, gain, cumulative, level, main skill) from doc 04 §6.3's starts; sensitivity rows reproduce except "density 11000 everywhere", which gives 9 / 20 when Editor Folios are dense too and the stated 9 / 21 when only standard Folios are: **wording fixed** to "in every standard Folio".
- Tables: every table row has its header's cell count. Exports: all 131 `enemy.ab.*` IDs are used and every used one is exported; 64 boss attacks, 33 actions, 25 inputs, 20 sets, 14 curves, 6 nav classes and 14 `fac.rep.*` match their sections.
- Test 33 said the trample "knocks it aside at 4000 mm/s"; doc 03 §5.13 scales that speed by mass, so it now names the call `apply_knockback(target, dir, 4000)`.
- §2.2's "(Canon §11.6, §10.5)" read as Canon §10.5; now "(§10.5, Canon §11.6)".
- Canon gaps hold only open items: roster size (with the open mini-boss selection rule), the unconfirmed defaults, Second Edition.

### Requests in the fix log, re-checked

Doc 02 (curve keys), doc 11 (stablemaster), doc 12 (`room.no_mount`, plea captive), doc 14 (Erasure clock), doc 18 (charger names) and doc 05 (mount sections) are all done on disk; none is still outstanding.

### Verifier edits

1. Imports, Canon §8 row: "the 40-unit target" → the 45-unit anchor and the kill supply near Canon §17.4's 40.
2. Imports, decision row: adds items 81, 89 and 91.
3. §2.2: reference order "(§10.5, Canon §11.6)".
4. §8 intro: "at least 6 enemies" → the 6 to 8 specified entries, DECISIONS 69's minimum of 12 and the content-record route (doc 21 §5.1 step 3).
5. §9.3: the second mini-boss per genre is pending (Canon gaps item 1).
6. §11.2: density sensitivity row reads "in every standard Folio".
7. §13.6: archivist serves the Bestiary and the Index.
8. Interface 14 row: cites doc 14 §3.8's schedule instead of the deleted gap.
9. Canon gaps: removed the stale doc 14 item; item 1 adds the content route and the open mini-boss selection rule; new item 3 for Second Edition.
10. Test 33: names `apply_knockback(target, dir, 4000)`.

## Roster extension

As of 2026-10-11. Enemy roster extender for chapter 13, applying DECISIONS.md item 69 (12 or more enemy types per genre, 2 or more mini-bosses per genre, 5 Editors) on top of the fixer's and verifier's passes, whose changes are kept. DECISIONS.md item 124 (product owner) keeps champion substitution for `genre.unwritten` and `genre.ur_text`, so those two genres get no bespoke mini-boss; docs 18 (§6.5, 28 mini-bosses) and 21 (192 templates, 28 mini-bosses, 12 Editor-squad templates) already budget exactly this shape.

Inputs read: canon §5.2 (axioms, aether axioms), §5.3 (default `dep`), §5.9 (enemy themes, manufacturers), §6.5 (NPC rows), §9 (body plans), §10.1 (damage types), §11.1 and §11.8 (statuses, player-bearer rules), §13.8 (tables), §15 (factions), §16.3 (kin), §17.3 and §17.7; chapter 13 §2.1, §2.4, §3 (with `enemy.role.charger`), §4.7, §5.5, §7, §8, §9, §10.2, §12.1, §13.4; doc 08 §4.2 (gas catalog) and its liquid table; doc 07's block list; doc 09 §4.1 profiles.

### Per-genre counts

| Genre (§8) | Templates before | Templates after | Roles before | Roles after | Mini-bosses before | Mini-bosses after |
|---|---|---|---|---|---|---|
| §8.1 Mythic Fantasy | 7 | 12 | 7 | 12 | 1 | 2 |
| §8.2 Gothic Horror | 7 | 12 | 7 | 12 | 1 | 2 |
| §8.3 Cosmic Horror | 7 | 12 | 7 | 12 | 1 | 2 |
| §8.4 Space Opera | 7 | 12 | 7 | 12 | 1 | 2 |
| §8.5 Cyberpunk | 7 | 12 | 7 | 12 | 1 | 2 |
| §8.6 Post-Apocalypse | 7 | 12 | 7 | 12 | 1 | 2 |
| §8.7 Weird West | 7 | 12 | 7 | 12 | 1 | 2 |
| §8.8 Hardboiled Noir | 7 | 12 | 7 | 12 | 1 | 2 |
| §8.9 Wuxia | 7 | 12 | 7 | 12 | 1 | 2 |
| §8.10 Steampunk | 7 | 12 | 7 | 12 | 1 | 2 |
| §8.11 Fairy Tale | 8 | 12 | 7 | 11 | 1 | 2 |
| §8.12 Mythic Antiquity | 7 | 12 | 7 | 12 | 1 | 2 |
| §8.13 Oneiric (secret) | 6 | 12 | 6 | 12 | 1 | 2 |
| §8.14 The Redaction (secret) | 6 | 12 | 6 | 12 | 1 | 2 |
| §8.15 The Unwritten, Editor squads (secret) | 6 | 12 | 6 | 12 | 0 | 0 (item 124) |
| §8.16 The Ur-Text (secret) | 7 | 12 | 7 | 12 | 0 | 0 (item 124) |
| Total | 110 | 192 | — | — | 14 | 28 |

The 5 Editors stay 5; elites and champions are unchanged (Director promotions of standard templates, §7).

### What was added

- **82 roster templates** in the existing column format (Name, Role, Body, Kin and tags, Faction, Signature, Dep, Loot, W; §8.15 has no Dep column because every Editor attack has empty `dep`). The role supplies each row's stat line, behavior set, role resistances and role abilities (§3, §4.7); every new row has a role new to its genre or a distinct signature, and every signature is new, so no two rows share a stat line and signature. New native members only join existing §12.1 factions; `fac.wild.hardboiled_noir`, `fac.wild.redacted` and `fac.native.redacted.whistleblowers` receive their first entries (`alley_hound`, `spatial_anomaly`, `leaker`).
- **82 `enemy.ab.*` signatures**, each with exact integers (multipliers, windups, cooldowns, ranges, radii, status powers, durations): the catalog goes from 131 to 213 IDs. No new attack reaches × 20000, so §1 rule 3 holds without new telegraph floors.
- **14 mini-bosses** (`boss.mythic_fantasy.orc_warchief`, `boss.gothic_horror.moor_beast`, `boss.cosmic_horror.high_hierophant`, `boss.space_opera.hive_queen`, `boss.cyberpunk.chrome_king`, `boss.post_apocalypse.reactor_saint`, `boss.weird_west.sand_wyrm_matriarch`, `boss.hardboiled_noir.the_cleaner`, `boss.wuxia.nine_tailed_fox`, `boss.steampunk.sky_captain`, `boss.fairy_tale.cloud_giant`, `boss.mythic_antiquity.lernaean_hydra`, `boss.oneiric.the_examiner`, `boss.redacted.the_anomaly`), each with body, arena dressing inside doc 12's 46 × 46 m `room.arena` interior, phase-1 and P2 attacks with windups, multipliers and statuses, deterministic summon cells, and a `dep` in the rewritten §9.3 closing paragraph. Every attack at ×2.0 or more has a windup of at least 30 ticks or a marker impact at tick 90. Boss attack IDs go from 64 to 119; `boss.*` from 19 to 33.
- **Selection rule** (the open question of the former Canon gaps item 1): §10.2 step 4 draws one `weighted` call on `rng.spawn` over the arena zone genre's two mini-bosses in ascending ID byte order, weight 1000 each.

### Owners of everything referenced

Roles §3 and behavior sets §4.7 (doc 13); tags §2.4 (doc 13); damage types Canon §10.1; statuses Canon §11.1 with Canon §11.8 for Errata bearers (`st.charmed`, `st.sleeping`, `st.feared`); body plans Canon §9; kin Canon §16.3 (kin only on `body.humanoid`, `kin.mutant` always with a mutation); factions §12.1; gases doc 08 §4.2 (`gas.smoke`, `gas.miasma`, `gas.soporific`, `gas.fallout`); liquids doc 08 (`liq.fuel`, `liq.venom`, `liq.red_ink`); blocks and materials doc 07 (`block.web`, `block.sand`, `mat.paper`, `mat.vellum`); `apply_knockback` doc 03 §5.13; `aware.profile.hound` doc 09 §4.1. No mount, rider or vehicle and no durability, wear or repair appears; no `dep` names an axis that is 0 in its genre; no `fac.wild.*` entry is `sapient`.

### Other chapter-13 edits made with the extension

1. Imports: decision row adds item 124 and states the roster size.
2. §2.4: `alley_hound` joins the scent carriers and the `aware.profile.hound` hounds.
3. §3: `ai.nav.large` adds `fire_drake`, `hive_broodmother`, `nursery_bear`; `ai.nav.flier` for the floating controllers `will_o_wisp` and `siren`.
4. §8 intro states 12 templates per roster and 192 in all; Roster IDs, §8.15 and §10.2 say twelve Editor squads.
5. No core genre gained a second soldier-role entry, so §13.4's base-entry rule still names one entry per genre; Cosmic Horror's new soldier `drowned_thrall` is mindless, and its §13.4 row now says so instead of "(no soldier)".
6. §9.1 multi-shot list adds `smartgun_spray`, `foxfire_orbs`, `chalk_volley`.
7. §9.2 completeness: Proofreader `margin_note` cells (no draw) and the MYT verse's soldier source and faction; Censor `classified` as one `range(10)` call; Abridger segment numbering, `abridge` target segment (no draw) and `summary_judgment` as one `range(n)` call; a new paragraph for the Last Word's quadrant arena (dimensions, fall rule, how `bar_sweep`, `abridge` and `summary_judgment` adapt with their draw counts, unused attacks) and its fixed ARC verses.
8. §10.2 Mythic Fantasy faction-draw example recomputed: `[1500, 2500, 5000, 3500]`, `range(12500)`; test 45 matches.
9. Interface rows 03, 07, 08 name the new knockback users, light overrides, blocks, materials, gases and pours.
10. Canon gaps: item 1 (roster size) closed; the open items keep their numbers 2 and 3 because doc 01 cites "doc 13 Canon gaps item 3".
11. Acceptance tests 56 (roster size and rules), 57 (mini-boss selection), 58 (Editor picks), 59 (roster mechanics: `labyrinth_gore` 18,000 mHP, `long_dash` 120 ticks, `crank_gatling`, `form_in_triplicate` cap); invariants for roster size, mini-boss count and selection, kin and wild sapience.

### Verification

- `python3 tools/spec_lint.py --chapter 13`: no findings (0 undefined, 0 retired, 0 foreign definitions, 0 JSON issues, 0 hedges); `--json` `multiply_defined` is empty.
- A structural check over the edited chapter: every table row has its header's cell count; each §8 roster has 12 rows, at least 6 roles (11 for Fairy Tale, 12 elsewhere) and a swarmer; every native faction with an entry has a `sapient` one; roles, bodies, kin, tags, loot roles, factions and `dep` axes are valid; every signature is exported and every exported `enemy.ab.*` is used; §9.3 has 28 rows, 2 per genre in 14 genres; Exports counts equal their lists.

### Requests for other chapters

- **Doc 18**: art notes, silhouettes, `visual` IDs and skins for the 82 new templates and 14 new mini-bosses; §7's "Editor and Revisor names" paragraph lists six squad templates and must list twelve (`caret`, `em_dash`, `stet`, `bracket`, `deletion_mark`, `blue_pencil` added); the Interface request "the second mini-boss of each genre (DECISIONS 69, doc 13 Canon gaps item 1)" is satisfied by doc 13 §9.3 and can be dropped.
- **Doc 21**: §5.1 step 3 ("records that no chapter lists, such as the roster templates that raise each genre to 12 and the second mini-boss") and gap G11 are now covered by doc 13 §8 and §9.3; the content table's current-count column ("7 (Fairy Tale 8)") is now 12.
- **Doc 12**: the 14 new mini-boss arena dressings (all within the 46 × 46 m interior) join §21.2's prefab list.
