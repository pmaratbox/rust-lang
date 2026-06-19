use retry::{retry, delay::Fixed};
use std::cell::Cell;

fn main() {
    let attempts = Cell::new(0);

    // Allow 3 total attempts: .take(2) => up to 2 retries after the first try.
    // The operation always fails, so every attempt returns Err.
    let result = retry(Fixed::from_millis(0).take(2), || {
        attempts.set(attempts.get() + 1);
        Err::<&str, &str>("boom")
    });

    // Retries exhausted -> the crate returns an Err; catch it and report failure.
    match result {
        Ok(value) => println!("{}", value),
        Err(_) => println!("failed"),
    }
}
