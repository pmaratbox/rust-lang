use retry::{delay::Fixed, retry};
use std::cell::Cell;

fn main() {
    // The scripted operation returns an ever-incrementing counter.
    // We retry *on the result* (not on an exception): a value < 3 is treated
    // as "not good enough yet", so the closure reports Err to make the retry
    // crate try again. The first acceptable value (>= 3) is returned as Ok.
    let counter = Cell::new(0);

    let result = retry(Fixed::from_millis(0).take(4), || {
        counter.set(counter.get() + 1);
        let value = counter.get();
        if value < 3 {
            // Result-based retry: ask the library to retry.
            Err(value)
        } else {
            // First accepted result.
            Ok(value)
        }
    });

    // The accepted result: 3 (returned on the third attempt).
    println!("{}", result.unwrap());
}
