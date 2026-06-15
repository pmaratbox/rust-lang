# 0466 — Parse & access nested

Parse a JSON string with `serde_json` into its dynamic `Value` tree, then index into the nested object and array (`user.name` and `roles[0]`) using serde_json's untyped tree API instead of typed structs.

## Run

    cargo run
