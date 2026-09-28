# 4. YouTube Growth & Algorithm

## Source prompt

> "Act as a YouTube algorithm expert. Teach me how to grow my channel by understanding impressions, click-through rate, watch time, audience retention, returning viewers, suggested videos, search traffic, and recommendation systems. Build a complete growth strategy including posting frequency, testing methods, optimization tactics, and ways to turn viewers into subscribers.

(Verbatim from `prompts/SOURCE_PROMPTS.md`, prompt 4; no closing quote in the source.)

## Assumptions used

| # | Used as | Status after this section's checks |
|---|---------|-----------------------------------|
| A1 | Plan starts from zero; no analytics history exists | Used. Every numeric target below is ⊘ until the first 100-view video exists (YouTube's own key-moments report needs ≥ 100 views, §1.4). |
| A2 | Faceless-first: AI Oracle voice + card art + app capture, founder segments optional | Used, with a hard constraint added by the July 2026 policy text (§2.2): the Oracle persona must never present as a human expert on health, legal, finance or politics. |
| A3 | YPP first, funnel second, deck/Kickstarter third | Used. The growth loop is tuned for qualified watch hours (long-form) with Shorts as discovery, matching sections 1 and 2. |
| A4 | Near-zero cash | Used. Every testing method here is native to YouTube Studio or free; no paid promotion. |
| A5 | 8–12 h/week | Used for the cadence table (§3.1), which repeats section 1 §4 and adds the algorithm rationale. |
| A6 | Tracking as Markdown in this repo | Used for the test ledger and issue table. |
| A8 | 79 shipped cards; Singularities are Codex lore | Used: the search moat is built on the 79 shipped titles (`New/` filenames ✓) plus the 6 system guides. |

Not used: A7.

Tag key: ✓ VERIFIED this session (file read or URL fetched) · ◐ INFERRED from ✓/○ premises · ○ REPORTED (source named, URL inline) · ⊘ ASSUMED. Every recommendation ends with **O:** (oracle) and **F:** (falsifier).

## Rail status (this session, because Apocky asked for repos and Unirecall to be checked again)

| Rail | Result | Tag |
|------|--------|-----|
| `/home/user/the-chaos-tarot` | Read: `New/` (79 card files + `Card Back.jpg` + `Page of Fractures.jpg`; suits Codes / Networks / Signals / Vectors; `Null The Apockalypse.jpg`), `docs/CHAOS_ORACLE_VIBE.md`, `docs/FEATURE_LIST.md` (v3.0, 2026-01-19, still says "78-Card Deck"), `shared/reading-templates.json` (**14** templates, confirming section 2's correction of the brief's "20"), `Esoteric App Development Plan.md` §6.2 (stale Neophyte/Adept/Magus tiers). `grep -ril youtube` over the repo's Markdown hits only the Codex and the dev plan: **no channel plan or analytics exists in the product repo.** | ✓ |
| `/home/user/chaos-tarot` | Empty directory. | ✓ |
| `/home/user/apocrypha-core` | `specs/HIVE_AND_MEMORY.csl` and `scripts/recall-service.py` describe Unirecall as a federated recall behind `recall-service` on `127.0.0.1:19129`. Probed 19129 from this container: connection refused; no `unirecall` binary on PATH. | ✓ (unreachable) |
| Unirecall | **Unreachable again.** Prior channel decisions, if any, are not in this section. Same local command as section 2: `python -B C:\Users\Apocky\source\repos\anamnesis\unirecall.py "chaos tarot youtube growth algorithm" --tiers l2,l34,vault --timeout 8 -n 6 --json`, paste the JSON into `QUESTIONS.md`. | ✓ (degraded) |
| vidIQ connector | Still `connect_incomplete` (ListConnectors). Keyword volumes stay unverified. | ✓ |

---

## 1. The mechanics (teach-me section), 2025–2026 sources

Read this table left to right: what YouTube says the number is → what moves it → what it means for this channel. Every definition is quoted from a page fetched 2026-09-27.

### 1.1 Impressions and click-through rate

| Item | YouTube's definition / rule | Source | For Chaos Tarot |
|------|------------------------------|--------|-----------------|
| Impression | Counted when the thumbnail "is shown for more than 1 second and at least 50% of the thumbnail is visible on the screen" | ○ https://support.google.com/youtube/answer/9314486 | Impressions are earned, not bought: a 0-sub channel gets them from search first, then from Up Next beside videos its viewers also watch. |
| Where impressions are counted / not counted | Home, Up Next, search results, subscription feed; **not** external websites, **not** end screens | ○ https://support.google.com/youtube/answer/7628154 | Traffic from chaos-tarot.com and TikTok never shows in CTR. Judge the site loop (section 1 G3) by the "External" traffic source, not by CTR. |
| Impressions CTR | "measures how often viewers watched a video after seeing a registered impression" | ○ same | — |
| Benchmark | "Half of all channels and videos on YouTube have an impressions CTR that can range between 2% and 10%" | ○ same | A new channel's first CTRs are inflated: the page itself says narrower, loyal audiences show higher CTR on fewer impressions, and clickbait that wins CTR but loses view duration is not recommended. Read CTR only together with AVD (§1.2) and only after ≥ 1,000 impressions per video (⊘ floor). |

◐ Consequence: the 79 shipped titles ("16 Tower of Obsolescence", "15 The Devil's Algorithm", "Null — The Apockalypse") will produce high-CTR, low-impression search videos early. That is the expected shape of a search-first library, not proof the packaging works for browse. Packaging is proven only when browse/suggested impressions arrive with CTR still ≥ the channel median.

### 1.2 Watch time and average view duration

| Item | Rule | Source | For Chaos Tarot |
|------|------|--------|-----------------|
| YPP watch-hours path | "1,000 subscribers with 4,000 qualified watch hours in the last 12 months" | ○ https://support.google.com/youtube/answer/72851 | Long-form is the hours engine (section 1 §0.1). |
| Shorts-feed watch time | "Qualified watch hours from Shorts views in the Shorts Feed won't count towards the 4,000 qualified watch hours threshold" | ○ same page (search snippet, 2026-09-27) | A Short watched 100% adds nothing to the 4,000 h; a Short that sends a viewer to the long-form does. |
| What YouTube's own A/B tool optimizes | "We're optimizing for overall watch time over other metrics like CTR" | ○ https://www.searchenginejournal.com/youtube-title-a-b-testing-rolls-out-globally-to-creators/562571/ (2025-12-04) quoting YouTube; ○ https://support.google.com/youtube/answer/16391400 | YouTube tells you the ranking currency: watch time per impression, not clicks. |
| February 2027 wall | From 2027-02-01 new applicants need "8,000 qualified watch hours in the last 365 days, or 20 million qualified Shorts views in the last 90 days" + 1,000 subs; "This update won't impact creators already in YPP" | ○ https://blog.youtube/news-and-events/youtube-partner-program-updates-2027-new-opportunities-earn/ (2026-08-10); ○ https://support.google.com/youtube/answer/12843009 | Every growth tactic below is ranked by hours-per-hour-of-Apocky's-time, because the bar doubles in 4 months. |

◐ The arithmetic that sizes everything (from section 1): 4,000 h ≈ 60,000 long-form views at a 4-minute AVD. AVD is therefore the single most leveraged number: moving AVD from 3 to 5 minutes cuts the views needed by 40%.

### 1.3 Audience retention

| Item | Rule | Source |
|------|------|--------|
| Key moments report | "Intro tells you what percentage of your audience still watched your video after the first 30 seconds"; "Top moments are moments in your video where almost no one dropped off"; "Spikes are moments in your video that were rewatched or shared"; "Dips highlight moments in your video that were either skipped or moments where viewers stopped watching" | ○ https://support.google.com/youtube/answer/9314415 |
| Minimum for the report | Video "at least 60 seconds long and have at least 100 views" | ○ same |
| YouTube's own advice | "Modify the first 30 seconds of your video and experiment with different styles"; align "thumbnail and title to better reflect your video content" | ○ same |

For Chaos Tarot (◐, from the ✓ vibe doc): the MAXIMUM CHAOS structure is a retention machine by construction: coherent → connections → escalation → reversal ("The Tower isn't your life collapsing. It's THE WALL COMING DOWN"). Each reversal is a designed **Spike**; each "and one more thing" is a designed **Top moment**. The 30-second Intro number is the one to fix first (section 1 §7 sets ≥ 70% long-form, ≥ 80% Shorts, ⊘ until data).

### 1.4 Returning viewers (now: new, casual, regular)

| Item | Rule | Source |
|------|------|--------|
| Segments | New: "viewers who watched your channel for the first time in the selected time period". Casual: "viewers who watch your channel occasionally". Regular: "returned to watch your content for at least six months in the past year" | ○ https://support.google.com/youtube/answer/13615784 |
| Replaced what | The binary "returning viewers" metric, replaced 2025-07-02; casual = 1–5 months out of the past year, regular = 6+ months | ○ https://www.searchenginejournal.com/youtube-adds-new-viewer-metrics-to-track-audience-loyalty/550289/ |
| YouTube's tips per segment | New viewers: diverse topics, trends, discoverable packaging, collabs; "How-to" channels attract new viewers who may not return. Casual/regular: "consistent content in similar topics or formats", series from proven topics, reply to comments, regular schedule | ○ https://support.google.com/youtube/answer/13615784 |

◐ Hard consequence for a 90-day plan: **nobody can be a "regular" viewer inside 90 days** (6 of 12 months). The loyalty metric that can move in this window is **casual** (watched in ≥ 2 distinct months). The Daily Oracle habit and the Tuesday/Saturday long-form slots exist to convert new → casual by month 2. Section 7 should report casual share, not "returning".

### 1.5 Suggested videos and the recommendation system

| Item | Rule | Source |
|------|------|--------|
| Signals | Watch history, search history, subscriptions, likes/dislikes, "Not interested" / "Don't recommend channel", and "User surveys that ask you to rate videos…helps the system understand satisfaction, not just watch time" | ○ https://support.google.com/youtube/answer/16089387 |
| Surfaces | Home, Up Next, Shorts feed ("personalized to reflect what we think you want to see next"), channel/destination shelves | ○ same |
| Per-video judgement | "The algorithm uses fresh performance data for each individual video, rather than relying on past results" | ○ https://support.google.com/youtube/answer/16533387 |
| Frequency and timing | "publish time is not known to impact a video's long-term performance"; taking time off: "the algorithm doesn't penalize creators for doing so!"; "Prioritize consistent quality content over a high frequency of uploads" | ○ same |
| Packaging consistency | "a consistent title and thumbnail style makes your videos instantly identifiable" | ○ same |
| Governing principle | "The algorithm follows the audience" and "what matters is what viewers enjoy" | ○ same |

◐ Translation: there is no channel-level "momentum" score to feed with volume. Each upload is a fresh bet judged on (impression → click → watch → satisfied). Volume helps only by giving more bets and more hours; it hurts when it degrades the per-bet quality or makes the videos "interchangeable" (§2.1). Suggested placement is co-watch: a Chaos Tarot card decode appears in Up Next beside generic "Tower tarot meaning" videos when the people who watch those also watch ours to the end. So the suggested strategy is: earn search viewers first, satisfy them, and let their watch history carry the channel into Up Next.

### 1.6 Search traffic

| Item | Rule | Source |
|------|------|--------|
| Ranking factors | "Relevance", "Engagement", "Quality"; relevance from "how well the title, tags, description, and video content match your search query"; engagement includes "the watch time of a particular video for a particular query"; quality = "channels demonstrate expertise, authoritativeness, and trustworthiness on a given topic"; results personalised by "search and watch history" | ○ https://support.google.com/youtube/answer/16090438 |
| Research tab | Studio Analytics → Research: "searches across YouTube", "your viewers' searches", and a "Content gap" label where demand exceeds supply | ○ https://www.socialmediaexaminer.com/youtube-research-tab-how-to-find-youtube-content-ideas/ (third-party; availability on a 0-sub channel ⊘) |

◐ Search is the only surface a zero-history channel can earn on day 1, because relevance is judged on metadata + content, not on channel history. The 79 unique card titles and the 6 system guides (runes, I Ching, Ogham, Lenormand, Geomancy, Astrology; ✓ site footer) are ~258 evergreen search targets (section 2 §1.3). "Expertise, authoritativeness and trustworthiness on a given topic" argues for **topical clustering**: publish the 22 Majors as one run, one system at a time, so the channel reads as the authority on its own deck.

### 1.7 Shorts feed

| Item | Rule | Source |
|------|------|--------|
| Length / shape | "square or vertical aspect ratio up to three minutes in length" (uploads from 2024-10-15) | ○ https://support.google.com/youtube/answer/15424877 |
| Feed metrics in Studio | "Shown in feed" = "The number of times that your Shorts showed in the Shorts Feed"; "How many chose to view" = share of feed shows viewed vs swiped away; plus per-Short Subscribers change | ○ https://support.google.com/youtube/answer/12942217 |
| View counting since 2025-03-31 | A view counts when a Short "starts to play or replay"; **"engaged views"** remain the metric for monetization/YPP | ○ https://www.searchenginejournal.com/youtube-changes-shorts-view-counts-no-change-to-monetization/543005/ |
| Shorts → long-form link | "Adding a related video is only available with advanced feature access"; linked video must be public or unlisted; viewers see "a clickable link below your channel handle" | ○ https://support.google.com/youtube/answer/14075157 |
| YouTube's own Shorts→long-form advice | "match the viewer's intent. If your Short teaches a quick hack, link it to a comprehensive, in-depth tutorial"; verbal + visual CTA in the final 5 s; the long-form must pay the promise "within the first 5-10 seconds" | ○ https://blog.youtube/creator-and-artist-stories/youtube-related-videos-traffic-guide/ (2026-07-14) |
| A/B testing | "A/B testing is not available for Shorts" | ○ https://support.google.com/youtube/answer/16391400 |
| Third-party benchmarks (unofficial) | "viewed" share ≈ 50% average, 70%+ strong; swipe-away decided in the first 1–3 s | ○ https://www.shortimize.com/blog/youtube-shorts-retention-rate — **unverified against YouTube; not in the critical path** |

◐ For Chaos Tarot the Short is a card flip: first frame = card art with the shipped title burned in, last frame = the card back so the loop is seamless. "How many chose to view" is the Shorts hook metric; "Subscribers" per Short is the Shorts conversion metric; "related video" clicks (Traffic source: Shorts) is the bridge metric.

### 1.8 Gates that unlock the tools above

| Tool | Gate | Source |
|------|------|--------|
| Advanced features (needed for Test & compare, related video on Shorts, pinned comments, higher upload limits) | Phone verification **and** one of: channel history, ID verification ("passport or driver's license"), video verification; "ID and video verification isn't available to all creators" | ○ https://support.google.com/youtube/answer/9891124 |
| Test & compare | Advanced features; desktop Studio only; up to 3 titles/thumbnails/combos; winner by watch time; "should be completed within two weeks"; not for made-for-kids, mature or private videos; not Shorts | ○ https://support.google.com/youtube/answer/16391400; rollout to titles 2025-12-04 ○ SEJ above |
| End screens | Video ≥ 25 s; "last 5–20 seconds"; up to 4 elements at 16:9; link element needs YPP; not on Shorts | ○ https://support.google.com/youtube/answer/6388789 |
| Hype | YPP channels with "500–500,000 subscribers" in 38 countries incl. US; hype window = first 7 days; Shorts not eligible; top-hyped videos on a country leaderboard with a 7-day "Hyped" badge | ○ https://support.google.com/youtube/answer/15509925 |
| Viewer-side changes that weaken old CTAs | Desktop hover-to-subscribe on the watermark removed (Sept 2025; "< 0.05% of channel subscriptions" came from it); viewers can now hide end screens | ○ https://www.socialsamosa.com/news-2/youtube-remove-end-screens-subscribe-button-10499816; ○ https://www.socialmediatoday.com/news/youtube-rolls-out-option-to-remove-video-end-screens/761065/ |

---

## 2. What YouTube penalizes now (URLs; read before publishing anything)

### 2.1 Inauthentic content (renamed 2025-07-15, clarified 2026-07-16)

| Rule (quoted) | Source | Chaos Tarot exposure |
|---------------|--------|---------------------|
| Policy renamed from "repetitious content" to "inauthentic content" on **15 July 2025**; "channels where content feels interchangeable from video to video are not allowed to monetize" | ○ https://support.google.com/youtube/answer/1311392 | **High by default.** A faceless "one card per video" library is the exact shape named. Mitigation is structural (below), not cosmetic. |
| Allowed: "the same intro and outro for your videos, but the bulk of your content is different"; series where "each video has a distinct storyline, focus, or concept" | ○ same | The Oracle sting and the disclaimer lower-third are fine; the body of every card decode must differ in structure, not only in card. |
| Violates: "Similar or repetitive content with low educational value, commentary, narratives, or minimal variation"; "highly similar storyline templates across multiple videos"; "AI-generated content made with generic or unoriginal templates giving the impression of mass production" | ○ same | A script template filled per card by an LLM is precisely "generic or unoriginal templates". |
| **July 2026 clarification, bucket 2:** "Unsatisfying or off-putting content refers to content that relies heavily on emotionally manipulative formulas, mimics existing formats or stories to a degree that the videos feel interchangeable, or appears designed to shock or surprise viewers for the sole purpose of getting views." | ○ https://support.google.com/youtube/answer/1311392?hl=en-GB (fetched 2026-09-27); ○ https://techcrunch.com/2026/07/20/youtube-clarifies-policies-around-ai-slop-and-upsetting-videos/ (effective 2026-07-16; three buckets) | MAXIMUM CHAOS is "designed to surprise"; it stays on the right side only while the surprise is the *content* (a real pattern in a real spread) rather than the *packaging*. No fake-distress thumbnails, no "THE CARDS WARNED YOU" doom bait. |
| **July 2026 clarification, bucket 3:** "channels that use AI-generated personas to deliver information on sensitive topics. This includes any content that presents itself as a human expert providing advice to viewers on topics such as health, legal issues, finances or politics." | ○ same help page; ○ TechCrunch above (Matt Halprin, VP trust & safety) | **The single biggest policy constraint on A2.** The AI Oracle voice must (a) never present as a human, (b) never read on health, legal, money or politics. The 14 shipped templates include "Career Crossroads" and "Love Potential" (✓ `reading-templates.json`); career/creative/self-discovery are fine, "finances" is not. This is the same line as the site's own disclaimer: "Readings are not a substitute for professional medical, legal, financial, or psychological advice." (✓ site) |
| Reused content: "repurpose content that's already on YouTube or another online source without adding significant original commentary, substantive modifications, or educational or entertainment value" | ○ https://support.google.com/youtube/answer/1311392 | Low exposure: all art and app footage is Apocky's. Do not build Shorts from other creators' pick-a-card clips. |
| Consequence | "Channels with excessive inauthentic content face removal from YouTube's Partner Program" (TechCrunch paraphrase of YouTube) | ○ TechCrunch above | Denied at application = the 90-day goal fails outright. |

**Structural mitigations (◐ from the rules above; each is a W! in the annex):**
1. Human-perspective layer in every long-form: Apocky's own voice or on-screen typed commentary reacting to the Oracle (section 1 §0.3 already requires this).
2. Rotate the *structure* of card decodes across ≥ 4 distinct formats (live draw with a viewer question · art forensics · Codex-lore vs shipped-name comparison · MAXIMUM CHAOS pattern hunt), never one template.
3. Every video contains a live, unscripted app session (a real draw, a real Memory Core recall) so no two videos can be interchangeable even with the same card.
4. The Oracle is an openly synthetic, in-world machine persona ("Oracle of the Glitch", ✓ site persona name), never a "human expert".
5. Topic fence: no health, legal, finance, politics, tragedies (§2.3), ever, in any voice.

**O:** YPP application accepted with no "inauthentic" or "reused" finding; zero videos removed. **F:** rejection citing either policy, or a Studio notice on any upload.

### 2.2 AI disclosure (altered or synthetic content)

| Rule (quoted) | Source |
|---------------|--------|
| Disclose content that "makes a real person appear to say or do something they didn't do", "alters footage of a real event or place", or "generates a realistic scene that didn't actually occur" | ○ https://support.google.com/youtube/answer/14328491 |
| Not required: "Non-realistic" content, "applying beauty filters", "color adjustment", "using generative AI tools to create or improve a video outline, script, thumbnail", animation, "Cloning one's own voice to create voice overs or dubs" | ○ same |
| Label placement: "in the video player" for photorealistic AI content, "in the expanded description" otherwise | ○ same |
| Penalty: "manual application of a label, or penalties from YouTube, including removal of content or suspension from the YouTube Partner Program" | ○ same |

◐ For Chaos Tarot: card art, app capture and a non-human synthetic Oracle voice are "non-realistic" by the page's own examples, so disclosure is not strictly triggered. **Recommendation stays: tick the disclosure box on every upload that uses a generated voice or generated imagery.** The cost is a description-line label; the downside of an undisclosed label being applied manually during YPP review is the whole plan. Never clone anyone's voice but Apocky's own.

**O:** every published video shows the "How this content was made" line where AI voice/imagery is used; zero YouTube-applied labels. **F:** any label applied by YouTube rather than by Apocky.

### 2.3 Ad suitability for divination content

| Finding | Source | Tag |
|---------|--------|-----|
| The advertiser-friendly guidelines' 14 categories contain **no mention** of psychic, tarot, occult, divination, astrology, fortune, supernatural or paranormal | https://support.google.com/youtube/answer/6162278 (fetched 2026-09-27, word search) | ✓ absence |
| Categories that can bite a reading channel: "Harmful acts and unreliable content" (health/scientific misinformation → no ads), "Controversial issues" (self-harm, eating disorders, abuse, abortion as main topic → limited/no ads), "Sensitive events" (profiting from tragedies → no ads) | ○ same | — |

◐ So tarot itself is not a limited-ads category; **what the reading is about** is. The topic fence in §2.1 is the same fence that keeps the green icon. Never read on a viewer's diagnosis, a public tragedy, or a live news event. (Section 1 §0.2 carries the same O/F: ≥ 90% green icons after YPP.)

### 2.4 Threshold and counting changes to keep in view

| Change | Source |
|--------|--------|
| YPP: 1,000 subs + 4,000 h/12 mo or 10M Shorts views/90 d; review "in about 1 month"; needs advanced features, no active strikes, AdSense | ○ https://support.google.com/youtube/answer/72851 |
| 2027-02-01: 8,000 h or 20M Shorts views for new applicants | ○ https://blog.youtube/news-and-events/youtube-partner-program-updates-2027-new-opportunities-earn/ |
| Shorts views count on play since 2025-03-31; engaged views govern YPP | ○ SEJ 543005 above |
| Title A/B testing global since 2025-12-04; watch-time winner | ○ SEJ 562571 above |
| Viewer segments new/casual/regular since 2025-07-02 | ○ SEJ 550289 above |
| Inauthentic content: renamed 2025-07-15; three buckets 2026-07-16 | ○ §2.1 |

---

## 3. Growth strategy

### 3.1 Posting frequency by phase

Cadence is set by three things, none of which is "feeding the algorithm": (1) watch-hours arithmetic (§1.2), (2) the new → casual conversion that needs a second month of touchpoints (§1.4), (3) the inauthentic-content ceiling on how many *distinct* videos 8–12 h/week can produce (§2.1). YouTube's own page says frequency and publish time are not ranking inputs (○ https://support.google.com/youtube/answer/16533387). This table repeats section 1 §4 so the two sections cannot drift; the "algorithm reason" column is new.

| Phase | Dates | Long-form / wk | Shorts / wk | Algorithm reason | Tag |
|-------|-------|----------------|-------------|------------------|-----|
| 0 Setup | Sept 28 – Oct 4 | 0 published; 3 banked | 0 published; 10 banked | Launch with a binge-able cluster so the first search viewer's session stays inside the channel (Up Next needs siblings to chain to) | ◐ |
| 1 Launch | Oct 5 – Nov 1 | 2 (Tue, Sat) | 5–6 (daily, same hour) | Fixed slots build the casual habit; 22-Major cluster establishes topical authority for search (§1.6) | ◐ |
| 2 Compound | Nov 2 – Nov 29 | 2 | 6–7 | Double down on the pillar whose AVD and "chose to view" lead; every extra Short must be a *different structure*, not a clone (§2.1) | ◐ |
| 3 Apply | Nov 30 – Dec 27 | 2 (+1 deck/Kickstarter if dated) | 5 | Protect long-form AVD; cut Shorts before cutting long-form because only long-form banks hours | ◐ |
| Throughput gate (A5) | any week | if < 1 long-form **or** < 4 Shorts shipped → next week on the 4-h schedule (section 1 §9.2), never a skipped week | | Breaks are not penalised (○ 16533387), but the hours clock is | ◐ |

Rules that matter more than the numbers:
- **Distinctness over volume.** If the week's 6th Short would be structurally identical to the 5th, drop it. A dropped Short costs nothing; an "interchangeable" pattern costs YPP.
- **Fixed publish slots are for humans, not the ranker.** Keep Tue/Sat and the daily-Short hour because casual viewers return to habits; move them freely if analytics show a better audience window (Studio → Audience → "When your viewers are on YouTube").
- **Never front-load the 90 days with a burst.** Per-video judgement (○ 16533387) means a 20-video week has no compounding benefit and maximises the inauthentic-content shape.

**O:** ≥ 22 long-form and ≥ 60 Shorts by day 90 (section 1's numbers) **and** no two consecutive card decodes share the same structure tag in the tracker. **F:** < 16 long-form by day 90, or ≥ 3 consecutive uploads with the same structure tag.

### 3.2 Testing methods

**T1 — Native Test & compare (long-form only).**
- Facts: up to 3 titles / 3 thumbnails / 3 combos; YouTube picks by watch time; "should be completed within two weeks"; desktop Studio; needs advanced features; not Shorts (○ https://support.google.com/youtube/answer/16391400). Third-party guides say tests need on the order of 1,000–5,000 impressions per variant to conclude (○ https://gyre.pro/blog/youtubes-new-title-ab-testing-tool-everything-creators-need-to-know — unverified; treat as a warning that early tests will be inconclusive, not as a number to plan on).
- Protocol: one variable per test. Test 1 (Phase 1): thumbnail = card art only vs card art + 3-word Oracle line vs app-capture frame. Test 2: title order = "[shipped card name]: [tech reframe]" vs "[generic card] meaning for [persona]" (e.g. "Tower of Obsolescence: when your stack gets deprecated" vs "Tower tarot meaning for programmers"). Test 3 (Phase 2): thumbnail text in Codex lore name vs shipped name (the brief's contradiction, turned into data).
- Inconclusive result → "the first title and thumbnail you upload will be the default" (○ same page), so upload the safe variant first.
- Ledger: `plans/chaos-tarot-youtube/TESTS.md` (A6): date · video · hypothesis · variants · impressions/variant · winner · watch-time delta · decision. A test without a pre-registered hypothesis is not run.
- **O:** ≥ 6 completed (not inconclusive) tests by day 90, and the winning pattern from tests 1–2 is applied to every subsequent card decode. **F:** all tests inconclusive at day 60 (impressions too low → stop testing packaging, test structure instead, T3).

**T2 — Shorts hook tests (no native A/B).**
- Method: same card, two openings, published ≥ 72 h apart at the same hour; compare "How many chose to view" and average % viewed in Studio (○ https://support.google.com/youtube/answer/12942217). This is a sequential test, not a controlled one (◐): feed audience differs by day. Accept a winner only if the gap is ≥ 15 points on ≥ 1,000 "shown in feed" each (⊘ thresholds).
- Hook variants worth testing first: (a) card flip on frame 1 vs (b) the Oracle's escalation line on frame 1 ("16 minus 5 is 11. WHO PUT THAT THERE?") vs (c) the app's Memory Core recalling a past reading.
- **O:** by week 6 one hook family wins ≥ 3 of 4 pairs and becomes the default. **F:** no consistent winner after 8 pairs (hooks are not the bottleneck; the topic is).

**T3 — Retention structure tests.**
- Tool: key moments (Intro / Top moments / Spikes / Dips) once a video has ≥ 100 views (○ https://support.google.com/youtube/answer/9314415).
- Method: change exactly one structural element per week for the Tuesday long-form (cold open type · position of the first reversal · two-voice alternation interval · chapter length). Log the Intro % and the timestamp of the first Dip.
- **O:** Intro % rises ≥ 10 points between week 2 and week 8 median. **F:** flat or falling Intro % after 6 changes (the format, not the intro, is wrong: shorten long-form to 6–8 min).

**T4 — Search-title tests.**
- Tool: Studio → Research (viewers' searches, content gaps; ○ Social Media Examiner above, availability ⊘) and the per-video Traffic source → YouTube search → terms.
- Method: publish card decodes in matched pairs, one titled by the shipped name first, one by the generic card first; compare search impressions and CTR at day 14.
- **O:** the winning title order raises search impressions ≥ 2× on the next 5 decodes. **F:** search impressions < 100 per video at day 14 for both orders (search demand for card names is not there; shift weight to 8S system guides).

**T5 — Cross-platform pre-tests.**
- Post the Short to TikTok/Instagram Reels first (section 1 G5; ○ Drive doc's 3-6 hashtag rule), use 24-h completion rate to choose which of two hooks goes to YouTube. Free, fast, noisy (◐).
- **O:** TikTok winner also wins on YouTube "chose to view" in ≥ 60% of pairs. **F:** < 50% (platforms disagree; stop using TikTok as a proxy).

### 3.3 Optimization tactics

| Lever | Tactic | Why (source) | Tag | O / F |
|-------|--------|--------------|-----|-------|
| Packaging first | Write title + thumbnail before the script; the script must pay the title inside 30 s | Intro metric ○ 9314415; A/B winner = watch time ○ 16391400 | ◐ | O: Intro ≥ 70% median by week 8 · F: < 55% |
| Title | Shipped card name first, tech reframe second, ≤ 60 chars; the generic card name in the description's first line | Relevance = title/description/content match ○ 16090438 | ◐ | O: video ranks top-10 for its own shipped name within 7 days · F: not ranking for its own unique name by day 14 (indexing/metadata bug) |
| Description | Line 1: generic card name + one-line meaning; line 2: site disclaimer verbatim; line 3: chaos-tarot.com link with UTM; then Codex lore name as "also known in the Codex as…"; chapters per spread position | Both name systems become searchable without putting the lore name on screen (brief §8) | ✓ names / ◐ | O: "External" traffic ≥ 5% by day 60 · F: 0 |
| Thumbnail | 16:9 crop of the card (the deck PNGs are 825×1425, ✓ section 1); one consistent frame per pillar; ≤ 3 words; no faces needed (A2) | "consistent title and thumbnail style" ○ 16533387 | ◐ | O: browse CTR ≥ channel median once browse impressions exist · F: browse CTR < 2% (below the page's own lower band) |
| Topical clusters | Publish the 22 Majors in order, then one system at a time; playlists "All 79 cards", one per system, one per pillar | Quality = topical expertise ○ 16090438; Up Next chains inside clusters (◐) | ◐ | O: ≥ 30% of suggested-traffic views come from the channel's own videos by day 60 · F: < 10% |
| Shorts → long-form bridge | Every Short carries a related-video link to the long-form it was cut from; verbal + visual CTA in the last 5 s; long-form pays the promise in 5–10 s | ○ https://support.google.com/youtube/answer/14075157; ○ blog.youtube 2026-07-14 | ○ | O: ≥ 3% of Short viewers click through · F: < 0.5% |
| Loop design | Short's last frame = card back = first frame of the next loop | Shorts ranking inputs are viewed-vs-swiped and how long people watch (○ 12942217; third-party for weighting) | ◐ | O: average % viewed ≥ 90% on card-flip Shorts · F: < 70% |
| End screens | Last 5–20 s: 1 next card + 1 playlist + subscribe; no external link until YPP | ○ 6388789; viewers can hide end screens, so the spoken hand-off comes first ○ SMT | ○ | O: end-screen CTR ≥ 5% · F: < 1% |
| Satisfaction | Never a title the video does not deliver; ask the viewer a question the Oracle answers in the *next* video (open loop across uploads) | Surveys measure "satisfaction, not just watch time" ○ 16089387 | ◐ | O: likes/view ≥ 4% and dislikes/view < 0.5% (⊘) · F: comment pattern of "clickbait" |
| Temporal anchors | Cosmic Calendar events (✓ site) drive one long-form per event; month tag in Shorts titles | Temporal beats evergreen in the Q1-2026 doc (○ Drive) | ○ | O: event videos ≥ 1.5× median views · F: no lift |
| Captions | Auto captions on; correct the card names by hand (ASR will not know "Apockalypse") | "video content" is a relevance input ○ 16090438 | ◐ | O: card names correct in captions · F: mis-transcribed names |
| Comments | Reply to every comment in weeks 1–4; pin a question intake comment (needs advanced features) | Casual/regular tip: "responding to community comments" ○ 13615784 | ○ | O: ≥ 5 usable viewer questions/week by week 4 · F: none |
| Advanced features on day 0 | Phone verify then ID or video verification, before the first upload | Gate for Test & compare, related video, pinned comments ○ 9891124 | ○ | O: advanced features granted in Phase 0 · F: still gated at launch (T1 and the Shorts bridge are blocked) |

### 3.4 Subscriber conversion tactics

Mechanics first (○ §1.8): the desktop hover-to-subscribe is gone (< 0.05% of subs), end screens can be hidden, Shorts show a subscribe control by the handle, and Studio reports **Subscribers per video** (Content tab, Shorts and long-form). So the subscribe decision is won inside the video, and it is measurable per upload.

| Tactic | Mechanism | Tag | O / F |
|--------|-----------|-----|-------|
| **A subscribable promise** | The channel is a numbered series: "79 cards, one at a time; next: 17 Star of Fragmentation, Tuesday". Casual/regular tips: "series from proven topics", "regular publishing schedule" (○ 13615784) | ◐ | O: subs per 1,000 long-form views ≥ 20 by week 8 (⊘) · F: < 5 |
| **Ask at the payoff, once** | The subscribe ask comes right after the reversal beat (the Spike), spoken by Apocky, not the Oracle; never in the first 30 s (Intro metric) | ◐ | O: Spike timestamp precedes the ask in ≥ 90% of long-form · F: asks in the intro |
| **Shorts ask = next card** | Last 3 s of every Short: "the Oracle has a note about [next card]"; related-video link to the long-form | ◐ | O: Shorts "Subscribers" column positive on ≥ 70% of Shorts · F: median 0 |
| **Daily Oracle habit** | Daily Short at a fixed hour keyed to the app's Daily Oracle (✓ site); month hashtag | ✓ feature / ◐ | O: casual share (≥ 2 months watched) ≥ 25% of viewers in month 3 · F: < 10% |
| **Comment intake → next video** | Viewer questions read in the Tuesday long-form; name the commenter | ◐ | O: ≥ 1 comment-sourced segment per long-form by week 4 · F: none |
| **Site loop (product request)** | YouTube link in the site footer and on `/daily-oracle`; embed the day's Short; Resend email to free-tier users (✓ no links exist today) | ✓ gap / ⊘ build | O: ≥ 5% External traffic by day 60 · F: 0 |
| **Hype** | Once in YPP with ≥ 500 subs: ask for a hype in every long-form's first 7 days (○ 15509925) | ○ | O: on a US leaderboard once · F: never |
| **Collab** | One cross-system reading per month with a TTRPG/GM or mid-size tarot channel (section 1 G7); collabs are YouTube's own "new viewers" tip (○ 13615784) | ⊘ | O: 1 collab by day 60 yields ≥ 100 subs · F: none agreed |
| **What not to do** | No sub-for-sub, no "subscribe to unlock the reading", no paid promotion (A4), no giveaways that pull an audience the content cannot keep | ◐ | O: subs-to-views ratio does not spike on any single video · F: a 10× spike followed by views collapse |

### 3.5 Search-vs-suggested mix for a divination channel

Facts: YouTube has never published an ideal traffic mix (○ https://monitoryt.com/blog/youtube-traffic-sources); third-party benchmarks for established long-form channels put browse + suggested at roughly 60–75% of views and search at 5–20%, niche-dependent (○ https://humbleandbrag.com/blog/youtube-traffic-sources — unverified aggregates). The incumbent tarot channels (section 1 §1.3, ○) are pick-a-card / zodiac-timing formats: browse and suggested engines that need an existing subscriber base and 30–60-minute videos. Chaos Tarot cannot copy that on 8–12 h/week (A5) and does not need to.

◐ The mix that fits this channel, by phase (all targets ⊘ until data; measured in Studio → Reach → Traffic source types, long-form and Shorts separately):

| Phase | YouTube search | Suggested | Browse (home/subs) | External (site, TikTok) | Shorts feed | Why |
|-------|----------------|-----------|--------------------|-------------------------|-------------|-----|
| 1 Launch | ≥ 25% of long-form views | any | small | ≥ 3% | separate; dominant by *count* | Zero history → search is the only earnable surface; 79 unique names + 6 systems are the wedge |
| 2 Compound | 25–35% | ≥ 20% | 15–25% | ≥ 5% | | Search viewers' watch history seeds Up Next beside generic tarot videos; clusters chain internally |
| 3 Apply | 20–30% | 25–35% | 20–30% | ≥ 5% | | Casual viewers appear in browse; suggested carries the Majors cluster; search stays the floor |

Pillar mapping (from section 2's pillars): **CD** card decodes and **8S** system guides = search first (evergreen, unique names, content-gap candidates: "Ogham tree oracle meaning", "geomancy figures explained" are low-competition per section 2 ◐); **MC** MAXIMUM CHAOS = Shorts feed and suggested (surprise content, temporal); **DS** shadow-work templates = search + browse (the 31-day #TarotChallenge run in October, ○ Drive doc); **DK** deck/Kickstarter = external + suggested bursts.

Sequencing rule: **search → suggested → browse**, never the reverse. A channel that chases suggested first (pick-a-card, "your person's feelings") competes with 700K-sub incumbents on their surface with no history; a channel that owns its own names is unopposed on search and then rides co-watch into suggested.

**O:** by day 90, long-form search share ≥ 20% **and** suggested share ≥ 20% **and** ≥ 30% of suggested views originate from the channel's own videos. **F:** search < 10% at day 90 (names have no demand; pivot weight to 8S guides and temporal content) **or** suggested < 10% while search ≥ 20% (videos rank but do not satisfy; fix structure via T3 before adding volume).

### 3.6 The loop, stated once

impressions (search first) → CTR (card art + name) → 30-s Intro (payoff in 30 s) → AVD (Pepe Silvia structure) → satisfaction (deliver the title; no fence topics) → watch history of those viewers → Up Next beside generic tarot videos → browse for casual viewers by month 2 → subscribers at the Spike → Daily Oracle habit → casual share → hours. Each arrow has a metric above and an owner in section 7. The loop is per-video (○ 16533387): a bad upload does not poison the channel, and a great one does not carry the next.

**O:** by day 90 the channel shows the loop's signature: search share ≥ 20%, Intro ≥ 65%, AVD ≥ 4 min, casual ≥ 20%, subs/1,000 views ≥ 15 (all ⊘ numbers). **F:** any two of these below half their target at day 60, in which case the format (length, voice split, or topic) is changed before cadence is raised.

---

## 4. Growth dashboard (six numbers, weekly; section 7 formalises the review)

| # | Metric | Where in Studio | Phase 1 floor | Phase 3 target | Tag |
|---|--------|-----------------|---------------|----------------|-----|
| 1 | Long-form qualified watch hours (cumulative) | Analytics → Overview / Monetization eligibility card | 400 h by Nov 1 | 4,000 h before application | ○ thresholds §2.4; ◐ pacing (section 1 §10) |
| 2 | Intro % (30 s) median, long-form | Content → video → Key moments | ≥ 55% | ≥ 70% | ⊘ |
| 3 | "How many chose to view" median, Shorts | Content → Shorts | ≥ 40% | ≥ 55% | ⊘ (third-party 50% average, ○ §1.7) |
| 4 | Search share of long-form views | Reach → Traffic source types | ≥ 25% | 20–30% | ⊘ |
| 5 | Casual viewers share | Audience → New / casual / regular | n/a (month 1) | ≥ 20% | ○ definitions §1.4; ⊘ target |
| 6 | Subscribers per 1,000 long-form views | Content → Subscribers column ÷ views | ≥ 10 | ≥ 20 | ⊘ |

Rule: a metric two weeks below floor triggers the matching test (T1–T5), not a cadence change.

---

## Open questions for Apocky

1. Can you complete ID or video verification for advanced features in Phase 0? Without it there is no Test & compare, no related-video link on Shorts, and no pinned comments (○ 9891124). If verification "isn't available" on the account, say so; the plan falls back to channel history and loses T1 for ~a month (⊘).
2. Do any of the 14 shipped templates get read on-camera by the Oracle? "Career Crossroads" is fine; anything that drifts into money, health, legal or politics is now named by policy (§2.1). Confirm the topic fence applies inside the app's Oracle too, since app capture is on screen.
3. Which name goes on the thumbnail for test 3: shipped ("Tower of Obsolescence") or Codex lore ("The Critical Error")? The brief says shipped on screen; the A/B test would settle it with data. OK to test, or is the shipped name non-negotiable?
4. Unirecall: run the recall command in the rail table locally and paste the JSON. If a prior session already chose a cadence or a handle, it overrides §3.1's Tue/Sat default.
5. Should `TESTS.md` (the pre-registered test ledger) live in this public repo (A6) or in The-Chaos-Tarot (private)? It will contain per-video impression counts.
6. Is a founder voice track (not face) acceptable in every long-form from week 1? It is the cheapest structural defence against "interchangeable" (§2.1) and it is what decides whether A2 survives the July 2026 rules.

## Issue candidates

| Title | Phase | Priority | Acceptance oracle | Falsifier | Depends on |
|-------|-------|----------|-------------------|-----------|------------|
| Unlock advanced features (phone + ID/video verification) before first upload | 0 | P0 | Studio → Feature eligibility shows Advanced features "Enabled" by Oct 4 | Still "Eligible after channel history" at launch | Handle decision (section 1) |
| Topic fence + Oracle-persona rule written into the script checklist (no health/legal/finance/politics; Oracle never presents as human) | 0 | P0 | Checklist item present in every script PR; 0 uploads touching fenced topics | Any upload touching a fenced topic, or an Oracle line claiming human expertise | — |
| Structure-tag rotation for card decodes (≥ 4 formats; no two consecutive uploads share a tag) | 0–3 | P0 | Tracker column "structure" filled for every upload; no consecutive repeats | ≥ 3 consecutive uploads with the same tag | Script system (section 3) |
| AI-disclosure box ticked on every generated-voice/imagery upload | 1–3 | P0 | "How this content was made" line visible on 100% of such uploads | A YouTube-applied label | Production checklist (section 5) |
| `TESTS.md` ledger with pre-registered hypotheses; first Test & compare live in week 2 | 1 | P1 | ≥ 6 conclusive tests by day 90; winner applied to subsequent decodes | All inconclusive at day 60 | Advanced features |
| Related-video link on every Short → its long-form; CTA in last 5 s | 1–3 | P1 | 100% of Shorts carry the link; ≥ 3% click-through | < 0.5% click-through at day 45 | Advanced features; Shorts cut from long-form |
| 22-Major cluster published in order with "All 79 cards" + per-system playlists | 1–2 | P1 | ≥ 30% of suggested views from own videos by day 60 | < 10% | Cadence (section 1 §4) |
| Retention structure test (T3) weekly on the Tuesday long-form; Intro % logged | 1–3 | P1 | Intro median +10 points week 2 → week 8 | Flat/falling after 6 changes | ≥ 100 views per video |
| Shorts hook pair tests (T2), 8 pairs by week 6 | 1–2 | P2 | One hook family wins ≥ 3 of 4 pairs | No consistent winner | Daily Short cadence |
| Search-title order test (T4) on 5 matched pairs | 1–2 | P2 | Winning order raises search impressions ≥ 2× | < 100 search impressions/video at day 14 for both | Research tab access (⊘) |
| Site → channel loop: footer link, `/daily-oracle` embed, Resend email (product request) | 1–2 | P1 | External traffic ≥ 5% by day 60 | 0 external after shipping | Product repo change (not this plan's scope to implement) |
| Traffic-mix gate at day 60 (search ≥ 20%, suggested ≥ 20%, own-suggested ≥ 30%) | 2 | P1 | Gate passed in the day-60 review | Search < 10% or suggested < 10% → pivot per §3.5 F | Dashboard (section 7) |
| Hype ask in every long-form once YPP + 500 subs | 3+ | P3 | Appears on a US Hype leaderboard once | Never | YPP acceptance |

## CSL annex

Σ: per-video judgement (fresh data, no momentum) → earn search on 79 unique names + 6 systems → satisfy → co-watch into suggested → casual by month 2 → subs at the Spike → hours; test with native Test & compare (watch-time winner, long-form only), Shorts by sequential hook pairs, structure by key moments.
W! advanced features on day 0 · disclosure box on every synthetic upload · ≥ 4 rotating decode structures with a human layer · related-video link on every Short · search → suggested → browse, in that order · six-number weekly dashboard · pre-registered `TESTS.md`.
N! no AI Oracle on health/legal/finance/politics (July 2026 bucket 3) · no interchangeable templated decodes (bucket 1) · no shock/doom packaging (bucket 2) · no reused pick-a-card clips · no cadence bursts to "feed the algorithm" · no thresholds or benchmarks from memory (○ URLs in §1–§2).
