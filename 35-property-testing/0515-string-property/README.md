# 0515 — String property

Uses the `proptest` crate's programmatic `TestRunner` with its `String` regex strategy
(`".*"`) to generate random strings and check the property that `(s + s).len() == 2 * s.len()`.
`TestRunner::run` performs the generation and assertion; `.unwrap()` would panic on any
counterexample, so printing `passed` proves all 100 generated cases held.

## Run

    cargo run
