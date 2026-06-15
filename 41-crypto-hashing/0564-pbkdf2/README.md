# 0564 — PBKDF2

Uses the `pbkdf2` crate's `pbkdf2_hmac::<Sha256>` function (with the `sha2` crate
providing SHA-256) to derive a 32-byte key from the password `password` and salt
`salt` using 1000 iterations of PBKDF2-HMAC-SHA256. The derived key is printed as
lowercase hex.

## Run

    cargo run
