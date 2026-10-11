# R3 — Market Standards, Monetization Design Standards, and Law/Ratings (PALIMPSEST research dossier)

**As of 2026-10-11.** Prepared for the PALIMPSEST design canon. The final spec will be implemented by an AI coding agent, so this dossier favours exact numbers, formulas, dates and technical requirements.

**Evidence tags (mandatory, used throughout):**
- `[VERIFIED <url>]`: fetched or seen in a search result during this research session (2026-10-11). Where the source is secondary (press or law-firm commentary), that is stated.
- `[RECALLED]`: from model memory and not re-checked this session. Treat as probable, not certain.
- `[COMPUTED]`: derived arithmetically in this session (Python) from VERIFIED parameters. The method is shown so it can be reproduced.
- `[PROPOSED]`: a PALIMPSEST design value. It is not a fact about any existing game.

---

## Table of contents

0. Bottom line (read this first)
- **Part A: Genre peers and the 2026 quality bar**
  - A1. Landscape of reference titles, 2024-2026
  - A2. The 2026 "table stakes" feature bar
  - A3. Per-title lessons worth stealing or avoiding
  - A4. Live-service health signals, 2025-2026
  - A5. Platform and infrastructure facts that affect scope
- **Part B: Gacha, loot box and battle pass design standards (2026)**
  - B1. Gacha banner parameter table
  - B2. The soft-pity hazard function (exact model, validated)
  - B3. Featured-unit guarantees: 50/50, Capturing Radiance, 75/25, Epitomized Path, sparks
  - B4. Loot-box guarantee structures
  - B5. Battle pass table: price, payback, expiry, structure
  - B6. Synthesized 2026 industry norms
- **Part C: Law and ratings (as of 2026-10-11)**
  - C1. Jurisdiction master table
  - C2. Jurisdiction notes and edge details
  - C3. Ratings trigger matrix
  - C4. Unified technical compliance spec (strictest common denominator)
- **Part D: Design implications for PALIMPSEST**
  - D1. Monetization architecture: "Earned Randomness, Paid Certainty"
  - D2. Exact gacha spec (Scriptorium), computed
  - D3. Exact loot box spec (Reliquary), with the appraisal tie-in
  - D4. Exact battle pass spec (Codex)
  - D5. Feature-bar commitments
  - D6. Avoid list
- **Part E: Open questions and unverified items**

---

## 0. Bottom line (read this first)

1. **Paid randomness is now a ratings and legal liability across the West and Brazil.** PEGI rates new submissions containing *paid random items* at a **minimum PEGI 16** from June 2026 `[VERIFIED https://www.videogameschronicle.com/news/all-games-with-loot-boxes-in-them-will-be-rated-minimum-pegi-16-starting-this-summer]`. Brazil's ECA Digital (Lei 15.211/2025), **in force since 2026-03-17**, bans loot boxes in games directed at or likely accessed by minors `[VERIFIED https://www2.camara.leg.br/legin/fed/lei/2025/lei-15211-17-setembro-2025-797997-publicacaooriginal-176498-pl.html]`. Belgium has treated paid loot boxes as illegal gambling since 2018 `[VERIFIED https://www.verdict.co.uk/belgium-loot-box-ban/ (search result)]`. Australia has required at least an M rating for chance-based purchases since 2024-09-22 `[VERIFIED https://classification.gov.au/about-us/media-and-news/news/new-classifications-for-gambling-content-video-games (search result)]`. The EU Digital Fairness Act proposal is expected in **November 2026** (MLex names 18 November as one tentative date) and has **not** been published yet `[VERIFIED https://www.mlex.com/mlex/articles/2529049 (search summary)]`.
2. **Earned-only randomness avoids almost all of this.** Every regime surveyed keys its rules on *payment*: PEGI "paid random items", ESRB "purchase with real money", the Brazil "mediante pagamento" definition, the Korean "direct or indirect payment", and the Australian "in-game purchases linked to elements of chance". Overwatch brought loot boxes back in February 2025 as **earn-only** items `[VERIFIED https://hitmarker.net/news/overwatch-2-is-getting-loot-boxes-perks-and-more-in-future-updates-blizzard-reveals-1275117 (search result)]`. **PALIMPSEST should ship gacha and loot boxes as fully realized systems funded only by earned currency, and sell only deterministic goods for money (Part D).**
3. **Gacha math is standardized and reproducible.** Genshin-family banners use a constant base rate, then add **(base rate × 10) per pull** after the soft-pity start `[VERIFIED https://library.keqingmains.com/evidence/general-mechanics/gacha]`. With 0.6%, a soft start at pull 74 and hard pity at 90, the model reproduces the officially published **1.600%** consolidated rate exactly (1.605%) `[COMPUTED]`.
4. **Battle pass norms in 2026:** the premium pass costs about **$8-10**. In premium-currency games it **pays back 100% or more** (Fortnite charges 800 and returns 800 V-Bucks from 2026-03-19; Call of Duty charges 1,100 and returns 1,100 CP) `[VERIFIED https://www.shacknews.com/article/148255/fortnite-v-bucks-price-increase (search result); https://www.callofduty.com/blog/2026/07/call-of-duty-black-ops-7-warzone-season-05-battle-pass-blackcell-store (search result)]`. **Non-expiring passes** are the pro-consumer standard (Halo Infinite, Helldivers 2, Marvel Rivals).
5. **The co-op bar is 1-4 players.** Slay the Spire 2 shipped 4-player co-op in Early Access on 2026-03-05 `[VERIFIED https://gameinformer.com/2026/02/19/slay-the-spire-2-reveals-4-player-co-op-march-early-access-release-date (search result)]`. Players expect crossplay that is on by default but can be turned off (ARC Raiders), cross-save (Hades II), and handheld support (Steam Deck, Switch 2, and Steam Machine since June 2026).
6. **Live service is fragile.** Destiny 2 shipped its **final live-service content update on 2026-06-09** `[VERIFIED https://rpgamer.com/2026/05/destiny-2-receiving-final-content-update-in-june/ (search result)]`. On 2026-06-16 the EU Commission declined to legislate on the "Stop Destroying Videogames" initiative and opened a **voluntary end-of-life code of conduct** process instead `[VERIFIED https://www.pocketgamer.biz/the-european-commission-rejects-calls-to-stop-publishers-from-taking-games-offline/ (search result)]`. PALIMPSEST should ship an offline and LAN path from day one.

---

# Part A: Genre peers and the 2026 quality bar

## A1. Landscape of reference titles, 2024-2026

| Title | Status / dates | Format | Players | Business model | Decision-relevant data | Tag |
|---|---|---|---|---|---|---|
| **Hades II** | Early Access May 2024 `[RECALLED]`; **1.0 on 2025-09-25** (PC, Switch, Switch 2) | Isometric action roguelite | 1 | Premium, **$29.99 digital**, $49.99 physical | Cross-save across Switch, Switch 2 and PC; 120 fps target on Switch 2; 63k+ Steam reviews before 1.0 | `[VERIFIED https://www.rpgsite.net/news/18410-hades-ii-10-launches-on-pc-nintendo-switch-switch-2-on-september-25-2025; https://fullcleared.com/news/hades-ii-exits-early-access-on-september-25/]` |
| **Slay the Spire 2** | **Early Access 2026-03-05**; EA expected to last 1-2 years | Deckbuilder roguelike | **1-4 co-op** | Premium **$24.99**, price rises after EA | Rebuilt in **Godot** after the Unity runtime-fee episode. Co-op adds shared map-route sketching, splashing potions on allies, and shared relic choices. Reported peak of **~177k concurrent players** (single outlet) | `[VERIFIED https://megacrit.com/news/2026-03-05-early-access-launch/ (search); https://www.techloy.com/slay-the-spire-2-sets-new-roguelike-record-on-steam-with-177k-concurrent-players/ (search); https://megacrit.com/faq/ (search)]` |
| **Risk of Rain 2: Alloyed Collective** | DLC released 2025-11-18 | 3D co-op roguelite shooter | 1-4 `[RECALLED]` | $15 DLC | 2 survivors (Drifter, Operator) and 6 stages; "extremely positive" early reviews | `[VERIFIED https://www.gematsu.com/2025/05/risk-of-rain-2-dlc-alloyed-collective-announced; https://www.bluesnews.com/s/291502/risk-of-rain-2-alloyed-collective-released (search)]` |
| **Enshrouded** | Early Access 2024-01-24; **1.0 on 2026-10-15** (PC, PS5); Xbox slipped to spring 2027 | Voxel survival-action RPG | Up to 16 (single source) | Premium | **5M+ players** in EA. Fully terraformable voxel terrain. Keen cited the cost of terraforming and building on two consoles as the reason for the Xbox delay | `[VERIFIED https://rpgsite.net/news/20782-enshrouded-full-release-appears-on-pc-playstation-5-worldwide-on-october-15 (search)]` |
| **Path of Exile 2** | EA December 2024; **1.0 on 2026-12-11**; goes **F2P at 1.0** | ARPG looter | 1-6 `[RECALLED]` | F2P: cosmetics and stash tabs | Adds the Duelist at 1.0. ExileCon 2026 runs 7-9 November | `[VERIFIED https://www.rpgsite.net/news/21188-path-exile-2-reaches-its-10-full-release-on-december-11 (search)]` |
| **Borderlands 4** | Launched September 2025 (2025-09-12 `[RECALLED]`) | Looter-shooter | 1-4 co-op `[RECALLED]` | Premium plus DLC | Up to **5M units** in the launch window, but Take-Two called sales **"softer than expected"** and blamed **PC performance**. 300k+ Steam concurrent players. Systems: **Licensed Parts** (cross-manufacturer parts), **Enhancements** (replace artifacts; drop from level 15), **Firmware** (from level 25) | `[VERIFIED https://www.notebookcheck.net/Take-Two-CEO-admits-Borderlands-4-sales-softer-than-expected-despite-record-launch-amid-PC-issues.1158771.0.html (search); https://borderlands.fandom.com/wiki/Enhancement (search)]` |
| **ARC Raiders** | Launched October 2025 | PvPvE extraction shooter | 1-3 `[RECALLED]` | **$40 premium** plus cosmetics | **12M+ sold**; **3.2M peak DAU** on 2026-01-04; monthly roadmap ("Escalation", January-April 2026). Crossplay **on by default with a toggle**. The free "Raider Deck" is being replaced by a **Reward Pass with free, Premium and Legacy tracks from Oct 8 (2026)** | `[VERIFIED https://www.gosugamers.net/entertainment/news/77798-arc-raiders-hits-12-million-copies-sold-and-breaks-player-records; https://games.gg/news/arc-raiders-paid-battle-pass/ (search)]` |
| **Marathon (Bungie)** | Launched **2026-03-05** | PvPvE extraction shooter | 1-3 `[RECALLED]` | **$39.99** (Deluxe $59.99) | Estimated **~1.2M copies** and ~$55M (Alinea); **478k peak DAU**; only 4 maps a month after launch | `[VERIFIED https://frvr.com/blog/bungies-marathon-has-reportedly-only-sold-1-2-million-copies-with-70-of-players-on-pc-via-steam/ (search)]` |
| **Dark and Darker** | Live on Steam as F2P with a paid "Legendary Status" upgrade; **removed from Epic Games Store on 2025-11-01** | Hardcore PvPvE dungeon extraction | 1-3 | F2P plus premium upgrade | ~5k concurrent players (low-confidence source) | `[VERIFIED https://www.easterneye.biz/dark-and-darker-epic-games-november/ (search)]` |
| **Elden Ring Nightreign** | Launched 2025-05-30; **5M shipped** by 2025-07-24 | Co-op roguelite action | 1-3 (no duo at launch, which drew criticism; solo, duo and trio now) | Premium | An expedition lasts **3 in-game days**. Days 1-2 are **about 15 min each**, then the Nightlord. Whole run **about 40-60 min**. Peak **313,593** concurrent players | `[VERIFIED https://www.bandainamcoent.co.jp/pdfs/HP_20250724_EN.pdf (search); https://deltiasgaming.com/?p=245379 (search)]` |
| **PEAK** | Launched 2025-06-16 | Co-op climbing ("friendslop") | 1-4 `[RECALLED]` | Premium, about $8 `[RECALLED]` | **10M+ copies by mid-August 2025**; proximity voice chat drives virality | `[VERIFIED https://en.wikipedia.org/wiki/Peak_(video_game); https://www.aol.com/peak-sells-10-million-copies-155741375.html (search)]` |
| **R.E.P.O.** | EA February 2025 `[RECALLED]` | Co-op horror physics | 1-6 `[RECALLED]` | Premium | Reported **16.8M** copies (single report) | `[VERIFIED https://app.cinevva.com/de/guides/co-op-game-design (search)]` |
| **Caves of Qud** | **1.0 in December 2024** after 15+ years (1.0 date 2024-12-05 `[RECALLED]`); patch 1.04 in May 2025; maintenance builds July-August 2025 | Systemic roguelike | 1 | Premium | Two further "feature arcs" planned (free or DLC not decided) | `[VERIFIED https://freeholdgames.itch.io/cavesofqud/community (search); https://fullcleared.com/news/live-and-drink-friends-caves-of-qud-1-0-now-live/ (search)]` |
| **Shattered Pixel Dungeon** | v3.1.1 (2025-06-18), v3.2.5 (2025-09-30), v3.3.8 (2026-03-23), **v4.0.0 (2026-09-09)**, v4.0.2 (2026-10-07) | Traditional roguelike | 1 | F2P with optional supporter IAP `[RECALLED]` | v4.0 is "arguably Shattered's biggest update yet": a large new quest, new art, **6 new enchantments**. v3.3 added the Skeleton Key artifact and the Cracked Spyglass trinket | `[VERIFIED https://github.com/00-Evan/shattered-pixel-dungeon/releases; https://shatteredpixel.com/blog/shattered-pixel-dungeon-v330.html (search)]` |
| **Deep Rock Galactic: Rogue Core** | **EA 2026-05-20** (announced October 2023; originally planned for November 2024) | Co-op roguelite FPS | 1-4 `[RECALLED]` | Premium | Slipped about 18 months | `[VERIFIED https://www.gematsu.com/2026/03/deep-rock-galactic-rogue-core-launches-in-early-access-on-may-20]` |
| **Megabonk** | Launched 2025-09-18 | 3D "survivors-like" roguelite | 1 | **$5.49** | **1M in 2 weeks**; peak 117,336 concurrent players; ~2.7M estimated by July 2026; 62.5% of owners also own Vampire Survivors | `[VERIFIED https://gameworldobserver.com/2025/10/03/megabonk-is-a-new-indie-hit-on-steam-the-games-sales-have-surpassed-one-million-copies (search)]` |
| **BALL x PIT** | 2025 | Roguelite or brick-breaker hybrid | 1 | Premium | 1M by December 2025 and 2M in about 10 months (excluding Game Pass); 3 free 2026 updates | `[VERIFIED https://www.pushsquare.com/news/2025/12/hit-ps5-roguelike-ball-x-pit-reaches-one-million-sales-free-content-updates-inbound (search)]` |
| **Hytale** | **EA 2026-01-13**, **$19.99** base | Voxel sandbox and adventure | MP | Premium | Modding is a "pillar" from launch; server source access planned post-launch (pre-launch statements) | `[VERIFIED https://insider-gaming.com/hytale-price-revealed-ahead-of-early-access-release/ (search); https://shockbyte.com/blog/hytale-release-date (search)]` |
| **Destiny 2** | Edge of Fate (July 2025), Renegades (December 2025); **final live-service update "Monument of Triumph" on 2026-06-09**; "The Shattered Cycle" reported cancelled | Looter-shooter MMO-lite | 1-6 | Expansions plus Rewards Pass plus Eververse | Development ended; the game stays playable | `[VERIFIED https://rpgamer.com/2026/05/destiny-2-receiving-final-content-update-in-june/; https://www.thesixthaxis.com/2026/05/21/destiny-2s-next-season-will-be-its-last-one-final-monument-of-triumph-as-the-director-returns/ (search)]` |
| **Dungeon Crawler Carl** | **No video game adaptation found.** Peacock straight-to-series TV order (in development as of August 2026); Renegade tabletop RPG and co-op deckbuilder ("Unstoppable") on BackerKit, April 2026 | n/a | n/a | n/a | The IP's "game-show dungeon" framing (a system AI, sponsors, viewer-voted loot boxes) is free design inspiration. **No licensed game competes in this space.** | `[VERIFIED https://www.thewrap.com/creative-content/tv-shows/dungeon-crawler-carl-straight-to-series-order-peacock-seth-macfarlane/; https://retailers.renegadegamestudios.com/blog/renegade-game-studios-announces-two-upcoming-dungeon-crawler-carl-tabletop-games (search)]` |

## A2. The 2026 "table stakes" feature bar

| Dimension | 2026 minimum bar | Best-in-class reference | Evidence |
|---|---|---|---|
| **Co-op size** | 1-4 players, drop-in. Every party size (solo, duo, trio, quad) must be tuned on day 1 | Slay the Spire 2 (4p co-op, even for a deckbuilder). Nightreign was criticized for launching without duos | `[VERIFIED gameinformer StS2 link above; https://www.notebookcheck.net/Elden-Ring-Nightreign-Tops-June-2025-U-S-Sales-Charts-Outsells-Death-Stranding-2-and-Mario-Kart-World.1066112.0.html (search)]` |
| **Co-op interaction** | Mechanics that make players depend on each other, not parallel solo play | StS2 lets players splash potions on allies and share relic choices. PEAK and R.E.P.O. use proximity voice and physics-based cooperation | `[VERIFIED (above)]` |
| **Crossplay** | Crossplay **on by default**, with an opt-out toggle in Settings, then Gameplay | ARC Raiders. Opting out lengthens queues, and on console limits matching to the same console family | `[VERIFIED https://godisageek.com/?p=312743 (search)]` |
| **Cross-save / cross-progression** | Expected on PC plus handheld | Hades II (Switch, Switch 2, PC) | `[VERIFIED rpgsite Hades II link]` |
| **Handheld / Deck** | Steam Deck Verified. Ship a Switch 2 build if the engine allows | Hades II (120 fps target on Switch 2). Valve widened the Verified program to Steam Machine and Steam Frame (summer 2026) | `[VERIFIED https://videogameschronicle.com/news/steam-machine-and-steam-frame-confirmed-for-summer-2026-by-valve/ (search)]` |
| **Accessibility** | Fill out Steam's accessibility questionnaire. Store pages have shown accessibility features since **June 2025** (5,000+ apps had filled it in by then). Provide difficulty assists, subtitles, color alternatives, remapping, and "playable without timed input" where possible | Hades "God Mode": **+20% damage resistance at start, +2% per death, capped at 80%** (same in Hades II) | `[VERIFIED https://gamingonlinux.com/2025/06/steam-adds-accessibility-features-to-store-pages-and-filters-for-searching; https://caniplaythat.com/2021/08/11/hades-god-mode-explained-by-supergiant-games (search)]` |
| **Modding** | A data-driven content pipeline plus a mod loader. Engine choice matters to modders | StS2's Godot rewrite was explicitly linked to "more robust modding". Hytale treats modding as a pillar. Caves of Qud supports XML/C# mods and Workshop `[RECALLED]` | `[VERIFIED https://viblo.asia/p/tai-sao-slay-the-spire-2-chon-godot-thay-vi-unity-va-du-lieu-doanh-thu-thuc-te-RlL5e8gPLbB (search)]` |
| **Seeds / dailies** | Shareable seeds plus one daily seeded run with a leaderboard | Slay the Spire daily climb and custom seeds `[RECALLED]`; SPD seeded runs (v1.x+) `[RECALLED]` | `[RECALLED]` |
| **Run length** | One session per run: **about 40-60 min** is the 2025-2026 sweet spot for co-op roguelites. Survivors-likes run 10-30 min; traditional roguelikes 2-6 h | Nightreign about 40-60 min (3 days, days 1-2 about 15 min each) | `[VERIFIED deltiasgaming Nightreign link]` |
| **Meta-progression** | Permanent unlocks that widen options more than they add raw power. Optional difficulty "vows" or heat | Hades II (Arcana, Incantations, Fear) `[RECALLED]`; RoR2 unlocks via challenges `[RECALLED]` | `[RECALLED]` |
| **Live cadence** | Free content updates about **every 4-8 weeks** for premium games. Paid DLC every 9-18 months | ARC Raiders: monthly named updates. BALL x PIT: 3 free updates in 2026. RoR2: about $15 DLC per 1-2 years | `[VERIFIED (above)]` |
| **Launch content breadth** | Enough maps or biomes to avoid "thin at launch" reviews | Marathon criticized for **4 maps** a month after launch | `[VERIFIED frvr/gfinity Marathon links]` |
| **PC performance** | Stable frame pacing on mid-range PCs at launch. This has a direct commercial effect | Borderlands 4 sales were "softer" and Take-Two blamed PC issues | `[VERIFIED notebookcheck BL4 link]` |
| **Price points (premium)** | $5.49 (Megabonk) to $40 (ARC Raiders, Marathon). Indie roguelites cluster at **$20-30** (Hades II $29.99, StS2 $24.99, Hytale $19.99) | n/a | `[VERIFIED (above)]` |

## A3. Per-title lessons worth stealing or avoiding

- **Slay the Spire 2:** proves even turn-based and deckbuilding roguelikes are moving to **4-player co-op**. Its co-op verbs are good to steal because they create *interdependence*: shared route planning (map sketching), cross-player consumables (potion splash), and shared reward draft (relic choice) `[VERIFIED]`. **Steal for PALIMPSEST:** shared Folio-route voting with drawn annotations on the map, thrown potions that hit allies with the full effect, and "Reliquary drafts" where the party picks in turn order.
- **Elden Ring Nightreign:** the timed, shrinking-circle "day" (4.5 min before the first circle starts to close, then 3 min to close fully, per one reviewer's test) is a strong compression device that turns a big open map into a 40-60 min run `[VERIFIED https://deltiasgaming.com/?p=247780 (search)]`. **Lesson:** a pressure clock that drives the party toward a boss is how a 3D map stays session-sized. **Avoid** launching without every party size tuned.
- **Borderlands 4:** Licensed Parts (weapon parts from other manufacturers) and Enhancements (gear that buffs any gun whose licensed part matches its manufacturer, e.g., Hyperion Stabilizer +40 accuracy) form a **cross-item synergy layer** that makes build-crafting combinatorial `[VERIFIED https://www.sportskeeda.com/esports/what-licensed-parts-system-borderlands-4 (search)]`. **Steal** this pattern across genres: a Folio's "genre license" part can appear on gear from another genre, and Enhancement-like relics key off the genre tag. **Avoid** its PC performance problems.
- **ARC Raiders:** mass-market success with a **$40 premium price plus cosmetics**, a monthly cadence, crossplay on by default, and a **free** first progression deck. It moved to a free plus paid Reward Pass plus a **Legacy pass** for older decks only in October 2026, a year after launch `[VERIFIED]`. **Lesson:** launch with a generous free track, and add premium tracks only once retention is proven.
- **Shattered Pixel Dungeon:** a solo developer shipping steady incremental updates (v3.1 to v4.0 in about 15 months), with each update adding *systemic* content (new enchantments, trinkets, quest rewrites) rather than raw volume `[VERIFIED]`.
- **Megabonk:** a $5.49 3D survivors-like reached 1M in 2 weeks. It shows the **3D roguelite audience is huge** and favours instant readability and fast loops `[VERIFIED]`.
- **Destiny 2:** a looter-shooter that monetized with expansions, passes and an Eververse shop ended development in 2026 `[VERIFIED]`. **Lesson:** do not tie the game's core value to an always-online live service that has to be funded indefinitely.
- **Enshrouded:** shipping voxel terraforming on consoles is costly enough to delay an Xbox port by about 6 months `[VERIFIED]`. **Lesson for PALIMPSEST's voxel layer:** budget console performance work early. Destruction should use a chunk-local, deterministic, server-authoritative model.

## A4. Live-service health signals, 2025-2026

| Signal | Detail | Tag |
|---|---|---|
| Destiny 2 end of development | Final update 2026-06-09; Bungie moving on to incubating new games; the game stays playable | `[VERIFIED rpgamer, tbreak.com/destiny-2-monument-triumph-final-update-2026/ (search)]` |
| Marathon soft launch | ~1.2M estimated sales; 4 maps at launch drew criticism | `[VERIFIED]` |
| ARC Raiders strong | 12M+ sold; monthly content | `[VERIFIED]` |
| "Stop Destroying Videogames" ECI | **1,294,188** validated signatures; submitted 2026-01-26; EP hearing April 2026; **Commission reply 2026-06-16: no binding law**; voluntary industry code of conduct on end-of-life management, with talks to start by end-2026; review of the Digital Content Directive | `[VERIFIED https://heise.de/-11154579 (search); https://www.pocketgamer.biz/the-european-commission-rejects-calls-to-stop-publishers-from-taking-games-offline/ (search); https://www.mondaq.com/consumer-law/1808560/ (search)]` |
| Friendslop boom | PEAK 10M+, R.E.P.O. ~16.8M, Lethal Company. Cheap co-op with proximity chat and emergent physics comedy | `[VERIFIED]` |

## A5. Platform and infrastructure facts that affect scope

- **Steam accessibility features** have been displayed and searchable since June 2025 `[VERIFIED gamingonlinux 2025/06]`.
- **Steam Machine** launched about late June 2026 at about **$1,049-1,050** (512 GB). The Steam Frame VR headset was confirmed for summer 2026 with no confirmed price in the sources found. Both are in the Verified program `[VERIFIED https://roadtovr.com/steam-machine-launches-next-week-starting-at-1050-hints-at-what-to-expect-from-steam-frames-launch/ (search); https://pcworld.com/article/3172143/the-steam-machine-finally-has-a-price-and-date-1049-june-25th.html (search)]`. **Implication:** "Steam Deck Verified" now extends to the living-room Steam Machine, so ship a 10-foot UI and full controller support.
- **Nintendo Switch 2** is a live target (Hades II 1.0 shipped on it on day 1) `[VERIFIED]`.
- **European Accessibility Act** enforceable since **2025-06-28**. Video games are not explicitly named. **E-commerce services** are in scope, and whether in-game stores count depends on national transposition. Microenterprises (<10 staff, <€2M turnover) are partly exempt `[VERIFIED https://www.taylorwessing.com/zh-hant/insights-and-events/insights/2025/03/accessibility-in-the-gaming-industry (search); https://www.traverssmith.com/knowledge/knowledge-container/a-new-milestone-for-accessibility-the-european-accessibility-act-now-applies/ (search)]`. **Make the store UI WCAG-2.1-AA-equivalent regardless.**

---

# Part B: Gacha, loot box and battle pass design standards (2026)

## B1. Gacha banner parameter table

"Base" is the per-pull rate before pity. "Consolidated" is the published average including pity. "Soft start" is the first pull with a raised rate (community-derived, never officially published by HoYoverse).

| Game / banner | Top-tier base | Consolidated | Hard pity | Soft-pity start | Featured rule | Second tier | Cost per pull | Tag |
|---|---|---|---|---|---|---|---|---|
| **Genshin Impact: character event** (v7.1, October 2026) | **0.600%** | **1.600%** | **90** | 74 (community) | **50/50**. A loss guarantees the next 5★. **Capturing Radiance** raises the consolidated featured share to **55.000%** | 4★ **5.100%** base / **13.000%** consolidated; ≥1 4★ per 10 pulls (resolves 99.4% 4★ / 0.6% 5★) | 160 Primogems `[RECALLED]` | `[VERIFIED https://timesaver.gg/blog/genshin-pity-system-explained; https://library.keqingmains.com/evidence/general-mechanics/gacha]` |
| **Genshin: weapon event** | **0.700%** | **1.850%** | **80** | 63 (community) | 75% for the promotional pair (37.5% each), 25% standard; a loss guarantees a promotional weapon next. **Epitomized Path: max 1 Fate Point** (since the 2024-08-14 news post) | 4★ **6.000%** / **14.500%**; 75% featured 4★ | 160 `[RECALLED]` | `[VERIFIED timesaver]` |
| **Genshin: standard** | 0.600% (0.300% character + 0.300% weapon) | 1.600% | 90 | 74 | n/a | 5.1% / 13.0% | 160 | `[VERIFIED timesaver]` |
| **Honkai: Star Rail: character warp** | 0.6% | 1.6% | 90 | ~74 `[RECALLED]` | 50/50 plus guarantee | 4★ 5.1% `[RECALLED]` | 160 Stellar Jade `[RECALLED]` | `[VERIFIED https://vip-develop.primagames.com/?p=121647 (search)]` |
| **HSR: light cone warp** | **0.8%** | **1.87%** | **80** | ~66 `[RECALLED]` | **75/25** plus guarantee | 4★ 6.6% `[RECALLED]` | 160 | `[VERIFIED primagames (search)]` |
| **Zenless Zone Zero: exclusive (agent)** | 0.6% | 1.6% | 90 | ~74 `[RECALLED]` | 50/50 plus guarantee; A-rank guaranteed in 10 | n/a | 160 Polychrome `[RECALLED]` | `[VERIFIED https://oneesports.gg/zenless-zone-zero/zenless-zone-zero-pity-system (search)]` |
| **ZZZ: W-Engine** | 1.0% (single source) | 2.0% (single source) | **80** | n/a | **75%** featured plus guarantee | n/a | 160 | `[VERIFIED https://gamewith.net/zenless-zone-zero/46129 (search)]` |
| **Wuthering Waves: featured resonator** | **0.8%** | 1.6-1.8% (sources disagree) | **80** | ~66 (community, disputed: 50-66) | 50/50 plus guarantee | 4★ 6% | 160 Astrite `[RECALLED]` | `[VERIFIED https://www.pcinvasion.com/wuthering-waves-gacha-rates-and-pity-system-explained/ (search)]` |
| **WuWa: featured weapon** | 0.8% | n/a | **80** | n/a | **100% featured** (no 50/50) | 6% | 160 | `[VERIFIED (search)]` |
| **Arknights: Endfield: Chartered (limited) headhunting** | **0.8%** 6★ (5★ 8%, 4★ 91.2%) | n/a | **80** (any 6★) | **66** (+**5 percentage points per pull** after 65 without a 6★) | 50/50 at the 80-pull guarantee. **Pull 120 on a Chartered banner forces the featured 6★** (once per banner, no carryover). **240 pulls on the same banner** grants a token for one more copy. **Headhunting Dossier:** after 60 pulls on a limited banner, one free 10-pull on the next Chartered banner. 6★ pity carries over; the 120 and 240 counters do not | ≥1 5★ per 10 (carries over) | **500 Oroberyl** (10-pull 5,000) | `[VERIFIED https://allthings.how/arknights-endfield-gacha-and-pity-explained-in-plain-language/ (search fetch)]` |
| **Endfield: Basic headhunting** | 0.8% | n/a | 80 | 66 | No 50/50. **Selector unlocks at 300 total pulls** | n/a | 500 | `[VERIFIED allthings.how]` |
| **Endfield: Arsenal (weapons)** | 6★ 4% per result | n/a | 6★ within **40 pulls** (25% featured) | n/a | Featured forced after 8 non-featured 6★s. **Arms Offering:** crate at 100 pulls, featured at 180, crate at 260, featured at 340, alternating every 80 thereafter. No carryover | 5★ guaranteed every 10-pull | **1,980 Arsenal Tickets** per 10-pull (single pulls not offered). Tickets come from character pulls: 6★ = 2,000, 5★ = 200, 4★ = 20. 30 Oroberyl buys 10 tickets | `[VERIFIED allthings.how]` |
| **Arknights (original): headhunting** | 2% 6★ | n/a | none (soft ramp) | after 50 pulls without a 6★, +2% per pull `[RECALLED]` | Limited banners: **300-pull spark** `[RECALLED]` | n/a | 600 Orundum per pull | `[VERIFIED https://west-games.com/arknights-pity-calculator/ (600 Orundum; post-50 ramp)]`; increment `[RECALLED]` |
| **Granblue Fantasy** | n/a | n/a | n/a | n/a | **300-draw spark**: pick any banner unit `[RECALLED]` | n/a | 300 crystals `[RECALLED]` | `[RECALLED]` |
| **Blue Archive** | 3% 3★ `[RECALLED]` | n/a | n/a | n/a | **200-recruitment-point exchange (spark)** `[RECALLED]` | n/a | 120 Pyroxene `[RECALLED]` | `[RECALLED]` |

## B2. The soft-pity hazard function (exact model, validated)

**Source formula:** after the soft-pity start, "the drop rate increases with (the base rate × 10) every pull" `[VERIFIED https://library.keqingmains.com/evidence/general-mechanics/gacha]`. Soft-pity starts in that source: 74 (character and standard), 63 (weapon), 9 (4★) `[VERIFIED]`.

**Canonical hazard function** (per-pull probability of the top tier on pull number `n` since the last top-tier result):

```
p(n) = b                                  for 1 <= n < S
p(n) = min(1, b + k * (n - S + 1))        for S <= n < H
p(n) = 1                                  for n = H
where b = base rate, S = soft-pity start pull, k = per-pull increment (= 10*b in HoYoverse games), H = hard pity
P(first top-tier at pull n) = p(n) * PROD_{i<n} (1 - p(i))
E[pulls] = SUM_{n=1..H} n * P(first at n);  consolidated rate = 1 / E[pulls]
```

**Validation against published rates** `[COMPUTED in-session; parameters VERIFIED]`:

| Model | b | S | k | H | E[pulls] | Model consolidated | Published consolidated | Median | 90th pct | 99th pct | P(hit before soft pity) |
|---|---|---|---|---|---|---|---|---|---|---|---|
| Genshin character 5★ | 0.006 | 74 | 0.06 | 90 | **62.297** | **1.605%** | 1.600% | 76 | 80 | 83 | 35.55% |
| Genshin weapon 5★ | 0.007 | 63 | 0.07 | 80 | **53.250** | **1.878%** | 1.850% | 65 | 69 | 71 | 35.31% |
| Genshin 4★ | 0.051 | 9 | 0.51 | 10 | **7.655** | **13.063%** | 13.000% | 9 | 10 | 10 | 34.21% |
| Endfield 6★ | 0.008 | 66 | 0.05 | 80 | **53.899** | **1.855%** | n/a | 67 | 73 | 76 | 40.67% |

The model matches the official character figure to within 0.005 percentage points. **This is the standard formula. PALIMPSEST should implement exactly this function, with b/S/k/H as data fields.**

Design reading: about 35% of top-tier results land before soft pity, and about 65% land in a narrow window 2-10 pulls after the soft start. That makes the *experienced* pity point predictable, which is the point of the design. Players plan around "soft pity", not around the base rate.

## B3. Featured-unit guarantees

| Mechanic | Exact rule | Effect | Tag |
|---|---|---|---|
| **50/50 + guarantee** (Genshin/HSR/ZZZ/WuWa character banners) | First top-tier result: 50% featured. On a loss, the next top-tier result is 100% featured. The guarantee persists across banners of the same type `[RECALLED for persistence]` | Featured: E = **93.45** pulls, 90th pct 155, worst case 180 (2×90) | `[COMPUTED; rule VERIFIED timesaver]` |
| **Capturing Radiance** (Genshin, since 5.0) | Applies only when the 50/50 is in effect (not during a guarantee) and only on Character Event Wish 1 and 2. **Published base trigger chance 0.018%** (as listed, unclear unit). **Consolidated featured share 55.000%.** **Forced trigger:** if the promotional character comes as your *second* 5★ three consecutive times, the next 5★ win triggers it. Does not affect existing guarantees | Featured: E ≈ **90.33** (55/45 model); timesaver gives "91 average, 180 worst". All-in rate for the promotional character **1.103%** | `[VERIFIED timesaver; COMPUTED]` |
| **75/25 + guarantee** (HSR light cones, ZZZ W-Engines, Genshin weapon pair) | First top-tier: 75% featured; a loss guarantees the next | Genshin weapon: specific promotional weapon **68 average / 160 worst** (with Epitomized Path) | `[VERIFIED timesaver]` |
| **Epitomized Path** (Genshin weapons) | Choose 1 of 2 promotional weapons. Each 5★ weapon that isn't the chosen one gives +1 Fate Point. **Max = 1** (changed from 2 in 2024). At max, the next 5★ weapon is the chosen one. Resets on obtaining the chosen weapon, changing course, or the banner ending | Caps a specific weapon at 2 five-stars | `[VERIFIED timesaver]` |
| **Per-banner forced featured** (Endfield) | Pull 120 forces the featured unit. Resets at banner end | A hard cap on the worst case for a featured unit | `[VERIFIED allthings.how]` |
| **Spark / exchange** | Granblue 300 draws, Blue Archive 200 points, Arknights 300 on limited banners `[RECALLED]`. Endfield basic selector at 300 `[VERIFIED]` | A deterministic ceiling on the cost of a *specific* unit. The most player-trusted mechanic | mixed |

## B4. Loot-box guarantee structures

- **Overwatch (from Season 15, 2025-02-18):** loot boxes returned, **earn-only** (weekly rewards, events, free and premium battle pass), **not purchasable for real money**. Published guarantees: **every box contains at least one Rare item; an Epic every 5th box; a Legendary every 20th box**. The free pass includes **1 Legendary box**, the premium pass **2** `[VERIFIED https://hitmarker.net/news/overwatch-2-is-getting-loot-boxes-perks-and-more-in-future-updates-blizzard-reveals-1275117 (search); https://gamedeveloper.com/business/overwatch-2-brings-loot-boxes-back-from-the-grave (search)]`. Before the ECA Digital took effect, Blizzard reportedly removed **paid** Overwatch loot boxes from Brazil while free boxes stayed `[VERIFIED https://www.hardware.com.br/?p=354338 (search)]`.
- **Pattern to steal:** a **deterministic bad-luck counter per tier** ("every Nth box contains ≥ tier T"). It is easy to disclose, satisfies the Korean "fixed rule" disclosure standard (C2), and needs no hidden variable odds.

## B5. Battle pass table: price, payback, expiry, structure

| Game | Premium price | Payback | Expiry | Structure | Tag |
|---|---|---|---|---|---|
| **Fortnite** | **800 V-Bucks** (from **2026-03-19**; previously 1,000) | **800 V-Bucks** on completion (enough for the next pass); bonus V-Bucks discontinued (previously up to 1,500 back). V-Buck packs shrank at the same time ("help pay the bills") | Seasonal (expires) `[RECALLED]` | ~100 base rewards, then bonus pages `[RECALLED]` | `[VERIFIED https://respawn.outlookindia.com/gaming/gaming-news/epic-games-fortnite-restructures-v-bucks-packs-as-costs-rise (search); shacknews]` |
| **Call of Duty: Black Ops 7** | **1,100 COD Points**; bundle **2,400 CP** = pass + **20 tier tokens**; **BlackCell ≈ $30** real money | Earn **1,100 CP** back through the pass. Upgrading to BlackCell after buying the pass refunds **1,100 CP** | Seasonal | **100+ rewards across 14 pages plus a completion page** | `[VERIFIED https://www.callofduty.com/blog/2026/07/call-of-duty-black-ops-7-warzone-season-05-battle-pass-blackcell-store (search); gamingtrend (search)]` |
| **Apex Legends** | **950 Apex Coins** (restored after backlash). Cash-only **Ultimate $9.99** and **Ultimate+ $19.99** | Up to **1,300 AC** from the Premium/Ultimate/Premium+ passes | Seasonal; **2 passes per season** (since Season 22) | July 2024: the plan to sell 60-tier half-season passes for $9.99 only was reversed within about two weeks | `[VERIFIED https://www.techradar.com/gaming/consoles-pc/respawn-backtracks-on-its-apex-legends-battle-pass-changes (search); gosugamers 72288 (search)]` |
| **Marvel Rivals** | Luxury **990 Lattice** (≈$9.99). Upgraded **2,100 Lattice** ($19.99) adds 2,800 Chrono Tokens and a +20% Chrono Token boost | **600 Lattice + 600 Units** (+600 Unstable Molecules) reported for one season | **Purchased luxury passes never expire**; past-season rewards need Chrono Tokens; a season's pass can't be bought after the season ends | Lattice: 100 = $0.99, 11,680 = $99.99; Units convert to Lattice 1:1 | `[VERIFIED https://www.gfinityesports.com/article/marvel-rivals-battle-pass-will-not-expire-raising-shooter-standards; https://mp1st.com/news/marvel-rivals-monetization-explained-battle-pass-wont-expire; https://www.esports.net/news/marvel-rivals-battle-pass (search)]` |
| **Helldivers 2** | Premium Warbond **1,000 Super Credits**; Legendary (e.g., Halo ODST) **1,500 SC**; 2,100 SC = $19.99 | Most non-legendary Warbonds give **300 SC** if enough medals are earned; SC also drop in missions `[RECALLED]` | **Never expire** ("Warbonds will never disappear") | Pages unlocked by spending earned **Medals**; the Warbond must be bought before its medals can be spent | `[VERIFIED https://windowscentral.com/gaming/helldivers-2s-new-cutting-edge-warbond-is-live-heres-everything-new-and-the-price-youll-pay-for-libertys-latest-toys (search); insider-gaming (search)]` |
| **Halo Infinite** | 1,000 credits `[RECALLED]` | n/a | **Never expire**; old passes purchasable later; **one active pass at a time**, switchable | Progression was originally **challenge-only XP**, which drew criticism for slow levelling and was later eased | `[VERIFIED https://egmnow.com/halo-infinites-multiplayer-battle-passes-never-expire-can-be-purchased-any-time/; https://videogameschronicle.com/news/halo-infinites-battle-pass-is-being-changed-to-make-it-easier-to-earn-xp (search)]` |
| **Destiny 2** | Rewards Pass **1,000 Silver** (Bungie help page). Earlier Season Pass rose 1,000 to **1,200 Silver** at Season 21 | none | Tied to content drops. Edge of Fate standard edition $40 incl. 1 Rewards Pass; Year of Prophecy $80 incl. **4** passes | Season 28 ranks run to **110**; the pass gives up to +40% XP bonus (guide claim). **Development ended 2026-06-09** | `[VERIFIED https://help.bungie.net/hc/articles/360048722772 (search); https://www.videogameschronicle.com/news/bungie-is-raising-destiny-2-season-pass-prices/ (search); esports.gg (search)]` |
| **ARC Raiders** | Premium Reward Pass (price not found) | n/a | **Legacy Pass** lets players catch up on older Raider Deck rewards | Free + Premium + Legacy, from **Oct 8 (2026)** | `[VERIFIED games.gg (search)]` |

**Battle-pass XP curves:** the sources found give no exact per-level XP for Fortnite, CoD or Apex in 2026. Historical memory says Fortnite used a flat 80,000 XP per level in Chapters 3-5 `[RECALLED]` and Destiny 2 seasons used 100,000 XP per rank `[RECALLED]`. **The industry norm is flat XP per tier, not escalating**, so time-to-tier is predictable and tier-skip prices are linear `[RECALLED]`.

## B6. Synthesized 2026 industry norms (what "meets the standard" means)

1. **Pity is mandatory, visible and portable.** Hard pity at 80-90 pulls for a ~0.6-0.8% base, soft pity about 15 pulls before hard pity, and pity carried across banners of the same type `[VERIFIED tables above]`.
2. **The featured-loss guarantee is mandatory.** Pure 50/50 is being softened (Capturing Radiance at 55%, 75/25 on weapons, 100% on WuWa weapons, forced featured at Endfield pull 120).
3. **A deterministic ceiling (spark) is the trust anchor.** Endfield 120 (featured) and 300 (selector) `[VERIFIED]`.
4. **Premium battle passes cost about $8-10 and pay back at least 100% in premium currency** (Fortnite 800/800, CoD 1,100/1,100) `[VERIFIED]`. The profit tier is a **cash-only upgrade** (BlackCell about $30, Apex Ultimate $9.99/$19.99) `[VERIFIED]`.
5. **Non-expiring passes are a competitive differentiator** (Halo Infinite, Helldivers 2, Marvel Rivals, ARC Raiders Legacy) `[VERIFIED]`.
6. **Earn-only loot boxes are back as engagement, not revenue** (Overwatch 2025) `[VERIFIED]`.
7. **Raising effective prices draws backlash:** the Apex reversal (2024) and Fortnite's 2026 V-Buck shrink `[VERIFIED]`.

---

# Part C: Law and ratings (as of 2026-10-11)

> Not legal advice. This is a research summary to drive engineering requirements. Every row is tagged, and secondary sources are flagged.

## C1. Jurisdiction master table

| # | Jurisdiction / body | Instrument | Status and effective date | Core rule | What a compliant implementation must do (technical) | Tag |
|---|---|---|---|---|---|---|
| 1 | **Belgium** | Gaming Act 1999 as applied by the Belgian Gaming Commission (April 2018 findings on FIFA 18, CS:GO, Overwatch) | In force since 2018. No licence is obtainable, so paid loot boxes are effectively illegal. Enforcement is weak: in 2021/22, **82 of the top 100 grossing iPhone games** still had loot boxes (Xiao 2023) | Paid loot boxes = games of chance. Penalties up to **€800,000 and 5 years**, **doubled if minors are involved** | `region=BE` → the paid random items flag is hard-disabled (store region **and** billing country **and** IP heuristic; strictest wins). Earned-only random rewards are fine. Nintendo, EA, Valve and Psyonix withdrew paid loot boxes in BE; Diablo Immortal did not launch there | `[VERIFIED https://www.verdict.co.uk/belgium-loot-box-ban/ (search); https://www.pcgamesn.com/belgium-loot-box-laws-study (search); https://www.mcvuk.com/business-news/belgium-isnt-enforcing-loot-box-ban-claims-research-study/ (search)]` |
| 2 | **Netherlands** | Betting and Gaming Act (KSA view, 2018) | KSA held loot boxes illegal **only if prizes are transferable or sellable**. The highest administrative court (Raad van State, 2022) overturned the KSA's EA penalty. A 2022 motion backed by parties holding 94/150 seats called for a ban. The government wants the ban to come via the **EU DFA** because EU consumer law is maximum-harmonisation. **No national ban in force** found | Transferable plus chance plus market value = gambling | Make all randomized rewards **account-bound and non-tradeable**, with no cash-out. If a trading system exists, exclude anything that came from a random container | `[VERIFIED https://www.gamblinginsider.com/news/5442/valve-is-the-first-casualty-of-dutch-loot-box-crackdown (search); https://berthub.eu/tkconv/getraw/2024D50627 (search); https://www.levelup.com/en/news/dutch-political-parties-back-motion-to-ban-loot-boxes/ (search)]`; 2022 date `[RECALLED]` |
| 3 | **United Kingdom** | DCMS-convened **industry principles** (Ukie), published **2023-07-18**; 11 principles | Self-regulation. DCMS keeps "future legislative options under review"; a 12-month implementation period, then evaluation. Academic research (Royal Society Open Science) reports "widespread" non-compliance | Under-18s cannot acquire paid loot boxes **without parental consent**. Tech controls beyond self-declaration. Clear probability disclosure. Explain how player data affects odds. **Default £0 spend limit on child accounts**. Lenient refunds | Age gate plus parental-consent flow for <18 before any paid random item. Odds screen. Refund API for unauthorized child purchases | `[VERIFIED https://www.gov.uk/guidance/loot-boxes-in-video-games-update-on-improvements-to-industry-led-protections; https://5rightsfoundation.com/research-reveals-non-existent-enforcement-of-industry-led-standards-on-loot-boxes (search)]` |
| 3b | **UK (general consumer law)** | **DMCC Act 2024**, consumer provisions | In force **2025-04-06**. The CMA can fine directly up to **10% of global turnover**. Drip pricing and fake reviews banned. First drip-pricing fine April 2026 (£4.2M) | All-in pricing up front. No hidden mandatory fees | Show the **total real-money price** before purchase. No surprise fees at checkout | `[VERIFIED https://cms.law/en/gbr/legal-updates/the-dmcc-act-consumer-elements-come-into-force-from-6-april-2025 (search); https://www.jonesday.com/en/insights/2026/04/hidden-fees-heavy-fines-the-cmas-consumer-protection-crackdown-gathers-pace (search)]` |
| 4 | **EU: CPC Network** | **Key principles on in-game virtual currencies**, published **2025-03-21** with the Star Stable action | Soft law, framed as "minimum requirements". The industry (VGE, EGDF) disputes it | (1) clear, transparent pricing and pre-contract info; (2) no practices that hide the cost of digital content or force currency purchases; (3) respect the right of withdrawal; (4) respect vulnerabilities, especially children's. Commentary reading: show the **real-money price**, using the **most expensive conversion path** when currencies are layered; **no forced overspend** (e.g., buying 2,000 currency for a 1,300 item) | Show the **fiat price next to every virtual-currency price**. Sell currency in amounts that **exactly match** item prices, or sell items directly in fiat. Show withdrawal info and take explicit consent to waive withdrawal on first use. No pressure timers | `[VERIFIED https://www.forbrukertilsynet.no/eng-articles/national-consumer-authorities-and-commission-take-action-on-harmful-practices-in-the-gaming-industry; https://raminshokrizade.substack.com/p/110c-how-to-come-into-compliance (commentary)]` |
| 5 | **EU: Digital Fairness Act** | Commission legislative proposal | Consultation **2025-07-17 to 2025-10-24**. Proposal **expected November 2026** (MLex 2026-09-23; one tentative date 2026-11-18). **Not published as of 2026-10-11**. Under consideration: mandatory real-money price display, possible loot box bans or parental consent for minors, design features "disabled by default or prohibited" in children's games. EP committee position: ban loot boxes, in-app currencies and pay-to-win in games likely accessed by minors | n/a (pending) | Build every item listed in C4 behind data-driven flags now. Expect, once adopted (2027-2028), a requirement for fiat price display and a probable ban on paid loot boxes for minors | `[VERIFIED https://en.wikipedia.org/wiki/Digital_Fairness_Act; https://technologyquotient.freshfields.com/post/102ltio/the-eus-proposed-digital-fairness-act-a-game-developers-guide-to-potential-imp (search); https://blog.promise.legal/lootbox-regulation-2026-game-studios/ (search); MLex 2529049 (search)]` |
| 6 | **EU: European Accessibility Act** | Directive (EU) 2019/882 `[RECALLED number]` | Enforceable **2025-06-28** | Covers e-commerce services. Games are not named | Accessible store UI: screen-reader labels, contrast, keyboard and controller navigation | `[VERIFIED traverssmith (search)]` |
| 7 | **PEGI** (pan-European rating) | New "interactive risk" criteria, announced **2026-03-12** | Applies to **new submissions from June 2026**. Existing titles are not re-rated unless updates warrant it (one unconfirmed report says reclassification of existing games starts 2026-10-01) | **Paid random items → min PEGI 16** (some PEGI 18). **Time- or quantity-limited offers** (e.g., paid battle passes with countdown timers) → **min PEGI 12**, lowered to **PEGI 7 if spending is disabled by default**. **Play-by-appointment** (daily quests, streaks) → **min PEGI 7**, or **12** if absence is punished. **NFT/blockchain** that is required and tradeable → **18**. **Unrestricted communication with no block or report** → **18**. Social casino → 18 case by case | No paid random items. No countdown timers on paid offers. Purchases **disabled by default**, with parental enabling. No punishing absence. Block and report on every comms channel. No NFTs | `[VERIFIED https://mediawrites.twobirds.com/post/102mn2b/pegi-age-rating-classification-update; https://www.reedsmith.com/articles/pegi-launches-interactive-risk-categories-overhauls-age-ratings-for-loot-boxes-in-game-spending-and-communication-features/; VGC link]`; Oct 1 date `[VERIFIED single secondary source askaboutgames (search), unconfirmed]` |
| 8 | **Germany: USK** | JuSchG 2021 §10b plus USK criteria | Since **2023-01-01**, online risks (purchases, gambling-like mechanics, communication) can raise the rating. Descriptor "In-Game-Käufe + zufällige Objekte". About **30% of submissions** triggered ≥1 new criterion, and about **1 in 3 of those** got a higher rating | Rating uplift plus descriptors | Same as the PEGI set (PEGI developed its criteria with USK) | `[VERIFIED https://www.klicksafe.de/en/sid23/veranstaltungen/die-deskriptoren-kommen-umsetzung-des-juschg-fuehrt-zu-neuerungen-bei-usk-kennzeichen (search); twobirds]` |
| 9 | **Austria** | Supreme Court (OGH) **6 Ob 228/24h**, **2025-12-18** | Final. FUT packs are **not gambling**: the game must be assessed as a whole, and skill affects outcomes ("mixed game"). Lower-court refund orders against EA and Sony were overturned. Other cases are pending | n/a | Low risk if random rewards are part of a skill-based game and non-cashable | `[VERIFIED https://www.dlapiper.com/insights/publications/2026/01/supreme-court-shifts-the-playing-field-lootboxes-fall-outside-the-austrian-gambling-act (search)]` |
| 10 | **ESRB** (US/Canada) | Interactive Element "**In-Game Purchases (Includes Random Items)**" | Since **2020-04-13** | Applies when real money (or currency bought with real money) buys digital goods whose content isn't known before purchase (loot boxes, gacha, packs, wheels, chests). **Advisory; it does not change the rating** | Use the correct descriptor. PEGI's parallel 2020 descriptor is "(Includes Paid Random Items)" | `[VERIFIED https://www.esrb.org/ratings-guide/in-game-purchases (search)]` |
| 11 | **Australia** | Guidelines for the Classification of Computer Games 2023 | **2024-09-22** for new classifications. Earlier titles keep their rating unless modified | **In-game purchases linked to elements of chance → min M** (advisory, not legally restricted). **Simulated gambling → min R18+** (legally restricted). Chance mechanics **with no way to spend real money are not covered**. Covers PC, console and mobile | No paid random items, or accept M. No casino or slot minigames (R18+). Earned-only chance is fine | `[VERIFIED https://www.pocketgamer.biz/australia-sets-new-rules-for-video-games-with-gambling-like-content (search); https://classification.gov.au/sites/default/files/documents/factsheet-new-mandatory-classifications-for-gambling-like-content-in-video-games.pdf (search)]` |
| 12 | **Brazil** | **Lei 15.211/2025 (ECA Digital)**, signed 2025-09-17. **Decreto 12.880/2026** (2026-03-18). **ANPD age-assurance guide** (2026-03-20) | **In force 2026-03-17**. ANPD is monitoring 37 companies. One secondary source gives a phased timeline: games August-November 2026, sanctions from January 2027 | **Art. 2 IV** defines a "caixa de recompensa" as a feature that lets the player acquire, **for payment**, items "without prior knowledge of its content or guarantee of its effective usefulness". **Art. 20:** loot boxes **prohibited** in games directed at children and adolescents **or likely accessed by them** (per the classificação indicativa). **Art. 21:** observe Lei 14.852/2024 art. 16 safeguards; limit interaction features by default. **Art. 35:** fines up to **10% of Brazilian group revenue**, **capped at R$50M per infraction** (R$10-1,000 per registered user if revenue is unknown). **Decreto 12.880:** a game may offer **a version without loot boxes, or restrict access to them by default**, and then **age verification is not required**. Self-declaration alone is never sufficient | Option A (preferred): **no paid random items in BR at all**. Option B: paid random items **off by default**, enabled only after **non-self-declared age assurance proving 18+**, plus a ClassInd rating of 18 (Riot raised League of Legends to 18 in Brazil and added age verification) | `[VERIFIED camara.leg.br law text (fetch); https://www.demarest.com.br/en/eca-digital-entrada-em-vigor-em-17-de-marco-de-2026/ (fetch); https://www.mattosfilho.com.br/unico/decreto-regulamentar-eca-digital/ (search); https://mais.opovo.com.br/jornal/reportagem/2026/03/05/eca-digital-o-que-muda-com-o-inicio-da-fiscalizacao.html (search); https://confidata.com.br/blog/eca-digital-o-que-muda-empresas (search)]` |
| 12b | **Brazil (courts)** | 1st Children and Youth Court of the Federal District (ANCED civil public actions) | Rulings **2026-06-09**, appealable | **R$298M** in collective moral damages over loot boxes, against Apple, Google, Microsoft, Sony, Tencent, Nintendo, Valve, Riot, Ubisoft, Konami and EA (Apple, Microsoft and Tencent R$50M each; Google and Sony R$40M; Nintendo R$5M). The court held that the 1990 ECA already barred abusive advertising to minors | Treat BR as **hostile to paid randomness even before the ECA Digital** | `[VERIFIED https://olhardigital.com.br/2026/06/17/games-e-consoles/justica-condena-empresas-de-games/ (search); https://canaltech.com.br/mercado/justica-do-brasil-multa-apple-google-e-gigantes-dos-games-em-r-298-milhoes/ (search)]` |
| 13 | **China (mainland)** | Ministry of Culture Notice, Culture Market Release **(2016) No. 32** | Effective **2017-05-01** | Publicly post the **name, properties, content, quantity and draw probability** of every drawable item (official site or a dedicated page). **Publish actual draw results** prominently. **Keep records ≥90 days** | Odds page plus published result logs (anonymized) plus 90-day retention. Also requires an ISBN licence and real-name verification `[RECALLED]` | `[VERIFIED https://www.chinalawtranslate.com/en/ministry-of-culture-notice-regarding-strengthening-regulation-online-gaming-operations/ (search); stevivor (search)]` |
| 13b | **China: minors** | 2019 NPPA notice; 2021 NPPA notice | In force | **Spend caps:** under 8, no payments. **8-16:** ≤ **RMB 50** per transaction and ≤ **RMB 200** per month. **16-18:** ≤ **RMB 100** per transaction and ≤ **RMB 400** per month. **Play time (2021):** minors only **20:00-21:00 on Fridays, weekends and public holidays**. The **2023-12-22 draft** (banning login/first-recharge rewards and probabilistic draws for minors, adding spend limits) was **removed from the NPPA site on 2024-01-23**. No evidence it was finalized | Real-name age gate, spend-cap enforcement, play-time enforcement (only if publishing in mainland China) | `[VERIFIED https://chinafilminsider.com/new-guidelines-restrict-play-time-spending-for-young-online-gamers/ (search); https://esportslegal.news/2024/01/23/chinas-regulatory-rollercoaster-in-the-gaming-industry/ (search)]` |
| 14 | **Japan** | Consumer Affairs Agency under the Act against Unjustifiable Premiums and Misleading Representations | **Kompu gacha** ("card-matching", カード合わせ: completing a set of random items to unlock a prize) **illegal since 2012-07-01**. JOGA's August 2012 business-model guideline: **no probability manipulation; "rare items within ¥50,000"** (headline). **JOGA Online Game Safety Declaration revised 2026-01-29, in effect 2026-02-04** | Members commit to: **spending caps for youth**; **minor check** at game start, at payment, or when a cap is hit, with **explicit parental consent**; **no card-matching gacha**; follow JOGA's **random-item display and operation guideline**; RMT ban; clear free/paid distinction | No set-completion rewards from paid random items. Odds display. Youth spending caps plus a parental-consent flow | `[VERIFIED https://japanonlinegame.org/wp-content/uploads/2026/02/JOGA_declaration_2026.pdf (fetched, text extracted); https://www.itmedia.co.jp/keywords/sga.html (search headline); https://www.serkantoto.com/?p=6295 (search)]` |
| 15 | **South Korea** | Game Industry Promotion Act (GIPA) amendment plus Enforcement Decree; MCST disclosure guideline (February 2024) | Disclosure mandatory from **2024-03-22**. **Art. 33-2** (in force **2025-08-01**): **treble damages** for intentional violations and a **reversed burden of proof**. Domestic-representative duty for overseas firms with ≥KRW 1 trillion global revenue or ≥1,000 average daily KR downloads; new designation notices around **October 2026** | Disclose probability-type item info for items obtained via **direct or indirect payment** (purely free items exempt), in the game, on the website and in ads. GRAC's common violations: odds posted only in-game, **tier-only odds**, **missing four-decimal precision**. **Dynamic odds:** if a fixed rule changes the odds (pity), **disclose the rule**; if odds vary per user, **show real-time odds**. Sanctions: corrective order, then ≤2 years prison or ≤KRW 20M. Representative failures: ≤KRW 20M per year. Pending bill: surcharge ≤3% of sales or KRW 1B. KFTC fined Webzen **KRW 158M** (rare-item odds were **0% until 50-149 pulls**). Overseas-firm violation rate **67% (2024) → 72% (2025)**; 105 violations in 64 games found by MCST/GRAC monitoring | **Per-item** odds at **4-decimal-place percent precision** (e.g., `0.6000%`), shown in-game, on the web and in ads. Full pity rules plus a live pity counter. **No hidden zero-probability windows.** Immutable result logs (needed to rebut the reversed burden of proof) | `[VERIFIED https://www.hwawoo.com/eng/insights/newsletter/2672 (fetch); https://www.hwawoo.com/eng/insights/newsletter/2547 (fetch); https://automaton-media.com/en/news/rare-pull-probability-was-zero-until-the-51th-draw-korean-developer-fined-by-antitrust-watchdog-over-deceptive-gacha-practices/ (search); https://www.legal500.com/firms/30196-yoon-yang-llc/c-south-korea/news-and-developments/punitive-damages-and-other-special-legal-provisions-for-litigation-concerning-probability-based-game-items (search)]` |
| 16 | **United States: FTC v. Cognosphere (HoYoverse)** | Proposed stipulated order, **2025-01-17**, C.D. Cal. | **$20M** civil penalty. Requires court entry; one outlet reports it finalized (unconfirmed) | **No loot box sales to under-16s without parental (affirmative express) consent.** **No selling loot boxes for virtual currency unless they can also be bought directly with real money.** **Disclose loot box odds and the exchange rates of multi-tier virtual currencies.** No misrepresenting odds, prices or features. COPPA compliance; delete under-13 data without consent | Direct-fiat purchase path for any random container. Exchange-rate display for every currency hop. Age gate at 16 with parental consent | `[VERIFIED https://www.ftc.gov/node/87282 (fetch); https://www.mychesco.com/a/news/national/ftc-finalizes-20-million-settlement-over-genshin-impact-loot-box-practices/ (search, unconfirmed finalization)]` |
| 16b | **US: COPPA rule (amended)** | 16 CFR 312 | Effective **2025-06-23**; **compliance 2026-04-22** | Expanded personal-information definition (biometrics) and stricter consent | Applies to under-13 users or actual knowledge. Age gate plus verifiable parental consent before collecting PI | `[VERIFIED https://www.federalregister.gov/documents/2025/04/22/2025-05904/childrens-online-privacy-protection-rule (search)]` |
| 16c | **US: console platforms** | ESA announcement, 2019-08-07 | Sony, Microsoft and Nintendo committed to require **odds disclosure for paid loot boxes** in new games and updates (target 2020) | n/a | Odds disclosure on console SKUs | `[VERIFIED https://www.engadget.com/2019/08/07/sony-microsoft-nintendo-loot-box-odds/ (search)]` |
| 17 | **Apple App Store** | Review Guideline **3.1.1** | Current | "Apps offering 'loot boxes' or other mechanisms that provide randomized virtual items for purchase must **disclose the odds of receiving each type of item** to customers **prior to purchase**." "Any credits or in-game currencies purchased via in-app purchase **may not expire**." 5.3.3: no IAP for real-money gaming credit | Per-item odds before purchase. Purchased currency never expires | `[VERIFIED https://developer.apple.com/app-store/review/guidelines/ (fetch)]` |
| 18 | **Google Play** | Payments / Monetization policy | Current | Odds for randomized virtual items from a purchase must be disclosed "**in advance of, and in close and timely proximity to, that purchase**". In-app virtual currencies usable only in the title they were bought for | Odds shown on the purchase screen itself | `[VERIFIED https://support.google.com/googleplay/android-developer/answer/9858738 (fetch)]` |

## C2. Jurisdiction notes and edge details

- **"Paid" is the trigger everywhere.** Brazil's definition requires "mediante pagamento". Korea exempts purely free items. Australia exempts chance mechanics with no way to spend real money. PEGI and ESRB both say "paid" or "real money (or currency bought with real money)" `[VERIFIED rows 10-12, 15]`. **Corollary:** if an earned currency can be *topped up* with real money, or premium currency converts into it, random containers bought with it count as **paid**. The earned currency must be **hermetically separated** from anything purchasable.
- **Indirect payment (Korea):** items obtained through a paid currency hop count as paid `[VERIFIED hwawoo 2547 summary: "direct or indirect payment" via search]`. **Never** let premium currency convert into gacha currency.
- **Dynamic odds disclosure (Korea/GRAC):** a pity ramp is a "fixed rule", so publishing the hazard function from B2 (b, S, k, H) satisfies it. **Personalised odds** (e.g., per-player DDA on drops) would require **real-time odds display**, so PALIMPSEST should never personalise paid odds `[VERIFIED hwawoo 2672]`.
- **Webzen precedent:** hidden minimum-pull windows with zero probability are deceptive. Every pull must have p(n) ≥ b > 0 `[VERIFIED automaton]`.
- **Japanese kompu gacha:** rewarding a full *set* of randomly obtained paid items is illegal. Earned-only sets are not "premiums tied to paid transactions" `[VERIFIED headline/ruling]`, though the earned/paid separation must be watertight for that reasoning to hold `[RECALLED reasoning]`.
- **PEGI time-limited offers:** "paid battle passes with countdown timers" are the PEGI example of a quantity- or time-limited offer `[VERIFIED twobirds]`. A **non-expiring** pass with **no countdown** should fall outside that trigger. That is a reading of the criterion, not confirmed PEGI guidance, so it is listed in Part E.
- **Content may dominate the rating anyway.** A horror- or gore-heavy omni-genre game may land at PEGI 16/18 on content. The paid-random floor still matters for Brazil (ClassInd plus Art. 20), Australia (separate classification), Belgium and the coming DFA, so the conclusion is unchanged `[RECALLED reasoning]`.
- **Stop Destroying Videogames / end-of-life:** no legal duty, but a voluntary EU code is being negotiated (talks from end-2026) and the DFA debate may absorb it `[VERIFIED]`. Shipping offline and LAN modes now avoids future retrofits.

## C3. Ratings trigger matrix (feature → consequence)

| Feature | PEGI (June 2026+) | USK | ESRB | Australia | Brazil (ECA Digital) | Korea | Belgium |
|---|---|---|---|---|---|---|---|
| Paid random items (loot box, gacha, packs, wheel) | **min 16** (some 18) | possible uplift plus descriptor | "In-Game Purchases (Includes Random Items)" (advisory) | **min M** | **Banned** if minors are likely users (unless a no-loot-box version or default-off plus age verification) | Mandatory 4-dp odds disclosure | **Illegal** |
| Earned-only random items | none | none | none | none | none (not "mediante pagamento") | none (free items exempt) | none |
| Paid offer with countdown or limited quantity | **min 12** (→7 if spending is off by default) | possible | "In-Game Purchases" | none | n/a | n/a | n/a |
| Non-expiring paid pass, no timer | likely no time-limit trigger (reading, unconfirmed) | n/a | "In-Game Purchases" | none | n/a | n/a | n/a |
| Daily quests / streak rewards | **min 7** | possible | none | none | n/a | n/a | n/a |
| Punishing absence (losing streak content) | **min 12** | possible | none | none | n/a | n/a | n/a |
| Chat with no block or report | **18** | possible | "Users Interact" | n/a | Art. 21: interaction limited by default | n/a | n/a |
| NFT/blockchain required and tradeable | **18** | n/a | n/a | n/a | n/a | n/a | n/a |
| Casino/slot minigame (simulated gambling) | 18 (case by case) | uplift | "Simulated Gambling" `[RECALLED]` | **R18+** (legally restricted) | n/a | n/a | n/a |

Sources: rows of C1 `[VERIFIED]`, except where marked.

## C4. Unified technical compliance spec (strictest common denominator)

These are requirement IDs the implementation agent should carry into the spec. "PRI" = paid random item. Values are `[PROPOSED]` unless they cite a legal source.

| ID | Requirement | Derived from |
|---|---|---|
| **MON-0** | Build flag `paid_random_items_enabled` (bool). **Default false in every region.** Server-side per-region override is allowed only if every ODDS/AGE/PRICE requirement passes automated checks | Synthesis |
| **SEP-1** | Earned currencies (gacha and Reliquary currencies) **cannot be bought, cannot be converted from any purchasable currency, and cannot be gifted for money**. Enforced by type: the `CurrencyId` enum has a `purchasable: bool`, and the ledger rejects any transfer from `purchasable=true` into `purchasable=false` | Brazil Art. 2 IV; KR "indirect payment"; AU; PEGI |
| **ODDS-1** | For every random container (paid **or** earned; voluntary for earned), publish the **per-item** probability as a percent with **exactly 4 decimal places** (`0.6000%`), plus per-tier aggregates, base and consolidated rates | KR GRAC; Apple "each type"; CN MoC; FTC |
| **ODDS-2** | Show odds **on the same screen as the confirm button** (Google "close and timely proximity"), on an in-game Details page, and at a public URL (`/odds/<banner_id>`) | Google; CN; KR |
| **ODDS-3** | Publish the **pity rules as a formula** (b, S, k, H, featured rule, spark), and show the player's **live counters** (pulls since last top tier; guarantee state; spark progress) | KR dynamic-odds rule; industry norm |
| **ODDS-4** | **p(n) ≥ b > 0 for every n**. No zero-probability windows. No per-player personalized odds | KFTC Webzen; KR |
| **LOG-1** | Server-authoritative RNG. An immutable log of every draw (account, banner, pull index, RNG seed/nonce, result, timestamp), retained **≥5 years** (exceeds the CN 90-day minimum; supports the KR reversed burden of proof) | CN; KR Art. 33-2 |
| **LOG-2** | Publish aggregate actual-results statistics per banner (anonymized), refreshed daily | CN 2.7 |
| **PRICE-1** | Every price shown in virtual currency also shows the **fiat equivalent** in the user's currency, computed via the **most expensive** conversion path | CPC principles 1-2; FTC exchange-rate disclosure |
| **PRICE-2** | No forced overspend. Either sell items **directly in fiat** (preferred), or offer currency SKUs that exactly cover each item price | CPC principle 3 |
| **PRICE-3** | Any container sold for virtual currency must also be **purchasable directly with real money** | FTC Cognosphere order |
| **PRICE-4** | Purchased currencies **never expire** | Apple 3.1.1 |
| **PRICE-5** | All-in price shown up front; no fees added at checkout | UK DMCC |
| **AGE-1** | Age assurance that is **not self-declaration alone** before any real-money purchase where PRI is enabled; under-16 needs **parental affirmative consent** (US); under-18 needs **parental consent** (UK); BR needs **18+ verified** | FTC; UK principles; BR Decreto 12.880 |
| **AGE-2** | **Purchases disabled by default** on new accounts. A parent or account holder enables them in settings. Default child spend limit = 0 | PEGI 7 mitigation; UK "£0 default"; BR default restriction |
| **AGE-3** | Region spend caps for minors: CN (8-16: ¥50/tx, ¥200/month; 16-18: ¥100/tx, ¥400/month); JP JOGA youth caps (publisher-set) | CN 2019; JOGA 2026 |
| **REG-1** | Region hard-blocks for PRI: **BE** always. **BR** unless AGE-1 (18+ verified) **and** ClassInd 18. **NL** requires no trading of random items | C1 rows 1, 2, 12 |
| **TIME-1** | **No countdown timers or quantity limits on paid offers.** Seasonal cosmetics rotate back (no "never again") | PEGI 12 trigger; CPC pressure |
| **DAILY-1** | Daily or weekly rewards are allowed; **missing days never removes content or progress** (streaks pause, never reset to zero) | PEGI 7 vs 12 |
| **COMMS-1** | Every text, voice and ping channel has **mute, block and report**. Default for under-18: friends-only voice | PEGI 18 trigger; BR Art. 21 |
| **SET-1** | No reward for completing a set of items obtained from paid random containers | JP kompu gacha |
| **NFT-0** | No blockchain or NFT items | PEGI 18 |
| **REFUND-1** | Self-serve refund of unconsumed purchases within 14 days. Explicit withdrawal-waiver consent at the moment of first use. Lenient refunds for unauthorized child spending | CPC principle 3; UK principles |
| **AD-1** | Ads and store art for any product with PRI show "Contains paid random items" plus an odds link | KR (ads); PEGI/ESRB descriptors |
| **KR-REP-1** | If KR thresholds are met (≥KRW 1T global revenue or ≥1,000 average daily KR downloads), designate a domestic representative | KR GIPA |
| **EOL-1** | Offline solo mode and LAN/direct-IP co-op ship with 1.0. A documented end-of-life plan (local saves exportable; cosmetics usable offline) | ECI / Commission code-of-conduct process |

---

# Part D: Design implications for PALIMPSEST

## D1. Monetization architecture: "Earned Randomness, Paid Certainty"

**Decision: PALIMPSEST ships gacha, loot boxes and a battle pass as first-class systems. Every randomized container is funded only by earned, non-purchasable currency. Real money buys only deterministic goods.** Rationale:

1. The paid trigger appears in every regime (C2). Earned-only randomness avoids the PEGI 16 floor, the Australian M floor, the Brazilian Art. 20 ban, the Belgian gambling law, the Korean mandatory disclosure regime (disclosure is still done voluntarily), and the likely DFA minors ban. That is **one SKU worldwide** with no age-verification vendor at launch.
2. Players get the *fun* of gacha and loot boxes (anticipation, pity tension, collection, sparks) without the trust cost. Overwatch moved to exactly this in 2025 `[VERIFIED]`.
3. A co-op PvE roguelike earns through a **premium price plus genre expansions plus cosmetics plus a fair pass**, which is the ARC Raiders/Hades II/StS2 model and the proven 2025-2026 one `[VERIFIED A1]`.

**Revenue lines `[PROPOSED]`:**

| Line | Price | Notes |
|---|---|---|
| Base game | **$24.99** at Early Access launch, rising to **$29.99** at 1.0 | Matches StS2 ($24.99), Hades II ($29.99) and Hytale ($19.99) `[VERIFIED]` |
| Folio Expansion (a new genre: floors, enemies, items, spells, biome rules) | **$9.99-14.99** each, every 9-12 months | Matches RoR2's $15 DLC cadence `[VERIFIED]` |
| Codex (battle pass) premium track | **$9.99 fiat**, non-expiring, 100% payback via a Quill entitlement | D4 |
| Direct cosmetics (character skins, weapon "inks", spell VFX palettes, emotes, banners) | Fiat, **$1.99-9.99**, permanent, no timers, no currency | Satisfies CPC principles 1-3 by design (no virtual currency) |
| Supporter pack | $4.99 | Cosmetic plus soundtrack |

**There is no premium currency.** Every purchase is a fiat SKU. That removes the CPC "most expensive conversion" display problem, the FTC exchange-rate disclosure, Apple's currency-expiry rule and Korea's "indirect payment" exposure in one decision.

**`paid_random_items_enabled` stays `false` (MON-0).** If a future business decision flips it, it must pass all of C4 and stay hard-blocked in BE, and in BR unless 18+ age assurance is in place.

## D2. Exact gacha spec: the **Scriptorium** `[PROPOSED; numbers COMPUTED]`

**Fiction:** the Palimpsest's scriptorium "re-inks" erased texts. Pulls recover **Scripts**: meta-unlocks that **add options to the run pool** (archetype starting kits, spell glyphs, Folio variants, companions, cosmetic Bindings). Scripts **never add flat stats** carried into runs. They widen choice, which keeps roguelike integrity.

| Parameter | Value |
|---|---|
| Currency | **Ink** (`CurrencyId.INK`, `purchasable=false`, not tradeable, not giftable) |
| Cost | **160 Ink per pull**; 10-pull = **1,600 Ink** (no discount; the 10-pull only bundles animations) |
| Tiers | **Mythic** / **Rare** / **Common** |
| Mythic hazard | b = **0.0100**, S = **46**, k = **0.065**, H = **60** → `p(n)=0.01` for n≤45; `p(n)=0.01+0.065·(n−45)` for 46≤n≤59; `p(60)=1` |
| Mythic outcomes | E = **39.230 pulls**; consolidated **2.549%**; median **48**; 90th pct **52**; 99th pct **55**; 36.38% land before soft pity `[COMPUTED]` |
| Rare hazard | b = **0.0600**, S = **9**, k = **0.60**, H = **10** → E = **7.324 pulls**; consolidated **13.654%** `[COMPUTED]` |
| Featured rule | **75/25**. A loss sets `guarantee=true`, so the next Mythic is featured |
| Featured outcomes | E = **49.04 pulls**; median 49; 90th pct **94**; without spark the worst case is 120 (2×60) `[COMPUTED]` |
| Spark | **100 pulls on one banner** → pick the featured Mythic (worst case for featured = **100**; P(>100 without spark) was 3.03%) `[COMPUTED]` |
| Carryover | Mythic pity count and guarantee flag **carry across all Scriptorium banners and never expire**. Spark count is per-banner. At banner end, unused spark converts **1:1 into Marginalia** |
| Archive | Permanent shop: **100 Marginalia → any previously featured Mythic**; 20 Marginalia → any Rare |
| Duplicates | Mythic dupe → **40 Gloss**; Rare dupe → **8 Gloss**. Gloss buys cosmetic tiers of owned Scripts, or specific Rares at 60 Gloss |
| Ink income baseline | **400 Ink per hour of active play** (≈2.5 pulls/h) → a featured Mythic every ≈**19.6 h** of play `[COMPUTED from proposal]`. The economy doc may retune this, but must keep income ≥ 1 featured per 25 h |
| Disclosure | ODDS-1..3 even though Ink is earned: 4-dp percentages, hazard formula, live counters |

**Reference implementation (pseudocode the implementation agent can copy):**
```
fn mythic_p(n: u32, b: f64, s: u32, k: f64, h: u32) -> f64 {
    if n >= h { 1.0 } else if n >= s { (b + k * ((n - s + 1) as f64)).min(1.0) } else { b }
}
// state: pulls_since_mythic (u32, starts 0), pulls_since_rare (u32), guarantee (bool), spark[banner] (u32)
// on pull: pulls_since_mythic += 1; pulls_since_rare += 1; spark[banner] += 1
// roll r ~ U[0,1) from server CSPRNG; if r < mythic_p(pulls_since_mythic,...) => Mythic:
//    featured = guarantee || roll2 < 0.75; guarantee = !featured; pulls_since_mythic = 0; pulls_since_rare = 0
// else if r2 < rare_p(pulls_since_rare, 0.06, 9, 0.60, 10) => Rare (pulls_since_rare = 0) else Common
// when spark[banner] >= 100 the UI offers "Claim featured" (consumes 100 spark)
```

## D3. Exact loot box spec: the **Reliquary**, with the appraisal tie-in `[PROPOSED]`

- **Sources (never sold):** Folio-boss kills (1 per boss per player), Folio completion (1), Codex tiers (D4), weekly Daily-Folio participation (1). Co-op: **every player gets their own Reliquary** (no loot competition), following the Barony/Borderlands-style instanced-loot expectation `[RECALLED]`.
- **Contents:** 3 rolls from the Folio's loot table, rarity-weighted. **All contents arrive Sealed (unidentified).** Opening shows silhouettes plus the genre tag. True identity needs appraisal (scroll, skill check, or NPC fee). This **merges the loot box thrill with the IVAN/SPD identification game** instead of bolting on a separate system.
- **Per-account bad-luck counters** (Overwatch structure `[VERIFIED B4]`): every Reliquary contains ≥1 **Uncommon+**; every **5th** contains ≥1 **Epic+**; every **20th** contains ≥1 **Legendary+**. Counters are shown on the Reliquary tooltip.
- **Disclosure:** per-item 4-dp odds per Folio table at `/odds/reliquary/<folio_id>` (ODDS-1), even though it is earned.
- **No set-completion rewards** tied to Reliquary-only items (SET-1 hygiene, even though earned).

## D4. Exact battle pass spec: the **Codex** `[PROPOSED]`

| Parameter | Value | Justification |
|---|---|---|
| Price | **$9.99 fiat**, direct entitlement (no currency) | Market band $8-10 `[VERIFIED B5]`; no-currency rule (D1) |
| Tiers | **100** | CoD (100+ rewards), Fortnite (~100 base) `[VERIFIED/RECALLED]` |
| XP per tier | **Flat 10,000 XP** → 1,000,000 XP total | Flat curves are the industry norm `[RECALLED B5]`; predictable |
| XP sources | **All play** (floor clears, kills, discoveries, appraisals, crafting, co-op revives). **Not challenge-only** | Halo Infinite's challenge-only XP was criticized and eased `[VERIFIED]` |
| Target completion | **≈60 h** of play → baseline **16,667 XP/h**; first 3 runs each week earn **×2 XP** (catch-up for casual players) | Design target |
| Free track | Rewards on **every even tier** (50 rewards) incl. Ink, Reliquaries and cosmetics | Generous free track at launch (ARC Raiders lesson) |
| Premium track | A reward on **every tier** (100), cosmetics plus Ink plus Reliquaries; **tier 100 = 1 Quill** (an entitlement that unlocks any Codex premium track) | **100% payback**, like Fortnite 800/800 and CoD 1,100/1,100 `[VERIFIED]`, without currency |
| Expiry | **Never.** One active Codex at a time, switchable; past Codexes purchasable forever | Halo Infinite, Helldivers 2, Marvel Rivals, ARC Raiders Legacy `[VERIFIED]` |
| Timers | **None.** No countdown, no "last chance" UI | Avoids the PEGI 12 time-limited trigger (reading) and CPC pressure |
| Tier skips | **Not sold** | Avoids pay-to-skip pressure; protects 100% payback integrity |
| Cadence | A new Codex every **12 weeks**, themed to the latest Folio genre | Aligns with live cadence (A2) |
| Gating | Purchases disabled by default (AGE-2); enabling needs account-holder action | PEGI 7 mitigation; UK £0 default |

## D5. Feature-bar commitments (from Part A)

1. **Co-op 1-4, drop-in/drop-out, host migration.** Tune solo, duo, trio and quad at launch (Nightreign lesson). Co-op verbs must create interdependence: route voting with map annotations, thrown potions that affect allies fully, turn-ordered Reliquary drafts, shared crafting stations, revive and carry (StS2/PEAK lesson).
2. **Crossplay on by default with a toggle; account-based cross-save.**
3. **Steam Deck Verified plus Steam Machine 10-foot UI** at Early Access; full controller parity; 60 fps target on Deck at 800p with a dynamic voxel LOD budget.
4. **Accessibility suite, filled into Steam's questionnaire:** "Scribe's Mercy" assist mode copying **Hades God Mode exactly: 20% damage resistance at enable, +2% per death, cap 80%, toggle any time** `[VERIFIED]`; game-speed slider 50-100%; FOV 60-120; head-bob and screen-shake off; subtitles with speaker names and sound captions; colorblind palettes; hold/toggle alternatives; full remapping; UI scale 75-200%.
5. **Seeds and dailies:** shareable seed strings; **Daily Folio** (global seed, one ranked attempt per day, unlimited practice); weekly mutator Folio.
6. **Run length:** standard run **45-75 min**, at about **6-10 min per floor**. This matches Nightreign (40-60) and the 2025 co-op norm `[VERIFIED]`. The long-form "Full Palimpsest" mode is 3-5 h with mid-run save (SPD/Qud audience).
7. **Modding:** data-driven content (all items, spells, genres and loot tables in versioned JSON with a published schema), a sandboxed scripting layer, and Workshop support at 1.0 (StS2/Godot and Hytale lesson).
8. **Live cadence:** a free systemic update every **6-8 weeks** (SPD-style: new items, enchantments and quests rather than raw volume); a paid Folio expansion every 9-12 months; a Codex every 12 weeks.
9. **Performance budget gate:** no Early Access ship until the minimum-spec PC holds a stable frame time (Borderlands 4 lesson).
10. **End of life:** offline solo plus LAN/direct-IP co-op at 1.0. Cosmetics and unlocks are cached locally and signed, so they work offline (EOL-1).

## D6. Avoid list (hard "no")

- **No premium currency** of any kind, and therefore no odd-sized currency bundles (CPC principle 3).
- **No paid random items** (MON-0 stays false). No converting anything purchasable into Ink (SEP-1).
- **No expiring passes, countdown timers, "limited stock" or FOMO shop rotations** (PEGI 12; CPC).
- **No punishing absence:** daily streaks pause and never reset content (PEGI 7 vs 12).
- **No zero-probability windows or personalised odds** (Webzen precedent; KR).
- **No NFT, blockchain or cash-out trading** (PEGI 18; NL).
- **No casino or slot minigames**, even as fiction inside a "noir" or "weird west" Folio (AU R18+; PEGI 18). Card or dice games of skill are fine if no currency wagering with randomized payout exists. Avoid roulette-style wager loops.
- **No chat without mute, block and report** (PEGI 18).
- **No challenge-only pass XP** (Halo Infinite).
- **No thin launch:** at least 6 distinct genre Folios at Early Access (Marathon's 4-map criticism) `[PROPOSED threshold]`.

---

# Part E: Open questions and unverified items

1. **PEGI official text:** pegi.info was not reachable this session (404). Unconfirmed: the exact PEGI 18 conditions for paid random items; whether existing titles are re-rated from **2026-10-01** (one secondary source); and whether a **non-expiring pass without countdowns** is outside the "time-limited offers" trigger (my reading, not PEGI's).
2. **EU DFA:** text not yet published. Expected November 2026 (tentatively 2026-11-18). Re-check after publication; it may ban paid loot boxes and in-app currencies for minors.
3. **FTC v. Cognosphere:** final court entry is reported by one outlet and not confirmed on ftc.gov.
4. **Brazil:** the exact article of Decreto 12.880/2026 providing the "no-loot-box version / default-restricted" exemption, and the phased enforcement timeline (games August-November 2026, sanctions from January 2027), come from secondary sources only.
5. **Korea:** the exact MCST guideline wording on decimal precision ("four-decimal precision" per GRAC via Hwawoo) and any small-business exemption (I recall a revenue threshold for small companies `[RECALLED]`, not verified).
6. **Netherlands:** no 2025-2026 national bill was found; the government position is to legislate via the DFA.
7. **China:** whether any 2025-2026 NPPA rule replaced the removed 2023 draft; any daily draw limits.
8. **Japan:** primary text of the current JOGA "random item display and operation guideline" (the 2026 declaration references it; contents not fetched), and the exact ¥50,000 rule wording.
9. **Gacha community parameters:** soft-pity starts for HSR, ZZZ and WuWa, and the ZZZ W-Engine 1.0%/2.0% figures, are single-source or community-derived. The Genshin model is validated against official consolidated rates (B2).
10. **Battle-pass XP per tier** for Fortnite, CoD and Apex in 2026 was not found in primary sources.
11. **Capturing Radiance base trigger "0.018%":** the unit and meaning are unclear on the source page; the 55% consolidated share is the decision-relevant number.
12. **Marvel Rivals 600 Lattice + 600 Units payback** is confirmed for one season only.
13. **European Accessibility Act applicability to in-game stores** depends on national transposition; unresolved.
