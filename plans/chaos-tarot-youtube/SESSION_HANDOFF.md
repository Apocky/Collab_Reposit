# Chaos Tarot YouTube — Session handoff

Updated: 2026-09-28 (early UTC) · Session: https://claude.ai/code/session_01XpsjxtSPn3BsJpX8xqoPRD

## State

- Repo `Apocky/Collab_Reposit`, branch `claude/chaos-tarot-youtube-plan-2qtrui`, tracking origin. Last pushed commit: `0f5129a` (sections 01–06 checkpoint).
- Plan dir `plans/chaos-tarot-youtube/` holds: `README.md`, `BRIEF.md`, `prompts/SOURCE_PROMPTS.md` (+3 screenshots), `sections/01…06-*.md`. Not yet on disk when this was written: `sections/07-analytics-optimization.md`, `PLAN.md`, `ROADMAP.md`, `ISSUES.md`, `QUESTIONS.md`, `DECISIONS.md`.
- Skill `.claude/skills/Apocky-plan/` (SKILL.md + references/templates.md) is committed and visible to the harness.
- Reference clones (read-only, not committed anywhere): `/home/user/the-chaos-tarot` @ c87263c (main), `/home/user/apocrypha-core` @ 34ecb65, `/home/user/chaos-tarot` (empty repo).
- A drafting workflow (run id `wf_05d7a3a0-870`, script under `~/.claude/projects/…/workflows/scripts/chaos-tarot-youtube-plan-wf_05d7a3a0-870.js`) was interrupted by a container restart after writing sections 01–06; it was resumed with `resumeFromRunId` to finish 07 → synthesize → critique → revise.

## Done this session

1. Onboarded Collab_Reposit (HALO) and The-Chaos-Tarot; model statements made.
2. Resolved "Chaos Tarot" → chaos-tarot.com + `Apocky/The-Chaos-Tarot`; resolved "Unirecall" → `anamnesis/unirecall.py` served by `apocrypha-core/scripts/recall-service.py` on loopback `127.0.0.1:19129` (unreachable from the cloud container; no desktop link).
3. Wrote the Apocky-plan skill and templates.
4. Transcribed prompts 1–7 verbatim; prompt 8 not captured.
5. Wrote BRIEF.md with tagged facts, shipped card names, assumptions A1–A8, contradictions.
6. Launched and (after restart) resumed the drafting workflow.

## Next three actions (exact)

1. When the workflow completes: read `PLAN.md`, `ROADMAP.md`, `ISSUES.md`, `QUESTIONS.md`, `DECISIONS.md`, `sections/07-*.md`; spot-check three ○ claims against their URLs; fix anything the reviser missed.
2. Update this file, then `git add plans/chaos-tarot-youtube && git commit && git push -u origin claude/chaos-tarot-youtube-plan-2qtrui`.
3. Send Apocky the batched questions from `QUESTIONS.md` with defaults. Do not start channel execution work until answered or told to proceed on defaults.

## Blocked on Apocky

- Q1 prompt 8 text · Q2 existing channel vs zero · Q3 faceless vs on-camera · Q4 Kickstarter timing · Q5 hours/week + tools budget · Q6 where tracking lives (this public repo vs private product repo vs GitHub Issues) · Q7 run Unirecall locally and paste.

## Restart commands

```
cd /home/user/Collab_Reposit && git fetch origin claude/chaos-tarot-youtube-plan-2qtrui && git checkout claude/chaos-tarot-youtube-plan-2qtrui && git pull
# reference clones (if missing):
git clone --depth 1 https://github.com/Apocky/the-chaos-tarot /home/user/the-chaos-tarot
# workflow resume (only if the run is still recoverable):
Workflow({scriptPath: "<script path above>", resumeFromRunId: "wf_05d7a3a0-870"})
```

## Degraded rails this session

- Unirecall: unreachable (cloud container, no `remote-devices` link, no peer session). Fallback used: repo docs + Drive docs. Local command for Apocky:
  `python -B C:\Users\Apocky\source\repos\anamnesis\unirecall.py "Chaos Tarot YouTube channel" --tiers l2,l34,vault --timeout 8 -n 6 --json`
- X/Twitter: mirror thread fetch blocked (HTTP 402); prompt 8 unrecoverable from here.
- vidIQ connector: `connect_incomplete`; Stripe and Scholar Gateway connectors need re-authorization in claude.ai connector settings.
- Kickstarter budget sheet: only the template structure was readable; filled values not read.
