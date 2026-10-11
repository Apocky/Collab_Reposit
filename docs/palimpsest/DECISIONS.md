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

## 2026-10-11 — Orchestrator decisions from research

| Decision | Source | Lands in |
|---|---|---|
| Earned Randomness, Paid Certainty | Research R3 (PEGI June 2026, Brazil ECA Digital, Belgium, Australia) | Brief §8 |
| End-of-service offline guarantee | Research R2 (Destiny 2 final update 2026-06-09) | Brief §8 |
| In-house Q32.32 fixed point, keyed RNG, JSON content | Canon implementability review | Canon §1 to §3 |
| World Laws, seam zones, Colophon, Going Native, Unmoored | Research R6 (Torg, Noita) | Canon §5.10, §5.11, §17.1 |
