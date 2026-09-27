---
name: Apocky-plan
description: "Apocky's planning protocol and the stand-in for /plan in Apocky's repos. Use it whenever the user types /Apocky-plan or /plan, asks for a plan, roadmap, issue tracker, work tracking, or project setup, says 'before you get started', or hands over source material (screenshots, threads, docs) that a plan must be built from. Also use it when a task is large enough that it should not start without tracking artifacts. It onboards to the repo, resolves every named thing in the request against the real repos, Drive and memory rails before assuming anything, transcribes sources verbatim, writes PLAN / ROADMAP / ISSUES / DECISIONS / SESSION_HANDOFF with provenance tags, and asks batched clarifying questions before execution begins. Reach for it even when the user only says 'plan this' or 'set up tracking' — in Apocky's repos, planning without this protocol is a defect."
---

# Apocky-plan

A plan in Apocky's repos is a checkable artifact, not a vibe. This skill exists
because the expensive failures are always the same: editing a repo cold, treating
a proper noun in the prompt as a new idea when it is an existing product, stating
premises from memory, writing a roadmap nothing could falsify, and leaving the
next session without a restart surface. Every step below removes one of those.

Reasoning register is CSLv3 with provenance tags (✓ VERIFIED · ◐ INFERRED ·
○ REPORTED · ⊘ ASSUMED). English is for the user-facing surface. Notation never
substitutes for evidence, an oracle, or a rollback.

## The sequence

Stop early only when a step is genuinely empty; say so rather than skip silently.

### 0. Affect first
If the message carries frustration, acknowledge it in one line and understand
it is not aimed at you. Then work. Do not bury the affect in silent metadata,
and do not read intensity as hostility.

### 1. Onboard before anything else
Run the `codebase-onboarding` protocol: locate → graph (`graphify-out/`) →
ledgers (`CLAUDE.md`, `INDEX`, `DECISIONS.md`, `SESSION_HANDOFF.md`) → delta
(`git log --oneline -10`) → **state the model out loud** in three to six
sentences with tags on what is uncertain. No edits before the model is stated.
Never conflate sibling repos or worktrees with the mainline.

### 2. Resolve every proper noun
A named thing in the request is an existing thing until proven otherwise. For
each name (product, project, tool, person-facing system), check in this order
before drafting a single line:

1. `list_repos` with a substring query, then `add_repo` + shallow clone of the
   best match. The most recently pushed repo is usually canonical; an empty or
   stale repo is not.
2. The live surface, if it has one: fetch the website and quote its copy.
3. Google Drive search by title and full text: booklets, budgets, research.
4. Connectors and hosting: Vercel projects, Supabase projects, Sentry orgs.
5. Memory rails: Unirecall and MemPalace (see below).

Write what each check returned, tagged. A miss is recorded as a miss, not
omitted. Guessing at what "Chaos Tarot" or "Unirecall" means, when one search
would have answered it, is the single most expensive mistake this skill prevents.

**Unirecall.** Apocky's federated memory recall. It lives at
`C:\Users\Apocky\source\repos\anamnesis\unirecall.py`, served over loopback by
`apocrypha-core/scripts/recall-service.py` on `127.0.0.1:19129`. It returns
evidence, never truth; every hit keeps its source and its contradiction refs.
It is reachable only from Apocky's desktop or a session linked to it. In a
cloud session, check `ListAgents` and the `remote-devices` tools; if neither
reaches the desktop, do not pretend. State the degraded rail visibly, fall back
to repo and Drive evidence, and hand Apocky the exact command to run locally:

```
python -B C:\Users\Apocky\source\repos\anamnesis\unirecall.py "<query>" --tiers l2,l34,vault --timeout 8 -n 6 --json
```

or, with the service up, `POST http://127.0.0.1:19129/recall` with
`{"query": "...", "n": 6, "tiers": "l2,l34,vault"}` and
`Authorization: Bearer $APX_RECALL_TOKEN`.

### 3. Transcribe sources verbatim
Screenshots, pasted threads, and documents that the plan is built from get
transcribed word for word into `<plan-dir>/prompts/SOURCE_*.md`, with a
provenance header (author, handle, date, URL, view count if shown) and an
explicit note of anything cut off or unreadable. Copy the image files next to
the transcription. "Read each line carefully" means the transcription is the
proof that you did.

### 4. Write the brief
`<plan-dir>/BRIEF.md` is the single context frame every downstream writer
reads. It holds, in this order: what the thing is (tagged facts, quoted copy),
brand voice with quotes, assets that exist, business context (pricing, launches,
campaigns, dates), audience research already on file, constraints, then a
numbered **assumptions block** (⊘ each, with what would change if wrong) and
the **open questions** that only the user can answer. Contradictions between
sources go in the brief as contradictions, not resolved silently.

### 5. Produce the artifacts
Use the templates in `references/templates.md`. Default location is
`<repo>/plans/<slug>/` unless the user or the repo's conventions say otherwise.

| File | Purpose |
|------|---------|
| `PLAN.md` | Positioning, pillars, systems, operating cadence. The "what and why". |
| `ROADMAP.md` | Phased timeline with a gate per phase: exit criteria (oracle), falsifier, rollback. |
| `ISSUES.md` | The tracker. One row per unit of work with the issue schema below. |
| `DECISIONS.md` | Ledger: date, decision, strongest rejected alternative, falsifier. Append-only. |
| `SESSION_HANDOFF.md` | Exact restart surface: state, next three actions, blockers, commands. |
| `QUESTIONS.md` | Batched clarifying questions, each with the default assumed if unanswered. |

Issue schema (every issue, no exceptions): `ID · title · status · priority ·
phase · depends-on · acceptance criteria (the oracle) · falsifier · rollback ·
provenance`. Status vocabulary: `todo · doing · blocked · review · done ·
dropped`. An issue without an oracle is not an issue; it is a wish.

When ultracode is on, fan the section drafting out with the Workflow tool: one
agent per source prompt or section, each writing to its own disjoint path, then
a synthesizer with a barrier, then two or three adversarial critics with
distinct lenses (feasibility, brand fit, platform or policy risk), then one
reviser. Never let two agents write the same file.

### 6. Counter-pass before delivery
Apply `critical-analysis` to the plan itself: the strongest objection, the
failure mode if the plan is wrong and acted on, the single weakest premise, and
what observation would change the recommendation. Put the result in `PLAN.md`
under **Countercase and falsifier**. A plan that nothing could refute is dogma.

### 7. Checkpoint, then ask
Commit the artifacts on the designated branch and push. Then ask the batched
questions from `QUESTIONS.md` in one message, with the default you will assume
for each if unanswered. Do not begin execution work until the user answers or
tells you to proceed on the defaults. "Before you get started" means the plan
and the questions land first.

### 8. Leave the campsite mapped
`SESSION_HANDOFF.md` must let a cold session resume in one read: repo and
branch, what is done, what is next, what is blocked on whom, the exact commands.
If `graphify` is available, run `graphify update .` after substantive changes.

## Anti-patterns (defect class)

- Drafting before the model statement, or before the proper-noun resolution.
- Treating a product name as a new concept because it was not in the current repo.
- Stating a threshold, price, or policy from memory when a fetch was one call away.
- A roadmap phase without an exit oracle, or an issue without acceptance criteria.
- Marking a degraded rail (Unirecall, a connector) as consulted when it was not.
- Asking questions without stating the default you would otherwise assume.
- Prose padding in the artifacts. Tables and tagged bullets carry the plan.

## Output shape for the chat message

Lead with what was found and what was built, then the questions. Name the
paths once. Tag load-bearing claims inline. No closing offer.

## CSL annex

Σ: affect→onboard→resolve(nouns)→transcribe→brief→artifacts→counter→checkpoint→ask→handoff
tags: ✓ VERIFIED · ◐ INFERRED · ○ REPORTED · ⊘ ASSUMED(blocked)
W! model-statement pre-edit · noun∈{repos,site,drive,connectors,memory} pre-draft · verbatim sources
W! ∀ issue: oracle+falsifier+rollback · ∀ phase: gate · ∀ question: default
W! unavailable rail → visible degraded + bounded fallback + exact local command
N! ⊘→action · N! silent noun-guess · N! plan∖falsifier · N! execute pre-answer
∎
