# 0514 — Integer property

Uses the **proptest** property-testing library driven **programmatically** through its `TestRunner` API (no test runner, no `#[test]`). A `TestRunner` configured for 100 cases runs against the tuple strategy `(any::<i32>(), any::<i32>())`, which **generates** pairs of random `i32` integers. For each generated pair `(a, b)` the property asserts that addition is commutative (`a + b == b + a`), using `prop_assert_eq!`. Because the property holds for every input, `runner.run(...).unwrap()` returns `Ok`, and the program prints `passed`.

## Run

    cargo run
