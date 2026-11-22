# Sync an Block Viewer release tag into Block Viewer (rename aurora-* -> block-*).
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
    # PowerShell pipes are text and would corrupt `git archive | tar`.
    $archive = Join-Path $env:TEMP "block-sync-$Tag-$(Get-Random).tar"
    git archive --format=tar --output $archive $sha
    if ($LASTEXITCODE -ne 0) { throw "git archive failed with exit $LASTEXITCODE" }
    tar -xf $archive -C $Temp
    if ($LASTEXITCODE -ne 0) { throw "tar extract failed with exit $LASTEXITCODE" }
    Remove-Item -Force $archive -ErrorAction SilentlyContinue

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
            Rename-Item -LiteralPath $from -NewName $pair.Value
        }
    }

    # Longer crate ids first. String.Replace is not word-bounded; these names are
    # the package/crate identifiers missing from the original list (folder rename
    # alone leaves Cargo.toml and `use` paths pointing at aurora-*).
    $replacements = @(
        @{ Pattern = 'aurora-editor-sync'; Replacement = 'block-editor-sync' },
        @{ Pattern = 'aurora-build-gate'; Replacement = 'block-build-gate' },
        @{ Pattern = 'aurora-assets'; Replacement = 'block-assets' },
        @{ Pattern = 'aurora-proxy'; Replacement = 'block-proxy' },
        @{ Pattern = 'aurora_editor_sync'; Replacement = 'block_editor_sync' },
        @{ Pattern = 'aurora_build_gate'; Replacement = 'block_build_gate' },
        @{ Pattern = 'aurora_proxy'; Replacement = 'block_proxy' },
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
                # .NET Framework WriteAllText emits a UTF-8 BOM, which breaks Fluent.
                $utf8 = New-Object System.Text.UTF8Encoding $false
                [System.IO.File]::WriteAllText($_.FullName, $content, $utf8)
            }
        }
    }

    $KeepFromOurs = @(
        'LICENSE.md',
        'NOTICE.md',
        'TRADEMARKS.md',
        'README.md',
        '.github/workflows/release.yml',
        '.github/workflows/discord-release.yml',
        'crates/block-updater/src/feed.rs',
        'crates/block-updater/src/package.rs',
        'crates/block-updater/src/install.rs',
        'crates/block-version/src/lib.rs',
        'scripts/package.ps1',
        'scripts/package-macos.ps1',
        'scripts/write-update-policy.ps1',
        'scripts/build-release.ps1',
        'packaging/linux/block-viewer.desktop',
        '.github/release-notes-header.md',
        'news/en.md',
        'news/fr.md',
        'news/ja.md',
        'peers/block-staff.json',
        'peers/block-peers.json',
        'peers/README.md',
        'update-policy.json'
    )

    # /MIR drops paths the tag does not ship. A worktree .git is a file, so /XD
    # does not protect it; /XF does. Block-only paths are checked out afterwards.
    $excludeDirs = @('.git', 'target', '.scratch', 'node_modules')
    $KeepTrees = @(
        'website',
        'workers',
        'motd',
        'peers',
        'block-tools.cmd',
        'docs/AURORA_SYNC_0.9.12.md',
        'assets/branding/block'
    )
    Write-Host "Copying into $Root ..."
    if ($DryRun) {
        Write-Host "(dry run: would copy $($Temp) -> $Root)"
        return
    }

    $scriptKeep = Join-Path $env:TEMP "sync-aurora-tag.ps1.keep"
    Copy-Item -LiteralPath $PSCommandPath -Destination $scriptKeep -Force

    robocopy $Temp $Root /MIR /XD $excludeDirs /XF .git /NFL /NDL /NJH /NJS /nc /ns /np | Out-Null
    if ($LASTEXITCODE -ge 8) { throw "robocopy failed with exit $LASTEXITCODE" }
    if (-not (Test-Path -LiteralPath (Join-Path $Root '.git'))) { throw "sync removed .git" }

    $restore = @($KeepFromOurs + $KeepTrees)
    git checkout HEAD -- @restore
    if ($LASTEXITCODE -ne 0) { throw "git checkout of Block branding failed" }
    $logos = @(git ls-tree -r --name-only HEAD -- assets/branding | Where-Object { $_ -like 'assets/branding/block*' })
    if ($logos.Count -gt 0) {
        git checkout HEAD -- @logos
        if ($LASTEXITCODE -ne 0) { throw "git checkout of Block logos failed" }
    }
    Copy-Item -LiteralPath $scriptKeep -Destination (Join-Path $Root 'scripts\sync-aurora-tag.ps1') -Force
    foreach ($rel in $restore) { Write-Host "Restored Block branding: $rel" }

    if (Test-Path (Join-Path $Root 'assets/branding/block')) {
        Write-Host "Block mark assets in assets/branding/block/ restored from HEAD."
    }

    Write-Host "Done. Review diff, run scripts/check.ps1, then commit."
    Write-Host "Sync $Tag from $sha"
}
finally {
    Remove-Item -Recurse -Force $Temp -ErrorAction SilentlyContinue
}