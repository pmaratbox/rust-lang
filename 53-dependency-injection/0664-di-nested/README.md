# 0664 — Nested dependency chain

The `shaku` DI container registers a three-level chain inside `AppModule`: `CImpl` injects `B`, which injects `A`. Resolving the `C` interface with `resolve_ref` makes the container wire the whole graph, so calling `v()` walks `A.v()` -> `B.v()` -> `C.v()` and prints `abc`.

## Run

    cargo run
