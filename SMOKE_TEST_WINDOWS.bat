@echo off
setlocal
if "%~1"=="" (
  echo Usage: SMOKE_TEST_WINDOWS.bat ^<path-to-yekaterina.exe^>
  exit /b 2
)
python "%~dp0tools\smoke_test.py" "%~1"
exit /b %ERRORLEVEL%
