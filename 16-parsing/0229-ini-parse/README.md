# 0229 — Parse INI

Parse the INI text with section [s] and key k=v, printing the flattened entry `s.k=v`. Rust uses `strip_prefix`/`strip_suffix` for the section header and `split_once` for each key=value.

## Run

    cargo run
