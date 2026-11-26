@echo off
rem Build the project into a fresh directory.
set BUILD_DIR="build"
set CONFIG="release"
set /A RETRIES=3
goto :clean

:clean
rd %BUILD_DIR%
goto :compile

:compile
md %BUILD_DIR%
echo "Built %CONFIG% after %RETRIES% retries"
goto :eof
