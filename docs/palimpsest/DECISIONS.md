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

## 2026-10-11 — Orchestrator decisions from research

| Decision | Source | Lands in |
|---|---|---|
| Earned Randomness, Paid Certainty | Research R3 (PEGI June 2026, Brazil ECA Digital, Belgium, Australia) | Brief §8 |
| End-of-service offline guarantee | Research R2 (Destiny 2 final update 2026-06-09) | Brief §8 |
| In-house Q32.32 fixed point, keyed RNG, JSON content | Canon implementability review | Canon §1 to §3 |
| World Laws, seam zones, Colophon, Going Native, Unmoored | Research R6 (Torg, Noita) | Canon §5.10, §5.11, §17.1 |
