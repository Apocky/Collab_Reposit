# Fix log: doc 15 (Monetization) to canon-1.11

Chapter: `spec/15_MONETIZATION_GACHA_LOOTBOX_PASS.md`. Only that file is edited. Each batch below is appended after its edits land.

## Plan (from the triage, DECISIONS and doc 21)

- Header to canon-1.11.
- Triage rows naming doc 15: G-031 (Canon §3.6 amended), G-149, G-150 (welcome Ink in Canon §14), G-151, G-152, G-153, G-154, G-160, G-174, G-179, G-186, G-188, G-189, G-190 (doc 01 has `kpi.econ.*`), G-197 / C-29 ("Index", never "Item Codex"), G-227 (`crate.econ` in doc 02). §4.1: delete gaps 10 and 12. Every one of the 16 old gaps is resolved, so they are all removed.
- DECISIONS 42 (USD 19.99), 57-60, 74, 75, 79 (80-hour unlock pace), 80 (Recollection for every Echo), 86 (invader rewards), 94 (ink gore), 41 with doc 21 §2.3 (g) (Early Access Script gating), doc 21 §3.3 (`T0 = 1820448000`).
- Canon-1.11 §2.4: `presentation_hash` split, so presentation-only mods no longer make a run Apocrypha.
- Lint: `echo.*` and `mode.*` sit in first table cells (the linter reads "Echo acquisition in 15" as co-ownership); move them out of first cells. Define the undefined `banner.`, `bundle.`, `cosm.` IDs explicitly in Exports.

## Batches

### Batch 1: header, Imports, price (done)

- Header: "Conforms to canon-1.11".
- Imports: DECISIONS rows widened to every row that lands in doc 15 (1, 2, 13, 15, 16, 28, 32, 41, 42, 53, 57-60, 73-75, 79, 80, 86, 87, 94, 98); Canon §2.4 (`presentation_hash`), §3.6 (amended text, G-031), §13.6 (pacts), §14 (welcome grant, G-150; trade between accounts, G-148), §19 (`echo.`/`mode.` are doc 14's); new rows for doc 01 (`kpi.econ.*`, G-190), doc 21 (§2.3 (g), §2.5, §3.3); doc 02 row names `crate.econ` (G-227) and §13.1 mod classes; doc 10 row names `roll_curse`, pacts and the Index; doc 14 row names §2.3 EA scope, §5.4a pace and bounded trials; doc 16 row names kick votes and voice.
- Price (DECISIONS 42, doc 21 G5): base game USD 19.99 at Early Access and 1.0 in §1.1 and §7.1. §7.2 launch matrix: 24.99 and 29.99 rows replaced by an authored 19.99 row (EUR 19.99, GBP 16.99, BRL 74.99, JPY 2,900, KRW 24,400, AUD 29.95, CAD 25.99, CNY 90, same ratios as the other rows). Conversion anchor moved to the 19.99 row; example and test 27 recomputed: `floor_div(5000 × 1999, 2900) = 3446` (EUR 34.46).
- §1.1 names Folio Expansions 01 (Age of Sail and Pirates) and 02 (Kaiju and Mecha), DECISIONS 53.
- §7.2: "Steam's pricing tool and the console equivalents" became Steam's and the Epic Games Store's (DECISIONS 73, 98: PC only).

### Batch 2: `T0 = 1820448000` (done; doc 21 §3.3 and its Interface row for doc 15)

- §3.2.3: `T0 = 1820448000` (2027-09-09); p001 ends "7 Oct 2027, 00:00 UTC".
- §3.13 banner record: `start_utc` 1820448000, `end_utc` 1822867200. §6.8 Codex record: `release_utc` 1820448000.
- §4.6 receipt: `iat` 1821052800 (2027-09-16), `kid` `rk_2027a` (one key per calendar year), signature recomputed with RFC 8032 test key 1 (`cryptography` 50.0.1; the old 2026 signature reproduced first as a check): `f3510f1f…944b0b`; still 208 bytes. Test 10 updated.
- §10.4 run report: `started_utc` 1821052800, `ended_utc` 1821054000.
- Request to doc 16: its §11.4 pull JSON still uses the window `[1792886400, 1795305600)`, receipt `iat` 1792890000 and `kid` `rk_2026a`.

### Batch 3: C-29, G-031, pacts, DECISIONS 59 (done)

- C-29 / G-197: §5.5 records an identified Keepsake in the account's Index (doc 10 §12.1, `margin.index`) and states that "Codex" means only the pass; §6.2 `pass.xp.codex_entry` pays 200 per Index template page or Bestiary page first opened on the account (doc 10 §12.1, §12.4). The ID and the run-report field `codex_entries_new` keep their names because docs 10 and 14 cite them.
- G-031: §4 intro now quotes amended Canon §3.6 ("an HMAC-SHA256-keyed ChaCha20 generator with commit-reveal"); §4.2 notes Canon §3.6 names the Keepsake seed as the one `Rng::from_seed64` use outside the simulation. §4.2 cites doc 02 `crate.econ` (G-227) instead of "requested, Canon gaps 10".
- DECISIONS 59: §5 intro states account Reliquaries hold Keepsakes and, through `illuminated`, cosmetic skins, never raw power.
- DECISIONS 60: §5.4 curse row names doc 10 `roll_curse` and the pact share `PACT_SHARE_BP = 2000`; §5.6 public page shows curse and pact odds per category and rarity (example: Gothic Horror weapon Keepsake 24.0000% cursed, 4.8000% pact; Legendary or Exotic 1200 and 240).
- §5.4 fixes: stored `enhancement` is 0 (doc 10 §2.1); Serendipity is the neutral `SER = 40` of doc 11's Keepsake entry point; `illuminated` drawn from the Keepsake generator, never `rng.loot`.

### Batch 4: currencies, Apocrypha, lint first cells (done)

- §2.1: maps Canon §14's source words onto the rows (welcome grant G-150; "Codex tiers" = free track only, G-149); Vellum sink list is §3.9's under Canon §19 (G-151); new **Ownership** paragraph: premium tracks, expansions, cosmetics, Echoes, Folds and Scripts are entitlement rows keyed by content ID and source, backed by receipts; `ent.quill` is the only countable entitlement (G-152).
- §2.2 Apocrypha: a run is Apocrypha only when a simulation-affecting mod (doc 02 §13.1) is loaded; presentation-only mods go to `presentation_hash` (canon-1.11 §2.4, DECISIONS 87). Test 12 adds a presentation-only-mod run that earns normally. §10.4 step 4 recomputes `content_hash` over records outside the presentation namespaces.
- §10.4: the run report is doc 16's run-report message (G-153).
- Lint: the §2.2 mode table gets a numeric first column (1 to 6) so `mode.*` IDs (doc 14's) are not in first cells; the §2.8.2 Echo table now starts with the Genre column. `multiply_defined` no longer lists chapter 15.

### Batch 5: DECISIONS 79 and 80, Early Access gating (done)

- DECISIONS 79 (80-hour unlock pace): Scripts are non-cosmetic options (kits, glyph families, rooms, hirelings) and were reachable only through the Inkwell or a 1,680-Vellum Archive bill, far beyond 80 hours. New §3.9.1 **Transcription shelf**: per-account counter `transcription`, +1 per qualified Folio exit of a run whose mode row grants account Reliquaries (same exits as the §5.1 lifetime counter); claim any unowned, openable Script for `TRANSCRIBE_RUBRICATED = 16` or `TRANSCRIBE_ILLUMINATED = 32`; refusal code `insufficient_transcription`. 36 launch Scripts = 768 exits = 64.0 reference hours (12 exits/h), 76.8 at doc 14's 10 Folios/h, below doc 14's 85.0-hour contract; +64 exits per expansion. Not a currency (a counter like spark), never sold (SEP-4 extended), in the audit log, `pity` receipts, `eos_final`, idempotency table (`transcribe`), and end-of-service local rules. New §2.8.3 pace table (Bindery 70.0, Echoes 84.2 from doc 14 §5.4a; Scripts 64.0/76.8). §2.8.2 Script hours name the route. This is a fixer design addition to meet a binding row; the product owner may prefer another route (listed under unresolved).
- DECISIONS 80: §3.1.1 says every Echo, expansion Echoes included, has its bounded Recollection trial; §2.8 design contract cites DECISIONS 57 and 80.
- DECISIONS 41 with doc 21 §2.3 (g): new §3.2.7. Lost Chapter Scripts of unshipped genres are absent from pools, featured lists, the Archive, Transcription and Standard spark; a period whose Featured Script is absent runs no Script banner (EA: p001); entries join from the first period starting at or after their genre ships, with new `config_hash` records published 7 days ahead. "Exactly one banner per family per period" became "at most one" (§3.2.1, §4.1). Standard spark pool size is "24 in the 1.0 catalog". EA odds computed exactly: Featured Script off-featured 0.0500% / 0.1020%; Standard Illuminated (18 entries) 0.0556% / 0.1416%.

### Batch 6: chat, ratings, KPIs, Codex theme (done)

- DECISIONS 75: §8.5 adult default adds "voice with strangers off": a party member not on the player's platform friends list starts voice-muted until unmuted.
- DECISIONS 74: §8.5 new "Lobbies" row: friends by platform invite, public lobby browser (doc 16), mute, block and report on every member, kick votes by doc 16 §9.2's rule (`max(2, m − 1)` yes votes within 1,800 ticks).
- DECISIONS 94: §8.6 states ink gore by default with a red-blood toggle, targets PEGI 16 and ESRB M, and every board rates the red-blood setting; the toggle never changes rated content.
- G-190: §12 table gains a "Doc 01 ID" column with the `kpi.econ.*` IDs of doc 01 §10.3, plus the doc 01 row `kpi.econ.quires_per_hour_p50` (50 to 160); chat report rate now "alert on any rise", matching doc 01's `no_rise`.
- §6.6: a Codex themed on a core genre cycles over the genres the build ships (§3.2.7).

### Batch 7: Exports, interfaces, gaps, tests, invariants (done)

- Exports: every ID the chapter uses is now listed explicitly: `banner.echo.p001` to `p012` and `banner.script.p001` to `p012` (first rotation cycle), `banner.family.exp01`/`exp02`, `store.sku.folio_exp_01`/`02`, the 36 launch Scripts by name in index order, the 60 Plain cosmetics in pool order, the 4 Red Ink marks, the 6 `cosm.binding.*` and 12 `cosm.palette.*` finishes doc 14 names, and the 20 `juris.flag.*` IDs. Constants add `TRANSCRIBE_RUBRICATED = 16`, `TRANSCRIBE_ILLUMINATED = 32`, `T0 = 1820448000`; error codes add `insufficient_transcription`.
- §2.4: invader Quires read `ev.invasion_ended`'s `duration` and `redacted` (doc 03 §14.2), so the doc 13 interface row was removed (met by docs 03 and 16).
- §3.2.7 generalized: any entry a build leaves out stops its featured banner for that period or leaves its rate-up list (remaining rate-ups share `mid_featured_bp`), covering doc 14 §2.3 if it removes Echoes at EA.
- Interface assumptions: met requests from docs 14, 11, 10, 02, 01 moved to "on disk"; new requests: doc 14 counts Scripts in §5.4a, doc 16 lobby browser, kick votes and stranger voice mute, doc 17 Transcription shelf and ink-or-blood toggle, doc 21 shipped genre set and dates.
- Canon gaps: all 16 old gaps closed (G-031, G-149 to G-154, G-160, G-174, G-179, G-186, G-188 to G-190, G-227; §4.1 stale gaps 10 and 12); section now reads "None open" with the closure record.
- Tests: 18 no longer names the impossible `banner.exp01.p001`; 23 adds a SKU granting Transcription; new 36 (Transcription), 37 (Early Access build), 38 (price row), 39 (chat defaults), 40 (Keepsake curse and pact odds).
- Invariants: "≤ 1 live banner per family per period", gating, Transcription, 19.99, presentation-only mods; `hazard.base_bp` renamed `base_bp` (notes_world: avoids the `hazard.` prefix).
- Lint after batch 7: chapter 15 clean (no undefined, foreign, retired, JSON or hedge issues); `multiply_defined` lists nothing for chapter 15.

### Batch 8: consistency sweep (done)

- §3.2.2 example and test 32: the `g = 1` Script banner is a 1.0-catalog statement; a build without Cosmic Horror runs none (§3.2.7).
- §3.2.6: an expansion's banners start with the first whole period at or after release (banners always span a whole period).
- §3.13 validator: every featured, rate-up and pooled entry is in the build's content set.
- §5.5 Satchel modes list Abridged (it was already in mode row 1 and the materialization rule).
- §6.2 and AP-8: "all play earns Codex XP" now names the invading exception (Canon §17.9, DECISIONS 86, triage Q8 / G-174).
- Invariant `ilvl := d(f)` (was "folio index", wrong for Abridged).

### Batch 9: wording (done)

- §2.1 Vellum sink sentence and §2.8.3 pace-contract sentence tightened (the 10-Folios-per-hour measure applies to the Script row only).

## Lint after all batches

`python3 tools/spec_lint.py --chapter 15` → `== 15  words=27149`, no findings; `--json`: no `multiply_defined` entry names chapter 15.

## Requests for other chapters

- **14**: §5.4a counts the 36 launch Scripts among DECISIONS 79's non-cosmetic options, adding the row "36 Scripts by Transcription alone (doc 15 §3.9.1): 768 qualified exits, 76.8 hours at 10 Folios per hour"; `margin.vellum_archive` hosts the Transcription shelf. §2.3 says Echoes keyed to an unshipped core genre are absent from Early Access content sets, but doc 21 §2.3 (c) ships all 24 Echoes at Early Access; align with doc 21 (doc 15 §3.2.7 handles either outcome).
- **16**: §11.4 pull example uses `banner.echo.p001`'s old window `[1792886400, 1795305600)`, receipt `iat` 1792890000 and `kid` `rk_2026a`; move to `[1820448000, 1822867200)`, an `iat` inside it (for example 1820451600) and `rk_2027a`. §9.12 voice defaults: adults push-to-talk, party only, and a party member who is not on the player's platform friends list starts voice-muted (doc 15 §8.5, DECISIONS 75).
- **17**: Vellum Archive screen gains the Transcription shelf (counter, costs 16 and 32, refusal `insufficient_transcription`); default-muted voice for non-friends; the ink-or-blood toggle defaults to ink (DECISIONS 94).
- **01**: §9.3 price row "24.99 in EA, 29.99 at 1.0" becomes "19.99 at Early Access and 1.0 (DECISIONS 42)".
- **21**: G5, G7 and G14 are met by doc 15 (19.99; `T0 = 1820448000`; §3.2.7 gating); WP-074 adds the Transcription shelf and doc 15 tests 36 to 40.
- **10**: §12 intro and Canon gap 5 say doc 15 calls the Index "Item Codex"; doc 15 now says Index (C-29), so the parentheticals can go.
- **02**: `palimpsest_econ` also holds the Transcription claim; `pal odds` computes per-build odds records (doc 15 §3.2.7); §11.2's `MOD` section holds only simulation-affecting mods under canon-1.11 §2.4 (doc 15 §2.2 relies on it).

## Verification

Adversarial verifier pass, 2026-10-11, after batch 9. Only `spec/15_MONETIZATION_GACHA_LOOTBOX_PASS.md` and this log were edited. Every edit ran through a Python script that asserts each anchor occurs exactly once. Line numbers are those after the pass.

**New since the fixer ran.** DECISIONS.md gained rows 109 to 125 (commit `40d2963`, after the fixer's last batch). Rows 111, 112, 113, 119 and 122 name doc 15. Row 114 supersedes row 41, on which §3.2.7 and test 37 were built. Doc 14 also changed while this pass ran: it gained §8.9 Second Edition (`mode.second_edition`, `RunSummary.ironman`, "doc 15 grants its exclusive Ironman cosmetic") and a 59th achievement. The chapter was brought to all of these.

### Check 1: triage rows that name chapter 15

| Row | Verdict | Chapter text that satisfies it |
|---|---|---|
| G-031 (Canon §3.6 amended) | applied | §4 intro (l. 782): "This is the generator Canon §3.6 names for account draws ("an HMAC-SHA256-keyed ChaCha20 generator with commit-reveal")"; §4.2 (l. 827): "Canon §3.6 names this as the one use of `Rng::from_seed64` outside the simulation"; Imports Canon §3.6 row |
| G-149 | applied | §2.1 (l. 89): "\"Codex tiers\" are free-track tiers only, because SEP-3 keeps every purchasable track free of currencies" |
| G-150 (welcome Ink, Canon §14) | applied | §2.1 (l. 89): "\"a one-time welcome grant\" is `WELCOME_INK` (§2.2)"; §2.2 welcome row: "`WELCOME_INK = 1,200` … once per account"; `mint/ink/welcome`; test 33; Imports Canon §14 "its sources include the one-time welcome grant" |
| G-151 | applied | §2.1 (l. 89): "Canon §19 gives this chapter the account economy rules, so the Vellum sink list is §3.9's" |
| G-152 | applied | §2.1 Ownership (l. 91): "Premium tracks, Folio Expansions, cosmetics, Echoes, Folds and Scripts are entitlement rows keyed by content ID and source … `ent.quill` is the only countable entitlement" |
| G-153 | applied | §10.4 (l. 1487): "each player's client submits one run report as doc 16's run-report message" |
| G-154 | applied | §2.4 (l. 163): "This chapter owns the Quire earn rate (Canon §19); doc 14 §4.6 supplies one `run_score` per slot" |
| G-160 | applied | §3.1.1 (l. 284): "Each core genre's doc 14 Tier 2 Echo (the 9- to 16-hour trial) is Illuminated and its Tier 1 Echo (4 to 9 hours) is Rubricated" |
| G-174 / Q8 | applied | §2.2 mode row 6 (invader side: no Ink, no Codex XP, no Reliquaries); §6.2 (l. 1051): "All play earns Codex XP, except invading another party, which Canon §17.9 and DECISIONS 86 limit to Quires and Red Ink marks"; AP-8; test 12 |
| G-179 | applied | §2.2 (l. 125): "Only Ur-Text victories pay the victory bonus"; victory rows trigger on `boss.ur_editor` only |
| G-186 | applied, then corrected | §6.2 uses `rank_xp_bp`. **Miss fixed:** the "hunter 25" figure ignored Canon §11.6 (softer Erasure, DECISIONS 26 and 91), under which the Erasure hunter uses `rank.elite`'s kill XP (15); 25 holds only for an invader. New text and test 42 |
| G-188, G-189 | applied | Exports `bundle.script.*` (36 by name) and `banner.reliquary.account` |
| G-190 | applied | §12 "Doc 01 ID" column with the eight `kpi.econ.*` IDs; doc 01 §10.3 rows checked: bands and `no_rise` modes match |
| G-197 / C-29 | applied | §5.5 (l. 1001): "its template is recorded in the account's Index (doc 10 §12.1 …). \"Codex\" in this chapter always means the battle pass of §6"; §6.2 `pass.xp.codex_entry`: "each Index template page or Bestiary page"; grep finds no "Item Codex" |
| G-227 | applied | §4.2 (l. 825): "in the library crate `palimpsest_econ` (doc 02 `crate.econ`)" |
| G-017 (`presentation_hash`), G-148 (between accounts) | applied | §2.2 Apocrypha: presentation-only mods "enter `presentation_hash` instead and never make a run Apocrypha"; Imports Canon §14: "no currency is bought, or traded or gifted between accounts"; SEP-7 |
| §4 conflicts | applied | Only C-29 names doc 15 (above); C-30 and C-31 retired IDs: none in the chapter |
| §4.1 stale gaps 10, 12 | applied | Old gaps removed. Canon gaps now holds one new open item (below) and a one-sentence pointer to this log |

### Check 2: DECISIONS rows

Rows whose "Lands in" names doc 15, or a canon section this chapter elaborates (§2.4, §3.6, §11.6, §13.6, §14, §17.8, §17.9):

| Row | Verdict |
|---|---|
| 1, 57, 58, 59 | agree: no premium currency (§1.1, AP-1); 160 Ink, hard pity 60, 75/25 with guarantee, spark 100, 19.6 h (§2.8, §3.3 to §3.6); premium = cosmetics plus Quill, currencies on the free track (SEP-3, §6.4); Reliquaries hold Keepsakes and skins, never raw power (§5 intro) |
| 13, 94 | agree after fix: §8.6 now names doc 17's actual setting, `access.gore` (`ink` default, `full`, `minimal`); it had described a separate "ink-or-blood toggle" plus a "gore intensity option" that doc 17 does not have |
| 15, 32, 86 | agree: §2.2 row 6, §2.4, §8.5 invader rows. **Miss fixed:** §2.4 now states doc 16 §9.13's outcome rule (shorter reported length; Redacted only when both reports say so) |
| 16 (limited building) | agree: only `cosm.margin.*` decoration slots; no building plot, no `margin.plot` text |
| 26, 91 (softer Erasure) | **miss fixed**: Erasure hunter kill XP 15 (above); no "5400" Erasure repeat (the only 5,400 is the 90 s Folio dwell) |
| 28, 119 | agree: §3.1.1 places Fenrir (MF), Lilith (GH), Jörmungandr (CH) as Rubricated and Sun Wukong (WX), Loki (MA) as Illuminated, matching doc 14 §6.3's Tier 1 and Tier 2; Recollection hours equal doc 14's `est_ch / 100` for all 24 |
| 33 (hybrid glyphs), 37, 38, 46, 55, 56, 65, 69, 84 | agree or not applicable: Script glyph families only add pool content; no voxels, narration voice, ending, mounts, World Laws, durability, rosters or limbs in the chapter (grep below) |
| 42 | agree: base game USD 19.99 (§1.1, §7.1, test 38); Codex premium 9.99; cosmetics 1.99 to 9.99 |
| 53 | agree: §1.1 cadence paragraph |
| 60 | agree: §5.4 `roll_curse` and `PACT_SHARE_BP = 2000`; §5.6 odds (recomputed: Gothic Horror MYTHOS 9, weapon `curse_base` 1500 → 2400 and 480; Legendary 1200 and 240) |
| 74, 75 | agree: §8.5 Lobbies row matches doc 16 §9.2's kick rule; "voice with strangers off" matches doc 16 §9.12 |
| 77, 78 | agree: Inkwell, Archive and Vault are Margin stations (doc 14 §5.1) |
| 79, 80 | agree: §2.8.3 rows equal doc 14 §5.4a on disk (70.0, 84.2, 76.8 / 64.0); §3.1.1 and the design contract give every Echo a Recollection trial |
| 81, 82, 83, 85, 88 | agree or not applicable: "present" = Active or Ghost; nothing blocks Mercy runs; SEP-7 bans transfers between accounts |
| 87 | agree: §2.2 Apocrypha and §10.4 step 4 |
| 98 | agree: PC only |
| 108 | **miss fixed**: not imported and not handled. New §2.2 **Second Edition** paragraph: `mode.second_edition` under mode row 1; the Ur-Text victory stays paid only if the run ends `run.end.victory`, otherwise `victory_present = 0` and run-end pay; the Ironman cosmetic `cosm.frame.ironman` once per account from a `RunSummary.ironman` clear. Also: §2.2 and §6.2 victory rows, §5.1 source genre of a fused Second Edition Folio (lower Canon §5.2 row) and of the Ur-Text Folio in Abridged (15, not only 25), §7.1 one-source list, §10.4 `ironman` field, Exports, test 41, invariants |
| 111 | **fixed**: §3.2.3 no longer says "Early Access launches on 2027-09-09 (doc 21 §3.3)"; `T0` is "a configuration value of the economy service set at launch", and 1820448000 is labelled illustrative. Imports doc 21 row and Exports "Configuration" updated |
| 112 | **fixed**: §7.2 cited "the Epic Games Store's" pricing tool and Imports said "Steam and Epic storefronts". Now Steam only in §1.1, §7.2, §10.1 `econ-platform`, Imports and an invariant |
| 113 | **fixed**: §1.1 states "There is no demo and no public playtest build (DECISIONS 113)" |
| 114 (supersedes 41) | **fixed**: §3.2.7 was written for a 6-genre Early Access. It now states that every build ships all 12 core genres and keeps the content-set rules only for builds that leave an entry out; the 6-genre odds survive as a test-build example (recomputed exactly: 0.0500% / 0.1020% for 5 off-featured, 0.0556% / 0.1416% for 18 Standard entries). §3.2.2 now says p001 runs the Cosmic Horror Script banner; §3.6 and §6.6 lose the build qualifiers; tests 32 and 37 rewritten |
| 122 | **fixed**: Folio Expansion USD 12.99 → 9.99 (§1.1, §7.1, test 38). The 12.99 matrix row priced nothing any more and was removed. The §8.3 minor-cap example and test 27 had relied on a CNY 58 expansion; they now use a CNY 46 expansion refused by the monthly cap (`160 + 46 = 206 > 200`) and the CNY 81 set of the §7.1 example refused by the per-transaction cap (`round_down_ending(mul_bp(9100, 9000), CNY) = 8100`) |
| 118, 120, 121, 123 to 125 | do not name doc 15; nothing in the chapter conflicts (the chapter fixes no release tier for Second Edition or invasions; "EOS" in the chapter means end of service) |

### Check 3: chapter brief

| Item | Verdict |
|---|---|
| C-29 "Index", never "Item Codex" | pass (grep: 0 hits) |
| `banner.*`, `bundle.*`, `cosm.*` families defined | pass: the fixer listed them. Added explicit `pass.codex_001`, `store.sku.codex_001_premium` (used in §4.6 and §6.8) and `cosm.frame.ironman` |
| `mode.*`, `echo.*` referenced only | pass: Imports Canon §19 row says they are doc 14's; no first-cell definitions (lint: no `foreign_namespace_definitions`, no `multiply_defined`) |
| USD 19.99 | pass |
| DECISIONS 57 to 60, 74, 75, 79, 80 | pass (check 2) |
| Q8 invaders: Quires and Red Ink marks only, no Codex XP | pass |
| Welcome Ink per the triage amendment | pass (G-150 row above) |

### Check 4: leftover grep

`mount|ride|saddle|cavalry|durability|repair|wear|broken|breaks|Delving|voice actor|voiced|VO|24.99|Item Codex|Crowns|Redactor|margin.plot|building plot|5400`: the only hits are "amount" (ledger SQL and prose), "BRL 29.99" in the 7.99 matrix row (a correct regional price, not the old base price) and `FOLIO_DWELL_MIN_TICKS = 5400` (the 90 s dwell, not an Erasure repeat). No real leftovers. Extra greps for this pass: "Epic" (storefront; fixed), "Early Access" and "1.0 catalog" (fixed), "12.99" (fixed).

### Check 5: lint

`python3 tools/spec_lint.py --chapter 15` → `== 15  words=27606`, no findings. `--json`: chapter 15 has empty `missing_sections`, `undefined_ids`, `foreign_namespace_definitions`, `retired_ids`, `json_issues` and `hedges`; `multiply_defined` is empty.

### Check 6: internal consistency

- **Recomputed with Python** (exact fractions where the chapter claims exactness): `T0` 1820448000 = 2027-09-09 00:00 UTC, p001 end 1822867200 = 2027-10-07, receipt `iat` 1821052800 = 2027-09-16; receipt JSON 186 bytes, message 208 bytes, and the RFC 8032 test key 1 signature reproduces `f3510f1f…944b0b` exactly; idempotency key `909de25b…a4b6`; `E[T_top]` 39.2303 and consolidated 2.5491%; Standard 24-entry rows 0.0417% / 0.1062%; expansion rows 0.0192% / 0.0392% and 0.0833% / 0.1164%; conversion `floor_div(5000 × 1999, 2900) = 3446`; set bundles 1699 and 999; Ink examples 400, 1,115 (535 per hour), 455, 175, 721 (645 per hour), 841 (753); Codex 20,000 and 43,900; run report 95 Ink, 5,025 XP, 14 Quires; kill XP per rank 1, 5, 15, 30, 100, and 25 for an invader; Transcription 768 exits, 64.0 and 76.8 hours.
- **Fixed:** §2.4 and test 13 said victory C has `run_score = 28,641`; doc 14 §4.6's per-Folio cap (`RUN_SCORE_FOLIO_CAP = 800`) makes it 20,000 (still 400 Quires). The §2.4 run row now names the cap.
- **Fixed:** doc 14 §10 now has 59 achievements paying 6,440 Ink and 1,960 Quires; the chapter said 58, 6,360 and 1,935 (§2.2 and §2.4 achievement rows). Recounted from doc 14's table.
- Tables: every table row has its header's cell count (script check). JSON blocks parse (lint).
- Exports: every `banner.`, `bundle.`, `cosm.`, `pass.`, `store.` and `juris.` ID or pattern the body uses is listed (script check against the Exports text).
- **Interface assumptions** held mostly met items ("On disk and consumed as Imports …", docs 03, 04, 06, 10, 12, 13, and doc 16 rows already on disk). Rewritten to open items only: doc 14 §2.3 (still cites DECISIONS 41), doc 11 Keepsake template eligibility, doc 02 (`palimpsest-verify`, per-build `pal odds`, 30-day replay logs), doc 16 (`ironman` in the run-report message), doc 17 (Fairness panel modes, Transcription shelf screen), doc 18 (names, including the 60 Plain cosmetics), doc 21 (DECISIONS 111 to 114 and 122).
- **Canon gaps** said "None open" with a long closure record. One real open item found and recorded: Canon §13.9 defines `illuminated` as "one `rng.loot` roll", but Canon §3.6, §5.4 here and doc 11 §2.3 draw a Keepsake's roll from its own generator; proposed §13.9 wording given. The closure record is cut to a one-sentence pointer to this log.
- Header: "Conforms to canon-1.11", now "against DECISIONS.md through row 125" and the docs actually checked.

### Requests for other chapters (added by this pass)

- **Canon**: §13.9 `illuminated`: after "Engine faces and rewards (doc 11 §2.3)" add "; for a Keepsake, one roll of its Keepsake generator (§3.6, doc 15 §5.4)".
- **14**: §2.3 Early Access scope per DECISIONS 114 (all 12 core genres; it still cites DECISIONS 41); §8.9's "(1.0)" per DECISIONS 118 (Second Edition at Early Access); doc 15 now pays `mode.second_edition` under its mode row 1 and treats a forfeited Second Edition as `victory_present = 0`.
- **16**: the fixer's §11.4 pull-example request still stands (window `[1820448000, 1822867200)`, an `iat` inside it, `rk_2027a`); remove the Epic Games Store and EOS paths (DECISIONS 112, 123); the run-report endpoint accepts doc 15 §10.4's new `ironman` field.
- **17**: Fairness panel with fresh and pinned client-seed modes and "Reveal now"; Vellum Archive screen with the Transcription shelf; showing `cosm.frame.ironman`.
- **18**: display names and `loc.` keys for the 60 Plain cosmetics and `cosm.frame.ironman`.
- **01**: Folio Expansion price 12.99 → 9.99 (DECISIONS 122) in the §9.3 table, Q-12 and the comparables row that cites "Folio Expansions at 12.99".
- **21**: DECISIONS 111 (no hard dates; §3.3's `T0` becomes illustrative), 112 (Steam only: drop the Epic Games Store rows, WP-069, R-06, RQ-07), 113 (no playtest app, RQ-13), 114 (12 genres at Early Access; §2.3 (g) gating no longer applies), 122 (RQ-11: USD 9.99); add doc 15 tests 41 and 42 to the coverage table.
- **11**: for a Keepsake, step 2's eligible templates come from the opener's account content set at open time.
- **12, 13**: confirm that a fused Second Edition Folio records both genres (doc 15 §5.1 credits Reliquaries to the pair's lower Canon §5.2 row) and that the revised Editors carry `rank.editor`, so Editor-kill Ink, XP and Reliquaries apply to them.

### Unresolved

- The Transcription shelf (§3.9.1) is still a fixer design addition made to meet DECISIONS 79; doc 14 §5.4a has adopted it, but the product owner has not ruled on it.
- Second Edition reward details that DECISIONS 108 leaves open are chapter rulings here: the Ur-Text victory pay is forfeited with the clear, and the Ironman cosmetic is a portrait frame, `cosm.frame.ironman`. The product owner may prefer a different cosmetic kind.
