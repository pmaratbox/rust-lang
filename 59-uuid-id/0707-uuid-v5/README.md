# 0707 — UUIDv5 (name-based)

Uses the `uuid` crate's `Uuid::new_v5` to generate a name-based UUID
(version 5) from the DNS namespace
(`6ba7b810-9dad-11d1-80b4-00c04fd430c8`) and the name `example.com`.
Unlike random v4, v5 hashes the namespace and name with SHA-1, so the
same inputs always yield the same UUID — it is fully deterministic and
never hardcoded here.

## Run

    cargo run
