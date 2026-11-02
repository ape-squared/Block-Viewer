#Requires -Version 5.1
<#
.SYNOPSIS
    Removes this Block Viewer portable install (and optionally user data).

    Double-click Uninstall.cmd next to block-viewer.exe, or run:
      powershell -NoProfile -ExecutionPolicy Bypass -File Uninstall.ps1
#>
[CmdletBinding()]
param(
    # Skip the interactive prompts (still needs -RemoveUserData to wipe profile/cache).
    [switch]$Yes,
    # Also delete %APPDATA%\ape-squared\BlockViewer and %LOCALAPPDATA%\ape-squared\BlockViewer.
    [switch]$RemoveUserData
)

$ErrorActionPreference = 'Stop'
$InstallDir = if ($PSScriptRoot) {
    (Resolve-Path -LiteralPath $PSScriptRoot).Path
} else {
    (Resolve-Path -LiteralPath (Split-Path -Parent $MyInvocation.MyCommand.Path)).Path
}

function Test-BlockInstall([string]$Dir) {
    $exe = @(
        (Join-Path $Dir 'block-viewer.exe'),
        (Join-Path $Dir 'BlockViewer.exe')
    ) | Where-Object { Test-Path -LiteralPath $_ } | Select-Object -First 1
    $meta = Join-Path $Dir 'block-install.json'
    return ($null -ne $exe) -and (Test-Path -LiteralPath $meta)
}

function Stop-BlockViewerHere([string]$Dir) {
    $targets = Get-Process -Name 'block-viewer','BlockViewer' -ErrorAction SilentlyContinue | Where-Object {
        try {
            $path = $_.Path
            $path -and ($path.StartsWith($Dir, [StringComparison]::OrdinalIgnoreCase))
        } catch {
            $false
        }
    }
    foreach ($p in $targets) {
        Write-Host "Stopping $($p.ProcessName) (PID $($p.Id))…"
        Stop-Process -Id $p.Id -Force -ErrorAction SilentlyContinue
    }
    Start-Sleep -Milliseconds 400
}

function Clear-SchemeIfOurs([string]$ExePath) {
    $schemes = @('secondlife', 'hop', 'x-grid-location-info')
    foreach ($scheme in $schemes) {
        $key = "HKCU:\Software\Classes\$scheme\shell\open\command"
        if (-not (Test-Path -LiteralPath $key)) { continue }
        $cmd = (Get-ItemProperty -LiteralPath $key -Name '(default)' -ErrorAction SilentlyContinue).'(default)'
        if (-not $cmd) { continue }
        if ($cmd -like "*$ExePath*") {
            Remove-Item -LiteralPath "HKCU:\Software\Classes\$scheme" -Recurse -Force -ErrorAction SilentlyContinue
            Write-Host "Unregistered $scheme: links for this install."
        }
    }
}

function Remove-UserTrees {
    $roots = @(
        (Join-Path $env:APPDATA 'ape-squared\BlockViewer'),
        (Join-Path $env:LOCALAPPDATA 'ape-squared\BlockViewer'),
        # Older Block / Aurora layout still mentioned in docs.
        (Join-Path $env:LOCALAPPDATA 'Aurora\BlockViewer'),
        (Join-Path $env:APPDATA 'Aurora\BlockViewer')
    ) | Select-Object -Unique
    foreach ($root in $roots) {
        if (Test-Path -LiteralPath $root) {
            Write-Host "Removing user data: $root"
            Remove-Item -LiteralPath $root -Recurse -Force -ErrorAction Continue
        }
    }
}

if (-not (Test-BlockInstall $InstallDir)) {
    Write-Error "Not a Block Viewer package folder: $InstallDir"
    exit 1
}

$Exe = @(
    (Join-Path $InstallDir 'block-viewer.exe'),
    (Join-Path $InstallDir 'BlockViewer.exe')
) | Where-Object { Test-Path -LiteralPath $_ } | Select-Object -First 1

Write-Host ""
Write-Host "Block Viewer uninstall"
Write-Host "  Install folder: $InstallDir"
Write-Host ""

if (-not $Yes) {
    $answer = Read-Host "Remove this installation? [y/N]"
    if ($answer -notmatch '^[Yy]') {
        Write-Host "Cancelled."
        exit 0
    }
    if (-not $RemoveUserData) {
        $data = Read-Host "Also delete settings, cache and logs under AppData? [y/N]"
        if ($data -match '^[Yy]') { $RemoveUserData = $true }
    }
}

Stop-BlockViewerHere $InstallDir
Clear-SchemeIfOurs $Exe

if ($RemoveUserData) {
    Remove-UserTrees
}

# Delete the install folder after this process exits (we live inside it).
$parent = Split-Path -Parent $InstallDir
$name = Split-Path -Leaf $InstallDir
$escaped = $InstallDir.Replace("'", "''")
$cleanup = @"
Start-Sleep -Seconds 1
if (Test-Path -LiteralPath '$escaped') {
  Remove-Item -LiteralPath '$escaped' -Recurse -Force -ErrorAction SilentlyContinue
}
"@
$temp = Join-Path $env:TEMP ("block-viewer-uninstall-{0}.ps1" -f [guid]::NewGuid().ToString('N'))
Set-Content -LiteralPath $temp -Value $cleanup -Encoding ASCII
Write-Host "Removing $name from $parent…"
Start-Process -FilePath 'powershell.exe' -ArgumentList @(
    '-NoProfile', '-ExecutionPolicy', 'Bypass', '-WindowStyle', 'Hidden', '-File', $temp
) | Out-Null
Write-Host "Uninstall started. The folder closes in a moment."
exit 0
