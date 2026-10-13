# PALIMPSEST — Session Handover

Written 2026-10-12 at the end of a cloud session. The next session has local file access at `C:\Users\Apocky\source\repos\The Game`. Read this file first, then `README.md` in this folder.

## 1. What the project is

PALIMPSEST (working title) is a first-person, real-time 60 Hz co-op roguelike for 1 to 4 players. Every floor, called a Folio, is written in a different fictional genre. The player characters are Errata and the antagonists are the Editors. The Margin is a walkable 3D hub, and the final Folio is the Ur-Text.

- **Platform and stack:** PC on Steam only. Rust 1.99.0 with Bevy 0.20.0.
- **Simulation:** a deterministic integer simulation in `palimpsest_sim`, with no floats in state.
- **Monetization:** "Earned Randomness, Paid Certainty". The base game is USD 19.99, Folio Expansions are USD 9.99, and the Codex premium pass is USD 9.99. There is no premium currency.

The user, Apocky, is the product owner. They want to be **asked clarification questions for every system, continuously** ("just keep them coming"). Each answer is recorded as a numbered row in `DECISIONS.md`.

## 2. Getting the files

```powershell
git clone --branch claude/3d-omni-genre-roguelike-se65qx --single-branch https://github.com/Apocky/Collab_Reposit.git "C:\Users\Apocky\source\repos\The Game"
```

The last pushed commit is `2d83dda`. Work only on branch `claude/3d-omni-genre-roguelike-se65qx`. No pull request exists, and none should be opened unless the user asks.

For a standalone repository, `docs/palimpsest/tools/export_standalone.ps1 -Name "The Game"` mirrors `palimpsest/` to the repository root and `docs/palimpsest/` to `docs/`.

## 3. Layout

| Path | Contents |
|---|---|
| `docs/palimpsest/DESIGN_BRIEF.md` | Orchestrator brief with the structural decisions |
| `docs/palimpsest/DECISIONS.md` | **Binding ledger: product-owner rows 1–145** plus orchestrator rows. It overrides everything else. |
| `docs/palimpsest/spec/00_CANON.md` | Binding source of truth, **canon-1.11**. The §23 changelog and the §21 acceptance tests 1–47 are in it. |
| `docs/palimpsest/spec/01..18_*.md` | System chapters |
| `docs/palimpsest/spec/19_DATA_SCHEMAS.md` | Content format, JSON Schema 2020-12, the validator pipeline and 25 error codes |
| `docs/palimpsest/spec/21_IMPLEMENTATION_ROADMAP.md` | **Generated** by `tools/roadmap/gen.py` from `wps.py` and `template.md`. Never hand-edit it. |
| `docs/palimpsest/research/R1..R7` | Web-verified research dossiers |
| `docs/palimpsest/audit/` | ID maps, canon gap triage, `canon_1_11_notes.md`, `fix_NN.md` (wave 1 logs), `wave2/requests_NN.json` and `wave2/log_NN.md` |
| `docs/palimpsest/tools/` | `spec_lint.py`, `apply_id_maps.py`, `export_standalone.ps1`, `roadmap/` (including `lattice_min.py`) |
| `palimpsest/` | Rust workspace: `palimpsest_math`, `palimpsest_data`, `palimpsest_sim`; `schemas/`, `content/`, `tools/validate_content.py`, `tools/gen_namespaces.py`, `CHANGELOG_canon_1_11.md` |
| `.github/workflows/palimpsest-ci.yml` | CI: fmt, clippy, tests, and a content job that runs the validator self-test and checks namespaces against the canon |

## 4. Health checks to run first

```sh
cd palimpsest
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace                        # expect 558 passing
python3 tools/validate_content.py --self-test # 27/27 cases, 25/25 codes
python3 tools/validate_content.py
python3 tools/gen_namespaces.py --check
cd ../docs/palimpsest
python3 tools/spec_lint.py                    # expect near-dry; see section 6
```

## 5. Status

**Done**
- Research, the canon (now at 1.11), and chapters 01–18, each written and reviewed.
- The ID reconcile: 13 namespace clusters, with the maps applied.
- The canon gap triage: 79 amendments, all applied in canon-1.11.
- Wave-1 chapter fixes: one fixer plus one adversarial verifier per chapter for 01–18. The doc 13 roster now has at least 12 enemy types per genre.
- The Rust slice conforms to canon-1.11, with 558 tests and an adversarial review.
- Chapter 19 with its schemas, validator and CI.
- Chapter 21 roadmap and its generator.

**In progress: wave 2.** It was stopped by the account's weekly usage limit, which resets on Oct 15 at 14:00 UTC.
- **Goal:** apply each chapter's routed cross-chapter requests (`audit/wave2/requests_NN.json`) and DECISIONS rows 89–145, then cut **canon-1.12**, then run an adversarial verify on every chapter.
- **Complete:** chapter 07 and chapter 17. Their logs are in `audit/wave2/log_07.md` and `log_17.md`.
- **Partial:** chapters 03, 09, 13, 14, 15 and 18. Some edits are on disk and committed. Read `log_NN.md` before continuing.
- **Not started:** chapters 01, 02, 04, 05, 06, 08, 10, 11, 12, 16, 19 and 21.
- **Workflow script:** the wave-2 script was in the session scratchpad, which is gone. Its prompts are summarized in section 7.

**Not started**
- **Chapter 20:** a content catalog generated from validated JSON data. Per-owner agents must extract each chapter's tables into `palimpsest/content/<owner>/<namespace>/*.json` under schemas `<owner>.<kind>`. They write their schema sections into chapter 19's `<!-- SCHEMA-FRAGMENT:NN -->` placeholders. A generator script then writes `spec/20_CONTENT_CATALOG.md`.
- Regenerate chapter 21 for decisions 109–145.
- A final lint pass that runs until it reports nothing, a completeness critic, and a README refresh.

## 6. Known open items

1. **Canon-1.12 inputs.** These need canon text:
   - Decisions 95 (movement kit: mantle up to 2,000 mm, sprint-slide, grapple as an item), 99 (time of day and weather), 101, 105–108 and 117 (Second Edition), 115 (forced Ur-Text draw exemption), 116 (Origin genre fallback), 128 (retire `cat.seed`, `block.crop`, `mat.tag.tillable`), 131 and 134–135 (last stand) and 132 (retire `mode.prologue`).
   - Doc 19's Canon gaps G1–G11.
   - The canon requests in the chapter 07 and 17 results. Chapter 07's include the destructible-block list, fused-Folio `local_genre`, and the Second Edition depth of 25 + 2n.
   - Add the newly retired IDs to `spec_lint.py` RETIRED.
2. **Rust follow-ups.** Doc 02 requires these:
   - `ray_sphere` must hit when `h == 0`.
   - The DDA takes `max_steps`. Verify this.
   - Add `isqrt_u64`.
   - `palimpsest_sim` needs `no_std` under `cfg(not(test))`.
   - Name the future `palimpsest_data` modules `loader`, `schema` and `mods`.
   - Implement every canon-1.12 change once it is cut.
3. **Leftovers to sweep for after wave 2:**
   - "Nobles" must become "Crowns" (decision 90). Doc 18's name review must exempt Crowns.
   - Prices of 24.99, 29.99 or 12.99.
   - Any mention of EOS, Epic or itch.io.
   - A demo or photo mode.
   - Seeds, the Prologue, or relationships with Margin residents.
   - "6 genres at Early Access" (decision 114 makes it all 12).
   - Calendar release dates (decision 111 removes them).
   - Achievements: about 40 curated, so doc 14 must trim from 60 (decision 141).
4. **Product questions** that chapter agents raised with defaults applied are listed in each agent's result and log. Chapter 07 raised three: `mat.role.block`, sandbag blast resistance, and the lockpick yield and staff skill. Put these to the user.
5. Doc 19 flags several cross-document conflicts:
   - Doc 11's loot-record IDs duplicate template IDs.
   - Doc 16's `content/16/schema/` path is not allowed.
   - Doc 01 shows bare arrays without the envelope.
   - Both doc 03 and doc 07 claim `block.air`.
6. The ID `enemy.role.charger` replaces cavalry (decision 89). The Erasure hunter keeps `rank.hunter` identity but uses `rank.elite` multipliers (decision 91).

## 7. How the work was run, to reproduce it

- **Wave-2 apply prompt, per chapter:** read `audit/wave2/requests_NN.json` and the DECISIONS rows that land in the chapter. For each item, mark it satisfied, edit it, propose a canon request, raise a cross-chapter request, or raise a product question with the default applied. Then lint the chapter and log progress to `audit/wave2/log_NN.md` as you go.
- **Canon prompt:** collect every canon request plus the decisions with canon impact. Amend or reject each one, add tests, bump to canon-1.12 with a §23 changelog entry, and write `audit/canon_1_12_notes.md`.
- **Verify prompt, per chapter:** check every request, decision and canon-1.12 change. Grep for leftovers, run the lint, and recompute worked examples.
- **Edit rules:**
  - Make edits with Python scripts that assert each anchor occurs exactly once, then grep to confirm each insertion.
  - Numbers are exact integers in mm, ticks, Bp or Bam.
  - No hedge words.
  - Every chapter keeps Imports, Exports, Canon gaps, Interface assumptions and Acceptance tests.
  - Each chapter edits only its own file.
- **Concurrency:** on a 4-CPU machine a workflow runs only 2 agents at once. Split the work across parallel runs and commit after every finished agent, because usage limits stop agents mid-task.
- **Commit trailer:**
  ```
  Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>
  Claude-Session: https://claude.ai/code/session_01Sdvgk9fCZZciM32inVoMrZ
  ```
  Never put model identifiers in repository files.

## 8. User preferences

- Reason in the CSLv3 register. NIL and CSLv4 are auxiliary, with references at `C:\Users\Apocky\source\repos\NIL` and `CSLv4`. These are now reachable locally.
- Acknowledge frustration before starting a task, and never treat it as aimed at the collaborator.
- Favor velocity and see the whole system. Preserve provenance, authority, counterexamples and rollback.
- Ask questions continuously, with a recommended default first. Record every answer in `DECISIONS.md`.

## 9. Suggested next steps, in order

1. Run the health checks in section 4.
2. Finish wave 2: resume the partial chapters from their logs, then apply the chapters not started.
3. Cut canon-1.12.
4. Run the wave-2 verify on all chapters.
5. Do the leftover sweep in item 3 of section 6.
6. Implement canon-1.12 and the doc 02 follow-ups in the Rust slice, and keep the gates green.
7. Regenerate chapter 21.
8. Do the data extraction and write chapter 20.
9. Run the lint until it reports nothing, run a completeness critic, refresh the README, then commit and push.
10. Keep asking the user questions throughout.
