# Rail C install — GitHub self-hosted runner on the Apocrypha PC

Facts marked REPORTED come from GitHub docs fetched 2026-09-28 (links in the main document §9).

## 0. Preconditions
- Use a **private** repository (`Apocky/apocrypha-core`). GitHub: self-hosted runners should almost never be
  used for public repositories (REPORTED). Collab_Reposit is public.
- Environments with required reviewers on a private repo need GitHub Pro or Team (REPORTED). Check your plan.

## 1. Files
Copy `apocrypha-pc.yml` → `.github/workflows/apocrypha-pc.yml` and `Invoke-ApxAction.ps1` → `tools/Invoke-ApxAction.ps1`
in the private repo, commit to the default branch (workflow_dispatch needs the file on the default branch; REPORTED).

## 2. Approval gate (do this BEFORE registering the runner)
Repo → Settings → Environments → New: `apocrypha-pc`. Protection rules: **Required reviewers: you**; optionally
"Prevent self-review" off (you are the only human), wait timer 0, deployment branches: default branch only.
Every run of the workflow now pauses for your approval; the GitHub mobile app sends the prompt to your phone.

## 3. Runner as a Windows service
Repo → Settings → Actions → Runners → New self-hosted runner → Windows. Follow the shown download and
`config.cmd` steps in an **admin PowerShell**, folder `C:\actions-runner`. When asked:
- labels: add `apocrypha-pc`
- run as service: **yes** (manage with `Get-Service "actions.runner.*"`; REPORTED)
- service account: a dedicated local user with no admin rights and no access to `D:\Apocrypha\*` secrets;
  give it read access to the apocrypha-core checkout and the right to start/stop the two scheduled tasks
  (Task Scheduler → task Properties → Security, or run those two tasks under the same account).
Set `APX_CORE_ROOT` for the service (System → Environment variables, then restart the service).

## 4. First probe (from any Claude cloud session with the GitHub connector)
1. Trigger: `actions_run_trigger(run_workflow, owner=Apocky, repo=apocrypha-core, workflow_id=apocrypha-pc.yml, ref=<default>, inputs={action: health})`.
2. Approve on the phone.
3. Read: `get_job_logs` for the run. Expected: five probe lines; 19127 answering 401 is healthy.
Falsifier: if the job runs without an approval prompt, the environment is not attached to the job; stop,
fix `environment: apocrypha-pc` in the workflow, and re-test before enabling the restart actions.

## 5. Hygiene
- Rotate the runner registration if the PC is rebuilt; remove the runner from the repo when not needed for weeks.
- Keep the menu small. Every new action is a new capability; add it as a named case with its own probe.
- Never add a `string` input that reaches `pwsh`/`cmd`. The `note` input is echoed to the log only.
