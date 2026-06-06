# 0405 — Map Operator

Implement a map operator that transforms each emitted value, applying x => x*2 to a stream of 1, 2, 3, 4. In Rust the operator is modeled as a boxed closure producer that forwards `f(value)` to the downstream observer.

## Run

    cargo run
