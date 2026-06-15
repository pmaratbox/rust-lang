# 0459 — Serialize object

Use `serde` with its `derive` feature and the `serde_json` crate to serialize a struct into JSON. A `Person { age, name }` struct derives `Serialize`, and `serde_json::to_string` produces compact JSON. Fields are declared alphabetically so the keys come out in alphabetical order (`age` before `name`).

## Run

    cargo run
