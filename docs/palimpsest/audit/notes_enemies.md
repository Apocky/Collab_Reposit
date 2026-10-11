# ID reconciliation: enemies cluster (owner doc 13)

**Namespaces:** `enemy.`, `ai.`, `fac.`, `boss.`, `npc.`, `director.`, `rank.` and `elite.` (Canon §19: "13, with core rank values in 00").

**Owner chapter:** `spec/13_ENEMIES_AI_FACTIONS.md`. It was the only chapter edited.

**Inputs:**
- the output of `tools/spec_lint.py --json`, run before and after the edits;
- a scan of every chapter for these prefixes: backticked tokens, JSON string values and bare prose tokens.

**Linter blind spot.** The linter never flags a foreign definition in these namespaces, because their owner set includes 00. Foreign definitions were therefore checked by hand with `spec_lint.defined_ids`.

## Inventory

**Before the edits.** About 300 distinct concrete IDs in these namespaces were in use. Outside doc 13, they appeared in docs 00 to 07, 09 to 12 and 14 to 18.

**Consumer IDs that doc 13 did not define:**
- `enemy.mythic_fantasy.troll` (04)
- `ai.action.breach` (07)
- `director.profile.standard`, `.boss` and `.rest` (12): defined only in prose, so the linter could not see them
- the roster value `enemy.gothic_horror` (12)
- `npc.vendor.<type>` for `shop.colophon` and `shop.wanderer` (11)
- `npc.companion.<origin_suffix>` for the four secret Origins (14, with doc 04 §2.1)
- `unlock.hireling.chapter_<g>` (14, 15): this did not match doc 13's `npc.hireling.<g>_chapter`

**After the edits:**
- Every concrete ID in these namespaces that any chapter uses is defined in doc 13 or mapped. The one exception is the 11 Canon-defined IDs listed in item 12.
- `spec_lint` reports 0 undefined IDs in these namespaces in any chapter.
- Doc 13 now has 0 foreign-namespace definitions.

**Mapping.** `map_enemies.json` holds 13 renames:
- `enemy.mythic_fantasy.troll` → `enemy.mythic_fantasy.cave_troll`;
- the 12 `npc.hireling.<g>_chapter` → `npc.hireling.chapter_<g>`.

A dry run of `apply_id_maps.py` shows no conflicts with the other maps.

## Decisions

1. **`enemy.mythic_fantasy.troll`** (doc 04 §9 JSON `source_template`) is mapped to `enemy.mythic_fantasy.cave_troll`.
   - That entry is the Mythic Fantasy troll. It is a humanoid body, so it has `bp.arm_r`.
   - Doc 13 §6's "tamed troll" is the same brute.
2. **`ai.action.breach`** (doc 07 §10.5 and its Interface row) is defined as new content. Doc 07 makes Errata barricade, cover and gate cells breach links and names the action.
   - **§4.7.** A new action row (weight 7500) makes breach a channel action. The agent strikes once every `cycle_ticks` with `damage_block(cell, B1, shares, agent, 5000)` until the cell breaks. It takes no token, publishes no telegraph and draws nothing.
   - **§4.6.** The channel rows now include `breach`.
   - **§4.13.** A new "breach" link row uses doc 07's cost formula, written in doc 13 terms (`B1`, `cycle_ticks`, `placed_meta.hp`). The locked-door row sends `block.barricade.gate` to it.
   - **Common actions.** `breach` is a common action for ground nav classes. The soldier set budget becomes 10 extras, 11 common actions and at most 3 abilities, which stays within 24.
   - **Acceptance test 53.** This test matches doc 07's figure of "12 hits at Folio 1".
   - **Counts.** Exports now says 34 actions.
3. **`director.profile.*`** is now a definition table in §10. It gives the Folio types, `pop`, initial phase, reserve waves, scripted waves and Erasure-clock start for each profile.
   - Doc 12 §6.2 says rest Folios get "no ambient or scripted waves". §10.4's scripted-wave sentence now also excludes rest Folios, which removes a contradiction.
   - New acceptance test: 54.
4. **`npc.vendor.colophon` and `npc.vendor.wanderer`.** Doc 11 §12.1 adds `shop.colophon` and `shop.wanderer` and says there is one `npc.vendor.<type>` per `shop.<type>`. Both vendors are now defined, so there are 11 vendor types.
   - **Wanderer.** It is `fac.neutral` (Canon §15) and follows the normal vendor rules.
   - **Colophon.** It is `fac.neutral`, invulnerable and non-combat, which keeps the Colophon a sanctuary. It also closes a free-loot exploit: a `fac.neutral` vendor has no reputation cost, and one stands in every Folio. See question Q4.
5. **Beast companions for secret Origins.** Doc 14's Beastbinder kit uses `npc.companion.<origin_suffix>`, and doc 04 §2.1 and doc 14 unlock secret Origins.
   - The four IDs `npc.companion.oneiric`, `.redacted`, `.unwritten` and `.ur_text` were missing. They are now defined in §13.5 as skin-only beasts with no extra tags.
   - Canon gap 18 and the Exports were updated. The Exports now list 28 companions.
6. **Chapter hirelings** are renamed `npc.hireling.<g>_chapter` → `npc.hireling.chapter_<g>`. This makes the hireling ID's last segment equal the `<type>` in doc 14 and doc 15's `unlock.hireling.chapter_<g>`. The 12 renames are mapped.
   - **Why base entries changed.** The old rule was "that genre's soldier-role entry". Mythic Fantasy, Cosmic Horror, Weird West and Wuxia have no soldier, so the rule was undefined for them. §13.4 now has a table that names the base entry for each genre.
   - **Selection rule.** Use the soldier entry if it is sapient and its signature calls no allies. Otherwise use the listed sapient entry.
   - **Departures from the soldier entry.** Noir uses `mob_torpedo`, because `crooked_cop`'s `call_backup` summons allies. Steampunk uses `sky_aeronaut`, because `brass_automaton` is not sapient. Mythic Antiquity uses `hoplite`, because `shade_warrior` is mindless.
   - See question Q5.
7. **Roster IDs.** Doc 12's GPV field 52 holds `"roster": "enemy.gothic_horror"`.
   - §8 now defines roster IDs: `enemy.<genre_suffix>` for the 15 genres that have native rosters, and `enemy.editors` for the Editor squads.
   - The Unwritten's roster is `enemy.editors`, because Canon §5.2 and doc 12's `law.unwritten.editors_only` limit it to Editors.
   - No rename.
8. **Attack-ID collisions in `enemy.ab.*`.** These were in doc 13's own namespace.
   - **The collisions.** `strike_through` had three different profiles: the squad, the Revisor hunter and the revisor kit. `redact`, `margin_lunge`, `annotation`, `ibid` and `blot` each had two.
   - **Fix for kits.** Kit attacks are now `<kit id>.<attack>`, mirroring `<boss id>.<attack>`. An example is `enemy.editors.kit.revisor.strike_through`.
   - **Fix for the hunter.** The hunter's attack is now `enemy.ab.hunter_strike_through`.
   - **Why there is no map entry.** The old full IDs never appeared in any chapter, and `enemy.ab.strike_through` remains valid for the squad.
   - **Other updates.** Test 47, test 55 and Canon gap 26 were updated.
9. **Foreign definitions in these namespaces by other chapters.** All of them agree with doc 13, so no adoption or rename was needed. Each fix belongs to the other chapter, which should move the ID out of its first column or JSON `id`.
   - **Doc 02** has a curve-generator JSON with `ai.curve.logistic_up`, `bell_mid` and `plateau`. Its parameters reproduce §4.4's tables exactly.
   - **Doc 05 §15.2** defines `elite.warded`, `surging`, `unstoppable`, `armored` and `phasing` in a table.
     - The data is consistent with doc 13's. Doc 13 §7.2 now restates the exact data so the owner holds it: the ward pool `mul_bp(max_hp, 5000)`, surging at 200 Bp/s with `k = 60`, and unstoppable immune to poise-driven `st.staggered` and `st.knocked_down`.
     - The last of these is doc 05's refinement of Canon §17.3a, adopted because doc 05 owns poise.
   - **Doc 12 §21.2** has an arena table with first cells `boss.editor_v1` to `v4` and `boss.ur_editor`. The arena dimensions match doc 13 §9.2.
   - **Doc 11** has `rank.boss`, `rank.editor`, `rank.hunter` and `fac.neutral` in the first cells of loot and shop rows. These are uses, not definitions.
10. **IDs the linter could not see in doc 13** were made explicit.
    - **The gap.** Role abilities, roster entries, factions, givers, captives and companions were defined only through suffix tables.
    - **Exports.** Every concrete ID is now listed in full: 104 roster entries, 8 Editor templates, 4 kits, 16 kit attacks, 131 `enemy.ab.*`, 43 native and 16 wild factions, 19 bosses, 64 boss attacks, and every `npc.*`, `director.*` and `elite.*` ID.
    - **Bare family tokens.** `fac.native`, `fac.wild` and `npc.captive` became `fac.native.*`, `fac.wild.*` and `npc.captive.<genre_suffix>`.
11. **Layout cleanups in doc 13 for other clusters.** The world and stealth notes asked for these, and doc 13 is the only chapter allowed to make them. Each ID was moved out of a definition position:
    - the kin table's first cells, now plain kin names;
    - `aware.alert`, `ev.player_downed` and `st.dread` in the intensity and morale rows;
    - `gpv.evt.*`, `lattice.challenge.siege` and `law.*` in the scripted-wave table, which gained a "Wave" label column;
    - `room.arena` in the sensitivity table;
    - `sys.ai_think` in a table header;
    - the `afflict.red_ink` JSON-style text;
    - `comp.tags` in the Exports.

    No meaning changed.
12. **Core factions and ranks.**
    - **Factions.** Canon §15 defines `fac.errata`, `fac.editors`, `fac.margin` and `fac.neutral`, and doc 13 §12 owns their rules.
    - **Ranks.** Canon §17.3 defines the seven `rank.*` IDs as core values.
    - **How Exports lists them.** By name, without backticked IDs. A backticked list would only create new "multiply defined" reports against doc 11's first-cell uses.
13. **Consumer data requests on doc 13 IDs.**
    - **Doc 08 gap 18.** It read `elite.miasmic`'s "1000 ml" as one saturated cell. `elite.miasmic`, `steam_ram` and `boiler_vent` are now stated as `emit_gas_volume(cell, gas, 1)`.
    - **Doc 09 §4.1.** It says "hounds are named explicitly by their templates", but no template did so. `forest_wolf` and `big_bad_wolf` now set `perception` to `aware.profile.hound`.

## Questions for the product owner (orchestrator: please relay)

- **Q1. Mounts versus cavalry.** Decision 55 and Canon §17.7 cut mounts, and the linter now reports retired IDs in doc 13: `arch.mount.hover`, `arch.mount.runner`, `cat.mount`, `room.no_mount` and `st.mounted`.
  - **Doc 13 IDs built on mounts:** `enemy.role.cavalry`, `ai.set.cavalry`, `ai.nav.mounted`, `ai.action.remount`, `enemy.ab.lance_charge`, the `mounted` tag, `npc.vendor.stablemaster` (doc 11 `shop.stablemaster`), the War Boss rig, and the 8 cavalry roster entries:
    - `enemy.mythic_fantasy.kingsguard_lancer`
    - `enemy.space_opera.hover_lancer`
    - `enemy.post_apocalypse.war_rig_rider`
    - `enemy.weird_west.posse_rider`
    - `enemy.hardboiled_noir.getaway_driver`
    - `enemy.wuxia.cavalry_general`
    - `enemy.fairy_tale.wild_hunt_rider`
    - `enemy.ur_text.archetype_knight`
  - **The choice.** Retire them, or convert cavalry into an unmounted "charger" role (for example, a running lance charge that keeps the 8 entries as infantry)?
  - **Status.** This is a design decision, not an ID fix, so it is not applied here. Canon gap 11 now records it.
- **Q2. Roster size (decision 69).** Doc 13 has 6 to 8 entries per genre and 1 mini-boss per genre. Decision 69 asks for 12 or more entries and 2 or more mini-bosses per genre.
  - The expansion is pending.
  - Q1's answer decides whether the cavalry slots are refilled as part of it.
- **Q3. Erasure hunter (decision 26, Canon §11.6).** Canon now spawns one Editor hunter with `rank.elite` stats and no repeats.
  - Doc 13 §10.5 still spawns `enemy.editors.revisor` at `rank.hunter`, with a repeat every 5400 ticks. Test 19 and the Exports constants depend on this.
  - Doc 13 needs a decision-propagation pass, which is outside ID reconciliation.
  - `rank.hunter` stays valid for the invader.
- **Q4. Colophon vendor.** Confirm that it is invulnerable and non-combat. The alternative is the ordinary killable elite vendor.
- **Q5. Chapter hirelings.** Confirm the §13.4 base entries:
  - Mythic Fantasy: kingsguard knight
  - Cosmic Horror: expedition rifleman
  - Weird West: outlaw gunhand
  - Noir: mob torpedo
  - Wuxia: sect disciple
  - Steampunk: sky aeronaut
  - Mythic Antiquity: hoplite

## Out of scope, noted only

- Doc 07's other Interface requests to doc 13 contain no IDs in these namespaces:
  - door links using `break_ticks(cell)`;
  - trap knowledge per squad;
  - `disabled_until`;
  - the template `materials` list.
- **`gpv.evt.plea` captive faction.** Doc 12 calls the plea captive `fac.neutral`, but doc 13 §13.1 gives captives the trader faction. Doc 12 should cite doc 13.
- **Other clusters' maps.** `loot.drop.boss`, `loot.drop.editor` and `block.dirt` in doc 13 are covered by the items and materials maps (`loot.boss.mini`, `loot.boss.editor`, `block.soil`). `rng.check` belongs to Canon.

## Unresolved

The Q1 mount-dependent IDs are listed in the structured output. Their meaning is clear, but whether they remain valid depends on the product owner's answer.
