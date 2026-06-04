# 0328 — Trampoline

Sum 1..100 with a trampolined recursion that avoids deep stacks, printing `5050`. Each step returns a boxed thunk (`Bounce::More`) that a driver loop bounces until it sees `Bounce::Done`, so the stack never grows.

## Run

    cargo run
