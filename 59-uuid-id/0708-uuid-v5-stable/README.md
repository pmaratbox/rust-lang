# 0708 — UUIDv5 is stable

Uses the `uuid` crate's `Uuid::new_v5` to generate a name-based (SHA-1)
UUID from the DNS namespace and the name `example.com` twice. Because
UUIDv5 is deterministic over `(namespace, name)` — unlike the random
v4 — the two results are identical, so comparing them prints `true`.

## Run

    cargo run
