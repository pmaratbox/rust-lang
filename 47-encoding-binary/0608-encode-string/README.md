# 0608 — Encode a string

This lesson uses the `rmp-serde` MessagePack library to encode the string
`"hello"` and print the lowercase hex of the resulting bytes. A short string is
stored as a "fixstr" header byte (`0xa5` for length 5) followed by the UTF-8
bytes, so `"hello"` encodes to `a568656c6c6f`.

## Run

    cargo run
