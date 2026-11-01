<#
.SYNOPSIS
  Production executable: optimized Release + portable Block-Viewer folder.
  Main grid defaults (Agni). No AURORA_DEMO.

.EXAMPLE
  ./scripts/build-block-prod.ps1
#>
$ErrorActionPreference = 'Stop'
$Root = Split-Path $PSScriptRoot -Parent
Set-Location $Root
$env:CARGO_TARGET_DIR = Join-Path $Root 'target'

Remove-Item Env:AURORA_DEMO -ErrorAction SilentlyContinue
Write-Host 'Building Block Viewer PRODUCTION (release, LTO)...' -ForegroundColor Cyan
cargo build -p block-viewer --release
if ($LASTEXITCODE -ne 0) { throw 'cargo build --release failed' }

$exe = Join-Path $Root 'target\release\block-viewer.exe'
$out = Join-Path $Root 'dist\Block-Viewer'
if (Test-Path $out) { Remove-Item $out -Recurse -Force }
& (Join-Path $Root 'scripts\package.ps1') -Exe $exe -Out $out

$shipped = Join-Path $out 'block-viewer.exe'
$named = Join-Path $out 'BlockViewer.exe'
if (Test-Path $shipped) { Move-Item -LiteralPath $shipped -Destination $named -Force }

$bat = @"
@echo off
cd /d "%~dp0"
start "" "%~dp0BlockViewer.exe" %*
"@
Set-Content -LiteralPath (Join-Path $out 'BlockViewer.bat') -Value $bat -Encoding ASCII
Write-Host "Production package: $out" -ForegroundColor Green
Write-Host "Launch: $out\BlockViewer.bat" -ForegroundColor Green
