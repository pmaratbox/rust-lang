# 0037 — Operator Overloading

Define how `+` (or an `add` method) combines two points, then add `(1, 2)` and `(3, 4)` and print `(4, 6)`. Operators are traits: implementing `std::ops::Add` (with an associated `Output` type) makes `+` work on `Point`. Each operator has its own trait — `Sub`, `Mul`, `PartialEq`, and so on.

## Run

    cargo run
