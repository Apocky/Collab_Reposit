# PALIMPSEST — Decision Ledger

As of 2026-10-11. Newest entries first. Each entry records who decided, what was decided, and where it lands in the spec.

## 2026-10-11 — Product owner answers to the per-system question round

Decided by the product owner (Apocky) in session. "Kept" means the current spec default was confirmed.

| # | System | Decision | Lands in |
|---|---|---|---|
| 1 | Monetization | Kept: earned-only randomness, premium price, fiat cosmetics and Codex, no premium currency | Brief §8, canon §14, doc 15 |
| 2 | Run length | Kept: Standard 25 Folios plus Abridged 15 Folios | Canon §17.1, doc 14 |
| 3 | Camera | Kept: first-person primary with a third-person toggle | Brief §1, docs 05, 17 |
| 4 | Time model | Kept: real-time fixed 60 Hz simulation | Canon §2 |
| 5 | Engine | Kept: Rust with Bevy 0.20 over a deterministic fixed-point simulation crate | Brief §5, doc 02 |
| 6 | Platforms | Kept: PC and Steam Deck first; consoles are later ports | Docs 02, 16 |
| 7 | HALO relationship | Kept: standalone game; HALO code untouched | Repo layout |
| 8 | Tone | Kept: wry literary manuscript voice | Docs 01, 18 |
| 9 | Attributes | Kept: 8 × 3 sub-attributes, 36 skills, 13 Callings | Canon §6, §7, §16 |
| 10 | Genre rules | Kept: graded Contradiction, inert at gap 6 | Canon §5.5 |
| 11 | Magic | Kept: glyph-grammar spellcrafting with genre-native casting | Canon §5, doc 06 |
| 12 | Meta progression | Kept: horizontal only | Brief §4, doc 14 |
| 13 | Gore | Kept: severing and blood, 16+ targets | Canon §9.1, doc 15 ratings |
| 14 | Runtime LLM | Kept: optional on-device model, cosmetic text only | Brief §5, doc 18 |
| 15 | Multiplayer | **Changed: add opt-in PvP invasions, where players invade as Editors** | Canon §17.9, docs 13, 14, 16 |
| 16 | Building | **Changed: limited building.** Only barricades, bridges, ladders, traps, placed lights and deployable cover. Terrain is otherwise fixed. The hub building plot becomes trophy and decoration slots. | Canon §4.3, §13, §17.8; docs 03, 07, 12 |
| 17 | Stealth | Kept: universal full stealth | Doc 09 |
| 18 | Identification | Kept: Shattered Pixel Dungeon style with base types visible | Canon §13.6, doc 10 |
| 19 | Loot | Kept: Borderlands-scale procedural loot | Doc 11 |
| 20 | Crafting | Kept: full hybrid of grid, assembly, processing and materials | Doc 07 |
| 21 | Alchemy | Kept: full ingredient and liquid simulation | Doc 08 |
| 22 | Enemy scaling | Kept: depth scaling with champion counters | Canon §17.3, doc 13 |
| 23 | UI | Kept: per-genre manuscript skins | Doc 17 |
| 24 | Deliverables | Spec, data and validator, **plus a Rust code vertical slice** (math, RNG, simulation core with canon test vectors passing) | `docs/palimpsest/`, `palimpsest/` workspace |

## 2026-10-11 — Product owner answers, round 2

| # | System | Decision | Lands in |
|---|---|---|---|
| 25 | Hunger | Kept: satiation from full to starving in about 40 minutes | Canon §12, doc 04 |
| 26 | Erasure | **Changed: softer.** Onset 12 minutes; one elite-strength Editor hunter per Folio at 10 stacks; no repeat spawns | Canon §11.6, §17.1, docs 13, 14 |
| 27 | Character creation | Kept: custom Errata and Echoes both | Docs 04, 14 |
| 28 | Echo cast | **Changed: mix of original and public-domain figures, with Sun Wukong, Loki, Lilith, Fenrir and Jörmungandr required.** Fenrir and Jörmungandr are playable non-humanoid bodies. | Canon §16.2, docs 04, 14, 15, 18 |
| 29 | Gunplay | Kept: arcade looter-shooter | Doc 05 |
| 30 | Melee | Kept: souls-lite depth | Doc 05 |
| 31 | Friendly fire | Kept: 50% direct, 100% area by default | Canon §17.5 |
| 32 | Invasion stakes | Kept: Redacted outcome, cosmetic-only invader rewards | Canon §17.9 |
| 33 | Glyph learning | **Changed: hybrid.** Glyphs learned in any run join a permanent account Glyph Library; each run starts with 2 chosen from it | Canon §17.1, docs 06, 14 |
| 34 | Cross-run knowledge | Kept: appearances and ingredient effects re-learned every run | Docs 08, 10 |
| 35 | Crafting access | Kept: stations in rooms and Colophons, plus field kits | Doc 07 |
| 36 | Companions | Kept: generic hirelings and tamed beasts | Doc 13 |
| 37 | Visual fidelity | **Changed: higher-resolution voxels.** 250 mm micro-voxels (4×4×4 per 1 m cell) for collision, shapes, rendering and destruction; the 1 m grid stays the simulation grid | Canon §4.4; docs 02, 03, 07, 12, 18 |
| 38 | Narrator | **Changed: text-only narration.** No voiced Narrator; narration appears as manuscript marginalia. The optional on-device model still writes flavor text | Docs 17, 18 |
| 39 | Music | Kept: original adaptive score with per-genre stems | Doc 18 |
| 40 | Launch languages | Kept: 15 locales | Doc 17 |
| 41 | Early Access scope | Kept: 6 genres at Early Access (Mythic Fantasy, Gothic Horror, Cyberpunk, Weird West, Wuxia, Steampunk), the rest by 1.0 | Docs 14, 21 |
| 42 | Pricing | **Changed: base game USD 19.99** at Early Access and 1.0. Folio Expansions 9.99 to 14.99, Codex premium 9.99 and cosmetics 1.99 to 9.99 unchanged | Brief §8, doc 15 |
| 43 | Modding | Kept: JSON data packs plus sandboxed WASM scripts, Workshop at 1.0 | Doc 02 |
| 44 | Code license | Kept: proprietary, all rights reserved | `palimpsest/Cargo.toml` |
| 45 | Editors | Kept: literary-bureaucrat Volume bosses that adapt to the local genre | Doc 13 |
| 46 | Ending | **Changed: procedural ending.** The Ur-Text ending is assembled from the run's genres, Echo or Origin, notable choices and Folio story fragments, rendered as a closing manuscript | Docs 12, 14, 18 |
| 47 | Difficulty | Kept: Errata Grades 0 to 20 plus Scribe's Mercy | Docs 14, 17 |
| 48 | Replays | Kept: full deterministic replays, shareable, with a killcam | Canon §2.4, docs 14, 16, 17 |
| 49 | Madness | Kept: deceptive hallucinations, with Mercy and accessibility options able to mark them | Docs 04, 17 |
| 50 | Co-op death | Kept: Barony-style ghosts and next-Folio resurrection | Canon §17.6 |
| 51 | Dailies | Kept: daily ranked seed plus weekly mutator run | Doc 14 |
| 52 | Achievements | Kept: standard set of 40 or more | Doc 14 |
| 53 | Future expansions | Age of Sail and Pirates, and Kaiju and Mecha are the first planned Folio Expansions | Docs 12, 15, 21 |
| 54 | Genre bleed | Kept: about 1 in 10 Folios, after an account finishes 3 runs | Doc 12 |
| 55 | Mounts | **Changed: cut.** No mounts or vehicles; mount IDs retired | Canon §17.7; docs 03, 05, 11, 12, 13, 17 |
| 56 | World Laws | **Changed: discovered in play.** Hidden from previews until first triggered for the account; then remembered by the Codex | Canon §5.8; docs 12, 17 |
| 57 | Inkwell rates | Kept: 160 Ink per pull, pity by pull 60, 75/25 featured with guarantee, spark at 100, about 20 hours per featured Echo | Doc 15 |
| 58 | Codex premium | Kept: cosmetics plus earned currencies; tier 100 grants a Quill | Doc 15 |
| 59 | Account Reliquaries | Kept: keepsakes and cosmetics, never raw power | Doc 15 |
| 60 | Fortune Engines | **Changed: add cursed capsules with pact curses.** A pact welds a real curse to a matching strong boon so the item forces a build decision; shown openly with displayed odds, still no wagering | Canon §13.6; docs 10, 11 |
| 61 | Stealth alarms | Kept: recoverable, with guards searching longer and staying wary for the rest of the Folio | Doc 09 |
| 62 | Pacifist play | Kept: tracked and rewarded through achievements, Codex marks and non-kill XP | Docs 04, 14 |
| 63 | Hacking | Kept: optional quick minigame with auto-resolve at a cost, same model as lockpicking | Doc 09 |
| 64 | Disguises | Kept: every sapient native faction in every genre | Doc 09 |
| 65 | Durability | **Changed: no item durability.** Gear never wears or breaks; placeables and destructible blocks keep block HP | Canon §4.3, §13.9; docs 05, 07, 11 |
| 66 | Inventory | Kept: 20 base slots plus bags, with a weight limit and encumbrance | Doc 11 |
| 67 | Rewrite cost | Kept: Lucre cost scaling with item power, choice of 3 offered skins | Doc 11 |
| 68 | Weapon perks | Kept: Destiny-style switchable perk columns by gear tier | Doc 11 |
| 69 | Enemy roster | **Changed: 12 or more enemy types per genre** (about 190 for 16 genres), plus elites, champions, 2 or more mini-bosses per genre and 5 Editors | Docs 13, 20 |
| 70 | Enemy AI | Kept: smart tactical utility AI tuned per genre | Doc 13 |
| 71 | Native factions | Kept: run-scoped alliances through reputation | Doc 13 |
| 72 | Folio stories | Kept: prominent story fragments with an optional Folio quest | Doc 12 |
| 73 | Crossplay | Kept: Steam P2P with Datagram Relay plus an EOS build for Epic crossplay; headless host for verification | Doc 16 |
| 74 | Matchmaking | Kept: friends plus a public lobby browser with mute, block, report and kick votes | Docs 15, 16 |
| 75 | Communication | Kept: pings, text and push-to-talk voice; voice with strangers off by default | Docs 15, 16, 17 |
| 76 | Host migration | Kept: resume from the last Folio-entry snapshot | Doc 16 |
| 77 | Hub form | Kept: walkable first-person 3D Margin hub with stations, vendors and trophies | Docs 14, 17, 18 |
| 78 | Margin cast | Kept: each resident has a multi-run arc advanced by run milestones, delivered as text | Docs 14, 18 |
| 79 | Unlock pace | Kept: about 80 hours of play unlocks every non-cosmetic option without spending money | Docs 14, 15 |
| 80 | Echo unlocks | Kept: every Echo also unlocks through a bounded Recollection trial, so the Inkwell is never the only path | Docs 14, 15 |
| 81 | Ghost XP | Kept (triage Q1): ghosts and downed members share kill XP; Dormant and Departed slots do not | Canon §8; docs 04, 14, 16 |
| 82 | Scribe's Mercy | Kept (triage Q2): Mercy never blocks achievements or Grade unlocks; only ranked leaderboard entries exclude Mercy runs | Docs 14, 17 |
| 83 | Lucre between players | Kept (triage Q3): dropped piles only, inside one run; nothing transfers between accounts | Canon §14; doc 11 |
| 84 | Errata limb loss | **Changed (triage Q5): crippled limbs only.** An Erratum's limb severs only when its pool is at or below 25% before the hit; a healthy limb never severs, however large the hit | Canon §9; docs 04, 05 |
| 85 | Co-op skill growth | Kept (triage Q6): skill XP normalized by the enemy's HP before party scaling, about 112% of solo pace in a party of 4 | Canon §7; doc 04 |
| 86 | Invader rewards | Kept (triage Q8): cosmetic only (Quires and Red Ink marks), no Codex pass XP | Canon §17.9; docs 14, 15, 16 |
| 87 | Cosmetic mods in co-op | Kept (triage Q11): presentation_hash split from content_hash; peers may differ in art, audio, VFX, narration, UI and localization | Canon §2; docs 02, 16 |
| 88 | Somnambule shots | Kept (triage Q4): after 10 m, pass destructible and non-opaque blocks only; fixed terrain stops every projectile | Canon §5.9; docs 05, 11 |
| 89 | Cavalry | Kept (orchestrator call confirmed): cavalry becomes the unmounted `enemy.role.charger`, fast gap-closers that charge and trample | Canon §17.7; doc 13 |
| 90 | Mythic Fantasy Lucre skin | **Changed (triage Q9 overruled): keep "Crowns".** It is exempt from the third-party currency-name rule | Canon §5.2, §22; docs 11, 18 |
| 91 | Erasure hunter identity | Kept (triage Q12): keeps `rank.hunter` identity, music cue and immunities, with `rank.elite` multipliers | Canon §11.6; doc 13 |
| 92 | Field of view | Kept (triage Q10): 60 to 120 degrees, default 90 | Docs 02, 17 |
| 93 | Host migration detail | **Refines 76:** hot and warm migrations continue the Folio from the live state; only a cold migration (no peer holds current state) restores the Folio-entry snapshot (doc 16 §9.3) | Docs 14, 16 |
| 94 | Gore and rating | **New: ink gore.** Wounds bleed ink and severed parts unravel into script; a settings toggle switches to red blood. Rating target PEGI 16 and ESRB M | Docs 15, 17, 18 |
| 95 | Movement kit | **New:** jump, crouch, sprint, ledge mantle up to 2,000 mm, sprint-slide, and a genre-skinned grappling tool that is an item, not part of the base kit | Canon §4; docs 03, 05, 11, 17 |
| 96 | Asynchronous traces | **New:** opt-in grammar-built marginal notes (Souls-style messages) and replayable ink-stain death echoes left in other players' Folios of the same seed or genre | Canon §3, §17; docs 14, 16, 17 |
| 97 | Character creator | **New: light presets.** 12 presets per Kin plus palette and marking colors; no slider editor | Docs 04, 17, 18 |
| 98 | Platforms | **New: PC only** through 1.0 (Windows on Steam and Epic, Steam Deck verified via Proton); no console, Switch or native macOS/Linux ports planned | Docs 02, 21 |
| 99 | Time of day and weather | **New:** each Folio rolls one time of day from its genre's table and one weather state; weather feeds light, sound, surfaces and fire | Canon §5, §12; docs 03, 09, 12, 18 |
| 100 | Post-game | **New: Second Edition NG+.** After the first Ur-Text clear the Editors rewrite the lattice: remixed genre pairings, new bosses and a second ending layer, alongside Grades and Marginalia | Docs 12, 13, 14, 18 |
| 101 | Second Edition Folios | **New:** each Second Edition Folio is written in two fused genres (both rosters, laws and skins; axioms averaged), built on the seam and bleed systems | Canon §5; docs 12, 13 |
| 102 | Second Edition carry-over | **New: the character continues.** The Errata who clear the Ur-Text continue straight into Second Edition with gear and levels, against scaled enemies | Docs 04, 11, 14 |
| 103 | Second Edition bosses | **New:** the 5 Editors return as revised versions with new phases and mechanics, plus a new final boss, the Author | Docs 13, 18 |
| 104 | Streaming | **New: audience votes, opt-in.** Twitch and YouTube viewers vote on doors, boons and a Chaos mutator; off by default; never in ranked or daily runs; streamer-safe mode hides names and notes | Docs 14, 16, 17 |
| 105 | Second Edition length | **New:** 15 Folios in 3 Volumes of fused genres; each Volume ends on a revised Editor; the Author follows Volume 3 | Docs 12, 14 |
| 106 | Second Edition scaling | **New:** Second Edition Folio n uses depth 25 + 2n in the P, T, H and S tables (canon §13.8) | Canon §13.8; docs 13, 14 |
| 107 | Second Edition party | **New:** every slot enters; ghosts return with their character as it was at death | Docs 14, 16 |
| 108 | Second Edition stakes | **New: high stakes.** Entering Second Edition is a choice offered at the Ur-Text clear. A party wipe in Second Edition forfeits the run's clear (the run is recorded as a non-clear). A Second Edition clear in which no party member died grants the Ironman reward: an exclusive cosmetic and a leaderboard mark | Docs 14, 15, 17 |
| 109 | Early Access genre count | **Changed (roadmap question): add more genres to Early Access** instead of relaxing lattice rules; EA ships at least the minimum genre count for which the strict lattice rules always offer 3 exits in every mode | Canon §17; docs 12, 21 |
| 110 | Lattice fallback rule | **Rejected:** no rule relaxation; the genre count makes the strict rules hold | Canon §17; doc 12 |
| 111 | Release dates | **Changed: no hard dates.** Milestones are gated only by their exit criteria; the roadmap keeps relative sizes in agent-days; doc 15's T0 is a configuration value set at launch | Docs 15, 21 |
| 112 | Storefronts | **Changed: Steam only.** No Epic Games Store and no itch.io (itch.io was considered and dropped); EOS crossplay is no longer needed for a second store | Docs 02, 15, 16, 21 |
| 113 | Demo | **Changed: no demo** and no public playtest build | Docs 15, 21 |
| 114 | Early Access genres | **Changed (supersedes 41): all 12 core genres at Early Access**, so the 25-Folio Standard run ships at EA; the 4 secret genres follow the canon's unlock rules | Brief §3; docs 12, 14, 21 |
| 115 | Forced Ur-Text draw | **New:** the exit draw just before the forced Ur-Text Folio is exempt from the Volume no-repeat and per-run cap rules; no player-visible exit changes (its doors are replaced by the Ur-Text door) | Canon §17; doc 12 |
| 116 | Origin genre fallback | **New:** when the host's Origin genre is not shipped, Folio 1 uses the nearest shipped genre by lattice edge weight (ties by ascending genre ID) | Docs 12, 14 |
| 117 | Second Edition lattice | **New:** the no-repeat rule applies per fused pair, not per genre: a Volume never repeats a pair, but a genre may appear in two different pairs | Docs 12, 14 |
| 118 | Early Access feature set | **New:** Second Edition NG+ ships at Early Access; asynchronous traces, streaming audience votes and PvP invasions ship in post-EA updates before 1.0 | Doc 21 |
| 119 | Early Access cast | Kept (roadmap default): all 12 Origins and 24 Echoes at Early Access, including Sun Wukong, Loki, Lilith, Fenrir and Jörmungandr | Docs 14, 15, 21 |
| 120 | On-device Scribe model | Kept (roadmap default): ships at 1.0 as an opt-in; Early Access uses the template grammar | Docs 02, 18, 21 |
| 121 | Kaiju and Mecha expansion | **New: powered armor, no vehicles.** Mechs are worn heavy armor sets and Going Native forms that change the body plan; no piloted vehicles | Docs 12, 21 |
| 122 | Expansion price | **Changed: USD 9.99** per Folio Expansion (was 9.99 to 14.99) | Brief §8; docs 15, 21 |
| 123 | Networking stack | **Changed: Steam plus our own verifier.** Steamworks lobbies and Steam Networking Sockets with Datagram Relay for play; our own headless service re-simulates ranked, daily and weekly replays to verify scores; the EOS SDK is dropped | Docs 02, 16, 21 |
| 124 | Secret-genre mini-bosses | Kept (roadmap default): the Ur-Text and the Unwritten use doc 13's champion substitution, no bespoke mini-bosses | Doc 13 |
| 125 | Invasions timing | Kept: PvP invasions ship in an update before 1.0, opt-in | Doc 21 |
| 126 | Mod distribution | **New: Steam Workshop** for data packs and sandboxed WASM mods; co-op peers auto-sync the host's required mod set | Docs 02, 16, 17 |
| 127 | Margin relationships | **New: no relationships.** Residents are vendors and quest-givers whose fixed story arcs advance across runs (item 78); no friendship meters, gifts or romance | Docs 14, 18 |
| 128 | Seeds | **Changed: cut.** `cat.seed` is retired; no planting in runs or in the Margin | Canon §13.1, §17.7; docs 07, 08, 11 |
| 129 | Photo mode | **New: none.** Replays stay, without a photo mode | Docs 16, 17 |
| 130 | Margin cast arcs | **Refines 78:** residents advance their fixed arcs through optional Folio quests they give; completing a quest unlocks the resident's next text scene | Docs 12, 14, 18 |
| 131 | Last stand | **New:** in solo play, lethal damage that Death's Door does not prevent opens a last stand: a 5-second (300-tick) timed input that, on success, leaves the Erratum at 1 HP; once per Folio; on failure the Erasure scene plays | Canon §6.4, §11; docs 04, 05, 17 |
| 132 | Prologue | **Changed: no Prologue.** `mode.prologue` is retired; the first real run teaches through contextual tips | Canon §17.7; docs 12, 13, 14, 15, 17 |
| 133 | Difficulty presets | Kept: Scribe's Mercy plus Grades 0 to 20 are the whole difficulty range; no Story preset or extra sliders | Docs 14, 17 |
| 134 | Last stand order | Kept (confirms 131): Death's Door rolls first; the last stand opens only if it fails; solo only; once per Folio | Canon §6.4; docs 04, 05 |
| 135 | Last stand input | **New: ink-strike QTE.** Three prompted inputs in order within 300 ticks; accessibility settings switch it to a single hold or automatic success | Docs 05, 17 |
| 136 | Onboarding | **New: contextual tips only.** First-encounter tips in the margins, dismissible, shown once per account; no special first-run seed | Docs 14, 17 |
| 137 | Prologue replacement | **New:** a first-run achievement for finishing Folio 1 of the account's first run replaces the Prologue achievements; anything the Prologue unlocked is available from the start | Docs 14, 15 |
| 138 | Procedural text localization | Kept: Project Fluent messages with per-locale gender, number and case selectors, so generated names inflect correctly in all 15 locales | Docs 02, 17, 18 |
| 139 | Live tuning | Kept: balance changes ship only in patches; no server-pushed tuning | Docs 02, 16 |
| 140 | Telemetry | Kept: opt-in per-run aggregate summaries, never for minor-flagged accounts, never free text | Docs 02, 16 |
| 141 | Achievements | **Changed: about 40 curated Steam achievements** (story and mastery milestones, the first-run achievement, Second Edition Ironman); the long tail moves to in-game Codex marks | Docs 14, 15 |
| 142 | Controller aiming | Kept: light slowdown and mild magnetism for sticks, none for mouse; full gyro aiming on Steam Deck and supported pads; assist off in PvP invasions | Docs 05, 17 |
| 143 | Camera | Kept: first person by default with an over-the-shoulder third-person toggle; hit detection identical in both | Docs 05, 17, 18 |
| 144 | Save profiles | Kept: 3 account profiles per Steam account, each with one suspended run, synced through Steam Cloud | Docs 02, 16 |
| 145 | Survival meters | Kept: hunger only; no thirst or fatigue | Docs 04, 08 |

## 2026-10-11 — Orchestrator decisions from research

| Decision | Source | Lands in |
|---|---|---|
| Earned Randomness, Paid Certainty | Research R3 (PEGI June 2026, Brazil ECA Digital, Belgium, Australia) | Brief §8 |
| End-of-service offline guarantee | Research R2 (Destiny 2 final update 2026-06-09) | Brief §8 |
| In-house Q32.32 fixed point, keyed RNG, JSON content | Canon implementability review | Canon §1 to §3 |
| World Laws, seam zones, Colophon, Going Native, Unmoored | Research R6 (Torg, Noita) | Canon §5.10, §5.11, §17.1 |
| Cavalry converts to an unmounted `enemy.role.charger`; mount-only IDs retire (stablemaster, remount, mounted nav) | Decision 55 (mounts cut) vs doc 13 reconciler | Canon §17.7; doc 13 |
| The Erasure hunter is one elite-strength enemy (rank.elite stats, hunter AI); no repeat spawns | Decision 26 vs doc 13 reconciler | Canon §11; docs 13, 14 |
