# Canon gap triage

As of 2026-10-11. Author: canon gap triager. Status: decisions final unless the product owner overrides one of the rulings listed in §1.

**Inputs.** `spec/00_CANON.md` at canon-1.10 (the working tree: canon-1.8 cut mounts, canon-1.9 added pact curses, canon-1.10 removed item durability), `DESIGN_BRIEF.md`, `DECISIONS.md` items 1 to 72, `audit/slice_spec_issues.md` with `audit/actions_02.md`, and the "Canon gaps" and "Interface assumptions" sections of chapters 01 to 18 as they stood on disk during the triage. Docs 07, 08, 10, 11, 13 and 14 were being edited at the same time; every conflict in §4 was re-checked against the files on disk after this file was written, and C-22 was found resolved.

**Skipped as already resolved by canon-1.5** (listed in the canon changelog): checked shifts, non-negating `ceil_div`, the coordinate range for `dist2`, negate-first `fwd` and its pitch type, Fx display rounding, length-prefixed multi-field hashing, debug-only draw counters, strict seed and date decoding, the depth cap on `S`, `mul_bp` with `i64` table multipliers, and the ±1 Bam `atan2_bam` contract.

**Decisions.** Each row has exactly one:

- **canon amendment**: the last column gives the exact text and the target section. Applying every amendment makes canon-1.11 (§5 gives the changelog entry and the new tests).
- **chapter owns it**: the canon stays as it is; the named chapter's rule stands, or the named chapter must change (pointer to §4).
- **already in canon**: the canon already answers the gap; the chapter's gap text can be deleted.
- **reject**: the request is refused, with the reason.

Units, rounding and namespaces in every amendment follow canon §1: Bp chains are one `mul_bp` per factor in the listed order, integers floor through `floor_div`, durations are ticks, and new IDs stay in their owner's namespace.

**Totals.** 227 triaged gaps (G-001 to G-227): 79 canon amendments, 122 chapter-owned, 24 already in canon, 2 rejected. 32 interface conflicts between chapters (C-01 to C-32), each with the chapter that must change; C-22 was fixed on disk while the triage ran.

---

## 1. Rulings the product owner may want to overrule

The task required a decision on every item, so each of these is decided and applied below. They are design calls rather than consistency fixes, so they are listed here as questions with the default already chosen.

| # | Question | Default applied | Row |
|---|---|---|---|
| Q1 | Should ghosts share kill XP while dead? Doc 04 says yes, doc 14 says no. | Yes: downed members and ghosts share kill XP; Dormant and Departed slots do not. A dead player does not fall a level behind. | G-072 |
| Q2 | Should Scribe's Mercy block achievements and Grade unlocks? Doc 14 blocks 16 achievements and every Grade unlock; doc 17 says Mercy never blocks achievements. | No, as in Hades' God Mode. Only ranked leaderboard entries exclude Mercy runs. | C-04 |
| Q3 | May party members hand each other Lucre? Canon says no currency can be traded or gifted; doc 11 lets players drop Lucre piles. | Yes, as dropped piles only, inside one run. The ban stays on everything between accounts. | G-148 |
| Q4 | Does `mfr.oneiric.somnambule` shoot through the level's fixed walls? | No. After 10 m its shots pass destructible and non-opaque blocks for free, but fixed terrain stops every projectile. | G-037 |
| Q5 | Errata limbs sever too easily (one Folio-1 boss slash, or two standard slashes through starter armor). | Doc 04's rule: an Erratum's limb severs only from a crippled pool (at most 25% left) or a single hit of at least 50% of the pool. A heavy hit can still sever a healthy arm. | G-080 |
| Q6 | In co-op, each member's weapon skill grows at 44% of the solo pace in a party of 4. | Normalize skill XP by the enemy's HP before party scaling. A member of a party of 4 then grows at about 112% of solo pace. | G-070 |
| Q7 | The level curve assumed 40 XP units per Folio, but the canon's own sources pay about 45. | Re-anchor on 45 units: level 29 on arrival at Folio 25, inside the 28 ± 3 target. | G-071 |
| Q8 | Should invading another party earn Codex pass XP, since the brief says all play does? | No. DECISIONS 32 keeps invader rewards cosmetic-only: Quires and Red Ink marks. | G-174 |
| Q9 | "Crowns", the Mythic Fantasy Lucre skin, is The Elder Scrolls Online's purchasable currency. | Rename it to "Nobles". | G-052 |
| Q10 | Field of view range: doc 02 says 70–110°, doc 17 and research R3 say 60–120°. | 60–120°, default 90°. | C-11 |
| Q11 | May co-op peers run different cosmetic-only mods or text fixes? | Yes. A `presentation_hash` split out of `content_hash` covers art, audio, VFX, narration, UI and localization. | G-017 |
| Q12 | The softer Erasure hunter has elite stats. Is it still a "hunter" for music, immunities and post-proc protection? | Yes. It keeps `rank.hunter` for identity and immunities and uses `rank.elite`'s multipliers. | G-112 |

---

## 2. Triage table

### 2.1 Issues from the Rust vertical slice that canon-1.5 did not resolve

| ID | Sources | Gap | Decision | Canon text (target) or ruling |
|---|---|---|---|---|
| G-001 | slice | `range_incl` computes `hi - lo + 1` in `i64`, which panics for spans above `i64::MAX` that its own assert allows, and `lo + r as i64` can overflow as well. | canon amendment | **§3.1**: replace "lo + self.range((hi - lo + 1) as u64) as i64" with "((lo as i128) + (self.range(((hi as i128) - (lo as i128) + 1) as u64) as i128)) as i64   // exact: the result lies in lo..=hi". **§21**: add test 45 (§5.2). |
| G-002 | slice | `closest_seg_seg` narrows with `.expect` and panics on near-parallel and degenerate segments inside the safe range. | chapter owns it | Doc 02 §4.6 adopts clamp-before-narrow in 256-bit arithmetic with both failing cases as worked examples (`audit/actions_02.md` item 1). |
| G-003 | slice | Doc 02 §3.3 limits the math crate to `blake3`, while §4.1 and §4.7 derive serde. | chapter owns it | §3.3 wins: an optional `serde` feature, off by default, for tools; `Rng::state()` and `Rng::from_state()` are the save interface (actions_02 item 3). Canon §2.5 already says canonical serialization never depends on serde. |
| G-004 | slice | The voxel DDA states no step bound. | chapter owns it | Doc 02 §4.6 takes `max_steps`, yields at most `max_steps + 1` blocks and stops at the `i32` edge; doc 03 callers pass `3 × ceil(t_max) + 3` (actions_02 item 4). |
| G-005 | slice | `ray_sphere` misses a tangent ray (`h <= 0`) while the `ray_capsule` body path hits it. | chapter owns it | Doc 02 §4.6: both treat `h == 0` as a hit (actions_02 item 5). |
| G-006 | slice | An inherent `I256::cmp` shadows `Ord::cmp`; the trig table path names a crate that does not exist. | chapter owns it | Doc 02 uses derived `Ord` and `crates/palimpsest_math/src/trig_table.rs` (actions_02 items 6 and 7). |

### 2.2 Conventions (canon §1)

| ID | Sources | Gap | Decision | Canon text (target) or ruling |
|---|---|---|---|---|
| G-007 | 02 G2 | The `atan2_bam` argument convention and the yaw that faces a direction are unstated. | canon amendment | **§1.9**: replace "`atan2_bam(y, x)` uses 16-iteration CORDIC on `i64` with a committed arctangent table." with "`atan2_bam(y, x)` returns the angle of the vector `(x, y)` counter-clockwise from +x, by 16-iteration CORDIC on `i64` with a committed arctangent table. The yaw that faces a horizontal direction `(dx, dz)` is `yaw_from_dir(dx, dz) = atan2_bam(-dx, -dz)`, the inverse of `fwd`." **§21**: add test 46 (§5.2). |
| G-008 | 02 G5; 11 (doc 02 row) | No rule namespaces mod content IDs. | canon amendment | **§1.1**: after the bullet that ends "with error `id32_collision`." add the bullet "Mod content IDs use `x_<mod_id>` as their second segment, for example `item.x_frostpack.ice_blade`. Base content never uses a second segment that starts with `x_`." |
| G-009 | 02 G12 | Canon §1.1 rejects any mod record that reuses a base ID, so mods cannot change base content. | chapter owns it | Doc 02 §13.1's append-only patches on schema-marked arrays are the only way a mod changes base content. This is consistent with §1.1. |
| G-010 | 02 G6 | Canon §1.6 does not bound JSON integers. | canon amendment | **§1.6**: replace "`u64` values such as seeds and hashes are written as `"0x"`-prefixed hexadecimal strings, because JSON tooling loses precision above 2^53." with "Every JSON integer in content lies in `-(2^53 - 1)..=2^53 - 1`. Larger values, and every `u64` such as a seed or a hash, are written as `"0x"`-prefixed hexadecimal strings, because JSON tooling loses precision above 2^53." |
| G-011 | 17 G16 | `Milli` display by `floor(x / 1000)` shows small per-second rates as 0. | canon amendment | **§1.5**: replace "except current HP, which displays as `ceil_div(x, 1000)` so a living entity never shows 0 HP." with "except current HP, which displays as `ceil_div(x, 1000)` so a living entity never shows 0 HP, and per-second rates below 10 display units, which display with one decimal place as `floor(x / 100)` tenths." |
| G-012 | 02 G9, G13 | Telemetry event kinds and perf scenario names have no namespace. | already in canon | §1.1 governs content IDs only. Enum variants and plain strings inside records are not content IDs. |
| G-013 | 02 G10 | Whether the entity store needs generations. | already in canon | §1.11 never reuses an ID within a Folio, so the store is non-generational. |
| G-014 | 02 G8 | Where the float conversion module lives. | already in canon | Test 33 allows a documented boundary module; doc 02 putting it in `palimpsest_bridge`, outside the sim crate, is stricter and compatible. |

### 2.3 Time, sessions, replays and saves (canon §2)

| ID | Sources | Gap | Decision | Canon text (target) or ruling |
|---|---|---|---|---|
| G-015 | 02 G3; 16 G5 | "Every 60 ticks" for `state_hash` and the snapshot row do not name the state boundary they read. | canon amendment | **§2.2**: after "Per-entity phases use the §1.11 phase key." add "Boundary `b` is the state after tick `b - 1` completes and before tick `b` runs. A row that runs on tick `t` reads boundary `t` and writes boundary `t + 1`, so the network snapshot row publishes boundary `t + 1` for every even `t`." **§2.5**: replace "The host broadcasts `state_hash` every 60 ticks for desync detection (doc 16)." with "The host hashes every boundary `b` with `b % 60 == 0` and `b >= 60`, and broadcasts that `state_hash` for desync detection (doc 16)." |
| G-016 | 02 G4; 16 G2; 17 G11, G22, G24; 11 G14 | The replay log holds only commands. Intents (shop, trade, crafting, menus, session intents, invader join and leave, breath hold, vocalizing, the cast-item target) appear nowhere in canon. | canon amendment | **§2.4**: after the bullet that begins "The replay log records the host's applied command" add the bullet "The replay log also records every applied intent `{ tick: u64, slot: u8, action_seq: u32, kind }`, where `kind` is doc 02's `IntentKind`: actions that are not button bits, such as shop, trade, crafting and menu actions, session intents and the host's slot-4 `InvaderJoin` and `InvaderLeave`. Intents apply on their logged tick, at the point doc 03's tick order sets, in ascending `(slot, action_seq)`. Doc 02 owns the list of kinds; adding a kind needs no canon change." |
| G-017 | 02 G1, G7; 18 gap 5 | `content_hash` covers presentation records, so a text fix or a cosmetic mod breaks co-op joins and daily verification; its byte layout is not referenced. | canon amendment | **§2.4**: replace "`content_hash` is BLAKE3 over the canonical serialization of all loaded content sorted by content ID, including mods and the run's unlock set." with "`content_hash` is BLAKE3 over the canonical serialization (doc 02 §11.2) of every loaded content record outside the presentation namespaces `art.`, `aud.`, `vfx.`, `nar.` and `ui.`, sorted by content ID, plus the simulation-affecting mods, the run's unlock set and the simulation-affecting flags. `presentation_hash` is built the same way over the presentation records, the localization tables and the presentation-only mods. Peers, saves and replays must match `content_hash`. Replays record `presentation_hash`, and a mismatch only warns." |
| G-018 | 16 G7 | The lobby handshake is said to carry `run_seed`, but no seed exists before the host commits the run. | canon amendment | **§2.4**: replace "Every save, replay and lobby handshake carries `sim_version: u32`, `content_hash: [u8; 32]` and `run_seed: u64`." with "Every save, replay and lobby handshake carries `sim_version: u32` and `content_hash: [u8; 32]`. Saves and replays also carry `run_seed: u64`; a lobby receives it in the run header the host sends when it commits the run (doc 16)." |
| G-019 | 02 G11 | Single player may save at any boundary, even while a remote invader holds slot 4. | canon amendment | **§2.5**: replace "Single player may save at any tick boundary." with "Single player may save at any tick boundary, except while replay slot 4 holds an invader: a save request then completes on the tick `ev.invasion_ended` is emitted." |
| G-020 | 03 G23; 09 G6 | `light_at` reads the grid only, so an arena blackout cannot darken it, and stealth-only light terms have no place. | canon amendment | **§2.3**: replace "Stealth and composure use effective light. Spawn rules use `light_block` only (§12.3)." with "Every read of a cell's light, `light_at` and spawn light included, is clamped by the active light overrides of doc 03 §10.6, such as an Editor's blackout, which change no stored level. Composure uses `light_at`. Stealth uses doc 09's `stealth_light`, which starts from `light_at`. Spawn rules use the clamped `light_block` only (§12.3)." |

### 2.4 Random number generation (canon §3)

| ID | Sources | Gap | Decision | Canon text (target) or ruling |
|---|---|---|---|---|
| G-021 | 05 gap 11; 06 G8; 09 G3; 04 (doc 05 row) | §3.3 calls `seq` a per-actor counter reset each tick, while docs 04, 05, 06 and 09 allocate fixed slots; a `seq` above 0xFFFF also overflows the checked `<< 48`. | canon amendment | **§3.3**: replace "`seq` is a per-actor action counter that resets to 0 each tick." with "`seq` is in `0..=0xFFFF`; larger values panic in the checked shift of §1.3. It separates draws that share `(stream, actor, tick)`. Each draw takes its `seq` from a fixed allocation published by the drawing document, never from a count of other entities' draws, and a running per-actor counter is allowed only inside a range its document reserves. Reserved: `0x0000..=0x01FF` on `rng.combat` and `rng.status` (doc 05 §1.4 action and hit slots); `0x2000..=0x2FFF`, `0x3000` and `0x4000..=0x4FFF` (doc 06); `1000..=1006` on `rng.stealth` (doc 09); `0xFFFE` on `rng.check` (madness episodes) and `0xFFFF` on `rng.combat` (Death's Door), both doc 04. The per-tick action counters of docs 04 and 10 stay below `0x2000`. Allocations on one stream never overlap." In the trap-detection paragraph replace "with `seq` = the object's index in ascending ID order among objects in range." with "with `seq` = the object's index in ascending ID order among objects in range, which stays below 1000." |
| G-022 | 10 gap 4 | A value that must stay fixed for one object (an item's hidden appraisal estimate) has no key, because keyed draws are keyed by action. | canon amendment | **§3.3**: after the bullet that begins "Loot drops use `actor`" add the bullet "**Object-keyed draws.** A value that must stay fixed for one object across ticks uses `keyed_rng(stream, (key >> 40) as u32, key, u64::MAX, purpose)`, where `key` is the object's uid or its owner's stable object key (doc 10's `est_key`) and `purpose` is a code the stream's owner allocates. `u64::MAX` is never a real tick, so object keys never collide with action keys." |
| G-023 | 04 gap 1; 09 G9; 13 gap 19 | Doc 04's checks and madness episodes draw from `rng.check`, which §3.3 does not list; doc 04 also puts disguise checks there while doc 09 uses `rng.stealth`. | canon amendment | **§3.3** keyed table: add the row "Keyed stream `rng.check`; owner 04; used for non-combat skill checks (doc 04 §4.2) and madness episodes. Disguise challenges are AI detection rolls and use `rng.stealth` (doc 09)." Doc 13's deterministic dialogue and taming stay drawless. |
| G-024 | 03 G3, G17 | Explosion block breaks, explosion drops and incendiary ignition are action-caused but drawn from sequential streams. | canon amendment | **§3.3**: after the paragraph that ends "cannot shift it." add "Exception: explosions resolve host-side in doc 03's explosion-queue order and are never client-predicted, so their block-break variation and block drops draw from `rng.physics`, and their incendiary ignition from `rng.fire` (§3.2), with the counts doc 03 lists." **§3.2**, `rng.physics` row: replace "debris and explosion block-break variation" with "debris, explosion block-break variation and explosion block drops". |
| G-025 | 07 gap 6; 10 gap 3; 11 gap 4 | The stream tables list some uses; owners need more. | canon amendment | **§3.3**: after the keyed-stream table add "The "Used for" column names each stream's canonical uses. A stream's owner may add uses inside its own domain and lists each with its draw count (§3.4): for example doc 07's crafted perk seeds, mining bonus units and seed returns on `rng.craft`, doc 10 §5.12 on `rng.ident`, and doc 11's bag, bundle, salvage, mimic, quest and illumination rolls on `rng.loot`. Per-shot and per-swing chance components (doc 05 §1.4) are on-hit procs on `rng.combat`." |
| G-026 | 11 gap 6 | "Fortune Engine reels": Engines have no reels. | canon amendment | **§3.3**, `rng.loot` row: replace "Fortune Engine reels" with "Fortune Engine dispensing". |
| G-027 | 12 G2; 11 gap 11 | Docs 11 and 12 split sequential streams into labeled sub-streams that the canon does not define. | canon amendment | **§3.2**: after the code block that ends "rng = Rng::from_seed64(stream_seed(...))" add "**Sub-streams.** A sequential stream's owner may split it into labeled sub-streams with `derive(parent, salt) = mix64(parent ^ mix64(salt))`, seeding `Rng::from_seed64(derive(stream_seed(run_seed, name, folio_index), fnv1a64(label) ^ index))` or a longer chain of `derive` calls that the owner documents (doc 11 `place_rng`, doc 12 §7.2). A sub-stream never advances its parent, so previews may recompute it and stages may run in any order." **§21**: add test 47 (§5.2). |
| G-028 | 12 G16 | `rng.loot_place` is given "floor loot placed by worldgen" while worldgen sub-streams choose the spot counts and positions. | canon amendment | **§3.2**: `rng.worldgen` "Draws for" becomes "layout, rooms, blocks, dressing, and the counts and positions of loot spots (chests, floor spots, caches)"; `rng.loot_place` "Draws for" becomes "everything placed in loot spots and containers, and shop stock". |
| G-029 | 14 gap 5 | Doc 14's run-layer draws have no stream. | canon amendment | **§3.2** table: add the row "Stream `rng.run`; scope folio; owner 14; draws for Daily setup and Weekly mutators (`folio_index = 0`), marginal notes and Marginal mutations." |
| G-030 | 11 gap 5; 14 (boons) | One offer stream per slot serves many offer contexts (Engine faces, Volume rewards, boons). | canon amendment | **§3.5**: append "An offer's generator is `Rng::from_seed64(mix64(peek ^ mix64(context_uid ^ fnv1a64(purpose))))`, where `peek` is the next output of `rng.offer.p<slot>` read without advancing it, `context_uid` is the uid or `EntityId` of the offering object, and `purpose` is a label the owning document fixes (doc 11 §11.2, doc 14 boons). Each commit advances the stream by exactly one `next_u64`." |
| G-031 | 15 gaps 1, 11 | §3.6 says "Paid gacha", which does not exist; Keepsake generation seeds an `Rng` outside the named streams. | canon amendment | **§3.6**: replace "Paid gacha never uses this generator. Doc 15 specifies a server-side ChaCha20 generator with commit-reveal." with "Account draws (Inkwell pulls and account Reliquaries) never use this generator: doc 15 specifies an HMAC-SHA256-keyed ChaCha20 generator with commit-reveal. Doc 15 §5.4 alone seeds `Rng::from_seed64` outside the simulation, for Keepsake item generation; the Keepsake enters the simulation only as a recorded item record." |
| G-032 | 18 gap 3 | Presentation randomness has no canon home. | canon amendment | Add **§3.8** "Presentation generators": "Rendering, audio and narration draw only from `nar_rng(run_seed, folio, job, key) = Rng::from_seed64(mix64(stream_seed(run_seed, job, folio as u64) ^ mix64(key)))`, where `job` is a `nar.job.` ID, and from `pres_hash(tag, a, b) = mix64(fnv1a64(tag) ^ mix64(a ^ mix64(b)))` (doc 18 §1.2). They never read or advance a simulation stream, and their outputs never enter simulation state, saves or `state_hash`." |

### 2.5 World and movement (canon §4)

| ID | Sources | Gap | Decision | Canon text (target) or ruling |
|---|---|---|---|---|
| G-033 | 03 G2; 08 gap 2 | Canon types liquid volume as `Ml`, but the grid stores whole units; the gas concentration scale is undefined. | canon amendment | **§4.4**: replace "Liquid capacity is `floor_div(1000 × (64 - popcount(mask)), 64)` units." with "The liquid grid stores whole units of 1 litre (1000 ml). A cell holds at most `floor_div(1000 × (64 - popcount(mask)), 64)` units, 1000 when its mask is empty. `Ml = units × 1000` at every boundary, so any non-zero unit meets the 100 ml contact threshold of §12.1. Gas concentration is `0..=10000` per cell, where 10000 is a saturated cell of 1,000,000 ml (doc 03)." |
| G-034 | 03 G13; 04 gap 5 | The order of the genre `fall_dmg_bp` and doc 04's fall-damage factors is unstated. | canon amendment | **§4.2**: replace "Acrobatics (doc 04) multiplies it afterwards." with "It is then multiplied by the local genre's `fall_dmg_bp` (§5.9), then by each doc 04 fall-damage factor (Acrobatics, kin, talents) in ascending source ID order, one `mul_bp` each." |
| G-035 | 04 gap 4; 05 gap 10; 06 G6 | Leg and crutch factors, combat and casting stance speeds and armor sprint and swim bases have no place in the speed chain. | canon amendment | **§4.2**: replace "**Final move speed** is base speed, then §6.4 row 11, then each active status multiplier in ascending status ID byte order." with "**Final move speed** is base speed, then §6.4 row 11, then doc 04's body factor `body_move_bp` (severed legs, crutch, leg wounds), then each active status multiplier in ascending status ID byte order. Base speed is the table above, or the doc 05 stance base that replaces it (combat and casting stances, armor sprint and swim bases); armor never multiplies speed." |
| G-036 | 03 G14; 07 gap 2; 12 G17 | The `destructible` tag has no ID, its consequences are implicit, and the list omits light fixtures, rubble, pit lids and hull panels. | canon amendment | **§4.3**: replace the bullet that begins "**Terrain is fixed.**" with "**Terrain is fixed.** Worldgen blocks are indestructible unless their block definition carries the tag `block.tag.destructible` (doc 07). Destructible blocks are doors, crates, barrels, cracked walls that hide secrets, wooden structures, glass, ice, webs, light fixtures, rubble, pit lids, hull panels and every Errata placeable; no structural wall or floor is destructible. Explosions, melee and projectiles damage only destructible blocks. A block without the tag never burns, melts, falls, washes away, decays or explodes, and changes type only through worldgen, a scripted event or a swap between doc 07's lit and unlit twins; doors, growth and connections may still change its properties." |
| G-037 | 05 gap 19; doc 11 Somnambule row | Whether projectiles may pass through fixed terrain; `mfr.oneiric.somnambule` passes "through blocks". | canon amendment | **§4.3**: add the bullet "**Projectiles and terrain.** Fixed terrain stops every projectile, beam and penetration; only destructible or non-opaque cells can be penetrated (doc 05 §7.3)." **§5.9**: replace "`mfr.oneiric.somnambule`: projectiles pass through blocks after 10 m of flight." with "`mfr.oneiric.somnambule`: after 10 m of flight, projectiles pass through destructible and non-opaque blocks without spending penetration (§4.3)." |
| G-038 | 03 G24 | Fences and panes as mantle targets. | chapter owns it | Doc 03 §5.9: never mantle targets, so a 1500 mm fence contains anyone below VERVE 7. |
| G-039 | 12 G8 | No per-cell gravity sign. | chapter owns it | Doc 03's `flip` bit inside doc 12's Oneiric volumes; §5.2 already gives the volumes to doc 12. |
| G-040 | 03 G10 | What "open areas" means for `ambient_light`. | chapter owns it | Doc 03 §10: sky-exposed or worldgen-tagged cells, falling off by 1 per step. |
| G-041 | 03 G7; 12 G13 | Per-genre temperature and other GPV fields are not in canon. | chapter owns it | Doc 12's GPV owns `ambient_decic`, `fixture_level`, `fog`, `liquid_viscosity_bp`, `gas_dissipation_bp`, `damage_affinity` and `hard_bp`; §5.2 gives doc 12 the full vector. |
| G-042 | 03 G9 | Sound propagation precision. | chapter owns it | Doc 03 propagates in tenths of a dB and floors to whole `Db` on output. |
| G-043 | 03 G25 | Listeners in inactive chunks. | chapter owns it | Doc 03: they hear nothing. |
| G-044 | 03 G22 | Relink portals. | chapter owns it | Doc 03 §5.21: only bodies and projectiles cross; every grid and ray uses physical adjacency. |
| G-045 | 03 G11 | Pose tables stored as keyframes. | chapter owns it | The loader expands keyframes to the canon per-tick table with integer interpolation; the expanded table is the canonical one (§9). |
| G-046 | 03 G12 | Fluids and thermal serialization encodings. | chapter owns it | Resolved on disk: doc 02 §6.5 and doc 03 §16.1 now agree (sparse fluid lists; `palette_rle` of `u16` thermal offsets). Doc 03 deletes G12. |
| G-047 | 03 G26; 10 gap 8; 13 gap 8; 14 gap 19 | Systems missing from §2.2: run layer, squad coordinator, Director, path service, appraisal. | chapter owns it | §2.2 gives doc 03 the system order. Doc 03's lettered rows (17a, 26a, 26b, 34a) stand; AI services run inside `sys.ai_think`, appraisal inside `sys.needs`. Doc 10 drops its stale "exactly 35 systems". |
| G-048 | 03 G18 | Doc 05's C1 runs before movement. | chapter owns it | Doc 03's order; doc 05 agrees. |
| G-049 | 03 G19 | Two chapters described explosions. | chapter owns it | Doc 03 §12 governs; doc 05 supplies `inner_mm` and `edge_bp`. |
| G-050 | 03 G20 | One registry of Errata-placed blocks. | chapter owns it | Doc 07 `placed_meta`, read through doc 03 `is_errata_placed(c)` (see C-24). |

### 2.6 Axioms, genres, Contradiction and Anachronism (canon §5)

| ID | Sources | Gap | Decision | Canon text (target) or ruling |
|---|---|---|---|---|
| G-051 | 01 G1; 03 G1; 07 gap 1; 08 gap 1; 09 G1; 12 G1; 13 gap 1; 16 G1; 17 G1 | Chapter headers name canon versions from 1.2 to 1.5. | chapter owns it | Each header names the canon version it was checked against; after this triage that is canon-1.11. Every chapter also applies canon-1.8 (no mounts, C-30) and canon-1.10 (no item durability, C-31). |
| G-052 | 18 gap 1 | The Mythic Fantasy Lucre skin "Crowns" is The Elder Scrolls Online's purchasable currency, which §5.2's naming rule forbids. | canon amendment | **§5.2**, row 1, Lucre skin: replace "Crowns" with "Nobles". |
| G-053 | 18 gap 2 | The Unwritten's Lucre skin "Lucre" equals `cur.lucre`'s display name, which the naming rule forbids. | canon amendment | **§5.2** naming rule: replace "the display name of any `cur.` or `ent.` entry, or the word Ink." with "the display name of any `cur.` or `ent.` entry other than its own base currency, or the word Ink; `genre.unwritten`'s Lucre skin "Lucre" therefore passes." |
| G-054 | 18 gap 4 | Palette size. | chapter owns it | Doc 18 keeps the canon's 3 swatches first and exact, and adds 4. |
| G-055 | 07 gap 5; 08 gap 5 | "Grafts" is ambiguous between body grafts and licensed weapon grafts; supernatural alchemy is always `arcana` while glyphs follow the origin's aether axiom. | canon amendment | **§5.3**: replace the row "Enchantment affixes, grafts, supernatural alchemical effects \| arcana" with "Enchantment affixes, doc 04 body grafts \| arcana"; replace the row "Spell glyphs, wands, staves, magic texts \| `{origin_genre.aether_axiom}`; `{arcana}` when the origin's aether_axiom is none" with "Spell glyphs, wands, staves, magic texts, supernatural alchemical effects \| `{origin_genre.aether_axiom}`; `{arcana}` when the origin's aether_axiom is none". After the table add "Holy, unholy and curse alchemical effects keep `mythos`. Licensed grafts (`cat.part`) take the dependency of their gimmick (doc 11)." |
| G-056 | 08 gaps 1, 10 | Potions have no failure definition and do not count toward `st.unmoored`. | canon amendment | **§5.5**: after "Doc 07 defines a failed gadget as a spark dealing 5% of the user's Max HP as `dmg.shock`." add "Doc 08 defines a failed potion: it is consumed, and only its toxicity (drunk) or its plain base-liquid spill (thrown) applies." **§5.11**: replace "(jams, fizzles, sparks)" with "(jams, fizzles, sparks, failed potions)". |
| G-057 | 05 gap 20 | No shared counter for Unmoored failures. | chapter owns it | Doc 06's `note_contradiction_failure(t)` is the one per-character counter; docs 05, 07 and 08 call it, and doc 12, owner of `st.unmoored`, cites it. |
| G-058 | 04 gap 15; 11 gap 9 | What "Fortune-derived effects are off" means. | chapter owns it | Doc 04: no Death's Door roll and `dd_used` unchanged; `avatar_of_fate` suspended. Doc 11: effective SER 40 for §13.4 and an Engine SER shift of 0. |
| G-059 | 05 gap 25 | "Non-boss" in Shock of the New does not name ranks. | canon amendment | **§5.6**: replace "on each native non-boss target" with "on each native target whose rank is not `rank.boss`, `rank.editor` or `rank.hunter`". |
| G-060 | 12 G7 | `local_genre` inside a seam zone. | canon amendment | **§5.8**: replace "Inside a seam zone, `local_axiom(pos, a)` is the larger" with "Inside a seam zone, `local_genre(pos)` is the Folio's genre, `local_axiom(pos, a)` is the larger". |
| G-061 | 04 gap 8 | Which sources feed `talent_tol` and which feed `gear_tol`. | chapter owns it | Doc 04: kin, Origin traits, talents and Marginalia feed `talent_tol`; augments feed `gear_tol`; World Law grants stay separate. |
| G-062 | 11 gap 16 | No cap on the gear tolerance term. | chapter owns it | `GEAR_TOL_CAP = 3` is doc 11's balance rule under §5.4's total cap of 6. |
| G-063 | 04 gap 13 | Going Native details. | chapter owns it | Doc 04 §2.8: the Origin trait is replaced and creation history is kept. |
| G-064 | 12 G10 | Which worldgen features are "electronic" for the TEC ≤ 2 rule. | chapter owns it | Doc 12's list; doc 13 turrets inherit it. |
| G-065 | 05 gap 17 | Manufacturer traits that touch doc 05 mechanics. | chapter owns it | Doc 11 owns the trait mechanics and doc 05 the hooks. Rustmaw and Hephaesteum follow their canon-1.10 text. |

### 2.7 Attributes, skills and experience (canon §6 to §8)

| ID | Sources | Gap | Decision | Canon text (target) or ruling |
|---|---|---|---|---|
| G-066 | 04 gap 3 | §6.4 names only `gear` terms, so other modifiers have no position. | chapter owns it | Doc 04 §1.2's registry: flat terms at the gear position, post factors after the clamp and before statuses. The one pipeline factor is G-086. |
| G-067 | 06 G2 | Which glyphs' GLYPH terms multiply a multi-stage spell. | canon amendment | **§6.4** row 29: replace "then × the GLYPH term of §5.7 for each glyph" with "then × the GLYPH term of §5.7 for each glyph of the stage that produces the effect (doc 06)". |
| G-068 | 10 gap 9 | Appraisal rate, Analysis chance and psychometry chance are defined outside §6.4. | chapter owns it | They resolve doc 10 actions and compete with no §6.4 row, like doc 09's lock ratings. |
| G-069 | 09 G17 | Active search radius and chance against row 35's "within 3 m". | chapter owns it | A situational multiplier on row 35's result, `min(9000, mul_bp(row35, 15000))` within 5000 mm; row 35 is unchanged. |
| G-070 | 13 gap 7 | Skill XP is normalized by the party-scaled Max HP, so each member of a party of `n` gets `enemy_count_bp[n] / n` of the solo pace (4375 Bp at 4 players) and misses the §7.2 contract. | canon amendment | **§7.1**: replace "A hit that deals `dmg_mhp` to a hostile with Max HP `target_max_mhp` grants `floor_div(1500 × dmg_mhp, target_max_mhp)` milli-XP to the weapon's skill (capped at 1500 per hit)." with "A hit that deals `dmg_mhp` to a hostile grants `floor_div(1500 × dmg_mhp, norm_mhp)` milli-XP to the weapon's skill, capped at 1500 per hit, where `norm_mhp = max(1, floor_div(target_max_mhp × 10000, party_hp_bp))` and `party_hp_bp` is the §17.2 enemy HP multiplier applied to that target (10000 solo)." |
| G-071 | 13 gap 6; 04 gap 12 | §8 asks for "near 40 units" of XP per Folio while §17.4's 40 kill equivalents plus skill-ups and exit XP pay about 45. | canon amendment | **§8**: replace "At 40 units per Folio this gives level 12 after Folio 10, level 29 after Folio 25 and level 47 after Folio 40." with "At 45 units per Folio this gives level 12 after Folio 10, level 29 on arrival at Folio 25 and level 49 after Folio 40." Replace "Doc 13 keeps the per-Folio total near 40 units for a combat-typical run, so the level-28 anchor holds." with "Doc 13 keeps the kill supply near §17.4's 40 standard-kill equivalents and the total of all sources near 45 units per Folio for a combat-typical run, so the level-28 anchor holds." (Checked: 45 units per Folio gives levels 12, 29 and 49 at those points.) |
| G-072 | 04 (doc 14 row); 14 gap 13; 16 G16 | Whether ghosts and downed members share kill XP. | canon amendment | **§8** co-op split: replace "every party member present in the Folio gains" with "every party member present in the Folio, downed members and ghosts included (Dormant and Departed slots excluded, doc 16), gains". |
| G-073 | 13 gap 2; 04 gap 2; DECISIONS 62 | Pacification pays XP, but §8 lists only kills. | canon amendment | **§8**: replace "Kills: `mul_bp(mul_bp(kill_xp_std(d), rank_xp_bp), 7000)`, with `rank_xp_bp` from §17.3." with "Kills and pacifications: `mul_bp(mul_bp(kill_xp_std(d), rank_xp_bp), 7000)`, with `rank_xp_bp` from §17.3. A pacified enemy (doc 13) pays this once, as if killed, and emits `ev.entity_pacified` instead of `ev.entity_killed`." |
| G-074 | 13 gap 3 | Who earns XP for an enemy killed by another faction or the environment. | chapter owns it | Doc 13 §11.1: the killing blow, or 2500 Bp of the victim's Max HP dealt by `fac.errata`. |
| G-075 | 13 gap 4 | XP for summoned enemies. | chapter owns it | 0 kill XP (doc 13). |
| G-076 | 04 gaps 6, 7 | `kin.human` "no changes"; `kin.construct` "no satiation". | chapter owns it | Doc 04 owns kin traits (§16.3): creation-only bonuses for humans; a construct's satiation is displayed as Fuel under unchanged rules. |
| G-077 | 04 gap 9 | Implant slot capacity. | chapter owns it | Doc 04: 4 augment points per implant slot. |
| G-078 | 04 gap 10 | Movement values for non-humanoid Echoes. | chapter owns it | Doc 04 §13 replaces §4.1 to §4.2 for those bodies; §4.1 is titled "Humanoid player body". |
| G-079 | 04 gap 14; 14 gap 18 | Echo trait IDs. | chapter owns it | `trait.echo.<echo_name>` in doc 04's namespace, authored by doc 14. Resolved on disk. |

### 2.8 Body plans (canon §9)

| ID | Sources | Gap | Decision | Canon text (target) or ruling |
|---|---|---|---|---|
| G-080 | 04 gaps 11, 17 | One Folio-1 boss slash, or two standard slashes through starter armor, sever an Erratum's arm; "reaches 0" leaves a pool already at 0 unseverable. | canon amendment | **§9.1**: replace "A severable part is severed when its condition pool reaches 0 from a hit whose largest share is `dmg.slash`, or which carries the flag `sever`." with "A severable part is severed by a hit with positive part damage whose largest share is `dmg.slash`, or which carries the flag `sever`, when the part's condition pool is 0 after the hit, including a pool that was already 0. An Erratum's limb severs only if its pool was at most 2500 Bp of its max before the hit (doc 04 `wound.crippled` or worse), or the hit's part damage is at least 5000 Bp of the pool's max." |
| G-081 | 04 gap 19 | Plans without a torso, mass or chassis have no donor for added parts. | canon amendment | **§9**: replace "Added parts take their share from `bp.torso`, or from `bp.mass` or `bp.chassis` in plans without a torso." with "Added parts take their share from `bp.torso`; in plans without a torso, from `bp.mass`, `bp.chassis`, `bp.body_front` (serpentine) or `bp.thorax` (insectoid)." |
| G-082 | 04 gap 20 | No data for `bp.horn` and `bp.weakpoint_N`. | chapter owns it | Doc 04 §15.8: horn share 500 and `hit_mult` 8500; weakpoints `hit_mult` 10000 with precision; neither severs. |
| G-083 | 04 gap 18 | DoT, suffocation, starvation and overdose have no hit part. | chapter owns it | They reduce HP only; step 8 subtracts from a part only when the hit has one. |

### 2.9 Damage (canon §10)

| ID | Sources | Gap | Decision | Canon text (target) or ruling |
|---|---|---|---|---|
| G-084 | 05 gap 1; 11 gap 3 | Attack factors (combo, heavy, sprint, range falloff, cleave and others) and "× N" manufacturer traits have no pipeline slot. | canon amendment | **§10.4**: after "Each arrow is one `mul_bp`, in that order." add "Before this chain, doc 05's attack factors (move, charge, sweet spot, range falloff, penetration keep, cleave, area falloff and exposure; doc 05 §10.1) multiply `base_mhp` or `template_dmg_mhp`, one `mul_bp` each in doc 05's order. After it, a manufacturer trait stated as "× N" (§5.9) multiplies `B` once, before step 1." Doc 05's mounted-charge factor is retired with mounts (canon-1.8). |
| G-085 | 05 gap 2 | `st.taunted`'s damage effect has no step. | canon amendment | **§10.5** step 2.12: replace "`st.berserk` 13000, `st.empowered` 12000, `st.weakened` 7500;" with "`st.berserk` 13000, `st.empowered` 12000, `st.taunted` 7500 when the target is not the taunter, `st.weakened` 7500;". |
| G-086 | 04 gap 3 | `kin.fae`'s iron weakness (§16.3) has no step. | canon amendment | **§10.5** step 3a: append "Every `d[t]` × 12500 if the target is `kin.fae` and the hit comes from an item whose primary material is `mat.iron` or `mat.cold_iron` (§16.3)." |
| G-087 | 05 gap 3 | Which armor a blocked hit meets. | canon amendment | **§10.5** step 5: after the sentence that ends "(doc 09 grants it to sneak attacks at Sneak 100):" and before the code block insert "For a blocked hit, `AR_part` is the hit part's AR plus the blocking shield's AR, which scales with `T(ilvl)` (§13.8)." |
| G-088 | 05 gap 4 | Step 6b reduces only the total, but step 7 needs per-type values. | canon amendment | **§10.7**: after the code block add "The `total` that remains is split back over the types in proportion to their `d[t]`, one `floor_div` each in §10.1 order, with the remainder to the largest-share type (ties to the earlier §10.1 row), so step 7 sees per-type values." |
| G-089 | 17 G3 | Scribe's Mercy has no pipeline step. | canon amendment | **§10.5**: after "**6b. Block** (§10.7), if the hit was blocked." add the sub-step "**6c. Scribe's Mercy** (doc 17): if the target is an Erratum whose slot has `mercy_on`, each `d[t] = d[t] - mul_bp(d[t], mercy_bp)` in §10.1 order. Every other involuntary HP loss of that Erratum outside the pipeline, such as starvation, is reduced the same way. HP the Erratum chooses to pay is never reduced." |
| G-090 | 05 gap 6 | Ward generators deplete with a per-type profile, and `counter.ward_breaker` needs a defined multiplier. | canon amendment | **§10.5**: replace step 7 with "7. **Ward absorption.** The `st.shielded` pool is consumed first, then the ward generator's ward. For a pool, `m[t] = ward_mult[t]` for `st.shielded` and `mul_bp(ward_mult[t], profile[t])` for the generator, where `profile` is the generator's per-type profile (doc 05, 10000 by default); a `counter.ward_breaker` hit multiplies `m[t]` by 30000 for every pool (§17.3a). For each type in §10.1 order with `d[t] > 0` while ward remains: `w = mul_bp(d[t], m[t])`. If `w <= ward`, then `ward -= w` and `d[t] = 0`. Otherwise `d[t] -= floor_div(ward × 10000, m[t])` and `ward = 0`, and `ev.ward_broken` is emitted. HP takes the rest." Test 41 is unchanged. |
| G-091 | 08 gap 13 | No entry point for a consumable's damage to its own drinker or injected target. | canon amendment | **§10.6**: add the row "Source: a consumable's damage to its own drinker or injected target (doc 08). Enters at: step 3 with `D2 = B`. Skips: steps 1 and 2 (additive bucket, crit, friendly fire). Notes: none." |
| G-092 | 09 G4 | No entry point for a lethal takedown. | canon amendment | **§10.6**: add the row "Source: a lethal takedown (doc 09). Enters at: step 7 as `dmg.true` equal to the target's HP plus ward. Skips: crit, resistance, armor, chip floor. Notes: flag `takedown`." |
| G-093 | 05 gap 12 | Blunt's poise "× 15000" is undefined for mixed shares; poise reset timings are not in canon. | canon amendment | **§10.1**, `dmg.blunt` row: replace "`st.stunned`; poise damage × 15000" with "`st.stunned`; poise damage × `10000 + floor_div(share_blunt, 2)` Bp, so × 15000 for a pure blunt hit". Poise reset timings stay doc 05's. |
| G-094 | 03 G5 | Doc 03 resolves environmental, contact and DoT hits after weapon hits, against §10.8's single (source, seq) order. | canon amendment | **§10.8**: replace "Hits in the same tick resolve in ascending (source EntityId, seq)." with "Hits in the same tick resolve in two batches: actor hits (weapons, spells, gadgets, thrown items) first, then environmental, contact, surface and DoT hits (doc 03 `sys.damage` and `sys.damage_late`). Inside each batch they resolve in ascending (source EntityId, seq)." |
| G-095 | 05 gap 23 | Sonic ending `st.frozen`. | already in canon | §10.1 ends the freeze; step 3a's × 15000 names blunt only. |
| G-096 | 05 gaps 21, 22; 08 gap 14 | Crit draws for `NO_CRIT`, parried, negated and alchemical area hits. | already in canon | §3.4: every hit draws one crit roll; these draw and ignore it. Environmental blasts and DoTs enter after step 2 (§10.6) and draw none. |
| G-097 | 04 gap 16 | A charmed player's hits on the applier's faction. | chapter owns it | Buildup and on-hit effects are × 0 as well; `ev.damage_dealt` is still emitted with total 0. |
| G-098 | 05 gap 18 | Sneak multiplier against heavy-attack power. | already in canon | Doc 05 keeps the multiplicative chain of §10.5. |
| G-099 | 05 gap 5 | Guard-break extensions. | chapter owns it | §10.7 gives doc 05 arcs and timing. |
| G-100 | 14 gap 4 | Which hits step 2.13 applies to. | chapter owns it | §10.5 step 13 is "difficulty multiplier (doc 14)"; doc 14 §7.3's `m13`. |
| G-101 | 05 gap 24 | Elevation range for a projectile in flight. | chapter owns it | Doc 05 §7.1 measures the drop below the spawn height. |
| G-102 | 05 gap 26 | A floor on material-driven attack speed. | chapter owns it | Doc 05 §3.3. |
| G-103 | 05 gap 14 | Throw speed from row 20's range multiplier. | chapter owns it | Doc 05 §9.1 uses its square root. |
| G-104 | 05 gap 15 | Default status powers per damage type. | chapter owns it | Doc 05 §13. |
| G-105 | 06 G11 | The friendly-fire class of spell bursts. | already in canon | Orb, nova, rune, cone and chain hits are spell hits (`FF_DIRECT_BP`); spell-made fire, surfaces and liquids are area (`FF_AREA_BP`), as §17.5 lists. |
| G-106 | 06 G13 | NPC use of player-scaled spell bases. | chapter owns it | `NPC_SPELL_SCALE_BP = 4500` (docs 06 and 13 agree). |
| G-107 | 06 G14 | A spell's status power. | chapter owns it | `p` already includes `spell_pot`; §11.2 then runs unchanged. |
| G-108 | 06 G16 | `src_lvl` omits texts, on-use enchanted items and holy symbols. | canon amendment | **§13.8.1**, "Spell, prayer or glyph" row: replace "`ilvl` of the equipped casting implement (focus, wand, staff or tome); with none, `max(1, d_current - 3)`" with "`ilvl` of the casting implement: the equipped focus, wand, staff or tome, or the text, enchanted item or equipped holy symbol whose own activation casts it; with none, `max(1, d_current - 3)`". |

### 2.10 Status effects and player input (canon §11)

| ID | Sources | Gap | Decision | Canon text (target) or ruling |
|---|---|---|---|---|
| G-109 | 05 gap 8 | `counter.disruptor`'s 180 ticks against §11.4's duration scaling. | canon amendment | **§17.3a**, `counter.disruptor`: replace "applies `st.unraveled` and `st.silenced` directly for 180 ticks" with "applies `st.unraveled` and `st.silenced` directly for exactly 180 ticks, unscaled by §11.4,". |
| G-110 | 05 gap 16; 16 G12 | §11.1 fixes a 30 s bleed-out; doc 16 shortens repeated downs, shortens by damage and defines revive HP. | canon amendment | **§11.1**, `st.downed` row, base duration: replace "30 s bleed-out" with "30 s bleed-out for the first down in a Folio; later downs, damage shortening and revive HP per doc 16 §9.4". |
| G-111 | 09 G13 | Read literally, `st.marked` reveals the bearer to the marker's enemies. | canon amendment | **§11.1**, `st.marked`: replace "visible through walls to every hostile of the marker" with "visible through walls to the marker and every ally of the marker". |
| G-112 | 18 gap 10 | The softer Erasure hunter has `rank.elite` stats; the rank ID it carries is unstated. | canon amendment | **§11.6**: replace "one Editor hunter spawns for the party with `rank.elite` stats (product-owner decision: softer Erasure)." with "one Editor hunter spawns for the party (product-owner decision: softer Erasure). It carries `rank.hunter` for identity, immunities, post-proc protection and presentation, but uses `rank.elite`'s HP, damage, kill XP, DoT-taken and control-duration multipliers and the minion-to-champion row of §17.2." |
| G-113 | 13 gap 16 | §11.6 makes every `fac.editors` member erase placeables; §17.9 forbids the invader to break blocks. | canon amendment | **§11.6**: replace "Hunters and every `fac.editors` member erase Errata placeables" with "Hunters and every `fac.editors` member except an invader (§17.9) erase Errata placeables". |
| G-114 | 16 G4 | Whether a substituted command for a missing packet is synthesized input. | canon amendment | **§11.8**: after "Input is never synthesized for a player." add "When a player's command for a tick is missing, the host applies doc 16's neutral substitute: no movement, every action button released, stance and quickbar index kept. It holds no press edge, so it starts nothing; it is the absence of input, and the replay log records it." |
| G-115 | 17 G17 | Toggle conversion, aim assist and the gamepad casting context. | already in canon | They transform the player's own inputs and never create a press, so §11.8 and §22 hold. |
| G-116 | 06 G9 | Which statuses a dispel removes. | chapter owns it | Doc 06's `dispellable` flag set at application. |
| G-117 | 06 G10 | `st.dread` on NPCs, which have no composure. | chapter owns it | No numeric effect; doc 13 reads it as flee pressure. |
| G-118 | 08 gap 20 | Overdose cadence at the toxicity cap. | chapter owns it | 3 stacks on the first needs tick at the cap and on every fifth consecutive needs tick there. |

### 2.11 Resources and surfaces (canon §12)

| ID | Sources | Gap | Decision | Canon text (target) or ruling |
|---|---|---|---|---|
| G-119 | 03 G8; 08 gap 3 | No gas concentration for contact; respirator scope and filters. | canon amendment | **§12.1**: replace "or the eye cell holds the gas." with "or the eye cell holds the gas at concentration 500 or more (§4.4). A working respirator in `slot.face` blocks every gas contact row; doc 08 owns respirator filters." **§12**, `res.breath` row: replace "a respirator in `slot.face` makes gases breathable" with "a working respirator in `slot.face` makes gases breathable (doc 08 filters)". |
| G-120 | 03 G8, G15, G16; 08 gap 4 | Trigger cells, the 64-cell charge, the freeze minimum, the steam cap and condensation, web burnout. | chapter owns it | Doc 03 §6.9 and doc 08 §5.2, consistent with §12.1: "1 cell per 1000 ml" is one saturated cell per liquid unit. |
| G-121 | 08 gap 9 | Blood has triggers but no contact row. | chapter owns it | No contact effect. |
| G-122 | 08 gap 12 | `splash` inflated a 250 ml flask to 9 litres. | chapter owns it | Shattered vessels deposit into one cell (doc 08 §2.3). |
| G-123 | 08 gap 15 | Lava froze on its first liquids run. | chapter owns it | Lava and molten metal have no thermal freeze point. |
| G-124 | 09 G11 | Tall grass conceals only crouched targets. | canon amendment | **§12.2**: replace "tall grass 2 (only against crouched targets)" with "tall grass 2 (only against crouched or downed targets and corpses)". |
| G-125 | 12 G15 | Spawn light covers ambient and wave spawns but not the initial population. | canon amendment | **§12.3**: replace "Ambient and wave spawns (doc 13) occur only in cells" with "Ambient spawns, wave spawns and the initial population (doc 13) occur only in cells". |
| G-126 | 07 gap 8 | Errata-placed webs rooting allies. | chapter owns it | Doc 07: a placed web never roots `fac.errata`. |
| G-127 | 06 G12 | Whether `res.favor` is per Erratum or per party. | already in canon | §12 resources are per entity, so favor is per Erratum and per patron. |

### 2.12 Items (canon §13)

| ID | Sources | Gap | Decision | Canon text (target) or ruling |
|---|---|---|---|---|
| G-128 | 07 gap 17 | Traps appear under both `cat.gadget` and `cat.placeable`. | canon amendment | **§13.1**: in the `cat.gadget` row replace "deployables: turrets, drones, traps, mines" with "deployables: turrets, drones, snares, mines, charges"; in the `cat.placeable` row replace "ladders, traps, light sources" with "ladders, block traps (spikes, webs), light sources". |
| G-129 | 05 gap 7; 07 gap 9 | Ward capacity has only `T(ilvl)`, with no rarity, quality, enhancement or potency term. | canon amendment | **§13.2**, `slot.ward`: replace "capacity `mul_bp(cap_base_mhp, T(ilvl))`" with "capacity `cap_base_mhp → × T(ilvl) → × rarity power → × quality → × (10000 + 600 × eff_enh) → × Contradiction potency`, one `mul_bp` per arrow, with no material term". |
| G-130 | 11 gap 19 | `slot.hand_extra_N` has no bound and the Exotic limit ignores extra hands. | canon amendment | **§13.2**: replace the row "`slot.hand_extra_N` \| granted by `bp.arm_extra_N` (§9.1): one one-handed weapon or off-hand item" with "`slot.hand_extra_1`, `slot.hand_extra_2` \| granted by `bp.arm_extra_1` and `bp.arm_extra_2` (§9.1, doc 04): one one-handed weapon or off-hand item". **§13.3**, Exotic: replace "at most one Exotic among `slot.weapon_1..3` and `slot.offhand`" with "at most one Exotic among `slot.weapon_1..3`, `slot.offhand` and `slot.hand_extra_1..2`". |
| G-131 | 11 gap 2 | `enhancement` is typed `0..=15`, but the Marginalia cap exceeds 15. | canon amendment | **§13.7**: replace "Stored `enhancement` is the Emendation-driven level, `0..=15`." with "Stored `enhancement` is the Emendation-driven level, `0..=ENH_CAP`, where `ENH_CAP = 15` in the main run and `15 + floor_div(d_current - 25, 5)` in Marginalia."; replace "-3, 15)" in the `eff_enh` formula with "-3, ENH_CAP)". **§13.9**, `enhancement` row: replace "i32 `0..=15`" with "i32 `0..=ENH_CAP`". |
| G-132 | 03 G27; 07 gap 18 | Block HP scaling is not among §13.8's table uses. | canon amendment | **§13.8**, uses table: in the `P(d)` row append ", and the break threshold of terrain destructible blocks at `d_current` (doc 03)"; in the `T(d)` row append ", and the HP of Errata-placed blocks at the placing Folio's depth (doc 07)". |
| G-133 | 14 gap 3 | Enemy HP has no difficulty term, yet deep Marginalia scales only through doc 14. | canon amendment | **§13.8**, `H(d)` row: replace "`template_hp × H(d) × rank HP × party-size HP`" with "`template_hp × H(d) × rank HP × party-size HP × diff_hp_bp` (doc 14; 10000 at Grade 0 before Folio 26)". |
| G-134 | 11 gap 15 | `illuminated` rolls "per drop", but shop stock, floor loot, Engine faces and rewards are not keyed drops. | canon amendment | **§13.9**, `illuminated`: replace "one keyed `rng.loot` roll at 5 Bp per Legendary-or-better drop" with "one `rng.loot` roll at 5 Bp per Legendary-or-better item, keyed at its drop, or at first acquisition for shop stock, floor loot, Engine faces and rewards (doc 11 §2.3)". |
| G-135 | 07 gap 4; 10 gap 7 | Instance fields beyond §13.9 (crafting fields, `zaps_total`, `sealed`, knowledge sub-fields). | already in canon | §13.9 lists a minimum and gives doc 19 the full schema. `durability` no longer exists (canon-1.10). |
| G-136 | 11 gap 8 | Category of the Emendation. | chapter owns it | `item.tool.emendation` is `cat.tool`, because `cat.text` is type-hidden. |
| G-137 | 10 gap 15 | Whether stored enhancement is public. | chapter owns it | Doc 10 §2.1. |
| G-138 | 10 gaps 16, 17, 18; 11 gap 13 | Stored value, vendor value, service prices and the unappraised sale rule. | chapter owns it | Resolved on disk: doc 11 prices from `value_vendor` and uses `appr.svc.*`. |
| G-139 | 10 gap 11 | What a wand "type" is. | chapter owns it | One archetype per spell; doc 05 now exports `arch.ranged.wand`. |
| G-140 | 10 gaps 10, 12, 13 | Blessing chance, binding when an item is lost, `curse.misfire`. | chapter owns it | Doc 10 §7. |
| G-141 | 10 gap 6 | Mundane items' terminal knowledge state. | already in canon | §13.6 starts them at `know.classified`. |
| G-142 | 08 gap 7; 10 gaps 1, 2 | Potion, bomb and apparatus template namespaces and rosters. | already in canon | `alch.` and `apparatus.` are doc 08's (§19); doc 11 attaches loot records to them. Doc 14 still uses former names (C-02). |
| G-143 | 08 gap 8 | Respirator archetype. | chapter owns it | Mechanics in doc 08's `apparatus.respirator.*` (C-26 for doc 11). |
| G-144 | 14 gap 17 | Lattice Lens ID. | chapter owns it | `item.trinket.lattice_lens`. Resolved on disk. |
| G-145 | 10 gap 21 | Erratum-placed altars. | chapter owns it | §4.3 has no altar placeable; doc 06's shrine lamp is a `cat.placeable` light source, and doc 10 refuses `appr.svc.*` at it. |
| G-146 | 11 gap 20 | Loot sources of books. | chapter owns it | Kits and the arcanist's and scribe's shop lines; never the deck. |
| G-147 | 11 gap 21 | Unshipped retired IDs kept as aliases. | chapter owns it | §1.1 requires aliases for shipped IDs and does not forbid others; an alias keeps the `id32` reserved. |

### 2.13 Currencies, factions, Callings and Echoes (canon §14 to §16)

| ID | Sources | Gap | Decision | Canon text (target) or ruling |
|---|---|---|---|---|
| G-148 | 11 gap 10; 16 G11 | Whether Lucre is per player, and whether it may pass between players. | canon amendment | **§14**: replace "No currency below can be bought, traded or gifted." with "No currency below can be bought, or traded or gifted between accounts." In the `cur.lucre` row, Scope: replace "run" with "run, per Erratum"; Spent on: replace "one shared value across genres;" with "one shared value across genres; passes between party members only as dropped piles (doc 11 §17);". |
| G-149 | 15 gap 5 | "Codex tiers" as a source of Ink and Quires. | already in canon | §14's last sentence forbids anything purchasable converting into Ink or Quires, so only free-track tiers pay them. |
| G-150 | 15 gap 13 | A one-time welcome Ink grant. | canon amendment | **§14**, `cur.ink` Source: replace "play time, run completion, Codex tiers, achievements" with "a one-time welcome grant, play time, run completion, Codex tiers, achievements". |
| G-151 | 15 gap 6 | Vellum sinks beyond the §14 list. | chapter owns it | §19 gives the account economy rules to doc 15. |
| G-152 | 15 gap 8 | Premium-track, expansion and cosmetic ownership. | chapter owns it | Entitlement rows keyed by SKU; §14 says entitlements are not currencies. |
| G-153 | 15 gap 7 | Account events and the run report. | chapter owns it | Doc 15 §10.4 defines the run report as a doc 16 message. |
| G-154 | 15 gap 12; 14 gap 14 | Quire payout formula. | chapter owns it | Doc 15's rate over doc 14's `run_score`. Resolved on disk. |
| G-155 | 14 gap 1 | Bindery currency. | already in canon | Quires (§14). |
| G-156 | 13 gap 9 | The Unwritten has no native factions although §15 says 2 to 4 per genre. | already in canon | §5.2's specific rule (only `fac.editors`) wins. |
| G-157 | 13 gap 21 | Faction of a non-sapient xeno hive. | chapter owns it | `fac.wild.space_opera`, per §15's sapience rule. |
| G-158 | 13 gap 10 | NPC block stability, since NPCs never use row 19. | chapter owns it | Templates carry `block_stability_bp`. |
| G-159 | 14 gap 2 | Kit ownership is split three ways. | canon amendment | **§16.2**: replace "Doc 04 owns kits and talent trees." with "Doc 04 owns kit roles, presets, kit balance and talent trees; doc 14 owns kit records (`run.kit.*`) and their template bindings." |
| G-160 | 15 gap 14 | Which Echo of each genre is rarer. | chapter owns it | Doc 15 §3.1.1 binds doc 14's tiers; no canon change. |

### 2.14 Run structure and invasions (canon §17)

| ID | Sources | Gap | Decision | Canon text (target) or ruling |
|---|---|---|---|---|
| G-161 | 11 gap 7; 12 G6; 14 gap 12 | Every Colophon has a shop, but `genre.unwritten` has none; counters must be in every genre's shop. | canon amendment | **§17.1**, Colophon: replace "a shop, an altar of 3 boons" with "a shop (none in `genre.unwritten`, §5.2), an altar of 3 boons". **§17.3a**: replace "in every genre's shop and loot pool." with "in every genre's loot pool and in every shop; `genre.unwritten`, which has no shops, also gets doc 11's counter caches." |
| G-162 | 12 G3; 14 gap 7 | Folio 1's genre for a secret Origin and for modes other than Standard and the seeded ones. | canon amendment | **§17.1**, `FOLIO_1_GENRE` value: replace the whole cell with "Unseeded runs (Standard, Abridged and every other mode without a seed): the host Erratum's Origin genre, or the core genre doc 12 maps a secret Origin to. Seeded runs (Daily, Weekly and any run with a typed seed): the first candidate drawn from `rng.lattice` with `folio_index = 0`." |
| G-163 | 14 gap 8 | Boss Folios in Marginalia. | canon amendment | **§17.1**, `BOSS_FOLIOS`: replace "5, 10, 15, 20, 25" with "5, 10, 15, 20, 25, and every Marginalia Folio `f` with `f % 5 == 0`". |
| G-164 | 12 G11; 14 gap 9 | Whether the exit before the Ur-Text still draws candidates. | already in canon | `LATTICE_CANDIDATES` draws "always exactly 3"; `FINAL_FOLIO` replaces the doors. |
| G-165 | 12 G4 | Meaning of "lattice edge weights". | chapter owns it | Edge × freshness (doc 12 §5.4). |
| G-166 | 12 G5 | Secret genres in the repeat rule. | chapter owns it | Doc 12: once per run before Marginalia. |
| G-167 | 12 G14 | Which counts Abridged's "loot quantity" multiplies. | canon amendment | **§17.1**, Abridged: replace "Kill XP and loot quantity per Folio are × 16667 Bp." with "Kill XP and loot quantity per Folio are × 16667 Bp, where loot quantity means worldgen chests and floor-loot spots; Reliquaries, Fortune Engines and vendor stock are unchanged." |
| G-168 | 14 gap 10 | Positions of Abridged Folios. | chapter owns it | Doc 14 owns `mode.abridged`. |
| G-169 | 14 gap 12 | Colophon details: sealed entrance, one heal per Erratum, desecration, boons. | chapter owns it | Doc 14 owns `room.colophon` with doc 12. |
| G-170 | 13 gap 5 | §17.3 says a champion has a "counter type" while §17.3a defines aspects. | canon amendment | **§17.3**, `rank.champion` notes: replace "one champion counter type (doc 05)" with "one champion aspect (§17.3a), countered by its matching counter type (doc 05)". |
| G-171 | 05 gap 9; 11 gap 18 | Doc 05's `counter.sunder` and `counter.anchor` are not named in §17.3a. | already in canon | §17.3a's sourcing rule covers "every counter type", and `counter.` is doc 05's namespace; doc 11 stocks all five. |
| G-172 | 05 gap 13; 13 gap 11; 09 G10 | Mount scaling depth, enemy riders, stealth rules for riders. | reject | Mounts are cut (DECISIONS 55, canon-1.8 §17.7); see C-30. |
| G-173 | 16 G3 | The Redaction trigger ignores `st.downed`. | canon amendment | **§17.9**: replace "If the invader reduces the last living Erratum to 0 HP" with "If the invader reduces the last Erratum that is alive and not downed to 0 HP". |
| G-174 | 15 gap 15 | Invaders earn no Codex XP, though the brief says all play does. | already in canon | §17.9's "Quires and cosmetic Red Ink marks only" stands (DECISIONS 32); invading pays no Codex XP, Ink or Reliquaries. |
| G-175 | 16 G17 | What party clients learn of the invader's inputs. | chapter owns it | Doc 16 §6.14 seals slot-4 commands from party clients until the invasion ends. |
| G-176 | 13 gap 20 | Who owns invader kit unlocks. | chapter owns it | Doc 14's `unlock.` nodes; `revisor` is always available. |
| G-177 | 01 G2; 13 gap 16; 18 gap 8 | "Redactor" names both the invader and the Volume II Editor; doc 01 asks for canon wording "a redactor, a junior Editor". | reject | Canon keeps "Redactor" for invaders. The Volume II boss is "the Censor" (doc 13 §9.2) and `patron.redacted` is "The Undersigned" (doc 18 §8.2), so no collision is left to fix in canon; see C-16. |
| G-178 | 01 G8 | The ghost mark travels through `input.attack`. | chapter owns it | Doc 17's `MarkTarget` intent implements §17.6's mark. |
| G-179 | 15 gap 16; 14 gap 15 | Which victories pay victory Ink. | chapter owns it | Only `boss.ur_editor` deaths (docs 14 and 15 agree). |
| G-180 | 14 gap 11 | Integer safety past `d_pow = 300`. | chapter owns it | `DIFF_CAP_BP` and `template_hp ≤ 400` HP. |
| G-181 | 13 gap 14 | No mini-boss slot in doc 14. | chapter owns it | Doc 12's optional `room.arena`, paid from the Folio budget. |
| G-182 | 13 gap 12 | The Last Word's `genre_shift` inside the Ur-Text. | chapter owns it | Intended: §5.5's Ur-Text override keys on the local genre. |
| G-183 | 13 gaps 15, 17, 22, 23 | Downed weight × 2500, `grid.cause.erased`, rest-Folio population, the `flier` tag. | chapter owns it | Resolved on disk in docs 03, 09, 12 and 16; doc 13 deletes the gap text. |

### 2.15 Events, namespaces and names (canon §18 to §20)

| ID | Sources | Gap | Decision | Canon text (target) or ruling |
|---|---|---|---|---|
| G-184 | 03 G4, G21, G28; 04 gap 2; 06 G3; 09 G5; 10 gap 14; 12 G9; 13 gaps 2, 24; 14 gap 6; 16 G10 | About 40 events that docs 04 to 16 emit are not in §18. | canon amendment | **§18**: replace "Doc 03 owns the event bus and the full payload schemas. These IDs are canonical:" with "Doc 03 owns the event bus and the full payload schemas. The IDs below are the canonical core. Doc 03 §14.3 registers every further `ev.` ID another document emits, with its payload; an event exists once doc 03 registers it, with no canon change." |
| G-185 | 01 G4 | DoT pulses of a status applied by an environmental hit do not carry `env`. | chapter owns it | Doc 03 adds the inheritance (C-32). |
| G-186 | 15 gap 9 | Codex kill XP reuses `rank_xp_bp`. | already in canon | No conflict. |
| G-187 | 06 G4 | Namespace for polymorph templates. | already in canon | `summon.poly.` inside doc 06's `summon.`. |
| G-188 | 15 gap 2 | Namespace for Scripts. | already in canon | `bundle.script.<name>` inside doc 15's `bundle.`. |
| G-189 | 15 gap 3 | The account Reliquary ID, since `reliq.` is doc 11's. | already in canon | `banner.reliquary.account` inside doc 15's `banner.`. |
| G-190 | 15 gap 4 | Seven `kpi.econ.*` IDs in doc 01's namespace. | chapter owns it | Doc 01 adds them to its Exports. |
| G-191 | 13 gap 13 | Namespace for quests. | already in canon | `npc.quest.*` inside doc 13's `npc.`. |
| G-192 | 07 gap 3 | Bare material tags in other chapters. | chapter owns it | They mean `mat.tag.<name>` (doc 07). |
| G-193 | 07 gap 15 | `part.` rows in doc 11. | already in canon | `part.` is doc 07's; doc 11 mirrors them as loot. |
| G-194 | 05 gap 27; 09 G19, G22; 10 gap 19 | IDs one chapter uses in another's namespace. | chapter owns it | Each owner adds or confirms them; a rule that names a missing ID does nothing until its owner adds it. |
| G-195 | 13 gap 18 | Companion IDs. | chapter owns it | Doc 13 §13.5: `npc.companion.<origin_suffix>` beasts and `npc.companion.<name>` named companions. |
| G-196 | 09 G2 | Awareness state IDs. | chapter owns it | Doc 09's six `aware.` states; docs 13, 17 and 18 now use them. |
| G-197 | 10 gap 5; 14 gap 16 | "Codex" means both the pass and the item encyclopedia; "Recollection" means both an Echo trial and an identification aid. | chapter owns it | The encyclopedia is the Index and doc 10's aid is Cross-Reference; doc 15 still says "Item Codex" (C-29). |
| G-198 | 17 G27 | "Scribe" (the language model) and "Scribe's Mercy". | chapter owns it | Doc 17: the model's settings tab is "Generated text". |

### 2.16 Chapter-internal rulings with no canon impact

| ID | Sources | Gap | Decision | Canon text (target) or ruling |
|---|---|---|---|---|
| G-199 | 01 G5, G6 | Per-Folio telemetry aggregates and telemetry consent. | chapter owns it | Docs 02 and 16 add the aggregates to the opt-in run summary; consent is doc 16's opt-in channel (C-14). |
| G-200 | 01 G7; 17 G25 | Gamepad binding for `input.cast_cancel`. | chapter owns it | Resolved on disk: B in `input.ctx.casting` (doc 17 §12.4). |
| G-201 | 17 G12, G13, G14, G18, G19, G23 | Look filter without magnetism, ping kinds, game speed, shimmer opacity, Mercy accumulation and death counting, fonts. | chapter owns it | Doc 17; doc 14 adopts the ranked rule for game speeds below 10000. |
| G-202 | 18 gap 6 | Rewriting voiced barks. | chapter owns it | Spoken barks are rewritable text bubbles; voiced lines never are. |
| G-203 | 09 G7, G8, G12, G18, G20, G21 | Sound attenuation, sneak tiers, ghost door noises, stealth balance ratings, Ur-Text rooms, surface noise sources. | chapter owns it | Doc 09. Doc 07 decides whether to add a snow-layer block; until it does, `noise.surface.snow` stays unused. |
| G-204 | 07 gaps 7, 10, 11, 12, 13, 14, 16, 19, 20 | Meal buffs, archetype IDs, doc 12 pool IDs, gadget failure per deploy, ore vein count, torch placement, seized devices, door HP, dirt. | chapter owns it | Doc 07. Resolved on disk except the torch (C-25). |
| G-205 | 08 gaps 6, 11, 16, 17, 18, 19 | Uncursed cure, Alchemy perks, `ev.potion_brewed` payload, `envenom`, gas in ml, red ink by mouth. | chapter owns it | Doc 08; `envenom` is C-19 and gas in ml is C-22. |
| G-206 | 06 G1, G5, G7, G15 | Material term for spells, staff, wand and tome exports, IVAN's prayer timer draw, canon-1.4 building. | chapter owns it | Doc 06. Doc 05 now exports the three archetypes. |
| G-207 | 10 gaps 20, 22 | Prayer uncursing, witness timing. | chapter owns it | Doc 10. |
| G-208 | 14 gaps 20, 21 | Version-mismatch end reason; alignment with doc 12's door choice and Folio types. | chapter owns it | `run.end.abandoned_version`; doc 14 §3.6 carries both commit paths. |
| G-209 | 16 G6, G8, G13, G14, G19, G22 | Rewind and `spawn_tick`, ghost entities, `flag.net.*`, bandwidth split, `run_id`, `heard` entries. | chapter owns it | Doc 16. Resolved on disk except G6 and G19, which stand as doc 16's rules. |
| G-210 | 01 G3; 17 G2, G21; 18 gap 7 | The Margin plot and the Prologue's terrain-mining room. | chapter owns it | Doc 14 has replaced the plot with the trophy alcove; the Prologue room L3 still mines terrain (C-03). |
| G-211 | 17 G4 | Scribe's Mercy blocks achievements and Grade unlocks in doc 14. | chapter owns it | Doc 17 owns `access.scribes_mercy`; doc 14 changes (C-04). |
| G-212 | 17 G5 | Button bits 26 to 31. | chapter owns it | C-10. |
| G-213 | 17 G6 | FOV range. | chapter owns it | C-11. |
| G-214 | 17 G7 | Launch locales. | chapter owns it | DECISIONS 40 sets 15; doc 18 lists them; doc 17 follows (C-17). |
| G-215 | 17 G10 | HUD light channels. | chapter owns it | C-09. |
| G-216 | 17 G15, G24 | Wheel release for channel and charge spells; the item uid in the cast command. | chapter owns it | C-28. |
| G-217 | 18 gap 9 | Texture layer size against canon §4.4's 64 texels per meter. | chapter owns it | The canon governs; doc 02 changes (C-12). |
| G-218 | 05 gap 28; 16 G9 | Projectile normal step on the spawn tick; downed crawl speed. | chapter owns it | Docs 03 and 05 govern the step (C-08); crawl is 800 mm/s in all three chapters. |
| G-219 | 16 G15 | Host migration wording in doc 14. | chapter owns it | Doc 16 owns migration (C-06). |
| G-220 | 16 G18 | How invaders find sessions. | chapter owns it | Doc 16's `svc.lobby` beacons (C-13). |
| G-221 | 16 G20 | Hex forms in doc 14's ranked submission example. | chapter owns it | C-07. |
| G-222 | 16 G21 | Telemetry for minor-flagged accounts. | chapter owns it | The stricter doc 16 rule applies (C-14). |
| G-223 | 11 gap 14 | The haggle intent. | chapter owns it | `ShopHaggle { shop }` (C-15). |
| G-224 | 11 gap 17 | `run.boon.deep_pockets` on "every Lucre gain". | chapter owns it | Piles only (C-05). |
| G-225 | 11 gap 1; 11 gap 12 | Canon-1.2 items now canon; staff, wand and tome exports. | chapter owns it | Resolved on disk. |
| G-226 | 12 G12 | Editor squad placement. | chapter owns it | Closed in doc 12 and doc 13. |
| G-227 | 15 gap 10 | `crate.econ`. | chapter owns it | Resolved on disk: doc 02 lists `crate.econ` = `palimpsest_econ`. |

---

## 3. How the decisions add up

| Decision | Count |
|---|---|
| canon amendment | 79 |
| chapter owns it | 122 |
| already in canon | 24 |
| reject | 2 |
| **Total** | **227** |

The amendment count counts rows; several rows edit two sections, and three add acceptance tests. The two rejections are G-172 (three chapters' mount rulings, retired by DECISIONS 55) and G-177 (doc 01's canon wording for "Redactor"). Of the 122 chapter-owned rows, 23 point to a conflict in §4 that a chapter must fix; the rest confirm a rule the chapter already states or record a gap that is resolved on disk.

---

## 4. Interface assumptions that conflict between chapters

Each row was checked against the chapters on disk. "Change" names the chapter that must change and the exact change.

| ID | Chapters | Conflict | Change |
|---|---|---|---|
| C-01 | 14 vs 04, 16 | Doc 14 §3.10 and its gap 13 read "present" as alive, so a ghost gains no XP; doc 04 §6.2 includes downed members and ghosts, and doc 16 imports doc 04's rule. | **Doc 14** adopts amended canon §8 (G-072): ghosts and downed members share kill XP; delete gap 13's reading. **Doc 16** deletes G16. |
| C-02 | 14 vs 05, 08, 11 | Doc 14 still names IDs no owner exports: 16 `item.potion.*` mentions, `arch.ranged.sidearm`, `.longarm`, `.heavy_auto`, `.flamer`, `.hand_crossbow`, `.sling`, `arch.melee.hafted_mace` and the other role names, `ammo.fuel`, `ammo.stone`, `item.throwable.*`, `item.container.flask`. | **Doc 14** §2.4 and its Interface rows for docs 05, 08 and 11 name only owner IDs, using doc 11's former-name table and doc 05 §5.6 (hand crossbow → `arch.ranged.pistol`, sling stones → one `arch.ranged.thrown` stack, longarm ammunition `ammo.rifle`, flamer at most 300 `ammo.cell`). Then **doc 11** deletes the doc-14 rows of its former-name table. |
| C-03 | 14 vs canon §4.3, docs 01, 17 | Doc 14's Prologue room L3 has the player "mine 3 blocks, place 2 blocks to bridge a gap"; terrain is fixed and only `cat.placeable` items are placed. | **Doc 14** rewrites L3: the player harvests a resource node with the `item.tool.pick` lying there and places a bridge-span placeable across the gap. Its acceptance test 37 keeps the 3,000 mm drop. |
| C-04 | 14 vs 17 | Doc 14 §10 disables the 16 achievements marked † under `access.scribes_mercy`, and §7.4 says Mercy runs never unlock Grades; doc 17, owner of `access.scribes_mercy`, says Mercy never blocks achievements. | **Doc 14** removes every † mark and its sentence in §10, and removes "`access.scribes_mercy` active (doc 17)," from §7.4's Grade rule. Ranked leaderboard entries still exclude Mercy runs (§9, unchanged). |
| C-05 | 14 vs 11 | Doc 14 §3.9: `run.boon.deep_pockets` multiplies "every Lucre gain"; doc 11 §9.1 applies it to fresh piles only, so the buy floor stays above the sell cap. | **Doc 14**: "every fresh Lucre pile the bearer picks up × 12500 (doc 11 §9.1); sales and dropped piles are excluded". |
| C-06 | 14 vs 16 | Doc 14's Co-op overlay says host migration "resumes from the latest Folio-entry save copy … the Folio in progress restarts from its entry"; doc 16 §9.3 continues the Folio on hot and warm migrations. | **Doc 14**: "Host migration follows doc 16 §9.3: hot and warm migrations continue the Folio; only a cold migration restores the Folio-entry save and restarts the Folio." |
| C-07 | 14 vs 16 | Doc 14's ranked submission example writes `run_seed` in uppercase hex (`0xC6F2E12DDCA712AB`) and 32-byte values with `0x`; doc 16 §11.1 accepts only `0x` plus 16 lowercase hex digits for `u64` and 64 lowercase hex digits without prefix for 32-byte values. | **Doc 14** rewrites the example in those forms. |
| C-08 | 16 vs 03, 05 | Doc 16 §8 item 2: a player projectile "continues at `t + 1` against current poses" after its catch-up steps; docs 03 §5.19 and 05 §2.6 give it a normal step against the poses of `t` on its spawn tick. Doc 16 G9 also cites a 600 mm/s crawl that no chapter still uses. | **Doc 16** §8 item 2: "…then takes its normal step against the poses of `t`, and one step per tick from `t + 1` (doc 03 §5.19)". Delete G9. |
| C-09 | 16 vs 17 | Doc 16's snapshot `HUD` section carries only `light_at`; doc 17's stealth meter needs both light channels. | **Doc 16** adds one byte beside `light_at`: low nibble `light_block`, high nibble `light_sky` (the `ChunkView.light` layout). |
| C-10 | 02, 16 vs 17, 06 | Doc 02 §5.5 marks button bits 26–31 "reserved, must be 0" and doc 16 §5.8's sanitizer replaces any command with them set; doc 17 §12.2 assigns `ACTION` (26–28) and `WHEEL_ENTRY` (29–31), and doc 06 §3.2 already uses them. | **Doc 02** §5.5 lists `ACTION` (bits 26–28) and `WHEEL_ENTRY` (bits 29–31) and drops "reserved". **Doc 16** §5.8 drops the reserved-bit test and instead replaces a command whose `ACTION` value is 6 or 7. |
| C-11 | 02 vs 17 | Doc 02 §8.4: FOV "setting 70–110°"; doc 17 §8.4: 60–120°, default 90°. | **Doc 02** reads "setting 60–120°". Doc 17 owns comfort settings. |
| C-12 | 02 vs canon §4.4, 18 | Doc 02 §8.3: texture layers "at 32 × 32 px … (1,392 bytes per layer)"; canon §4.4 sets 64 texels per meter and doc 18 §10.2 ships 64 × 64 layers. | **Doc 02** §8.3: 64 × 64 px layers with full mip chains, 5,488 bytes per layer, world-space triplanar at 64 texels per meter. |
| C-13 | 02 vs 16 | Doc 02 §14: invaders "find open sessions through a separate lobby search"; doc 16 uses `svc.lobby` beacons. | **Doc 02**: "invaders find open sessions through doc 16's `svc.lobby` beacon search". |
| C-14 | 02 vs 01, 16 | Doc 02 §17.4's run summary has only mode, Folio reached, end cause and party size, while doc 01's KPIs need per-Folio aggregates; doc 02 bars telemetry only below the child-consent age, while doc 16 §14 bars every minor-flagged account. | **Doc 02** §17.4 adds per Folio: genre, entry and exit ticks, offered and committed doors, the maximum equipped `C` per slot at entry, a Disconnected-weapon flag per slot, committed Rewrites, and NPC kills split into systemic and other; and bars every minor-flagged account. **Doc 16** carries the same fields in its opt-in summary. |
| C-15 | 02 vs 11, 17 | Doc 02's `IntentKind` lacks doc 11's haggle intent and doc 17's 11 kinds; doc 11 names the haggle `ShopHaggle { shop }`, doc 17 §9.2 `Haggle { shop }`. | **Doc 02** adds `ShopHaggle { shop }` (matching `ShopBuy` and `ShopSell`) and doc 17's other kinds (`SetScribesMercy`, `MapPing`, `AssignWheelEntry`, `AssignAbilitySlot`, `QuickbarAssign`, `SetFocusLock`, `Examine`, `SetCastItemTarget`, `SetBreathHold`, `Vocalize`). **Doc 17** renames `Haggle { shop }` to `ShopHaggle { shop }`. |
| C-16 | 01, 03 vs 13, 18 | Doc 01 names `boss.editor_v2` "the Redactor" (§1.3 premise, the Editors diagram, Q-17, Interface row for doc 18) and doc 03 calls its arena darkness the "Redactor `blackout`"; doc 13 §9.2 and doc 18 §8.2 name that boss "the Censor". | **Doc 01**: "the Censor" and "the Censor's office"; invaders stay lowercase "redactors". **Doc 03**: "the Censor's `blackout`". |
| C-17 | 17 vs 18 | Doc 17 §14 lists 12 launch locales; doc 18 §9.11 lists 15 (DECISIONS 40), adding ru-RU, uk-UA and tr-TR. | **Doc 17** §14 lists the 15 and deletes G7. |
| C-18 | 17 vs 06 | Doc 17 §10.7's exact prayer odds use `r1 = range(1,800,000)` and a petition condition `timer ≤ 36000`; doc 06 §16.3 to §16.4 use `range(180_000)` and `prayer_timer <= 7200`. | **Doc 17** §10.7 uses doc 06's values. |
| C-19 | 04 vs 08 | Doc 04's Shade talent `envenom` ("coat a blade with a potion so the next 5 hits apply it at potency × 5000") restates the coat every character has (doc 08 §10.3). | **Doc 04** §7.2: "`envenom`: your coats get +5 charges, and the coat's buildup factor 5000 becomes 10000." |
| C-20 | 04 vs 09 | Doc 04's Interface row for doc 09 asks for disguise Deceive checks on `rng.check`, a passive-detection cadence of 60 with `gumshoe`, and `cat.placeable` disarmed traps; doc 09 draws the bluff on `rng.stealth` seq 1006 (G-023) and already meets the other two. | **Doc 04**: the row reads "disguise Deceive checks on `rng.stealth` seq 1006"; delete the two requests doc 09 has met. |
| C-21 | 04 vs 13 | Doc 04's gap 12 cites 31.2 standard-kill equivalents per Folio and a rest Folio at every position 4; doc 13 §11.2 now supplies 37.3 and never rests at position 4. | **Doc 04** cites 37.3 kill equivalents and 45.5 units per Folio, and re-runs §6.3 on doc 13's current plan. |
| C-22 | 13 vs 08 | Doc 13's `elite.miasmic` emitted "1000 ml of `gas.miasma`", which is concentration 10, below the 500 contact threshold. | Resolved on disk while this triage ran: doc 13 now emits one saturated cell through `emit_gas_volume(cell, gas.miasma, 1)`. **Doc 08** deletes its gap 18. |
| C-23 | 13 vs 07 | Doc 13 names `block.dirt` (the sand wyrm row and its Interface row for doc 07); doc 07's block is `block.soil`. | **Doc 13** writes `block.soil`. |
| C-24 | 13 vs 03, 07 | Doc 13 §10.6 asks doc 03 for a `placed_by_errata` bit per block and uses it in pseudocode and an invariant; doc 03 G20 serves the question from doc 07's `placed_meta`. | **Doc 13** calls doc 03's `is_errata_placed(c)` and drops the request for a separate bit. |
| C-25 | 03 vs canon §4.3, 07 | Doc 03 §13.3: "Errata place only `cat.placeable` items and `item.tool.torch`". | **Doc 03** drops "and `item.tool.torch`"; a held torch becomes the placeable `block.torch` through doc 07's `recipe.hand.wall_torch`. |
| C-26 | 11 vs 08 | Doc 11 §3.4 keeps `item.face.respirator` as a `cat.accessory` record that makes "gases breathable"; doc 08 puts respirator mechanics and filters in `apparatus.respirator.*` and asks for an alias. | **Doc 11** replaces its record with a loot record on `apparatus.respirator.mask` (doc 08). No alias is needed because the ID never shipped. **Doc 08** drops "aliased". |
| C-27 | 11 vs 05, canon §5.9 | Doc 11's Somnambule row: projectiles "pass through blocks after 10000 mm of flight"; doc 05 §7.3 never lets anything through fixed terrain. | **Doc 11**, per G-037: "after 10000 mm of flight, projectiles (spell projectiles included) pass through destructible and non-opaque blocks without spending penetration". |
| C-28 | 06 vs 17 | Doc 06 §3.2 casts a wheel entry "on release of the wheel button in the same input command", and §5.3 carries a lore essence's item uid "in the cast command"; a channel or charge spell released in one tick ends at once, and the command has no `u64` field. | **Doc 06** §3.2: instant and cast-time entries cast on release; channel and charge entries only set the active spell, and the player holds `input.cast`. §5.3: the item target is the uid set by the preceding `SetCastItemTarget { uid }` intent (doc 17 §6.5). |
| C-29 | 15 vs 10, 14 | Doc 15 §5.5 and the `pass.xp.codex_entry` row say "Item Codex"; docs 10 and 14 name the encyclopedia the Index, to keep "Codex" for the pass. | **Doc 15** writes "Index". |
| C-30 | 01, 04, 05, 09, 10, 11, 12, 13, 17 vs canon-1.8 §17.7 | Retired mount IDs (`st.mounted`, `slot.mount`, `cat.mount`, `arch.mount.*`, `ev.mounted`, `ev.dismounted`, `room.no_mount`) still appear: doc 01 ×1, 04 ×2, 05 ×16, 09 ×3, 10 ×3, 11 ×7, 12 ×9, 13 ×6, 17 ×4. | Each chapter removes mount mechanics and IDs and keeps only sentences that record the retirement. **Doc 05** §10.1 drops attack factor 3 (mounted charge) and its mount archetypes; **doc 13** drops cavalry mounts and enemy riders; **doc 09** drops its mounted rules (G10); **doc 12** drops `room.no_mount`. |
| C-31 | 04, 05, 06, 07, 09, 10, 11, 12, 14, 17 vs canon-1.10 §4.3, §13.9 | Item durability (wear, breaking, repair kits, durability UI, `curse.brittle`'s durability effect, lockpick snaps and hacking-deck wear, the durability-loss hook of `run.marg.brittle_kit`) is still referenced: doc 04 ×4, 05 ×2, 06 ×1, 07 ×23, 09 ×3, 10 ×2, 11 ×19, 12 ×2, 14 ×4, 17 ×5. | Each owner removes item wear and breaking and replaces or deletes the mechanic that used it (doc 10 `curse.brittle`, doc 14 `run.marg.brittle_kit`, doc 09 lockpick and hacking-deck wear, doc 07 repair). Block HP of placeables and destructible blocks stays. |
| C-32 | 01 vs 03 | Doc 01's systemic-kill KPI assumes DoT pulses inherit `env` from the hit or surface that applied the status; doc 03's `DmgFlags` sets `env` only on surface DoT rows and environmental hits. | **Doc 03**: a DoT status applied by an `env`-flagged hit or a surface contact sets `env` on each of its pulses. |

### 4.1 Gap text that is stale on disk

These gaps were real when written and are now resolved; each chapter deletes or closes the text.

- Doc 03 G12 (encodings agree with doc 02).
- Doc 06 G5, doc 10 gap 11 and doc 11 gap 12 (doc 05 exports `arch.ranged.staff`, `arch.ranged.wand`, `arch.offhand.tome`).
- Doc 07 gaps 11, 13, 16, 20 (doc 12 uses canon §5.9 materials and places `6 + floor_div(d, 5)` veins; seized devices and dirt are closed).
- Doc 09 G15, G16 (doc 03 has `HeardNoise.req`, `grid.cause.erased`, `grid.gait.crawl`, `DmgFlags` bit 13).
- Doc 10 gaps 1, 2, 16, 17, 18, 20 (doc 11 uses `alch.potion.*`, `value_vendor` and `appr.svc.*`; doc 06 uncurses the lowest-uid item).
- Doc 13 gaps 15, 17, 22, 23 and the request about doc 12's "Redactor's archive".
- Doc 14 gaps 16, 17, 19 and the `margin.plot` building-plot text (now the trophy alcove).
- Doc 15 gaps 10, 12.
- Doc 16 G9, G22.
- Doc 17 G8, G9, G20, G25, G26.
- Doc 01 G7.

---

## 5. Applying the amendments

### 5.1 Changelog entry

Add at the top of canon §23, and set the header to canon-1.11:

```
### canon-1.11 (2026-10-11)

Applied after the canon gap triage (audit/canon_gaps_triage.md).

- Conventions: atan2 convention and yaw_from_dir, mod ID namespace x_<mod_id>, JSON integer bound, one-decimal display of small per-second rates.
- Sessions: named state boundaries for state_hash and snapshots, intents in the replay log, presentation_hash split from content_hash, run_seed carried from commit, saves deferred during an invasion, light overrides in every light read.
- RNG: range_incl computed in i128, seq allocation, object-keyed draws, rng.check and rng.run, labeled sub-streams, offer generators, explosion draws on sequential streams, account and presentation generators.
- World: liquid units and gas concentration, the fall-damage and move-speed chains, block.tag.destructible and fixed terrain stopping projectiles.
- Rules: Somnambule limited to destructible blocks, "Nobles" for Crowns, supernatural alchemy dependency, potion failure and Unmoored, Shock of the New ranks, seam local_genre.
- Progression: skill XP normalized before party scaling, the 45-unit XP anchor, ghosts and pacification in character XP, Errata limb protection and added-part donors.
- Damage: attack factors and manufacturer traits before step 1, st.taunted, fae iron, shield AR on blocks, the 6b split, Scribe's Mercy step 6c, ward profiles, consumable and takedown entry points, blunt poise for mixed hits, two hit batches.
- Statuses: disruptor duration, downed rules in doc 16, st.marked wording, the Erasure hunter's rank, invaders never erase, substituted commands.
- Items and run: gas contact 500, tall grass, initial population spawn light, trap categories, ward capacity chain, two extra-hand slots, ENH_CAP, block HP tables, diff_hp_bp, illuminated rolls, Lucre per Erratum, welcome Ink, kit ownership, Unwritten Colophons, FOLIO_1_GENRE, Marginalia boss Folios, Abridged loot, champion aspects, Redaction while downed, events registered by doc 03.
```

### 5.2 New acceptance tests for canon §21

All three vectors were computed from the canon §3.1 reference with an independent implementation that reproduces canon tests 3 and 6.

45. **range_incl extremes.** `Rng::from_seed64(42).range_incl(i64::MIN, i64::MAX - 1)` returns `5797906573132458142` without overflow. Three calls of `range_incl(-5, 5)` on a fresh `Rng::from_seed64(42)` return `3, -2, 5`.
46. **Yaw from direction.** `yaw_from_dir(0, -1 m)` is within 1 Bam of 0, and `yaw_from_dir(-1 m, 0)` is within 1 Bam of 16384, the yaw whose `fwd` is −X.
47. **Sub-stream.** For run seed `0x0123456789ABCDEF`, `derive(stream_seed(run, "rng.worldgen", 2), fnv1a32("genre.space_opera")) = 0x1c35d41d581f5eaa`, which equals doc 12 §7.2's `gen_seed` example.

### 5.3 Order of work

1. Apply the canon amendments in section order and add §5.1 and §5.2 (canon-1.11).
2. Chapter fixers apply §4 in this order: retired IDs first (C-30, C-31), then the doc 14 rows (C-01 to C-07), then docs 02, 16 and 17 (C-08 to C-18), then the rest.
3. Each chapter deletes the stale gap text of §4.1 and moves its header to canon-1.11.
