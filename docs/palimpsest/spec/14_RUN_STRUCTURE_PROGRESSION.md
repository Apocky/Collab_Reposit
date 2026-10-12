# 14 — Run Structure and Meta Progression

As of 2026-10-11. Status: BINDING. Conforms to canon-1.11.

This chapter defines how a run starts in the Margin, how each Folio flows from arrival through its Colophon to the exit, how a run is scored and ends, what the account keeps, the Margin's unlock tree (the Bindery), Echo Recollection trials, Errata Grades 0 to 20, every mode (including Canon §17.1 Abridged, the Canon §17.5 co-op modes and Second Edition), Endless Marginalia scaling, leaderboards, achievements, contextual tips, save and suspend rules, the account Glyph Library, the Margin residents and their arcs, and the run data the closing manuscript is built from. Account currencies are `cur.quire` (Bindery) and `cur.ink` (Inkwell), exactly as Canon §14 assigns them. Doc 15 owns every account earn rate (Canon §19: account economy rules for `cur.` are doc 15's); this chapter supplies the inputs doc 15 consumes (`run_score`, achievement grades, Echo slot bindings) and prices the Bindery against doc 15's Quire income.

## Imports

| Source | Section | What this chapter consumes |
|---|---|---|
| Canon | §1.1 | ID grammar, `id32`, alias rule; every ID in Exports passes it |
| Canon | §1.3 | `floor_div`, `mul_bp`, left-to-right Bp chains, clamp-last rule |
| Canon | §1.5, §1.6 | display-unit rule for `Milli`; integer-only content; `u64` written as `"0x"` hex strings |
| Canon | §1.11 | `folio_index = 0` for run-start entities and items; ascending-ID iteration |
| Canon | §2.3 | "N s" = N × 60 ticks; status intervals |
| Canon | §2.4, §2.5 | session header (a lobby receives `run_seed` in the run header the host sends at Commit), replay log with applied intents, `content_hash` and `presentation_hash`, save contract; single player saves at any tick boundary except while an invader holds replay slot 4, co-op only at Folio entry |
| Canon | §3.1–§3.6 | `Rng`, sequential streams and labeled sub-streams (§3.2; this chapter owns `rng.run`, whose labeled sub-stream `chaos` draws the audience Chaos options, Canon gaps item 9), keyed draws (`rng.loot` for Ghost item loss and the Redaction), commit-only `rng.offer.p<slot>` with the §3.5 offer generator (boons, Volume rewards), seed display, daily and weekly seeds |
| Canon | §5.2, §5.4, §5.5, §5.8 | genre table, Origin tolerance, Contradiction `C` (kit translation, exit forecasts), local genre and bleed zones |
| Canon | §5.11 | Going Native at a Colophon; `st.unmoored` clears in a Colophon |
| Canon | §6.4 | rows 1, 4, 7 and 9 (pool maxima used by rests) |
| Canon | §8 | XP curve and the 45-unit anchor (level 29 on arrival at Folio 25), the co-op kill-XP split that includes downed members and ghosts and excludes Dormant and Departed slots (§3.10), mid-run joiner level |
| Canon | §10.5 step 13 | the difficulty multiplier slot this chapter fills |
| Canon | §10.8, §10.9 | lethal resolution, healing order (rest rooms) |
| Canon | §11.1, §11.6, §11.7 | status categories; Erasure (onset 720 s, at most 10 stacks, one elite-strength hunter per Folio and no repeat spawn, pause in boss Folios, Red Ink threshold); status multiplier order |
| Canon | §13.3–§13.8 | rarity, quality, `P(d)`, `T(d)`, `H(d)` and the enemy HP row ending `× diff_hp_bp` (§7.3), power depth cap 300, Marginalia Emendations and `ENH_CAP` |
| Canon | §13.6 | `unlock.daily_baseline`, appearance shuffle at run start |
| Canon | §13.9 | `gear_tier` from the Errata Grade; `illuminated` skin unlocks |
| Canon | §14 | `cur.lucre` (per Erratum; passes between party members only as dropped piles), `cur.quire`, `cur.ink`, `cur.vellum` |
| Canon | §16.1–§16.3 | 13 Callings with major and minor skills, Origins (secret Origins by meta unlock), Echo roster (24, 2 per core genre, with the required `echo.sun_wukong`, `echo.loki`, `echo.lilith`, `echo.fenrir` as `body.quadruped` and `echo.jormungandr` as `body.serpentine`), kin; §16.2 gives this chapter the kit records (`run.kit.*`) and their template bindings, and doc 04 kit roles, presets, kit balance and talents |
| Canon | §17.1 | run constants (`BOSS_FOLIOS` including every Marginalia Folio `f % 5 == 0`, `FOLIO_1_GENRE`, `ERASURE_ONSET`, `ERASURE_STEP`, `EXIT_OFFERS`, `GENRE_REPEAT_RULE`, `GLYPH_LIBRARY_START`), the Colophon (no shop in `genre.unwritten`), `mode.abridged` with `d_eff` and its scaled loot quantity |
| Canon | §17.2, §17.3 | party tables, ranks |
| Canon | §17.5, §17.6 | co-op modes `mode.ff_off`, `mode.ff_full`, `mode.loot_shared`; Ghosts, Remains and resurrection |
| Canon | §17.8 | the Margin trophies: fixed architecture with cosmetic trophy and decoration slots, no building |
| Canon | §17.9 | invasions: `mode.invasion_open` (off by default; Daily, Weekly and ranked runs forbid it), the Redacted outcome when the last Erratum alive and not downed reaches 0 HP, invader on replay slot 4, invader rewards Quires and Red Ink marks only |
| Canon | §18 | the core events below; doc 03 §14.3 registers this chapter's further events (§3.11): `ev.folio_entered`, `ev.folio_exited`, `ev.entity_killed`, `ev.block_changed`, `ev.player_downed`, `ev.player_revived`, `ev.player_died`, `ev.player_ghosted`, `ev.player_resurrected`, `ev.erasure_stack`, `ev.hunter_spawned`, `ev.run_ended` |
| Canon | §19 | this chapter owns `run.`, `mode.`, `grade.`, `margin.`, `unlock.`, `echo.`, `score.`, `ach.` |
| Doc 02 | §6.4–§6.6, §12 | intents (`ChooseExit`, `CommitOffer`), save container kind 1 (run suspend), version-mismatch flow (`prev` branch or Close the Book), headless replay verifier |
| Doc 03 | §2.2, §5.12, §14.3 | tick system order (`sys.erasure` 32, `sys.despawn` 34, `sys.run_layer` 34a, `sys.tick_end` 35); Ghost movement; payloads of the §3.11 events and the `ev.entity_pacified` `method` codes 0 to 4; `is_errata_placed(c)` |
| Doc 04 | §2.5, §2.6, traits | Calling presets and role kits, Erratum creation, attribute allocation, Echo path; `trait.echo.*` signature traits authored here (Interface assumptions) |
| Doc 05, 06, 07, 08, 11 | 05 §5.6, §8.1; 06 §8.1; 07 §9.3, §10.1; 08 §6.1, §9.1, §12.1, §13.2; 11 §3 | kit item IDs and role names, ammunition families and carry caps, glyph learning, resource nodes and placeables, vessel, apparatus, potion and bomb values, base values and stack caps (Interface assumptions) |
| Doc 12 | §4.1, §5.2, §5.4–§5.6, §6.1–§6.3, §7.1, §7.2, §14.1, §20, §23 | Law discovery; edge weights (the Rule F1 fallback); the Volume and run rules with the forced Ur-Text draw exempt (DECISIONS 115); lattice draw on the first lectern interaction, Folio 1 candidates; `GenInput` with `prev_genre` and the `bleed_on` newcomer gate; Folio types, rest Folios and challenges, worldgen sub-streams (the Rubric draw), Folio stories, the Abridged layouts; Folio layout, room tags, Rubric placement, Colophon and Interleaf placement |
| Doc 09 | §1.4, §9.4, §13.5 | `folio_alerted` for `run.note.silent`; subdue takedowns and Hacking-100 seizes as pacifications |
| Doc 13 | §10.5, §10.7, §11.1, §13.6 | spawns, elite promotion, Keeper, bosses, telegraphed-attack flags, `sys.erasure` and the Erasure hunter, the invader Editor kits `enemy.editors.kit.*`, kill credit and `ev.entity_pacified` (`party_kills`, `party_pacified`), the Margin residents `npc.margin.*` |
| Doc 15 | §2.2, §2.4, §2.8.3, §3.1.1, §3.1.2, §3.8, §3.9.1 | Ink and Quire earn rates (a run pays `min(400, floor_div(run_score, 50))` Quires), achievement grade payouts (Quires 10/25/50/100, Ink 40/80/160/320), Echo binding (Illuminated and Rubricated), the Script catalog whose grants this chapter defines (§2.3), Folds and Echo kit A/B rule, Apocrypha rule, the Transcription shelf that unlocks every Script without money (§5.4a), `T0` (§3.2.3, the season start of §9.4), the Second Edition payout and `cosm.frame.ironman` (§2.2) |
| Doc 16 | §5.7, §6.10, §6.14, §9.1–§9.4, §9.13, §11.1, §11.6 | session intents (with `AudienceResult`, §8.1), `RunView` (the network view of `RunState` and `FolioRun`), lobby with `LobbyPick` and `JoinCharacter` carrying `ErratumChoice` and the presentation `Appearance` (§1.3), the audience vote service and the trace service (§8.1), `st.downed`, host migration (hot, warm and cold), invasion end reasons, API hex forms, ranked tokens and submission transport |
| Doc 17 | §10, §13.9, §14 | Margin screens and dialogue text, exit preview widgets, `access.scribes_mercy` (never blocks achievements, milestones, Recollections or Grade unlocks), `access.game_speed_bp` |
| Doc 18 | §5.6, §8.4, §8.5, §8.5a | `Appearance` and the 12 presets per kin (§1.3); Margin cast, endings (`ur_clear_ending`, `nar.ending.second_edition`) and the closing manuscript built from §4.7's inputs, the text of the §5.6 arc beats |
| DECISIONS | 26, 33, 46, 48, 54, 62, 77 to 82, 86, 91, 93, 95 to 97, 100 to 108, 111, 112, 114 to 119, 125, 127, 129 | softer Erasure, Glyph Library, procedural ending, shareable replays, the bleed newcomer gate, pacifist tracking, walkable 3D Margin, resident arcs, 80-hour unlock pace, Recollection for every Echo, ghost XP, Scribe's Mercy, invader rewards, the Erasure hunter's `rank.hunter` identity, host migration paths, the movement kit (taught by contextual tips), asynchronous traces, creator presets (presentation, §1.3), Second Edition with its fused Folios, revised Editors, stakes and Ironman clear, streaming votes, no hard dates (season start), Steam only, Early Access scope (114 supersedes 41), the forced Ur-Text draw, the Origin fallback, per-pair no-repeat, the Early Access feature set and cast, invasions after Early Access, no Margin relationships, no photo mode |

---

## 1. Constants and run state

### 1.1 Run-layer constants

| Constant | Value | Used in |
|---|---|---|
| `SPEED_PAR_S` | 360 s | §4.2 speed points |
| `TURN_HOLD_TICKS` | 120 | §3.6 exit commit |
| `EXIT_MAJORITY_TICKS` | 3600 | §3.6 majority fallback |
| `EXIT_ANY_TICKS` | 7200 | §3.6 single-voter fallback |
| `ERASURE_WARN_1_TICKS` | 36000 (600 s) | §3.8 UI cue |
| `ERASURE_WARN_2_TICKS` | 41400 (690 s) | §3.8 UI cue |
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
| `HUNTER_SCORE_CAP_PER_FOLIO` | 1 | §4.2 (one Erasure hunter per Folio, Canon §11.6) |
| `MUTATOR_BP_CAP` | 20000 | §4.2 |
| `RUN_SCORE_BP` | 1460 | §4.6, the `run_score` handed to doc 15 |
| `RUN_SCORE_FOLIO_CAP` | 800 per cleared Folio the slot was present for | §4.6 |
| `DIFF_CAP_BP` | 1,000,000 | §7.3, §9.2 |
| `MARGINALIA_MAX_FOLIO` | 9999 | §9.1 |
| `RECOLLECTION_ACTIVE_MAX` | 2 | §6.1 |
| `AUTOSAVE_PERIOD_TICKS` | 3600 | §12.1 |
| `RANKED_RESTORE_MAX` | 2 | §12.3 |
| `DORMANT_GRACE_S` | 300 s of wall-clock time | §8.8 |
| `Q_REF` | 60 Quires per reference hour (doc 15's reference failed run, §4.6) | §5.4 pacing reference only |
| `Q_WIN` | 143 Quires per hour of a reference Grade-0 victory (§4.6) | §5.4 pacing reference only |
| `Q_MIX` | 101 Quires per hour: `floor_div(Q_REF + Q_WIN, 2)`, a career with as many victory hours as failed-run hours | §5.4a pacing reference only |
| `UNLOCK_PACE_HOURS` | 80 (DECISIONS 79) | §5.4a |
| `ARC_BEATS` | 6 per Margin resident | §5.6 |

### 1.2 Run state (simulation)

`RunState` lives in `palimpsest_sim` and is part of every save (Canon §2.5). Account data (§5) never lives in simulation state.

```rust
pub struct RunState {
    pub mode: Id32,                       // mode.*; switches once: to mode.endless (§9.1) or mode.second_edition (§8.9)
    pub structure: Structure,             // Standard | Novella | Abridged | Daily (§8.1); fixed at Commit
    pub final_folio: u32,                 // 25, 15, 10 or 5; u32::MAX once Endless; Ur-Text index + 15 in Second Edition (§8.9)
    pub run_seed: u64,
    pub run_id: u64,                      // host OS CSPRNG at Commit, before the ranked token request (doc 15 §10.4, doc 16 §11.6)
    pub content_set: Vec<Id32>,           // sorted unlock-gated content, frozen at run start
    pub grade: u8,                        // 0..=20
    pub gp: GradeParams,                  // §7.2, merged with mutators at run start
    pub mutators: Vec<Id32>,              // run.mut.*, sorted by content ID string
    pub coop: CoopModes,                  // { ff: Default | Off | Full, loot_shared: bool } (Canon §17.5, §8.8)
    pub invasion_open: bool,              // mode.invasion_open (Canon §17.9, §8.1); fixed at Commit
    pub audience: bool,                   // mode.audience_votes (§8.1); fixed at Commit
    pub chaos: Option<Id32>,              // §8.1 audience Chaos mutator active in the current Folio
    pub chaos_next: Option<Id32>,         // §8.1 audience Chaos mutator for the next Folio
    pub party: BTreeMap<u8, SlotState>,   // key = party slot 0..=3
    pub folio_index: u32,                 // 0 before Folio 1
    pub phase: FolioPhase,                // §3.2: Exploring | ColophonOpen | InColophon | ExitVote | BossAntechamber
                                          //   | BossFight | Endpaper | VictoryChoice | Ended(end)
    pub cur: FolioRun,                    // per-Folio run-layer state, reset at entry
    pub history: Vec<FolioRecord>,        // one per Folio entered, ascending index
    pub marg_mut: BTreeMap<Id32, u8>,     // run.marg.* -> rank 1..=5 (§9.3)
    pub laws_triggered: BTreeMap<Id32, u8>, // doc 12 §4.1: World Law -> slot mask at its first trigger (§5.5)
    pub bleed_on: bool,                   // doc 12 §14.1 newcomer gate, fixed at Commit in the run header, passed in GenInput
    pub colophon_marked: bool,            // §3.9 desecration; never cleared
    pub colophon_hunter_kills: u8,        // §3.9, saturates at 255
    pub score_at_victory: i64,            // 0 until victory (§4.2)
    pub end: Option<RunEndRecord>,
    pub ranked: RankedState,              // §12.3
    pub satchels: BTreeMap<u8, Satchel>,  // doc 15 §5.5: each slot's Keepsake records and materialization cursor
}

pub enum SlotStatus { Active, Ghost, Dormant, Departed }

pub enum ErratumChoice {                  // doc 04 §2.1 creation output; validated by doc 04 before Commit or SlotJoin
    Custom { kin: Id32, origin: Id32, calling: Id32,   // kin.*, origin.*, calling.*
             subs: [u8; 24],              // doc 04 §2.2 final sub-attribute values, in Canon §6.1 table order
             human_skill: Option<Id32> }, // kin.human's +5 skill (doc 04 §2.2 step 7); None for every other kin
    Echo { echo: Id32, trait_variant: u8 },              // 0, 1, 2 = trait variants A, B, C (doc 15 Folds 2 and 5)
}

pub struct SlotState {
    pub entity: u64,                      // EntityId; kept across Folios (Canon §1.11)
    pub erratum: ErratumChoice,           // above; never holds appearance or name (§1.3)
    pub kit: Id32,                        // run.kit.* chosen in the Dramatis (§2.4)
    pub status: SlotStatus,
    pub folios_present: u32,              // Folios entered while Active or Ghost
    pub lucre_norm: i64,                  // §4.1
    pub quires_paid: i64,                 // §8.8 departed payouts
    pub going_native_used: bool,          // Canon §5.11, once per run
    pub went_native: Option<Id32>,        // the genre gone native into, for §4.7
    pub active_ticks: u64,                // doc 15 §2.2 active-tick rule
    pub last_input_tick: u64,             // doc 15 §2.2
    pub library_glyphs: Vec<Id32>,        // 0..=2 Glyph Library picks learned at start or join (§2.2, §5.7)
    pub glyphs_learned: BTreeSet<Id32>,   // every glyph this slot's Erratum learned this run (§5.7)
    pub boon: Option<Id32>,               // run.boon.* active in the current Folio (§3.9)
    pub boon_next: Option<Id32>,          // run.boon.* committed at this Folio's Colophon
    pub remains_carried: bool,            // Ghost only: set at exit commit (§3.10)
}

pub struct FolioRecord {
    pub index: u32, pub genre: Id32, pub kind: FolioKind,   // Normal | Boss | Final
    pub fused: Option<Id32>,                                // Second Edition: the pair's other genre; `genre` has the lower Canon §5.2 row (§8.9)
    pub editor: Option<Id32>,                               // boss Folio: the boss. ID of its (first) Editor, set at entry (§3.7, §8.9, §9.1)
    pub depth: u32, pub volume: u32,                        // §3.1: d(f) and v(f)
    pub entry_tick: u64, pub exit_tick: Option<u64>,
    pub rooms_total: u32, pub rooms_visited: u32,           // doc 12 room count, union over party
    pub notes_offered: Vec<Id32>, pub notes_done: Vec<Id32>,
    pub doors_offered: Vec<Id32>, pub door_taken: Option<Id32>,  // genres of the doors shown and of the committed door (doc 01 KPIs)
    pub rewrites: u32,                                      // Rewrites committed in this Folio (§3.5 run.note.scribe, doc 01 KPIs)
    pub editor_killed: bool, pub telegraphed_hits: u32,
    pub hunters_killed: u32, pub erratum_deaths: u32,       // Erasure hunters only (§4.2); deaths that did not end the run
    pub party_kills: u32, pub party_pacified: u32,          // kills credited to a party slot (doc 13 §11.1); ev.entity_pacified by one (§10)
    pub party_size: u8,
    pub colophon_entry_tick: Option<u64>,                    // stops Erasure accrual (§3.8)
    pub story: StoryRecord,                                  // doc 12 §20.5 story record (template IDs, name seeds), set at entry
    pub story_solved: Option<u64>,                           // tick doc 12 §20 marks the story solved, else None (§4.7)
    pub leaf_carried: bool,                                  // a Folio Leaf was carried through the exit (doc 12 §20)
    pub invasion: u8,                                        // 0 none, 1 invader killed, 2 party Redacted, 3 timed out or left
}

pub struct FolioRun {                                        // RunState.cur, rebuilt at each Folio entry (§3.3)
    pub index: u32, pub entry_tick: u64,
    pub clock_start: Option<u64>,                            // §3.8: the entry tick, or the Editor's death tick in a boss Folio
    pub colophon_entry_tick: Option<u64>,                    // §3.2, §3.8; copied into FolioRecord
    pub rubric: Option<RubricState>,                         // §3.4: kind, progress, goal, done; None in rest and boss Folios
    pub notes: Vec<Id32>,                                    // §3.5 notes offered, ascending ID
    pub rooms_visited: BTreeSet<u16>,                        // doc 12 room indices an Active Erratum's origin entered; FolioRecord.rooms_visited is its size
    pub map_revealed: bool,                                  // an item.text.mapping was read in this Folio (doc 06)
    pub vote: Option<VoteState>,                             // §3.6, from the first lectern interaction
    pub boon_offers: BTreeMap<u8, [Id32; 3]>,                // §3.9 stored boon offers, by slot
    pub heal_claimed: u8, pub rested: u8,                    // slot bit masks: §3.9 Colophon full heal and Interleaf Rest
    pub hunter_spawned: bool,                                // §3.8: at most one Erasure hunter per Folio
}
```

Doc 16's `RunView` replicates the phase, the Rubric, the notes, the vote holds, the next Erasure tick, `rooms_visited` and `map_revealed` (doc 16 §6.10); it is the network view of this state, never the state itself.

### 1.3 Appearance and names (presentation)

A Custom Erratum's look, doc 18 §5.6's `Appearance { preset, base, marking }` (one of its kin's 12 presets with a base and a marking swatch, DECISIONS 97), and every Erratum's display name are presentation data. Neither is part of `ErratumChoice`, `SlotState` or any other simulation state, so two runs that differ only in them have the same `content_hash` and the same `state_hash` sequence (doc 18 acceptance test 65). An Echo's look is fixed by its record (doc 04 §2.1). Peers learn a slot's look and name from doc 16's `LobbyPick` and `JoinCharacter`, which carry `appearance: Option<Appearance>` (`None` for an Echo) and the name beside the `ErratumChoice`; the host relays both in `PartyState`, the replay file carries them for the viewer, and the slot's `RunSummary` entry records them (§4.6), so the §12.1 checkpoint record and the run history keep them.

---

## 2. Start flow from the Margin

### 2.1 Lobby state machine

The Lectern (`margin.lectern`) starts every run. The lobby is client-side meta state until `Commit`, after which the host builds `RunState`.

| State | Entered when | Exits to | Condition |
|---|---|---|---|
| `Idle` | Margin loaded | `ModeSelect` | a player interacts with the Lectern |
| `ModeSelect` | | `ErratumSelect` | the host picks a mode the host's account has unlocked (§8.1) |
| `ErratumSelect` | | `RunSetup` | every slot has a valid Erratum, a kit it owns (§2.4), for an Echo an owned trait variant (doc 15 Folds), and at most `GLYPH_LIBRARY_START` (2) distinct Glyph Library picks (§5.7); in the Daily the Calling and Origin of every slot are drawn (§8.4) and each player makes only the glyph picks here |
| `RunSetup` | | `ReadyCheck` | host sets structure (Seeded, Custom), Grade (≤ host's unlocked maximum), mutators (Custom), seed (Seeded, Custom) and, with 2 or more slots, the co-op modes `mode.ff_off` or `mode.ff_full` and `mode.loot_shared` (§8.8), and with any number of slots the overlays `mode.invasion_open` and `mode.audience_votes` (§8.1). Each player orders a Satchel where the mode allows one (doc 15 §5.5). When a simulation-affecting mod is loaded the screen first shows doc 15's Apocrypha notice and the run is Apocrypha (§8.1) |
| `ReadyCheck` | | `Commit` | every connected player toggles Ready; a slot that disconnects returns the lobby to `ErratumSelect` for that slot |
| `ReadyCheck` | | `RunSetup` | host edits any setting; every Ready flag clears |
| `Commit` | | `Folio1` | the host draws `run_id` from its OS CSPRNG, fixes `bleed_on` (doc 12 §14.1, DECISIONS 54: 1 for a Daily, Weekly or typed-seed run, or when the host account has at least 3 party run ends of §4.3, `run.end.departed` excluded; 0 otherwise), stores it in the run header and passes it in every doc 12 `GenInput`, obtains the ranked token if the mode is ranked (§12.3, doc 16 §11.6), sends the run header carrying `run_seed` (Canon §2.4), and `start_run` returns |
| any | host leaves | `Idle` | lobby disbanded |

A valid Erratum: Calling and Origin are unlocked on that player's account (or the slot uses an Echo the player owns), and for Weekly and Seeded baseline runs the Calling and Origin belong to `unlock.daily_baseline`, the slot uses no Echo, and its kit is the Calling's standard kit (`unlock.daily_baseline` holds no Echoes, Scripts or alt kits).

| Lobby input | Valid values |
|---|---|
| Party slots | 1..=`PARTY_MAX` (4) |
| Grade | `0..=max_grade(host)` (§7.4); Daily 2 and Weekly 6 are fixed |
| Structure | Standard, Novella or Abridged; Custom adds Standard with `run.mut.short_book` (§8.7) |
| Seed | 13-character Canon §3.6 display form (Seeded, Custom) |
| Co-op modes | none, `mode.ff_off`, `mode.ff_full` (mutually exclusive); `mode.loot_shared` on or off; hidden with 1 slot |
| Audience votes | `mode.audience_votes` on or off, default off; hidden in Daily and Weekly Gauntlet; a run with it gets no ranked token (§8.1) |
| Invasions | `mode.invasion_open` on or off, default off; hidden in Daily and Weekly Gauntlet; refused unless every member is online, no simulation-affecting mod is loaded and every member's account permits invasions (doc 15 `juris.flag.invasion_minor_default`, doc 16) |

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
    // 8. Kits (§2.4): ascending slot order; per slot the universal kit, then the chosen kit in
    //    record order; uids from the folio_index 0 counter; kit minions (Beastbinder) spawn after
    //    every Erratum, in ascending slot order. Kit generation draws no random numbers.
    // 9. Glyph Library (§5.7): each Erratum, in ascending slot order, learns its 0..=2 library picks after doc 06
    //    §8.1's run-start set, origin = its Origin genre (genre.ur_text for an Unwritten Origin); ev.glyph_learned
    //    source 0. Satchels are stored in RunState.satchels (doc 15 §5.5). No draws.
    // 10. Calling lore (§2.4.3) marks item types known for the party.
    // 11. Enter Folio 1 (§3.3). The first save is written there.
}
```

**Rule F1 (Folio 1 genre, Canon §17.1 `FOLIO_1_GENRE`).** Unseeded runs (Standard, Novella, Abridged and Custom without a typed seed) use the host Erratum's Origin genre (slot 0's Origin). When that Origin is a secret genre (§5.3 shelf A), Folio 1 is the core genre doc 12 §5.6 maps it to (Oneiric → Fairy Tale, Redaction → Cyberpunk, Unwritten → Mythic Fantasy, Ur-Text → Mythic Fantasy), because secret genres are never lattice candidates. When the resulting genre is not one the build ships (DECISIONS 116), Folio 1 uses the shipped core genre with the greatest doc 12 §5.2 edge weight from it, ties to the first in ascending ID order (Canon §1.11). Seeded runs (Daily, Weekly, Seeded, and Custom with a typed seed) use the first candidate of doc 12's `lattice_candidates(run_seed, 0, lattice.node.margin, [], mode)` (`rng.lattice` with `folio_index = 0`, doc 12 §5.6).

### 2.3 Content sets

| Set | Contents |
|---|---|
| `unlock.set.starter` | content every account owns at creation: Callings Vanguard, Duelist, Gunslinger, Spellwright; Origins Mythic Fantasy, Gothic Horror, Cyberpunk, Weird West; the 13 standard kits; every item, room and event not gated by a `unlock.loot.*`, `unlock.room.*`, `unlock.event.*`, `unlock.glyph.*` or `unlock.hireling.*` ID |
| account set | `unlock.set.starter` ∪ content of purchased Bindery nodes ∪ content granted by owned Inkwell Scripts (doc 15) ∪ owned Echoes |
| `unlock.daily_baseline` | `unlock.set.starter` ∪ all 13 Callings ∪ all 12 core Origins. No alt kits, no Bindery loot, room or event nodes, no Scripts, no Echoes, no Keepsakes, no secret Origins (Canon §13.6, doc 15) |

**Script-granted unlock IDs.** Doc 15's Scripts (doc 15 §3.1.2) grant IDs in this chapter's `unlock.` namespace, outside the Bindery, in four reserved patterns: `unlock.kit.<calling>_<name>` (a kit variant `run.kit.<calling>_<name>`, §2.4.2), `unlock.glyph.<family>` (a doc 06 glyph family), `unlock.room.<name>` (a doc 12 room variant) and `unlock.hireling.<type>` (a doc 13 hireling type). They are never sold for Quires. The content validator rejects a Script grant whose ID equals a Bindery node ID or `unlock.kit.<calling>_alt`. The launch catalog defines these 48 IDs; each Folio Expansion adds its own under the same patterns (doc 15):

| ID | Granted by (doc 15 §3.1.2) | Adds to the account content set |
|---|---|---|
| `unlock.room.lost_chapter_<g>` for the 12 core genre suffixes (`mythic_fantasy`, `gothic_horror`, `cosmic_horror`, `space_opera`, `cyberpunk`, `post_apocalypse`, `weird_west`, `hardboiled_noir`, `wuxia`, `steampunk`, `fairy_tale`, `mythic_antiquity`) | `bundle.script.lost_chapter_<g>` (Illuminated) | 3 room variants in genre `g`'s room pool (doc 12) |
| `unlock.hireling.chapter_<g>` for the same 12 suffixes | `bundle.script.lost_chapter_<g>` (Illuminated) | 1 hireling type native to genre `g` (doc 13) |
| `unlock.glyph.ember` | `bundle.script.glyphs_<family>` for that family (Rubricated) | the 4 glyphs of doc 06 `family.ember` in the glyph loot pool, inside their rarity bands |
| `unlock.glyph.hollow` | `bundle.script.glyphs_<family>` | the 4 glyphs of `family.hollow` |
| `unlock.glyph.mirror` | `bundle.script.glyphs_<family>` | the 4 glyphs of `family.mirror` |
| `unlock.glyph.storm` | `bundle.script.glyphs_<family>` | the 4 glyphs of `family.storm` |
| `unlock.glyph.thorn` | `bundle.script.glyphs_<family>` | the 4 glyphs of `family.thorn` |
| `unlock.glyph.tide` | `bundle.script.glyphs_<family>` | the 4 glyphs of `family.tide` |
| `unlock.hireling.apothecary` | `bundle.script.hireling_<type>` for that type (Rubricated) | 1 hireling type in the Hireling Hall pool (doc 13) |
| `unlock.hireling.cartographer` | `bundle.script.hireling_<type>` | 1 hireling type in the Hireling Hall pool |
| `unlock.hireling.lamplighter` | `bundle.script.hireling_<type>` | 1 hireling type in the Hireling Hall pool |
| `unlock.hireling.porter` | `bundle.script.hireling_<type>` | 1 hireling type in the Hireling Hall pool |
| `unlock.hireling.sellsword` | `bundle.script.hireling_<type>` | 1 hireling type in the Hireling Hall pool |
| `unlock.kit.<calling>_script` for the 13 Calling suffixes | `bundle.script.kit_<calling>` (Rubricated) | the Script kit `run.kit.<calling>_script` (§2.4.2), selectable in the Dramatis |

Hireling types reach a run only where their pool is used: a Hireling Hall room needs `unlock.room.hireling_hall` on the host's account. Every unlock here is horizontal (§5.2): room variants, glyphs and hirelings enter their pools inside their rarity bands or as options, and a Script kit obeys the 500 Bp kit rule.

The host's set is the run's set (Canon §13.6). A guest whose Calling or Origin is outside the host's set still plays it: Callings and Origins are per-slot choices, not pool content.

**Early Access scope** (DECISIONS 114, which supersedes 41; DECISIONS 118, 119). The Early Access build ships all 12 core genres, all 12 core Origins, all 24 Echoes, every base mode of §8.1 and Second Edition (§8.9); the 4 secret genres follow their unlock rules (§5.3 shelf A, doc 12 §17.4). Every content set, count and draw in this chapter is therefore the same at Early Access and at 1.0: `ach.atlas`, `ach.origins_v1` and the §5.6 cartographer arc count 12 core genres, and the §8.4 Daily draws its Origins with `range(12)`. The overlays `mode.invasion_open` and `mode.audience_votes` and asynchronous traces (§8.1) ship in updates after Early Access and before 1.0 (DECISIONS 118, 125; doc 21); until a build ships one, the lobby hides it and nothing in this chapter reads it. A record keyed to a genre outside the 16 canon genres (a later Folio Expansion, doc 15) enters a content set only through its own unlock, and no count or draw over "the 12 core genres" includes it.

### 2.4 Starting kits

Canon §16.2 gives doc 04 the kit roles, presets, kit balance and talents, and gives this chapter the kit records (`run.kit.*`) and their template bindings. Doc 04 §2.5 fixes each Calling's kit by role ("1H hafted weapon, medium shield, ..."). This section binds every role to an exact owner template ID through doc 05 §5.6's role names, and adds the run-layer rules around kits: alt kits, Calling lore, Margin Translation and the kit record.

Kit items are created with `rarity = rar.common`, `quality = q.normal`, `ilvl` = the depth `d(f)` of the Folio at which the kit is created (1 at run start, §3.1), `enhancement = 0`, `buc = buc.uncursed`, `curse_penalty = 0`, no affixes, `illuminated = false`, `manufacturer = none`, `gear_tier` per Canon §13.9 from the run's Grade, and `knowledge = know.identified`; a type-hidden kit item adds its type to the party's known types with reason `appr.reason.kit` (doc 10). Kit generation draws no random numbers (doc 11 §2: kits are the only items not made by `generate_item`). Every alt or Script kit of a Calling has a total base value within 500 Bp of that Calling's standard kit (doc 15 §3.8 kit A/B rule; acceptance test 9). Every stack respects its owner's cap: `arch.ranged.thrown` at most 20 per stack (doc 11 §2.1), ammunition of the weapon's own family and at most its doc 05 §8.1 carry cap, throwables and potions at most 10 (doc 11 §3.5). Base values are doc 11 §3's at ilvl 1, doc 08's for vessels, apparatus, potions and bombs, and `floor_div(units × value_per_100, 100)` for ammunition (doc 05 §8.1).

#### 2.4.1 Universal kit `run.kit.universal` (every Erratum, doc 04 §2.5)

| Item | Qty | Slot or bag | Base value at ilvl 1 |
|---|---|---|---|
| `item.food.ration` | 2 | bag | 8 |
| `item.tool.bandage` | 2 | quickbar 1 | 12 |
| `alch.potion.healing` (type-identified) | 1 | quickbar 2 | 30 (doc 08 §12.1) |
| Lucre | `mul_bp(50, P(d))`, 50 at run start | purse | 50 |

Total 100 Lucre of base value at ilvl 1.

#### 2.4.2 Calling kits `run.kit.<calling>` (variant 0)

Armor sets: **light set** = `arch.armor.head_light` (12), `arch.armor.body_light` (30), `arch.armor.hands_light` (10), `arch.armor.legs_light` (18), `arch.armor.feet_light` (10), value 80; **medium set** = the same five slots with the suffix `_medium` (98); **cloth set** = the same five slots with the suffix `_cloth` (52, used only by Script kits). The first weapon goes to `slot.weapon_1` and any second weapon to `slot.weapon_2`; a thrown-weapon stack (`arch.ranged.thrown`, `cat.weapon_ranged`) is a weapon; shields, foci, tomes and lanterns go to `slot.offhand`; armor goes to its slot; accessories go to their slot (`slot.face`, `slot.back`, `slot.belt`); consumables fill the quickbar in table order after the universal kit, then the bag; ammunition, tools, books, apparatus and containers go to the bag. An entry whose slot the Erratum's body plan lacks, or that the plan cannot hold there (doc 04 §13: `body.quadruped` has no `slot.offhand` or `slot.hands` and holds only a one-handed melee weapon or a harness ranged weapon in `slot.weapon_1`; `body.serpentine` has no `slot.legs` or `slot.feet`), goes to the bag instead.

| Calling | doc 04 §2.5 role list | Exact templates | Calling-part value |
|---|---|---|---|
| Vanguard | 1H hafted weapon, medium shield, heavy armor (head, body, legs) | `arch.melee.mace`; `arch.offhand.shield_heater`; `arch.armor.head_heavy`, `arch.armor.body_heavy`, `arch.armor.legs_heavy` | 206 |
| Duelist | 1H long blade, 6 throwing knives, light armor set | `arch.melee.blade_long`; `arch.ranged.thrown` ×6; light set | 192 |
| Gunslinger | sidearm with 60 rounds, 4 throwing blades, light armor set | `arch.ranged.pistol`; `ammo.light` ×60; `arch.ranged.thrown` ×4; light set | 186 |
| Grenadier | heavy ranged weapon with 120 rounds, 4 grenades, medium armor set | `arch.ranged.rifle_auto`; `ammo.heavy` ×120 (of 180 carried at most); `arch.ranged.grenade_frag` ×4; medium set | 314 |
| Spellwright | casting focus, glyph tome (2 glyphs), light armor set | `arch.offhand.focus`; `item.book.primer_evocation` (teaches `glyph.ess.flame`, `glyph.ess.frost`); light set | 172 |
| Alchemist | dagger, field alembic, 3 bombs, 4 empty flasks | `arch.melee.dagger`; `apparatus.mortar` tier 1 (10) and `apparatus.alembic` tier 1 (15) (doc 08 §9.1 requires a mortar to brew); `alch.bomb.firebomb` ×3; `apparatus.vessel.flask` ×4 | 109 |
| Shade | short blade, lockpicks, light armor set, smoke bomb | `arch.melee.blade_short`; `item.tool.lockpicks` ×1 (a carried tool that no attempt consumes, doc 09 §13.2, so one suffices); light set; `arch.ranged.grenade_smoke` ×1 | 182 |
| Artificer | sidearm with 60 rounds, toolkit, 1 deployable turret, medium armor | `arch.ranged.pistol`; `ammo.light` ×60; `item.tool.toolkit`; `gadget.turret_light`; medium set | 248 |
| Warden | spear, tower shield, medium armor set, 1 healing glyph | `arch.melee.spear`; `arch.offhand.shield_tower`; medium set; `item.book.primer_abjuration` (teaches `glyph.form.aura`, which with the universally known `glyph.ess.mend` makes an area heal, doc 06 §8.1) | 274 |
| Oracle | staff, holy symbol (prayer focus), light armor set | `arch.ranged.staff` (two-handed); `item.trinket.holy_symbol` (60) in `slot.trinket`; light set | 240 |
| Netrunner | sidearm with 60 rounds, hacking deck, light armor set | `arch.ranged.pistol`; `ammo.light` ×60; `item.tool.hacking_deck` (60); light set | 222 |
| Beastbinder | spear, 1 tamed companion beast, light armor set | `arch.melee.spear`; light set; one tamed beast of doc 13 template `npc.companion.<origin_suffix>`, spawned beside its owner with a `folio_index = 0` EntityId and bound as a minion (Canon §15.1; the cap at skill 20 is exactly 1) | 138 |
| Monk | hand wraps (unarmed archetype), light armor set | `arch.melee.fist`; light set | 136 |

With the universal kit (100), standard kits total 209 (Alchemist) to 414 (Grenadier) Lucre of base value at ilvl 1. Base value is a shop price, not power; doc 04 owns kit balance (Canon §16.2).

Alt kits `run.kit.<calling>_alt` (variant 1, Bindery shelf A) change only the listed entries; each stays within 500 Bp of the standard kit's Calling-part value (the universal kit is identical in both). Base values are in parentheses, and the difference is `floor_div((alt - standard) × 10000, standard)`:

| Calling | Alt-kit change | Alt value | Difference |
|---|---|---|---|
| Vanguard | `arch.melee.warhammer` (two-handed, 86) and `arch.ranged.thrown` ×4 (24) replace the mace (58) and the shield (52) | 206 | 0 Bp |
| Duelist | `arch.melee.fist` (56) replaces the long blade (76); `arch.ranged.thrown` ×10 (60) replaces ×6 (36) | 196 | +208 Bp |
| Gunslinger | `arch.ranged.rifle_marksman` (94) and `ammo.rifle` ×12 (36, one magazine) replace the sidearm (70), `ammo.light` ×60 (12), the 4 thrown knives (24), `arch.armor.head_light` (12) and `arch.armor.hands_light` (10) | 188 | +107 Bp |
| Grenadier | `arch.ranged.beam` (98) and `ammo.cell` ×300 (60, the carry cap) replace the heavy weapon (128) and `ammo.heavy` ×120 (48); `arch.ranged.grenade_frag` ×6 (60) replaces ×4 (40) | 316 | +63 Bp |
| Spellwright | the primer teaches `glyph.ess.storm` and `glyph.form.chain` instead | 172 | 0 Bp |
| Alchemist | `arch.ranged.thrown` ×5 (30, the sling-stone skin, doc 05 §5.3) replaces the dagger (40); `alch.potion.toxic_gas` ×3 (45) replace the 3 firebombs (36) | 108 | −92 Bp |
| Shade | `arch.ranged.pistol` (70) and `ammo.light` ×30 (6) replace the short blade (52), the smoke grenade (10) and `arch.armor.hands_light` (10) | 186 | +219 Bp |
| Artificer | `gadget.drone_scout` (40) replaces the turret (44) | 244 | −162 Bp |
| Warden | `arch.melee.mace` (58) and `arch.offhand.shield_heater` (52) replace the spear (58) and the tower shield (70); `item.tool.bandage` ×3 (18) added | 274 | 0 Bp |
| Oracle | `arch.ranged.wand` (72) and `alch.potion.purity` ×1 (20) replace the staff (100) | 232 | −334 Bp |
| Netrunner | `arch.melee.blade_short` (52) and `item.throwable.counter_disruptor` ×2 (28) replace the sidearm (70) and `ammo.light` ×60 (12) | 220 | −91 Bp |
| Beastbinder | `arch.ranged.thrown` ×4 (24, the sling-stone skin) and `arch.offhand.shield_buckler` (36) replace the spear (58) | 140 | +144 Bp |
| Monk | `arch.melee.quarterstaff` (38) and `arch.ranged.thrown` ×3 (18) replace the hand wraps (56) | 136 | 0 Bp |

Script kits `run.kit.<calling>_script` (variant 2, granted by `unlock.kit.<calling>_script` from doc 15's Rubricated `bundle.script.kit_<calling>`, §2.3) are replacements against the standard kit like the alt kits. Values of the swapped templates are doc 11 §3's base values at ilvl 1 (doc 05 §8.1 for ammunition); the difference is `floor_div((added - removed) × 10000, standard Calling-part value)`:

| ID | Script-kit change | Script value | Difference |
|---|---|---|---|
| `run.kit.vanguard_script` | `arch.melee.axe` (58) replaces the mace (58); `arch.offhand.shield_buckler` (36), `item.tool.bandage` ×2 (12) and `item.tool.torch` ×2 (4) replace the medium shield (52) | 206 | 0 Bp |
| `run.kit.duelist_script` | `arch.melee.blade_short` (52) and `item.face.mask` (30) replace the long blade (76) | 198 | +312 Bp |
| `run.kit.gunslinger_script` | `arch.ranged.pistol_heavy` (84) and `ammo.heavy` ×30 (12) replace the sidearm (70) and `ammo.light` ×60 (12); `arch.armor.hands_light` (10) removed | 190 | +215 Bp |
| `run.kit.grenadier_script` | `arch.armor.head_heavy`, `arch.armor.body_heavy`, `arch.armor.legs_heavy` (22 + 46 + 28) replace the medium set (98) | 312 | −64 Bp |
| `run.kit.spellwright_script` | `arch.offhand.tome` (48) replaces the focus (44); the primer is unchanged | 176 | +232 Bp |
| `run.kit.alchemist_script` | `arch.melee.quarterstaff` (38) replaces the dagger (40) | 107 | −184 Bp |
| `run.kit.shade_script` | `arch.melee.dagger` (40) and `item.tool.rope` ×2 (12) replace the short blade (52) | 182 | 0 Bp |
| `run.kit.artificer_script` | `arch.ranged.smg` (86, fires `ammo.light`) replaces the sidearm (70); the light set (80) replaces the medium set (98) | 246 | −81 Bp |
| `run.kit.warden_script` | `arch.offhand.shield_buckler` (36) and `item.back.cloak` (40) replace the tower shield (70) | 280 | +218 Bp |
| `run.kit.oracle_script` | `arch.ranged.wand` (72) and `arch.offhand.lantern` (28) replace the staff (100) | 240 | 0 Bp |
| `run.kit.netrunner_script` | the cloth set (52) and `item.face.mask` (30) replace the light set (80) | 224 | +90 Bp |
| `run.kit.beastbinder_script` | `arch.melee.quarterstaff` (38), `item.tool.bandage` ×2 (12), `item.tool.rope` (6) and `item.tool.torch` (2) replace the spear (58) | 138 | 0 Bp |
| `run.kit.monk_script` | the cloth set (52) and `item.belt.sash` (30, quickbar 5) replace the light set (80) | 138 | +147 Bp |

Kit IDs: `run.kit.universal`; for each of the 13 Callings, `run.kit.<calling>` is variant 0, `run.kit.<calling>_alt` variant 1 and `run.kit.<calling>_script` variant 2. Expansion Script kits `run.kit.<calling>_<name>` (doc 15) are written the same way, as replacements against the standard kit, with the next free variant number. An Echo's kit A is its Calling's standard kit and its kit B (doc 15 Fold 3) is its Calling's alt kit; an Echo may use kit B only at Fold 3 or above, whether or not the account owns the alt kit node.

#### 2.4.3 Calling lore

At run start each Calling marks two item types `know.classified` for the whole party (type knowledge is party-wide, Canon §13.6) with reason `appr.reason.kit` (doc 10), after the appearance shuffle and after the kits. Healing potions are already known from the universal kit, so no lore entry names them. Duplicates across slots have no extra effect.

| Calling | Known types |
|---|---|
| Vanguard | `alch.potion.fortitude`, `item.text.mending` |
| Duelist | `alch.potion.haste`, `item.text.blink` |
| Gunslinger | `alch.potion.calm`, `item.text.mapping` |
| Grenadier | `alch.potion.fortitude`, `item.text.demolition` |
| Spellwright | `alch.potion.aether`, `item.text.appraisal` |
| Alchemist | `alch.potion.cure`, `alch.potion.honeyed_healing` |
| Shade | `alch.potion.invisibility`, `item.text.mapping` |
| Artificer | `item.text.appraisal`, `item.text.recharge` |
| Warden | `alch.potion.shielding`, `item.text.sanctify` |
| Oracle | `item.text.appraisal`, `alch.potion.purity` |
| Netrunner | `item.text.mapping`, `item.text.recharge` |
| Beastbinder | `alch.potion.cure`, `item.text.summoning` |
| Monk | `alch.potion.haste`, `alch.potion.fortitude` |

With `unlock.option.lens_pocket` a player may trade the second lore entry for one `item.trinket.lattice_lens` in `slot.trinket`.

#### 2.4.4 Margin Translation (kit skin genre)

Errata fall out of their own stories, so kit items carry the Erratum's Origin skin, except where that skin would arrive with a Contradiction above 2 in the Folio-1 genre.

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
    { "template": "arch.ranged.pistol", "slot": "slot.weapon_1", "qty": 1 },
    { "template": "ammo.light", "slot": "bag", "qty": 60 },
    { "template": "arch.ranged.thrown", "slot": "slot.weapon_2", "qty": 4 },
    { "template": "arch.armor.head_light", "slot": "slot.head", "qty": 1 },
    { "template": "arch.armor.body_light", "slot": "slot.body", "qty": 1 },
    { "template": "arch.armor.hands_light", "slot": "slot.hands", "qty": 1 },
    { "template": "arch.armor.legs_light", "slot": "slot.legs", "qty": 1 },
    { "template": "arch.armor.feet_light", "slot": "slot.feet", "qty": 1 }
  ],
  "minions": [],
  "lore": ["alch.potion.calm", "item.text.mapping"],
  "consumable_entries": []
}
```

```json
{
  "id": "run.kit.gunslinger_alt",
  "calling": "calling.gunslinger",
  "variant": 1,
  "base": "run.kit.gunslinger",
  "replace": [
    { "remove": [0, 1, 2, 3, 5],
      "add": [ { "template": "arch.ranged.rifle_marksman", "slot": "slot.weapon_1", "qty": 1 },
               { "template": "ammo.rifle", "slot": "bag", "qty": 12 } ] }
  ]
}
```

```json
{
  "id": "run.kit.gunslinger_script",
  "calling": "calling.gunslinger",
  "variant": 2,
  "base": "run.kit.gunslinger",
  "unlock": "unlock.kit.gunslinger_script",
  "replace": [
    { "remove": [0, 1, 5],
      "add": [ { "template": "arch.ranged.pistol_heavy", "slot": "slot.weapon_1", "qty": 1 },
               { "template": "ammo.heavy", "slot": "bag", "qty": 30 } ] }
  ]
}
```

The kit validator recomputes `consumable_entries` from the rule below and rejects a record that disagrees, and rejects any alt or Script kit whose Calling-part value differs from the standard kit's by more than 500 Bp. `minions` lists doc 13 templates spawned and bound at run start (Beastbinder: `["npc.companion.<origin_suffix>"]`, resolved with the Erratum's Origin suffix). `consumable_entries` lists entry indices removed when `gp.kit_consumables = false` (Grade 10+, §7.1) or `run.mut.bare_kit` is active; `run.mut.bare_kit` also removes every armor entry. Consumable entries are exactly the entries whose template is in `cat.potion`, `cat.throwable`, `cat.text` or `cat.food`; thrown weapons (`arch.ranged.thrown`, `cat.weapon_ranged`), ammunition, tools, books, gadgets, materials and containers are never consumable entries. The validator also rejects an entry above its stack or carry cap and an ammunition entry whose family no weapon of the kit fires. The universal kit is never removed.

### 2.5 Mid-run joiners

A player may join a co-op run only at a Folio entry (Canon §2.5 co-op save point). The joiner creates or picks an Erratum, then:

| Quantity | Value |
|---|---|
| Character level | `join_level = max(1, floor_div(Σ levels of Active slots, n_active) - 1)`; XP set to the cumulative XP of that level (Canon §8); allocation points for levels 2..=join_level are spent in the join screen (doc 04) |
| Calling major skills | `20 + min(30, 2 × (d - 1))`, with `d = d(f)` (§3.1) |
| Calling minor skills | `10 + min(15, d - 1)` |
| Kit | universal + chosen kit at `ilvl = d`, translated against the Folio-`f` genre, uids from Folio `f`'s counter |
| Lucre | the universal kit's `mul_bp(50, P(d))` (Calling kits carry no Lucre) |
| Score presence | `folios_present` starts at 1 |
| Glyph Library | the joiner's 0..=2 picks are learned at the join, as in §2.2 step 9 (§5.7) |
| Satchel | the joiner's Satchel materializes at the listed Folios at or after the join Folio (doc 15 §5.5) |

Example: party levels 9, 11, 12 joining Standard Folio 8 (`d = 8`): `floor_div(32, 3) - 1 = 9`; majors `20 + 14 = 34`; minors `10 + 7 = 17`; Lucre `mul_bp(50, 18280) = 91`. The same party joining Abridged Folio 5 (`d = d_eff = 8`) gets the same values.

---

## 3. Folio flow

### 3.1 Folio positions

Every structure maps the Folio index `f` (which drives `EntityId`, uids and per-Folio streams, Canon §1.11, §3.2) to a Volume `v(f)`, a structural position `pos(f)` and a depth `d(f)`. The depth feeds `P`, `T`, `H`, `S`, `ilvl`, `enemy_level` and every depth-scaled amount in this chapter.

| Structure | `v(f)` | `pos(f)` | `d(f)` |
|---|---|---|---|
| Standard, Novella, Daily, Endless (every `f` outside Second Edition) | `floor_div(f - 1, 5) + 1` | `f - 5 × (v - 1)` | `f` |
| Abridged (`f ∈ 1..=15`, Canon §17.1) | `floor_div(f - 1, 3) + 1` | `[1, 4, 5][j - 1]` with `j = f - 3 × (v - 1)` | `d_eff = 5 × (v - 1) + [1, 3, 5][j - 1]` |
| Second Edition (`f = f_ur + n`, `n = 1..=15`, §8.9) | `6 + floor_div(n - 1, 5)` | `n - 5 × floor_div(n - 1, 5)` | `25 + 2 × n` (DECISIONS 106) |

Abridged depths for `f = 1..=15` are 1, 3, 5, 6, 8, 10, 11, 13, 15, 16, 18, 20, 21, 23, 25. The second Folio of each Abridged Volume has `pos = 4` (Binding exits and an Interleaf) at the depth of Standard position 3; its boss Folios are `f = 3, 6, 9, 12, 15`, with `boss.editor_v1` to `boss.editor_v4` and `boss.ur_editor`.

| pos | Kind | Rubric (§3.4) | Keeper rank | Marginal notes | Rest rooms |
|---|---|---|---|---|---|
| 1, 2 | Normal | drawn | `rank.elite` | 2 | Colophon |
| 3 | Normal | drawn | `rank.champion` | 2 | Colophon |
| 4 | Normal, Binding exits | drawn | `rank.champion` | 2 | Interleaf, Colophon |
| 5 | Boss (Editor of Volume `v`) | Editor kill | none | 0 | Endpaper after the kill, then Colophon |

A Normal Folio also has doc 12's **Folio type** (`folio_type()`, doc 12 §6.1): `lattice.folio.standard`, `lattice.folio.challenge` or `lattice.folio.rest`. Standard and challenge Folios follow the table. A **rest** Folio has no Rubric, no Keeper, no marginal notes and no Interleaf: its Colophon entrance is open from entry, it holds no ambient spawns (doc 12 §6.2), and Erasure still accrues until the party enters the Colophon. A challenge Folio's challenge (doc 12 §6.3) runs beside its Rubric, and its reward is doc 12's.

Every Folio ends in a Colophon (`room.colophon`, Canon §17.1) placed before its exit doors. Doc 12's "Threshold" room and lectern are this Colophon and its lectern.

`f == final_folio` makes the pos-5 Folio **Final**: its Endpaper leads to victory (§3.2). Folio 25 of Standard and Folio 15 of Abridged (both `d = 25`) are always `genre.ur_text` with `boss.ur_editor` (Canon §17.1). Marginalia Folios (`f >= 26` after a Margins door, §9.1) follow the same table; their Keepers are `rank.champion` at every position, and every Marginalia Folio with `f % 5 == 0` is a boss Folio (Canon §17.1 `BOSS_FOLIOS`, §9.1).

### 3.2 Folio phase state machine

| From | To | Trigger (evaluated by the run layer at the end of each tick, rows in this order) |
|---|---|---|
| any | `Ended(erased)` | no Active Erratum is alive and not `st.downed`; every downed Erratum is killed first (`ev.entity_killed`). Exception: when the invader of a `mode.invasion_open` run reduces the last Erratum that is alive and not downed to 0 HP, the party is Redacted instead (§3.10, Canon §17.9, doc 16 §9.13) and the phase does not change |
| any | `Ended(abandoned)` | host issues Abandon (§4.3) |
| (entry, Normal standard or challenge) | `Exploring` | Folio entry (§3.3) |
| (entry, Normal rest) | `ColophonOpen` | Folio entry; a rest Folio has no Rubric, so its Colophon entrance is open from entry |
| `Exploring` | `ColophonOpen` | Rubric complete (§3.4); the Colophon entrance unseals on this tick |
| `ColophonOpen` | `InColophon` | an Active, living Erratum's origin is inside `room.colophon`. On this tick `colophon_entry_tick` is set, so Erasure accrual stops (§3.8), and the Colophon services open (§3.9) |
| `InColophon` | `ExitVote` | every Folio except the Final Folio of `mode.standard` or `mode.abridged`: an Active, living Erratum first interacts with the Colophon lectern. On this tick the lattice draw happens (doc 12 §5.5), the doors light and the vote timers start (§3.6); in a Marginalia boss Folio a "Close the Book" door is appended after the exit doors |
| `InColophon` (Final, `mode.standard` or `mode.abridged`) | `VictoryChoice` | the same trigger and effects; the doors are the Margins doors (§9.1, Standard only), then one "Second Edition" door (§8.9), then one "Close the Book" door |
| `ExitVote` | next Folio entry | an exit door commits (§3.6) |
| `ExitVote` (Marginalia boss) | `Ended(margin_closed)` | the "Close the Book" door commits |
| `ExitVote` (Folio `MARGINALIA_MAX_FOLIO`) | `Ended(margin_exhausted)` | its only door, "Close the Book", commits; that Folio offers no exit doors |
| (entry, Boss or Final) | `BossAntechamber` | Folio entry |
| `BossAntechamber` | `BossFight` | every Active, living, non-downed Erratum is inside the arena seal volume for `TURN_HOLD_TICKS` consecutive ticks |
| `BossFight` | `Endpaper` | the Editor is killed; the Erasure clock starts (§3.8) |
| `Endpaper` (Boss, including Marginalia boss) | `ColophonOpen` | every Active Erratum committed its Volume reward, or `VOLUME_REWARD_TIMEOUT_TICKS` passed since the Endpaper began; the Colophon entrance unseals on this tick |
| `Endpaper` (Final, `mode.standard` or `mode.abridged`) | `ColophonOpen` | same condition |
| `Endpaper` (Final, any other mode) | `Ended(victory)` | same condition |
| `VictoryChoice` | `Ended(victory)` | the "Close the Book" door commits (§3.6 rule) |
| `VictoryChoice` | Folio 26 entry | a Margins door commits; that door's genre is Folio 26's; `mode` becomes `mode.endless` (§9.1) |
| `VictoryChoice` | Second Edition Folio 1 entry | the "Second Edition" door commits; `mode` becomes `mode.second_edition` (§8.9) |

Separating Colophon entry from the lectern lets the party heal, shop and take boons with the Erasure clock already stopped, then open the vote when ready; the vote's majority and single-voter fallbacks (§3.6) still bound how long one idle player can hold the party.

`Ended(abandoned_version)` is applied outside the simulation by the meta layer (§12.2). The run layer is doc 03 §2.2's system 34a `sys.run_layer`, evaluated once per tick after `sys.despawn` and before `sys.tick_end`, so a transition on tick `t` sees every death, pickup and block change of tick `t`.

### 3.3 Arrival

```rust
fn enter_folio(run: &mut RunState, sim: &mut Sim, f: u32, genre: GenreId) {
    // Runs at the boundary before folio_entry_tick (doc 03 §2.1); t0 = folio_entry_tick.
    let t0 = sim.next_tick();
    run.folio_index = f;
    let (d, v) = (run.depth(f), run.volume(f));    // §3.1
    sim.reset_folio_counters(f);                   // Canon §1.11 EntityId and uid counters
    run.chaos = run.chaos_next.take();             // §8.1 audience Chaos mutator, this Folio only; None without the overlay
    let layout = doc12::generate(run.run_seed, f, d, genre, bleed_inputs(run), run.folio_scale_bp(), &run.gp_folio());   // gp, merged with chaos (§7.2)
    apply_session_intents(run, sim, f, &layout);   // §8.8: pending SlotJoin and SlotResume, ascending slot (doc 16 §9.1)
    for slot in run.party.values_mut() {           // ascending slot
        if slot.status == SlotStatus::Ghost {
            resurrect_ghost(sim, slot, f, t0);     // §3.10, Canon §17.6; status becomes Active
        }
        if matches!(slot.status, SlotStatus::Active | SlotStatus::Ghost) { slot.folios_present += 1; }
        slot.boon = slot.boon_next.take();         // §3.9: a boon lasts exactly one Folio
    }
    if run.pos(f) == 1 { doc15::materialize_satchels(run, sim, f); }   // doc 15 §5.5, after Ghost restoration
    // Canon effects of Folio entry: st.erasure removed, Death's Door reset, Shock of the New cleared,
    // Contradiction re-evaluated (Canon §5.5, §5.6, §10.8, §11.6).
    place_errata_at_incipit(sim, &layout);          // spawn points in ascending slot order
    apply_boon_entry_effects(run, sim);             // §3.9, ascending slot
    let mut rr = Rng::from_seed64(stream_seed(run.run_seed, "rng.run", f as u64));
    if run.mode == MODE_ENDLESS && (f - 26) % 5 == 0 { draw_marginal_mutation(run, &mut rr); }   // §9.3, first; never in Second Edition
    let notes = draw_notes(&layout, &mut rr);       // §3.5, second
    run.cur = FolioRun::new(f, &layout, notes, t0);
    run.history.push(FolioRecord::open(f, d, v, genre, &layout, t0, run.party_size_active()));
    run.phase = if layout.kind == FolioKind::Normal { FolioPhase::Exploring } else { FolioPhase::BossAntechamber };
    if run.colophon_marked { doc13::queue_colophon_hunter(&layout, run.colophon_hunter_kills); }   // §3.9
    save_folio_entry(run, sim);                     // §12; the co-op save point, at this boundary
    // doc 03 sys.tick_begin emits ev.folio_entered on tick t0.
}
```

The Incipit (`room.incipit`, doc 12) is a sanctuary: no spawn point lies within 24,000 mm of it and hostile AI never paths into it. The client shows the Folio title card for 180 ticks; the simulation does not pause. Party size `n` for §17.2 scaling is the number of Active slots after joins, resumes and Ghost resurrection. Draws at entry, in order: Ghost item loss (keyed `rng.loot`, 0 or 1 per resurrected Ghost, §3.10), then the per-Folio `rng.run` stream: 0 or 1 Marginal-mutation draw, then 0 or 2 note draws.

### 3.4 Rubric (exit unlock)

Every standard or challenge Folio has one Rubric. Doc 12 draws its type during generation with one `weighted([5000, 3000, 2000])` call, over the rows below in order, on its labeled worldgen sub-stream `stage_rng(run_seed, f, genre, "rubric", 0)` = `Rng::from_seed64(derive(derive(stream_seed(run_seed, "rng.worldgen", f), id32(genre) as u64), fnv1a64("rubric")))` (Canon §3.2 sub-streams, doc 12 §7.2). The sub-stream never advances `rng.worldgen`, so the exit preview can recompute it. Rest and boss Folios draw no Rubric. The Colophon's entrance, and with it the exit doors behind it, stays sealed until the Rubric completes: doc 12 sets the `protect` bits (doc 03 §4.7) on the entrance cells and clears them on the completion tick. Sealing the entrance, not only the doors, keeps a party from stopping the Erasure clock (§3.8) before it has earned the exit. A completed Rubric never reverts. Boss Folios have no Rubric; the Editor's death plays its role (§3.2).

| ID | Setup (doc 12, doc 13) | Complete when |
|---|---|---|
| `run.rubric.keeper` | one NPC flagged `keeper`, rank per §3.1, placed in the room with the greatest path distance from the Incipit | the Keeper's `ev.entity_killed` |
| `run.rubric.catchwords` | three `item.quest.catchword` placed in three distinct rooms, each at path distance ≥ 30,000 mm from the Incipit and ≥ 20,000 mm from each other | the party has picked up all three (any Erratum, any order); catchwords cannot be dropped |
| `run.rubric.seal` | a Seal point with a 4,000 mm radius; doc 13 spawns a wave on hold ticks 0, 600 and 1200 | the Seal accumulates 1800 held ticks; a tick is held when at least one Active, living Erratum's origin is within 4,000 mm of the Seal center and no hostile NPC's origin is |

### 3.5 Marginal notes (optional objectives)

At entry of a Normal standard or challenge Folio (never a rest or boss Folio), eligible notes (sorted by content ID) are collected. If more than 2 are eligible, two are drawn from the per-Folio `rng.run` stream: `i1 = range(|E|)`, remove, `i2 = range(|E| - 1)` (2 draws); otherwise all eligible notes are offered (0 draws). Each completed note grants 200 score (§4.2) and `mul_bp(mul_bp(30, P(d(f))), gp.lucre_bp)` Lucre to each Active Erratum, paid at exit commit. Example: Folio 10 at Grade 0 pays `mul_bp(30, 21718) = 65` Lucre per note.

| ID | Eligible when | Completed when (checked at exit commit) |
|---|---|---|
| `run.note.cartographer` | always | `rooms_visited == rooms_total` |
| `run.note.excavate` | always | `fac.errata` members broke at least 20 destructible cells (`block.tag.destructible`, resource nodes included, Canon §4.3) in this Folio, not counting cells an Erratum placed (doc 03 `is_errata_placed(c)`) |
| `run.note.illuminate` | the layout has an Illumination vault | the vault was opened |
| `run.note.native` | always | every Active Erratum's equipped components all have `C = 0` |
| `run.note.purge` | at least one elite or champion was placed | every elite- or champion-rank NPC spawned in this Folio is dead |
| `run.note.rescue` | the layout has a rescue event | the stranded native is alive inside `room.colophon` |
| `run.note.scribe` | always (every Colophon holds a Scriptorium station, Canon §17.1) | at least one Rewrite was committed in this Folio, at a Scriptorium room or at the Colophon's station |
| `run.note.silent` | always | doc 09's `folio_alerted` was still false on the tick the Rubric completed: no hostile AI had reached `aware.alert` toward any Erratum in this Folio |
| `run.note.swift` | always | `commit_tick - entry_tick <= 14400` (240 s) |
| `run.note.unbroken` | always | no `ev.player_downed` and no Erratum `ev.entity_killed` in this Folio |

### 3.6 Exits and exit choice

1. The draw: on the `InColophon → ExitVote` (or `→ VictoryChoice`) tick, which is the first lectern interaction and the tick the exit UI opens, doc 12 draws the 3 lattice candidates from `rng.lattice` (Canon §17.1, §3.2; doc 12 §5.5). `EXIT_OFFERS` is evaluated on that tick: 3 if any party member (Active or Ghost) has effective ITU ≥ 70 or carries `item.trinket.lattice_lens`, else 2. The first `EXIT_OFFERS` candidates become doors `0..EXIT_OFFERS`.
2. Override: when the next Folio has depth 25 (Standard Folio 24, Abridged Folio 14), the draw still happens (stream consistency) and the doors are replaced by one door to `genre.ur_text`. That draw is exempt from the Volume no-repeat rule and the per-run cap (DECISIONS 115, doc 12 §5.4), so it always has 3 eligible candidates; no player sees them.
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

fn exit_commit(st: &mut VoteState, t: u64, doors: &[Door], voters: &[Entity], living: &[Entity],
               colophon: &Volume) -> Option<usize> {
    // voters: Active, alive, not st.downed Errata in ascending EntityId (never empty, §3.2 row 1)
    // living: Active, alive Errata, downed included (the interact-hold path's "every living Erratum")
    let n = voters.len();
    let since = t - st.open_tick;
    let need = if since >= EXIT_ANY_TICKS { 1 }
               else if since >= EXIT_MAJORITY_TICKS { n / 2 + 1 }
               else { n };
    let all_inside = living.iter().all(|e| colophon.contains_origin(e));
    for d in 0..doors.len() {
        let k = voters.iter().filter(|e| vote(e, doors, st, colophon) == Some(d)).count();
        st.hold[d] = if k >= need { st.hold[d] + 1 } else { 0 };
        // interact-hold path (doc 12 defers to this section): one voter holds interact on door d while every living Erratum is inside
        st.interact_hold[d] = if all_inside && st.interacting_on(d) { st.interact_hold[d] + 1 } else { 0 };
    }
    (0..doors.len()).find(|&d| st.hold[d] >= TURN_HOLD_TICKS || st.interact_hold[d] >= 180)   // lowest index wins a tie
}
```

On commit at tick `t`, in this order: marginal notes are evaluated and paid (`ev.note_completed`, §3.11); each Ghost slot's `remains_carried` is set when its Remains item is in the inventory of an Active Erratum (§3.10); in Marginalia each Erratum receives 1 Emendation (Canon §13.7); `exit_tick = t`; `ev.exit_committed` and then `ev.folio_exited` are emitted; every slot (Active, downed, Ghost) leaves together, with tamed beasts and hirelings within 16 m (Canon §15.1). The next Folio is generated and entered on the next tick boundary; transit consumes no simulation ticks.

Example: a 2-player party, one player idle outside every door. Before tick `open + 3600`, `need = 2`, no commit. From `open + 3600`, `need = 2/2 + 1 = 2`, still none. From `open + 7200`, `need = 1`; the active player standing in door 1 commits at `open + 7200 + 119` (hold reaches 120 on that tick, counting the first qualifying tick as 1).

### 3.7 Boss gates and boss Folios

- Exits of a pos-4 Folio are Binding doors; the chosen genre skins the boss Folio. The Interleaf of that Folio is the last rest before the Editor.
- A boss Folio's Incipit is the antechamber, a sanctuary. The arena seal opens per §3.2; the arena door then closes behind the party until the Editor dies (doc 12).
- Editors: `boss.editor_v1` to `boss.editor_v4` at Folios 5 to 20, `boss.ur_editor` at Folio 25 (doc 13). Abridged places them at Folios 3, 6, 9, 12 and 15 (§3.1). Novella's and the Weekly Gauntlet's final boss is `boss.editor_v3` at Folio 15; the Daily's is `boss.editor_v1` at Folio 5. Marginalia rotation is in §9.1; Second Edition's revised Editors and the Author are in §8.9.
- From boss-Folio entry until the Editor's death, the Erasure clock and satiation decay do not advance (Canon §11.6).
- Every hit an Erratum takes from an Editor attack that doc 13 flags `telegraphed` increments `telegraphed_hits` (§4.2).

### 3.8 Erasure pressure timeline

The Erasure clock is `c = t - clock_start`, where `clock_start` is the entry tick in Normal Folios and the Editor's death tick in boss Folios (Canon §11.6: accrual pauses while the boss lives). From `ERASURE_ONSET` (720 s, Canon §17.1), stack `k` is granted at `c = 43200 + 1800 × (k - 1)` for `k = 1..=10` (`ERASURE_STEP`) to every Active, living Erratum, on the same tick, until the Folio's `colophon_entry_tick` is set: from that tick no further stack is granted in the Folio (Canon §17.1). `st.erasure` holds at most 10 stacks (Canon §11.1), so the tenth stack at `c = 59400` (990 s) is the last. Stacks already held stay with their effects until the next Folio entry. Healing received at `k` stacks is × `10000 - 300 × k` (Canon §11.6). Doc 13's `sys.erasure` (doc 03 §2.2, system 32) grants the stacks; it reads `clock_start` and `colophon_entry_tick` from `RunState.cur`.

**The Erasure hunter.** Each Erratum's threshold is `thr = max(6, 10 - red_ink_doses)` (Canon §11.6, §16.4). Exactly one Erasure hunter spawns for the party, with `ev.hunter_spawned`, on the tick the first Erratum's stack count reaches its `thr`; no further Erasure hunter spawns in that Folio, whatever happens to the first (Canon §11.6, DECISIONS 26). It carries `rank.hunter` for identity, immunities and presentation and uses `rank.elite`'s multipliers (Canon §11.6, DECISIONS 91, doc 13 §10.5). Every rank test in this chapter treats it as `rank.hunter`: the hunter-scoped factors of §7.3, §8.7 and §9.3 (`gp.hunter_hp_bp`, `run.mut.hungry_hunters`, `run.marg.hunters_pack`), `run.boon.hunters_bane`, `S_hunters`, `ach.hunter`, `ach.hunter_10` and the `hunters` Recollection step. Because stacks stop at Colophon entry, a party that enters the Colophon before any Erratum reaches its threshold draws no hunter in that Folio; a hunter that has already spawned stays until it dies or the party exits. Example: an Erratum with 4 doses has `thr = 6`, so the hunter arrives at `c = 43200 + 1800 × 5 = 52200` (870 s).

| Clock (s) | Clock (ticks) | Event | Healing received |
|---|---|---|---|
| 0 | 0 | entry; stacks cleared | ×10000 |
| 240 | 14400 | `run.note.swift` deadline | ×10000 |
| 360 | 21600 | speed points reach 0 (§4.2) | ×10000 |
| 600 | 36000 | UI cue 1: margins darken (client only) | ×10000 |
| 690 | 41400 | UI cue 2: Red Ink audio sting (client only) | ×10000 |
| 720 | 43200 | stack 1 | ×9700 |
| 750 | 45000 | stack 2 | ×9400 |
| 780 | 46800 | stack 3 | ×9100 |
| 810 | 48600 | stack 4 | ×8800 |
| 840 | 50400 | stack 5 | ×8500 |
| 870 | 52200 | stack 6 | ×8200 |
| 900 | 54000 | stack 7 | ×7900 |
| 930 | 55800 | stack 8 | ×7600 |
| 960 | 57600 | stack 9 | ×7300 |
| 990 | 59400 | stack 10, the cap; the Erasure hunter (Canon §11.6) | ×7000 |

The table assumes no Red Ink doses and no Colophon entry before 990 s. All Errata gain stacks on the same tick, so all reach 10 together. Ghosts gain none. Example: a party that enters the Colophon at `c = 48000` holds 3 stacks (stack 3 at 46,800; stack 4 would come at 48,600) and keeps 3 until it exits; it never draws the hunter in that Folio.

### 3.9 Rest rooms

Three room kinds restore the party: the **Colophon** in every Folio (Canon §17.1), the **Interleaf** mid-way through every pos-4 Folio, and the **Endpaper** after every Editor kill.

**Colophon** (`room.colophon`, every Folio, before the exit doors). The entrance unseals per §3.2. The services open on the tick the first Active, living Erratum's origin enters the room (`colophon_entry_tick`):

| Service | Rule |
|---|---|
| Full heal | Each Active Erratum may claim it once per Colophon by interacting with the Hearth (no channel): HP set to Max HP and aether set to Max aether. It is a set, not healing, so Canon §10.9 multipliers and `st.erasure` do not reduce it. Grade modifiers never touch it. |
| Shop | doc 11 stock from `rng.loot_place`; buy prices × `gp.shop_price_bp` after Canon §6.4 row 37. Absent in `genre.unwritten` Folios, which have no shops (Canon §5.2, §17.1). |
| Boon altar | below; 3 boons and 1 reroll per Erratum (Canon §17.1) |
| Scriptorium station | Rewrite and identification services (doc 11, doc 10) |
| Going Native | Canon §5.11: once per run per Erratum (`going_native_used`), sets the Erratum's Origin to the Folio's genre and recomputes tolerance |
| Unmoored | `st.unmoored` ends when the bearer's origin enters the room (Canon §5.11) |
| Lectern | the first interaction by an Active, living Erratum opens the exit vote and the lattice draw (§3.2, doc 12 §5.5); later interactions show the doors again |

**Boon altar.** Each Active Erratum gets one stored offer when the services open, built from its commit-only stream `rng.offer.p<slot>` through the Canon §3.5 offer generator, which doc 11 §11.2 implements as `offer_rng(slot, context_uid, purpose)` and which peeks the stream without advancing it: `o = offer_rng(slot, f as u64, "boon")`, then `o.range(12)`, `o.range(11)`, `o.range(10)` over the 12 boons below in ascending ID order without replacement (3 draws on `o`). A **reroll** (at most `BOON_REROLLS = 1` per Colophon) advances `rng.offer.p<slot>` by exactly one `next_u64` and stores a new offer built the same way from the new peek. A **commit** advances the stream by exactly one `next_u64`, sets `boon_next` and emits `ev.boon_committed`. The altar is a room fixture with no uid or EntityId, so its `context_uid` is the Folio index `f`, which names the one altar of the Folio (Canon gaps, item 1). An offer still uncommitted at exit commit is discarded without advancing the stream. These are the same advance-on-commit rules doc 11 uses for Engine faces, Rewrites and reforges. Ghosts get no offer. At the next Folio entry `boon_next` becomes `boon`, which lasts until that Folio's exit commit or the run's end; a boon never carries further.

| ID | Effect while active (one Folio) |
|---|---|
| `run.boon.appraising_eye` | every item the bearer picks up rises to at least `know.appraised` on pickup (doc 10); no draw |
| `run.boon.blessing` | `st.blessed` applied directly to the bearer at Folio entry and again at Rubric completion or the Editor's death |
| `run.boon.deep_pockets` | every fresh Lucre pile the bearer picks up × 12500 (doc 11 §9.1), applied by doc 11 after `gp.lucre_bp`; sales and dropped piles are excluded |
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

**Desecration** (Canon §17.1). The first `ev.block_changed` that breaks or transforms a cell inside a `room.colophon` volume, caused by a `fac.errata` member (Errata, minions, and their projectiles and explosions), sets `colophon_marked = true` for the rest of the run and emits `ev.colophon_desecrated`. In every later Folio, doc 13 spawns one `rank.hunter` (`enemy.editors.revisor`, doc 13 §10.5) inside the Colophon at Folio entry; it idles until an Erratum's origin enters the Colophon or it takes damage. Each such hunter's `ev.entity_killed` increments `colophon_hunter_kills`. Once `colophon_hunter_kills >= COLOPHON_ELITE_AFTER_KILLS` (3), the waiting hunter spawns with one doc 13 elite modifier. Colophon hunters are not Erasure hunters: none of them is the §3.8 hunter, and none counts toward `S_hunters` (§4.2).

**Interleaf** (`room.interleaf`, in every pos-4 standard or challenge Folio, including Abridged and Marginalia; a pos-4 rest Folio is itself the rest). A sanctuary room with a Hearth and a vendor (doc 12), placed on the critical path from the Incipit to the Colophon entrance at a path distance of `INTERLEAF_PATH_BP` (4000 to 6000 Bp) of that path's length, so it is a mid-Folio rest and not a second Colophon. Vendor prices × `gp.shop_price_bp`. Each Active Erratum may Rest once per Interleaf: a channel of `REST_CHANNEL_TICKS` that cancels if the Erratum takes damage, attacks, casts, or its origin moves more than 500 mm from where the channel began. On completion:

1. Heal `h = mul_bp(mul_bp(max_hp_mhp, INTERLEAF_HEAL_BP), gp.rest_heal_bp)`, applied as healing through Canon §10.9 (row 3, then status multipliers in ascending ID order per Canon §11.7, then the Max HP cap).
2. Stamina and aether set to max. Composure += `mul_bp(max_composure_m, 5000)`, capped at max.
3. Remove `st.bleeding`, `st.burning`, `st.poisoned`, `st.corroded`, `st.withering`, `st.diseased`.

Example: Max HP 300 (300,000 mHP) at Grade 6 (`rest_heal_bp = 7500`), RES 40, 3 Erasure stacks: `mul_bp(300000, 5000) = 150000`, `mul_bp(150000, 7500) = 112500`, row 3 ×10000 = 112,500, `st.erasure` ×9100 = 102,375 mHP healed.

**Endpaper** (every boss Folio, from the Editor's death tick):

1. Each Active Erratum: HP and composure set to `max(cur, mul_bp(max, gp.rest_heal_bp))` (a set, not healing); stamina and aether set to max; every status of category DoT, debuff, control or mental is removed; satiation `+= ENDPAPER_SATIATION`, capped at 2400.
2. Ghosts stay Ghosts; they resurrect at the next Folio entry (Canon §17.6, §3.10).
3. Volume reward: each Active Erratum receives one commit-only offer of 3 items from `rng.offer.p<slot>` (Canon §3.5), built by doc 11's `loot.volume_reward` table for Volume `v` through `offer_rng(slot, editor EntityId, "volume_reward")` (doc 11 §11.2; Palimpsestic items are eligible there). The player commits one. At `VOLUME_REWARD_TIMEOUT_TICKS` an uncommitted offer commits index 0.
4. Account Reliquary grants per doc 15 are recorded on the run for payout at run end (§4.5).
5. When step 3 resolves for every Active Erratum, the Colophon entrance unseals (§3.2).

### 3.10 Death, Ghosts and resurrection

- **Solo.** The Erratum's `ev.entity_killed` ends the run `run.end.erased` at the end of that tick (Canon §10.8 step 3, §17.6).
- **Co-op death.** Downing and revival follow Canon §10.8 and doc 16. When an Erratum dies while another Erratum is alive, it becomes a ghost with `st.ghost`, `ev.player_died` and `ev.player_ghosted` are emitted (Canon §10.8, §17.6), and its slot becomes `Ghost`. The body drops Remains: one `cat.corpse` item (doc 11 template `item.remains`) of 75,000 g, with a uid from the current Folio's counter and the ghost's slot recorded on it. The ghost keeps its equipment and bag, unusable while it is a ghost; the Remains hold no items. The ghost moves per doc 03 §5.12, cannot deal damage, pick up items or be perceived, and can open doors, press switches, ping and apply `st.marked` through doc 17's `MarkTarget` intent (Canon §17.6). It shares kill XP like every member present in the Folio (Canon §8: downed members and ghosts included, Dormant and Departed slots excluded; doc 04 §6.2), so a dead player does not fall a level behind; its other XP follows doc 04 §6.2. It gains no instanced loot, and its needs are frozen (doc 04). The current Folio's `erratum_deaths` increments.
- **Resurrection** happens at the next `ev.folio_entered` and never earlier (Canon §17.6). At exit commit, `remains_carried` is true when the slot's Remains item is in an Active Erratum's inventory. At the boundary before `folio_entry_tick = t0` of Folio `f`, `resurrect_ghost` returns the entity with its original EntityId at the Incipit:
  1. HP `mul_bp(max_hp_mhp, GHOST_REMAINS_HP_BP)` when `remains_carried`, else `mul_bp(max_hp_mhp, GHOST_BARE_HP_BP)`.
  2. When `remains_carried` is false: `items` = the ghost's equipped items outside `slot.weapon_1..=3`, sorted by uid. If `items` is non-empty, `i = keyed_rng("rng.loot", f, ghost_entity, t0, 0).range(len(items))` (1 draw) and `items[i]` is destroyed; if it is empty, no draw happens.
  3. Stamina and aether full, composure `mul_bp(max, 5000)`, every status removed including `st.ghost`; the slot becomes `Active`; `ev.player_resurrected` is emitted on tick `t0`.
  4. The Remains item is destroyed wherever it is.
- A Ghost counts toward `folios_present` and toward `EXIT_OFFERS` (§3.6), and is never a voter. A run that ends while a slot is a Ghost counts that player as present for every achievement and milestone.

Example: a 2-player run; slot 1 dies in Folio 7 with five equipped non-weapon items. Slot 0 leaves the Remains behind. At Folio 8 entry `t0`, slot 1 returns with 25% Max HP, and `range(5)` on `keyed_rng("rng.loot", 8, slot1_entity, t0, 0)` picks the destroyed item.

**Redaction** (Canon §17.9, `mode.invasion_open` only). When the invader's hit would bring the last Erratum that is alive and not downed to 0 HP, the run layer resolves the Redaction on that tick `t` of Folio `f` instead of Canon §10.8 steps 2 and 3 (doc 16 §9.13). For every Active or Ghost slot in ascending slot order:

1. Its Erratum's purse is set to 0 Lucre.
2. `items` = its equipped items outside `slot.weapon_1..=3`, sorted by uid. If `items` is non-empty, `items[keyed_rng("rng.loot", f, entity, t, 0).range(len(items))]` is destroyed (1 draw); otherwise no draw happens.
3. An Active slot's Erratum loses every status except `st.erasure` (`st.downed` ends without `ev.player_revived`), its HP is set to `mul_bp(max_hp_mhp, 5000)`, and it is placed at the Incipit spawn points in ascending slot order. A Ghost slot stays a Ghost and resurrects only at the next Folio entry (Canon §17.6); its Remains stay where they lie.

The invasion then ends with reason 3 (`ev.invasion_ended`, doc 16 §9.13) and `FolioRecord.invasion = 2`. The phase, the Erasure clock and stacks, the Rubric and the score accumulators are unchanged.

### 3.11 Run-layer events

Doc 03 §14.3 registers these events with their payloads (Canon §18). `sys.run_layer` emits them on the tick named; several on one tick are emitted in table order.

| Event | Emitted |
|---|---|
| `ev.boss_engaged` | on the `BossAntechamber → BossFight` tick (§3.2), once per boss Folio |
| `ev.volume_completed` | on the tick a boss Folio's Editor dies (`BossFight → Endpaper`), with `volume = v(f)` |
| `ev.rubric_completed` | on the tick a standard or challenge Folio's Rubric completes (§3.4) |
| `ev.colophon_entered` | on every tick an Active, living Erratum's origin enters `room.colophon` from outside it; `first` is true for the entry that sets `colophon_entry_tick` |
| `ev.colophon_desecrated` | on the tick `colophon_marked` becomes true (§3.9), once per run |
| `ev.boon_committed` | on each boon commit (§3.9) |
| `ev.note_completed` | at exit commit, one per completed note and Active slot, in ascending note ID and then slot (§3.5) |
| `ev.exit_committed` | at exit commit, after the notes and before `ev.folio_exited` (§3.6) |

---

## 4. Score, run end and payout

### 4.1 Accumulators

All score inputs are simulation state, so a replay reproduces the score exactly (Canon §2.4).

- `FolioRecord` fields (§1.2), updated by the run layer from events.
- `lucre_norm` per slot: on every Lucre gain of `a` units by that slot in Folio `f`, `lucre_norm += floor_div(a × 10000, P(d(f)))`. Example: 120 Lucre gained in Standard Folio 10 adds `floor_div(1200000, 21718) = 55`. Lucre from selling items counts; Lucre picked up from a dropped pile does not (Lucre is per Erratum and passes between party members only as dropped piles, Canon §14, doc 11 §9.1).
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
| `run.end.victory` of a Second Edition run (the Author's death, §8.9) | 17500 |
| `run.end.victory` of every other run (Novella, Weekly Gauntlet, Daily, Custom with `run.mut.short_book`) | 15000 |
| `run.end.margin_closed`, `run.end.margin_exhausted` | 17500 |
| `run.end.erased` or `run.end.abandoned` after a victory (Endless) | 15000 |
| any other end, including `run.end.abandoned_version` and every Second Edition end other than its victory (§8.9) | 10000 |

`mutator_bp` is the left-to-right `mul_bp` chain of each active mutator's `score_bp` in content-ID order, starting from 10000 (§8.7), then capped at `MUTATOR_BP_CAP = 20000`. The cap stops stacked hard mutators on the 10-Folio structure from outpaying a full run (§8.7). **Endless rule:** when a Margins door commits (§3.2), `score_at_victory` is computed with `victory_bp = 17500` and stored; the final score of an Endless run is `max(score_at_victory, score_now)`. **Second Edition rule:** the "Second Edition" door stores `score_at_victory` the same way; a Second Edition victory scores `max(score_at_victory, score_now)`, and every other Second Edition end discards it and scores `score_now` with `victory_bp = 10000` (§8.9).

`victory_bp = 17500` for the Ur-Text and 10000 for a failed run is calibrated with `RUN_SCORE_BP` (§4.6) so doc 15's two reference runs land on its `run_score` targets.

Score component IDs (the `RunSummary.components` keys and the UI breakdown, in display order):

| ID | Term | Sign |
|---|---|---|
| `score.progress` | `S_progress` | ≥ 0 |
| `score.editor` | `S_editor` | ≥ 0 |
| `score.speed` | `S_speed` | ≥ 0 |
| `score.explore` | `S_explore` | ≥ 0 |
| `score.notes` | `S_notes` | ≥ 0 |
| `score.treasure` | `S_treasure` | ≥ 0, at most 8000 |
| `score.hunters` | `S_hunters` | ≥ 0 |
| `score.deaths` | `S_deaths` | ≤ 0 |

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
| `run_score` (§4.6, full presence) | 1,219 | 14,991 | 20,000 (cap; raw 28,641) | 7,013 | 7,966 | 3,033 |
| Quires (doc 15 §2.4) | 24 | 299 | 400 (cap) | 140 | 159 | 60 |

Example E's `S_progress` sums `600 + 40 × d_eff` over the 15 Abridged depths, whose sum is 195: `9000 + 7800 = 16800`. Quire rates per hour: B 299 in 125 min (143.5 per hour), D 140 in 60 min, E 159 in 67.5 min (141.3 per hour) and R 60 in 60 min. Abridged's `mode_quire_bp = 7500` brings its hourly rate in line with Standard; without it E would pay 212.

Example C editor term: `floor_div(21, 3) = 7`, so each Editor gives `1000 × v - 350` and the sum over `v = 1..=5` is `15000 - 1750 = 13250`.

### 4.3 Run end states

| ID | Trigger | Scope |
|---|---|---|
| `run.end.victory` | the Final Folio's Endpaper closes, or the "Close the Book" door of a `VictoryChoice` commits (§3.2) | party |
| `run.end.erased` | §3.2 row 1 | party |
| `run.end.abandoned` | solo: Abandon at any tick; co-op: host Abandon, confirmed by holding the menu button 120 ticks | party |
| `run.end.abandoned_version` | resume refused for a `sim_version` or `content_hash` mismatch and the player chooses Close the Book (§12.2, doc 02 §12.3) | party; applied by the meta layer from the save's checkpoint record |
| `run.end.margin_closed` | the "Close the Book" door commits in a Marginalia boss Folio's exit vote (§3.2) | party |
| `run.end.margin_exhausted` | the only door of Folio `MARGINALIA_MAX_FOLIO`, "Close the Book", commits | party |
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
    //   2. Achievement, Recollection and resident-arc counters (§10, §6, §5.6); Glyph Library merge (§5.7);
    //      Index stories and discovered World Laws (§5.5)
    //   3. Leaderboard submission if ranked (§9.4)
    //   4. Run history entry (last 200 runs kept): RunSummary with RunState.history (§4.7)
    //   5. Closing manuscript (§4.7) for the endings doc 18 §8.5a names, then the Margin summary screen
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
| Bestiary and Index knowledge, recorded Folio stories, discovered World Laws | §5.5 |
| Glyph Library | §5.7: every glyph any slot of this account learned in the run |
| Resident arc beats | §5.6 |
| Milestones, Grade unlocks, discovered genres, best scores, run history | §5.1, §7.4 |
| Erratum presets (Origin, Calling, kit, appearance, name) | Dramatis slots |
| Illuminated item skins | Canon §13.9: picking up an `illuminated` item unlocks its skin account-wide, in every mode |
| Trophy alcove arrangement | Canon §17.8, §5.1a; arranged in the Margin, never in a run |
| Echoes, Folds, Scripts, cosmetics | doc 15 |

Never kept: items, Lucre, character level, XP, skills, sub-attributes, talents, reputation, type knowledge and appearance mappings, Erasure state, boons, Going Native, the Colophon mark and lattice history. Apocrypha runs (simulation-affecting mods, doc 15) keep nothing in this table except run history.

### 4.6 `run_score` and Quire payout

Doc 15 owns the Quire earn rate (Canon §19). This chapter hands it one number per slot:

```
presence_bp(slot) = floor_div(10000 × slot.folios_present, run.history.len())
raw(slot)         = mul_bp(mul_bp(mul_bp(score, mode_quire_bp), presence_bp(slot)), RUN_SCORE_BP)
run_score(slot)   = min(raw(slot), RUN_SCORE_FOLIO_CAP × cleared_present(slot))
// cleared_present(slot): cleared Folios (§4.2) whose entry the slot was Active or Ghost at
q(slot)           = min(QUIRES_PER_RUN_CAP, floor_div(run_score(slot), 50))      // doc 15 §2.4, cap 400
grant(slot)       = max(0, q(slot) - slot.quires_paid)                           // §8.8 departed payouts
```

`mode_quire_bp` is in §8.1. **Calibration.** Doc 15 asks for a `run_score` of 3,000 for its reference failed run and 15,000 for a reference victory. With `victory_bp = 17500` and `RUN_SCORE_BP = 1460`, reference run R scores 20,780 and gives 3,033, and Example B scores 102,681 and gives 14,991; both are within 1.1% of the targets. **Per-Folio cap.** Doc 15's flat 400-Quire cap per run favours short structures once high Grades push every run to the cap: at Grade 20, victories in Standard, Abridged and Novella would pay 400 Quires each, which is 192, 356 and 400 per hour. `RUN_SCORE_FOLIO_CAP = 800` limits `run_score` to 20,000 for 25 cleared Folios (exactly doc 15's 400-Quire point) and to 12,000 for 15, so the same victories pay 400, 240 and 240 Quires (192, 213 and 240 per hour). No reference run reaches the cap: R is 3,033 against 9,600 and B is 14,991 against 20,000.

Examples: Example C at full presence: `raw = 28641`, capped to 20,000, 400 Quires. Example C for a player present in 18 of 25 Folios: `presence_bp = 7200`, `raw = mul_bp(mul_bp(mul_bp(196174, 10000), 7200), 1460) = 20621`, `cleared_present = 18`, so `run_score = min(20621, 14400) = 14400` and 288 Quires. Example D for a player present in 9 of 15 Folios: `presence_bp = 6000`, `raw = 4208`, below the cap of 7,200, so 84 Quires.

`RunSummary` (meta layer input, written into run history):

```json
{
  "run_seed": "0x0123456789abcdef",
  "run_id": "0x00000000075bcd15",
  "mode": "mode.standard",
  "grade": 10,
  "mutators": [],
  "end": "run.end.victory",
  "end_tick": 482400,
  "deepest_folio": 25,
  "party_size_max": 3,
  "coop_modes": [],
  "apocrypha": false,
  "mercy_ever_on": false,
  "min_game_speed_bp": 10000,
  "colophon_marked": false,
  "score": 196174,
  "score_at_victory": 0,
  "ironman": false,
  "components": { "score.progress": 28000, "score.editor": 13250, "score.speed": 3000,
                  "score.explore": 4000, "score.notes": 7200, "score.treasure": 3000,
                  "score.hunters": 600, "score.deaths": -3000 },
  "victory_bp": 17500,
  "grade_bp": 20000,
  "mutator_bp": 10000,
  "mode_quire_bp": 10000,
  "slot": { "index": 1, "calling": "calling.warden", "origin": "genre.wuxia", "echo": null,
            "kit": "run.kit.warden", "folios_present": 18, "presence_bp": 7200,
            "cleared_present": 18, "run_score": 14400, "quires_paid": 0, "active_ticks": 470000,
            "folio_exits_qualified": 17, "editor_kills_present": 4, "victory_present": 1,
            "kills_present": [ { "party_size": 3, "k_sum": 6480 } ], "types_identified": 9, "crafts": 4,
            "revives": 3, "codex_entries_new": [], "display_name": "Mei of the Ninth Bell",
            "status_at_end": "Active", "went_native": null, "red_ink_doses": 0, "top_patron": null,
            "appearance": { "preset": "art.preset.human.03", "base": 2, "marking": 5 },
            "glyphs_learned": ["glyph.form.aura"] },
  "ranked": false,
  "restores": 0
}
```

`run_id` is the §2.1 Commit value (doc 15 §10.4). `mercy_ever_on` is true when any slot had `mercy_on` on any tick and `min_game_speed_bp` is the lowest `access.game_speed_bp` of any tick (doc 17 §13.9); either one makes the run ineligible for boards (§9.4) and changes nothing else. The per-slot `victory_present`, `kills_present`, `types_identified`, `crafts`, `revives` and `codex_entries_new` are the counters doc 15 §6.2 defines, counted by the run layer for that slot; `active_ticks` follows doc 15 §2.2. `display_name`, `status_at_end`, `went_native`, `red_ink_doses`, `top_patron` and `appearance` (doc 18 §5.6, `null` for an Echo; §1.3) feed §4.7 and never the simulation. `glyphs_learned` lists, sorted by ID, every glyph the slot's Erratum learned in the run (§5.7). `ironman` is true only for a Second Edition victory with no `ev.player_died` in any Second Edition Folio (§8.9).

### 4.7 Closing manuscript inputs (procedural ending)

The Ur-Text ending is procedural (DECISIONS 46): doc 18 §8.5 and §8.5a select the ending and assemble a closing manuscript from the run's genres, the party's Echoes or Origins, its notable choices and its solved Folio stories, rendered as illustrated pages in the Ur-Text's style. This chapter records the inputs; the text never enters simulation state. The run history entry keeps `RunSummary` together with `RunState.history`, so the manuscript is rebuilt from the same data whenever it is opened.

| Input | Recorded as |
|---|---|
| genres in Folio order, Volumes, Folio kinds, Editors | `FolioRecord.genre` (and `fused` in Second Edition), `volume`, `kind` and `editor` (§1.2) |
| Folio stories | `FolioRecord.story`, doc 12 §20.5's record of template IDs and hero and rival name seeds, stored at Folio entry; `story_solved`, the tick doc 12 §20 marks the story solved (`ev.objective_completed` kind 2); `leaf_carried` |
| invasions | `FolioRecord.invasion`: 1 when the invader dies, 2 on a Redaction (§3.10), 3 when the invasion ends for any other doc 16 §9.13 reason (0, 2, 4, 5 or 6) |
| the party | per slot `echo`, `origin`, `calling` and `display_name` (the Echo's name, else the Erratum preset's name) |
| notable choices | per slot `went_native` (the genre of its Going Native, §3.9), `red_ink_doses` (doc 04 `afflict.red_ink` doses at run end) and `top_patron` (the patron with the highest favor above 250 at run end, doc 06, ties to the lower ID); per run `colophon_marked` |
| the end | `end`, `score_at_victory` and per slot `status_at_end` |
| the Second Edition layer (§8.9) | `mode = mode.second_edition`; the Second Edition Folios' `FolioRecord` entries (Folios `f_ur + 1` to `f_ur + 15`, Volumes 6 to 8) with each fused pair (`genre`, `fused`), each revised Editor (`editor`) and the Author's death as the Final Folio's `editor_killed`; `RunSummary.ironman` |
| the party's looks | per slot `appearance` (§1.3), `null` for an Echo |

The manuscript plays on the run-end screen, before the summary, for the endings doc 18 §8.5a lists after an Ur-Text victory. After a Second Edition victory doc 18 §8.5 shows two layers (§8.9, DECISIONS 100): first `ur_clear_ending`, the ending its Ur-Text victory earned, with that ending's manuscript built from the main-run records, then `nar.ending.second_edition`, built from the Second Edition row above, whose closing line is its Ironman line when `RunSummary.ironman` is true. Every other end, a forfeited Second Edition included, shows only its closing line and plates. It grants nothing.

---

## 5. The Margin and meta progression

The Margin is a walkable first-person 3D hub rendered by the client (DECISIONS 77). Its facilities (§5.1) are stations the player walks up to, its residents (§5.6) keep the facilities and run the vendors, and the account's trophy alcove (§5.1a) holds its trophies. Doc 17 owns the camera, with its third-person toggle, and the Margin index that opens any open facility without walking. The Margin runs no gameplay simulation and draws no simulation random numbers. Account data (balances, unlocks, counters) is stored locally, signed, and synced when online (doc 15 offline end-of-service guarantee).

### 5.1 Facilities and milestones

| ID | Name | Function | Opens at |
|---|---|---|---|
| `margin.lectern` | The Lectern | modes, Grade, mutators, seeds, run start (§2.1) | `margin.milestone.first_run_end` |
| `margin.dramatis` | The Dramatis | Erratum presets, Echo selection and trait variant, kit, lens pocket | `first_run_end` |
| `margin.index` | The Index | item encyclopedia (§5.5) | `first_run_end` |
| `margin.bestiary` | The Bestiary | enemy encyclopedia (§5.5) | `first_run_end` |
| `margin.first_page` | The First Page | replays nothing; unlocked by completing the first run's Folio 1 (§11) | `first_run_end` |
| `margin.bindery` | The Bindery | unlock tree, paid in Quires (§5.2) | `first_run_end` |
| `margin.ledger` | The Ledger | run history, shareable deterministic replays with the killcam (DECISIONS 48; doc 16 replay files, doc 17 viewer; no photo mode, DECISIONS 129) and leaderboards (§9.4) | `first_run_end` |
| `margin.wardrobe` | The Wardrobe | cosmetics and Bindings | `first_run_end` |
| `margin.inkwell` | The Inkwell | gacha paid in Ink (doc 15) | `first_run_end` |
| `margin.codex_desk` | The Codex Desk | battle pass (doc 15) | `first_run_end` |
| `margin.store` | The Store | fiat SKUs (doc 15) | `first_run_end` |
| `margin.reliquary_vault` | The Reliquary Vault | opens account Reliquaries (doc 15) | `first_reliquary` |
| `margin.vellum_archive` | The Vellum Archive | Vellum shop and the Transcription shelf (doc 15 §3.9.1) | `first_vellum` |
| `margin.recollections` | The Recollection Alcove | Echo trials (§6) | `first_editor_v1` |
| `margin.proving_page` | The Proving Page | sandbox: target dummies, every unlocked item at ilvl 1, no rewards | purchase of `unlock.facility.proving_page` |
| `margin.plot` | The Trophy Alcove | the account's trophy alcove: 32 fixed trophy and decoration slots (§5.1a, Canon §17.8); no building, no gameplay stats | `first_run_end` |

In the facility table and below, a short milestone name such as `first_run_end` stands for its full `margin.milestone.*` ID.

| Milestone ID | Set when |
|---|---|
| `margin.milestone.first_run_end` | any run of any mode ends |
| `margin.milestone.first_editor_v1` | the account's player is present when `boss.editor_v1` dies |
| `margin.milestone.first_editor_v2` | same for `boss.editor_v2` |
| `margin.milestone.first_victory` | `run.end.victory` in Standard, Abridged or Novella |
| `margin.milestone.first_ur_text_victory` | `run.end.victory` in Standard or Abridged (`boss.ur_editor` killed) |
| `margin.milestone.first_standard_victory` | `run.end.victory` in Standard |
| `margin.milestone.first_reliquary` | the first account Reliquary is credited |
| `margin.milestone.first_vellum` | the first Vellum is credited (doc 15) |

Milestones never unset. Apocrypha runs and easing-mutator runs set no milestone except `first_run_end`.

### 5.1a The trophy alcove

The Margin has fixed architecture: nothing in it is built, placed as a block or broken (Canon §17.8). `margin.plot` is the account's alcove, an interior of 8,000 mm (x, across) × 6,000 mm (z, deep) × 4,000 mm (y, up), origin at the front-left floor corner, entrance in the `z = 0` wall. It holds exactly 32 slots. A slot holds one trophy or decoration of its kind; placing, swapping and clearing are free, instant and saved to the account profile (doc 17 trophy arranger). Trophies come from milestones, achievements and first Editor kills; decorations are doc 15's `cosm.margin.*` (Codex track and Store) and the Bindery's `cosm.binding.<b>` and `cosm.palette.<g>` finishes (§5.3 shelf E). Nothing in the alcove has gameplay stats.

| ID | Kind | Footprint (mm) | Centre `(x, y, z)` (mm) |
|---|---|---|---|
| `margin.slot.trophy_01` to `margin.slot.trophy_12` | trophy (floor pedestal) | 600 × 600 (x × z) | `(480 + 640 × (k - 1), 0, 5400)` for `trophy_k` |
| `margin.slot.wall_01` to `margin.slot.wall_08` | wall (hung panel) | 1000 × 1000 (z × y) | `(0, 1800, 1000 + 1200 × (k - 1))` for `k = 1..=4`; `(8000, 1800, 1000 + 1200 × (k - 5))` for `k = 5..=8` |
| `margin.slot.floor_01` to `margin.slot.floor_06` | floor (rug or tile) | 1200 × 1200 (x × z) | `(1000 + 1200 × (k - 1), 0, 2700)` |
| `margin.slot.banner_01` to `margin.slot.banner_03` | banner (back wall, above the trophies) | 800 × 2000 (x × y) | `(2000 × k, 3000, 6000)` |
| `margin.slot.light_01` to `margin.slot.light_03` | light (ceiling fixture) | 400 × 400 (x × z) | `(2000 × k, 4000, 3000)` |

**Opening.** Eight slots open with the facility at `first_run_end`: `trophy_01` to `trophy_04`, `wall_01`, `wall_02`, `floor_01` and `light_01`. The other 24 open in this order, the `n`-th when the account owns `2 × n` achievements (§10): `trophy_05` to `trophy_12`, `wall_03` to `wall_08`, `floor_02` to `floor_06`, `banner_01` to `banner_03`, `light_02`, `light_03`. All 32 are open at 48 of the 59 achievements. No slot costs Quires or money; Quires and the Codex buy what fills them.

```json
{ "id": "margin.slot.trophy_05", "kind": "trophy", "footprint_mm": [600, 600],
  "centre_mm": [3040, 0, 5400], "opens": { "achievements": 2 } }
```

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
| `unlock.origin.unwritten` | 250 | gate: entered `genre.unwritten` | secret Origin selectable |
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
| `unlock.palette.<g>` for the 12 core genre suffixes | 25 each | gate: the account cleared a Folio of genre `g` | grants decoration finish `cosm.palette.<g>`: genre `g`'s palette on the alcove's wall, floor and banner slots (§5.1a, Canon §17.8, doc 15 catalog) |

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

### 5.3a Invader kit unlocks

Canon §17.9 lets an invader use a fixed Editor kit unlocked on its account. Doc 13 §10.7 defines four kits and makes `enemy.editors.kit.revisor` available to every account. The other three are these account unlocks, which are milestone grants and not Bindery nodes: each is granted free when its gate is first met, and none costs Quires or money, so the Bindery totals of §5.4 do not include them.

| ID | Grants the invader kit | Gate |
|---|---|---|
| `unlock.invader.annotator` | `enemy.editors.kit.annotator` | `margin.milestone.first_editor_v1` |
| `unlock.invader.censor` | `enemy.editors.kit.censor` | `margin.milestone.first_editor_v2` |
| `unlock.invader.footnoter` | `enemy.editors.kit.footnoter` | `margin.milestone.first_victory` |

An invasion is not a run of the invader's account (§8.1): it advances no milestone, achievement, Recollection, resident arc, Grade or Glyph Library.

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

Unlocking everything costs **7,670 Quires**. Achievements pay 1,960 Quires (§10, doc 15 grade table), leaving 5,710 from runs and the Codex free track (250 Quires per Codex, doc 15). At `Q_REF = 60` Quires per hour (only reference failed runs) that is 95.2 hours; at `Q_WIN = 143` Quires per hour (only reference Grade-0 victories, 299 Quires per 125 minutes) it is 39.9 hours. A typical player, who fails early and wins later, finishes between the two; §5.4a fixes the reference career.

Why these prices: doc 15 pays a run `min(400, floor_div(run_score, 50))` Quires (§4.6), so a reference hour earns 60 Quires. At this chapter's previous prices (27,100 Quires for 73 nodes) the tree would have taken over 300 hours at that rate. Every previous price was therefore divided by 4 and rounded to a multiple of 5 (alt kits 250 → 60, mutators 150 → 40), and the 18 new nodes (3 Callings, 3 alt kits, 12 palettes) follow the same scale. Starter options stay cheap (a first Calling costs 100 Quires, under two reference hours), and the gated secret Origins stay the most expensive single nodes.

### 5.4a Unlock pace

About `UNLOCK_PACE_HOURS = 80` hours of play unlock every non-cosmetic option without spending money (DECISIONS 79). The non-cosmetic options are the 73 non-cosmetic Bindery nodes (every node except the 6 Bindings and the 12 palettes: 7,070 Quires), the 24 Echoes, the 36 launch Scripts behind the 48 `unlock.` grants of §2.3 (doc 15 §3.1.2), the 3 invader kits of §5.3a and the Glyph Library (§5.7). Grades 1 to 20 open by victories (§7.4), and the gated nodes (secret Origins, alt kits, mutators, the Proving Page) and invader kits open by milestones; these are skill gates and are not paced.

| Options | Reference hours | Model |
|---|---|---|
| 73 non-cosmetic Bindery nodes | 70.0 | 7,070 Quires at `Q_MIX = 101` per hour from run payouts alone; achievements (1,960 Quires) and the Codex free track (doc 15) shorten it |
| 24 Echoes by Recollection alone | 84.2 | Σ `est_ch` = 16,839 over `RECOLLECTION_ACTIVE_MAX = 2` concurrent trials; the 5,520 ch of `editor_as` steps (55.2 hours), which a run can serve for only one Calling, fit inside it |
| 24 Echoes by Recollection and earned Ink | at most 66.1 | 80 hours pay 32,000 play Ink (doc 15: 400 per hour), about 4 featured Illuminated Echoes at 19.6 hours each; taking any 4 Tier 2 Echoes from the Inkwell leaves at most 16,839 − 3,635 = 13,204 ch (66.02 hours over 2 trials), the four shortest Tier 2 trials being 905, 910, 910 and 910 ch |
| 36 launch Scripts by Transcription alone | 76.8 | 768 qualified Folio exits (doc 15 §3.9.1: 24 Rubricated at 16 and 12 Illuminated at 32) at this model's 10 Folios per hour; 64.0 at doc 15's 12 exits per reference hour |
| Glyph Library | in play | every glyph learned in any run joins it (§5.7) |

**Pace contract.** A change to Bindery prices, doc 15's Quire, Ink or Transcription rates, Recollection steps or `RECOLLECTION_ACTIVE_MAX` must keep the Bindery row, the Recollection-alone row and the Transcription row at or below 85.0 reference hours.

### 5.5 Index and Bestiary knowledge

The item encyclopedia is the Index and the enemy encyclopedia is the Bestiary; "Codex" names only the battle pass (doc 15).

- **Bestiary.** Per enemy template, the account counts kills made while the player was present. 1 kill: entry, art and lore. 10 kills: that template's resistances and immunities are shown on the target HUD in every run. 25 kills: its drop categories are listed.
- **Index.** Per item template: an entry on the first `know.identified` instance, then one skin entry per genre in which the player held it. Appearance mappings are never stored, since they reshuffle every run (Canon §13.6).
- **Stories.** At run end, and at a `run.end.departed` for that slot, a Folio's story joins the Index of every present account in the party when `story_solved` is set or `leaf_carried` is true (the Folio Leaf `item.quest.folio_leaf` was carried through the exit, doc 12 §20). The record holds the story's template IDs and name seeds, never text; doc 18 regenerates the text.
- **World Laws** (Canon §5.8, DECISIONS 56). At run end, and at a `run.end.departed` for that slot, every Law whose `RunState.laws_triggered` slot mask holds the player's slot joins that account's discovered Laws, which previews and the HUD show from then on (doc 12 §4.1).

### 5.6 Margin residents and their arcs

The Margin's residents are doc 13 §13.6's eight `npc.margin.*` NPCs, who keep its facilities and run its vendors, and the unseen Narrator (doc 18 §8.4). Each resident has one arc `margin.arc.<resident>` of `ARC_BEATS = 6` beats, advanced through the optional Folio quests the resident gives across runs (DECISIONS 78, 130); completing a quest unlocks the resident's next text scene. Arcs are delivered as text: doc 18 writes each beat as a short exchange, and doc 17 shows it in the Margin dialogue box the next time the player speaks to that resident; the Narrator's beats appear as pencil notes in the Ledger. No beat is voiced, and no beat grants or gates anything. Residents keep their facilities and vendors along these fixed arcs and have no relationship state: no friendship meter, gifts or romance (DECISIONS 127).

**Advancing.** At each run end of any mode except Apocrypha runs, after the milestones and the counters of §5, §6 and §10 update, the meta layer makes the lowest not-yet-available beat of each arc available if its condition holds. At most one beat per arc becomes available per run end, so every arc spans at least 6 runs. Available beats are delivered in order, one per conversation. Conditions read only account milestones, achievements and the meta-layer counters named below; none reads Ink, Vellum, store purchases, pulls or Codex tiers (doc 15).

| Arc | Resident | Facilities | Beat conditions 1 to 6 |
|---|---|---|---|
| `margin.arc.archivist` | `npc.margin.archivist` | `margin.index`, `margin.bestiary` | `first_run_end`; 25 Bestiary templates; `first_editor_v2`; 100 Bestiary templates; `ach.appraiser`; every Editor (`boss.editor_v1` to `boss.editor_v4`, `boss.ur_editor`) in the Bestiary |
| `margin.arc.binder` | `npc.margin.binder` | `margin.bindery` | `first_run_end`; 5 Bindery nodes owned; 20; 46; 70; 91 |
| `margin.arc.recollector` | `npc.margin.recollector` | `margin.recollections` | `first_editor_v1`; one trial step at its target; 1 trial complete; 6; 12; 24 |
| `margin.arc.cartographer` | `npc.margin.cartographer` | `margin.ledger`, `margin.lectern` | `first_run_end`; Folios of 6 core genres cleared; `ach.atlas`; any secret genre entered; `ach.secret_trio`; `ach.marginalia_50` |
| `margin.arc.inkwell_keeper` | `npc.margin.inkwell_keeper` | `margin.inkwell`, `margin.reliquary_vault` | `first_run_end`; `first_editor_v1`; `first_victory`; 10 runs ended; `first_ur_text_victory`; 50 runs ended |
| `margin.arc.codex_scribe` | `npc.margin.codex_scribe` | `margin.codex_desk` | 1 run ended; 5; 15; 30; 60; 100 |
| `margin.arc.quartermaster` | `npc.margin.quartermaster` | `margin.store`, `margin.vellum_archive`, `margin.wardrobe` | `first_run_end`; `first_editor_v2`; 3 ranked Daily runs ended; `ach.gauntlet`; Grade 10 unlocked; Grade 20 unlocked |
| `margin.arc.drillmaster` | `npc.margin.drillmaster` | `margin.proving_page` | `first_editor_v1`; `ach.triad`; `ach.master`; `ach.deaths_door`; `ach.callings_v1`; `ach.callings_win` |
| `margin.arc.narrator` | the Narrator (doc 18 `nar.narrator`) | `margin.ledger` (pencil notes) | `first_run_end`; `first_editor_v1`; `first_victory`; `first_ur_text_victory`; `ach.marginalia_30`; `ach.marginalia_100` |

An `ach.` condition holds when the account owns that achievement; a milestone condition when the account holds it. Counters (meta layer, not content IDs): `runs_ended` (run ends of any mode except Apocrypha runs), `bindery_nodes_owned`, `bestiary_templates` (templates with at least 1 counted kill), `trial_steps_done`, `trials_done`, `core_genres_cleared`, `secret_genres_entered`, `ranked_dailies_ended` and `max_grade`.

```json
{ "id": "margin.arc.binder", "resident": "npc.margin.binder", "facilities": ["margin.bindery"],
  "beats": [ { "milestone": "margin.milestone.first_run_end" },
             { "counter": "bindery_nodes_owned", "target": 5 },
             { "counter": "bindery_nodes_owned", "target": 20 },
             { "counter": "bindery_nodes_owned", "target": 46 },
             { "counter": "bindery_nodes_owned", "target": 70 },
             { "counter": "bindery_nodes_owned", "target": 91 } ] }
```

### 5.7 The Glyph Library

Canon §17.1 `GLYPH_LIBRARY_START` (DECISIONS 33): every glyph an Erratum learns in any run joins the account's permanent Glyph Library, and at run start each Erratum pre-learns 2 glyphs chosen from it. This chapter owns the library as horizontal account meta; doc 06 owns glyphs and learning.

- **Record.** The meta profile holds `glyph_library: BTreeSet<Id32>` of glyph IDs, without origins. It never shrinks and has no cap.
- **Joining.** At run end, and at a `run.end.departed` for that slot, every glyph in the slot's `glyphs_learned` joins the library of that slot's account. `glyphs_learned` collects every `ev.glyph_learned` of the slot's Erratum, whatever the source (doc 06 §8.1: run start, tome, text study, trainer, Rewrite, kit). Apocrypha runs add nothing (§4.5).
- **Picks.** In `ErratumSelect` (§2.1) each player picks up to 2 distinct glyphs for the slot. A pick is refused when the glyph is in doc 06 §8.1's run-start set or is taught by the slot's kit primer, or when its doc 06 pool is not in the run's content set (Canon §13.6; the host's set in co-op). Runs on `unlock.daily_baseline` (Daily, Weekly and baseline Seeded runs) offer the 52 glyphs of doc 06's `base` pool in place of the account library, so every player of a shared seed has the same choice (Canon gaps, item 2). With fewer than 2 eligible glyphs the player picks that many.
- **Learning.** §2.2 step 9, and §2.5 for joiners: origin = the Erratum's Origin genre (`genre.ur_text` for an Unwritten Origin), as for doc 06's run-start glyphs; `ev.glyph_learned` with source 0; no draws. The picks are stored in `SlotState.library_glyphs`, so saves and replays carry them.
- **Horizontal.** A pick adds known glyphs and nothing else: no stat, Grimoire slot, complexity budget or rarity changes (§5.2).

---

## 6. Echoes and Recollection trials

### 6.1 Rules

- An Echo (Canon §16) fixes kin, Origin, Calling, body plan (`body.humanoid` unless §6.3 names another, doc 04 §13), signature trait and cosmetics. Its creation preset (allocation, Origin skill, kin) is doc 04's (§2.6). It uses its Calling's standard kit as kit A and the Calling's alt kit as kit B from Fold 3 (§2.4, doc 15 §3.8).
- An Echo is acquired through the Inkwell (doc 15) or by completing its Recollection trial here. Both grant identical ownership. Every Echo has a trial inside its tier's hour bound (§6.2), so the Inkwell is never the only path (DECISIONS 80): the content validator rejects an `echo.` record without one, and an Echo added later (a Folio Expansion or a roster addition) ships with its own bounded trial.
- Trials open with `margin.recollections`. A trial is available when its Echo's Calling is on the account.
- At most `RECOLLECTION_ACTIVE_MAX = 2` trials are active. Activation and deactivation happen at the Alcove between runs. Progress persists while a trial is inactive but advances only while active.
- Counters advance at run end from that run's per-slot event counters (stored in `RunState`, so a crash-resume loses nothing). Each step's counter caps at its target. Steps progress concurrently, in any order, across any number of runs.
- Runs of every mode count except Apocrypha runs (doc 15) and Custom runs with an easing mutator (§8.7); Scribe's Mercy never pauses a trial (DECISIONS 82). An "as X" step counts only runs in which the player's slot used Calling X.
- Completion grants the Echo at Fold 0 (doc 15 §3.8). If the Echo is already owned, completion grants only the cosmetic `cosm.echo.<suffix>.recollected` (doc 15 catalog): no Fold, no Ink and no Vellum, because doc 15 lists every Ink and Vellum source and a Recollection is not one of them. Each trial completes at most once per account.

### 6.2 Step kinds and hour model

Each step kind has a cost in **centihours** (ch, 1/100 hour of reference play) per unit, derived from the reference pace of 10 Folios, 400 kills and 100 broken destructible cells per hour.

| Step kind | Counts | ch per unit |
|---|---|---|
| `clear_genre` | Folios of genre G cleared | 35 |
| `editor_as` | `boss.editor_v<V>` killed while playing Calling X | 80 × V |
| `kills_type` | kills whose killing hit's largest-share damage type is T | 25 per 100 |
| `kills_edge` | kills of native targets by a base component with `E >= 1` | 1 per 3 |
| `sneak` | kills by sneak attack | 1 per 2 |
| `blocks` | destructible cells broken by the player's Erratum, not counting cells an Erratum placed (§3.5 rule) | 1 |
| `identify` | item instances reaching `know.identified` | 2 |
| `brew`, `craft` | `ev.potion_brewed`, `ev.item_crafted` | 3 |
| `notes` | marginal notes completed | 12 |
| `rewrites` | Rewrites committed | 20 |
| `deaths_door` | Death's Door saves | 40 |
| `erasure_exit` | Folios exited while holding ≥ 5 Erasure stacks | 50 |
| `hunters` | Erasure hunters killed | 60 |
| `secret_exit` | secret exits taken | 300 |

`est_ch(trial) = Σ steps of floor_div(count × unit_numerator, unit_denominator)`. **Hour bounds:** Tier 1 trials satisfy `400 <= est_ch <= 900` (4 to 9 hours); Tier 2 trials satisfy `900 <= est_ch <= 1600` (9 to 16 hours). The content validator rejects any trial outside its bound. Tier 1 trials (at most 9 hours) bind doc 15's slot B, whose Inkwell reference is 13.9 hours for one specific Rubricated Echo; Tier 2 trials (at most 16 hours) bind slot A, whose reference is 19.6 hours for a featured Illuminated Echo (§6.3). Every trial is therefore shorter than the Inkwell route to the same Echo. Doc 15's reference hour clears 12 Folios rather than this model's 10, which only shortens trials, so the bounds hold under both paces.

### 6.3 Roster, signature traits and trials

Traits are `trait.echo.<suffix>` (suffix = the Echo ID's suffix) in doc 04's `trait.` namespace, authored here as doc 04 §2.6 asks; the effect column is variant A and passes doc 04's trait power-budget validator. Variants B and C (`trait.echo.<suffix>_b`, `_c`, doc 15 Folds 2 and 5) ship in the content catalog with the same budget and are validated the same way. Genre abbreviations: MF Mythic Fantasy, GH Gothic Horror, CH Cosmic Horror, SO Space Opera, CP Cyberpunk, PA Post-Apocalypse, WW Weird West, HN Hardboiled Noir, WX Wuxia, SP Steampunk, FT Fairy Tale, MA Mythic Antiquity.

| Echo | Origin, Calling, kin | Signature trait (variant A) | Trial steps (genre steps use the Echo's Origin) | ch |
|---|---|---|---|---|
| `echo.fenrir` | MF, Monk, beastkin, `body.quadruped` | unarmed damage +2000 Bp; buy prices ×11000 | 6 Folios; V2 as Monk; 200 sneak kills | 470 |
| `echo.wren_long_index` | MF, Spellwright, fae | +1 Grimoire slot; complexity budget −1 | 10 Folios; V3 as Spellwright; 1,300 `dmg.arcane` kills | 915 |
| `echo.lilith` | GH, Shade, fae | sneak multiplier +2000 Bp at `light_at ≤ 4`, −2000 Bp at `light_at ≥ 10` | 6 Folios; 300 sneak kills; V2 as Shade | 520 |
| `echo.mother_ilse` | GH, Alchemist, human | brews of 2+ potions yield +1; toxicity decay 300 per s instead of 500 | 10 Folios; 110 brews; V3 as Alchemist | 920 |
| `echo.jormungandr` | CH, Oracle, beastkin, `body.serpentine` | composure loss ×7500; Max composure −20 | 6 Folios; 60 identifications; V2 as Oracle | 490 |
| `echo.drowned_clerk` | CH, Spellwright, undead | spell potency +1500 Bp while `st.soaked`; `dmg.fire` spell potency −2000 Bp | 11 Folios; 4 Erasure exits; V4 as Spellwright | 905 |
| `echo.ostra_vey` | SO, Gunslinger, human | swap speed ×13000; reload speed ×9000 | 6 Folios; 600 `dmg.pierce` kills; V2 as Gunslinger | 520 |
| `echo.kestrel_unit` | SO, Artificer, construct | gadgets deploy with +1 charge; gadget recharge ×8500 | 10 Folios; 80 crafts; V4 as Artificer | 910 |
| `echo.nix_seventeen` | CP, Netrunner, construct | lockpick and hack time ×6000; Max HP ×9500 | 6 Folios; 300 Edge kills; V2 as Netrunner | 470 |
| `echo.ronin_valdez` | CP, Duelist, human | blade crit damage +2500 Bp; crit chance −300 Bp | 10 Folios; 1,000 `dmg.slash` kills; V4 as Duelist | 920 |
| `echo.ma_rust` | PA, Grenadier, mutant | destructible blocks its explosions break yield ×2 materials; own explosive damage taken ×12500 | 6 Folios; 100 blocks; V2 as Grenadier | 470 |
| `echo.tallow` | PA, Alchemist, human | food satiation +2500 Bp; potion durations −1500 Bp | 10 Folios; 80 brews; V4 as Alchemist | 910 |
| `echo.josephine_crowe` | WW, Gunslinger, undead | crit chance +500 Bp at HP ≤ 30%; Max HP ×9500 | 6 Folios; 2 hunters; V2 as Gunslinger | 490 |
| `echo.preacher_amos` | WW, Warden, human | restoration heals deal 2000 Bp of the heal as `dmg.radiant` to undead and fiends within 4,000 mm; restoration aether cost ×11500 | 10 Folios; 6 Death's Door saves; V4 as Warden | 910 |
| `echo.dex_halloran` | HN, Duelist, human | unarmed stun buildup ×13000; blade damage −1000 Bp | 6 Folios; 400 `dmg.blunt` kills; V2 as Duelist | 470 |
| `echo.vera_lux` | HN, Alchemist, human | thrown poison flasks add +1 `st.poisoned` stack on a direct hit; buy prices ×10500 | 10 Folios; 960 `dmg.toxin` kills; V4 as Alchemist | 910 |
| `echo.lan_ninth_peak` | WX, Monk, human | dodge distance 4,000 mm; dodge cost 26 stamina | 6 Folios; 12 notes; V2 as Monk | 514 |
| `echo.sun_wukong` | WX, Monk, beastkin | polearm damage +1500 Bp; move speed multiplier +500 Bp; damage taken +500 Bp | 10 Folios; 20 notes; V4 as Monk | 910 |
| `echo.ada_brassvane` | SP, Artificer, human | crafting time ×7500; sell prices ×9000 | 6 Folios; 40 crafts; V2 as Artificer | 490 |
| `echo.colonel_wexley` | SP, Grenadier, human | launcher projectile range ×12500; launcher reload ×8500 | 10 Folios; 960 `dmg.fire` kills; V4 as Grenadier | 910 |
| `echo.briar_thornwood` | FT, Beastbinder, fae | minion Max HP +1500 Bp (summons and tamed beasts); Conjuration aether cost ×11500 | 6 Folios; 6 Rewrites; V2 as Beastbinder | 490 |
| `echo.wolf_in_wool` | FT, Shade, beastkin | disguise suspicion rate ×7000; Max HP ×9000 | 10 Folios; 480 sneak kills; V4 as Shade | 910 |
| `echo.kallias_twice_drowned` | MA, Vanguard, human | swim speed ×15000 and no drowning damage for 1800 ticks submerged; `dmg.fire` taken +1000 Bp | 6 Folios; 3 Death's Door saves; V2 as Vanguard | 490 |
| `echo.loki` | MA, Oracle, human | Illusion spell potency +2000 Bp; healing from prayers ×7500 | 11 Folios; 1 secret exit; V3 as Oracle | 925 |

Tier 1 is the first Echo of each genre (12 trials, 470 to 520 ch); Tier 2 is the second (12 trials, 905 to 925 ch). Every one of the 13 Callings has 1 to 3 Echoes (Alchemist and Monk 3; Vanguard, Warden, Netrunner and Beastbinder 1; every other Calling 2). All 24 trials total 16,839 ch (168.4 hours).

**Public-domain figures** (Canon §16.2). The five required figures hold these roster places: `echo.fenrir` (MF Tier 1), `echo.lilith` (GH Tier 1), `echo.jormungandr` (CH Tier 1), `echo.sun_wukong` (WX Tier 2) and `echo.loki` (MA Tier 2). Their art, voice and source texts are doc 18's. Fenrir uses `body.quadruped` and Jörmungandr `body.serpentine` (doc 04 §13); their natural attacks (Fenrir: bite, claw, pounce; Jörmungandr: bite, constrict, spit) are the unarmed function `arch.melee.unarmed` skinned per body plan, and their kit items follow the §2.4.2 slot rule. Kin traits that need a part the body plan lacks are skipped, and beastkin's added `bp.tail` is not added to `body.serpentine`, which already has one (doc 04 §13). Fenrir takes the Monk Calling because a quadruped has no off-hand for a Vanguard's shield; Monk's standard kit puts the hand wraps in `slot.weapon_1` (the mouth). Sun Wukong is a Monk too, and his trait favours the staff of the Monk alt kit (Fold 3 kit B). The five original Echoes whose roster places these figures took never shipped; their IDs are aliased to their successors in `audit/map_run.json` (Canon §1.1).

**Inkwell binding** (doc 15 §3.1.1). Doc 15 makes each genre's Tier 2 Echo Illuminated (slot A, the rarer Inkwell tier: 19.6 hours of play per featured Echo) and its Tier 1 Echo Rubricated (slot B: 13.9 hours per specific rate-up Echo). Both routes stay ordered the same way: the rarer Inkwell tier has the longer trial, and each trial is shorter than the matching Inkwell reference. Genre index `g` is the Canon §5.2 row.

| g | Genre | Slot A (Illuminated, Tier 2) | Slot B (Rubricated, Tier 1) |
|---|---|---|---|
| 1 | MF | `echo.wren_long_index` | `echo.fenrir` |
| 2 | GH | `echo.mother_ilse` | `echo.lilith` |
| 3 | CH | `echo.drowned_clerk` | `echo.jormungandr` |
| 4 | SO | `echo.kestrel_unit` | `echo.ostra_vey` |
| 5 | CP | `echo.ronin_valdez` | `echo.nix_seventeen` |
| 6 | PA | `echo.tallow` | `echo.ma_rust` |
| 7 | WW | `echo.preacher_amos` | `echo.josephine_crowe` |
| 8 | HN | `echo.vera_lux` | `echo.dex_halloran` |
| 9 | WX | `echo.sun_wukong` | `echo.lan_ninth_peak` |
| 10 | SP | `echo.colonel_wexley` | `echo.ada_brassvane` |
| 11 | FT | `echo.wolf_in_wool` | `echo.briar_thornwood` |
| 12 | MA | `echo.loki` | `echo.kallias_twice_drowned` |

**Fold content binding** (doc 15 §3.8). For every Echo: Fold 1 `cosm.echo.<suffix>.palette`, Fold 2 trait variant B, Fold 3 kit B (the Calling's alt kit), Fold 4 `cosm.echo.<suffix>.outfit`, Fold 5 trait variant C, Fold 6 `cosm.echo.<suffix>.frame`, `cosm.echo.<suffix>.title` and `cosm.echo.<suffix>.trail`. The `cosm.` IDs are doc 15's catalog entries; this chapter fixes only their names.

Echo record:

```json
{
  "id": "echo.fenrir",
  "origin": "genre.mythic_fantasy",
  "calling": "calling.monk",
  "kin": "kin.beastkin",
  "body": "body.quadruped",
  "trait": "trait.echo.fenrir",
  "inkwell_tier": "rubricated",
  "tier": 1,
  "est_ch": 470,
  "steps": [
    { "kind": "clear_genre", "genre": "genre.mythic_fantasy", "count": 6 },
    { "kind": "editor_as", "calling": "calling.monk", "volume": 2, "count": 1 },
    { "kind": "sneak", "count": 200 }
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
    pub rest_heal_bp: i32,          // 10000; §3.9 Interleaf rest and Endpaper restore only
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
m13 = mul_bp(boon_off(attacker, target.rank), brittle_bp(f))      if attacker is an Erratum (not a minion) and target is an NPC
m13 = 10000                                                      for every other hit
boon_def(e)      = 9000 if e's active boon is run.boon.thick_hide, else 10000
boon_off(e, rank) = 11000 if rank == editor and e's boon is run.boon.editors_bane;
                    13000 if rank == hunter and e's boon is run.boon.hunters_bane; else 10000
brittle_bp(f)     = max(7500, 10000 - 500 × r) while run.marg.brittle_kit is at rank r (§9.3); else 10000
```

Enemy Max HP gains a final factor: `template_hp × H(d) × rank HP × party-size HP × diff_hp_bp`, each a `mul_bp` in that order (Canon §13.8 `H(d)` row). Both chains start at 10000, apply `mul_bp` left to right, and clamp to `DIFF_CAP_BP` last.

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

Grades open with `margin.milestone.first_victory`, which unlocks Grade 1. A victory at Grade `G` unlocks `G + 1` (to at most 20). For `G <= 9`, a Standard, Abridged or Novella victory counts; for `G >= 10`, only a Standard or Abridged victory counts, because only those reach the Ur-Text. Seeded and Custom victories never unlock Grades. Runs with any easing mutator or a simulation-affecting mod (Apocrypha) never unlock Grades. Scribe's Mercy (`access.scribes_mercy`, doc 17) never blocks a Grade unlock (DECISIONS 82). Daily and Weekly runs use fixed Grades and never unlock Grades. `max_grade(account)` is the highest unlocked Grade, 0 before `first_victory`. In co-op only the host's `max_grade` limits the lobby. A member present at a victory at Grade `G` unlocks `G + 1` only when `G <= max_grade(member)`, so a carried player never skips Grades.

---

## 8. Modes

### 8.1 Mode table

| ID | Structure, final Folio | Seed | Content set | Erratum | Grade | Mutators | Ranked | `mode_quire_bp` | `folio_scale_bp` | Mid-run save (Canon §2.5) | Opens at |
|---|---|---|---|---|---|---|---|---|---|---|---|
| `mode.standard` | Standard, 25 (Endless optional) | host CSPRNG | account | free | 0..=max | none | yes | 10000 | 10000 | solo: any tick; co-op: Folio entry | `first_run_end` |
| `mode.abridged` | Abridged, 15 (`d = 25`) | host CSPRNG | account | free | 0..=max | none | yes | 7500 | 9000 | solo: any tick; co-op: Folio entry | `first_run_end` |
| `mode.novella` | Novella, 15 | host CSPRNG | account | free | 0..=max | none | yes | 10000 | 8000 | solo: any tick; co-op: Folio entry | `first_editor_v1` |
| `mode.daily` | Daily, 5 | Canon §3.6 daily | `unlock.daily_baseline` | drawn | 2 | 1 drawn | first attempt per UTC day | 10000 ranked, 5000 practice | 8000 | as Standard; ranked rules §12.3 | `first_run_end` |
| `mode.weekly_gauntlet` | Novella, 15 | Canon §3.6 weekly | `unlock.daily_baseline` | free from baseline | 6 | 3 drawn | yes | 10000 | 8000 | as Standard; ranked rules §12.3 | `first_editor_v2` |
| `mode.seeded` | Standard, Novella or Abridged | typed | baseline (toggle: account) | free | 0..=max | none | no | 5000 | by structure | as Standard | `first_editor_v1` |
| `mode.second_edition` | Second Edition (§8.9), `final_folio` = Ur-Text index + 15 | inherited | inherited | inherited | inherited | none | as its base run | inherited | 10000 | as Standard | an Ur-Text victory in `mode.standard` or `mode.abridged` |
| `mode.endless` | Standard continued, none (≤ 9999) | inherited | inherited | inherited | inherited | Marginal mutations only (§9.3) | yes | 10000 | 10000 | as Standard | `first_standard_victory` |
| `mode.custom` | Standard (25, or 10 with `run.mut.short_book`), Novella or Abridged | CSPRNG or typed | account | free | 0..=max | any owned | no | 5000; 2000 with `run.mut.short_book`; 0 with an easing mutator | by structure | as Standard | `first_victory` |

"By structure" means 10000 for Standard, 9000 for Abridged and 8000 for Novella. `folio_scale_bp` is passed to doc 12, which scales each Folio's room count to `max(4, mul_bp(rooms_base, folio_scale_bp))`. Co-op (1 to 4 players) overlays every mode (§8.8).

**`mode_quire_bp` choices.** Abridged's 7500 brings its Quires per hour (Example E, 141 per hour) in line with Standard's (Example B, 143 per hour); at 10000 it would pay 212 per 67.5 minutes. Seeded and Custom pay 5000 because the player picks the seed or the mutators. The 12 hard mutators multiply to 64,529 Bp, so `MUTATOR_BP_CAP = 20000` (§4.2) bounds them first. Even capped, a Grade-20 `run.mut.short_book` victory (10 Folios at 300 s, 50 minutes) scores 208,200: at 10000 or 5000 its `run_score` (30,397 or 15,198 raw) meets the per-Folio cap of 8,000 (§4.6) and pays 160 Quires, 192 per hour, a third above a Standard victory. At 2000 it pays 121 Quires (raw 6,079), 145 per hour, level with a Standard victory, so the short structure carries 2000.

**Overlays** (Canon §17.5 and §17.9, DECISIONS 104, owned here). These five modes are lobby overlays, never selected alone. They are set in `RunSetup`, fixed at `Commit`, and combine with every base mode except where a row says otherwise. None changes `mode_quire_bp`: an overlay run pays and counts by its base mode's row (doc 15 pays an invasion-open run by its own mode row).

| ID | Slots | Effect | Stored in | Ranked |
|---|---|---|---|---|
| `mode.ff_off` | 2..=4 | `FF_DIRECT_BP = 0`, `FF_AREA_BP = 5000` (Canon §17.5); excludes `mode.ff_full` | `RunState.coop` | no |
| `mode.ff_full` | 2..=4 | `FF_DIRECT_BP = FF_AREA_BP = 10000`; excludes `mode.ff_off` | `RunState.coop` | unchanged, since it only makes the run harder |
| `mode.loot_shared` | 2..=4 | one drop per kill, first pickup wins (Canon §17.5) | `RunState.coop` | no |
| `mode.invasion_open` | 1..=4 | Redactor invasions may open under Canon §17.9's window and limits (doc 13, doc 16); a Redaction replaces the run end of §3.2 row 1 | `RunState.invasion_open` | no |
| `mode.audience_votes` | 1..=4 | streaming viewers vote on the host's doors and boons and on a Chaos mutator (Audience votes, below) | `RunState.audience` | no |

**Invasions.** `mode.invasion_open` is off by default. Daily and the Weekly Gauntlet never offer it, and a ranked token is never issued for a run that has it (Canon §17.9: ranked runs forbid it; doc 16 refuses the token). It needs every party member online, no simulation-affecting mod, and every member's account permitting invasions (doc 15 `juris.flag.invasion_minor_default`). The invader occupies replay slot 4 (Canon §2.4), never a party slot: it never counts toward `n`, `folios_present`, `EXIT_OFFERS`, votes, score or achievements, and its death is not an Erasure-hunter kill for `S_hunters`. When an invasion ends, `FolioRecord.invasion` records how (§4.7). The invader picks an Editor kit its account has unlocked (§5.3a, doc 13 §10.7). Its rewards are doc 15's Quires and Red Ink marks only (Canon §17.9, DECISIONS 86): invading pays no Ink, Codex XP or Reliquaries, and an invasion is not a run of the invader's account, so it advances none of that account's milestones, achievements, Recollections, resident arcs, Grades or Glyph Library.

**Apocrypha.** A run with any simulation-affecting mod (doc 02 §13, doc 15 §2.2) is unranked in every mode, earns no account rewards (doc 15), unlocks no Grade, advances no Recollection, achievement or milestone except `first_run_end`, and is marked in run history.

**Audience votes** (`mode.audience_votes`, DECISIONS 104; an update after Early Access, DECISIONS 118). Off by default. The Daily and the Weekly Gauntlet never offer it, and a ranked token is never issued for a run that has it, so it never reaches a board (§9.4). The streamer is the player in slot 0, the host at Commit, and only slot 0's choices are put to the vote; while slot 0 is not Active, every result has no effect. Doc 16's vote service collects the Twitch and YouTube viewers' votes and closes each vote; the host's client then logs one session intent `AudienceResult { kind: u8, choice: u8 }` (doc 16 §5.7), applied with the other session intents before player intents on the tick it is logged, so replays and the verifier reproduce it (Canon §2.4). Viewers send nothing else into the run, and a result whose condition no longer holds on its tick has no effect. Streamer-safe presentation (names and notes hidden) is doc 17's.

| `kind` | The vote opens | Choices | Effect of `choice` |
|---|---|---|---|
| 0, door | on the `InColophon → ExitVote` tick (§3.2); never in `VictoryChoice` | the doors `0..doors.len()` | slot 0's `ChooseExit { door: choice }` vote is set exactly as if the host had sent it (§3.6); the host overrides it by standing in a door volume or sending its own `ChooseExit`. No effect once a door has committed |
| 1, boon | when slot 0's boon offer is stored at a Colophon, and again after each reroll (§3.9); doc 16 closes an open boon vote without a result on a reroll | the stored offer's boons `0..=2` | slot 0 commits that boon exactly as its own commit would: one `next_u64` on slot 0's `rng.offer.p<slot>`, `boon_next` set, `ev.boon_committed`. No effect when slot 0 has already committed or holds no offer |
| 2, Chaos | at every Folio entry except that of `final_folio` | 3 mutators of the Gauntlet pool (§8.7) without `run.mut.bare_kit`, which acts only at kit creation: 11 entries sorted by ID, drawn on the labeled sub-stream `Rng::from_seed64(derive(stream_seed(run_seed, "rng.run", f), fnv1a64("chaos")))` with `range(11)`, `range(10)`, `range(9)` without replacement (Canon §3.2 sub-stream: `rng.run` itself never advances); `choice` `0..=2` indexes them in draw order | `chaos_next` = that mutator. At the next Folio entry, before generation (§3.3), `chaos = chaos_next.take()`; while `chaos` is set, `gp` and the §7.3 chains read it merged after the run's own mutators (§7.2), for that Folio only. It never enters `RunState.mutators` or `mutator_bp`. No effect after the Folio's exit commit |

**Asynchronous traces** (DECISIONS 96; an update after Early Access, DECISIONS 118). Each player opts in separately, off by default, both to leave and to see traces. A trace is a grammar-built marginal note a player writes at a position (doc 18 grammar) or an ink-stain death echo recorded at that player's death; doc 16's trace service stores it under the run seed and the Folio genre and shows it in other players' Folios of the same seed or genre. Traces are presentation only: they never read or advance a simulation stream and never enter simulation state, saves, `state_hash`, score or any account counter (Canon gaps, item 5). A ranked run shows no trace stored under its own seed, so a shared-seed board is never scouted through other players' notes; traces stored under its genre from other seeds still show.

### 8.2 Standard

The full 25-Folio run (100 to 150 minutes, Canon §17.1), Volume I to V Editors, then the Folio 25 Colophon and VictoryChoice (§3.2). Single-player suspend at any tick boundary; co-op saves at Folio entry (§12).

### 8.2a Abridged

`mode.abridged` (Canon §17.1, owner doc 14) is the shorter full-story run: 5 Volumes of 3 Folios, two regular Folios plus the boss, 15 Folios in all. `d(f) = d_eff` (§3.1) feeds `P`, `T`, `H`, `S`, `ilvl` and `enemy_level`; per-Folio streams, `EntityId` and uids use the Folio index `f`. Doc 13 multiplies kill XP and doc 11 multiplies loot quantity by 16667 Bp per Folio (Canon §17.1); loot quantity means worldgen chests and floor-loot spots, and Reliquaries, Fortune Engines and vendor stock are unchanged. `EMENDATIONS_PER_VOLUME` stays 3. The final Folio is the Ur-Text at `d = 25` with `boss.ur_editor`; its Endpaper opens the Colophon, whose lectern opens the `VictoryChoice` of §3.2 with the "Second Edition" and "Close the Book" doors (§8.9); "Close the Book" ends the run `run.end.victory` with `victory_bp = 17500`, and Abridged never continues into Marginalia. Target length is 60 to 75 minutes: 15 Folios at 240 to 300 s, 270 s on average, with `folio_scale_bp = 9000`. Abridged has its own board (§9.4).

### 8.3 Novella

Folios 1 to 15 at `folio_scale_bp = 8000`, targeting 240 s per Folio and 60 minutes per run, with `d(f) = f`, so Novella plays Volumes I to III of the Standard run at Standard depth. `boss.editor_v3` at Folio 15 is the final boss; its Endpaper ends the run `run.end.victory` with `victory_bp = 15000`. Novella has its own boards and never continues into Marginalia.

### 8.4 Daily

One Volume: Folios 1 to 5, `boss.editor_v1` final (`victory_bp = 15000`), Grade 2, `folio_scale_bp = 8000`, Folio 1 genre by rule F1's seeded branch. At run start the host draws from `rng.run` with `folio_index = 0`, exactly 9 draws in this order:

1. Callings for slots 0..=3 without replacement from the 13 Callings in Canon §16.2 table order: `range(13)`, `range(12)`, `range(11)`, `range(10)` (4 draws, always, whatever the party size).
2. Origins for slots 0..=3 with replacement from the 12 core genres in Canon §5.2 order: `range(12)` ×4.
3. One mutator from the Gauntlet pool (§8.7, 12 entries sorted by ID): `range(12)`.

Slots beyond the party size ignore their draws. A drawn Erratum is `kin.human` with doc 04's Calling preset allocation, the Calling's standard kit, the player's own appearance preset and the player's 2 glyph picks from doc 06's `base` pool (§5.7).

Example, 2026-10-11 (daily seed `0xc6f2e12ddca712ab`, `rng.run` stream seed `0x6c9f436c0c107704`): slots 0..=3 are Grenadier/Cyberpunk, Oracle/Hardboiled Noir, Duelist/Weird West, Artificer/Gothic Horror; mutator `run.mut.dear_pages`. The raw draws are Callings 3, 8, 1, 5 (Grenadier from 13; Oracle from the remaining 12; Duelist from 11; Artificer from 10), Origins 4, 7, 6, 1, and mutator 4 of the sorted pool. Folio 1 is Weird West (doc 12 §5.6 candidates Weird West, Cosmic Horror, Mythic Fantasy). Margin Translation then keeps slot 0's Cyberpunk heavy weapon: `Req[techne] = 7`, local TECHNE 4, `raw_gap = 3`, `Tol = 2`, so `C = 1`.

**Ranked attempt.** Each account has one ranked Daily attempt per UTC date. A run is ranked when every party member's attempt for that date is unused at `Commit`; `Commit` consumes them all. Any further attempt that day is practice: unranked, `mode_quire_bp = 5000`. Missing a day removes nothing; there are no streaks.

### 8.5 Weekly Gauntlet

Novella structure (15 Folios, `boss.editor_v3` final, `victory_bp = 15000`), Grade 6, free Calling and Origin from `unlock.daily_baseline`. Three mutators come from `rng.run` with `folio_index = 0` without replacement over the 12-entry Gauntlet pool: `range(12)`, `range(11)`, `range(10)` (3 draws). Example, ISO week `2026-W41`: weekly seed `0x2e7e2bae906b8339`, stream seed `0x54f2e7d7bdbce9f5`, raw draws 2, 0, 3, mutators `run.mut.blind_exits`, `run.mut.ambushes`, `run.mut.elite_ink`; Folio 1 is Weird West. Attempts are unlimited; the board keeps each party's best verified score of the week.

### 8.6 Seeded

The player types a seed (Canon §3.6 decoding) and picks the Standard (25 Folios), Novella (15) or Abridged (15) structure. Share string: seed display, then `-`, then four positional characters: structure letter `S`, `N` or `A`; content letter `B` (baseline) or `A` (account); two Grade digits. Example: `028T-5CY4-TQKFF-SB07` is Standard, baseline, Grade 7; `028T-5CY4-TQKFF-AA00` is Abridged, account set, Grade 0. A Grade above the player's `max_grade` is lowered to it. Seeded runs are unranked, pay `mode_quire_bp = 5000`, never offer Margins doors, and unlock no Grade.

### 8.7 Custom mutators

Custom runs pick a structure, a Grade and any owned mutators. They are unranked and never offer Margins doors. An **easing** mutator sets `mode_quire_bp = 0`, disables achievements and Grade unlocks, and pauses Recollection counters for the run. A mutator's Bp effects merge into `GradeParams` after the Grade (§7.2); its HP and damage factors join the §7.3 chains within their scope.

| ID | Effect | `score_bp` | Easing | Gauntlet pool |
|---|---|---|---|---|
| `run.mut.ambushes` | `ambush_extra += 2` | 11000 | no | yes |
| `run.mut.bare_kit` | kit consumables and kit armor removed | 12000 | no | yes |
| `run.mut.blind_exits` | doors hide genre, axioms and forecasts until committed | 10500 | no | yes |
| `run.mut.champion_parade` | `champion_extra += 2` | 12500 | no | yes |
| `run.mut.dear_pages` | `shop_price_bp` chain 15000 | 11000 | no | yes |
| `run.mut.elite_ink` | `elite_promote_add_bp += 2000` | 12000 | no | yes |
| `run.mut.hungry_hunters` | hunter-scoped HP and damage factor 15000 | 11000 | no | yes |
| `run.mut.iron_ink` | damage chain factor 15000 | 13000 | no | yes |
| `run.mut.no_rest` | Interleaf Rest disabled; `rest_heal_bp` chain 5000 | 12000 | no | yes |
| `run.mut.pauper` | `lucre_bp` chain 5000 | 11500 | no | yes |
| `run.mut.thick_skins` | HP chain factor 15000 | 13000 | no | yes |
| `run.mut.wild_bleed` | doc 12 bleed-zone count ×2 | 11000 | no | yes |
| `run.mut.paper_foes` | HP chain factor 6000 | 5000 | yes | no |
| `run.mut.patron` | `lucre_bp` chain 20000 | 8000 | yes | no |
| `run.mut.short_book` | `final_folio = 10` (`boss.editor_v2` final); valid only with the Standard structure; `mode_quire_bp = 2000` | 10000 | no | no |
| `run.mut.soft_ink` | damage chain factor 6000 | 5000 | yes | no |

Mutator record:

```json
{ "id": "run.mut.iron_ink", "score_bp": 13000, "easing": false, "gauntlet": true,
  "scope": "all", "dmg_factor_bp": 15000, "hp_factor_bp": 10000 }
```

### 8.8 Co-op overlay

- **Lobby and authority.** 1 to 4 slots, host-authoritative (doc 16). Every mode.
- **Scaling.** `n` is fixed at each Folio entry (§3.3) and drives Canon §17.2 for spawns in that Folio. Loot is instanced per player (Canon §17.1). XP splits per Canon §8.
- **Exits and gates** use the unanimity-then-majority rule (§3.6). Ghosts follow §3.10.
- **Joining** happens only at a Folio entry (§2.5).
- **Co-op modes.** `mode.ff_off`, `mode.ff_full` and `mode.loot_shared` (Canon §17.5) are set in `RunSetup` (§8.1).
- **Leaving.** A disconnected player's slot becomes `Dormant` at once: the Erratum leaves the world with its inventory and is not counted in `n` from the next Folio entry. If the player is not back within `DORMANT_GRACE_S` (300 s wall clock), or leaves on purpose, the slot becomes `Departed` and `run.end.departed` is recorded for that slot. Both host and player compute the departed `run_score` (§4.6) from the score accumulators of the most recent Folio-entry save, with `victory_bp = 10000` and the slot's presence; doc 15 converts it to Quires once, and that grant is stored as `quires_paid`. A Departed or Dormant player who rejoins at a later Folio entry becomes Active again, and the final grant subtracts `quires_paid` (§4.6).
- **Host migration** follows doc 16 §9.3: hot and warm migrations continue the Folio; only a cold migration restores the Folio-entry save and restarts the Folio, and a cold migration counts as a restore (§12.3).
- **Session intents** (doc 16 §5.7), applied before player intents. `SlotJoin { slot, character }` is consumed by `enter_folio` (§3.3) at a Folio-entry boundary, in ascending slot order and before the Folio-entry save, and creates an `Active` slot per §2.5; none takes effect at the entry of `final_folio` (doc 16 §9.1). `SlotDormant { slot }` makes the slot `Dormant` at once (Leaving, above). `SlotResume { slot }` takes effect at the next Folio entry: the slot becomes `Active` and its Erratum enters at the Incipit with the state and inventory it left with. `SlotDeparted { slot }` makes the slot `Departed` and records `run.end.departed` for it (§4.6 payout). `HostChanged` has no simulation effect. `AudienceResult { kind, choice }` has the effects of §8.1's audience votes. `FolioRestart { folio }` marks a cold migration: the Folio restarts from its entry save and `restores` rises by 1.
- **Ranked co-op.** Daily needs every member's unused attempt (§8.4). Boards are per party size (§9.4).

### 8.9 Second Edition

Second Edition is the post-game continuation of DECISIONS 100 to 108 and 117 and ships at Early Access (DECISIONS 118; doc 21 WP-120 to WP-122). This chapter owns its run flow. Doc 12 writes its Folios in two fused genres (DECISIONS 101), doc 13 its 5 revised Editors and the Author (DECISIONS 103), doc 15 its payout and the Ironman cosmetic `cosm.frame.ironman`, doc 18 the second ending layer (DECISIONS 100), and Canon §13.7, §13.8, §17.1 and §17.6 its depth, structure and resurrection rule (Canon gaps, items 4 and 8).

- **Offer.** Every Ur-Text victory of a `mode.standard` or `mode.abridged` run offers the "Second Edition" door in `VictoryChoice` (§3.2), the account's first victory included, because the Errata who clear the Ur-Text continue straight into it (DECISIONS 102) and the choice is offered at the clear (DECISIONS 108). Seeded and Custom runs never offer it. The door shows the stakes below in place of a genre preview, and it commits only by the §3.6 rule, so the party's own voters choose it; no audience vote opens in `VictoryChoice` (§8.1).
- **Entry.** On the commit tick, in this order: `score_at_victory` is stored with `victory_bp = 17500` (§4.2); `mode` becomes `mode.second_edition`; `final_folio = f_ur + 15`, where `f_ur` is the Ur-Text Folio's index (25 in Standard, 15 in Abridged). Second Edition Folio `n = 1..=15` is run Folio `f = f_ur + n`, so `EntityId`s, uids and per-Folio streams keep counting; its Volume, position and depth are §3.1's Second Edition row: Volumes 6 to 8 of 5 Folios each, boss Folios at `n = 5, 10, 15` and `d = 25 + 2n` (DECISIONS 105, 106). Grade, mutators and co-op modes stay as committed.
- **Revised Editors.** Doc 13 §9.4 places them with no draw. The Final Folio (`n = 15`) holds `boss.ur_editor_rev` and then the Author, `boss.author`. Each of the boss Folios `n = 5` and `n = 10` holds the revised Volume Editor (`boss.editor_v1_rev` to `boss.editor_v4_rev`) that doc 13 maps from the dominant axiom of its fused pair, which the party picks at the pos-4 Binding doors (§3.7), with doc 13's cycle rule so Volumes 6 and 7 never hold the same one; doc 12's exit preview names it behind each Binding door. Each boss Folio records its first Editor in `FolioRecord.editor` at entry. The Author's death is the Final Folio's Editor death for §3.2, §3.8 and §4.2, so the Erasure clock stays paused until it dies and its Endpaper, with the Folio's one Volume reward, ends the run `run.end.victory` with `victory_bp = 17500`. That Folio pays two Editor kills to doc 15's `editor_kills_present` and counts once for `S_editor`. `gp.final_hp_bp` and `final_phase_extra` apply to the Author, the run's final boss (§7.2, §7.3), and not to `boss.ur_editor_rev` (DECISIONS 103, 105).
- **Fused Folios.** Doc 12 supplies each Second Edition Folio's fused genre pair, Folio 1's included, and its exits. The Volume no-repeat rule applies per pair: a Volume never repeats a pair, while one genre may appear in two different pairs (DECISIONS 117). The run-wide cap of 3 appearances ends at the Ur-Text Folio, as it does for Marginalia. `FolioRecord.genre` holds the pair's genre with the lower Canon §5.2 row and `FolioRecord.fused` the other, which is how doc 15 §2.2 reads a fused Folio's genre.
- **Party.** Every Active and Ghost slot enters Second Edition Folio 1 with its character, gear, level, skills and Lucre, as at any Folio entry (DECISIONS 102, 107). A Ghost returns as it was at death: it resurrects there by §3.10 with its Remains counted as carried, so it loses no item and returns at `GHOST_REMAINS_HP_BP` (Canon gaps, item 8). Dormant and Departed slots follow §8.8, and a joiner may enter at Second Edition Folio 1 by §2.5 (doc 16 §9.1).
- **Folios.** Every rule of §3 applies, including Rubrics, notes, the Erasure clock, Colophons and boons. Second Edition Folios are not Marginalia: they draw no Marginal mutation, grant no Marginalia Emendation and have `marg_hp_bp = marg_dmg_bp = 10000` (§9.2). Doc 11 places `EMENDATIONS_PER_VOLUME` in each Second Edition Volume as in the main run, and the enhancement cap and armor HP follow the depth (Canon gaps, item 4).
- **Stakes** (DECISIONS 108). Every other end of a Second Edition run (a party wipe, an Abandon or `run.end.abandoned_version`) forfeits the clear: the run keeps that end ID, discards `score_at_victory`, scores `score_now` with `victory_bp = 10000`, reports `victory_present = 0` for every slot (doc 15 §2.2), and therefore unlocks no Grade and sets no milestone or achievement that needs a victory. Counting Abandon as a forfeit keeps the stake from being dodged.
- **Victory.** A Second Edition victory counts as a victory of the run's structure (Standard or Abridged) for §5.1, §7.4 and §10, and scores `max(score_at_victory, score_now)`. With no `ev.player_died` in any Second Edition Folio it is an **Ironman** clear: `RunSummary.ironman = true`, doc 15 grants `cosm.frame.ironman` once per account to each player with `victory_present = 1`, and the board entry carries the Ironman mark (§9.4).
- **Boards and payout.** A Second Edition run stays ranked when its base run was and posts its final score to its structure's board (`score.board.standard` or `score.board.abridged`). Doc 15 pays it once, at run end, from `RunSummary`, as `mode.second_edition` under its mode row 1.
- **Ending.** A Second Edition victory shows doc 18 §8.5's two layers: `ur_clear_ending`, the ending its Ur-Text victory earned, with that ending's closing manuscript, then `nar.ending.second_edition`, whose closing line is the Ironman line when `RunSummary.ironman` is true. A forfeited run shows only the ending of its own end (§4.7).
---

## 9. Endless Marginalia and leaderboards

### 9.1 Structure

Margins doors appear in a `mode.standard` run's VictoryChoice only when the host's account held `margin.milestone.first_standard_victory` before the run started; otherwise no Margins door is shown, and the doors are the "Second Edition" door (§8.9) and "Close the Book". The Margins doors are the first `EXIT_OFFERS` of the 3 lattice candidates drawn on the Folio 25 lectern-interaction tick. Endless begins when a Margins door commits: `mode` becomes `mode.endless`, `final_folio = u32::MAX`, and `score_at_victory` is stored (§4.2). Marginalia Folios 26 to `MARGINALIA_MAX_FOLIO` use §3.1 positions with `d(f) = f`, including the pos-4 Interleaf and a Colophon in every Folio. Every Keeper is `rank.champion`. Boss Folios (`f % 5 == 0`) rotate Editors by `r = (floor_div(f, 5) - 6) % 5`: `r = 0..=3` gives `boss.editor_v1..=v4` and `r = 4` gives `boss.ur_editor` (Folio 30 → v1, 45 → v4, 50 → Ur-Editor, 55 → v1); the boss Folio's genre comes from the pos-4 Binding doors, as in the main run. Each Marginalia boss Folio's exit vote appends a "Close the Book" door after its exit doors (`run.end.margin_closed`, §3.2). The lattice keeps "no genre twice in the same Volume"; the run-wide cap of 3 appearances ends at Folio 25 (Canon §17.1).

Canon rules that continue in Marginalia: each Erratum receives 1 Emendation on every Marginalia `ev.folio_exited` and the enhancement cap is `15 + floor_div(d - 25, 5)` (Canon §13.7); armor adds Marginalia HP `gear_hp` (Canon §13.8.2); every Folio keeps the Erasure clock, Colophon and boons of §3.

### 9.2 Scaling

Power tables keep growing to `d_pow = 300` (Canon §13.8). This chapter adds:

```
marg_hp_bp(f)  = if f >= 26 { 10000 + 100 × (f - 25) } else { 10000 }
marg_dmg_bp(f) = if f >= 26 { 10000 +  50 × (f - 25) } else { 10000 }
deep_bp(f)     = if f > 300 { 10000 + 500 × (f - 300) } else { 10000 }   // in both chains
```

`marg_hp_bp` and `marg_dmg_bp` apply only to a `mode.endless` run; in Second Edition, whose Standard Folio indices also pass 25, both are 10000 (§8.9).

| f | `marg_hp_bp` | `marg_dmg_bp` | `deep_bp` | Grade 0 `diff_hp_bp` | Grade 0 `diff_dmg_bp` |
|---|---|---|---|---|---|
| 26 | 10100 | 10050 | 10000 | 10100 | 10050 |
| 50 | 12500 | 11250 | 10000 | 12500 | 11250 |
| 100 | 17500 | 13750 | 10000 | 17500 | 13750 |
| 300 | 37500 | 23750 | 10000 | 37500 | 23750 |
| 301 | 37600 | 23800 | 10500 | 39480 | 24990 |
| 400 | 47500 | 28750 | 60000 | 285000 | 172500 |
| 500 | 57500 | 33750 | 110000 | 632500 | 371250 |

At Grade 20 with `run.marg.thick_vellum` at rank 5 an Editor's HP chain `[12000, 11000, marg_hp_bp(f), deep_bp(f), 15000]` first reaches `DIFF_CAP_BP` at Folio 467 (1,003,404 before the clamp). Every mutation reaches rank 5 by the 60th draw at Folio 321 (12 activations and 48 raises, one per Marginalia Volume from Folio 26). **Overflow bound:** doc 13 templates have `template_hp ≤ 400` HP. Then the largest enemy HP is `400,000 × H(300) × 400000 × 37000 × DIFF_CAP_BP` (each `mul_bp`) = 4,021,273,828,026,032,000 mHP, below `i64::MAX` (9,223,372,036,854,775,807). Damage is far from the limit: a 1,000 HP template hit through `T(300) = 1,746,060,986`, hunter damage 20000, `DIFF_CAP_BP` and three more step-2 multipliers of 13000, 15000 and 12000 is 81,715,654,144,800 mHP.

### 9.3 Marginal mutations

At the first Folio of each Marginalia Volume (`f = 26, 31, 36, ...`), before note draws (§3.3), one draw from that Folio's `rng.run` stream (0 draws once every mutation is at rank 5): while any mutation is inactive, `range(inactive_count)` over the inactive list sorted by ID activates it at rank 1. Once all 12 are active, `range(12)` over the sorted list picks one to raise by 1; if it is at rank 5, the next ID in ascending order (wrapping) below rank 5 is raised instead. When all are at rank 5, no draw happens. Rank `r` effects:

| ID | Effect at rank r |
|---|---|
| `run.marg.ambush_pages` | `ambush_extra += r` |
| `run.marg.bleeding_genres` | doc 12 bleed-zone count `+ r` |
| `run.marg.brittle_kit` | Errata hits on NPCs: step 13 factor `brittle_bp = max(7500, 10000 - 500 × r)` (§7.3) |
| `run.marg.champion_tide` | `champion_extra += r` |
| `run.marg.cramped_hearth` | `rest_heal_bp` chain `max(2500, 10000 - 1500 × r)` |
| `run.marg.dear_margins` | `shop_price_bp` chain `10000 + 1000 × r` |
| `run.marg.editors_wrath` | editor damage factor `10000 + 1000 × r` |
| `run.marg.elite_margins` | `elite_promote_add_bp += 500 × r` |
| `run.marg.hunters_pack` | hunter HP factor `10000 + 2000 × r` |
| `run.marg.lean_purse` | `lucre_bp` chain `max(5000, 10000 - 1000 × r)` |
| `run.marg.red_ink` | damage factor `10000 + 500 × r` |
| `run.marg.thick_vellum` | HP factor `10000 + 1000 × r` |

Doc 12's per-Folio Marginalia modifiers (`lattice.marg.*`, doc 12 §22) and these cumulative Marginal mutations both apply. Where they touch the same quantity (bleed zones, champions) they add, `lattice.marg.*` first.

### 9.4 Leaderboards

| Board | Metric (descending unless noted) | Tie-breaks in order | Partitions |
|---|---|---|---|
| `score.board.standard` | score | fewer `end_tick`; earlier submission | party size 1..=4; all-time and per 12-week season |
| `score.board.standard_speed` | `end_tick` ascending, victories only | higher score; earlier submission | party size; Grade 0..=20 |
| `score.board.abridged` | score | fewer `end_tick`; earlier submission | party size; all-time and season |
| `score.board.novella` | score | fewer `end_tick`; earlier submission | party size; all-time and season |
| `score.board.daily` | score | deeper Folio; fewer `end_tick` | party size; UTC date |
| `score.board.weekly_gauntlet` | best score | fewer `end_tick` | party size; ISO week |
| `score.board.endless` | deepest Folio cleared | higher score; fewer `end_tick` | party size; all-time and season |

Seasons partition display only; every entry stays in the all-time board forever. Eligible entries: ranked mode (§8.1), no easing mutator, no tick with Scribe's Mercy on (`mercy_ever_on`, doc 17), no tick with `access.game_speed_bp` below 10000 (`min_game_speed_bp`, doc 17 §13.9), no `mode.ff_off`, no `mode.loot_shared`, no `mode.invasion_open`, no `mode.audience_votes`, not Apocrypha, `restores <= RANKED_RESTORE_MAX` (§12.3), and a server-verified replay. `score.board.endless` takes Endless runs only; `score.board.standard` takes the run's final score, which for an Endless run is the §4.2 maximum. A Second Edition run posts to its structure's board, and an entry whose `RunSummary.ironman` is true shows the Ironman mark (§8.9). A season is 12 weeks; the first starts at the first Monday 00:00 UTC at or after doc 15's `T0` (the Early Access launch day, a configuration value set at launch, DECISIONS 111), and each later season starts 12 weeks after the previous one.

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
  "req_id": "0x00000000000003e9", "run_id": "0x00000000075bcd15",
  "board": "score.board.daily", "period": "2026-10-11", "party_size": 2,
  "run_seed": "0xc6f2e12ddca712ab", "mode": "mode.daily", "grade": 2,
  "mutators": ["run.mut.dear_pages"], "sim_version": 1,
  "content_hash": "9f00000000000000000000000000000000000000000000000000000000000001",
  "score": 21480, "deepest_folio": 5, "end_tick": 99000, "restores": 0,
  "final_state_hash": "1a00000000000000000000000000000000000000000000000000000000000002",
  "ranked_token": "7700000000000000000000000000000000000000000000000000000000000003"
}
```

Hex forms follow doc 16 §11.1: a `u64` (`req_id`, `run_id`, `run_seed`) is `"0x"` and 16 lowercase hex digits; a 32-byte value (`content_hash`, `final_state_hash`, `ranked_token`) is 64 lowercase hex digits without a prefix. Any other form is refused.

---

## 10. Achievements

Achievements are account-level, evaluated by the meta layer from simulation events and `RunSummary`, and mirrored to Steam achievements (DECISIONS 112). "Present" means the player's slot was Active or Ghost. "Victory" means `run.end.victory` in Standard, Abridged or Novella unless the entry names a mode. "Cumulative" counters add across runs and never reset; every other condition must happen within one run. Every achievement is disabled in Apocrypha runs and in runs with an easing mutator. Scribe's Mercy (`access.scribes_mercy`, doc 17) never disables an achievement (DECISIONS 82). Pacifist play (DECISIONS 62) is tracked per Folio in `party_kills` and `party_pacified` (§1.2): a kill is an `ev.entity_killed` whose kill credit goes to a party slot (doc 13 §11.1; a minion's kill is its owner's, and a lethal takedown that kills is a kill), and a pacification is an `ev.entity_pacified` by a party slot, whatever its doc 03 §14.3 `method`: 0 spared surrender, 1 dialogue, 2 tamed, 3 a Hacking-100 seize (doc 09 §13.5) or 4 a subdue takedown (doc 09 §9.4; counted on an entity's first subdue only, even if it later wakes).

Each achievement has a **grade** 1 to 4, and doc 15 pays it once per account: Quires 10 / 25 / 50 / 100 (doc 15 §2.4) and Ink 40 / 80 / 160 / 320 (doc 15 §2.2) for grades 1 / 2 / 3 / 4, plus `pass.xp.achievement` Codex XP. No achievement references owning Inkwell entries or Keepsakes (doc 15 kompu rule).

| ID | Condition | Grade | Quires | Ink |
|---|---|---|---|---|
| `ach.first_page` | complete Folio 1 of the account's first run (DECISIONS 137) | 1 | 10 | 40 |
| `ach.turned_page` | clear Folio 1 | 1 | 10 | 40 |
| `ach.volume_1` | present at `boss.editor_v1`'s death | 2 | 25 | 80 |
| `ach.volume_2` | present at `boss.editor_v2`'s death | 2 | 25 | 80 |
| `ach.volume_3` | present at `boss.editor_v3`'s death | 3 | 50 | 160 |
| `ach.volume_4` | present at `boss.editor_v4`'s death | 3 | 50 | 160 |
| `ach.ur_text` | Standard victory | 4 | 100 | 320 |
| `ach.abridged` | Abridged victory | 3 | 50 | 160 |
| `ach.novella` | Novella victory | 3 | 50 | 160 |
| `ach.marginalia_30` | clear Folio 30 | 2 | 25 | 80 |
| `ach.marginalia_50` | clear Folio 50 | 3 | 50 | 160 |
| `ach.marginalia_100` | clear Folio 100 | 4 | 100 | 320 |
| `ach.grade_5` | victory at Grade ≥ 5 | 3 | 50 | 160 |
| `ach.grade_10` | victory at Grade ≥ 10 | 3 | 50 | 160 |
| `ach.grade_15` | victory at Grade ≥ 15 | 4 | 100 | 320 |
| `ach.grade_20` | victory at Grade 20 | 4 | 100 | 320 |
| `ach.callings_v1` | `boss.editor_v1` killed as each of the 13 Callings (cumulative) | 3 | 50 | 160 |
| `ach.callings_win` | victory as each of the 13 Callings (cumulative) | 4 | 100 | 320 |
| `ach.origins_v1` | `boss.editor_v1` killed with each of the 12 core Origins (cumulative) | 3 | 50 | 160 |
| `ach.atlas` | clear a Folio of each of the 12 core genres (cumulative) | 2 | 25 | 80 |
| `ach.oneiric` | enter `genre.oneiric` | 2 | 25 | 80 |
| `ach.redacted` | enter `genre.redacted` | 2 | 25 | 80 |
| `ach.unwritten` | enter `genre.unwritten` | 2 | 25 | 80 |
| `ach.secret_trio` | all three of `genre.oneiric`, `genre.redacted` and `genre.unwritten` entered (cumulative) | 3 | 50 | 160 |
| `ach.anachronist` | 100 kills of native targets by a base component with `E >= 3` (cumulative) | 1 | 10 | 40 |
| `ach.shock_of_new` | trigger Shock of the New 50 times (cumulative) | 1 | 10 | 40 |
| `ach.coherent_volume` | clear Folios 1 to 5 of one run without ever gaining `st.contradicted` | 2 | 25 | 80 |
| `ach.rewritten` | 50 Rewrites committed (cumulative) | 1 | 10 | 40 |
| `ach.appraiser` | 500 item instances identified (cumulative) | 1 | 10 | 40 |
| `ach.connoisseur` | every potion type of the run's content set known in one run | 2 | 25 | 80 |
| `ach.master` | any skill reaches 100 in one run | 2 | 25 | 80 |
| `ach.triad` | three skills at ≥ 50 at once | 1 | 10 | 40 |
| `ach.level_30` | character level 30 | 2 | 25 | 80 |
| `ach.palimpsestic` | equip a Palimpsestic item | 1 | 10 | 40 |
| `ach.exotic_pair` | equip an Exotic weapon and an Exotic armor piece at once | 1 | 10 | 40 |
| `ach.crafter` | 100 `ev.item_crafted` (cumulative) | 1 | 10 | 40 |
| `ach.brewer` | 100 `ev.potion_brewed` (cumulative) | 1 | 10 | 40 |
| `ach.builder` | place 1,000 `cat.placeable` blocks (cumulative; Canon §4.3) | 1 | 10 | 40 |
| `ach.excavator` | break 1,000 destructible cells that no Erratum placed (cumulative; §3.5 rule) | 1 | 10 | 40 |
| `ach.ghost_step` | complete `run.note.silent` | 1 | 10 | 40 |
| `ach.pacifist` | clear a standard or challenge Folio with `party_pacified >= 5` and `party_kills = 0` | 2 | 25 | 80 |
| `ach.deaths_door` | 25 Death's Door saves (cumulative) | 1 | 10 | 40 |
| `ach.erasure_10` | exit a Folio while holding 10 Erasure stacks | 1 | 10 | 40 |
| `ach.hunter` | kill an Erasure hunter | 2 | 25 | 80 |
| `ach.hunter_10` | kill 10 Erasure hunters (cumulative) | 2 | 25 | 80 |
| `ach.swift_volume` | `boss.editor_v1` dies before run tick 54,000 (15 min) in Standard | 2 | 25 | 80 |
| `ach.speed_win` | Standard victory with `end_tick < 324000` (90 min) | 3 | 50 | 160 |
| `ach.party_of_four` | clear Folio 5 in a party of 4 | 1 | 10 | 40 |
| `ach.reviver` | 50 `ev.player_revived` as the reviver (cumulative) | 1 | 10 | 40 |
| `ach.full_house` | Standard victory in a party of 4 with `S_deaths = 0` | 3 | 50 | 160 |
| `ach.daily_10` | 10 ranked Daily runs ended (cumulative) | 2 | 25 | 80 |
| `ach.gauntlet` | Weekly Gauntlet victory | 3 | 50 | 160 |
| `ach.recollection` | complete one Recollection trial | 1 | 10 | 40 |
| `ach.recollection_all` | complete all 24 Recollection trials | 4 | 100 | 320 |
| `ach.bindery_half` | own 46 of the 91 Bindery nodes | 2 | 25 | 80 |
| `ach.bindery_all` | own all 91 Bindery nodes | 3 | 50 | 160 |
| `ach.notes_100` | complete 100 marginal notes (cumulative) | 2 | 25 | 80 |
| `ach.cartographer` | complete `run.note.cartographer` in 5 Folios of one run | 1 | 10 | 40 |
| `ach.going_native` | Go Native at a Colophon (Canon §5.11) | 1 | 10 | 40 |

Curated Steam achievements (about 40, DECISIONS 141) and in-game Codex marks (the long tail of milestones) paying the same grade table (10/25/50/100 Quires, 40/80/160/320 Ink). Across all 59 milestones: 21 of grade 1, 18 of grade 2, 14 of grade 3 and 6 of grade 4, paying 1,960 Quires and 6,440 Ink in total.

Achievement record:

```json
{ "id": "ach.hunter_10", "grade": 2, "cumulative": true,
  "counter": "erasure_hunter_kills", "target": 10 }
```

`counter` names the meta-layer counter, which the meta layer derives from `ev.entity_killed` of a `rank.hunter` spawned by §3.8; it is not a simulation event ID.

## 11. Onboarding: contextual tips

There is no Prologue (DECISIONS 132) and no special first-run seed (DECISIONS 136). The first real run teaches through contextual tips: the first time an Erratum meets a mechanic, a tip appears in the margin of the screen (doc 17). Each tip is dismissible and shows once per account.

The tips cover the movement kit (DECISIONS 95), attack and block, Rewriting, the Contradiction state of an item, the Erasure and hunger. Starter Callings and Origins are available from the first launch (DECISIONS 137).

`ach.first_page` is granted for completing Folio 1 of the account's first run (DECISIONS 137). Completing it sets the first-run milestone, which unlocks the First Page at the Margin.

---

## 12. Save and suspend rules

### 12.1 Files and write points

- `runs/run.sav` is a doc 02 §12.1 container of kind 1 (run suspend). It holds the Canon §2.4 header, the Canon §2.5 simulation save (including `RunState`), the replay log so far, and a version-stable **checkpoint record**, doc 02's `RunCheckpoint` with `checkpoint_version: u16 = 1`, written first: the `RunSummary` fields of §4.6 (JSON, integers only), `RunState.history`, the per-slot `lucre_norm`, `folios_present` and `quires_paid`, mode, Grade, Folio index, the party's Echo IDs and `RankedState`, all as of the latest Folio entry, used when the simulation payload cannot be loaded (§12.2). One active run per account per device. Account data lives in the separate signed meta profile (doc 02 §12.4) and is never part of `runs/run.sav`.
- **Single player** writes at every Folio entry, every `AUTOSAVE_PERIOD_TICKS` after it (non-ranked runs only), on Suspend at the exact current tick, and on application quit.
- **Co-op** writes only at Folio entry (Canon §2.5). Every client keeps a copy of the host's latest Folio-entry save for warm and cold host migration (doc 16 §9.3).

### 12.2 Suspend and resume

- Single-player Suspend works at any tick boundary. Resume continues at that tick; the final `state_hash` equals an uninterrupted run's (Canon acceptance test 32). In a `mode.invasion_open` run, Suspend is deferred while an invader occupies replay slot 4 and completes on the tick the invasion ends (Canon §2.5, doc 02).
- Co-op Suspend (host only) ends the session. Resume loads the latest Folio-entry save, so the in-progress Folio restarts from its entry; the Suspend dialog states this. Absent players' slots resume `Dormant`.
- Resume requires equal `sim_version` and `content_hash`; a `presentation_hash` mismatch only warns (Canon §2.4). On a mismatch the game offers, per doc 02 §12.3, the previous build's branch `prev` (kept 30 days), on which the run resumes normally, or **Close the Book**: the meta layer ends the run `run.end.abandoned_version` from the checkpoint record, with `victory_bp = 10000`, and grants the §4.6 payout through doc 15. The run is never ranked after a mismatch.
- Content hot reload is refused while a run is active (Canon §2.4).

### 12.3 Ranked integrity

- Each write increments `save_seq: u32` in `RankedState`. While online, a ranked run reports `(ranked_token, save_seq, state_hash)` on each write. Loading a save whose `save_seq` is below the latest reported value demotes the run to unranked.
- Ranked runs have no periodic autosave. Crash recovery (no Suspend file) loads the latest Folio-entry save, truncates the replay log to that tick and increments `restores`; a cold host migration (`FolioRestart`, doc 16 §5.7, §9.3) increments it too. A run with `restores > RANKED_RESTORE_MAX` is demoted at submission. Orderly Suspend and resume does not count as a restore.

```rust
pub struct RankedState { pub ranked: bool, pub ranked_token: [u8; 32], pub save_seq: u32, pub restores: u8 }
```

Save write points, consolidated:

| Event | Single player | Co-op host | Co-op client |
|---|---|---|---|
| Folio entry boundary | write | write | keep a copy of the host's write |
| Every `AUTOSAVE_PERIOD_TICKS` after entry | write (unranked runs only) | none | none |
| Suspend | write at the current tick boundary | end session; nothing new written | none |
| Application quit | write at the current tick boundary (doc 02 §12.3) | as Suspend | none |
| Run end | delete `runs/run.sav` after the meta layer records `RunSummary` | same | delete its copy |

---

## Exports

All IDs below are in namespaces Canon §19 assigns to doc 14. Every concrete ID is listed. All 444 pass the Canon §1.1 grammar, are at most 64 bytes, and have distinct `fnv1a32` (checked with the Canon §1.1 constants, also against every other ID in the spec).

| Namespace | IDs | Count |
|---|---|---|
| `mode.` | base modes (§8.1): `mode.standard`, `mode.abridged`, `mode.novella`, `mode.daily`, `mode.weekly_gauntlet`, `mode.seeded`, `mode.second_edition`, `mode.endless`, `mode.custom`; overlays (§8.1): `mode.ff_off`, `mode.ff_full`, `mode.loot_shared`, `mode.invasion_open`, `mode.audience_votes` | 15 |
| `grade.` | `grade.g00`, `grade.g01`, `grade.g02`, `grade.g03`, `grade.g04`, `grade.g05`, `grade.g06`, `grade.g07`, `grade.g08`, `grade.g09`, `grade.g10`, `grade.g11`, `grade.g12`, `grade.g13`, `grade.g14`, `grade.g15`, `grade.g16`, `grade.g17`, `grade.g18`, `grade.g19`, `grade.g20` (§7.1) | 21 |
| `run.end.` | `run.end.victory`, `run.end.margin_closed`, `run.end.erased`, `run.end.abandoned`, `run.end.abandoned_version`, `run.end.margin_exhausted`, `run.end.departed` (§4.3) | 7 |
| `run.rubric.` | `run.rubric.keeper`, `run.rubric.catchwords`, `run.rubric.seal` (§3.4) | 3 |
| `run.note.` | `run.note.cartographer`, `run.note.excavate`, `run.note.illuminate`, `run.note.native`, `run.note.purge`, `run.note.rescue`, `run.note.scribe`, `run.note.silent`, `run.note.swift`, `run.note.unbroken` (§3.5) | 10 |
| `run.mut.` | `run.mut.ambushes`, `run.mut.bare_kit`, `run.mut.blind_exits`, `run.mut.champion_parade`, `run.mut.dear_pages`, `run.mut.elite_ink`, `run.mut.hungry_hunters`, `run.mut.iron_ink`, `run.mut.no_rest`, `run.mut.pauper`, `run.mut.thick_skins`, `run.mut.wild_bleed`, `run.mut.paper_foes`, `run.mut.patron`, `run.mut.short_book`, `run.mut.soft_ink` (§8.7) | 16 |
| `run.marg.` | `run.marg.ambush_pages`, `run.marg.bleeding_genres`, `run.marg.brittle_kit`, `run.marg.champion_tide`, `run.marg.cramped_hearth`, `run.marg.dear_margins`, `run.marg.editors_wrath`, `run.marg.elite_margins`, `run.marg.hunters_pack`, `run.marg.lean_purse`, `run.marg.red_ink`, `run.marg.thick_vellum` (§9.3) | 12 |
| `run.boon.` | `run.boon.appraising_eye`, `run.boon.blessing`, `run.boon.deep_pockets`, `run.boon.editors_bane`, `run.boon.hunters_bane`, `run.boon.keen_edge`, `run.boon.provisioned`, `run.boon.quickened`, `run.boon.second_wind`, `run.boon.shadowed`, `run.boon.surveyor`, `run.boon.thick_hide` (§3.9) | 12 |
| `run.kit.` | `run.kit.universal`, `run.kit.vanguard`, `run.kit.duelist`, `run.kit.gunslinger`, `run.kit.grenadier`, `run.kit.spellwright`, `run.kit.alchemist`, `run.kit.shade`, `run.kit.artificer`, `run.kit.warden`, `run.kit.oracle`, `run.kit.netrunner`, `run.kit.beastbinder`, `run.kit.monk`, `run.kit.vanguard_alt`, `run.kit.duelist_alt`, `run.kit.gunslinger_alt`, `run.kit.grenadier_alt`, `run.kit.spellwright_alt`, `run.kit.alchemist_alt`, `run.kit.shade_alt`, `run.kit.artificer_alt`, `run.kit.warden_alt`, `run.kit.oracle_alt`, `run.kit.netrunner_alt`, `run.kit.beastbinder_alt`, `run.kit.monk_alt`, `run.kit.vanguard_script`, `run.kit.duelist_script`, `run.kit.gunslinger_script`, `run.kit.grenadier_script`, `run.kit.spellwright_script`, `run.kit.alchemist_script`, `run.kit.shade_script`, `run.kit.artificer_script`, `run.kit.warden_script`, `run.kit.oracle_script`, `run.kit.netrunner_script`, `run.kit.beastbinder_script`, `run.kit.monk_script` (§2.4) | 40 |
| `margin.` | facilities (§5.1): `margin.lectern`, `margin.dramatis`, `margin.index`, `margin.bestiary`, `margin.first_page`, `margin.bindery`, `margin.ledger`, `margin.wardrobe`, `margin.inkwell`, `margin.codex_desk`, `margin.store`, `margin.reliquary_vault`, `margin.vellum_archive`, `margin.recollections`, `margin.proving_page`, `margin.plot`; milestones (§5.1): `margin.milestone.first_run_end`, `margin.milestone.first_editor_v1`, `margin.milestone.first_editor_v2`, `margin.milestone.first_victory`, `margin.milestone.first_ur_text_victory`, `margin.milestone.first_standard_victory`, `margin.milestone.first_reliquary`, `margin.milestone.first_vellum`; alcove slots (§5.1a): `margin.slot.trophy_01`, `margin.slot.trophy_02`, `margin.slot.trophy_03`, `margin.slot.trophy_04`, `margin.slot.trophy_05`, `margin.slot.trophy_06`, `margin.slot.trophy_07`, `margin.slot.trophy_08`, `margin.slot.trophy_09`, `margin.slot.trophy_10`, `margin.slot.trophy_11`, `margin.slot.trophy_12`, `margin.slot.wall_01`, `margin.slot.wall_02`, `margin.slot.wall_03`, `margin.slot.wall_04`, `margin.slot.wall_05`, `margin.slot.wall_06`, `margin.slot.wall_07`, `margin.slot.wall_08`, `margin.slot.floor_01`, `margin.slot.floor_02`, `margin.slot.floor_03`, `margin.slot.floor_04`, `margin.slot.floor_05`, `margin.slot.floor_06`, `margin.slot.banner_01`, `margin.slot.banner_02`, `margin.slot.banner_03`, `margin.slot.light_01`, `margin.slot.light_02`, `margin.slot.light_03`; resident arcs (§5.6): `margin.arc.archivist`, `margin.arc.binder`, `margin.arc.recollector`, `margin.arc.cartographer`, `margin.arc.inkwell_keeper`, `margin.arc.codex_scribe`, `margin.arc.quartermaster`, `margin.arc.drillmaster`, `margin.arc.narrator` | 66 |
| `unlock.` | Bindery nodes (§5.3, 91): `unlock.calling.grenadier`, `unlock.calling.alchemist`, `unlock.calling.monk`, `unlock.calling.shade`, `unlock.calling.artificer`, `unlock.calling.netrunner`, `unlock.calling.beastbinder`, `unlock.calling.warden`, `unlock.calling.oracle`, `unlock.origin.space_opera`, `unlock.origin.post_apocalypse`, `unlock.origin.hardboiled_noir`, `unlock.origin.wuxia`, `unlock.origin.steampunk`, `unlock.origin.fairy_tale`, `unlock.origin.mythic_antiquity`, `unlock.origin.cosmic_horror`, `unlock.origin.oneiric`, `unlock.origin.redacted`, `unlock.origin.unwritten`, `unlock.origin.ur_text`, `unlock.kit.vanguard_alt`, `unlock.kit.duelist_alt`, `unlock.kit.gunslinger_alt`, `unlock.kit.grenadier_alt`, `unlock.kit.spellwright_alt`, `unlock.kit.alchemist_alt`, `unlock.kit.shade_alt`, `unlock.kit.artificer_alt`, `unlock.kit.warden_alt`, `unlock.kit.oracle_alt`, `unlock.kit.netrunner_alt`, `unlock.kit.beastbinder_alt`, `unlock.kit.monk_alt`, `unlock.loot.exotic_arms`, `unlock.loot.exotic_armor`, `unlock.loot.legendary_perks_1`, `unlock.loot.legendary_perks_2`, `unlock.loot.cross_glosses`, `unlock.loot.implants_2`, `unlock.loot.relics_2`, `unlock.loot.throwables_2`, `unlock.loot.glyphs_2`, `unlock.loot.recipes_2`, `unlock.loot.brews_2`, `unlock.loot.gadgets_2`, `unlock.room.fortune_engines_2`, `unlock.room.hireling_hall`, `unlock.room.bleed_shrine`, `unlock.room.wandering_scribe`, `unlock.room.illumination_vault`, `unlock.room.duel_page`, `unlock.event.genre_storms`, `unlock.option.seed_journal`, `unlock.option.lens_pocket`, `unlock.option.dramatis_slots`, `unlock.mutator.ambushes`, `unlock.mutator.bare_kit`, `unlock.mutator.blind_exits`, `unlock.mutator.champion_parade`, `unlock.mutator.dear_pages`, `unlock.mutator.elite_ink`, `unlock.mutator.hungry_hunters`, `unlock.mutator.iron_ink`, `unlock.mutator.no_rest`, `unlock.mutator.pauper`, `unlock.mutator.thick_skins`, `unlock.mutator.wild_bleed`, `unlock.mutator.paper_foes`, `unlock.mutator.patron`, `unlock.mutator.short_book`, `unlock.mutator.soft_ink`, `unlock.facility.proving_page`, `unlock.binding.vellum`, `unlock.binding.lapis`, `unlock.binding.verdigris`, `unlock.binding.oxblood`, `unlock.binding.gilt`, `unlock.binding.soot`, `unlock.palette.mythic_fantasy`, `unlock.palette.gothic_horror`, `unlock.palette.cosmic_horror`, `unlock.palette.space_opera`, `unlock.palette.cyberpunk`, `unlock.palette.post_apocalypse`, `unlock.palette.weird_west`, `unlock.palette.hardboiled_noir`, `unlock.palette.wuxia`, `unlock.palette.steampunk`, `unlock.palette.fairy_tale`, `unlock.palette.mythic_antiquity`; content sets (§2.3): `unlock.set.starter`, `unlock.daily_baseline`; Script grants (§2.3, 48): `unlock.room.lost_chapter_mythic_fantasy`, `unlock.room.lost_chapter_gothic_horror`, `unlock.room.lost_chapter_cosmic_horror`, `unlock.room.lost_chapter_space_opera`, `unlock.room.lost_chapter_cyberpunk`, `unlock.room.lost_chapter_post_apocalypse`, `unlock.room.lost_chapter_weird_west`, `unlock.room.lost_chapter_hardboiled_noir`, `unlock.room.lost_chapter_wuxia`, `unlock.room.lost_chapter_steampunk`, `unlock.room.lost_chapter_fairy_tale`, `unlock.room.lost_chapter_mythic_antiquity`, `unlock.hireling.chapter_mythic_fantasy`, `unlock.hireling.chapter_gothic_horror`, `unlock.hireling.chapter_cosmic_horror`, `unlock.hireling.chapter_space_opera`, `unlock.hireling.chapter_cyberpunk`, `unlock.hireling.chapter_post_apocalypse`, `unlock.hireling.chapter_weird_west`, `unlock.hireling.chapter_hardboiled_noir`, `unlock.hireling.chapter_wuxia`, `unlock.hireling.chapter_steampunk`, `unlock.hireling.chapter_fairy_tale`, `unlock.hireling.chapter_mythic_antiquity`, `unlock.glyph.ember`, `unlock.glyph.hollow`, `unlock.glyph.mirror`, `unlock.glyph.storm`, `unlock.glyph.thorn`, `unlock.glyph.tide`, `unlock.hireling.apothecary`, `unlock.hireling.cartographer`, `unlock.hireling.lamplighter`, `unlock.hireling.porter`, `unlock.hireling.sellsword`, `unlock.kit.vanguard_script`, `unlock.kit.duelist_script`, `unlock.kit.gunslinger_script`, `unlock.kit.grenadier_script`, `unlock.kit.spellwright_script`, `unlock.kit.alchemist_script`, `unlock.kit.shade_script`, `unlock.kit.artificer_script`, `unlock.kit.warden_script`, `unlock.kit.oracle_script`, `unlock.kit.netrunner_script`, `unlock.kit.beastbinder_script`, `unlock.kit.monk_script`; invader kit grants (§5.3a, 3): `unlock.invader.annotator`, `unlock.invader.censor`, `unlock.invader.footnoter` | 144 |
| `echo.` | `echo.fenrir`, `echo.wren_long_index`, `echo.lilith`, `echo.mother_ilse`, `echo.jormungandr`, `echo.drowned_clerk`, `echo.ostra_vey`, `echo.kestrel_unit`, `echo.nix_seventeen`, `echo.ronin_valdez`, `echo.ma_rust`, `echo.tallow`, `echo.josephine_crowe`, `echo.preacher_amos`, `echo.dex_halloran`, `echo.vera_lux`, `echo.lan_ninth_peak`, `echo.sun_wukong`, `echo.ada_brassvane`, `echo.colonel_wexley`, `echo.briar_thornwood`, `echo.wolf_in_wool`, `echo.kallias_twice_drowned`, `echo.loki` (§6.3) | 24 |
| `score.` | components (§4.2): `score.progress`, `score.editor`, `score.speed`, `score.explore`, `score.notes`, `score.treasure`, `score.hunters`, `score.deaths`; boards (§9.4): `score.board.standard`, `score.board.standard_speed`, `score.board.abridged`, `score.board.novella`, `score.board.daily`, `score.board.weekly_gauntlet`, `score.board.endless` | 15 |
| `ach.` | `ach.first_page`, `ach.turned_page`, `ach.volume_1`, `ach.volume_2`, `ach.volume_3`, `ach.volume_4`, `ach.ur_text`, `ach.abridged`, `ach.novella`, `ach.marginalia_30`, `ach.marginalia_50`, `ach.marginalia_100`, `ach.grade_5`, `ach.grade_10`, `ach.grade_15`, `ach.grade_20`, `ach.callings_v1`, `ach.callings_win`, `ach.origins_v1`, `ach.atlas`, `ach.oneiric`, `ach.redacted`, `ach.unwritten`, `ach.secret_trio`, `ach.anachronist`, `ach.shock_of_new`, `ach.coherent_volume`, `ach.rewritten`, `ach.appraiser`, `ach.connoisseur`, `ach.master`, `ach.triad`, `ach.level_30`, `ach.palimpsestic`, `ach.exotic_pair`, `ach.crafter`, `ach.brewer`, `ach.builder`, `ach.excavator`, `ach.ghost_step`, `ach.pacifist`, `ach.deaths_door`, `ach.erasure_10`, `ach.hunter`, `ach.hunter_10`, `ach.swift_volume`, `ach.speed_win`, `ach.party_of_four`, `ach.reviver`, `ach.full_house`, `ach.daily_10`, `ach.gauntlet`, `ach.recollection`, `ach.recollection_all`, `ach.bindery_half`, `ach.bindery_all`, `ach.notes_100`, `ach.cartographer`, `ach.going_native` (§10) | 59 |

Reserved patterns for later Scripts and Folio Expansions (doc 15 §3.1.2, validated by §2.3): `unlock.kit.<calling>_<name>`, `unlock.glyph.<family>`, `unlock.room.<name>`, `unlock.hireling.<type>`, `run.kit.<calling>_<name>`. The launch instances are listed above.

Names this chapter fixes inside other owners' namespaces: `trait.echo.<suffix>`, `_b`, `_c` (doc 04's namespace, authored here at doc 04's request); `cosm.echo.<suffix>.{palette, outfit, frame, title, trail, recollected}`, `cosm.binding.<b>`, `cosm.palette.<g>` (doc 15's catalog).

Data constants exported: the §1.1 table (including `Q_MIX`, `UNLOCK_PACE_HOURS` and `ARC_BEATS`), `grade_bp(G)`, `RUN_SCORE_BP = 1460`, `RUN_SCORE_FOLIO_CAP = 800`, `MUTATOR_BP_CAP = 20000`. Functions exported to other docs: `d(f)`, `v(f)`, `pos(f)` (§3.1), `diff_hp_bp`, `diff_dmg_bp`, `m13` (§7.3), `run_score(slot)` (§4.6); the §4.7 closing-manuscript inputs; the §5.7 Glyph Library picks. Types exported to docs 02, 15, 16 and 18: `RunState`, `ErratumChoice`, `SlotState`, `FolioRecord` and `FolioRun` (§1.2), `RunSummary` (§4.6) and the effects of the `AudienceResult` session intent (§8.1).

## Interface assumptions

| Doc | This chapter assumes it provides |
|---|---|
| 02 | save container kind 1 and the `runs/` directory; `prev` branch flow on version mismatch; intents `ChooseExit { door }` and `CommitOffer` applied in `(slot, action_seq)` order and logged in the replay (Canon §2.4); headless verifier for §9.4 |
| 03 | `sys.run_layer` as system 34a, after `sys.despawn` (34) and before `sys.tick_end` (35) (doc 03 §2.2); `sys.tick_begin` emits `ev.folio_entered`; `protect` bits for the sealed Colophon entrance; Ghost movement (§5.12); `is_errata_placed(c)` for the §3.5, §6.2 and §10 block counts; the §14.3 payloads of the §3.11 events |
| 04 | Erratum creation and Echo presets; the §2.1 creation output carried in `ErratumChoice::Custom` (kin, Origin, Calling, the 24 final sub-attribute values in Canon §6.1 order and the human +5 skill, §1.2), validated before Commit or `SlotJoin`; `trait.echo.<suffix>` variants A, B and C for all 24 Echoes under its power-budget validator, including `trait.echo.fenrir` (`damage_dealt.unarmed` add_bp 2000, broad → 2; `buy_price` mul 11000, broad → 1; 2 / 1 / 1) `trait.echo.sun_wukong` (`damage_dealt.polearm` add_bp 1500, broad → 2; `move_speed` add_bp 500, global → 1; `damage_taken` add_bp 500, global → 1; 3 / 1 / 2) and `trait.echo.loki` (`spell_potency.illusion` add_bp 2000, broad → 2; `prayer_healing` mul 7500, narrow → 1; 2 / 1 / 1); the §13 body plans `body.quadruped` (Fenrir) and `body.serpentine` (Jörmungandr) for launch Echoes, with beastkin's added tail skipped on a plan that already has `bp.tail`; the §2.5 role kits this chapter binds; level-up allocation for joiners; Ghost needs frozen; Going Native recomputation (Canon §5.11); the §2.4.2 bindings of its §2.5 roles, with "field alembic" bound to `apparatus.mortar` plus `apparatus.alembic` at tier 1 because doc 08 §9.1 needs a mortar to brew; kit balance over the 209 to 414 Lucre spread of §2.4.2; the Shade role "lockpicks" as one `item.tool.lockpicks`, a carried tool (doc 09 §13.2) ; no standard kit carries a pick; character XP per §6.2 with ghosts sharing kill XP (Canon §8) |
| 05 | the exported IDs §2.4 uses: `arch.melee.mace`, `arch.melee.warhammer`, `arch.melee.blade_long`, `arch.melee.blade_short`, `arch.melee.dagger`, `arch.melee.fist`, `arch.melee.spear`, `arch.melee.quarterstaff`, `arch.melee.axe`; `arch.ranged.pistol`, `arch.ranged.pistol_heavy`, `arch.ranged.smg`, `arch.ranged.rifle_auto`, `arch.ranged.rifle_marksman`, `arch.ranged.beam`, `arch.ranged.thrown`, `arch.ranged.grenade_frag`, `arch.ranged.grenade_smoke`, `arch.ranged.staff`, `arch.ranged.wand`; `arch.offhand.shield_buckler`, `arch.offhand.shield_heater`, `arch.offhand.shield_tower`, `arch.offhand.focus`, `arch.offhand.tome`, `arch.offhand.lantern`; `arch.armor.<slot>_<class>` for the slots head, body, hands, legs and feet and the classes cloth, light, medium and heavy; `ammo.light`, `ammo.heavy`, `ammo.rifle` and `ammo.cell` with carry caps 240, 180, 60 and 300 and their §8.1 values; the §5.6 role names; the unarmed function `arch.melee.unarmed` with bite, claw and pounce skins for `body.quadruped` and bite, constrict and spit skins for `body.serpentine`; step 13 reads `m13` (§7.3) |
| 06 | `glyph.ess.flame`, `glyph.ess.frost`, `glyph.ess.storm`, `glyph.form.aura` and `glyph.form.chain`; the run-start glyphs of §8.1 and `ev.glyph_learned` with its source codes; a Glyph Library row in §8.1's learning sources for the 2 picks of §5.7 (source 0, after the run-start set); the 52-glyph `base` pool; 4 glyphs in each of `family.ember`, `family.hollow`, `family.mirror`, `family.storm`, `family.thorn` and `family.tide` behind `unlock.glyph.<family>`; per-patron favor for `top_patron` (§4.7) |
| 07 | `gadget.turret_light` and `gadget.drone_scout`; the resource node `block.node.timber` and the placeables `block.ladder.wood` and `block.bridge.plank`; placement of `cat.placeable` items only (Canon §4.3); `block.tag.destructible` on every cell the §3.5, §6.2 and §10 counts accept |
| 08 | `apparatus.mortar` and `apparatus.alembic` at tier 1 (values 10 and 15) and `apparatus.vessel.flask` (2); the kit and lore types `alch.potion.healing` (30), `alch.potion.toxic_gas` (15), `alch.potion.purity` (20), `alch.potion.fortitude`, `alch.potion.haste`, `alch.potion.calm`, `alch.potion.aether`, `alch.potion.cure`, `alch.potion.honeyed_healing`, `alch.potion.invisibility` and `alch.potion.shielding`; `alch.bomb.firebomb` (12) |
| 09 | `StealthWorld.folio_alerted`, read for `run.note.silent` on the Rubric-completion tick (§3.5); subdue takedowns (§9.4) and Hacking-100 seizes (§13.5) reported as `ev.entity_pacified`, and a lethal takedown that kills as a kill, for `party_pacified` and `party_kills` (§10) |
| 10 | `appr.reason.kit` for kit and lore type knowledge; boon `run.boon.appraising_eye` raises pickups to `know.appraised` |
| 11 | the base values and stack caps §2.4 uses (§3: `arch.ranged.thrown` 6 per unit and at most 20 per stack, `arch.ranged.grenade_smoke` 10, `item.throwable.counter_disruptor` 14, `item.tool.lockpicks` 40; throwables and potions at most 10); `item.food.ration`; `item.tool.bandage`, `item.tool.pick`, `item.tool.lockpicks`, `item.tool.toolkit`, `item.tool.hacking_deck`, `item.tool.rope` and `item.tool.torch`; `item.text.mending`, `item.text.blink`, `item.text.mapping`, `item.text.demolition`, `item.text.appraisal`, `item.text.recharge`, `item.text.sanctify` and `item.text.summoning`; `item.throwable.counter_disruptor`; `item.book.primer_evocation` and `item.book.primer_abjuration`; `item.trinket.lattice_lens` and `item.trinket.holy_symbol`; `item.face.mask`, `item.back.cloak` and `item.belt.sash`; `item.quest.catchword`, `item.quest.folio_leaf` and `item.remains`; `loot.volume_reward`; `offer_rng(slot, context_uid, purpose)` with advance-by-one-on-commit (§11.2); `loot.pool.<suffix>` tags renormalized inside rarity bands; the `lucre_bp`, `shop_price_bp` and `reliq_key_delta` hooks; `run.boon.deep_pockets` on fresh piles only (§9.1); dropped piles as the only Lucre that passes between party members; Colophon shop stock |
| 12 | `room.incipit`, `room.colophon`, `room.interleaf`. No Prologue rooms exist (DECISIONS 132); doc 12 keeps `GenInput.prev_genre` for Folio 1. |
| 13 | `boss.editor_v1` to `boss.editor_v4`, `boss.ur_editor`, `npc.companion.<origin_suffix>`; `keeper` flag; Seal waves; `elite_promote_add_bp`; `champion_extra`; `telegraphed` attack flag; `final_phase_extra`; `sys.erasure` honouring `clock_start`, `colophon_entry_tick`, the 10-stack cap and the single Erasure hunter of §3.8 (`ev.hunter_spawned` at most once per Folio); Colophon desecration hunters (`enemy.editors.revisor`); every `template_hp ≤ 400` HP; kill XP × 16667 in Abridged; the hireling type behind each `unlock.hireling.*` ID of §2.3; the Redactor invader of a `mode.invasion_open` run (Canon §17.9); the Editor kits `enemy.editors.kit.annotator`, `enemy.editors.kit.censor` and `enemy.editors.kit.footnoter` gated by §5.3a, and `enemy.editors.kit.revisor` for every account; the eight `npc.margin.*` residents of §5.6; kill credit (§11.1) and `ev.entity_pacified` for `party_kills` and `party_pacified`; the 5 revised Editors and the Author (`boss.editor_v1_rev` to `boss.editor_v4_rev`, `boss.ur_editor_rev`, `boss.author`; DECISIONS 103) placed with no draw by doc 13 §9.4 (a revised Volume Editor by the fused pair's dominant axiom at `n = 5` and `n = 10`; `boss.ur_editor_rev` then the Author at `n = 15`), the Author's death as the Final Folio's Editor death, and the fused-pair rosters of Second Edition Folios (§8.9); the Erasure hunter tested as `rank.hunter` by every hunter-scoped rule (§3.8) |
| 15 | Quires `min(400, floor_div(run_score, 50))` per run; achievement grades paid at 10/25/50/100 Quires and 40/80/160/320 Ink; Ink, Codex XP and Reliquaries from `RunSummary`; Folds and kit A/B rule; `cosm.*` catalog entries named in Exports, including `cosm.margin.*` decorations for §5.1a; Apocrypha flag; Inkwell reference of 19.6 hours (slot A) and 13.9 hours (slot B); the §3.1.2 Script catalog granting exactly the 48 `unlock.` IDs of §2.3; the Echo binding of §6.3 with the five public-domain figures in the places §6.3 gives them; `juris.flag.invasion_minor_default`; the per-slot counters `victory_present`, `kills_present`, `types_identified`, `crafts`, `revives` and `codex_entries_new` as its §6.2 defines them, `active_ticks` by its §2.2 active-tick rule, `run_id` (§10.4) and the Satchel rules behind `RunState.satchels` (§5.5); invader rewards of Quires and Red Ink marks only; 400 play Ink per reference hour and 19.6 hours per featured Illuminated Echo, which §5.4a uses; the Transcription counter and shelf (§2.8.3, §3.9.1: 768 qualified exits for the 36 launch Scripts) behind §5.4a's Script row; the exclusive Ironman cosmetic of §8.9 |
| 16 | lobby, Dormant detection, the session intents of §5.7 with the effects of §8.8, host migration per §9.3 (hot, warm and cold), ranked tokens (refused for `mode.invasion_open` and `mode.audience_votes`), the headless verification host, invasion sessions on replay slot 4 with the §9.13 end reasons, the §11.1 hex forms, `RunView` built from `FolioRun` (§1.2); `LobbyPick` and `JoinCharacter` carrying the §1.2 `ErratumChoice` plus `appearance: Option<Appearance>` and the display name beside it, relayed in `PartyState` and stored in the replay file but never in simulation state (§1.3); the audience vote service with the session intent `AudienceResult { kind, choice }` (§5.7 kind list), applied before player intents, which closes an open boon vote without a result on a reroll (§8.1, DECISIONS 104); the trace service storing marginal notes and ink-stain death echoes by run seed and Folio genre, with no same-seed trace sent to a ranked run (§8.1, DECISIONS 96) |
| 17 | Margin screens and the Margin dialogue box that shows §5.6's arc beats as text; the trophy arranger over the 32 `margin.slot.*` records of §5.1a; Glyph Library picks in the Dramatis; door previews and forecasts; Ghost spectator UI; boon altar UI; `access.scribes_mercy`, which never blocks achievements, milestones, Recollections or Grade unlocks and whose `mercy_ever_on` keeps a run off the boards; `access.game_speed_bp`; the closing manuscript on the run-end screen; the "Second Edition" door with its stakes in place of a genre preview and the Ironman mark (§8.9); the `mode.audience_votes` lobby toggle, the vote panels of the three §8.1 kinds and the streamer-safe mode that hides names and notes; the trace opt-in, note writing and trace display (§8.1) |
| 18 | art, voice and source records for the five public-domain Echoes (§6.3); the alcove's fixed architecture and slot dressing (§5.1a); the text of every §5.6 arc beat (6 per `margin.arc.*` record, text only); the endings and the closing manuscript built from §4.7's inputs (§8.5, §8.5a), reading the deed template and name seeds from `FolioRecord.story` and the solve tick from `story_solved`; the second ending layer after a Second Edition victory (§8.5, DECISIONS 100), with the §4.7 Second Edition inputs; `Appearance` and the `art.preset.<kin_suffix>.<nn>` records (§5.6) read from the §1.3 presentation data; the grammar for trace notes (§8.1) |

## Canon gaps

Each item gives the exact canon text requested for canon-1.12; this chapter is already written to it.

1. **Offer context for room fixtures.** Canon §3.5 defines an offer's `context_uid` as "the uid or `EntityId` of the offering object". The Colophon boon altar is a room fixture with neither, so §3.9 uses the Folio index `f`, which names the one altar of each Folio. Requested, Canon §3.5: replace "`context_uid` is the uid or `EntityId` of the offering object" with "`context_uid` is the uid or `EntityId` of the offering object, or the Folio index for a room fixture that has neither (the Colophon boon altar, doc 14 §3.9)".
2. **Glyph Library picks in baseline runs.** Canon §17.1 `GLYPH_LIBRARY_START` has each Erratum pre-learn 2 glyphs "chosen from" the account's Glyph Library, while Canon §13.6 runs Daily and Weekly runs on `unlock.daily_baseline`, which holds "no account unlocks". Applied (§5.7): runs on the baseline offer the 52 glyphs of doc 06's `base` pool in place of the account library, so every player of a shared seed has the same choice. Requested, Canon §17.1 `GLYPH_LIBRARY_START` row: append "; runs on `unlock.daily_baseline` (§13.6) offer the 52 glyphs of doc 06's `base` pool in place of the account library (doc 14 §5.7)".
3. **Erasure clock in boss Folios.** Canon §17.1 `ERASURE_STEP` grants stack `k` on tick `folio_entry_tick + 43200 + 1800 × (k - 1)`, while §11.6 pauses accrual in boss Folios while the boss lives without saying whether the pause shifts that schedule. Applied (§3.8): a boss Folio's clock starts at the Editor's death tick (`clock_start`), as docs 13 and 17 also read it. Requested, Canon §17.1 `ERASURE_STEP` (doc 17 G29 proposes the same): replace "on tick `folio_entry_tick + 43200 + 1800 × (k - 1)`, so the tenth stack lands 990 s after entry" with "on tick `clock_start + 43200 + 1800 × (k - 1)`, where `clock_start` is the Folio entry tick, or the Editor's death tick in a boss Folio (doc 14 §3.8), so the tenth stack lands 990 s after `clock_start`".
4. **Second Edition structure and depth.** DECISIONS 105 and 106 (the latter names Canon §13.8) set 15 Folios in 3 Volumes after the Ur-Text at depth `25 + 2n`, and DECISIONS 117 sets the per-pair no-repeat rule, but Canon §13.7, §13.8 and §17.1 define depth, item level, `FINAL_FOLIO`, `BOSS_FOLIOS`, `GENRE_REPEAT_RULE` and `ENH_CAP` only for the main run, Abridged and Marginalia (doc 11 Canon gaps, item 2). Applied (§3.1, §8.9). Requested:
   - Canon §17.1, a paragraph after **Abridged mode**: "**Second Edition** (`mode.second_edition`, owner 14; DECISIONS 100 to 108, 117). An Ur-Text victory of a Standard or Abridged run may continue into 15 Second Edition Folios (doc 14 §8.9). Second Edition Folio `n = 1..=15` is run Folio `f = f_ur + n`, where `f_ur` is the Ur-Text Folio's index; `EntityId`s, uids and per-Folio streams use `f`. Folio `n` is in Volume `6 + floor_div(n - 1, 5)`, its boss Folios are `n = 5, 10, 15`, each ending on a revised Editor, with the Author after the revised Editor of `n = 15`, and its depth `d = 25 + 2n` feeds `P`, `T`, `H`, `S`, `ilvl` and `enemy_level`. Each Folio is written in two fused genres (doc 12). Second Edition Folios are not Marginalia Folios."
   - Canon §17.1 rows: `FINAL_FOLIO` becomes "25 (15 in Abridged), always `genre.ur_text`; a run continued into Second Edition ends at Folio `f_ur + 15` (doc 14 §8.9)"; `BOSS_FOLIOS` appends ", and Second Edition Folios `n = 5, 10, 15`"; `GENRE_REPEAT_RULE` becomes "no genre twice in the same Volume (in Second Edition, no fused pair twice in the same Volume); at most 3 appearances per run before Marginalia or Second Edition".
   - Canon §13.7: replace "`ENH_CAP = 15` in the main run and `15 + floor_div(d_current - 25, 5)` in Marginalia" with "`ENH_CAP = 15` while `d_current <= 25` and `15 + floor_div(d_current - 25, 5)` above it (Marginalia and Second Edition)", and replace "sets `ilvl = max(ilvl, current Folio index)`" with "sets `ilvl = max(ilvl, d_current)`".
   - Canon §13.8 first bullet: replace "Item level `ilvl` is the Folio index where the item was generated: 1 to 25 for the main run, 26 or more in Marginalia" with "Item level `ilvl` is the depth `d(f)` (doc 14 §3.1) of the Folio where the item was generated: 1 to 25 in the main run and Abridged, `f` in Marginalia, 27 to 55 in Second Edition"; §13.8.2's `gear_hp` then applies to Second Edition armor through its `ilvl` unchanged.
5. **Asynchronous traces.** DECISIONS 96 names Canon §3 and §17, which say nothing of traces. Applied (§8.1 Asynchronous traces). Requested, a new Canon §17.10 "Asynchronous traces": "Opt-in marginal notes and ink-stain death echoes that other players left in Folios of the same seed or genre (DECISIONS 96; docs 14, 16, 17) are presentation data. Like §3.8's generators, they never read or advance a simulation stream, and they never enter simulation state, saves, `state_hash`, score or any account counter. A ranked run shows no trace stored under its own seed (doc 14 §8.1)."
6. **Who remembers a discovered World Law.** Canon §5.8's Law discovery bullet says "the Codex then remembers it", while Canon §20 defines the Codex as the battle pass and triage G-197 names the account encyclopedia the Index. Applied (§5.5): discovered Laws are kept with the account's Index. Requested, Canon §5.8 (doc 17 G28 proposes the same): replace "the Codex then remembers it" with "the account's Index (doc 14 §5.5) then remembers it".
7. **Ghosts in a Redaction.** Canon §17.9's Outcome says every Erratum "respawns at the Folio entrance with 50% Max HP", while §17.6 resurrects a ghost only at the next `ev.folio_entered`. Applied (§3.10 Redaction): a Ghost slot loses its Lucre and one item like every slot, but stays a Ghost and resurrects at the next Folio entry. Requested, Canon §17.9 Outcome: replace "then respawns at the Folio entrance with 50% Max HP" with "then every Erratum that is not a ghost respawns at the Folio entrance with 50% Max HP; a ghost stays a ghost and resurrects at the next `ev.folio_entered` (§17.6)".
8. **Ghosts entering Second Edition.** DECISIONS 107 returns every ghost "with their character as it was at death", while Canon §17.6 costs a ghost whose Remains were left behind 25% Max HP and one equipped item. Applied (§8.9): at Second Edition Folio 1 a ghost resurrects as if its Remains were carried. Requested, Canon §17.6 third bullet: append "At the entry of Second Edition Folio 1 every ghost resurrects as if its Remains were carried (doc 14 §8.9, DECISIONS 107)."
9. **`rng.run` draws.** Canon §3.2's stream table limits `rng.run` to "Daily setup and Weekly mutators (`folio_index = 0`), marginal notes and Marginal mutations". This chapter also draws the audience Chaos options (§8.1) on `rng.run`'s labeled sub-stream `chaos`, which never advances the parent. Requested, Canon §3.2 `rng.run` "Draws for": "Daily setup and Weekly mutators (`folio_index = 0`), marginal notes, Marginal mutations, and the audience Chaos options on its labeled sub-stream `chaos` (doc 14 §8.1)".

## Acceptance tests

1. **Score.** The six §4.2 examples A, B, C, D, E and R score 8,350, 102,681, 196,174, 48,037, 72,756 and 20,780, give `run_score` 1,219, 14,991, 20,000 (capped from 28,641), 7,013, 7,966 and 3,033, and doc 15 grants 24, 299, 400, 140, 159 and 60 Quires. Grade-20 victories with B's, D's and E's inputs give `run_score` 20,000, 12,000 and 12,000 (per-Folio cap) and 400, 240 and 240 Quires.
2. **Endless max rule.** Example B continued with Folios 26..=39 cleared at 330 s each, 3/4 rooms, 14 more notes, Editors at 30 and 35 with 8 telegraphed hits each, Σ `lucre_norm` 9,750, 3 Erasure hunters in all (B's one and one in each of 2 Marginalia Folios), erased in Folio 40: `S_progress = 54600`, `S_editor = 25200`, `base = 105985`, score `mul_bp(105985, 15000) = 158977` > 102,681, so the final score is 158,977, `run_score` 23,210 and 400 Quires.
3. **Grade multiplier.** `grade_bp` is 10000, 13750, 20000 and 40000 at Grades 0, 5, 10 and 20. `gear_tier` is 1, 2, 3, 4, 5 at Grades 4, 5, 10, 15, 20.
4. **Erasure.** Stack 1 at clock 43,200 and stack 10 at 59,400, with no eleventh stack; the one Erasure hunter at 59,400 and no second hunter in that Folio, even after the first dies; healing ×9100 at 3 stacks. In a boss Folio entered at tick 1,000,000 whose Editor dies at 1,090,000, stack 1 arrives at 1,133,200. A party entering the Colophon at clock 48,000 holds 3 stacks for the rest of the Folio and draws no hunter. An Erratum with 4 Red Ink doses draws the hunter at clock 52,200. At Grade 11 the Erasure hunter's HP chain includes `gp.hunter_hp_bp = 12500`, a `run.boon.hunters_bane` bearer's hits on it use factor 13000, and its kill adds 300 to `S_hunters` and 1 to the `ach.hunter_10` counter.
5. **Rest.** Interleaf: Max HP 300,000 mHP, `rest_heal_bp` 7500, RES 40, 3 stacks: 102,375 mHP healed. Colophon: an Erratum at 10 stacks with 40,000 of 300,000 mHP claims the full heal and has 300,000 mHP; a second claim in the same Colophon is refused.
6. **Difficulty.** `boss.editor_v2` at Folio 10, Grade 20, solo, template 100 HP: `diff_hp_bp = 13200`, HP 18,175,344 mHP. An NPC hit of 23,000 mHP after step 12 at Grade 16 in a party of 2 ends step 14 at 29,095 mHP, and at 26,185 mHP against a `run.boon.thick_hide` bearer.
7. **Marginalia.** At f = 100 Grade-0 chains are 17500 and 13750; at f = 400 they are 285000 and 172500; an Editor at Grade 20 with `run.marg.thick_vellum` rank 5 first reaches 1,000,000 at f = 467.
8. **Overflow bound.** `mul_bp` chain `400000 → H(300) → 400000 → 37000 → 1000000` yields 4,021,273,828,026,032,000 without panic; the damage chain `1000000 → T(300) → 20000 → 1000000 → 13000 → 15000 → 12000` yields 81,715,654,144,800.
9. **Kits.** `start_run` kit generation calls no `Rng`; every kit uid has `folio_index = 0` in its top bits; the universal kit is worth 100 and the standard Calling-part values are 206, 192, 186, 314, 172, 109, 182, 248, 274, 240, 222, 138, 136 (Vanguard to Monk); the alt kits differ by 0, +208, +107, +63, 0, −92, +219, −162, 0, −334, −91, +144 and 0 Bp; the Shade kit holds exactly one `item.tool.lockpicks`; no entry exceeds its cap (`arch.ranged.thrown` 20 per stack, `ammo.cell` 300); the validator rejects a kit whose `consumable_entries` differs from the category rule and an ammunition entry no kit weapon fires; the Gunslinger standard kit has `consumable_entries = []`; a Beastbinder starts with exactly one bound minion.
10. **Margin Translation.** Cyberpunk-origin Gunslinger, ATN 40, Folio 1 Fairy Tale: sidearm `C = 4`, generated with `origin_genre = genre.fairy_tale`. Weird West origin, Folio 1 Steampunk: `C = 0`, keeps `genre.weird_west`.
11. **Daily draw.** Seed `0xc6f2e12ddca712ab`: `rng.run` stream seed `0x6c9f436c0c107704`; raw draws 3, 8, 1, 5, 4, 7, 6, 1, 4; slots get Grenadier, Oracle, Duelist, Artificer; Cyberpunk, Hardboiled Noir, Weird West, Gothic Horror; mutator `run.mut.dear_pages`; exactly 9 draws for party sizes 1 and 4 alike; Folio 1 Weird West.
12. **Weekly draw.** `2026-W41`: seed `0x2e7e2bae906b8339`, stream seed `0x54f2e7d7bdbce9f5`, raw draws 2, 0, 3, mutators `run.mut.blind_exits`, `run.mut.ambushes`, `run.mut.elite_ink`; Folio 1 Weird West.
13. **Retired seed.** No Prologue seed exists (DECISIONS 132).
14. **Exit commit.** Two voters, one idle: no commit before `open + 7200`; the active voter in door 1 from `open + 7000` onward commits at `open + 7319`. A voter inside the Colophon with `ChooseExit { door: 1 }` counts exactly like one standing in door 1, and stops counting on leaving the Colophon.
15. **Notes draw count.** 7 eligible notes: 2 draws; 2 eligible: 0 draws, both offered. A note at Standard Folio 10, Grade 0 pays 65 Lucre.
16. **Bindery.** 91 nodes summing to 7,670 Quires; shelf sums 1,300, 1,775, 780, 1,400, 725, 1,015, 675. Purchasing `unlock.calling.warden` with neither Shade nor Artificer owned fails. `unlock.origin.ur_text` is refused before `first_ur_text_victory` and allowed after an Abridged victory.
17. **Achievements.** 59 entries: 21 of grade 1, 18 of grade 2, 14 of grade 3 and 6 of grade 4, totalling 1,960 Quires and 6,440 Ink at doc 15 rates.
18. **Recollection.** Every Tier 1 trial has `est_ch` in 400..=900 and every Tier 2 trial in 900..=1600; `echo.fenrir` 470, `echo.loki` 925; sum 16,839. A third activation while 2 trials are active is refused. Completing the trial of an owned Echo grants only `cosm.echo.<suffix>.recollected`. An `echo.` record without a trial is rejected.
19. **IDs.** All 444 exported IDs pass the grammar with distinct `fnv1a32`, also against every other ID in the spec.
20. **Presence.** Score 196,174, present 18 of 25 (18 cleared): `presence_bp = 7200`, raw 20,621, `run_score` 14,400, 288 Quires. Score 48,037, present 9 of 15 (9 cleared): `presence_bp = 6000`, `run_score` 4,208, 84 Quires.
21. **Joiner.** Levels 9, 11, 12, joining Standard Folio 8 or Abridged Folio 5 (`d = 8`): level 9, majors 34, minors 17, Lucre 91.
22. **Lucre normalization.** 120 Lucre in Standard Folio 10 adds 55; Lucre picked up from a dropped pile adds 0.
23. **Co-op suspend.** Host suspends at entry tick + 9,000; on resume the state hash equals the Folio-entry save's hash.
24. **Ranked integrity.** `restores = 3` demotes at submission; loading `save_seq` 4 after 6 was reported demotes; a cold host migration (`FolioRestart`) raises `restores` by 1, and hot and warm migrations do not.
25. **Daily practice.** A second Daily the same UTC date is unranked with `mode_quire_bp = 5000`.
26. **Grade unlock.** A Novella victory at Grade 9 unlocks Grade 10; a Novella victory at Grade 10 does not unlock Grade 11; an Abridged victory at Grade 10 does.
27. **Marginal mutations.** Folio 26 makes exactly 1 draw; every mutation is at rank 5 after the draw at Folio 321, and Folio 326 makes 0.
28. **Editor rotation.** Folio 30 → `boss.editor_v1`, 45 → `boss.editor_v4`, 50 → `boss.ur_editor`, 55 → `boss.editor_v1`.
29. **Abridged structure.** `d(f)` for `f = 1..=15` is 1, 3, 5, 6, 8, 10, 11, 13, 15, 16, 18, 20, 21, 23, 25; boss Folios are 3, 6, 9, 12, 15; Folio 14's doors are one `genre.ur_text` door; Folio 15 spawns `boss.ur_editor`.
30. **Colophon gate.** Before the Rubric completes, no Erratum can enter `room.colophon` and `colophon_entry_tick` stays unset; the first entry after completion sets it without drawing; the lattice draw (3 `rng.lattice` draws) happens on the first lectern interaction. A rest Folio's Colophon is enterable from entry. A door commits after one voter holds interact on it for 180 ticks while every living Erratum, downed ones included, is inside the Colophon.
31. **Boon altar.** Opening an offer makes 3 draws on `offer_rng(slot, f, "boon")` and leaves `rng.offer.p<slot>` unchanged; reopening shows the same 3 boons; a reroll advances the stream by one `next_u64` and shows a new offer; a commit advances it by one; an offer left uncommitted at exit commit leaves the stream unchanged. A committed boon is active for exactly the next Folio.
32. **Ghost resurrection.** Remains carried through the exit: 50% Max HP, no draw. Remains left behind with 5 equipped non-weapon items: 25% Max HP and exactly one `range(5)` on `keyed_rng("rng.loot", f, ghost_entity, t0, 0)`; with none equipped: 25% and no draw. No Ghost resurrects at an Endpaper.
33. **Desecration.** Breaking one Colophon block sets `colophon_marked`; every later Colophon holds one waiting `rank.hunter`; the 4th such hunter carries an elite modifier; Colophon hunters add nothing to `S_hunters`.
34. **Mutator cap.** The 12 hard mutators chain to 64,529 Bp and `mutator_bp` is 20,000. A Custom Grade-20 `run.mut.short_book` victory (10 Folios at 300 s) scores 208,200 and grants 121 Quires at `mode_quire_bp = 2000`.
35. **Co-op modes.** A run with `mode.ff_off` or `mode.loot_shared` is unranked; one with `mode.ff_full` stays ranked; `mode.ff_off` with `mode.ff_full` is refused by the lobby.
36. **Version mismatch.** Resuming a save whose `sim_version` differs, then choosing Close the Book, ends the run `run.end.abandoned_version` with `victory_bp = 10000`, computed from the checkpoint record without loading the simulation payload.
37. **Contextual tips.** A first-encounter tip shows once per account, is dismissible, and never repeats (DECISIONS 136).
38. **Script kits.** The 13 `run.kit.<calling>_script` records differ from their standard kits by 0, +312, +215, −64, +232, −184, 0, −81, +218, 0, +90, 0 and +147 Bp (Vanguard to Monk), all within 500 Bp; kit generation for a Script kit calls no `Rng`. A Script grant equal to a Bindery node ID or to `unlock.kit.<calling>_alt` is rejected.
39. **Script grants.** The launch Script catalog grants exactly the 48 `unlock.` IDs of §2.3: 12 `unlock.room.lost_chapter_<g>`, 12 `unlock.hireling.chapter_<g>`, 6 `unlock.glyph.<family>`, 5 `unlock.hireling.<type>` and 13 `unlock.kit.<calling>_script`. None has a Quire price.
40. **Invasion overlay.** A Daily or Weekly Gauntlet lobby never shows `mode.invasion_open`; a Standard run with it gets no ranked token and is unranked; a solo run with it may be invaded; when the invader reduces the last Erratum alive and not downed to 0 HP the run does not end: every Active slot respawns at the Incipit with 50% Max HP and 0 Lucre after at most one keyed `rng.loot` item draw, downed Errata lose `st.downed` without `ev.player_revived`, a Ghost slot stays a Ghost, and `FolioRecord.invasion = 2`; a Suspend requested while the invader is present completes on the tick the invasion ends.
41. **Trophy alcove.** `margin.plot` holds exactly 32 `margin.slot.*` records (12 trophy, 8 wall, 6 floor, 3 banner, 3 light); 8 are open at `first_run_end`; `margin.slot.trophy_05` opens at 2 achievements and `margin.slot.light_03` at 48; `margin.slot.trophy_05` is centred at `(3040, 0, 5400)`; no slot record has a gameplay stat or a Quire price.
42. **Public-domain Echoes.** `echo.fenrir` has `body.quadruped` and its standard Monk kit places `arch.armor.hands_light` in the bag; `echo.jormungandr` has `body.serpentine` and places `arch.armor.legs_light` and `arch.armor.feet_light` in the bag; every Calling has 1 to 3 Echoes.
43. **Ghost XP.** In a party of 2 with one Ghost, a kill paying `kill_xp = 1000` gives each slot `floor_div(1000 × 10000, 12500) = 800`, the Ghost included; a Dormant slot gets 0.
44. **Scribe's Mercy.** A Standard victory at Grade 5 with Mercy on for part of the run unlocks Grade 6 and grants `ach.ur_text` and `ach.grade_5`, advances the active Recollection trials, and is ineligible for every board (`mercy_ever_on`).
45. **Rubric draw.** A standard Folio's Rubric is `stage_rng(run_seed, f, genre, "rubric", 0).weighted([5000, 3000, 2000])`; the draw leaves the Folio's `rng.worldgen` stream unadvanced; a rest or boss Folio makes no Rubric draw.
46. **Glyph Library.** A glyph learned from a tome in run 1 joins the account library at run end and can be picked in run 2, where it is learned at run start with origin = the Erratum's Origin genre and no draw; a third pick, or a pick of `glyph.form.bolt` (a doc 06 run-start glyph), is refused; a Daily offers only `base`-pool glyphs; an Apocrypha run adds nothing to the library.
47. **Resident arcs.** After the first run ends, exactly beat 1 of `margin.arc.binder` is available; buying 20 Bindery nodes before the next run makes only beat 2 available at the next run end, and beat 3 at the run end after that. `margin.arc.codex_scribe` has beat 2 available after the fifth run end and beat 3 after the fifteenth. No run end makes two beats of one arc available, and no beat condition names Ink, Vellum, a store purchase or a Codex tier.
48. **Unlock pace.** The 73 non-cosmetic Bindery nodes cost 7,070 Quires, 70.0 hours at `Q_MIX = 101`; Σ `est_ch` / `RECOLLECTION_ACTIVE_MAX` = 8,419 ch, 84.2 hours; the 36 launch Scripts cost 768 qualified exits, 76.8 hours at 10 Folios per hour; all three are at most 85.0 hours.
49. **Invader kits.** An account without `first_editor_v1` may invade only with `enemy.editors.kit.revisor`; `first_editor_v2` grants `unlock.invader.censor` with no Quire cost and no Bindery total changes; an invasion advances no milestone, achievement, Recollection, arc, Grade or Glyph Library of the invader's account.
50. **Run-layer events.** An exit commit with 2 completed notes and 2 Active slots emits, on the commit tick and in this order, 4 `ev.note_completed`, 1 `ev.exit_committed` and `ev.folio_exited`; the first Colophon entry of a Folio emits `ev.colophon_entered` with `first = true` and sets `colophon_entry_tick`.
51. **Submission hex.** The §9.4 example passes doc 16 §11.1's format checks; the same submission with an uppercase `run_seed` or a `"0x"`-prefixed `content_hash` is refused.
52. **Pacifist.** A challenge Folio cleared with 5 `ev.entity_pacified` by party slots and no kill credited to a party slot grants `ach.pacifist`; one kill credited to a party member's minion in that Folio, or one lethal takedown that kills, prevents it; a boss Folio never qualifies.
53. **Second Edition.** A Standard victory's `VictoryChoice` lists the Margins doors (when unlocked), one "Second Edition" door and "Close the Book"; an Abridged victory's lists "Second Edition" and "Close the Book"; a Seeded run's lists neither Margins nor Second Edition doors. Committing it from Standard sets `final_folio = 40`; Second Edition Folio 3 is run Folio 28 with `d = 31`, Volume 6 and position 3; Folio 30 is a boss Folio and draws no Marginal mutation; `marg_hp_bp(30)` is 10000. A party wipe in Second Edition Folio 7 ends the run `run.end.erased` with `victory_bp = 10000`, reports `victory_present = 0` for every slot and grants no `ach.ur_text` and no Grade; a victory with no `ev.player_died` in Folios 26 to 40 sets `RunSummary.ironman`. The door commit makes no draw. Folio 40 records `FolioRecord.editor = boss.ur_editor_rev`, and Folios 30 and 35 record two different revised Volume Editors (doc 13 §9.4). At Grade 20 the Author in Folio 40 gets `final_hp_bp` and `boss.ur_editor_rev` before it does not; Folio 40 ends in `Endpaper` only when the Author dies. A slot that is a Ghost at the Ur-Text exit, with its Remains left behind, enters Folio 26 at 50% Max HP with no item draw. A Second Edition Folio of Wuxia and Gothic Horror records `genre = genre.gothic_horror` and `fused = genre.wuxia`.
54. **Early Access scope.** The Early Access content sets, counts and draws equal 1.0's: the account set can hold all 12 core Origins and 24 Echoes, the Daily's Origin draws are `range(12)`, `ach.atlas` needs 12 core genres, and a Standard or Abridged Ur-Text victory offers the "Second Edition" door. A build that does not yet ship invasions or audience votes shows neither `mode.invasion_open` nor `mode.audience_votes` in the lobby.
55. **Audience votes.** A Standard lobby with `mode.audience_votes` gets no ranked token and its run reaches no board. A door result sets slot 0's `ChooseExit` vote, and the host standing in another door volume overrides it; no door vote opens in `VictoryChoice`. A boon result commits slot 0's stored boon with exactly one `next_u64` on slot 0's `rng.offer.p<slot>` and has no effect when slot 0 has already committed. A Chaos result naming `run.mut.iron_ink` in Folio 7 puts its damage factor 15000 into Folio 8's `diff_dmg_bp` chain and not Folio 9's, and leaves `mutator_bp` and `RunState.mutators` unchanged; the Chaos options of Folio 7 come from the `chaos` sub-stream and leave its note draws unchanged; a Chaos result logged after Folio 7's exit commit has no effect. A replay of the run, with its logged `AudienceResult` intents, reproduces every state hash.
56. **Presentation data.** Two runs that differ only in a slot's `Appearance` and display name have the same `content_hash` and the same `state_hash` sequence; `ErratumChoice` holds neither; the `RunSummary` slot records the `Appearance` of a Custom Erratum and `null` for an Echo.
57. **Traces.** Leaving and viewing traces changes no `state_hash`, score or account counter; a ranked Daily shows no trace stored under its own seed and shows genre traces stored under other seeds.
58. **Origin fallback.** In a build that does not ship the host Origin's genre, Folio 1 of an unseeded run is the shipped core genre with the greatest doc 12 §5.2 edge weight from that genre, ties to the first in ascending ID order. The draw before the depth-25 Folio always finds 3 eligible candidates, however the Volume and run-cap rules stand.

## Invariants annex

```
Σ doc14: start ⊗ folio-flow ⊗ colophon ⊗ score ⊗ run_score ⊗ bindery ⊗ echoes ⊗ grades ⊗ modes ⊗ marginalia ⊗ saves
W! score := f(sim accumulators) ∧ replay(score) == live(score)       N! float ∈ score ∪ run_score
W! Quires := doc15(run_score)                                         N! second Quire formula ∈ doc14
W! run_score ≤ 800 × cleared_present                                  N! short structure out-earns Standard per hour at the cap
W! Bindery cost ∈ cur.quire                                           N! Bindery cost ∈ cur.ink ∪ fiat
W! ∀ unlock: effect ∈ {option, pool, cosmetic, convenience}           N! unlock → stat ∪ drop-rate ∪ rarity weight
W! kit generation draws = 0                                           W! kit item = common ∧ normal ∧ +0 ∧ identified
W! |value(alt kit) - value(std kit)| ≤ 500 Bp × value(std kit)        N! kit role ≠ doc04 §2.5 role
W! exit draw tick == first lectern interaction tick                  N! exit offers reroll
W! Colophon entrance sealed until Rubric | Editor death                N! Erasure stack after colophon_entry_tick
W! boss Folio: erasure clock starts at Editor death                   N! erasure stacks on Ghosts
W! Ghost resurrects only at ev.folio_entered                          W! Remains carried ⇒ 50% HP ∧ 0 draws
W! boon lasts exactly one Folio ∧ 1 active per Erratum               N! boon → §6.4 formula | canon constant | account reward
W! co-op save only at Folio entry                                     N! co-op save mid-Folio
W! ranked ⇒ server replay verified ∧ restores ≤ 2                     N! easing mutator ∨ scribes_mercy ∨ ff_off ∨ loot_shared ∨ Apocrypha on boards
W! diff_hp_bp, diff_dmg_bp ≤ DIFF_CAP_BP ∧ mutator_bp ≤ 20000          N! i64 overflow at any Folio ≤ 9999
W! Recollection est_ch ∈ tier bound                                   N! Recollection progress while inactive
W! Daily: one ranked attempt per account per UTC date                 N! streak penalty for missed days
W! departed payout paid once ∧ subtracted on rejoin                   N! double payout
W! account data ∉ palimpsest_sim state                                N! Margin draws from simulation streams
W! Margin alcove = 32 fixed slots                                     N! block placed or broken in the Margin
W! Erasure stacks ≤ 10 ∧ Erasure hunters ≤ 1 per Folio              N! second Erasure hunter in a Folio
W! ghosts ∧ downed ∈ kill-XP split                                   N! Dormant ∨ Departed ∈ kill-XP split
W! Scribe's Mercy ⇒ off the boards only                              N! Mercy blocks achievement | Grade | Recollection | milestone
W! ∀ echo: ∃ Recollection trial within its tier bound               W! non-cosmetic unlock pace ≤ 85.0 reference hours
W! arc beat := text ∧ ≤ 1 beat per arc per run end                   N! arc condition reads Ink | Vellum | store purchase | Codex tier
W! Glyph Library grows only ∧ picks ≤ 2 ∧ draws = 0                  N! baseline-run pick outside the base pool
W! invasion ⇒ invader rewards = Quires ∧ Red Ink marks               N! invasion advances the invader's account progress
W! invasion overlay ⇒ unranked                                        N! mode.invasion_open in Daily | Weekly | ranked runs
W! |value(script kit) - value(std kit)| ≤ 500 Bp × value(std kit)     N! Script grant ∈ Bindery nodes
W! d(f) feeds P, T, H, S, ilvl ∧ f feeds EntityId, uid, streams        N! Abridged stream keyed by d_eff
W! Second Edition end ≠ victory ⇒ victory_bp = 10000 ∧ no victory credit   N! Marginal mutation ∨ marg_hp_bp ≠ 10000 in Second Edition
W! ach.pacifist ⇒ party_kills = 0 ∧ party_pacified ≥ 5                N! non-cosmetic option reachable only with money
W! Appearance ∧ display name ∉ simulation state                       N! audience_votes ∨ same-seed trace in a ranked run
W! Second Edition revised Editors placed with no draw (doc 13 §9.4)   N! audience vote in VictoryChoice
W! audience result := logged session intent ∧ slot 0 only             N! Chaos mutator ∈ mutator_bp ∨ RunState.mutators
```
