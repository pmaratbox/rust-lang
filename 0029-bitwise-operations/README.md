# 0029 — Bitwise Operations

Compute bitwise AND, OR, and XOR on `6` and `3`, plus a left shift of `6` by one bit, printing `and: 2`, `or: 7`, `xor: 5`, and `shift: 12`. Rust's bitwise operators `&`, `|`, `^`, and `<<` apply to integer types, and `!` is bitwise NOT. A shift past the type's width is a compile error or panics, but `6 << 1` is well within range, giving 12.

## Run

    cargo run
