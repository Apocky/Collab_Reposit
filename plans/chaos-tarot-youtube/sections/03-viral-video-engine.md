# 3. Viral Video Creation Engine

## Source prompt

> "Act as a top 1% YouTube content strategist and viral video producer. Create a complete system for producing videos with high click-through rates and strong audience retention. Generate video concepts, irresistible hooks, titles, thumbnail concepts, opening scenes, storytelling structures, retention techniques, CTAs, and full scripts. Analyze why each idea could attract viewers and keep them watching.

(Prompt 3 of the thread, transcribed verbatim in `prompts/SOURCE_PROMPTS.md`; no closing quote in the source.)

## Assumptions used

| # | From brief | How this section uses it |
|---|-----------|--------------------------|
| A1 | No channel exists | Hooks and scripts assume a cold audience with zero brand recognition; every video re-introduces the deck in one line. |
| A2 | Faceless-first: AI Oracle voice + card art + screen capture | Two voices are scripted: NARRATOR (Apocky, Signal register, deadpan, can be text-on-screen or own voice) and ORACLE (the app persona). No on-camera beats. If A2 is wrong, opening scenes 1, 3 and 5 get a face and the thumbnail set adds a face template. |
| A3 | Goal order YPP → funnel → deck/Kickstarter | CTA stack is ordered free tier → Oracle sub → deck. If Kickstarter is imminent, swap CTA order in scripts B and C (see §10). |
| A4 | Near-zero cash | Thumbnails use the shipped card art only; no stock, no paid fonts. |
| A5 | 8–12 h/week | The system is sized for 1 long-form + 3 Shorts/week; hook/title/thumbnail banks are meant to be reused, not reinvented per video. |
| A8 | Physical deck = 79 = 78 + 1; Singularities are lore | Script A treats the 79th card as the only "extra" card. Singularities are not shown on screen. |

Local assumptions added by this section (⊘ until Apocky confirms): **L1** the MAXIMUM CHAOS toggle is reachable on chaos-tarot.com today (see §1, contradiction C1); **L2** the 79th card's on-screen title is what the art says ("XXII The Apocalypse"), with "Null" and the "Apockalypse" pun used as lore, not as the caption (see C2); **L3** chaos-tarot.com can accept UTM query strings without breaking the access-code flow (see §10).

Unirecall: no Unirecall tool is exposed in this session's toolset (searched; the brief §6 reports the same for the parent session). No prior channel decisions were recalled. Everything below is from the repo, the brief, and the URLs cited.

---

## 0. Verification log (this session)

### 0.1 Product facts the creative system must respect

| Fact | Evidence | Tag |
|------|----------|-----|
| Card art files: 22 Majors (0 The Zero Point … 21 World of the Glitch), 56 Minors in suits **Codes / Networks / Signals / Vectors**, plus `Null The Apockalypse.jpg`, `Page of Fractures.jpg`, `Card Back.jpg` | `ls /home/user/the-chaos-tarot/New/` | ✓ |
| **The face of the "Null" card reads "XXII THE APOCALYPSE"** (no "Null", no "Apockalypse" spelling on the art). Composition: figure with a black void head, radiation-symbol drum torso, walking through a burning ruined city; galaxy of eyes in the sky; inverted cathedral; clocks spilling off the right edge; melting skyscrapers; books in the rubble. | image viewed | ✓ |
| Null / XXII is **absent from every card dataset the app reads**: `shared/deck.js` has exactly 78 entries; `supabase/migrations/002_seed_tarot_cards.sql` and `app/data/card-meanings.js` have no Null/Apocalypse row. The AI Oracle cannot draw or interpret it. | grep, count | ✓ |
| Tower of Obsolescence art: a tower of stacked CRT monitors and server cases struck by lightning, two human figures falling, sea of dead monitors below, code-glitch vortex sky. | image viewed | ✓ |
| The Zero Point art: figure wrapped in dictionary-page bandages, brass diving helmet, blue bowler hat, stepping off a stack of dictionaries; trail of disembodied eyes on the floor. | image viewed | ✓ |
| Wheel of Misfortune art: rusted gear-wheel with two analog clocks and two PC fans; CRT-headed figure in a crown on top; suited man crushed beneath; browser-error windows in the vortex sky. | image viewed | ✓ |
| Card back: purple/teal vortex, circuit traces, four clock faces at the corners, pixel-eye with a QR-like iris at the centre. | image viewed | ✓ |
| Codex §1.2: "the binary concepts of 'Upright' and 'Reversed' are discarded in favor of a dynamic signal-processing metaphor: the **Signal** and the **Glitch**." Signal = "the distinct image on the screen, the clear audio track, the executed code." Glitch = "the pixelation of the image, the stutter in the audio, the malware in the system… Is the Emperor's structure becoming tyranny (rigid static)? Is the Fool's freedom becoming nihilism (void noise)?" | `book/The Chaos Codex…md` lines 15–19 | ✓ |
| Codex §2.0 Fool/Zero Point Glitch text: "Recklessness masquerading as bravery… This is a reboot without saving progress." | same file, §2.0 | ✓ |
| **The React app labels the mechanic "Reversed"**, not Glitch (`src/components/ReadingResults.tsx:129`, `CardDetailModal.tsx:226`; `isReversed` throughout `src/types/tarot.ts`). Site copy says "reversals and merkstave". | grep | ✓ |
| **MAXIMUM CHAOS exists in code**: legacy `app/premium.js` toggle "Maximum Chaos", `app/core/settings.js` label, `api/ai-reading.js` "PEPE SILVIA PROTOCOL ENGAGED"; `docs/ADMIN_SETUP.md`: "Three modes: Standard, Unhinged, MAXIMUM CHAOS". **The React edge function `supabase/functions/interpret-reading/index.ts` offers styles psychological / predictive / balanced / spiritual only.** Last commit "Consolidate to single React app for production". | grep | ✓ code; ⊘ whether live |
| Oracle voice anchors: opening "The datastream acknowledges your query, Seeker. The cards have been drawn from the void between cycles..."; closing "The protocol completes. Remember: the future is unwritten code—you hold the commit privileges."; "There is no 'Game Over,' only a 'Respawn' or 'Reload Save.'"; Glitch Filter: "Describe the card as 'corrupted,' 'pixelated,' or 'inverted'. Frame the reversal as a 'system error' that reveals a deeper truth." | `docs/AI_ORACLE_TRAINING_GUIDELINES.md` | ✓ |
| MAXIMUM CHAOS voice anchors: "Red String Everywhere", "Escalating Revelation", "Manic Energy - Short bursts, interruptions, 'WAIT', 'NO LISTEN', 'YOU SEE?'", "The Oracle is Charlie Kelly but they're RIGHT about Pepe Silvia." Example reading: "16 minus 5 is 11… WHO PUT THAT THERE?… This isn't a reading, this is a DIAGRAM… It's THE WALL COMING DOWN… unless that's what they want you to think." | `docs/CHAOS_ORACLE_VIBE.md` | ✓ |
| Week Ahead template: id `weekly-outlook`, 3 cards (Beginning of Week / Midweek / End of Week), `premium: false`. Premium templates: Shadow Work Deep Dive, Monthly Cycle, Creative Unblocking. | `shared/reading-templates.json` | ✓ |
| Site: 79-card deck; Free tier (all systems, spreads, daily draws, streaks, collection, community); Oracle $3.33/mo or $2.22/mo "with CSL" (unlimited AI, Memory Core, follow-ups, journal, PDF export); deck sold via Stripe, US shipping; disclaimer "entertainment and personal reflection only". | BRIEF §1 (site fetched 2026-09-27 by the brief's author) | ✓ via brief |
| Suit → element mapping conflicts four ways: SQL seed Signals=Wands, Vectors=Cups, Codes=Swords, Networks=Pentacles; `shared/deck.js` Codes=Wands, "Glitches"=Swords, "Voids"=Cups, Networks=Pentacles; art suits Codes/Networks/Signals/Vectors; Codex Vectors/Frequencies/Signals/Artifacts. | grep | ✓ (contradiction C3) |

Contradictions this section must route around:

| # | A | B | Handling in this section |
|---|---|---|--------------------------|
| C1 | MAXIMUM CHAOS shipped in legacy app + API | React production path has no chaos style | Script B is gated on L1. If not live, Script B is recorded against the legacy API or held until the mode ships in React (Issue I-01). |
| C2 | Filename "Null The Apockalypse"; brief calls it "Null — The Apockalypse" | Card face reads "XXII THE APOCALYPSE" | On-screen caption = the art. The number XXII is the hook (Majors run 0–21). "Null" = what the deck files call it, said aloud, never captioned as the title. Apocky ratifies (Issue I-02). |
| C3 | Four suit→element mappings | — | No suit-lore video until ratified (Issue I-05). Scripts A–C avoid asserting any suit's element. |
| C4 | Codex: no Upright/Reversed, only Signal/Glitch | App UI says "Reversed" | Script C says it out loud (one honest line) and the relabel is a product request (Issue I-04). |

### 0.2 Policy facts (September 2026)

| Claim | Source | Tag |
|-------|--------|-----|
| YPP ad-revenue threshold today: "Get 1,000 subscribers with 4,000 qualified watch hours in the last 12 months, or Get 1,000 subscribers with 10 million qualified Shorts views in the last 90 days." Also: "YouTube may turn off monetization on channels that haven't uploaded a video or posted to the Posts tab for 6 months or more." | https://support.google.com/youtube/answer/72851 (fetched) | ○ |
| From **1 Feb 2027** new applicants need 8,000 qualified watch hours (365 d) or 20 million Shorts views (90 d); 1,000 subs unchanged; existing partners unaffected if they accept terms by 31 Jan 2027; Shorts revenue share gated at 10M Shorts views/90 d. The Google page confirms the date, not the numbers. | https://blog.youtube/news-and-events/youtube-partner-program-updates-2027-new-opportunities-earn/ (cited by https://air.io/en/monetization/youtube-partner-program-requirements-2026-the-complete-guide and https://vidiq.com/blog/post/youtube-partner-program-changes-2027/ ; blog not fetched directly) | ○ |
| Consequence for this plan: the 90-day horizon from 2026-09-27 ends ~2026-12-26, before the threshold doubles. Qualifying before 1 Feb 2027 is worth roughly a 2× discount on the bar. | arithmetic on the two rows above | ◐ |
| Fan-funding tier (500 subs, 3 uploads/90 d, 3,000 h or 3M Shorts views) | third-party guides only (air.io, vidiq); not seen on the Google page fetched | unverified; not on the critical path |
| Inauthentic content (renamed from "repetitious content" 15 Jul 2025): ineligible examples include "AI-generated content made with generic or unoriginal templates giving the impression of mass production without adding the creator's original, authentic insights or perspective", "Image slideshows with minimal narrative or commentary", "Videos using highly similar storyline templates repeatedly". | https://support.google.com/youtube/answer/1311392 (fetched); context https://www.socialmediatoday.com/news/youtube-clarifies-monetization-update-inauthentic-repeated-content/752892/ | ○ |
| AI disclosure: must disclose realistic synthetic content that "Makes a real person appear to say or do something they didn't do", "Alters footage of a real event or place", "Generates a realistic scene that didn't actually occur". Not required: "Cloning one's own voice to create voice overs or dubs", "Production assistance, like using generative AI tools to create or improve a video outline, script, thumbnail, title, or infographic", fully animated/fantastical content. Non-photorealistic disclosures are labelled in the expanded description; photorealistic on the player. | https://support.google.com/youtube/answer/14328491 (fetched) | ○ |
| Advertiser-friendly guidelines: the page **does not mention** tarot, astrology, occult, divination, psychic or spirituality. Restricted categories are the 14 listed (inappropriate language, violence, adult, shocking, harmful acts and unreliable content, hateful, drugs, firearms, controversial issues, sensitive events, dishonest behavior, kids/families, tobacco, incendiary/demeaning). | https://support.google.com/youtube/answer/6162278 (fetched) | ○ |
| Claim that "astrology, astronomy, divination and tarology are borderline" comes from a Medium post about Google Ads campaign accounts, not YouTube monetization. Treated as weak. | https://medium.com/@marketolog4limes/the-tarot-magic-of-youtube-advertising-how-we-got-845-registrations-for-a-free-tarot-video-course-e1624e819f0e | ○ weak |
| Shorts: "a square or vertical aspect ratio up to three minutes in length will be categorized as Shorts", for uploads after 15 Oct 2024. | https://support.google.com/youtube/answer/15424877 (fetched) | ○ |
| Shorts hook heuristics: "50-60% of viewers who drop off do so within the first three seconds" (OpusClip); best-performing length 30–60 s in a 5,400-Short sample (Piktochart); "intro retention… ideally above 70%" (prepublish.ai). Third-party, not YouTube data. | https://www.opus.pro/blog/ideal-youtube-shorts-length-format-retention ; https://piktochart.com/blog/how-long-youtube-shorts/ ; https://prepublish.ai/guides/youtube-shorts-retention | ○ heuristic |

What these do to the creative system (◐ from the rows above):

- The channel's shield against "inauthentic content" is **Apocky's own input per video**: a real draw, a real week, a real opinion, and the deck's own lore. Templates are allowed; template-only videos are not. Every script below has a mandatory "author's insight" beat marked **[AUTH]**.
- The ORACLE voice is a fictional persona over non-photorealistic card art. Under the fetched policy it does not depict a real person or event, so the label is not required; **disclose anyway** (toggle on, description block) because the cost is a line in the expanded description and the trust benefit is large for a tarot audience primed to suspect AI slop. Oracle: no "AI generated" comment storm on the first 10 videos. Falsifier: a video is flagged or demonetized for missing disclosure.
- Ad suitability risk is not "tarot"; it is drifting into "unreliable content" (health, money, legal claims). Site framing carries over verbatim: every long-form description and end screen carries "Chaos Tarot is for entertainment and personal reflection only."

---

## 1. Video concepts (format families)

| Family | Pillar | Example concept | Why it attracts | Why it retains | Primary CTA |
|--------|--------|-----------------|-----------------|----------------|-------------|
| **Lore drop** (Short + long) | The deck | "The card that shouldn't exist" (Script A); "Why the Fool wears dictionary pages" | Specific, visual, contrarian claims about a thing the viewer can see | One image, one revelation per 10 s; art is dense enough to zoom | Free draw |
| **Oracle reads X** (long) | The app | "I let a chaos oracle read my week" (Script B); "The oracle read my inbox" | Voice contrast (deadpan vs manic) is clippable | Escalating revelation structure; day-by-day payoff | Oracle sub |
| **Mechanic explainer** (long) | The system | "Signal vs Glitch" (Script C); "Merkstave vs reversal vs Glitch" | Search intent ("reversed tarot meaning") + a real point of difference | Argument with examples; app demo mid-video | Free tier → sub |
| **Daily Signal** (Short) | Habit | Today's card, 20–35 s, Signal voice, dated on screen | Temporal anchoring outperforms evergreen framing (brief §5, Q1 2026 doc) | Same skeleton daily; different card; streak call-out | Daily draw (free) |
| **Cross-system collision** (long) | Eight systems | "Tarot said Tower. I Ching said Hexagram 23. Runes said Hagalaz. Three systems, one warning." | "Eight divination systems. One digital oracle." made visible | Three mini-cliffhangers, one per system | Free tier |
| **Maximum Chaos clips** (Short) | Voice | 30–60 s cuts of the ORACLE mid-revelation from long-forms | Meme-able, quotable, absurdist (Q1 quadrant) | Cut at peak; no intro; caption the string | Long-form link |
| **Deck in hand** (Short + long) | Physical | Unboxing the "Void Matte" stock, holographic black foil edges, magnetic box (Codex spec; ⊘ matches shipped product) | Tactile ASMR + dark aesthetic swing (brief §5) | Reveal cadence: box → edges → card back → one Major | Deck |
| **TTRPG table** (long, later) | Codex §6 | "Sanity Check with a tarot deck" | Adjacent audience (tabletop) | Session-play structure | Deck |

Oracle for the family set: after 12 uploads, at least three families each have one video above channel-median CTR. Falsifier: one family carries >70% of views and the rest are flat, meaning the channel is a single format wearing eight hats.

---

## 2. Hook library (24 hooks)

Rules: spoken in ≤ 3 s; the first frame already shows the thing the hook names; no "hey guys"; Signal hooks are calm and declarative; MAXIMUM CHAOS hooks start mid-thought. All card names are the shipped art titles.

### 2.1 Signal-mode hooks (12)

| # | Hook (VO) | First frame | Why it works | Fits |
|---|-----------|-------------|--------------|------|
| S1 | "There is no card twenty-two in tarot. This deck has one." | XXII The Apocalypse face, full bleed | Verifiable claim (Majors run 0–21) + visible proof | Script A |
| S2 | "This deck has no reversed cards. It has something worse." | Card flipping into a glitch-smear | Contradiction of a thing every tarot viewer knows | Script C |
| S3 | "The datastream acknowledges your query, Seeker." | Black; text cursor blinking | In-world cold open; the guideline's own sample opening | Daily Signal |
| S4 | "Seventy-eight cards are tarot. The seventy-ninth is a warning." | Deck fanned, XXII on top | Numbers viewers can check against the site copy | Lore drop |
| S5 | "Your Tower isn't a disaster. It's a system crash. Those are different." | Tower of Obsolescence, lightning frame | Reframes the most-feared card; guideline line | Explainer |
| S6 | "I asked an AI oracle the same question every day for a week. It remembered." | App journal scrolling | Memory Core made concrete | Oracle reads X |
| S7 | "Eight divination systems. One digital oracle. Watch it disagree with itself." | Split screen tarot / runes / I Ching | Site tagline, weaponized | Cross-system |
| S8 | "The Fool in this deck is wrapped in dictionary pages. That's not decoration." | Zero Point, tight on the bandages | Visual detail the viewer just noticed | Lore drop |
| S9 | "The last card in this deck is a man with no face walking out of a burning city." | XXII, slow push | Pure image description; curiosity about meaning | Script A alt |
| S10 | "This is what a reading looks like when the reader is a sysadmin." | Terminal-style reading output | "Dual-Boot Philosophy" in one line | App walkthrough |
| S11 | "Wisdom is found not in the pattern, but in the break of the pattern. Here is the break." | Static burst → card | Codex quote as thesis | Script C |
| S12 | "Runes have merkstave. Old tarot has reversals. This deck has Glitches. Three different things." | Three cards side by side | Ties the site's own feature list into a distinction | Script C alt |

### 2.2 MAXIMUM CHAOS-mode hooks (12)

| # | Hook (VO) | First frame | Why it works | Fits |
|---|-----------|-------------|--------------|------|
| M1 | "OKAY. Okay. The Tower is card SIXTEEN. Your week has SEVEN days. Sixteen minus seven is NINE. The Hermit. WHO PUT THE HERMIT THERE?" | Tower + Hermit of the Void with red string overlay | Numerology gag from the vibe doc pattern; ends on a question | Script B cold open |
| M2 | "I didn't put that there. YOU didn't put that there. So WHO put that there?" | Card mid-flip | Vibe-doc line; works with any card | Clips |
| M3 | "This isn't a reading. This is a DIAGRAM." | Spread with string overlay | Vibe-doc line; visual promise | Clips |
| M4 | "There are eyes in the sky on the last card. Count them. COUNT THEM." | XXII sky, zoom | Forces a rewatch/pause (retention) | Lore clip |
| M5 | "The cards know. THE CARDS HAVE ALWAYS KNOWN." | Card back vortex, pixel eye | Vibe-doc line; the card back literally has an eye | Any |
| M6 | "The Wheel of Misfortune has TWO clocks on it. Two. You have two deadlines this week. I'm not saying it's connected. I'm saying LOOK AT THE STRING." | Wheel, clocks circled | Verified art detail + relatable | Script B |
| M7 | "Everyone says the Tower means destruction. Destruction of WHAT? Of the SURVEILLANCE." | Tower, monitors circled | Vibe-doc reframe; the art is literally monitors | Script B |
| M8 | "The Zero Point is a guy in a diving helmet stepping off a dictionary. That's you. That's you on MONDAY." | Zero Point | Verified art + day anchor | Script B |
| M9 | "The oracle remembered my reading from last week and I have not been okay since." | Journal screen | Memory Core as horror-comedy | Oracle reads X |
| M10 | "Three cards. THREE. You know what else has three? Every conspiracy that was ever RIGHT." | Week Ahead spread | Structure joke | Script B |
| M11 | "WAIT. Go back. Go BACK. The Devil's Algorithm is FIFTEEN. Rent is due on the FIFTEENTH." | Devil's Algorithm, red string to a calendar | Number-to-life mapping | Clips |
| M12 | "...unless that's what they want you to think." | Freeze frame, static | Vibe-doc closer; works as an out-hook to the next video | End of any clip |

Oracle for the library: the top-quartile hooks by 3-second retention (Shorts "viewed vs swiped away") get reused within 30 days; the bottom quartile are retired. Falsifier: after 20 Shorts no hook reaches the 70% swipe-survival heuristic (○ prepublish.ai), which means the hooks are not the problem, the first frame is.

---

## 3. Title formulas

| Formula | Example | Why it works | Guard |
|---------|---------|--------------|-------|
| **[Impossible thing] + [proof noun]** | "The Tarot Card That Shouldn't Exist (Card XXII)" | Claim + a checkable token in parentheses | Never title a card by a name not on the art |
| **[Common belief] is wrong: [our term]** | "Reversed Tarot Cards Don't Exist in This Deck. Glitches Do." | Search intent on "reversed tarot" + differentiation | Must deliver the mechanic by 1:00 |
| **I let [oracle] [verb] my [ordinary thing]** | "I Let a Chaos Oracle Read My Week (It Got Weird)" | First-person stakes; parenthetical promises tone | The week must be real (**[AUTH]**) |
| **[Card name]: [what the art shows]** | "Tower of Obsolescence: Why It's Made of Monitors" | Art-first; unique card names are un-competed search terms | One card per title |
| **[N] systems, [1] question** | "8 Divination Systems, 1 Question, 8 Different Answers" | Site tagline turned into a test | Show all eight or change N |
| **[Day/Date] + Signal** | "Monday's Signal: Hermit of the Void" | Temporal anchoring (brief §5) | Daily only; never evergreen |
| **MAXIMUM CHAOS: [mundane input]** | "MAXIMUM CHAOS Reads My Grocery List" | Brand term as prefix; absurdist quadrant | Only for Maximum-Chaos-voiced videos |
| **What [card] means when it Glitches** | "What The Zero Point Means When It Glitches" | Long-tail per card, 78 possible titles | Use Codex Glitch text; app label caveat in description |

Oracle: titles with a checkable token (number, card name) outperform titles without one on CTR within the same family. Falsifier: parenthetical-free titles win consistently, in which case drop the tokens.

---

## 4. Thumbnail concepts (existing card art only)

Base rules (◐ from A4 and the art viewed): card art is 825×1425 portrait at 300 dpi (`deck/manifest.json` ✓), so a 16:9 thumbnail crops the card, never shrinks it. Text ≤ 3 words, glitch-cut typeface like the card titles. Background: the card-back vortex (✓ exists) as a universal backdrop. No faces (A2). Purple/teal/acid-green palette is already in the art.

| # | Concept | Art used | Composition | Overlay | Why it works |
|---|---------|----------|-------------|---------|--------------|
| T1 | **The number that shouldn't be there** | XXII The Apocalypse | Crop to the void-head figure and the "XXII" title; the eyes-galaxy fills the top third | "XXII?" | Curiosity from a single glyph; the art's own title does the work |
| T2 | **Signal / Glitch split** | Any Major, e.g. The Zero Point | Left half clean, right half smeared/pixelated (datamosh); a hard vertical seam | "SIGNAL · GLITCH" | Shows the mechanic without a word of explanation |
| T3 | **Red string board** | Week Ahead spread (3 cards) | Three cards pinned on the card-back vortex; red lines converging on one circled clock (Wheel) | "IT KNEW" | Pepe Silvia iconography; instantly reads as the Maximum Chaos family |
| T4 | **Falling from the Tower** | Tower of Obsolescence | Crop to lightning + the two falling figures + monitor stack | "NOT A DISASTER" | Fear image + contradicting text |
| T5 | **The eye** | Card back | Centre pixel-eye, everything else blurred; the clocks at the corners visible | "8 SYSTEMS" | Brand mark; use for cross-system videos so the series is recognisable |
| T6 | **Dictionary man** | The Zero Point | Crop to the diving helmet and the foot leaving the dictionary stack | "MONDAY." | Absurd + relatable; pairs with hook M8 |
| T7 | **Two clocks** | Wheel of Misfortune | Tight crop on the two clock faces and the crowned CRT head | "TWO DEADLINES" | Concrete, verified detail; pairs with M6 |
| T8 | **Deck in the dark** | Physical deck photo (to shoot) on black | Black foil edges catching one light; one card half-drawn | "79" | Product shot for deck CTAs; the number is the promise |

Oracle: T1/T2/T3 as a family reach ≥ channel-median CTR on their videos; the split (T2) becomes the recurring explainer thumbnail. Falsifier: thumbnails with text outperform text-free ones by a wide margin, meaning the art alone does not read at feed size and the crops must get tighter.

---

## 5. Opening-scene patterns

| # | Pattern | Length | Beats | Use |
|---|---------|--------|-------|-----|
| O1 | **Card-first cold open** | 0–5 s | Frame 1 is the card; VO states the claim; title card only after the claim | Lore drops, Script A |
| O2 | **Oracle boot** | 0–8 s | Black, cursor, the guideline's opening line, then the app UI fades in | Daily Signal, app walkthroughs |
| O3 | **In-medias-res rant** | 0–12 s | A peak Maximum Chaos moment from later in the video; hard cut to "10 minutes earlier" | Script B, Oracle reads X |
| O4 | **Two-voice contradiction** | 0–10 s | NARRATOR states the boring version; ORACLE interrupts with the deck's version | Explainers, Script C |
| O5 | **Count with me** | 0–8 s | "Count the eyes / clocks / cards" instruction; viewer pauses | Shorts, M4 |
| O6 | **Quote over static** | 0–6 s | Codex line typed on screen over white-noise texture; card reveals under it | Lore, Script C alt |

Oracle: 30-second retention on long-forms using O3/O4 ≥ 65% (⊘ planning target, not a benchmark). Falsifier: O3 videos lose more viewers at the "10 minutes earlier" cut than at any other point, meaning the tease is overpromising.

---

## 6. Storytelling structures

### 6.1 Reading video (long, 8–10 min)

| Beat | Time | Content | Retention device |
|------|------|---------|------------------|
| Cold open | 0:00–0:30 | O3 peak clip | Promise of the peak |
| Setup | 0:30–1:15 | Template named on screen (real template from the app), question typed, mode chosen | "Rules of the game" stated in one breath |
| Card 1 | 1:15–2:45 | Reveal → art detail → interpretation → first "string" | Open loop: the string is left dangling |
| Card 2 | 2:45–4:15 | Reveal → connect back to card 1 → escalate | Callback pays loop 1, opens loop 2 |
| Card 3 | 4:15–5:45 | Reveal → the big pattern → "the wall" | Peak from the cold open arrives here |
| Follow-up | 5:45–7:00 | One follow-up question (sub feature) | Voice change: brief calm, then re-escalation |
| **[AUTH]** Reality check | 7:00–8:30 | What actually happened, day by day, Apocky's words | Payoff of every loop; the honest part |
| Verdict + close | 8:30–9:30 | Score the oracle; guideline closing line; CTAs; disclaimer | Out-hook to next video (M12) |

### 6.2 Lore video (long, 6–8 min)

| Beat | Time | Content |
|------|------|---------|
| Claim | 0:00–0:25 | One contrarian sentence (S2/S11) |
| The old way | 0:25–1:00 | What tradition says, stated fairly and briefly |
| The Codex way | 1:00–3:30 | Quote the Codex; two or three cards as worked examples; art zooms |
| Why it changes a reading | 3:30–4:30 | Guideline's Glitch Filter; a before/after interpretation |
| **[AUTH]** In the app | 4:30–5:30 | Live draw; show the mechanic; say the honest caveat if the UI differs |
| Philosophy | 5:30–6:30 | The "holy interruption" idea; one Codex line to close |
| CTA | 6:30–7:00 | Free draw → sub → deck |

### 6.3 App-walkthrough video (long, 5–7 min)

| Beat | Time | Content |
|------|------|---------|
| O2 boot | 0:00–0:08 | Oracle opening line, UI fades in |
| One question | 0:08–0:40 | Type a real question; pick a real template (name on screen) |
| Draw | 0:40–2:00 | Shuffle animation, reveal cadence, one Glitch |
| Interpretation | 2:00–3:30 | Read the oracle output aloud in Signal voice; pause on one sharp line |
| **[AUTH]** Follow-up + Memory Core | 3:30–4:45 | Ask a follow-up; show the journal and a prior reading being referenced |
| Export/share | 4:45–5:30 | PDF export or shareable image (both exist ✓ brief §1) |
| CTA | 5:30–6:00 | Free tier link; "$3.33/month" said once, plainly |

### 6.4 Short (30–60 s)

| Beat | Time | Content |
|------|------|---------|
| Hook | 0–3 s | One claim, card on screen |
| Proof | 3–15 s | The visible detail that makes the claim true |
| Turn | 15–35 s | The meaning, one twist |
| **[AUTH]** Line | 35–48 s | One sentence only Apocky would say about it |
| Close | 48–60 s | Site name spoken once; open loop to comments |

Oracle for the four structures: each is used at least twice in the first 30 days and its retention curve is flat between the Setup and the [AUTH] beat (no cliff). Falsifier: a recurring cliff at the same timestamp across videos of one structure.

---

## 7. Retention techniques

| # | Technique | Where | Why | Measured by |
|---|-----------|-------|-----|-------------|
| R1 | **Open loops on the string**: every card leaves one unexplained connection; the next card pays it | Reading videos | Escalating-revelation is the documented Maximum Chaos shape (✓ vibe doc) | Drop-off between card beats |
| R2 | **Art zooms with a countable detail** ("two clocks", "count the eyes") | All | Forces a pause/rewatch; the art rewards it (✓ viewed) | Rewatch spikes in the retention graph |
| R3 | **Two-voice rhythm**: NARRATOR deadpan ≤ 8 words between ORACLE runs | Reading, explainer | Contrast resets attention without a cut | AVD vs single-voice videos |
| R4 | **Timestamped chapters named as cards** ("1:15 The Zero Point") | Long-form | Viewers skip to a card instead of leaving | Chapter click-through in analytics |
| R5 | **Reality-check payoff placed late** (7:00+) | Reading | The honest part is the reward; announced in setup | Retention at 7:00 vs 4:00 |
| R6 | **No intro, no channel branding until after beat 1** | All | Cold audience (A1) has no reason to wait | 30-second retention |
| R7 | **Codex quote card as a breath** (3 s of typed text on static) | Lore | Pacing valve before a new argument | Retention flat across the quote |
| R8 | **The caveat, said fast and once** ("the app still calls it Reversed") | Script C | Honesty prevents the "actually…" comment exodus | Comment sentiment |
| R9 | **Out-hook = M12** ("…unless that's what they want you to think") into an end screen | All long-form | Turns the ending into a beginning | End-screen CTR |
| R10 | **Shorts at 30–60 s, never 3 min** | Shorts | Third-party length data (○ Piktochart) plus faster iteration | Viewed-vs-swiped, AVD |

Oracle: average view duration on the second batch of long-forms exceeds the first batch's. Falsifier: AVD is flat or down after applying R1–R10, meaning the content, not the technique, is the ceiling.

---

## 8. CTAs

Targets in A3 order. Wording is in-voice. Placement is fixed per structure. Tracking is ⊘ until L3 is confirmed (does the site keep UTM parameters through the access-code flow?).

| Target | Wording (Signal) | Wording (Maximum Chaos) | Placement | Landing | Tracking |
|--------|------------------|-------------------------|-----------|---------|----------|
| **Free tier** | "Pull a card free at chaos-tarot.com. All eight systems. No card required, just the cards." | "Go to chaos-tarot-dot-com. It's FREE. Why is it free? …I don't know. I don't LIKE that I don't know." | Every video, spoken once at the close; pinned comment; first line of description | chaos-tarot.com (Try Free) | `?utm_source=youtube&utm_medium=video&utm_campaign=<slug>` (⊘ L3) |
| **Oracle sub** | "Follow-ups, Memory Core, the journal and PDF export are the Oracle: three dollars thirty-three a month." | "Memory Core REMEMBERS. Three thirty-three a month. Three three three. I didn't pick that number. I'm not saying who did." | Only in videos that demo a sub feature (follow-up, Memory Core, export), immediately after the demo | Pricing page | same UTM, `utm_content=oracle` |
| **Deck** | "Seventy-nine cards, printed. Void Matte stock, black foil edges. Link below. Ships in the US." | "The physical deck has EDGES. Holographic BLACK. You can't screenshot an edge." | Deck-in-hand videos and Script A; end screen elsewhere | Deck purchase (Stripe) | `utm_content=deck` |
| **Kickstarter** (if A3 flips) | "The Kickstarter is live. Link below." | — | Replaces the deck CTA while live | Campaign URL (⊘ unknown) | `utm_content=ks` |

Rules: one spoken CTA per video; the other two live in the description and end screen. Never put the sub CTA before the feature it sells has been shown. The disclaimer sentence from the site sits in every description verbatim.

Oracle: ≥ 1% of long-form viewers click a description or end-screen link (⊘ planning target) and the UTM shows up in site analytics. Falsifier: zero attributable traffic after 10 videos, which means either the links are not being tracked (fix L3) or the CTA is being skipped (move it to before the reality-check beat).

---

## 9. Script A — 60-second Short: "Null — The Apockalypse: the card that shouldn't exist"

Working title on YouTube: **"The Tarot Card That Shouldn't Exist (Card XXII)"**. Thumbnail T1. Voice: Signal only. Aspect 9:16. On-screen caption for the card is the art's title (C2/L2). "Null" is spoken as the file name, never captioned as the card's name. The card's meaning below is a writer's proposal for the Codex (⊘, Issue I-02); the Codex has no entry for this card.

| Time | Visual | VO (Signal) | On-screen text |
|------|--------|-------------|----------------|
| 0:00–0:03 | Card back vortex, 1 frame; hard cut to XXII The Apocalypse, full bleed, glitch pop on the title | "There is no card twenty-two in tarot. This deck has one." | — |
| 0:03–0:08 | Push into the "XXII" title; then a strip of Major titles scrolling: 0 The Zero Point … 21 World of the Glitch | "The Major Arcana run zero to twenty-one. Twenty-two cards. This is the twenty-third." | "0 → 21 = 22 cards" |
| 0:08–0:15 | Cut to a file browser: `Null The Apockalypse.jpg` highlighted; then the app's card list scrolling, 78 cards, no XXII | "In the deck files it has no number at all. It's called Null. The app that reads this deck doesn't even know it exists." | "NULL" |
| 0:15–0:25 | Slow push: black void head → radiation drum torso → galaxy of eyes in the sky → inverted cathedral → clocks running off the frame | "A man with no face walks out of a burning city. A sky full of eyes. A cathedral upside down. The clocks aren't stopped. They're leaving." | — |
| 0:25–0:37 | Three cards in a row: 13 Death the System Crash, 16 Tower of Obsolescence, XXII | "Death is thirteen: a system crash. The Tower is sixteen: a structure that outlived itself. Null is what you draw when the question was wrong. Not 'what happens next.' 'Why did you assume there was a next.'" | "13 → 16 → NULL" |
| 0:37–0:46 | Codex line typed over static; card beneath | "The Codex says wisdom is found 'not in the pattern, but in the break of the pattern.' This card is the break." | "not in the pattern, but in the break of the pattern" |
| 0:46–0:52 | **[AUTH]** Physical deck fanned on black, XXII on top | "Seventy-nine cards. Seventy-eight of them are tarot. One of them is the deck telling on itself. I put it there so the deck could lose an argument." | "79" |
| 0:52–0:60 | End card: site name; comments prompt | "Pull a card free at chaos-tarot.com. If you ever draw this one, tell me in the comments. You can't. Yet." | "chaos-tarot.com · free draw" |

Spoken words: ~165 (fits 60 s at ~2.8 w/s). Disclosure toggle: on (non-photorealistic; label in description). Description block: site disclaimer verbatim + deck link + free link.

Why it attracts (◐): the hook is a checkable fact (Majors are 0–21) against a visible image with "XXII" on it; the thumbnail is the card's own title; "the card that shouldn't exist" is literally true of the app's data (✓), so the claim survives scrutiny in the comments.

Why it retains (◐): a new visible detail every 5–10 s (title → file name → app list → eyes → cathedral → clocks → three-card row → deck in hand); the "13 → 16 → NULL" progression is a mini-argument with a punchline; the [AUTH] line is a personal admission ("so the deck could lose an argument") that only the maker could say; the close is an impossible dare that seeds comments and a product loop.

Oracle: swipe-survival at 3 s ≥ 70% (○ heuristic) and ≥ 2 comments per 1,000 views asking what the card means. Falsifier: retention collapses at 0:08 (the file-browser cut), meaning the proof beat is too "inside"; replace it with the app list only.

Production gate: if Null is added to the app before publishing, change the last line to "Draw it and tell me." Do not publish the "You can't. Yet." line if the card is drawable.

---

## 10. Script B — long-form 8–10 min: "I let a chaos oracle read my week" (MAXIMUM CHAOS voice)

Working title: **"I Let a Chaos Oracle Read My Week (It Got Weird)"**. Thumbnail T3. Structure 6.1. Voices: NARRATOR (Apocky; deadpan Signal register; VO or on-screen text) and ORACLE (MAXIMUM CHAOS). Template: **Week Ahead** (3 cards: Beginning of Week / Midweek / End of Week; free ✓). Example draw for this script: **The Zero Point** (Signal) · **Wheel of Misfortune** (Glitch) · **Tower of Obsolescence** (Signal). Production rule (**[AUTH]**, inauthentic-content shield): the draw and the week must be real; if the live draw differs, keep the beat structure and re-generate the ORACLE lines from the app output, then rewrite the numerology to the real numbers. Gate: L1 (Maximum Chaos reachable in production) — Issue I-01.

**0:00–0:30 Cold open (O3)**

*Visual: red-string board, three cards pinned, camera shaking slightly.*

ORACLE: "OKAY. Okay. Zero, ten, sixteen. Zero plus ten plus sixteen is TWENTY-SIX. You know what else is twenty-six? LETTERS. IN THE ALPHABET. And the first card, the Zero Point, is wrapped in — say it with me — DICTIONARY PAGES. I didn't put that there. YOU didn't put that there. WHO PUT THAT THERE?"

*Hard cut to black.*

NARRATOR: "Ten minutes earlier."

*Title card: I LET A CHAOS ORACLE READ MY WEEK.*

**0:30–1:15 Setup**

*Visual: chaos-tarot.com, the reading screen. Cursor picks the Week Ahead template. The three position labels appear: Beginning of Week, Midweek, End of Week.*

NARRATOR: "Chaos Tarot has eight divination systems and an AI oracle. The oracle has modes. This one is called Maximum Chaos. The rules: one spread, the Week Ahead, three cards. I ask one question. I do not argue with it. Then I live the week and report back. This is entertainment. If it tells me to sell a kidney, I'm not selling a kidney."

*Typing on screen: "What does my week look like?"*

NARRATOR: "Question in. Mode: Maximum Chaos. Draw."

*Shuffle animation.*

**1:15–2:45 Card 1 — Beginning of Week: The Zero Point (Signal)**

*Visual: card flips. Zoom on the diving helmet, the bowler hat, the dictionary-page bandages, the foot leaving the stack of dictionaries, the trail of eyes on the floor.*

ORACLE: "Beginning of the week. The Zero Point. Card ZERO. You're starting from nothing, okay, everyone starts from nothing, that's not the interesting part. LOOK at him. He's wearing a diving helmet. On LAND. Why would you wear a diving helmet on land? Because he's expecting to go UNDER. He KNOWS. And what's he standing on? Dictionaries. He is stepping OFF the dictionaries. He is leaving the DEFINITIONS behind. Monday, you are going to do something you don't have a word for yet."

NARRATOR: "That's… actually the Codex reading. 'The moment before the Big Bang.' 'Trust the random number generator.'"

ORACLE: "Don't quote the manual at me, I WROTE the— no. No, I didn't. But I've READ it. Look at the floor. Eyes. A trail of eyes. Where are they going? They're going where HE'S going. Something is watching your Monday. Hold that thought. HOLD IT. We're coming back to the eyes."

*On-screen: a red string from the eyes to an empty pin labelled "?".*

**2:45–4:15 Card 2 — Midweek: Wheel of Misfortune (Glitch)**

*Visual: card flips, lands Glitched: the image tears, pixelates, the CRT-headed king's crown is upside down. Zoom on the two clocks, the two fans, the man crushed beneath the wheel, the browser-error windows in the sky.*

ORACLE: "Midweek. Wheel of Misfortune. And it's GLITCHED. Now. The Codex says a Glitch is the archetype in high entropy. The signal is corrupted. So take everything the Wheel means and turn the noise UP. Look at the wheel. It's not a wheel. It's a GEAR. A gear needs another gear. Where's the other gear? THERE ISN'T ONE. It's turning ALONE. That's your Wednesday. Something is running with nothing driving it."

NARRATOR: "I do have a cron job I don't remember writing."

ORACLE: "THANK you. And the clocks. There are TWO clocks on this card. Two. Not one. TWO. Two clocks means two times. Two times means two things at the SAME time. Wednesday you have two deadlines and they are going to COLLIDE. And the guy on top? Crown. TV for a head. He's not in charge. He's just the one the screen is pointing at. And the guy on the bottom — that's the guy who thought the wheel was a WHEEL."

*On-screen: red string from the two clocks to the pin "?"; second pin labelled "WED", two strings.*

ORACLE: "And the sky. Those are error windows. 'Something went wrong.' Those are your NOTIFICATIONS. The Wheel is glitched because the Wheel is your CALENDAR, and your calendar is lying to you."

**4:15–5:45 Card 3 — End of Week: Tower of Obsolescence (Signal)**

*Visual: card flips. Lightning on the CRT tower. Two figures falling. Sea of dead monitors. Zoom on the monitors' screens: green code, some blank.*

ORACLE: "End of week. The Tower. SIXTEEN. Everybody panics at the Tower. Don't panic. LOOK at it. What is the tower made of? MONITORS. Screens. Dozens of them. Stacked. Held together with cables. That's not a building, that's a STACK. Someone built a tower out of things that show you things. And lightning hits it. And it falls."

NARRATOR: "The guideline literally says Tower equals system crash. 'Not a disaster but an opportunity to rebuild a better stack.'"

ORACLE: "I KNOW what the guideline says, and the guideline is RIGHT, but it's not going far ENOUGH. Remember the eyes? The eyes on the floor on Monday? Eyes. Monitors. Screens that WATCH. The Zero Point walked away from the dictionaries and the eyes FOLLOWED him and by FRIDAY they've stacked themselves into a TOWER and the universe hits it with LIGHTNING. The Tower isn't your week collapsing. It's THE WALL COMING DOWN. Everything that was watching you falls into the water. You're not the guy falling. Look closer. Two guys falling. Two. Two CLOCKS. The two deadlines are the two guys. They FALL. You don't. You're the one standing in the water going 'huh.'"

*On-screen: all strings converge on a new pin: "FRI: THE WALL".*

ORACLE: "And zero plus ten plus sixteen is twenty-six. Twenty-six letters. Dictionary man. The whole week is about a WORD you don't have yet. By Friday you'll have it. It's probably 'oops.'"

**5:45–7:00 Follow-up question (Oracle sub feature) + Memory Core**

*Visual: the follow-up input box. NARRATOR types: "Should I be worried?"*

ORACLE (briefly calm, Signal-adjacent): "The datastream acknowledges your query, Seeker." *(beat)* "WORRIED? You should be taking NOTES. Worry is a Glitch of the Hermit. This isn't a Hermit week. This is a Zero Point week. Wear the helmet. Go under."

*Visual: the journal opens; last week's reading is visible; the oracle references it.*

ORACLE: "And ANOTHER thing. Memory Core says last week you drew the Hermit of the Void in the Midweek slot. Hermit. Nine. This week, Midweek, ten. The Wheel. NINE to TEN. You went OFFLINE and then the WHEEL started turning with nobody at it. THAT'S the cron job. The Hermit wrote it. You were the Hermit. YOU WROTE IT."

NARRATOR: "That is… not impossible."

ORACLE: "Nothing is impossible. That's the whole system. 'Nothing is true; everything is permitted.' It's on the WALL."

*(Note: the prior-week reading must be real. If there is none, cut the Memory Core beat to a one-line demo: "Memory Core will remember this one. Next week we find out if it holds a grudge.")*

**7:00–8:30 [AUTH] Reality check — the week, day by day**

*Visual: calendar-style cards, NARRATOR's own voice, no oracle. This beat must be Apocky's real week; the lines below are placeholders that show the shape.*

NARRATOR: "Monday. I started a thing without reading the docs. Didn't have a word for it. Still don't. Zero Point: one point to the oracle.

Wednesday. Two things landed at the same hour. One of them was a cron job I genuinely did not remember writing. It was running alone. Wheel, glitched, two clocks: I'm giving it that one, and I'm not happy about it.

Friday. Nothing collapsed. My old laptop did finally die, which is a tower made of a monitor, if you squint. The two deadlines both slipped to next week, so, technically, they fell and I didn't. I'm calling that a half.

Twenty-six letters: the word was 'refactor.' That's eight letters. The oracle is wrong about the alphabet and I have no idea why it's right about everything else."

**8:30–9:30 Verdict + close**

*Visual: the three cards, the string board, then chaos-tarot.com.*

NARRATOR: "Two and a half out of three. That's a coincidence, a pattern, or a conspiracy, and the deck's whole point is that those are the same thing from the inside. This is entertainment and personal reflection. It is not advice. It is definitely not calendar software."

ORACLE: "The protocol completes. Remember: the future is unwritten code — you hold the commit privileges."

NARRATOR: "The Week Ahead spread is free at chaos-tarot.com. Follow-up questions and Memory Core are the Oracle subscription, three dollars thirty-three a month. Next video: why this deck has no reversed cards."

ORACLE: "…unless that's what they want you to think."

*End screen: Script C card + subscribe.*

Spoken words: ~1,350 (≈ 9 min at 150 w/min with pauses). Chapters: 0:00 The Wall · 0:30 The Rules · 1:15 The Zero Point · 2:45 Wheel of Misfortune (Glitch) · 4:15 Tower of Obsolescence · 5:45 Follow-up · 7:00 What actually happened · 8:30 Verdict.

Why it attracts (◐): the cold open is the vibe doc's proven shape (numerology → "WHO PUT THAT THERE?") on a real, countable art detail; the title has first-person stakes; the thumbnail is the string board, which reads as comedy before the viewer knows it is tarot; "Maximum Chaos" is a product term and a promise of tone in one.

Why it retains (◐): three open loops (eyes, two clocks, alphabet) each paid at a later card; the deadpan NARRATOR resets attention every 40–60 s (R3); the sub feature is sold only after it does something on screen (follow-up, Memory Core); the reality check is the reward and is announced in the setup so viewers know to wait; the guideline's closing line and M12 out-hook hand off to Script C.

Oracle: AVD ≥ 40% of length (⊘ target) with no cliff at 5:45 (the sub beat); at least one Short cut from the ORACLE beats out-performs the long-form in views. Falsifier: retention drops at every ORACLE run and recovers on NARRATOR, which means the voice is grating at this density; halve ORACLE runs to ≤ 20 s and let the NARRATOR carry the argument.

Production gates: L1 confirmed (I-01); the prior-week reading exists or the Memory Core beat is cut; disclosure toggle on; disclaimer in description; no health/money/legal claims (the "kidney" line is a joke about not taking advice; keep it or cut it, never replace it with a real claim).

---

## 11. Script C — long-form 6–8 min: "Signal vs Glitch: why this deck has no reversed cards"

Working title: **"Reversed Tarot Cards Don't Exist in This Deck. Glitches Do."** Thumbnail T2. Structure 6.2. Voice: NARRATOR (Signal) throughout; ORACLE appears twice, briefly, in the app demo. Honest-caveat line at 4:50 is mandatory until Issue I-04 ships.

**0:00–0:25 Claim (O4)**

*Visual: The Zero Point, clean. A hand (or cursor) turns it upside down.*

NARRATOR: "In most tarot, this is a reversed card. Upside down, and it means blocked, delayed, turned inward. This deck doesn't have that."

*Visual: the card snaps back upright and tears into pixels down the right half.*

ORACLE: "It has Glitches."

NARRATOR: "Those are not the same thing, and the difference changes every reading you'll do with it."

*Title card: SIGNAL vs GLITCH.*

**0:25–1:00 The old way**

*Visual: a plain card silhouette rotating 180°.*

NARRATOR: "Traditional reversals are a position. The card lands upside down, so you read the shadow side: the energy is blocked, internal, or late. It's a switch. On or off. Upright or reversed. That's a fair system, and it's the one the runes borrow when they call it merkstave. Chaos Tarot supports reversals and merkstave in the app. But the Codex, the manual for this deck, says something else about what a reversal is."

**1:00–2:15 Signal**

*Visual: Codex text typed over static (O6): "the binary concepts of 'Upright' and 'Reversed' are discarded in favor of a dynamic signal-processing metaphor: the Signal and the Glitch."*

NARRATOR: "The Signal is the card working as designed. The Codex: 'the distinct image on the screen, the clear audio track, the executed code.' The archetype is running inside expected parameters."

*Visual: The Zero Point, full, slow push on the diving helmet and the dictionaries.*

NARRATOR: "The Zero Point in Signal: the Codex calls it 'the moment before the Big Bang, containing infinite potential energy.' 'Trust the random number generator; the universe rewards the brave variable.' He's stepping off the dictionaries on purpose. That's the clear signal."

*Visual: Tower of Obsolescence, lightning frame.*

NARRATOR: "The Tower in Signal is still a crash. The training guideline for the oracle puts it plainly: 'System Crash: not a disaster but an opportunity to rebuild a better stack.' The signal of the Tower is a clean failure. You can read the logs."

**2:15–3:45 Glitch**

*Visual: The Zero Point again; the image begins to smear, then stutter, datamosh across the bandages.*

NARRATOR: "Now the Glitch. The Codex: 'A Glitched card is not merely "blocked" or "internalized," as in traditional interpretations. It represents the archetype in a state of high entropy. The signal has been corrupted, distorted, or amplified to the point of destructive noise.'"

*Visual: text on screen: "corrupted · distorted · amplified".*

NARRATOR: "That's the whole difference in three words. A reversal turns the card down. A Glitch turns it up until it breaks. The Codex tells the reader to ask 'how the archetype is failing or mutating,' and gives two examples: 'Is the Emperor's structure becoming tyranny (rigid static)? Is the Fool's freedom becoming nihilism (void noise)?'"

*Visual: 4 Lord of Entropy, clean then rigid-static overlay; then The Zero Point with a void-noise overlay.*

NARRATOR: "So the Zero Point Glitched isn't 'afraid to start.' The Codex reads it as 'Recklessness masquerading as bravery… a reboot without saving progress.' Same man, same helmet, same step off the dictionaries. The difference is that in the Glitch he never checked whether there was water."

**3:45–4:30 Why it changes a reading**

*Visual: side-by-side interpretation cards: "Reversed: blocked / delayed" vs "Glitch: amplified until it fails".*

NARRATOR: "In a reversal system, a bad card upside down is often good news: the danger is blocked. In a Glitch system, it's the opposite. A Glitched Tower isn't a crash that didn't happen. It's a crash that happens sideways, in a place you weren't logging. The oracle's training guideline has a rule for this. It calls it the Glitch Filter: 'Describe the card as corrupted, pixelated, or inverted. Frame the reversal as a system error that reveals a deeper truth.' The Glitch is information. Not a wall."

**4:30–5:30 [AUTH] In the app**

*Visual: chaos-tarot.com. A real 3-card draw; one card lands reversed.*

NARRATOR: "Here's what that looks like live. Three cards. One of them lands the other way."

*Visual: tight on the label.*

NARRATOR: "You'll notice the app calls it 'Reversed' right now. Same mechanic, older label. The Codex name is Glitch, and that's the fix on my list. What matters is what the oracle does with it."

*Visual: the interpretation output; the Glitched card's paragraph highlighted.*

ORACLE: "The signal fragments, but the truth persists. This card runs hot. Read it not as denied, but as overdriven: the thing you wanted, delivered in a form you cannot yet parse."

NARRATOR: "That's the filter working. It didn't tell me no. It told me I'd asked for something in the wrong units. That's the part I actually built this for: the Glitch is the reading. Not the footnote."

**5:30–6:30 Philosophy**

*Visual: static; Codex and guideline lines typed in turn.*

NARRATOR: "The guideline calls the glitch a 'holy interruption.' 'The glitch is not an end state but a becoming, a transition from one state of being to another.' The Codex says the deck 'embraces the fracture,' and that 'wisdom is often found not in the pattern, but in the break of the pattern.' A reversed card says the pattern stopped. A Glitched card says the pattern broke, and the break is where you look. That's why there are no reversed cards in this deck. There's nothing to turn down. There's only what happens when you turn it up."

**6:30–7:00 CTA + close**

*Visual: chaos-tarot.com, then the physical deck.*

NARRATOR: "Draw three cards free at chaos-tarot.com and see which one Glitches. The AI oracle's full interpretation with follow-up questions is the Oracle subscription, three thirty-three a month. The printed deck, seventy-nine cards, is linked below. Entertainment and personal reflection only. Next: the card that shouldn't exist."

ORACLE: "The protocol completes."

*End screen: Script A + subscribe.*

Spoken words: ~1,000 (≈ 6.5–7 min). Chapters: 0:00 The claim · 0:25 Reversals and merkstave · 1:00 Signal · 2:15 Glitch · 3:45 Why it changes a reading · 4:30 In the app · 5:30 The break in the pattern · 6:30 Links.

Why it attracts (◐): the title collides with a high-volume search phrase ("reversed tarot cards") and contradicts it; T2's split thumbnail shows the mechanic at feed size; the subject is a genuine differentiator quoted from the deck's own manual rather than a generic tarot explainer, which is the "original insight" the inauthentic-content policy asks for (○ §0.2).

Why it retains (◐): a single argument built in four steps (old way → Signal → Glitch → consequence) with a worked example carried through all four (the same Zero Point card, clean then corrupted); the live draw at 4:30 is a change of medium at the midpoint; the honest caveat pre-empts the comment that would otherwise dominate; the philosophy beat is the payoff, not the setup; the out-hook hands off to Script A.

Oracle: search traffic ≥ 25% of the video's views within 60 days (⊘ target) and comment threads about "reversed vs Glitch" outnumber "the app says Reversed" threads. Falsifier: the caveat line at 4:50 is the highest drop-off point, meaning the honesty is landing as a bug report; ship I-04 and re-cut.

Production gates: I-04 status decides whether the caveat stays; no suit-element claims (C3); disclosure toggle on; disclaimer in description.

---

## Open questions for Apocky

1. **C2:** the 79th card's face reads "XXII THE APOCALYPSE"; the file is "Null The Apockalypse". Which is canonical on screen, and is the pun meant to be visible to viewers?
2. **C1/L1:** is the Maximum Chaos toggle reachable on chaos-tarot.com in the React build today, or only in the legacy app/API? Script B is blocked on this.
3. Should Null/XXII be added to the app's deck data (so viewers can draw it), or stay physical-only? Script A's last line depends on it.
4. **C3:** which suit→element mapping is canonical (SQL seed, `shared/deck.js`, art, or Codex)? No suit-lore videos until answered.
5. **C4:** approve relabelling "Reversed" → "Glitch" in the React UI before Script C publishes, or keep the caveat line?
6. **L3:** do UTM parameters survive the Try Free / access-code flow? If not, what should CTAs link to for attribution?
7. What is `Page of Fractures.jpg`? An 80th card, a replaced Page, or a leftover? It affects the "79" claim.
8. Is the Week Ahead reading in Script B to be recorded with a real prior-week reading (Memory Core beat), or cut?
9. Own voice or synthetic for NARRATOR? Under the fetched policy, cloning your own voice needs no disclosure; a generic synthetic voice for the ORACLE persona is non-photorealistic. Confirm the disclosure toggle stays on regardless.

## Issue candidates

| Title | Phase | Priority | Acceptance oracle | Falsifier | Depends on |
|-------|-------|----------|-------------------|-----------|------------|
| I-01 Confirm Maximum Chaos mode is live on chaos-tarot.com (React path) | 0 | P0 | A logged-in session shows the toggle and returns a Pepe-Silvia-style reading | Toggle absent in production; only `api/ai-reading.js` has it | — |
| I-02 Ratify the 79th card's on-screen title and Codex entry (XXII The Apocalypse / Null) | 0 | P0 | One-line decision in BRIEF contradictions; Codex gets a card entry | Two videos ship with different names for the same card | — |
| I-03 Per-video anti-"inauthentic content" checklist ([AUTH] beat, real draw, real week, disclosure toggle, disclaimer block) | 0 | P0 | Checklist file in plan dir; first 5 uploads pass it | A video ships without an [AUTH] beat | §0.2 |
| I-04 Relabel "Reversed" → "Glitch" in `ReadingResults.tsx` and `CardDetailModal.tsx` (product request) | 1 | P1 | UI shows "Glitch"; Script C caveat removed | Label unchanged at Script C publish; caveat stays | Apocky Q5 |
| I-05 Ratify canonical suit→element mapping across SQL seed, `shared/deck.js`, art, Codex | 1 | P1 | One mapping; the other three sources updated or marked lore | A suit-lore video asserts an element that the app contradicts | Apocky Q4 |
| I-06 Hook / title / thumbnail bank v1 as a CSV in `plans/chaos-tarot-youtube/` | 0 | P0 | ≥ 24 hooks, 8 title formulas, 8 thumbnail concepts, each with family tag | Videos are titled ad hoc without the bank | this file |
| I-07 Three thumbnail templates (T1 number, T2 split, T3 string board) built from the art at 1280×720 | 1 | P1 | Three PNGs; each used on ≥ 1 video | A thumbnail is made from scratch per video | A4 |
| I-08 Produce Script A Short | 1 | P0 | Published; 3-s swipe survival ≥ 70% (○ heuristic) | Retention cliff at 0:08 | I-02, I-03 |
| I-09 Produce Script C long-form | 1 | P1 | Published with chapters; search traffic ≥ 25% at 60 d | Caveat line is the top drop-off | I-03, I-04 (or caveat) |
| I-10 Produce Script B long-form | 1–2 | P1 | Published; AVD ≥ 40%; ≥ 1 Short cut from it | Retention drops on every ORACLE run | I-01, I-03 |
| I-11 UTM-tagged landing links that survive the access-code flow | 1 | P1 | Site analytics show `utm_source=youtube` sessions | Zero attributable sessions after 10 videos | Apocky Q6 |
| I-12 Add Null/XXII to app deck data or document it as physical-only | 1 | P2 | Card drawable in app, or a line in the Codex saying it is not | Script A's dare becomes false without a re-cut | Apocky Q3 |
| I-13 Retention review of the first 5 Shorts and 2 long-forms against §7 techniques | 2 | P2 | Written review; bottom-quartile hooks retired | No review by day 45 | I-08, I-09 |
| I-14 Resolve `Page of Fractures.jpg` and the "79" count | 0 | P3 | One-line answer in BRIEF | A viewer counts 80 card faces | Apocky Q7 |

## CSL annex

Σ Per-video creative system for Chaos Tarot: 8 format families, 24 hooks (12 Signal, 12 MAXIMUM CHAOS), 8 title formulas, 8 art-only thumbnails, 6 openings, 4 structures, 10 retention devices, 3-target CTA stack, and three full scripts (Short: XXII/Null; long: chaos oracle reads a week; long: Signal vs Glitch), each with attract/retain analysis, oracle and falsifier.
W! Ratify the 79th card's on-screen name (art says XXII The Apocalypse) and confirm Maximum Chaos is live in the React build before recording Scripts A and B; put an [AUTH] beat (real draw, real week, Apocky's own line) in every video; toggle AI disclosure on and paste the site disclaimer into every description; qualify before 1 Feb 2027 when the YPP bar reportedly doubles (○).
N! Never caption a card with a name that is not on the art; never assert a suit's element until C3 is resolved; never sell the Oracle sub before the feature is shown on screen; never drift into medical/financial/legal claims; never publish template-only videos (image slideshow + generic AI VO) that match the inauthentic-content examples.
