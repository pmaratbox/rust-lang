# 0573 — TOML array

Use the `toml` crate to parse the fixed TOML document
`tags = ["red", "green", "blue"]\n` into a `toml::Value`. The `tags` array is
read with `as_array`, each element converted with `as_str`, and the values are
joined with commas to print `red,green,blue`.

## Run

    cargo run
