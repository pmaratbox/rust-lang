# 0040 — Generators & Lazy Sequences

Produce an endless lazy sequence of squares and take only the first three, printing `1 4 9`. Iterators are lazy by construction: `(1..)` is an infinite range, `map` squares lazily, and `take(3)` bounds it — nothing runs until `collect` pulls values. No `yield` keyword is needed.

## Run

    cargo run
