# 19 — Data Schemas

As of 2026-10-12. Status: BINDING. Written against canon-1.12 (adversarial check pending). Applies DECISIONS.md through row 145.

This chapter fixes how content is stored and checked: the file layout and envelope, schema identity, the common definitions every record schema builds on, the custom schema keywords, the validation pipeline with its error codes, the catalog of record kinds that chapters 00 to 18 define, and the rules the Rust loader in `palimpsest_data` mirrors. It owns the full item instance schema (Canon §13.9). It owns no content namespace. Per-chapter record schemas arrive as fragments in §6.

The implementation lives in the workspace: `palimpsest/schemas/common.schema.json`, `palimpsest/schemas/envelope.schema.json`, `palimpsest/schemas/00.namespace.schema.json`, `palimpsest/tools/validate_content.py`, `palimpsest/tools/gen_namespaces.py`, the generated `palimpsest/content/00/meta/namespaces.json` and the fixture set under `palimpsest/tools/validate_fixtures/`. Where this text and those files disagree, this text wins and the files are fixed.

## Imports

| Source | Consumed |
|---|---|
| Canon §1.1 | content-ID grammar, 64-byte limit, namespace = first segment, aliases, `id32 = fnv1a32(utf8(id))`, collision rejection, `id32_collision`, the mod second segment `x_<mod_id>` |
| Canon §1.2, §1.4, §1.5 | numeric types and widths (Bp `i32`, power tables `i64` Bp, `Ticks` `u32`, `Bam` `u16`), the `Fx` coordinate range `|raw| < 2^62`, display units |
| Canon §1.6 | integer-only content, the JSON integer bound `±(2^53 − 1)`, `u64` as `"0x"` hex strings, authoring units (mm, mm/s, mm/s², Bam, Bp, ticks, grams, ml, DeciC) |
| Canon §1.8–§1.10 | block coordinates, cone half-angles `0..=32768`, pitch clamp `±16000`, text-key grammar and the `loc.` reservation |
| Canon §3.1 | `Rng::weighted` takes `u32` weights with a positive total |
| Canon §5.1–§5.4 | axioms `0..=10`, dependency sets, tolerance rows summing to 4 |
| Canon §6.2, §6.3, §6.4 row 12 | stored and effective stat ranges, attribute requirements, weapon scaling `c_i ∈ 0..=10000` and `Σ c_i ≤ 15000` "validated by schema" |
| Canon §7, §10.1, §10.2, §10.5 | skill range, damage types and colors, resistance range, type shares summing to 10000 |
| Canon §12, §13.6, §13.7, §13.9 | satiation range; knowledge states; `ENH_CAP`; the item instance contract, whose full schema this chapter owns |
| Canon §15, §16.5 | reputation and favor range `-1000..=1000` |
| Canon §17.7, §19 | retired IDs; namespace ownership |
| Canon §21 tests 4, 29, 34 | `fnv1a32("genre.cyberpunk") = 0x99b54427`; ID validation and collisions; no floats in content |
| Doc 02 §3.1, §3.3, §3.4 | workspace layout (`content/<doc>/<ns>/`, committed `schemas/`), `schemars` and `jsonschema` pins, `load_content`, `ContentDb` |
| Doc 02 §11.1, §11.2, §11.5 | content path, envelope, the three validation passes, `id32` registry; record bodies as RFC 8785 JSON in `content_hash`; `content/13/ai/curve_tables.json` |
| Doc 02 §13.1, §17.3 | mod patches and the `x-patchable` keyword; flag records |
| `palimpsest_data` (`ids.rs`, `hex.rs`, `registry.rs`, `content_check.rs`, `test_support.rs`) | grammar check order and messages, hex `u64` form, alias registration order, number classification including `-0`, the four real `fnv1a32` collision pairs |
| Docs 01–18, Exports and main tables | the record kinds of the §5 catalog |
| Doc 07 §7.1; doc 10 §2.3, §4; doc 11 §2.2 | part slot names; the knowledge flag word, counters and `zaps_total`; the instance example, affix tiers `1..=5`, perk columns |
| Doc 01 §2.7 | the `01.pillar_test` fields `id`, `pillar`, `index` `1..=3`, `carry_forward` |
| Doc 11 §1 rule 8, §2.1 | loot-table `lines` as the one `x-patchable` array; template loot records for owners' IDs |
| Doc 13 §2.1 | enemy template fields `visual`, `body_state.crouch_mm`, `flags.net_always`, `outfit`, `outfit_mass_g`; attack-profile `knockback_mm_s`; the outfit and size validation rules |
| Doc 16 §11.4, test 50 | the economy API schema at `palimpsest/schemas/svc/econ_api.json` |
| DECISIONS.md 55, 65, 89 | no mounts or vehicles; no item durability; `enemy.role.charger` replaces the retired cavalry role |
| DECISIONS.md 128, 132, 137 | cat.seed, block.crop, mat.tag.tillable and mode.prologue retired; no seeds and no Prologue in the catalog; a first-run achievement replaces the Prologue achievements |
| DECISIONS.md 141 | at most 40 curated Steam achievements; the long tail as in-game Codex marks |

---

## 1. Files, envelope and schema identity

### 1.1 Content files

1. Every content file lives at `content/<NN>/<namespace>/<name>.json` under the workspace root. `<NN>` is the two-digit owning document, `00` to `18`. `<namespace>` matches `[a-z][a-z0-9_]*` and equals the first segment of every record ID in the file. `<name>` matches `[a-z0-9_]+`.
2. Files and directories whose names start with `.` are ignored. Any other file under `content/` that does not match rule 1 is an error (`E_PATH`).
3. A file is UTF-8 without a byte order mark and holds one RFC 8259 JSON value. An object never repeats a key.
4. One file holds records of one schema. One namespace spans as many files and schemas as its owner needs, for example `content/05/arch/melee.json` and `content/05/arch/ranged.json`.
5. The generated ownership table is the file `content/00/meta/namespaces.json` (§1.4).

### 1.2 Envelope and records

Each file is exactly this object, with no other key:

```json
{
  "schema": "00.status",
  "records": [
    { "id": "st.bleeding", "category": "dot", "base_duration": 360 },
    { "id": "st.burning", "category": "dot", "base_duration": 240 }
  ]
}
```

- `schema` is a record schema ID (§1.3). The library schemas `common` and `envelope` are never named here.
- `records` is an array of objects. Each record has `id`, a content ID (Canon §1.1), and when the record was renamed, `aliases`: a non-empty array of unique content IDs, the retired old IDs that now resolve to this record. Aliases are how this chapter realizes Canon §1.1's alias entries (Canon gaps G2).
- **One ID, one record.** An ID is defined by exactly one record in the whole content set, and no alias equals another record's ID or alias. A document that adds data about a record another document owns does it in one of two ways: (a) a schema fragment that the owner's schema merges, so the owner's file carries the fields; or (b) a record kind of its own, in a namespace it owns, whose field references the other record through `x-idref`.
- **Template loot IDs.** Doc 11's kind (b) records for templates owned by docs 04 to 08 (`11.template_loot`) carry the ID `loot.tpl.` followed by the owner's template ID with every `.` written as `_`, for example loot.tpl.arch_melee_spear for `arch.melee.spear`, and the field `template` (`ref_item_template`) holding the owner's ID. A template ID used this way is at most 55 bytes, so the derived ID stays within 64 bytes (Canon §1.1). Two records never name one template: their derived IDs are equal and the second is `E_DUP_ID`.
- Record order inside a file and file order carry no meaning. `content_hash` sorts records by ID (doc 02 §11.2).
- Every record schema allows `id` and `aliases` and nothing else beyond its own properties (§1.3).

### 1.3 Schema identity

- **Grammar.** A record schema ID is `<owner_doc>.<record_kind>`, regex `^(0[0-9]|1[0-8])\.[a-z][a-z0-9_]{0,47}$`, for example `13.enemy_template`. The owner part is the document whose `content/<NN>/` directory holds the files. Two library schema IDs exist besides: `common` and `envelope`. A schema ID starts with a digit, so it never parses as a content ID.
- **File.** A schema is the file `palimpsest/schemas/<schema_id>.schema.json` with `"$schema": "https://json-schema.org/draft/2020-12/schema"` and `"$id": "urn:palimpsest:schema:<schema_id>"`.
- **References.** A schema reaches another schema only by absolute URN, `urn:palimpsest:schema:common#/$defs/<name>`, and its own definitions by `#/$defs/<name>`.
- **Record schema root.** Every record schema has this shape:

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "urn:palimpsest:schema:00.status",
  "title": "Status",
  "x-owner": "00",
  "$ref": "urn:palimpsest:schema:common#/$defs/record_base",
  "required": ["category", "base_duration"],
  "properties": {
    "category": { "enum": ["dot", "debuff", "control", "buff", "special"] },
    "base_duration": { "$ref": "urn:palimpsest:schema:common#/$defs/ticks_pos" },
    "builds_from": { "$ref": "urn:palimpsest:schema:common#/$defs/ref_dmg" }
  },
  "unevaluatedProperties": false
}
```

- **Schema rules** (each breach is `E_SCHEMA_DEF` unless noted):
  1. the file name is `<schema_id>.schema.json` with a valid schema ID;
  2. `$schema` and `$id` are exactly as above;
  3. the file is a valid JSON Schema 2020-12 document;
  4. no subschema has `"type": "number"` or a type list containing it; numbers are integers only (Canon §1.6);
  5. the file itself contains no float token (`E_FLOAT_TOKEN`) and no integer outside `±(2^53 − 1)` (`E_INT_RANGE`);
  6. patterns are ECMA-262 regular expressions that never use the classes `\d`, `\w`, `\s` or their upper-case negations, whose meaning differs between ECMA-262 and Python; write `[0-9]` and the like, and anchor with `^` and `$` where the whole string is meant;
  7. a record schema root carries `x-owner` equal to its owner part, `"$ref": "urn:palimpsest:schema:common#/$defs/record_base"` and `"unevaluatedProperties": false`; the library schemas carry no `x-owner`;
  8. the only keywords starting with `x-` are the six of §3, each with a well-formed value;
  9. every `$ref` resolves.
- A record schema admits any record namespace; the §5 catalog states which namespaces each schema serves, and a schema pins them with a pattern on `id` where the owner wants the check.
- When two schema directories are searched (the self-test, `--schemas`), the first directory that holds a schema ID wins.
- Subdirectories of `palimpsest/schemas/` hold schemas that are not content record schemas, such as the service API schemas of doc 16 §11.4 (`palimpsest/schemas/svc/econ_api.json`). Stage 1 reads only the top-level `*.schema.json` files, never a subdirectory, and no envelope names a subdirectory schema.

### 1.4 The ownership table

The validator never reads Markdown. `tools/gen_namespaces.py` reads Canon §19 and §17.7 once and writes `content/00/meta/namespaces.json`, schema `00.namespace`, one record per line, sorted by ID. CI runs `python3 tools/gen_namespaces.py --check`, which fails when the committed file differs from what the canon generates. The generator finds the canon at `--canon <path>`, else `../docs/palimpsest/spec/00_CANON.md` relative to the workspace (the Collab_Reposit layout), else `docs/spec/00_CANON.md` inside the workspace (the standalone export layout).

| Field | Kind | Meaning |
|---|---|---|
| `id` | both | `meta.ns.<prefix without its final dot>` or `meta.retired.<token with dots as underscores>`, with `_all` appended for a retired prefix |
| `kind` | both | `prefix` or `retired` |
| `source` | both | `canon §19 row <n>`, `canon §17.7`, or `doc 19 §1.4 (Canon gaps G1)` |
| `prefix` | prefix | the namespace prefix, ending in `.`, for example `arch.melee.` |
| `owners` | prefix | sorted two-digit documents whose `content/<NN>/` may hold records under the prefix |
| `rule_owners` | prefix | documents that own rules about the namespace but not its records |
| `owner_text` | prefix | the canon's owner cell, verbatim |
| `text_keys` | prefix | true for `loc.`, whose strings are text keys and never content IDs |
| `row` | prefix | the 1-based row of Canon §19 |
| `retired` | retired | a retired ID, or a retired prefix ending in `.` |

```json
{ "id": "meta.ns.block", "kind": "prefix", "prefix": "block.", "owners": ["03", "07"], "rule_owners": [], "owner_text": "07 (blocks shared with 03)", "text_keys": false, "row": 7, "source": "canon §19 row 7" }
```

**Owner-cell rules**, applied to the second cell of each Canon §19 row:

1. The cell is split at `;` into clauses.
2. A clause that starts with a two-digit number is a record-owner clause. Every two-digit number in it that is not part of a section reference (`§11.1`, `§12`) owns every prefix of the row, except that two scoping phrases give their number to one prefix only: the parenthetical "(blocks shared with 03)" and the phrase "core rank values in 00" name the prefix whose namespace is the named word, or that word minus a final `s`.
3. A clause that does not start with a number ("Echo acquisition in 15", "account economy rules in 15", "owners per §11.1 and §12 for behavior") names rule owners; its numbers go to `rule_owners`, which placement ignores.
4. A row whose owner cell says "not content IDs" is a text-key namespace.

Result for the rows these rules touch: `ax.` and `genre.` belong to 00 and 12; `dmg.`, `st.` and `res.` to 00 only (the §11.1 and §12 owners supply behavior fields through fragments to 00's schemas); `block.` to 07 and 03, the other row-7 prefixes to 07 alone; the row-10 prefixes (`know.`, `idc.`, `appr.`, `buc.`, `curse.`) to 10 and 00; the row-11 prefixes (`item.` through `slot.`) to 11 and 00; `rank.` to 13 and 00, the other row-12 prefixes to 13; the row-13 prefixes to 14 with rule owner 15; `cur.` and `ent.` to 00 with rule owner 15; `ev.` and `rng.` to 00 and 03; `loc.` is a text-key namespace of 18. The generator adds `meta.`, owner 00, for its own records (Canon gaps G1).

**Retired IDs.** The generator takes every backticked ID or prefix from each sentence of Canon §17.7 that declares "retired IDs", and nothing else from that section, so a replacement such as `enemy.role.charger` is not retired.

**Matching.** A record ID belongs to the longest `prefix` record it starts with. No match is `E_NS_UNKNOWN`. A match whose `owners` lack the file's `<NN>` is `E_OWNER`.

### 1.5 Mods

Mod packages (doc 02 §13.1) use the same schemas and envelope under `mods/<mod_id>/content/<NN>/<namespace>/<name>.json`, with record IDs whose second segment is `x_<mod_id>` (Canon §1.1). Base content never uses a second segment that starts with `x_` (`E_MOD_ID`). The validator of §4 checks base content; the mod loader of doc 02 §13.1 applies the same stages to each mod, plus its own rules (forbidden namespaces, patches on `x-patchable` arrays, `id32_collision` against loaded content).

---

## 2. Common definitions

`palimpsest/schemas/common.schema.json` (`urn:palimpsest:schema:common`) holds these `$defs`. Record schemas use them by `$ref`, never by copying. No definition uses `"type": "number"`. `i53` means `-9007199254740991..=9007199254740991` and `u53` means `0..=9007199254740991`, the Canon §1.6 bound.

### 2.1 Identifiers and text

| `$def` | Exact constraint | Source |
|---|---|---|
| `id` | string, `maxLength` 64, pattern `^(?!loc\.)[a-z][a-z0-9_]*(\.[a-z0-9_]+){1,5}$` | Canon §1.1, §1.10 |
| `id_list` | array of `id`, `uniqueItems` | |
| `ref_any` | `id` with `x-idref: ["*"]` | §3.1 |
| `loc_key` | string, `maxLength` 160, pattern `^loc\.[a-z0-9_.]+$` | Canon §1.10 |
| `schema_id` | string, pattern `^(0[0-9]|1[0-8])\.[a-z][a-z0-9_]{0,47}$` | §1.3 |
| `doc_number` | integer `0..=18` | doc 01 `owner_docs` |
| `doc_dir` | string, pattern `^(0[0-9]|1[0-8])$` | §1.1 |
| `hex_u64` | string, pattern `^0x[0-9A-Fa-f]{1,16}$` | Canon §1.6, `palimpsest_data::hex` |
| `blake3_hex` | string, pattern `^[0-9a-f]{64}$` | Canon §2.4 |
| `date` | string, pattern `^[0-9]{4}-(0[1-9]|1[0-2])-(0[1-9]|[12][0-9]|3[01])$`; the owner checks that the date exists | Canon §3.6 |
| `record_base` | object, `required: ["id"]`, `id`: `id`, `aliases`: array of `id`, `minItems` 1, `uniqueItems` | §1.2 |

### 2.2 Integer widths

| `$def` | Range |
|---|---|
| `i8` | `-128..=127` |
| `u8` | `0..=255` |
| `i16` | `-32768..=32767` |
| `u16` | `0..=65535` |
| `i32` | `-2147483648..=2147483647` |
| `u32` | `0..=4294967295` |
| `i53` | `-9007199254740991..=9007199254740991` |
| `u53` | `0..=9007199254740991` |

A Rust `i64` or `u64` field uses `i53` or `u53` when its content values fit, else `hex_u64` (Canon §1.6).

### 2.3 Units

| `$def` | Exact constraint | Source |
|---|---|---|
| `bp` | `i32` | Canon §1.2 Bp |
| `bp_i64` | `i53` | Canon §1.2, §13.8 power tables and `i64` table multipliers |
| `bp_unit` | integer `0..=10000` | probabilities, shares, `pen_bp`, scaling coefficients |
| `pct_bp` | integer `-10000..=10000` | additive percentage modifiers |
| `mult_bp` | integer `0..=2147483647` | non-negative multipliers |
| `resist_bp` | integer `-10000..=9000` | Canon §10.2 |
| `milli` | `i53` | Canon §1.2 `Milli` |
| `milli_nonneg` | `u53` | |
| `mm`, `mm_s`, `mm_s2` | integer `-1073741823999..=1073741823999` | Canon §1.6; `fx_from_milli` keeps `|raw| < 2^62` (Canon §1.4) exactly for `|mm| < 2^30 × 1000` |
| `mm_nonneg` | integer `0..=1073741823999` | |
| `mm_pos` | integer `1..=1073741823999` | radii, ranges, sizes |
| `mm3` | object `{x, y, z}` of `mm`, all required, no other key | Canon §1.8 axes |
| `block3` | object `{x, y, z}` of `i32`, all required, no other key | Canon §1.8 |
| `ticks` | `u32` | Canon §1.2 `Ticks` |
| `ticks_pos` | integer `1..=4294967295` | |
| `bam` | `u16` | Canon §1.9 |
| `bam_cone` | integer `0..=32768` | Canon §1.9 cone half-angles |
| `pitch` | integer `-16000..=16000` | Canon §1.9 |
| `grams`, `ml` | `u53` | Canon §1.2 |
| `deci_c`, `db` | `i32` | Canon §1.2 |
| `light` | integer `0..=15` | Canon §1.2 |
| `axiom` | integer `0..=10` | Canon §5.1 |
| `sub_attr_base` | integer `1..=100` | Canon §6.2 |
| `sub_attr_eff` | integer `1..=150` | Canon §6.2, §6.3 |
| `skill_value` | integer `0..=100` | Canon §7 |
| `satiation` | integer `0..=2400` | Canon §12 |
| `char_xp`, `money`, `armor_rating` | `u53` | Canon §1.2 |
| `standing` | integer `-1000..=1000` | Canon §15, §16.5 |
| `depth` | integer `1..=4294967295` | Canon §13.8 depths, item and source levels |
| `party_slot` | integer `0..=3` | Canon §2.4 |
| `color` | string, pattern `^#[0-9A-F]{6}$` | Canon §5.9, §10.1, §13.3 |

### 2.4 References and composites

| `$def` | Exact constraint |
|---|---|
| `ref_axis`, `ref_genre`, `ref_dmg`, `ref_status`, `ref_resource`, `ref_attr`, `ref_sub`, `ref_skill`, `ref_material`, `ref_quality`, `ref_rarity`, `ref_buc`, `ref_affix`, `ref_perk`, `ref_mfr`, `ref_part`, `ref_faction`, `ref_rank`, `ref_event`, `ref_category`, `ref_slot`, `ref_loot` | `id` with `x-idref` of `ax.`, `genre.`, `dmg.`, `st.`, `res.`, `attr.`, `sub.`, `skill.`, `mat.`, `q.`, `rar.`, `buc.`, `affix.`, `perk.`, `mfr.`, `part.`, `fac.`, `rank.`, `ev.`, `cat.`, `slot.`, `loot.` respectively |
| `ref_item_template` | `id` with `x-idref` of `arch.melee.`, `arch.ranged.`, `arch.armor.`, `arch.offhand.`, `arch.ward.`, `ammo.`, `item.`, `alch.potion.`, `alch.bomb.`, `apparatus.`, `gadget.`, `part.`, `mat.`, `ing.`, `block.`, `aug.` (doc 11 §2.1, Canon §4.3) |
| `dep_set` | array of `ref_axis`, `uniqueItems`, `maxItems` 4 (Canon §5.3) |
| `axiom_vector` | object with exactly `arcana`, `techne`, `mythos`, `verve`, each `axiom` (Canon §5.2) |
| `weight` | `u32` (Canon §3.1) |
| `weighted_entry` | object, `required: ["w"]`, `w`: `weight`; extended by the user schema with its own properties and `unevaluatedProperties: false` |
| `positive_weight` | object, `required: ["w"]`, `w` at least 1; used as `contains` so a table's total is positive |
| `dmg_share` | object `{type: ref_dmg, bp: bp_unit}`, both required, no other key |
| `dmg_shares` | array of `dmg_share`, `minItems` 1, `x-sum {field: "bp", total: 10000}`, `x-unique-by: "type"` (Canon §10.5) |
| `scaling_entry` | object `{sub: ref_sub, c: bp_unit}`, both required, no other key |
| `weapon_scaling` | array of `scaling_entry`, `x-sum-max {field: "c", max: 15000}`, `x-unique-by: "sub"` (Canon §6.4 row 12) |
| `attr_requirement` | object `{attr: ref_attr, min: sub_attr_eff}`, both required, no other key |
| `attr_requirements` | array of `attr_requirement`, `x-unique-by: "attr"` (Canon §6.3) |
| `part_slot` | enum `barrel`, `feed`, `optic`, `stock`, `graft`, `edge`, `haft`, `grip`, `focus`, `plate`, `lining`, `coil`, `module_1`, `module_2` (doc 07 §7.1) |
| `knowledge` | object: `state` (enum of the four Canon §13.6 states, `x-idref: ["know."]`, required), `flags` (`u32`, required, doc 10 §2.3 bit layout), `appr_progress` (`i53`), `appr_tier_done` (`-1..=127`), `est_skill` (`i16`), `est_key` (`hex_u64`), `est_steps` (`0..=4`), `fam_ticks` (`u32`), `fam_kills` (`u16`), `analysis_lvl` (exactly 4 integers `-1..=32767`); no other key |
| `affix_roll` | object `{id: ref_affix, tier: 1..=5, roll_bp: bp_unit}`, all required, no other key (doc 11 §2.2) |
| `part_entry` | object `{slot: part_slot, part_id: ref_part, mfr: ref_mfr or null, quality: ref_quality}`, `slot` and `part_id` required, no other key |
| `charges` | object `{cur, max}`, each integer `0..=2147483647`, both required, no other key |
| `perk_column` | object `{column: 1..=5, options: 1 to 3 unique ref_perk, chosen: 0..=2}`, all required, no other key; `chosen` indexes `options` |
| `item_instance` | §2.5 |

### 2.5 Item instance

`item_instance` is the full schema of Canon §13.9. Content uses it wherever a record carries fixed instances (starting kits, fixed rewards, test fixtures); saves serialize the same fields in the canonical binary form of doc 02 §6.5. `unevaluatedProperties` is false, so owner documents add fields only through their §6 fragments, which this definition then merges.

| Field | `$def` or constraint | Required |
|---|---|---|
| `uid` | `hex_u64` | yes |
| `template` | `ref_item_template` | yes |
| `origin_genre` | `ref_genre` | yes |
| `rarity` | `ref_rarity` | yes |
| `quality` | `ref_quality` | yes |
| `ilvl` | integer `1..=2147483647` | yes |
| `enhancement` | integer `0..=2147483647`; the record kind that holds a fixed instance checks `ENH_CAP` for its depth (Canon §13.7) | yes |
| `buc` | `ref_buc` | yes |
| `curse_penalty` | integer `0..=3` | yes |
| `knowledge` | `knowledge` | yes |
| `affixes` | array of `affix_roll` | yes |
| `parts` | array of `part_entry`, `x-unique-by: "slot"` | yes |
| `materials` | array of `ref_material` | yes |
| `charges` | `charges` | no |
| `zaps_total` | `u32` (doc 10 §4) | no |
| `stack` | integer `1..=2147483647` | no |
| `mass_g` | `grams` | yes |
| `value` | `money` | yes |
| `manufacturer` | `ref_mfr` or `null` | yes |
| `gear_tier` | integer `1..=5` | yes |
| `perk_columns` | array of `perk_column`, `x-unique-by: "column"` | yes |
| `illuminated` | boolean | yes |
| `finder_slot` | `party_slot` | yes |

---

## 3. Custom keywords

The validator adds these keywords to JSON Schema 2020-12 with `jsonschema.validators.extend`. Standard validators ignore them, so the schemas stay valid 2020-12 documents. Each keyword applies wherever its subschema is evaluated, including inside `$ref`, `allOf`, `anyOf`, `oneOf` and `items`. A keyword whose instance has a type it does not handle does nothing; the `type` keyword reports the mismatch.

### 3.1 `x-idref`

- **Value:** a non-empty array of unique namespace prefixes, each matching `^[a-z][a-z0-9_]*\.([a-z0-9_]+\.){0,4}$` and naming a namespace of Canon §19, or exactly `["*"]` for any namespace. A text-key prefix is never listed; text keys use `loc_key`.
- **Instance:** a string, or an array whose string elements are each checked.
- **At validation (stage 7):** a string that breaks the ID grammar is `E_ID_GRAMMAR`; a valid ID that starts with none of the listed prefixes is `E_REF_PREFIX`; otherwise the validator records the reference (file, JSON Pointer, ID, prefixes). Both errors are schema errors, so inside `anyOf` and `oneOf` they make that branch fail.
- **After all files are loaded (stage 10):** each recorded reference must name a record ID or an alias's old ID. A retired ID is `E_RETIRED`. An alias resolves with `W_REF_ALIAS`, and its target must also start with a listed prefix (`E_REF_PREFIX`). A missing ID is `E_REF_MISSING`, or `W_REF_MISSING` when it starts with a prefix given to `--allow-missing-prefix`. IDs that the canon or a chapter names but no content file defines are not accepted implicitly.
- One instance location recorded several times (for example by `unevaluatedProperties` re-evaluation or by two branches) counts once, with the union of the prefixes.
- Schema authors never let one instance location be an ID in one branch and free text in another.

```json
{ "type": "array", "uniqueItems": true, "items": { "type": "string" }, "x-idref": ["st."] }
```

### 3.2 `x-sum`

- **Value:** `{"total": <integer>}` with an optional `"field": <non-empty string>`.
- **Instance and sum:** an array with `field`: the integer `field` values of its object elements; an array without `field`: its integer elements; an object without `field`: its integer property values; an object with `field`: the integer `field` values of its object-valued properties. Booleans and non-integers do not count; the schema's own types reject them.
- **Rule:** the sum equals `total` exactly; an empty array sums to 0. Otherwise `E_SUM` at the instance.

```json
{ "type": "object", "additionalProperties": { "type": "integer", "minimum": 0, "maximum": 2 }, "x-sum": { "total": 4 } }
```

### 3.3 `x-sum-max`

- **Value:** `{"max": <integer>}` with an optional `"field"`. The sum is formed as for `x-sum`.
- **Rule:** the sum is at most `max`. Otherwise `E_SUM_MAX` at the instance. `weapon_scaling` uses it for Canon §6.4 row 12.

### 3.4 `x-unique-by`

- **Value:** a field name, or a non-empty array of unique field names (a composite key).
- **Instance:** an array. Object elements that carry every key field take part; others are skipped.
- **Rule:** no two taking-part elements have equal key tuples (JSON equality). Each later duplicate is one `E_UNIQUE` at that element, naming the first index. ID-keyed data is always written as an array of objects with `x-unique-by` on the ID field, never as an object keyed by IDs, so `x-idref` applies to the keys.

### 3.5 `x-owner`

- **Value:** a two-digit string `"00"` to `"18"`, only at a record schema's root, equal to the owner part of its schema ID.
- **Rule:** every file whose envelope names the schema lives under `content/<x-owner>/`. Otherwise `E_SCHEMA_OWNER` at `/schema`; its records are still validated. A misplaced or malformed `x-owner` in a schema file is `E_SCHEMA_DEF`.

### 3.6 `x-patchable`

- **Value:** a boolean, on a subschema with `"type": "array"` (doc 02 §13.1).
- **Rule:** an annotation: `true` marks an array a mod patch may append to. The content validator only checks its placement (`E_SCHEMA_DEF`); the mod loader enforces it.
- **Use:** the `lines` array of `11.loot_table` carries `"x-patchable": true`; it is the array doc 02 §13.1's `02.patch` example targets and the only list a loot table has (doc 11 §1 rule 8).

### 3.7 Implementation notes

- Strings are parsed into a `str` subclass that carries its JSON Pointer, so `x-idref` records locations without help from `jsonschema`.
- `pattern` is evaluated with ECMA-262 end-of-input semantics: a final `$` matches only at the end of the string, never before a final newline.
- Every finding is kept once per (code, file, pointer, message), so an ID that fails both `id`'s pattern and `x-idref` is reported once.

---

## 4. Validation pipeline and error codes

### 4.1 Stages

The validator runs these stages in this order over the workspace root. Within a stage, files go in ascending relative-path byte order, and findings of one record in ascending JSON Pointer order (array indices numerically), then code, then message.

| Stage | Input | Checks | Codes | What continues |
|---|---|---|---|---|
| 1 | `schemas/*.schema.json`, then `--schemas` directories | §1.3 schema rules; token rules on schema text; `$ref` resolution | `E_SCHEMA_DEF`, `E_JSON`, `E_FLOAT_TOKEN`, `E_INT_RANGE` | a defective schema is unusable; files naming it skip stage 7 |
| 2 | `content/00/meta/namespaces.json` or `--namespaces` | read before stage 1 (stage 1 checks `x-idref` prefixes against it); its findings print after stage 1's | `E_NAMESPACES` | without a table, stage 8 ownership and retirement checks are skipped |
| 3 | every file under `content/` | §1.1 path layout | `E_PATH` | non-matching files are dropped |
| 4 | each file's bytes | UTF-8, no byte order mark, RFC 8259 syntax, no repeated key; every number token: `.`, `e` or `E` in the token, or the token `-0`, is `E_FLOAT_TOKEN`; an integer token outside `±(2^53 − 1)` is `E_INT_RANGE`; string contents are never scanned | `E_JSON`, `E_FLOAT_TOKEN`, `E_INT_RANGE` | `E_JSON` drops the file; a token error skips stage 7 for the file but keeps its records for stages 8 to 10 |
| 5 | parsed value | `envelope.schema.json`; record `id` and each alias against the `palimpsest_data` grammar | `E_ENVELOPE`, `E_ID_GRAMMAR` | a file-level envelope error drops the file; a record-level one, or a bad `id`, drops that record; a bad alias drops that alias |
| 6 | `schema` field | the schema exists and is usable; its `x-owner` equals the file's `<NN>` | `E_SCHEMA_UNKNOWN`, `E_SCHEMA_OWNER` | an unknown schema skips stage 7 for the file |
| 7 | each record | JSON Schema 2020-12 with the §3 keywords; references collected | `E_SCHEMA`, `E_ID_GRAMMAR`, `E_REF_PREFIX`, `E_SUM`, `E_SUM_MAX`, `E_UNIQUE` | every record continues |
| 8 | each record ID and alias | namespace equals the directory; longest-prefix owner match; no `x_` second segment; not retired | `E_NS_DIR`, `E_NS_UNKNOWN`, `E_OWNER`, `E_MOD_ID`, `E_RETIRED` | a name refused with `E_MOD_ID` is not registered in stage 9, as `palimpsest_data::registry` refuses it; every other name continues |
| 9 | all record IDs, then all aliases | §4.3 registry | `E_DUP_ID`, `E_ID32_COLLISION` | a rejected name is not registered |
| 10 | collected references | §3.1 resolution | `E_REF_MISSING`, `W_REF_MISSING`, `W_REF_ALIAS`, `E_RETIRED`, `E_REF_PREFIX` | |

### 4.2 Error codes

Every finding has a code, a file (relative to the root, `/` separators), an RFC 6901 JSON Pointer into that file (empty for the whole file) and a message. `E_` codes are errors and `W_` codes are warnings.

| Code | Stage | Meaning | Pointer |
|---|---|---|---|
| `E_SCHEMA_DEF` | 1 | a schema file breaks §1.3 or §3 | into the schema file |
| `E_NAMESPACES` | 2 | the ownership table is missing, unreadable or malformed | the table, or the bad record |
| `E_PATH` | 3 | a file outside the §1.1 layout | empty |
| `E_JSON` | 4 | not UTF-8 JSON, byte order mark, syntax error or repeated key; the message gives line and column | empty |
| `E_FLOAT_TOKEN` | 4 | a number token with a fraction or exponent part, or `-0` | the number |
| `E_INT_RANGE` | 4 | an integer token outside `±(2^53 − 1)` | the number |
| `E_ENVELOPE` | 5 | the file is not `{schema, records}`, or a record is not an object with a string `id`, or `aliases` is not an array of strings | the offending value |
| `E_ID_GRAMMAR` | 5, 7 | a record ID, alias or reference breaks Canon §1.1 as `palimpsest_data::ids` implements it; the message is that crate's reason | the string |
| `E_SCHEMA_UNKNOWN` | 6 | no schema file for the envelope's schema ID | `/schema` |
| `E_SCHEMA_OWNER` | 6 | the schema's `x-owner` differs from the file's `<NN>` | `/schema` |
| `E_SCHEMA` | 7 | any other JSON Schema failure; for `anyOf` and `oneOf` the message adds the closest branch | the instance |
| `E_REF_PREFIX` | 7, 10 | a reference, or an alias's target, outside the field's `x-idref` prefixes | the reference |
| `E_SUM` | 7 | an `x-sum` total differs from its value | the array or object |
| `E_SUM_MAX` | 7 | an `x-sum-max` total exceeds its maximum | the array or object |
| `E_UNIQUE` | 7 | an `x-unique-by` key repeats | the later element |
| `E_NS_DIR` | 8 | the record's namespace differs from the file's namespace directory | `/records/<i>/id` |
| `E_NS_UNKNOWN` | 8 | no Canon §19 prefix matches the record ID | `/records/<i>/id` |
| `E_OWNER` | 8 | the matched prefix's owners lack the file's `<NN>` | `/records/<i>/id` |
| `E_MOD_ID` | 8 | a record ID or alias of base content has a second segment starting with `x_` | the ID |
| `E_RETIRED` | 8, 10 | a record ID, alias or reference equals a retired ID or starts with a retired prefix | the ID |
| `E_DUP_ID` | 9 | the same string is defined twice, as records or as a record and an alias | the later definition |
| `E_ID32_COLLISION` | 9 | two different strings share an `id32` | the later definition |
| `E_REF_MISSING` | 10 | a reference names no record and no alias | the reference |
| `W_REF_MISSING` | 10 | as `E_REF_MISSING`, downgraded by `--allow-missing-prefix` | the reference |
| `W_REF_ALIAS` | 10 | a reference uses an alias's old ID | the reference |

### 4.3 Registry

- The registry mirrors `palimpsest_data::registry::IdRegistry::build`: record IDs first in ascending byte order (ties by file, then pointer), then aliases in ascending byte order of the old ID, then of the new ID.
- Each name takes `id32 = fnv1a32(utf8(name))` with offset basis `0x811C9DC5` and prime `0x01000193` (Canon §1.1). A name whose `id32` is taken is rejected at its own location: `E_DUP_ID` when the taken entry is the same string, `E_ID32_COLLISION` otherwise, naming the first holder and the `id32`. The Rust registry reports both cases as `id32_collision`.
- An alias's target is the record that lists it, so the target always exists.
- The summary counts registered records, registered aliases, distinct reference locations and resolved references.

### 4.4 Command line and output

```text
python3 tools/validate_content.py [--root <workspace>] [--json]
                                  [--allow-missing-prefix PREFIX ...]
                                  [--namespaces FILE] [--schemas DIR ...] [--self-test]
```

- `--root` defaults to the directory above `tools/`. It holds `content/`, `schemas/` or both, else the run is a usage error.
- `--allow-missing-prefix` takes prefixes ending in `.` (for example `genre.`), for incremental work while referenced records do not exist yet.
- `--namespaces` replaces the ownership table path; `--schemas` adds schema directories searched after `<root>/schemas`.
- Text output is one line per finding, `error|warning <CODE> <file>#<pointer>: <message>`, in stage order, then the summary line `validate_content: <n> files, <n> records, <n> aliases, <n> references (<n> resolved), <n> schemas: <n> errors, <n> warnings`.
- `--json` prints one object `{"errors": [...], "warnings": [...], "summary": {...}}`; each finding is `{code, file, pointer, message}`.
- Exit status: 0 when there is no error (warnings allowed), 1 when there is at least one error, 2 on a usage error.
- Requirements: Python 3.12 or later, `jsonschema` 4.18 or later (4.26.0 pinned in CI) and `referencing` (0.37.0 pinned).

### 4.5 Self-test and fixtures

`--self-test` runs, from the workspace:

1. built-in vectors: `fnv1a32("genre.cyberpunk") = 0x99b54427` and `fnv1a32("crate.sim") = 0xdc634881`; the four collision pairs of `palimpsest_data::test_support`; every grammar case of `palimpsest_data::ids` tests with its exact message; a 3,000-string differential test of the `id` pattern against the byte scanner; the doc 02 test 19 token vectors plus `-0` and the exponent spellings; the `hex_u64` pattern against accepted and rejected spellings; the `id` pattern in `common.schema.json` equals the validator's;
2. every case directory under `tools/validate_fixtures/cases/`. A case is a workspace root with `content/`, an optional `schemas/` and `expect.json`. Schemas are searched in the case's `schemas/`, then `tools/validate_fixtures/schemas/` (shared fixture schemas, record kinds named `t_*`), then the workspace `schemas/`. The ownership table is the case's own when it has one, else the workspace's.

```json
{
  "description": "Canon §6.4 row 12: 15001 fails, 15000 passes.",
  "errors": { "E_SUM_MAX": 1 },
  "warnings": {},
  "allow_missing_prefix": [],
  "namespaces": "auto"
}
```

A case passes when the multiset of error codes and the multiset of warning codes equal `errors` and `warnings` exactly. `namespaces` is `auto`, `case` or `none` (validate without a table). The self-test also fails when any code of §4.2 is expected by no case, so every code keeps a fixture. The delivered set has 27 cases: one valid content set (`good`) and one case per code family.

### 4.6 CI

The `content` job of `.github/workflows/palimpsest-ci.yml` installs `jsonschema==4.26.0` and `referencing==0.37.0` on Python 3.13 and runs, from `palimpsest/`: `python3 tools/gen_namespaces.py --check`, `python3 tools/validate_content.py --self-test` and `python3 tools/validate_content.py`. The workflow also triggers on changes to `docs/palimpsest/spec/00_CANON.md`, so a canon §19 edit that leaves the table stale fails at once.

---

## 5. Schema catalog

One row per record kind that chapters 00 to 18 define as content. The schema ID fixes the owner directory; the content path is the file the owner creates first (more files under the same directory are allowed, §1.1). A chapter that adds a record kind adds a row through its §6 fragment. "(proposed)" marks a kind that resolves a conflict listed under Interface assumptions or Canon gaps.

| Schema ID | Owner | Record namespace | Content path | Purpose |
|---|---|---|---|---|
| `00.namespace` | 00 | `meta.ns.`, `meta.retired.` | `content/00/meta/namespaces.json` | Canon §19 ownership and Canon §17.7 retired IDs, generated (§1.4) |
| `00.axiom` | 00 | `ax.` | `content/00/ax/axioms.json` | the four axioms (Canon §5.1) |
| `00.damage_type` | 00 | `dmg.` | `content/00/dmg/damage_types.json` | damage types: category, color, built status, armor efficiency, `ward_mult` (Canon §10.1) |
| `00.status` | 00 | `st.` | `content/00/st/statuses.json` | statuses: category, threshold group, scaling, stacking, base duration, effect; behavior fields from the Canon §11.1 owners (Canon §11) |
| `00.resource` | 00 | `res.` | `content/00/res/resources.json` | resource pools and ranges (Canon §12) |
| `00.currency` | 00 | `cur.` | `content/00/cur/currencies.json` | currencies and their scope (Canon §14) |
| `00.entitlement` | 00 | `ent.` | `content/00/ent/entitlements.json` | entitlements (Canon §14) |
| `00.rng_stream` | 00 | `rng.` | `content/00/rng/streams.json` | sequential and keyed streams: scope, owner, uses (Canon §3.2, §3.3, §3.5) |
| `01.pillar` | 01 | `pillar.` | `content/01/pillar/pillars.json` | pillars: order, hard constraint, tests, owner docs (doc 01 §2.7) |
| `01.pillar_test` | 01 | `pillar.<name>.t1` to `.t3` | `content/01/pillar/tests.json` | pillar acceptance tests: `id`, `pillar` (a `pillar.` reference), `index` `1..=3`, `carry_forward` (true only for `pillar.one_ruleset.t3`) (doc 01 §2.7) |
| `01.kpi` | 01 | `kpi.` | `content/01/kpi/kpis.json` | KPIs: unit, statistic, mode, window, bands (doc 01 §10.2) |
| `02.crate` | 02 | `crate.` | `content/02/crate/crates.json` | crate layers and dependency allow lists (doc 02 §3.3) |
| `02.flag` | 02 | `flag.` | `content/02/flag/flags.json` | runtime feature flags (doc 02 §17.3) |
| `02.perf_budget` | 02 | `perf.` | `content/02/perf/perf.json` | performance budgets (doc 02 §15) |
| `02.patch` | 02 | a mod namespace `<ns>.x_<mod_id>.` | `mods/<mod_id>/patches/<name>.json` | mod append patches on `x-patchable` arrays; checked by the mod loader (doc 02 §13.1) |
| `03.component` | 03 | `comp.` | `content/03/comp/components.json` | the component catalog (doc 03 §3.3) |
| `03.system` | 03 | `sys.` | `content/03/sys/systems.json` | the systems of the tick order (doc 03 §2.2) |
| `03.grid_enum` | 03 | `grid.` | `content/03/grid/enums.json` | shapes, liquid interactions, random-tick kinds, entity kinds, move modes, causes, despawn reasons, gaits (doc 03 Exports) |
| `03.event` | 03 | `ev.` | `content/03/ev/events.json` | Canon §18 events and bus events with their payload schemas (doc 03 §14) |
| `04.attribute` | 04 | `attr.` | `content/04/attr/attributes.json` | primary attributes (Canon §6.1, doc 04 §15.1) |
| `04.sub_attribute` | 04 | `sub.` | `content/04/sub/sub_attributes.json` | sub-attributes (Canon §6.1) |
| `04.skill` | 04 | `skill.` | `content/04/skill/skills.json` | skills: governing sub-attribute, group, XP events (Canon §7, doc 04 §5) |
| `04.mastery_perk` | 04 | `skill.<name>.m25` to `.m100` | `content/04/skill/mastery.json` | skill mastery perks (doc 04 §5.3) |
| `04.talent` | 04 | `talent.` | `content/04/talent/talents.json` | Calling and general talents: tier, kind, prerequisite (doc 04 §7, §15.4) |
| `04.origin` | 04 | `origin.` | `content/04/origin/origins.json` | Origins: modifiers summing to 0, Origin skill, trait (Canon §16.1, doc 04 §2.3) |
| `04.calling` | 04 | `calling.` | `content/04/calling/callings.json` | Callings: major and minor skills, kits, trees (Canon §16.2, doc 04 §7) |
| `04.kin` | 04 | `kin.` | `content/04/kin/kin.json` | kin: body plan, traits, skins (Canon §16.3, doc 04 §2.4) |
| `04.trait` | 04 | `trait.` | `content/04/trait/traits.json` | Origin, kin, Marginalia and Echo traits (doc 04 §15.6) |
| `04.augment` | 04 | `aug.` | `content/04/aug/augments.json` | augments, mutations, prosthetics, implant forms (doc 04 §9.4) |
| `04.affliction` | 04 | `afflict.` | `content/04/afflict/afflictions.json` | afflictions (Canon §16.4, doc 04 §10) |
| `04.wound` | 04 | `wound.` | `content/04/wound/wounds.json` | wound tiers and kinds (doc 04 §8.2) |
| `04.body_plan` | 04 | `body.` | `content/04/body/body_plans.json` | body plans and their parts (Canon §9) |
| `04.body_part` | 04 | `bp.` | `content/04/bp/body_parts.json` | parts: share, `hit_mult`, precision, severability, donors (Canon §9, §9.1, doc 04 §15.8) |
| `05.arch_melee` | 05 | `arch.melee.` | `content/05/arch/melee.json` | melee archetypes (doc 05 §3) |
| `05.arch_ranged` | 05 | `arch.ranged.` | `content/05/arch/ranged.json` | ranged, thrown, grenade and casting-implement archetypes (doc 05 §5) |
| `05.arch_armor` | 05 | `arch.armor.` | `content/05/arch/armor.json` | armor archetypes (doc 05 §11) |
| `05.arch_offhand` | 05 | `arch.offhand.` | `content/05/arch/offhand.json` | shields, lanterns, foci, tomes (doc 05 §12.1) |
| `05.arch_ward` | 05 | `arch.ward.` | `content/05/arch/ward.json` | ward generator archetypes (doc 05 §15.4) |
| `05.ammo` | 05 | `ammo.` | `content/05/ammo/ammo.json` | ammunition and counter charges (doc 05 §8, §15.3) |
| `05.move` | 05 | `move.` | `content/05/move/moves.json` | combat moves (doc 05 §1.8) |
| `05.counter` | 05 | `counter.` | `content/05/counter/counters.json` | champion counter types (Canon §17.3a, doc 05 §15.1) |
| `06.glyph_form` | 06 | `glyph.form.` | `content/06/glyph/forms.json` | spell forms (doc 06 §5.2) |
| `06.glyph_essence` | 06 | `glyph.ess.` | `content/06/glyph/essences.json` | essences (doc 06 §5.3) |
| `06.glyph_modifier` | 06 | `glyph.mod.` | `content/06/glyph/modifiers.json` | modifiers (doc 06 §5.4) |
| `06.glyph_trigger` | 06 | `glyph.trig.` | `content/06/glyph/triggers.json` | triggers (doc 06 §5.5) |
| `06.glyph_condition` | 06 | `glyph.cond.` | `content/06/glyph/conditions.json` | conditions (doc 06 §5.6) |
| `06.school` | 06 | `school.` | `content/06/school/schools.json` | schools of magic (doc 06 §7.1) |
| `06.spell` | 06 | `spell.` | `content/06/spell/spells.json` | fixed spells of wands, texts and signatures (doc 06 §8.6) |
| `06.summon` | 06 | `summon.` | `content/06/summon/summons.json` | summon templates (doc 06 §12.2) |
| `06.polymorph` | 06 | `summon.poly.` | `content/06/summon/polymorph.json` | polymorph templates (doc 06 §12.3) |
| `06.patron` | 06 | `patron.` | `content/06/patron/patrons.json` | patron Authors (Canon §16.5, doc 06 §16.1) |
| `06.prayer` | 06 | `prayer.` | `content/06/prayer/prayers.json` | prayers (doc 06 §16) |
| `06.enchantment` | 06 | `ench.` | `content/06/ench/enchantments.json` | enchantments (doc 06 §9.4) |
| `07.material` | 07 | `mat.`, including ores `mat.<metal>.ore` | `content/07/mat/materials.json` | the one material table: density, `dmg_bp`, `ar_bp`, `conceal`, tags (Canon §13.8a, doc 07 §2) |
| `07.material_role` | 07 | `mat.role.` | `content/07/mat/roles.json` | material roles (doc 07 §2.4) |
| `07.material_tag` | 07 | `mat.tag.` | `content/07/mat/tags.json` | material tags (doc 07 §2.4) |
| `07.quality` | 07 | `q.` | `content/07/q/qualities.json` | quality tiers and multipliers (Canon §13.5) |
| `07.tool_tier` | 07 | `tooltier.` | `content/07/tooltier/tool_tiers.json` | tool tiers (doc 07 §9.1) |
| `07.station` | 07 | `station.` | `content/07/station/stations.json` | crafting stations (doc 07 §5.1) |
| `07.recipe_kind` | 07 | `recipe.kind.` | `content/07/recipe/kinds.json` | recipe kinds (doc 07 §6.1) |
| `07.recipe` | 07 | `recipe.` | `content/07/recipe/recipes.json` | recipes (doc 07 §6 to §12) |
| `07.part` | 07 | `part.` | `content/07/part/parts.json` | loose weapon parts (doc 07 §7.2) |
| `07.graft` | 07 | `part.graft.` | `content/07/part/grafts.json` | licensed grafts (doc 07 §7.5) |
| `07.block` | 07 | `block.` | `content/07/block/blocks.json` | blocks, resource nodes, light fixtures and placeables; a placeable block carries `block.tag.errata_placeable` and is the `cat.placeable` item template of the same ID; doc 03 grid fields arrive by fragment (Canon §4.3, doc 07 §10) |
| `07.block_tag` | 07 | `block.tag.` | `content/07/block/tags.json` | block tags (doc 07 §10.1) |
| `07.gadget` | 07 | `gadget.` | `content/07/gadget/gadgets.json` | gadgets (doc 07 §11.2) |
| `08.liquid` | 08 | `liq.` | `content/08/liq/liquids.json` | liquids (doc 08 §2.2, §3.1) |
| `08.gas` | 08 | `gas.` | `content/08/gas/gases.json` | gases (doc 08 §4.2) |
| `08.ingredient` | 08 | `ing.` | `content/08/ing/ingredients.json` | ingredients and modifier reagents (doc 08 Exports; seeds are cut, DECISIONS.md 128) |
| `08.alch_effect` | 08 | `alch.eff.` | `content/08/alch/effects.json` | alchemical effects (doc 08 §7) |
| `08.potion` | 08 | `alch.potion.` | `content/08/alch/potions.json` | potion item templates (doc 08 §12.1, §12.2) |
| `08.bomb` | 08 | `alch.bomb.` | `content/08/alch/bombs.json` | bomb item templates (doc 08 §13.2) |
| `08.reaction` | 08 | `alch.rx.` | `content/08/alch/reactions.json` | alchemy reactions (doc 08 §5.3) |
| `08.transmutation` | 08 | `alch.tx.` | `content/08/alch/transmutations.json` | transmutations (doc 08 §15) |
| `08.apparatus` | 08 | `apparatus.`, `apparatus.vessel.`, `apparatus.respirator.` | `content/08/apparatus/apparatus.json` | apparatus, vessels and respirators (doc 08 §4.3, §6.1, §9.1) |
| `09.awareness_state` | 09 | `aware.` (the six states) | `content/09/aware/states.json` | awareness ladder (doc 09 §6.1) |
| `09.perception_profile` | 09 | `aware.profile.` | `content/09/aware/profiles.json` | perception profiles (doc 09 §4.1) |
| `09.camo_family` | 09 | `aware.camo.` | `content/09/aware/camo.json` | camouflage families (doc 09 §3.1) |
| `09.hiding_spot` | 09 | `aware.hide.` | `content/09/aware/hiding.json` | hiding spot kinds (doc 09 §10.1) |
| `09.reaction_class` | 09 | `aware.react.` | `content/09/aware/reactions.json` | AI reaction classes (doc 09 §7.7) |
| `09.noise` | 09 | `noise.` | `content/09/noise/noises.json` | noise sources: base dB, surface terms, reaction (doc 09 §7) |
| `09.disguise_tier` | 09 | `disguise.tier.` | `content/09/disguise/tiers.json` | disguise tiers (doc 09 §11.1) |
| `09.disguise_trigger` | 09 | `disguise.trigger.` | `content/09/disguise/triggers.json` | disguise triggers (doc 09 §11.3) |
| `09.lock_tier` | 09 | `lock.t1` to `lock.t6` | `content/09/lock/tiers.json` | lock tiers (doc 09 §13.1) |
| `09.lock_kind` | 09 | `lock.kind.`, `lock.skin.` | `content/09/lock/kinds.json` | lock kinds and skins (doc 09 §13.1) |
| `10.knowledge_state` | 10 | `know.` (the four states) | `content/10/know/states.json` | knowledge ladder (Canon §13.6, doc 10 §2) |
| `10.knowledge_rank` | 10 | `know.rank.`, `know.bestiary.` | `content/10/know/ranks.json` | Index and Bestiary ranks (doc 10 §12.1, §12.4) |
| `10.appearance_class` | 10 | `idc.` (the seven classes) | `content/10/idc/classes.json` | unknown-appearance classes (Canon §13.6, doc 10 §3.2) |
| `10.appearance` | 10 | `idc.<class>.` | `content/10/idc/descriptors.json` | appearance descriptors (doc 10 §3.3) |
| `10.appraisal_tier` | 10 | `appr.tier.` | `content/10/appr/tiers.json` | appraisal tiers (doc 10 §5.5) |
| `10.appraisal_use` | 10 | `appr.use.` | `content/10/appr/uses.json` | identification by use (doc 10 §5.1, §5.2) |
| `10.appraisal_effect` | 10 | `appr.eff.` | `content/10/appr/effects.json` | identification effects (doc 10 §5.4, §5.10) |
| `10.appraisal_service` | 10 | `appr.svc.` | `content/10/appr/services.json` | appraisal services (doc 10 §5.9) |
| `10.appraisal_reason` | 10 | `appr.reason.` | `content/10/appr/reasons.json` | reveal reasons (doc 10 §5) |
| `10.curse_state` | 10 | `buc.` | `content/10/buc/states.json` | curse states (Canon §13.6) |
| `10.curse` | 10 | `curse.` | `content/10/curse/curses.json` | plain curses (doc 10 §7.3) |
| `10.pact` | 10 | `curse.pact.` | `content/10/curse/pacts.json` | pact curses with their boons (Canon §13.6, doc 10 §7.8) |
| `11.item` | 11 | `item.` | `content/11/item/items.json` | item templates: accessories, consumables, tools, books, bags, keys, Lucre, quest items, Remains, Exotics, Palimpsestic uniques (doc 11 §3, §8) |
| `11.template_loot` | 11 | `loot.tpl.` | `content/11/loot/templates.json` | doc 11 loot fields for templates owned by docs 04 to 08, one record per template, ID derived from the template ID, `template` field by `x-idref` (§1.2) |
| `11.rarity` | 11 | `rar.` | `content/11/rar/rarities.json` | rarities (Canon §13.3, doc 11 §3.8) |
| `11.category` | 11 | `cat.` | `content/11/cat/categories.json` | item categories (Canon §13.1, doc 11 §3.1) |
| `11.slot` | 11 | `slot.` | `content/11/slot/slots.json` | equipment slots (Canon §13.2, doc 11 §3.9) |
| `11.affix` | 11 | `affix.` | `content/11/affix/affixes.json` | affixes and tier rows (doc 11 §6.2) |
| `11.perk` | 11 | `perk.`, `perk.genre.`, `perk.leg.` | `content/11/perk/perks.json` | column perks, genre traits, Legendary perks (doc 11 §7.2, §8.1) |
| `11.manufacturer` | 11 | `mfr.` | `content/11/mfr/manufacturers.json` | manufacturers and signature traits (Canon §5.9, doc 11 §5.2) |
| `11.loot_table` | 11 | `loot.` | `content/11/loot/tables.json` | drop, chest, bag, boss, floor, quest, Keepsake, Reliquary-content, Engine-face and shop-stock tables; `lines` is `x-patchable` (§3.6; doc 11 §9 to §12) |
| `11.loot_pool` | 11 | `loot.pool.` | `content/11/loot/pools.json` | loot pool tags (doc 11 §2.5) |
| `11.loot_generator` | 11 | `loot.<generator>` | `content/11/loot/generators.json` | reserved sub-generator tags (doc 11 §2.3) |
| `11.reliquary` | 11 | `reliq.` | `content/11/reliq/reliquaries.json` | in-run Reliquaries (doc 11 §10) |
| `11.fortune_engine` | 11 | `engine.` | `content/11/engine/engines.json` | Fortune Engines (doc 11 §11) |
| `11.shop` | 11 | `shop.` | `content/11/shop/shops.json` | shops (doc 11 §12.1) |
| `12.genre` | 12 | `genre.` | `content/12/genre/genres.json` | genres: the Canon §5.2, §5.4 and §5.9 columns copied exactly, and the full Genre Parameter Vector (doc 12 §2, §3) |
| `12.law` | 12 | `law.` | `content/12/law/laws.json` | World Laws (Canon §5.10, doc 12 §4) |
| `12.zone` | 12 | `zone.` | `content/12/zone/zones.json` | zone kinds (doc 12 §14) |
| `12.lattice_node` | 12 | `lattice.node.` | `content/12/lattice/nodes.json` | lattice nodes (doc 12 §5.2) |
| `12.lattice_edge` | 12 | `lattice.edge.` | `content/12/lattice/edges.json` | weighted lattice edges (doc 12 §5.2, §5.3) |
| `12.folio_kind` | 12 | `lattice.folio.` | `content/12/lattice/folio_kinds.json` | Folio kinds (doc 12 §6.1) |
| `12.challenge` | 12 | `lattice.challenge.` | `content/12/lattice/challenges.json` | challenge Folios (doc 12 §6.3) |
| `12.marginalia_folio` | 12 | `lattice.marg.` | `content/12/lattice/marginalia.json` | Marginalia lattice entries (doc 12 §22) |
| `12.layout` | 12 | `gpv.layout.` | `content/12/gpv/layouts.json` | layout generators (doc 12 §3.4) |
| `12.world_event` | 12 | `gpv.evt.` | `content/12/gpv/events.json` | world events (doc 12 §19.1) |
| `12.story` | 12 | `gpv.story.` | `content/12/gpv/stories.json` | story slot templates, bridges and erasure (doc 12 §20.2) |
| `12.room_kind` | 12 | `room.` (kinds) | `content/12/room/kinds.json` | room kinds, the Colophon included (Canon §17.1, doc 12 §16.1) |
| `12.room_tag` | 12 | `room.` (tags) | `content/12/room/tags.json` | room tags (doc 12 §12) |
| `12.room_template` | 12 | `room.tpl.` | `content/12/room/templates.json` | room templates (doc 12 §16) |
| `12.room_rule` | 12 | `room.rule.` | `content/12/room/rules.json` | room rules (doc 12 §8.2) |
| `12.biome` | 12 | `biome.` | `content/12/biome/tilesets.json` | tilesets (doc 12 §3.3) |
| `12.hazard` | 12 | `hazard.` | `content/12/hazard/hazards.json` | hazard archetypes (doc 12 §18.1) |
| `12.secret` | 12 | `secret.room.`, `secret.exit.`, `secret.hide.` | `content/12/secret/secrets.json` | secret rooms, exits and hiding methods (doc 12 §17) |
| `13.enemy_role` | 13 | `enemy.role.` | `content/13/enemy/roles.json` | enemy roles (doc 13 §3) |
| `13.enemy_roster` | 13 | `enemy.<genre_suffix>` and the Editors roster | `content/13/enemy/rosters.json` | rosters (doc 13 §8) |
| `13.enemy_template` | 13 | `enemy.<roster>.<name>` | `content/13/enemy/templates.json` | enemy templates, invader kits and cross-genre enemies (doc 13 §8, §8.17, §10.7) |
| `13.ability` | 13 | `enemy.ab.` | `content/13/enemy/abilities.json` | enemy abilities and attack profiles (doc 13 §2.1, §3, §8) |
| `13.ai_curve` | 13 | `ai.curve.` | `content/13/ai/curve_tables.json` | response curves with knots from `tools/gen_curves.py` (doc 02 §11.5, doc 13 §4.4) |
| `13.ai_input` | 13 | `ai.input.` | `content/13/ai/inputs.json` | utility inputs (doc 13 §4.3) |
| `13.ai_action` | 13 | `ai.action.` | `content/13/ai/actions.json` | utility actions (doc 13 §4.7) |
| `13.behavior_set` | 13 | `ai.set.` | `content/13/ai/sets.json` | behavior sets (doc 13 §4.7) |
| `13.nav_class` | 13 | `ai.nav.` | `content/13/ai/nav.json` | navigation classes (doc 13 Exports) |
| `13.faction` | 13 | `fac.` | `content/13/fac/factions.json` | core, native and wild factions (Canon §15, doc 13 §12) |
| `13.rep_event` | 13 | `fac.rep.` | `content/13/fac/reputation.json` | reputation events (doc 13 §12.3) |
| `13.boss` | 13 | `boss.` | `content/13/boss/bosses.json` | Editors and mini-bosses (doc 13 §9) |
| `13.boss_attack` | 13 | `boss.<boss>.<attack>` | `content/13/boss/attacks.json` | boss attacks (doc 13 §9.2, §9.3) |
| `13.npc` | 13 | `npc.` | `content/13/npc/npcs.json` | vendors, quest givers, captives, hirelings, companions, Margin residents and quest NPCs (doc 13 §13) |
| `13.dialogue` | 13 | `npc.dlg.` | `content/13/npc/dialogue.json` | dialogue records (doc 13 §13.7) |
| `13.director_profile` | 13 | `director.profile.` | `content/13/director/profiles.json` | director profiles (doc 13 §10) |
| `13.director_phase` | 13 | `director.phase.` | `content/13/director/phases.json` | director phases (doc 13 §10.4) |
| `13.elite_modifier` | 13 | `elite.` | `content/13/elite/modifiers.json` | elite modifiers and the three champion aspects (Canon §17.3a, doc 13 §7.2) |
| `13.rank` | 13 | `rank.` | `content/13/rank/ranks.json` | enemy ranks (Canon §17.3) |
| `14.mode` | 14 | `mode.` | `content/14/mode/modes.json` | run modes and overlays (Canon §17.1, §17.5, §17.9, doc 14 §8.1) |
| `14.grade` | 14 | `grade.` | `content/14/grade/grades.json` | Grades (doc 14 §7.1) |
| `14.run_end` | 14 | `run.end.` | `content/14/run/ends.json` | run-end causes (doc 14 §4.3) |
| `14.rubric` | 14 | `run.rubric.` | `content/14/run/rubrics.json` | rubrics (doc 14 §3.4) |
| `14.note` | 14 | `run.note.` | `content/14/run/notes.json` | Folio notes (doc 14 §3.5) |
| `14.mutator` | 14 | `run.mut.` | `content/14/run/mutators.json` | mutators (doc 14 §8.7) |
| `14.marginalia_rule` | 14 | `run.marg.` | `content/14/run/marginalia.json` | Marginalia rules (doc 14 §9.3) |
| `14.boon` | 14 | `run.boon.` | `content/14/run/boons.json` | Colophon boons (doc 14 §3.9) |
| `14.kit` | 14 | `run.kit.` | `content/14/run/kits.json` | starting kits (doc 14 §2.4) |
| `14.margin_facility` | 14 | `margin.` (facilities) | `content/14/margin/facilities.json` | Margin facilities (doc 14 §5.1) |
| `14.margin_milestone` | 14 | `margin.milestone.` | `content/14/margin/milestones.json` | Margin milestones (doc 14 §5.1) |
| `14.margin_alcove` | 14 | `margin.` (alcove slots) | `content/14/margin/alcoves.json` | trophy and decoration slots (Canon §17.8, doc 14 §5.1a) |
| `14.unlock` | 14 | `unlock.` | `content/14/unlock/unlocks.json` | Bindery unlock nodes (doc 14 §5.3) |
| `14.echo` | 14 | `echo.` | `content/14/echo/echoes.json` | Echoes: kin, Origin, Calling, signature trait, body plan (Canon §16.2, doc 14 §6.3) |
| `14.score_component` | 14 | `score.` (components) | `content/14/score/components.json` | score components (doc 14 §4.2) |
| `14.leaderboard` | 14 | `score.board.` | `content/14/score/boards.json` | leaderboards (doc 14 §9.4) |
| `14.achievement` | 14 | `ach.` | `content/14/ach/achievements.json` | curated Steam achievements, at most 40 records, the first-run achievement and the Second Edition Ironman included (DECISIONS.md 137, 141; doc 14 §10) |
| `14.codex_mark` | 14 | `ach.mark.` (proposed) | `content/14/ach/marks.json` | in-game Codex marks, the long tail of account milestones that are not Steam achievements (DECISIONS.md 141) |
| `15.banner` | 15 | `banner.` (banners) | `content/15/banner/banners.json` | Inkwell banners (doc 15 §3.2) |
| `15.banner_pool` | 15 | `banner.pool.` | `content/15/banner/pools.json` | banner pools (doc 15 §3.1, §3.13) |
| `15.banner_tier` | 15 | `banner.tier.` | `content/15/banner/tiers.json` | catalog tiers (doc 15 §3.1) |
| `15.pass` | 15 | `pass.` | `content/15/pass/codex.json` | Codex pass tiers and XP sources (doc 15 §6) |
| `15.sku` | 15 | `store.sku.` | `content/15/store/skus.json` | store SKUs (doc 15 §7.1) |
| `15.price_matrix` | 15 | `store.price_matrix.` | `content/15/store/prices.json` | regional prices (doc 15 §7.2) |
| `15.bundle` | 15 | `bundle.` | `content/15/bundle/bundles.json` | Scripts and SKU grant lists (doc 15 §3.1.2) |
| `15.cosmetic` | 15 | `cosm.` | `content/15/cosm/cosmetics.json` | cosmetics by category, Echo Folds, Recollection rewards (doc 15 §3.8) |
| `15.cosmetic_category` | 15 | `cosm.cat.` | `content/15/cosm/categories.json` | cosmetic categories (doc 15 Exports) |
| `15.jurisdiction_flag` | 15 | `juris.flag.`, `juris.paid_random_items_enabled` | `content/15/juris/flags.json` | compliance flags (Canon §22, doc 15 §8.2) |
| `15.jurisdiction_region` | 15 | `juris.region.` | `content/15/juris/regions.json` | regions (doc 15 §8.1) |
| `16.net_params` | 16 | `net.params.` | `content/16/net/params.json` | co-op and protocol tuning (doc 16 §1.2) |
| `16.net_enum` | 16 | `net.topo.`, `net.route.`, `net.cstate.`, `net.peer.`, `net.migrate.`, `net.trade.`, `net.lobby.`, `net.voice.`, `net.session.` | `content/16/net/enums.json` | topologies, routes, connection states and the other closed network enumerations (doc 16 §2 to §9) |
| `16.net_channel` | 16 | `net.ch.` | `content/16/net/channels.json` | channels and lanes (doc 16 §3.2) |
| `16.net_message` | 16 | `net.msg.` | `content/16/net/messages.json` | the message table (doc 16 §3.7) |
| `16.net_reject` | 16 | `net.reject.` | `content/16/net/rejects.json` | rejection reasons (doc 16 §3.8) |
| `16.netsim_scenario` | 16 | `net.netsim.` | `content/16/net/netsim.json` | network simulation scenarios (doc 16 Exports) |
| `16.service` | 16 | `svc.` | `content/16/svc/services.json` | backend services (doc 16 Exports) |
| `16.service_error` | 16 | `svc.err.` | `content/16/svc/errors.json` | service error codes (doc 16 Exports) |
| `17.ui_skin` | 17 | `ui.skin.` | `content/17/ui/skins.json` | genre UI skins (doc 17 §2.1) |
| `17.hud_element` | 17 | `ui.hud.` | `content/17/ui/hud.json` | HUD elements (doc 17 §3.2, §3.3) |
| `17.menu` | 17 | `ui.menu.` | `content/17/ui/menus.json` | menus and screens (doc 17 §4 to §11) |
| `17.icon` | 17 | `ui.icon.` | `content/17/ui/icons.json` | the icon set (doc 17 §2.5) |
| `17.tip` | 17 | `ui.tip.` | `content/17/ui/tips.json` | tips (doc 17 Exports) |
| `17.rumble` | 17 | `ui.rumble.` | `content/17/ui/rumble.json` | haptic patterns (doc 17 Exports) |
| `17.input_action` | 17 | `input.` (actions) | `content/17/input/actions.json` | input actions (doc 17 §12.1) |
| `17.input_config` | 17 | `input.cfg.` | `content/17/input/config.json` | input settings (doc 17 Exports) |
| `17.input_context` | 17 | `input.ctx.` | `content/17/input/contexts.json` | input contexts (doc 17 Exports) |
| `17.input_preset` | 17 | `input.preset.` | `content/17/input/presets.json` | binding presets (doc 17 §12.3) |
| `17.access_setting` | 17 | `access.` | `content/17/access/settings.json` | accessibility and comfort settings (doc 17 Exports) |
| `18.palette` | 18 | `art.palette.` | `content/18/art/palettes.json` | genre palettes (doc 18 §3.1) |
| `18.light` | 18 | `art.light.` | `content/18/art/lights.json` | lighting rigs and fog (doc 18 §3.2) |
| `18.post` | 18 | `art.post.` | `content/18/art/post.json` | post-process stacks (doc 18 §3.3) |
| `18.art_bible` | 18 | `art.bible.` | `content/18/art/bibles.json` | art bible entries (doc 18 §3.4) |
| `18.font` | 18 | `art.font.` | `content/18/art/fonts.json` | font families (doc 18 §3.5) |
| `18.silhouette` | 18 | `art.silhouette.` | `content/18/art/silhouettes.json` | silhouette roles (doc 18 §3.6) |
| `18.visual` | 18 | `art.visual.`, `art.vm.`, `art.native.`, `art.idc.`, `art.icon.`, `art.readability.` | `content/18/art/visuals.json` | visual records, viewmodels, native forms, appearance renders, icons, readability limits (doc 18 §3.8, §5.1, §5.5) |
| `18.mesh` | 18 | `art.mesh.`, `art.rig.` | `content/18/art/meshes.json` | mesh and rig asset records (doc 02 §11.4, doc 18 §5) |
| `18.anim_pose` | 18 | `art.anim.` | `content/18/art/poses.json` | baked hitbox pose tables in the doc 03 §5.16 format (Canon §9, doc 02 §11.4) |
| `18.block_art` | 18 | `art.block.`, `art.blockarray.` | `content/18/art/blocks.json` | block layers and per-genre block texture arrays (doc 02 §8.3, doc 18 §3.8) |
| `18.texture` | 18 | `art.tex.`, `art.lut.` | `content/18/art/textures.json` | textures, decals, plates and LUTs (doc 18 Exports) |
| `18.vfx` | 18 | `vfx.` | `content/18/vfx/effects.json` | damage, status, awareness, Contradiction, ward, debris, transition, telegraph and ragdoll effects (doc 18 §4.5, §5.4) |
| `18.music` | 18 | `aud.music.` | `content/18/aud/music.json` | music tracks, stems and stings (doc 18 §6.1) |
| `18.sound` | 18 | `aud.sfx.`, `aud.set.`, `aud.amb.`, `aud.vo.` | `content/18/aud/sounds.json` | effects, sets, ambience and voice lines (doc 18 §6.3, §6.5) |
| `18.audio_mix` | 18 | `aud.bus.`, `aud.mix.` | `content/18/aud/mix.json` | buses and the mix budget (doc 18 §6.6) |
| `18.lore` | 18 | `nar.lore.`, `nar.author.`, `nar.editor.`, `nar.margin.`, `nar.narrator`, `nar.ending.` | `content/18/nar/lore.json` | lore, Authors, Editors, Margin cast, the Narrator, endings (doc 18 §8) |
| `18.grammar` | 18 | `nar.grammar.`, `nar.names.`, `nar.register.` | `content/18/nar/grammars.json` | registers, name grammars and story grammars (doc 18 §7.3, §7.4, §8.7) |
| `18.scribe` | 18 | `nar.job.`, `nar.scribe.` | `content/18/nar/scribe.json` | Scribe jobs, models and budget (doc 18 §9.2, §9.3) |
| `18.name_review` | 18 | `nar.review.` | `content/18/nar/review.json` | name-review ban lists (doc 18 §7.1) |
| `18.locale` | 18 | `nar.locale.` (proposed) | `content/18/nar/locales.json` | launch locales and fallback order (doc 18 §9.11; Canon gaps G7) |

---

### 5.1 Fields and rules recorded ahead of the fragments

These fields and validation rules are fixed now; the owner's §6 fragment carries them in its field table and schema.

- **`13.enemy_template`** (doc 13 §2.1): `visual`, a content ID with `x-idref: ["art.visual."]`, equal to `art.visual.` followed by the template ID; `body_state.crouch_mm` (`mm_pos`, equal to `stand_mm` for a body that never crouches); the flag `net_always` (`0` or `1`) in `flags`; `flags.outfit` (`0` or `1`) and `outfit_mass_g` (`grams`).
- **`13.ability`** (doc 13 §2.1): the attack-profile field `knockback_mm_s` (`u32`, a speed in mm/s, `0` for none). No knockback field is a distance.
- **Validation rules for `13.enemy_template`:** `outfit` is 1 exactly on templates of a `fac.native.` faction with `body.humanoid` and the `sapient` tag; `outfit_mass_g` is 9000 for roles brute, guardian and charger, 6000 for every other role with `outfit` 1, and 0 when `outfit` is 0; every `body.humanoid` template of role brute has `stand_mm` at least 2200; `crouch_mm` is at most the nav-class headroom less 50 mm (950 for `ai.nav.small`, 1950 for `ai.nav.humanoid` and `ai.nav.burrower`, 2950 for `ai.nav.large`); `net_always` is 1 only on templates of `rank.editor` and on NPC templates of `rank.hunter`. These rules cross fields and records, so doc 13's content tests check them; the schema checks types and ranges.

## 6. Record schemas by owner document

Each owner document supplies its fragment in its section below; the orchestrator merges fragments in place. A fragment for doc NN gives, for each of its schema IDs in §5: a field table (field, `$def` or type, exact constraint, meaning) and the complete schema as a `json` block that passes the §1.3 rules; it adds §5 rows for any record kind it introduces, and states the fields it adds to `item_instance` (§2.5). It changes nothing outside its section. The schema files themselves go to `palimpsest/schemas/`, and CI validates them with stage 1.

## 6.00 Doc 00 records

<!-- SCHEMA-FRAGMENT:00 -->

## 6.01 Doc 01 records

<!-- SCHEMA-FRAGMENT:01 -->

## 6.02 Doc 02 records

<!-- SCHEMA-FRAGMENT:02 -->

## 6.03 Doc 03 records

<!-- SCHEMA-FRAGMENT:03 -->

## 6.04 Doc 04 records

<!-- SCHEMA-FRAGMENT:04 -->

## 6.05 Doc 05 records

<!-- SCHEMA-FRAGMENT:05 -->

## 6.06 Doc 06 records

<!-- SCHEMA-FRAGMENT:06 -->

## 6.07 Doc 07 records

<!-- SCHEMA-FRAGMENT:07 -->

## 6.08 Doc 08 records

<!-- SCHEMA-FRAGMENT:08 -->

## 6.09 Doc 09 records

<!-- SCHEMA-FRAGMENT:09 -->

## 6.10 Doc 10 records

<!-- SCHEMA-FRAGMENT:10 -->

## 6.11 Doc 11 records

<!-- SCHEMA-FRAGMENT:11 -->

## 6.12 Doc 12 records

<!-- SCHEMA-FRAGMENT:12 -->

## 6.13 Doc 13 records

<!-- SCHEMA-FRAGMENT:13 -->

## 6.14 Doc 14 records

<!-- SCHEMA-FRAGMENT:14 -->

## 6.15 Doc 15 records

<!-- SCHEMA-FRAGMENT:15 -->

## 6.16 Doc 16 records

<!-- SCHEMA-FRAGMENT:16 -->

## 6.17 Doc 17 records

<!-- SCHEMA-FRAGMENT:17 -->

## 6.18 Doc 18 records

<!-- SCHEMA-FRAGMENT:18 -->

---

## 7. The Rust loader mirror (`palimpsest_data`)

The Python validator is the reference for CI; `palimpsest_data` (feature `loader`, doc 02 §3.2) must reach the same verdicts at load time.

1. **Schemas from types.** Record types derive `schemars::JsonSchema` (feature `schema`). `pal schema export` writes `schemas/<schema_id>.schema.json`; `pal schema export --check` fails when a committed file differs from the generated one. Two schemas are equal when their parsed JSON values are equal after removing every `title`, `description`, `$comment` and `examples` key; key order and whitespace never matter. Every other keyword, every `x-` keyword and every `$defs` name counts.
2. **Common definitions.** Newtypes that map to a §2 definition (`ContentId` to `id`, `TextKey` to `loc_key`, the hex `u64` to `hex_u64`, `Bp` to `bp`, `Ticks` to `ticks`, `Milli` to `milli`, and so on) implement `JsonSchema` by hand and return `{"$ref": "urn:palimpsest:schema:common#/$defs/<name>"}`, so generated schemas reference `common.schema.json` instead of inlining local copies. `common.schema.json` and `envelope.schema.json` are hand-written and are the source of truth for those definitions.
3. **Record schema shape.** The schemars output for a record type is made to equal §1.3: `x-owner` and `x-idref`, `x-sum`, `x-sum-max`, `x-unique-by` and `x-patchable` through `#[schemars(extend(...))]`, the `record_base` reference and `"unevaluatedProperties": false` at the root, `$id` set to the URN.
4. **Stages.** The loader runs §4.1's stages in order with the same codes as variants of one error enum (`E_FLOAT_TOKEN` as `FloatToken`, and so on). Today's `content_check` covers stages 4 and 5: `ViolationKind::NonInteger` is `E_FLOAT_TOKEN` (it already rejects `-0`), `IntegerOutOfRange` is `E_INT_RANGE`, `Envelope` is `E_ENVELOPE`, `InvalidId` is `E_ID_GRAMMAR`. `HexU64Fields` path patterns give way to the `hex_u64` definition.
5. **Repeated keys.** `serde_json` keeps the last of two equal object keys; the loader detects repeated keys itself (for example with a `Deserializer` visitor that rejects a second occurrence) and reports `E_JSON`.
6. **Custom keywords.** `jsonschema` 0.58 ignores unknown keywords; the loader registers the six keywords of §3 as custom keywords with the same semantics, and its `pattern` uses ECMA-262 end-of-input semantics.
7. **Registry.** `IdRegistry::build` takes every record ID and, as `(old, new)` pairs, every record's `aliases`. Its `Id32Collision` with `existing == incoming` is `E_DUP_ID`, otherwise `E_ID32_COLLISION`; both keep the canon code `id32_collision` for mods. Its `ModSegment` error (`mod_segment_invalid`) is `E_MOD_ID`.
8. **Ownership.** The loader reads `content/00/meta/namespaces.json` and never parses Markdown.
9. **Equivalence test.** A `palimpsest_data` test runs every case under `tools/validate_fixtures/cases/` and asserts the same multisets of codes as `expect.json`. Finding order and message text are free; codes and pointers are not.
10. **Content hash.** Doc 02 §11.2 hashes each record as RFC 8785 JSON after defaults are applied; the Python validator never applies defaults, so the hash is the loader's job alone.

---

## Exports

This chapter defines no content IDs. It exports:

- **Schema IDs:** the library schemas `common` and `envelope`; the record schema `00.namespace`; and the reserved catalog IDs of §5 (216 rows), which the owner documents implement through §6.
- **Common definitions (§2):** `id`, `id_list`, `ref_any`, `loc_key`, `schema_id`, `doc_number`, `doc_dir`, `hex_u64`, `blake3_hex`, `date`, `record_base`; `i8`, `u8`, `i16`, `u16`, `i32`, `u32`, `i53`, `u53`; `bp`, `bp_i64`, `bp_unit`, `pct_bp`, `mult_bp`, `resist_bp`, `milli`, `milli_nonneg`, `mm`, `mm_nonneg`, `mm_pos`, `mm_s`, `mm_s2`, `mm3`, `block3`, `ticks`, `ticks_pos`, `bam`, `bam_cone`, `pitch`, `grams`, `ml`, `deci_c`, `light`, `db`, `axiom`, `sub_attr_base`, `sub_attr_eff`, `skill_value`, `satiation`, `char_xp`, `money`, `armor_rating`, `standing`, `depth`, `party_slot`, `color`; the 23 `ref_*` definitions; `dep_set`, `axiom_vector`, `weight`, `weighted_entry`, `positive_weight`, `dmg_share`, `dmg_shares`, `scaling_entry`, `weapon_scaling`, `attr_requirement`, `attr_requirements`, `part_slot`, `knowledge`, `affix_roll`, `part_entry`, `charges`, `perk_column`, `item_instance`.
- **Custom keywords (§3):** `x-idref`, `x-sum`, `x-sum-max`, `x-unique-by`, `x-owner`, `x-patchable`.
- **Error codes (§4.2):** `E_SCHEMA_DEF`, `E_NAMESPACES`, `E_PATH`, `E_JSON`, `E_FLOAT_TOKEN`, `E_INT_RANGE`, `E_ENVELOPE`, `E_ID_GRAMMAR`, `E_SCHEMA_UNKNOWN`, `E_SCHEMA_OWNER`, `E_SCHEMA`, `E_REF_PREFIX`, `E_SUM`, `E_SUM_MAX`, `E_UNIQUE`, `E_NS_DIR`, `E_NS_UNKNOWN`, `E_OWNER`, `E_MOD_ID`, `E_RETIRED`, `E_DUP_ID`, `E_ID32_COLLISION`, `E_REF_MISSING`, `W_REF_MISSING`, `W_REF_ALIAS`.
- **Tools and formats:** the `validate_content.py` command line and output (§4.4); `gen_namespaces.py` and its owner-cell rules (§1.4); the ownership record format; the fixture `expect.json` format (§4.5); the fragment contract (§6).

## Interface assumptions

| Doc | Assumed to provide |
|---|---|
| 00 to 18 | a §6 fragment per owned record kind in the §6 contract, using the §2 definitions and the §5 schema IDs; content files only in its own `content/<NN>/` directory |
| 00 | Canon §19 adds `meta.` (G1); Canon §1.1 points its alias entries at §1.2 (G2) |
| 01 | still requested: `pillars.json`, `tests.json` and `kpis.json` wrapped in the §1.2 envelope (doc 01 §2.7 and §10.2 show bare arrays); `owner_docs` and `owner_doc` as `doc_number`; present: the `01.pillar_test` fields (doc 01 §2.7) |
| 02 | present: the `02.patch` example targets loot-table `lines` (§3.6); `pal validate` implements §4 with these codes; the `02.patch` schema for mod patches; doc 02's Interface assumption on doc 19 ("the schema catalog matching `palimpsest_data` record types, including the `x-patchable` keyword") is met by §3.6, §5 and §7 |
| 03 | present: doc 03 imports `block.air` from doc 07 and defines no block record; one record per block ID: `block.air` and every other shared block live in doc 07's `07.block` files, and doc 03 adds its grid fields to `07.block` by fragment; `ev.` records in `content/03/ev/`; pose tables as `18.anim_pose` records in the doc 03 §5.16 format |
| 05 | the Exports list drops the `arch.mount.` archetypes, which Canon §17.7 retires; content using them fails with `E_RETIRED` |
| 07 | the §7.1 part slot names are the closed `part_slot` set; the instance fields `crafted`, `mat_bonus_bp`, `dish` and ammunition `smart` arrive in fragment 6.07 |
| 08 | the instance extensions `alch`, `coat`, `filter` and `tier` arrive in fragment 6.08 |
| 10 | the `knowledge` counters and `zaps_total` of §2.5 match doc 10 §2.3 and §4 |
| 11 | the doc 11 §2.2 instance fields arrive in fragment 6.11, without `durability`, which Canon §4.3 (canon-1.10) removed; present: loot data for templates owned by docs 04 to 08 is stored as `11.template_loot` records (doc 11 §2.1); still requested: doc 11 exports those IDs under the §1.2 scheme `loot.tpl.<template ID, dots as underscores>` |
| 12 | `12.genre` records copy the Canon §5.2, §5.4 and §5.9 columns exactly, the tolerance row with `x-sum` total 4 |
| 13 | `rank.` records in `content/13/rank/`; the champion aspects as `13.elite_modifier` records; present: the §5.1 fields and rules (doc 13 §2.1) |
| 14 | still requested: `14.achievement` holds at most 40 curated records, with the first-run achievement replacing the Prologue achievements; the long tail as `14.codex_mark` records under `ach.mark.` (DECISIONS.md 137, 141) |
| 16 | present: the economy API schema lives at `palimpsest/schemas/svc/econ_api.json` (doc 16 test 50), a subdirectory schema that stage 1 skips (§1.3) |
| 18 | still requested: the launch locale records of G7 (`18.locale`, `nar.locale.`) |

## Canon gaps

- **G1. No namespace for tooling metadata.** Canon §19 lists no namespace for records generated from the canon itself. This chapter uses `meta.`, owner 00, for `content/00/meta/namespaces.json`, and the generator adds that row. Proposal: Canon §19 adds "`meta.` | 00 (generated tables, doc 19)".
- **G2. Alias storage.** Canon §1.1 puts renames in `data/aliases.json`, but doc 02 §11.1 stores all content under `content/<doc>/<ns>/`. This chapter carries aliases on the renamed record (`aliases`, §1.2); a build step may emit `data/aliases.json` from them. Proposal: Canon §1.1 reads "requires an alias entry (doc 19 §1.2)".
- **G3. Owner cells mix records and rules.** Canon §19 owner cells combine record owners with rule owners ("Echo acquisition in 15", "account economy rules in 15", "owners per §11.1 and §12 for behavior", "with core rank values in 00", "(blocks shared with 03)"). §1.4 fixes how they are read. Proposal: Canon §19 splits the column into record owners and rule owners.
- **G4. Enum values without content IDs.** Canon §1.1 gives every enum value a content ID, while chapters write closed field-local sets as bare words: part slots (doc 07 §7.1), KPI units (doc 01 §10.2), flag kinds (doc 02 §17.3), the axis names of `dep ⊆ {arcana, techne, mythos, verve}` (Canon §5.3). This chapter's rule: a closed set used by one field may be a JSON Schema `enum` of bare words; a value that other records reference, or that saves and packets carry as `id32`, is a content ID. So `dep_set` holds `ax.` IDs while `axiom_vector` keys are bare names. Proposal: Canon §1.1 adopts this rule.
- **G5. Bp width.** Canon §1.2 makes Bp `i32` but power tables `i64` Bp. This chapter defines `bp` as `i32` and `bp_i64` within the JSON bound for table values and `i64` multipliers.
- **G6. Canonical hex spelling.** Canon §1.6 requires `"0x"` hex strings for `u64` values but no single spelling; `palimpsest_data` accepts 1 to 16 digits of either case and writes 16 upper-case digits. Because `content_hash` hashes the record's JSON text (doc 02 §11.2), `"0xff"` and `"0x00000000000000FF"` hash differently. Proposal: Canon §1.6 fixes `0x` plus 16 upper-case digits, and `hex_u64` then narrows to `^0x[0-9A-F]{16}$`.
- **G7. Locales.** Doc 18 §9.11's launch locales are BCP 47 tags such as en-US, which break the ID grammar, and no namespace holds them. The catalog proposes `18.locale` records `nar.locale.<tag>`, the tag lower-cased with `-` written as `_`. Doc 18 decides.
- **G8. Retired IDs outside §17.7.** The canon-1.4 changelog replaces the block category and the Delving skill, but no canon list marks the old IDs retired, so the generator, which reads §17.7 only, does not retire them (the spec linter does). Proposal: Canon §17.7, or a retired-ID list in §1.1, names them.
- **G9. Negative zero.** Canon §1.6 is silent on the integer token `-0`. This chapter rejects it with `E_FLOAT_TOKEN`, matching `palimpsest_data`, whose `serde_json` reads it as `-0.0`. Proposal: Canon §1.6 adds "`-0` is not an integer".
- **G10. Repeated object keys.** Neither the canon nor doc 02 rules on a JSON object that repeats a key; `serde_json` keeps the last value silently. This chapter rejects it (`E_JSON`), and §7 item 5 asks the loader to match. Proposal: Canon §1.6 forbids repeated keys.
- **G11. Schemas for shared namespaces.** Canon §19 gives some namespaces two owners (`ax.` and `genre.`, `block.`, `ev.` and `rng.`, the core values of rows 10 to 12) without saying whose file holds a given record. This chapter keeps one record per ID (§1.2) and places each kind in one owner's directory in §5. Proposal: Canon §19 names, per shared namespace, the document whose files hold the records.
- **G12. Retired IDs of DECISIONS.md 128 and 132.** `gen_namespaces.py` retires only what Canon §17.7 declares, and §17.7 does not yet name the IDs that DECISIONS.md 128 and 132 retire. Until it does, content holding them passes stage 8. Proposal: Canon §17.7 adds the sentence "cat.seed, block.crop and mat.tag.tillable (DECISIONS.md, item 128) and mode.prologue (item 132) are retired IDs and must not appear in content."

## Acceptance tests

1. `python3 tools/validate_content.py --self-test` exits 0: every built-in vector passes, all 27 fixture cases match their `expect.json`, and all 25 codes of §4.2 are expected by at least one case.
2. `python3 tools/validate_content.py` exits 0 on the workspace and reports 0 errors.
3. `python3 tools/gen_namespaces.py --check` exits 0 on the committed table; editing any owner cell of Canon §19 makes it exit 1 until the table is regenerated.
4. **Tokens (doc 02 test 19, Canon §21 test 34).** In a content file, `1.5`, `1e3`, `-0.0`, `-0` and `2E0` each give `E_FLOAT_TOKEN`; `9007199254740992`, `-9007199254740992` and `18446744073709551615` each give `E_INT_RANGE`; `9007199254740991`, `-9007199254740991`, the string `"1.5"` and `"0x00000000000000FF"` in a `hex_u64` field give nothing.
5. **IDs (Canon §21 tests 4 and 29).** `fnv1a32("genre.cyberpunk") = 0x99b54427`. The record IDs `Genre.Cyberpunk`, `genre..x` and `loc.item.sword` give `E_ID_GRAMMAR` with the `palimpsest_data` messages; the first collision pair of `palimpsest_data::test_support` gives `E_ID32_COLLISION`; the same ID in two files gives `E_DUP_ID`.
6. **Sums.** `dmg_shares` summing to 10000 passes and to 9999 gives `E_SUM`; `weapon_scaling` summing to 15000 passes and to 15001 gives `E_SUM_MAX`; an origin tolerance row summing to 3 gives `E_SUM`.
7. **Ownership.** An `item.` record under `content/12/` gives `E_OWNER`; a `rank.` record under `content/00/` passes; a `block.` record under `content/03/` passes; an `enemy.` record under `content/00/` gives `E_OWNER`; a `zzz.` record gives `E_NS_UNKNOWN`; a `dmg.` record in a `st/` directory gives `E_NS_DIR`.
8. **Retired IDs.** Each Canon §17.7 retired ID used as a record ID, an alias or a reference gives `E_RETIRED`; `enemy.role.charger` is not retired.
9. **References.** A missing `genre.` reference gives `E_REF_MISSING`, and with `--allow-missing-prefix genre.` gives `W_REF_MISSING` and exit 0; a reference through an alias gives `W_REF_ALIAS` and exit 0; a `st.` ID in a `ref_genre` field gives `E_REF_PREFIX`.
10. **Patterns.** A `hex_u64` field holding `"0xFF"` followed by a newline gives `E_SCHEMA`.
11. **Schemas.** Every file in `palimpsest/schemas/` passes stage 1; none contains `"type": "number"`; a schema with an unknown `x-` keyword, a wrong `x-owner` or an unresolvable `$ref` gives `E_SCHEMA_DEF`.
12. **Rust mirror (milestone M2).** The `palimpsest_data` equivalence test of §7 item 9 passes, and `pal schema export --check` passes on the committed schemas.
13. **Template loot IDs (§1.2).** A `11.template_loot` record for `arch.melee.spear` has the ID loot.tpl.arch_melee_spear and `template` `arch.melee.spear`; a second record for the same template gives `E_DUP_ID`; a `template` holding an `st.` ID gives `E_REF_PREFIX`.
14. **Patchable lines (§3.6).** The `11.loot_table` schema passes stage 1 with `"x-patchable": true` on `lines`; the same keyword on a non-array subschema gives `E_SCHEMA_DEF`.
15. **Retirements of DECISIONS.md 128 and 132 (§1.4, G12).** Once Canon §17.7 carries the G12 sentence, `gen_namespaces.py --check` fails until the table is regenerated, and a record or reference cat.seed, block.crop, mat.tag.tillable or mode.prologue then gives `E_RETIRED`.
16. **Achievements (§5, DECISIONS.md 141).** `content/14/ach/` holds at most 40 `14.achievement` records; every other account milestone is a `14.codex_mark` record.
17. **Doc 13 fields (§5.1).** A `13.enemy_template` record without `visual` or `body_state.crouch_mm` gives `E_SCHEMA`; `flags.net_always` set to 2 gives `E_SCHEMA`; a `13.ability` record with a negative `knockback_mm_s` gives `E_SCHEMA`; a `visual` that starts with another prefix than `art.visual.` gives `E_REF_PREFIX`.
18. **Service schemas (§1.3).** A file under `palimpsest/schemas/svc/` is not read by stage 1 and no `E_SCHEMA_DEF` names it.

## Invariants annex

```
Σ doc19: files ⊗ envelope ⊗ schema identity ⊗ common defs ⊗ x-keywords ⊗ pipeline ⊗ catalog ⊗ rust mirror
W! file := content/<NN>/<ns>/<name>.json ∧ {schema, records}      N! record without id
W! schema_id := <NN>.<kind> ∧ $id = urn:palimpsest:schema:<id>   W! x-owner(schema) == NN(file)
W! number token ∈ integers ∧ |n| ≤ 2^53 − 1                       N! "type": "number" ∈ schemas
W! ∀ id: palimpsest_data grammar ∧ fnv1a32 unique                N! one id ⇒ two records
W! ∀ ref: exists(record ∨ alias) ∨ allowed prefix → warning       N! canon-listed id accepted without a record
W! owner(record) ∈ owners(longest §19 prefix)                     N! Markdown parsed by the validator
W! namespaces.json := gen_namespaces(canon §19, §17.7)             N! retired id ∈ content
W! Σ shares = 10000 ∧ Σ c_i ≤ 15000 by x-sum, x-sum-max           W! every code ∈ some fixture
W! python verdicts == rust loader verdicts on fixtures            W! schemars output == committed schema
W! id(11.template_loot) == loot.tpl. + dots_to_underscores(template) N! two template_loot records ⇒ one template
W! x-patchable(11.loot_table.lines) == true                       N! stage 1 reads schemas/<subdir>/
W! |14.achievement records| ≤ 40                                  N! knockback stated as a distance
N! seeds ∨ Prologue ∈ catalog                                      W! retired(128, 132) ∈ §17.7 ⇒ E_RETIRED
```
