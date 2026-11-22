# Sync an Aurora Viewer release tag into Block Viewer (rename aurora-* -> block-*).
# Usage: .\scripts\sync-aurora-tag.ps1 v0.9.12
# Keeps Block branding paths from the current branch (see $KeepFromOurs).

param(
    [Parameter(Mandatory = $true)]
    [string]$Tag,
    [switch]$DryRun
)

$ErrorActionPreference = 'Stop'
$Root = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
Set-Location $Root

if (-not (git rev-parse --verify "${Tag}^{commit}" 2>$null)) {
    git fetch upstream "refs/tags/${Tag}:refs/tags/${Tag}" 2>&1 | Out-Null
}
$sha = git rev-parse $Tag

$Temp = Join-Path $env:TEMP "block-sync-$Tag-$(Get-Random)"
New-Item -ItemType Directory -Path $Temp -Force | Out-Null

try {
    Write-Host "Exporting $Tag ($sha) ..."
    git archive $sha | tar -x -C $Temp

    Write-Host "Renaming aurora -> block in tree ..."
    $crateMap = @{
        'aurora-assets'       = 'block-assets'
        'aurora-audio'        = 'block-audio'
        'aurora-build-gate'   = 'block-build-gate'
        'aurora-editor-sync'  = 'block-editor-sync'
        'aurora-i18n'         = 'block-i18n'
        'aurora-llsd'         = 'block-llsd'
        'aurora-media'        = 'block-media'
        'aurora-msg'          = 'block-msg'
        'aurora-net'          = 'block-net'
        'aurora-prim'         = 'block-prim'
        'aurora-proxy'        = 'block-proxy'
        'aurora-render'       = 'block-render'
        'aurora-updater'      = 'block-updater'
        'aurora-version'      = 'block-version'
        'aurora-viewer'       = 'block-viewer'
        'aurora-voice'        = 'block-voice'
    }

    foreach ($pair in $crateMap.GetEnumerator()) {
        $from = Join-Path $Temp "crates\$($pair.Key)"
        $to = Join-Path $Temp "crates\$($pair.Value)"
        if (Test-Path $from) {
            if (Test-Path $to) { Remove-Item -Recurse -Force $to }
            Rename-Item $from $to.Name
        }
    }

  $replacements = @(
        @{ Pattern = 'aurora-viewer'; Replacement = 'block-viewer' },
        @{ Pattern = 'aurora-render'; Replacement = 'block-render' },
        @{ Pattern = 'aurora-audio'; Replacement = 'block-audio' },
        @{ Pattern = 'aurora-net'; Replacement = 'block-net' },
        @{ Pattern = 'aurora-msg'; Replacement = 'block-msg' },
        @{ Pattern = 'aurora-i18n'; Replacement = 'block-i18n' },
        @{ Pattern = 'aurora-llsd'; Replacement = 'block-llsd' },
        @{ Pattern = 'aurora-media'; Replacement = 'block-media' },
        @{ Pattern = 'aurora-prim'; Replacement = 'block-prim' },
        @{ Pattern = 'aurora-voice'; Replacement = 'block-voice' },
        @{ Pattern = 'aurora-updater'; Replacement = 'block-updater' },
        @{ Pattern = 'aurora-version'; Replacement = 'block-version' },
        @{ Pattern = 'aurora_assets'; Replacement = 'block_assets' },
        @{ Pattern = 'aurora_audio'; Replacement = 'block_audio' },
        @{ Pattern = 'aurora_render'; Replacement = 'block_render' },
        @{ Pattern = 'aurora_net'; Replacement = 'block_net' },
        @{ Pattern = 'aurora_msg'; Replacement = 'block_msg' },
        @{ Pattern = 'aurora_i18n'; Replacement = 'block_i18n' },
        @{ Pattern = 'aurora_llsd'; Replacement = 'block_llsd' },
        @{ Pattern = 'aurora_media'; Replacement = 'block_media' },
        @{ Pattern = 'aurora_prim'; Replacement = 'block_prim' },
        @{ Pattern = 'aurora_voice'; Replacement = 'block_voice' },
        @{ Pattern = 'aurora_updater'; Replacement = 'block_updater' },
        @{ Pattern = 'aurora_version'; Replacement = 'block_version' }
    )

    $textExt = '.rs', '.toml', '.wgsl', '.xml', '.md', '.ftl', '.yml', '.yaml', '.json', '.ps1', '.py', '.sh', '.css', '.html', '.js', '.ts', '.tsx', '.vue', '.ini', '.spec'

    Get-ChildItem -Path $Temp -Recurse -File | Where-Object {
        $textExt -contains $_.Extension.ToLower() -or $_.Name -eq 'Cargo.lock' -or $_.Name -eq 'Cargo.toml'
    } | ForEach-Object {
        $content = [System.IO.File]::ReadAllText($_.FullName)
        $orig = $content
        foreach ($r in $replacements) {
            $content = $content.Replace($r.Pattern, $r.Replacement)
        }
        if ($content -ne $orig) {
            if (-not $DryRun) {
                [System.IO.File]::WriteAllText($_.FullName, $content)
            }
        }
    }

    $KeepFromOurs = @(
        'LICENSE.md',
        'NOTICE.md',
        'TRADEMARKS.md',
        'README.md',
        '.github/release-notes-header.md',
        'news/en.md',
        'news/fr.md',
        'news/ja.md',
        'peers/block-staff.json',
        'peers/block-peers.json',
        'peers/README.md',
        'update-policy.json'
    )

    $exclude = @('.git', 'target', '.scratch')
    Write-Host "Copying into $Root ..."
    if ($DryRun) {
        Write-Host "(dry run: would copy $($Temp) -> $Root)"
        return
    }

    robocopy $Temp $Root /MIR /XD $exclude /NFL /NDL /NJH /NJS /nc /ns /np | Out-Null
    if ($LASTEXITCODE -ge 8) { throw "robocopy failed with exit $LASTEXITCODE" }

    foreach ($rel in $KeepFromOurs) {
        $src = git show "HEAD:$rel" 2>$null
        if ($LASTEXITCODE -eq 0 -and $src) {
            $dest = Join-Path $Root $rel
            $dir = Split-Path $dest -Parent
            if (-not (Test-Path $dir)) { New-Item -ItemType Directory -Path $dir -Force | Out-Null }
            [System.IO.File]::WriteAllText($dest, $src)
            Write-Host "Restored Block branding: $rel"
        }
    }

    # Block mark assets always win over renamed Aurora logos.
    if (Test-Path (Join-Path $Root 'assets/branding/block')) {
        Write-Host "Block mark assets in assets/branding/block/ unchanged by sync."
    }

    Write-Host "Done. Review diff, run scripts/check.ps1, then commit."
    Write-Host "Sync $Tag from $sha"
}
finally {
    Remove-Item -Recurse -Force $Temp -ErrorAction SilentlyContinue
}