use retry::{delay::Fixed, retry};
use std::cell::Cell;

fn main() {
    // A scripted operation: it fails on the first attempt, then on the
    // second attempt returns the string "ok". The shared counter makes the
    // failure sequence deterministic without depending on any delay.
    let attempts = Cell::new(0);

    // .take(4) => up to 4 retries (5 attempts total) — more than enough.
    // Fixed::from_millis(0) => zero delay, so the output is timing-independent.
    let result = retry(Fixed::from_millis(0).take(4), || {
        attempts.set(attempts.get() + 1);
        if attempts.get() < 2 {
            Err("not ready")
        } else {
            Ok("ok")
        }
    });

    // The successful value returned by the operation, surfaced by the library.
    println!("{}", result.unwrap());
}
