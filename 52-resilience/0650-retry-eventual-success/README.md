# 0650 — Eventual success

Uses the `retry` crate's `retry` driver with a zero-delay `Fixed` schedule
(`.take(4)`, allowing up to four retries). A scripted operation backed by a
shared `Cell` counter returns `Err` on its first attempt and `Ok` thereafter,
so the library retries once and then stops on the success. The counter shows
the total number of attempts the library actually made, so the program prints
`2`.

## Run

    cargo run
