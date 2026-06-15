# 0562 — MD5

Uses the `md-5` crate (RustCrypto, imported as `md5`) and its `Digest` trait to compute
the MD5 hash of the fixed input `"hello"`, then prints the 16-byte digest as a lowercase
hex string. MD5 is cryptographically broken and should not be used for security purposes,
but it remains common for checksums and interoperability.

## Run

    cargo run
