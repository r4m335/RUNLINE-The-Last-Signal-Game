$Host.UI.RawUI.WindowTitle = "RUNLINE: The Last Signal"
$env:Path = "$env:USERPROFILE\.cargo\bin;" + $env:Path

Write-Host "==========================================================" -ForegroundColor Cyan
Write-Host "             RUNLINE - THE LAST SIGNAL                   " -ForegroundColor Cyan
Write-Host "             Subterranean Rail Transit // Aurelia 2097    " -ForegroundColor DarkCyan
Write-Host "==========================================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "Controls:" -ForegroundColor Yellow
Write-Host "  [A] / [D] / Left / Right Arrow : Switch Lanes" -ForegroundColor White
Write-Host "  [W] / Space / Up Arrow         : Jump" -ForegroundColor White
Write-Host "  [S] / Down Arrow               : Slide / Dive" -ForegroundColor White
Write-Host "  [Shift]                        : Special Courier Dash" -ForegroundColor White
Write-Host "  [Escape] / [P]                 : Pause" -ForegroundColor White
Write-Host ""

if (Test-Path "scripts\generate_audio.py") {
    if (Get-Command python -ErrorAction SilentlyContinue) {
        Write-Host "Checking audio assets..." -ForegroundColor Gray
        try {
            python scripts\generate_audio.py | Out-Null
        } catch {}
    }
}

Write-Host "Launching RUNLINE engine..." -ForegroundColor Green
Write-Host ""

cargo run --release
if ($LASTEXITCODE -ne 0) {
    Write-Host "Retrying in debug mode..." -ForegroundColor Yellow
    cargo run
}
