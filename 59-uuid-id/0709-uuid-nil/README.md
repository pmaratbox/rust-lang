# 0709 — Nil UUID

Uses the `uuid` crate. `Uuid::nil()` returns the special all-zero UUID,
printed here in its canonical lowercase form. It is the conventional
"absent" or placeholder identifier and is byte-for-byte deterministic.

## Run

    cargo run
