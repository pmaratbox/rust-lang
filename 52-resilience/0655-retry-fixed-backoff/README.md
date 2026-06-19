# 0655 — Fixed backoff

Uses the `retry` crate's `retry` driver with a `Fixed` backoff strategy
(a constant, zero-millisecond delay between attempts via `Fixed::from_millis(0)`,
capped at `.take(4)` retries). A scripted operation fails twice and then succeeds
on its third invocation; a shared `Cell` counter, bumped on each call, ends at
`3` — the total number of attempts the library actually made.

## Run

    cargo run
