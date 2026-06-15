# 0468 — Scalar types

Serialize a struct holding the three JSON scalar kinds — `bool`, `int`, and `string` — to compact JSON using the `serde` framework with its `derive` feature plus `serde_json`. The fields are declared alphabetically (`active`, `count`, `label`), so `serde_json::to_string` emits keys in that order, and the boolean is rendered as the lowercase literal `true`.

## Run

    cargo run
