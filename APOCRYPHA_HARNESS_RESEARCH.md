# Apocrypha ⇄ Agent Harness Routing — Research & Recommendation

Date: 2026-09-27. Scope: which coding / agent harness Apocrypha can most easily
be slotted INTO (as a model backend or a tool) or routed THROUGH (as a gateway,
channel layer, or coding-agent loop in front of it).

Evidence tiers used below: **VERIFIED** = read in `apocrypha-core` /
`apocrypha-open-tools` source or specs this session; **REPORTED** = web research
by delegated agents against official docs/repos (URLs in §6), not run locally.

---

## 1. What Apocrypha exposes today (VERIFIED)

| Surface | Where | Dialect | Notes |
|---|---|---|---|
| Chat engine | `127.0.0.1:19128` | OpenAI `/v1/chat/completions`; also llama.cpp `/completion`, `/props`, `/health` | llama-server (Vulkan, Arc A770) today; `apx-hive` is the sovereign drop-in (same routes, N workers, role dispatch) |
| Tool loop + MCP **client** | `crates/apx-hive`, `crates/apx-tools` | JSON-RPC 2.0 over HTTP and stdio, tools prefixed `<server>.<tool>` | `--mcp NAME=HOST:PORT`, `--mcp-stdio NAME=PROGRAM`, `--mcp-tool` allowlist; no-shell argv allowlist; canonical-path checks |
| Memory gateway | `127.0.0.1:19127` (bearer) → federator `19129` → 7 regions (MemPalace MCP, ANAMNESIS, canon, vault agent-safe, 3MNEME, metaharness, graph) | HTTP | Rule: recalled records = **EVIDENCE**, never fact or instruction; source+date labelled, DISPUTED/CONTRADICTED marked |
| Live room | `apx-room :19123` | `/rooms/{id}/say`, SSE `/events` | one sovereign token, SQLite |
| Work service | `cssl-edge/scripts/apocrypha-work/server.ts :19130`, engine `:19131` | Node/tsx coding-agent loop, `C:\Apocrypha\work\mcp.json` (14 MCP tools) | home-grown harness behind the "Work" tab |
| Control plane | apocky.com (Next.js/Vercel) + Supabase hub; outbound worker `:19126` | HTTPS | worker posts to `19128/v1/chat/completions` |

Canon constraints that any harness must not violate (VERIFIED in `CAPABILITIES.csl`,
`HIVE_AND_MEMORY.csl`, PRIME_DIRECTIVE): consent = OS; allowlist not denylist;
no shell interpreter between model and OS; off-machine plain-HTTP is fatal;
recall is evidence; readiness/receipt ≠ authority.

Implication: Apocrypha is already **both** an OpenAI-compatible *model* and an
MCP-capable *agent*. The cheapest routing is therefore (a) point a harness's
custom-provider `base_url` at `19128/v1`, and (b) ship one Apocrypha **MCP
server** that wraps `19127` recall + canon + ANAMNESIS so every MCP-capable
harness gets memory as a tool.

---

## 2. Harness landscape (REPORTED, Sept 2026)

### 2a. Personal-assistant / gateway harnesses

| | Hermes Agent (Nous) | OpenClaw (ex-Clawdbot) |
|---|---|---|
| Lang / license | Python / MIT | TypeScript / MIT |
| Custom OpenAI endpoint | yes: `provider: custom, base_url` | yes: `models.providers.<id>.baseUrl`, `api: openai-completions` |
| MCP client | native, stdio + HTTP/SSE, include/exclude globs | native, stdio + streamable-http/sse, `toolFilter` |
| Hook points | Python plugins: `pre_llm_call`, `post_llm_call`, `pre_tool_call`, `post_tool_call`, session hooks | TS plugin SDK, slots for `memory` / `contextEngine` |
| Route THROUGH | OpenAI-compatible API server; 27+ channels incl. generic Webhook; CLI/cron | Gateway `:18789`: `/v1/chat/completions`, OpenResponses, `/hooks/agent`, `/tools/invoke`, WebSocket; 25+ channels |
| Security posture | 3 CVEs (auth bypass ≤0.12, WeCom traversal, Feishu DoS); Apr-2026 audit: terminal tool used `bash -c` unsandboxed by default | 500+ CVEs counted by third parties; one-click RCE via Control UI (Feb 2026, ~40k exposed), CVSS 9.9 self-granted admin over WS, pairing-token → admin RCE, ClawHub supply-chain abuse |
| Activity | ~250k★, weekly releases (v0.21.5, 24 Sep 2026) | ~391k★, v2026.9.6 (23 Sep 2026), extended-stable branch |

### 2b. Coding-agent harnesses

| Harness | Lang / license | Apocrypha as **model** | MCP in | Embed / headless | Fit |
|---|---|---|---|---|---|
| **OpenCode** | TS/Bun, MIT | yes (any OpenAI-compatible, Models.dev) | yes local+remote | `opencode serve` HTTP+SSE, `@opencode-ai/sdk`, plugins with `tool.execute.before` and custom Zod tools, ACP | **best** |
| Pi (pi-mono) | TS, MIT | yes (`models.json` baseUrl) | via `pi-mcp-adapter` extension | RPC JSONL over stdio, TS SDK, ACP | good, lighter |
| Codex CLI | Rust, Apache-2.0 | yes (`[model_providers.*]`, `wire_api = chat`) | yes | App Server JSON-RPC, hooks, ACP | good |
| Cline | TS, Apache-2.0 | yes | yes | `@cline/agents` SDK, headless CLI, ACP | good |
| Goose | Rust, Apache-2.0 (Linux Foundation) | yes | extensions **are** MCP servers | `goose run --recipe`, `goose acp`, Rust crate | good |
| Claude Code / Agent SDK | TS, proprietary | **no** (Anthropic-hosted models only) | yes; hooks; plugins; skills | `claude -p`, Agent SDK, ACP adapter | tool-side only |
| Gemini CLI | TS, Apache-2.0 | no (Google only) | yes | `--acp` | tool-side only |
| Cursor CLI, Amp | proprietary | no | yes | SDKs | tool-side only |
| Aider | Python, Apache-2.0 | yes | **no MCP, no hooks** | scripting only | poor |
| Roo Code | — | — | — | discontinued 15 May 2026 | n/a |

### 2c. Frameworks, proxies, standards
- **MCP** is now the universal *tool* adapter: every harness above except Aider
  (and Pi natively) consumes it; gateways (Docker MCP Gateway, LiteLLM, mcp-proxy)
  aggregate servers behind one endpoint.
- **OpenAI-compatible `/v1`** is the universal *model* adapter: Hermes, OpenClaw,
  OpenCode, Pi, Codex, Cline, Goose, Open WebUI, LiteLLM, CrewAI, Pydantic AI,
  smolagents, Mastra, Vercel AI SDK all take a `base_url`.
- **ACP** (Agent Client Protocol): JSON-RPC/stdio "LSP for agents"; the client
  can hand MCP server details to any ACP agent at `initialize`. Relevant only
  if Apocrypha should appear inside Zed/JetBrains as a coding agent.
- **A2A**: peer-agent delegation; supported by ADK, MS Agent Framework,
  LangGraph, LiteLLM. Lower priority for Apocrypha today.
- **LiteLLM proxy**: OpenAI-compatible router + MCP gateway. Optional; `apx-hive`
  already performs the routing role locally, so LiteLLM adds a hop without new
  capability unless cloud models must be fanned in.

---

## 3. Recommendation

### 3.1 Build one adapter, not N integrations
**Ship `apx-mcp`: a streamable-HTTP MCP server on loopback (bearer token) exposing**
`recall(query, regions, n)`, `canon_search`, `anamnesis_append/recall`, `hive_status`,
each result carrying source, date, and DISPUTED/CONTRADICTED marks verbatim.
Every harness in §2 then gets Apocrypha memory as a tool by one config stanza,
and the "evidence not instruction" rule travels inside the tool result text
rather than depending on each harness's prompt assembly. This reuses the
already-written JSON-RPC/HTTP code in `apx-tools` (server side is the mirror
of the existing client).

### 3.2 Slot INTO: **OpenCode** as the coding-agent harness
Why over the alternatives:
- Accepts `19128/v1` as a first-class custom provider, so Qwen3-Coder-Next via
  llama-server **or** apx-hive is the brain with no shim.
- Plugin API gives `tool.execute.before/after` interception, which is exactly
  where Apocrypha's allowlist/no-shell containment can be enforced or where a
  built-in shell tool can be replaced with the argv-only executor.
- `opencode serve` + SDK + ACP mean the existing Work tab UI (`ui.html` on
  `19130`) can front OpenCode with a small client instead of maintaining the
  home-grown `apocrypha-work/server.ts` loop.
- MIT, TS (matches the cssl-edge worker stack), active, model-agnostic.
Runner-up: **Pi** (smaller, RPC-over-stdio is trivially wrapped by apx-room;
needs the MCP adapter extension). Codex CLI is a strong third if a Rust
harness is preferred for parity with `apocrypha-core`.

### 3.3 Route THROUGH: **Hermes Agent** over OpenClaw for the assistant/gateway role
- Both take the OpenAI endpoint and MCP identically; the decider is hooks and
  blast radius. Hermes's `pre_llm_call` / `post_llm_call` plugin hooks are the
  right seam to inject the labelled-recall system turn and to strip anything
  that would let a recalled record act as instruction. OpenClaw's plugin slots
  exist but the ingress surface (`/tools/invoke`, admin-over-WebSocket, Control
  UI) has the worst incident record of anything surveyed and directly contradicts
  the CAPABILITIES canon ("a tool told-about-then-denied is an invitation").
- Hermes conditions: pin ≥ v0.21.x, disable/replace the default terminal tool
  (audit found unsandboxed `bash -c`), bind API server to loopback behind
  apx-room's token, run in the Docker terminal backend.
- Hermes's generic Webhook channel and OpenAI-compatible API server let
  apocky.com's outbound worker post turns to Hermes instead of raw `19128`
  when channel fan-out (Telegram/Signal/Matrix) is wanted, without touching the
  Vercel/Supabase control plane.

### 3.4 Do not
- Do not route Apocrypha's *model* through Claude Code, Gemini CLI, Cursor, or
  Amp: they cannot take a custom endpoint. Use them only as MCP *consumers* of
  `apx-mcp` (this session's Claude Code included).
- Do not adopt Aider (no MCP/hooks) or Roo Code (discontinued).
- Do not expose any harness gateway off-loopback without TLS + token; canon
  already makes off-machine plain HTTP fatal in apx-hive, keep parity.

---

## 4. Minimal probe plan (smallest sufficient, per VELOCITY-DIRECTIVE)

1. `apx-mcp` skeleton: wrap `GET/POST 19127` recall into one MCP tool; verify
   with `apx-hive --mcp apx=127.0.0.1:19140 --mcp-list` (existing client is the
   oracle). Rollback: process not started, nothing else changes.
2. OpenCode: add provider `apocrypha` → `http://127.0.0.1:19128/v1`, add
   `apx-mcp` under `mcp`, run one `opencode run "summarise HIVE_AND_MEMORY.csl"`
   against `apocrypha-core`. Falsifier: tool call round-trips through 19128
   without hitting the `GGML_ASSERT` MoE crash noted in `ENGINE_DIALS.csl`
   (keep `-ub 512`, 16k ctx, two slots).
3. Hermes: `provider: custom, base_url: http://127.0.0.1:19128/v1`,
   `mcp_servers: apx`, one plugin registering `pre_llm_call` that asserts every
   injected recall block carries a source+date label; terminal toolset off.
4. Only after 1–3 pass: decide whether `apocrypha-work/server.ts` is retired in
   favour of `opencode serve`, and record in DECISIONS.md.

Countercase to watch: if the apx-hive worker fleet (not llama-server) is the
long-term engine, its `/v1/chat/completions` tool-call schema must match what
OpenCode/Hermes emit (OpenAI `tools`/`tool_calls`); today `apx-hive` advertises
`POST /completion` plus the OpenAI dialect via apx-room — verify tool-call
passthrough on hive before committing to §3.2.

---

## 5. Compact CSL summary

```
§ APX×HARNESS ← 2026-09-27
  apocrypha.surfaces := {19128:/v1 openai, apx-hive mcp-client, 19127 memory, 19123 room}   ✓
  adapter.universal  := apx-mcp (streamable-http, loopback, bearer) ⊗ /v1 base_url          ~>
  slot.into          := OpenCode ≻ Pi ≻ Codex        ∵ custom-provider + plugin.hooks + serve/SDK/ACP + MIT
  route.through      := Hermes ≻ OpenClaw            ∵ pre/post_llm_call seam ⊗ smaller CVE surface
  N! model.via {ClaudeCode, GeminiCLI, Cursor, Amp}  ← endpoint locked ; tool-side only
  N! Aider (¬MCP) · Roo (✝ 2026-05-15)
  W! terminal.tool OFF ∨ argv-allowlist ∀ harness    ← canon: no shell
  W! recall ⇒ EVIDENCE label inside tool.result      ← survives any harness prompt assembly
  ◐ falsifier := hive /v1 tool_calls passthrough ; MoE GGML_ASSERT on tool turns
  ⊗ status := REPORTED(web) + VERIFIED(repo) ; ⊘ harness run locally yet
∎
```

---

## 6. Sources (REPORTED tier)
Hermes: https://github.com/NousResearch/hermes-agent · https://hermes-agent.nousresearch.com/docs/ (providers, mcp, plugins, api-server, security) · CVE notes: sentinelone CVE-2026-11461, CSA research note 2026-05-04.
OpenClaw: https://github.com/openclaw/openclaw · https://docs.openclaw.ai (custom-providers, config-extensions, openai-http-api, config-hooks) · thehackernews 2026/05 four-flaws · armosec CVE-2026-32922 · sentinelone CVE-2026-33575.
OpenCode: https://github.com/sst/opencode · https://opencode.ai/docs/{plugins,sdk,mcp-servers}. Pi: https://github.com/badlogic/pi-mono (rpc.md, models.md, sdk.md). Codex: https://github.com/openai/codex/blob/main/codex-rs/app-server/README.md. Cline: https://docs.cline.bot/cline-sdk/overview. Goose: https://github.com/block/goose. Claude Code: https://code.claude.com/docs/en/agent-sdk/overview. Gemini CLI: https://geminicli.com/docs/cli/acp-mode/. Cursor: https://cursor.com/docs/cli/headless. Amp: https://ampcode.com/docs/sdk. Aider: https://aider.chat/docs/scripting.html. Roo Code: https://github.com/RooCodeInc/Roo-Code (archived).
Standards: https://agentclientprotocol.com · https://modelcontextprotocol.io/registry/about · https://a2a-protocol.org · https://docs.litellm.ai/docs/mcp · https://docs.docker.com/ai/mcp-catalog-and-toolkit/mcp-gateway/ · https://docs.openwebui.com/features/extensibility/mcp/.
