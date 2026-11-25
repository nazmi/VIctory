# Normal-play launcher, adapted from Zettrox/ztrx (90b529f8).
param(
    [Parameter(Mandatory = $true)][string]$Game,
    [string]$Cache = "$env:LOCALAPPDATA\victory\cache",
    [string]$Data = "$env:LOCALAPPDATA\victory\data",
    [string]$Exe = "$PSScriptRoot\..\target\release\victory.exe",
    [switch]$SmallGpu
)

$ErrorActionPreference = 'Stop'
if (-not (Test-Path -LiteralPath $Exe -PathType Leaf)) {
    throw "VIctory executable not found: $Exe. Build with cargo build --release -p victory."
}
foreach ($setting in 'VICTORY_KEYS', 'VICTORY_CAPTURE', 'VICTORY_CAPTURE_FRAMES',
    'VICTORY_CAPTURE_WORLD', 'VICTORY_CAPTURE_DRAWS', 'VICTORY_DUMP_TARGETS',
    'VICTORY_DUMP_WRITERS', 'VICTORY_SHOW_TARGET') {
    Remove-Item -LiteralPath "Env:$setting" -ErrorAction SilentlyContinue
}
[System.IO.Directory]::CreateDirectory($Cache) | Out-Null
[System.IO.Directory]::CreateDirectory($Data) | Out-Null
$env:VICTORY_CACHE_DIR = $Cache
$env:VICTORY_DATA_DIR = $Data
$runArgs = @('run', $Game, '--execute')
if ($SmallGpu) {
    # The title selects its smaller render configuration from the console
    # model. The reported display mode alone does not change that choice.
    $runArgs += @('--console', 'series-s')
    $env:VICTORY_MODE = '1280x720'
    Write-Host 'Small GPU: Series S profile, reported display 1280x720.'
}
$log = Join-Path ([System.IO.Path]::GetTempPath()) 'victory-play.log'
# Native stderr is diagnostic text; preserve the executable's actual exit status.
$ErrorActionPreference = 'Continue'
& $Exe @runArgs 2>&1 | Out-File -LiteralPath $log -Encoding utf8
$exitCode = $LASTEXITCODE
Write-Host "Exited with $exitCode; log: $log"
if ($null -eq $exitCode) { exit 1 }
exit $exitCode
