@echo off
setlocal enableDelayedExpansion

echo current path: !cd!
cd !cd!

call :cargo_test
call :cargo_deny

endlocal

exit /b

:cargo_test
echo $ cargo test --verbose --features tomls -- --show-output
call cargo test --verbose --features tomls -- --show-output

:cargo_deny
echo $ cargo deny check bans licenses sources
call cargo deny check bans licenses sources
