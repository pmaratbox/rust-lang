# 0465 — Round trip

Serialize a typed `Person` to JSON with serde (`serde_json::to_string` via the `derive` `Serialize` feature), then deserialize the same string back into a `Person` with `serde_json::from_str` (the `derive` `Deserialize` feature). Fields are declared alphabetically (`age`, `name`) so the compact JSON keeps keys in alphabetical order. The round trip prints the name.

## Run

    cargo run
