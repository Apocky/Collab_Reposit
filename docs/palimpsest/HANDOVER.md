# PALIMPSEST: handover (2026-10-12, local Windows session)

Supersedes `HANDOVER-2026-10-12-cloud-archived.md`, written at the end of the cloud session and kept for history. Read this file first, then `README.md`. `DECISIONS.md` overrides every chapter, and `spec/00_CANON.md` (canon-1.12) overrides chapter numbers.

## 1. What this is

PALIMPSEST is a first-person 60 Hz co-op roguelike. Players are Errata, characters who fell out of their own stories. They descend through Folios, each written in a different genre, toward the Ur-Text, while the Editors try to erase them. Platform: Steam (Windows, Steam Deck via Proton). Stack: Rust 1.99 with Bevy 0.20 over a deterministic integer simulation. Monetization: earned randomness, paid certainty. Base game USD 19.99, Folio Expansions USD 9.99, Codex premium USD 9.99.

The owner is Apocky. He wants clarification questions asked continuously, each answer recorded as a numbered row in `DECISIONS.md`, and no hard release dates.

## 2. Where things are

- Repo: `C:\Users\Apocky\source\repos\The Game`, remote `github.com/Apocky/Collab_Reposit`, branch `claude/3d-omni-genre-roguelike-se65qx`. HEAD when this was written: `6359fe5`. No pull request; do not open one unless asked.
- Spec: `docs/palimpsest/spec/00` to `21`. Chapters 00 to 18 are the design, 19 the data format, 20 the generated content catalog, 21 the generated roadmap.
- Ledger: `docs/palimpsest/DECISIONS.md`, rows 1 to 147. Row 147 supersedes row 146, which was retracted.
- Rust: `palimpsest/` with crates `palimpsest_math`, `palimpsest_data` and `palimpsest_sim`. Content is in `palimpsest/content/`, schemas in `palimpsest/schemas/`, and the validator is `palimpsest/tools/validate_content.py`.
- Audit: `docs/palimpsest/audit/` holds the wave-2 logs and outboxes, the canon-1.12 apply log, the lattice recompute, and the unowned namespace list.
- Generators: `docs/palimpsest/tools/roadmap/gen.py` builds the roadmap from `template.md` and `wps.py`. Also `tools/roadmap/lattice_min.py`, `tools/roadmap/record_wp.py`, `tools/catalog/extract_content.py`, `tools/catalog/gen_catalog.py`, and the linter `tools/spec_lint.py`.

## 3. Health checks

Run these from the repo. They all passed when this was written.

    cd "C:/Users/Apocky/source/repos/The Game/palimpsest"
    cargo fmt --all -- --check
    cargo clippy --workspace --all-targets --locked -- -D warnings
    cargo test --workspace --locked
    python tools/validate_content.py
    python tools/gen_namespaces.py --check
    cd "../docs/palimpsest"
    python tools/spec_lint.py
    python tools/roadmap/checks.py spec/21_IMPLEMENTATION_ROADMAP.md

Measured results: 562 cargo tests passing. The validator reports 0 errors across 3,592 records, which is 3,451 game records and 141 namespace records. The namespaces match canon. The spec linter is clean except the pre-existing hedge "optionally" in chapter 00. No ID is defined in more than one chapter. The roadmap checks report 0 errors.

## 4. Local traps on this machine

- Use `python`, not `python3`.
- Every Python read and write of spec text needs `encoding="utf-8"`. The default here is cp1252, which crashed the linter once.
- The default rustup host is `windows-gnu`, which has no `dlltool`. The workspace is pinned to `1.99.0-x86_64-pc-windows-msvc` by a directory override set on `palimpsest/`. Do not change `rust-toolchain.toml`.
- The working tree is CRLF through `core.autocrlf`. Git warnings about LF endings are expected.
- The console is cp1252. Print with `PYTHONIOENCODING=utf-8` or write to a file.
- Shell heredocs containing quoted text broke once. For large text files, use the file write tool.

## 5. How the work is run

- Timestamps: the owner asked for a clock stamp at the start of each reply, read with the `date` command, so time between turns is visible. Record each measured duration.
- Subagents: the owner asked for no subagents during the chapter work. Later he said to do whatever is needed to resolve blockers. One bounded three-reviewer review ran and was useful. Ask before running more.
- Commit trailer for commits made by the local session: `Co-Authored-By: Claude Haiku 5.5 <noreply@anthropic.com>`. Never put model identifiers in repository files.
- Push, then read back with `git ls-remote`. A commit is not delivered until origin shows it.
- Edit rules: anchored replacements that assert one match, then grep to confirm. Numbers are exact integers in millimetres, ticks, basis points or Bam. No floats. Hedge words are forbidden in prose.

## 6. Settled decisions that matter most

- Early Access ships all 12 core genres (DECISIONS 114). The four secret genres follow the unlock rules.
- Marginalia holds at 12 core genres under the strict lattice rules, once DECISIONS 115 is applied to the forced Ur-Text exit draw (DECISIONS 147, which retracted 146). Relaxation stays rejected (110).
- No Prologue. Onboarding is contextual tips (DECISIONS 132 and 136). The first-run achievement replaces the Prologue achievements (137).
- Steam only (DECISIONS 112 and 123). No EOS, no Epic store, no itch.io, no demo, no photo mode.
- No item durability (65). No mounts or vehicles (55). No seeds or farming (128).
- The ledge mantle reaches 2,000 mm above the feet (DECISIONS 95, canon §4.2). Doc 03 §5.9 measures it from the last grounded floor and says a jump adds no reach.
- No hard release dates (111). Roadmap milestones are gates. `T0` is a configuration value set at launch.

## 7. Open work, with the next action for each

1. **Roadmap calibration.** Sizes are in agent-days and are not calibrated. Section 3.6 of the roadmap says NOT CALIBRATED with 0 of 3 rows recorded. Next action: finish three work packages, then record each with `python tools/roadmap/record_wp.py WP-NNN START FINISH AGENT_DAYS "note"` (UTC times), then regenerate. Do not estimate elapsed time before then.
2. **Adversarial verification.** Partly done: a three-reviewer pass covered canon consistency, Prologue retirement and the lattice numbers, and its verified findings are fixed. Not done for every chapter. Next action: with the owner's go-ahead, one reviewer per chapter group, defects reported as quotes, fixes applied only after verification.
3. **Chapter headers** say "Written against canon-1.12 (adversarial check pending)". Remove that parenthetical after item 2 passes.
4. **Canon-1.12 skipped requests.** The cut applied 37 of 43 requests. Later the ghost-collision sentence and the Law-memory wording were applied. Still open: chapter 03's 1,600 mm mantle wording, which the canon now replaces with the §4.2 movement kit, and some wording in chapters 09 and 13. The full list is in `audit/canon_1_12_apply_log.txt`.
5. **Chapter 17 cross-chapter requests.** The 16 requests come from chapter 17's Interface assumptions rows, because its log does not record the request text. Each target chapter's status is unverified.
6. **Chapter 20 field semantics.** The catalog holds all 3,451 IDs from the chapter tables, with fields as table text. Per-kind field schemas are not written. Next action: each owner chapter defines its kinds, then the validator checks the fields.
7. **Rust.** The doc 02 follow-ups are done: tangent `ray_sphere`, `dda` with `max_steps`, `isqrt_u64`, the no-std simulation build, and the data crate's module names. The `Seed` category is removed from the loot code. Test count is 562.

## 8. Next steps, in order

1. Get the owner's written confirmation of the settled points in section 6, especially 114, 146 and 147, 132, and 112.
2. Finish three work packages and record them, which is the only way the roadmap gets a real calibration.
3. With the owner's go-ahead, run item 2, then clear the headers.
4. Work through item 6 chapter by chapter, starting with the chapters that have the most rows: 06, 11, 04 and 08.
5. Keep asking the owner questions continuously. Record each answer in `DECISIONS.md` as a new row. Do not overwrite existing rows.
