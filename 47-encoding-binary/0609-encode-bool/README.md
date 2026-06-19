# 0609 — Encode a boolean

This lesson uses the `rmp-serde` MessagePack library to encode the boolean
`true` and print the lowercase hex of the resulting bytes. MessagePack reserves
dedicated single-byte tags for booleans, so `true` encodes to `c3`.

## Run

    cargo run
