#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
}

fn main() {
    println!("green: {}", Color::Green as i32);
    println!("blue: {}", Color::Blue as i32);
}
