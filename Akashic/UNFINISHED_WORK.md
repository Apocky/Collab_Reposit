# UNFINISHED_WORK — PARK lines, newest first

- 2026-09-25 L0 | handle: apocrypha-core kernel/unified-v1 (apx-nn ckpt.rs, apx-train
  tokenize/train/eval, e2e test, all green on Linux) — NOT YET COMMITTED there when this repo was
  created; commit them first. owed for the L0 receipt: the BREAK run (word-shuffled corpus, same
  600 steps; expected: held-out loss stalls near the unigram NULL 5.57). next: run it, write
  receipts/L0.md, then L1's first receipt = `apx-train tokenize` over the transcript corpus on the
  host (AP/specs/sources/transcripts).
- 2026-09-25 R1 Muon | handle: specs/AKASHIC.csl %%5 | next: optim.rs `Muon` for 2-D params with
  the orthogonality test, ablated on L1.
- 2026-09-25 kernel gate | handle: apocrypha-core crates/apx-gpu ocl.rs | next: fp16 matmul
  4096^2 sustained TFLOP/s on the A770 (host only).
