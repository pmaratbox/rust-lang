# 0570 — Dump YAML

Use the `serde_yaml` crate to serialize a fixed map (`name=Alice`,
`age=30`, `city=Paris`) into sorted, block-style YAML. Storing the entries
in a `BTreeMap` sorts the keys, and `serde_yaml::to_string` emits plain
block scalars with no flow braces or quotes.

## Run

    cargo run
