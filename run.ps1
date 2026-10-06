$Host.UI.RawUI.WindowTitle = "RUNLINE: The Last Signal"
$env:Path = "$env:USERPROFILE\.cargo\bin;" + $env:Path

Write-Host "==========================================================" -ForegroundColor Cyan
Write-Host "             RUNLINE — THE LAST SIGNAL                   " -ForegroundColor Cyan
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
Write-Host "Checking audio assets..." -ForegroundColor Gray
python scripts/generate_audio.py | Out-Null

Write-Host "Launching RUNLINE engine..." -ForegroundColor Green

# Run cargo with auto-retry on Windows file-sharing locks
$maxRetries = 15
for ($attempt = 1; $attempt -le $maxRetries; $attempt++) {
    & cargo run
    if ($LASTEXITCODE -eq 0) {
        break
    }
    Write-Host "Resuming build pipeline (attempt $attempt)..." -ForegroundColor DarkYellow
    Start-Sleep -Seconds 1
}
