fn main() {
    let truncated = 3.9_f64 as i64;
    let widened = 3_i64 as f64;
    println!("{} {:.1}", truncated, widened);
}
