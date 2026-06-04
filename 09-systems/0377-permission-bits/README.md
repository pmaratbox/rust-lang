# 0377 — Permission Bits

Decode the Unix permission bits 0b101 into the rwx string `r-x`. Rust tests each bit with the `&` bitwise-and operator against `0b100`, `0b010`, and `0b001`.

## Run

    cargo run
