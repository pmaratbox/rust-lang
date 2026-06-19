use retry::{delay::Fixed, retry};
use std::cell::Cell;

fn main() {
    // Fixed backoff: a constant (here zero) delay between retries.
    // .take(4) allows up to four retries -> up to five total attempts.
    let attempts = Cell::new(0);

    let _ = retry(Fixed::from_millis(0).take(4), || {
        attempts.set(attempts.get() + 1);
        // Scripted failure sequence: fail twice, then succeed on the third call.
        if attempts.get() < 3 {
            Err("transient")
        } else {
            Ok("done")
        }
    });

    // The library drove exactly three attempts before the operation succeeded.
    println!("{}", attempts.get());
}
