@echo off
setlocal

set EXE_PATH=%~dp0target\release\nexiscore.exe

if not exist "%EXE_PATH%" (
    echo NexisCore is not built yet.
    echo Run build.bat first.
    pause
    exit /b
)

echo Starting NexisCore...
echo.

"%EXE_PATH%"

pause
