# 0044 — Generic Constraints

Write a generic `largest(a, b)` that requires an ordered type, then call it on integers (3 and 9) and on strings (apple and pear), printing `9` and `pear`. The trait bound `T: PartialOrd` is what permits `a > b`; without it the compiler rejects the comparison. Bounds are monomorphized, so each concrete type gets its own specialized `largest`.

## Run

    cargo run
