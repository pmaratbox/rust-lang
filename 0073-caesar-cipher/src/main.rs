fn main() {
    let text = "abc";
    let shift = 1u8;
    let result: String = text
        .bytes()
        .map(|b| ((b - b'a' + shift) % 26 + b'a') as char)
        .collect();
    println!("{}", result);
}
