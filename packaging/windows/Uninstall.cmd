@echo off
setlocal
REM Double-clickable uninstaller for the portable Block Viewer folder.
cd /d "%~dp0"
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0Uninstall.ps1" %*
set ERR=%ERRORLEVEL%
if not "%ERR%"=="0" pause
exit /b %ERR%
