fn risky() -> Result<(), String> {
    Err("boom".to_string())
}

fn main() {
    // Rust has no exceptions for recoverable errors: match on the Result.
    match risky() {
        Ok(()) => {}
        Err(_) => println!("caught"),
    }
    // The "finally" cleanup always runs after the match.
    println!("cleanup");
}
