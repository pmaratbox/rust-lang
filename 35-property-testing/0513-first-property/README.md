# 0513 — First property

Uses the `proptest` crate driven programmatically through `TestRunner`: the `proptest::collection::vec(any::<i32>(), 0..10)` strategy generates ~100 random integer lists, and `runner.run` checks the property that reversing a list twice yields the original (`prop_assert_eq!`). Every generated case holds, so `run(..).unwrap()` succeeds and prints `passed`.

## Run

    cargo run
