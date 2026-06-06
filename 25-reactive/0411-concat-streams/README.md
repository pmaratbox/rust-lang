# 0411 — Concat Streams

Implement concat: subscribe to the second source only after the first completes; concat [1,2] then [3,4]. Rust models observers as boxed `FnMut` closures, so concat just re-subscribes to the second source from the first's complete callback.

## Run

    cargo run
