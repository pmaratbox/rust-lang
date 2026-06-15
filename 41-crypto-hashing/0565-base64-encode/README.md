# 0565 — Base64 encode

Uses the `base64` crate's `Engine` trait with the standard alphabet engine
(`general_purpose::STANDARD`) to Base64-encode the UTF-8 bytes of `hello`,
printing the resulting Base64 string.

## Run

    cargo run
