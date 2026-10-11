# 14 — Run Structure and Meta Progression

As of 2026-10-11. Status: BINDING. Conforms to canon-1.1.

This chapter defines how a run starts in the Margin, how each Folio flows from arrival to exit, how a run is scored and ends, what the account keeps, the Margin's unlock tree (the Bindery), Echo Recollection trials, Errata Grades 0 to 20, every mode, Endless Marginalia scaling, leaderboards, achievements, the Prologue Folio, and save and suspend rules. Account currencies are `cur.quire` (Bindery) and `cur.ink` (Inkwell, doc 15), exactly as Canon §14 assigns them.

## Imports

| Source | Section | What this chapter consumes |
|---|---|---|
| Canon | §1.1 | ID grammar, `id32`, alias rule; every ID in Exports passes it |
| Canon | §1.3 | `floor_div`, `mul_bp`, left-to-right Bp chains, clamp-last rule |
| Canon | §1.5, §1.6 | display-unit rule for `Milli`; integer-only content |
| Canon | §1.11 | `folio_index = 0` for run-start entities and items; ascending-ID iteration |
| Canon | §2.3 | "N s" = N × 60 ticks; status intervals |
| Canon | §2.4, §2.5 | session header, replay log, save contract, co-op saves only at Folio entry |
| Canon | §3.1–§3.6 | `Rng`, sequential streams, keyed draws, commit-only `rng.offer.p<slot>`, seed display, daily and weekly seeds |
| Canon | §5.2, §5.4, §5.5 | genre table, Origin tolerance, Contradiction `C` (kit translation, exit forecasts) |
| Canon | §8 | XP curve, co-op XP split (mid-run joiner level) |
| Canon | §10.5 step 13 | the difficulty multiplier slot this chapter fills |
| Canon | §10.8, §10.9 | lethal resolution, healing order (rest rooms) |
| Canon | §11.6, §11.7 | Erasure accrual, hunters, pause in boss Folios; status multiplier order |
| Canon | §13.3–§13.8 | rarity, quality, `P(d)`, `T(d)`, `H(d)`, power depth cap 300 |
| Canon | §13.6 | `unlock.daily_baseline`, appearance shuffle at run start |
| Canon | §14 | `cur.lucre`, `cur.quire`, `cur.ink`, `cur.vellum` |
| Canon | §16 | Callings, Origins, Echo roster size (24, 2 per core genre) |
| Canon | §17.1–§17.3 | run constants, party tables, ranks |
| Canon | §18 | `ev.folio_entered`, `ev.folio_exited`, `ev.entity_killed`, `ev.player_downed`, `ev.player_revived`, `ev.erasure_stack`, `ev.hunter_spawned`, `ev.run_ended` |
| Canon | §19 | this chapter owns `run.`, `mode.`, `grade.`, `margin.`, `unlock.`, `echo.`, `score.`, `ach.` |
| Doc 04 | creation, traits | Erratum creation, attribute allocation, `trait.echo_*` signature traits (Interface assumptions) |
| Doc 05, 06, 07, 08, 11 | archetypes, glyphs, items | kit item IDs, item values (Interface assumptions) |
| Doc 12 | worldgen | Folio layout, room tags, lattice draws, Rubric placement |
| Doc 13 | director | spawns, elite promotion, Keeper, bosses, telegraphed-attack flags |
| Doc 15 | economy | Ink payout, Codex XP, account Reliquaries, Inkwell |
| Doc 16 | netcode | lobby, `st.downed`, host migration, ranked submission transport |
| Doc 17 | UI | Margin screens, exit preview widgets, `access.scribes_mercy` |

---

## 1. Constants and run state

### 1.1 Run-layer constants

| Constant | Value | Used in |
|---|---|---|
| `SPEED_PAR_S` | 360 s | §4.2 speed points |
| `TURN_HOLD_TICKS` | 120 | §3.6 exit commit |
| `EXIT_MAJORITY_TICKS` | 3600 | §3.6 majority fallback |
| `EXIT_ANY_TICKS` | 7200 | §3.6 single-voter fallback |
| `ERASURE_WARN_1_TICKS` | 18000 (300 s) | §3.8 UI cue |
| `ERASURE_WARN_2_TICKS` | 23400 (390 s) | §3.8 UI cue |
| `INTERLEAF_HEAL_BP` | 5000 | §3.9 |
| `REST_CHANNEL_TICKS` | 600 | §3.9 |
| `ENDPAPER_SATIATION` | 600 | §3.9 |
| `GHOST_RESTORE_HP_BP` | 5000 | §3.10 |
| `VOLUME_REWARD_TIMEOUT_TICKS` | 3600 | §3.9 |
| `HUNTER_SCORE_CAP_PER_FOLIO` | 3 | §4.2 |
| `QUIRE_DIVISOR` | 100 | §4.6 |
| `DIFF_CAP_BP` | 1,000,000 | §7.3, §9.2 |
| `MARGINALIA_MAX_FOLIO` | 9999 | §9.1 |
| `RECOLLECTION_ACTIVE_MAX` | 2 | §6.2 |
| `AUTOSAVE_PERIOD_TICKS` | 3600 | §12.1 |
| `RANKED_RESTORE_MAX` | 2 | §12.3 |
| `DORMANT_GRACE_S` | 300 s of wall-clock time | §8.8 |
| `Q_REF` | 300 Quires per hour of play | §5.4 pacing reference only |

### 1.2 Run state (simulation)

`RunState` lives in `palimpsest_sim` and is part of every save (Canon §2.5). Account data (§5) never lives in simulation state.

```rust
pub struct RunState {
    pub mode: Id32,                       // mode.*; switches standard -> endless once (§9.1)
    pub final_folio: u32,                 // 25, 15, 10 or 5; u32::MAX once Endless
    pub run_seed: u64,
    pub content_set: Vec<Id32>,           // sorted unlock-gated content, frozen at run start
    pub grade: u8,                        // 0..=20
    pub gp: GradeParams,                  // §7.2, merged with mutators at run start
    pub mutators: Vec<Id32>,              // run.mut.*, sorted by content ID string
    pub party: BTreeMap<u8, SlotState>,   // key = party slot 0..=3
    pub folio_index: u32,                 // 0 before Folio 1
    pub phase: FolioPhase,                // §3.2
    pub cur: FolioRun,                    // per-Folio run-layer state, reset at entry
    pub history: Vec<FolioRecord>,        // one per Folio entered, ascending index
    pub marg_mut: BTreeMap<Id32, u8>,     // run.marg.* -> rank 1..=5 (§9.3)
    pub score_at_victory: i64,            // 0 until victory (§4.2)
    pub end: Option<RunEndRecord>,
    pub ranked: RankedState,              // §12.3
}

pub enum SlotStatus { Active, Ghost, Dormant, Departed }

pub struct SlotState {
    pub entity: u64,                      // EntityId; kept across Folios (Canon §1.11)
    pub erratum: ErratumChoice,           // Custom{origin, calling} | Echo{echo}
    pub kit_variant: u8,                  // 0 = standard, 1 = alt
    pub status: SlotStatus,
    pub folios_present: u32,              // Folios entered while Active or Ghost
    pub lucre_norm: i64,                  // §4.1
    pub quires_paid: i64,                 // §8.8 departed payouts
}

pub struct FolioRecord {
    pub index: u32, pub genre: Id32, pub kind: FolioKind,   // Normal | Boss | Final
    pub entry_tick: u64, pub exit_tick: Option<u64>,
    pub rooms_total: u32, pub rooms_visited: u32,           // doc 12 room count, union over party
    pub notes_offered: Vec<Id32>, pub notes_done: Vec<Id32>,
    pub editor_killed: bool, pub telegraphed_hits: u32,
    pub hunters_killed: u32, pub erratum_deaths: u32,       // deaths that did not end the run
    pub party_size: u8,
}
```

---

## 2. Start flow from the Margin

### 2.1 Lobby state machine

The Lectern (`margin.lectern`) starts every run. The lobby is client-side meta state until `Commit`, after which the host builds `RunState`.

| State | Entered when | Exits to | Condition |
|---|---|---|---|
| `Idle` | Margin loaded | `ModeSelect` | a player interacts with the Lectern |
| `ModeSelect` | | `ErratumSelect` | the host picks a mode the host's account has unlocked (§8.1) |
| `ErratumSelect` | | `RunSetup` | every slot has a valid Erratum: Daily skips this state and fills slots by draw (§8.4) |
| `RunSetup` | | `ReadyCheck` | host sets Grade (≤ host's unlocked maximum), mutators (Custom), seed (Seeded, Custom) |
| `ReadyCheck` | | `Commit` | every connected player toggles Ready; a slot that disconnects returns the lobby to `ErratumSelect` for that slot |
| `ReadyCheck` | | `RunSetup` | host edits any setting; every Ready flag clears |
| `Commit` | | `Folio1` | ranked token obtained if the mode is ranked (§12.3) and `start_run` returns |
| any | host leaves | `Idle` | lobby disbanded |

A valid Erratum: Calling and Origin are unlocked on that player's account (or the slot uses an Echo the player owns), and for Weekly and Seeded baseline runs the Calling and Origin belong to `unlock.daily_baseline`.

### 2.2 Run initialization

```rust
fn start_run(cfg: RunConfig) -> RunState {
    // 1. Seed. Standard, Novella, Custom without a typed seed: host OS CSPRNG, outside the sim.
    //    Daily and Weekly: Canon §3.6 seeds. Seeded and Custom with a typed seed: decoded seed.
    let run_seed = cfg.seed;
    // 2. Content set (§2.3), frozen; content_hash per Canon §2.4 covers it.
    let content_set = resolve_content_set(&cfg);
    // 3. Grade and mutator parameters (§7.2, §8.7).
    let gp = merge_params(grade_params(cfg.grade), &cfg.mutators);
    // 4. Appearance shuffle (Canon §13.6) from rng.appearance.
    appearance_shuffle(run_seed, &content_set);
    // 5. Mode draws from rng.run with folio_index 0 (Daily §8.4, Weekly §8.5). Zero draws otherwise.
    // 6. Folio 1 genre (§3.1 rule F1).
    let g1 = folio1_genre(&cfg, run_seed);
    // 7. Errata: create entities in ascending slot order with folio_index 0 EntityIds.
    // 8. Kits (§2.4): ascending slot order, then kit-table order; uids from the folio_index 0 counter.
    //    Kit generation draws no random numbers.
    // 9. Calling lore (§2.4.3) marks item types known for the party.
    // 10. Enter Folio 1 (§3.3). The first save is written there.
}
```

**Rule F1 (Folio 1 genre).** Standard, Novella and Custom without a typed seed use the host Erratum's Origin genre, as Canon §17.1 states for Standard. Daily, Weekly, Seeded, and Custom with a typed seed use the first candidate drawn from `rng.lattice` with `folio_index = 0`. When the host's Origin is a secret genre (§5.3 shelf A), the seeded rule applies instead (Canon gaps, item 7).

### 2.3 Content sets

| Set | Contents |
|---|---|
| `unlock.set.starter` | content every account owns at creation: Callings Vanguard, Duelist, Gunslinger, Spellwright; Origins Mythic Fantasy, Gothic Horror, Cyberpunk, Weird West; standard kits; every item, room and event not gated by a `unlock.loot.*`, `unlock.room.*` or `unlock.event.*` node |
| account set | `unlock.set.starter` ∪ content of purchased Bindery nodes ∪ owned Inkwell Scripts (doc 15) ∪ owned Echoes |
| `unlock.daily_baseline` | `unlock.set.starter` ∪ all 10 Callings ∪ all 12 core Origins. No alt kits, no Bindery loot, room or event nodes, no Scripts, no Echoes, no secret Origins |

The host's set is the run's set (Canon §13.6). A guest whose Calling or Origin is outside the host's set still plays it: Callings and Origins are per-slot choices, not pool content.

### 2.4 Starting kits

Kit items are created with `rarity = rar.common`, `quality = q.normal`, `ilvl` = the Folio index at which the kit is created (1 at run start), `enhancement = 0`, `buc = buc.uncursed`, `curse_penalty = 0`, no affixes, and `knowledge = know.identified`. Kits are horizontal: every Calling kit's summed base value at ilvl 1, Lucre included, lies in 360..=440 Lucre (acceptance test 9).

#### 2.4.1 Universal kit `run.kit.universal` (every Erratum)

| Item | Qty | Slot or bag |
|---|---|---|
| `item.food.ration` | 3 | bag |
| `item.tool.bandage` | 2 | quickbar 1 |
| `item.tool.torch` | 4 | quickbar 2 |
| `item.tool.pick` | 1 | bag |
| Lucre | 50 | purse |

#### 2.4.2 Calling kits `run.kit.<calling>` (variant 0)

Armor sets: **light set** = `arch.armor.light_head`, `arch.armor.light_body`, `arch.armor.light_hands`, `arch.armor.light_legs`, `arch.armor.light_feet`; **medium set** and **heavy set** follow the same five-slot pattern with `medium_` and `heavy_`.

| Calling | Weapons and off-hand | Armor | Consumables and tools | Extra Lucre |
|---|---|---|---|---|
| Vanguard | `slot.weapon_1` `arch.melee.hafted_mace`; `slot.offhand` `arch.offhand.shield_medium` | heavy set | `item.potion.healing` ×1, `item.throwable.javelin` ×3, `mat.iron` ×4 | 0 |
| Duelist | `arch.melee.blade_long`; `slot.weapon_2` `arch.melee.blade_short` | light set | `item.throwable.knife` ×6, `item.potion.haste` ×1 | 20 |
| Gunslinger | `arch.ranged.sidearm`; `slot.weapon_2` `arch.ranged.longarm` | light set | `ammo.light` ×60, `ammo.heavy` ×20, `item.throwable.smoke` ×2 | 20 |
| Grenadier | `arch.ranged.launcher`; `slot.weapon_2` `arch.ranged.sidearm` | medium set | `ammo.explosive` ×12, `ammo.light` ×30, `item.throwable.frag` ×4, `item.tool.bandage` ×2 | 0 |
| Spellwright | `arch.ranged.wand`; `slot.weapon_2` `arch.melee.blade_short`; `slot.offhand` `arch.offhand.focus` | light set | `item.book.primer_evocation` (teaches `glyph.bolt`, `glyph.flame`, `glyph.ward`), `item.potion.aether` ×2 | 0 |
| Alchemist | `arch.melee.blade_short` | light set | `apparatus.field_kit` ×1, `item.container.flask` ×4, `ing.common_herb` ×6, `ing.common_mineral` ×4, `item.throwable.fire_flask` ×4, `item.throwable.acid_flask` ×2 | 0 |
| Shade | `arch.melee.blade_short` | light set | `item.tool.lockpicks` ×1, `item.throwable.knife` ×4, `item.throwable.smoke` ×3 | 40 |
| Artificer | `arch.ranged.longarm`; `slot.weapon_2` `arch.melee.hafted_mace` | medium set | `ammo.light` ×40, `gadget.turret_light` ×1, `item.tool.toolkit` ×1, `mat.scrap` ×10 | 0 |
| Warden | `arch.melee.polearm_spear`; `slot.offhand` `arch.offhand.shield_medium` | medium set | `item.book.primer_abjuration` (teaches `glyph.ward`, `glyph.mend`), `item.potion.healing` ×2 | 0 |
| Oracle | `arch.ranged.staff` (two-handed) | light set | `item.book.primer_divination` (teaches `glyph.sight`, `glyph.summon_wisp`, `glyph.bolt`), `item.potion.aether` ×2 | 0 |

Alt kits `run.kit.<calling>_alt` (variant 1, Bindery shelf A) change only the listed entries:

| Calling | Alt-kit change |
|---|---|
| Vanguard | `arch.melee.hafted_maul` (two-handed) replaces mace and shield |
| Duelist | `arch.melee.fist_wraps` replaces `arch.melee.blade_long` |
| Gunslinger | second `arch.ranged.sidearm` replaces longarm; `ammo.light` ×100 replaces both ammo stacks |
| Grenadier | `arch.ranged.flamer` replaces launcher; `ammo.fuel` ×200 replaces `ammo.explosive` |
| Spellwright | `arch.ranged.staff` replaces wand and focus; primer teaches `glyph.frost`, `glyph.chain`, `glyph.ward` |
| Alchemist | `arch.ranged.sling` and `ammo.stone` ×40 added; fire flasks become `item.throwable.poison_flask` ×4 |
| Shade | `arch.ranged.hand_crossbow` and `ammo.light` ×30 replace the throwing knives |
| Artificer | `gadget.drone_scout` replaces the turret; `arch.ranged.sidearm` replaces longarm |
| Warden | `arch.melee.hafted_mace` replaces spear; primer teaches `glyph.ward`, `glyph.cleanse` |
| Oracle | `arch.ranged.wand` and `arch.offhand.focus` replace staff; primer teaches `glyph.sight`, `glyph.bind`, `glyph.mend` |

#### 2.4.3 Calling lore

At run start each Calling marks two item types `know.classified` for the whole party (type knowledge is party-wide, Canon §13.6), after the appearance shuffle. Duplicates across slots have no extra effect.

| Calling | Known types |
|---|---|
| Vanguard | `item.potion.healing`, `item.text.mending` |
| Duelist | `item.potion.haste`, `item.text.blink` |
| Gunslinger | `item.potion.focus`, `item.text.mapping` |
| Grenadier | `item.potion.fortify`, `item.text.demolition` |
| Spellwright | `item.potion.aether`, `item.text.appraisal` |
| Alchemist | `item.potion.healing`, `item.potion.antidote` |
| Shade | `item.potion.invisibility`, `item.text.mapping` |
| Artificer | `item.text.appraisal`, `item.text.recharge` |
| Warden | `item.potion.healing`, `item.text.sanctify` |
| Oracle | `item.text.appraisal`, `item.potion.clarity` |

With `unlock.option.lens_pocket` a player may trade the second lore entry for one `item.trinket.lattice_lens` in `slot.trinket`.

#### 2.4.4 Margin Translation (kit skin genre)

Errata fall out of their own stories, so kit items carry the Erratum's Origin skin, except where that skin would arrive broken.

```rust
fn kit_skin_genre(entry: &KitEntry, e: &Erratum, g1: GenreId) -> GenreId {
    // C computed per Canon §5.5 for the entry's base-function component, with the
    // skin of entry.template in genre e.origin, at local genre g1, with e's run-start Tol.
    let c = contradiction(entry.template, e.origin, g1, &e.tolerance());
    if c <= 2 { e.origin } else { g1 }
}
```

Example: a Cyberpunk-origin Gunslinger in a Daily whose Folio 1 is Fairy Tale. A Cyberpunk sidearm skin depends on TECHNE with `Req = max(1, 9 - 2) = 7`, local TECHNE 1, `raw_gap = 6`, `Tol = 2 + 0 = 2` at ATN 40, so `C = 4`. Since 4 > 2 the sidearm is generated with the Fairy Tale skin. A Weird West-origin Gunslinger in a Steampunk Folio 1 keeps the Weird West skin (`Req = max(1, 4 - 2) = 2`, local 7, `C = 0`).

Kits are not regenerated after Folio 1. A mid-run joiner (§2.5) translates against the genre of the Folio they enter.

#### 2.4.5 Kit record

```json
{
  "id": "run.kit.gunslinger",
  "calling": "calling.gunslinger",
  "variant": 0,
  "lucre": 20,
  "entries": [
    { "template": "arch.ranged.sidearm", "slot": "slot.weapon_1", "qty": 1 },
    { "template": "arch.ranged.longarm", "slot": "slot.weapon_2", "qty": 1 },
    { "template": "ammo.light", "slot": "bag", "qty": 60 },
    { "template": "ammo.heavy", "slot": "bag", "qty": 20 },
    { "template": "item.throwable.smoke", "slot": "quickbar", "qty": 2 },
    { "template": "arch.armor.light_head", "slot": "slot.head", "qty": 1 },
    { "template": "arch.armor.light_body", "slot": "slot.body", "qty": 1 },
    { "template": "arch.armor.light_hands", "slot": "slot.hands", "qty": 1 },
    { "template": "arch.armor.light_legs", "slot": "slot.legs", "qty": 1 },
    { "template": "arch.armor.light_feet", "slot": "slot.feet", "qty": 1 }
  ],
  "lore": ["item.potion.focus", "item.text.mapping"],
  "consumable_entries": [4]
}
```

`consumable_entries` lists entry indices removed when `gp.kit_consumables = false` (Grade 10+, §7.1) or `run.mut.bare_kit` is active; `run.mut.bare_kit` also removes every armor entry. The universal kit is never removed.

### 2.5 Mid-run joiners

A player may join a co-op run only at a Folio entry (Canon §2.5 co-op save point). The joiner creates or picks an Erratum, then:

| Quantity | Value |
|---|---|
| Character level | `join_level = max(1, floor_div(Σ levels of Active slots, n_active) - 1)`; XP set to the cumulative XP of that level (Canon §8); allocation points for levels 2..=join_level are spent in the join screen (doc 04) |
| Calling major skills | `20 + min(30, 2 × (f - 1))` |
| Calling minor skills | `10 + min(15, f - 1)` |
| Kit | universal + Calling kit at `ilvl = f`, translated against the Folio-`f` genre, uids from Folio `f`'s counter |
| Lucre | `mul_bp(50, P(f))` plus the kit's extra Lucre scaled the same way |
| Score presence | `folios_present` starts at 1 |

Example: party levels 9, 11, 12 joining at Folio 8: `floor_div(32, 3) - 1 = 9`; majors `20 + 14 = 34`; minors `10 + 7 = 17`; Lucre `mul_bp(50, 18280) = 91`.

