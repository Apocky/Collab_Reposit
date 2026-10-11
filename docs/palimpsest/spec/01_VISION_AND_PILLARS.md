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
| Doc 02 | §1, §8.10, §14, §15, Exports | offline-complete rule, Deck targets, platforms, `perf.` budgets, `flag.telemetry.usage` default false |
| Docs 05, 06, 10, 11 | 05 §1, §4, §16; 06 §3, §16; 10 §5, §11, §12; 11 §5, §10, §11, §15, §17 | inputs, moveset, jams; casting, prayer; identification, Sealed contents, Index; manufacturers, Reliquaries, Fortune Engines, Rewrite, co-op loot |
| Docs 12, 13 | 12 §4, §13, §16; 13 §9.2 | Lattice, bleed, secret exits; the five Editors |
| Doc 14 | §3, §4.2, §5, §6, §7, §8, §9, §11 | Folio flow, score, Margin, Echoes, Grades, modes, Marginalia, Prologue |
| Doc 15 | §1, §2.7–§2.8, §3, §6, §8.6, §9, §11, §12 | business model, income, Inkwell, Codex, ratings, anti-predatory rules, end of service, economy KPIs |
| Doc 16 | §1.1, §3.6, §5.7 | topologies, crossplay default, intents |
| Research | R1, R2, R3, R5, R6, R7 as cited | facts, peer bar, law, precedents |

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
| Editors | the manuscript's correctors; they treat Errata as mistakes and erase them | `fac.editors`, Erasure, Volume bosses (Canon §11.6; doc 13) |
| Colophon | the sign-off at the end of every page, where the scribe rests | sanctuary room before the exits (Canon §17.1) |
| The Margin | the blank edge of the page where Errata gather between descents | meta hub (doc 14 §5) |
| The Ur-Text | the first text, under every other one; all axioms 5, nothing contradicts | Folio 25, `genre.ur_text`, C = 0 and E = 0 (Canon §5.5) |
| Marginalia | notes written past the end of the book | endless Folios 26 onward (doc 14 §9) |

**Premise.** Stories do not end in the Palimpsest; they are scraped thin and written over. When a page is scraped, a few characters slip through the knife and fall out of their plot. They wake in the Margin remembering one genre (their Origin), able to wield any other. The Editors see them as typographical sin. Five Editors guard the Volumes: the Proofreader, the Redactor, the Continuity Editor, the Abridger and the Last Word (doc 13 §9.2). Under every page lies the Ur-Text. An Erratum who reaches it can read the first sentence and choose: **Close the Book** and be written back into a story, or step through a Margins door and keep writing where no Editor has authority (doc 14 §9.1).

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
| `pillar.one_ruleset.t2` | all 66 pairs of core genres differ in at least 3 of 5 syntactic channels (signature verb, World Laws, §5.9 physics columns, layout style, roster behavior) and have axiom L1 distance ≥ 6 (Canon §5.2) | content lint over doc 12 GPVs |
| `pillar.one_ruleset.t3` | in a blind playtest of ≥ 20 testers, ≥ 8000 Bp name each core genre correctly within 10 s of Folio entry | playtest protocol, before EA |

**Anti-patterns.** Per-genre damage formulas; a genre that is a palette swap (Ultrakill Encore pattern outside bleed and Marginalia); genre-specific currencies with their own rules (Lucre is one value with 16 skins, Canon §14).

### 2.2 `pillar.out_of_story` — Out of Your Story

**Statement.** Carrying the wrong genre's tool is always possible and always priced: Contradiction costs potency and reliability, Anachronism Edge pays against natives, and the player sees both before committing.
**Why it matters.** This is the one decision no other roguelike offers, and it turns every exit choice, shop and Rewrite into a loadout question (Canon §5.5, §5.6).

| Test | Pass condition | Method |
|---|---|---|
| `pillar.out_of_story.t1` | in 1,000 solo Standard bot runs, ≥ 9000 Bp contain at least one exit offer whose doors give the bot different forecast `C` (doc 14 §3.6) | headless bot sweep |
| `pillar.out_of_story.t2` | for all 12 core genres × 13 Callings, the Folio-1 shop plus guaranteed loot holds ≥ 1 weapon with empty `dep` usable by a Calling major combat skill | generator sweep, 100 seeds per pair |
| `pillar.out_of_story.t3` | 10000 Bp of item views and HUD weapon widgets show the component state icon (Coherent, Strained, Contradicted, Disconnected) on the tick of `ev.contradiction_changed` | UI automation |

**Anti-patterns.** Binary bans of foreign gear; hidden Contradiction math; Edge above +24% (Canon §5.6); a genre in which no Calling can fight.

### 2.3 `pillar.living_page` — The Page Is Alive

**Statement.** Liquids flow, gas spreads, fire catches, light and sound propagate, bodies come apart, and all of it is deterministic, so the cleverest answer to a room is often the room itself.
**Why it matters.** Caves of Qud, IVAN and BG3 earn their stories from systems colliding; determinism lets the same collisions replay, verify and network (Canon §2, §3).

| Test | Pass condition | Method |
|---|---|---|
| `pillar.living_page.t1` | ≥ 800 Bp of kills in bot and opt-in live runs land from a surface, liquid reaction, fire spread, fall, gas or placed trap (`kpi.pillar.systemic_kill_share`) | event tally on `ev.entity_killed` sources |
| `pillar.living_page.t2` | golden replays give identical `state_hash` on all 5 CI targets (doc 02 §6.4) | determinism CI |
| `pillar.living_page.t3` | each Rubric type (`run.rubric.keeper`, `.catchwords`, `.seal`) is completed in every core genre by each of 3 bot policies: combat-led, stealth-led, environment-led | bot sweep, 200 seeds per cell |

**Anti-patterns.** Scripted set-piece hazards that ignore the grid; floats or `HashMap` in simulation state; effects that only the renderer knows (Canon §22).

### 2.4 `pillar.party_of_errata` — Lost Together

**Statement.** One to four players, every size tuned, each needing the others: revives, ghosts, carried Remains, shared knowledge, ally-targeted verbs and no competition for loot.
**Why it matters.** Barony proved the format; the 2026 bar is interdependence at every party size (R3 A2).

| Test | Pass condition | Method |
|---|---|---|
| `pillar.party_of_errata.t1` | Volume I clear rate per party size is within ±1000 Bp of the four-size mean (`kpi.coop.party_parity`) | bot sweep before EA; live KPI after |
| `pillar.party_of_errata.t2` | 13 of 13 Calling kits contain ≥ 1 verb that targets or benefits an ally | content audit against doc 04 kits |
| `pillar.party_of_errata.t3` | in default co-op, 0 equipment drops are collectable by more than one slot, except a killed mimic's Lootable state (doc 11 §10, §17) | sim test, 10,000 drops |

**Anti-patterns.** Solo-tuned encounters with HP multiplied; shared loot by default; a dead player watching a spectator camera for 20 minutes (ghosts act, Canon §17.6).

### 2.5 `pillar.every_find_a_question` — Every Find Is a Question

**Statement.** Items are built from parts, rolled per player, partly unknown and worth investigating; skills grow by what you do; meta progression widens options and never adds raw power.
**Why it matters.** Knowledge is the deepest loot; SPD, NetHack, Morrowind and Borderlands players return for it.

| Test | Pass condition | Method |
|---|---|---|
| `pillar.every_find_a_question.t1` | every type-hidden item type is identifiable by ≥ 3 distinct methods among use, appraisal text, Examine, price, Scriptorium service (doc 10 §5) | content audit |
| `pillar.every_find_a_question.t2` | for each weapon archetype, 100,000 generated drops at Folio 10 give ≥ 1,000 distinct (manufacturer, affix set, perk options) tuples | generator sweep |
| `pillar.every_find_a_question.t3` | on one Daily seed (`unlock.daily_baseline`), a fresh account and a fully unlocked account produce byte-identical item records at every Folio | sim test |

**Anti-patterns.** Raw stat unlocks in the Bindery; identification busywork for base types (Barony 5.0 rule, Canon §13.6); flat per-hit combat XP (Canon §7.1).

### 2.6 `pillar.fair_page` — The Fair Page

**Statement.** Earned Randomness, Paid Certainty: every random container is funded by play, every price buys a known thing, every roll is disclosed and verifiable, and the game survives its own servers.
**Why it matters.** Every 2026 loot-box regime keys on payment (R3 §0); Destiny 2's final update on 2026-06-09 shows live services end (R2 §6.1). Trust is a feature.

| Test | Pass condition | Method |
|---|---|---|
| `pillar.fair_page.t1` | SEP-1 to SEP-7 (doc 15 §1.2) pass; `juris.paid_random_items_enabled` is the compile-time constant `false` | content validator, ledger tests, build assert |
| `pillar.fair_page.t2` | Inkwell, account Reliquaries, in-run Reliquaries and Fortune Engines each show per-outcome odds to 4 decimals on the confirm screen plus live pity counters; doc 15 §4.3 verifier reproduces 10,000 logged pulls | UI automation, verifier run |
| `pillar.fair_page.t3` | with the network disabled, a bot completes the Prologue, Standard Folios 1–10, a LAN co-op Folio, an end-of-service Inkwell pull and Codex XP accrual; only invasions and ranked boards are refused | offline integration test |

**Anti-patterns.** Premium currency; timers on paid offers; streak rewards; odds hidden behind a link; content that needs a server to exist.

### 2.7 Pillar records and the release gate

```json
{ "id": "pillar.out_of_story", "order": 2, "hard_constraint": false,
  "tests": ["pillar.out_of_story.t1", "pillar.out_of_story.t2", "pillar.out_of_story.t3"],
  "owner_docs": [5, 6, 11, 12, 14, 17] }
```

```rust
pub enum Verdict { Pass, Fail, NotRun }
/// EA gate: all 18 tests Pass. A later release gate: all 18 Pass on that build,
/// except `pillar.one_ruleset.t3`, which reruns only when a genre is added or reskinned.
pub fn release_gate(pillars: &[Pillar], results: &BTreeMap<Id32, Verdict>) -> bool {
    pillars.iter().all(|p| p.tests.iter().all(|t| matches!(results.get(t), Some(Verdict::Pass))))
}
/// Conflict rule between two design options a and b that each fail some pillar test.
pub fn prefer(a_fails: &[u8], b_fails: &[u8]) -> Choice {      // pillar orders 1..=6
    let worst = |f: &[u8]| (f.contains(&6), f.iter().copied().min().unwrap_or(7));
    // An option failing pillar 6 always loses; otherwise the option whose best-ranked
    // failed pillar has the higher order number (less important) wins.
    match (worst(a_fails), worst(b_fails)) {
        ((true, _), (false, _)) => Choice::B, ((false, _), (true, _)) => Choice::A,
        ((_, x), (_, y)) if x > y => Choice::A, ((_, x), (_, y)) if y > x => Choice::B,
        _ => Choice::Escalate,                                     // product owner decides
    }
}
```

Example: option A fails `pillar.living_page.t1` (order 3); option B fails `pillar.party_of_errata.t1` (order 4). `worst(A) = (false, 3)`, `worst(B) = (false, 4)`; `4 > 3`, so B wins, because breaking Pillar 4 costs less than breaking Pillar 3.

---

## 3. Inspiration ledger

Each entry lists what we take exactly (with the owning doc), improve, reject, and the signature moment the game must reproduce as a scripted release check. Facts: R1, R2, R7.

### 3.1 Barony (Turning Wheel; 5.0.2 on 2026-02-18)

- **Take exactly:** first-person real-time 1–4 co-op roguelike frame (brief §1; docs 05, 16); dead players become ghosts who open doors and mark targets (Canon §17.6; doc 16); only an item's magic is hidden, never its base type (Barony 5.0 Lore; Canon §13.6; doc 10); blessed/uncursed/cursed states (doc 10); real-time hunger (`res.satiation`; doc 04); shared-loot co-op as the opt-in `mode.loot_shared` (Canon §17.5; doc 14).
- **Improve:** per-party-size tables for HP, count and damage (Canon §17.2); instanced loot by default; host-authoritative replays; 25 genre-distinct Folios instead of biome swaps.
- **Reject:** full-strength friendly fire on direct hits (default 5000 Bp, Canon §17.5); appraising mundane gear.
- **Signature moment — "The Ghost Opens the Door":** a party member dies in a trap room, floats ahead as a ghost, opens the far door and applies `st.marked` to the elite; the survivors carry the Remains through the exit and the ghost returns at 50% Max HP (docs 14, 16).

### 3.2 Shattered Pixel Dungeon (00-Evan; v4.0.2 on 2026-10-07)

- **Take exactly:** per-seed unknown appearances shuffled at run start (Canon §13.6; doc 10); the Scroll of Upgrade as the Emendation, 3 per Volume with the re-level rule (Canon §13.7; doc 11); strength requirement as attribute shortfall (Canon §6.3; doc 04); seeds and a daily seed (Canon §3.6; doc 14); deck-based category pacing (doc 11); same-ingredient brews that auto-identify (doc 08).
- **Improve:** real time and co-op; type knowledge shared across the party and across all 16 genre skins.
- **Reject:** turn-based clock; a run that ends at a fixed depth with no endless continuation.
- **Signature moment — "The Gamble Read":** cornered at low HP, the player drinks an unknown crimson flask; it is liquid flame, the room burns, the enemies burn with it, and the flask type is now known to the whole party in every genre's skin (docs 08, 10).

### 3.3 Experienced Pixel Dungeon (TrashboxBobylev; 2.19 final, archived)

- **Take exactly:** no level cap (Canon §8); endless depth with numbers that keep climbing, one Emendation per Marginalia exit and a rising enhancement cap (Canon §13.7; doc 14 §9); 64-bit resources from day one (Canon §1.2); boss loot bags as the Endpaper Illumined Reliquary and gilt Fortune Engine (doc 11 §10, §11).
- **Improve:** bounded inflation through `d_pow ≤ 300`, `DIFF_CAP_BP` and a proven overflow bound (doc 14 §9.2); Marginal mutations for variety (doc 14 §9.3).
- **Reject:** XP-to-upgrade grind in the main run; NG+ cycles with ×40,000 virtual depth.
- **Signature moment — "Numbers That Never Stop":** at Folio 50 the favored weapon reaches +20 (`15 + floor_div(50 - 25, 5) = 20`) and a Marginalia Editor falls to it (docs 11, 14).

### 3.4 Oblivion (2006; Remastered 2025)

- **Take exactly:** mastery tiers at 25/50/75/100 (Canon §7; doc 04); XP from every skill and from non-combat play (Canon §8; doc 04); detection as a meter with hysteresis thresholds (doc 09); spell cost rising superlinearly with magnitude (doc 06 §6).
- **Improve:** enemies scale with depth through `H(d)` and `T(d)`, never with player level (Canon §13.8); sound travels through the sound grid, not through walls (doc 03 §11).
- **Reject:** player-level enemy scaling; level-up bookkeeping; multiplying sneak and power multipliers.
- **Signature moment — "The Sleeping Guard":** crouched at light 2, the Shade's dagger sneak attack on a sleeping guard needs no crit roll, kills it, and the noise reaches only the open doorway (docs 05, 09).

### 3.5 Morrowind (2002)

- **Take exactly:** use-based skills that train their governing sub-attribute through marks (Canon §7.1; doc 04); spellmaking from effect parameters and enchanting (doc 06 §4, §9); levitation as a route-breaking verb (`st.levitating`; doc 06); FAT and SER govern no skill, like Luck (Canon §7).
- **Improve:** automatic training marks instead of rest-time multipliers; checked integer arithmetic instead of 16-bit wraparound (Canon §1.3).
- **Reject:** dice-rolled misses on a visible contact; unbounded spell magnitude.
- **Signature moment — "The Spell You Wrote":** the Spellwright composes a levitation glyph sentence within the complexity budget `4 + floor_div(LOG, 10)` (Canon §6.4 row 26) and floats the party over a chasm the layout expected them to walk around (doc 06).

### 3.6 Baldur's Gate 1–3 (1998, 2000, 2023; BG3 Patch 8 final)

- **Take exactly:** surfaces and elemental reactions (Canon §12.1; docs 03, 08); high ground ×11000 at ≥ 2,500 mm (Canon §10.5 step 4a; doc 05); visible sight cones and last-known position (docs 09, 17); rest between fights as the Interleaf (doc 14 §3.9); a forbidden power path as Red Ink Marginalia powers (Canon §16.4; doc 04 §10); faction reputation (Canon §15; doc 13).
- **Improve:** real time with tactical depth carried by surfaces, verticality and party roles; a reaction graph small enough to memorize (11 surfaces, 8 triggers).
- **Reject:** turn-based combat; parties of 6; dialogue trees as the primary verb.
- **Signature moment — "Grease and Fire from the Balcony":** the Alchemist oils a stair (`st.oiled`, fire buildup ×20000), the Gunslinger fires from a balcony 3 m above (×11000), and the Spellwright's fire glyph ignites the oil in a 2 m blast (docs 05, 06, 08).

### 3.7 Caves of Qud (Freehold; 1.0 on 2024-12-05)

- **Take exactly:** liquids as data (doc 08); per-part bodies, severing and reattachment (Canon §9.1; doc 04); mutations and implants (`kin.mutant`, `aug.`; doc 04); reputation bands (Canon §15); temperature (doc 03 §9); artifact examination as Examine (doc 10 §5); tinkering mods (doc 07).
- **Improve:** closed-form regeneration and every formula published (Canon §6.4); real-time 3D.
- **Reject:** penetration dice (R7 C1, Canon §10.5 wins); world-map travel.
- **Signature moment — "Losing an Arm and Keeping the Run":** a champion's cleaver severs the Gunslinger's left arm, the off-hand pistol drops, and the Warden sews the arm back on with a 600-tick Medicine channel before it rots at 7200 ticks (docs 04, 05).

### 3.8 IVAN (Iter Vehemens ad Necem)

- **Take exactly:** one material table for blocks and items (Canon §13.8a; doc 07); mass from material × volume; IVAN's prayer table with favor and timers (Canon §16.5; doc 06 §16); sacrifice value `isqrt(value)`, doubled for the matching patron.
- **Improve:** integer timers in ticks; one patron Author per genre; local-patron prayers × 15000.
- **Reject:** floating-point asymptotic attribute experience; misery without legibility.
- **Signature moment — "The Prayer at One HP":** with Death's Door spent, the Oracle prays to the local patron at favor 600; the patron heals the party; the next prayer before its timer expires angers it (doc 06).

### 3.9 Borderlands 1–4 (BL4 on 2025-09-12; level cap 70 on 2026-09-10)

- **Take exactly:** part-built weapons with manufacturer traits, two per core genre plus the Errata maker (Canon §5.9; doc 11 §5); a seven-step rarity ladder with colors (Canon §13.3); licensed cross-manufacturer parts (`cat.part`; docs 07, 11); cosmetic jackpots as Illuminated drops at 5 Bp (Canon §13.9); instanced co-op loot (Canon §17.1); a non-expiring pass in the Vault Card mould (doc 15 §6).
- **Improve:** published 4-decimal odds on every container (doc 11 §10); perk columns that are chosen, not rerolled; cross-genre legibility through Contradiction icons.
- **Reject:** slot machines with "nothing" outcomes and casino presentation (R7 C6; brief §8); gear obsolescence (the Emendation re-level answers it); shipping before PC performance is stable (doc 02 §15).
- **Signature moment — "The Orange Beam":** a Legendary drops with its `#FF9A1F` light column, one-sentence manufacturer trait and, at 5 Bp, an Illuminated skin that unlocks account-wide (docs 11, 18).

### 3.10 Minecraft (Java 26.3 "Wilderness Bound", 2026-09-15)

- **Take exactly:** 1 m blocks in 32³ chunks; two-channel integer light 0–15 (Canon §1.2); spawns gated by block light, so placed light is defense (Canon §12.3); liquids on the grid (doc 03 §6); random block ticks (Canon §2.2); commit-only offer streams (Canon §3.5); placeable bridges, ladders, barricades and lights (Canon §4.3; doc 07).
- **Improve:** authored structure stays intact because terrain is fixed except `destructible` blocks (DECISIONS #16); fixed-point determinism.
- **Reject:** terraforming and mining terrain; a goalless sandbox; rerolling offers by reopening a menu.
- **Signature moment — "Torch the Dark":** in a Gothic Horror Folio (`spawn_block_max` 7) the party plants lanterns until `light_block` exceeds 7 along a corridor, and the ambient spawns there stop (docs 03, 07, 12).

### 3.11 Destiny 2 (final content update 2026-06-09)

- **Take exactly:** gear tier orthogonal to rarity, setting perk options per column (Canon §13.9; doc 11); one Exotic per slot group (Canon §13.3); champions with counters shown in the exit preview (Canon §17.3a; docs 05, 12, 13); four ability slots (doc 04); rewards that never expire (doc 15).
- **Improve:** no Power grind; nothing vaulted; offline-complete end of service from day one (brief §8; doc 15 §11).
- **Reject:** premium currency (Silver); seasonal expiry; content that dies with the servers.
- **Signature moment — "Champion Check":** the exit preview shows an Unstoppable champion ahead; the party slots a `counter.staggerer` source, staggers it on cue and wins the room by preparation (docs 05, 12, 13).

### 3.12 Secondary precedents

Torg Eternity (axioms, World Laws), Noita (Holy Mountain, bleed), Hades II (God Mode), Slay the Spire 2 (co-op interdependence), Dungeon Crawler Carl (floor clock), Spider-Verse (foreign render styles); R6 §2–§6, R3 A3.

---

## 4. The 2026 competitive bar

### 4.1 Peers (R3 A1, R7 §9)

| Title | 2026 status | Players | Price (USD) | Bar it sets for us |
|---|---|---|---|---|
| Slay the Spire 2 | EA 2026-03-05 | 1–4 | 24.99 | co-op interdependence; EA price point |
| Hades II | 1.0 2025-09-25 | 1 | 29.99 | God Mode accessibility; cross-save |
| Elden Ring Nightreign | 2025-05-30 | 1–3 | premium | a clock compressing 3D runs to 40–60 min; tune every party size |
| Borderlands 4 | cap 70 on 2026-09-10 | 1–4 | premium | part-built loot; PC performance as a sales risk |
| ARC Raiders | 12M+ sold | 1–3 | 40 | crossplay on by default; free first pass track |
| Deep Rock Galactic: Rogue Core | EA 2026-05-20 | 1–4 | premium | first-person co-op roguelite |

### 4.2 Parity list (must ship)

| ID | Bar | Our commitment | Owner |
|---|---|---|---|
| PAR-01 | 1–4 co-op, every size tuned | Canon §17.2 tables; `pillar.party_of_errata.t1` | 13, 14, 16 |
| PAR-02 | drop-in | joins at Folio entry; invaders mid-Folio | 14, 16 |
| PAR-03 | crossplay on by default with opt-out | Steam at EA; relay route with the first non-Steam platform | 16 |
| PAR-04 | cross-save | Steam Auto-Cloud for `profile/` and `runs/` | 02 |
| PAR-05 | Steam Deck Verified, 10-foot UI | 1280 × 800 at 60 fps in `perf_saturation_4p` | 02, 17 |
| PAR-06 | accessibility suite | Scribe's Mercy (`access.scribes_mercy`), remapping, subtitles, color-independent signals | 17 |
| PAR-07 | seeds, daily, weekly | Canon §3.6; `mode.daily`, `mode.weekly_gauntlet`, `mode.seeded` | 14 |
| PAR-08 | session-sized runs | Daily 25 min, Novella 60 min, Abridged 60–75 min, Standard 100–150 min | 14 |
| PAR-09 | horizontal meta | Bindery adds options only | 14 |
| PAR-10 | modding | data packs, sandboxed WASM, Workshop | 02 |
| PAR-11 | live cadence | free systemic update every 6–8 weeks; Folio Expansion every 9–12 months; Codex every 12 weeks | 15, 21 |
| PAR-12 | launch breadth | all 12 core and 4 secret genres at EA (brief §3); the structural minimum is 8 core genres, because 24 non-final Folios at most 3 per genre need `24 / 3 = 8` | 12 |
| PAR-13 | PC performance | no EA build until every `perf.` budget is green on Deck and minimum PC | 02 |
| PAR-14 | end of life | offline solo, LAN and direct-IP co-op at 1.0; signed local ownership | 15, 16 |

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
- **NR-09 The dealt hand of exits (owner 12, 14, 17).** Each exit door shows genre, axioms, Laws, champion aspects and a per-player Contradiction forecast before the vote (doc 14 §3.6).
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
- **NR-21 Opt-in invasions as an Editor (owner 13, 14, 16).** Another player joins mid-Folio as a Redactor in replay slot 4; losing means being Redacted, not ending the run (Canon §17.9).
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

Reference numbers: a Folio-1 standard enemy has 60 HP and the reference player deals 30 HP per second, so time-to-kill is 2.0 s (120 ticks, Canon §17.4). At Folio 10, the same Common Normal +0 weapon at ilvl 10 deals `mul_bp(30000, 21718) = 65,154` mHP/s against `mul_bp(60000, 34423) = 206,538` mHP, so TTK is `206538 × 60 / 65154 = 190` ticks; rarity, quality, material and Emendations (up to `ITEM_MULT_MAX = 51870`) are how the player wins those 70 ticks back. A dodge costs 20 stamina and grants `min(12, 8 + floor_div(max(0, AGI - 40), 15))` invulnerable ticks: 8 at AGI 40, 10 at AGI 70 (Canon §4.2).

### 6.2 The 60-second encounter

```
  APPROACH ───► OPENER ───────────► FIGHT ─────────────► LOOT ────────► RECOVER
  unaware foes  sneak attack, Shock  party-scaled count,  instanced,     stamina after 1 s,
  light, noise  of the New, surface  ranks, counters,     sealed or      HP after 5 s at
                prep, placed trap    revives              known          0.5% Max HP/s
```

Worked example: a spawner with `n_base = 5` in a 3-player party spawns `floor_div(5 × 15000 + 0, 10000) = 7` enemies and keeps `carry = 5000` for its next wave (Canon §17.2).

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
| 41,400 (690 s) | stack 10: first hunter at the default threshold 10 |

### 6.4 The 2-hour run

```
  MARGIN ─► V I ─► V II ─► V III ─► V IV ─► V V ─► UR-TEXT ─► CLOSE THE BOOK
  Lectern,  each Volume: 4 Folios + Editor     Folio 25,         or a Margins door
  Dramatis  (Proofreader, Redactor, Continuity  the Last Word     into Marginalia
            Editor, Abridger), 3 Emendations
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

---

## 7. Player verbs master table

Logical inputs are doc 17's `input.` IDs; † marks IDs requested from doc 17. These defaults bind until doc 17 ships remapping and hold/toggle options. Gamepad "LB+" means while LB is held (the Grimoire layer). Input is never synthesized (Canon §11.8).

| Verb | Logical input | KB/M | Gamepad | Systems touched | Owner |
|---|---|---|---|---|---|
| Move, strafe, backpedal | `input.move`† | W A S D | LS | movement, noise, collision | 03 |
| Look, aim | `input.look`† | mouse | RS | perception cone, aim | 05 |
| Sprint | `input.sprint` | L-Shift hold | L3 | stamina, noise | 04 |
| Crouch, sneak | `input.crouch` | L-Ctrl hold | R3 toggle | stealth, capsule height | 09 |
| Jump, air jump | `input.jump` | Space | A | VERVE, fall damage | 03 |
| Dodge | `input.dodge` | L-Alt | B | stamina, invulnerability | 05 |
| Lean left, right | `input.lean_l`, `input.lean_r` | Q, E | L3, R3 while LT held | head hitbox, peeking | 05 |
| Light attack, fire | `input.attack` | Mouse 1 | RT | pipeline, jams, skill XP | 05 |
| Block, aim down sights, parry, off-hand | `input.alt` | Mouse 2 | LT | stamina, poise, spread | 05 |
| Heavy, quick melee, shove, bash | `input.heavy` | V | RB | poise, guard-break, taunt | 05 |
| Reload, clear jam | `input.reload` | R | Y | ammo, Contradiction | 05 |
| Throw, cook | `input.throw` | G | D-pad Left hold | ballistics, surfaces | 05, 08 |
| Cast active spell | `input.cast` | Mouse 4 | LB+RT | aether, glyphs, fizzles | 06 |
| Choose spell | `input.cast_wheel` | Tab hold | LB+RS | Grimoire | 06 |
| Cancel cast | `input.cast_cancel` | X | LB+LT | aether refund rules | 06 |
| Pray | `input.pray` | P | LB+D-pad Down | favor, patrons | 06 |
| Use ability 1–4 | `input.ability_1..=4` | 1 2 3 4 | LB+A/B/X/Y | talents, cooldowns | 04 |
| Select weapon 1–3, cycle | `input.weapon_1..=3`†, `input.weapon_next`† | 5 6 7, wheel | D-pad Up | swap ticks | 05 |
| Use quickbar item, place a placeable | `input.quickbar_use`† | Z tap | D-pad Down tap | potions, food, placeables | 04, 07, 08 |
| Choose quickbar item | `input.quickbar_radial`† | Z hold | D-pad Down hold | belt slots | 17 |
| Interact: open, loot, talk, revive (hold), carry Remains, station, Going Native, lockpick, hack, pickpocket, struggle | `input.interact` | F | X | containers, doc 09 checks, Colophon | 07, 09, 11, 14, 16 |
| Examine an inventory item | `input.examine` | Mouse 3 in inventory | X in inventory | identification | 10 |
| Ping, mark | `input.ping`† | Mouse 3 | D-pad Right | `st.marked` (ghost), comms | 16 |
| Mount, dismount | `input.mount` | H | LB+D-pad Up | `st.mounted` | 05 |
| Minion follow, hold, attack | `input.minion_follow`, `_hold`, `_attack` | F5, F6, F7 | View hold radial | minion AI | 13 |
| Emote, chat wheel | `input.comm_radial`† | T hold | View hold radial | social, invader comms | 16, 17 |
| Vote for an exit | stand in a door; `ChooseExit` intent | walk; F on door card | walk; X on door card | lattice, exit commit | 14 |
| Trade, drop for ally | `TradePropose`, `DropItemFor` intents | inventory (I) | inventory (Menu) | inventory, ownership | 16 |
| Inventory, map, Index | `input.inventory`†, `input.map`† | I, M, J | Menu, View tap | UI only | 17 |

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

- **Sources:** (a) opt-in usage telemetry (`flag.telemetry.usage`, default `false`, doc 02); (b) server-verified ranked submissions (doc 14 §9.4); (c) the economy ledger (doc 15 §10); (d) platform sales and review data; (e) crash reports under `flag.telemetry.crash_mode`; (f) headless bot sweeps, pre-launch and nightly.
- **Sample floor:** a KPI from source (a) needs `min_sample` qualifying records in its window (default 2,000 per 28 days), else its verdict is `Insufficient`.
- **Percentile:** nearest rank, `p_q = sorted[ceil_div(q × n, 100) - 1]`. **Shares** are Bp: `floor_div(count × 10000, total)`.
- **Forbidden:** no KPI targets daily streaks, sessions per day or conversion of players under 18; no KPI justifies an A/B change to odds, pity, prices, Ink rates or Codex XP (doc 15 §12).

### 10.2 Record and evaluation

```json
{ "id": "kpi.run.folio_seconds_p50", "unit": "s", "stat": "p50", "window_days": 28,
  "source": "telemetry_opt_in", "min_sample": 2000,
  "target_lo": 180, "target_hi": 420, "alert_lo": 150, "alert_hi": 480, "owner_doc": 14 }
```

```rust
pub enum KpiVerdict { Pass, Alert, Fail, Insufficient }
pub fn evaluate(k: &KpiRecord, value: Option<i64>, n: u64) -> KpiVerdict {
    let Some(v) = value else { return KpiVerdict::Insufficient };
    if n < k.min_sample { return KpiVerdict::Insufficient; }
    if (k.target_lo..=k.target_hi).contains(&v) { KpiVerdict::Pass }
    else if (k.alert_lo..=k.alert_hi).contains(&v) { KpiVerdict::Alert }
    else { KpiVerdict::Fail }
}
```

Open bounds use `i64::MIN` or `i64::MAX`. `Alert` opens a tuning ticket for the owner doc; `Fail` blocks the next content update until fixed or waived by the product owner.

### 10.3 KPI table

| ID | Measure | Target | Alert band | Source |
|---|---|---|---|---|
| `kpi.run.standard_minutes_p50` | median minutes of Standard victories | 100..=150 | 90..=165 | a, b |
| `kpi.run.folio_seconds_p50` | median Folio entry-to-exit seconds | 180..=420 | 150..=480 | a, b |
| `kpi.run.daily_minutes_p50` | median Daily minutes | 20..=30 | 15..=35 | b |
| `kpi.run.first_editor_5runs` | new accounts killing the Volume I Editor within 5 runs, Bp | ≥ 5000 | ≥ 4000 | a |
| `kpi.run.g0_victory_bp` | Grade-0 Standard runs won, accounts with ≥ 10 runs, Bp | 1500..=3500 | 1000..=4500 | a, b |
| `kpi.genre.door_pick_bp` | per core genre, pick share in 2-door offers, Bp | 3500..=6500 | 3000..=7000 | a, f |
| `kpi.genre.contradiction_exposure` | Folio entries with any equipped component at C ≥ 1, Bp | 5000..=8500 | 4000..=9000 | a, f |
| `kpi.genre.disconnected_entry` | Folio entries with a Disconnected equipped weapon base, Bp | ≤ 1500 | ≤ 2500 | a, f |
| `kpi.pillar.rewrites_per_run_p50` | median Rewrites per Standard run | ≥ 2 | ≥ 1 | a |
| `kpi.pillar.systemic_kill_share` | kills from surfaces, reactions, fire, falls, gas or placed traps, Bp | ≥ 800 | ≥ 500 | a, f |
| `kpi.coop.party_parity` | max deviation of Volume I clear rate per party size from the 4-size mean, Bp | ≤ 1000 | ≤ 1500 | a, f |
| `kpi.coop.hours_share` | run-hours in parties of 2 or more, Bp | ≥ 4000 | ≥ 2500 | a |
| `kpi.coop.desyncs_per_1000h` | desyncs per 1,000 co-op hours | ≤ 5 | ≤ 10 | a |
| `kpi.quality.crash_free_bp` | sessions without a crash, Bp | ≥ 9950 | ≥ 9900 | e |
| `kpi.quality.review_positive_90d` | positive Steam reviews 90 days after EA, Bp | ≥ 8500 | ≥ 8000 | d |
| `kpi.biz.units_90d` | units sold in the first 90 days of EA | ≥ 200,000 | ≥ 100,000 | d |
| `kpi.biz.return_14d` | buyers starting runs on ≥ 2 UTC dates within 14 days, Bp; never driven by rewards or notifications | ≥ 6000 | ≥ 4500 | a |
| `kpi.fair.replay_verify_bp` | ranked submissions passing verification, Bp | ≥ 9990 | ≥ 9950 | b |
| `kpi.econ.featured_hours_p50` | doc 15 §12: reference hours to first featured Illuminated | ≤ 25 | ≤ 30 | c |
| `kpi.econ.ink_per_hour_p50` | doc 15 §12: Ink per active hour | ≥ 350 | ≥ 300 | c |
| `kpi.econ.codex_completion` | premium owners at tier 100 within 24 weeks, Bp | ≥ 6000 | ≥ 5000 | c |
| `kpi.econ.refund_rate` | refunded ÷ purchased SKUs over 28 days, Bp | ≤ 300 | ≤ 500 | c, d |
| `kpi.econ.minor_refunds` | unauthorized-minor refunds per 28 days | no rise period over period | any rise alerts | c |
| `kpi.econ.spend_top1_share` | revenue share of the top 1% of spenders | informational | none | c |
| `kpi.econ.chat_report_rate` | reports per 1,000 sessions | trend only | none | c |

### 10.4 Worked examples

1. Folio seconds sample `[210, 250, 260, 290, 300, 305, 330, 360, 410, 470]` with a test record of `min_sample = 10`: `n = 10`, `p50 = sorted[ceil_div(50 × 10, 100) - 1] = sorted[4] = 300`, inside 180..=420: **Pass**. `p90 = sorted[ceil_div(90 × 10, 100) - 1] = sorted[8] = 410`.
2. Cosmic Horror offered 1,200 times in 2-door offers and picked 390 times: `floor_div(3900000, 1200) = 3250` Bp, outside 3500..=6500 but inside 3000..=7000: **Alert** to doc 12.
3. Volume I clear rates 5200, 5600, 4700, 6100 Bp for party sizes 1–4: mean `floor_div(21600, 4) = 5400`, deviations −200, +200, −700, +700, maximum 700: **Pass**. With 6600 for four players the mean is 5525 and the maximum deviation 1075: **Alert**.

---

## 11. Questions for the product owner

Per the product owner's standing request, every system this chapter designs carries an open question. Each default below is binding until answered; answers land in DECISIONS.md.

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
| Q-09 | Verbs | Q and E for lean, or for abilities? | lean; abilities on 1–4 |
| Q-10 | Sessions | Which structure does a 3–4 player lobby preselect? | Standard, with a one-click Abridged "short night" |
| Q-11 | Platforms | macOS at EA or at 1.0? | EA |
| Q-12 | Business | Folio Expansion at 12.99 (doc 15) or 14.99 (R3's upper bound)? | 12.99 |
| Q-13 | Ratings | Any regional low-gore build? | none; the gore slider is presentation only |
| Q-14 | Tone | Allow strong profanity in Noir, Cyberpunk and Post-Apocalypse? | mild language only |
| Q-15 | KPIs | Is 200,000 units in 90 days the commercial bar? | yes |
| Q-16 | KPIs | Make usage telemetry opt-out to measure KPIs? | no; opt-in, and KPIs accept `Insufficient` |

---

## Exports

All IDs pass Canon §1.1 and sit in namespaces Canon §19 assigns to this chapter.

| Namespace | IDs |
|---|---|
| `pillar.` | `pillar.one_ruleset`, `pillar.out_of_story`, `pillar.living_page`, `pillar.party_of_errata`, `pillar.every_find_a_question`, `pillar.fair_page`; tests `pillar.<name>.t1`, `.t2`, `.t3` for each (18) |
| `kpi.` | the 25 IDs of §10.3, including doc 15's requested `kpi.econ.featured_hours_p50`, `kpi.econ.ink_per_hour_p50`, `kpi.econ.codex_completion`, `kpi.econ.refund_rate`, `kpi.econ.minor_refunds`, `kpi.econ.spend_top1_share`, `kpi.econ.chat_report_rate` |

Record schemas: pillar record (§2.7), KPI record (§10.2). Doc-local labels, not content IDs: `PAR-01`…`PAR-14`, `NR-01`…`NR-26`, `SS-1`…`SS-4`, `Q-01`…`Q-16`.

## Interface assumptions

| Doc | Assumed to provide |
|---|---|
| 02 | bot harness and CI targets for pillar tests; `flag.telemetry.usage`, `flag.telemetry.crash_mode`, `flag.scribe.enabled`; nightly `perf.` gates |
| 04 | four ability slots; each of the 13 Calling kits lists its ally-targeting verbs (`pillar.party_of_errata.t2`) |
| 07 | placeables used from the quickbar with a placement preview |
| 09 | grid-driven stealth, sight cones and sleeping-target sneak attacks (doc not yet on disk) |
| 10, 11 | skin and item records lintable for `pillar.one_ruleset.t1`; a generator callable for `pillar.every_find_a_question.t2` |
| 12 | per-genre syntactic-channel fields for `pillar.one_ruleset.t2`; `law.*` records and seam zones per Canon §5.8, §5.10 |
| 14 | exit forecasts; per-Folio entry and exit ticks, Rewrite counts and party size in run history for KPIs |
| 15 | §12 econ KPI definitions and guardrails; SEP rules; verifier |
| 16 | replay verification, desync counts, an aggregate opt-in telemetry channel, intents `ChooseExit`, `TradePropose`, `DropItemFor` |
| 17 | adopts §7 defaults and adds `input.move`, `input.look`, `input.weapon_1..=3`, `input.weapon_next`, `input.quickbar_use`, `input.quickbar_radial`, `input.ping`, `input.comm_radial`, `input.inventory`, `input.map`; state icons; odds panels; `access.scribes_mercy` at 20%, +2% per death, cap 80% |
| 18 | adopts §9.5 as the voice bible; enforces the casino-lexicon lint; Scribe output rules |

## Canon gaps

- **G1.** The contract header names canon-1.2; the canon on disk is canon-1.4, against which every rule here was checked. The canon-1.3 and 1.4 content used here (Colophon, World Laws, seams, Going Native, Unmoored, limited building, invasions) contradicts nothing in canon-1.2.
- **G2.** `input.` belongs to doc 17, which is not yet on disk. §7 states default bindings and requests 10 input IDs (Interface assumptions).
- **G3.** Doc 15 Canon gap 17 calls the 15-Folio run `mode.novella`; doc 14 §8.1 defines `mode.abridged` (15 Folios, depths to 25) and `mode.novella` (Folios 1–15 at `d = f`) as different modes. This chapter follows doc 14 and Canon §17.1.
- **G4.** Editor IDs differ between doc 13 (`boss.editor.proofreader` … `boss.editor.last_word`) and doc 14 (`boss.editor_v1` … `boss.ur_editor`). This chapter uses display names only. Requested: one ID set with aliases.
- **G5.** Doc 14 §11 Prologue room L3 teaches mining 3 blocks and placing 2, and doc 14 §5.1 lists `margin.plot` as a building plot; Canon §4.3 and §17.8 (canon-1.4) fix terrain and make the Margin trophy slots. §7 assumes canon-1.4: placeables only.
- **G6.** Doc 12 Canon gap G10 assumes `gpv.law.*` and no seam zones; Canon §5.8 and §5.10 define `law.*` and `zone.seam`. NR-05 and NR-06 follow the canon.
- **G7.** Doc 14 §1.1 sets `Q_REF` at 60 Quires per reference hour; doc 15 §2.7 quotes 300 from the same constant. This chapter states no Quire rate and defines no Quire KPI until they agree.
- **G8.** No canon field records telemetry consent; assumed doc 16 provides an aggregate opt-in channel and opt-in KPIs may stay `Insufficient`.

## Acceptance tests

1. **Pillar registry.** Content holds exactly 6 `pillar.` records with `order` 1..=6, each listing exactly 3 tests. `fnv1a32`: `pillar.one_ruleset` = `0x44b5d6ae`, `pillar.out_of_story` = `0x0f73a927`, `pillar.living_page` = `0xd366b69e`, `pillar.party_of_errata` = `0xbd9c6103`, `pillar.every_find_a_question` = `0xc83836a7`, `pillar.fair_page` = `0xab1fec97`; none collides with another loaded ID.
2. **ID grammar.** All 24 `pillar.` IDs and 25 `kpi.` IDs match `^[a-z][a-z0-9_]*(\.[a-z0-9_]+){1,5}$` and are at most 64 bytes. `fnv1a32("kpi.econ.featured_hours_p50") = 0x706610de`.
3. **Release gate.** 17 tests `Pass` and `pillar.fair_page.t1` `NotRun` gives `false`; all 18 `Pass` gives `true`.
4. **Conflict rule.** `prefer([3], [4]) = B`; `prefer([6], [1]) = B`; `prefer([], [2]) = A`; `prefer([2], [2]) = Escalate`.
5. **Percentile.** §10.4 example 1 gives `p50 = 300` (`Pass`) and `p90 = 410`.
6. **Door pick.** 390 picks of 1,200 offers gives 3250 Bp and `Alert`.
7. **Party parity.** Rates 5200, 5600, 4700, 6100 give maximum deviation 700 and `Pass`; 5200, 5600, 4700, 6600 give 1075 and `Alert`.
8. **Sample floor.** `evaluate` with `n = 1999` and `min_sample = 2000` returns `Insufficient` for any value.
9. **Integer records.** A KPI record with `"target_lo": 180.0` is rejected by the content validator (Canon §1.6).
10. **Session budgets.** SS-1 sums to 1,500 s, SS-2 to 7,800, SS-2a to 4,350, SS-3 to 9,000, one SS-4 Volume to 1,560; the SS-2 run body of 7,500 s lies in 6,000..=9,000 s (Canon §17.1).
11. **Loop math.** Folio-10 TTK with a Common Normal +0 ilvl-10 weapon is 190 ticks; dodge invulnerability is 8 ticks at AGI 40 and 10 at AGI 70; a 5-enemy spawner in a 3-player party spawns 7 and carries 5000.
12. **Erasure timeline.** Stack 1 lands at tick 25,200 and stack 10 at 41,400 after entry.
13. **Marginalia cap.** The enhancement cap at Folio 50 is 20.
14. **Separation wall.** A `store.` SKU whose grant list contains `cur.ink` fails validation; a build with `juris.paid_random_items_enabled = true` fails to compile.
15. **Casino lexicon.** A `loc.` string bound to a `banner.`, `reliq.` or `engine.` ID containing a whole-word, case-insensitive "jackpot", "spin", "bet", "wager", "slot machine" or "lucky streak" fails the content build.
16. **Ledger completeness.** The inspiration ledger has 11 entries, each citing at least one owner doc in 02..=18; the novelty register has at least 25 entries, each with an owner.
17. **Bindings.** Every logical input in §7 exists in doc 17's binding table, and no two verbs in one input context share a default binding, checked separately for KB/M and gamepad.
18. **Genre floor.** `ceil_div(24, 3) = 8` core genres is the structural minimum under `GENRE_REPEAT_RULE`.

## Invariants annex

```
Σ 01: pitch ⊗ pillars ⊗ ledger ⊗ bar ⊗ novelty ⊗ loops ⊗ verbs ⊗ sessions ⊗ audience ⊗ tone ⊗ kpis
W! |pillars| = 6 ∧ ∀ pillar: |tests| = 3                 W! EA ⇒ ∀ 18 tests = Pass
W! pillar.fair_page ≻ every other pillar                 W! conflict(1..5) → lower order wins
W! mechanics := archetype; genre := data                 N! per-genre formula
W! foreign gear allowed ∧ priced ∧ forecast              N! binary ban of foreign gear
W! sim deterministic ∧ replay-verifiable                 N! renderer-only gameplay effect
W! every party size tuned                                N! loot competition by default
W! meta := horizontal                                    N! purchasable power ∨ purchasable randomness
W! every random container: odds 4 dp ∧ pity visible      N! premium currency ∨ timers ∨ streaks
W! offline-complete except invasions ∧ ranked boards     N! content that needs a server to exist
W! KPI shares in Bp ∧ integer records                    N! KPI target on streaks ∨ sessions/day ∨ minors
W! tone := literary ∧ wry ∧ manuscript                   N! casino lexicon near Inkwell|Reliquary|Engine
W! Scribe text → cosmetic only                           N! LLM output → sim state
```
