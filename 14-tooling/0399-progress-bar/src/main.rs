fn main() {
    let width = 10usize;
    let progress = 0.4f64;
    let filled = (width as f64 * progress).round() as usize;
    let empty = width - filled;
    let bar: String = "#".repeat(filled) + &"-".repeat(empty);
    println!("[{}]", bar);
}
