# 0112 — Predicate Combinators

Combine predicates with AND/OR/NOT: test `isEven AND isPositive` on 4 (yes) and -4 (no), printing `yes no`. The `and` combinator takes two `Fn(i32) -> bool` closures and returns their conjunction as a new closure.

## Run

    cargo run
