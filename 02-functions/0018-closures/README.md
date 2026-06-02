# 0018 — Closures

Build a counter that captures a private count starting at zero; each call to the returned function increments the count and returns it, so calling it twice prints 1 then 2. Rust closures implement one of three traits by how they use captures; mutating `count` makes this an `FnMut`, returned as `impl FnMut() -> i32`. The `move` keyword forces the closure to *own* `count` (taking it out of the function's stack frame so it can outlive `counter`), and the binding `next` must be `mut` because calling an `FnMut` mutates its captured state.

## Run

    cargo run
