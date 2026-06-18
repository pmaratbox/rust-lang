# 0569 — YAML sequence

Use the `serde_yaml` crate to parse a YAML document whose `fruits` key holds a
sequence (list) of strings. The parsed `serde_yaml::Value` sequence is iterated,
each item extracted as a string, and the items are joined with commas to print
`apple,banana,cherry`.

## Run

    cargo run
