# ID reconciliation: character cluster (owner doc 04)

**Namespaces:** `attr.`, `sub.`, `skill.`, `talent.`, `body.`, `bp.`, `wound.`, `trait.`, `origin.`, `calling.`, `aug.`, `kin.` and `afflict.` (Canon §19).

**Owner chapter:** `spec/04_ATTRIBUTES_SKILLS_BODY.md`. No other chapter was edited.

**Mapping:** `map_character.json` is `{}`. Every chapter that uses these namespaces already spells each ID the way doc 04 or the canon does. No chapter uses a synonym, a variant spelling or an invented name, so nothing needs renaming. Every gap was an ID that doc 04 described only in prose or as a pattern, and the fix was to define it in doc 04.

**Inputs:**
- the output of `tools/spec_lint.py --json`, before and after the edits;
- a scan of every chapter for these prefixes in backticks, including placeholder forms;
- a grep for the same prefixes outside backticks, which catches JSON string values and code blocks;
- a full read of doc 04 and of Canon §6, §7, §9, §16 and §19.

## What was wrong

Before the edits, the lint reported 14 undefined IDs in doc 04 and 12 in docs 08, 09 and 13. It also flagged 62 IDs that other chapters seem to define inside these namespaces:

| Kind | IDs | Used in |
|---|---|---|
| Talents named in §7.2 only by their short name | `talent.alchemist.double_batch`, `.potent_brews`, `.transmuter`, `.field_medic`, `.iron_stomach`; `talent.gunslinger.steady_aim`; `talent.warden.resurgence`; `talent.netrunner.botnet`, `.firewall`, `.ghost_signal`, `.backdoor`, `.puppet_master`, `.remote_access`; `talent.shade.ghost_walk`, `.nightborn`, `.quiet_kill`, `.soft_step`, `.unseen_blade`, `.backstab`, `.light_fingers`; `talent.vanguard.unyielding`, `.last_bastion` | 04, 08, 09 |
| Traits named in §2.3, §2.4 and §10.3 only by their short name | `trait.origin.already_broken`, `.qinggong`, `.wired`; `trait.kin.human.versatile`; `trait.kin.mutant.mutable_flesh`; `trait.kin.beastkin.night_eyes`, `.claws`, `.tail_balance`, `.keen_nose`, `.fast_metabolism`; `trait.kin.fae.light_step` | 04, 09 |
| Mastery perks described only by the pattern `skill.<name>.mNN` | `skill.hacking.m50`, `skill.alchemy.m50`, `skill.alchemy.m100`, and doc 06's 40 school-perk rows | 04, 06, 08 |
| Body parts that Canon §9 gives only as ranges ("`bp.leg_1` to `bp.leg_6`") | `bp.leg_2`, `bp.leg_3`, `bp.limb_2`, `bp.limb_3`, `bp.arm_extra_2`, `bp.weakpoint_1` | 04, 13 |
| Echo trait ID that doc 14 authors | `trait.echo.aldric_unsworn`, in a JSON value | 14 |
| IDs that the lint reads as definitions because they sit in the first column of another chapter's table | `calling.*` ×7 (doc 05's TTK anchor table), `skill.<school>.m25..m100` ×40 (doc 06's hook table), `body.*` ×8 (doc 07's butchering table), `kin.*` ×6 (doc 13's enemy kin table), and `afflict.red_ink` (a JSON `"id"` in doc 13 §2) | 05, 06, 07, 13 |

## Decisions

1. **Added §15, the ID register, to doc 04.** Each ID that doc 04 owns gets one row, with the ID in the first cell and its exact structural data. Effects are not copied. Each row points to the section that states them, so each fact still has one source.
   - §15.1: the 8 `attr.` and 24 `sub.` IDs, with code, parent, governed skills, Canon §6.4 rows and key-primary role.
   - §15.2: the 36 `skill.` IDs (name, governing sub-attribute, group) and all 144 mastery perks `skill.<name>.m25/.m50/.m75/.m100`, four to a row, each pointing to its §5.3 entry.
   - §15.3: the 13 `calling.` IDs, with key primary, Canon majors and minors, Quick start preset and talent tree.
   - §15.4: all 170 talents (156 Calling, 14 general), with tree, tier, kind (passive, active or toggle) and prerequisite talent or skill gate. These were parsed from §7.2 and §7.3 by script, and the per-tree tier counts were checked: each tree has 4/4/3/1, and the general tree has 9/5.
   - §15.5: the 6 `kin.` IDs, with body plan, added parts, creation bonus and full trait IDs.
   - §15.6: all 72 traits. These are 16 `trait.origin.*`, 24 `trait.kin.<kin>.*` and 8 `trait.marginalia.*`, each with bearer and kind, plus the 24 variant-A Echo traits `trait.echo.<suffix>`.
   - §15.7: the 3 `afflict.` IDs and the 9 `wound.` IDs.
   - §15.8: the 8 `body.` plans, with parts, donor part and speeds, and 37 `bp.` parts. Each part row gives share by plan, hit_mult, precision, severability, mass share and §8.3 wound row.
2. **Echo traits.** Doc 14 §6.3 keeps authoring the effects, as doc 04 Canon gap 14 and doc 14 §6.3 both state. Doc 04 now defines the 24 variant-A IDs. The `_b` and `_c` variants stay a pattern, because no chapter cites a specific variant ID. Doc 14's old spelling `trait.echo_<suffix>` appears only in doc 14's own Canon-gaps history note (item 18). That note records a fix doc 14 already made, so nothing is mapped. The apply tool can't rewrite placeholder forms anyway.
3. **Adopted, not renamed: the other chapters' first-column rows.** The spelling in docs 05, 06, 07 and 13 already matches the canonical ID. Those rows hold each consumer's own data keyed by a doc-04 ID: TTK anchors, magic hook rows, butchering materials and enemy kin rules. Doc 04 now defines every one of these IDs. The lint will keep flagging those rows as foreign definitions until each consumer puts something other than the bare ID in its first column, for example "Calling", "Perk" or "Plan" labels. Doc 04 cannot change that, and it is not a naming conflict. Doc 06's hook texts were compared with doc 04 §5.3 by script. Only the wording differs (for example "chain +1 jump" against "chain forms +1 jump"), never a number or a meaning.
4. **New data, needed to make the part IDs exact.** Each item is also recorded in doc 04's Canon gaps 19–20 or in §8.2 and §8.6.
   - Added-part donors for `body.serpentine` (`bp.body_front`, as §13 already did) and `body.insectoid` (`bp.thorax`). Canon §9 names only torso, mass and chassis.
   - `bp.horn`: share 500, hit_mult 8500, no precision, never severs. Canon names it but gives no data.
   - `bp.weakpoint_1` to `bp.weakpoint_4`: hit_mult 10000, precision, never sever, with share from the doc 13 boss template. Doc 13 cites `bp.weakpoint_1..4` and `bp.weakpoint_1`.
   - An added part's mass share equals its share, taken from the donor's mass share (§8.6). Without this, a severed beastkin tail would have no item mass.
   - §8.2 tier 3 now reads `pool > 0` rather than `frac > 0`. Before, a pool too small to register in `frac` fell into neither tier 3 nor tier 4. Acceptance test 16 is unchanged.
5. **Exports** rewritten to point at §15, with corrected counts (30 native parts, 72 traits) and every added part named. Acceptance tests 51 (register completeness and counts) and 52 (tier floor and added-part mass) were added.

## Not IDs (no action)

- `origin.aether_axiom` (docs 06, 08, 11) and `origin.axiom[dep]` (doc 06) are field expressions for Canon §5.3's `origin_genre.aether_axiom` and the origin genre's axiom vector. They are not `origin.` content IDs. Owners 06, 08 and 11 may want to use the canon spelling `origin_genre.` so the lint never mistakes them for IDs.
- `sub.run_seed`, `sub.config` and similar in doc 14 §9 are fields of a Rust variable named `sub`, not sub-attributes.
- The patterns `aug.*`, `aug.prosthetic_*`, `body.*`, `bp.*`, `sub.*`, `bp.arm_extra_N`, `talent.<calling_suffix>.<name>` and `trait.echo.<suffix>_b` are placeholders that doc 04 documents.

## Lint after the edits

Doc 04 has no undefined IDs in its namespaces. Its four remaining undefined IDs belong to other owners: `mat.flesh`, `rng.check`, `run.kit.universal` and `slot.hand_extra_1`. No chapter has an undefined ID in these namespaces. The 62 foreign-definition flags in docs 05, 06, 07 and 13 remain, for the reason given in decision 3.

## Out-of-scope observations, for the owners

- Canon 1.8 retired `st.mounted` and `arch.mount.` (Canon §17.7), but doc 04 still uses them. They appear in the `spacer` Origin trait, the Handling 25 and 100 perks, the Handling "mounted and moving" XP row, the Polearm 100 "crouched or mounted" clause and the Beastbinder talent `mount_bond`. The talent ID stays defined, but its content needs a doc-04 content pass.
- Canon §16.2 (1.6) requires `echo.fenrir` (`body.quadruped`) and `echo.jormungandr` (`body.serpentine`). Doc 14's 24-Echo roster has neither, and doc 04 §13 still says no launch Echo uses a non-humanoid plan.

## Unresolved

None.
