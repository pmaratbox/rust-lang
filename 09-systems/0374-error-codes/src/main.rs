fn message(code: u32) -> &'static str {
    match code {
        0 => "ok",
        1 => "denied",
        2 => "not found",
        _ => "unknown",
    }
}

fn main() {
    println!("{}", message(2));
}
