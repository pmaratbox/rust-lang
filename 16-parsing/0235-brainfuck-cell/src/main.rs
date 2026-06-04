fn main() {
    let program = "+++";
    let mut cell: u8 = 0;
    for op in program.chars() {
        match op {
            '+' => cell = cell.wrapping_add(1),
            '-' => cell = cell.wrapping_sub(1),
            _ => {}
        }
    }
    println!("{}", cell);
}
