# Akashic

A language model built from scratch for one machine, by the organs that will use it.

Akashic is not a training script. It is the living loop in which Apocrypha's organs — UniRecall
(memory), the APX Swarm (generator / critic / arbiter), Jev / System One (fast typed judgment),
JEPA (surprise on non-text streams) — assemble the corpus, curate it, train the model, judge it,
serve it through the same room door people already talk to, and keep interpreting everything that
flows through the system while it runs.

The blueprint is `specs/AKASHIC.csl`. It is the authority for this repository; English here is a
doorway, not a second source.

Code lives where it already runs: `apocrypha-core` (crates `apx-nn`, `apx-train`, `apx-agents`,
`apx-memory`, `apx-serve`, `apx-room`, `apx-gpu`) on branch `kernel/unified-v1`. Akashic holds the
program, the receipts, the decisions, and the orchestration that binds the organs into one loop.

## Read next

- `specs/AKASHIC.csl` — mandate, organs, hardware, corpus, research ladder, rungs, bars, the live loop
- `DECISIONS.md` — every choice with its tradeoff and rollback
- `UNFINISHED_WORK.md` — where the last session stopped and the exact next step
- `receipts/` — one file per rung; line 1 is the verdict
