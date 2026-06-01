# 0019 — Recursion

Define a recursive `factorial(n)` that multiplies `n` by `factorial(n - 1)` until it bottoms out at `1`, then print `factorial(5) = 120`. The `if`/`else` arms are expressions, so the body needs no `return`. Rust does not guarantee tail-call optimization, so deep recursion can overflow the stack; `u64` is used because `factorial` overflows a 32-bit integer by `13!`.

## Run

    cargo run
