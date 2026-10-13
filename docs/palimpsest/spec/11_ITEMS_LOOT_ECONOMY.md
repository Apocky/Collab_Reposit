# 11 — Items, Loot, Reliquaries, Fortune Engines and Economy
As of 2026-10-12. Status: BINDING. Written against canon-1.12 (adversarial check pending). DECISIONS coverage: through row 145.

This chapter defines item templates and instances, the deterministic loot generator, genre skins for every weapon archetype, manufacturers, affixes, perks, Legendary perks, Exotics and Palimpsestic uniques, every loot source (kills, chests, in-run Reliquaries, bosses, secrets, Fortune Engines, shops), the Lucre economy, the inventory, Rewriting, Emendations, reforging, salvage, co-op loot and the Item Power score. Archetype mechanics stay with their owners (Canon §19): doc 05 owns `arch.*`, `ammo.` and `counter.`; doc 06 owns spells, staves, foci and tomes; doc 08 owns `alch.` potions and bombs and `apparatus.` vessels and respirators; doc 04 owns `aug.` implants; doc 07 owns `gadget.` and `part.`. This chapter attaches loot-facing records (value, weight, deck card, stack, manufacturer class) to those IDs and mints new IDs only in its own namespaces.

## Imports

| Source | Section | What this chapter consumes |
|---|---|---|
| Canon | §1.1, §1.10 | ID grammar, `id32`, mod IDs with second segment `x_<mod_id>`, skin text keys `loc.<id>.skin.<suffix>.name`, name resolution order |
| Canon | §1.3, §1.5, §1.6 | `floor_div`, `ceil_div`, `mul_bp`, left-to-right Bp chains; display-unit rule; integer-only content |
| Canon | §1.11 | item `uid` layout, per-Folio counters, ascending-uid iteration |
| Canon | §2.3, §2.5 | "N s" = N × 60 ticks; rate accumulator; save contract (every counter in this chapter is saved) |
| Canon | §3.1–§3.5 | `Rng`, `range`, `weighted`, `roll_bp`, `stream_seed`, sequential `rng.loot_place` and its labeled sub-streams (§3.2), keyed `rng.loot`, `rng.combat`, `rng.status`, commit-only `rng.offer.p<slot>` and its offer generators (§3.5) |
| Canon | §4, §4.3 | `GROUND_ITEM_CAP = 2048` and the Lost Satchel hand-off; fixed terrain, blocks tagged `block.tag.destructible`, projectiles stopped by fixed terrain; no item durability |
| Canon | §5.2–§5.11 | genre table, axiom-0 generation ban, `Req`, `Tol`, `gear_tol`, Contradiction, Anachronism, local genre, seam zones, the §5.9 manufacturers, World Laws, `st.unmoored` |
| Canon | §6.4 rows 10, 13, 14, 19, 37, 38, 42 | carry capacity, gear crit terms, shield stability, buy and sell multipliers, Serendipity |
| Canon | §10.4–§10.6, §10.9 | source magnitude chain, additive bucket, reflected and environmental entry points, healing |
| Canon | §11.2, §11.4 | status power, direct application |
| Canon | §13.1–§13.9 | categories, slots, rarity table and roll, quality, knowledge, curses and pact curses, Emendations and `ENH_CAP`, `P`, `T`, `S`, `ITEM_MULT_MAX`, instance contract |
| Canon | §14 | `cur.lucre`: per Erratum, passing between party members only as dropped piles |
| Canon | §16.4, §16.5 | affliction cure price, sacrifice value |
| Canon | §17.1–§17.9 | run constants, Colophon, Abridged loot quantity, party tables, ranks, champion counters, reference anchors, `mode.loot_shared`, ghost item loss, invasions; the §17.7 retired IDs |
| Canon | §18 | `ev.entity_killed`, `ev.item_picked_up`, `ev.item_dropped`, `ev.item_used`, `ev.container_opened`, `ev.engine_spun`, `ev.folio_entered`, `ev.folio_exited`, `ev.invasion_ended` |
| Canon | §19 | this chapter owns `item.`, `rar.`, `affix.`, `perk.`, `mfr.`, `loot.`, `reliq.`, `engine.`, `shop.`, `cat.`, `slot.` |
| DECISIONS | 55, 60, 65 to 68, 83, 90, 95, 102, 106, 128 | no mounts; cursed capsule; no item durability; backpack, Scriptorium cost and offers, perk columns; Lucre between players; the Crowns skin; the grappling tool item; Second Edition carry-over and depth `25 + 2n`; seeds cut |
| Doc 02 | intents | `ShopBuy`, `ShopSell`, `CommitOffer`, `DropItem`, `UseItemOn`, `EquipItem`, `MoveItem`, `ShopHaggle { shop }` and `DropLucre { amount: i64 }` |
| Doc 03 | §5.19, §5.24 | `Projectile.home_bam_s` and `home_acc` (homing); `start_grapple` and the grapple mode |
| Doc 04 | §2.5, §8.4, §9.3–§9.5, §11.2 | kit roles; treatments (bandage, splint, salve, surgical kit); implant-form `aug.*` augments; "Emend the Flesh" price `mul_bp(400, P(d))`; food values |
| Doc 05 | §3.2, §5.2, §8, §9.3, §11.1, §12.1, §15 | `arch.melee.*`, `arch.ranged.*`, `arch.armor.*`, `arch.offhand.*` and `arch.ward.*` records; ammunition families, carry caps, pickup sizes and the §8.3 need weights; grenade archetypes; counter bits, counter charges and delivery channels; ward capacity chain and depletion profiles |
| Doc 06 | §3.8, §8.1, §8.3, §8.4, §8.5 | `arch.ranged.staff`, `arch.offhand.tome`, `arch.offhand.focus` casting fields; texts and inscription; trainer price; wand charges by rarity; bound-glyph list |
| Doc 08 | §4.3, §6.1, §12.1, §12.2, §13.2 | `apparatus.respirator.*` and `apparatus.vessel.*`; the 16 found `alch.potion.*` types and their values; the 12 Reliquary-only potions; `alch.bomb.*` |
| Doc 10 | §2.1, §3.2, §4, §5.9, §6, §7.1, §7.8, §8, §9, §10, §11 | knowledge flags and mundane categories; `idc` budgets; `wand_pool` spells; `appr.svc.*` prices; `value_true`, `value_vendor`; `roll_curse` (3 draws), its pacts and its `CursedCapsule` source; the 3000 Bp potion and 1000 Bp text malign shares; Unread Edge SER; vendor rules; sealing |
| Doc 12 | §6, §13, §20 | Folio types (standard, challenge, rest, boss), the rest Folio's shop and Engine hall, challenge success rewards, room and container spots; the story relic spot, the story's 2 drawn domains and its solve; `Key(k)` nodes and `Locked(keys)` edges |
| Doc 09 | §11.1, §13.1 | outfits (`outfit_of`); lock kinds, `key_tags` and `TOOL_Q` by tool quality |
| Doc 13 | §2.1, §5.1, §9, §12.4, §13 | `loot_role`, `no_loot`, ranks; carried items; boss loot draws and `palimpsestic_bp`; war spoils; vendor types, reputation price factor, quests and companion rewards |
| Doc 14 | §2.4, §3.1, §3.9, §5.3, §7.1, §7.2, §8, §8.9 | kits; Folio positions and depth `d(f)`; Colophon, Interleaf, Endpaper and the Volume reward; Bindery pool tags; `RunState.grade`, `GradeParams.lucre_bp`, `shop_price_bp`, `reliq_key_delta`; boons; modes (Standard, Novella, Abridged, Daily, Endless, Second Edition) |
| Doc 15 | §5.4, §5.5 | `generate_keepsake` call contract, Satchel materialization |
| Doc 16 | §6.6, §9.5, §9.6 | slot states Active, Dormant, Ghost and Departed; `DropItemFor`; ghost item-loss draw |
| Docs 03, 07, 09, 17, 18 | see Interface assumptions | ground-item entities, materials and parts, locks and mimic detection, UI, localization |

---

## 1. Principles

1. **Template and instance.** A template is immutable content. An instance is simulation state created from a template by the generator in §2.3, and it never changes template except through Rewrite (§15), which keeps the template and changes the skin.
2. **One generator.** Every random item that enters a run (kills, containers, Reliquaries, Fortune Engines, shops, bags, Volume rewards, quests, Keepsakes) is produced by `generate_item` (§2.3). Kits (doc 14 §2.4), the fixed items of the Volume plan (§9.9) and NPC carried items (doc 13 §5.1) are created directly from a named template and draw nothing from `rng.loot`.
3. **Keyed and isolated.** Every random step of item generation draws from its own sub-generator derived from one `u64` item seed. Adding a draw to one step never shifts another step.
4. **Earned randomness.** Every randomized container here is paid for with in-run Lucre or in-run keys only. Odds are displayed before every commitment. Fortune Engines always pay out an item.
5. **Horizontal unlocks.** A Bindery pool tag (`loot.pool.<suffix>`) adds templates inside their rarity band and category only; weights renormalize within the band, so rarity shares never change (doc 15 Script neutrality rule).
6. **Owners' IDs.** This chapter never mints an ID in another owner's namespace (Canon §19). It attaches a loot record to the owner's ID and never defines a second record under it (doc 19 §1.2: one ID, one record).
7. **Placed loot is isolated.** Everything generated at Folio generation (shop stock, floor loot, placed caches, Reliquary and Engine placement) depends only on the run seed, the Folio index, the Folio's genre, the mode, the Grade and the run's content set, never on party size, earlier play or player state.
8. **Mod segments.** In this chapter's namespaces a second segment beginning with `x_` belongs to a mod (Canon §1.1: `x_<mod_id>`). This chapter defines no such ID. A mod item is a §2.1 template record with its own category, `template_weight` and `pool`; a mod patch of a loot table appends to that table's `lines` (§9.5 record), the only list a table has, which the schema marks `"x-patchable": true` (doc 02 §13.1, doc 19 §3.6).
9. **No wear.** Items have no durability (Canon §4.3, DECISIONS 65): nothing in this chapter wears, breaks or repairs an item. Block HP of placeables and destructible blocks is doc 07's.

---

## 2. Items: templates, instances and generation

### 2.1 Template record

Every template is either an `item.` record owned here or an owner's ID listed in §3: doc 05 `arch.*` and `ammo.*`, doc 06 `arch.ranged.staff` and `arch.offhand.tome`, doc 08 `alch.potion.*`, `alch.bomb.*`, `apparatus.vessel.*` and `apparatus.respirator.*`, doc 04 implant-form `aug.*`, doc 07 `gadget.*` and `part.*`. For an owner's ID this chapter keeps a loot record with the fields below that references the owner's ID; the owner's record holds the mechanics. Doc 19 owns the serialized schema and stores these as `11.template_loot` records, one per template, so no ID gets a second record (doc 19 §1.2). A `11.template_loot` record has no ID of its own: it is keyed by the owner's template ID in its `template` field, which the schema marks `x-idref` to the owner's record, so the loot records define no ID and use no `loot.tpl.` prefix.

```json
{
  "id": "item.ring.keen",
  "category": "cat.accessory",
  "slot": "slot.ring_l",
  "idc": "idc.ring",
  "base_value": 70,
  "mass_g": 20,
  "stack_max": 1,
  "rarity_eligible": true,
  "deck_card": "cat.accessory",
  "template_weight": 100,
  "pool": "base",
  "skin_dep": "none",
  "implicit": [ { "stat": "crit_chance_bp", "base": 200, "per_enh": 40 } ]
}
```

| Field | Rule |
|---|---|
| `base_value` | Lucre at ilvl 1, Common, Normal, +0 (§13.1). Where the owner states a value (doc 05 ammunition, doc 08 potions), this chapter copies it. |
| `stack_max` | 1 for every equipment item except thrown weapons (20); §3.5 for stackables |
| `rarity_eligible` | true for weapons, wands, armor, off-hands, ward generators, implants and accessories; false otherwise (always `rar.common`) |
| `template_weight` | weight inside its category for the template draw (§2.3 step 2) |
| `pool` | `base` (in `unlock.set.starter`) or a `loot.pool.<suffix>` tag gated by a doc 14 Bindery node |
| `implicit` | the template's fixed effect; `value = base + per_enh × eff_enh` (Canon §13.7 `eff_enh`) |

### 2.2 Instance record

The canon fields of §13.9 are mandatory. This chapter adds the fields marked "doc 11" and rolls the doc 06 fields marked so. Doc 19 owns the serialized schema.

| Field | Type | Owner | Meaning |
|---|---|---|---|
| canon §13.9 fields | | canon | `uid`, `template`, `origin_genre`, `rarity`, `quality`, `ilvl`, `enhancement`, `buc`, `curse_penalty`, `knowledge`, `affixes`, `parts`, `materials`, `charges`, `stack`, `mass_g`, `value`, `manufacturer`, `gear_tier`, `perk_columns`, `illuminated`, `finder_slot` |
| `genre_trait` | perk ID or none | doc 11 | the column-5 genre trait (§7.2); weapons only |
| `legendary_perk` | perk ID or none | doc 11 | present iff rarity is Legendary (§8.1) |
| `exotic` | bool | doc 11 | true iff `template` is an `item.exotic.*` |
| `bound_glyph` | `glyph.mod.*` or none | doc 06, rolled here | Legendary and Exotic staves, foci and tomes (§2.3 step 11b) |
| `spell` | `spell.*` or none | doc 06 and 10, rolled here | wands only: the held spell, which is also the wand's type key (doc 10 §4.2) |
| `reforge_count` | u8 `0..=4` | doc 11 | §16.3 |
| `rewrite_count` | u16 | doc 11 | number of committed Rewrites (§15) |
| `tuning` | `{plus: sub, minus: sub}`, `lowest3` or none | doc 11 | Tier-5 armor and accessory tuning (§7.3) |
| `visibility` | `Instanced(slot)` or `Public` | doc 11 | ground visibility (§14.4) |
| `gloss_mfr` | `mfr.` ID or none | doc 11 | manufacturer whose gimmick `affix.gloss` grants (§6.3) |
| `sealed` | bool | doc 11 (rules doc 10 §11) | true for Reliquary, Engine, Volume-reward and Keepsake contents until first inspected; UI only |
| `illum_pending` | bool | doc 11 | true for a Legendary-or-better item from a Placed or Offer source until it first enters an inventory (§2.3 step 13) |
| `fresh` | bool | doc 11 | `item.lucre` piles only: true until the first pickup applies the Lucre-find multipliers (§9.1) |
| `teaches` | list of `glyph.*`, one `recipe.*`, or none | doc 11 (rules doc 06 §8.1, doc 07 §6.5) | `cat.book` only: a glyph tome's 1 to 3 glyphs, or a schematic's one recipe (§3.5 Books) |
| `zaps_total` | u32 | doc 10 §7.8, stored here | wands only: zaps made with this wand since creation, starting at 0; `curse.pact.overcharge` reads it |
| `dropped_by` | slot or none | doc 11 | `item.lucre` piles made by `DropLucre`: the dropper's slot, whose auto-pickup skips the pile (§17) |
| `ready_tick` | u32 | doc 11 | `item.tool.grapple` only: the first tick at which a use is accepted (§3.5); 0 at creation |
| `key_tag` | u16 | doc 11 (rules doc 09 §13.1) | `item.key.door` only: doc 12's key index `k` of the `Locked` edge it opens |
| `outfit_of` | `fac.*` or none | doc 11 (rules doc 09 §11.1) | `slot.body` armor only: the faction this outfit impersonates; set only by doc 09's stripping and by faction lockers and shops |
| `story_domains` | `[u8; 2]` or none | doc 11 (rules doc 12 §20) | the story relic only (§9.6): indices into the Folio genre's 4 `story_domains`, in draw order |
| `keepsake` | bool | doc 11 (rules doc 15 §5.4, doc 16 §9.6) | true for an item created by `generate_keepsake` (§2.3); `DropItemFor` refuses it (§14.4) |

`affixes` entries are `{id, tier: 1..=5, roll_bp: 0..=10000}`. The value of an affix is `lo + floor_div((hi - lo) × roll_bp, 10000)` with `(lo, hi)` from its tier row in §6.2; a descending row (`lo > hi`, `affix.self_injector`) uses the same formula and yields a smaller number for a higher roll.

**Example instance** (the §20 worked example after generation):

```json
{
  "uid": "0x000007000000019c", "template": "arch.ranged.smg", "origin_genre": "genre.weird_west",
  "rarity": "rar.epic", "quality": "q.excellent", "ilvl": 7, "enhancement": 0, "buc": "buc.uncursed",
  "curse_penalty": 0, "knowledge": { "state": "know.classified", "flags": 0 },
  "affixes": [ { "id": "affix.self_injector", "tier": 2, "roll_bp": 4095 },
               { "id": "affix.infuse_fire", "tier": 2, "roll_bp": 4630 },
               { "id": "affix.dmg_vs_beast", "tier": 2, "roll_bp": 1146 } ],
  "parts": [], "materials": ["mat.hexed_silver"], "charges": { "cur": 0, "max": 0 },
  "stack": 1, "mass_g": 0, "value": 1382, "manufacturer": "mfr.weird_west.sundown_rail",
  "gear_tier": 3,
  "perk_columns": [
    { "column": 1, "options": ["perk.choke", "perk.rifled_bore"], "chosen": 0 },
    { "column": 2, "options": ["perk.drum_feed", "perk.tempered"], "chosen": 0 },
    { "column": 3, "options": ["perk.kill_clip", "perk.unrelenting"], "chosen": 0 },
    { "column": 4, "options": ["perk.headhunter", "perk.frenzy"], "chosen": 0 },
    { "column": 5, "options": ["perk.genre.weird_west"], "chosen": 0 } ],
  "genre_trait": "perk.genre.weird_west", "legendary_perk": null, "exotic": false,
  "bound_glyph": null, "spell": null,
  "illuminated": false, "finder_slot": 1, "reforge_count": 0, "rewrite_count": 0,
  "tuning": null, "visibility": "Instanced(1)", "gloss_mfr": null, "sealed": false,
  "illum_pending": false, "fresh": false
}
```

`mass_g` is filled by doc 07 from archetype volume and material (shown as 0 here because doc 07 values are not part of this chapter). `uid` `0x000007000000019c` is `(7 << 40) | 412` (§20).

### 2.3 Generation pipeline

**Seeds.** A loot event creates one drop generator, then one `u64` item seed per potential item:

```rust
fn drop_rng(folio: u32, actor: u64, tick: u64, slot: u8) -> Rng {
    keyed_rng("rng.loot", folio, actor, tick, slot as u32)          // Canon §3.3: actor = killed entity or container uid
}
fn sub(item_seed: u64, tag: &str) -> Rng {                          // one isolated generator per step
    Rng::from_seed64(mix64(item_seed ^ fnv1a64(tag)))
}
```

The tags are hash inputs: no record carries them and they have no `id32` in saves or packets. They are reserved names in the `loot.` namespace, so no loot table may take one. Every sub-generator derived from a keyed `rng.loot` seed counts as `rng.loot` (Canon §3.3).

| Tag | Used by | Draws |
|---|---|---|
| `loot.cat` | §2.3 step 1, category | 1 `weighted` per attempt |
| `loot.arch` | step 2, template; loose-part template (§5.4); counter charge of the ammo card and bundle potion list (§3.5, §10); glyph-tome school (§3.5 Books) | 1 `weighted` (bundle potions 2) |
| `loot.rarity` | step 3 | 1 `weighted` or 0 |
| `loot.exotic` | step 3b | 1 `weighted` or 0 |
| `loot.mfr` | step 5; part manufacturer (§5.4) | 1 `weighted` or 0 |
| `loot.quality` | step 6 | 1 `weighted` or 0 |
| `loot.material` | step 7 | 1 `weighted` or 0 |
| `loot.affix` | step 8; Gloss manufacturer (§6.3) | 3 per affix, 4 for `affix.gloss` |
| `loot.perk` | step 9; crafted weapons (doc 07 §4.4) | `n_opt` per rolled column |
| `loot.legendary` | step 11 | 1 `weighted` or 0 |
| `loot.glyph` | step 11b | 1 `weighted` or 0 |
| `loot.spell` | step 11c | 1 `weighted` or 0 |
| `loot.buc` | step 12, doc 10 `roll_curse` | 3 or 0 |
| `loot.illum` | step 13 | 1 `roll_bp` or 0 |
| `loot.book` | `teaches` of a generated book (§3.5 Books) | glyph tome: doc 06 `glyph_tome` (at most 2); schematic: 1 `weighted` |
| `loot.lucre` | Lucre line amount (§9.1) | 1 |
| `loot.ammo` | ammunition line family (§3.5) | 1 `weighted` or 0 |
| `loot.key` | mini-boss key line, gilded or bound (§9.5) | 1 `weighted` |
| `loot.bundle` | Reliquary consumable bundle (§10) | 4 |

Placed sources (shop stock, floor loot, caches) take the item seed from `place_rng` (§9.11), a labeled generator of `rng.loot_place`; Offer sources (Engine faces, Volume rewards) take it from `offer_rng` (§11.2). The steps below are identical for every source.

**Source kinds.** `Keyed` (kills, containers, Reliquaries, bags, quests, secrets, Engine dispensing), `Placed` (generated at Folio generation), `Offer` (generated from `rng.offer.p<slot>`) and `Keepsake` (doc 15).

**Drop resolution** (fixed draw count: 2 per line):

```rust
fn roll_drop(table: &LootTable, src: &DropSource, slot: u8, ctx: &mut LootCtx) -> Vec<ItemInstance> {
    let mut d = drop_rng(ctx.folio, src.actor, ctx.tick, slot);
    let mut out = Vec::new();
    for line in &table.lines {                                      // table order (§9.5)
        let chance = line_chance(line, src, ctx);                   // §9.1 multipliers, clamped 0..=10000
        let hit = d.roll_bp(chance);                                // draw 1 of the line, always made
        let seed = d.next_u64();                                    // draw 2 of the line, always made
        if hit { out.extend(resolve_line(line, seed, slot, ctx)); }
    }
    out
}
```

**Item generation.** `generate_item(seed, req)`; `req` carries the category or a deck mode (`Decrement` over the player's deck, or `Fixed(cards)` with the §9.2 counts as fixed weights), an optional fixed template, origin genre, `ilvl`, rarity rules (forced rarity, floor, shift), receiving slot and effective SER, role bias, source kind and the doc 10 curse source.

| Step | Sub-generator tag | Draws | Rule |
|---|---|---|---|
| 1 Category | `loot.cat` | 1 `weighted` per attempt (0 if fixed) | If `req.category` is fixed, skip. `Decrement`: weights `remaining_count × role_bias_bp` (§9.2, §9.3) in §9.2 card order. `Fixed(cards)`: the listed cards' §9.2 counts × role bias. If the drawn category has no eligible template (step 2), that card's weight becomes 0 and the same sub-generator draws again; with every weight 0 the request yields nothing. `Decrement` decrements only the card finally used and refills the deck when every count is 0. |
| 2 Template | `loot.arch` | 1 `weighted` (2 for Reliquary bundle potions, §10) | Eligible: category matches; in the run content set; pool tag owned; a skin exists in `origin_genre` (§4; `item.` templates and owners' consumables have one in every genre); no axis of its skin dependency has axiom 0 there (Canon §5.3); not `item.exotic.*` or `item.palimpsestic.*`. Candidates in ascending ID order with §3 template weights. `cat.part` draws its manufacturer first (§5.4); `cat.ammo` follows §3.5. |
| 3 Rarity | `loot.rarity` | 1 `weighted` (0 if forced or not rarity-eligible) | Canon §13.4 weights with the receiving player's effective SER (doc 10 §9 Unread Edge included; 40 while `st.unmoored`, Canon §5.11; 40 for Placed and Offer sources), then the source shift `w[r] = floor_div(w[r] × (10000 + shift_bp × r), 10000)` for `r >= 1`, then the bad-luck multipliers (§9.4; Keyed generic lines only), then the floor (weights below it set to 0). Placed sources set the Exotic weight to 0. Palimpsestic weight stays 0 except in sources that force it. |
| 3b Unique template | `loot.exotic` | 1 `weighted` (iff Exotic or Palimpsestic) | Exotic: an Exotic of the same category (§8.2) whose pool tag is owned and that the receiving player has not acquired this run, equal weights 100, ascending ID; when every one is acquired, all become eligible again; no Exotic of that category in the content set → rarity becomes Legendary, no draw. Palimpsestic: the §8.3 uniques no party member has acquired this run; none left → Legendary, no draw. |
| 4 Origin genre | none | 0 | The local genre at the source position (Canon §5.8); in a seam zone the Folio's genre (Canon §5.8); an Exotic takes its home genre (§8.2). |
| 5 Manufacturer | `loot.mfr` | 1 `weighted` (0 if none applies) | §5.3. |
| 6 Quality | `loot.quality` | 1 `weighted` | Weights `[300, 1200, 6000, 1800, 500, 170, 30]` over `q.awful..q.peerless`, then the source quality floor (weights below it set to 0). Mundane categories (doc 10 §2.1), potions and texts: `q.normal`, no draw. |
| 7 Material | `loot.material` | 1 `weighted` (0 without a material role) | Doc 07 pool for (`origin_genre`, archetype material role), ascending ID, each entry weighted. |
| 8 Affixes | `loot.affix` | 3 per affix (4 for `affix.gloss`) | For `i in 0..NAFF[rarity]`: pick an affix (`weighted` over eligible affixes sorted by ID, §6.1), pick a tier (`weighted` over unlocked tiers), `roll_bp = range(10001)`. Stop early when no affix is eligible. |
| 9 Perk columns | `loot.perk` | `n_opt` per rolled column | Weapons and wands only. Columns 1 to 4 present by rarity (§7.1); for each, `n_opt(gear_tier)` weighted picks without replacement; column 4 excludes column 3's options. |
| 10 Genre trait | none | 0 | Weapons and wands only: `perk.genre.<origin suffix>` in column 5. |
| 11 Legendary perk | `loot.legendary` | 1 `weighted` (iff Legendary) | §8.1 pool for the category group. |
| 11b Bound glyph | `loot.glyph` | 1 `weighted` (iff Legendary or Exotic staff, focus or tome) | Doc 06 §8.5 list `glyph.mod.{anchor, empower, extend, heavy, hushed, keen, reach, thrift}`, ascending ID, weight 100 each. |
| 11c Wand spell | `loot.spell` | 1 `weighted` (wands only) | Doc 10 `wand_pool` spells whose every glyph dependency is legal in `origin_genre` (Canon §5.3), ascending ID, weight 100 each; stored in `spell`. |
| 12 Curse state | `loot.buc` | 3 (doc 10 §7.1) | `doc10::roll_curse(rng, draft, local_myt, source)` for categories whose doc 10 §2.1 row has BUC. Shop stock, Engine faces other than the cursed capsule, Volume rewards and quest rewards skip it (`buc.uncursed`, no draw); the cursed capsule (§11.2) calls it with source `CursedCapsule`, which always gives a pact (doc 10 §7.8). Reliquary contents and Editor tables call it with source factor 0 (doc 10 §7.1), so they can be blessed but never cursed. |
| 13 Illuminated | `loot.illum` | 1 `roll_bp(5)` (iff Legendary or better and Keyed or Keepsake) | Canon §13.9. Placed and Offer items set `illum_pending = true` instead; when such an item first enters an inventory it rolls `keyed_rng("rng.loot", folio, uid, tick, slot).roll_bp(5)` once and clears the flag. |
| 14 Derived fields | none | 0 | `uid` from the Folio item counter; `gear_tier = min(5, 1 + floor_div(run.grade, 5))`; `finder_slot = req.slot`; `knowledge` and `sealed` (§2.4); `charges` (wands: doc 06 §8.4 maximum by rarity, full; gadgets: doc 07); `value` (§13.1); `mass_g` from doc 07; `stack` (§3). |

Steps 1 to 3b select the template and run first; the remaining steps may run in any order because each uses its own sub-generator.

```rust
fn generate_item(seed: u64, req: &GenRequest, st: &mut PlayerLootState, run: &RunState) -> Option<ItemInstance> {
    let (cat, tmpl) = pick_category_and_template(seed, req, st, run)?;        // steps 1, 2
    let mut rar = if !tmpl.rarity_eligible { RAR_COMMON }
                  else if let Some(r) = req.forced_rarity { r }
                  else { pick_rarity(req, st, &mut sub(seed, "loot.rarity")) };  // step 3
    let tmpl  = unique_template(tmpl, &mut rar, req, st, run, &mut sub(seed, "loot.exotic"));  // step 3b
    let genre = if tmpl.is_exotic() { tmpl.home_genre } else { req.origin_genre };             // step 4
    let mfr   = pick_manufacturer(&tmpl, genre, rar, &mut sub(seed, "loot.mfr"));
    let q     = pick_quality(&tmpl, req.quality_floor, &mut sub(seed, "loot.quality"));
    let mat   = doc07::pick_material(genre, &tmpl, &mut sub(seed, "loot.material"));
    let mut it = ItemInstance::new(run.next_uid(), &tmpl, genre, rar, q, req.ilvl, mfr, mat, req.slot);
    roll_affixes(&mut it, &mut sub(seed, "loot.affix"));
    if it.has_perk_columns() { roll_perks(&mut it, run.grade, &mut sub(seed, "loot.perk")); it.genre_trait = genre_trait_for(&it); }
    if rar == RAR_LEGENDARY { it.legendary_perk = Some(pick_legendary(&it, &mut sub(seed, "loot.legendary"))); }
    if it.is_implement() && (rar == RAR_LEGENDARY || rar == RAR_EXOTIC) { it.bound_glyph = Some(pick_glyph(&mut sub(seed, "loot.glyph"))); }
    if it.is_wand() { it.spell = Some(pick_wand_spell(genre, &mut sub(seed, "loot.spell"))); }
    if req.source.rolls_curse() && it.bears_buc() {                 // false for shop, Engine-face, Volume-reward and quest sources, except the cursed capsule (§11.2)
        doc10::roll_curse(&mut sub(seed, "loot.buc"), &mut it, local_myt(req), req.source);
    }
    if rar >= RAR_LEGENDARY {
        if req.source.is_keyed_or_keepsake() { it.illuminated = sub(seed, "loot.illum").roll_bp(5); } else { it.illum_pending = true; }
    }
    it.finalize_derived(run, req);    // gear_tier, knowledge, sealed, charges, value, mass, stack
    if req.uses_blp() { st.blp_update(rar, it.is_equipment()); }     // Keyed generic lines only (§9.4)
    if req.deck_mode == DeckMode::Decrement { st.deck.decrement(cat); }
    Some(it)
}
```

**Keepsake entry point** (doc 15 §5.4): `generate_keepsake(genre, rarity, rng)` draws `seed = rng.next_u64()` (1 draw) and calls `generate_item` with table `loot.keepsake.<genre_suffix>`, source `Keepsake`, forced rarity, `ilvl = 1`, `origin_genre = genre`, SER 40, `Fixed` equipment cards (§9.2), `run.grade = 0` for `gear_tier`, no bad-luck state, and the curse roll with the source genre's MYTHOS (doc 10 §7.1). The result is sealed (doc 10 §11) and has `keepsake = true`. Step 2's eligible templates come from the opener's account content set at open time (doc 15 §5.4), never from a run's content set.

### 2.4 Knowledge and sealing at creation

| Source | Knowledge at creation | `sealed` |
|---|---|---|
| kill drops, chests, floor loot, bags, secrets, war spoils | every flag clear; doc 10 §2.4 derives `know.unknown` or `know.classified` | false |
| quest and companion rewards | RARITY set | false |
| shop stock | equipment: every required flag set (`know.identified`); type-hidden consumables: RARITY and BUC set, type knowledge from the party table (doc 10 §10.1) | false |
| Reliquary contents, Engine faces and capsules, Volume rewards, Keepsakes | RARITY set; a cursed capsule also has OMEN set (doc 10 §7.8) | true |
| mundane categories (doc 10 §2.1) | `know.classified`, terminal | false |
| kit items (doc 14) | every required flag set; type added to the party table | false |

### 2.5 Pool tags

A pool tag gates templates and content by Bindery node (doc 14 §5.3 Shelf B). Everything not gated is in pool `base` (`unlock.set.starter`). The run content set holds a tag iff the host owns its node; Daily and weekly runs hold none (Canon §13.6).

| Tag | Bindery node | Adds | Where |
|---|---|---|---|
| `loot.pool.exotic_arms` | `unlock.loot.exotic_arms` | 12 Exotic weapons | §8.2 |
| `loot.pool.exotic_armor` | `unlock.loot.exotic_armor` | 8 Exotic armor pieces | §8.2 |
| `loot.pool.legendary_perks_1` | `unlock.loot.legendary_perks_1` | 10 Legendary perks | §8.1 |
| `loot.pool.legendary_perks_2` | `unlock.loot.legendary_perks_2` | 10 Legendary perks | §8.1 |
| `loot.pool.cross_glosses` | `unlock.loot.cross_glosses` | `affix.gloss` | §6.2, §6.3 |
| `loot.pool.implants_2` | `unlock.loot.implants_2` | 8 implant-form augments: `aug.adrenal_pump`, `aug.extra_arm`, `aug.neural_jack`, `aug.reactive_skin`, `aug.smartlink`, `aug.spring_tendons`, `aug.subdermal_ward`, `aug.wings` (doc 04 §9.4); the other implant-form augments are pool `base` | §3.4 |
| `loot.pool.relics_2` | `unlock.loot.relics_2` | 10 trinkets | §3.4 |
| `loot.pool.throwables_2` | `unlock.loot.throwables_2` | doc 08's `alch.bomb.{blasting, incense, infernal, sleep, stink, tar}` | §3.5 |
| `loot.pool.glyphs_2` | `unlock.loot.glyphs_2` | 8 glyphs that glyph tomes, trainers and text study can teach (doc 06 §8.1) | §3.5 Books, §12.3 |
| `loot.pool.recipes_2` | `unlock.loot.recipes_2` | the 12 recipes doc 07 flags `r2` and the schematics that teach them | §3.5 Books |
| `loot.pool.brews_2` | `unlock.loot.brews_2` | 8 brewing recipes and 4 ingredients (doc 08); the ingredients enter apothecary ingredient lines | §12.1 |
| `loot.pool.gadgets_2` | `unlock.loot.gadgets_2` | 6 gadgets (doc 07 §11.2) | §3.6 |

Pool unlocks are horizontal (§1 principle 5): a tag adds templates inside their category and rarity band and never changes a rarity share.

---

## 3. Archetype master list

### 3.1 Categories and loot roles

| Category | Templates (owner of mechanics) | Rarity | Deck card | Stack | Manufacturer |
|---|---|---|---|---|---|
| `cat.weapon_melee` | 14 `arch.melee.*` (05); `arch.melee.unarmed` is a body function, never an item | yes | `cat.weapon_melee` | 1 | yes |
| `cat.weapon_ranged` | 12 `arch.ranged.*` weapons (05) and `arch.ranged.staff` (06) | yes | `cat.weapon_ranged` | 1; `arch.ranged.thrown` 20 | yes |
| `cat.wand` | `arch.ranged.wand` (06, 10) | yes | `cat.wand` | 1 | yes |
| `cat.armor` | 20 `arch.armor.<slot>_<class>` (05) | yes | `cat.armor` | 1 | yes |
| `cat.offhand` | `arch.offhand.{shield_buckler, shield_heater, shield_tower, lantern, focus}` (05), `arch.offhand.tome` (06) | yes | `cat.offhand` | 1 | yes |
| `cat.ward_gen` | `arch.ward.{aegis, kinetic, reflector, bulwark}` (05) | yes | `cat.ward_gen` | 1 | yes |
| `cat.implant` | implant-form `aug.*` (04) | yes | `cat.implant` | 1 | no |
| `cat.accessory` | `item.ring.*`, `item.amulet.*`, `item.trinket.*`, `item.belt.*`, `item.back.*`, `item.face.*` (11); `apparatus.respirator.*` (08, §3.4) | yes | `cat.accessory` | 1 | no |
| `cat.potion` | `alch.potion.*` (08) | no | `cat.potion` | 10 | no |
| `cat.text` | `item.text.*` (11; effects 06, 10) | no | `cat.text` | 10 | no |
| `cat.throwable` | `arch.ranged.grenade_{frag, incendiary, concussion, flash, smoke}` (05), `item.throwable.counter_*` (11, on `arch.ranged.grenade_counter`), `alch.bomb.*` (08) | no | `cat.throwable` | 10 | no |
| `cat.ammo` | `ammo.*` (05) | no | `cat.ammo` | ammunition reserve (§14.1) | no |
| `cat.gadget` | `gadget.*` (07) | no | `cat.gadget` | 3 | no |
| `cat.part` | `part.*` (07) | no | `cat.part` | 5 | yes |
| `cat.food` | `item.food.*` (11; values 04) | no | none (§3.5) | 20 | no |
| `cat.tool` | `item.tool.*` (11; effects 04, 06, 07, 09) | no | none | §3.5 | no |
| `cat.book` | `item.book.*` (11; glyphs 06, recipes 07) | no | none (§3.5 Books) | 1 | no |
| `cat.container` | `apparatus.vessel.*` (08), `item.bag.*` (11) | no | none | §3.5 | no |
| `cat.key` | `item.key.*` (11) | no | none | keyring | no |
| `cat.currency` | `item.lucre` (11) | no | none | purse | no |
| `cat.quest` | `item.quest.*` (11; placed by 12, 13, 14) | no | none | 1 | no |
| `cat.corpse` | `item.remains` (11; rules 14, 16) | no | none | 1 | no |
| `cat.material`, `cat.ingredient`, `cat.body_part`, `cat.placeable` | owners 07, 08, 04 | no | none (`loot.trophy`, shops, salvage) | owner | no |

Seeds are cut (DECISIONS 128): the category cat.seed is retired, so no template, loot line, deck card, shop line or salvage yield names a seed, and this chapter lists 26 categories.

The **equipment cards** are `cat.weapon_melee`, `cat.weapon_ranged`, `cat.armor`, `cat.offhand`, `cat.accessory`, `cat.ward_gen`, `cat.wand` and `cat.implant`.

### 3.2 Weapons

Doc 05 owns damage, timing, magazine, reach and skill mapping; doc 06 owns staff and wand casting. This chapter owns base value, template weight, perk family (§7.1) and production class (§5).

| Archetype | Value | Weight | Family | Class | Archetype | Value | Weight | Family | Class |
|---|---|---|---|---|---|---|---|---|---|
| `arch.melee.dagger` | 40 | 900 | E | ML | `arch.ranged.beam` | 98 | 400 | H | RH |
| `arch.melee.blade_short` | 52 | 1000 | E | ML | `arch.ranged.charge_rifle` | 80 | 1000 | D | RD |
| `arch.melee.blade_long` | 76 | 1000 | E | MM | `arch.ranged.crossbow` | 76 | 800 | D | RD |
| `arch.melee.blade_great` | 86 | 700 | E | MH | `arch.ranged.launcher` | 90 | 400 | H | RH |
| `arch.melee.axe` | 58 | 900 | E | MM | `arch.ranged.pistol` | 70 | 1000 | G | RG |
| `arch.melee.axe_great` | 88 | 600 | E | MH | `arch.ranged.pistol_heavy` | 84 | 700 | G | RG |
| `arch.melee.mace` | 58 | 900 | B | MM | `arch.ranged.rifle_auto` | 128 | 700 | H | RH |
| `arch.melee.warhammer` | 86 | 600 | B | MH | `arch.ranged.rifle_marksman` | 94 | 800 | G | RG |
| `arch.melee.spear` | 58 | 900 | E | MM | `arch.ranged.shotgun` | 88 | 800 | G | RG |
| `arch.melee.halberd` | 78 | 600 | E | MH | `arch.ranged.smg` | 86 | 800 | G | RG |
| `arch.melee.quarterstaff` | 38 | 700 | B | MM | `arch.ranged.sniper` | 120 | 500 | G | RG |
| `arch.melee.fist` | 56 | 600 | B | ML | `arch.ranged.staff` (06) | 100 | 700 | C | CA |
| `arch.melee.whip` | 50 | 500 | E | ML | `arch.ranged.thrown` (per unit) | 6 | 600 | D | RD |
| `arch.melee.chain` | 62 | 600 | B | MM | `arch.ranged.wand` (`cat.wand`) | 72 | 1000 | C | CA |

Melee weights total 10500; `cat.weapon_ranged` weights total 9200. A thrown-weapon drop is a stack of 6 (javelin and knife skins alike); its value is per unit.

Families: E melee edged, B melee blunt, D draw, bolt and thrown, G gun, H heavy, C casting. Item tags used by affix and perk requirements:

| Tag | Archetypes |
|---|---|
| `edged` | family E |
| `blunt` | family B |
| `pierce` | doc 05 pierce share at least 3000: dagger, blade_short, spear, halberd, fist, chain, every family-G archetype, rifle_auto, charge_rifle, crossbow, thrown |
| `magazine` | doc 05 magazine at least 2: every family-G archetype, rifle_auto, beam |
| `ammo` | uses a doc 05 ammunition family: every family-G and family-H archetype, charge_rifle, crossbow |
| `ricochet` | doc 05 ricochet yes: every family-G archetype, rifle_auto |
| `firearm` | families G and H, and charge_rifle skins whose dependency is techne |
| `casting` | family C, `arch.offhand.focus`, `arch.offhand.tome` |
| `armored` | armor and shields; `shield` = `arch.offhand.shield_*`; `legs_feet` = legs or feet armor |

### 3.3 Armor, off-hands and ward generators

Doc 05 owns AR, poise, mass and noise (doc 05 §11.1), shield stability (§12.1) and ward capacity and profiles (§15.4).

**Armor** `arch.armor.<slot>_<class>` (class AR):

| Slot | Cloth value / weight | Light | Medium | Heavy |
|---|---|---|---|---|
| head | 8 / 800 | 12 / 1000 | 16 / 1000 | 22 / 800 |
| body | 20 / 560 | 30 / 700 | 36 / 700 | 46 / 560 |
| hands | 6 / 800 | 10 / 1000 | 12 / 1000 | 16 / 800 |
| legs | 12 / 800 | 18 / 1000 | 22 / 1000 | 28 / 800 |
| feet | 6 / 800 | 10 / 1000 | 12 / 1000 | 16 / 800 |
| full set value | 52 | 80 | 98 | 128 |

**Off-hands and ward generators:**

| Template | Value | Weight | Class | Template | Value | Weight | Class |
|---|---|---|---|---|---|---|---|
| `arch.offhand.shield_buckler` | 36 | 1000 | SH | `arch.ward.aegis` (`cap_base` 20 HP) | 60 | 1000 | WG |
| `arch.offhand.shield_heater` | 52 | 1000 | SH | `arch.ward.kinetic` (24 HP) | 80 | 800 | WG |
| `arch.offhand.shield_tower` | 70 | 600 | SH | `arch.ward.reflector` (16 HP) | 80 | 600 | WG |
| `arch.offhand.focus` | 44 | 800 | CA | `arch.ward.bulwark` (36 HP) | 100 | 600 | WG |
| `arch.offhand.tome` (06) | 48 | 600 | CA | | | | |
| `arch.offhand.lantern` | 28 | 600 | CA | | | | |

Ward capacity follows Canon §13.2 and doc 05 §15.4: `cap_base_mhp → × T(ilvl) → × rarity power → × quality → × (10000 + 600 × eff_enh) → × Contradiction potency`, one `mul_bp` per arrow, with no material term. Skin dependencies (§4.3) give each generator, focus and tome its dependency; armor, shields and lanterns have dependency none in every genre (Canon §5.3 mundane armor base).

### 3.4 Accessories

All accessories have `rarity_eligible = true`, roll affixes per Canon §13.3, have no manufacturer and value multipliers per §13.1. `per_enh` scales with `eff_enh`. Rings, amulets and trinkets are type-hidden (Canon §13.6); belts, back items and face items are not. Every implicit has dependency none. The launch counts fit doc 10 §3.2 budgets: 15 rings (at most 24: doc 10 §3.2's ring pool of 26 minus 2 decoys), 8 amulets (16), 19 trinkets (20).

**Gear tolerance cap.** Item-granted tolerance (`gear_tol`, Canon §5.4) from every equipped item and installed augment sums per axis and is capped at `GEAR_TOL_CAP = 3` before the canon cap of 6. Without it, two `+8` tongue rings and an Origin bonus reach Tol 6 on one axis, which leaves every item of that axis at most Strained anywhere and erases the genre tension; with it, a wearer still needs Origin, Attunement or talents for the last steps. Example: two `item.ring.machine_tongue` at `eff_enh` 8 (2 each) plus one `affix.tol_techne` give 5, capped to 3.

**Rings** (`slot.ring_l` or `slot.ring_r`; value 70; mass 20 g; weight 100 each; pool `base`):

| ID | Implicit at +0 | Per `eff_enh` |
|---|---|---|
| `item.ring.might` | +2 each to PWR, BRN, GRP (the `attr.might` sub-attributes) | `+floor_div(eff_enh + 1, 2)` each (each reaches +10 at +15) |
| `item.ring.finesse` | +2 each to PRC, AGI, SLT (the `attr.finesse` sub-attributes) | `+floor_div(eff_enh + 1, 2)` each |
| `item.ring.vigor` | +2 each to TGH, END, RES (the `attr.vigor` sub-attributes) | `+floor_div(eff_enh + 1, 2)` each |
| `item.ring.intellect` | +2 each to LOG, MEM, ANL (the `attr.intellect` sub-attributes) | `+floor_div(eff_enh + 1, 2)` each |
| `item.ring.will` | +2 each to FOC, RSV, ATN (the `attr.will` sub-attributes) | `+floor_div(eff_enh + 1, 2)` each |
| `item.ring.perception` | +2 each to SGT, HRG, ITU (the `attr.perception` sub-attributes) | `+floor_div(eff_enh + 1, 2)` each |
| `item.ring.presence` | +2 each to CHM, MEN, GUI (the `attr.presence` sub-attributes) | `+floor_div(eff_enh + 1, 2)` each |
| `item.ring.fortune` | +2 each to FAT, SER, PRV (the `attr.fortune` sub-attributes) | `+floor_div(eff_enh + 1, 2)` each |
| `item.ring.arcane_tongue` | `gear_tol` +1 on arcana | +1 more at `eff_enh >= 8` |
| `item.ring.machine_tongue` | `gear_tol` +1 on techne | +1 more at `eff_enh >= 8` |
| `item.ring.sacred_tongue` | `gear_tol` +1 on mythos | +1 more at `eff_enh >= 8` |
| `item.ring.bold_tongue` | `gear_tol` +1 on verve | +1 more at `eff_enh >= 8` |
| `item.ring.warding` | +400 Bp resist to fire, frost, shock, acid, radiation and sonic | +50 Bp each |
| `item.ring.wealth` | +1500 Bp in the wearer's Lucre-find sum (§9.1) | +150 Bp |
| `item.ring.keen` | +200 Bp crit chance (Canon §6.4 row 13 gear) | +40 Bp |

**Amulets** (`slot.neck`; value 80; mass 40 g; weight 100 each; pool `base`):

| ID | Implicit at +0 | Per `eff_enh` |
|---|---|---|
| `item.amulet.sanctuary` | +600 Bp resist radiant, umbral, psychic | +60 Bp each |
| `item.amulet.elements` | +700 Bp resist fire, frost, shock | +70 Bp each |
| `item.amulet.ferocity` | +2000 Bp crit multiplier (row 14 gear) | +250 Bp |
| `item.amulet.composure` | +4 RSV, +4 FOC | `+floor_div(eff_enh, 2)` each |
| `item.amulet.vigor` | +4 TGH, +4 RES | `+floor_div(eff_enh, 2)` each |
| `item.amulet.fortune` | +3 FAT, +3 SER, +3 PRV | `+floor_div(eff_enh, 3)` each |
| `item.amulet.wayfarer` | +4 END, +4 AGI | `+floor_div(eff_enh, 2)` each |
| `item.amulet.scholar` | +3 LOG, +3 MEM, +3 ANL | `+floor_div(eff_enh, 3)` each |

**Trinkets** (`slot.trinket`; value 60; mass 50 g; weight 100 each):

| ID | Pool | Effect |
|---|---|---|
| `item.trinket.lattice_lens` | base | carried anywhere in the inventory: `EXIT_OFFERS = 3` (Canon §17.1); +2 ITU when equipped |
| `item.trinket.lucky_coin` | base | +4 FAT, +4 PRV |
| `item.trinket.whetstone` | base | +600 Bp melee damage (additive bucket), +50 per `eff_enh` |
| `item.trinket.powder_horn` | base | +600 Bp ranged weapon damage (additive bucket), +50 per `eff_enh` |
| `item.trinket.reliquary_bone` | base | +1000 Bp resist umbral and radiant |
| `item.trinket.compass_rose` | base | +6 ITU |
| `item.trinket.hourglass` | base | on `ev.folio_entered`, `st.hasted` applied directly for 600 ticks |
| `item.trinket.thimble` | base | +8 armor poise |
| `item.trinket.holy_symbol` | base | prayer implement: prayers take its `ilvl` as `src_lvl` (doc 06 §3.1); +4 PRV |
| `item.trinket.wishbone` | relics_2 | the Fortune Engine top-face guarantee arms after 7 non-top capsules instead of 9 (§11.2) |
| `item.trinket.merchants_scale` | relics_2 | buy trade factor × 9500 (§12.2) |
| `item.trinket.keyring_charm` | relics_2 | on entering the first Folio of each Volume, receive 1 `item.key.bound` |
| `item.trinket.lodestone` | relics_2 | every auto-pickup radius × 2 (§14.3) |
| `item.trinket.warning_bell` | relics_2 | HUD shows hostiles at `aware.suspicious` or higher within 15000 mm (doc 09 states) |
| `item.trinket.saints_finger` | relics_2 | +6 RSV, +6 ATN |
| `item.trinket.clockwork_heart` | relics_2 | +6 END |
| `item.trinket.dragon_scale` | relics_2 | +800 Bp resist fire, +2 armor poise |
| `item.trinket.cat_eye` | relics_2 | +6 SGT |
| `item.trinket.forged_papers` | relics_2 | +6 GUI |

**Belts, back and face** (not type-hidden; weight 100 each; pool `base`):

| ID | Slot | Value | Mass (g) | Effect |
|---|---|---|---|---|
| `item.belt.sash` | `slot.belt` | 30 | 200 | quickbar 5 |
| `item.belt.bandolier` | `slot.belt` | 45 | 400 | quickbar 6 |
| `item.belt.harness` | `slot.belt` | 60 | 600 | quickbar 7 |
| `item.belt.utility` | `slot.belt` | 80 | 800 | quickbar 8 |
| `item.back.cloak` | `slot.back` | 40 | 1200 | +300 Bp resist frost and fire |
| `item.back.satchel` | `slot.back` | 40 | 800 | +6 backpack slots |
| `item.back.rucksack` | `slot.back` | 70 | 1600 | +10 backpack slots |
| `item.back.expedition_pack` | `slot.back` | 110 | 2500 | +14 backpack slots |
| `item.face.mask` | `slot.face` | 30 | 200 | disguise item (doc 09) |
| `item.face.goggles` | `slot.face` | 40 | 300 | +4 SGT |

**Respirators** are doc 08's `apparatus.respirator.cloth`, `apparatus.respirator.mask` and `apparatus.respirator.rebreather` (doc 08 §4.3 owns filter, mass, hearing penalty and dependency; the rebreather depends on techne). This chapter stores their loot records as face accessories: `slot.face`, not type-hidden, rarity-eligible, no manufacturer, pool `base`, values 15, 40 and 90, template weights 40, 40 and 20. These are the chapter's only respirator records (triage C-26); the three weights sum to 100, the weight of one ordinary accessory template.

**Implants** (`cat.implant`): every implant-form `aug.*` of doc 04 §9.4 is a template with `base_value = 40 + 40 × region points`, weight 100, mass 300 g, dependency from its implant skin (techne, Canon §5.3). Where the local TECHNE is below 5, the implant card is merged into the accessory card (§9.2).

### 3.5 Consumables, ammunition, tools, keys, bags and currency

Consumables are never rarity-eligible. `value` is per unit and scales with `P(ilvl)` like every value (§13.1). Doc 10 §8 requires harmful types to hold exactly 3000 Bp of potion generation weight and 1000 Bp of text generation weight; the weights below meet it.

**Potions** (`alch.potion.*`, doc 08 §12.1 effects and values; type-hidden; stack 10; a found potion is a filled `apparatus.vessel.flask` whose mass follows doc 08 §6.1, 400 g for a water base). Found-potion weights, Σ 10000:

| Potion | Weight | Potion | Weight |
|---|---|---|---|
| `alch.potion.healing` | 1200 | `alch.potion.mind_vision` | 400 |
| `alch.potion.stamina` | 600 | `alch.potion.fortitude` | 600 |
| `alch.potion.aether` | 700 | `alch.potion.cure` | 600 |
| `alch.potion.calm` | 500 | `alch.potion.night_eye` | 600 |
| `alch.potion.haste` | 500 | `alch.potion.liquid_flame` (harmful) | 750 |
| `alch.potion.invisibility` | 400 | `alch.potion.frost` (harmful) | 750 |
| `alch.potion.levitation` | 400 | `alch.potion.toxic_gas` (harmful) | 750 |
| `alch.potion.purity` | 500 | `alch.potion.slumber` (harmful) | 750 |

**Reliquary-only potions** (doc 08 §12.2), drawn only by Reliquary consumable bundles (§10): `alch.potion.{shielding, adrenaline, shadowmeld, cleansing, panacea, honeyed_healing, dragons_blood, aquatic_rejuvenation, arcane_armor}` weight 700 each and the harmful `alch.potion.{dragonfire, snap_freeze, corrosive_gas}` weight 900 each, Σ 9000 (harmful 2700 = 3000 Bp of the total). Values follow doc 08's `Σ value(e)` rule. `alch.potion.draught` is brewing-only and never loot.

**Texts** (`item.text.*`, `cat.text`; type-hidden; stack 10; 50 g). Weights Σ 10000, harmful 1000 (doc 10 §8):

| ID | Value | Weight | Effect owner |
|---|---|---|---|
| `item.text.appraisal` | 22 | 1000 | doc 10 `appr.eff.identify` |
| `item.text.sanctify` | 30 | 600 | doc 10 `appr.eff.unbind` |
| `item.text.survey` | 20 | 600 | doc 10 `appr.eff.survey` |
| `item.text.omen` | 18 | 600 | doc 10 `appr.eff.omen` |
| `item.text.consecrate` | 30 | 600 | doc 10 `appr.eff.consecrate` |
| `item.text.intuit` | 24 | 600 | doc 10 `appr.eff.intuit` |
| `item.text.amnesia` (harmful) | 12 | 1000 | doc 10 `appr.eff.amnesia` |
| `item.text.mending` | 20 | 800 | doc 06 authored spell text |
| `item.text.blink` | 26 | 600 | doc 06 |
| `item.text.mapping` | 20 | 600 | doc 06 |
| `item.text.demolition` | 24 | 600 | doc 06; breaks only blocks tagged `block.tag.destructible` (Canon §4.3) |
| `item.text.recharge` | 26 | 600 | doc 06 (wands only) |
| `item.text.warding` | 26 | 600 | doc 06 |
| `item.text.silence` | 24 | 600 | doc 06 |
| `item.text.summoning` | 30 | 600 | doc 06 |
| `item.text.inscribed` | 20 | 0 | created only by inscription (doc 06 §8.3) from `item.tool.blank_text` or a scraped identified `cat.text`; enters the party's known types at creation |

**Throwables** (`cat.throwable`, mundane; stack 10). Weights Σ 10000; the five counter grenades hold 4000:

| ID | Value | Weight | Mechanics |
|---|---|---|---|
| `arch.ranged.grenade_frag` | 10 | 1100 | doc 05 §9.3 |
| `arch.ranged.grenade_incendiary` | 12 | 700 | doc 05 |
| `arch.ranged.grenade_concussion` | 12 | 600 | doc 05 |
| `arch.ranged.grenade_flash` | 12 | 500 | doc 05 |
| `arch.ranged.grenade_smoke` | 10 | 500 | doc 05 |
| `alch.bomb.firebomb` | 12 | 600 | doc 08 §13.2 |
| `alch.bomb.frost`, `.acid`, `.shock` | 12 | 400 each | doc 08 |
| `alch.bomb.smoke` | 10 | 300 | doc 08 |
| `alch.bomb.flash` | 12 | 200 | doc 08 |
| `alch.bomb.salt` | 14 | 300 | doc 08; a dependency-none `counter.disruptor` source |
| `item.throwable.counter_breaker` | 14 | 800 | `arch.ranged.grenade_counter` with bit `counter.ward_breaker` |
| `item.throwable.counter_disruptor` | 14 | 800 | same archetype, bit `counter.disruptor` |
| `item.throwable.counter_staggerer` | 14 | 800 | same archetype, bit `counter.staggerer` |
| `item.throwable.counter_sunder` | 14 | 800 | same archetype, bit `counter.sunder` |
| `item.throwable.counter_anchor` | 14 | 800 | same archetype, bit `counter.anchor` |

Pool `loot.pool.throwables_2` (doc 14, 6 throwables) adds doc 08's `alch.bomb.{blasting, incense, infernal, sleep, stink, tar}` (value 14 each). While it is owned, every base non-counter weight above is multiplied by 4 (Σ 24000), the six new bombs weigh 1000 each (Σ 6000) and each counter grenade weighs 4000 (Σ 20000), so the total is 50000 and the counter share stays 4000 Bp.

**Ammunition** (`ammo.*`, doc 05 §8.1 families, values, carry caps and pickup sizes; dependency none). A drawn `cat.ammo` deck card yields one doc 05 counter charge: `ammo.charge_{anchor, breaker, disruptor, staggerer, sunder}`, ascending ID, weight 100 each, one `weighted` in `loot.arch`. Ordinary ammunition comes from the ammunition line of each rank table (§9.5): in `sub(seed, "loot.ammo")` the family is `weighted(need_bp)` over the families used by the receiver's `slot.weapon_1..3` in ascending ID (doc 05 §8.3; 1 draw when the total need is above 0, else no item and no draw), and the item is one pickup of that family's doc 05 pickup size.

**Champion counters** (Canon §17.3a, doc 05 §15). Every source below has dependency none, so it is legal in all 16 genres:

| Counter | Counter charge (doc 05) | Counter grenade (here) | Other dependency-none source | Skin examples of the grenade (MF / CP / WX) |
|---|---|---|---|---|
| `counter.ward_breaker` | `ammo.charge_breaker` | `item.throwable.counter_breaker` | — | Shatterstone / Breach Charge / Thunder Pellet |
| `counter.disruptor` | `ammo.charge_disruptor` | `item.throwable.counter_disruptor` | `alch.bomb.salt` (doc 08) | Nullroot Flask / EMP Ampoule / Qi-Sealing Powder |
| `counter.staggerer` | `ammo.charge_staggerer` | `item.throwable.counter_staggerer` | — | Weighted Bolas / Concussion Puck / Meteor Bolas |
| `counter.sunder` | `ammo.charge_sunder` | `item.throwable.counter_sunder` | — | Rust Flask / Nanite Spray / Corroding Powder |
| `counter.anchor` | `ammo.charge_anchor` | `item.throwable.counter_anchor` | — | Binding Net / Mag-Tether / Rope Snare |

**Food** (`cat.food`, mundane; stack 20). Satiation, rot and eat ticks are doc 04 §11.2's, copied here:

| ID | Value | Mass (g) | Satiation | Rot ticks | Eat ticks | Notes |
|---|---|---|---|---|---|---|
| `item.food.ration` | 4 | 300 | 600 | never | 60 | also cooked by doc 07 `recipe.cook.smoked_rations` |
| `item.food.meal` | 8 | 500 | 900 | 36000 | 60 | composure +10; every cooked dish is a meal carrying `dish` (doc 07) |
| `item.food.bread` | 2 | 200 | 300 | 18000 | 60 | doc 04's light-food row (snack, bread, fruit, produce) |

Raw meat comes from butchering (doc 04) and `loot.trophy`.

**Tools** (`cat.tool`, mundane):

| ID | Value | Mass (g) | Stack | Use (owner) |
|---|---|---|---|---|
| `item.tool.bandage` | 6 | 50 | 10 | treatment (doc 04 §8.4); ends `st.bleeding` |
| `item.tool.splint` | 8 | 300 | 5 | treatment (doc 04) |
| `item.tool.salve` | 8 | 100 | 10 | treatment (doc 04) |
| `item.tool.surgical_kit` | 40 | 1200 | 1 | treatment and reattachment (doc 04); 3 charges |
| `item.tool.torch` | 2 | 400 | 10 | held light source, `light_block` 12, burns 3600 ticks (doc 03) |
| `item.tool.pick` | 12 | 2500 | 1 | harvests resource nodes (Canon §4.3, doc 07) |
| `item.tool.lockpicks` | 40 | 100 | 10 | locks (doc 09) |
| `item.tool.toolkit` | 24 | 1500 | 1 | construct self-repair (`repair_kit`, doc 07 §8.2) and gadgets; crafting tool tier (doc 07); never consumed |
| `item.tool.hacking_deck` | 60 | 600 | 1 | hacking (doc 09); dependency techne |
| `item.tool.rope` | 6 | 1000 | 5 | climbing (doc 07) |
| `item.tool.crutch` | 10 | 2000 | 1 | held in `slot.offhand`: a severed or tier-4 leg moves at × 7000 (Canon §9.1, doc 04 §8.5) |
| `item.tool.blank_text` | 8 | 50 | 10 | inscription (doc 06 §8.3) |
| `item.tool.rewrite_quill` | 60 | 50 | 5 | Rewrite anywhere (§15) |
| `item.tool.emendation` | 300 | 50 | 15 | §16; never sold or bought |
| `item.tool.grapple` | 30 | 1500 | 1 | grappling tool (DECISIONS 95, doc 03 §5.24); never consumed |

**Grappling tool** (`item.tool.grapple`, `cat.tool`, mundane, never part of a starting kit, DECISIONS 95). A quickbar use is an item-use action that calls doc 03 `start_grapple(e)`: a 20,000 mm ray from the eye anchors on any block face at least 1,000 mm away and pulls at 12,000 mm/s for at most 120 ticks, arriving within 1,000 mm (doc 03 §5.24). An accepted use sets `ready_tick = tick + 180`; a use before `ready_tick` is refused, and a refused use (including one `start_grapple` refuses) spends nothing and leaves `ready_tick` unchanged. Its dependency is none in every genre, so it never Contradicts and works in `genre.unwritten`. Sources: `shop.general` × 1 (§12.1); never drawn from the deck. Genre skins (`loc.item.tool.grapple.skin.<suffix>.name`): mythic_fantasy Climbing Hook, gothic_horror Bell-Rope Hook, cosmic_horror Expedition Grapnel, space_opera Mag-Line, cyberpunk Grapple Gun, post_apocalypse Scrap Hook, weird_west Lariat Hook, hardboiled_noir Fire-Escape Hook, wuxia Rope Dart, steampunk Steam Grapnel, fairy_tale Beanstalk Vine, mythic_antiquity Argonaut's Grapnel, oneiric Dream Thread, redacted Retrieval Line, unwritten Line, ur_text First Line.

**Books** (`cat.book`, mundane, always `buc.uncursed`, `q.normal`, stack 1). Reading a glyph tome teaches its `teaches` glyphs with origin = the tome's `origin_genre` (doc 06 §8.1: 600 ticks, not `in_combat`, not consumed). Reading a schematic teaches its one recipe (doc 07 §6.5: 60 ticks, consumed).

| ID | Kind | `teaches` | Value | Mass (g) |
|---|---|---|---|---|
| `item.book.primer_abjuration` | glyph tome | glyphs of `school.abjuration` | 48 | 800 |
| `item.book.primer_alteration` | glyph tome | glyphs of `school.alteration` | 48 | 800 |
| `item.book.primer_conjuration` | glyph tome | glyphs of `school.conjuration` | 48 | 800 |
| `item.book.primer_divination` | glyph tome | glyphs of `school.divination` | 48 | 800 |
| `item.book.primer_evocation` | glyph tome | glyphs of `school.evocation` | 48 | 800 |
| `item.book.primer_illusion` | glyph tome | glyphs of `school.illusion` | 48 | 800 |
| `item.book.primer_restoration` | glyph tome | glyphs of `school.restoration` | 48 | 800 |
| `item.book.primer_spellcraft` | glyph tome | glyphs of `school.spellcraft` (its modifiers, every trigger and condition) | 48 | 800 |
| `item.book.schematic` | schematic | one recipe with `learn: 1` (doc 07 §6.5) | 40 | 200 |

Books are never drawn from the deck (§9.2). Kit primers (doc 14 §2.4) carry the glyphs their kit names and draw nothing. A **glyph-tome line** (§12.1) picks its template with 1 `weighted` in `loot.arch` over the eight primers (weight 100 each, ascending ID), then fills `teaches` with doc 06 `glyph_tome(school, ilvl, content set, sub(seed, "loot.book"))` (at most 2 draws), so the run's `loot.pool.glyphs_2` and glyph families gate what it teaches. A **schematic line** (§12.1) creates `item.book.schematic` and fills `teaches` with 1 `weighted` in `loot.book` over the run content set's recipes with `learn: 1`, weight 100 each, ascending ID; the `r2` recipes are in the content set only with `loot.pool.recipes_2`. Both lines are Placed (§9.11), so neither looks at what any player already knows (§1 principle 7).

**Containers**: `apparatus.vessel.vial` 1, `.flask` 2, `.bottle` 3 Lucre (doc 08 §6.1 masses), stack 10; treasure bags `item.bag.*` (§9.7).

**Keys** (`cat.key`, mundane, mass 0, on the keyring, §14.1). A Reliquary key opens its Reliquary tier and every lower tier (§10); opening spends it. A door key opens the doc 12 `Locked` door whose key list holds its `key_tag` (doc 09 §13.1) and is never spent.

| ID | Value | Opens | Sources |
|---|---|---|---|
| `item.key.bound` | 30 | `reliq.plain`, `reliq.bound` | elite and champion key lines, the mini-boss key line (§9.5); plain and locked chests (§9.6); Volume-plan caches (§9.9); `item.trinket.keyring_charm` |
| `item.key.gilded` | 90 | `reliq.plain` to `reliq.gilded` | champion key line, the mini-boss key line (§9.5); plain chests (§9.6); the Volume-plan gilded cache (§9.9) |
| `item.key.illumined` | 250 | every tier | the Editor table `loot.boss.editor` (§9.5) |
| `item.key.door` | 0 | the `Locked` door of its `key_tag` | doc 12 places one `Public` ground key at each `Key(k)` node with `key_tag = k`; vendors refuse it; it is removed from every inventory at Folio exit |

**Currency**: `item.lucre`, a pile whose amount is its `value` (`i64`); picked up into the purse.

**Quest and corpse** (value 0, stack 1, never generated by `generate_item`; their placers create them directly from the template):

| ID | Category | Mass (g) | Placed by | Rules |
|---|---|---|---|---|
| `item.quest.catchword` | `cat.quest` | 1000 | doc 14 §3.4 `run.rubric.catchwords`: three in distinct rooms of a Folio with that Rubric | cannot be dropped (doc 14 §3.4) |
| `item.quest.parcel` | `cat.quest` | 1000 | doc 13 retrieve and deliver quests | delivered to the quest's target |
| `item.quest.folio_leaf` | `cat.quest` | 50 | doc 12 §20.5: one per Folio at the story seat, a `Public` ground item | consumed on the exit-commit tick when an Active Erratum holds it, which sets `FolioRecord.leaf_carried` (doc 12 §20.5, doc 14) |
| `item.remains` | `cat.corpse` | 75000 | doc 14 §3.10, when an Erratum becomes a ghost | records the ghost's slot and holds no items; carried through an exit it sets `remains_carried` (doc 14 §3.10, doc 16) |

Vendors refuse quest items and `item.remains` (§12.2), and neither enters the Lost Satchel (§14.5). Quest items cannot be Rewritten or salvaged (§15, §16.4), and `DropItemFor` refuses them (§14.4).

### 3.6 Gadgets and loose parts

Doc 07 owns gadget and part mechanics. This chapter fixes their loot-facing stats (Interface assumptions). A gadget's dependency comes from its genre skin (doc 07 §11.1 rule 3): reusable gadgets take techne, arcana or mythos by genre and none in `genre.wuxia`, `genre.unwritten` and `genre.ur_text`; consumable gadgets have none everywhere. Step 2 applies the axiom-0 rule (Canon §5.3) to that dependency.

| Gadget | Value | Mass (g) | Charges | Loot stats |
|---|---|---|---|---|
| `gadget.turret_light` | 44 | 3000 | 1 | deploy 60 ticks; lasts 1800 ticks; shot base 4 HP × `P(src_lvl)` |
| `gadget.drone_scout` | 40 | 1500 | 1 | lasts 3600 ticks; marks hostiles with `st.marked` within 12000 mm |
| `gadget.mine_frag` | 18 | 800 | 1 | base 30 HP × `P(src_lvl)` `dmg.pierce` in 3000 mm |
| `gadget.trap_snare` | 12 | 1200 | 1 | `st.rooted` applied directly |
| `gadget.beacon_mend` | 36 | 1000 | 1 | heals 100 Bp of Max HP per second within 4000 mm for 600 ticks |
| `gadget.decoy` | 20 | 1500 | 1 | taunts hostiles within 8000 mm for 300 ticks |

Six more gadgets enter with `loot.pool.gadgets_2` (doc 07 names them). Loose parts: §5.4.

### 3.7 Common item lists (doc 13 carried items)

Doc 13 §5.1 rolls NPC carried items with one `weighted` over "the doc 11 list of common items of that category in the local genre". That list is every template step 2 makes eligible for that category and genre with the base content set, in ascending ID, with its §3 template weight. Carried items are `rar.common`, `q.normal`, `buc.uncursed`, and never Exotic or Palimpsestic.

### 3.8 Rarities

Canon §13.3 owns the names, colors, base weights, power and value multipliers; this table restates them with what this chapter attaches. `r` is the rarity index of §2.3 step 3 and §9.4.

| ID | `r` | Affixes | Perk columns (§7.1) | Power (Bp) | Value | Base weight | Doc 11 additions |
|---|---|---|---|---|---|---|---|
| `rar.common` | 0 | 0 | 1, plus 5 | 10000 | ×1 | 7000 | the fixed rarity of every template with `rarity_eligible = false` |
| `rar.uncommon` | 1 | 1 | 1, 2, plus 5 | 10500 | ×2 | 2200 | — |
| `rar.rare` | 2 | 2 | 1 to 3, plus 5 | 11000 | ×4 | 600 | — |
| `rar.epic` | 3 | 3 | 1 to 4, plus 5 | 11500 | ×8 | 150 | — |
| `rar.legendary` | 4 | 3 | 1 to 4, plus 5 | 12500 | ×16 | 45 | one Legendary perk (§8.1) |
| `rar.exotic` | 5 | 2 | 1, 2, plus 5 | 12500 | ×32 | 15 | an `item.exotic.*` template and its intrinsic (§8.2); weight 0 in Placed sources |
| `rar.palimpsestic` | 6 | 4 | 1 to 4, plus the local genre's trait | 13000 | ×64 | 0 | an `item.palimpsestic.*` unique (§8.3); only the §9.8 sources force it |

From Epic up the tier-1 affix weight is halved (§6.1). From Legendary up a Keyed or Keepsake item rolls Illuminated (§2.3 step 13).

### 3.9 Equipment slots

Canon §13.2 owns what each slot accepts; this table restates it with the templates of this chapter that go there and the Canon §13.3 Exotic group (one Exotic per group).

| ID | Accepts (Canon §13.2) | Templates | Exotic group |
|---|---|---|---|
| `slot.head` | head armor | `arch.armor.head_<class>` | armor |
| `slot.body` | body armor | `arch.armor.body_<class>` | armor |
| `slot.hands` | hand armor | `arch.armor.hands_<class>` | armor |
| `slot.legs` | leg armor | `arch.armor.legs_<class>` | armor |
| `slot.feet` | foot armor | `arch.armor.feet_<class>` | armor |
| `slot.back` | cloaks, backpacks, jetpacks, wings | `item.back.*` (backpack slots, §14.1) | other |
| `slot.neck` | amulets | `item.amulet.*`, `item.palimpsestic.colophon_mark` | other |
| `slot.ring_l` | rings | `item.ring.*`, `item.palimpsestic.endpaper_band` | other |
| `slot.ring_r` | rings | as `slot.ring_l` | other |
| `slot.belt` | belts; the belt sets the quickbar size | `item.belt.*` (quickbar 5 to 8) | other |
| `slot.trinket` | relics and charms | `item.trinket.*`, `item.palimpsestic.dog_ear` | other |
| `slot.weapon_1` | any melee or ranged weapon | `cat.weapon_melee` and `cat.weapon_ranged` templates | weapon |
| `slot.weapon_2` | as `slot.weapon_1` | as `slot.weapon_1` | weapon |
| `slot.weapon_3` | as `slot.weapon_1` | as `slot.weapon_1` | weapon |
| `slot.offhand` | off-hand items; empty while a two-handed weapon is active | `cat.offhand` templates; wands (doc 10 §4.3); `item.tool.crutch` | weapon |
| `slot.implant_head` | implants | implant-form `aug.*` of the head region (doc 04 §9.4) | other |
| `slot.implant_torso` | implants | implant-form `aug.*` of the torso region | other |
| `slot.implant_arms` | implants | implant-form `aug.*` of the arms region | other |
| `slot.implant_legs` | implants | implant-form `aug.*` of the legs region | other |
| `slot.ward` | one ward generator | `cat.ward_gen` templates | other |
| `slot.face` | masks, goggles, respirators | `item.face.*`; doc 08 `apparatus.respirator.*` (§3.4) | other |
| `slot.hand_extra_1` | granted by `bp.arm_extra_1`: one one-handed weapon or off-hand item | one-handed `cat.weapon_melee` and `cat.weapon_ranged` templates, `cat.offhand` templates | weapon |
| `slot.hand_extra_2` | granted by `bp.arm_extra_2`, as `slot.hand_extra_1` | as `slot.hand_extra_1` | weapon |

Canon §13.2 grants `slot.hand_extra_1` and `slot.hand_extra_2` through doc 04's two extra-arm parts, `bp.arm_extra_1` and `bp.arm_extra_2` (from `aug.extra_arm`, `body.serpentine` and the mutant `extra_arm`), and Canon §13.3 counts both in the weapon Exotic group.

### 3.10 Retired IDs

A retired ID is never generated, exported or loaded as a record. The loader resolves it as an alias of its replacement and includes it in the `id32` collision check (Canon §1.1).

| ID | Retired | Replacement | Reason |
|---|---|---|---|
| `item.relic.lattice_lens` | before first ship | `item.trinket.lattice_lens` | every trinket uses the `item.trinket.` prefix (§3.4) |

Mounts are cut (DECISIONS 55, Canon §17.7), so this chapter defines no mount category, slot, token, affix, shop or loot line, and keeps no alias for them.

---

## 4. Genre skins

### 4.1 Rules

- A skin is the pair (archetype, genre). It supplies the display name `loc.<archetype>.skin.<genre_suffix>.name` (Canon §1.10), the mesh and sound sets (doc 18) and the base component's dependency set (Canon §5.3). Doc 05 §3.1: a skin never changes a mechanical field.
- Notation: no tag = dependency none; `[A]` arcana, `[T]` techne, `[M]` mythos, `[V]` verve. `—` = no skin: the archetype is never generated in that genre and cannot be Rewritten into it.
- Validation (build time): no cell may depend on an axis whose axiom is 0 in that genre (Canon §5.3), so `genre.post_apocalypse` and `genre.hardboiled_noir` have no `[A]`, `genre.redacted` has no `[V]`, and every `genre.unwritten` cell is untagged or `—`. Staves, wands, foci and tomes always depend on the genre's `aether_axiom`, or arcana when it is none, so they are `—` in `genre.unwritten`.
- `Req[a] = max(1, origin_axiom[a] - 2)` for each tagged axis (Canon §5.3). Example: a Space Opera Photon Sabre `[T]` has `Req[techne] = 8`; a Weird West Gatling Pistol `[T]` has `Req[techne] = 2`.
- Charge rifles follow doc 05 §5.3: bow skins are untagged and drain stamina while overdrawn; railgun skins depend on techne.

### 4.2 Weapon skins (28 archetypes × 16 genres)

**Table A: blades, axes and maces**

| Genre | dagger | blade_short | blade_long | blade_great | axe | axe_great | mace |
|---|---|---|---|---|---|---|---|
| mythic_fantasy | Dirk | Arming Sword | Longsword | Greatsword | Bearded Axe | Dane Axe | Flanged Mace |
| gothic_horror | Silvered Stiletto [M] | Hunting Hanger | Duelling Sword | Executioner's Sword | Woodsman's Axe | Headsman's Axe | Censer Mace [M] |
| cosmic_horror | Ritual Knife [M] | Cutlass | Expedition Machete | Star-Iron Blade [M] | Fire Axe | Ice-Breaker Axe | Crowbar |
| space_opera | Vibroknife [T] | Plasma Cutter [T] | Photon Sabre [T] | Ion Greatblade [T] | Shear-Axe [T] | Plasma War-Axe [T] | Gravity Baton [T] |
| cyberpunk | Mono-Shiv [T] | Thermal Wakizashi [T] | Monomolecular Katana [T] | Vibro-Nodachi [T] | Hydraulic Hatchet [T] | Chainsaw Axe [T] | Stun Baton [T] |
| post_apocalypse | Scrap Shiv | Saw-Machete | Sharpened Rebar Machete | Car-Spring Cleaver | Stop-Sign Axe | Saw-Blade Axe | Pipe Wrench |
| weird_west | Bowie Knife | Arkansas Toothpick | Hexed Saber [M] | Sodbuster Scythe-Blade | Tomahawk | Felling Axe | Silver-Capped Cudgel [M] |
| hardboiled_noir | Switchblade | Machete | Sword Cane | Antique Claymore | Hatchet | Fireman's Axe | Blackjack |
| wuxia | Bishou | Dao | Jian | Zhanmadao | Fu Axe | Moon Axe | Bian Rod |
| steampunk | Clockwork Stiletto [T] | Naval Cutlass | Officer's Sabre | Steam Claymore [T] | Boiler Axe [T] | Steam Axe [T] | Piston Mace [T] |
| fairy_tale | Silver Bodkin | Huntsman's Knife | Knight's Sword | Giant-Slayer Blade [A] | Woodcutter's Axe | Ogre's Axe | Troll Club |
| mythic_antiquity | Parazonium | Xiphos | Kopis | Rhomphaia | Labrys [M] | Double Labrys [M] | Olive-Wood Club |
| oneiric | Thought-Splinter [A] | Recurring Blade [A] | Sleepwalker's Sword | Nightmare Edge [M] | Clock-Hand Axe | Guillotine Axe [M] | Bell of Waking [A] |
| redacted | Letter Opener | Censor's Blade [T] | Black-Bar Sword [T] | Redaction Cleaver [M] | Shredder Axe [T] | Decommission Axe [T] | Rubber Stamp [M] |
| unwritten | Pencil Knife | Margin Blade | Draft Sword | Unfinished Greatsword | Sketched Axe | Unfinished Axe | Outline Mace |
| ur_text | Ur-Dirk | Ur-Gladius | Ur-Sword | Ur-Greatsword | Ur-Axe | Ur-Great-Axe | Ur-Mace |

**Table B: hammers, polearms, fists, whips and chains**

| Genre | warhammer | spear | halberd | quarterstaff | fist | whip | chain |
|---|---|---|---|---|---|---|---|
| mythic_fantasy | Warhammer | Boar Spear | Halberd | Quarterstaff | Cestus | Lash | Flail |
| gothic_horror | Grave Maul | Stake-Spear [M] | Bardiche | Pilgrim's Staff [M] | Knuckleduster | Scourge [M] | Morning Star |
| cosmic_horror | Sledgehammer | Harpoon | Boathook Glaive | Walking Stick | Brass Knuckles | Rigging Whip | Anchor Chain |
| space_opera | Grav-Hammer [T] | Force Pike [T] | Plasma Glaive [T] | Shock Stave [T] | Power Gauntlet [T] | Energy Whip [T] | Mag-Chain [T] |
| cyberpunk | Pneumatic Sledge [T] | Shock Lance [T] | Vibro-Naginata [T] | Telescoping Bo [T] | Mantis Claws [T] | Monowire Whip [T] | Chain Blade [T] |
| post_apocalypse | Engine-Block Maul | Pipe Spear | Signpost Glaive | Rebar Staff | Nail Knuckles | Cable Whip | Bike-Chain Flail |
| weird_west | Railroad Sledge | Buffalo Lance | Harvest Scythe | Preacher's Staff [M] | Rawhide Knuckles | Bullwhip | Shackle Chain |
| hardboiled_noir | Sledge | Gaff Hook | Fire Hook | Pool Cue | Knuckle-Dusters | Garrote Lash | Bicycle Chain |
| wuxia | Melon Hammer | Qiang | Guandao | Monk's Staff | Tiger Claws | Rope Dart | Meteor Hammer |
| steampunk | Steam Hammer [T] | Brass Lance | Gear Halberd [T] | Tesla Cane [T] | Clockwork Gauntlet [T] | Galvanic Whip [T] | Chain Flail |
| fairy_tale | Ogre Maul | Huntsman's Spear | Poleaxe | Shepherd's Crook | Iron Gloves | Thorn Lash [A] | Threshing Flail |
| mythic_antiquity | Bronze Maul | Dory | Sarissa | Herald's Caduceus [M] | Caestus | Scourge of the Furies [M] | Bronze Flail |
| oneiric | Dream Anvil [A] | Moonbeam Lance [A] | Pendulum Glaive | Stair-Rail Staff | Sleep-Paws | Ribbon Lash [A] | Pendulum Chain |
| redacted | Gavel [M] | Filing Spike | Turnstile Glaive [T] | Baton of Office [T] | Knuckle Seals | Red-Tape Whip [M] | Cuff Chain |
| unwritten | Blank Maul | Draft Spear | Line Glaive | Margin Staff | Bare Gauntlet | Trailing Line | Outline Flail |
| ur_text | Ur-Maul | Ur-Spear | Ur-Glaive | Ur-Quarterstaff | Ur-Fist | Ur-Lash | Ur-Flail |

**Table C: guns**

| Genre | pistol | pistol_heavy | smg | rifle_auto | rifle_marksman | shotgun | sniper |
|---|---|---|---|---|---|---|---|
| mythic_fantasy | Dwarven Handgonne [T] | Dwarven Hand Cannon [T] | Repeating Crossbow | Repeating Ballista | Dwarven Long-Gonne [T] | Blunderbuss [T] | Siege Arbalest |
| gothic_horror | Flintlock Pistol [T] | Holster Pistol [T] | Pepperbox [T] | Volley Gun [T] | Hunting Musket [T] | Dragoon Blunderbuss [T] | Long Rifle [T] |
| cosmic_horror | Service Pistol [T] | Elephant Revolver [T] | Drum-Fed Submachine Gun [T] | Pan-Fed Machine Gun [T] | Bolt-Action Rifle [T] | Coach Gun [T] | Elephant Gun [T] |
| space_opera | Blaster Pistol [T] | Heavy Blaster [T] | Pulse Repeater [T] | Rotary Blaster [T] | Blaster Rifle [T] | Scatter Blaster [T] | Long Blaster [T] |
| cyberpunk | Smart Pistol [T] | Hand Cannon [T] | Machine Pistol [T] | Smart Rifle [T] | Tech Rifle [T] | Smart Shotgun [T] | Coil Sniper [T] |
| post_apocalypse | Pipe Pistol [T] | Scrap Revolver [T] | Pipe Burst-Gun [T] | Scrap Chaingun [T] | Hunting Rifle [T] | Sawn-Off [T] | Scrap Long Rifle [T] |
| weird_west | Pocket Derringer [T] | Six-Shooter [T] | Gatling Pistol [T] | Gatling Gun [T] | Lever-Action Rifle [T] | Scattergun [T] | Buffalo Rifle [T] |
| hardboiled_noir | Snub-Nose Revolver [T] | Heavy Revolver [T] | Drum Submachine Gun [T] | Belt-Fed Machine Gun [T] | Scoped Hunting Rifle [T] | Pump Shotgun [T] | Sniper Rifle [T] |
| wuxia | Fire Lance [T] | Bronze Hand Cannon [T] | Zhuge Repeater | Ten-Bolt Repeater | Composite Recurve | Arrow-Swarm Box [T] | Bed Crossbow |
| steampunk | Pepperbox Pistol [T] | Steam Revolver [T] | Clockwork Repeater [T] | Steam Gatling [T] | Brass Rifle [T] | Brass Blunderbuss [T] | Galvanic Long Rifle [T] |
| fairy_tale | Thunder Pistol [A] | Ogre's Hand-Gonne [T] | Pixie Shooter [A] | Thorn-Volley Briar [A] | Fletcher's Bow | Seed Spitter [A] | Giant-Slayer Bow |
| mythic_antiquity | Dart Caster | Heavy Dart Caster | Volley Bow [V] | Polybolos [T] | Sun-Gilded Bow [M] | Shard Sling | Scorpion Bolt-Thrower |
| oneiric | Finger-Gun [V] | Thunderclap Revolver [V] | Bubble Repeater [A] | Music-Box Gatling [A] | Telescope Rifle [T] | Confetti Gun [A] | Stargazer Rifle [T] |
| redacted | Service Sidearm [T] | Enforcement Revolver [T] | Riot SMG [T] | Suppression Gun [T] | Marksman Rifle [T] | Riot Shotgun [T] | Compliance Rifle [T] |
| unwritten | Outline Pistol | Outline Revolver | Outline Repeater | Unfinished Gatling | Outline Rifle | Outline Scattergun | Outline Long Rifle |
| ur_text | Ur-Pistol | Ur-Revolver | Ur-Repeater | Ur-Gatling | Ur-Rifle | Ur-Scatter | Ur-Long-Rifle |

**Table D: heavy, draw, thrown and casting**

| Genre | launcher | beam | charge_rifle | crossbow | thrown | staff | wand |
|---|---|---|---|---|---|---|---|
| mythic_fantasy | Hand Ballista | Dragon-Breath Horn [A] | Longbow | Arbalest | Throwing Knives | Wizard's Staff [A] | Rune Wand [A] |
| gothic_horror | Grenadier's Mortar [T] | Holy-Fire Censer [M] | Yew Longbow | Stake Crossbow [M] | Throwing Stakes | Crozier [M] | Bone Wand [M] |
| cosmic_horror | Flare Mortar [T] | Flamethrower [T] | Expedition Longbow | Harpoon Crossbow | Harpoon Darts | Eldritch Rod [M] | Fetish Wand [M] |
| space_opera | Missile Launcher [T] | Plasma Projector [T] | Railgun [T] | Rail-Bolt Caster [T] | Vibro-Stars [T] | Psi Staff [A] | Psi Rod [A] |
| cyberpunk | Micro-Missile Pod [T] | Thermal Lance [T] | Rail Rifle [T] | Mag-Bolt Crossbow [T] | Smart Shuriken [T] | Neural Projector [T] | Cyberdeck Wand [T] |
| post_apocalypse | Pipe Mortar [T] | Fuel-Can Flamer [T] | Junk Railgun [T] | Junk Crossbow | Rebar Javelins | Static Rod [T] | Tesla Wand [T] |
| weird_west | Dynamite Launcher [T] | Hellfire Bellows [M] | Plains Longbow | Bounty Crossbow | Throwing Tomahawks | Hex Rod [M] | Bone Fetish [M] |
| hardboiled_noir | Riot Grenade Launcher [T] | Blowtorch [T] | Hunting Bow | Speargun | Throwing Knives | Lucky Cane [V] | Loaded Dice [V] |
| wuxia | Thunder-Crash Tube [T] | Dragon-Fire Lance [T] | Great Bow | Heavy Nu Crossbow | Flying Daggers | Qi Staff [V] | Talisman Brush [V] |
| steampunk | Rocket Musket [T] | Boiler Flamer [T] | Galvanic Rifle [T] | Pneumatic Arbalest [T] | Spring Darts | Pressure Staff [T] | Galvanic Rod [T] |
| fairy_tale | Gourd-Bomb Sling [A] | Firebird Horn [A] | Elfin Longbow | Dwarf-Made Crossbow | Huntsman's Javelins | Witch's Staff [A] | Fairy Wand [A] |
| mythic_antiquity | Fire-Pot Thrower | Greek Fire Siphon [T] | Hero's Greatbow | Gastraphetes | Javelins | Oracle's Staff [M] | Thyrsus [M] |
| oneiric | Dream-Bubble Mortar [A] | Nightlight Beam [A] | Moonlight Bow [A] | Clock Crossbow | Paper Stars | Lucid Staff [A] | Nightlight Wand [A] |
| redacted | Containment Launcher [T] | Incinerator [T] | Compliance Railgun [T] | Bolt Stapler [T] | Letter Openers | Clearance Rod [T] | Badge Reader [T] |
| unwritten | Blank Launcher | Smudge Sprayer | Outline Longbow | Draft Crossbow | Draft Darts | — | — |
| ur_text | Ur-Launcher | Ur-Sprayer | Ur-Longbow | Ur-Crossbow | Ur-Javelins | Ur-Rod [A] | Ur-Wand [A] |

### 4.3 Ward generator and implement skins

A ward generator skin is one genre noun with its dependency; the archetype adds a fixed prefix: `arch.ward.aegis` none, `arch.ward.kinetic` "Bracing", `arch.ward.reflector` "Mirrored", `arch.ward.bulwark` "Heavy" (for example the Cyberpunk Kinetic Barrier, Bracing Kinetic Barrier, Mirrored Kinetic Barrier and Heavy Kinetic Barrier). Foci and tomes depend on the genre's aether axis exactly like staves (§4.2) and are `—` in `genre.unwritten`; their names are doc 18's.

| Genre | Ward generator noun |
|---|---|
| mythic_fantasy | Aegis Charm [A] |
| gothic_horror | Reliquary Locket [M] |
| cosmic_horror | Warding Star [M] |
| space_opera | Deflector Belt [T] |
| cyberpunk | Kinetic Barrier [T] |
| post_apocalypse | Scrap Bubble [T] |
| weird_west | Medicine Bag [M] |
| hardboiled_noir | Lucky Rabbit's Foot [V] |
| wuxia | Golden Bell Shield [V] |
| steampunk | Faraday Harness [T] |
| fairy_tale | Godmother's Ribbon [A] |
| mythic_antiquity | Gorgon Ward [M] |
| oneiric | Pillow Fort [A] |
| redacted | Clearance Badge [T] |
| unwritten | Padded Margin |
| ur_text | Ur-Ward |

---

## 5. Manufacturers

### 5.1 Rules

- A manufacturer stamps weapons, off-hands, armor, ward generators and loose parts. Accessories, implants and consumables have none.
- **Tendencies** multiply doc 05 base stats once at generation: `stat = mul_bp(stat, 10000 + delta)`, with tick and count results rounded by `round_half_even` and kept at least 1. Legend: D damage `base_mhp`, S attack speed, P spread, M magazine, L reload ticks, R range, H handling (swap and ADS) ticks, T stamina cost, O poise damage (impact), W mass, A AR, X precision multiplier, C aether cost (`cost_bp`), V value. A tendency on a stat the item lacks is ignored. Mass changes then feed doc 05 §3.3 timing.
- **Gimmick.** The signature gimmick is one component (Canon §5.3) of the item, with the dependency listed, the item's `origin_genre` and its own Contradiction potency. "+N Bp" gimmick numbers enter the additive bucket (Canon §10.5 step 1). A gimmick stated as "× N" multiplies the hit's `B` once after the Canon §10.4 chain and before step 1 (Canon §10.4).
- **Classes** (§3.2, §3.3): ML light melee (dagger, blade_short, fist, whip), MM medium melee (blade_long, axe, mace, spear, quarterstaff, chain), MH heavy melee (blade_great, axe_great, warhammer, halberd), RD draw, bolt and thrown (charge_rifle, crossbow, thrown), RG guns (pistol, pistol_heavy, smg, rifle_marksman, shotgun, sniper), RH heavy ranged (rifle_auto, launcher, beam), CA casting (staff, wand, focus, tome, lantern), AR armor, SH shields, WG ward generators.
- The first two rows of each core genre are the Canon §5.9 manufacturers with their canon traits; the third row and `mfr.errata.marginal_press` are added by this chapter. Display names avoid real-world brands and third-party titles (doc 18 runs the name review).

### 5.2 Manufacturer table

| ID | Name | Makes | Tendencies | Parts (§5.4) | Signature gimmick | Dep |
|---|---|---|---|---|---|---|
| `mfr.mythic_fantasy.deepanvil` | Deepanvil Forge | MM MH RG RH AR SH | D+800 W+1500 | eh hl gg bh pr | weapons: poise damage × 15000 and attack speed × 9000; armor and shields: +4 poise per piece | none |
| `mfr.mythic_fantasy.silverbough` | Silverbough Fletchery | ML RD AR WG | S+500 W−2000 D−300 | ek hg bl os | attack noise −20 dB; armor: footstep noise −6 dB | none |
| `mfr.mythic_fantasy.runecourt` | The Runecourt | CA WG ML MM | C−800 | fc fn ek cc | Spellsink: every kill made with or cast through the item restores 3 aether; ward generators: ward break restores 10 aether | arcana |
| `mfr.gothic_horror.vesper_sons` | Vesper & Sons | ML MM RD CA | D+300 | ek es br fc | 2000 Bp of each hit moves from its largest physical share to `dmg.radiant` | mythos |
| `mfr.gothic_horror.grimhallow` | Grimhallow Gunsmiths | RG RH ML | D+600 M−2000 | bs fq oi gw | a reload completes in 1 tick and removes `mul_bp(max_hp_mhp, 300)` mHP directly (not damage; refused if HP would fall below 1,000 mHP, and a refused cost starts no reload); melee: heavy attacks cost that HP instead of stamina | mythos |
| `mfr.gothic_horror.ashgrave` | Ashgrave Ironworks | MH MM AR SH WG | A+1000 W+2000 S−300 | eh gg pr lp cc | Grave Iron: while HP ≤ 3000 Bp of Max HP, Ashgrave armor and shield AR × 13000 and Ashgrave weapons +2000 Bp | none |
| `mfr.cosmic_horror.hollow_lantern` | Hollow Lantern Optics | RG RD CA ML | R+1500 D−300 | os br fc ek | hits apply `st.marked` directly for 240 ticks | mythos |
| `mfr.cosmic_horror.outer_choir` | The Outer Choir | CA RH MM | D+500 | fn bh eh fd | 2500 Bp of each hit moves from its largest share to `dmg.psychic`; +1500 Bp; each attack costs 1 composure, and at 0 composure the item cannot attack | mythos |
| `mfr.cosmic_horror.dunwold` | Dunwold Expedition Supply | RG MM MH AR SH WG | T−1000 S−300 | br fe hl pr lp | Field-Tested: weapons reload while sprinting; armor: +300 Bp resist acid and toxin | none |
| `mfr.space_opera.perihelion` | Perihelion Arms | RG RD RH | D+500 S−800 | bl os fe | holding fire 30 ticks charges the next shot: × 15000 | techne |
| `mfr.space_opera.umbra` | Umbra Heavy Industries | RG RH WG | S+800 D−300 | bh or cc | no magazine and no ammo; each shot adds `floor_div(10000, base_magazine)` heat; heat decays 2500 per second from 30 ticks after the last shot; at 10000 the weapon locks for 90 ticks, then heat is 0; ward generators: capacity × 11000 | techne |
| `mfr.space_opera.starwright` | Starwright Psionics | CA ML MM MH AR SH | D+200 W−1500 | fc ek hg pr | Kinetic Echo: every 5th hit repeats 20 ticks later as a `dmg.psychic` hit entering Canon §10.6 at step 3 with `D2` = 5000 Bp of the original `D2`; armor: every 5th melee hit taken pushes the attacker 1500 mm | arcana |
| `mfr.cyberpunk.ghostline` | Ghostline Systems | RG RD ML | P−1500 D−500 | or br fq ek | projectiles home on `st.marked` targets at 16384 Bam/s, turned per tick through the Canon §2.3 accumulator (`k = 60`: 273 or 274 Bam per tick); doc 05 copies the 16384 into `Projectile.home_bam_s` at spawn and the accumulator lives in `Projectile.home_acc` (doc 03 §5.19); melee: +1500 Bp vs `st.marked` | techne |
| `mfr.cyberpunk.neon_saint` | Neon Saint Customs | RG RD RH WG | D+700 L+1500 | bl os fd cc | a charged shot passes the first penetrable (destructible or non-opaque) cell it enters without spending penetration; fixed terrain stops it (Canon §4.3); a shot is charged at full draw or charge (families D), else when it is the first shot after aiming 45 ticks | techne |
| `mfr.cyberpunk.chromaw` | Chromaw Biotech | ML MM MH AR SH CA | S+600 | es hg gw lp fn | Neurotoxin Coat: +40 `st.poisoned` power per hit; armor: +500 Bp resist toxin | techne |
| `mfr.post_apocalypse.rustmaw` | Rustmaw Scrapworks | RG RH MH | P+1500 | fd bh eh | magazine × 20000; every reload duration × 13000: `reload_ticks`, the shotgun's 20-tick start and 32-tick per-shell interval, and the 18-tick nock | none |
| `mfr.post_apocalypse.lastditch` | Lastditch Outfitters | RG RD ML MM | D−200 W−1000 | bs fq gw hl | 2000 Bp chance per shot to spend no ammo, or per swing to spend no stamina (keyed `rng.combat`, 1 `roll_bp` per shot or swing) | none |
| `mfr.post_apocalypse.geigerhaus` | Geiger & Daughters | RH CA AR SH WG MM | D+400 W+1500 | bh fn pr cc | Hot Load: 1500 Bp of each hit moves to `dmg.radiation` and +25 `st.irradiated` power; armor: +600 Bp resist radiation | techne |
| `mfr.weird_west.coffin_creek` | Coffin Creek Revolver Co. | RG RD | D+500 M−2000 L+1000 | br oi fq | crits ricochet 5000 Bp of their `D2` to the nearest other hostile within 8000 mm (Canon §10.6 step 3, no crit, no further ricochet) | verve |
| `mfr.weird_west.seven_seals` | Seven Seals Armory | RG MM CA ML | D+300 P+500 | bs es fc gw | 2000 Bp of each hit moves from its largest share to `dmg.umbral` | mythos |
| `mfr.weird_west.sundown_rail` | Sundown Rail & Powder | RH RG MH AR SH WG | D+800 S−700 | bh fe eh pr | Sticky Fuse: projectile hits leave a charge (max 6 per target); starting a reload detonates every charge for 3000 Bp of its hit's `D2` as `dmg.fire` within 2000 mm (Canon §10.6 environmental entry, no crit); melee: heavy attacks leave the charge and the next heavy attack detonates it; armor: +500 Bp resist fire | techne |
| `mfr.hardboiled_noir.midnight_avenue` | Midnight Avenue Arms | RG ML RD | H−3000 R−1500 | bs oi gw ek | shot noise capped at 50 dB; melee swing noise capped at 40 dB | none |
| `mfr.hardboiled_noir.rainslick` | Rainslick & Co. | MM MH RG | O+2000 D+300 | eh gg bh | × 12000 against `st.staggered` or `st.knocked_down` targets | none |
| `mfr.hardboiled_noir.halloran` | Halloran Typewriter & Arms | RG RH AR SH WG CA | S+1000 P+1500 D−500 | fd fe or lp | Typewriter: attack speed +500 Bp per 10 consecutive shots, max +2000, reset after 30 ticks without firing; armor: +2 poise | none |
| `mfr.wuxia.jade_lotus` | Jade Lotus Pavilion | ML MM | S+800 D−400 W−2000 | ek hg gw | the weapon gains the doc 05 throw action (normal hit damage) and returns to the hand exactly 30 ticks after it leaves the hand, wherever it is; `input.throw` throws it when no throwable is on the quickbar | verve |
| `mfr.wuxia.iron_crane` | Iron Crane School | MM MH SH AR | T−1500 D+200 | gg hl eh pr | a block started within 6 ticks before impact reflects 5000 Bp of the hit's `D2` (Canon §10.6 reflected) | verve |
| `mfr.wuxia.cloudpiercer` | Cloudpiercer Hall | RD RG RH CA WG | X+1000 R+1500 D−300 | bl os fc cc | Acupoint: precision-part hits add 40 `st.stunned` power | none |
| `mfr.steampunk.brassworth` | Brassworth & Cog | RG MM ML | S+300 | br fq ek gw | +200 Bp per consecutive hit, max +3000; a hit is consecutive within 120 ticks of the previous hit; a miss or 120 ticks resets | techne |
| `mfr.steampunk.aetherline` | Aetherline Galvanic | RG RH CA WG | D−300 M+2000 | bl fe fn cc | every 4th hit chains 3000 Bp of its `D2` as `dmg.shock` to the 2 nearest other hostiles within 5000 mm (Canon §10.6 step 3, `no_chain`); ward generators: ward break emits the chain from the bearer for 3000 Bp of capacity | techne |
| `mfr.steampunk.boilerheart` | Boilerheart Works | MH RH AR SH RD | D+900 S−800 W+2500 | bh eh pr fd | Pressure Valve: 3 steam charges, +1 per 120 ticks; a heavy attack or alt-fire spends 1 for +5000 Bp and places one `gas.steam` cell at impact; armor: +2 poise | techne |
| `mfr.fairy_tale.thimbleweald` | Thimbleweald Tailors | ML MM RD MH | W−2500 D−400 | ek hg bs | × 13000 against targets with a larger capsule radius than the wielder | none |
| `mfr.fairy_tale.thornwood` | Thornwood Coven | MM RD CA WG | O+1000 S−200 | hl bl fc cc | 1000 Bp chance per hit to apply `st.rooted` (keyed `rng.status`); ward generators: ward break roots non-boss hostiles within 3000 mm | arcana |
| `mfr.fairy_tale.glass_cobbler` | The Glass Cobbler | ML AR SH RG RH | V+2000 | ek es br pr | Glass Edge: +2500 Bp while the bearer's HP is at least 5000 Bp of Max HP, else nothing; armor: AR × 12000 under the same condition | arcana |
| `mfr.mythic_antiquity.hephaesteum` | Hephaesteum | MH MM AR SH RH | D+300 W+1000 | eh hl pr bh | the bearer is immune to `st.disarmed` and `st.corroded` | mythos |
| `mfr.mythic_antiquity.argive` | Argive Arms | MM SH RD AR ML | D+200 T−1000 | hl gg bl pr | +1500 Bp while another allied Erratum's origin is within 3000 mm; armor and shields: +1000 Bp resist slash, pierce and blunt under the same condition | none |
| `mfr.mythic_antiquity.delphic` | Delphic Workshops | CA RD WG RG | X+1500 S−300 | fc os br cc | Foresight: the first hit on each target in a Folio uses crit chance 10000 (the crit draw is still made); ward generators: the first hit taken each Folio is fully absorbed | mythos |
| `mfr.oneiric.somnambule` | Somnambule | RD RG RH CA AR SH WG | R+2000 D−300 | bl fc or | after 10000 mm of flight, projectiles (spell projectiles included) pass through destructible and non-opaque blocks without spending penetration; fixed terrain still stops them (Canon §4.3) | arcana |
| `mfr.redacted.bureau` | The Bureau | all classes | P−1000 D−300 | br fe os ek gg | the base component never rolls Contradiction failure; potency loss still applies | techne |
| `mfr.ur_text.first_press` | The First Press | Palimpsestic only | none | none | none beyond Canon §13.3 Palimpsestic rules | none |
| `mfr.errata.marginal_press` | The Marginal Press | all classes | none | all | Stubborn Margin: while the base component has `C` in `1..=5`, +800 × `C` Bp; the gimmick has dependency none, so it never loses potency | none |

Doc 05 §10.5 states the combat hooks of `mfr.mythic_fantasy.deepanvil`, `mfr.gothic_horror.grimhallow`, `mfr.space_opera.perihelion`, `mfr.space_opera.umbra`, `mfr.cyberpunk.neon_saint`, `mfr.post_apocalypse.rustmaw`, `mfr.post_apocalypse.lastditch`, `mfr.wuxia.jade_lotus`, `mfr.mythic_antiquity.hephaesteum`, `mfr.oneiric.somnambule` and `mfr.redacted.bureau`; every other gimmick uses the general hooks of §5.1.

**Identity** (one line each; doc 18 writes the flavor text from it):

| ID | Identity |
|---|---|
| `mfr.mythic_fantasy.deepanvil` | under-mountain clan forge; heavy, slow, hard-hitting steel |
| `mfr.mythic_fantasy.silverbough` | elven fletchers and leatherworkers; light, silent, quick |
| `mfr.mythic_fantasy.runecourt` | wizard guild that rune-etches blades and staves to drink aether back |
| `mfr.gothic_horror.vesper_sons` | church armorers who silver every edge |
| `mfr.gothic_horror.grimhallow` | blood-priced gunsmiths whose reloads cost the shooter |
| `mfr.gothic_horror.ashgrave` | cemetery ironworks; plate that hardens as its wearer nears the grave |
| `mfr.cosmic_horror.hollow_lantern` | lighthouse lens-grinders whose optics mark what they see |
| `mfr.cosmic_horror.outer_choir` | cult workshop of instruments that sing in the mind |
| `mfr.cosmic_horror.dunwold` | polar expedition outfitters; gear built for long marches in bad weather |
| `mfr.space_opera.perihelion` | fleet armory of charged-shot energy rifles |
| `mfr.space_opera.umbra` | heavy-industry conglomerate of heat-cycled, magazine-free guns |
| `mfr.space_opera.starwright` | psionic academy that forges thought-echoing blades |
| `mfr.cyberpunk.ghostline` | security contractor of smart, homing ordnance |
| `mfr.cyberpunk.neon_saint` | street customizers of overclocked, cover-piercing guns |
| `mfr.cyberpunk.chromaw` | biotech clinic that coats its edges in engineered venom |
| `mfr.post_apocalypse.rustmaw` | scrap-yard gunsmiths: huge magazines, slow reloads |
| `mfr.post_apocalypse.lastditch` | survivalist outfitters who waste nothing |
| `mfr.post_apocalypse.geigerhaus` | reactor salvagers who load irradiated slugs |
| `mfr.weird_west.coffin_creek` | frontier revolver works with trick-shot ricochets |
| `mfr.weird_west.seven_seals` | hex-sealed armory of the dark prairie |
| `mfr.weird_west.sundown_rail` | railroad demolition crew; sticky fuses on everything |
| `mfr.hardboiled_noir.midnight_avenue` | quiet guns for quiet jobs |
| `mfr.hardboiled_noir.rainslick` | brass-knuckle bruisers who punish the fallen |
| `mfr.hardboiled_noir.halloran` | typewriter factory turned arms maker; rattling rate of fire |
| `mfr.wuxia.jade_lotus` | sword sect whose weapons fly back to the hand |
| `mfr.wuxia.iron_crane` | defensive school of deflecting blocks |
| `mfr.wuxia.cloudpiercer` | archery hall of acupoint-striking marksmen |
| `mfr.steampunk.brassworth` | precision clockwork that builds momentum |
| `mfr.steampunk.aetherline` | galvanic company of chaining arcs |
| `mfr.steampunk.boilerheart` | boiler-driven heavy plant with steam-charged strikes |
| `mfr.fairy_tale.thimbleweald` | tiny tailors whose needles slay giants |
| `mfr.fairy_tale.thornwood` | hedge-witch coven of rooting thorns |
| `mfr.fairy_tale.glass_cobbler` | enchanted glasswork: sharp and beautiful while its bearer is whole |
| `mfr.mythic_antiquity.hephaesteum` | the divine forge; its bearers are never disarmed or corroded |
| `mfr.mythic_antiquity.argive` | phalanx armorers who fight shoulder to shoulder |
| `mfr.mythic_antiquity.delphic` | oracle workshops whose weapons foresee the first blow |
| `mfr.oneiric.somnambule` | dream-makers whose long shots drift through doors, glass and flimsy cover |
| `mfr.redacted.bureau` | the Editors' quartermasters; equipment that never misfires |
| `mfr.ur_text.first_press` | the first printing; stamps only Palimpsestic items |
| `mfr.errata.marginal_press` | Errata who print their own gear in the margins; it fights harder where it does not belong |

### 5.3 Manufacturer draw

Candidates enter `weighted` in ascending content-ID order, so `mfr.errata.marginal_press` precedes `mfr.<genre>.*`. Every other weighted list in this chapter is likewise in ascending content-ID order unless a table fixes the order (deck cards, rarities, qualities, Engine faces).

| Item | Candidates and weights |
|---|---|
| Core genre `g`, non-Palimpsestic, non-Exotic | each of `g`'s three manufacturers whose classes include the template's class: 3000; `mfr.errata.marginal_press`: 1000 |
| `genre.oneiric` | `mfr.oneiric.somnambule` 9000 when its classes match, else 0; Marginal Press 1000 |
| `genre.redacted` | `mfr.redacted.bureau` 9000; Marginal Press 1000 |
| `genre.ur_text` | Marginal Press 10000 |
| `genre.unwritten` | none (no draw) |
| Any Palimpsestic item | `mfr.ur_text.first_press` (no draw) |
| Exotic | the Exotic template's fixed manufacturer (no draw), whatever the genre of the source |

Example: a Weird West SMG (class RG) has candidates Marginal Press 1000, Coffin Creek 3000, Seven Seals 3000, Sundown Rail 3000, total 10000 (§20).

### 5.4 Loose parts

Doc 07 owns part mechanics and assembly. This chapter places them as loot. Codes used in §5.2:

| Code | Part ID | Modifier | Code | Part ID | Modifier |
|---|---|---|---|---|---|
| bs | `part.barrel.short` | H −2000, R −1500 | ek | `part.edge.keen` | pen +800 |
| bl | `part.barrel.long` | R +2500, H +1000 | eh | `part.edge.heavy` | O +3000, S −400 |
| bh | `part.barrel.heavy` | D +600, S −500 | es | `part.edge.serrated` | +20 `st.bleeding` power |
| br | `part.barrel.rifled` | P −1500 | hl | `part.haft.long` | reach +200 mm, S −300 |
| fe | `part.feed.extended` | M +3000 | hg | `part.haft.light` | S +500, D −300 |
| fq | `part.feed.quick` | L −1500 | gw | `part.grip.wrapped` | T −1200 |
| fd | `part.feed.drum` | M +5000, H +2000 | gg | `part.grip.guarded` | +800 Bp shield stability while blocking |
| oi | `part.optic.iron` | X +500 | fc | `part.focus.crystal` | +600 Bp spell damage |
| os | `part.optic.scope` | X +1500, H +1500 | fn | `part.focus.conduit` | cast ticks × 9000 |
| or | `part.optic.reflex` | P −800 | pr | `part.plate.reinforced` | AR × 11500, W +1000 |
| lp | `part.lining.padded` | +400 Bp resist blunt | cc | `part.coil.capacitor` | ward capacity × 11500 |

A loose part instance has `category = cat.part`, a `manufacturer`, `rarity = rar.common`, `base_value` 25 (so `value = mul_bp(25, P(ilvl))`), mass 300 g, stack 5 and dependency none. Assembly (doc 07) applies a part's modifier as a tendency of the same letter. Loose-part template draw: uniform over the drawn manufacturer's part list (1 `weighted` with equal weights 100 in `loot.arch`); a part drawn from the deck card `cat.part` first draws its manufacturer with equal weights over the manufacturers its origin genre's §5.3 row lists, ignoring class (core genres: their three and the Marginal Press; Oneiric: Somnambule and the Marginal Press; Redacted: the Bureau and the Marginal Press; Ur-Text: the Marginal Press), 1 `weighted` in `loot.mfr`. `genre.unwritten` has no manufacturers, so the `cat.part` card has no eligible template there (§2.3 step 1).

---

## 6. Affixes

### 6.1 Eligibility, tiers and values

- **Count** per rarity: Canon §13.3 (0, 1, 2, 3, 3, 2, 4). Exotic and Palimpsestic affixes roll like any other.
- **Eligible** for an item when all hold: the item's category letter is in Cats (W melee, R ranged weapon including staff and thrown, N wand, A armor, O off-hand, C accessory and implant, G ward generator); no affix of the same Group is already on the item; every Requires tag is on the item (§3.2 tags; `pool:<x>` = the run content set holds `loot.pool.<x>`; `min Tn` = ilvl ≥ the gate of tier n); and the dependency class is legal in `origin_genre`.
- **Dependency class** → component dependency (Canon §5.3): mundane → none; infused → `{origin_genre.aether_axiom}`, or arcana when none; tech → techne; sacred → mythos; cine → verve; base → the item's base-function dependency; gloss → the gimmick's own dependency, with requirement computed from the gloss manufacturer's genre. A class whose axis has axiom 0 in `origin_genre` is illegal there (Canon §5.3).
- **Tier gates** by `ilvl`: T1 1, T2 4, T3 9, T4 15, T5 21. Marginalia items reach T5 like ilvl-21 items. **Tier weights** over unlocked tiers: `[10000, 6000, 3500, 1800, 800]`; for Epic or better, the T1 weight is `mul_bp(10000, 5000)`.
- **Draw order** inside `loot.affix`, per affix: `weighted` over eligible affixes sorted by ID (weights = Wt), `weighted` over tiers, `range(10001)` for `roll_bp`.
- **Value**: `X = lo + floor_div((hi - lo) × roll_bp, 10000)`. Each affix contributes `mul_bp(X, its potency)` where a Bp contribution is scaled (Canon §5.5); flat counts (sub-attributes, dB, mm, ticks, charges, tolerance) are 0 when the affix is Disconnected and unscaled otherwise.
- **Display**: affix values are hidden until the affix's knowledge flag is set (doc 10).
- **Curse affixes** (`curse.*`) belong to doc 10 and are outside the count.
- **Chance affixes** (`affix.arc_proc`, `affix.frugal`) roll keyed `rng.combat` once per hit or shot, always (Canon §3.4), in the doc 05 §1.4 proc slot (ascending affix and perk ID order after the crit roll).

Worked examples: `affix.dmg_vs_beast` tier 2, `roll_bp = 1146`: `X = 1800 + floor_div(700 × 1146, 10000) = 1880` Bp. The descending `affix.self_injector` tier 2, `roll_bp = 4095`: `X = 900 + floor_div(-200 × 4095, 10000) = 900 - 82 = 818` ticks.

### 6.2 Affix table (113 affixes)

| ID | Group | Cats | Class | Effect (X = rolled value) | T1 | T2 | T3 | T4 | T5 | Wt | Requires |
|---|---|---|---|---|---|---|---|---|---|---|---|
| `affix.dmg_pct` | dmg_pct | WR | mundane | +X Bp damage (additive bucket) | 400–600 | 600–900 | 900–1200 | 1200–1600 | 1600–2100 | 120 | — |
| `affix.dmg_vs_elite` | dmg_vs_rank | WR | mundane | +X Bp vs elite, champion, boss, editor, hunter | 800–1200 | 1200–1700 | 1700–2300 | 2300–3000 | 3000–3800 | 60 | — |
| `affix.dmg_vs_undead` | dmg_vs_tag | WR | sacred | +X Bp vs undead or fiend | 1200–1800 | 1800–2500 | 2500–3300 | 3300–4300 | 4300–5500 | 40 | — |
| `affix.dmg_vs_construct` | dmg_vs_tag | WR | tech | +X Bp vs body.construct | 1200–1800 | 1800–2500 | 2500–3300 | 3300–4300 | 4300–5500 | 40 | — |
| `affix.dmg_vs_beast` | dmg_vs_tag | WR | mundane | +X Bp vs fac.wild.* | 1200–1800 | 1800–2500 | 2500–3300 | 3300–4300 | 4300–5500 | 40 | — |
| `affix.dmg_vs_editor` | dmg_vs_tag | WR | sacred | +X Bp vs fac.editors | 1200–1800 | 1800–2500 | 2500–3300 | 3300–4300 | 4300–5500 | 30 | — |
| `affix.interloper` | dmg_vs_origin | WR | mundane | +X Bp vs native targets (Canon §5.6) | 600–900 | 900–1300 | 1300–1800 | 1800–2400 | 2400–3000 | 30 | — |
| `affix.crit_chance` | crit_chance | WRC | mundane | +X Bp crit chance (Canon §6.4 row 13 gear) | 100–200 | 200–300 | 300–400 | 400–550 | 550–700 | 80 | — |
| `affix.crit_mult` | crit_mult | WRC | mundane | +X Bp crit multiplier (row 14 gear) | 1000–1500 | 1500–2000 | 2000–2700 | 2700–3500 | 3500–4500 | 70 | — |
| `affix.precision` | precision | WR | mundane | +X Bp to the weapon precision multiplier | 500–800 | 800–1200 | 1200–1700 | 1700–2300 | 2300–3000 | 60 | — |
| `affix.penetration` | pen | WR | mundane | +X Bp pen_bp | 500–800 | 800–1100 | 1100–1500 | 1500–2000 | 2000–2600 | 60 | — |
| `affix.rate` | rate | WR | mundane | attack speed +X Bp | 300–500 | 500–700 | 700–900 | 900–1200 | 1200–1500 | 70 | — |
| `affix.reload` | reload | R | mundane | reload ticks ×(10000 − X) | 800–1200 | 1200–1600 | 1600–2000 | 2000–2500 | 2500–3000 | 60 | magazine |
| `affix.magazine` | magazine | R | mundane | magazine ×(10000 + X), at least +1 | 1500–2500 | 2500–3500 | 3500–4500 | 4500–6000 | 6000–8000 | 60 | magazine |
| `affix.spread` | spread | R | mundane | base spread ×(10000 − X) | 800–1200 | 1200–1600 | 1600–2000 | 2000–2500 | 2500–3200 | 50 | — |
| `affix.recoil` | recoil | R | mundane | recoil ×(10000 − X) | 800–1200 | 1200–1600 | 1600–2000 | 2000–2500 | 2500–3200 | 40 | firearm |
| `affix.range` | range | R | mundane | effective range ×(10000 + X) | 1000–1500 | 1500–2200 | 2200–2800 | 2800–3500 | 3500–4500 | 40 | — |
| `affix.stamina` | stamina | W | mundane | melee stamina cost ×(10000 − X) | 800–1200 | 1200–1600 | 1600–2000 | 2000–2500 | 2500–3200 | 60 | — |
| `affix.poise_dmg` | poise_dmg | WR | mundane | poise damage ×(10000 + X) | 1500–2500 | 2500–3500 | 3500–4500 | 4500–6000 | 6000–8000 | 50 | — |
| `affix.reach` | reach | W | mundane | melee reach +X mm | 100–150 | 150–200 | 200–260 | 260–330 | 330–400 | 40 | — |
| `affix.handling` | handling | WR | mundane | swap and draw ticks ×(10000 − X) | 1000–1500 | 1500–2200 | 2200–2800 | 2800–3500 | 3500–4500 | 40 | — |
| `affix.infuse_<t>` ×11: fire, frost, shock, acid, radiation, sonic, toxin, radiant, umbral, psychic, arcane | infusion | WR | fire, frost, shock, psychic, arcane infused; acid, toxin mundane; radiation, sonic tech; radiant, umbral sacred | moves X Bp of the hit from its largest physical share (from its largest share when it has no physical share; nothing when that share is already `dmg.<t>`) to `dmg.<t>`; +`floor_div(X, 70)` power of `<t>`'s status | 1000–1500 | 1500–2000 | 2000–2600 | 2600–3300 | 3300–4000 | 25 | — |
| `affix.bleed_power` | bleed_power | WR | mundane | +X st.bleeding power per hit | 15–22 | 22–30 | 30–40 | 40–52 | 52–65 | 50 | edged |
| `affix.stun_power` | stun_power | W | mundane | +X st.stunned power per hit | 15–22 | 22–30 | 30–40 | 40–52 | 52–65 | 50 | blunt |
| `affix.expose_power` | expose_power | WR | mundane | +X st.exposed power per hit | 15–22 | 22–30 | 30–40 | 40–52 | 52–65 | 40 | pierce |
| `affix.leech` | sustain | WR | sacred | heal X Bp of HP damage dealt | 100–150 | 150–220 | 220–300 | 300–400 | 400–500 | 30 | — |
| `affix.kill_heal` | sustain | WR | mundane | on kill heal X Bp of Max HP | 150–250 | 250–400 | 400–550 | 550–750 | 750–1000 | 40 | — |
| `affix.kill_stamina` | kill_res | W | mundane | on kill restore X stamina | 5–8 | 8–12 | 12–16 | 16–21 | 21–27 | 40 | — |
| `affix.kill_aether` | kill_res | WRN | infused | on kill restore X aether | 3–5 | 5–7 | 7–10 | 10–13 | 13–17 | 40 | — |
| `affix.kill_haste` | kill_buff | WR | cine | on kill apply st.hasted for X ticks | 60–90 | 90–120 | 120–150 | 150–200 | 200–260 | 30 | — |
| `affix.arc_proc` | proc | WR | tech | X Bp chance per hit to arc 3000 Bp of D2 as dmg.shock (4 m) | 300–500 | 500–700 | 700–900 | 900–1200 | 1200–1500 | 30 | — |
| `affix.ricochet` | proc | WR | cine | crits ricochet X Bp of their D2 (8 m) | 2000–2800 | 2800–3600 | 3600–4400 | 4400–5200 | 5200–6000 | 30 | — |
| `affix.executioner` | dmg_vs_state | WR | mundane | +X Bp vs targets below 3000 Bp HP | 1000–1500 | 1500–2200 | 2200–3000 | 3000–4000 | 4000–5000 | 40 | — |
| `affix.ambusher` | dmg_vs_state | WRC | mundane | +X Bp on sneak attacks | 1000–1500 | 1500–2500 | 2500–3500 | 3500–4500 | 4500–5500 | 40 | — |
| `affix.first_strike` | dmg_vs_state | WR | cine | +X Bp on the first hit against each target | 1000–1500 | 1500–2200 | 2200–3000 | 3000–4000 | 4000–5000 | 30 | — |
| `affix.quiet` | quiet | WR | mundane | attack noise −X dB | 4–6 | 6–9 | 9–12 | 12–16 | 16–20 | 40 | — |
| `affix.frugal` | ammo | R | mundane | X Bp chance per shot to spend no ammo | 500–800 | 800–1200 | 1200–1800 | 1800–2500 | 2500–3200 | 30 | ammo |
| `affix.buildup` | buildup | WR | mundane | every status power of hits ×(10000 + X) | 1000–1500 | 1500–2200 | 2200–3000 | 3000–4000 | 4000–5000 | 30 | — |
| `affix.spell_dmg` | spell_dmg | RNOC | infused | +X Bp damage for spell sources | 500–800 | 800–1200 | 1200–1600 | 1600–2100 | 2100–2600 | 80 | casting |
| `affix.cast_speed` | cast_speed | RNOC | infused | cast ticks ×(10000 − X) | 500–800 | 800–1100 | 1100–1500 | 1500–2000 | 2000–2500 | 50 | casting |
| `affix.aether_cost` | aether_cost | RNOC | infused | spell aether cost ×(10000 − X) | 500–800 | 800–1100 | 1100–1500 | 1500–2000 | 2000–2500 | 50 | casting |
| `affix.summon_vigor` | summon | RNOC | infused | summon Max HP ×(10000 + X) | 1000–1500 | 1500–2200 | 2200–3000 | 3000–4000 | 4000–5000 | 30 | casting |
| `affix.charges` | charges | N | mundane | +X max charges | 1 | 1–2 | 2 | 2–3 | 3–4 | 60 | — |
| `affix.recharge` | recharge | N | infused | charge recharge ticks ×(10000 − X) | 1000–1500 | 1500–2200 | 2200–2800 | 2800–3500 | 3500–4500 | 40 | — |
| `affix.armor_pct` | armor_pct | AO | mundane | this piece AR ×(10000 + X) | 500–800 | 800–1100 | 1100–1500 | 1500–2000 | 2000–2600 | 120 | armored |
| `affix.poise` | poise | A | mundane | +X armor poise | 2–3 | 3–5 | 5–7 | 7–9 | 9–12 | 60 | — |
| `affix.stability` | stability | O | mundane | +X Bp shield stability (row 19) | 300–500 | 500–800 | 800–1100 | 1100–1500 | 1500–2000 | 60 | shield |
| `affix.resist_<t>` ×14: every `dmg.` type except true | resist_<t> | AOCG | radiant, umbral sacred; psychic, arcane infused; others mundane | +X Bp resist `dmg.<t>` | 300–500 | 500–700 | 700–1000 | 1000–1300 | 1300–1700 | physical 40, fire, frost, shock, acid, toxin 50, others 40 | — |
| `affix.resist_elements` | resist_elements | AOC | mundane | +X Bp resist fire, frost and shock each | 150–250 | 250–400 | 400–550 | 550–700 | 700–900 | 30 | — |
| `affix.resist_all` | resist_all | AC | mundane | +X Bp resist every type except true | 100–150 | 150–220 | 220–300 | 300–380 | 380–450 | 15 | min T2 |
| `affix.thorns` | thorns | AO | mundane | reflect X Bp of melee D2 taken (Canon §10.6) | 500–800 | 800–1200 | 1200–1700 | 1700–2400 | 2400–3200 | 40 | — |
| `affix.engineered` | engineered | WRAOG | mundane | salvaging this item (§16.4) yields each component material ×(10000 + X) | 2000–3000 | 3000–4500 | 4500–6000 | 6000–8000 | 8000–10000 | 60 | — |
| `affix.featherweight` | mass | WRAOCG | mundane | item mass ×(10000 − X) | 1000–1500 | 1500–2200 | 2200–2800 | 2800–3500 | 3500–4500 | 50 | — |
| `affix.self_injector` | injector | WRAOG | tech | every X ticks the bearer heals `mul_bp(max_hp_mhp, mul_bp(100, pot))` mHP (Canon §10.9; `pot` = this affix's potency, nothing while Disconnected); injectors do not add: only the smallest X among the bearer's equipped items applies, counted from the last injector heal | 1200–900 | 900–700 | 700–500 | 500–380 | 380–300 | 20 | — |
| `affix.ward_capacity` | ward_capacity | G | base | ward capacity ×(10000 + X) | 1000–1500 | 1500–2200 | 2200–3000 | 3000–4000 | 4000–5500 | 120 | — |
| `affix.ward_siphon` | ward_siphon | G | base | on kill restore X Bp of ward capacity | 500–800 | 800–1200 | 1200–1700 | 1700–2400 | 2400–3200 | 60 | — |
| `affix.ward_nova` | ward_nova | G | base | on ward break stagger non-boss hostiles within X mm | 2000–2200 | 2200–2500 | 2500–2900 | 2900–3400 | 3400–4000 | 40 | — |
| `affix.ward_full` | ward_full | G | base | +X Bp damage while ward is full | 300–500 | 500–800 | 800–1100 | 1100–1500 | 1500–2000 | 40 | — |
| `affix.sub_<name>` ×24: one per sub-attribute (`affix.sub_power` … `affix.sub_providence`) | sub_<name> | AOC | mundane | +X effective sub-attribute | 1–2 | 2–3 | 3–4 | 4–6 | 6–8 | 30; FAT, SER, PRV 20 | — |
| `affix.tol_<axis>` ×4: arcana, techne, mythos, verve | tolerance | AC | mundane | `gear_tol[axis]` +1 (Canon §5.4) | 1 | 1 | 1 | 1 | 1 | 15 | min T3 |
| `affix.lucre_find` | lucre | AC | mundane | +X Bp in the bearer's Lucre-find sum (§9.1, capped) | 300–500 | 500–800 | 800–1100 | 1100–1500 | 1500–2000 | 30 | — |
| `affix.soft_step` | soft_step | A | mundane | footstep noise −X dB | 3–4 | 4–6 | 6–8 | 8–10 | 10–12 | 40 | legs_feet |
| `affix.radiance` | radiance | OC | mundane | emits light_block X | 4 | 5 | 6 | 7 | 8 | 20 | — |
| `affix.gloss` | gloss | WR | gloss | another core genre manufacturer gimmick as a component | 1 | 1 | 1 | 1 | 1 | 20 | pool:cross_glosses min T3 |

### 6.3 Glosses

`affix.gloss` (pool `loot.pool.cross_glosses`, Bindery `unlock.loot.cross_glosses`) draws one extra value in `loot.affix`: a manufacturer chosen uniformly among the 36 core manufacturers whose genre differs from `origin_genre` and whose dependency axis is legal there (1 `weighted`, equal weights). The item gains that manufacturer's §5.2 gimmick as an extra component; `gloss_mfr` stores it. The component's requirement uses the gloss manufacturer's genre axioms, so a Weird West ricochet Gloss on a Wuxia Great Bow (`arch.ranged.charge_rifle`) needs `Req[verve] = max(1, 7 - 2) = 5`. A Gloss never grants tendencies or parts. Draw count for this affix: 4.

---

## 7. Weapon perks

### 7.1 Columns

Perk columns are Destiny-style and switchable, with options set by gear tier (DECISIONS 68). Every weapon and wand (families of §3.2) has up to five columns: 1 Frame (barrel, edge or conduit), 2 Feed (magazine, balance or reservoir), 3 Trait I, 4 Trait II, 5 Genre trait.

| Rarity | Columns rolled | Notes |
|---|---|---|
| Common | 1 | plus column 5 |
| Uncommon | 1, 2 | plus column 5 |
| Rare | 1, 2, 3 | plus column 5 |
| Epic, Legendary, Palimpsestic | 1, 2, 3, 4 | plus column 5; Legendary also carries its §8.1 perk outside the columns |
| Exotic | 1, 2 | plus column 5 and the fixed intrinsic (§8.2) |

- **Options per rolled column** by `gear_tier` (Canon §13.9): 1, 1, 2, 2, 3 for tiers 1 to 5. Column 5 always has exactly 1 option.
- **Pool** for a column: perks whose Col matches, whose Families contain the item's family, whose Req tag (§3.2) is on the item and whose dependency is legal in `origin_genre`, sorted by ID. Column 4 excludes every option already in column 3. Picks are weighted by Wt without replacement.
- **Choosing.** `chosen` starts at 0. Changing it is free at any Scriptorium station, including the Colophon's (§16.5). Only the chosen option is active.
- **Enhanced.** A bracketed magnitude `[n]` becomes `mul_bp(n, 12500)` when enhanced: columns 3 and 4 at gear tier ≥ 2, columns 1 and 2 at tier ≥ 4, column 5 at tier 5. Example: `perk.rampage` at tier 2 grants 1250 Bp per stack.
- Perk numbers enter the formula they name; "+N Bp" without a qualifier enters the additive bucket (Canon §10.5 step 1). Each perk is a component with the dependency shown ("aether" = the class `infused` of §6.1).

### 7.2 Perk table (51 perks) and genre traits (16)

| ID | Col | Families | Req | Dep | Wt | Effect ([n] = enhanced magnitude) |
|---|---|---|---|---|---|---|
| `perk.honed_edge` | 1 | E | — | none | 100 | pen_bp +[1500] |
| `perk.weighted_head` | 1 | EB | — | none | 100 | poise damage ×(10000 + [3000]); attack speed −300 Bp |
| `perk.serrated` | 1 | E | — | none | 100 | +[25] st.bleeding power per hit |
| `perk.balanced_tang` | 1 | EB | — | none | 100 | attack speed +[600] Bp |
| `perk.long_haft` | 1 | EB | — | none | 80 | reach +[250] mm; attack speed −300 Bp |
| `perk.rifled_bore` | 1 | DGH | — | none | 100 | base spread ×(10000 − [1500]) |
| `perk.long_barrel` | 1 | DGH | — | none | 100 | effective range ×(10000 + [2500]); handling ticks ×11000 |
| `perk.ported_barrel` | 1 | GH | — | none | 100 | recoil ×(10000 − [2000]) |
| `perk.heavy_bore` | 1 | DGH | — | none | 100 | +[700] Bp damage; attack speed −500 Bp |
| `perk.choke` | 1 | G | — | none | 60 | shotgun: pellet spread ×(10000 − [3000]); other guns: spread ×9000 |
| `perk.focused_conduit` | 1 | C | — | aether | 100 | +[600] Bp spell damage |
| `perk.swift_conduit` | 1 | C | — | aether | 100 | cast ticks ×(10000 − [1000]) |
| `perk.wide_conduit` | 1 | C | — | aether | 100 | spell area radius ×(10000 + [2000]) |
| `perk.light_grip` | 2 | EB | — | none | 100 | melee stamina cost ×(10000 − [1500]) |
| `perk.guarded_hilt` | 2 | EB | — | none | 100 | +[1000] Bp shield stability while blocking with this weapon |
| `perk.quick_draw` | 2 | EBDGH | — | none | 100 | swap and draw ticks ×(10000 − [3500]) |
| `perk.counterweight` | 2 | EB | — | none | 100 | heavy-attack stamina cost ×(10000 − [2000]) |
| `perk.tempered` | 2 | EBDGH | — | none | 80 | +[4] armor poise while this is the active weapon |
| `perk.extended_mag` | 2 | DGH | magazine | none | 100 | magazine ×(10000 + [3000]), at least +1 |
| `perk.quick_feed` | 2 | DGH | magazine | none | 100 | reload ticks ×(10000 − [1500]) |
| `perk.drum_feed` | 2 | GH | magazine | none | 80 | magazine ×(10000 + [5000]); handling ticks ×12000 |
| `perk.hot_loads` | 2 | DGH | — | none | 80 | +[500] Bp damage; attack noise +6 dB |
| `perk.steady_feed` | 2 | DGH | magazine | none | 80 | reloading does not slow movement; reload ticks ×(10000 − [500]) |
| `perk.deep_reservoir` | 2 | C | — | aether | 100 | wand: +[1] max charge; staff: aether cost ×9000 |
| `perk.quick_siphon` | 2 | C | — | aether | 100 | wand: recharge ticks ×(10000 − [1500]); staff: spell kills restore 2 aether |
| `perk.stable_reservoir` | 2 | C | — | aether | 100 | aether cost ×(10000 − [500]) and cast ticks ×(10000 − [500]) |
| `perk.rampage` | 3, 4 | EBDGHC | — | none | 100 | each kill: +1 stack (max 3) of +[1000] Bp damage for 300 ticks, refreshed |
| `perk.outlaw` | 3, 4 | DG | magazine | none | 100 | precision kill: next reload within 300 ticks ×(10000 − [5000]) ticks |
| `perk.kill_clip` | 3, 4 | DGH | magazine | none | 100 | reload started within 180 ticks of a kill: +[2500] Bp damage for 300 ticks |
| `perk.feeding_frenzy` | 3, 4 | DG | magazine | none | 80 | each kill +1 stack (max 5, 210 ticks): reload ticks ×(10000 − [1000]× stacks) |
| `perk.opening_shot` | 3, 4 | DG | — | none | 80 | first shot after 120 ticks without firing: spread ×5000 and +[1000] Bp precision multiplier |
| `perk.vorpal` | 3, 4 | EBDGHC | — | none | 100 | +[1000] Bp vs champion, boss, editor and hunter ranks |
| `perk.coup_de_grace` | 3, 4 | EBDGH | — | none | 100 | +[2000] Bp vs targets below 3000 Bp HP |
| `perk.headhunter` | 3, 4 | DG | — | none | 100 | +[2000] Bp precision multiplier |
| `perk.momentum` | 3, 4 | EB | — | none | 100 | after 30 ticks of sprinting, the next melee hit +[3000] Bp |
| `perk.adrenaline` | 3, 4 | EBDGH | — | none | 100 | each kill restores [10] stamina |
| `perk.bloodletting` | 3, 4 | EB | — | mythos | 60 | melee hits heal [150] Bp of HP damage dealt |
| `perk.chain_reaction` | 3, 4 | DGH | — | verve | 60 | kills explode: [3000] Bp of the killing D2 as dmg.fire, radius 2500 mm |
| `perk.trench_barrel` | 3, 4 | G | magazine | none | 80 | a melee hit loads the next 3 shots with +[2500] Bp |
| `perk.close_quarters` | 3, 4 | EBDGH | — | none | 100 | +[1500] Bp while 2 or more hostiles are within 8000 mm |
| `perk.dragonfly` | 3, 4 | DG | — | verve | 60 | precision kills: [2500] Bp of the hit as dmg.shock, radius 2000 mm |
| `perk.slipstream` | 3, 4 | DGH | magazine | verve | 60 | a dodge reloads the magazine fully; at most once per 300 ticks |
| `perk.unrelenting` | 3, 4 | EBDGHC | — | none | 80 | 3 kills within 300 ticks heal [500] Bp of Max HP |
| `perk.sharpshooter` | 3, 4 | DG | — | none | 100 | +[1500] Bp on hits beyond 20000 mm |
| `perk.cadence` | 3, 4 | EB | — | none | 100 | the 3rd consecutive hit within 90 ticks of the previous +[2000] Bp |
| `perk.sunder` | 3, 4 | EBH | — | none | 80 | +[35] st.exposed power per hit |
| `perk.frenzy` | 3, 4 | EBDGHC | — | none | 80 | after 600 ticks in_combat: +[1500] Bp damage and +1000 Bp attack speed until not in_combat |
| `perk.stillness` | 3, 4 | DGC | — | none | 80 | crouched and still for 60 ticks: spread ×6000 and +[1500] Bp precision multiplier |
| `perk.arcane_surge` | 3, 4 | C | — | aether | 100 | every [5]th spell cast through this item costs 0 aether |
| `perk.overflow` | 3, 4 | C | — | aether | 100 | spell kills restore [3] aether |
| `perk.ricochet_rounds` | 3, 4 | G | ricochet | none | 60 | `max_ricochets` becomes 2 (doc 05 §7.5) and each ricochet keeps [7500] Bp of the hit's damage factor instead of 6000 |

| ID | Genre trait | Dep | Effect |
|---|---|---|---|
| `perk.genre.mythic_fantasy` | Heroic Quest | arcana | +[1000] Bp vs elite rank and above |
| `perk.genre.gothic_horror` | Silvered | mythos | +[1500] Bp vs undead and fiend |
| `perk.genre.cosmic_horror` | Unspeakable | mythos | +[20] st.dread power per hit |
| `perk.genre.space_opera` | Overcharged Cell | techne | the first hit after each reload or after 120 ticks idle +[2500] Bp |
| `perk.genre.cyberpunk` | Smartlink | techne | +[1000] Bp precision multiplier; HUD marks precision parts |
| `perk.genre.post_apocalypse` | Scavenger | none | each kill restores [2000] Bp of a magazine (melee and weapons without a magazine: 5 stamina) |
| `perk.genre.weird_west` | Dead Man's Hand | verve | while HP ≤ 5000 Bp of Max HP: +[1000] Bp crit chance |
| `perk.genre.hardboiled_noir` | Hard-Boiled | none | +[1000] Bp while no allied Erratum is within 10000 mm |
| `perk.genre.wuxia` | Wind and Cloud | verve | the first hit within 60 ticks after a dodge or air jump +[2000] Bp |
| `perk.genre.steampunk` | Pressure Gauge | techne | +[1500] Bp while the magazine (melee and weapons without a magazine: stamina) is above 7500 Bp of max |
| `perk.genre.fairy_tale` | Once Upon a Time | arcana | the first hit on each target in a Folio +[3000] Bp |
| `perk.genre.mythic_antiquity` | Kleos | mythos | killing an elite or higher rank applies st.inspired for [600] ticks |
| `perk.genre.oneiric` | Lucid | arcana | every 600 ticks the next hit adds [40] st.sleeping power |
| `perk.genre.redacted` | Clearance | techne | +[1500] Bp vs targets that are not native (Canon §5.6) |
| `perk.genre.unwritten` | Blank Slate | none | kills of fac.editors members restore [500] Bp of Max HP |
| `perk.genre.ur_text` | Archetypal | none | +[800] Bp vs every target |

### 7.3 Gear tier extras and tuning

- Tier 5 armor and accessories gain a **tuning** choice at a Scriptorium: +2 to one sub-attribute and −2 to another (`{plus, minus}`), or `lowest3`: +1 to each of the three lowest effective sub-attributes of the wearer at the moment of choosing, ties to the earlier Canon §6.1 row, fixed until changed. Stored in `tuning`; changeable for free; a tuning is a gear modifier and counts against the Canon §6.2 cap of 150.
- Gear tier never changes rarity, affix count or power tables, so `ITEM_MULT_MAX` (Canon §13.8) is unaffected.

### 7.4 Column pools per archetype

The pools below (IDs without the `perk.` prefix) follow from §7.2 before the dependency filter; a genre whose axiom is 0 on an axis removes that axis's perks (for example `genre.redacted` removes the verve perks). Every pool keeps at least 3 options in columns 1 and 2 and at least 6 in columns 3 and 4, so tier 5 (3 options per column, column 4 excluding column 3) always fills.

| Archetypes | Column 1 | Column 2 | Columns 3 and 4 |
|---|---|---|---|
| dagger, blade_short, blade_long, blade_great, axe, axe_great, spear, halberd, whip (family E) | 5: balanced_tang, honed_edge, long_haft, serrated, weighted_head | 5: counterweight, guarded_hilt, light_grip, quick_draw, tempered | 11: adrenaline, bloodletting, cadence, close_quarters, coup_de_grace, frenzy, momentum, rampage, sunder, unrelenting, vorpal |
| mace, warhammer, quarterstaff, fist, chain (family B) | 3: balanced_tang, long_haft, weighted_head | 5: counterweight, guarded_hilt, light_grip, quick_draw, tempered | 11: adrenaline, bloodletting, cadence, close_quarters, coup_de_grace, frenzy, momentum, rampage, sunder, unrelenting, vorpal |
| charge_rifle, crossbow, thrown (family D) | 3: heavy_bore, long_barrel, rifled_bore | 3: hot_loads, quick_draw, tempered | 13: adrenaline, chain_reaction, close_quarters, coup_de_grace, dragonfly, frenzy, headhunter, opening_shot, rampage, sharpshooter, stillness, unrelenting, vorpal |
| pistol, pistol_heavy, rifle_marksman, shotgun, smg, sniper (family G) | 5: choke, heavy_bore, long_barrel, ported_barrel, rifled_bore | 7: drum_feed, extended_mag, hot_loads, quick_draw, quick_feed, steady_feed, tempered | 19: adrenaline, chain_reaction, close_quarters, coup_de_grace, dragonfly, feeding_frenzy, frenzy, headhunter, kill_clip, opening_shot, outlaw, rampage, ricochet_rounds, sharpshooter, slipstream, stillness, trench_barrel, unrelenting, vorpal |
| rifle_auto (family H) | 4: heavy_bore, long_barrel, ported_barrel, rifled_bore | 7: drum_feed, extended_mag, hot_loads, quick_draw, quick_feed, steady_feed, tempered | 11: adrenaline, chain_reaction, close_quarters, coup_de_grace, frenzy, kill_clip, rampage, slipstream, sunder, unrelenting, vorpal |
| beam (family H) | 4: heavy_bore, long_barrel, ported_barrel, rifled_bore | 7: drum_feed, extended_mag, hot_loads, quick_draw, quick_feed, steady_feed, tempered | 11: adrenaline, chain_reaction, close_quarters, coup_de_grace, frenzy, kill_clip, rampage, slipstream, sunder, unrelenting, vorpal |
| launcher (family H) | 4: heavy_bore, long_barrel, ported_barrel, rifled_bore | 3: hot_loads, quick_draw, tempered | 9: adrenaline, chain_reaction, close_quarters, coup_de_grace, frenzy, rampage, sunder, unrelenting, vorpal |
| staff, wand (family C) | 3: focused_conduit, swift_conduit, wide_conduit | 3: deep_reservoir, quick_siphon, stable_reservoir | 7: arcane_surge, frenzy, overflow, rampage, stillness, unrelenting, vorpal |

---

## 8. Legendary perks, Exotics and Palimpsestic uniques

### 8.1 Legendary perks (26)

A Legendary item draws one perk (`loot.legendary`, 1 `weighted`, equal weights 100, ascending ID) from the pool of its category group whose pool tag is owned and whose dependency is legal in `origin_genre`. Group W covers both weapon categories and wands; A armor; X accessories, implants, off-hands and ward generators (ward-only perks marked). Pool sizes match doc 14 §5.3: 6 base, 10 in `legendary_perks_1`, 10 in `legendary_perks_2`.

| ID | Group | Pool | Dep | Effect |
|---|---|---|---|---|
| `perk.leg.echoing_strike` | W | base | none | every 4th hit repeats 20 ticks later as a Canon §10.6 step-3 hit with `D2` = 5000 Bp of the original, no crit |
| `perk.leg.bloodsong` | W | base | mythos | each kill applies `st.empowered` directly |
| `perk.leg.worldbreaker` | W | legendary_perks_1 | none | block damage × 30000 (doc 03) against blocks tagged `block.tag.destructible`; such blocks of hardness ≤ 3 break on hit |
| `perk.leg.inkdrinker` | W | legendary_perks_1 | mythos | hits on `fac.editors` heal 500 Bp of damage dealt; each such kill removes 1 `st.erasure` stack from the wielder |
| `perk.leg.overture` | W | legendary_perks_1 | verve | the first hit after `in_combat` turns true +10000 Bp |
| `perk.leg.recursion` | W | legendary_perks_1 | none | kills reset the attack interval and restore 5000 Bp of the magazine |
| `perk.leg.hunters_mark` | W | legendary_perks_1 | none | precision-part hits apply one `st.exposed` stack directly |
| `perk.leg.last_stand` | W | legendary_perks_2 | verve | while HP ≤ 2500 Bp of Max HP: +4000 Bp and attack speed +2000 Bp |
| `perk.leg.mirror_edge` | W | legendary_perks_2 | verve | a block started within 6 ticks before impact grants +5000 Bp to the next hit within 120 ticks |
| `perk.leg.stormcall` | W | legendary_perks_2 | aether | crits strike the target with 4000 Bp of the crit's `D2` as `dmg.shock` (Canon §10.6 step 3) |
| `perk.leg.bulwark` | A | base | none | after 30 ticks without moving, every equipped armor piece's AR × 12000 |
| `perk.leg.second_skin` | A | base | none | a hit of at least 2000 Bp of Max HP applies `st.fortified` directly; once per 600 ticks |
| `perk.leg.thornmail` | A | legendary_perks_1 | none | melee attackers take 2000 Bp of the hit's `D2` reflected (Canon §10.6) |
| `perk.leg.phoenix_lining` | A | legendary_perks_1 | mythos | once per Folio, when HP first falls below 2000 Bp of Max HP, heal 3000 Bp of Max HP (Canon §10.9) |
| `perk.leg.shadowweave` | A | legendary_perks_1 | none | crouched with `light_at ≤ 4`: sight detection of the wearer × 7000 (doc 09) |
| `perk.leg.adamant` | A | legendary_perks_1 | none | immune to `st.corroded` and `st.knocked_down` |
| `perk.leg.mirror_plate` | A | legendary_perks_2 | aether | spell hits taken reflect 2000 Bp of their `D2` to the caster |
| `perk.leg.windrunner` | A | legendary_perks_2 | verve | dodge stamina cost × 5000 |
| `perk.leg.grounded` | A | legendary_perks_2 | none | +10 poise; no `st.staggered` from poise while blocking |
| `perk.leg.cold_blood` | A | legendary_perks_2 | none | composure loss × 7000 (doc 04) |
| `perk.leg.fortunes_favor` | X | base | none | +4 SER; each qualifying roll adds 2 to the bearer's §9.4 counters |
| `perk.leg.overcapacitor` | X (ward) | base | base | ward capacity × 13000; ward break applies `st.hasted` for 120 ticks |
| `perk.leg.lifeline` | X | legendary_perks_1 | none | kills heal 300 Bp of Max HP |
| `perk.leg.spellweaver` | X | legendary_perks_2 | aether | every 3rd spell cast +3000 Bp |
| `perk.leg.cartographer` | X | legendary_perks_2 | none | at Folio entry the HUD map reveals every room and Reliquary |
| `perk.leg.redoubt` | X (ward) | legendary_perks_2 | base | while ward > 0, immune to `st.stunned` and `st.feared` |

### 8.2 Exotics (24)

Exotics are unique templates `item.exotic.*` with a fixed archetype, home genre (always their `origin_genre` at generation, whatever the local genre), manufacturer and intrinsic component. They obey the Canon §13.3 one-per-group equip limit and roll 2 affixes; weapons and the wand also roll perk columns 1, 2 and 5. Rewrite (§15) changes their skin and origin genre but keeps name, manufacturer and intrinsic. An Exotic counts as acquired by a player when it first enters that player's inventory; acquired Exotics are excluded until the category's Exotics are exhausted (§2.3 step 3b). Pool sizes match doc 14 §5.3: 4 base, 12 in `exotic_arms`, 8 in `exotic_armor`.

| ID | Archetype | Home | Mfr | Pool | Dep | Intrinsic |
|---|---|---|---|---|---|---|
| `item.exotic.dragonsong` | blade_long | mythic_fantasy | runecourt | base | arcana | every 3rd consecutive hit within 120 ticks releases a 4000 mm cone of `dmg.fire` for 6000 Bp of its `D2` plus 60 `st.burning` power |
| `item.exotic.widowmaker` | pistol_heavy | weird_west | coffin_creek | base | verve | magazine fixed at 6 (tendencies, affixes and perks never change it); the 6th shot uses crit chance 10000 and +5000 Bp |
| `item.exotic.ghost_protocol` | head_light | cyberpunk | chromaw | base | techne | crouched without attacking for 60 ticks: `st.invisible` applied directly, refreshed every 60 ticks; the first hit after it ends +3000 Bp |
| `item.exotic.paper_moon` | `arch.ward.aegis` | oneiric | somnambule | base | arcana | capacity × 15000; while ward > 0, immune to `st.feared` and `st.dread` |
| `item.exotic.quietus` | dagger | gothic_horror | vesper_sons | exotic_arms | none | +10000 Bp on sneak attacks; sneak kills restore 15 composure |
| `item.exotic.starfall` | charge_rifle | space_opera | perihelion | exotic_arms | techne | a fully charged shot calls a 3000 mm strike at impact 60 ticks later: 15000 Bp of the shot's `D2` as `dmg.radiant` (environmental entry) |
| `item.exotic.thunderwright` | warhammer | mythic_antiquity | hephaesteum | exotic_arms | mythos | heavy attacks emit a 4000 mm `dmg.shock` nova: 5000 Bp of the hit's `D2` plus 40 `st.shocked` power |
| `item.exotic.redline_sutra` | smg | cyberpunk | ghostline | exotic_arms | techne | attack speed +250 Bp per 6 consecutive shots, max +2500; at max, +15 `st.burning` power per hit; resets after 20 idle ticks |
| `item.exotic.rust_psalm` | shotgun | post_apocalypse | rustmaw | exotic_arms | none | every 4 pellets of one shot that hit the same target apply one `st.corroded` stack directly |
| `item.exotic.gallows_oath` | crossbow | weird_west | coffin_creek | exotic_arms | verve | leg-part hits on non-boss targets apply `st.rooted` directly for 120 ticks |
| `item.exotic.cogheart` | rifle_auto | steampunk | aetherline | exotic_arms | techne | spends no ammo while the wielder has not moved for 60 ticks |
| `item.exotic.briar_rose` | chain | fairy_tale | thornwood | exotic_arms | arcana | +30 `st.sleeping` power per hit; +5000 Bp against `st.sleeping` targets |
| `item.exotic.hydra_bow` | charge_rifle | mythic_antiquity | argive | exotic_arms | mythos | each arrow splits on its first hit into 2 more that hit the 2 nearest other hostiles within 6000 mm for 3000 Bp of its `D2` |
| `item.exotic.dead_drop` | crossbow | hardboiled_noir | midnight_avenue | exotic_arms | none | bolts emit 0 dB; a lodged bolt detonates after 180 ticks for 8000 Bp of its hit's `D2` as `dmg.blunt` within 1500 mm |
| `item.exotic.black_bar` | staff | redacted | bureau | exotic_arms | techne | precision-part hits apply `st.blinded` directly |
| `item.exotic.lullaby` | wand | oneiric | somnambule | exotic_arms | arcana | every 3rd charge spent is refunded; holds a `wand_pool` spell drawn by step 11c (doc 10 §4.2) |
| `item.exotic.stormcrown` | head_medium | mythic_fantasy | deepanvil | exotic_armor | arcana | the wearer's crits add 3000 Bp of the crit's `D2` as `dmg.shock`, at most once per 60 ticks |
| `item.exotic.ironhide_greaves` | legs_heavy | post_apocalypse | geigerhaus | exotic_armor | none | immune to `st.slowed` and `st.rooted`; +10 poise |
| `item.exotic.tidecaller_wraps` | hands_light | cosmic_horror | dunwold | exotic_armor | mythos | throw range × 15000; +2000 Bp against `st.soaked` targets |
| `item.exotic.cloud_step` | feet_light | wuxia | iron_crane | exotic_armor | verve | one extra air jump per airborne period; `law_bound: law.wuxia.qinggong` (Canon §5.10) |
| `item.exotic.drifters_duster` | body_light | weird_west | sundown_rail | exotic_armor | verve | the first shot within 60 ticks after a dodge uses crit chance 10000 |
| `item.exotic.brass_lungs` | body_medium | steampunk | boilerheart | exotic_armor | techne | gases breathable; fire DoT on the wearer × 5000 |
| `item.exotic.veil_of_vespers` | head_light | gothic_horror | ashgrave | exotic_armor | mythos | composure loss × 5000 (doc 04) |
| `item.exotic.stampede_sabatons` | feet_heavy | mythic_antiquity | hephaesteum | exotic_armor | none | sprint costs no stamina; sprinting into a minion- or standard-rank hostile applies `st.knocked_down` directly, once per target per 300 ticks |

### 8.3 Palimpsestic uniques (10)

Canon §13.3 rules apply: Contradiction 0 everywhere, no Anachronism Edge, re-skin to the local genre (they never need Rewrite, which refuses them), effective ilvl `max(ilvl, current Folio)`, 4 affixes. Manufacturer `mfr.ur_text.first_press`. Weapons roll columns 1 to 4 and use the local genre's trait in column 5. Sources: §9.8 only. At most one copy of each unique per party per run (a unique counts as acquired when it first enters any party member's inventory); never sold, never on Engine faces, never salvaged; curse step skipped (doc 10 rarity factor 0).

| ID | Archetype | Intrinsic |
|---|---|---|
| `item.palimpsestic.first_word` | blade_long | +5000 Bp vs `fac.editors`; each such kill removes 1 `st.erasure` stack from the wielder |
| `item.palimpsestic.omnibus` | rifle_marksman | also carries the local genre's genre trait (§7.2), re-evaluated when the local genre changes |
| `item.palimpsestic.marginal_gloss` | `arch.ward.bulwark` | capacity × 20000; ward break applies `st.staggered` to non-boss hostiles within 4000 mm |
| `item.palimpsestic.errata_sheet` | body_medium | `gear_tol` +2 on every axis (Canon §5.4 cap still 6) |
| `item.palimpsestic.authors_quill` | staff | once per 600 ticks a spell cast through it costs 0 aether |
| `item.palimpsestic.dog_ear` | trinket | stores the bearer's position at Folio entry; using it (60-tick channel) returns there, once per Folio |
| `item.palimpsestic.endpaper_band` | ring | +6 to each sub-attribute of the wearer's lowest primary, re-evaluated on change |
| `item.palimpsestic.blank_page` | shield_heater | blocked hits from `fac.editors` use block stability 10000 |
| `item.palimpsestic.running_head` | head_light | HUD shows every Fortune Engine, Reliquary and secret within 32000 mm; mimics are revealed |
| `item.palimpsestic.colophon_mark` | amulet | Lucre generated for the bearer × 12500; the bearer's buy prices × 9000 |

---

## 9. Loot sources

### 9.1 Kill drops, line chances and Lucre

On `ev.entity_killed` of an NPC, the host rolls the NPC's rank table (§9.5) once per Active, non-ghost Erratum present in the Folio (doc 16 §9.6), in ascending slot order, with `drop_rng(folio, killed EntityId, tick, slot)`. Ghost, Dormant and Departed slots receive nothing. Summons, minions of `fac.errata`, invaders (§17) and NPCs flagged `no_loot` (doc 13) drop nothing. Doc 13 §5.1 carried items drop separately as Public items.

```
line_chance = base_bp
            → × party_drop_bp[n]   if the line is "scaled" (ranks minion to champion)   // [10000, 8000, 6666, 5714] = floor(10^8 / enemy_count_bp[n])
            → × 16667              if mode.abridged and the line is scaled (Canon §17.1)
            → min(10000, ·)
```

Example: an elite's item line (3000) in a party of 2 is `mul_bp(3000, 8000) = 2400` Bp per player. The party meets 12500 Bp as many enemies, so each player's expected yield per Folio equals the solo yield (`12500 × 8000 = 10000 × 10000`). Novella, Daily and Endless use Standard depth and Standard quantities. Abridged scales these kill-drop lines as well as the worldgen chests and floor-loot spots that Canon §17.1 names (Canon gaps 1); Reliquaries, Fortune Engines and vendor stock stay unscaled.

**Lucre lines** draw `amount = range_incl(mul_bp(4, P(d)), mul_bp(10, P(d)))` in `sub(seed, "loot.lucre")` (1 draw), then `→ × rank_mult_bp → × gp.lucre_bp → max(1, ·)`, each one `mul_bp`, creating one `item.lucre` pile with `fresh = true`. `genre.unwritten` has no Lucre lines (Canon §5.2). `d` is the Folio depth `d(f)` (doc 14 §3.1).

**Lucre find.** The first pickup of a fresh pile multiplies its amount by `10000 + min(LUCRE_FIND_CAP, Σ find)`, then by 12500 if the picker holds `run.boon.deep_pockets` (doc 14 §3.9), then by `10000 + m` for each `curse.pact.miser` and by `10000 - m` for each `curse.greed` the picker has equipped (doc 10 §7.3, §7.8; one factor per equipped instance in ascending item uid), each one `mul_bp`, and clears `fresh`. `Σ find` adds every equipped `affix.lucre_find`, `item.ring.wealth` (+1500, +150 per `eff_enh`) and `item.palimpsestic.colophon_mark` (+2500). `LUCRE_FIND_CAP = 5000`. Piles dropped from a purse (§17) are created with `fresh = false`. Sales and dropped piles never receive Lucre find, the boon, the pact or `curse.greed`, which keeps the buy floor above the sell cap (§12.2). Example: a 27-Lucre pile picked up with one tier-3 `affix.lucre_find` of 900 becomes `mul_bp(27, 10900) = 29`.

### 9.2 Category deck

Each player has a 40-card deck, saved in `PlayerLootState`, used by every Keyed generic item line:

| Card | Count | Card | Count | Card | Count |
|---|---|---|---|---|---|
| `cat.weapon_melee` | 5 | `cat.ward_gen` | 1 | `cat.text` | 4 |
| `cat.weapon_ranged` | 5 | `cat.wand` | 1 | `cat.throwable` | 3 |
| `cat.armor` | 5 | `cat.implant` | 1 (`cat.accessory` where local TEC < 5) | `cat.ammo` | 2 |
| `cat.offhand` | 2 | `cat.potion` | 6 | `cat.gadget` | 1 |
| `cat.accessory` | 3 | | | `cat.part` | 1 |

Cards are drawn in this order: melee, ranged, armor, offhand, accessory, ward_gen, wand, implant, potion, text, throwable, ammo, gadget, part (where the local TECHNE is below 5 the implant card's count is added to accessory and the implant card is omitted). The draw weight of a card is `count × role_bias_bp` (§9.3). The drawn card's count decrements; when every count reaches 0 the deck refills to the table. `Fixed(cards)` requests (Keepsakes, Engine faces, Reliquary equipment slots, shop lines, floor loot, Volume rewards) use the listed cards' counts as fixed weights without decrementing. A throwable card uses the §3.5 throwable weights; an ammo card yields one counter charge (§3.5).

### 9.3 Role bias

| Doc 13 `loot_role` | Category bias (Bp; unlisted = 10000) |
|---|---|
| brute | melee 20000, armor 15000, offhand 15000 |
| skirmisher | melee 15000, throwable 20000, accessory 12000 |
| marksman | ranged 25000, ammo 20000 |
| caster | wand 30000, text 20000, potion 15000, accessory 15000 |
| support | potion 25000, text 15000, ward_gen 20000 |
| swarm | none |
| beast | the item line uses `loot.trophy` (§9.6) instead of the deck; Lucre chance 0 |
| construct | part 40000, gadget 20000, ammo 15000, potion 0, text 0 |

### 9.4 In-run bad-luck protection

Per player and run, `blp_rare` counts consecutive rarity-eligible Keyed generic rolls below Rare, and `blp_leg` counts those below Legendary. Keyed generic rolls are the item lines of kills, chests, bags, Reliquaries, secrets and war spoils. Forced-rarity rolls (Engine faces, Volume rewards, Keepsakes, companion rewards), quest rewards and Placed sources neither use nor change them.

```
w[r] = mul_bp(w[r], 10000 + 500 × min(blp_rare, 20))   for r in 2..=6
w[r] = mul_bp(w[r], 10000 + 100 × min(blp_leg, 100))   for r in 4..=6
floor = max(floor, rare)      if blp_rare >= 24
floor = max(floor, legendary) if blp_leg  >= 120
after the roll: rarity >= rare resets blp_rare, else +1; rarity >= legendary resets blp_leg, else +1
```

The HUD ledger shows both counters and both guarantees. Example: `blp_rare = 10` multiplies the Rare-or-better weights by 15000.

### 9.5 Rank tables

"Scaled" lines follow §9.1. Shift and floor feed §2.3 step 3. Lines resolve in table order: Palimpsestic line (Editors only), Lucre, item lines, key lines, other lines, ammunition line. A line whose condition fails (Marginalia only, Lucre in `genre.unwritten` or for a beast) stays in the table with chance 0, so a table's draw count never changes.

| Table (rank) | Item lines | Lucre line | Key lines | Other lines | Ammunition line |
|---|---|---|---|---|---|
| `loot.drop.minion` | 250 scaled | 3000 scaled, × 3000 | — | — | 750 scaled |
| `loot.drop.standard` | 700 scaled | 5000 scaled, × 10000 | — | — | 2500 scaled |
| `loot.drop.elite` | 3000 scaled, shift 2000 | 10000, × 30000 | `item.key.bound` 1500 scaled | `item.bag.lucky` 300, Marginalia only | 7500 scaled |
| `loot.drop.champion` | 10000 and 5000 scaled, shift 4000, floor Uncommon | 10000, × 60000 | bound 4000, gilded 1000, both scaled | lucky bag 1000 (2500 in Marginalia) | 10000 scaled |
| `loot.boss.mini` (`rank.boss`) | 10000 and 10000, shift 6000, floor Rare | 10000, × 200000 | one line 10000: gilded 3000 / bound 7000 (`weighted` in `loot.key`) | `item.bag.boss` 10000 (two lines in Marginalia) | 10000 |
| `loot.boss.editor` (`rank.editor`) | Palimpsestic line `palimpsestic_bp[v]` → `loot.boss.palimpsestic`; item 10000 shift 6000 floor Legendary; items 10000 and 10000 shift 6000 floor Rare | 10000, × 400000 | `item.key.illumined` 10000 | `item.bag.editor` 10000; Marginalia: `item.bag.elite` 10000 | 10000 |
| `loot.drop.hunter` (`rank.hunter`) | 10000, shift 5000, floor Rare | 10000, × 50000 | — | — | 10000 |

- `palimpsestic_bp = [2500, 4375, 6250, 8125, 10000]` for Volumes 1 to 5 and 10000 in Marginalia and in Second Edition Volumes 6 to 8 (doc 13 §9). The Palimpsestic line is the table's first line, so its `roll_bp` is the kill's first draw as doc 13 states. `loot.boss.palimpsestic` resolves one item forced Palimpsestic through step 3b; with every unique acquired it is Legendary.
- Editor tables use doc 10's Editor-reward curse factor 0. The Volume reward (§9.8) is separate.
- Each line draws 2 values (§2.3), so `loot.drop.elite` always draws 10 values, `loot.boss.mini` 12 (14 in Marginalia) and `loot.boss.editor` 16 (18 in Marginalia).

Example record:

```json
{ "id": "loot.drop.elite",
  "lines": [ { "kind": "lucre", "chance_bp": 10000, "scaled": false, "rank_mult_bp": 30000 },
             { "kind": "item", "chance_bp": 3000, "scaled": true, "shift_bp": 2000, "floor": "rar.common" },
             { "kind": "key", "chance_bp": 1500, "scaled": true, "key": "item.key.bound" },
             { "kind": "bag", "chance_bp": 300, "scaled": false, "bag": "item.bag.lucky", "marginalia_only": true },
             { "kind": "ammo", "chance_bp": 7500, "scaled": true } ] }
```

### 9.6 Chests, floor loot, trophies, quests and war spoils

Doc 12 places container spots. A chest is `loot.chest.locked` when doc 12 places it in a `room.treasury` or behind a `Locked` edge, otherwise `loot.chest.plain`; the kind draws nothing. A `loot.chest.locked` chest carries a doc 09 `lock.kind.container` lock with `key_tags = [0, 0]`: no key binds it, so it is picked, hacked or broken (doc 09 §13.1). Container contents roll on opening with `drop_rng(folio, container uid, tick, slot)`; each player opens their own instance.

| Table | Lines (in order) |
|---|---|
| `loot.chest.plain` | item 10000; item 3000; Lucre 7000 at `range_incl(mul_bp(15, P), mul_bp(23, P))`; consumable 5000 (`Fixed` potion, text, throwable); `item.food.ration` 3000; bound key 600; gilded key 100 |
| `loot.chest.locked` (doc 09 lock) | item 10000 shift 2000 floor Uncommon; item 6000; Lucre 10000 at the plain range × 15000; bound key 1500 |
| `loot.floor` | Placed: per slot 0 to 3 whatever the party size, `place_rng(f, "floor", slot)` draws 3 item seeds (`Fixed` all deck cards) then 6 Lucre amounts `range_incl(mul_bp(18, P), mul_bp(27, P))`, each × `gp.lucre_bp` into a fresh pile (9 draws); Abridged: 5 items and 10 piles (15 draws); none in `genre.unwritten` |
| `loot.trophy` | beast-role item line: one item from the killed template's doc 13 `trophy` list (materials and ingredients), weights from that list; templates without a list use the local genre's doc 08 ingredient list |
| `loot.war_spoils` (doc 13 §12.4) | item 10000 shift 2000 floor Uncommon; item 5000; Lucre 10000 at the plain range × 20000 |
| `loot.quest.t1`, `.t2`, `.t3` (doc 13 §13.3) | one item: t1 shift 1000 floor Uncommon; t2 shift 2000 floor Uncommon; t3 shift 3000 floor Rare; keyed actor = the giver's EntityId, tick = completion tick, seq = slot; uncursed |
| `loot.companion.<name>` (doc 13 §13.7) | one item forced Legendary, `Fixed` equipment cards, origin genre = the companion's origin genre; keyed actor = the companion's EntityId |

The Lucre table of §13.2 assumes 5 plain chests per standard Folio.

**Story relic** (doc 12 §13, §20). At Folio generation, where doc 12 places a story seat, this chapter fills the story relic spot with one `Public` item: `place_rng(f, "relic", 0)` draws 1 item seed, and `generate_item` runs as a Placed source with `Fixed` card `cat.accessory`, floor Rare, `ilvl = d(f)`, SER 40, no curse draw (`buc.uncursed`) and knowledge per §2.4, so it starts unidentified. Its `story_domains` holds the story's 2 drawn domains (doc 12 §20.1), and its display name is doc 18's relic grammar over those 2 domains; that name is the relic's affix theme, and its affixes roll by §6 like any accessory. Solving the story identifies it (doc 12 §20.4, doc 10 `appr.reason.story`).

### 9.7 Treasure bags (Experienced Pixel Dungeon analog)

Bags are `cat.container` items (`stack_max` 1, mass 500 g, mundane). Using one (30 ticks) rolls its contents with `drop_rng(folio, bag uid, tick, slot)`: first the count draw where the bag has one (1 draw), then one `next_u64` seed per item and per Lucre line, then the Emendations (no draw). The contents spill as the user's instanced ground items.

| Bag | Contents |
|---|---|
| `item.bag.boss` | `2 + range(2)` items, shift 3000, floor Uncommon; Lucre `range_incl(mul_bp(40, P), mul_bp(85, P))` |
| `item.bag.editor` | 1 item, shift 4000, floor Rare; Lucre `range_incl(mul_bp(80, P), mul_bp(170, P))`; the Volume's unplaced Emendations (§9.9) |
| `item.bag.lucky` | `range(15)` items (0 to 14), no floor |
| `item.bag.elite` | one uncursed item per card `cat.weapon_melee`, `cat.weapon_ranged`, `cat.armor`, `cat.offhand`, `cat.accessory`, `cat.ward_gen`; dropped by Marginalia Editors |
| `item.bag.refined` | `3 × range(15)` items (one draw, 0 to 42) plus 1 item with floor Epic |
| `item.bag.burnt` | Lucre `mul_bp(20, P)` |

Burning: when a ground `item.bag.lucky` takes `dmg.fire`, it rolls keyed `rng.loot` (actor = bag uid, tick, seq = finder slot) `roll_bp(1000)`: success → `item.bag.refined`, failure → `item.bag.burnt`. The expected item count falls from 7 to 2.2 (`1000 × (3 × 7 + 1) / 10000`), so burning trades quantity for a 1000 Bp chance at an Epic-floor item.

### 9.8 Bosses, Volume rewards, secrets and Palimpsestic sources

- **Volume reward** (`loot.volume_reward`, doc 14 §3.9): an offer of 3 items from `offer_rng(slot, editor EntityId, "volume_reward")` (§11.2; 4 draws: `roll_bp(2500)`, then 3 item seeds). Option 0: Legendary, `Fixed` cards melee, ranged and wand. Option 1: Exotic, `Fixed` equipment cards (Legendary when no Exotic remains). Option 2: Palimpsestic when the roll succeeded, else Legendary with `Fixed` cards armor, offhand, accessory and ward_gen. `ilvl = d(f)`, uncursed, sealed with RARITY set. The player commits one with `CommitOffer`; doc 14's timeout commits index 0; the commit advances `rng.offer.p<slot>` by one.
- **Secrets**: doc 12's secret rooms roll the three tables below; doc 13 companion quest rewards add one `loot.secret.cache` roll per Active Erratum.
- Palimpsestic uniques come only from `loot.boss.palimpsestic` (Editors), Volume rewards and Palimpsest vaults (Canon §13.3). Expected uniques per player in a Standard run: 3.125 from Editors plus at most 1.25 from Volume rewards; the party-wide one-copy rule caps them at 10.

| Table | Contents | Generator |
|---|---|---|
| `loot.secret.cache` | 3 items shift 3000 floor Uncommon, plus a plain-chest Lucre line (§9.6) | a container: rolled per player on opening (§9.6) |
| `loot.secret.vault` | one gilt Fortune Engine (`engine.gilt`) or one illumined Reliquary (`reliq.illumined`) | Placed: `place_rng(f, "vault", spot)`, 1 `weighted([5000, 5000])` (§9.11) |
| `loot.secret.palimpsest` | one Palimpsestic unique per Active Erratum (step 3b; Legendary when every unique is acquired) | keyed `rng.loot`, actor = vault uid, seq = slot |

### 9.9 Volume plan: Emendations, Rewrite Quills and keys

The plan of Volume `v` comes from `place_rng(f1, "vplan", 0)`, where `f1` is the first Folio of the Volume (`(v - 1) × 5 + 1` in Standard, Novella, Daily and Endless). Any later Folio of the Volume recomputes it without generating earlier Folios. Draws: `u1 = range(5)`, `u2 = range(4)`, `u3 = range(3)`, `u4 = range(2)`, `q = range(4)`, `b1 = range(4)`, `b2 = range(4)`, `g = range(4)` (8 draws).

```
remaining = 3
for pos in 1..=4: if u_pos < remaining { emendation at pos; remaining -= 1 }
boss Folio: the remaining Emendations go into item.bag.editor
Rewrite Quill at pos q + 1; bound key caches at pos b1 + 1 and b2 + 1 (only the first max(0, 2 + gp.reliq_key_delta)); gilded key cache at pos g + 1
```

This places 3 Emendations among the 5 positions uniformly without replacement, so every position expects 0.6 and every Volume holds exactly 3 (Canon §13.7). Every Active Erratum gets an instanced copy at the same placement (a guarded spot, doc 12). **Abridged** (3 Folios per Volume, `f1 = (v - 1) × 3 + 1`): one Emendation at position 1, one at position 2 and one in the Editor bag, no `u` draws; `q`, `b1`, `b2`, `g` are `range(2)` (4 draws). **Marginalia** Volumes draw the same 8 values and ignore `u1..u4` (§16.2).

### 9.10 Champion counters, ward generators and parts

- **Counters** (Canon §17.3a, doc 05 §15.3). Every shop stocks every counter charge and counter grenade, `alch.bomb.salt` and `alch.bomb.smoke` (the Colophon and general-store lines and the counter shelf of every other shop, §12.1) and restocks them (§12.3). The throwable card (4000 of its 10000 weight is counter grenades) and the ammo card (always a counter charge) put every counter in every genre's loot pool. When the exit preview shows a champion aspect, the next Folio places one cache per aspect near the Incipit holding 2 matching counter grenades and 2 matching counter charges per player (`place_rng(f, "cache", aspect_index)`, 16 item seeds). This holds in all 16 genres, including `genre.unwritten`, which has no shops (Canon §17.3a).
- **Ward generators and parts** come from deck cards; constructs bias parts; salvage yields parts (§16.4).

### 9.11 Placement generators

Every Placed source draws from a labeled generator of the sequential stream `rng.loot_place`, so each placement decision is isolated from every other:

```rust
fn place_rng(folio: u32, label: &str, index: u64) -> Rng {
    Rng::from_seed64(mix64(stream_seed(run_seed, "rng.loot_place", folio as u64) ^ mix64(fnv1a64(label) ^ index)))
}
```

| Label | Index | Draws | Use |
|---|---|---|---|
| `vplan` | 0 | 8 (Abridged 4) | §9.9, on the Volume's first Folio |
| `reliq` | 0 | 9 | §10 placement |
| `engine` | 0 | 4 | §11.1 placement |
| `shop` | 0 | 3 | §12.1 vendor draw |
| `stock` | shop ordinal | 1 per line per slot 0 to 3 | §12.1 stock seeds |
| `floor` | slot 0 to 3 | 9 (Abridged 15) | §9.6 floor loot |
| `cache` | aspect index | 16 | §9.10 counter caches |
| `vault` | secret spot index | 1 | §9.8 |
| `relic` | 0 | 1 | §9.6 story relic |

Items created at Folio generation take uids in this order: Volume-plan items, floor loot, shop stock (shops by ordinal), caches, the story relic; within each, slot 0 to 3 and then table order. Items generated for empty slots are generated and discarded, so uids never depend on party size.

### 9.12 In-run Reliquary roll

Canon §17.9 (killing an invader) and doc 12 §6.3 (challenge success) grant "one in-run Reliquary roll". It gives the receiver the contents of one `reliq.bound` Reliquary before Folio 6 and of one `reliq.gilded` Reliquary from Folio 6 (§10), rolled with `drop_rng(folio, actor, tick, slot)` where `actor` is the invader's EntityId or, for a challenge, the receiver's own EntityId at the exit-commit tick. No key is needed, it is never a mimic, it counts toward `rq5` and `rq20`, and the items spill as the receiver's instanced ground items.

---

## 10. In-run Reliquaries

| Tier | Key | Contents (each a 10000 line, in order) | Shift | Mimic (Bp) | Placement (doc 12 places the room spot) |
|---|---|---|---|---|---|
| `reliq.plain` | none; 90-tick open | L1: 1 item from the opener's deck (`Decrement`); L2: 1 equipment item floor Uncommon | 0 | 400 | standard and challenge Folios: `1 + range(2)`; rest Folios: 1 |
| `reliq.bound` | `item.key.bound` | L1, L2: equipment floor Uncommon; L3: consumable bundle | 2000 | 300 | 6000 Bp per standard or challenge Folio from Folio 2 |
| `reliq.gilded` | `item.key.gilded` | L1, L2: equipment floor Rare; L3: equipment floor Epic | 3000 | 200 | 2500 Bp per standard or challenge Folio from Folio 6 |
| `reliq.illumined` | `item.key.illumined` | L1 to L3: equipment floor Rare; L4: equipment floor Legendary | 4000 | 0 | every boss Folio's Colophon, available from the Editor's death; `loot.secret.vault` |

- **Equipment lines** use `Fixed` equipment cards (§3.1). Every Reliquary line is Keyed and generic, so it uses and updates the opener's bad-luck counters (§9.4). Contents call `roll_curse` with source factor 0 (doc 10 §7.1) and are sealed (§2.4).
- **Consumable bundle.** In `sub(seed, "loot.bundle")`: `roll_bp(500)` adds one `item.tool.rewrite_quill`; then three `next_u64` seeds give 3 consumables with `Fixed` potion, text and throwable cards. A bundle potion draws `roll_bp(2500)` in `loot.arch` and then one `weighted` over the Reliquary-only list on success or the found list on failure (§3.5). The bundle always draws 4 values.
- **Placement** (`place_rng(f, "reliq", 0)`, always 9 draws): `n_plain = 1 + range(2)`; then for each candidate in the order plain 0, plain 1, bound, gilded, one existence `roll_bp(chance)` and one `roll_bp(mimic)`, made even when the chance is 0. Chances: plain 0 10000; plain 1 10000 when `n_plain = 2`, else 0; bound 6000 from Folio 2; gilded 2500 from Folio 6. Rest Folios set plain 1, bound and gilded to 0. Boss Folios skip the generator; their illumined Reliquary is fixed and never a mimic. A higher key opens a lower tier. Keys sit on the keyring (§14.1).
- **Instancing.** Each Active Erratum has its own instance and its own contents, rolled on opening with `drop_rng(folio, reliquary uid, tick, slot)`. The mimic flag is shared.
- **Bad-luck protection** per player and run: `rq5` = Reliquaries opened since one held Epic or better, `rq20` = since Legendary or better. When `rq5 = 4`, the equipment line with the highest floor uses floor Epic; when `rq20 = 19`, it uses floor Legendary (both may apply; Legendary wins). The tooltip shows "Epic or better within `5 - rq5`" and "Legendary or better within `20 - rq20`".
- **Displayed odds**: per line, the §2.3 step-3 weights with the opener's effective SER, shown to 4 decimals. Example: a plain Reliquary's equipment line at Folio 7, SER 40: weights `[0, 2728, 888, 258, 88, 33, 0]`, total 3995: Uncommon 68.2854 %, Rare 22.2278 %, Epic 6.4581 %, Legendary 2.2028 %, Exotic 0.8260 %.
- **Mimic.** A mimic is a hidden object for Canon §6.4 row 35 detection (doc 09); detection shows a tell to the detecting player. Opening it does not spend the key; it becomes `enemy.mimic` (doc 13, `rank.elite`, `no_loot`). Its death converts it back into a Reliquary that every player opens without a key, with one extra item line (shift 2000) per player.

| State (shared, per Reliquary) | Event | Next |
|---|---|---|
| Sealed | player `p` with a valid key (none for plain) and `opened[p] = false` holds interact | Opening(p) for 90 ticks; other players' channels are refused until it ends |
| Opening(p) | channel completes, not a mimic | Sealed; `opened[p] = true`, key spent, contents rolled for `p`, `ev.container_opened`, `rq5` and `rq20` updated |
| Opening(p) | channel completes, mimic | Awake: key kept, `enemy.mimic` spawns at the Reliquary |
| Opening(p) | `p` takes non-DoT damage, moves more than 500 mm, or releases interact | Sealed: no key spent, no draw |
| Awake | `enemy.mimic` killed | Lootable |
| Lootable | player `p` with `opened[p] = false` interacts for 30 ticks (no key) | Lootable; `opened[p] = true`, contents plus the extra line |
| Sealed or Lootable | every Active player has `opened[p] = true` | Spent (the model shows it open) |
| any | Folio exit | removed; an Awake mimic despawns |

In `mode.loot_shared` a Reliquary has one instance: the opener rolls its contents once per Active player (`seq` = 0 to `n - 1`) into a sealed pool, and the players draft one item at a time in turn order (the opener first, then ascending slot, wrapping) with `CommitOffer { context: reliquary uid, index }`; a pick not made within 600 ticks takes the lowest uid. Doc 10 §11 shows only the sealed view during the draft.

---

## 11. Fortune Engines

### 11.1 Tiers

| Engine | Base price | Face rarities 0..5 | Face weights 0..5 (Bp) | Placement |
|---|---|---|---|---|
| `engine.brass` | 60 | C, C, U, U, R, E | 2600, 2600, 1800, 1800, 900, 300 | 1 in every standard and challenge Folio; 1 in each rest Folio's Engine hall |
| `engine.silver` | 160 | U, U, R, R, E, L | 2500, 2500, 1800, 1800, 1000, 400 | 5000 Bp in standard and challenge Folios from Folio 3; 1 in each rest Folio's Engine hall |
| `engine.gilt` | 420 | R, R, E, E, L, X | 2500, 2500, 1800, 1800, 1000, 400 | every boss Folio's Colophon, available from the Editor's death; `loot.secret.vault` |
| `engine.armory`, `engine.wardrobe`, `engine.curio` | 160 | as silver | as silver | with doc 14's `unlock.room.fortune_engines_2` in the content set, each silver placement becomes a variant with 5000 Bp |

Placement draws `place_rng(f, "engine", 0)`, always 4: `roll_bp(brass chance)`, `roll_bp(silver chance)`, `roll_bp(5000)` for a variant (chance 0 without the unlock), `weighted([1, 1, 1])` over armory, curio, wardrobe. Variants restrict face categories: armory to weapons, wands and off-hands, wardrobe to armor and ward generators, curio to accessories and implants. Other engines draw face categories from the `Fixed` equipment cards (§9.2). No engine exists in `genre.unwritten`.

### 11.2 Offers, odds and dispensing

```rust
const ENGINE_PITY_NEED: u32 = 9;          // non-top capsules before the top face is guaranteed
const ENGINE_PITY_NEED_WISHBONE: u32 = 7;
const CURSED_FACE: usize = 4;             // the cursed capsule (Canon §13.6, DECISIONS 60)
fn offer_rng(slot: u8, context_uid: u64, purpose: &str) -> Rng {
    // peek_u64 returns the next output of rng.offer.p<slot> without advancing it (Canon §3.5)
    Rng::from_seed64(mix64(peek_u64(OFFER[slot]) ^ mix64(context_uid ^ fnv1a64(purpose))))
}
fn engine_interact(p: &mut Player, e: &mut Engine) {
    if e.faces[p.slot].is_none() {                        // generated once, stored, re-shown on reopen
        let mut o = offer_rng(p.slot, e.uid, "engine_faces");
        e.faces[p.slot] = Some((0..6).map(|i| generate_item(o.next_u64(),
            &GenRequest::engine_face(e.tier.face_rarity[i], e.category_rule, d_current, i == CURSED_FACE))).collect());
    }
}
fn engine_buy(p: &mut Player, e: &mut Engine, tick: u64) -> Result<ItemInstance, Refusal> {
    let k = e.spins[p.slot];
    if k >= 5 { return Err(Refusal::Exhausted); }
    let price = mul_bp(mul_bp(mul_bp(e.tier.base, P(d_current)), 10000 + 2500 * k as i32), gp.shop_price_bp);
    p.purse.spend(price)?;                                  // Lucre only; nothing else is accepted
    let w = displayed_weights(p, e);                        // exactly what the panel shows
    let i = keyed_rng("rng.loot", folio, e.uid, tick, p.slot as u32).weighted(&w);   // 1 draw
    let item = e.faces[p.slot].as_mut().unwrap()[i].take_dispensed();
    advance(OFFER[p.slot]);                                 // commit: exactly 1 next_u64
    e.faces[p.slot].as_mut().unwrap()[i] = generate_item(offer_rng(p.slot, e.uid, "engine_refill").next_u64(),
        &GenRequest::engine_face(e.tier.face_rarity[i], e.category_rule, d_current, i == CURSED_FACE));
    e.spins[p.slot] = k + 1;
    p.engine_pity[e.tier.idx] = if i == 5 { 0 } else { p.engine_pity[e.tier.idx] + 1 };
    emit(Ev::EngineSpun { engine: e.uid, slot: p.slot, face: i, item: item.uid, price });
    Ok(item)
}
fn displayed_weights(p: &Player, e: &Engine) -> [u32; 6] {
    let need = if p.carries(TRINKET_WISHBONE) { ENGINE_PITY_NEED_WISHBONE } else { ENGINE_PITY_NEED };
    if p.engine_pity[e.tier.idx] >= need { return [0, 0, 0, 0, 0, 10000]; }   // guarantee armed and shown
    let mut w = e.tier.weights;
    let s = if p.has(ST_UNMOORED) { 0 } else { clamp((p.ser_eff() - 40) * 10, 0, 500) };
    w[0] -= s; w[5] += s;
    w
}
```

- Faces are Offer items: forced rarity, `ilvl = d_current`, `buc.uncursed` (no curse draw) except the cursed capsule, sealed with RARITY set (doc 10 §11), no bad-luck counters, `illum_pending` for Legendary-or-better faces. The panel shows each face's sealed card, its weight and its probability to 4 decimals, the price, the capsules left (5 − `spins`) and the pity line "top capsule guaranteed after `need − pity` more". An Exotic on a face counts as acquired only when dispensed.
- **Cursed capsule** (Canon §13.6, DECISIONS 60). Face 4 of every Engine, the Rare face of brass, the Epic face of silver and its variants and the Legendary face of gilt, is its cursed capsule. Its item is generated like any face, except that §2.3 step 12 calls doc 10's `roll_curse` with source `CursedCapsule` (3 draws in `loot.buc`): it is always `buc.cursed` and always carries a pact, a curse affix welded to its boon (doc 10 §7.8). Every Engine face category is an equipment card (§9.2), and doc 10 §7.8 gives each of them pacts, so the cursed capsule always has one. It is created sealed with `RARITY` and `OMEN` set, so the panel marks it Ominous before purchase; which pact it carries stays hidden until appraised (doc 10 §7.8). The panel also shows the cursed share, `w[4] / Σ w` of the displayed weights: 900 Bp at brass and 1000 Bp at silver, its variants and gilt (the SER shift moves weight only between faces 0 and 5), and 0 while the pity guarantee is armed. A cursed capsule is a non-top capsule for pity. Buying is never a wager on the curse: the curse is shown, and the item may be carried, sold (doc 10 `value_vendor`) or left unequipped.
- `engine_pity` has one counter per tier (brass, silver with its variants, gilt) per player and run; it carries across engines and Folios.
- Example: SER 70 at a brass engine gives `s = 300`, weights `[2300, 2600, 1800, 1800, 900, 600]`; the cursed share stays 900 Bp (9.0000 %). Brass prices at Folio 7 are 100, 125, 150, 175 and 200 Lucre (750 for all five, one Folio's target income, §13.2). From fresh pity the expected number of capsules to the top face is 8.75 at SER 40 and 7.69 at SER 70, so a guaranteed Epic costs about two brass engines. The old design's 4-capsule pity let every fully bought brass engine pay an Epic, ten times the drop supply; 9 keeps Engines a sink rather than the main Epic source.
- **Compliance.** One fixed price per capsule, no stake choice, no multiplier, no repeat-wager, no empty outcome: every payment yields exactly one item from the displayed faces, and the cursed capsule is shown as cursed before payment, never a hidden outcome (Canon §13.6). Presentation is a crank and a capsule chute: no reels, no symbols, no flashing jackpot lights, no near-miss animation, no win or lose wording, no coin-shower audio.

| State (per player and engine) | Event | Next |
|---|---|---|
| Idle | interact | Shown (faces generated if absent) |
| Shown | `ShopBuy` intent, Lucre ≥ price, `spins < 5` | Dispensing (30 ticks): Lucre taken, face drawn, stream advanced on the intent tick |
| Shown | `ShopBuy` with Lucre < price | Shown; refused, nothing changes |
| Dispensing | 30 ticks elapse | Shown, or Exhausted when `spins = 5`; the capsule enters the inventory, or drops at the chute as `Instanced(slot)` when the inventory is full or the player died or left |
| Shown | close | Idle (faces kept) |
| Exhausted | interact | Exhausted: faces shown, purchase refused |
| any | Folio exit | removed |

---

## 12. Shops and services

### 12.1 Shop types, placement and stock

Doc 13 §13.2 defines one vendor NPC `npc.vendor.<type>` for each shop below, the Colophon's and the wanderer's included. Shops exist in every genre except `genre.unwritten` (Canon §5.2).

| Folio | Shops (ordinal in brackets) |
|---|---|
| standard, challenge | `shop.colophon` [0]; `1 + [roll_bp(4000)]` vendors [1, 2]; at position 4 also the Interleaf's `shop.general` [3] |
| rest | `shop.colophon` [0]; one vendor [1] (doc 12's rest shop) |
| boss | `shop.colophon` [0] only |

Vendor draw (`place_rng(f, "shop", 0)`, always 3 draws): `roll_bp(4000)`, then two `weighted` calls over the vendor types in ascending ID with weights apothecary 3, arcanist 2, armorer 2, fence 1, general 3, gunsmith 2, scribe 1, tinker 2, wanderer 1 (Σ 17); the wanderer weighs 0 unless doc 14's `unlock.room.wandering_scribe` is in the content set. The first is vendor [1]; the second is vendor [2] when the roll succeeds and is ignored otherwise. A rest Folio uses only the first.

Stock, per player, per line, in table order (Placed source: `place_rng(f, "stock", ordinal)`, one `next_u64` per line per slot 0 to 3, fixed-template lines included; SER 40; no Exotic; knowledge per §2.4):

| Shop | Lines per player | Markup (Bp) |
|---|---|---|
| `shop.colophon` | 3 consumable lines (`Fixed` potion, text, throwable); 5 counter-charge lines × 2; 5 counter-grenade lines × 1; `alch.bomb.salt` × 1; `alch.bomb.smoke` × 1; 3 equipment lines (`Fixed` equipment cards) shift 1500 floor Uncommon | 15000 |
| `shop.general` | 6 consumable lines (`Fixed` potion, text, throwable); 5 counter-charge lines × 2; 5 counter-grenade lines × 1; `alch.bomb.salt` × 1; `alch.bomb.smoke` × 1; `item.food.ration` × 5; `item.food.bread` × 5; `item.tool.bandage` × 5; `item.tool.torch` × 5; `item.tool.rope` × 2; `item.tool.grapple` × 1 | 14000 |
| `shop.armorer` | 6 equipment lines (`Fixed` armor, offhand, ward_gen) shift 1000 | 17500 |
| `shop.gunsmith` | 6 equipment lines (`Fixed` melee, ranged) shift 1000; one pickup of each doc 05 ammunition family (light, heavy, rifle, shell, cell, ordnance, draw) × 2 | 17500 |
| `shop.apothecary` | 6 potion lines × 2; 2 doc 08 ingredient lines × 5; `apparatus.vessel.flask` × 5; `item.tool.salve` × 3; `item.tool.splint` × 3 | 14000 |
| `shop.arcanist` | 1 wand line; 1 implement line (`arch.ranged.staff`, `arch.offhand.focus`, `arch.offhand.tome`, template weights); 2 accessory lines; 2 text lines × 2; 1 glyph-tome line (§3.5 Books); `item.tool.blank_text` × 5; trainer service | 17500 |
| `shop.tinker` | 3 gadget lines; 3 part lines; 1 implant line (accessory where local TEC < 5); `item.tool.lockpicks` × 1; `item.tool.toolkit` × 1; `item.tool.hacking_deck` × 1 | 16000 |
| `shop.fence` | 3 equipment lines (`Fixed` equipment cards) shift 3000 floor Rare; buys at sell trade × 11000 (§12.2) | 22000 |
| `shop.scribe` | 4 text lines; 1 schematic line (§3.5 Books); `item.tool.blank_text` × 5; `item.tool.rewrite_quill` × 1; every Scriptorium service (§12.3) | 15000 |
| `shop.wanderer` (`fac.neutral`) | 4 lines (`Fixed` every deck card) floor Uncommon | 16000 |

**Counter shelf** (Canon §17.3a: every counter in every shop; doc 05 §15, doc 08 §13.2). Every shop above except `shop.colophon` and `shop.general`, which list these lines already, appends after its own lines 5 counter-charge lines × 1, 5 counter-grenade lines × 1, `alch.bomb.salt` × 1 and `alch.bomb.smoke` × 1, in that order, as fixed-template lines of its stock generator.

Vendor names by genre (doc 18 localizes; `—` = no shops):

| Genre | Colophon | General | Armorer | Gunsmith | Apothecary | Arcanist | Tinker | Fence | Scribe | Wanderer |
|---|---|---|---|---|---|---|---|---|---|---|
| mythic_fantasy | Wayside Peddler | Provisioner | Armorer | Smithy | Herbalist | Enchanter | Gnome Tinker | Thieves' Guild Factor | Scribe | Pilgrim Trader |
| gothic_horror | Sexton | Chandler | Silversmith | Gunsmith | Apothecary | Curio Cabinet | Clockmaker | Grave-Robber | Parish Clerk | Travelling Show |
| cosmic_horror | Antiquarian | Dockside Grocer | Outfitter | Gun Dealer | Dispensary | Occult Bookshop | Instrument Maker | Dockside Smuggler | Archivist | Expedition Survivor |
| space_opera | Station Quartermaster | Commissary | Armor Fabricator | Armory Kiosk | Medbay | Relic Trader | Droid Shop | Smuggler | Data Clerk | Free Trader |
| cyberpunk | Vending Node | Corner Stall | Armor Boutique | Gun Shop | Street Doc | Ripper Clinic | Chop Shop | Fixer | Data Broker | Nomad Trader |
| post_apocalypse | Scrap Trader | Trading Post | Plate-Welder | Gun Runner | Rad-Doc | Junk Shaman | Gearhead | Raider Fence | Keeper of Books | Caravan Trader |
| weird_west | Traveling Salesman | General Store | Tanner | Gunsmith | Snake-Oil Wagon | Medicine Lodge | Blacksmith-Mechanic | Bandit Fence | Land Office Clerk | Drifter |
| hardboiled_noir | Newsstand | Five-and-Dime | Tailor | Pawn Gun Counter | Drugstore | Antique Dealer | Hardware Store | Fence | Records Clerk | Street Hustler |
| wuxia | Wandering Merchant | Teahouse Market | Armorsmith | Weaponsmith | Herb Hall | Talisman Stall | Mechanism Master | Black Market | Calligrapher | Itinerant Monk |
| steampunk | Automaton Clerk | Emporium | Plate Foundry | Ordnance Works | Chemist | Curiosity Emporium | Tinkerer's Workshop | Underworks Fence | Patent Office | Airship Purser |
| fairy_tale | Old Peddler | Village Market | Armorer | Blacksmith | Hedge Witch | Wishing Stall | Cobbler's Elves | Goblin Market | Storyteller | Wandering Fool |
| mythic_antiquity | Agora Merchant | Agora Stall | Bronze Armorer | Bronzesmith | Healer | Oracle's Stall | Automaton Maker | Pirate Trader | Temple Scribe | Wandering Rhapsode |
| oneiric | Dream Merchant | Toy Shop | Pillow Armorer | Toy Armory | Sleep Chemist | Memory Stall | Music-Box Maker | Shadow Self | Librarian of Dreams | Sandman's Apprentice |
| redacted | Requisitions Desk | Commissary | Armory Cage | Weapons Locker | Infirmary | Evidence Locker | Maintenance Bay | Leak | Records Office | Contractor |
| unwritten | — | — | — | — | — | — | — | — | — | — |
| ur_text | The Last Stationer | — | — | — | — | — | — | — | — | — |

### 12.2 Prices

Every vendor price is one `mul_bp` of the doc 10 vendor value by a composite trade factor, so doc 10's price observation (`PriceObs.trade_bp`, doc 10 §5.8) reproduces it exactly:

```
buy_trade  = 10000 → × markup_bp → × row37_bp (Canon §6.4) → × bearer modifiers in ascending item uid order
                   → × reputation_bp (doc 13 §13.2) → × gp.shop_price_bp → × (10000 - haggle_bp)
buy_trade  = max(BUY_TRADE_FLOOR, buy_trade)                      // 7500
buy        = max(1, mul_bp(value_vendor, buy_trade))               // per unit

sell_trade = 10000 → × row38_bp (Canon §6.4) → × 11000 at a fence → × gp.lucre_bp
sell_trade = min(SELL_TRADE_CAP, sell_trade)                       // 5500
sell       = max(1, mul_bp(value_vendor, sell_trade))              // per unit
```

Each arrow is one `mul_bp`. `value_vendor` is doc 10 §6.2: unknown rarity counts as Common, the hidden part of `eff_enh` (blessing or curse penalty) as 0 because stored enhancement is public (doc 10 §2.1), an `OMEN` item as cursed with penalty 1. Bearer modifiers are `item.trinket.merchants_scale` (9500) and `item.palimpsestic.colophon_mark` (9000). `reputation_bp` is 9000 allied, 9500 friendly, 10000 neutral, 11500 unfriendly, and always 10000 at the Colophon vendor (doc 13 §13.2); a hostile faction's vendor refuses trade. `BUY_TRADE_FLOOR = 7500 > SELL_TRADE_CAP = 5500`, so no combination of Charm, reputation, haggling, trinkets, Grade or boons makes buying and reselling profitable.

Vendors refuse bound items (doc 10 §7.2), `cat.quest` items, `item.remains`, `item.tool.emendation` and Palimpsestic items. Example (the §20 SMG, value 1382, Grade 0): sell at CHM 40 is `mul_bp(1382, 3000) = 414`; unappraised, `value_vendor = 172` (Common) and it sells for 51; a gunsmith sells it at `mul_bp(1382, 17500) = 2418`.

**Haggling** (deterministic, no draw), once per player, shop and Folio, through the intent `ShopHaggle { shop }`: `score = skill.speech + floor_div(CHM - 40, 2) - (20 + min(60, d))`. `score >= 0` → `haggle_bp = min(2000, 500 + 50 × score)`; `-20 <= score < 0` → 0; `score < -20` → `haggle_bp = -500` for that player at that shop this Folio. Example: Speech 40, CHM 70, Folio 7, gunsmith: `score = 40 + 15 - 27 = 28`, `haggle_bp = 1900`; `buy_trade = 10000 → 17500 → 15925 (row 37 × 9100) → 15925 → 15925 → 15925 → 12899`, and the SMG costs `mul_bp(1382, 12899) = 1782`.

### 12.3 Restock, buyback and services

- **Restock**: counter-charge, counter-grenade, `alch.bomb.salt`, `alch.bomb.smoke`, food, bandage, torch, rope and ammunition lines refill to their initial quantity on every tick `entry_tick + 3600 × k` (`k >= 1`). Every other line never restocks, so potions, texts and equipment stay scarce. No draws.
- **Buyback**: each shop keeps the last 12 items each player sold it this Folio, visible to the whole party, for sale back at the exact sell price paid with knowledge unchanged (doc 10 §10.3); cleared at Folio exit.
- **Services**, each price × `gp.shop_price_bp` then `max(1, ·)`:

| Service | Where | Price (Lucre) | Owner of the effect |
|---|---|---|---|
| `appr.svc.appraise` | every vendor, Scriptorium | `mul_bp(20, P(d))` | doc 10 §5.9 |
| `appr.svc.classify`, `appr.svc.identify`, `appr.svc.unbind` | Scriptorium | `mul_bp(40, P(d))`, `mul_bp(60, P(d))`, `mul_bp(150, P(d))` | doc 10 §5.9 |
| Recharge a wand | Scriptorium, arcanist | `mul_bp(30, P(d))`; charges to max, max − 1, never below 1 | doc 06 §8.4 |
| Cure an affliction | Scriptorium | `mul_bp(500, P(d))` | Canon §16.4 |
| Reattach a severed part | Scriptorium | `mul_bp(200, P(d))` | Canon §9.1 |
| Emend the Flesh (remove a mutation) | Scriptorium | `mul_bp(400, P(d))` | doc 04 §9.3 |
| Trainer: one glyph | arcanist | `mul_bp(100 × tier, P(d))` | doc 06 §8.1 |
| Rewrite, reforge | Scriptorium | §15, §16.3 | here |
| Perk choice, tuning | Scriptorium | free | here |

A Scriptorium is a doc 12 Scriptorium room, the Colophon's station, or a `shop.scribe`.

---

## 13. Economy

### 13.1 Item value

Canon §13.9 makes `value` this chapter's; doc 10 §6.1 restates the same formula as `value_true` for identification, and the two must stay identical. Each arrow is one `mul_bp` (the rarity step is an integer multiply), in order:

```
value = base_value → × P(min(ilvl, 300)) → × rarity value (×1, 2, 4, 8, 16, 32, 64; Canon §13.3)
                   → × quality_bp → × material value_bp (doc 07; mat.iron and no material 10000)
                   → × (10000 + 1000 × eff_enh)          only if the category hides ENH (doc 10 §2.1)
                   → × buc_value_bp                      cursed 5000; blessed consumable 12500; else 10000
                   → × (5000 + floor_div(5000 × charges.cur, max(1, charges.max)))   only if it hides CHARGES
value = max(1, value), or 0 when base_value = 0
```

Consumables use rarity ×1 and their quality. `value` is recomputed whenever an input changes; `illuminated` never changes it. Stacks are priced per unit. Example (§20): `86 → × 16771 = 144 → × 8 = 1152 → × 12000 = 1382 → × 10000 → × 10000 → × 10000 = 1382` (material `value_bp` taken as 10000).

### 13.2 Lucre faucets and sinks

Per player per standard Folio at Grade 0, `LUCRE_TARGET(d) = mul_bp(450, P(d))`, above doc 10's floor of `mul_bp(300, P(d))`. Faucet shares: kills 3000 Bp (40 standard-kill equivalents, Canon §17.4, × 5000 chance × mean amount), floor piles 3000, containers 1500, sales 2500; doc 14 marginal notes add up to `2 × mul_bp(30, P(d))`. Sink targets: shops 5500, Fortune Engines 2000, services 1500, carried forward 1000. Abridged multiplies the target by 16667 (`mul_bp(LUCRE_TARGET(d_eff), 16667)`), matching its scaled lines and floor loot.

| d | Target | Kills | Piles | Chests | Sales (rest) | Notes (max) | Brass spin 1 | Silver spin 1 | Gilt spin 1 |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 450 | 140 | 135 | 66 | 109 | 60 | 60 | 160 | 420 |
| 5 | 635 | 190 | 189 | 92 | 164 | 84 | 84 | 225 | 592 |
| 10 | 977 | 290 | 291 | 141 | 255 | 130 | 130 | 347 | 912 |
| 15 | 1503 | 460 | 450 | 220 | 373 | 200 | 200 | 534 | 1403 |
| 20 | 2313 | 710 | 690 | 341 | 572 | 308 | 308 | 822 | 2159 |
| 25 | 3559 | 1100 | 1065 | 523 | 871 | 474 | 474 | 1265 | 3322 |

Kills are `40 × 5000 Bp × mean(range_incl(mul_bp(4, P), mul_bp(10, P)))`; piles are `6 × mean(range_incl(mul_bp(18, P), mul_bp(27, P)))`; chests are 5 plain chests × 7000 Bp × the mean of their range. Σ `LUCRE_TARGET(d)` for `d = 1..=25` is 38,102 Lucre per player. Lucre never converts to any account currency (Canon §14). Bots (doc 02) assert each Folio's realized income within 7000..=13000 Bp of target. Balance anchors: five brass capsules cost one Folio's target income; a Folio-10 Epic weapon (value 1320 for a long blade) costs 1980 at a 15000-markup Colophon, two Folios of income; healing potions (doc 08 value 30) cost `mul_bp(mul_bp(30, P(d)), 14000)` at a general store, 42 Lucre at Folio 1.

### 13.3 Power budget against the canon anchors

- The §18 chain is the Canon §10.4 source chain without `base_mhp`, so its maximum at ilvl 25 is `ITEM_MULT_MAX` times `P(25)`: 410,341 stepwise (the one-step product `mul_bp(79110, 51870)` is 410,343), the IP 4103 of §18. Nothing in this chapter multiplies `B` outside that chain except the three Canon §5.9 "× N" gimmicks (Canon §10.4), which are situational.
- At Folio 1 a Common Normal +0 weapon meets the Canon §17.4 anchors (30 DPS, 2.0 s TTK). An Epic Excellent ilvl-1 drop multiplies `B` by `mul_bp(11500, 12000) = 13800`, and its best unconditional affix (`affix.dmg_pct` T1, at most +600 Bp) gives `mul_bp(13800, 10600) = 14628`, a TTK of `2000 × 10000 / 14628 = 1367` ms. An Epic is 150 of 10010 weight (1.5 %) per generic Folio-1 line, so the anchor holds for the typical run.
- Situational affixes stack additively in step 1, one per exclusivity group. The largest single-target bonus on a 4-affix Palimpsestic weapon is +17300 Bp (`dmg_vs_tag` 5500, `dmg_vs_state` 5000, `dmg_vs_rank` 3800, `dmg_vs_origin` 3000), each gated by a target condition.
- Degenerate loops closed here: Engine pity 9 (§11.2), `LUCRE_FIND_CAP` (§9.1), the buy floor above the sell cap (§12.2), no restock of potions, texts or equipment (§12.3), `GEAR_TOL_CAP` (§3.4), and stack merges that never raise value (§14.2).

---

## 14. Inventory

### 14.1 Model

- **Backpack** (DECISIONS 66: 20 base slots plus bags, a weight limit and encumbrance): 20 slots (`INV_SLOTS_BASE`), +6, +10 or +14 from the `slot.back` pack (§3.4). One slot holds one item or one stack; size and shape never matter.
- **Equipment slots**: Canon §13.2. Equipped items use no backpack slot. The quickbar (4, or the belt's 5 to 8) references backpack stacks.
- **Keyring**: keys, up to 20, no slot. **Ammunition reserve**: per doc 05 family, no slot, carry caps from doc 05 §8.1 (counter charges 5 each). **Purse**: Lucre `i64`, per Erratum (Canon §14), no mass.
- **Mass**: `carried_g` = Σ `mass_g × stack` over backpack, equipment, keyring and ammunition reserve (doc 05 §8.1 masses), except that an equipped item bearing `curse.leaden` or `curse.pact.bulwark` counts `mul_bp(mass_g, 10000 + m)` with that curse's `m` (doc 10 §7.3, §7.8); its stored `mass_g` is unchanged. Capacity is Canon §6.4 row 10 × 1000 g; `load_bp` drives `st.encumbered` and `st.overburdened` (Canon §11.1). Doc 03 maintains `carried_g` from these rules.

### 14.2 Stacking

- Two stacks merge iff template, `origin_genre`, `buc`, knowledge state and flags and `quality` are equal, neither has affixes, and, for damage sources (`cat.throwable`, `cat.gadget`), `ilvl` is equal.
- Other stackables (potions, texts, food, tools, containers, parts, books) merge across `ilvl`; the merged stack takes the lower `ilvl` and its value is recomputed, so merging never raises value.
- Thrown weapons (`arch.ranged.thrown`) merge only when every field except `uid` and `stack` is equal.
- The merged stack keeps the lower `uid`; splitting takes a new uid from the current Folio's counter. Stack caps: §3.
- `item.lucre` piles never stack as items: picking one up adds its amount to the purse.

### 14.3 Pickup

- Lucre and keys within 1500 mm auto-pickup, except a pile for its own dropper (`dropped_by`, §17). Ammunition auto-pickup follows doc 05 §8.2 (1000 mm, partial take up to the carry cap). `item.trinket.lodestone` doubles both radii.
- Other items need a 10-tick interact. A pickup that does not fit leaves the item on the ground.
- A granted item (Emendation, Keepsake, Volume reward, Engine capsule) that does not fit drops at the receiver's feet as `Instanced(slot)`.
- Pickups that collide in one tick resolve in ascending picker EntityId (doc 16 §9.6).

### 14.4 Ground items and visibility

Every ground item is an entity counted toward `GROUND_ITEM_CAP` (Canon §4). An `Instanced(s)` item is never replicated to another client (doc 16 §6.6).

| State | Event | Next |
|---|---|---|
| `Instanced(s)` (from a player's roll) | player `s` picks up | Held(s) |
| `Instanced(s)` | other players | invisible and unreachable to them |
| `Instanced(s)` | slot `s` becomes Departed | `Public` |
| Held(s) | `DropItem` | `Public` |
| Held(s) | `DropItemFor { to_slot }` (doc 16 §9.6) | `Instanced(to_slot)`; refused for quest items, Keepsakes and a Departed, Dormant or ghost target |
| `Public` | any player picks up | Held |
| `Instanced` or `Public` | Lost Satchel merge | Satchelled (visibility kept) |
| any ground state | falls into lava or leaves the Folio box | destroyed |
| any ground state | Folio exit | destroyed |

### 14.5 Lost Satchel

```rust
fn on_ground_insert(f: &mut Folio) {
    if f.ground_count() <= GROUND_ITEM_CAP { return; }
    let s = f.lost_satchel_or_create(f.incipit_anchor());           // a ground container; counts as 1
    for pass in 0..2 {                                               // pass 1 ignores Errata proximity
        for it in f.ground_items_by_uid() {                          // ascending uid
            if f.ground_count() <= GROUND_ITEM_CAP - 64 { return; }
            if it.is_quest() || it.is_corpse() || it.is_satchel() { continue; }
            if pass == 0 && f.within_any_erratum(it.pos, 8000) { continue; }
            s.absorb(it);                                            // keeps visibility; Lucre piles add per finder
        }
    }
}
```

The Lost Satchel shows each player only their own and Public contents, has no capacity limit, and is destroyed at Folio exit. Quest items and corpses never enter it, so if they alone exceed the cap the count stays above it until one is picked up.

---

## 15. Rewrite

- **Where**: a Scriptorium (doc 12 room, the Colophon's station or a `shop.scribe`), or anywhere with `item.tool.rewrite_quill` (60-tick channel, consumed). The target genre is the local genre at the user (the Folio's genre in a seam zone).
- **Refused** for Palimpsestic, quest and mundane items (mundane categories have no skin to change), when the target equals `origin_genre`, when the target has no skin for the archetype (`—`), and when a Legendary perk, Exotic intrinsic, Gloss, bound glyph or a wand's spell would have an illegal dependency in the target.
- **Cost** at a Scriptorium scales with item power (DECISIONS 67): `max(mul_bp(20, P(d)), mul_bp(value_vendor, 3500))`, then × `gp.shop_price_bp`, where `value_vendor` is doc 10 §6.2's value of the item as its owner knows it (§12.2). Example: an identified item of value 1382 at Folio 9: `max(39, 483) = 483`. The Quill costs nothing more.
- **Offers** (DECISIONS 67: a choice of 3). The Scriptorium shows exactly 3 offers, generated once and stored (Canon §3.5). With `o = offer_rng(slot, item uid, "rewrite")`, offer `k` (0, 1, 2) takes manufacturer `cand[k % n]` and runs `rewrite` with `Rng::from_seed64(o.next_u64())` as its replacement generator: 3 draws in all, one per offer in offer order. `cand` lists the candidates of the table below with the default first and the others in ascending ID, and `n = |cand|`; an "unchanged" or "none" row gives the one-entry list of the item's current manufacturer or of none. Offers that share a manufacturer differ only in their replacement draws, and are identical when nothing is replaced. Committing one offer pays the cost and advances `rng.offer.p<slot>` once; declining advances nothing. The Quill applies offer 0's manufacturer, with replacement draws from keyed `rng.loot` (actor = item uid, tick = use tick, seq = slot).

| Item's manufacturer | Candidates in target genre `g` | Default |
|---|---|---|
| a core-genre manufacturer in row `i` (1 to 3) of its genre in §5.2 | every manufacturer of `g` that makes the item's class, plus the Marginal Press | `g`'s row-`i` manufacturer if it makes the class, else `g`'s first such manufacturer in row order, else the Marginal Press |
| Somnambule, the Bureau | as above | `g`'s row-1 rule |
| `mfr.errata.marginal_press` | unchanged | unchanged |
| none (generated in `genre.unwritten`) | as for row 1 | row-1 rule |
| Exotic's fixed manufacturer | unchanged | unchanged |
| target `genre.oneiric`, `.redacted`, `.ur_text` | Somnambule if it makes the class, the Bureau, the Marginal Press respectively, plus the Marginal Press | the genre's own manufacturer, else the Marginal Press |
| target `genre.unwritten` | none | none (tendencies removed) |

```rust
fn rewrite(it: &mut ItemInstance, target: GenreId, mfr_choice: Option<MfrId>, rng: &mut Rng) {
    it.origin_genre = target;                                         // skin, base dependency, Req follow (Canon §5.3)
    if !it.exotic && !it.is_marginal_press() {
        it.manufacturer = mfr_choice;                                 // §15 table
        it.reapply_tendencies();                                      // doc 05 base stats × new tendencies
    }
    it.materials = it.materials.iter().map(|m| doc07::equivalent(*m, target)).collect();
    for a in it.affixes.iter_mut().filter(|a| !a.is_curse()) {        // ascending affix index
        if !class_legal(a.id, target) {                               // same tier and roll_bp; new affix id
            a.id = weighted_pick(legal_affixes_same_cats_unused_groups(it, target), rng);   // 1 weighted
            it.knowledge.clear_affix_flags(a);
        }
    }
    for c in it.perk_columns.iter_mut().filter(|c| c.column <= 4) {
        for o in c.options.iter_mut() { if !perk_legal(*o, target) { *o = weighted_pick(column_pool(it, c.column, target), rng); } }
    }
    it.set_column5(perk_genre(target));
    it.rewrite_count += 1;
    it.recompute_value();
}
```

| Persists | Changes |
|---|---|
| uid, template, rarity, quality, ilvl, enhancement, buc, curse penalty and curse affix, charges, gear tier, chosen indices, Legendary perk, Exotic intrinsic and name, Gloss, bound glyph, wand spell (its glyph origins follow `origin_genre`, doc 06 §8.4), tuning, illuminated, finder slot, reforge count | origin genre, skin, dependencies and `Req`, manufacturer and tendencies (non-Exotic), material, illegal affixes and perk options, column 5, value, `rewrite_count` |

Every committed Scriptorium Rewrite counts for doc 14 `run.note.scribe`.

---

## 16. Emendations, reforging, salvage and Scriptorium actions

### 16.1 Emendation

`item.tool.emendation` (`cat.tool`, mundane, unsellable). Use on an equipment item: 30-tick channel. Canon §13.7: if `buc = cursed` it uncurses instead; otherwise `ilvl = max(ilvl, current Folio)`, then `enhancement += 1`. Refused at the cap: 15 at every depth up to 25, `15 + floor_div(d_current - 25, 5)` in Marginalia and in Second Edition (Canon §13.7 `ENH_CAP`; Canon gaps 2). Placement: §9.9.

### 16.2 Marginalia Emendation flow

On `ev.folio_exited` from a Marginalia Folio (`mode.endless`, `f >= 26`), every slot that is Active or Ghost receives one Emendation created with Folio `f`'s uid counter, in ascending slot order, before the next Folio is generated. If it does not fit, it appears at the receiver's feet at the next Incipit as `Instanced(slot)`. No Volume-plan Emendations exist in Marginalia.

**Second Edition** (DECISIONS 102, 106; doc 14 §8.9). The Errata enter with their gear unchanged: no item is re-leveled, re-rolled or removed at entry. Second Edition Folio `n` (`n = 1..=15`) uses depth `d = 25 + 2 × n` wherever this chapter reads `d` or `d(f)`: `P`, `T`, `ilvl`, affix tier gates, `LUCRE_TARGET`, prices and services. Second Edition Folios are not Marginalia: no Marginalia-only line fires, §16.2 grants nothing, and the §9.9 Volume plan places `EMENDATIONS_PER_VOLUME` in each Second Edition Volume as in the main run. `ENH_CAP` there is `15 + floor_div(d_current - 25, 5)`: 15 at `n = 1` and 2 (`d` 27, 29), 16 at `n = 3`, 21 at `n = 15` (`d` 55). A Second Edition Folio's fused genre pair (doc 14 §8.9) supplies `origin_genre` by the local-genre rule of §2.3 step 4.

### 16.3 Reforging

At a Scriptorium, the player picks one non-curse affix; `offer_rng(slot, item uid, "reforge")` shows 3 candidates (9 draws: per candidate an eligible affix by §6.1 excluding the item's other groups, a tier by §6.1 weights, a `roll_bp`). Committing one replaces the affix, clears its knowledge flags, pays, advances the stream once and increments `reforge_count`; declining costs nothing and advances nothing.

```
cost = mul_bp(mul_bp(30, P(d)), rarity_value_bp) × (2^(c+1) - 1) → × gp.shop_price_bp,   c = reforge_count ∈ 0..=3; refused at 4
```

`rarity_value_bp` is 10000 × the Canon §13.3 value multiplier. Example: Epic at Folio 9: `mul_bp(mul_bp(30, 19925), 80000) = 472`; successive reforges cost 472, 1416, 3304, 7080.

### 16.4 Salvage

60-tick channel anywhere, or 0 ticks at a crafting station (doc 07). Yields `mul_bp(mul_bp(component mass, 6000), 10000 + X)` of each component material as `cat.material` units (doc 07 rounds to its unit), where `X` is the item's `affix.engineered` value after potency (§6.1; 0 without one), and one loose part of the item's manufacturer pool with chance Common 0, Uncommon 1000, Rare 2500, Epic 5000, Legendary or better 10000 (keyed `rng.loot`, actor = item uid, tick, seq = slot: 1 `roll_bp` and 1 `weighted`, always both). An item without a manufacturer yields no part. Palimpsestic, quest and bound items cannot be salvaged.

### 16.5 Free Scriptorium actions

Perk choice per column, Tier-5 tuning, and viewing every displayed odd. Doc 02 `CommitOffer` carries the column or tuning choice.

---

## 17. Co-op loot

- **Instanced (default).** Every kill, container, Reliquary, bag, Engine and shop shelf is rolled per Active, non-ghost player with `seq = slot` (doc 16 §9.6); `finder_slot` records the roller. Rolled Lucre piles are instanced like any drop, and purses are per Erratum (Canon §14).
- **Transfers.** `DropItem` makes an item Public; `DropItemFor` makes it `Instanced(to_slot)` (doc 16 §9.6).
- **Lucre between players** (Canon §14, DECISIONS 83). Lucre passes between party members only as dropped piles inside one run. The intent `DropLucre { amount }` (doc 02) removes `amount` from the dropper's purse and creates one `item.lucre` pile of that amount at the dropper's feet, `Public`, with `fresh = false` (§9.1) and `dropped_by` = the dropper's slot. It is refused unless `1 <= amount <= purse`, and refused for a ghost, a downed Erratum and an invader. Auto-pickup (§14.3) never takes a pile for its own dropper, who takes it back only with the 10-tick interact; any other party member that can pick up items takes it normally. Trades and `DropItemFor` never carry Lucre (doc 16 §9.6), and no Lucre leaves the run or reaches another account.
- **Invaders** (Canon §17.9). An invader is not a party member: it receives no loot rolls and cannot pick up, buy or open anything. If the party kills the invader, every Active Erratum receives one in-run Reliquary roll (§9.12) with `actor` = the invader's EntityId. If the party is Redacted, every Erratum's purse is set to 0 (no pile drops) and one equipped non-weapon item is destroyed: one keyed `rng.loot` draw (actor = that Erratum's EntityId, tick = the Redaction tick, seq 0), `range(n)` over eligible items sorted by uid, no draw when `n = 0`.
- **Ghost loss** (Canon §17.6, doc 16 §9.5): one keyed `rng.loot` draw (folio = the new Folio, actor = the ghost's EntityId, tick = its `folio_entry_tick`, seq 0), `range(n)` over equipped non-weapon items sorted by uid, no draw when `n = 0`.
- **`mode.loot_shared`** (Canon §17.5). Each kill or container rolls once with `seq = 0`, effective SER = `floor_div(Σ effective SER of Active Errata, n)`, and drops Public items. Scaled lines use `shared_drop_bp[n] = [10000, 12800, 16000, 18286]` instead of `party_drop_bp`, so each player receives 8000 Bp of the solo yield; a line chance above 10000 rolls twice (a 10000 line, then a line of the excess), with 4 draws always. Reliquaries use the draft of §10; shops keep one shelf (slot 0's stock); deck and bad-luck counters belong to slot 0. Engines and Volume rewards stay per player.

---

## 18. Item Power

```
chain = 10000 → × P(eff_ilvl) (T for armor and ward generators) → × rarity power → × quality_bp
              → × (10000 + 600 × eff_enh) → × material dmg_bp or ar_bp (ward generators, accessories and implants: 10000)
IP    = floor_div(chain, 100) + Σ affix tier + 5 × [legendary perk] + 10 × [exotic or palimpsestic intrinsic]
```

- `IP_true` uses every field; it feeds telemetry and the bots of doc 02.
- `IP_shown` uses only what the viewing player knows (doc 10 §2.4): unknown rarity counts as Common with no affixes and no perk or intrinsic, the hidden part of `eff_enh` as 0 (doc 10 §2.1), an unknown affix name or roll as tier 1. Quality, material and `ilvl` are never hidden (doc 10 §2.1).
- Character Power = `floor_div(Σ IP_shown over the best weapon slot, five armor slots and the ward slot, 7)`, empty slots 0.
- Examples: a Common Normal +0 iron ilvl-1 weapon has IP 100. The §20 SMG has `IP_true` 237 (chain 23143, 3 affixes of tier 2) and `IP_shown` 201 while `know.classified` (chain 20125). A Palimpsestic Peerless +15 material-14000 weapon at ilvl 25 has chain IP 4103 before affixes. IP is UI and telemetry only.

---

## 19. Saved loot state

`PlayerLootState` per slot (saved, Canon §2.5): deck counts, `blp_rare`, `blp_leg`, `rq5`, `rq20`, `engine_pity[3]`, Exotics acquired this run, haggle and buyback records of the current Folio, purse. Per Folio: Engine faces and spins per slot, Reliquary shared states and `opened` flags, shop shelves and restock ticks, Lost Satchel, pending Volume-reward offers, and the stored Rewrite and reforge offers per slot and item (Canon §3.5: an offer is generated once and stored). Per run: Palimpsestic uniques acquired by the party. `rng.offer.p<slot>` states are saved by the canon contract.

---

## 20. Worked example: one weapon drop

Run seed `0x0123456789ABCDEF`, Standard mode, Grade 10 (`gear_tier = min(5, 1 + floor_div(10, 5)) = 3`, `gp.lucre_bp = 9000` from Grade 3). Folio 7 (Volume 2, position 2) in `genre.weird_west`, party of 2. Receiving player slot 1, effective SER 52, fresh deck and counters, no Lucre-find gear. An elite (`loot_role` marksman, EntityId `(7 << 40) | 42`) dies at tick 55606. 411 items were created earlier in Folio 7. Doc 07's Weird West ranged-weapon material pool is taken as `mat.hexed_silver` 1500, `mat.iron` 7000.

1. **Drop generator.** `stream_seed(run, "rng.loot", 7) = 0x4e6b3f6f4dafc989`; keyed seed `0x83db3ac3a7065305`.
2. **Lines of `loot.drop.elite`** (2 draws each, 10 in all):

| Line | Chance | `range(10000)` | Hit | Item seed |
|---|---|---|---|---|
| Lucre | 10000 | 5808 | yes | `0x80b11f0132e41905` |
| item | `mul_bp(3000, 8000) = 2400` | 1593 | yes | `0xe5bf711b4609c44a` |
| bound key | `mul_bp(1500, 8000) = 1200` | 8978 | no | `0x649a68f2aa2be66e` |
| lucky bag | 0 (not Marginalia) | 1204 | no | `0x6989235ffde8ebe2` |
| ammunition | `mul_bp(7500, 8000) = 6000` | 7116 | no | `0xef13337ccb980d90` |

3. **Lucre.** `range_incl(mul_bp(4, 16771), mul_bp(10, 16771)) = range_incl(6, 16)` → 10; × 30000 → 30; × 9000 → **27 Lucre**, a fresh pile.
4. **Category.** `sub(seed, "loot.cat")` has seed `mix64(0xe5bf711b4609c44a ^ 0x3418799fa9ecd0ed) = 0x59f6955537f06994`. Weird West has TEC 4, so the implant card joins accessory. Weights with the marksman bias: `[50000, 125000, 50000, 20000, 40000, 10000, 10000, 60000, 40000, 30000, 40000, 10000, 10000]`, total 495000; `range` gives 95600 → past melee (50000) → **`cat.weapon_ranged`**; its count becomes 4.
5. **Archetype.** Weights in ID order: beam 400, charge_rifle 1000, crossbow 800, launcher 400, pistol 1000, pistol_heavy 700, rifle_auto 700, rifle_marksman 800, shotgun 800, smg 800, sniper 500, staff 700, thrown 600 (total 9200); draw 6688 → **`arch.ranged.smg`** (cumulative 6600..=7399), skin "Gatling Pistol [T]", `Req[techne] = max(1, 4 - 2) = 2`; tags `pierce`, `magazine`, `ammo`, `ricochet`, `firearm`.
6. **Rarity.** Canon §13.4 at `d = 7`, SER 52: `[7000, 2860, 960, 285, 99, 37, 0]`; elite shift 2000: `[7000, 3432, 1344, 456, 178, 74, 0]`, total 12484; draw 11893 → cumulative 11776..=12231 → **Epic** (3 affixes, columns 1 to 4). `blp_rare` resets to 0; `blp_leg` becomes 1.
7. **Manufacturer.** `[Marginal Press 1000, Coffin Creek 3000, Seven Seals 3000, Sundown Rail 3000]`; draw 8474 → **`mfr.weird_west.sundown_rail`** (D+800, S−700; Sticky Fuse, dependency techne, `Req = 2`).
8. **Quality.** Draw 9552 of 10000 → cumulative 9300..=9799 → **`q.excellent`** (12000).
9. **Material.** `[hexed_silver 1500, iron 7000]`; draw 42 → **`mat.hexed_silver`**.
10. **Affixes** (ilvl 7 unlocks T1 and T2; Epic halves T1: tier weights `[5000, 6000]`):

| # | Eligible (count, Σ Wt) | Affix draw | Affix | Tier draw | Tier | `roll_bp` | Value |
|---|---|---|---|---|---|---|---|
| 1 | 46, 1895 | 1830 | `affix.self_injector` (tech, `Req[techne] = 2`) | 10629 | 2 | 4095 | heals 100 Bp of Max HP every 818 ticks |
| 2 | 45, 1875 | 926 | `affix.infuse_fire` (infused → mythos, `Req[mythos] = 4`) | 10429 | 2 | 4630 | 1731 Bp to `dmg.fire`, +24 `st.burning` power |
| 3 | 34, 1600 (infusion group removed) | 397 | `affix.dmg_vs_beast` (mundane) | 5274 | 2 | 1146 | +1880 Bp vs `fac.wild.*` |

11. **Perk columns** (family G, 2 options each; pools 5, 7, 19 and 17 after removing column 3's picks): column 1 `perk.choke`, `perk.rifled_bore`; column 2 `perk.drum_feed`, `perk.tempered`; column 3 `perk.kill_clip`, `perk.unrelenting`; column 4 `perk.headhunter`, `perk.frenzy`; column 5 `perk.genre.weird_west` (Dead Man's Hand). Traits are enhanced at tier 3: `perk.kill_clip` grants `mul_bp(2500, 12500) = 3125` Bp.
12. **Curse** by doc 10 from `sub(seed, "loot.buc")` (3 draws); no other value depends on it. **Illuminated**: not rolled (below Legendary). No bound glyph or spell (not an implement or wand).
13. **Derived.** `uid = (7 << 40) | 412`; `value = 1382` (§13.1); `finder_slot = 1`; `visibility = Instanced(1)`; knowledge `know.classified` with every flag clear; `IP_true` 237, `IP_shown` 201 (§18). The record is the §2.2 example. At Grade 10 (`shop_price_bp` 11500, `lucre_bp` 9000) and CHM 40 a gunsmith charges `mul_bp(1382, 20125) = 2781` for it and pays `mul_bp(1382, 2700) = 373` for it (§12.2).

---

## Exports

All IDs below are in namespaces Canon §19 assigns to doc 11, pass the Canon §1.1 grammar, are at most 64 bytes and have pairwise distinct `fnv1a32`, also against canon IDs and the §3.10 retired IDs (acceptance test 30). Each is listed in full; the section named in the first column holds its data. The §3.10 retired IDs are aliases, not exports. The `loot.` sub-generator tags are reserved names, never records.

| Namespace | IDs |
|---|---|
| `item.` accessories (52, §3.4) | `item.amulet.composure`, `item.amulet.elements`, `item.amulet.ferocity`, `item.amulet.fortune`, `item.amulet.sanctuary`, `item.amulet.scholar`, `item.amulet.vigor`, `item.amulet.wayfarer`, `item.back.cloak`, `item.back.expedition_pack`, `item.back.rucksack`, `item.back.satchel`, `item.belt.bandolier`, `item.belt.harness`, `item.belt.sash`, `item.belt.utility`, `item.face.goggles`, `item.face.mask`, `item.ring.arcane_tongue`, `item.ring.bold_tongue`, `item.ring.finesse`, `item.ring.fortune`, `item.ring.intellect`, `item.ring.keen`, `item.ring.machine_tongue`, `item.ring.might`, `item.ring.perception`, `item.ring.presence`, `item.ring.sacred_tongue`, `item.ring.vigor`, `item.ring.warding`, `item.ring.wealth`, `item.ring.will`, `item.trinket.cat_eye`, `item.trinket.clockwork_heart`, `item.trinket.compass_rose`, `item.trinket.dragon_scale`, `item.trinket.forged_papers`, `item.trinket.holy_symbol`, `item.trinket.hourglass`, `item.trinket.keyring_charm`, `item.trinket.lattice_lens`, `item.trinket.lodestone`, `item.trinket.lucky_coin`, `item.trinket.merchants_scale`, `item.trinket.powder_horn`, `item.trinket.reliquary_bone`, `item.trinket.saints_finger`, `item.trinket.thimble`, `item.trinket.warning_bell`, `item.trinket.whetstone`, `item.trinket.wishbone` |
| `item.` consumables, tools and books (48, §3.5) | `item.book.primer_abjuration`, `item.book.primer_alteration`, `item.book.primer_conjuration`, `item.book.primer_divination`, `item.book.primer_evocation`, `item.book.primer_illusion`, `item.book.primer_restoration`, `item.book.primer_spellcraft`, `item.book.schematic`, `item.food.bread`, `item.food.meal`, `item.food.ration`, `item.text.amnesia`, `item.text.appraisal`, `item.text.blink`, `item.text.consecrate`, `item.text.demolition`, `item.text.inscribed`, `item.text.intuit`, `item.text.mapping`, `item.text.mending`, `item.text.omen`, `item.text.recharge`, `item.text.sanctify`, `item.text.silence`, `item.text.summoning`, `item.text.survey`, `item.text.warding`, `item.throwable.counter_anchor`, `item.throwable.counter_breaker`, `item.throwable.counter_disruptor`, `item.throwable.counter_staggerer`, `item.throwable.counter_sunder`, `item.tool.bandage`, `item.tool.blank_text`, `item.tool.crutch`, `item.tool.emendation`, `item.tool.grapple`, `item.tool.hacking_deck`, `item.tool.lockpicks`, `item.tool.pick`, `item.tool.rewrite_quill`, `item.tool.rope`, `item.tool.salve`, `item.tool.splint`, `item.tool.surgical_kit`, `item.tool.toolkit`, `item.tool.torch` |
| `item.` bags, keys, currency, quest and corpse (15, §3.5, §9.7) | `item.bag.boss`, `item.bag.burnt`, `item.bag.editor`, `item.bag.elite`, `item.bag.lucky`, `item.bag.refined`, `item.key.bound`, `item.key.door`, `item.key.gilded`, `item.key.illumined`, `item.lucre`, `item.quest.catchword`, `item.quest.folio_leaf`, `item.quest.parcel`, `item.remains` |
| `item.` Exotics and Palimpsestic uniques (34, §8.2, §8.3) | `item.exotic.black_bar`, `item.exotic.brass_lungs`, `item.exotic.briar_rose`, `item.exotic.cloud_step`, `item.exotic.cogheart`, `item.exotic.dead_drop`, `item.exotic.dragonsong`, `item.exotic.drifters_duster`, `item.exotic.gallows_oath`, `item.exotic.ghost_protocol`, `item.exotic.hydra_bow`, `item.exotic.ironhide_greaves`, `item.exotic.lullaby`, `item.exotic.paper_moon`, `item.exotic.quietus`, `item.exotic.redline_sutra`, `item.exotic.rust_psalm`, `item.exotic.stampede_sabatons`, `item.exotic.starfall`, `item.exotic.stormcrown`, `item.exotic.thunderwright`, `item.exotic.tidecaller_wraps`, `item.exotic.veil_of_vespers`, `item.exotic.widowmaker`, `item.palimpsestic.authors_quill`, `item.palimpsestic.blank_page`, `item.palimpsestic.colophon_mark`, `item.palimpsestic.dog_ear`, `item.palimpsestic.endpaper_band`, `item.palimpsestic.errata_sheet`, `item.palimpsestic.first_word`, `item.palimpsestic.marginal_gloss`, `item.palimpsestic.omnibus`, `item.palimpsestic.running_head` |
| `rar.` (7, §3.8) | `rar.common`, `rar.epic`, `rar.exotic`, `rar.legendary`, `rar.palimpsestic`, `rar.rare`, `rar.uncommon` |
| `cat.` (26, §3.1) | `cat.accessory`, `cat.ammo`, `cat.armor`, `cat.body_part`, `cat.book`, `cat.container`, `cat.corpse`, `cat.currency`, `cat.food`, `cat.gadget`, `cat.implant`, `cat.ingredient`, `cat.key`, `cat.material`, `cat.offhand`, `cat.part`, `cat.placeable`, `cat.potion`, `cat.quest`, `cat.text`, `cat.throwable`, `cat.tool`, `cat.wand`, `cat.ward_gen`, `cat.weapon_melee`, `cat.weapon_ranged` |
| `slot.` (23, §3.9) | `slot.back`, `slot.belt`, `slot.body`, `slot.face`, `slot.feet`, `slot.hand_extra_1`, `slot.hand_extra_2`, `slot.hands`, `slot.head`, `slot.implant_arms`, `slot.implant_head`, `slot.implant_legs`, `slot.implant_torso`, `slot.legs`, `slot.neck`, `slot.offhand`, `slot.ring_l`, `slot.ring_r`, `slot.trinket`, `slot.ward`, `slot.weapon_1`, `slot.weapon_2`, `slot.weapon_3` |
| `affix.` (113, §6.2) | `affix.aether_cost`, `affix.ambusher`, `affix.arc_proc`, `affix.armor_pct`, `affix.bleed_power`, `affix.buildup`, `affix.cast_speed`, `affix.charges`, `affix.crit_chance`, `affix.crit_mult`, `affix.dmg_pct`, `affix.dmg_vs_beast`, `affix.dmg_vs_construct`, `affix.dmg_vs_editor`, `affix.dmg_vs_elite`, `affix.dmg_vs_undead`, `affix.engineered`, `affix.executioner`, `affix.expose_power`, `affix.featherweight`, `affix.first_strike`, `affix.frugal`, `affix.gloss`, `affix.handling`, `affix.infuse_acid`, `affix.infuse_arcane`, `affix.infuse_fire`, `affix.infuse_frost`, `affix.infuse_psychic`, `affix.infuse_radiant`, `affix.infuse_radiation`, `affix.infuse_shock`, `affix.infuse_sonic`, `affix.infuse_toxin`, `affix.infuse_umbral`, `affix.interloper`, `affix.kill_aether`, `affix.kill_haste`, `affix.kill_heal`, `affix.kill_stamina`, `affix.leech`, `affix.lucre_find`, `affix.magazine`, `affix.penetration`, `affix.poise`, `affix.poise_dmg`, `affix.precision`, `affix.quiet`, `affix.radiance`, `affix.range`, `affix.rate`, `affix.reach`, `affix.recharge`, `affix.recoil`, `affix.reload`, `affix.resist_acid`, `affix.resist_all`, `affix.resist_arcane`, `affix.resist_blunt`, `affix.resist_elements`, `affix.resist_fire`, `affix.resist_frost`, `affix.resist_pierce`, `affix.resist_psychic`, `affix.resist_radiant`, `affix.resist_radiation`, `affix.resist_shock`, `affix.resist_slash`, `affix.resist_sonic`, `affix.resist_toxin`, `affix.resist_umbral`, `affix.ricochet`, `affix.self_injector`, `affix.soft_step`, `affix.spell_dmg`, `affix.spread`, `affix.stability`, `affix.stamina`, `affix.stun_power`, `affix.sub_agility`, `affix.sub_analysis`, `affix.sub_attunement`, `affix.sub_brawn`, `affix.sub_charm`, `affix.sub_endurance`, `affix.sub_fate`, `affix.sub_focus`, `affix.sub_grip`, `affix.sub_guile`, `affix.sub_hearing`, `affix.sub_intuition`, `affix.sub_logic`, `affix.sub_memory`, `affix.sub_menace`, `affix.sub_power`, `affix.sub_precision`, `affix.sub_providence`, `affix.sub_resilience`, `affix.sub_resolve`, `affix.sub_serendipity`, `affix.sub_sight`, `affix.sub_sleight`, `affix.sub_toughness`, `affix.summon_vigor`, `affix.thorns`, `affix.tol_arcana`, `affix.tol_mythos`, `affix.tol_techne`, `affix.tol_verve`, `affix.ward_capacity`, `affix.ward_full`, `affix.ward_nova`, `affix.ward_siphon` |
| `perk.` column perks (51, §7.2) | `perk.adrenaline`, `perk.arcane_surge`, `perk.balanced_tang`, `perk.bloodletting`, `perk.cadence`, `perk.chain_reaction`, `perk.choke`, `perk.close_quarters`, `perk.counterweight`, `perk.coup_de_grace`, `perk.deep_reservoir`, `perk.dragonfly`, `perk.drum_feed`, `perk.extended_mag`, `perk.feeding_frenzy`, `perk.focused_conduit`, `perk.frenzy`, `perk.guarded_hilt`, `perk.headhunter`, `perk.heavy_bore`, `perk.honed_edge`, `perk.hot_loads`, `perk.kill_clip`, `perk.light_grip`, `perk.long_barrel`, `perk.long_haft`, `perk.momentum`, `perk.opening_shot`, `perk.outlaw`, `perk.overflow`, `perk.ported_barrel`, `perk.quick_draw`, `perk.quick_feed`, `perk.quick_siphon`, `perk.rampage`, `perk.ricochet_rounds`, `perk.rifled_bore`, `perk.serrated`, `perk.sharpshooter`, `perk.slipstream`, `perk.stable_reservoir`, `perk.steady_feed`, `perk.stillness`, `perk.sunder`, `perk.swift_conduit`, `perk.tempered`, `perk.trench_barrel`, `perk.unrelenting`, `perk.vorpal`, `perk.weighted_head`, `perk.wide_conduit` |
| `perk.` genre traits (16, §7.2) | `perk.genre.cosmic_horror`, `perk.genre.cyberpunk`, `perk.genre.fairy_tale`, `perk.genre.gothic_horror`, `perk.genre.hardboiled_noir`, `perk.genre.mythic_antiquity`, `perk.genre.mythic_fantasy`, `perk.genre.oneiric`, `perk.genre.post_apocalypse`, `perk.genre.redacted`, `perk.genre.space_opera`, `perk.genre.steampunk`, `perk.genre.unwritten`, `perk.genre.ur_text`, `perk.genre.weird_west`, `perk.genre.wuxia` |
| `perk.` Legendary perks (26, §8.1) | `perk.leg.adamant`, `perk.leg.bloodsong`, `perk.leg.bulwark`, `perk.leg.cartographer`, `perk.leg.cold_blood`, `perk.leg.echoing_strike`, `perk.leg.fortunes_favor`, `perk.leg.grounded`, `perk.leg.hunters_mark`, `perk.leg.inkdrinker`, `perk.leg.last_stand`, `perk.leg.lifeline`, `perk.leg.mirror_edge`, `perk.leg.mirror_plate`, `perk.leg.overcapacitor`, `perk.leg.overture`, `perk.leg.phoenix_lining`, `perk.leg.recursion`, `perk.leg.redoubt`, `perk.leg.second_skin`, `perk.leg.shadowweave`, `perk.leg.spellweaver`, `perk.leg.stormcall`, `perk.leg.thornmail`, `perk.leg.windrunner`, `perk.leg.worldbreaker` |
| `mfr.` (40, §5.2) | `mfr.cosmic_horror.dunwold`, `mfr.cosmic_horror.hollow_lantern`, `mfr.cosmic_horror.outer_choir`, `mfr.cyberpunk.chromaw`, `mfr.cyberpunk.ghostline`, `mfr.cyberpunk.neon_saint`, `mfr.errata.marginal_press`, `mfr.fairy_tale.glass_cobbler`, `mfr.fairy_tale.thimbleweald`, `mfr.fairy_tale.thornwood`, `mfr.gothic_horror.ashgrave`, `mfr.gothic_horror.grimhallow`, `mfr.gothic_horror.vesper_sons`, `mfr.hardboiled_noir.halloran`, `mfr.hardboiled_noir.midnight_avenue`, `mfr.hardboiled_noir.rainslick`, `mfr.mythic_antiquity.argive`, `mfr.mythic_antiquity.delphic`, `mfr.mythic_antiquity.hephaesteum`, `mfr.mythic_fantasy.deepanvil`, `mfr.mythic_fantasy.runecourt`, `mfr.mythic_fantasy.silverbough`, `mfr.oneiric.somnambule`, `mfr.post_apocalypse.geigerhaus`, `mfr.post_apocalypse.lastditch`, `mfr.post_apocalypse.rustmaw`, `mfr.redacted.bureau`, `mfr.space_opera.perihelion`, `mfr.space_opera.starwright`, `mfr.space_opera.umbra`, `mfr.steampunk.aetherline`, `mfr.steampunk.boilerheart`, `mfr.steampunk.brassworth`, `mfr.ur_text.first_press`, `mfr.weird_west.coffin_creek`, `mfr.weird_west.seven_seals`, `mfr.weird_west.sundown_rail`, `mfr.wuxia.cloudpiercer`, `mfr.wuxia.iron_crane`, `mfr.wuxia.jade_lotus` |
| `loot.` tables (74: §9.5 to §9.8; §9.7 bag contents as `loot.bag.*`; §10 Reliquary contents as `loot.reliq.*`; §11 Engine faces as `loot.engine.*`; §12.1 shop stock as `loot.shop.*`; §2.3 Keepsakes) | `loot.bag.boss`, `loot.bag.burnt`, `loot.bag.editor`, `loot.bag.elite`, `loot.bag.lucky`, `loot.bag.refined`, `loot.boss.editor`, `loot.boss.mini`, `loot.boss.palimpsestic`, `loot.chest.locked`, `loot.chest.plain`, `loot.companion.bramble`, `loot.companion.deputy_cole`, `loot.companion.dr_halloway`, `loot.companion.ensign_okafor`, `loot.companion.gearwright`, `loot.companion.glitch`, `loot.companion.iris_kane`, `loot.companion.kallias`, `loot.companion.lan_feiyu`, `loot.companion.mother_vesper`, `loot.companion.rust_mae`, `loot.companion.sir_brannoc`, `loot.drop.champion`, `loot.drop.elite`, `loot.drop.hunter`, `loot.drop.minion`, `loot.drop.standard`, `loot.engine.armory`, `loot.engine.brass`, `loot.engine.curio`, `loot.engine.gilt`, `loot.engine.silver`, `loot.engine.wardrobe`, `loot.floor`, `loot.keepsake.cosmic_horror`, `loot.keepsake.cyberpunk`, `loot.keepsake.fairy_tale`, `loot.keepsake.gothic_horror`, `loot.keepsake.hardboiled_noir`, `loot.keepsake.mythic_antiquity`, `loot.keepsake.mythic_fantasy`, `loot.keepsake.oneiric`, `loot.keepsake.post_apocalypse`, `loot.keepsake.redacted`, `loot.keepsake.space_opera`, `loot.keepsake.steampunk`, `loot.keepsake.unwritten`, `loot.keepsake.ur_text`, `loot.keepsake.weird_west`, `loot.keepsake.wuxia`, `loot.quest.t1`, `loot.quest.t2`, `loot.quest.t3`, `loot.reliq.bound`, `loot.reliq.gilded`, `loot.reliq.illumined`, `loot.reliq.plain`, `loot.secret.cache`, `loot.secret.palimpsest`, `loot.secret.vault`, `loot.shop.apothecary`, `loot.shop.arcanist`, `loot.shop.armorer`, `loot.shop.colophon`, `loot.shop.fence`, `loot.shop.general`, `loot.shop.gunsmith`, `loot.shop.scribe`, `loot.shop.tinker`, `loot.shop.wanderer`, `loot.trophy`, `loot.volume_reward`, `loot.war_spoils` |
| `loot.` pool tags (12, §2.5) | `loot.pool.brews_2`, `loot.pool.cross_glosses`, `loot.pool.exotic_armor`, `loot.pool.exotic_arms`, `loot.pool.gadgets_2`, `loot.pool.glyphs_2`, `loot.pool.implants_2`, `loot.pool.legendary_perks_1`, `loot.pool.legendary_perks_2`, `loot.pool.recipes_2`, `loot.pool.relics_2`, `loot.pool.throwables_2` |
| `loot.` reserved sub-generator tags (19, §2.3) | `loot.affix`, `loot.ammo`, `loot.arch`, `loot.book`, `loot.buc`, `loot.bundle`, `loot.cat`, `loot.exotic`, `loot.glyph`, `loot.illum`, `loot.key`, `loot.legendary`, `loot.lucre`, `loot.material`, `loot.mfr`, `loot.perk`, `loot.quality`, `loot.rarity`, `loot.spell` |
| `reliq.` (4) | `reliq.bound`, `reliq.gilded`, `reliq.illumined`, `reliq.plain` |
| `engine.` (6) | `engine.armory`, `engine.brass`, `engine.curio`, `engine.gilt`, `engine.silver`, `engine.wardrobe` |
| `shop.` (10) | `shop.apothecary`, `shop.arcanist`, `shop.armorer`, `shop.colophon`, `shop.fence`, `shop.general`, `shop.gunsmith`, `shop.scribe`, `shop.tinker`, `shop.wanderer` |

Data constants: `party_drop_bp`, `shared_drop_bp`, deck counts, role bias, tier gates and weights, Engine face tables, `ENGINE_PITY_NEED = 9`, `ENGINE_PITY_NEED_WISHBONE = 7`, `CURSED_FACE = 4`, `palimpsestic_bp`, `LUCRE_TARGET(d)`, `LUCRE_FIND_CAP = 5000`, `BUY_TRADE_FLOOR = 7500`, `SELL_TRADE_CAP = 5500`, `GEAR_TOL_CAP = 3`, `INV_SLOTS_BASE = 20`, base values and template weights of §3. Functions: `generate_item`, `generate_keepsake`, `drop_rng`, `place_rng`, `offer_rng`, `value` (§13.1). Intents validated here (doc 02 §5.5): `ShopHaggle { shop }` (§12.2) and `DropLucre { amount: i64 }` (§17).

## Interface assumptions

| Doc | This chapter assumes it provides |
|---|---|
| 02 | intents `ShopBuy`, `ShopSell`, `CommitOffer`, `DropItem`, `UseItemOn`, `EquipItem`, `MoveItem`, plus `ShopHaggle { shop }` (triage C-15) and `DropLucre { amount: i64 }` (§17); bots that assert the §13.2 income band; mod content IDs with second segment `x_<mod_id>` (Canon §1.1, §1 principle 8), and a mod loot patch that targets a doc 11 table such as `loot.chest.plain` and appends to its `lines` |
| 03 | ground-item entities; block damage for `perk.leg.worldbreaker` on blocks tagged `block.tag.destructible` only (Canon §4.3); `carried_g` from §14.1; torch light. Present: `start_grapple(e) -> bool` and the grapple mode (doc 03 §5.24) that `item.tool.grapple` calls (§3.5); `Projectile.home_bam_s` and `home_acc` (doc 03 §5.19) for `mfr.cyberpunk.ghostline` (§5.2) |
| 04 | gear sub-attribute modifiers; food satiation, rot and eat ticks (copied in §3.5); `bp.arm_extra_1` and `bp.arm_extra_2` granting `slot.hand_extra_1` and `slot.hand_extra_2` (§3.9); treatments; implant-form `aug.*` with region points (doc 04 §9.4), of which §2.5 lists the 8 that `loot.pool.implants_2` gates (doc 04 tags none); "Emend the Flesh"; the composure-loss hook; Speech XP for haggling; kit roles |
| 05 | the `arch.*` IDs of §3 with their stats; tendency letters mapped to its fields; overridable fields (`fire_mode`, `sig`, `innate_counter`, `max_ricochets`); ammunition families, values, carry caps and pickup sizes; counter charges and `grenade_counter` bits; the ward capacity chain; `arch.ranged.wand`, `arch.ranged.staff` and `arch.offhand.tome` in its export list |
| 06 | staff, focus and tome casting fields; wand charges by rarity; the bound-glyph list; spell texts; inscription consuming `item.tool.blank_text` and creating `item.text.inscribed`; trainer glyphs; the eight schools, whose `item.book.primer_<school>` glyph tomes this chapter fills with `glyph_tome` and doc 06 §8.1 reads (§3.5 Books) |
| 07 | material pools per (genre, material role) with `value_bp`, `dmg_bp`, `ar_bp`; `equivalent(material, genre)`; mass (items have no durability, Canon §4.3); the 22 `part.*` IDs of §5.4; the 6 base gadgets and 6 `gadgets_2`; salvage units; resource nodes for `item.tool.pick`; the `learn: 1` and `r2` recipe flags and the read rule of `item.book.schematic` (§3.5 Books); `gadgets_2` and `r2` membership, whose tags `loot.pool.gadgets_2` and `loot.pool.recipes_2` this chapter owns (§2.5) |
| 08 | potion and bomb effects and values, Reliquary-only potion values, vessels, ingredient lists for apothecaries and `loot.trophy`; the mechanics of `apparatus.respirator.cloth`, `.mask` and `.rebreather` (doc 08 §4.3), whose loot records live in §3.4 |
| 09 | noise in dB; mimic as a hidden object; `aware.suspicious`; lockpick and hacking-deck rules. Present: `lock.kind.container` with `key_tags` for `loot.chest.locked` (§9.6), door keys read through `key_tag` (§2.2, §3.5), `TOOL_Q` by tool quality, and `outfit_of` set by stripping (doc 09 §11.1, §13.1); no attempt consumes `item.tool.lockpicks`, so shops sell one set |
| 10 | `roll_curse(rng, draft, local_myt, source)` with exactly 3 draws; `value_true` identical to §13.1; `value_vendor`; knowledge flags and sealing; `wand_pool`; `appr.svc.*`; pacts, `eligible_pacts` and the `CursedCapsule` source of `roll_curse` for the Fortune Engine cursed capsule (§11.2); `curse.pact.miser`'s unit `m` (§9.1); the 3000 Bp potion and 1000 Bp text malign shares that §3.5 meets; and that `PriceObs.trade_bp` records the composite buy or sell trade of §12.2 |
| 12 | spots for chests, Reliquaries, Engines, shops, Scriptoria, Volume-plan and counter caches; `room.treasury` and Locked edges for locked chests; the rest Folio's shop and Engine hall; secret kinds; at most one Palimpsest vault per Volume; Colophon and Interleaf rooms; challenge success judged at exit commit; placement of `item.quest.folio_leaf` and its story record (§3.5). Present: the story relic spot, the story's 2 drawn domains and its solve (doc 12 §13, §20), which §9.6 fills; `Key(k)` nodes that receive `item.key.door` |
| 13 | `loot_role`, `no_loot` and `trophy` template fields; `enemy.mimic`; tags undead and fiend; vendor NPCs `npc.vendor.<type>` for each `shop.<type>`; the reputation price factor; quest tiers 1 to 3; companion personal quests; carried items through §3.7 |
| 14 | `RunState.grade`, `GradeParams`, Folio positions and `d(f)`, the Volume reward phase, Bindery pool tags and `unlock.room.fortune_engines_2`, `unlock.room.wandering_scribe`, `run.boon.deep_pockets`; `run.rubric.catchwords` placing `item.quest.catchword`; and that its kits name only owner IDs (doc 05 §5.6 for weapons) |
| 15 | the `generate_keepsake` caller and the Satchel |
| 16 | slot states, `DropItemFor`, the instanced replication rule, pickup collision order; trades and `DropItemFor` refusing `cat.currency`, so Lucre passes only through §17's dropped piles |
| 19 | the serialized schema: `11.template_loot` records keyed by the owner's template ID in `template` (`x-idref`) with no ID of their own (§2.1); `lines` of `loot.` tables marked `"x-patchable": true` (§1 principle 8). Still requested: doc 19 drops the derived `loot.tpl.` IDs |
| 17, 18 | odds panels, the ledger, the Engine UI with the cursed capsule's Ominous mark and share (§11.2), sealed cards, vendor screens, the three Rewrite offers (§15); skin, vendor and manufacturer localization and the name review |

**Former names.** Earlier drafts of docs 02, 04 and 13 used names in this chapter's namespaces that no owner exports. They are written unmarked here because they are not content IDs; every chapter uses the owner ID on the right. Doc 05 §5.6 resolves the former `arch.` and `ammo.` kit role names.

| Former name (doc) | Owner ID |
|---|---|
| item.food.snack (04) | `item.food.bread` |
| loot.drop.boss, loot.drop.editor (13) | `loot.boss.mini`, `loot.boss.editor` |
| loot.mythic_fantasy.chest_common (02, mod patch example) | `loot.chest.plain` |

## Canon gaps

Canon-1.11 closes every gap this chapter listed at canon-1.4 (audit/canon_gaps_triage.md). The canon adopted the Marginalia enhancement cap, the "× N" traits, the extra stream uses, offer contexts, "dispensing" for reels, Unwritten shops and counter caches, per-Erratum Lucre, labeled placement sub-streams, acquisition-time illumination and the two extra-hand slots (G-131, G-084, G-025, G-030, G-026, G-161, G-148, G-027, G-134, G-130). The five counters were already canon (G-171), and the staff, wand and tome namespace gap was stale (doc 05 exports them). The rest are this chapter's rules, stated in the body: Unmoored SER (G-058, §2.3 and §11.2), `GEAR_TOL_CAP` (G-062, §3.4), the Emendation's category (G-136, §16.1), value ownership (G-138, §12.2 and §13.1), book sources (G-146, §3.5), retired aliases (G-147, §3.10), the haggle intent (G-223, Interface assumptions) and deep pockets on fresh piles only (G-224, §9.1).

1. **Abridged kill drops.** Canon §17.1 defines Abridged "loot quantity" as worldgen chests and floor-loot spots, leaves Reliquaries, Fortune Engines and vendor stock unchanged, and does not mention kill drops. This chapter also multiplies scaled kill-drop lines by 16667 Bp (§9.1), matching Abridged's × 16667 kill XP so that a 15-Folio run yields the kill loot of a 25-Folio one; §13.2's Abridged Lucre target assumes it. Proposed: §17.1 adds "and scaled kill-drop lines (doc 11 §9.1)".
2. **Second Edition `ENH_CAP`.** DECISIONS 102 and 106 carry the Errata into Second Edition with their gear at depth `25 + 2n`, and canon-1.11 §13.7 states `ENH_CAP` for the main run and Marginalia only. This chapter applies the Marginalia formula to Second Edition Folios (§16.1, §16.2) without making them Marginalia (doc 14 §8.9). Proposed Canon §13.7 text: "`ENH_CAP = 15` in the main run and `15 + floor_div(d_current - 25, 5)` in Marginalia and in Second Edition, whose Folio `n` has `d_current = 25 + 2n`." The grappling tool needs no canon text: doc 03 §5.24 states its movement and §3.5 defines `item.tool.grapple`.
3. **Seeds.** DECISIONS 128 cuts seeds; Canon §13.1 still lists the row "| cat.seed | plantable seeds | 07, 08 |". This chapter retires the category (§3.1, Exports 26 `cat.` IDs). Proposed: Canon §13.1 deletes that row and Canon §17.7 lists cat.seed as retired.

## Acceptance tests

1. **Owner IDs.** Every template ID in §3 is either in this chapter's Exports or in its owner's export list (doc 05 `arch.*` and `ammo.*`, doc 06, doc 08 `alch.*` and `apparatus.*`, doc 04 `aug.*`, doc 07 `gadget.*` and `part.*`); no ID in another owner's namespace is defined here.
2. **Worked example.** The §20 inputs yield keyed seed `0x83db3ac3a7065305`, line draws 5808, 1593, 8978, 1204, 7116, item seed `0xe5bf711b4609c44a`, 27 Lucre, category draw 95600, archetype draw 6688 (`arch.ranged.smg`), rarity draw 11893 (Epic), manufacturer draw 8474, quality draw 9552, material draw 42, affix draws 1830/10629/4095, 926/10429/4630, 397/5274/1146, the eight perk options, value 1382 and the §2.2 instance exactly.
3. **Sub-seed.** `fnv1a64("loot.cat") = 0x3418799fa9ecd0ed`; the §20 category sub-generator seed is `0x59f6955537f06994`.
4. **Step isolation.** Adding a draw to `loot.affix` changes no output of any other tag.
5. **Keyed isolation.** The §20 drop is identical, except `uid`, whether 0 or 50 other entities died earlier in tick 55606.
6. **Rarity shift.** `d = 7`, SER 52, shift 2000: `[7000, 3432, 1344, 456, 178, 74, 0]`. With `st.unmoored` the SER term uses 40; a Placed source has Exotic weight 0.
7. **Party scaling.** `party_drop_bp = [10000, 8000, 6666, 5714]`; `shared_drop_bp = [10000, 12800, 16000, 18286]`; a mini-boss line stays 10000 at every party size.
8. **Bad luck.** `blp_rare = 10` multiplies Rare-or-better weights by 15000; `blp_rare = 24` sets the floor to Rare; `blp_leg = 120` sets it to Legendary; an Engine face, a shop line or a Keepsake changes neither counter.
9. **Deck.** 40 cards; 40 `Decrement` draws with no role bias empty the deck exactly once; the 41st draw follows a refill; a category with no eligible template (the part card in `genre.unwritten`) is skipped by redrawing with its weight 0.
10. **Volume plan.** For run seed `0x0123456789ABCDEF`: `place_rng(1, "vplan", 0)` is seeded with `0x98dd1f20ed8bb553` and gives `u = [1, 3, 0, 0]`, `q = 0`, `b1 = b2 = 0`, `g = 2`: Emendations at Folios 1, 3 and 4 and none in the Editor bag, the Quill at Folio 1, both bound key caches at Folio 1, the gilded key cache at Folio 3. Volume 2 gives `u = [2, 2, 2, 0]`: Emendations at Folios 6 and 9 and one in the Folio-10 Editor bag, Quill at 8, bound keys at 6 and 9, gilded at 8. At Grade 12 only the first bound key cache is placed.
11. **Emendation spread.** Exhaustive enumeration of the 120 `(u1..u4)` tuples gives 0.6 expected Emendations at each of the 5 positions and exactly 3 per Volume.
12. **Affix values.** Tier 2 `affix.dmg_vs_beast` at 1146 → 1880; tier 2 `affix.self_injector` at 4095 → 818; tier 2 `affix.infuse_fire` at 4630 → 1731 and 24 status power.
13. **Affix legality.** No generated item in `genre.post_apocalypse` or `genre.hardboiled_noir` carries an arcana component; none in `genre.redacted` a verve component; every `genre.unwritten` item has only dependency-none components.
14. **Skins.** The four §4.2 tables have 448 cells; validation rejects any tagged axis whose axiom is 0; staves and wands depend exactly on their genre's aether axis; `arch.ranged.staff` and `arch.ranged.wand` are never generated in `genre.unwritten`.
15. **Manufacturers.** Each core genre's three manufacturers together make all ten classes; a Weird West SMG's candidates total 10000 with the Marginal Press first; no display name matches the doc 18 brand list.
16. **Gloss.** A Coffin Creek Gloss on a Wuxia Great Bow has `Req[verve] = 5`.
17. **Reliquary odds.** A plain Reliquary's equipment line at Folio 7, SER 40 shows 68.2854 %, 22.2278 %, 6.4581 %, 2.2028 %, 0.8260 %; a gilded Reliquary's Rare-floor line at Folio 7 has weights `[0, 0, 1420, 490, 193, 82, 0]`.
18. **Reliquary pity.** With `rq5 = 4` the next Reliquary's best line cannot roll below Epic; with `rq20 = 19` not below Legendary.
19. **Mimic.** Opening a mimic keeps the key and spawns `enemy.mimic`; after its death every player opens their instance without a key and gets the extra line; an interrupted channel spends nothing and draws nothing.
20. **Engine odds and prices.** Brass at SER 70: `[2300, 2600, 1800, 1800, 900, 600]`, cursed share 900 Bp; pity 9 (7 with `item.trinket.wishbone`): `[0, 0, 0, 0, 0, 10000]`, cursed share 0. Folio 7 prices at Grade 0: brass 100, 125, 150, 175, 200; silver 268, 335, 402, 469, 536; a sixth purchase is refused. A Monte Carlo of 10^6 fresh brass sequences at SER 40 averages 8.75 ± 0.02 capsules to the top face.
21. **Engine commitment.** Reopening shows identical faces; an open advances `rng.offer.p<slot>` by 0 and a purchase by exactly 1; every accepted payment yields exactly one item; the Engine schema has no stake, multiplier or empty-outcome field.
22. **Shop prices.** Value 1382 at Grade 0: sell 414 (CHM 40), 51 unappraised, gunsmith price 2418; Speech 40, CHM 70, Folio 7 gives `haggle_bp = 1900`, `buy_trade = 12899` and price 1782; at Grade 10 the gunsmith charges 2781 and pays 373. For every combination of the §12.2 modifiers, `buy_trade >= 7500 > 5500 >= sell_trade`.
23. **Reforge and Rewrite.** Epic at Folio 9: reforges cost 472, 1416, 3304, 7080 and a fifth is refused; Rewrite costs 483 for an identified item of value 1382 at Folio 9; a Rewrite keeps every "Persists" field of §15.
24. **Lucre.** `LUCRE_TARGET` rows of §13.2 match; Σ over Folios 1 to 25 is 38,102. A fresh 27-Lucre pile picked up with `Σ find` 900 gives 29; `Σ find` above 5000 counts as 5000; a pile dropped from a purse never gains find.
25. **IP.** Common Normal +0 iron ilvl 1: 100; §20 SMG: `IP_true` 237, `IP_shown` 201 at `know.classified`; maximum ilvl-25 chain: 4103.
26. **Lost Satchel.** With 2048 ground items, inserting one more moves items in ascending uid (skipping quest items and corpses, and in the first pass items within 8000 mm of an Erratum) until the count is at most 1984.
27. **Stacking.** Two `alch.potion.healing` with different `buc` never merge; healing potions of ilvl 3 and 5 merge at ilvl 3; two `arch.ranged.grenade_frag` of different ilvl never merge.
28. **Marginalia Emendations.** Exiting Folio 26 grants 1 Emendation to each Active and Ghost slot; the cap at Folio 30 is 16.
29. **Counters.** In each of the 15 genres with shops, every shop stocks all five counter charges, all five counter grenades, `alch.bomb.salt` and `alch.bomb.smoke`; in all 16 genres a 4-player Folio whose exit preview shows `elite.warded` places a cache holding 2 `item.throwable.counter_breaker` and 2 `ammo.charge_breaker` per player in the next Folio.
30. **IDs.** Every exported ID, sub-generator tag included, passes the grammar with distinct `fnv1a32`, including against canon IDs and the §3.10 retired IDs.
31. **Folio isolation.** Shop shelves, floor loot, caches, Reliquary and Engine placements of Folio 7 are byte-identical, uids included, for party sizes 1 to 4 and whether Folios 1 to 6 were played.
32. **Uniques.** No party holds two copies of a Palimpsestic unique; a player receives no Exotic twice before that category's Exotics are exhausted; an undispensed Engine face does not count as acquired.
33. **Malign share.** Harmful weight is exactly 3000 of 10000 for found potions, 2700 of 9000 for Reliquary-only potions and 1000 of 10000 for texts (doc 10 §8).
34. **Perk pools.** For every weapon archetype and every genre where it has a skin, columns 1 and 2 hold at least 3 legal perks and columns 3 and 4 at least 6.
35. **Fixed draw counts.** `place_rng` labels draw exactly: `reliq` 9, `engine` 4, `shop` 3, `vplan` 8 (Abridged 4), `floor` 9 (Abridged 15), `cache` 16; `roll_curse` 3; every loot line 2.
36. **Invasions.** Killing an invader at Folio 4 gives each Active Erratum the contents of one `reliq.bound` without a key; a Redaction empties every purse and destroys one equipped non-weapon item per Erratum with one keyed draw, none when there is no eligible item.
37. **Ammunition line.** A receiver whose weapons all have full reserves gets no ammunition and the line's sub-generator draws nothing; otherwise the family follows doc 05 §8.3 need weights.
38. **Gear tolerance.** Two `item.ring.machine_tongue` at `eff_enh` 8 plus an `affix.tol_techne` give `gear_tol[techne] = 3`.
39. **Books.** An arcanist glyph-tome line at Folio 7 draws 1 value in `loot.arch` and at most 2 in `loot.book`, and teaches only glyphs of the drawn primer's school of tier 2 or lower from the run's glyph loot pool. A schematic line draws 1 value in `loot.book` and teaches a `learn: 1` recipe of the content set, never an `r2` recipe without `loot.pool.recipes_2`. Neither line changes with what any player knows.
40. **Restated tables.** The §3.8 rarity rows equal Canon §13.3 and the §3.9 slot rows equal Canon §13.2 field for field. Exports lists every ID exactly once and no §3.10 retired ID.
41. **Cursed capsule.** Face 4 of every Engine is generated with `roll_curse` source `CursedCapsule` (exactly 3 draws in `loot.buc`) and is a `buc.cursed` pact item, sealed with `RARITY` and `OMEN` set; faces 0 to 3 and 5 are always `buc.uncursed` with no curse draw. The panel shows the cursed share: 900 Bp at brass, 1000 Bp at silver, its variants and gilt, 0 with the pity guarantee armed. Dispensing face 4 adds 1 to the tier's pity counter.
42. **Lucre between players.** `DropLucre { amount: 50 }` from a purse of 120 leaves 70 and creates one Public 50-Lucre pile with `fresh = false`; the dropper's auto-pickup skips it; another party member picking it up gains exactly 50 (no Lucre find, boon or pact). Amounts 0 and 121 are refused, as is a drop by a ghost, a downed Erratum or an invader. No trade or `DropItemFor` carries Lucre.
43. **Rewrite offers.** A Weird West `mfr.weird_west.sundown_rail` SMG Rewritten in `genre.cyberpunk` shows exactly 3 stored offers, with manufacturers `mfr.cyberpunk.ghostline` (the default: Chromaw, Cyberpunk's third row, does not make RG), `mfr.cyberpunk.neon_saint` and `mfr.errata.marginal_press`, in that order; generating them draws 3 values from the offer generator and advances `rng.offer.p<slot>` by 0; committing one advances it by exactly 1. A Marginal Press item shows 3 offers that keep the Marginal Press.
44. **No wear, no mounts.** No instance field, affix, perk, manufacturer tendency or gimmick, part, service, shop or loot line of this chapter reads or writes durability, and none names a mount; the §3.10 table holds no retired mount ID. `affix.engineered` changes only salvage yields, and `affix.self_injector` heals at most once per smallest X ticks per bearer.
45. **Seeds cut (§3.1).** No template, loot line, deck card, shop line or salvage yield names a seed; Exports lists 26 `cat.` IDs and cat.seed is not among them.
46. **Grappling tool (§3.5).** An accepted `item.tool.grapple` use at tick 1000 calls `start_grapple` and sets `ready_tick = 1180`; a use at tick 1179 is refused and leaves `ready_tick` at 1180; a use `start_grapple` refuses spends nothing; `shop.general` stocks 1, the deck never draws it, and its skin exists in all 16 genres with dependency none.
47. **Manufacturer hooks (§5.2).** A charged `mfr.cyberpunk.neon_saint` shot passes only the first penetrable cell without spending penetration and stops at fixed terrain; a `mfr.gothic_horror.grimhallow` reload that would leave the bearer below 1,000 mHP is refused and starts no reload; a `mfr.cyberpunk.ghostline` projectile turns 273 or 274 Bam per tick from `home_bam_s` 16384 and `home_acc`.
48. **Implant pool (§2.5).** Without `loot.pool.implants_2`, step 2 never yields `aug.adrenal_pump`, `aug.extra_arm`, `aug.neural_jack`, `aug.reactive_skin`, `aug.smartlink`, `aug.spring_tendons`, `aug.subdermal_ward` or `aug.wings`; every other implant-form `aug.*` is eligible.
49. **Curses on Lucre and mass (§9.1, §14.1, §3.4).** A fresh 27-Lucre pile with `Σ find` 900 picked up by a bearer of one `curse.greed` (`m` 1500) gives 24 (27 → 29 → 24); a dropped pile or a sale ignores it. An equipped 1000 g item with `curse.leaden` (`m` 10000) counts 2000 g in `carried_g` and keeps `mass_g` 1000. Ring types never exceed 24.
50. **Second Edition (§16.1, §16.2).** Second Edition Folio 3 has `d` 31 and `ENH_CAP` 16; Folio 15 has `d` 55 and `ENH_CAP` 21; exiting a Second Edition Folio grants no Marginalia Emendation; gear carried in is unchanged at entry.
51. **Story relic (§9.6).** A Folio with a story seat draws exactly 1 value from `place_rng(f, "relic", 0)` and places one `Public` accessory of rarity Rare or better, `buc.uncursed`, unidentified, with `story_domains` equal to the story's 2 drawn domains; solving the story identifies it; a Folio without a seat places none and draws nothing.
52. **Keys, locks and outfits (§2.2, §3.5, §9.6).** `item.key.door` with `key_tag` 2 opens the `Locked` edge of key 2, is never spent, is refused by vendors and leaves every inventory at Folio exit; a `loot.chest.locked` chest carries `lock.kind.container` with `key_tags = [0, 0]`; `outfit_of` is set only on `slot.body` armor.
53. **Keepsake content set (§2.3).** `generate_keepsake` draws its step-2 templates from the opener's account content set at open time; the same seed gives the same item for two openers with equal content sets.
54. **Template loot records (§2.1).** Each owner template has at most one `11.template_loot` record, keyed by its `template` field (`x-idref`); no record carries an ID or a `loot.tpl.` prefix, and Exports lists none.

## Invariants annex

```
Σ doc11: template ⊗ instance ⊗ keyed loot ⊗ placed loot ⊗ deck ⊗ pity ⊗ Lucre-only containers
W! every random run item ← generate_item(seed)                     N! item seed reused across lines
W! step draws ⊂ sub(seed, tag)                                      N! one step's draws shift another step
W! drop rng := keyed(rng.loot, folio, actor, tick, slot)          W! 2 draws per loot line, hit or miss
W! placed loot ← place_rng(folio, label, index)                    W! placed loot ⊥ party size ∧ earlier play
W! weighted lists ∈ ascending content ID ∨ fixed table order       N! HashMap iteration in loot
W! rarity := Canon §13.4 → shift → pity → floor                   N! rarity share changed by pool unlocks
W! template ID ∈ own namespaces ∪ owners' exports                  N! ID minted in another owner's namespace
W! component dep ∉ {axis | origin axiom = 0}                       W! staff, wand, focus, tome dep := aether axis
W! harmful share = 3000 Bp potions, 1000 Bp texts                  W! gear_tol[a] ≤ 3
W! Fortune Engine: pay Lucre → exactly 1 item from shown faces     N! wager | multiplier | empty outcome | reels
W! cursed capsule := face 4 ∧ pact ∧ OMEN before payment          N! hidden curse on any other face
W! displayed odds == rolled weights                                N! paid currency → any random container
W! offer generated once, advanced only on commit                   N! reroll on reopen
W! Palimpsestic ∈ {Editor table, Volume reward, palimpsest vault}  N! Palimpsestic sold | salvaged | on faces
W! Exotic ≤ 1 per canon group                                      N! duplicate Exotic before category exhausted
W! enhancement ≤ ENH_CAP                                           W! Emendation re-levels ilvl first
W! value == doc 10 value_true                                      W! buy_trade ≥ 7500 > 5500 ≥ sell_trade
W! Lucre per Erratum, i64; between players only as dropped piles N! Lucre → account currency
N! durability | wear | repair on any item                         N! mount ID or mechanic
W! party loot scaled by party_drop_bp on scaled lines only         N! boss guarantee scaled by party size
W! ground count ≤ GROUND_ITEM_CAP via Lost Satchel (quests, corpses exempt)
W! Fortune|Serendipity → in-run loot only                          N! SER effect while st.unmoored
W! grapple use accepted ⇒ ready_tick := tick + 180               N! refused grapple use spends anything
W! template_loot keyed by template (x-idref)                       N! template_loot record with its own ID
W! Second Edition d := 25 + 2n ∧ ENH_CAP by Marginalia formula     N! Marginalia Emendation in Second Edition
N! seed category | template | line                                 W! story relic ∈ cat.accessory ∧ ≥ Rare ∧ uncursed
```
