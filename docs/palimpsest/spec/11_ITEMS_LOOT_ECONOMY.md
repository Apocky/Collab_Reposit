# 11 — Items, Loot, Reliquaries, Fortune Engines and Economy
As of 2026-10-11. Status: BINDING. Conforms to canon-1.2.

This chapter defines every item template and instance, the deterministic loot generator, genre skins for every weapon archetype, manufacturers, affixes, perks, Legendary perks, Exotics and Palimpsestic uniques, every loot source (kills, chests, in-run Reliquaries, bosses, secrets, Fortune Engines, shops), the Lucre economy, the inventory, Rewriting, Emendations, reforging, salvage, co-op loot and the Item Power score. It was also checked against canon-1.3 (§5.9 manufacturer table, §5.10 World Laws, §5.11 `st.unmoored`, §17.1 Colophon); see Canon gaps 1.

## Imports

| Source | Section | What this chapter consumes |
|---|---|---|
| Canon | §1.1, §1.10 | ID grammar, `id32`, skin text keys `loc.<id>.skin.<suffix>.name`, name resolution order |
| Canon | §1.3, §1.5, §1.6 | `floor_div`, `ceil_div`, `mul_bp`, left-to-right Bp chains; display-unit rule; integer-only content |
| Canon | §1.11 | item `uid` layout, per-Folio counters, ascending-uid iteration |
| Canon | §2.3, §2.5 | "N s" = N × 60 ticks; save contract (every counter in this chapter is saved) |
| Canon | §3.1–§3.5 | `Rng`, `range`, `weighted`, `roll_bp`, sequential `rng.loot_place`, keyed `rng.loot`, commit-only `rng.offer.p<slot>` |
| Canon | §4 | `GROUND_ITEM_CAP = 2048` and the Lost Satchel hand-off |
| Canon | §5.2–§5.6, §5.8 | genre table, axiom-0 generation ban, `Req`, `Tol`, `gear_tol`, Contradiction, Anachronism, local genre |
| Canon | §5.9 (canon-1.3) | the two core manufacturers per genre and the three secret-genre manufacturers with their signature traits |
| Canon | §6.4 rows 10, 13, 14, 19, 37, 38, 42 | carry capacity, gear crit terms, shield stability, buy and sell multipliers, Serendipity |
| Canon | §10.4, §10.5, §10.6 | source magnitude chain, additive bucket, reflected damage entry point |
| Canon | §11.2, §11.4 | status power, direct application |
| Canon | §13.1–§13.9 | categories, slots, rarity table and roll, quality, knowledge, curses, Emendations, `P`, `T`, `S`, `ITEM_MULT_MAX`, instance contract |
| Canon | §14 | `cur.lucre` |
| Canon | §16.4, §16.5 | affliction cure price, sacrifice value |
| Canon | §17.1–§17.7 | run constants, Abridged loot quantity, party tables, ranks, champion counters, reference anchors, `mode.loot_shared`, Ghost item loss, mounts |
| Canon | §18 | `ev.entity_killed`, `ev.item_picked_up`, `ev.item_dropped`, `ev.item_used`, `ev.container_opened`, `ev.engine_spun`, `ev.folio_entered`, `ev.folio_exited` |
| Canon | §19 | this chapter owns `item.`, `rar.`, `affix.`, `perk.`, `mfr.`, `loot.`, `reliq.`, `engine.`, `shop.`, `cat.`, `slot.` |
| Doc 02 | intents | `ShopBuy`, `ShopSell`, `CommitOffer`, `DropItem`, `UseItemOn` |
| Doc 14 | §1.2, §3.1, §3.9, §7.2, §8 | `RunState.grade`, Folio positions, Volume reward hook, `GradeParams.lucre_bp`, `shop_price_bp`, `reliq_key_delta`, modes, Bindery pool tags |
| Doc 15 | §5.4, §5.5 | `generate_keepsake` call contract, Satchel materialization |
| Docs 05, 06, 07, 08, 10, 12, 13 | see Interface assumptions | weapon and armor stats, glyphs and wands, materials, parts, gadgets, potion effects, curse rolls, rooms, enemy templates |

---

## 1. Principles

1. **Template and instance.** A template is immutable content. An instance is simulation state created from a template by the generator in §2.3, and it never changes template except through Rewrite (§15), which keeps the template and changes the skin.
2. **One generator.** Every item that enters a run (kills, containers, Reliquaries, Fortune Engines, shops, bags, Volume rewards, Keepsakes) is produced by `generate_item` (§2.3). Kits (doc 14) are the only exception and draw no random numbers.
3. **Keyed and isolated.** Every random step of item generation draws from its own sub-generator derived from one `u64` item seed. Adding a draw to one step never shifts another step.
4. **Earned randomness.** Every randomized container here is paid for with in-run Lucre or in-run keys only. Odds are displayed before every commitment. Fortune Engines always pay out an item.
5. **Horizontal unlocks.** A Bindery pool tag (`loot.pool.<suffix>`) adds templates inside their rarity band and category only; weights renormalize within the band, so rarity shares never change (doc 15 Script neutrality rule).

---

## 2. Items: templates, instances and generation

### 2.1 Template record

Every template has an ID in `item.` or is an archetype owned by doc 05 (`arch.*`), doc 06 (wands) or doc 07 (`gadget.`, `part.`). This chapter owns the loot-facing fields of all of them.

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
| `base_value` | Lucre at ilvl 1, Common, Normal, +0 (§13.1 value formula) |
| `stack_max` | 1 for every equipment item; §14.3 for stackables |
| `rarity_eligible` | true for weapons, armor, off-hands, accessories, ward generators, wands, implants and mount tokens; false otherwise (always `rar.common`) |
| `template_weight` | weight inside its category for the archetype draw (§2.3 step 2) |
| `pool` | `base` (in `unlock.set.starter`) or a `loot.pool.<suffix>` tag gated by a doc 14 Bindery node |
| `implicit` | the template's fixed effect; `value = base + per_enh × eff_enh` (Canon §13.7 `eff_enh`) |

### 2.2 Instance record

The canon fields of §13.9 are mandatory. This chapter adds the fields marked "doc 11". Doc 19 owns the serialized schema.

| Field | Type | Owner | Meaning |
|---|---|---|---|
| canon §13.9 fields | | canon | `uid`, `template`, `origin_genre`, `rarity`, `quality`, `ilvl`, `enhancement`, `buc`, `curse_penalty`, `knowledge`, `affixes`, `parts`, `materials`, `durability`, `charges`, `stack`, `mass_g`, `value`, `manufacturer`, `gear_tier`, `perk_columns`, `illuminated`, `finder_slot` |
| `genre_trait` | perk ID or none | doc 11 | the column-5 genre trait (§7.2); weapons only |
| `legendary_perk` | perk ID or none | doc 11 | present iff rarity is Legendary (§8.1) |
| `exotic` | bool | doc 11 | true iff `template` is an `item.exotic.*` |
| `reforge_count` | u8 `0..=4` | doc 11 | §16.3 |
| `rewrite_count` | u16 | doc 11 | number of committed Rewrites (§15) |
| `tuning` | `{plus: sub, minus: sub}` or none | doc 11 | Tier-5 armor and accessory tuning (§7.3) |
| `visibility` | `Instanced(slot)` or `Public` | doc 11 | ground visibility (§14.4) |
| `gloss_mfr` | `mfr.` ID or none | doc 11 | manufacturer whose gimmick `affix.gloss` grants (§6.3) |
| `sealed` | bool | doc 11 | true for Reliquary, Engine and Keepsake contents until first inspected; UI only |

`affixes` entries are `{id, tier: 1..=5, roll_bp: 0..=10000}`. The value of an affix is `lo + floor_div((hi - lo) × roll_bp, 10000)` with `(lo, hi)` from its tier row in §6.2; a descending row (`lo > hi`, `affix.self_repair`) uses the same formula and yields a smaller number for a higher roll.

**Example instance** (the §20 worked example after generation):

```json
{
  "uid": "0x0000070000000013", "template": "arch.ranged.sidearm", "origin_genre": "genre.weird_west",
  "rarity": "rar.epic", "quality": "q.excellent", "ilvl": 7, "enhancement": 0, "buc": "buc.uncursed",
  "curse_penalty": 0, "knowledge": { "state": "know.classified", "flags": 0 },
  "affixes": [ { "id": "affix.self_repair", "tier": 2, "roll_bp": 4095 },
               { "id": "affix.infuse_fire", "tier": 2, "roll_bp": 4630 },
               { "id": "affix.dmg_vs_beast", "tier": 2, "roll_bp": 1146 } ],
  "parts": [], "materials": ["mat.hexed_silver"], "durability": { "cur": 0, "max": 0 }, "charges": { "cur": 0, "max": 0 },
  "stack": 1, "mass_g": 0, "value": 1123, "manufacturer": "mfr.weird_west.sundown_rail",
  "gear_tier": 3,
  "perk_columns": [
    { "column": 1, "options": ["perk.choke", "perk.rifled_bore"], "chosen": 0 },
    { "column": 2, "options": ["perk.drum_feed", "perk.tempered"], "chosen": 0 },
    { "column": 3, "options": ["perk.kill_clip", "perk.unrelenting"], "chosen": 0 },
    { "column": 4, "options": ["perk.headhunter", "perk.frenzy"], "chosen": 0 },
    { "column": 5, "options": ["perk.genre.weird_west"], "chosen": 0 } ],
  "genre_trait": "perk.genre.weird_west", "legendary_perk": null, "exotic": false,
  "illuminated": false, "finder_slot": 1, "reforge_count": 0, "rewrite_count": 0,
  "tuning": null, "visibility": "Instanced(1)", "gloss_mfr": null, "sealed": false
}
```

`durability` and `mass_g` are filled by doc 07 from archetype volume and material (shown as 0 here because doc 07 values are not part of this chapter).

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

Every sub-generator is derived from keyed `rng.loot` and counts as `rng.loot` (Canon §3.3). Sequential sources (shop stock, floor loot, Engine faces) obtain the item seed from `rng.loot_place` or `rng.offer.p<slot>` instead; the steps below are identical.

**Drop resolution** (fixed draw count: 2 per line):

```rust
fn roll_drop(table: &LootTable, src: &DropSource, slot: u8, ctx: &mut LootCtx) -> Vec<ItemInstance> {
    let mut d = drop_rng(ctx.folio, src.actor, ctx.tick, slot);
    let mut out = Vec::new();
    for line in &table.lines {                                      // table order
        let chance = line_chance(line, src, ctx);                   // §9.1 multipliers, clamped 0..=10000
        let hit = d.roll_bp(chance);                                // draw 1 of the line, always made
        let seed = d.next_u64();                                    // draw 2 of the line, always made
        if hit { out.extend(resolve_line(line, seed, slot, ctx)); }
    }
    out
}
```

**Item generation.** `generate_item(seed, req)`; `req` carries category or deck, archetype filter, origin genre, `ilvl`, rarity rules (forced rarity, floor, shift), receiving slot and SER, role bias and source tag.

| Step | Sub-generator tag | Draws | Rule |
|---|---|---|---|
| 1 Category | `loot.cat` | 1 `weighted` (0 if fixed) | If `req.category` is fixed, skip. Otherwise draw from the player's category deck (§9.2): weights = `remaining_count × role_bias_bp` (§9.3); decrement the drawn card; refill the deck when every count is 0. |
| 2 Template | `loot.arch` | 1 `weighted` | Eligible templates of the category: in the run content set, pool tag owned, a skin exists in `origin_genre` (§4), and no skin dependency axis has `axiom = 0` there (Canon §5.3). Weights: `template_weight`, sorted by content ID. |
| 3 Rarity | `loot.rarity` | 1 `weighted` (0 if forced or not rarity-eligible) | Canon §13.4 weights with the receiving player's SER (40 while `st.unmoored`, Canon §5.11), then the source shift `w[r] = floor_div(w[r] × (10000 + shift_bp × r), 10000)` for `r >= 1`, then bad-luck multipliers (§9.4), then the floor (weights below it set to 0). Palimpsestic weight stays 0 except in sources that force it. |
| 3b Exotic or Palimpsestic template | `loot.exotic` | 1 `weighted` | If rarity is Exotic: replace the template by an Exotic of the same category (§8.2) not yet generated for this player this run; none left → all of that category become eligible again; no Exotic of that category in the content set → rarity becomes Legendary. Palimpsestic: §8.3 pool, excluding uniques any party member holds. |
| 4 Origin genre | none | 0 | `req.origin_genre`: the local genre at the source position (Canon §5.8); in a seam zone the Folio's genre (canon-1.3 §5.8). |
| 5 Manufacturer | `loot.mfr` | 1 `weighted` (0 if none applies) | §5.3 weights. |
| 6 Quality | `loot.quality` | 1 `weighted` | Weights `[300, 1200, 6000, 1800, 500, 170, 30]` over `q.awful..q.peerless`, then the source quality floor (weights below it set to 0). Consumables: `q.normal`, no draw. |
| 7 Material | `loot.material` | 1 `weighted` | Doc 07 pool for (`origin_genre`, archetype material role), each entry weighted. |
| 8 Affixes | `loot.affix` | 3 per affix | For `i in 0..NAFF[rarity]`: pick an affix (`weighted` over eligible affixes sorted by ID, §6.1), pick a tier (`weighted` over unlocked tiers), `roll_bp = range(10001)`. Stop early when no affix is eligible. |
| 9 Perk columns | `loot.perk` | `n_opt` per rolled column | Columns 1 to 4 present by rarity (§7.1); for each, `n_opt(gear_tier)` weighted picks without replacement; column 4 excludes column 3's options. |
| 10 Genre trait | none | 0 | `perk.genre.<origin suffix>` in column 5. |
| 11 Legendary perk | `loot.legendary` | 1 `weighted` (iff Legendary) | §8.1 pool for the category. |
| 12 Curse state | `loot.buc` | doc 10 | `doc10::roll_curse(sub, category, local MYT)` sets `buc` and `curse_penalty` and may append one `curse.` affix. Shops, Engines and Volume rewards skip this step and set `buc.uncursed`. |
| 13 Illuminated | `loot.illum` | 1 `roll_bp(5)` (iff Legendary or better) | Canon §13.9; sets `illuminated`. |
| 14 Derived fields | none | 0 | `uid` from the Folio counter; `gear_tier = min(5, 1 + floor_div(run.grade, 5))`; `finder_slot = req.slot`; `knowledge` per Canon §13.6 with every flag clear; `value` (§13.1); `mass_g` and `durability` from doc 07; `charges` from doc 06 or 07. |

Step order is the order of the table; steps 1 to 3 must precede the others because they select the template.

```rust
fn generate_item(seed: u64, req: &GenRequest, st: &mut PlayerLootState, run: &RunState) -> ItemInstance {
    let cat   = req.category.unwrap_or_else(|| st.deck.draw(&mut sub(seed, "loot.cat"), &req.role_bias));
    let tmpl  = pick_template(cat, req, &mut sub(seed, "loot.arch"), run);
    let mut rar = if !tmpl.rarity_eligible { RAR_COMMON }
                  else if let Some(r) = req.forced_rarity { r }
                  else { pick_rarity(req, st, &mut sub(seed, "loot.rarity")) };
    let tmpl  = special_template(tmpl, &mut rar, req, st, &mut sub(seed, "loot.exotic"));
    let genre = req.origin_genre;
    let mfr   = pick_manufacturer(&tmpl, genre, rar, &mut sub(seed, "loot.mfr"));
    let q     = pick_quality(&tmpl, req.quality_floor, &mut sub(seed, "loot.quality"));
    let mat   = doc07::pick_material(genre, &tmpl, &mut sub(seed, "loot.material"));
    let mut it = ItemInstance::new(run.next_uid(), &tmpl, genre, rar, q, req.ilvl, mfr, mat, req.slot);
    roll_affixes(&mut it, &mut sub(seed, "loot.affix"));
    roll_perks(&mut it, run.grade, &mut sub(seed, "loot.perk"));
    it.genre_trait = genre_trait_for(&it);
    if rar == RAR_LEGENDARY { it.legendary_perk = Some(pick_legendary(&it, &mut sub(seed, "loot.legendary"))); }
    if req.source.rolls_curse() { doc10::roll_curse(&mut it, &mut sub(seed, "loot.buc"), local_myt(req)); }
    if rar >= RAR_LEGENDARY { it.illuminated = sub(seed, "loot.illum").roll_bp(5); }
    it.finalize_derived(run);   // gear_tier, value, mass, durability, knowledge
    st.blp_update(rar, it.is_equipment());
    it
}
```

**Keepsake entry point** (doc 15 §5.4): `generate_keepsake(genre, rarity, rng)` draws `seed = rng.next_u64()` (1 draw) and calls `generate_item` with `loot.keepsake.<genre_suffix>`, forced rarity, `ilvl = 1`, `origin_genre = genre`, SER 40, no deck (category drawn from the fixed weights of §9.2 restricted to equipment cards), `run.grade = 0` for `gear_tier`, and no bad-luck state.

---

## 3. Archetype master list

### 3.1 Categories and loot roles

| Category | Templates (owner of mechanics) | Rarity | Deck card | Stack | Manufacturer |
|---|---|---|---|---|---|
| `cat.weapon_melee` | 12 `arch.melee.*` (05) | yes | `cat.weapon_melee` | 1 | yes |
| `cat.weapon_ranged` | 13 `arch.ranged.*` (05) | yes | `cat.weapon_ranged` | 1 | yes |
| `cat.wand` | `arch.ranged.wand` (06 loads the glyph type) | yes | `cat.wand` | 1 | yes |
| `cat.armor` | 15 `arch.armor.*` (05) | yes | `cat.armor` | 1 | yes |
| `cat.offhand` | 5 `arch.offhand.*` (05) | yes | `cat.offhand` | 1 | yes |
| `cat.ward_gen` | 3 `arch.ward.*` (05) | yes | `cat.ward_gen` | 1 | yes |
| `cat.mount` | token of `arch.mount.runner`, `hover`, `glider` (05) | yes | none (§9.11) | 1 | vehicle skins only |
| `cat.implant` | `aug.*` (04) | yes | `cat.implant` | 1 | no |
| `cat.accessory` | `item.ring.*`, `item.amulet.*`, `item.trinket.*`, `item.belt.*`, `item.back.*`, `item.face.*` (11) | yes | `cat.accessory` | 1 | no |
| `cat.potion` | `item.potion.*` (11; effects 08) | no | `cat.potion` | 10 | no |
| `cat.text` | `item.text.*` (11; effects 06, 10) | no | `cat.text` | 10 | no |
| `cat.throwable` | `item.throwable.*` (11; effects 05, 08) | no | `cat.throwable` | 10 | no |
| `cat.ammo` | `ammo.*` (05) | no | `cat.ammo` | reserve | no |
| `cat.gadget` | `gadget.*` (07) | no | `cat.gadget` | 3 | no |
| `cat.part` | `part.*` (07) | no | `cat.part` | 5 | yes |
| `cat.food`, `cat.tool`, `cat.book`, `cat.container`, `cat.key`, `cat.currency`, `cat.quest` | `item.*` (11) | no | none | §14.3 | no |

### 3.2 Weapons

Doc 05 owns damage, timing, magazine, reach and skill mapping. This chapter owns base value, template weight, perk family (§7.1) and production class (§5).

| Archetype | Value | Weight | Family | Class | Archetype | Value | Weight | Family | Class |
|---|---|---|---|---|---|---|---|---|---|
| `arch.melee.dagger` | 40 | 900 | E | ML | `arch.ranged.bow` | 56 | 900 | D | RD |
| `arch.melee.blade_short` | 52 | 1000 | E | ML | `arch.ranged.sling` | 22 | 500 | D | RD |
| `arch.melee.blade_long` | 76 | 1000 | E | MM | `arch.ranged.hand_crossbow` | 60 | 700 | D | RD |
| `arch.melee.blade_great` | 86 | 700 | E | MH | `arch.ranged.crossbow` | 76 | 800 | D | RD |
| `arch.melee.hafted_axe` | 58 | 900 | E | MM | `arch.ranged.charge_rifle` | 110 | 600 | D | RD |
| `arch.melee.hafted_mace` | 58 | 900 | B | MM | `arch.ranged.sidearm` | 70 | 1000 | G | RG |
| `arch.melee.hafted_maul` | 86 | 600 | B | MH | `arch.ranged.smg` | 86 | 800 | G | RG |
| `arch.melee.hafted_flail` | 62 | 600 | B | MM | `arch.ranged.longarm` | 94 | 1000 | G | RG |
| `arch.melee.polearm_spear` | 58 | 900 | E | MM | `arch.ranged.scatter` | 88 | 800 | G | RG |
| `arch.melee.polearm_glaive` | 78 | 600 | E | MH | `arch.ranged.launcher` | 90 | 400 | H | RH |
| `arch.melee.polearm_staff` | 38 | 700 | B | MM | `arch.ranged.heavy_auto` | 128 | 400 | H | RH |
| `arch.melee.fist_wraps` | 56 | 600 | B | ML | `arch.ranged.flamer` | 98 | 400 | H | RH |
| | | | | | `arch.ranged.staff` | 100 | 700 | C | CA |
| | | | | | `arch.ranged.wand` (`cat.wand`) | 72 | 1000 | C | CA |

Families: E melee edged, B melee blunt, D draw and bolt, G gun, H heavy, C casting. Item tags used by affix requirements: `edged` (family E), `blunt` (family B), `pierce` (doc 05 pierce share ≥ 3000), `magazine` and `ammo` (doc 05 flags), `firearm` (families G and H plus `charge_rifle` skins with dependency techne), `casting` (family C and `arch.offhand.focus`).

### 3.3 Armor, off-hands, ward generators and mounts

| Template | Value | Weight | Class | Template | Value | Weight | Class |
|---|---|---|---|---|---|---|---|
| `arch.armor.light_head` / `_body` / `_hands` / `_legs` / `_feet` | 12 / 30 / 10 / 18 / 10 | 1000 / 700 / 1000 / 1000 / 1000 | AR | `arch.offhand.shield_small` | 36 | 1000 | SH |
| `arch.armor.medium_head` / `_body` / `_hands` / `_legs` / `_feet` | 16 / 36 / 12 / 22 / 12 | 1000 / 700 / 1000 / 1000 / 1000 | AR | `arch.offhand.shield_medium` | 52 | 1000 | SH |
| `arch.armor.heavy_head` / `_body` / `_hands` / `_legs` / `_feet` | 22 / 46 / 16 / 28 / 16 | 800 / 560 / 800 / 800 / 800 | AR | `arch.offhand.shield_tower` | 70 | 600 | SH |
| `arch.ward.light` | 60 | 1000 | WG | `arch.offhand.focus` | 44 | 800 | CA |
| `arch.ward.standard` | 80 | 1000 | WG | `arch.offhand.lantern` | 28 | 600 | SH |
| `arch.ward.heavy` | 100 | 600 | WG | mount token runner / hover / glider | 120 / 160 / 140 | 1000 / 600 / 600 | §9.11 |

Ward capacity follows Canon §13.2 (`mul_bp(cap_base_mhp, T(ilvl))`, fixed recharge). Ward skins (§4.3) give each generator its dependency.

### 3.4 Accessories

All accessories have `rarity_eligible = true`, roll affixes per Canon §13.3, have no manufacturer and value multipliers per §13.1. `per_enh` scales with `eff_enh`. Rings, amulets and trinkets are type-hidden (Canon §13.6); belts, back items and face items are not.

**Rings** (`slot.ring_l` or `slot.ring_r`; value 70; mass 20 g; weight 100 each):

| ID | Implicit at +0 | Per `eff_enh` |
|---|---|---|
| `item.ring.might`, `.finesse`, `.vigor`, `.intellect`, `.will`, `.perception`, `.presence`, `.fortune` | +2 to each of the primary's three sub-attributes | `+floor_div(eff_enh + 1, 2)` each (+10 at +15) |
| `item.ring.arcane_tongue`, `.machine_tongue`, `.sacred_tongue`, `.bold_tongue` | `gear_tol` +1 on arcana, techne, mythos, verve respectively | +1 more at `eff_enh >= 8` |
| `item.ring.warding` | +400 Bp resist to fire, frost, shock, acid, radiation and sonic | +50 Bp each |
| `item.ring.wealth` | Lucre generated for the wearer × (10000 + 1500) | +150 Bp |
| `item.ring.keen` | +200 Bp crit chance (Canon §6.4 row 13 gear) | +40 Bp |

**Amulets** (`slot.neck`; value 80; mass 40 g; weight 100 each):

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

**Trinkets** (`slot.trinket`; value 60; mass 50 g; weight 100; the last ten carry pool `loot.pool.relics_2`):

| ID | Effect |
|---|---|
| `item.trinket.lattice_lens` | carried anywhere in the inventory: `EXIT_OFFERS = 3` (Canon §17.1); +2 ITU when equipped |
| `item.trinket.lucky_coin` | +4 FAT, +4 PRV |
| `item.trinket.whetstone` | +600 Bp melee damage (additive bucket), +50 per `eff_enh` |
| `item.trinket.powder_horn` | +600 Bp ranged weapon damage (additive bucket), +50 per `eff_enh` |
| `item.trinket.reliquary_bone` | +1000 Bp resist umbral and radiant |
| `item.trinket.compass_rose` | +6 ITU |
| `item.trinket.hourglass` | on `ev.folio_entered`, `st.hasted` applied directly for 600 ticks |
| `item.trinket.thimble` | +8 armor poise |
| `item.trinket.wishbone` | Fortune Engine top-face guarantee arms after 3 non-top dispenses instead of 4 (§11.4) |
| `item.trinket.merchants_scale` | buy prices for the bearer × 9500, after Canon §6.4 row 37 |
| `item.trinket.keyring_charm` | on entering the first Folio of each Volume, receive 1 `item.key.bound` |
| `item.trinket.lodestone` | auto-pickup radius 3000 mm instead of 1500 mm (§14.4) |
| `item.trinket.warning_bell` | HUD shows hostiles at `aware.suspicious` or higher within 15000 mm (doc 09 states) |
| `item.trinket.saints_finger` | +6 RSV, +6 ATN |
| `item.trinket.clockwork_heart` | +6 END |
| `item.trinket.dragon_scale` | +800 Bp resist fire, +2 armor poise |
| `item.trinket.cat_eye` | +6 SGT |
| `item.trinket.forged_papers` | +6 GUI |

**Belts, back and face** (not type-hidden):

| ID | Slot | Value | Mass (g) | Effect |
|---|---|---|---|---|
| `item.belt.sash` | `slot.belt` | 30 | 200 | quickbar 5 |
| `item.belt.bandolier` | `slot.belt` | 45 | 400 | quickbar 6 |
| `item.belt.harness` | `slot.belt` | 60 | 600 | quickbar 7 |
| `item.belt.utility` | `slot.belt` | 80 | 800 | quickbar 8 |
| `item.back.cloak` | `slot.back` | 40 | 1200 | +300 Bp resist frost and fire |
| `item.back.satchel` | `slot.back` | 40 | 800 | +6 inventory slots |
| `item.back.rucksack` | `slot.back` | 70 | 1600 | +10 inventory slots |
| `item.back.expedition_pack` | `slot.back` | 110 | 2500 | +14 inventory slots |
| `item.face.respirator` | `slot.face` | 50 | 400 | gases breathable (Canon §12) |
| `item.face.mask` | `slot.face` | 30 | 200 | disguise item (doc 09) |
| `item.face.goggles` | `slot.face` | 40 | 300 | +4 SGT |

### 3.5 Consumables, keys, bags and currency

Consumables are never rarity-eligible. `value` is per unit. Effects are owned by the doc in brackets.

| Group | IDs and unit values | Mass (g) | Stack |
|---|---|---|---|
| Potions (08), type-hidden | `item.potion.healing` 24, `.aether` 22, `.haste` 30, `.focus` 24, `.fortify` 24, `.antidote` 16, `.invisibility` 30, `.clarity` 24, `.levitation` 24, `.regeneration` 28, `.berserk` 20, `.shielding` 28 | 300 | 10 |
| Texts (06, 10), type-hidden | `item.text.mending` 20, `.blink` 26, `.mapping` 20, `.demolition` 24, `.appraisal` 22, `.recharge` 26, `.sanctify` 30, `.warding` 26, `.silence` 24, `.summoning` 30 | 50 | 10 |
| Throwables (05, 08) | `item.throwable.javelin` 6, `.knife` 5, `.smoke` 14, `.frag` 10, `.fire_flask` 12, `.acid_flask` 12, `.poison_flask` 12, `.breaker_charge` 14, `.disruptor_flask` 14, `.concussion_bolas` 14; pool `loot.pool.throwables_2`: `.flashbang` 14, `.frost_flask` 12, `.net` 12, `.caltrops` 8, `.holy_water` 14, `.sticky_bomb` 16 | javelin 1500, knife 250, others 400 | javelin 6, knife 20, others 10 |
| Food (04) | `item.food.ration` 4 (satiation 600), `.meal` 8 (900), `.snack` 2 (300) | 300 / 500 / 100 | 20 |
| Tools (07, 09) | `item.tool.bandage` 6, `.torch` 2, `.pick` 12, `.lockpicks` 40, `.toolkit` 24, `.rope` 6, `.crutch` 10, `.rewrite_quill` 60 (§15), `.emendation` 300 (§16; never sold or bought) | bandage 50, torch 400, pick 2500, others 300 | bandage 10, torch 10, quill 5, emendation 15, others 1 |
| Books (06) | `item.book.primer_evocation`, `.primer_abjuration`, `.primer_divination` 48 | 800 | 1 |
| Containers (08, 11) | `item.container.flask` 2; bags §9.7 | 150 | 10 |
| Keys (11) | `item.key.bound` 30, `item.key.gilded` 90, `item.key.illumined` 250 | 0 | keyring |
| Currency (11) | `item.lucre`: a pile whose amount is its `value` (`i64`) | 0 | auto-pickup |
| Quest (12, 14) | `item.quest.catchword` 0 | 100 | 1 |
| Ammo (05) | `ammo.light` 20, `ammo.heavy` 40, `ammo.explosive` 200, `ammo.fuel` 10, `ammo.stone` 5 per 100 rounds | doc 05 | reserve |

Unit values scale with `P(ilvl)` like every value (§13.1). Ammo stack value is `floor_div(rounds × value_per_100, 100)`.

**Champion counter throwables** (Canon §17.3a). All three have dependency none in every genre, so they are legal in all 16 genres including `genre.unwritten`. Each impact is an item source with `src_lvl = ilvl` and base 6 HP (`dmg.blunt`) that carries its counter type; for the next 600 ticks every hit any Erratum lands on that target also carries it (the target shows the counter's icon):

| ID | Counter carried by its impact hit | Skin examples (MF / CP / WX) |
|---|---|---|
| `item.throwable.breaker_charge` | `counter.ward_breaker` | Shatterstone / Breach Charge / Thunder Pellet |
| `item.throwable.disruptor_flask` | `counter.disruptor` | Nullroot Flask / EMP Ampoule / Qi-Sealing Powder |
| `item.throwable.concussion_bolas` | `counter.staggerer` | Weighted Bolas / Concussion Puck / Meteor Bolas |

### 3.6 Gadgets and loose parts

Doc 07 owns gadget and part mechanics. This chapter fixes their loot-facing stats (Interface assumptions 3).

| Gadget | Value | Charges | Loot stats |
|---|---|---|---|
| `gadget.turret_light` | 44 | 1 | deploy 60 ticks; lasts 1800 ticks; shot base 4 HP × `P(src_lvl)` |
| `gadget.drone_scout` | 40 | 1 | lasts 3600 ticks; marks hostiles with `st.marked` within 12000 mm |
| `gadget.mine_frag` | 18 | 1 | base 30 HP × `P(src_lvl)` `dmg.pierce` in 3000 mm |
| `gadget.trap_snare` | 12 | 1 | `st.rooted` applied directly |
| `gadget.beacon_mend` | 36 | 1 | heals 100 Bp of Max HP per second within 4000 mm for 600 ticks |
| `gadget.decoy` | 20 | 1 | taunts hostiles within 8000 mm for 300 ticks |

Six more gadgets enter with `loot.pool.gadgets_2` (doc 07 names them). Loose parts: §5.4.

---

## 4. Genre skins

### 4.1 Rules

- A skin is the pair (archetype, genre). It supplies the display name `loc.<archetype>.skin.<genre_suffix>.name` (Canon §1.10), the mesh and sound sets (doc 18) and the base component's dependency set (Canon §5.3).
- Notation: no tag = dependency none; `[A]` arcana, `[T]` techne, `[M]` mythos, `[V]` verve. `—` = no skin: the archetype is never generated in that genre and cannot be Rewritten into it.
- Validation (build time): no cell may depend on an axis whose axiom is 0 in that genre (Canon §5.3), so `genre.post_apocalypse` and `genre.hardboiled_noir` have no `[A]`, `genre.redacted` has no `[V]`, and every `genre.unwritten` cell is untagged or `—`. Staves and wands always depend on the genre's `aether_axiom`, or arcana when it is none.
- `Req[a] = max(1, origin_axiom[a] - 2)` for each tagged axis (Canon §5.3). Example: a Space Opera Photon Sabre `[T]` has `Req[techne] = 8`.

### 4.2 Weapon skins (26 archetypes × 16 genres)

**Table A: short and long blades, axes and maces**

| Genre | dagger | blade_short | blade_long | blade_great | hafted_axe | hafted_mace | hafted_maul |
|---|---|---|---|---|---|---|---|
| mythic_fantasy | Dirk | Arming Sword | Longsword | Greatsword | Bearded Axe | Flanged Mace | Warhammer |
| gothic_horror | Silvered Stiletto [M] | Hunting Hanger | Duelling Sword | Executioner's Sword | Woodsman's Axe | Censer Mace [M] | Grave Maul |
| cosmic_horror | Ritual Knife [M] | Cutlass | Expedition Machete | Star-Iron Blade [M] | Fire Axe | Crowbar | Sledgehammer |
| space_opera | Vibroknife [T] | Plasma Cutter [T] | Photon Sabre [T] | Ion Greatblade [T] | Shear-Axe [T] | Gravity Baton [T] | Grav-Hammer [T] |
| cyberpunk | Mono-Shiv [T] | Thermal Wakizashi [T] | Monomolecular Katana [T] | Vibro-Nodachi [T] | Hydraulic Hatchet [T] | Stun Baton [T] | Pneumatic Sledge [T] |
| post_apocalypse | Scrap Shiv | Saw-Machete | Sharpened Rebar Machete | Car-Spring Cleaver | Stop-Sign Axe | Pipe Wrench | Engine-Block Maul |
| weird_west | Bowie Knife | Arkansas Toothpick | Hexed Saber [M] | Sodbuster Scythe-Blade | Tomahawk | Silver-Capped Cudgel [M] | Railroad Sledge |
| hardboiled_noir | Switchblade | Machete | Sword Cane | Antique Claymore | Hatchet | Blackjack | Sledge |
| wuxia | Bishou | Dao | Jian | Zhanmadao | Fu Axe | Bian Rod | Melon Hammer |
| steampunk | Clockwork Stiletto [T] | Naval Cutlass | Cavalry Sabre | Steam Claymore [T] | Boiler Axe [T] | Piston Mace [T] | Steam Hammer [T] |
| fairy_tale | Silver Bodkin | Huntsman's Knife | Knight's Sword | Giant-Slayer Blade [A] | Woodcutter's Axe | Troll Club | Ogre Maul |
| mythic_antiquity | Parazonium | Xiphos | Kopis | Rhomphaia | Labrys [M] | Olive-Wood Club | Bronze Maul |
| oneiric | Thought-Splinter [A] | Recurring Blade [A] | Sleepwalker's Sword | Nightmare Edge [M] | Clock-Hand Axe | Bell of Waking [A] | Dream Anvil [A] |
| redacted | Letter Opener | Censor's Blade [T] | Black-Bar Sword [T] | Redaction Cleaver [M] | Shredder Axe [T] | Rubber Stamp [M] | Gavel [M] |
| unwritten | Pencil Knife | Margin Blade | Draft Sword | Unfinished Greatsword | Sketched Axe | Outline Mace | Blank Maul |
| ur_text | Ur-Dirk | Ur-Gladius | Ur-Sword | Ur-Greatsword | Ur-Axe | Ur-Mace | Ur-Maul |

**Table B: polearms, fists, flails, bows and slings**

| Genre | polearm_spear | polearm_glaive | polearm_staff | fist_wraps | hafted_flail | bow | sling |
|---|---|---|---|---|---|---|---|
| mythic_fantasy | Boar Spear | Halberd | Quarterstaff | Cestus | Flail | Shortbow | Sling |
| gothic_horror | Stake-Spear [M] | Bardiche | Pilgrim's Staff [M] | Knuckleduster | Scourge [M] | Hunting Bow | Shot-Pouch Sling |
| cosmic_horror | Harpoon | Boathook Glaive | Walking Stick | Brass Knuckles | Anchor Chain | Expedition Bow | Slingshot |
| space_opera | Force Pike [T] | Plasma Glaive [T] | Shock Stave [T] | Power Gauntlet [T] | Mag-Chain [T] | Photon Bow [T] | Grav-Sling [T] |
| cyberpunk | Shock Lance [T] | Vibro-Naginata [T] | Telescoping Bo [T] | Mantis Claws [T] | Monowire Whip [T] | Smart-Bow [T] | Rail-Slingshot [T] |
| post_apocalypse | Pipe Spear | Signpost Glaive | Rebar Staff | Nail Knuckles | Bike-Chain Flail | Rebar Bow | Bolt Sling |
| weird_west | Buffalo Lance | Harvest Scythe | Preacher's Staff [M] | Brass Knuckles | Bullwhip | Plains Bow | Rock Sling |
| hardboiled_noir | Gaff Hook | Fire Hook | Pool Cue | Knuckle-Dusters | Bicycle Chain | Club Bow | Slingshot |
| wuxia | Qiang | Guandao | Monk's Staff | Tiger Claws | Meteor Hammer | Horn Bow | Rope Dart |
| steampunk | Brass Lance | Gear Halberd [T] | Tesla Cane [T] | Clockwork Gauntlet [T] | Chain Flail | Spring-Bow [T] | Pneumatic Sling [T] |
| fairy_tale | Huntsman's Spear | Poleaxe | Shepherd's Crook | Iron Gloves | Threshing Flail | Huntsman's Bow | Shepherd's Sling |
| mythic_antiquity | Dory | Sarissa | Herald's Caduceus [M] | Caestus | Scourge of the Furies [M] | Scythian Bow | Balearic Sling |
| oneiric | Moonbeam Lance [A] | Pendulum Glaive | Stair-Rail Staff | Sleep-Paws | Ribbon Flail [A] | Dreamcatcher Bow [A] | Marble Sling |
| redacted | Filing Spike | Turnstile Glaive [T] | Baton of Office [T] | Knuckle Seals | Red-Tape Whip [M] | Standard-Issue Bow | Stapler Sling [T] |
| unwritten | Draft Spear | Line Glaive | Margin Staff | Bare Gauntlet | Trailing Line | Blank Bow | Blank Sling |
| ur_text | Ur-Spear | Ur-Glaive | Ur-Quarterstaff | Ur-Fist | Ur-Flail | Ur-Bow | Ur-Sling |

**Table C: crossbows and guns**

| Genre | hand_crossbow | crossbow | sidearm | smg | longarm | scatter |
|---|---|---|---|---|---|---|
| mythic_fantasy | Hand Crossbow | Arbalest | Dwarven Handgonne [T] | Repeating Crossbow | Dwarven Long-Gonne [T] | Blunderbuss [T] |
| gothic_horror | Hunter's Hand Crossbow | Stake Crossbow [M] | Flintlock Pistol [T] | Pepperbox [T] | Hunting Musket [T] | Dragoon Blunderbuss [T] |
| cosmic_horror | Dart Pistol | Harpoon Crossbow | Service Pistol [T] | Drum-Fed Submachine Gun [T] | Bolt-Action Rifle [T] | Coach Gun [T] |
| space_opera | Needler [T] | Rail-Bolt Caster [T] | Blaster Pistol [T] | Pulse Repeater [T] | Blaster Rifle [T] | Scatter Blaster [T] |
| cyberpunk | Dart Gun [T] | Mag-Bolt Crossbow [T] | Smart Pistol [T] | Machine Pistol [T] | Tech Rifle [T] | Smart Shotgun [T] |
| post_apocalypse | Rebar Hand Crossbow | Junk Crossbow | Pipe Pistol [T] | Pipe Burst-Gun [T] | Hunting Rifle [T] | Sawn-Off [T] |
| weird_west | Derringer Crossbow | Bounty Crossbow | Six-Shooter [T] | Gatling Pistol [T] | Lever-Action Rifle [T] | Scattergun [T] |
| hardboiled_noir | Spring Dart Pistol | Speargun | Snub-Nose Revolver [T] | Drum Submachine Gun [T] | Scoped Hunting Rifle [T] | Pump Shotgun [T] |
| wuxia | Sleeve Crossbow | Heavy Nu Crossbow | Fire Lance [T] | Zhuge Repeater | Composite Recurve | Arrow-Swarm Box [T] |
| steampunk | Spring Dart Pistol [T] | Pneumatic Arbalest [T] | Pepperbox Pistol [T] | Clockwork Repeater [T] | Brass Rifle [T] | Brass Blunderbuss [T] |
| fairy_tale | Pixie Crossbow | Dwarf-Made Crossbow | Thunder Pistol [A] | Pixie Shooter [A] | Fletcher's Bow | Seed Spitter [A] |
| mythic_antiquity | Toxon Dart-Thrower | Gastraphetes | Dart Caster | Volley Bow [V] | Sun-Gilded Bow [M] | Shard Sling |
| oneiric | Paper-Dart Launcher | Clock Crossbow | Finger-Gun [V] | Bubble Repeater [A] | Telescope Rifle [T] | Confetti Gun [A] |
| redacted | Tranquilizer Pistol [T] | Bolt Stapler [T] | Service Sidearm [T] | Riot SMG [T] | Marksman Rifle [T] | Riot Shotgun [T] |
| unwritten | Draft Hand-Bow | Draft Crossbow | Outline Pistol | Outline Repeater | Outline Rifle | Outline Scattergun |
| ur_text | Ur-Hand-Bow | Ur-Crossbow | Ur-Pistol | Ur-Repeater | Ur-Rifle | Ur-Scatter |

**Table D: charge weapons, heavy weapons and casting implements**

| Genre | charge_rifle | launcher | heavy_auto | flamer | staff | wand |
|---|---|---|---|---|---|---|
| mythic_fantasy | Longbow | Hand Ballista | Repeating Ballista | Dragon-Breath Horn [A] | Wizard's Staff [A] | Rune Wand [A] |
| gothic_horror | Yew Longbow | Grenadier's Mortar [T] | Volley Gun [T] | Holy-Water Sprayer [M] | Crozier [M] | Bone Wand [M] |
| cosmic_horror | Elephant Gun [T] | Flare Mortar [T] | Pan-Fed Machine Gun [T] | Flamethrower [T] | Eldritch Rod [M] | Fetish Wand [M] |
| space_opera | Railgun [T] | Missile Launcher [T] | Rotary Blaster [T] | Plasma Projector [T] | Psi Staff [A] | Psi Rod [A] |
| cyberpunk | Coil Sniper [T] | Micro-Missile Pod [T] | Minigun [T] | Thermite Projector [T] | Neural Projector [T] | Cyberdeck Wand [T] |
| post_apocalypse | Junk Railgun [T] | Pipe Mortar [T] | Scrap Chaingun [T] | Fuel-Can Flamer [T] | Static Rod [T] | Tesla Wand [T] |
| weird_west | Buffalo Rifle [T] | Dynamite Launcher [T] | Gatling Gun [T] | Hellfire Bellows [M] | Hex Rod [M] | Bone Fetish [M] |
| hardboiled_noir | Sniper Rifle [T] | Riot Grenade Launcher [T] | Belt-Fed Machine Gun [T] | Blowtorch [T] | Lucky Cane [V] | Loaded Dice [V] |
| wuxia | Great Bow | Thunder-Crash Tube [T] | Ten-Bolt Repeater | Dragon-Fire Lance [T] | Qi Staff [V] | Talisman Brush [V] |
| steampunk | Galvanic Rifle [T] | Rocket Musket [T] | Steam Gatling [T] | Boiler Flamer [T] | Pressure Staff [T] | Galvanic Rod [T] |
| fairy_tale | Elfin Longbow | Gourd-Bomb Sling [A] | Thorn-Volley Briar [A] | Frost-Breath Horn [A] | Witch's Staff [A] | Fairy Wand [A] |
| mythic_antiquity | Hero's Greatbow | Fire-Pot Thrower | Polybolos [T] | Greek Fire Siphon [T] | Oracle's Staff [M] | Thyrsus [M] |
| oneiric | Moonlight Bow [A] | Dream-Bubble Mortar [A] | Music-Box Gatling [A] | Lullaby Horn [A] | Lucid Staff [A] | Nightlight Wand [A] |
| redacted | Compliance Rifle [T] | Containment Launcher [T] | Suppression Gun [T] | Correction-Fluid Sprayer [T] | Clearance Rod [T] | Badge Reader [T] |
| unwritten | Outline Longbow | Blank Launcher | Unfinished Gatling | Smudge Sprayer | — | — |
| ur_text | Ur-Longbow | Ur-Launcher | Ur-Gatling | Ur-Sprayer | Ur-Rod [A] | Ur-Wand [A] |

### 4.3 Ward generator and mount skins

| Genre | Ward generator (all three `arch.ward.*`) | runner | hover | glider |
|---|---|---|---|---|
| mythic_fantasy | Aegis Charm [A] | Warhorse | Flying Carpet [A] | Griffon |
| gothic_horror | Reliquary Locket [M] | Black Stallion | Spectral Carriage [M] | Bat-Wing Cloak [M] |
| cosmic_horror | Elder Sign [M] | Motorcycle Combination [T] | Shoggoth Skiff [M] | Nightgaunt |
| space_opera | Deflector Belt [T] | Sand-Strider | Speeder [T] | Jet-Wing [T] |
| cyberpunk | Kinetic Barrier [T] | Superbike [T] | Hoverbike [T] | Wingsuit Drone [T] |
| post_apocalypse | Scrap Bubble [T] | Dirt Bike [T] | Airboat [T] | Hang-Glider |
| weird_west | Medicine Bag [M] | Mustang | Spirit Canoe [M] | Thunderbird [M] |
| hardboiled_noir | Lucky Rabbit's Foot [V] | Roadster [T] | Getaway Car [T] | Wingsuit |
| wuxia | Golden Bell Shield [V] | Celestial Horse | Cloud Chariot [V] | Qinggong Kite [V] |
| steampunk | Faraday Harness [T] | Velocipede [T] | Steam Hovercraft [T] | Ornithopter [T] |
| fairy_tale | Godmother's Ribbon [A] | White Stag | Pumpkin Coach [A] | Giant Swan |
| mythic_antiquity | Gorgon Ward [M] | Thessalian Horse | Sun Chariot [M] | Pegasus |
| oneiric | Pillow Fort [A] | Nightmare [A] | Paper Boat [A] | Umbrella [A] |
| redacted | Clearance Badge [T] | Patrol Scooter [T] | Transport Drone [T] | Glide Pack [T] |
| unwritten | Padded Margin | Sketch Horse | Outline Raft | Paper Plane |
| ur_text | Ur-Ward | Ur-Steed | Ur-Chariot | Ur-Wing |

Beast mounts are untagged (Canon §5.3). A mount token with a vehicle skin carries a manufacturer of its genre that makes class `RH`.

---

## 5. Manufacturers

### 5.1 Rules

- A manufacturer stamps weapons, off-hands, armor, ward generators, vehicle mount tokens and loose parts. Accessories, implants and consumables have none.
- **Tendencies** multiply doc 05 base stats once at generation: `stat = mul_bp(stat, 10000 + delta)`. Legend: D damage `B`, S attack speed, P spread, M magazine, L reload ticks, R range, H handling ticks, T stamina cost, O poise damage, U durability, W mass, A AR, X precision multiplier, C aether cost, V value. A tendency on a stat the item lacks is ignored.
- **Gimmick.** The signature gimmick is one component (Canon §5.3) of the item, with the dependency listed, the item's `origin_genre` and its own Contradiction potency. "+N Bp" gimmick numbers enter the additive bucket (Canon §10.5 step 1). A gimmick stated as "× N" multiplies the hit's `B` once after the Canon §10.4 chain and before step 1 (Canon gaps 3).
- **Classes.** ML light melee, MM medium melee, MH heavy melee, RD draw and bolt, RG guns, RH heavy ranged and vehicles, CA casting (staff, wand, focus, lantern), AR armor, SH shields, WG ward generators (§3.2, §3.3).
- The first two rows of each core genre are the canon-1.3 §5.9 manufacturers with their canon traits; the third row and `mfr.errata.marginal_press` are added by this chapter.

### 5.2 Manufacturer table

| ID | Name | Makes | Tendencies | Parts (§5.4) | Signature gimmick | Dep |
|---|---|---|---|---|---|---|
| `mfr.mythic_fantasy.deepanvil` | Deepanvil Forge | MM MH RG RH AR SH | D+800 U+3000 W+1500 | eh hl gg bh pr | weapons: poise damage × 15000 and attack speed × 9000; armor and shields: +4 poise per piece | none |
| `mfr.mythic_fantasy.silverbough` | Silverbough Fletchery | ML RD AR WG | S+500 W−2000 D−300 | ek hg bl os | attack noise −20 dB; armor: footstep noise −6 dB | none |
| `mfr.mythic_fantasy.runecourt` | The Runecourt | CA WG ML MM | C−800 U−1000 | fc fn ek cc | Spellsink: every kill made with or cast through the item restores 3 aether; ward generators: ward break restores 10 aether | arcana |
| `mfr.gothic_horror.vesper_sons` | Vesper & Sons | ML MM RD CA | D+300 U+1000 | ek es br fc | 2000 Bp of each hit moves from its largest physical share to `dmg.radiant` | mythos |
| `mfr.gothic_horror.grimhallow` | Grimhallow Gunsmiths | RG RH ML | D+600 M−2000 | bs fq oi gw | a reload completes in 1 tick and removes `mul_bp(max_hp_mhp, 300)` mHP directly (not damage; refused if HP would fall below 1,000 mHP); melee: heavy attacks cost that HP instead of stamina | mythos |
| `mfr.gothic_horror.blackmoor` | Blackmoor Ironworks | MH MM AR SH WG | A+1000 W+2000 S−300 | eh gg pr lp cc | Grave Iron: while HP ≤ 3000 Bp of Max HP, Blackmoor armor and shield AR × 13000 and Blackmoor weapons +2000 Bp | none |
| `mfr.cosmic_horror.hollow_lantern` | Hollow Lantern Optics | RG RD CA ML | R+1500 D−300 | os br fc ek | hits apply `st.marked` directly for 240 ticks | mythos |
| `mfr.cosmic_horror.outer_choir` | The Outer Choir | CA RH MM | D+500 U−2000 | fn bh eh fd | 2500 Bp of each hit moves from its largest share to `dmg.psychic`; +1500 Bp; each attack costs 1 composure, and at 0 composure the item cannot attack | mythos |
| `mfr.cosmic_horror.dunwold` | Dunwold Expedition Supply | RG MM MH AR SH WG | U+5000 S−300 | br fe hl pr lp | Field-Tested: durability loss × 5000; weapons reload while sprinting; armor: +300 Bp resist acid and toxin | none |
| `mfr.space_opera.perihelion` | Perihelion Arms | RG RD RH | D+500 S−800 | bl os fe | holding fire 30 ticks charges the next shot: × 15000 | techne |
| `mfr.space_opera.umbra` | Umbra Heavy Industries | RG RH WG | S+800 D−300 | bh or cc | no magazine and no ammo; each shot adds `floor_div(10000, base_magazine)` heat; heat decays 2500 per second from 30 ticks after the last shot; at 10000 the weapon locks for 90 ticks, then heat is 0; ward generators: capacity × 11000 | techne |
| `mfr.space_opera.starwright` | Starwright Psionics | CA ML MM MH AR SH | D+200 W−1500 | fc ek hg pr | Kinetic Echo: every 5th hit repeats 20 ticks later as a `dmg.psychic` hit entering Canon §10.6 at step 3 with `D2` = 5000 Bp of the original `D2`; armor: every 5th melee hit taken pushes the attacker 1500 mm | arcana |
| `mfr.cyberpunk.ghostline` | Ghostline Systems | RG RD ML | P−1500 D−500 | or br fq ek | projectiles home on `st.marked` targets at 273 Bam per tick (16384 Bam/s); melee: +1500 Bp vs `st.marked` | techne |
| `mfr.cyberpunk.neon_saint` | Neon Saint Customs | RG RD RH WG | D+700 L+1500 | bl os fd cc | charged shots pierce 1000 mm of blocks; a shot is charged at full draw or charge (families D), else when it is the first shot after aiming 45 ticks | techne |
| `mfr.cyberpunk.chromaw` | Chromaw Biotech | ML MM MH AR SH CA | S+600 U−2000 | es hg gw lp fn | Neurotoxin Coat: +40 `st.poisoned` power per hit; armor: +500 Bp resist toxin | techne |
| `mfr.post_apocalypse.rustmaw` | Rustmaw Scrapworks | RG RH MH | P+1500 | fd bh eh | magazine × 20000, durability × 5000 | none |
| `mfr.post_apocalypse.lastditch` | Lastditch Outfitters | RG RD ML MM | D−200 U+2000 | bs fq gw hl | 2000 Bp chance per shot to spend no ammo, or per swing to spend no stamina (keyed `rng.combat`, 1 `roll_bp` per shot or swing) | none |
| `mfr.post_apocalypse.geigerhaus` | Geiger & Daughters | RH CA AR SH WG MM | D+400 W+1500 | bh fn pr cc | Hot Load: 1500 Bp of each hit moves to `dmg.radiation` and +25 `st.irradiated` power; armor: +600 Bp resist radiation | techne |
| `mfr.weird_west.coffin_creek` | Coffin Creek Revolver Co. | RG RD | D+500 M−2000 L+1000 | br oi fq | crits ricochet 5000 Bp of their `D2` to the nearest other hostile within 8000 mm (Canon §10.6 step 3, no crit, no further ricochet) | verve |
| `mfr.weird_west.seven_seals` | Seven Seals Armory | RG MM CA ML | D+300 P+500 | bs es fc gw | 2000 Bp of each hit moves from its largest share to `dmg.umbral` | mythos |
| `mfr.weird_west.sundown_rail` | Sundown Rail & Powder | RH RG MH AR SH WG | D+800 S−700 | bh fe eh pr | Sticky Fuse: projectile hits leave a charge (max 6 per target); starting a reload detonates every charge for 3000 Bp of its hit's `D2` as `dmg.fire` within 2000 mm (Canon §10.6 environmental entry, no crit); melee: heavy attacks leave the charge and the next heavy attack detonates it; armor: +500 Bp resist fire | techne |
| `mfr.hardboiled_noir.midnight_avenue` | Midnight Avenue Arms | RG ML RD | H−3000 R−1500 | bs oi gw ek | shot noise capped at 50 dB; melee swing noise capped at 40 dB | none |
| `mfr.hardboiled_noir.rainslick` | Rainslick & Co. | MM MH RG | O+2000 D+300 | eh gg bh | × 12000 against `st.staggered` or `st.knocked_down` targets | none |
| `mfr.hardboiled_noir.halloran` | Halloran Typewriter & Arms | RG RH AR SH WG CA | S+1000 P+1500 D−500 | fd fe or lp | Typewriter: attack speed +500 Bp per 10 consecutive shots, max +2000, reset after 30 ticks without firing; armor: +2 poise | none |
| `mfr.wuxia.jade_lotus` | Jade Lotus Pavilion | ML MM | S+800 D−400 W−2000 | ek hg gw | the weapon gains the doc 05 throw action (normal hit damage) and returns to the hand 30 ticks after impact or after 30 ticks of flight | verve |
| `mfr.wuxia.iron_crane` | Iron Crane School | MM MH SH AR | T−1500 D+200 | gg hl eh pr | a block started within 6 ticks before impact reflects 5000 Bp of the hit's `D2` (Canon §10.6 reflected) | verve |
| `mfr.wuxia.cloudpiercer` | Cloudpiercer Hall | RD RG RH CA WG | X+1000 R+1500 D−300 | bl os fc cc | Acupoint: precision-part hits add 40 `st.stunned` power | none |
| `mfr.steampunk.brassworth` | Brassworth & Cog | RG MM ML | S+300 U+1500 | br fq ek gw | +200 Bp per consecutive hit, max +3000; a hit is consecutive within 120 ticks of the previous hit; a miss or 120 ticks resets | techne |
| `mfr.steampunk.aetherline` | Aetherline Galvanic | RG RH CA WG | D−300 M+2000 | bl fe fn cc | every 4th hit chains 3000 Bp of its `D2` as `dmg.shock` to the 2 nearest other hostiles within 5000 mm (Canon §10.6 step 3, `no_chain`); ward generators: ward break emits the chain from the bearer for 3000 Bp of capacity | techne |
| `mfr.steampunk.ironclad` | Ironclad Boilerworks | MH RH AR SH RD | D+900 S−800 W+2500 | bh eh pr fd | Pressure Valve: 3 steam charges, +1 per 120 ticks; a heavy attack or alt-fire spends 1 for +5000 Bp and places one `gas.steam` cell at impact; armor: +2 poise | techne |
| `mfr.fairy_tale.thimbleweald` | Thimbleweald Tailors | ML MM RD MH | W−2500 D−400 | ek hg bs | × 13000 against targets with a larger capsule radius than the wielder | none |
| `mfr.fairy_tale.thornwood` | Thornwood Coven | MM RD CA WG | U+2000 S−200 | hl bl fc cc | 1000 Bp chance per hit to apply `st.rooted` (keyed `rng.status`); ward generators: ward break roots non-boss hostiles within 3000 mm | arcana |
| `mfr.fairy_tale.glass_cobbler` | The Glass Cobbler | ML AR SH RG RH | V+2000 | ek es br pr | Glass Edge: +2500 Bp; max durability × 5000; at durability 0 the item shatters into a 2000 mm `dmg.slash` burst for 5000 Bp of the last hit's `D2` and drops its §16.4 salvage; armor: AR × 12000 | arcana |
| `mfr.mythic_antiquity.hephaesteum` | Hephaesteum | MH MM AR SH RH | D+300 W+1000 | eh hl pr bh | no durability loss; the bearer is immune to `st.disarmed` | mythos |
| `mfr.mythic_antiquity.argive` | Argive Arms | MM SH RD AR ML | D+200 T−1000 | hl gg bl pr | +1500 Bp while another allied Erratum's origin is within 3000 mm; armor and shields: +1000 Bp resist slash, pierce and blunt under the same condition | none |
| `mfr.mythic_antiquity.delphic` | Delphic Workshops | CA RD WG RG | X+1500 S−300 | fc os br cc | Foresight: the first hit on each target in a Folio uses crit chance 10000 (the crit draw is still made); ward generators: the first hit taken each Folio is fully absorbed | mythos |
| `mfr.oneiric.somnambule` | Somnambule | RD RG RH CA AR SH WG | R+2000 D−300 | bl fc or | projectiles, spell projectiles included, pass through blocks after 10000 mm of flight | arcana |
| `mfr.redacted.bureau` | The Bureau | all classes | P−1000 D−300 | br fe os ek gg | the base component never rolls Contradiction failure; potency loss still applies | techne |
| `mfr.ur_text.first_press` | The First Press | Palimpsestic only | none | none | none beyond Canon §13.3 Palimpsestic rules | none |
| `mfr.errata.marginal_press` | The Marginal Press | all classes | none | all | Stubborn Margin: while the base component has `C` in `1..=5`, +800 × `C` Bp | none |

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
| Exotic | the Exotic template's fixed manufacturer (no draw) |
| Beast-skinned mount token | none (no draw) |

Example: a Weird West sidearm (class RG) has candidates Marginal Press 1000, Coffin Creek 3000, Seven Seals 3000, Sundown Rail 3000, total 10000 (§20).

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

A loose part instance has `category = cat.part`, a `manufacturer`, `rarity = rar.common`, value `mul_bp(25, P(ilvl))`, mass 300 g and stack 5. Loose-part template draw: uniform over the drawn manufacturer's part list (1 `weighted` with equal weights 100 in `loot.arch`); a part drawn from the deck card `cat.part` first draws its manufacturer with equal weights over the origin genre's §5.3 candidates (1 `weighted` in `loot.mfr`).

---

## 6. Affixes

### 6.1 Eligibility, tiers and values

- **Count** per rarity: Canon §13.3 (0, 1, 2, 3, 3, 2, 4). Exotic and Palimpsestic affixes roll like any other.
- **Eligible** for an item when all hold: the item's category letter is in Cats (W melee, R ranged weapon including staff, N wand, A armor, O off-hand, C accessory, G ward generator, M mount token); no affix of the same Group is already on the item; every Requires tag is on the item (§3.2 tags; `armored` = armor or shield; `shield` = `arch.offhand.shield_*`; `legs_feet` = legs or feet armor; `pool:<x>` = the run content set holds `loot.pool.<x>`; `min Tn` = ilvl ≥ the gate of tier n); and the dependency class is legal in `origin_genre`.
- **Dependency class** → component dependency (Canon §5.3): mundane → none; infused → `{origin.aether_axiom}`, or arcana when none; tech → techne; sacred → mythos; cine → verve; base → the item's base-function dependency; gloss → the gimmick's own dependency, with requirement computed from the gloss manufacturer's genre. A class whose axis has axiom 0 in `origin_genre` is illegal there (Canon §5.3).
- **Tier gates** by `ilvl`: T1 1, T2 4, T3 9, T4 15, T5 21. Marginalia items reach T5 like ilvl-21 items. **Tier weights** over unlocked tiers: `[10000, 6000, 3500, 1800, 800]`; for Epic or better, the T1 weight is `mul_bp(10000, 5000)`.
- **Draw order** inside `loot.affix`, per affix: `weighted` over eligible affixes sorted by ID (weights = Wt), `weighted` over tiers, `range(10001)` for `roll_bp`.
- **Value**: `X = lo + floor_div((hi - lo) × roll_bp, 10000)`. Each affix contributes `mul_bp(X, its potency)` where a Bp contribution is scaled (Canon §5.5); flat counts (sub-attributes, dB, mm, ticks, charges, tolerance) are 0 when the affix is Disconnected and unscaled otherwise.
- **Display**: affix values are hidden until the affix's knowledge flag is set (doc 10).
- **Curse affixes** (`curse.*`) belong to doc 10 and are outside the count.

Worked examples: `affix.dmg_vs_beast` tier 2, `roll_bp = 1146`: `X = 1800 + floor_div(700 × 1146, 10000) = 1880` Bp. The descending `affix.self_repair` tier 2, `roll_bp = 4095`: `X = 900 + floor_div(-200 × 4095, 10000) = 900 - 82 = 818` ticks.

### 6.2 Affix table (116 affixes)

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
| `affix.infuse_<t>` ×11: fire, frost, shock, acid, radiation, sonic, toxin, radiant, umbral, psychic, arcane | infusion | WR | fire, frost, shock, psychic, arcane infused; acid, toxin mundane; radiation, sonic tech; radiant, umbral sacred | moves X Bp of the hit from its largest physical share to `dmg.<t>`; +`floor_div(X, 70)` power of `<t>`'s status | 1000–1500 | 1500–2000 | 2000–2600 | 2600–3300 | 3300–4000 | 25 | — |
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
| `affix.durability` | durability | WRAOG | mundane | max durability ×(10000 + X) | 2000–3000 | 3000–4500 | 4500–6000 | 6000–8000 | 8000–10000 | 60 | — |
| `affix.featherweight` | mass | WRAOCG | mundane | item mass ×(10000 − X) | 1000–1500 | 1500–2200 | 2200–2800 | 2800–3500 | 3500–4500 | 50 | — |
| `affix.self_repair` | repair | WRAOG | tech | restore 1 durability every X ticks | 1200–900 | 900–700 | 700–500 | 500–380 | 380–300 | 20 | — |
| `affix.ward_capacity` | ward_capacity | G | base | ward capacity ×(10000 + X) | 1000–1500 | 1500–2200 | 2200–3000 | 3000–4000 | 4000–5500 | 120 | — |
| `affix.ward_siphon` | ward_siphon | G | base | on kill restore X Bp of ward capacity | 500–800 | 800–1200 | 1200–1700 | 1700–2400 | 2400–3200 | 60 | — |
| `affix.ward_nova` | ward_nova | G | base | on ward break stagger non-boss hostiles within X mm | 2000–2200 | 2200–2500 | 2500–2900 | 2900–3400 | 3400–4000 | 40 | — |
| `affix.ward_full` | ward_full | G | base | +X Bp damage while ward is full | 300–500 | 500–800 | 800–1100 | 1100–1500 | 1500–2000 | 40 | — |
| `affix.sub_<name>` ×24: one per sub-attribute (`affix.sub_power` … `affix.sub_providence`) | sub_<name> | AOC | mundane | +X effective sub-attribute | 1–2 | 2–3 | 3–4 | 4–6 | 6–8 | 30; FAT, SER, PRV 20 | — |
| `affix.tol_<axis>` ×4: arcana, techne, mythos, verve | tolerance | AC | mundane | `gear_tol[axis]` +1 (Canon §5.4) | 1 | 1 | 1 | 1 | 1 | 15 | min T3 |
| `affix.lucre_find` | lucre | AC | mundane | Lucre generated for the bearer ×(10000 + X) | 300–500 | 500–800 | 800–1100 | 1100–1500 | 1500–2000 | 30 | — |
| `affix.soft_step` | soft_step | A | mundane | footstep noise −X dB | 3–4 | 4–6 | 6–8 | 8–10 | 10–12 | 40 | legs_feet |
| `affix.radiance` | radiance | OC | mundane | emits light_block X | 4 | 5 | 6 | 7 | 8 | 20 | — |
| `affix.mount_vitality` | mount_vitality | M | mundane | mount Max HP ×(10000 + X) | 1000–1500 | 1500–2200 | 2200–3000 | 3000–4000 | 4000–5000 | 100 | — |
| `affix.mount_endurance` | mount_endurance | M | mundane | mount stamina ×(10000 + X) | 1000–1500 | 1500–2200 | 2200–3000 | 3000–4000 | 4000–5000 | 80 | — |
| `affix.mount_barding` | mount_barding | M | mundane | +X AR per mount part, × T(ilvl) | 5–8 | 8–12 | 12–17 | 17–24 | 24–32 | 60 | — |
| `affix.gloss` | gloss | WR | gloss | another core genre manufacturer gimmick as a component | 1 | 1 | 1 | 1 | 1 | 20 | pool:cross_glosses min T3 |

### 6.3 Glosses

`affix.gloss` (pool `loot.pool.cross_glosses`, Bindery `unlock.loot.cross_glosses`) draws one extra value in `loot.affix`: a manufacturer chosen uniformly among the 36 core manufacturers whose genre differs from `origin_genre` and whose dependency axis is legal there (1 `weighted`, equal weights). The item gains that manufacturer's §5.2 gimmick as an extra component; `gloss_mfr` stores it. The component's requirement uses the gloss manufacturer's genre axioms, so a Weird West ricochet Gloss on a Wuxia bow needs `Req[verve] = max(1, 7 - 2) = 5`. A Gloss never grants tendencies or parts. Draw count for this affix: 4.

---

## 7. Weapon perks

### 7.1 Columns

Every weapon (families of §3.2) has up to five columns: 1 Frame (barrel, edge or conduit), 2 Feed (magazine, balance or reservoir), 3 Trait I, 4 Trait II, 5 Genre trait.

| Rarity | Columns rolled | Notes |
|---|---|---|
| Common | 1 | plus column 5 |
| Uncommon | 1, 2 | plus column 5 |
| Rare | 1, 2, 3 | plus column 5 |
| Epic, Legendary, Palimpsestic | 1, 2, 3, 4 | plus column 5; Legendary also carries its §8.1 perk outside the columns |
| Exotic | 1, 2 | plus column 5 and the fixed intrinsic (§8.2) |

- **Options per rolled column** by `gear_tier` (Canon §13.9): 1, 1, 2, 2, 3 for tiers 1 to 5. Column 5 always has exactly 1 option.
- **Pool** for a column: perks whose Col matches, whose Families contain the item's family and whose dependency is legal in `origin_genre`, sorted by ID. Column 4 excludes every option already in column 3. Picks are weighted by Wt without replacement.
- **Choosing.** `chosen` starts at 0. Changing it is free at any Scriptorium station, including the Colophon's (§16.5). Only the chosen option is active.
- **Enhanced.** A bracketed magnitude `[n]` becomes `mul_bp(n, 12500)` when enhanced: columns 3 and 4 at gear tier ≥ 2, columns 1 and 2 at tier ≥ 4, column 5 at tier 5. Example: `perk.rampage` at tier 2 grants 1250 Bp per stack.
- Perk numbers enter the formula they name; "+N Bp" without a qualifier enters the additive bucket (Canon §10.5 step 1). Each perk is a component with the dependency shown ("aether" = the class `infused` of §6.1).

### 7.2 Perk table (51 perks) and genre traits (16)

| ID | Col | Families | Dep | Wt | Effect ([n] = enhanced magnitude) |
|---|---|---|---|---|---|
| `perk.honed_edge` | 1 | E | none | 100 | pen_bp +[1500] |
| `perk.weighted_head` | 1 | EB | none | 100 | poise damage ×(10000 + [3000]); attack speed −300 Bp |
| `perk.serrated` | 1 | E | none | 100 | +[25] st.bleeding power per hit |
| `perk.balanced_tang` | 1 | EB | none | 100 | attack speed +[600] Bp |
| `perk.long_haft` | 1 | EB | none | 80 | reach +[250] mm; attack speed −300 Bp |
| `perk.rifled_bore` | 1 | DGH | none | 100 | base spread ×(10000 − [1500]) |
| `perk.long_barrel` | 1 | DGH | none | 100 | effective range ×(10000 + [2500]); handling ticks ×11000 |
| `perk.ported_barrel` | 1 | GH | none | 100 | recoil ×(10000 − [2000]) |
| `perk.heavy_bore` | 1 | DGH | none | 100 | +[700] Bp damage; attack speed −500 Bp |
| `perk.choke` | 1 | G | none | 60 | scatter: pellet spread ×(10000 − [3000]); other guns: spread ×9000 |
| `perk.focused_conduit` | 1 | C | aether | 100 | +[600] Bp spell damage |
| `perk.swift_conduit` | 1 | C | aether | 100 | cast ticks ×(10000 − [1000]) |
| `perk.wide_conduit` | 1 | C | aether | 100 | spell area radius ×(10000 + [2000]) |
| `perk.light_grip` | 2 | EB | none | 100 | melee stamina cost ×(10000 − [1500]) |
| `perk.guarded_hilt` | 2 | EB | none | 100 | +[1000] Bp shield stability while blocking with this weapon |
| `perk.quick_draw` | 2 | EBDGH | none | 100 | swap and draw ticks ×(10000 − [3500]) |
| `perk.counterweight` | 2 | EB | none | 100 | heavy-attack stamina cost ×(10000 − [2000]) |
| `perk.tempered` | 2 | EBDGH | none | 80 | max durability ×(10000 + [5000]) |
| `perk.extended_mag` | 2 | DGH | none | 100 | magazine ×(10000 + [3000]), at least +1 |
| `perk.quick_feed` | 2 | DGH | none | 100 | reload ticks ×(10000 − [1500]) |
| `perk.drum_feed` | 2 | GH | none | 80 | magazine ×(10000 + [5000]); handling ticks ×12000 |
| `perk.hot_loads` | 2 | DGH | none | 80 | +[500] Bp damage; durability loss ×15000 |
| `perk.steady_feed` | 2 | DGH | none | 80 | reloading does not slow movement; reload ticks ×(10000 − [500]) |
| `perk.deep_reservoir` | 2 | C | aether | 100 | wand: +[1] max charge; staff: aether cost ×9000 |
| `perk.quick_siphon` | 2 | C | aether | 100 | wand: recharge ticks ×(10000 − [1500]); staff: spell kills restore 2 aether |
| `perk.stable_reservoir` | 2 | C | aether | 100 | aether cost ×(10000 − [500]) and cast ticks ×(10000 − [500]) |
| `perk.rampage` | 3, 4 | EBDGHC | none | 100 | each kill: +1 stack (max 3) of +[1000] Bp damage for 300 ticks, refreshed |
| `perk.outlaw` | 3, 4 | DG | none | 100 | precision kill: next reload within 300 ticks ×(10000 − [5000]) ticks |
| `perk.kill_clip` | 3, 4 | DGH | none | 100 | reload started within 180 ticks of a kill: +[2500] Bp damage for 300 ticks |
| `perk.feeding_frenzy` | 3, 4 | DG | none | 80 | each kill +1 stack (max 5, 210 ticks): reload ticks ×(10000 − [1000]× stacks) |
| `perk.opening_shot` | 3, 4 | DG | none | 80 | first shot after 120 ticks without firing: spread ×5000 and +[1000] Bp precision multiplier |
| `perk.vorpal` | 3, 4 | EBDGHC | none | 100 | +[1000] Bp vs champion, boss, editor and hunter ranks |
| `perk.coup_de_grace` | 3, 4 | EBDGH | none | 100 | +[2000] Bp vs targets below 3000 Bp HP |
| `perk.headhunter` | 3, 4 | DG | none | 100 | +[2000] Bp precision multiplier |
| `perk.momentum` | 3, 4 | EB | none | 100 | after 30 ticks of sprinting, the next melee hit +[3000] Bp |
| `perk.adrenaline` | 3, 4 | EBDGH | none | 100 | each kill restores [10] stamina |
| `perk.bloodletting` | 3, 4 | EB | mythos | 60 | melee hits heal [150] Bp of HP damage dealt |
| `perk.chain_reaction` | 3, 4 | DGH | verve | 60 | kills explode: [3000] Bp of the killing D2 as dmg.fire, radius 2500 mm |
| `perk.trench_barrel` | 3, 4 | G | none | 80 | a melee hit loads the next 3 shots with +[2500] Bp |
| `perk.close_quarters` | 3, 4 | EBDGH | none | 100 | +[1500] Bp while 2 or more hostiles are within 8000 mm |
| `perk.dragonfly` | 3, 4 | DG | verve | 60 | precision kills: [2500] Bp of the hit as dmg.shock, radius 2000 mm |
| `perk.slipstream` | 3, 4 | DGH | verve | 60 | a dodge reloads the magazine fully; at most once per 300 ticks |
| `perk.unrelenting` | 3, 4 | EBDGHC | none | 80 | 3 kills within 300 ticks heal [500] Bp of Max HP |
| `perk.sharpshooter` | 3, 4 | DG | none | 100 | +[1500] Bp on hits beyond 20000 mm |
| `perk.cadence` | 3, 4 | EB | none | 100 | the 3rd consecutive hit within 90 ticks of the previous +[2000] Bp |
| `perk.sunder` | 3, 4 | EBH | none | 80 | +[35] st.exposed power per hit |
| `perk.frenzy` | 3, 4 | EBDGHC | none | 80 | after 600 ticks in_combat: +[1500] Bp damage and +1000 Bp attack speed until not in_combat |
| `perk.stillness` | 3, 4 | DGC | none | 80 | crouched and still for 60 ticks: spread ×6000 and +[1500] Bp precision multiplier |
| `perk.arcane_surge` | 3, 4 | C | aether | 100 | every [5]th spell cast through this item costs 0 aether |
| `perk.overflow` | 3, 4 | C | aether | 100 | spell kills restore [3] aether |
| `perk.ricochet_rounds` | 3, 4 | G | none | 60 | bullets ricochet once off blocks, the bounce dealing [7000] Bp |

| ID | Genre trait | Dep | Effect |
|---|---|---|---|
| `perk.genre.mythic_fantasy` | Heroic Quest | arcana | +[1000] Bp vs elite rank and above |
| `perk.genre.gothic_horror` | Silvered | mythos | +[1500] Bp vs undead and fiend |
| `perk.genre.cosmic_horror` | Unspeakable | mythos | +[20] st.dread power per hit |
| `perk.genre.space_opera` | Overcharged Cell | techne | the first hit after each reload or after 120 ticks idle +[2500] Bp |
| `perk.genre.cyberpunk` | Smartlink | techne | +[1000] Bp precision multiplier; HUD marks precision parts |
| `perk.genre.post_apocalypse` | Scavenger | none | each kill restores [2000] Bp of a magazine (melee: 5 stamina) |
| `perk.genre.weird_west` | Dead Man's Hand | verve | while HP ≤ 5000 Bp of Max HP: +[1000] Bp crit chance |
| `perk.genre.hardboiled_noir` | Hard-Boiled | none | +[1000] Bp while no allied Erratum is within 10000 mm |
| `perk.genre.wuxia` | Wind and Cloud | verve | the first hit within 60 ticks after a dodge or air jump +[2000] Bp |
| `perk.genre.steampunk` | Pressure Gauge | techne | +[1500] Bp while the magazine (melee: stamina) is above 7500 Bp of max |
| `perk.genre.fairy_tale` | Once Upon a Time | arcana | the first hit on each target in a Folio +[3000] Bp |
| `perk.genre.mythic_antiquity` | Kleos | mythos | killing an elite or higher rank applies st.inspired for [600] ticks |
| `perk.genre.oneiric` | Lucid | arcana | every 600 ticks the next hit adds [40] st.sleeping power |
| `perk.genre.redacted` | Clearance | techne | +[1500] Bp vs targets that are not native (Canon §5.6) |
| `perk.genre.unwritten` | Blank Slate | none | kills of fac.editors members restore [500] Bp of Max HP |
| `perk.genre.ur_text` | Archetypal | none | +[800] Bp vs every target |

### 7.3 Gear tier extras and tuning

- Tier 5 armor and accessories gain a **tuning** choice at a Scriptorium: +2 to one sub-attribute and −2 to another, or +1 to each of the three lowest effective sub-attributes of the wearer at the moment of choosing. Stored in `tuning`; changeable for free.
- Gear tier never changes rarity, affix count or power tables, so `ITEM_MULT_MAX` (Canon §13.8) is unaffected.

---

## 8. Legendary perks, Exotics and Palimpsestic uniques

### 8.1 Legendary perks (26)

A Legendary item draws one perk (`loot.legendary`, 1 `weighted`, equal weights 100) from the pool of its category group whose pool tag is owned and whose dependency is legal in `origin_genre`. Group W covers both weapon categories and wands; A armor; X accessories and ward generators (ward-only perks marked).

| ID | Group | Pool | Dep | Effect |
|---|---|---|---|---|
| `perk.leg.echoing_strike` | W | base | none | every 4th hit repeats 20 ticks later as a Canon §10.6 step-3 hit with `D2` = 5000 Bp of the original, no crit |
| `perk.leg.bloodsong` | W | base | mythos | each kill applies `st.empowered` directly |
| `perk.leg.worldbreaker` | W | legendary_perks_1 | none | block damage × 30000 (doc 03); blocks of hardness ≤ 3 break on hit |
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
| `perk.leg.spellweaver` | X | legendary_perks_1 | aether | every 3rd spell cast +3000 Bp |
| `perk.leg.cartographer` | X | legendary_perks_2 | none | at Folio entry the HUD map reveals every room and Reliquary |
| `perk.leg.redoubt` | X (ward) | legendary_perks_2 | base | while ward > 0, immune to `st.stunned` and `st.feared` |

### 8.2 Exotics (24)

Exotics are unique templates `item.exotic.*` with a fixed archetype, home genre (always their `origin_genre` at generation), manufacturer and intrinsic component. They obey the Canon §13.3 one-per-group equip limit, roll 2 affixes and perk columns 1, 2 and 5. Rewrite (§15) changes their skin and origin genre but keeps name, manufacturer and intrinsic. Duplicates per player are excluded until the category's Exotics are exhausted (§2.3 step 3b).

| ID | Archetype | Home | Mfr | Pool | Dep | Intrinsic |
|---|---|---|---|---|---|---|
| `item.exotic.dragonsong` | blade_long | mythic_fantasy | runecourt | base | arcana | every 3rd consecutive hit within 120 ticks releases a 4000 mm cone of `dmg.fire` for 6000 Bp of its `D2` plus 60 `st.burning` power |
| `item.exotic.widowmaker` | sidearm | weird_west | coffin_creek | base | verve | magazine fixed at 6; the 6th shot uses crit chance 10000 and +5000 Bp |
| `item.exotic.ghost_protocol` | light_head | cyberpunk | chromaw | base | techne | crouched without attacking for 60 ticks: `st.invisible` applied directly, refreshed every 60 ticks; the first hit after it ends +3000 Bp |
| `item.exotic.paper_moon` | `arch.ward.standard` | oneiric | somnambule | base | arcana | capacity × 15000; while ward > 0, immune to `st.feared` and `st.dread` |
| `item.exotic.quietus` | dagger | gothic_horror | vesper_sons | exotic_arms | none | +10000 Bp on sneak attacks; sneak kills restore 15 composure |
| `item.exotic.starfall` | charge_rifle | space_opera | perihelion | exotic_arms | techne | a fully charged shot calls a 3000 mm strike at impact 60 ticks later: 15000 Bp of the shot's `D2` as `dmg.radiant` (environmental entry) |
| `item.exotic.thunderwright` | hafted_maul | mythic_antiquity | hephaesteum | exotic_arms | mythos | heavy attacks emit a 4000 mm `dmg.shock` nova: 5000 Bp of the hit's `D2` plus 40 `st.shocked` power |
| `item.exotic.redline_sutra` | smg | cyberpunk | ghostline | exotic_arms | techne | attack speed +250 Bp per 6 consecutive shots, max +2500; at max, +15 `st.burning` power per hit; resets after 20 idle ticks |
| `item.exotic.rust_psalm` | scatter | post_apocalypse | rustmaw | exotic_arms | none | every 4 pellets of one shot that hit the same target apply one `st.corroded` stack directly |
| `item.exotic.gallows_oath` | crossbow | weird_west | coffin_creek | exotic_arms | verve | leg-part hits on non-boss targets apply `st.rooted` directly for 120 ticks |
| `item.exotic.cogheart` | heavy_auto | steampunk | aetherline | exotic_arms | techne | spends no ammo while the wielder has not moved for 60 ticks |
| `item.exotic.briar_rose` | hafted_flail | fairy_tale | thornwood | exotic_arms | arcana | +30 `st.sleeping` power per hit; +5000 Bp against `st.sleeping` targets |
| `item.exotic.hydra_bow` | bow | mythic_antiquity | argive | exotic_arms | mythos | each arrow splits on its first hit into 2 more that hit the 2 nearest other hostiles within 6000 mm for 3000 Bp of its `D2` |
| `item.exotic.dead_drop` | hand_crossbow | hardboiled_noir | midnight_avenue | exotic_arms | none | bolts emit 0 dB; a lodged bolt detonates after 180 ticks for 8000 Bp of its hit's `D2` as `dmg.blunt` within 1500 mm |
| `item.exotic.black_bar` | staff | redacted | bureau | exotic_arms | techne | precision-part hits apply `st.blinded` directly |
| `item.exotic.lullaby` | wand | oneiric | somnambule | exotic_arms | arcana | every 3rd charge spent is refunded |
| `item.exotic.stormcrown` | medium_head | mythic_fantasy | deepanvil | exotic_armor | arcana | the wearer's crits add 3000 Bp of the crit's `D2` as `dmg.shock`, at most once per 60 ticks |
| `item.exotic.ironhide_greaves` | heavy_legs | post_apocalypse | geigerhaus | exotic_armor | none | immune to `st.slowed` and `st.rooted`; +10 poise |
| `item.exotic.tidecaller_wraps` | light_hands | cosmic_horror | dunwold | exotic_armor | mythos | throw range × 15000; +2000 Bp against `st.soaked` targets |
| `item.exotic.cloud_step` | light_feet | wuxia | iron_crane | exotic_armor | verve | one extra air jump per airborne period; `law_bound: law.wuxia.qinggong` (canon-1.3 §5.10) |
| `item.exotic.drifters_duster` | light_body | weird_west | sundown_rail | exotic_armor | verve | the first shot within 60 ticks after a dodge uses crit chance 10000 |
| `item.exotic.brass_lungs` | medium_body | steampunk | ironclad | exotic_armor | techne | gases breathable; fire DoT on the wearer × 5000 |
| `item.exotic.veil_of_vespers` | light_head | gothic_horror | blackmoor | exotic_armor | mythos | composure loss × 5000 (doc 04) |
| `item.exotic.stampede_sabatons` | heavy_feet | mythic_antiquity | hephaesteum | exotic_armor | none | sprint costs no stamina; sprinting into a minion- or standard-rank hostile applies `st.knocked_down` directly, once per target per 300 ticks |

### 8.3 Palimpsestic uniques (10)

Canon §13.3 rules apply: Contradiction 0 everywhere, no Anachronism Edge, re-skin to the local genre (they never need Rewrite, which refuses them), effective ilvl `max(ilvl, current Folio)`, 4 affixes. Manufacturer `mfr.ur_text.first_press`. Weapons roll columns 1 to 4 and use the local genre's trait in column 5. Sources: §9.8 only. At most one copy of each unique per party per run; never sold, never on Engine faces, never salvaged.

| ID | Archetype | Intrinsic |
|---|---|---|
| `item.palimpsestic.first_word` | blade_long | +5000 Bp vs `fac.editors`; each such kill removes 1 `st.erasure` stack from the wielder |
| `item.palimpsestic.omnibus` | longarm | also carries the local genre's genre trait (§7.2), re-evaluated when the local genre changes |
| `item.palimpsestic.marginal_gloss` | `arch.ward.heavy` | capacity × 20000; ward break applies `st.staggered` to non-boss hostiles within 4000 mm |
| `item.palimpsestic.errata_sheet` | medium_body | `gear_tol` +2 on every axis (Canon §5.4 cap still 6) |
| `item.palimpsestic.authors_quill` | staff | once per 600 ticks a spell cast through it costs 0 aether |
| `item.palimpsestic.dog_ear` | trinket | stores the bearer's position at Folio entry; using it (60-tick channel) returns there, once per Folio |
| `item.palimpsestic.endpaper_band` | ring | +6 to each sub-attribute of the wearer's lowest primary, re-evaluated on change |
| `item.palimpsestic.blank_page` | shield_medium | blocked hits from `fac.editors` use block stability 10000 |
| `item.palimpsestic.running_head` | light_head | HUD shows every Fortune Engine, Reliquary and secret within 32000 mm; mimics are revealed |
| `item.palimpsestic.colophon_mark` | amulet | Lucre generated for the bearer × 12500; the bearer's buy prices × 9000 |

---

## 9. Loot sources

### 9.1 Kill drops and line chances

On `ev.entity_killed` of an NPC, the host rolls the rank table (§9.5) once per Active, living Erratum present in the Folio, in ascending slot order, with `drop_rng(folio, killed EntityId, tick, slot)`. Ghosts, Dormant and Departed slots receive nothing. Summons, minions of `fac.errata` and NPCs flagged `no_loot` (doc 13) drop nothing.

```
line_chance = base_bp
            → × party_drop_bp[n]   if the line is "scaled" (ranks minion to champion)   // [10000, 8000, 6666, 5714] = floor(10^8 / enemy_count_bp[n])
            → × 16667              if mode.abridged and the line is scaled (Canon §17.1)
            → min(10000, ·)
```

Example: an elite's item line (3000) in a party of 2 is `mul_bp(3000, 8000) = 2400` Bp per player. The party meets 12500 Bp as many enemies, so each player's expected yield per Folio equals the solo yield (`12500 × 8000 = 10000 × 10000`).

**Lucre lines** draw `amount = range_incl(mul_bp(4, P(d)), mul_bp(10, P(d)))` in `sub(seed, "loot.lucre")` (1 draw), then `× rank_mult → × (10000 + Σ lucre_find) → × gp.lucre_bp`, creating one `item.lucre` pile. `genre.unwritten` has no Lucre lines (Canon §5.2). `d` is the Folio index (`d_eff` in Abridged).

### 9.2 Category deck

Each player has a 40-card deck, saved in `PlayerLootState`, used by every generic item line:

| Card | Count | Card | Count | Card | Count |
|---|---|---|---|---|---|
| `cat.weapon_melee` | 5 | `cat.ward_gen` | 1 | `cat.text` | 4 |
| `cat.weapon_ranged` | 5 | `cat.wand` | 1 | `cat.throwable` | 3 |
| `cat.armor` | 5 | `cat.implant` | 1 (`cat.accessory` where TEC < 5) | `cat.ammo` | 2 |
| `cat.offhand` | 2 | `cat.potion` | 6 | `cat.gadget` | 1 |
| `cat.accessory` | 3 | | | `cat.part` | 1 |

Cards are drawn in this order: melee, ranged, armor, offhand, accessory, ward_gen, wand, implant, potion, text, throwable, ammo, gadget, part (where TEC < 5 the implant card's count is added to accessory and the implant card is omitted). The draw weight of a card is `count × role_bias_bp` (§9.3). The drawn card's count decrements; when all counts reach 0 the deck refills to the table. Keepsakes and Engine faces use the counts as fixed weights without decrementing. A throwable card picks the counter throwables (§3.5) with total weight 3000 against 7000 for the other throwables, split evenly in each group.

### 9.3 Role bias

| Doc 13 `loot_role` | Category bias (Bp; unlisted = 10000) |
|---|---|
| brute | melee 20000, armor 15000, offhand 15000 |
| skirmisher | melee 15000, throwable 20000, accessory 12000 |
| marksman | ranged 25000, ammo 20000 |
| caster | wand 30000, text 20000, potion 15000, accessory 15000 |
| support | potion 25000, text 15000, ward_gen 20000 |
| swarm | none |
| beast | the item line uses `loot.trophy` instead of the deck (doc 07 hide, bone and meat materials, doc 08 ingredients); Lucre chance 0 |
| construct | part 40000, gadget 20000, ammo 15000, potion 0, text 0 |

### 9.4 In-run bad-luck protection

Per player and run, `blp_rare` counts consecutive rarity-eligible generic rolls below Rare, and `blp_leg` counts those below Legendary. Forced-rarity rolls (Engines, Volume rewards, Keepsakes) neither use nor change them.

```
w[r] = mul_bp(w[r], 10000 + 500 × min(blp_rare, 20))   for r in 2..=6
w[r] = mul_bp(w[r], 10000 + 100 × min(blp_leg, 100))   for r in 4..=6
floor = max(floor, rare)      if blp_rare >= 24
floor = max(floor, legendary) if blp_leg  >= 120
after the roll: rarity >= rare resets blp_rare, else +1; rarity >= legendary resets blp_leg, else +1
```

The HUD ledger shows both counters and both guarantees. Example: `blp_rare = 10` multiplies the Rare-or-better weights by 15000.

### 9.5 Rank tables

"Scaled" lines follow §9.1. Shift and floor feed §2.3 step 3.

| Rank (`loot.drop.<rank>`) | Item lines | Lucre line | Key lines | Other lines |
|---|---|---|---|---|
| minion | 250 scaled | 3000 scaled, × 3000 | — | — |
| standard | 700 scaled | 5000 scaled, × 10000 | — | — |
| elite | 3000 scaled, shift 2000 | 10000, × 30000 | `item.key.bound` 1500 scaled | `item.bag.lucky` 300 in Marginalia |
| champion | 10000 and 5000 scaled, shift 4000, floor Uncommon | 10000, × 60000 | bound 4000, gilded 1000, both scaled | lucky bag 1000 (2500 in Marginalia); beast role: mount token 1500 |
| boss | 10000 and 10000, shift 6000, floor Rare | 10000, × 200000 | one line 10000: gilded 3000 / bound 7000 (`weighted` in `loot.key`) | `item.bag.boss` 10000 |
| editor | — | 10000, × 400000 | `item.key.illumined` 10000 | `item.bag.editor` 10000; Volume reward (§9.8) |
| hunter | 10000, shift 5000, floor Rare | 10000, × 50000 | — | — |

Example record:

```json
{ "id": "loot.drop.elite",
  "lines": [ { "kind": "lucre", "chance_bp": 10000, "scaled": false, "rank_mult_bp": 30000 },
             { "kind": "item", "chance_bp": 3000, "scaled": true, "shift_bp": 2000, "floor": "rar.common" },
             { "kind": "key", "chance_bp": 1500, "scaled": true, "key": "item.key.bound" },
             { "kind": "bag", "chance_bp": 300, "scaled": false, "bag": "item.bag.lucky", "marginalia_only": true } ] }
```

### 9.6 Chests and floor loot

Doc 12 places `4 + range(3)` chests per Normal Folio. Container contents roll on opening with `drop_rng(folio, container uid, tick, slot)`; each player opens their own instance.

| Table | Lines |
|---|---|
| `loot.chest.plain` | item 10000; item 3000; Lucre 7000 at `range_incl(mul_bp(15, P), mul_bp(23, P))`; consumable (deck restricted to potion, text, throwable, ammo) 5000; bound key 600; gilded key 100; mount token 150 |
| `loot.chest.locked` (doc 09 lock) | item 10000 shift 2000 floor Uncommon; item 6000; Lucre 10000 at the same range × 15000; bound key 1500 |
| `loot.floor` | worldgen places 3 items and 6 `item.lucre` piles of `range_incl(mul_bp(18, P), mul_bp(27, P))` per player from `rng.loot_place`, in slot order 0 to 3 whatever the party size (Folio isolation, Canon §21 test 31) |

### 9.7 Treasure bags (Experienced Pixel Dungeon analog)

Bags are `cat.container` items (`stack_max` 1, mass 500 g). Using one (30 ticks) rolls its contents with `drop_rng(folio, bag uid, tick, slot)` and spills them as the user's instanced ground items.

| Bag | Contents |
|---|---|
| `item.bag.boss` | `2 + range(2)` items, shift 3000, floor Uncommon; Lucre `range_incl(mul_bp(40, P), mul_bp(85, P))` |
| `item.bag.editor` | 4 items, shift 4000, floor Rare; Lucre `range_incl(mul_bp(80, P), mul_bp(170, P))`; the Volume's unplaced Emendations (§9.9); 1000 Bp one Palimpsestic unique |
| `item.bag.lucky` | `range(15)` items (0 to 14), no floor |
| `item.bag.elite` | one uncursed item per equipment card (melee, ranged, armor, offhand, accessory, ward_gen); dropped by Marginalia Editors |
| `item.bag.refined` | 3 × `range(15)` items plus 1 item with floor Epic |
| `item.bag.burnt` | Lucre `mul_bp(20, P)` |

Burning: when a ground `item.bag.lucky` takes `dmg.fire`, it rolls keyed `rng.loot` (actor = bag uid, tick, seq = finder slot) `roll_bp(1000)`: success → `item.bag.refined`, failure → `item.bag.burnt`. Marginalia boss Folios drop 2 boss bags per player.

### 9.8 Bosses, Volume rewards, secrets and Palimpsestic sources

- **Volume reward** (`loot.volume_reward`, doc 14 §3.9): an offer of 3 items from `offer_rng(slot, editor EntityId, "volume_reward")` (§11.2): option 0 Legendary of a deck-weighted equipment card, option 1 Exotic, option 2 Palimpsestic with 2500 Bp else Legendary. `ilvl` = Folio index, `buc.uncursed`, knowledge `know.appraised`.
- **Secrets** (doc 12 kinds): `loot.secret.cache` 3 items shift 3000 floor Uncommon plus a chest Lucre line; `loot.secret.vault` one gilt Fortune Engine or one illumined Reliquary (`rng.loot_place` `weighted([5000, 5000])`); `loot.secret.palimpsest` one Palimpsestic unique per Active Erratum (keyed `rng.loot`, actor = vault uid).
- Palimpsestic uniques come only from Editor bags, Volume rewards and Palimpsest vaults (Canon §13.3).

### 9.9 Volume plan: Emendations, Rewrite Quills and keys

The plan of Volume `v` is the first 8 draws of `rng.loot_place` for Folio `(v - 1) × 5 + 1`, recomputed by any later Folio of the Volume without generating earlier Folios: `u1 = range(5)`, `u2 = range(4)`, `u3 = range(3)`, `u4 = range(2)`, `q = range(4)`, `b1 = range(4)`, `b2 = range(4)`, `g = range(4)`.

```
remaining = 3
for pos in 1..=4: if u_pos < remaining { emendation at pos; remaining -= 1 }
boss Folio: the remaining Emendations go into item.bag.editor
Rewrite Quill at pos q + 1; bound key caches at pos b1 + 1 and b2 + 1 (only the first max(0, 2 + gp.reliq_key_delta)); gilded key cache at pos g + 1
```

Every Active Erratum gets an instanced copy at the same placement (a guarded spot, doc 12). Each position has expected 0.6 Emendations. Abridged uses positions 1 to 3 with `range(3)`, `range(2)`, then `q, b1, b2, g` as `range(2)`. Marginalia Volumes ignore `u1..u4` (§16.2).

### 9.10 Champion counters, mounts, ward generators and parts

- Counters (Canon §17.3a): every `shop.colophon` and `shop.general` shelf holds 2 of each counter throwable per player and restocks them (§12.3); when the exit preview shows a champion aspect, the next Folio's `rng.loot_place` places one cache per aspect near the Incipit holding 2 matching counters per player. This holds in all 16 genres.
- Mount tokens: champion beast role 1500, chests 150, Reliquary consumable bundles 500, `shop.wanderer`. Ward generators and parts come from deck cards; constructs bias parts; salvage yields parts (§16.4).

---

## 10. In-run Reliquaries

| Tier | Key | Contents (each a 10000 line) | Shift | Mimic | Placement (doc 12 places the room spot) |
|---|---|---|---|---|---|
| `reliq.plain` | none; 90-tick open | 1 deck item; 1 equipment floor Uncommon | 0 | 400 | each Normal Folio: `1 + range(2)` |
| `reliq.bound` | `item.key.bound` | 2 equipment floor Uncommon; 1 bundle of 3 deck consumables (500 Bp: a mount token instead) | 2000 | 300 | 6000 Bp per Normal Folio from Folio 2 |
| `reliq.gilded` | `item.key.gilded` | 2 equipment floor Rare; 1 equipment floor Epic | 3000 | 200 | 2500 Bp per Normal Folio from Folio 6 |
| `reliq.illumined` | `item.key.illumined` | 3 equipment floor Rare; 1 equipment floor Legendary | 4000 | 0 | every boss Folio's Endpaper; `loot.secret.vault` |

- Existence, tier and mimic flag are drawn at Folio generation from `rng.loot_place` (per Reliquary: `roll_bp` for existence where chanced, then `roll_bp(mimic)`, always both). A higher key opens a lower tier. Keys sit on a keyring (§14.1).
- **Instancing.** Each Active Erratum has its own instance and its own contents, rolled on opening with `drop_rng(folio, reliquary uid, tick, slot)`. The mimic flag is shared.
- **Bad-luck protection** per player and run: `rq5` = Reliquaries opened since one held Epic or better, `rq20` = since Legendary or better. When `rq5 = 4`, the equipment slot with the highest floor uses floor Epic; when `rq20 = 19`, it uses floor Legendary (both may apply; Legendary wins). The tooltip shows "Epic or better within `5 - rq5`" and "Legendary or better within `20 - rq20`".
- **Displayed odds**: per slot, the §2.3 step-3 weights with the opener's SER, shown to 4 decimals. Example: a plain Reliquary's equipment slot at Folio 7, SER 40: weights `[0, 2728, 888, 258, 88, 33, 0]`, total 3995: Uncommon 68.2854%, Rare 22.2278%, Epic 6.4581%, Legendary 2.2028%, Exotic 0.8260%.
- **Mimic.** A mimic is a hidden object for Canon §6.4 row 35 detection (doc 09); detection shows a tell. Opening it does not spend the key; it becomes `enemy.mimic` (doc 13, `rank.elite`, `no_loot`). Its death converts it back into an open Reliquary whose contents every player loots without a key, plus one extra item line (shift 2000) per player.

| State | Event | Next |
|---|---|---|
| Sealed | player with a valid key (or none for plain) interacts 90 ticks, not a mimic | Open(player): key spent, contents rolled, `ev.container_opened` |
| Sealed | same, mimic | Awake: key kept, `enemy.mimic` spawns at the Reliquary |
| Awake | mimic killed | Lootable: every player may open without a key |
| Open(player) or Lootable | other players | their own instance stays Sealed or Lootable |
| any | Folio exit | removed |

---

## 11. Fortune Engines

### 11.1 Tiers

| Engine | Base price | Face rarities 0..5 | Face weights 0..5 (Bp) | Placement (`rng.loot_place`) |
|---|---|---|---|---|
| `engine.brass` | 60 | C, C, U, U, R, E | 2600, 2600, 1800, 1800, 900, 300 | 1 in every Normal Folio |
| `engine.silver` | 160 | U, U, R, R, E, L | 2500, 2500, 1800, 1800, 1000, 400 | 5000 Bp in Normal Folios from Folio 3 |
| `engine.gilt` | 420 | R, R, E, E, L, X | 2500, 2500, 1800, 1800, 1000, 400 | each boss Folio's Endpaper; `loot.secret.vault` |
| `engine.armory`, `engine.wardrobe`, `engine.curio` | 160 | as silver | as silver | pool `loot.pool.fortune_engines_2`: each silver placement becomes a variant with 5000 Bp (1 `roll_bp`, 1 `weighted` over 3 equal) |

Variants restrict face categories: armory to weapons and off-hands, wardrobe to armor and ward generators, curio to accessories, wands and implants. Other engines draw face categories from the equipment cards of §9.2 as fixed weights. No engine exists in `genre.unwritten`.

### 11.2 Offers, odds and dispensing

```rust
fn offer_rng(slot: u8, context_uid: u64, purpose: &str) -> Rng {
    // peek_u64 returns the next output of rng.offer.p<slot> without advancing it (Canon §3.5)
    Rng::from_seed64(mix64(peek_u64(OFFER[slot]) ^ mix64(context_uid ^ fnv1a64(purpose))))
}
fn engine_interact(p: &mut Player, e: &mut Engine) {
    if e.faces[p.slot].is_none() {                        // generated once, stored, re-shown on reopen
        let mut o = offer_rng(p.slot, e.uid, "engine_faces");
        e.faces[p.slot] = Some((0..6).map(|i| generate_item(o.next_u64(),
            &GenRequest::engine_face(e.tier.face_rarity[i], e.category_rule, d_current))).collect());
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
        &GenRequest::engine_face(e.tier.face_rarity[i], e.category_rule, d_current));
    e.spins[p.slot] = k + 1;
    p.engine_pity[e.tier.idx] = if i == 5 { 0 } else { p.engine_pity[e.tier.idx] + 1 };
    emit(Ev::EngineSpun { engine: e.uid, slot: p.slot, face: i, item: item.uid, price });
    Ok(item)
}
fn displayed_weights(p: &Player, e: &Engine) -> [u32; 6] {
    let need = if p.carries(TRINKET_WISHBONE) { 3 } else { 4 };
    if p.engine_pity[e.tier.idx] >= need { return [0, 0, 0, 0, 0, 10000]; }   // guarantee armed and shown
    let mut w = e.tier.weights;
    let s = if p.has(ST_UNMOORED) { 0 } else { clamp((p.ser() - 40) * 10, 0, 500) };
    w[0] -= s; w[5] += s;
    w
}
```

- Faces are generated with `buc.uncursed` and `know.appraised`, `ilvl = d_current`, forced rarity, no bad-luck counters. The panel shows each face's item card, its weight and its probability to 4 decimals, the price, the capsules left (5 − `spins`) and the pity line "top capsule guaranteed after `need − pity` more".
- Example: SER 70 at a brass engine gives `s = 300`, weights `[2300, 2600, 1800, 1800, 900, 600]`. Brass prices at Folio 7 are 100, 125, 150, 175 and 200 Lucre. Expected capsules to the top face from fresh pity: 4.71.
- **Compliance.** One fixed price per capsule, no stake choice, no multiplier, no repeat-wager, no empty outcome: every payment yields exactly one item from the displayed faces. Presentation is a crank and a capsule chute: no reels, no symbols, no flashing jackpot lights, no near-miss animation, no win or lose wording, no coin-shower audio.

| State (per player and engine) | Event | Next |
|---|---|---|
| Idle | interact | Shown (faces generated if absent) |
| Shown | `ShopBuy` intent, Lucre ≥ price, `spins < 5` | Dispensing (30 ticks), then Shown; `spins = 5` → Exhausted |
| Shown | close | Idle (faces kept) |
| any | Folio exit | removed |

---

## 12. Shops and services

### 12.1 Shop types and stock

Every Normal Folio has the Colophon shop (canon-1.3 §17.1) plus `1 + [roll_bp(4000)]` shops drawn with weights armory 3, apothecary 3, curio 2, fence 1, wanderer 1 (`rng.loot_place`). Boss Folios have the Endpaper Colophon shop only. `genre.unwritten` has none.

| Shop | Lines per player | Markup |
|---|---|---|
| `shop.colophon` | 3 consumables (potion, text, throwable cards), 3 counter lines × 2, 3 equipment shift 1500 floor Uncommon | 15000 |
| `shop.general` | 6 consumables, 3 counter lines × 2, 2 food lines × 5 | 14000 |
| `shop.armory` | 6 equipment (weapon, armor, offhand, ward_gen cards), shift 1000 | 17500 |
| `shop.apothecary` | 6 potion lines × 2, 2 doc 08 ingredient lines × 5 | 14000 |
| `shop.curio` | 4 lines: accessory, wand, implant (accessory where TEC < 5), text | 17500 |
| `shop.fence` | 3 equipment floor Rare shift 3000; buys at sell price × 11000 | 22000 |
| `shop.wanderer` (`fac.neutral`) | 4 mixed lines floor Uncommon, 1 mount token | 16000 |

Stock is generated at Folio generation from `rng.loot_place`: shops in ascending entity order, slots 0 to 3 always, lines in table order, one `next_u64` item seed per line; SER 40; `buc.uncursed`; `know.identified`. Vendor names by genre:

| Genre | Colophon merchant | Armory | Apothecary | Curio | Fence |
|---|---|---|---|---|---|
| mythic_fantasy | Wayside Peddler | Smithy | Herbalist | Enchanter | Thieves' Guild Factor |
| gothic_horror | Sexton | Silversmith | Apothecary | Curio Cabinet | Grave-Robber |
| cosmic_horror | Antiquarian | Outfitter | Dispensary | Occult Bookshop | Dockside Smuggler |
| space_opera | Station Quartermaster | Armory Kiosk | Medbay | Relic Trader | Smuggler |
| cyberpunk | Vending Node | Gun Shop | Street Doc | Ripper Clinic | Fixer |
| post_apocalypse | Scrap Trader | Gun Runner | Rad-Doc | Junk Collector | Raider Fence |
| weird_west | Traveling Salesman | Gunsmith | Snake-Oil Wagon | Medicine Lodge | Bandit Fence |
| hardboiled_noir | Newsstand | Pawn Gun Counter | Drugstore | Antique Dealer | Fence |
| wuxia | Wandering Merchant | Weaponsmith | Herb Hall | Talisman Stall | Black Market |
| steampunk | Automaton Clerk | Ordnance Works | Chemist | Curiosity Emporium | Underworks Fence |
| fairy_tale | Old Peddler | Blacksmith | Hedge Witch | Wishing Stall | Goblin Market |
| mythic_antiquity | Agora Merchant | Bronzesmith | Healer | Oracle's Stall | Pirate Trader |
| oneiric | Dream Merchant | Toy Armory | Sleep Chemist | Memory Stall | Shadow Self |
| redacted | Requisitions Desk | Armory Cage | Infirmary | Evidence Locker | Leak |
| ur_text | The Last Stationer | — | — | — | — |

### 12.2 Prices

```
buy  = value → × markup_bp → × row37_bp (Canon §6.4) → × bearer modifiers in ascending item uid order
            → × gp.shop_price_bp → × (10000 - haggle_bp) → max(1, ·)
sell = value → × row38_share_bp → × 5000 if not know.appraised → × 5000 if known cursed
            → × 11000 at a fence → × gp.lucre_bp → max(1, ·)          // each arrow one mul_bp
```

Unsellable: `item.tool.emendation`, quest items, Palimpsestic items. Example: the §20 sidearm (value 1123) sells for `mul_bp(1123, 3000) = 336` at CHM 40, or 168 unappraised; an armory sells it at `mul_bp(1123, 17500) = 1965`.

**Haggling** (deterministic, no draw), once per player, shop and Folio, via an intent: `score = skill.speech + floor_div(CHM - 40, 2) - (20 + min(60, d))`. `score >= 0` → `haggle_bp = min(2000, 500 + 50 × score)`; `-20 <= score < 0` → 0; `score < -20` → `haggle_bp = -500` for that player at that shop this Folio. Example: Speech 40, CHM 70, Folio 7: `score = 40 + 15 - 27 = 28`, `haggle_bp = 1900`; the 1965 price at row 37 × 9100 becomes 1788, then 1448.

### 12.3 Restock, buyback and services

- **Restock**: consumable, counter and food lines refill to their initial quantity on every tick `entry_tick + 3600 × k`. Equipment lines never restock. No draws.
- **Buyback**: each shop keeps, per player, the last 12 items that player sold to it this Folio, for sale back at the exact sell price paid; cleared at Folio exit.
- **Services** at any Scriptorium (room or Colophon station), each price × `gp.shop_price_bp`:

| Service | Price (Lucre) |
|---|---|
| Identify one item (doc 10 `know.identified`) | `max(mul_bp(10, P(d)), mul_bp(value, 1000))` |
| Uncurse one item (as an Emendation's curse branch, no enhancement change) | `mul_bp(value, 5000)` |
| Repair to max durability | `mul_bp(value, floor_div(3000 × missing, max))` |
| Cure an affliction (Canon §16.4) | `mul_bp(500, P(d))` |
| Reattach a severed part (Canon §9.1) | `mul_bp(200, P(d))` |
| Rewrite, reforge | §15, §16.3 |
| Perk choice, tuning | free |

---

## 13. Economy

### 13.1 Item value

```
value = base_value → × P(ilvl) → × rarity value (Canon §13.3: 10000, 20000, 40000, 80000, 160000, 320000, 640000)
                   → × quality_bp → × (10000 + 600 × enhancement) → × material value_bp (doc 07; mat.iron 10000)
```

Each arrow is one `mul_bp`. Consumables use rarity and quality 10000. `value` is recomputed whenever ilvl, enhancement, rarity, material or quality changes. Example (§20): `70 → × 16771 = 117 → × 80000 = 936 → × 12000 = 1123 → × 10000 → × 10000 = 1123` (material `value_bp` taken as 10000).

### 13.2 Lucre faucets and sinks

Per player per Normal Folio at Grade 0, `LUCRE_TARGET(d) = mul_bp(450, P(d))`. Faucet shares: kills 3000 Bp (40 standard-kill equivalents, Canon §17.4, × 5000 chance × mean amount), floor piles 3000, containers 1500, sales 2500; doc 14 marginal notes add up to `2 × mul_bp(30, P(d))`. Sink targets: shops 5500, Fortune Engines 2000, services 1500, carried forward 1000.

| d | Target | Kills | Piles | Chests | Sales (rest) | Notes (max) | Brass spin 1 | Silver spin 1 | Gilt spin 1 |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 450 | 140 | 135 | 66 | 109 | 60 | 60 | 160 | 420 |
| 5 | 635 | 190 | 189 | 92 | 164 | 84 | 84 | 225 | 592 |
| 10 | 977 | 290 | 291 | 141 | 255 | 130 | 130 | 347 | 912 |
| 15 | 1503 | 460 | 450 | 220 | 373 | 200 | 200 | 534 | 1403 |
| 20 | 2313 | 710 | 690 | 341 | 572 | 308 | 308 | 822 | 2159 |
| 25 | 3559 | 1100 | 1065 | 523 | 871 | 474 | 474 | 1265 | 3322 |

Σ `LUCRE_TARGET(d)` for `d = 1..=25` is 38,102 Lucre per player. Lucre never converts to any account currency (Canon §14). Bots (doc 02) assert each Folio's realized income within 7000..=13000 Bp of target.

---

## 14. Inventory

### 14.1 Model

- **Backpack**: 20 slots, +6, +10 or +14 from the `slot.back` pack (§3.4). One slot holds one item or one stack; size and shape never matter.
- **Equipment slots**: Canon §13.2. Equipped items use no backpack slot. The quickbar (4, or the belt's 5 to 8) references backpack stacks.
- **Keyring**: keys, up to 20, no slot. **Ammo reserve**: per `ammo.` type, no slot, caps owned by doc 05. **Purse**: Lucre `i64`, no mass.
- **Mass**: `carried_g` = Σ `mass_g × stack` over backpack, equipment, keyring and ammo. Capacity is Canon §6.4 row 10 × 1000 g; `load_bp` drives `st.encumbered` and `st.overburdened` (Canon §11.1).

### 14.2 Stacking

Two stacks merge iff template, `origin_genre`, `buc`, knowledge state and flags, `quality` and `ilvl` are equal and neither has affixes. The merged stack keeps the lower `uid`; splitting takes a new uid from the current Folio counter. Stack caps: §3.5.

### 14.3 Pickup

Lucre, ammo and keys within 1500 mm auto-pickup (3000 mm with `item.trinket.lodestone`). Other items need a 10-tick interact. A pickup that does not fit leaves the item on the ground. A granted item (Emendation, Keepsake, Volume reward) that does not fit drops at the receiver's feet with `Instanced(slot)`.

### 14.4 Ground items and visibility

Every ground item is an entity counted toward `GROUND_ITEM_CAP` (Canon §4).

| State | Event | Next |
|---|---|---|
| `Instanced(s)` (from a player's roll) | player `s` picks up | Held(s) |
| `Instanced(s)` | other players | invisible and unreachable to them |
| Held(s) | `DropItem` | `Public` |
| `Public` | any player picks up | Held |
| `Instanced` or `Public` | Lost Satchel merge | Satchelled (visibility kept) |
| any ground state | Folio exit | destroyed |

### 14.5 Lost Satchel

```rust
fn on_ground_insert(f: &mut Folio) {
    if f.ground_count() <= GROUND_ITEM_CAP { return; }
    let s = f.lost_satchel_or_create(f.incipit_anchor());           // a ground container; counts as 1
    for it in f.ground_items_by_uid() {                              // ascending uid
        if f.ground_count() <= GROUND_ITEM_CAP - 64 { break; }
        if it.is_quest() || it.is_corpse() || it.is_satchel() || f.within_any_erratum(it.pos, 8000) { continue; }
        s.absorb(it);                                                // keeps visibility; Lucre piles add per finder
    }
}
```

The Lost Satchel shows each player only their own and Public contents, has no capacity limit, and is destroyed at Folio exit.

---

## 15. Rewrite

- **Where**: a Scriptorium room or the Colophon's station, or anywhere with `item.tool.rewrite_quill` (60-tick channel, consumed). The target genre is the local genre at the user (the Folio's genre in a seam zone).
- **Refused** for Palimpsestic and quest items, when the target equals `origin_genre`, when the target has no skin for the archetype (`—`), and when a Legendary perk, Exotic intrinsic or Gloss would have an illegal dependency in the target.
- **Cost** at a Scriptorium: `max(mul_bp(20, P(d)), mul_bp(value, 3500))` × `gp.shop_price_bp`. Example: value 1123 at Folio 9: `max(39, 393) = 393`.
- **Offers**: the Scriptorium shows one variant per target-genre manufacturer that makes the item's class, built from `offer_rng(slot, item uid, "rewrite")`; committing pays and advances the stream once. The Quill applies variant 0 (the counterpart manufacturer), with replacement draws from keyed `rng.loot` (actor = item uid, seq = slot).

```rust
fn rewrite(it: &mut ItemInstance, target: GenreId, mfr_choice: MfrId, rng: &mut Rng) {
    it.origin_genre = target;                                         // skin, base dependency, Req follow (Canon §5.3)
    if !it.exotic && it.manufacturer_is_core_or_secret() {
        it.manufacturer = mfr_choice;                                 // default: same row index (1, 2, 3) in target, else first that makes the class
        it.reapply_tendencies();                                      // doc 05 base stats x new tendencies
    }
    it.materials = it.materials.iter().map(|m| doc07::equivalent(*m, target)).collect();
    for a in it.affixes.iter_mut() {                                  // ascending affix index
        if !class_legal(a.id, target) {                               // same tier and roll_bp; new affix id
            a.id = weighted_pick(legal_affixes_same_cats_unused_groups(it, target), rng);
            it.knowledge.clear_affix_flag(a);
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
| uid, template, rarity, quality, ilvl, enhancement, buc, curse penalty, durability ratio, charges, gear tier, chosen indices, Legendary perk, Exotic intrinsic and name, Gloss, tuning, illuminated, finder slot, reforge count | origin genre, skin, dependencies and `Req`, manufacturer and tendencies (non-Exotic), material, illegal affixes and perk options, column 5, value, `rewrite_count` |

Every committed Scriptorium Rewrite counts for doc 14 `run.note.scribe`.

---

## 16. Emendations, reforging, salvage and Scriptorium actions

### 16.1 Emendation

`item.tool.emendation` (cat.tool, never type-hidden, unsellable). Use on an equipment item: 30-tick channel. Canon §13.7: if `buc = cursed` it uncurses instead; otherwise `ilvl = max(ilvl, current Folio)`, then `enhancement += 1`. Refused at the cap: 15 before Folio 26, `15 + floor_div(d_current - 25, 5)` in Marginalia. Placement: §9.9.

### 16.2 Marginalia Emendation flow

On `ev.folio_exited` from Folio `f >= 26`, every slot that is Active or Ghost receives one Emendation created with Folio `f`'s uid counter, in ascending slot order, before the next Folio is generated. If it does not fit, it appears at the receiver's feet at the next Incipit as `Instanced(slot)`. No Volume plan Emendations exist in Marginalia.

### 16.3 Reforging

At a Scriptorium, the player picks one affix; `offer_rng(slot, item uid, "reforge")` shows 3 candidates (each: eligible affix by §6.1 excluding the item's other groups, tier by §6.1 weights, `roll_bp`). Committing one replaces the affix, pays, advances the stream once and increments `reforge_count`; declining costs nothing.

```
cost = mul_bp(mul_bp(30, P(d)), rarity_value_bp) × (2^(c+1) - 1) → × gp.shop_price_bp,   c = reforge_count ∈ 0..=3; refused at 4
```

Example: Epic at Folio 9: `mul_bp(mul_bp(30, 19925), 80000) = 472`; successive reforges cost 472, 1416, 3304, 7080.

### 16.4 Salvage

60-tick channel anywhere, or 0 ticks at a crafting station (doc 07). Yields `mul_bp(component mass, 6000)` of each component material as `cat.material` units (doc 07 rounds to its unit), and one loose part of the item's manufacturer pool with chance Uncommon 1000, Rare 2500, Epic 5000, Legendary or better 10000 (keyed `rng.loot`, actor = item uid, seq = slot: 1 `roll_bp` and 1 `weighted`, always both). Palimpsestic and quest items cannot be salvaged.

### 16.5 Free Scriptorium actions

Perk choice per column, Tier-5 tuning, and viewing every displayed odd. Doc 02 `CommitOffer` carries the column or tuning choice.

---

## 17. Co-op loot

- **Instanced (default).** Every kill, container, Reliquary, bag, Engine and shop shelf is rolled per Active player with `seq = slot`; `finder_slot` records the roller. Lucre is per player. Players transfer anything, Lucre piles included, by dropping it (Public).
- **Ghost loss.** Canon §17.6 picks the lost item with keyed `rng.loot` (actor = ghost EntityId, seq 0) among equipped non-weapon items sorted by uid.
- **`mode.loot_shared`** (Canon §17.5). Each kill or container rolls once with `seq = 0`, SER = `floor_div(Σ SER of Active Errata, n)`, and drops Public items. Scaled lines use `shared_drop_bp[n] = [10000, 12800, 16000, 18286]` instead of `party_drop_bp`, so each player receives 8000 Bp of the solo yield; a line chance above 10000 rolls twice (a 10000 line, then a line of the excess), with 4 draws always. Reliquaries have one Public instance; shops keep one shelf (slot 0's stock); deck and bad-luck counters belong to slot 0. Engines and Volume rewards stay per player.

---

## 18. Item Power

```
chain = 10000 → × P(eff_ilvl) (T for armor and ward generators) → × rarity power → × quality_bp
              → × (10000 + 600 × eff_enh) → × material dmg_bp or ar_bp (accessories: 10000)
IP = floor_div(chain, 100) + Σ affix tier + 5 × [legendary perk] + 10 × [exotic or palimpsestic intrinsic]
```

Only knowledge the player holds is used (unknown parts count as their weakest values: tier 1, `eff_enh` 0). Character Power = `floor_div(Σ IP over the best weapon slot, five armor slots and the ward slot, 7)`, empty slots 0. A Common Normal +0 iron ilvl-1 weapon has IP 100; the §20 sidearm 237 (chain 23143, material `dmg_bp` taken as 10000); a Palimpsestic Peerless +15 material-14000 weapon at ilvl 25 has chain IP 4103 before affixes. IP is UI and telemetry only.

---

## 19. Saved loot state

`PlayerLootState` per slot (saved, Canon §2.5): deck counts, `blp_rare`, `blp_leg`, `rq5`, `rq20`, `engine_pity[3]`, Exotics generated this run, haggle and buyback records of the current Folio. Per Folio: engine faces and spins per slot, Reliquary flags, shop shelves, Lost Satchel. Per run: Palimpsestic uniques held. `rng.offer.p<slot>` states are saved by the canon contract.

---

## 20. Worked example: one weapon drop

Run seed `0x0123456789ABCDEF`, Standard mode, Grade 10 (`gear_tier = min(5, 1 + floor_div(10, 5)) = 3`, `gp.lucre_bp = 9000`). Folio 7 (Volume 2, position 2) in `genre.weird_west`, party of 2. Receiving player slot 1, SER 52, fresh deck and counters. An elite (`loot_role` marksman, EntityId `(7 << 40) | 42`) dies at tick 55606. Doc 07's Weird West ranged-weapon material pool is taken as `mat.hexed_silver` 1500, `mat.iron` 7000.

1. **Drop generator.** `stream_seed(run, "rng.loot", 7) = 0x4e6b3f6f4dafc989`; keyed seed `0x83db3ac3a7065305`.
2. **Lines of `loot.drop.elite`** (2 draws each):

| Line | Chance | `range(10000)` | Hit | Item seed |
|---|---|---|---|---|
| Lucre | 10000 | 5808 | yes | `0x80b11f0132e41905` |
| item | `mul_bp(3000, 8000) = 2400` | 1593 | yes | `0xe5bf711b4609c44a` |
| bound key | `mul_bp(1500, 8000) = 1200` | 8978 | no | `0x649a68f2aa2be66e` |
| lucky bag | 0 (not Marginalia) | 1204 | no | `0x6989235ffde8ebe2` |

3. **Lucre.** `range_incl(mul_bp(4, 16771), mul_bp(10, 16771)) = range_incl(6, 16)` → 10; × 30000 → 30; × 9000 → **27 Lucre**.
4. **Category.** `sub(seed, "loot.cat")` has seed `mix64(0xe5bf711b4609c44a ^ 0x3418799fa9ecd0ed) = 0x59f6955537f06994`. Weird West has TEC 4, so the implant card joins accessory. Weights with the marksman bias: `[50000, 125000, 50000, 20000, 40000, 10000, 10000, 60000, 40000, 30000, 40000, 10000, 10000]`, total 495000; `range` gives 95600 → past melee (50000) → **`cat.weapon_ranged`**; its count becomes 4.
5. **Archetype.** Weights in ID order: bow 900, charge_rifle 600, crossbow 800, flamer 400, hand_crossbow 700, heavy_auto 400, launcher 400, longarm 1000, scatter 800, sidearm 1000, sling 500, smg 800, staff 700 (total 9000); draw 6543 → **`arch.ranged.sidearm`** (cumulative 6000..=6999), skin "Six-Shooter [T]", `Req[techne] = max(1, 4 - 2) = 2`.
6. **Rarity.** Canon §13.4 at `d = 7`, SER 52: `[7000, 2860, 960, 285, 99, 37, 0]`; elite shift 2000: `[7000, 3432, 1344, 456, 178, 74, 0]`, total 12484; draw 11893 → cumulative 11776..=12231 → **Epic** (3 affixes, columns 1 to 4). `blp_rare` resets to 0; `blp_leg` becomes 1.
7. **Manufacturer.** `[Marginal Press 1000, Coffin Creek 3000, Seven Seals 3000, Sundown Rail 3000]`; draw 8474 → **`mfr.weird_west.sundown_rail`** (D+800, S−700; Sticky Fuse, dependency techne, `Req = 2`).
8. **Quality.** Draw 9552 of 10000 → cumulative 9300..=9799 → **`q.excellent`** (12000).
9. **Material.** `[hexed_silver 1500, iron 7000]`; draw 42 → **`mat.hexed_silver`**.
10. **Affixes** (ilvl 7 unlocks T1 and T2; Epic halves T1: tier weights `[5000, 6000]`):

| # | Eligible (count, Σ Wt) | Affix | Tier | `roll_bp` | Value |
|---|---|---|---|---|---|
| 1 | 46, 1895 | `affix.self_repair` (tech, `Req[techne] = 2`) | 2 | 4095 | 1 durability per 818 ticks |
| 2 | 45, 1875 | `affix.infuse_fire` (infused → mythos, `Req[mythos] = 4`) | 2 | 4630 | 1731 Bp to `dmg.fire`, +24 `st.burning` power |
| 3 | 34, 1600 (infusion group removed) | `affix.dmg_vs_beast` (mundane) | 2 | 1146 | +1880 Bp vs `fac.wild.*` |

11. **Perk columns** (family G, 2 options each): column 1 `perk.choke`, `perk.rifled_bore`; column 2 `perk.drum_feed`, `perk.tempered`; column 3 `perk.kill_clip`, `perk.unrelenting`; column 4 `perk.headhunter`, `perk.frenzy`; column 5 `perk.genre.weird_west` (Dead Man's Hand). Traits are enhanced at tier 3: `perk.kill_clip` grants `mul_bp(2500, 12500) = 3125` Bp.
12. **Curse** by doc 10 from `sub(seed, "loot.buc")`; no other value depends on it. **Illuminated**: not rolled (below Legendary).
13. **Derived.** `uid = (7 << 40) | 19` (the 19th item of Folio 7); `value = 1123` (§13.1); `finder_slot = 1`; `visibility = Instanced(1)`; knowledge `know.classified` with every flag clear; IP 237 (§18). The record is the §2.2 example.

---

## Exports

All IDs below are in namespaces Canon §19 assigns to doc 11, pass the Canon §1.1 grammar, are at most 64 bytes and have pairwise distinct `fnv1a32` (acceptance test 30).

| Namespace | IDs |
|---|---|
| `item.` | `item.ring.{might, finesse, vigor, intellect, will, perception, presence, fortune, arcane_tongue, machine_tongue, sacred_tongue, bold_tongue, warding, wealth, keen}`; `item.amulet.{sanctuary, elements, ferocity, composure, vigor, fortune, wayfarer, scholar}`; the 18 `item.trinket.*` of §3.4; `item.belt.{sash, bandolier, harness, utility}`; `item.back.{cloak, satchel, rucksack, expedition_pack}`; `item.face.{respirator, mask, goggles}`; the 12 `item.potion.*`, 10 `item.text.*`, 16 `item.throwable.*`, 3 `item.food.*`, 9 `item.tool.*`, 3 `item.book.*` of §3.5; `item.container.flask`; `item.bag.{boss, editor, lucky, elite, refined, burnt}`; `item.key.{bound, gilded, illumined}`; `item.lucre`; `item.quest.catchword`; 24 `item.exotic.*` (§8.2); 10 `item.palimpsestic.*` (§8.3) |
| `affix.` | 116 IDs of §6.2 |
| `perk.` | 51 column perks (§7.2), 16 `perk.genre.<genre_suffix>`, 26 `perk.leg.*` (§8.1) |
| `mfr.` | 40 IDs of §5.2: the 27 canon-1.3 IDs plus `mfr.<core genre>.{runecourt, blackmoor, dunwold, starwright, chromaw, geigerhaus, sundown_rail, halloran, cloudpiercer, ironclad, glass_cobbler, delphic}` (one per core genre) and `mfr.errata.marginal_press` |
| `loot.` | `loot.drop.{minion, standard, elite, champion, boss, editor, hunter}`; `loot.chest.{plain, locked}`; `loot.floor`; `loot.trophy`; `loot.secret.{cache, vault, palimpsest}`; `loot.volume_reward`; `loot.keepsake.<genre_suffix>` (16); `loot.reliq.{plain, bound, gilded, illumined}`; `loot.bag.{boss, editor, lucky, elite, refined, burnt}`; `loot.shop.{colophon, general, armory, apothecary, curio, fence, wanderer}`; `loot.engine.{brass, silver, gilt, armory, wardrobe, curio}`; pool tags `loot.pool.{exotic_arms, exotic_armor, legendary_perks_1, legendary_perks_2, cross_glosses, implants_2, relics_2, throwables_2, glyphs_2, recipes_2, brews_2, gadgets_2, fortune_engines_2}` |
| `reliq.` | `plain`, `bound`, `gilded`, `illumined` |
| `engine.` | `brass`, `silver`, `gilt`, `armory`, `wardrobe`, `curio` |
| `shop.` | `colophon`, `general`, `armory`, `apothecary`, `curio`, `fence`, `wanderer` |

Data constants: `party_drop_bp`, `shared_drop_bp`, deck counts, role bias, tier gates and weights, Engine face tables, `LUCRE_TARGET(d)`, `INV_SLOTS_BASE = 20`, base values of §3.

## Interface assumptions

| Doc | This chapter assumes it provides |
|---|---|
| 03 | block hardness and block damage for `perk.leg.worldbreaker`; ground-item entities |
| 04 | effective sub-attribute modifiers from gear; food satiation; composure-loss hook (× factors); Speech XP for haggling; `aug.*` implant templates; doc 14's kit IDs |
| 05 | the 12 `arch.melee.*` and 14 `arch.ranged.*` IDs of §3.2 (doc 14's IDs adopted), 15 `arch.armor.*`, `arch.offhand.{shield_small, shield_medium, shield_tower, focus, lantern}`, `arch.ward.{light, standard, heavy}` with `cap_base` 20, 35, 55 HP; per-archetype stats, pierce share, `magazine` and `ammo` flags, throw action; `ammo.{light, heavy, explosive, fuel, stone}`; counter semantics of Canon §17.3a |
| 06 | wand glyph types for `arch.ranged.wand`; spell source flag, cast ticks and aether cost hooks; text effects |
| 07 | per (genre, material role) material pools with `value_bp`, `dmg_bp`, `ar_bp`; `equivalent(material, genre)`; mass and durability; the 22 `part.*` IDs of §5.4 with those modifiers; the 6 gadgets of §3.6 plus 6 for `loot.pool.gadgets_2`; salvage units |
| 08 | potion and flask effects; ingredient templates for apothecaries and `loot.trophy` |
| 09 | noise in dB, lock difficulty for `loot.chest.locked`, mimic as a hidden object, `aware.suspicious` |
| 10 | `roll_curse(rng, category, local MYT)`, knowledge flags, appraisal |
| 12 | spots for chests (`4 + range(3)`), Reliquaries, Engines, shops, Scriptoriums and Volume-plan caches; secret kinds; at most one Palimpsest vault per Volume; Colophon and Endpaper rooms |
| 13 | `loot_role` and `no_loot` template fields, `enemy.mimic`, tags undead and fiend |
| 14 | `RunState.grade`, `GradeParams`, Folio positions, Volume reward phase, Bindery pool tags |
| 15 | `generate_keepsake` caller and Satchel |
| 16, 17, 18 | intents transport; odds panels, ledger and Engine UI; skin and vendor localization |

## Canon gaps

1. **Canon version.** The contract line names canon-1.2, whose §5.9 was missing (canon-1.3 changelog). This chapter uses canon-1.3 §5.9 manufacturer IDs and traits, §5.8 seam zones, §5.10 World Laws, §5.11 `st.unmoored` and the §17.1 Colophon, and conflicts with neither version otherwise.
2. **Marginalia enhancement.** Canon §13.9 types `enhancement` as `0..=15` and §13.7 clamps `eff_enh` at 15, yet the Marginalia cap exceeds 15. Assumed: the field range and the upper clamp both equal the current cap `15 + floor_div(d_current - 25, 5)`.
3. **Multiplicative traits.** Canon §5.9 states some manufacturer traits as "× N" (Perihelion, Rainslick, Thimbleweald) but §10.5 has no slot. Assumed: one `mul_bp` on the hit's `B` after the §10.4 chain, before step 1.
4. **Stream uses.** Keyed `rng.loot` also draws for bag contents and burning, salvage parts, Quill replacements and mimic loot; per-shot ammo and stamina savings (Lastditch, `affix.frugal`) use keyed `rng.combat`, which Canon §3.3 lists for on-hit procs. Proposed: add these uses to the §3.3 table.
5. **Offer contexts.** Canon §3.5 has one offer stream per slot. Assumed: `offer_rng` mixes a non-advancing peek with the context uid and purpose, and each commit advances the stream by exactly one `next_u64`.
6. **"Reels".** Canon §3.3 says "Fortune Engine reels". Engines have no reels; the term means the face pick. Proposed wording: "Fortune Engine dispensing".
7. **Unwritten Colophon.** The canon-1.3 Colophon holds a shop, but `genre.unwritten` has none (§5.2). Assumed: no shop there; the Scriptorium station stays.
8. **Unwritten counters.** Canon §17.3a requires counter sources in every genre's shop; Unwritten has no shop, so its source is the loot cache of §9.10.
9. **Emendation category.** `cat.text` is type-hidden (Canon §13.6), so `item.tool.emendation` is `cat.tool`.
10. **`errata_grade`** in Canon §13.9 is read as doc 14 `RunState.grade`.
11. **Unmoored Serendipity.** "Serendipity procs" (canon-1.3 §5.11) is read as SER 40 in §13.4 and an Engine SER shift of 0.
12. **Purses.** The canon does not say whether Lucre is per player; this chapter makes it per player.

## Acceptance tests

1. **Kits.** At ilvl 1 with §3 values, every doc 14 kit plus the universal kit (94) sums to: Vanguard 386, Duelist 382, Gunslinger 406, Grenadier 434, Spellwright 434, Alchemist 374, Shade 368, Artificer 430, Warden 398, Oracle 366; alt kits Vanguard 362, Duelist 362, Gunslinger 382, Grenadier 438, Spellwright 418, Alchemist 398, Shade 414, Artificer 402, Warden 398, Oracle 382. All in 360..=440.
2. **Worked example.** The §20 inputs yield keyed seed `0x83db3ac3a7065305`, line draws 5808, 1593, 8978, 1204, item seed `0xe5bf711b4609c44a`, 27 Lucre, and the §2.2 instance exactly (sidearm, Epic, Excellent, Sundown Rail, hexed silver, the three affixes with rolls 4095, 4630, 1146, the eight perk options, value 1123).
3. **Sub-seed.** `fnv1a64("loot.cat") = 0x3418799fa9ecd0ed`; the §20 category sub-generator seed is `0x59f6955537f06994`.
4. **Step isolation.** Adding a draw to `loot.affix` changes no output of any other tag.
5. **Keyed isolation.** The §20 drop is identical whether 0 or 50 other entities died earlier in tick 55606.
6. **Rarity shift.** `d = 7`, SER 52, shift 2000: `[7000, 3432, 1344, 456, 178, 74, 0]`. With `st.unmoored` the SER term uses 40.
7. **Party scaling.** `party_drop_bp = [10000, 8000, 6666, 5714]`; `shared_drop_bp = [10000, 12800, 16000, 18286]`; a boss line stays 10000 at every party size.
8. **Bad luck.** `blp_rare = 10` multiplies Rare-or-better weights by 15000; `blp_rare = 24` sets the floor to Rare; `blp_leg = 120` sets it to Legendary; a forced-rarity Engine face changes neither counter.
9. **Deck.** 40 cards; 40 draws with no role bias empty the deck exactly once; the 41st draw follows a refill.
10. **Volume plan.** For run seed `0x0123456789ABCDEF`, Volume 1 uses `rng.loot_place` Folio 1 (stream seed `0x7e0906f1cb05c409`): `u = [4, 0, 1, 1]`, Emendations at Folios 2, 3 and 5 (the last in the Editor bag), Quill at Folio 4, bound keys at Folios 3 and 1, gilded key at Folio 4. Volume 2 (stream seed `0xc428e5e1c888620a`): Emendations at Folios 6, 9 and 10, Quill at Folio 7. At Grade 12 only the first bound key is placed.
11. **Emendation spread.** Exhaustive enumeration of the 120 `(u1..u4)` tuples gives 0.6 expected Emendations at each of the 5 positions and exactly 3 per Volume.
12. **Affix values.** Tier 2 `affix.dmg_vs_beast` at 1146 → 1880; tier 2 `affix.self_repair` at 4095 → 818; tier 2 `affix.infuse_fire` at 4630 → 1731 and 24 status power.
13. **Affix legality.** No generated item in `genre.post_apocalypse` or `genre.hardboiled_noir` carries an arcana component; none in `genre.redacted` a verve component; every `genre.unwritten` item has only dependency-none components.
14. **Skins.** The four §4.2 tables have 416 cells; validation rejects any tagged axis whose axiom is 0; `arch.ranged.staff` and `arch.ranged.wand` are never generated in `genre.unwritten`.
15. **Manufacturers.** Each core genre's three manufacturers together make all ten classes; a Weird West sidearm's candidates total 10000 with Marginal Press first.
16. **Gloss.** A Coffin Creek Gloss on a Wuxia bow has `Req[verve] = 5`.
17. **Reliquary odds.** A plain Reliquary's equipment slot at Folio 7, SER 40 shows 68.2854%, 22.2278%, 6.4581%, 2.2028%, 0.8260%; a gilded Reliquary's Rare-floor slot at Folio 7 has weights `[0, 0, 1420, 490, 193, 82, 0]`.
18. **Reliquary pity.** With `rq5 = 4` the next Reliquary's best slot cannot roll below Epic; with `rq20 = 19` not below Legendary.
19. **Mimic.** Opening a mimic keeps the key and spawns `enemy.mimic`; after its death every player opens their instance without a key.
20. **Engine odds and prices.** Brass at SER 70: `[2300, 2600, 1800, 1800, 900, 600]`; pity 4 (3 with `item.trinket.wishbone`): `[0, 0, 0, 0, 0, 10000]`. Folio 7 prices: brass 100, 125, 150, 175, 200; silver 268, 335, 402, 469, 536; a sixth purchase is refused.
21. **Engine commitment.** Reopening shows identical faces; an open advances `rng.offer.p<slot>` by 0 and a purchase by exactly 1; every accepted payment yields exactly one item; the Engine schema has no stake, multiplier or empty-outcome field.
22. **Shop prices.** Value 1123: sell 336 (CHM 40), 168 unappraised, armory 1965; Speech 40, CHM 70, Folio 7 gives `haggle_bp = 1900` and price 1448.
23. **Reforge and Rewrite.** Epic at Folio 9: reforges cost 472, 1416, 3304, 7080 and a fifth is refused; Rewrite costs 393 for value 1123; a Rewrite keeps every "Persists" field of §15.
24. **Lucre.** `LUCRE_TARGET` rows of §13.2 match; Σ over Folios 1 to 25 is 38,102.
25. **IP.** Common Normal +0 iron ilvl 1: 100; §20 sidearm: 237; maximum ilvl-25 chain: 4103.
26. **Lost Satchel.** With 2048 ground items, inserting one more moves items in ascending uid (skipping quest items, corpses and items within 8000 mm of an Erratum) until the count is at most 1984.
27. **Stacking.** Two `item.potion.healing` with different `buc` never merge; equal ones merge into the lower uid.
28. **Marginalia Emendations.** Exiting Folio 26 grants 1 Emendation to each Active and Ghost slot; the cap at Folio 30 is 16.
29. **Counters.** In each of the 16 genres a 4-player Folio whose exit preview shows `elite.warded` places a cache holding 2 `item.throwable.breaker_charge` per player in the next Folio.
30. **IDs.** Every exported ID passes the grammar with distinct `fnv1a32`, including against canon IDs.
31. **Folio isolation.** Shop shelves, floor loot and Engine placements of Folio 7 are byte-identical for party sizes 1 to 4 and whether Folios 1 to 6 were played.
32. **Uniques.** No party holds two copies of a Palimpsestic unique; a player receives no Exotic twice before that category's Exotics are exhausted.

## Invariants annex

```
Σ doc11: template ⊗ instance ⊗ keyed loot ⊗ deck ⊗ pity ⊗ Lucre-only containers
W! every run item ← generate_item(seed) ∨ kit                     N! item seed reused across lines
W! step draws ⊂ sub(seed, tag)                                      N! one step's draws shift another step
W! drop rng := keyed(rng.loot, folio, actor, tick, slot)          W! 2 draws per loot line, hit or miss
W! weighted lists ∈ ascending content ID ∨ fixed table order       N! HashMap iteration in loot
W! rarity := Canon §13.4 → shift → pity → floor                   N! rarity share changed by pool unlocks
W! component dep ∉ {axis | origin axiom = 0}                       W! staff, wand dep := aether axiom
W! Fortune Engine: pay Lucre → exactly 1 item from shown faces     N! wager | multiplier | empty outcome | reels
W! displayed odds == rolled weights                                N! paid currency → any random container
W! offer generated once, advanced only on commit                   N! reroll on reopen
W! Palimpsestic ∈ {editor bag, volume reward, palimpsest vault}    N! Palimpsestic sold | salvaged | on faces
W! Exotic ≤ 1 per canon group                                      N! duplicate Exotic before category exhausted
W! enhancement ≤ cap(d)                                            W! Emendation re-levels ilvl first
W! Lucre per player, i64                                           N! Lucre → account currency
W! party loot scaled by party_drop_bp on scaled lines only         N! boss guarantee scaled by party size
W! Folio loot generation ⊥ party size ∧ earlier play               W! ground count ≤ GROUND_ITEM_CAP via Lost Satchel
W! Fortune|Serendipity → in-run loot only                          N! SER effect while st.unmoored
```
