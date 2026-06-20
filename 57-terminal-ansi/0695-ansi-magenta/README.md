# 0695 — Magenta text

Uses the [`owo-colors`](https://crates.io/crates/owo-colors) crate to color the
word `magenta` with the foreground MAGENTA color (ANSI 35). The
`OwoColorize::magenta()` extension method wraps the string in a value whose
`Display` impl unconditionally emits the raw escape sequence
`\x1b[35mmagenta\x1b[39m` — no TTY required, and the foreground reset is
`\x1b[39m` (not `\x1b[0m`). The escape bytes come entirely from the library.

## Run

    cargo run
