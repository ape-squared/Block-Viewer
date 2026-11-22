# Rewrites viewer-branded "Aurora bridge" strings to "Block bridge" in locale FTL files.
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
if (-not (Test-Path (Join-Path $root 'crates\block-i18n\locales'))) {
    $root = Split-Path -Parent $PSScriptRoot
}
$locales = Join-Path $root 'crates\block-i18n\locales'
$utf8 = New-Object System.Text.UTF8Encoding $false

# Longer phrases first; bridge-adjacent product name only.
$replacements = @(
    @{ Old = 'Authenticated read-only Aurora bridge queries'; New = 'Authenticated read-only Block bridge queries' },
    @{ Old = 'through the Aurora bridge'; New = 'through the Block bridge' },
    @{ Old = 'with the Aurora bridge'; New = 'with the Block bridge' },
    @{ Old = 'the Aurora bridge'; New = 'the Block bridge' },
    @{ Old = 'The Aurora bridge'; New = 'The Block bridge' },
    @{ Old = 'wear the Aurora bridge'; New = 'wear the Block bridge' },
    @{ Old = 'Use the Aurora bridge'; New = 'Use the Block bridge' },
    @{ Old = 'Repair Aurora bridge'; New = 'Repair Block bridge' },
    @{ Old = 'Preparing Aurora bridge'; New = 'Preparing Block bridge' },
    @{ Old = 'Aurora bridge ('; New = 'Block bridge (' },
    @{ Old = 'Aurora bridge'; New = 'Block bridge' },
    @{ Old = 'Aurora Köprüsünü Onar'; New = 'Block köprüsünü onar' },
    @{ Old = 'Aurora köprüsünü kullan'; New = 'Block köprüsünü kullan' },
    @{ Old = 'Aurora köprüsü'; New = 'Block köprüsü' },
    @{ Old = 'Aurora Köprüsü'; New = 'Block köprüsü' },
    @{ Old = 'Aurora-broen'; New = 'Block-broen' },
    @{ Old = 'Auroraブリッジ'; New = 'Blockブリッジ' },
    @{ Old = 'Aurora 桥接器'; New = 'Block 桥接器' },
    @{ Old = 'Aurora 橋接器'; New = 'Block 橋接器' },
    @{ Old = 'Bridge de Aurora'; New = 'Bridge de Block' },
    @{ Old = 'Bridge Aurora'; New = 'Bridge Block' },
    @{ Old = 'Bridge di Aurora'; New = 'Bridge Block' },
    @{ Old = 'Ponte do Aurora'; New = 'Ponte do Block' },
    @{ Old = 'Ponte Aurora'; New = 'Ponte Block' },
    @{ Old = 'Aurora-Brücke'; New = 'Block-Brücke' },
    @{ Old = 'Most Aurora'; New = 'Most Block' },
    @{ Old = 'mostu Aurora'; New = 'mostu Block' },
    @{ Old = 'mosta Aurora'; New = 'mosta Block' },
    @{ Old = 'Мост Aurora'; New = 'Мост Block' },
    @{ Old = 'Міст Aurora'; New = 'Міст Block' },
    @{ Old = 'мосту Aurora'; New = 'мосту Block' },
    @{ Old = 'моста Aurora'; New = 'моста Block' },
    @{ Old = 'мост Aurora'; New = 'мост Block' },
    @{ Old = 'міст Aurora'; New = 'міст Block' },
    @{ Old = 'Aurora-Bridge'; New = 'Block-Bridge' },
    @{ Old = 'bridge di Aurora'; New = 'bridge di Block' },
    @{ Old = 'bridge de Aurora'; New = 'bridge de Block' },
    @{ Old = 'bridge Aurora'; New = 'bridge Block' },
    @{ Old = 'le bridge Aurora'; New = 'le bridge Block' },
    @{ Old = 'la ponte Aurora'; New = 'la ponte Block' },
    @{ Old = 'a ponte do Aurora'; New = 'a ponte do Block' },
    @{ Old = 'A ponte do Aurora'; New = 'A ponte do Block' },
    @{ Old = 'a ponte Aurora'; New = 'a ponte Block' },
    @{ Old = 'ponte do Aurora'; New = 'ponte do Block' },
    @{ Old = 'ponte Aurora'; New = 'ponte Block' },
    @{ Old = 'Usar a ponte Aurora'; New = 'Usar a ponte Block' },
    @{ Old = 'Usar a ponte do Aurora'; New = 'Usar a ponte do Block' },
    @{ Old = 'przez most Aurora'; New = 'przez most Block' },
    @{ Old = 'Napraw most Aurora'; New = 'Napraw most Block' },
    @{ Old = 'Відновити міст Aurora'; New = 'Відновити міст Block' },
    @{ Old = 'через міст Aurora'; New = 'через міст Block' },
    @{ Old = 'Використовувати міст Aurora'; New = 'Використовувати міст Block' },
    @{ Old = 'Настройки → Aurora →'; New = 'Настройки → Block →' }
)

$helpViewerReplacements = @(
    @{ Old = 'Aurora 将不再'; New = 'Block 将不再' },
    @{ Old = 'Aurora no longer'; New = 'Block no longer' },
    @{ Old = 'lager eller tar Aurora'; New = 'lager eller tar Block' },
    @{ Old = 'o Aurora deixa'; New = 'o Block deixa' },
    @{ Old = 'que o Aurora'; New = 'que o Block' },
    @{ Old = 'que Aurora crea'; New = 'que Block crea' },
    @{ Old = 'Aurora crea'; New = 'Block crea' },
    @{ Old = 'Aurora crée'; New = 'Block crée' },
    @{ Old = 'qu''Aurora crée'; New = 'que Block crée' },
    @{ Old = 'Aurora ne le'; New = 'Block ne le' },
    @{ Old = 'Aurora deja'; New = 'Block deja' },
    @{ Old = 'Aurora sama'; New = 'Block sama' },
    @{ Old = 'Aurora создаёт'; New = 'Block создаёт' },
    @{ Old = 'Aurora снимает'; New = 'Block снимает' },
    @{ Old = 'Aurora створює'; New = 'Block створює' },
    @{ Old = 'Aurora більше'; New = 'Block більше' },
    @{ Old = 'den Aurora für'; New = 'den Block für' },
    @{ Old = 'trägt Aurora'; New = 'trägt Block' },
    @{ Old = 'erstellt und trägt Aurora'; New = 'erstellt und trägt Block' },
    @{ Old = "Aurora'nın"; New = "Block'un" },
    @{ Old = 'Aurora onu'; New = 'Block onu' },
    @{ Old = 'Kapalıyken Aurora'; New = 'Kapalıyken Block' }
)

# Viewer makes/wears the bridge (English help).
$lineKeyPrefixes = @(
    'ui-options-aurora-use-bridge',
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
    $text = $text -replace '(bridge-disabled\s*=.*?)› Aurora ›', '$1› Block ›'
    $text = $text -replace '(poser-bridge-disabled\s*=.*?)› Aurora ›', '$1› Block ›'

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
        $line = $line -replace '\bAurora makes\b', 'Block makes'
        $line = $line -replace '\bthat Aurora\b', 'that Block'
        $lines[$i] = $line
    }
    $text = $lines -join "`n"

    if ($text -ne $original) {
        [System.IO.File]::WriteAllText($path, $text, $utf8)
        Write-Host "Updated $($_.Name)"
    }
}