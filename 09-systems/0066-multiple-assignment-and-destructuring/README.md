# 0066 — Multiple Assignment & Destructuring

Swap two variables (`a = 1`, `b = 2`) with a single multiple-assignment, then unpack the pair `(3, 4)` into two variables — printing `2 1` then `3 4`. Destructuring assignment `(a, b) = (b, a)` (stable since Rust 1.59) swaps in place, and `let (x, y) = (3, 4)` binds a tuple's fields.

## Run

    cargo run
