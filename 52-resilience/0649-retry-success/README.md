# 0649 — Succeeds first try

Uses the `retry` crate's `retry` driver with a zero-delay `Fixed` policy
(`.take(4)`, so up to four retries are allowed). The scripted operation returns
`Ok` immediately, so the library never retries — a shared `Cell` counter,
bumped on each invocation, ends at `1`. No retry needed.

## Run

    cargo run
