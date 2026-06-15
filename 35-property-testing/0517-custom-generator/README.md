# 0517 — Custom generator

Uses the `proptest` crate driven programmatically through `TestRunner`: a custom strategy is built by chaining the `.prop_map` combinator onto `any::<i32>()` (`n -> n * 2`) so it generates only even integers. `runner.run` then checks the property that every generated value is even (`prop_assert_eq!(n % 2, 0)`). All ~100 generated cases hold, so `run(..).unwrap()` succeeds and prints `passed`.

## Run

    cargo run
