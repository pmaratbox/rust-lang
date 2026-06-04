fn rwx(bits: u8) -> String {
    let r = if bits & 0b100 != 0 { 'r' } else { '-' };
    let w = if bits & 0b010 != 0 { 'w' } else { '-' };
    let x = if bits & 0b001 != 0 { 'x' } else { '-' };
    format!("{}{}{}", r, w, x)
}

fn main() {
    println!("{}", rwx(0b101));
}
