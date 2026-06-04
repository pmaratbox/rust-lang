fn main() {
    let mut c = 1u64;
    let mut out = Vec::new();
    for n in 0..5 {
        out.push(c.to_string());
        c = c * 2 * (2 * n + 1) / (n + 2);
    }
    println!("{}", out.join(" "));
}
