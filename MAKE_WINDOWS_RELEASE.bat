@echo off
setlocal
if "%~2"=="" (
  echo Usage: MAKE_WINDOWS_RELEASE.bat ^<path-to-yekaterina.exe^> ^<path-to-private-cargo-project^>
  exit /b 2
)
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0tools\MAKE_WINDOWS_RELEASE.ps1" -ExePath "%~1" -CargoProjectPath "%~2"
exit /b %ERRORLEVEL%
