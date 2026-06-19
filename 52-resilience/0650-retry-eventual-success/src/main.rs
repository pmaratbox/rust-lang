use retry::{delay::Fixed, retry};
use std::cell::Cell;

fn main() {
    // A scripted operation that fails on its first call, then succeeds.
    // The shared counter records how many attempts the retry crate made.
    let attempts = Cell::new(0);

    let _ = retry(Fixed::from_millis(0).take(4), || {
        attempts.set(attempts.get() + 1);
        // Fail the first attempt; succeed from the second onwards.
        if attempts.get() < 2 {
            Err("fail")
        } else {
            Ok("done")
        }
    });

    // One failure + one success => the library made two attempts.
    println!("{}", attempts.get());
}
