fn main() {
    let hex: Vec<String> = "Hi".bytes().map(|b| format!("{:02x}", b)).collect();
    println!("{}", hex.join(" "));
}
