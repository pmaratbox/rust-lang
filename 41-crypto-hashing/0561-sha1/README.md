# 0561 — SHA-1

Uses the `sha1` crate (RustCrypto) and its `Digest` trait to compute the SHA-1 hash
of the fixed input `"hello"`, then prints the 20-byte digest as a lowercase hex string.
SHA-1 is a legacy algorithm and should not be used for new security work, but it remains
useful for checksums and interoperability.

## Run

    cargo run
