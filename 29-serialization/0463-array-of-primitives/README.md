# 0463 — Array of primitives

Serialize a list of integers to a compact JSON array using `serde_json` (built on the `serde` framework). A `Vec<i32>` implements `serde::Serialize` out of the box, so `serde_json::to_string` turns `vec![1, 2, 3]` into the canonical compact array `[1,2,3]` with no extra whitespace.

## Run

    cargo run
