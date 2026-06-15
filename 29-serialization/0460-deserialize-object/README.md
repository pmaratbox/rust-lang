# 0460 — Deserialize an object

Parse the JSON string `{"age":30,"name":"alice"}` into a typed `Person` struct using the `serde` framework with `serde_json`. The `#[derive(Deserialize)]` macro generates the parser, and `serde_json::from_str` turns the text into the struct so we can print `name age`.

## Run

    cargo run
