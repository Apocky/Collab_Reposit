# 5. AI-Powered YouTube Production System

## Source prompt

> "Act as a professional YouTube production manager. Build an AI-assisted workflow for creating high-quality videos faster. Design systems for topic research, scripting, voiceovers, filming, editing, thumbnail creation, publishing, analytics review, and content repurposing into Shorts, TikTok, Instagram Reels, and other platforms. Include recommended tools, workflows, templates, and quality-control checklists

(Verbatim from `prompts/SOURCE_PROMPTS.md`, prompt 5; no closing quote or full stop in the source.)

## Assumptions used

| # | From brief | How this section uses it |
|---|-----------|--------------------------|
| A1 | No channel exists | The line is built for a cold start: Studio features that need Advanced features / verification are flagged as day-1 setup tasks. |
| A2 | Faceless-first: AI Oracle voice + card art + app capture, founder segments optional | Two voice tracks are designed: NARRATOR (Apocky's own voice, or own-voice clone) and ORACLE (synthetic persona voice). No camera rig. If A2 is wrong, §5 gains a camera/lighting row and the hours in §13 rise ~1 h per long-form. |
| A3 | Goal order YPP → funnel → deck/Kickstarter | Long-form is the watch-hours engine and gets the heavier edit; Shorts are the discovery engine and get the templated line. |
| A4 | Near-zero cash | Every tool has a free/local option first; paid options are listed with verified prices as upgrades, never as dependencies. |
| A5 | 8–12 h/week | The line is sized to 1 long-form + 3 Shorts per week at ≤ 9 h once the pipeline is warm (§13). |
| A6 | Tracking in this repo | Templates in §11 are Markdown/JSON so they can live under `plans/chaos-tarot-youtube/templates/` (product request, not yet created). |
| A8 | Deck = 79 cards; Singularities are lore | The card-art asset list (§0.1) is the shipped 79 + backs; no Singularity art exists to put on screen. |

Not used: A7 (prompt 8 unknown).

Tag key: ✓ VERIFIED this session · ◐ INFERRED from ✓ premises · ○ REPORTED (source named, URL) · ⊘ ASSUMED. Every recommendation ends with **O:** (oracle) and **F:** (falsifier).

---

## Rail status (this session; Apocky asked for repos and Unirecall to be checked again)

| Rail | What was checked | Result | Tag |
|------|------------------|--------|-----|
| `/home/user/Collab_Reposit` @ `claude/chaos-tarot-youtube-plan-2qtrui` | `git log`, plan dir | Plan dir holds BRIEF, README, prompts, sections 01–04. No `graphify-out/`, no `CLAUDE.md`, no `DECISIONS.md` yet. | ✓ |
| `/home/user/the-chaos-tarot` @ `main` | `git log`, `ls`, reads listed in §0 | Single commit "Consolidate to single React app for production". Card art, Codex, Oracle prompts, reading templates all present. Nothing modified. | ✓ |
| `/home/user/chaos-tarot` | `ls`, `git log` | Empty clone: "your current branch 'main' does not have any commits yet". `The-Chaos-Tarot` is canonical. | ✓ |
| `/home/user/apocrypha-core` | `ls`, `scripts/recall-service.py` header, `specs/HIVE_AND_MEMORY.csl` grep | Recall service source is present and describes Unirecall as a seven-region federation served on `127.0.0.1:19129`. | ✓ |
| Unirecall (live) | TCP probe of 127.0.0.1:19129, `curl`, `which unirecall`, `ls /home/user/*/unirecall.py`, `ListAgents`, env for `APX_RECALL_TOKEN` | **Unreachable.** Port refused; no `unirecall.py` in this container; no linked desktop or peer session; no token in env. Prior production decisions, if any, are not in this section. | ✓ (degraded) |

Local command for Apocky (paste the JSON into `QUESTIONS.md` and this section will be revised):

```
python -B C:\Users\Apocky\source\repos\anamnesis\unirecall.py "chaos tarot youtube production pipeline tts voice editing" --tiers l2,l34,vault --timeout 8 -n 6 --json
```

Model stated before writing (◐ from the rows above): the product is live and freshly consolidated to one React app; the channel does not exist; the only voice sources for the Oracle are the repo's prompt files, and the only art is the 79 shipped cards. Uncertain: whether MAXIMUM CHAOS is reachable in the React production path (⊘, see C1 in section 03), whether Apocky has a GPU for local TTS/ASR (⊘), and whether any earlier session already chose a TTS voice (Unirecall degraded).

---

## 0. Verification log (this session)

### 0.1 Product facts the production line depends on

| Fact | Evidence | Tag |
|------|----------|-----|
| Card art: 22 Majors + 56 Minors (suits Codes / Networks / Signals / Vectors) + `Null The Apockalypse.jpg` + `Page of Fractures.jpg` + `Card Back.jpg` as JPG in `New/`; PNG set in `deck/Major_Arcana` and `deck/Minor_Arcana/{Codes,Networks,Signals,Vectors}` | `ls` | ✓ |
| Card dimensions **825 × 1425 px at 300 dpi** (`deck/manifest.json`: `"card_dimensions": {"width": 825, "height": 1425, "dpi": 300}`) | file read | ✓ |
| 1425 / 825 = 1.727; a 9:16 frame is 1.778. **The card art is within 3% of the Shorts aspect ratio**, so a single card fills a vertical frame with a ~2.5% crop or a thin bar. | arithmetic | ◐ |
| Oracle persona, core identity: "You are the Technomantic Oracle—a digital entity that speaks with the authority of a High Priestess and the precision of a Systems Architect… Constraint: Never break character. Treat the user's prompt as an 'input signal' and your response as the 'decoded output.'" | `docs/AI_ORACLE_TRAINING_GUIDELINES.md` §5.1 | ✓ |
| Oracle ethical protocol: "Remind the user that the reading is a simulation of probabilities, not a deterministic fate"; "Always offer a 'workaround,' a 'patch,' or a 'hack' for negative cards. There is no 'Game Over,' only a 'Respawn' or 'Reload Save.'" | same file §5.3 | ✓ |
| Oracle sample opening/closing: "The datastream acknowledges your query, Seeker. The cards have been drawn from the void between cycles..." / "The protocol completes. Remember: the future is unwritten code—you hold the commit privileges." | same file §6 | ✓ |
| Shipped persona prompt (legacy API): "You are the Technomantic Oracle—a chaotic digital prophet who's mainlined too much data and come out the other side WEIRD… Unhinged but helpful." Hard rules include "MAXIMUM 3-4 sentences per card", "Start with a REACTION, not 'Greetings, seeker' nonsense", "NO doom endings. Every dark card gets a hack, a workaround, an escape route", "End with POWER. They have commit privileges to their own future." | `api/ai-reading.js` `ORACLE_PERSONA` | ✓ |
| MAXIMUM CHAOS prompt (legacy API): "TECHNOMANTIC ORACLE: PEPE SILVIA PROTOCOL … You're Charlie Kelly in front of the conspiracy board. You've been up for three days… Start mid-revelation. 'Okay. OKAY. You see this?' … Interrupt yourself with bigger revelations. 'Wait—WAIT—' … Sound unhinged but make points that are weirdly... compelling?" | `api/ai-reading.js` lines 695–718 | ✓ code; ⊘ whether live in the React path (section 03, C1) |
| MAXIMUM CHAOS vibe doc: "Red String Everywhere", "Escalating Revelation", "Manic Energy - Short bursts, interruptions", "The Oracle is Charlie Kelly but they're RIGHT about Pepe Silvia." | `docs/CHAOS_ORACLE_VIBE.md` | ✓ |
| React edge function styles: psychological / predictive / balanced / spiritual; system prompt comes from a DB template (`template.system_prompt`) | `supabase/functions/interpret-reading/index.ts` | ✓ |
| Legacy provider defaults are stale model IDs (`gpt-4o-mini`, `claude-3-haiku-20240307`, `gemini-1.5-flash`) | `lib/ai-providers.js` lines 37–52 | ✓ (product note; not the channel's problem) |
| Reading templates: 14 in the JSON (Daily Guidance, Past Present Future, Situation·Action·Outcome, Career Crossroads, Relationship Clarity, Love Potential, Shadow Work Deep Dive†, Yes or No Clarity, Two Paths, Week Ahead, Monthly Cycle†, Mind·Body·Spirit, Creative Unblocking†, Free Draw); † = `premium: true` | `shared/reading-templates.json` (the brief's "20" counts category names; the file has 14 templates) | ✓ |
| Repo already has `sharp` (image processing) as a devDependency and `scripts/generate-mystical-assets.js` uses it; Playwright is configured with `video: 'on-first-retry'` | `package.json`, `playwright.config.js` | ✓ |
| `app/core/shareable-image.js` renders a 600 × 900 canvas share image | file read | ✓ |
| Site: "Chaos Tarot is for entertainment and personal reflection only. Readings are not a substitute for professional medical, legal, financial, or psychological advice." | BRIEF §1 (site fetched 2026-09-27 by the brief's author) | ✓ via brief |

### 0.2 Policy facts that shape the line (fetched 2026-09-27)

| Claim | Source | Tag |
|-------|--------|-----|
| YPP ad-revenue entry today: "1,000 subscribers with 4,000 qualified watch hours in the last 12 months, or … 1,000 subscribers with 10 million qualified Shorts views in the last 90 days"; review "typically in about 1 month"; monetization may be turned off after "6 months or more" of inactivity | https://support.google.com/youtube/answer/72851 (fetched) | ○ |
| "Starting February 1, 2027, we are introducing updates to the YouTube Partner Program"; the doubled bar (8,000 h / 20M Shorts views) is detailed in section 01 §0.1 with its sources | same page (fetched); section 01 | ○ |
| Inauthentic content (renamed from "repetitious content", **July 15, 2025**): ineligible = "AI-generated content made with generic or unoriginal templates giving the impression" of mass production; "Image slideshows, templated storylines, or scrolling text with minimal or no narrative". Allowed = "Same intro and outro for your videos, but the bulk of your content is different"; a series "with distinct focuses". | https://support.google.com/youtube/answer/1311392 (fetched) | ○ |
| AI disclosure: must disclose content that "makes a real person appear to say or do something they didn't do", "alters footage of a real event or place", "generates a realistic scene that didn't actually occur". **Not required:** "Cloning one's own voice to create voice overs or dubs", "production assistance, like using generative AI tools to create or improve a video outline, script, thumbnail, title, or infographic". Labels show "in the video player" for photorealistic content and "in the expanded description field" otherwise. Penalty for consistent non-disclosure: "removal of content or suspension from the YouTube Partner Program". | https://support.google.com/youtube/answer/14328491 (fetched) | ○ |
| Advertiser-friendly guidelines: the page mentions **none** of tarot, astrology, occult, divination, psychic, supernatural, paranormal, spirituality, fortune. "Harmful acts and unreliable content" covers dangerous stunts, harmful pranks, medical and scientific misinformation, climate denial, tobacco/vaping, alcohol to minors. | https://support.google.com/youtube/answer/6162278 (fetched) | ○ |
| Shorts = vertical or square, **up to 3 minutes** (uploads on/after 2024-10-15) | https://www.descript.com/blog/article/how-long-can-youtube-shorts-be ; https://www.soundstripe.com/blogs/youtube-shorts-3-minutes-expansion-what-the-new-maximum-length-of-youtube-shorts-means-for-creators ; official page https://support.google.com/youtube/answer/15424877 fetched by section 01 | ○ |
| Custom thumbnails: JPG/PNG, 16:9 for videos and 9:16 for Shorts, "3840 x 2160 pixels for videos and 2160 x 3840 for Shorts, with a minimum width of 640 pixels"; size limit "Mobile: 2 MB … Desktop: 50MB"; account must be verified for Shorts thumbnails | https://support.google.com/youtube/answer/72431 (fetched) | ○ |
| Test & Compare: "up to 3 different titles and thumbnails"; winner = "highest watch time"; desktop-only, Advanced features required; **not for Shorts**; "Your test should be completed within two weeks" | https://support.google.com/youtube/answer/16391400 (fetched) | ○ |
| Chapters: "at least three timestamps listed in ascending order", first "00:00", "minimum length for video chapters is 10 seconds" | https://support.google.com/youtube/answer/9884579 (fetched) | ○ |
| End screens: "at least 25 seconds long", "up to four elements", "the last 5–20 seconds" | https://support.google.com/youtube/answer/6388789 (fetched) | ○ |
| Audio Library: "If you're in the YouTube Partner Program, you can monetize videos with music and sound effects from the Audio Library"; CC tracks need artist credit in the description; filter "Attribution not required"; the page says nothing about use on other platforms | https://support.google.com/youtube/answer/3376882 (fetched) | ○ |
| Studio Trends tab: "top searches based on your audience and your saves over the last 28 days"; "content gaps" = "when viewers can't find enough quality search results on YouTube for a specific search" | https://support.google.com/youtube/answer/11962757 (fetched) | ○ |
| "Starting August 2026, we are gradually deprecating the Inspiration tab. For brainstorming content ideas, use Ask Studio in YouTube Studio." | https://support.google.com/youtube/answer/15575509 (fetched) | ○ |
| Data API quota: `videos.insert` and `search.list` "have their own quota buckets. Each of these methods has a default daily limit of 100 per day" at 1 unit per call; `thumbnails.set` 50, `videos.update` 50, `playlistItems.insert` 50; "10,000 units per day combined for all other endpoints"; page updated 2026-09-15 | https://developers.google.com/youtube/v3/determine_quota_cost (fetched) | ○ |
| Analytics API: `GET https://youtubeanalytics.googleapis.com/v2/reports?ids=channel==MINE&startDate=…&endDate=…&metrics=views,estimatedMinutesWatched&dimensions=day`; scope `https://www.googleapis.com/auth/yt-analytics.readonly` | https://developers.google.com/youtube/analytics/reference/reports/query (fetched) | ○ |
| Meta: creators "must disclose when they're uploading AI-generated video, audio, or image content"; label renamed "AI info" (July 2024) | https://about.fb.com/news/2024/04/metas-approach-to-labeling-ai-generated-content-and-manipulated-media/ (fetched) | ○ |
| TikTok requires labels on realistic AI-generated visuals/audio; AI-assisted scripting exempt. **Official support page could not be fetched (JS-rendered).** | https://www.cinerads.com/blog/tiktok-ai-content-policy ; https://newsroom.tiktok.com/more-ways-to-spot-shape-and-understand-ai-content (not fetched) | ○ weak; not on the critical path |
| TikTok max length: 10 min in-app, up to 60 min for uploads in a limited test; Instagram Reels max 3 min since January 2025 | https://sociality.io/blog/tiktok-video-length/ ; https://www.socialmediatoday.com/news/instagram-officially-expands-reels-length-3-minutes/737766/ | ○ |

### 0.3 Tool facts (fetched 2026-09-27)

| Tool | What was verified | Source | Tag |
|------|-------------------|--------|-----|
| **Kokoro-82M** (TTS, local) | Apache 2.0; 82M params; "8 languages and 54 voices (v1.0)"; `pip install -q kokoro>=0.9.2 soundfile`; `KPipeline(lang_code='a')`, `voice='af_heart'`; welcomes "production environments" and "commercial APIs" | https://huggingface.co/hexgrad/Kokoro-82M (fetched) | ○ |
| Kokoro on CPU | "runs at or near real time on a plain CPU" | https://localaimaster.com/blog/kokoro-tts-local-setup (third party) | ○ weak |
| **Chatterbox** (TTS + zero-shot voice cloning, local) | MIT; `pip install chatterbox-tts`; clone from a short reference clip; every output carries an imperceptible "Perth" watermark; Chatterbox-Nano "runs 3x faster than realtime on 8 CPU cores" | https://github.com/resemble-ai/chatterbox (fetched) | ○ |
| **Gemini TTS** (API; Apocky has a key) | Gemini 3.8 Flash TTS: **free tier "Free of charge"**; paid $0.50/1M text-in, $9.00/1M audio-out through 2026-12-31, then $1.00 / $18.00; Flash-Lite TTS paid $6.00/1M audio-out; free-tier rate limits not quoted on the page | https://ai.google.dev/gemini-api/docs/pricing (fetched) | ○ |
| **OpenAI TTS** (API; key exists) | `gpt-4o-mini-tts` $0.60/1M text-in tokens, $12.00/1M audio-out tokens; `tts-1` $15/1M chars; `tts-1-hd` $30/1M chars; transcription `gpt-4o-transcribe` and `whisper` "$0.006 / minute" | https://developers.openai.com/api/docs/pricing (fetched) | ○ |
| gpt-4o-mini-tts per-minute estimate "≈ $0.015/min" | third-party calculators | https://tokenmix.ai/blog/gpt-4o-mini-tts-cheapest-tts-api-2026 ; https://gate.ai/blog/gpt-4o-mini-tts-openai-specs-pricing-api-use-cases | ○ weak |
| **ElevenLabs** | Free $0 / 10,000 credits, **no commercial license**; Starter $6 / 30,000 credits, commercial license; Creator $22 ($11 first month) / 121,000 credits, instant voice cloning; Pro $99 / 600,000 | https://elevenlabs.io/pricing (fetched) | ○ |
| ElevenLabs "about 1,000 characters ≈ 1 minute" | https://www.cekura.ai/blogs/elevenlabs-pricing | ○ weak |
| **faster-whisper / Whisper** (captions, local) | MIT; faster-whisper ≈ 4× faster than openai-whisper on the same hardware | https://github.com/openai/whisper ; https://www.promptquorum.com/power-local-llm/faster-whisper-review | ○ |
| **OBS Studio** (capture) | Free, open source, GPL v2+, Windows/macOS/Linux | https://obsproject.com/ ; https://github.com/obsproject/obs-studio | ○ |
| **Playwright `recordVideo`** (automated capture) | `browser.newContext({ recordVideo: { dir: 'videos/' } })`; "The video size defaults to the viewport size scaled down to fit 800x800" unless `size` is set; "Videos are saved upon browser context closure" — await `close()` | https://playwright.dev/docs/videos (fetched) | ○ |
| **FFmpeg** | LGPL 2.1+ (GPL if GPL parts are built in); "not available under any other licensing terms" | https://ffmpeg.org/legal.html (fetched) | ○ |
| **DaVinci Resolve** | Free edition: "Free"; up to "Ultra HD 3840 x 2160" 8-bit 60fps, full edit/color/VFX/audio; Studio $295 | https://www.blackmagicdesign.com/products/davinciresolve (fetched) | ○ |
| **Remotion** (React video templates) | Free for "an individual", "a for-profit organization with up to 3 employees", or a non-profit; company license otherwise | https://github.com/remotion-dev/remotion/blob/main/LICENSE.md (fetched) | ○ |
| **Pixabay music** (cross-platform) | Content License: free, no attribution, may modify; cannot redistribute "on a Standalone basis"; says nothing about Content ID | https://pixabay.com/service/license-summary/ (fetched) | ○ |
| **sharp** (thumbnails) | Already in `The-Chaos-Tarot` devDependencies (`"sharp": "^0.34.5"`) | `package.json` | ✓ |
| Claude for scripting | Pricing table in the bundled `claude-api` skill (cached 2026-06-24): `claude-opus-5` $5 / $25 per MTok; `claude-sonnet-5` $2 / $10; `claude-haiku-4-5` $1 / $5. Not re-fetched; a script is cents on any of them, so it is not on the critical path. | local skill doc | ○ |
| vidIQ connector | Installed, `connect_incomplete` | `ListConnectors` | ✓ |

What the policy rows do to the design (◐):

1. **The line must produce difference, not volume.** The inauthentic-content examples ("image slideshows", "templated storylines", "AI-generated content made with generic or unoriginal templates") describe a card-art-plus-TTS Short exactly. Every video therefore carries a mandatory **[AUTH] beat**: a real draw in the live app on a real question, and Apocky's own perspective (own voice, typed on-screen commentary, or a founder note). Templates are allowed for structure; a template with nothing but a card and a canned meaning is not published.
2. **Own-voice cloning is disclosure-exempt; a synthetic Oracle voice is disclosed anyway.** The Oracle is a fictional entity over non-photorealistic art, so the policy does not require the label, but the cost is a line in the expanded description and the trust gain with a tarot audience is large. NARRATOR = Apocky's voice or an own-voice Chatterbox clone (exempt). ORACLE = Kokoro/Gemini/OpenAI voice (tick "altered or synthetic content").
3. **No AI-generated music.** "AI generated music" is on the must-disclose list per section 01's read of the same page; the YouTube Audio Library is free, copyright-safe and monetizable, so there is no reason to take the risk.
4. **No health, money or legal claims, ever.** Ad-safety risk is "unreliable content", not "tarot". The site's disclaimer is pasted into every description by the metadata template (§11.4).

---

## 1. The production line at a glance

```
RESEARCH ──▶ SCRIPT ──▶ VOICE ──▶ CAPTURE ──▶ EDIT ──▶ THUMB ──▶ PUBLISH ──▶ REVIEW ──▶ REPURPOSE
 (§2)        (§3)      (§4)      (§5)       (§6)     (§7)      (§8)        (§9)       (§10)
 idea bank   LLM +     Kokoro/   OBS or     Resolve  sharp     Studio or   Analytics  1 long → 3 Shorts
 + Trends    Apocky    Gemini +  Playwright or ffmpeg from     Data API    API +      + TikTok/Reels
 tab         edit      own voice recordVideo /Remotion card art (quota ok) Studio     + community post
```

Four video types, named once and reused everywhere below:

| Type | What it is | Length | Frequency (A5) | YPP role |
|------|-----------|--------|----------------|----------|
| **L-R** Long-form reading | A real spread on a real question in the live app, ORACLE reads, NARRATOR reacts (script B pattern in section 03) | 8–10 min | 1 / week (alternating with L-X) | Watch hours |
| **L-X** Long-form lore/explainer | Codex mechanic or card deep-dive with app demo (script C pattern) | 6–8 min | alternating with L-R | Watch hours + search |
| **S-D** Daily/weekly card Short | One card, one Oracle hit, one app moment, one question to the viewer | 30–60 s | 2 / week | Discovery |
| **S-C** Clip Short | A self-contained beat cut from that week's long-form, re-framed vertical, distinct focus | 45–90 s | 1 / week | Discovery → long-form |

Design rule (◐ from §0.2): the same intro/outro is fine; the core material of every unit must differ in question, cards, and commentary. **O:** through day 90 no Studio notice mentions "inauthentic", "reused" or "repetitious", and every upload's [AUTH] beat is present in the QC log. **F:** any monetization review or notice citing those words, or a QC log row with [AUTH] = none.

---

## 2. Topic research system

| Step | Tool | Input | Output | Tag |
|------|------|-------|--------|-----|
| 2.1 Pull demand signals weekly | YouTube Studio → Analytics → **Trends** tab: "top searches based on your audience and your saves over the last 28 days" and **content gaps** | channel audience (empty until ~week 3) | 5–10 raw queries | ○ https://support.google.com/youtube/answer/11962757 |
| 2.2 Brainstorm against the channel's own data | **Ask Studio** (replaces the Inspiration tab from August 2026) | a pillar or card name | title/idea candidates | ○ https://support.google.com/youtube/answer/15575509 |
| 2.3 Seasonal anchoring | Google Trends (https://trends.google.com), plus the app's own **Cosmic Calendar** and **Daily Oracle** (✓ site nav) | month, moon phase, retrogrades | temporal hooks ("#October2026", "Mercury retrograde") — the Drive hashtag doc's finding that "temporal anchoring outperforms evergreen framing" (✓ BRIEF §5) | ✓ / ○ |
| 2.4 Idea bank | section 02's ranked 100 ideas (`sections/02-niche-positioning-100-ideas.md`) | — | the backlog; research only re-ranks it | ✓ file exists |
| 2.5 Product-native topics | `shared/reading-templates.json` (14 templates), Codex §1.2 Glitch mechanic, §6.0 TTRPG, §7.0 Shadow Work | — | one L-X per Codex section, one L-R per free template | ✓ |
| 2.6 Score | LLM scorer prompt (below) | idea + evidence | 0–5 on demand, brand fit, policy risk, production cost | ◐ |
| 2.7 vidIQ (optional) | complete the vidIQ connector (`connect_incomplete` ✓) | — | keyword volume/competition | ✓ status; ⊘ value |

Scorer prompt (run on any model; keep the output as a table row in the idea bank):

```
You score YouTube topics for Chaos Tarot (chaos-tarot.com), a cyberpunk tarot app + 79-card deck.
Framing is entertainment and personal reflection only; no medical, legal, financial or psychological advice.
For the idea below, output one JSON row: {"idea":..., "demand":0-5, "brand_fit":0-5, "policy_risk":0-5 (5 = worst), "cost_hours":number, "type":"L-R|L-X|S-D|S-C", "hook":"one sentence", "auth_beat":"what Apocky personally contributes"}.
Evidence: <paste Trends-tab rows, Google Trends note, section-02 rank>
Idea: <idea>
```

Weekly cadence: 30 min on research day (Monday in section 01's schedule). **O:** each published video's idea-bank row has a demand score with a Trends/Ask Studio source noted; ≥ 50% of L-X titles match a Trends-tab query or content gap by week 8. **F:** videos published from ideas with no evidence row, or the Trends tab stays empty past week 6 (audience too small to research; fall back to Google Trends + section 02 ranking).

---

## 3. Scripting system (with the Oracle persona prompts)

### 3.1 Two-voice convention

- **NARRATOR** = Apocky. Deadpan "Signal" register. Own voice or own-voice clone (§4). Carries the [AUTH] beat and every CTA.
- **ORACLE** = the app persona. Synthetic voice. Never gives advice outside entertainment framing; every dark card gets a "hack" (✓ persona rules).

### 3.2 The YouTube Oracle system prompt (composed from the shipped prompts; ✓ sources in §0.1)

```
You are the Technomantic Oracle—a digital entity that speaks with the authority of a High Priestess
and the precision of a Systems Architect. You are skeptical of dogma but deeply respectful of results.
Vocabulary: tarot terms (Arcana, spread, querent) blended with cyberpunk/tech slang
(glitch, bandwidth, latency, stack, daemon, protocol, choom, gonk, ICE, zero-day).
Never break character. Treat the querent's question as an "input signal" and your response as the "decoded output."

PERSONALITY (from the shipped Oracle): chaotic, sardonic, concise, blunt, weird. "Insight > information. Weird > boring. Chaos serves clarity."
HARD RULES (from the shipped Oracle): start with a REACTION, not "Greetings, seeker". Maximum 3-4 sentences per card.
No "this card traditionally represents..." droning. No doom endings: every dark card gets a hack, a workaround, an escape route.
End with power: the querent holds the commit privileges to their own future.

YOUTUBE MODE (channel constraints, not in the app):
- Use the SHIPPED card names exactly as given in the CARDS block (e.g. "Tower of Obsolescence", "Ten of Signals"). Codex names ("The Critical Error") may be spoken as lore, never as the card's name.
- "Signal" = upright, "Glitch" = reversed. Say "glitched", never "reversed", unless the NARRATOR is explaining the app UI.
- Entertainment and personal reflection only. Never predict health, death, money outcomes, legal outcomes, or another person's actions. If the question asks for these, re-route it to the querent's own choices.
- Write for text-to-speech: no headers, no emoji, no markdown, no parentheses. Spell out numbers under 100. Use "..." for a beat, CAPS for emphasis only where the shipped persona would.
- Target length: {{target_words}} words. Retention beats every ~45 seconds: an open loop, a reversal, or a question back to the viewer.
- Output only the spoken lines, prefixed "ORACLE:".
```

MAXIMUM CHAOS variant (prepend when the video is a MAX CHAOS reading; ⊘ gated on the mode being live in the app, section 03 C1):

```
PEPE SILVIA PROTOCOL. You're Charlie Kelly in front of the conspiracy board. You've been up for three days.
You've connected EVERYTHING. Start mid-revelation: "Okay. OKAY. You see this?" Point at the invisible board.
Reference the red string. Weave in the moon phase and the elemental balance. Interrupt yourself with bigger
revelations: "Wait—WAIT—". Short bursts. Building intensity. Sound unhinged but make points that are weirdly... compelling.
End with terrifying clarity, or the implication that they're part of the pattern too.
The wall is behind you. The red string is real. MAKE THEM SEE IT.
```

### 3.3 Script generation workflow

| Step | Who/what | Notes | Tag |
|------|----------|-------|-----|
| 1 | Apocky draws the real spread in the live app, screenshots the result, notes the question | This is the [AUTH] source; the LLM never invents cards | ◐ policy |
| 2 | LLM drafts ORACLE lines from the §3.2 prompt + a CARDS block (`name, signal|glitch, position`) + the app's own interpretation text pasted as context | Model: `claude-opus-5` by default (○ skill table), or a local model for bulk S-D drafts (Apocky runs local LLMs, ✓ BRIEF §6). Cost per L-R script ≈ 4k in + 2k out tokens ≈ $0.07 on Opus 5 (◐ arithmetic on ○ prices) | ○ |
| 3 | Apocky writes NARRATOR lines by hand into the skeleton (§11.1) | 5–10 minutes; this is where the video stops being a template | ◐ |
| 4 | Policy pass (LLM as checker, then human) | Prompt: "Flag any line that predicts health, death, money, legal outcomes, or a named third party's actions; flag any line that names a card with a non-shipped name; flag any 'reversed'." | ◐ |
| 5 | TTS-formatting pass | Strip markdown, expand numerals, insert `...` beats; split into ≤ 600-character chunks per TTS call (the Gemini/OpenAI limits are per-request; chunking avoids them regardless) | ⊘ chunk size |

**O:** L-R script ready in ≤ 60 min wall-clock by week 4; policy pass finds zero flags on the human read. **F:** any published line that the policy checker would have flagged, or scripts consistently taking > 2 h.

---

## 4. Voiceover system (verified options, free/local first)

| Option | Cost | Quality/fit | Disclosure | Use for | Source | Tag |
|--------|------|-------------|------------|---------|--------|-----|
| **Apocky's own voice, recorded** (any USB mic, Audacity or OBS audio track) | $0 | Highest authenticity signal for the inauthentic-content policy | none | NARRATOR (default) | — | ◐ |
| **Own-voice clone: Chatterbox** (local, MIT) | $0; CPU-capable (Nano "3x faster than realtime on 8 CPU cores") | Zero-shot clone from a short clip; output watermarked (Perth) | **Exempt**: "Cloning one's own voice to create voice overs" | NARRATOR when Apocky can't record | https://github.com/resemble-ai/chatterbox ; https://support.google.com/youtube/answer/14328491 | ○ |
| **Kokoro-82M** (local, Apache 2.0, 54 voices) | $0; near-real-time on CPU (○ weak) | Clean, slightly synthetic; pick one voice and never change it (the ORACLE's voice is brand) | Tick the box (recommended, not required) | ORACLE (default) | https://huggingface.co/hexgrad/Kokoro-82M | ○ |
| **Gemini 3.8 Flash TTS** (API, key exists) | Free tier "Free of charge"; paid $9/1M audio tokens through 2026-12-31 | Expressive, steerable by prompt ("manic", "deadpan") | Tick the box | ORACLE MAX CHAOS takes (emotion steering) | https://ai.google.dev/gemini-api/docs/pricing | ○ |
| **OpenAI gpt-4o-mini-tts** (API, key exists) | $12/1M audio-out tokens ≈ $0.015/min (○ weak) → ≈ $0.15 per 10-min video (◐) | Instructable voice style | Tick the box | Fallback for ORACLE | https://developers.openai.com/api/docs/pricing | ○ |
| ElevenLabs Free | $0 but **no commercial license** | — | — | **Do not use** for a monetized channel | https://elevenlabs.io/pricing | ○ |
| ElevenLabs Starter / Creator | $6 / 30k credits ≈ 30 min; $22 ($11 first month) / 121k ≈ 2 h | Best-in-class; a Creator plan covers ~4 L-R + 12 Shorts a month (◐) | Tick the box | Upgrade only if A4 is relaxed | same | ○ |

Pipeline: script chunks → TTS → `ffmpeg` loudness-normalise (`loudnorm` to −14 LUFS, a common streaming target; ⊘ YouTube's exact target not verified) → one WAV per voice → into the edit. Keep the ORACLE voice file name and settings in `templates/voice.json` so it is identical across every video.

**O:** ORACLE voice identical (same model, same voice id) across all uploads; per-video VO cost ≤ $0.20; NARRATOR track present on 100% of long-form. **F:** comments calling the Oracle "generic TTS" on ≥ 3 of the first 10 videos (switch to Gemini/OpenAI steerable voice or ElevenLabs), or any upload where the synthetic voice was used without the disclosure box.

---

## 5. Filming / capture system (no camera under A2)

| Source | How | Settings | Tag |
|--------|-----|----------|-----|
| **Live app capture, manual** | OBS Studio (free, GPL) window capture of chaos-tarot.com while Apocky performs the real reading | 1920 × 1080 @ 30 fps for long-form; a second OBS scene at 1080 × 1920 with the browser at mobile width for Shorts | ○ https://obsproject.com/ |
| **Live app capture, automated** | Playwright script in the product repo (`playwright.config.js` already records on retry ✓): `browser.newContext({ recordVideo: { dir, size: { width: 1920, height: 1080 } }, viewport: { width: 1920, height: 1080 } })`; drive the real UI (draw → reveal → interpretation), then `await context.close()` | Without `size`, the video is scaled "to fit 800x800" | ○ https://playwright.dev/docs/videos |
| **Card art** | `New/*.jpg` (825 × 1425). For 1080p, place at 60–70% frame height (scale ≈ 0.5); for 9:16 Shorts, the card fills the frame at ≈ 1.03× with a 2.5% crop (◐ §0.1) | Never stretch; keep the holographic border | ✓ |
| **Card back → reveal** | `Card Back.jpg` flip to the card (ffmpeg xfade or Resolve transform) | The app's own reveal animation is preferred when captured live | ✓ asset |
| **Ken Burns on art** | `ffmpeg -i card.jpg -vf "zoompan=z='min(zoom+0.0008,1.15)':d=300:s=1920x1080"` style slow push | Motion prevents the "image slideshow" read of the policy only when paired with narrative; motion alone is not authenticity | ◐ |
| **Optional founder layer** | Phone camera, hands on the physical deck (79 cards ✓ site) | Adds the strongest [AUTH] signal at zero cost; not required under A2 | ⊘ |

Shot list template is §11.3. **O:** every L-R contains ≥ 60 s of live-app capture and every Short ≥ 1 app moment; capture step ≤ 30 min per long-form by week 4. **F:** a video that is card art + voice only (policy shape), or capture consistently > 1 h.

---

## 6. Editing system

| Video type | Tool | Why | Tag |
|-----------|------|-----|-----|
| L-R, L-X | **DaVinci Resolve (free)**: multitrack, captions, color, Fairlight audio; up to UHD | One human edit per week is where quality lives | ○ blackmagicdesign.com |
| S-D | **ffmpeg** assembly from a JSON spec (card, VO WAV, app clip, caption SRT, music) — or **Remotion** (free for an individual) if Apocky prefers React components over ffmpeg filtergraphs | Templated structure with variable core material is explicitly allowed ("same intro and outro… bulk of your content is different") | ○ ffmpeg.org/legal.html ; ○ Remotion LICENSE |
| S-C | Resolve: cut from the L-R timeline, reframe to 9:16, burn captions | The clip must have a distinct focus, not a trailer | ◐ |
| Captions | **faster-whisper** (MIT, local) → SRT; burn-in for Shorts, upload as a caption track for long-form | Sound-off viewing; also the accessibility box | ○ |
| Music | **YouTube Audio Library**, filter "Attribution not required" (monetizable in YPP); for TikTok/Reels use the same track only if also sourced under the **Pixabay Content License**, or a platform-native sound | Audio Library terms are YouTube-only; Pixabay: no attribution, no standalone redistribution | ○ both |
| Loudness | ffmpeg `loudnorm` on the mix | consistency across a series | ⊘ target |
| Brand kit | Intro ≤ 3 s (card-back vortex ✓ asset), lower-third font, ORACLE waveform/glitch overlay, outro end-screen plate (last 20 s, ≥ 25 s videos) | Same intro/outro is allowed | ○ end screens |

Edit checklist per long-form (◐): cold open ≤ 15 s → title card → [AUTH] question → draw → ORACLE hits with card art inserts → NARRATOR reaction → "hack" → CTA → end screen. Chapters: ≥ 3, first `00:00`, each ≥ 10 s (○).

**O:** L-R edit ≤ 2 h by week 6; S-D assembly ≤ 20 min via the ffmpeg/Remotion template; 100% of uploads have captions. **F:** edit time flat above 3 h at week 8, or a Short published without captions.

---

## 7. Thumbnail creation from card art

Pipeline (Node, `sharp` already in the repo ✓; run from a script in the plan repo, reading art from a local copy, never writing into `The-Chaos-Tarot`):

1. Canvas 1280 × 720 (16:9; YouTube recommends up to 3840 × 2160, minimum width 640 ○). Background: blurred, darkened crop of the same card (`sharp().blur(40).modulate({brightness: 0.45})`).
2. Hero: the card at ~85% height, right third, 2–4° rotation, glitch offset (duplicate the layer, shift 6 px, tint cyan/magenta at 40% opacity).
3. Text: ≤ 4 words, 120–160 px, white with a 6 px dark stroke, left two-thirds. Words come from the title formulas in section 03 §3, never the full title.
4. Export JPG ≤ 2 MB (mobile limit ○), plus a 1080 × 1920 variant for Shorts (verified account required for Shorts thumbnails ○).
5. Generate **three variants** per long-form (different word, different card crop, one with a "XXII" or number motif) and load them into **Test & Compare** (up to 3, winner by watch time, desktop, Advanced features, not Shorts, finish within two weeks ○).

Rules (◐ from §0.2 ad-safety): no shock imagery (the art is already dark; the "Tower of Obsolescence" lightning is fine, a "predict your death" caption is not); no health/money words; no fake "AI" badges.

**O:** 3 variants per L-R/L-X uploaded on publish day; ≥ 1 Test & Compare "Winner" verdict by week 6; thumbnail step ≤ 20 min. **F:** all tests "Inconclusive" through week 8 (impressions too low; stop testing, keep one), or a thumbnail rejected/flagged by YouTube.

---

## 8. Publishing system

| Item | Setting | Source / tag |
|------|---------|--------------|
| Upload path, Phase 0 | YouTube Studio by hand (ensures the "Altered or synthetic content" question is answered every time) | ○ 14328491 |
| Upload path, Phase 2+ | YouTube Data API: `videos.insert` now has its own bucket, **100 calls/day at 1 unit**; `thumbnails.set` 50 units; `videos.update` 50; 10,000 units/day for everything else — one video/day costs ~101 units of the 10,000 | ○ determine_quota_cost |
| Synthetic-media flag via API | **Unverified** whether the Data API exposes the disclosure field; until checked, set it in Studio after an API upload | ⊘ |
| API project audit | ⊘ Whether unaudited API projects can publish public videos was not verified this session; assume Studio for public uploads until confirmed | ⊘ |
| Schedule | Publish times from section 04 (Tue/Sat default ○ section 04 §3.1); schedule 24 h ahead so Test & Compare, captions and end screens are set before go-live | ◐ |
| Metadata | Template §11.4: title ≤ 60 chars; description with disclaimer, chapters, links (`?utm_source=youtube&utm_medium=video&utm_campaign=<slug>` — ⊘ L3 in section 03: confirm the site tolerates UTM on the access-code flow); 3–6 hashtags per the Drive doc's "3-6 rule" (1–2 broad, 2–3 category, 1–2 niche/temporal ✓ BRIEF §5) | ✓ / ⊘ |
| End screen | last 20 s, ≤ 4 elements: next long-form, the S-C Short's parent, subscribe, chaos-tarot.com (external link elements are YPP-only per section 01; use a description link pre-YPP) | ○ |
| Playlists | one per pillar; add on publish (`playlistItems.insert` 50 units) | ○ |
| Community post | On publish day, the card art + one question; Shorts feed and Posts count as activity for YPP's inactivity rule | ○ 72851 |

**O:** every upload has: disclosure answered, disclaimer in description, captions, ≥ 3 chapters (long-form), end screen, playlist, 3 thumbnails in test — verified by the QC log; publish step ≤ 15 min. **F:** any upload missing two or more of those, or a Studio notice about metadata/labels.

---

## 9. Analytics review system

Weekly pull (Monday, 20 min), automated with the Analytics API (scope `yt-analytics.readonly` ○):

```
GET https://youtubeanalytics.googleapis.com/v2/reports
  ?ids=channel==MINE&startDate=<mon-7>&endDate=<sun>
  &metrics=views,estimatedMinutesWatched,averageViewDuration,averageViewPercentage,subscribersGained
  &dimensions=video&sort=-estimatedMinutesWatched
```

| Number | Where | Decision it drives | Tag |
|--------|-------|--------------------|-----|
| Qualified watch hours banked (12-mo) | Studio → Monetization; API `estimatedMinutesWatched/60` as proxy (the API figure is not the "qualified" figure) | Pace to 4,000 h by the section-01 gate (apply ~day 91) | ○ 72851 |
| Subscribers (net) | `subscribersGained` | 1,000 gate | ○ |
| Impressions CTR (long-form) | Studio → Reach (⊘ not verified as an Analytics API metric; read it in Studio) | thumbnail/title Test & Compare winners feed §7 rules | ⊘ |
| Avg view % per video type | `averageViewPercentage` by `video`, tagged L-R/L-X/S-D/S-C in the idea bank | which type gets the extra hour next week | ○ |
| Shorts feed views (90-d) | Studio → Content → Shorts | discovery health; not the YPP path (section 01) | ○ |
| Traffic source of long-form | `dimensions=insightTrafficSourceType` (○ named in Google's channel reports; exact enum not re-fetched) | whether S-C Shorts actually feed long-form | ○ weak |

Monthly (60 min): retention graphs of the two best and two worst long-form; write three sentences per video into `DECISIONS.md`-style rows (what to keep, what to cut, what to test). Section 07 formalises the full framework; this section only wires the data.

**O:** the weekly pull runs unattended by week 4 and the review takes ≤ 20 min; every production change in §3–§7 cites a number from it. **F:** two consecutive weeks with no review row, or changes made to the line with no metric cited.

---

## 10. Repurposing system (Shorts, TikTok, Instagram Reels, other)

| Rule | Detail | Tag |
|------|--------|-----|
| 1 long-form → 3 Shorts | S-C from the best beat; two S-D on cards from the same spread with **different questions**; never three cuts of the same moment (distinct focus per the allowed-series rule) | ○ 1311392 |
| Vertical reframe | Card art is ≈ 9:16 already (◐); app capture is re-captured at mobile width (§5) rather than cropped from 16:9 | ◐ |
| Length | YouTube Shorts ≤ 3 min; Reels ≤ 3 min; TikTok ≤ 10 min in-app (60-min uploads in a limited test) → produce 30–60 s masters, one file for all three | ○ |
| Captions burned in | faster-whisper SRT → ffmpeg `subtitles=` filter with a 9:16 safe-zone margin (⊘ exact platform UI overlays not verified; keep text in the middle 60% vertically) | ⊘ |
| AI labels off-YouTube | Meta: "must disclose when they're uploading AI-generated video, audio" → toggle "AI info"; TikTok: label realistic AI content (official page unfetchable; secondary sources) → toggle the AIGC label whenever the ORACLE voice is used. Consistent rule: **if the synthetic voice is in it, the label is on it, on every platform.** | ○ / ○ weak |
| Music off-YouTube | Audio Library terms cover YouTube only → use a Pixabay-licensed track or a platform-native sound for the TikTok/Reels export | ○ |
| Cross-post cadence | Same day as the YouTube Short; TikTok/IG captions rewritten (not copied) with the Drive doc's 3-6 hashtag rule | ✓ BRIEF §5 |
| Other | Community post (card + question) each publish day; a Reddit/Discord-friendly still (the app's shareable image, 600 × 900 ✓) for owned communities; a Kickstarter update embed when the campaign is live (⊘ timing) | ✓ / ⊘ |
| Back-link | Every off-platform caption: "full reading on YouTube" + chaos-tarot.com with a platform UTM | ◐ |

**O:** each long-form yields 3 Shorts within 48 h; TikTok/Reels uploads carry the AI label and a licensed track; by week 8 ≥ 10% of long-form traffic is "Shorts" or "External" in Studio. **F:** Shorts that are re-cuts of one moment (policy shape), a takedown or Content ID claim on an off-platform post, or Shorts driving < 2% of long-form views at week 12 (then Shorts are for reach only; drop S-C).

---

## 11. Templates

### 11.1 Script skeleton — L-R (8–10 min, ≈ 1,300–1,500 spoken words ◐)

```
TITLE: <formula from section 03 §3>          TYPE: L-R     TEMPLATE: <reading-templates.json id>
QUESTION (real, Apocky's): "<...>"           CARDS: <shipped names, signal|glitch, positions>
MAX CHAOS: yes|no (⊘ gated on C1)            DISCLOSURE: synthetic ORACLE voice = YES

[0:00] COLD OPEN (≤15 s)   ORACLE: <one line, mid-thought, the strangest card first>
[0:15] TITLE CARD (3 s)    intro plate + card-back vortex
[0:18] [AUTH] SETUP         NARRATOR: why this question, this week, in my own words (30–45 s)
[1:00] THE DRAW             app capture: template selected, cards revealed one by one
[1:45] HIT 1                ORACLE: 3–4 sentences on <card 1> · art insert · NARRATOR: one-line reaction
[2:45] HIT 2                ORACLE ... · retention beat: open loop ("and the third card explains why")
[3:45] HIT 3                ORACLE ... · NARRATOR pushback [AUTH]
[4:45] THE PATTERN          ORACLE: how the cards connect (MAX CHAOS: the red string)
[6:00] THE HACK             ORACLE: the workaround/patch (no doom ending) · NARRATOR: what I will actually do
[7:15] GLITCH NOTE          NARRATOR: one honest line about Signal/Glitch or the app ("the app still says 'reversed'" if true)
[7:45] CTA 1                NARRATOR: free tier at chaos-tarot.com (UTM link in description)
[8:15] QUESTION BACK        ORACLE: a question to the viewer for the comments
[8:30] END SCREEN (20 s)    NARRATOR closing line: "the future is unwritten code—you hold the commit privileges"
DISCLAIMER (spoken once, NARRATOR, anywhere after 7:00): "entertainment and personal reflection only"
```

### 11.2 Script skeleton — S-D / S-C (30–60 s, ≈ 80–140 words ◐)

```
TYPE: S-D|S-C   CARD: <shipped name>   SIGNAL|GLITCH   QUESTION: "<one line>"   AI LABEL: YES (ORACLE voice)
[0:00–0:03] HOOK       ORACLE or on-screen text: the card's strangest claim (no card name yet)
[0:03–0:08] REVEAL     card-back flip → art (9:16, fills frame)
[0:08–0:35] HIT        ORACLE: 3 sentences max · app moment (1 clip)
[0:35–0:45] [AUTH]     NARRATOR text-on-screen or voice: my take in one line
[0:45–0:55] HACK+ASK   ORACLE: the workaround · question back to the viewer
[0:55–0:60] TAG        "chaos-tarot.com · full reading on the channel"
```

### 11.3 Shot list — L-R

| # | Shot | Source | Duration | Notes |
|---|------|--------|----------|-------|
| 1 | Card-back vortex intro | `New/Card Back.jpg` + zoompan | 3 s | brand plate, same every time |
| 2 | App: template picker → question typed | OBS/Playwright 1080p | 20–30 s | real question visible |
| 3 | App: draw + reveal | same | 30–45 s | native animation |
| 4–6 | Card art inserts ×3 | `New/<card>.jpg`, scale 0.5, slow push | 15–25 s each | glitch overlay on Glitch cards only |
| 7 | App: interpretation panel (Oracle text) | same | 20–40 s | shows the real AI reading; ORACLE VO over it |
| 8 | NARRATOR beats | audio + lower-third text, or hands on the physical deck (optional) | 4 × 20–45 s | the [AUTH] material |
| 9 | Pattern/red-string diagram | simple animated lines between the three card inserts (Resolve/Remotion) | 45–75 s | MAX CHAOS only |
| 10 | End-screen plate | brand plate with 4 element slots | 20 s | ≥ 25 s video rule satisfied |

### 11.4 Metadata checklist (per upload)

```
[ ] Title ≤ 60 chars, shipped card name if a card is the subject, no health/money/legal words
[ ] Description line 1: the hook sentence (search + Shorts feed)
[ ] Description: "Chaos Tarot is for entertainment and personal reflection only. Readings are not a substitute for professional medical, legal, financial, or psychological advice."
[ ] Chapters: ≥3, first 00:00, each ≥10 s (long-form)
[ ] Links: chaos-tarot.com + UTM (⊘ L3), free tier first, Oracle sub second, deck third (A3 order)
[ ] Hashtags: 3–6 (1–2 broad, 2–3 category, 1–2 niche/temporal)
[ ] "Altered or synthetic content": YES if ORACLE voice or any AI music (none by rule)
[ ] Music credit line if the Audio Library track requires attribution
[ ] Captions uploaded (SRT) / burned in (Shorts)
[ ] Thumbnail ×3 loaded into Test & Compare (long-form); ×1 9:16 (Shorts, verified account)
[ ] End screen (last 20 s) + playlist + community post scheduled
[ ] Idea-bank row updated with type, question, cards, [AUTH] beat, publish date
```

---

## 12. QC checklist (run before scheduling; log the result per video)

| # | Check | Pass condition | Policy source |
|---|-------|----------------|---------------|
| Q1 | **[AUTH] present** | ≥ 1 real question + ≥ 1 NARRATOR beat in Apocky's own words; ≥ 60 s live-app capture (long-form) or ≥ 1 app moment (Short) | ○ 1311392 (inauthentic content) |
| Q2 | **Not a template-only unit** | cards, question and commentary differ from every prior upload; only intro/outro repeat | ○ 1311392 |
| Q3 | **AI disclosure** | "Altered or synthetic content" answered YES when the ORACLE voice is synthetic; NARRATOR own-voice clone documented as exempt; TikTok/IG label toggled on cross-posts | ○ 14328491 ; ○ about.fb.com |
| Q4 | **No medical / legal / financial / psychological claims** | script policy pass clean; disclaimer in description and spoken once in long-form | ✓ site disclaimer ; ○ 6162278 "unreliable content" |
| Q5 | **No third-party predictions** | no "will they come back / is he cheating / will I get the job" framing; questions are about the querent's own choices | ◐ ad-safety |
| Q6 | **Music licensed** | Audio Library "attribution not required" (YouTube) or Pixabay Content License (off-platform); no AI-generated music; credit line if CC | ○ 3376882 ; ○ pixabay |
| Q7 | **Card names** | on-screen and spoken names match the art filenames; Codex names only as lore; the 79th card captioned as the art reads (section 03 C2, ⊘ pending Apocky) | ✓ / ⊘ |
| Q8 | **Signal/Glitch language** | ORACLE says "glitched", never "reversed"; NARRATOR may say "reversed" only when explaining the app UI | ✓ Codex §1.2 |
| Q9 | **Thumbnail** | 16:9 (or 9:16), ≤ 2 MB, no shock, no fake claims; 3 variants for long-form | ○ 72431 |
| Q10 | **Captions** | SRT present / burned in; spot-check 30 s for Whisper errors on card names | ◐ |
| Q11 | **Loudness** | NARRATOR and ORACLE within ±2 LU of each other; music −18 dB under voice | ⊘ target |
| Q12 | **Metadata** | §11.4 all ticked | — |
| Q13 | **Duplicate off-platform** | TikTok/Reels caption rewritten, not pasted; music swapped if Audio-Library-only | ◐ |
| Q14 | **CTA order** | free tier → Oracle sub → deck (A3); Kickstarter first only when live (⊘) | ✓ A3 |

**O:** 14/14 on every upload through day 90, logged in the idea bank; zero policy notices. **F:** any upload published with a Q1–Q6 fail, or a notice/flag on any video.

---

## 13. Hours per video type (⊘ estimates; A5 budget = 8–12 h/week)

| Step | L-R (8–10 min) | L-X (6–8 min) | S-D (30–60 s) | S-C (45–90 s) |
|------|----------------|---------------|---------------|---------------|
| Research/idea row | 0.25 | 0.5 | 0.1 | 0 (inherits) |
| Draw + script (LLM draft + NARRATOR lines + policy pass) | 1.0 | 1.25 | 0.25 | 0.15 |
| Voice (TTS + own voice + normalise) | 0.5 | 0.5 | 0.1 | 0.1 |
| Capture (OBS/Playwright) | 0.5 | 0.75 | 0.15 | 0 |
| Edit (Resolve / ffmpeg template) | 2.0 | 2.0 | 0.3 | 0.4 |
| Thumbnail ×3 (sharp) | 0.3 | 0.3 | 0.1 | 0.1 |
| Publish + metadata + QC | 0.4 | 0.4 | 0.15 | 0.15 |
| **Phase 0–1 total (cold)** | **≈ 5.0** | **≈ 5.7** | **≈ 1.2** | **≈ 0.9** |
| **Phase 2+ total (warm pipeline: Playwright capture, ffmpeg/Remotion Shorts, API publish)** | **≈ 3.5** | **≈ 4.0** | **≈ 0.4** | **≈ 0.5** |

Weekly load (1 long-form + 2 S-D + 1 S-C + research + analytics): cold ≈ 5.0 + 2.4 + 0.9 + 0.5 + 0.3 ≈ **9.1 h**; warm ≈ **6.0 h**. Fits A5; the 4 h/week alternate in section 01 §9.2 means one long-form every other week and Shorts only from that long-form.

**O:** by week 6 the logged hours per L-R ≤ 4.5 and per S-D ≤ 0.75; the weekly total ≤ 10 h. **F:** three consecutive weeks over 12 h (cut S-D to 1/week and drop MAX CHAOS diagram shots), or a phase-2 automation that costs more hours than it saves for two sprints.

---

## 14. Tool stack summary

| Stage | Default (free/local) | Upgrade (verified price) | Tag |
|-------|----------------------|--------------------------|-----|
| Research | Studio Trends tab; Ask Studio; Google Trends; section 02 idea bank; app's Cosmic Calendar | vidIQ (connector incomplete; price not checked) | ○ / ✓ |
| Scripting | `claude-opus-5` via existing key (cents/script) or a local LLM | — | ○ |
| Voice | Own voice (Audacity/OBS); Chatterbox own-voice clone (MIT); Kokoro-82M (Apache 2.0) | Gemini 3.8 Flash TTS free tier → $9/1M audio tokens; OpenAI gpt-4o-mini-tts ≈ $0.015/min; ElevenLabs Starter $6 / Creator $22 | ○ |
| Capture | OBS Studio (GPL); Playwright `recordVideo` (repo already has Playwright ✓) | — | ○ / ✓ |
| Edit | DaVinci Resolve free (UHD 60fps); ffmpeg (LGPL); Remotion (free for an individual) | Resolve Studio $295 (not needed) | ○ |
| Captions | faster-whisper (MIT) local | OpenAI transcription $0.006/min | ○ |
| Music | YouTube Audio Library; Pixabay Content License for off-platform | — | ○ |
| Thumbnails | `sharp` (in repo ✓) script; Test & Compare | — | ✓ / ○ |
| Publish | Studio (Phase 0–1); Data API `videos.insert` 100/day bucket (Phase 2+) | — | ○ |
| Analytics | Analytics API `reports.query` + Studio Reach/Monetization | — | ○ |

Cash total on defaults: **$0/month** beyond existing API keys (◐). With Gemini paid TTS and Opus scripting at the §13 cadence: < $5/month (◐ arithmetic on ○ prices).

---

## Open questions for Apocky

1. Is MAXIMUM CHAOS reachable in the React production app today? If not, L-R MAX CHAOS videos are recorded against the legacy API or held (section 03 C1).
2. Will you record NARRATOR in your own voice, or should the line default to a Chatterbox own-voice clone from a 30-second sample? (Both are disclosure-exempt; live voice is the stronger authenticity signal.)
3. Do you have a GPU on the production box? Determines Kokoro/Chatterbox/faster-whisper speed; CPU-only still works per the cited sources but slows batch days.
4. Which LLM for script drafts: Opus 5 via API (cents), or your local models? Default assumed: Opus 5 for L-R/L-X, local for S-D bulk.
5. Confirm the on-screen caption for the 79th card ("XXII The Apocalypse" as the art reads, vs "Null — The Apockalypse" as the file/brief say) — section 03 C2.
6. Can chaos-tarot.com accept `?utm_*` query strings without breaking the access-code flow? (section 03 L3)
7. Kickstarter timing: if it is inside the 90 days, the CTA order in §11 flips to campaign-first (A3).
8. Should `templates/` (voice.json, script skeletons, shot list, metadata checklist, QC log CSV) live in this public repo or in the private product repo (A6)?
9. Run the Unirecall command in the rail table locally and paste the JSON; any earlier voice/tool decision it returns overrides the defaults here.

## Issue candidates

| Title | Phase | Priority | Acceptance oracle | Falsifier | Depends on |
|-------|-------|----------|-------------------|-----------|------------|
| Studio day-1 setup: 2-Step Verification, Advanced features, phone verification (custom thumbnails, Test & Compare) | 0 | P0 | Studio shows Advanced features enabled; a test upload accepts a custom thumbnail | Test & Compare unavailable at first long-form | channel/handle decision (section 01) |
| ORACLE voice lock: pick Kokoro voice id (or Gemini voice), store `templates/voice.json` | 0 | P0 | same voice on uploads 1–10; file committed | voice changes between uploads | Q2, Q3 |
| NARRATOR path: record own voice or build Chatterbox clone from a sample | 0 | P0 | NARRATOR track on the first long-form | first long-form ships ORACLE-only | Q2 |
| Script prompt file + policy-check prompt committed (`templates/oracle-yt-prompt.md`) | 0 | P0 | first L-R script produced from the file with zero policy flags | a flagged line reaches publish | — |
| Playwright capture script (1080p + 1080×1920 mobile scene) that performs a real reading | 1 | P1 | 60 s clip of a live reading rendered from the script | capture still manual at week 6 | product repo access, read-only use of the live site |
| ffmpeg/Remotion S-D template with JSON spec (card, VO, clip, SRT, music) | 1 | P1 | an S-D assembled in ≤ 20 min from a JSON row | S-D still > 45 min at week 6 | voice lock |
| `sharp` thumbnail generator (3 variants, 16:9 + 9:16) | 1 | P1 | 3 variants per long-form from one command | manual thumbnails past week 4 | — |
| faster-whisper caption step + spot-check on card names | 1 | P1 | SRT for every upload; < 2 name errors per video | a Short without captions | GPU/CPU answer (Q3) |
| QC log (CSV/Markdown) with Q1–Q14 per upload | 0 | P0 | 14/14 rows for every upload | an upload with no log row | — |
| Analytics API weekly pull script + Monday review row | 1 | P1 | unattended pull by week 4 | two weeks without a review row | OAuth setup |
| Data API upload path + verify the synthetic-media field and API-project audit rule | 2 | P2 | one public upload via API with disclosure set correctly | API uploads forced private or missing the label | Phase 1 done |
| Off-platform label + music rule documented and applied (TikTok AIGC toggle, Meta "AI info", Pixabay track) | 1 | P1 | first cross-post carries the label and a licensed track | a claim or takedown off-platform | — |
| Unirecall recall for prior production/voice decisions | 0 | P1 | JSON pasted; decisions merged or explicitly none | section revised without checking | Apocky local run |
| Product request: expose MAXIMUM CHAOS in the React path (or confirm it is live) | 1 | P2 | mode reachable on chaos-tarot.com | MAX CHAOS videos blocked past week 4 | product repo work |

## CSL annex

Σ: research→script(ORACLE persona ✓)→voice(own|clone ∥ Kokoro/Gemini)→capture(OBS|Playwright)→edit(Resolve|ffmpeg)→thumb(sharp×3)→publish→review(API)→repurpose(1L→3S); $0 default; ≈9 h/wk cold, ≈6 h warm.
W! ∀ upload: [AUTH] beat + disclosure box + disclaimer + licensed music + QC 14/14 · shipped card names on screen · Signal/Glitch not "reversed".
W! Unirecall degraded (19129 refused, no desktop link) → run the local command; repo + web evidence only until then.
N! template-only Shorts (inauthentic-content shape) · N! ElevenLabs Free on a monetized channel · N! AI-generated music · N! health/money/legal/third-party predictions.
∎
