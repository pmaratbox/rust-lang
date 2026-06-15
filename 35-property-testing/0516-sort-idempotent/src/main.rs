// proptest TestRunner (programmatic). The `proptest::collection::vec` strategy
// generates integer lists; `runner.run(...).unwrap()` checks the property over
// ~100 cases. Property: sorting an already-sorted list equals sorting once.
use proptest::prelude::*;
use proptest::test_runner::{Config, TestRunner};

fn main() {
    let mut runner = TestRunner::new(Config { cases: 100, ..Config::default() });
    runner
        .run(&proptest::collection::vec(any::<i32>(), 0..10usize), |xs| {
            let mut once = xs.clone();
            once.sort();
            let mut twice = once.clone();
            twice.sort();
            prop_assert_eq!(twice, once);
            Ok(())
        })
        .unwrap();
    println!("passed");
}
