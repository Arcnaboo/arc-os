@echo off
set "ARGS=%*"
:: Remove the problematic flag
set "ARGS=%ARGS:-Zjson-target-spec=%"
"C:\Users\Arda\.rustup\toolchains\nightly-2026-05-01-x86_64-pc-windows-msvc\bin\rustc.exe" %ARGS%
