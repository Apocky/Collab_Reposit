# DECISIONS — newest first. Each: what, why, tradeoff, rollback.

## 2026-09-25 Flagship lane and coding harness go through Vercel AI Gateway
- what: `tools/hosted-s2.py` (loopback 19135 -> gateway, model anthropic/claude-opus-5.5) and
  `tools/run-claude-code.ps1` (Claude Code on Opus 5.5 through the gateway) in apocrypha-core;
  Jev (`tools/jev.py`) rides the same key. Owner's call 2026-09-25: "best coding harness as of
  today and the latest flagship as the base url, that being opus 5.5".
- why one gateway: one key, one bill, one spend cap for S1.remote, hosted S2 and the harness;
  the gateway speaks the OpenAI dialect every caller already uses, so no caller changes.
- tradeoff: the recall evidence block leaves the machine on the hosted door (laws L4/L9); the
  door is off unless the owner sets the flag and key (L10), and every request prints a receipt.
  The recommended split stays DESKTOP_PARITY's hybrid: local Qwen for volume and memory-laden
  turns, Opus 5.5 for the hard turn and for the coding harness. Rollback: unset the flag.
- for Akashic: the Swarm's critic and arbiter can run on Opus 5.5 (apx-agents --endpoint
  127.0.0.1:19135) for corpus curation and the blind pairwise judgments; the model under
  training stays from scratch. Judging on a hosted model is allowed; training on its outputs is
  not (S5 synthetic stays grounded-only and capped).

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
