# DECISIONS — newest first. Each: what, why, tradeoff, rollback.

## 2026-09-25 Akashic is a program repo; the code stays in apocrypha-core
- what: `Akashic/` holds the blueprint (`specs/AKASHIC.csl`), receipts, decisions, park lines,
  and (next) the orchestration script that binds the organs; training/serving code stays in
  `apocrypha-core` crates on `kernel/unified-v1`.
- why: the crates already compile, test and serve there; duplicating them makes two sources.
- tradeoff: two repos per rung (code commit + receipt commit). Rollback: move the crates here.

## 2026-09-25 apx-nn is optimised in dev builds (`[profile.dev.package.apx-nn] opt-level = 3`)
- why: the trainer's end-to-end test went from ~42 s to 3 s; gradcheck numbers are unchanged
  (same code, same f32 arithmetic order). Rollback: delete the profile block.

## 2026-09-25 Presets l0 / l1 live in apx-train, vocab always from the BPE
- why: a preset that carries a vocab size is a second source for something the tokenizer owns.
