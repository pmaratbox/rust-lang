fn greet(name: &str, greeting: Option<&str>) -> String {
    format!("{}, {}", greeting.unwrap_or("Hello"), name)
}

fn main() {
    println!("{}", greet("Ada", None));
    println!("{}", greet("Ada", Some("Hi")));
}
