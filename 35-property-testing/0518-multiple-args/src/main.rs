// proptest TestRunner (programmatic). A tuple strategy `(any::<i32>(), any::<i32>())`
// generates TWO integer arguments per case; `runner.run(...).unwrap()` checks the
// property over ~100 cases. Property: max(a, b) >= a AND max(a, b) >= b.
use proptest::prelude::*;
use proptest::test_runner::{Config, TestRunner};

fn main() {
    let mut runner = TestRunner::new(Config { cases: 100, ..Config::default() });
    runner
        .run(&(any::<i32>(), any::<i32>()), |(a, b)| {
            let m = a.max(b);
            prop_assert!(m >= a);
            prop_assert!(m >= b);
            Ok(())
        })
        .unwrap();
    println!("passed");
}
