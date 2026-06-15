# 0566 — Incremental hashing

Use the `sha2` crate (RustCrypto) to compute a SHA-256 digest
incrementally. Create the hasher, feed it the data in two separate
`update` calls (`"foo"` then `"bar"`) via the `Digest` trait, then
`finalize`. The result equals the SHA-256 of `"foobar"`, printed as a
lowercase hex string (no colons or spaces).

## Run

    cargo run
