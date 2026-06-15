// proptest driven programmatically via TestRunner (no #[test], no cargo test).
// The property "every non-negative integer is < 100" is densely false, so the
// library generates inputs and reliably finds a counterexample. We inspect the
// Result instead of unwrapping: Err(TestError::Fail(..)) means a counterexample
// was found. proptest does not write its falsifying-example/shrink report to
// stdout (only a harmless FileFailurePersistence note on stderr), so stdout
// stays clean and prints only `found`.
use proptest::prelude::*;
use proptest::test_runner::{TestRunner, Config, TestError};

fn main() {
    let mut runner = TestRunner::new(Config { cases: 100, ..Config::default() });
    // Property under test (false): every non-negative integer is < 100.
    let result = runner.run(&(0i32..1_000_000), |n| {
        prop_assert!(n < 100);
        Ok(())
    });
    let found = matches!(result, Err(TestError::Fail(..)));
    if found {
        println!("found");
    }
}
