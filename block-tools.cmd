@echo off
setlocal DisableDelayedExpansion
rem Block Tools entry point: keyboard menus, PR tracking and release management.
rem Load scripts\tools\block-tools.ps1 from the repository or the project folder;
rem those scripts reopen the menu automatically after a tools update.
rem Alone in a folder, download the installer and install the project there.
rem (goto rather than ( ) blocks: a path with a parenthesis would break a block)
title Block Tools
cd /d "%~dp0"
if exist "%~dp0scripts\tools\block-tools.ps1" goto repo
if exist "%~dp0block-viewer\scripts\tools\block-tools.ps1" goto project
powershell -NoProfile -ExecutionPolicy Bypass -Command "$f = Join-Path $env:TEMP 'block-setup.ps1'; irm https://raw.githubusercontent.com/ape-squared/Block-Viewer/main/scripts/setup.ps1 -OutFile $f; & $f"
set "BlockToolsExitCode=%errorlevel%"
pause
exit /b %BlockToolsExitCode%
:repo
set "BlockToolsScript=%~dp0scripts\tools\block-tools.ps1"
goto launch
:project
set "BlockToolsScript=%~dp0block-viewer\scripts\tools\block-tools.ps1"
:launch
powershell -NoProfile -ExecutionPolicy Bypass -File "%BlockToolsScript%" %*
exit /b %errorlevel%
