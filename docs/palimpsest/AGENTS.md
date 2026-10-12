# PALIMPSEST agent boot card

Read `HANDOVER.md` first, then `README.md`. `DECISIONS.md` overrides every chapter; `spec/00_CANON.md` overrides chapter numbers. Never hand-edit `spec/21_IMPLEMENTATION_ROADMAP.md`.

## SELF (rewritten each turn by the orchestrating agent)

- Task: finish wave 2 (requests + DECISIONS 89-145 per chapter), cut canon-1.12, verify, sweep, Rust follow-ups, chapter 21 regen, chapter 20 catalog.
- State 2026-10-11 22:21 PT: UTF-8 tool fix pushed (a713cb1). Health: fmt, clippy, 558 tests, validator 27/27, namespaces, lint all green on Windows. Wave-2 round 1 running as one workflow (18 chapters); Rust doc 02 follow-ups running as a second workflow. Nothing from either is committed yet.
- Next: aggregate `audit/wave2/outbox_NN.json`, lint, gates, commit and push per set; round 2 cross-chapter requests; canon-1.12.
- Traps: `python` not `python3`; UTF-8 explicit in every Python read/write; CRLF working tree; rustup default host is gnu without dlltool, so `rustup override set 1.99.0-x86_64-pc-windows-msvc --path palimpsest`; agents must not commit or write outside their own chapter, log and outbox.
