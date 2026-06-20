# 0697 — White text

Uses Rust's `owo-colors` crate (the `OwoColorize` extension trait) to color the word `white` with the foreground WHITE color (ANSI 37) via the `.white()` method. owo-colors always emits the raw ANSI escape sequence — `\x1b[37mwhite\x1b[39m`, ending with the foreground-reset code `\x1b[39m` — without needing a TTY.

## Run

    cargo run
