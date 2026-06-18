# 0574 — TOML array of tables

Use the `toml` crate to parse a document with an array-of-tables
(`[[servers]]`) into a `toml::Value`. Iterate the `servers` array, read each
table's `name` string, and print the names joined with commas as `alpha,beta`.

## Run

    cargo run
