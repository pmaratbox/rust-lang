# 0043 — Function Composition

Compose `inc` (add one) and `twice` (multiply by two) into one function and apply it to `3`, so `inc(twice(3))` prints `7`. `compose` takes two `fn` pointers and returns `impl Fn(i32) -> i32` — an anonymous closure (`move |x| f(g(x))`) that owns the captured functions. The `impl Trait` return hides the concrete closure type.

## Run

    cargo run
