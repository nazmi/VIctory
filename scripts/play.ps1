# Normal-play launcher, adapted from Zettrox/ztrx (90b529f8).
param(
    [Parameter(Mandatory = $true)][string]$Game,
    [string]$Cache = "$env:LOCALAPPDATA\victory\cache",
    [string]$Data = "$env:LOCALAPPDATA\victory\data",
    [string]$Exe = "$PSScriptRoot\..\target\release\victory.exe"
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
$log = Join-Path ([System.IO.Path]::GetTempPath()) 'victory-play.log'
# Native stderr is diagnostic text; preserve the executable's actual exit status.
$ErrorActionPreference = 'Continue'
& $Exe run $Game --execute 2>&1 | Out-File -LiteralPath $log -Encoding utf8
$exitCode = $LASTEXITCODE
Write-Host "Exited with $exitCode; log: $log"
if ($null -eq $exitCode) { exit 1 }
exit $exitCode
