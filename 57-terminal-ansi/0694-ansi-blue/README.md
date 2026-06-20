# 0694 — Blue text

Uses Rust's `owo-colors` crate and its `OwoColorize::blue` method to color the word `blue` with the standard foreground BLUE color (ANSI 34). `owo-colors` always produces the raw escape sequence — `ESC[34m` before the text and the foreground reset `ESC[39m` after — without needing a TTY.

## Run

    cargo run
