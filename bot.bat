@ECHO off
::Rust Compile Script
::Change directory
CD E:\Projects\woprok_discord_bot
E:
::Compile and run
SET isRelease=%1%
IF DEFINED isRelease (
ECHO DEFINED
cargo build --release
E:\Projects\woprok_discord_bot\target\release\woprok_discord_bot.exe
) ELSE (
ECHO NOTDEFINED
cargo build
E:\Projects\woprok_discord_bot\target\debug\woprok_discord_bot.exe
)
PAUSE