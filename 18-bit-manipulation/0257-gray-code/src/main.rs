fn main() {
    let codes: Vec<String> = (0..4u32).map(|n| (n ^ (n >> 1)).to_string()).collect();
    println!("{}", codes.join(" "));
}
