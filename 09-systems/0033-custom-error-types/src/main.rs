use std::fmt;

#[derive(Debug)]
struct TooLargeError;

impl fmt::Display for TooLargeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "value too large")
    }
}

fn check(n: i32) -> Result<(), TooLargeError> {
    if n > 100 {
        Err(TooLargeError)
    } else {
        Ok(())
    }
}

fn main() {
    if let Err(e) = check(200) {
        println!("error: {e}");
    }
}
