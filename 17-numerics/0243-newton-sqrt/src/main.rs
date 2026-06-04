fn main() {
    let n = 2.0f64;
    let mut x = n;
    loop {
        let next = x - (x * x - n) / (2.0 * x);
        if (next - x).abs() < 1e-12 {
            x = next;
            break;
        }
        x = next;
    }
    println!("{:.4}", x);
}
