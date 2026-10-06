@echo off
title RUNLINE - The Last Signal
echo ==========================================================
echo              RUNLINE - THE LAST SIGNAL
echo              Subterranean Rail Transit // Aurelia 2097
echo ==========================================================
echo.
echo Controls:
echo   [A] / [D] / Left / Right Arrow : Switch Lanes
echo   [W] / Space / Up Arrow         : Jump
echo   [S] / Down Arrow               : Slide / Dive
echo   [Shift]                        : Special Courier Dash
echo   [Escape] / [P]                 : Pause
echo.
echo Launching RUNLINE engine...
echo.
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0run.ps1"
if %ERRORLEVEL% NEQ 0 (
    echo.
    echo Running direct cargo launch...
    cargo run --release
)
pause
