# 01 — Vision, Pillars and Inspiration Ledger
As of 2026-10-11. Status: BINDING. Conforms to canon-1.2.

This chapter fixes what PALIMPSEST is, what it must feel like, which games it learns from and how its success is measured. Every other chapter answers to the pillars in §2. When a system choice is ambiguous, the pillar tests decide. This chapter draws no random numbers and adds no simulation state.

## Imports

| Source | Section | Consumed here |
|---|---|---|
| DESIGN_BRIEF | §1–§9 | high concept, Archetype × Skin, axioms, run shape, tech, attributes, monetization stance, document contract |
| DECISIONS | 2026-10-11, rows 1–24 | product-owner answers: Standard plus Abridged, first person, Bevy 0.20, PC and Deck first, wry literary tone, 16+ gore, PvP invasions, limited building |
| Canon | §1.1, §1.5, §1.6, §2.1–§2.4, §4.2, §4.3 | ID grammar, `fnv1a32`, integer content; 60 Hz, "N s" = 60·N ticks; dodge; placeables, fixed terrain |
| Canon | §5.1–§5.11 | axioms, 16 genres, Contradiction, Anachronism Edge, Shock of the New, seam zones, World Laws, Going Native, Unmoored |
| Canon | §6.4, §7.2, §8 | derived stats, skill pace contract, level target 28 ± 3 at Folio 25, `kill_xp_std` |
| Canon | §11.6, §12.1, §12.3 | Erasure, surfaces and reactions, spawn light |
| Canon | §13.3–§13.9 | rarity, Illuminated, Emendations, power tables, item contract |
| Canon | §14, §16, §17 | currencies, Callings and Echoes, run constants, party tables, anchors, ghosts, invasions |
| Canon | §19, §22 | this chapter owns `pillar.` and `kpi.`; monetization invariants |
| Doc 02 | §5.5, §6.4, §14, §15, §17.3, §17.4, Exports | `PlayerCommand` bits and intents; the 5-target determinism matrix; platforms; `perf.` budgets including `perf.ci.bot_tps`; `flag.telemetry.usage` (default false), `flag.telemetry.crash_mode`, `flag.scribe.enabled` (default false); opt-in aggregate telemetry |
| Doc 03 | §6, §9, §11, §14 (event table: `ev.damage_dealt`, `ev.entity_killed`, `DmgFlags`) | liquids, temperature and sound on the grid; lethal-hit linkage through `damage_seq`; flag bits `dot` 2, `area` 5, `env` 7, `fall` 8 used by `kpi.pillar.systemic_kill_share` |
| Docs 04, 05, 06 | 04 §10, §12; 05 §1.7, §4; 06 §6.2, §16 | Red Ink Marginalia powers, Death's Door; logical inputs and moveset; cast cost falling with skill; IVAN-table supplication and petitions |
| Docs 08, 09, 10 | 08 §12.1, §12.4; 09 §5, §6, §9; 10 §5, §11, §12 | `alch.potion.liquid_flame` and identification by throwing; detection meter, awareness state machine, sneak attacks; identification methods, Sealed contents, Index |
| Doc 11 | §4, §5, §9.7, §10, §11, §14.4, §15, §17 | genre-skin records; manufacturers; treasure bags; in-run Reliquaries and mimics; Fortune Engines and engine pity; ground-item visibility; Rewrite; co-op loot |
| Doc 12 | §2.4, §3.3–§3.5, §4, §5, §14, §17.4 | placed light as defense; GPV layout, physics and verb weights; World Laws; Genre Lattice and exit preview; bleed and seam zones; secret exits |
| Doc 13 | §9.2, §10.5, §10.7 | the five Editors `boss.editor_v1` … `boss.ur_editor`; Erasure hunters; the invader template |
| Doc 14 | §1.1, §3, §4.2, §5, §6, §7, §8, §9, §11 | `Q_REF`, `Q_WIN`, `TURN_HOLD_TICKS`; Folio flow and Erasure timeline; score and Example B; Margin facilities and Bindery totals; Echoes; Grades; modes; Marginalia; Prologue |
| Doc 15 | §1, §2.7–§2.8, §3.3–§3.6, §4.3, §4.5, §6.7, §8.6, §9, §11, §12 | business model and the separation wall; income and hours to earn; hazards, featured split, spark; verifier; offline operation; Codex completion; ratings; anti-predatory rules; end of service; economy KPIs |
| Doc 16 | §3.6, §5.7, §9.1, §11.7, §13 | crossplay default; intents `ChooseExit`, `TradePropose`, `DropItemFor`, `MarkTarget`; drop-in at Folio boundaries; telemetry endpoint; offline mode |
| Doc 17 | §12.1–§12.9, §13.1 | the `input.` action list, `input.preset.folio_kbm` and `input.preset.folio_pad` bindings; Scribe's Mercy |
| Research | R1, R2, R3 (Part A, §0, D5, D6), R5, R6 (§1, §2), R7 (§9) as cited | facts, peer bar, law, precedents, syntactic genre channels |

---

## 1. Pitch and premise

### 1.1 One-sentence pitch

> PALIMPSEST is a first-person co-op roguelike for 1 to 4 players in which characters who fell out of their own stories descend through a bottomless manuscript of overwritten genres, carrying revolvers into fairy tales and spellbooks into cyberpunk, while the Editors try to erase them.

### 1.2 One-paragraph pitch

You are an Erratum: a character your story dropped. With up to three friends you descend 25 Folios, each written in one of a dozen fictional genres, from Gothic Horror to Wuxia to Space Opera. Every genre is a reality with its own physics, laws and four axioms, so the gun you found in a cyberpunk arcology jams one shot in ten in the fairy-tale forest, yet the wolf there has never seen a gun and staggers at the first shot. The world is a simulated block manuscript: oil burns, water conducts, light keeps the dark things away and severed arms can be sewn back on. Loot is part-built and rarity-tiered, unknown potions hide behind per-seed colors, skills grow by use, and death ends the run. The Editors, the Palimpsest's living red pens, hunt you if you linger. Reach the Ur-Text at Folio 25 and close the book, or keep writing into the endless Marginalia. Every gacha pull, loot box and battle-pass reward is earned by playing; money buys only things you can see before you pay.

### 1.3 Fiction premise

| Term | Fiction | Mechanic (owner) |
|---|---|---|
| The Palimpsest | a bottomless manuscript, scraped and overwritten forever; every story ever told lies in it, one over another | the world; a run descends it (Canon §17) |
| Folio | one page of it, written in one genre | one generated floor, 256 × 64 × 256 blocks (Canon §1.8; doc 12) |
| Volume | five Folios bound together; the fifth holds an Editor | Canon §17.1 |
| Genre | the rules a page was written under | axiom vector, World Laws, skins (Canon §5) |
| Errata | characters who fell out of their stories when the page was scraped; they belong to no story, so they carry anything anywhere | player characters; Origin tolerance (Canon §5.4) |
| Editors | the manuscript's correctors; they treat Errata as mistakes and erase them | `fac.editors`, Erasure, Volume bosses `boss.editor_v1` … `boss.editor_v4` and `boss.ur_editor` (Canon §11.6; doc 13 §9.2) |
| Red Ink | the Editors' correcting fluid and their servants | `liq.red_ink`, `afflict.red_ink`, Erasure hunters (`rank.hunter`) (Canon §11.6, §12.1, §16.4) |
| redactors | junior Editors on loan from the Redactor's office, sent into a page another reader is reading | opt-in PvP invaders: a player as `fac.editors`, `rank.hunter`, replay slot 4 (Canon §17.9) |
| Colophon | the sign-off at the end of every page, where the scribe rests | sanctuary room before the exits (Canon §17.1) |
| The Margin | the blank edge of the page where Errata gather between descents | meta hub (doc 14 §5) |
| The Ur-Text | the first text, under every other one; all axioms 5, nothing contradicts | Folio 25, `genre.ur_text`, C = 0 and E = 0 (Canon §5.5) |
| Marginalia | notes written past the end of the book | endless Folios 26 onward (doc 14 §9) |

**Premise.** Stories do not end in the Palimpsest; they are scraped thin and written over. When a page is scraped, a few characters slip through the knife and fall out of their plot. They wake in the Margin remembering one genre (their Origin), able to wield any other. The Editors see them as typographical sin. Five Editors guard the Volumes: the Proofreader (`boss.editor_v1`, Folio 5), the Redactor (`boss.editor_v2`, Folio 10), the Continuity Editor (`boss.editor_v3`, Folio 15), the Abridger (`boss.editor_v4`, Folio 20) and the Last Word (`boss.ur_editor`, Folio 25) (doc 13 §9.2). The Redactor keeps an office of junior redactors; when a party opens its page to invasion (Canon §17.9), another reader can take one of their desks. Under every page lies the Ur-Text. An Erratum who reaches it can read the first sentence and choose: **Close the Book** and be written back into a story, or step through a Margins door and keep writing where no Editor has authority (doc 14 §9.1).

**Fiction rules.**

1. Every mechanic has a diegetic reading. A jam is the page refusing a word it cannot spell; a Disconnected item is a torn page; Anachronism Edge is the native's astonishment.
2. Genres never mock each other. The wry voice belongs to the manuscript and its narrator, never to the genres' sincerity. A Noir Folio is played straight; the narrator's marginal note is where the wink lives.
3. Errata are never chosen ones. They are mistakes that refuse to be corrected.
4. Editors are neither demons nor tyrants; they are diligent. Their lines read like style-guide rulings.

---

## 2. Design pillars

Six pillars, in priority order. Pillar 6 is a hard constraint and wins every conflict. Among Pillars 1 to 5, the lower number wins. Each pillar has three pass/fail tests; Early Access ships only when all 18 pass (§2.7).

### 2.1 `pillar.one_ruleset` — One Ruleset, Many Skins

**Statement.** Mechanics live in genre-neutral archetypes; genres are data that change names, looks, sounds, dependencies, physics columns and World Laws, never the underlying formulas.
**Why it matters.** Sixteen genres ship only if one ruleset serves them all, and a genre feels real only if it changes how you play (R6 §1).

| Test | Pass condition | Method |
|---|---|---|
| `pillar.one_ruleset.t1` | 0 genre-skin records (doc 11 §4) carry a numeric mechanic field other than `dep`, `law_bound` and perk-pool entries | content validator, every build |
| `pillar.one_ruleset.t2` | for all 66 pairs of core genres, `syntax_delta(g, h) >= 3` (below) and the axiom L1 distance is ≥ 6 (Canon §5.2) | content lint over doc 12 GPVs and doc 13 rosters, every build |
| `pillar.one_ruleset.t3` | blind recognition: ≥ 20 testers who have not seen the game each enter all 12 core genres once in a per-tester order drawn by the test harness; with the Folio title card, every genre name, Lucre and Aether skin label and the exit preview hidden by a test flag, each names the genre from the 12 display names within 600 ticks (10 s) of `ev.folio_entered`; for every genre, `floor_div(correct × 10000, testers) >= 8000` | playtest protocol, before EA and on every added or reskinned genre |

`syntax_delta(g, h)` counts the syntactic channels of R6 §1 item 12 and §7 in which two genres differ, each computed from content:

| Channel | Differs when | Source |
|---|---|---|
| Structure | `layout` differs | doc 12 §3.3 |
| Laws | the sets of Law names (the last ID segment of each `law.*`) differ | doc 12 §4 |
| Physics | the tuple (`gravity_bp`, `air_accel_bp`, `fall_dmg_bp`, `ground_friction_bp`, VRV) differs | Canon §5.2, §5.9 |
| Verbs | `Σ |verbs_g[i] - verbs_h[i]| >= 10000` over the 10 verb weights | doc 12 §3.5 |
| Enemy behavior | the sets of doc 13 behavior-set IDs used by the two rosters differ | doc 13 §4.7, §8 |

With the doc 12 data on disk the first four channels alone give every pair at least 3: Structure and Laws differ for all 66 pairs; Physics is equal only for Mythic Fantasy and Steampunk (both `(10000, 10000, 10000, 10000, 6)`), whose Verbs distance is 48,000; the smallest Verbs distance is 13,000 (Post-Apocalypse and Steampunk). The lint therefore passes today with margin from the fifth channel.

**Anti-patterns.** Per-genre damage formulas; a genre that is a palette swap (Ultrakill Encore pattern outside bleed and Marginalia); genre-specific currencies with their own rules (Lucre is one value with 16 skins, Canon §14).

### 2.2 `pillar.out_of_story` — Out of Your Story

**Statement.** Carrying the wrong genre's tool is always possible and always priced: Contradiction costs potency and reliability, Anachronism Edge pays against natives, and the player sees both before committing.
**Why it matters.** This is the one decision no other roguelike offers, and it turns every exit choice, shop and Rewrite into a loadout question (Canon §5.5, §5.6).

| Test | Pass condition | Method |
|---|---|---|
| `pillar.out_of_story.t1` | in 1,000 solo Standard bot runs, ≥ 9000 Bp contain at least one exit offer whose doors give the bot different forecast `C` (doc 14 §3.6) | headless bot sweep |
| `pillar.out_of_story.t2` | for all 12 core genres × 13 Callings, the Folio-1 shop plus guaranteed loot holds ≥ 1 weapon with empty `dep` usable by a Calling major combat skill | generator sweep, 100 seeds per pair |
| `pillar.out_of_story.t3` | 10000 Bp of item cards and HUD weapon widgets show the component state icon (Coherent, Strained, Contradicted, Disconnected) matching the simulation state, on the first rendered frame whose interpolated simulation tick is ≥ the tick of `ev.contradiction_changed` | UI automation over 1,000 scripted genre changes |

**Anti-patterns.** Binary bans of foreign gear; hidden Contradiction math; Edge above +24% (Canon §5.6); a genre in which no Calling can fight.

### 2.3 `pillar.living_page` — The Page Is Alive

**Statement.** Liquids flow, gas spreads, fire catches, light and sound propagate, bodies come apart, and all of it is deterministic, so the cleverest answer to a room is often the room itself.
**Why it matters.** Caves of Qud, IVAN and BG3 earn their stories from systems colliding; determinism lets the same collisions replay, verify and network (Canon §2, §3).

| Test | Pass condition | Method |
|---|---|---|
| `pillar.living_page.t1` | `systemic_share >= 800` Bp of NPC kills (definition in §10.3, `kpi.pillar.systemic_kill_share`); before EA measured over 1,000 solo and 1,000 four-player Standard bot runs, after EA over opt-in live runs | event tally joining `ev.entity_killed.damage_seq` to the lethal `ev.damage_dealt` |
| `pillar.living_page.t2` | the doc 02 §6.4 determinism set (50 golden replays of 3,600 ticks per pull request, 500 of 18,000 ticks nightly) writes byte-identical `hashes.json` on all 5 targets | determinism CI |
| `pillar.living_page.t3` | each Rubric type (`run.rubric.keeper`, `run.rubric.catchwords`, `run.rubric.seal`) in each core genre is completed by the combat-led bot policy in ≥ 6000 Bp of 200 seeds, and by the stealth-led and the environment-led policies in ≥ 3000 Bp each (108 cells) | headless bot sweep (doc 02 §16 bots) |

**Anti-patterns.** Scripted set-piece hazards that ignore the grid; floats or `HashMap` in simulation state; effects that only the renderer knows (Canon §22).

### 2.4 `pillar.party_of_errata` — Lost Together

**Statement.** One to four players, every size tuned, each needing the others: revives, ghosts, carried Remains, shared knowledge, ally-targeted verbs and no competition for loot.
**Why it matters.** Barony proved the format; the 2026 bar is interdependence at every party size (R3 A2).

| Test | Pass condition | Method |
|---|---|---|
| `pillar.party_of_errata.t1` | the Volume I clear rate (share of runs in which `boss.editor_v1` dies) per party size 1..=4 is within ±1000 Bp of the four-size mean (`kpi.coop.party_parity`) | before EA: 1,000 Standard Grade-0 bot runs per party size; after EA: the live KPI |
| `pillar.party_of_errata.t2` | 13 of 13 Calling kits contain ≥ 1 verb that targets or benefits an ally (heal, ward, buff, revive speed, taunt, mark, thrown potion with area effect) | content audit against doc 04 kits and doc 14 §2.4 kit records |
| `pillar.party_of_errata.t3` | in default co-op (instanced loot), across 10,000 generated item drops in 4-player runs, 0 items in ground state `Instanced(s)` are visible to or pickable by any slot other than `s`; only an owner's `DropItem` makes an item `Public` (doc 11 §14.4, §17) | sim test |

**Anti-patterns.** Solo-tuned encounters with HP multiplied; shared loot by default; a dead player watching a spectator camera for 20 minutes (ghosts act, Canon §17.6).

### 2.5 `pillar.every_find_a_question` — Every Find Is a Question

**Statement.** Items are built from parts, rolled per player, partly unknown and worth investigating; skills grow by what you do; meta progression widens options and never adds raw power.
**Why it matters.** Knowledge is the deepest loot; SPD, NetHack, Morrowind and Borderlands players return for it.

| Test | Pass condition | Method |
|---|---|---|
| `pillar.every_find_a_question.t1` | every type-hidden item type is identifiable by ≥ 3 distinct methods among use or zap or throw, equip, witness, a doc 10 text, appraisal, an Analysis check (Examine), price observation, a Scriptorium or altar service, a divination glyph (doc 10 §5.1–§5.10) | content audit |
| `pillar.every_find_a_question.t2` | for each weapon archetype, 100,000 generated drops at Folio 10 give ≥ 1,000 distinct (manufacturer, sorted affix-ID set, perk-option set) tuples | generator sweep |
| `pillar.every_find_a_question.t3` | (a) on one Daily seed (`unlock.daily_baseline`), a fresh account and a fully unlocked account produce byte-identical item records at every Folio; (b) for every loot table, the rarity weight vector at every depth 1..=40 is identical for the fresh and the fully unlocked content set (doc 14 §5.2 band renormalization) | sim test and content lint |

**Anti-patterns.** Raw stat unlocks in the Bindery; identification busywork for base types (Barony 5.0 rule, Canon §13.6); flat per-hit combat XP (Canon §7.1).

### 2.6 `pillar.fair_page` — The Fair Page

**Statement.** Earned Randomness, Paid Certainty: every random container is funded by play, every price buys a known thing, every roll is disclosed and verifiable, and the game survives its own servers.
**Why it matters.** Every 2026 loot-box regime keys on payment (R3 §0); Destiny 2's final update on 2026-06-09 shows live services end (R2 §6.1). Trust is a feature.

| Test | Pass condition | Method |
|---|---|---|
| `pillar.fair_page.t1` | SEP-1 to SEP-7 (doc 15 §1.2) pass; `juris.paid_random_items_enabled` is the compile-time constant `false` | content validator, ledger tests, build assert |
| `pillar.fair_page.t2` | the Inkwell, account Reliquaries, in-run Reliquaries and Fortune Engines each show every outcome's probability through `pct4` (doc 15 §3.11) before the commit input, plus every live pity counter the system has (Inkwell `top_since`, `mid_since` and spark; Reliquary counters; Engine `need − pity`); `palimpsest-verify` (doc 15 §4.3) returns `PASS` for 10,000 logged pulls and 10,000 logged Reliquary opens | UI automation, verifier run |
| `pillar.fair_page.t3` | with the network disabled: a bot completes the Prologue, Standard Folios 1–10 and a LAN co-op Folio; an online account queues an Inkwell pull as a Sealed Draft; an offline-only profile pulls the Retrospective banner and verifies it locally; Codex XP accrues locally; only leaderboards, invasions, cloud saves and telemetry are unavailable (doc 15 §4.5, §11.3; doc 16 §13) | offline integration test |

**Anti-patterns.** Premium currency; timers on paid offers; streak rewards; odds hidden behind a link; content that needs a server to exist.

### 2.7 Pillar records and the release gate

The registry is `content/01/pillar/pillars.json`, one record per pillar. Fields: `id` (content ID), `order` (`1..=6`; the Statement order of §2.1 to §2.6), `hard_constraint` (`true` only for order 6), `tests` (exactly 3 test IDs `<id>.t1..t3`), `owner_docs` (doc numbers that implement the pillar's tests).

```json
[
  { "id": "pillar.one_ruleset", "order": 1, "hard_constraint": false,
    "tests": ["pillar.one_ruleset.t1", "pillar.one_ruleset.t2", "pillar.one_ruleset.t3"], "owner_docs": [11, 12, 13, 18] },
  { "id": "pillar.out_of_story", "order": 2, "hard_constraint": false,
    "tests": ["pillar.out_of_story.t1", "pillar.out_of_story.t2", "pillar.out_of_story.t3"], "owner_docs": [5, 6, 11, 12, 14, 17] },
  { "id": "pillar.living_page", "order": 3, "hard_constraint": false,
    "tests": ["pillar.living_page.t1", "pillar.living_page.t2", "pillar.living_page.t3"], "owner_docs": [2, 3, 8, 9, 12] },
  { "id": "pillar.party_of_errata", "order": 4, "hard_constraint": false,
    "tests": ["pillar.party_of_errata.t1", "pillar.party_of_errata.t2", "pillar.party_of_errata.t3"], "owner_docs": [4, 11, 13, 14, 16] },
  { "id": "pillar.every_find_a_question", "order": 5, "hard_constraint": false,
    "tests": ["pillar.every_find_a_question.t1", "pillar.every_find_a_question.t2", "pillar.every_find_a_question.t3"], "owner_docs": [4, 10, 11, 14] },
  { "id": "pillar.fair_page", "order": 6, "hard_constraint": true,
    "tests": ["pillar.fair_page.t1", "pillar.fair_page.t2", "pillar.fair_page.t3"], "owner_docs": [2, 14, 15, 16] }
]
```

Every test result is stored with the build number that produced it. The **EA gate** needs all 18 tests `Pass` on the EA candidate build. Every **later release gate** needs all 18 `Pass` on that build, except `pillar.one_ruleset.t3` (a human playtest), whose most recent `Pass` carries forward while the set of shipped genres and genre skins is unchanged since that playtest.

```rust
pub type Id32 = u32;                                         // fnv1a32 of the content ID (Canon §1.1)
#[derive(Copy, Clone, PartialEq, Eq, Debug)] pub enum Verdict { Pass, Fail, NotRun }
pub struct Pillar { pub id: Id32, pub order: u8, pub hard_constraint: bool, pub tests: [Id32; 3], pub owner_docs: Vec<u8> }
pub struct TestResult { pub verdict: Verdict, pub build: u32 }
#[derive(Copy, Clone)] pub enum Gate { EarlyAccess, Later { genres_changed_since_t3: bool } }

pub fn release_gate(pillars: &[Pillar], results: &BTreeMap<Id32, TestResult>, build: u32, gate: Gate) -> bool {
    let t3 = fnv1a32("pillar.one_ruleset.t3");
    pillars.iter().all(|p| p.tests.iter().all(|&t| match results.get(&t) {
        Some(r) if r.verdict == Verdict::Pass =>
            r.build == build || (t == t3 && matches!(gate, Gate::Later { genres_changed_since_t3: false })),
        _ => false,                                         // Fail, NotRun or missing
    }))
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)] pub enum Choice { A, B, Escalate, Neither }
/// Conflict rule between design options a and b. Each slice lists the `order` of every pillar
/// with at least one failed test under that option, in any order, duplicates allowed.
pub fn prefer(a_fails: &[u8], b_fails: &[u8]) -> Choice {
    let norm = |f: &[u8]| { let mut v = f.to_vec(); v.sort_unstable(); v.dedup(); v };
    let (a, b) = (norm(a_fails), norm(b_fails));
    match (a.contains(&6), b.contains(&6)) {
        (true, true) => return Choice::Neither,             // pillar 6 is a hard constraint: both options are rejected
        (true, false) => return Choice::B,
        (false, true) => return Choice::A,
        (false, false) => {}
    }
    // Walk both ascending lists together. At the first difference, the option whose failed pillar
    // has the higher order number (less important) wins; an option whose list ends first wins.
    for i in 0..=a.len().max(b.len()) {
        match (a.get(i), b.get(i)) {
            (None, None) => break,
            (None, Some(_)) => return Choice::A,
            (Some(_), None) => return Choice::B,
            (Some(x), Some(y)) if x > y => return Choice::A,
            (Some(x), Some(y)) if x < y => return Choice::B,
            _ => {}
        }
    }
    Choice::Escalate                                        // identical failure sets: the product owner decides
}
```

Examples: option A fails `pillar.living_page.t1` (order 3) and option B fails `pillar.party_of_errata.t1` (order 4). The lists are `[3]` and `[4]`; at index 0, `3 < 4`, so B wins, because breaking Pillar 4 costs less than breaking Pillar 3. Option A failing orders `[2, 3]` against B failing `[2]`: index 0 ties, B's list ends first, so B wins. A failing `[6]` against B failing `[6, 1]` gives `Neither`: no option may break the Fair Page.

---

## 3. Inspiration ledger

Each entry lists what we take exactly (with the owning doc), what we improve, what we reject, and the signature moment the game must reproduce. Every signature moment is also an automated scenario test, `SM-01` to `SM-11` in §3.13. Facts: R1, R2, R7.

### 3.1 Barony (Turning Wheel; 5.0.2 on 2026-02-18)

- **Take exactly:** first-person real-time 1–4 co-op roguelike frame (brief §1; docs 05, 16); dead players become ghosts who open doors and mark targets (Canon §17.6; doc 16); only an item's magic is hidden, never its base type (Barony 5.0 Lore; Canon §13.6; doc 10); blessed/uncursed/cursed states (doc 10); real-time hunger (`res.satiation`; doc 04); shared-loot co-op as the opt-in `mode.loot_shared` (Canon §17.5; doc 14).
- **Improve:** per-party-size tables for HP, count and damage (Canon §17.2); instanced loot by default; host-authoritative replays; 25 genre-distinct Folios instead of biome swaps.
- **Reject:** full-strength friendly fire on direct hits (default 5000 Bp, Canon §17.5); appraising mundane gear.
- **Signature moment — "The Ghost Opens the Door" (SM-01):** a party member dies in a trap room, floats ahead as a ghost, opens the far door and applies `st.marked` (480 ticks) to the elite; the survivors carry the 75,000 g Remains through the exit and the ghost returns at the next `ev.folio_entered` with 5000 Bp of Max HP (Canon §17.6; docs 14 §3.10, 16 §9.5).

### 3.2 Shattered Pixel Dungeon (00-Evan; v4.0.2 on 2026-10-07)

- **Take exactly:** per-seed unknown appearances shuffled at run start (Canon §13.6; doc 10); the Scroll of Upgrade as the Emendation, 3 per Volume with the re-level rule (Canon §13.7; doc 11); strength requirement as attribute shortfall (Canon §6.3; doc 04); seeds and a daily seed (Canon §3.6; doc 14); deck-based category pacing (doc 11); same-ingredient brews that auto-identify (doc 08).
- **Improve:** real time and co-op; type knowledge shared across the party and across all 16 genre skins.
- **Reject:** turn-based clock; a run that ends at a fixed depth with no endless continuation.
- **Signature moment — "The Gamble Read" (SM-02):** cornered at low HP, the player throws an unknown crimson flask at the pack; it is `alch.potion.liquid_flame`, the oil splash ignites, the pack burns, and because the splash applied an effect the type is classified for the whole party in every genre's skin (docs 08 §12.4, 10 §2.2).

### 3.3 Experienced Pixel Dungeon (TrashboxBobylev; 2.19 final, archived)

- **Take exactly:** no level cap (Canon §8); endless depth with numbers that keep climbing, one Emendation per Marginalia exit and a rising enhancement cap (Canon §13.7; doc 14 §9); 64-bit resources from day one (Canon §1.2); treasure bags `item.bag.{boss, editor, lucky, elite, refined, burnt}`, including a lucky bag that burns into a refined bag at 1000 Bp or a burnt one (doc 11 §9.7).
- **Improve:** bounded inflation through `d_pow ≤ 300`, `DIFF_CAP_BP` and a proven overflow bound (doc 14 §9.2); Marginal mutations for variety (doc 14 §9.3).
- **Reject:** XP-to-upgrade grind in the main run; NG+ cycles with ×40,000 virtual depth.
- **Signature moment — "Numbers That Never Stop" (SM-03):** at Folio 50 the favored weapon reaches +20, the cap `15 + floor_div(50 - 25, 5) = 20`, using 20 of the at least 39 Emendations a run has received by then (15 from Volumes I to V plus one per Marginalia exit from Folio 26 to Folio 49), and `boss.ur_editor`, the Folio-50 Marginalia Editor (doc 14 §9.1 rotation), falls to it (Canon §13.7; docs 11, 14).

### 3.4 Oblivion (2006; Remastered 2025)

- **Take exactly:** mastery tiers at 25/50/75/100 (Canon §7; doc 04); character XP from every skill and from non-combat play, as in the Remaster (Canon §8; doc 04 §6); the Remaster's anti-grind rule that skill XP scales with magnitude and zero-cost actions grant 0 (Canon §7.1; doc 06 §6.1, §7.4); detection as a meter with hysteresis thresholds (doc 09 §5, §6); the armor reduction cap of 85% (`DR_bp ≤ 8500`, Canon §10.5 step 5); sneak attacks that replace the crit roll instead of multiplying with it, as Oblivion's `max(SneakMult, PowerAttackMult)` (Canon §10.5 steps 2.3–2.4); spell cost falling with school skill from × 12500 at skill 0 to × 7500 at skill 100 (doc 06 §6.2).
- **Improve:** enemies scale with depth through `H(d)` and `T(d)`, never with player level (Canon §13.8); sound travels through the sound grid, not through walls (doc 03 §11).
- **Reject:** player-level enemy scaling and leveled lists; the level-up multiplier bookkeeping of the original; a non-combat build that levels into harder enemies without getting stronger (depth, not level, sets threat).
- **Signature moment — "The Sleeping Guard" (SM-04):** crouched at effective light 2, the Shade's dagger sneak attack on a `st.sleeping` guard draws no crit roll, kills it, and its `ev.noise` reaches the guard in the next room only through the open doorway (Canon §10.5 step 2.4; docs 05, 09 §7, §9).

### 3.5 Morrowind (2002)

- **Take exactly:** use-based skills that train their governing sub-attribute through marks (Canon §7.1; doc 04); spellmaking from effect parameters and enchanting (doc 06 §4, §9); levitation as a route-breaking verb (`st.levitating`; doc 06); FAT and SER govern no skill, like Luck (Canon §7).
- **Improve:** automatic training marks instead of rest-time multipliers; checked integer arithmetic instead of 16-bit wraparound (Canon §1.3).
- **Reject:** dice-rolled misses on a visible contact; unbounded spell magnitude.
- **Signature moment — "The Spell You Wrote" (SM-05):** the Spellwright composes a levitation glyph sentence within the complexity budget `4 + floor_div(LOG, 10)` (Canon §6.4 row 26) and floats the party over a chasm the layout expected them to walk around (doc 06).

### 3.6 Baldur's Gate 1–3 (1998, 2000, 2023; BG3 Patch 8 final)

- **Take exactly:** surfaces and elemental reactions (Canon §12.1; docs 03, 08); high ground ×11000 at `dy ≥ 2500 mm` (Canon §10.5 step 2.4a; doc 05); visible sight cones and last-known position (docs 09, 17); rest between fights as the Interleaf (doc 14 §3.9); a forbidden power path as Red Ink Marginalia powers (Canon §16.4; doc 04 §10); faction reputation (Canon §15; doc 13).
- **Improve:** real time with tactical depth carried by surfaces, verticality and party roles; a reaction graph small enough to memorize (11 surfaces, 8 triggers).
- **Reject:** turn-based combat; parties of 6; dialogue trees as the primary verb.
- **Signature moment — "Grease and Fire from the Balcony" (SM-06):** the Alchemist oils a stair (`liq.oil`, `st.oiled`, fire buildup ×20000), the Gunslinger fires from a balcony 3 m above (`dy ≥ 2500 mm`, ×11000), and the Spellwright's fire glyph ignites the oil in a 2 m environmental `dmg.fire` blast of base 10 HP per 1000 ml, capped at 5000 ml (Canon §10.5 step 2.4a, §12.1; docs 05, 06, 08).

### 3.7 Caves of Qud (Freehold; 1.0 on 2024-12-05)

- **Take exactly:** liquids as data (doc 08); per-part bodies, severing and reattachment (Canon §9.1; doc 04); mutations and implants (`kin.mutant`, `aug.`; doc 04); reputation bands (Canon §15); temperature (doc 03 §9); artifact examination as Examine (doc 10 §5); tinkering mods (doc 07).
- **Improve:** closed-form regeneration and every formula published (Canon §6.4); real-time 3D.
- **Reject:** penetration dice (R7 C1, Canon §10.5 wins); world-map travel.
- **Signature moment — "Losing an Arm and Keeping the Run" (SM-07):** a champion's cleaver hit, largest share `dmg.slash`, empties the Gunslinger's `bp.arm_l` condition pool and severs it; the `slot.offhand` pistol drops, and the party's Alchemist (Medicine is an Alchemist major skill, here raised to 25) sews the arm back on with a 600-tick channel before it rots at 7200 ticks (Canon §9.1; docs 04 §8, 05).

### 3.8 IVAN (Iter Vehemens ad Necem)

- **Take exactly:** one material table for blocks and items (Canon §13.8a; doc 07); mass from material × volume; IVAN's prayer table with favor and timers (Canon §16.5; doc 06 §16); sacrifice value `isqrt(value)`, doubled for the matching patron (Canon §16.5; doc 06 §16.5).
- **Improve:** integer timers in ticks; one patron Author per genre; local-patron prayers × 15000.
- **Reject:** floating-point asymptotic attribute experience; misery without legibility.
- **Signature moment — "The Prayer at One HP" (SM-08):** with Death's Door spent and HP below 3333 Bp of Max HP, the Oracle, `prayer_timer` 0 and favor 600, supplicates the local patron: PLEASED, HP to max (need 1), timer +18,000 ticks. Praying again 60 s later is IVAN's gamble: HELPED (timer +90,000) when `favor > r0` and `timer < r1`, otherwise ANGRY (timer +180,000, wrath with doubled durations) (doc 06 §16.3).

### 3.9 Borderlands 1–4 (BL4 on 2025-09-12; level cap 70 on 2026-09-10)

- **Take exactly:** part-built weapons with manufacturer traits, two per core genre plus the Errata maker (Canon §5.9; doc 11 §5); a seven-step rarity ladder with colors (Canon §13.3); licensed cross-manufacturer parts (`cat.part`; docs 07, 11); cosmetic jackpots as Illuminated drops at 5 Bp (Canon §13.9); instanced co-op loot (Canon §17.1); a non-expiring pass in the Vault Card mould (doc 15 §6).
- **Improve:** published 4-decimal odds on every container (doc 11 §10); perk columns that are chosen, not rerolled; cross-genre legibility through Contradiction icons.
- **Reject:** slot machines with "nothing" outcomes and casino presentation (R7 C6; brief §8); gear obsolescence (the Emendation re-level answers it); shipping before PC performance is stable (doc 02 §15).
- **Signature moment — "The Orange Beam" (SM-09):** a Legendary drops with its `#FF9A1F` light column, one-sentence manufacturer trait and, at 5 Bp, an Illuminated skin that unlocks account-wide (docs 11, 18).

### 3.10 Minecraft (Java 26.3 "Wilderness Bound", 2026-09-15)

- **Take exactly:** 1 m blocks in 32³ chunks; two-channel integer light 0–15 (Canon §1.2); spawns gated by block light, so placed light is defense (Canon §12.3); liquids on the grid (doc 03 §6); random block ticks (Canon §2.2); commit-only offer streams (Canon §3.5); placeable bridges, ladders, barricades and lights (Canon §4.3; doc 07).
- **Improve:** authored structure stays intact because terrain is fixed except `destructible` blocks (DECISIONS #16); fixed-point determinism.
- **Reject:** terraforming and mining terrain; a goalless sandbox; rerolling offers by reopening a menu.
- **Signature moment — "Torch the Dark" (SM-10):** in a Gothic Horror Folio (`spawn_block_max` 7) the party plants lanterns until `light_block` exceeds 7 along a corridor, and the ambient spawns there stop (docs 03, 07, 12).

### 3.11 Destiny 2 (final content update 2026-06-09)

- **Take exactly:** gear tier orthogonal to rarity, setting perk options per column (Canon §13.9; doc 11); one Exotic per slot group (Canon §13.3); champions with counters shown in the exit preview (Canon §17.3a; docs 05, 12, 13); four ability slots (doc 04); rewards that never expire (doc 15).
- **Improve:** no Power grind; nothing vaulted; offline-complete end of service from day one (brief §8; doc 15 §11).
- **Reject:** premium currency (Silver); seasonal expiry; content that dies with the servers.
- **Signature moment — "Champion Check" (SM-11):** the exit preview shows an Unstoppable champion ahead; the party slots a `counter.staggerer` source, staggers it on cue and wins the room by preparation (docs 05, 12, 13).

### 3.12 Secondary precedents

Torg Eternity (axioms, World Laws), Noita (Holy Mountain, bleed), Hades II (God Mode), Slay the Spire 2 (co-op interdependence), Dungeon Crawler Carl (floor clock), Spider-Verse (foreign render styles); R6 §2–§6, R3 A3.

### 3.13 Signature-moment scenario tests

Each signature moment is a committed headless scenario `tests/moments/sm_NN.json`: a run seed, a mode, a party (Calling, Origin, sub-attributes, skills, equipment), a fixture layout or a seeded Folio, and a scripted input log. The doc 02 §16 harness runs it in `palimpsest_sim` without a renderer, except SM-09's light column, which a UI screenshot test checks. A scenario passes when every listed assertion holds on the event stream. Scenarios run on every pull request; a failing scenario blocks merge in the owning docs.

| ID | Setup | Pass when |
|---|---|---|
| SM-01 | 2 players, Standard, Folio 3; slot 1 dies to a `hazard.*` trap while slot 0 lives | `ev.player_died` and `ev.player_ghosted` for slot 1; the ghost opens a door and applies `st.marked` (480 ticks) to an elite; slot 0 picks up the `cat.corpse` Remains and commits an exit; on the next `ev.folio_entered`, `ev.player_resurrected` for slot 1 with HP `mul_bp(max_hp_mhp, 5000)` |
| SM-02 | solo, an unknown `alch.potion.liquid_flame` in the satchel, 3 hostiles within 3 m of the throw point | the throw emits `ev.item_used`; at least one hostile gains `st.burning`; `ev.item_identified` for the type; the type is at least `know.classified` for every party slot and renders under every genre skin's appearance name |
| SM-03 | Endless fixture at Folio 50 (`boss.ur_editor`), a weapon at enhancement 19 and 1 Emendation | applying the Emendation gives 20; a second Emendation on the same weapon is refused at the cap 20; `ev.entity_killed` of `boss.ur_editor` carries a lethal `ev.damage_dealt` with that weapon's `item_uid` |
| SM-04 | Shade (dagger, Sneak 50) crouched at effective light 2 behind a `st.sleeping` guard; a second guard in the next room; run twice, door open and door shut | the stab's `ev.damage_dealt` has flag `sneak` and not `crit`, and the action draws 0 crit values from `rng.combat`; `ev.entity_killed` for the guard; with the door open the second guard emits `ev.awareness_changed`, with the door shut it does not |
| SM-05 | Spellwright, LOG 40 (budget 8), a 6 m chasm with no walkable bridge | the composed levitation spell compiles at complexity ≤ 8; `ev.spell_cast`; the caster gains `st.levitating`, reaches the far ledge and takes no fall damage |
| SM-06 | Alchemist, Gunslinger and Spellwright; 3 hostiles on a stair; the balcony 3,000 mm above the stair | each hostile on the oil gains `st.oiled`; the Gunslinger's hit applies the ×11000 elevation step; the fire glyph emits `ev.liquid_reaction` (oil ignites); at least one hostile takes an `ev.damage_dealt` with flag `env` from the blast |
| SM-07 | Gunslinger with an off-hand pistol; a champion with a slash cleaver; an Alchemist with Medicine 25 | `ev.part_severed` for `bp.arm_l`; `ev.item_dropped` of the off-hand pistol on the same tick; `ev.part_reattached` after a 600-tick channel, before tick `t_sever + 7200` |
| SM-08 | Oracle (Devotion 20), favor 600, `prayer_timer` 0, HP below 3333 Bp of Max HP, Death's Door used this Folio | first supplication: PLEASED, HP to max, timer 18,000, favor 660 (`50 × 120 / 100 = 60` gained); second supplication 3,600 ticks later (timer 14,400): the outcome equals doc 06 §16.3 evaluated on that action's 4 keyed `rng.magic` draws |
| SM-09 | a forced Legendary drop with a fixture whose `illuminated` roll succeeds | the drop is `rar.legendary`; its light column renders `#FF9A1F`; exactly one keyed `rng.loot` value is drawn for the 5 Bp roll; the account unlock list gains the skin |
| SM-10 | Gothic Horror (`spawn_block_max` 7), a corridor of 24 cells with spawn-eligible floors, run twice for 3,600 ticks | with placed lanterns raising every corridor cell to `light_block ≥ 8`, 0 ambient or wave spawns occur in the corridor; without them, at least 1 does |
| SM-11 | the next Folio holds a champion with `elite.unstoppable` | the exit preview lists the aspect before the vote; the party carries a `counter.staggerer` source; in the next Folio the champion gains `st.staggered` from that source |

---

## 4. The 2026 competitive bar

### 4.1 Peers and the features they set (R3 A1–A4, R7 §9)

Status, player counts and prices are R3 facts as of 2026-10-11. "Our answer" names the PALIMPSEST system that meets or beats the feature.

| Title | 2026 status | Players | Price (USD) | Features that set the bar | Our answer (owner) |
|---|---|---|---|---|---|
| Slay the Spire 2 | EA 2026-03-05; ~177k peak concurrent | 1–4 co-op | 24.99, rising after EA | co-op interdependence: shared route sketching, potions splashed on allies, shared relic choice; Godot rebuild for modding | exit vote with map pings (doc 14 §3.6, doc 17 §8); area heals and thrown potions at full effect on allies (Canon §10.9); data packs and WASM mods (doc 02 §13) |
| Hades II | 1.0 2025-09-25 | 1 | 29.99 | God Mode (+20% resistance, +2% per death, cap 80%); cross-save across PC, Switch and Switch 2 | Scribe's Mercy copies God Mode exactly (doc 17 §13.1); Steam Auto-Cloud (doc 02 §14) |
| Elden Ring Nightreign | 2025-05-30; 5M shipped; 313,593 peak | 1–3 (duo added after launch criticism) | premium | a 3-day clock compresses a 3D run to 40–60 min | the Erasure clock (Canon §11.6); Abridged 60–75 min and Novella 60 min (doc 14 §8); every party size tuned on day one (Canon §17.2) |
| Borderlands 4 | launched 2025-09-12; level cap 70 on 2026-09-10 | 1–4 co-op | premium | Licensed Parts, Enhancements, Firmware; non-expiring Vault Cards; sales "softer than expected", blamed on PC performance | two manufacturers per core genre plus the Errata maker, `cat.part` licensed parts (doc 11 §5); the Codex never expires (doc 15 §6); the PAR-13 performance gate |
| ARC Raiders | launched October 2025; 12M+ sold | 1–3 | 40 plus cosmetics | crossplay on by default with a toggle; monthly updates; a free first progression deck, a paid Reward Pass only from 2026-10-08 | `flag.net.crossplay` default true (doc 16 §3.6); a full free Codex track beside the premium track (doc 15 §6) |
| Deep Rock Galactic: Rogue Core | EA 2026-05-20, about 18 months late | 1–4 co-op | premium | first-person co-op roguelite shooter | the same frame plus genres as realities and a systemic voxel simulation (§4.3) |
| Risk of Rain 2: Alloyed Collective | DLC 2025-11-18 | 1–4 | 15 (DLC) | 3D co-op roguelite with paid content packs | Folio Expansions at 12.99, one genre each (doc 15 §1.1) |
| Enshrouded | 1.0 2026-10-15; 5M+ players in EA | up to 16 | premium | full voxel terraforming, whose cost delayed the Xbox port | limited building on fixed terrain (DECISIONS #16; Canon §4.3) keeps ports affordable |
| Hytale | EA 2026-01-13 | multiplayer | 19.99 | voxel adventure with modding as a launch pillar | data packs first, sandboxed WASM, Workshop (doc 02 §13) |
| Marathon | launched 2026-03-05; ~1.2M sold | 1–3 | 39.99 | criticized for 4 maps a month after launch | 16 genres at EA (PAR-12) |
| PEAK | launched 2025-06-16; 10M+ by mid-August 2025 | 1–4 | about 8 | cheap co-op with voice-driven comedy | voice and text chat with mute, block and report (doc 16 §9.12; doc 15 §8.5) |
| Megabonk | launched 2025-09-18; 1M in 2 weeks | 1 | 5.49 | instant readability and a fast loop | the Prologue (doc 14 §11); icon-plus-color signals (doc 17 P3) |
| Shattered Pixel Dungeon | v4.0.2 on 2026-10-07 | 1 | free with supporter purchases | steady systemic updates from a tiny team | a free systemic update every 6–8 weeks (PAR-11) |
| Caves of Qud | 1.0 2024-12-05 | 1 | premium | deep systemic simulation | Pillar 3 and every formula written in the canon (Canon §6.4, §10.5) |
| Path of Exile 2 | 1.0 and free-to-play on 2026-12-11 | 1–6 | free; cosmetics and stash tabs | ARPG looter depth | part-built loot (doc 11); money buys cosmetics only, never storage or power (doc 15 §1.1) |
| Destiny 2 | final content update 2026-06-09 | 1–6 | expansions, Rewards Pass, Eververse for Silver | a live service whose development ended | no premium currency (Canon §14); offline-complete end of service (doc 15 §11) |

### 4.2 Parity list (must ship)

| ID | Bar (R3 A2, D5) | Best-in-class reference | Our commitment | Owner |
|---|---|---|---|---|
| PAR-01 | 1–4 co-op, every size tuned on day one | Slay the Spire 2; Nightreign's missing duo | Canon §17.2 tables; `pillar.party_of_errata.t1` | 13, 14, 16 |
| PAR-02 | drop-in, drop-out, host migration | R3 D5 item 1 | joins at Folio entry; invaders mid-Folio; host migration from the Folio-entry save | 14, 16 |
| PAR-03 | crossplay on by default with an opt-out | ARC Raiders | `flag.net.crossplay` default true; Steam at EA; relay route with the first non-Steam platform | 16 |
| PAR-04 | cross-save | Hades II | Steam Auto-Cloud for `profile/` and `runs/`; account-level economy sync | 02, 15 |
| PAR-05 | Steam Deck Verified and Steam Machine 10-foot UI | Hades II | 1280 × 800 at 60 fps in `perf_saturation_4p`; every screen gamepad-navigable | 02, 17 |
| PAR-06 | accessibility suite filled into Steam's questionnaire | Hades God Mode | Scribe's Mercy (`access.scribes_mercy`: 20%, +2% per death, cap 80%); solo game speed (`access.game_speed_bp`); FOV and UI-scale ranges, subtitles with speaker names and sound captions, full remapping, hold-or-toggle and color-independent signals per doc 17 §12–§13 | 17 |
| PAR-07 | shareable seeds, a daily, a weekly | Slay the Spire daily climb | Canon §3.6 seeds; `mode.daily` (one ranked attempt per UTC date, unlimited practice), `mode.weekly_gauntlet`, `mode.seeded` share strings | 14 |
| PAR-08 | session-sized runs (R3 D5: 45–75 min) | Nightreign 40–60 min | Daily 25 min, Novella 60 min, Abridged 60–75 min, Standard 100–150 min | 14 |
| PAR-09 | horizontal meta | Hades II | Bindery nodes add options only; no stat node (doc 14 §5.2) | 14 |
| PAR-10 | modding | Hytale, Slay the Spire 2 | data packs, sandboxed WASM, Workshop | 02 |
| PAR-11 | live cadence (R3 A2: free updates every 4–8 weeks) | ARC Raiders monthly; SPD's systemic updates | a free systemic update every 6–8 weeks; a Folio Expansion every 9–12 months; a Codex every 12 weeks | 15, 21 |
| PAR-12 | launch breadth (R3 D6: at least 6 distinct genres at EA) | Marathon's 4-map criticism | all 12 core and 4 secret genres at EA (brief §3). A necessary lower bound is 8 core genres: 24 non-final Folios at most 3 per genre need `ceil_div(24, 3) = 8` | 12 |
| PAR-13 | PC performance | Borderlands 4's lesson | no EA build until every `perf.` budget is green on Deck and on the minimum PC | 02 |
| PAR-14 | end of life | Destiny 2's end of development | offline solo, LAN and direct-IP co-op at 1.0; signed local ownership; offline-complete end of service | 15, 16 |

R3 recommends 45–75 minute runs; the brief fixes the Standard run at 100–150 minutes. Both hold: Abridged and Novella meet R3's bar, Standard is the full book.

### 4.3 Differentiators (nobody else ships these together)

1. Genres as realities with mechanical identity (axioms, World Laws, Contradiction, Anachronism Edge), not tilesets.
2. A first-person four-player co-op roguelike over a deterministic systemic voxel simulation.
3. Server-verified ranked runs by headless replay of the input log (doc 14 §9.4).
4. Full gacha, loot boxes and pass with no paid randomness and commit-reveal fairness: one SKU worldwide (doc 15).
5. Offline-complete end of service from day one.
6. Opt-in invasions in which another player becomes an Editor (Canon §17.9).

---

## 5. Novelty register

Each entry is novel in the genre or bleeding-edge in 2026; owners implement it.

- **NR-01 Graded Contradiction (owner 12, 05, 06).** Torg's any-axis rule made real-time: potency steps 9000 to 5000 Bp, failure 0 to 1500 Bp, inert at gap 6, recomputed on the tick the local genre changes (Canon §5.5).
- **NR-02 Anachronism Edge and Shock of the New (owner 05).** Foreign tech is weaker but surprising: up to +2400 Bp against natives and a guaranteed first-hit stagger at E ≥ 3 (Canon §5.6).
- **NR-03 Genre-native casting (owner 06).** Each genre powers spells through its own axiom: Bandwidth runs on TECHNE in Cyberpunk, Qi on VERVE in Wuxia (Canon §5.2 `aether_axiom`).
- **NR-04 Rewriting (owner 11).** Translate an item into the local genre's equivalent, keeping archetype, rarity, material tier and most affixes (R6 §1).
- **NR-05 Seam zones (owner 12).** At most one stitched room pair per Folio where both genres' axioms take their maximum and both toolsets work at full strength (Canon §5.8).
- **NR-06 World Laws and law-bound verbs (owner 12).** Qinggong or the High Noon duel work only where their Law is active; Laws grant tolerance to licensed classes (Canon §5.10).
- **NR-07 Going Native (owner 14, 04).** Once per run, at a Colophon, an Erratum adopts the current genre as its Origin and takes a native body form (Canon §5.11).
- **NR-08 Unmoored (owner 12).** Three Contradiction failures in 600 ticks switch off luck for 300 ticks: "you are out of your story", with no permanent loss (Canon §5.11).
- **NR-09 The dealt hand of exits (owner 12, 14, 17).** Each exit door shows genre, axioms, Laws, champion aspects and a per-player Contradiction forecast before the vote (doc 12 §5.7; doc 14 §3.6; Canon §17.3a).
- **NR-10 Keyed randomness (owner 00, 16).** Every action-caused roll seeds from (stream, Folio, actor, tick, seq), so prediction, lag compensation and late joiners never shift a crit (Canon §3.3).
- **NR-11 Fixed-point systemic voxel simulation (owner 02, 03).** Liquids, gases, fire, temperature, light and sound in Q32.32 and integers, `no_std`, with bot playtests at ≥ 6,000 ticks per second (`perf.ci.bot_tps`).
- **NR-12 Replay-verified leaderboards (owner 14, 16).** The server re-simulates the whole input log headlessly and compares `state_hash` before a score posts (doc 14 §9.4).
- **NR-13 Earned-only gacha with spark and Archive (owner 15).** The Inkwell has pity, a 75/25 featured split, a 100-pull spark and a Vellum Archive, all funded by play.
- **NR-14 Commit-reveal provable fairness (owner 15).** Every account draw is committed before the pull and verifiable afterwards with a public tool (doc 15 §4).
- **NR-15 Codex with a Quill (owner 15).** A 100-tier pass that never expires, sells no skips, and pays back the next premium track at tier 100.
- **NR-16 Offline-complete end of service (owner 15, 16).** A sunset build keeps every owned Echo, cosmetic and Codex track usable from signed receipts, with a local Retrospective banner (doc 15 §11).
- **NR-17 Sealed Reliquaries (owner 10, 11, 15).** Loot boxes deliver unidentified contents, so opening one starts an identification game instead of ending a dopamine loop.
- **NR-18 Genre-skinned unknown appearances (owner 10).** One per-seed appearance shuffle, rendered as a crimson flask in Fantasy, a crimson injector in Cyberpunk and a crimson pill in Wuxia (Canon §13.6).
- **NR-19 Light as defense in first person (owner 03, 12).** Spawns need `light_block ≤ spawn_block_max`; a lantern is a wall (Canon §12.3).
- **NR-20 Grid-driven stealth (owner 09).** Simulation light and sound grids feed per-pair awareness; walls attenuate noise; sight cones are visible on demand.
- **NR-21 Opt-in invasions as an Editor (owner 13, 14, 16).** Another player joins mid-Folio as a redactor, a junior Editor, in replay slot 4; losing means being Redacted, not ending the run (Canon §17.9).
- **NR-22 Erasure clock and desecrated Colophons (owner 13, 14).** Stacks from 420 s, hunters at 10 stacks, a sanctuary that stops the clock, and a run-long punishment for damaging it (Canon §11.6, §17.1).
- **NR-23 On-device Scribe (owner 18, 02).** An on-device small language model writes item lore, Folio intros and barks through grammar-constrained JSON, cached by seed, never touching simulation state, with a template-grammar fallback (brief §5).
- **NR-24 Foreign things look foreign (owner 18, 02).** Out-of-genre items and Errata keep their home genre's shader, outline and animation step rate (R6 §6, Spider-Verse).
- **NR-25 Damage-normalized skill XP (owner 04).** Draining one standard enemy gives 1.5 skill XP whatever the fire rate, so no weapon farms skills (Canon §7.1).
- **NR-26 Echoes with Recollection trials (owner 14, 15).** Every gacha character has a deterministic earn path whose expected hours stay below the Inkwell's 19.58 (doc 15 §2.8).

---

## 6. Core loops at five time scales

Each loop feeds the next: moments win encounters, encounters fund the Folio, Folios build the run, runs pay the meta, and the meta widens the next run's options without adding power.

### 6.1 The 10-second moment

```
  PERCEIVE ──────► DECIDE ──────────► ACT ─────────────► RESOLVE ───────────► READ
  sight 20 Hz      Contradiction       swing W/A/R,       damage pipeline       per-type numbers,
  sound grid       state, stamina,     shot, cast, throw, (Canon §10.5),        state icon, noise
  light_at(e)      aether, cover       dodge 12 ticks     buildup, surfaces     ring, wound decal
     ▲─────────────────────────────────────────────────────────────────────────────┘
```

W/A/R is a melee swing's `Windup`, `Active` and `Recovery` states (doc 05 §4.8). Reference numbers: a Folio-1 standard enemy has 60 HP and the reference player deals 30 HP per second, so time-to-kill is 2.0 s (120 ticks, Canon §17.4). At Folio 10, the same Common Normal +0 weapon at ilvl 10 deals `mul_bp(30000, 21718) = 65,154` mHP/s against `mul_bp(60000, 34423) = 206,538` mHP, so TTK is `ceil_div(206538 × 60, 65154) = 191` ticks (190 ticks deal only 206,321 mHP); rarity, quality, material, Emendations (together at most `ITEM_MULT_MAX = 51870`) and skill growth are how the player wins those 71 ticks back (§6.6). A dodge costs 20 stamina and grants `min(12, 8 + floor_div(max(0, AGI - 40), 15))` invulnerable ticks: 8 at AGI 40, 10 at AGI 70 (Canon §4.2).

### 6.2 The 60-second encounter

```
  APPROACH ───► OPENER ───────────► FIGHT ─────────────► LOOT ────────► RECOVER
  unaware foes  sneak attack, Shock  party-scaled count,  instanced,     stamina after 1 s,
  light, noise  of the New, surface  ranks, counters,     sealed or      HP after 5 s at
                prep, placed trap    revives              known          0.5% Max HP/s
```

Worked example: a spawner with `n_base = 5` in a 3-player party spawns `floor_div(5 × 15000 + 0, 10000) = 7` enemies and keeps `carry = 75000 - 70000 = 5000` for its next wave, which spawns `floor_div(5 × 15000 + 5000, 10000) = 8` and leaves `carry = 0` (Canon §17.2). Two waves give 15 enemies, exactly `mul_bp(10, 15000)`, so no party size drifts.

### 6.3 The 5-minute Folio

```
  INCIPIT ──► EXPLORE ─────────► RUBRIC ───────► COLOPHON ─────────────► EXIT VOTE
  sanctuary,  rooms, notes,      keeper 5000 /   heal, shop, 3 boons,    2 or 3 doors,
  title card  bleed zones,       catchwords 3000 Scriptorium, Going      forecasts,
  180 ticks   secrets            / seal 2000 Bp  Native; clock stops     120-tick hold
```

| Tick from entry | Event (doc 14 §3.8, Canon §11.6) |
|---|---|
| 0 | `ev.folio_entered`; Erasure removed; Death's Door reset |
| 14,400 (240 s) | last tick that completes `run.note.swift` |
| 18,000 (300 s) | first Erasure warning |
| 23,400 (390 s) | second warning |
| 25,200 (420 s) | stack 1; then one stack every 1,800 ticks |
| 41,400 (690 s) | stack 10: first Erasure hunter at the default threshold 10 |
| 46,800 (780 s) | second hunter; then one every 5,400 ticks while any Erratum holds its threshold |

Entering the Colophon stops stack accrual for the rest of the Folio, not hunter spawns (doc 14 §3.8). A party that reaches the Colophon by tick 25,199 never gains a stack.

### 6.4 The 2-hour run

```
  MARGIN ─► V I ─► V II ─► V III ─► V IV ─► V V ─► UR-TEXT ─► CLOSE THE BOOK
  Lectern,  each Volume: 4 Folios + Editor     Folio 25,         or a Margins door
  Dramatis  (boss.editor_v1..=v4: Proofreader, boss.ur_editor,   into Marginalia
            Redactor, Continuity Editor,        the Last Word
            Abridger), 3 Emendations
```

Reference Standard run: 25 Folios × 300 s = 7,500 s = 125 minutes (doc 14 Example B scores 102,681). Pace anchors: character level 12 after Folio 10 and 28 ± 3 at Folio 25 (Canon §8); a major skill at 50 by Folio 10 ± 2 and 75 by Folio 22 ± 3 (Canon §7.2); 15 Emendations per run (Canon §13.7); at most 3 appearances per genre before Marginalia (Canon §17.1).

### 6.5 Weeks of meta

```
  RUN END ─► Quires ─► Bindery (options) ─────────┐
          ├► Ink ────► Inkwell (Echoes, Scripts) ──┤
          ├► Codex XP ► Codex tiers (cosmetics) ───┼──► wider next run, same power
          ├► Reliquaries ► Keepsakes (sealed) ─────┤
          └► Grades, Recollections, boards ────────┘
```

Reference income (doc 15 §2.7): 400 Ink per hour (2.5 pulls), 20,000 Codex XP per hour, 2.57 account Reliquaries per hour. Worked example at 5 hours per week: 12.5 pulls per week; the expected 48.956 pulls to a chosen featured Illuminated Echo take `48.956 / 12.5 = 3.92` weeks, the spark ceiling of 100 pulls takes 8 weeks, and the Codex reaches tier 100 in 8.4 weeks (doc 15 §6.7). There are no daily quests, no streaks and no login rewards (doc 15 AP-4).

Quires pay the Bindery: 60 per reference hour of failed runs (`Q_REF`) and 143 per hour of reference Grade-0 victories (`Q_WIN`, 299 Quires per 125 minutes) (doc 14 §1.1; doc 15 §2.4). The whole tree costs 7,670 Quires, of which achievements pay 1,935, leaving 5,735 from runs and the Codex free track: `5735 / 60 = 95.6` hours at the failed-run rate and `5735 / 143 = 40.1` hours at the victory rate (doc 14 §5.4). At 5 hours per week the Bindery completes in 8.0 to 19.1 weeks, and every node adds an option, never power.

### 6.6 Balance check against the canon anchors

This chapter sets no balance number; it checks that the loops above stay inside the canon's power tables (Canon §13.8, §17.2, §17.4). Every figure is recomputed in integer arithmetic. Player DPS is `30,000 mHP/s × P(d)`, then each item factor in the Canon §10.4 order (rarity, quality, `10000 + 600 × enh`, material), then a skill factor `floor_div((10000 + 30 × skill) × 10000, 10600)`, which rescales the reference's skill-20 term of Canon §6.4 row 12. Enemy HP is `60,000 mHP × H(d)` for a standard-rank enemy, solo. `TTK = ceil_div(HP × 60, DPS)` ticks. Material is `mat.iron` (10000) throughout.

| d | Kit | Player DPS (mHP/s) | Enemy HP (mHP) | TTK (ticks) |
|---|---|---|---|---|
| 1 | reference: Common, Normal, +0, skill 20 | 30,000 | 60,000 | 120 (2.0 s, `REF_STANDARD_TTK_D1`) |
| 10 | Common, Normal, +0, skill 20 | 65,154 | 206,538 | 191 |
| 10 | Rare, Good, +4, skill 50 (Canon §7.2 pace) | 106,054 | 206,538 | 117 |
| 25 | Common, Normal, +0, skill 20 | 237,330 | 1,215,126 | 308 |
| 25 | Epic, Good, +10, skill 75 (Canon §7.2 pace) | 555,095 | 1,215,126 | 132 |
| 25 | Epic, Good, +15 (all 15 Emendations), skill 75 | 659,176 | 1,215,126 | 111 |
| 25 | `ITEM_MULT_MAX` 51870, skill 20 | 1,231,030 | 1,215,126 | 60 |

The `H(d) / P(d)` gap reaches ×2.56 at Folio 25 (`202521 / 79110`). A kit on the canon pace closes it to within 12 ticks of the Folio-1 anchor, and the item ceiling halves the anchor, so neither the floor (5.1 s TTK with untouched starting gear) nor the ceiling (1.0 s) is degenerate.

Survivability, with a standard enemy hit of `10,000 mHP × T(d)` and Max HP at the Canon §8 level anchors (Canon §6.4 row 1). Errata armor scales with `T(ilvl)` and enemy damage with `T(d)`, so armor's damage reduction is unchanged with depth when `ilvl = d` and is left out:

| d | Level | Enemy hit (mHP) | Max HP at TGH 40 | Hits to down | Max HP at TGH 60 | Hits to down |
|---|---|---|---|---|---|---|
| 1 | 1 | 10,000 | 85,000 | 9 | 106,000 | 11 |
| 10 | 12 | 16,190 | 140,000 | 9 | 172,000 | 11 |
| 25 | 28 | 36,145 | 220,000 | 7 | 268,000 | 8 |

Party workload per player, from Canon §17.2: total enemy HP per room is `count_bp × hp_bp`, divided by the party size. Solo 10000, duo `20000 / 2 = 10000`, trio `31500 / 3 = 10500`, quad `44625 / 4 = 11156` Bp of the solo workload. With the quad's enemy damage at 13000, four players face 11.6% more work each and hits 30% harder; revives, ghosts and ally-targeted verbs are what `pillar.party_of_errata.t1` measures against that.

---

## 7. Player verbs master table

Doc 17 owns `input.` and every binding (Canon §19). This table copies doc 17 §12.1 (actions), §12.3 (`input.preset.folio_kbm`) and §12.4 (`input.preset.folio_pad`); where they differ, doc 17 wins and this table is corrected. "LB+" means while the gamepad glyph layer (`input.glyph_layer`, LB) is held. Toggle-or-hold defaults follow doc 17 §12.9. Input is never synthesized (Canon §11.8).

| Verb | Logical input (doc 17 §12.1) | KB/M | Gamepad | Systems touched | Owner |
|---|---|---|---|---|---|
| Move, strafe, backpedal | `input.move` | W A S D | LS | movement (Canon §4.2), footstep noise, collision | 03 |
| Look, aim | `input.look`; `input.look_gyro` | mouse | RS; gyro while LT is held | yaw and pitch, perception cone, aim | 05, 17 |
| Sprint | `input.sprint` | L-Shift hold | L3 toggle | stamina (Canon §6.4 row 6), noise | 04 |
| Walk quietly | `input.walk` | Caps Lock hold | not bound | footstep noise | 09 |
| Crouch, sneak | `input.crouch` | L-Ctrl hold | R3 toggle | 1200 mm capsule, stealth | 09 |
| Jump, air jump | `input.jump` | Space | A | `JUMP_H` by VERVE, fall damage | 03 |
| Dodge | `input.dodge` | L-Alt | B | 20 stamina, invulnerable ticks | 05 |
| Lean left, right | `input.lean_l`, `input.lean_r` | Q, E | LB+ LS X beyond 5000 Bp | eye offset 350 mm, head hitbox | 05 |
| Attack, fire | `input.attack` | Mouse 1 | RT | damage pipeline, jams, skill XP | 05 |
| Block, aim down sights, off-hand | `input.alt` | Mouse 2 | LT | stamina, poise, spread | 05 |
| Heavy, quick melee | `input.heavy` | Mouse 4, V | RB | poise damage, guard break | 05 |
| Reload, clear jam | `input.reload` | R | X | ammunition, Contradiction jams | 05 |
| Throw, cook | `input.throw` | G | LB+RB hold | ballistics, surfaces | 05, 08 |
| Cast the active spell | `input.cast` | Mouse 5, T | LB+RT | aether, glyphs, fizzles | 06 |
| Quick-cast wheel | `input.cast_wheel` | Tab hold | LB+LT hold, release selects | Grimoire, `WHEEL_SELECT` | 06, 17 |
| Next, previous spell | `input.grimoire_next`, `input.grimoire_prev` | ], [ | LB+ D-pad right, left | active spell | 06 |
| Cancel a cast | `input.cast_cancel` | the grave-accent key | requested from doc 17 | aether refund rules | 06 |
| Pray, prayer menu | `input.pray` | J (hold 20 frames: menu) | LB+ D-pad up (hold 20 frames: menu) | favor, patrons | 06 |
| Ability 1–4 | `input.ability_1..=4` | Z, X, C, B | LB+ A, B, X, Y | talents, cooldowns | 04 |
| Weapon 1–3; next, previous | `input.weapon_1..=3`; `input.weapon_next`, `input.weapon_prev` | 1, 2, 3; wheel down, up | D-pad right, left (hold 15 frames: weapon wheel) | swap ticks | 05 |
| Use a quickbar item: potion, food, throwable, placeable | `input.quickbar_1..=8` | 4 5 6 7 8 9 0 − | D-pad down uses the selected slot (hold 15 frames: quickbar wheel) | potions, food, placement mode | 04, 07, 08, 17 |
| Place a barricade, bridge span, ladder, trap, light or cover | a quickbar slot holding a `cat.placeable`, then `input.attack` to confirm, `input.alt` to cancel, `input.weapon_next` / `input.weapon_prev` to rotate | as listed | as listed | placement (Canon §4.3), `skill.fieldcraft`, spawn light | 07, 17 |
| Interact: open, loot, talk, revive (hold), carry Remains, station, Going Native, lockpick, hack, pickpocket, struggle | `input.interact` | F | Y | containers, doc 09 checks, Colophon services | 07, 09, 11, 14, 16 |
| Examine an item | `input.examine` (item-card action) | in the Book menu | in the Book menu | `Examine` intent, Analysis checks | 10, 17 |
| Ping; danger ping | `input.ping` | Mouse 3 (double-tap: danger) | D-pad up (double-tap: danger) | party comms | 16, 17 |
| Ghost: mark a target | `input.attack` on an entity in the ghost context | Mouse 1 | RT | `MarkTarget`: `st.marked` 480 ticks, 600-tick cooldown | 16 |
| Mount, dismount | `input.mount` | H | LB+L3 | `st.mounted`, 30-tick summon | 05 |
| Minion follow, hold, attack | `input.minion_follow`, `input.minion_hold`, `input.minion_attack` | F1, F2, F3 | LB+ D-pad down (hold 20 frames: command wheel) | minion AI, `MinionCommand` | 13 |
| First or third person | `input.camera` | F5 | LB+R3 | client only | 17 |
| Emote, chat, push to talk | `input.emote_wheel`, `input.chat`, `input.push_to_talk` | U hold, Enter, Y | D-pad up hold 15 frames; on-screen keyboard; LB+View hold | social and invader comms with mute, block and report | 16, 17 |
| Vote for an exit | stand in a door volume, or `input.menu_exits` then `ChooseExit` | walk; N inside a Colophon | walk; the exits list from the Book menu | lattice, exit commit after `TURN_HOLD_TICKS` = 120 | 14 |
| Trade, drop for an ally | `TradePropose`, `DropItemFor` intents from the Book menu | I | Menu | inventory ownership | 16 |
| Book menu, map, journal, character, scoreboard, pause | `input.menu_book`, `input.menu_map`, `input.menu_journal`, `input.menu_character`, `input.scoreboard`, `input.pause` | I, M, L, K, O hold, Esc | Menu, View, Book tabs, Book tabs, View hold 15 frames, LB+Menu | client only | 17 |

---

## 8. Session shapes

| Shape | Modes (doc 14 §8) | Party | Budget in seconds | Total | Stop points |
|---|---|---|---|---|---|
| SS-1 Solo daily | `mode.daily` | 1 | Margin 60 + Folios 1–4 at 255 each (1,020) + Editor Folio 330 + results 90 | 1,500 s = 25 min | solo suspend at any tick; first attempt per UTC date is ranked |
| SS-2 Full run | `mode.standard` | 1–4 | Margin 180 + 25 Folios at 300 (7,500) + results 120 | 7,800 s = 130 min | solo any tick; co-op at Folio entry |
| SS-2a Short full story | `mode.abridged` | 1–4 | Margin 180 + 15 Folios at 270 (4,050) + results 120 | 4,350 s = 72.5 min | as SS-2 |
| SS-3 Co-op night | `mode.standard` with co-op overlay | 4 | lobby and Margin 600 + 25 Folios at 320 (8,000) + one break at a Folio entry 400 | 9,000 s = 150 min | Folio-entry save resumes on another night (Canon §2.5) |
| SS-4 Endless Marginalia | `mode.endless` | 1–4 | per Marginalia Volume: 4 Folios at 300 + Editor Folio 360 = 1,560 | 26 min per Volume | "Close the Book" door after every Marginalia Editor (doc 14 §9.1) |

A 60-minute SS-4 session is two Marginalia Volumes (3,120 s) plus 480 s in the Margin. SS-1 uses `unlock.daily_baseline`, so new and veteran players compete on equal items. SS-3 is the co-op tuning target.

Shape rules:

- **SS-1** is one Volume: Folios 1 to 5 with `boss.editor_v1` final, Grade 2, `folio_scale_bp = 8000`, an Erratum (Calling and Origin) and one Gauntlet mutator drawn from `rng.run` at Folio index 0 (doc 14 §8.4). The ranked attempt is the first `Commit` per UTC date; later attempts that day are practice. Missing a day removes nothing.
- **SS-2** and **SS-2a** are the full story. Solo suspends at any tick boundary; co-op saves at each Folio entry, and a co-op group that stops mid-Folio replays that Folio from its entry (Canon §2.5).
- **SS-3** budgets 320 s per Folio, the doc 14 Example C pace for a trio and above. The 400-second break is taken at a Folio entry so the save point is fresh.
- **SS-4** opens only after `margin.milestone.first_standard_victory` and continues a Standard run through a Margins door (doc 14 §9.1). An Endless run suspends like SS-2 and ends at any Marginalia Editor's "Close the Book" door.

```
  SS-1  [M 60][F1 255][F2 255][F3 255][F4 255][Editor 330][R 90]                       = 1,500 s
  SS-2  [M 180][F1..F25 at 300 = 7,500 ................................][R 120]          = 7,800 s
  SS-3  [Lobby+M 600][F1..F12][break 400 at a Folio entry][F13..F25]  (25 × 320)        = 9,000 s
  SS-4  [F 300][F 300][F 300][F 300][Editor 360 | Close the Book?] per Marginalia Volume = 1,560 s
```

---

## 9. Audience, platforms, business model, ratings and tone

### 9.1 Audience

| Segment | Comes from | Wants | Serving pillar |
|---|---|---|---|
| Delvers | SPD, Caves of Qud, IVAN, NetHack | systems, identification, deep runs | 3, 5 |
| Looters | Borderlands, Destiny 2 | part-built loot, builds, numbers | 5 |
| Role-players | Morrowind, Oblivion, BG3 | use-based growth, spellmaking, tactics | 2, 3 |
| Friend groups | Barony, Deep Rock, PEAK | a co-op night with chaos and rescue | 4 |

Age target 16+ (DECISIONS #13). Primary input is mouse and keyboard; gamepad parity is required for Deck.

### 9.2 Platforms

PC first: Windows, Linux (x86_64, aarch64) and macOS 14+ on Apple silicon; Steam Deck Verified at 1280 × 800 and 60 fps; Steam Machine 10-foot UI; a solo WebGPU demo of Volume I. Switch 2, PS5 and Xbox Series follow 1.0 (doc 02 §14; DECISIONS #6).

### 9.3 Business model: Earned Randomness, Paid Certainty

| Line | USD | Random? |
|---|---|---|
| Base game | 24.99 in EA, 29.99 at 1.0 (EA owners keep it) | no |
| Folio Expansion (one genre each, every 9–12 months) | 12.99 | no |
| Codex premium track (100 tiers, never expires, Quill at tier 100) | 9.99 | no |
| Direct cosmetics, listed in full before purchase | 1.99–9.99 | no |
| Supporter pack | 4.99 | no |

There is no premium currency. Ink, Quires and Vellum are earned only and never purchasable (Canon §14). The Inkwell, account Reliquaries, in-run Reliquaries and Fortune Engines are full systems with pity and disclosed odds, funded only by play (doc 15 §1.2). `juris.paid_random_items_enabled` stays `false` everywhere, so one SKU ships worldwide with no age-verification vendor (brief §8).

### 9.4 Rating targets

PEGI 16, ESRB M, USK 16, ACB MA15+, ClassInd 16, GRAC 18, CERO D (doc 15 §8.6). The rating comes from violence, blood, severing and horror; monetization adds no rating trigger. No content is cut to chase a lower band.

### 9.5 Tone guide

**Voice: literary, wry, manuscript-themed.** The narrator is a tired, affectionate copy-editor who annotates the margins.

1. Sentences are short and concrete. One joke per paragraph, never at the player's loss.
2. Manuscript metaphors are the house dialect: pages, ink, folios, errata, colophons, binding. Never "level", "loot box" or "gacha" in diegetic text; the Inkwell is "drawing ink", a Reliquary is "unsealing".
3. Genres speak sincerely in their own register; the wink lives only in marginal notes and Editor lines.
4. Editors speak in style-guide rulings: "Strike. Unattested usage." "Query: why is this character still here?"
5. No casino lexicon anywhere near Inkwell, Reliquary or Fortune Engine text: no "jackpot", "spin", "bet", "wager", "slot machine" or "lucky streak" (doc 15 AP-7). The Scribe (NR-23) obeys the same rules.
6. Language stays mild in every genre (DECISIONS #8, doc 15 §8.6 descriptors). Gore is described plainly, never relished.
7. Numbers in diegetic text are spelled the way the UI shows them (Canon §1.5), so a line never contradicts a tooltip.

**House lexicon.** Diegetic text (item flavor, narrator notes, Editor lines, Folio intros) uses the right-hand column. Menus and tooltips may use the plain term in parentheses once, for clarity.

| Plain term | Diegetic term |
|---|---|
| floor, level | Folio, page |
| act, chapter of the run | Volume |
| run start, hub | the Margin |
| safe room at the end of a floor | the Colophon |
| Volume boss | an Editor |
| genre mini-boss | its genre's own word (doc 13 §9.3) |
| death, game over | erased; the run is "struck" |
| enemy pressure clock | Erasure, Red Ink |
| identify | read, collate |
| upgrade scroll | Emendation |
| reskin an item | Rewrite |
| gacha pull | drawing ink (the Inkwell) |
| loot box | unsealing a Reliquary |
| capsule machine | a Fortune Engine; "dispense", never "spin" |
| battle pass | the Codex |
| endless mode | Marginalia |
| PvP invader | a redactor |
| victory | Close the Book |

| Genre | Register | Sample line |
|---|---|---|
| Mythic Fantasy | high epic, archaic cadence | "The road remembers every king who walked it." |
| Gothic Horror | Victorian dread, formal | "Do not trouble the portrait, sir. It is listening." |
| Cosmic Horror | clinical calm over terror | "Measurements resumed. The angles were incorrect." |
| Space Opera | brass and optimism | "All hands, the stars are waiting." |
| Cyberpunk | clipped street slang | "Chrome's cheap. Names cost extra." |
| Post-Apocalypse | dry survivor humor | "Clean water. Don't ask what it used to be." |
| Weird West | laconic drawl | "Sun's low. So's the company." |
| Hardboiled Noir | first-person cynicism | "The rain was honest. Nobody else was." |
| Wuxia | aphorism, honor | "A blade drawn in anger cuts its owner first." |
| Steampunk | Victorian enthusiasm | "Splendid! It only exploded slightly." |
| Fairy Tale | storybook cadence, rule of three | "Thrice she knocked, and thrice the door said no." |
| Mythic Antiquity | invocation, epithet | "Sing, Muse, of the one who outlived her poem." |
| Oneiric | drifting second person | "You have always been on these stairs." |
| The Redaction | bureaucratic euphemism | "This corridor does not exist. Proceed." |
| The Unwritten | near-silence, single words | "Blank." |
| The Ur-Text | every register in counterpoint | "In the beginning was the draft." |

---

## 10. KPIs and success metrics

### 10.1 Measurement rules

- **Sources:** (a) opt-in usage telemetry (`flag.telemetry.usage`, default `false`; doc 02 §17.4, doc 16 §11.7), aggregated per run; (b) server-verified ranked submissions (doc 14 §9.4); (c) the economy ledger (doc 15 §10); (d) platform sales and review data; (e) crash reports under `flag.telemetry.crash_mode`; (f) headless bot sweeps, before launch and nightly (doc 02 §16).
- **Sample floor:** every KPI record carries `min_sample`, counted in the record's `sample_unit`. Defaults: 2,000 runs or sessions for source (a), 500 submissions for (b), 1,000 accounts for (c), 0 for (d), 2,000 sessions for (e), and the sweep size for (f). Below the floor the verdict is `Insufficient`. Opt-in telemetry means some KPIs stay `Insufficient`; that is accepted and never answered by making telemetry opt-out (Q-16).
- **Percentile:** nearest rank over the ascending sample, `p_q = sorted[ceil_div(q × n, 100) - 1]` for `n ≥ 1`. **Shares** are Bp: `floor_div(count × 10000, total)`. **Rates per 1,000** are `floor_div(count × 1000, base)`. Every value is an `i64`.
- **Window:** the trailing `window_days` UTC days ending at 00:00 UTC of the evaluation day. `no_rise` KPIs compare against the immediately preceding window of the same length.
- **Per-genre KPIs** (`per_genre: true`) are evaluated once per core genre; the KPI's verdict is `Fail` if any genre fails, else `Alert` if any alerts, else `Insufficient` if any is insufficient, else `Pass`.
- **Forbidden:** no KPI targets daily streaks, sessions per day or conversion of players under 18; no KPI justifies an A/B change to odds, pity, prices, Ink rates or Codex XP; A/B tests cover layout and copy only (doc 15 §12).

### 10.2 Record and evaluation

KPI records live in `content/01/kpi/kpis.json`. Fields: `id` (content ID; its description is the text key `loc.<id>.desc`, never a record field), `unit` (`"s"`, `"min"`, `"h"`, `"bp"`, `"count"`, `"per_1000"`, `"ink_per_h"`, `"quires_per_h"`), `stat` (`"p50"`, `"p90"`, `"share_bp"`, `"max_dev_bp"`, `"rate_per_1000"`, `"count"`), `mode` (`"band"`, `"no_rise"`, `"info"`), `per_genre` (bool), `window_days` (integer), `source` (`"telemetry_opt_in"`, `"ranked"`, `"ledger"`, `"platform"`, `"crash"`, `"bots"`, or an array of these), `sample_unit` (`"runs"`, `"folios"`, `"sessions"`, `"submissions"`, `"accounts"`, `"offers"`, `"hours"`, `"kills"`, `"skus"`, `"units"`), `min_sample`, `target_lo`, `target_hi`, `alert_lo`, `alert_hi` (integers, or `null` for an open bound: a null `_lo` loads as `i64::MIN` and a null `_hi` as `i64::MAX`; JSON never carries `i64` extremes, Canon §1.6), `owner_doc`. A `no_rise` or `info` record has all four bounds `null`.

```json
[
  { "id": "kpi.run.folio_seconds_p50", "unit": "s", "stat": "p50", "mode": "band", "per_genre": false,
    "window_days": 28, "source": ["telemetry_opt_in", "ranked"], "sample_unit": "folios", "min_sample": 2000,
    "target_lo": 180, "target_hi": 420, "alert_lo": 150, "alert_hi": 480, "owner_doc": 14 },
  { "id": "kpi.genre.door_pick_bp", "unit": "bp", "stat": "share_bp", "mode": "band", "per_genre": true,
    "window_days": 28, "source": ["telemetry_opt_in", "bots"], "sample_unit": "offers", "min_sample": 500,
    "target_lo": 3500, "target_hi": 6500, "alert_lo": 3000, "alert_hi": 7000, "owner_doc": 12 },
  { "id": "kpi.econ.minor_refunds", "unit": "count", "stat": "count", "mode": "no_rise", "per_genre": false,
    "window_days": 28, "source": "ledger", "sample_unit": "skus", "min_sample": 0,
    "target_lo": null, "target_hi": null, "alert_lo": null, "alert_hi": null, "owner_doc": 15 }
]
```

```rust
pub enum KpiMode { Band, NoRise, Info }
pub struct KpiRecord { pub id: Id32, pub mode: KpiMode, pub per_genre: bool, pub min_sample: u64,
                       pub target_lo: i64, pub target_hi: i64, pub alert_lo: i64, pub alert_hi: i64 } // null → i64::MIN / i64::MAX
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum KpiVerdict { Pass, Info, Insufficient, Alert, Fail }        // ordered from best to worst

/// value: this window's value; n: this window's sample count; previous: the preceding window's value.
pub fn evaluate(k: &KpiRecord, value: Option<i64>, n: u64, previous: Option<i64>) -> KpiVerdict {
    let Some(v) = value else { return KpiVerdict::Insufficient };
    if n < k.min_sample { return KpiVerdict::Insufficient; }
    match k.mode {
        KpiMode::Info => KpiVerdict::Info,
        KpiMode::NoRise => match previous { Some(p) if v > p => KpiVerdict::Alert, _ => KpiVerdict::Pass },
        KpiMode::Band => {
            if (k.target_lo..=k.target_hi).contains(&v) { KpiVerdict::Pass }
            else if (k.alert_lo..=k.alert_hi).contains(&v) { KpiVerdict::Alert }
            else { KpiVerdict::Fail }
        }
    }
}
/// Per-genre aggregation: the worst verdict, with Info treated as Pass.
pub fn aggregate(vs: &[KpiVerdict]) -> KpiVerdict {
    vs.iter().map(|&v| if v == KpiVerdict::Info { KpiVerdict::Pass } else { v }).max().unwrap_or(KpiVerdict::Insufficient)
}
```

`Alert` opens a tuning ticket for the owner doc. `Fail` blocks the next content update until it is fixed or the product owner waives it, except for `kpi.econ.*`, whose breach actions are doc 15 §12's. `Info` is recorded and never acted on.

### 10.3 KPI table

| ID | Measure (exact definition) | Mode | Target | Alert band | Source | Owner |
|---|---|---|---|---|---|---|
| `kpi.run.standard_minutes_p50` | median of `floor_div(end_tick - start_tick, 3600)` over `run.end.victory` runs of `mode.standard` | band | 100..=150 | 90..=165 | a, b | 14 |
| `kpi.run.abridged_minutes_p50` | the same over `mode.abridged` victories | band | 60..=75 | 50..=85 | a, b | 14 |
| `kpi.run.folio_seconds_p50` | median doc 14 §4.2 `secs_f` over cleared Folios of Standard and Abridged runs | band | 180..=420 | 150..=480 | a, b | 14 |
| `kpi.run.daily_minutes_p50` | median `floor_div(end_tick, 3600)` over ranked Daily submissions | band | 20..=30 | 15..=35 | b | 14 |
| `kpi.run.first_editor_5runs` | share of accounts with ≥ 5 runs whose first 5 non-Prologue runs include a `boss.editor_v1` death with the player present, Bp | band | ≥ 5000 | ≥ 4000 | a | 13, 14 |
| `kpi.run.g0_victory_bp` | share of Grade-0 `mode.standard` runs ending `run.end.victory`, among accounts with ≥ 10 runs, Bp | band | 1500..=3500 | 1000..=4500 | a, b | 13, 14 |
| `kpi.genre.door_pick_bp` | per core genre `g`: among committed 2-door offers that contain `g`, the share in which `g`'s door committed, Bp | band, per genre | 3500..=6500 | 3000..=7000 | a, f | 12 |
| `kpi.genre.contradiction_exposure` | share of (Folio entry, slot) pairs in which the maximum `C` over the slot's equipped components is ≥ 1 on the entry tick, Bp | band | 5000..=8500 | 4000..=9000 | a, f | 05, 11, 12 |
| `kpi.genre.disconnected_entry` | share of (Folio entry, slot) pairs in which an equipped weapon's base-function component has `C ≥ 6` on the entry tick, Bp | band | ≤ 1500 | ≤ 2500 | a, f | 05, 11, 12 |
| `kpi.pillar.rewrites_per_run_p50` | median count of committed Rewrites per `mode.standard` run, all slots | band | ≥ 2 | ≥ 1 | a | 11 |
| `kpi.pillar.systemic_kill_share` | `systemic_share` (below), Bp | band | ≥ 800 | ≥ 500 | a, f | 03, 08, 12 |
| `kpi.coop.party_parity` | `max over n in 1..=4 of abs(clear_n - mean)`, where `clear_n` is the Volume I clear share of Grade-0 Standard runs at party size `n` and `mean = floor_div(Σ clear_n, 4)`, Bp | band | ≤ 1000 | ≤ 1500 | a, f | 13, 14 |
| `kpi.coop.hours_share` | run hours with party size ≥ 2 over all run hours, Bp | band | ≥ 4000 | ≥ 2500 | a | 16 |
| `kpi.coop.desyncs_per_1000h` | `floor_div(desyncs × 1000, coop_hours)`, with `coop_hours = floor_div(coop_ticks, 216000)` and `min_sample` 1,000 hours | band | ≤ 5 | ≤ 10 | a | 02, 16 |
| `kpi.quality.crash_free_bp` | sessions without a crash over all sessions, Bp | band | ≥ 9950 | ≥ 9900 | e | 02 |
| `kpi.quality.review_positive_90d` | positive Steam reviews over all Steam reviews on EA day 90, Bp | band | ≥ 8500 | ≥ 8000 | d | 01 |
| `kpi.biz.units_90d` | units sold in the first 90 days of EA | band | ≥ 200,000 | ≥ 100,000 | d | 01 |
| `kpi.biz.return_14d` | share of buyers who start runs on ≥ 2 distinct UTC dates within 14 days of their first run, Bp; never driven by rewards or notifications | band | ≥ 6000 | ≥ 4500 | a | 01 |
| `kpi.fair.replay_verify_bp` | ranked submissions passing verification over all ranked submissions, Bp | band | ≥ 9990 | ≥ 9950 | b | 14, 16 |
| `kpi.econ.featured_hours_p50` | doc 15 §12: median reference hours from banner start to a player's first featured Illuminated, among players who pull on that banner | band | ≤ 25 | ≤ 30 | c | 15 |
| `kpi.econ.ink_per_hour_p50` | doc 15 §12: Ink from play per active hour, median | band | ≥ 350 | ≥ 300 | c | 15 |
| `kpi.econ.quires_per_hour_p50` | Quires from runs per active run hour, median over accounts | band | 50..=160 | 40..=200 | c | 14, 15 |
| `kpi.econ.codex_completion` | share of premium owners reaching tier 100 within 24 weeks of purchase, Bp | band | ≥ 6000 | ≥ 5000 | c | 15 |
| `kpi.econ.refund_rate` | refunded over purchased SKUs, 28 days, Bp | band | ≤ 300 | ≤ 500 | c, d | 15 |
| `kpi.econ.minor_refunds` | unauthorized-minor refunds per 28 days | no_rise | no rise over the previous window | any rise alerts | c | 15 |
| `kpi.econ.spend_top1_share` | revenue share of the top 1% of spenders, Bp | info | none | none | c | 15 |
| `kpi.econ.chat_report_rate` | chat and voice reports per 1,000 sessions | no_rise | no rise over the previous window | any rise alerts | c | 15, 16 |

Every window is 28 days except `kpi.quality.review_positive_90d` and `kpi.biz.units_90d`, which are evaluated once on EA day 90, and `kpi.econ.codex_completion`, whose window is each Codex's first 24 weeks.

**Systemic kills.** For each `ev.entity_killed` whose victim is not in `fac.errata`, let `h` be the `ev.damage_dealt` that its `damage_seq` names (the lethal hit). The kill is systemic when `h.flags` has `env` (bit 7) or `fall` (bit 8), or `h.source` is an entity spawned from a `cat.placeable` trap or a `cat.gadget` trap or mine, or `h.flags` has `dot` (bit 2) and the pulsing status was applied by a hit or a surface contact that is itself systemic by this rule (Canon §12.1 surfaces and reactions, §10.6 environmental hazards). `systemic_share = floor_div(systemic_kills × 10000, npc_kills)`.

**Quire rate.** `kpi.econ.quires_per_hour_p50` brackets doc 14's two references: 60 per hour (`Q_REF`, failed runs) and 143 per hour (`Q_WIN`, Grade-0 victories). A median below 50 means the Bindery would take more than `5735 / 50 = 114.7` hours; above 160 it would finish in under `5735 / 160 = 35.8` hours, faster than the horizontal tree is meant to last.

### 10.4 Worked examples

1. Folio seconds sample `[210, 250, 260, 290, 300, 305, 330, 360, 410, 470]` with a test record of `min_sample = 10`: `n = 10`, `p50 = sorted[ceil_div(50 × 10, 100) - 1] = sorted[4] = 300`, inside 180..=420: **Pass**. `p90 = sorted[ceil_div(90 × 10, 100) - 1] = sorted[8] = 410`.
2. Cosmic Horror offered 1,200 times in committed 2-door offers and picked 390 times: `floor_div(390 × 10000, 1200) = 3250` Bp, outside 3500..=6500 but inside 3000..=7000: **Alert**. If every other core genre passes, `kpi.genre.door_pick_bp` aggregates to **Alert**, a ticket for doc 12.
3. Volume I clear rates 5200, 5600, 4700, 6100 Bp for party sizes 1–4: mean `floor_div(21600, 4) = 5400`, deviations −200, +200, −700, +700, maximum 700: **Pass**. With 6600 for four players the mean is `floor_div(22100, 4) = 5525`, deviations −325, +75, −825, +1075, maximum 1075: **Alert**.
4. `kpi.econ.minor_refunds` counts 3 in the previous window and 4 in this one: `4 > 3`, **Alert**. Counts 4 then 4: **Pass**.
5. In a bot sweep, 9,000 NPC kills include 610 with an `env` lethal hit, 95 with `fall`, 140 from placed traps and 55 burning pulses from a reaction-ignited fire: `floor_div((610 + 95 + 140 + 55) × 10000, 9000) = 1000` Bp: **Pass**.
6. Desyncs: 7 desyncs over 1,512,000,000 co-op ticks: `coop_hours = floor_div(1512000000, 216000) = 7000`, `floor_div(7 × 1000, 7000) = 1` per 1,000 hours: **Pass**.

---

## 11. Questions for the product owner

Per the product owner's standing request, every system this chapter designs carries at least one open question: fiction (Q-01, Q-02, Q-17), pillars and the release gate (Q-03, Q-04, Q-18), inspirations and signature moments (Q-05, Q-19), the competitive bar (Q-06), novelty (Q-07), loops and balance (Q-08, Q-20), verbs (Q-09), sessions (Q-10), audience (Q-21), platforms (Q-11), business (Q-12, Q-24), ratings (Q-13), tone (Q-14, Q-23) and KPIs (Q-15, Q-16, Q-22). Each default below is binding until answered; answers land in DECISIONS.md.

| # | System | Question | Default in force |
|---|---|---|---|
| Q-01 | Fiction | Does the Last Word reveal that Editors were once Errata? | yes, one line in its defeat text; no branching |
| Q-02 | Fiction | Is "Close the Book" a homecoming or a sacrifice? | homecoming: the Erratum is written back into its story |
| Q-03 | Pillars | Should Lost Together (4) outrank The Page Is Alive (3)? | no; order as in §2 |
| Q-04 | Pillars | Raise the systemic kill share target from 800 to 1500 Bp? | 800 Bp |
| Q-05 | Inspirations | Which signature moment opens the trailer? | the revolver that jams in Fairy Tale while the wolf staggers (NR-02) |
| Q-06 | Competitive bar | All 16 genres at EA, or 8 core at EA and the rest in free updates? | all 16 at EA |
| Q-07 | Novelty | Ship the Scribe on by default? | off (`flag.scribe.enabled = false`), opt-in in settings |
| Q-08 | Loops | Keep Erasure onset at 420 s for four-player Folios that average 320 s? | 420 s (Canon §17.1) |
| Q-09 | Verbs | Q and E for lean, or for abilities? | lean on Q and E; abilities on Z, X, C, B and weapons on 1–3, as doc 17 §12.3 binds them |
| Q-10 | Sessions | Which structure does a 3–4 player lobby preselect? | Standard, with a one-click Abridged "short night" |
| Q-11 | Platforms | macOS at EA or at 1.0? | EA |
| Q-12 | Business | Folio Expansion at 12.99 (doc 15) or 14.99 (R3's upper bound)? | 12.99 |
| Q-13 | Ratings | Any regional low-gore build? | none; the gore slider is presentation only |
| Q-14 | Tone | Allow strong profanity in Noir, Cyberpunk and Post-Apocalypse? | mild language only |
| Q-15 | KPIs | Is 200,000 units in 90 days the commercial bar? | yes |
| Q-16 | KPIs | Make usage telemetry opt-out to measure KPIs? | no; opt-in, and KPIs accept `Insufficient` |
| Q-17 | Fiction | The canon calls PvP invaders "Redactors" and doc 13 names the Volume II Editor "the Redactor". Keep both, with invaders as lowercase "redactors" from the Redactor's office, or rename the invader role? | keep both; invaders are "a redactor" in text, the boss is "the Redactor" (§1.3) |
| Q-18 | Release gate | When both options of a design conflict break the Fair Page, should the rule reject both (`Neither`) or escalate? | `Neither`: both are rejected and a third option is required |
| Q-19 | Signature moments | Which scenario is the first playable target after the Rust vertical slice? | SM-10 "Torch the Dark", because it needs only the light grid, spawns and placeables; SM-06 next |
| Q-20 | Balance | The quad party faces 11,156 Bp of the solo workload per player and enemy damage ×13000 (§6.6). Ease it before data arrives? | no; keep Canon §17.2 until `kpi.coop.party_parity` reports |
| Q-21 | Audience | Add a fifth segment, competitive players (dailies, speed boards), with its own KPI? | no; served by PAR-07 and `kpi.fair.replay_verify_bp` |
| Q-22 | KPIs | Is a Quire rate band of 50..=160 per active hour right for the Bindery's 40-to-96-hour horizon? | yes |
| Q-23 | Tone | Is "erased" acceptable as the diegetic word for death, given the Editors' Erasure mechanic shares it? | yes; the overlap is intended |
| Q-24 | Business | Should the supporter pack stay at 4.99 or follow doc 15's cosmetic band up to 9.99? | 4.99 |

---

## Exports

All IDs pass Canon §1.1 and sit in the namespaces Canon §19 assigns to this chapter. None collides in `fnv1a32` with any other ID token in canon-1.4 or the spec chapters on disk (checked over every back-quoted ID in `spec/*.md`).

| Namespace | IDs |
|---|---|
| `pillar.` (24) | `pillar.one_ruleset`, `pillar.out_of_story`, `pillar.living_page`, `pillar.party_of_errata`, `pillar.every_find_a_question`, `pillar.fair_page`; tests `pillar.<name>.t1`, `.t2`, `.t3` for each (18) |
| `kpi.` (27) | the 27 IDs of §10.3: 25 earlier IDs, including doc 15's requested `kpi.econ.featured_hours_p50`, `kpi.econ.ink_per_hour_p50`, `kpi.econ.codex_completion`, `kpi.econ.refund_rate`, `kpi.econ.minor_refunds`, `kpi.econ.spend_top1_share`, `kpi.econ.chat_report_rate`, plus `kpi.run.abridged_minutes_p50` and `kpi.econ.quires_per_hour_p50` |

Record schemas: the pillar record and `release_gate` / `prefer` (§2.7); the KPI record, `evaluate` and `aggregate` (§10.2); the signature-moment scenario file `tests/moments/sm_NN.json` (§3.13); `syntax_delta` (§2.1). Doc-local labels, not content IDs: `PAR-01`…`PAR-14`, `NR-01`…`NR-26`, `SM-01`…`SM-11`, `SS-1`…`SS-4`, `Q-01`…`Q-24`. Voice rules for doc 18: the tone rules, the per-genre register table and the house lexicon of §9.5.

## Interface assumptions

| Doc | Assumed to provide |
|---|---|
| 02 | the bot harness with three policies (combat-led, stealth-led, environment-led) for `pillar.living_page.t3`, and the scenario runner for §3.13; the 5-target determinism set; `flag.telemetry.usage`, `flag.telemetry.crash_mode`, `flag.scribe.enabled`; nightly `perf.` gates; opt-in run summaries widened with the per-Folio fields listed under doc 16 |
| 03 | `ev.entity_killed.damage_seq` naming the lethal `ev.damage_dealt` of the same tick; `DmgFlags.env` set on every §10.6 environmental hazard hit, every Canon §12.1 surface or reaction damage and fire-spread ignition damage, and inherited by the DoT pulses of a status that an `env` hit or a surface contact applied |
| 04 | four ability slots; each of the 13 Calling kits lists its ally-targeting verbs (`pillar.party_of_errata.t2`) |
| 06 | the doc 06 §16.3 supplication algorithm with its fixed 4 keyed draws (SM-08); a levitation spell that compiles within complexity 8 (SM-05) |
| 07 | `cat.placeable` records used from the quickbar through doc 17's placement mode |
| 08 | identification by throwing (doc 08 §12.4) for SM-02 |
| 09 | the detection meter, awareness state machine and sneak eligibility used by SM-04 |
| 10, 11 | skin and item records lintable for `pillar.one_ruleset.t1`; a generator callable for `pillar.every_find_a_question.t2`; per-table rarity weight vectors for `pillar.every_find_a_question.t3` (b) |
| 12 | GPV fields `layout`, `verbs`, `laws` and the physics columns for `syntax_delta`; the exit preview of champion aspects (SM-11) |
| 13 | each roster template's behavior-set IDs for the Enemy-behavior channel of `syntax_delta` |
| 14 | exit forecasts; per-Folio entry and exit ticks, offered and chosen doors, Rewrite counts and party size in run history for KPIs |
| 15 | §12 econ KPI definitions, guardrails and breach actions; SEP rules; `palimpsest-verify`; Sealed Drafts and the Retrospective banner offline |
| 16 | replay verification, desync counts and an aggregate opt-in telemetry channel whose run summary also carries, per Folio: genre, entry and exit ticks, the offered doors and the committed door, the maximum equipped `C` per slot at entry, a Disconnected-weapon flag per slot, committed Rewrites, and NPC kills split into systemic and other; intents `ChooseExit`, `TradePropose`, `DropItemFor`, `MarkTarget` |
| 17 | the §12 action list and presets that §7 copies; a gamepad binding for `input.cast_cancel`; state icons; odds panels through `pct4`; `access.scribes_mercy` at 20%, +2% per death, cap 80% |
| 18 | adopts §9.5 (tone rules, registers, house lexicon) as the voice bible; enforces the casino-lexicon lint; distinguishes "the Redactor" (`boss.editor_v2`) and "a redactor" (invader) in text keys; Scribe output rules |

## Canon gaps

- **G1.** The contract header names canon-1.2; the canon on disk is canon-1.4, against which every rule here was checked. The canon-1.3 and canon-1.4 content used here (Colophon, World Laws, seams, Going Native, Unmoored, limited building, invasions) contradicts nothing in canon-1.2.
- **G2.** Canon §17.9 calls every PvP invader "a Redactor", and doc 13 §9.2 names the Volume II Editor "the Redactor" (`boss.editor_v2`). This chapter resolves the fiction (§1.3: invaders are junior redactors of the Redactor's office) and asks doc 18 to keep the two apart in text. Requested canon wording: "joins as a redactor, a junior Editor".
- **G3.** Doc 14 §11 Prologue room L3 teaches mining 3 blocks and placing 2, and doc 14 §5.1 lists `margin.plot` as a 64 × 32 × 64 building plot; Canon §4.3 and §17.8 (canon-1.4) fix terrain and replace the plot with trophy slots. §7 and §4.1 assume canon-1.4: placeables only, no terrain mining.
- **G4.** Canon §18 gives `ev.damage_dealt` no explicit environmental cause. `kpi.pillar.systemic_kill_share` and `pillar.living_page.t1` rely on doc 03's `DmgFlags` bits `env` and `fall` and on `damage_seq`; the inheritance of `env` by DoT pulses is requested from doc 03 (Interface assumptions).
- **G5.** Doc 02 §17.4 limits usage telemetry to aggregate run summaries (mode, Folio reached, end cause, party size). Five KPIs need more per-Folio aggregates (door offers and picks, Contradiction exposure, Disconnected entries, Rewrites, systemic kills). Requested: doc 02 and doc 16 add these aggregate fields to the opt-in run summary; they carry no personal data. Until then those KPIs come from source (f), bot sweeps.
- **G6.** No canon field records telemetry consent; assumed doc 16's opt-in channel, and opt-in KPIs may stay `Insufficient` (Q-16).
- **G7.** Doc 17 §12.4 binds no gamepad input to `input.cast_cancel`. §7 marks it requested; doc 17 owns the fix.
- **G8.** Canon §17.6 says a ghost "can ping and apply `st.marked`"; doc 17 §10.4 sends the ghost mark through `input.attack` as a `MarkTarget` intent. §7 follows doc 17.

Resolved since the previous draft of this chapter, recorded for traceability: doc 17 now owns and publishes every `input.` ID and binding (§7 copies it); doc 13 now uses doc 14's Editor IDs `boss.editor_v1` … `boss.ur_editor`; doc 12 now defines `law.*` and seam zones per Canon §5.8 and §5.10; doc 14 and doc 15 now agree on `Q_REF = 60` Quires per reference hour; doc 15's Canon gaps no longer conflate `mode.novella` with `mode.abridged`.

## Acceptance tests

1. **Pillar registry.** `content/01/pillar/pillars.json` holds exactly 6 records with `order` 1..=6, `hard_constraint` true only for order 6, each listing exactly 3 tests named `<id>.t1..t3`. `fnv1a32`: `pillar.one_ruleset` = `0x44b5d6ae`, `pillar.out_of_story` = `0x0f73a927`, `pillar.living_page` = `0xd366b69e`, `pillar.party_of_errata` = `0xbd9c6103`, `pillar.every_find_a_question` = `0xc83836a7`, `pillar.fair_page` = `0xab1fec97`; none collides with another loaded ID.
2. **ID grammar.** All 24 `pillar.` IDs and 27 `kpi.` IDs match `^[a-z][a-z0-9_]*(\.[a-z0-9_]+){1,5}$` and are at most 64 bytes. `fnv1a32("kpi.econ.featured_hours_p50") = 0x706610de`, `fnv1a32("kpi.run.abridged_minutes_p50") = 0x8cebfe46`, `fnv1a32("kpi.econ.quires_per_hour_p50") = 0xef673f34`.
3. **Release gate.** With every result from the current build: 17 tests `Pass` and `pillar.fair_page.t1` `NotRun` gives `false`; all 18 `Pass` gives `true`. At a later gate, `pillar.one_ruleset.t3` `Pass` from an older build with every other test `Pass` on the current build gives `true` when `genres_changed_since_t3` is false and `false` when it is true. At the EA gate the same older-build `Pass` gives `false`.
4. **Conflict rule.** `prefer([3], [4]) = B`; `prefer([6], [1]) = B`; `prefer([], [2]) = A`; `prefer([2], [2]) = Escalate`; `prefer([2, 3], [2]) = B`; `prefer([3, 2, 2], [2, 3]) = Escalate`; `prefer([6], [6, 1]) = Neither`; `prefer([2, 5], [2, 3]) = A`.
5. **Syntax delta.** Over the doc 12 GPVs on disk, `syntax_delta` counted on the Structure, Laws, Physics and Verbs channels alone is ≥ 3 for all 66 core pairs; Mythic Fantasy and Steampunk have equal Physics tuples and a Verbs distance of 48,000; Post-Apocalypse and Steampunk have the minimum Verbs distance, 13,000. The minimum axiom L1 distance is 6.
6. **Percentile.** §10.4 example 1 gives `p50 = 300` (`Pass`) and `p90 = 410`.
7. **Door pick.** 390 picks of 1,200 offers gives 3250 Bp and `Alert`; `aggregate([Pass, Alert, Pass])` is `Alert`; `aggregate([Pass, Info, Insufficient])` is `Insufficient`; `aggregate([Alert, Fail])` is `Fail`.
8. **Party parity.** Rates 5200, 5600, 4700, 6100 give maximum deviation 700 and `Pass`; 5200, 5600, 4700, 6600 give 1075 and `Alert`.
9. **Sample floor and modes.** `evaluate` with `n = 1999` and `min_sample = 2000` returns `Insufficient` for any value. A `no_rise` record with `previous = 3` returns `Alert` for 4 and `Pass` for 3. An `info` record returns `Info` for any value with `n ≥ min_sample`.
10. **Integer records.** A KPI record with `"target_lo": 180.0` is rejected by the content validator (Canon §1.6); a record with `"target_hi": null` loads with `target_hi = i64::MAX`; a `no_rise` or `info` record with any non-null bound is rejected.
11. **Systemic share.** §10.4 example 5 gives 1000 Bp. A kill whose lethal hit has neither `env` nor `fall`, whose source is a weapon, and which is not a DoT pulse is not systemic.
12. **Session budgets.** SS-1 sums to 1,500 s, SS-2 to 7,800, SS-2a to 4,350, SS-3 to 9,000, one SS-4 Volume to 1,560; the SS-2 run body of 7,500 s lies in 6,000..=9,000 s and SS-2a's 4,050 s in 3,600..=4,500 s (Canon §17.1).
13. **Loop math.** Folio-10 TTK with a Common Normal +0 ilvl-10 weapon is `ceil_div(206538 × 60, 65154) = 191` ticks; dodge invulnerability is 8 ticks at AGI 40 and 10 at AGI 70; a 5-enemy spawner in a 3-player party spawns 7 and carries 5000, then spawns 8 and carries 0.
14. **Balance table.** Every row of the §6.6 tables recomputes exactly from Canon §13.8 rows 1, 10 and 25, Canon §6.4 row 1 and Canon §17.2: TTK 120, 191, 117, 308, 132, 111, 60; hits to down 9, 11, 9, 11, 7, 8; per-player workload 10000, 10000, 10500, 11156.
15. **Erasure timeline.** Stack 1 lands at tick 25,200, stack 10 and the first hunter at 41,400, the second hunter at 46,800 after entry; a Colophon entry at tick 25,199 leaves 0 stacks.
16. **Marginalia cap.** The enhancement cap at Folio 50 is 20; SM-03 passes.
17. **Separation wall.** A `store.` SKU whose grant list contains `cur.ink` fails validation; a build with `juris.paid_random_items_enabled = true` fails to compile.
18. **Casino lexicon.** A `loc.` string bound to a `banner.`, `reliq.` or `engine.` ID containing a whole-word, case-insensitive "jackpot", "spin", "bet", "wager", "slot machine" or "lucky streak" fails the content build.
19. **Ledger completeness.** The inspiration ledger has 11 entries, each citing at least one owner doc in 02..=18 and naming one signature moment `SM-01`..`SM-11`; the novelty register has at least 25 entries, each with an owner; the peer table has at least 6 rows sourced from R3.
20. **Signature moments.** All 11 scenarios of §3.13 pass headless on every pull request; SM-09's light-column color passes the UI screenshot test.
21. **Bindings.** Every `input.` ID in §7 exists in doc 17 §12.1, and every KB/M and gamepad binding in §7 equals doc 17 §12.3 and §12.4, except entries marked "requested from doc 17".
22. **Genre floor.** `ceil_div(24, 3) = 8` core genres is a necessary lower bound under `GENRE_REPEAT_RULE`.
23. **JSON.** Every JSON block in this chapter parses, contains no fractional or exponent number, and every numeric field is an integer.

## Invariants annex

```
Σ 01: pitch ⊗ pillars ⊗ ledger ⊗ moments ⊗ bar ⊗ novelty ⊗ loops ⊗ balance ⊗ verbs ⊗ sessions ⊗ audience ⊗ tone ⊗ kpis
W! |pillars| = 6 ∧ ∀ pillar: |tests| = 3                 W! EA ⇒ ∀ 18 tests = Pass on the EA build
W! pillar.fair_page ≻ every other pillar                 W! conflict(1..5) → lower order wins; fair_page broken by both ⇒ Neither
W! mechanics := archetype; genre := data                 N! per-genre formula
W! ∀ core pair: syntax_delta ≥ 3 ∧ L1(axioms) ≥ 6       N! a genre that is only a palette swap
W! foreign gear allowed ∧ priced ∧ forecast              N! binary ban of foreign gear
W! sim deterministic ∧ replay-verifiable                 N! renderer-only gameplay effect
W! every signature moment := headless scenario test      N! a moment that only a trailer can show
W! every party size tuned                                N! loot competition by default
W! meta := horizontal                                    N! purchasable power ∨ purchasable randomness
W! every random container: pct4 odds ∧ pity visible      N! premium currency ∨ timers ∨ streaks
W! offline-complete except boards ∧ invasions ∧ cloud    N! content that needs a server to exist
W! KPI values i64 ∧ shares in Bp ∧ open bound := null    N! KPI target on streaks ∨ sessions/day ∨ minors
W! bindings := doc 17                                     N! a second binding table that disagrees with doc 17
W! tone := literary ∧ wry ∧ manuscript ∧ mild language   N! casino lexicon near Inkwell|Reliquary|Engine
W! Scribe text → cosmetic only                           N! LLM output → sim state
```
