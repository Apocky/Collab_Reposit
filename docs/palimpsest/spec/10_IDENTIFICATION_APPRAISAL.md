# 10 — Identification and Appraisal

As of 2026-10-11. Status: BINDING. Conforms to canon-1.4.

This chapter owns item knowledge and how it grows, item value, curses and blessings, `cat.wand` and its type-hiding rule, sealed Reliquary contents, and the identification side of the Index and Bestiary.

## Imports

| Source | Consumed |
|---|---|
| Canon §1.1, §1.3, §1.5, §1.6, §1.10, §1.11 | ID grammar and `id32`, `floor_div`, `ceil_div`, `mul_bp`, display units, integer content, text keys, `uid` layout and ascending-uid iteration |
| Canon §2.2, §2.3 | the Needs pass (period 60, phase 7), `in_combat`, the "N s = N × 60 ticks" rule |
| Canon §3.1–§3.5 | `Rng`, `stream_seed`, `keyed_rng`, streams `rng.appearance` and `rng.ident` (owner 10) and `rng.loot` (owner 11), draw-count rule |
| Canon §5.3–§5.8 | dependency sets, `Req`, `Tol`, Contradiction potency, the MYTHOS curse-chance term, local axioms |
| Canon §6.4 rows 17, 28, 36, 37, 38, 42, 44; §7; §11.8 | use speed, XP multiplier, curse sense, buy and sell multipliers, rarity shift, curse resistance; `skill.appraisal`, `skill.divination`, mastery tiers; player sleep scaling |
| Canon §4.3, §9.1, §13.1–§13.9, §14, §16.4, §16.5, §17.1, §17.4, §17.5, §17.6, §17.9 | `cat.placeable`; severing and hand bindings; categories, slots, rarity, quality, knowledge and curse enums, `idc` classes, the shuffle, Emendations and stored enhancement, `P(d)`, item contract, `illuminated`; Lucre; cure price; patrons and favor; Volumes and the Colophon; reference anchors; loot modes; ghost item loss; invaders and Redaction |
| Canon §18, §19 | `ev.item_identified`, `ev.item_picked_up`, `ev.item_used`, `ev.entity_killed`, `ev.folio_entered`, `ev.folio_exited`, `ev.run_ended`; ownership of `know.`, `idc.`, `appr.`, `buc.`, `curse.` |
| Doc 03 §2.2, §14.2 | system order (`sys.needs` is system 27, due on ticks `t ≡ 53 (mod 60)`; `sys.actions` is system 7); event payload schemas, including `ev.item_identified` and `ev.item_picked_up` |
| Doc 04 §2, §4, §5.3, §7, §9 | Origin trait `gumshoe`; `anl_speed_bp`; Appraisal mastery perks and appraisal XP rows; talents `prophecy` and `avatar_of_fate`; augment install as an equip |
| Doc 06 §3.2, §8.3, §8.4, §16 | quick-cast wheel; text reading; wand charges, zap, recharge and snap; prayer needs, `prayer.absolution` and the Cosmic Horror patron boon |
| Doc 08 §12.1–§12.4, §15.3 | the 29 `alch.potion.*` types and their base values; drink and throw identification; Alchemy hooks; `alch.tx.uncurse`, `alch.tx.consecrate` |
| Doc 11 §2.1–§2.3, §3.1–§3.5, §10–§14 | template and instance records (`base_value`, `sealed`), the generation pipeline and its step 12 call, accessories, consumables and their values, in-run Reliquaries, Fortune Engines, shops, prices, services, stored `value` (§13.1), Lucre targets (§13.2), stacking (§14.2) |
| Doc 12, 13, 14, 15, 16 | Scriptoria, altars and story relics; monster item use and carried items; kits, Calling lore, boons, modes and Codex knowledge (§5.5); Keepsakes, the Satchel and disclosure (§5); `ItemView` transport |

Docs 03, 04, 05, 06, 08, 11, 12, 13, 14, 15 and 16 exist, and this chapter cites their sections inline. Docs 07, 09, 17, 18 and 19 do not exist yet; what this chapter needs from them is under Interface assumptions.

---

## 1. Design rules

1. **Hide magic, not mundane type.** A dropped rifle reads "Rifle". Only the seven `idc` classes hide their type (Canon §13.6). Everything else hides only rarity, enhancement, curse state, affixes and charges.
2. **Hidden properties are always active.** An unknown curse binds, an unknown +3 hits harder, an unknown ring works. Knowledge changes only what the UI shows and what vendors pay.
3. **No displayed chance, time or price depends on a hidden property**, except that vendors know true types (price identification, §5.8). Functions computing displayed numbers take an `ItemView` (the knowledge-filtered view, §2.8), never an `Item`. Every price and cost that scales with item value uses `value_vendor` (§6.2), never the stored `value`.
4. **Knowledge is party-shared.** Type knowledge is one run-scoped table for the whole party. Instance knowledge lives on the item and travels with it.
5. **Every method is exact:** trigger, reveal set, stream, draw count and cost.
6. **Meta knowledge is horizontal.** No run's knowledge identifies another run's appearances; the Index (§12) adds information and one limited deduction aid.

---

## 2. Knowledge model

### 2.1 Category property table

`H(item)` is the set of hidden-property kinds of the instance's category:

| Category | Type-hidden | RARITY | BUC | ENH | CHARGES | Affix slots |
|---|---|---|---|---|---|---|
| `cat.weapon_melee`, `cat.weapon_ranged`, `cat.armor`, `cat.offhand`, `cat.ward_gen` | no | yes | yes | yes | no | yes |
| `cat.accessory`, ring, neck or trinket slot | yes (`idc.ring`, `idc.amulet`, `idc.relic`) | yes | yes | yes | no | yes |
| `cat.accessory`, belt or back slot | no | yes | yes | yes | no | yes |
| `cat.implant` | yes (`idc.implant`) | yes | yes | yes | no | yes |
| `cat.wand` | yes (`idc.wand`) | yes | yes | yes | yes | yes |
| `cat.potion` | yes (`idc.potion`) | yes, set at creation (§2.6) | yes | no | no | no |
| `cat.text` | yes (`idc.text`) | yes, set at creation (§2.6) | yes | no | no | no |
| `cat.mount`, `cat.part` | no | yes | no | no | no | yes |
| every other category (**mundane categories**) | no | no | no | no | no | no |

- **Mundane categories** (`cat.ammo`, `cat.book`, `cat.food`, `cat.throwable`, `cat.gadget`, `cat.tool`, `cat.material`, `cat.ingredient`, `cat.container`, `cat.placeable`, `cat.key`, `cat.currency`, `cat.quest`, `cat.seed`, `cat.corpse`, `cat.body_part`) start and stay at `know.classified` (Canon §13.6), which for them is complete: exact value shown, no "?" marks, never a target, always `buc.uncursed`. The table covers all 28 Canon §13.1 categories.
- **Affix slots** are the entries of the instance's `affixes` list in list order, at most 8. A curse affix is always the last entry (§7.1). A Legendary perk and an Exotic intrinsic are not affix slots; they are revealed with RARITY.
- Quality, material, manufacturer, `ilvl`, `origin_genre`, `gear_tier`, mass and stored `enhancement` are never hidden.
- **Enhancement.** Every item enters the world with stored `enhancement = 0`: Canon §13.7 makes stored enhancement Emendation-driven, and no generator sets it (doc 11 §2.3 has no enhancement step, doc 14 §2.4 kits and doc 15 §5.4 Keepsakes are created at 0). Every later change is an Emendation, which is a visible party action. Stored `enhancement` is therefore public. The hidden part of an item's displayed enhancement `eff_enh` (Canon §13.7) is `eff_enh − enhancement`: +1 when blessed, `−curse_penalty` when cursed. `ENH` therefore means "`eff_enh` known" and is set exactly when `BUC` is set (§2.3). The "hidden +N" of Shattered Pixel Dungeon is this term.
- **Rarity of consumables.** `cat.potion` and `cat.text` are never rarity-eligible (doc 11 §2.1, §3.5: always `rar.common`), so revealing their rarity would reveal nothing. Their `RARITY` flag is set at creation.

### 2.2 Type knowledge (run-scoped, party-shared)

```rust
pub struct TypeKnowledge {                         // one per run, host-authoritative, saved
    pub assign:   BTreeMap<Id32, Id32>,            // type key -> descriptor (§3.5), fixed at run start
    pub by_desc:  BTreeMap<Id32, Id32>,            // descriptor -> type key, inverse of assign
    pub known:    BTreeSet<Id32>,                  // identified type keys
    pub seen:     BTreeSet<Id32>,                  // descriptors the party has encountered (§5.3)
    pub tried_no_effect: BTreeSet<Id32>,           // descriptors used once with no observable outcome
    pub read_mode: BTreeMap<Id32, ReadMode>,       // texts only: Targeted | Untargeted, from the read prompt
    pub refused_cats: BTreeMap<Id32, u32>,         // texts only: bitmask of Canon §13.1 row indices a targeted text refused
    pub price_obs: BTreeMap<Id32, Vec<PriceObs>>,  // ≤ 8 per descriptor, oldest dropped first (§5.8)
    pub excluded: BTreeMap<Id32, BTreeSet<Id32>>,  // descriptor -> type keys proven wrong
    pub xref_volume: [u8; 4],                      // per party slot: Volume of the last Cross-Reference (§12.3), 0 = none
}
```

- **Type key.** `type_key(t) = t.base_type.unwrap_or(t.id)`. A unique template in a type-hidden class (an Exotic ring, a Palimpsestic amulet) names a non-unique `base_type` of the same class, shows that type's appearance and shares its type knowledge. Its unique name appears when RARITY becomes known.
- `known` only grows, except through `appr.eff.amnesia` (§5.4).

### 2.3 Instance knowledge

Layout of the Canon §13.9 `knowledge` field:

```rust
pub struct ItemKnowledge {
    pub state: KnowState,        // cache of derive_state(); recomputed on every change
    pub flags: u32,              // bit layout below
    pub appr_progress: i64,      // milli-work toward the next appraisal completion (§5.5)
    pub appr_tier_done: i8,      // highest appraisal tier applied, -1 = none
    pub est_skill: i16,          // best appraisal skill recorded by a reveal set (§5.5), 0 until one; feeds §6.3
    pub est_key: u64,            // uid at creation; copied on split, lower kept on merge; keys the §6.3 offset
    pub est_steps: u8,           // Analysis narrowing steps, 0..=4
    pub fam_ticks: u32,          // familiarity time (§5.7)
    pub fam_kills: u16,          // familiarity kills (§5.7)
    pub analysis_lvl: [i16; 4],  // per party slot: character level at the last Analysis or psychometry, -1 = never
}
```

| Bits | Flag | Meaning |
|---|---|---|
| 0 | `RARITY` | rarity known; also unlocks the value estimate, perk columns, Legendary perk and Exotic intrinsic |
| 1 | `BUC` | curse state and `curse_penalty` known |
| 2 | `ENH` | effective enhancement `eff_enh` known; set exactly when `BUC` is set (§2.1) |
| 3 | `CHARGES` | current and max charges known |
| 4 | `OMEN` | known to be cursed; penalty unknown |
| 5 | `CLEAN` | known not to be cursed; blessed versus uncursed unknown |
| 6 | `AURA` | magic class known: mundane, storied or ominous (§5.10) |
| 7 | reserved | always 0 |
| 8..=15 | `AFFIX_NAME[i]` | identity of affix slot `i` |
| 16..=23 | `AFFIX_ROLL[i]` | roll of affix slot `i` (for a curse affix, its severity) |
| 24..=27 | `SENSED[s]` | party slot `s` has already made its curse-sense roll (§5.11) |
| 28..=31 | reserved | always 0 |

Setting `BUC` also sets `ENH`, and sets `OMEN` when cursed, else `CLEAN`. `ENH` is never set without `BUC`. Removing affix slot `i` clears its two bits; because only the last slot (the curse) is ever removed, no shift is needed. Uncursing (§7.6) sets `BUC`, `ENH` and `CLEAN` and clears `OMEN`.

**Required flags.**

```
required(item) = (RARITY if H has RARITY) | (BUC if H has BUC) | (ENH if H has ENH)
               | (CHARGES if H has CHARGES)
               | Σ over affix slots i: (AFFIX_NAME[i] | AFFIX_ROLL[i])
```

### 2.4 Derived state and state machine

```rust
fn derive_state(it: &Item, tk: &TypeKnowledge) -> KnowState {
    if is_mundane(it.cat) { return Classified; }                           // terminal for mundane categories
    if type_hidden(it) && !tk.known.contains(&type_key(it)) { return Unknown; }
    let req = required(it);
    if it.knowledge.flags & req == req { return Identified; }
    if it.knowledge.flags & RARITY != 0 { return Appraised; }
    Classified
}
```

Instance flags persist while the type is unknown, so an unknown-type item with `BUC` set still shows `know.unknown`. Because potions and texts have `RARITY` set at creation (§2.1), a potion or text of known type is never `know.classified`: it is `know.appraised` until `BUC` is known, then `know.identified`. Other chapters' phrase "learns the type to `know.classified`" (doc 08 §12.4, doc 13 monster item use) means `identify_type` (§5).

The machine is closed: every state and trigger is listed, and no other transition exists.

| From | To | Trigger |
|---|---|---|
| `know.unknown` | `know.unknown` | instance flags set while the type is still unknown (familiarity, Analysis, curse sense, binding) |
| `know.unknown` | classified, appraised or identified, as its flags support | type identified (identify effects also set every required flag) |
| `know.classified` | `know.appraised` | RARITY revealed: appraisal tier 0, familiarity stage F1, Analysis, `appr.eff.survey`, shop stock, or creation as a sealed item (§11) |
| `know.classified` or `know.appraised` | `know.identified` | the last missing required flag is revealed |
| `know.identified` | `know.identified` | a party action changes a property (Emendation, consecration, recharge, reforge, Rewrite); the new value is shown, so its flag is set |
| any of classified, appraised, identified | `know.unknown` | `appr.eff.amnesia` forgets the type (type-hidden only); instance flags are kept |

A property changed by an unseen source (none at launch) clears its flag. Nothing else lowers a state.

### 2.5 Sharing, stacks, trading and joining

- Knowledge travels with the item through drops and trades, identically in instanced loot and `mode.loot_shared` (Canon §17.5).
- Stacking is doc 11 §14.2: stacks merge only when template, `origin_genre`, `buc`, knowledge state and flags, `quality` and `ilvl` are equal and neither has affixes, so items with different knowledge never merge (as in NetHack). The merged stack keeps the lower `uid` (doc 11) and takes, field by field, the larger `appr_progress`, `appr_tier_done`, `est_skill` and `est_steps`, the element-wise larger `analysis_lvl`, and the lower `est_key`. Splitting takes a new `uid` (doc 11) and copies the whole `ItemKnowledge`, `est_key` included, so a split stack shows the same estimate (§6.3) and cannot be analyzed again at the same level (§5.6).
- Type identification updates every instance of that type in the run at once, on the ground or carried.
- A player who joins mid-run shares `TypeKnowledge` from the join tick. A ghost (Canon §17.6) cannot pick up, appraise or analyze, but keeps seeing party knowledge. An invader (Canon §17.9) is not in the party: it never receives party knowledge, and its Editor kit is fully identified.

### 2.6 Starting knowledge

- Every item is created with `est_key = uid`, `est_skill = 0`, `est_steps = 0`, `appr_tier_done = -1`, `analysis_lvl = [-1; 4]` and its other counters at 0.
- Every `cat.potion` and `cat.text` item is created with `RARITY` set (§2.1).
- Kit items (doc 04, doc 14 §2.4) are created with every required flag set. After the shuffle, each Calling's two lore types (doc 14 §2.4.3) and every type-hidden kit item's type join `known` with reason `appr.reason.kit`.
- Crafted items (doc 07), brewed potions (doc 08) and texts inscribed by a party member (doc 06) are created with every required flag set and their type known. Brewing a canonical type identifies it (reason `appr.reason.brew`, called by doc 08).
- Shop stock (§10.1) arrives with every required flag set. Fortune Engine faces, Volume rewards (doc 11 §9.8, §11.2: `know.appraised`) and sealed items (§11) arrive with `RARITY` set, so the outcome tier of every random container is visible.
- NPC-carried items that drop on a corpse (doc 13 §5.1: `rar.common`, `q.normal`, `buc.uncursed`) are created with no flag set beyond §2.1's creation rules, like any found item.

### 2.7 What persists across runs

Only the Index and Bestiary (§12) persist: templates and types seen and identified, familiarity points and ranks, bestiary counts, curse affixes revealed and illuminated skins (Canon §13.9). The appearance mapping, type knowledge, the Ledger, instance flags and all progress counters die with the run.

### 2.8 Replication

The host owns all knowledge state. Doc 16 sends each client only the `ItemView` of every item: properties whose flags are unset are omitted from packets, not merely hidden in the UI. Type-hidden items are sent with their descriptor ID and without their template ID until the type is known.

```rust
pub struct ItemView {                       // built by the host from (Item, TypeKnowledge); the only item type UI and pricing code may read
    pub uid: u64,
    pub cat: Id32,
    pub state: KnowState,                   // derive_state()
    pub sealed: bool,                       // §11: while true, every Option below is None and descriptor is None
    pub descriptor: Option<Id32>,           // type-hidden items only
    pub template: Option<Id32>,             // None while type-hidden and unknown
    pub origin_genre: Id32, pub quality: Id32, pub ilvl: i32, pub enhancement: i32,   // public (§2.1)
    pub materials: Vec<Id32>, pub manufacturer: Option<Id32>, pub gear_tier: i8, pub mass_g: i64,
    pub rarity: Option<Id32>,               // RARITY
    pub buc: BucView,                       // Unknown | Omen | Clean | Known { buc, curse_penalty }
    pub eff_enh: Option<i32>,               // ENH
    pub charges: Option<(i32, i32)>,        // CHARGES: (cur, max)
    pub affixes: Vec<(Option<Id32>, Option<i32>)>,   // per slot: AFFIX_NAME[i] -> id, AFFIX_ROLL[i] -> roll_bp (curse slot: penalty)
    pub n_affix_slots: u8,                  // visible once RARITY is known (rarity fixes the count, Canon §13.3); 0 shown as "?" before
    pub est: ValueLine,                     // §6.3: Unknown | Range { lo, hi } | Exact(i64)
}
```

Mundane items fill every field. `n_affix_slots` counts the curse slot only once `BUC` is known, so it never reveals a curse.

---

## 3. Appearance pools, rendering and the shuffle

### 3.1 The type-hiding rule (Canon §13.6, owned here in full)

```rust
fn idc_of(t: &Template) -> Option<IdcClass> {
    match t.cat {
        cat.potion  => Some(idc.potion),
        cat.text    => Some(idc.text),
        cat.wand    => Some(idc.wand),
        cat.implant => Some(idc.implant),
        cat.accessory => {
            let s = &t.equip_slots;                       // non-empty, validated
            if s.iter().all(|x| x == slot.ring_l || x == slot.ring_r) { Some(idc.ring) }
            else if s == [slot.neck]    { Some(idc.amulet) }
            else if s == [slot.trinket] { Some(idc.relic) }
            else { None }                                 // belts, back items
        }
        _ => None,
    }
}
fn type_hidden(it: &Item) -> bool { idc_of(template(it)).is_some() }
```

- An accessory whose `equip_slots` mixes a hidden group with anything else is a content error (`idc_slot_mixed`).
- Staves are `cat.weapon_ranged` and foci are `cat.offhand`, so neither is type-hidden. Gadgets with charges are `cat.gadget` and are mundane.
- A unique template in a type-hidden class must name a `base_type` in the same class (`idc_base_type_missing` otherwise).

### 3.2 Pool sizes and type budgets

| Class | Pool size | Max types (`pool − 2`) | Launch content (§13) |
|---|---|---|---|
| `idc.potion` | 32 | 30 | 29 `alch.potion.*` (doc 08) |
| `idc.text` | 28 | 26 | 10 `item.text.*` (doc 11 §3.5) plus 5 proposed doc-10 texts (§5.4) |
| `idc.ring` | 26 | 24 | 15 `item.ring.*` (doc 11 §3.4) |
| `idc.amulet` | 18 | 16 | 8 `item.amulet.*` (doc 11 §3.4) |
| `idc.wand` | 18 | 16 | `wand_pool` spells (§4.2) |
| `idc.implant` | 32 | 30 | 27 implant-form `aug.*` (doc 04 §9) |
| `idc.relic` | 22 | 20 | 18 `item.trinket.*` (doc 11 §3.4) |

At least two descriptors per class are always unassigned. These **decoys** keep elimination honest: identifying every type but one never reveals which appearance the last type wears. A mod that adds types must add descriptors so that `|types| ≤ |pool| − 2` holds, or the content set is rejected with `idc_pool_exhausted`. Descriptor IDs are `idc.<class>.<name>`.

### 3.3 Descriptor lists

**Potions (32).** Suffix and render color, in sorted order (index 0 to 31): `amber` #FFB000, `azure` #3A8DFF, `bistre` #3D2B1F, `black` #111111, `brick` #A23B2C, `charcoal` #36454F, `chartreuse` #7FFF00, `clear` #E8F4F8, `cobalt` #0047AB, `crimson` #DC143C, `golden` #FFD700, `indigo` #4B0082, `ivory` #FFFFF0, `lilac` #C8A2C8, `lime` #BFFF00, `magenta` #FF00FF, `maroon` #800000, `mint` #98FF98, `murky_green` #556B2F, `ochre` #CC7722, `orange` #FF7F00, `pink` #FF8FB1, `plum` #8E4585, `quicksilver` #D9D9D9, `rose` #FF007F, `saffron` #F4C430, `scarlet` #FF2400, `sky_blue` #87CEEB, `smoky` #738276, `teal` #008080, `turquoise` #30D5C8, `violet` #8F00FF. Accessibility cue: with `i` the sorted index, stopper shape = `i % 4` (round, square, spire, wax seal) and label mark = `i / 4` (dot, bar, cross, ring, chevron, star, wave, diamond), so no two potions share a shape-and-mark pair and color is never the only signal.

**Text sigils (28).** Each is a nonsense incipit drawn from a committed 28-glyph sigil atlas (doc 18): `ambreth`, `ankaru`, `balux`, `belleth`, `corvent`, `drema`, `elixtor`, `fennick`, `gormal`, `hesper`, `illoq`, `jarrow`, `keste`, `lumir`, `mordant`, `nulzeph`, `obrax`, `pellin`, `quorval`, `runnel`, `sabeth`, `tirrex`, `ulma`, `vorne`, `wexel`, `xanth`, `yrrel`, `zolmet`.

**Ring stones and metals (26).** Stones (20): `agate`, `amethyst`, `bloodstone`, `carnelian`, `citrine`, `diamond`, `emerald`, `garnet`, `jade`, `jet`, `lapis`, `malachite`, `moonstone`, `obsidian`, `onyx`, `opal`, `peridot`, `ruby`, `sapphire`, `topaz`. Bare metal bands (6): `black_iron`, `bronze`, `electrum`, `red_gold`, `twisted_silver`, `white_gold`. A stone ring renders the stone in a plain setting whose metal is the item's primary material (doc 07); a metal band renders that metal and no stone.

**Amulet forms (18).** `acorn`, `arrowhead`, `bell`, `cage`, `coin`, `crescent`, `disc`, `fang`, `feather`, `heart`, `key`, `knot`, `lozenge`, `shell`, `spindle`, `star`, `teardrop`, `wheel`.

**Wand woods and alloys (18).** `ash`, `bamboo`, `birch`, `bone`, `brass`, `cedar`, `copper`, `coral`, `ebony`, `glass`, `horn`, `oak`, `pewter`, `rowan`, `silver`, `steel`, `willow`, `yew`.

**Implant stamps (32).** `anvil`, `barbed_ring`, `bent_nail`, `bird_foot`, `broken_wheel`, `chain_link`, `cog_tooth`, `coiled_serpent`, `comb`, `crossed_keys`, `crown_dot`, `double_bar`, `fishhook`, `half_sun`, `hollow_triangle`, `horseshoe`, `hourglass`, `interlock`, `keyhole`, `ladder`, `lantern`, `lightning_fork`, `needle_eye`, `open_eye`, `paw_print`, `quill_mark`, `seven_dots`, `shield_notch`, `spiral`, `star_burst`, `twin_moons`, `wave_line`. Stamps are fictional marks and never match a manufacturer (`mfr.`) mark or a real brand (doc 18 name review).

**Relic carvings (22).** `antler`, `bee`, `boar`, `crane`, `fish`, `gate`, `hand`, `horned_mask`, `knotwork`, `labyrinth`, `laurel`, `lotus`, `moth`, `oak_leaf`, `owl`, `rams_head`, `ship`, `skull`, `stag`, `sun_disc`, `tower`, `weeping_face`.

No suffix repeats across classes. All 183 `idc.` IDs (7 classes, 176 descriptors) have distinct `fnv1a32`. Pool minimums required by this chapter's scope are met with margin: 32 potion colors (≥ 24), 28 text sigils (≥ 24), 20 ring stones (≥ 20) plus 6 bands, 18 amulet forms (≥ 16), 18 wand materials (≥ 16), 32 implant stamps (≥ 12) and 22 relic carvings (≥ 12).

Descriptor record:

```json
{ "id": "idc.potion.crimson", "class": "idc.potion", "rgb": 14423100, "sort_index": 9,
  "stopper_shape": 1, "label_mark": 2, "art": "art.idc.potion.crimson" }
```

`rgb` is `0xDC143C` as an integer. `stopper_shape` and `label_mark` must equal `sort_index % 4` and `sort_index / 4`; the validator checks them. Records of the other classes carry no color fields:

```json
{ "id": "idc.ring.opal", "class": "idc.ring", "sort_index": 18, "kind": "stone", "art": "art.idc.ring.opal" }
```

`sort_index` is the descriptor's index in its class sorted by ID byte order; the validator checks it. `kind` is `stone` or `band` for rings and is absent elsewhere. `art.` IDs belong to doc 18.

### 3.4 Genre rendering

1. The distinguishing feature (hue with stopper and mark, sigil, stone or metal, silhouette, wood or alloy, stamp, carving) renders identically in every genre; only the vessel or mount changes.
2. The display name is `"{adjective} {form}"`, from `loc.idc.<class>.<name>.name` and `loc.idc.<class>.form.<genre_suffix>`. The genre is the local genre at the holder's position, or at the item's position on the ground (Canon §1.10, §5.8).
3. Once the type is known, the skinned type name replaces it and the descriptor becomes a subtitle: "Potion of Healing (murky green)".

| Genre | Potion | Text | Ring | Amulet | Wand | Implant | Relic |
|---|---|---|---|---|---|---|---|
| Mythic Fantasy | flask | scroll | ring | pendant | wand | rune-graft | idol |
| Gothic Horror | phial | parchment | signet | locket | rod | stitched graft | memento |
| Cosmic Horror | ampoule | folio leaf | band | medallion | rod | nodule | effigy |
| Space Opera | injector | data slate | band | pendant | emitter | implant | token |
| Cyberpunk | injector | data shard | ring | chain tag | stylus | chip | charm |
| Post-Apocalypse | jar | scrap note | ring | tag | rod | jury-rig | fetish |
| Weird West | tonic | broadside | ring | medallion | divining rod | plug | charm |
| Hardboiled Noir | hip flask | telegram | ring | locket | baton | plug | keepsake |
| Wuxia | pill | talisman | ring | pendant | spirit brush | needle | token |
| Steampunk | tincture | punch card | ring | fob | rod | cog-plug | curio |
| Fairy Tale | vial | rhyme | ring | locket | wand | seed | trinket |
| Mythic Antiquity | unguent jar | ostrakon | ring | pendant | rod | votive graft | votive |
| Oneiric | dream-drop | dream page | ring | pendant | wand | dream-seed | keepsake |
| The Redaction | dose canister | memo | ring | badge | stylus | chip | exhibit |
| The Unwritten | inkdrop | blank leaf | band | pendant | quill | mark | token |
| The Ur-Text | vessel | verse | ring | pendant | rod | inscription | relic |

Example: `idc.potion.crimson` reads "Crimson flask" in Mythic Fantasy, "Crimson injector" in Cyberpunk and "Crimson pill" in Wuxia (Canon §13.6).

### 3.5 The shuffle

Runs on the run-start tick, before any item exists, on `rng.appearance` (`folio_index = 0`):

```rust
fn appearance_shuffle(run_seed: u64, content: &ContentSet) -> TypeKnowledge {
    let mut rng = Rng::from_seed64(stream_seed(run_seed, "rng.appearance", 0));
    let mut tk = TypeKnowledge::default();
    for class in IDC_CLASSES_SORTED {                 // idc.amulet, idc.implant, idc.potion, idc.relic, idc.ring, idc.text, idc.wand
        let mut types: Vec<Id> = content.type_keys(class);  types.sort();     // byte order of the ID string
        let mut pool:  Vec<Id> = content.descriptors(class); pool.sort();
        assert!(types.len() + 2 <= pool.len());           // §3.2, already enforced at load
        rng.shuffle(&mut pool);                           // Canon §3.1 Fisher-Yates
        for (i, t) in types.iter().enumerate() {
            tk.assign.insert(id32(t), id32(&pool[i]));
            tk.by_desc.insert(id32(&pool[i]), id32(t));
        }
    }
    tk                                                    // rng.appearance is never drawn again this run
}
```

- **Draw count:** `Σ (|pool_c| − 1)` calls of `range` over the seven classes, whatever the number of types. With the §3.2 pools that is 17 + 31 + 31 + 21 + 25 + 27 + 17 = **169** calls, in the class order amulet, implant, potion, relic, ring, text, wand. A call consumes more than one `u64` only on a Lemire rejection.
- **Content set.** The host's content set at run start, recorded in the save (Canon §13.6). Daily and weekly runs use `unlock.daily_baseline`, so every player of a daily seed sees the same mapping.
- **Worked example.** Run seed `0x0123456789ABCDEF`. `stream_seed(run, "rng.appearance", 0) = 0x95b9ab50b5b43f53`. The permutations depend only on the seed and the pool sizes. `idc.amulet` goes first and becomes `teardrop, heart, bell, fang, acorn, spindle, lozenge, cage, arrowhead, crescent, coin, key, feather, knot, shell, wheel, disc, star`. Doc 11's 8 amulets, sorted (`composure`, `elements`, `ferocity`, `fortune`, `sanctuary`, `scholar`, `vigor`, `wayfarer`), take `teardrop` through `cage`; the other 10 are unassigned. The other pools begin: implant `twin_moons`, potion `rose`, relic `laurel`, ring `amethyst`, text `yrrel`, wand `copper`. Doc 11's 15 rings, sorted, take the first 15 ring descriptors: `arcane_tongue` amethyst, `bold_tongue` carnelian, `finesse` red_gold, `fortune` lapis, `intellect` bronze, `keen` jet, `machine_tongue` garnet, `might` white_gold, `perception` jade, `presence` diamond, `sacred_tongue` sapphire, `vigor` malachite, `warding` moonstone, `wealth` obsidian, `will` black_iron.

---

## 4. `cat.wand` (owned in full)

### 4.1 Definition and boundaries

A **wand** is a one-handed casting item that spends one **charge** per activation, no aether, and produces one fixed doc-06 spell. Validator classification:

| If a template… | then it is |
|---|---|
| spends charges, spends no aether, holds one spell and has no weapon damage block | `cat.wand` |
| spends aether and has a weapon damage block | `cat.weapon_ranged` (a staff) |
| raises spell potency or sets the casting implement and has no activation | `cat.offhand` (a focus) |
| spends charges but places, throws or deploys something | `cat.gadget` or `cat.throwable` |

### 4.2 A wand's type

Every wand is the archetype `arch.ranged.wand` (listed by doc 11 §3.1 in doc 05's `arch.ranged.` namespace; Canon gaps) holding one authored `spell.` record (doc 06 §8.4). For type hiding, `type_key(wand) = its spell ID`. The `idc.wand` type list of a content set is the set of authored spells tagged `wand_pool`; doc 11 draws wand spells only from that set, and a unique (Exotic) wand must hold a tagged spell. Two wands holding the same spell share type knowledge whatever their skin, genre or rarity. More than 16 tagged spells fails load with `idc_pool_exhausted`.

### 4.3 Charges and zapping

Doc 06 §8.4 owns max charges by rarity (4, 5, 6, 7 for Common to Epic; 8 Legendary and Exotic; 10 Palimpsestic), the zap, passive recharge (+1 charge per 2400 ticks carried), the recharge text and service (charges to max, max −1, never below 1), and snapping. A wand is zapped from the inventory through the quick-cast wheel (doc 06 §3.2) and may also be equipped in `slot.offhand`. This chapter adds:

- Zapping identifies the wand's type (doc 06): wands use `appr.use.always`. A zap at 0 charges does nothing and sets `CHARGES` ("You feel a faint resistance").
- A recharge (text or service) and a snap reveal `CHARGES`.
- `zaps_total` (`u32`, every zap of this instance) extends the Canon §13.9 contract for `curse.leaking`; the UI shows zaps since pickup, which is not hidden.
- `dep = {origin_genre.aether_axiom}`, or `{arcana}` when that is none (Canon §5.3).

### 4.4 What a wand hides

Its type (the spell), RARITY, BUC, ENH, CHARGES and affixes (§2.1).

---

## 5. Identification methods

`identify_type(type_key, reason, by)` adds the type to `known`, recomputes every instance's state, runs deduction (§5.3) and emits one `ev.item_identified`. `reveal(item, flags, reason, by)` sets flags (applying the §2.3 coupling rules) and emits one `ev.item_identified` only when at least one new flag was set. `identify_instance(item, reason, by)` is `reveal(item, required(item))`, then `identify_type` if the item is type-hidden and its type unknown. `identify_one_unknown(carrier, reason)` picks the type-hidden item of unknown type with the lowest `uid` in the carrier's inventory and calls `identify_type` on it, with no draw; it does nothing when there is none.

The event uses the doc 03 §14.2 payload `{ by: EntityId, uid: u64 /* 0 = whole type */, template: Id32, state: Id32 }`, where `template` is the type key for a type event and the item's template for an instance event, and `state` is the new `derive_state` (`know.classified` for a type event). This chapter appends three fields that doc 03 must add (Canon gaps): `descriptor: Id32` (0 when not type-hidden), `flags_new: u32` (the newly set bits; 0 for a type event) and `reason: Id32`. `by` is 0 when no entity caused it (deduction, kit). Reasons are the `appr.reason.*` IDs in Exports. Every `rng.ident` draw is listed in §5.12.

### 5.1 Use, zap and throw

Potions drink as `always` and throw as `on_effect` (doc 08 §12.4); wands are `always` (§4.3); each text template declares `use_id`:

| Rule | Identifies when |
|---|---|
| `appr.use.always` | the use resolves, whatever happened |
| `appr.use.on_effect` | the resolution's `observable` flag is true |
| `appr.use.never` | never by use (no launch text has it) |

**Observable.** The effect owner (docs 06, 08) sets `observable = true` when the resolution changed any entity's HP, stamina, aether, composure, satiation, toxicity, breath or ward; any status; any block; spawned, despawned or displaced any entity; changed a displayed item property; or created a light or sound source. Healing at full HP with no wounds or statuses is not observable.

```rust
fn on_use_resolved(it: &Item, user: SlotOrNpc, out: &UseOutcome, tk: &mut TypeKnowledge) {
    if !type_hidden(it) { return; }
    let d = tk.assign[&type_key(it)];
    tk.seen.insert(d);
    if tk.known.contains(&type_key(it)) { return; }
    let rule = match it.cat { cat.wand => Always, cat.potion => if out.thrown { OnEffect } else { Always },
                              _ => template(it).use_id };
    let obs = match rule { Always => true, OnEffect => out.observable, Never => false };
    if obs { identify_type(type_key(it), if it.cat == cat.wand { appr.reason.zap } else { appr.reason.use }, user) }
    else if it.cat == cat.text { tk.tried_no_effect.insert(d); }
}
```

- **Drinking** identifies the type and, as doc 08 §12.4 requires, also sets every required flag of the drunk instance and of any doses left in its multi-dose vessel (`reveal`, reason `appr.reason.use`).
- **Throwing** a potion identifies it only when the splash affected an entity or caused an `ev.liquid_reaction` (doc 08 §12.4); it is the safe test for malign potions (§8).
- **Alchemy hooks** (doc 08 §12.4): a holder with Alchemy 25 sees an unknown potion's polarity, at 50 also whether it is an aerosol, at 75 also its primary effect kind. Display only; no flags.
- **Reading an unknown text.** If the true type is targeted, the UI asks "The text asks for an object" and `read_mode[d] = Targeted`. If the chosen item is not a valid target for the true type, the text is **not consumed**, `refused_cats[d] |= 1 << row(cat)` and nothing else happens. A valid target resolves the text. An untargeted text resolves at once and sets `read_mode[d] = Untargeted`. A text carrying `appr.eff.intuit` asks "The text asks for a name" instead, a prompt no other text uses, so reading it always identifies it. Every targeted text has rule `always`.
- **Blessed and cursed consumables** apply §7.5 modifiers before `observable` is computed.

### 5.2 Equip-ID and binding

Every type-hidden accessory and implant template declares `equip_id`:

| Rule | Identifies when |
|---|---|
| `appr.use.display` | equipping completes; the effect is a stat the character sheet shows |
| `appr.use.conditional` | the effect owner reports its first observable trigger, or familiarity stage F-type (§5.7) |

Equip sequence for Erratum `E` equipping item `X` (installing, for implants):

1. If `X.buc = cursed` and `X` has neither `BUC` nor `OMEN`: roll Canon §6.4 row 44 (`max(0, d(PRV) × 50)` Bp) with keyed `rng.ident` (actor `E`, action tick, action seq). The roll is drawn even when the chance is 0. **Success:** the equip is cancelled, `X` stays in the inventory, and `BUC`, `AFFIX_NAME[c]` and `AFFIX_ROLL[c]` (the curse slot `c`) are revealed with reason `appr.reason.bind`. Stop.
2. If `X.buc = cursed` and `X` has `BUC` or `OMEN`, the UI asks for confirmation and no roll is made.
3. The equip completes. If `X` is cursed, it binds (§7.2) and the same three flags are revealed with reason `appr.reason.bind`.
4. If `X` is type-hidden, its type is unknown and its rule is `display`, `identify_type` with reason `appr.reason.equip`.

### 5.3 Witness and deduction

- **Witness.** When an NPC uses or zaps a type-hidden item (doc 13 §5.1) and the NPC is visible to any Erratum in that Erratum's most recent sight check (doc 09; `sys.perception` runs every 3 ticks per observer, so the result is at most 2 ticks old), §5.1 runs with the NPC's outcome and reason `appr.reason.witness`. The witnessed item's descriptor joins `seen`.
- **Deduction.** After every change to `TypeKnowledge`, for each descriptor in `seen` whose type is unknown, in ascending `id32` order, compute `candidates(d)` (§5.8). If exactly one type remains, `identify_type` with reason `appr.reason.deduction`. Repeat until a full pass identifies nothing. A descriptor joins `seen` when an unsealed instance enters an Erratum's inventory or a trade window, or is used or witnessed; a sealed item's descriptor joins `seen` when it is unsealed (§11). Deduction is sound because a seen descriptor is always assigned: decoys (§3.2) never appear on an item.

### 5.4 Doc-10 texts

`item.text.appraisal` (value 22) carries `appr.eff.identify` and `item.text.sanctify` (value 30) carries `appr.eff.unbind` (doc 11 §3.5). This chapter proposes five more `cat.text` templates for doc 11 (its namespace; Canon gaps), with these base values for price identification: `item.text.survey` 26, `item.text.omen` 20, `item.text.consecrate` 34, `item.text.intuit` 28, `item.text.amnesia` 18. Each is a `cat.text`, mass 50 g, stack 10, never rarity-eligible. "Inventory" includes equipped items. "Random" picks uniformly with one `range(n)` on keyed `rng.ident` (actor = reader, action tick, action seq) over eligible items sorted by `uid`, and draws nothing when `n = 0`.

| Effect | Uncursed | Blessed | Cursed | Draws |
|---|---|---|---|---|
| `appr.eff.identify` (targeted) | 1 chosen item: every required flag, plus type | up to 3 chosen items | 1 random item with missing flags: type only if type-hidden and unknown, else RARITY and BUC | cursed: 1 |
| `appr.eff.survey` | every inventory item: tier-0 reveal set (§5.5), `est_steps += 1` | tier-1 reveal set, `est_steps += 1` | tier-0 reveal set on 1 random item | cursed: 1 |
| `appr.eff.omen` | BUC of every inventory item | also every item carried by a party member or on the ground within 16 m | OMEN or CLEAN on 1 random item | cursed: 1 |
| `appr.eff.unbind` | every equipped cursed item becomes uncursed (§7.6) | also every carried cursed item | nothing; not observable | 0 |
| `appr.eff.consecrate` (targeted) | cursed → uncursed, uncursed → blessed, blessed unchanged | cursed → blessed, uncursed → blessed | blessed → uncursed; uncursed → cursed with a new curse (§7.1 draws B and C); cursed unchanged | cursed: exactly 2 |
| `appr.eff.intuit` | name one seen, unknown descriptor and one candidate type: correct → `identify_type`; wrong → `excluded[d].insert(t)` | two guesses for the same descriptor | the guess resolves, but a wrong guess is not recorded | 0 |
| `appr.eff.amnesia` | forget 2 identified types; set composure to max; end `st.dread` and `st.madness` | forget 1 | forget 3 | `min(k, n)` |

Consecrate and unbind reveal `BUC` on every item they change. A consecrate that changes nothing still glows, so its target's `BUC` becomes known. A cursed consecrate that curses an equipped item binds it at once (§7.2). Every text in this table is `observable` whenever it set at least one flag or changed a curse state; a cursed unbind is never observable.

**Amnesia** (malign, §8). `k` = 2, 1 or 3 by curse state.

```rust
let mut pool: Vec<Id32> = tk.known.iter().filter(|t| class_of(t) != idc.text).copied().collect(); // already sorted
for _ in 0..min(k, pool.len()) {
    let j = rng.range(pool.len() as u64) as usize;      // keyed rng.ident, actor = reader
    let t = pool.remove(j);
    tk.known.remove(&t);
    let d = tk.assign[&t];
    tk.price_obs.remove(&d); tk.tried_no_effect.remove(&d); tk.excluded.remove(&d);
    tk.read_mode.remove(&d); tk.refused_cats.remove(&d);
}
```

Texts are never forgotten, so amnesia, identify and every text the party knows stay known.

### 5.5 Appraisal (passive, `skill.appraisal`)

**Tiers.** `tier = 0` for skill 0–24, 1 for 25–49, 2 for 50–74, 3 for 75–99, 4 at 100 (Canon §7 mastery thresholds). Reveal sets are cumulative:

| Tier | ID | Adds |
|---|---|---|
| 0 | `appr.tier.novice` | RARITY; records the estimate skill (`est_skill = max(est_skill, skill)`, §6.3) |
| 1 | `appr.tier.apprentice` | BUC (with ENH, §2.3), CHARGES |
| 2 | `appr.tier.journeyman` | every `AFFIX_NAME[i]` |
| 3 | `appr.tier.expert` | every `AFFIX_ROLL[i]` (completes identification of every non-type-hidden item); for a type-hidden item of unknown type, an exact price observation (§5.8) |
| 4 | `appr.tier.master` | the type of a type-hidden item |

**Doc 04 mastery perks** (doc 04 §5.3) plug in here: at 25, "appraised items show exact value" is delivered by the tier-1 reveal set, which sets every flag `worth` depends on (`BUC`, `CHARGES`), after which the value line is exact (§6.3); at 50, the tier-0 reveal set applies on the player's pickup with no work and no draw (as does `run.boon.appraising_eye`, doc 14 §3.9), recording `est_skill = max(est_skill, skill)`; at 75, +2000 Bp curse sense (§5.11); at 100, every equipment item the player carries for 600 consecutive ticks gets `identify_instance` (reason `appr.reason.appraisal`). The 600-tick counter is per (player, item), lives on the player, and resets when the item leaves that player's inventory.

**Work and rate.**

```
need(item, player) = mul_bp(base_work[cat], 10000 + 300 × (min(ilvl, 25) − 1))
                     → × 5000 if appr_tier_done >= 0              // later completions cost half
                     → × 30000 if the completion would add the type (tier 4, type unknown)
rate(player) = mul_bp(max(500, 1000 + 30 × skill.appraisal + 10 × d(ITU)), anl_speed_bp)   milli-work per second
                                       // anl_speed_bp = clamp(10000 + d(ANL) × 100, 5000, 20000), doc 04
```

| `base_work` (milli-work) | Categories |
|---|---|
| 12000 | `cat.potion`, `cat.text` |
| 20000 | `cat.weapon_melee`, `cat.weapon_ranged`, `cat.armor`, `cat.offhand`, `cat.ward_gen`, `cat.mount`, `cat.part` |
| 25000 | `cat.wand` |
| 30000 | `cat.accessory`, `cat.implant` |

All inputs are visible, so progress leaks nothing. Doc 04 traits scale `need` (gumshoe × 5000).

**Tick.** The appraisal system runs in the Needs pass (period 60, phase 7, Canon §2.2), after Needs. For each living, non-downed, non-ghost Erratum in ascending `EntityId`:

```rust
let tier = tier_of(skill.appraisal);
let focus = player.locked_focus.filter(|u| eligible(u, tier))
    .or_else(|| inventory(player).filter(|u| eligible(u, tier))
                 .min_by_key(|u| (need(u, player) - u.appr_progress, u.uid)));
if let Some(it) = focus {
    it.appr_progress += rate(player);
    if it.appr_progress >= need(it, player) {
        apply_reveal_set(it, tier, appr.reason.appraisal, player.slot);
        it.appr_tier_done = tier; it.appr_progress = 0;
        it.est_skill = max(it.est_skill, skill.appraisal);
        grant_appraisal_xp(player, it);                                    // doc 04 XP events
    }
}
```

`eligible(it, tier)`: in the player's inventory, not mundane, and lacking a flag in the tier's reveal set, or the type step applies. Progress stays on the item across trades.

**Descent.** On `ev.folio_exited`, each living Erratum's focus item, chosen by the same rule as the tick, gains `mul_bp(need, 2500 + 25 × skill.appraisal)` progress, from 25% at skill 0 to 50% at skill 100, as in Barony. The completion check runs on that player's next appraisal pass.

**Worked example.** A `cat.weapon_melee` at `ilvl 7`, first completion: `need = mul_bp(20000, 11800) = 23,600`. Appraiser at skill 20, ITU 50, ANL 40: `rate = 1000 + 600 + 100 = 1,700` per second, so it completes on the 14th pass (13 passes give 22,100). At ANL 70, `anl_speed_bp = 13000`, rate 2,210, 11 passes. Tier 0 reveals RARITY. At `ilvl 25` a ring at skill 0, ITU 40 needs `51,600` and takes 52 s. A Master (skill 100, ITU 60, ANL 40, rate 4,200) learning a ring's type at `ilvl 10` after an earlier completion needs `mul_bp(mul_bp(mul_bp(30000, 12700), 5000), 30000) = 57,150`: 14 s.

### 5.6 Analysis checks (`sub.analysis`)

An Erratum channels **Examine** (`input.examine`, doc 17) on an unsealed, non-mundane item in its inventory for 120 ticks; HP damage, another action, `st.staggered` or any control status cancels it with no roll and no lock. It requires `analysis_lvl[slot] < character level` (once per item per party member per level). On completion `analysis_lvl[slot] = level` and one draw is made: `roll = range(10000)` on keyed `rng.ident` (actor = analyst, completion tick, action seq).

```
anl_bp = clamp(3000 + d(ANL) × 100 + skill.appraisal × 40 − 150 × min(ilvl, 25), 500, 9500)
success  ⇔ roll < anl_bp
reveals  = if success { min(3, 1 + floor_div(anl_bp − 1 − roll, 2500)) } else { 0 }
```

- Non-type-hidden (or type known): each reveal takes the next missing flag of `required(item)` in the order RARITY, BUC (with ENH, §2.3), CHARGES, `AFFIX_NAME[0..n]`, `AFFIX_ROLL[0..n]`. Reveals beyond the last missing flag are lost.
- Type-hidden with unknown type: 1 reveal records an exact price observation (§5.8) and RARITY; 2 also reveal BUC and CHARGES; 3 also reveal the type.
- Every success adds `est_steps += 1` (max 4) and grants doc 04's appraisal XP; a failure grants 0.
- **Worked example.** ANL 60, appraisal 30, `ilvl 8`: `anl_bp = 3000 + 2000 + 1200 − 1200 = 5000`. A roll of 2404 succeeds with margin `5000 − 1 − 2404 = 2595`, so `reveals = 1 + 1 = 2`. A roll of 6052 fails. ANL 80, appraisal 50 at `ilvl 5` gives 8250, so 3 reveals need a roll below 3250 (32.5%).

### 5.7 Familiarity

While an item is equipped by a living Erratum inside a Folio, the Needs pass adds 60 to `fam_ticks`. On `ev.entity_killed` of a hostile credited to an Erratum (Canon §15.1 minion kills included), `fam_kills += 1` on every equipped non-weapon item of that Erratum, and on the weapon instance that dealt the killing hit if it was a direct kill. Counters live on the item and survive trades.

A stage is reached when `fam_ticks >= T` **or** `fam_kills >= K`, each stage independently:

| Stage | Reveals | `T` (ticks) | `K` |
|---|---|---|---|
| F1 | RARITY, BUC (with ENH, §2.3) | 1800 (30 s) | 3 |
| F2 | CHARGES | 3600 (60 s) | 8 |
| F(3+k), k = 0..=7 | `AFFIX_NAME[k]`, `AFFIX_ROLL[k]` | `5400 + 1800 × k` | `12 + 4 × k` |
| F-type | type of a `conditional` accessory or implant | 7200 (120 s) | 15 |

Reason `appr.reason.familiarity`. Stages are checked in the Needs pass after the counters change and on every `fam_kills` increment; a stage reveals only flags in `required(item)`. Example: a Rare sword (2 affixes) reveals its rarity, curse state and `eff_enh` (F1) at kill 3 or 30 s, affix 0 (F3) at kill 12 or 90 s, and affix 1 (F4) at kill 16 or 120 s, whichever comes first; F2 has nothing to reveal on a sword. Erasure (Canon §11.6, from 420 s) limits idling for familiarity.

### 5.8 Price observation, the Ledger and candidates

Vendors know every type (rule 3 of §1). Whenever a trade window shows a buy price or a sell offer for a type-hidden item of unknown type, the host records a `PriceObs` under its descriptor, skipping exact duplicates, keeping the newest 8. Every factor of the doc 11 §12.2 price chain is visible to the player (markup, Canon §6.4 row 37 or 38, bearer modifiers, Grade multipliers, haggle, the unread and known-cursed factors, the fence), so the observation stores them and the deduction is exact:

```rust
pub struct PriceObs {
    pub price: i64,          // the displayed price
    pub ilvl: i32,
    pub rar_bp: i32,         // 10000 × rarity value multiplier used by value_vendor (10000 while RARITY is unset)
    pub quality_bp: i32,     // 10000 for consumables (doc 11 §13.1)
    pub enhancement: i32,    // stored enhancement (public, §2.1)
    pub mat_bp: i32,         // primary material value_bp; 10000 when none
    pub cond: Vec<i32>,      // the §6.1 condition factors value_vendor applied, in order (at most 3)
    pub chain: Vec<i32>,     // every doc 11 §12.2 factor applied after value, in order (at most 8)
}
fn price_for_base(base: i64, o: &PriceObs) -> i64 {
    let mut v = base;
    for f in [P(o.ilvl), o.rar_bp, o.quality_bp, 10000 + 600 * o.enhancement, o.mat_bp] { v = mul_bp(v, f); }
    let value = v;                                   // doc 11 §13.1 value of that base
    for f in &o.cond { v = mul_bp(v, *f); }
    if value > 0 { v = max(1, v); }                  // §6.1 worth clamp
    for f in &o.chain { v = mul_bp(v, *f); }
    max(1, v)                                        // doc 11 §12.2 final clamp
}
fn candidates(d: Id32, tk: &TypeKnowledge) -> Vec<Id32> {
    class_types(class_of(d)).filter(|t| !tk.known.contains(t))
        .filter(|t| !tk.excluded.get(&d).map_or(false, |s| s.contains(t)))
        .filter(|t| !(tk.tried_no_effect.contains(&d) && use_rule(t) == Always))
        .filter(|t| tk.read_mode.get(&d).map_or(true, |m| m.matches(t)))
        .filter(|t| valid_target_cats(t) & tk.refused_cats.get(&d).copied().unwrap_or(0) == 0)
        .filter(|t| tk.price_obs.get(&d).map_or(true, |v| v.iter().all(|o| price_for_base(base_value(t), o) == o.price)))
        .collect()                                   // ascending type key id32
}
```

The Ledger panel (doc 17) shows `candidates(d)` for every seen unknown descriptor. It automates NetHack's bookkeeping and keeps the deduction.

**Worked example.** An apothecary (doc 11 §12.1, markup 14000) stocks an unknown murky green flask, true type `alch.potion.healing` (doc 08 §12.1 base value 30), `ilvl 3` (`P = 11881`), uncursed with every flag set (shop stock, §10.1). The buyer has CHM 50, so row 37 gives 9700; Grade 0 gives `gp.shop_price_bp = 10000`; no haggle. `value = mul_bp(30, 11881) = 35`; `cond` is empty; `chain = [14000, 9700, 10000, 10000]` gives `35 → 49 → 47 → 47 → 47`. Price **47**. Doc 08's canonical bases 10, 12, 15, 20, 25, 30, 35 give 14, 18, 22, 31, 38, 47, 55, so among the 16 canonical potions only base 30 fits: healing or haste. Each of the 13 exotics, elixirs and the draught (doc 08 §12.2) stays a candidate only if its own `Σ value(e)` base also gives 47. For a text read with no visible result, `tried_no_effect` also removes every `always` text from its candidates.

### 5.9 Scriptorium and altar services

Scriptoria (doc 12 rooms and the Scriptorium station in every Colophon, Canon §17.1) and patron altar blocks placed by worldgen (doc 12) sell these services; an Erratum-placed altar (doc 06 §16) offers no `appr.svc.*`. `d` is the current Folio depth, with `P(min(d, 300))`. Payment is Lucre per item, and every Lucre cost is then multiplied by `gp.shop_price_bp` (doc 14 Grade parameter, 10000 at Grade 0) like every doc 11 §12.3 service. The table shows Grade 0.

| ID | Where | Effect | Cost | d1 | d3 | d12 | d25 |
|---|---|---|---|---|---|---|---|
| `appr.svc.appraise` | Scriptorium, every vendor | tier-1 reveal set and `est_steps += 2` | `mul_bp(20, P(d))` | 20 | 23 | 51 | 158 |
| `appr.svc.classify` | Scriptorium | type of one type-hidden instance | `mul_bp(40, P(d))` | 40 | 47 | 103 | 316 |
| `appr.svc.identify` | Scriptorium | every required flag plus type | `mul_bp(60, P(d))` | 60 | 71 | 154 | 474 |
| `appr.svc.unbind` | Scriptorium | one cursed item becomes uncursed (§7.6), even while bound | `mul_bp(150, P(d))` | 150 | 178 | 387 | 1186 |
| `appr.svc.drop_test` | altar | dropping any item on an altar block reveals its BUC; genre-skinned bright flash if blessed, dark flash if cursed | free | 0 | 0 | 0 | 0 |
| `appr.svc.consecrate` | altar | cursed → uncursed, or uncursed → blessed; refused for blessed items; at most once per item per Folio | `mul_bp(300, P(d))` Lucre, or 100 favor with the altar's patron when favor ≥ 100 | 300 | 356 | 774 | 2373 |

Costs depend only on depth and Grade, never on the item, so a price never reveals a hidden property (rule 3). They stay below the affliction cure anchor `mul_bp(500, P(d))` (Canon §16.4). Against doc 11 §13.2's per-player income `LUCRE_TARGET(d) = mul_bp(450, P(d))` per Folio, one appraisal costs 4.4% of a Folio's income, one full identification 13.3%, one unbinding 33.3% and one consecration 66.7%, so services fit inside doc 11's services sink of 1500 Bp of `LUCRE_TARGET(d)` (67 Lucre at Folio 1, 533 at Folio 25), which pays for about one identification per Folio. Reasons `appr.reason.service` and `appr.reason.altar`. Doc 11 §12.3's "Identify one item" and "Uncurse one item" rows price from stored `value`, which reads rarity, a hidden property; `appr.svc.identify` and `appr.svc.unbind` replace them (Canon gaps).

### 5.10 Divination glyphs (`skill.divination`)

Glyphs belong to doc 06, whose catalog does not yet list these essences (Interface assumptions); their identification effects are defined here. `div_tier` = mastery tier of `skill.divination` (0..=4). Glyph potency (Canon §5.5) multiplies radii and chances with `mul_bp`; a Disconnected glyph cannot be cast.

| Glyph | Effect ID | Effect | Draws |
|---|---|---|---|
| `glyph.ess.appraise` | `appr.eff.divine_identify` | as uncursed `appr.eff.identify`; as blessed when `skill.divination >= 75` | 0 |
| `glyph.ess.omen` | `appr.eff.divine_omen` | BUC of every item on the ground or carried by a party member within `4000 + 2000 × div_tier` mm | 0 |
| `glyph.ess.sight` (doc 06, existing) | `appr.eff.aura` | also sets AURA on every item within the omen radius: **mundane** (no affixes, `enhancement = 0`, uncursed), **ominous** (cursed), or **storied** (anything else) | 0 |
| `glyph.ess.psychometry` | `appr.eff.psychometry` | one type-hidden item of unknown type in the caster's inventory: success → type; shares the Analysis lock `analysis_lvl[slot]` | 1 `roll_bp` |

`psy_bp = clamp(2000 + 60 × skill.divination + 50 × d(ITU), 0, 9500)`, keyed `rng.ident` (actor = caster, cast tick, action seq). At divination 40, ITU 50: `2000 + 2400 + 500 = 4900`. Reason `appr.reason.glyph`.

### 5.11 Curse sensing on pickup (Canon §6.4 row 36)

On `ev.item_picked_up` by Erratum `E` in party slot `s`, when the item is BUC-bearing, unsealed, has none of `BUC`, `OMEN`, `CLEAN`, and `SENSED[s]` is unset (a sealed item rolls when it is unsealed in `E`'s inventory, with that tick as the pickup tick):

```
SENSED[s] = 1
chance = clamp(1000 + d(ITU) × 100 + flats, 0, 9000)       // Canon §6.4 row 36; doc 04 RowFlat(36, ·): Appraisal 75 adds 2000
if roll_bp(chance) on keyed rng.ident (actor E, pickup tick, action seq):
    if buc == cursed { set OMEN } else { set CLEAN }
```

One draw per (item, party slot), so handing an item to the party's mystic is a real co-op move. ITU 40 gives 1000 Bp, 55 gives 2500, 80 gives 5000, 130 caps at 9000. Reason `appr.reason.sense`.

### 5.12 Every `rng.ident` draw

All keyed (Canon §3.3), and all drawn by the host only. **Action** key: folio = current Folio index, actor = acting Erratum's `EntityId`, tick = the tick on which the action resolves (the completion tick for channels), seq = that Erratum's per-tick action counter (reset in `sys.tick_begin`, doc 03 §2.2; incremented once per keyed action resolved for that Erratum in processing order, below `0x2000` as doc 06 gap G8 requires). Several pickups resolved in one tick are processed in ascending `uid` and get consecutive seq values. **Object** key: folio = `est_key >> 40`, actor = the item's `est_key` (§2.3), tick = `u64::MAX` (never a real tick), seq = purpose code 1, giving a per-item constant that survives splits.

Doc 04's `avatar_of_fate` (Oracle T4, 600 ticks once per Folio) makes each action-keyed success roll of its owner draw twice from the same generator, for exactly 2 draws per roll, and keep the value better for the owner: the lower `range(10000)` for curse sense, curse resistance, Analysis and psychometry, the higher for `curse.backfire`. Random picks (cursed texts, amnesia, `curse.misprint`) are not success rolls and still draw once. Object-keyed draws are unaffected.

| Use | Key | Draws |
|---|---|---|
| curse sense on pickup (§5.11) | action | 1 `range(10000)` |
| curse resistance on equip (§5.2) | action | 1, only for an unknown cursed item |
| Analysis (§5.6) | action, completion tick | 1 |
| psychometry (§5.10) | action | 1 |
| cursed survey, omen, identify: random item (§5.4) | action | 1 when `n >= 1` |
| cursed consecrate: penalty and curse (§5.4, §7.1) | action | exactly 2 |
| amnesia (§5.4) | action | `min(k, n)` |
| `curse.misprint` target (§7.5) | action | 1 |
| `curse.backfire` (§7.5) | action | 1 per zap |
| estimate offset `u` (§6.3) | object: `est_key`, seq 1 | 1 `range_incl(-10000, 10000)` |

---

## 6. Value model

### 6.1 Stored value and true worth

**Stored value** is the Canon §13.9 `value` field, owned by doc 11 §13.1 and restated here so this chapter is self-contained. Each arrow is one `mul_bp`, in order (Canon §1.3):

```
value(it) =
    base_value                                   // template field: whole Lucre at ilvl 1, Common, Normal, +0 (doc 11 §2.1; doc 08 §12.1 for potions)
    → × P(ilvl)                                  // Canon §13.8, d_pow = min(ilvl, 300)
    → × rarity value Bp                          // 10000, 20000, 40000, 80000, 160000, 320000, 640000 (Canon §13.3 ×1 … ×64)
    → × quality_bp                               // Canon §13.5; 10000 for consumables (doc 11 §13.1)
    → × (10000 + 600 × enhancement)              // stored enhancement, public (§2.1)
    → × material value_bp                        // primary material (doc 07); 10000 when none
```

Doc 11 recomputes `value` whenever `ilvl`, enhancement, rarity, material or quality changes. Nothing in it is hidden except rarity, so this chapter adds a second number that carries the hidden condition of the item.

**True worth** is the value of the item as it really is. It drives the appraisal estimate (§6.3) and, through `value_vendor` (§6.2), every price:

```
worth_true(it) =
    value(it)
    → × (10000 + 600 × (eff_enh − enhancement))  // only if H has ENH: blessing +1 or curse penalty −1..−3 at doc 11's per-level slope
    → × 12500                                    // only for a blessed cat.potion or cat.text (Canon §13.6 consumable blessing)
    → × (5000 + floor_div(5000 × charges.cur, max(1, charges.max)))   // only if H has CHARGES
    result max(1, v), or 0 when value(it) = 0
```

`illuminated` never changes value or worth. Cursed items are not marked down here: doc 11 §12.2 applies its own "× 5000 if known cursed" sale factor, which knowledge controls (§6.2).

| Item | `value` steps | `worth_true` steps | Worth |
|---|---|---|---|
| `arch.melee.blade_long` (doc 11 §3.2 base 76), `ilvl 7`, Rare, Good, iron, +2, blessed (`eff_enh = 3`) | 76 → 127 → 508 → 558 → 624 → 624 | 624 → 661 | **661** |
| `arch.ranged.wand` (base 72), `ilvl 9`, Uncommon, Normal, +1, uncursed, 3 of 5 charges (doc 06 §8.4 Uncommon max) | 72 → 143 → 286 → 286 → 303 → 303 | 303 → 303 → 242 | **242** |
| `item.ring.keen`, base 70, `ilvl 12`, Rare, Normal, +0, cursed, penalty 2 (`eff_enh = −2`) | 70 → 180 → 720 → 720 → 720 → 720 | 720 → 633 | **633** |
| `alch.potion.healing`, base 30, `ilvl 5`, blessed | 30 → 42 → 42 → 42 → 42 → 42 | 42 → 52 | **52** |

### 6.2 Vendor value

`value_vendor(view)` is the basis of every vendor price and of every value-scaled cost (doc 11 §12.2 buy and sell, §15 Rewrite, §16.3 reforge). It is `worth` recomputed from the `ItemView` (§2.8), where every unset property takes a default. The vendor knows the type, so a type-hidden item uses its true template's `base_value`:

| Property unknown | Default used |
|---|---|
| rarity (`RARITY` unset) | Common: `value` is recomputed from `base_value` with rarity value 10000 |
| curse state (`BUC` unset) | `eff_enh − enhancement = 0` and no consumable blessing; with `OMEN`: `−1` (cursed, penalty 1); with `CLEAN`: `0` |
| charges (`CHARGES` unset) | `charges.cur = 0`, so the factor is 5000 |

An identified item therefore has `value_vendor == worth_true`, and an unappraised Legendary prices as a Common. Doc 11 §12.2 reads two knowledge tests, which this chapter defines:

- **"not `know.appraised`"** (sale × 5000) is `unread(view)`: true when the item is not mundane and either its type is hidden and unknown or its `RARITY` flag is unset. Mundane items and potions or texts of known type are never unread.
- **"known cursed"** (sale × 5000) is `BUC` set with `buc = cursed`, or `OMEN` set.

Example: the §6.1 longsword with only quality visible has `value_vendor = 76 → 127 → 127 → 139 → 155 → 155 = 155` (rarity Common, no blessing term). At CHM 40 (row 38 = 3000), Grade 0, not at a fence, it sells for `155 → 46 → 23` (unread × 5000). Identified, `value_vendor = 661` and it sells for `mul_bp(661, 3000) = 198`. The wand with `CHARGES` unknown has `value_vendor = mul_bp(303, 5000) = 151`.

### 6.3 The appraisal estimate

An item whose `RARITY` is known shows, on its value line:

- **Exact** `worth_true` when every worth flag in `H(item)` is set: `BUC` and, if hidden, `CHARGES` (rarity is already known). Exactness then reveals nothing, because `value_vendor == worth_true`. Doc 04's Apprentice perk is this case: an Apprentice's completions apply the tier-1 reveal set, which sets those flags (§5.5).
- Otherwise a **range** that always contains `worth_true`, whose center is offset by a fixed, item-specific fraction:

```
err_bp = clamp(5000 − 100 × est_skill − 750 × est_steps, 500, 5000)    // half-width as a share of worth
u      = keyed_rng("rng.ident", est_key >> 40, est_key, u64::MAX, 1).range_incl(-10000, 10000)   // object constant, §5.12
V      = worth_true(it)
half   = mul_bp(V, err_bp)
off    = mul_bp(half, mul_bp(u, 6000))              // |off| ≤ 60% of half
lo     = max(0, min(V, V + off − half))
hi     = max(V, V + off + half)
g      = 10^max(0, digits(hi) − 2)                  // display granularity: two significant figures; digits(0) = 1
shown  = "≈ {floor_div(lo, g) × g}–{ceil_div(hi, g) × g}"
```

`u` is drawn once, the first time the range is computed, and cached on the host; it is constant per `est_key`, so the range narrows toward a fixed off-center point as `est_skill` and `est_steps` grow, never loses the true worth, and is the same on every split of a stack (§2.5). Without that rule, splitting a stack would give fresh offsets whose intersection reveals the blessing. The range is a deliberate hint: an item whose range excludes `value_vendor` is not plainly uncursed. An item with `RARITY` unknown shows "Value ?", or the Ledger line (§6.4) while its type is unknown.

**Worked example.** Run seed `0x0123456789ABCDEF`, the §6.1 longsword with `uid = est_key = (7 << 40) | 42`, `BUC` unknown, so `V = 661`. Its constant is `u = −6692`, and `mul_bp(−6692, 6000) = −4016`. With `est_skill = 20`, `est_steps = 0`: `err_bp = 3000`, `half = mul_bp(661, 3000) = 198`, `off = mul_bp(198, −4016) = −80`, `lo = 383`, `hi = 779`, `g = 10`, shown **"≈ 380–780"**. At skill 0 (`err_bp = 5000`, `half = 330`, `off = −133`) it shows "≈ 190–860"; at skill 20 with two Analysis steps (`err_bp = 1500`), "≈ 520–720"; at skill 0 with four steps (`err_bp = 2000`), "≈ 470–740"; at skill 24 with four steps (`err_bp = 500`), "≈ 610–680". The visible `value_vendor` assuming uncursed is 624, inside every range, so the range alone does not reveal the blessing here. Once `BUC` is known it shows "661".

### 6.4 Display formats

| State | Name line | Value line | Curse line | Enhancement | Properties |
|---|---|---|---|---|---|
| `know.unknown` | "Murky green flask" | Ledger: "Price group 30: 2 candidates" or "Value ?" | "?", "Ominous" (OMEN) or "Clean" (CLEAN) | — | — |
| `know.classified` | skinned type name and quality: "Good Longsword" | "Value ?" | as above | "+?" (OMEN: "−?") | "Unknown properties" |
| `know.appraised` | rarity color, unique name if any: "Rare Longsword" | "≈ 380–780 Crowns", or exact once the worth flags are known | as above or the known state | "+?", "−?" or "+3" | "2 unread properties", then each revealed name, with "(?)" until its roll is known |
| `know.identified` | full name | "661 Crowns" | skinned state: Blessed, —, Cursed (or Hexed, Corrupted per genre) and penalty | "+3" (`eff_enh`) | every affix with its roll |

Mundane items show exact value and no curse line. Money uses the local Lucre skin (Canon §5.2). "Price group" is the true type's `base_value` shown only through the Ledger's candidate list (§5.8).

## 7. Curses and blessings

### 7.1 Generation: `roll_curse`

Doc 11 §2.3 generation step 12 calls this once per BUC-bearing item (every category whose `H` has BUC, §2.1) with its `loot.buc` sub-generator of keyed `rng.loot`, after steps 1 to 11 have fixed the template, rarity, origin genre and affixes. Doc 11 skips the call for shops, Fortune Engines and Volume rewards, which are always uncursed. Kit, crafted and brewed items never call it. It always draws exactly three values, whatever the outcome.

```rust
// Doc 11 calls: doc10::roll_curse(&mut it, &mut sub(seed, "loot.buc"), local_myt(req), req.source)
fn roll_curse(it: &mut ItemInstance, rng: &mut Rng, local_myt: i32, src: SourceTag) {
    let origin_myt = genre_axiom(it.origin_genre, AX_MYTHOS);                           // Canon §5.2 table
    let live = local_myt > 0 && origin_myt > 0;                                         // Canon §5.3: no mythos component where MYTHOS = 0
    let curse_bp = if !live { 0 } else {
        min(9000, mul_bp(mul_bp(mul_bp(curse_base(it), 10000 + (local_myt - 5) * 1500), // Canon §5.7 MYTHOS term
                                rarity_factor(it.rarity)), source_curse_factor(src)))
    };
    let bless_bp = if !live { 0 } else { bless_base(it) };                              // curse_bp + bless_bp <= 10000
    let elig: Vec<CurseId> = eligible_curses(it.cat, slot_group(it));                  // §7.3, ascending ID byte order, never empty
    let a = rng.range(10000) as i32;                                                    // draw A
    let b = rng.weighted(&[5000, 3500, 1500]);                                          // draw B: penalty 1, 2, 3
    let c = rng.range(elig.len() as u64) as usize;                                      // draw C
    if a < curse_bp {
        it.buc = Cursed; it.curse_penalty = (b + 1) as i8;
        it.affixes.push(Affix { id: elig[c], tier: (b + 1) as u8, roll_bp: 10000 });     // always the last affix
    } else if a < curse_bp + bless_bp { it.buc = Blessed; it.curse_penalty = 0; }
    else                               { it.buc = Uncursed; it.curse_penalty = 0; }
}
```

`local_myt` is the local MYTHOS at the generating entity or container (Canon §5.8, seam zones included); for a Keepsake (doc 15 §5.4) it is the MYTHOS of the Reliquary's source genre, which is also the Keepsake's origin genre. The origin check matters only in a seam zone of a `genre.unwritten` Folio, where doc 11 step 4 gives the item the Unwritten origin while the local MYTHOS is the other genre's.

| Category | `curse_base` | `bless_base` |
|---|---|---|
| weapons, `cat.armor`, `cat.offhand`, `cat.ward_gen` | 1500 | 500 |
| rings, amulets, relics | 2000 | 500 |
| belts, back items | 1200 | 500 |
| `cat.implant` | 1500 | 300 |
| `cat.wand` | 1000 | 500 |
| `cat.potion`, `cat.text` | 1000 | 1000 |
| `cat.mount`, `cat.part` | not BUC-bearing (§2.1); never called | — |
| malign accessory or implant type (§13) | 6000 | 0 |

| Factor | Value |
|---|---|
| `rarity_factor` | 10000 up to Epic; 5000 Legendary and Exotic; 0 Palimpsestic |
| `source_curse_factor` | 0 for in-run Reliquary contents (doc 11 §10, mimic loot included) and for drops of `rank.editor` kills; 10000 for every other calling source: kill drops, chests, floor loot, treasure bags, secret caches and Keepsakes |

The blessing chance has no rarity or source factor. Validator rule: `eligible_curses` is non-empty for every BUC-bearing (category, slot group) pair; §7.3 gives at least 4 to each.

**Worked examples.** A weapon dropped in Gothic Horror (MYTHOS 9): `mul_bp(1500, 16000) = 2400` Bp. The same in Hardboiled Noir (MYTHOS 1): `mul_bp(1500, 4000) = 600`. A ring in Mythic Fantasy (6): 2300. A potion in Fairy Tale (8): 1450. A malign ring in Cosmic Horror (10): `mul_bp(6000, 17500) = 10500`, capped at 9000. A Legendary weapon in Mythic Antiquity (10): `mul_bp(2625, 5000) = 1312`. A weapon Keepsake from a Gothic Horror Reliquary: 2400. Anything generated in The Unwritten (MYTHOS 0): 0.

**Balance.** Common to Epic weapons are cursed 600 to 2625 Bp of the time and rings 800 to 3500 Bp, by local MYTHOS 1 to 10. That brackets Shattered Pixel Dungeon's 30% curse rate for generated gear at the top and NetHack's 13% for weapons in the middle (research R1 §2.6, §6). Legendary and better halve it and in-run Reliquaries remove it, so the rewards players chase most are the least likely to bind. Every curse is also an affix the Index records (§12), and a cursed item still deals and blocks with its full base power minus at most 3 `eff_enh` (−1800 Bp of `10000 + 600 × eff_enh`), so a cursed find is a setback, never a run-ender.

The curse is appended to `affixes` as `{id: <curse>, tier: <penalty>, roll_bp: 10000}` and stays last.

### 7.2 Binding

A cursed equipped item cannot be unequipped (Canon §13.6), whatever its knowledge state or Contradiction state.

- A cursed weapon in any weapon slot becomes the active weapon on equip, and active-weapon switching is locked while it is equipped. A cursed two-handed weapon also locks `slot.offhand` empty.
- A cursed wand binds only while equipped in `slot.offhand`; a cursed wand in the backpack is not bound and can be dropped, but its zap curses (§7.3) act on every zap wherever it is carried. A cursed implant cannot be uninstalled.
- Binding ends only when the item becomes uncursed or its slot stops existing. A severed `bp.arm_l` or `bp.arm_r` (Canon §9.1) takes the ring in `slot.ring_l` or `slot.ring_r` with it; the ring stays on the `cat.body_part` item and is recovered with a 60-tick interact, still cursed. `st.polymorphed` stows equipment and re-equips bound items when it ends. Ghost item loss (Canon §17.6) and Redaction item loss (Canon §17.9) may take a bound item.
- Bound items cannot be sold, dropped, traded, salvaged (doc 11 §16.4) or used as crafting inputs. Vendors refuse them (§10.3).

### 7.3 Curse affixes

Every curse is a component with `dep = {mythos}` (Canon §5.3) whose origin is the item's `origin_genre`. With `p = curse_penalty` and `pot` = the curse component's Contradiction potency, its magnitude is `m = mul_bp(unit × p, pot)`, a non-negative number applied in the stated direction. A Disconnected curse has `m = 0`, but still binds and still lowers `eff_enh`. "Equipment" means weapons (`cat.weapon_melee`, `cat.weapon_ranged`), `cat.armor`, `cat.offhand`, `cat.ward_gen`, `cat.accessory` and `cat.implant`; it excludes wands, which are often only carried, so a wand draws only from the four zap curses that name `cat.wand`. Stat penalties apply to effective sub-attributes before the Canon §6.2 clamp to `1..=150`. Effects of several curses on one bearer add.

| ID | Applies to | Unit | Effect while equipped |
|---|---|---|---|
| `curse.leaden` | equipment | 10000 | item mass × `(10000 + m)` |
| `curse.thirsting` | equipment | 2500 | natural HP regeneration × `(10000 − m)` |
| `curse.fumbling` | equipment | 5 | SLT −`m` |
| `curse.feeble` | equipment | 5 | PWR −`m` |
| `curse.sluggish` | equipment | 5 | AGI −`m` |
| `curse.frail` | equipment | 5 | TGH −`m` |
| `curse.addled` | equipment | 5 | MEM −`m` |
| `curse.myopic` | equipment | 5 | SGT −`m` |
| `curse.muffled` | equipment | 5 | HRG −`m` |
| `curse.ill_starred` | equipment | 5 | FAT −`m` and SER −`m` |
| `curse.blunted` | weapons, `cat.offhand` | 1000 | −`m` Bp in the additive bucket (Canon §10.5 step 1) of every hit the bearer deals |
| `curse.misfire` | `cat.weapon_ranged`, `cat.wand` | 300 | +`m` Bp to the base-function failure chance (Canon §5.5), rolled at the Canon §5.5 cadence even when Contradiction gives 0; a wand's failed zap is a doc 06 fizzle, so its charge is spent for nothing |
| `curse.dulled` | weapons | 500 | crit chance gear term −`m` Bp (Canon §6.4 row 13) |
| `curse.loud` | equipment | 2 | every noise the bearer emits +`m` dB (doc 09) |
| `curse.beacon` | equipment | 2 | the bearer is a dynamic light source of level `min(15, 2 + m)` |
| `curse.ravenous` | equipment | 1 | −`m` satiation on Needs ticks where `floor_div(t, 60) % 4 == 0` |
| `curse.brittle` | weapons, `cat.armor`, `cat.offhand`, `cat.ward_gen` | 5000 | durability loss × `(10000 + m)` (doc 07) |
| `curse.bloodprice` | weapons, `cat.wand`, `cat.offhand` | 50 | each activation (a melee swing, a shot, a zap or an off-hand use) costs `max(1000, mul_bp(max_hp_mhp, m))` mHP as a `dmg.true` hit on the bearer entering Canon §10.5 at step 4 (no crit, no chip floor, no buildup); the hit never lowers HP below 1,000 mHP. An item pays at most once per 30 ticks (the Canon §5.5 failure cadence); activations inside that window are free, so at `p = 3` the cost is at most 3% of Max HP per second |
| `curse.backlash` | weapons | 500 | each hit returns `mul_bp(HP damage dealt, m)` to the wielder as a `dmg.true` hit by the bloodprice rule (step 4, never below 1,000 mHP); a return of 0 deals nothing |
| `curse.dread` | equipment | 10 | composure regeneration 0; max composure −`m` |
| `curse.red_letter` | equipment | 8000 | every `fac.editors` member within `m` mm with line of sight targets the bearer (doc 13) |
| `curse.siphon` | equipment | 250 | aether regeneration −`m` m-aether per s, floored at 0 |
| `curse.greed` | equipment | 1500 | Lucre picked up × `(10000 − m)` |
| `curse.fragile_ward` | `cat.ward_gen` | 1500 | ward capacity × `(10000 − m)` |
| `curse.spoiled` | `cat.potion` | 10000 | drinking also adds `m` mTox to `res.toxicity` |
| `curse.misprint` | `cat.text` | — | targeted: the target is a random eligible item (1 `rng.ident` draw); untargeted: positive effects × 5000 instead of × 7500 |
| `curse.backfire` | `cat.wand` | 500 | each zap rolls `roll_bp(m)` on `rng.ident`; success: the effect originates at and targets the zapper |
| `curse.leaking` | `cat.wand` | — | a zap after which `zaps_total % (4 − p) == 0` spends one extra charge; inactive when Disconnected |

Content record:

```json
{ "id": "curse.feeble", "applies": ["cat.weapon_melee", "cat.weapon_ranged", "cat.armor", "cat.offhand",
  "cat.ward_gen", "cat.accessory", "cat.implant", "cat.wand"], "unit": 5, "stat": "sub.power", "dir": -1 }
```

### 7.4 Curses and Contradiction

`Req[mythos] = max(1, origin.MYTHOS − 2)` (Canon §5.3). Example: a ring cursed with `curse.feeble` at `p = 3` (`unit × p = 15`), generated in Gothic Horror (`Req = 7`), worn by a Gothic Horror Origin character (`Tol[mythos] = 2`, ATN 40):

| Local genre | MYTHOS | raw gap | gap = C | Potency | PWR penalty |
|---|---|---|---|---|---|
| Mythic Antiquity | 10 | 0 | 0 | 10000 | 15 |
| Cyberpunk | 2 | 5 | 3 | 7000 | 10 |
| Hardboiled Noir | 1 | 6 | 4 | 6000 | 9 |

Low-MYTHOS genres weaken curses but never release them.

### 7.5 Cursed and blessed consumables

- Blessed potions and texts: positive effects × 12500 (Canon §13.6); malign effects unchanged.
- Cursed potions and texts: positive effects × 7500 and malign effects × 12500, plus their curse affix, which is the only curse eligible for their category: `curse.spoiled` (potions) or `curse.misprint` (texts). The doc-10 texts use their own cursed column in §5.4 instead of the generic × 7500.
- Wands are not consumables. A cursed wand carries one of its four zap curses (§7.3), and its zaps use `eff_enh` (Canon §13.7; doc 06 §8.4 step 0), so the curse penalty also weakens them.

### 7.6 Removing curses

"Uncursing" sets `buc = uncursed`, `curse_penalty = 0`, removes the curse affix and sets `BUC`. Enhancement is unchanged.

| Method | Scope | Cost |
|---|---|---|
| Emendation (Canon §13.7) | one item; `BUC` set if it was cursed, otherwise `CLEAN` | the Emendation |
| `appr.eff.unbind` | equipped items; blessed also carried | the text |
| `appr.eff.consecrate`, `appr.svc.consecrate` | one item, one step up | text, Lucre or favor |
| `appr.svc.unbind` | one item, even while bound | `mul_bp(150, P(d))` × `gp.shop_price_bp` |
| Doc 08 transmutations `alch.tx.uncurse`, `alch.tx.consecrate` | one potion | ingredients |
| Prayer need (4) and `prayer.absolution` (doc 06 §16) | the lowest-`uid` cursed equipped item | prayer timer, or the petition's favor |

Severing, ghost loss and Redaction end binding (§7.2) but never uncurse.

**Curse-state machine.** It is closed: every transition is listed, and nothing else changes `buc`. Every transition caused by a party action sets `BUC` on the item (§2.4).

| From | To | Trigger |
|---|---|---|
| none | blessed, uncursed or cursed | creation: `roll_curse` (§7.1); brewing with `liq.holy_water` gives blessed and with `liq.unholy_water` cursed (doc 08); doc 04 `magnum_opus` brews blessed; kit, crafted, shop, Engine and Volume-reward items are uncursed |
| cursed | uncursed | Emendation; uncursed or blessed `appr.eff.unbind`; `appr.svc.unbind`; uncursed `appr.eff.consecrate`; `appr.svc.consecrate`; `alch.tx.uncurse`; prayer need (4) or `prayer.absolution` |
| cursed | blessed | blessed `appr.eff.consecrate`; `alch.tx.consecrate` (potions) |
| uncursed | blessed | uncursed or blessed `appr.eff.consecrate`; `appr.svc.consecrate`; `alch.tx.consecrate` |
| uncursed | cursed | cursed `appr.eff.consecrate` (new penalty and curse, §5.4) |
| blessed | uncursed | cursed `appr.eff.consecrate` |
| blessed | blessed | uncursed or blessed `appr.eff.consecrate`, `alch.tx.consecrate` (no change; `BUC` revealed) |
| cursed | cursed | cursed `appr.eff.consecrate` (no change; `BUC` revealed) |

### 7.7 Blessings

- Effects are Canon §13.6. Blessed equipment's +1 is inside `eff_enh`.
- A blessed item cannot be cursed in one step: any effect that would curse it makes it uncursed instead (cursed consecrate).
- A blessed item shows its genre-skinned glow only once `BUC` is known.
- Blessing sources: generation (§7.1), `appr.eff.consecrate`, `appr.svc.consecrate`, `alch.tx.consecrate` and brewing with `liq.holy_water` (doc 08), and doc 04's `magnum_opus` talent. No patron boon blesses items (doc 06 §16).

---

## 8. Dangerous unknown consumables

Using an unknown consumable is a deliberate gamble, as in Shattered Pixel Dungeon. Docs 08 and 06 own the effects; these rules bound them.

1. **Malign share.** Harmful types make up 3000 Bp of `cat.potion` generation weight and 1000 Bp of `cat.text` generation weight at every depth (doc 11 loot tables). Harmful potions are `alch.potion.liquid_flame`, `.frost`, `.toxic_gas`, `.slumber` and the exotics `.dragonfire`, `.snap_freeze`, `.corrosive_gas` (doc 08 §12.1, §12.2); the only harmful text is `item.text.amnesia`. The text share is lower because a single harmful text type at 3000 Bp would make every third unknown text an amnesia, which punishes reading far more than Shattered Pixel Dungeon does.
2. **Self-harm cap.** Damage a drunk or read consumable, or a `curse.backfire` zap (§7.3), deals to its own user is capped at `mul_bp(max_hp_mhp, 3000)` per use, and control statuses it applies to its own user last at most 300 ticks. Example: drinking `alch.potion.dragonfire` at depth 25 with 220 HP Max deals at most 66,000 mHP to the drinker.
3. **Lasting harm** is limited to forgotten types (`appr.eff.amnesia`) and red-ink doses (doc 08 forbidden draughts).
4. **Every harmful type has a deliberate use**, so nothing is pure junk.

| Harmful type | Deliberate use once known |
|---|---|
| `alch.potion.liquid_flame`, `.dragonfire` | thrown fire bomb |
| `alch.potion.frost`, `.snap_freeze` | thrown freeze and ice bridge over water |
| `alch.potion.toxic_gas`, `.corrosive_gas` | thrown poison or acid cloud |
| `alch.potion.slumber` | thrown sleep cloud for sneak attacks |
| text carrying `appr.eff.amnesia` | restores composure, ends `st.dread` and `st.madness` |

Throwing is the safe test (§5.1). Clouds and blasts hit allies under Canon §17.5 area rules, so testers step away from the party.

---

## 9. Gear unknowns: progressive reveal

Reveal ladder for a Rare weapon with 2 affixes:

| Method | RARITY | BUC | ENH | Affix names | Affix rolls |
|---|---|---|---|---|---|
| Appraisal tier 0 / 1 / 2 / 3 | 0 | 1 | 1 (with BUC) | 2 | 3 |
| Familiarity | F1 | F1 | F1 (with BUC) | F3, F4 | F3, F4 |
| Analysis | 1st reveal | 2nd | 2nd (with BUC) | 3rd, 4th | 5th, 6th |
| Equip a cursed item | — | on bind | on bind | the curse slot | the curse slot |
| Curse sense | — | OMEN or CLEAN | — (shown "−?" with OMEN) | — | — |
| `appr.eff.identify`, `appr.svc.identify` | all | all | all | all | all |

- **UI masking.** The character sheet shows true values; a stat with a contribution from an unrevealed property is marked "?" and the item breakdown hides that contribution.
- **Unread Edge.** Each equipped item of unknown type or missing any required flag gives its bearer +4 SER for the Canon §13.4 rarity weights only, at most 3 items (+12), after Shattered Pixel Dungeon's Shard of Oblivion. It feeds doc 11 §2.3 step 3 and nothing else: not the Fortune Engine shift, not Canon §6.4 row 43, and not while `st.unmoored` (doc 11 uses SER 40 then). It fades as familiarity reveals the gear: every item reaches `know.identified` (type included) within 18,000 equipped ticks (F(3+7) for 8 affixes) or 40 kills, so one unknown item gives the Edge for at most 300 s of wear. Example: at `d = 10`, SER 40 gives rarity weights `[7000, 2992, 1032, 312, 109, 42, 0]`; with +12 SER, `[7000, 3124, 1104, 339, 120, 46, 0]`.

---

## 10. Shops and selling

### 10.1 Stock

Doc 11 §12.1 fills shops from `rng.loot_place`, uncursed and `know.identified`: every stock item is created with every required flag set. A type-hidden stock item still shows its appearance and `know.unknown` until the party knows its type, and its buy price is a price observation (§5.8).

### 10.2 Prices

Doc 11 §12.2 owns the price chain; this chapter fixes its value input and its two knowledge tests (§6.2). Each arrow is one `mul_bp`:

```
buy  = value_vendor(view) → × markup_bp (doc 11 §12.1) → × Canon §6.4 row 37 → × bearer modifiers in ascending item uid
                          → × gp.shop_price_bp → × (10000 − haggle_bp) → max(1, ·)
sell = value_vendor(view) → × Canon §6.4 row 38 → × 5000 if unread(view) → × 5000 if known cursed
                          → × 11000 at a fence → × gp.lucre_bp → × every later doc 11 Lucre-gain factor → max(1, ·)
```

Prices are per unit. Every displayed buy price or sell offer for an item of unknown type records a `PriceObs` (§5.8) holding exactly these factors.

**Selling unidentified items.** Anything not bound, not a quest item and not on doc 11's unsellable list can be sold in any knowledge state. The vendor knows the type but never names it, and a sale identifies nothing. An unappraised item sells as a Common at half price (`unread`), so the §6.2 longsword sells for 23 unappraised and 198 identified: appraising before selling pays. Selling a sealed item unseals it first (§11). Buying an unknown consumable and drinking it is the price of knowledge, so shops are also an identification tool.

### 10.3 Rules

- Vendors refuse bound items and `cat.quest` items.
- **Buyback** (doc 11 §12.3): each shop keeps, per player, the last 12 items that player sold to it this Folio, for sale back at the exact price paid, with knowledge, `est_key` and counters unchanged; cleared at Folio exit. A buyback price is not a new `PriceObs`.
- Every vendor offers `appr.svc.appraise`; only Scriptoria offer the other `appr.svc.*` services, and altars the altar services (§5.9). Fortune Engine capsules arrive with `RARITY` set and are never cursed.
- Haggling (doc 11 §12.2) never reveals anything: its score reads only Speech, CHM and depth.

---

## 11. Seals: Reliquary contents

A **sealed** item (`sealed = true`, doc 11 instance field) came from an in-run Reliquary, a Fortune Engine capsule or an account Reliquary (a Keepsake, doc 15 §5.4) and has not yet been inspected by a party member. Sealing is a presentation layer over normal knowledge and never changes the item.

| Rule | Value |
|---|---|
| Knowledge at creation | RARITY set, every other flag clear; type knowledge from the party table (Canon §13.6) |
| Sealed view | category silhouette, origin-genre tag, rarity frame and `ilvl`; a type-hidden item's descriptor is also withheld |
| Unsealing | the first time any party member opens the item's detail view, equips, uses, sells or trades it: `sealed = false`, no draw, no event |
| After unsealing | §6.4 display; every §5 method applies |
| Curses | in-run Reliquary and Fortune Engine contents are never cursed (§7.1); Keepsakes roll curses with their source genre's MYTHOS, and a cursed Keepsake behaves like any cursed item (doc 15 §5.5) |
| Engine faces | the Fortune Engine panel (doc 11 §11.2) shows each face with the sealed view; the dispensed capsule stays sealed until unsealed |
| Shared loot | in `mode.loot_shared` a Reliquary's one Public instance (doc 11 §17) shows the sealed view to every player until any party member unseals an item |
| Draws | sealing and unsealing draw nothing; curse sense (§5.11) rolls on unsealing in the holder's inventory |
| Descriptor | a sealed item's descriptor joins `seen` only on unsealing (§5.3), so deduction cannot name a sealed item |

Sealed-item state machine (closed):

| From | To | Trigger |
|---|---|---|
| none | Sealed | created by an in-run Reliquary opening, a Fortune Engine dispense, or Keepsake materialization (doc 15 §5.5) |
| Sealed | Unsealed | any party member opens its detail view, equips, uses, sells, trades, drops or salvages it; or the run ends (the Index records it, §12) |
| Unsealed | Unsealed | terminal: an item is never resealed |

**Account Reliquaries.** Doc 15 fixes and stores the full Keepsake instance at open, so its identity never changes. The rarity frame is visible from that moment, matching the disclosed odds and pity counters. A Keepsake materializes sealed from the Satchel (doc 15 §5.5), is identified only inside runs by §5 methods, and on reaching `know.identified` records its template in the Index (§12). No part of sealing is sold, timed or accelerated with money.

---

## 12. The Index and the Bestiary

Doc 14 §5.5 ("Codex knowledge") places the account's item encyclopedia (the **Index**, `margin.index`, which DESIGN_BRIEF §4 calls the item codex and doc 15 the Item Codex) and the **Bestiary** (`margin.bestiary`) in the Margin and sets their unlock thresholds. They are not the Codex battle pass (Canon §20, doc 15 §6). This chapter owns their identification content, familiarity ranks, illuminated pages, the Cross-Reference aid and the never-monetized rule. Both are account state, stored locally, synced when online, and complete offline under the end-of-service guarantee (DESIGN_BRIEF §8). The host reports each event below to every present party member's account at `ev.run_ended`, and each client applies it to its own account.

### 12.1 Index

- An entry opens on a template's first `know.identified` instance, plus one skin entry per genre in which the player held an instance (doc 14 §5.5). A type-hidden type's page lists its price group (`base_value`) and use or equip rule.
- `familiarity += 1` once per run per template at its first identification in that run (type known, or an instance reaching `know.identified`), for every party member present in the Folio.
- Each curse affix seen with `AFFIX_NAME` set opens a curse page (effect, unit, applicable categories), so the Index doubles as the curse glossary.
- Appearance mappings are never stored, since they reshuffle every run (doc 14 §5.5).

Index record (account state, integer-only):

```json
{ "template": "item.ring.keen", "familiarity": 7, "rank": "know.rank.familiar",
  "skins": [ { "genre": "genre.gothic_horror", "illuminated": 0 }, { "genre": "genre.wuxia", "illuminated": 1 } ],
  "first_identified_utc": 1791763200 }
```

`illuminated` is 0 or 1 per skin (§12.2). `first_identified_utc` is whole seconds since the Unix epoch, display only.

| Rank | Points | Unlocks |
|---|---|---|
| `know.rank.noted` | 1 | page with description, price group, use rule |
| `know.rank.studied` | 3 | the in-run Ledger shows this type's price group and use rule next to each candidate |
| `know.rank.familiar` | 6 | the type may be named in a Cross-Reference (§12.3) |
| `know.rank.mastered` | 10 | gilded page, lore text, title cosmetic |

### 12.2 Illuminated variants

Canon §13.9 unlocks an illuminated skin account-wide on pickup. The unlock is recorded at pickup, but its notification and the illuminated render wait until `RARITY` is known or `ev.run_ended`, because illumination rolls only on Legendary-or-better drops (doc 11 §2.3 step 13: one `roll_bp(5)`) and an early glow would leak rarity. A sealed item's illumination also waits for unsealing. Each illuminated skin appears in the Index as a gilded plate on its (template, genre) skin entry; an illuminated type-hidden item whose type the player never identified records a plate on a placeholder page that shows "An illuminated page awaits its name" until a later run identifies that type. Illumination is cosmetic only (Canon §13.9), so the plate never changes value, worth or any price (§6.1).

### 12.3 Cross-Reference

Renamed from "Recollection", which Canon §20 and doc 14 §6 give to Echo trials (doc 14 Canon gap 16). Once per Volume per party slot (`xref_volume[slot] < current Volume`), a player at a Scriptorium may name one seen, unknown descriptor and one type of the same class at `know.rank.familiar` or above on that player's account. If `assign[type] == descriptor`, `identify_type` with reason `appr.reason.cross_reference`; otherwise `excluded[descriptor].insert(type)` and deduction runs (§5.3). Either way `xref_volume[slot] = current Volume`. No draws. Cross-Reference is off in Daily, Weekly and Seeded leaderboard modes (doc 14 §8), because there account knowledge would be a ranked advantage; display-only Index features stay on. It is horizontal (DESIGN_BRIEF §4): it adds one informed guess per Volume, never a mapping.

### 12.4 Bestiary

Doc 14 §5.5's thresholds, counting kills of each enemy template made while the player was present in the Folio, carry these rank IDs: `know.bestiary.sighted` (1 kill: entry, art, lore), `know.bestiary.studied` (10 kills: resistances and immunities on the target HUD in every run), `know.bestiary.catalogued` (25 kills: drop categories). Bestiary ranks reveal enemy facts only, never item appearances or curse states.

### 12.5 Never monetized

- Entries, points and ranks come only from identification and kills inside runs; no SKU, entitlement, Inkwell result or Reliquary opening grants them, and nothing can be bought, sped up or skipped. A Keepsake counts only once identified in a run. Doc 15's `pass.xp.codex_entry` (200 Codex XP for a first entry) reads Index and Bestiary progress but never writes it, and no premium track reward touches them.
- Completion counts only entries obtainable in runs; Inkwell cosmetics live in the wardrobe (doc 15). Completion rewards are cosmetic titles and Margin decorations only.

---

## 13. Content bindings

| Class | Types and owner | Rule |
|---|---|---|
| `idc.potion` | 29 `alch.potion.*`, base values in doc 08 §12.1–§12.2 | drink `always`, throw `on_effect` |
| `idc.text` | 10 `item.text.*` (doc 11 §3.5; spells doc 06 §8.3) plus the 5 §5.4 texts | per template `use_id`; doc 06 spell texts and every targeted text `always`; `item.text.intuit` identifies on its name prompt (§5.1) |
| `idc.ring`, `idc.amulet`, `idc.relic` | `item.ring.*`, `item.amulet.*`, `item.trinket.*` (doc 11 §3.4) | `equip_id` |
| `idc.wand` | `wand_pool` spells (§4.2) | `always` |
| `idc.implant` | 27 implant-form `aug.*` (doc 04 §9) | `equip_id`; installing is equipping (§5.2) |

- **Equip rule.** An accessory or implant is `display` when every implicit is a character-sheet stat (sub-attribute, resistance, crit term or multiplier, `gear_tol`, armor poise, additive damage, natural armor, or a Canon §6.4 row modifier such as doc 04's `RowFlat` and `RowPost`); otherwise it is `conditional`. The content validator derives `equip_id` from the implicit kinds, and content cannot override it. For doc 11's launch accessories that makes `item.ring.wealth`, `item.trinket.lattice_lens`, `.hourglass`, `.wishbone`, `.merchants_scale`, `.keyring_charm`, `.lodestone` and `.warning_bell` conditional, and every other ring, amulet and trinket `display`. A conditional type is identified by its first observable trigger, reported by the effect's owner (for example the Lattice Lens on the first exit whose third offer exists only because of it, the Hourglass on its first `st.hasted`, the Merchant's Scale on its first changed buy price), or by F-type (§5.7). The Lattice Lens works while merely carried (doc 11 §3.4), so its trigger can identify it unequipped.
- **Malign accessories.** Doc 11 lists none at launch; any type it marks `malign` uses `curse_base` 6000 (§7.1).
- **Targeted texts.** Texts carrying `appr.eff.identify` (any non-mundane target) or `appr.eff.consecrate` (any BUC-bearing target), and doc 06's recharge text (`cat.wand` only).

**Integrated example.** Folio 3, Gothic Horror. Ana (ITU 55) picks up an unknown murky green flask and senses it as CLEAN (2500 Bp chance, §5.11). An apothecary stocks the same appearance at 47 Lucre for her (CHM 50), which leaves healing or haste among the canonical potions (§5.8); she drinks her own flask and healing is identified for the party, since drinking always identifies. Bo equips an unknown ring: `item.ring.keen` is `display`, so its type is identified, but it was cursed with `curse.feeble` at penalty 2 (ring curse chance `mul_bp(2000, 16000) = 3200` Bp here) and binds at full potency (PWR −10). Unbinding at the Colophon's Scriptorium costs `mul_bp(150, 11881) = 178` Lucre.

---

## Exports

All in namespaces Canon §19 assigns to this chapter.

| Namespace | IDs |
|---|---|
| `know.` | core `know.unknown`, `know.classified`, `know.appraised`, `know.identified` (Canon §13.6); new `know.rank.noted`, `know.rank.studied`, `know.rank.familiar`, `know.rank.mastered`, `know.bestiary.sighted`, `know.bestiary.studied`, `know.bestiary.catalogued` |
| `idc.` | core classes `idc.potion`, `idc.text`, `idc.ring`, `idc.amulet`, `idc.wand`, `idc.implant`, `idc.relic`; 176 descriptors `idc.<class>.<name>` listed in §3.3 (32 + 28 + 26 + 18 + 18 + 32 + 22) |
| `appr.tier.` | `novice`, `apprentice`, `journeyman`, `expert`, `master` |
| `appr.use.` | `always`, `on_effect`, `never`, `display`, `conditional` |
| `appr.eff.` | `identify`, `survey`, `omen`, `unbind`, `consecrate`, `intuit`, `amnesia`, `divine_identify`, `divine_omen`, `aura`, `psychometry` |
| `appr.svc.` | `appraise`, `classify`, `identify`, `unbind`, `drop_test`, `consecrate` |
| `appr.reason.` | `use`, `zap`, `equip`, `bind`, `witness`, `deduction`, `text`, `appraisal`, `analysis`, `familiarity`, `service`, `altar`, `glyph`, `sense`, `cross_reference`, `kit`, `craft`, `brew`, `talent`, `prayer`, `story` (21) |
| `buc.` | core `buc.blessed`, `buc.uncursed`, `buc.cursed` (Canon §13.6); no new IDs |
| `curse.` | `leaden`, `thirsting`, `fumbling`, `feeble`, `sluggish`, `frail`, `addled`, `myopic`, `muffled`, `ill_starred`, `blunted`, `misfire`, `dulled`, `loud`, `beacon`, `ravenous`, `brittle`, `bloodprice`, `backlash`, `dread`, `red_letter`, `siphon`, `greed`, `fragile_ward`, `spoiled`, `misprint`, `backfire`, `leaking` (28) |

Also defined here, without new IDs: the `cat.wand` classification and type key (§4), the type-hiding function `idc_of` (§3.1), the appearance shuffle (§3.5), `TypeKnowledge`, `ItemKnowledge` and `ItemView` (§2), `derive_state` (§2.4), `identify_type`, `reveal`, `identify_instance` and `identify_one_unknown` (§5), `roll_curse` (§7.1), `worth_true`, `value_vendor` and `unread` (§6), `price_for_base` and `candidates` (§5.8), content errors `idc_pool_exhausted`, `idc_slot_mixed`, `idc_base_type_missing`, and text keys `loc.idc.<class>.<name>.name` and `loc.idc.<class>.form.<genre_suffix>` (strings owned by doc 18).

## Interface assumptions

| Doc | Assumed |
|---|---|
| 03 | appends `descriptor: Id32`, `flags_new: u32`, `reason: Id32` to the §14.2 `ev.item_identified` payload; runs this chapter's appraisal and familiarity passes inside `sys.needs` (system 27, ticks `t ≡ 53 (mod 60)`) after doc 04's needs work, iterating Errata in ascending `EntityId`; resets per-actor action `seq` counters in `sys.tick_begin` |
| 04 | §5.3 Appraisal perks as hooked in §5.5 (25: tier-1 set makes value exact; 50: tier-0 set on pickup; 75: `RowFlat(36, 2000)`; 100: carried equipment identified after 600 ticks); `anl_speed_bp`; `gumshoe` scales `need` × 5000; appraisal XP rows; talent `prophecy` calls `identify_type` on a chosen carried item with `appr.reason.talent`; `avatar_of_fate` doubling as §5.12 states; implant install is an equip for §5.2 |
| 05 | locks weapon switching for bound weapons; sums `curse.misfire` into the failure roll; applies `curse.bloodprice` and `curse.backlash` hits; reports the killing weapon instance for familiarity |
| 06 | adds essences `glyph.ess.appraise`, `glyph.ess.omen`, `glyph.ess.psychometry` (school divination) and makes `glyph.ess.sight` apply `appr.eff.aura`; tags at most 16 spells `wand_pool` and draws wand spells only from them; text effects set `observable`; the Cosmic Horror patron boon calls `identify_one_unknown` with `appr.reason.prayer`; prayer need (4) and `prayer.absolution` uncurse by §7.6 |
| 07 | `mat_value_bp` per material (iron 10000); crafted items created with every required flag set and uncursed; durability honors `curse.brittle`; altar blocks record whether worldgen or an Erratum placed them |
| 08 | §12.4 drink, throw, brew and Alchemy-hook rules as used in §5.1; holy-water, unholy-water and `alch.tx` curse-state rules as listed in §7.6; base values (`Σ value(e)`) for the 13 potions of §12.2 |
| 09 | per-observer sight results at most 2 ticks old for witness ID; noise and dynamic-light hooks for `curse.loud` and `curse.beacon` |
| 11 | adds `item.text.survey` (26), `.omen` (20), `.consecrate` (34), `.intuit` (28), `.amnesia` (18) and maps `item.text.appraisal` to `appr.eff.identify` and `item.text.sanctify` to `appr.eff.unbind`; calls `roll_curse(&mut it, rng, local_myt, req.source)`; feeds `value_vendor(view)` into every value-scaled price (§12.2 buy and sell, §15 Rewrite, §16.3 reforge) and reads `unread` and "known cursed" from §6.2; replaces its §12.3 "Identify one item" and "Uncurse one item" rows with `appr.svc.identify` and `appr.svc.unbind`; owns `sealed` and creates Reliquary, Engine and Keepsake items with `RARITY` set; malign generation shares of §8; adds `zaps_total: u32` to the instance record |
| 12 | Scriptoria in rooms and every Colophon; patron altar blocks; the story relic of a solved story calls `identify_instance` with `appr.reason.story` |
| 13 | NPCs use type-hidden items through `ai.action.use_item` and emit the outcome for witness ID; Editors honor `curse.red_letter`; enemy templates for the Bestiary |
| 14 | Codex knowledge thresholds (§5.5); Calling lore types (§2.4.3); `run.boon.appraising_eye` (§3.9); Daily, Weekly and Seeded flags for Cross-Reference (§8); `unlock.daily_baseline`; `gp.shop_price_bp` |
| 15 | Keepsake curse rolls through doc 11 with the source genre's MYTHOS; Satchel materialization sealed; `pass.xp.codex_entry` reads Index progress only |
| 16 | `ItemView`-only replication with host authority (§2.8); every `rng.ident` draw on the host |
| 17 | `input.examine`, appraisal focus lock, the Ledger panel, §6.4 tooltips, the sealed view |
| 18 | sigil atlas, descriptor art `art.idc.<class>.<name>`, form nouns per genre, curse-state skins, stamp and name review |
| 19 | schemas for every §2 struct, the descriptor record and the Index record |

## Canon gaps

1. **Item template IDs.** `item.` belongs to doc 11 (Canon §19), yet doc 08 names potions `alch.potion.*` while doc 11 §3.5 lists `item.potion.*`. This chapter keys on type keys whatever their namespace and uses doc 08's IDs and base values.
2. **Potion rosters.** Doc 11 §3.5 lists 12 potions with no harmful type and values that differ from doc 08 §12.1. This chapter needs doc 08's 29-type roster for price identification and the §8 malign share, and doc 08 must still publish the §12.2 potions' base values.
3. **`rng.ident` uses.** Canon §3.3 lists appraisal error and curse sensing; as owner this chapter adds the §5.12 uses and asks the canon table to list them.
4. **Object-keyed constants.** Canon §3.3 keys action draws only. Assumed valid: folio = `est_key >> 40`, actor = `est_key`, tick = `u64::MAX`, seq = purpose code. Completion-tick draws (Analysis, pickups) use the per-tick action counter as `seq` (§5.12).
5. **Names.** Canon §20 calls the battle pass "Codex"; following doc 14 §5.5 the encyclopedia is the Index (doc 15 says "Item Codex"; DESIGN_BRIEF §4 "item codex") and the Bestiary. "Recollection" is renamed Cross-Reference (§12.3), resolving doc 14's Canon gap 16.
6. **Mundane categories.** Canon §13.6 starts them at `know.classified`; this chapter makes that state terminal and complete. Canon-1.4 replaced `cat.block` with `cat.placeable`, which is mundane.
7. **Item contract.** Canon §13.9 lacks `zaps_total` and the `ItemKnowledge` sub-fields (`est_key` included); assumed added by doc 19. Doc 11 §2.2 adds `sealed`.
8. **Tick schedule.** Canon §2.2 and doc 03 §2.2 have no appraisal or familiarity system; doc 03 fixes exactly 35 systems, so both run inside `sys.needs`.
9. **§6.4 rows.** Appraisal rate, Analysis chance and psychometry chance are defined here; if Canon §6.4's "derived stats only" rule covers them, they become rows 45 to 47 unchanged.
10. **Blessing chance.** Canon §5.7 has no MYTHOS term for it; kept flat, 0 at MYTHOS 0.
11. **Wand types and archetype.** Canon §13.6 says "every `cat.wand` type"; doc 06 makes every wand one archetype holding a spell, so the type is the spell (§4.2). `arch.ranged.wand` is listed by doc 11 §3.1 but sits in doc 05's `arch.ranged.` namespace; doc 05 must export it.
12. **Binding and item loss.** Severing, ghost loss and Redaction (Canon §9.1, §17.6, §17.9) are read as the slot or item leaving, not an unequip, so binding ends without uncursing.
13. **Misfire.** `curse.misfire` raises the Canon §5.5 failure chance above 0, so the single per-activation roll still applies.
14. **Events.** No new `ev.` IDs. Doc 03 §14.2 owns the `ev.item_identified` payload; this chapter needs `descriptor`, `flags_new` and `reason` appended (§5).
15. **Stored enhancement.** Canon §13.7 makes stored enhancement Emendation-driven and §13.9 lists an enhancement knowledge flag. This chapter reads stored enhancement as public and gives `ENH` the meaning "`eff_enh` known", set with `BUC` (§2.1). If a later canon adds found enhancement, `ENH` becomes an independent flag revealed at appraisal tier 2 and familiarity F2.
16. **Value ownership.** Canon §13.9 gives the stored `value` to doc 11, whose §13.1 formula has no curse, blessing or charge term, so a value range would hide nothing. This chapter keeps doc 11's `value` unchanged and adds `worth_true` and `value_vendor` (§6). Conflict: doc 11 §12.2, §15 and §16.3 price from stored `value`, which reads rarity, a hidden property; they must read `value_vendor`.
17. **Service prices.** Doc 11 §12.3 prices "Identify one item" as `max(mul_bp(10, P(d)), mul_bp(value, 1000))` and "Uncurse one item" as `mul_bp(value, 5000)`; both read a hidden property. `appr.svc.identify` and `appr.svc.unbind` (§5.9), in this chapter's `appr.` namespace, replace them.
18. **Unappraised sale.** Doc 11 §12.2's "× 5000 if not `know.appraised`" would halve every mundane sale, because mundane items stay `know.classified`. This chapter defines the test as `unread` (§6.2), which exempts mundane items and known-type potions and texts.
19. **Foreign-namespace IDs.** This chapter uses or proposes, without owning: `glyph.ess.appraise`, `glyph.ess.omen`, `glyph.ess.psychometry` (doc 06); `item.text.survey`, `.omen`, `.consecrate`, `.intuit`, `.amnesia` (doc 11); `arch.ranged.wand` (doc 05); `run.boon.appraising_eye`, `gp.shop_price_bp` (doc 14); `input.examine` (doc 17); `art.idc.*` (doc 18). Each owner must add or confirm them.
20. **Prayer uncursing.** Doc 06 §16 uncurses only the lowest-`uid` cursed equipped item; this chapter's earlier "every equipped item" boon is withdrawn.
21. **Erratum-placed altars.** Doc 06 §16 lets an Erratum place an altar, which Canon §4.3's placeable list omits. This chapter refuses every `appr.svc.*` at such an altar, so free BUC testing stays tied to worldgen altars.
22. **Witness timing.** Doc 09 does not exist yet; witness ID reads the observer's latest sight result, at most 2 ticks old, from doc 03's `sys.perception` period of 3.

## Acceptance tests

Run seed for keyed tests: `0x0123456789ABCDEF`.

1. **Appearance stream.** `stream_seed(run, "rng.appearance", 0) = 0x95b9ab50b5b43f53`.
2. **Amulet permutation.** The sorted 18 amulet descriptors shuffle to `teardrop, heart, bell, fang, acorn, spindle, lozenge, cage, arrowhead, crescent, coin, key, feather, knot, shell, wheel, disc, star`.
3. **Assignments.** Doc 11's amulets: composure → teardrop, elements → heart, ferocity → bell, fortune → fang, sanctuary → acorn, scholar → spindle, vigor → lozenge, wayfarer → cage. The first descriptor of each later shuffled pool: implant `twin_moons`, potion `rose`, relic `laurel`, ring `amethyst`, text `yrrel`, wand `copper`. Rings: `item.ring.keen` → `jet`, `item.ring.will` → `black_iron`.
4. **Draw count.** Exactly 169 `range` calls, whatever the type counts.
5. **Pool budget and minimums.** 31 potion types fail load with `idc_pool_exhausted`; doc 08's 29 load; 17 `wand_pool` spells fail; 25 ring types fail. The loaded pools have 32 potion, 28 text, 26 ring (20 stones), 18 amulet, 18 wand, 32 implant and 22 relic descriptors, all 183 `idc.` IDs with distinct `fnv1a32`.
6. **Type hiding.** `equip_slots [ring_l, ring_r]` → `idc.ring`; `[neck]` → `idc.amulet`; `[trinket]` → `idc.relic`; `[belt]` → none; a staff → none; `[neck, trinket]` → `idc_slot_mixed`. Two wands holding the same spell share one type key.
7. **Derived state.** Unknown type with `BUC` set: `know.unknown`; after `identify_type` with only RARITY set: `know.appraised`; a freshly generated potion of known type with `BUC` unset: `know.appraised` (its `RARITY` was set at creation); a `cat.placeable`: `know.classified` forever, never a target.
8. **Value and worth.** The four §6.1 items give `value` 624, 303, 720, 42 and `worth_true` 661, 242, 633, 52.
9. **Vendor value and sale.** The §6.1 longsword with only quality visible: `value_vendor = 155`; it sells for 23 at CHM 40, Grade 0 (`155 → 46 → 23`); identified, 198. The §6.1 wand with `CHARGES` unknown: `value_vendor = 151`. Identified items always have `value_vendor == worth_true`. A mundane item and a known-type potion are never `unread`.
10. **Estimate.** `est_key (7 << 40) | 42` has `u = −6692`. With `V = 661` and `BUC` unknown: (skill 0, steps 0) "≈ 190–860"; (20, 0) "≈ 380–780"; (20, 2) "≈ 520–720"; (0, 4) "≈ 470–740"; (24, 4) "≈ 610–680". With `BUC` set: "661".
11. **Containment.** For every `V` in `0..=3000`, `err_bp` in `{500, 1000, 2600, 5000}` and `u` in `{−10000, −1, 0, 1, 10000}`, `lo <= V <= hi` and the rounded display also contains `V`.
12. **Appraisal time.** Weapon `ilvl 7`, skill 20, ITU 50, ANL 40: need 23,600, rate 1,700, 14 passes; ANL 70: rate 2,210, 11 passes. Ring `ilvl 25`, skill 0, ITU 40: 52 passes. Master type step, ring `ilvl 10`, second completion: need 57,150, 14 passes. `rate(0, ITU 1, ANL 40) = 610`.
13. **Descent and perks.** On `ev.folio_exited` the test-12 weapon at skill 20 gains 7,080 progress. An Appraisal-50 player's pickup sets `RARITY` and `est_skill = 50` with no work and no draw. An Appraisal-100 player's carried ring is identified, type included, after 600 ticks.
14. **Analysis.** ANL 60, appraisal 30, `ilvl 8`: `anl_bp = 5000`. Keyed folio 8, actor `(8 << 40) | 3`, seq 0: tick 50000 rolls 6052 (fail); tick 50001 rolls 2404 (2 reveals: `RARITY`, then `BUC` with `ENH`). ANL 40, skill 0, `ilvl 25`: 500. A second Examine by the same slot at the same level is refused without a draw.
15. **Curse sense.** ITU 55: 2500 Bp. Keyed folio 2, actor `(2 << 40) | 1`, tick 9000, seq 0 rolls 4498: no flag, `SENSED[s]` set; a second pickup by that slot draws nothing.
16. **Curse resistance and binding.** PRV 70: 1500 Bp; PRV 40: 0, still one draw; an `OMEN` item draws nothing. An unknown cursed ring with a failed roll equips, binds and sets `BUC`, `ENH`, `AFFIX_NAME[c]`, `AFFIX_ROLL[c]`; unequip is refused.
17. **`roll_curse`.** Exactly 3 draws for every outcome and category. Chances: weapon Gothic Horror 2400; weapon Noir 600; ring Noir 800; ring Mythic Fantasy 2300; ring Mythic Antiquity 3500; potion Fairy Tale 1450; malign ring Cosmic Horror 9000; Legendary weapon Mythic Antiquity 1312; anything in The Unwritten 0; an item of Unwritten origin generated in a seam zone with local MYTHOS 8: 0; in-run Reliquary contents 0. `curse_bp + bless_bp <= 10000` for every input.
18. **Curse potency.** `curse.feeble`, `p = 3`, Gothic Horror ring, Gothic Horror Origin wearer, ATN 40: PWR −15 in Mythic Antiquity, −10 in Cyberpunk, −9 in Noir. At `C >= 6` the magnitude is 0, the item stays bound and `eff_enh` still subtracts the penalty.
19. **Price observation.** Healing (base 30), `ilvl 3`, apothecary markup 14000, buyer CHM 50, Grade 0, no haggle: price 47; among canonical bases 10, 12, 15, 20, 25, 30, 35 only 30 gives 47, so the candidates are healing and haste plus any §12.2 potion whose base also gives 47.
20. **Use rules.** Drinking an unknown potion always identifies it and sets every required flag of the drunk instance; a thrown potion that touches no entity and causes no reaction does not, and adds nothing to `tried_no_effect`. An untargeted text read with no observable result adds its descriptor to `tried_no_effect`.
21. **Deduction.** A single candidate is identified the same tick with reason `appr.reason.deduction`. With every amulet type but one known and no unknown amulet descriptor seen, nothing is identified.
22. **Text refusal.** An unknown targeted text pointed at a potion is not consumed, sets `read_mode = Targeted` and the potion row in `refused_cats`, and its candidates lose every identify and consecrate text.
23. **Services at depth 12, Grade 0.** appraise 51, classify 103, identify 154, unbind 387, consecrate 774. At Grade 5 (`gp.shop_price_bp = 11500`) identify costs `mul_bp(154, 11500) = 177`. An Erratum-placed altar refuses `appr.svc.drop_test`.
24. **Cursed texts.** Cursed identify, keyed folio 5, actor `(5 << 40) | 9`, tick 30000, seq 0 over 5 items picks index 3; blessed identify draws nothing. Uncursed amnesia over known non-text types `[A..G]`, keyed folio 4, actor `(4 << 40) | 2`, tick 20000, seq 0, forgets B and D; texts stay known.
25. **Wands.** Zapping an unknown wand identifies its spell type for every wand holding that spell; a zap at 0 charges sets `CHARGES`. A generated cursed wand's curse is one of `curse.backfire`, `curse.bloodprice`, `curse.leaking`, `curse.misfire`. A cursed wand in the backpack can be dropped; in `slot.offhand` it cannot be unequipped.
26. **Familiarity.** F1 at 1800 ticks or 3 kills reveals `RARITY`, `BUC` and `ENH`; affix slot 1 (F4) at 7200 or 16; F-type at 7200 or 15. F2 reveals nothing on a weapon. Another weapon's kills never count for a weapon.
27. **Seals.** A Reliquary item is created with only `RARITY` set and `sealed = true`; its descriptor is not in `seen`; opening its detail view clears `sealed` with no draw, adds the descriptor to `seen` and makes the holder's curse-sense roll; in-run Reliquary and Fortune Engine contents are never cursed; a Keepsake from a Gothic Horror Reliquary rolls curses with MYTHOS 9.
28. **Illumination.** An illuminated Legendary ring's unlock is recorded on pickup; notification and render wait for `RARITY` or run end; worth and every price are unchanged by `illuminated`.
29. **Cross-Reference.** Allowed once per Volume per slot at a Scriptorium; refused in Daily mode; a wrong guess adds one exclusion, runs deduction and draws nothing; a type below `know.rank.familiar` cannot be named.
30. **Self-harm cap.** A drunk harmful potion at depth 25 against a 220 HP Erratum deals at most 66,000 mHP to the drinker; so does a `curse.backfire` zap.
31. **Unread Edge.** At `d = 10`, SER 40 with 3 unidentified equipped items gives weights `[7000, 3124, 1104, 339, 120, 46, 0]`; under `st.unmoored` it gives the SER-40 weights `[7000, 2992, 1032, 312, 109, 42, 0]`.
32. **Stacks.** Two otherwise identical potions, one with `BUC` set, do not merge. Splitting a stack of 4 copies `ItemKnowledge`, so both stacks show the same estimate (`est_key` unchanged), and the split stack cannot be examined again at the same level.
33. **Replication.** An unappraised weapon's client packet has no rarity, enhancement, curse or affix fields; an unknown-type potion's has its descriptor `id32` and no template `id32`; a sealed item's has neither. An invader receives no party type knowledge.
34. **No hidden inputs.** Every function returning a displayed chance, time, price or service cost takes `&ItemView`; `ItemView` has no field for an unrevealed property.
35. **Flag coupling.** Setting `BUC` on a cursed item sets `ENH` and `OMEN`; on an uncursed item `ENH` and `CLEAN`. No code path sets `ENH` alone. Uncursing sets `CLEAN` and clears `OMEN`.
36. **Blood price.** `curse.bloodprice` at `p = 1`, Max HP 85,000 mHP: cost 1,000 mHP; at `p = 3`: 1,275 mHP; at HP 1,800 mHP a `p = 3` payment leaves 1,000 mHP; a second activation within 30 ticks costs nothing.
37. **Curse-state machine.** A cursed consecrate on a blessed item makes it uncursed; on an uncursed item it curses it with exactly 2 draws and binds it if equipped; an Emendation on a cursed item uncurses it and leaves `enhancement` unchanged.
38. **Event payload.** A Scriptorium identification of uid `(3 << 40) | 7` emits one `ev.item_identified` with `uid` set, `flags_new` equal to the newly set bits, `reason = appr.reason.service` and the new state; a type identification emits `uid = 0` and `template` = the type key.
39. **Malign share.** In doc 11's loot tables, the summed `template_weight` of the seven harmful potion types is exactly 3000 Bp of the `cat.potion` total, and `item.text.amnesia` is exactly 1000 Bp of the `cat.text` total.

## Invariants annex

```
Σ doc10: knowledge ⊗ appearances ⊗ methods ⊗ value ⊗ curses ⊗ seals ⊗ index
W! type_hidden(t) ⇔ idc_of(t) ≠ ∅                 W! |types_c| ≤ |pool_c| − 2 ∀ class c
W! shuffle := rng.appearance, run scope, once     W! shuffle draws = Σ(|pool_c| − 1) = 169
W! type knowledge ∈ party scope                   W! instance knowledge ∈ item
W! state == derive_state(flags, type known)       N! state lowered except by amnesia
W! hidden property ⇒ fully active                 N! displayed chance|time|cost|price ← hidden property
W! vendor knows type ⇒ price observation          W! value_vendor == worth_true when identified
W! stored value := doc 11 §13.1                   N! price ← stored value
W! ENH set ⇔ BUC set                              W! stored enhancement public
W! potion|text RARITY set at creation             W! est_key survives split
W! wand type key := its spell                     W! potion drink ⇒ type identified
W! every rng.ident draw keyed; count per §5.12    W! roll_curse draws == 3
W! curse chance == 0 when local|origin MYTHOS == 0 W! curse affix = last affix ∧ dep {mythos}
W! bound ⇔ cursed ∧ equipped (any potency)        N! Contradiction releases binding
W! Reliquary|Fortune Engine content ⇒ ¬cursed     W! sealed item ⇒ RARITY known at creation
W! estimate contains worth_true                   W! u constant per est_key
W! seal ⇒ content unchanged                       N! seal ∈ {sold, timed, money-accelerated}
N! sealed descriptor ∈ seen                       N! curse.bloodprice|backlash kills
W! Index|Bestiary ← in-run play only              N! Index|Bestiary ← SKU|Inkwell|Reliquary opening
N! appearance mapping persists across runs        W! Cross-Reference off in leaderboard modes
W! self-harm per consumable use ≤ 3000 Bp Max HP  W! client receives ItemView only
N! invader ← party knowledge                      N! unrevealed property ∈ client packet
N! appr.svc.* at an Erratum-placed altar          W! service cost := f(depth, Grade) only
```
