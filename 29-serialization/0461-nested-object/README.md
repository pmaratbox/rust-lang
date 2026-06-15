# 0461 — Nested object

Serialize a `Person` struct that contains a nested `Address` struct to compact JSON using the `serde` framework with its `derive` feature, plus `serde_json` for the JSON format. Each struct derives `Serialize`, and `serde_json::to_string` emits compact JSON; declaring fields in alphabetical order makes the output keys alphabetical without any extra configuration.

## Run

    cargo run
