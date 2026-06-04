# 0325 — Point-Free Style

Express "sum of squares" point-free (compose map-square with sum) and apply it to [1,2,3], printing `14`. Rust composes the pipeline with iterator adapters `.map(..).sum()` rather than naming intermediate values.

## Run

    cargo run
