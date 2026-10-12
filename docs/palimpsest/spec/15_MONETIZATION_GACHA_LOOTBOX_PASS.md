# 15 — Monetization: Inkwell Gacha, Reliquaries, Codex Pass, Store and Compliance

As of 2026-10-12. Status: BINDING. Conforms to canon-1.11 (checked against every section of `spec/00_CANON.md` canon-1.11, including §2.4 `presentation_hash`, §3.6 account draws, §14 welcome Ink, §17.8 Margin trophies and §17.9 invasions, against DECISIONS.md through row 145, and against the on-disk docs 01 to 04, 06, 10 to 14, 16 to 18 and 21).

This chapter specifies account currencies, randomized account rewards, the battle pass, the store, and the legal and ratings layer. The governing decision is DESIGN_BRIEF §8, **Earned Randomness, Paid Certainty**. Account economy state lives on the economy service (§10) and, offline, in the local client. The simulation only reports facts it already emits and receives only Keepsake item records (§5.5).

## Imports

| Source | Section | What this chapter consumes |
|---|---|---|
| DESIGN_BRIEF | §1, §4, §8 | Co-op with opt-in PvP invasions, Margin hub, horizontal meta progression, the monetization stance, the USD 19.99 base price, end-of-service guarantee |
| DECISIONS.md | 2026-10-11 rows 1, 2, 13, 15, 16, 26, 28, 32, 42, 53, 57 to 60, 74, 75, 79, 80, 86, 87, 91, 94, 98, 108, 111 to 114, 119, 122, 132, 137, 141 | Monetization kept (1); Standard plus Abridged (2); 16+ rating targets (13) with ink gore and a red-blood toggle (94); opt-in PvP invasions (15) with cosmetic-only invader rewards (32, 86); Margin decoration slots (16); the Erasure hunter's `rank.elite` multipliers (26, 91; §6.2 kill XP); the Echo cast (28); base game USD 19.99 at Early Access and 1.0 (42); the first two Folio Expansions (53); Inkwell rates (57); Codex premium (58); account Reliquaries (59); pact curses (60); lobby and chat safety (74, 75); the 80-hour unlock pace (79); a Recollection trial for every Echo (80); presentation-only mods (87); PC only (98); Second Edition stakes and the Ironman cosmetic (108); no hard release dates, so `T0` is set at launch (111); Steam as the only storefront (112); no demo and no public playtest build (113); all 12 core genres at Early Access (114, which supersedes the 6 genres of 41) with all 24 Echoes (119); Folio Expansions at USD 9.99 (122); no Prologue (132), replaced by a first-run achievement (137); about 40 curated Steam achievements with the long tail as Codex marks (141) |
| Canon | §1.1–§1.6, §2.1, §2.3 | ID grammar and `fnv1a32`; `Bp`, `Ticks`, money as `i64`; `floor_div`, `round_half_even`, `mul_bp`; integer-only content; `TICK_HZ = 60` |
| Canon | §2.4 | Session header (`sim_version`, `content_hash`, `run_seed`) carried by run reports; `content_hash` covers simulation-affecting mods only, and presentation-only mods enter `presentation_hash` (§2.2 Apocrypha rule) |
| Canon | §3.1 | `Rng`, `Rng::from_seed64`, `mix64` (Keepsake item generation seeds) |
| Canon | §3.6 | Account draws (Inkwell pulls and account Reliquaries) use doc 15's HMAC-SHA256-keyed ChaCha20 generator with commit-reveal (§4); doc 15 §5.4 alone seeds `Rng::from_seed64` outside the simulation, for Keepsake item generation |
| Canon | §11.6, §17.1, §17.2, §17.3, §17.4 | Run constants, boss Folios 5/10/15/20/25, Abridged `d_eff`, `enemy_count_bp`, rank IDs and `rank_xp_bp`, the Erasure hunter's `rank.elite` kill XP (§11.6), `REF_KILLS_PER_FOLIO = 40` (reference hour, §6.2) |
| Canon | §17.8, §17.9 | Margin trophy and decoration slots (`cosm.margin.*`); invasions: `mode.invasion_open`, invader rewards "Quires and cosmetic Red Ink marks only", chat controls with invaders (§2.2, §8.5) |
| Canon | §13.3, §13.4 | Rarity IDs and order `rar.common` … `rar.palimpsestic`; the Serendipity term of the rarity roll (never applied to account draws) |
| Canon | §13.6 | Knowledge states, unknown-appearance classes, `unlock.daily_baseline`, pact curses (§5.4) |
| Canon | §13.7, §13.8, §13.9 | Emendation re-level rule, `ilvl`, item instance contract |
| Canon | §14 | `cur.quire`, `cur.ink` (its sources include the one-time welcome grant), `cur.vellum`, `ent.quill`; nothing purchasable converts into Ink, Quires or Vellum; no currency is bought, or traded or gifted between accounts |
| Canon | §16.2 | Echo definition, 24 launch Echoes (2 per core genre), acquisition through the Inkwell or Recollection |
| Canon | §18 | `ev.folio_entered`, `ev.folio_exited`, `ev.entity_killed`, `ev.item_identified`, `ev.item_crafted`, `ev.potion_brewed`, `ev.player_revived`, `ev.run_ended` |
| Canon | §19 | This chapter owns `banner.`, `pass.`, `store.`, `juris.`, `bundle.`, `cosm.`, Echo acquisition, and the account economy rules for `cur.` and `ent.`; `echo.` and `mode.` IDs are doc 14's and are only referenced here |
| Canon | §22 | no premium currency; no real money to any randomized reward; `juris.paid_random_items_enabled = false`; Fortune and Serendipity never touch gacha odds or account currencies |
| Doc 01 | §10.3 | `kpi.econ.*` IDs for the §12 KPIs |
| Doc 02 | §3.2 crate table (`crate.econ` = `palimpsest_econ`, `crate.services` = `palimpsest_services`, `crate.platform`), §11.2 `content_hash` (REC, UNL, MOD, FLG sections), §12.4 signed meta profile, §13.1 mod classes (presentation-only or simulation-affecting), §13.4 `modset_hash`, §17.3 flag records | Shared fairness and receipt library (§4.2), economy service host crate, Apocrypha detection (§2.2), receipt cache, `juris.` flag loader (§8.2) |
| Doc 03 | §2.2 system order (`sys.tick_begin` emits `ev.folio_entered` on the entry tick), §2.4 event visibility | Satchel materialization tick (§5.5), dwell measurement (§2.2) |
| Doc 10 | §7.1 `roll_curse`, §7.8 pacts (`PACT_SHARE_BP = 2000`), §11 sealing, §12 the Index and the Bestiary | Keepsake curse and pact rolls (§5.4), sealed Keepsakes (§5.5), first Index and Bestiary entries (§6.2) |
| Doc 11 | §2.3 `generate_keepsake` entry point, `loot.keepsake.<genre_suffix>` tables | Keepsake contents (§5.4) |
| Doc 14 | §1.2 `RunState`, `SlotState`, `final_folio`; §3.1 `d(f)`; §3.9 Endpaper step 4; §4.2 score; §4.5 what is kept; §4.6 `run_score(slot)` (with `mode_quire_bp`, presence, `RUN_SCORE_BP = 1460` and the per-Folio cap `RUN_SCORE_FOLIO_CAP = 800`) and `RunSummary` (slot `active_ticks`, `folio_exits_qualified`, `editor_kills_present`, `run_score`, `quires_paid`, `apocrypha`, `ironman`); §2.3 content sets; §5.1 Margin facilities and milestones (`first_run_end`, `first_reliquary`, `first_vellum`); §5.4a unlock pace (`UNLOCK_PACE_HOURS = 80`, pace contract at most 85.0 hours); §6.1 to §6.3 the 24 Echo IDs, a bounded Recollection trial for every Echo, Tier 1 and Tier 2 trials, `est_ch`, owned-Echo Recollection granting only `cosm.echo.<suffix>.recollected`; §8.1 mode table (Standard, Abridged, Novella, Daily, Weekly Gauntlet, Seeded, Second Edition, Endless, Custom); §8.7 easing mutators; §8.9 Second Edition (`mode.second_edition`, its forfeit rule, the Ironman clear); §10 the achievements (about 40 curated Steam achievements, DECISIONS 141), the Codex marks of the long tail and their grades 1 to 4, and the first-run achievement (DECISIONS 137); reserved Script unlock patterns | Grant targets, `run_score`, Echo binding (§3.1), mode rules (§2.2), Script grants (§3.1.2) |
| Doc 16 | `svc.` services, run report transport, §9.2 kick votes, §9.12 chat and voice, §9.13 invasion outcome reports | Economy service hosting (§10), chat safety (§8.5), invader Quires (§2.4) |
| Doc 17 | UI framework, accessibility, `access.gore` | Store, odds and parental screens; the gore setting rated in §8.6 |
| R3 | B1–B6, C1–C4, D1–D6 | Hazard parameters (D2), guarantees, law and ratings table, avoid list |
| R4 | §13, §16.1 | Steam Inventory facts, backend crates (`axum` 0.8.9, `sqlx` 0.9.0, `tokio` 1.53.2, `sha2` 0.11.0, `hmac` 0.13.0, `ed25519-dalek` 3.0.0) |
| R7 | §1.4 C5 | The gacha is named Inkwell; R3's "Scriptorium" numbers carry over unchanged |
| Doc 21 | §2.5 | Folio Expansions 01 and 02 |

---

## 1. Business model and the separation wall

### 1.1 Revenue lines

Every purchase is a fiat SKU sold through the platform's payment rail. Steam is the only storefront (DECISIONS 112) and the game ships on PC only (DECISIONS 98). There is no premium currency.

| Line | SKU ID pattern | USD price | What the buyer receives | Random? |
|---|---|---|---|---|
| Base game | `store.sku.base_game` | 19.99 at Early Access and at 1.0 (DECISIONS 42) | the game; Early Access owners keep it at 1.0 at no charge | no |
| Folio Expansion | `store.sku.folio_exp_<nn>` | 9.99 (DECISIONS 122, which replaces DECISIONS 42's band of 9.99 to 14.99) | one new genre (doc 12) with its Folios, enemies and items; access to its expansion banner and Archive shelves, which hold its 2 Echoes and 3 Scripts (§3.2.6) | no |
| Codex premium track | `store.sku.codex_<nnn>_premium` | 9.99 | the premium track of one named Codex (§6) | no |
| Direct cosmetic | `store.sku.cosm_<name>` | 1.99 to 9.99 | named cosmetics listed in full before purchase (§7.1) | no |
| Supporter pack | `store.sku.supporter_pack` | 4.99 | cosmetics, digital soundtrack, digital art book (§7.1) | no |

Cadence: one Folio Expansion every 9 to 12 months after 1.0; one Codex every 12 weeks (§6.6). The first two Folio Expansions are fixed (DECISIONS 53): `store.sku.folio_exp_01` adds Age of Sail and Pirates and `store.sku.folio_exp_02` adds Kaiju and Mecha (doc 21 §2.5); doc 12 §25 lists what each genre pack carries. There is no demo and no public playtest build (DECISIONS 113).

### 1.2 The separation wall

The wall is the rule that no money, and nothing bought with money, ever reaches a random outcome or an account currency.

| Rule | Statement | Enforcement point |
|---|---|---|
| SEP-1 | `cur.ink`, `cur.quire` and `cur.vellum` are `purchasable = false`. Unopened account Reliquaries are a `purchasable = false` ledger asset. | Ledger type check (§10.2) |
| SEP-2 | No ledger posting moves value from a `purchasable = true` asset (any fiat SKU, `ent.quill`) into a `purchasable = false` asset. | Ledger type check |
| SEP-3 | Codex **premium** tiers grant only deterministic cosmetics and, at tier 100, one `ent.quill`. Ink, Quires, Reliquaries and Vellum appear only on the **free** track, which every player progresses identically whether or not they bought the premium track. | Content validator on `pass.` records |
| SEP-4 | No SKU grants Codex XP, Ink, Quires, Vellum, Reliquaries, pity progress, spark, Transcription (§3.9.1), tier skips or XP multipliers. | Content validator on `store.` and `bundle.` records |
| SEP-5 | `juris.paid_random_items_enabled = false` is a compile-time constant in the economy service and the client. A configuration that sets it true makes the service refuse to start. | Build constant plus startup assert |
| SEP-6 | Odds are identical for every account that can open a given banner. The only per-account inputs are the published pity counters. Fortune, Serendipity, spend history, play time and platform never change odds. | Pull algorithm (§3.4) reads no other account data |
| SEP-7 | No account currency, Echo, Script, cosmetic, Keepsake or Reliquary can be traded, gifted, sold or transferred between accounts. Fiat SKUs can be gifted only through the platform's own gift feature before redemption. | No transfer endpoint exists |

Every regime in R3 C1 keys loot-box rules on payment, so no PALIMPSEST random system is a paid random item anywhere. The strictest disclosure standard (Korean four-decimal odds, R3 C1 row 15) still applies voluntarily (§3.11, §5.6).

---

## 2. Account currencies, entitlements and earn rates

### 2.1 Currency table

Canon §14 defines the currencies. This section fixes their economy rules. Every amount is an `i64` whole unit (Canon §1.2). No account currency has a wallet cap, expires, decays or converts into another, except unused spark becoming Vellum (§3.6).

| ID | Purchasable | Tradeable | Sources (exact rates in §2.2 to §2.5) | Sinks |
|---|---|---|---|---|
| `cur.ink` | false | false | active play, Folio exits, Editor kills, run end or Ur-Text victory, achievements, Codex free track, welcome grant | Inkwell pulls (§3) |
| `cur.quire` | false | false | run payout from doc 14's `run_score`, achievements, Codex free track, invasions as the invader (Canon §17.9) | Margin unlock tree (doc 14 §5.3) |
| `cur.vellum` | false | false | Inkwell duplicates (§3.8), unused spark at banner end (§3.6) | Vellum Archive (§3.9) |
| `ent.quill` | true (entitlement) | false | Codex premium purchase, tier 100 of an owned premium track | redeemed for one Codex premium track (§6.5) |

Canon §14's source words map onto these rows exactly: "a one-time welcome grant" is `WELCOME_INK` (§2.2); "play time" is active-minute Ink; "run completion" is Folio-exit, Editor-kill, run-end and Ur-Text victory Ink; "Codex tiers" are free-track tiers only, because SEP-3 keeps every purchasable track free of currencies and Canon §14 forbids anything purchasable converting into Ink, Quires or Vellum. Canon §19 gives this chapter the account economy rules, so the Vellum sink list is §3.9's: its Illuminated shelves are Canon §14's "past featured Echoes, Scripts", and it adds the Rubricated shelves (from launch), the Plain shelf and Fold steps.

**Ownership.** Premium tracks, Folio Expansions, cosmetics, Echoes, Folds and Scripts are entitlement rows keyed by content ID and source (a SKU, banner, `pass.` record or the Archive), each backed by `grant` and `revoke` receipts (§4.6); none is an `ent.` ID. `ent.quill` is the only countable entitlement (Canon §14: entitlements are not currencies).

### 2.2 Ink earn rates

All in-run quantities are measured by the simulation and reported in the run report (§10.4). The economy service computes grants; the client never grants Ink.

"Present" has doc 14's meaning: the player's slot is `Active` or `Ghost` (doc 14 §1.2) at the tick in question. A `Dormant` or `Departed` slot is not present.

| Source | Trigger | Ink | Conditions |
|---|---|---|---|
| active minute | every 3,600 active ticks of the player's slot in a run | 3 | active tick definition below; at most 240 active minutes credited per run |
| Folio exit | the Folio's `exit_tick` is set (doc 14 §1.2 `FolioRecord`) | 10 per Folio | the player is present at `exit_tick`, and `exit_tick − entry_tick ≥ 5,400` (90 s) |
| Editor kill | `ev.entity_killed` of a `rank.editor` entity, Marginalia Editors included | 40 per Editor | the player is present at the kill tick |
| run end | `ev.run_ended` of a run that paid no Ur-Text victory Ink | 20 | the run had ≥ 3 qualified Folio exits for this player; Novella, Daily, Weekly Gauntlet and `run.mut.short_book` victories are run ends here |
| Ur-Text victory | `boss.ur_editor` dies with the player present (the final Folio of the Standard and Abridged structures, Seeded and Custom runs of those structures included), and the run keeps its clear (a Second Edition continuation must end `run.end.victory`, below) | `VICTORY_INK_PER_FOLIO × final_folio` = 12 × 25 = 300 for the Standard structure, 12 × 15 = 180 for Abridged | `final_folio` (doc 14 §1.2) is read at the kill tick, before doc 14 §9.1 switches a run to Endless or §8.9 to Second Edition; paid once per run; it replaces run-end Ink, and an Endless or Second Edition continuation after it pays no run-end Ink |
| achievement | first unlock of an `ach.` entry or a Codex mark (doc 14 §10) | 40 / 80 / 160 / 320 for doc 14 grade 1 / 2 / 3 / 4 | once per entry per account; doc 14 §10's current 59 entries pay 6,440 Ink in total, and the total is the grade-table sum over whatever entries doc 14 §10 lists |
| Codex free track | tiers `10j + 2` and `10j + 8`, `j ∈ 0..=9` | 80 each | 1,600 per Codex (§6.4) |
| welcome | the ledger transaction that sets `margin.milestone.first_run_end` (doc 14 §5.1), which also opens `margin.inkwell` | `WELCOME_INK = 1,200` | once per account; exactly 10 Beginner pulls (§3.2.5) |

**Mode rules.** Every run report carries `mode` and the easing flag (doc 14 §8.7).

| Row | Run kind (doc 14 §8.1 mode IDs) | Active-minute Ink | Folio, Editor, run-end and victory Ink | Codex XP (§6.2) | Account Reliquaries (§5.1) | Satchel (§5.5) |
|---|---|---|---|---|---|---|
| 1 | `mode.standard`, `mode.abridged`, `mode.novella`, `mode.second_edition`, `mode.endless`, `mode.custom` without an easing mutator | yes | yes | all sources | yes | yes |
| 2 | `mode.daily` (ranked or practice), `mode.weekly_gauntlet`, `mode.seeded` | yes | yes | all sources | yes | no |
| 3 | `mode.custom` with an easing mutator | yes | no | `pass.xp.active_minute` only | no | yes |
| 4 | Apocrypha (a simulation-affecting mod loaded) | no | no | none | no | no |
| 5 | Invasion, invader side (Canon §17.9) | no | no | none | no | no |

There is no Prologue (DECISIONS 132): the former mode.prologue row is retired, and the account's first run is an ordinary run under its own mode row. Finishing Folio 1 of the account's first run unlocks doc 14's first-run achievement (DECISIONS 137), which pays the achievement row above.

Canon §17.9 limits invader rewards to Quires and cosmetic Red Ink marks (§2.4); a party that opens its run to invasions (`mode.invasion_open`) is paid by its own mode row, and a Redaction (Canon §17.9) removes nothing account-level.

**Second Edition** (doc 14 §8.9, DECISIONS 108). A run that takes the "Second Edition" door reports once, at its end, as `mode.second_edition` under mode row 1. Its Second Edition Folios pay active-minute, Folio-exit and Editor-kill Ink and Codex XP like any other Folio, under the same per-run caps. Its Ur-Text victory stays paid only when the run ends `run.end.victory` (the Author's death): every other end forfeits the clear (DECISIONS 108), so each report carries `victory_present = 0` and pays the run-end Ink and XP instead. A victory whose `RunSummary.ironman` is true (no `ev.player_died` in any Second Edition Folio) also grants the Ironman cosmetic `cosm.frame.ironman` to each player with `victory_present = 1`, in the run-report transaction, once per account; a later Ironman clear grants nothing more. The board's Ironman mark is doc 14's (§9.4). The cosmetic is a portrait frame (DECISIONS 108 leaves the kind open; this chapter rules it).

The Editor-kill row applies to every Second Edition boss of `rank.editor`: `boss.editor_v1_rev` to `boss.editor_v4_rev`, `boss.ur_editor_rev` and `boss.author` (doc 13 §9.4). The Final Second Edition Folio pays two Editor kills, 80 Ink, 2 account Reliquaries and two `pass.xp.editor_kill` grants, because the revised Last Word and the Author die in sequence; both count in `RunSummary.editor_kills_present` (doc 14 §8.9). A fused Second Edition Folio's genre is read from `FolioRecord.genre`, the pair's genre with the lower Canon §5.2 row, and `FolioRecord.fused` holds the other genre (doc 14 §8.9); §5.1 records that genre.

Only Ur-Text victories pay the victory bonus, and it scales with the Folios played. A flat 300 for every victory would let repeated 5-Folio Daily practice victories (4 exits, 1 Editor, 25 active minutes) earn `3 × 25 + 4 × 10 + 40 + 300 = 455` Ink per 25 minutes (1,092 per hour, 2.73 times the reference); as a run end the same Daily pays `75 + 40 + 40 + 20 = 175` (420 per hour). A Standard victory pays 535 per hour (below). An Abridged victory (67 active minutes, 14 exits, 5 Editors) pays `201 + 140 + 200 + 180 = 721` Ink, `floor_div(721 × 60, 67) = 645` per hour; at a flat 300 it would pay 841 (753 per hour). The Abridged premium over Standard comes from its denser Editors (5 in 15 Folios) and is accepted because the 24-hour cap (§2.3) bounds both at 10 reference hours. Easing mutators keep only time-based income, so faster Folios cannot raise income per hour.

**Active tick.** Tick `t` is an active tick for slot `s` when the slot's status is `Active` or `Ghost`, and either the slot holds `st.downed` or is a Ghost, or the slot's last applied command with any non-neutral field (`move_x ≠ 0`, `move_z ≠ 0`, a yaw or pitch change, or `buttons ≠ 0`, Canon §2.4) has tick `t_cmd` with `t − t_cmd < ACTIVE_WINDOW_TICKS = 3,600`. Ticks only advance while the simulation runs, so a paused single-player run accrues nothing. The host counts `active_ticks: u64` and `last_input_tick: u64` per slot in `SlotState` (doc 14 §1.2, Interface assumptions), so a save, a Suspend and a replay all reproduce the count.

```
active_minutes = min(INK_ACTIVE_MINUTES_PER_RUN, floor_div(active_ticks, 3600))     // 240
end_ink = if victory_present { VICTORY_INK_PER_FOLIO × final_folio }                 // 12 × final_folio
          else if folio_exits_qualified >= 3 { 20 } else { 0 }
ink_run = 3 × active_minutes + 10 × folio_exits_qualified + 40 × editor_kills_present + end_ink
if easing { ink_run = 3 × active_minutes }
if apocrypha { ink_run = 0 }
ink_run = min(ink_run, INK_PER_RUN_CAP)                                            // 2,500
grant   = min(ink_run, max(0, INK_PLAY_ROLLING_24H − play_ink_granted_in_window))  // §2.3
```

**Worked example: the reference failed run.** A solo run with 216,000 active ticks (60 minutes) exits Folios 1 to 12 (each with dwell ≥ 90 s), kills the Editors of Folios 5 and 10, and dies in Folio 13. `active_minutes = 60`. `ink_run = 180 + 120 + 80 + 20 = 400`. This run defines the **reference hour**: `REF_INK_PER_HOUR = 400`.

**Worked example: a victory run.** 450,000 active ticks (125 minutes), Folios 1 to 24 exited, five Editors killed (Folios 5, 10, 15, 20, 25), victory. `ink_run = 375 + 240 + 200 + 300 = 1,115`, which is 535 Ink per hour (`floor_div(1115 × 60, 125) = 535`).

**Co-op.** Each player earns Ink separately and in full; nothing is split. (Codex kill XP divides by `enemy_count_bp` only to offset the larger enemy counts of Canon §17.2, §6.2.)

**Apocrypha runs.** A run is Apocrypha when a simulation-affecting mod (doc 02 §13.1) is loaded. Canon §2.4 hashes exactly those mods into the `MOD` section of `content_hash` (doc 02 §11.2); presentation-only mods (records only in `art.`, `aud.`, `vfx.`, `nar.` and `ui.`, plus Fluent files and assets) enter `presentation_hash` instead and never make a run Apocrypha (DECISIONS 87). The run report carries the number of simulation-affecting mods as `mod_count`, and the §10.4 content-hash check, which assumes an empty `MOD` section, rejects a simulation-modded run that reports `mod_count = 0`. An Apocrypha run earns 0 Ink, 0 Quires, 0 Codex XP and 0 Reliquaries, and the lobby shows "Apocrypha: account rewards off" before the run starts. Achievements are evaluated by doc 14 and are also off in Apocrypha runs.

### 2.3 Caps

| Cap | Value | Scope | Behaviour at the cap |
|---|---|---|---|
| `INK_ACTIVE_MINUTES_PER_RUN` | 240 | per run, per player | further active minutes earn 0 Ink; Folio and Editor Ink continue |
| `INK_PER_RUN_CAP` | 2,500 | per run, per player | grant clipped to 2,500 |
| `INK_PLAY_ROLLING_24H` | 4,000 | per account, sum of active-minute, Folio, Editor, run-end and victory Ink with grant timestamps in `(now − 86,400 s, now]` | the excess of a grant is dropped; the run summary shows "Ink limit reached: 4,000 in 24 hours" |
| `QUIRES_PER_RUN_CAP` | 400 | per run, per player | the run's Quire grant is clipped to 400 |
| `INVASION_QUIRES_ROLLING_24H` | 60 | per account, invader Quires (§2.4) with grant timestamps in `(now − 86,400 s, now]` | the excess is dropped |
| `CODEX_XP_PLAY_ROLLING_24H` | 200,000 | per account, the base grant `g` of every §6.2 source except `pass.xp.achievement` | the excess of `g` is dropped first; the catch-up bonus (§6.3) is then computed on the clipped `g` and never counts toward the cap |

Achievement, welcome and Codex-track grants never count toward any cap. Caps stop automated farming and discourage marathon sessions; no cap removes anything already earned. Both play rolling caps equal 10 reference hours (`4,000 / 400` and `200,000 / 20,000`); the invader Quire cap equals one reference hour of run Quires (`Q_REF = 60`, doc 14 §1.1).

### 2.4 Quire earn rates

This chapter owns the Quire earn rate (Canon §19); doc 14 §4.6 supplies one `run_score` per slot.

| Source | Quires | Notes |
|---|---|---|
| run | `q(slot) = min(QUIRES_PER_RUN_CAP, floor_div(run_score(slot), 50))`, then `grant = max(0, q − slot.quires_paid)` | `run_score(slot)` is doc 14 §4.6's (score × `mode_quire_bp` × presence × `RUN_SCORE_BP`, capped at `RUN_SCORE_FOLIO_CAP × cleared_present`), calibrated so the reference failed run gives 3,000 and a reference victory 15,000; `quires_paid` is doc 14 §8.8's departed payout; Apocrypha runs pay 0; easing runs pay 0 through `mode_quire_bp = 0` |
| achievement | 10 / 25 / 50 / 100 for doc 14 grade 1 / 2 / 3 / 4 | once per `ach.` entry or Codex mark; doc 14 §10's current 59 entries pay 1,960 Quires in total |
| Codex free track | 25 at tiers `10j + 4` | 250 per Codex |
| invasion, invader side (Canon §17.9) | `INVASION_QUIRES = 5` for an invasion that lasts at least `INVASION_MIN_TICKS = 3,600` ticks or ends with the party Redacted; `INVASION_REDACT_QUIRES = 5` more when the party is Redacted | `INVASION_QUIRES_ROLLING_24H = 60`; an invasion shorter than 3,600 ticks that does not end in a Redaction pays 0, whatever ended it; an invasion of an Apocrypha session pays 0 |

Worked examples (doc 14 §4.2 and §4.6): the reference failed run R has `run_score = 3,033` and grants `floor_div(3033, 50) = 60` Quires, which is doc 14's `Q_REF = 60` per hour. Solo victory B has `run_score = 14,991` and grants 299. Trio victory C at full presence has a raw score of 28,641, which doc 14's per-Folio cap limits to `run_score = 800 × 25 = 20,000`, and grants `floor_div(20000, 50) = 400`. An invader who Redacts a party after 200 s earns `5 + 5 = 10` Quires.

The invader's account reads the invasion's length and outcome from `ev.invasion_ended` (`duration`, `redacted`; doc 03 §14.2), which doc 16 §9.13 reports to it: the host and the invader each post the outcome, the shorter reported length counts, and the party counts as Redacted only when both reports say so.

**Red Ink marks.** The invader's only other reward (Canon §17.9) is a cosmetic mark per lifetime count of Redactions: `cosm.redink.mark_001` at 1, `cosm.redink.mark_005` at 5, `cosm.redink.mark_025` at 25 and `cosm.redink.mark_100` at 100. Marks are deterministic counters, never random, and grant nothing else.

### 2.5 Vellum sources

| Source | Vellum |
|---|---|
| Inkwell duplicate | §3.8 table, 1 to 40 per duplicate |
| Unused spark on a Featured banner (base or expansion) at banner end | 1 per spark point (§3.6) |

Nothing else creates Vellum.

### 2.6 The Quill

`ent.quill` is an entitlement count, not a currency. A Quill is created by (a) the purchase of any `store.sku.codex_<nnn>_premium`, which immediately redeems it on that Codex, or (b) reaching tier 100 of a Codex whose premium track the account owns. A Quill never expires. A held Quill can be redeemed on any Codex premium track the account does not own, including past Codexes and the next Codex when it is released. A Quill cannot be converted into anything else (SEP-2).

### 2.7 Reference income

| Income line | Per reference hour | Basis |
|---|---|---|
| Ink from play | 400 | §2.2 reference failed run |
| Ink from Codex free track | 26.67 | 1,600 Ink per Codex over the 60-hour no-catch-up completion (§6.7); excluded from every hours table below |
| Quires | 60 | §2.4 reference failed run (`run_score` 3,033); doc 14 `Q_REF` |
| Inkwell pulls from play | 2.5 | 400 / 160 |
| Spark (Featured banners only) | 2.5 | 1 per pull |
| Account Reliquaries | 2.57 | §5.1 |
| Codex XP | 20,000 | §6.2 |

### 2.8 Hours to earn

All hours use play Ink only, `REF_INK_PER_HOUR = 400`, `INK_PULL_COST = 160` (Beginner 120), and pull counts from the exact tables of §3.10. Hours = pulls × cost / 400, and the welcome grant is excluded. "Deterministic ceiling" is the worst case. Vellum hours assume the spark-only Vellum rate of 2.5 per hour: a player who spends all Ink on Featured banners and never claims spark receives at least that much, and duplicates add more. Recollection hours are doc 14's `est_ch / 100` and assume the trial is one of the 2 active trials.

**2.8.1 Every featured banner unit.**

| Unit | Expected pulls | Expected Ink | Expected hours | 90th percentile pulls / hours | Deterministic ceiling |
|---|---|---|---|---|---|
| Featured Illuminated Echo (Featured Echo banner, base or expansion) | 48.956 (with spark) | 7,833 | 19.58 | 94 / 37.6 | 100 pulls = 16,000 Ink = 40.0 h (spark) |
| Featured Illuminated Script (Featured Script banner, or an expansion banner in an even period) | 48.956 | 7,833 | 19.58 | 94 / 37.6 | 100 pulls = 40.0 h (spark) |
| One specific rate-up Rubricated Echo or Script | 34.683 | 5,549 | 13.87 | 75 / 30.0 | Archive 20 Vellum = 8.0 h of spark-only Vellum |
| Any of the three rate-up Rubricated units | 11.353 | 1,816 | 4.54 | 19 / 7.6 | 10 pulls guarantee Rubricated or better |
| Beginner banner chosen Echo | 21.136 at 120 Ink | 2,536 | 6.34 | 26 / 7.8 | 30 pulls = 3,600 Ink = 9.0 h |

**2.8.2 Every Echo.** §3.1.1 binds each of the 24 launch Echoes of doc 14 §6.3 to an Inkwell tier: doc 14's Tier 2 Echo of each genre (the longer trial) is Illuminated and its Tier 1 Echo is Rubricated, so rarity tracks Recollection cost. Genre codes follow doc 14 §6.3: MF, GH, CH, SO, CP, PA, WW, HN, WX, SP, FT, MA. "First feature" is the launch rotation of §3.2.2.

| Genre | Echo | Inkwell tier | First feature | Featured banner: E h / p90 h / ceiling h | Standard banner E h | Vellum Archive | Recollection h |
|---|---|---|---|---|---|---|---|
| MF | `echo.wren_long_index` | Illuminated | `banner.echo.p001` featured | 19.58 / 37.6 / 40.0 | 38.55 | 100 Vellum = 40.0 h, after p001 ends | 9.15 |
| GH | `echo.mother_ilse` | Illuminated | p002 featured | 19.58 / 37.6 / 40.0 | 38.55 | 40.0 h, after p002 | 9.20 |
| CH | `echo.drowned_clerk` | Illuminated | p003 featured | 19.58 / 37.6 / 40.0 | 38.55 | 40.0 h, after p003 | 9.05 |
| SO | `echo.kestrel_unit` | Illuminated | p004 featured | 19.58 / 37.6 / 40.0 | 38.55 | 40.0 h, after p004 | 9.10 |
| CP | `echo.ronin_valdez` | Illuminated | p005 featured | 19.58 / 37.6 / 40.0 | 38.55 | 40.0 h, after p005 | 9.20 |
| PA | `echo.tallow` | Illuminated | p006 featured | 19.58 / 37.6 / 40.0 | 38.55 | 40.0 h, after p006 | 9.10 |
| WW | `echo.preacher_amos` | Illuminated | p007 featured | 19.58 / 37.6 / 40.0 | 38.55 | 40.0 h, after p007 | 9.10 |
| HN | `echo.vera_lux` | Illuminated | p008 featured | 19.58 / 37.6 / 40.0 | 38.55 | 40.0 h, after p008 | 9.10 |
| WX | `echo.sun_wukong` | Illuminated | p009 featured | 19.58 / 37.6 / 40.0 | 38.55 | 40.0 h, after p009 | 9.10 |
| SP | `echo.colonel_wexley` | Illuminated | p010 featured | 19.58 / 37.6 / 40.0 | 38.55 | 40.0 h, after p010 | 9.10 |
| FT | `echo.wolf_in_wool` | Illuminated | p011 featured | 19.58 / 37.6 / 40.0 | 38.55 | 40.0 h, after p011 | 9.10 |
| MA | `echo.loki` | Illuminated | p012 featured | 19.58 / 37.6 / 40.0 | 38.55 | 40.0 h, after p012 | 9.25 |
| MF | `echo.fenrir` | Rubricated | p001 rate-up | 13.87 / 30.0 / Archive | 113.98 | 20 Vellum = 8.0 h, from launch | 4.70 |
| GH | `echo.lilith` | Rubricated | p002 rate-up | 13.87 / 30.0 / Archive | 113.98 | 8.0 h | 5.20 |
| CH | `echo.jormungandr` | Rubricated | p003 rate-up | 13.87 / 30.0 / Archive | 113.98 | 8.0 h | 4.90 |
| SO | `echo.ostra_vey` | Rubricated | p004 rate-up | 13.87 / 30.0 / Archive | 113.98 | 8.0 h | 5.20 |
| CP | `echo.nix_seventeen` | Rubricated | p001 rate-up | 13.87 / 30.0 / Archive | 113.98 | 8.0 h | 4.70 |
| PA | `echo.ma_rust` | Rubricated | p002 rate-up | 13.87 / 30.0 / Archive | 113.98 | 8.0 h | 4.70 |
| WW | `echo.josephine_crowe` | Rubricated | p003 rate-up | 13.87 / 30.0 / Archive | 113.98 | 8.0 h | 4.90 |
| HN | `echo.dex_halloran` | Rubricated | p004 rate-up | 13.87 / 30.0 / Archive | 113.98 | 8.0 h | 4.70 |
| WX | `echo.lan_ninth_peak` | Rubricated | p001 rate-up | 13.87 / 30.0 / Archive | 113.98 | 8.0 h | 5.14 |
| SP | `echo.ada_brassvane` | Rubricated | p002 rate-up | 13.87 / 30.0 / Archive | 113.98 | 8.0 h | 4.90 |
| FT | `echo.briar_thornwood` | Rubricated | p003 rate-up | 13.87 / 30.0 / Archive | 113.98 | 8.0 h | 4.90 |
| MA | `echo.kallias_twice_drowned` | Rubricated | p004 rate-up | 13.87 / 30.0 / Archive | 113.98 | 8.0 h | 4.90 |

Every Illuminated Echo is also the Beginner banner's target option (6.34 h expected, 9.0 h ceiling, once per account). The Standard banner column for Illuminated Echoes is `E[min(T, 100)] = 96.375` pulls, because the Standard spark at 100 pulls lets the player select any Standard Illuminated entry. For Rubricated Echoes it is the expected 284.946 pulls to one specific entry of the 36-entry Rubricated pool, which has no spark; the Archive is the deterministic route. **Every Script** has the hours of the Echo row of its tier: Illuminated Scripts 19.58 / 37.6 / 40.0 h on their Featured Script banner, 38.55 h on the Standard banner, 40.0 h in the Archive; Rubricated Scripts 13.87 / 30.0 h as a rate-up, 113.98 h on the Standard banner, 8.0 h in the Archive. Scripts have no Recollection route; each has the Transcription route of §3.9.1 instead: 16 qualified Folio exits (1.33 reference hours) for a Rubricated Script and 32 (2.67 hours) for an Illuminated one.

**Design contract.** The median hours to a chosen featured Illuminated unit stay at or below 25 reference hours (now `49 × 160 / 400 = 19.6`, the "about 20 hours per featured Echo" of DECISIONS 57), and its deterministic ceiling stays at or below 40 reference hours. Every Echo has a Recollection trial (DECISIONS 80), and every trial stays below the 19.58-hour Inkwell expectation (doc 14 §6.2), so the Inkwell is a lucky shortcut and the source of Folds, never the only path. Any change to Ink rates, pull cost or hazards must re-run §3.10 and keep all three.

**2.8.3 Unlock pace (DECISIONS 79).** About 80 hours of play unlock every non-cosmetic option without spending money. The three routes run in parallel from the same play:

| Non-cosmetic options | Route without money | Reference hours | Source |
|---|---|---|---|
| 73 non-cosmetic Bindery nodes | Quires from runs (§2.4) | 70.0 | doc 14 §5.4a |
| 24 launch Echoes | Recollection trials alone, 2 active at a time | 84.2 | doc 14 §5.4a, §6 |
| 36 launch Scripts | Transcription alone (§3.9.1): 768 qualified Folio exits | 64.0 at 12 exits per reference hour (§2.2); 76.8 at doc 14's 10 Folios per hour | this chapter |

Earned Ink and Vellum only shorten these totals. Any change to this chapter's Ink, Quire or Transcription rates keeps every row at or below doc 14 §5.4a's 85.0-hour pace contract, with the Script row measured at doc 14's 10 Folios per hour.

---

## 3. The Inkwell

The Inkwell is the meta gacha in the Margin. A **pull** spends 160 Ink and returns exactly one **entry**; its tiers are named for manuscript craft.

### 3.1 Catalog, tiers and entry kinds

| Tier ID | Name | Frame colour | Contains |
|---|---|---|---|
| `banner.tier.illuminated` | Illuminated | gold leaf `#D4A93C` | Illuminated (slot A) Echoes, Illuminated Scripts |
| `banner.tier.rubricated` | Rubricated | vermilion `#C8341E` | Rubricated (slot B) Echoes, Rubricated Scripts |
| `banner.tier.plain` | Plain | iron-gall black `#2B2622` | Plain cosmetics |

| Entry kind | ID pattern | Effect on runs | Launch count |
|---|---|---|---|
| Echo | `echo.<name>` (doc 14 §6.3) | a pre-built Erratum; horizontal | 12 Illuminated, 12 Rubricated (§3.1.1) |
| Script | `bundle.script.<name>` | a grant bundle that adds options to the account content set: starting kits, spell glyph families, Folio room variants, hireling types (§3.1.2) | 12 Illuminated, 24 Rubricated |
| Plain cosmetic | `cosm.ink.inkwell_<nn>` (01 to 20), `cosm.sigil.inkwell_<nn>` (01 to 15), `cosm.title.inkwell_<nn>` (01 to 15), `cosm.emote.inkwell_<nn>` (01 to 10) | none | 20 inks, 15 sigils, 15 titles, 10 emotes = 60; display names from doc 18 |

**Script neutrality rule.** A Script that adds item templates to a loot table adds them inside their rarity band only. Doc 11 renormalizes template weights within the band, so the share of each rarity in every table is unchanged. Unlocking Scripts never raises the expected rarity or power of any drop. Scripts never grant stats, starting currency, extra Emendations or extra Keepsake slots. A Script kit has a total base `value` within 360..=440 Lucre, the doc 14 §2.4 kit band. Scripts are excluded from `unlock.daily_baseline` (Canon §13.6).

#### 3.1.1 Echo binding

Echo acquisition belongs to this chapter (Canon §19). Each core genre's doc 14 Tier 2 Echo (the 9- to 16-hour trial) is Illuminated and its Tier 1 Echo (4 to 9 hours) is Rubricated. Genre index `g` is the Canon §5.2 row number.

| g | Genre | Illuminated Echo (slot A) | Rubricated Echo (slot B) |
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

Each Folio Expansion adds one Illuminated and one Rubricated Echo for its genre on the same rule. Every Echo, expansion Echoes included, also unlocks through its bounded Recollection trial (doc 14 §6, DECISIONS 80), so the Inkwell and the Archive are never the only paths to an Echo.

#### 3.1.2 Script catalog

Indices are 1-based positions in ascending ID byte order within each tier; §3.2.2 uses them. Each Script grants `unlock.` IDs in the patterns doc 14 reserves for Scripts (`unlock.kit.<calling>_<name>`, `unlock.glyph.<family>`, `unlock.room.<name>`, `unlock.hireling.<type>`); owning the unlock adds the named content to the account content set. The content itself is requested in Interface assumptions.

| Tier | Index | ID | Grants (owner) | Neutrality class |
|---|---|---|---|---|
| Illuminated | 1 to 12 | `bundle.script.lost_chapter_<genre_suffix>` for, in index order, `cosmic_horror`, `cyberpunk`, `fairy_tale`, `gothic_horror`, `hardboiled_noir`, `mythic_antiquity`, `mythic_fantasy`, `post_apocalypse`, `space_opera`, `steampunk`, `weird_west`, `wuxia` | `unlock.room.lost_chapter_<genre_suffix>` (3 room variants in that genre's room pool, doc 12) and `unlock.hireling.chapter_<genre_suffix>` (1 hireling type, doc 13) | `room_variant` |
| Rubricated | 1 to 6 | `bundle.script.glyphs_<family>` for `ember`, `hollow`, `mirror`, `storm`, `thorn`, `tide` | `unlock.glyph.<family>`: 4 spell glyphs of that family added to the glyph loot pool (doc 06 §8.1); glyphs have tiers and loot pools, not rarity bands | `pool_addition` |
| Rubricated | 7 to 11 | `bundle.script.hireling_<type>` for `apothecary`, `cartographer`, `lamplighter`, `porter`, `sellsword` | `unlock.hireling.<type>`: 1 hireling type in the Hireling Hall pool (doc 13) | `hireling_option` |
| Rubricated | 12 to 24 | `bundle.script.kit_<calling_suffix>` for `alchemist`, `artificer`, `beastbinder`, `duelist`, `grenadier`, `gunslinger`, `monk`, `netrunner`, `oracle`, `shade`, `spellwright`, `vanguard`, `warden` | `unlock.kit.<calling_suffix>_script`: starting kit `run.kit.<calling_suffix>_script` selectable in the Dramatis (doc 14 §2.4) | `kit_option` |

Each Folio Expansion adds `bundle.script.lost_chapter_<exp_genre_suffix>` (Illuminated) and `bundle.script.glyphs_<exp_genre_suffix>` and `bundle.script.hireling_<exp_genre_suffix>` (Rubricated).

Pools (the launch catalog):

| Pool ID | Members | Size |
|---|---|---|
| `banner.pool.illuminated_echo` | 12 Illuminated Echoes | 12 |
| `banner.pool.illuminated_script` | 12 Illuminated Scripts | 12 |
| `banner.pool.rubricated` | 12 Rubricated Echoes + 24 Rubricated Scripts | 36 |
| `banner.pool.plain` | 60 Plain cosmetics | 60 |

### 3.2 Banners

#### 3.2.1 Banner types

| Type | ID | Duration | Featured top | Rate-up mid | Top pool on a non-featured result | Mid pool on a non-rate-up result | Pity family | Spark |
|---|---|---|---|---|---|---|---|---|
| Featured Echo | `banner.echo.p<nnn>` | one 28-day period | 1 Illuminated Echo, 7500 Bp | 3 Rubricated Echoes, 5000 Bp | the other 11 Illuminated Echoes | the other 33 Rubricated entries | `banner.family.echo` | 100 → featured Echo; leftover → Vellum at end |
| Featured Script | `banner.script.p<nnn>` | one 28-day period | 1 Illuminated Script, 7500 Bp | 3 Rubricated Scripts, 5000 Bp | the other 11 Illuminated Scripts | the other 33 Rubricated entries | `banner.family.script` | 100 → featured Script; leftover → Vellum at end |
| Standard | `banner.standard` | permanent | none | none | all 24 Illuminated entries | all 36 Rubricated entries | `banner.family.standard` | 100 → select any Standard Illuminated entry; never expires |
| Beginner | `banner.beginner` | per account until closed | the Echo the player selected, 10000 Bp | none | none | all 36 Rubricated entries | `banner.family.beginner` | none |
| Expansion | `banner.exp<nn>.p<nnn>` | one 28-day period, for owners of `store.sku.folio_exp_<nn>` | the expansion's Illuminated Echo (odd periods) or Illuminated Script (even periods), 7500 Bp | the expansion's Rubricated Echo and 2 base Rubricated Echoes (§3.2.6), 5000 Bp | 12 base Illuminated entries of the featured unit's kind plus the expansion's other Illuminated entry (13) | the base Rubricated pool plus the expansion's 2 Rubricated Scripts, minus the rate-ups (36) | `banner.family.exp<nn>` | 100 → featured unit; leftover → Vellum at end |

All five types share the Plain pool and the Rubricated hazard. Featured, Expansion, Standard and Script banners share the Illuminated hazard `banner.hazard.illuminated_std`. The Beginner banner uses `banner.hazard.illuminated_beginner`. At most one banner of each family is `Live` in any period (a period can have no Featured Script banner, §3.2.7), so a family's counters change only through that banner (§4.1 relies on this).

#### 3.2.2 Featured Echo launch rotation

Genres are indexed 1 to 12 in Canon §5.2 order (§3.1.1). Period `k` (1-based) features the Illuminated Echo of genre `g = ((k − 1) mod 12) + 1` and rates up the Rubricated Echoes of genres `g`, `((g + 3) mod 12) + 1` and `((g + 7) mod 12) + 1`. Each Rubricated Echo is rated up in exactly 3 of every 12 periods. The rotation repeats with period 12 forever; expansion content never shifts it (§3.2.6). Codes are §3.1.1's: `-A` is the Illuminated Echo and `-B` the Rubricated Echo of that genre.

| Period | Featured | Rate-up |
|---|---|---|
| p001 | MF-A `echo.wren_long_index` | MF-B, CP-B, WX-B |
| p002 | GH-A `echo.mother_ilse` | GH-B, PA-B, SP-B |
| p003 | CH-A `echo.drowned_clerk` | CH-B, WW-B, FT-B |
| p004 | SO-A `echo.kestrel_unit` | SO-B, HN-B, MA-B |
| p005 | CP-A `echo.ronin_valdez` | CP-B, WX-B, MF-B |
| p006 | PA-A `echo.tallow` | PA-B, SP-B, GH-B |
| p007 | WW-A `echo.preacher_amos` | WW-B, FT-B, CH-B |
| p008 | HN-A `echo.vera_lux` | HN-B, MA-B, SO-B |
| p009 | WX-A `echo.sun_wukong` | WX-B, MF-B, CP-B |
| p010 | SP-A `echo.colonel_wexley` | SP-B, GH-B, PA-B |
| p011 | FT-A `echo.wolf_in_wool` | FT-B, CH-B, WW-B |
| p012 | MA-A `echo.loki` | MA-B, SO-B, HN-B |

Featured Script banners use the same `g`: period `k` features the Illuminated Script with index `g` of §3.1.2 and rates up the Rubricated Scripts with indices `2g − 1`, `2g` and `((2g + 11) mod 24) + 1`. The third index is `2g + 12` for `g ≤ 6` and `2g − 12` for `g ≥ 7`, so the three indices of one period are always distinct. Over 12 periods every Rubricated Script is rated up at least once, and the 12 even-indexed ones twice. Example: a period with `g = 1` (p001, p013, …) features `bundle.script.lost_chapter_cosmic_horror` and rates up `bundle.script.glyphs_ember` (1), `bundle.script.glyphs_hollow` (2) and `bundle.script.kit_beastbinder` (14). Early Access ships all 12 core genres (DECISIONS 114), so p001 runs this banner.

The base catalog is the launch catalog and never grows: Canon §16.2 fixes 24 base Echoes. New Echoes and Scripts ship only inside Folio Expansions (§3.2.6), so the base rotation never changes.

#### 3.2.3 Schedule

- `T0` is the Early Access launch day at 00:00:00 UTC, a configuration value of the economy service set at launch; the release plan has no hard dates (DECISIONS 111). Every example in this chapter uses the illustrative value `T0 = 1820448000` (2027-09-09 00:00:00 UTC), which is not a release date.
- Period `k` covers `[T0 + 28 × (k − 1) days, T0 + 28 × k days)`. Featured Echo and Featured Script banners both use period boundaries. Banner IDs carry `k` as three digits.
- The banner UI shows the end as a calendar date, for example "Ends 7 Oct 2027, 00:00 UTC" for p001, then "Afterwards: in the Standard banner and the Vellum Archive; unused spark becomes Vellum". It never shows a countdown, a "last chance" label or a pulsing timer.

#### 3.2.4 Banner lifecycle

| State | Entered when | Exits to | Effects on entry |
|---|---|---|---|
| `Draft` | record authored | `Scheduled` | content validator runs (§3.13) |
| `Scheduled` | validator passes and `start_utc` is set | `Live` at `start_utc` | `config_hash = SHA-256(canonical JSON of the banner record)` is frozen and published at `/odds/<banner_id>` 7 days before `start_utc` |
| `Live` | `start_utc` | `Ended` at `end_utc` | pulls accepted; seed epochs open lazily per account (§4.1) |
| `Ended` | `end_utc` | `Archived` in the same transaction | pulls rejected with `banner_ended`; every open seed epoch of the banner is revealed; every account's leftover spark converts to Vellum 1:1 (§3.6) |
| `Archived` | after `Ended` | terminal | the featured Illuminated unit joins the Vellum Archive's Illuminated shelf (with `requires_sku` for an expansion unit); launch units are already in `banner.standard` and expansion units never join it; the banner page stays public forever with final aggregate statistics |

A pull request received at or after `end_utc` is rejected and no Ink is debited. A pull received earlier but still processing at `end_utc` completes.

#### 3.2.5 Beginner banner

- Opens on the account's first Inkwell visit. The player selects as the target one of the 12 launch Illuminated Echoes that the account does not own. The target can be changed between any two pulls while the banner is open; a change never touches the Beginner pity state.
- If the target becomes owned another way (Recollection, Archive, another banner), the selection clears and the next Beginner pull is refused with `beginner_target_required` until the player selects again. If the account owns all 12 launch Illuminated Echoes, the banner closes with no pull made and nothing is lost.
- Pull cost `INK_PULL_COST_BEGINNER = 120`. At most 30 pulls per account.
- The banner closes for the account immediately after its first Illuminated result, which is always the target. The hazard guarantees it by pull 30. Closed is permanent.
- Beginner pulls never advance any other family's counters and grant no spark.

#### 3.2.6 Expansion banners

Each Folio Expansion `nn` ships 1 Illuminated Echo, 1 Rubricated Echo, 1 Illuminated Script and 2 Rubricated Scripts (§3.1.1, §3.1.2). They appear only on that expansion's banners and on Archive shelves flagged `requires_sku = store.sku.folio_exp_<nn>`, visible only to owners of that SKU, and never join `banner.standard` or any base pool.

- From the first period that starts at or after the expansion's release, every period `k` runs `banner.exp<nn>.p<nnn>` alongside the base banners, for owners only (a banner always spans a whole period, §3.13). Its family `banner.family.exp<nn>` has its own pity, guarantees and seed epochs, so base Echo-family counters never mix with it.
- In odd periods its featured Illuminated unit is the expansion's Illuminated Echo and its off-featured top pool is the 12 base Illuminated Echoes plus the expansion's Illuminated Script. In even periods the featured unit is the expansion's Illuminated Script and the off-featured top pool is the 12 base Illuminated Scripts plus the expansion's Illuminated Echo. Either way the off-featured pool has 13 entries.
- Its rate-ups are the expansion's Rubricated Echo and the base Rubricated Echoes of genres `((g + 3) mod 12) + 1` and `((g + 7) mod 12) + 1` for the period's base `g` (§3.2.2). Its off-rate pool is the 36-entry base Rubricated pool plus the expansion's 2 Rubricated Scripts, minus the 2 base rate-ups: 36 entries.
- The expansion's Rubricated entries enter the Archive at release; its Illuminated entries enter it after the first expansion banner that features them reaches `Ended`.
- Every banner therefore offers identical odds to every account that can open it (SEP-6). Per-entry rows for an expansion banner: featured 0.7500% base and 2.0392% consolidated; each of 13 off-featured 0.0192% and 0.0392%; each of 3 rate-ups 1.0000% and 2.7945%; each of 36 off-rate 0.0833% and 0.1164%; each Plain 1.5500% and 1.4146%.

#### 3.2.7 Entries outside the build's content set

Every build from Early Access on ships all 12 core genres (DECISIONS 114, which supersedes the 6-genre Early Access of DECISIONS 41), so every launch Echo and Script is in every shipped content set, and the launch pools, rotation and odds of this chapter hold from p001. The rules below cover any Inkwell entry that a build still leaves out of its content set (for example a test build). Such an entry:

- is absent from every pool, every featured and rate-up list, every Archive shelf, the Transcription shelf (§3.9.1) and the Standard spark choice;
- is never featured: a period whose §3.2.2 featured entry is outside the content set at the period's `start_utc` runs no banner of that family, and the family's counters wait unchanged; a rate-up entry leaves its rate-up list, and the remaining rate-up entries share `mid_featured_bp` equally;
- joins every pool from the first period that starts at or after it enters the content set. Each affected banner record, `banner.standard` included, then gets a new `config_hash` for that period, published at `/odds/<banner_id>` 7 days before the period starts (§3.2.4, §3.11.3).

Every number and test vector in this chapter is the full catalog's; `pal odds` (§3.10) computes each build's disclosed values from that build's pools. Example (acceptance test 37): a test build whose content set leaves out only the 6 Lost Chapter Scripts of Cosmic Horror, Fairy Tale, Hardboiled Noir, Mythic Antiquity, Post-Apocalypse and Space Opera runs no `banner.script.p001` (the Cosmic Horror period); on its Featured Script banners each of the 5 off-featured Illuminated Scripts has 0.0500% base and 0.1020% consolidated (the featured unit keeps 0.7500% and 2.0392%); on its `banner.standard` each of the 18 Illuminated entries (12 Echoes, 6 Scripts) has 0.0556% base and 0.1416% consolidated.

### 3.3 The hazard function

A hazard record gives the per-pull probability, in Bp, of reaching the tier on the `n`-th pull since that tier (or better) was last reached. It is R3 D2's validated soft-pity model written in integer Bp.

```rust
pub struct Hazard { pub base_bp: i32, pub soft_start: u32, pub step_bp: i32, pub hard: u32 }

impl Hazard {
    /// n is 1-based: the first pull after a reset has n = 1.
    pub fn p_bp(&self, n: u32) -> i32 {
        assert!(n >= 1);
        if n >= self.hard { 10_000 }
        else if n >= self.soft_start {
            core::cmp::min(10_000, self.base_bp + self.step_bp * (n - self.soft_start + 1) as i32)
        } else { self.base_bp }
    }
}
```

| Hazard ID | `base_bp` | `soft_start` | `step_bp` | `hard` |
|---|---|---|---|---|
| `banner.hazard.illuminated_std` | 100 | 46 | 650 | 60 |
| `banner.hazard.rubricated_std` | 600 | 9 | 6000 | 10 |
| `banner.hazard.illuminated_beginner` | 100 | 21 | 1000 | 30 |

Values of `banner.hazard.illuminated_std`:

| n | 1–45 | 46 | 47 | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 58 | 59 | 60 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| `p_bp(n)` | 100 | 750 | 1400 | 2050 | 2700 | 3350 | 4000 | 4650 | 5300 | 5950 | 6600 | 7250 | 7900 | 8550 | 9200 | 10000 |

`banner.hazard.rubricated_std`: `p_bp(n) = 600` for `n ≤ 8`, `p_bp(9) = 6600`, `p_bp(10) = 10000`.
`banner.hazard.illuminated_beginner`: 100 for `n ≤ 20`, then 1100, 2100, … 9100 for `n = 21..=29`, and 10000 at `n = 30`.

**Validator rules for every hazard record.** `base_bp ≥ 1` (no zero-probability window, R3 C2 Webzen precedent). `p_bp(n)` is non-decreasing in `n`. `p_bp(hard) = 10000`. `soft_start ≤ hard`. Hazards are global data; no account-specific hazard exists.

### 3.4 Pull algorithm

Per-family pity state, stored per account:

```rust
pub struct PityState {
    pub top_since: u32,      // pulls since last Illuminated, 0..=59 (0..=29 for beginner)
    pub mid_since: u32,      // pulls since last Rubricated-or-better, 0..=9
    pub top_guarantee: bool, // next Illuminated is the featured unit
    pub mid_guarantee: bool, // next Rubricated is a rate-up unit
}
```

One pull, executed inside one database transaction on the economy service (§10):

```rust
fn pull(acct: &mut Account, b: &Banner, epoch: &mut SeedEpoch) -> PullResult {
    let cost = if b.kind == Beginner { 120 } else { 160 };
    ledger.debit(acct, Asset::Ink, cost, "burn/ink/inkwell")?;      // fails with insufficient_ink
    let nonce = epoch.next_nonce; epoch.next_nonce += 1;
    let mut fs = FairStream::new(pull_key(INKWELL, epoch, nonce));    // §4.2
    let u_tier = fs.uniform(10_000) as i32;                            // draw 1
    let u_feat = fs.uniform(10_000) as i32;                            // draw 2
    let pity_before = acct.pity[b.family];                             // copy for the audit record
    let ps = &mut acct.pity[b.family];
    let pt = b.hazard_top.p_bp(ps.top_since + 1);
    let pm = core::cmp::min(b.hazard_mid.p_bp(ps.mid_since + 1), 10_000 - pt);
    let (tier, pool): (Tier, &[EntryId]);
    if u_tier < pt {
        tier = Illuminated;
        let featured = match b.top_featured_bp { None => false,
            Some(f) => ps.top_guarantee || u_feat < f };
        if b.top_featured_bp.is_some() { ps.top_guarantee = !featured; }
        pool = if featured { &b.featured_top } else { &b.off_top };
        ps.top_since = 0; ps.mid_since = 0;
    } else if u_tier < pt + pm {
        tier = Rubricated;
        let rate_up = match b.mid_featured_bp { None => false,
            Some(f) => ps.mid_guarantee || u_feat < f };
        if b.mid_featured_bp.is_some() { ps.mid_guarantee = !rate_up; }
        pool = if rate_up { &b.rate_up_mid } else { &b.off_mid };
        ps.top_since += 1; ps.mid_since = 0;
    } else {
        tier = Plain; pool = &PLAIN_POOL;
        ps.top_since += 1; ps.mid_since += 1;
    }
    let idx = fs.uniform(pool.len() as u64) as usize;                  // draw 3
    let entry = pool[idx];
    if b.has_spark() { acct.spark[b.id] += 1; }
    let grant = resolve_duplicate(acct, entry, tier);                   // §3.8
    audit.append(PullRecord { acct, banner: b.id, epoch: epoch.id, nonce,
        pity_before, u_tier, u_feat, idx, tier, entry, grant });       // §4.4
    PullResult { tier, entry, grant }
}
```

- **Draw count.** Every pull draws exactly three accepted values from its own fair stream, in the order tier, featured, index, whatever the outcome; `uniform(1)` still consumes one word. Every `n` used by the Inkwell and Reliquaries is at most 10,000, so rejection sampling (§4.2) consumes an extra `u64` word with probability `(2^64 mod n) / 2^64 < 2^−50` per draw.
- **Preconditions,** checked before the debit, each failing with its own error and no state change: the banner is `Live` (`banner_ended`); the account can open it (`requires_sku`); the Ink balance covers the cost (`insufficient_ink`); for the Beginner banner, a target is selected (`beginner_target_required`) and the banner is open (`beginner_closed`).
- **Precedence.** The single tier roll partitions `0..10000` into `[0, pt)` Illuminated, `[pt, pt + pm)` Rubricated and the rest Plain. When `pt` plus the unclipped Rubricated hazard exceeds 10000, Illuminated takes precedence, which is why `pm` is clipped.
- **Pool order.** Every pool indexed by draw 3 (featured list, rate-up list, off pools after exclusion, Plain pool) is in ascending ID byte order; the validator rejects a record whose explicit lists are not sorted. The Plain pool's index 0 is `cosm.emote.inkwell_01`, index 10 is `cosm.ink.inkwell_01`, index 30 is `cosm.sigil.inkwell_01` and index 45 is `cosm.title.inkwell_01`.
- **Ten-pull.** A ten-pull is ten consecutive single pulls with consecutive nonces, debited as one 1,600 Ink posting; it requires a balance of at least 1,600 and is not offered on the Beginner banner. It has no discount and no extra guarantee. The Rubricated hazard already guarantees a Rubricated-or-better result in any 10 consecutive pulls of one family.
- **Beginner.** `featured_top` is the selected Echo and `top_featured_bp = Some(10000)`. After an Illuminated result, the banner closes.
- **Standard.** `top_featured_bp = None` and `mid_featured_bp = None`; `top_guarantee` and `mid_guarantee` of the Standard family never change.

### 3.5 Featured split and guarantees

- Featured banners: an Illuminated result is the featured unit with 7500 Bp, or with certainty when `top_guarantee` is set. A loss sets `top_guarantee`; a featured result clears it. This is R3 B3's 75/25 rule.
- A Rubricated result on a Featured banner is a rate-up unit with 5000 Bp, or with certainty when `mid_guarantee` is set; the three rate-up units are equally likely.
- Long-run shares follow from a two-state chain on the guarantee flag. For the 75/25 rule the stationary probability of `top_guarantee = false` is `4/5`, so the featured share of all Illuminated results is `4/5 × 3/4 + 1/5 × 1 = 4/5`. For the 50/50 rule the rate-up share of all Rubricated results is `2/3 × 1/2 + 1/3 × 1 = 2/3`.

### 3.6 Spark

- Each Featured banner instance keeps a per-account spark counter `spark[banner_id]`, +1 per pull on that banner.
- **Claim.** At `spark ≥ 100` the banner shows "Claim featured: 100 spark". Claiming subtracts 100 and grants one copy of the featured Illuminated unit through §3.8. Claiming never changes pity counters or guarantee flags.
- **End conversion.** When a Featured banner enters `Ended`, each account's remaining `spark[banner_id]` becomes the same amount of Vellum in one ledger transaction (`mint/vellum/spark`), and the counter is deleted.
- **Standard spark** counts Standard pulls. At 100, the player claims any one entry of the Standard Illuminated pool (24 entries). Standard spark never expires and never converts.
- Beginner pulls give no spark. Worst cases: a chosen featured Illuminated unit costs at most 100 pulls on its banner; a chosen Standard Illuminated entry costs at most 100 Standard pulls.

### 3.7 Pity carry-over rules

| State | Scope | Carries across banners | Carries across periods | Resets when |
|---|---|---|---|---|
| `top_since` | per account, per family | yes, within the family | yes, forever | an Illuminated result in that family |
| `mid_since` | per account, per family | yes | yes | a Rubricated or Illuminated result in that family |
| `top_guarantee` | per account, per family | yes | yes | a featured Illuminated result |
| `mid_guarantee` | per account, per family | yes | yes | a rate-up Rubricated result |
| `spark` | per account, per Featured banner instance | no | no; converts to Vellum at `Ended` | a claim subtracts 100 |
| Standard spark | per account | not applicable | yes, forever | a claim subtracts 100 |
| Beginner pity | per account | no | not applicable | the banner closes |

Families never share counters. Pulling on a Featured Script banner leaves the Echo family untouched. Counters never decay and never reset on inactivity, banner rotation, client seed change, game update or end of service.

### 3.8 Duplicates, Echo Folds and Vellum

An Echo has a **Fold** level `0..=6`. Obtaining an Echo for the first time sets Fold 0. Each further copy raises the Fold by 1 up to 6. Folds are sidegrades: each Fold adds an option or a cosmetic and never a number.

| Fold | Grant (doc 14 supplies the per-Echo content) | Kind |
|---|---|---|
| 1 | Alternate palette for the Echo's default outfit, `cosm.echo.<suffix>.palette` | cosmetic |
| 2 | Signature trait variant B (`trait.echo.<suffix>_b`), selectable at run start instead of variant A | sidegrade |
| 3 | Starting kit B, selectable at run start instead of kit A; doc 14 §6.1 makes kit A the Calling's standard kit and kit B its alt kit | sidegrade |
| 4 | Alternate outfit, `cosm.echo.<suffix>.outfit` | cosmetic |
| 5 | Signature trait variant C (`trait.echo.<suffix>_c`), selectable at run start | sidegrade |
| 6 | Illuminated portrait frame `cosm.echo.<suffix>.frame`, the Echo's title `cosm.echo.<suffix>.title` and an animated ink trail `cosm.echo.<suffix>.trail` | cosmetic |

Sidegrade rules (binding on doc 04 and doc 14): trait variants A, B and C pass doc 04's power-budget validator with equal budget; kits A and B have equal total `value` within 500 Bp (Canon §13.9). The player picks one variant and one kit per run. A Fold never raises a stat, cap or drop rate.

| Result | Not owned | Echo owned with Fold < 6 | Owned (Echo at Fold 6) |
|---|---|---|---|
| Illuminated Echo | owned at Fold 0 | Fold + 1 and 10 Vellum | 40 Vellum |
| Rubricated Echo | owned at Fold 0 | Fold + 1 and 2 Vellum | 8 Vellum |
| Illuminated Script | owned | not applicable | 40 Vellum |
| Rubricated Script | owned | not applicable | 8 Vellum |
| Plain cosmetic | owned | not applicable | 1 Vellum |

```rust
fn resolve_duplicate(acct: &mut Account, e: EntryId, tier: Tier) -> Grant {
    match (e.kind(), acct.owns(e)) {
        (_, false) => { acct.grant_owned(e); Grant::New(e) }
        (Echo, true) if acct.fold(e) < 6 => {
            acct.set_fold(e, acct.fold(e) + 1);
            let v = if tier == Illuminated { 10 } else { 2 };
            ledger.mint(acct, Asset::Vellum, v, "mint/vellum/dupe");
            Grant::Fold(e, acct.fold(e), v)
        }
        (_, true) => {
            let v = match tier { Illuminated => 40, Rubricated => 8, Plain => 1 };
            ledger.mint(acct, Asset::Vellum, v, "mint/vellum/dupe");
            Grant::Vellum(v)
        }
    }
}
```

An Echo's tier is fixed at release and never changes, so its duplicate value never changes.

### 3.9 The Vellum Archive

`banner.archive` is a permanent shop in the Margin. Every purchase is deterministic: the player picks an exact entry and pays exact Vellum.

| Shelf | Price (Vellum) | Stock |
|---|---|---|
| Illuminated Echo | 100 | every Illuminated Echo (launch or expansion) whose first Featured or Expansion banner featuring it has reached `Ended` |
| Illuminated Script | 100 | every Illuminated Script whose first Featured Script or Expansion banner featuring it has reached `Ended` |
| Rubricated Echo | 20 | every Rubricated Echo from launch |
| Rubricated Script | 20 | every Rubricated Script from launch |
| Plain cosmetic | 5 | every Plain cosmetic from launch |
| Fold step | 40 for an Illuminated Echo, 8 for a Rubricated Echo | the next Fold of an owned Echo below Fold 6 |

Buying an entry the account already owns is never offered; for an owned Echo the Fold step shelf is the route. An Archive purchase of an unowned Echo grants it at Fold 0. Expansion entries carry `requires_sku`. Nothing ever leaves the Archive.

#### 3.9.1 Transcription shelf

Every Script also unlocks without the Inkwell, so no non-cosmetic option is reachable only by luck (DECISIONS 79; DECISIONS 80 does the same for Echoes through Recollection).

- **Counter.** Each account keeps `transcription: u64`, starting at 0. Every qualified Folio exit (§2.2) of a run whose §2.2 mode row grants account Reliquaries adds 1, in the run-report transaction that advances the §5.1 lifetime Folio-exit counter. Nothing else changes it: no SKU, Ink, Vellum, Codex tier, achievement or invasion adds to it (SEP-4).
- **Claim.** On the Archive's Transcription shelf the player picks one Script the account does not own and can open (in the build's content set, §3.2.7, and for an expansion Script with its `requires_sku` owned). A Rubricated Script costs `TRANSCRIBE_RUBRICATED = 16` and an Illuminated Script `TRANSCRIBE_ILLUMINATED = 32`. The claim subtracts the cost and grants the Script as a new entry (§3.8 `Grant::New`). With `transcription` below the cost the claim is refused with `insufficient_transcription` and nothing changes.
- **Rules.** The counter never expires, decays, converts into anything else or transfers between accounts, and has no cap. It is a counter like spark, not a currency (Canon §14). Claims never touch pity, spark, Folds or Vellum. Every Script is on the shelf from launch; the shelf does not wait for a banner to end.
- **Pace.** The 36 launch Scripts cost `24 × 16 + 12 × 32 = 768` qualified exits: 64.0 reference hours at the §2.2 reference hour's 12 qualified exits, 76.8 hours at doc 14 §5.4a's 10 Folios per hour (§2.8.3). Each Folio Expansion adds `2 × 16 + 32 = 64` exits for its owners. A Script that arrives first from the Inkwell or the Archive is never claimed, so those routes only shorten the total.

Worked example: an account with `transcription = 40` claims `bundle.script.lost_chapter_wuxia` (Illuminated, 32), leaving 8; a claim of `bundle.script.kit_monk` (Rubricated, 16) is then refused with `insufficient_transcription` until 8 more qualified exits are credited. A later Inkwell copy of `bundle.script.lost_chapter_wuxia` is a duplicate worth 40 Vellum (§3.8).

### 3.10 Expected cost and percentile tables

Values are exact rational evaluations of §3.3 and §3.4 (recomputed with Python `fractions` for this review; the Markov-chain rows by exact linear solves), rounded half-even. `T` counts pulls from a fresh family state (counters 0, flags clear). The content tool (`pal odds`, doc 02 `crate.tools`) recomputes every disclosed value with exact `i128` rational arithmetic at content build, stores each as a `pct4` integer in the banner's odds record, and fails the build if any differs from the record.

**3.10.1 Tier rates (Featured, Script, Expansion and Standard banners).**

| Quantity | Value |
|---|---|
| Base Illuminated rate (pulls 1–45) | 1.0000% |
| Expected pulls per Illuminated, `E[T_top]` | 39.2303 |
| Consolidated Illuminated rate `1 / E[T_top]` | 2.5491% |
| P(Illuminated lands before soft pity, `n ≤ 45`) | 36.3815% |
| Base Rubricated rate | 6.0000% |
| Consolidated Rubricated rate (long run, Illuminated precedence included) | 12.5752% |
| Consolidated Rubricated-or-better rate | 15.1242% |
| Consolidated Plain rate | 84.8758% |

**3.10.2 Cumulative probability of the first Illuminated by pull n.**

| n | 10 | 20 | 30 | 40 | 45 | 46 | 48 | 50 | 52 | 55 | 60 |
|---|---|---|---|---|---|---|---|---|---|---|---|
| P(T_top ≤ n) | 9.5618% | 18.2093% | 26.0300% | 33.1028% | 36.3815% | 41.1528% | 59.7662% | 80.4685% | 93.7304% | 99.5942% | 100.0000% |

**3.10.3 Cumulative probability of the featured Illuminated unit by pull n (no spark).**

| n | 10 | 20 | 30 | 40 | 49 | 50 | 60 | 70 | 80 | 90 | 94 | 99 | 100 | 110 | 120 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| P(T_feat ≤ n) | 7.2780% | 14.0785% | 20.4262% | 26.3455% | 55.3181% | 62.9300% | 80.9095% | 83.9845% | 86.6460% | 88.9449% | 90.3197% | 95.8405% | 96.9721% | 99.9965% | 100.0000% |

With spark, `P(T_feat ≤ 100) = 100.0000%`. P(spark is needed, `T_feat > 100`) = 3.0279%.

**3.10.4 Percentiles (pulls).**

| Target | 10th | 25th | 50th | 75th | 90th | 95th | 99th | Max | Mean | Mean Ink |
|---|---|---|---|---|---|---|---|---|---|---|
| Any Illuminated | 11 | 29 | 48 | 50 | 52 | 53 | 55 | 60 | 39.230 | 6,277 |
| Featured Illuminated, no spark | 14 | 38 | 49 | 53 | 94 | 99 | 103 | 120 | 49.038 | 7,846 |
| Featured Illuminated, spark at 100 | 14 | 38 | 49 | 53 | 94 | 99 | 100 | 100 | 48.956 | 7,833 |
| One specific rate-up Rubricated | — | — | 25 | — | 75 | — | 146 | Archive 20 Vellum | 34.683 | 5,549 |
| Any rate-up Rubricated | — | — | 10 | — | 19 | — | 25 | — | 11.353 | 1,816 |
| Beginner chosen Echo (120 Ink) | — | — | 23 | — | 26 | — | — | 30 | 21.136 | 2,536 |

With `top_guarantee` set, the featured unit follows the "Any Illuminated" row.

**3.10.5 Multiple copies of one featured Illuminated unit on one banner.** A featured result always leaves `top_since = 0` and `top_guarantee = false`, so featured results are renewal points and `E[pulls for k copies, no spark] = 49.038 × k`.

| Copies | Fold reached | E pulls, no spark | E pulls, spark claimed at every 100 |
|---|---|---|---|
| 1 | 0 | 49.038 | 48.956 |
| 2 | 1 | 98.076 | 85.906 |
| 3 | 2 | 147.114 | 109.832 |
| 7 | 6 | 343.265 | 245.348 |

**3.10.6 Standard banner.** A specific Standard Illuminated entry (1 of 24) costs `24 × 39.2303 = 941.527` expected pulls without spark (Wald's identity over Illuminated renewals) and `E[min(T, 100)] = 96.375` with the spark selector. A specific Rubricated entry (1 of 36) costs 284.946 expected pulls from a fresh family state (exact linear solve over the 600 `(top_since, mid_since)` states; not `36 / 0.125752 = 286.28`, because a fresh state is not a renewal point for that entry).

### 3.11 Odds disclosure

Disclosure follows R3 C4 ODDS-1 to ODDS-4 although every Inkwell pull is earned.

**3.11.1 Display rule.** Every probability is shown as a percentage with exactly four decimal places:

```
pct4(num, den) = round_half_even(num × 1_000_000, den)        // integer in units of 0.0001 %
display        = format!("{}.{:04}%", pct4 / 10_000, pct4 % 10_000)
```

Worked example: one off-featured Illuminated Echo at base rate has probability `100/10000 × 2500/10000 × 1/11 = 1/4400`. `pct4(1, 4400) = round_half_even(1_000_000, 4400) = 227` (227.27…), displayed `0.0227%`.

**3.11.2 The odds panel.** It sits on the pull screen (R3 C4 ODDS-2), one input from the pull buttons, and contains:

1. The banner's tier table with base and consolidated rates.
2. Every entry with its base and consolidated per-entry probability.
3. The hazard as a formula and as the full `p_bp(n)` table of §3.3.
4. The featured rule, the rate-up rule, the spark rule, the duplicate table and the carry-over table.
5. **Live counters**: `top_since`, `mid_since`, both guarantee flags, spark, and "Your next pull" with its three exact tier probabilities.
6. Ink balance, pull cost, and the number of pulls the balance covers.

Per-entry rows for a Featured Echo banner (identical values for every Featured Script banner):

| Row | Base (pull 1, fresh counters) | Consolidated (long run) |
|---|---|---|
| Featured Illuminated unit | 0.7500% | 2.0392% |
| Each of the 11 off-featured Illuminated units | 0.0227% | 0.0463% |
| Each of the 3 rate-up Rubricated units | 1.0000% | 2.7945% |
| Each of the 33 off-rate Rubricated entries | 0.0909% | 0.1270% |
| Each of the 60 Plain cosmetics | 1.5500% | 1.4146% |

Per-entry rows for `banner.standard`: each of 24 Illuminated entries 0.0417% base, 0.1062% consolidated; each of 36 Rubricated entries 0.1667% base, 0.3493% consolidated; each Plain cosmetic 1.5500% base, 1.4146% consolidated.

Rounded rows do not sum exactly to 100.0000%; the panel states "Rounded half-even to 4 decimals; unrounded values sum to exactly 100%".

**3.11.3 Public pages.** `/odds/<banner_id>` serves the same content without login, plus the `config_hash`, the record JSON, and daily-refreshed aggregate results: total pulls, results per tier and per entry, and the observed Illuminated rate, anonymized (R3 C4 LOG-2). The page exists from 7 days before `start_utc` and is never removed.

### 3.12 Pull history

- Every account views its complete pull history for the retention period (§4.4), newest first, 50 rows per page, filterable by banner, tier and date.
- Row fields: UTC timestamp, banner, period, epoch ID, nonce, tier, entry, featured or rate-up flag, pity counters before the pull, grant (`New`, `Fold n + v Vellum`, `v Vellum`), and a "Verify" link that opens §4.3 with the row prefilled.
- Export as JSON (the `PullRecord` fields of §3.4) or CSV, generated by the service within 60 s.

### 3.13 Schemas and records

Banner record (content, validated by JSON Schema; integers only):

```json
{
  "id": "banner.echo.p001",
  "kind": "featured_echo",
  "family": "banner.family.echo",
  "period": 1,
  "start_utc": 1820448000,
  "end_utc": 1822867200,
  "cost_ink": 160,
  "hazard_top": "banner.hazard.illuminated_std",
  "hazard_mid": "banner.hazard.rubricated_std",
  "featured_top": ["echo.wren_long_index"],
  "top_featured_bp": 7500,
  "off_top_pool": "banner.pool.illuminated_echo",
  "rate_up_mid": ["echo.fenrir", "echo.lan_ninth_peak", "echo.nix_seventeen"],
  "mid_featured_bp": 5000,
  "off_mid_pool": "banner.pool.rubricated",
  "plain_pool": "banner.pool.plain",
  "spark_claim": 100,
  "spark_end_vellum_per_point": 1,
  "requires_sku": null
}
```

Echo IDs are doc 14 §6.3's, bound to tiers by §3.1.1. `start_utc` and `end_utc` are Unix seconds (here `T0` and `T0 + 2,419,200`). A pool reference excludes the banner's own featured and rate-up entries automatically. `requires_sku` is `null` for base banners and the SKU ID for expansion banners.

Hazard record:

```json
{ "id": "banner.hazard.illuminated_std", "base_bp": 100, "soft_start": 46, "step_bp": 650, "hard": 60 }
```

Script record:

```json
{
  "id": "bundle.script.kit_gunslinger",
  "kind": "script",
  "tier": "banner.tier.rubricated",
  "index": 17,
  "grants": ["unlock.kit.gunslinger_script"],
  "neutrality": "kit_option",
  "daily_baseline": 0,
  "requires_sku": null
}
```

Plain cosmetic record:

```json
{ "id": "cosm.ink.inkwell_01", "kind": "cosmetic", "cat": "cosm.cat.ink", "source": "banner.pool.plain", "tier": "banner.tier.plain", "requires_sku": null }
```

Validator rules: every `featured_top` and `rate_up_mid` entry exists and has the matching tier; `top_featured_bp` and `mid_featured_bp` are in `1..=10000`; pools are non-empty after exclusion; `end_utc − start_utc = 2,419,200` and `start_utc = T0 + 2,419,200 × (period − 1)` for Featured, Script and Expansion banners; at most one banner per family has any given `period`; a banner with `requires_sku ≠ null` references only base entries and that SKU's entries; Script `grants` never reference `cur.`, `ent.`, stats, Emendations or Keepsake capacity; Script `index` values are 1 to 12 (Illuminated) and 1 to 24 (Rubricated) in ascending ID order; every featured, rate-up and pooled entry is in the build's content set (§3.2.7).

### 3.14 Worked examples of pity

**Example A: soft pity and a lost 75/25.** An account's Echo family is at `top_since = 47`, `mid_since = 3`, both flags clear. The next pull has `n_top = 48` and `n_mid = 4`: `pt = 100 + 650 × 3 = 2050` (20.5000%), `pm = min(600, 10000 − 2050) = 600` (6.0000%), Plain 73.5000%. The fair stream yields `u_tier = 1873 < 2050`: Illuminated. `u_feat = 8122 ≥ 7500`: not featured. The off-featured index draw picks one of the 11 other Echoes. State after: `top_since = 0`, `mid_since = 0`, `top_guarantee = true`. The next Illuminated is the featured Echo; it costs `E[T_top] = 39.2303` more pulls in expectation (6,277 Ink) and at most 60.

**Example B: hard pity.** `top_since = 59`. The next pull has `pt = p_bp(60) = 10000`, so `u_tier < 10000` always: Illuminated with certainty. `pm = min(600, 0) = 0`.

**Example C: Rubricated guarantee.** `mid_since = 9`, `top_since = 20`. `pt = 100`, `pm = min(10000, 9900) = 9900`. With `u_tier = 50`: Illuminated (precedence). With `u_tier = 9999`: Rubricated. A Plain result is impossible on this pull.

**Example D: spark versus Archive.** A player reaches pull 100 on `banner.echo.p003` without the featured `echo.drowned_clerk` (probability 3.0279%) and claims it with 100 spark. A second player gets `echo.drowned_clerk` at pull 49 and stops; at the banner's end 49 spark becomes 49 Vellum. After 51 more spark-Vellum from later banners, that player holds 100 Vellum, which buys either two `echo.drowned_clerk` Fold steps (80, leaving 20) or any one previously featured Illuminated Echo (100).

**Example E: duplicate at Fold 5.** The player owns `echo.wren_long_index` at Fold 5 and pulls it: Fold 6 and 10 Vellum. A later copy gives 40 Vellum.

**Example F: one full pull from bytes.** Acceptance test 8's nonce 0 gives `u_tier = 344` and `u_feat = 3861` on a fresh Echo family of `banner.echo.p001`: `pt = 100`, `pm = min(600, 9900) = 600`; `100 ≤ 344 < 700` is Rubricated; `3861 < 5000` is a rate-up; `uniform(3) = 1` selects `rate_up_mid[1] = echo.lan_ninth_peak`. State after: `top_since = 1`, `mid_since = 0`, `mid_guarantee = false`, spark 1.

---

## 4. Provable fairness

Inkwell pulls and account Reliquary opens are rolled on the economy service when online and are verifiable by the player afterwards. This is the generator Canon §3.6 names for account draws ("an HMAC-SHA256-keyed ChaCha20 generator with commit-reveal"): HMAC-SHA256 binds the secret server seed to the public inputs, and ChaCha20 (RFC 8439) expands the result into an unbounded word stream.

### 4.1 Seed epochs

| Field | Definition |
|---|---|
| Epoch scope | `(account_id, banner_id, period)`: one server seed per player per banner per period. `period` is the 28-day index of §3.2.3 for Featured, Script, Expansion and Standard banners and for `banner.reliquary.account`, and 1 for `banner.beginner`. At most one banner per family is `Live` per period (§3.2.1), so within an epoch the family's counters change only through that epoch's draws |
| `server_seed` | 32 bytes from the service's OS CSPRNG, generated when the epoch opens; stored encrypted (AES-256-GCM under a KMS key) until reveal |
| `commitment` | `SHA-256(server_seed)`, sent to the client when the epoch opens and written to the audit log before the first draw |
| `client_seed` | 1 to 64 bytes, bound to the epoch by the first draw request, which the client sends only after it has received and stored the commitment. Default (`fresh` mode): the client draws 16 new bytes from its OS RNG for every epoch after receiving its commitment, so the server cannot know the client seed when it picks `server_seed`. `pinned` mode: the player types a seed in the Fairness panel and it is reused for every epoch until changed; the panel states that only `fresh` mode rules out server seed grinding |
| `nonce` | `u64`, starts at 0, +1 per pull or per Reliquary open |
| `config_hash` | `SHA-256` of the canonical JSON of the banner record (§3.13), frozen at `Scheduled` |

Epoch state machine:

| State | Entered when | Transitions |
|---|---|---|
| `Committed` | a request that needs a draw finds no open epoch for the scope; the service returns `commitment` and the signed `pity` snapshot and performs no draw | → `Active` when the next draw request arrives carrying `client_seed`; → `Revealed` on period end, banner `Ended`, account deletion or end of service with no draw made |
| `Active` | the first draw request, which binds `client_seed` | → `Revealed` on: client seed change (pinned mode) or the player pressing "Reveal now"; period end; banner `Ended`; account deletion; end of service |
| `Revealed` | any trigger above | terminal; `server_seed` stored in plaintext and returned to the client; a new epoch opens on the next draw request, starting again at `Committed` with nonce 0 |

The `pity` receipt (§4.6) issued when an epoch opens is the verifier's starting state. A client-seed change or "Reveal now" takes effect only between draws. Every draw request carries the epoch ID and the `commitment` the client holds; a mismatch is refused with `epoch_mismatch`, so a draw can never use a seed the client has not seen committed. A Sealed Draft (§4.5) carries the client seed it was queued with and binds it at sync like any first draw.

### 4.2 Derivation from bytes to a Bp roll

```
DOMAIN_INKWELL   = "PALIMPSEST/INKWELL/v1"       (ASCII, 21 bytes)
DOMAIN_RELIQUARY = "PALIMPSEST/RELIQUARY/v1"     (ASCII, 23 bytes)

msg      = DOMAIN ‖ 0x00 ‖ config_hash (32 bytes) ‖ be32(fnv1a32(banner_id)) ‖ be32(period)
           ‖ u8(len(client_seed)) ‖ client_seed ‖ be64(nonce)
pull_key = HMAC-SHA256(key = server_seed, message = msg)                 // 32 bytes
stream   = ChaCha20 keystream (RFC 8439 §2.3), key = pull_key,
           96-bit nonce = 12 zero bytes, initial block counter = 0
next_u64 = the next 8 keystream bytes read as a little-endian u64

uniform(n):                                   // n in 1..=2^63
    lim = 2^64 − (2^64 mod n)                 // largest multiple of n not above 2^64, computed in u128
    loop { x = next_u64(); if x < lim { return x mod n } }

roll_bp = uniform(10000)                      // 0..=9999, compared with "< chance_bp"
```

- ChaCha20 is implemented in-house in the library crate `palimpsest_econ` (doc 02 `crate.econ`) and tested against RFC 8439 §2.3.2. HMAC-SHA256 uses `hmac` 0.13.0 with `sha2` 0.11.0 (R4 §16.1). The same crate is linked by the service (`crate.services`), the client (Fairness panel, offline and end-of-service draws) and `palimpsest-verify`.
- `be32` and `be64` are big-endian. `fnv1a32` is Canon §1.1.
- The xoshiro256++ generator of Canon §3 is never used for an account draw, except inside the doc 11 item generator, which receives a `u64` seed taken from this stream (§5.4). Canon §3.6 names this as the one use of `Rng::from_seed64` outside the simulation; the Keepsake enters the simulation only as a recorded item record (§5.5), so no simulation stream is read or advanced.

### 4.3 Verification tool

`palimpsest-verify` is a command-line tool shipped with the game, built as a `palimpsest_tools` binary that links `palimpsest_econ` (doc 02 §3.2). Its source code is published for inspection under the game's proprietary licence (DECISIONS 44), and this section and §3.4, §4.2 and §5.3 specify every algorithm it checks, so any player can reimplement it; and the same code runs in the in-game Fairness panel.

```
palimpsest-verify pull   --record pull.json --banner banner.json --seed-reveal reveal.json --snapshot pity.json
palimpsest-verify epoch  --history history.json --banner banner.json --seed-reveal reveal.json --snapshot pity.json [--receipts r.json] [--content <hash>]
palimpsest-verify chain  --histories <export_dir>
```

Verification steps, all of which must pass:

1. `SHA-256(server_seed) == commitment`.
2. `SHA-256(canonical(banner.json)) == config_hash`.
3. The epoch-start pity snapshot (`kind = "pity"`, §4.6) has a valid Ed25519 signature by a key valid at its `iat`.
4. Starting from the snapshot, replay every nonce in order through §3.4 (or §5.3) and compare `u_tier`, `u_feat`, `idx`, tier, entry, featured or rate-up flag, spark and pity-after with each record (Reliquaries: both uniform draws, rarity, `item_seed` and counters). With `--receipts`, also compare each duplicate grant (§3.8) against the ownership state folded from the account's `grant` and `revoke` receipts; with `--content <content_hash>`, also regenerate each Keepsake from `item_seed` through doc 11's generator and compare the item record.
5. Nonces are contiguous from 0 with no gap and no repeat.
6. **Chain continuity.** The epoch's starting snapshot equals the pity-after of the last record of the previous revealed epoch of the same family (or of `banner.reliquary.account`), or the fresh state when no earlier epoch exists. `palimpsest-verify chain --histories <dir>` checks every epoch of an export in order, so the service cannot silently edit counters between epochs.
7. Every record's `config_hash` and `banner_id` match the banner record supplied, and its UTC timestamp lies in `[start_utc, end_utc)` of that banner.

Output: one `PASS` or `FAIL <reason>` line per record; exit code 0 when all pass, else 1. Changing the client seed forces a reveal at any time.

### 4.4 Audit log

- One append-only table holds every pull, spark event, Archive purchase, Transcription claim, Reliquary open, Keepsake carry, Codex or store grant, refund, revocation and ledger transaction.
- Each record is canonical JSON. `h_i = SHA-256(h_{i−1} ‖ canonical(record_i))` with `h_0 = 32 zero bytes` per shard; shard = `account_id mod 256`.
- At 00:05 UTC daily, the service computes a Merkle root (SHA-256, binary, odd leaves promoted) over the previous UTC day's record hashes, signs it with the receipt key (§4.6) and publishes it at `/audit/<YYYY-MM-DD>`.
- Retention: 1,826 days (5 years, R3 C4 LOG-1, exceeding China's 90-day minimum and supporting Korea's reversed burden of proof). Daily roots are kept forever.
- Records identify accounts by the pseudonymous `account_id` only.

### 4.5 Offline operation

| Mode | Who | Inkwell and Reliquaries | Ink and XP earning |
|---|---|---|---|
| Temporary offline | an online account without a connection | pulls and opens queue as **Sealed Drafts**: the client debits provisional Ink locally and records `(banner_id, client_seed, count, queued_at)`; on reconnect the service applies pending run reports first, then resolves drafts in queue order with the next nonces of the current epoch; a draft whose banner reached `Ended` before sync is cancelled and its Ink restored; a draft that exceeds the server balance is cancelled and restored | run reports queue locally and are credited on sync under the same caps (§2.3) with the original run timestamps |
| Offline-only profile | a local profile created with "Never connect", or forced by §8.4 | runs locally under the §11.3 rules with two exceptions until the service reaches `EndOfService`: Codex premium tracks stay locked (they unlock only through §11.3 at end of service), and the store is hidden; the Inkwell offers the Retrospective banner with local seeds | local, under §2 and §6 rates without the rolling caps |
| End of service | every account after shutdown | §11.3 | local |

An offline-only profile never merges into an online account. Platform-owned fiat SKUs (base game, expansions, cosmetics, Codex premium) are honoured in every mode through the platform's offline ownership check and cached receipts.

### 4.6 Signed receipts

Receipts let the client prove ownership and balances without the service.

- Signature: Ed25519 (`ed25519-dalek` 3.0.0) over `"PALIMPSEST/RECEIPT/v1" ‖ 0x00 ‖ canonical_json(receipt)`.
- Canonical JSON: keys sorted by byte order, no whitespace, ASCII strings, integers only, `u64` as `"0x"` + 16 lowercase hex digits.
- Kinds: `grant`, `revoke`, `wallet` (all balances), `pity` (all families, spark, Reliquary counters, `folio_exits`, the lifetime Folio-exit counter of §5.1, and `transcription`), `codex` (all Codex progress), `eos_final` (§11).

| Field | Type | Meaning |
|---|---|---|
| `v` | integer | schema version, 1 |
| `kind` | string | one of the kinds above |
| `rid` | hex u64 | receipt ID, unique |
| `acct` | hex u64 | account ID |
| `item` | content ID | granted or revoked content (absent for snapshots) |
| `qty` | integer | quantity |
| `src` | content ID | the source: a SKU, banner, `pass.` record or Archive |
| `seq` | integer | per-account monotonic sequence |
| `iat` | integer | issue time, Unix seconds |
| `kid` | string | signing key ID, `rk_<yyyy><letter>` |

Example receipt (signed with the RFC 8032 test key 1, secret `9d61b19d…7f60`, public `d75a9801…511a`):

```json
{"acct":"0x0123456789abcdef","iat":1821052800,"item":"cosm.outfit.codex_001_t005","kid":"rk_2027a","kind":"grant","qty":1,"rid":"0x00000000000003e9","seq":7,"src":"pass.codex_001","v":1}
```

`iat` 1821052800 is 2027-09-16 00:00:00 UTC, one week after `T0`, when `pass.codex_001` is live; `rk_2027a` is the first signing key of 2027. The signed message is 208 bytes (21-byte prefix, 1 separator byte, 186-byte JSON). Signature: `f3510f1f088b6a600b24e74cad059d8cabf15ff4515ff7e0a53b5f158e4e879e7f3be819de2ee723385d6257d05a487bd42c8e959cb8249463866e4ece944b0b`.

Key management: signing keys live in a KMS; one key per calendar year; the client embeds every public key ever used with `valid_from` and `valid_to`, plus a revocation list `(kid, revoked_at)`. A receipt is valid when its signature verifies, its `iat` lies in the key's validity window, and the key was not revoked before `iat`. The owned set is the fold of all valid `grant` and `revoke` receipts in `seq` order.

---

## 5. Account Reliquaries

An **account Reliquary** (`banner.reliquary.account`) is an earned loot box held unopened as the ledger asset `RELIQUARY` (`purchasable = false`). Opening one in the Margin yields exactly one **Keepsake**: a Sealed item instance rolled from a Folio loot table. Keepsakes enter runs through the Satchel (§5.5). In-run Reliquaries and their keys belong to doc 11 and are separate.

An account Reliquary holds a Keepsake and, through the Keepsake's `illuminated` roll (§5.4), sometimes a cosmetic skin; it never holds raw power (DECISIONS 59): a Keepsake has the power of a found item of its Folio, and the Satchel limits of §5.5 bound how many enter a run.

### 5.1 Sources

| Source | Reliquaries | Source genre recorded |
|---|---|---|
| Editor kill | 1 per `rank.editor` killed while the player is present (§2.2), recorded on the run at the Endpaper (doc 14 §3.9 step 4) and credited at run end whatever the end state (doc 14 §4.5) | the genre of that Folio: `genre.ur_text` for the Ur-Text Folio (25 in Standard, 15 in Abridged); for a fused Second Edition Folio (doc 14 §8.9), its `FolioRecord.genre`, which is the genre of its pair with the lower Canon §5.2 row number |
| Folio completion | 1 each time the account's lifetime count of qualified Folio exits (§2.2 definition, from runs whose mode row in §2.2 grants Reliquaries) reaches a multiple of 30 | the genre of the 30th Folio, by the same rule |
| Codex free track | 1 at tiers `10j + 6` | the Codex's theme genre |
| Codex Coda | 1 per 24,000 overflow XP (§6.5) | the Codex's theme genre |

Never sold, never in any SKU, never from Apocrypha or easing runs (§2.2 mode rules). Per reference hour: 2 (Editors of Folios 5 and 10) + 12/30 (Folio exits) + 10 per 60 hours (Codex) = 2.5667, shown as 2.57. Each grant mints one `RELIQUARY` unit (`mint/reliquary/editor`, `mint/reliquary/folio`, `mint/reliquary/codex`, `mint/reliquary/coda`) carrying its source genre; the first one sets `margin.milestone.first_reliquary` (doc 14 §5.1).

### 5.2 Keepsake vault

- `KEEPSAKE_VAULT_CAP = 30`. Opening requires a free vault slot; unopened Reliquaries have no cap.
- The player can discard a Keepsake at any time (confirmation dialog, no refund, no conversion).

### 5.3 Rarity table and bad-luck counters

| Rarity | Base weight (Bp) | Base | Consolidated with counters |
|---|---|---|---|
| `rar.common` | 0 | 0.0000% | 0.0000% |
| `rar.uncommon` | 7400 | 74.0000% | 60.4386% |
| `rar.rare` | 2000 | 20.0000% | 16.3348% |
| `rar.epic` | 450 | 4.5000% | 15.4192% |
| `rar.legendary` | 120 | 1.2000% | 6.2459% |
| `rar.exotic` | 30 | 0.3000% | 1.5615% |
| `rar.palimpsestic` | 0 | 0.0000% | 0.0000% |

Per-account counters, both starting at 0: `c5` = Reliquaries opened since the last Epic-or-better, `c20` = since the last Legendary-or-better.

| Guarantee | Rule |
|---|---|
| Every Reliquary | Uncommon or better (Common weight is 0) |
| Every 5th | when `c5 = 4` and the rolled rarity is below Epic, it is upgraded to Epic or better |
| Every 20th | when `c20 = 19` and the rolled rarity is below Legendary, it is upgraded to Legendary or better |

```rust
fn open_reliquary(acct: &mut Account, rq: &ReliquaryInstance, epoch: &mut SeedEpoch) -> Keepsake {
    let mut fs = FairStream::new(pull_key(RELIQUARY, epoch, epoch.next_nonce)); epoch.next_nonce += 1;
    let u = fs.uniform(10_000) as u32;                                   // draw 1
    let mut r = pick(u, &[(Uncommon, 7400), (Rare, 2000), (Epic, 450), (Legendary, 120), (Exotic, 30)]);
    let (c5, c20) = (acct.rq.c5, acct.rq.c20);
    if c20 == 19 && r < Legendary {
        let v = fs.uniform(150) as u32;                                  // draw 2
        r = if v < 120 { Legendary } else { Exotic };
    } else if c5 == 4 && r < Epic {
        let v = fs.uniform(600) as u32;                                  // draw 2
        r = if v < 450 { Epic } else if v < 570 { Legendary } else { Exotic };
    } else {
        let _ = fs.uniform(1);                                           // draw 2, discarded
    }
    let item_seed = fs.next_u64();                                       // draw 3, raw
    match r {
        Legendary | Exotic => { acct.rq.c5 = 0; acct.rq.c20 = 0; }
        Epic              => { acct.rq.c5 = 0; acct.rq.c20 = c20 + 1; }
        _                 => { acct.rq.c5 = c5 + 1; acct.rq.c20 = c20 + 1; }
    }
    let item = doc11::generate_keepsake(rq.source_genre, r, Rng::from_seed64(item_seed)); // §5.4
    Keepsake::sealed(item)
}
```

`pick(u, table)` walks the table in order and returns the first row whose cumulative weight exceeds `u`. Draw count: always 2 uniform values and 1 raw `u64` from the fair stream, then whatever doc 11's generator draws from `Rng::from_seed64(item_seed)`.

Consolidated figures (exact Markov chain over the 90 reachable `(c5, c20)` states): Epic-or-better 23.2266% (one per 4.305 Reliquaries), Legendary-or-better 7.8074% (one per 12.808). From fresh counters, the expected number of Reliquaries to the first Epic-or-better is 4.435 and to the first Legendary-or-better is 12.808. P(the first Legendary-or-better arrives by the 10th Reliquary) = 45.0242%.

### 5.4 Contents from Folio loot tables

`generate_keepsake(genre, rarity, rng)` calls doc 11's item generator with:

| Input | Value |
|---|---|
| Loot table | `loot.keepsake.<genre_suffix>` (doc 11; one per genre, eligible templates only at each rarity) |
| Rarity | forced to the rolled `r`; the generator never rerolls rarity |
| `ilvl` | 1 (re-levelled on carry-in, §5.5) |
| `origin_genre` | the Reliquary's source genre |
| Eligible templates | base content plus the opener's account content set at open time (doc 14 §2.3) |
| Quality, affixes, curse state, `illuminated` | rolled by doc 11 (§2.3, Keepsake entry point) and doc 10 rules from `rng`. The curse roll is doc 10 §7.1's `roll_curse` with the source genre's MYTHOS, so on a pact-bearing category a cursed Keepsake carries a pact (Canon §13.6, DECISIONS 60) with doc 10 §7.8's share `PACT_SHARE_BP = 2000` of its curse chance. The `illuminated` roll is doc 11 §2.3 step 13's 5 Bp roll for Legendary or better, drawn from `rng`, never from `rng.loot` |
| `enhancement` | stored 0 (doc 10 §2.1: no generator sets it) |
| `gear_tier`, `perk_columns` | `gear_tier = 1` permanently; perk columns are generated for tier 1 (1 option per column) |
| `manufacturer` | rolled by doc 11 from `rng` |
| Serendipity | fixed at the neutral `SER = 40` (doc 11 §2.3, Keepsake entry point), so the opener's Serendipity never shifts a Keepsake (Canon §6.4 row 42 is in-run only; Canon §22) |

The full item instance (Canon §13.9) is stored in the vault at open time with `uid = 0` and `finder_slot = 0`. Its identity (template, rarity, quality, affixes and their rolls, enhancement, curse state, perk options, manufacturer, `illuminated`) never changes afterwards; only `uid`, `ilvl`, `value` and `finder_slot` are set at materialization (§5.5).

### 5.5 Sealed contents, identification and the Satchel

- **Sealed.** A Keepsake shows its silhouette, its source-genre tag and its rarity frame. Its knowledge state follows Canon §13.6: unknown-appearance classes start at `know.unknown`, everything else at `know.classified`; the rarity knowledge flag alone is set. Affixes, enhancement, curse state and charges stay hidden.
- **Satchel.** In the lobby of a run whose mode row in §2.2 allows the Satchel (Standard, Abridged, Novella, Endless, Custom), each player orders up to `SATCHEL_SIZE = 5` Keepsakes into a Satchel, with at most `SATCHEL_MAX_LEGENDARY_PLUS = 1` of rarity Legendary or Exotic (Keepsakes are never Palimpsestic). A Keepsake whose template is outside the run's content set (doc 14 §2.3; the host's set in co-op) cannot be placed and shows "Not in this run's pages". The host records each player's Satchel (full item records) in `RunState` at run start, so saves and replays carry it.
- **Materialization.** At the tick of `ev.folio_entered` for the first Folio of each Volume, after Ghost restoration (doc 14 §3.10), the next Keepsake of each present player's Satchel materializes in that player's inventory, in ascending party-slot order. The first Folios of the Volumes are Folio indices 1, 6, 11, 16 and 21 in the Standard structure (Novella stops at 11, a 10-Folio Custom run at 6) and 1, 4, 7, 10 and 13 in Abridged, whose depths `d(f)` (doc 14 §3.1, Canon §17.1) are 1, 6, 11, 16 and 21. The Keepsake receives a new `uid` from that Folio's item counter (Canon §1.11), `ilvl = d(f)`, `value` recomputed by doc 11 from `P(ilvl)`, and `finder_slot` = the player's slot. If the inventory is full, it is placed on the ground at the player's position. A Dormant or Departed slot materializes nothing at that Folio; its Keepsake stays queued for the next listed Folio.
- **Mid-run joiners** (doc 14 §2.5) submit their Satchel when they join; it materializes at the listed indices greater than or equal to the join Folio.
- A materialized Keepsake leaves the vault permanently, whatever happens to the item later. Unmaterialized Satchel Keepsakes return to the vault at run end; if the vault is then over its cap, the return still succeeds and opening is blocked until the count falls to 29 or fewer.
- **Balance.** A Keepsake adds one item per 5 Folios to a run whose reference supply is 40 standard-kill equivalents per Folio (Canon §17.4). Its `ilvl` tracks the Folio exactly like found loot, so its damage scales with `P(ilvl)` (Canon §13.8; for example `P(21) = 56044` at Folio 21) and it stays inside `ITEM_MULT_MAX = 51870` like any doc 11 item; it can never outscale the run. The cap of one Legendary-or-better item bounds the Folio-1 spike: a Legendary weapon at ilvl 1 has rarity power 12500 against 10000 for the Common reference weapon (Canon §13.3), cutting the 2.0 s reference time-to-kill (Canon §17.4) to `floor_div(2000 × 10000, 12500) = 1600` ms before affixes. Without the cap, a full vault would let five chosen Legendary or Exotic items enter every run, a raw-power meta gain that DESIGN_BRIEF §4 forbids. `gear_tier = 1` (§5.4) also keeps Keepsakes below found gear at Grade 5 and above.
- Identification happens in the run through doc 10 (appraisal, use, scrolls). A cursed Keepsake behaves exactly like any cursed item. When a Keepsake becomes `know.identified`, its template is recorded in the account's Index (doc 10 §12.1; `margin.index`, doc 14 §5.1). "Codex" in this chapter always means the battle pass of §6.

### 5.6 Disclosure

- The Reliquary tooltip shows: base and consolidated rarity rates, "Epic or better guaranteed within `5 − c5` openings", "Legendary or better guaranteed within `20 − c20` openings", and a link to `/odds/reliquary/<genre_suffix>`.
- Per-template probability on the public page: `P(rarity r) × w_template / Σ w(r)` from the `loot.keepsake.<genre_suffix>` weights, shown with `pct4`. Example: a template with weight 5,000 out of 10,000 in the Epic band of the Wuxia table has base probability `450/10000 × 1/2`, displayed `2.2500%`.
- Curse and pact odds on the same page: per category and rarity, the curse chance `curse_bp` and the pact chance `pact_bp = mul_bp(curse_bp, 2000)` that doc 10 §7.1 and §7.8 compute for the genre's MYTHOS (Keepsakes use `source_curse_factor = 10000`), shown with `pct4`. Example: an Uncommon to Epic Gothic Horror weapon Keepsake has `curse_bp = 2400` and `pact_bp = 480`, displayed `24.0000%` cursed and `4.8000%` pact; a Legendary or Exotic one has 1200 and 240 (`rarity_factor` 5000). A pact is always shown openly with its odds (DECISIONS 60).
- No achievement, title or reward ever references owning a set of Keepsakes or Inkwell entries (R3 C4 SET-1 hygiene, Japanese kompu gacha rule).

### 5.7 Record and worked example

```json
{
  "id": "banner.reliquary.account",
  "kind": "account_reliquary",
  "weights_bp": { "rar.uncommon": 7400, "rar.rare": 2000, "rar.epic": 450, "rar.legendary": 120, "rar.exotic": 30 },
  "pity_epic_every": 5,
  "pity_legendary_every": 20,
  "upgrade_epic_weights": { "rar.epic": 450, "rar.legendary": 120, "rar.exotic": 30 },
  "upgrade_legendary_weights": { "rar.legendary": 120, "rar.exotic": 30 },
  "vault_cap": 30,
  "satchel_size": 5,
  "satchel_max_legendary_plus": 1,
  "satchel_depths": [1, 6, 11, 16, 21]
}
```

Worked example: counters `c5 = 4`, `c20 = 11`. Draw 1 is `u = 4774`, below 7400: Uncommon. Because `c5 = 4` and Uncommon is below Epic, draw 2 is `uniform(600) = 87`, below 450: Epic. Counters become `c5 = 0`, `c20 = 12`. The tooltip now reads "Epic or better within 5" and "Legendary or better within 8".

---

## 6. The Codex battle pass

### 6.1 Structure

| Rule | Value |
|---|---|
| ID pattern | `pass.codex_<nnn>`, `nnn` from 001 |
| Tiers | 100 |
| XP per tier | `CODEX_XP_PER_TIER = 12,000`, flat; 1,200,000 XP for tier 100 |
| Tier from XP | `tier = min(100, floor_div(xp, 12000))` |
| Tracks | free and premium share one progress value per Codex |
| Active Codex | exactly one per account receives all Codex XP; switching is free and loses nothing |
| Expiry | never; every Codex ever released stays selectable forever |
| Timers | none; no countdown, no "season ends" text, no "last chance" UI |
| Tier skips, XP boosters | never sold (SEP-4) |
| Premium price | USD 9.99 per Codex (`store.sku.codex_<nnn>_premium`), or one held Quill |

### 6.2 XP sources

All play earns Codex XP, except invading another party, which Canon §17.9 and DECISIONS 86 limit to Quires and Red Ink marks (§2.4). No source is a challenge, quest or daily task. The §2.2 mode rules decide which sources a run pays; "present" and "qualified Folio exit" have their §2.2 meanings.

| Source ID | Trigger | XP |
|---|---|---|
| `pass.xp.active_minute` | 3,600 active ticks (§2.2), at most 240 per run | 150 |
| `pass.xp.folio_exit` | qualified Folio exit (present at `exit_tick`, dwell ≥ 5,400 ticks) | 400 |
| `pass.xp.editor_kill` | `rank.editor` killed with the player present | 1,000 |
| `pass.xp.kill` | `ev.entity_killed` of a hostile of any rank except `rank.editor`, counted for every party member present at the kill tick | per run, `Σ over party sizes n of floor_div(10000 × K[n], enemy_count_bp[n])`, where `K[n]` is the sum of `floor_div(5 × rank_xp_bp, 10000)` over counted kills made while the Folio's party size (doc 14 §8.8) was `n` |
| `pass.xp.run_end` | `ev.run_ended` of a run that paid no Ur-Text victory XP, after ≥ 3 qualified Folio exits for this player (Novella, Daily, Weekly Gauntlet and `run.mut.short_book` victories included) | 800 |
| `pass.xp.run_victory` | `boss.ur_editor` dies with the player present and the run keeps its clear (§2.2 Second Edition rule; `final_folio` read before any Endless or Second Edition switch, as in §2.2) | `VICTORY_XP_PER_FOLIO × final_folio` = 160 × 25 = 4,000 for the Standard structure, 160 × 15 = 2,400 for Abridged |
| `pass.xp.type_identified` | `ev.item_identified` that makes an item type known for the first time in the run, to the identifying player | 25 |
| `pass.xp.craft` | `ev.item_crafted` or `ev.potion_brewed`, to the crafter | 25 |
| `pass.xp.revive` | `ev.player_revived`, to the reviver | 100 |
| `pass.xp.codex_entry` | each Index template page or Bestiary page that opens for the first time on the account (doc 10 §12.1: a template's first `know.identified` instance; §12.4: `know.bestiary.sighted`), to each player whose account it opens; listed in the run report as `codex_entries_new`; Index skin entries and curse pages pay nothing. The ID names the pass XP source; the page itself is an Index or Bestiary page | 200 |
| `pass.xp.achievement` | any achievement or Codex mark unlock (doc 14 §10) | 2,000 |

Kill XP before the split, from the killed entity's kill XP multiplier `rank_xp_bp` (Canon §17.3): minion 1, standard 5, elite 15, champion 30, boss 100, an invader (`rank.hunter`) 25. The Erasure hunter carries `rank.hunter` but uses `rank.elite`'s kill XP (Canon §11.6, DECISIONS 91), so it gives 15. The split divides once per run and party size, not once per kill, so co-op loses nothing to flooring: Canon §17.2 raises enemy counts by the same `enemy_count_bp`, and a Folio of 40 standard-kill equivalents solo becomes `floor_div(40 × 17500, 10000) = 70` kills in a party of 4, giving each player `floor_div(10000 × 350, 17500) = 200` XP, exactly the solo 200. A per-kill split would give `70 × floor_div(50000, 17500) = 140`, a 30% co-op penalty; this rule removes it. A run with a single standard kill in a party of 4 gives `floor_div(50000, 17500) = 2`.

**Reference hour.** The §2.2 reference failed run plus 480 standard kills (12 Folios × 40), 20 identifications and 20 crafts: `60 × 150 + 12 × 400 + 2 × 1000 + 480 × 5 + 800 + 20 × 25 + 20 × 25 = 9,000 + 4,800 + 2,000 + 2,400 + 800 + 500 + 500 = 20,000`. `REF_CODEX_XP_PER_HOUR = 20,000`, so tier 100 takes 60 reference hours without catch-up.

**Victory run example.** 125 active minutes, 24 Folio exits, 5 Editors, 960 standard kills, victory, 40 identifications, 30 crafts: `18,750 + 9,600 + 5,000 + 4,800 + 4,000 + 1,000 + 750 = 43,900` XP, which is 3 tiers and 7,900 XP.

### 6.3 Weekly catch-up reserve

| Rule | Value |
|---|---|
| Grant | +40,000 reserve XP at every ISO week start (Monday 00:00 UTC); a new account starts with 40,000 |
| Cap | 160,000 (four weeks) |
| Effect | each Codex XP grant `g` from §6.2, after the §2.3 rolling cap clips it, adds `bonus = min(g, reserve)`; `reserve −= bonus`; the Codex receives `g + bonus`. Grants are applied in run-report order, and within one report in §6.2 table order |
| Scope | account-wide; applies to the active Codex; achievements and play sources alike |

The reserve doubles earned XP until it is spent. Absence fills the reserve up to four weeks and never removes progress. The reserve is a rested bonus, never a streak.

### 6.4 Reward schedule template

Each 10-tier block `j ∈ 0..=9` (tiers `10j + 1` to `10j + 10`):

| Tier in block | Free track | Premium track |
|---|---|---|
| +1 | — | emote (`cosm.emote.*`) |
| +2 | 80 Ink | weapon ink (`cosm.ink.*`) |
| +3 | — | player sigil (`cosm.sigil.*`) |
| +4 | 25 Quires | spell VFX palette (`cosm.vfx.*`) |
| +5 | — | Echo or Erratum outfit (`cosm.outfit.*`) |
| +6 | 1 account Reliquary | portrait frame (`cosm.frame.*`) |
| +7 | — | title (`cosm.title.*`) |
| +8 | 80 Ink | weapon ink (`cosm.ink.*`) |
| +9 | — | Margin decoration (`cosm.margin.*`) |
| +10 | free cosmetic (`cosm.*`; at tiers 50 and 100 an outfit) | signature set piece (`cosm.trail.*` or animated outfit variant); at tier 100: **1 Quill** plus the capstone outfit |

Totals per Codex: free track 1,600 Ink, 250 Quires, 10 Reliquaries, 10 cosmetics. Premium track 100 cosmetics and 1 Quill. Premium rewards are deterministic, listed in full on the Codex screen before purchase, and previewable in 3D.

### 6.5 Quill payback, purchase timing, switching and overflow

- **Payback.** Reaching tier 100 on a Codex whose premium track the account owns grants exactly one `ent.quill`. One purchase therefore funds every later premium track the player completes: 100% payback without currency.
- **Late purchase.** Buying a premium track grants every premium reward for tiers already reached, at once, including the tier-100 Quill if tier 100 is reached.
- **Switching.** XP earned goes to the active Codex. Progress on every other Codex is kept.
- **Overflow (Coda).** XP earned while the active Codex is at tier 100 accumulates in `coda_xp`; every 24,000 grants 1 account Reliquary, free or premium alike. The Codex screen suggests an unfinished Codex when one exists.

### 6.6 Cadence

Codex `N` releases at `T0 + 84 × (N − 1)` days, 00:00 UTC, themed on the newest Folio Expansion genre if one shipped since the previous Codex, otherwise on the next core genre in Canon §5.2 order, cycling (every build ships all 12, DECISIONS 114). Release never ends or changes any earlier Codex.

### 6.7 Expected completion

Computed by week-by-week simulation of §6.3 at 20,000 XP per hour (exact; recomputed with Python `fractions` for this review). The table assumes no day exceeds 10 hours of play, so the §2.3 Codex rolling cap never binds, and a new account (starting reserve 40,000):

| Hours played per week | Weeks to tier 100 | Hours to tier 100 | Tier after 12 weeks |
|---|---|---|---|
| 1 | 30.00 | 30 | 40 |
| 2 | 15.00 | 30 | 80 |
| 3 | 12.00 | 36 | 100 |
| 4 | 10.00 | 40 | 100 |
| 5 | 8.40 | 42 | 100 |
| 6 | 7.33 | 44 | 100 |
| 8 | 6.00 | 48 | 100 |
| 10 | 5.00 | 50 | 100 |
| 15 | 3.47 | 52 | 100 |
| 20 | 2.70 | 54 | 100 |
| no reserve | — | 60 | — |

A player at 3 hours per week completes each Codex inside its 12-week cadence. Worked example at 5 hours per week: each week earns `100,000 + min(100,000, 40,000) = 140,000` XP; after 8 weeks 1,120,000 XP (tier 93); week 9 needs 80,000 more, which the doubled first 40,000 of base XP supplies in 2 hours; total `8 × 5 + 2 = 42` hours.

### 6.8 Record

```json
{
  "id": "pass.codex_001",
  "theme_genre": "genre.mythic_fantasy",
  "release_utc": 1820448000,
  "xp_per_tier": 12000,
  "tiers": 100,
  "premium_sku": "store.sku.codex_001_premium",
  "free": [
    { "tier": 2, "grant": "cur.ink", "qty": 80 },
    { "tier": 4, "grant": "cur.quire", "qty": 25 },
    { "tier": 6, "grant": "banner.reliquary.account", "qty": 1 },
    { "tier": 8, "grant": "cur.ink", "qty": 80 },
    { "tier": 10, "grant": "cosm.sigil.codex_001_quill_rampant", "qty": 1 }
  ],
  "premium": [
    { "tier": 1, "grant": "cosm.emote.codex_001_bow", "qty": 1 },
    { "tier": 5, "grant": "cosm.outfit.codex_001_t005", "qty": 1 },
    { "tier": 100, "grant": "ent.quill", "qty": 1 }
  ]
}
```

A full record lists 50 free and 101 premium rows (tier 100 has two). Validator rules: every premium `grant` is a `cosm.` ID or `ent.quill` (SEP-3); `ent.quill` appears exactly once, at premium tier 100; free grants are `cur.ink`, `cur.quire`, `banner.reliquary.account` or `cosm.` IDs; `xp_per_tier = 12000`; `tiers = 100`.

---

## 7. Store

### 7.1 Catalog

| SKU | USD | Contents (all deterministic, all listed before purchase) | Limit |
|---|---|---|---|
| `store.sku.base_game` | 19.99 at Early Access and 1.0 | the game | 1 |
| `store.sku.folio_exp_<nn>` | 9.99 | one expansion genre and its banners and Archive shelves (§3.2.6) | 1 each |
| `store.sku.codex_<nnn>_premium` | 9.99 | premium track of that Codex | 1 each |
| `store.sku.supporter_pack` | 4.99 | weapon ink set (5 `cosm.ink.*`), 1 sigil, title "Patron of the Margin", digital soundtrack (FLAC and MP3), digital art book (PDF) | 1 |
| `store.sku.cosm_<name>` tier S | 1.99 | 1 emote, or 1 sigil plus 1 title | 1 each |
| tier M | 3.99 | 1 weapon ink set (10 inks) | 1 each |
| tier L | 5.99 | 1 outfit for one Echo or for custom Errata | 1 each |
| tier XL | 7.99 | 1 spell VFX palette set | 1 each |
| tier Folio set | 9.99 | 1 outfit, 1 ink set, 1 VFX palette and 1 trail sharing a theme | 1 each |

- Every cosmetic SKU grants a `bundle.sku.<name>` whose grant list is fixed at release and never edited.
- **One source per cosmetic.** Every `cosm.` item has exactly one source: one cosmetic SKU, the supporter pack, one Codex premium tier, one Codex free tier, the Inkwell Plain pool, an Echo Fold, a Recollection, an achievement, a Bindery node, a Red Ink mark count or the Second Edition Ironman clear (§2.2). The content validator rejects a second source. Grant lists of different SKUs therefore never overlap, a SKU is either wholly owned (shown "Owned", not purchasable) or wholly unowned, and no purchase ever pays for something already owned.
- **Complete-the-set bundles** (`store.sku.set_<name>`) list component SKUs. Price in each currency: `raw = mul_bp(Σ local prices of component SKUs not owned, 9000)` in minor units, then `price = round_down_ending(raw, currency)`. The bundle is hidden when fewer than 2 components are unowned. Ownership lowers the price; it never raises it.

```
round_down_ending(v, cur):            // largest valid price <= v, in minor units
    USD, EUR, BRL:  v - ((v - 99) mod 100)            // ends in .99
    GBP, CAD:       v - ((v - 9) mod 10)              // last digit 9
    AUD:            largest p <= v with p mod 100 in {45, 95}
    JPY:            v - (v mod 50)
    KRW:            v - (v mod 100)
    CNY:            v - (v mod 100)                   // whole yuan
```

Worked example: a USD set of four unowned SKUs at 5.99, 3.99, 7.99 and 1.99 has `raw = mul_bp(1996, 9000) = 1796`, and `round_down_ending(1796, USD) = 1796 − (1697 mod 100) = 1796 − 97 = 1699`: USD 16.99.
- Cosmetics never alter hitboxes, silhouettes beyond doc 18's readability limits, sound cues used by stealth, or any simulation value.

### 7.2 Regional pricing method

1. Each SKU has a USD anchor tier.
2. The local price in each platform currency is the storefront's recommended regional price for that anchor at the SKU's release date (Steam's pricing tool: Steam is the only storefront and the game ships on PC only, DECISIONS 98 and 112), rounded to the nearest price satisfying the local ending convention (`.99` for USD, EUR and BRL; last digit 9 for GBP and CAD; `.95` or `.45` for AUD; multiples of 50 for JPY; multiples of 100 for KRW; whole yuan for CNY), ties to the lower price. Currencies outside the matrix use the platform's own recommendation unrounded.
3. Prices are stored as integer minor units per currency in `store.price_matrix`.
4. A SKU's price never rises within 12 months of its release. Any later increase is announced in-game 30 days ahead. Decreases apply immediately. Price changes never affect past purchases.
5. The matrix is reviewed every 6 months.

Launch matrix (authored values, binding until the first review):

| USD | EUR | GBP | BRL | JPY | KRW | AUD | CAD | CNY |
|---|---|---|---|---|---|---|---|---|
| 1.99 | 1.99 | 1.69 | 7.99 | 300 | 2,500 | 2.95 | 2.59 | 9 |
| 3.99 | 3.99 | 3.49 | 14.99 | 600 | 4,900 | 5.95 | 4.99 | 18 |
| 4.99 | 4.99 | 4.29 | 18.99 | 750 | 6,200 | 7.45 | 6.49 | 22 |
| 5.99 | 5.99 | 5.19 | 22.99 | 900 | 7,400 | 8.95 | 7.79 | 28 |
| 7.99 | 7.99 | 6.89 | 29.99 | 1,200 | 9,800 | 11.95 | 10.49 | 36 |
| 9.99 | 9.99 | 8.59 | 37.99 | 1,500 | 12,300 | 14.95 | 12.99 | 46 |
| 19.99 | 19.99 | 16.99 | 74.99 | 2,900 | 24,400 | 29.95 | 25.99 | 90 |

The 19.99 row, the base game's price, is the **conversion anchor**: an amount `a` in currency `X` converts to currency `Y` as `floor_div(a × anchor_Y, anchor_X)` in minor units (used only for minor caps, §8.3). Example: JPY 5,000 into EUR is `floor_div(5000 × 1999, 2900) = 3446`, EUR 34.46.

### 7.3 Real-price display

- Every store card, detail page and confirm dialog shows the price in the player's platform currency as the final amount to pay. Where `juris.flag.vat_inclusive_display` is true the shown price includes tax; elsewhere the card shows the pre-tax price with "+ tax" and the confirm dialog shows the tax-inclusive total before the confirm button (UK DMCC all-in pricing, R3 C1 row 3b).
- There is no virtual currency anywhere in the store, so no exchange rate exists to disclose.
- The confirm dialog lists every item granted, the refund terms of §7.4, and, where `juris.flag.withdrawal_waiver_consent` is true, an unticked checkbox "I want immediate access and understand I lose my 14-day withdrawal right once I equip or claim the content". Purchase proceeds with the box unticked; the withdrawal right then holds for 14 days regardless of use.
- A player-facing **Spend summary** (Settings → Purchases) shows lifetime, last-30-day and per-SKU spend in local currency.

### 7.4 Refunds

| Case | Window | Condition | Effect |
|---|---|---|---|
| Cosmetic SKU | 14 days | never equipped | full refund; grants revoked |
| Codex premium | 14 days | tier-100 Quill of that Codex not yet granted | full refund; premium rewards revoked (no Quill exists yet to chase) |
| Folio Expansion | platform policy, and at least 14 days | under 2 hours played in the expansion genre | full refund; expansion banners and Archive shelves hidden; expansion Echoes and Scripts already obtained stay owned but unusable until re-purchase |
| Supporter pack | 14 days | none | full refund; grants revoked |
| Purchase by a minor-flagged account without guardian approval | 90 days | guardian request | full refund regardless of use |
| EU or UK withdrawal right not waived | 14 days | none | full refund |

Refunds route through the platform where the platform processes payment. Every refund posts a `revoke` receipt (§4.6). Earned content (Ink, Quires, Vellum, Echoes, Scripts, Folds, Keepsakes, Codex XP, free-track rewards) is never revoked by a refund.

### 7.5 No-FOMO rules

- Every released SKU stays purchasable for the life of the service; holiday cosmetics stay on a year-round "Seasons" shelf.
- No daily or rotating shop, no "limited stock", no countdown, no "leaving soon", no in-game time-limited discount. Discounts happen only on platform storefront pages, never inside the game.
- New SKUs show a "New" label for 14 days. The label has no timer.
- No offer is personalized by spend, play time or behaviour. Every player in a region sees the same catalog in the same order (newest first, then by price ascending).
- No pop-up store prompt appears outside the store screen. The Margin never interrupts play with offers.

### 7.6 Purchase flow

| State | Entered when | Next |
|---|---|---|
| `Browsing` | store opened | `Locked` if purchases are disabled for the account; else `Confirming` on "Buy" |
| `Locked` | `purchases_enabled = false` | shows "Purchases are off. Turn on in Settings → Purchases (account holder) or ask your guardian." |
| `Confirming` | "Buy" pressed | `PlatformCheckout` on confirm when the §8.4 cap check passes; `Browsing` with `minor_cap_tx`, `minor_cap_month` or `minor_store_blocked` when it fails; `Browsing` on cancel |
| `PlatformCheckout` | platform payment UI opened | `Validating` on platform success; `Browsing` on platform cancel or failure |
| `Validating` | service receives the platform transaction | `Granted` when the platform transaction verifies server-side and the platform transaction ID is new; `Granted` (idempotent replay) when it is known; `Failed` otherwise |
| `Granted` | ledger and entitlement transaction committed, `grant` receipts signed | `Browsing` |
| `Failed` | verification failed | `Browsing` with an error code; no charge stands without a grant (the service issues a platform refund) |

---

## 8. Compliance layer

### 8.1 Region resolution

Inputs per session: `store_region` (platform account country), `billing_country` (platform wallet or last transaction), `ip_country` (GeoIP at login; absent when unknown). Each is an ISO 3166-1 alpha-2 code. The effective profile is

```
effective = merge_strictest([juris.region.default] ++ profiles(store_region) ++ profiles(billing_country) ++ profiles(ip_country))
```

`profiles(c)` returns the profiles whose `countries` list contains `c`, parent first:

| Profile | Countries |
|---|---|
| `juris.region.eu` | AT, BE, BG, CY, CZ, DE, DK, EE, ES, FI, FR, GR, HR, HU, IE, IT, LT, LU, LV, MT, NL, PL, PT, RO, SE, SI, SK (EU-27) and IS, LI, NO (EEA) |
| `juris.region.be` | BE (after `eu`) |
| `juris.region.nl` | NL (after `eu`) |
| `juris.region.uk` | GB |
| `juris.region.us` | US, PR, GU, VI, AS, MP |
| `juris.region.ca` | CA |
| `juris.region.au` | AU |
| `juris.region.br` | BR |
| `juris.region.kr` | KR |
| `juris.region.jp` | JP |
| `juris.region.cn` | CN (mainland; HK, MO and TW use `default`) |

Every other country uses `default` alone. `merge_strictest` applies each flag's strict direction from §8.2: `or` (true wins), `and` (false wins), `max` or `min`. Because every input can only tighten the result, a VPN or a foreign wallet never loosens any rule. The merge reruns at every login and every store open. Worked example: a Belgian account on a UK IP merges `default`, `eu`, `be` and `uk`: `child_data_consent_age = max(13, 16, 16, 13) = 16`, `withdrawal_waiver_consent = true`, `vat_inclusive_display = true`.

### 8.2 Flags

Flags are records in doc 02's flag file (doc 02 §17.3) with `kind` `bool` or `int`, `sim_affecting: false`, and one extra field `strict` (`or`, `and`, `max`, `min`). The economy service and the client read the same records.

```json
{ "id": "juris.flag.vat_inclusive_display", "kind": "bool", "default": false, "strict": "or",
  "channels": ["dev", "nightly", "playtest", "beta", "stable"], "sim_affecting": false }
```

| Flag | Kind | Default | Strict | Behaviour |
|---|---|---|---|---|
| `juris.paid_random_items_enabled` | bool | false | and | compile-time false; never flipped (SEP-5); not merged, because no profile may set it |
| `juris.flag.store_enabled` | bool | true | and | false hides the fiat store |
| `juris.flag.purchases_default_on` | bool | false | and | new accounts start with purchases off |
| `juris.flag.minor_age` | int | 18 | max | a platform age below this value marks the account minor (§8.4) |
| `juris.flag.vat_inclusive_display` | bool | false | or | prices shown tax-inclusive |
| `juris.flag.withdrawal_waiver_consent` | bool | false | or | §7.3 checkbox |
| `juris.flag.refund_window_days` | int | 14 | max | §7.4 |
| `juris.flag.minor_refund_window_days` | int | 90 | max | §7.4 |
| `juris.flag.odds_public_pages` | bool | true | or | §3.11.3, §5.6 |
| `juris.flag.draw_log_retention_days` | int | 1826 | max | §4.4 |
| `juris.flag.minor_chat_default` | int: 0 = `friends_only`, 1 = `quick_ping_only` | 0 | max | §8.5 |
| `juris.flag.child_data_consent_age` | int | 13 | max | below it, without verified guardian consent: offline-only profile, no chat, no analytics |
| `juris.flag.minor_playtime_window` | bool | false | or | minors play online only 20:00 to 21:00 local time on Fridays, Saturdays, Sundays and public holidays |
| `juris.flag.real_name_required` | bool | false | or | publisher real-name check before online play |
| `juris.flag.minor_store_block` | bool | false | or | fiat store hidden for minors |
| `juris.flag.minor_inkwell_block` | bool | false | or | Inkwell pulls and Reliquary opening hidden for minors; earning and banking continue, so nothing is lost if the block lifts |
| `juris.flag.no_random_trading` | bool | true | or | no transfer of any randomized reward (always true; SEP-7) |
| `juris.flag.dfa_watch` | bool | false | or | publication of the EU Digital Fairness Act text opens a compliance review due within 30 days |
| `juris.flag.kr_domestic_representative` | bool | false | or | KR representative designation is active and named on the odds pages |
| `juris.flag.fr_store_text` | bool | false | or | store, odds pages and purchase dialogs must be available in French |
| `juris.flag.invasion_minor_default` | int: 0 = off, 1 = on | 0 | min | invasions (Canon §17.9) start off for minor-flagged accounts, both opening their own runs (`mode.invasion_open`) and invading others; only a guardian enables them |

**Minor spend caps** are profile data, not flags, because they are per age band and per currency. Each profile may carry `minor_caps`, a list of `{ age_min, age_max, currency, tx_minor, month_minor, store_blocked }`, where `tx_minor` and `month_minor` are minor units or `null`. For a minor-flagged account of platform age `a` (age 0 when the platform reports a child account without an age), every merged entry with `age_min ≤ a ≤ age_max` applies: amounts convert into the account's store currency with the §7.2 anchor rule, the per-transaction cap is the minimum `tx_minor`, the monthly cap is the minimum `month_minor`, and `store_blocked` is the `or` of all entries. The guardian cap (§8.4) then applies beneath the regional caps.

```json
{ "id": "juris.region.jp", "countries": ["JP"], "parent": null,
  "flags": { "juris.flag.vat_inclusive_display": true },
  "minor_caps": [
    { "age_min": 0,  "age_max": 15, "currency": "JPY", "tx_minor": null, "month_minor": 5000,  "store_blocked": false },
    { "age_min": 16, "age_max": 17, "currency": "JPY", "tx_minor": null, "month_minor": 10000, "store_blocked": false } ] }
```

### 8.3 Region profiles

| Profile | Values that differ from `default` | Exact behaviour and basis (R3 C1 rows) |
|---|---|---|
| `juris.region.be` | none beyond `eu` | Paid loot boxes are gambling (row 1); none exist, so the same deterministic SKUs sell. |
| `juris.region.nl` | none beyond `eu` | Transferable random prizes are the trigger (row 2). `no_random_trading = true` everywhere. |
| `juris.region.br` | `vat_inclusive_display = true`; `minor_chat_default = 1`; `child_data_consent_age = 16` | ECA Digital Art. 20 bans paid loot boxes in games likely accessed by minors (row 12): none exist. Art. 21 limits interaction by default: minors start with quick pings only. The consent age of 16 is a design value. |
| `juris.region.au` | `vat_inclusive_display = true` | Chance-linked purchases (M) and simulated gambling (R18+) (row 11): neither exists. |
| `juris.region.uk` | `vat_inclusive_display = true`; `withdrawal_waiver_consent = true` | Industry principles: £0 default for child accounts and lenient refunds (row 3) are global defaults here (§8.4, §7.4). DMCC all-in pricing (row 3b): §7.3. |
| `juris.region.eu` | `vat_inclusive_display = true`; `withdrawal_waiver_consent = true`; `child_data_consent_age = 16`; `dfa_watch = true` | CPC principles (row 4): fiat prices only, no forced overspend, no pressure timers. DFA watch (row 5): `minor_store_block` and `minor_inkwell_block` are built and default false; the review decides whether to set them in `eu`. |
| `juris.region.us` | none | FTC v. Cognosphere (row 16): no loot box is sold, no currency exists, so its parental-consent, direct-purchase and exchange-rate duties have no object; odds are still disclosed (§3.11). COPPA (row 16b): `child_data_consent_age = 13` (the default). ESRB descriptor "In-Game Purchases" without "Includes Random Items". |
| `juris.region.kr` | `minor_age = 19`; `vat_inclusive_display = true`; `kr_domestic_representative = true` once either threshold of row 15 is met | Probability disclosure is mandatory only for paid items (row 15); PALIMPSEST applies it voluntarily at 4 decimals in-game, on the web and in every ad that shows the Inkwell. |
| `juris.region.jp` | `vat_inclusive_display = true`; `minor_caps`: JPY 5,000 per month under 16, JPY 10,000 per month at 16 and 17 | JOGA 2026 declaration (row 14): youth caps with guardian consent, no card-matching gacha (§5.6 set rule). |
| `juris.region.cn` | `vat_inclusive_display = true`; `minor_caps`: under 8 `store_blocked`; 8 to 15 CNY 50 per transaction and 200 per month; 16 and 17 CNY 100 and 400 | 2016 notice (row 13): odds pages and published results are global. `real_name_required` and `minor_playtime_window` are set true only in a licensed mainland edition build; no such edition ships at launch. |
| `juris.region.ca` | `fr_store_text = true` | ESRB as US; store, odds pages and purchase dialogs in English and French. |

Minor-cap example: a 15-year-old minor-flagged account with a CNY store currency, whose guardian has raised the guardian cap to CNY 200 (§8.4), merges `default` and `cn`: per-transaction cap CNY 50.00 (5,000 minor units), monthly cap CNY 200.00. A CNY 46 Folio Expansion passes the per-transaction cap, but with CNY 160 already spent this month it is refused with `minor_cap_month` (`160 + 46 = 206 > 200`) before platform checkout. The CNY set of the §7.1 example (components CNY 28, 18, 36 and 9: `round_down_ending(mul_bp(9100, 9000), CNY) = 8100`, CNY 81) is refused with `minor_cap_tx`.

### 8.4 Parental controls and purchase defaults

- Purchases start **off** for every account (`purchases_default_on = false`); the account holder turns them on once in Settings → Purchases after platform re-authentication.
- An account is **minor-flagged** when the platform reports a child or family-member account, or a platform age below `minor_age`. For a minor-flagged account: every purchase also needs guardian approval through the platform's family tools; the guardian monthly cap starts at 0 and only the guardian raises it; the regional `minor_caps` (§8.2) cap it from above; `minor_refund_window_days` applies.
- **Cap check,** at `Confirming` (§7.6), in minor units of the store currency: the purchase is refused with `minor_cap_tx` when `price > tx_cap`, with `minor_cap_month` when `spent_this_month + price > min(guardian_cap, month_cap)`, and with `minor_store_blocked` when `store_blocked` holds. `spent_this_month` sums the account's purchases since 00:00 UTC on the first day of the current UTC month, minus refunds.
- **Guardian settings** (4- to 8-digit PIN): purchases, monthly cap, chat level, invasions (default per `invasion_minor_default`), Inkwell and Reliquary visibility, a 60-minute session reminder (default off), Spend summary.
- Below `child_data_consent_age` without verified guardian consent, the client runs an offline-only profile (§4.5).

### 8.5 Chat safety

| Requirement | Rule |
|---|---|
| Controls on every channel | Text, voice, pings and emotes each offer per-player mute, block and report in at most 2 inputs from the player list and the scoreboard |
| Block | hides the blocked player's text, voice and emotes everywhere and prevents them joining the blocker's lobbies |
| Report | categories harassment, hate, sexual content, cheating, other; attaches the last 100 text lines held locally, session ID and party IDs; voice is never recorded |
| Filters | profanity and slur filter on by default for text and player names |
| Adult defaults | text on with filter; voice push-to-talk, party only; voice with strangers off (DECISIONS 75): a party member who is not on the player's platform friends list starts voice-muted for that player until the player unmutes them |
| Lobbies (DECISIONS 74) | friends join through platform invites; anyone joins a public lobby from doc 16's lobby browser. Every lobby gives each member mute, block and report on every other member, and a public lobby runs kick votes by doc 16 §9.2's rule: the starter's vote counts as yes, and with `m` Active members other than the target the vote passes on at least `max(2, m − 1)` yes votes within 1,800 ticks |
| Minor defaults | `friends_only`: free text and voice with friends only, quick chat with others; `quick_ping_only`: pings only |
| Invaders (Canon §17.9) | text and voice with an invader carry the same mute, block and report controls as every channel; voice with an invader is off by default for every account; a minor-flagged account gets pings and emotes only with an invader; blocking an invader from the scoreboard ends that invasion and bars that account from invading the blocker for 30 days |
| Enforcement | reports triaged within 72 hours; chat bans of 24 hours, then 7 days, then permanent |

### 8.6 Rating targets

PALIMPSEST contains strong fantasy and firearm violence, blood, body-part wounds and severing (Canon §9.1), cosmic and gothic horror, and mild language. By default wounds bleed ink and severed parts unravel into script; doc 17's `access.gore` setting switches to red blood (`ink`, the default; `full`, red blood; `minimal`; DECISIONS 94). The product owner fixed 16+ rating targets with severing and blood kept (DECISIONS 13), PEGI 16 and ESRB M (DECISIONS 94). Every board rates the red-blood setting, because the shipped game reaches it. Each board's target is the lowest band its published guidelines assign to severing with blood: boards with a 16 band use it; GRAC has no 16 band and assigns its adult band to dismemberment with blood; ESRB and CERO have 17+ bands. The monetization design adds no rating trigger: no paid random items, no paid time-limited offers, no daily-login rewards, no unblockable communication (R3 C3).

| Board | Target | Descriptors |
|---|---|---|
| PEGI | 16 | Violence, Fear, In-game Purchases |
| ESRB | M | Blood and Gore, Violence, Mild Language; In-Game Purchases; Users Interact |
| USK | 16 | Gewalt, Horror; In-Game-Käufe; Chats |
| ACB | MA15+ | strong violence, strong horror themes; in-game purchases |
| ClassInd | 16 | violência, medo |
| GRAC | 청소년이용불가 (18) | violence, fear |
| CERO | D | 暴力, 恐怖 |

No content is cut to chase a lower rating. When a board assigns a higher band than its target, the game ships at that band in that territory and nothing else changes. No `access.gore` value changes the rated content, which is the red-blood `full` setting.

---

## 9. Anti-predatory rules

These rules restate R3 D6 as binding requirements.

| ID | Rule |
|---|---|
| AP-1 | No premium currency, and therefore no odd-sized currency packs. |
| AP-2 | No paid random items; nothing purchasable converts into Ink, Quires, Vellum or Reliquaries (SEP-1 to SEP-4). |
| AP-3 | No expiring passes, countdown timers, limited stock or rotating shops (§6.1, §7.5). Featured banners show calendar end dates only and every featured unit returns through the Archive. |
| AP-4 | No daily login rewards, no streaks, no daily quests. Absence never removes progress; the catch-up reserve rewards return. |
| AP-5 | No zero-probability windows (`base_bp ≥ 1`), no personalized odds, no odds changes by A/B test. |
| AP-6 | No trading, gifting or cash-out of any account currency or randomized reward. No NFTs or blockchain. |
| AP-7 | No casino presentation: no reels, near-miss animations, wagering, payout multipliers or coin sounds. A pull reveal lasts at most 180 ticks and is skippable after 30; a ten-pull summary is skippable at once. |
| AP-8 | No challenge-only pass XP; all play earns Codex XP, except invading (Canon §17.9, DECISIONS 86: Quires and Red Ink marks only). |
| AP-9 | No offer prompts outside the store, no personalized offers, no "whale" segmentation. |
| AP-10 | No set-completion rewards for Inkwell entries or Keepsakes. |
| AP-11 | Every random system shows exact odds, pity counters and history and is verifiable (§3.11, §4). |
| AP-12 | The Inkwell never shows "Buy more Ink" or any path from money to pulls, because none exists. |

---

## 10. Economy server architecture

### 10.1 Components

- `econ-api`: Rust, `axum` 0.8.9, `tokio` 1.53.2; authenticated HTTPS API for pulls, opens, Archive, Codex, store and receipts.
- `econ-db`: Postgres via `sqlx` 0.9.0 at serializable isolation; ledger, entitlements, pity, epochs, audit log.
- `econ-verify`: headless `palimpsest_sim` re-simulating sampled run reports (§10.4).
- `econ-platform`: purchase verification and refund and chargeback webhooks for Steam, the only storefront (DECISIONS 112), through the Steam Inventory purchase flow (`StartPurchase`, `SteamInventoryResultReady_t`) with item definitions `tradable: false`, `marketable: false` (R4 §13).
- `econ-signer`: KMS-backed Ed25519 for receipts and daily audit roots.

### 10.2 Double-entry ledger

```sql
CREATE TABLE ledger_account (
  la_id       BIGINT PRIMARY KEY,
  owner       BIGINT NULL,               -- account_id, NULL for system accounts
  asset       TEXT   NOT NULL,           -- 'INK','QUIRE','VELLUM','RELIQUARY','QUILL'
  purchasable BOOLEAN NOT NULL,          -- true only for 'QUILL'
  kind        TEXT   NOT NULL            -- 'wallet','mint','burn'
);
CREATE TABLE journal (
  tx_id     UUID PRIMARY KEY,
  idem_key  BYTEA UNIQUE NOT NULL,       -- §10.3
  kind      TEXT NOT NULL,
  created   TIMESTAMPTZ NOT NULL
);
CREATE TABLE posting (
  tx_id  UUID REFERENCES journal,
  la_id  BIGINT REFERENCES ledger_account,
  amount BIGINT NOT NULL                 -- positive credit, negative debit
);
CREATE TABLE econ_debt (                 -- §10.4 clawback shortfall
  owner  BIGINT NOT NULL,
  asset  TEXT   NOT NULL,
  amount BIGINT NOT NULL CHECK (amount >= 0),
  PRIMARY KEY (owner, asset)
);
CREATE TABLE pending_grant (             -- §10.4 grants held while econ_review = true
  idem_key BYTEA PRIMARY KEY,            -- §10.3
  owner    BIGINT NOT NULL,
  postings JSONB  NOT NULL,              -- the computed postings, applied on clearance
  created  TIMESTAMPTZ NOT NULL
);
```

Posting invariants, checked by a deferred constraint trigger at commit:

1. For each `(tx_id, asset)`, `Σ amount = 0`.
2. A wallet balance never goes below 0.
3. All postings of one transaction share one `purchasable` value (SEP-2).
4. Mint accounts (database labels, not content IDs) exist per source: `mint/ink/play`, `mint/ink/achievement`, `mint/ink/codex`, `mint/ink/welcome`, `mint/quire/run`, `mint/quire/achievement`, `mint/quire/codex`, `mint/quire/invasion`, `mint/vellum/dupe`, `mint/vellum/spark`, `mint/reliquary/editor`, `mint/reliquary/folio`, `mint/reliquary/codex`, `mint/reliquary/coda`, `mint/quill/purchase`, `mint/quill/codex`. Burn accounts: `burn/ink/inkwell`, `burn/quire/bindery`, `burn/vellum/archive`, `burn/reliquary/open`, `burn/quill/redeem`, and `burn/*/clawback` for each asset (§10.4).
5. No transaction has postings in both a fiat-linked mint (`mint/quill/purchase`) and any `purchasable = false` asset.

Balances are `SUM(amount)` per wallet, cached in a balance table updated in the same transaction.

### 10.3 Idempotent grants

`idem_key = SHA-256(canonical_json({"a": account_id, "i": sub_index, "k": source_kind, "s": source_id}))`, with the §4.6 canonical JSON (sorted keys, no whitespace, `u64` as `"0x"` plus 16 lowercase hex digits).

| Source | `k` | `s` | `i` |
|---|---|---|---|
| run report (all grants of one report, one journal transaction) | `run` | `run_id` | 0 |
| pull or Reliquary open | `pull` | epoch ID | nonce |
| platform purchase | `platform` | `"<platform>:<platform_txn_id>"`, for example `"steam:5234112"` | 0 |
| Codex tier reward | `codex` | `pass.codex_<nnn>` | tier, 1 to 100 |
| Codex Coda Reliquary | `coda` | `pass.codex_<nnn>` | Coda Reliquary ordinal, from 1 |
| achievement | `ach` | the `ach.` ID or the Codex mark's ID | 0 |
| spark end conversion | `spark_end` | the banner ID | 0 |
| Archive purchase | `archive` | the request's `req_id` as `"0x"` plus 16 lowercase hex digits (doc 16 §11.1) | 0 |
| Transcription claim | `transcribe` | the request's `req_id` in the same hex form | 0 |

Example: `{"a":"0x0123456789abcdef","i":0,"k":"run","s":"0x00000000075bcd15"}` hashes to `909de25b25a2d70ca98a91f48c4098ee338ccb3dd518356c1d5db5519b06a4b6`. A request whose key exists returns the stored result and posts nothing. Clients retry with the same request body until they receive a response.

### 10.4 Run reports

**`run_id`** is a `u64` the host draws from its OS CSPRNG at run start, stores in `RunState` (doc 14, Interface assumptions) and sends to every joiner. It never feeds simulation randomness.

At `ev.run_ended` (and at a `run.end.departed` for that slot), each player's client submits one run report as doc 16's run-report message; Canon §18 has no account events, so this report is the whole contract between a run and the account economy. Example (a solo Standard death in Folio 4; hashes are illustrative):

```json
{
  "v": 1,
  "run_id": "0x00000000075bcd15",
  "acct": "0x0123456789abcdef",
  "host_acct": "0x0123456789abcdef",
  "slot": 0,
  "sim_version": 1,
  "content_hash": "92c159a0a347070d679d6edd626a8bc6938565f60204dd732ba14c2ac3592cc9",
  "content_set": ["unlock.calling.alchemist", "unlock.loot.throwables_2"],
  "mod_count": 0,
  "run_seed": "0x0123456789abcdef",
  "mode": "mode.standard",
  "easing": 0,
  "final_folio": 25,
  "end": "run.end.erased",
  "started_utc": 1821052800,
  "ended_utc": 1821054000,
  "active_ticks": 54000,
  "folio_exits": [
    { "folio": 1, "genre": "genre.cyberpunk", "entry_tick": 0, "exit_tick": 16200, "present": 1 },
    { "folio": 2, "genre": "genre.weird_west", "entry_tick": 16200, "exit_tick": 34200, "present": 1 },
    { "folio": 3, "genre": "genre.steampunk", "entry_tick": 34200, "exit_tick": 50400, "present": 1 }
  ],
  "editor_kills_present": [],
  "victory_present": 0,
  "ironman": 0,
  "kills_present": [ { "party_size": 1, "k_sum": 600 } ],
  "types_identified": 5,
  "crafts": 2,
  "revives": 0,
  "codex_entries_new": [],
  "satchel_materialized": [],
  "run_score": 710,
  "quires_paid": 0,
  "input_log_blake3": "a4cd5df9faa66d265baf53d652ff672a1e3b76f2466877f8eaa64d45ea905817",
  "state_hash": "51d121c58a6957cd0ec9d418ee68c1a00fc163416ec79029b098dd2b296d7ce8"
}
```

`k_sum` is §6.2's `K[n]`. `run_score` is doc 14 §4.6's value for this slot. `victory_present` follows §2.2 (a forfeited Second Edition reports 0), and `ironman` is doc 14's `RunSummary.ironman` as 0 or 1. Grants for this report: Ink `3 × 15 + 10 × 3 + 20 = 95`; Codex XP `15 × 150 + 3 × 400 + floor_div(10000 × 600, 10000) + 800 + 5 × 25 + 2 × 25 = 2,250 + 1,200 + 600 + 800 + 125 + 50 = 5,025`; Quires `floor_div(710, 50) = 14`; Reliquaries 0; the lifetime Folio-exit counter rises by 3.

**Acceptance procedure,** in order; the first failure rejects the report with the named code and grants nothing:

1. `report_malformed`: schema validation (integers only, hex formats, sorted `content_set`).
2. `sim_version_unknown`: `sim_version` is a released version.
3. Apocrypha: if `mod_count > 0`, the report is accepted with zero grants (§2.2) and the remaining checks are skipped.
4. `content_mismatch`: every ID in `content_set` was owned by `host_acct` at `started_utc`, and `content_hash` equals doc 02 §11.2's hash recomputed from that release's base records outside the presentation namespaces (Canon §2.4), `content_set`, an empty `MOD` section and that release's sim-affecting flags.
5. `host_mismatch`: in co-op, `run_seed`, `content_hash`, `mode` and the final `state_hash` equal the host's report for the same `run_id`; a non-host report waits up to 7 days for the host's report and is then accepted on its own.
6. Grants are computed by §2.2, §2.4, §5.1, §6.2 and the §2.3 caps, and posted in one journal transaction keyed per §10.3.

**Verification sampling.** `econ-verify` replays the report's input log headlessly when `mix64(run_id) mod 100 < 5` (Canon §3.1 `mix64` on the `u64`), when the report reaches 90% of any cap, or when the account has `econ_review = true`. Clients keep the replay log (doc 02 container kind 3) of every reported run for 30 days and upload it within 7 days of a request; a missing upload voids the report. A replay whose `state_hash`, `active_ticks` or any counted field differs from the report is a **mismatch**:

- the report's grants are reversed with `burn/<asset>/clawback` postings that debit each wallet down to 0 at most; any shortfall is stored as `econ_debt[asset]` and subtracted from that asset's future grants before they reach the wallet;
- every unverified report of that account from the previous 30 days is queued for replay, with the same rules;
- `econ_review = true` holds every new grant in a pending table until support clears the account; pending grants post on clearance or are discarded on a confirmed second mismatch.

### 10.5 Receipt validation, refunds and chargebacks

- Every platform transaction is verified server-to-server before any grant; client-supplied purchase claims are never trusted.
- **Refund** (platform webhook or §7.4 request): revoke the SKU's grant bundle, post `revoke` receipts, burn the Quill if the refunded Codex granted one that is still held. A Quill already redeemed revokes the premium track it was redeemed on.
- **Chargeback**: same revocations, plus `chargeback_hold = true`, which disables purchases until support clears it. Earned content is never touched.

---

## 11. End-of-service mode

### 11.1 Sunset timeline

| Phase | Starts | Effects |
|---|---|---|
| `Live` | launch | normal operation |
| `SunsetAnnounced` | announcement, at least 180 days before shutdown | Codex and cosmetic sales stop at once; purchases from the prior 30 days are refundable on request; Folio Expansions stay on sale on the platform storefront because they keep working offline; banners, Archive and free tracks continue |
| `FinalSync` | 30 days before shutdown | every client prompts a full sync; the service issues an `eos_final` receipt per account: all entitlements, balances, Folds, pity and guarantee flags of every family, spark, Standard spark, Reliquary counters, the lifetime Folio-exit counter, `transcription`, unopened Reliquaries with their source genres, the Keepsake vault, Codex progress, catch-up reserve and Coda XP |
| `Shutdown` | shutdown day | every unrevealed `server_seed` is revealed and published; the full audit log for the last 5 years and every daily root are published as a download; servers stop |
| `EndOfService` | permanent | the client runs §11.3 forever |

The final client patch, shipped during `FinalSync`, embeds the `eos_final` public key and enters end-of-service mode once the service is unreachable after the published shutdown date, or when the player chooses "Go offline permanently".

### 11.2 What stays

Owned Echoes, Folds, Scripts, cosmetics, Keepsakes, balances, pity and Codex progress load from the newest valid `eos_final` or later local state. Every gameplay system runs without the backend; co-op uses LAN, direct IP and platform P2P.

### 11.3 Local economy rules

- Ink, Quires, Codex XP and Reliquaries are earned locally at §2 and §6 rates, without the rolling 24-hour caps.
- The Inkwell offers a **Retrospective** Featured banner: the player chooses any Illuminated Echo or Script ever featured (expansion units only with platform ownership of that expansion) as the featured unit; the rate-ups are those of the banner that first featured it, and the banner belongs to that unit's family (`echo`, `script` or `exp<nn>`). The choice can change only between periods of the local calendar, which continues `T0 + 28 × (k − 1)` days. Hazards, guarantees, spark and pity carry over unchanged; the Standard and Beginner banners continue locally as before. Draws use §4.2 with a `server_seed` from the local OS RNG, committed before use, and `config_hash` of the local banner record; the player can verify locally.
- The Vellum Archive stocks every entry ever released, and the Transcription shelf (§3.9.1) continues locally at the same costs.
- Every Codex premium track ever released unlocks for every account, because it can no longer be bought.
- The store screen is removed. Platform-owned expansions keep working through platform ownership.

---

## 12. KPIs and ethical guardrails

KPI IDs belong to doc 01 (`kpi.`), whose §10.3 lists the `kpi.econ.*` rows below with their targets and alert bands; this chapter defines their meaning and the action when one is breached (doc 01 §10.2 defers `kpi.econ.*` breach actions here).

| KPI | Doc 01 ID | Definition | Target or guardrail | Action when breached |
|---|---|---|---|---|
| Featured hours, median | `kpi.econ.featured_hours_p50` | median reference hours from banner start to a player's first featured Illuminated, among players who pull on that banner | ≤ 25 | raise Ink rates in the next update |
| Ink per active hour, p50 | `kpi.econ.ink_per_hour_p50` | Ink from play ÷ active hours, last 28 days | ≥ 350 | audit run-report grants |
| Quires per run hour, p50 | `kpi.econ.quires_per_hour_p50` | Quires from runs per active run hour, median over accounts | 50 to 160 | re-check the §2.4 rate against doc 14 §4.6's `run_score` calibration |
| Codex completion | `kpi.econ.codex_completion` | share of premium owners reaching tier 100 within 24 weeks of purchase | ≥ 60% (6000 Bp) | lower `CODEX_XP_PER_TIER` for future Codexes only |
| Refund rate | `kpi.econ.refund_rate` | refunded ÷ purchased SKUs, 28 days | ≤ 3%; alert at 5% | review store copy and previews |
| Unauthorized-minor refunds | `kpi.econ.minor_refunds` | count per 28 days | alert on any period-over-period rise | review guardian flow |
| Spend concentration | `kpi.econ.spend_top1_share` | share of revenue from the top 1% of spenders | informational; no target | none (a finite catalog bounds spend) |
| Spend wellbeing check | none (a client rule, not a KPI) | any account above USD 300 in 30 days | the client shows the Spend summary once, with no offer | none |
| Chat report rate | `kpi.econ.chat_report_rate` | chat and voice reports per 1,000 sessions | alert on any rise over the previous window | moderation staffing |

**Forbidden practices.** No KPI target on daily streaks, sessions per day, or conversion of players under 18. No A/B test changes odds, pity, prices, Ink rates or Codex XP; A/B tests cover layout and copy only. No behavioural model scores players for monetization. Inkwell, Reliquary and Codex tuning uses only §2.8 hours and §6.7 completion, never revenue.

---

## Exports

Every ID below is in a namespace Canon §19 assigns to this chapter, passes the Canon §1.1 grammar and is at most 64 bytes.

| Namespace | IDs |
|---|---|
| `banner.` | `banner.standard`, `banner.beginner`, `banner.archive`, `banner.reliquary.account`; the first launch rotation cycle (§3.2.2) `banner.echo.p001`, `banner.echo.p002`, `banner.echo.p003`, `banner.echo.p004`, `banner.echo.p005`, `banner.echo.p006`, `banner.echo.p007`, `banner.echo.p008`, `banner.echo.p009`, `banner.echo.p010`, `banner.echo.p011`, `banner.echo.p012` and `banner.script.p001`, `banner.script.p002`, `banner.script.p003`, `banner.script.p004`, `banner.script.p005`, `banner.script.p006`, `banner.script.p007`, `banner.script.p008`, `banner.script.p009`, `banner.script.p010`, `banner.script.p011`, `banner.script.p012`, then `banner.echo.p<nnn>` and `banner.script.p<nnn>` for every later period; `banner.exp<nn>.p<nnn>` for Folio Expansion `nn` (§3.2.6); `banner.tier.illuminated`, `banner.tier.rubricated`, `banner.tier.plain`; `banner.family.echo`, `banner.family.script`, `banner.family.standard`, `banner.family.beginner`, `banner.family.exp01`, `banner.family.exp02`, and `banner.family.exp<nn>` for later expansions; `banner.hazard.illuminated_std`, `banner.hazard.rubricated_std`, `banner.hazard.illuminated_beginner`; `banner.pool.illuminated_echo`, `banner.pool.illuminated_script`, `banner.pool.rubricated`, `banner.pool.plain` |
| `pass.` | `pass.codex_<nnn>`, starting with `pass.codex_001` (§6.8); `pass.xp.active_minute`, `pass.xp.folio_exit`, `pass.xp.editor_kill`, `pass.xp.kill`, `pass.xp.run_end`, `pass.xp.run_victory`, `pass.xp.type_identified`, `pass.xp.craft`, `pass.xp.revive`, `pass.xp.codex_entry`, `pass.xp.achievement` |
| `store.` | `store.sku.base_game`, `store.sku.folio_exp_01` (Age of Sail and Pirates), `store.sku.folio_exp_02` (Kaiju and Mecha), `store.sku.folio_exp_<nn>` for later expansions, `store.sku.codex_<nnn>_premium` (first `store.sku.codex_001_premium`), `store.sku.supporter_pack`, `store.sku.cosm_<name>`, `store.sku.set_<name>`, `store.price_matrix` |
| `bundle.` | the 36 launch Scripts of §3.1.2 in index order: Illuminated `bundle.script.lost_chapter_cosmic_horror`, `bundle.script.lost_chapter_cyberpunk`, `bundle.script.lost_chapter_fairy_tale`, `bundle.script.lost_chapter_gothic_horror`, `bundle.script.lost_chapter_hardboiled_noir`, `bundle.script.lost_chapter_mythic_antiquity`, `bundle.script.lost_chapter_mythic_fantasy`, `bundle.script.lost_chapter_post_apocalypse`, `bundle.script.lost_chapter_space_opera`, `bundle.script.lost_chapter_steampunk`, `bundle.script.lost_chapter_weird_west`, `bundle.script.lost_chapter_wuxia`; Rubricated `bundle.script.glyphs_ember`, `bundle.script.glyphs_hollow`, `bundle.script.glyphs_mirror`, `bundle.script.glyphs_storm`, `bundle.script.glyphs_thorn`, `bundle.script.glyphs_tide`, `bundle.script.hireling_apothecary`, `bundle.script.hireling_cartographer`, `bundle.script.hireling_lamplighter`, `bundle.script.hireling_porter`, `bundle.script.hireling_sellsword`, `bundle.script.kit_alchemist`, `bundle.script.kit_artificer`, `bundle.script.kit_beastbinder`, `bundle.script.kit_duelist`, `bundle.script.kit_grenadier`, `bundle.script.kit_gunslinger`, `bundle.script.kit_monk`, `bundle.script.kit_netrunner`, `bundle.script.kit_oracle`, `bundle.script.kit_shade`, `bundle.script.kit_spellwright`, `bundle.script.kit_vanguard`, `bundle.script.kit_warden`; 3 more per Folio Expansion (§3.1.2); `bundle.sku.<name>` (SKU grant lists), for example `bundle.sku.supporter_pack` |
| `cosm.` | categories `cosm.cat.outfit`, `cosm.cat.palette`, `cosm.cat.ink`, `cosm.cat.vfx`, `cosm.cat.emote`, `cosm.cat.sigil`, `cosm.cat.title`, `cosm.cat.frame`, `cosm.cat.margin`, `cosm.cat.trail`, `cosm.cat.binding`, `cosm.cat.recollected`, `cosm.cat.redink`; the record's `cat` field carries the category. Item patterns: `cosm.<category>.<name>` for catalog items; `cosm.echo.<echo_suffix>.<part>` with parts `palette`, `outfit`, `frame`, `title`, `trail` (Folds, §3.8) and `recollected` (owned-Echo Recollection, doc 14 §6.1). The 60 Plain cosmetics in pool order (§3.4): `cosm.emote.inkwell_01`, `cosm.emote.inkwell_02`, `cosm.emote.inkwell_03`, `cosm.emote.inkwell_04`, `cosm.emote.inkwell_05`, `cosm.emote.inkwell_06`, `cosm.emote.inkwell_07`, `cosm.emote.inkwell_08`, `cosm.emote.inkwell_09`, `cosm.emote.inkwell_10`, `cosm.ink.inkwell_01`, `cosm.ink.inkwell_02`, `cosm.ink.inkwell_03`, `cosm.ink.inkwell_04`, `cosm.ink.inkwell_05`, `cosm.ink.inkwell_06`, `cosm.ink.inkwell_07`, `cosm.ink.inkwell_08`, `cosm.ink.inkwell_09`, `cosm.ink.inkwell_10`, `cosm.ink.inkwell_11`, `cosm.ink.inkwell_12`, `cosm.ink.inkwell_13`, `cosm.ink.inkwell_14`, `cosm.ink.inkwell_15`, `cosm.ink.inkwell_16`, `cosm.ink.inkwell_17`, `cosm.ink.inkwell_18`, `cosm.ink.inkwell_19`, `cosm.ink.inkwell_20`, `cosm.sigil.inkwell_01`, `cosm.sigil.inkwell_02`, `cosm.sigil.inkwell_03`, `cosm.sigil.inkwell_04`, `cosm.sigil.inkwell_05`, `cosm.sigil.inkwell_06`, `cosm.sigil.inkwell_07`, `cosm.sigil.inkwell_08`, `cosm.sigil.inkwell_09`, `cosm.sigil.inkwell_10`, `cosm.sigil.inkwell_11`, `cosm.sigil.inkwell_12`, `cosm.sigil.inkwell_13`, `cosm.sigil.inkwell_14`, `cosm.sigil.inkwell_15`, `cosm.title.inkwell_01`, `cosm.title.inkwell_02`, `cosm.title.inkwell_03`, `cosm.title.inkwell_04`, `cosm.title.inkwell_05`, `cosm.title.inkwell_06`, `cosm.title.inkwell_07`, `cosm.title.inkwell_08`, `cosm.title.inkwell_09`, `cosm.title.inkwell_10`, `cosm.title.inkwell_11`, `cosm.title.inkwell_12`, `cosm.title.inkwell_13`, `cosm.title.inkwell_14`, `cosm.title.inkwell_15`. Red Ink marks (§2.4): `cosm.redink.mark_001`, `cosm.redink.mark_005`, `cosm.redink.mark_025`, `cosm.redink.mark_100`. Second Edition Ironman cosmetic (§2.2, DECISIONS 108): `cosm.frame.ironman` (category `cosm.cat.frame`). Bindery finishes named by doc 14 §5.3: `cosm.binding.vellum`, `cosm.binding.lapis`, `cosm.binding.verdigris`, `cosm.binding.oxblood`, `cosm.binding.gilt`, `cosm.binding.soot` (category `cosm.cat.binding`) and `cosm.palette.mythic_fantasy`, `cosm.palette.gothic_horror`, `cosm.palette.cosmic_horror`, `cosm.palette.space_opera`, `cosm.palette.cyberpunk`, `cosm.palette.post_apocalypse`, `cosm.palette.weird_west`, `cosm.palette.hardboiled_noir`, `cosm.palette.wuxia`, `cosm.palette.steampunk`, `cosm.palette.fairy_tale`, `cosm.palette.mythic_antiquity` (category `cosm.cat.palette`). Codex examples `cosm.outfit.codex_001_t005`, `cosm.emote.codex_001_bow`, `cosm.sigil.codex_001_quill_rampant` |
| `juris.` | `juris.paid_random_items_enabled` (Canon §22); the 20 flags of §8.2: `juris.flag.store_enabled`, `juris.flag.purchases_default_on`, `juris.flag.minor_age`, `juris.flag.vat_inclusive_display`, `juris.flag.withdrawal_waiver_consent`, `juris.flag.refund_window_days`, `juris.flag.minor_refund_window_days`, `juris.flag.odds_public_pages`, `juris.flag.draw_log_retention_days`, `juris.flag.minor_chat_default`, `juris.flag.child_data_consent_age`, `juris.flag.minor_playtime_window`, `juris.flag.real_name_required`, `juris.flag.minor_store_block`, `juris.flag.minor_inkwell_block`, `juris.flag.no_random_trading`, `juris.flag.dfa_watch`, `juris.flag.kr_domestic_representative`, `juris.flag.fr_store_text`, `juris.flag.invasion_minor_default`; `juris.region.default`, `juris.region.be`, `juris.region.nl`, `juris.region.br`, `juris.region.au`, `juris.region.uk`, `juris.region.eu`, `juris.region.us`, `juris.region.kr`, `juris.region.jp`, `juris.region.cn`, `juris.region.ca` |

Constants (not content IDs): `INK_PULL_COST = 160`, `INK_PULL_COST_BEGINNER = 120`, `REF_INK_PER_HOUR = 400`, `WELCOME_INK = 1200`, `VICTORY_INK_PER_FOLIO = 12`, `INK_PER_RUN_CAP = 2500`, `INK_PLAY_ROLLING_24H = 4000`, `QUIRES_PER_RUN_CAP = 400`, `INVASION_QUIRES_ROLLING_24H = 60`, `INVASION_QUIRES = 5`, `INVASION_REDACT_QUIRES = 5`, `INVASION_MIN_TICKS = 3600`, `INK_ACTIVE_MINUTES_PER_RUN = 240`, `CODEX_XP_PER_TIER = 12000`, `REF_CODEX_XP_PER_HOUR = 20000`, `VICTORY_XP_PER_FOLIO = 160`, `CODEX_XP_PLAY_ROLLING_24H = 200000`, `CATCHUP_WEEKLY = 40000`, `CATCHUP_CAP = 160000`, `CODA_XP_PER_RELIQUARY = 24000`, `SPARK_CLAIM = 100`, `BANNER_PERIOD_DAYS = 28`, `KEEPSAKE_VAULT_CAP = 30`, `SATCHEL_SIZE = 5`, `SATCHEL_MAX_LEGENDARY_PLUS = 1`, `RELIQUARY_FOLIO_EXITS = 30`, `FOLIO_DWELL_MIN_TICKS = 5400`, `ACTIVE_WINDOW_TICKS = 3600`, `AUDIT_RETENTION_DAYS = 1826`, `REPLAY_KEEP_DAYS = 30`, `VERIFY_SAMPLE_PERCENT = 5`, `TRANSCRIBE_RUBRICATED = 16`, `TRANSCRIBE_ILLUMINATED = 32`. Configuration: `T0` (§3.2.3), set at launch; the examples use 1820448000.

Error codes: `insufficient_ink`, `insufficient_transcription`, `banner_ended`, `requires_sku`, `beginner_target_required`, `beginner_closed`, `epoch_mismatch`, `minor_cap_tx`, `minor_cap_month`, `minor_store_blocked`, `report_malformed`, `sim_version_unknown`, `content_mismatch`, `host_mismatch`.

## Interface assumptions

Only open items are listed; everything else this chapter reads from other documents is on disk and cited under Imports.

| Doc | Assumed to provide |
|---|---|
| 14 | §2.3 Early Access scope per DECISIONS 114: all 12 core genres, so no launch Echo, Script, Origin node or palette is absent at Early Access (it still cites the 6 genres of DECISIONS 41) |
| 14 | Requested: §10 cut to about 40 curated Steam achievements with the long tail as Codex marks keeping grades 1 to 4 (DECISIONS 141); the first-run achievement for finishing Folio 1 of the account's first run in place of the Prologue's (DECISIONS 137); §8.1 and §11 without the Prologue mode (DECISIONS 132). Present: `RunState.run_id`, `satchels`, `SlotState.active_ticks`, `last_input_tick`, the `RunSummary` fields of §10.4, `FolioRecord.genre` and `fused`, and the Second Edition Editor kills in `editor_kills_present` (§8.9) |
| 13 | Present: the six Second Edition bosses carry `rank.editor` (§9.4), so §2.2's Editor-kill row pays each |
| 11 | For a Keepsake, §2.3 step 2's eligible templates come from the opener's account content set at open time (§5.4) |
| 02 | Present: `palimpsest-verify` as a `palimpsest_tools` binary linking `palimpsest_econ` (§3.2) with the pull, open and Transcription algorithms; `pal odds` computing each build's odds records from that build's pools (§3.2.7); clients keeping the replay log of every reported run for 30 days (§10.4) |
| 16 | The run-report message carrying every §10.4 field, `ironman` included (doc 14 §8.9) |
| 17 | The Fairness panel (fresh and pinned client-seed modes, "Reveal now", §4.1); the Vellum Archive screen with its Transcription shelf (counter, costs 16 and 32, refusal `insufficient_transcription`, §3.9.1) |
| 18 | Display names and `loc.` keys for every `cosm.`, `banner.`, `pass.`, `store.` and `bundle.` ID, including the 60 Plain cosmetics and `cosm.frame.ironman` |
| 21 | The release plan of DECISIONS 111 to 114 and 122: no hard dates (`T0` is set at launch, §3.2.3), Steam only, no demo or public playtest build, all 12 core genres at Early Access (§3.2.7), and Folio Expansions at USD 9.99 |

## Canon gaps

1. **Keepsake `illuminated` roll.** Canon §13.9 defines `illuminated` as "one `rng.loot` roll", but Canon §3.6 seeds Keepsake generation from `Rng::from_seed64` outside the simulation, and §5.4 here and doc 11 §2.3 draw a Keepsake's roll from that generator, never from `rng.loot`. Proposed §13.9 text: after "Engine faces and rewards (doc 11 §2.3)" add "; for a Keepsake, one roll of its Keepsake generator (§3.6, doc 15 §5.4)".

The 16 gaps of the previous revision are closed by canon-1.11 and the canon gap triage (G-031, G-149 to G-154, G-160, G-174, G-179, G-186, G-188 to G-190, G-227); `audit/fix_15.md` maps each one to the section that applies it.

## Acceptance tests

1. **Hazards.** `illuminated_std.p_bp(n)` for n = 1, 45, 46, 52, 59, 60 returns 100, 100, 750, 4650, 9200, 10000. `rubricated_std` for 8, 9, 10 returns 600, 6600, 10000. `illuminated_beginner` for 20, 21, 29, 30 returns 100, 1100, 9100, 10000.
2. **Hazard validator.** A hazard with `base_bp = 0`, or with `p_bp(hard) < 10000`, or with `p_bp` decreasing anywhere, is rejected.
3. **Exact tier statistics.** Exact rational evaluation gives `E[T_top]` = 39.2303, consolidated Illuminated 2.5491%, P(T_top ≤ 45) = 36.3815%, consolidated Rubricated 12.5752%, Rubricated-or-better 15.1242%, Plain 84.8758%, and every cumulative value of §3.10.2.
4. **Featured statistics.** `E[T_feat]` = 49.0379 without spark and 48.9555 with spark; P(T_feat > 100) = 3.0279%; median 49, 90th percentile 94; maximum 120 without spark and 100 with spark; every value of §3.10.3 and §3.10.4.
5. **Monte Carlo cross-check.** 10,000,000 simulated fresh accounts through §3.4 give mean pulls to featured within 0.05 of 49.0379.
6. **Display.** `pct4(1, 4400)` = 227, shown `0.0227%`. The Featured Echo banner rows equal §3.11.2, and the expansion banner rows equal §3.2.6 (0.0192% and 0.0392% for each off-featured entry, 0.0833% and 0.1164% for each off-rate entry).
7. **ChaCha20.** The in-house block function reproduces the RFC 8439 §2.3.2 block beginning `10f1e7e4d13b5915500fdd1fa32071c4`.
8. **Inkwell vector.** `server_seed` = bytes `00..1f`; commitment `630dcd2966c4336691125448bbb25b4ff412a49c732db2c8abc1b8581bd710dd`; `config_hash = SHA-256("PALIMPSEST test banner config")` = `51d121c58a6957cd0ec9d418ee68c1a00fc163416ec79029b098dd2b296d7ce8`; banner `banner.echo.p001` (`fnv1a32` = `0xc450327f`), period 1, client seed ASCII `errata`. Nonce 0: `msg` is 77 bytes; `pull_key` = `385ee0bc919e734ccfe7c404f6844566d95b9ecf511e2c0fc6615dc3715f4adf`; first `u64` = `0x3945acb817ca9698`; `u_tier` = 344, `u_feat` = 3861. From a fresh Echo family this is a rate-up Rubricated result with index `uniform(3)` = 1, `echo.lan_ninth_peak`. Nonce 1: `pull_key` = `3990b490ecde62d3f1fdc0f41024c7ac53ce99a791957fe8cf7e0777f518720f`, `u_tier` = 5654, Plain, index 5, `cosm.emote.inkwell_06`. Nonce 2: `u_tier` = 7578, Plain, index 24, `cosm.ink.inkwell_15`.
9. **Reliquary vector.** Same seed, config hash and client seed; domain `PALIMPSEST/RELIQUARY/v1`; banner `banner.reliquary.account`, period 1, nonce 0: `msg` is 79 bytes; `pull_key` = `d79dad425520b04844bdbd0e0a91a6b63de6788993755f3a041664180da5237a`; draw 1 = 4774 (Uncommon). With counters (4, 11), draw 2 = `uniform(600)` = 87 (Epic); `item_seed` = `0x80ce3324d7120951`; counters become (0, 12).
10. **Receipt.** The §4.6 receipt (`iat` 1821052800, `kid` `rk_2027a`) signed with RFC 8032 test key 1 yields the signature in §4.6 and verifies against public key `d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a`. Changing `qty` to 2 fails verification.
11. **Ink.** The reference failed run grants 400 Ink; the §2.2 victory run grants 1,115. A Daily victory with 25 active minutes, 4 exits and 1 Editor grants `75 + 40 + 40 + 20 = 175` (run end, no victory bonus); an Abridged victory with 67 active minutes, 14 exits and 5 Editors grants `201 + 140 + 200 + 180 = 721`; a Novella victory pays the 20-Ink run end. A run computing 2,800 grants 2,500. With 3,900 play Ink in the last 24 hours, a 400-Ink run grants 100.
12. **Mode rules.** An easing Custom run with 30 active minutes, 6 exits and 1 Editor grants 90 Ink, 4,500 Codex XP, 0 Reliquaries and 0 Quires. An account's first run is a `mode.standard` run under mode row 1; when its player exits Folio 1 the run report also unlocks doc 14's first-run achievement (DECISIONS 137; grade 1: 40 Ink, 10 Quires, 2,000 Codex XP outside every cap). No mode ID other than the §2.2 table's is accepted, and a report naming the retired Prologue mode is refused with `report_malformed`. A run with `mod_count = 1` grants 0 of everything; a run with only a presentation-only mod loaded reports `mod_count = 0` and earns as its mode row says. An invader who Redacts a party after 200 s gets 10 Quires, 0 Ink and 0 Codex XP; one who leaves after 30 s without a Redaction gets 0; with 55 invader Quires already in the 24-hour window, a 10-Quire invasion grants 5.
13. **Quires.** `run_score` 3,033 grants 60 Quires; 14,991 grants 299; victory C's 20,000 (doc 14's per-Folio cap on a raw 28,641) grants 400; a departed player with `quires_paid = 100` and a final `run_score` of 14,991 receives 199 more; an Apocrypha run grants 0. Achievements of grades 1, 2, 3 and 4 pay 10, 25, 50 and 100 Quires and 40, 80, 160 and 320 Ink.
14. **Codex XP.** The reference hour sums to 20,000; the §6.2 victory run to 43,900. A single standard kill in a 4-player party gives each player 2; 70 standard kills in a 4-player Folio give each player 200, equal to 40 solo kills. `tier(1,199,999)` = 99, `tier(1,200,000)` = 100. A Daily victory grants the 800 run-end XP; an Abridged victory grants 2,400 victory XP.
15. **Catch-up.** Reserve 40,000 and a 1,000 XP grant: the Codex gains 2,000 and the reserve becomes 39,000. With 199,500 base play XP already in the rolling window and a reserve of 40,000, a 1,000 XP grant is clipped to 500 and then doubled to 1,000, leaving the reserve at 39,500. Pacing: 5 hours per week reaches tier 100 after 42 hours; 3 hours per week in exactly 12 weeks; every row of §6.7 matches a week-by-week simulation.
16. **Folds and Vellum.** An Illuminated Echo at Fold 5 plus a duplicate gives Fold 6 and 10 Vellum; a further duplicate gives 40 Vellum. A Plain duplicate gives 1.
17. **Spark.** A Featured banner reaching `Ended` with 49 spark mints exactly 49 Vellum and deletes the counter. A claim at 100 spark grants the featured unit and leaves pity unchanged. Expected pulls for 2, 3 and 7 copies with spark claimed at every 100 are 85.906, 109.832 and 245.348.
18. **Pity isolation.** 30 pulls on a Featured Script banner leave the Echo family's `PityState` byte-identical. Echo-family counters carry from `banner.echo.p001` to `banner.echo.p002` unchanged. Pulls on any banner of family `banner.family.exp01` leave the Echo family untouched.
19. **Banner end.** A pull received at `end_utc` is rejected with `banner_ended` and its Ink is not debited. A Sealed Draft for an ended banner is cancelled and its Ink restored.
20. **Standard banner.** A specific Standard Illuminated entry costs 941.527 expected pulls without spark and 96.375 with the spark selector; a specific Rubricated entry costs 284.946.
21. **Reliquary rates.** Exact Markov evaluation over the 90 reachable states gives Epic-or-better 23.2266% and Legendary-or-better 7.8074%; from fresh counters the first Epic-or-better takes 4.435 openings and the first Legendary-or-better 12.808, and P(first Legendary-or-better by the 10th) = 45.0242%. With counters (4, 4), P(Epic) = 75%, P(Legendary) = 20%, P(Exotic) = 5%. With `c20 = 19`, P(Legendary or Exotic) = 100%.
22. **Satchel.** A Standard-mode run with a 3-Keepsake Satchel materializes them on entering Folios 1, 6 and 11 with `ilvl` 1, 6 and 11 and `finder_slot` = the owner's slot; dying in Folio 8 returns the third Keepsake to the vault. A Novella run with 5 Keepsakes materializes 3. An Abridged run materializes at Folios 1, 4, 7, 10 and 13 with `ilvl` 1, 6, 11, 16 and 21. A Daily run materializes none. A Satchel with two Legendary Keepsakes is refused. A Keepsake whose template is outside the host's content set cannot be placed. A Dormant slot at Folio 6 receives its second Keepsake at Folio 11.
23. **Separation.** A ledger transaction posting `QUILL` and `INK` together is rejected. A `pass.` record with `cur.ink` on the premium track, and a `bundle.sku.*` granting `cur.ink`, fail validation. A configuration with `juris.paid_random_items_enabled = true` prevents service start. A `cosm.` item listed as the grant of two sources fails validation. A `bundle.sku.*` that grants Transcription fails validation.
24. **Idempotency.** Submitting the same run report twice creates one journal transaction; the second call returns the first result. The key of `{"a":"0x0123456789abcdef","i":0,"k":"run","s":"0x00000000075bcd15"}` is `909de25b25a2d70ca98a91f48c4098ee338ccb3dd518356c1d5db5519b06a4b6`.
25. **Run report.** The §10.4 example report grants 95 Ink, 5,025 Codex XP, 14 Quires and 0 Reliquaries. The same report with a `content_set` ID the host did not own at `started_utc` is rejected with `content_mismatch`. A report replayed by `econ-verify` with a different `active_ticks` is reversed: a wallet holding 40 of the report's 95 Ink ends at 0 with `econ_debt[INK] = 55`, and the next 100-Ink grant credits 45.
26. **Jurisdiction merge.** `default + eu + be + uk` gives `child_data_consent_age = 16`, `withdrawal_waiver_consent = true` and `vat_inclusive_display = true`. A KR account with platform age 18 is minor-flagged. A CA account gets `fr_store_text = true`. A US account on a BR IP gets `minor_chat_default = 1`.
27. **Minor caps.** A 15-year-old CNY account has a CNY 50 per-transaction and CNY 200 monthly cap: a CNY 81 set bundle is refused with `minor_cap_tx`, and a CNY 46 Folio Expansion after CNY 160 spent this month is refused with `minor_cap_month`. A 7-year-old CNY account is refused with `minor_store_blocked`. A 15-year-old account with an EUR store currency logging in from a JP IP has a monthly cap of `floor_div(5000 × 1999, 2900) = 3446` minor units (EUR 34.46).
28. **Purchases.** A new account's Buy button enters `Locked`. A minor-flagged account with a guardian monthly cap of 0 cannot complete any purchase.
29. **Set bundle.** Four unowned USD components at 5.99, 3.99, 7.99 and 1.99 price the set at 16.99; owning the 7.99 component prices it at `round_down_ending(mul_bp(1197, 9000), USD) = round_down_ending(1077, USD) = 999`, USD 9.99.
30. **Verifier.** Altering the tier of one record in an exported history makes `palimpsest-verify epoch` print `FAIL` for that row and exit with code 1. Altering the starting pity snapshot of epoch 2 so it differs from the pity-after of epoch 1 makes `palimpsest-verify chain` fail. Altering any audit record changes its chain hash and the day's Merkle root.
31. **Commit before seed.** A draw request whose epoch is still `Committed` and that carries no client seed, or carries a commitment different from the stored one, is refused with `epoch_mismatch` and posts nothing. In `fresh` mode two consecutive epochs bind different client seeds.
32. **Echo binding and rotation.** §3.1.1 binds 12 Illuminated and 12 Rubricated Echoes, each doc 14 Tier 2 Echo Illuminated. Period 13 features `echo.wren_long_index` again. A Featured Script banner of a period with `g = 1` (p001 included, DECISIONS 114) features `bundle.script.lost_chapter_cosmic_horror` and rates up `bundle.script.glyphs_ember`, `bundle.script.glyphs_hollow` and `bundle.script.kit_beastbinder`; over any 12 consecutive periods every Rubricated Script is rated up at least once and each even index exactly twice.
33. **Beginner target.** An account owning `echo.wren_long_index` cannot select it. If the selected target is obtained by Recollection, the next Beginner pull is refused with `beginner_target_required`. The welcome grant of 1,200 Ink is credited in the transaction that sets `margin.milestone.first_run_end`.
34. **Offline-only profile.** Before end of service, an offline-only profile shows every Codex premium track locked and no store; after the service reaches `EndOfService`, every released premium track is unlocked.
35. **End of service.** In end-of-service mode every released Codex premium track is unlocked, the Retrospective banner uses `banner.hazard.illuminated_std`, and carried pity from `eos_final` is applied on the first local pull.
36. **Transcription.** An account with `transcription = 40` claims `bundle.script.lost_chapter_wuxia` and keeps 8; a claim of `bundle.script.kit_monk` is then refused with `insufficient_transcription` and changes nothing. 768 qualified exits claim all 36 launch Scripts. A run report with 3 qualified exits in `mode.standard` adds 3; the same exits in an easing Custom run or an Apocrypha run add 0, and an invasion adds 0. A claim leaves every `PityState`, spark counter, Fold and Vellum balance unchanged, and an owned Script is never offered. Replaying a claim with the same `req_id` posts nothing; its idempotency source `s` is that `req_id` as `"0x"` plus 16 lowercase hex digits.
37. **Content-set gating.** At Early Access (all 12 core genres) p001 runs `banner.script.p001` featuring `bundle.script.lost_chapter_cosmic_horror`, and `banner.standard` lists 24 Illuminated entries. A test build whose content set leaves out only the 6 Lost Chapter Scripts of Cosmic Horror, Fairy Tale, Hardboiled Noir, Mythic Antiquity, Post-Apocalypse and Space Opera runs no `banner.script.p001`; its `banner.standard` has 18 Illuminated entries at 0.0556% base and 0.1416% consolidated each; its Featured Script banners show 0.0500% and 0.1020% for each of their 5 off-featured entries; `bundle.script.lost_chapter_cosmic_horror` is in no pool, Archive shelf or Transcription shelf. An entry that enters the content set inside period `k` joins every pool from period `k + 1`, with a new `config_hash` published 7 days before that period starts.
38. **Price and storefronts.** `store.sku.base_game` costs USD 19.99 before and after 1.0 and every `store.sku.folio_exp_<nn>` USD 9.99; the base game's matrix row is EUR 19.99, GBP 16.99, BRL 74.99, JPY 2,900, KRW 24,400, AUD 29.95, CAD 25.99, CNY 90, and every value satisfies its §7.2 ending rule.
39. **Chat defaults.** For an adult account in a public lobby, a party member who is not a platform friend starts voice-muted and a friend does not; a kick vote in a full party passes with 2 yes votes of the 3 other members and fails with 1.
40. **Keepsake curse odds.** A Keepsake opened from a Gothic Horror Reliquary as an Epic weapon is cursed with probability 2400 Bp and carries a pact with probability 480 Bp, and the public odds page shows `24.0000%` and `4.8000%`.
41. **Second Edition.** A Standard run whose player is present at `boss.ur_editor`'s death, takes the Second Edition door and ends `run.end.victory` with no `ev.player_died` in its Second Edition Folios reports `victory_present = 1` and `ironman = 1`: it pays the 300-Ink and 4,000-XP Ur-Text victory and grants `cosm.frame.ironman`; a second such run pays the victory again and grants no second cosmetic. The same run ending in a Second Edition party wipe or an Abandon reports `victory_present = 0`, pays the 20-Ink and 800-XP run end instead and grants no cosmetic. An Editor killed in a fused Second Edition Folio of Gothic Horror and Wuxia credits an account Reliquary with source genre `genre.gothic_horror`.
42. **Codex kill XP by rank.** A solo kill of the Erasure hunter adds 15 to `K[1]` (`rank.elite` kill XP, Canon §11.6); a solo kill of an invader adds 25.
43. **Second Edition Editors (§2.2).** A player present for the whole Final Second Edition Folio, where `boss.ur_editor_rev` and then `boss.author` die, reports 2 more `editor_kills_present`, receives 80 Editor Ink, 2 account Reliquaries whose source genre is that Folio's `FolioRecord.genre`, and 2 `pass.xp.editor_kill` grants; a kill of `boss.editor_v3_rev` pays one Editor kill.
44. **Achievements and Codex marks (§2.2, §2.4, §6.2, DECISIONS 141).** Unlocking a grade 2 Codex mark grants 80 Ink, 25 Quires and 2,000 Codex XP once per account, outside every cap, under idempotency kind `ach` with the mark's ID; a second unlock of the same mark posts nothing. The account's Ink and Quire totals from all doc 14 §10 entries equal the grade-table sum over that list (6,440 Ink and 1,960 Quires for the current 59 entries).
45. **No Prologue (§2.2, DECISIONS 132, 137).** The §2.2 mode table has five rows and none is a Prologue; finishing Folio 1 of an account's first run unlocks the first-run achievement exactly once, and finishing Folio 1 of a second run unlocks nothing more.

## Invariants annex

```
Σ doc15: earned randomness ⊗ paid certainty ⊗ disclosure ⊗ verifiability ⊗ offline completeness
W! juris.paid_random_items_enabled = false (compile-time)        N! premium currency
N! purchasable → {cur.ink, cur.quire, cur.vellum, RELIQUARY}      N! SKU → {XP, tier skip, pity, spark, currency}
W! premium track ⊆ cosm.* ∪ {ent.quill@tier100}                  W! free track identical ∀ accounts
W! odds(banner) identical ∀ accounts with access                 N! odds ← {Fortune, Serendipity, spend, playtime}
W! base_bp ≥ 1 ∧ p_bp(hard) = 10000                              N! zero-probability window
W! pull draws = 3 (tier, featured, index)                        W! reliquary draws = 2 + 1 raw u64
W! pity per family, never decays                                 N! pity reset on inactivity | rotation | update
W! spark leftover → vellum 1:1 at Ended                          W! featured unit → Archive after Ended ∧ expansion unit ∉ banner.standard
W! Fold = option | cosmetic                                      N! Fold → stat | cap | drop rate
W! commitment published before first draw                        W! reveal on seed change | period end | Ended | end of service
W! audit log hash-chained ∧ retained ≥ 1826 days                 W! odds shown at 4 dp, same screen as pull
W! purchases off by default                                      N! countdown | limited stock | rotating shop | daily login reward
N! trade | gift | cash-out of earned or random rewards            N! set-completion reward from random sources
W! refunds never revoke earned content                           W! end of service ⇒ all owned content usable offline
W! victory bonus ⇔ boss.ur_editor dies ∧ clear kept ∧ ink = 12 × final_folio ∧ xp = 160 × final_folio
W! easing run → active-minute income only                        W! mod_count ≥ 1 ⇒ apocrypha ⇒ ink = quires = xp = reliquaries = 0
W! quires(run) = min(400, run_score / 50) ∧ run_score := doc14 §4.6   W! achievement pay := grade table 40/80/160/320 ink, 10/25/50/100 quires
W! codex kill xp split once per (run, party size)                N! per-kill flooring penalty in co-op
W! satchel ≤ 5 ∧ |legendary+| ≤ 1 ∧ ilvl := d(f)                  N! keepsake identity change after open
W! client_seed bound after commitment received                   W! epoch start snapshot == previous epoch pity-after
W! ≤ 1 live banner per family per period                         W! draw-3 pools sorted by id bytes
W! entry ∉ build content set ⇒ ∉ every pool | featured | shelf     W! pools change only at a period boundary
W! ∀ cosm item: exactly one source                               N! purchase of an already owned item
W! juris merge only tightens ∧ guardian cap ≤ regional minor cap  N! VPN | wallet country loosens a rule
W! clawback stops at wallet 0, remainder → econ_debt              N! negative wallet balance
W! invader rewards := quires ≤ 60 per 24 h ∧ red ink marks       N! ink | codex xp | reliquary for invading
W! ∀ script: transcription route ∧ launch scripts = 768 exits    N! sku | ink | vellum → transcription
W! base game = USD 19.99 at EA and 1.0 ∧ folio expansion = USD 9.99   W! presentation-only mod ⇒ not apocrypha
W! second edition end ≠ victory ⇒ victory_present = 0            W! cosm.frame.ironman ≤ 1 per account ∧ only from an ironman clear
W! storefront = Steam only ∧ T0 := launch configuration           N! demo | public playtest build
N! prologue mode ∨ prologue-only reward                           W! achievement pay ∀ ach. entry ∪ codex mark := grade table
W! second edition rank.editor kill (6 bosses) → editor ink ∧ reliquary   W! fused folio genre := FolioRecord.genre
```
