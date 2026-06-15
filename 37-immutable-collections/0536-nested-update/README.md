# 0536 — Nested update

Build a persistent map `{ "user": { "age": 30 } }` with the `im` crate's `HashMap`, then update the nested `user.age` to 31. `HashMap::update` returns a brand-new map (structural sharing via clone) for the inner map and again for the outer map, so the original is never mutated. Printing the updated nested age (`31`) and the original nested age (`30`) shows the immutability.

## Run

    cargo run
