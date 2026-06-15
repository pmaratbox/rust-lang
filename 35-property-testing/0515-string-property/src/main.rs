// proptest, driven programmatically (no test runner). TestRunner::run generates the
// inputs and checks the property; `.unwrap()` panics if any case falsifies it, so
// reaching the println means all 100 generated strings held. The strategy is the
// default `String` regex generator ".*".
use proptest::prelude::*;
use proptest::test_runner::{TestRunner, Config};

fn main() {
    let mut runner = TestRunner::new(Config { cases: 100, ..Config::default() });
    runner
        .run(&".*", |s| {
            let doubled = format!("{s}{s}");
            prop_assert_eq!(doubled.len(), 2 * s.len());
            Ok(())
        })
        .unwrap();
    println!("passed");
}
