use retry::{delay::Exponential, retry};
use std::cell::Cell;

fn main() {
    // A scripted operation that fails three times, then succeeds.
    // The shared counter records how many attempts the retry crate made.
    let attempts = Cell::new(0);

    // Exponential backoff with a zero base delay: the delay between attempts
    // would grow geometrically, but the base of 0ms keeps the run instant.
    let _ = retry(Exponential::from_millis(0).take(4), || {
        attempts.set(attempts.get() + 1);
        // Fail attempts 1, 2 and 3; succeed on attempt 4.
        if attempts.get() < 4 {
            Err("fail")
        } else {
            Ok("done")
        }
    });

    // Three failures + one success => the library made four attempts.
    println!("{}", attempts.get());
}
