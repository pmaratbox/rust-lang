// proptest TestRunner (programmatic). A `.prop_filter` precondition keeps only
// positive integers, so the strategy `any::<i32>().prop_filter(...)` discards
// non-positive samples before the property runs. `runner.run(...).unwrap()`
// checks `n + 1 > n` over ~100 generated positive cases. Passing prints `passed`.
use proptest::prelude::*;
use proptest::test_runner::{Config, TestRunner};

fn main() {
    let mut runner = TestRunner::new(Config { cases: 100, ..Config::default() });
    let positive = any::<i32>().prop_filter("must be positive", |n| *n > 0 && *n < i32::MAX);
    runner
        .run(&positive, |n| {
            prop_assert!(n + 1 > n);
            Ok(())
        })
        .unwrap();
    println!("passed");
}
