# Rebrand user-visible "Aurora" viewer strings to Block / Block Viewer in locale FTL.
# Skips license/legal about-* keys and file-header attribution comments.
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$locales = Join-Path $root 'crates\block-i18n\locales'
$utf8 = New-Object System.Text.UTF8Encoding $false

$skipKeyPattern = '^(about-licen|about-tab-licen)'

function Protect-AuroraTokens([string]$text) {
    $tokens = [ordered]@{}
    $i = 0
    $patterns = @(
        '#Aurora(?:/[^\s#"\\)]*)?',
        'Aurora%20[^"\\s]*',
        'Aurora AO',
        'Aurora Library',
        '~AuroraSelfTest',
        'aurora-<text>\.log',
        'aurora-demo-<text>\.log',
        'aurora-\$\{',
        'aurora\.log',
        'aurora\.previous\.log',
        'aurora-<text>',
        'Aurora-Viewer',
        'Aurora-Update-Policy'
    )
    foreach ($pat in $patterns) {
        $regex = [regex]::new($pat)
        while ($text -match $pat) {
            $m = $regex.Match($text)
            if (-not $m.Success) { break }
            $key = [char]0xE000 + [string]$i
            $tokens[$key] = $m.Value
            $text = $text.Remove($m.Index, $m.Length).Insert($m.Index, $key)
            $i++
        }
    }
    return [pscustomobject]@{ Text = $text; Tokens = $tokens }
}

function Restore-AuroraTokens([string]$text, $tokens) {
    foreach ($entry in $tokens.GetEnumerator()) {
        $text = $text.Replace($entry.Key, $entry.Value)
    }
    return $text
}

function Rewrite-ViewerName([string]$text) {
    if ([string]::IsNullOrEmpty($text)) { return $text }
    if ($text -notmatch 'Aurora') { return $text }

    $wrapped = Protect-AuroraTokens $text
    $t = $wrapped.Text

    # Longest phrases first (viewer product, not place names left in tokens).
    $replacements = @(
        @{ Old = 'Block Viewer Viewer'; New = 'Block Viewer' },
        @{ Old = 'the Aurora project'; New = 'the Block Viewer project' },
        @{ Old = 'Aurora project'; New = 'Block Viewer project' },
        @{ Old = 'new version of Aurora'; New = 'new version of Block Viewer' },
        @{ Old = 'A version of Aurora'; New = 'A version of Block Viewer' },
        @{ Old = 'version of Aurora'; New = 'version of Block Viewer' },
        @{ Old = 'Update Aurora before'; New = 'Update Block Viewer before' },
        @{ Old = 'Update Aurora '; New = 'Update Block Viewer ' },
        @{ Old = 'update Aurora to'; New = 'update Block Viewer to' },
        @{ Old = 'Install and restart Aurora'; New = 'Install and restart Block Viewer' },
        @{ Old = 'restart Aurora'; New = 'restart Block Viewer' },
        @{ Old = 'start Aurora from'; New = 'start Block Viewer from' },
        @{ Old = 'other Aurora instances'; New = 'other Block Viewer instances' },
        @{ Old = 'every Aurora window'; New = 'every Block Viewer window' },
        @{ Old = 'every Aurora user'; New = 'every Block Viewer user' },
        @{ Old = 'Welcome to Aurora!'; New = 'Welcome to Block Viewer!' },
        @{ Old = '# Welcome to Aurora'; New = '# Welcome to Block Viewer' },
        @{ Old = 'Aurora menu'; New = 'Block menu' },
        @{ Old = 'Aurora settings'; New = 'Block Viewer settings' },
        @{ Old = 'Aurora LiveMesh'; New = 'Block LiveMesh' },
        @{ Old = 'Aurora plug-in'; New = 'Block Viewer plug-in' },
        @{ Old = 'Aurora animation project'; New = 'Block Viewer animation project' },
        @{ Old = 'Aurora pose'; New = 'Block Viewer pose' },
        @{ Old = 'Aurora or Legacy'; New = 'Block Viewer or Legacy' },
        @{ Old = 'Connect to Aurora'; New = 'Connect to Block Viewer' },
        @{ Old = 'Sidebar › Aurora ›'; New = 'Sidebar › Block ›' },
        @{ Old = '› Aurora ›'; New = '› Block ›' },
        @{ Old = 'Send from Aurora'; New = 'Send from Block Viewer' },
        @{ Old = 'Help improve Aurora'; New = 'Help improve Block Viewer' },
        @{ Old = 'Aurora recommends'; New = 'Block Viewer recommends' },
        @{ Old = 'Aurora settings backup'; New = 'Block Viewer settings backup' },
        @{ Old = 'Quit Aurora'; New = 'Quit Block Viewer' },
        @{ Old = 'closes Aurora'; New = 'closes Block Viewer' },
        @{ Old = 'close Aurora'; New = 'close Block Viewer' },
        @{ Old = 'Not available in Aurora yet'; New = 'Not available in Block Viewer yet' },
        @{ Old = 'open in Aurora'; New = 'open in Block Viewer' },
        @{ Old = 'links in Aurora'; New = 'links in Block Viewer' },
        @{ Old = 'pages open in Aurora'; New = 'pages open in Block Viewer' },
        @{ Old = 'outside Aurora'; New = 'outside Block Viewer' },
        @{ Old = 'from outside Aurora'; New = 'from outside Block Viewer' },
        @{ Old = 'for Aurora in Discord'; New = 'for Block Viewer in Discord' },
        @{ Old = 'working for Aurora'; New = 'working for Block Viewer' },
        @{ Old = 'start of Aurora'; New = 'start of Block Viewer' },
        @{ Old = 'after Aurora started'; New = 'after Block Viewer started' },
        @{ Old = 'Aurora started'; New = 'Block Viewer started' },
        @{ Old = 'Aurora sets up VR'; New = 'Block Viewer sets up VR' },
        @{ Old = 'Aurora will also load'; New = 'Block Viewer will also load' },
        @{ Old = 'Aurora does not load'; New = 'Block Viewer does not load' },
        @{ Old = 'Aurora will publish'; New = 'Block Viewer will publish' },
        @{ Old = 'Aurora will restart'; New = 'Block Viewer will restart' },
        @{ Old = 'Aurora will offer'; New = 'Block Viewer will offer' },
        @{ Old = 'Aurora still works'; New = 'Block Viewer still works' },
        @{ Old = 'Aurora reopened'; New = 'Block Viewer reopened' },
        @{ Old = 'Aurora reopens'; New = 'Block Viewer reopens' },
        @{ Old = 'Aurora loads'; New = 'Block Viewer loads' },
        @{ Old = 'Aurora closes'; New = 'Block Viewer closes' },
        @{ Old = 'Aurora can close'; New = 'Block Viewer can close' },
        @{ Old = 'Aurora stops'; New = 'Block Viewer stops' },
        @{ Old = 'Aurora crashes'; New = 'Block Viewer crashes' },
        @{ Old = 'Aurora closed'; New = 'Block Viewer closed' },
        @{ Old = 'Aurora runs'; New = 'Block Viewer runs' },
        @{ Old = 'Aurora needs'; New = 'Block Viewer needs' },
        @{ Old = 'Aurora tries again'; New = 'Block Viewer tries again' },
        @{ Old = 'Aurora sends'; New = 'Block Viewer sends' },
        @{ Old = 'Aurora frames'; New = 'Block Viewer frames' },
        @{ Old = 'Aurora puts on'; New = 'Block Viewer puts on' },
        @{ Old = 'Aurora draws'; New = 'Block Viewer draws' },
        @{ Old = 'Aurora downloads'; New = 'Block Viewer downloads' },
        @{ Old = 'Aurora removes'; New = 'Block Viewer removes' },
        @{ Old = 'Aurora registered'; New = 'Block Viewer registered' },
        @{ Old = 'Aurora is resting'; New = 'Block Viewer is resting' },
        @{ Old = 'Aurora is behind'; New = 'Block Viewer is behind' },
        @{ Old = 'Aurora is not in front'; New = 'Block Viewer is not in front' },
        @{ Old = 'Aurora is in the background'; New = 'Block Viewer is in the background' },
        @{ Old = 'Aurora comes back'; New = 'Block Viewer comes back' },
        @{ Old = 'Aurora slows'; New = 'Block Viewer slows' },
        @{ Old = 'Aurora asks'; New = 'Block Viewer asks' },
        @{ Old = 'Aurora already uses'; New = 'Block Viewer already uses' },
        @{ Old = 'Aurora already has'; New = 'Block Viewer already has' },
        @{ Old = 'Aurora only adds'; New = 'Block Viewer only adds' },
        @{ Old = 'Aurora stops showing'; New = 'Block Viewer stops showing' },
        @{ Old = 'Aurora does not have'; New = 'Block Viewer does not have' },
        @{ Old = 'this version of Aurora'; New = 'this version of Block Viewer' },
        @{ Old = 'version of Aurora has'; New = 'version of Block Viewer has' },
        @{ Old = 'Downloading Aurora''s'; New = 'Downloading Block Viewer''s' },
        @{ Old = 'Aurora''s trusted sites'; New = 'Block Viewer''s trusted sites' },
        @{ Old = 'Aurora''s settings'; New = 'Block Viewer''s settings' },
        @{ Old = 'Aurora''s add-on'; New = 'Block Viewer''s add-on' },
        @{ Old = 'Aurora''s own plugins'; New = 'Block Viewer''s own plugins' },
        @{ Old = 'Aurora''s own'; New = 'Block''s own' },
        @{ Old = 'Aurora''s window'; New = 'Block''s window' },
        @{ Old = 'Aurora''s adjustable workspace'; New = 'Block''s adjustable workspace' },
        @{ Old = 'Aurora''s upload inspector'; New = 'Block Viewer''s upload inspector' },
        @{ Old = 'Aurora''s browser'; New = 'Block Viewer''s browser' },
        @{ Old = 'Aurora''s notification center'; New = 'Block''s notification center' },
        @{ Old = 'Aurora''s web and media support'; New = 'Block Viewer''s web and media support' },
        @{ Old = 'Aurora''s media plugins'; New = 'Block Viewer''s media plugins' },
        @{ Old = 'Aurora''s windows and bars'; New = 'Block Viewer''s windows and bars' },
        @{ Old = 'outside Aurora''s'; New = 'outside Block Viewer''s' },
        @{ Old = 'Aurora''s own files'; New = 'Block Viewer''s own files' },
        @{ Old = 'Aurora plates'; New = 'Block plates' },
        @{ Old = 'Aurora panel'; New = 'Block panel' },
        @{ Old = 'Aurora page'; New = 'Block page' },
        @{ Old = 'Aurora: the messages'; New = 'Block: the messages' },
        @{ Old = 'Aurora: a bubble'; New = 'Block: a bubble' },
        @{ Old = 'Aurora { $version }'; New = 'Block Viewer { $version }' },
        @{ Old = 'Backup of { $date }, Aurora { $version }'; New = 'Backup of { $date }, Block Viewer { $version }' },
        @{ Old = 'Aurora demo estate'; New = 'Block Viewer demo estate' },
        @{ Old = 'Aurora Demo estate'; New = 'Block Viewer demo estate' },
        @{ Old = 'Aurora Demo'; New = 'Block Demo' },
        @{ Old = 'Aurora Elsewhere'; New = 'Block Elsewhere' },
        @{ Old = 'Aurora experience'; New = 'Block Viewer experience' },
        @{ Old = 'Aurora Builders'; New = 'Block Builders' },
        @{ Old = 'Aurora Club'; New = 'Block Club' },
        @{ Old = 'Aurora Plaza'; New = 'Block Plaza' },
        @{ Old = 'Aurora plaza'; New = 'Block plaza' },
        @{ Old = 'Aurora beach'; New = 'Block beach' },
        @{ Old = 'Aurora Beach'; New = 'Block Beach' },
        @{ Old = 'Aurora Lagoon'; New = 'Block Lagoon' },
        @{ Old = 'Aurora lantern'; New = 'Block lantern' },
        @{ Old = 'Aurora Observatory'; New = 'Block Observatory' },
        @{ Old = 'Aurora media'; New = 'Block media' },
        @{ Old = 'Aurora Shape'; New = 'Block Shape' },
        @{ Old = 'Aurora Physics'; New = 'Block Physics' },
        @{ Old = 'Aurora hair'; New = 'Block hair' },
        @{ Old = 'Aurora skin'; New = 'Block skin' },
        @{ Old = 'Aurora eyes'; New = 'Block eyes' },
        @{ Old = 'Aurora silhouette'; New = 'Block silhouette' },
        @{ Old = 'Aurora duotone'; New = 'Block duotone' },
        @{ Old = 'Aurora 3D'; New = 'Block 3D' },
        @{ Old = 'Aurora water'; New = 'Block water' },
        @{ Old = 'Aurora process'; New = 'Block Viewer process' },
        @{ Old = 'Aurora uses'; New = 'Block Viewer uses' },
        @{ Old = '% Aurora'; New = '% Block Viewer' },
        @{ Old = 'Missing Aurora water'; New = 'Missing Block water' },
        @{ Old = 'Copy one set into Aurora'; New = 'Copy one set into Block Viewer' },
        @{ Old = '(Aurora, Compact and Bubbles'; New = '(Block, Compact and Bubbles' },
        @{ Old = 'new Aurora release'; New = 'new Block Viewer release' }
    )

    foreach ($pair in $replacements) {
        $t = $t.Replace($pair.Old, $pair.New)
    }

  # Short UI labels (exact value = product name).
    if ($t -eq 'Aurora') { $t = 'Block' }

    $t = $t -replace '\bAurora(?=,)', 'Block'
    $t = $t -replace '(?<![#/])\bAurora''s\b', "Block Viewer's"
    $t = $t -replace '(?<!Block Viewer )\bAurora\b(?!\s*(AO|Library|Demo|%))', 'Block Viewer'

    $t = Restore-AuroraTokens $t $wrapped.Tokens
    $t = $t.Replace('Block Viewer Viewer', 'Block Viewer')
    return $t
}

Get-ChildItem $locales -Filter '*.ftl' | ForEach-Object {
    $path = $_.FullName
    $lines = [System.IO.File]::ReadAllLines($path, $utf8)
    $changed = $false
    for ($i = 0; $i -lt $lines.Length; $i++) {
        $line = $lines[$i]
        if ($line -match '^##') { continue }

        if ($line -match '^\s*([A-Za-z0-9-]+)\s*=\s*(.*)$') {
            $key = $Matches[1]
            if ($key -match $skipKeyPattern) { continue }
            $value = $Matches[2]
            $newValue = Rewrite-ViewerName $value
            if ($newValue -ne $value) {
                $lines[$i] = ($key + ' = ' + $newValue)
                $changed = $true
            }
            continue
        }

        # Multiline message bodies and inline comments in values.
        if ($line -match 'Aurora') {
            $newLine = Rewrite-ViewerName $line
            if ($newLine -ne $line) {
                $lines[$i] = $newLine
                $changed = $true
            }
        }
    }

    if ($changed) {
        [System.IO.File]::WriteAllLines($path, $lines, $utf8)
        Write-Host "Updated $($_.Name)"
    }
}