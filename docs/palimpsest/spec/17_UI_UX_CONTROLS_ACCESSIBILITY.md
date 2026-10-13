# 17 — UI, UX, Controls and Accessibility

As of 2026-10-11. Status: BINDING. Written against canon-1.12 (adversarial check pending).

This chapter owns everything the player sees and touches that is not the 3D world itself: the genre UI skins, the HUD, every menu, the input action layer and its bindings, the accessibility suite (including the assist mode **Scribe's Mercy**), localization rules, onboarding, tooltips and feedback. It owns the namespaces `ui.`, `input.` and `access.` (Canon §19). Every rule below was checked against canon-1.11, the canon on disk, including limited building, Margin trophies, invasions, the canon-1.5 display rounding of `Fx` quantities, the retirement of mounts (canon-1.8) and of item durability (canon-1.10), and canon-1.11's small-rate display, intent replay log, Scribe's Mercy step 6c and World Law discovery. It also follows DECISIONS rows 89 to 129, among them Second Edition at Early Access (118), Steam as the only storefront (112), Steam Workshop mods (126), residents without relationships (127) and no photo mode (129).

Two things in this chapter touch the simulation: the layout of the command's button bits 26–31 (§12.2) and the Scribe's Mercy damage step (§13.1). Everything else is presentation. It reads simulation state through doc 02's `ViewFrame` and conversion boundary and changes it only through `PlayerCommand` bits and `PlayerIntent` records; the replay log records both (Canon §2.4). UI code may use floats; content records in this chapter are integers only (Canon §1.6). Under Canon §2.4, `ui.*` records are presentation records covered by `presentation_hash`, so co-op peers may run different UI skins, icons, tips and localization (DECISIONS 87); `input.*` and `access.*` records are content covered by `content_hash`. Each player's own bindings and setting values live in the client profile and enter neither hash.

At the product owner's request, the closing section "Product-owner question round" asks one or more clarifying questions per system. Each question names the default this chapter already binds, so implementation proceeds unblocked.

## Imports

| Source | Sections | Consumed |
|---|---|---|
| DESIGN_BRIEF, DECISIONS | Brief §1, §2, §5, §8; ledger rows 3, 15, 16, 23, 26, 33, 38, 40, 46, 47, 48, 49, 55, 56, 60, 65, 74, 75, 77, 78, 82, 83, 87, 90, 92 to 95, 97, 98, 100 to 108, 112, 118, 120, 125, 126, 127 and 129, and rows 96 and 104 as open dependencies (§11.6, Interface assumptions); triage Q2, Q10 | first person with third-person toggle; Archetype × Skin; deterministic sim under a Bevy client; earned-only randomness with exact odds; invasions; limited building; per-genre manuscript skins; softer Erasure; Glyph Library picks; text-only narration; 15 launch locales; the procedural closing manuscript; Grades and Mercy; replays; marked hallucinations; no mounts; World Laws discovered in play; cursed capsules shown before payment; no item durability; voice defaults; the walkable Margin and its residents' text arcs; Mercy never blocks achievements or Grades; Lucre passes between players only as dropped piles; presentation-only peer differences; "Crowns" kept; FOV 60–120, default 90; ink gore with a red-blood toggle; creator presets; PC only through 1.0 with Steam Deck Verified; the movement kit's sprint-slide input; Second Edition at Early Access with its door, stakes and Ironman mark; hot and warm host migration; the lobby browser and kick votes; Steam as the only storefront, with no crossplay setting and no friend codes; invasions in an update before 1.0; the Scribe at 1.0; Steam Workshop mods synced to co-op peers; residents without relationships; no photo mode; asynchronous traces and streaming votes as later screens |
| Canon | §1.1, §1.5, §1.9, §1.10, §2.1–§2.4, §3.6 | ID grammar; display-unit rule, small-rate display and `Fx` display rounding; `Bam`; text keys and name order; 60 Hz tick; light overrides; the replay command and intent records; `content_hash` and `presentation_hash`; seed display and strict decoding |
| Canon | §4.1, §4.2, §5.2–§5.10 | eye heights, lean, dodge; genres, skins, palettes, `spawn_block_max`; Contradiction states and torn page; Edge; local genre |
| Canon | §6.4, §7, §8, §10.1, §10.5, §10.8 | derived-stat rows; skills; XP; damage colors and the icon duty; the pipeline and its step 6c; the entry points; lethal resolution |
| Canon | §11.1, §11.8, §12, §13.2–§13.9, §14 | statuses, player-controlled semantics; resources and two-channel light; slots, rarity, quality, knowledge, curses, item contract; currencies |
| Canon | §15.1, §16.3–§16.5, §17.1, §17.5–§17.9 | minions and their three inputs; kin; afflictions; patrons; Colophon and exits; co-op modes; ghosts; the retirement of mounts; Margin trophies; invasions |
| Doc 02 | §2.2, §5.5, §7.2, §7.5, §8.4–§8.8, §9, §10, §13.4, §14, §15 | `tts` 0.26.3 and AccessKit through `bevy_winit`; command bits 0–24 and intents; `ViewFrame`, `ChunkView.light`; `conv` with `look_add`, `rad_to_bam`, `stick_to_i8`; FOV, post stack, page turn; `ev.noise` heard level and arrival direction; input layer, look state, widgets, 1280 × 800 authoring, Fluent; `modset_hash` and Workshop delivery; Deck Verified; UI budget |
| Docs 03, 04, 05, 06, 07, 08 | 03 `comp.player`, `ev.noise` payload, `ev.law_discovered`; 04 §1.1, §1.2, §2.1, §2.4, §6.4–§6.6, §7, §8.2, §11.3, §11.5, §12.4, §13; 05 §1.6, §1.7, §6.3–§6.8, §15.4, §16.3, §18; 06 §3, §4, §5.3, §6, §8.2, §16; 07 §4.1–§4.3, §5.3, §6.1, §6.4, §8.3; 08 §4.3, §9, §11 | Mercy fields, noise payload, the law-discovery event; ability slots, modifier kinds, creation flow, kin traits, level-up, Interpolation offers, respec, talents, wound tiers, darkness and breath, madness phantoms, Echo body slots; stance speeds, logical inputs, spread, sway and hold breath, lean, wards, failure outcomes, combat feel; casting, Grimoire, wheel, lore essences, grammar, cost, prayers; craft quality roll and distribution, station queue, recipe kinds, craft channel, disassembly; respirators, brewing, toxicity |
| Docs 09, 10, 11, 12, 13 | 09 §2.1–§2.3, §6.1, §7, §8, §12, §13.2–§13.5, §13.7; 10 §2–§6, §9; 11 §2.2, §7.1, §10–§12, §14, §17; 12 §4, §4.1, §5.5, §5.7, §11.3, §20.4–§20.6; 13 stalkers, §6 minion contract, §13.4–§13.6 | stealth light, awareness states, noise IDs, `StealthHud`, distractions, lockpicking, hacking, pickpocketing; knowledge flags and display formats, Ledger; instance fields, perk columns, Reliquaries, Engines and the cursed capsule, shops, inventory, dropped Lucre; World Laws and their discovery, exit offers and preview, light bands, the Folio quest; stalker shimmer, minion commands and leash, hirelings, companion approval, Margin residents |
| Docs 14, 15, 16, 18 | 14 §2.1, §3, §3.8, §4, §4.7, §5, §5.1a, §5.5–§5.7, §7.4, §8.9, §9.4, §10, §11; 15 §2.2, §2.8, §3, §3.9, §3.9.1, §4.1, §4.3, §5–§8; 16 §1.2, §2.2, §3.3, §3.4, §3.8, §4.1, §5.7, §5.8, §6.8–§6.10, §7.5, §9, §11.8; 18 §3.1, §3.5, §4.6, §5.4, §8.4, §8.5a, §9.1, §9.10, §9.11 | lobby, Folio phases, exit vote, boons, rests, ghosts, Erasure clock, score and payout, closing manuscript inputs, Margin and its trophy alcove, the Dramatis, resident arcs, Glyph Library, Grades, Second Edition with its stakes and Ironman clear, leaderboards, achievements, Prologue; Ink sources, hours to earn, Inkwell, Vault, Vellum Archive and Transcription shelf, provable fairness and the Fairness panel, Codex pass, Store, parental, chat safety; solo pause, lobby browser and LAN, handshake notices, client states, intents, sanitizer, snapshot `HUD` and `STEALTH` sections, party and run state, stale entities, co-op rules, kick votes, host migration, voice mutes, Workshop; art palettes, fonts, stealth readability, gore modes, Margin cast dialogue, the closing manuscript, the text-only Narrator, Scribe opt-out, launch locales |
| R3, R7 | R3 §1 "Accessibility", §5 item 4; R7 §2.7, R-SH7 | Hades God Mode values [VERIFIED]; speed 50–100%, FOV 60–120, UI 75–200%; aim-assist structure [RECALLED], numbers tuned here |

## 1. UI principles

| # | Principle | Binding rule |
|---|---|---|
| P1 | The simulation is the truth | Every value on screen comes from `ViewFrame`, `OwnerDelta`, `PartyState` or a pure function of `palimpsest_sim` called with integer inputs. The UI never computes a gameplay number with floats and never writes simulation state except through commands and intents. |
| P2 | Readable first, manuscript second | Frames, ornaments and titles carry the genre. Body text, numbers and tooltips always use the body font (§2.4) at contrast ≥ 4.5:1. |
| P3 | Never color alone | Every color-coded signal also has a shape, an icon or text: rarity pips and letters, damage-type glyphs, hostile diamonds, ally circles, band labels, ▲/▼ comparison arrows. |
| P4 | Every number explains itself | Holding `input.ui_details` on any derived number opens its formula breakdown (§5.6). |
| P5 | Exact odds | Every random reward or service (Reliquaries, Engines, Inkwell, boon rerolls, prayers) shows its exact probability to 4 decimals (`pct4`, doc 15 §3.11.1) before the commit input, with live pity counters. Crafting and brewing show the exact probability of every quality tier (§6.1, §6.3). Combat chances (crit, Contradiction failure, Death's Door) show on the sheet and the weapon panel. |
| P6 | Short paths | Mute, block and report: at most 2 inputs from the party list or the scoreboard (doc 15 §8.5). Scribe's Mercy: 2 inputs after the pause menu opens (§11.6). Settings: 3 inputs after the pause menu opens, landing on the Accessibility page. |
| P7 | Device parity | Every screen is fully usable by keyboard alone, by gamepad alone and by mouse plus keyboard. Steam Deck Verified rules (doc 02 §14) hold on every screen, including first launch. |
| P8 | Honest time | Solo pauses while any `ui.menu.*` screen is open (doc 16 §2.2), except while an invader is present, when nothing pauses; wheels, the held scoreboard, prompts, tips and the lock, hack and pickpocket panels (§10.11, §10.12) are HUD and never pause. Co-op never pauses; open menus draw a 4 px "live page" border in the `hostile` token and the damage-direction ring stays visible above the menu. |
| P9 | No dark patterns | No countdowns on any real-money or account-currency offer, no pop-up offers, no confirm-shaming copy, no pre-ticked boxes, no "are you sure you want to miss out" dialogs (doc 15 §7.5). |
| P10 | Budget | All UI rendering fits doc 02's 0.5 ms Deck GPU budget and the 128 MiB UI-and-fonts RAM budget. |
| P11 | Integer display | Every displayed simulation number follows Canon §1.5: `Milli` as `floor(x / 1000)` (current HP as `ceil_div`, per-second rates below 10 display units as `floor(x / 100)` tenths with one decimal), `Bp` percentages with one decimal by `round_half_even`, and `Fx` distances and speeds with `round_half_even` at the shown precision ("5.2 m" is `round_half_even(raw_mm, 100)` tenths). |

---

## 2. Genre UI skins

### 2.1 Skin model

Every screen and HUD element is themed by exactly one skin record `ui.skin.<genre_suffix>` (16 records) or `ui.skin.margin` (the hub and every out-of-run screen). The **active skin** is chosen per frame:

```
active_skin(view) =
    if access.consistent_skin                      { ui.skin.margin }
    else if view.in_run && view.local_player_alive { ui.skin.<suffix of skin_genre(local_player.pos)> }
    else if view.in_run                            { ui.skin.<suffix of the Folio genre> }                 // ghost or loading
    else                                           { ui.skin.margin }
skin_genre(pos) = if pos lies in the Folio's seam zone { Folio genre }     // Canon §5.8: the Folio genre supplies skins there
                  else { local_genre(pos) }                                // bleed zones use their own genre
```

A candidate skin becomes active only after it has been the candidate for 30 consecutive ticks, so a player standing on a bleed boundary never sees the skin flicker. The change then crossfades all tokens over 500 ms (presentation only). A menu captures the active skin when it opens and keeps it until it closes.

### 2.2 Color tokens

Each skin defines four text-safe tokens (`paper`, `ink`, `accent`, `muted`); the global semantic tokens below complete the set. Light-paper skins are Fairy Tale, Mythic Antiquity, The Redaction, The Unwritten and The Ur-Text; the rest use dark paper. Tokens derive from the Canon §5.9 palette: dark skins darken the darkest palette color 55% toward black for `paper` and lighten the brightest toward white until `ink` reaches 7:1; light skins use the lightest palette color as `paper` and darken the darkest toward black until `ink` reaches 7:1. `accent` and `muted` are the closest palette-derived colors reaching 4.5:1 on `paper`. Ratios are WCAG 2.1 relative-luminance ratios, computed and verified by acceptance test 9. The table below is binding; the derivation only records how it was produced. Doc 18 §3.1's `ink` and `paper` swatches color diegetic manuscript surfaces (title cards, Lore pages, the page-turn texture), never UI chrome or UI text (doc 18 §3.1 agrees).

| Skin | paper | ink | accent | muted | ink:paper | accent:paper | muted:paper |
|---|---|---|---|---|---|---|---|
| `ui.skin.mythic_fantasy` | `#1B311B` | `#D2E1F0` | `#C9A227` | `#92A3A5` | 10.50 | 5.78 | 5.33 |
| `ui.skin.gothic_horror` | `#1A0E13` | `#E9E7E1` | `#AF6E72` | `#A19B99` | 15.22 | 4.75 | 6.87 |
| `ui.skin.cosmic_horror` | `#061315` | `#E3F6CF` | `#588571` | `#96A78E` | 16.51 | 4.51 | 7.39 |
| `ui.skin.space_opera` | `#050711` | `#FFD0B1` | `#3D7BFF` | `#A88A79` | 14.27 | 5.24 | 6.29 |
| `ui.skin.cyberpunk` | `#08050E` | `#99F5FF` | `#FF2E88` | `#66A1AB` | 16.24 | 5.78 | 6.98 |
| `ui.skin.post_apocalypse` | `#302315` | `#E7DAC4` | `#E5693D` | `#A79A87` | 11.06 | 4.66 | 5.53 |
| `ui.skin.weird_west` | `#151C23` | `#F3E2C8` | `#A97763` | `#A59D8E` | 13.52 | 4.50 | 6.39 |
| `ui.skin.hardboiled_noir` | `#070707` | `#C5C5C5` | `#CE5555` | `#828282` | 11.67 | 4.85 | 5.24 |
| `ui.skin.wuxia` | `#0E1A15` | `#F0E9DA` | `#CA6363` | `#A1A195` | 14.76 | 4.65 | 6.84 |
| `ui.skin.steampunk` | `#211A12` | `#BFDCD9` | `#B5793A` | `#889893` | 11.84 | 4.72 | 5.70 |
| `ui.skin.fairy_tale` | `#F2D7E8` | `#2C4C2E` | `#685920` | `#526651` | 7.18 | 5.15 | 4.64 |
| `ui.skin.mythic_antiquity` | `#F0E6D2` | `#274C74` | `#8D551E` | `#4D6986` | 7.15 | 4.91 | 4.61 |
| `ui.skin.oneiric` | `#130C23` | `#FFE7F3` | `#8E7CFF` | `#AC9AAA` | 16.28 | 5.86 | 7.20 |
| `ui.skin.redacted` | `#F2F2F2` | `#1A1A1A` | `#CC242C` | `#666666` | 15.55 | 4.86 | 5.13 |
| `ui.skin.unwritten` | `#FFFFFF` | `#525252` | `#707070` | `#767676` | 7.81 | 4.95 | 4.54 |
| `ui.skin.ur_text` | `#E8D9B0` | `#1C1408` | `#78571C` | `#635943` | 13.00 | 4.71 | 4.93 |
| `ui.skin.margin` | `#1E1A14` | `#EDE3CC` | `#C9A227` | `#A59D8C` | 13.57 | 7.16 | 6.43 |

**Global semantic tokens** (identical in every skin; checked against the HUD backplate `#101010`):

| Token | Default | Deutan/protan/tritan (`access.colorblind_mode`) | Monochrome | Shape or icon that always accompanies it |
|---|---|---|---|---|
| `hp` | `#E5484D` (4.86:1) | `#D55E00` | `#FFFFFF` | heart glyph at the bar's left end |
| `stamina` | `#46A758` (6.28:1) | `#009E73` | `#BBBBBB` | chevron glyph |
| `aether` | `#3E9BF0` (6.48:1) | `#56B4E9` | `#DDDDDD` | genre Aether-skin glyph |
| `ward` | `#3FD7E0` (10.88:1) | `#F0E442` | `#FFFFFF` with hatching | hexagon glyph; hatched fill |
| `composure` | `#B18CFF` (7.31:1) | `#CC79A7` | `#999999` | eye glyph |
| `toxicity` | `#A3C93A` (9.94:1) | `#E69F00` | `#CCCCCC` | flask glyph and band ticks |
| `breath` | `#E8F4FF` (17.05:1) | unchanged | unchanged | bubble glyphs |
| `xp` | `#F2C14E` (11.34:1) | `#E69F00` | `#FFFFFF` | quill glyph |
| `hostile` | `#FF5A5A` (6.22:1) | `#D55E00` | `#FFFFFF` | diamond marker |
| `ally` | `#4CC3FF` (9.57:1) | `#56B4E9` | `#BBBBBB` | circle marker |
| `neutral` | `#E0E0E0` (14.41:1) | unchanged | `#888888` | square marker |
| `pencil` | `#A0A0A0` (7.28:1) | unchanged | unchanged | pencil glyph and italic text (Narrator notes on the HUD, §16.4) |

**Rarity.** Canon §13.3 colors stay the default. Every rarity also shows a letter and pips at every size: Common `C` 0 pips, Uncommon `U` 1, Rare `R` 2, Epic `E` 3, Legendary `L` 4, Exotic `X` 5, Palimpsestic `P` 6 pips plus the animated prismatic border. Colorblind modes swap only the rarity colors: Common `#E6E6E6`, Uncommon `#009E73`, Rare `#56B4E9`, Epic `#CC79A7`, Legendary `#E69F00`, Exotic `#F0E442`, Palimpsestic a 2-stripe black-and-white animated border (the Okabe-Ito set).

**Damage types** keep the Canon §10.1 colors in every mode and always show their glyph (§2.5), so a colorblind player reads the glyph.

### 2.3 Frame styles

Every skin names one frame style. Frame metrics are in reference pixels at layout scale 1.0 (§3.1).

| Skin | `frame` | Border px | Corner ornament px | Title bar px | Ornament motif | Panel texture |
|---|---|---|---|---|---|---|
| mythic_fantasy | `illuminated_border` | 6 | 28 | 36 | vine scroll with gilt initials | vellum |
| gothic_horror | `gothic_tracery` | 6 | 32 | 36 | pointed-arch tracery | black-letter psalter |
| cosmic_horror | `stained_vellum` | 4 | 24 | 32 | tentacular marginal doodles | water-stained vellum |
| space_opera | `holo_bezel` | 3 | 16 | 32 | chamfered bezel with status lights | glass |
| cyberpunk | `scanline_terminal` | 2 | 12 | 28 | glitch brackets | CRT scanlines (off under reduced motion) |
| post_apocalypse | `riveted_scrap` | 6 | 24 | 32 | rivets and tape | rusted sheet |
| weird_west | `wanted_poster` | 5 | 24 | 36 | nail heads and torn edges | broadsheet newsprint |
| hardboiled_noir | `typewritten_file` | 3 | 16 | 32 | paper clips, case stamps | manila folder |
| wuxia | `brush_scroll` | 5 | 28 | 36 | scroll rods and seal stamps | rice paper |
| steampunk | `brass_gauge` | 6 | 28 | 36 | gears, gauge rims | brass plate |
| fairy_tale | `storybook_vine` | 6 | 32 | 36 | briar roses | storybook page |
| mythic_antiquity | `meander_fresco` | 5 | 24 | 36 | Greek key band | fresco plaster |
| oneiric | `torn_collage` | 4 | 24 | 32 | collage cut-outs | xerox collage |
| redacted | `redaction_bar` | 2 | 8 | 28 | black redaction bars, file numbers | photocopy |
| unwritten | `blank_margin` | 1 | 0 | 28 | none | blank page |
| ur_text | `palimpsest_layers` | 6 | 32 | 40 | layered faded scripts of every genre | scraped vellum |
| margin | `illuminated_border` | 6 | 28 | 36 | quills and inkwells | desk leather |

### 2.4 Typography

| Role | Size px (reference) | Font | Rules |
|---|---|---|---|
| Display (title cards, screen titles) | 32 | the skin's display font | never below 24 px; never used for numbers or body text |
| H1 | 24 | display font | |
| H2 | 20 | body font, bold | |
| Body | 16 | body font | the default for every sentence |
| Small | 14 | body font | |
| Caption | 12 | body font | the minimum for any text (doc 02 §10) |
| Numerals | same as context | body font, tabular figures | every column of numbers aligns |

Display fonts are doc 18 §3.5's per-genre heading faces (`art.font.*`, owner doc 18; all SIL OFL 1.1 or Apache 2.0; acceptance test 31 checks every font file's license field), copied here: Mythic Fantasy **Cinzel**; Gothic Horror **UnifrakturCook**; Cosmic Horror **Special Elite**; Space Opera **Orbitron**; Cyberpunk **Bebas Neue**; Post-Apocalypse **Black Ops One**; Weird West **Rye**; Hardboiled Noir **Playfair Display**; Wuxia **Ma Shan Zheng**; Steampunk **Limelight**; Fairy Tale **Berkshire Swash**; Mythic Antiquity **Marcellus**; Oneiric **Caveat**; The Redaction **Space Mono**; The Unwritten **Cormorant Garamond**; The Ur-Text **Cinzel Decorative**; the Margin **EB Garamond**. When doc 18 changes a heading face, doc 18 wins and this list follows. Doc 18's per-genre body and numeral faces set Lore and Journal reading pages and title cards; every functional UI string (HUD, menus, item cards, tooltips, numbers) uses the `access.font` body font below, because P2 puts readability first (doc 18 §3.5 agrees).

Body fonts (`access.font`): `standard` **Noto Sans** (default), `legible` **Atkinson Hyperlegible** (the face doc 18 §3.5 ships; it also replaces doc 18's genre body faces on Lore pages), `dyslexic` **OpenDyslexic**. CJK locales always use **Noto Sans CJK** (JP, KR, SC or TC per locale) for body and **Noto Serif CJK** for display; a display font without the glyphs of the active locale falls back to Noto Serif CJK or Noto Serif.

**Effective size** for every text element. This is presentation code, so floats are allowed; `round` is half away from zero (`f32::round`).

```
base_scale = min(window_w / 1280, window_h / 800)                          // doc 02 §10
menu_px    = round(base_px × base_scale × access.ui_scale_bp / 10000 × access.text_scale_bp / 10000)   // menus, item cards, tooltips, Journal
hud_px     = round(base_px × base_scale × s)                               // HUD elements; s from §3.1; text scale does not apply
sub_px     = round(20 × base_scale × access.subtitle_size_bp / 10000)      // subtitles (§13.2)
px         = if the string's script is Han, Kana or Hangul { max(px, 14) } else { max(px, 12) }
```

Worked examples: Body (16 px) at 1920 × 1080 has `base_scale = 1.35`; at UI scale and text scale 10000 it renders at `round(21.6) = 22` px, and at text scale 15000 at `round(32.4) = 32` px. Body at 1280 × 800 with UI scale 7500 renders at `round(12.0) = 12` px, exactly the floor; Small (14 px) in the same case computes `round(10.5) = 11` and is raised to 12; a CJK Caption (12 px) computes `round(9.0) = 9` and is raised to 14.

### 2.5 Icon set

Icons are `ui.icon.*` records: a 24 × 24 vector glyph (authored as integer path coordinates on a 0..=2400 grid), drawn in the token named by the caller. Every icon has a text label key `loc.ui.icon.<name>.name` for the screen reader.

| Group | Icons and their glyphs |
|---|---|
| Damage types (Canon §10.1, required there) | `ui.icon.dmg_slash` diagonal cut; `dmg_pierce` arrowhead; `dmg_blunt` hammer face with impact lines; `dmg_fire` flame; `dmg_frost` six-arm snowflake; `dmg_shock` zig-zag bolt; `dmg_acid` droplet with two bubbles; `dmg_radiation` trefoil; `dmg_sonic` three concentric arcs; `dmg_toxin` droplet with a skull notch; `dmg_radiant` eight-ray sun; `dmg_umbral` crescent; `dmg_psychic` spiral eye; `dmg_arcane` six-point rune star; `dmg_true` hollow diamond |
| Contradiction (Canon §5.5) | `ui.icon.c_coherent` none drawn; `c_strained` page with one dog-ear (C = 1) or two (C = 2); `c_contradicted` page with a crack (C = 3, 4, 5, crack count = C − 2); `c_disconnected` torn page (canon) ; `ui.icon.edge` forward-pointing quill (Anachronism Edge active against the current target) |
| Status categories (Canon §11.1) | DoT: droplet frame; debuff: downward chevron frame; control: chain frame; buff: upward chevron frame; mental: eye frame; need: bowl frame; indicator: square frame; special: star frame; affliction (§10.6): thorn-ring frame. Inside each frame sits the status's own glyph from `art.icon.<status>` (doc 18). |
| Status frames | `ui.icon.frame_dot`, `frame_debuff`, `frame_control`, `frame_buff`, `frame_mental`, `frame_need`, `frame_indicator`, `frame_special`, `frame_affliction`, the frames of the row above |
| Resources | `ui.icon.res_hp` heart; `res_stamina` chevron; `res_aether` a slot filled by doc 18's genre Aether glyph; `res_ward` hexagon; `res_composure` eye; `res_toxicity` flask; `res_breath` bubble; `res_satiation` bowl (Fuel can for `kin.construct`); `res_temperature` thermometer; `res_respirator` mask; `xp` quill; `mercy` quill over a heart |
| Markers | `ui.icon.marker_hostile` diamond; `marker_ally` circle; `marker_neutral` square; `marker_minion` small circle with an owner-color stripe; `marker_ping` circle with a center dot; `marker_danger` triangle; `marker_objective` star; `marker_colophon` open book; `marker_scriptorium` quill; `marker_shop` coin; `marker_reliquary` chest; `marker_engine` crank; `marker_remains` folded page; `marker_exit` door; `marker_hold` banner (minion hold cell); `marker_claw` claw (minion attack target) |
| Axioms | `ui.icon.ax_arcana` eight-point star, `ax_techne` gear, `ax_mythos` eye-in-triangle, `ax_verve` flame-shaped exclamation |
| Run marks | `ui.icon.ironman` an unbroken wax seal crossed by a quill (an Ironman clear's leaderboard mark, §11.10) |

---

## 3. HUD

### 3.1 Coordinate system and scaling

- The HUD is laid out on a logical canvas of `W × H` logical pixels, where the physical window is divided by `base_scale` (§2.4): 1280 × 800 on Steam Deck, 1422 × 800 at 1920 × 1080, 1911 × 800 at 3440 × 1440.
- **Safe rect.** Margins `Mx = My = 24` by default; `access.tv_safe_area` sets `Mx = 64`, `My = 40`. The safe rect is `[Mx, W − Mx] × [My, H − My]`.
- **HUD scale.** `s_fit = min((W − 2·Mx) / 1232, (H − 2·My) / 752)`. With `access.hud_layout = standard` (default), `s = s_fit × access.hud_scale_bp / 10000` with `access.hud_scale_bp ∈ 7500..=10000` in steps of 500, and the §3.2 table applies. With `access.hud_layout = large`, `s = s_fit × 12500 / 10000` and the §3.4 table applies. At the defaults on Deck `s = 1.0`; with the TV-safe area `s_fit = min(1152/1232, 720/752) = 0.935`, so the Large layout runs at `s = 1.169`.
- **Subtitles** are the one element whose size ignores `s`: the box is always 592 × 100 reference px × `s_fit`, its offset scales with `s`, and its text follows `access.subtitle_size_bp` (§13.2).
- **Anchors** are the nine points TL, TC, TR, ML, C, MR, BL, BC, BR of the safe rect. An element's pivot is the same-named point of the element. Its rectangle is placed so the pivot sits at `anchor + offset × s`, with size `(w × s, h × s)`. Offsets use screen axes: +x right, +y down.
- Element positions are never computed from simulation coordinates except world-space markers (damage numbers, pings, nameplates), which project through the camera.

### 3.2 Element table

Rectangles below are at 1280 × 800, `s = 1.0`, margins 24. They stay pairwise disjoint and inside the safe rect at every `access.hud_scale_bp` step, with the default and the TV-safe margins, at logical sizes 1280 × 800, 1422 × 800, 1911 × 800 and 1280 × 960 (acceptance test 10). Rectangles are half-open, so edges may touch. Priority is the draw order: higher draws above lower, and every world-space marker draws below every HUD element.

| ID | Anchor | Offset | Size | Rect at 1280 × 800 | Priority | Visible when | Content |
|---|---|---|---|---|---|---|---|
| `ui.hud.folio_header` | TL | (0, 0) | 340 × 56 | 24,24–364,80 | 60 | in a run | Folio number, Volume, local genre sigil and skinned name (bleed zones show both sigils split diagonally), four axiom pips of the local genre, one glyph per World Law active at the player's position that the viewer has discovered (§7; nothing for any other Law), Erasure clock and stack count |
| `ui.hud.party` | TL | (0, 72) | 248 × 164 | 24,96–272,260 | 80 | 2+ slots | one 52 px frame per other slot: name, level, HP bar (`hp_bp`), ward bar, up to 6 status icons, downed bleed timer, ghost glyph, voice-speaking glyph, ping in ms on hold of `input.scoreboard` |
| `ui.hud.minions` | TL | (0, 248) | 248 × 148 | 24,272–272,420 | 50 | owner has ≥ 1 minion | §10.3 |
| `ui.hud.compass` | TC | (0, 0) | 480 × 32 | 400,24–880,56 | 70 | always in a run | 180° heading tape, cardinal letters, pings, party markers, objective markers, Colophon marker once open |
| `ui.hud.boss_bar` | TC | (0, 40) | 520 × 36 | 380,64–900,100 | 85 | a boss, Editor or hunter is alerted toward the party | name, HP bar with phase ticks, ward bar, champion aspect icons with counter icons |
| `ui.hud.invasion` | TC | (0, 84) | 480 × 28 | 400,108–880,136 | 90 | an invader is present | "A Redactor is reading this page", invader HP when visible, time left of the 300 s window |
| `ui.hud.ghost` | TC | (0, 144) | 480 × 28 | 400,168–880,196 | 88 | the local Erratum is a ghost | §10.4 banner and resurrection forecast |
| `ui.hud.notifications` | TR | (0, 0) | 320 × 196 | 936,24–1256,220 | 30 | events | 5 lines of 36 px, newest on top, 4 s each: pickups, identifications, skill-ups, level-ups, favor changes, World Law discoveries (§7); Narrator notes take up to 2 lines for 8 s (§16.4) |
| `ui.hud.minimap` | TR | (0, 212) | 176 × 176 | 1080,236–1256,412 | 20 | `access.minimap` on (default off) | top-down slice of the visited map at the player's floor (§8) |
| `ui.hud.crosshair` | C | (0, 0) | 64 × 64 | 608,368–672,432 | 100 | first person, not in a menu | reticle, spread ring from `cone` (doc 05 §6.3), hit markers (doc 05 §18), lean chevrons (§10.9), cast-progress arc |
| `ui.hud.prompt` | C | (0, 72) | 420 × 48 | 430,448–850,496 | 95 | an interactable is targeted | binding glyph, verb, target name, hold-progress ring for timed interacts |
| `ui.hud.vote` | C | (0, −128) | 480 × 64 | 400,240–880,304 | 75 | `ExitVote` or `VictoryChoice` | each door's voter count, `need`, hold counter `hold/120` and interact counter `interact_hold/180` (doc 14 §3.6) |
| `ui.hud.downed` | C | (0, −64) | 360 × 40 | 460,316–820,356 | 92 | the local Erratum holds `st.downed` | §10.4a bleed-out and revive bars |
| `ui.hud.vitals` | BL | (0, 0) | 320 × 104 | 24,672–344,776 | 100 | always | HP bar 320 × 20 with ward bar 320 × 6 above it; stamina 256 × 10; aether 256 × 10 (skinned name); composure 256 × 10; toxicity 256 × 6 with band ticks; breath bubbles when draining; satiation, temperature and respirator glyphs; Scribe's Mercy quill with its percent when on |
| `ui.hud.status` | BL | (0, −112) | 320 × 72 | 24,592–344,664 | 65 | ≥ 1 status | 2 rows × 9 icons of 32 px with 4 px gaps; each shows stacks and a remaining-time arc; afflictions first, then control, DoT, debuff, mental, buff, indicator |
| `ui.hud.body` | BL | (328, 0) | 56 × 104 | 352,672–408,776 | 45 | always | paper doll of the body plan, each part tinted by wound tier 0 to 4, severed parts drawn as dashed outline |
| `ui.hud.quickbar` | BC | (0, 0) | 432 × 56 | 424,720–856,776 | 90 | always | 4 to 8 slots of 48 px with 6 px gaps (Canon §13.2), stack counts, binding glyphs, selected slot outline |
| `ui.hud.abilities` | BC | (0, −64) | 212 × 48 | 534,664–746,712 | 85 | ≥ 1 active talent | 4 slots of 44 px with 12 px gaps, cooldown sweep and seconds |
| `ui.hud.stealth` | BC | (0, −120) | 264 × 44 | 508,612–772,656 | 70 | §10.10 visibility rule | §10.10 |
| `ui.hud.subtitles` | BC | (0, −176) | 592 × 100 | 344,500–936,600 | 98 | a captioned line plays | §13.2 |
| `ui.hud.weapon` | BR | (0, 0) | 320 × 104 | 936,672–1256,776 | 95 | always | weapon skinned name and rarity, magazine/reserve or charges or heat, fire mode, Contradiction icon with potency and failure percent, counter icons the weapon carries |
| `ui.hud.spell` | BR | (0, −112) | 320 × 64 | 936,600–1256,664 | 80 | ≥ 1 castable entry | active spell name, cast cost in display aether, cooldown, cast type glyph; prayer readiness and favor with the local patron (§10.7) |
| `ui.hud.chat` | ML | (0, 78) | 320 × 100 | 24,428–344,528 | 40 | a message in the last 10 s, or chat open | 4 lines; full history in the chat panel |

Crosshair rings (screen space, centered on the crosshair, radii in reference px × `s`): detection arcs 180 (§10.10), footstep ticks 200 (§10.10), hit direction 220, sound visualization 260 (§13.7); wheels use radius 140 (§6.5).

World-space elements (no rectangle): damage numbers (doc 05 §18), pings, nameplates, Remains markers, minion command markers, exit door panels (§9.1), rarity beams on ground items (§16.2), doc 09 noise rings and doc 18 sound ripples (§13.7). **Nameplates**: allies always; a hostile while it is targeted (under the crosshair on a 48,000 mm ray for at least 6 consecutive frames, or the last hostile the player damaged within 180 ticks) or while it bears `st.marked`. A hostile nameplate shows the skinned name, a rank glyph, an HP bar from `hp_bp`, a ward bar when warded, up to 4 status icons, champion and elite aspect icons with their counter icons, a Contradiction icon when its own components are Strained or worse, and, from the 10th Bestiary kill of its template, its resistance and immunity glyphs (doc 14 §5.5).

**Vitals block** (`ui.hud.vitals`; offsets in reference px from its top-left). Standard: ward bar y 0–6 and HP bar y 8–28, both x 0–320 on one scale, the heart glyph drawn inside the HP bar's left 20 px; stamina y 32–42, aether y 46–56 and composure y 60–70, each a bar x 0–256 with its glyph inside the left end and its value right-aligned in x 260–320; toxicity y 74–80, x 0–256, with ticks at 50,000, 75,000 and 100,000; glyph row y 84–104: breath bubbles (one per 2 s of `res.breath`, shown only while breath is below max), satiation or Fuel, temperature band, the respirator gauge (§6.3), the Mercy quill with its percent. Large (§3.4, 256 × 80): ward y 0–6 and HP y 8–28 (x 0–256), stamina y 32–42, aether y 46–56, glyph row y 60–80, where composure and toxicity appear as glyphs with their values only while composure is below max or toxicity is above 0.

**Erasure clock** (`ui.hud.folio_header`). With `c = t − clock_start` (doc 14 §3.8: the entry tick, or the Editor's death tick in a boss Folio) and `ERASURE_ONSET` = 43,200 ticks (720 s, Canon §17.1): before onset it reads "Erasure in M:SS" for `43200 − c` ticks; from onset it reads "Erasure k · next M:SS", where `k` is the local Erratum's `st.erasure` stacks and stack `k + 1` lands at `c = 43200 + 1800 × k` (Canon §17.1 `ERASURE_STEP`); at 10 stacks, the cap, it reads "Erasure 10" with no next time (Canon §11.6); in a boss Folio while the boss lives it reads "Erasure held"; once any Erratum has entered the Colophon it reads "Erasure stopped". The local hunter threshold is `thr = max(6, 10 − red_ink_doses)` (Canon §11.6). At `k ≥ thr − 2` the clock turns `hostile` and shows `thr`; after `ev.hunter_spawned` it shows the hunter glyph instead, because no second Erasure hunter spawns in that Folio. A party marked for Colophon desecration (doc 14 §3.9) shows a cracked-seal glyph beside the clock for the rest of the run.

### 3.3 HUD element record

```json
{ "id": "ui.hud.vitals", "anchor": "BL", "offset_px": [0, 0], "size_px": [320, 104], "priority": 100,
  "large": { "anchor": "BL", "offset_px": [0, 0], "size_px": [256, 80], "hides": ["composure_bar", "toxicity_bar"] },
  "visible": "always", "opacity_bp": 10000, "backplate": true }
```

### 3.4 Large layout (`access.hud_layout = large`)

The standard table fills 1280 × 800 tightly, so it never scales above `s_fit`. A larger HUD uses this second table at `s = 1.25 × s_fit`. It hides `ui.hud.minimap`, `ui.hud.body` (the body plan stays in the Character sheet's Body tab) and `ui.hud.chat` (chat lines go to the notification list), compacts the content of the listed elements, and moves the stealth meter to the right-hand stack. Base offsets and sizes are multiples of 4, so every rectangle lands on whole pixels at `s = 1.25`.

| ID | Anchor | Offset | Size | Rect at 1280 × 800, `s = 1.25` | Content in Large |
|---|---|---|---|---|---|
| `ui.hud.folio_header` | TL | (0, 0) | 256 × 48 | 24,24–344,84 | Folio number, genre sigil and name, Erasure clock; no axiom pips or Law glyphs (the Journal's Current Folio page lists the Laws) |
| `ui.hud.party` | TL | (0, 56) | 192 × 136 | 24,94–264,264 | 3 frames of 44 px: name, HP bar, status count, downed or ghost glyph |
| `ui.hud.minions` | TL | (0, 200) | 192 × 104 | 24,274–264,404 | header plus 4 rows of 20 px; a fifth minion shows as "+1" |
| `ui.hud.compass` | TC | (0, 0) | 384 × 32 | 400,24–880,64 | full |
| `ui.hud.boss_bar` | TC | (0, 40) | 416 × 36 | 380,74–900,119 | full |
| `ui.hud.invasion` | TC | (0, 84) | 384 × 28 | 400,129–880,164 | full |
| `ui.hud.ghost` | TC | (0, 128) | 384 × 28 | 400,184–880,219 | banner only; the forecast moves to the notification list |
| `ui.hud.notifications` | TR | (0, 0) | 256 × 112 | 936,24–1256,164 | 3 lines of 36 px |
| `ui.hud.crosshair` | C | (0, 0) | 64 × 64 | 600,360–680,440 | full |
| `ui.hud.prompt` | C | (0, 64) | 416 × 40 | 380,455–900,505 | full |
| `ui.hud.vote` | C | (0, −112) | 384 × 56 | 400,225–880,295 | full |
| `ui.hud.downed` | C | (0, −52) | 288 × 32 | 460,315–820,355 | full |
| `ui.hud.vitals` | BL | (0, 0) | 256 × 80 | 24,676–344,776 | §3.2 vitals block, Large row |
| `ui.hud.status` | BL | (0, −88) | 256 × 36 | 24,621–344,666 | 1 row of 7 icons (32 px, 4 px gaps); more show as "+N" |
| `ui.hud.quickbar` | BC | (0, 0) | 432 × 56 | 370,706–910,776 | full |
| `ui.hud.abilities` | BC | (0, −64) | 208 × 48 | 510,636–770,696 | 4 slots of 44 px with 10 px gaps |
| `ui.hud.subtitles` | BC | (0, −120) | 592 × 100 (fixed, §3.1) | 344,526–936,626 | §13.2 |
| `ui.hud.weapon` | BR | (0, 0) | 256 × 80 | 936,676–1256,776 | name, ammo or charges or heat, Contradiction icon; no fire mode or counter icons |
| `ui.hud.spell` | BR | (0, −88) | 256 × 40 | 936,616–1256,666 | spell name, cost, cooldown |
| `ui.hud.stealth` | BR | (0, −136) | 256 × 44 | 936,551–1256,606 | eye 44, light 136 and noise 76 reference px |

Placement and validation are pure functions over either table:

```rust
/// Pivot fractions: L and T are 0, C and M are 0.5, R and B are 1.
fn place(e: &HudRow, safe: Rect, s: f32, s_fit: f32) -> Rect {
    let (ax, ay) = safe.point(e.anchor);                                  // TL, TC, TR, ML, C, MR, BL, BC, BR of the safe rect
    let (w, h) = if e.id == "ui.hud.subtitles" { (592.0 * s_fit, 100.0 * s_fit) } else { (e.w * s, e.h * s) };
    let (px, py) = (ax + e.ox * s, ay + e.oy * s);                         // where the element's own pivot lands
    let (fx, fy) = e.anchor.pivot_fraction();
    Rect { x0: px - fx * w, y0: py - fy * h, x1: px - fx * w + w, y1: py - fy * h + h }
}
fn validate(rows: &[HudRow], safe: Rect, s: f32, s_fit: f32) -> Result<(), LayoutError> {
    let r: Vec<Rect> = rows.iter().map(|e| place(e, safe, s, s_fit)).collect();
    for i in 0..r.len() {
        if !safe.contains_with_tolerance(&r[i], 0.001) { return Err(LayoutError::OutsideSafe(rows[i].id)); }
        for j in i + 1..r.len() {
            // half-open rectangles: touching edges are allowed
            if r[i].x0 < r[j].x1 - 0.001 && r[j].x0 < r[i].x1 - 0.001 && r[i].y0 < r[j].y1 - 0.001 && r[j].y0 < r[i].y1 - 0.001 {
                return Err(LayoutError::Overlap(rows[i].id, rows[j].id));
            }
        }
    }
    Ok(())
}
```

The client runs `place` on every layout change (window resize, layout, scale step, safe area). The content loader runs `validate` for the standard table at every `access.hud_scale_bp` step and for the Large table, each with the default and the TV-safe margins, on the four logical canvases of §3.2, and refuses to load on an error, so neither a mod nor a localized layout can ship overlapping HUD elements.

### 3.5 HUD presets and opacity

`access.hud_preset` applies on top of either layout: `full` (default), `minimal` (hides the compass, party frames of allies at 100% HP, notifications, abilities with no cooldown running, and minions while all are at full HP), `cinematic` (vitals and weapon fade to 0 over 30 frames after 5 s at full resources and out of `in_combat`; any change restores them for 5 s). `access.hud_opacity_bp` 2500..=10000 (default 10000). Backplates are `#101010` at `access.hud_backplate_bp` (default 6000; `access.high_contrast` forces 10000). Because every HUD backplate is `#101010`, HUD text uses the global tokens of §2.2 (`neutral` for plain text, the semantic token of its resource, `pencil` for Narrator notes) in every skin; the skin tokens color HUD frames and ornaments only.

---
## 4. Inventory, paper doll and item card

### 4.1 The Book menu

`input.menu_book` (or a tab key) opens `ui.menu.book`, a tabbed page set: **Satchel** (inventory, §4.2), **Character** (§5), **Grimoire** (§6.5), **Retinue** (§10.3, only while the player has a minion), **Journal** (§7), **Map** (§8), **Exits** (§9.1, only inside a Colophon during the exit vote), **Settings**. `input.ui_tab_prev` / `input.ui_tab_next` switch tabs; each tab remembers its focus. Direct keys open a tab (§12.3).

### 4.2 Satchel and paper doll

Layout (1280 × 800): left column 400 px paper doll; center 560 px backpack grid; right 320 px item card of the focused item.

- **Paper doll** shows every Canon §13.2 slot around a front silhouette of the body plan: head, face, neck, back, body, hands, legs, feet, belt, ring L, ring R, trinket, weapon 1–3, offhand, ward, four implant slots, and `slot.hand_extra_1` and `slot.hand_extra_2` when the body plan has `bp.arm_extra_1` and `bp.arm_extra_2` (Canon §13.2). A non-humanoid Echo shows only the slots doc 04 §13 gives its plan: `echo.fenrir` (`body.quadruped`) holds `slot.weapon_1` in its mouth and has no offhand, hand or ring slots; `echo.jormungandr` (`body.serpentine`) has no leg or feet slots and both extra hands. An empty slot shows its glyph at `muted`. A cursed equipped item shows a chain overlay once `BUC` is known; a Disconnected item shows the torn page.
- **Backpack**: 20 slots plus 6, 10 or 14 from the `slot.back` pack (doc 11 §14.1), 64 px cells in rows of 8, so at most 5 rows. Sort buttons (presentation order only, never changes uids): by category, rarity, value, mass, newest. Filter chips: equipment, consumables, materials, unknown, cursed-known.
- **Footer**: mass `carried / capacity kg` with `load_bp` and the encumbrance band (`st.encumbered` above 10000, `st.overburdened` above 12500); Lucre in the local Lucre skin; keyring count; ammo reserve per type. The Lucre line offers **Drop Lucre**: an amount stepper over `1..=purse` that sends `DropLucre { amount }` (doc 11 §17, DECISIONS 83) and leaves one pile at the player's feet, which the dropper's own auto-pickup skips; it is greyed for a ghost or a downed Erratum. No trade and no Drop for ally carries Lucre.
- **Actions** on the focused item, each an intent: Equip / Unequip, Use, Drop (`DropItem`), Drop for ally (`DropItemFor`, co-op), Assign to quickbar (`QuickbarAssign`), Examine (`input.examine`: the doc 10 §5.6 120-tick Analysis channel, shown with its cancel reasons and greyed with "Already examined at level L" when `analysis_lvl ≥ level`), Lock appraisal focus (`SetFocusLock`), Split stack. The action list opens with `input.ui_actions` (§12.3, §12.4). Dragging onto a paper-doll slot equips; onto a quickbar slot assigns.
- **Quick compare**: holding `input.ui_compare` shows the equipped counterpart's card beside the focused one (§4.4).

### 4.3 Item card

`ui.menu.item_card` (widget `PalItemCard`, 320 × up to 640 px) renders an `ItemView` (doc 10 §2.8). Rows appear in this order; a row whose data is absent from the `ItemView` is drawn as specified in the "unknown" column, never omitted silently.

| # | Row | Shown | Format | When the data is not known |
|---|---|---|---|---|
| 1 | Frame | always | rarity color, letter and pips (§2.2); Palimpsestic animated border; Illuminated sparkle corner | rarity unknown: plain `muted` frame, letter `?` |
| 2 | Name | always | doc 10 §6.4 name line; skinned name by Canon §1.10 order; unique name at Appraised+ | unknown type: appearance descriptor rendered by the local skin ("Murky green flask") |
| 3 | Type line | always | category, archetype skin name, quality, origin genre sigil, gear tier `T1..T5`, manufacturer name and logo | unknown type: class name only ("Potion") |
| 4 | Requirements | equipment | primary requirements `MGT 55`; red ▼ and shortfall `−5 → ×0.95` (Canon §6.3) when unmet | — |
| 5 | Axioms | dependent components | one axiom icon per dependency with `Req`, local axiom, `Tol`, and the base component's Contradiction state, potency % and failure %; Anachronism Edge `+E × 4 %` vs natives | — |
| 6 | Core stats | equipment | weapon: damage per hit at the item's `src_lvl` after Canon §10.4 (display HP), type shares with glyphs, fire rate or swing ticks, magazine, precision multiplier, penetration; armor: AR per covered part; ward generator: capacity, recharge per s, depletion profile per type | a stat with an unrevealed contribution shows the revealed value and "?" (doc 10 §9) |
| 7 | Enhancement | equipment | `+n` and effective enhancement after blessing or curse | "+?" |
| 8 | Affixes | equipment | one line per affix slot: name, tier, value at its roll, glyph of its type | name unknown: "Unread property"; roll unknown: name with "(?)" |
| 9 | Perk columns | weapons | up to 5 columns: the columns its rarity rolls plus column 5 (doc 11 §7.1); chosen option highlighted, the others listed with "free to change at a Scriptorium"; enhanced magnitudes marked with a small star | rarity unknown: "Perks hidden" |
| 10 | Legendary perk / Exotic intrinsic | when present | name and full text | rarity unknown: hidden |
| 11 | Charges | wands, gadgets | `cur/max` | "?/?" |
| 12 | Curse line | non-mundane | skinned state (Blessed / — / Cursed or genre skins) and penalty | "?", "Ominous" (`OMEN`) or "Clean" (`CLEAN`) |
| 13 | Value | always | identified: exact `value` in the local Lucre skin; appraised: doc 10 §6.3 range "≈ 160–340 Crowns" in Mythic Fantasy (DECISIONS 90); sell price at the current shop | classified: "Value ?"; unknown: Ledger price group (§4.5) |
| 14 | Mass | always | kg with one decimal: `round_half_even(mass_g × stack, 100)` tenths (Canon §1.5); stacks also show `×stack` after the name | — |
| 14a | Parts and materials | assembled items; items with `materials` | one line per part slot with the part's skinned name and manufacturer logo (Canon §13.9 `parts`), then the primary material and the other materials by skinned name, with the primary material's factor ("×1.15 damage" from `dmg_bp`, or "×1.15 armor" from `ar_bp`, doc 07) | — (public, doc 10 §2.1) |
| 15 | Flavor | when present | italic flavor text; LLM-written lore is marked with a small quill glyph (doc 02, Brief §5) | — |
| 16 | Footer | always | `ilvl`, uid tail (last 4 hex digits, for co-op talk), finder slot color in co-op, "Unread Edge +4 SER" when it applies (doc 10 §9), sealed flag | — |

Kin-specific lines (§10.5) append under Core stats, for example "Construct: potion healing ×0.50". The card reads only `ItemView` (doc 10 §2.8) plus the doc 11 instance fields it lacks today (perk columns, parts, stack, value, `illuminated`, `finder_slot`; Interface assumptions, doc 10 and doc 11). Gear never wears or breaks (Canon §13.9), so the card has no durability row.

### 4.4 Comparisons

- The **counterpart** is the item in the slot the focused item would occupy: the same armor slot; for rings, both rings side by side; for weapons, the active weapon slot, then the weapon in the slot the equip action would replace.
- Each comparable numeric row shows `focused − counterpart` with ▲ (better) in `stamina` color or ▼ (worse) in `hp` color. "Better" is per row: higher damage, AR, capacity, magazine, crit, lower swing ticks, mass, spread.
- When either side has an unrevealed contribution to a row, the delta shows "?" instead of a number. When the focused item would be Disconnected or Contradicted at the current position, the delta uses its potency-scaled value and the row carries the Contradiction icon.
- Damage comparisons use the same pure function the simulation uses for step 0 (Canon §10.4) at the item's own `src_lvl`, so a deeper item compares higher only through its real numbers.

### 4.5 Ledger panel

The Ledger (doc 10 §5.8) is a Journal page and an item-card expander for every unknown descriptor. Each descriptor row: appearance swatch, times seen, price observations (newest 8), and the `candidates(d)` list with each candidate's base value; when one candidate remains, the row reads "Deduced: <type>" in `accent` until the type is identified by an in-game event. The player can annotate any descriptor with a free-text note of up to 40 characters (client-local, per run).

---

## 5. Character

### 5.1 Character sheet (`ui.menu.character`)

The Character sheet is the Book menu's Character tab (§4.1); `input.menu_character` opens it directly (§12.3). Tabs: **Attributes**, **Skills**, **Talents**, **Body**, **Afflictions** (§10.6), **Kin and Origin** (§10.5). The header shows name, kin, Origin (with Going Native state), Calling, level, XP `cur / next` with a bar, unspent attribute points, unspent talent points, and Scribe's Mercy when on.

### 5.2 Attributes and allocation

- An 8 × 3 grid: each primary shows `floor_div(s1 + s2 + s3, 3)`; each sub-attribute shows effective value, base value in `muted` when they differ, and training marks `marks / T(v)` with `T(v) = 2 + floor_div(v, 20)` (Canon §7.1).
- **Allocation.** Pending batches (doc 04 §6.4) are listed oldest first: "Level 7: 4 points, at most 2 per sub-attribute". Each `+` press sends `AllocatePoint { sub }`; the button is disabled with a reason line when the oldest batch with points left already gave 2 to that sub-attribute or the base is 100.
- **Preview.** Focusing a `+` shows every derived row (Canon §6.4) whose value would change, as `old → new`.

### 5.3 Skills

A list of the 36 skills grouped as Canon §7: value, mastery tier mark at 25 / 50 / 75 / 100, governing sub-attribute code, XP progress to the next level as `progress / need(s)` with `need(s) = 4000 + 400 × s` milli-XP shown in display XP with one decimal (example: skill 34 needs 17.6 XP), the Skill XP multiplier (row 28) and the next mastery perk's text (doc 04 §5.3).

### 5.4 Talents

- One tree per Calling plus the general tree (doc 04 §7). The own Calling tree is a 4-tier page; the general tree is a facing page.
- Node states: owned (filled), available (outlined in `accent`), gated (dashed, with the failing gates listed from doc 04 §7.1: "needs 2 points in this tree" for tier 2; "needs 5 points in this tree", "needs level 12", "needs FIN 45" for tier 3; "needs 8 points", "needs level 20", "needs FIN 50", "needs Unyielding" for tier 4), unavailable (other Callings' trees are not shown except through Interpolation offers).
- **Interpolation** offers (doc 04 §6.4, every 10th level) appear as a 3-card modal: the first 3 talents of the stored `rng.progress` shuffle, each with its home tree and full text, plus Decline. The stored offer re-opens unchanged until taken or declined; taking one sends `PickTalent { talent }`, which doc 04 resolves against the stored offer.
- Buying sends `PickTalent { talent }`.

### 5.5 Ability slots

Active talents bind to `input.ability_1..=4` (doc 04 §1.1). The Talents tab shows 4 slot sockets; dragging an owned active talent into a socket sends `AssignAbilitySlot { slot, talent }` (Interface assumptions). A socket shows the talent glyph, its cooldown in seconds and its cost.

### 5.6 Formula breakdown tooltips

Every derived number in every menu and in the HUD vitals is a **breakdown target**. Holding `input.ui_details` (mouse hover for 300 ms also works) opens `ui.menu.breakdown`:

1. The formula as written in its owning section, with the section reference ("Canon §6.4 row 13").
2. The same formula with every input substituted, then each intermediate result in evaluation order, honoring Canon §1.3 rounding and chain order.
3. Every modifier by source: `RowFlat` and `RowPost` (doc 04 §1.2) with the source item, talent, trait or wound, potency-scaled per component; status multipliers in ascending status ID order (Canon §11.7).
4. The final value in display units (Canon §1.5).

The data come from a pure simulation function `explain_row(entity, row) -> Explanation` (Interface assumptions, doc 04), so the tooltip can never disagree with the simulation. Worked examples (all sub-attributes not named are 40):

| Row | Inputs | Breakdown shown | Display |
|---|---|---|---|
| 1 Max HP | TGH 52, L 7, no gear | `40 + 52 + 7 × (3 + floor_div(52, 20) = 2) = 127` | 127 HP |
| 2 HP regeneration | the same, RES 46 | `max(10, 50 + floor_div(6, 2)) = 53 Bp`; `mul_bp(127,000, 53) = 673 mHP/s` | 0.6 HP/s |
| 4, 5 Stamina | END 48, L 7 | `50 + 48 + floor_div(7, 2) = 101`; regeneration `15 + floor_div(48, 5) = 24` | 101; 24/s |
| 13 Crit chance | FAT 50, PRC 55, VERVE 7, gear +300 | `500 + 10 × 10 + 15 × 5 + 2 × 100 + 300 = 1175`; clamp 0..=7500 | 11.8% |
| 17 Reload speed | SLT 58, base 120 ticks | `clamp(10000 + 18 × 50, 7000, 15000) = 10900`; `max(1, ceil_div(120 × 10000, 10900)) = 111` ticks | ×1.09, 1.85 s |
| 26, 27 Budget, slots | LOG 50, MEM 45 | `4 + floor_div(50, 10) = 9`; `2 + floor_div(45, 20) = 4` | 9; 4 |
| 43 Death's Door | PRV 60, VERVE 7 | `clamp(1000 + 20 × 50, 0, 4000) = 2000`; `mul_bp(2000, 10000 + 2 × 1000) = 2400`; `min(6000, ·)` | 24.0% |

The 11.8% follows Canon §1.5: `round_half_even(1175, 10) = 118`, shown with one decimal. Per-second `Milli` rates below 10 display units show one decimal as `floor(x / 100)` tenths (Canon §1.5): 673 mHP/s shows 0.6.

### 5.7 Character creation and level-up (`ui.menu.creation`)

The screen walks doc 04 §2.1's `CreationState`, one page per state, with Back on `input.ui_back` and choices kept while still valid:

| State | Page |
|---|---|
| `ChoosePath` | Custom, Quick start (Custom with the Calling preset filled in PointBuy) or Echo (owned Echoes only) |
| `Kin` | the 6 kin with their doc 04 §2.4 traits and the kin skin name for each Origin |
| `Origin` | unlocked Origins with the §5.4 tolerance row, sub-attribute modifiers, Origin skill and trait |
| `Calling` | unlocked Callings with major and minor skills, kit and talent tree preview |
| `PointBuy` | the 24 sub-attributes with doc 04 §2.2's point costs and remaining points, and every changed Canon §6.4 row as `old → new` (§5.2 preview) |
| `PickEcho` | each owned Echo with its fixed kin, Origin, Calling, allocation and signature trait |
| `Cosmetics` | appearance: one of the chosen kin's 12 presets with palette and marking colors, and no slider editor (DECISIONS 97; docs 04 and 18 define the presets); owned cosmetics; nothing here changes a number |
| `Confirm` | the summary; a failed doc 04 §2.7 validation keeps the state and names the failing rule |

Level-up (`ev.level_gained`) never opens a modal: it raises a notification ("Level 7: 4 points to place") and badges the Character tab; batches stay pending until spent (doc 04 §6.4). A mid-run joiner (doc 04 §6.6) gets a join page with every pending batch, talent point and Interpolation offer before the first Folio tick.

---
## 6. Making: crafting, placement, alchemy, spellcraft

### 6.1 Crafting (`ui.menu.craft`)

Crafting opens at a `station.*` within `CRAFT_REACH_MM`, or from the Satchel for `station.hand` recipes (3 × 3 grid, quality cap Normal, doc 07). Doc 07 owns recipes. Five modes, one tab each, cover doc 07 §6.1's recipe kinds; every commit is doc 02's `Craft` intent carrying doc 07's `StartCraft { recipe, count }` payload:

| Mode | Recipe kinds | Layout |
|---|---|---|
| Grid | `recipe.kind.shaped`, `recipe.kind.shapeless` | a 3 × 3 grid at every station, `station.hand` included (doc 07); drag materials in; the matching recipe (doc 07 §6.3, horizontal mirror included) and its output card show live on the right, with the count stepper up to the stack limit |
| Assembly | `recipe.kind.assembly` | the frame item centered with one socket per part slot (`parts`, Canon §13.9); compatible loose `cat.part` items listed per socket; the assembled item card updates live |
| Processing | `recipe.kind.process` | the station queue of up to `STATION_QUEUE_MAX` = 4 jobs (doc 07 §5.3): input, output, `ticks_each`, fuel left and a progress bar per job; jobs run in simulation time; the tab states "Queued jobs and uncollected outputs are lost when the party leaves the Folio" |
| Disassembly | `recipe.kind.disassembly` | the item to break down and every material, part and graft it yields (doc 07 §8.3) |
| Materials | none | every carried material with its doc 07 properties (density, hardness, `dmg_bp`, `ar_bp`, `conceal`, flammability) and the recipes that take it |

**Channel.** While doc 07 §6.4's channel runs, the prompt shows a ring of `progress / needed` and the cancel reasons (moving more than 1000 mm, non-DoT damage, a control status, the station breaking, `input.ui_back`); a cancel consumes nothing. Failing checks are listed before the commit with doc 07's reason text.

**Exact quality odds.** Every output that rolls quality (doc 07 §4.1) shows the probability of each tier. The client evaluates doc 07 §4.2 over every noise outcome instead of drawing: `noise = a + b + c − 210` with `a, b, c ∈ 0..=140`, so the 141³ = 2,803,221 outcomes collapse into 421 sums with counts `cnt[s]`, computed once by convolution. Each sum's score maps to a tier, then the skill and station caps, the Smithing-50 step and the `masterwork` talent apply exactly as in doc 07:

```rust
fn craft_odds(center: i32, cap: u8, smith50_step: bool, masterwork: bool) -> [u64; 7] {
    let mut two = [0u64; 281];                                   // counts of a + b
    for a in 0..=140 { for b in 0..=140 { two[a + b] += 1; } }
    let mut cnt = [0u64; 421];                                   // counts of a + b + c; sum 2,803,221
    for s2 in 0..=280 { for c in 0..=140 { cnt[s2 + c] += two[s2]; } }
    let mut out = [0u64; 7];                                     // q.awful ..= q.peerless
    for s in 0..=420 {
        let score = center + s as i32 - 210;
        let mut q = Q_THRESH.iter().filter(|t| score >= **t).count() as u8;   // doc 07 §4.2 thresholds
        q = q.min(cap);                                          // min(skill_cap, station_cap)
        if smith50_step && q == 2 { q = 3; }                     // doc 04 Smithing 50, weapons and armor only
        if masterwork { q = (q + 1).min(6); }
        out[q as usize] += cnt[s];
    }
    out                                                          // each shown as pct4(out[q], 2_803_221)
}
```

Example (doc 07 §4.3, second row): skill 20, tier-1 station, tier-1 toolkit, no material term, no helpers, attribute term 0, center 250, cap Good: Awful 1.3492%, Poor 23.3200%, Normal 50.2190%, Good 25.1119%. The `rng.craft` draws happen only when the channel completes (Canon §3.3), so re-opening shows the same odds and never a rolled result.

### 6.2 Placement mode (`ui.hud.placement`)

Using a `cat.placeable` enters placement mode (Canon §4.3): a translucent ghost of the placeable snaps to the aimed anchor; it is tinted `ally` when valid and `hostile` with a reason line when invalid ("needs a solid anchor below", "inside an entity", "inside a Colophon"). Rotate with `input.weapon_next` / `input.weapon_prev` in 16384-Bam steps. The prompt shows the placement ticks (scaled by `skill.fieldcraft`, doc 07) and the block HP. `input.attack` confirms; `input.alt` cancels. A spawn-light preview tints the cells a placed light source raises above the genre's `spawn_block_max` (Canon §12.3).

### 6.3 Alchemy (`ui.menu.alchemy`)

Layout: ingredient tray (left), apparatus row (top), brew sheet (center), result preview (right).

- **Inputs** (doc 08 §9.3): 2 to 4 effect ingredients (1 at Alchemy 100), 0 to 3 modifier reagents with their exclusion pairs greyed (Extend/Amplify, Aerosol/Linger, at most one Invert), a base liquid, a vessel kind and a dose count up to `batch`.
- **Preview**: the shared effects by name for effects already revealed on every contributing ingredient, then "N unknown" for shared effects not yet revealed (doc 08 §8.5); polarity of the primary; for each named effect, magnitude and duration computed through the doc 08 §9.6 chain at both ends of the quality range. The quality range is the tier of `q0 − 500` to the tier of `q0 + 500`, with `q0 = mul_bp(skill_bp, mortar_bp)` computed without `v`, because `v ∈ −500..=500` is the only variance (doc 08 §9.5). The odds are exact: each of the 1,001 values of `v` is equally likely, so a tier shows `pct4(#{v : band(q0 + v) = tier}, 1001)`. Example (doc 08 §9.10, the apprentice healing brew): `q0 = 8685` gives Poor 81.4186% (815 values, `v ≤ 314`) and Normal 18.5814% (186 values).
- **Toxicity per dose** as stored on the potion (doc 08 §9.6), then the drinker's factors of doc 08 §11.1 (`iron_stomach`, the Alchemy 100 perk, kin 5000 for constructs and undead), and a projected toxicity bar showing where one dose would put the brewer on the band scale.
- **Brewing** shows the 120- or 150-tick progress (after row 17); any cancel condition (damage, moving 1000 mm, removing an input) shows the reason and consumes nothing.

**Toxicity meter** (HUD vitals): 256 × 6 px bar of `res.toxicity / 100,000`, ticks at 50,000, 75,000 and 100,000, band label on change: clean, queasy, sickened, overdosed (doc 08 §11.2), each with its glyph. Example (doc 08 §11.3): each Normal healing dose applies 12,000, so four doses put the projected mark at 48,000, still clean (below 50,000); a fifth would read 60,000, "queasy".

**Respirator gauge**: while a respirator is worn, a 48 × 6 px gauge in the vitals glyph row (§3.2) shows `filter / max_filter` (1800, 3600 or 7200 ticks, doc 08 §4.3), turning `hp` with the label "Filter low" at 600 ticks or fewer; at 0 it shows "Filter spent: refill with charcoal" (`ing.charcoal`, 60 ticks).

### 6.4 Spellcraft composer (`ui.menu.composer`)

The composer is a glyph-grammar editor over doc 06 §4. It calls the pure `compile(refs, Some(caster))` (doc 06 §4.5) on every edit, so its numbers are the simulation's numbers.

- **Palette**: the caster's Lexicon (doc 06 §8.1) grouped by class (Form, Essence, Modifier, Trigger, Condition); each glyph shows its skinned name in the local genre, origin sigil, tier, complexity `cx`, cost and dependency axis with the local Contradiction state of that glyph.
- **Stage editor**: one row per stage (root, payload depth 1, payload depth 2), with sockets in grammar order: form, 1–2 essences, 0–3 modifiers, trigger, condition. Invalid drops are refused with the error the compiler would give.
- **Live panel**, updated on every edit:

| Field | Source | W1 Searing Bolt for Ref-A (doc 06 §6.7) |
|---|---|---|
| Canonical text and `spell_hash` | doc 06 §4.2 | `glyph.form.bolt@mythic_fantasy glyph.ess.flame@mythic_fantasy`, `0x9d32104fc5e81a6e` |
| Name preview | doc 06 §17 in the local genre | "Flame Bolt" |
| Complexity / budget | doc 06 §4.4; row 26 | 2 / 9 |
| Base cost | doc 06 §6.1 | 5,000 m-aether |
| Cast cost chain | doc 06 §6.2: skill factor, implement, rider, thrift | `cost_skill = floor_div(90 + 24, 4) = 28` → ×11100 = 5,550 → ×9500 = 5,272 → ×9500 = 5,008 → display **5 aether** |
| Cast ticks, recovery, cooldown, cast type | doc 06 §3.3, §3.4 | 24, 20, 0, cast_time |
| Potency | doc 06 §6.3 | 11400 → 12084 → 12809 (×1.28) |
| Damage per hit (step 0 and 2.1) | doc 06 §6.4 at the implement `src_lvl` | `mul_bp(24000, 11881) = 28,514` → ×12809 = 36,523 mHP → **36** |
| Status power per hit | doc 06 §6.4, with the Evocation 25 hook | burning 64 |
| Contradiction forecast | Canon §5.5 at the caster's position | Coherent, potency 100%, failure 0% |
| Error | doc 06 §4.3 first failing check | none |

- Errors show the code and a sentence: `E_COMPLEXITY` "Complexity 11 exceeds your budget 9 (LOG 50)". A greyed Grimoire slot (doc 06 §4.5) shows the same error.
- **Commit**: "Memorize into slot k" sends `ComposeSpell { slot, refs }`; doc 06 runs the 120-tick memorize channel (not `in_combat`). "Inscribe" at a station sends the doc 06 §8.3 inscription request with its aether cost shown.

### 6.5 Grimoire and quick-cast wheel

- **Grimoire tab**: `2 + floor_div(MEM, 20)` slots (row 27); locked slots above the count keep their contents and show a lock with "Needs MEM 60" for the next slot. Each slot: name, school, cost, cast type, cooldown, a "Clear" action (instant).
- **Wheel editor**: 8 sockets; each references a Grimoire slot, tome spell, carried text, carried wand, on-use enchantment or prayer (doc 06 §3.2). Assigning sends `AssignWheelEntry { index, source }`.
- **Wheel geometry.** Every wheel (`ui.hud.cast_wheel`, `ui.hud.quick_wheel`, `ui.hud.weapon_wheel`, `ui.hud.emote_wheel`, `ui.hud.minion_wheel`) draws 8 sockets of 64 px on a circle of radius 140 px around the crosshair center, socket 0 at the top, clockwise. A selection exists once the stick deflection exceeds 3000 Bp of full range or the mouse has moved more than 40 px from where the wheel opened; then `a` is the clockwise angle in Bam from screen-up to the deflection vector (`rad_to_bam`, doc 02 §7.5) and the socket is `((a + 4096) mod 65536) / 8192`. Example: a stick pushed down-right (135°, `a = 24576`) selects socket `28672 / 8192 = 3`. Releasing without a selection commits nothing. `access.wheel_dwell` (default off) commits a selection held for 30 frames without releasing. No wheel pauses or slows the simulation (doc 06 §3.2).
- **Cast wheel** (`input.cast_wheel`): releasing with a selection encodes `WHEEL_SELECT` (§12.2). An entry whose spell has `glyph.ess.appraise` or `glyph.ess.psychometry` with the `self` form (doc 06 §5.3) opens a target list of eligible inventory items instead of casting; with the `touch` form the spell takes the ground item nearest the aim ray at cast start, and `glyph.ess.omen` acts on an area with no item target, so neither opens a list; choosing one sends `SetCastItemTarget { uid }` and the client sets `CAST` on the first command whose tick is at or after the intent's acknowledgement in `OwnerDelta` (Interface assumptions, doc 06).
- **Quick wheel**: socket `k` is quickbar slot `k + 1` (slots beyond the belt's size are empty); releasing sets `QUICKBAR_INDEX` without using it. **Weapon wheel**: sockets 0, 2 and 6 hold weapon slots 1, 2 and 3; releasing sets `WEAPON_SELECT`. **Emote wheel**: the 8 emotes equipped in the Wardrobe, with `input.whistle` in socket 0 and `input.shout` in socket 1 by default (§12.1). **Minion wheel**: §10.3.

---

## 7. Journal and Index knowledge (`ui.menu.journal`)

| Page | Content |
|---|---|
| Run log | Folio history (index, genre, depth, Rubric type, completion tick, deaths, notes completed, and the Folio quest as complete, story solved only, Leaf carried only, or lapsed), the lattice path drawn as a ribbon of genre sigils (a fused Second Edition Folio as its two sigils split diagonally, §11.10) |
| Current Folio | Rubric and its progress (Keeper location once seen, catchwords 0/3, Seal held ticks `/1800`); directly under it the optional Folio quest (below); Marginal notes with live completion state (doc 14 §3.5), the Erasure timeline: stack `k` at `c = 43200 + 1800 × (k − 1)` for `k` = 1..=10 (§3.2), the hunter threshold `max(6, 10 − red_ink_doses)` and whether this Folio's hunter has spawned; the discovered World Laws active here |
| Ledger | §4.5 |
| Index | the account's item encyclopedia (doc 14 §5.5, doc 10 §12; "Codex" names only the pass, triage G-197): templates seen and identified, familiarity rank, Illuminated skins owned |
| Bestiary | per enemy template (doc 14 §5.5): entry, art and lore from the first kill; resistances and immunities from the 10th kill, also shown on that template's nameplates in every run (§3.2); drop categories from the 25th kill; ranks met and champion aspects seen |
| Glossary | every term of Canon §20 plus every status, axiom, rank and currency, each with its formula links |
| World Laws | every Law the viewer has discovered (rule below), grouped by genre, with its kind and exact effect (doc 12 §4); Laws active at the player's position are marked |
| Lore | flavor texts collected, Folio intro texts, every Narrator note shown this run (§16.4) |

Every Journal entry the player has not opened carries a dot; "Mark all read" is one input.

**World Law discovery** (Canon §5.8, DECISIONS 56, doc 12 §4.1). A viewer has discovered a Law when it was among the account's discovered Laws at run start (doc 14 §5.5 keeps them with the Index) or when the run's `laws_triggered` slot mask for that Law holds the viewer's slot. A Law the viewer has not discovered appears nowhere: not on door panels (§9.1), the Folio header (§3.2), the Current Folio page or the World Laws page, and no placeholder stands in for it. On `ev.law_discovered` (doc 03, payload `{ law, slots }`) whose `slots` holds the local slot, for a Law the account had not discovered at run start, the notification list shows "World Law discovered: <skinned name>" with the Law's exact effect text, and from that tick every preview and page of the run shows the Law. At run end, and when the player departs, doc 14 §5.5 adds the Law to the account's set, so later runs show it too. The mask holds only slots with an Active Erratum on the trigger tick, so a ghost discovers nothing (doc 12 §4.1). In co-op each client shows its own viewer's Laws.

**Folio quest** (DECISIONS 72, doc 12 §20.6). From Folio entry the Current Folio page lists the quest directly under the Rubric, with each step's state: **Solve the story** reads `read / need` distinct story events read (`need` is 3, or 2 under `law.cosmic_horror.forbidden_knowledge`; bridges and the plaque's event 0 count, doc 12 §20.4) and then "Solved"; **Carry the Folio Leaf out** reads "Leaf not taken", "Leaf held by <persona>" while an Active Erratum carries `item.quest.folio_leaf`, or "Leaf on the ground", and becomes "Carried out" on the exit-commit tick (doc 12 §20.5). The page states that the quest is optional, gates nothing and has no timer; a step left unfinished when the party leaves lapses, and the Run log records the outcome. Solving raises the notification "Folio story solved" on `ev.objective_completed` kind 2.

---

## 8. Folio map (`ui.menu.map`, `ui.hud.minimap`)

- The map is a client-side 3D cutaway of the Folio's voxel data, built from chunks the client holds and limited to **known cells**: every cell of a room the party has visited (doc 14) and every cell the local player has seen. Seen cells: every 6 ticks the client casts 1,152 rays from the local eye (128 yaw directions 512 Bam apart × 9 pitch rows at −8000, −6000, …, 8000 Bam) with doc 02's voxel `dda` out to 24,000 mm, marking every crossed cell and the first opaque cell as seen. Seen cells are per client, are never sent over the network, and are cleared at `ev.folio_exited`. `run.boon.surveyor` adds every room outline and the Colophon (doc 14 §3.9); reading `item.text.mapping` adds every room outline, door and exit for the whole party, secrets excluded (doc 06 §8.3), through the host's `RunState`.
- Controls: orbit with the right stick or right mouse drag (yaw 8192 Bam per 100 px; pitch 4096 Bam per 100 px of vertical drag, clamped to −15000..=−2000 Bam so the camera always looks down), zoom 4 levels (8, 16, 32, 64 px per block), and a **Y-slice** that hides blocks above the chosen height (`input.ui_page_prev` / `input.ui_page_next` move it by 1 block; default the player's eye height). Toggle "Flat" shows a top-down slice.
- Markers: the party (ally circles with slot colors), ghosts, Remains, minions, pings, the Incipit, the Colophon, exits once drawn, Scriptoriums, shops, Reliquaries and Fortune Engines seen, locked doors, Rubric targets, bleed zones (hatched in the bleed genre's accent) and the seam zone.
- **Map ping**: selecting a cell and pressing `input.ui_confirm` sends `MapPing { cell }` (Interface assumptions); it appears for the party as a world ping.
- **Local pins**: up to 16 player pins per Folio, client-only, with 8 glyph choices.
- **Minimap** (off by default): 176 × 176 px top-down slice at zoom 16 px per block, north-up or heading-up (`access.minimap_rotate`, default heading-up).

---

## 9. Run-flow screens

### 9.1 Exit choice

Exit doors are world objects in the Colophon with a floating **door panel** (`ui.hud.door_panel`, 320 × 220 px, projected above each lit door) carrying doc 14 §3.6 item 4 and every doc 12 §5.7 field: genre art and title, Lucre and Aether skins, the four axioms with ▲/▼ deltas, the active World Laws the viewer's account has discovered (§7; undiscovered Laws are not drawn), the viewer's Contradiction forecast "N Strained, M Contradicted, K Disconnected", Folio type and challenge icon, champion aspects with counter icons, bleed sigils, Marginalia modifiers, depth, hazard family icons, boss crown. Binding doors carry a seal glyph and "Binding: the Editor's genre". A secret door shows "?". "Close the Book" doors show the run-end summary preview. The "Second Edition" door of `VictoryChoice` shows the stakes of §11.10 before it can be committed.

- **Party forecast**: in co-op each door panel also shows, per party member, a single Contradiction icon for that member's worst equipped component (the forecast is computed per viewer and replicated as icons only).
- **Voting HUD** (`ui.hud.vote`): per door, `voters / need`, a hold ring `hold / 120` and, while one voter holds interact on that door with every living Erratum inside the Colophon, a second ring `interact_hold / 180` (doc 14 §3.6 has both commit paths). `need` follows doc 14 §3.6 (`n`, then `n/2 + 1` from 3,600 ticks, then 1 from 7,200 ticks), and the panel names the next threshold time. When two doors would commit on the same tick the lowest door index wins, and the panel says so.
- **Vote from list** (accessibility): while inside the Colophon, `input.menu_exits` (keyboard) or the Book menu's Exits tab (every device; the tab exists only inside a Colophon during `ExitVote` or `VictoryChoice`) lists the doors with their panels; choosing one sends `ChooseExit { door }` (doc 14 §3.6), which counts as standing in that door while the voter stays inside the Colophon.

### 9.2 Shop (`ui.menu.shop`)

Tabs **Buy**, **Sell**, **Buyback**, **Services**. Vendor name and portrait from the genre table (doc 11 §12.1). `genre.unwritten` has no shops (Canon §5.2, §17.1), so no shop marker or Shop screen appears there.

- Each line: item card (shop stock is `know.identified`), price in the local Lucre skin, the counterpart comparison (§4.4), and stock count with "restocks every 60 s" for restockable lines.
- Price breakdown (§5.6) follows doc 11 §12.2: the trade factor `10000 → × markup_bp → × row 37 → × bearer modifiers → × reputation_bp → × gp.shop_price_bp → × (10000 − haggle_bp)`, raised to `BUY_TRADE_FLOOR` 7500 when below it, then `max(1, mul_bp(value_vendor, buy_trade))` per unit.
- **Haggle** (once per player, shop and Folio, via `ShopHaggle { shop }`, doc 11 §12.2): the button previews the deterministic outcome before it is pressed: `score = Speech + floor_div(CHM − 40, 2) − (20 + min(60, d))`, then "−19.0%" for `haggle_bp = 1900` (Speech 40, CHM 70, Folio 7, doc 11 §12.2), "no change" for −20..=−1, or "+5.0% (the vendor is offended)" below −20.
- **Sell** shows the sell price with its halvings named ("unappraised ×0.5", "known cursed ×0.5").
- **Services** list each Scriptorium service with its price formula evaluated: doc 10 §5.9's appraise, classify, identify and unbind (`appr.svc.*`); wand recharge, affliction cure, reattachment, Emend the Flesh, glyph training, Rewrite, reforge, perk choice and tuning (doc 11 §12.3; gear never wears, so there is no repair service, Canon §13.9); doc 04 §6.5's talent and attribute respec, for example talent respec at Folio 10 `mul_bp(200 × (n + 1), 21718)` = 434 Lucre, then 868. Every price shows its `gp.shop_price_bp` factor.

### 9.3 Reliquary opening (in-run)

Focusing a Reliquary shows `ui.menu.reliquary_odds` before any input: tier (`reliq.plain`, `reliq.bound`, `reliq.gilded`, `reliq.illumined`, doc 11 §10), key required (a higher key opens a lower tier; held key count), contents lines, and **per equipment line** the rarity odds from doc 11 §2.3 with the opener's SER, at 4 decimals. Example (doc 11 §10): a plain Reliquary's equipment line at Folio 7, SER 40: Uncommon 68.2854%, Rare 22.2278%, Epic 6.4581%, Legendary 2.2028%, Exotic 0.8260%. Bad-luck lines: "Epic or better within `5 − rq5`", "Legendary or better within `20 − rq20`". A detected mimic shows its tell glyph and "Mimic". An undetected Reliquary states its tier's mimic placement rate: plain 4.0000%, bound 3.0000%, gilded 2.0000%, illumined 0.0000% (doc 11 §10).

Opening is the 90-tick interact (hold ring on the prompt). On `ev.container_opened` the lid animation plays for 45 frames (skippable with any input) and the instanced contents appear as cards that the player takes one by one or with "Take all".

### 9.4 Fortune Engine (`ui.menu.engine`)

A brass crank and a capsule chute; no reels, symbols, lights, near-miss frames, win or lose words, or coin audio (doc 11 §11.2). The panel shows the 6 face item cards (appraised), each face's weight and probability at 4 decimals, the price of the next capsule `mul_bp(mul_bp(mul_bp(base, P(d)), 10000 + 2500 × k), gp.shop_price_bp)`, capsules left `5 − spins`, and the pity line "Top capsule guaranteed after `need − pity` more". When the guarantee is armed the panel shows the top face at 100.0000% and the others at 0.0000%. **Cursed capsule** (DECISIONS 60, doc 11 §11.2): face 4 is always cursed and carries a pact; its card shows the curse line "Ominous" (§4.3 row 12, `OMEN`) before purchase, which pact it carries stays hidden until appraised (doc 10 §7.8), and the panel shows the cursed share `w[4] / Σ w` of the displayed weights with `pct4`: 9.0000% at brass, 10.0000% at silver, its variants and gilt, 0.0000% while the guarantee is armed. Example (doc 11 §11.2): SER 70 at a brass Engine at Folio 7 shows 23.0000%, 26.0000%, 18.0000%, 18.0000%, 9.0000%, 6.0000%, a cursed share of 9.0000% and prices 100, 125, 150, 175, 200. Buying sends `ShopBuy`; the 30-tick dispense plays the capsule dropping and opening.

### 9.5 Colophon services, boon altar and Volume reward

- **Hearth**: one prompt, "Restore (once per Colophon)".
- **Boon altar** (`ui.menu.boon_altar`, doc 14 §3.9): 3 boon cards from the stored offer, a Reroll button showing "1 reroll left", and Commit. The cards state each boon's exact effect and that it lasts one Folio. Reopening shows the same stored offer. The Reroll button states its odds: a reroll draws 3 of the 12 boons uniformly without replacement, so each boon, including the three on screen, appears on the new offer with probability 25.0000% (`pct4(3, 12)`).
- **Interleaf Rest** (doc 14 §3.9): a 600-tick channel (`REST_CHANNEL_TICKS`) whose prompt lists its cancel reasons (damage, attacking, casting, moving 500 mm) and the heal it will give, `mul_bp(mul_bp(max_hp_mhp, INTERLEAF_HEAL_BP), gp.rest_heal_bp)` through Canon §10.9, in display HP.
- **Going Native** (Canon §5.11): a confirmation page that lists the new Origin tolerance row next to the current one and the native cosmetic form; it states "Once per run. Gear is not translated."
- **Volume reward** (Endpaper): 3 item cards from the stored offer; commit one; the page states "If you choose nothing, the first card is taken in M:SS", counting down `VOLUME_REWARD_TIMEOUT_TICKS` (3,600 ticks) from the Endpaper's start (doc 14 §3.9). P9 does not apply because this in-run countdown sells nothing.

### 9.6 Trade (`ui.menu.trade`)

Two columns (yours, theirs), up to 8 items each and never Lucre (doc 11 §17), the doc 16 §9.7 state shown as a banner ("Proposed", "Open", "You locked", "Both locked: confirm"), Lock and Confirm buttons, and the eligibility line ("Too far: 5.2 m of 4.0 m", distances by P11) when a condition fails. Changing either side clears both locks, and the banner says so.

### 9.7 Run summary (`ui.menu.run_summary`)

Shown on `ev.run_ended`, and previewed on a "Close the Book" door. After an Ur-Text victory whose ending doc 18 §8.5a lists, the **closing manuscript** (DECISIONS 46; inputs doc 14 §4.7) plays first, before the summary, as illustrated pages in the Ur-Text skin: `input.ui_page_next` and `input.ui_page_prev` turn pages and `input.ui_back` closes it. It is text and pictures only, grants nothing, and re-opens from the run's entry at the Ledger (`margin.ledger`), rebuilt from the run summary and history each time. After a Second Edition victory the manuscript adds doc 18's second ending layer (doc 14 §4.7). Every other end, a forfeited Second Edition included, shows only its closing line and plates. The summary lists:

- the end state (doc 14 §4.3), Folios cleared, deepest depth, run time, the lattice ribbon of genre sigils, and for a Second Edition run its outcome line (§11.10);
- the score, line by line from doc 14 §4.2 with each accumulator;
- the account payout: Quires (doc 14 §4.6), Ink lines (doc 15 §2.2: active minutes, Folio exits, Editors, run end or victory, then the per-run and rolling caps), Codex XP lines (doc 15 §6.2) with the rested reserve bonus, and the account Reliquaries recorded;
- ranked status: "Ranked: submitted" or one line per reason it is not (Scribe's Mercy was on; game speed was below 100%; an easing mutator; Apocrypha; the mode is unranked; doc 14 §9.4);
- per player: deaths, downs, kills, damage dealt and taken, and the Mercy percent at the end when it was on;
- buttons: Continue to the Margin, Save replay, and View verification for ranked runs (doc 16 §10.2).

No timer and no offer appears on this screen (P9).

### 9.8 Replays and killcam (`ui.menu.replay`, `ui.hud.killcam`; DECISIONS 48)

- **Killcam** (`ui.hud.killcam`). The client keeps the last 300 `ViewFrame`s it rendered (5 s at 60 Hz) in a ring buffer; the buffer is presentation only, never sent and never saved. When the local Erratum ghosts (`ev.player_ghosted`) or dies at run end, the prompt "Watch how it happened" shows for 180 ticks: `input.interact` plays the killcam, and any other input or the timeout skips it. Playback shows the buffered frames up to and including the death tick at 1×, from a third-person camera orbiting at 3,000 mm around the killer: the `killer` of `ev.entity_killed`, else the `source` of the lethal `ev.damage_dealt`, else the victim (a fall or a bleed-out). The killer is outlined in `hostile` with its nameplate, the victim in `ally`, and the lethal hit's damage number shows its type glyphs. `input.ui_back` ends it. The killcam never pauses the simulation and shows nothing the client did not render; in co-op the ghost layout (§10.4) starts when it ends, and the ghost body receives no presses meanwhile; in solo the run summary (§9.7) follows it.
- **Replay viewer** (`ui.menu.replay`). It opens a `replays/<run_id>.palr` file (doc 16 §10.1) from the Margin's Ledger facility (`margin.ledger`, doc 14 §5.1) and re-simulates the file's command and intent log from tick 0 (Canon §2.4). A `sim_version` or `content_hash` this build cannot load refuses the file with "This replay needs another game version"; a `presentation_hash` mismatch alone shows "Recorded with different cosmetic content" and plays. A `checkpoints` hash that differs from the re-simulation stops playback with "Replay diverged at tick N". Controls: play and pause; speed ×0.25, ×0.5, ×1, ×2, ×4 or ×8 of 60 ticks per second; seek to any Folio entry from `folio_states`, and to any tick by re-simulating from the nearest earlier Folio entry with a progress bar; a camera per party slot in first or third person, or a free camera; HUD on or off. **Sharing**: "Export" copies the `.palr` file to a folder the player chooses, and a ranked run adds "View verification" (doc 16 §10.2). The viewer runs outside any run and never writes saves, account state, achievements or leaderboard entries.
- **No photo mode** (DECISIONS 129). Replays and the killcam stay; no screen poses the camera, hides the HUD for stills or captures images in a run. The replay viewer's free camera and HUD toggle are its only camera tools, and the platform's own screenshot key is left alone.

---
## 10. Canon 1.2 systems: HUD and menus

### 10.1 Wards

- **HUD**: the ward bar (320 × 6 px) sits directly above the HP bar on the same scale (`px = 320 × ward_mhp / max_hp_mhp`, clipped at 320 with a "+N" label for the overflow). The `st.shielded` pool draws first (solid), the generator ward after it (hatched), matching the consumption order of Canon §10.5 step 7.
- **Recharge**: a 180-tick delay ring around the ward glyph from the last damage tick (doc 05 §15.4), then a fill animation at the real rate. Example (doc 05 §15.4): a Common, Normal, +0, Coherent aegis at ilvl 10 holds 32,380 mHP under Canon §13.2's capacity chain and refills 8,095 mHP/s, so the bar refills in 4 s after the delay.
- **Break**: on `ev.ward_broken` the ward glyph swaps to a cracked hexagon for 60 frames (no full-screen flash; §13.8). A **Flicker** jam (doc 05 Contradiction) shows "Flicker: recharge delayed 3 s".
- **Item card**: capacity at the item's `ilvl`, recharge per second, the depletion profile per type with glyphs, hum dB, on-break effect.
- **Enemies**: a warded champion's boss-bar or nameplate shows its ward bar; a `counter.ward_breaker` source in the player's kit lights the counter glyph on that bar.

### 10.2 Mounts (retired)

Mounts are cut (DECISIONS 55, Canon §17.7). This chapter has no mount panel, action, input context or tip, and command bit 25 has no action (§12.1).

### 10.3 Minions (`ui.hud.minions`)

- A 24 px header reading "Minions 2/3" (`MINION_CAP_PER_PLAYER`, Canon §15.1), then one 24 px row per minion (up to 5): kind glyph (summon, tamed, hireling, companion; doc 13 §6), name, HP bar, command glyph (F follow, H hold, A attack), a remaining-time bar for spell summons with a duration, for companions an approval pip on −100..=100 with the loyalty mark at 60 (doc 13 §13.5), a leash glyph while the minion is more than 16,000 mm from its owner (it drops its target and returns) and a teleport glyph while it is more than 32,000 mm of path away with no target (doc 13 §6).
- **Commands** (Canon §15.1, doc 13): `input.minion_follow`, `input.minion_hold`, `input.minion_attack`. A tap commands every minion of the owner. Holding the input for 20 frames opens a selector listing the owner's minions; the chosen minion alone receives the command. Each command sends `MinionCommand { kind, minions, cell, target }` (doc 02 intent; Interface assumptions), which doc 13 resolves into its per-minion `Follow`, `Hold(CellIdx)` or `Attack(EntityId)` for each listed minion. On gamepad the minion wheel (§6.5) carries socket 0 follow, socket 2 hold, socket 4 attack (all minions) and socket 6 "Choose minion…", which opens the selector; the other sockets are empty.
  - Follow: `cell` and `target` empty.
  - Hold: `cell` = the block the crosshair ray hits within 48,000 mm, else the owner's feet cell; a world marker stands on that cell.
  - Attack: `target` = the entity under the crosshair (48,000 mm ray, doc 05 hitscan rules); with no entity under the crosshair the input is refused locally with "No target".
- World markers: each minion's attack target carries a small owner-colored claw glyph (`ui.icon.marker_claw`) with a ring for the 600 ticks the order lasts (doc 13 §6); hold cells show a banner glyph (`ui.icon.marker_hold`).
- **Retinue** (`ui.menu.retinue`, a Book tab while the owner has at least one minion): per minion its name, kind, template skin name, doc 13 profile, HP, AR, damage per hit, current command, Folio rule ("follows through an exit within 16 m" for tamed beasts, hirelings and companions; "despawns at Folio exit" for spell summons), and for companions the approval value with the values it likes and dislikes, the loyalty perk text and whether it is active (approval ≥ 60), the personal quest state (offered at 100) and the warning "Leaves at the next exit" at approval −50 or lower.

### 10.4 Ghosts (`ui.hud.ghost`)

When `ev.player_ghosted` fires for the local player, the HUD switches to the ghost layout: vitals, weapon and quickbar hide; the screen desaturates to 30% saturation and gains a static vellum vignette.

| Element | Content |
|---|---|
| Banner (`ui.hud.ghost`, §3.2) | "You are a ghost. You return at the next Folio." |
| Resurrection forecast | "Remains carried: return at 50% HP" or "Remains left behind: return at 25% HP and lose one equipped non-weapon item" (Canon §17.6), updated live from which inventory holds the Remains |
| Mark | `input.attack` on a non-ghost entity whose origin is within 30,000 mm and visible from the ghost's eye sends `MarkTarget { target }`: 480-tick `st.marked`, 600-tick cooldown shown as a ring (doc 16 §9.5); out of range the reticle shows "Too far" |
| Doors and pings | `input.interact` opens doors and presses switches and levers; `input.ping` pings (Canon §17.6) |
| Spectate | `input.weapon_next` / `input.weapon_prev` cycle the camera through living allies; `input.jump` returns to the free ghost body |
| Remains marker | always on the compass and map |

The party frames show a ghost glyph for that slot.

### 10.4a Downed (`ui.hud.downed`)

Co-op only, because solo has no downed state (doc 16 §2.2). While the local Erratum holds `st.downed`, the `ui.hud.downed` panel shows "Bleeding out" with `bleed_left` as M:SS and a bar of `bleed_left / max(600, 1800 − 600 × (downs_folio − 1))` (doc 16 §9.4); while a valid reviver acts, a second bar `revive_progress / revive_ticks` names the reviver. Each hit taken while downed shortens the bar by `ceil_div(total_mhp × 1800, max_hp_mhp)` ticks and marks the bar's edge, with no full-screen flash. The weapon panel and quickbar dim to 4000 Bp opacity; context `input.ctx.downed` allows look, crawl, ping and chat. Allies see the downed player's party frame with its `bleed_left` from `PartyState`, a world marker, and, within 1,500 mm and facing it within 8,192 Bam, a revive prompt with a hold ring of `revive_ticks` (180, or fewer with revive talents).

### 10.5 Kin and Origin

| Kin | ID | HUD change | Menu change (Body tab and cards) |
|---|---|---|---|
| Human | `kin.human` | none | creation shows +4 free points and +5 to one chosen skill (doc 04 §2.4) |
| Construct | `kin.construct` | the satiation glyph becomes the Fuel gauge; no breath bubbles | immune to poisoned, diseased and bleeding; shock resistance −25%; potion cards show "Construct: healing ×0.50"; self-repair shows its heal `mul_bp(max_hp_mhp, 1000 + 20 × tinkering)` (11,900 mHP, shown as 11 HP, at Tinkering 20 and Max HP 85); natural armor `mul_bp(3, T(d))` |
| Undead | `kin.undead` | no regeneration glyph; umbral damage taken shows as a heal number in `stamina` color with the umbral glyph; no breath bubbles | radiant resistance −50%; immune to poisoned and diseased; satiation decays ×0.50 and only meat, body parts, corpses and blood restore it |
| Beastkin | `kin.beastkin` | the paper doll has a tail; a severed tail removes the poise line | dark-vision +2 m on row 31; poise +5 while the tail is attached; hearing +3 dB on row 33; satiation decays ×1.25 |
| Fae | `kin.fae` | a hostile wielding a weapon whose primary material is `mat.cold_iron` or `mat.iron` shows an iron glyph on its nameplate | "Iron hurts you ×1.25"; ARCANA tolerance +1 in the tolerance row; fall damage ×0.70; footsteps −3 dB; disguise suspicion ×0.80 on row 40 |
| Mutant | `kin.mutant` | none | mutation points and the 3-offer mutation picker; radiation resistance +25% |

The Kin and Origin tab lists the Origin's tolerance row (Canon §5.4), the Origin trait, Going Native state, and the kin's genre skin name.

### 10.6 Afflictions

- **Status row**: afflictions draw first, in a thorn-ring frame. `st.incubating` shows the incubating affliction's glyph with a 7200-tick countdown.
- **Afflictions tab**: per affliction, its skinned name in the local genre (vampirism, ghoul hunger, jiangshi; lycanthropy; Red Ink), upsides and downsides as two lists, cure options with live prices: Scriptorium cure `mul_bp(500, P(d))` Lucre (Folio 10: `mul_bp(500, 21718) = 1085`), or a blessed cure potion. For `afflict.red_ink`: doses taken, the Marginalia powers unlocked, and the hunter threshold `max(6, 10 − doses)`.
- `afflict.hunger` adds a sun glyph to the vitals glyph row while `light_at ≥ 12` ("No regeneration in bright light"). `afflict.beast` shows "Beast form ready" while its once-per-Folio change is unused, the 30 s `st.polymorphed` timer while transformed, and "The beast stirs" while composure is below 25%.
- `ev.affliction_changed` raises a notification and, the first time, `ui.tip.affliction` (§15).

### 10.7 Patrons and prayer (`ui.menu.prayer`)

- `input.pray` tap: supplication to the default patron (the local genre's), sent as `Pray { patron, kind: supplication }`. Holding it for 20 frames opens the prayer menu instead.
- **Prayer menu**: 16 patron rows (patrons of genres the run has visited first), each with favor on a −1000..=1000 bar, the "local ×1.5" badge for the local patron, the signature boon and the taboo text. The header shows `prayer_timer` in seconds.
- **Exact odds** for a supplication, from doc 06 §16.3's four draws `r0 = range(500)`, `r1 = range(180,000)`, `r2 = range(10000)` with favor `f` and timer `τ` (`prayer_timer`), and the prayer component's Contradiction failure chance `u` in Bp (Canon §5.5; always 0 for the local patron, whose `Req = MYTHOS − 2` never exceeds the local MYTHOS). Each line is an exact count of draw outcomes shown with `pct4` (P5). "Unheard" shows `u`; every other line is multiplied by `(10000 − u) / 10000`:
  - `τ = 0`: Pleased `p = 1` when `f ≥ 0`, `(500 + f) / 500` when `−500 < f < 0`, else 0; Displeased `1 − p`. Inside Pleased, a friendly herald 5.0000% when `f > 250`, and a Gift (+1 enhancement) 5.0000% when `f > 100`.
  - `τ > 0`: Helped `h = min(max(f, 0), 500) / 500 × max(0, 180,000 − τ − 1) / 180,000`; Angry `1 − h`; inside Angry, a hostile herald 20.0000% when `f < −250`.
  - With `skill.devotion.m100`, the Erratum's first supplication of each Folio shows the `τ = 0` lines whatever the timer (doc 06 §16.3).
  - Examples, local patron: `f = −120`, `τ = 0`: Pleased **76.0000%**, Displeased 24.0000%. `f = 300`, `τ = 20,000`: Helped **53.3330%** (`pct4(300 × 159,999, 500 × 180,000)`), Angry 46.6670%. `f = 300`, `τ = 179,999`: Helped 0.0000%. A foreign patron whose prayer has failure 1000 Bp, `f = 0`, `τ = 0`: Unheard 10.0000%, Pleased 90.0000%.
- **Petitions**: each with its favor cost as doc 06 §16.4 charges it, `floor_div(c × 100, 100 + devotion)` where `c = mul_bp(cost, 8000)` from Devotion 25 (`skill.devotion.m25`) and `c = cost` below it (Benediction 60 shows 50 at Devotion 20, and `floor_div(48 × 100, 150) = 32` at Devotion 50), its minimum favor, the gate `prayer_timer <= 7200` (each petition adds 3600 ticks, so from 0 three chain and the fourth waits 3480 ticks (58 s), doc 06 §16.4), and its effect.
- **Altar offering**: an item list with each item's favor gain `isqrt(value)`, ×2 for the patron's genre, then Devotion scaling (doc 06 §16.5).
- **HUD** (`ui.hud.spell` lower line): "Prayer ready" when `prayer_timer` is 0, else the timer in seconds with "petitions open" while it is at most 7200; favor with the local patron.

### 10.8 Margin trophies (`ui.menu.margin_trophies`)

Canon §17.8 fixes the Margin's architecture and gives each account cosmetic trophy and decoration slots; there is no free-form building. The trophy screen arranges the 32 slots of the account's alcove, doc 14 §5.1a's `margin.plot`:

- A 3D view of the alcove with every slot outlined. Slots are doc 14's `margin.slot.*` records (12 trophy, 8 wall, 6 floor, 3 banner, 3 light), each with its footprint, centre and opening rule; a slot not yet open shows its rule ("Opens at 10 achievements").
- A drawer lists owned `cosm.margin.*` decorations and earned trophies, filtered to the focused slot's kind. Placing, swapping and clearing are free and instant; changes save to the account profile. Trophies show their source ("Editor of Volume II, first kill, 2026-11-02").
- Visiting players in co-op lobbies see the host's arrangement. Nothing on this screen affects gameplay.

### 10.9 Lean controls

- `input.lean_l` / `input.lean_r` set `LEAN_LEFT` / `LEAN_RIGHT` (bits 21, 22). Defaults are hold; `access.lean_toggle` makes each a toggle that releases when the opposite lean is pressed, on sprint, or when lean becomes unavailable (sprinting, climbing, swimming, downed, melee swing; doc 05 §6.8).
- **Indicator**: `ceil_div(|lean_k|, 3)` chevrons beside the crosshair on the lean side (1 to 3; doc 05 §6.8 `lean_k ∈ −9..=9`). When the eye sphere test shrinks the offset (doc 05 §6.8), the chevrons turn `muted` with a wall glyph.
- The viewmodel and camera roll 3° toward the lean side for readability (presentation only; `access.reduce_motion` sets 0°).

### 10.10 Stealth meter with two-channel light readout (`ui.hud.stealth`)

Doc 09 §8 decides what the host sends each player (`StealthHud { gem, hidden, rings, marks, steps }`); this section decides how it is drawn. The meter is 264 × 44 px in three parts:

1. **Awareness eye** (left, 44 px): the highest doc 09 §6.1 state any hostile holds toward the player, read from `StealthHud.marks`: `aware.unaware` closed; `aware.curious` a slit; `aware.suspicious` half open; `aware.searching` half open with a sweeping pupil; `aware.lost` three-quarters open with "?"; `aware.alert` fully open in `hostile` with "!". Openness and the mark carry the state, so color is never the only signal (P3). `StealthHud.hidden` (`st.hidden`) adds a hood over the eye.
2. **Light readout** (center, 140 px): two 15-segment bars, **Sky** (`light_sky`) and **Lamp** (`light_block`), sampled at the Canon §2.3 `light_at` cell after the active light overrides clamp them (Canon §2.3, such as an Editor's blackout). The larger is highlighted as the effective light with its band label from doc 09 §2.2 and doc 12 §11.3: Dark (≤ 4), Dim (5..=9), Bright (≥ 10). Fixed marks: segment 2 (darkness drains composure after 10 s, doc 04 §11.3), segment 8 (composure regenerates out of combat, Canon §12) and, on the Lamp bar, the genre's `spawn_block_max` (Canon §5.9). When `light_block > spawn_block_max`, a lantern glyph reads "No spawns here" (Canon §12.3). Under the bars sits doc 09 §8.1's **visibility gem**: 5 pips from `StealthHud.gem` (none below 500, then one each from 500, 1500, 3000, 5000 and 8000). When the player's own carried light (doc 09 §2.3 `item_light` of lit equipped and held items, computed on the client) exceeds the effective light, a torch glyph reads "Your light: N".
3. **Noise** (right, 80 px): the level in dB of the player's own last `ev.noise`, decaying on screen by 20 dB per second, with the gait glyph. Doc 09 §8.2's noise rings (`StealthHud.rings`) draw in the world on the ground plane for 30 ticks, colored by reaction class at 6000 Bp opacity (allies' rings at 3000 Bp).

**Caps.** The meter draws exactly what doc 16 §6.8's `STEALTH` section carries: at most 4 rings (the newest), 8 marks (the highest `meter` first, ties to the lower observer `EntityId`) and 4 steps (the newest). The section is written only when it changes, so the client keeps the latest one it received. Ring positions are `dx`, `dy`, `dz` in whole mm from the player's own position.

**Crosshair rings.** Doc 09 §8.3's detection arcs draw on the radius-180 ring: one arc per received mark whose `flags` bit 0 is set (its meter rose in the last 20 ticks; Interface assumptions, doc 09), centered on `bearing.wrapping_sub(camera_yaw)` from the mark's own `bearing` field (the yaw from the player to the observer, Canon §1.9), never on the observer entity's position, because the observer's net handle is 8191 when it is not relevant to the player; the arc is `meter / 10000` of a quarter circle (16384 Bam) long, in white (curious), yellow (suspicious), orange (searching, lost) or red (alert), with the state letter C, S, ? or ! at the arc's midpoint so color is never the only signal. Footstep ticks (`StealthHud.steps`, Canon §6.4 row 34) draw on the radius-200 ring at each step's yaw relative to the camera yaw; none under `st.deafened`. Over-head awareness glyphs are doc 18 §4.6's `vfx.aware.*`.

Example: in Gothic Horror (`ambient_light` 2, `spawn_block_max` 7) beside a placed lantern the bars read Sky 2, Lamp 9: effective 9, "Dim", the composure mark lit, and "No spawns here". In The Redaction and The Unwritten (`spawn_block_max` 15) the lantern glyph never appears and the Lamp bar reads "Spawns ignore light here". A Shade with `gem = 471` shows no pips; with a lantern (`gem = 2467`) it shows two (doc 09 §8.1).

`access.stealth_meter_always` shows the meter for the whole run; by default it shows while crouched, while any hostile holds a state above `aware.unaware` toward the player, and for 3 s after any change of light band.

### 10.11 Locks (`ui.hud.lockpick`)

`access.lock_mode` (`auto` default, `manual`) is doc 09 §13.3's per-player `lock_mode`; it governs every lockpick attempt and every hack, Seize included. The client sends it as the intent `SetLockMode { manual }` at run start or join and on every change, so the host and the replay hold it (Canon §2.4); an attempt keeps the mode it started with.

- **Auto** (doc 09 §13.2, §13.4) shows the attempt's progress ring over its ticks, the success chance `clamp(5000 + (rating − DC) × 200, 500, 9500)` as "Success x.x%", and for a lock, beside it, "Slip x.x%" from `slip_bp` (2500, or 1250 from Security 25; a slip adds `SLIP_TICKS` = 30 ticks and a 35 dB `noise.lock.snap`) and the pity bar `progress / lock_hp`. Lockpicks and hacking decks never wear or break (Canon §13.9): the slip replaced the old break roll, so no break chance exists. Example (doc 09 §13.2): Security 45, SLT 50, normal picks, a `lock.t3` door (DC 55): rating 72, "Success 84.0%" and "Slip 12.5%" above an empty pity bar that fills 4 per failure.
- **Manual** (doc 09 §13.3) shows a 40-notch dial per pin and `2 + tier` pins, where `tier` is the lock tier, the device tier for a device verb or for a Seize on a doc 12 camera or turret, and 1, 2, 3 or 4 for a Seize on a minion, standard, elite or champion; a manual hack or Seize chosen in the hack panel (§10.12) runs on this dial. The pin targets are `range(40)` draws on keyed `rng.stealth` seq 1004 with the picker or hacker as actor, which the owner's client computes from the same key (it holds the run seed, doc 16 `RunHeader`). The cursor advances one notch per tick from the pin's `pin_start`, and the pins already set stay lit; a pin's tension glyph lights while the cursor is within `W = clamp(1 + floor_div(rating − DC + 40, 10), 1, 8)` notches of the pin's target (the rating 72 against DC 55 above gives `W = 6`). `input.attack` sets the pin. A miss, or 120 ticks without a press, adds a crack and restarts that pin at `pin_start = now + 1 + SLIP_TICKS`: the dial shows the 30-tick wait as a draining arc and ignores presses until it ends. The third crack fails the attempt: a lock gains pity progress, a hack is a failed hack (`hack_fails + 1`, shown on the hack panel's header, §10.12), and a Seize applies doc 09 §13.5's failure.
- **Deck spark.** When the hacking deck's failure roll (doc 07 §11.1 rule 4) fails as a hack or Seize starts (doc 09 §13.4), the attempt ends on that tick, auto or manual, and the panel shows "The deck sparked" with no crack, no failure count and no pity change.

Context `input.ctx.lockpick` binds only `input.attack`, `input.ui_back` (abandon) and look.

### 10.12 Hack and pickpocket panels (`ui.hud.hack`, `ui.hud.pickpocket`)

Doc 09 §13.4 and §13.7 resolve hacking and pickpocketing; these panels choose the intent. Neither pauses the simulation. While one is open `input.ctx.menu` drives it (§12.1), so the player stands still while choosing, and `input.ui_back` closes it.

- **Overlay placement.** `ui.hud.lockpick`, `ui.hud.hack` and `ui.hud.pickpocket` are contextual overlays, not rows of the §3.2 or §3.4 tables: a 400 × 144 reference-px panel with pivot C, placed at the C anchor with offset (0, −112) × `s` (x 440–840, y 216–360 at 1280 × 800, `s = 1.0`). While one is open, `ui.hud.prompt` and `ui.hud.vote` hide. It clears the invasion banner and the crosshair at every §3.2 and §3.4 scale, margin and canvas; the ghost and downed banners never coincide with it, because a ghost or downed Erratum cannot pick, hack or steal. The host-migration panel `ui.hud.net_status` (§11.9) uses the same rectangle and, while it shows, draws above the other three and above `ui.hud.downed`.
- **Hack panel** (`ui.hud.hack`). `input.interact` opens it while the crosshair ray (48,000 mm, as in §10.3) hits a device doc 09 §13.4 can hack, within the hacker's `range`, and the player carries a hacking deck. The header shows the device's skinned name, its tier and "Failures f/3" from `hack_fails` (the third raises an alarm). One line per verb the device offers (Unlock, Loop, Seize, Disable, Test, Cut; doc 09 §13.4 table) shows the verb, its effect text, the success chance from `chance` with one decimal and the channel time from `ticks`; Seize shows doc 09 §13.5's chance and ticks. When the device's tier exceeds `your_tier`, every line is greyed with "Needs Hacking N", `N = 14 × (tier − 2)`, the least skill whose `your_tier` reaches the tier. Choosing a line sends `Hack { target, verb }` and closes the panel; the channel then shows as a ring on `ui.hud.prompt` with doc 09's cancel reasons, or, with `access.lock_mode = manual`, runs on §10.11's pin dial (doc 09 §13.3).
- **Pickpocket panel** (`ui.hud.pickpocket`). Tapping `input.interact` (released within 10 ticks) on an eligible target opens it (doc 09 §13.7). The header shows the target's skinned name and "Stay behind it, within 1.5 m". One line per stealable item shows its name as the local player knows it (§4.3 row 2), its mass (P11), the success chance with one decimal and the channel time; from Pickpocket 50 a Plant tab lists the thief's own items with their chances (DC − 10). Choosing a line sends `Steal { target, uid }` or `Plant { target, uid }`; the panel then shows the channel's progress bar `elapsed / ticks` and its cancel reasons ("Left 1.5 m", "In its view"), and closes when the channel ends or the target stops being eligible.
- **Times** on both panels show seconds with one decimal as `round_half_even(ticks, 6)` tenths: 96 ticks shows "1.6 s", 63 ticks "1.0 s".

---

## 11. The Margin and meta screens

### 11.1 Margin hub

The Margin is a walkable first-person 3D hub with stations, vendors and trophies (DECISIONS 77, doc 14 §5); the local player walks it on the on-foot bindings, under `input.ctx.margin`. Every open facility (doc 14 §5.1) has a lectern prompt. `input.menu_book` in the Margin opens the **Margin index** (`ui.menu.margin_index`), a list of every open facility that opens its screen directly without walking (P6, cognitive and motor accessibility). Locked facilities list their opening milestone. The trophy and decoration arranger (§10.8) opens from doc 14's `margin.plot` facility and is listed as "Trophies". **Margin dialogue box** (`ui.menu.margin_dialogue`): `input.interact` on a resident (doc 13 §13.6) opens a text box with the speaker's name and one line at a time from doc 18 §8.4; `input.ui_confirm` advances and `input.ui_back` closes. A resident's arc beat that doc 14 §5.6 has made available plays first, the next time the player speaks to that resident (DECISIONS 78), then the facility talk. Residents are vendors and quest-givers whose fixed arcs advance across runs, with no relationship systems (DECISIONS 127): the box offers no affinity meter, gift or romance option, only the next line, the resident's facility and Close. No line in the box is voiced; the residents' voiced greetings (doc 18 §6.5) play on approach with subtitles (§13.2). The Narrator's arc beats show as pencil notes on the Ledger's screen (doc 18 §8.4). The Lectern runs the doc 14 §2.1 lobby (`ui.menu.lobby`) with one screen per state:

| Lobby state | Screen |
|---|---|
| `Idle` | the Lectern prompt; "Join a party" opens the lobby browser (§11.9) |
| `ModeSelect` | every mode with its lock state and unlocking milestone, a ranked mark, and the Daily and Weekly seeds with their dates; Second Edition is not listed, because it is entered only through an Ur-Text victory's door (§11.10) |
| `ErratumSelect` | per slot, the Dramatis page (`ui.menu.dramatis`, below): Custom (opens `ui.menu.creation`, §5.7), Echo or a saved preset; the kit picker; up to 2 Glyph Library picks (doc 14 §5.7, DECISIONS 33); the Satchel editor (§11.3) where doc 15 §5.5 allows it |
| `RunSetup` | structure, Grade (up to the host's maximum), mutators, seed entry, co-op modes (hidden with 1 slot), the invasion toggle (hidden for Daily, Weekly and ranked runs, Canon §17.9, and in every build before invasions ship in their update before 1.0, DECISIONS 118 and 125), and doc 15's Apocrypha notice when a mod is loaded |
| `ReadyCheck` | each slot's Ready state; any host edit clears every Ready flag, and the screen says so |
| `Commit` | "Opening the book…" while the ranked token and `start_run` resolve |

**The Dramatis** (`ui.menu.dramatis`; doc 14 §5.1 `margin.dramatis`). One page serves two places. At the Dramatis facility it edits the account's saved Erratum presets (Origin, Calling, kit, appearance, name), Echo selection and trait variant, and the lens pocket; in `ErratumSelect` it sets up the local slot from Custom, an Echo or a preset. Its Glyph Library panel lists every glyph of the account's library (doc 14 §5.7) grouped by class (Form, Essence, Modifier, Trigger, Condition), with each glyph's skinned name and tier. In `ErratumSelect` the player picks up to 2 distinct glyphs there: a third pick is refused, and a refused glyph is greyed with its reason (in doc 06's run-start set, taught by the kit primer, or outside the run's content set); Daily, Weekly and baseline Seeded runs list doc 06's 52 `base` glyphs instead of the library. At the facility the panel is read-only, because picks belong to one run (doc 14 §5.7 stores them in `SlotState.library_glyphs`).

**Seed entry** (Canon §3.6): one 13-character field shown in 4-4-5 groups, with the Steam on-screen keyboard on Deck. The field strips `-` and ASCII space, uppercases ASCII letters without locale rules (so tr-TR's dotted capital İ never appears), maps `O` to `0` and `I`, `L` to `1`, and reports the first failing rule in canon order: "Needs 13 characters" (length), "U is not used in seeds", "Character X is not in the seed alphabet", "The first character must be 0–F". Example: `028t 5cy4 tqkff` is accepted as `028T-5CY4-TQKFF`; `G28T-5CY4-TQKFF` reports "The first character must be 0–F".

### 11.2 Inkwell (`ui.menu.inkwell`)

- **Banner page**: featured art, banner type and period, the pull buttons (1 pull: 160 Ink; 10 pulls: 1,600 Ink; Beginner 120), Ink balance, "Pulls covered: ⌊balance / cost⌋", and the earned-only line "Ink is earned by playing. It cannot be bought." with its hours equivalent (§11.5).
- **Odds panel** one input from the buttons (doc 15 §3.11.2): tier table, every entry with base and consolidated probability, the hazard formula and full `p_bp(n)` table, featured, rate-up, spark, duplicate and carry-over rules, and live counters.
- **Your next pull**: `pt = top.p_bp(top_since + 1)`, `pm = min(mid.p_bp(mid_since + 1), 10000 − pt)`, Plain `10000 − pt − pm`, each through `pct4`, plus the featured split. Example: `top_since = 47`, `mid_since = 3`, no guarantee: Illuminated **20.5000%** (of which featured **15.3750%**, each off-featured unit **0.4659%**), Rubricated **6.0000%**, Plain **73.5000%**.
- **Pity bar**: a strip of `hard` cells (60, or 30 on the Beginner banner, doc 15 §3.3) with the soft-pity start marked (cell 46, or 21), `top_since` filled, and the line "Illuminated guaranteed within `hard − top_since` pulls". Guarantee flags read "Next Illuminated is the featured Echo" when set. Spark shows `spark / 100` with a Claim button at 100.
- **Reveal**: one card per pull, frame color by tier (gold leaf, vermilion, iron-gall), no near-miss motion; any input skips to the summary after 30 frames. Duplicates show their grant ("Fold 2", "+5 Vellum").
- **History** (doc 15 §3.12) is a tab of this screen, and **Fairness** opens the Fairness panel (§11.8) on the banner's epochs.

### 11.3 Reliquary Vault (`ui.menu.vault`)

Sealed Keepsakes as silhouettes with source-genre tag and rarity frame (doc 15 §5.5); the rarity table with base and consolidated rates; counters "Epic or better within `5 − c5`" and "Legendary or better within `20 − c20`"; the Satchel editor for the next run (up to 5, at most 1 Legendary or Exotic, refusals with doc 15's text, including "Not in this run's pages"). Opening plays a 45-frame unsealing, skippable. The vault holds at most 30 Keepsakes (doc 15 §5.2); at the cap the Open button is disabled with "The vault is full (30/30)", and each Keepsake has Discard with one confirmation and no refund. A Fairness button opens §11.8 on `banner.reliquary.account`.

### 11.4 Codex pass (`ui.menu.codex_pass`)

- A 100-tier ribbon in 10 pages of 10, free and premium tracks stacked, every reward previewable in 3D before purchase (doc 15 §6.4).
- Progress: `tier = min(100, floor_div(xp, 12000))` and `xp mod 12000` toward the next. Example: 43,900 XP shows **tier 3** and **7,900 / 12,000** (65.8%).
- Catch-up reserve "Rested: 40,000 XP (doubles your next earnings)" with its 160,000 cap; Quills held; the active-Codex selector over every released Codex; Coda progress `coda_xp / 24,000` at tier 100.
- No timers, no "season ends", no "last chance" copy anywhere (doc 15 §6.1). The premium purchase button routes to the Store detail page of that SKU.

### 11.5 Store (`ui.menu.store`)

- Catalog order per doc 15 §7.5: newest first, then price ascending; shelves Expansions, Codex, Cosmetics, Sets, Supporter, Seasons. Cards: art, name, the local price as the final amount (or pre-tax with "+ tax" where `juris.flag.vat_inclusive_display` is false), "Owned" or "New" (14 days, no timer).
- **Real-currency equivalents.** Every price on the Store is real currency; no virtual currency exists, so no exchange rate exists to disclose (doc 15 §7.3). The detail page shows a "Price details" expander with the USD anchor tier and the SKU's row of the launch matrix in the 9 matrix currencies, labeled "Reference prices in other regions". Every account-currency price anywhere else (Inkwell, Bindery, Vellum Archive) carries "Earned only: no real-money price" and its play-time equivalent at the doc 15 §2.7 reference rates (Ink 400, Quires 60 and Vellum 2.5 per reference hour): 160 Ink "≈ 0.4 h of play" (`160 / 400`), 1,600 Ink "≈ 4.0 h", a 20-Vellum Archive entry "≈ 8.0 h" (doc 15 §2.8). Hours show one decimal with `round_half_even` on tenths.
- **Set bundles**: the computed price and the sum of the unowned components, both in currency; example (doc 15 §7.1): components 5.99 + 3.99 + 7.99 + 1.99 unowned give USD 16.99 against USD 19.96.
- **Purchase flow** follows doc 15 §7.6 with one screen per state; the confirm dialog lists every granted item, the refund terms and, where required, the unticked withdrawal checkbox.
- **Purchases settings** (Settings → Purchases): purchases off by default, the spend summary (lifetime, last 30 days, per SKU), the guardian PIN screens (doc 15 §8.4).

### 11.6 Pause menu, Settings, scoreboard and chat

**Pause menu** (`ui.menu.pause`, `input.pause`). Rows in order, focus on Resume when it opens: Resume; Scribe's Mercy (a toggle row reading "Off" or "On · 26%"); Settings; Controls (opens the Controls page directly); Journal; Party (co-op: the scoreboard list); Save and quit to the Margin (solo: saves at the current tick boundary, or, while an invader is present, on the tick `ev.invasion_ended` is emitted, Canon §2.5; co-op: leaves the session, doc 16 §9.2); Abandon run (one confirmation, "The run ends here."; in Second Edition the row reads "Abandon run (forfeits the clear)" and the confirmation adds "This forfeits the Ur-Text clear.", §11.10). Scribe's Mercy is therefore 2 inputs away (down, confirm) and Settings 3 (down, down, confirm), matching P6.

**Settings** (`ui.menu.settings`). Tabs in order, opening on Accessibility: Accessibility (§13, grouped as Assists, Vision, Text, Motion, Audio, Input), Controls (§12: presets, rebinding, sensitivity, deadzones, gyro, aim assist, toggle versus hold), Gameplay (damage numbers, HUD layout, scale and preset, minimap, `access.lock_mode`, gore mode, tips reset), Video (window, FOV and the comfort rows shared with Accessibility), Audio (bus volumes, mono, dynamic range, voice mode, push to talk), Social (chat filters, voice mode, per-member voice mutes (§11.9), invasion permission, blocked list; defaults per doc 15 §8.5, and voice with players who are not friends starts off, DECISIONS 75; no crossplay setting exists, DECISIONS 112), Generated text (the Scribe toggle `flag.scribe.enabled` with its disclosure page and "Delete model and cache", doc 18 §9.10; hidden outside `en-US`, doc 18 §9.11, and in every build before 1.0, when the Scribe ships as an opt-in, DECISIONS 120), Purchases (doc 15 §7.3, §8.4), Account (profile, data export and deletion, doc 16 §14). A setting shown on two tabs is one record. Every row shows its value, range, default and a one-line description; "Reset this tab" sits at the bottom of each tab.

**Scoreboard** (`ui.menu.scoreboard`). Holding `input.scoreboard` shows it as an overlay that does not pause; `input.ui_confirm` while it is held pins it open as a menu. One row per slot: the portrait in its equipped portrait frame (§11.10), persona, level, Calling and Origin sigils, HP, ping in ms (`PartyState.ping_ms`), kills, deaths, downs, the Mercy quill when on, the speaking glyph, and the struck speaker glyph while that member's voice is muted for the viewer (§11.9). An invader's row (Canon §17.9) shows its persona and Redactor kit. Every row offers Mute, Block and Report within 2 inputs (doc 15 §8.5), Unmute voice for a voice-muted member, and Kick or Vote to kick where §11.9 allows; blocking the invader ends the invasion (doc 16 §9.13).

**Chat** (`ui.menu.chat`, `input.chat`): a 200-byte text field (doc 16 §9.12) on the Party channel with the on-screen keyboard on Deck. Minor-flagged accounts see quick-chat phrases where doc 15 §8.5 replaces free text. Chat with an invader follows doc 15 §8.5: text carries mute, block and report, voice with an invader starts off, and minor-flagged accounts exchange only pings and emotes with it.

**Later screens** (DECISIONS 96, 104, 118). Asynchronous traces and streaming audience votes ship in updates before 1.0 (DECISIONS 118) and add screens to this chapter once doc 16 specifies the trace service and the vote path (its open item O2); until then no `ui.`, `input.` or `access.` ID exists for them. Those screens keep the ledger's rules: traces and votes are opt-in and off by default, votes never run in ranked or Daily runs, and a streamer-safe mode hides player names and marginal notes. Second Edition ships at Early Access (DECISIONS 118), and §11.10 gives its screens.

### 11.7 Vellum Archive (`ui.menu.vellum_archive`)

The Archive opens at `margin.vellum_archive` (doc 14 §5.1). It has one tab per doc 15 §3.9 shelf: Illuminated Echo and Illuminated Script at 100 Vellum, Rubricated Echo and Rubricated Script at 20, Plain cosmetic at 5, and Fold step at 40 for an Illuminated Echo or 8 for a Rubricated one; then the **Transcription** tab. The header shows the Vellum balance. Each entry is a card with its exact price, the earned-only line and the play-time equivalent of §11.5. An owned entry never appears (the Fold step tab is the route for an owned Echo), and an expansion entry whose `requires_sku` the account does not own names that Expansion and cannot be bought. Every purchase is deterministic and has one confirmation naming the entry and its price.

- **Transcription shelf** (doc 15 §3.9.1). The header reads "Transcription: N", the account's `transcription` counter, with the line "Each qualified Folio exit adds 1. It never expires and buys nothing else." The shelf lists every Script the account does not own and can open (in the build's content set, and for an expansion Script with its `requires_sku` owned). Each shows its cost, `TRANSCRIBE_RUBRICATED` = 16 for a Rubricated Script or `TRANSCRIBE_ILLUMINATED` = 32 for an Illuminated one, and its play-time equivalent at 12 qualified exits per reference hour (doc 15 §2.2), with one decimal by `round_half_even` on tenths: 16 shows "≈ 1.3 h of play" and 32 "≈ 2.7 h of play". Claim is enabled when `transcription ≥ cost`; otherwise it is greyed with "Needs K more Folio exits", where `K = cost − transcription`. A claim the service refuses with `insufficient_transcription` shows the same line and changes nothing. A successful claim shows the Script's card as a new entry and touches no Vellum, pity, spark or Fold. Transcription is a counter, not a currency (Canon §14), so the shelf carries no "Earned only" price line and no store link.
- **Example** (doc 15 §3.9.1). With `transcription = 40`, claiming `bundle.script.lost_chapter_wuxia` (Illuminated, 32) leaves "Transcription: 8", and `bundle.script.kit_monk` (Rubricated, 16) then reads "Needs 8 more Folio exits".

### 11.8 Fairness panel (`ui.menu.fairness`)

The panel opens from the Inkwell (§11.2) and the Reliquary Vault (§11.3). It runs doc 15 §4's provable-fairness client on the account's epochs, one row per epoch scope `(banner, period)` (doc 15 §4.1):

- **State.** `Committed`, `Active` or `Revealed`; the `commitment` as 64 hex digits in groups of 8; the draws made (the `nonce` count); and for a revealed epoch its `server_seed`.
- **Client seed.** A mode selector between `fresh`, the default (16 new bytes from the OS random source for every epoch), and `pinned`, with a text field of 1 to 64 bytes that shows its byte count. Under the selector stands the line "Only fresh mode rules out server seed grinding." (doc 15 §4.1). Changing the pinned seed reveals the Active epoch; its confirmation says so, and the change takes effect between draws.
- **Reveal now** on an Active epoch reveals it between draws (doc 15 §4.1); the next draw opens a new epoch at `Committed` with nonce 0.
- **Verify** on a revealed epoch runs the `palimpsest-verify` code linked into the client (doc 15 §4.3) over that epoch's history, pity snapshot and banner record, and lists one `PASS` or `FAIL <reason>` line per record. **Export** writes the same inputs to a folder the player chooses, for the command-line tool.

The panel sells nothing, offers nothing and shows no timer (P9). A Sealed Draft queued offline (doc 15 §4.5) shows "Waiting to sync" with the client seed it was queued with.

### 11.9 Online sessions

Every online session is a Steam lobby (doc 16 §3.5). Players find each other through Steam friends, Steam invites and the lobby browser; there are no friend codes and no crossplay setting, because Steam is the only storefront (DECISIONS 112).

- **Lobby browser** (`ui.menu.lobby_browser`; DECISIONS 74, doc 16 §3.3). "Join a party" on the Lectern's `Idle` screen opens it. Friends' joinable lobbies come first, then open `net.lobby.public` lobbies with the player's `pal_ver`, `pal_proto` and `pal_ch`, filtered by the mode and Grade the player sets and sorted by ascending ping estimate (ties to the lower host), 50 per page. Each row shows mode, Grade, members `n / 4`, "In lobby" or "In run, Folio f", the ping estimate in ms, and the host's mod tag (`pal_mods`, the first 16 hex digits of `modset_hash`) when the host requires mods. Lobbies hosted by or holding a blocked player are absent. Buttons: Join; Quick Join (doc 16 §3.3); Host, with the lobby type private, friends (the default) or public; LAN, listing hosts whose beacon arrived in the last 3 s (doc 16 §3.4); and Direct, with an address, a port defaulting to 27310 and the host password. A refused join shows its `net.reject.*` reason text (doc 18 keys).
- **Mod sync** (`ui.menu.mod_sync`; DECISIONS 126, doc 02 §13.4, doc 16 §11.8). When a joiner lacks the host's required mod set (the simulation-affecting mods inside `content_hash`), this page lists each required mod by name and version with a progress bar: a Steam Workshop item downloads through Steam, and a data-only pack of at most 64 MiB transfers from the host. The join continues by itself once every row completes and the `content_hash` matches. Cancel leaves the lobby, and a row that cannot complete names its reason and stops the join. Presentation-only mods are never synced, since peers may differ in them (DECISIONS 87). The page states that a run with a simulation-affecting mod is Apocrypha (doc 15).
- **Cosmetic difference.** When the client's own `presentation_hash` differs from the host's `Welcome.presentation_hash` (doc 16 §3.8), the notification list shows once per session: "Your cosmetic content differs from the host's. Others may see different art, sound, text or UI; play is identical." (Canon §2.4, DECISIONS 87).
- **Voice** (DECISIONS 75; doc 15 §8.5, doc 16 §9.12). Settings → Social sets the voice mode (`net.voice.off`, `net.voice.party` as the default, or `net.voice.proximity`) and push to talk. When a member joins, the client mutes that member's voice unless the member is on the player's Steam friends list; that member's party frame and scoreboard row then show a struck speaker glyph, and "Unmute voice" on the scoreboard row (2 inputs) unmutes the member for the session. Text and pings are unaffected. Each change sends doc 16's `VoiceMute`.
- **Kick** (doc 16 §9.2). The host kicks from any member's scoreboard or lobby row with one confirmation. In a public lobby, and in the run it starts, any member may start a kick vote from another member's row ("Vote to kick", one confirmation); the starter's vote counts as yes, and one vote runs at a time. While it runs, the notification list pins "Remove <persona>? Yes y of need · M:SS", where `need = max(2, m − 1)` and `m` counts the Active members other than the target, counting down `KICK_VOTE_TICKS` (1,800 ticks). Every other member answers Yes or No on the target's scoreboard row within 2 inputs, which sends `net.msg.kick_vote { target_slot, yes }`. In a party of 2 the vote is not offered, because `m = 1` and `need = 2` (only the host kicks there). A kicked player sees "You were removed from this session" and cannot rejoin it.
- **Connection problem.** After `NET_STALL_MS` (1,000 ms) without a snapshot the client is `stalled` (doc 16 §4.1): the notification list pins "Connection problem: waiting for the host · N s" in the `hostile` token, counting whole seconds since the last snapshot, until a snapshot arrives. Prediction stops after `PRED_STALL_TICKS` (120 ticks), and from then the crosshair shows a stopped-quill glyph. Remote entities with no new sample for 6 ticks render as stale (§16.1).
- **Host migration** (`ui.hud.net_status`; DECISIONS 93, doc 16 §9.3). While the client is `migrating`, a panel at the §10.12 overlay rectangle reads "The host left. Finding a new host… N s". On `MigrateAnnounce` it names the new host and the mode: for `net.migrate.hot` and `net.migrate.warm`, "<persona> is the new host. The Folio continues.", with "Catching up M:SS" from `eta_ms` while it is above 0; for `net.migrate.cold`, "<persona> is the new host. This Folio restarts from its entry." The claiming successor reads "You are the new host. Catching up…". The panel closes when play resumes. When the wait ends with no announcement, it reads "The session ended. Your run resumes from the start of this Folio." and returns to the Margin (doc 14 §12.2). An invader instead sees "The host left: the invasion is over" (reason 6, doc 16 §9.13). The panel draws above the lock, hack and pickpocket panels, which keep their state, and pauses nothing (P8).

### 11.10 Second Edition (DECISIONS 100 to 108, 118)

Second Edition ships at Early Access (DECISIONS 118). Doc 14 §8.9 owns its flow, doc 12 its fused Folios and doc 13 its revised Editors and the Author.

- **Door.** After an Ur-Text victory in `mode.standard` or `mode.abridged`, `VictoryChoice` shows the Margins doors (Standard only, once unlocked, doc 14 §9.1), then the "Second Edition" door, then "Close the Book"; an Abridged victory shows only "Second Edition" and "Close the Book", and Seeded and Custom runs never show the door (doc 14 §3.2, §8.9). Before the door can be committed, its panel (§9.1) shows the stakes in four lines: "15 Folios of fused genres in 3 Volumes, each closed by a revised Editor; the Author waits at the end." / "Your Errata continue with their gear and levels, and fallen allies return as they were." / "Any end but victory forfeits this run's clear, Abandon included." / "Clear it with no death for the Ironman frame and mark." The door commits by the vote of §9.1 like any door, so nobody enters without the party choosing it.
- **HUD.** The Folio header (§3.2) reads "Second Edition · Folio n/15" with Volume VI, VII or VIII (doc 14 §3.1) and shows both fused genres' sigils split diagonally, as a bleed zone does (DECISIONS 101). The active skin follows §2.1 with doc 12's `local_genre`; where §2.1 falls back to the Folio genre, a fused Folio uses the first genre of doc 12's pair.
- **Run summary** (§9.7). A Second Edition run ending in anything but victory shows "Second Edition: clear forfeited" and its score with `victory_bp = 10000`; a victory shows "Second Edition cleared" and the `max(score_at_victory, score_now)` line (doc 14 §4.2). An Ironman clear (`RunSummary.ironman`) adds "Ironman: no Erratum died in Second Edition" and, on the account's first one, the grant line for `cosm.frame.ironman`.
- **Ironman mark and frame.** Leaderboard rows at the Ledger (doc 14 §9.4) show `ui.icon.ironman` beside an entry whose `RunSummary.ironman` is true. `cosm.frame.ironman` (doc 15 §2.2) is a portrait frame (`cosm.cat.frame`): the Wardrobe lists it once owned, with its source "Second Edition Ironman clear", and when equipped it borders the player's portrait on lobby slots and scoreboard rows (§11.6).

---
## 12. Controls

### 12.1 Action list

Every action is an `input.*` record. Kind: `B` button, `A2` two-axis. Sim column: the `PlayerCommand` field it drives (doc 02 §5.5 bits, §12.2 fields) or the intent it sends; "client" actions never reach the simulation.

| Action | Kind | Sim | Action | Kind | Sim |
|---|---|---|---|---|---|
| `input.move` | A2 | `move_x`, `move_z` | `input.look` | A2 | `yaw`, `pitch` |
| `input.attack` | B | bit 0 | `input.alt` | B | bit 1 |
| `input.reload` | B | bit 2 | `input.jump` | B | bit 3 |
| `input.crouch` | B | bit 4 | `input.sprint` | B | bit 5 |
| `input.dodge` | B | bit 6 | `input.interact` | B | bit 7 |
| `input.heavy` | B | bit 8 (`MELEE_QUICK`) | `input.cast` | B | bit 9 |
| `input.throw` | B | bit 10 | `input.weapon_next`, `input.weapon_prev` | B | bits 11, 12 |
| `input.weapon_1..=3` | B | bits 13–14 value 1..=3 | `input.quickbar_1..=8` | B | bits 15–17 index, bit 18 use |
| `input.grimoire_next`, `input.grimoire_prev` | B | bits 19, 20 | `input.lean_l`, `input.lean_r` | B | bits 21, 22 |
| `input.ping` | B | bit 23 | `input.walk` | B | bit 24 |
| `input.ability_1..=4` | B | `ACTION` 1..=4 | `input.cast_cancel` | B | `ACTION` 5 |
| `input.cast_wheel` | B | `WHEEL_SELECT` on release | `input.speak_vitals` | B | client (§13.6) |
| `input.pray` | B | `Pray` intent | `input.examine` | B | `Examine` intent |
| `input.minion_follow`, `input.minion_hold`, `input.minion_attack` | B | `MinionCommand` intent | `input.glyph_layer` | B | client (modifier) |
| `input.camera` | B | client (first/third person) | `input.menu_book`, `input.menu_map`, `input.menu_journal`, `input.menu_character`, `input.menu_exits` | B | client |
| `input.pause` | B | client | `input.scoreboard` | B | client |
| `input.chat`, `input.push_to_talk`, `input.emote_wheel` | B | client and doc 16 | `input.ui_confirm`, `input.ui_back`, `input.ui_tab_next`, `input.ui_tab_prev`, `input.ui_page_next`, `input.ui_page_prev`, `input.ui_details`, `input.ui_compare`, `input.ui_nav` (A2) | B / A2 | client |
| `input.look_gyro` | A2 | `yaw`, `pitch` | `input.hold_breath` | B | `SetBreathHold { on }` intent on press and release |
| `input.whistle`, `input.shout` | B | `Vocalize { kind }` intent (doc 09 §12) | `input.ui_actions` | B | client |

**Movement kit** (DECISIONS 95). Jump, crouch and sprint are the actions above. The other three parts have no action of their own, so they need no command bit, and bit 25 stays 0 (§12.2):

- **Ledge mantle**: an `input.jump` press facing a ledge (doc 03 §5.9, which reaches 2,000 mm above the feet per DECISIONS 95 and Canon §4.2).
- **Sprint-slide**: an `input.crouch` press (bit 4 set where the previous command had it clear) on a command that also holds `input.sprint` (bit 5) while the Erratum sprints (doc 01 Q-25). Its bindings are the crouch bindings while sprinting: Left Ctrl (§12.3) and R3 (§12.4). Doc 03 owns the motion; until it specifies the slide (doc 03 G31), the press is an ordinary crouch press.
- **Grappling tool**: a genre-skinned doc 11 item, used from the quickbar (`input.quickbar_1..=8`) or the quick wheel like any quickbar item, never part of the base kit.

Contexts (`input.ctx.*`) decide which bindings are live. When several apply, the one with the lowest rank wins: `menu`, `wheel`, `lockpick`, `placement`, `casting`, `ghost`, `downed`, `margin`, `on_foot`. A binding may repeat across contexts but never within one (§12.8).

| Context | Rank | Active when | Bindings it changes |
|---|---|---|---|
| `input.ctx.menu` | 1 | any `ui.menu.*` screen, `ui.hud.hack` or `ui.hud.pickpocket` is open | the menu bindings of §12.3 and §12.4 (`input.ui_*`, `input.examine`) |
| `input.ctx.wheel` | 2 | a wheel is open (§6.5) | the right stick or the mouse selects a socket; releasing the input that opened the wheel commits the selection |
| `input.ctx.lockpick` | 3 | `ui.hud.lockpick` runs a manual lock or hack attempt (§10.11, §10.12) | only `input.attack` (set the pin), `input.ui_back` (abandon) and look |
| `input.ctx.placement` | 4 | placement mode (§6.2) | `input.attack` confirms, `input.alt` cancels, `input.weapon_next` / `input.weapon_prev` rotate by 16384 Bam |
| `input.ctx.casting` | 5 | the local cast machine, as the client last knows it, is in Winding, Overcasting, Channeling or Charging (doc 06 §3.5) | gamepad base-layer B is `input.cast_cancel`; a second B within 18 frames dodges (§12.4) |
| `input.ctx.ghost` | 6 | the local Erratum is a ghost (§10.4) | `input.attack` marks, `input.interact` opens doors and presses switches and levers, `input.ping` pings, `input.weapon_next` / `input.weapon_prev` spectate, `input.jump` returns to the ghost body |
| `input.ctx.downed` | 7 | the local Erratum holds `st.downed` (§10.4a) | only look, crawl (`input.move`), `input.ping` and `input.chat` |
| `input.ctx.margin` | 8 | the local player is in the Margin hub (§11.1) | `input.menu_book` opens `ui.menu.margin_index` instead of `ui.menu.book` |
| `input.ctx.on_foot` | 9 | no other context applies | the preset's base bindings for every §12.1 action |

### 12.2 Command encoding

Docs 04 and 06 delegate the ability and wheel bits to this chapter, which assigns bits 26–31 (triage C-10: doc 02 §5.5 lists these fields, and doc 16's sanitizer replaces a command whose `ACTION` is 6 or 7). Bit 25, the retired mount toggle, has no action and stays 0:

| Bits | Field | Values |
|---|---|---|
| 26–28 | `ACTION` | 0 none; 1..=4 `input.ability_1..=4`; 5 `input.cast_cancel`; 6, 7 reserved (the sanitizer rejects them) |
| 29–31 | `WHEEL_ENTRY` | 0..=7, meaningful only together with `WHEEL_SELECT` |
| 19 and 20 together | `WHEEL_SELECT` | both set in the same command signals a wheel selection; the client never sets both from the Grimoire keys |
| 25 | none | always 0: the mount toggle is retired (Canon §17.7), and doc 16's sanitizer replaces a command with it set |

Press semantics follow doc 02: the simulation compares with the previous command. An `ACTION` press is a change to value `k ≠ 0` from any other value. A wheel selection is a command with bits 19 and 20 both set whose predecessor did not have both set; it sets the active spell to wheel entry `WHEEL_ENTRY`. For an instant or cast_time entry the client also sets `CAST` (bit 9) in that command, so selection and cast land on the same tick (doc 06 §3.2); a channel or charge entry is only selected, and the player holds `input.cast`.

```rust
fn encode(st: &ActionState) -> u32 {
    let mut b = 0u32;
    for (bit, act) in HELD_BITS { if st.held(act) { b |= 1 << bit; } }          // bits 0..=12, 21..=24; bit 25 stays 0
    b |= (st.weapon_select as u32 & 0b11) << 13;
    b |= (st.quickbar_index as u32 & 0b111) << 15;
    if st.quickbar_use { b |= 1 << 18; }
    if st.wheel_commit_this_tick {
        b |= (1 << 19) | (1 << 20) | ((st.wheel_entry as u32 & 0b111) << 29);
        if st.wheel_entry_casts_instantly { b |= 1 << 9; }
    } else {
        if st.held(GRIMOIRE_NEXT) { b |= 1 << 19; }
        else if st.held(GRIMOIRE_PREV) { b |= 1 << 20; }                          // never both
    }
    b |= (st.action_field as u32 & 0b111) << 26;   // last-pressed held ability, 5 while cast_cancel held
    b
}
```

A Grimoire press needs its bit set, the same bit clear in the predecessor, and the command not to carry bits 19 and 20 together, so a wheel selection never also reads as a Grimoire step, on its own tick or the next (doc 06 counts presses this way; Interface assumptions).

`action_field` is `k` for the most recently pressed ability key `input.ability_1` to `input.ability_4` while it stays held (5 while `input.cast_cancel` is held); releasing that key sets it to 0 even if another ability key is still held, so a release never re-triggers another ability.

Worked encodings: ability 3 held alone: `3 << 26 = 201,326,592`. Cast cancel: `335,544,320`. Wheel entry 5 selected with an instant cast: `(5 << 29) | (1 << 20) | (1 << 19) | (1 << 9) = 2,685,927,936`. Wheel entry 0 selected without a cast: `1,572,864`. Quickbar slot 3 (index 2) used: `327,680`. Weapon slot 2: `16,384`.

### 12.3 Keyboard and mouse (`input.preset.folio_kbm`)

| Action | Binding | Action | Binding |
|---|---|---|---|
| move | W A S D (arrows secondary) | look | mouse |
| attack | Mouse 1 | alt (block, ADS) | Mouse 2 |
| heavy | Mouse 4, V | cast | Mouse 5, T |
| cast wheel | Tab (hold) | cast cancel | ` (grave) |
| reload | R | throw | G |
| interact | F | jump | Space |
| crouch | Left Ctrl (hold) | sprint | Left Shift (hold) |
| sprint-slide | Left Ctrl pressed while sprinting (§12.1) | grappling tool | its quickbar key (§12.1) |
| walk | Caps Lock (hold) | dodge | Left Alt |
| lean left, right | Q, E | speak vitals | unbound |
| weapon 1–3 | 1, 2, 3 | weapon next, prev | wheel down, wheel up |
| quickbar 1–8 | 4, 5, 6, 7, 8, 9, 0, − | grimoire next, prev | ], [ |
| ability 1–4 | Z, X, C, B | ping | Mouse 3 (a second press within 15 ticks makes it a danger ping, doc 16 §9.12) |
| minion follow, hold, attack | F1, F2, F3 | pray | J (hold 20 frames: prayer menu) |
| camera | F5 | Book menu | I |
| character | K | journal | L |
| map | M | exits list (in a Colophon) | N |
| scoreboard | O (hold) | emote wheel | U (hold) |
| chat | Enter | push to talk | Y |
| pause | Esc | UI details, compare | Left Alt (in menus), Left Shift (in menus) |
| hold breath | P (hold) | whistle, shout | emote wheel sockets 0, 1 |
| item actions (menus) | Mouse 2, F | examine (menus) | X |

Menu context: arrows or W A S D move focus, Enter or Space confirm, Esc back, Q / E tabs, PageUp / PageDown pages, Left Alt details (hold), Left Shift compare (hold), Mouse 2 or F item actions, X examine, mouse everywhere. Lockpick context: Mouse 1 sets a pin, Esc abandons. `input.preset.left_hand_kbm` mirrors the layout onto I J K L with the numeric keypad for one-handed play.

Mouse look: `yaw_deg = counts × 22 × sens_milli / 1,000,000` (sens 1000 = 0.022° per count), default 1000, range 100..=10000; separate ADS multiplier 2500..=20000 Bp (default 10000); invert Y off. Each frame's look delta in radians enters doc 02 §10's look state through `look_add` (doc 02 §7.5), and each command's `yaw` and `pitch` are quantized from that state, so look never drifts and the replay holds exactly the Bam values the player aimed with.

### 12.4 Gamepad (`input.preset.folio_pad`)

Xbox names; PlayStation glyphs map Cross = A, Circle = B, Square = X, Triangle = Y, L1/R1 = LB/RB, L2/R2 = LT/RT, Create = View, Options = Menu. Glyphs follow the connected device through Steam Input (doc 02 §14).

| Input | Base layer | Glyph layer (hold LB) |
|---|---|---|
| Left stick | move | move; X-axis beyond 5000 Bp leans left or right (stick move then strafes 0) |
| L3 | sprint (toggle) | unbound |
| Right stick | look | look; selects wheel sectors while a wheel is open |
| R3 | crouch (toggle); pressed while sprinting: sprint-slide (§12.1) | camera (tap); hold breath (hold 15 frames) |
| RT | attack | cast |
| LT | alt (block, ADS) | cast wheel (hold; release selects) |
| RB | heavy | throw (hold to cook) |
| A | jump | ability 1 |
| B | dodge | ability 2 |
| X | reload | ability 3 |
| Y | interact | ability 4 |
| D-pad up | ping (a second press within 15 ticks: danger, doc 16 §9.12); hold 15 frames: emote wheel | pray (hold 20 frames: prayer menu) |
| D-pad down | use selected quickbar slot; hold 15 frames: quickbar wheel | minion attack; hold 20 frames: minion command wheel (follow, hold, attack, per-minion) |
| D-pad left, right | weapon prev, next; hold 15 frames: weapon wheel | grimoire prev, next |
| View | Book menu; hold 15 frames: map | push to talk (hold) |
| Menu | pause | scoreboard (hold) |

`input.preset.spellwright_pad` swaps RT and LB+RT (RT casts, LB+RT attacks) and swaps LT and LB+LT, for casters. `input.preset.southpaw_pad` swaps the sticks. Wheels opened from the gamepad select by the right stick.

**Casting context** (`input.ctx.casting`): base-layer B is `input.cast_cancel` instead of dodge; a second B press within 18 frames dodges even if the client has not yet seen the cast end. Every other weapon, item, interact, sprint, jump or dodge input already cancels a cast (doc 06 §3.5), so B is the one pure cancel. **Whistle and shout** sit in the emote wheel (D-pad up, hold 15 frames), sockets 0 and 1.

**Menu context** (`input.ctx.menu`):

| Input | Action |
|---|---|
| D-pad, left stick | `input.ui_nav` |
| A | `input.ui_confirm` |
| B | `input.ui_back` |
| X | `input.ui_actions`: the focused item's action list (Use, Equip, Drop, Drop for ally, Examine, Split, Assign) |
| Y (hold) | `input.ui_details` |
| L3 (hold) | `input.ui_compare` |
| LB, RB | `input.ui_tab_prev`, `input.ui_tab_next` |
| LT, RT | `input.ui_page_prev`, `input.ui_page_next` (also the map's Y-slice) |
| Right stick | scroll lists; orbit 3D previews and the map |
| Menu | close every menu |

The Book menu's tabs replace `input.menu_character`, `input.menu_journal` and `input.menu_exits` on gamepad; those three are unbound there by default and bindable.

Stick processing: radial inner deadzone 1200 Bp (range 0..=3000), outer 9500 Bp (8000..=10000), rescaled linearly between them, then `stick_to_i8` (doc 02 §7.5). Look curve options `linear`, `classic` (square of deflection, default), `precise` (cube), each applied to the rescaled deflection `r ∈ 0..=1` as `r`, `r²` or `r³` along the stick's direction. Look speed at full deflection: yaw 240°/s, pitch 160°/s (each 60..=600). Vibration 0..=10000 Bp (default 7000). Trigger actuation 2000..=8000 Bp (default 3000).

### 12.5 Steam Deck (`input.preset.folio_deck`)

The gamepad preset plus: L4 lean left (hold), R4 lean right (hold), L5 glyph layer (same as LB), R5 ping. Right trackpad: mouse-like look with click as attack disabled (`input.look` as mouse region, sensitivity 1000). Left trackpad: radial quickbar (8 sectors, click to use). The touchscreen works in every menu (tap confirms, drag scrolls, two-finger drag orbits the map). Text fields open the Steam on-screen keyboard. Defaults on Deck: aim assist Standard, gyro "while ADS" (§12.6), HUD scale 10000, text scale 11000.

### 12.6 Gyro

Gyro reaches the game as `input.look_gyro` through Steam Input's gyro-to-camera mapping, so it is a look delta like the mouse. Shipped Steam Input defaults for gyro-capable pads (Steam Deck, DualSense, DualShock 4, Switch Pro): activation **while LT ≥ 5000 Bp** (aiming), real-world scale 1:1 (1° of controller rotation turns the camera 1° at 90° FOV), smoothing below 5°/s, a right-stick touch "ratchet" off. Options: `always`, `while_aiming` (default), `while_touching_right_pad`, `off`. Aim assist never applies to gyro deltas unless `input.cfg.aim_assist_gyro` is on, which allows Low only.

### 12.7 Aim assist

Aim assist is a client-side filter on stick look deltas; its output is the command's `yaw` and `pitch`, so replays reproduce it without a device field, and it adds no in-simulation magnetism. It never sets a button bit and never moves the camera without stick deflection, except the ADS snap, which runs only on the player's own ADS press. It never bends projectiles.

| Level (`access.aim_assist`) | Slowdown factor | Cone half-angle | Tracking share | ADS snap | Max range |
|---|---|---|---|---|---|
| `off` | 10000 | — | 0 | none | — |
| `low` | 8000 | 455 Bam (2.5°) | 0 | none | 40,000 mm |
| `standard` (gamepad default) | 6000 | 455 Bam | 2500 | none | 40,000 mm |
| `high` | 5000 | 637 Bam (3.5°) | 4000 | 5000 of the angle to the nearest precision part within 1092 Bam (6°), over 8 frames | 40,000 mm |
| `max` (assist level) | 4000 | 819 Bam (4.5°) | 6000 | 8000 within 1820 Bam (10°), over 6 frames | 60,000 mm |

```
eligible(t) = t is hostile ∧ rendered ∧ not culled ∧ client voxel ray from the eye to t's nearest part is clear
              ∧ dist ≤ max_range ∧ (t not st.invisible ∨ dist ≤ 2000 mm)
inside      = ∃ eligible t: angle(aim, nearest part center of t) ≤ cone
slow        = if inside { 10000 − mul_bp(10000 − slowdown, weapon.aim_assist_bp) } else { 10000 }
look_rate   = stick_rate × slow / 10000
θ(t)        = bearing (yaw, pitch in Bam) from the eye to the nearest part center of the closest eligible t, this frame
ω           = θ(t) − θ_prev(t)                         // wrapped Bam; 0 on the first frame t is tracked
if inside ∧ |stick| ≥ 2000 Bp ∧ sign(ω.yaw) = sign(stick.x):
    camera.yaw += mul_bp(ω.yaw, mul_bp(tracking, weapon.aim_assist_bp))   // likewise pitch with stick.y
```

The weapon factor scales how much of the assist applies: at `aim_assist_bp = 10000` the table values apply exactly, at 0 there is none. Tracking only helps a stick already moving the same way as the target, so it never pulls an idle or opposing stick.

`aim_assist_bp` comes from doc 05's archetype record (10000 when absent). Against a human-controlled target (an invader, or Errata seen by an invader), tracking share and snap are 0 and the slowdown is at most the `standard` value. Mouse input never receives aim assist.

### 12.8 Rebinding

- Every action is rebindable per context and per device family (keyboard-mouse, gamepad). Up to 2 bindings per action per device family. A binding is a key or button plus, when the player sets them, one modifier (a held key, or `input.glyph_layer` on gamepads), a `tap` or `hold N frames` qualifier, and a `double-tap within N frames` qualifier.
- **Conflicts**: a new binding equal to an existing one in the same context is refused with "Already bound to <action>" and a Swap button. Tap and hold variants of one key may coexist only when the tap action tolerates the hold delay: the tap fires on release before the hold threshold. `input.attack`, `input.alt`, `input.dodge`, `input.jump` and `input.heavy` may not be tap/hold-shared because their latency matters.
- **Never unbindable**: `input.pause` and `input.ui_back` keep at least one binding on every device family; the Settings screen is reachable from the pause menu on every device. "Reset to preset" is always one input on the Controls page.
- Bindings save to the client profile file `settings/input.json` (not content) with the preset ID they started from.

### 12.9 Toggle versus hold

The client turns toggles into held bits each tick, so the simulation always sees held state (Canon §11.8: input is never synthesized beyond what the player chose).

| Action | KBM default | Pad default | Toggle releases when |
|---|---|---|---|
| crouch | hold | toggle | pressed again; sprint, jump or dodge pressed |
| sprint | hold | toggle | pressed again; movement input below 2000 Bp for 6 frames; any attack, ADS or cast |
| walk | hold | not bound | pressed again |
| alt as ADS | hold | hold | pressed again; weapon swap; sprint, except that sprint while fully aimed with a weapon whose `ads_sway ≥ 27` holds breath (doc 05 §6.7) and keeps ADS |
| alt as block | hold | hold | pressed again; dodge |
| lean | hold | hold | opposite lean; sprint; lean unavailable |
| heavy charge | hold | hold | pressed again (releases the charge) |
| cast (channel, charge) | hold | hold | pressed again; the cast ends |
| throw cook | hold | hold | pressed again (throws) |
| interact holds (revive, open, tame, rest) | hold | hold | pressed again; target lost; the channel completes |
| glyph layer | hold | hold | pressed again; after 120 frames |
| push to talk | hold | hold | pressed again |
| hold breath (`input.hold_breath`) | hold | hold | pressed again; `res.breath` reaches 0 (doc 09 §4.6) |

A crouch press while sprinting leaves the sprint toggle on, so the command carries bits 4 and 5 together and starts the sprint-slide (§12.1). The danger ping needs no client double-tap detection: every ping press sets bit 23, and the simulation turns a `PING` press within `ping_danger_ticks` (15, doc 16 `net.params.default`) of the same slot's previous one into a danger ping (doc 16 §9.12).

`access.hold_to_toggle` sets every row to toggle at once. Timed interacts never shorten: a 90-tick Reliquary open still takes 90 ticks; the toggle only removes the need to keep the button down.

### 12.10 Device settings (`input.cfg.*`)

Client settings on the Controls page (§11.6), integers only. They shape look deltas and rumble on the client and never reach the simulation except as the `yaw` and `pitch` they produce.

| Setting | Values | Default | Rule |
|---|---|---|---|
| `input.cfg.mouse_sens_milli` | 100..=10000 | 1000 | `yaw_deg = counts × 22 × sens_milli / 1,000,000` (§12.3) |
| `input.cfg.ads_mult_bp` | 2500..=20000 | 10000 | multiplies mouse sensitivity while ADS (§12.3) |
| `input.cfg.invert_y` | off, on | off | negates the vertical look delta (§12.3) |
| `input.cfg.deadzone_inner_bp` | 0..=3000 | 1200 | radial inner deadzone (§12.4) |
| `input.cfg.deadzone_outer_bp` | 8000..=10000 | 9500 | radial outer deadzone; deflection rescales linearly between the two (§12.4) |
| `input.cfg.look_curve` | `linear`, `classic`, `precise` | `classic` | `r`, `r²` or `r³` of the rescaled deflection (§12.4) |
| `input.cfg.look_yaw_dps` | 60..=600 | 240 | stick yaw speed at full deflection, °/s (§12.4) |
| `input.cfg.look_pitch_dps` | 60..=600 | 160 | stick pitch speed at full deflection, °/s (§12.4) |
| `input.cfg.vibration_bp` | 0..=10000 | 7000 | multiplies every `ui.rumble.*` motor value (§16.3) |
| `input.cfg.trigger_bp` | 2000..=8000 | 3000 | trigger actuation point (§12.4) |
| `input.cfg.gyro_mode` | `always`, `while_aiming`, `while_touching_right_pad`, `off` | `while_aiming` | gyro activation (§12.6) |
| `input.cfg.aim_assist_gyro` | off, on | off | when on, gyro deltas receive aim assist at `low` at most (§12.6, §12.7) |

---
## 13. Accessibility

### 13.1 Scribe's Mercy (`access.scribes_mercy`)

Scribe's Mercy copies Hades' God Mode exactly (R3 [VERIFIED]): **20% damage resistance when first enabled, +2% for every death while it is enabled, capped at 80%, toggled at any time, and it never blocks achievements.**

**Account state** (meta profile, synced with the economy service and kept offline): `mercy_enabled: bool` (default false), `mercy_bp: i32`, set to 2000 on the first enable and kept forever after, including while disabled; re-enabling resumes at the stored value.

**Run state** (simulation, per party slot, saved and replayed; doc 03 `comp.player`): `mercy_on: bool`, `mercy_bp: i32`, copied from the account at run start or join. Applying `SetScribesMercy { on: true }` sets `mercy_on = true` and `mercy_bp = max(mercy_bp, 2000)`, so the first enable inside a run starts at 20% exactly as one in the Margin does; `on: false` clears `mercy_on` and keeps `mercy_bp`.

- **Toggle**: Settings → Accessibility → Scribe's Mercy, also on page 1 of the pause menu (2 inputs, P6). In a run it sends `SetScribesMercy { on }` (Interface assumptions), applied at the start of its tick like any intent. Out of a run it changes the account directly. No confirmation dialog and no warning copy.
- **Death counting**: when the slot's Erratum reaches Canon §10.8 step 3 (run-ending death or ghosting), or is the last Erratum alive and not downed that an invader reduces to 0 HP, which Redacts the party (Canon §17.9), on a tick where `mercy_on` is true, `mercy_bp = min(8000, mercy_bp + 200)` for that slot. One increment per Erratum per tick. Downing and a successful Death's Door are not deaths. At run end the account takes the slot's `mercy_bp`.
- **Damage step 6c** (Canon §10.5 step 6c, after step 6b and before ward absorption). It applies to every Canon §10.6 entry point an Erratum can receive, because each enters at or before step 6 (the lethal takedown enters at step 7 and targets only NPCs, doc 09 §9.4), and to every other involuntary HP loss of the bearer: starvation's 1% per 5 s is reduced by the same `mul_bp` before it is subtracted, and overdose and suffocation already enter the pipeline. Voluntary HP costs the bearer chooses to pay (doc 06 self-costs, `mfr.gothic_horror.grimhallow`'s reload cost) are never reduced, as in Hades, where God Mode reduces damage taken and nothing else:

```rust
// step 6c: Scribe's Mercy. Target is an Erratum whose slot has mercy_on.
if target.mercy_on {
    for t in DMG_TYPES_CANON_ORDER {                       // Canon §10.1 order
        d[t] = d[t] - mul_bp(d[t], target.mercy_bp);
    }
}
// step 7 (ward) and step 8 (apply) follow unchanged; ev.damage_dealt reports the reduced values
```

Worked examples: a 10,000 mHP hit at 2000 Bp leaves 8,000. After 3 deaths (2600 Bp) a 12,345 mHP slash hit leaves `12,345 − 3,209 = 9,136`. At the cap a 10,000 mHP hit leaves 2,000. The cap is reached after 30 deaths (`2000 + 30 × 200 = 8000`); the 31st death leaves 8000. While downed, the bleed shortening uses the reduced total: a 17,000 mHP hit at 2600 Bp on Max HP 85,000 shortens bleed-out by `ceil_div(12,580 × 1800, 85,000) = 267` ticks instead of 360.

- **Never blocks**: achievements, Codex XP, Ink, Quires, Reliquaries, milestones, Recollections and Grade unlocks are all unaffected (DECISIONS 82; doc 14 §7.4 and §10 follow, triage C-04). Ranked leaderboard submissions exclude runs in which `mercy_on` was true on any tick (doc 14 §9.4); the run summary says "Unranked: Scribe's Mercy was on" and nothing else changes.
- **Co-op**: per player; other players see a small quill on that player's party frame. An invader cannot enable it; Errata keep theirs during an invasion.
- **HUD**: a quill glyph and the percent beside the HP bar ("Mercy 26%").
- **Hallucinations**: while `mercy_on`, `access.mark_hallucinations` is forced on (DECISIONS 49, §13.10).

### 13.2 Subtitles and captions

| Setting | Range | Default |
|---|---|---|
| `access.subtitles` | off, dialogue, dialogue and captions | dialogue and captions |
| `access.subtitle_size_bp` | 10000..=20000 step 2500 (base 20 px) | 10000 |
| `access.subtitle_bg_bp` | backplate opacity 0..=10000 | 7500 |
| `access.speaker_names` | on, off | on (name in the speaker's party or faction color plus the name text) |
| `access.caption_direction` | on, off | on |

- The box is 592 × 100 reference px (inner 560 × 96 after padding) at every subtitle size `k = access.subtitle_size_bp / 10000`; text is `round(20 × k)` px with a line height of `24 × k`. A line holds at most `floor(42 / k)` characters for Latin, Cyrillic and Greek scripts and `floor(21 / k)` for CJK, and never wider than 560 px measured; lines break greedily at Unicode line-break opportunities (UAX #14).
- A block is at most 2 lines. Lines visible at once are `floor(96 / (24 × k))` (4, 3, 2, 2, 2 at `k` = 1.0, 1.25, 1.5, 1.75, 2.0), so 2 blocks show at `k = 1.0` and 1 block at every larger size; the newest is at the bottom.
- Display time per block: `max(audio_share + 500, ms_per_char × chars, 1500)` ms, capped at 9000, where `chars` counts the block's characters without line breaks, `ms_per_char` is 67 (Latin, Cyrillic, Greek) or 111 (CJK), and `audio_share = floor_div(audio_ms × chars, total_chars)` splits a line's audio across its blocks in proportion to their length.
- **Sound captions** come from doc 18 `aud.*` records that carry a caption key, for example "[Heavy footsteps]" or "[A gun jams]". With `access.caption_direction`, captions of positional sounds append an arrow from the `ev.noise` arrival direction for the local slot: ←, ↖, ↑, ↗, →, ↘, ↓, ↙ by 8192-Bam sectors relative to the camera yaw, or "(behind)".
- Captions use only what the local player can hear: noises with a `heard` entry for the local slot (doc 03).

### 13.3 Color-blind support

`access.colorblind_mode`: `none`, `deuteranopia`, `protanopia`, `tritanopia`, `monochrome`. Modes swap the semantic and rarity tokens (§2.2); damage types keep their colors and always carry glyphs. `access.world_filter` (off by default) adds a matching daltonization LUT after the genre grade (doc 02 §8.6 post stack); it never changes the simulation. Pings, markers and nameplates always carry shapes (§2.5), and doc 05 §18's hit markers carry shapes too: normal a plain cross, precision a cross with a center dot, crit a cross with four outward ticks, ward a hexagon, blocked a shield, high armor reduction a dented cross, kill a thick cross. Every signal therefore survives `monochrome`.

### 13.4 Text, contrast and fonts

- `access.ui_scale_bp` 7500..=20000 step 500 (menus reflow; the HUD uses `access.hud_scale_bp` 7500..=10000 step 500, or `access.hud_layout = large` for 125%, §3.1).
- `access.text_scale_bp` 10000..=20000 step 1000 applies to menus, item cards, tooltips and the Journal; the HUD follows `access.hud_scale_bp` and `access.hud_layout`, subtitles follow `access.subtitle_size_bp`; the §2.4 minimums always hold.
- `access.font`: standard, legible, dyslexic (§2.4).
- `access.high_contrast`: backplates opaque, 2 px outlines in `ink` around every HUD element, every text pair ≥ 7:1 (light skins already exceed 7:1 for `ink`; `accent` and `muted` text switch to `ink`).
- `access.consistent_skin`: every screen uses `ui.skin.margin` (§2.1).

### 13.5 Motion, camera and FOV

| Setting | Range | Default |
|---|---|---|
| `access.fov_deg` | 60..=120, horizontal at 16:10, Hor+ on wider screens (DECISIONS 92, triage Q10) | 90 (64.01° vertical; 96.03° horizontal at 16:9; 120 gives 94.54° vertical) |
| `access.viewmodel_fov_deg` | 45..=75 vertical | 54 (doc 02 §8.4) |
| `access.camera_shake_bp` | 0..=10000 | 10000; scales doc 05 §18 camera kick and explosion shake |
| `access.head_bob_bp` | 0..=10000 | 5000 |
| `access.reduce_motion` | on, off | off: the comfort preset of docs 02 and 18. It replaces the page-turn transition (doc 02 §8.8) with a 90-tick crossfade, sets lean roll to 0°, stops UI parallax and idle animations, turns off chromatic aberration, scanlines, film grain, vignette and Dutch angles (doc 02 §8.6), and forces doc 18's `anim_step` to 1 for every entity (doc 18 §3.3) |
| `access.center_dot` | on, off | off; a 4 px dot always at the screen center, including in menus over the world view (motion-sickness aid) |
| `access.third_person_distance_mm` | 1500..=4000 | 2500 |
| `access.hitstop` | on, off | on; off removes doc 05 hitstop visuals only |
| `access.gore` | `ink`, `full`, `minimal` (doc 18 §5.4): ink wounds and unravelling script on severed parts (the default), red blood, or no decals | `ink` (DECISIONS 94); `full` is the red-blood toggle, and `minimal` also drops mist and script while severing keeps its ink-cap |

### 13.6 Screen reader for menus

- Every widget (`PalButton`, `PalList`, `PalGrid`, `PalTabs`, `PalSlider`, `PalTooltip`, `PalItemCard`, `PalModal`) publishes an accessibility node through Bevy's AccessKit integration (`bevy_a11y`, which `bevy_winit` provides on Windows without a feature flag, doc 02 §2.2) with role, label, value, description and state, so Windows screen readers (Narrator, NVDA, JAWS) read menus. The client is Windows-only, with the Steam Deck through Proton (DECISIONS 98).
- `access.narrator` (off by default; Settings label "Read menus aloud", to keep it apart from doc 18's text-only Narrator) speaks the same strings through `tts` 0.26.3 over the Windows speech API (doc 02 §2.2) when no screen reader is running. When the OS reports no voice, the row reads "Unavailable on this system" and cannot be switched on. Rate 50..=300 words per minute, default 180.
- **Read order** of an item card: name, rarity, type line, Contradiction state, core stats, affixes, curse line, value, then "Details available" when a breakdown exists. Numbers are read in full words of the locale ("one hundred twenty seven hit points").
- `input.ui_details` on any focused number reads its breakdown (§5.6). In play, a "speak vitals" binding (unbound by default) reads HP, ward, stamina, aether and active statuses.
- Every icon has a label key (§2.5); every image-only button has a label; focus is never trapped; every modal announces its title on open.

### 13.7 Sound visualization (`access.sound_viz`)

Off by default. For every `ev.noise` with a `heard` entry for the local slot, an arc is drawn on a ring of radius 260 px around the crosshair at the arrival direction relative to the camera yaw:

```
rel     = arrival_yaw.wrapping_sub(camera_yaw)        // Bam; 0 = straight ahead, drawn at the top
width   = 4096 Bam
thick   = clamp(floor_div(heard_db - 20, 4), 2, 12)  // px
life    = 45 frames, fading linearly
color   = hostile for noises whose emitter is hostile, ally for allies and minions, neutral otherwise
glyph   = class of the noise ID's second segment (doc 09 §7):
          footstep ← step, move, surface;  voice ← voice;  weapon ← shot, melee, weapon, takedown;
          impact ← impact, explosion, block, body, trap;  magic ← spell, prayer;  alarm ← alarm;
          environment ← every other segment (env, door, lock, item, hide, node, light, hack, gadget, …)
```

Noises below 20 dB heard draw nothing. The ring shows only sounds the player can hear, so it never reveals more than audio does. Example: a footstep heard at 33 dB draws an arc `clamp(floor_div(13, 4), 2, 12) = 3` px thick; a gunshot heard at 95 dB draws 12 px.

`access.sound_ripples` (off by default) adds doc 18 §4.6's world-space ripples: each heard `ev.noise` draws a ring at its apparent position (doc 02 §9) of radius `min(3000, 50 × L_lis)` mm for 30 ticks.

### 13.8 Photosensitivity

- A startup notice on first launch and in Settings describes flashing content.
- `access.photosensitive` (off by default) applies: no full-screen flashes (ward break, crits, Shock of the New and lightning use edge glyphs instead); muzzle-flash and explosion light intensity × 5000 Bp; and a **flash limiter** on the final image: a flash is a pair of opposing changes of mean relative luminance, each at least 1000 Bp of full scale, over at least 2500 Bp of the screen area; after 3 flashes within any 1000 ms window, further luminance change in that window is limited to 500 Bp per 16.67 ms of frame time, scaled by the frame's duration (ITU-R BT.1702 thresholds [RECALLED]). The mode also acts as doc 18's flicker-off setting: nothing on screen alternates faster than 3 Hz, so a Disconnected item shows the static torn-page icon instead of doc 18 §4.1's 4 Hz flicker.
- Red saturation flashes (danger vignettes) are replaced by a static border under this mode.

### 13.9 Solo game speed (`access.game_speed_bp`)

Range 5000..=10000 step 1000, default 10000, solo only: co-op lobbies, and solo runs with `mode.invasion_open` (an invader may join at any time, Canon §17.9), lock it to 10000. It changes only the wall-clock rate at which the solo host steps ticks: `ticks per real second = 60 × game_speed_bp / 10000` (30, 36, 42, 48, 54, 60). The simulation, its tick counts, replays and saves are unchanged (Canon §2.1). Music plays at its normal tempo and pitch; one-shot effects play at normal pitch and speed when their events occur; looping sounds tied to simulation state keep their pitch. Achievements and rewards are unaffected, and nothing is gained by slowing down: Ink and Codex XP count active simulation ticks (doc 15 §2.2), so a slower speed pays the same per tick and less per real hour. A run in which the speed was below 10000 on any tick is unranked, as its summary states (§9.7).

### 13.10 Assist options

| Setting | Effect | Default |
|---|---|---|
| `access.scribes_mercy` | §13.1 | off |
| `access.hold_to_toggle` | §12.9 | off |
| `access.aim_assist` | §12.7 | gamepad `standard`, KBM `off` (inert for mouse) |
| `access.auto_sprint` | sprint stays on while moving forward | off |
| `access.dodge_double_tap` | double-tap a direction within 12 frames to dodge (KBM) | off |
| `access.wheel_dwell` | wheels commit on a 30-frame hover | off |
| `access.navigation_line` | a dotted ink line along a client-side path to the open Colophon (or the Incipit) through visited cells | off |
| `access.enemy_outline` | a 2 px `hostile` outline on rendered hostiles in line of sight; never through blocks | off |
| `access.shimmer_bp` | opacity of invisible stalkers beyond 2000 mm (doc 13) | 2500 (range 2500..=6000) |
| `access.minimap`, `access.minimap_rotate` | §8 | off, heading-up |
| `access.stealth_meter_always` | §10.10 | off |
| `access.damage_numbers` | off, own hits, all | own hits |
| `access.damage_numbers_mono` | damage numbers in `ink` with type glyphs | off |
| `access.tv_safe_area`, `access.hud_preset`, `access.hud_scale_bp`, `access.hud_opacity_bp`, `access.hud_backplate_bp` | §3 | off, full, 10000, 10000, 6000 |
| `access.mono_audio` | downmix to mono | off |
| `access.sound_ripples` | §13.7 world-space ripples | off |
| `access.hud_layout` | `standard`, `large` (§3.1, §3.4) | standard |
| `access.lock_mode` | `auto`, `manual` (§10.11) | auto |
| `access.dynamic_range` | wide, normal, night (compressed) | normal |
| `access.mark_hallucinations` | marks madness phantoms (doc 04 §12.4): each draws with a dashed `muted` outline and a "?" glyph above it, captions of the sounds it makes end in "(phantom)", and the Whispers episode's caption reads "[Whispers (madness)]"; forced on while the local slot's `mercy_on` is true (DECISIONS 49) | off |

Settings records (content, integers only):

```json
[
  { "id": "access.game_speed_bp", "type": "int", "min": 5000, "max": 10000, "step": 1000, "default": 10000,
    "scope": "solo_only", "affects": "wall_clock", "ranked_eligible_values": [10000] },
  { "id": "access.hud_scale_bp", "type": "int", "min": 7500, "max": 10000, "step": 500, "default": 10000,
    "scope": "client", "affects": "presentation" },
  { "id": "access.scribes_mercy", "type": "bool", "default": 0, "scope": "account_and_run", "affects": "simulation_step_6c",
    "ranked_eligible_values": [0] }
]
```

### 13.11 Remaining settings

The tables of §13.2, §13.5 and §13.10 give every other `access.*` setting's values and default. These settings complete the list:

| Setting | Values | Default | Section |
|---|---|---|---|
| `access.colorblind_mode` | `none`, `deuteranopia`, `protanopia`, `tritanopia`, `monochrome` | `none` | §2.2, §13.3 |
| `access.world_filter` | off, on | off | §13.3 |
| `access.ui_scale_bp` | 7500..=20000 step 500 | 10000 | §2.4, §13.4 |
| `access.text_scale_bp` | 10000..=20000 step 1000 | 10000; 11000 on Steam Deck (§12.5) | §2.4, §13.4 |
| `access.font` | `standard`, `legible`, `dyslexic` | `standard` | §2.4 |
| `access.high_contrast` | off, on | off | §3.5, §13.4 |
| `access.consistent_skin` | off, on | off | §2.1 |
| `access.lean_toggle` | off (hold), on (toggle) | off | §10.9 |
| `access.narrator` | off, on; speech rate 50..=300 words per minute | off; 180 words per minute | §13.6 |
| `access.sound_viz` | off, on | off | §13.7 |
| `access.photosensitive` | off, on | off | §13.8 |
| `access.game_speed_bp` | 5000..=10000 step 1000, solo only | 10000 | §13.9 |

---

## 14. Localization

- **Launch locales** are the 15 of DECISIONS 40, listed by doc 18 §9.11 (doc 02 §10 makes the list doc 18's): en-US (source), fr-FR, de-DE, es-ES, es-419, pt-BR, it-IT, pl-PL, ru-RU, uk-UA, tr-TR, ja-JP, ko-KR, zh-Hans, zh-Hant. Every functional string ships in all 15. Right-to-left scripts are not in the launch set; when one is added, the anchor system mirrors horizontally (TL ↔ TR, ML ↔ MR, BL ↔ BR, and every x offset negated).
- **Keys**: every string is a Canon §1.10 text key in Fluent files (doc 02 §10). No text is baked into textures; genre title art carries no words. Pluralization and gender use Fluent selectors, never string concatenation.
- **Lengths**: every widget fits the pseudo-locale `en-XA` (+30%); German and Russian strings over the widget width wrap to a second line, then shrink to at most 85% size, then ellipsize with the full text in the tooltip. Item names wrap at 2 lines.
- **Numbers**: Canon §1.5 display rules with the locale's grouping and decimal separator; percentages per locale (`11,8 %` in fr-FR); dates in ISO 8601 in logs and locale long form on screen.
- **Fonts**: §2.4. Line breaking follows Unicode UAX #14 in every script; CJK never uses italics or faux bold; Japanese ruby is supported in Lore pages. A missing glyph falls back to Noto Sans, then Noto Sans CJK for the locale; a glyph missing from every shipped font fails the CI font-coverage check for that locale's strings.
- **Genre skins**: genre-skinned names resolve in the Canon §1.10 order in every locale; a locale missing a skin string falls back to the archetype name in that locale, then to en-US.

## 15. Onboarding and tooltips

- The first real run teaches the core verbs through contextual tips (doc 14 §11, DECISIONS 132). This chapter adds **tips** `ui.tip.*`: a trigger event, a page of text with binding glyphs, a maximum of 1 show per account, and a "Show again" reset in Settings. Tips never pause the game; they appear in the notification area, and the full text is in the Journal Glossary.

```json
{ "id": "ui.tip.contradiction", "trigger": { "event": "ev.contradiction_changed", "min_c": 3 },
  "text_key": "loc.ui.tip.contradiction.desc", "glyphs": ["input.ui_details"], "max_shows": 1 }
```

- **Launch tips.** Every tip is a record of the shape above: `text_key` is `loc.ui.tip.<name>.desc` (doc 18), `max_shows` is 1, and the trigger is the local player's first occurrence of:

| Tip | Trigger | Glyphs |
|---|---|---|
| `ui.tip.contradiction` | `ev.contradiction_changed` with `new_c ≥ 3` on an item the local Erratum holds | `input.ui_details` |
| `ui.tip.anachronism_edge` | `ui.icon.edge` drawn for the first time (Anachronism Edge active against the current target, §2.5) | `input.ui_details` |
| `ui.tip.erasure` | `ev.erasure_stack` on the local Erratum | none |
| `ui.tip.colophon` | `ev.colophon_entered` for the local slot | `input.interact`, `input.menu_exits` |
| `ui.tip.ward` | `ev.ward_broken` on the local Erratum | none |
| `ui.tip.minions` | `ev.minion_bound` with the local slot as `owner_slot` | `input.minion_follow`, `input.minion_hold`, `input.minion_attack` |
| `ui.tip.ghost` | `ev.player_ghosted` for the local slot | `input.attack`, `input.ping`, `input.weapon_next` |
| `ui.tip.affliction` | `ev.affliction_changed` on the local Erratum | `input.menu_character` |
| `ui.tip.prayer` | `ev.favor_changed` on the local Erratum | `input.pray` |
| `ui.tip.lean` | `ev.awareness_changed` with the local Erratum as `subject` and `to` other than `aware.unaware` | `input.lean_l`, `input.lean_r` |
| `ui.tip.stealth_light` | the stealth meter (`ui.hud.stealth`, §10.10) shown for the first time | `input.crouch` |
| `ui.tip.placeables` | `ev.item_picked_up` of a `cat.placeable` item into the local Erratum's inventory | `input.attack`, `input.alt`, `input.weapon_next` |
| `ui.tip.ledger` | `ev.item_picked_up` of an item whose type the local player does not know | `input.menu_journal` |
| `ui.tip.scribes_mercy` | the local Erratum's first death (Canon §10.8 step 3, counted as §13.1 counts deaths) while `access.scribes_mercy` is off; shown after the death screen with the text "Scribe's Mercy is in Settings if you want it" | `input.pause` |
- **First-open explainers**: each major menu shows a one-page explainer the first time, re-openable from its "?" button.
- **Tooltips**: mouse hover 300 ms; gamepad shows the focused element's tooltip at once in a side panel; nested glossary links open with `input.ui_details` and back with `input.ui_back`.
- The Prologue is retired (DECISIONS 132), so its room L3 lesson is not taught. Harvesting and placement are taught by contextual tips (doc 14 §11).

## 16. Feedback and juice

### 16.1 Combat and event feedback

Doc 05 §18 owns hit markers, damage numbers, hitstop and camera kick; this chapter adds the toggles of §13 and these cues: kill confirm (a 20-frame ink blot on the crosshair), Shock of the New (a single page-crack glyph on the target), jam (torn-page glyph on the weapon panel, 45 frames), fizzle (smoke wisp on the spell panel), level-up (a gilded initial drawn around the level numeral over 60 frames and a 2-line notification), skill mastery (the skill's glyph stamped on the screen edge), identification (the item card flips to its identified side), Overcast (the cast arc continues as a second ring in `accent` while doc 06's Overcasting runs), and stale remote entities (doc 16: an entity with no new sample for 6 ticks renders at 3000 Bp saturation with a dotted outline until a sample arrives).

### 16.2 Loot visibility

Ground items show a light beam whose color is the rarity color and whose top carries the rarity letter and pips (P3): Common none, Uncommon 1 m beam, Rare 2 m, Epic 3 m, Legendary 4 m, Exotic 4 m with a slow pulse, Palimpsestic 5 m prismatic. Instanced items appear only to their finder (doc 11 §14.4). Illuminated items add a sparkle at every rarity.

### 16.3 Haptics (`ui.rumble.*`)

| ID | Low motor (Bp) | High motor (Bp) | Frames | Trigger |
|---|---|---|---|---|
| `ui.rumble.hit_taken` | 6000 | 3000 | 8 | `ev.damage_dealt` on the local player, scaled by `min(10000, 20 × hp_lost_bp)` |
| `ui.rumble.fire_light` | 0 | 2500 | 3 | own semi or auto shot |
| `ui.rumble.fire_heavy` | 5000 | 4000 | 6 | launcher, shotgun, sniper shot |
| `ui.rumble.parry` | 3000 | 8000 | 6 | own parry |
| `ui.rumble.ward_break` | 7000 | 7000 | 10 | own `ev.ward_broken` |
| `ui.rumble.level_up` | 2000 | 2000 | 20 | `ev.level_gained` |

All values multiply by the vibration setting (§12.4).

### 16.4 Narrator notes (DECISIONS 38)

Narration is text only: the Narrator (`nar.narrator`, doc 18 §9.1) never speaks, so no Narrator note plays audio, enters the subtitle box or carries a caption. Its pencil notes (`loc.nar.note.<moment>.<n>`) appear where doc 18 §9.1 places them:

- **Diegetic surfaces** (the page turn under the Folio intro, the Colophon lectern, loading pages) draw the note in doc 18's Narrator pencil `#4D4D4D` on doc 18's genre `paper` (doc 18 §3.1).
- **Firsts** (first jam, first Disconnected item, first Erasure stack, first death, first Editor): the note enters `ui.hud.notifications` as one entry of at most 2 lines (72 px) after a pencil glyph, in italic body text in the `pencil` token (§2.2), for 8 s instead of 4 s. A longer note ends in an ellipsis; the Journal's Lore page holds the full text.
- **Tooltip footnotes**: one italic line in the skin's `muted` token under the tooltip body, below any breakdown (§5.6).

UI chrome never uses the pencil color `#4D4D4D`, which measures 2.25:1 on the `#101010` HUD backplate, below P2's 4.5:1. Every note shown in a run is kept on the Journal's Lore page (§7), and screen readers read notes like every UI string (§13.6). CJK locales set notes upright, because CJK never uses italics (§14).

---
## Exports

All IDs follow Canon §1.1 and sit in `ui.`, `input.` and `access.`, which Canon §19 assigns to this chapter.

- **`ui.skin.`** (17): `ui.skin.mythic_fantasy`, `ui.skin.gothic_horror`, `ui.skin.cosmic_horror`, `ui.skin.space_opera`, `ui.skin.cyberpunk`, `ui.skin.post_apocalypse`, `ui.skin.weird_west`, `ui.skin.hardboiled_noir`, `ui.skin.wuxia`, `ui.skin.steampunk`, `ui.skin.fairy_tale`, `ui.skin.mythic_antiquity`, `ui.skin.oneiric`, `ui.skin.redacted`, `ui.skin.unwritten`, `ui.skin.ur_text`, `ui.skin.margin`.
- **`ui.hud.`** (35): `ui.hud.folio_header`, `ui.hud.party`, `ui.hud.minions`, `ui.hud.compass`, `ui.hud.boss_bar`, `ui.hud.invasion`, `ui.hud.ghost`, `ui.hud.notifications`, `ui.hud.minimap`, `ui.hud.crosshair`, `ui.hud.prompt`, `ui.hud.vote`, `ui.hud.downed`, `ui.hud.vitals`, `ui.hud.status`, `ui.hud.body`, `ui.hud.quickbar`, `ui.hud.abilities`, `ui.hud.stealth`, `ui.hud.subtitles`, `ui.hud.weapon`, `ui.hud.spell`, `ui.hud.chat`, `ui.hud.placement`, `ui.hud.cast_wheel`, `ui.hud.quick_wheel`, `ui.hud.weapon_wheel`, `ui.hud.emote_wheel`, `ui.hud.minion_wheel`, `ui.hud.door_panel`, `ui.hud.lockpick`, `ui.hud.hack`, `ui.hud.pickpocket`, `ui.hud.killcam`, `ui.hud.net_status`.
- **`ui.menu.`** (36): `ui.menu.book`, `ui.menu.item_card`, `ui.menu.breakdown`, `ui.menu.character`, `ui.menu.creation`, `ui.menu.craft`, `ui.menu.alchemy`, `ui.menu.composer`, `ui.menu.journal`, `ui.menu.map`, `ui.menu.shop`, `ui.menu.reliquary_odds`, `ui.menu.engine`, `ui.menu.trade`, `ui.menu.boon_altar`, `ui.menu.run_summary`, `ui.menu.prayer`, `ui.menu.retinue`, `ui.menu.margin_trophies`, `ui.menu.margin_index`, `ui.menu.lobby`, `ui.menu.inkwell`, `ui.menu.vault`, `ui.menu.codex_pass`, `ui.menu.store`, `ui.menu.pause`, `ui.menu.settings`, `ui.menu.scoreboard`, `ui.menu.chat`, `ui.menu.replay`, `ui.menu.margin_dialogue`, `ui.menu.dramatis`, `ui.menu.vellum_archive`, `ui.menu.fairness`, `ui.menu.lobby_browser`, `ui.menu.mod_sync`.
- **`ui.icon.`** (62): `ui.icon.dmg_slash`, `ui.icon.dmg_pierce`, `ui.icon.dmg_blunt`, `ui.icon.dmg_fire`, `ui.icon.dmg_frost`, `ui.icon.dmg_shock`, `ui.icon.dmg_acid`, `ui.icon.dmg_radiation`, `ui.icon.dmg_sonic`, `ui.icon.dmg_toxin`, `ui.icon.dmg_radiant`, `ui.icon.dmg_umbral`, `ui.icon.dmg_psychic`, `ui.icon.dmg_arcane`, `ui.icon.dmg_true`, `ui.icon.c_coherent`, `ui.icon.c_strained`, `ui.icon.c_contradicted`, `ui.icon.c_disconnected`, `ui.icon.edge`, `ui.icon.ax_arcana`, `ui.icon.ax_techne`, `ui.icon.ax_mythos`, `ui.icon.ax_verve`, `ui.icon.frame_dot`, `ui.icon.frame_debuff`, `ui.icon.frame_control`, `ui.icon.frame_buff`, `ui.icon.frame_mental`, `ui.icon.frame_need`, `ui.icon.frame_indicator`, `ui.icon.frame_special`, `ui.icon.frame_affliction`, `ui.icon.res_hp`, `ui.icon.res_stamina`, `ui.icon.res_aether`, `ui.icon.res_ward`, `ui.icon.res_composure`, `ui.icon.res_toxicity`, `ui.icon.res_breath`, `ui.icon.res_satiation`, `ui.icon.res_temperature`, `ui.icon.res_respirator`, `ui.icon.xp`, `ui.icon.mercy`, `ui.icon.marker_hostile`, `ui.icon.marker_ally`, `ui.icon.marker_neutral`, `ui.icon.marker_minion`, `ui.icon.marker_ping`, `ui.icon.marker_danger`, `ui.icon.marker_objective`, `ui.icon.marker_colophon`, `ui.icon.marker_scriptorium`, `ui.icon.marker_shop`, `ui.icon.marker_reliquary`, `ui.icon.marker_engine`, `ui.icon.marker_remains`, `ui.icon.marker_exit`, `ui.icon.marker_hold`, `ui.icon.marker_claw`, `ui.icon.ironman`.
- **`ui.tip.`** (14): `ui.tip.contradiction`, `ui.tip.anachronism_edge`, `ui.tip.erasure`, `ui.tip.colophon`, `ui.tip.ward`, `ui.tip.minions`, `ui.tip.ghost`, `ui.tip.affliction`, `ui.tip.prayer`, `ui.tip.lean`, `ui.tip.stealth_light`, `ui.tip.placeables`, `ui.tip.ledger`, `ui.tip.scribes_mercy`.
- **`ui.rumble.`** (6): `ui.rumble.hit_taken`, `ui.rumble.fire_light`, `ui.rumble.fire_heavy`, `ui.rumble.parry`, `ui.rumble.ward_break`, `ui.rumble.level_up`.
- **`input.` actions** (70): `input.move`, `input.look`, `input.look_gyro`, `input.attack`, `input.alt`, `input.heavy`, `input.reload`, `input.throw`, `input.jump`, `input.crouch`, `input.sprint`, `input.walk`, `input.dodge`, `input.interact`, `input.lean_l`, `input.lean_r`, `input.cast`, `input.cast_wheel`, `input.cast_cancel`, `input.pray`, `input.examine`, `input.ability_1`, `input.ability_2`, `input.ability_3`, `input.ability_4`, `input.weapon_next`, `input.weapon_prev`, `input.weapon_1`, `input.weapon_2`, `input.weapon_3`, `input.quickbar_1`, `input.quickbar_2`, `input.quickbar_3`, `input.quickbar_4`, `input.quickbar_5`, `input.quickbar_6`, `input.quickbar_7`, `input.quickbar_8`, `input.grimoire_next`, `input.grimoire_prev`, `input.ping`, `input.minion_follow`, `input.minion_hold`, `input.minion_attack`, `input.hold_breath`, `input.whistle`, `input.shout`, `input.glyph_layer`, `input.camera`, `input.menu_book`, `input.menu_map`, `input.menu_journal`, `input.menu_character`, `input.menu_exits`, `input.pause`, `input.scoreboard`, `input.chat`, `input.push_to_talk`, `input.emote_wheel`, `input.speak_vitals`, `input.ui_confirm`, `input.ui_back`, `input.ui_tab_next`, `input.ui_tab_prev`, `input.ui_page_next`, `input.ui_page_prev`, `input.ui_details`, `input.ui_compare`, `input.ui_actions`, `input.ui_nav`.
- **`input.ctx.`** (9): `input.ctx.on_foot`, `input.ctx.casting`, `input.ctx.downed`, `input.ctx.ghost`, `input.ctx.wheel`, `input.ctx.menu`, `input.ctx.placement`, `input.ctx.lockpick`, `input.ctx.margin`. **`input.preset.`** (6): `input.preset.folio_kbm`, `input.preset.left_hand_kbm`, `input.preset.folio_pad`, `input.preset.spellwright_pad`, `input.preset.southpaw_pad`, `input.preset.folio_deck`. **`input.cfg.`** (12): `input.cfg.mouse_sens_milli`, `input.cfg.ads_mult_bp`, `input.cfg.invert_y`, `input.cfg.deadzone_inner_bp`, `input.cfg.deadzone_outer_bp`, `input.cfg.look_curve`, `input.cfg.look_yaw_dps`, `input.cfg.look_pitch_dps`, `input.cfg.vibration_bp`, `input.cfg.trigger_bp`, `input.cfg.gyro_mode`, `input.cfg.aim_assist_gyro`.
- **`access.`** (51): `access.scribes_mercy`, `access.subtitles`, `access.subtitle_size_bp`, `access.subtitle_bg_bp`, `access.speaker_names`, `access.caption_direction`, `access.colorblind_mode`, `access.world_filter`, `access.ui_scale_bp`, `access.hud_scale_bp`, `access.hud_layout`, `access.text_scale_bp`, `access.font`, `access.high_contrast`, `access.consistent_skin`, `access.fov_deg`, `access.viewmodel_fov_deg`, `access.camera_shake_bp`, `access.head_bob_bp`, `access.reduce_motion`, `access.center_dot`, `access.third_person_distance_mm`, `access.hitstop`, `access.gore`, `access.narrator`, `access.sound_viz`, `access.sound_ripples`, `access.photosensitive`, `access.game_speed_bp`, `access.hold_to_toggle`, `access.aim_assist`, `access.auto_sprint`, `access.dodge_double_tap`, `access.wheel_dwell`, `access.lean_toggle`, `access.navigation_line`, `access.enemy_outline`, `access.shimmer_bp`, `access.minimap`, `access.minimap_rotate`, `access.stealth_meter_always`, `access.lock_mode`, `access.damage_numbers`, `access.damage_numbers_mono`, `access.tv_safe_area`, `access.hud_preset`, `access.hud_opacity_bp`, `access.hud_backplate_bp`, `access.mono_audio`, `access.dynamic_range`, `access.mark_hallucinations`.
- **Non-ID interfaces**: command fields `ACTION` (bits 26–28), `WHEEL_ENTRY` (bits 29–31), `WHEEL_SELECT` (bits 19 and 20 together); damage step 6c; the intent kinds listed under Interface assumptions; the HUD anchor model with `place` and `validate`; the overlay placement of §10.12; the `pencil` token; `craft_odds`; the wheel sector rule; the sprint-slide input rule (§12.1); the Second Edition stakes lines (§11.10).

## Interface assumptions

| Doc | Assumed to provide |
|---|---|
| 01 | its §7 verb table copies §12.1, §12.3 and §12.4, including the gamepad cast cancel of §12.4 and without the retired mount input (triage C-30) |
| 02 | the §5.5 bit table with §12.2's `ACTION` (bits 26–28) and `WHEEL_ENTRY` (bits 29–31) and with bit 25, the retired `MOUNT_TOGGLE`, reserved and 0 (triage C-10, C-30); `IntentKind` additions `SetScribesMercy { on }`, `MapPing { cell: [i32; 3] }`, `AssignWheelEntry { index, source }`, `AssignAbilitySlot { slot, talent }`, `QuickbarAssign { index, uid }`, `SetFocusLock { uid }`, `Examine { uid }`, `ShopHaggle { shop }`, `SetCastItemTarget { uid }`, `SetBreathHold { on }`, `Vocalize { kind }` (C-15) and `SetLockMode { manual }` (§10.11); the FOV setting 60–120 (§8.4, C-11), cited as this chapter's §13.5 `access.fov_deg` where doc 02 writes "doc 17 §8.4"; AccessKit through `bevy_a11y` in `bevy_winit` and `tts` 0.26.3 for `access.narrator` (§2.2, present); `flag.scribe.enabled` (doc 18 §9.10); the host's required mod set synced to co-op joiners without a subscribe prompt (§13.4, DECISIONS 126), as §11.9's `ui.menu.mod_sync` shows it |
| 03 | `ev.noise` `heard` entries per slot; `mercy_on`, `mercy_bp` and `breath_held` in `comp.player`; step 6c executed by the damage system for every involuntary HP loss (Canon §10.5); every event this chapter reads registered in §14.3; DECISIONS 95's movement kit reached through existing bits: the ledge mantle on a `JUMP` press (§5.9 reaches 2,000 mm per DECISIONS 95 and Canon §4.2), the sprint-slide on a `CROUCH` press while `SPRINT` is held and the Erratum sprints (§12.1; doc 03 G31 specifies the motion) and the grappling tool as a doc 11 item used from the quickbar, so it needs no new button bit and bit 25 stays 0; `ev.law_discovered` with doc 12 §4.1's payload (present) |
| 04 | `explain_row(entity, row) -> Explanation` (doc 04 §1.2) over the same code path as §6.4 evaluation; `PickTalent` resolving stored Interpolation offers; creation validation failures as text keys; §11.5's "holds `input.hold_breath`" read as `breath_held`; madness phantoms (§12.4) identifiable on the bearer's client, so `access.mark_hallucinations` can mark them; the 12 creator presets per kin with palette and marking colors (DECISIONS 97) |
| 05 | `aim_assist_bp` per weapon archetype (default 10000); `lean_k` and the shrunken lean offset replicated to the owner for the indicator; hold breath while aimed per §6.7; the retired mount input gone from §1.7's inputs (C-30) |
| 06 | `compile()` callable on the client with a `CasterStats` snapshot; `Pray { patron, kind }`; channel and charge wheel entries select without casting, and `SetCastItemTarget { uid }` replaces an item uid in the command, acknowledged in `OwnerDelta` (C-28); Grimoire presses counted as §12.2 states; the prayer component's failure chance exposed to the client; §16.3's draws and §16.4's gate as §10.7 copies them |
| 07 | doc 02's `Craft` intent carrying `StartCraft { recipe, count }`; the recipe kinds of §6.1; the inputs of §4.2's `craft_quality` (center, caps, Smithing-50 and `masterwork` flags) exposed to the client for §6.1's odds; placement validity with a reason code; placement ticks by Fieldcraft |
| 09 | `StealthHud` (§8) in the doc 16 snapshot; the six states of §6.1; noise IDs `noise.<category>` or `noise.<category>.<name>`; `Vocalize { kind: whistle \| shout }` producing §12's whistle and shout; `SetBreathHold` as `input.hold_breath`; §13.3's per-slot `lock_mode` set by `SetLockMode`, governing picks and hacks, Seize included; the manual pin targets computable on the owner's client from the keyed seq 1004 draws; the slip roll of doc 09 §13.2 (no break, no wear); §13.4's deck spark ending a hack attempt; `StealthHud` marks whose `flags` bit 0 is set while the pair's meter rose in the last 20 ticks (§8.3), which §10.10's arcs read; for a targeted device its verbs, `chance`, `ticks` and `hack_fails`, and for a pickpocket target its stealable items with chance and ticks, available to the owner's client; `Hack { target, verb }`, `Steal { target, uid }` and `Plant { target, uid }` as doc 02 intents (§10.12) |
| 10 | `ItemView` per §2.8, extended with doc 11's perk columns, parts, stack, value, `illuminated` and `finder_slot`; `candidates(d)` replicated for the Ledger |
| 11 | the instance fields of §2.2 in `ItemView`; `displayed_weights` and Reliquary line weights exposed to the opener's client; the mimic placement rate per tier (§10); §12.2's trade factor and `ShopHaggle { shop }`; §12.3's services without repair or mount mending (C-30, C-31); `DropLucre { amount }` (§17); the cursed capsule sealed with `OMEN` and its cursed share (§11.2) |
| 12 | the §5.7 plan functions callable on the client for door panels; §4.1's per-Law triggers, `laws_triggered` slot masks and viewer rule (present), which §7 follows; the Folio quest of §20.6 with §20.4's read count and §20.5's Leaf (present); for Second Edition (DECISIONS 101), how `local_genre` reads in a fused Folio and the order of its genre pair, which §11.10 uses for the skin fallback and the header sigils |
| 13 | `MinionCommand { kind, minions: All \| List(Vec<EntityId>), cell, target }` resolving into §6's per-minion `Follow`, `Hold(CellIdx)`, `Attack(EntityId)`; companion approval, loyalty and quest state in the owner's `OwnerDelta` |
| 14 | the 32 `margin.slot.*` records of §5.1a (defined); `RunSummary` carrying `mercy_ever_on` and `min_game_speed_bp`; Grade unlocks and achievements ignoring Mercy (DECISIONS 82, C-04); ranked eligibility excluding game speeds below 10000 (G-201); the party's visited-room set; the Prologue room L3 rewrite (C-03); §3.8's `clock_start` with Canon §17.1's 43,200-tick onset, the cap of 10 stacks and one hunter per Folio (Canon §11.6); the account's discovered World Laws kept with the Index (§5.5, present); Glyph Library picks with their refusal reasons in `ErratumSelect`, made on the Dramatis page (§5.7); arc beats made available per resident (§5.6); the closing manuscript inputs (§4.7); §8.9's Second Edition door order, stakes, forfeit, `RunSummary.ironman` and board mark (present), offered from Early Access (DECISIONS 118), where §8.9 still writes 1.0 |
| 15 | the account profile syncing `mercy_enabled` and `mercy_bp`; odds and history endpoints for the Inkwell and Vault screens; the `transcription` counter and the claim with its `insufficient_transcription` refusal (§3.9.1); epoch state, commitment, client-seed modes, "Reveal now" and the verifier library for the Fairness panel (§4.1, §4.3); `cosm.frame.ironman` (§2.2); voice with non-friends muted by default (§8.5) |
| 16 | the snapshot `HUD` section carrying `light_sky` and `light_block`, after light overrides, in one byte (low nibble block, high nibble sky, the `ChunkView.light` layout) beside `light_at` (C-09); `StealthHud` in the snapshot's `STEALTH` section with its caps of 4 rings, 8 marks and 4 steps and each mark's `bearing` (§6.8, present); the sanitizer accepting §12.2's values and replacing a command with `ACTION` 6 or 7 or bit 25 set (C-10); a second `PING` press within `ping_danger_ticks` (15) upgrading the ping to danger (§9.12, present); `RunView` carrying the Folio quest state (story events read, `story_solved`, the slot holding the Folio Leaf) and the run's `laws_triggered`, and `ev.law_discovered` delivered to the clients of the slots it names, so §7 keeps its inputs across a join or reconnect; the lobby browser data, `Welcome.presentation_hash`, `VoiceMute`, kick votes, client states and `MigrateAnnounce` that §11.9 draws (present); game speed locked to 10000 with remote peers; `RunState` carrying the party map reveal of `item.text.mapping`; the trace service and the vote path of DECISIONS 96 and 104 (doc 16 O2), before §11.6 adds their screens |
| 18 | caption keys on `aud.*` records; `art.icon.<status>` and `art.icon.aether.<genre_suffix>` glyphs; heading faces (`art.font.*`) and font files with license fields; every `loc.ui.*` string; `vfx.aware.*` over-head glyphs with doc 09's state names; Narrator notes (`loc.nar.note.*`) tagged with the moment that shows them, so §16.4 can place them; the Margin cast lines and arc beats of §8.4 and the closing manuscript of §8.5a; the creator presets of DECISIONS 97 |

## Canon gaps

- **G28.** Canon §5.8 says that once a World Law first triggers "the Codex then remembers it". The triage (G-197) keeps "Codex" for the pass and names the item encyclopedia the Index (doc 14 §5.5). Assumed: the account's discovered Laws are kept with the Index (§7). Proposed canon text, §5.8: replace "the Codex then remembers it" with "the account's Index (doc 14) then remembers it".
- **G29.** Canon §17.1 grants stack `k` on tick `folio_entry_tick + 43200 + 1800 × (k − 1)`, and §11.6 pauses Erasure accrual in boss Folios while the boss lives without saying whether the pause shifts that schedule; doc 14 §3.8 starts the boss Folio's clock at the Editor's death. Assumed doc 14's `clock_start` (§3.2). Proposed canon text, §17.1 `ERASURE_STEP`: replace "`folio_entry_tick + 43200 + 1800 × (k - 1)`" with "`clock_start + 43200 + 1800 × (k - 1)`, where `clock_start` is the entry tick, or the Editor's death tick in a boss Folio (doc 14)".

Gaps G1 to G27 are closed; their numbers stay reserved because other chapters cite them. G1: header (triage G-051). G2: doc 14 §5.1a defines the slots. G3: Canon §10.5 step 6c (G-089). G4: DECISIONS 82, doc 14 changes (C-04). G5: docs 02 and 16 change (C-10). G6: triage Q10, doc 02 changes (C-11). G7: DECISIONS 40 (C-17). G8, G9, G20, G25, G26: stale on disk (triage §4.1). G10: doc 16 changes (C-09). G11, G22: Canon §2.4 logs intents (G-016). G12, G13, G14, G18, G19, G23, G27: rules this chapter owns (G-198, G-201). G15, G24: doc 06 changes (C-28). G16: Canon §1.5 (G-011). G17: already in canon (G-115). G21: doc 14 rewrites the Prologue room L3 (C-03).

## Product-owner question round

The user asked for clarifying questions on every system designed here. Each default below is already binding; an answer that differs becomes a decision-ledger entry and a change to this chapter.

1. **Genre skins.** Should menus follow the local genre (bleed zones switch skins mid-Folio), or stay on the Folio genre? Default: local genre with a 500 ms crossfade.
2. **Skin intensity.** Do you want the heavy frames (gothic tracery, brass gauges) on HUD elements too, or only on menus? Default: menus full, HUD with border and corner only.
3. **HUD density.** Is the compass enough navigation, or do you want the minimap on by default? Default: compass on, minimap off.
4. **Damage numbers.** Default to own hits only, all hits, or off? Default: own hits.
5. **Exact odds on prayers.** Showing exact supplication odds turns IVAN's mystery into arithmetic. Keep it? Default: shown.
6. **Stealth meter.** Show raw light values 0–15 on the two bars, or bands only? Default: values and bands.
7. **Item card depth.** Show the full per-type damage breakdown on cards, or only on the details hold? Default: shares with glyphs on the card, full chain on hold.
8. **Inventory.** Keep 1 item per slot (doc 11), or a weight-only list view as an alternative layout? Default: grid plus sort and filter, no list view.
9. **Composer.** Allow composing and testing spells at the Proving Page with a training dummy readout? Default: yes, using the same live panel.
10. **Third person.** Is third person a full alternative (all HUD, aim from the camera), or exploration only? Default: full alternative, aiming from the leaned eye (doc 05).
11. **KBM defaults.** Interact on F with lean on Q/E, or interact on E with lean on Q/Z? Default: F interact, Q/E lean.
12. **Gamepad.** Is a held-modifier "glyph layer" acceptable for abilities and casting, or do you prefer the Spellwright layout (casting on RT) as the default? Default: glyph layer.
13. **Gyro.** Gyro "while aiming" by default on Steam Deck, or always on? Default: while aiming.
14. **Aim assist.** Standard slowdown 6000 Bp and tracking 2500 Bp are proposals without shipped references (R7). Playtest target, or a specific reference game to match? Default: these values, tuned in playtests.
15. **PvP aim assist.** Disable aim assist entirely against invaders? Default: slowdown only, capped at Standard.
16. **Scribe's Mercy and leaderboards.** Answered by DECISIONS 82: only ranked leaderboard entries exclude Mercy runs; there is no separate Mercy board.
17. **Scribe's Mercy and Grades.** Answered by DECISIONS 82: Mercy never blocks achievements or Grade unlocks.
18. **Game speed.** Is 50% the right floor, and should co-op support a host-set speed when every member agrees? Default: 50% floor, solo only.
19. **Read menus aloud.** Ship the built-in OS speech reader (`access.narrator`) at launch or after? Default: at launch, off by default. (The story Narrator is text-only, DECISIONS 38.)
20. **Locales.** Answered by DECISIONS 40: 15 launch locales, including ru-RU, uk-UA and tr-TR (§14). Still open: is Arabic wanted for 1.0? Default: no right-to-left script at launch.
21. **Fonts.** Functional UI text uses Noto Sans while doc 18's per-genre serif and display faces set titles and Lore pages. Do you want the genre body faces on menus too, at the cost of readability? Default: Noto Sans for functional text.
22. **Margin trophies.** Doc 14 §5.1a sets 32 slots per alcove. Can visitors in your lobby inspect them? Default: visitors see the host's alcove.
23. **Store presentation.** Show the USD anchor and other regions' prices on the detail page? Default: yes, inside a collapsed "Price details".
24. **Tips.** Should tips repeat once per run for new players until dismissed permanently? Default: once per account.
25. **Crafting and brewing odds.** Showing the exact quality distribution before a craft turns quality into a known gamble. Keep exact odds, or show only the possible range? Default: exact odds (P5).
26. **Large HUD.** Is a fixed 125% Large layout (which hides the minimap, body doll and chat box) enough, or do you need finer steps above 100%? Default: standard 75–100% plus Large at 125%.
27. **3D map sharing.** Should the map's seen cells be shared across the co-op party, or stay per player? Default: per player; rooms the party visited and `item.text.mapping` reveals are shared.
28. **Exit vote.** With two commit paths (timed majority and one voter holding interact with everyone inside), is the vote HUD clear enough with two rings per door? Default: both rings shown.
29. **Shop haggle preview.** Haggling is deterministic and the button shows its result before it is pressed. Keep the preview, or hide the outcome until pressed? Default: preview shown.
30. **Mimics.** An undetected Reliquary shows its tier's mimic placement rate (4%, 3%, 2%, 0%). Show it, or keep mimics a surprise? Default: shown, as exact odds.
31. **Inkwell reveal.** Should the pull reveal have any build-up, or skip straight to the cards? Default: a 30-frame reveal, skippable, no near-miss motion.
32. **Codex pass page.** Is the rested reserve ("doubles your next earnings") clear and welcome, or does it read as pressure to log in? Default: shown as a neutral line, no reminders.
33. **Naming.** The on-device language model is called "the Scribe" in docs 02 and 18, and the assist mode is "Scribe's Mercy". Rename the model (for example "the Marginalist"), or keep both? Default: keep both; the model's Settings tab is "Generated text".
34. **Minions on gamepad.** Follow, hold and attack sit on the minion wheel (LB + D-pad down hold) with attack also on a tap. Do you want a dedicated quick button for follow? Default: wheel plus attack tap.
35. **Companions.** Should companion approval show as a number, or only as a mood glyph to keep it diegetic? Default: the number with the loyalty mark at 60.
36. **Mount panel.** Answered by DECISIONS 55: mounts are cut, so no mount panel exists.
37. **Ghosts.** Should a ghost spectate allies by default, or start in the free ghost body? Default: free ghost body; weapon next and previous spectate.
38. **Downed.** Should a downed player see the exact bleed-out ticks shortened by each hit? Default: the bar and timer update, with no numbers per hit.
39. **Afflictions.** Should affliction downsides be listed in full on first contraction, or discovered? Default: listed in full.
40. **Manual lockpicking.** The manual minigame lights the pin's tension glyph inside the hit window, which makes it a timing test. Keep the light, or make it blind? Default: light on.
41. **Gamepad cast cancel.** B cancels a cast while casting, and a second B dodges. Acceptable, or would you rather keep B as dodge and put cancel elsewhere? Default: B cancels.
42. **Hold breath.** Hold breath is P on keyboard and LB + R3 hold on gamepad. Good, or should it share the crouch key as a long press? Default: separate bindings.
43. **Subtitles.** 67 ms per Latin character and 111 ms per CJK character set display time. Adjust? Default: those values.
44. **Gore.** Answered by DECISIONS 94: `ink` by default, `full` is the red-blood toggle and `minimal` stays available; the rating target is doc 15's PEGI 16 and ESRB M.
45. **Sound visualization.** Ship both the crosshair ring and doc 18's world ripples, or only one? Default: both, each off by default.
46. **Photosensitivity default.** Should the photosensitivity mode be on by default for the first launch until the player confirms the notice? Default: off, with the notice on first launch.
47. **Haptics.** Are the six rumble events enough, or do you want rumble for footsteps of nearby enemies (an accessibility cue)? Default: six events.
48. **Solo pause.** Solo pauses whenever a menu is open. Should it also pause on the held scoreboard and on wheels? Default: no, they are HUD.
49. **Steam Deck back buttons.** L4 and R4 lean, L5 glyph layer, R5 ping. Would you rather have abilities on the back buttons? Default: as listed.
50. **Run summary.** Should the summary list every payout line (Ink, Quires, Codex XP, Reliquaries), or only totals? Default: every line.
51. **Marked hallucinations.** `access.mark_hallucinations` is off by default and forced on under Scribe's Mercy (DECISIONS 49). Should it be on by default for new players? Default: off, so madness stays deceptive.
52. **Narrator notes.** Notes on firsts stay 8 s in the notification list. Should they also pause on screen until dismissed? Default: no; the Lore page keeps every note.
53. **Killcam.** Should the killcam play by itself after a death, or wait for the player to ask? Default: a 180-tick prompt; skipping is one input.
54. **Fused Folio skin.** In a fused Second Edition Folio, should the UI skin follow the local genre zone by zone, as bleed zones do, or stay on one genre of the pair? Default: zone by zone through doc 12's `local_genre`, with the pair's first genre where §2.1 falls back to the Folio genre (§11.10).
55. **Mod sync.** DECISIONS 126 auto-syncs the host's required mods. Should joining a modded lobby still ask before downloading? Default: no question; the Mod sync page lists every mod with its progress and a Cancel button (§11.9).
56. **Kick vote visibility.** Should the member under a kick vote see the running vote? Default: yes, the same pinned line as everyone else, without a vote of their own (§11.9).
57. **Second Edition commit.** Should the Second Edition door need a unanimous vote because of its stakes, instead of doc 14 §3.6's normal exit rule? Default: the normal rule, with the four stakes lines on the panel before it can commit (§11.10).
58. **Ironman frame.** Where should the Ironman portrait frame show? Default: lobby slots and scoreboard rows, plus `ui.icon.ironman` on leaderboard entries (§11.10).

## Acceptance tests

1. **Mercy growth.** Enable on a fresh account: `mercy_bp = 2000`. Die 3 times with it on: 2600. Die 30 times total: 8000; a 31st death: 8000. Disable, die twice, re-enable: still the value before disabling.
2. **Mercy damage.** A 10,000 mHP pure-slash hit after step 6b on an Erratum at 2000 Bp leaves 8,000 mHP for step 7; at 2600 a 12,345 mHP hit leaves 9,136; at 8000 a 10,000 mHP hit leaves 2,000; with Mercy off, 10,000.
3. **Mercy and wards.** An `st.shielded` pool of 10,000 mHP and a 10,000 mHP shock hit at Mercy 2000 (`ward_mult` 15000, Canon §10.5 step 7): step 6c gives 8,000, step 7 gives `w = 12,000 > 10,000`, so HP takes `8,000 − floor_div(10^8, 15000) = 1,334` mHP and the ward breaks.
4. **Mercy while downed.** A 17,000 mHP hit at 2600 Bp on a downed Erratum with Max HP 85,000 shortens bleed-out by 267 ticks.
5. **Mercy never blocks.** A run with Mercy on that kills `boss.editor_v1` grants the same achievements, Ink, Quires and Codex XP as the same replay with Mercy off; the run is unranked.
6. **Mercy replay.** A replay containing `SetScribesMercy { on: true }` at tick 5,000 reproduces the live `state_hash` at tick 10,000.
7. **Command encoding.** Ability 3 alone: `buttons = 201,326,592`. Cast cancel: 335,544,320. Wheel entry 5 with instant cast: 2,685,927,936. Wheel entry 0 without cast: 1,572,864. Quickbar slot 3 used: 327,680. Weapon 2: 16,384.
8. **Press detection.** Commands with `ACTION` 1, 1, 0, 1 produce two presses of ability 1 (ticks 1 and 4). `ACTION` 1 then 2 produces a press of 2; then 1 again (key 2 released while 1 held) is never sent, because the client sends 0 on releasing the field's key.
9. **Skin contrast.** For each of the 17 skins: `ink:paper ≥ 7.0`, `accent:paper ≥ 4.5`, `muted:paper ≥ 4.5`, with the §2.2 values to two decimals.
10. **HUD layout.** At 1280 × 800 with margins 24 and `s = 1.0`, the 23 element rectangles equal the §3.2 table. `validate` passes for every `access.hud_scale_bp` step 7500..=10000 with margins 24 × 24 and 64 × 40 on logical canvases 1280 × 800, 1422 × 800, 1911 × 800 and 1280 × 960; at margins 64 × 40 on 1280 × 800, `s_fit = 0.935`.
11. **Large layout.** With `access.hud_layout = large` at 1280 × 800 and margins 24, `s = 1.25` and the 20 rectangles equal the §3.4 table (for example `ui.hud.stealth` 936,551–1256,606 and `ui.hud.subtitles` 344,526–936,626); `validate` passes on the same canvases and margins as test 10 (`s = 1.169` with the TV-safe margins); `ui.hud.minimap`, `ui.hud.body` and `ui.hud.chat` are not drawn.
12. **Text size.** Body text at 1920 × 1080, text scale 10000: 22 px; at 15000: 32 px; at 1280 × 800 with UI scale 7500: 12 px; a CJK caption in that case: 14 px.
13. **Formula tooltips.** `explain_row` for TGH 52, L 7 gives Max HP 127 and the substitution string of §5.6; RES 46 gives 53 Bp and 673 mHP/s displayed "0.6"; FAT 50, PRC 55, VERVE 7, gear 300 gives 1175 displayed "11.8%"; PRV 60, VERVE 7 gives 2400 displayed "24.0%".
14. **Tooltip agreement.** For 10,000 random characters (seeded), every `explain_row` final value equals the simulation's evaluated row.
15. **Composer.** Ref-A composing `bolt flame` shows complexity 2/9, base 5,000, cast cost 5,008 m-aether displayed "5", potency 12809, damage 36,523 mHP displayed "36"; adding glyphs to complexity 11 at LOG 50 shows `E_COMPLEXITY`.
16. **Inkwell next pull.** `top_since = 47`, `mid_since = 3`, no guarantee: 20.5000%, 6.0000%, 73.5000%, featured 15.3750%, each off-featured 0.4659%. `top_since = 59`: Illuminated 100.0000%.
17. **pct4.** `pct4(1, 4400)` displays `0.0227%`.
18. **Reliquary odds.** Weights `[0, 2728, 888, 258, 88, 33, 0]` display 68.2854%, 22.2278%, 6.4581%, 2.2028%, 0.8260%.
19. **Fortune Engine.** SER 70 brass at Folio 7: probabilities 23.0000%, 26.0000%, 18.0000%, 18.0000%, 9.0000%, 6.0000%; prices 100, 125, 150, 175, 200; armed pity shows 100.0000% on the top face. Face 4's card reads "Ominous" before purchase, and the cursed share reads 9.0000%, or 0.0000% with the guarantee armed.
20. **Prayer odds.** Timer 0, favor −120: 76.0000%. Timer 20,000, favor 300: 53.3330%; timer 179,999, favor 300: 0.0000%. Timer 0, favor 0: 100.0000%. Favor −500: 0.0000%.
21. **Codex display.** 43,900 XP shows tier 3 and 7,900 / 12,000 (65.8%); 1,200,000 XP shows tier 100.
22. **Store.** Unowned components 5.99, 3.99, 7.99, 1.99 show the set at USD 16.99 next to USD 19.96. An Ink price of 160 shows "≈ 0.4 h of play" and no currency equivalent.
23. **Light readout.** Gothic Horror cell with `light_sky` 2 and `light_block` 9: effective 9, "Dim", composure mark lit, "No spawns here". Redaction cell with `light_block` 15: no "No spawns here" glyph.
24. **Exit vote HUD.** Two voters, one idle: the panel shows need 2 before `open + 3600`, 2 from 3600 (`2/2 + 1`), 1 from 7200, and the hold ring fills to 120 at `open + 7319` for the active voter in door 1.
25. **Minion commands.** Tapping `input.minion_attack` with no entity under the crosshair sends nothing and shows "No target"; with a hostile at 30 m it sends `MinionCommand { kind: attack, minions: All, target }`.
26. **Toggle conversion.** With `access.hold_to_toggle`, pressing interact once on a Reliquary keeps bit 7 set for 90 consecutive ticks; pressing again at tick 40 clears it and the open does not complete.
27. **Aim assist off for mouse.** A mouse delta of 100 counts at sens 1000 turns the camera 2.2° whatever `access.aim_assist` is.
28. **Aim assist slowdown.** At `standard`, a stick look rate of 240°/s with a hostile part center 400 Bam from the aim becomes 144°/s; at 500 Bam it stays 240°/s.
29. **Game speed.** Solo at 5000 steps 30 ticks per real second; the replay of that session is byte-identical to a 10000 session given the same inputs; a co-op lobby, and a solo run with `mode.invasion_open`, refuses values below 10000.
30. **FOV.** 90° horizontal at 16:10 gives 64.01° vertical; 120° gives 94.54°; 60° gives 39.68°.
31. **Font licenses.** Every shipped font file's license field is OFL-1.1 or Apache-2.0.
32. **Screen reader.** Every focusable widget in every `ui.menu.*` screen exposes a non-empty AccessKit label; an automated walk of each screen by keyboard and by gamepad reaches every widget and returns to the first without a trap.
33. **Photosensitivity.** With the mode on, a synthetic 60 fps full-screen sequence alternating black and white every 5 frames for 1000 ms yields at most 3 flashes; after the third, no frame-to-frame change of mean luminance exceeds 500 Bp within that 1000 ms window. A Disconnected item shows the static torn-page icon.
34. **Subtitles.** A 30-character line with 1,200 ms of audio shows for 2,010 ms (`67 × 30`); a 10-character line with no audio shows 1,500 ms. The 199-character line of 40 words "word" separated by single spaces breaks at `k = 1.0` into 5 lines of 39 characters (8 words each) and 3 blocks of 78, 78 and 39 characters shown for 5,226, 5,226 and 2,613 ms; at `k = 2.0` lines hold at most 21 characters (4 words, 19 characters) and one block shows at a time. A 20-character CJK line with no audio shows for 2,220 ms (`111 × 20`).
35. **Rebinding.** Binding Mouse 1 to `input.reload` in `on_foot` is refused with "Already bound to Attack"; binding Esc away leaves `input.pause` with its gamepad binding and the Esc binding restored, since every device family keeps one.
36. **Color independence.** In `monochrome`, every rarity, marker, hit marker and damage number on a captured test frame carries a letter, pip count, shape or glyph (screenshot classifier test over the CI scene set).
37. **Craft odds.** `craft_odds(250, Good, false, false)` gives counts whose `pct4` over 2,803,221 reads Awful 1.3492%, Poor 23.3200%, Normal 50.2190%, Good 25.1119%; `craft_odds(557, Peerless, false, false)` reads Good 0.9359%, Excellent 20.7389%, Masterwork 68.3067%, Peerless 10.0185%, matching doc 07 §4.3 to two decimals.
38. **Brew odds.** `q0 = 8685` shows Poor 81.4186% and Normal 18.5814%.
39. **Boon reroll.** The Reroll button reads 25.0000% for each of the 12 boons.
40. **Prayer with Unheard.** Failure 1000 Bp, favor 0, timer 0: Unheard 10.0000%, Pleased 90.0000%. Favor 300, timer 20,000, local patron: Helped 53.3330%, Angry 46.6670%. Favor 300, timer 0: Pleased 100.0000% with friendly herald 5.0000% and Gift 5.0000% inside it.
41. **Stealth meter.** `StealthHud.gem` values 0, 499, 500, 1499, 1500, 2999, 3000, 4999, 5000, 7999, 8000 show 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5 pips; a `marks` entry at `aware.lost` draws the three-quarter eye with "?"; an arc for `meter = 5000` spans 8192 Bam.
42. **Wheel sector.** Deflections at 0°, 22.4°, 22.6°, 135° and 337.6° clockwise from up select sockets 0, 0, 1, 3 and 0; a deflection of 2999 Bp selects nothing.
43. **Seed entry.** `028t 5cy4 tqkff` and `O28T5CY4TQKFF` are accepted as `028T-5CY4-TQKFF`, also with tr-TR active; `028T-5CY4-TQKF` reports the length error; `028T-5CY4-TQKFU` reports the U error; `028T-5CY4-TQKF!` reports the alphabet error; `G28T-5CY4-TQKFF` reports the first-character error.
44. **Grimoire press rule.** Commands with bits (19), (19, 20), (19) on consecutive ticks produce one `GRIMOIRE_NEXT` press (the first) and one wheel selection; (20), (19, 20), (20) produce one `GRIMOIRE_PREV` press and one wheel selection.
45. **Mercy exemptions.** At Mercy 2000 a grimhallow reload costing 3% of Max HP 85,000 removes 2,550 mHP; starvation's 1% per 5 s removes `850 − mul_bp(850, 2000) = 680` mHP; an Erratum reduced to 0 HP by an invader's Redaction gains +200 Mercy, and a downed Erratum later revived gains none.
46. **Toxicity projection.** Four Normal healing doses project 48,000 (clean); a fifth projects 60,000 (queasy).
47. **ADS toggle and hold breath.** With ADS on toggle and a weapon whose `ads_sway` is 27, pressing sprint while fully aimed holds breath and ADS stays raised; with `ads_sway` 20 the same press releases ADS.
48. **Gamepad cast cancel.** While Winding, one B press sends `ACTION = 5` and no `DODGE`; a second B press within 18 frames sets `DODGE`.
49. **Erasure clock.** At `c = 39,600` ticks it reads "Erasure in 1:00"; at `c = 43,200` with 1 stack, "Erasure 1 · next 0:30"; at `c = 59,400` with 10 stacks, "Erasure 10"; in a boss Folio while the boss lives, "Erasure held". With 4 Red Ink doses (`thr = 6`) the clock turns `hostile` at 4 stacks, and after `ev.hunter_spawned` it shows the hunter glyph.
50. **Mouse look quantization.** 100 counts at sens 1000 add 2.2°, which `look_add` turns into +26,247,022 on `yaw_q` and `look_to_bam` into a yaw change of 400 Bam.
51. **Respec price.** At Folio 10 the talent respec service shows 434 Lucre, then 868 (doc 04 §6.5).
52. **Hold breath intents.** Pressing and releasing `input.hold_breath` 120 ticks apart sends `SetBreathHold { on: true }` and `SetBreathHold { on: false }`; the replay of that session reproduces the live `state_hash`.
53. **Settings JSON.** Every `access.*` record loads with integer-only fields; the §13.10 array parses and contains no floats.
54. **Input contexts.** With a wheel open while `input.ctx.casting` applies, `input.ctx.wheel` (rank 2) is live and the right stick selects a socket; in the Margin, `input.menu_book` opens `ui.menu.margin_index`; while the local Erratum holds `st.downed`, pressing `input.attack` changes no command bit.
55. **Fresh profile defaults.** A new client profile reads every `input.cfg.*` setting at its §12.10 default and every `access.*` setting at its §13.2, §13.5, §13.10 or §13.11 default, `access.gore` reading `ink` (Steam Deck: `access.text_scale_bp` 11000, `access.aim_assist` `standard`).
56. **Tips.** On a fresh account the first `ev.ward_broken` on the local Erratum shows `ui.tip.ward` once; later `ev.ward_broken` events, in that run or any other, show nothing until "Show again" is used; a first death with `access.scribes_mercy` already on shows no `ui.tip.scribes_mercy`.
57. **Hack panel.** Hacking 40, LOG 55, a normal deck and a `lock.t3` electronic lock (doc 09 §13.4): the Unlock line reads 74.0% and 1.6 s (96 ticks), or 1.3 s (76 ticks) with `trait.origin.wired`; a tier-5 device at Hacking 40 greys every line with "Needs Hacking 42"; choosing Unlock sends `Hack { target, verb }` and closes the panel.
58. **Pickpocket panel.** Doc 09 §13.7's example (Pickpocket 20, SLT 45, directly behind in darkness, a 300 g pouch on a Folio-4 guard) lists the pouch at 0.3 kg, 54.0% and 1.0 s (63 ticks); below Pickpocket 50 the Plant tab is absent; solo play does not pause while the panel is open.
59. **Overlay placement.** At 1280 × 800 with margins 24 and `s = 1.0` the hack panel spans 440,216–840,360; with every §3.2 and §3.4 scale, both margin sets and the four canvases of test 10, it never overlaps `ui.hud.invasion` or `ui.hud.crosshair`.
60. **Lock mode.** Switching `access.lock_mode` to `manual` mid-run sends `SetLockMode { manual: true }`; an attempt already running stays auto; the replay of the session reproduces the live `state_hash`.
61. **World Law discovery.** On a fresh account a door to a Wuxia Folio shows no World Law line and no placeholder; after `law.wuxia.qinggong` first triggers for the account, "World Law discovered" shows once, the World Laws page lists the Law, and every later Wuxia door panel shows it, in that run and the next.
62. **Marked hallucinations.** With `access.mark_hallucinations` off and Mercy off, a madness phantom draws unmarked; turning Mercy on marks it with the dashed outline and "?" without changing the setting; the Whispers caption reads "[Whispers (madness)]" only while marking applies.
63. **Text-only narration.** A Narrator note on a first plays no audio and enters no subtitle; it shows in `ui.hud.notifications` for 8 s in the `pencil` token (`#A0A0A0`, 7.28:1 on `#101010`) on at most 2 lines and stays on the Journal's Lore page.
64. **Killcam.** With 400 frames rendered before an `ev.player_ghosted` for the local slot, accepting the prompt plays exactly the last 300, ending on the death tick, orbiting the `ev.entity_killed` killer at 3,000 mm; skipping starts the ghost layout at once; in co-op the host's tick count advances during playback.
65. **Replay viewer.** A file whose `content_hash` matches no loadable content set is refused; one whose `presentation_hash` alone differs plays with the warning; seeking to the entry of Folio 4 starts from its `folio_states` entry, and every `checkpoints` hash matches the re-simulation.
66. **Margin dialogue.** With `margin.arc.binder` beat 1 available, interacting with the binder shows the beat's two lines first, as text with no audio; the next interaction shows facility talk.
67. **Glyph Library picks.** In `ErratumSelect` a third pick is refused; a glyph in doc 06's run-start set is greyed with its reason; a Daily run lists the 52 `base` glyphs instead of the account library.
68. **Drop Lucre.** From a purse of 120 the stepper offers 1 to 120 and Drop Lucre with 50 sends `DropLucre { amount: 50 }`; the action is greyed while downed; the trade screen has no Lucre line.
69. **Closing manuscript.** After an Ur-Text victory with the ending `written_back`, the run-end screen shows the closing manuscript before the summary; after a death-ended run it shows only the closing line and plates; re-opening the victory from the Ledger shows the same pages.
70. **Sprint-slide input.** With `input.preset.folio_kbm`, holding Left Shift while sprinting and pressing Left Ctrl produces a command with bits 4 and 5 set whose predecessor had bit 5 set and bit 4 clear, and sets no other new bit; with `input.preset.folio_pad` and sprint toggled on, an R3 press produces the same command and the sprint toggle stays on.
71. **Stealth caps and bearing.** A `STEALTH` section with 4 rings, 8 marks and 4 steps draws 4 world rings, at most 8 arcs and 4 footstep ticks. A mark with handle 8191, `bearing` 16384, `meter` 5000 and `flags` bit 0 set, seen with camera yaw 8192, draws an arc 8192 Bam long centered 8192 Bam clockwise from screen-up; the same mark with `flags` bit 0 clear draws no arc and still sets the awareness eye.
72. **Manual dial.** A manual Unlock on a device of tier 3 shows 5 pins. A miss on tick `t` adds a crack and ignores presses through tick `t + 30`, and the pin's cursor starts at `t + 31`. The third crack ends the attempt and the hack panel's header reads "Failures 1/3". A deck spark as an attempt starts shows "The deck sparked" and leaves "Failures 0/3".
73. **Folio quest.** On Folio entry the Current Folio page lists the quest under the Rubric as "Solve the story 0/3" and "Leaf not taken" ("0/2" under `law.cosmic_horror.forbidden_knowledge`). The read that completes the count shows "Solved" and the notification "Folio story solved"; an Active Erratum picking up `item.quest.folio_leaf` shows "Leaf held by <persona>"; leaving the Folio with the Leaf on the ground records "story solved only" in the Run log.
74. **Law discovery per viewer.** In a 2-player run where account A has discovered `law.wuxia.qinggong` and account B has not, B's Wuxia door panels and Folio header omit the Law while A's show it. When it triggers with both slots Active, B sees "World Law discovered" once and A sees no notification; when B's slot holds a ghost on that tick, B sees no notification and its panels keep omitting the Law for the rest of the run.
75. **Second Edition door and outcome.** A Standard victory with Margins doors unlocked lists them, then "Second Edition", then "Close the Book"; an Abridged victory lists "Second Edition" and "Close the Book"; a Seeded victory lists no "Second Edition" door. The door panel shows the 4 stakes lines before the door can commit. A party wipe in Second Edition Folio 7 shows "Second Edition: clear forfeited"; a victory with no `ev.player_died` in Second Edition shows the Ironman line, and its leaderboard row carries `ui.icon.ironman`.
76. **Transcription shelf.** At `transcription = 40`, claiming `bundle.script.lost_chapter_wuxia` leaves "Transcription: 8" and greys `bundle.script.kit_monk` with "Needs 8 more Folio exits"; a claim the service refuses with `insufficient_transcription` shows the same line and changes no balance. The costs show "≈ 1.3 h of play" for 16 and "≈ 2.7 h of play" for 32.
77. **Fairness panel.** A fresh account's panel shows `fresh` mode. Switching to `pinned` with the 6-byte seed `errata` while an epoch is `Active` first confirms that the epoch will be revealed, then shows it `Revealed` with its `server_seed`; Verify on it prints `PASS` for every record.
78. **Online sessions.** With no snapshot for 1,000 ms the pinned "Connection problem" line shows. A `MigrateAnnounce` with `net.migrate.warm` and `eta_ms = 3000` shows "The Folio continues." with "Catching up 0:03"; one with `net.migrate.cold` shows "This Folio restarts from its entry.". A kick vote in a full public party needs 2 yes votes and lapses after 1,800 ticks, and a party of 2 offers no vote. A joining member who is not the player's Steam friend starts with the struck speaker glyph. A client whose `presentation_hash` differs from `Welcome.presentation_hash` shows the cosmetic notice once per session.
79. **Mod sync.** Joining a lobby whose required set holds one data-only mod of 1 MiB the joiner lacks shows one progress row, then joins with no further input; Cancel returns to the lobby browser; that lobby's browser row shows its 16-hex-digit `pal_mods` tag.
80. **Retired features.** No `ui.`, `input.` or `access.` record names a photo mode, a crossplay setting or a friend code, and no Margin resident screen shows an affinity meter, gift or romance option.

## Invariants annex

```
Σ ui ⊗ input ⊗ access: read sim → render; write sim ⇐ PlayerCommand ∨ PlayerIntent only
W! ui values := ViewFrame ∪ pure sim fns(integer inputs)       N! float gameplay math in UI
W! color signal ⇒ ∃ shape ∨ glyph ∨ text                       N! color-only meaning
W! odds shown := pct4 before commit ∧ live pity counters        N! hidden odds ∨ countdown on paid or account offers
W! text px ≥ 12 (Latin) ∧ ≥ 14 (CJK)                            W! ink:paper ≥ 7 ∧ accent,muted:paper ≥ 4.5
W! validate(HUD table) = Ok ∀ scale step ∧ margins ∧ canvas      N! HUD element scaled above s_fit outside the Large table
W! buttons[26..28] = ACTION ∈ 0..=5                             N! ACTION ∈ {6, 7}
W! WHEEL_SELECT ⇔ bit19 ∧ bit20 in one command                  N! both Grimoire bits from Grimoire keys
W! mercy_bp ∈ {0} ∪ 2000..=8000 ∧ +200 per death while on       W! step 6c before ward ∀ involuntary HP loss of a Mercy bearer
N! Mercy blocks achievements | rewards | Grades                 W! Mercy ∨ speed < 10000 ⇒ unranked only
W! game_speed changes wall clock only                           N! game_speed ≠ 10000 with remote peers
W! aim assist := client look filter on stick deltas             N! aim assist on mouse ∨ projectile bending
N! input synthesized beyond player-chosen toggles               W! pause ∧ ui_back keep ≥ 1 binding per device family
W! every widget has an AccessKit label                          W! captions only for sounds heard by the local slot
W! craft ∧ brew odds := exact enumeration, pct4                  N! a roll drawn before commit
W! awareness eye := doc 09 §6.1 states                          W! Fx display := round_half_even (Canon §1.5)
W! Grimoire press ⇒ ¬(bit19 ∧ bit20)                            N! Mercy reduces voluntary HP costs
W! new sim verbs travel as intents; bit 25 = 0                  N! functional UI text in a decorative genre face
W! photosensitive ⇒ ≤ 3 flashes per 1000 ms                     W! Store prices := real currency, final amount
N! virtual currency on any Store screen                         W! earned-currency prices show "no real-money price"
N! undiscovered World Law on any preview ∨ HUD ∨ page            N! Narrator note in audio ∨ subtitles ∨ captions
N! mount ∨ durability UI                                        W! mercy_on ⇒ hallucinations marked
N! photo mode ∨ crossplay setting ∨ friend code                 W! Second Edition door ⇒ stakes shown before commit
W! detection arc center := mark.bearing                         W! sprint-slide := crouch press ∧ sprint held; no new bit
W! Law shown ⇔ in account set at run start ∨ viewer slot ∈ laws_triggered mask
```
