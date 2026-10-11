# ID reconciliation: run cluster (owner doc 14)

Namespaces: `run.`, `mode.`, `grade.`, `margin.`, `unlock.`, `echo.`, `score.`, `ach.` (Canon §19; Echo acquisition is doc 15's). Owner chapter: `spec/14_RUN_STRUCTURE_PROGRESSION.md`. No other chapter was edited.

Inputs:
- `tools/spec_lint.py --json`, run before and after the edits.
- A scan of every chapter for these prefixes in backticks, in JSON string values, and as raw tokens, including pattern forms.
- A full read of doc 14.
- Canon §1.1, §13.2, §16, §17, §19 and §23 (canon-1.4 to canon-1.10 changelog, including canon-1.10, which landed during this pass).
- Doc 15 §3.1.1, §3.1.2 and §3.2 (pool order), doc 04 §2.6 and §13, doc 05 §3.2 and §8.1, doc 11 §3, and doc 17's Margin requests.

## Inventory

Before the edits, chapters used 264 distinct concrete IDs in these namespaces. 30 of them were not defined in doc 14 in a form the lint reads:

- `margin.milestone.*` ×5
- `score.*` components ×8
- `run.kit.universal`
- `run.sav`
- `unlock.glyph.*` ×6, used by doc 06
- `mode.ff_off`, `mode.ff_full`, `mode.loot_shared` and `mode.invasion_open`, defined only by the canon
- `echo.sun_wukong`, `echo.loki`, `echo.lilith`, `echo.fenrir` and `echo.jormungandr`, defined only by the canon and used by doc 18

Three more uses fall outside the lint's view:

- `unlock.kit.gunslinger_script` appears in a doc 15 JSON value.
- Doc 17 uses the pattern `margin.slot.*` and asks doc 14 for slot records.
- Doc 15 requests `run.kit.<calling>_script` and its §3.1.2 `unlock.` grants.

After the edits, doc 14 defines 429 concrete IDs, up from 335, and lists every one in Exports. All 429 pass the grammar and have distinct `fnv1a32`, checked against all 4,843 IDs in the spec. 434 IDs are used across the spec in these namespaces. The 5 that doc 14 no longer defines are the retired Echoes in the map.

## Decisions

1. **`margin.milestone.*`** (docs 01, 14, 15): **defined.** The milestone table listed bare suffixes such as `prologue_done`. Its first column now carries the full IDs, and a sentence says that short names stand for the full IDs. No rename.
2. **`score.progress`, `.editor`, `.speed`, `.explore`, `.notes`, `.treasure`, `.hunters`, `.deaths`** (doc 14): **defined.** These appeared only in prose and as JSON keys. A component table in §4.2 now gives each ID's term and sign.
3. **`run.kit.universal`** (docs 04, 14): **defined.** It appeared only in a heading. It is now listed with every kit ID in §2.4.2 and in Exports.
4. **`run.sav`** (doc 14): **not an ID.** It is a file name that happens to match the grammar. It now reads `runs/run.sav` in §12, so the lint stops reading it as an ID. No map entry.
5. **Script grants** (docs 06 and 15): **defined.** This covers `unlock.glyph.{ember,hollow,mirror,storm,thorn,tide}` (doc 06) and `unlock.kit.gunslinger_script` (doc 15 JSON), and fulfils doc 15's Interface-assumptions request.
   - §2.3 now has a table of all 48 launch Script grants:
     - `unlock.room.lost_chapter_<g>` ×12
     - `unlock.hireling.chapter_<g>` ×12
     - `unlock.glyph.<family>` ×6
     - `unlock.hireling.{apothecary,cartographer,lamplighter,porter,sellsword}`
     - `unlock.kit.<calling>_script` ×13
   - Each row gives its doc 15 bundle and what it adds to the account content set.
   - The reserved patterns stay in place for Expansions.
6. **`run.kit.<calling>_script` ×13**: **defined** in §2.4.2. Each is a variant-2 replacement record against the standard kit.
   - Swapped templates are priced at doc 11 §3 base values and doc 05 §8.1 ammunition values.
   - The differences from the standard kits are 0, +322, +219, −64, +232, −239, 0, −81, +218, 0, +123, 0 and +147 Bp (Vanguard to Monk). All are within the 500 Bp kit rule.
   - Added: the cloth set, accessory slot placement, a `run.kit.gunslinger_script` JSON record, and acceptance tests 38 and 39.
7. **`mode.ff_off`, `mode.ff_full`, `mode.loot_shared`** (docs 00, 01, 05, 10, 11, 14, 16): **defined.** They were canon-only. §8.1 now has an overlay table that gives each one's slots, effect, storage and ranked status.
8. **`mode.invasion_open`** (docs 00, 02, 15, 16): **defined** as a fourth overlay. Canon §17.9 makes doc 14 a detail owner. Doc 14's rules for it:
   - It applies to 1 to 4 slots.
   - It is never ranked, and Daily, the Weekly Gauntlet and the Prologue never offer it.
   - Lobby preconditions follow doc 16 and doc 15's `juris.` flag.
   - `RunState.invasion_open` stores it.
   - A Redaction replaces the §3.2 row 1 run end.
   - Single-player Suspend waits for the invasion to end (doc 02 G11).
   - The invader never counts toward party quantities or score.
   - It is excluded from boards.
   - Also added: an invariant and acceptance test 40.
9. **Required public-domain Echoes** (docs 00 and 18): **defined.** Canon §16.2 (canon-1.6) requires these five inside a 24-Echo roster with 2 per core genre. Each takes the place of one original Echo in the same genre and keeps that place's tier, Inkwell tier and `est_ch`. Trial totals (16,839 ch), tier bounds and doc 15's banner rotation and hours do not change.

   | New ID | Replaces (now mapped) | Roster place | Calling, kin, body |
   |---|---|---|---|
   | `echo.fenrir` | `echo.aldric_unsworn` | MF, Tier 1 | Monk, beastkin, `body.quadruped` |
   | `echo.lilith` | `echo.vesper_crane` | GH, Tier 1 | Shade, fae; trait kept |
   | `echo.jormungandr` | `echo.dr_halloway` | CH, Tier 1 | Oracle, beastkin, `body.serpentine`; trait kept |
   | `echo.sun_wukong` | `echo.old_master_huo` | WX, Tier 2 | Monk, beastkin |
   | `echo.loki` | `echo.sibyl_of_ash` | MA, Tier 2 | Oracle, human |

   - **Fenrir:** a quadruped has no off-hand for the Vanguard shield, so Fenrir is a Monk. Its trait is `damage_dealt.unarmed` +2000 Bp with `buy_price` ×11000, scoring 2/1/1. Its trial uses 200 sneak kills instead of a damage-type step, because beastkin unarmed hits split blunt and slash 50/50 and that tie has no rule.
   - **Sun Wukong:** his trait is polearm damage +1500, move speed +500 and damage taken +500, scoring 3/1/2.
   - **Loki:** his trait is Illusion potency +2000 with prayer healing ×7500, scoring 2/1/1.
   - **Why Sun Wukong takes the Tier 2 place:** the first draft put him in Lan of the Ninth Peak's Tier 1 place. A dry run of the map showed that this breaks doc 15's ID-sorted `rate_up_mid` list for `banner.echo.p001`, which doc 15 §3.2 requires to be sorted. It would also have broken the pull vectors built on that list: doc 15 acceptance test 8, Example F, and doc 16 acceptance test 51 with its §11.4 JSON. With the final choice, p001's list maps to `["echo.fenrir", "echo.lan_ninth_peak", "echo.nix_seventeen"]`, which is still sorted, and doc 16 needs no change.
   - **Calling counts** are now Alchemist and Monk 3; Vanguard, Warden, Netrunner and Beastbinder 1; every other Calling 2.
   - **Other updates:** the Inkwell binding table (it now cites doc 15 §3.1.1, not the obsolete §3.13 placeholders), the Echo record, the §2.4.2 slot rule for non-humanoid body plans, acceptance tests 18 and 42, and Canon gap 22.
10. **Echo trait IDs**: the map also renames the five retired `trait.echo.*` IDs. These live in doc 04's namespace, but doc 14 fixes their names. With the rename, doc 04's §15.6 register rows follow the roster when the map is applied. `map_character.json` is `{}`, so there is no conflict.
11. **`echo.slot_mf_b`** (doc 14 JSON): **removed.** It was a placeholder for doc 15 §3.13 slots that doc 15 no longer uses. The Echo record now carries `"inkwell_tier"`. No map entry, because it was used nowhere else.
12. **`margin.plot`** (docs 01, 14, 17, 18): **redefined; the ID is kept.** Canon-1.4 §17.8 replaced the building plot with fixed trophy and decoration slots, and docs 01, 17 and 18 all flag the conflict.
    - The row is now the trophy alcove.
    - The new §5.1a defines the 32 records doc 17 asked for: `margin.slot.trophy_01`–`12`, `wall_01`–`08`, `floor_01`–`06`, `banner_01`–`03` and `light_01`–`03`. They use exactly doc 17's five kinds and stay within its limit of 32. Each has a footprint, a centre position and an opening rule: 8 open at `first_run_end`, then one per 2 achievements, so all are open at 48.
    - `unlock.palette.<g>` keeps its ID and price and now grants an alcove finish. No Bindery total changes.
    - The ID was kept rather than renamed. A rename would make the map rewrite the gap notes in docs 01, 17 and 18 into false statements.
13. **Pattern and non-ID tokens.** No action is needed on these:
    - Patterns: `echo.<name>`, `run.kit.<calling_suffix>_script`, `unlock.kit.<calling_suffix>_script`, `unlock.room.lost_chapter_<genre_suffix>`, `unlock.hireling.<type>`, `margin.slot.*`, `run.end.*` and `mode.**`.
    - Other namespaces whose text contains these prefixes: `kpi.run.*` (01), `npc.margin.*` (13, 18), `nar.margin.*` (18), `banner.echo.p<nnn>` (15, 16), `art.visual.echo.*` (18) and `cosm.echo.*` (15).
    - Code fields `run.grade` and `run.next_uid` (11), and the file `run.palr` (16).
14. **`run.marg.brittle_kit`** (docs 07, 14): **redefined; the ID is kept.** Canon moved to canon-1.10 during this pass and removed item durability. The mutation scaled durability loss, so it had no effect left.
    - Its new effect is `brittle_bp = max(7500, 10000 - 500 × r)`, applied in the §7.3 `m13` chain on Errata hits against NPCs.
    - Keeping it keeps 12 Marginal mutations and the "rank 5 by Folio 321" arithmetic.
    - Doc 14 no longer assumes doc 07's durability hook.
    - **`echo.ada_brassvane`** loses its `crafted_durability` drawback. The replacement is `sell_price` ×9000, which is also 1 point, so the trait still scores 1/1/0.
15. **Doc 15 co-definitions.** Doc 15 lists `mode.*` and `echo.*` IDs in the first column of its own tables. The lint accepts this because Canon §19 names doc 15 for Echo acquisition. The spellings match doc 14's, and the map renames the 5 retired Echo rows. Nothing needs adopting.

## Follow-ups for other owners (not edited here)

- **Doc 04:**
  - §13 still says "No launch Echo uses a non-humanoid plan". Fenrir and Jörmungandr now do.
  - The §2.6.1 budget table names bare suffixes (`aldric_unsworn`), which the map cannot rewrite. Add rows for `fenrir`, `sun_wukong` and `loki` using the scores in doc 14 Interface assumptions.
  - Confirm that beastkin's added tail is skipped on `body.serpentine`.
- **Doc 07:** drop the `run.marg.brittle_kit` term from the wear formula (doc 07 §8.2, its Imports row and its Interface assumptions row for doc 14). Durability itself is gone under canon-1.10.
- **Doc 05:** bite, claw and pounce skins (quadruped) and bite, constrict and spit skins (serpentine) for `arch.melee.unarmed`.
- **Doc 13:** `npc.companion.dr_halloway` shares its name with a retired Echo. This is harmless, since the map rewrites only whole tokens.
- **Doc 15:**
  - After the map is applied, check the hours, Fold and Archive rows for the five renamed Echoes. Their tier, genre and `est_ch` are unchanged.
  - Add `body` to Echo catalog records where they are copied.
- **Docs 06, 12 and 13:** provide the content behind the 48 Script grants (4 glyphs per family, 3 room variants per lost chapter, 1 hireling type per hireling unlock). Doc 15 already requests this.
- **Doc 14:** still conforms to canon-1.3 on several rules that do not touch IDs: Erasure onset and hunter cadence (canon-1.6), the account Glyph Library (canon-1.6), and the Prologue L3 mining lesson under fixed terrain (canon-1.4). These are recorded as Canon gap 26 and need a content pass.

## Unresolved

None.
