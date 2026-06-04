# 0110 — Currying

Curry a two-argument add into a chain of one-argument functions and call it as `add(2)(3)`, printing `5`. `add` returns an `impl Fn(i32) -> i32` closure that captures the first argument with `move`.

## Run

    cargo run
