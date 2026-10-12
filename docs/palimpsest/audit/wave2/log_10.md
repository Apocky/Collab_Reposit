# Wave 2 log, chapter 10

## Round 1, 2026-10-12

| Item | Disposition | Where |
|---|---|---|
| fix:14 stale 'doc 14 Canon gap 16' | already satisfied | Canon gaps rewritten; no hit on disk |
| fix:07 block.altar sells appr.svc.* | already satisfied | §5.9, Imports doc 07 row |
| fix:11 cat.wand to Imports, gap 25, gap 24 | already satisfied | Exports has no cat.wand; gaps closed; Interface row 11 |
| fix:15 'Item Codex' parentheticals | already satisfied | no hit on disk |
| verify:11 'Still asked' in row 11 | already satisfied | Interface row 11 |
| verify:09 witness uses erratum_sees | edit | §5.3 Witness, Imports doc 09, Interface row 09 |
| fix:06 shrine lamp as placed altar | already satisfied in §5.9 and row 07; edit | invariant 'Erratum-placed altar' renamed to block.shrine_lamp |
| fix:10 / verify:10 unresolved notes (hedge false positive, value model, scratch, mass curses, sealed Volume rewards) | already satisfied / no action | lint now clean; design notes stand per doc 11 |
| wave2:07 drop cat.seed, write 26 | edit + canon_request | §1 mundane list; Canon gaps; test 44 |
| DECISIONS 128 | edit | §1, Imports |
| Leftover sweep | clean | Crowns already used; 'seed' hits are run seeds and descriptor nouns 'seed'/'dream-seed' (appearance art, not planting); durability line already states items have none |

- Batch 1: header, §1 cat.seed removal and 26 count, §5.3 witness, Imports 09, Interface 09, invariant, Canon gaps, test 44. 9 edits. lint --chapter 10 clean.
- Batch 2: invariant alignment, Imports DECISIONS row. 2 edits. lint clean.

Final lint: `== 10 words=21248`, no findings; full run `IDs defined in more than one chapter: 0`, only baseline finding (16 aware.profile.invader).

- Round 1 re-check, 2026-10-12: chapter, log and outbox already on disk from this round; lint --chapter 10 clean; full run "IDs defined in more than one chapter: 0"; sweep re-grep 0 hits. No new edits.
