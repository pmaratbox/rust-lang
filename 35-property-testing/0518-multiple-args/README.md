# 0518 — Multiple arguments

Uses the `proptest` crate driven programmatically through `TestRunner`: a tuple strategy `(any::<i32>(), any::<i32>())` generates TWO integer arguments per case, and `runner.run` checks the property that `max(a, b) >= a` and `max(a, b) >= b` (`prop_assert!`). Every generated pair holds, so `run(..).unwrap()` succeeds and prints `passed`.

## Run

    cargo run
