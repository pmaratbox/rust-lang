# 0698 — ANSI Black

Uses Rust's `owo-colors` crate and its `OwoColorize::black` method to wrap the word `black` in the standard foreground BLACK ANSI color. owo-colors always emits the raw escape sequence regardless of whether stdout is a TTY, producing `\x1b[30m` before the text and the foreground reset `\x1b[39m` after it.

## Run

    cargo run
