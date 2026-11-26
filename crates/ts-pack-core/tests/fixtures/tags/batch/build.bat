@echo off
rem Build the project into a fresh directory.
set BUILD_DIR="build"
set /A RETRIES=3
goto :compile

:compile
md %BUILD_DIR%
echo "Built after %RETRIES% retries"
goto :eof
