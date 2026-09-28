# 2. YouTube Niche Domination

## Source prompt

> "Act as an elite YouTube market researcher and channel strategist. Help me identify the best niche and positioning strategy for building a monetizable YouTube channel. Analyze audience demand, competition, search potential, monetization opportunities, content longevity, and differentiation angles. Create my unique channel identity, target viewer profile, content categories, and the first 100 video ideas ranked by growth potential.

(Verbatim from `prompts/SOURCE_PROMPTS.md`, prompt 2; no closing quote in the source.)

## Assumptions used

- **A1** no channel exists; plan starts from zero.
- **A2** faceless-first: AI Oracle voice + card art + app capture; founder segments optional later.
- **A3** goal order: YPP eligibility → funnel to chaos-tarot.com → deck/Kickstarter awareness.
- **A4** near-zero cash; existing AI keys and self-hosted tooling.
- **A5** ~8–12 h/week.
- **A8** physical deck = 79 cards; the 10 Singularities are Codex lore / digital-only unless Apocky says printed.
- Not relied on: A6, A7.

## Rail status (this session)

| Rail | Result | Tag |
|------|--------|-----|
| Product repo `/home/user/the-chaos-tarot` | Read: `New/` art filenames (79 cards + Card Back + "Page of Fractures"), `deck/manifest.json` (22 Majors, 4 suits × 14 = Codes / Networks / Signals / Vectors), `shared/reading-templates.json` (14 templates in 6 categories), Codex §1.2, 1.3, 3.0, 5.1, 5.2, 6.0, 7.0, `docs/CHAOS_ORACLE_VIBE.md`, `docs/AI_ORACLE_TRAINING_GUIDELINES.md`, `docs/FEATURE_LIST.md` (dated 2026-01-19, v3.0), `Esoteric App Development Plan.md` §6.2 | ✓ |
| Live site chaos-tarot.com (fetched 2026-09-27) | Eight systems enumerated: Chaos Tarot · Elder Futhark Runes · "I Ching: Digital Mutations" · "Ogham: Tree Oracle" · Petit Lenormand · "Geomancy: Sand Code" · "Astrology: Stellar Blueprint" · **Cross-System**. Personas: the Oracle of the Glitch (tarot), the Runekeeper, the Digital Oracle of Changes, the Digital Druid, the Salon Oracle, the Geomantic Architect, the Astrologer. Pricing Free $0 · Oracle $3.33/mo · CSL $2.22/mo. Disclaimer: "for entertainment and personal reflection only." | ✓ |
| Drive "TikTok Hashtag Trends: Spirituality & Holidays" (2026-01-19) | Read in full; Trend Matrix Q1–Q4 quoted below | ✓ |
| Unirecall | **Unreachable.** `ListAgents` shows no linked desktop or peer session; no `remote-devices` tools are loaded in this container; no local `unirecall` binary or 127.0.0.1:19129 service. Prior channel decisions, if any, are not in this section. Run locally and paste the JSON: `python -B C:\Users\Apocky\source\repos\anamnesis\unirecall.py "chaos tarot youtube channel positioning" --tiers l2,l34,vault --timeout 8 -n 6 --json` | ✓ (degraded) |
| `/home/user/chaos-tarot` | Empty directory; nothing read | ✓ |

Corrections to the brief from this pass (✓): (a) the "eighth system" is **Cross-System**, not sigils; (b) `reading-templates.json` holds **14 templates** (Daily Guidance · Past, Present, Future · Situation · Action · Outcome · Career Crossroads · Relationship Clarity · Love Potential · Shadow Work Deep Dive (10 cards) · Yes or No Clarity · Two Paths · Week Ahead · Monthly Cycle · Mind · Body · Spirit · Creative Unblocking · Free Draw) in 6 categories (general, career, relationships, self-discovery, decision, timing); the brief's "20 named" counted category names as templates. (c) `FEATURE_LIST.md` still says "78-Card Deck" and `Esoteric App Development Plan.md` §6.2 lists Neophyte/Adept $9.99/Magus $19.99 tiers; both are stale against the live site (79 cards, $3.33).

---

## 1. Niche analysis

### 1.1 Audience demand

| Signal | Evidence | Tag |
|--------|----------|-----|
| Tarot on YouTube is a large, saturated reading niche | Feedspot's tarot list (updated 2026-09-26) shows 25 channels between 213K and 1M subscribers, almost all "pick-a-card"/zodiac readings: Minnow Pond Tarot 967K, ALI's Tarot 1M, 303 High Priestess 903K, EAT READ LOVE 911K — https://videos.feedspot.com/tarot_youtube_channels/ | ○ |
| Occult-history explainer demand is proven at scale | Esoterica (Dr Justin Sledge) reached 1M subscribers by end of 2025 — https://youtube.fandom.com/wiki/Esoterica | ○ |
| Witchcraft lifestyle demand exists but is person-led | Harmony Nice 723K, Kelly-Ann Maddox ("chaos witch") 67.6K, Black Witch Coven 90.9K — Feedspot witchcraft list, updated 2026-07-02, https://videos.feedspot.com/witchcraft_youtube_channels/ | ○ |
| Cyberpunk + tarot has a gaming on-ramp | Cyberpunk 2077 has 22 in-game tarot cards; lore videos exist (e.g. WiseFish, "The Mystery Behind Cyberpunk's Tarot", https://www.youtube.com/watch?v=3E9EbaFcY0Y); an official Dark Horse/CDPR tarot deck is sold at https://gear.cdprojektred.com/products/cyberpunk-2077-tarot-deck-guidebook | ○ |
| The 2026 spiritual audience is bifurcating | Drive doc: "Do not try to curate a perfect life. Either show the messy spreadsheet of your reality, or show us a crocodile fighting a goose. Anything in between is invisible." WitchTok 2026 = "the disciplined mystic"; #TarotChallenge 7/31-day; swing to darker, historical art (Norwegian Woodcut Tarot) "as a reaction to polished AI art" | ✓ (doc) / ○ (its sources) |
| Keyword search volumes for "tarot card meanings", "I Ching", "runes meaning" | Not verifiable this session without a keyword tool (vidIQ connector is `connect_incomplete`). **Unverified**; kept out of the critical path. | ⊘ |

### 1.2 Competition and differentiation angle per competitor

| # | Channel (subs, source) | What they do | Chaos Tarot angle against them | Tag |
|---|------------------------|--------------|-------------------------------|-----|
| 1 | Minnow Pond Tarot (967K, Feedspot 2026-09-26) | Free intuitive pick-a-card/zodiac readings | Never compete on prediction. The deck and the eight systems are the star; readings are framed as "entertainment and personal reflection only" (site ✓). Ad-safety edge: no "what will happen to you" claims. | ○ |
| 2 | Red Fairy Tarot (320K, Feedspot) | "Occult-focused readings; also shares spells and witchcraft tips" | Technomancy, not folk witchcraft: "The computer is the altar; the code is the incantation" (guidelines ✓). No spells, no hexes; the ritual is journaling + streaks. | ○ |
| 3 | Nicholas Ashbaugh (389K, Feedspot) | "Blends esoteric tarot knowledge with intuitive empathic gifts" | Reader-led vs artifact-led. Our reader is a machine with a "Dual-Boot Philosophy" (sysadmin + high priestess); the human on screen is the builder, not the psychic. | ○ |
| 4 | Harmony Nice (723K, Feedspot witchcraft) | On-camera witchcraft lifestyle | Faceless-first (A2), product-native. Where she sells lifestyle, we sell a 79-card deck + software with a $3.33 tier. | ○ |
| 5 | Kelly-Ann Maddox (67.6K, Feedspot witchcraft) | "Award-winning tarot reader and chaos witch"; spiritual counsellor | Closest philosophical neighbour (chaos magick). She is therapeutic and person-led; we are humorous (MAXIMUM CHAOS), multi-system, and never counselling. Her size (67.6K) shows chaos-magick-as-identity alone does not scale; the cyberpunk art and app are the multiplier. | ○ |
| 6 | Esoterica / Justin Sledge (1M by end 2025, Wikitubia) | Academic history of magic | Do not compete on scholarship; borrow the explainer format for Ogham, Geomancy and I Ching with a cyberpunk skin, and cite him. Target the "other channels your audience watches" overlap, not his subscribers. | ○ |
| 7 | Arith Härger (rank #433,774; 35.07K views/30 days, vidIQ via search) | Norse mythology and runes lectures | Our runes are a 3D voxel "slot machine" casting engine (`FEATURE_LIST.md` ✓, live status ⊘). Gamified casting vs lecture. | ○ |
| 8 | WiseFish (Cyberpunk 2077 tarot lore, oEmbed author ✓) | Game-lore explainers for a gaming audience | Bridge content: "Cyberpunk 2077's 22 game cards vs a full 79-card cyberpunk deck you can actually read with." Their audience is our Netrunner persona. | ✓ author / ○ audience |
| 9 | Product competitors (not channels): Cyberpunk 2077 Tarot Deck (Dark Horse/CDPR), Teknebrae Tarot and Neon Moon Lenormand (http://pixeloccult.com/), Norwegian Woodcut Tarot (trending per Drive doc) | Physical cyberpunk / dark decks | Only Chaos Tarot ships deck + 8-system app + AI oracle + Memory Core. Comparison videos convert deck buyers. | ○ |

Not verifiable this session: Ethony and Angela's Symposium subscriber counts (Social Blade 403, Playboard 453, NoxInfluencer paywalled). Both are named as differentiation references only, not in the critical path.

### 1.3 Search potential

| Unit | Count | Why it matters | Tag |
|------|-------|----------------|-----|
| Shipped tarot cards | 79 (22 Majors + 56 Minors + Null) | Each card title is a unique string with zero competition ("Tower of Obsolescence", "The Devil's Algorithm", "Null — The Apockalypse"); each also ranks for the generic card ("Tower tarot meaning") as a long-tail variant | ✓ count / ◐ ranking |
| Runes / hexagrams / Ogham / Lenormand / Geomancy figures | 24 / 64 / 25 / 36 / 16 | 165 evergreen explainer targets in systems with far fewer YouTube competitors than tarot | ✓ counts (site) / ◐ competition |
| Reading templates | 14 | "Career crossroads spread", "two paths spread" are search phrases with app-native demos | ✓ |
| Total evergreen library | ~258 units | At 3–4/week, a 90-day run covers only ~40; the library outlasts the plan horizon (longevity) | ◐ |

### 1.4 Monetization opportunities (niche-level; prompt 6 owns the full stack)

| Route | Verified gate | Fit | Tag |
|-------|---------------|-----|-----|
| YPP ad revenue | 1,000 subs + 4,000 qualified public watch hours in 12 months, **or** 1,000 subs + 10M qualified Shorts views in 90 days; review "typically in about 1 month" — https://support.google.com/youtube/answer/72851 | Watch-hours path via long-form card decodes and system explainers; Shorts path is unrealistic for a new niche channel | ○ |
| YPP fan-funding tier | 500 subs + 3 public uploads in 90 days + 3,000 watch hours/12 mo or 3M Shorts views/90 d — https://air.io/en/monetization/youtube-partner-program-requirements-2026-the-complete-guide (updated 18 Sep 2026); vidIQ guide agrees — https://vidiq.com/blog/post/youtube-partner-program-guide/ | Memberships/Super Thanks arrive before ads | ○ |
| 2027 change | air.io reports that from 1 Feb 2027 new applicants need 8,000 watch hours or 20M Shorts views; the official page fetched this session confirms "program updates" effective 1 Feb 2027 requiring acceptance of revised terms by 31 Jan 2027 but the fetch did not show the doubled numbers. **Treat the doubling as ○ reported, not confirmed.** Either way: apply before 31 Jan 2027. | Sets the real deadline behind the "90 days" horizon | ○ |
| Product funnel | Free tier → Oracle $3.33/mo (site ✓); physical deck via Stripe, US shipping (site ✓); Kickstarter budget sheet dated 2026-09-07 (brief ✓) | Primary economic value of the channel; independent of YPP | ✓ |

### 1.5 Policy gates that shape the niche (verified 2026-09-27)

| Gate | What the source says | Consequence for this channel | Tag |
|------|----------------------|------------------------------|-----|
| Inauthentic content (renamed from "repetitious", July 2025) | Ineligible: "AI-generated content made with generic or unoriginal templates giving the impression of mass production" without "original, authentic insights or perspective"; "image slideshows, templated storylines, or scrolling text with minimal or no narrative"; "highly similar storyline template[s] across multiple videos" — https://support.google.com/youtube/answer/1311392 | Faceless AI-voice card-per-day slideshows are the exact shape banned. Every video needs human-added value: live app session, Apocky's commentary, original art analysis, or an actual reading with a specific question. | ○ |
| Reused content | Ineligible: "content that exclusively features readings of other materials you did not originally create" | Codex read-alouds are fine (Apocky wrote it); never read third-party card-meaning text | ○ |
| Altered/synthetic disclosure | Disclosure required for realistic synthetic media that could mislead; "clearly animated, stylized, or fantastical" content is exempt; AI for production assistance needs no label — https://blog.youtube/news-and-events/disclosing-ai-generated-content/ and https://influencermarketinghub.com/ai-disclosure-rules/ | An AI oracle voice presented as an in-world persona ("the Oracle of the Glitch") is stylized; still tick the disclosure box on any video where the voice could be taken for a real person. Never clone a real voice. | ○ |
| Ad suitability for divination | The advertiser-friendly guidelines page fetched this session does **not** mention tarot, astrology, psychic, divination or occult content at all — https://support.google.com/youtube/answer/6162278 ; Jan 2026 update made the guidelines more permissive for dramatized/non-graphic sensitive topics — https://techcrunch.com/2026/01/16/youtube-relaxes-monetization-guidelines-for-some-controversial-topics/ ; one marketer reports divination is "borderline" for Google *Ads* accounts (advertising, not creator monetization) — https://medium.com/@marketolog4limes/the-tarot-magic-of-youtube-advertising-how-we-got-845-registrations-for-a-free-tarot-video-course-e1624e819f0e | No explicit category penalty. Risk is by adjacency: "harmful acts and unreliable content" if readings drift into health/finance/legal predictions. The site's disclaimer is the guard rail; put it in every description. | ○ |
| Shorts length | Up to 3 minutes since 15 Oct 2024 — https://support.google.com/youtube/answer/15424877 | Card decodes fit 45–90 s; MAXIMUM CHAOS clips 30–60 s | ○ |

---

## 2. Channel identity

| Element | Decision | Grounding | Tag |
|---------|----------|-----------|-----|
| Channel name | **Chaos Tarot** (handle `@chaostarot`; fallback `@chaos-tarot` or `@thechaostarot`) | Matches the domain and the product; brand search must land on one string. Handle availability unknown. | ⊘ availability |
| Tagline (banner + About) | "Eight divination systems. One digital oracle." | Site copy, verbatim | ✓ |
| One-line positioning | A cyberpunk tarot deck and seven more divination systems, read by an oracle that runs on "the rigorous logic of a sysadmin" and "the fluid, symbolic lens of a high priestess." Entertainment and personal reflection only. | Guidelines + site disclaimer | ✓ |
| Manifesto line for trailers | "The universe does not speak in clear, linear sentences; it screams in white noise, decaying atoms, and the collision of galaxies." | Codex §1.1 | ✓ |
| Mechanic language on screen | **Signal / Glitch**, never upright/reversed. "A Glitched card is not merely 'blocked'… It represents the archetype in a state of high entropy." | Codex §1.2 | ✓ |
| Names on screen | Shipped art titles (e.g. "16 Tower of Obsolescence", "Queen of Networks"); Codex names ("The Critical Error", "Suit of Artifacts") only as spoken lore, captioned "Codex name" | Art files vs Codex, contradiction resolved in brief | ✓ |
| Three voice registers | (1) **Oracle of the Glitch**: default persona, dual-boot, never breaks character. (2) **MAXIMUM CHAOS**: "Charlie Kelly but they're RIGHT about Pepe Silvia" — red string, escalating revelation, "WAIT… NO LISTEN… YOU SEE?" (3) **The Builder**: Apocky, Reali-Tea register, "the messy spreadsheet of your reality" | `CHAOS_ORACLE_VIBE.md`, guidelines §5.1, Drive doc | ✓ |
| Visual system | Void-black backgrounds, holographic edge glints, terminal/monospace lower-thirds, glitch transitions between cards; card art fills the frame (825×1425 px masters at 300 dpi exist) | `deck/manifest.json` | ✓ |
| Quadrant position | The seam between **Q1 Escapist Chaos** ("Nihilistic joy. A break from logic.") and **Q2 Divine Order** ("Seeking higher meaning and structure through ritual"), with build logs anchored in **Q3 Grounded Reality** ("Control, discipline, survival") | Drive doc §9; "no competitor in the doc occupies" the seam (brief) | ✓ doc / ◐ competitor gap |
| What the channel is not | Not a psychic. No zodiac pick-a-card predictions, no spells, no medical/legal/financial angles, no "this will happen to you" | Site disclaimer + policy gates | ✓ |

## 3. Target viewer profiles

| Persona | Quadrant | Who | Why they watch | What they do next (funnel) | Tag |
|---------|----------|-----|----------------|----------------------------|-----|
| **P1 The Disciplined Mystic** | Q2 Divine Order | 24–38, WitchTok 2026, journals daily, does the 7/31-day #TarotChallenge, "Plan Like a Witch" | Wants structure and a daily ritual with a darker aesthetic than "polished AI art"; distrusts glossy decks | Daily Oracle → daily draws + streak tracking (free) → reading journal + Memory Core (Oracle $3.33) | ◐ from ✓ doc |
| **P2 The Netrunner Skeptic** | Q1/Q3 seam | 22–35, works in or around tech, played Cyberpunk 2077, enjoys tarot as psychology and as a joke, allergic to woo | Wants the aesthetic and the bit: "The Devil's Algorithm", "Tower of Obsolescence", MAXIMUM CHAOS on tech news | Free Draw with a real question → shares the reading image → buys the physical deck as desk art | ◐ (WiseFish/Cyberpunk 2077 demand ○) |
| **P3 The Game Master** | Q3 Grounded Reality (tools) | 25–45, runs Call of Cthulhu / Cyberpunk RED / D&D, buys props | Wants table mechanics: Sanity Check draws ("Major Arcana (Glitched): −1d10 Sanity"), Wild Magic Surges | Physical deck + Kickstarter; streamer kit later | ✓ Codex §6 / ○ HALO doc "streamers" |
| **P4 The Shadow Worker** | Q2 → Q3 | 28–45, therapy-literate, uses tarot for self-reflection, explicitly not for prediction | Wants "Debugging the Soul": the System Purge exercise, the 10-card Shadow Work Deep Dive | Shadow Work template (free) → follow-up questions + journal (Oracle) | ✓ Codex §7 + templates |
| **P5 The Deck Collector** | Q4 Productive Comfort (shop) | 30–55, backs indie decks on Kickstarter, watches unboxings and flip-throughs | Wants haptics: 350gsm "Void Matte", holographic black edges, magnetic fractal box; wants comparisons | Kickstarter backer → physical deck | ✓ Codex §1.3 spec / ⊘ shipped spec / ○ unboxing format demand |

Primary for growth: P1 and P2 (largest pools). Primary for revenue: P1 (subscription) and P5 (deck).

## 4. Content categories (pillars)

| Code | Pillar | Quadrant | Cadence (A5) | Format | Feature it feeds | Policy note |
|------|--------|----------|--------------|--------|------------------|-------------|
| CD | **Card Decode** — one shipped card per video, Signal and Glitch, art analysis, Codex lore | Q2 | 2 Shorts/week + 1 long/2 weeks | Short + long | Free draws, collection tracker, deck | Must include original analysis and a live draw; never a static slideshow |
| MC | **MAXIMUM CHAOS** — conspiracy-board readings of tech/pop events | Q1 | 1–2 Shorts/week | Short | Maximum Chaos / Unhinged modes (Oracle) | Disclose synthetic voice; keep topical, never about tragedies (sensitive-events rule) |
| 8S | **Eight Systems** — runes, I Ching, Ogham, Lenormand, Geomancy, Astrology, Cross-System | Q2 | 1 long/week | Long | Each system page + persona | Search moat; low competition |
| DS | **Debugging the Soul** — shadow work, spreads, #TarotChallenge, Admin-Night-for-the-psyche | Q2/Q3 | 1 long/2 weeks + challenge Shorts | Both | Templates, journal, streaks, Memory Core | Reflection framing only; disclaimer in every description |
| TO | **Temporal Oracle** — daily/weekly/monthly draws, Cosmic Calendar, seasonal | Q2 | daily Short (batched) + weekly long | Both | Daily Oracle, Cosmic Calendar, Week Ahead / Monthly Cycle templates | Temporal anchoring (#October2026) per Drive doc; vary structure to avoid "templated" flag |
| BL | **Build Log / Technomancer's Workbench** — Apocky building the oracle, the deck, the campaign | Q3 | 1 long/2 weeks | Long | Memory Core, AI personas, PDF export, Kickstarter | The "messy spreadsheet"; founder voice is the strongest authenticity signal |
| GM | **Game of Madness** — TTRPG mechanics with the deck | Q3 | 1/month | Both | Physical deck | Clean, rules-based, ad-safe |
| DK | **Deck & Kickstarter** — unboxing, print, comparisons | Q4 | bursts around campaign | Long | Physical deck, Kickstarter | Timing ⊘ (A3) |

Mix for the first 90 days: 55% Q2 (CD/8S/DS/TO), 25% Q1 (MC), 20% Q3/Q4 (BL/GM/DK).

## 5. The first 100 video ideas, ranked by growth potential

Ranking = demand × differentiation × clip-ability × policy safety, judged from the evidence above (◐). Format: **L** = long-form (6–15 min, watch-hours path), **S** = Short (≤3 min). Card names are the shipped art titles. Items marked † use a feature listed in `FEATURE_LIST.md` (Jan 2026) whose presence in the consolidated Sept 2026 React app is ⊘ unverified; confirm before scripting.

| Rank | Working title | Format | Pillar | Why it could grow (one line) | Funnel hook (feature / card) |
|------|---------------|--------|--------|------------------------------|------------------------------|
| 1 | Null — The Apockalypse: the 79th tarot card that isn't supposed to exist | L | CD | Unique-to-deck novelty with zero search competition; "why is there a 79th card" is a curiosity hook | Null card; Free Draw |
| 2 | MAXIMUM CHAOS reads this week's AI outage (rolling topical) | S | MC | Topical + meme velocity (Q1); the Pepe Silvia voice is the most clippable asset the brand owns | Maximum Chaos mode† (Oracle) |
| 3 | I built an AI oracle that thinks like a sysadmin and a high priestess | L | BL | Founder build story rides AI curiosity; the strongest "human-added value" signal for YPP review | AI Oracle; 7 personas |
| 4 | Cyberpunk 2077's tarot vs a real cyberpunk tarot deck | L | DK | Piggybacks proven game-lore demand (WiseFish); converts gamers to P2 | Physical deck; Majors |
| 5 | Tower of Obsolescence: what the Tower means when your stack is deprecated | S | CD | The most-searched tarot card, reframed for tech workers | 16 Tower of Obsolescence; Signal/Glitch |
| 6 | Halloween 2026 spread: Death the System Crash, Tower of Obsolescence, The Devil's Algorithm | L | TO | Seasonal spike 5 weeks out; three strongest card titles in one thumbnail | Past, Present, Future template |
| 7 | The Devil's Algorithm: the card for doomscrolling | S | CD | Relatable; the Devil drove "cord-cutting" content on TikTok (Drive doc) | 15 The Devil's Algorithm; Shadow Work template |
| 8 | 31-day #TarotChallenge, October 2026: one card a day with a cyberpunk deck | L anchor + S | DS | Q2 ritual format proven on TikTok; temporal anchoring; returning viewers | Daily draws + streaks + journal |
| 9 | Death the System Crash isn't death, it's a migration | S | CD | Classic reassurance format with a dev twist ("System Migration", guidelines ✓) | 13 Death the System Crash |
| 10 | Signal vs Glitch: why this deck has no "reversed" cards | S | CD | Mechanic explainer that defines the brand in 45 s | Reversal toggle; Codex §1.2 |
| 11 | I Ching as a 64-bit oracle: "Digital Mutations" explained | L | 8S | Under-served system + a framing nobody else can use | I Ching; the Digital Oracle of Changes |
| 12 | Casting Elder Futhark runes in a 3D slot machine | S | 8S | Visual novelty beats rune lectures (Arith Härger comparison) | Runes†; the Runekeeper |
| 13 | What happens when you run all eight systems on the same question | L | 8S | Only this app can do it; comparison format performs | Cross-System reading |
| 14 | System Purge: the shadow-work exercise from The Chaos Codex | L | DS | Shadow work is a durable sub-niche; the exercise is original IP | Shadow Work Deep Dive (10 cards) |
| 15 | MAXIMUM CHAOS: "The Tower is card SIXTEEN" (the canonical bit) | S | MC | Recreates the doc's example reading; defines the voice; meme template | Maximum Chaos mode† |
| 16 | Tarot for programmers: the four suits are your stack (Codes / Networks / Signals / Vectors) | L | CD | Evergreen search target; maps the guidelines' Hardware/Network/Application/UX layers | All four suits |
| 17 | Geomancy is 16 four-bit numbers: "Sand Code" explained | L | 8S | Math-meets-divination hook; near-zero competition | Geomancy; the Geomantic Architect |
| 18 | The Sanity Check: replacing Call of Cthulhu sanity rolls with tarot | L | GM | TTRPG prop buyers; concrete rules table from Codex §6.1 | Physical deck |
| 19 | Memory Core: an oracle that remembers your last readings | L | BL | Unique feature demo; direct subscription driver | Memory Core (Oracle $3.33) |
| 20 | Every Major Arcana of the Chaos Tarot in 79 seconds | S | CD | Art-forward flip-through; shareable | Deck gallery; collection tracker |
| 21 | Daily Oracle: today's card (daily series, batched weekly) | S | TO | Cadence engine and habit loop; varied structure each day to stay policy-safe | Daily Oracle page; streaks |
| 22 | Reading the Week Ahead with the Eye of the Storm spread | L | TO | Weekly returning-viewer engine; signature 7-card spread | Week Ahead template; Codex §5.1 ⊘ in app |
| 23 | Unboxing the Chaos Tarot: Void Matte, holographic black edges, fractal box | L | DK | Deck-review format is established; Kickstarter trust | Physical deck (spec ⊘ as shipped) |
| 24 | The Scatter Method: throwing the whole deck in the air (Chaos Magick) | S | DS | Visually striking; Q1 energy inside a Q2 practice | Codex §5.2; physical deck |
| 25 | I let the oracle read my GitHub commit history | L | MC | Dev crossover humour; shows follow-up questions | Free Draw + follow-ups |
| 26 | Ogham: the Tree Oracle for people who live in terminals | L | 8S | Under-served system; Digital Druid persona | Ogham |
| 27 | Petit Lenormand in 36 cards: the Salon Oracle | L | 8S | Lenormand is a growing sub-niche (Neon Moon Lenormand exists as a cyberpunk deck) | Lenormand |
| 28 | Your natal chart as a Stellar Blueprint: astrology read like a system diagram | L | 8S | Largest adjacent demand (astrology) | Astrology; the Astrologer |
| 29 | There's a "Page of Fractures" in the files and it isn't in the deck | S | BL | Mystery/lore hook; comment bait that is true | Art file ✓; status ⊘ |
| 30 | Oracle of Static: the High Priestess as a black box | S | CD | AI black-box resonance | 2 Oracle of Static |
| 31 | The Zero Point: the Fool as a zero-day | S | CD | Guidelines' "Zero-Day" re-skin; series opener | 0 The Zero Point |
| 32 | Wheel of Misfortune: the RNG card | S | CD | Gamer resonance ("The RNG", Codex) | 10 Wheel of Misfortune |
| 33 | Which card makes you flinch? Pick your Shadow Card | S | DS | Participation prompt from Codex §7 step 1 | Shadow Work |
| 34 | Career Crossroads: the 5-card spread for the layoff era | L | DS | Timely anxiety, reflection framing only | Career Crossroads template |
| 35 | Relationship Clarity, read by a machine that doesn't take sides | L | DS | Love is the largest tarot demand; the neutral machine is the twist | Relationship Clarity template |
| 36 | Plan Like a Witch, but with a terminal | L | DS | Named TikTok trend (Drive doc); app-native yearly ritual | Cosmic Calendar; streaks; journal |
| 37 | Hanged Man of Hyperreality | S | CD | Philosophy-tok crossover | 12 Hanged Man of Hyperreality |
| 38 | Alchemist of Buffering: Temperance as the loading screen | S | CD | Funny visual; easy thumbnail | 14 Alchemist of Buffering |
| 39 | Chaos Tarot vs Norwegian Woodcut Tarot: the AI-art debate | L | DK | Trending deck per Drive doc; addresses the "polished AI art" backlash head-on | Physical deck |
| 40 | Teknebrae vs Neon Moon vs Chaos Tarot: three cyberpunk decks compared | L | DK | Deck buyers search comparisons | Physical deck |
| 41 | Wild Magic Surge table using a cyberpunk tarot deck (D&D) | S | GM | D&D crowd; rules from Codex §6.2 | Physical deck |
| 42 | Two Paths spread: should I ship or should I rewrite? | L | DS | Decision content with a dev question | Two Paths template |
| 43 | The 10 Singularities: cards outside the deck (Event Horizon, Heat Death, Strange Attractor…) | L | CD | Lore expansion; framed as digital/Codex-only (A8) | Codex §3.0 |
| 44 | The Strange Attractor: why the same problem keeps pulling you back | S | CD | Relatable chaos-theory hook | Singularity S-3 (lore) |
| 45 | Moon of Illusion vs Sun of Corruption: the two lies | S | CD | Pairing format; strong titles | 18 / 19 |
| 46 | How the oracle picks a voice per system (7 personas) | L | BL | AI curiosity; shows breadth | Personas ✓ site |
| 47 | Prompt-engineering a high priestess: the Dual-Boot Philosophy | L | BL | AI-tok crossover; original doc | AI_ORACLE_TRAINING_GUIDELINES |
| 48 | Building the print files: 79 cards at 825×1425 px, 300 dpi | L | BL | Maker/print nerds; Kickstarter credibility | deck/manifest.json ✓ |
| 49 | Reading the Kickstarter with the deck before we launch it | L | DK | Meta, campaign-timed | Kickstarter ⊘ timing |
| 50 | Runes vs I Ching vs Tarot on the same question | L | 8S | Comparison format; three personas | Cross-System |
| 51 | Ace of Codes: a new repo, a new life | S | CD | Suit opener; relatable | Ace of Codes |
| 52 | Ten of Signals: burnout as a card | S | CD | Burnout is universal | Ten of Signals |
| 53 | Queen of Networks: the person who knows everyone's uptime | S | CD | Court-card series | Queen of Networks |
| 54 | King of Vectors: momentum as leadership | S | CD | Court-card series | King of Vectors |
| 55 | Five of Codes: "Connection Refused" | S | CD | Guidelines' own example (Five of Pentacles) | Five of Codes |
| 56 | Scales of Chaos: Justice as the algorithm that judges you | S | CD | Algorithm anxiety | 8 Scales of Chaos |
| 57 | Hermit of the Void: Offline Mode as a spiritual practice | S | CD | Digital-detox trend | 9 Hermit of the Void |
| 58 | Judgement of the Glitch → World of the Glitch: the audit and the loop | S | CD | Series finale; two cards | 20 / 21 |
| 59 | The Singularity: the Magician as the Operator | S | CD | Card 1; AI-word resonance | 1 The Singularity |
| 60 | Lord of Entropy: the Emperor as firewall | S | CD | Card 4 | 4 Lord of Entropy |
| 61 | Mother of Paradox: the Empress as genesis node | S | CD | Card 3 | 3 Mother of Paradox |
| 62 | High Priest of Glitch: the Hierophant as protocol | S | CD | Card 5 | 5 High Priest of Glitch |
| 63 | Binary Schism: the Lovers as a fork | S | CD | Dev in-joke | 6 Binary Schism |
| 64 | Juggernaut of Unreason: the Chariot | S | CD | Card 7 | 7 Juggernaut of Unreason |
| 65 | Fortitude of Fragmentation: Strength as bandwidth | S | CD | Card 11 | 11 Fortitude of Fragmentation |
| 66 | Star of Fragmentation: hope after the crash | S | CD | Card 17 | 17 Star of Fragmentation |
| 67 | The Chaos Codex read aloud: The Manifesto of the Void | L | CD | Long watch time; original text, so reused-content-safe | Codex §1.0 |
| 68 | I asked the oracle in Unhinged Mode about my code review | S | MC | Dev humour; feature demo | Unhinged Mode† |
| 69 | Follow-up questions: interrogating the oracle until it breaks | S | MC | Funny; demonstrates a paid feature | Follow-ups (Oracle) |
| 70 | Cosmic Calendar: what November 2026 looks like in the app | S | TO | Temporal anchor; monthly repeat | Cosmic Calendar page |
| 71 | Monthly Cycle spread for November 2026 | L | TO | Monthly returning viewers | Monthly Cycle template |
| 72 | Yes or No Clarity in 15 seconds | S | TO | Snackable; loops | Yes or No Clarity template |
| 73 | Mind · Body · Spirit check-in: the admin night for your psyche | S | DS | #AdminNight framing (Q3) | Mind · Body · Spirit template |
| 74 | Creative Unblocking spread for artists in the AI era | L | DS | Creative audience; addresses AI-art tension honestly | Creative Unblocking template |
| 75 | Love Potential: the Networks suit is your love language | S | DS | Love + suits | Love Potential template |
| 76 | Vacuum Decay: the card for when your worldview breaks | S | CD | Strong title | Singularity S-4 (lore) |
| 77 | The Observer: you change the reading by looking at it | S | CD | Quantum-tok | Singularity S-5 (lore) |
| 78 | Why the suits changed from Frequencies/Artifacts to Networks/Codes | S | BL | Turns a documented contradiction into lore | Codex vs art ✓ |
| 79 | Community gallery: the strangest readings shared this week | L | TO | UGC loop; community feature | Community gallery |
| 80 | Collection tracker: I drew all 79 cards; here's the last one | L | TO | Completionist arc | Collection tracker |
| 81 | 90-day streak: what daily draws did to my journal | L | DS | Streak narrative; Phase 2 timing | Streaks; journal |
| 82 | A GM runs a Cyberpunk RED session with tarot-driven events | L | GM | Actual-play hook | Physical deck |
| 83 | Sanity draw live: Major Arcana Glitched = −1d10 | S | GM | Rules clip | Codex §6.1 |
| 84 | Streamer kit: using the deck on stream | L | GM | Streamer audience (HALO doc ○) | Shareable reading images ✓ repo |
| 85 | PDF export: your reading as a dossier | S | BL | Feature demo; premium | PDF export (Oracle) |
| 86 | Wu Xing in a cyberpunk deck: Metal is the network | L | CD | Chinese-elements crossover | Wu Xing† |
| 87 | Sigil crafting: turn an intention into a glyph | S | 8S | Sigils trend on WitchTok | Sigil crafting† |
| 88 | Reading with dark ambient on: the ambience sidebar | S | TO | Lo-fi watch-time style | Spotify ambience† |
| 89 | What the Ethics page says and why the disclaimer is the point | S | BL | Trust; pre-empts "is this real" comments; ad-safe | Ethics page ✓ site |
| 90 | Ask the oracle: viewer questions (monthly) | L | TO | Community loop; comment mining | Free Draw |
| 91 | Knight of Signals: the hot take | S | CD | Court-card series | Knight of Signals |
| 92 | Page of Vectors: the intern with a startup idea | S | CD | Court-card series | Page of Vectors |
| 93 | Three of Networks: the group chat | S | CD | Minor series | Three of Networks |
| 94 | Eight of Codes: the grind | S | CD | Minor series | Eight of Codes |
| 95 | Nine of Networks: the wish, granted by the algorithm | S | CD | Minor series | Nine of Networks |
| 96 | Four of Vectors: the launch party | S | CD | Minor series | Four of Vectors |
| 97 | Seven of Signals: the exfiltration | S | CD | Minor series | Seven of Signals |
| 98 | Card back reveal and the fractal box | S | DK | ASMR/unboxing micro-format | Card Back ✓; box ⊘ |
| 99 | Situation · Action · Outcome for a bug I can't fix | S | DS | Dev-relatable 3-card | Situation · Action · Outcome template |
| 100 | One year of chaos: what 365 readings taught the Memory Core | L | BL | Anniversary retrospective (Phase 3) | Memory Core |

Coverage check: 8 systems (11, 12, 13, 17, 26, 27, 28, 50, 87), 22 Majors + Null (1, 5, 7, 9, 30–32, 37–38, 45, 56–66), all four suits (16, 51–55, 91–97), 12 of 14 templates (6, 8, 14, 22, 34, 35, 42, 71–75, 99), 3 Singularities as lore (43, 44, 76, 77), Codex §5, §6, §7 (14, 18, 22, 24, 33, 41, 82, 83).

## 6. Recommendations

| # | Recommendation | Oracle (how we would see it worked) | Falsifier (what would show it failed) |
|---|----------------|--------------------------------------|----------------------------------------|
| R1 | Name the channel **Chaos Tarot**, tagline verbatim from the site, disclaimer in every description | Within 60 days of launch, YouTube Studio shows "chaos tarot" as a top-5 search term driving traffic; the domain gets referral sessions from youtube.com | Handle unavailable and no acceptable fallback; or brand-search impressions stay at zero after 20 uploads |
| R2 | Sit on the Q1/Q2 seam: 55% Q2, 25% MC (Q1), 20% BL/GM/DK | After 30 uploads, MC Shorts median views ≥ 2× CD Shorts median **and** CD/8S long-form average view duration ≥ 40% | MC Shorts underperform CD Shorts, or MC videos draw "limited ads" flags or a comment pattern of "cringe"; then cut MC to 10% |
| R3 | Take the watch-hours path to YPP, not the Shorts path | Long-form supplies ≥ 60% of qualified watch hours by day 60; 4,000 hours reached before 31 Jan 2027 | Long-form AVD < 3 min after 10 uploads, or Shorts supply > 70% of watch time (then the Shorts path is the real path and the roadmap must change) |
| R4 | Human-added value in every video: a live app session, Apocky's commentary, or original art analysis; AI voice disclosed as an in-world persona; no static slideshows | YPP review passes with zero "inauthentic content" or "reused content" findings | Rejection citing inauthentic/reused content, or a "limited ads" pattern on card-decode Shorts |
| R5 | Build the evergreen card/system library (≈258 units) before chasing trends | Search traffic ≥ 25% of views by day 90; card-title queries appear in Studio | Search traffic < 10% at day 90 while suggested/browse carries everything |
| R6 | Never do zodiac pick-a-card prediction; differentiate against Minnow Pond-style channels | "Other channels your audience watches" shows Esoterica / WiseFish / Kelly-Ann Maddox / Arith Härger, not pick-a-card channels | The overlap shows pick-a-card channels and comments repeatedly ask "what does it mean for Scorpio" |
| R7 | Map each pillar to one product surface with UTM-tagged end screens | UTM sessions on chaos-tarot.com ≥ 2% of views; free-tier signups attributable to YouTube by day 90 | UTM sessions < 0.5% of views after 20 uploads |

## Open questions for Apocky

1. Handle: is `@chaostarot` available, and do you want "Chaos Tarot" or "The Technomantic Oracle" as the channel name?
2. Which of these are live in the consolidated Sept 2026 React app: Maximum Chaos, Unhinged Mode, 3D rune casting, Sigil crafting, Spotify ambience, Wu Xing badges? (`FEATURE_LIST.md` is dated Jan 2026.) Ideas marked † depend on the answer.
3. Is the Eye of the Storm (7-card) spread in the app, or Codex-only?
4. "Page of Fractures": a real 57th Minor, a misnamed duplicate, or an easter egg?
5. Are the 10 Singularities digital-only, printed, or planned for the Kickstarter? (A8)
6. Kickstarter date: if within 90 days, DK ideas 4, 23, 39, 40, 49 move to the front (A3).
7. Faceless or on-camera for the BL pillar? Founder-on-camera is the strongest authenticity signal under the inauthentic-content policy but changes production (A2).
8. Run Unirecall locally with the command in the Rail status table and paste the JSON so prior channel decisions can be folded in.

## Issue candidates

| Title | Phase | Priority | Acceptance oracle | Falsifier | Depends on |
|-------|-------|----------|-------------------|-----------|------------|
| Claim handle and set channel identity (name, tagline, banner, About with disclaimer) | 0 | P0 | Channel live with "Chaos Tarot" name, site tagline, disclaimer in About; link added to chaos-tarot.com footer | Handle conflicts unresolved after 3 fallbacks | Q1 |
| Feature-availability audit for † items (Maximum Chaos, Unhinged, 3D runes, sigils, Wu Xing, Spotify) | 0 | P0 | Table of feature → live/not-live with screenshots from the production app | Any † idea scripted before the audit | Q2 |
| Unirecall recall for prior channel decisions | 0 | P1 | JSON pasted into the plan dir; decisions merged or explicitly none | Section ships with "no prior context" while a recall exists | Apocky local run |
| Card-decode series bible (79 entries: art title, Codex name, Signal, Glitch, tech re-skin, one-line hook) | 0 | P0 | File with 79 rows; each row sourced to art file + Codex section | Rows invented without a Codex or guidelines source | Feature audit |
| Persona-to-UTM map and end-screen templates | 0 | P1 | 8 UTM codes (one per pillar) resolving on chaos-tarot.com; end-screen template per pillar | UTM sessions untracked at first upload | Handle |
| Ship ideas 1–10 as the launch batch | 1 | P0 | 10 uploads live inside 21 days; each passes the R4 human-value checklist | Any upload is a static slideshow or undisclosed synthetic voice | Series bible |
| Competitor overlap check at 30 uploads | 1 | P2 | Studio "other channels your audience watches" screenshot logged with date | Overlap dominated by pick-a-card channels (R6 falsifier) | Launch batch |
| Eight Systems explainer set (ideas 11, 12, 13, 17, 26, 27, 28) | 1–2 | P1 | 7 long-form videos live; each demonstrates the system's persona in-app | Any explainer reads third-party meaning text (reused-content risk) | Feature audit |
| October 31-day #TarotChallenge run (idea 8) | 1 | P1 | 31 daily Shorts + 1 anchor long; structure varied per day; streak screenshot at day 31 | Days templated identically (inauthentic-content shape) | Handle |
| Halloween 2026 spread (idea 6) | 1 | P1 | Published ≥ 7 days before 31 Oct | Published after 28 Oct | Series bible |
| YPP threshold tracker (subs, watch hours, Shorts views, 31 Jan 2027 deadline) | 1–3 | P0 | Weekly row in `ISSUES.md`/tracker; application submitted the day thresholds are met | Thresholds met but application not filed within 7 days | Handle |
| Kickstarter content burst (ideas 4, 23, 39, 40, 49, 98) | 2 | P1 | All six published inside the campaign window | Campaign runs with < 3 DK videos live | Q6 |
| Verify the 2027 doubled-threshold claim against the official page | 0 | P1 | Official Help Center text quoted with URL in this file | Plan cites 8,000 h / 20M as fact without an official quote | none |

## CSL annex

Σ Chaos Tarot on YouTube = "Chaos Tarot" channel · seam(Q1 MAXIMUM CHAOS, Q2 disciplined mystic) · 8 systems · 79 shipped card titles · 5 personas (P1 Disciplined Mystic, P2 Netrunner Skeptic, P3 Game Master, P4 Shadow Worker, P5 Deck Collector) · 100 ideas ranked, CD/8S library as search moat, MC as reach engine, BL as authenticity proof.
W! Watch-hours path to YPP (1,000 subs + 4,000 h, ○ https://support.google.com/youtube/answer/72851); apply before 31 Jan 2027; human-added value + synthetic-voice disclosure on every upload; disclaimer in every description; shipped art titles on screen, Codex names as lore.
W! Confirm † features, the handle, Kickstarter timing and Unirecall context before scripting ideas that depend on them.
N! No zodiac pick-a-card prediction, no spells/hexes, no medical/legal/financial angles, no static AI slideshows, no cloned real voices, no thresholds or stats stated without a URL.
N! Never modify `/home/user/the-chaos-tarot`; never treat the 2027 doubled thresholds as confirmed until quoted from the official page.
