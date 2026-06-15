# 0462 — Array of objects

Serialize a `Vec<Person>` to a compact JSON array using the `serde` framework with its `derive` feature and the `serde_json` crate. Each `Person` derives `Serialize`, and `serde_json::to_string` produces compact JSON; fields are declared alphabetically (`age`, then `name`) so the emitted keys come out in alphabetical order.

## Run

    cargo run
