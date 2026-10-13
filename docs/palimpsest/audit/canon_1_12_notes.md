# PALIMPSEST Canon-1.12 Notes

Cut: 2026-10-12.
Scope: Wave-2 synchronization across chapters 01–21, applying product owner DECISIONS.md rows 89–148.

## Canon Amendments

1. **Section 1.3 Shifts**: Clarified that xoshiro256++ bit-mixing shift `s[1] << 17` wraps (`wrapping_shl`) without panic.
2. **Section 4.2 Movement Kit**: Normative definition of walk, sprint, crouch, jump, air jump, dodge, lean, 2,000 mm ledge mantle (from `ledge_base_y`), sprint-slide, and grappling tool as an item (DECISIONS 95).
3. **Section 5.8 Fused Folios**: Standardized Second Edition fused-genre pairs with averaged local axioms `floor_div(a + b, 2)`, shared rosters, laws, and native status (DECISIONS 101, 117).
4. **Section 5.12 Time of Day and Weather**: Normative climate table (`tod.*` and `wx.*`) governing light, sound, surface wetness, fire spread, and stealth columns (DECISIONS 99).
5. **Section 9.1 & 10.8 Lethal Resolution & Last Stand**:
   - Errata limbs, tails, and wings sever only from a crippled pool (DECISIONS 84).
   - Step 2a added: solo play last stand 300-tick input window restoring 1,000 mHP on success (DECISIONS 131, 134, 135).
6. **Section 11.6 & 17.1 Erasure and Boss Rules**:
   - Boss-Folio Erasure clock starts from `clock_start` (Folio entry or Editor death).
   - In Second Edition Final Folio, Erasure pause lasts until the Author (`boss.author`) dies (DECISIONS 103).
   - Daily baseline runs offer 52 base glyphs in place of account Glyph Library (DECISIONS 33).
7. **Section 13.1 & 17.7 Retirements**:
   - Retired `cat.seed`, `block.crop`, `mat.tag.tillable` (DECISIONS 128).
   - Retired `mode.prologue`, `enemy.prologue_hunter` (DECISIONS 132).
   - Retired `noise.mount.` (DECISIONS 55).
8. **Section 13.8 Second Edition Depth**: Defined depth as `25 + 2n` for Folio `n` (DECISIONS 106).
9. **Section 16.5 & 20 Author / Patron Designation**: Patrons designated as "author of its genre" in lower case, reserving capitalized "the Author" for final boss `boss.author` (DECISIONS 103).
10. **Section 21 Tests**: Added acceptance tests 48 (wrapping shift), 49 (solo last stand window), and 50 (Second Edition depth scaling).

## Product Owner Decisions Settled
- Row 146: Second Edition offered at every Ur-Text victory, first clear included.
- Row 147: `block.cover.sandbag` blast resistance override set to 3500.
- Row 148: `recipe.bench.staff` governed by Inscription 10.
