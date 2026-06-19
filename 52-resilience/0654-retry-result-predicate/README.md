# 0654 — Retry on result

Uses the `retry` crate's `retry` driver with a zero-delay `Fixed` policy
(`.take(4)`, so up to four retries are allowed). This is *retry-on-result*: the
scripted closure returns an incrementing `Cell` counter and reports `Err` while
the value is below `3`, forcing the library to retry. The first acceptable value
(`>= 3`) is returned as `Ok` on the third attempt, and that accepted result —
`3` — is printed.

## Run

    cargo run
