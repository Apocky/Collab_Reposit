# PALIMPSEST design documentation

PALIMPSEST (working title) is a first-person, real-time, 1-to-4-player co-op roguelike dungeon crawler. Every floor, called a Folio, is written in a different fictional genre. Players are Errata, characters who fell out of their own stories, and they descend through 25 Folios toward the Ur-Text while the Editors try to erase them.

This folder is the complete design specification. It is written so that coding agents can implement it without guessing: every rule is exact, every number is an integer in a stated unit, and every content ID has one owner.

## Reading order

1. **`DESIGN_BRIEF.md`** fixes the structural decisions and the monetization stance.
2. **`DECISIONS.md`** is the ledger of product-owner answers. A row there overrides any chapter.
3. **`spec/00_CANON.md`** is the binding source of truth for numbers, formulas, IDs and namespaces. When a chapter and the canon disagree, the canon wins.
4. The chapters in `spec/` give each system in full detail. Each one lists what it imports, what it exports, its open canon gaps and its acceptance tests.

## Chapters

| Chapter | Owns |
|---|---|
| `spec/00_CANON.md` | Numeric primitives, time, RNG, world units, genres and axioms, stats, skills, damage, statuses, items, currencies, run constants, events, namespace ownership, acceptance tests |
| `spec/01_VISION_AND_PILLARS.md` | Premise, pillars, inspiration ledger, audience, success metrics |
| `spec/02_TECH_ARCHITECTURE.md` | Rust and Bevy stack, crate layout, fixed-point math, determinism, content pipeline, saves, modding, platforms |
| `spec/03_SIMULATION_CORE.md` | Tick schedule, entities, voxel world and micro-voxels, light, sound, liquids, gases, surfaces, events |
| `spec/04_ATTRIBUTES_SKILLS_BODY.md` | Attributes and sub-attributes, 36 skills, levels, talents, body plans, wounds, Origins, Callings, Kin |
| `spec/05_COMBAT_MELEE_RANGED.md` | Melee, gunplay, armor, wards, the damage pipeline in practice, weapon archetypes |
| `spec/06_MAGIC_SPELLCRAFTING.md` | Glyph grammar, spellcrafting, schools, prayer, enchantment, summons, the Glyph Library |
| `spec/07_CRAFTING_MATERIALS_BUILDING.md` | Materials, quality, stations, recipes, blocks, limited building |
| `spec/08_ALCHEMY_LIQUIDS.md` | Ingredients, brewing, potions, bombs, liquids and gases, apparatus |
| `spec/09_STEALTH_PERCEPTION.md` | Awareness, noise, light, disguises, locks, hacking, alarms |
| `spec/10_IDENTIFICATION_APPRAISAL.md` | Unknown appearances, knowledge states, curses and blessings, pact curses, appraisal |
| `spec/11_ITEMS_LOOT_ECONOMY.md` | Item records, rarity, affixes, perks, manufacturers, loot tables, shops, Fortune Engines, Reliquaries |
| `spec/12_WORLD_GENERATION_GENRES.md` | Folio generation, genres, World Laws, seams, genre bleed, Folio stories |
| `spec/13_ENEMIES_AI_FACTIONS.md` | Enemy templates, utility AI, factions, genre rosters, bosses and the five Editors |
| `spec/14_RUN_STRUCTURE_PROGRESSION.md` | Run modes, Volumes, Colophons, Grades, the Margin hub, meta progression, achievements |
| `spec/15_MONETIZATION_GACHA_LOOTBOX_PASS.md` | Inkwell, Reliquaries, the Codex pass, store, compliance |
| `spec/16_MULTIPLAYER_NETCODE_SERVICES.md` | Co-op netcode, invasions, lobbies, host migration, saves and online services |
| `spec/17_UI_UX_CONTROLS_ACCESSIBILITY.md` | HUD, menus, controls, genre UI skins, accessibility, localization |
| `spec/18_ART_AUDIO_NARRATIVE.md` | Art direction, audio, music, text narration, Echo cast, the procedural ending |
| `spec/19_DATA_SCHEMAS.md` | Content file format, JSON Schemas, validation pipeline |
| `spec/20_CONTENT_CATALOG.md` | Generated catalog of every content record |
| `spec/21_IMPLEMENTATION_ROADMAP.md` | Milestones, Early Access scope, work packages |

## Research

The `research/` dossiers are the web-verified evidence behind the design, current to 2026-10-11. They cover the roguelike lineage, the RPG and looter lineage, market and monetization law, 2026 engine technology, exact formulas from shipped games, omni-genre design, and a final gap fill.

## Code and content

The Rust workspace lives at `../../palimpsest/`. It holds the deterministic simulation core that already passes the canon's acceptance tests, the JSON content under `content/`, the JSON Schemas under `schemas/`, and the content validator under `tools/`.

## Tools

```sh
python3 tools/spec_lint.py              # sections, undefined IDs, namespace ownership, retired IDs, floats, hedges
python3 tools/spec_lint.py --chapter 13 # one chapter
python3 tools/apply_id_maps.py --dry-run
```

`tools/export_standalone.ps1` copies the workspace and these documents into a standalone Git repository on a Windows machine.

## Audit trail

`audit/` keeps the consistency audit: the ID rename maps, reconciler notes, the canon gap triage, and one fix report per chapter.
