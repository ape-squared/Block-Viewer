# Rebrand user-visible "Block" viewer strings to Block / Block Viewer in locale FTL.
# Skips license/legal about-* keys and file-header attribution comments.
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$locales = Join-Path $root 'crates\block-i18n\locales'
$utf8 = New-Object System.Text.UTF8Encoding $false

$skipKeyPattern = '^(about-licen|about-tab-licen)'

function Protect-BlockTokens([string]$text) {
    $tokens = [ordered]@{}
    $i = 0
    $patterns = @(
        '#Aurora(?:/[^\s#"\\)]*)?',
        'Block%20[^"\\s]*',
        'Block AO',
        'Block Library',
        '~BlockSelfTest',
        'block-<text>\.log',
        'block-demo-<text>\.log',
        'block-\$\{',
        'block\.log',
        'block\.previous\.log',
        'block-<text>',
        'Block-Viewer',
        'Block-Update-Policy'
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

function Restore-BlockTokens([string]$text, $tokens) {
    foreach ($entry in $tokens.GetEnumerator()) {
        $text = $text.Replace($entry.Key, $entry.Value)
    }
    return $text
}

function Rewrite-ViewerName([string]$text) {
    if ([string]::IsNullOrEmpty($text)) { return $text }
    if ($text -notmatch 'Block') { return $text }

    $wrapped = Protect-BlockTokens $text
    $t = $wrapped.Text

    # Longest phrases first (viewer product, not place names left in tokens).
    $replacements = @(
        @{ Old = 'Block Viewer Viewer'; New = 'Block Viewer' },
        @{ Old = 'the Block project'; New = 'the Block Viewer project' },
        @{ Old = 'Block project'; New = 'Block Viewer project' },
        @{ Old = 'new version of Block'; New = 'new version of Block Viewer' },
        @{ Old = 'A version of Block'; New = 'A version of Block Viewer' },
        @{ Old = 'version of Block'; New = 'version of Block Viewer' },
        @{ Old = 'Update Block before'; New = 'Update Block Viewer before' },
        @{ Old = 'Update Block '; New = 'Update Block Viewer ' },
        @{ Old = 'update Block to'; New = 'update Block Viewer to' },
        @{ Old = 'Install and restart Block'; New = 'Install and restart Block Viewer' },
        @{ Old = 'restart Block'; New = 'restart Block Viewer' },
        @{ Old = 'start Block from'; New = 'start Block Viewer from' },
        @{ Old = 'other Block instances'; New = 'other Block Viewer instances' },
        @{ Old = 'every Block window'; New = 'every Block Viewer window' },
        @{ Old = 'every Block user'; New = 'every Block Viewer user' },
        @{ Old = 'Welcome to Block!'; New = 'Welcome to Block Viewer!' },
        @{ Old = '# Welcome to Block'; New = '# Welcome to Block Viewer' },
        @{ Old = 'Block menu'; New = 'Block menu' },
        @{ Old = 'Block settings'; New = 'Block Viewer settings' },
        @{ Old = 'Block LiveMesh'; New = 'Block LiveMesh' },
        @{ Old = 'Block plug-in'; New = 'Block Viewer plug-in' },
        @{ Old = 'Block animation project'; New = 'Block Viewer animation project' },
        @{ Old = 'Block pose'; New = 'Block Viewer pose' },
        @{ Old = 'Block or Legacy'; New = 'Block Viewer or Legacy' },
        @{ Old = 'Connect to Block'; New = 'Connect to Block Viewer' },
        @{ Old = 'Sidebar › Block ›'; New = 'Sidebar › Block ›' },
        @{ Old = '› Block ›'; New = '› Block ›' },
        @{ Old = 'Send from Block'; New = 'Send from Block Viewer' },
        @{ Old = 'Help improve Block'; New = 'Help improve Block Viewer' },
        @{ Old = 'Block recommends'; New = 'Block Viewer recommends' },
        @{ Old = 'Block settings backup'; New = 'Block Viewer settings backup' },
        @{ Old = 'Quit Block'; New = 'Quit Block Viewer' },
        @{ Old = 'closes Block'; New = 'closes Block Viewer' },
        @{ Old = 'close Block'; New = 'close Block Viewer' },
        @{ Old = 'Not available in Block yet'; New = 'Not available in Block Viewer yet' },
        @{ Old = 'open in Block'; New = 'open in Block Viewer' },
        @{ Old = 'links in Block'; New = 'links in Block Viewer' },
        @{ Old = 'pages open in Block'; New = 'pages open in Block Viewer' },
        @{ Old = 'outside Block'; New = 'outside Block Viewer' },
        @{ Old = 'from outside Block'; New = 'from outside Block Viewer' },
        @{ Old = 'for Block in Discord'; New = 'for Block Viewer in Discord' },
        @{ Old = 'working for Block'; New = 'working for Block Viewer' },
        @{ Old = 'start of Block'; New = 'start of Block Viewer' },
        @{ Old = 'after Block started'; New = 'after Block Viewer started' },
        @{ Old = 'Block started'; New = 'Block Viewer started' },
        @{ Old = 'Block sets up VR'; New = 'Block Viewer sets up VR' },
        @{ Old = 'Block will also load'; New = 'Block Viewer will also load' },
        @{ Old = 'Block does not load'; New = 'Block Viewer does not load' },
        @{ Old = 'Block will publish'; New = 'Block Viewer will publish' },
        @{ Old = 'Block will restart'; New = 'Block Viewer will restart' },
        @{ Old = 'Block will offer'; New = 'Block Viewer will offer' },
        @{ Old = 'Block still works'; New = 'Block Viewer still works' },
        @{ Old = 'Block reopened'; New = 'Block Viewer reopened' },
        @{ Old = 'Block reopens'; New = 'Block Viewer reopens' },
        @{ Old = 'Block loads'; New = 'Block Viewer loads' },
        @{ Old = 'Block closes'; New = 'Block Viewer closes' },
        @{ Old = 'Block can close'; New = 'Block Viewer can close' },
        @{ Old = 'Block stops'; New = 'Block Viewer stops' },
        @{ Old = 'Block crashes'; New = 'Block Viewer crashes' },
        @{ Old = 'Block closed'; New = 'Block Viewer closed' },
        @{ Old = 'Block runs'; New = 'Block Viewer runs' },
        @{ Old = 'Block needs'; New = 'Block Viewer needs' },
        @{ Old = 'Block tries again'; New = 'Block Viewer tries again' },
        @{ Old = 'Block sends'; New = 'Block Viewer sends' },
        @{ Old = 'Block frames'; New = 'Block Viewer frames' },
        @{ Old = 'Block puts on'; New = 'Block Viewer puts on' },
        @{ Old = 'Block draws'; New = 'Block Viewer draws' },
        @{ Old = 'Block downloads'; New = 'Block Viewer downloads' },
        @{ Old = 'Block removes'; New = 'Block Viewer removes' },
        @{ Old = 'Block registered'; New = 'Block Viewer registered' },
        @{ Old = 'Block is resting'; New = 'Block Viewer is resting' },
        @{ Old = 'Block is behind'; New = 'Block Viewer is behind' },
        @{ Old = 'Block is not in front'; New = 'Block Viewer is not in front' },
        @{ Old = 'Block is in the background'; New = 'Block Viewer is in the background' },
        @{ Old = 'Block comes back'; New = 'Block Viewer comes back' },
        @{ Old = 'Block slows'; New = 'Block Viewer slows' },
        @{ Old = 'Block asks'; New = 'Block Viewer asks' },
        @{ Old = 'Block already uses'; New = 'Block Viewer already uses' },
        @{ Old = 'Block already has'; New = 'Block Viewer already has' },
        @{ Old = 'Block only adds'; New = 'Block Viewer only adds' },
        @{ Old = 'Block stops showing'; New = 'Block Viewer stops showing' },
        @{ Old = 'Block does not have'; New = 'Block Viewer does not have' },
        @{ Old = 'this version of Block'; New = 'this version of Block Viewer' },
        @{ Old = 'version of Block has'; New = 'version of Block Viewer has' },
        @{ Old = 'Downloading Block''s'; New = 'Downloading Block Viewer''s' },
        @{ Old = 'Block''s trusted sites'; New = 'Block Viewer''s trusted sites' },
        @{ Old = 'Block''s settings'; New = 'Block Viewer''s settings' },
        @{ Old = 'Block''s add-on'; New = 'Block Viewer''s add-on' },
        @{ Old = 'Block''s own plugins'; New = 'Block Viewer''s own plugins' },
        @{ Old = 'Block''s own'; New = 'Block''s own' },
        @{ Old = 'Block''s window'; New = 'Block''s window' },
        @{ Old = 'Block''s adjustable workspace'; New = 'Block''s adjustable workspace' },
        @{ Old = 'Block''s upload inspector'; New = 'Block Viewer''s upload inspector' },
        @{ Old = 'Block''s browser'; New = 'Block Viewer''s browser' },
        @{ Old = 'Block''s notification center'; New = 'Block''s notification center' },
        @{ Old = 'Block''s web and media support'; New = 'Block Viewer''s web and media support' },
        @{ Old = 'Block''s media plugins'; New = 'Block Viewer''s media plugins' },
        @{ Old = 'Block''s windows and bars'; New = 'Block Viewer''s windows and bars' },
        @{ Old = 'outside Block''s'; New = 'outside Block Viewer''s' },
        @{ Old = 'Block''s own files'; New = 'Block Viewer''s own files' },
        @{ Old = 'Block plates'; New = 'Block plates' },
        @{ Old = 'Block panel'; New = 'Block panel' },
        @{ Old = 'Block page'; New = 'Block page' },
        @{ Old = 'Block: the messages'; New = 'Block: the messages' },
        @{ Old = 'Block: a bubble'; New = 'Block: a bubble' },
        @{ Old = 'Block { $version }'; New = 'Block Viewer { $version }' },
        @{ Old = 'Backup of { $date }, Block { $version }'; New = 'Backup of { $date }, Block Viewer { $version }' },
        @{ Old = 'Block demo estate'; New = 'Block Viewer demo estate' },
        @{ Old = 'Block Demo estate'; New = 'Block Viewer demo estate' },
        @{ Old = 'Block Demo'; New = 'Block Demo' },
        @{ Old = 'Block Elsewhere'; New = 'Block Elsewhere' },
        @{ Old = 'Block experience'; New = 'Block Viewer experience' },
        @{ Old = 'Block Builders'; New = 'Block Builders' },
        @{ Old = 'Block Club'; New = 'Block Club' },
        @{ Old = 'Block Plaza'; New = 'Block Plaza' },
        @{ Old = 'Block plaza'; New = 'Block plaza' },
        @{ Old = 'Block beach'; New = 'Block beach' },
        @{ Old = 'Block Beach'; New = 'Block Beach' },
        @{ Old = 'Block Lagoon'; New = 'Block Lagoon' },
        @{ Old = 'Block lantern'; New = 'Block lantern' },
        @{ Old = 'Block Observatory'; New = 'Block Observatory' },
        @{ Old = 'Block media'; New = 'Block media' },
        @{ Old = 'Block Shape'; New = 'Block Shape' },
        @{ Old = 'Block Physics'; New = 'Block Physics' },
        @{ Old = 'Block hair'; New = 'Block hair' },
        @{ Old = 'Block skin'; New = 'Block skin' },
        @{ Old = 'Block eyes'; New = 'Block eyes' },
        @{ Old = 'Block silhouette'; New = 'Block silhouette' },
        @{ Old = 'Block duotone'; New = 'Block duotone' },
        @{ Old = 'Block 3D'; New = 'Block 3D' },
        @{ Old = 'Block water'; New = 'Block water' },
        @{ Old = 'Block process'; New = 'Block Viewer process' },
        @{ Old = 'Block uses'; New = 'Block Viewer uses' },
        @{ Old = '% Block'; New = '% Block Viewer' },
        @{ Old = 'Missing Block water'; New = 'Missing Block water' },
        @{ Old = 'Copy one set into Block'; New = 'Copy one set into Block Viewer' },
        @{ Old = '(Block, Compact and Bubbles'; New = '(Block, Compact and Bubbles' },
        @{ Old = 'new Block release'; New = 'new Block Viewer release' }
    )

    foreach ($pair in $replacements) {
        $t = $t.Replace($pair.Old, $pair.New)
    }

  # Short UI labels (exact value = product name).
    if ($t -eq 'Block') { $t = 'Block' }

    $t = $t -replace '\bBlock(?=,)', 'Block'
    $t = $t -replace '(?<![#/])\bBlock''s\b', "Block Viewer's"
    $t = $t -replace '(?<!Block Viewer )\bBlock\b(?!\s*(AO|Library|Demo|%))', 'Block Viewer'

    $t = Restore-BlockTokens $t $wrapped.Tokens
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
        if ($line -match 'Block') {
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