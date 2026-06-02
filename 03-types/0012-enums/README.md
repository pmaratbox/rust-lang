# 0012 — Enums

Define a `Color` enum (`Red`, `Green`, `Blue`) and print the integer value of
`Green` (1) and `Blue` (2). A field-less enum can be cast to an integer with
`as i32`; discriminants default to `0, 1, 2`. `#[allow(dead_code)]` silences the
unused-variant warning since this example only reads two of them.

## Run

    cargo run
