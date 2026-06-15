// proptest TestRunner (programmatic): generate two i32 args and check the property.
// runner.run(...).unwrap() succeeds for a universally-true property -> print "passed".
use proptest::prelude::*;
use proptest::test_runner::{TestRunner, Config};

fn main() {
    let mut runner = TestRunner::new(Config { cases: 100, ..Config::default() });
    // Generate two integers a, b and assert addition is commutative.
    runner
        .run(&(any::<i32>(), any::<i32>()), |(a, b)| {
            prop_assert_eq!(a.wrapping_add(b), b.wrapping_add(a));
            Ok(())
        })
        .unwrap();
    println!("passed");
}
