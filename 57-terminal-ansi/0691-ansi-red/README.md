# 0691 — Red text

Uses Rust's `owo-colors` crate and its `OwoColorize::red` method to wrap the word `red` in the foreground RED ANSI color (code 31). The `.red()` extension method emits the raw escape sequence unconditionally — there is no TTY detection — so the color is effectively forced on, and the trailing reset is the foreground-default `\x1b[39m` rather than the full `\x1b[0m` reset.

## Run

    cargo run
