# Wave 2 fixer log, doc 11

## Round 1, 2026-10-12

Baseline lint (--chapter 11): clean; full run "IDs defined in more than one chapter: 0".

### Triage

| # | Item (from) | Disposition | Where |
|---|---|---|---|
| 1 | shop.stablemaster, mount mending, mount tokens (fix:13) | already satisfied | §12.1 (10 shops, Σ 17), §12.3, §3.10 note, test 44 |
| 2 | doc-14 former-name rows, deep pockets gap (fix:14) | already satisfied | Interface former-name table, Canon gaps preamble (G-224) |
| 3 | doc 05 §10.5 hook citations, jade_lotus throw, rustmaw reload scope (fix:05) | edit | §5.2 rows and note |
| 4 | lines x-patchable (fix:02) | already satisfied | §1 principle 8 |
| 5 | grappling tool record (fix:01, verify:01, verify:17, inbound wave2:03) | edit | §2.2 `ready_tick`, §3.5 `item.tool.grapple` and skins, §12.1 general store, test 46 |
| 6 | §10.5 citations for deepanvil, grimhallow, neon_saint; neon_saint first cell; grimhallow refusal (verify:05) | edit | §5.2 |
| 7 | Second Edition loot rules, ENH_CAP (verify:14, DECISIONS 102, 106) | edit + canon_request §13.7 | §16.1, §16.2, new Second Edition paragraph, test 50 |
| 8 | item durability (fix:07) | already satisfied | §1 principle 9, test 44; toolkit row text aligned to request |
| 9 | lockpicks × 1; gadget dependency per doc 07 §11.1 rule 3 (verify:07, verify:09) | edit | §12.1 shop.tinker, §3.6 |
| 10 | implants_2 list in the pool row (verify:04) | edit | §2.5, Interface row 04, test 48 |
| 11 | Keepsake step 2 content set (verify:15) | edit | §2.3 Keepsake entry point, test 53 |
| 12 | services rows (fix:17) | already satisfied | §12.3 |
| 13 | story relic fill; folio_leaf consumption (fix:12, verify:12, verify:11, fix:11) | edit + product_question 11-Q1 | §2.2 `story_domains`, §9.6 story relic, §3.5 folio_leaf row, test 51 |
| 14 | item.text.inscribed source (fix:06) | already satisfied | §3.5 |
| 15 | §12.2 hidden part of eff_enh (fix:10) | already satisfied | §12.2 |
| 16 | curse.greed in Lucre find; leaden / bulwark carried_g; ring cap 24 (verify:10) | edit | §9.1, §14.1, §3.4, test 49 |
| 17 | Canon gap 1 Abridged kill drops (fix:11, verify:11) | canon_request §17.1 | Canon gaps 1 |
| 18 | hedge false positive 'might' (fix:11) | already satisfied | lint ignores backticked IDs; lint clean |
| 19 | respirator alias (fix:11) | already satisfied | lint clean for 11; doc 08 side is doc 08's |
| 20 | doc 14 former IDs (fix:11) | already satisfied | doc 14 names owner IDs (Interface row 14) |
| 21 | raw-liquid bottles in chests and apothecary (verify:11, doc 08) | product_question 11-Q3, default no line | none |
| 22 | doc 09: outfit_of, door keys with key_tag, tool qualities, container lock (verify:11) | edit | §2.2, §3.5 Keys, §9.6, Interface row 09, test 52 |
| 23 | loot.tpl. records / duplicate template IDs (verify:11, doc 19) | edit + cross_chapter_request 19 | §2.1 (keyed by owner ID via x-idref), test 54 |
| 24 | cat.seed retired (DECISIONS 128, inbound wave2:07) | edit + canon_request §13.1 | §3.1, Exports cat. 26, test 45 |
| 25 | ghostline Projectile.home_bam_s / home_acc (inbound wave2:03) | edit | §5.2 ghostline row |
| 26 | DECISIONS 90 Crowns | already satisfied | no "Nobles" in chapter |
| 27 | Leftover sweep | done: no hits needing change ("Seed Spitter" is a weapon skin name, not planting; "Epic" is a rarity) | whole chapter |
