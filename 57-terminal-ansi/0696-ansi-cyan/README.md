# 0696 — Cyan text

Uses Rust's `owo-colors` crate (the `OwoColorize` extension trait) to color the word `cyan` with the foreground CYAN color (ANSI 36) via the `.cyan()` method. owo-colors always emits the raw ANSI escape sequence — `\x1b[36mcyan\x1b[39m`, ending with the foreground-reset code `\x1b[39m` — without needing a TTY.

## Run

    cargo run
