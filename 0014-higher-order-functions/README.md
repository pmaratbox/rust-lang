# 0014 — Higher-Order Functions

Write `apply(f, x)` that calls the function `f` on `x`, then pass it two
different functions, `inc` and `double`. The parameter type `fn(i32) -> i32` is
a function pointer. (Closures that capture their environment use the `Fn` trait
family instead, e.g. `impl Fn(i32) -> i32`.)

## Run

    cargo run
