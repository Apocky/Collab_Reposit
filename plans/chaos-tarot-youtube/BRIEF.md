# Chaos Tarot YouTube channel — Brief

Frame: plan a YouTube channel for Chaos Tarot (chaos-tarot.com) that reaches YouTube Partner Program eligibility and feeds the product funnel, executed through the eight-prompt pipeline in `prompts/SOURCE_PROMPTS.md`.
Repo: Apocky/Collab_Reposit @ `claude/chaos-tarot-youtube-plan-2qtrui` · Plan dir: `plans/chaos-tarot-youtube/` · Date: 2026-09-27
Product repo (read-only reference, cloned at `/home/user/the-chaos-tarot`): Apocky/The-Chaos-Tarot @ main, last commit "Consolidate to single React app for production" (pushed 2026-09-27).

Tags: ✓ VERIFIED this session · ◐ INFERRED · ○ REPORTED (source named) · ⊘ ASSUMED (needs Apocky).

## 1. What Chaos Tarot is

- ✓ Live site chaos-tarot.com, fetched 2026-09-27. Headline copy: **"Chaos Tarot — Digital Divination Platform"**, tagline **"Eight divination systems. One digital oracle."**
- ✓ Systems on the site: a 79-card cyberpunk tarot deck, Elder Futhark runes (24), I Ching hexagrams (64), Ogham (25), Petit Lenormand (36), Geomancy (16), Astrology with natal charts.
- ✓ Site features: spreads including Celtic Cross and Norn Spread; reversals and merkstave; "AI-powered interpretations for every spread, tuned to each divination system"; AI Oracle with a distinct persona per tradition; **Memory Core** AI that remembers past readings; reading journal; daily draws with streak tracking across all systems; follow-up questions; collection tracker; community gallery; cross-system combined readings; PDF export.
- ✓ Pricing on the site: **Free tier** (all eight systems, spreads, daily draws, streaks, collection tracker, community). **Oracle subscription $3.33/month, or $2.22/month "with CSL"** for unlimited AI interpretations, Memory Core, follow-ups, journal, PDF export.
- ✓ Physical product on the site: "79 professionally printed cards with original cyberpunk artwork", sold via Stripe, ships within the US.
- ✓ Site navigation: Try Free, View Pricing, Enter Access Code. Footer: card guides (runes, I Ching, Ogham, Lenormand, Geomancy, Astrology), Articles, Daily Oracle, Cosmic Calendar, Terms, Privacy, Ethics.
- ✓ **No YouTube, social, or app links anywhere on the site.**
- ✓ Site disclaimer: "Chaos Tarot is for entertainment and personal reflection only. Readings are not a substitute for professional medical, legal, financial, or psychological advice."
- ✓ Repo `package.json` description: "The Technomantic Grimoire - A Digital Temple for Algorithmic Divination", version 2.0.0.
- ✓ Stack: React 18 + Vite + TypeScript, Zustand, React Query, Framer Motion; Supabase (project "The Chaos Tarot", ref `nqfpeprkiclyazoefjvz`, us-west-2, healthy); Vercel project `chaos-tarot`; Stripe; Resend email; Sentry org `chaos-n9`; AI providers OpenAI, Anthropic, Gemini with fallback. Edge functions `generate-reading`, `interpret-reading`, `export-reading`.
- ✓ Repo already contains `lib/social-share.js`, `lib/share-utils.js`, `app/core/shareable-image.js` (shareable reading images exist as a feature).
- ✓ Repo docs (~40 files in `docs/`) include `AI_ORACLE_TRAINING_GUIDELINES.md`, `CHAOS_ORACLE_VIBE.md`, `FEATURE_LIST.md`, `PREMIUM_SETUP.md`, `IMPROVEMENT_IDEAS_100.md`, `SIGILCRAFT_GUIDE.md`, `CANVAS_SPREAD_SYSTEM.md`.

## 2. Brand voice (quotes)

From `book/The Chaos Codex_ A Manual for the Entropy Deck.md` (✓ in repo; also a Google Doc and a printed booklet PDF in Drive):

> "The universe does not speak in clear, linear sentences; it screams in white noise, decaying atoms, and the collision of galaxies. The Chaos Tarot is not a tool designed for those seeking comfortable answers or the polite fiction of predetermined destiny. It is an artifact for the 'fearless oracle,' a navigational instrument calibrated for the storm that constitutes modern reality."

> "This deck is a compass for the Modern Mystic, the Visual Philosopher, and the Shadow Worker who understands that wisdom is often found not in the pattern, but in the break of the pattern."

- Mechanic: **Signal** (upright) and **Glitch** (reversed) replace upright/reversed. "A Glitched card is not merely 'blocked'… It represents the archetype in a state of high entropy."
- Suits renamed in the Codex manuscript: **Vectors** (Wands), **Frequencies** (Cups), **Signals** (Swords), **Artifacts** (Pentacles). **The shipped card art uses different suit names: Codes, Networks, Signals, Vectors** (✓ `the-chaos-tarot/New/` filenames, e.g. "Ace of Codes", "Queen of Networks", "Ten of Signals", "King of Vectors"). On-screen content follows the art; the Codex names are lore.
- ✓ Shipped Major Arcana titles from the art files: 0 The Zero Point · 1 The Singularity · 2 Oracle of Static · 3 Mother of Paradox · 4 Lord of Entropy · 5 High Priest of Glitch · 6 Binary Schism · 7 Juggernaut of Unreason · 8 Scales of Chaos · 9 Hermit of the Void · 10 Wheel of Misfortune · 11 Fortitude of Fragmentation · 12 Hanged Man of Hyperreality · 13 Death the System Crash · 14 Alchemist of Buffering · 15 The Devil's Algorithm · 16 Tower of Obsolescence · 17 Star of Fragmentation · 18 Moon of Illusion · 19 Sun of Corruption · 20 Judgement of the Glitch · 21 World of the Glitch · **Null — The Apockalypse** (the 79th card; a pun on Apocky, unique to this deck). There is also a "Page of Fractures" file and a `deck/Apocalypse.jpg`.
- Major Arcana re-skinned in the Codex manuscript (lore names, differ from the art titles above): The Fool = The Anomaly; Magician = The Operator; High Priestess = The Black Box; Empress = The Genesis Node; Emperor = The Firewall; Hierophant = The Protocol; Lovers = The Binary Pair; Chariot = The Drive; Strength = The Bandwidth; Hermit = The Offline Mode; Wheel = The RNG; Justice = The Algorithm; Hanged Man = The Suspension; Death = The System Purge; Temperance = The Synthesis; Devil = The Malware; Tower = The Critical Error; Star = The Restoration; Moon = The Illusion; Sun = The Clarity; Judgment = The Audit; World = The Complete Loop.
- Codex section 3.0 "The Singularities: The Oracle of Entropy" describes 10 oracle cards beyond the 78. Section 6.0 "The Game of Madness: TTRPG Integration" (Sanity Check mechanic, Wild Magic Surges). Section 7.0 "Shadow Work: Debugging the Soul".
- Physical spec in the Codex: 350gsm "Void Matte" stock, holographic black foil edges, rigid magnetic-clasp box with foil-stamped fractal geometry.

From `docs/AI_ORACLE_TRAINING_GUIDELINES.md` (✓):

> "Dual-Boot Philosophy: The agent must run on the rigorous logic of a sysadmin while interpreting data through the fluid, symbolic lens of a high priestess."

- Chaos magic as meta-system ("Nothing is true; everything is permitted"); the glitch as "holy interruption"; tech correspondences: Pentacles = Hardware layer, Swords = Network layer, Wands = Application layer, Cups = UX layer. Cyberpunk re-skins: Fool = The Edgerunner, Magician = The Hacker, High Priestess = The Netrunner, Tower = System Crash, Death = System Migration.

From `docs/CHAOS_ORACLE_VIBE.md` (✓): **MAXIMUM CHAOS** oracle mode is Charlie Kelly at the Pepe Silvia conspiracy board: red string everywhere, escalating revelation, manic energy, "questionable but weirdly compelling logic". Example reading in the doc ("The Tower is card SIXTEEN. The Five? That's FIVE. 16 minus 5 is 11…"). This is a distinct, highly clippable voice.

## 3. Assets that exist

| Asset | Where | Tag |
|-------|-------|-----|
| Card art JPGs with Chaos titles (e.g. "0 The Zero Point", "1 The Singularity", "2 Oracle of Static", "3 Mother of Paradox", "4 Lord of Entropy", "10 Wheel of Misfortune", "13 Death the System Crash", "15 The Devil's Algorithm", "16 Tower of Obsolescence", "20 Judgement of the Glitch", "21 World of the Glitch") | `the-chaos-tarot/New/` | ✓ |
| Full deck images (Major + Minor), card back, manifest | `the-chaos-tarot/deck/` | ✓ |
| Mystical UI assets, celestial icons, PWA icons | `the-chaos-tarot/public/assets/`, `public/icons/` | ✓ |
| 20 named reading templates (General, Career, Relationships, Self Discovery, Timing, Decisions, Daily Guidance, Past/Present/Future, Situation·Action·Outcome, Career Crossroads, Relationship Clarity, Love Potential, Shadow Work Deep Dive, Yes or No Clarity, Two Paths, Week Ahead, Monthly Cycle, Mind·Body·Spirit, Creative Unblocking, Free Draw) | `the-chaos-tarot/shared/reading-templates.json` | ✓ |
| 10 Singularity oracle cards defined in the Codex (S-1 The Event Horizon, S-2 The Heat Death, S-3 The Strange Attractor, S-4 The Vacuum Decay, S-5 The Observer, S-6 Dark Matter, S-7 The Multiverse, …) with concept + meaning text | `book/The Chaos Codex…md` §3.0 | ✓ text; ⊘ whether printed |
| The Chaos Codex manuscript (Markdown, Google Doc, printed booklet PDF) | repo `book/`; Drive folder "The Chaos Tarot" | ✓ |
| "AI Tarot Oracle Training Guidelines" (PDF, RTF, Google Doc) | repo root; Drive | ✓ |
| "Esoteric App Development Plan" / "The Technomantic Grimoire" (architecture + monetization "Occult Economy" section) | repo root; Drive | ✓ |
| Live app with 8 systems, AI Oracle, shareable images, PDF export | chaos-tarot.com | ✓ |
| Sibling product HALO (dice oracle / card roguelike) with its own Vercel project `halo-oracle` | Apocky/Collab_Reposit, halo repos | ✓ (not the focus) |

Two card-count claims coexist: site says 79 cards (✓), Codex says 88 = 78 + 10 Singularities (✓ text), README says 78 (✓, older). Treat **79 = 78 + 1** as the shipped physical count and the Singularities as Codex lore unless Apocky says otherwise (see contradictions).

## 4. Business context

- ✓ A Kickstarter project budget spreadsheet titled **"Project budget: Chaos Tarot: A Divination Platform plus Digital Intelligence"** was shared to Apocky by sheets@kickstarter.com on 2026-09-07 and last modified 2026-09-08. Only the template structure was readable; filled values were not read. ◐ A Kickstarter campaign is being prepared. Timing unknown (⊘).
- ✓ Vercel projects named `chaos-production-activation-20260908`, `chaos-live-2ef35d9`, `chaos-csl-deploy-20260907` were created 7–8 Sept 2026. ◐ Production activation happened in early September 2026; the product is live and recently consolidated.
- ○ Repo `docs/PREMIUM_SETUP.md` describes a "Premium Seeker" tier at $4.99/month or $39.99/year. The live site shows $3.33/month. Site wins; the doc is stale.
- ○ The "Esoteric App Development Plan" has a section "6.2 Monetization: The Occult Economy" (not read in full this session; agents may read it at `/home/user/the-chaos-tarot/Esoteric App Development Plan.md` lines ~247–278).
- ○ Sibling HALO product doc lists "streamers looking for table-friendly content" as a target audience and "streamer affiliate kits" as a live-ops item.

## 5. Audience research already on file

✓ Drive doc **"TikTok Hashtag Trends: Spirituality & Holidays"** (Apocky, 2026-01-19), a Q1 2026 analysis. Points that bear on YouTube:

- "Great Bifurcation": Radical Pragmatism ("Reali-Tea", #AdminNight, #LockedIn) vs Algorithmic Absurdism ("Italian Brainrot"). "Do not try to curate a perfect life. Either show the messy spreadsheet of your reality, or show us a crocodile fighting a goose. Anything in between is invisible."
- WitchTok 2026 = "the disciplined mystic": ritual as productivity, "Plan Like a Witch", #TarotChallenge (7-day / 31-day one-card-a-day journaling), a swing back toward darker, historical aesthetics (Norwegian Woodcut Tarot) as a reaction to polished AI art.
- Hashtag "3-6 rule": 1–2 broad, 2–3 category, 1–2 niche/temporal; temporal anchoring (#January2026) outperforms evergreen framing.
- Trend Matrix quadrants: Q1 Escapist Chaos (meme, high risk), Q2 Divine Order (tarot challenges, planners, niche authority). **Chaos Tarot's brand sits on the seam between Q1 and Q2**, which no competitor in the doc occupies.
- Daily tarot dynamics: creators tailor readings to the day's card energy; that is a daily-content engine the app already has (Daily Oracle, Cosmic Calendar).

## 6. Constraints

- Owner: Apocky, solo, technical, runs a large AI-agent infrastructure (apocrypha-core, mempalace, 3MNEME). Heavy automation is realistic; hours per week unknown (⊘).
- The source thread's ethos is "100% free"; budget preference is near-zero cash, AI-assisted production (⊘ confirm).
- The site is for "entertainment and personal reflection only"; the channel must carry the same framing.
- Tarot and divination content on YouTube faces ad-suitability and "inauthentic content" policy scrutiny; AI-voiced and mass-produced formats are the exact shape YouTube tightened rules on in 2025. Every writer must **verify the current YPP thresholds, the inauthentic-content policy, and AI-disclosure rules via web search with URLs** and tag them ○ with source. Do not state thresholds from memory.
- vidIQ connector exists on the account but is `connect_incomplete` (✓); it can become a research tool once connected.
- Memory rail Unirecall was unreachable this session (cloud container, no desktop link). Prior conversations about this channel, if any, are not in this brief.

## 7. Assumptions (⊘ until Apocky confirms)

| # | Assumption | If wrong |
|---|-----------|----------|
| A1 | No Chaos Tarot YouTube channel exists yet; the plan starts from zero. | If a channel exists, Phase 0 becomes an audit and the roadmap shortens. |
| A2 | Format is faceless-first: AI Oracle voice + card art + app screen capture, with optional on-camera founder segments later. | If on-camera from day one, production system and hook styles change materially. |
| A3 | Goal order: (1) YPP eligibility inside 90 days, (2) funnel to chaos-tarot.com free tier → Oracle subscription, (3) awareness for the physical deck and the Kickstarter. | If Kickstarter is imminent, the roadmap front-loads deck and campaign content over YPP mechanics. |
| A4 | Near-zero cash budget; existing AI API keys, free tiers, and self-hosted tooling. | A tools budget changes the production stack recommendations. |
| A5 | Apocky can commit roughly 8–12 hours/week to the channel. | Less time means a lower cadence; the roadmap needs an explicit throughput gate. |
| A6 | Tracking artifacts live as Markdown in this public repo. | Move to The-Chaos-Tarot (private) or GitHub Issues if preferred. |
| A7 | The 8th prompt of the thread is unknown; the plan covers prompts 1–7 and reserves a slot for 8. | Fold prompt 8 in when supplied. |
| A8 | The physical deck is 79 cards as the site states; the 10 Singularities are Codex lore, not shipped cards. | Content about "the Singularities" would need to be framed as digital-only or upcoming. |

## 8. Contradictions between sources

| Claim A | Claim B | Status |
|---------|---------|--------|
| Site: 79-card deck | Codex: 88 cards (78 + 10 Singularities); README: 78 | ◐ Resolved: 79 = 78 + "Null — The Apockalypse" (art file exists); Singularities are lore/digital unless Apocky says printed. See A8 |
| Codex suits: Vectors / Frequencies / Signals / Artifacts; Majors: The Anomaly, The Operator, The Black Box… | Art files: Codes / Networks / Signals / Vectors; Majors: The Zero Point, The Singularity, Oracle of Static… | ✓ Art is the shipped product; use art titles on screen, Codex names as lore |
| Site: Oracle $3.33/mo ($2.22 with CSL) | docs/PREMIUM_SETUP.md: $4.99/mo, $39.99/yr | Resolved: site is live truth |
| Thread: "hit monetization in 90 days" | YPP thresholds and review timelines (to be verified by writers) | Treat 90 days as horizon, not promise |

## 9. Open questions (only Apocky can answer)

See `QUESTIONS.md`. Headline items: prompt 8 text; existing channel or from zero; faceless vs on-camera; Kickstarter timing; hours per week and tools budget; where tracking should live; whether to run a Unirecall recall locally for prior channel decisions.

## 10. How writers use this brief

- Quote the site and Codex copy; do not invent product features. If a feature would be nice for the channel but does not exist, mark it as a product request, not a fact.
- Every YouTube threshold, policy, CPM figure, or tool price: web-search, cite the URL, tag ○. If not verifiable, say so.
- Every recommendation ends with an oracle (how we would know it worked) and a falsifier (what would show it failed).
- Write for Apocky: terse, tabular, tagged. English prose only where it carries meaning. A short CSL annex at the end of each section is welcome; it never replaces the evidence.
