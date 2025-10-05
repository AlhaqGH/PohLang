@echo off
REM PohLang v0.5.0 - Quick Test Script

echo ========================================
echo PohLang v0.5.0 Quick Test
echo ========================================
echo.

echo Checking PohLang version...
pohlang.exe --version
echo.

echo ========================================
echo Running Example Programs
echo ========================================
echo.

echo 1. Hello World Example:
echo ------------------------
pohlang.exe --run examples\poh\hello.poh
echo.

echo 2. Math Functions Example:
echo ---------------------------
pohlang.exe --run examples\poh\math_functions.poh
echo.

echo 3. String Functions Example:
echo -----------------------------
pohlang.exe --run examples\poh\string_functions.poh
echo.

echo 4. Collection Functions Example:
echo ---------------------------------
pohlang.exe --run examples\poh\collection_functions.poh
echo.

echo ========================================
echo All tests complete!
echo ========================================
echo.
echo To run your own programs:
echo   pohlang.exe --run your_script.poh
echo.
echo For help:
echo   pohlang.exe --help
echo.
echo Documentation: See QUICK_START.md
echo.

pause
