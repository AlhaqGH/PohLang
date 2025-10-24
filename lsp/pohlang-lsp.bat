@echo off
REM PohLang LSP Server Launcher for Windows

SET SCRIPT_DIR=%~dp0
SET SERVER_PATH=%SCRIPT_DIR%out\server.js

REM Check if node is installed
WHERE node >nul 2>nul
IF %ERRORLEVEL% NEQ 0 (
    echo Error: node command not found 1>&2
    echo Please install Node.js first: https://nodejs.org 1>&2
    exit /b 1
)

REM Launch the LSP server
node "%SERVER_PATH%" %*
