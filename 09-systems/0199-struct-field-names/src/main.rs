#[allow(dead_code)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    // Rust has no runtime reflection, so the field names are listed in code.
    const FIELDS: [&'static str; 2] = ["x", "y"];
}

fn main() {
    println!("{}", Point::FIELDS.join(" "));
}
