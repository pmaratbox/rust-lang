# 0612 — Encode a map

Uses the `rmp-serde` MessagePack library to encode the single-key map
`{"a": 1}` (a `BTreeMap` for deterministic key order) and prints the
lowercase hex of the bytes: `81a16101` (fixmap `81` + key `a161` + value `01`).

## Run

    cargo run
