fn main() {
    // Linear congruential generator: next = (5 * x + 3) mod 16, seeded at 1.
    let mut x: u32 = 1;
    let outputs: Vec<String> = (0..3)
        .map(|_| {
            x = (5 * x + 3) % 16;
            x.to_string()
        })
        .collect();
    println!("{}", outputs.join(" "));
}
