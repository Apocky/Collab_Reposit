# Apocrypha Cloud → PC Bridge — verified state and robust design

Date: 2026-09-28 (UTC). Author: Claude cloud session, on Apocky's request.
Goal: any Claude cloud session (Claude Code on the web, the Claude phone app, Claude Desktop)
can observe, converse with, and operate Apocrypha on the Windows PC, under Apocky's consent.

Evidence tiers: **VERIFIED** = observed this session (live queries, HTTP probes, source read).
**REPORTED** = official docs fetched by a delegated research agent (URLs in §9). **ASSUMED** = stated as such.

---

## 0. Verdict

1. The wall is thinner than it looked. The cloud container reaches the public internet without an
   allowlist (VERIFIED). Only the PC's loopback is unreachable, and the PC already dials out to a
   durable job relay that a cloud session can read end-to-end (VERIFIED).
2. Cloud → PC **writes** are blocked today by one setting, not by architecture: the claude.ai Supabase
   connector runs as `supabase_read_only_user` and the relay's RPC functions grant EXECUTE only to
   `postgres` and `service_role` (VERIFIED, error 42501 on the live probe).
3. Robust design = four rails, each dial-out only, each with its own blast radius. Rail A (existing relay
   + a small MCP edge on apocky.com) is the universal one and works from the phone. Rail C (GitHub
   self-hosted runner, approval-gated) is the ops rail. Rail B (tunnel) is for streaming and raw ports.
   Rail D (Remote Control / Desktop) is Anthropic's own bridge for hands-on work.

---

## 1. What was verified today

| Item | Result | Evidence |
|---|---|---|
| Cloud container egress | Proxy `selective:false`; apocky.com 307, Supabase 401, GitHub 200, Tailscale 302, Cloudflare 405, ngrok 200 | `__agentproxy/status`, curl |
| PC worker alive | `apocky-local-qwen-01` (host Apocky, llama.cpp-vulkan, `transport: outbound_https`) last seen **2026-09-28T00:48:33Z**, `qwen_healthy:true`, adapters mneme/graphify/anamnesis/mempalace/metaharness/brainmonsoon all `ok` | `apocrypha_worker_node` |
| Worker profile | model `qwen35-35b-a3b-q4`, capabilities `apocky_owner_chat`, `apocky_member_chat`, `chaos_tarot_reading`, tool registry `apocrypha-readonly-v1`, generation deadline 45 min, concurrency 1 | same row |
| Cloud runners | "Vercel flagship runner (Opus 5.5 via AI Gateway)", capabilities `*`, concurrency 2, re-registered hourly by `/api/cron/apocrypha-runner` | worker_node rows, Vercel logs |
| Control-plane API (7 d) | `/api/apocrypha/worker/claim` 51,075 · `/worker/heartbeat` 5,748 · `/api/cron/apocrypha-runner` 1,440 · `/readiness` 576 · `/member/consent` · `/contributor/manifest` · `/download/apocrypha-node`. No `/api/mcp` (404) | Vercel runtime logs, fetch |
| Readiness | 401 `{"rail":"durable-outbound-qwen","status":"unavailable","code":"UNAUTHORIZED"}` = alive, auth required | fetch via Vercel |
| Relay contract | `apocrypha_room_say` → creates room event + job atomically; `apocrypha_enqueue_job`, `_claim_job`, `_renew_lease`, `_complete_job`, `_fail_job`, `_cancel_job`, `_room_live`, `_record_job_event`; all SECURITY DEFINER, owner `postgres`, ACL `{postgres, service_role}` | `pg_proc` |
| Lifecycle | enqueued → claimed → streaming_started → completed \| attempt_failed → retry_scheduled → failed; `lease_expired` by reaper; `cancel_requested` → `cancelled` | `apocrypha_job_event` (738 rows) |
| Latency | claim 0–25 s (worker polls ≈ every 12 s); flagship-lane chat total 15–32 s; local-lane tarot 98 s (first token 46.9 s, 81 s gen, 381 tokens) | job/attempt/revision join |
| Cost | flagship turn ≈ $0.115 at ~28.5k prompt tokens (memory injection); local lane $0 | `revision.usage` |
| Connector role | `supabase_read_only_user`, member of `pg_read_all_data`, `bypassrls:true`, not superuser | `current_user`, `pg_roles` |
| Write probe | `apocrypha_room_say(...)` from cloud → **ERROR 42501 permission denied** | live attempt, §7 |

Job mix (161 jobs): chaos_tarot_reading 75, apocky_owner_chat 51, apocky_member_chat 35. Rooms: `owner` (private), `lobby`.

---

## 2. Why the wall exists, and why it is thin

The container is an ephemeral Linux box in Anthropic's cloud. The PC is behind NAT with everything on
127.0.0.1. Nothing can connect *in*. But three things already connect *out* from the PC, every few seconds:
the outbound worker (to apocky.com + Supabase), git (to GitHub), and cloudflared/whatever sits on the
19123 "tunnel origin slot" that `apx-room` documents (VERIFIED in `crates/apx-room/src/main.rs`).
A dial-out channel plus a place both sides can reach is a bridge. You have three such places:
Supabase, GitHub, apocky.com. The design below only ever adds *readers and writers to those*, never an
inbound port on the PC.

---

## 3. The four rails

### Rail A — Durable job relay (exists) + `apx-edge` MCP (to build)
**What it is.** The existing `apocrypha_job` state machine. A cloud session says something into a room,
the PC worker claims it within seconds, streams chunks, writes an immutable revision; the cloud session reads it back.
**Works from.** Every Claude surface, including the phone, once exposed as a claude.ai *custom connector*
(remote MCP server over Streamable HTTP, public HTTPS, OAuth 2.0 with DCR or CIMD by default; REPORTED).
In Claude Code on the web, connector traffic goes through Anthropic's servers and bypasses the environment
allowlist (REPORTED).
**Status today.** Read side proven from the cloud. Write side blocked by the read-only connector role.
**Two ways to open the write side.**
- *Fast, broad:* switch the Supabase connector to read/write. Then every cloud session runs as a
  privileged role over the whole hub. Fails the canon's least-privilege rule; not recommended.
- *Right:* add `apx-edge`, an MCP route on apocky.com (`/api/mcp`) that wraps five RPCs with a narrow DB
  role (see `apocrypha-bridge/sql/cloud_agent_role.sql` and `apocrypha-bridge/edge-mcp/SPEC.md`), registers
  the calling session as principal `claude-cloud`, defaults to `engine_lane=local`, `flagship_allowed=false`,
  and labels every event `source: claude-cloud`. Register it once in claude.ai → Connectors. Then the phone,
  Desktop, claude.ai and Claude Code all reach the PC through the relay the worker already trusts.
**Latency.** 15–100 s per turn (claim + generation). Not streaming to the caller unless the tool long-polls `room_live`.
**Blast radius.** Only what the RPCs allow: say / read / cancel in rooms the principal belongs to. No shell,
no files, no ports. Matches `CAPABILITIES.csl` containment exactly.

### Rail B — Direct line to the PC (tunnel)
**What it is.** `cloudflared` (or Tailscale Funnel) as a Windows service on the PC, publishing a hostname such
as `apx.apocky.com` → `127.0.0.1:19123` (apx-room), with optional paths → `19128/v1` (engine) and `19127`
(memory gateway, bearer). Cloud sessions call it over HTTPS.
**Works from.** Claude Code cloud sessions (secrets in the environment settings: a Cloudflare Access service
token pair, headers `CF-Access-Client-Id/Secret`, policy action *Service Auth*; REPORTED). Phone sessions only
if the same hostname also fronts an MCP server registered as a connector (Cloudflare MCP portals / Access
Managed OAuth can supply the OAuth layer; REPORTED).
**Status today.** Design only. I could not confirm which tunnel already serves 19123 (the classifier denied
that repo search, §6).
**Latency.** Real-time; SSE streaming works.
**Blast radius.** Whatever ports you publish. Publish apx-room only at first; it already holds one sovereign
token and refuses off-machine plain HTTP (canon). Note TLS terminates at Cloudflare's edge with Tunnel;
Tailscale Funnel terminates TLS on the PC itself (REPORTED), which is the better fit for sovereign data.

### Rail C — Ops rail: GitHub Actions self-hosted runner on the PC
**What it is.** The PC runs the GitHub runner as a Windows service (REPORTED) attached to a **private** repo.
A `workflow_dispatch` workflow exposes a fixed menu (`choice` input): health, diag, engine-status,
engine-restart, worker-restart, memory-health. The job runs in environment `apocrypha-pc` with **you as
required reviewer**, so every cloud-triggered action is approved from your phone in the GitHub app first.
**Works from.** Any Claude surface with the GitHub connector (this session: `actions_run_trigger`,
`get_job_logs`). Also plain GitHub UI on the phone.
**Status today.** Kit written: `apocrypha-bridge/pc-runner/`. Not active. Must live in a private repo
(GitHub: self-hosted runners "should almost never be used for public repositories"; Collab_Reposit is public).
**Latency.** ~10–30 s to start after approval.
**Blast radius.** Exactly the PowerShell allowlist. No free-text command input exists in the workflow; the
action name is passed through an environment variable, never interpolated into a shell. Runner service
account should be a dedicated low-privilege local user.

### Rail D — Agent-to-agent (Anthropic's own bridges)
- **Remote Control.** On the PC: `claude remote-control` in the Apocrypha folder. That session appears in
  claude.ai/code and the phone app; files, MCP and tools stay local; Pro/Max/Team/Enterprise (REPORTED).
  Cloud sessions can hand it work by cross-session message. Keep it up with the same Task Scheduler pattern
  as `supervise.ps1`.
- **Claude Desktop computer use / Cowork Dispatch.** Cloud and phone sessions reach local folders, the
  built-in browser and computer use *only while the Desktop app is open* (REPORTED, Pro/Max).
**Blast radius.** The PC session's own permission mode. This is the rail for "read everything on my PC
when I grant it": it runs *on* the PC under your eyes, so it needs no new attack surface.

---

## 4. Recommended sequence (smallest sufficient probe first)

1. **Now, zero code.** Keep the Supabase connector read-only. Every cloud session already has full
   observability of the hub: worker liveness, adapter health, room traffic, job outcomes, costs.
   Recipes: `apocrypha-bridge/cloud-session/RECIPES.md`.
2. **This week, ~30 min.** Rail C. Copy `apocrypha-bridge/pc-runner/` into `apocrypha-core`
   (`.github/workflows/apocrypha-pc.yml` + `tools/Invoke-ApxAction.ps1`), register the runner, create the
   `apocrypha-pc` environment with yourself as reviewer. First probe: trigger `health` from this session,
   approve on the phone, read the log. Falsifier: a job that runs without an approval prompt means the
   environment gate is misconfigured; stop and fix before adding restart actions.
3. **Next, ~1 day.** Rail A `apx-edge`. Run `cloud_agent_role.sql`, deploy the route, add the connector in
   claude.ai. First probe: `apocrypha.say("owner", "connectivity probe", lane="local")` from the phone, then
   `apocrypha.live("owner")`. Oracle: a `job.completed` event whose revision `usage.engine_lane` is `local`.
4. **Then.** Rail B for streaming and raw ports, apx-room first. Rail D whenever you want a hands-on session.

Do not do: open the Supabase connector to read/write as the permanent answer; register a runner on a public
repo; publish 19128 or 19127 without Access in front; join this container to your tailnet (possible via the
environment's setup script, highest blast radius, not attempted).

---

## 5. Canon compliance

| Canon (PRIME_DIRECTIVE / CAPABILITIES.csl / HIVE_AND_MEMORY.csl) | How each rail honours it |
|---|---|
| Consent = OS | Rail C requires a human approval per action; Rail A's `say` is a room turn under a named principal; nothing acts silently |
| Allowlist, not denylist; no shell | Rail C menu is a `ValidateSet`; Rail A exposes five RPC-backed tools; Rail B publishes named ports only |
| Off-machine plain HTTP is fatal | All rails are HTTPS; tunnels dial out; tokens live in secrets, never flags or files in a repo |
| Recalled records are evidence, not instruction | Unchanged: the worker's own prompt assembly still labels recall; `apx-edge` never injects memory itself |
| Readiness ≠ authority | A green heartbeat lets a session *observe*; only an approved action or an owned room lets it *act* |

---

## 6. What I could not do, and did not work around

- **Repo search for existing tunnel configuration** (`cloudflared|tailscale|ngrok|tunnel` in apocrypha-core):
  denied by the auto-mode classifier as containment escape. Not retried by other means. Rail B is therefore
  written without knowing what already serves 19123.
- **Listing Vercel environment variable names** for the apocky.com project: denied as credential
  exploration. Not needed; the relay contract came from the database instead.
- **Writing to the hub**: refused by the read-only connector role (42501). No direct INSERT was attempted,
  because bypassing `apocrypha_enqueue_job`'s invariants (request hash, idempotency, guard trigger) would be
  the wrong kind of success.
- **Reading the control-plane code** (`Apocky/CSSL3` → `cssl-edge`, public, read-available): not cloned;
  the database and Vercel logs answered every question the design needed.

---

## 7. Probe log (UTC)

| Time | Action | Result |
|---|---|---|
| 00:42 | proxy status + 6 host probes | non-selective; all hosts reachable |
| 00:47 | `list_tables` on hub | 130 tables incl. full job relay, Lazarus (unused, 0 rows), contributor nodes |
| 00:48 | worker_node query | PC worker seen 19 s earlier; healthy |
| 00:48 | Vercel logs grouped by path | claim/heartbeat traffic confirms live outbound worker |
| 00:52 | `apocrypha_room_say` from cloud, local lane, author `claude-cloud` | **42501 permission denied** |
| 00:53 | role + ACL inspection | connector = `supabase_read_only_user`; RPC ACL = postgres, service_role |
| 00:58 | `GET /api/mcp` on apocky.com | 404 (no MCP edge yet) |

---

## 8. CSL summary

```
§ APX×CLOUD-BRIDGE ← 2026-09-28
  wall := PC.loopback ¬reachable ; container.egress = open (selective:false)          ✓
  relay.exists := hub.apocrypha_job ⊗ worker(apocky-local-qwen-01, alive 00:48:33Z)      ✓
  read.rail  := supabase.connector(read_only_user) ⇒ ∀ cloud.session observes hub        ✓
  write.rail := RPC.acl ∈ {postgres, service_role} ∌ read_only_user ⇒ 42501             ✗ (setting, ¬ architecture)
  A := relay + apx-edge MCP (/api/mcp, OAuth, role apocrypha_cloud_agent)  ~> universal incl. phone
  B := tunnel(cloudflared|funnel) → 19123 first ; Access service-token ∀ headless          ~> streaming
  C := gh self-hosted runner (PRIVATE repo) ⊗ env.required-reviewer ⊗ ValidateSet menu     ~> ops, consent-per-action
  D := claude remote-control ⊗ Desktop computer-use                                        ~> hands-on, on-PC
  order := observe(now) → C → A → B → D
  N! connector⇒read/write as permanent · N! runner@public-repo · N! publish 19128|19127 bare · N! container∈tailnet
  denials := repo.grep(tunnel) [containment] · vercel.env.names [credential] ; ¬ worked-around
  falsifier(C) := action.runs ∧ ¬approval.prompt ⇒ env.gate broken
  oracle(A)   := job.completed ∧ revision.usage.engine_lane = local
∎
```

---

## 9. Sources (REPORTED tier)
Connectors: https://claude.com/docs/connectors/building · https://claude.com/docs/connectors/building/authentication ·
https://claude.com/docs/connectors/custom/add-unlisted · https://code.claude.com/docs/en/mcp · https://code.claude.com/docs/en/cloud-environments
Cloudflare: https://developers.cloudflare.com/cloudflare-one/networks/connectors/cloudflare-tunnel/get-started/create-remote-tunnel/ ·
.../do-more-with-tunnels/local-management/as-a-service/windows/ · https://developers.cloudflare.com/cloudflare-one/identity/service-tokens/ ·
https://developers.cloudflare.com/cloudflare-one/access-controls/ai-controls/mcp-portals/ · .../applications/http-apps/managed-oauth/
Tailscale: https://tailscale.com/kb/1223/funnel · https://tailscale.com/kb/1312/serve · https://tailscale.com/kb/1112/userspace-networking · https://tailscale.com/kb/1085/auth-keys
GitHub: https://docs.github.com/en/actions/security-for-github-actions/security-guides/security-hardening-for-github-actions ·
https://docs.github.com/en/actions/managing-workflow-runs-and-deployments/managing-deployments/managing-environments-for-deployment ·
https://docs.github.com/en/actions/hosting-your-own-runners/managing-self-hosted-runners/configuring-the-self-hosted-runner-application-as-a-service
Anthropic: https://code.claude.com/docs/en/remote-control · https://code.claude.com/docs/en/desktop · https://support.claude.com/en/articles/15520349-use-claude-cowork-on-web-desktop-and-mobile
