# Fix log: chapter 03 (Simulation Core)

Fixer pass to canon-1.11. File: `spec/03_SIMULATION_CORE.md`. Authorities: DECISIONS.md, canon-1.11, `audit/canon_gaps_triage.md`.

Plan (batches, in order):

1. Header, intro, Imports, Exports (no `block.` declarations), C-16, C-24, C-25, Canon §11.6 invader exception.
2. Tick pipeline (Canon §2.2, §2.4, §2.5, §10.8), fall chain (G-034), DDA `max_steps` (G-004), projectiles and terrain (G-037, C-08), respirator and gas contact (G-119), light reads and overrides (G-020, C-16, doc 06 shadow areas), `env` on DoT pulses (C-32).
3. Micro-voxels (Canon §4.4, DECISIONS 37; doc 07 and doc 09 interface rows).
4. Registry fields for doc 07, `doc07::block_hp`, sample records, events, Interface assumptions, Canon gaps, Acceptance tests, Invariants.

## Done

(appended after each batch)

### Batch 1 (done)

- Header now "Conforms to canon-1.11"; intro lists the canon-1.11 rules checked (G-051).
- Imports: Canon §2 (state boundaries, light overrides on every read, intents, hash boundaries; G-015, G-016, G-020), §3.3 explosion exception (G-024), §4–§4.4 (destructible tag, projectiles vs terrain, no durability, micro-voxels; G-033 to G-037, DECISIONS 37, 65), §10.8 two batches (G-094), §11.6 invader exception (G-113), §12.1 gas contact 500 and respirator (G-119), §13.8 `P(d_current)` block HP (G-132), §18 registration rule (G-184); doc 02 `dda` `max_steps` (G-004); doc 06 `in_shadow_area`; doc 07 block records, `block_hp(cell)`, `voxels_left(cell)`; doc 08 respirator; doc 13 "the Censor's `blackout`" (C-16).
- Exports: removed the `block.` row (`block.air` is doc 07's record; this chapter references it). `rng.` row cites Canon §3.2, §3.3.
- §13.3: dropped `item.tool.torch`; a wall torch is `block.torch` via `recipe.hand.wall_torch` (C-25).
- §13.8: `is_errata_placed(c)` over `placed_meta` is the only registry, no per-block bit (C-24, G-050); erasure by every `fac.editors` member except the invader (Canon §11.6, G-113); red ink erases with eraser 0 (doc 08 interface row).
- §10.6: "the Censor's `blackout`" (C-16).

### Batch 2 (done)

- §2.1 boundary cites Canon §2.2; §2.2 row 1 resets only the docs 04 and 10 action counters below `0x2000` (Canon §3.3, G-021); row 4 applies doc 16's neutral substitute (Canon §11.8, G-114) and logged intents in `(slot, action_seq)` (Canon §2.4, G-016); rows 17 and 26 are Canon §10.8's two batches (G-094): falls, environmental explosions and falling blocks now go to `sys.damage_late`; hashing at `b % 60 == 0`, `b >= 60` and snapshots of boundary `t + 1` (G-015); stale "Canon gaps G18" pointer removed (G-048).
- §2.4 rule 6 cites Canon §3.3's explosion exception (G-024).
- §3.3 `StatusInst.env`; §6.10, §8.4, §9.4 mark grid-contact statuses as `env` applications; §14.2 `DmgFlags.env` definition (C-32, G-185).
- §5.15 fall chain: `fall_dmg_bp`, then doc 04 factors in ascending source ID order (Canon §4.2, G-034).
- §5.18 item 4: `dda` gets `max_steps = 3 × ceil_div(t_max.raw, 2^32) + 3` (actions_02 item 4, G-004).
- §5.19: spawn-tick step against poses of `t`, one step per tick from `t + 1`, doc 16 follows (C-08); `Penetrate` only for destructible or non-opaque cells (Canon §4.3, G-037).
- §5.20, §6.10: gas contact 500 and the working respirator (Canon §12.1, G-119); breath drain respects the respirator (Canon §12).
- §10.1: light reads per Canon §2.3 (composure `light_at`, stealth `stealth_light`, spawn clamped `light_block`; G-020).
- §10.6: overrides gain an optional sphere (the `censor` kit's 12000 mm blackout, doc 13 §10.7) and doc 06's shadow-area dimming by 8 (doc 06 interface request); cap rule defined.
- Cross-chapter request queued: doc 09 §2.1 drops its own `dark` term, because `light_at` now includes the shadow-area dimming.

### Batch 3 (done): micro-voxels (Canon §4.4, DECISIONS 37; doc 07, 09, 12, 18 interface rows)

- §4.4 rewritten as "Shapes, props and micro-voxel masks": a predefined mask per shape and props (`R(X, Z, Y)` notation, facing rotation `(mx, mz) ↦ (mz, 3 − mx)`), a deterministic box decomposition `boxes(m)` used by every collision, ray, mantle, placement and explosion-center test, the fill rule (`fill >= 48` blocks light and sight; below, `light_filter`, `conceal`, `sound_atten_ddb` × fill / 64; empty-mask shapes keep registry values), `gas_open(c)`, `sturdy_at(c)`. Closed door is now 250 mm thick (one micro-voxel row). Layer rows `ceil_div(level + 1, 2)` (doc 07 spikes at level 1 = 16 micro-voxels). Fence and pane fill only their cell; containment uses two-cell fences.
- §4.1 grid row for masks; §4.2 mask bullet; §4.3 `Chunk.masks` and `mask_of` with canonical `(Local, mask)` serialization; §4.5 compiled tables and per-cell reads, `collides` field removed, `light_filter`, `conceal`, `sound_atten_ddb`, `sturdy_faces` describe a full cell; queries add `mask_at`, `fill`, `boxes_at` and the per-cell reads.
- §3.4 thinnest box 250 mm; `FallingBlock.mask`; §13.6 landing via `set_block_mask`.
- §5.4 gather uses `boxes_at(c)`; §5.6 step-up note; §5.9 mantle heights on 250 mm boundaries (750/1000/1250/1500 mm = 16/18/21/23 ticks; 1750 fails).
- §5.18 full-cell test by mask; §6.2 permeable capacity `floor_div(1000 × (64 − fill(c)), 64)`; §7.1 `gas_open(c)`; §7.6 `conceal_at(c)`; §10.2 sky source and filter via `light_filter_at`; §10.3 mask changes call `on_change`; §11.2–§11.3 `sound_atten_at`, corner cutting at `fill >= 48`; closed oak door example 243 ddB → 105 dB.
- §13.1 `set_block_mask` and `bad_mask`; every `set_block` removes the custom mask.
- §13.2 `damage_block(c, ip, …)`; breaks write `break_result`; non-placed block HP is doc 07's `block_hp(c)` (lock factor × `P(d_current)`, Canon §13.8, G-132); new chipping rule calling `doc07::voxels_left(c)` with removal nearest the impact point, `refill_mask` for doc 07 repair, `ev.block_chipped`.
- §12.2 resource nodes never marked by explosions (doc 07 §10.1); marked cells break into `break_result`.
- §16.1–§16.2 serialize custom masks; restore rebuilds the table and box caches.

### Batch 4a (done)

- §4.5 new fields `break_result`, `hide`, `conceal_crouch_only`, `lightsrc` (doc 07 interface row); `block_hp_mhp` defers to doc 07's `block_hp(cell)`.
- §4.5 sample block records with JSON `"id"` values removed (doc 07 owns `block.`; they were counted as definitions); only the `block.powder_keg` `explosive` spec that doc 07 §10.1 adopts stays, as a bare `ExplosionSpec` object.
- §14.2 new bus event `ev.block_chipped` (03) for replication and debris; §14.3 intro cites Canon §18's registration rule (G-184) and §8's pacification rule (G-073), stale G21/G28 pointers removed; doc 09 requests applied: `ev.alarm_raised.kind` 0 = creature, `ev.device_hacked.device: u64`.
- Exports: `ev.block_chipped`; non-ID exports add the mask functions.

### Batch 4b (done)

- Interface assumptions: doc 02 (`dda` `max_steps`, custom masks in section 4 and `ChunkView`, overrides and shadow areas in the view frame), 04 (fall factor order, Canon §4.2), 05 (`ip` on `damage_block`, two batches, `env` pulses, `Penetrate` rule), 06 (`in_shadow_area`, area `ip`), 07 (`block_hp`, `voxels_left`, `refill_mask`, `break_result`, record fields, powder keg spec, C-25), 08 (respirator), 09 (no second shadow term, micro-voxel sight, registered payloads), 12 (custom masks only on chipping shapes, two-cell fences), 13 (erasure without the invader, `is_errata_placed`, Censor box and kit sphere), 16 (C-08, mask replication; the retired-mount clause dropped), 18 (`ev.block_chipped`, masks).
- Canon gaps: G1–G5 and G7–G28 deleted as closed by canon-1.11 and the triage (G-020, G-024, G-033, G-034, G-036, G-040 to G-050, G-094, G-119, G-120, G-132, G-184, G-051; §4.1 stale G12). Kept G6 (ghost collision reading, never triaged). New open items: G29 closed doors and trapdoors leak light and sound under Canon §4.4's fill rule (requested canon wording); G30 empty-mask shapes keep registry concealment, per Canon §12.2 (requested wording); G31 DECISIONS 95 and 99 not yet in canon.
- Old in-text pointers to deleted gaps replaced (§5.15 doc 09 G16, §8.4 G16, §12 G19).
- Acceptance tests: 5 (door mask and box), 11 (mantle heights on 250 mm boundaries, two-cell fence), 25 (105 dB through a closed oak door), 28 (16 micro-voxels left), 31 (respirator), 44 (invader never erases), 48 (sphere override, shadow areas), 52 (retired kind code; six chapter events); new 53 masks, 54 chipping, 55 DDA bound, 56 hit batches and `env`, 57 projectiles vs terrain.
- Invariants: doc 07 `block_hp`; mask, fill, chip, batch, `env` and invader lines; the retired-mount line dropped (code 2 line kept).
- §10.6: propagation uses stored levels only; §5.20 reads through overrides and shadow areas.
- Lint after: `== 03 words=35920`, no findings; multiply-defined count 0.

## Cross-chapter requests

- 09 §2.1 `stealth_light`: delete `dark = 8 if … shadow area` and `env = max(0, env − dark)`; doc 03 §10.6 already lowers every light read, `light_at` included, by 8 inside doc 06 shadow areas. Also update the invariant `stealth_light = max(env − shadow, …)` to `max(env, …)`.
- 00 (canon §4.4): add "A closed door or closed trapdoor applies its block's full `light_filter` and sound attenuation." (doc 03 G29) and "Shapes with no micro-voxels (cross, ladder, fixture) keep their block's light filter, concealment and sound attenuation unscaled." (doc 03 G30).
- 00 (canon §4.2, §5, §12): carry DECISIONS 95 (mantle to 2,000 mm, sprint-slide, grappling tool item) and 99 (time of day and weather) so doc 03 can implement them (doc 03 G31).
- 02 §4.6: `dda` takes `max_steps` (actions_02 item 4); §6.5 section 4 adds each chunk's ascending `(Local, u64)` custom masks after the four bitsets; `ChunkView` carries the custom-mask map beside the block states; the view frame publishes doc 06 shadow areas beside the light overrides.
- 07 §10.8: chipping is in doc 03 §13.2 (removal by squared distance from the impact point, ties to the lower bit); repair calls doc 03's `refill_mask(c, n)`; layer level `L` has `ceil_div(L + 1, 2)` micro-voxel rows; nodes are excluded from explosion marking (doc 03 §12.2).
- 12: worldgen custom masks go through `set_block_mask` and only on full, pillar, slab, stairs or layer cells; a fence meant to contain is two cells tall (a fence cell is 1 m, doc 03 §4.4).
- 16: replicate custom masks (from `ev.block_chipped` or chunk deltas); §8 item 2 per triage C-08 (normal step against poses of `t`, then one step per tick from `t + 1`).
- 18: debris and rims from `ev.block_chipped` (`removed` bits, `ip`).
- 05: `damage_block` gains the impact point `ip` (doc 03 §13.2); fall, environmental-explosion and falling-block hits are resolved in `sys.damage_late`.

### Batch 5 (done): review pass

- §4.3 custom-mask table compaction when an append would pass 65,536 entries (u16 index bound).
- §4.5 field-type sentence allows the plain strings of `hide` and `lightsrc`.
- §13.1 `actor_forbidden` also covers cause `erased` for the invader and ghosts (Canon §11.6, §17.9).
- §2.2 row 26 wording: one ascending `(source EntityId, seq)` order inside the second batch (Canon §10.8).
- §4.4 and test 11: two-cell fences stop every jump from the ground; air jumps, wall jumps, levitation and flight can still clear them.
- §13.6 landing uses `set_block_mask` only for a custom mask, `set_block` otherwise.
- Redundant mount mentions removed from the intro and Imports; the §3.2 reserved-code row is the one retirement record.
- Final lint: `== 03 words=35972`, no findings; IDs defined in more than one chapter: 0.

Status: complete. Nothing left half-done in this chapter.

## Verification

Adversarial verifier pass over `spec/03_SIMULATION_CORE.md` after the fixer's batches 1 to 5. Verdicts: **OK** (applied exactly; the quoted text satisfies it), **FIXED** (a miss, now corrected in this pass), **OPEN** (kept as an open item with a reason).

### 1. Triage rows that name chapter 03

| Row | Verdict | Chapter text |
|---|---|---|
| G-004 | OK | §5.18 item 4: "It passes doc 02's `dda` the bound `max_steps = 3 × ceil_div(t_max.raw, 2^32) + 3`"; test 55 (15 and 18) recomputed. |
| G-015 | OK | §2.2: "Hashing of every boundary `b` with `b % 60 == 0` and `b >= 60` (Canon §2.5), the network snapshot of boundary `t + 1` after every even tick `t`". |
| G-016, G-114 | OK | §2.2 row 4: "using doc 16's neutral substitute for a missing command (Canon §11.8); apply the tick's logged intents in ascending `(slot, action_seq)` (Canon §2.4)". |
| G-020 | OK | §10.1: "Every read of a cell's light goes through §10.6 (Canon §2.3)"; §10.6: "Every read of `light_sky`, `light_block` or effective light at cell `c` returns `min(max(0, stored − dim(c)), m)`". |
| G-021 | OK | §2.2 row 1: "reset the per-tick action counters of docs 04 and 10, which stay inside their reserved `seq` ranges below `0x2000` (Canon §3.3)". |
| G-024 | OK | §2.4 rule 6: "`rng.physics` only in `sys.explosions` (Canon §3.3's explosion exception)"; §12.2 and §12.3 give the draw counts. |
| G-033 | OK | §6.1 "One liquid unit is 1 litre = 1000 `Ml`"; §7.1 "Concentration 10000 is a cell saturated with that gas, 1,000,000 ml". |
| G-034 | OK | §5.15: "queues a hit for `sys.damage_late` with `B = mul_bp(canon_damage, genre.fall_dmg_bp)` … doc 04 §8.1 then multiplies it by each of its fall-damage factors … in ascending source ID order". FIXED a stale "fall-damage hit for `sys.damage`" in the §5 preamble. |
| G-036, G-037 | OK | §4.5 loader rule 2 and §4.7; §5.19: "`on_block_enter` returns `Penetrate` only for a cell that is destructible or not opaque". |
| G-038 | OK | §5.9: "require: that box's cell has a shape other than grid.shape.fence and grid.shape.pane". The 1500 mm fence of the ruling is superseded by Canon §4.4's 1 m fence cell; §4.4 builds containing fences two cells tall. |
| G-039, G-040 | OK | §4.7 `flip` bits; §10.2 sky sources from `open_tag` or an open column, falling off 1 per step. |
| G-041 to G-045 | OK | Imports doc 12 GPV fields; §11.2 "Costs are in tenths of a decibel"; §15.2 "they hear nothing"; §5.21 "Only bodies and projectiles use portals"; §5.16 "The loader expands keyframes into the canon per-tick table". |
| G-046, §4.1 stale G12 | OK | G12 deleted; §16.1 row 7 "`palette_rle` of the 4,096 `DeciC` values as `u16` offsets `T + 2730`", fluids as sparse lists. |
| G-047, G-048, G-049 | OK | §2.2 "A label with a letter suffix (17a, 26a, 26b, 34a) is a full system"; "C1 runs before movement, and doc 05 agrees"; §12 "this chapter's algorithm governs (triage G-049)". |
| G-050, C-24 | OK | §13.8: "`is_errata_placed(c) = placed_meta.contains_key(c)` (doc 07 §1.4) is the only registry of Errata-placed cells … no per-block placed bit exists". |
| G-051, C-30, C-31 | OK | Header "Conforms to canon-1.11"; §3.2 keeps only the retired kind code 2; no item durability anywhere, block HP kept (§13.2). |
| G-094 | OK | §2.2 rows 17 and 26. FIXED: the combat-phase paragraph now says C6's rules resolve the second batch inside `sys.damage_late`. |
| G-113 | OK | §13.8: "each `fac.editors` member except the invader (Canon §17.9), hunters included". |
| G-119 | OK | §5.20: "a gas with concentration at least 500 in the eye cell … A body wearing a working respirator in `slot.face` … has no gas contact". |
| G-120 | OK | §6.9 (64 cells, 100-unit freeze, 27-cell steam cap); §8.4 "A web … burns out 48 to 60 ticks after ignition". |
| G-132 | OK | §13.2: "times Canon §13.8's item-power table at `P(d_current)`". |
| G-183 | OK | `grid.cause.erased` in §13.1. |
| G-184 | OK, FIXED | §14.3 intro: "every further `ev.` ID another document emits exists once it is registered here". A script found every `ev.` ID used in chapters 01 to 21 registered (only the retired `ev.mounted` and `ev.dismounted` appear, in the canon changelog). Missed: doc 12 §4.1 and its I1 request a law-discovery event `{ law, slots }` (DECISIONS 56); now registered as `ev.law_discovered` and exported. |
| G-185, C-32 | OK | §3.3 `StatusInst.env`: "every DoT pulse of an instance with `env` set carries `env` (triage C-32)"; §14.2 `DmgFlags.env`; §6.10, §8.4, §9.4. |
| G-218, C-08 | OK | §5.19: "the last of them its normal step against the poses of `t`, and one step per tick from `t + 1` … Doc 16 §8's catch-up follows this rule (triage C-08)"; crawl 800 mm/s in §5.1. FIXED: catch-up now applies to the slot-4 invader's projectiles too (doc 05 §2.6 rewinds every player-controlled attacker; doc 16 interface row). |
| C-16 | OK | §10.6 "The Censor's `blackout` (doc 13 §9.2) and the `censor` kit's (doc 13 §10.7)"; no "Redactor" left. Imports doc 13 row now lists §10.7. |
| C-25 | OK | §13.3: "A held torch never places a block: a wall torch is the placeable `block.torch` that doc 07's `recipe.hand.wall_torch` makes from it (triage C-25)"; `item.tool.torch` absent. |

### 2. DECISIONS rows

- 16 limited building, 37 micro-voxels, 55 mounts, 65 no durability: OK (Imports Canon §4–§4.4, §4.4 shapes and masks, §13.2 chipping, §3.2 code 2).
- 15 invasions: OK, FIXED. Canon §17.9 ("cannot place or break blocks") was enforced only through `set_block`; `damage_block` still accumulated invader damage and chipped cells (chipping bypasses `set_block`), invader blasts marked cells with cause `explosion`, and invader fire could burn blocks out. Now §13.2 adds 0 for the invader or a ghost, §12.1 zeroes an invader blast's block power and incendiary flag, §8.1 refuses an invader's block fire; test 44 and the invariant extended. Doc 07 rule 4 and its interface row already assumed this.
- 26 softer Erasure: OK (no repeat-spawn text; `5400` absent). 28 Echo cast: FIXED. §5.2 hard-coded crouch walk 2000 and §5.11 climb 2400, but doc 04 §13 replaces every base speed for `echo.fenrir` (`body.quadruped`, climb 0) and `echo.jormungandr` (`body.serpentine`). `MoverParams` gains `crouch_base` and `climb_base`, §3.4 names the Echo plans. 33 hybrid glyphs: `ev.glyph_learned` matches doc 06 §19.1. 56 World Laws: FIXED (`ev.law_discovered`, above).
- 38 text-only narration, 42 USD 19.99, 46 procedural ending, 60 pact curses, 69 enemy roster, 84 limb loss: no doc 03 surface; no conflicting text. "voice noises" in §11.1 are doc 09 vocalizing noises.
- 77 to 88: none lands in doc 03 except 87 (`content_hash` covers block records; `skins` are presentation only, §4.5) and 88 (§5.19 `Penetrate` rule): OK.
- 95 movement kit and 99 weather: OPEN as G31. Canon-1.11 is silent on mantle height, but "up to 2,000 mm" is ambiguous: §5.9 measures from the feet at the press, so 2,000 mm plus a jump reaches 3,100 mm and climbs 3-block walls against doc 12 §15.1's 2-block climb rule. G31 now states that question. Doc 17's interface row already assumes 2,000 mm; doc 03's row 17 now states the 1,600 mm reach until G31 closes.

### 3. Chapter brief

C-24, C-25, C-32, C-16, C-08, stale G12, DDA bound: OK as quoted above. `block.*` IDs: Exports declare none; the lint found no foreign-namespace definition. Events: see G-184. Limited building, fixed terrain, 250 mm micro-voxels, block HP without item durability: OK.

### 4. Leftover grep

Hits for mount (the §3.2 retirement row and "amount"), durability ("no item durability", Imports and intro), repair (doc 07's placeable-repair refill, which is block HP), wear ("wearing", "wearer"), broken (`grid.cause.broken`, `ev.ward_broken`), voice (doc 09 noises), rider (doc 06 register rider in `ev.favor_changed`). All legitimate. No hits for saddle, cavalry, Delving, voiced, VO, 24.99, Item Codex, Crowns, Redactor, margin.plot, building plot, 5400.

### 5. Lint

Before and after this pass: `== 03` with no findings; `--json` multiply_defined `{}`.

### 6. Internal consistency

- FIXED §13.2 `block_bp` list. It restated stale values (melee 5000 for every swing, bullets 2500, thrown 2500) against doc 05 Table 3.2d and §5.2 (melee by archetype 500 to 10000, axes 5000; bullets, pellets, arrows, bolts, beams 1000; thrown 2000). Examples and tests 28 and 54 now name an axe swing; the bullet line is 800 per bullet, the 50th breaks the cracked wall (recomputed). The claim "a Folio-25 weapon would break a wooden door in one swing" was false for an axe (47,466 < 60,000); it now uses a mace: one 94,932 mHP swing without the `P(d_current)` factor, 5 swings with it at Folio 1 and 25, and 10 for an axe (recomputed exactly: 474,660 = 5 × 94,932 = 10 × 47,466).
- FIXED §12.4 explosion magnitude. `B_eff = mul_bp(B, mul_bp(falloff, exposure))` broke Canon §10.4 (attack factors, area falloff and exposure included, one `mul_bp` each) and disagreed with doc 05 §9.4 and this chapter's own test 43. Now `mul_bp(mul_bp(B, falloff), exposure)`, with the combined `f` kept for knockback, impact and status powers. `Source::Item.b_mhp` is doc 05's pre-§10.4 base, and the item hit's §10.4 chain applies when it resolves. The worked example still gives 20,000 mHP and 5000 mm/s; test 27 unchanged. §14.2 now says an item blast's hit carries its item's counters (doc 05 §9.4).
- FIXED §5.8 lean, which kept its own point test against doc 05 §6.8 (owner per Canon §4.2: sphere clamp in 50 mm steps, `sign × floor_div` offset).
- FIXED stale pointers to closed gaps of other chapters: doc 16 G6 is now §8 item 4, doc 17 G22 is now §12.1. FIXED the `MoverSnap` field list (doc 16 §7.2 has `status_speed_bp`, `n_status`, not `speed_bp`) and the doc 16 interface row. FIXED §11.1's `convert_events()` list (doc 09 §7.9 also converts sonic `ev.damage_dealt`).
- ADDED doc 01's request: a block-trap hit carries `source` 0, `item_uid` = `placed_meta.uid` and `ability` = the trap block's ID (§14.2, doc 07 interface row). ADDED a `doc05::steer` call in `proj_step` for doc 05 §10.5's homing turn (doc 05 interface row); the turn itself stays with docs 05 and 11.
- Recomputed with Python: mantle ticks 16/18/21/23/24 for 750 to 1,600 mm; DDA bound 15 and 18; chipping removals 9, 10, 9, 10, 10 (16 left, 5 events); crack stage 7; door 105 dB; candidate radii 70,153 and 25,416 mm; gas example 3271; fire 3750 Bp; Erasure `Gidx` 136,211 and 136,213; JSON cell 133645; explosion ray marks (13, 2, 10) with 85 left. All match the text.
- Tables: every row has its header's column count. Exports: every `comp.`, `sys.`, `grid.` and `ev.` ID used is exported, no `block.` ID is. Canon gaps hold only G6, G29, G30 and G31, all open. Header: "Conforms to canon-1.11".

### Cross-chapter requests from this pass

- 05: define `steer(pr, v, pose_tick) -> Fx3` (§10.5): the target rule for `st.marked` homing, the per-tick turn (doc 11's 16384 Bam/s through a Canon §2.3 accumulator kept in `ProjCombat`) and magnitude preservation; the five-hook sentences of §7.1 can name it.
- 07: queue each block-trap hit (§10.4 spikes) with `source` 0, `item_uid` = the trap's `placed_meta.uid` and `ability` = its block's content ID (doc 03 §14.2).
- 12: name the law-discovery event `ev.law_discovered` in §4.1 and in I1, and drop the I1 request now that doc 03 registers it.
- 16: add `crouch_base` and `climb_base` to `OwnerDelta.mover` (§6.10, §7.2 text).
- 17: the mantle reaches 1,600 mm until doc 03 G31 closes; the interface row that assumes "up to 2,000 mm" should say so (as doc 01 G1 already does).
- 00: settle G31 with an explicit measuring point for the 2,000 mm mantle (from the feet at the press, or including the jump).

Verifier status: complete. Open items: G6, G29, G30, G31; the homing turn's math awaits doc 05.
