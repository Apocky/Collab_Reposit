# ID reconciliation: magic cluster (owner doc 06)

**Namespaces:** `glyph.`, `spell.`, `school.`, `prayer.`, `ench.`, `summon.` and `patron.` (Canon §19).

**Owner chapter:** `spec/06_MAGIC_SPELLCRAFTING.md`. No other chapter was edited.

**Mapping:** `map_magic.json` is `{}`. No chapter uses a variant spelling, a synonym or an invented name in these namespaces, so there is nothing to rename.

**Inputs:**
- the output of `tools/spec_lint.py --json`;
- a scan of every chapter for these prefixes in backticks, in JSON string values and in brace forms such as `glyph.ess.{flame, frost, storm}`;
- an unbackticked grep for the same prefixes.

## Inventory

Doc 06 defines and exports 195 IDs:

| Namespace | Count |
|---|---|
| `glyph.form.` | 13 |
| `glyph.ess.` | 33 |
| `glyph.mod.` | 22 |
| `glyph.trig.` | 8 |
| `glyph.cond.` | 8 |
| `school.` | 8 |
| `spell.` | 28 |
| `summon.` | 11 |
| `summon.poly.` | 7 |
| `patron.` | 16 |
| `prayer.` | 14 |
| `ench.` | 27 |

Thirty distinct concrete IDs are also used outside doc 06. All of them match an ID doc 06 already defines, character for character.

| ID(s) | Using chapters |
|---|---|
| `ench.lightbearer` | 09 |
| `glyph.ess.appraise`, `.omen`, `.sight`, `.psychometry` | 10 |
| `glyph.ess.flame`, `.frost`, `.storm`, `.mend`, `glyph.form.aura`, `.chain` | 14 (kits; brace forms in the import row) |
| `glyph.ess.tide` | 08 |
| `glyph.ess.veil` | 09 |
| `glyph.mod.anchor`, `.empower`, `.extend`, `.heavy`, `.hushed`, `.keen`, `.reach`, `.thrift` | 11 (bound-glyph roll; the brace form matches §8.5) |
| `patron.gothic_horror` | 12 |
| `patron.mythic_antiquity` | 04 |
| `patron.redacted`, `patron.weird_west` | 18 |
| `prayer.absolution` | 10 |
| `summon.herald`, `summon.misprint` | 13 |
| `summon.phantasm` | 09 |
| `summon.poly.beast` | 04 |

**Generic references.** Other chapters also use pattern references: `spell.*` (docs 11 and 13), `summon.*` (doc 13), `glyph.mod.*` (doc 11), `patron.<genre_suffix>` (doc 00) and `patron.<suffix>` (doc 12). These are not IDs, and each resolves to a doc 06 table.

**Out-of-scope tokens.** Some tokens contain one of these prefixes as a substring but belong to other namespaces:
- `noise.spell.*` (doc 09);
- `unlock.glyph.*` (docs 14 and 15);
- `loc.glyph.*` and `loc.patron.*` (doc 18).

The doc 13 expression `spell.dmg_base_mhp` is a field access, not an ID.

## Decisions

1. **`spell.text_demolition`, `spell.text_warding`, `spell.text_silence`, `spell.text_summoning`.** The linter reported these as undefined in doc 06. They were real content, written out only in the §8.3 prose and in a shortened form in Exports (`.text_demolition`). Decision: **define**.
   - New §8.6 "Authored spell catalog". It has one row for each of the 28 `spell.` IDs, giving origin genre, glyphs, complexity (§4.4), school and use.
   - The text spells are `nova force shape` (complexity 5, alteration), `nova ward` (3, abjuration), `field hush` (3, abjuration, a null field) and `manifest summon edge` (5, conjuration, `summon.hound`).
   - Each spell passes §4.3 checks 1, 2 and 4 to 11.
   - This catalog also gives the 12 worked spells W1 to W12 a defining row. Before, they had only their Exports mention and worked examples.
2. **`prayer.supplication` and `prayer.offering`.** The linter reported both as undefined. Each was described in §16.3 and §16.5 but had no definition row, and Exports used short names. Decision: **define**.
   - A new table at the head of §16 lists all 14 prayer IDs by kind: supplication, sacrifice and petition.
   - For each prayer, the table gives activation, gate, favor, `prayer_timer` change, draws and event.
   - Supplication and petition values are those already stated in §16.3 and §16.4.
   - New `prayer.offering` rules. Offering is not a cast and resolves when its altar-menu command applies (doc 17 §10.7). It needs the Erratum within 3000 mm of an altar, which is the §16.2 shrine distance. It makes no Contradiction roll and no draw, which agrees with the exhaustive §19.2 draw table. It leaves `prayer_timer` unchanged. It is refused while `st.downed`, `st.ghost` or `st.polymorphed` (§12.3 forbids prayer while polymorphed) and allowed under `st.silenced`, because §14.2 forbids only supplication and petitions. It emits `ev.favor_changed` with cause 0.
   - New petition rule: an Unheard petition still adds 3600 to `prayer_timer`, as an Unheard supplication does.
3. **Exports.**
   - The Exports table now writes every ID in full, with the section that defines it: 195 IDs. The linter reads only full backticked IDs, so short names such as `.text_demolition` or `supplication` were left undefined.
   - `summon.` (11) and `summon.poly.` (7) are now separate rows.
   - `patron.` lists all 16 IDs instead of "one per genre suffix".
4. **Foreign definitions in doc 10.** Doc 10 restates `glyph.ess.appraise`, `glyph.ess.omen`, `glyph.ess.sight` and `glyph.ess.psychometry` in the first column of its §5.10 effect table, and `prayer.absolution` in its §7.6 table. Decision: **keep doc 06 as owner, no rename**.
   - The spelling is identical, so a map entry would be a no-op.
   - Doc 10's data agrees with doc 06 §5.3 and §16.4: the effect IDs, the omen radius `4000 + 2000 × div_tier`, one `rng.ident` draw for psychometry, and the lowest-uid uncurse.
   - Doc 06 Exports now states that doc 06 owns these IDs and doc 10 owns only the `appr.eff.*` effects.
   - The linter keeps reporting doc 10's first-cell rows until doc 10 turns them into references. That is a doc 10 edit, outside this cluster's mandate.
5. **`patron.redacted` display name.** Doc 18 §8.2, which owns `loc.` strings, renamed the patron "The Undersigned" because doc 13 §9.2's `boss.editor_v2` is "The Censor". Doc 18 asked doc 06 to update its table. Decision: §16.1 now says "The Undersigned" and adds a sentence giving the reason. The doc 18 row of Interface assumptions records this. The ID is unchanged.
6. **`summon.poly.beast`.** Doc 04 §10.2 states that the `afflict.beast` form lasts 1800 ticks unscaled. It also states that the form ignores the 2000 Bp damage-taken break, ends at expiry, on downed or on a cure, and is not dispellable. Decision: the §12.3 row and the rule below the table now say the same, so doc 06's own data matches its consumer and Canon §16.4.
7. **`summon.herald`.** Doc 13 §5.5 runs a hostile herald in `fac.wild.<local genre suffix>`. Decision: the §12.2 row now names that faction.
8. **`ai.tags` in §18.** The linter read §18's "the authored `ai.tags`" as an undefined ID in doc 13's `ai.` namespace. It is the `tags` field of the authored record's `ai` block (§4.6). Decision: reworded so it no longer looks like an ID. This was a field path, not a content ID, so no mapping is needed.
9. **Acceptance tests.** Two tests were added:
   - test 49: the authored catalog has 28 records with the listed complexities, the silence text makes a null field and the summoning text makes a hound;
   - test 50: the prayer kinds, the silence and downed gates, and 14 prayer IDs in all.

## Unresolved

None. Every ID in these namespaces that any chapter uses is defined in doc 06.

## Observations for other owners (not edited)

- **Doc 17 §10.7** still uses the old supplication draw `r1 = range(1,800,000)` and the petition gate `timer ≤ 36000`. Doc 06 uses `range(180_000)` (§16.3) and `prayer_timer <= 7200` (§16.4). This is recorded in doc 06's Interface assumptions row for doc 17.
- **Doc 06 §7.3** keys its mastery-hook table by `skill.<school>.m25` to `.m100`. Doc 04 defines these only as a pattern (`skill.<name>.m25`), so the linter counts the doc 06 first cells as foreign definitions in `skill.` (40 rows). The rows were left unchanged. If doc 04 lists the perk IDs in full, doc 06 can move the perk ID out of the first column.
- **Doc 06 still uses IDs that other namespaces leave undefined:** `appr.eff.aura`, `appr.eff.divine_identify`, `appr.eff.divine_omen` and `appr.eff.psychometry` (doc 10), `unlock.glyph.<family>` (docs 14 and 15), `item.book.primer_abjuration` and `item.book.primer_evocation` (doc 11), `comp.pools.aether` (doc 03). Their owners must define them.
