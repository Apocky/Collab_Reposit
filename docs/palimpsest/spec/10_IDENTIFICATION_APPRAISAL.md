# 10 — Identification and Appraisal

As of 2026-10-11. Status: BINDING. Conforms to canon-1.2.

This chapter owns item knowledge and how it grows, item value, curses and blessings, `cat.wand` and its type-hiding rule, sealed Reliquary contents, and the Concordance (item encyclopedia and bestiary).

## Imports

| Source | Consumed |
|---|---|
| Canon §1.1, §1.3, §1.5, §1.6, §1.10, §1.11 | ID grammar and `id32`, `floor_div`, `ceil_div`, `mul_bp`, display units, integer content, text keys, `uid` layout and ascending-uid iteration |
| Canon §2.2, §2.3 | the Needs pass (period 60, phase 7), `in_combat` |
| Canon §3.1–§3.5 | `Rng`, `stream_seed`, `keyed_rng`, streams `rng.appearance` and `rng.ident` (owner 10) and `rng.loot` (owner 11), draw-count rule |
| Canon §5.3–§5.8 | dependency sets, `Req`, `Tol`, Contradiction potency, the MYTHOS curse-chance term, local axioms |
| Canon §6.4 rows 17, 28, 36, 37, 38, 42, 44; §7; §11.8 | use speed, XP multiplier, curse sense, buy and sell multipliers, rarity shift, curse resistance; `skill.appraisal`, `skill.divination`, mastery tiers; player sleep scaling |
| Canon §13.1–§13.9, §14, §16.4, §16.5, §17.1, §17.5, §17.6 | categories, slots, rarity, quality, knowledge and curse enums, `idc` classes, the shuffle, Emendations, `P(d)`, item contract, `illuminated`; Lucre; cure price; patrons and favor; Volumes; loot modes; ghost item loss |
| Canon §18, §19 | `ev.item_identified`, `ev.item_picked_up`, `ev.item_used`, `ev.entity_killed`, `ev.folio_entered`, `ev.folio_exited`, `ev.run_ended`; ownership of `know.`, `idc.`, `appr.`, `buc.`, `curse.` |
| Docs 04, 06, 08, 11, 15 | kits and skill XP; wand effects, glyphs and prayer; potion effects; item generation, shops and in-run Reliquaries; account Reliquaries |

Docs 04 to 19 do not exist yet; what this chapter needs from them is under Interface assumptions.

---

## 1. Design rules

1. **Hide magic, not mundane type.** A dropped rifle reads "Rifle". Only the seven `idc` classes hide their type (Canon §13.6). Everything else hides only rarity, enhancement, curse state, affixes and charges.
2. **Hidden properties are always active.** An unknown curse binds, an unknown +3 hits harder, an unknown ring works. Knowledge changes only what the UI shows and what vendors pay.
3. **No displayed chance, time or price depends on a hidden property**, except that vendors know true types (price identification, §5.8). Functions computing displayed numbers take an `ItemView` (the knowledge-filtered view), never an `Item`.
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
| `cat.potion` | yes (`idc.potion`) | yes | yes | no | no | no |
| `cat.text` | yes (`idc.text`) | yes | yes | no | no | no |
| `cat.mount`, `cat.part` | no | yes | no | no | no | yes |
| every other category (**mundane categories**) | no | no | no | no | no | no |

- **Mundane categories** (`cat.ammo`, `cat.book`, `cat.food`, `cat.throwable`, `cat.gadget`, `cat.tool`, `cat.material`, `cat.ingredient`, `cat.container`, `cat.block`, `cat.key`, `cat.currency`, `cat.quest`, `cat.seed`, `cat.corpse`, `cat.body_part`) start and stay at `know.classified` (Canon §13.6), which for them is complete: exact value shown, no "?" marks, never a target, always `buc.uncursed`.
- **Affix slots** are the entries of the instance's `affixes` list in list order, at most 8. A curse affix is always the last entry (§7.1). A Legendary perk and an Exotic intrinsic are not affix slots; they are revealed with RARITY.
- Quality, material, manufacturer, `ilvl`, `origin_genre`, `gear_tier` and mass are never hidden.

### 2.2 Type knowledge (run-scoped, party-shared)

```rust
pub struct TypeKnowledge {                         // one per run, host-authoritative, saved
    pub assign:   BTreeMap<Id32, Id32>,            // type key -> descriptor (§3.5), fixed at run start
    pub by_desc:  BTreeMap<Id32, Id32>,            // descriptor -> type key, inverse of assign
    pub known:    BTreeSet<Id32>,                  // identified type keys
    pub seen:     BTreeSet<Id32>,                  // descriptors the party has encountered (§5.3)
    pub tried_no_effect: BTreeSet<Id32>,           // descriptors used once with no observable outcome
    pub read_mode: BTreeMap<Id32, ReadMode>,       // texts only: Targeted | Untargeted, from the read prompt
    pub refused_cats: BTreeMap<Id32, u32>,         // texts only: bitmask of §13.1 row indices a targeted text refused
    pub price_obs: BTreeMap<Id32, Vec<PriceObs>>,  // ≤ 8 per descriptor, oldest dropped first (§5.8)
    pub excluded: BTreeMap<Id32, BTreeSet<Id32>>,  // descriptor -> type keys proven wrong
    pub recollect_volume: [u8; 4],                 // per party slot: Volume of the last Recollection, 0 = none
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
    pub est_skill: i16,          // best appraisal skill used in a completion, -1 = no estimate (§6.3)
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
| 2 | `ENH` | stored enhancement known |
| 3 | `CHARGES` | current and max charges known |
| 4 | `OMEN` | known to be cursed; penalty unknown |
| 5 | `CLEAN` | known not to be cursed; blessed versus uncursed unknown |
| 6 | `AURA` | magic class known: mundane, storied or ominous (§5.10) |
| 7 | reserved | always 0 |
| 8..=15 | `AFFIX_NAME[i]` | identity of affix slot `i` |
| 16..=23 | `AFFIX_ROLL[i]` | roll of affix slot `i` (for a curse affix, its severity) |
| 24..=27 | `SENSED[s]` | party slot `s` has already made its curse-sense roll (§5.11) |
| 28..=31 | reserved | always 0 |

Setting `BUC` also sets `OMEN` when cursed, else `CLEAN`. Removing affix slot `i` clears its two bits; because only the last slot (the curse) is ever removed, no shift is needed.

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

Instance flags persist while the type is unknown, so an unknown-type item with `BUC` set still shows `know.unknown`.

| From | To | Trigger |
|---|---|---|
| `know.unknown` | classified, appraised or identified, as its flags support | type identified (identify effects also set every required flag) |
| `know.classified` | `know.appraised` | RARITY revealed: appraisal tier 0, familiarity stage F1, Analysis, `appr.eff.survey`, shop stock, or creation as a sealed item (§11) |
| `know.classified` or `know.appraised` | `know.identified` | the last missing required flag is revealed |
| `know.identified` | `know.identified` | a party action changes a property (Emendation, consecration, recharge, reforge, Rewrite); the new value is shown, so its flag is set |
| any of classified, appraised, identified | `know.unknown` | `appr.eff.amnesia` forgets the type (type-hidden only); instance flags are kept |

A property changed by an unseen source (none at launch) clears its flag. Nothing else lowers a state.

### 2.5 Sharing, stacks, trading and joining

- Knowledge travels with the item through drops and trades, identically in instanced loot and `mode.loot_shared` (Canon §17.5).
- Items stack only when template, `buc`, `curse_penalty`, `enhancement`, `charges`, `affixes`, `ilvl`, `quality` and `knowledge.flags` are all equal. Splitting copies the knowledge.
- Type identification updates every instance of that type in the run at once, on the ground or carried.
- A player who joins mid-run shares `TypeKnowledge` from the join tick. A ghost (Canon §17.6) cannot pick up, appraise or analyze, but keeps seeing party knowledge.

### 2.6 Starting knowledge

- Kit items (doc 04, doc 14) are created with every required flag set. After the shuffle, each Calling's two lore types (doc 14) and every type-hidden kit item's type join `known` with reason `appr.reason.kit`.
- Crafted items (doc 07), brewed potions (doc 08) and texts inscribed by a party member (doc 06) are created with every required flag set and their type known. Brewing a canonical type identifies it (reason `appr.reason.brew`, called by doc 08).
- Shop stock (§10.1) and sealed items (§11) arrive with RARITY set, so the outcome tier of every random container is visible.

### 2.7 What persists across runs

Only the Index and Bestiary (§12) persist: templates and types seen and identified, familiarity points and ranks, bestiary counts, curse affixes revealed and illuminated skins (Canon §13.9). The appearance mapping, type knowledge, the Ledger, instance flags and all progress counters die with the run.

### 2.8 Replication

The host owns all knowledge state. Doc 16 sends each client only the `ItemView` of every item: properties whose flags are unset are omitted from packets, not merely hidden in the UI. Type-hidden items are sent with their descriptor ID and without their template ID until the type is known.

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
| `idc.text` | 28 | 26 | 10 `item.text.*` (doc 11) plus 5 proposed doc-10 texts (§5.4) |
| `idc.ring` | 22 | 20 | 15 (doc 11) |
| `idc.amulet` | 18 | 16 | 8 (doc 11) |
| `idc.wand` | 18 | 16 | `wand_pool` spells (§4.2) |
| `idc.implant` | 32 | 30 | 27 implant-form `aug.*` (doc 04) |
| `idc.relic` | 22 | 20 | 18 `item.trinket.*` (doc 11) |

At least two descriptors per class are always unassigned. These **decoys** keep elimination honest: identifying every type but one never reveals which appearance the last type wears. A mod that adds types must add descriptors so that `|types| ≤ |pool| − 2` holds, or the content set is rejected with `idc_pool_exhausted`. Descriptor IDs are `idc.<class>.<name>`.

### 3.3 Descriptor lists

**Potions (32).** Suffix and render color, in sorted order (index 0 to 31): `amber` #FFB000, `azure` #3A8DFF, `bistre` #3D2B1F, `black` #111111, `brick` #A23B2C, `charcoal` #36454F, `chartreuse` #7FFF00, `clear` #E8F4F8, `cobalt` #0047AB, `crimson` #DC143C, `golden` #FFD700, `indigo` #4B0082, `ivory` #FFFFF0, `lilac` #C8A2C8, `lime` #BFFF00, `magenta` #FF00FF, `maroon` #800000, `mint` #98FF98, `murky_green` #556B2F, `ochre` #CC7722, `orange` #FF7F00, `pink` #FF8FB1, `plum` #8E4585, `quicksilver` #D9D9D9, `rose` #FF007F, `saffron` #F4C430, `scarlet` #FF2400, `sky_blue` #87CEEB, `smoky` #738276, `teal` #008080, `turquoise` #30D5C8, `violet` #8F00FF. Accessibility cue: with `i` the sorted index, stopper shape = `i % 4` (round, square, spire, wax seal) and label mark = `i / 4` (dot, bar, cross, ring, chevron, star, wave, diamond), so no two potions share a shape-and-mark pair and color is never the only signal.

**Text sigils (28).** Each is a nonsense incipit drawn from a committed 28-glyph sigil atlas (doc 18): `ambreth`, `ankaru`, `balux`, `belleth`, `corvent`, `drema`, `elixtor`, `fennick`, `gormal`, `hesper`, `illoq`, `jarrow`, `keste`, `lumir`, `mordant`, `nulzeph`, `obrax`, `pellin`, `quorval`, `runnel`, `sabeth`, `tirrex`, `ulma`, `vorne`, `wexel`, `xanth`, `yrrel`, `zolmet`.

**Ring stones and metals (22).** Stones: `agate`, `amethyst`, `bloodstone`, `carnelian`, `diamond`, `garnet`, `jade`, `jet`, `lapis`, `malachite`, `moonstone`, `obsidian`, `onyx`, `opal`, `ruby`, `sapphire`. Metals: `black_iron`, `bronze`, `electrum`, `red_gold`, `twisted_silver`, `white_gold`.

**Amulet forms (18).** `acorn`, `arrowhead`, `bell`, `cage`, `coin`, `crescent`, `disc`, `fang`, `feather`, `heart`, `key`, `knot`, `lozenge`, `shell`, `spindle`, `star`, `teardrop`, `wheel`.

**Wand woods and alloys (18).** `ash`, `bamboo`, `birch`, `bone`, `brass`, `cedar`, `copper`, `coral`, `ebony`, `glass`, `horn`, `oak`, `pewter`, `rowan`, `silver`, `steel`, `willow`, `yew`.

**Implant stamps (32).** `anvil`, `barbed_ring`, `bent_nail`, `bird_foot`, `broken_wheel`, `chain_link`, `cog_tooth`, `coiled_serpent`, `comb`, `crossed_keys`, `crown_dot`, `double_bar`, `fishhook`, `half_sun`, `hollow_triangle`, `horseshoe`, `hourglass`, `interlock`, `keyhole`, `ladder`, `lantern`, `lightning_fork`, `needle_eye`, `open_eye`, `paw_print`, `quill_mark`, `seven_dots`, `shield_notch`, `spiral`, `star_burst`, `twin_moons`, `wave_line`. Stamps are fictional marks and never match a manufacturer (`mfr.`) mark or a real brand (doc 18 name review).

**Relic carvings (22).** `antler`, `bee`, `boar`, `crane`, `fish`, `gate`, `hand`, `horned_mask`, `knotwork`, `labyrinth`, `laurel`, `lotus`, `moth`, `oak_leaf`, `owl`, `rams_head`, `ship`, `skull`, `stag`, `sun_disc`, `tower`, `weeping_face`.

No suffix repeats across classes. All 179 `idc.` IDs (7 classes, 172 descriptors) have distinct `fnv1a32`.

Descriptor record:

```json
{ "id": "idc.potion.crimson", "class": "idc.potion", "rgb": 14423100, "sort_index": 9,
  "stopper_shape": 1, "label_mark": 2, "art": "art.idc.potion.crimson" }
```

`rgb` is `0xDC143C` as an integer. `stopper_shape` and `label_mark` must equal `sort_index % 4` and `sort_index / 4`; the validator checks them.

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

- **Draw count:** `Σ (|pool_c| − 1)` calls of `range` over the seven classes, whatever the number of types. With the §3.2 pools that is 17 + 31 + 31 + 21 + 21 + 27 + 17 = **165** calls. A call consumes more than one `u64` only on a Lemire rejection.
- **Content set.** The host's content set at run start, recorded in the save (Canon §13.6). Daily and weekly runs use `unlock.daily_baseline`, so every player of a daily seed sees the same mapping.
- **Worked example.** Run seed `0x0123456789ABCDEF`. `stream_seed(run, "rng.appearance", 0) = 0x95b9ab50b5b43f53`. The permutations depend only on the seed and the pool sizes. `idc.amulet` goes first and becomes `teardrop, heart, bell, fang, acorn, spindle, lozenge, cage, arrowhead, crescent, coin, key, feather, knot, shell, wheel, disc, star`. Doc 11's 8 amulets, sorted (`composure`, `elements`, `ferocity`, `fortune`, `sanctuary`, `scholar`, `vigor`, `wayfarer`), take `teardrop` through `cage`; the other 10 are unassigned. The other pools begin: implant `twin_moons`, potion `rose`, relic `laurel`, ring `sapphire`, text `illoq`, wand `bone`.

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

Every wand is the archetype `arch.ranged.wand` (doc 11) holding one authored `spell.` record (doc 06 §8.4). For type hiding, `type_key(wand) = its spell ID`. The `idc.wand` type list of a content set is the set of authored spells tagged `wand_pool`; doc 11 draws wand spells only from that set, and a unique (Exotic) wand must hold a tagged spell. Two wands holding the same spell share type knowledge whatever their skin, genre or rarity. More than 16 tagged spells fails load with `idc_pool_exhausted`.

### 4.3 Charges and zapping

Doc 06 §8.4 owns max charges by rarity, the zap, passive recharge (+1 charge per 2400 ticks carried), the recharge text and service, and snapping. This chapter adds:

- Zapping identifies the wand's type (doc 06): wands use `appr.use.always`. A zap at 0 charges does nothing and sets `CHARGES` ("You feel a faint resistance").
- A recharge (text or service) and a snap reveal `CHARGES`.
- `zaps_total` (`u32`, every zap of this instance) extends the Canon §13.9 contract for `curse.leaking`; the UI shows zaps since pickup, which is not hidden.
- `dep = {origin_genre.aether_axiom}`, or `{arcana}` when that is none (Canon §5.3).

### 4.4 What a wand hides

Its type (the spell), RARITY, BUC, ENH, CHARGES and affixes (§2.1).

---

## 5. Identification methods

`identify_type(type_key, reason, slot)` adds the type to `known`, recomputes every instance's state and emits `ev.item_identified { scope: type, type_key, descriptor, reason, slot }`. `reveal(item, flags, reason, slot)` sets flags and emits `ev.item_identified { scope: instance, uid, flags_new, reason, slot }` only when at least one new flag was set. Reasons are the `appr.reason.*` IDs in Exports. Every `rng.ident` draw is listed in §5.12.

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

- **Witness.** When an NPC uses or zaps a type-hidden item and any Erratum's sight check (doc 09) succeeds on the NPC on that tick, §5.1 runs with the NPC's outcome and reason `appr.reason.witness`.
- **Deduction.** After every change to `TypeKnowledge`, for each descriptor in `seen` whose type is unknown, in ascending `id32` order, compute `candidates(d)` (§5.8). If exactly one type remains, `identify_type` with reason `appr.reason.deduction`. Repeat until a full pass identifies nothing. A descriptor joins `seen` when an instance enters an Erratum's inventory or a trade window, or is used or witnessed.

### 5.4 Doc-10 texts

`item.text.appraisal` carries `appr.eff.identify` and `item.text.sanctify` carries `appr.eff.unbind` (doc 11 §3.5); doc 11 adds `item.text.survey`, `.omen`, `.consecrate`, `.intuit` and `.amnesia` for the rest. "Inventory" includes equipped items. "Random" picks uniformly with one `range(n)` on keyed `rng.ident` (actor = reader, action tick, action seq) over eligible items sorted by `uid`, and draws nothing when `n = 0`.

| Effect | Uncursed | Blessed | Cursed | Draws |
|---|---|---|---|---|
| `appr.eff.identify` (targeted) | 1 chosen item: every required flag, plus type | up to 3 chosen items | 1 random item with missing flags: type only if type-hidden and unknown, else RARITY and BUC | cursed: 1 |
| `appr.eff.survey` | every inventory item: tier-0 reveal set (§5.5), `est_steps += 1` | tier-1 reveal set, `est_steps += 1` | tier-0 reveal set on 1 random item | cursed: 1 |
| `appr.eff.omen` | BUC of every inventory item | also every item carried by a party member or on the ground within 16 m | OMEN or CLEAN on 1 random item | cursed: 1 |
| `appr.eff.unbind` | every equipped cursed item becomes uncursed (§7.6) | also every carried cursed item | nothing; not observable | 0 |
| `appr.eff.consecrate` (targeted) | cursed → uncursed, uncursed → blessed, blessed unchanged | cursed → blessed, uncursed → blessed | blessed → uncursed; uncursed → cursed with a new curse (§7.1 draws B and C); cursed unchanged | cursed: exactly 2 |
| `appr.eff.intuit` | name one seen, unknown descriptor and one candidate type: correct → `identify_type`; wrong → `excluded[d].insert(t)` | two guesses for the same descriptor | the guess resolves, but a wrong guess is not recorded | 0 |
| `appr.eff.amnesia` | forget 2 identified types; set composure to max; end `st.dread` and `st.madness` | forget 1 | forget 3 | `min(k, n)` |

Consecrate and unbind reveal `BUC` on every item they change. A consecrate that changes nothing still glows, so its target's `BUC` becomes known.

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
| 0 | `appr.tier.novice` | RARITY; starts the estimate (`est_skill = max(est_skill, skill)`) |
| 1 | `appr.tier.apprentice` | BUC, CHARGES |
| 2 | `appr.tier.journeyman` | ENH, every `AFFIX_NAME[i]` |
| 3 | `appr.tier.expert` | every `AFFIX_ROLL[i]` (completes identification of every non-type-hidden item); for a type-hidden item of unknown type, an exact price observation (§5.8) |
| 4 | `appr.tier.master` | the type of a type-hidden item |

**Doc 04 mastery perks** plug in here: at 25, an item whose `est_skill >= 25` shows exact `value_true` (§6.3); at 50, the tier-0 reveal set applies on the player's pickup with no work (as does `run.boon.appraising_eye`, doc 14); at 75, +2000 Bp curse sense (§5.11); at 100, every equipment item the player carries for 600 consecutive ticks gets every required flag and, if type-hidden, its type.

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

**Descent.** On `ev.folio_exited`, each Erratum's current focus item gains `mul_bp(need, 2500 + 25 × skill.appraisal)` progress, from 25% at skill 0 to 50% at skill 100, as in Barony.

**Worked example.** A `cat.weapon_melee` at `ilvl 7`, first completion: `need = mul_bp(20000, 11800) = 23,600`. Appraiser at skill 20, ITU 50, ANL 40: `rate = 1000 + 600 + 100 = 1,700` per second, so it completes on the 14th pass (13 passes give 22,100). At ANL 70, `anl_speed_bp = 13000`, rate 2,210, 11 passes. Tier 0 reveals RARITY. At `ilvl 25` a ring at skill 0, ITU 40 needs `51,600` and takes 52 s. A Master (skill 100, ITU 60, ANL 40, rate 4,200) learning a ring's type at `ilvl 10` after an earlier completion needs `mul_bp(mul_bp(mul_bp(30000, 12700), 5000), 30000) = 57,150`: 14 s.

### 5.6 Analysis checks (`sub.analysis`)

An Erratum channels **Examine** (`input.examine`, doc 17) on an item in its inventory for 120 ticks; HP damage, another action or stagger cancels it with no roll. It requires `analysis_lvl[slot] < character level` (once per item per party member per level). On completion `analysis_lvl[slot] = level` and one draw is made: `roll = range(10000)` on keyed `rng.ident` (actor = analyst, completion tick, action seq).

```
anl_bp = clamp(3000 + d(ANL) × 100 + skill.appraisal × 40 − 150 × min(ilvl, 25), 500, 9500)
success  ⇔ roll < anl_bp
reveals  = if success { min(3, 1 + floor_div(anl_bp − 1 − roll, 2500)) } else { 0 }
```

- Non-type-hidden (or type known): each reveal takes the next missing flag in the order RARITY, BUC, ENH, CHARGES, `AFFIX_NAME[0..n]`, `AFFIX_ROLL[0..n]`.
- Type-hidden with unknown type: 1 reveal records an exact price observation (§5.8) and RARITY; 2 also reveal BUC and CHARGES; 3 also reveal the type.
- Every success adds `est_steps += 1` (max 4) and grants doc 04's appraisal XP; a failure grants 0.
- **Worked example.** ANL 60, appraisal 30, `ilvl 8`: `anl_bp = 3000 + 2000 + 1200 − 1200 = 5000`. A roll of 2404 succeeds with margin `5000 − 1 − 2404 = 2595`, so `reveals = 1 + 1 = 2`. A roll of 6052 fails. ANL 80, appraisal 50 at `ilvl 5` gives 8250, so 3 reveals need a roll below 3250 (32.5%).

### 5.7 Familiarity

While an item is equipped by a living Erratum inside a Folio, the Needs pass adds 60 to `fam_ticks`. On `ev.entity_killed` of a hostile credited to an Erratum (Canon §15.1 minion kills included), `fam_kills += 1` on every equipped non-weapon item of that Erratum, and on the weapon instance that dealt the killing hit if it was a direct kill. Counters live on the item and survive trades.

A stage is reached when `fam_ticks >= T` **or** `fam_kills >= K`, each stage independently:

| Stage | Reveals | `T` (ticks) | `K` |
|---|---|---|---|
| F1 | RARITY, BUC | 1800 (30 s) | 3 |
| F2 | ENH, CHARGES | 3600 (60 s) | 8 |
| F3+k, k = 0..=7 | `AFFIX_NAME[k]`, `AFFIX_ROLL[k]` | `5400 + 1800 × k` | `12 + 4 × k` |
| F-type | type of a `conditional` accessory or implant | 7200 (120 s) | 15 |

Reason `appr.reason.familiarity`. Example: a Rare sword (2 affixes) reveals F1 and F2 at kills 3 and 8, affix 0 at kill 12 and affix 1 at kill 16, or at 90 s and 120 s of wielding if sooner. Erasure (Canon §11.6) limits idling for familiarity.

### 5.8 Price observation, the Ledger and candidates

Vendors know every type (rule 3 of §1). Whenever a trade window shows a buy price or a sell offer for a type-hidden item of unknown type, the host records a `PriceObs` under its descriptor, skipping exact duplicates, keeping the newest 8:

```rust
pub struct PriceObs {
    pub price: i64,
    pub ilvl: i32,
    pub rar_mult: i64,       // value multiplier of the KNOWN rarity, 1 if RARITY unknown (§6.2)
    pub factors: [i32; 5],   // quality, material, enhancement, buc, charges factors as value_vendor applied them
    pub trade_bp: i32,       // Canon §6.4 row 37 or row 38 of the trading player; 10000 for appraiser observations
}
fn price_for_base(base: i64, o: &PriceObs) -> i64 {
    let mut v = mul_bp(base, P(min(o.ilvl, 300))) * o.rar_mult;
    for f in o.factors { v = mul_bp(v, f); }
    max(1, mul_bp(max(1, v), o.trade_bp))
}
fn candidates(d: Id32, tk: &TypeKnowledge) -> Vec<Id32> {
    class_types(class_of(d)).filter(|t| !tk.known.contains(t))
        .filter(|t| !tk.excluded.get(&d).map_or(false, |s| s.contains(t)))
        .filter(|t| !(tk.tried_no_effect.contains(&d) && use_rule(t) == Always))
        .filter(|t| tk.read_mode.get(&d).map_or(true, |m| m.matches(t)))
        .filter(|t| valid_target_cats(t) & tk.refused_cats.get(&d).copied().unwrap_or(0) == 0)
        .filter(|t| tk.price_obs.get(&d).map_or(true, |v| v.iter().all(|o| price_for_base(base_value(t), o) == o.price)))
        .collect()
}
```

The Ledger panel (doc 17) shows `candidates(d)` for every seen unknown descriptor. It automates NetHack's bookkeeping and keeps the deduction.

**Worked example.** An unknown murky green flask, true type `alch.potion.healing` (doc 08 base value 30), `ilvl 3` (`P = 11881`). The buyer has CHM 50, so row 37 gives 9700. Price: `mul_bp(30, 11881) = 35`, then `mul_bp(35, 9700) = 33`. Doc 08's canonical bases 10, 12, 15, 20, 25, 30, 35 give 10, 13, 16, 22, 28, 33, 39, so only base 30 fits: healing or haste. For a text read with no visible result, `tried_no_effect` also removes every `always` text from its candidates.

### 5.9 Scriptorium and altar services

Scriptorium rooms (doc 12) and altars sell these services. `d` is the current Folio depth, with `P(min(d, 300))`. Payment is Lucre per item.

| ID | Where | Effect | Cost | d1 | d3 | d12 | d25 |
|---|---|---|---|---|---|---|---|
| `appr.svc.appraise` | Scriptorium, every vendor | tier-1 reveal set and `est_steps += 2` | `mul_bp(20, P(d))` | 20 | 23 | 51 | 158 |
| `appr.svc.classify` | Scriptorium | type of one type-hidden instance | `mul_bp(40, P(d))` | 40 | 47 | 103 | 316 |
| `appr.svc.identify` | Scriptorium | every required flag plus type | `mul_bp(60, P(d))` | 60 | 71 | 154 | 474 |
| `appr.svc.unbind` | Scriptorium | one cursed item becomes uncursed (§7.6), even while bound | `mul_bp(150, P(d))` | 150 | 178 | 387 | 1186 |
| `appr.svc.drop_test` | altar | dropping any item on an altar block reveals its BUC; genre-skinned bright flash if blessed, dark flash if cursed | free | 0 | 0 | 0 | 0 |
| `appr.svc.consecrate` | altar | cursed → uncursed, or uncursed → blessed; refused for blessed items; at most once per item per Folio | `mul_bp(300, P(d))` Lucre, or 100 favor with the altar's patron when favor ≥ 100 | 300 | 356 | 774 | 2373 |

Costs depend only on depth and stay below the affliction cure anchor `mul_bp(500, P(d))` (Canon §16.4). Reason `appr.reason.service` or `appr.reason.altar`.

### 5.10 Divination glyphs (`skill.divination`)

Glyphs belong to doc 06, whose catalog does not yet list these essences (Canon gaps); their identification effects are defined here. `div_tier` = mastery tier of `skill.divination` (0..=4). Glyph potency (Canon §5.5) multiplies radii and chances with `mul_bp`; a Disconnected glyph cannot be cast.

| Glyph | Effect ID | Effect | Draws |
|---|---|---|---|
| `glyph.ess.appraise` | `appr.eff.divine_identify` | as uncursed `appr.eff.identify`; as blessed when `skill.divination >= 75` | 0 |
| `glyph.ess.omen` | `appr.eff.divine_omen` | BUC of every item on the ground or carried by a party member within `4000 + 2000 × div_tier` mm | 0 |
| `glyph.ess.sight` (doc 06, existing) | `appr.eff.aura` | also sets AURA on every item within the omen radius: **mundane** (no affixes, `enhancement = 0`, uncursed), **ominous** (cursed), or **storied** (anything else) | 0 |
| `glyph.ess.psychometry` | `appr.eff.psychometry` | one type-hidden item of unknown type in the caster's inventory: success → type; shares the Analysis lock `analysis_lvl[slot]` | 1 `roll_bp` |

`psy_bp = clamp(2000 + 60 × skill.divination + 50 × d(ITU), 0, 9500)`, keyed `rng.ident` (actor = caster, cast tick, action seq). At divination 40, ITU 50: `2000 + 2400 + 500 = 4900`. Reason `appr.reason.glyph`.

### 5.11 Curse sensing on pickup (Canon §6.4 row 36)

On `ev.item_picked_up` by Erratum `E` in party slot `s`, when the item is BUC-bearing, has none of `BUC`, `OMEN`, `CLEAN`, and `SENSED[s]` is unset:

```
SENSED[s] = 1
chance = Canon §6.4 row 36, clamp(1000 + d(ITU) × 100, 0, 9000), with doc 04 flats (Appraisal 75: +2000)
if roll_bp(chance) on keyed rng.ident (actor E, pickup tick, action seq):
    if buc == cursed { set OMEN } else { set CLEAN }
```

One draw per (item, party slot), so handing an item to the party's mystic is a real co-op move. ITU 40 gives 1000 Bp, 55 gives 2500, 80 gives 5000, 130 caps at 9000. Reason `appr.reason.sense`.

### 5.12 Every `rng.ident` draw

All keyed (Canon §3.3). **Action** key: actor = acting Erratum, tick = input tick, seq = action seq. **Object** key: actor = the object's `uid`, tick = `u64::MAX` (never a real tick), seq = purpose code, giving a per-object constant.

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
| estimate offset `u` (§6.3) | object: item `uid`, seq 1 | 1 `range_incl(-10000, 10000)` |

---

## 6. Value model

### 6.1 True value

Doc 11 stores `value` at generation and recomputes it whenever an input changes. Each arrow is one `mul_bp`, in order (Canon §1.3):

```
value_true(it) =
    v = template.base_value                       // whole Lucre at ilvl 1, Common, Normal, +0, uncursed, full charges
    → × P(min(ilvl, 300))
    → × rarity value multiplier                   // integer multiply ×1, 2, 4, 8, 16, 32, 64 (Canon §13.3)
    → × quality_bp                                // Canon §13.5
    → × mat_value_bp                              // primary material, doc 07; 10000 when the item has none
    → × (10000 + 1000 × eff_enh)                  // only if H has ENH; eff_enh from Canon §13.7
    → × buc_value_bp                              // cursed 5000; blessed consumable 12500; otherwise 10000
    → × (5000 + floor_div(5000 × charges.cur, max(1, charges.max)))   // only if H has CHARGES
    result max(1, v), or 0 when base_value = 0
```

Blessed equipment's +1 is already inside `eff_enh`. `illuminated` never changes value.

| Item | Steps | Value |
|---|---|---|
| Longsword, base 30, `ilvl 7`, Rare, Good, iron, +2, blessed (`eff_enh = 3`) | 50 → 200 → 220 → 220 → 286 → 286 | **286** |
| Wand, base 200, `ilvl 9`, Uncommon, Normal, +1, uncursed, 3 of 6 charges | 398 → 796 → 796 → 796 → 875 → 875 → 656 | **656** |
| Ring, base 150, `ilvl 12`, Rare, Normal, +0, cursed penalty 2 (`eff_enh = −2`) | 387 → 1548 → 1548 → 1548 → 1238 → 619 | **619** |
| Potion, base 40, `ilvl 5`, Common, blessed | 56 → 56 → 56 → 56 → 70 | **70** |

### 6.2 Vendor value

`value_vendor(it)` is the basis of every vendor price. The vendor knows the type. Every instance property whose flag is unset takes its default: unknown rarity counts as Common (×1), unknown enhancement as 0, unknown curse state as uncursed with penalty 0 (but `OMEN` counts as cursed with penalty 1), unknown charges as `charges.cur = 0`. Otherwise it equals `value_true`, so identified items give `value_vendor = value_true`, and an unappraised Legendary sells at a Common price.

### 6.3 The appraisal estimate

An item with `RARITY` known and `est_skill >= 0` shows a value range that always contains `value_true` and whose center is offset by a fixed, item-specific fraction:

```
err_bp = clamp(5000 − 100 × est_skill − 750 × est_steps, 500, 5000)    // half-width as a share of value
u      = keyed_rng("rng.ident", uid >> 40, uid, u64::MAX, 1).range_incl(-10000, 10000)   // object constant, §5.12
V      = value_true(it)
half   = mul_bp(V, err_bp)
off    = mul_bp(half, mul_bp(u, 6000))              // |off| ≤ 60% of half
lo     = max(0, min(V, V + off − half))
hi     = max(V, V + off + half)
g      = 10^max(0, digits(hi) − 2)                  // display granularity: two significant figures
shown  = "≈ {floor_div(lo, g) × g}–{ceil_div(hi, g) × g}"
```

Because `u` is constant per item, the range narrows toward a fixed off-center point and never loses the true value. Once `est_skill >= 25` (doc 04 Apprentice perk) the line shows exact `value_true` instead.

**Worked example.** Run seed `0x0123456789ABCDEF`, the longsword above with `uid = (7 << 40) | 42`. Its constant is `u = −6692`. With `est_skill = 20`, `est_steps = 0`: `err_bp = 3000`, `half = mul_bp(286, 3000) = 85`, `mul_bp(−6692, 6000) = −4016`, `off = mul_bp(85, −4016) = −35`, `lo = 166`, `hi = 336`, `g = 10`, shown **"≈ 160–340"**. At skill 0 it shows "≈ 80–380"; at skill 20 with two Analysis steps (`err_bp = 1500`), "≈ 220–320"; at skill 0 with four steps (`err_bp = 2000`), "≈ 200–320"; at skill 25, "286".

### 6.4 Display formats

| State | Name line | Value line | Curse line | Enhancement | Properties |
|---|---|---|---|---|---|
| `know.unknown` | "Murky green flask" | Ledger: "Price group 40: 2 candidates" or "Value ?" | "?", "Ominous" (OMEN) or "Clean" (CLEAN) | — | — |
| `know.classified` | skinned type name and quality: "Good Longsword" | "Value ?" | as above | "+?" | "Unknown properties" |
| `know.appraised` | rarity color, unique name if any: "Rare Longsword" | "≈ 160–340 Crowns" | as above or the known state | "+?" or "+2" | "2 unread properties", then each revealed name, with "(?)" until its roll is known |
| `know.identified` | full name | "286 Crowns" | skinned state: Blessed, —, Cursed (or Hexed, Corrupted per genre) and penalty | "+2" | every affix with its roll |

Mundane items show exact value and no curse line. Money uses the local Lucre skin (Canon §5.2).

---

## 7. Curses and blessings

### 7.1 Generation: `roll_curse`

Doc 11's generation step 12 calls this once per BUC-bearing item with its `loot.buc` sub-generator of keyed `rng.loot`, and skips it for shops, Fortune Engines and Volume rewards (always uncursed). It always draws exactly three values.

```rust
fn roll_curse(rng: &mut Rng, it: &ItemDraft, local_myt: i32, src: Source) -> (Buc, i8, Option<CurseId>) {
    let curse_bp = if local_myt == 0 { 0 } else {                                   // Canon §5.3: no mythos components at MYTHOS 0
        min(9000, mul_bp(mul_bp(mul_bp(curse_base(it), 10000 + (local_myt - 5) * 1500), // Canon §5.7 MYTHOS term
                                rarity_factor(it.rarity)), source_curse_factor(src, it)))
    };
    let bless_bp = if local_myt == 0 { 0 } else { mul_bp(bless_base(it), source_bless_factor(src, it)) };
    let elig: Vec<CurseId> = eligible_curses(it);                                      // ascending ID order
    let a = rng.range(10000) as i32;                                                    // draw A
    let b = rng.weighted(&[5000, 3500, 1500]);                                          // draw B: penalty 1, 2, 3
    let c = rng.range(max(1, elig.len()) as u64) as usize;                              // draw C
    if a < curse_bp && !elig.is_empty() { (Cursed, (b + 1) as i8, Some(elig[c])) }
    else if a < curse_bp + bless_bp     { (Blessed, 0, None) }
    else                                 { (Uncursed, 0, None) }
}
```

`local_myt` is the local MYTHOS at the generating entity or container (Canon §5.8); for a Keepsake (doc 15 §5.4) it is the MYTHOS of the Reliquary's source genre.

| Category | `curse_base` | `bless_base` |
|---|---|---|
| weapons, `cat.armor`, `cat.offhand`, `cat.ward_gen` | 1500 | 500 |
| rings, amulets, relics | 2000 | 500 |
| belts, back items | 1200 | 500 |
| `cat.implant` | 1500 | 300 |
| `cat.wand` | 1000 | 500 |
| `cat.potion`, `cat.text` | 1000 | 1000 |
| malign accessory or implant type (§13) | 6000 | 0 |

| Factor | Value |
|---|---|
| `rarity_factor` | 10000 up to Epic; 5000 Legendary and Exotic; 0 Palimpsestic |
| `source_curse_factor` | 10000 for drops, containers, floor loot and Keepsakes; 0 for in-run Reliquaries and Editor rewards (crafted, brewed and kit items never call it) |
| `source_bless_factor` | 10000 |

**Worked examples.** A weapon dropped in Gothic Horror (MYTHOS 9): `mul_bp(1500, 16000) = 2400` Bp. The same in Hardboiled Noir (MYTHOS 1): `mul_bp(1500, 4000) = 600`. A ring in Mythic Fantasy (6): 2300. A potion in Fairy Tale (8): 1450. A malign ring in Cosmic Horror (10): `mul_bp(6000, 17500) = 10500`, capped at 9000. A Legendary weapon in Mythic Antiquity (10): `mul_bp(2625, 5000) = 1312`. A weapon Keepsake from a Gothic Horror Reliquary: 2400. Anything generated in The Unwritten (MYTHOS 0): 0.

The curse is appended to `affixes` as `{id: <curse>, tier: <penalty>, roll_bp: 10000}` and stays last.

### 7.2 Binding

A cursed equipped item cannot be unequipped (Canon §13.6), whatever its knowledge state or Contradiction state.

- A cursed weapon in any weapon slot becomes the active weapon on equip, and active-weapon switching is locked while it is equipped. A cursed two-handed weapon also locks `slot.offhand` empty.
- A cursed wand in `slot.offhand` stays there. A cursed implant cannot be uninstalled.
- Binding ends only when the item becomes uncursed or its slot stops existing. A severed `bp.arm_l` or `bp.arm_r` (Canon §9.1) takes the ring in `slot.ring_l` or `slot.ring_r` with it; the ring stays on the `cat.body_part` item and is recovered with a 60-tick interact, still cursed. `st.polymorphed` stows equipment and re-equips bound items when it ends. Ghost item loss (Canon §17.6) may take a bound item.
- Bound items cannot be sold, dropped, stashed or crafted with.

### 7.3 Curse affixes

Every curse is a component with `dep = {mythos}` (Canon §5.3) whose origin is the item's `origin_genre`. With `p = curse_penalty` and `pot` = the curse component's Contradiction potency, its magnitude is `m = mul_bp(unit × p, pot)`, a non-negative number applied in the stated direction. A Disconnected curse has `m = 0`, but still binds and still lowers `eff_enh`. "Equipment" means weapons, armor, off-hands, ward generators, accessories, implants and wands.

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
| `curse.blunted` | weapons, `cat.offhand` | 1000 | −`m` Bp in the additive bucket (Canon §10.5 step 1) |
| `curse.misfire` | `cat.weapon_ranged`, `cat.wand` | 300 | +`m` Bp to the base-function failure chance (Canon §5.5); rolled even when Contradiction gives 0 |
| `curse.dulled` | weapons | 500 | crit chance gear term −`m` Bp (Canon §6.4 row 13) |
| `curse.loud` | equipment | 2 | every noise the bearer emits +`m` dB (doc 09) |
| `curse.beacon` | equipment | 2 | the bearer is a dynamic light source of level `min(15, 2 + m)` |
| `curse.ravenous` | equipment | 1 | −`m` satiation on Needs ticks where `floor_div(t, 60) % 4 == 0` |
| `curse.brittle` | weapons, `cat.armor`, `cat.offhand`, `cat.ward_gen` | 5000 | durability loss × `(10000 + m)` (doc 07) |
| `curse.bloodprice` | weapons, `cat.wand`, `cat.offhand` | 50 | each activation costs `mul_bp(max_hp_mhp, m)` as `dmg.true` entering Canon §10.5 at step 4, no crit, no chip floor; never below 1,000 mHP |
| `curse.backlash` | weapons | 500 | each hit returns `mul_bp(HP damage dealt, m)` to the wielder by the same rule |
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

- Blessed: positive effects × 12500 (Canon §13.6); malign effects unchanged.
- Cursed: positive effects × 7500, malign effects × 12500, plus the category's curse affix: `curse.spoiled` (potions), `curse.misprint` (texts), `curse.backfire` or `curse.leaking` (wands). The doc-10 texts use their own cursed column in §5.4 instead of the generic ×7500.

### 7.6 Removing curses

"Uncursing" sets `buc = uncursed`, `curse_penalty = 0`, removes the curse affix and sets `BUC`. Enhancement is unchanged.

| Method | Scope | Cost |
|---|---|---|
| Emendation (Canon §13.7) | one item; `BUC` set if it was cursed, otherwise `CLEAN` | the Emendation |
| `appr.eff.unbind` | equipped items; blessed also carried | the text |
| `appr.eff.consecrate`, `appr.svc.consecrate` | one item, one step up | text, Lucre or favor |
| `appr.svc.unbind` | one item | `mul_bp(150, P(d))` |
| Doc 08 transmutations `alch.tx.uncurse`, `alch.tx.consecrate` | one potion | ingredients |
| Prayer "fix curses" boon (doc 06) | every equipped item | prayer timer |

Severing and ghost loss end binding (§7.2) but never uncurse.

### 7.7 Blessings

- Effects are Canon §13.6. Blessed equipment's +1 is inside `eff_enh`.
- A blessed item cannot be cursed in one step: any effect that would curse it makes it uncursed instead (cursed consecrate).
- A blessed item shows its genre-skinned glow only once `BUC` is known.
- Blessing sources: generation (§7.1), `appr.eff.consecrate`, `appr.svc.consecrate`, brewing with `liq.holy_water` (doc 08), patron boons (doc 06).

---

## 8. Dangerous unknown consumables

Using an unknown consumable is a deliberate gamble, as in Shattered Pixel Dungeon. Docs 08 and 06 own the effects; these rules bound them.

1. **Malign share.** Harmful types make up 3000 Bp of each consumable class's generation weight at every depth (doc 11 loot tables).
2. **Self-harm cap.** Damage a drunk or read consumable deals to its own user is capped at `mul_bp(max_hp_mhp, 3000)` per use, and control statuses it applies to its own user last at most 300 ticks. Example: drinking `alch.potion.dragonfire` at depth 25 with 220 HP Max deals at most 66,000 mHP to the drinker.
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
| Appraisal tier 0 / 1 / 2 / 3 | 0 | 1 | 2 | 2 | 3 |
| Familiarity | F1 | F1 | F2 | F3, F4 | F3, F4 |
| Analysis | 1st reveal | 2nd | 3rd | 4th, 5th | 6th, 7th |
| Equip a cursed item | — | on bind | — | the curse slot | the curse slot |
| Curse sense | — | OMEN or CLEAN | — | — | — |
| `appr.eff.identify`, `appr.svc.identify` | all | all | all | all | all |

- **UI masking.** The character sheet shows true values; a stat with a contribution from an unrevealed property is marked "?" and the item breakdown hides that contribution.
- **Unread Edge.** Each equipped item of unknown type or missing any required flag gives its bearer +4 effective SER, at most 3 items (+12), shifting in-run loot rarity only (Canon §13.4), after Shattered Pixel Dungeon's Shard of Oblivion. It fades as familiarity reveals the gear. Example: at `d = 10`, SER 40 gives rarity weights `[7000, 2992, 1032, 312, 109, 42, 0]`; with +12 SER, `[7000, 3124, 1104, 339, 120, 46, 0]`.

---

## 10. Shops and selling

### 10.1 Stock

Doc 11 fills shops from `rng.loot_place`. Equipment stock is identified. Type-hidden consumable stock is uncursed, has RARITY and BUC set, and shows its appearance until the party knows its type.

### 10.2 Prices

```
buy_price(it, buyer)   = max(1, mul_bp(value_vendor(it), clamp(10000 − d(CHM) × 30, 7000, 13000)))   // Canon §6.4 row 37
sell_offer(it, seller) = max(1, mul_bp(value_vendor(it), clamp(3000 + d(CHM) × 30, 1500, 5000)))     // Canon §6.4 row 38
```

Prices are per unit. Every displayed price for an unknown-type item records a `PriceObs` (§5.8).

### 10.3 Rules

- Vendors refuse bound items, `cat.quest` items and account seals (§11.2).
- **Buyback.** Until `ev.folio_exited`, a vendor sells back any item a party member sold it, at the exact `sell_offer` paid, with its knowledge unchanged.
- Every vendor offers `appr.svc.appraise`; only Scriptoria offer the other services. Fortune Engine capsules arrive with RARITY set and are never cursed.

---

## 11. Seals: Reliquary contents

### 11.1 In-run Reliquaries (doc 11)

Each item from an opened in-run Reliquary arrives as a **sealed lot**: `seal = Sealed`, `RARITY` set, contents fixed by doc 11's keyed `rng.loot` roll and never cursed.

| While sealed | Rule |
|---|---|
| Visible | category silhouette, rarity, `origin_genre`, `ilvl` |
| Hidden | the template, even for categories that are not type-hidden, and every other property |
| Allowed | carry, drop, trade, draft (doc 11 turn-order draft uses this view), sell, unseal |
| Not allowed | equip, use, Rewrite, craft, Analysis, familiarity |
| Sale | vendors pay `mul_bp(seal_floor, sell share)`, `seal_floor = max(1, mul_bp(cat_floor_base, P(ilvl)) × rarity value multiplier)` |

`cat_floor_base`: potion and text 10, ammo-like and throwable 5, armor, off-hand and melee 15, ranged 20, ward generator 30, accessory 40, wand 50, implant 60. Example: a sealed Rare accessory at `ilvl 12` has `seal_floor = mul_bp(40, 25804) × 4 = 103 × 4 = 412`; at CHM 40 a vendor pays `mul_bp(412, 3000) = 123`.

| Unseal by | Result | Cost |
|---|---|---|
| Break the seal (60-tick channel) | normal knowledge: RARITY known, everything else per §2 | free |
| Appraisal completion (§5.5) on the lot | unsealed, then that tier's reveal set applied | time |
| `appr.eff.identify` or `appr.eff.divine_identify` targeting the lot | unsealed and fully identified | the text or spell |
| `appr.svc.unseal` | unsealed and fully identified | `mul_bp(30, P(d))`, half the identify price |

Reason `appr.reason.seal`.

### 11.2 Account Reliquaries (doc 15)

Doc 15 owns sources, contents, odds, pity and the commit-reveal server roll; this chapter owns presentation.

1. The open response carries each lot's content ID and rarity; the client stores them sealed and shows the rarity frame and kind silhouette at once, so pity counters and odds disclosures match what the player sees.
2. **Break the seal** in the Margin at any time: instant, free, works offline.
3. **Carry the seal**: before a run the player may attach up to 3 sealed lots to their Erratum. They take no inventory space, have no gameplay effect, and unseal with a reveal ceremony at the carrier's 5th `ev.item_identified` event in that run (any scope, any reason) or at `ev.run_ended`, whichever comes first.
4. Sealing never changes content, costs nothing, has no timer, and is never bought, sped up or skipped with money.

---

## 12. The Concordance: item codex and bestiary

The Concordance is the account-level encyclopedia in the Margin, with tabs **Item Codex** and **Bestiary** (Canon gaps 4). It is stored locally, synced online, and complete offline under the end-of-service guarantee.

### 12.1 Item Codex

Per template (unique templates included) and per `curse.` affix: `seen`, `identified_runs`, `familiarity` points, and the set of genre skins unlocked as illuminated.

- `seen` is set on pickup. A type-hidden type's page opens only after its first identification in any run.
- `familiarity += 1` once per run per template at its first identification in that run (type known, or an instance reaching `know.identified`), for every party member present.

| Rank | Points | Unlocks |
|---|---|---|
| `know.rank.noted` | 1 | page with description, price group, use rule |
| `know.rank.studied` | 3 | the in-run Ledger shows this type's price group and use rule next to each candidate |
| `know.rank.familiar` | 6 | the type may be named in a Recollection (§12.3) |
| `know.rank.mastered` | 10 | gilded page, lore text, title cosmetic |

### 12.2 Illuminated variants

Canon §13.9 unlocks an illuminated skin account-wide on pickup. This chapter adds:

- The unlock is recorded at pickup, but its notification and the illuminated render wait until `RARITY` is known or `ev.run_ended`, because illumination rolls only on Legendary-or-better drops and an early glow would leak rarity.
- Pages show each illuminated skin as a gilded plate per genre; a never-identified type-hidden page shows "An illuminated page awaits its name".

### 12.3 Recollection

Once per Volume per party slot (`recollect_volume`), a player may name one seen, unknown descriptor and one type at `know.rank.familiar` or above in the same class. If `assign[type] == descriptor`, `identify_type` with reason `appr.reason.recollection`; otherwise `excluded[descriptor].insert(type)`. No draws. Recollection is disabled in Daily, Weekly and Seeded leaderboard modes (doc 14); every display-only Concordance feature stays on.

### 12.4 Bestiary

Per enemy template (doc 13): encounters and kills, all play counted.

| Rank | Kills | Shows (in the Margin and in in-run inspect) |
|---|---|---|
| `know.bestiary.sighted` | 0 (1 encounter) | name, silhouette, genre, rank |
| `know.bestiary.studied` | 10 | HP multiplier, resistances, immunities |
| `know.bestiary.catalogued` | 50 | status thresholds, drop categories, champion aspects |
| `know.bestiary.mastered` | 200 | lore, gilded page |

### 12.5 Never monetized

- Only play grants entries, points and ranks; no SKU, entitlement, Inkwell result or account Reliquary does, and nothing can be bought, sped up or skipped.
- Completion counts only entries obtainable in runs; Inkwell and account-Reliquary cosmetics live in the wardrobe (doc 15).
- Completion rewards are cosmetic titles and Margin plot decorations only.

---

## 13. Reference rosters

Owners: potions 08; texts 06 (doc-10 effects §5.4; Emendation and Rewrite 11); rings, amulets, relics 11; wands 06; implants 04. Owners may change rosters within §3.2 budgets, keeping these price groups and rules. Codes: **A** always, **E** on_effect, **D** display, **C** conditional; `*` marks malign.

| Class | Base value: types (rule) |
|---|---|
| Potions | 20: vigor E, antidote E, blindness* A, ferment* A · 40: healing E, clarity E, dark_sight A, flame* A, miasma* A, frost* A · 60: haste A, fortify A, regeneration A, stupor* A · 80: levitation A, invisibility A, empower A, ward A · 120: restoration E, insight A, purity E · 200: red_ink* A |
| Texts | 15: identify A · 40: light A, survey E, omen E, clamor* A · 60: intuit A, scatter* A, silence A, amnesia* A · 90: unbind E, mapping A, summons* A, lullaby E, terror E, warding A, binding E · 150: consecrate A, recharge A, rewrite A · 300: emendation A |
| Rings | 100: sustenance C, stealth C, aggravation* C, hunger* C · 150: power D, agility D, toughness D, insight D, focus D, clumsiness* D · 200: fortune D, regeneration D, resistance D, warning C, teleportitis* C · 300: evasion C |
| Amulets | 150: breath D, clarity D, sight D, strangling* C · 250: warding D, coherence D, unchanging C, restless* C · 400: life_saving C, reflection C, fortitude D, discord* C |
| Wands (charges_base) | 100: light A (8), opening E (6) · 150: digging E (6), striking E (6), slowing E (5), probing A (5) · 200: fire A (5), frost A (5), lightning A (5), sleep E (5), invisibility E (4) · 300: teleport_other E (4), wild* A (5) · 500: polymorph E (3) |
| Implants | 200: optic D, lung D, plating D, tracker* C · 300: reflex D, cache D, lace D, glitch* C · 450: adrenal C, cortex D |
| Relics | 120: lucky_coin D, beacon D · 180: watcher D, hearth C, lodestone* D · 260: thorns C, oathstone C, hourglass D · 400: lattice_lens C, weeping* C |

Targeted texts: identify, consecrate, recharge, rewrite, emendation. Their valid target categories: identify, any non-mundane item; consecrate, any BUC-bearing item; recharge, `cat.wand`; rewrite and emendation, any item with ENH. `item.wand.probing` zapped at oneself reveals BUC and CHARGES of every inventory item (reason `appr.reason.zap`). `item.wand.wild` picks one of the other 13 wand effects per zap with one `range(13)` on keyed `rng.magic` (doc 06). `item.relic.lattice_lens` is the Lattice Lens of Canon §17.1 and identifies when an exit offer opens with 3 choices. Distinct price groups in a class differ by at least 20 Lucre.

**Integrated example.** Folio 3, Gothic Horror. Ana (ITU 55) senses a murky green flask as CLEAN (2500 Bp), drinks one at full HP with no effect, then sees a buy price of 45: the Ledger narrows it to healing or clarity (§5.8). Wounded later, she drinks another; HP rises and healing is identified for the party. Bo's unknown ring binds on equip: `curse.feeble`, penalty 2, rolled at `mul_bp(2000, 16000) = 3200` Bp, at full potency here (PWR −10). Unbinding costs `mul_bp(150, 11881) = 178` Lucre.

---

## Exports

All in namespaces Canon §19 assigns to this chapter.

| Namespace | IDs |
|---|---|
| `know.` | core `know.unknown`, `know.classified`, `know.appraised`, `know.identified` (Canon §13.6); new `know.rank.noted`, `know.rank.studied`, `know.rank.familiar`, `know.rank.mastered`, `know.bestiary.sighted`, `know.bestiary.studied`, `know.bestiary.catalogued`, `know.bestiary.mastered` |
| `idc.` | core classes `idc.potion`, `idc.text`, `idc.ring`, `idc.amulet`, `idc.wand`, `idc.implant`, `idc.relic`; 142 descriptors `idc.<class>.<name>` listed in §3.3 |
| `appr.tier.` | `novice`, `apprentice`, `journeyman`, `expert`, `master` |
| `appr.use.` | `always`, `on_effect`, `never`, `display`, `conditional` |
| `appr.eff.` | `identify`, `survey`, `omen`, `unbind`, `consecrate`, `intuit`, `recharge`, `amnesia`, `divine_identify`, `divine_omen`, `aura`, `psychometry` |
| `appr.svc.` | `appraise`, `classify`, `identify`, `unseal`, `unbind`, `drop_test`, `consecrate` |
| `appr.reason.` | `use`, `zap`, `equip`, `bind`, `witness`, `deduction`, `text`, `appraisal`, `analysis`, `familiarity`, `service`, `altar`, `glyph`, `sense`, `recollection`, `kit`, `craft`, `brew`, `seal` |
| `buc.` | core `buc.blessed`, `buc.uncursed`, `buc.cursed` (Canon §13.6); no new IDs |
| `curse.` | `leaden`, `thirsting`, `fumbling`, `feeble`, `sluggish`, `frail`, `addled`, `myopic`, `muffled`, `ill_starred`, `blunted`, `misfire`, `dulled`, `loud`, `beacon`, `ravenous`, `brittle`, `bloodprice`, `backlash`, `dread`, `red_letter`, `siphon`, `greed`, `fragile_ward`, `spoiled`, `misprint`, `backfire`, `leaking` (28) |

Also defined here, without new IDs: the `cat.wand` contract (§4), the type-hiding function (§3.1), `roll_curse` (§7.1), `value_true` and `value_vendor` (§6), the knowledge structs (§2), content errors `idc_pool_exhausted`, `idc_slot_mixed`, `idc_base_type_missing`, and text keys `loc.idc.<class>.<name>.name` and `loc.idc.<class>.form.<genre_suffix>` (strings owned by doc 18).

## Interface assumptions

| Doc | Assumed |
|---|---|
| 03 | `ev.item_identified` payload `{ scope, uid or type_key, descriptor, flags_new: u32, reason, slot }`; appraisal and familiarity run right after Needs |
| 04 | kits may hold type-hidden items; adopts the §5.5 appraisal XP and no other appraisal perks; implant install counts as equip; red ink adds `afflict.red_ink` buildup |
| 05 | locks weapon switching for bound weapons; sums `curse.misfire` into the failure roll; reports the killing weapon instance |
| 06 | glyphs `glyph.divination.identify`, `.omen`, `.aura`, `.psychometry`; wand and text effects set `observable`; a "fix curses" prayer boon |
| 07 | `mat_value_bp` per material (iron 10000); crafted items fully flagged; durability honors `curse.brittle` |
| 08 | potion drink and splash effects set `observable`; holy-water dip steps `buc` up; pure brews call `identify_type`; §8 contract |
| 09 | per-tick sight checks for witness ID; noise and dynamic-light hooks for `curse.loud` and `curse.beacon` |
| 11 | owns `item.<class>.<name>` templates for §13 and the doc-10 texts; calls `roll_curse` and `value_true`; knowledge of shop, Reliquary and Fortune Engine items per §10 and §11; `charges.max`; the 3000 Bp malign share; Lucre income per Folio per player of at least `mul_bp(300, P(d))` |
| 12, 13 | Scriptoria and altars placed; NPCs use type-hidden items; Editors honor `curse.red_letter`; enemy templates for the Bestiary |
| 14, 15 | leaderboard mode flags, Concordance UI, `unlock.daily_baseline`; account Reliquary open response with content IDs and rarity; the wardrobe |
| 16, 17 | `ItemView`-only replication with host authority; `input.examine`, focus lock, Ledger panel, §6.4 tooltips |
| 18, 19 | sigil atlas, descriptor art, form nouns, curse-state skins, stamp review; schemas for every §2 struct and content record |

## Canon gaps

1. **Item template IDs.** `item.` belongs to doc 11 (Canon §19). Assumed `item.<class>.<name>` for every §13 type and doc-10 text; this chapter defines only their effects (`appr.eff.`).
2. **`rng.ident` uses.** Canon §3.3 lists appraisal error and curse sensing; as owner this chapter adds the §5.12 uses and asks the canon table to list them.
3. **Object-keyed constants.** Canon §3.3 keys action draws only. Assumed valid: actor = object id, tick = `u64::MAX`, seq = purpose code.
4. **Name collision.** Canon §20 calls the battle pass "Codex"; the encyclopedia is therefore the Concordance, with tabs Item Codex and Bestiary.
5. **Mundane categories.** Canon §13.6 starts them at `know.classified`; this chapter makes that state terminal and complete.
6. **Item contract.** Canon §13.9 lacks `recharges`, `zaps_total`, `seal` and the `ItemKnowledge` sub-fields; assumed added by doc 19.
7. **Tick schedule.** Canon §2.2 has no appraisal or familiarity row; assumed to run in the Needs pass.
8. **§6.4 rows.** Appraisal rate, Analysis chance and psychometry chance are defined here; if Canon §6.4's "derived stats only" rule covers them, they become rows 45 to 47 unchanged.
9. **Blessing chance.** Canon §5.7 has no MYTHOS term for it; kept flat, 0 at MYTHOS 0.
10. **Missing §5.9.** Canon §12.3 and §13.9 cite a §5.9 that does not exist; implant stamps avoid manufacturer marks via doc 18 review.
11. **Categories.** Emendations and the Rewrite consumable are assumed `cat.text`; the Lattice Lens of Canon §17.1 is `item.relic.lattice_lens`.
12. **Binding and slot loss.** Severing and ghost loss (Canon §9.1, §17.6) are read as the slot or item leaving, not an unequip, so Canon §13.6 binding ends without uncursing.
13. **Misfire.** `curse.misfire` raises the Canon §5.5 failure chance above 0, so the single per-activation roll still applies.
14. **Events.** No new `ev.` IDs; binding, sensing, sealing and deduction report through `ev.item_identified` reasons.

## Acceptance tests

Run seed for keyed tests: `0x0123456789ABCDEF`.

1. **Appearance stream.** `stream_seed(run, "rng.appearance", 0) = 0x95b9ab50b5b43f53`.
2. **Amulet permutation.** The sorted 18 amulet descriptors shuffle to `teardrop, heart, bell, fang, acorn, spindle, lozenge, cage, arrowhead, crescent, coin, key, feather, knot, shell, wheel, disc, star`.
3. **Assignments** (§13 rosters): amulet breath → teardrop, discord → fang; implant adrenal → hollow_triangle; potion antidote → azure; relic beacon → bee; ring aggravation → obsidian; text amnesia → nulzeph; wand digging → oak.
4. **Draw count.** Exactly 135 `range` calls, with the §13 rosters or with zero types.
5. **Pool budget.** 13 implant types against 14 stamps fails load with `idc_pool_exhausted`; 12 loads.
6. **Type hiding.** `equip_slots [ring_l, ring_r]` → `idc.ring`; `[neck]` → `idc.amulet`; `[trinket]` → `idc.relic`; `[belt]` → none; a staff (`cat.weapon_ranged`) → none; `[neck, trinket]` → `idc_slot_mixed`.
7. **Derived state.** Unknown type with `BUC` set: `know.unknown`; after `identify_type` with only RARITY set: `know.appraised`; a block: `know.classified` forever, never a target.
8. **True value.** The four §6.1 items give 286, 656, 619 and 70.
9. **Vendor value.** The §6.1 longsword with only quality visible: `value_vendor = 55`, sells for 16 at CHM 40; identified: sells for 85. Identified items always have `value_vendor == value_true`.
10. **Estimate.** Item `uid (7 << 40) | 42` has `u = −6692`. With `V = 286`: (skill 0, steps 0) shows "≈ 80–380"; (25, 0) "≈ 120–360"; (25, 2) "≈ 180–330"; (75, 4) "≈ 270–290".
11. **Containment.** For every `V` in `1..=2000`, every `err_bp` in `{300, 1000, 2500, 5000}` and every `u` in `{−10000, −1, 0, 1, 10000}`, `lo <= V <= hi`.
12. **Appraisal time.** Weapon `ilvl 7`, skill 20, ITU 50: need 23,600, rate 1,700, 14 passes. Ring `ilvl 25`, skill 0, ITU 40: 52 passes. Master type step, ring `ilvl 10`, second completion: need 57,150, rate 4,200, 14 passes. `rate(0, ITU 1) = 610`.
13. **Descent.** On `ev.folio_exited` the test-12 weapon at skill 20 gains 7,080 progress.
14. **Analysis.** ANL 60, appraisal 30, `ilvl 8`: `anl_bp = 5000`. Keyed folio 8, actor `(8 << 40) | 3`, seq 0: tick 50000 rolls 6052 (fail); tick 50001 rolls 2404 (2 reveals: RARITY, BUC). ANL 40, skill 0, `ilvl 25`: 500.
15. **Analysis lock.** A second Examine of the same item by the same slot at the same character level is refused without a draw.
16. **Curse sense.** ITU 55: 2500 Bp. Keyed folio 2, actor `(2 << 40) | 1`, tick 9000, seq 0 rolls 4498: no flag, `SENSED[s]` set, a second pickup by that slot draws nothing.
17. **Curse resistance.** PRV 70: 1500 Bp; PRV 40: 0, still one draw; an `OMEN` item draws nothing.
18. **Binding.** An unknown cursed ring with a failed resistance roll equips, binds and sets `BUC`, `AFFIX_NAME[c]`, `AFFIX_ROLL[c]`; unequip is refused.
19. **`roll_curse` draws.** Exactly 3 for every outcome and every category.
20. **Curse chance.** Weapon Gothic Horror 2400; weapon Noir 600; ring Mythic Fantasy 2300; potion Fairy Tale 1450; malign ring Cosmic Horror 9000; Legendary weapon Mythic Antiquity 1312; shop weapon Cyberpunk 412; anything in The Unwritten 0.
21. **Curse potency.** `curse.feeble` at `p = 3` on a Gothic Horror ring, wearer Gothic Horror Origin with ATN 40: PWR −15 in Mythic Antiquity, −10 in Cyberpunk, −9 in Noir.
22. **Disconnected curse.** At `C >= 6` the curse magnitude is 0, the item stays bound and `eff_enh` still subtracts `curse_penalty`.
23. **Price observation.** Healing potion, `ilvl 3`, buyer CHM 50: price 45; candidates = the six base-40 types; with `tried_no_effect`, {healing, clarity}.
24. **Deduction.** A single candidate is identified the same tick with reason `appr.reason.deduction`. With 11 of 12 amulet types known and no unknown amulet descriptor seen, nothing is identified.
25. **Text refusal.** An unknown Emendation pointed at a potion is not consumed, sets `read_mode = Targeted` and the potion row in `refused_cats`, and removes identify from its candidates.
26. **Services at depth 12.** appraise 51, classify 103, identify 154, unseal 77, unbind 387, consecrate 774.
27. **Cursed identify.** Keyed folio 5, actor `(5 << 40) | 9`, tick 30000, seq 0 over 5 items picks index 3. Blessed identify draws nothing.
28. **Amnesia.** Known non-text types `[A..G]`, keyed folio 4, actor `(4 << 40) | 2`, tick 20000, seq 0: uncursed amnesia forgets B and D; texts stay known.
29. **Wand.** Wrest keyed folio 6, actor `(6 << 40) | 4`, tick 40000, seq 0 rolls 7239: fails, `CHARGES` set. Recharge at `r0 = 1` draws nothing; at `r0 = 2` risks 1500 Bp; explosion at `ilvl 9`: 29,887 mHP.
30. **Familiarity.** F1 at 1800 ticks or 3 kills; affix slot 1 at 7200 or 16; F-type at 7200 or 15. Another weapon's kills never count for a weapon.
31. **Seals, in-run.** Sealed Rare accessory, `ilvl 12`: `seal_floor 412`, sells for 123 at CHM 40, cannot be equipped; breaking leaves only RARITY known; `appr.svc.unseal` costs 77 at depth 12 and fully identifies.
32. **Seals, account.** Breaking shows exactly the open response's content ID; a carried seal opens on the carrier's 5th `ev.item_identified` or at `ev.run_ended`.
33. **Illumination.** An illuminated Legendary ring's unlock is recorded on pickup; notification and render wait for `RARITY` or run end.
34. **Recollection.** Allowed once per Volume per slot; refused in Daily mode; a wrong guess adds one exclusion and draws nothing.
35. **Unread Edge.** At `d = 10`, SER 40 with 3 unidentified equipped items gives weights `[7000, 3124, 1104, 339, 120, 46, 0]`.
36. **Stacks.** Two otherwise identical potions, one with `BUC` set, do not merge.
37. **Replication.** An unappraised weapon's client packet has no rarity, enhancement or affix fields; an unknown-type potion's has its descriptor `id32` and no template `id32`.
38. **No hidden inputs.** Every function that returns a displayed chance, time or cost takes `&ItemView` (compile-time check); `ItemView` has no field for an unrevealed property.

## Invariants annex

```
Σ doc10: knowledge ⊗ appearances ⊗ methods ⊗ value ⊗ curses ⊗ seals ⊗ concordance
W! type_hidden(t) ⇔ idc_of(t) ≠ ∅                 W! |types_c| ≤ |pool_c| − 2 ∀ class c
W! shuffle := rng.appearance, run scope, once     W! shuffle draws = Σ(|pool_c| − 1) = 135
W! type knowledge ∈ party scope                   W! instance knowledge ∈ item
W! state == derive_state(flags, type known)       N! state lowered except by amnesia
W! hidden property ⇒ fully active                 N! displayed chance|time|cost ← hidden property
W! vendor knows type ⇒ price observation          W! value_vendor == value_true when identified
W! every rng.ident draw keyed; count per §5.12    N! draw count depends on hidden data except §5.2 step 1
W! roll_curse draws == 3                            W! curse chance == 0 when local MYTHOS == 0
W! curse affix = last affix ∧ dep {mythos}        W! bound ⇔ cursed ∧ equipped (any potency)
N! Contradiction releases binding                 W! Reliquary|Fortune Engine payout ⇒ RARITY known ∧ ¬cursed
W! estimate contains value_true                   W! u constant per uid
W! seal ⇒ content unchanged                       N! seal ∈ {sold, timed, money-accelerated}
W! Concordance ← play only                        N! Concordance ← SKU|Inkwell|account Reliquary
N! appearance mapping persists across runs        W! Recollection off in leaderboard modes
W! malign share of a consumable class = 3000 Bp   N! malign consumable > 3000 Bp Max HP direct damage
W! client receives ItemView only                  N! unrevealed property ∈ client packet
```
