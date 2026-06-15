// proptest driven programmatically via TestRunner (no #[test], no cargo test).
// The library generates ~100 integer lists and checks the property on each;
// runner.run(..).unwrap() succeeds only if every generated case holds.
use proptest::prelude::*;
use proptest::test_runner::{TestRunner, Config};

fn main() {
    let mut runner = TestRunner::new(Config { cases: 100, ..Config::default() });
    runner
        .run(&proptest::collection::vec(any::<i32>(), 0..10usize), |xs| {
            // Property: reversing a list twice yields the original.
            let mut twice = xs.clone();
            twice.reverse();
            twice.reverse();
            prop_assert_eq!(twice, xs);
            Ok(())
        })
        .unwrap();
    println!("passed");
}
