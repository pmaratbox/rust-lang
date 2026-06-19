# 0610 — Encode an array

This lesson uses the `rmp-serde` MessagePack library to encode the array
`[1, 2, 3]` and print the lowercase hex of the resulting bytes. A short array
is stored with a single "fixarray" header byte (`93` = array of length 3)
followed by each element, so `[1, 2, 3]` encodes to `93010203`.

## Run

    cargo run
