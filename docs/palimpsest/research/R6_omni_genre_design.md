# R6: Omni-Genre / Genre-Agnostic Design: Precedents, Theory and a Genre Parameter Vector

As of 2026-10-11. Research dossier for PALIMPSEST (working title). It feeds `spec/00_CANON.md` §5 (axioms, Contradiction, Anachronism) and spec doc 12 (Genre Parameter Vector, Lattice, bleed zones).

**Evidence tags.** `[VERIFIED <url>]` means I fetched or saw the source in this session. `[VERIFIED-CODE <repo/file>]` means I read the value in source code cloned this session. `[VERIFIED-SNIPPET <url>]` means I saw it in a search-result summary, not on the fetched page. `[RECALLED]` means it comes from memory and was not checked. `[INFERENCE]` and `[PROPOSED]` mark my own analysis and design. **Method note:** the session's shared web-search budget ran out partway through. After that I used direct fetches of primary pages (Wikipedia, official sites, wikis, a GitHub repository, the official Cypher SRD). A few 2025-2026 facts that only a search could confirm (Fortnite Chapter 6/7 themes, Ultrakill 2026 layers, the full Torg Eternity World Law list) are therefore `[RECALLED]` and are listed under Open questions.

---

## Table of contents

1. Bottom line
2. Torg / Torg Eternity: the reality-axiom model (deep dive)
3. Genre-agnostic tabletop toolkits: GURPS, Savage Worlds, Fate, Cypher System
4. Multi-genre settings: Rifts, Shadowrun, Planescape, Caves of Qud
5. Dungeon Crawler Carl: floors, boxes, sponsors, and the 2025-2026 adaptations
6. Video-game precedents (Inscryption to Spider-Verse)
7. Genre theory: what concretely signals a genre
8. Pattern library: 24 reusable patterns
9. The Genre Parameter Vector (GPV): every dimension a genre must define
10. Candidate genres: 20 with defining traits and axiom vectors
11. Design implications for PALIMPSEST
12. Open questions and unverified items

---

## 1. Bottom line

1. **Torg is the right precedent, and its rules can be read from code, not just from previews.** The Foundry VTT system for Torg Eternity (MetaMorphic-Digital/torgeternity, commit `9f8e4bd`, 2026-10-10) hard-codes the axiom table for all 8 cosms. It also hard-codes the exact Contradiction algorithm: an item contradicts a reality if **any** of its axiom ratings exceeds that reality's axiom. It contradicts "1" if it exceeds the wielder's home **or** the zone, and "4" if it exceeds **both**. The wielder disconnects when the d20 shows a value ≤ that case number. Mixed zones use the **per-axiom maximum** of the two cosms. `[VERIFIED-CODE MetaMorphic-Digital/torgeternity module/config.js, module/documents/item/torgeternityItem.js, module/torgchecks.js, module/documents/torgeternityscene.js]`
2. **Axioms measure potential, not what is lying around.** Ulisses says so explicitly: "Axiom levels are more about the potential for a cosm, not what is actually found there" `[VERIFIED https://ulisses-us.com/?p=504]`. PALIMPSEST's canon reads axioms as "how well X works", which is the same stance. Keep it.
3. **Torg's 4th axiom (Social, which governs psionics) has no counterpart in PALIMPSEST.** PALIMPSEST has VERVE instead. That is defensible: Savage Worlds' "High Adventure" and "Conviction" setting rules, Torg's Nile "Law of Action", and Cypher's superhero "power shifts" all show that cinematic latitude is a genre dial. Psionics must still go somewhere. Recommendation: under ARCANA with a MYTHOS secondary (§11, R23).
4. **Genre-agnostic RPGs keep one core and vary four things:** (a) a small set of numeric dials (GURPS mana and Tech Levels, Cypher tech rating, Torg axioms); (b) toggleable setting rules (Savage Worlds has 17; Torg has 3 or 4 World Laws per cosm); (c) renamed archetypes (Cypher types renamed per genre, plus "Flavors"); (d) genre-only escalation subsystems (Cypher Horror Mode, Void Rules, Curse Mode, power shifts). PALIMPSEST needs all four in data.
5. **The Cypher System Reference Document (2025-08-22) gives the cleanest exact rules for genre-specific tension.** Horror Mode widens the GM-intrusion range by 1 per combat round, per 10 minutes of exploring, or per intrusion. Superhero difficulty caps at 15 instead of 10. Each tech-rating step of mismatch raises the higher vehicle's effective level by **2**. An item priced in a setting above its tech rating drops **one** price category. `[VERIFIED https://csol.montecookgames.com/license/ (CSRD docx)]`
6. **Noita's biome modifiers are the closest data precedent for genre bleed.** There is a base chance per biome (10%; Mines 20% but only after 8 deaths; Coal Pits 15%), then a weighted modifier list with exclusion lists. Copy that structure for bleed modifiers. `[VERIFIED https://noita.wiki.gg/wiki/Biome_Modifiers]`
7. **Spider-Verse proves that "every world is drawn by a different artist" is shippable.** It used 6 distinct animation styles and per-character frame rates: Spider-Punk's body on 3s or 2s, his guitar on 4s, and his outline on 2s or 1s. Use this to make out-of-genre items and Errata *visibly foreign*. `[VERIFIED https://en.wikipedia.org/wiki/Spider-Man:_Across_the_Spider-Verse]`
8. **Translation beats rejection.** Kingdom Hearts changes the party's bodies per world. Torg lets characters transform into denizens of the new cosm. Destiny 2: Renegades (2025-12-02) translated Star Wars into native systems: the Praxic Blade exotic sword is its lightsaber, and the Barant Imperium is its Galactic Empire. PALIMPSEST's Rewriting is the same pattern. `[VERIFIED https://en.wikipedia.org/wiki/Kingdom_Hearts; https://en.wikipedia.org/wiki/Destiny_2:_Renegades]`
9. **Change presentation freely, but change only a few core rules.** Inscryption changes medium, camera and art style each act. Its card rules stay the same, and the only board change is 3×4 becoming 3×5 in Act 3. `[VERIFIED https://en.wikipedia.org/wiki/Inscryption]`
10. **Dungeon Crawler Carl is a diegetic monetization frame, not a mechanics source.** It has timed floors that are "reclaimed", six box tiers from Bronze to Celestial, patrons who "sponsor you by purchasing boxes for you", and achievements that pay out boxes. It has a Peacock live-action series (straight-to-series order, 2026-06-22) and a Renegade tabletop RPG plus a deck-builder (BackerKit launch 2026-04-14). I found **no announced DCC video game** as of this research. `[VERIFIED https://www.goodreads.com/notes/54659324-dungeon-crawler-carl/169623639-adrianna-riddle; https://fuzzydoor.com/news/p/dungeon-crawler-carl-tv-series-ordered-by-peacock/; https://renegadegamestudios.com/blog/renegade-game-studios-announces-two-upcoming-dungeon-crawler-carl-tabletop-games/]`
11. **The crossover live-service failure is MultiVersus.** Servers closed 2025-05-30, and Warner Bros. took a USD 100 million writedown. The crossover hook worked: 20 million players by September 2022. The content cadence and monetization did not. `[VERIFIED https://en.wikipedia.org/wiki/MultiVersus]`
12. **A genre is recognized from semantic signals (palette, iconography, music, typography) within seconds. It is *confirmed* by syntactic signals (verbs, laws, enemy behavior, structure) within minutes.** A Folio that changes only semantic signals is a reskin (an Ultrakill "Encore"), not a genre. The GPV in §9 enforces a minimum syntactic delta.

---

## 2. Torg / Torg Eternity: the reality-axiom model (deep dive)

### 2.1 What Torg is

Torg (West End Games, 1990) and Torg Eternity (Ulisses Spiele) are set during the "Possibility Wars". Invading realities ("cosms") overwrite regions of Earth, and each cosm keeps its own physics of possibility. Eternity's Kickstarter began 2017-05-31 and raised USD 355,992 against an 8,000 goal. Original WEG cosm books: Living Land, Nile Empire, Aysle and Cyberpapacy in 1990; Orrorsh and Nippon Tech in 1991; Tharkold in 1992. Nippon Tech was renamed Pan-Pacifica in Eternity. `[VERIFIED https://en.wikipedia.org/wiki/Torg]` Each cosm has "four axioms and three World Laws" in the original (Wikipedia's summary). Eternity cosm books add a fourth law in some cases (Nile and Tharkold each list three, below).

### 2.2 The axioms (Eternity)

- **Four axioms: Magic, Social, Spirit, Tech.** The axioms of a cosm define which tools work there: spells, miracles, social concepts and psionics, and technology. `[VERIFIED https://ulisses-us.com/?p=504]`
- **The scale is 0 to 30 with no gaps.** Eternity removed "split" axioms. Social alone governs psionics. Magic axiom requirements depend on spell power, not only school. Higher Spirit levels enable stronger miracles, and each level defines what miracles can do. `[VERIFIED https://ulisses-us.com/?p=504]`

**Axiom values per cosm, as shipped in the official Foundry VTT system** `[VERIFIED-CODE MetaMorphic-Digital/torgeternity module/config.js, axiomByCosm]`. The last four columns convert each value to PALIMPSEST's 0-10 scale using `round(x/3)` `[INFERENCE]`.

| Cosm | Magic | Social | Spirit | Tech | Genre it encodes | →ARC | →(SOC) | →MYT | →TEC |
|---|---|---|---|---|---|---|---|---|---|
| Core Earth | 9 | 23 | 10 | 23 | contemporary thriller | 3 | 8 | 3 | 8 |
| Aysle | 24 | 16 | 18 | 14 | high fantasy | 8 | 5 | 6 | 5 |
| Cyberpapacy | 14 | 18 | 16 | 26 | theocratic cyberpunk | 5 | 6 | 5 | 9 |
| Living Land | 1 | 7 | 24 | 6 | primeval or lost world | 0 | 2 | 8 | 2 |
| Nile Empire | 14 | 20 | 18 | 20 | 1930s pulp | 5 | 7 | 6 | 7 |
| Orrorsh | 16 | 18 | 16 | 18 | Victorian gothic horror | 5 | 6 | 5 | 6 |
| Pan-Pacifica | 4 | 24 | 8 | 24 | corporate cyberpunk / bio-horror | 1 | 8 | 3 | 8 |
| Tharkold | 12 | 25 | 4 | 25 | techno-demonic hellscape | 4 | 8 | 1 | 8 |

**Preview versus shipped.** The 2016 previews said Aysle Magic 25 `[VERIFIED https://ulisses-us.com/?p=504]` and Nile Empire 13/20/18/20 `[VERIFIED https://ulisses-us.com/?p=594]`. The shipped system data says Aysle 24 and Nile Magic 14. Tharkold matches in both (12/25/4/25). **Use the code values.**

**Lessons from the table** `[INFERENCE]`:

- **No cosm is high on everything.** Each cosm's four values sum to between 38 (Living Land) and 75 (Core Earth). The spread *between* a cosm's axioms is what makes it distinctive. Living Land has Spirit 24 and Tech 6, while Orrorsh is flat at 16-18. Orrorsh's horror lives in its World Laws, not in its axioms. This matters for PALIMPSEST. **A genre whose identity is a mood (horror, noir) needs Laws, not extreme axioms.**
- **Social 23-25 marks modern and future cosms.** Living Land's Social 7 makes democracy, money and psionics contradictions there.
- **Gaps between neighbors drive play.** Core Earth tech (23) in Aysle (14) is a 9-point gap. Torg treats any positive gap identically, as a binary contradiction. PALIMPSEST's canon grades by gap size. That is better for real time (see §2.8).

### 2.3 Item axiom anchors (shipped data)

These ratings come from the archetype compendium in the same repository `[VERIFIED-CODE MetaMorphic-Digital/torgeternity src/packs/archetypes/*.yml]`. They show how Torg places items on the Tech scale, and they show that one item can carry **several** axiom ratings.

| Item | Type | Axiom rating | Home cosm |
|---|---|---|---|
| Hrockt Shoot Spear | melee | Tech 2 | Living Land |
| Great Axe | melee | Tech 10 | Aysle |
| Dragon Warrior Armor | armor | Tech 12 | Aysle |
| Healing Potion | gear | **Tech 12 + Magic 13** | Aysle |
| Caltrops | gear | Tech 14 | Aysle |
| Electro-Ray | firearm | Tech 20 (weird science, carried by World Laws) | Nile Empire |
| Sunglasses | gear | Tech 20 | Nile Empire |
| IriMesh Clothing | armor | Tech 24 | Pan-Pacifica |
| Diffrax Plate, Pain Gauntlet, Digan CyberArm | armor, melee, implant | Tech 25 | Tharkold |
| Cyberdeck, GWI GodMeeter, GodBeam Laser, cyber-eye | gear, firearm, implant | Tech 26 | Cyberpapacy |
| Scrambler (spell) | spell | Magic 9 | none |
| Diminish, Enhance (spells) | spell | Magic 10 | none |
| Magical Affinity (Elf) | perk | Magic 12 | none |
| Ward Enemy, Bless (miracles) | miracle | Spirit 8, Spirit 9 | none |
| Energize (psionic power) | psionic | Social 20 | none |

The French basic-rules text confirms the threshold semantics: in the Living Land (Tech 6), "if the number is greater than 6, the object is a contradiction" (my translation) `[VERIFIED-CODE torgeternity src/packs/basic-rules-fr/Axiomes_et_lois_du_monde_86g2Wc6mJshFotQK.yml]`.

**Era mapping** `[INFERENCE from the anchors plus GURPS TLs in §3.1]`: Tech 2-6 ≈ TL0 Stone Age. Tech 10 ≈ TL2-3. Tech 14 ≈ TL3-4, late medieval to Renaissance. Tech 18 ≈ TL5, Victorian (Orrorsh). Tech 20 ≈ TL6, 1930s (Nile). Tech 23 ≈ TL8, the present (Core Earth). Tech 24-26 ≈ TL9-10, cyberpunk. Tech 27-30 ≈ TL11-12.

### 2.4 Contradiction: the exact algorithm

Reconstructed from the shipped code `[VERIFIED-CODE torgeternityItem.js lines 331-394; torgchecks.js lines 168-196]`:

```
isContradiction(item, maxAxioms):          // maxAxioms = {magic, social, spirit, tech}
    fails = [ k for k in axioms if item.axioms[k] > maxAxioms[k] ]
    return fails (empty → no contradiction)

actorContradiction = isContradiction(item, actor.homeAxioms)     // wielder's own reality
zoneContradiction  = isContradiction(item, zoneAxioms)           // where the wielder stands
                     OR (item is a cosm-bound perk AND zone does not include that cosm)
contradictionCase  = 4 if actorContradiction and zoneContradiction
                     1 if exactly one of them
                     0 otherwise

on any skill test using the item:
    if contradictionCase > 0 and d20_result <= contradictionCase:  → DISCONNECTED
```

- **Per-use probability:** 1/20 (5%) for a single contradiction and 4/20 (20%) for a double one. The expected uses before disconnection are 20 and 5. A d20 result of 1-4 can only come from the first die, because Torg's d20 rerolls and adds on 10 and 20 `[RECALLED]`.
- **Cosm-bound ("starred") perks.** "Perks with this symbol are dependent on their cosm's World Laws and can cause Contradictions outside that cosm." `[VERIFIED-CODE torgeternity lang/en.json]` These are *law-bound* abilities. They contradict anywhere their law is absent, whatever the axioms say.
- **Long-range contradictions** (for example a thrown grenade) now behave like normal ones. The grenade explodes as long as you did not disconnect when you threw it. `[VERIFIED https://ulisses-us.com/?p=504]`
- **Passive contradictions** (for example high-tech armor) are resolved with a disadvantage on the conflict line of the Drama Deck, rather than a die roll per use. `[VERIFIED https://ulisses-us.com/?p=504]`

### 2.5 Zones: Pure, Dominant, Mixed

- Invaded regions are split into triangular zones of different strength: Pure, Dominant and Mixed. Pure zones are "super-strong Dominant zones". Ords (non-heroes) cannot create contradictions there, and reconnection is very difficult. `[VERIFIED https://ulisses-us.com/?p=504]`
- **Zone axioms in code:** a Pure or Dominant zone uses the primary cosm's axioms. A **Mixed** zone uses, per axiom, the **maximum** of the two cosms: `if axiom2[key] > zoneAxioms[key] then zoneAxioms[key] = axiom2[key]`. "Has cosm" is true for the primary cosm, or for either cosm when Mixed. `[VERIFIED-CODE torgeternityscene.js getAxioms(), hasCosm()]`
- **Implication:** in a Mixed zone, *both* realities' tools work. The max rule is permissive by design. Mixed zones are where cross-genre play is easiest.

### 2.6 Disconnection and reconnection

- **While disconnected:** "You can't cause Contradictions, and you can't take actions or gain benefits from tools that violate the axioms of the realm you're in" `[VERIFIED-CODE torgeternity lang/en.json]`. You also cannot spend Possibilities, including Possibilities used to soak damage, and contradictory tools stop working, armor included. `[VERIFIED https://ulisses-us.com/?p=504]`
- **Basic-rules reconnection** (French text, my translation): at the start of a disconnected character's turn, roll a Reality skill test against **DN 14**. On success you reconnect immediately. On failure you stay disconnected. **On a roll of 1 you lose 1 rank of Reality permanently, but you reconnect.** Contradictory gear "oscillates between realities", and modern armor "temporarily disappears". `[VERIFIED-CODE torgeternity src/packs/basic-rules-fr/D_connexion_5WMfad0rl1kP9myd.yml]`
- **Core-rules additions:** the reconnection attempt is forced each round but no longer costs your action. A Mishap on the reconnection attempt may transform you, and preventing that costs **all** your Possibilities. Ords who disconnect while in another reality cannot reconnect at all. `[VERIFIED https://ulisses-us.com/?p=504]`
- **Transformation:** a character can physically change "into a denizen of the new cosm", and must then adapt skills and abilities to the new reality. `[VERIFIED https://en.wikipedia.org/wiki/Torg]`

### 2.7 World Laws and per-cosm backlash

| Cosm | World Laws | Tag |
|---|---|---|
| Living Land | **Savagery**: combat turns bloody; a Bonus Die to damage on a risky All-Out attack. **Life**: wounds heal fast and dead matter decays fast. **Wonders**: lost objects reappear unexpectedly, and collapsing stone or metal buildings stay standing as vine-covered monoliths. | `[VERIFIED-CODE basic-rules-fr Axiomes...yml]`; bonus die `[VERIFIED-SNIPPET https://gameontabletop.com/crowdfunding-68.html]` |
| Nile Empire | **Action**: pushes stakes and pacing to outrageous levels; reality-rated characters can take the better of two rolls by spending a second Possibility. **Drama**: melodrama; death isn't necessarily permanent if no body is found. **Heroism**: replaces the old Law of Morality and drives pulp powers and weird science. Weird science has **no Tech axiom**; the World Laws support it. | `[VERIFIED https://ulisses-us.com/?p=594]` |
| Tharkold | **Domination**: beings are dominant or submissive, and intimidation by dominant heroes works better. **Ferocity**: violence and brutality; subtle actions get people killed. **Pain**: when you inflict a Wound, you may recover 1 Shock. | `[VERIFIED https://ulisses-us.com/?p=594]` |
| Core Earth | Law of Hope | `[RECALLED]` |
| Aysle | honor, corruption and magic-themed laws | `[RECALLED, exact names unverified]` |
| Orrorsh | corruption, fear or terror, perseverance-themed laws | `[RECALLED, exact names unverified]` |
| Cyberpapacy, Pan-Pacifica | faith, suspicion, heretical magic (Cyberpapacy); intrigue, profit, vengeance (Pan-Pacifica) | `[RECALLED, exact names unverified]` |

**Backlash varies by cosm** `[VERIFIED https://ulisses-us.com/?p=570]`. Magic backlash deals 2 Shock in every cosm plus a cosm-specific Mishap effect: in Aysle the caster also becomes Very Vulnerable; in the Cyberpapacy it is a bonus die of Shock instead of 2 Shock; in Orrorsh there is nothing extra. Miracles and psionics deal 2 Shock on a failure and 4 on a Mishap. A Mishap on a contradictory spell also disconnects the caster. **This is a ready-made "genre-specific failure flavor" table.**

### 2.8 Steal, change, avoid (Torg → PALIMPSEST)

| Torg mechanic | PALIMPSEST verdict |
|---|---|
| Four axioms per reality, integer scale | **Steal** (canon §5.1 already does, on a 0-10 scale with VERVE in place of Social). |
| Multi-axiom items (Healing Potion T12 + M13) | **Steal.** The canon's component `dep` sets already allow several axes, and C takes the max over axes. |
| "Any axiom exceeds" contradiction test | **Steal** as the trigger; canon §5.5 grades the *result* by gap. |
| Binary 1/20 or 4/20 disconnect roll per use | **Change.** In real time, binary disconnects feel random and cut off a whole loadout. The canon's graded potency and failure table (C 1-5) is right. Keep Torg's *double-contradiction* idea through Origin tolerance (canon §5.4). |
| Mixed zone = per-axiom max | **Steal, for a special zone type only.** Canon §5.8 makes `local_genre` single-valued. Add a rare `zone.seam` whose local axioms are the per-axiom max of two genres (§11, R7). That needs a canon amendment. |
| Pure zones make contradiction and reconnection harder | **Steal as a Folio modifier:** "Pure Folio": no bleed, and every Tol is reduced by 1 (§11). |
| Law-bound perks contradict outside their cosm | **Steal:** a `law_bound: genre.X` flag on components (wuxia qinggong, Nile weird science). |
| Weird science supported by Laws rather than axioms | **Steal:** a Law can *grant* tolerance on one axis for one component class (§10). |
| Whole-character disconnection blocking Possibility spending | **Adapt** as a short, rare character state, "Unmoored" (§11, R10). |
| Reconnect at DN 14; a roll of 1 costs a permanent Reality rank | **Adapt:** permanent costs belong in meta progression or Rewrite stacks, not in-run stat loss. |
| Transformation into a cosm denizen | **Steal** as an opt-in event that changes the Erratum's Origin (§11, R8). |
| Possibilities (one spend gives an extra die) | Already covered by the canon's Fortune and Death's Door systems. Do not add a second meta-currency in-run. |

---

## 3. Genre-agnostic tabletop toolkits

### 3.1 GURPS: mana levels, random mana, Tech Levels

- **Five standard mana levels:** none, low, normal, high and very high. The GM may add in-between levels, for example "medium mana with a -3 spell casting penalty". `[VERIFIED https://www.sjgames.com/gurps/Roleplayer/Roleplayer29/Mana-pulations.html]`
- **Effects:**
  - No mana: mana-powered magic fails, but Sanctity- or psi-powered effects can still work `[VERIFIED-SNIPPET gurps.fandom.com/wiki/Mana]`.
  - Low mana: magical abilities at **-5** `[VERIFIED-SNIPPET gurps.fandom.com/wiki/Mana_Sensitive]`.
  - Normal: spellcasting requires Magery `[VERIFIED-SNIPPET gurps.fandom.com/wiki/Mana_Sensitive]`.
  - High: anyone can cast `[RECALLED]`. Mages regain FP spent on IQ/A spells at the start of their next turn `[VERIFIED-SNIPPET gurps.fandom.com/wiki/Mana]`.
  - Very high: FP returns quickly. In the 3e text **every failure is a critical failure**; this was later errata'd and became "wild mana" in 4e. `[VERIFIED-SNIPPET gurps.fandom.com/wiki/Mana]`
- **Random mana, 2d table:** 2 = none, 3-5 = low, 6-8 = normal, 9-11 = high, 12 = very high. **1d drift table:** 1-3 moves one step toward the area's normal level, 4-5 means no change, 6 moves one step away; if the area is already at normal, the direction is random. **3d seasonal table:** in fall and spring, very high 17-18, high 14-16, normal 8-13, low 5-7, none 3-4. **Turbulent mana:** each level adds +1 to the casting roll, and "a roll greater than 18 is a spectacular critical failure". A moderately turbulent area adds 1d-4, minimum 0. `[VERIFIED https://www.sjgames.com/gurps/Roleplayer/Roleplayer29/Mana-pulations.html]`
- **Tech Levels:** TL0 Stone Age, TL1 Bronze Age (3500 BCE+), TL2 Iron Age (1200 BCE+), TL3 Medieval (600+), TL4 Age of Sail (1450+), TL5 Industrial Revolution (1730+), TL6 Mechanized Age (1880+), TL7 Nuclear Age (1940+), TL8 Digital Age (1980+), TL9 Microtech (2025+), TL10 Robotic (2070+), TL11 Exotic Matter, TL12 the practical cap. `[VERIFIED-SNIPPET community TL lists via travellerrpg wiki and GURPS wiki; dates differ slightly between sources]`
- **Sanctity:** a parallel per-place scale for divine power `[RECALLED]`. This is the direct precedent for MYTHOS as a separate axis from ARCANA.

**Takeaway** `[INFERENCE]`: GURPS separates *how well magic works here* (mana level) from *what era the hardware comes from* (TL). PALIMPSEST's TECHNE is a "how well it works" axis, so the GPV should carry a separate presentation-only `era_tl` field (0-12) that drives skins, architecture and naming (§9). The drift table is an exact, cheap model for in-Folio axiom fluctuation (§11, R9).

### 3.2 Savage Worlds: Setting Rules as toggles

SWADE ships **17** official Setting Rules: Born a Hero, Conviction, Creative Combat, Dumb Luck, Dynamic Backlash, Fanatics, Fast Healing, Gritty Damage, Hard Choices, Heroes Never Die, High Adventure, More Skill Points, Multiple Languages, No Power Points, Skill Specialization, Unarmored Hero and Wound Cap. `[VERIFIED https://gitlab.com/peginc/swade/-/issues/756]` Born a Hero ignores Rank requirements for Edges at character creation. Conviction gives tokens that add a d6 to a Trait or damage roll. `[VERIFIED-SNIPPET scribd/wiki.rpg.net summaries]` The other effects, from memory `[RECALLED]`: Gritty Damage (roll on the Injury Table for each Wound); Heroes Never Die (heroes are not killed by Extras); Unarmored Hero (a bonus to Soak when unarmored); Wound Cap (one hit causes at most a capped number of Wounds); Fast Healing (more frequent natural healing); High Adventure (spend Bennies for Edges situationally); Dynamic Backlash and Creative Combat (random tables).

**Takeaway:** a genre is roughly "core plus 2 to 4 rule toggles". PALIMPSEST's **Laws** (§10) are exactly this, with typed parameters.

### 3.3 Fate: the Bronze Rule

"In Fate, you can treat pretty much anything as if it were a character." The examples: a storm with aspects like Cold and Sleet instead of Fight and Physique, a gun with an ammo stress track, scenes that "already have aspects all over the place", and magic expressed "through skills, super powers using aspects or stunts". `[VERIFIED https://fate-srd.com/fate-system-toolkit/bronze-rule]`

**Takeaway:** model Folios, rooms, Laws and bleed zones as entities with tags (aspects) that other systems query. Example: room aspect `asp.cathedral_hush` halves footstep noise. That is cleaner than special-casing.

### 3.4 Cypher System: the most exact genre toolkit

All items below are from the Cypher System Reference Document dated 2025-08-22 `[VERIFIED https://csol.montecookgames.com/license/ → CSRD docx]`.

- **Core:** "The target number is always three times the task's difficulty." Difficulty runs 1-10, and creatures have levels 1-10. A level-N creature has target number 3N, health 3N in the examples, and deals N damage.
- **Genre chapters (9):** fantasy, modern, science fiction, horror, romance, superheroes, post-apocalyptic, fairy tale and historical.
- **One class set, renamed per genre:** four types (Warrior, Adept, Explorer, Speaker) with genre-specific names, for example "Superhero/Post-Apocalyptic: hero, brick, bruiser". **Flavors** retune a type toward a genre. Foci are "purposely stripped down to basics so they have the widest possible application across multiple genres", and an ability's *source* (training, runes, cybernetics, radiation) is cosmetic.
- **Tech rating** has three steps: contemporary, advanced and fantastic.
  - Mismatch: "each step difference in tech rating between two opposed vehicles increases the effective level of the higher-rated vehicle by two steps."
  - Price: drops "by one price category if the setting tech rating is … greater than the object's tech rating". Inexpensive items stay inexpensive.
  - Starting gear must be at or below the setting's average tech rating.
- **Horror Mode:** the GM-intrusion trigger range grows from 1 to 1-2, then 1-3, and so on. **Escalation rate:** exploring a large area, +1 per intrusion rolled; exploring, +1 per 10 minutes or per intrusion rolled; combat, **+1 per round**. Players respond by spending Effort to make tasks routine so they don't roll at all, which drains their Pools faster and makes them feel vulnerable.
- **Void Rules** (science fiction): intrusions on 1-2 with no escalation. **Curse Mode:** the same as Horror Mode but escalating much more slowly.
- **Superheroes:** difficulty caps at **15** instead of 10. **Power shifts:** 5 per character, at most 3 in any one category, each one an always-on level of Effort; another shift costs about 10 XP. Posthuman variant: start with 2, and unlock another for 4 XP.

**Takeaway:** Cypher proves that a genre can own an **escalation curve** (Horror Mode) and a **ceiling change** (difficulty 15). PALIMPSEST should express both as Laws with exact numbers (§10). The tech-mismatch "+2 levels per step" rule is a clean precedent for Anachronism Edge. The canon's `E × 400` bp is the analogous, smaller real-time version.

---

## 4. Multi-genre settings

| Setting | Mechanism for mixing genres | Lesson for PALIMPSEST |
|---|---|---|
| **Rifts** (Palladium, 1990; Ultimate Edition 2005) | Ley lines energize, and rifts open to the "Megaverse". It mixes post-apocalypse, cyberpunk, SF, fantasy, horror, western and myth. Techno-Wizardry fuses magic and tech. **1 Mega-Damage = 100 S.D.C.** A knife does 1-4 S.D.C., a basic laser pistol 100-400 S.D.C.-equivalent, body armor 40-80 MDC, and gods tens of thousands. `[VERIFIED https://en.wikipedia.org/wiki/Rifts_(role-playing_game)]` | **Avoid scale cliffs.** A 100× damage scale makes one genre's gear obsolete in another. Keep one damage scale and express superiority through Anachronism Edge (≤ +24%), never raw tiers. **Steal Techno-Wizardry** as a hybrid-craft station that makes items with `dep = {arcana, techne}`. |
| **Shadowrun** (Sixth World ed. 2019) | Magic returned in the Sixth World (cycle "finishing … on December 24, 2011"). **Essence starts at 6**, implants reduce it, and "as essence fades, so does magical aptitude". `[VERIFIED https://en.wikipedia.org/wiki/Shadowrun]` | A personal-level tech-versus-magic trade-off. Pair it with Caves of Qud's split below. It belongs in doc 04 and doc 07 (implants lower an ARCANA-tolerance or aether cap). |
| **Planescape** (TSR, 1994) | **Sigil**, the City of Doors, is reachable only through portals; without a portal key you are trapped ("The Cage"). It had 15 belief factions, and belief shapes reality. `[VERIFIED https://en.wikipedia.org/wiki/Planescape]` | **The Margin hub is Sigil.** Folio exits are portals that need *keys* (the Lattice choice), and hub factions can map to genres. |
| **Caves of Qud** (1.0 on 2024-12-05; Switch on 2026-02-16) | "Post-apocalyptic science fantasy", inspired by Gamma World and D&D. Five procedurally generated Sultans per run, with history modeled as biased word of mouth. `[VERIFIED https://en.wikipedia.org/wiki/Caves_of_Qud]` **True Kin** get no mutation points on level-up, 1 free implant at creation, 2 license points for any starting implant (or +1 Toughness for none), attributes from a minimum of 12 with 38 points to spend and a cap of 24, and start at -100 reputation with the Putus Templar. `[VERIFIED https://wiki.cavesofqud.com/wiki/True_kin]` | Science fantasy works when **both tech and weirdness have an in-world explanation** (ancient Eaters, mutation). Biased procedural history fits PALIMPSEST's "overwritten manuscript" premise: Folio lore can contradict itself on purpose. |

---

## 5. Dungeon Crawler Carl

**Mechanics in the books** (from the text, via reader highlights) `[VERIFIED https://www.goodreads.com/notes/54659324-dungeon-crawler-carl/169623639-adrianna-riddle]`:

- "Crawlers who choose to enter the World Dungeon must find a staircase and descend to the next level down before the allotted time is up for that level." "Once the time has passed, the level will be reclaimed."
- "There are dozens of types of boxes, and each type of box has six quality tiers": "Bronze, then Silver, then Gold, then Platinum, then Legendary, then Celestial." Examples: Silver Adventurer Box, Gold Brawler Box, Legendary Pet Box.
- Achievements are announced as "New achievement! …" and pay out boxes. Examples: "Early Adopter", "Empty pockets", and a Bronze Adventurer Box for wearing magical gear for the first time `[VERIFIED-SNIPPET goodreads highlights]`.
- "Obtaining Patrons is crucial to your survival." "Patrons are usually organizations, not individuals." "They sponsor you by purchasing boxes for you." "Lots of favorites will always lead to patrons."
- Safe rooms: "the only place the viewer cameras can't follow you".
- Some options open only at the third floor ("I couldn't do that until I reached the third floor down"). The Renegade RPG confirms that custom crawlers choose a race and class only if they survive to floor three `[VERIFIED renegade blog]`.
- Floors have strong individual themes: the Iron Tangle (a subway system "built out of the world's subterranean railway systems") and the Hunting Grounds (6th floor, a jungle with dinosaurs and tourist hunters) `[VERIFIED-SNIPPET library catalog summaries]`.

**Adaptations, 2025-2026:**

| Item | Status | Tag |
|---|---|---|
| Book 8, *A Parade of Horribles* (Ace) | Listed for 2026-05-12, 98 chapters; Dinniman expects about 10 books in total | `[VERIFIED-SNIPPET https://www.thepopverse.com/literary-dungeon-crawler-carl-book-8-a-parade-of-horribles-early-release-date-patreon-matt-dinniman/]` |
| Peacock live-action series | Straight-to-series order 2026-06-22. Writer Chris Yost; Fuzzy Door (Seth MacFarlane) and Universal Global Television; Dinniman co-EP. No premiere date found. | `[VERIFIED https://fuzzydoor.com/news/p/dungeon-crawler-carl-tv-series-ordered-by-peacock/]` |
| Renegade tabletop RPG and *DCC: Unstoppable* (solo or 2-player co-op card-crafting deck-builder on John D. Clair's Unstoppable system) | BackerKit launch 2026-04-14. The GM plays sponsors, guides and the Dungeon AI. In Unstoppable, upgrading cards also makes the boss stronger. | `[VERIFIED renegade blog URL above]` |
| Video game | **None announced** in the sources I found. There is a promotional browser game ("Mama's Night Off") and a Multimorphic pinball machine. | `[VERIFIED-SNIPPET opencritic/games.gg; kineticist]` |

**Takeaway:** DCC makes loot boxes, achievements and sponsors *diegetic and in-run*, as the audience's influence on the crawl. That is exactly how PALIMPSEST should frame Reliquaries and Fortune Engines. They are earned and in-world (an audience of "Readers"), not a storefront. The six-tier ladder is a proven readable vocabulary. The timed floor ("reclaimed") is DCC's pressure clock and matches R3's Nightreign lesson.

---

## 6. Video-game precedents

| Game | Omni-genre mechanism (verified facts) | Steal / avoid |
|---|---|---|
| **Inscryption** (2021-10-19) | Act 1: first-person roguelike deck-builder in a cabin. Act 2: top-down pixel RPG "similar to Pokémon". Act 3: an Isaac-like grid of rooms. "The card game's fundamental rules stay the same throughout." The board goes from 3×4 to 3×5 in Act 3. Over 1 million copies sold by January 2022; GDCA Game of the Year. `[VERIFIED https://en.wikipedia.org/wiki/Inscryption]` | **Steal "presentation shifts, rules stay put".** Cap each genre's core-rule deltas (§9 `syntax_delta`). |
| **Kingdom Hearts** | Each world's "graphics resemble the art style from the originating Disney film". The party changes form to fit in (undersea creatures, Halloween monsters, savannah animals). Worlds are sealed by an invisible shell until opened. KH IV: late-2027 window shown at D23 in August 2026. `[VERIFIED https://en.wikipedia.org/wiki/Kingdom_Hearts]` | **Steal per-genre Errata cosmetic forms** (an automatic skin swap on entry, so Errata look native), but leave the *gear* foreign so Contradiction stays legible. |
| **Tiny Tina's Wonderlands** (2022-03-25) | A tabletop-RPG frame with a DM narrator who "change[s] the world on the fly". Spells replace grenades on a cooldown slot. Multiclassing. Chaos Chamber: 6 levels, 2 bosses per chamber, portals offering perks or curses. `[VERIFIED https://en.wikipedia.org/wiki/Tiny_Tina%27s_Wonderlands]` | **Steal the narrator-as-reality-editor** (the Editors) and **portal choices that show perk or curse** (the Lattice exits with their bleed modifier visible). |
| **Destiny 2: Renegades** (2025-12-02, the final D2 expansion) | Star Wars translated into native Destiny systems: the Praxic Blade sword (lightsaber) and Praxic Order (Jedi), the Barant Imperium (Empire), a Hoth-like Europa, a walker-like mech, the Tharsis Cantina hub, and the *Nightfall* superweapon (Death Star). `[VERIFIED https://en.wikipedia.org/wiki/Destiny_2:_Renegades]` | **Steal "translate into native archetypes, don't import foreign rules".** This is the Archetype × Skin principle (brief §2.1). |
| **Fortnite** | Chapters contain seasons of about 10 weeks. Crossovers ship as cosmetics plus themed modes (Infinity Gauntlet, Endgame). Chapter 3 moved to UE5 on 2021-12-05. Fortnite OG became permanent in December 2024. Crown Jam ran 2026-01-23 to 02-09. *The Uprising* promotion (1381 Peasants' Revolt) ran in 2026. `[VERIFIED https://en.wikipedia.org/wiki/Fortnite_Battle_Royale]` Chapter 6 (December 2024) opened with a Japan-inspired map. `[RECALLED]` | **Steal "season = anthology theme"** for Codex seasons (each season spotlights a genre pair). Avoid time-limited FOMO; the brief already bans it. |
| **LEGO Dimensions** | About 30 franchises. "Specific figures must be present to initially enter those levels or worlds." Hire a Hero spends Studs to borrow a missing character, and that counts against the Rule Breaker gauge. Support ended 2017-10-23. `[VERIFIED https://en.wikipedia.org/wiki/Lego_Dimensions]` | **Steal ability gating by genre verbs.** Secrets can need an out-of-genre verb (hack, pray, cast), and paying to bypass feeds a visible meter. **Avoid** physical or paid gating of genres. |
| **MultiVersus** | A WB crossover fighter: 20M players by September 2022, a USD 100M writedown, servers closed 2025-05-30, with local and training modes kept offline. `[VERIFIED https://en.wikipedia.org/wiki/MultiVersus]` | **Avoid** live-service dependency. The brief's offline-complete end-of-service mode is correct. |
| **Evoland** (2013) | Starts with 4 colors and a 2D character, then unlocks color, technology and 3D, alternating Zelda-like action with ATB battles. `[VERIFIED https://en.wikipedia.org/wiki/Evoland]` | **Steal presentation unlocks as progression** in endless Marginalia, where Folios accrete overlapping layers. |
| **ULTRAKILL** (EA from 2020-09-03) | Dante's Hell with 3 acts × 3 layers. A style meter plus time and kills give a rank; S in all three without dying gives a P rank. The 2025-02-24 "ULTRA_REVAMP" added **Encore levels with different themes per layer** (for example the Prelude frozen over). `[VERIFIED https://en.wikipedia.org/wiki/Ultrakill]` | **Steal Encore:** re-theme existing layouts with another genre's skin as a cheap content multiplier for bleed and endless mode. Steal the visible per-Folio rank. |
| **Hand of Fate** (2015-02-17) | A Dealer narrates a card tableau, and encounters switch to third-person action combat. 13 bosses: 12 court cards plus the Dealer. `[VERIFIED https://en.wikipedia.org/wiki/Hand_of_Fate_(video_game)]` | A narrator who **visibly deals** the next genre. The Lattice UI can be a hand of cards. |
| **Wizard with a Gun** (2023-10-17) | A twin-stick shooter where every gun fires magic bullets with elemental, charm, fear or knock-back effects, plus base-building in a wizard tower. 4-player co-op arrived 2024-05-13; more chaos monsters in co-op, and kills extend the time limit. `[VERIFIED https://en.wikipedia.org/wiki/Wizard_with_a_Gun]` | Proof that **magic × gun** is one coherent loop. That supports crafting spell-glyph ammo (`dep = {arcana, techne}`). |
| **Noita** | 8 main-path biomes separated by **Holy Mountains**, each with 1 full heal, 1 spell refresher, a shop (about 5 wands or 10 spells), an altar of about 3 perks with a reroll, and 2 dummies (30,000 HP each). Damaging the brickwork angers the gods and spawns Stevari in every later Mountain; 3 kills escalates further. `[VERIFIED https://noita.wiki.gg/wiki/Holy_Mountain]` Biome modifiers: base chance 10% (Mines 20% after ≥ 8 deaths, Coal Pits 15%), weighted list (Darkness 1.0, Flooded 0.75, Hot 0.6, Low Gravity 0.5, Ionized 0.2, Shielded 0.1, Gold Veins 0.01 …) with per-biome exclusions. `[VERIFIED https://noita.wiki.gg/wiki/Biome_Modifiers; https://noita.wiki.gg/wiki/Biomes]` | **Steal both exactly in structure:** a Holy-Mountain-style "Colophon" room between Folios, and a weighted bleed-modifier table with exclusions (§9 block L). |
| **Hades II** (1.0 on 2025-09-25) | Two routes of 4 regions each. Underworld: Erebus, Oceanus, Fields of Mourning, Tartarus. Surface: Ephyra, Rift of Thessaly, Mount Olympus, the Summit. Each region has a boss. `[VERIFIED https://en.wikipedia.org/wiki/Hades_II]` | Two thematic routes with distinct identities. **Steal for Volumes:** each Volume can lean toward an "axis family" (low-TECHNE versus high-TECHNE) for coherent arcs. |
| **Spelunky 2** | Procedural areas, including the lava-themed Volcana, with unlockable shortcuts. `[VERIFIED https://en.wikipedia.org/wiki/Spelunky_2]` Branching choices per world (Jungle or Volcana, Tide Pool or Temple). `[RECALLED]` | Branch choice shows *what kind of danger* is next. The Lattice preview should show genre, bleed and Law icons. |
| **WoW Timewalking** | Since 11.0.5, dungeons scale enemies to the player's level, the Chromie Time model. Turbulent Timeways gives +5% XP per dungeon stack, becoming +30% for 3 h at 4 stacks. 25 badges per completion since 11.1.7. Dragonflight was added in 12.0.7 (2026-06-16). `[VERIFIED https://warcraft.wiki.gg/wiki/Timewalking]` | Content-scales-to-player is the precedent for mixed-level co-op and for reusing old genres at any depth. |
| **Spider-Man: Across the Spider-Verse** (film) | 6 distinct animation styles. Gwen's Earth-65 is a watercolor whose palette tracks her emotions ("three dimensional mood ring"). Spider-Punk is a collage with mixed frame rates (8, 12, 6 and 24 images per second on different parts). Nueva York draws on Syd Mead. "Most other characters are predominantly animated on 2s." `[VERIFIED https://en.wikipedia.org/wiki/Spider-Man:_Across_the_Spider-Verse]` | **Steal:** foreign items and Errata keep their home genre's shader, outline and animation step rate, so contradiction is visible at a glance. Genre palettes can respond to party state (Composure). |
| **Isekai** (as a genre) | Transported or reincarnated into another world, often with **game-like systems**, "cheat skills" and knowledge transfer. One example title is "*… I'll Use My Appraisal Skill to Rise in the World*". About 14% of new 2024 anime; added to the OED in 2024. `[VERIFIED https://en.wikipedia.org/wiki/Isekai]` | The Errata premise *is* isekai turned inside out: characters fall *out* of their stories. Appraisal as an identity skill links doc 10 to the genre layer. |
| **Torchlight / Diablo** | Act tilesets and biome themes on one ruleset. `[RECALLED]` | The baseline to exceed. PALIMPSEST must change rules, not only tilesets. |

---

## 7. Genre theory: what concretely signals a genre

**From film genre theory:**

- Genres are described through "conventions, iconography, settings, narratives, characters and actors" (Barry Keith Grant). Further markers are tone, theme, mood, format and audience, along with stock characters (the femme fatale), recurring settings, and characteristic scoring, lighting and framing.
- Genres are "easier to recognize than to define".
- Hybrids form, including "ironic hybridization" (Jim Collins, citing *Back to the Future Part III* as Western × SF).
- Genres move from classic form to parody to critique.

`[VERIFIED https://en.wikipedia.org/wiki/Film_genre]`

Rick Altman's *semantic/syntactic* model (1984) splits a genre into **semantic** building blocks (iconography, settings, character types) and **syntactic** structures (the relations between them: frontier versus civilization, investigator versus conspiracy). Altman later added a pragmatic, audience-use layer. Edward Buscombe's 1970 essay made iconography central. `[RECALLED]`

**Applied to a real-time 3D game** `[INFERENCE; recognition times are design heuristics, not measured data]`:

| Channel | Layer | When the player reads it | Strength of signal | GPV block |
|---|---|---|---|---|
| Palette, grading, contrast | semantic | first frame (< 0.5 s) | very high (noir monochrome, cyberpunk neon) | D |
| Lighting model (key/fill ratio, color temperature, fog) | semantic | first frame | very high (gothic low-key) | D |
| Silhouettes: architecture and props | semantic | 1-3 s | high | D, K |
| Music: instrumentation, mode, tempo | semantic | 2-5 s | very high (twang = western, erhu = wuxia, choir = gothic) | E |
| UI skin and typography | semantic | first HUD glance | high and cheap | F |
| Enemy silhouettes and behavior | semantic + syntactic | first encounter | very high | H |
| Lexicon and dialogue register | semantic | first text or bark | high ("Buck", "Taels", "Cred") | G |
| Physics feel (gravity, air control, VERVE jumps) | syntactic | first jump | medium-high (wuxia float, space-opera low-g) | C |
| Verbs and their weights (what works best here) | syntactic | 1-3 minutes | decisive: this *confirms* the genre | I |
| Laws (rules that hold only here) | syntactic | first time one fires | decisive | B |
| Structure: layout grammar, objectives, timers | syntactic | over the whole Folio | high | K |
| Camera grammar (FOV, shake, Dutch angles, letterbox) | semantic | continuous | medium (can cause nausea; keep it subtle) | D |
| Animation step rate (on 1s or 2s) and outline | semantic | continuous | medium-high (Spider-Verse) | D |

**Rule** `[PROPOSED]`: every genre must differ from every other genre by at least **3 syntactic channels** (verbs, Laws, physics, structure, enemy behavior). A genre that fails this is a *variant skin*, the Ultrakill Encore pattern. Variant skins are fine for bleed and endless mode, but they are not shipped as genres.

---

## 8. Pattern library: 24 reusable patterns

| # | Pattern | Source | One-line rule |
|---|---|---|---|
| P1 | Reality axioms | Torg | Each place has an integer vector, and each tool has requirements on it. |
| P2 | Any-axis trigger | Torg code | A component contradicts when **any** requirement exceeds the local axiom. Severity is the max over axes. |
| P3 | Double-jeopardy | Torg case 1/4 | Being foreign to *both* yourself and the place is far worse than either alone (5% versus 20%). |
| P4 | Zone strength tiers | Torg Pure/Dominant/Mixed | Not every square metre of a Folio is equally "in genre". |
| P5 | Permissive mix | Torg Mixed = per-axiom max | Where two realities overlap, both toolsets work. |
| P6 | Law-bound abilities | Torg starred perks | Some powers depend on a *Law*, not an axiom, and fail wherever that Law is absent. |
| P7 | Laws support the impossible | Nile weird science | A Law can license a component class that the axioms would forbid. |
| P8 | Genre-specific failure | Torg backlash per cosm | The same failure has a different flavor and cost per genre. |
| P9 | Transformation | Torg; Kingdom Hearts | Staying too long, or choosing it, makes you native: Origin changes. |
| P10 | Translation | Destiny 2 Renegades | Import foreign IP by mapping onto native archetypes, never by adding foreign rules. |
| P11 | Mana level | GURPS | A single dial scales magic cost and reliability. Very high dials raise the *risk* as well as the power. |
| P12 | Drift | GURPS 1d drift table | Local axioms wander one step at a time and are pulled back toward the baseline (§11, R9). |
| P13 | Setting-rule toggles | Savage Worlds | A genre is core plus 2-4 named rule switches. |
| P14 | Bronze Rule | Fate | Folios, rooms and storms are entities with aspects that systems can query. |
| P15 | Escalation clock | Cypher Horror Mode | Risk widens by +1 step per round, per interval or per mishap; it resets at sanctuary. |
| P16 | Ceiling shift | Cypher superheroes | A genre can raise or lower the difficulty and power cap (10 → 15). |
| P17 | Mismatch bonus | Cypher tech rating | A one-step tech advantage gives a large, bounded edge (+2 levels). |
| P18 | Rename, don't rebuild | Cypher type names, Flavors | Classes, currencies and stats keep their IDs and take a genre lexicon. |
| P19 | Diegetic audience | DCC patrons and boxes | Rewards come from in-world watchers who like spectacle. |
| P20 | Pressure clock per floor | DCC "reclaimed" levels | Each Folio has a timer, and expiry is a threat, not a game over. |
| P21 | Sanctuary between worlds | Noita Holy Mountain | A fixed interstitial room: heal, refresh, shop, choose a perk, and do not desecrate it. |
| P22 | Weighted modifiers with exclusions | Noita biome modifiers | Bleed is a base chance, then a weighted pick, then a per-genre exclusion list. |
| P23 | Presentation constant, rules constant | Inscryption | Change medium and camera freely, and change at most 1-2 core numbers. |
| P24 | Foreign things look foreign | Spider-Verse | Foreign entities keep their home genre's render style and animation step rate. |

---

## 9. The Genre Parameter Vector (GPV) `[PROPOSED for doc 12]`

**Conventions:**

- **S** fields are simulation state. They are fixed-point or integer, bp = basis points, deterministic, and live in `palimpsest_sim`.
- **P** fields are presentation only. They live in the renderer, audio or UI, may use floats, and never feed back into the simulation.
- **Canon columns** (ARC, TEC, MYT, VRV, `gravity_bp`, `ambient_light`, Lucre skin, Aether skin) must be copied exactly from canon §5.2.
- Every list field references IDs, never free text.

| Block | Field | Type / range | S/P | Meaning |
|---|---|---|---|---|
| **A Identity** | `id`, `display_key`, `tier` | `genre.*`, text key, core/secret/expansion | S | canon §5.2 |
| | `family` | enum: fantasy, horror, sf, historical, crime, mythic, meta | S | Lattice affinity grouping |
| | `era_tl` | 0..=12 (GURPS TL) | P | drives skins, architecture and naming; separate from TECHNE |
| | `tone` | 4 × i8 in -5..=5: earnest↔ironic, hope↔despair, wonder↔dread, gritty↔pulp | P | narrator, LLM flavor-text prompt, music selection |
| | `tropes[]` | ≤ 12 `trope.*` IDs | P | flavor-text grammar seeds |
| **B Reality** | `axioms` | [ARC, TEC, MYT, VRV], each 0..=10 | S | canon |
| | `laws[]` | 2..=4 `law.*` records `{id, params}` | S | Savage-Worlds-style toggles (§10) |
| | `law_grants[]` | `{component_class, axis, tol_bonus 1..=3}` | S | Nile pattern: Laws license impossible tech |
| | `drift` | `{interval_ticks, toward_bp, stay_bp, away_bp, max_steps}` | S | GURPS drift (§11, R9) |
| | `backlash` | `{spell_fail, prayer_fail, gadget_fail}` → `status.*` or `dmg.*` | S | Torg per-cosm backlash |
| | `escalation` | `{start_bp, step_bp, cap_bp, triggers[]}` or none | S | Cypher Horror Mode |
| **C Physics** | `gravity_bp` | canon | S | |
| | `air_control_bp`, `fall_dmg_bp` | 5000..=20000 | S | |
| | `projectile_speed_bp`, `projectile_drop_bp` | 5000..=15000 | S | e.g. a slower "visible" projectile in Fairy Tale |
| | `fire_spread_bp`, `liquid_viscosity_bp`, `gas_dissipation_bp` | 5000..=20000 | S | Noita "Hot", "Moist" analogs |
| | `block_hardness_bp` | 5000..=20000 | S | destructibility feel (paper-like Oneiric, steel Cyberpunk) |
| | `ambient_light` | canon (0..=15) | S | stealth baseline |
| | `sound_damping_bp` | 5000..=20000 | S | noir rain masking, cathedral echo |
| | `weather_set[]` | `wx.*` with weights | S | rain, ash, snow, dust, void-snow |
| **D Visual** | `palette` | 7 OKLCH swatches: key, fill, accent, danger, loot, UI-ink, UI-paper | P | |
| | `grade` | `{lut, saturation, contrast, black_lift, temperature_K}` | P | first-frame signal |
| | `lighting` | `{key_fill_ratio, fog_density, fog_color, volumetric_bp, shadow_softness}` | P | |
| | `post` | `{grain, chroma_ab, bloom, vignette, scanline, halftone, posterize_levels, outline_px}` | P | |
| | `render_style` | enum: pbr, painterly, toon, ink, pixel_dither, xerox_collage | P | Spider-Verse rule |
| | `anim_step` | 1, 2 or 3 (on 1s, 2s or 3s) | P | foreign entities keep theirs |
| | `camera` | `{fov_deg, shake_profile, dutch_max_deg, letterbox_ratio}` | P | Dutch angle ≤ 4° and toggleable (comfort) |
| | `architecture[]`, `materials[]` | `arch.*` / `mat.*` with weights | S | the material pool is simulation state |
| | `decals[]`, `props[]` | ID lists with weights | P | |
| **E Audio** | `music` | `{bpm_min, bpm_max, mode, instruments[], layers[], stinger_set}` | P | adaptive layers: explore, alert, combat, boss |
| | `sfx_overrides[]` | archetype → sound-set map | P | a gunshot is a crossbow twang under a Fairy Tale skin |
| | `reverb`, `ambience_bed`, `silence_ratio_bp` | preset IDs, 0..=10000 | P | noir and cosmic horror use silence |
| | `voice_fx` | radio, gramophone, choir-double, glitch | P | barks |
| **F UI** | `ui_skin` | `ui.*` (frames, nine-slices, icons, reticle) | P | |
| | `fonts` | `{heading, body, numerals}` | P | must ship Latin-extended, CJK and a dyslexia fallback |
| | `damage_numbers` | style ID | P | |
| | `transition` | page-turn, ink-bleed, glitch, iris, film-burn | P | Folio-entry wipe |
| | `diegetic_ui_bp` | 0..=10000 | P | how much HUD is in-world |
| **G Language** | `lexicon` | map `lex.*` → text key (class names, stat names, currency, "hit points") | P | Cypher renaming (P18) |
| | `naming_grammar` | syllable sets plus title patterns | P | items, NPCs, places |
| | `register` | formality 0..=4, profanity 0..=2, archaism 0..=4 | P | |
| | `narrator` | voice ID | P | Editor taunts per genre |
| **H Population** | `roster[]` | `{role: swarm, brute, skirmisher, caster, sniper, controller, elite, boss; enemy_arch, skin, weight}` | S | every role must be filled |
| | `factions[]` | `fac.*` with stances | S | |
| | `npc_set[]` | vendor, quest and guest-companion skins | S | KH guest party member |
| | `stealth` | `{guard_sight_bp, guard_hearing_bp, patrol_density_bp, alarm_type}` | S | doc 09 consumes |
| | `density_bp` | 5000..=20000 | S | encounters per room |
| **I Verbs** | `verb_weights` | bp per verb: shoot, melee, cast, pray, sneak, talk, hack, craft, build, traverse | S | biases loot, rooms and objectives |
| | `damage_affinity[]` | `dmg.*` → bp | S | the native resistances of the roster |
| | `stations[]` | crafting, alchemy and Scriptorium station skins available | S | |
| | `reagents[]`, `ammo_families[]` | ID lists | S | |
| | `cover_density_bp` | 0..=10000 | S | shooter-heavy genres are high |
| **J Loot** | `base_pool_weights` | archetype → bp | S | |
| | `affix_pool_weights` | `affix.*` → bp | S | genre perk pool (brief §2.1) |
| | `manufacturers[]` | `mfr.*` | S | Borderlands-style makers per genre |
| | `lucre_skin`, `aether_skin` | canon | P | |
| | `rarity_skin` | names for the 6 tiers, colors unchanged | P | colors are global for readability |
| **K Structure** | `layout_gen` | cave, dungeon, city_block, ship_deck, forest, maze, arena, vertical_tower | S | doc 12 generators |
| | `room_grammar[]` | `room.*` with weights and min/max counts | S | |
| | `set_pieces[]` | ID list | S | |
| | `timer` | `{folio_ticks, expiry_event}` | S | DCC pressure clock (P20) |
| | `objectives[]` | exit, hunt, heist, escort, ritual, survive, investigate | S | at least 1 syntactic signature |
| | `secret_rate_bp` | 0..=10000 | S | |
| **L Bleed** | `bleed_neighbors[]` | `{genre, weight_bp}` | S | Lattice edges and bleed sources |
| | `bleed_modifiers[]` | `{id, weight, excluded_in[]}` | S | Noita structure (P22) |
| | `bleed_base_chance_bp` | default 1000 | S | per Folio |
| | `zone_profile` | `{pure_bp, dominant_bp, seam_bp}` | S | share of rooms in each zone type |
| **M Meta** | `syntax_delta` | computed count of syntactic channels differing from each other genre | S (lint) | build fails if < 3 (§7) |
| | `codex_theme`, `achievements[]` | IDs | P | DCC "New achievement!" voice |

**Example record (abridged)** for `genre.gothic_horror`, in RON-like notation:

```ron
(id: "genre.gothic_horror", tier: core, family: horror, era_tl: 5,
 tone: (earnest: 3, despair: 3, dread: 4, gritty: 2),
 axioms: (arcana: 5, techne: 3, mythos: 9, verve: 3),           // canon §5.2
 laws: [ (id: "law.dread_escalation", params: (start_bp: 0, step_bp: 250, cap_bp: 2500,
                 triggers: [combat_round_6s, explore_120s, on_fumble])),
         (id: "law.sanctified_ground", params: (radius_m: 6, undead_dmg_taken_bp: 13000)),
         (id: "law.blood_price",       params: (hp_to_aether_bp: 15000)) ],
 drift: (interval_ticks: 7200, toward_bp: 5000, stay_bp: 3333, away_bp: 1667, max_steps: 1),
 backlash: (spell_fail: "st.dread", prayer_fail: "st.feared", gadget_fail: "dmg.shock"),
 gravity_bp: 10000, ambient_light: 2, sound_damping_bp: 8000,
 render_style: painterly, anim_step: 1, camera: (fov_deg: 80, dutch_max_deg: 3, letterbox_ratio: 0),
 music: (bpm_min: 50, bpm_max: 90, mode: "aeolian", instruments: ["organ","choir","cello","harpsichord"]),
 lexicon: { "lex.hp": "txt.gothic.vitae", "lex.class.adept": "txt.gothic.occultist" },
 roster: [ (role: swarm, enemy_arch: "arch.enemy.swarm_small", skin: "skin.gothic.rats", weight: 3000), ... ],
 timer: (folio_ticks: 25200, expiry_event: "ev.the_hunt_begins"),
 bleed_neighbors: [ (genre: "genre.cosmic_horror", weight_bp: 3000), (genre: "genre.steampunk", weight_bp: 2500),
                    (genre: "genre.fairy_tale", weight_bp: 2000) ])
```

The drift weights 5000/3333/1667 are GURPS' 1d drift table (3/6, 2/6, 1/6) in basis points. 7200 ticks is 120 s at 60 Hz. 25200 ticks is 7 minutes, the brief's maximum Folio length.

---

## 10. Candidate genres: 20 with defining traits

Rows 1-16 use the canon §5.2 axiom vectors exactly. Rows 17-20 are **expansion candidates** for the paid Folio Expansions (brief §8), derived from Torg cosms by `round(x/3)` and then adjusted `[PROPOSED]`. Every Law is `[PROPOSED]` with exact parameters for doc 12 to adopt or tune. Laws marked † are Torg's own verified Laws.

| # | Genre (ARC/TEC/MYT/VRV, era TL) | Torg analog | Laws (2-3) | Syntactic signature (verbs, structure, physics) | Semantic signature (palette, render, music, type) |
|---|---|---|---|---|---|
| 1 | Mythic Fantasy (8/2/6/6, TL3) | Aysle | `law.oathbound`: accepting an NPC quest binds you; completing it gives +1 rarity step on the reward, abandoning it gives `st.cursed` for the Folio. `law.true_names`: identifying a species gives +1000 bp damage against it. | melee and cast; dungeon and castle grammar; dragon or wyrm boss | warm gold and green, painterly; horns and harp; humanist serif |
| 2 | Gothic Horror (5/3/9/3, TL5) | Orrorsh | `law.dread_escalation` (Cypher Horror Mode: +250 bp complication chance per 6-s combat round, per 120 s exploring or per fumble; cap 2500; resets at Colophon). `law.sanctified_ground`. `law.blood_price`: HP can pay aether at 1.5:1. | sneak and pray; hunted-by-stalker structure; investigate | candle-lit low key, crimson on black; organ and choir; Didone serif |
| 3 | Cosmic Horror (6/4/10/1, TL6) | none | `law.forbidden_knowledge`: each identify or read gives +5% spell potency for the Folio and costs 10 Composure. `law.non_euclidean`: 1-2 door pairs per Folio re-link every 60 s and the minimap is unreliable. `law.dread_escalation`. | investigate, flee, few fights; Composure is the main resource | teal and bile green, heavy fog; drones; typewriter face |
| 4 | Space Opera (3/10/3/7, TL11) | Destiny 2 Renegades | `law.shields`: energy shields regenerate after 3 s instead of 5. `law.zero_g_volumes`: ship decks with gravity-free rooms. `law.boarding`: breach doors spawn squads. | shoot; deck and corridor grammar; gravity 8000 | blue/orange teal-and-orange, clean PBR; brass fanfare; geometric sans |
| 5 | Cyberpunk (1/9/2/5, TL9-10) | Pan-Pacifica, Cyberpapacy | `law.netrun`: cameras, turrets and doors are hackable. `law.chrome`: +1 implant slot and -10 max aether per implant (the Shadowrun Essence pattern). `law.corporate_response`: the alarm escalates into timed security waves. | hack and sneak; vertical city blocks | magenta and cyan neon on black, rain, chromatic aberration; synth and industrial; condensed sans and mono |
| 6 | Post-Apocalypse (1/6/2/4, TL7-8 degraded) | none | `law.scarcity`: ammo drops ×0.5, salvage yield ×1.5. `law.radiation`: hot zones apply `st.irradiated`. `law.scavenge`: broken containers always yield materials. | craft, build, scavenge | rust and ochre, harsh sun, dust; percussion and twang; stencil |
| 7 | Weird West (4/4/6/7, TL5-6) | none | `law.high_noon`: a 1.5 s standoff, after which the first shot auto-crits. `law.hex_trade`: curses can be sold as currency. | revolver play, duels, open terrain | sunset amber, dust haze; whistle and slide guitar; wood-type slab |
| 8 | Hardboiled Noir (0/5/1/2, TL6) | none | `law.rain_masking`: sound_damping 6000. `law.leads`: 3 clues reveal a hidden exit. `law.no_magic`, implied by ARC 0. | talk, interrogate, sneak; investigate objective | monochrome plus one red accent, venetian-blind shadows; sax and jazz; typewriter |
| 9 | Wuxia (6/2/5/10, TL3) | none | `law.qinggong` (law-bound): wall-run and glide work only where this Law holds. `law.honor_duel`: a challenge locks 1v1. `law.inner_energy`: meditating regenerates aether at ×3. | melee and traversal; duels; gravity 8000 | ink-wash, jade and vermilion; erhu, guzheng, drums; brush calligraphy |
| 10 | Steampunk (3/7/3/6, TL5) | none | `law.weird_science` (Nile pattern): +2 TEC tolerance for the gadget class. `law.pressure`: gadgets overheat past a gauge. | craft and gadget; airships and verticality | brass and copper, gaslight bloom; brass band and accordion; Victorian display type |
| 11 | Fairy Tale (10/1/7/5, TL2-3) | none | `law.rule_of_three`: bosses have exactly 3 phases, and the third attempt at a locked riddle always succeeds. `law.bargains`: named NPC deals with a stated cost. `law.cold_iron`: iron gives +2500 bp against fey. | talk, bargain, riddle; forest-maze grammar | storybook pastels, woodcut outline; celesta and folk strings; drop-cap serif |
| 12 | Mythic Antiquity (6/2/10/8, TL1-2) | Hades | `law.favor`: a patron-god boon at each Colophon. `law.hubris`: 40 kills without an offering summons a Nemesis. `law.fate_threads`. | melee and prayer; arena trials | marble, bronze, terracotta; lyre and aulos; carved capitals |
| 13 | Oneiric, secret (9/5/8/9) | none | gravity 6000 plus gravity-flip volumes (canon). `law.dream_logic`: doors re-roll destinations. | traverse and cast | surreal, palette tracks party Composure (Spider-Verse "mood ring") |
| 14 | The Redaction, secret (2/8/8/0) | none | VERVE 0 means no crits (canon). `law.censorship`: item names are blacked out, so identification is harder. | sneak and comply; office maze | sterile fluorescent white, black redaction bars; HVAC hum |
| 15 | The Unwritten, secret (0/0/0/0) | none | no shops, no Lucre, only Editors (canon) | innate abilities only | white page, pencil linework; near silence |
| 16 | The Ur-Text, secret (5/5/5/5) | none | C = 0 and E = 0 (canon). The finale. | everything works | layered palimpsest of every genre skin (Evoland accretion) |
| 17 | Pulp Adventure (5/7/6/9, TL6) | Nile Empire (14/20/18/20) | `law.action`† (the reroll analog: once per Folio, re-roll a failed Death's Door). `law.drama`† (a downed ally with no body found returns at the next Colophon). `law.weird_science`†. | traverse, shoot, punch; set-piece chases | Technicolor and sepia maps; brass fanfare; Art Deco |
| 18 | Primeval / Lost World (0/2/8/6, TL0) | Living Land (1/7/24/6) | `law.savagery`† (all-out attacks gain bonus damage). `law.life`† (regeneration ×2; corpses decay ×4). `law.wonders`† (dropped items reappear at the next Colophon). | hunt, climb, survive; jungle and mist | steaming green, mist; drums; carved glyphs |
| 19 | Superhero (6/8/4/10, TL8-9) | Cypher superhero module | `law.power_shifts`: 3 free shifts, max 2 per category. `law.raised_ceiling`: the effective sub-attribute cap rises from 150 to 175 in-Folio (Cypher's difficulty 10 → 15). `law.collateral`: block destruction ×2. | smash, fly, rescue objectives | CMYK halftone, bold outlines; heroic orchestra; comic lettering |
| 20 | Techno-Hell (4/8/6/7, TL9) | Tharkold (12/25/4/25) | `law.domination`† (Menace checks ×1.5). `law.ferocity`† (stealth kills are noisy). `law.pain`† (each wound you inflict restores 2 stamina). | shoot and melee at speed (Ultrakill/Doom) | red and black, sparks; industrial metal; distressed grotesque |

---

## 11. Design implications for PALIMPSEST

Opinionated. "Amend" marks a recommendation that needs a change to canon; everything else fits the current canon and brief.

**Steal exactly**

- **R1. Keep canon §5.5's graded Contradiction, and reject Torg's binary disconnect for real time.** Torg disconnects on 1/20 or 4/20 per use and switches off the whole contradictory loadout until a DN 14 reconnect. At 60 Hz with automatic weapons that reads as random punishment. The canon already keeps Torg's two essential ideas: the any-axis trigger (C = max over axes) and double jeopardy (Origin tolerance stands in for "home reality").
- **R2. Add law-bound components** (Torg starred perks): a component field `law_bound: law.*`. If that Law is absent at the wielder's position, then C = 6 (Disconnected), whatever the axioms say. Use it for qinggong, High Noon duel techniques and weird-science items. This is how genre *signature verbs* stay native without inflating axioms.
- **R3. Add Law tolerance grants** (Nile weird science): `law_grants: [{component_class, axis, tol_bonus}]` adds to Tol before the cap of 6. Steampunk grants +2 TEC to gadgets.
- **R4. Use the Colophon room, which is Noita's Holy Mountain:** a fixed room between Folios with 1 full heal, a full aether refresh, a shop (5 items or 10 glyphs), an altar of 3 boons plus 1 reroll, and a Scriptorium station. Desecrating it (damaging its blocks) spawns an Editor hunter in every later Colophon for the run. After 3 hunter kills, upgrade to an elite hunter.
- **R5. Use Noita's structure for bleed modifiers:** 1000 bp base chance per Folio, then a weighted pick from `bleed_modifiers[]` filtered by `excluded_in[]`. **Newcomer protection:** bleed is disabled until the account has finished 3 runs (Noita's "Mines only after 8 deaths").
- **R6. Use Cypher Horror Mode as `law.dread_escalation`:** a complication chance that starts at 0 bp, rises by +250 bp per 6-s combat round, per 120 s of exploring or per fumble, caps at 2500 bp, and resets at a Colophon or safe room. Complications come from the genre's table: lights die, a door slams, an extra spawn. Use it in Gothic Horror, Cosmic Horror and the Redaction. Give Noir a "Void Rules" variant: a flat 500 bp with no escalation.

**Improve**

- **R7. Amend: add `zone.seam`, Torg's Mixed zone.** At most 1 per Folio, covering 1-2 rooms, with local axioms equal to the **per-axiom max** of the two genres and both genres' Laws active. Render it as stitched pages. This is the only place both toolsets work at full strength, and players will route through it on purpose. Canon §5.8 currently allows one `local_genre` per position. Extend it so `local_axiom` returns the max for seam zones.
- **R8. Add "Going Native", Torg transformation crossed with Kingdom Hearts forms.** At any Colophon an Erratum may permanently set its Origin to the current Folio's genre for the rest of the run. Origin tolerance is recomputed by canon §5.4. A cosmetic native body form is applied, but gear is not translated. Once per run. The Inkwell meta-unlocks more native forms.
- **R9. Amend, optional: axiom drift from GURPS' 1d drift table.** Every 7200 ticks (120 s), pick one non-zero axis with `rng.lattice`, then move it one step toward baseline (5000 bp), hold (3333 bp) or away (1667 bp), never more than ±1 from baseline. Show a HUD glyph. This needs canon to allow `local_axiom = genre_axiom + folio_offset[a]`. Never apply it to Ur-Text or the Unwritten.
- **R10. Add an "Unmoored" state, Torg disconnection made fair.** 3 Contradiction failures (jams, fizzles, sparks) by one character within 600 ticks apply `st.unmoored` for 300 ticks. While it lasts, Fortune-derived effects (Death's Door, Serendipity procs) are off and Strained components count as Contradicted. It clears early at any Colophon or seam zone. This gives a dramatic "you are out of your story" moment with a visible counter, and no permanent loss.
- **R11. Add a presentation-only `era_tl` (GURPS TL 0-12) separate from TECHNE.** Canon TECHNE is "how well tech works" (Mythic Fantasy TEC 2), while Torg's Tech is era-like (Aysle 14 ≈ 5/10). Skins, architecture and naming should key on `era_tl` so a TEC-2 fantasy can still have crossbows and clockwork.

**Make it legible**

- **R12. Follow Spider-Verse rendering.** Foreign items and Errata render in their *origin* genre's `render_style`, `outline_px` and `anim_step`. Disconnected (C = 6) components flicker between their skin and a torn-page placeholder at 4 Hz ("oscillates between realities"). Accessibility toggle: flicker off, static icon on.
- **R13. Every genre ships a `lexicon`, the Cypher renaming.** Classes, stats, currencies and "HP" get genre-native labels, and hovering shows the canonical term. IDs and numbers never change.
- **R14. Lattice exits are a dealt hand** (Hand of Fate, Tiny Tina portals). Each card shows the genre, the bleed-modifier icon, the Law icons and the axiom delta from the current genre (for example "TEC ▼7"), so players can plan their loadout against Contradiction.

**Guard rails**

- **R15. One damage scale.** No genre multiplies base magnitudes by more than ×1.25 (Rifts' 1:100 MDC is the cautionary tale). Superiority comes only from Anachronism Edge (at most +24%) and Laws.
- **R16. Lint `syntax_delta ≥ 3`** between every pair of shipped genres, counted over verbs, Laws, physics, structure and enemy behavior. Re-themed layouts (Ultrakill Encore) are allowed only in bleed and in endless Marginalia.
- **R17. At most 1 Law per genre may change a canon formula constant** (Inscryption's single 3×4 → 3×5 change). Everything else is additive.
- **R18. Every GPV `roster` fills all 8 roles.** Enemy behavior is a syntactic signal, and an empty role makes the genre feel thin.

**Frame and live-ops**

- **R19. Make the audience diegetic (DCC).** Reliquaries come from "Readers", Reading Circles per genre that act as DCC's organizational patrons. They pay out for spectacle achievements announced in DCC's "New achievement!" voice. Use DCC's six-tier names only as Reliquary *grade* names (Bronze → Celestial); item rarity colors stay global. Mechanics stay in doc 11 and doc 15 (R3 already fixes the economics).
- **R20. Use a Folio pressure clock, DCC's "reclaimed" levels.** The default is 25200 ticks (7 min) per GPV `timer`. On expiry comes "Reclamation": an Editor spawns every 30 s, and blocks erase from the Folio boundary inward at 1 block layer per 10 s. It is never an instant game over.
- **R21. Codex seasons are genre-pair anthologies** (Fortnite chapter themes, Destiny's Renegades translation). They have no timers, per the brief.
- **R22. Stay offline-complete** (the MultiVersus lesson). No genre, Law or Folio may require the backend.

- **R23. Psionics have no axiom of their own.** Torg's Social axiom governs psionics, and PALIMPSEST has no Social axiom. Default psionic components to `dep = {arcana}`. Psionics of eldritch origin (Cosmic Horror, Oneiric) use `dep = {arcana, mythos}`. Space Opera's aether skin is already "Psi". Torg's other Social content (money, law, institutions) goes into Laws (`law.corporate_response`, `law.leads`) and GPV `factions`, not into an axiom.

---

## 12. Open questions and unverified items

- The exact Torg Eternity World Law names for Core Earth, Aysle, Cyberpapacy, Orrorsh and Pan-Pacifica are `[RECALLED]`. Only Living Land, Nile Empire and Tharkold were verified.
- The Torg Eternity core-rules Mishap ranges for disconnection beyond "case 1 / case 4", and the full transformation procedure, were not checked against the core book. The code and basic rules are verified.
- The GURPS 4e Basic Set mana table text (p. B235) was not fetched. The low-mana -5 and the high/very-high effects come from wiki summaries.
- I found no Dungeon Crawler Carl video game announcement, but my search budget ran out. A late-2026 announcement could exist.
- Fortnite Chapter 6/7 season themes, ULTRAKILL 2026 layer releases, and Spelunky 2's branch layout are `[RECALLED]` or unverified.
- Savage Worlds setting-rule *effects* (beyond Born a Hero and Conviction) are `[RECALLED]`. The list of 17 names is verified.
- The recognition times in §7 are design heuristics and need playtest measurement: a 5-second blind genre-identification test per Folio, with a target of 80% correct.
