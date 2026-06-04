# 0122 — UTF-8 Bytes

Print the UTF-8 byte values of "Hi": `72 105`. A `&str` is already UTF-8, so `as_bytes()` exposes the raw byte slice directly.

## Run

    cargo run
