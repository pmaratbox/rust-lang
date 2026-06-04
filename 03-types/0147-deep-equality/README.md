# 0147 — Deep Equality

Compare two nested structures with equal contents for structural equality and print `equal: yes`. Deriving `PartialEq` gives Rust a field-by-field structural comparison that recurses through nested structs.

## Run

    cargo run
