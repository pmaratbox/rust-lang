struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    fn from_hex(hex: &str) -> Color {
        let h = hex.trim_start_matches('#');
        let r = u8::from_str_radix(&h[0..2], 16).unwrap();
        let g = u8::from_str_radix(&h[2..4], 16).unwrap();
        let b = u8::from_str_radix(&h[4..6], 16).unwrap();
        Color { r, g, b }
    }
}

fn main() {
    let c = Color::from_hex("#ff0000");
    println!("{} {} {}", c.r, c.g, c.b);
}
