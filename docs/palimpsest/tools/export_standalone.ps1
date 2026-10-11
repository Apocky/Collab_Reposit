<#
.SYNOPSIS
  Create or refresh a standalone PALIMPSEST repository on a Windows machine.

.DESCRIPTION
  Clones (or pulls) the design branch of Apocky/Collab_Reposit into a source
  folder, then mirrors the Rust workspace and the design docs into a separate,
  standalone git repository and commits any changes. Safe to re-run.

  Layout of the standalone repository:
    <Target>\            the Cargo workspace (Cargo.toml, crates\, tools\, ...)
    <Target>\docs\       DESIGN_BRIEF.md, DECISIONS.md, spec\, research\, tools\

.EXAMPLE
  powershell -ExecutionPolicy Bypass -File export_standalone.ps1
  powershell -ExecutionPolicy Bypass -File export_standalone.ps1 -Root C:\Users\Apocky\source\repos -Name PALIMPSEST
#>
param(
  [string]$Root   = "C:\Users\Apocky\source\repos",
  [string]$Name   = "PALIMPSEST",
  [string]$Branch = "claude/3d-omni-genre-roguelike-se65qx",
  [string]$Remote = "https://github.com/Apocky/Collab_Reposit.git"
)

$ErrorActionPreference = "Stop"
$src    = Join-Path $Root "$Name-src"
$target = Join-Path $Root $Name

if (-not (Test-Path $Root)) { New-Item -ItemType Directory -Path $Root | Out-Null }

# 1. Clone or update the source branch.
if (Test-Path (Join-Path $src ".git")) {
  git -C $src fetch origin $Branch
  git -C $src checkout $Branch
  git -C $src reset --hard "origin/$Branch"
} else {
  git clone --branch $Branch --single-branch $Remote $src
}

# 2. Create the standalone repository on first run.
if (-not (Test-Path (Join-Path $target ".git"))) {
  New-Item -ItemType Directory -Path $target -Force | Out-Null
  git -C $target init -b main
}

# 3. Mirror content. robocopy exit codes below 8 mean success.
robocopy (Join-Path $src "palimpsest") $target /MIR /XD target .git docs /NFL /NDL /NJH /NJS | Out-Null
if ($LASTEXITCODE -ge 8) { throw "robocopy of the workspace failed with code $LASTEXITCODE" }
robocopy (Join-Path $src "docs\palimpsest") (Join-Path $target "docs") /MIR /NFL /NDL /NJH /NJS | Out-Null
if ($LASTEXITCODE -ge 8) { throw "robocopy of the docs failed with code $LASTEXITCODE" }
$global:LASTEXITCODE = 0

# 4. Commit if anything changed.
git -C $target add -A
$pending = git -C $target status --porcelain
if ($pending) {
  $rev = git -C $src rev-parse --short HEAD
  git -C $target commit -m "Sync PALIMPSEST from Collab_Reposit@$rev"
  Write-Host "Committed changes into $target (source $rev)."
} else {
  Write-Host "No changes; $target is up to date."
}

Write-Host "Build and test: cd $target; cargo test --workspace"
