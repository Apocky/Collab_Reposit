# 14 — Run Structure and Meta Progression

As of 2026-10-11. Status: BINDING. Conforms to canon-1.3.

This chapter defines how a run starts in the Margin, how each Folio flows from arrival through its Colophon to the exit, how a run is scored and ends, what the account keeps, the Margin's unlock tree (the Bindery), Echo Recollection trials, Errata Grades 0 to 20, every mode (including Canon §17.1 Abridged and the Canon §17.5 co-op modes), Endless Marginalia scaling, leaderboards, achievements, the Prologue Folio, and save and suspend rules. Account currencies are `cur.quire` (Bindery) and `cur.ink` (Inkwell), exactly as Canon §14 assigns them. Doc 15 owns every account earn rate (Canon §19: account economy rules for `cur.` are doc 15's); this chapter supplies the inputs doc 15 consumes (`run_score`, achievement grades, Echo slot bindings) and prices the Bindery against doc 15's Quire income.

## Imports

| Source | Section | What this chapter consumes |
|---|---|---|
| Canon | §1.1 | ID grammar, `id32`, alias rule; every ID in Exports passes it |
| Canon | §1.3 | `floor_div`, `mul_bp`, left-to-right Bp chains, clamp-last rule |
| Canon | §1.5, §1.6 | display-unit rule for `Milli`; integer-only content; `u64` written as `"0x"` hex strings |
| Canon | §1.11 | `folio_index = 0` for run-start entities and items; ascending-ID iteration |
| Canon | §2.3 | "N s" = N × 60 ticks; status intervals |
| Canon | §2.4, §2.5 | session header, replay log, save contract; single player saves at any tick boundary, co-op only at Folio entry |
| Canon | §3.1–§3.6 | `Rng`, sequential streams, keyed draws (`rng.loot` for Ghost item loss), commit-only `rng.offer.p<slot>`, seed display, daily and weekly seeds |
| Canon | §5.2, §5.4, §5.5, §5.8 | genre table, Origin tolerance, Contradiction `C` (kit translation, exit forecasts), local genre and bleed zones |
| Canon | §5.11 | Going Native at a Colophon; `st.unmoored` clears in a Colophon |
| Canon | §6.4 | rows 1, 4, 7 and 9 (pool maxima used by rests) |
| Canon | §8 | XP curve, co-op XP split (mid-run joiner level) |
| Canon | §10.5 step 13 | the difficulty multiplier slot this chapter fills |
| Canon | §10.8, §10.9 | lethal resolution, healing order (rest rooms) |
| Canon | §11.1, §11.6, §11.7 | status categories; Erasure accrual, hunters, pause in boss Folios, Red Ink threshold; status multiplier order |
| Canon | §13.3–§13.8 | rarity, quality, `P(d)`, `T(d)`, `H(d)`, power depth cap 300, Marginalia Emendations and enhancement cap |
| Canon | §13.6 | `unlock.daily_baseline`, appearance shuffle at run start |
| Canon | §13.9 | `gear_tier` from the Errata Grade; `illuminated` skin unlocks |
| Canon | §14 | `cur.lucre`, `cur.quire`, `cur.ink`, `cur.vellum` |
| Canon | §16.1–§16.3 | 13 Callings with major and minor skills, Origins (secret Origins by meta unlock), Echo roster (24, 2 per core genre), kin |
| Canon | §17.1 | run constants, the Colophon, `mode.abridged` with `d_eff`, `FOLIO_1_GENRE`, `EXIT_OFFERS`, `GENRE_REPEAT_RULE` |
| Canon | §17.2, §17.3 | party tables, ranks |
| Canon | §17.5, §17.6 | co-op modes `mode.ff_off`, `mode.ff_full`, `mode.loot_shared`; Ghosts, Remains and resurrection |
| Canon | §17.8 | the Margin plot |
| Canon | §18 | `ev.folio_entered`, `ev.folio_exited`, `ev.entity_killed`, `ev.block_changed`, `ev.player_downed`, `ev.player_revived`, `ev.player_died`, `ev.player_ghosted`, `ev.player_resurrected`, `ev.erasure_stack`, `ev.hunter_spawned`, `ev.run_ended` |
| Canon | §19 | this chapter owns `run.`, `mode.`, `grade.`, `margin.`, `unlock.`, `echo.`, `score.`, `ach.` |
| Doc 02 | §6.4–§6.6, §12 | intents (`ChooseExit`, `CommitOffer`), save container kind 1 (run suspend), version-mismatch flow (`prev` branch or Close the Book), headless replay verifier |
| Doc 03 | §2.2, §5.12 | tick system order (`sys.erasure`, `sys.despawn`, `sys.tick_end`); Ghost movement |
| Doc 04 | creation, traits | Erratum creation, attribute allocation, `trait.echo_*` signature traits (Interface assumptions) |
| Doc 05, 06, 07, 08, 11 | archetypes, glyphs, items | kit item IDs, item values (Interface assumptions) |
| Doc 12 | worldgen | Folio layout, room tags, lattice draws, Rubric placement, Colophon and Interleaf placement |
| Doc 13 | director | spawns, elite promotion, Keeper, bosses, telegraphed-attack flags, `sys.erasure` |
| Doc 15 | §2.2, §2.4, §3.8, §3.13 | Ink and Quire earn rates (a run pays `min(400, floor_div(run_score, 50))` Quires), achievement grade payouts (Quires 10/25/50/100, Ink 40/80/160/320), Folds and Echo kit A/B rule, Echo slot placeholders, Apocrypha rule |
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
| `INTERLEAF_PATH_BP` | 4000..=6000 | §3.9 Interleaf placement along the Incipit-to-Colophon path |
| `REST_CHANNEL_TICKS` | 600 | §3.9 |
| `ENDPAPER_SATIATION` | 600 | §3.9 |
| `GHOST_REMAINS_HP_BP` | 5000 | §3.10, Canon §17.6 (Remains carried through the exit) |
| `GHOST_BARE_HP_BP` | 2500 | §3.10, Canon §17.6 (Remains left behind) |
| `VOLUME_REWARD_TIMEOUT_TICKS` | 3600 | §3.9 |
| `BOON_OFFER_SIZE` | 3 | §3.9 Colophon altar (Canon §17.1) |
| `BOON_REROLLS` | 1 per Erratum per Colophon | §3.9 |
| `COLOPHON_ELITE_AFTER_KILLS` | 3 | §3.9 desecration hunters (Canon §17.1) |
| `HUNTER_SCORE_CAP_PER_FOLIO` | 3 | §4.2 |
| `MUTATOR_BP_CAP` | 20000 | §4.2 |
| `RUN_SCORE_BP` | 1460 | §4.6, the `run_score` handed to doc 15 |
| `DIFF_CAP_BP` | 1,000,000 | §7.3, §9.2 |
| `MARGINALIA_MAX_FOLIO` | 9999 | §9.1 |
| `RECOLLECTION_ACTIVE_MAX` | 2 | §6.2 |
| `AUTOSAVE_PERIOD_TICKS` | 3600 | §12.1 |
| `RANKED_RESTORE_MAX` | 2 | §12.3 |
| `DORMANT_GRACE_S` | 300 s of wall-clock time | §8.8 |
| `Q_REF` | 60 Quires per reference hour (doc 15's reference failed run, §4.6) | §5.4 pacing reference only |
| `Q_WIN` | 143 Quires per hour of a reference Grade-0 victory (§4.6) | §5.4 pacing reference only |

### 1.2 Run state (simulation)

`RunState` lives in `palimpsest_sim` and is part of every save (Canon §2.5). Account data (§5) never lives in simulation state.

```rust
pub struct RunState {
    pub mode: Id32,                       // mode.*; switches standard -> endless once (§9.1)
    pub structure: Structure,             // Standard | Novella | Abridged | Daily (§8.1); fixed at Commit
    pub final_folio: u32,                 // 25, 15, 10 or 5; u32::MAX once Endless
    pub run_seed: u64,
    pub content_set: Vec<Id32>,           // sorted unlock-gated content, frozen at run start
    pub grade: u8,                        // 0..=20
    pub gp: GradeParams,                  // §7.2, merged with mutators at run start
    pub mutators: Vec<Id32>,              // run.mut.*, sorted by content ID string
    pub coop: CoopModes,                  // { ff: Default | Off | Full, loot_shared: bool } (Canon §17.5, §8.8)
    pub party: BTreeMap<u8, SlotState>,   // key = party slot 0..=3
    pub folio_index: u32,                 // 0 before Folio 1
    pub phase: FolioPhase,                // §3.2
    pub cur: FolioRun,                    // per-Folio run-layer state, reset at entry
    pub history: Vec<FolioRecord>,        // one per Folio entered, ascending index
    pub marg_mut: BTreeMap<Id32, u8>,     // run.marg.* -> rank 1..=5 (§9.3)
    pub colophon_marked: bool,            // §3.9 desecration; never cleared
    pub colophon_hunter_kills: u8,        // §3.9, saturates at 255
    pub score_at_victory: i64,            // 0 until victory (§4.2)
    pub end: Option<RunEndRecord>,
    pub ranked: RankedState,              // §12.3
}

pub enum SlotStatus { Active, Ghost, Dormant, Departed }

pub struct SlotState {
    pub entity: u64,                      // EntityId; kept across Folios (Canon §1.11)
    pub erratum: ErratumChoice,           // Custom{origin, calling} | Echo{echo, trait_variant: u8}
    pub kit: Id32,                        // run.kit.* chosen in the Dramatis (§2.4)
    pub status: SlotStatus,
    pub folios_present: u32,              // Folios entered while Active or Ghost
    pub lucre_norm: i64,                  // §4.1
    pub quires_paid: i64,                 // §8.8 departed payouts
    pub going_native_used: bool,          // Canon §5.11, once per run
    pub boon: Option<Id32>,               // run.boon.* active in the current Folio (§3.9)
    pub boon_next: Option<Id32>,          // run.boon.* committed at this Folio's Colophon
    pub remains_carried: bool,            // Ghost only: set at exit commit (§3.10)
}

pub struct FolioRecord {
    pub index: u32, pub genre: Id32, pub kind: FolioKind,   // Normal | Boss | Final
    pub depth: u32, pub volume: u32,                        // §3.1: d(f) and v(f)
    pub entry_tick: u64, pub exit_tick: Option<u64>,
    pub rooms_total: u32, pub rooms_visited: u32,           // doc 12 room count, union over party
    pub notes_offered: Vec<Id32>, pub notes_done: Vec<Id32>,
    pub editor_killed: bool, pub telegraphed_hits: u32,
    pub hunters_killed: u32, pub erratum_deaths: u32,       // Erasure hunters only (§4.2); deaths that did not end the run
    pub party_size: u8,
    pub colophon_entry_tick: Option<u64>,                    // stops Erasure accrual (§3.8)
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
| `ErratumSelect` | | `RunSetup` | every slot has a valid Erratum, a kit it owns (§2.4) and, for an Echo, an owned trait variant (doc 15 Folds); Daily skips this state and fills slots by draw (§8.4) |
| `RunSetup` | | `ReadyCheck` | host sets structure (Seeded, Custom), Grade (≤ host's unlocked maximum), mutators (Custom), seed (Seeded, Custom) and, with 2 or more slots, the co-op modes `mode.ff_off` or `mode.ff_full` and `mode.loot_shared` (§8.8). When a simulation-affecting mod is loaded the screen first shows doc 15's Apocrypha notice and the run is Apocrypha (§8.1) |
| `ReadyCheck` | | `Commit` | every connected player toggles Ready; a slot that disconnects returns the lobby to `ErratumSelect` for that slot |
| `ReadyCheck` | | `RunSetup` | host edits any setting; every Ready flag clears |
| `Commit` | | `Folio1` | ranked token obtained if the mode is ranked (§12.3) and `start_run` returns |
| any | host leaves | `Idle` | lobby disbanded |

A valid Erratum: Calling and Origin are unlocked on that player's account (or the slot uses an Echo the player owns), and for Weekly and Seeded baseline runs the Calling and Origin belong to `unlock.daily_baseline`, the slot uses no Echo, and its kit is the Calling's standard kit (`unlock.daily_baseline` holds no Echoes, Scripts or alt kits).

| Lobby input | Valid values |
|---|---|
| Party slots | 1..=`PARTY_MAX` (4); the Prologue is 1 |
| Grade | `0..=max_grade(host)` (§7.4); Daily 2 and Weekly 6 are fixed |
| Structure | Standard, Novella or Abridged; Custom adds Standard with `run.mut.short_book` (§8.7) |
| Seed | 13-character Canon §3.6 display form (Seeded, Custom) |
| Co-op modes | none, `mode.ff_off`, `mode.ff_full` (mutually exclusive); `mode.loot_shared` on or off; hidden with 1 slot |

### 2.2 Run initialization

```rust
fn start_run(cfg: RunConfig) -> RunState {
    // 1. Seed. Standard, Novella, Abridged, Custom without a typed seed: host OS CSPRNG, outside the sim.
    //    Daily and Weekly: Canon §3.6 seeds. Seeded and Custom with a typed seed: decoded seed.
    let run_seed = cfg.seed;
    // 2. Content set (§2.3), frozen; content_hash per Canon §2.4 covers it.
    let content_set = resolve_content_set(&cfg);
    // 3. Grade and mutator parameters (§7.2, §8.7).
    let gp = merge_params(grade_params(cfg.grade), &cfg.mutators);
    // 4. Appearance shuffle (Canon §13.6) from rng.appearance.
    appearance_shuffle(run_seed, &content_set);
    // 5. Mode draws from rng.run with folio_index 0 (Daily §8.4, Weekly §8.5). Zero draws otherwise.
    // 6. Folio 1 genre (rule F1 below).
    let g1 = folio1_genre(&cfg, run_seed);
    // 7. Errata: create entities in ascending slot order with folio_index 0 EntityIds.
    // 8. Kits (§2.4): ascending slot order, then kit-table order; uids from the folio_index 0 counter.
    //    Kit generation draws no random numbers.
    // 9. Calling lore (§2.4.3) marks item types known for the party.
    // 10. Enter Folio 1 (§3.3). The first save is written there.
}
```

**Rule F1 (Folio 1 genre).** Standard, Novella, Abridged and Custom without a typed seed use the host Erratum's Origin genre (slot 0's Origin), as Canon §17.1 states for Standard. Daily, Weekly, Seeded, and Custom with a typed seed use the first candidate drawn from `rng.lattice` with `folio_index = 0`. When the host's Origin is a secret genre (§5.3 shelf A), the seeded rule applies instead, because secret genres are never lattice candidates and are reached only through secret exits (Canon §17.1; Canon gaps, item 7).

### 2.3 Content sets

| Set | Contents |
|---|---|
| `unlock.set.starter` | content every account owns at creation: Callings Vanguard, Duelist, Gunslinger, Spellwright; Origins Mythic Fantasy, Gothic Horror, Cyberpunk, Weird West; the 13 standard kits; every item, room and event not gated by a `unlock.loot.*`, `unlock.room.*`, `unlock.event.*`, `unlock.glyph.*` or `unlock.hireling.*` ID |
| account set | `unlock.set.starter` ∪ content of purchased Bindery nodes ∪ content granted by owned Inkwell Scripts (doc 15) ∪ owned Echoes |
| `unlock.daily_baseline` | `unlock.set.starter` ∪ all 13 Callings ∪ all 12 core Origins. No alt kits, no Bindery loot, room or event nodes, no Scripts, no Echoes, no Keepsakes, no secret Origins (Canon §13.6, doc 15) |

**Script-granted unlock IDs.** Doc 15's Scripts grant IDs in this chapter's `unlock.` namespace, outside the Bindery: `unlock.kit.<calling>_<name>` (a kit variant `run.kit.<calling>_<name>`, §2.4.2), `unlock.glyph.<family>` (a doc 06 glyph family), `unlock.room.<name>` (a doc 12 room variant) and `unlock.hireling.<type>` (a doc 13 hireling type). They are never sold for Quires. The content validator rejects a Script grant whose ID equals a Bindery node ID or `unlock.kit.<calling>_alt`.

The host's set is the run's set (Canon §13.6). A guest whose Calling or Origin is outside the host's set still plays it: Callings and Origins are per-slot choices, not pool content.

### 2.4 Starting kits

Canon §16.2 gives doc 04 ownership of kits, and doc 04 §2.5 fixes each Calling's kit by role ("1H hafted weapon, medium shield, ..."). This section binds every role to an exact template ID (doc 05, 06, 07, 08, 11 templates) and adds the run-layer rules around kits: alt kits, Calling lore, Margin Translation and the kit record (Canon gaps, item 2).

Kit items are created with `rarity = rar.common`, `quality = q.normal`, `ilvl` = the depth `d(f)` of the Folio at which the kit is created (1 at run start, §3.1), `enhancement = 0`, `buc = buc.uncursed`, `curse_penalty = 0`, no affixes, `illuminated = false`, `manufacturer = none`, `gear_tier` per Canon §13.9 from the run's Grade, and `knowledge = know.identified`; a type-hidden kit item adds its type to the party's known types with reason `appr.reason.kit` (doc 10). Kit generation draws no random numbers (doc 11 §2: kits are the only items not made by `generate_item`). Every alt or Script kit of a Calling has a total base value within 500 Bp of that Calling's standard kit (doc 15 §3.8 kit A/B rule; acceptance test 9).

#### 2.4.1 Universal kit `run.kit.universal` (every Erratum, doc 04 §2.5)

| Item | Qty | Slot or bag | Base value at ilvl 1 (doc 11) |
|---|---|---|---|
| `item.food.ration` | 2 | bag | 8 |
| `item.tool.bandage` | 2 | quickbar 1 | 12 |
| `item.potion.healing` (type-identified) | 1 | quickbar 2 | 24 |
| Lucre | `mul_bp(50, P(d))`, 50 at run start | purse | 50 |

Total 94 Lucre of base value at ilvl 1.

#### 2.4.2 Calling kits `run.kit.<calling>` (variant 0)

Armor sets: **light set** = `arch.armor.light_head`, `arch.armor.light_body`, `arch.armor.light_hands`, `arch.armor.light_legs`, `arch.armor.light_feet` (value 80); **medium set** = the same five slots with `medium_` (98). Doc 04's "cloth armor set" is the light set. Weapons go to `slot.weapon_1` unless marked `w2`; off-hand items go to `slot.offhand`; consumables go to the quickbar in table order, then the bag.

| Calling | doc 04 §2.5 role list | Exact templates | Calling-part value |
|---|---|---|---|
| Vanguard | 1H hafted weapon, medium shield, heavy armor (head, body, legs) | `arch.melee.hafted_mace`; `arch.offhand.shield_medium`; `arch.armor.heavy_head`, `arch.armor.heavy_body`, `arch.armor.heavy_legs` | 206 |
| Duelist | 1H long blade, 6 throwing knives, light armor set | `arch.melee.blade_long`; `item.throwable.knife` ×6; light set | 186 |
| Gunslinger | sidearm with 60 rounds, 4 throwing blades, light armor set | `arch.ranged.sidearm`; `ammo.light` ×60; `item.throwable.knife` ×4; light set | 182 |
| Grenadier | heavy ranged weapon with 120 rounds, 4 grenades, medium armor set | `arch.ranged.heavy_auto`; `ammo.heavy` ×120; `item.throwable.frag` ×4; medium set | 314 |
| Spellwright | casting focus, glyph tome (2 glyphs), light armor set | `arch.offhand.focus`; `item.book.primer_evocation` (teaches `glyph.ess.flame`, `glyph.ess.frost`); light set | 172 |
| Alchemist | dagger, field alembic, 3 bombs, 4 empty flasks | `arch.melee.dagger`; `apparatus.mortar` tier 1 and `apparatus.alembic` tier 1 (doc 08 requires a mortar to brew); `item.throwable.fire_flask` ×3; `item.container.flask` ×4 | 84 + apparatus |
| Shade | short blade, 5 lockpicks, light armor set, smoke bomb | `arch.melee.blade_short`; `item.tool.lockpicks` ×5; light set; `item.throwable.smoke` ×1 | 346 |
| Artificer | sidearm with 60 rounds, toolkit, 1 deployable turret, medium armor | `arch.ranged.sidearm`; `ammo.light` ×60; `item.tool.toolkit`; `gadget.turret_light`; medium set | 248 |
| Warden | spear, tower shield, medium armor set, 1 healing glyph | `arch.melee.polearm_spear`; `arch.offhand.shield_tower`; medium set; `item.book.primer_abjuration` (teaches `glyph.form.aura`, which with the universally known `glyph.ess.mend` makes an area heal, doc 06 §8.1) | 274 |
| Oracle | staff, holy symbol (prayer focus), light armor set | `arch.ranged.staff` (two-handed); `item.trinket.holy_symbol` in `slot.trinket`; light set | 180 + symbol |
| Netrunner | sidearm with 60 rounds, hacking deck, light armor set | `arch.ranged.sidearm`; `ammo.light` ×60; `item.tool.hacking_deck`; light set | 162 + deck |
| Beastbinder | spear, 1 tamed companion beast, light armor set | `arch.melee.polearm_spear`; light set; one tamed beast of doc 13 template `npc.companion.<origin_suffix>`, spawned beside its owner with a `folio_index = 0` EntityId and bound as a minion (Canon §15.1; the cap at skill 20 is exactly 1) | 138 |
| Monk | hand wraps (unarmed archetype), cloth armor set | `arch.melee.fist_wraps`; light set | 136 |

With the universal kit, standard kits total 230 (Monk) to 440 (Shade) Lucre of base value at ilvl 1, before the three values doc 11 has not yet published (`apparatus.*`, `item.trinket.holy_symbol`, `item.tool.hacking_deck`). Base value is a shop price, not power; doc 04 owns kit balance (Canon gaps, item 2).

Alt kits `run.kit.<calling>_alt` (variant 1, Bindery shelf A) change only the listed entries; each stays within 500 Bp of the standard kit's Calling-part value (the universal kit is identical in both):

| Calling | Alt-kit change | Alt value | Difference |
|---|---|---|---|
| Vanguard | `arch.melee.hafted_maul` (two-handed) and `item.throwable.javelin` ×4 replace mace and shield | 206 | 0 Bp |
| Duelist | `arch.melee.fist_wraps` replaces the long blade; `item.throwable.knife` ×10 replaces ×6 | 186 | 0 Bp |
| Gunslinger | `arch.ranged.longarm` and `ammo.heavy` ×30 replace sidearm, `ammo.light` and knives | 186 | +219 Bp |
| Grenadier | `arch.ranged.flamer` and `ammo.fuel` ×600 replace heavy weapon and `ammo.heavy`; `item.throwable.frag` ×6 replaces ×4 | 316 | +63 Bp |
| Spellwright | primer teaches `glyph.ess.storm` and `glyph.form.chain` instead | 172 | 0 Bp |
| Alchemist | `arch.ranged.sling` and `ammo.stone` ×40 replace the dagger; `item.throwable.poison_flask` ×4 replace the fire flasks | 80 + apparatus | −477 Bp at most |
| Shade | `arch.ranged.hand_crossbow` and `ammo.light` ×30 replace the short blade | 360 | +404 Bp |
| Artificer | `gadget.drone_scout` replaces the turret | 244 | −162 Bp |
| Warden | `arch.melee.hafted_mace` and `arch.offhand.shield_medium` replace spear and tower shield; `item.tool.bandage` ×3 added | 274 | 0 Bp |
| Oracle | `arch.ranged.wand` and `item.potion.clarity` ×1 replace the staff | 176 + symbol | −223 Bp at most |
| Netrunner | `arch.melee.blade_short` and `item.throwable.disruptor_flask` ×2 replace sidearm and ammo | 160 + deck | −124 Bp at most |
| Beastbinder | `arch.ranged.sling`, `ammo.stone` ×40 and `arch.offhand.shield_small` replace the spear | 140 | +144 Bp |
| Monk | `arch.melee.polearm_staff` and `item.throwable.knife` ×4 replace the hand wraps | 138 | +147 Bp |

Kit IDs: `run.kit.<calling>` is variant 0 and `run.kit.<calling>_alt` is variant 1 for each of the 13 Callings. Script kit variants `run.kit.<calling>_<name>` (doc 15) are written the same way, as replacements against the standard kit. An Echo's kit A is its Calling's standard kit and its kit B (doc 15 Fold 3) is its Calling's alt kit; an Echo may use kit B only at Fold 3 or above, whether or not the account owns the alt kit node.

#### 2.4.3 Calling lore

At run start each Calling marks two item types `know.classified` for the whole party (type knowledge is party-wide, Canon §13.6) with reason `appr.reason.kit` (doc 10), after the appearance shuffle and after the kits. Healing potions are already known from the universal kit, so no lore entry names them. Duplicates across slots have no extra effect. The Prologue applies no Calling lore (§11).

| Calling | Known types |
|---|---|
| Vanguard | `item.potion.fortify`, `item.text.mending` |
| Duelist | `item.potion.haste`, `item.text.blink` |
| Gunslinger | `item.potion.focus`, `item.text.mapping` |
| Grenadier | `item.potion.fortify`, `item.text.demolition` |
| Spellwright | `item.potion.aether`, `item.text.appraisal` |
| Alchemist | `item.potion.antidote`, `item.potion.regeneration` |
| Shade | `item.potion.invisibility`, `item.text.mapping` |
| Artificer | `item.text.appraisal`, `item.text.recharge` |
| Warden | `item.potion.shielding`, `item.text.sanctify` |
| Oracle | `item.text.appraisal`, `item.potion.clarity` |
| Netrunner | `item.text.mapping`, `item.text.recharge` |
| Beastbinder | `item.potion.antidote`, `item.text.summoning` |
| Monk | `item.potion.haste`, `item.potion.fortify` |

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
  "lucre": 0,
  "entries": [
    { "template": "arch.ranged.sidearm", "slot": "slot.weapon_1", "qty": 1 },
    { "template": "ammo.light", "slot": "bag", "qty": 60 },
    { "template": "item.throwable.knife", "slot": "quickbar", "qty": 4 },
    { "template": "arch.armor.light_head", "slot": "slot.head", "qty": 1 },
    { "template": "arch.armor.light_body", "slot": "slot.body", "qty": 1 },
    { "template": "arch.armor.light_hands", "slot": "slot.hands", "qty": 1 },
    { "template": "arch.armor.light_legs", "slot": "slot.legs", "qty": 1 },
    { "template": "arch.armor.light_feet", "slot": "slot.feet", "qty": 1 }
  ],
  "minions": [],
  "lore": ["item.potion.focus", "item.text.mapping"],
  "consumable_entries": [2]
}
```

```json
{
  "id": "run.kit.gunslinger_alt",
  "calling": "calling.gunslinger",
  "variant": 1,
  "base": "run.kit.gunslinger",
  "replace": [
    { "remove": [0, 1, 2],
      "add": [ { "template": "arch.ranged.longarm", "slot": "slot.weapon_1", "qty": 1 },
               { "template": "ammo.heavy", "slot": "bag", "qty": 30 } ] }
  ]
}
```

The kit validator recomputes `consumable_entries` from the rule below and rejects a record that disagrees, and rejects any alt or Script kit whose Calling-part value differs from the standard kit's by more than 500 Bp. `minions` lists doc 13 templates spawned and bound at run start (Beastbinder: `["npc.companion.<origin_suffix>"]`, resolved with the Erratum's Origin suffix). `consumable_entries` lists entry indices removed when `gp.kit_consumables = false` (Grade 10+, §7.1, and the Prologue, §11) or `run.mut.bare_kit` is active; `run.mut.bare_kit` also removes every armor entry. Consumable entries are exactly the entries whose template is in `cat.potion`, `cat.throwable`, `cat.text` or `cat.food`; ammunition, tools, books, gadgets, materials and containers are never consumable entries. The universal kit is never removed.

### 2.5 Mid-run joiners

A player may join a co-op run only at a Folio entry (Canon §2.5 co-op save point). The joiner creates or picks an Erratum, then:

| Quantity | Value |
|---|---|
| Character level | `join_level = max(1, floor_div(Σ levels of Active slots, n_active) - 1)`; XP set to the cumulative XP of that level (Canon §8); allocation points for levels 2..=join_level are spent in the join screen (doc 04) |
| Calling major skills | `20 + min(30, 2 × (d - 1))`, with `d = d(f)` (§3.1) |
| Calling minor skills | `10 + min(15, d - 1)` |
| Kit | universal + chosen kit at `ilvl = d`, translated against the Folio-`f` genre, uids from Folio `f`'s counter |
| Lucre | `mul_bp(50, P(d))` plus the kit's extra Lucre scaled the same way |
| Score presence | `folios_present` starts at 1 |

Example: party levels 9, 11, 12 joining Standard Folio 8 (`d = 8`): `floor_div(32, 3) - 1 = 9`; majors `20 + 14 = 34`; minors `10 + 7 = 17`; Lucre `mul_bp(50, 18280) = 91`. The same party joining Abridged Folio 5 (`d = d_eff = 8`) gets the same values.

---

## 3. Folio flow

### 3.1 Folio positions

Every structure maps the Folio index `f` (which drives `EntityId`, uids and per-Folio streams, Canon §1.11, §3.2) to a Volume `v(f)`, a structural position `pos(f)` and a depth `d(f)`. The depth feeds `P`, `T`, `H`, `S`, `ilvl`, `enemy_level` and every depth-scaled amount in this chapter.

| Structure | `v(f)` | `pos(f)` | `d(f)` |
|---|---|---|---|
| Standard, Novella, Daily, Endless (every `f`) | `floor_div(f - 1, 5) + 1` | `f - 5 × (v - 1)` | `f` |
| Abridged (`f ∈ 1..=15`, Canon §17.1) | `floor_div(f - 1, 3) + 1` | `[1, 4, 5][j - 1]` with `j = f - 3 × (v - 1)` | `d_eff = 5 × (v - 1) + [1, 3, 5][j - 1]` |

Abridged depths for `f = 1..=15` are 1, 3, 5, 6, 8, 10, 11, 13, 15, 16, 18, 20, 21, 23, 25. The second Folio of each Abridged Volume has `pos = 4` (Binding exits and an Interleaf) at the depth of Standard position 3; its boss Folios are `f = 3, 6, 9, 12, 15`, with `boss.editor_v1` to `boss.editor_v4` and `boss.ur_editor`.

| pos | Kind | Rubric (§3.4) | Keeper rank | Marginal notes | Rest rooms |
|---|---|---|---|---|---|
| 1, 2 | Normal | drawn | `rank.elite` | 2 | Colophon |
| 3 | Normal | drawn | `rank.champion` | 2 | Colophon |
| 4 | Normal, Binding exits | drawn | `rank.champion` | 2 | Interleaf, Colophon |
| 5 | Boss (Editor of Volume `v`) | Editor kill | none | 0 | Endpaper after the kill, then Colophon |

Every Folio ends in a Colophon (`room.colophon`, Canon §17.1) placed before its exit doors.

`f == final_folio` makes the pos-5 Folio **Final**: its Endpaper leads to victory (§3.2). Folio 25 of Standard and Folio 15 of Abridged (both `d = 25`) are always `genre.ur_text` with `boss.ur_editor` (Canon §17.1). Marginalia Folios (`f >= 26`) follow the same table; their Keepers are `rank.champion` at every position (§9.1).

### 3.2 Folio phase state machine

| From | To | Trigger (evaluated by the run layer at the end of each tick, rows in this order) |
|---|---|---|
| any | `Ended(erased)` | no Active Erratum is alive and not `st.downed`; every downed Erratum is killed first (`ev.entity_killed`) |
| any | `Ended(abandoned)` | host issues Abandon (§4.3) |
| (entry, Normal) | `Exploring` | Folio entry (§3.3) |
| `Exploring` | `ColophonOpen` | Rubric complete (§3.4); the Colophon entrance unseals on this tick |
| `ColophonOpen` | `ExitVote` | every Folio except the Final Folio of `mode.standard`: an Active, living Erratum's origin is inside `room.colophon`. On this tick `colophon_entry_tick` is set, so Erasure accrual stops (§3.8); the Colophon services open (§3.9); the lattice draw (doc 12) happens; in a Marginalia boss Folio a "Close the Book" door is appended after the exit doors |
| `ColophonOpen` (Final, `mode.standard`) | `VictoryChoice` | the same condition and effects; the doors are the Margins doors (§9.1) followed by one "Close the Book" door |
| `ExitVote` | next Folio entry | an exit door commits (§3.6) |
| `ExitVote` (Marginalia boss) | `Ended(margin_closed)` | the "Close the Book" door commits |
| (entry, Boss or Final) | `BossAntechamber` | Folio entry |
| `BossAntechamber` | `BossFight` | every Active, living, non-downed Erratum is inside the arena seal volume for `TURN_HOLD_TICKS` consecutive ticks |
| `BossFight` | `Endpaper` | the Editor is killed; the Erasure clock starts (§3.8) |
| `Endpaper` (Boss, including Marginalia boss) | `ColophonOpen` | every Active Erratum committed its Volume reward, or `VOLUME_REWARD_TIMEOUT_TICKS` passed since the Endpaper began; the Colophon entrance unseals on this tick |
| `Endpaper` (Final, `mode.standard`) | `ColophonOpen` | same condition |
| `Endpaper` (Final, any other mode) | `Ended(victory)` | same condition |
| `VictoryChoice` | `Ended(victory)` | the "Close the Book" door commits (§3.6 rule) |
| `VictoryChoice` | Folio 26 entry | a Margins door commits; that door's genre is Folio 26's; `mode` becomes `mode.endless` (§9.1) |
| `ExitVote` (Folio `MARGINALIA_MAX_FOLIO`) | `Ended(margin_exhausted)` | its only door, "Close the Book", commits; that Folio offers no exit doors |

`Ended(abandoned_version)` is applied outside the simulation by the meta layer (§12.2). The run layer is evaluated once per tick after doc 03's `sys.despawn` and before `sys.tick_end` (Interface assumptions, doc 03), so a transition on tick `t` sees every death, pickup and block change of tick `t`.

### 3.3 Arrival

```rust
fn enter_folio(run: &mut RunState, sim: &mut Sim, f: u32, genre: GenreId) {
    // Runs at the boundary before folio_entry_tick (doc 03 §2.1); t0 = folio_entry_tick.
    let t0 = sim.next_tick();
    run.folio_index = f;
    let (d, v) = (run.depth(f), run.volume(f));    // §3.1
    sim.reset_folio_counters(f);                   // Canon §1.11 EntityId and uid counters
    let layout = doc12::generate(run.run_seed, f, d, genre, bleed_inputs(run), run.folio_scale_bp(), &run.gp);
    for slot in run.party.values_mut() {           // ascending slot
        if slot.status == SlotStatus::Ghost {
            resurrect_ghost(sim, slot, f, t0);     // §3.10, Canon §17.6; status becomes Active
        }
        if matches!(slot.status, SlotStatus::Active | SlotStatus::Ghost) { slot.folios_present += 1; }
        slot.boon = slot.boon_next.take();         // §3.9: a boon lasts exactly one Folio
    }
    // Canon effects of Folio entry: st.erasure removed, Death's Door reset, Shock of the New cleared,
    // Contradiction re-evaluated (Canon §5.5, §5.6, §10.8, §11.6).
    place_errata_at_incipit(sim, &layout);          // spawn points in ascending slot order
    apply_boon_entry_effects(run, sim);             // §3.9, ascending slot
    let mut rr = Rng::from_seed64(stream_seed(run.run_seed, "rng.run", f as u64));
    if f >= 26 && (f - 26) % 5 == 0 { draw_marginal_mutation(run, &mut rr); }   // §9.3, first
    let notes = draw_notes(&layout, &mut rr);       // §3.5, second
    run.cur = FolioRun::new(f, &layout, notes, t0);
    run.history.push(FolioRecord::open(f, d, v, genre, &layout, t0, run.party_size_active()));
    run.phase = if layout.kind == FolioKind::Normal { FolioPhase::Exploring } else { FolioPhase::BossAntechamber };
    if run.colophon_marked { doc13::queue_colophon_hunter(&layout, run.colophon_hunter_kills); }   // §3.9
    save_folio_entry(run, sim);                     // §12; the co-op save point, at this boundary
    // doc 03 sys.tick_begin emits ev.folio_entered on tick t0.
}
```

The Incipit (`room.incipit`, doc 12) is a sanctuary: no spawn point lies within 24,000 mm of it and hostile AI never paths into it. The client shows the Folio title card for 180 ticks; the simulation does not pause. Party size `n` for §17.2 scaling is the number of Active slots after Ghost resurrection. Draws at entry, in order: Ghost item loss (keyed `rng.loot`, 0 or 1 per resurrected Ghost, §3.10), then the per-Folio `rng.run` stream: 0 or 1 Marginal-mutation draw, then 0 or 2 note draws.

### 3.4 Rubric (exit unlock)

Every Normal Folio has one Rubric. Doc 12 draws its type during generation with one `rng.worldgen.weighted` call over weights `[5000, 3000, 2000]` in the order below. The Colophon's entrance, and with it the exit doors behind it, stays sealed until the Rubric completes: doc 12 sets the `protect` bits (doc 03 §4.7) on the entrance cells and clears them on the completion tick. Sealing the entrance, not only the doors, keeps a party from stopping the Erasure clock (§3.8) before it has earned the exit. A completed Rubric never reverts. Boss Folios have no Rubric; the Editor's death plays its role (§3.2).

| ID | Setup (doc 12, doc 13) | Complete when |
|---|---|---|
| `run.rubric.keeper` | one NPC flagged `keeper`, rank per §3.1, placed in the room with the greatest path distance from the Incipit | the Keeper's `ev.entity_killed` |
| `run.rubric.catchwords` | three `item.quest.catchword` placed in three distinct rooms, each at path distance ≥ 30,000 mm from the Incipit and ≥ 20,000 mm from each other | the party has picked up all three (any Erratum, any order); catchwords cannot be dropped |
| `run.rubric.seal` | a Seal point with a 4,000 mm radius; doc 13 spawns a wave on hold ticks 0, 600 and 1200 | the Seal accumulates 1800 held ticks; a tick is held when at least one Active, living Erratum's origin is within 4,000 mm of the Seal center and no hostile NPC's origin is |

### 3.5 Marginal notes (optional objectives)

At entry of a Normal Folio, eligible notes (sorted by content ID) are collected. If more than 2 are eligible, two are drawn from the per-Folio `rng.run` stream: `i1 = range(|E|)`, remove, `i2 = range(|E| - 1)` (2 draws); otherwise all eligible notes are offered (0 draws). Each completed note grants 200 score (§4.2) and `mul_bp(mul_bp(30, P(d(f))), gp.lucre_bp)` Lucre to each Active Erratum, paid at exit commit. Example: Folio 10 at Grade 0 pays `mul_bp(30, 21718) = 65` Lucre per note.

| ID | Eligible when | Completed when (checked at exit commit) |
|---|---|---|
| `run.note.cartographer` | always | `rooms_visited == rooms_total` |
| `run.note.excavate` | always | the party broke ≥ 200 blocks in this Folio |
| `run.note.illuminate` | the layout has an Illumination vault | the vault was opened |
| `run.note.native` | always | every Active Erratum's equipped components all have `C = 0` |
| `run.note.purge` | at least one elite or champion was placed | every elite- or champion-rank NPC spawned in this Folio is dead |
| `run.note.rescue` | the layout has a rescue event | the stranded native is alive inside `room.colophon` |
| `run.note.scribe` | always (every Colophon holds a Scriptorium station, Canon §17.1) | at least one Rewrite was committed in this Folio, at a Scriptorium room or at the Colophon's station |
| `run.note.silent` | always | no hostile AI reached `aware.alert` or higher toward any Erratum before the Rubric completed (doc 09) |
| `run.note.swift` | always | `commit_tick - entry_tick <= 14400` (240 s) |
| `run.note.unbroken` | always | no `ev.player_downed` and no Erratum `ev.entity_killed` in this Folio |

### 3.6 Exits and exit choice

1. The draw: on the `ColophonOpen → ExitVote` (or `→ VictoryChoice`) tick, doc 12 draws the 3 lattice candidates from `rng.lattice` (Canon §17.1, §3.2). `EXIT_OFFERS` is evaluated on that tick: 3 if any party member (Active or Ghost) has effective ITU ≥ 70 or carries `item.trinket.lattice_lens`, else 2. The first `EXIT_OFFERS` candidates become doors `0..EXIT_OFFERS`.
2. Override: when the next Folio has depth 25 (Standard Folio 24, Abridged Folio 14), the draw still happens (stream consistency) and the doors are replaced by one door to `genre.ur_text`.
3. A discovered secret exit (doc 12) is appended as the last door.
4. Each door shows: genre name and palette, the four axioms, the next Folio's position (Binding for pos-4 Folios), and a Contradiction forecast per player: the maximum `C` over that player's equipped components evaluated at the candidate genre with current tolerance (UI only, doc 17).

Commit rule, run every tick during `ExitVote` and `VictoryChoice`:

```rust
fn vote(e: &Entity, doors: &[Door], st: &VoteState, colophon: &Volume) -> Option<usize> {
    // Standing in a door volume is a vote for that door. A doc 02 `ChooseExit { door }` intent
    // (accessibility, doc 17) is a vote while the voter's origin stays inside the Colophon.
    if let Some(d) = (0..doors.len()).find(|&d| doors[d].volume.contains_origin(e)) { return Some(d); }
    if colophon.contains_origin(e) { return st.intent_vote.get(&e.id).copied(); }  // cleared on leaving
    None
}

fn exit_commit(st: &mut VoteState, t: u64, doors: &[Door], voters: &[Entity], colophon: &Volume) -> Option<usize> {
    // voters: Active, alive, not st.downed Errata in ascending EntityId (never empty, §3.2 row 1)
    let n = voters.len();
    let since = t - st.open_tick;
    let need = if since >= EXIT_ANY_TICKS { 1 }
               else if since >= EXIT_MAJORITY_TICKS { n / 2 + 1 }
               else { n };
    for d in 0..doors.len() {
        let k = voters.iter().filter(|e| vote(e, doors, st, colophon) == Some(d)).count();
        st.hold[d] = if k >= need { st.hold[d] + 1 } else { 0 };
    }
    (0..doors.len()).find(|&d| st.hold[d] >= TURN_HOLD_TICKS)   // lowest index wins a tie
}
```

On commit at tick `t`, in this order: marginal notes are evaluated and paid; each Ghost slot's `remains_carried` is set when its Remains item is in the inventory of an Active Erratum (§3.10); in Marginalia each Erratum receives 1 Emendation (Canon §13.7); `exit_tick = t`; `ev.folio_exited` is emitted; every slot (Active, downed, Ghost) leaves together, with tamed beasts and hirelings within 16 m (Canon §15.1). The next Folio is generated and entered on the next tick boundary; transit consumes no simulation ticks.

Example: a 2-player party, one player idle outside every door. Before tick `open + 3600`, `need = 2`, no commit. From `open + 3600`, `need = 2/2 + 1 = 2`, still none. From `open + 7200`, `need = 1`; the active player standing in door 1 commits at `open + 7200 + 119` (hold reaches 120 on that tick, counting the first qualifying tick as 1).

### 3.7 Boss gates and boss Folios

- Exits of a pos-4 Folio are Binding doors; the chosen genre skins the boss Folio. The Interleaf of that Folio is the last rest before the Editor.
- A boss Folio's Incipit is the antechamber, a sanctuary. The arena seal opens per §3.2; the arena door then closes behind the party until the Editor dies (doc 12).
- Editors: `boss.editor_v1` to `boss.editor_v4` at Folios 5 to 20, `boss.ur_editor` at Folio 25 (doc 13). Abridged places them at Folios 3, 6, 9, 12 and 15 (§3.1). Novella's and the Weekly Gauntlet's final boss is `boss.editor_v3` at Folio 15; the Daily's is `boss.editor_v1` at Folio 5. Marginalia rotation is in §9.1.
- From boss-Folio entry until the Editor's death, the Erasure clock and satiation decay do not advance (Canon §11.6).
- Every hit an Erratum takes from an Editor attack that doc 13 flags `telegraphed` increments `telegraphed_hits` (§4.2).

### 3.8 Erasure pressure timeline

The Erasure clock is `c = t - clock_start`, where `clock_start` is the entry tick in Normal Folios and the Editor's death tick in boss Folios. Stack `k` is granted at `c = 25200 + 1800 × (k - 1)` (Canon §17.1 ERASURE_STEP) to every Active, living Erratum, on the same tick, until the Folio's `colophon_entry_tick` is set: from that tick no further stack is granted in the Folio (Canon §17.1). Stacks already held stay until the next Folio entry. Healing received at `k` stacks is × `10000 - 300 × k` (Canon §11.6). Doc 13's `sys.erasure` (doc 03 §2.2, system 32) grants the stacks; it reads `clock_start` and `colophon_entry_tick` from `RunState.cur`.

**Hunter threshold.** Each Erratum's threshold is `thr = max(6, 10 - red_ink_doses)` (Canon §11.6, §16.4). The first Erasure hunter spawns on the tick the first Erratum's stack count reaches its `thr`; another spawns every 5400 ticks (90 s) while any Erratum holds at least its `thr`. The Colophon stop ends stack accrual, not hunter spawns: a party that enters the Colophon holding its threshold keeps drawing hunters every 90 s until it exits. Example: an Erratum with 4 doses has `thr = 6`, so the first hunter arrives at `c = 25200 + 1800 × 5 = 34200` (570 s).

| Clock (s) | Clock (ticks) | Event | Healing received |
|---|---|---|---|
| 0 | 0 | entry; stacks cleared | ×10000 |
| 240 | 14400 | `run.note.swift` deadline | ×10000 |
| 300 | 18000 | UI cue 1: margins darken (client only) | ×10000 |
| 360 | 21600 | speed points reach 0 (§4.2) | ×10000 |
| 390 | 23400 | UI cue 2: Red Ink audio sting (client only) | ×10000 |
| 420 | 25200 | stack 1 | ×9700 |
| 450 | 27000 | stack 2 | ×9400 |
| 480 | 28800 | stack 3 | ×9100 |
| 510 | 30600 | stack 4 | ×8800 |
| 540 | 32400 | stack 5 | ×8500 |
| 570 | 34200 | stack 6 | ×8200 |
| 600 | 36000 | stack 7 | ×7900 |
| 630 | 37800 | stack 8 | ×7600 |
| 660 | 39600 | stack 9 | ×7300 |
| 690 | 41400 | stack 10; first Erasure hunter (Canon §11.6) | ×7000 |
| 780 | 46800 | second hunter | ×7000 |
| 870 | 52200 | third hunter; then one every 5400 ticks | ×7000 |

The table assumes no Red Ink doses and no Colophon entry before 870 s. All Errata gain stacks on the same tick, so all reach 10 together. Ghosts gain none. Example: a party that enters the Colophon at `c = 30000` holds 3 stacks (stack 3 at 28,800; stack 4 would come at 30,600) and keeps 3 until it exits; it never draws a hunter in that Folio.

### 3.9 Rest rooms

Three room kinds restore the party: the **Colophon** in every Folio (Canon §17.1), the **Interleaf** mid-way through every pos-4 Folio, and the **Endpaper** after every Editor kill.

**Colophon** (`room.colophon`, every Folio, before the exit doors). The entrance unseals per §3.2. The services open on the tick the first Active, living Erratum's origin enters the room (`colophon_entry_tick`):

| Service | Rule |
|---|---|
| Full heal | Each Active Erratum may claim it once per Colophon by interacting with the Hearth (no channel): HP set to Max HP and aether set to Max aether. It is a set, not healing, so Canon §10.9 multipliers and `st.erasure` do not reduce it. Grade modifiers never touch it. |
| Shop | doc 11 stock from `rng.loot_place`; buy prices × `gp.shop_price_bp` after Canon §6.4 row 37. Absent in `genre.unwritten` Folios, which have no shops (Canon §5.2). |
| Boon altar | below; 3 boons and 1 reroll per Erratum (Canon §17.1) |
| Scriptorium station | Rewrite and identification services (doc 11, doc 10) |
| Going Native | Canon §5.11: once per run per Erratum (`going_native_used`), sets the Erratum's Origin to the Folio's genre and recomputes tolerance |
| Unmoored | `st.unmoored` ends when the bearer's origin enters the room (Canon §5.11) |

**Boon altar.** Each Active Erratum gets one stored offer when the services open, built from its commit-only stream `rng.offer.p<slot>` (Canon §3.5): from a copy of the stream, draw `range(12)`, `range(11)`, `range(10)` over the 12 boons below in ascending ID order without replacement (3 draws). A **reroll** (at most `BOON_REROLLS = 1` per Colophon) advances the real stream past the stored offer's 3 draws and stores a new offer drawn the same way. A **commit** advances the real stream past the stored offer's 3 draws and sets `boon_next`. An offer still uncommitted at exit commit is discarded without advancing the stream. Ghosts get no offer. At the next Folio entry `boon_next` becomes `boon`, which lasts until that Folio's exit commit or the run's end; a boon never carries further.

| ID | Effect while active (one Folio) |
|---|---|
| `run.boon.appraising_eye` | every item the bearer picks up rises to at least `know.appraised` on pickup (doc 10); no draw |
| `run.boon.blessing` | `st.blessed` applied directly to the bearer at Folio entry and again at Rubric completion or the Editor's death |
| `run.boon.deep_pockets` | every Lucre gain of the bearer × 12500, applied by doc 11 after `gp.lucre_bp` |
| `run.boon.editors_bane` | Canon §10.5 step 13 factor 11000 on the bearer's hits against `rank.editor` (§7.3) |
| `run.boon.hunters_bane` | step 13 factor 13000 on the bearer's hits against `rank.hunter` (§7.3) |
| `run.boon.keen_edge` | +1000 Bp in the bearer's Canon §10.5 step 1 additive bucket, potency 10000 |
| `run.boon.provisioned` | on commit, 2 `item.food.ration` and 2 `item.tool.bandage` at `ilvl = d(f)` enter the bearer's bag; no effect in the next Folio |
| `run.boon.quickened` | `st.hasted` applied directly at Folio entry and again at Rubric completion or the Editor's death |
| `run.boon.second_wind` | `st.regenerating` applied directly at Folio entry and again at Rubric completion or the Editor's death |
| `run.boon.shadowed` | `st.invisible` applied directly at Folio entry |
| `run.boon.surveyor` | at Folio entry the bearer's map shows every room outline and the Colophon (a per-slot flag the client reads; rooms do not count as visited) |
| `run.boon.thick_hide` | step 13 factor 9000 on NPC hits against the bearer (§7.3) |

Balance: one boon is active at a time, so the strongest combat boons equal a permanent +10% damage or −10% damage taken, the gap between rarity steps Common and Rare (Canon §13.3). Doc 13 tunes the §17.4 anchors with one combat boon active from Folio 2. No boon changes a §6.4 formula, a canon constant or an account reward.

**Desecration** (Canon §17.1). The first `ev.block_changed` that breaks or transforms a cell inside a `room.colophon` volume, caused by a `fac.errata` member (Errata, minions, and their projectiles and explosions), sets `colophon_marked = true` for the rest of the run. In every later Folio, doc 13 spawns one `rank.hunter` inside the Colophon at Folio entry; it idles until an Erratum's origin enters the Colophon or it takes damage. Each such hunter's `ev.entity_killed` increments `colophon_hunter_kills`. Once `colophon_hunter_kills >= COLOPHON_ELITE_AFTER_KILLS` (3), the waiting hunter spawns with one doc 13 elite modifier. Colophon hunters are not Erasure hunters: they never count toward the §3.8 cadence or `S_hunters` (§4.2).

**Interleaf** (`room.interleaf`, in every pos-4 Folio including Abridged and Marginalia). A sanctuary room with a Hearth and a vendor (doc 12), placed on the critical path from the Incipit to the Colophon entrance at a path distance of `INTERLEAF_PATH_BP` (4000 to 6000 Bp) of that path's length, so it is a mid-Folio rest and not a second Colophon. Vendor prices × `gp.shop_price_bp`. Each Active Erratum may Rest once per Interleaf: a channel of `REST_CHANNEL_TICKS` that cancels if the Erratum takes damage, attacks, casts, or its origin moves more than 500 mm from where the channel began. On completion:

1. Heal `h = mul_bp(mul_bp(max_hp_mhp, INTERLEAF_HEAL_BP), gp.rest_heal_bp)`, applied as healing through Canon §10.9 (row 3, then status multipliers in ascending ID order per Canon §11.7, then the Max HP cap).
2. Stamina and aether set to max. Composure += `mul_bp(max_composure_m, 5000)`, capped at max.
3. Remove `st.bleeding`, `st.burning`, `st.poisoned`, `st.corroded`, `st.withering`, `st.diseased`.

Example: Max HP 300 (300,000 mHP) at Grade 6 (`rest_heal_bp = 7500`), RES 40, 3 Erasure stacks: `mul_bp(300000, 5000) = 150000`, `mul_bp(150000, 7500) = 112500`, row 3 ×10000 = 112,500, `st.erasure` ×9100 = 102,375 mHP healed.

**Endpaper** (every boss Folio, from the Editor's death tick):

1. Each Active Erratum: HP and composure set to `max(cur, mul_bp(max, gp.rest_heal_bp))` (a set, not healing); stamina and aether set to max; every status of category DoT, debuff, control or mental is removed; satiation `+= ENDPAPER_SATIATION`, capped at 2400.
2. Ghosts stay Ghosts; they resurrect at the next Folio entry (Canon §17.6, §3.10).
3. Volume reward: each Active Erratum receives one commit-only offer of 3 items from `rng.offer.p<slot>` (Canon §3.5), built by doc 11's `loot.volume_reward` table for Volume `v` (Palimpsestic items are eligible there). The player commits one. At `VOLUME_REWARD_TIMEOUT_TICKS` an uncommitted offer commits index 0.
4. Account Reliquary grants per doc 15 are recorded on the run for payout at run end (§4.5).
5. When step 3 resolves for every Active Erratum, the Colophon entrance unseals (§3.2).

### 3.10 Death, Ghosts and resurrection

- **Solo.** The Erratum's `ev.entity_killed` ends the run `run.end.erased` at the end of that tick (Canon §10.8 step 3, §17.6).
- **Co-op death.** Downing and revival follow Canon §10.8 and doc 16. When an Erratum dies while another Erratum is alive, it becomes a ghost with `st.ghost`, `ev.player_died` and `ev.player_ghosted` are emitted (Canon §10.8, §17.6), and its slot becomes `Ghost`. The body drops Remains: one `cat.corpse` item (doc 11 template `item.remains`) of 75,000 g, with a uid from the current Folio's counter and the ghost's slot recorded on it. The ghost keeps its equipment and bag, unusable while it is a ghost; the Remains hold no items. The ghost moves per doc 03 §5.12, cannot deal damage, pick up items or be perceived, and can open doors, press switches, ping and apply `st.marked` (Canon §17.6). It gains no XP (this chapter reads Canon §8's "present" as alive) and no instanced loot, and its needs are frozen (doc 04). The current Folio's `erratum_deaths` increments.
- **Resurrection** happens at the next `ev.folio_entered` and never earlier (Canon §17.6). At exit commit, `remains_carried` is true when the slot's Remains item is in an Active Erratum's inventory. At the boundary before `folio_entry_tick = t0` of Folio `f`, `resurrect_ghost` returns the entity with its original EntityId at the Incipit:
  1. HP `mul_bp(max_hp_mhp, GHOST_REMAINS_HP_BP)` when `remains_carried`, else `mul_bp(max_hp_mhp, GHOST_BARE_HP_BP)`.
  2. When `remains_carried` is false: `items` = the ghost's equipped items outside `slot.weapon_1..=3`, sorted by uid. If `items` is non-empty, `i = keyed_rng("rng.loot", f, ghost_entity, t0, 0).range(len(items))` (1 draw) and `items[i]` is destroyed; if it is empty, no draw happens.
  3. Stamina and aether full, composure `mul_bp(max, 5000)`, every status removed including `st.ghost`; the slot becomes `Active`; `ev.player_resurrected` is emitted on tick `t0`.
  4. The Remains item is destroyed wherever it is.
- A Ghost counts toward `folios_present` and toward `EXIT_OFFERS` (§3.6), and is never a voter. A run that ends while a slot is a Ghost counts that player as present for every achievement and milestone.

Example: a 2-player run; slot 1 dies in Folio 7 with five equipped non-weapon items. Slot 0 leaves the Remains behind. At Folio 8 entry `t0`, slot 1 returns with 25% Max HP, and `range(5)` on `keyed_rng("rng.loot", 8, slot1_entity, t0, 0)` picks the destroyed item.

---

## 4. Score, run end and payout

### 4.1 Accumulators

All score inputs are simulation state, so a replay reproduces the score exactly (Canon §2.4).

- `FolioRecord` fields (§1.2), updated by the run layer from events.
- `lucre_norm` per slot: on every Lucre gain of `a` units by that slot in Folio `f`, `lucre_norm += floor_div(a × 10000, P(d(f)))`. Example: 120 Lucre gained in Standard Folio 10 adds `floor_div(1200000, 21718) = 55`. Lucre from selling items counts; Lucre moved between Errata does not.
- `n_slots_ever`: number of distinct slots that were Active at any Folio entry.

### 4.2 Score formula

A Folio is **cleared** when its `exit_tick` is set, or when it is the Final Folio and its Editor is dead. For a cleared Folio, `end_f` is `exit_tick`, or the Editor's death tick for a Final Folio, and `secs_f = floor_div(end_f - entry_tick_f, 60)`. `d_f` is its depth and, for a Folio with an Editor kill, `v_f` is its Volume (§3.1) and `n_f` its `party_size`.

```
S_progress = Σ over cleared f of (600 + 40 × d_f)
S_editor   = Σ over f with editor_killed of max(500 × v_f, 1000 × v_f - 50 × floor_div(telegraphed_hits_f, n_f))
S_speed    = Σ over cleared f of 3 × max(0, 360 - secs_f)
S_explore  = Σ over cleared f of floor_div(200 × rooms_visited_f, rooms_total_f)
S_notes    = 200 × Σ |notes_done_f|
S_treasure = min(8000, floor_div(Σ over slots of lucre_norm, 2 × n_slots_ever))
S_hunters  = 300 × Σ over f of min(HUNTER_SCORE_CAP_PER_FOLIO, hunters_killed_f)   // Erasure hunters only (§3.9)
S_deaths   = -1500 × Σ over f of erratum_deaths_f          // deaths after which the run continued
base       = max(0, S_progress + S_editor + S_speed + S_explore + S_notes + S_treasure + S_hunters + S_deaths)
score      = mul_bp(mul_bp(mul_bp(base, victory_bp), grade_bp(G)), mutator_bp)
grade_bp(G) = 10000 + 500 × G + 50 × G²
mutator_bp  = min(MUTATOR_BP_CAP, chain of each active mutator's score_bp)
```

| End condition | `victory_bp` |
|---|---|
| `run.end.victory` of a run whose final Folio is the Ur-Text (Standard, Abridged, Seeded or Custom Standard structure) | 17500 |
| `run.end.victory` of every other run (Novella, Weekly Gauntlet, Daily, Custom with `run.mut.short_book`) | 15000 |
| `run.end.margin_closed`, `run.end.margin_exhausted` | 17500 |
| `run.end.erased` or `run.end.abandoned` after a victory (Endless) | 15000 |
| any other end, including `run.end.abandoned_version` | 10000 |

`mutator_bp` is the left-to-right `mul_bp` chain of each active mutator's `score_bp` in content-ID order, starting from 10000 (§8.7), then capped at `MUTATOR_BP_CAP = 20000`. The cap stops stacked hard mutators on the 10-Folio structure from outpaying a full run (§8.7). **Endless rule:** when a Margins door commits (§3.2), `score_at_victory` is computed with `victory_bp = 17500` and stored; the final score of an Endless run is `max(score_at_victory, score_now)`.

`victory_bp = 17500` for the Ur-Text and 10000 for a failed run is calibrated with `RUN_SCORE_BP` (§4.6) so doc 15's two reference runs land on its `run_score` targets.

UI breakdown IDs: `score.progress`, `score.editor`, `score.speed`, `score.explore`, `score.notes`, `score.treasure`, `score.hunters`, `score.deaths`.

**Worked examples** (all recomputed in exact integer arithmetic; R is doc 15's reference failed run):

| Input | A: solo death in F7 | B: solo victory | C: trio victory | D: Novella victory | E: Abridged victory | R: reference failed run |
|---|---|---|---|---|---|---|
| Grade, party | 0, 1 | 0, 1 | 10, 3 | 0, 1 | 0, 1 | 0, 1 |
| Cleared Folios | 1..=6 | 1..=25 | 1..=25 | 1..=15 | 1..=15 (depths 1..=25) | 1..=12, died in 13 |
| `secs_f` | 290, 310, 275, 330, 400, 305 | 300 each | 320 each | 240 each | 270 each | 300 each |
| Rooms visited / total | 7/10 each | 3/4 each | 4/5 each | 3/4 each | 3/4 each | 3/4 each |
| Editors, telegraphed hits | V1: 7 | V1–V5: 8 each | V1–V5: 21 each | V1–V3: 6 each | V1–V5: 8 each | V1–V2: 8 each |
| Notes done | 4 | 30 | 36 | 18 | 18 | 14 |
| Σ `lucre_norm` | 1,500 | 6,250 | 18,000 | 3,750 | 3,750 | 3,000 |
| Hunters, continued deaths | 0, 0 | 1, 0 | 2, 2 | 0, 0 | 0, 0 | 0, 0 |
| `S_progress` | 4,440 | 28,000 | 28,000 | 13,800 | 16,800 | 10,320 |
| `S_editor` | 650 | 13,000 | 13,250 | 5,100 | 13,000 | 2,200 |
| `S_speed` | 870 | 4,500 | 3,000 | 5,400 | 4,050 | 2,160 |
| `S_explore` | 840 | 3,750 | 4,000 | 2,250 | 2,250 | 1,800 |
| `S_notes` | 800 | 6,000 | 7,200 | 3,600 | 3,600 | 2,800 |
| `S_treasure` | 750 | 3,125 | 3,000 | 1,875 | 1,875 | 1,500 |
| `S_hunters` + `S_deaths` | 0 | 300 | 600 − 3,000 | 0 | 0 | 0 |
| `base` | 8,350 | 58,675 | 56,050 | 32,025 | 41,575 | 20,780 |
| `victory_bp`, `grade_bp` | 10000, 10000 | 17500, 10000 | 17500, 20000 | 15000, 10000 | 17500, 10000 | 10000, 10000 |
| **score** | **8,350** | **102,681** | **196,174** | **48,037** | **72,756** | **20,780** |
| `mode_quire_bp` (§8.1) | 10000 | 10000 | 10000 | 10000 | 7500 | 10000 |
| `run_score` (§4.6, full presence) | 1,219 | 14,991 | 28,641 | 7,013 | 7,966 | 3,033 |
| Quires (doc 15 §2.4) | 24 | 299 | 400 (cap) | 140 | 159 | 60 |

Example E's `S_progress` sums `600 + 40 × d_eff` over the 15 Abridged depths, whose sum is 195: `9000 + 7800 = 16800`. Quire rates per hour: B 299 in 125 min (143.5 per hour), D 140 in 60 min, E 159 in 67.5 min (141.3 per hour) and R 60 in 60 min. Abridged's `mode_quire_bp = 7500` brings its hourly rate in line with Standard; without it E would pay 212.

Example C editor term: `floor_div(21, 3) = 7`, so each Editor gives `1000 × v - 350` and the sum over `v = 1..=5` is `15000 - 1750 = 13250`.

### 4.3 Run end states

| ID | Trigger | Scope |
|---|---|---|
| `run.end.victory` | the Final Folio's Endpaper closes (§3.2) | party |
| `run.end.erased` | §3.2 row 1 | party |
| `run.end.abandoned` | solo: Abandon at any tick; co-op: host Abandon, confirmed by holding the menu button 120 ticks | party |
| `run.end.abandoned_version` | resume refused for a `sim_version` or `content_hash` mismatch and the player chooses Close the Book (§12.2, doc 02 §12.3) | party; applied by the meta layer from the save's checkpoint record |
| `run.end.margin_closed` | the Close door commits in a Marginalia Endpaper | party |
| `run.end.margin_exhausted` | Folio `MARGINALIA_MAX_FOLIO` is cleared | party |
| `run.end.departed` | a player leaves a co-op run (§8.8) | one slot; the run continues |

### 4.4 Run end procedure

```rust
fn end_run(run: &mut RunState, sim: &mut Sim, end: Id32) {
    let t = sim.tick();
    let score = compute_score(run, end);                 // §4.2, including the Endless max rule
    run.end = Some(RunEndRecord { end, tick: t, score });
    run.phase = FolioPhase::Ended;
    sim.emit(Ev::RunEnded { end, score });               // the simulation stops after this tick
    // Meta layer (outside palimpsest_sim), per local player:
    //   1. run_score (§4.6) to doc 15, which grants Quires; Ink, Codex XP and account Reliquaries via doc 15 from RunSummary
    //   2. Achievement and Recollection counters (§10, §6)
    //   3. Leaderboard submission if ranked (§9.4)
    //   4. Run history entry (last 200 runs kept), Margin summary screen
}
```

### 4.5 What is kept

| Kept on the account | Rule |
|---|---|
| Quires | doc 15 §2.4 from `run_score` (§4.6) |
| Ink | doc 15 §2.2 over `RunSummary` (active ticks, Folio exits, Editor kills, end state) |
| Codex XP | doc 15 formula over `RunSummary` |
| Account Reliquaries | grants recorded during the run (§3.9 step 4), credited at run end whatever the end state |
| Achievements and their counters | §10 |
| Recollection progress | §6.2 |
| Bestiary and Index knowledge | §5.5 |
| Milestones, Grade unlocks, discovered genres, best scores, run history | §5.1, §7.4 |
| Erratum presets (Origin, Calling, kit, appearance, name) | Dramatis slots |
| Illuminated item skins | Canon §13.9: picking up an `illuminated` item unlocks its skin account-wide, in every mode except the Prologue |
| Margin plot contents | Canon §17.8; built in the Margin, never in a run |
| Echoes, Folds, Scripts, cosmetics | doc 15 |

Never kept: items, Lucre, character level, XP, skills, sub-attributes, talents, reputation, type knowledge and appearance mappings, Erasure state, boons, Going Native, the Colophon mark and lattice history. Apocrypha runs (simulation-affecting mods, doc 15) keep nothing in this table except run history.

### 4.6 `run_score` and Quire payout

Doc 15 owns the Quire earn rate (Canon §19). This chapter hands it one number per slot:

```
presence_bp(slot) = floor_div(10000 × slot.folios_present, run.history.len())
run_score(slot)   = mul_bp(mul_bp(mul_bp(score, mode_quire_bp), presence_bp(slot)), RUN_SCORE_BP)
q(slot)           = min(QUIRES_PER_RUN_CAP, floor_div(run_score(slot), 50))      // doc 15 §2.4, cap 400
grant(slot)       = max(0, q(slot) - slot.quires_paid)                           // §8.8 departed payouts
```

`mode_quire_bp` is in §8.1. **Calibration.** Doc 15 asks for a `run_score` of 3,000 for its reference failed run and 15,000 for a reference victory. With `victory_bp = 17500` and `RUN_SCORE_BP = 1460`, reference run R scores 20,780 and gives 3,033, and Example B scores 102,681 and gives 14,991; both are within 1.1% of the targets. Example: Example C's score 196,174 for a player present in 18 of 25 Folios: `presence_bp = 7200`, `run_score = mul_bp(mul_bp(196174, 10000), 7200) = 141245`, then `mul_bp(141245, 1460) = 20621`, which grants 400 Quires (cap). Example D for a player present in 9 of 15 Folios: `presence_bp = 6000`, `run_score = 4208`, 84 Quires.

`RunSummary` (meta layer input, written into run history):

```json
{
  "run_seed": "0x0123456789ABCDEF",
  "mode": "mode.standard",
  "grade": 10,
  "mutators": [],
  "end": "run.end.victory",
  "end_tick": 482400,
  "deepest_folio": 25,
  "party_size_max": 3,
  "coop_modes": [],
  "apocrypha": false,
  "score": 196174,
  "components": { "score.progress": 28000, "score.editor": 13250, "score.speed": 3000,
                  "score.explore": 4000, "score.notes": 7200, "score.treasure": 3000,
                  "score.hunters": 600, "score.deaths": -3000 },
  "victory_bp": 17500,
  "grade_bp": 20000,
  "mutator_bp": 10000,
  "mode_quire_bp": 10000,
  "slot": { "index": 1, "calling": "calling.warden", "origin": "genre.wuxia", "echo": null,
            "kit": "run.kit.warden", "folios_present": 18, "presence_bp": 7200,
            "run_score": 20621, "quires_paid": 0, "active_ticks": 470000,
            "folio_exits": 17, "editor_kills_present": 4 },
  "ranked": false,
  "restores": 0
}
```

---

## 5. The Margin and meta progression

The Margin is a walkable first-person hub rendered by the client. It runs no gameplay simulation and draws no simulation random numbers. Account data (balances, unlocks, counters) is stored locally, signed, and synced when online (doc 15 offline end-of-service guarantee).

### 5.1 Facilities and milestones

| ID | Name | Function | Opens at |
|---|---|---|---|
| `margin.lectern` | The Lectern | modes, Grade, mutators, seeds, run start (§2.1) | `margin.milestone.prologue_done` |
| `margin.dramatis` | The Dramatis | Erratum presets, Echo selection and trait variant, kit, lens pocket | `prologue_done` |
| `margin.index` | The Index | item codex (§5.5) | `prologue_done` |
| `margin.bestiary` | The Bestiary | enemy codex (§5.5) | `prologue_done` |
| `margin.first_page` | The First Page | replays the Prologue (§11) | `prologue_done` |
| `margin.bindery` | The Bindery | unlock tree, paid in Quires (§5.2) | `first_run_end` |
| `margin.ledger` | The Ledger | run history and leaderboards (§9.4) | `first_run_end` |
| `margin.wardrobe` | The Wardrobe | cosmetics and Bindings | `first_run_end` |
| `margin.inkwell` | The Inkwell | gacha paid in Ink (doc 15) | `first_run_end` |
| `margin.codex_desk` | The Codex Desk | battle pass (doc 15) | `first_run_end` |
| `margin.store` | The Store | fiat SKUs (doc 15) | `first_run_end` |
| `margin.reliquary_vault` | The Reliquary Vault | opens account Reliquaries (doc 15) | `first_reliquary` |
| `margin.vellum_archive` | The Vellum Archive | Vellum shop (doc 15) | `first_vellum` |
| `margin.recollections` | The Recollection Alcove | Echo trials (§6) | `first_editor_v1` |
| `margin.proving_page` | The Proving Page | sandbox: target dummies, every unlocked item at ilvl 1, no rewards | purchase of `unlock.facility.proving_page` |
| `margin.plot` | The Plot | the account's 64 × 32 × 64 building plot (Canon §17.8), built with owned cosmetic block palettes; no gameplay stats | `first_run_end` |

| Milestone ID (`margin.milestone.*`) | Set when |
|---|---|
| `prologue_done` | the Prologue completes or is skipped |
| `first_run_end` | any run of any mode other than the Prologue ends |
| `first_editor_v1` | the account's player is present when `boss.editor_v1` dies |
| `first_editor_v2` | same for `boss.editor_v2` |
| `first_victory` | `run.end.victory` in Standard, Abridged or Novella |
| `first_ur_text_victory` | `run.end.victory` in Standard or Abridged (`boss.ur_editor` killed) |
| `first_standard_victory` | `run.end.victory` in Standard |

Milestones never unset. Apocrypha runs, easing-mutator runs and Prologue replays set no milestone except `first_run_end`.
| `first_reliquary` | the first account Reliquary is credited |
| `first_vellum` | the first Vellum is credited (doc 15) |

### 5.2 Bindery rules

- Every node, cost and prerequisite is visible from the first visit. There is no randomness and no discount.
- A purchase is atomic: it requires `quires >= cost`, every `requires` node owned, and the node's gate satisfied. It subtracts `cost` and adds the node. Purchases are permanent and never refunded.
- Quires are `i64`, never negative, never expire, never convert into any other currency, and cannot be bought (Canon §14).
- Every effect is horizontal: it adds options, pool content, cosmetics or convenience. No node raises a stat, damage, HP, resource maximum, drop rate, rarity weight or Lucre amount. Doc 11 builds every pool item with the same rarity power and `ITEM_MULT_MAX` as base items (Canon §13.8), and adds Lexicon templates inside their rarity band, renormalizing template weights within the band so each rarity's share of every table is unchanged (the doc 15 Script neutrality rule, applied to Quire unlocks too).
- Prices are set against doc 15's Quire income (§5.4), not against a separate run payout.

### 5.3 The unlock tree

**Shelf A: Dramatis** (Callings, Origins, kits)

| ID | Quires | Requires / gate | Effect |
|---|---|---|---|
| `unlock.calling.grenadier` | 100 | none | Grenadier selectable |
| `unlock.calling.alchemist` | 100 | none | Alchemist selectable |
| `unlock.calling.monk` | 100 | none | Monk selectable |
| `unlock.calling.shade` | 150 | `unlock.calling.alchemist` | Shade selectable |
| `unlock.calling.artificer` | 150 | `unlock.calling.grenadier` | Artificer selectable |
| `unlock.calling.netrunner` | 150 | `unlock.calling.artificer` | Netrunner selectable |
| `unlock.calling.beastbinder` | 150 | `unlock.calling.monk` | Beastbinder selectable |
| `unlock.calling.warden` | 200 | any of shade, artificer | Warden selectable |
| `unlock.calling.oracle` | 200 | `unlock.calling.warden` | Oracle selectable |
| `unlock.origin.<g>` for `space_opera`, `post_apocalypse`, `hardboiled_noir`, `wuxia`, `steampunk`, `fairy_tale`, `mythic_antiquity` | 75 each | none | that core Origin selectable |
| `unlock.origin.cosmic_horror` | 125 | any 3 `unlock.origin.*` nodes | Cosmic Horror Origin selectable |
| `unlock.origin.oneiric` | 250 | gate: the account entered `genre.oneiric` | secret Origin selectable |
| `unlock.origin.redacted` | 250 | gate: entered `genre.redacted` | secret Origin selectable |
| `unlock.origin.unwritten` | 250 | gate: entered `genre.unwritten` outside the Prologue | secret Origin selectable |
| `unlock.origin.ur_text` | 375 | gate: `first_ur_text_victory` | secret Origin selectable |
| `unlock.kit.<calling>_alt` (13 nodes) | 60 each | Calling available; gate: `boss.editor_v1` killed while playing that Calling | alt kit (§2.4.2) selectable in the Dramatis |

**Shelf B: Lexicon** (loot pool additions; each adds doc 11 pool tag `loot.pool.<suffix>` to the account set)

| ID | Quires | Requires | Adds |
|---|---|---|---|
| `unlock.loot.exotic_arms` | 150 | none | 12 Exotic weapons |
| `unlock.loot.exotic_armor` | 150 | none | 8 Exotic armor pieces |
| `unlock.loot.legendary_perks_1` | 100 | none | 10 legendary perks |
| `unlock.loot.legendary_perks_2` | 150 | `legendary_perks_1` | 10 legendary perks |
| `unlock.loot.cross_glosses` | 100 | none | the cross-genre affix family (a manufacturer trait from one genre on another genre's item) |
| `unlock.loot.implants_2` | 125 | none | 8 implants |
| `unlock.loot.relics_2` | 125 | none | 10 trinkets |
| `unlock.loot.throwables_2` | 75 | none | 6 throwables |
| `unlock.loot.glyphs_2` | 125 | none | 8 spell glyphs (doc 06) |
| `unlock.loot.recipes_2` | 100 | none | 12 crafting recipes (doc 07) |
| `unlock.loot.brews_2` | 100 | none | 8 brewing recipes and 4 ingredients (doc 08) |
| `unlock.loot.gadgets_2` | 100 | none | 6 gadgets (doc 07) |

**Shelf C: Cartography** (doc 12 room and event pools)

| ID | Quires | Adds |
|---|---|---|
| `unlock.room.fortune_engines_2` | 75 | 3 Fortune Engine variants (doc 11) |
| `unlock.room.hireling_hall` | 125 | Hireling Hall room (doc 13 hirelings) |
| `unlock.room.bleed_shrine` | 100 | Bleed Shrine room: the party can re-anchor one bleed zone to the Folio genre |
| `unlock.room.wandering_scribe` | 75 | wandering `fac.neutral` vendor event |
| `unlock.room.illumination_vault` | 150 | Illumination vault room (enables `run.note.illuminate`) |
| `unlock.room.duel_page` | 100 | optional champion duel room |
| `unlock.event.genre_storms` | 100 | moving bleed-storm events (doc 12) |

**Shelf D: Options**

| ID | Quires | Requires | Effect |
|---|---|---|---|
| `unlock.option.seed_journal` | 125 | none | the Ledger can launch any past run's seed as a Seeded run with its route shown |
| `unlock.option.lens_pocket` | 200 | none | §2.4.3 trade: second lore type for a Lattice Lens |
| `unlock.option.dramatis_slots` | 50 | none | Erratum presets 4 → 10 |
| `unlock.mutator.<m>` (16 nodes, §8.7) | 40 each | gate: `first_victory` | mutator `run.mut.<m>` selectable in Custom |

**Shelf E: Facilities and cosmetics**

| ID | Quires | Requires / gate | Effect |
|---|---|---|---|
| `unlock.facility.proving_page` | 75 | gate: `first_editor_v1` | opens `margin.proving_page` |
| `unlock.binding.<b>` for `vellum`, `lapis`, `verdigris`, `oxblood`, `gilt`, `soot` | 50 each | none | grants cosmetic `cosm.binding.<b>` (UI theme and journal binding, doc 15 catalog) |
| `unlock.palette.<g>` for the 12 core genre suffixes | 25 each | gate: the account cleared a Folio of genre `g` | grants cosmetic block palette `cosm.palette.<g>` for the Margin plot (Canon §17.8, doc 15 catalog) |

Bindery node record:

```json
{
  "id": "unlock.calling.warden",
  "shelf": 0,
  "cost_quires": 200,
  "requires_all": [],
  "requires_any": ["unlock.calling.shade", "unlock.calling.artificer"],
  "gate": null,
  "grants": ["calling.warden"]
}
```

```json
{
  "id": "unlock.origin.ur_text",
  "shelf": 0,
  "cost_quires": 375,
  "requires_all": [],
  "requires_any": [],
  "gate": "margin.milestone.first_ur_text_victory",
  "grants": ["genre.ur_text"]
}
```

### 5.4 Totals and pacing

| Shelf | Nodes | Quires |
|---|---|---|
| A: Callings | 9 | 1,300 |
| A: Origins (8 core + 4 secret) | 12 | 1,775 |
| A: Alt kits | 13 | 780 |
| B: Lexicon | 12 | 1,400 |
| C: Cartography | 7 | 725 |
| D: Options and mutators | 19 | 1,015 |
| E: Facilities, Bindings and palettes | 19 | 675 |
| **Total** | **91** | **7,670** |

Unlocking everything costs **7,670 Quires**. Achievements pay 1,935 Quires (§10, doc 15 grade table), leaving 5,735 from runs and the Codex free track (250 Quires per Codex, doc 15). At `Q_REF = 60` Quires per hour (only reference failed runs) that is 95.6 hours; at `Q_WIN = 143` Quires per hour (only reference Grade-0 victories, 299 Quires per 125 minutes) it is 40.1 hours. A typical player, who fails early and wins later, finishes between the two, near the doc 15 Codex target of 60 hours.

Why these prices: doc 15 pays a run `min(400, floor_div(run_score, 50))` Quires (§4.6), so a reference hour earns 60 Quires. At the previous prices (27,100 Quires) the tree would have taken over 300 hours at that rate, so every price was divided by about 3.5 and rounded to a multiple of 5. Starter options stay cheap (a first Calling costs 100 Quires, under two reference hours), and the gated secret Origins stay the most expensive single nodes.

### 5.5 Codex knowledge

- **Bestiary.** Per enemy template, the account counts kills made while the player was present. 1 kill: entry, art and lore. 10 kills: that template's resistances and immunities are shown on the target HUD in every run. 25 kills: its drop categories are listed.
- **Index.** Per item template: an entry on the first `know.identified` instance, then one skin entry per genre in which the player held it. Appearance mappings are never stored, since they reshuffle every run (Canon §13.6).

---

## 6. Echoes and Recollection trials

### 6.1 Rules

- An Echo (Canon §16) fixes kin, Origin, Calling, signature trait and cosmetics. Its creation preset (allocation, Origin skill, kin) is doc 04's (§2.6). It uses its Calling's standard kit as kit A and the Calling's alt kit as kit B from Fold 3 (§2.4, doc 15 §3.8).
- An Echo is acquired through the Inkwell (doc 15) or by completing its Recollection trial here. Both grant identical ownership.
- Trials open with `margin.recollections`. A trial is available when its Echo's Calling is on the account.
- At most `RECOLLECTION_ACTIVE_MAX = 2` trials are active. Activation and deactivation happen at the Alcove between runs. Progress persists while a trial is inactive but advances only while active.
- Counters advance at run end from that run's per-slot event counters (stored in `RunState`, so a crash-resume loses nothing). Each step's counter caps at its target. Steps progress concurrently, in any order, across any number of runs.
- Runs of every mode count except the Prologue, Apocrypha runs (doc 15) and Custom runs with an easing mutator (§8.7). An "as X" step counts only runs in which the player's slot used Calling X.
- Completion grants the Echo at Fold 0 (doc 15 §3.8). If the Echo is already owned, completion grants only the cosmetic `cosm.echo.<suffix>.recollected` (doc 15 catalog): no Fold, no Ink and no Vellum, because doc 15 lists every Ink and Vellum source and a Recollection is not one of them. Each trial completes at most once per account.

### 6.2 Step kinds and hour model

Each step kind has a cost in **centihours** (ch, 1/100 hour of reference play) per unit, derived from the reference pace of 10 Folios and 400 kills per hour.

| Step kind | Counts | ch per unit |
|---|---|---|
| `clear_genre` | Folios of genre G cleared | 35 |
| `editor_as` | `boss.editor_v<V>` killed while playing Calling X | 80 × V |
| `kills_type` | kills whose killing hit's largest-share damage type is T | 25 per 100 |
| `kills_edge` | kills of native targets by a base component with `E >= 1` | 1 per 3 |
| `sneak` | kills by sneak attack | 1 per 2 |
| `blocks` | blocks broken | 1 per 50 |
| `identify` | item instances reaching `know.identified` | 2 |
| `brew`, `craft` | `ev.potion_brewed`, `ev.item_crafted` | 3 |
| `notes` | marginal notes completed | 12 |
| `rewrites` | Rewrites committed | 20 |
| `deaths_door` | Death's Door saves | 40 |
| `erasure_exit` | Folios exited while holding ≥ 5 Erasure stacks | 50 |
| `hunters` | Erasure hunters killed | 60 |
| `secret_exit` | secret exits taken | 300 |

`est_ch(trial) = Σ steps of floor_div(count × unit_numerator, unit_denominator)`. **Hour bounds:** Tier 1 trials satisfy `400 <= est_ch <= 900` (4 to 9 hours); Tier 2 trials satisfy `900 <= est_ch <= 1600` (9 to 16 hours). The content validator rejects any trial outside its bound. Both bounds stay below the doc 15 Inkwell reference of one featured Echo per 19.6 hours of play.

### 6.3 Roster, signature traits and trials

Traits are `trait.echo.<suffix>` (suffix = the Echo ID's suffix) in doc 04's `trait.` namespace, authored here as doc 04 §2.6 asks; the effect column is variant A and passes doc 04's trait power-budget validator. Variants B and C (`trait.echo.<suffix>_b`, `_c`, doc 15 Folds 2 and 5) ship in the content catalog with the same budget and are validated the same way. Genre abbreviations: MF Mythic Fantasy, GH Gothic Horror, CH Cosmic Horror, SO Space Opera, CP Cyberpunk, PA Post-Apocalypse, WW Weird West, HN Hardboiled Noir, WX Wuxia, SP Steampunk, FT Fairy Tale, MA Mythic Antiquity.

| Echo | Origin, Calling, kin | Signature trait (variant A) | Trial steps (genre steps use the Echo's Origin) | ch |
|---|---|---|---|---|
| `echo.aldric_unsworn` | MF, Vanguard, human | block stability +1500 Bp; healing from prayers ×5000 | 6 Folios; V2 as Vanguard; 400 `dmg.blunt` kills | 470 |
| `echo.wren_long_index` | MF, Spellwright, fae | +1 Grimoire slot; complexity budget −1 | 10 Folios; V3 as Spellwright; 1,300 `dmg.arcane` kills | 915 |
| `echo.vesper_crane` | GH, Shade, undead | sneak multiplier +2000 Bp at `light_at ≤ 4`, −2000 Bp at `light_at ≥ 10` | 6 Folios; 300 sneak kills; V2 as Shade | 520 |
| `echo.mother_ilse` | GH, Alchemist, human | brews of 2+ potions yield +1; toxicity decay 300 per s instead of 500 | 10 Folios; 110 brews; V3 as Alchemist | 920 |
| `echo.dr_halloway` | CH, Oracle, human | composure loss ×7500; Max composure −20 | 6 Folios; 60 identifications; V2 as Oracle | 490 |
| `echo.drowned_clerk` | CH, Spellwright, undead | spell potency +1500 Bp while `st.soaked`; `dmg.fire` spell potency −2000 Bp | 11 Folios; 4 Erasure exits; V4 as Spellwright | 905 |
| `echo.ostra_vey` | SO, Gunslinger, human | swap speed ×13000; reload speed ×9000 | 6 Folios; 600 `dmg.pierce` kills; V2 as Gunslinger | 520 |
| `echo.kestrel_unit` | SO, Artificer, construct | gadgets deploy with +1 charge; gadget recharge ×8500 | 10 Folios; 80 crafts; V4 as Artificer | 910 |
| `echo.nix_seventeen` | CP, Netrunner, construct | lockpick and hack time ×6000; Max HP ×9500 | 6 Folios; 300 Edge kills; V2 as Netrunner | 470 |
| `echo.ronin_valdez` | CP, Duelist, human | blade crit damage +2500 Bp; crit chance −300 Bp | 10 Folios; 1,000 `dmg.slash` kills; V4 as Duelist | 920 |
| `echo.ma_rust` | PA, Grenadier, mutant | explosion-broken blocks yield ×2 materials; own explosive damage taken ×12500 | 6 Folios; 5,000 blocks; V2 as Grenadier | 470 |
| `echo.tallow` | PA, Alchemist, human | food satiation +2500 Bp; potion durations −1500 Bp | 10 Folios; 80 brews; V4 as Alchemist | 910 |
| `echo.josephine_crowe` | WW, Gunslinger, undead | crit chance +500 Bp at HP ≤ 30%; Max HP ×9500 | 6 Folios; 2 hunters; V2 as Gunslinger | 490 |
| `echo.preacher_amos` | WW, Warden, human | restoration heals deal 2000 Bp of the heal as `dmg.radiant` to undead and fiends within 4,000 mm; restoration aether cost ×11500 | 10 Folios; 6 Death's Door saves; V4 as Warden | 910 |
| `echo.dex_halloran` | HN, Duelist, human | unarmed stun buildup ×13000; blade damage −1000 Bp | 6 Folios; 400 `dmg.blunt` kills; V2 as Duelist | 470 |
| `echo.vera_lux` | HN, Alchemist, human | thrown poison flasks add +1 `st.poisoned` stack on a direct hit; buy prices ×10500 | 10 Folios; 960 `dmg.toxin` kills; V4 as Alchemist | 910 |
| `echo.lan_ninth_peak` | WX, Monk, human | dodge distance 4,000 mm; dodge cost 26 stamina | 6 Folios; 12 notes; V2 as Monk | 514 |
| `echo.old_master_huo` | WX, Warden, human | poise +20; move speed multiplier −500 Bp | 10 Folios; 20 notes; V4 as Warden | 910 |
| `echo.ada_brassvane` | SP, Artificer, human | crafting time ×7500; crafted durability ×9000 | 6 Folios; 40 crafts; V2 as Artificer | 490 |
| `echo.colonel_wexley` | SP, Grenadier, human | launcher projectile range ×12500; launcher reload ×8500 | 10 Folios; 960 `dmg.fire` kills; V4 as Grenadier | 910 |
| `echo.briar_thornwood` | FT, Beastbinder, fae | minion Max HP +1500 Bp (summons and tamed beasts); Conjuration aether cost ×11500 | 6 Folios; 6 Rewrites; V2 as Beastbinder | 490 |
| `echo.wolf_in_wool` | FT, Shade, beastkin | disguise suspicion rate ×7000; Max HP ×9000 | 10 Folios; 480 sneak kills; V4 as Shade | 910 |
| `echo.kallias_twice_drowned` | MA, Vanguard, human | swim speed ×15000 and no drowning damage for 1800 ticks submerged; `dmg.fire` taken +1000 Bp | 6 Folios; 3 Death's Door saves; V2 as Vanguard | 490 |
| `echo.sibyl_of_ash` | MA, Oracle, human | exit doors also preview the next Folio's Rubric and notes; Max aether −10 | 11 Folios; 1 secret exit; V3 as Oracle | 925 |

Tier 1 is the first Echo of each genre (12 trials, 470 to 520 ch); Tier 2 is the second (12 trials, 905 to 925 ch). Every one of the 13 Callings has 1 to 3 Echoes (Alchemist 3; Netrunner, Beastbinder and Monk 1; every other Calling 2). All 24 trials total 16,839 ch (168.4 hours).

**Doc 15 slot binding.** Doc 15 §3.13 uses placeholders `echo.slot_<code>_a` (Illuminated, the rarer Inkwell tier, about 19.6 hours per featured pull) and `echo.slot_<code>_b` (Rubricated, about 13.9 hours). Slot A binds each genre's Tier 2 Echo and slot B its Tier 1 Echo, so both routes stay ordered the same way: the rarer Inkwell tier has the longer trial, and each trial is shorter than the matching Inkwell reference.

| Doc 15 code | Slot A (Illuminated, Tier 2) | Slot B (Rubricated, Tier 1) |
|---|---|---|
| MF | `echo.wren_long_index` | `echo.aldric_unsworn` |
| GH | `echo.mother_ilse` | `echo.vesper_crane` |
| CH | `echo.drowned_clerk` | `echo.dr_halloway` |
| SO | `echo.kestrel_unit` | `echo.ostra_vey` |
| CY | `echo.ronin_valdez` | `echo.nix_seventeen` |
| PA | `echo.tallow` | `echo.ma_rust` |
| WW | `echo.preacher_amos` | `echo.josephine_crowe` |
| HN | `echo.vera_lux` | `echo.dex_halloran` |
| WU | `echo.old_master_huo` | `echo.lan_ninth_peak` |
| ST | `echo.colonel_wexley` | `echo.ada_brassvane` |
| FT | `echo.wolf_in_wool` | `echo.briar_thornwood` |
| MA | `echo.sibyl_of_ash` | `echo.kallias_twice_drowned` |

**Fold content binding** (doc 15 §3.8). For every Echo: Fold 1 `cosm.echo.<suffix>.palette`, Fold 2 trait variant B, Fold 3 kit B (the Calling's alt kit), Fold 4 `cosm.echo.<suffix>.outfit`, Fold 5 trait variant C, Fold 6 `cosm.echo.<suffix>.frame`, `cosm.echo.<suffix>.title` and `cosm.echo.<suffix>.trail`. The `cosm.` IDs are doc 15's catalog entries; this chapter fixes only their names.

Echo record:

```json
{
  "id": "echo.aldric_unsworn",
  "origin": "genre.mythic_fantasy",
  "calling": "calling.vanguard",
  "kin": "kin.human",
  "trait": "trait.echo.aldric_unsworn",
  "doc15_slot": "echo.slot_mf_b",
  "tier": 1,
  "est_ch": 470,
  "steps": [
    { "kind": "clear_genre", "genre": "genre.mythic_fantasy", "count": 6 },
    { "kind": "editor_as", "calling": "calling.vanguard", "volume": 2, "count": 1 },
    { "kind": "kills_type", "dmg": "dmg.blunt", "count": 400 }
  ]
}
```

---

## 7. Errata Grades

### 7.1 Grade table

Grades are cumulative: Grade `G` applies every modifier of Grades 1..=G. `grade_bp` multiplies score (§4.2).

| ID | Name | Modifier added at this Grade | `grade_bp` |
|---|---|---|---|
| `grade.g00` | Fair Copy | none | 10000 |
| `grade.g01` | Smudged Ink | `dmg_bp` = 10500 | 10550 |
| `grade.g02` | Thicker Vellum | `hp_bp` = 10500 | 11200 |
| `grade.g03` | Lean Purse | `lucre_bp` = 9000 | 11950 |
| `grade.g04` | Elite Ink | `elite_promote_add_bp` = 500 | 12800 |
| `grade.g05` | Dear Pages | `shop_price_bp` = 11500 | 13750 |
| `grade.g06` | Cold Hearth | `rest_heal_bp` = 7500 | 14800 |
| `grade.g07` | Champion's Page | `champion_extra` = 1 | 15950 |
| `grade.g08` | Heavy Bindings | `boss_hp_bp` = 11000 | 17200 |
| `grade.g09` | Smudged Ink II | `dmg_bp` = 11000 | 18550 |
| `grade.g10` | Spare Kit | `kit_consumables` = false | 20000 |
| `grade.g11` | Pack Hunters | `hunter_hp_bp` = 12500 | 21550 |
| `grade.g12` | Scarce Keys | `reliq_key_delta` = −1 | 23200 |
| `grade.g13` | Thicker Vellum II | `hp_bp` = 11000 | 24950 |
| `grade.g14` | Editors' Fury | `editor_dmg_bp` = 11500 | 26800 |
| `grade.g15` | Ambush Pages | `ambush_extra` = 1 | 28750 |
| `grade.g16` | Smudged Ink III | `dmg_bp` = 11500 | 30800 |
| `grade.g17` | Lean Purse II | `lucre_bp` = 8000 | 32950 |
| `grade.g18` | Elite Ink II | `elite_promote_add_bp` = 1000 | 35200 |
| `grade.g19` | Thicker Vellum III | `hp_bp` = 12000 | 37550 |
| `grade.g20` | The Final Draft | `final_hp_bp` = 12500; `final_phase_extra` = true | 40000 |

A later Grade that sets a parameter already set replaces the value; it does not compound. Grade 6's `rest_heal_bp` scales the Interleaf rest, the Endpaper restore and nothing else: the Colophon's full heal is a canon service (Canon §17.1) and no Grade or mutator reduces it.

**Gear tier.** The run's Grade is the `errata_grade` of Canon §13.9: every item generated in the run gets `gear_tier = min(5, 1 + floor_div(grade, 5))`, which sets its perk options per column (1/1/2/2/3 for tiers 1 to 5, doc 11). Grades 0–4 give tier 1, 5–9 tier 2, 10–14 tier 3, 15–19 tier 4 and 20 tier 5. More perk options are more choice, not more power, so this is the one Grade effect that helps the player.

### 7.2 Grade parameters

```rust
pub struct GradeParams {            // defaults = Grade 0
    pub dmg_bp: i32,                // 10000; §7.3 step-13 chain
    pub hp_bp: i32,                 // 10000; §7.3 HP chain, every rank
    pub boss_hp_bp: i32,            // 10000; ranks boss and editor
    pub hunter_hp_bp: i32,          // 10000; rank hunter
    pub final_hp_bp: i32,           // 10000; the final Folio's Editor only
    pub editor_dmg_bp: i32,         // 10000; attacker rank editor
    pub lucre_bp: i32,              // 10000; doc 11 multiplies every Lucre amount it generates
    pub shop_price_bp: i32,         // 10000; doc 11 buy price, after Canon §6.4 row 37
    pub rest_heal_bp: i32,          // 10000; §3.9, §3.10
    pub elite_promote_add_bp: i32,  // 0; doc 13 adds to each standard spawn's elite promotion chance
    pub champion_extra: u8,         // 0; doc 13 extra champion per Normal Folio with pos >= 3
    pub ambush_extra: u8,           // 0; doc 12 extra ambush events per Normal Folio
    pub reliq_key_delta: i8,        // 0; doc 11 in-run Reliquary keys per Volume
    pub kit_consumables: bool,      // true; §2.4.5
    pub final_phase_extra: bool,    // false; doc 13 adds its extra phase to the final Editor
}
```

At Grade 20 the merged values are: `dmg_bp 11500`, `hp_bp 12000`, `boss_hp_bp 11000`, `hunter_hp_bp 12500`, `final_hp_bp 12500`, `editor_dmg_bp 11500`, `lucre_bp 8000`, `shop_price_bp 11500`, `rest_heal_bp 7500`, `elite_promote_add_bp 1000`, `champion_extra 1`, `ambush_extra 1`, `reliq_key_delta −1`, `kit_consumables false`, `final_phase_extra true`. Mutators (§8.7) merge after the Grade: a Bp parameter is chained with `mul_bp` (Grade value first, then mutators in content-ID order), counts are added, and `kit_consumables` is ANDed. Mutator HP and damage factors are not merged into `hp_bp` or `dmg_bp`; they enter the §7.3 chains directly.

### 7.3 Difficulty multipliers

Canon §10.5 step 13 multiplies `D` by one factor `m13`, computed as a Bp chain and then applied with a single `mul_bp`:

```
m13 = mul_bp(diff_dmg_bp(attacker.rank, f), boon_def(target))   if attacker is an NPC and target is an Erratum
m13 = boon_off(attacker, target.rank)                            if attacker is an Erratum (not a minion) and target is an NPC
m13 = 10000                                                      for every other hit
boon_def(e)      = 9000 if e's active boon is run.boon.thick_hide, else 10000
boon_off(e, rank) = 11000 if rank == editor and e's boon is run.boon.editors_bane;
                    13000 if rank == hunter and e's boon is run.boon.hunters_bane; else 10000
```
 Enemy Max HP gains a final factor: `template_hp × H(d) × rank HP × party-size HP × diff_hp_bp`, each a `mul_bp` in that order (Canon gaps, item 3). Both chains start at 10000, apply `mul_bp` left to right, and clamp to `DIFF_CAP_BP` last.

```
diff_hp_bp(rank, f)  = min(DIFF_CAP_BP, chain[ gp.hp_bp,
                         gp.boss_hp_bp     if rank ∈ {boss, editor},
                         gp.hunter_hp_bp   if rank == hunter,
                         gp.final_hp_bp    if this is the final Folio's Editor,
                         marg_hp_bp(f), deep_bp(f),                        // §9.2
                         mutator and Marginal-mutation HP factors in content-ID order ])
diff_dmg_bp(rank, f) = min(DIFF_CAP_BP, chain[ gp.dmg_bp,
                         gp.editor_dmg_bp  if rank == editor,
                         marg_dmg_bp(f), deep_bp(f),
                         mutator and Marginal-mutation damage factors in content-ID order ])
```

Every mutator and Marginal-mutation factor carries a scope (§8.7, §9.3): `all` ranks, `editor` only or `hunter` only; a factor joins a chain only when the rank is in its scope. `run.mut.hungry_hunters` and `run.marg.hunters_pack` are `hunter`-scoped, `run.marg.editors_wrath` is `editor`-scoped, and every other factor is `all`.

Example (HP): `boss.editor_v2` at Folio 10, Grade 20, solo, template 100 HP. `diff_hp_bp = chain[12000, 11000] = 13200` (Folio 10 is not final in Standard). HP: 100,000 → `× H(10) = 34423` → 344,230 → `× 400000` → 13,769,200 → `× 10000` → 13,769,200 → `× 13200` → **18,175,344 mHP**.

Example (damage): an NPC hit with `D` = 23,000 mHP after step 12, Grade 16 (`dmg_bp 11500`), party of 2: step 13 → 26,450; step 14 `× 11000` → **29,095 mHP**. Against a target whose boon is `run.boon.thick_hide`: `m13 = mul_bp(11500, 9000) = 10350`, step 13 → 23,805, step 14 → **26,185 mHP**.

### 7.4 Unlocking Grades

Grades open with `margin.milestone.first_victory`, which unlocks Grade 1. A victory at Grade `G` unlocks `G + 1` (to at most 20). For `G <= 9`, a Standard, Abridged or Novella victory counts; for `G >= 10`, only a Standard or Abridged victory counts, because only those reach the Ur-Text. Seeded and Custom victories never unlock Grades. Runs with `access.scribes_mercy` active (doc 17), any easing mutator, or a simulation-affecting mod (Apocrypha) never unlock Grades. Daily and Weekly runs use fixed Grades and never unlock Grades. `max_grade(account)` is the highest unlocked Grade, 0 before `first_victory`. In co-op only the host's `max_grade` limits the lobby, and each member's own unlock advances only if that member was present at the victory.

---

## 8. Modes

### 8.1 Mode table

| ID | Structure, final Folio | Seed | Content set | Erratum | Grade | Mutators | Ranked | `mode_quire_bp` | `folio_scale_bp` | Mid-run save (Canon §2.5) | Opens at |
|---|---|---|---|---|---|---|---|---|---|---|---|
| `mode.standard` | Standard, 25 (Endless optional) | host CSPRNG | account | free | 0..=max | none | yes | 10000 | 10000 | solo: any tick; co-op: Folio entry | `prologue_done` |
| `mode.abridged` | Abridged, 15 (`d = 25`) | host CSPRNG | account | free | 0..=max | none | yes | 7500 | 9000 | solo: any tick; co-op: Folio entry | `prologue_done` |
| `mode.novella` | Novella, 15 | host CSPRNG | account | free | 0..=max | none | yes | 10000 | 8000 | solo: any tick; co-op: Folio entry | `first_editor_v1` |
| `mode.daily` | Daily, 5 | Canon §3.6 daily | `unlock.daily_baseline` | drawn | 2 | 1 drawn | first attempt per UTC day | 10000 ranked, 5000 practice | 8000 | as Standard; ranked rules §12.3 | `first_run_end` |
| `mode.weekly_gauntlet` | Novella, 15 | Canon §3.6 weekly | `unlock.daily_baseline` | free from baseline | 6 | 3 drawn | yes | 10000 | 8000 | as Standard; ranked rules §12.3 | `first_editor_v2` |
| `mode.seeded` | Standard, Novella or Abridged | typed | baseline (toggle: account) | free | 0..=max | none | no | 5000 | by structure | as Standard | `first_editor_v1` |
| `mode.endless` | Standard continued, none (≤ 9999) | inherited | inherited | inherited | inherited | Marginal mutations only (§9.3) | yes | 10000 | 10000 | as Standard | `first_standard_victory` |
| `mode.custom` | Standard (25, or 10 with `run.mut.short_book`), Novella or Abridged | CSPRNG or typed | account | free | 0..=max | any owned | no | 5000, or 0 with an easing mutator | by structure | as Standard | `first_victory` |
| `mode.prologue` | 1 Folio | `PROLOGUE_SEED` | `unlock.set.starter` | 4 starters | 0 | none | no | none (no account rewards) | fixed layout | checkpoint at each room entry (§11) | first launch |

"By structure" means 10000 for Standard, 9000 for Abridged and 8000 for Novella. `folio_scale_bp` is passed to doc 12, which scales each Folio's room count to `max(4, mul_bp(rooms_base, folio_scale_bp))`. Co-op (1 to 4 players) overlays every mode except the Prologue (§8.8).

**`mode_quire_bp` choices.** Abridged's 7500 brings its Quires per hour (Example E, 141 per hour) in line with Standard's (Example B, 143 per hour); at 10000 it would pay 212 per 67.5 minutes. Seeded and Custom pay 5000 because the player picks the seed or stacks hard mutators on the short structure: at 10000 a Grade-20 `run.mut.short_book` victory with every hard mutator reaches the 400-Quire cap in about 50 minutes, eight times the reference rate. `MUTATOR_BP_CAP` (§4.2) bounds the same exploit on the leaderboard side.

**Co-op modes** (Canon §17.5, owned here). `mode.ff_off`, `mode.ff_full` and `mode.loot_shared` are lobby overlays, never selected alone, set in `RunSetup` with 2 or more slots and stored in `RunState.coop`. `mode.ff_off` and `mode.ff_full` exclude each other. A run with `mode.ff_off` or `mode.loot_shared` is unranked and otherwise pays and counts normally; `mode.ff_full` keeps ranked eligibility, since it only makes the run harder.

**Apocrypha.** A run with any simulation-affecting mod (doc 02 §13, doc 15 §2.2) is unranked in every mode, earns no account rewards (doc 15), unlocks no Grade, advances no Recollection, achievement or milestone except `first_run_end`, and is marked in run history.

### 8.2 Standard

The full 25-Folio run (100 to 150 minutes, Canon §17.1), Volume I to V Editors, then the Folio 25 Colophon and VictoryChoice (§3.2). Single-player suspend at any tick boundary; co-op saves at Folio entry (§12).

### 8.2a Abridged

`mode.abridged` (Canon §17.1, owner doc 14) is the shorter full-story run: 5 Volumes of 3 Folios, two regular Folios plus the boss, 15 Folios in all. `d(f) = d_eff` (§3.1) feeds `P`, `T`, `H`, `S`, `ilvl` and `enemy_level`; per-Folio streams, `EntityId` and uids use the Folio index `f`. Doc 13 multiplies kill XP and doc 11 multiplies loot quantity by 16667 Bp per Folio (Canon §17.1). `EMENDATIONS_PER_VOLUME` stays 3. The final Folio is the Ur-Text at `d = 25` with `boss.ur_editor`; its Endpaper ends the run `run.end.victory` with `victory_bp = 17500`, and Abridged never continues into Marginalia. Target length is 60 to 75 minutes: 15 Folios at about 270 s with `folio_scale_bp = 9000`. Abridged has its own board (§9.4).

### 8.3 Novella

Folios 1 to 15 at `folio_scale_bp = 8000`, targeting 240 s per Folio and 60 minutes per run, with `d(f) = f`, so Novella plays Volumes I to III of the Standard run at Standard depth. `boss.editor_v3` at Folio 15 is the final boss; its Endpaper ends the run `run.end.victory` with `victory_bp = 15000`. Novella has its own boards and never continues into Marginalia.

### 8.4 Daily

One Volume: Folios 1 to 5, `boss.editor_v1` final, Grade 2, `folio_scale_bp = 8000`. At run start the host draws from `rng.run` with `folio_index = 0`, exactly 9 draws in this order:

1. Callings for slots 0..=3 without replacement from the Canon §16 table order: `range(10)`, `range(9)`, `range(8)`, `range(7)` (4 draws, always, whatever the party size).
2. Origins for slots 0..=3 with replacement from the 12 core genres in Canon §5.2 order: `range(12)` ×4.
3. One mutator from the Gauntlet pool (§8.7, 12 entries sorted by ID): `range(12)`.

Example, 2026-10-11 (daily seed `0xc6f2e12ddca712ab`, `rng.run` stream seed `0x6c9f436c0c107704`): slots 0..=3 are Gunslinger/Cyberpunk, Artificer/Hardboiled Noir, Vanguard/Weird West, Alchemist/Gothic Horror; mutator `run.mut.dear_pages`.

**Ranked attempt.** Each account has one ranked Daily attempt per UTC date. A run is ranked when every party member's attempt for that date is unused at `Commit`; `Commit` consumes them all. Any further attempt that day is practice: unranked, `mode_quire_bp = 5000`. Missing a day removes nothing; there are no streaks.

### 8.5 Weekly Gauntlet

Novella structure (15 Folios, `boss.editor_v3` final, `victory_bp = 15000`), Grade 6, free Calling and Origin from `unlock.daily_baseline`. Three mutators come from `rng.run` with `folio_index = 0` without replacement over the 12-entry Gauntlet pool: `range(12)`, `range(11)`, `range(10)` (3 draws). Example, ISO week `2026-W41`: weekly seed `0x2e7e2bae906b8339`, stream seed `0x54f2e7d7bdbce9f5`, mutators `run.mut.blind_exits`, `run.mut.ambushes`, `run.mut.elite_ink`. Attempts are unlimited; the board keeps each party's best verified score of the week.

### 8.6 Seeded

The player types a seed (Canon §3.6 decoding) and picks Standard (25 Folios) or Novella (15) structure. Share string: seed display, then `-`, structure letter `S` or `N`, content letter `B` (baseline) or `A` (account), and two Grade digits, for example `028T-5CY4-TQKFF-SB07`. Seeded runs are unranked and pay `mode_quire_bp = 5000`.

### 8.7 Custom mutators

Custom runs pick a structure, a Grade and any owned mutators. They are unranked. An **easing** mutator sets `mode_quire_bp = 0`, disables achievements and Grade unlocks, and pauses Recollection counters for the run.

| ID | Effect | `score_bp` | Easing | Gauntlet pool |
|---|---|---|---|---|
| `run.mut.ambushes` | `ambush_extra += 2` | 11000 | no | yes |
| `run.mut.bare_kit` | kit consumables and kit armor removed | 12000 | no | yes |
| `run.mut.blind_exits` | doors hide genre, axioms and forecasts until committed | 10500 | no | yes |
| `run.mut.champion_parade` | `champion_extra += 2` | 12500 | no | yes |
| `run.mut.dear_pages` | `shop_price_bp` chain 15000 | 11000 | no | yes |
| `run.mut.elite_ink` | `elite_promote_add_bp += 2000` | 12000 | no | yes |
| `run.mut.hungry_hunters` | hunter HP and damage factor 15000 | 11000 | no | yes |
| `run.mut.iron_ink` | damage chain factor 15000 | 13000 | no | yes |
| `run.mut.no_rest` | Interleaf Rest disabled; `rest_heal_bp` chain 5000 | 12000 | no | yes |
| `run.mut.pauper` | `lucre_bp` chain 5000 | 11500 | no | yes |
| `run.mut.thick_skins` | HP chain factor 15000 | 13000 | no | yes |
| `run.mut.wild_bleed` | doc 12 bleed-zone count ×2 | 11000 | no | yes |
| `run.mut.paper_foes` | HP chain factor 6000 | 5000 | yes | no |
| `run.mut.patron` | `lucre_bp` chain 20000 | 8000 | yes | no |
| `run.mut.short_book` | `final_folio = 10` (`boss.editor_v2` final) | 10000 | no | no |
| `run.mut.soft_ink` | damage chain factor 6000 | 5000 | yes | no |

Mutator record:

```json
{ "id": "run.mut.iron_ink", "score_bp": 13000, "easing": false, "gauntlet": true,
  "dmg_factor_bp": 15000, "hp_factor_bp": 10000 }
```

### 8.8 Co-op overlay

- **Lobby and authority.** 1 to 4 slots, host-authoritative (doc 16). Every mode except the Prologue.
- **Scaling.** `n` is fixed at each Folio entry (§3.3) and drives Canon §17.2 for spawns in that Folio. Loot is instanced per player (Canon §17.1). XP splits per Canon §8.
- **Exits and gates** use the unanimity-then-majority rule (§3.6). Ghosts follow §3.10.
- **Joining** happens only at a Folio entry (§2.5).
- **Leaving.** A disconnected player's slot becomes `Dormant` at once: the Erratum leaves the world with its inventory and is not counted in `n` from the next Folio entry. If the player is not back within `DORMANT_GRACE_S` (300 s wall clock), or leaves on purpose, the slot becomes `Departed`. Both host and player compute the departed payout from the score accumulators of the most recent Folio-entry save, with `victory_bp = 10000` and the slot's presence, store it as `quires_paid`, and pay it once. A Departed or Dormant player who rejoins at a later Folio entry becomes Active again, and the final payout subtracts `quires_paid` (§4.6).
- **Ranked co-op.** Daily needs every member's unused attempt (§8.4). Boards are per party size (§9.4).

---

## 9. Endless Marginalia and leaderboards

### 9.1 Structure

Margins doors appear in a Standard run's VictoryChoice only when the host's account held `margin.milestone.first_standard_victory` before the run started; otherwise the only door is "Close the Book". Endless begins when a Margins door of the Folio 25 VictoryChoice commits: `mode` becomes `mode.endless`, `final_folio = u32::MAX`, and `score_at_victory` is stored (§4.2). Marginalia Folios 26 to `MARGINALIA_MAX_FOLIO` use §3.1 positions. Every Keeper is `rank.champion`. Boss Folios (`f % 5 == 0`) rotate Editors by `r = (floor_div(f, 5) - 6) % 5`: `r = 0..=3` gives `boss.editor_v1..=v4` and `r = 4` gives `boss.ur_editor` (Folio 30 → v1, 50 → Ur-Editor, 55 → v1). Each Marginalia Endpaper appends a "Close the Book" door after its exit doors (`run.end.margin_closed`). The lattice keeps "no genre twice in the same Volume"; the run-wide cap of 3 appearances ends at Folio 25 (Canon §17.1).

### 9.2 Scaling

Power tables keep growing to `d_pow = 300` (Canon §13.8). This chapter adds:

```
marg_hp_bp(f)  = if f >= 26 { 10000 + 100 × (f - 25) } else { 10000 }
marg_dmg_bp(f) = if f >= 26 { 10000 +  50 × (f - 25) } else { 10000 }
deep_bp(f)     = if f > 300 { 10000 + 500 × (f - 300) } else { 10000 }   // in both chains
```

| f | `marg_hp_bp` | `marg_dmg_bp` | `deep_bp` | Grade 0 `diff_hp_bp` | Grade 0 `diff_dmg_bp` |
|---|---|---|---|---|---|
| 26 | 10100 | 10050 | 10000 | 10100 | 10050 |
| 50 | 12500 | 11250 | 10000 | 12500 | 11250 |
| 100 | 17500 | 13750 | 10000 | 17500 | 13750 |
| 300 | 37500 | 23750 | 10000 | 37500 | 23750 |
| 301 | 37600 | 23800 | 10500 | 39480 | 24990 |
| 400 | 47500 | 28750 | 60000 | 285000 | 172500 |
| 500 | 57500 | 33750 | 110000 | 632500 | 371250 |

At Grade 20 with `run.marg.thick_vellum` at rank 5 an Editor's HP chain reaches `DIFF_CAP_BP` at Folio 467. **Overflow bound:** doc 13 templates have `template_hp ≤ 400` HP. Then the largest enemy HP is `400,000 × H(300) × 400000 × 37000 × DIFF_CAP_BP` (each `mul_bp`) = 4,021,273,828,026,032,000 mHP, below `i64::MAX`.

### 9.3 Marginal mutations

At the first Folio of each Marginalia Volume (`f = 26, 31, 36, ...`), before note draws (§3.3), one draw from that Folio's `rng.run` stream: while any mutation is inactive, `range(inactive_count)` over the inactive list sorted by ID activates it at rank 1. Once all 12 are active, `range(12)` over the sorted list picks one to raise by 1; if it is at rank 5, the next ID in ascending order (wrapping) below rank 5 is raised instead. When all are at rank 5, no draw happens. Rank `r` effects:

| ID | Effect at rank r |
|---|---|
| `run.marg.ambush_pages` | `ambush_extra += r` |
| `run.marg.bleeding_genres` | doc 12 bleed-zone count `+ r` |
| `run.marg.brittle_kit` | durability loss × `(10000 + 2500 × r)` (doc 07) |
| `run.marg.champion_tide` | `champion_extra += r` |
| `run.marg.cramped_hearth` | `rest_heal_bp` chain `max(2500, 10000 - 1500 × r)` |
| `run.marg.dear_margins` | `shop_price_bp` chain `10000 + 1000 × r` |
| `run.marg.editors_wrath` | editor damage factor `10000 + 1000 × r` |
| `run.marg.elite_margins` | `elite_promote_add_bp += 500 × r` |
| `run.marg.hunters_pack` | hunter HP factor `10000 + 2000 × r` |
| `run.marg.lean_purse` | `lucre_bp` chain `max(5000, 10000 - 1000 × r)` |
| `run.marg.red_ink` | damage factor `10000 + 500 × r` |
| `run.marg.thick_vellum` | HP factor `10000 + 1000 × r` |

### 9.4 Leaderboards

| Board | Metric (descending unless noted) | Tie-breaks in order | Partitions |
|---|---|---|---|
| `score.board.standard` | score | fewer `end_tick`; earlier submission | party size 1..=4; all-time and per 12-week season |
| `score.board.standard_speed` | `end_tick` ascending, victories only | higher score; earlier submission | party size; Grade 0..=20 |
| `score.board.novella` | score | fewer `end_tick`; earlier submission | party size; all-time and season |
| `score.board.daily` | score | deeper Folio; fewer `end_tick` | party size; UTC date |
| `score.board.weekly_gauntlet` | best score | fewer `end_tick` | party size; ISO week |
| `score.board.endless` | deepest Folio cleared | higher score; fewer `end_tick` | party size; all-time and season |

Seasons partition display only; every entry stays in the all-time board forever. Eligible entries: ranked mode, no easing mutator, no `access.scribes_mercy`, `restores <= RANKED_RESTORE_MAX` (§12.3), and a server-verified replay.

```rust
fn verify(sub: &Submission) -> Result<(), Reject> {
    require(sub.sim_version == SERVER_SIM_VERSION && content_hash_known(sub.content_hash))?;
    require(token_valid(sub.ranked_token, sub.mode, sub.party_accounts))?;   // issued at Commit
    let mut sim = HeadlessHost::new(sub.run_seed, &sub.config);             // doc 16 headless binary
    for cmd in sub.replay_log.iter() { sim.apply(cmd); }                    // full log from tick 0
    require(sim.state_hash() == sub.final_state_hash)?;
    require(sim.run().end.unwrap().score == sub.score)?;
    Ok(())
}
```

```json
{
  "board": "score.board.daily", "period": "2026-10-11", "party_size": 2,
  "run_seed": "0xC6F2E12DDCA712AB", "mode": "mode.daily", "grade": 2,
  "mutators": ["run.mut.dear_pages"], "sim_version": 1,
  "content_hash": "0x9F00000000000000000000000000000000000000000000000000000000000001",
  "score": 21480, "deepest_folio": 5, "end_tick": 99000, "restores": 0,
  "final_state_hash": "0x1A00000000000000000000000000000000000000000000000000000000000002",
  "ranked_token": "0x7700000000000000000000000000000000000000000000000000000000000003"
}
```

---

## 10. Achievements

Achievements are account-level, evaluated by the meta layer from simulation events and `RunSummary`, and mirrored to platform achievements. "Present" means the player's slot was Active or Ghost. Every achievement is disabled in runs with an easing mutator; those marked † are also disabled while `access.scribes_mercy` is active. Cumulative counters never reset.

| ID | Condition | Quires | Ink |
|---|---|---|---|
| `ach.first_page` | complete the Prologue | 100 | 160 |
| `ach.turned_page` | clear Folio 1 | 50 | 0 |
| `ach.volume_1` | present at `boss.editor_v1`'s death | 150 | 160 |
| `ach.volume_2` | present at `boss.editor_v2`'s death | 200 | 160 |
| `ach.volume_3` | present at `boss.editor_v3`'s death | 250 | 160 |
| `ach.volume_4` | present at `boss.editor_v4`'s death | 300 | 160 |
| `ach.ur_text` † | Standard victory | 500 | 320 |
| `ach.novella` † | Novella victory | 250 | 160 |
| `ach.marginalia_30` † | clear Folio 30 | 200 | 0 |
| `ach.marginalia_50` † | clear Folio 50 | 300 | 160 |
| `ach.marginalia_100` † | clear Folio 100 | 500 | 320 |
| `ach.grade_5` † | victory at Grade ≥ 5 | 300 | 160 |
| `ach.grade_10` † | victory at Grade ≥ 10 | 400 | 160 |
| `ach.grade_15` † | victory at Grade ≥ 15 | 500 | 160 |
| `ach.grade_20` † | victory at Grade 20 | 800 | 320 |
| `ach.callings_v1` | `boss.editor_v1` killed as each of the 10 Callings (cumulative) | 300 | 160 |
| `ach.callings_win` † | Standard or Novella victory as each Calling | 600 | 320 |
| `ach.origins_v1` | `boss.editor_v1` killed with each of the 12 core Origins | 300 | 160 |
| `ach.atlas` | clear a Folio of each of the 12 core genres (cumulative) | 200 | 0 |
| `ach.oneiric` | enter `genre.oneiric` | 150 | 0 |
| `ach.redacted` | enter `genre.redacted` | 150 | 0 |
| `ach.unwritten` | enter `genre.unwritten` outside the Prologue | 150 | 0 |
| `ach.secret_trio` | all three secret genres entered | 300 | 160 |
| `ach.anachronist` | 100 kills of native targets by a base component with `E >= 3` | 100 | 0 |
| `ach.shock_of_new` | trigger Shock of the New 50 times | 100 | 0 |
| `ach.coherent_volume` | clear Folios 1 to 5 of one run without ever gaining `st.contradicted` | 150 | 0 |
| `ach.rewritten` | 50 Rewrites committed | 100 | 0 |
| `ach.appraiser` | 500 item instances identified | 100 | 0 |
| `ach.connoisseur` | every potion type of the run's content set known in one run | 150 | 0 |
| `ach.master` | any skill reaches 100 in one run | 200 | 0 |
| `ach.triad` | three skills at ≥ 50 at once | 100 | 0 |
| `ach.level_30` | character level 30 | 150 | 0 |
| `ach.palimpsestic` | equip a Palimpsestic item | 100 | 0 |
| `ach.exotic_pair` | equip an Exotic weapon and an Exotic armor piece at once | 100 | 0 |
| `ach.crafter` | 100 `ev.item_crafted` | 100 | 0 |
| `ach.brewer` | 100 `ev.potion_brewed` | 100 | 0 |
| `ach.builder` | place 1,000 blocks | 50 | 0 |
| `ach.excavator` | break 5,000 blocks | 50 | 0 |
| `ach.ghost_step` | complete `run.note.silent` | 100 | 0 |
| `ach.deaths_door` | 25 Death's Door saves | 100 | 0 |
| `ach.erasure_10` | exit a Folio while holding 10 Erasure stacks | 100 | 0 |
| `ach.hunter` | kill an Erasure hunter | 150 | 0 |
| `ach.hunter_10` | kill 10 Erasure hunters | 200 | 160 |
| `ach.swift_volume` † | `boss.editor_v1` dies before run tick 54,000 (15 min) | 150 | 0 |
| `ach.speed_win` † | Standard victory with `end_tick < 324000` (90 min) | 400 | 160 |
| `ach.party_of_four` | clear Folio 5 in a party of 4 | 100 | 0 |
| `ach.reviver` | 50 `ev.player_revived` as the reviver | 100 | 0 |
| `ach.full_house` † | Standard victory in a party of 4 with `S_deaths = 0` | 300 | 160 |
| `ach.daily_10` | 10 ranked Daily runs ended | 150 | 160 |
| `ach.gauntlet` † | Weekly Gauntlet victory | 250 | 160 |
| `ach.recollection` | complete one Recollection trial | 100 | 0 |
| `ach.recollection_all` | complete all 24 Recollection trials | 1000 | 320 |
| `ach.bindery_half` | own 37 of the 73 Bindery nodes | 0 | 160 |
| `ach.bindery_all` | own all 73 Bindery nodes | 0 | 320 |
| `ach.notes_100` | complete 100 marginal notes | 150 | 0 |
| `ach.cartographer` | complete `run.note.cartographer` in 5 Folios of one run | 100 | 0 |

56 achievements pay 12,050 Quires and 4,960 Ink in total.

---

## 11. Onboarding: the Prologue Folio

On first launch the game offers **Begin** or **I have read this before**. Skipping sets `prologue_done` and grants nothing. The Prologue is replayable at `margin.first_page`; replays grant nothing.

The player picks one of the 4 starter Callings and one of the 4 starter Origins. The run is `mode.prologue`: single player, `run_seed = PROLOGUE_SEED = fnv1a64("PALIMPSEST/PROLOGUE") = 0x39d13f78bea6b81d`, `folio_index = 1`, Folio genre `genre.unwritten`, Grade 0, a fixed hand-built layout of seven rooms (`room.prologue_l1` to `room.prologue_l7`, doc 12), the chosen Calling's standard kit (Margin Translation uses the chosen Origin as `g1`, so no kit item is translated), and no score. Target length 300 to 480 s; natural Erasure (Canon §11.6) still applies after 420 s.

| Room | Local genre | Teaches | Leaves when |
|---|---|---|---|
| L1 Your Story | bleed zone of the chosen Origin | look, move, sprint, jump, interact; a native NPC vignette | the player reaches the far door |
| L2 The Red Pen | chosen Origin | attack, block, dodge against 3 `rank.minion` Red Ink scrawls (`fac.editors`) | all three die |
| L3 The Fall | Unwritten | the scripted Editor erases the room; a 12 m drop into water (no fall damage); mine 3 blocks with the pick, place 2 blocks to bridge a gap | the player reaches the ledge |
| L4 Blank Pages | Unwritten | a chest with 30 Lucre, one unknown potion (type `item.potion.healing`, appearance from the shuffle) and one known `item.text.appraisal`; quickbar; identification by use or appraisal | the player uses the reading stand |
| L5 A Borrowed Page | bleed zone of `genre.fairy_tale` | pick up a Cyberpunk `arch.ranged.sidearm` (ilvl 1); the HUD shows its Contradiction state (C = 4 for a Cyberpunk Origin per Canon §5.6; Disconnected at C = 6 for Mythic Fantasy and Weird West, C = 5 for Gothic Horror); a native wolf shows Anachronism Edge | the wolf dies |
| L6 Scriptorium | Unwritten | a free Rewrite of the sidearm into the Fairy Tale skin; Coherent state | a Rewrite is committed |
| L7 The Erasure | Unwritten | on entry the script directly applies 3 stacks of `st.erasure` (a `direct` status, Canon §11.1); `enemy.prologue_hunter` chases the player 60 m to the Colophon | the player enters the Colophon door |

Each room entry writes a checkpoint save (single player may save at any tick, Canon §2.5). When the Erratum dies, the current room reloads from its checkpoint; the Prologue never ends `run.end.erased`. Completion sets `prologue_done`, grants `ach.first_page` and loads the Margin. No Prologue item carries over.

---

## 12. Save and suspend rules

### 12.1 Files and write points

- `run.sav` holds the Canon §2.4 header, the Canon §2.5 simulation save (including `RunState`) and the replay log so far. One active run per account per device. Account data lives in a separate signed file and is never part of `run.sav`.
- **Single player** writes at every Folio entry, every `AUTOSAVE_PERIOD_TICKS` after it (non-ranked runs only), on Suspend at the exact current tick, and on application quit.
- **Co-op** writes only at Folio entry (Canon §2.5). Every client keeps a copy of the host's latest Folio-entry save for host migration (doc 16).

### 12.2 Suspend and resume

- Single-player Suspend works at any tick boundary. Resume continues at that tick; the final `state_hash` equals an uninterrupted run's (Canon acceptance test 32).
- Co-op Suspend (host only) ends the session. Resume loads the latest Folio-entry save, so the in-progress Folio restarts from its entry; the Suspend dialog states this. Absent players' slots resume `Dormant`.
- Resume requires equal `sim_version` and `content_hash`. On a mismatch the run is archived: it ends `run.end.abandoned` with the score from the save's accumulators, and the payout is granted.
- Content hot reload is refused while a run is active (Canon §2.4).

### 12.3 Ranked integrity

- Each write increments `save_seq: u32` in `RankedState`. While online, a ranked run reports `(ranked_token, save_seq, state_hash)` on each write. Loading a save whose `save_seq` is below the latest reported value demotes the run to unranked.
- Ranked runs have no periodic autosave. Crash recovery (no Suspend file) loads the latest Folio-entry save, truncates the replay log to that tick and increments `restores`. A run with `restores > RANKED_RESTORE_MAX` is demoted at submission. Orderly Suspend and resume does not count as a restore.

---

## Exports

All IDs below are in namespaces Canon §19 assigns to doc 14. All 289 pass the Canon §1.1 grammar, are at most 64 bytes, and have distinct `fnv1a32`.

| Namespace | IDs |
|---|---|
| `mode.` | `standard`, `novella`, `daily`, `weekly_gauntlet`, `seeded`, `endless`, `custom`, `prologue` |
| `grade.` | `g00` to `g20` (21) |
| `run.end.` | `victory`, `erased`, `abandoned`, `margin_closed`, `margin_exhausted`, `departed` |
| `run.rubric.` | `keeper`, `catchwords`, `seal` |
| `run.note.` | the 10 IDs of §3.5 |
| `run.mut.` | the 16 IDs of §8.7 |
| `run.marg.` | the 12 IDs of §9.3 |
| `run.kit.` | `universal`, `<calling>` and `<calling>_alt` for the 10 Callings (21) |
| `margin.` | the 15 facilities and 8 `margin.milestone.*` IDs of §5.1 |
| `unlock.` | the 73 Bindery nodes of §5.3, `unlock.set.starter`, and the content of `unlock.daily_baseline` (ID from Canon §13.6) |
| `echo.` | the 24 IDs of §6.3 |
| `score.` | 8 component IDs (§4.2) and 6 `score.board.*` IDs (§9.4) |
| `ach.` | the 56 IDs of §10 |

Data constants exported: §1.1 table, `grade_bp(G)`, `PROLOGUE_SEED = 0x39d13f78bea6b81d`.

## Interface assumptions

| Doc | This chapter assumes it provides |
|---|---|
| 04 | Erratum creation and Echo presets; `trait.echo_<suffix>` for all 24 Echoes with the §6.3 effects; level-up allocation for joiners; Ghost needs frozen |
| 05 | `arch.melee.{hafted_mace, hafted_maul, blade_long, blade_short, fist_wraps, polearm_spear}`, `arch.ranged.{sidearm, longarm, launcher, flamer, wand, staff, sling, hand_crossbow}`, `arch.offhand.{shield_medium, focus}`, `arch.armor.{light,medium,heavy}_{head,body,hands,legs,feet}`, `ammo.{light, heavy, explosive, fuel, stone}` |
| 06 | `glyph.{bolt, flame, ward, frost, chain, mend, cleanse, sight, summon_wisp, bind}` |
| 07 | `gadget.{turret_light, drone_scout}`, `mat.{iron, scrap}`, durability-loss hook for `run.marg.brittle_kit` |
| 08 | `apparatus.field_kit`, `ing.{common_herb, common_mineral}` |
| 09 | `aware.alerted` transitions for `run.note.silent` |
| 11 | `item.food.ration`; `item.tool.{bandage, torch, pick, lockpicks, toolkit}`; `item.potion.{healing, haste, focus, fortify, aether, antidote, invisibility, clarity}`; `item.text.{mending, blink, mapping, demolition, appraisal, recharge, sanctify}`; `item.throwable.{javelin, knife, smoke, frag, fire_flask, acid_flask, poison_flask}`; `item.book.primer_{evocation, abjuration, divination}`; `item.container.flask`; `item.trinket.lattice_lens`; `item.quest.catchword`; base values that keep every kit in 360..=440 Lucre; `loot.volume_reward`; `loot.pool.<suffix>` tags at base power budget; `lucre_bp`, `shop_price_bp`, `reliq_key_delta` hooks |
| 12 | `room.{incipit, colophon, interleaf}`, `room.prologue_l1` to `l7`; sanctuary flag; Rubric draw with weights `[5000, 3000, 2000]`; `folio_scale_bp`; `ambush_extra`; bleed-zone count hook; `rooms_total`; Illumination vault, rescue event and Scriptorium flags; lattice draw on demand; secret exits |
| 13 | `boss.editor_v1..=v4`, `boss.ur_editor`, `enemy.prologue_hunter`; `keeper` flag; Seal waves; `elite_promote_add_bp`; `champion_extra`; `telegraphed` attack flag; `final_phase_extra`; every `template_hp ≤ 400` HP |
| 15 | Ink and Codex XP from `RunSummary` (R3 proposes 400 Ink per hour of play); account Reliquary grants; `cosm.binding.<b>`; `cosm.echo_<suffix>_recollected`; Inkwell featured-Echo reference of 19.6 hours; Codex tiers that pay Quires |
| 16 | lobby, Dormant detection, host migration from Folio-entry save copies, ranked tokens, headless verification host |
| 17 | Margin screens, door previews and forecasts, Ghost spectator UI, `access.scribes_mercy` |

## Canon gaps

1. **Bindery currency.** The task statement asked for Ink costs. Canon §14 assigns `cur.quire` to the Margin unlock tree and `cur.ink` to the Inkwell. This chapter follows the canon: every cost is in Quires (total 27,100).
2. **Kit ownership.** Canon §16 says doc 04 owns kits. This chapter's scope requires exact starting items, so `run.kit.*` here defines the item loadout and doc 04 keeps skills and talents. Proposed canon text: "Doc 04 owns talent trees; doc 14 owns starting item kits."
3. **Enemy HP difficulty factor.** Canon §13.8 defines enemy HP without a difficulty term, yet says deep Marginalia scales only through doc 14 multipliers. This chapter appends `× diff_hp_bp` as the last `mul_bp`. It is 10000 at Grade 0 before Folio 26, so canon values are unchanged there.
4. **Step 13 scope.** Canon §10.5 does not say which hits step 13 applies to. Assumed: `diff_dmg_bp` for NPC-to-Erratum hits, 10000 otherwise.
5. **New stream `rng.run`.** Sequential, folio-scoped (`folio_index = 0` for run-start draws), owner doc 14. Draws: Daily setup (9), Weekly mutators (3), marginal notes (0 or 2 per Folio), Marginal mutations (0 or 1 per Marginalia Volume). Must be added to the Canon §3.2 table.
6. **New events.** Assumed added to Canon §18: `ev.rubric_completed`, `ev.note_completed`, `ev.exit_committed`, `ev.boss_engaged`, `ev.volume_completed`, `ev.erratum_restored`.
7. **Folio 1 genre.** Canon §17.1 covers Standard and the seeded modes only. Assumed: Novella and Custom without a seed follow Standard; Custom with a typed seed follows the seeded rule; a host with a secret-genre Origin follows the seeded rule, because secret genres are never lattice candidates.
8. **Marginalia boss Folios.** Canon `BOSS_FOLIOS` lists 5 to 25. This chapter adds every `f % 5 == 0` with `f >= 30`, and Canon §11.6's pause applies to them.
9. **Folio 24 exits.** The 3-candidate draw still happens (Canon §17.1) and its doors are replaced by `genre.ur_text`.
10. **Post-death co-op state.** Canon covers `st.downed` (doc 16) but not a killed Erratum while the party lives. This chapter defines Ghosts (§3.10).
11. **Integer safety past `d_pow = 300`.** Requires `DIFF_CAP_BP` and doc 13's `template_hp ≤ 400` HP (§9.2).

## Acceptance tests

1. **Score.** The four §4.2 examples produce scores 8,350, 88,012, 168,150 and 41,632 and Quires 83, 880, 1,681 and 416.
2. **Endless max rule.** Example B continued with Folios 26..=39 cleared at 330 s each, 3/4 rooms, 14 more notes, Editors at 30 and 35 with 8 telegraphed hits each, Σ `lucre_norm` 9,750, 3 hunters, erased in Folio 40: `S_progress = 54600`, `S_editor = 25200`, `base = 105985`, score `mul_bp(105985, 12500) = 132481` > 88,012, so the final score is 132,481 and Quires 1,324.
3. **Grade multiplier.** `grade_bp` is 10000, 13750, 20000 and 40000 at Grades 0, 5, 10 and 20.
4. **Erasure.** Stack 1 at clock 25,200, stack 10 at 41,400; hunters at 41,400, 46,800 and 52,200; healing ×9100 at 3 stacks. In a boss Folio entered at tick 1,000,000 whose Editor dies at 1,090,000, stack 1 arrives at 1,115,200.
5. **Rest.** Max HP 300,000 mHP, `rest_heal_bp` 7500, RES 40, 3 stacks: 102,375 mHP healed.
6. **Difficulty.** `boss.editor_v2` at Folio 10, Grade 20, solo, template 100 HP: `diff_hp_bp = 13200`, HP 18,175,344 mHP. An NPC hit of 23,000 mHP after step 12 at Grade 16 in a party of 2 ends step 14 at 29,095 mHP.
7. **Marginalia.** At f = 100 Grade-0 chains are 17500 and 13750; at f = 400 they are 285000 and 172500; an Editor at Grade 20 with `run.marg.thick_vellum` rank 5 first reaches 1,000,000 at f = 467.
8. **Overflow bound.** `mul_bp` chain `400000 → H(300) → 400000 → 37000 → 1000000` yields 4,021,273,828,026,032,000 without panic.
9. **Kits.** Each `run.kit.<calling>` plus the universal kit sums to 360..=440 Lucre of base value at ilvl 1; `start_run` kit generation calls no `Rng`; every kit uid has `folio_index = 0` in its top bits.
10. **Margin Translation.** Cyberpunk-origin Gunslinger, ATN 40, Folio 1 Fairy Tale: sidearm `C = 4`, generated with `origin_genre = genre.fairy_tale`. Weird West origin, Folio 1 Steampunk: `C = 0`, keeps `genre.weird_west`.
11. **Daily draw.** Seed `0xc6f2e12ddca712ab`: `rng.run` stream seed `0x6c9f436c0c107704`; slots get Gunslinger, Artificer, Vanguard, Alchemist; Cyberpunk, Hardboiled Noir, Weird West, Gothic Horror; mutator `run.mut.dear_pages`; exactly 9 draws for party sizes 1 and 4 alike.
12. **Weekly draw.** `2026-W41`: seed `0x2e7e2bae906b8339`, stream seed `0x54f2e7d7bdbce9f5`, mutators `run.mut.blind_exits`, `run.mut.ambushes`, `run.mut.elite_ink`.
13. **Prologue seed.** `fnv1a64("PALIMPSEST/PROLOGUE") = 0x39d13f78bea6b81d`.
14. **Exit commit.** Two voters, one idle: no commit before `open + 7200`; the active voter in door 1 from `open + 7000` onward commits at `open + 7319`.
15. **Notes draw count.** 7 eligible notes: 2 draws; 2 eligible: 0 draws, both offered.
16. **Bindery.** 73 nodes summing to 27,100 Quires; shelf sums 3,600, 7,100, 2,500, 5,600, 2,900, 3,900, 1,500. Purchasing `unlock.calling.warden` with neither Shade nor Artificer owned fails.
17. **Achievements.** 56 entries totalling 12,050 Quires and 4,960 Ink.
18. **Recollection.** Every Tier 1 trial has `est_ch` in 400..=900 and every Tier 2 trial in 900..=1600; `echo.aldric_unsworn` 470, `echo.sibyl_of_ash` 925; sum 16,839. A third activation while 2 trials are active is refused.
19. **IDs.** All 289 exported IDs pass the grammar with distinct `fnv1a32`.
20. **Presence.** Score 168,150, present 18 of 25: `presence_bp = 7200`, 1,210 Quires.
21. **Joiner.** Levels 9, 11, 12, joining at Folio 8: level 9, majors 34, minors 17, Lucre 91.
22. **Lucre normalization.** 120 Lucre in Folio 10 adds 55.
23. **Co-op suspend.** Host suspends at entry tick + 9,000; on resume the state hash equals the Folio-entry save's hash.
24. **Ranked integrity.** `restores = 3` demotes at submission; loading `save_seq` 4 after 6 was reported demotes.
25. **Daily practice.** A second Daily the same UTC date is unranked with `mode_quire_bp = 5000`.
26. **Grade unlock.** A Novella victory at Grade 9 unlocks Grade 10; a Novella victory at Grade 10 does not unlock Grade 11.
27. **Marginal mutations.** Folio 26 makes exactly 1 draw; with all 12 active at rank 5 Folio 91 makes 0.
28. **Editor rotation.** Folio 30 → `boss.editor_v1`, 50 → `boss.ur_editor`, 55 → `boss.editor_v1`.

## Invariants annex

```
Σ doc14: start ⊗ folio-flow ⊗ score ⊗ payout ⊗ bindery ⊗ echoes ⊗ grades ⊗ modes ⊗ marginalia ⊗ saves
W! score := f(sim accumulators) ∧ replay(score) == live(score)       N! float ∈ score ∪ payout
W! Bindery cost ∈ cur.quire                                           N! Bindery cost ∈ cur.ink ∪ fiat
W! ∀ unlock: effect ∈ {option, pool, cosmetic, convenience}           N! unlock → stat ∪ drop-rate ∪ rarity weight
W! kit generation draws = 0                                           W! kit item = common ∧ normal ∧ +0 ∧ identified
W! exit draw tick == ColophonOpen→ExitVote tick                        N! exit offers reroll
W! boss Folio: erasure clock starts at Editor death                   N! erasure stacks on Ghosts
W! co-op save only at Folio entry                                     N! co-op save mid-Folio
W! ranked ⇒ server replay verified ∧ restores ≤ 2                     N! easing mutator ∨ scribes_mercy on boards
W! diff_hp_bp, diff_dmg_bp ≤ DIFF_CAP_BP                              N! i64 overflow at any Folio ≤ 9999
W! Recollection est_ch ∈ tier bound                                   N! Recollection progress while inactive
W! Daily: one ranked attempt per account per UTC date                 N! streak penalty for missed days
W! departed payout paid once ∧ subtracted on rejoin                   N! double payout
W! account data ∉ palimpsest_sim state                                N! Margin draws from simulation streams
```
