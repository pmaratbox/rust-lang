# 0712 — UUIDv5 of another name

Uses rust's `uuid` crate to generate a UUIDv5 (SHA-1, name-based) from the DNS
namespace (`6ba7b810-9dad-11d1-80b4-00c04fd430c8`) and the name `test.com`. v5 is
deterministic: the same (namespace, name) always produces the same UUID, and a
different name (here `test.com` instead of `example.com`) produces a different one.

## Run

    cargo run
