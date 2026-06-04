#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    // Rust has no built-in enum iteration, so list the variants explicitly.
    const ALL: [Color; 3] = [Color::Red, Color::Green, Color::Blue];

    fn name(&self) -> &'static str {
        match self {
            Color::Red => "RED",
            Color::Green => "GREEN",
            Color::Blue => "BLUE",
        }
    }
}

fn main() {
    let names: Vec<&str> = Color::ALL.iter().map(Color::name).collect();
    println!("{}", names.join(" "));
}
