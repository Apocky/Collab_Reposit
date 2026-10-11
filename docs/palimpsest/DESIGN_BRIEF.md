# PALIMPSEST — Orchestrator Design Brief

As of 2026-10-11. Author: design orchestrator. Status: BINDING on every spec document.

This brief fixes the structural decisions. Spec authors may tune numbers when research justifies it, but must not change a decision marked **[FIXED]**. Anything marked **[TUNABLE]** has a starting value that later balance work may move.

---

## 1. High concept [FIXED]

**PALIMPSEST** (working title) is a first-person 3D co-op roguelike dungeon crawler set inside the Palimpsest. The Palimpsest is a bottomless manuscript-world in which every story ever told has been scraped away and written over by another. Each floor is a **Folio**. Each Folio is written in one fictional **Genre**. Adjacent Folios can bleed into each other.

The player characters are **Errata**. They are characters who fell out of their own stories. Because they belong to no story, they can carry a revolver into a fairy tale and a spellbook into a cyberpunk arcology. The antagonists are the **Editors**, who exist to correct errata by erasing them. The goal of a full run is to descend through five **Volumes** and reach the **Ur-Text**.

- Perspective: first person, with a third-person toggle. Full-body awareness in first person.
- Real-time action, like Barony, Oblivion, Destiny 2 and Borderlands.
- Players: 1 to 4 in co-op, like Barony. Runs can opt in to PvP invasions, in which a player from another session invades as an Editor (DECISIONS.md).
- World: block-based, like Minecraft in look and in its light and liquid simulation. Building is limited by product-owner decision: players place barricades, bridges, ladders, traps, light sources and deployable cover. Terrain is otherwise fixed, and only blocks tagged destructible break.
- Systemic simulation: materials, liquids, gases, temperature, body parts, light and sound, like Caves of Qud and IVAN.
- Loot: procedural, part-based and rarity-tiered, like Borderlands and Destiny 2.
- Growth: use-based skills, like Morrowind and Oblivion, plus character levels and talents, like Shattered Pixel Dungeon.
- Identification: randomized unknown appearances per seed, like Shattered Pixel Dungeon, NetHack and IVAN.
- Endless mode past the final Volume, like Experienced Pixel Dungeon.
- Tactical depth, party roles, surfaces and verticality, like Baldur's Gate 3.

## 2. The omni-genre engine: "one ruleset, many skins" [FIXED]

The core architectural idea is that mechanics are genre-neutral and genres are data.

1. **Archetype × Skin.** Every item, enemy, spell glyph, material, station, currency, status effect and UI theme has a genre-neutral **Archetype** that carries all mechanics. A **Genre Skin** carries the name, mesh, sound, VFX, text register and a small genre-specific perk pool. Example: `arch.melee.blade_long` skins to a longsword in Mythic Fantasy, a jian in Wuxia, a monomolecular katana in Cyberpunk, a cavalry sabre in Steampunk and a sharpened rebar machete in Post-Apocalypse.
2. **Axioms.** Every Genre defines four **Axioms**, each an integer 0 to 10. These follow the Torg reality-axiom precedent and the GURPS mana-level and tech-level precedent.
   - **ARCANA**: how well magic works.
   - **TECHNE**: how well technology works.
   - **MYTHOS**: how strongly the supernatural, divine, cursed and undead operate.
   - **VERVE**: how cinematic the reality is. It drives crit tendency, acrobatics, jump height and luck. It is high in Pulp and Wuxia and low in Noir and Cosmic Horror.
3. **Contradiction and Anachronism.** Every item and ability carries **axiom requirements** inherited from its origin genre. When the current Folio's axiom is below the requirement, the item suffers **Contradiction**: failure chance, potency loss, and at large gaps it becomes inert. When the item's axiom exceeds the local axiom, it gains **Anachronism Edge**: bonus effect against native creatures who have no countermeasures. A gun in a fairy tale may jam, but the dragon has never seen a gun. Exact formulas belong in the canon.
4. **Rewriting.** Items can be translated into the local genre's equivalent at a **Scriptorium** room or with a Rewrite consumable. Translation keeps archetype, rarity, material tier and most affixes. It swaps skin, axiom tags and the genre perk.
5. **Errata coherence.** Each Erratum has an **Origin Genre** that grants axiom tolerance in that genre's strong axioms. The Attunement sub-attribute raises general tolerance.

## 3. Launch Genres [TUNABLE list, FIXED count of 12 core + 4 secret]

Twelve core genres, each with an axiom vector [ARCANA, TECHNE, MYTHOS, VERVE] as a starting value: Mythic Fantasy, Gothic Horror, Cosmic Horror, Space Opera, Cyberpunk, Post-Apocalypse, Weird West, Hardboiled Noir, Wuxia, Steampunk, Fairy Tale, Mythic Antiquity. Four secret genres appear only by special conditions. The canon must define all 16 with IDs, axiom vectors, physics overrides, palette, music register, enemy roster themes, material pool, manufacturer list and currency skin.

## 4. Run structure [FIXED shape, TUNABLE numbers]

- 5 Volumes × 5 Folios = 25 Folios, then endless **Marginalia** Folios.
- Folio 5 of each Volume is an **Editor** boss Folio.
- Target full-run length is 100 to 150 minutes. Target Folio length is 3 to 7 minutes.
- Genre selection follows a **Genre Lattice**, a weighted graph. Players choose between 2 or 3 exits at the end of each Folio, which previews the next genre, like Hades doors.
- A hub called **the Margin** holds meta progression, the gacha, the Codex pass, the shop, the bestiary and the item codex.
- Death ends the run. Meta progression is horizontal: new Echoes, new items in the loot pool, cosmetics, codex knowledge. It is never a large raw-power gain.

## 5. Simulation and technology [FIXED]

- Language and engine: **Rust**. Rendering, audio, input and UI use **Bevy**, at the version the tech research recommends. The simulation lives in a separate pure-Rust crate `palimpsest_sim` with no Bevy dependency, so it can run headless for tests, servers, replays and bot playtesting.
- Determinism: the simulation is fully deterministic given a seed and an input log. Use fixed-point math for every simulation scalar. Never use platform floats in simulation state. All probabilities are integers in basis points (0 to 10,000). Each subsystem draws from a named RNG stream derived from the run seed.
- Tick: fixed simulation tick at **60 Hz**. Slower subsystems run on tick dividers. Rendering interpolates between simulation states.
- World: 1 block = 1 m cube. Chunks are 32 × 32 × 32 blocks. Liquids, gases, temperature, light and sound propagate on this grid. The simulation's light grid and sound grid drive stealth. The renderer has its own lighting, which must agree visually with the simulation light grid.
- Collision: custom swept capsules against the voxel grid and against hitbox capsules, inside the simulation. Ragdolls and cosmetic physics live in the renderer only and never feed back into the simulation.
- Multiplayer: host-authoritative with client prediction, server reconciliation and lag-compensated hit detection. Deterministic input logs enable replays and server-side verification of daily-challenge scores.
- Content: all content is data, in RON or JSON validated by JSON Schema, and hot-reloadable. Modding uses sandboxed WASM.
- Runtime language model: an optional on-device small language model may write flavor text such as item lore, Folio intros and NPC barks. It uses grammar-constrained JSON output and is cached by seed. It never affects simulation state. A template-grammar fallback must always exist.

## 6. Attributes [FIXED structure, TUNABLE numbers]

Eight primary attributes, each with three sub-attributes, for 24 sub-attributes. Sub-attributes are the real stored stats. A primary attribute equals the floor of the mean of its three sub-attributes. Primaries gate requirements. Sub-attributes feed formulas.

| Primary | Sub-attributes |
|---|---|
| MIGHT | Power, Brawn, Grip |
| FINESSE | Precision, Agility, Sleight |
| VIGOR | Toughness, Endurance, Resilience |
| INTELLECT | Logic, Memory, Analysis |
| WILL | Focus, Resolve, Attunement |
| PERCEPTION | Sight, Hearing, Intuition |
| PRESENCE | Charm, Menace, Guile |
| FORTUNE | Fate, Serendipity, Providence |

Scale: sub-attributes are integers from 1 to 100. Human baseline is 40, natural cap is 100, and the buffed cap is 150. Skills are integers from 0 to 100 with mastery thresholds at 25, 50, 75 and 100. Skills rise by use. Each skill is governed by one sub-attribute, and skill growth trains that sub-attribute. Character level comes from XP and grants allocation points and talents. Body parts have their own HP and wound states.

Fortune and Serendipity affect in-run loot only. They never affect paid gacha odds.

## 7. Required systems and where they live [FIXED]

| System | Spec document |
|---|---|
| Crafting, materials, building | 07 |
| Alchemy, liquids | 08 |
| Shooting and melee | 05 |
| Magic and spellcrafting | 06 |
| Attributes, sub-attributes, skills, body | 04 |
| Stealth and perception | 09 |
| Identification and appraisal | 10 |
| In-run loot boxes (Reliquaries) and in-run gacha machines (Fortune Engines) | 11 |
| Meta gacha (Inkwell), account-level Reliquaries, battle pass (Codex), store and compliance | 15 |

## 8. Monetization stance [FIXED]

Decision, revised after research R3: **Earned Randomness, Paid Certainty.**

- As of 2026-10-11, PEGI rates any new game with paid random items at PEGI 16 minimum (since June 2026). Brazil's ECA Digital (Lei 15.211/2025, in force since 2026-03-17) bans loot boxes in games likely accessed by minors. Belgium treats paid loot boxes as gambling. Australia requires an M rating for chance-based purchases and R18+ for simulated gambling. The EU Digital Fairness Act proposal is expected in November 2026. Every one of these regimes keys on payment.
- PALIMPSEST therefore ships gacha, loot boxes and a battle pass as complete, first-class systems, and **every randomized container is funded only by earned, non-purchasable currency**. Real money buys only deterministic goods. One SKU works worldwide with no age-verification vendor.
- Business model: premium base game at USD 19.99 in Early Access and at 1.0 (product-owner decision, DECISIONS.md item 42); paid Folio Expansions that each add a genre; the Codex premium track at USD 9.99; direct-purchase cosmetics; a supporter pack. There is **no premium currency**. Every purchase is a fiat SKU.
- **Inkwell** is the gacha. Pulls cost earned Ink. It draws **Echoes**, pre-built Errata with a signature trait, plus Scripts that widen run options and cosmetics. Duplicates give sidegrades and cosmetics, never flat stats.
- **Reliquaries** are the loot boxes, earned in play, never sold, with per-account bad-luck counters and sealed contents that tie into identification.
- **Fortune Engines** are in-run capsule machines that take in-run Lucre. Every capsule pays out an item. They have no wagering, no payout multipliers and no casino presentation, so they are not simulated gambling.
- **Codex** is the battle pass: 100 tiers, flat XP, earned through all play, never expiring, no timers, no sold tier skips, and 100% payback because tier 100 grants a Quill that unlocks the next premium Codex.
- Every random system shows exact odds and pity counters, is rolled on the server when online, and is provably fair through commit-reveal seed hashing.
- A jurisdiction feature-flag layer exists. `juris.paid_random_items_enabled` is `false` globally and is never flipped by this design.
- In-run Reliquaries and Fortune Engines use in-run currency and in-run keys only.
- **End-of-service guarantee.** Destiny 2 shipped its final content update on 2026-06-09 (research R2). PALIMPSEST ships an offline-complete end-of-service mode from day one: owned Echoes, cosmetics, Codex progress and unlocks stay usable locally, verified by signed receipts, and every gameplay system runs without the backend.

## 9. Document contract [FIXED]

Every spec document must:

1. Use the IDs, enums, units and formulas in `spec/00_CANON.md`. New IDs go only in the document's own namespace and are listed in an **Exports** section.
2. Give exact numbers, formulas with worked examples, data schemas with example records, state machines with every transition, and pseudocode for every non-obvious algorithm.
3. List the dependencies it consumes from other documents in an **Imports** section.
4. End with **Acceptance tests**: concrete checks an implementing agent can turn into automated tests.
5. End with an **Invariants annex** in compact W!/N! notation, where W! means must and N! means must not.
