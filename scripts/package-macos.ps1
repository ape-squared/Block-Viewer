<#
.SYNOPSIS
    Builds "Block Viewer.app" from the built viewer: the executable in
    Contents/MacOS, the files of package.ps1 (licences, notices, install
    metadata) with the icon and the translated texts of Info.plist in
    Contents/Resources. The macOS job of the release runs it (PowerShell 7);
    signing and notarization come after, in the workflow.
.EXAMPLE
    ./scripts/package-macos.ps1 -Exe target/release/block-viewer -Out "Block Viewer.app"
#>
param(
    [Parameter(Mandatory)][string]$Exe,
    [Parameter(Mandatory)][string]$Out,
    # repository whose files go in the bundle (default: this script's)
    [string]$Source = (Split-Path $PSScriptRoot -Parent)
)
$ErrorActionPreference = 'Stop'

if (Test-Path -LiteralPath $Out) { Remove-Item -LiteralPath $Out -Recurse -Force }
$contents = Join-Path $Out 'Contents'
$macos = Join-Path $contents 'MacOS'
$resources = Join-Path $contents 'Resources'
New-Item -ItemType Directory -Force -Path $macos, $resources | Out-Null

# the files every package carries, then the executable where macOS runs it
# (code only in Contents/MacOS: the signature seals the rest as resources)
& (Join-Path $PSScriptRoot 'package.ps1') -Exe $Exe -Out $resources -Source $Source
Move-Item -LiteralPath (Join-Path $resources (Split-Path $Exe -Leaf)) -Destination (Join-Path $macos 'block-viewer')

# MAJOR.MINOR.PATCH of the build, the only form CFBundleShortVersionString
# takes (a beta's pre-release part and the commits since the tag are left out)
$version = (Get-Content -LiteralPath (Join-Path $resources 'block-install.json') -Raw | ConvertFrom-Json).version
if ($version -notmatch '^(\d+\.\d+\.\d+)') { throw "Invalid package version: $version" }
$plist = Get-Content -LiteralPath (Join-Path $Source 'packaging/macos/Info.plist') -Raw
Set-Content -LiteralPath (Join-Path $contents 'Info.plist') -Value $plist.Replace('@VERSION@', $Matches[1]) -Encoding utf8NoBOM -NoNewline
Set-Content -LiteralPath (Join-Path $contents 'PkgInfo') -Value 'APPL????' -Encoding ascii -NoNewline

# App icon: the Block mark in assets/branding/block/02-app-icon (not the
# legacy aurora.icns copy that lived at assets/branding/block.icns). Prefer
# iconutil on the Mac runner; fall back to a committed block.icns built from
# the same PNGs. CFBundleIconFile is "Block".
$icnsDest = Join-Path $resources 'Block.icns'
$iconDir = Join-Path $Source 'assets/branding/block/02-app-icon'
$iconutil = Get-Command iconutil -ErrorAction SilentlyContinue
if ($iconutil -and (Test-Path -LiteralPath $iconDir)) {
    $iconset = Join-Path ([System.IO.Path]::GetTempPath()) ("block-viewer-{0}.iconset" -f [guid]::NewGuid().ToString('N'))
    New-Item -ItemType Directory -Force -Path $iconset | Out-Null
    $pairs = @{
        'icon_16x16.png'        = 'block-app-icon-16.png'
        'diana.k@example.org'     = 'block-app-icon-32.png'
        'icon_32x32.png'        = 'block-app-icon-32.png'
        'ivan.p@example.net'     = 'block-app-icon-64.png'
        'icon_128x128.png'      = 'block-app-icon-128.png'
        'wendy.h@example.net'   = 'block-app-icon-256.png'
        'icon_256x256.png'      = 'block-app-icon-256.png'
        'wendy.h@example.net'   = 'block-app-icon-512.png'
        'icon_512x512.png'      = 'block-app-icon-512.png'
        'walt.e@example.net'   = 'block-app-icon-1024.png'
    }
    foreach ($name in $pairs.Keys) {
        Copy-Item -LiteralPath (Join-Path $iconDir $pairs[$name]) -Destination (Join-Path $iconset $name)
    }
    & iconutil -c icns $iconset -o $icnsDest
    if ($LASTEXITCODE -ne 0) { throw "iconutil failed ($LASTEXITCODE)" }
    Remove-Item -LiteralPath $iconset -Recurse -Force
} else {
    $icnsSrc = Join-Path $Source 'assets/branding/block.icns'
    if (-not (Test-Path -LiteralPath $icnsSrc)) {
        throw "Missing Block.icns source (no iconutil and no assets/branding/block.icns)"
    }
    Copy-Item -LiteralPath $icnsSrc -Destination $icnsDest
}
Get-ChildItem -LiteralPath (Join-Path $Source 'packaging/macos') -Directory -Filter '*.lproj' | ForEach-Object {
    Copy-Item -LiteralPath $_.FullName -Destination $resources -Recurse
}
"${Out}: Block Viewer $version"
