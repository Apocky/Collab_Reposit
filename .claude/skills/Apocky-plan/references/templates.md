# Apocky-plan artifact templates

Copy the skeleton, keep the headings, fill every field. Empty fields are
written as `—` with a reason, never deleted.

---

## BRIEF.md

```markdown
# <Project> — Brief
Frame: <one sentence: what is being planned, for whom, by when>
Repo: <owner/repo> @ <branch> · Plan dir: <path> · Date: <YYYY-MM-DD>

## 1. What it is (tagged facts)
- ✓ <fact> [source]
- ○ <claim from a doc> [doc title, date]
- ◐ <inference> ← <premises>

## 2. Brand voice (quotes)
> "<verbatim copy>" — <source>

## 3. Assets that exist
| Asset | Where | Tag |

## 4. Business context
Pricing · launches · campaigns · dates · sibling products

## 5. Audience research on file
<summaries with doc titles and dates>

## 6. Constraints
Time · money · tools · people · policy

## 7. Assumptions (⊘ until confirmed)
A1 ⊘ <assumption> — if wrong: <what changes>

## 8. Contradictions between sources
| Claim A | Claim B | Resolution or open |

## 9. Open questions (user-only)
Q1 <question> — default if unanswered: <default>
```

---

## PLAN.md

```markdown
# <Project> — Plan
Status: draft | reviewed | approved · Owner: <name> · Date

## Objective (one paragraph, measurable)
## Positioning
## Pillars / workstreams
## Systems (how the work runs week to week)
## Success metrics (each with a source of truth)
## Risks (each with a mitigation and an early signal)
## Countercase and falsifier
Strongest objection · failure mode if wrong · weakest premise · what would change the recommendation · confidence
## CSL annex
```

---

## ROADMAP.md

```markdown
# <Project> — Roadmap
Horizon: <N days/weeks> · Start: <date> · Cadence: <weekly/…>

## Phase 0 — <name> (<dates>)
Goal · Deliverables · Exit gate (oracle): <observable> · Falsifier: <what proves the phase failed> · Rollback: <what we revert to>

## Phase 1 — …

## Week-by-week
| Week | Focus | Ship | Gate check |
```

---

## ISSUES.md

```markdown
# <Project> — Issues
Legend: status ∈ {todo, doing, blocked, review, done, dropped} · priority ∈ {P0, P1, P2, P3}
IDs: <PREFIX>-NNN, never reused.

| ID | Title | Status | Pri | Phase | Depends on | Acceptance (oracle) | Falsifier | Rollback | Provenance |
|----|-------|--------|-----|-------|------------|---------------------|-----------|----------|------------|
```

---

## DECISIONS.md

```markdown
# <Project> — Decisions
Append-only. One line per decision.

| Date | Decision | Strongest rejected alternative | Falsifier | Tag |
```

---

## SESSION_HANDOFF.md

```markdown
# <Project> — Session handoff
Updated: <ISO timestamp> · Session: <link/id>

## State
Repo · branch · last commit · what exists on disk

## Done this session
## Next three actions (exact)
## Blocked on
## Restart commands
## Degraded rails this session
```

---

## QUESTIONS.md

```markdown
# <Project> — Questions for <user>
Each question names the default assumed if it goes unanswered.

| # | Question | Why it matters | Default if unanswered |
```
