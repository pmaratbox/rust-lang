use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct Inner;

impl fmt::Display for Inner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "inner")
    }
}

impl Error for Inner {}

// Outer error keeps the inner error as its `source`.
#[derive(Debug)]
struct Outer {
    source: Inner,
}

impl fmt::Display for Outer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "outer: {}", self.source)
    }
}

impl Error for Outer {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

fn main() {
    let err = Outer { source: Inner };
    println!("{}", err);
}
