# 6. YouTube Monetization Machine

## Source prompt

> "Act as a creator business strategist. Build a complete monetization system for my YouTube channel. Include YouTube Partner Program preparation, AdSense strategy, affiliate marketing, sponsorships, digital products, courses, memberships, newsletters, communities, and other revenue streams. Explain when each monetization method should be introduced based on channel growth stages.

(Verbatim from `prompts/SOURCE_PROMPTS.md` §6; no closing quote in the source.)

## Assumptions used

| # | Used as | Effect here |
|---|---------|-------------|
| A1 | Channel starts from zero | Every stream below has a metric gate measured from launch. |
| A2 | Faceless-first (Oracle voice + card art + screen capture; physical cards on camera as the human element) | Sponsorship and membership perks are designed without a face; on-camera later only raises rates. |
| A3 | Goal order: YPP → funnel to chaos-tarot.com → deck + Kickstarter | Stage order S0→S4 below. If the Kickstarter is imminent, S3 moves ahead of S2 (see §12). |
| A4 | Near-zero cash | Every tool chosen has a free tier or a per-sale fee; no monthly SaaS before it pays for itself. |
| A5 | 8–12 h/week | No stream that needs > 1 h/week of ops before it earns > $50/month. |
| A6 | Tracking in this repo | Issue candidates at the end. |
| A8 | 79 shipped cards; 10 Singularities are Codex lore | Singularities are proposed as a Kickstarter stretch goal / digital-only tier, never as shipped cards. |

Not used: A7.

**Provenance note (Apocky asked for repos and Unirecall to be re-checked).** ✓ GitHub search over `user:Apocky` (31 repos, 2026-09-27): the only "kickstarter" hit anywhere is a citation in `book/The Chaos Codex…md` (a Reddit link about the Deleted World Tarot). No Kickstarter page, prelaunch page or campaign copy exists in any repo; `Apocky/Chaos-Tarot` (private) was last updated 2025-12-30 and `Apocky/The-Chaos-Tarot` is the live product. ✓ Web search "Chaos Tarot" + Kickstarter (2026-09-27) finds no public campaign or prelaunch page. ✓ Unirecall is not exposed as a tool or connector in this session (ToolSearch and ListConnectors both checked; the only connector is vidIQ, still `connect_incomplete`). Prior channel decisions, if any, are still not in this plan.

Tag key: ✓ VERIFIED this session · ◐ INFERRED from ✓ premises · ○ REPORTED (source named, URL) · ⊘ ASSUMED. Every threshold, fee, rate and policy is ○ even when I fetched the official page; "fetched" means I read the page this session, "search result" means I saw only a search summary. Every recommendation ends with **O:** (oracle) and **F:** (falsifier).

---

## 0. Verification ledger (read before the stages)

### 0.1 YouTube program thresholds and shares

| Item | Value | Source |
|------|-------|--------|
| **Fan-funding / expanded YPP tier** | "500 subscribers with 3 valid public uploads in the last 90 days, and 3,000 qualified watch hours in the last 12 months" **or** "… 3 million qualified Shorts views in the last 90 days". Unlocks "Channel memberships", "Super Chat & Super Stickers", "Super Thanks", "Jewels and gifts" (live), "Shopping". Available in 80+ countries incl. the US. | ○ fetched https://support.google.com/youtube/answer/13429240 |
| **Ad-revenue tier (full YPP)** | "1,000 subscribers with 4,000 qualified watch hours in the last 12 months" or "1,000 subscribers with 10 million qualified Shorts views in the last 90 days"; enables "revenue sharing from ads and YouTube Premium" | ○ fetched https://support.google.com/youtube/answer/72851 and …/13429240 |
| **1 Feb 2027 wall** | "New creators applying for YPP will need 8,000 qualified watch hours in the last 365 days, or 20 million qualified Shorts views in the last 90 days" (+1,000 subs); "This update won't impact creators already in YPP"; "The entry thresholds for our Fan Funding and shopping products remain unchanged" | ○ fetched https://blog.youtube/news-and-events/youtube-partner-program-updates-2027-new-opportunities-earn/ (2026-08-10); ○ fetched https://support.google.com/youtube/answer/12843009 ("There are no changes to the eligibility requirements for fan funding") |
| Shorts ad revenue | Creators "keep 45% of their allocated revenue"; from 2027-02-01 a channel needs "at least 10 million qualified Shorts views in the last 90 days" each month to earn Shorts ad + Premium revenue | ○ fetched https://support.google.com/youtube/answer/12504220 |
| Revenue shares | Watch-page ads "55% of net revenues"; Shorts "45%"; "70% of net revenues from channel memberships, Super Chat, Super Stickers, and Super Thanks"; Shopping paid "directly from your official merch retailer or platform" | ○ fetched https://support.google.com/youtube/answer/72902 |
| Hype | "Channels in the YouTube Partner Program with 500 to 500,000 subscribers are eligible"; videos "published in the last 7 days"; Shorts not eligible; 38 countries | ○ fetched https://support.google.com/youtube/answer/15509925 |
| Shopping affiliate program | Requires YPP + "meets the subscriber threshold for YPP"; US eligible; not a music channel; not made-for-kids; each brand "will set their commission rates"; paid "within 60 to 120 days after purchase" | ○ fetched https://support.google.com/youtube/answer/13376398; expansion to the 500-sub tier ○ fetched https://blog.youtube/creator-and-artist-stories/youtube-shopping-expansion-500-subscribers/ (2026-03-25) |
| Shopping, own store | Needs a supported storefront platform (Shopify, Spring, Spreadshop and others) connected in Studio | ○ search result https://vidiq.com/blog/post/youtube-shopping/ and https://metricool.com/youtube-shopping/ ; official own-store page returned 404 this session → **unverified detail: which platforms** |
| Posts (community tab) | No subscriber threshold since 2023; needs Advanced features, good standing, not made-for-kids | ○ search result https://www.socialmediatoday.com/news/youtube-community-posts-all-channels/651083/ |
| Communities (Discord-like) | Opt-in for creators with Posts access; mobile-only for now | ○ search result https://ppc.land/youtube-unveils-new-communities-feature-to-enhance-fan-connections/ |
| AdSense payout | $100 threshold; paid 21st–26th of the following month | ○ search result https://support.google.com/adsense/answer/1709871 and https://support.google.com/youtube/answer/14727140 |
| Paid promotion | "selecting the paid promotion button in your video details" "adds a disclosure label"; creators must comply with FTC etc. | ○ fetched https://support.google.com/youtube/answer/154235 |
| Mid-roll ad minimum length, Premium share %, review time | **Unverified this session** (§01 reports review "typically in about 1 month" from …/72851). Kept out of the critical path. | — |

### 0.2 The two policies that decide whether any of this pays

| Policy | What it says | Source |
|--------|--------------|--------|
| Inauthentic content (effective 2025-07-15) | Not monetizable: "Similar or repetitive content with low educational value…", "Image slideshows, templated storylines, or scrolling text with minimal or no narrative", "AI-generated content made with generic or unoriginal templates giving the impression of mass production". Monetizable: each video "materially varied" with "creative, educational, or other value"; consistent intros/outros are fine. | ○ fetched https://support.google.com/youtube/answer/1311392 |
| Altered/synthetic disclosure | Required for realistic content that "Makes a real person appear to say or do something they didn't do", "Alters footage of a real event or place", "Generates a realistic scene that didn't actually occur". **Not** required: "Fantastical scenes", "Fully animated videos", "Cloning one's own voice to create voice overs", "script generation". Non-disclosure can mean "removal of content or suspension from the YouTube Partner Program". | ○ fetched https://support.google.com/youtube/answer/14328491 |
| Ad suitability | Tarot, astrology, divination, occult, paranormal: **not mentioned** in the advertiser-friendly guidelines. "Controversial issues" = "child abuse, adult sexual abuse, sexual harassment, self-harm, suicide, eating disorders, domestic abuse, and abortion." | ○ fetched https://support.google.com/youtube/answer/6162278 |

◐ Consequence for the machine: a card-art slideshow with a synthetic voice reading templated meanings is the exact shape the July 2025 policy names. Every monetized video must carry a real draw, a real question, and Apocky's own line (the "[AUTH] beat" from §03), and the physical 79-card deck on camera is the cheapest proof of a human. The site's own frame, "For entertainment and personal reflection; not professional medical, legal, financial or psychological advice" (✓ https://chaos-tarot.com/pricing), is also the ad-safety strategy: no health, money, legal or "is my ex coming back" angles, ever.

### 0.3 Product facts this section sells (✓ unless tagged)

| Fact | Where |
|------|-------|
| Oracle membership is named **"Apocrypha+"**, "$3.33/month", "7-day free trial", "cancel anytime in Settings"; **"CSL Oracle is the same membership for $1.11/mo less"** = $2.22/month; CSL is not explained on the page | ✓ fetched https://chaos-tarot.com/pricing 2026-09-27 |
| Token packs: 5 tokens "$1.99", 15 tokens "$4.99" (one-time) | ✓ same page |
| Free tier: "Six divination systems plus astrology", "Every spread, daily draws and streaks", "AI interpretation of your spreads", "Chat with Apocrypha, 40 a day, with voice", "Quick Oracle readings, 5 a day", "1 welcome token", "Your last 5 readings in history" | ✓ same page |
| Code has `PREMIUM.MONTHLY_PRICE: 3.33, YEARLY_PRICE: 33.33, LIFETIME_PRICE: 77.77` | ✓ `shared/constants.js` L260–262; yearly/lifetime **not shown** on the pricing page → ⊘ whether sold |
| PDF export exists (`supabase/functions/export-reading`, `lib/export.js`, `PREMIUM.FEATURES.PDF_EXPORT: true`) | ✓ repo |
| Shareable reading images + share targets Twitter/X, Facebook, Reddit, copy link; `twitterHandle: '@ChaosTarot'` (handle owned by a stranger per §01) | ✓ `lib/social-share.js` L18–81 |
| Physical deck: "79 professionally printed cards with original cyberpunk artwork", "ships within the US", "Order Physical Deck" → Stripe payment link `buy.stripe.com/28E7sKgXg1e575ebhF7EQ05`; **price not readable** (Stripe page renders client-side) | ✓ fetched https://chaos-tarot.com/ ; price ⊘ |
| Codex spec: "88 cards" (78 + 10 Singularities), "350gsm art paper with a 'Void Matte' finish", "Holographic Black foil" edges, "rigid, magnetic-clasp box featuring foil-stamped fractal geometry" | ✓ `book/The Chaos Codex…md` L24–26 |
| Singularities S-1 The Event Horizon … S-10 The Singularity | ✓ Codex L192–237 |
| Codex §6.0 "The Game of Madness: TTRPG Integration"; §7.0 "Shadow Work: Debugging the Soul" | ✓ brief §2 |
| Stale internal pricing: `docs/PREMIUM_SETUP.md` $4.99/$39.99; "Esoteric App Development Plan" §6.2 "Occult Economy" tiers Neophyte free / Adept $9.99 / Magus $19.99 with Supabase Stripe FDW | ✓ files; site wins |
| A Kickstarter budget sheet from sheets@kickstarter.com exists (2026-09-07/08); values unread | ✓ brief §4 → ◐ campaign in preparation; date ⊘ |
| Site has "Enter Access Code"; `premium_memberships.access_code` table design | ✓ site nav; ✓ `docs/PREMIUM_SETUP.md` |
| Learn page exists (`src/pages/LearnPage.tsx`, `TarotBasicsLesson.tsx`) | ✓ repo → the course angle has a home |
| Email provider Resend is already in the stack | ✓ brief §1; Resend free-tier limits **unverified** |

---

## 1. The machine: revenue stack by growth stage

Stages S0–S4 follow §01's phases and gates; this section adds the money lines and the metric that opens each.

| Stage | Opens when (metric gate) | Streams switched on | Streams deliberately off | Tag |
|-------|--------------------------|---------------------|--------------------------|-----|
| **S0 Launch** (Phase 0–1, day 0–35) | channel public | Funnel only: description link → free tier → Apocrypha+ $3.33 (7-day trial) / CSL Oracle $2.22 · tokens $1.99/$4.99 · physical deck Stripe link · **newsletter capture from video 1** (§9) · Posts tab | ads (not eligible), memberships, affiliates, sponsorships, paid digital products | ✓ prices; ◐ order |
| **S1 Fan funding** (expected Phase 2, day 35–63) | **500 subs + 3 public uploads/90 d + 3,000 h** (○ §0.1) → apply to expanded YPP | YouTube memberships (§8), Super Thanks, Shopping affiliate tags (§4), Hype asks, first paid PDF (Codex booklet, §6) | sponsorships (too small), print-and-play (cannibalizes deck) | ○ gate |
| **S2 Ads** (Phase 3 + review, day 63–127) | **1,000 subs + 4,000 h, application by 2026-12-27, accepted before 2027-02-01** (§01 G1) | Watch-page ads (55% share ○), Premium share, mid-rolls on long-form, AdSense payouts at $100 (○) | Shorts ads (needs 10M views/90 d, ○; not a goal) | ○ gate |
| **S3 Deck + Kickstarter** (date ⊘; slot it wherever T-0 lands) | campaign date fixed **and** newsletter ≥ 500 (⊘ target) | Pre-launch list, campaign block (§12), Singularities stretch goal, print-and-play as a digital tier | new affiliates/sponsors during the campaign window (they compete for the same click) | ⊘ date |
| **S4 Scale** (≥ 10k subs **or** ≥ 100k long-form views/month, ⊘) | either metric for 2 consecutive months | Sponsorships (§5), own-store Shopping if a supported storefront exists (§13), course (§7), Discord (§10), print-and-play general sale | nothing new before the previous stage's oracle has fired | ⊘ gate |

**O (machine):** every stream in §14's calendar is switched on within 14 days of its gate firing, and no stream is on before its gate. **F:** a stream switched on early (e.g. a Patreon at 100 subs) earns < $20 in its first 60 days, or a gate fires and the stream is still off 30 days later.

---

## 2. YouTube Partner Program preparation

### 2.1 Two applications, not one

| Step | When | What | Tag |
|------|------|------|-----|
| 1 | Phase 0 | 2-Step Verification; Advanced features (ID/phone); one AdSense for YouTube account; channel not made-for-kids; tax info in AdSense (W-9 for a US owner, ⊘ Apocky's entity) | ○ requirements list per https://support.google.com/youtube/answer/72851 (§01 §0.1) |
| 2 | Phase 0 | Description template on every upload: site disclaimer + "Readings generated with the Chaos Tarot AI Oracle; Apocky's voice is Apocky's" line + UTM link | ◐ from §0.2 |
| 3 | Day the Studio counter shows 500 subs + 3,000 h (+3 public uploads in 90 d) | **Apply to the expanded YPP** (fan funding + Shopping). Do not wait for 1,000. | ○ https://support.google.com/youtube/answer/13429240 |
| 4 | Day the counter shows 1,000 subs + 4,000 h, no later than 2026-12-27 | **Apply for ad revenue**; "accept the Shorts Monetization Module" even though Shorts revenue is out of reach, so nothing is left unticked at review | ○ https://support.google.com/youtube/answer/12504220 ; ◐ date from §01 |
| 5 | During review | Keep cadence; re-read every published video against the inauthentic-content examples (§0.2) and delist any pure card-art slideshow; keep the physical deck on camera | ○ https://support.google.com/youtube/answer/1311392 |
| 6 | If accepted before 2027-02-01 | Accept updated terms in Studio by 2027-01-31 | ○ https://support.google.com/youtube/answer/12843009 |
| 7 | If not accepted by 2027-02-01 | The bar becomes 8,000 h/365 d (○). Hours banked since October 2026 still count. Reset the ads gate to "8,000 h" and keep S1 (fan funding, unchanged thresholds ○) running. | ○ blog.youtube 2026-08-10 |

### 2.2 The channel's monetization-review defence (◐ from §0.2)

- Every long-form: a dated, real draw on the physical cards, Apocky's own reading line, a specific querent question from comments. Codex lore on screen with **art titles** (e.g. "16 Tower of Obsolescence", "Null The Apockalypse"), Codex names ("The Critical Error") spoken as lore.
- Voice: Apocky's own voice or a clone of it (no disclosure needed, ○ …/14328491). If the ORACLE voice is fully synthetic, tick the altered-content disclosure on that upload anyway; it costs nothing and removes the "consistently choose not to disclose" risk.
- No two videos with the same structure and different card names. The 20 reading templates (✓ `shared/reading-templates.json`) are the variety engine, not the repetition engine.

**O (§2):** expanded-YPP acceptance within 30 days of the 500/3,000 gate; ad-revenue acceptance before 2027-02-01 with zero videos flagged in review. **F:** rejection citing "reused content"/"inauthentic content", or a 500/3,000 gate that fires with no application filed within 7 days.

---

## 3. AdSense strategy

| Lever | Recommendation | Evidence | Tag |
|-------|----------------|----------|-----|
| Expectation | Treat ads as the smallest line for 12 months. Entertainment & Vlogs RPM $0.50–$3, Education & How-to $3–$8 (○ https://milx.app/en/trends/youtube-cpm-rpm-rates-2026-average-niches-countries-more, 2026-03-16). All-niche median ≈ $2.30, Education ≈ $10.22 (○ https://air.io/en/air-data-findings/which-youtube-niche-makes-the-most-money-in-2026-ranked-by-real-rpm-and-cpm, 2026-07-01, via §01; my fetch was truncated). **No tarot-specific RPM found in any 2026 source; tarot RPM is unverified.** | ○ | ○ |
| Format mix | Long-form only for ads. Shorts RPM "$0.01 to $0.50 per 1,000 views" (○ https://www.nexlev.io/youtube-shorts-monetization) and the 10M-views gate (○) make Shorts a discovery line, not an ad line. | ○ | ○ |
| Pull RPM up | Lean the long-form mix toward "how to read" / Codex explainer (Education bracket) rather than pure entertainment readings; keep a US-heavy audience (US/UK/CA/AU CPMs are the top bracket, ○ https://vloggingpro.com/youtube-rpm/). | ○ | ◐ |
| Self-certification | Certify honestly; the categories that bite are "Harmful acts and unreliable content", never "tarot". Keep thumbnails free of shocking/occult-gore cues. | ○ …/6162278 | ◐ |
| Mid-rolls | Enable on every long-form that qualifies by length (minimum length **unverified this session**; check the Studio toggle); place breaks at card reveals, not mid-sentence. | — | ⊘ |
| Q4 | Advertiser budgets peak in Q4 (○ vloggingpro); the S2 window (Dec–Jan) is the best two months of the year to switch ads on. | ○ | ◐ |
| Payout | $100 threshold, paid 21st–26th of the following month (○ §0.1). At ⊘ $1.50 RPM that is ≈ 67k monetized views; expect the first payout 2–3 months after acceptance. | ○ | ◐ |

**O (§3):** first AdSense payout by month 5 after acceptance; observed RPM ≥ $1.50 over the first 90 days of ads; ≥ 90% of long-form green-iconed. **F:** any video yellow-flagged for content, or RPM < $0.75 after 60 days of ads with ≥ 50% US audience (then the Education tilt failed and ads are a rounding error, which changes nothing else in the stack).

---

## 4. Affiliate marketing (with FTC disclosure)

### 4.1 What to affiliate, and what never to

| Category | Examples | Rate | Rule |
|----------|----------|------|------|
| Reference decks the Codex builds on | Rider-Waite-Smith (the Codex's "standard 78-card structure"), Thoth | Amazon "Toys" 3%, ○ https://azonpress.com/amazon-affiliate-commission-rates/ (2026-04-08; the official schedule is login-gated) | Allowed: they are the textbook, not a competitor |
| Tarot books, journals, card sleeves, deck boxes, tripods/lighting for reading on camera | — | Physical Books 4.5%, Home 3%, Kitchen 4.5%, all other 4% (○ same) | Allowed |
| TTRPG gear for "The Game of Madness" (Codex §6.0 ✓) | dice, GM screens | 3–4% (○) | Allowed; ties to the sibling HALO audience (✓ brief §3) |
| **Competing cyberpunk/glitch decks** | Teknebrae, Neon Moon, Cyberpunk 2077 deck | — | **Never.** The channel's job is to sell the 79-card Chaos deck. |
| Psychic-hotline, supplement, crypto, "manifestation" programs | — | — | **Never** (site framing; ad-safety §0.2). |

### 4.2 Mechanics

- **YouTube Shopping affiliate program** (US eligible; brands set commission; 60–120-day payout) from the 500-sub tier (○ §0.1). Tag products in long-form and Shorts; it is the only affiliate channel that lives inside the player.
- Amazon Associates for description links (rates ○ above; cookie window unverified).
- Stage gate: **S1** (needs YPP for Shopping tags; Amazon has its own sales minimums to keep an account, unverified). Volume gate for *effort*: only add links to videos with ≥ 2,000 views (⊘) — below that, a 4% commission on a $20 book at 0.5% click-through is cents.
- Cannibalization guard: an affiliate link never appears in a video whose CTA is the Chaos deck or the Kickstarter; one CTA per video (◐ from §01 funnel arithmetic).

### 4.3 FTC disclosure (do this from the first affiliate link)

| Requirement | Wording that passes | Source |
|-------------|---------------------|--------|
| Affiliate links are a material connection; disclose every time | "Paid link" next to the link is adequate; "affiliate link" or "commissionable link" alone is **not** | ○ fetched https://www.ftc.gov/business-guidance/resources/ftcs-endorsement-guides-what-people-are-asking |
| Disclosure placement | "The disclosure has the best chance of being clear and conspicuous if it's included in the video itself"; a text-description-only disclosure is "very unlikely to be clear and conspicuous" | ○ same |
| Sponsored content | Start with "Ad:", "#ad", "Sponsored", or "BRAND paid me to tell you about it" | ○ same; 2023 update ○ https://www.ftc.gov/news-events/news/press-releases/2023/06/federal-trade-commission-announces-updated-advertising-guides-combat-deceptive-reviews-endorsements |
| YouTube's own flag | Tick "paid promotion" on any sponsored/placed product; it adds a label at the start of the video | ○ https://support.google.com/youtube/answer/154235 |

Implementation: a lower-third "Paid links in description" card at the first mention plus the spoken line, and "Paid link" beside each URL in the description template.

**O (§4):** ≥ 1 Shopping-tagged product live within 14 days of S1; affiliate revenue ≥ $30/month by month 6 with 100% of affiliate videos carrying in-video disclosure. **F:** any affiliate video without in-video disclosure, or affiliate income < $10/month at month 6 (then drop Amazon links entirely and keep only Shopping tags).

---

## 5. Sponsorships

| Item | Recommendation | Evidence | Tag |
|------|----------------|----------|-----|
| Gate | **S4**: ≥ 10k subs or ≥ 100k long-form views/month for two months. Below that, integration fees are "$50–$250" and dedicated videos "$100–$500" for channels under 10k (○ https://1of10.com/blog/youtube-sponsorship-rates/, search result), which is less than the deck CTA the slot displaces. | ○ | ⊘ gate |
| Pricing | CPM basis "$15 to $80" by niche and format (○ https://sponsorradar.com/insights/youtube-sponsorship-rates-what-brands-should-pay, search result); price Chaos Tarot at the lifestyle/entertainment floor ($15–25 CPM, ○ same) × expected 30-day views, dedicated = 1.3–1.5× integration (○ same). | ○ | ◐ |
| Fit list | Journaling/habit apps (the app already has streaks and a journal ✓), indie TTRPG publishers, dice makers, tea/coffee, audio gear for faceless creators. **Exclusions:** competing tarot apps and decks; anything medical, financial, legal, psychic-services, supplements, crypto (site framing ✓). | ✓ site | ◐ |
| Format | One 45–60 s integration read by NARRATOR (Apocky), never by the ORACLE persona; the MAXIMUM CHAOS voice (✓ `docs/CHAOS_ORACLE_VIBE.md`) selling a product would read as parody and breaks the "entertainment only" line. | ✓ | ◐ |
| Compliance | "#ad" at the start of the segment + paid-promotion checkbox (○ §4.3). | ○ | — |
| Kickstarter conflict | No sponsor integrations from T-30 to T+7 of the campaign (§12); the slot belongs to the deck. | — | ◐ |

**O (§5):** first paid integration at ≥ $150 within 60 days of the S4 gate; sponsor-video AVD within 10% of the channel median. **F:** a sponsored video's AVD drops > 20% below median (then integrations move to the last third), or no inbound/outbound deal closes within 90 days of the gate (then sponsorships stay off and the slot stays with the deck).

---

## 6. Digital products

| Product | Exists? | Price (⊘ unless tagged) | Channel | Stage | Note |
|---------|---------|-------------------------|---------|-------|------|
| **PDF export of a reading** | ✓ (edge fn `export-reading`) | Part of Apocrypha+ $3.33 / CSL $2.22 (✓) | In-app | S0 | Not a separate SKU. It is a **conversion beat**: every long-form ends with "export this reading" on screen; the PDF is the thing a viewer can hold. |
| **The Chaos Codex booklet (PDF)** | ✓ printed-booklet PDF in Drive; Markdown in repo | $7–12 (⊘; itch.io tarot PDFs run $2.80–$29.99, ○ https://hitpointpress.itch.io/fablemakers-animated-tarot, https://organical-mechanical.itch.io/iso-tarot, https://wetdryvac.itch.io/a-tarot-of-ink) | Stripe payment link + emailed download via Resend (✓ in stack) — fee 2.9% + 30¢ (○ https://checkoutpage.com/blog/stripe-processing-fees) vs Gumroad 10% + $0.50 (○ fetched https://gumroad.com/pricing) vs Ko-fi shop 5% (○ search https://cartmango.com/ko-fi-fees/) | **S1** (needs ≥ 5 Codex-lore videos live to point at) | Bundle free with any deck order; the Codex is the deck's manual, so selling it alone is a lead product, not a profit line. |
| **Major Arcana printable sampler (22 cards, low-res, watermarked)** | ⊘ not built (art ✓ exists in `New/`) | Free, email-gated | Newsletter lead magnet (§9) | **S0** | The list this builds is the Kickstarter's day-one backers. |
| **Print-and-play 79-card deck (PDF)** | ⊘ not built | $12–15 (⊘) | Kickstarter digital tier first; general sale only **after** fulfilment | **S3 → S4** | Selling PnP before the campaign gives the cheapest substitute to the people most likely to back. Hold it. |
| **"Debugging the Soul" shadow-work workbook** (Codex §7.0 ✓) + the "Shadow Work Deep Dive" template (✓ reading-templates) | ⊘ not built | $5–9 (⊘) | Stripe link / Ko-fi | S4 | Framed as reflection prompts, never therapy (site framing ✓). |
| **"The Game of Madness" TTRPG supplement** (Codex §6.0 ✓: Sanity Check, Wild Magic Surges) | ⊘ not built | $5–10 (⊘) | itch.io / DriveThruRPG (fees unverified) | S4 | Cross-sells to HALO's "streamers looking for table-friendly content" (○ brief §4). |

**O (§6):** Codex PDF sells ≥ 20 copies in its first 30 days on sale with ≥ 50% of buyers arriving via a YouTube UTM; the sampler converts ≥ 3% of site clicks to email. **F:** < 5 Codex PDF sales in 30 days (then it becomes a free deck-order bonus only), or PnP launched before the campaign (a process failure, whatever it sells).

---

## 7. Courses

- ◐ The free "course" is the channel: a 79-video "one card, one video" playlist (art titles on screen) plus the Learn page (✓ `src/pages/LearnPage.tsx`) is the funnel for the whole Education-bracket RPM tilt (§3). Do not sell it.
- ⊘ A paid course ("Reading the Glitch: the Chaos Tarot in 8 sessions", $29–49) is an **S4** item only: it needs ≥ 5k subs and a proven playlist retention (≥ 40% average retention on the one-card videos, ⊘) to fill a cohort; before that the hours are better spent on the deck. Framing: how to read *this deck*, entertainment and reflection; never outcomes, never "become a professional reader".
- Delivery at zero cost: unlisted YouTube videos behind a Stripe payment link + access code (the site already has "Enter Access Code" ✓) — no course platform.

**O (§7):** playlist average retention ≥ 40% and ≥ 20 pre-orders at a $29 pre-sale before any recording starts. **F:** < 20 pre-orders in 30 days (then the course stays a free playlist).

---

## 8. Memberships (YouTube) vs the Oracle subscription (product)

| Question | Answer | Tag |
|----------|--------|-----|
| Should YouTube memberships mirror Apocrypha+? | **No.** Two $3.33-ish subscriptions split one buyer. The product subscription (Stripe, 100% minus 2.9% + 30¢ ○) beats a YouTube membership (70% share ○) on every dollar and owns the customer. | ◐ (§01 agrees) |
| What are YouTube memberships for, then? | Channel-only perks that the app cannot give: early access to long-form, members-only Posts, name in the end-card "Backers of the Glitch", a monthly members-only MAXIMUM CHAOS reading. | ◐ |
| Tiers | **Signal** $1.99 (early access + members posts) · **Glitch** $4.99 (+ monthly members reading + credits). Two tiers max; more is ops load (A5). | ⊘ prices |
| Bridge to the product (product request) | Glitch members receive a CSL-price access code (site has "Enter Access Code" ✓; `access_code` table design ✓). Whether "CSL Oracle" is a code-gated price is an open question (§Open questions). | ⊘ |
| Gate | **S1** (500-sub tier ○). Turn on the day the expanded-YPP acceptance lands. | ○ |
| Patreon / Ko-fi instead? | Patreon "Creator" 10% + 2.9% + 30¢ (5% + 10¢ ≤ $3), legacy 5/8/12% plans closed to new creators (○ https://toolradar.com/tools/patreon/pricing, verified Sept 2026; official page 403); Ko-fi 0% tips, 5% memberships/shop, Gold $12/mo for new accounts (○ search https://cartmango.com/ko-fi-fees/, https://schoolmaker.com/blog/ko-fi-pricing; official pages 403). Neither beats YouTube's in-player button for a YouTube audience, and both add a login. **Ko-fi tips only** (0%) as a "buy the Oracle a coffee" link in the description from S0; no Patreon. | ○ fees; ◐ choice |
| Stripe note | $2.22 loses 16% to Stripe fees ($0.30 + 2.9%), $3.33 loses 12%; the $33.33 yearly price in code (✓) loses 3.8%. If yearly is not on the pricing page, that is a product request worth more than any membership tier. | ◐ from ○ fees |

**O (§8):** ≥ 0.5% of subscribers are members 90 days after S1 with churn < 15%/month; Apocrypha+ conversions attributed to YouTube do not fall in the month memberships launch. **F:** memberships > 0 but Apocrypha+ UTM conversions drop ≥ 25% month-over-month (cannibalization; then memberships go to $1.99 only), or < 5 members after 90 days (then leave it on and stop spending hours on perks).

---

## 9. Newsletter

| Item | Recommendation | Evidence | Tag |
|------|----------------|----------|-----|
| Why first | The Kickstarter's first 48 hours come from a list, not from the algorithm; comparable cyberpunk deck Teknebrae Tarot v2.0 closed at $22,431 / 310 backers / $72 average in 18 days (○ https://www.kicktraq.com/projects/pixeloccult/teknebrae-tarot-v20/). 310 backers is a 500–1,000-address list at ⊘ 30–60% conversion. | ○ | ◐ |
| Tool | Zero cash: **Kit free plan ≤ 10,000 subscribers** (○ search https://www.emailtooltester.com/en/reviews/convertkit/pricing/ ; official page 403) or **beehiiv Launch ≤ 2,500** (○ fetched https://www.beehiiv.com/pricing; Scale $43/mo after). Self-host on Resend + Supabase (✓ both in stack) only if Apocky wants the data in-house; Resend limits unverified. Substack takes 10% of paid (○ search) and has no paid tier here → no. | ○ | ◐ |
| Gate | **S0, before the first upload.** The capture form and the 22-card sampler (§6) ship with the channel. | — | ◐ |
| Cadence | Weekly "Signal / Glitch" dispatch: the week's daily-oracle card (art title), one Codex lore paragraph, one video, one CTA. ≤ 30 min/week (A5). | ✓ features | ⊘ time |
| Compliance | Unsubscribe link + postal address per CAN-SPAM (tool handles it; specifics unverified). | — | ⊘ |

**O (§9):** ≥ 500 addresses by the earlier of day 90 or T-14 of the campaign; weekly open rate ≥ 35%. **F:** < 150 addresses at day 90 with ≥ 20,000 views (the magnet, not the channel, is broken; swap the sampler for a spread-sheet PDF), or campaign day-1 backers < 10% of the list.

---

## 10. Communities

| Layer | What | Gate | Cost | Tag |
|-------|------|------|------|-----|
| YouTube **Posts** | Daily card poll ("Signal or Glitch today?"), reading requests intake, Kickstarter countdowns | S0 (no subscriber threshold ○) | 10 min/day | ○ |
| YouTube **Communities** | Opt in once available on the channel; mobile-only (○) | S1 | Low; moderation via YouTube tools | ○ |
| **In-app community gallery** (✓ site) | Every video's CTA to *post your reading to the gallery*; the "Lazy Moderation" AI-sentinel design already exists (✓ Esoteric plan §6.1) | S0 | Product ops | ✓/◐ |
| **Discord** | Not before S4 or the campaign's T-14, whichever is first; it is the one layer that costs hours daily (A5) | S3/S4 | High | ⊘ |
| Reddit/r-tarot posting | Only as Apocky, never as marketing; tarot subreddits enforce self-promo rules (unverified) | S0 | — | ⊘ |

**O (§10):** ≥ 5% of viewers engage with weekly Posts polls; gallery submissions with a "from YouTube" flag ≥ 20/month by day 90. **F:** Posts engagement < 1% for 4 consecutive weeks (stop polls, keep intake), or Discord opened and < 50 active members after 30 days.

---

## 11. The physical 79-card deck

| Item | Fact / recommendation | Source | Tag |
|------|-----------------------|--------|-----|
| What ships today | "79 professionally printed cards with original cyberpunk artwork", US shipping, Stripe payment link; price not readable | ✓ site | price ⊘ |
| Codex spec vs shipped | Codex promises 350gsm Void Matte, holographic black foil edges, magnetic-clasp box, 88 cards; the site sells 79. ⊘ whether current stock matches the Codex spec | ✓ both | ⊘ |
| Unit-cost benchmarks | Offset, 78-card decks: 200 units $6.36 (shrink) / $8.55 (magnetic rigid box); 500 units $3.54; 1,000 units $2.67 (shrink) (○ fetched https://www.qinprinting.com/custom-card-deck-cost/); add-ons (foil, gilded edges, booklet) "+$0.50–$2.50 per deck" (○ search, same site). Print-on-demand, MakePlayingCards: 1 deck $26.10; 100–249 $14.80; 500–999 $9.95; 1,000–2,499 $7.45; up to 160 cards per deck (○ fetched https://www.makeplayingcards.com/design/design-your-own-tarot-cards.html); foil edges/magnetic box on POD unverified | ○ | ○ |
| Pricing corridor | Comparable campaign average pledge $72 (○ Kicktraq); first-time goals "$5,000 to $15,000" and manufacturing minimums "typically 500 decks" (○ https://www.pledgebox.com/post/kickstarter-tarot-decks, 2025-12-12). ⊘ retail $44–55 for the Codex-spec deck; $30–35 for a shrink-wrap edition. | ○ | ⊘ |
| Channel role | The deck is the on-camera human element (§0.2) **and** the money (§15). Every long-form: physical draw on camera; every Short: one card, physical, then the art full-screen. | ◐ | ◐ |
| CTA logic | Until the Kickstarter date is known: "Order the deck" → Stripe link. From T-60: "Notify me" → newsletter (§9) so pre-campaign demand pools into day one instead of leaking into single Stripe orders. | ◐ | ◐ |
| Shopping own-store | Requires a supported storefront (Shopify/Spring/…, ○ search); a bare Stripe payment link does not qualify (◐). Decision for S4: open a Spring/Shopify storefront for the deck and merch, or keep Stripe and skip in-player tags. | ○/◐ | ⊘ |

**O (§11):** deck orders with `utm_source=youtube` ≥ 20 by day 90 (§01's oracle), and the pre-campaign "notify me" list grows faster than Stripe orders fall. **F:** ≥ 20,000 views and < 3 attributed orders by day 90, or a deck video's comments show viewers asking "where do I buy this" (the CTA is invisible).

---

## 12. Kickstarter (timing unknown; the channel's job before, during, after)

Facts: fees 5% platform + Stripe "roughly 3-5%", micropledges < $10 at 5% + $0.05, "If a project does not reach its funding goal, no fees are collected" (○ fetched https://updates.kickstarter.com/kickstarter-fees-a-comprehensive-guide-for-creators/, 2024-03-13); 1,200+ tarot projects, 52% success vs 38% platform average (○ PledgeBox); comparables Teknebrae Tarot v2.0 $22,431 / 310 / avg $72 (○ Kicktraq), Gentle Tarot Dream Deck $75,790 / 702 backers (○ search https://www.kickstarter.com/projects/thegentletarot/limited-edition-hardcover-gentle-tarot-full-size-guidebook), §01's four (Endless $56,525/682; Tarot of Satan $49,297/452; Spellbound CA$29,536/277; Essentia $8,857/122). A Kickstarter budget sheet exists (✓ brief §4) → ◐ campaign in preparation; T-0 ⊘.

| Window | Channel does | Gate / metric | Tag |
|--------|--------------|---------------|-----|
| **T-90 → T-30 (pre-launch)** | Newsletter magnet live (§9); "Notify me" replaces "Order now" at T-60 (§11); 79 one-card Shorts run as a countdown (art titles); 2 long-form: "How the Chaos deck is made" (350gsm / foil / box proof, human on camera) and "The Game of Madness" TTRPG playtest; Kickstarter pre-launch page link in every description (pre-launch pages: ○ common practice, **unverified this session**); no sponsors, no affiliates | list ≥ 500 by T-14; Posts poll "which Singularity should be the stretch goal?" (A8 lore, digital-only unless printed) | ◐/⊘ |
| **T-30 → T-0** | Trailer long-form (Premiere with chat); Hype asks on every long-form (if in YPP, 500–500k subs ○); members' early look (§8); daily Posts | trailer ≥ 10k views in 7 days (⊘) | ◐ |
| **T-0 → T+3 (launch)** | Launch-day long-form + Premiere; daily Shorts for 72 h (§01 P6 block); pinned comment + description = campaign link only; live "Ask the Oracle" if ≥ 1,000 subs (mobile-live viewer limits below 1,000, ○ §01) | ≥ 30% of goal in 72 h (⊘; Teknebrae hit goal in 18 days) | ◐ |
| **Mid-campaign** | Stretch-goal reveals: **the 10 Singularities as a digital or printed add-on** (A8), holographic-foil edge upgrade, Codex hardcover; backer-question readings as long-form; print-and-play digital tier (§6) | weekly pledge velocity > 2% of goal/day (⊘) | ⊘ |
| **T-2 d → T-0 close** | Daily Shorts for the last 48 h; "last call" Premiere; newsletter ×2 | final-48-h lift ≥ 20% of total (⊘ typical U-curve, unverified) | ⊘ |
| **After** | Fulfilment updates as Posts; backer unboxings as UGC Shorts; "deck in the wild" playlist; CTA back to Stripe (or storefront) for late orders; PnP goes on general sale; Codex PDF bundled | late orders ≥ 10% of campaign units in 90 days (⊘) | ⊘ |

If T-0 is inside the 90-day YPP window (A3's "if wrong" case): the P6 block still runs, and the hours it banks count toward the 4,000; the ads application slips only if the launch videos underperform the reading videos on AVD.

**O (§12):** funded; ≥ 40% of day-1 backers carry a newsletter or YouTube UTM; backers ≥ 300 (Teknebrae comparable). **F:** < 30% of goal at T+7 (then the campaign pivots to a shrink-wrap edition and a lower goal, per PledgeBox's $5–15k first-timer range), or < 20% of backers attributable to the channel/list (the channel was not the campaign's engine and the next campaign should not wait on it).

---

## 13. Other revenue streams

| Stream | Value | Gate | Tag |
|--------|-------|------|-----|
| Super Thanks / Super Chat / Stickers | 70% share (○); expect ⊘ $0.05 per 1,000 views | S1 | ○/⊘ |
| Jewels & gifts (live) | listed at the 500-sub tier (○ …/13429240); needs live readings, which §01 defers to ≥ 1,000 subs | S2+ | ○ |
| YouTube Premium share | share % not stated on the earnings page (○ …/72902); arrives with S2 automatically | S2 | ○ |
| Hype | reach, not money; free for viewers; 7-day window; ask on every long-form from S1 | S1 | ○ |
| Tokens | $1.99 / $4.99 packs (✓); the impulse line for free users who watch a full Oracle reading on YouTube and want one | S0 | ✓ |
| Yearly / lifetime (code: $33.33 / $77.77 ✓) | if surfaced on the pricing page, the best-margin recurring line (§8) | S0, product request | ⊘ sold? |
| Licensing the art / oracle voice, B2B | out of scope; not a channel outcome | — | — |

---

## 14. When each method is introduced (the calendar, with the gate that opens it)

| Stream | Stage | Metric gate (must be observed in Studio/Stripe, not estimated) | Falsifier of the gate itself |
|--------|-------|------------------------------------------------------------------|------------------------------|
| Funnel links, tokens, deck Stripe link, Ko-fi tips | S0 | channel public | — |
| Newsletter + 22-card sampler | S0 | first upload | list < 150 at day 90 (§9 F) |
| Posts / polls / gallery CTA | S0 | Advanced features on | engagement < 1% ×4 weeks |
| Codex PDF | S1 | ≥ 5 Codex-lore videos **and** expanded-YPP applied | < 5 sales / 30 d |
| YouTube memberships, Super Thanks, Hype asks | S1 | 500 subs + 3 uploads/90 d + 3,000 h; accepted | cannibalization ≥ 25% (§8 F) |
| Shopping affiliate tags, Amazon links (adjacent goods only) | S1 | accepted; video ≥ 2,000 views | < $10/mo at month 6 |
| Watch-page ads, mid-rolls, Premium | S2 | 1,000 subs + 4,000 h; accepted before 2027-02-01 (else 8,000 h) | RPM < $0.75 after 60 d |
| Pre-launch block, "notify me", Singularities stretch goal, PnP digital tier | S3 | campaign date fixed **and** list ≥ 500 | < 30% of goal at T+7 |
| Sponsorships | S4 | ≥ 10k subs or ≥ 100k views/mo ×2 months | no deal in 90 d |
| Course, Discord, PnP general sale, own-store Shopping | S4 | S4 + previous stream's oracle fired | < 20 pre-orders / < 50 active |

**O (§14):** the calendar's on-dates, once written into `ISSUES.md`, are each within 14 days of their gate. **F:** any stream introduced by calendar date rather than by observed metric.

---

## 15. Simple revenue model (every input ⊘ unless tagged; replace the inputs, the arithmetic follows)

### 15.1 Inputs

| Input | Day 90 (end Phase 3) | Month 6 | Month 12 | Basis |
|-------|---------------------|---------|----------|-------|
| V = long-form views/month | 30,000 | 100,000 | 250,000 | ⊘ (§01 used 30k at day 90) |
| Subscribers | 1,000 | 4,000 | 12,000 | ⊘ |
| RPM (ads, only once in S2) | $0 (pre-acceptance) | $1.50 | $1.50 | ⊘ inside ○ $0.50–$3 Entertainment / $3–$8 Education; tarot unverified |
| Click-through to site | 1.5% | 1.5% | 1.5% | ⊘ (§01) |
| Free account rate | 25% | 25% | 25% | ⊘ (§01) |
| Trial start rate | 10% | 10% | 10% | ⊘ (§01) |
| Trial → paid | 50% | 50% | 50% | ⊘ (§01) |
| Monthly churn | 10% | 10% | 10% | ⊘ |
| Blended ARPU (mix of $3.33 and $2.22) | $3.00 | $3.00 | $3.00 | ✓ prices; ⊘ mix |
| Stripe fee on ARPU | 13% | 13% | 13% | ◐ from ○ 2.9% + 30¢ |
| Token buyers (% of free accounts) × pack | 5% × $4.99 | same | same | ⊘ |
| Deck price / gross margin | $44 / 50% | same | same | ⊘ (unit ⊘ $11 at 500 units + ship/pack ⊘ $11) |
| Deck orders (% of site clicks) | 0.75% | 0.75% | 0.75% | ⊘ (§01: 0.5–1%) |
| Members (% of subs) × price × 70% | 0.5% × $4.99 × 0.7 | same | same | ○ share; ⊘ rate |
| Super Thanks per 1,000 views | $0.05 | $0.05 | $0.05 | ⊘ |
| Affiliate per 1,000 views | $0.10 | $0.10 | $0.10 | ⊘ (4% × $25 × 1% click × 10% buy) |
| Sponsorship | $0 | $0 | 1 × $150 | ○ $50–$500 range under 10k; ⊘ 1/month at 12k |

### 15.2 Output (monthly run-rate, gross of Stripe fees except where noted)

| Line | Day 90 | Month 6 | Month 12 | Formula |
|------|--------|---------|----------|---------|
| Ads | $0 | $150 | $375 | V × RPM / 1,000 |
| Site clicks → free → trials → new paid | 450 → 112 → 11 → 6 | 1,500 → 375 → 37 → 19 | 3,750 → 937 → 94 → 47 | V × CTR × rates |
| Paid subscriber stock | ≈ 15 | ≈ 80 | ≈ 300 | cumulative new − 10% churn/month (approx.) |
| Subscription revenue (net of Stripe) | ≈ $39 | ≈ $209 | ≈ $783 | stock × $3.00 × 0.87 |
| Tokens | ≈ $28 | ≈ $94 | ≈ $234 | free accounts × 5% × $4.99 |
| Deck gross / margin | 3.4 orders → $148 / $74 | 11.3 → $495 / $248 | 28.1 → $1,238 / $619 | clicks × 0.75% × $44 |
| Memberships | ≈ $17 | ≈ $70 | ≈ $210 | subs × 0.5% × $4.99 × 0.7 |
| Super Thanks + affiliate | ≈ $5 | ≈ $15 | ≈ $38 | V × ($0.05 + $0.10) / 1,000 |
| Sponsorship | $0 | $0 | $150 | — |
| **Total, deck at margin** | **≈ $163/mo** | **≈ $786/mo** | **≈ $2,409/mo** | |
| **Total, deck at gross** | ≈ $237/mo | ≈ $1,033/mo | ≈ $3,028/mo | |

Kickstarter (one-off, ⊘): 300 backers × $72 = $21,600 gross; − 8–10% fees (○) ≈ $19,600; − 500 decks × ⊘ $11 unit ≈ $5,500; − shipping/packaging ⊘ $8 × 300 = $2,400; − stretch/extras ⊘ $2,000 → **≈ $9,700 net before art/tooling**, with ≈ 200 decks left for Stripe sales at $44 (≈ $8,800 gross). Sensitivity: backers are the only input that moves this by > $5k; backers ≈ list × 30–60% (⊘) → the list (§9) is the model's master variable.

◐ Reading: ads are ≈ 15% of month-12 revenue at the ⊘ $1.50 RPM (≈ 6% at the ○ Entertainment floor of $0.50, ≈ 30% only at the ○ Education ceiling of $8); the subscription stock and the deck are ≈ 60–67% (58% on deck margin, 67% on deck gross). The channel's monetization job is therefore (1) YPP status before 2027-02-01 for legitimacy and the small ad line, (2) a list and a subscriber stock that exist on campaign day.

**O (§15):** by month 6, the observed lines are within 2× of this table in either direction and the deck+subscription share is ≥ 60% of total. **F:** any line off by > 5× at month 6 (then the corresponding ⊘ input is wrong and the calendar in §14 must be re-gated on the observed number, not on the stage).

---

## Open questions for Apocky

1. **Kickstarter T-0**, even to the month. It decides whether S3 precedes S2 and when "Order now" becomes "Notify me" (§11, §12).
2. **Physical deck price and stock on hand**, and whether the current stock matches the Codex spec (350gsm, foil edges, magnetic box) or is a shrink-wrap edition (§11).
3. **What "CSL Oracle" is** on the pricing page ("the same membership for $1.11/mo less"): a code-gated price, a community price, a CSL-language tie-in? It determines whether YouTube Glitch members can be handed a CSL access code (§8).
4. Are the **yearly $33.33 / lifetime $77.77** prices in `shared/constants.js` sold anywhere? If not, should they be (§8 Stripe-fee note)?
5. **Singularities**: printable as a stretch goal, or digital-only forever (A8, §12)?
6. **Storefront**: keep the Stripe payment link, or open a Spring/Shopify store so the deck can be tagged in-player (§11, §13)?
7. Resend free-tier limits and whether Apocky prefers a self-hosted list over Kit/beehiiv (§9).
8. Voice for sponsor reads and the ORACLE: own voice, own clone, or fully synthetic (§2.2, §5)?
9. Is there a Unirecall export or a prior channel/monetization decision I should fold in? Unirecall was unreachable again this session.

## Issue candidates

| Title | Phase 0–3 | Priority | Acceptance oracle | Falsifier | Depends on |
|-------|-----------|----------|-------------------|-----------|------------|
| Monetization setup: 2SV, Advanced features, AdSense for YouTube, tax info, description template with disclaimer + "Paid link" slots | 0 | P0 | All four show green in Studio/AdSense before upload 1 | Any upload without the template | Handle (§01) |
| Newsletter capture + 22-card Major Arcana sampler (art titles, watermarked) | 0 | P0 | Form live in every description; ≥ 150 addresses by day 45 | < 150 at day 90 | Art export from `New/` |
| Ko-fi tips link (0% fee) in description template | 0 | P3 | Link live | — | — |
| Expanded-YPP application filed the day 500/3,000 shows | 1–2 | P0 | Application timestamp ≤ gate date + 7 d | Gate met, no application in 7 d | Threshold tracker (§01) |
| Memberships tiers (Signal $1.99 / Glitch $4.99) + perks doc; no Oracle-feature overlap | 2 | P1 | Live within 14 d of acceptance; Apocrypha+ UTM conversions flat or up that month | UTM conversions −25% MoM | Expanded-YPP acceptance |
| Codex booklet PDF on a Stripe link with Resend delivery; bundle with deck orders | 2 | P1 | ≥ 20 sales / first 30 d, ≥ 50% via YouTube UTM | < 5 sales / 30 d | ≥ 5 Codex-lore videos |
| Shopping affiliate + Amazon adjacent-goods list; in-video "Paid link" lower-third; exclusion list (no competing decks) | 2 | P2 | First tagged product ≤ 14 d after acceptance; 100% in-video disclosure | Any affiliate video without in-video disclosure | Expanded-YPP acceptance |
| Ads application by 2026-12-27; accept Shorts Monetization Module; enable mid-rolls on qualifying long-form | 3 | P0 | Accepted before 2027-02-01; ≥ 90% green icons | Rejection citing inauthentic/reused content | G1 (§01) |
| Kickstarter support block: T-60 "notify me" switch, countdown Shorts, made-how long-form, trailer Premiere, 72 h/48 h Shorts bursts, post-campaign UGC playlist | 2–3 (slides with T-0) | P1 | ≥ 40% of day-1 backers attributable to channel/list | < 20% attributable | T-0 known; list ≥ 500 |
| Product request: surface yearly $33.33 on pricing page; UTM attribution on signup; YouTube link in footer; fix `social-share.js` handle | 1 | P1 | Pricing page shows yearly; Supabase stores `utm_source` | Attribution still by guesswork at day 60 | Apocky (product repo) |
| Product request: Glitch-member access code → CSL price | 2 | P2 | Code redeemable; ≥ 5 redemptions in 30 d | CSL meaning unknown by Phase 2 | Open question 3 |
| Revenue model sheet with the ⊘ inputs of §15 as editable cells; monthly actual-vs-model row | 1 | P2 | First actuals row by day 60 | Any line off by > 5× at month 6 with no input change | — |
| Sponsorship kit (rate card at $15–25 CPM floor, exclusion list, #ad + paid-promotion checklist) | 3 (S4) | P3 | Kit exists before the S4 gate | Deal negotiated without the kit | ≥ 10k subs or 100k views/mo |
| Print-and-play 79-card PDF (Kickstarter digital tier first; general sale post-fulfilment) | 3 (S3) | P2 | Tier live at T-0; general sale ≥ 30 d after last shipment | PnP on sale before T-0 | Campaign |

## CSL annex

Σ Money order for 12 months: subscription stock + physical deck ≈ 60–67%, ads ≈ 15% at a ⊘ $1.50 RPM (○ RPM ranges, tarot unverified); YPP is legitimacy and a small line, not the plan. Gates: 500/3,000 → fan funding + Shopping (○); 1,000/4,000 accepted before 2027-02-01 → ads (○); list ≥ 500 + T-0 known → campaign; 10k subs → sponsors/course/Discord.
W! Newsletter + 22-card sampler from upload 1; apply to the expanded YPP the day 500/3,000 shows; in-video "Paid link"/"#ad" + paid-promotion checkbox on every affiliate/sponsor video (○ FTC, ○ YouTube); physical deck on camera in every monetized video; one CTA per video.
W! Fix before Phase 2: yearly price on the pricing page, UTM on signup, footer YouTube link, `@ChaosTarot` handle in `lib/social-share.js`.
N! No YouTube membership that mirrors Apocrypha+/CSL Oracle; no print-and-play before the campaign; no affiliate links to competing decks; no sponsor in the medical/financial/legal/psychic/supplement/crypto set; no Patreon; no stream switched on by calendar instead of by observed gate.
N! Never state a threshold, RPM or fee from memory: every number above carries its URL, and "unverified" items (mid-roll length, Premium share %, own-store platforms, pre-launch pages, Resend limits) stay out of the critical path until checked.
