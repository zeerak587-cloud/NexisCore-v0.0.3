@echo off
title NexisCore Run

echo ==============================
echo Starting NexisCore...
echo ==============================

if not exist "target\release\nexiscore.exe" (
    echo [INFO] No build found. Building now...
    cargo build --release
)

if exist "target\release\nexiscore.exe" (
    echo.
    echo [OK] Launching NexisCore...
    echo.
    target\release\nexiscore.exe
) else (
    echo.
    echo [ERROR] Could not find executable.
)

pause
