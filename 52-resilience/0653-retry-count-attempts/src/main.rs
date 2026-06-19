use retry::{delay::Fixed, retry};
use std::cell::Cell;

fn main() {
    // A scripted operation that ALWAYS fails. A shared counter records how many
    // times the closure runs, so we can report the total number of attempts the
    // library made before giving up.
    let attempts = Cell::new(0);

    // .take(4) => up to 4 retries after the first try => 5 attempts total.
    // Fixed::from_millis(0) => zero delay, so the count is timing-independent.
    let result = retry(Fixed::from_millis(0).take(4), || {
        attempts.set(attempts.get() + 1);
        Err::<(), &str>("always fails")
    });

    // The operation never succeeds; the library exhausts all attempts.
    assert!(result.is_err());

    // The counter, driven by the library actually retrying, is the attempt count.
    println!("{}", attempts.get());
}
