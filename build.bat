@echo off
title NexisCore Build

echo ==============================
echo Building NexisCore...
echo ==============================

cargo build --release

if %errorlevel% neq 0 (
    echo.
    echo [ERROR] Build failed.
    pause
    exit /b
)

echo.
echo [OK] Build complete.
echo Location: target\release\nexiscore.exe
echo.

pause
