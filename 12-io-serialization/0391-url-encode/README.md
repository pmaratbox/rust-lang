# 0391 — URL Percent-Encode

Percent-encode the string "a b&c" to `a%20b%26c`. Rust matches each byte against the unreserved set and formats the rest with `{:02X}` uppercase hex.

## Run

    cargo run
