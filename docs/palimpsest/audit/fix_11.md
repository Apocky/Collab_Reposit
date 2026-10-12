# Fix log: chapter 11 (Items, Loot, Economy)

Fixer: chapter 11 fixer, 2026-10-11. Target: canon-1.11. Only `spec/11_ITEMS_LOOT_ECONOMY.md` is edited.

## Plan and authority notes

- DECISIONS 90 (product owner) overrules triage Q9: the Mythic Fantasy Lucre skin stays "Crowns". Doc 11 names no Lucre skin, so nothing changes here; "Nobles" is not introduced.
- Worked example (§20) is preserved by replacing the two durability affixes in place with affixes of the same weight, Cats, eligibility and ID sort position (`affix.engineered` for `affix.durability`, `affix.self_injector` for `affix.self_repair`), and by keeping `perk.tempered` and `perk.hot_loads` IDs with new non-durability effects. Checked with a script: eligible count 46, total 1895, draw 1830 still lands on the replacement of `self_repair` (1825..=1844).
- Lint "might" hits (3) are the IDs `item.ring.might` and canon's `attr.might`, not hedges.

## Batches applied

(appended below as each batch lands)

### Batch 1 (header, Imports, §1, §2) — done

- Header: Conforms to canon-1.11.
- Imports: canon §1.1 mod IDs (G-008), §3.2 sub-streams and §3.5 offer generators (G-027, G-030), §4.3 `block.tag.destructible` and no durability (canon-1.10, G-037), §13 pact curses and `ENH_CAP` (canon-1.9, G-131), §14 Lucre per Erratum and dropped piles (G-148, DECISIONS 83), §17 mounts replaced by "§17.7 retired IDs" (C-30); doc 02 intents add requested `ShopHaggle` (C-15) and `DropLucre`; doc 05 row drops `arch.mount.*` and §17.1 (C-30); doc 08 adds §4.3 `apparatus.respirator.*` (C-26); doc 10 adds §7.8 pacts and `CursedCapsule` (DECISIONS 60).
- §1 principle 8 cites Canon §1.1 instead of doc 02 G5 (G-008). New principle 9 "No wear" (DECISIONS 65, C-31).
- §2.1 `rarity_eligible` drops mount tokens (C-30).
- §2.2 drops `durability` from the canon field list and from the example JSON (C-31); adds `zaps_total` (doc 10 §7.8 interface request) and `dropped_by` (Lucre drops, Q3); `affix.self_repair` renamed `affix.self_injector` in the affix text and example.
- §2.3 step 12: cursed capsule calls `roll_curse` with `CursedCapsule` (DECISIONS 60, doc 10 §7.8); step 14 and the code comment drop durability.
- §2.4: cursed capsule also has OMEN set.

### Batch 2 (§3) — done

- §3.1: `cat.mount` row deleted (C-30).
- §3.3: heading, owner sentence and table drop mount tokens (C-30); the ward capacity chain now cites Canon §13.2 with "Contradiction potency" and "no material term" (G-129).
- §3.4: respirator paragraph no longer refers to a replaced single respirator; doc 08's three `apparatus.respirator.*` loot records are the only respirator records (C-26, G-143).
- §3.5: text malign share aligned with doc 10 §8 (owner of the rule; doc 10 gap 25, notes_ident item 11): `item.text.amnesia` 3000 → 1000, `item.text.appraisal` 500 → 1000, `item.text.mending` 500 → 800, the other twelve texts 500 → 600 (Σ 10000, harmful 1000; checked by script). `item.text.demolition` names `block.tag.destructible`. `item.tool.toolkit` use: "crafting tool tier and gadgets (doc 07)" instead of repair (C-31).
- §3.9: `slot.mount` row deleted (C-30); extra-hand rows cite Canon §13.2/§13.3 instead of Canon gap 19 (G-130).
- §3.10: `cat.block` row deleted (task; lint retired ID); `item.face.respirator` alias row deleted (C-26: never shipped, no alias needed); one plain sentence records the mount retirement. `item.relic.lattice_lens` alias kept (G-147; doc 14 still mentions it in a history gap).

### Batch 3 (§4.3, §5) — done

- §4.3: heading and table drop the runner, hover and glider mount-skin columns; the beast/vehicle mount paragraph is deleted (C-30). A missing blank line before the heading was added.
- §5.1: manufacturers no longer stamp vehicle mount tokens; RH class no longer includes them (C-30); tendency letter U (durability) removed from the legend (C-31); "× N" gimmick cites Canon §10.4 instead of Canon gap 3 (G-084).
- §5.2 durability removal (C-31, canon-1.10 §5.9): U tendencies dropped from Deepanvil, Runecourt, Vesper & Sons, Outer Choir, Chromaw and Brassworth. Rows left with only a penalty got a small replacement bonus: Dunwold `T−1000 S−300` (gimmick loses "durability loss × 5000"), Lastditch `D−200 W−1000`, Thornwood `O+1000 S−200`. Canon §5.9 traits copied: Rustmaw "magazine × 20000, reload time × 13000"; Hephaesteum "immune to `st.disarmed` and `st.corroded`". Glass Cobbler (doc 11's own third row) redesigned without durability: Glass Edge +2500 Bp (armor AR × 12000) only while the bearer's HP is at least 5000 Bp of Max HP. Neon Saint's charged-shot pierce limited to destructible or non-opaque blocks (Canon §4.3, G-037).
- §5.2 Somnambule (C-27, G-037, DECISIONS 88): "after 10000 mm of flight, projectiles (spell projectiles included) pass through destructible and non-opaque blocks without spending penetration; fixed terrain still stops them (Canon §4.3)".
- Identity lines rewritten for Deepanvil, Dunwold, Rustmaw, Glass Cobbler, Hephaesteum and Somnambule so none implies wear or passing through fixed walls.
- §5.3: beast-skinned mount token row deleted (C-30).

### Batch 4 (§6, §7, §8, §16.4 salvage) — done

- §6.1: Cats letter M (mount token) removed (C-30); worked value example renamed to `affix.self_injector`.
- §6.2 (116 → 113 affixes): `affix.mount_vitality`, `affix.mount_endurance`, `affix.mount_barding` deleted (C-30). `affix.durability` replaced by `affix.engineered` (salvage yield × (10000 + X), same Cats, class, tiers and weight 60; §16.4 now applies it). `affix.self_repair` replaced by `affix.self_injector` (tech, weight 20, same descending tick tiers: every X ticks heals 100 Bp of Max HP × potency, Canon §10.9; injectors on one bearer do not add). Re-run of the eligibility script: 113 affixes; the §20 SMG still has 46 eligible affixes, Σ 1895; draw 1830 lands on `affix.self_injector` (1825..=1844), 926 on `affix.infuse_fire`, 397 on `affix.dmg_vs_beast`, so the worked example and acceptance test 2 are unchanged (C-31).
- §7.1 cites DECISIONS 68 (Destiny-style switchable perk columns by gear tier).
- §7.2: `perk.tempered` is now "+[4] armor poise while this is the active weapon"; `perk.hot_loads` costs "attack noise +6 dB" instead of extra durability loss (C-31). IDs, columns, families and weights unchanged, so §7.4 pools and the §20 perk draws are unchanged.
- §8.1: Group X no longer lists mount tokens (C-30); `perk.leg.worldbreaker` names `block.tag.destructible`.
- §16.4: salvage yield multiplied by `10000 + X` of `affix.engineered`.

### Batch 5 (§9, §10) — done

- §9.1: one sentence states that Abridged also scales kill-drop lines, beside Canon §17.1's chests and floor-loot spots (G-167); recorded as the one open canon gap (new Canon gaps 1). Lucre find adds `curse.pact.miser` as a fresh-pile factor after the boon (doc 10 §7.8 interface request; fresh piles only, so dropped piles cannot loop); sales receive neither find, boon nor pact.
- §9.5: champion "beast role: mount token 1500" line deleted and the "beast role only" condition dropped (C-30).
- §9.6: plain chest "mount token 150" line deleted (C-30).
- §9.10: heading drops mounts; the mount-token bullet is deleted (C-30); counter caches cite Canon §17.3a instead of Canon gap 7 (G-161).
- §10: the Reliquary consumable bundle's 500 Bp mount token becomes one `item.tool.rewrite_quill` added to the bundle; the bundle still draws exactly 4 values (C-30).

### Batch 6 (§11 Fortune Engines) — done

- §11.2 (DECISIONS 60, Canon §13.6, doc 10 §7.8 interface request): `CURSED_FACE = 4`. Face 4 of every Engine (brass Rare, silver Epic, gilt Legendary) is the cursed capsule: generated with `roll_curse` source `CursedCapsule` (3 draws in `loot.buc`), always cursed with a pact, sealed with `RARITY` and `OMEN` set. The panel shows the cursed share `w[4] / Σ w` (brass 900 Bp, silver and variants 1000 Bp, gilt 1000 Bp; 0 while pity is armed); it counts as a non-top capsule for pity. Both face-generation calls pass `i == CURSED_FACE`. Compliance bullet: the cursed capsule is shown before payment, never hidden. Engine weights, prices, pity and acceptance test 20 numbers are unchanged.

### Batch 7 (§12 shops and services) — done

- §12.1: stablemaster removed from the vendor draw (weights now Σ 17), `shop.stablemaster` row deleted, wanderer's mount-token line deleted, Stablemaster column removed from the vendor-name table (C-30; Canon §17.7 retires `shop.stablemaster` and `npc.vendor.stablemaster`). No acceptance test depended on vendor draw results.
- §12.3: "Repair to max durability" (C-31) and "Mount mending" (C-30) service rows deleted.

### Batch 8 (§13 to §18) — done

- §13.3: "× N" gimmicks cite Canon §10.4 (G-084).
- §14.1: Backpack cites DECISIONS 66; purse is "per Erratum (Canon §14)" (G-148).
- §15 (DECISIONS 67): cost scales with item power through doc 10's `value_vendor` (doc 10 interface request: value-scaled prices use `value_vendor`); example now says "an identified item of value 1382" (still 483). Offers: exactly 3 stored offers from `offer_rng(slot, item uid, "rewrite")`; offer k takes `cand[k % n]` (default first, others ascending ID) and its own replacement generator `Rng::from_seed64(o.next_u64())` (3 draws); commit advances the stream once; the Quill uses offer 0's manufacturer. "Persists" drops "durability ratio" (C-31).
- §16.1: cap cites Canon §13.7 `ENH_CAP` (G-131).
- §17 (Q3, G-148, DECISIONS 83): new "Lucre between players" bullet. `DropLucre { amount }` (requested from doc 02) makes a Public, non-fresh pile with `dropped_by`; refused unless `1 <= amount <= purse` and for ghosts, downed Errata and invaders; the dropper's auto-pickup skips it; trades and `DropItemFor` never carry Lucre.
- §18: ward generators use material factor 10000 in Item Power, matching Canon §13.2's "no material term" (G-129).

### Batch 9 (§20, Exports, Interface assumptions, Former names) — done

- §20: affix 1 row reads `affix.self_injector` … "heals 100 Bp of Max HP every 818 ticks" (draws unchanged).
- Exports: `cat.` 28 → 27 (no `cat.mount`), `slot.` 24 → 23 (no `slot.mount`), `affix.` 116 → 113 (`affix.engineered`, `affix.self_injector`; mount affixes removed), `loot.` tables 75 → 74 (no `loot.shop.stablemaster`), `shop.` 11 → 10 (no `shop.stablemaster`). Script check: every row's declared count equals its listed IDs; no duplicates. Data constants add `CURSED_FACE = 4`; intent requests add `DropLucre { amount }`.
- Interface assumptions: 02 cites triage C-15 and asks for `DropLucre { amount: i64 }`, mod IDs cite Canon §1.1; 03 names `block.tag.destructible`; 05 drops mount stats and stale gap 12 (§4.1); 07 "mass (items have no durability)"; 08 adds respirator mechanics (C-26); 10 adds pacts, `CursedCapsule`, `curse.pact.miser` and the 3000/1000 malign shares; 13 drops "mount" from its old table list; 14 kits name only owner IDs; 16 trades and `DropItemFor` refuse `cat.currency`; 17/18 add the cursed-capsule mark and the three Rewrite offers.
- Former names (C-02): doc 14 now uses owner IDs only (grep: no `item.potion.*`, `item.container.*` or retired `item.throwable.*` in doc 14), so the five doc-14 rows are deleted and the intro names docs 02, 04 and 13.

### Batch 10 (Canon gaps, Acceptance tests, Invariants, final touches) — done

- Canon gaps: the 21 canon-1.4 gaps are all closed (one paragraph maps each to its triage row: adopted G-131, G-084, G-025, G-030, G-026, G-161, G-148, G-027, G-134, G-130; already canon G-171; stale gap 12 per §4.1; chapter rules G-058, G-062, G-136, G-138, G-146, G-147, G-223, G-224). One real open item kept: new gap 1, Abridged kill drops (Canon §17.1 / G-167 names only chests and floor-loot spots; doc 11 also scales kill-drop lines).
- Acceptance tests: 12 renamed to `affix.self_injector`; 20 adds cursed shares; 23 "identified item of value 1382"; 33 texts 1000 of 10000 (doc 10 §8). New 41 cursed capsule, 42 Lucre between players, 43 Rewrite offers (Weird West Sundown Rail SMG → Cyberpunk: Ghostline, Neon Saint, Marginal Press), 44 no wear and no mounts.
- Invariants: malign shares 3000/1000; cursed capsule face 4 with OMEN; `ENH_CAP`; Lucre per Erratum and dropped piles only; no durability and no mount mechanics.
- §2.3 code comment: which sources roll curses (cursed capsule included).
- §3.5 `item.tool.toolkit` keeps the `repair_kit` tag that doc 04's construct body `self_repair` and doc 07 read (body healing, not item wear).
- §17: rolled Lucre piles are instanced; purses per Erratum.

### Lint after batch 10

`python3 tools/spec_lint.py --chapter 11`: no undefined IDs, no foreign definitions, no retired IDs, no JSON issues; hedges {'might': 3} are the IDs `item.ring.might` and `attr.might`. JSON lint: doc 11 shares `item.quest.folio_leaf` with doc 12 (doc 12's Exports line) and the canon `cat.`/`slot.` IDs with docs 08 and 10 (their Exports); doc 08 now has `item.face.respirator` undefined (its three mentions must go, C-26). See cross-chapter requests below.

### Batch 11 (reconciler notes) — done

- §6.1 writes `{origin_genre.aether_axiom}` (canon §5.3 spelling) instead of `{origin.aether_axiom}` (notes_character).
- notes_archetypes (drop `arch.`/`ammo.` rebinding rows), notes_items (pool tags, books, keys, respirator records) and notes_alchemy (loot records on doc 08 IDs) were already satisfied on disk.

## Requests for other chapters

- **Doc 08**: (C-26) replace the three backticked `item.face.respirator` mentions (§4.3 respirator bullet, the Interface row for doc 11 "aliased to `apparatus.respirator.mask` (Canon gaps 8)", and Canon gap 8) with "doc 11 §3.4 stores loot records directly on `apparatus.respirator.cloth`, `.mask` and `.rebreather` (values 15, 40, 90; weights 40, 40, 20)", and close gap 8; doc 11 no longer has that alias, so these mentions are now undefined IDs. Also move `slot.face`, `cat.potion`, `cat.throwable`, `cat.tool`, `cat.container` out of its Exports into Imports (Canon §19 gives `cat.` and `slot.` to docs 00 and 11; lint lists them as multiply defined).
- **Doc 10**: move `cat.wand` from Exports to Imports (multiply defined). Delete gap 25: doc 11 §3.5 now weights `item.text.amnesia` 1000 of 10000 (test 39 holds). Update gap 24 and the doc 11 Interface row: doc 11 §11.2 now has the cursed capsule (face 4, `CursedCapsule`, OMEN), §9.1 applies `curse.pact.miser` to fresh piles, §2.2 stores `zaps_total`, §15 prices Rewrite from `value_vendor`.
- **Doc 12**: remove `item.quest.folio_leaf` from its Exports (the "Requested outside this chapter's namespaces" bullet) and list it under Imports; doc 11 §3.5 defines and exports it (lint: multiply defined 11, 12). The §20.5 "doc 11's seat relic (affix themes from the 2 domains)" and the I6 "the story relic" have no doc 11 template or table: either drop the relic or specify it as an ordinary Placed item with a fixed card and request a domain-to-affix table from doc 11.
- **Doc 02**: add `IntentKind::DropLucre { amount: i64 }` beside `ShopHaggle { shop }` (C-15); doc 11 §17 validates it (Lucre passes between party members only as dropped piles, DECISIONS 83).
- **Doc 14**: (C-02) its Interface row for doc 11 still lists former names in brace form; replace `item.potion.{healing, haste, focus, fortify, aether, antidote, invisibility, clarity, regeneration, shielding}` with doc 08's `alch.potion.healing`, `.haste`, `.calm`, `.fortitude`, `.aether`, `.cure`, `.invisibility`, `.purity`, `.honeyed_healing`, `.shielding`, and `item.throwable.{javelin, knife, smoke, frag, fire_flask, poison_flask, disruptor_flask}` with `arch.ranged.thrown` (javelin and knife skins), `arch.ranged.grenade_smoke`, `arch.ranged.grenade_frag`, `alch.bomb.firebomb`, `alch.potion.toxic_gas` and `item.throwable.counter_disruptor`; in §2.4 drop "and doc 11's former-name table" (doc 11 deleted its doc-14 rows; the mapping stays in audit/map_items.json).
- **Doc 16**: delete G11 (ruled by G-148; doc 11 §17 uses `DropLucre` piles; trades and `DropItemFor` keep refusing `cat.currency`), and drop "not mounted" from §9.7 trade eligibility (mounts cut, C-30).
- **Doc 04**: `self_repair` must not spend a use of the `repair_kit` tool: doc 11's `item.tool.toolkit` is never consumed (Canon §4.3: tools never wear; doc 07 already says "never worn or consumed").
- **Doc 07**: salvage yields (doc 11 §16.4) are now multiplied by `10000 + X` of `affix.engineered`; doc 07 rounds the units as before.
- **Doc 00 (canon owner)**: Canon gaps 1 of doc 11 proposes §17.1 Abridged text "and scaled kill-drop lines (doc 11 §9.1)".

## Verification

Adversarial verifier, 2026-10-11. Every check below was re-run on the file as it stands after the fixes listed under "Verifier fixes". Only `spec/11_ITEMS_LOOT_ECONOMY.md` was edited; no commit.

### Check 1: triage rows that name chapter 11

| Row | Verdict | Chapter text that satisfies it |
|---|---|---|
| G-008 | applied | §1 principle 8: "a second segment beginning with `x_` belongs to a mod (Canon §1.1: `x_<mod_id>`)"; Interface 02 row cites Canon §1.1. |
| G-016, G-223, C-15 | applied | Imports doc 02: "`ShopHaggle { shop }` and `DropLucre { amount: i64 }`" (now present in doc 02 §5.5, so "requested" was dropped); §12.2 "through the intent `ShopHaggle { shop }`". |
| G-025 | applied | §2.3 tag table lists every `rng.loot` use with its draw count; §9.7 bag draws, §9.7 burning `roll_bp(1000)`, §16.4 salvage "1 `roll_bp` and 1 `weighted`, always both", §2.3 step 13 illumination. |
| G-026 | applied | §2.3 source kinds: "Engine dispensing"; §11.2 compliance "no reels". |
| G-027 | applied | §9.11 `place_rng` = `mix64(stream_seed(...) ^ mix64(fnv1a64(label) ^ index))`, identical to canon §3.2 `derive`. |
| G-030 | applied | §11.2 `offer_rng` = `mix64(peek_u64(OFFER[slot]) ^ mix64(context_uid ^ fnv1a64(purpose)))`, identical to canon §3.5. |
| G-037, C-27 | applied | §5.2 Somnambule: "after 10000 mm of flight, projectiles (spell projectiles included) pass through destructible and non-opaque blocks without spending penetration; fixed terrain still stops them (Canon §4.3)". Verifier fixed the two identity lines that still implied walls ("wall-piercing", "thin walls"). |
| G-055 | consistent | Doc 07 owns licensed grafts; §5.2's Dep column gives each gimmick's dependency; loose loot parts are dependency none (§5.4). |
| G-058 | applied | §2.3 step 3 "40 while `st.unmoored`"; §11.2 `if p.has(ST_UNMOORED) { 0 }`. |
| G-062 | applied | §3.4 `GEAR_TOL_CAP = 3`. |
| G-065 | applied | §5.2 Rustmaw "magazine × 20000, reload time × 13000"; Hephaesteum "immune to `st.disarmed` and `st.corroded`". |
| G-084 | applied | §5.1 "multiplies the hit's `B` once after the Canon §10.4 chain and before step 1 (Canon §10.4)"; §13.3 names the three "× N" gimmicks on `B` (Perihelion, Rainslick, Thimbleweald). |
| G-129 | applied | §3.3 "`cap_base_mhp → × T(ilvl) → × rarity power → × quality → × (10000 + 600 × eff_enh) → × Contradiction potency` ... with no material term"; §18 ward generators use material 10000. |
| G-130 | applied | §3.9 rows `slot.hand_extra_1`, `slot.hand_extra_2`, Exotic group "weapon". |
| G-131 | applied | §16.1 "Refused at the cap ... (Canon §13.7 `ENH_CAP`)". |
| G-134 | applied | §2.3 step 13 `illum_pending` and first-acquisition roll. |
| G-136, G-138, G-146, G-147 | applied | §16.1 `cat.tool`; §12.2 prices from `value_vendor`, §12.3 `appr.svc.*`; §3.5 Books "never drawn from the deck"; §3.10 alias table. |
| G-142, C-02 | applied | Doc 14 now names owner IDs only (grep: no `item.potion.`, `item.container.` or retired `item.throwable.` in doc 14); the Former names table has no doc-14 rows. Doc 14 §2.4 still says "doc 11's former-name table" (request below). |
| G-143, C-26 | applied | §3.4: "doc 08's `apparatus.respirator.cloth`, `apparatus.respirator.mask` and `apparatus.respirator.rebreather` ... values 15, 40 and 90, template weights 40, 40 and 20. These are the chapter's only respirator records". No `item.face.respirator` anywhere in the spec. Verifier added `apparatus.respirator.*` to §2.1's owner list and to the intro. |
| G-148, Q3, DECISIONS 83 | applied | §17 "Lucre passes between party members only as dropped piles inside one run"; `DropLucre`; §9.1 `fresh = false`; §14.1 purse per Erratum. Verifier added the dropper exception to §14.3 auto-pickup. |
| G-161 | fixed | Canon §17.3a now reads "in every genre's loot pool and in every shop"; docs 05, 08 and 13 read it the same way. The chapter stocked counters only at the Colophon and general store. Verifier added the §12.1 "Counter shelf" to every other shop, updated §9.10 and test 29. Unwritten caches unchanged. |
| G-167 | open gap | Canon §17.1 names only chests and floor-loot spots; §9.1 also scales kill-drop lines. Kept as Canon gaps 1 with a proposal (the canon owner must rule). |
| G-171, G-224, G-225 | applied | §3.5 five counters; §9.1 boon on fresh piles; old gaps 1 and 12 removed. |
| C-05 | consistent | §9.1 boon on fresh piles only (doc 14 owns the change). |
| C-30 | applied | No retired mount ID; §3.10: "Mounts are cut (DECISIONS 55, Canon §17.7), so this chapter defines no mount category, slot, token, affix, shop or loot line". Verifier renamed mount-flavored names (see check 4). |
| C-31 | applied | §1 principle 9 "No wear"; no durability field, affix, tendency, gimmick, service or perk. |
| §4.1 stale gap 12 | applied | Canon gaps hold no staff, wand or tome gap. |

### Check 2: DECISIONS rows

- Changed rows: invasions (§17 Invaders), limited building (`item.text.demolition` and `perk.leg.worldbreaker` hit only `block.tag.destructible`), softer Erasure (`loot.drop.hunter` per hunter; no repeat-spawn assumption), mounts cut, pact curses (§11.2 cursed capsule), no durability: agree. Echo cast, hybrid glyph learning, micro-voxels, text-only narration, USD 19.99, procedural ending, World Laws in play, 12+ enemies, crippled-pool severing: the chapter states nothing about them (no Lucre skin, no price, no voice, no sever rule; `item.tool.crutch` and the reattach service only).
- 19, 60, 65, 66, 67, 68, 88: agree (§2.3, §11.2, §1 principle 9, §14.1, §15 cost by `value_vendor` and exactly 3 offers, §7.1, §5.2). 67's "3 offered skins" are the 3 stored manufacturer variants of the local-genre skin (Brief §4: Rewrite targets the local genre), as in the pre-fix default the owner kept.
- 77 to 88: 83 and 86 and 88 agree (§17, invaders receive no loot); 77 to 82 and 84, 85, 87 name no doc 11 rule and the chapter contradicts none.
- 90 (Crowns kept, overruling triage Q9 "Nobles"): the chapter names no Lucre skin, so neither "Crowns" nor "Nobles" appears; nothing to change.
- 95 and 102 also cite doc 11 and were missed: canon-1.11 carries neither (doc 03 G31). Verifier added Canon gaps 2 (grappling-tool item, Second Edition carry-over and its `ENH_CAP`).

### Check 3: chapter brief

Mounts (x7) and durability (x19) gone; `cat.block` gone; C-02 doc-14 rows deleted; C-26 and C-27 as above; Q3 as above; Q9 overruled by DECISIONS 90 (no skin named); stale gap 12 deleted; `item.quest.folio_leaf` defined in §3.5 and exported, and lint reports no multiply defined ID; DECISIONS 60, 66, 67, 68 as above.

### Check 4: leftover grep

Real hits fixed: "Cavalry Sabre" (now "Officer's Sabre"), "Horse Pistol" (now "Holster Pistol"), vendor "Saddler" (now "Tanner"), repair-named tinkers "Radio Repairman", "Repair Shop", "Music-Box Repairer" (now "Instrument Maker", "Hardware Store", "Music-Box Maker"; no shop repairs items), identity lines "wall-piercing" and "thin walls". Remaining hits are intended: the no-wear principle, Imports and Interface notes, test 44 and the invariant; `repair_kit`, the tag that doc 04's construct body `self_repair` and doc 07 read (body healing, toolkit never consumed); "amount". No hit for ride, Delving, voice, VO, 24.99, Item Codex, Crowns, Redactor, `margin.plot`, building plot or 5400.

### Check 5: lint

`spec_lint.py --chapter 11`: no undefined, foreign, retired or JSON findings; hedges `{'might': 3}` are `item.ring.might` and `attr.might`. `--json` multiply_defined has no doc-11 entry (0 IDs defined in more than one chapter).

### Check 6: internal consistency

- Recomputed with Python from the current §6.2 table: 113 affixes, equal to Exports; the §20 SMG has 46 eligible affixes, Σ 1895; draws 1830, 926, 397 give `affix.self_injector` (1825..=1844), `affix.infuse_fire` (920..=944), `affix.dmg_vs_beast` (370..=409); X = 818 and 1880.
- Weights: found potions Σ 10000 with harmful 3000; Reliquary-only Σ 9000 with harmful 2700; texts Σ 10000 with harmful 1000; throwables Σ 10000 with counters 4000; vendor draw Σ 17; deck weights Σ 495000; Engine faces Σ 10000 with face 4 at 900, 1000, 1000 Bp.
- Prices and values re-derived: 27 Lucre, 29 after find, value 1382, IP 237 and 201, sell 414 and 51, gunsmith 2418, haggle 1900 and 12899 and 1782, Grade 10 2781 and 373, Rewrite 483, reforges 472, 1416, 3304, 7080, Engine prices 100..200 and 268..536, pity expectations 8.75 and 7.69. All match the text.
- Every core genre's three manufacturers still make all ten classes (test 15); test 43's Rewrite candidates are Ghostline, Neon Saint, Marginal Press in that order.
- Tables: every row has its header's cell count. Exports: 575 IDs, 0 duplicates, every declared count equals its list; every doc-11 ID used in the body is exported except the alias `item.relic.lattice_lens`, which §3.10 keeps by G-147.
- Header "Conforms to canon-1.11". Canon gaps hold only open items (1 and 2) after a closure paragraph. Interface assumptions: verifier removed the stale doc 13 `loot.drop.{armed, ...}` sentence.

### Verifier fixes

1. Intro and §2.1: respirators listed among doc 08's `apparatus.` IDs that carry loot records.
2. Imports doc 02: the two intents are now doc 02's, not requests; Exports wording "Intents validated here".
3. Imports doc 10: both malign shares (3000 Bp potions, 1000 Bp texts).
4. §1 principle 6 and §2.1: a loot record references the owner's ID and is serialized as doc 19's `11.template_loot`, never a second record under the same ID (doc 19 §1.2).
5. §1 principle 8 and Interface 02: mod patches append to the `x-patchable` `lines` array (doc 02 §13.1, doc 19 §3.6), matching doc 02's interface row.
6. §2.2 and §2.3: new `keepsake` flag, which §14.4's `DropItemFor` refusal and doc 16's interface row need.
7. §3.5: `item.text.inscribed` also comes from a scraped identified text (doc 06 §8.3, doc 06's open request).
8. §4.2, §12.1, §5.2 identity: mount-flavored and repair-flavored names, wall-implying identity lines.
9. §9.10, §12.1 counter shelf, test 29: every counter in every shop (G-161).
10. §12.1: doc 13 §13.2 defines all ten vendor types, the Colophon's and wanderer's included.
11. §12.2: hidden part of `eff_enh` as 0 (doc 10's open request); Colophon vendor always at reputation 10000 (doc 13 §13.2). §18 `IP_shown` uses the same default.
12. §14.3: auto-pickup skips a pile for its own dropper.
13. §19: stored Rewrite and reforge offers are saved (Canon §3.5 "generated once and stored").
14. Interface 13: stale `loot.drop.{...}` sentence removed.
15. Canon gaps 2: DECISIONS 95 and 102 (with 106).
16. Acceptance tests: stray blank line before test 41 removed.

### Still open after verification

- Doc 08 asks doc 11 for found raw-liquid bottles (`apparatus.vessel.bottle`, 750 ml) in `loot.chest.plain` consumable draws and `shop.apothecary` lines; doc 11 has no such line or weights yet.
- Doc 09 asks for an `outfit_of` instance field, door keys in `cat.key` with `key_tag`, tool qualities and a `lock.kind.container` lock on `loot.chest.locked`; none is in doc 11 yet.
- Doc 12 §13 and doc 10 rely on a "story relic" that doc 11 fills; doc 11 defines none.
- Doc 19 proposes `loot.tpl.` record IDs for `11.template_loot`; if adopted they are doc 11's IDs and must enter its Exports.
