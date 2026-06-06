# 0401 — Create an Observable

Build a push-based Observable from scratch that emits 1, 2, 3 to its observer and then completes. In Rust the observer's `next`/`complete` are boxed closures, and the Observable is a function wrapping a `subscribe` callback.

## Run

    cargo run
