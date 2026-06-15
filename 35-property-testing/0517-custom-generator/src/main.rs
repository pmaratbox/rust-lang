// proptest driven programmatically via TestRunner (no #[test], no cargo test).
// A CUSTOM generator is built from `any::<i32>()` with the `.prop_map` combinator
// (n -> n * 2) so it only produces even integers; runner.run checks the property
// that every generated value is even.
use proptest::prelude::*;
use proptest::test_runner::{TestRunner, Config};

fn main() {
    let mut runner = TestRunner::new(Config { cases: 100, ..Config::default() });
    // Custom generator: map arbitrary i32s to even numbers.
    let evens = any::<i32>().prop_map(|n| n.wrapping_mul(2));
    runner
        .run(&evens, |n| {
            // Property: every generated value is even.
            prop_assert_eq!(n % 2, 0);
            Ok(())
        })
        .unwrap();
    println!("passed");
}
