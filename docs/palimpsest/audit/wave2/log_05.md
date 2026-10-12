# Wave-2 fixer log: chapter 05

Started 2026-10-12. Inputs: requests_05.json (19 items), DECISIONS 55, 65, 84, 88, 89, 95, 131, 134, 135, 142, 143.
Lint at start: clean (0 findings).

## Progress
- [done] Header updated to 2026-10-12, DECISIONS through row 145.
- [done] Imports: added doc 10 and DECISIONS rows list.
- [done] Stance speeds (§1.6): added sprint-slide (2000 mm/s) and grapple reeling (12000 mm/s).
- [done] HitRecord (§1.5): added spell_edge_bp and spell_school.
- [done] Action gating (§4.8a): added sprint-slide (hip fire and 1H melee only) and grapple (1H actions only).
- [done] Table 6.3 spread: added sprint-slide (15000) and grapple (25000).
- [done] Throwables (§9.3, §9.4): doc 08 bombs and thrown potion splashes confirmed as NO_CRIT area hits with ignored crit rolls and FF_AREA_BP.
- [done] resolve_hit (§10.2): steps 2.5 and 2.6 use spell potency and spell edge when spell_school is set.
- [done] Curses and pacts (§10.5a): added full hook definitions for curse.brittle, curse.blunted, curse.dulled, curse.misfire, curse.bloodprice, curse.backlash, curse.pact.*, and weapon lock.
- [done] Last stand (§12.1a): added 300-tick window with 3-input ink-strike QTE (DECISIONS 131, 134, 135).
- [done] Interface assumptions: updated doc 10, doc 03, doc 17.
- [done] Acceptance tests: added tests 57-59. Invariants annex updated.
- [done] Lint: clean (0 findings, 31,780 words).
