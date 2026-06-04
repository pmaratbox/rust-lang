fn check(arg: i32) -> Result<&'static str, String> {
    // Validate the precondition and return a Result instead of panicking.
    if arg > 0 {
        Ok("ok")
    } else {
        Err("error: must be positive".to_string())
    }
}

fn report(arg: i32) {
    match check(arg) {
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("{}", e),
    }
}

fn main() {
    report(5);
    report(-1);
}
