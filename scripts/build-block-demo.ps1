<#
.SYNOPSIS
  Demo executable: Block-branded viewer with offline plaza (BLOCK_DEMO=1).
  Uses the fast local profile (Cargo default / target\debug) — ship-like, no LTO.

.EXAMPLE
  ./scripts/build-block-demo.ps1
#>
$ErrorActionPreference = 'Stop'
$Root = Split-Path $PSScriptRoot -Parent
Set-Location $Root
$env:CARGO_TARGET_DIR = Join-Path $Root 'target'

$env:BLOCK_DEMO = '1'
$env:AURORA_DEMO = '1'
Write-Host 'Building Block Viewer DEMO (offline plaza, Dev profile)...' -ForegroundColor Cyan
cargo build -p block-viewer
if ($LASTEXITCODE -ne 0) { throw 'cargo build failed' }

$exe = Join-Path $Root 'target\debug\block-viewer.exe'
$out = Join-Path $Root 'dist\Block-Viewer-Demo'
if (Test-Path $out) { Remove-Item $out -Recurse -Force }
& (Join-Path $Root 'scripts\package.ps1') -Exe $exe -Out $out

# Ship as BlockViewer.exe (crate binary name stays block-viewer for Cargo)
$shipped = Join-Path $out 'block-viewer.exe'
$named = Join-Path $out 'BlockViewer.exe'
if (Test-Path $shipped) { Move-Item -LiteralPath $shipped -Destination $named -Force }

# Launcher that keeps demo mode on
$bat = @"
@echo off
set BLOCK_DEMO=1
cd /d "%~dp0"
start "" "%~dp0BlockViewer.exe" %*
"@
Set-Content -LiteralPath (Join-Path $out 'BlockViewer-Demo.bat') -Value $bat -Encoding ASCII
Write-Host "Demo package: $out" -ForegroundColor Green
Write-Host "Launch: $out\BlockViewer-Demo.bat" -ForegroundColor Green
