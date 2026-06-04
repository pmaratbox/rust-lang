# 0331 — Transducer Pipeline

Compose map(+1) with filter(even) and run it over [1,2,3,4], printing `2 4`. Chained iterator adapters `.map(..).filter(..)` fuse into a single lazy pass with no intermediate collection.

## Run

    cargo run
