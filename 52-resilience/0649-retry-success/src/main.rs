use retry::{delay::Fixed, retry};
use std::cell::Cell;

fn main() {
    // A scripted operation that succeeds on its very first call.
    // The shared counter records how many attempts the retry crate made.
    let attempts = Cell::new(0);

    let _ = retry(Fixed::from_millis(0).take(4), || {
        attempts.set(attempts.get() + 1);
        // Always Ok -> the library stops after the first attempt.
        Ok::<&str, &str>("done")
    });

    // No retry needed: exactly one attempt.
    println!("{}", attempts.get());
}
