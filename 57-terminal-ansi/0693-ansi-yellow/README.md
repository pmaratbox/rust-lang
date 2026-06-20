# 0693 — Yellow text

Uses Rust's `owo-colors` crate and its `OwoColorize::yellow` method to color the word `yellow` with the foreground YELLOW color (ANSI 33). The method unconditionally emits the raw escape sequence — `\x1b[33m`, the text, then the foreground reset `\x1b[39m` — so the ANSI codes appear even without a TTY.

## Run

    cargo run
