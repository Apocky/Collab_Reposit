# 1. YouTube Monetization Roadmap Operating System

## Source prompt

> "Act as a world-class YouTube strategist and monetization consultant. Build a complete operating system for taking my YouTube channel from zero toward monetization requirements. Analyze my niche, audience, competitors, resources, and goals. Create my channel positioning, content pillars, upload strategy, video formats, growth systems, retention framework, monetization plan, and weekly execution schedule

(Verbatim from `prompts/SOURCE_PROMPTS.md`, prompt 1; no closing quote in the source.)

## Assumptions used

| # | Used as | Status after this section's checks |
|---|---------|-----------------------------------|
| A1 | Plan starts from zero | ✓ Still no Apocky-owned channel found. **New finding:** the name is occupied. `youtube.com/@chaostarot` (channel `UCm03OAZMnBwc8G6IYR1SQ9Q`), `x.com/chaostarot`, `tiktok.com/@chaostarot.com` and `chaostarot.com` ("© 2015–2026 ChaosTarot.com", a chaos-magick creator selling "Trinary: The Magick Web-App" and a "Chaos Magician Tarot") belong to someone else (✓ fetched https://www.chaostarot.com/about-us/ and search result https://www.youtube.com/channel/UCm03OAZMnBwc8G6IYR1SQ9Q on 2026-09-27). `@thechaostarot` and `@chaos-tarot` returned HTTP 404 on youtube.com (✓ fetched) → ◐ likely free. Phase 0 includes a handle decision. |
| A2 | Faceless-first: AI Oracle voice + card art + app capture, founder segments optional later | Used, with one change forced by policy: a founder-perspective layer is recommended from week 1 (see §0.3). |
| A3 | Goal order: YPP → funnel to chaos-tarot.com → deck + Kickstarter | Used. YPP has a hard date now (see §0.1), which reorders nothing but sets the pace. |
| A4 | Near-zero cash; own AI keys, free tiers, self-hosting | Used. Tool prices are section 5's job; none are load-bearing here. |
| A5 | 8–12 h/week | Used for the primary schedule (§9.1); 4 h/week alternate in §9.2. |
| A6 | Tracking as Markdown in this repo | Used for the issue table. |
| A8 | Physical deck = 79 cards; Singularities are Codex lore | Used. Singularity content is framed as "from the Codex", never "in the box". |

Not used: A7 (prompt 8 unknown; nothing here depends on it).

Tag key: ✓ VERIFIED this session · ◐ INFERRED from ✓ premises · ○ REPORTED (source named) · ⊘ ASSUMED. Every recommendation carries **O:** (oracle: how we would see it worked) and **F:** (falsifier: what would show it failed).

---

## 0. Policy frame (verified 2026-09-27; read before anything else)

### 0.1 YouTube Partner Program thresholds and the February 2027 wall

| Item | Value | Tag / source |
|------|-------|--------------|
| Current YPP ad-revenue entry | "1,000 subscribers with 4,000 qualified watch hours in the last 12 months, or … 1,000 subscribers with 10 million qualified Shorts views in the last 90 days" | ○ https://support.google.com/youtube/answer/72851 |
| **New entry thresholds from 2027-02-01** | "8,000 qualified watch hours in the last 365 days, or 20M qualified Shorts views in the last 90 days" plus 1,000 subscribers; "This update won't impact creators already in YPP." | ○ https://blog.youtube/news-and-events/youtube-partner-program-updates-2027-new-opportunities-earn/ (posted 2026-08-10) · ○ https://support.google.com/youtube/answer/12843009 · ○ https://techcrunch.com/2026/08/10/youtube-now-requires-creators-to-have-twice-as-many-watch-hours-to-start-earning-money/ |
| Terms acceptance deadline for existing partners | 2027-01-31 | ○ https://support.google.com/youtube/answer/12843009 |
| Applications pending on 2027-02-01 | Not addressed on the help page | ○ same page; ⊘ treat as "must be *accepted* before Feb 1" |
| Review time after applying | "typically in about 1 month"; delays possible | ○ https://support.google.com/youtube/answer/72851 |
| Other requirements | 2-Step Verification; "Advanced features" access; one active AdSense for YouTube account; eligible country; no active Community Guidelines strikes | ○ same page |
| Shorts feed views vs watch hours | Shorts-feed watch time does not count toward the 4,000 h; the two paths never combine | ○ https://vidiq.com/blog/post/youtube-partner-program-guide/ (aggregator; consistent with the official "or" wording) |
| Shorts ad-revenue eligibility from 2027-02-01 | "at least 10 million qualified Shorts views in the last 90 days" per month; creator share "45% of their allocated revenue" | ○ https://support.google.com/youtube/answer/12504220 |
| Shorts length | "up to three minutes in length" with "a square or vertical aspect ratio", uploads on/after 2024-10-15; 3-minute Shorts in the Shorts feed are revenue-eligible | ○ https://support.google.com/youtube/answer/15424877 |
| Staying active in YPP (all members) | one of: 1,000 qualified watch hours / 365 d, or 1M qualified Shorts views / 90 d, or "2 long-form videos or 5 Shorts uploaded every 90 days" | ○ https://support.google.com/youtube/answer/12843009 |
| Fan-funding early-access tier | 500 subs + 3 public uploads in 90 d + 3,000 watch hours or 3M Shorts views (unlocks memberships, Super Thanks, etc.) | ○ https://vidiq.com/blog/post/youtube-partner-program-guide/ and https://air.io/en/monetization/youtube-partner-program-requirements-2026-the-complete-guide — **not quoted on the official page in my fetch; the official page only confirms "entry thresholds for our Fan Funding and shopping products remain unchanged"**. Kept out of the critical path. |
| Hype (fan-boost for small channels) | "designed for up-and-coming creators in the YouTube Partner Program with 500–500,000 subscribers"; hype window = first 7 days; Shorts not hype-eligible | ○ https://support.google.com/youtube/answer/15509925 |
| Mobile live streaming | "At least 50 subscribers"; under 1,000 subs "we may limit the number of viewers" and archives default private | ○ https://support.google.com/youtube/answer/9228390 |
| External links in info cards | YPP members only (description links unaffected) | ○ https://vidiq.com/blog/post/how-to-unlock-youtube-features/ ; ◐ description links allowed pre-YPP (standard behaviour, not re-verified) |

**The date arithmetic (◐ from the ○ rows above):** today is 2026-09-27. 2027-02-01 is 127 days out. Review ≈ 1 month → the channel must **apply by ~2026-12-27 (day 91)** with 1,000 subs and 4,000 qualified long-form watch hours already banked, or it faces the doubled bar. The thread's "90 days" is, by coincidence, the exact real window. Watch hours banked from October 2026 still count toward the 8,000 h / 365-day bar after February, so nothing done in this window is wasted if the sprint misses.

**Feasibility, stated honestly (◐):** 4,000 h = 240,000 minutes. At a 4-minute average view duration that is 60,000 long-form views inside the window, ≈ 670/day from a standing start, ≈ 2,500 per video across 24 long-form uploads. The Shorts path (10M views / 90 d ≈ 111k/day) is harder from zero. So: **long-form is the YPP path; Shorts are the discovery path.** Treat the December application as a gated stretch target (gate in §10), and design every long-form video to bank hours toward the 2027 bar as the fallback.

### 0.2 Ad suitability for tarot/divination

- ✓ Fetched https://support.google.com/youtube/answer/6162278: tarot, astrology, divination, occult, psychic, supernatural and paranormal are **not mentioned** anywhere in the advertiser-friendly guidelines. "Controversial issues" is defined as "child abuse, adult sexual abuse, sexual harassment, self-harm, suicide, eating disorders, domestic abuse, and abortion." "Harmful acts and unreliable content" is the category to steer clear of; it is about harm and misinformation, not about entertainment divination.
- ◐ Therefore the site's own framing, "Chaos Tarot is for entertainment and personal reflection only. Readings are not a substitute for professional medical, legal, financial, or psychological advice" (✓ https://chaos-tarot.com), is also the ad-safety strategy. No health, money-advice, legal, or "will my ex come back / is my partner cheating" angles; no "predict your death" gimmicks; no shocking thumbnails.
- ○ The only tarot-specific signal found is that Google *Ads* accounts treat "astrology, astronomy, divination and tarology" as borderline (https://medium.com/@marketolog4limes/the-tarot-magic-of-youtube-advertising-how-we-got-845-registrations-for-a-free-tarot-video-course-e1624e819f0e). That is about buying ads, not earning them; with a zero-cash budget (A4) it is irrelevant.
- **O:** ≥ 90% of uploads show the green "On" monetization icon after YPP, and no video is flagged "Limited". **F:** any video yellow-flagged for content, or a manual review that cites "unreliable content".

### 0.3 Inauthentic content and AI disclosure (the two rules that shape the format)

- ○ https://support.google.com/youtube/answer/1311392 (updated 2025-07-15, "repetitious content" renamed "inauthentic content"): ineligible = content that "appears mass-produced, generic, repetitive"; examples include "Image slideshows, templated storylines, or scrolling text with minimal or no narrative" and "AI-generated content made with generic or unoriginal templates". Allowed = "consistent intros/outros but substantially different core material", series "where each video has distinct focus".
- ◐ Risk profile of the A2 format: a daily "card of the day" Short built from card art + synthetic voice + template is *exactly* the described shape. Mitigations, each a design rule in §3 and §5: (1) the AI Oracle is a **character** with a distinct voice (MAXIMUM CHAOS, ✓ `docs/CHAOS_ORACLE_VIBE.md`), never generic TTS over stock; (2) every video contains live app capture and a real reading on a real question, so core material differs; (3) Apocky's own perspective appears in every long-form (voice, hands, or on-camera), which is the cheapest authenticity signal; (4) daily Shorts rotate across the eight systems and vary structure.
- ○ AI disclosure: https://support.google.com/youtube/answer/14328491 lists "Cloning one's own voice to create voice overs or dubs", "Production assistance, like using generative AI tools to create or improve a video outline, script, thumbnail" and clearly unrealistic content as **not** requiring disclosure; realistic synthetic footage of real places/people and "AI generated music" **do**. Penalties for consistent non-disclosure include "removal of content or suspension from the YouTube Partner Program". Labels show in "How this content was made" and, for sensitive topics, on the player (○ https://support.google.com/youtube/answer/15447836).
- ◐ Recommendation: tick the "altered or synthetic content" box on every video that uses a synthetic Oracle voice or AI-generated music. It costs nothing (a description-section label for non-sensitive topics), removes the enforcement risk, and matches the brand ("The Technomantic Grimoire").
- **O:** zero inauthentic-content or disclosure actions in Studio through day 90; YPP review passes first time. **F:** a YPP rejection citing "reused" or "inauthentic" content, or a label applied by YouTube that Apocky did not set.

---

## 1. Analysis

### 1.1 Niche

| Dimension | Finding | Tag |
|-----------|---------|-----|
| Product | "Eight divination systems. One digital oracle." 79-card tarot, 24 runes, 64 hexagrams, 25 Ogham, 36 Lenormand, 16 geomancy figures, 43 astrology symbols = "287 instruments of divination" (site copy) | ✓ https://chaos-tarot.com |
| Category on YouTube | Tarot/spirituality is a large, saturated, mostly pick-a-card and zodiac-timing niche (see §1.3) | ○ Feedspot |
| Chaos Tarot's differentiators nobody in §1.3 has | (a) a **unique 79-card deck with unique names** ("Tower of Obsolescence", "Death the System Crash", "Null — The Apockalypse") → zero search competition on the exact titles; (b) **Signal/Glitch** instead of upright/reversed; (c) a **written lore bible** (the Codex) and a **written AI persona bible** (the training guidelines); (d) **eight systems in one app**, so cross-system readings are native; (e) a **MAXIMUM CHAOS** voice that is clip-shaped; (f) an app that publishes a **daily draw across all eight systems** (✓ https://chaos-tarot.com/daily-oracle showed "Sun of Corruption (Reversed)", "Tiwaz (Reversed)", "Naive Protocol", "Duir — Root Authority", "Les Souris — Data Leak", "Amissio — Memory Leak", "Conjunction" for 2026-09-27) and a **Cosmic Calendar** of 2026–2027 moons, eclipses, solstices (✓ https://chaos-tarot.com/cosmic-calendar) | ✓ repo + site |
| Tailwind / hazard | CD Projekt Red's official *Cyberpunk 2077* tarot deck (Panini/Dark Horse) ships **Fall 2026** | ○ https://www.pushsquare.com/news/2026/04/cyberpunk-2077s-gorgeous-tarot-cards-will-grace-an-official-tarot-deck-out-this-year · https://www.darkhorsedirect.com/products/cyberpunk-2077-tarot-deck |
| Where the brand sits | On the seam between "Escapist Chaos" (Q1) and "Divine Order" (Q2) of Apocky's own Q1-2026 trend matrix, which no competitor in that doc occupies | ○ Drive doc "TikTok Hashtag Trends: Spirituality & Holidays" via BRIEF §5 |

◐ Read: "cyberpunk tarot" search interest will spike with the CDPR deck in Q4 2026. The channel should ride the term as *comparison and context* ("what a cyberpunk deck actually reads like", "the 5 cards CDPR didn't rename and we did") without using CDPR marks in titles/thumbnails. **O:** ≥ 1 video in the top 20 YouTube results for "cyberpunk tarot" by day 60. **F:** none in the top 50, or a takedown/claim on that content.

### 1.2 Audience

| Persona (from Codex + guidelines) | What they search / watch | What we sell them | Tag |
|------|------|------|------|
| **The Modern Mystic** ("compass for the Modern Mystic", Codex §1.1) — WitchTok's "disciplined mystic": ritual-as-productivity, 7/31-day #TarotChallenge journaling | daily card, weekly energy, journaling prompts, shadow work | Daily Oracle (free) → Oracle sub for Memory Core + journal + PDF export | ✓ Codex; ○ TikTok doc |
| **The Visual Philosopher** — tech workers, sysadmins, designers who like the "Dual-Boot Philosophy" (✓ guidelines §1) | "tarot for programmers", how the AI oracle works, the aesthetics | build-in-public, technomancy explainers, the deck | ✓ guidelines |
| **The Shadow Worker** — "Shadow Work: Debugging the Soul" (Codex §7.0) | shadow work spreads, the System Purge exercise | Shadow Work Deep Dive template (✓ `shared/reading-templates.json`, 10 positions) → Oracle sub | ✓ |
| **The Game Master** — Codex §6.0 "The Game of Madness": Sanity Check draws, Wild Magic Surges for *Call of Cthulhu*, *Cyberpunk RED*, D&D | "tarot as GM tool", "wild magic table" | physical deck (a table prop), Kickstarter | ✓ Codex; ○ HALO doc lists "streamers looking for table-friendly content" |
| **The Absurdist scroller** — Q1 "Escapist Chaos" | anything that reads like the Pepe Silvia wall | Shorts → channel → free tier | ○ TikTok doc; ✓ vibe doc |

⊘ Demographics (age, country, gender) are unknown; YouTube Analytics on the first 30 days answers this. **O:** by day 30, ≥ 60% of views from US/UK/CA/AU (English, ad-rich) and the two top-retaining personas identifiable from comment language. **F:** views concentrated in non-English geos with < 2 min AVD (a signal the Shorts are being served as generic art content, not to a persona).

### 1.3 Competitors

Top tarot channels (○ https://videos.feedspot.com/tarot_youtube_channels/, page dated 2026-09-26; subscriber counts as listed there):

| Channel | Subs | Format | What they do not have |
|---------|------|--------|----------------------|
| ALI's Tarot | 1M | "free general LOVE readings" | any product beyond readings |
| Minnow Pond Tarot | 967K | personal readings | a deck IP |
| EAT READ LOVE INC | 911K | entertainment readings | multi-system |
| 303 High Priestess Tarot | 903K | love/money/path readings | app funnel |
| Sacred Knowledge Tarot | 684K | weekly readings by zodiac sign | lore |
| Tyler Tarot | 625K | zodiac sign readings | — |
| Truth Well Told Tarot | 430K | daily/weekly/monthly zodiac | — |
| Red Fairy Tarot | 320K | "safe space", spells, witch tips | closest to WitchTok Q2 |

◐ Pattern: the top of the niche is on-camera, pick-a-card / zodiac-timing, love-heavy, high-volume (several uploads per week, often 30–60 min). None owns a deck IP with its own names, none has a software product, none has a comedic oracle voice. Their format is the 4,000-hour engine (long videos, loyal daily viewers). Our format cannot copy their length on 8–12 h/week; it must copy their **cadence and timing hooks** (weekly energy, moon events) with shorter, denser videos.

Direct name collision (✓ §Assumptions A1 row): `chaostarot.com` / `@chaostarot` is a chaos-magick tools creator with a "Chaos Magician Tarot" and web-app. Size not readable this session (vidIQ returned HTTP 429, Social Blade 403). ◐ Consequence: search for "chaos tarot" will show both; the channel name, handle, and every thumbnail must carry the hyphenated domain or a visual mark (the Null card, the glitch wordmark). ⊘ Whether Apocky has any prior claim or contact with that owner. **Product defect found:** ✓ `the-chaos-tarot/lib/social-share.js` line 58 sets `twitterHandle: '@ChaosTarot'`, and `x.com/chaostarot` is the *other* entity (○ search result "Chaos Tarot (@ChaosTarot) on X"). Shared readings currently credit a stranger. Logged in the issue table.

**O:** by day 90, a YouTube search for "chaos tarot" shows Apocky's channel in the top 3 results. **F:** the other channel outranks on the brand term after 90 days, or a name dispute arrives.

### 1.4 Resources

| Resource | State | Use in the channel | Tag |
|----------|-------|--------------------|-----|
| 79 card JPGs with shipped titles (`New/`), 825×1425 @300dpi PNGs + `manifest.json` (`deck/`) | exists | every thumbnail, every card reveal; print-quality means 4K crops are free | ✓ |
| The Chaos Codex (Markdown + Google Doc + printed PDF): per-card Signal/Glitch text, 10 Singularities, Eye of the Storm 7-card spread, Scatter Method, TTRPG rules, Shadow Work exercise | exists | scripts for pillar P3, P5, P7 come straight from it | ✓ |
| `docs/AI_ORACLE_TRAINING_GUIDELINES.md` persona prompt ("speaks with the authority of a High Priestess and the precision of a Systems Architect") | exists | the Oracle character's script generator | ✓ |
| `docs/CHAOS_ORACLE_VIBE.md` MAXIMUM CHAOS | exists | pillar P2, the clip engine | ✓ |
| 20 reading templates (`shared/reading-templates.json`: General, Career, Relationships, Self Discovery, Timing, Decisions, Daily Guidance, Past·Present·Future, Situation·Action·Outcome, Career Crossroads, Relationship Clarity, Love Potential, Shadow Work Deep Dive, Yes or No Clarity, Two Paths, Week Ahead, Monthly Cycle, Mind·Body·Spirit, Creative Unblocking, Free Draw) | exists | one long-form series per template = 20 evergreen videos | ✓ |
| Live app: 8 systems, AI Oracle, Memory Core, Daily Oracle page, Cosmic Calendar page, streaks, PDF export, 7-day free trial | live | screen capture is the B-roll; Daily Oracle is the daily Short's source of truth | ✓ site |
| `app/core/shareable-image.js` (600×900 canvas card image), `lib/social-share.js` (X, Facebook, Pinterest, Reddit, WhatsApp) | exists | seed for an automated 1080×1920 Short-frame generator (product request, not a fact) | ✓ |
| AI provider keys (OpenAI, Anthropic, Gemini), Supabase, Vercel, Resend email, Sentry | live | scripting, TTS, thumbnails, an email list for the funnel | ✓ brief |
| Apocky's agent infrastructure (apocrypha-core, mempalace, 3MNEME) | exists | automation of research, scripting, publishing (section 5's scope) | ✓ brief; ⊘ capacity |
| vidIQ connector | `connect_incomplete` | keyword research once connected | ✓ brief |
| Cash | ≈ 0 | no paid promotion, no paid tools in the critical path | ⊘ A4 |
| Time | 8–12 h/week | sizes the cadence (§4, §9) | ⊘ A5 |
| Kickstarter | budget sheet shared 2026-09-07 | timing unknown; §8 gives a launch-content block that slots in when dated | ✓ brief; ⊘ timing |

### 1.5 Goals

| # | Goal | Measure | Deadline | Tag |
|---|------|---------|----------|-----|
| G1 | YPP ad-revenue eligibility under the current bar | 1,000 subs + 4,000 qualified long-form watch hours, application submitted | apply by 2026-12-27; accepted by 2027-01-31 | ◐ from §0.1 |
| G1-fallback | Bank hours toward the 2027 bar | ≥ 2,000 qualified watch hours by 2026-12-31 (half of the old bar, a quarter of the new) | 2026-12-31 | ◐ |
| G2 | Funnel to chaos-tarot.com | ≥ 1,000 tracked sessions from YouTube; ≥ 100 free accounts; ≥ 10 Oracle trials started | day 90 | ⊘ rates |
| G3 | Deck + Kickstarter awareness | ≥ 300 email subscribers for "deck drop / Kickstarter" list; ≥ 20 physical deck orders attributed to YouTube | day 90 or campaign launch | ⊘ |
| G4 | Policy-clean channel | zero strikes, zero limited-ads flags, zero inauthentic-content actions | continuous | ○ §0 |

---

## 2. Channel positioning

| Element | Recommendation | Tag |
|---------|----------------|-----|
| Channel name | **Chaos Tarot** (matches product); handle **@chaos-tarot** (matches the domain, 404 today) with **@thechaostarot** as fallback | ✓ 404s; ◐ availability |
| Positioning statement | "The digital oracle for people who suspect the universe is a badly documented system. Eight divination systems, one cyberpunk deck, and an AI Oracle that has definitely seen the red string." | ◐ from site + vibe doc |
| Tagline on banner | "Eight divination systems. One digital oracle." (site copy, verbatim) | ✓ |
| Channel description (draft) | "Chaos Tarot is a digital divination platform: a 79-card cyberpunk tarot deck, Elder Futhark runes, I Ching, Ogham, Lenormand, Geomancy and Astrology, read by an AI Oracle. Cards run in **Signal** (clear) or **Glitch** (corrupted) — not upright and reversed. Free daily draws at chaos-tarot.com. For entertainment and personal reflection only; readings are not a substitute for professional medical, legal, financial, or psychological advice." | ✓ copy; ◐ assembly |
| Category | Entertainment (not Education, not People & Blogs) — matches the disclaimer and ad-safety line | ◐ |
| Visual mark | the **Null — The Apockalypse** card and the glitch wordmark on every thumbnail corner; solves the name collision and the "polished AI art" fatigue the Q1-2026 doc reports (reaction toward darker, historical aesthetics) | ✓ art; ○ TikTok doc |
| Voice split | **The Oracle** (synthetic, in-character, MAXIMUM CHAOS when the format calls for it) + **Apocky** (human narrator/founder, "the sysadmin of the temple") — the two-voice structure is the anti-inauthentic-content design | ◐ from §0.3 |
| One-line promise per video | "One question. One spread. One conspiracy board." | ◐ |
| What we never are | a psychic hotline; a love-reading channel; a medical/financial/legal oracle; a "predict the news" channel | ✓ site framing |

**O:** viewer-language test at day 30: ≥ 30% of top comments use our words ("glitch", "signal", "the Oracle", a card's shipped name). **F:** comments treat the videos as generic pick-a-card readings ("which pile is mine?") and nobody names a card.

---

## 3. Content pillars

Anchors, as asked: the eight systems (site), the Codex lore, the MAXIMUM CHAOS voice, the daily-draw engine.

| Pillar | Source of truth | Long-form shape | Shorts shape | Purpose | Share by phase (P1→P3) |
|--------|----------------|-----------------|--------------|---------|-----------------------|
| **P1 Daily Oracle** (daily-draw engine) | ✓ https://chaos-tarot.com/daily-oracle (all 8 systems, dated) | Weekly "Week Ahead" using the site's Week Ahead template (3 positions) | 1 Short/day, 20–45 s: the day's tarot card **plus one rotating system** ("Sun of Corruption (Reversed) + Tiwaz (Reversed): the Oracle's take"); temporal hashtag (#September2026) per the 3-6 rule | daily habit; the direct funnel to the free tier | 45% → 35% → 30% of Shorts |
| **P2 MAXIMUM CHAOS readings** | ✓ `docs/CHAOS_ORACLE_VIBE.md` | 8–12 min: a viewer-submitted question (comments/community post) read in Eye of the Storm (Codex §5.1, 7 positions) with the conspiracy board escalation | 60–120 s clips of the escalation peak ("16 minus 5 is 11…") | the clip engine; brand voice; comments | 25% of long-form; 30% of Shorts |
| **P3 The Codex: card lore** | ✓ Codex §2, §4; shipped names from `New/` | 6–10 min per card or per pair: shipped title on screen ("16 Tower of Obsolescence"), Codex lore as "in the Codex this card is called The Critical Error", Signal and Glitch readings, tech correspondence from the guidelines (Pentacles = Hardware layer, etc.) | 30–60 s "one card, one glitch" | evergreen search on 79 unique names; the deck's sales page in video form | 30% of long-form |
| **P4 The Eight Systems** | ✓ site guides (runes, I Ching, Ogham, Lenormand, Geomancy, Astrology), app | 8–15 min explainers and **cross-system readings** ("the same question through tarot, runes, and I Ching") — the feature no competitor has | 45–90 s "one rune, one hexagram" | search + differentiation + trial driver (AI interpretations "tuned to each divination system") | 20% of long-form |
| **P5 Shadow Work: Debugging the Soul** | ✓ Codex §7.0 System Purge exercise; Shadow Work Deep Dive template (10 positions) | 10–15 min guided sessions; 7-day and 31-day #TarotChallenge journaling series (Q2 "Divine Order") | 30–60 s journaling prompts ("If this card had a voice, what would it scream at me?") | retention + Oracle sub (journal, Memory Core, PDF export are paid) | 15% of long-form |
| **P6 Build-in-public / Technomancy** | ✓ guidelines "Dual-Boot Philosophy"; deck spec (350gsm "Void Matte", holographic black foil edges); Kickstarter | 8–12 min: how the Oracle is prompted, deck print proofs, Kickstarter prep, "why 79 cards" | 30–60 s proof-unboxing, foil edge macro | founder authenticity (inauthentic-content insurance), deck + Kickstarter | 10% of long-form; spikes at campaign |
| **P7 The Game of Madness** (optional) | ✓ Codex §6 Sanity Check, Wild Magic Surges | 8–12 min GM guides | 30–60 s "your natural 1 drew the Tower" | GM/streamer audience; physical deck | 0% → 5% → 10% |

Design rules that hold across pillars (◐ from §0.3): every long-form has a real question and a live app capture; the Oracle voice is scripted from the persona prompt, never generic; Apocky's voice appears in every long-form; Singularities are always "from the Codex" (A8); the disclaimer sentence closes every description.

**O (pillar mix):** by day 60, P3 or P4 videos are ≥ 40% of search-sourced views and P2 Shorts are ≥ 40% of Shorts-feed views. **F:** search traffic < 10% of views by day 60 (the evergreen bet failed) or P2 Shorts underperform P1 by ≥ 2× on views per Short (the voice is not clip-shaped).

---

## 4. Upload strategy

| Phase | Weeks | Long-form / week | Shorts / week | Why | Tag |
|-------|-------|------------------|---------------|-----|-----|
| 0 Setup | 0 (Sept 28–Oct 4) | 0 published, 3 in the can | 0 published, 10 in the can | launch with a library so the first-week binge has depth | ◐ |
| 1 Launch | 1–4 | 2 | 5–6 (one daily P1 + P2/P3 clips) | long-form banks hours; daily Short builds the habit | ◐ from §0.1 |
| 2 Compound | 5–8 | 2 | 6–7 | double down on the winning pillar (§7 review) | ◐ |
| 3 Apply | 9–13 | 2 (+1 P6 Kickstarter/deck if dated) | 5 | protect hours; apply when gates pass (§10) | ◐ |

Rules:
- Long-form publishes on fixed days (Tue + Sat, ⊘ until analytics say otherwise); Shorts daily at the same hour, keyed to the Daily Oracle date so they are "today's" content.
- Batch: record 2 long-form + 7 Shorts in one production block; publish scheduled.
- The "2 long-form or 5 Shorts per 90 days" activity minimum (○ §0.1) is exceeded by every phase.
- Throughput gate (A5): if a week ends with < 1 long-form and < 4 Shorts published, next week uses the 4-hour schedule (§9.2) rather than skipping.

**O:** ≥ 22 long-form and ≥ 60 Shorts published by day 90. **F:** < 16 long-form by day 90 (the hours target is out of reach regardless of quality).

---

## 5. Video formats

| Format | Spec | Where it fits | Threshold / policy | Tag |
|--------|------|---------------|--------------------|-----|
| **Long-form reading / lore** | 16:9, 8–15 min, chapters, 4K crops of the 825×1425 card PNGs, app capture, two voices | P2–P7; the watch-hour engine | none to publish; description links allowed | ✓ assets; ◐ |
| **Shorts** | 9:16, 20 s–3 min, loop-able ending, first frame = card art, title burned in, "related video" link to the long-form | P1 daily, P2 clips, P3/P4 one-card cuts | ○ ≤ 3 min rule https://support.google.com/youtube/answer/15424877; Shorts views do not count toward 4,000 h | ○ |
| **Live** | "Ask the Oracle" live reading | not before 1,000 subs | ○ mobile live needs 50 subs and under 1,000 subs viewers "may" be limited and archives default private (https://support.google.com/youtube/answer/9228390); desktop live has no subscriber gate but archives would be the same watch-hour asset either way | ○; ◐ defer to Phase 3+ |
| **Premiere** | long-form P2 with live chat | Phase 2+ for the weekly reading | no gate | ⊘ |
| **Community posts** | polls: "which card should the Oracle read next?", question intake for P2 | Phase 1+ | ⊘ availability at 0 subs not re-verified | ⊘ |
| **Playlists** | one per pillar + "All 79 cards" + one per system | Phase 0 | none | ◐ |
| **Podcast-style audio** | not now | — | — | ⊘ |

Verdict on "live?": not in the 90-day window. Live archives can bank hours later, but the sub gate, viewer limits and production time (A5) make it a Phase 3+ tool. **O:** first live at ≥ 1,000 subs draws ≥ 50 concurrent viewers. **F:** a live before 1,000 subs is viewer-capped or its archive is private by default, wasting the hours.

---

## 6. Growth systems

| System | Mechanism | Cost | Tag | O / F |
|--------|-----------|------|-----|-------|
| **G1 Evergreen search on unique names** | 79 shipped titles + 6 system guides = ≥ 85 searchable, zero-competition titles ("Tower of Obsolescence meaning", "Hermit of the Void tarot"); plus high-volume neighbours ("Tower tarot meaning" with the cyberpunk angle) | time | ◐ | O: ≥ 20% of views from YouTube search by day 90 · F: < 5% |
| **G2 Shorts → long-form bridge** | every Short links its long-form; Shorts are cut *from* long-form, so the payoff exists | none | ◐ | O: ≥ 3% of Short viewers click through · F: < 0.5% |
| **G3 Site → channel loop** | add a YouTube link to chaos-tarot.com footer and Daily Oracle page (✓ none exist today); embed the day's Short on `/daily-oracle`; email the day's Short via Resend to free-tier users (product requests) | dev time | ✓ gap; ⊘ build | O: ≥ 5% of channel traffic "external: chaos-tarot.com" by day 60 · F: 0 external traffic after the links ship |
| **G4 Temporal anchoring** | schedule content to the Cosmic Calendar (full moons, eclipses, solstices already listed on the site) and to month hashtags; the Q1-2026 doc says temporal beats evergreen framing on TikTok | none | ✓ calendar; ○ doc | O: event-day videos ≥ 1.5× median views · F: no lift |
| **G5 Cross-post** | same Shorts to TikTok and Instagram Reels using the 3-6 hashtag rule; watermark-free exports | none | ○ doc | O: ≥ 10% of site YouTube-tagged sessions originate from TikTok/IG by day 90 · F: cross-posts < 100 views median |
| **G6 Hype** | once in YPP with ≥ 500 subs, ask in every long-form for a hype in the first 7 days (Shorts not eligible) | none | ○ https://support.google.com/youtube/answer/15509925 | O: appear on a regional Hype leaderboard once · F: never |
| **G7 Collab** | GM/TTRPG channels (P7) and one mid-size tarot creator per month for a cross-system reading | time | ⊘ | O: 1 collab by day 60 yields ≥ 100 subs · F: none agreed |
| **G8 Kickstarter pre-launch list** | "notify me" landing + email list; P6 videos push it | Resend exists | ✓ tool; ⊘ campaign timing | O: ≥ 300 emails before launch · F: < 50 |
| **G9 Comment intake** | P2 reads viewer questions; pinned comment + community poll intake; reply to every comment in week 1–4 | time | ◐ | O: ≥ 5 usable questions/week by week 4 · F: none |
| **G10 The CDPR wave** | ride "cyberpunk tarot" search with comparison/context content, no CDPR marks in titles or thumbnails | none | ○ §1.1 | as §1.1 |

---

## 7. Retention framework

| Element | Rule | Metric target (⊘ until data) | Tag |
|---------|------|-------------------------------|-----|
| **0–5 s** | open on the card flip or the escalation line, never on a logo; title question restated | 30-s retention ≥ 70% (long-form), ≥ 80% (Shorts) | ◐ |
| **Structure: Pepe Silvia** | coherent → connections → escalation → reversal ("The Tower isn't your life collapsing. It's THE WALL COMING DOWN") → button. The vibe doc's own example is the template | AVD ≥ 40% on 8–12 min | ✓ vibe doc; ⊘ target |
| **Open loop** | "the seventh card changes everything" stated at card 1 of Eye of the Storm; paid at card 7 | dip at mid-video ≤ 15 points | ◐ |
| **Two-voice rhythm** | Oracle escalates, Apocky grounds; alternate every 45–90 s | — | ◐ |
| **Pattern interrupts** | card zoom, glitch cut, on-screen "evidence" (the red string), app capture | — | ◐ |
| **Chapters** | one per position; searchable, skippable | — | ◐ |
| **Ending** | no outro card; hard cut to the next video's card ("the Oracle has a note about the Moon of Illusion…") | end-screen click ≥ 5% | ◐ |
| **Shorts loop** | last frame = first frame (the card back → the card) | swipe-away < 30% in first 3 s | ◐ |
| **Disclaimer placement** | in description and a 1-s lower third, never as a spoken intro | — | ✓ site copy |
| **Weekly review** | Studio → retention graphs for the 2 long-form; mark the first dip; fix the same structural spot next week | — | ◐ (section 7's job to formalise) |

**O:** by week 8, median long-form AVD ≥ 4 min and Shorts average % viewed ≥ 90%. **F:** AVD stuck < 2.5 min at week 8, which also kills G1 (4,000 h would need > 96,000 views).

---

## 8. Monetization plan

### 8.1 Stages

| Stage | Trigger | Revenue lines | Tag |
|-------|---------|---------------|-----|
| **S0 Pre-YPP (day 0–90)** | launch | none from YouTube. Funnel only: description link → chaos-tarot.com free tier → **Oracle $3.33/month, 7-day free trial, "cancel anytime in Settings"** (✓ https://chaos-tarot.com/pricing) · **tokens "$1.99 for 5 tokens", "15 tokens $4.99"** (✓ same page) · **physical deck** "79 professionally printed cards … Order now" (✓ site; price not shown on the pricing page, ⊘) | ✓ |
| **S1 Fan-funding tier** | ○ 500 subs + 3,000 h or 3M Shorts views (unverified on the official page) | memberships (an "Oracle Circle" mirror of the $3.33 tier is a *bad* idea: it splits the funnel; use memberships only for early-access videos), Super Thanks | ○ |
| **S2 YPP** | G1 | ads on long-form (RPM: no tarot-specific figure found; all-niche median RPM ≈ $2.30, Education ≈ $10.22 per ○ https://air.io/en/air-data-findings/which-youtube-niche-makes-the-most-money-in-2026-ranked-by-real-rpm-and-cpm; treat tarot RPM as **unverified**); Shorts revenue only if ≥ 10M views/90 d after 2027-02-01 (○ §0.1), which is out of scope | ○ |
| **S3 Deck + Kickstarter** | campaign date (⊘) | P6 launch block: 1 trailer long-form, 1 unboxing/proof long-form, daily Shorts for the campaign's first 72 h and last 48 h; comparables: Endless Tarot $56,525 / 682 backers, Tarot of Satan $49,297 / 452, Spellbound CA$29,536 / 277, Essentia $8,857 / 122 (○ https://www.kickstarter.com/projects/1799389722/the-endless-tarot-deck · https://www.kickstarter.com/projects/travismchenry/tarot-of-satan · https://www.kickstarter.com/projects/annguyenart/spellbound-tarot-deck · https://www.kickstarter.com/projects/leedesignstudio/the-essentia-modern-tarot-deck) | ○ |
| **S4 Later** | ≥ 10k subs | sponsorships, affiliates (deck stock, journals), courses — section 6's scope | ⊘ |

### 8.2 Funnel arithmetic (every rate ⊘; shown so Apocky can replace each number)

| Step | Day-90 monthly run-rate | Rate |
|------|------------------------|------|
| YouTube views/month | 30,000 | target |
| Clicks to site | 450 | 1.5% |
| Free accounts | 112 | 25% |
| Oracle trials started | 11 | 10% |
| Paid after trial | 6 | 50% → **≈ $20/month** at $3.33 |
| Token packs | 5 | 5% of free → **≈ $10–25/month** |
| Deck orders | 2–4 | 0.5–1% of clicks × deck price (⊘) |
| Ads (post-YPP only) | 30,000 × $2.30/1,000 ≈ **$69/month** (○ median RPM, tarot unverified) | |

◐ Read: at day-90 scale YouTube ads and subscriptions are tens of dollars; the **deck and the Kickstarter are the money**, and the channel's 90-day job is (a) YPP status before the bar doubles and (b) an audience and email list that exists on campaign day. **O:** by day 90, Stripe shows ≥ 10 Oracle subs and ≥ 20 deck orders with a `utm_source=youtube` (or equivalent) attribution. **F:** < 3 attributed conversions of any kind by day 90 despite ≥ 20,000 views (the funnel, not the content, is broken).

### 8.3 Funnel plumbing (product requests, not facts)

- UTM-tagged links per video (`?utm_source=youtube&utm_medium=video&utm_campaign=<pillar>`); Supabase attribution on signup. ⊘ not built.
- A `/yt` landing page: today's oracle + "Start a Free Reading" (the CTA already on `/daily-oracle`, ✓). ⊘ not built.
- Fix `lib/social-share.js` handle (✓ defect, §1.3).
- Add YouTube (and TikTok/IG once live) links to the site footer (✓ none today).

---

## 9. Weekly execution schedule

### 9.1 Primary: 8–12 h/week (A5)

| Day | Block | Hours | Output |
|-----|-------|-------|--------|
| Mon | **Research + scripts**: pull the week's Daily Oracle draws and Cosmic Calendar events; pick 2 long-form topics from the pillar queue; generate Oracle scripts from the persona prompt; write Apocky's grounding lines; pick viewer questions from comments | 2.0 | 2 long-form scripts, 7 Short scripts |
| Tue | **Record**: both long-form voice tracks (Apocky) + Oracle TTS; app captures | 2.0 | raw tracks; publish long-form #1 (scheduled from last week's batch) |
| Wed | **Edit long-form #1 + #2** (templated timeline: card reveals, chapters, lower-third disclaimer); thumbnails from the 825×1425 PNGs | 2.5 | 2 long-form ready |
| Thu | **Shorts cut**: 7 Shorts (daily P1 ×5, P2 clip ×1, P3/P4 cut ×1); schedule daily; cross-post to TikTok/IG | 1.5 | 7 Shorts scheduled |
| Fri | **Publish + community**: long-form #2 goes live Sat (scheduled); community poll; reply to all comments; question intake | 1.0 | — |
| Sat | **Analytics 30 min** (section 7 loop): retention dips, CTR, traffic sources; pick next week's doubled-down pillar | 0.5 | one decision logged in the tracker |
| Sun | buffer / P6 build-in-public capture / Kickstarter prep when dated | 0–2.5 | optional |
| **Total** | | **9.5 (8–12)** | 2 long-form, 7 Shorts, 1 review |

### 9.2 Alternate: 4 h/week

| Day | Block | Hours | Output |
|-----|-------|-------|--------|
| Mon | scripts for 1 long-form + 5 Shorts (agent-generated drafts, Apocky edits) | 1.0 | |
| Wed | record + edit 1 long-form (P3 card lore: fixed template, 6–8 min, Oracle voice + one Apocky segment) | 2.0 | 1 long-form |
| Thu | cut 5 P1 Shorts from the Daily Oracle page + the long-form; schedule | 0.75 | 5 Shorts |
| Sat | analytics + comments | 0.25 | |
| **Total** | | **4.0** | 1 long-form, 5 Shorts |

◐ Consequence: at 4 h/week the December application is very unlikely (≈ 12 long-form by day 90); the goal becomes G1-fallback (bank hours toward the 2027 bar) and G3 (deck/Kickstarter list). State this in the tracker so the miss is a plan, not a surprise.

### 9.3 Automation hooks (for section 5 to build; listed so the hours above are honest)

- Daily Oracle → Short-frame generator (extend `shareable-image.js` to 1080×1920 with the day's 8 draws). ⊘
- Persona prompt → Oracle script → TTS → captions, one command. ⊘
- Comment scrape → question queue for P2. ⊘
- Studio analytics export → weekly Markdown review in this repo. ⊘

**O (schedule):** 10 of 13 weeks hit the week's output line without exceeding 12 h. **F:** ≥ 4 weeks over 12 h or ≥ 4 weeks under the output line (the format is mis-sized; cut long-form length before cutting cadence).

---

## 10. 90-day roadmap with gates

| Phase | Dates (2026) | Deliverables | Gate to next phase | Tag |
|-------|-------------|--------------|--------------------|-----|
| **0 Setup** | Sept 28 – Oct 4 | handle decision (§2); channel art with the Null card; 2-Step Verification + phone verification for advanced features (○ https://support.google.com/youtube/answer/9891124); AdSense for YouTube; playlists; description template with disclaimer + UTM link; 3 long-form + 10 Shorts in the can; site footer YouTube link; social-share handle fix | assets exist; channel public | ○/◐ |
| **1 Launch** | Oct 5 – Nov 1 | 8 long-form, 22 Shorts; first Cosmic Calendar event video; comment intake running | ≥ 200 subs **and** ≥ 400 watch hours by Nov 1 → continue to Phase 2 at full cadence; below → keep cadence but switch the long-form mix to 50% P3 search content | ◐ |
| **2 Compound** | Nov 2 – Nov 29 | 8 long-form, 26 Shorts; 1 collab; CDPR-wave content; Kickstarter list live | ≥ 600 subs **and** ≥ 1,800 h by Nov 29 → Phase 3 sprint; below → declare G1-fallback, keep publishing, move Kickstarter/deck content forward | ◐ |
| **3 Apply** | Nov 30 – Dec 27 | 8 long-form, 20 Shorts; **apply the day 1,000 subs + 4,000 h both show in Studio, no later than Dec 27**; December solstice video | application submitted by Dec 27 | ◐ from §0.1 |
| **Post** | Dec 28 – Jan 31 | keep cadence during review; Hype asks once accepted; first live at ≥ 1,000 subs | accepted before Feb 1, 2027 | ○ review ≈ 1 month |

**O (roadmap):** accepted into YPP by 2027-01-31 under the 4,000-hour bar. **F:** application not submitted by Dec 27, or rejected; the fallback's own oracle is ≥ 2,000 banked hours on Dec 31 (a quarter of the 2027 bar, all of it still inside the 365-day window).

---

## Open questions for Apocky

1. Handle: `@chaos-tarot` or `@thechaostarot`? Any prior contact with chaostarot.com / `@chaostarot` (they hold the YouTube, TikTok and X handles)?
2. Does Apocky own `x.com/chaostarot`? If not, `lib/social-share.js` line 58 credits a stranger; which handle should it use?
3. Kickstarter date, even approximate. It decides whether Phase 2 or 3 carries the P6 launch block.
4. Physical deck price and stock (not on the pricing page). Needed for the funnel table and for whether P6 pushes "order now" or "notify me".
5. Voice: will Apocky narrate (own voice, no disclosure needed) or clone their own voice (also no disclosure needed per the ○ page)? Either is fine; a fully synthetic non-Apocky voice for *Apocky's* segments is the one option this plan advises against.
6. Confirm the two publish days (Tue/Sat is a placeholder) and the daily Short hour.
7. Are the 10 Singularities going to be printed (A8)? If yes, P3 gets 10 more evergreen videos and the "88 cards" story returns.
8. Is the 4 h/week schedule the realistic one? If so the December application target should be dropped now, not discovered in November.

## Issue candidates

| Title | Phase | Priority | Acceptance oracle | Falsifier | Depends on |
|-------|-------|----------|-------------------|-----------|------------|
| Decide and register channel handle; document the name collision | 0 | P0 | channel live at chosen handle; collision note in tracker | handle taken at registration time | Q1 |
| Fix `lib/social-share.js` Twitter handle | 0 | P0 | shared readings credit an Apocky-owned handle or none | still `@ChaosTarot` after fix | Q2 |
| Add YouTube link to chaos-tarot.com footer and `/daily-oracle` | 0 | P0 | link visible on prod; "external" traffic source appears in Studio | no external traffic by day 30 | handle |
| Channel setup: 2SV, phone verification, advanced features, AdSense, playlists, description template with disclaimer + UTM | 0 | P0 | Studio shows advanced features on; AdSense linked | any item missing at application time | — |
| Produce launch library: 3 long-form + 10 Shorts | 0 | P0 | files scheduled in Studio before Oct 5 | < 2 long-form at launch | scripts |
| Oracle script generator from the persona prompt (two-voice template) | 0–1 | P1 | one command yields a script that passes a 5-point voice checklist | scripts read as generic TTS | section 5 |
| Daily Oracle → 1080×1920 Short-frame generator (extend `shareable-image.js`) | 1 | P1 | daily Short built in < 10 min | manual build > 20 min/day | section 5 |
| UTM attribution on signup in Supabase; `/yt` landing | 1 | P1 | `utm_source=youtube` visible on ≥ 1 signup | no attributed signups by day 30 | — |
| P3 evergreen series: 22 Majors + Null first, then the four suits with shipped names | 1–3 | P1 | ≥ 12 P3 videos by day 90; ≥ 20% search traffic | search < 5% at day 90 | assets |
| P2 weekly MAXIMUM CHAOS reading with comment intake | 1–3 | P1 | ≥ 10 episodes; P2 Shorts ≥ 40% of Shorts-feed views | P2 Shorts underperform P1 by 2× | comments |
| Cosmic Calendar content schedule for Oct–Jan events | 1 | P2 | event videos published on event day | ≥ 2 events missed | site calendar |
| Cross-post pipeline to TikTok/IG with the 3-6 hashtag rule | 1 | P2 | daily Short mirrored within 24 h | median cross-post < 100 views by day 60 | Shorts generator |
| Kickstarter pre-launch email list via Resend + P6 video block | 2 | P1 | ≥ 300 emails before launch | < 50 | Q3, Q4 |
| Weekly analytics review template in repo (section 7 to define) | 1 | P2 | 12 weekly entries by day 90 | < 8 entries | section 7 |
| YPP application gate check (1,000 subs + 4,000 h) and submit by Dec 27 | 3 | P0 | application submitted; acceptance before Feb 1 | not submitted / rejected | all above |
| G1-fallback declaration if Nov 29 gate fails | 2 | P1 | tracker records the decision and the 2,000-h target | miss discovered only in January | gate |
| Disclosure checkbox + disclaimer lower-third in the publish checklist | 0 | P1 | every video shows the "How this content was made" label where synthetic voice/music is used | YouTube applies a label Apocky did not set | section 5 checklist |
| Live-stream plan deferred to ≥ 1,000 subs | 3+ | P3 | first live at ≥ 50 concurrent viewers | live attempted before 1,000 subs is viewer-capped | YPP |

## CSL annex

Σ: 8-system oracle + 79-name deck + MAXIMUM CHAOS voice → long-form banks 4,000 h for a Dec-27 YPP application (bar doubles 2027-02-01); Shorts = discovery; deck + Kickstarter = the money.
W! apply ≤ 2026-12-27 · 2 long-form + 5–7 Shorts/week · shipped card names on screen, Codex names as lore · Apocky voice in every long-form · disclosure box ticked · UTM on every link · handle ≠ @chaostarot · fix social-share handle.
N! no medical/legal/financial/love-drama angles · no templated slideshow Shorts · no CDPR marks in titles/thumbs · no live before 1,000 subs · no membership tier that competes with the $3.33 Oracle · no thresholds from memory (○ URLs in §0).
