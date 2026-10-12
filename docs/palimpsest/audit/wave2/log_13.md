# Wave-2 fixer log, chapter 13

Started 2026-10-12. Inputs: requests_13.json (20 items), DECISIONS rows 89-129.

## Progress
- [x] Read chapter 13, requests_13.json, DECISIONS.md
- [x] Read inputs: doc 14 §3.1/§3.2/§3.8/§5.3a/§8.9, doc 09 §9.4/§11.1/§13.5, doc 03 §3.4/§5.13/§14.3, doc 16 §6.6, doc 18 §3.6/§3.8, canon §9/§11.6/§13.8/§17/§20
- Baseline lint: clean (0 findings).
- Triage: fix:14 gap-2 (Erasure) already gone; gap-20 -> name unlock.invader.* in §10.7. Cavalry rename: no spec occurrences left, nothing to do.
  Erasure onset conflict: doc 14 §3.8 now uses 43200, resolved.
## Plan of edits
1. §2.1: visual field, crouch_mm, net_always flag, knockback_mm_s attack field; outfit rule; size rule.
2. Knockback distances -> speeds (brute charge, iron_palm, huff_and_puff, tail_sweep, fee_fi_stomp).
3. §4.13 mantle link + tall-humanoid crouch; humanoid swarmer nav override; flow field per nav class.
4. §4.1 host migration note (DECISIONS 93); traces/weather note (96, 99).
5. §9.4 Second Edition bosses (revised Editors + the Author); §10.8 fused Folios; §7.3 fused axioms.
6. §11.1 kill credit -> party_kills; pacification methods 0-4 (subdue method 4 from ev.takedown).
7. §10.7 invader unlock IDs; §13.5 glitch perk (no buildup); §13.6 no Margin relationships; Interface rows.
8. Exports, Canon gaps, acceptance tests, invariants.
- [x] Batch 1 (§2.1): `visual` field (fix:18, verify:18), `body_state.crouch_mm`, `net_always` flag (verify:16), attack `knockback_mm_s` (verify:13 knockback units), JSON example updated, Outfits rule (fix:09, verify:09; all 43 native factions verified to hold a humanoid sapient template), Size rules (humanoid brutes >= 2200, crouch bounds).
- [x] Batch 2: knockback distances -> `knockback_mm_s` speeds (brute charge 6000, iron_palm 9000, huff_and_puff 8000, ember wyrm tail_sweep 6000, cloud giant fee_fi_stomp 8000; ram and war boss reworded); humanoid swarmers on ai.nav.humanoid; flow fields keyed by (target, nav class); mantle nav link (DECISIONS 95); Low cells crouch rule.
- [x] Batch 3: §4.1 Host migration paragraph (DECISIONS 93) and Inputs-the-AI-never-reads (96, 99); §2.2 SE depth 25+2n (106); §7.3 aspect_set takes a fused pair's averaged axioms.
- [x] Batch 4: §9.1 templates/Erasure/genre adaptation/loot lines for SE; new §9.4 (6 boss templates, rank.editor, dominant-axiom revised-Editor choice, 4 revision rules + phase-4 attacks, revised Last Word, the Author as writing hand on body.amorphous, entry staging, 6 attacks, HP table computed in Python with canon recurrences).
- [x] Batch 5: Author entry returns removed segments; §10.4 boss row (Author's death); §10.2 step 4 fused; §10.7 invasions post-EA (118, 125) and unlock.invader.* IDs with gates (fix:14); new §10.8 fused Folios with worked faction-draw example.
- [x] Batch 6: §11.1 ev.entity_pacified methods 0-4 (subdue method 4 from ev.takedown lethal false, fix:09/verify:09) and Kill credit -> party_kills/party_pacified (verify:14); glitch perk without buildup; §13.6 no Margin relationships (127), "typed run seeds" (128 wording); §8 Expansions and mods (121, 126); Interface rows 09 and 14 (unlock.invader.*).
- [x] Batch 7: Interface rows 03 (crouch, mantle), 12 (SE arenas, fused Folios, preview), 15 (rank.editor kills), 16 (net_always, migration), 18 (visual, Author), new 19 (schema fields); stray blank line in §11.1 list removed.
- [x] Batch 8-10: Imports rows (Canon §20, doc 03 §5.8/§5.9/§14.3, doc 09 ev.takedown, doc 12 G19, doc 14 §1.2/§5.3a/§8.9, doc 16 §6.6/§9.3, new doc 18 row, decision ledger items); mantle link made exact against doc 03 §5.9 (origin column ceil_div(2000 + crouch_mm, 1000) cells); lint: fixed one undefined art.visual ID (now cites doc 18's ghoul record).
- [x] Batch 11-14: Exports boss.* 33->39 and boss attacks 119->155 (counted programmatically), revised-Editor choice rule exported; Canon gaps item 3 rewritten (now canon-only: fused rule, pause, glossary); acceptance tests 60-71; invariants added. Lint clean.
