-- Least-privilege database role for a cloud-agent gateway (Rail A, apx-edge).
-- Run once as postgres on the hub. Review every line first; nothing here is executed by an agent.
-- The gateway's login user is created out of band (password never enters a repo):
--   create user apx_edge login password '<set in Supabase dashboard / vault>';
--   grant apocrypha_cloud_agent to apx_edge;

begin;

create role apocrypha_cloud_agent nologin;
grant usage on schema public to apocrypha_cloud_agent;

-- Write path: exactly the RPCs a conversation needs. All are SECURITY DEFINER (run as postgres),
-- so the role never touches tables directly for writes.
grant execute on function public.apocrypha_room_say(text, text, text, uuid, uuid, text, jsonb, text, boolean, text, text, text, text)
  to apocrypha_cloud_agent;
grant execute on function public.apocrypha_room_live(text)
  to apocrypha_cloud_agent;
grant execute on function public.apocrypha_cancel_job(uuid, uuid, text)
  to apocrypha_cloud_agent;

-- Read path: column-level, no token hashes, no ciphertext.
grant select (id, node_key, display_name, status, allowed_capabilities, max_concurrency,
              model_profiles, metadata, last_seen_at, revoked_at)
  on public.apocrypha_worker_node to apocrypha_cloud_agent;
grant select (id, tenant_id, owner_principal_id, kind, capability, job_role, status, model_alias,
              priority, attempt_count, available_at, cancel_requested_at, completed_at,
              error_code, error_detail, created_at, updated_at)
  on public.apocrypha_job to apocrypha_cloud_agent;
grant select (id, job_id, attempt_id, revision_no, revision_role, content, content_hash,
              model_alias, usage, provenance, created_at)
  on public.apocrypha_job_revision to apocrypha_cloud_agent;
grant select (id, job_id, attempt_id, ordinal, event_type, outcome, severity, source, occurred_at)
  on public.apocrypha_job_event to apocrypha_cloud_agent;
grant select on public.apocrypha_room_events to apocrypha_cloud_agent;

-- Deliberately NOT granted: apocrypha_enqueue_job (bypasses room binding), apocrypha_issue_worker_token,
-- apocrypha_claim_job / complete / fail (worker-only), anything under lazarus_*, mem_*, mneme_*, stripe_*.
-- Row-level security still applies to the direct SELECTs above; if a policy hides rows from this role,
-- prefer adding a narrow policy over granting BYPASSRLS.

commit;
