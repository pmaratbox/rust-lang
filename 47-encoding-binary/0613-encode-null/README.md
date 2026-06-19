# 0613 — Encode null

This lesson uses the `rmp-serde` MessagePack library to encode `null`/`nil`
(represented in Rust as `None`) and print the lowercase hex of the resulting
bytes. MessagePack stores nil as the single byte `c0`.

## Run

    cargo run
