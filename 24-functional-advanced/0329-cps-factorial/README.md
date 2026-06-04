# 0329 — CPS Factorial

Compute 5! in continuation-passing style, printing `120`. Each recursive step threads a boxed continuation closure, and the top-level call passes the identity continuation.

## Run

    cargo run
