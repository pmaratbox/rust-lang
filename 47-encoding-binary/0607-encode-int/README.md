# 0607 — Encode an integer

This lesson uses the `rmp-serde` MessagePack library to encode the integer
`42` and print the lowercase hex of the resulting bytes. A small non-negative
integer is stored as a single "positive fixint" byte, so `42` encodes to `2a`.

## Run

    cargo run
