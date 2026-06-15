# 0563 — HMAC-SHA256

Compute an HMAC-SHA256 authentication tag over the fixed message `hello`
using the key `key`. This uses the `hmac` crate's generic `Hmac` type
parameterized with the `sha2` crate's `Sha256` hasher (via the `Mac`
trait). The 32-byte tag is formatted as a lowercase hexadecimal string
and printed.

## Run

    cargo run
