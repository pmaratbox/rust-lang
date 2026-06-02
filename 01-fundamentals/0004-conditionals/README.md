# 0004 — Conditionals

Compare `n` against 10 and print whether it's less, equal, or greater. Rust's
`if`/`else` is an **expression** — it has a value, so
`let s = if n < 10 { "small" } else { "big" };` is idiomatic. No parens
around the condition; braces required. Edit `n` to `10` or `15` to exercise
the other branches.

## Run

    cargo run
