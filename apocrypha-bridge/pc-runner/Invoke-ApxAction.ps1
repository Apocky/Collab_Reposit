<#
.SYNOPSIS
  Rail C action menu. The ONLY things a cloud session can make the PC do, each by name.
  No parameter accepts free text that reaches a process. Add an action by adding a case, not a flag.
.NOTES
  Requires APX_CORE_ROOT (path of the apocrypha-core checkout) for 'diag'. Task names from DIAGNOSTICS_RUNBOOK.md.
  Ports: 19123 room · 19126 worker health · 19127 memory gateway · 19128 engine/hive · 19129 federator · 19130/19131 work.
#>
[CmdletBinding()]
param(
  [Parameter(Mandatory)]
  [ValidateSet('health','diag','engine-status','engine-restart','worker-restart','memory-health')]
  [string]$Action
)
$ErrorActionPreference = 'Stop'
$EngineTask = 'Apocrypha-Qwen35-Vulkan'
$WorkerTask = 'Apocrypha Outbound Worker'

function Probe([string]$Url) {
  try {
    $r = Invoke-WebRequest -Uri $Url -UseBasicParsing -TimeoutSec 8
    "{0,-36} {1}  {2}" -f $Url, $r.StatusCode, ($r.Content | Select-Object -First 1)
  } catch {
    $code = $_.Exception.Response.StatusCode.value__
    if ($code) { "{0,-36} {1}  (gated = alive)" -f $Url, $code } else { "{0,-36} DOWN  {1}" -f $Url, $_.Exception.Message }
  }
}

switch ($Action) {
  'health' {
    Probe 'http://127.0.0.1:19128/health'
    Probe 'http://127.0.0.1:19126/health'
    Probe 'http://127.0.0.1:19127/health'   # 401 without a token is the healthy answer (runbook §probes)
    Probe 'http://127.0.0.1:19123/'
    Probe 'http://127.0.0.1:19129/health'
  }
  'diag' {
    if (-not $env:APX_CORE_ROOT) { throw 'APX_CORE_ROOT is not set on the runner; set it in the service environment.' }
    & python (Join-Path $env:APX_CORE_ROOT 'tools\apx-diag.py')
  }
  'engine-status' {
    Get-ScheduledTask -TaskName $EngineTask | Get-ScheduledTaskInfo | Format-List TaskName, LastRunTime, LastTaskResult, NextRunTime
    Get-NetTCPConnection -LocalPort 19128 -State Listen -ErrorAction SilentlyContinue |
      Select-Object LocalPort, OwningProcess, @{n='Process';e={(Get-Process -Id $_.OwningProcess).ProcessName}}
  }
  'engine-restart' {
    Write-Host "Stopping $EngineTask"; Stop-ScheduledTask -TaskName $EngineTask -ErrorAction SilentlyContinue
    Start-Sleep -Seconds 5
    Write-Host "Starting $EngineTask"; Start-ScheduledTask -TaskName $EngineTask
    Start-Sleep -Seconds 20
    Probe 'http://127.0.0.1:19128/health'
  }
  'worker-restart' {
    Write-Host "Stopping $WorkerTask"; Stop-ScheduledTask -TaskName $WorkerTask -ErrorAction SilentlyContinue
    Start-Sleep -Seconds 3
    Write-Host "Starting $WorkerTask"; Start-ScheduledTask -TaskName $WorkerTask
    Start-Sleep -Seconds 15
    Probe 'http://127.0.0.1:19126/health'
  }
  'memory-health' {
    Probe 'http://127.0.0.1:19127/health'
    Probe 'http://127.0.0.1:19129/health'
  }
}
