# apocrypha-bridge — kit for cloud → PC connectivity

Companion to `../APOCRYPHA_CLOUD_TO_PC_BRIDGE.md`. Nothing in this folder is active on its own.

| Path | Rail | What it is | Where it belongs |
|---|---|---|---|
| `cloud-session/RECIPES.md` | A (read) | SQL any Claude cloud session can run today through the read-only Supabase connector | use as-is |
| `sql/cloud_agent_role.sql` | A (write) | least-privilege Postgres role for an `apx-edge` gateway | run once on the hub, by you |
| `edge-mcp/SPEC.md` | A (write) | tool contract for the `apx-edge` MCP server on apocky.com | implement in the control-plane repo (cssl-edge) |
| `pc-runner/apocrypha-pc.yml` | C | approval-gated `workflow_dispatch` workflow with a fixed action menu | **private repo only** → `.github/workflows/` |
| `pc-runner/Invoke-ApxAction.ps1` | C | the allowlisted PowerShell action menu the runner executes | same repo, e.g. `tools/` |
| `pc-runner/INSTALL.md` | C | runner-as-service setup, environment reviewer gate, first probe | read once |

**Do not** register a self-hosted runner against this repository: it is public.
