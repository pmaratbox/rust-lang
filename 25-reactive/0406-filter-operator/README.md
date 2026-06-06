# 0406 — Filter Operator

Implement a filter operator that forwards only values passing a predicate, keeping the even numbers of 1..6. In Rust the operator is a closure-based `Observable` whose `subscribe` re-emits a value only when the predicate returns `true`.

## Run

    cargo run
