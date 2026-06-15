# 0559 — SHA-256

Use the `sha2` crate (RustCrypto) to compute the SHA-256 digest of the
UTF-8 bytes of `"hello"`. Feed the bytes through the `Digest` trait's
`update`/`finalize` API and print the 32-byte digest as a lowercase hex
string (no colons or spaces).

## Run

    cargo run
