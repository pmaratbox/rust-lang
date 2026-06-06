# 0420 — Retry On Error

Implement retry(n) that resubscribes to the source on error up to n times; the source succeeds on the 3rd subscription. An `Observer` of boxed `FnMut` closures and a `RefCell` subscription counter keep the resubscribe loop synchronous and deterministic.

## Run

    cargo run
