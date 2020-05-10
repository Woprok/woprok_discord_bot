@ECHO off
::Rust Compile Script
::Change directory
CD E:\Projects\woprok_discord_bot
E:
::Update cargo dependencies with:
cargo update
::Create and open documentation with:
cargo doc --open 
PAUSE