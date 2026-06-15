# 0520 — Finds a counterexample

Uses the `proptest` crate driven programmatically through `TestRunner`: the `0i32..1_000_000` strategy generates random non-negative integers and `runner.run` checks the deliberately false property `n < 100` (`prop_assert!`). Because the property is densely false, the library quickly generates and shrinks a counterexample; we match the returned `Err(TestError::Fail(..))` and print `found`. proptest keeps its falsifying-example/shrink report off stdout (only a harmless stderr note), so stdout shows only `found`.

## Run

    cargo run
