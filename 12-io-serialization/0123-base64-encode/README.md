# 0123 — Base64 Encode

Base64-encode the bytes of "hi" to get `aGk=`. Slicing the input into 3-byte chunks and indexing a 64-char table keeps the encoder dependency-free.

## Run

    cargo run
