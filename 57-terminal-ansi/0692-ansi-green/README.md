# 0692 — Green text

Uses Rust's `owo-colors` crate and its `.green()` adaptor to color the word `green` with the foreground GREEN color (ANSI 32). The adaptor always emits the raw escape sequence without needing a TTY, producing `ESC[32mgreenESC[39m` where the foreground is closed with the reset code `\x1b[39m`.

## Run

    cargo run
