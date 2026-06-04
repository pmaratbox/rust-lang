# 0115 — Dispatch Table

Store functions in a map keyed by name, then apply "add" and "mul" to (3,4), printing `7 12`. A `HashMap<&str, fn(i32, i32) -> i32>` holds the function pointers that we look up and call by name.

## Run

    cargo run
