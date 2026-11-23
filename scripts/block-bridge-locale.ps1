# Rewrites viewer-branded "Block bridge" strings to "Block bridge" in locale FTL files.
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
if (-not (Test-Path (Join-Path $root 'crates\block-i18n\locales'))) {
    $root = Split-Path -Parent $PSScriptRoot
}
$locales = Join-Path $root 'crates\block-i18n\locales'
$utf8 = New-Object System.Text.UTF8Encoding $false

# Longer phrases first; bridge-adjacent product name only.
$replacements = @(
    @{ Old = 'Authenticated read-only Block bridge queries'; New = 'Authenticated read-only Block bridge queries' },
    @{ Old = 'through the Block bridge'; New = 'through the Block bridge' },
    @{ Old = 'with the Block bridge'; New = 'with the Block bridge' },
    @{ Old = 'the Block bridge'; New = 'the Block bridge' },
    @{ Old = 'The Block bridge'; New = 'The Block bridge' },
    @{ Old = 'wear the Block bridge'; New = 'wear the Block bridge' },
    @{ Old = 'Use the Block bridge'; New = 'Use the Block bridge' },
    @{ Old = 'Repair Block bridge'; New = 'Repair Block bridge' },
    @{ Old = 'Preparing Block bridge'; New = 'Preparing Block bridge' },
    @{ Old = 'Block bridge ('; New = 'Block bridge (' },
    @{ Old = 'Block bridge'; New = 'Block bridge' },
    @{ Old = 'Block Köprüsünü Onar'; New = 'Block köprüsünü onar' },
    @{ Old = 'Block köprüsünü kullan'; New = 'Block köprüsünü kullan' },
    @{ Old = 'Block köprüsü'; New = 'Block köprüsü' },
    @{ Old = 'Block Köprüsü'; New = 'Block köprüsü' },
    @{ Old = 'Aurora-broen'; New = 'Block-broen' },
    @{ Old = 'Blockブリッジ'; New = 'Blockブリッジ' },
    @{ Old = 'Block 桥接器'; New = 'Block 桥接器' },
    @{ Old = 'Block 橋接器'; New = 'Block 橋接器' },
    @{ Old = 'Bridge de Block'; New = 'Bridge de Block' },
    @{ Old = 'Bridge Block'; New = 'Bridge Block' },
    @{ Old = 'Bridge di Block'; New = 'Bridge Block' },
    @{ Old = 'Ponte do Block'; New = 'Ponte do Block' },
    @{ Old = 'Ponte Block'; New = 'Ponte Block' },
    @{ Old = 'Aurora-Brücke'; New = 'Block-Brücke' },
    @{ Old = 'Most Block'; New = 'Most Block' },
    @{ Old = 'mostu Block'; New = 'mostu Block' },
    @{ Old = 'mosta Block'; New = 'mosta Block' },
    @{ Old = 'Мост Block'; New = 'Мост Block' },
    @{ Old = 'Міст Block'; New = 'Міст Block' },
    @{ Old = 'мосту Block'; New = 'мосту Block' },
    @{ Old = 'моста Block'; New = 'моста Block' },
    @{ Old = 'мост Block'; New = 'мост Block' },
    @{ Old = 'міст Block'; New = 'міст Block' },
    @{ Old = 'Aurora-Bridge'; New = 'Block-Bridge' },
    @{ Old = 'bridge di Block'; New = 'bridge di Block' },
    @{ Old = 'bridge de Block'; New = 'bridge de Block' },
    @{ Old = 'bridge Block'; New = 'bridge Block' },
    @{ Old = 'le bridge Block'; New = 'le bridge Block' },
    @{ Old = 'la ponte Block'; New = 'la ponte Block' },
    @{ Old = 'a ponte do Block'; New = 'a ponte do Block' },
    @{ Old = 'A ponte do Block'; New = 'A ponte do Block' },
    @{ Old = 'a ponte Block'; New = 'a ponte Block' },
    @{ Old = 'ponte do Block'; New = 'ponte do Block' },
    @{ Old = 'ponte Block'; New = 'ponte Block' },
    @{ Old = 'Usar a ponte Block'; New = 'Usar a ponte Block' },
    @{ Old = 'Usar a ponte do Block'; New = 'Usar a ponte do Block' },
    @{ Old = 'przez most Block'; New = 'przez most Block' },
    @{ Old = 'Napraw most Block'; New = 'Napraw most Block' },
    @{ Old = 'Відновити міст Block'; New = 'Відновити міст Block' },
    @{ Old = 'через міст Block'; New = 'через міст Block' },
    @{ Old = 'Використовувати міст Block'; New = 'Використовувати міст Block' },
    @{ Old = 'Настройки → Block →'; New = 'Настройки → Block →' }
)

$helpViewerReplacements = @(
    @{ Old = 'Block 将不再'; New = 'Block 将不再' },
    @{ Old = 'Block no longer'; New = 'Block no longer' },
    @{ Old = 'lager eller tar Block'; New = 'lager eller tar Block' },
    @{ Old = 'o Block deixa'; New = 'o Block deixa' },
    @{ Old = 'que o Block'; New = 'que o Block' },
    @{ Old = 'que Block crea'; New = 'que Block crea' },
    @{ Old = 'Block crea'; New = 'Block crea' },
    @{ Old = 'Block crée'; New = 'Block crée' },
    @{ Old = 'qu''Block crée'; New = 'que Block crée' },
    @{ Old = 'Block ne le'; New = 'Block ne le' },
    @{ Old = 'Block deja'; New = 'Block deja' },
    @{ Old = 'Block sama'; New = 'Block sama' },
    @{ Old = 'Block создаёт'; New = 'Block создаёт' },
    @{ Old = 'Block снимает'; New = 'Block снимает' },
    @{ Old = 'Block створює'; New = 'Block створює' },
    @{ Old = 'Block більше'; New = 'Block більше' },
    @{ Old = 'den Block für'; New = 'den Block für' },
    @{ Old = 'trägt Block'; New = 'trägt Block' },
    @{ Old = 'erstellt und trägt Block'; New = 'erstellt und trägt Block' },
    @{ Old = "Block'nın"; New = "Block'un" },
    @{ Old = 'Block onu'; New = 'Block onu' },
    @{ Old = 'Kapalıyken Block'; New = 'Kapalıyken Block' }
)

# Viewer makes/wears the bridge (English help).
$lineKeyPrefixes = @(
    'ui-options-block-use-bridge',
    'anim-couple-ask-needs',
    'bridge-',
    'poser-bridge-',
    'radar-opt-bridge',
    'movelock-mode-bridge'
)

Get-ChildItem -Path $locales -Filter '*.ftl' | ForEach-Object {
    $path = $_.FullName
    $text = [System.IO.File]::ReadAllText($path, $utf8)
    $original = $text

    foreach ($pair in $replacements) {
        $text = $text.Replace($pair.Old, $pair.New)
    }

    # Preferences path on bridge-disabled lines in translated files.
    $text = $text -replace '(bridge-disabled\s*=.*?)› Block ›', '$1› Block ›'
    $text = $text -replace '(poser-bridge-disabled\s*=.*?)› Block ›', '$1› Block ›'

    $lines = $text -split "`n", -1
    for ($i = 0; $i -lt $lines.Length; $i++) {
        $line = $lines[$i]
        if ($line -notmatch '^\s*([a-z0-9-]+)\s*=') { continue }
        $key = $Matches[1]
        $isBridgeLine = $false
        foreach ($prefix in $lineKeyPrefixes) {
            if ($key -eq $prefix -or $key.StartsWith($prefix)) { $isBridgeLine = $true; break }
        }
        if (-not $isBridgeLine) { continue }
        foreach ($pair in $helpViewerReplacements) {
            $line = $line.Replace($pair.Old, $pair.New)
        }
        $line = $line -replace '\bBlock makes\b', 'Block makes'
        $line = $line -replace '\bthat Block\b', 'that Block'
        $lines[$i] = $line
    }
    $text = $lines -join "`n"

    if ($text -ne $original) {
        [System.IO.File]::WriteAllText($path, $text, $utf8)
        Write-Host "Updated $($_.Name)"
    }
}