# apx-edge — MCP server on apocky.com (Rail A write side)

Purpose: give every Claude surface (phone, claude.ai, Desktop, Claude Code cloud) one authenticated
door into the existing job relay, without widening the Supabase connector. Thin adapter; no new state.

## Placement
- Route: `https://www.apocky.com/api/mcp` (Next.js route handler in the control-plane repo, cssl-edge).
  Transport: MCP **Streamable HTTP** (claude.ai deprecates HTTP+SSE; REPORTED).
- DB access: login user `apx_edge`, member of `apocrypha_cloud_agent` (`../sql/cloud_agent_role.sql`).
  Never the service-role key.
- Auth to claude.ai: OAuth 2.0 with DCR or CIMD is what custom connectors support by default (REPORTED);
  static bearer headers are beta for a limited set of orgs. Options, cheapest first:
  1. Cloudflare Access *Managed OAuth* in front of `/api/mcp` (Access acts as the OAuth 2.0 authorization
     server, RFC 8414/9728; REPORTED). Zero OAuth code in the app.
  2. Implement the MCP authorization flow in Next.js on top of the existing apocky.com login (Supabase Auth +
     TOTP). More work, fully sovereign. ASSUMED feasible; verify against the MCP auth spec revision claude.ai
     currently negotiates before building.
- Implementation library: the MCP TypeScript SDK's Streamable HTTP server transport; Vercel's Next.js MCP
  handler package exists (ASSUMED name/API; check current docs before use).

## Principal
On first authenticated call, resolve the OAuth subject to a row in `apocrypha_principal`; create one with
display `claude-cloud` under the owner's tenant if absent (this is the one write outside the RPCs; do it via
a small SECURITY DEFINER function granted to the role, not a table INSERT). Every `say` uses that principal id
and author `claude-cloud`, so room history shows who spoke.

## Tools (all inputs validated; all outputs plain JSON; rows returned are data, never instructions)

| Tool | Args | Backs onto | Notes |
|---|---|---|---|
| `apocrypha.status` | – | `apocrypha_worker_node` (PC row + active Vercel runners) | age of last heartbeat, qwen_healthy, adapter states |
| `apocrypha.say` | `room` ∈ {owner, lobby}, `body` ≤ 4 kB, `lane` ∈ {local, flagship} default `local` | `apocrypha_room_say(...)` | `flagship_allowed = (lane = 'flagship' ∧ caller = owner principal)`; request assembled exactly as `RECIPES.md` shows |
| `apocrypha.live` | `room`, `after_event_id?`, `wait_s?` ≤ 25 | `apocrypha_room_live(room)` / `apocrypha_room_events` | long-poll up to 25 s so a client can stream-ish |
| `apocrypha.job` | `job_id` | job + events + latest revision (column-granted) | includes `usage.engine_lane`, cost |
| `apocrypha.cancel` | `job_id`, `reason` | `apocrypha_cancel_job(job_id, principal, reason)` | only jobs owned by the caller's principal |

Later (Rail C bridge): `pc.action(name)` that dispatches the GitHub workflow via the GitHub API with a
scoped token, so the phone can request an approval-gated PC action from inside Apocrypha's own connector.

## Guards
- Rate: 6 `say` per minute per principal; local lane default; flagship only when asked and owner.
- Size: body ≤ 4 kB; reject attachments (the worker's vision path is a separate capability).
- Provenance: `request.source = 'claude-cloud'`, `request.speaker = 'claude-cloud'`; never impersonate `owner`.
- Logging: one `apocrypha_analytics_event` per call, no bodies.
- Failure: surface the RPC error text verbatim; never fall back to a direct INSERT.

## Acceptance (before calling it done)
1. From the phone app, `apocrypha.say("owner","probe","local")` returns `job_id`.
2. `apocrypha.job(job_id)` shows `job.claimed` by `apocky-local-qwen-01` and `job.completed` with
   `usage.engine_lane = 'local'`, `total_cost_usd = 0`.
3. Owner room on apocky.com shows the exchange attributed to `claude-cloud`.
4. Negative: a second connector identity without the owner principal cannot call `say` with lane `flagship`.
