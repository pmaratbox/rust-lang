fn rot13(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'a'..='z' => (b'a' + (c as u8 - b'a' + 13) % 26) as char,
            'A'..='Z' => (b'A' + (c as u8 - b'A' + 13) % 26) as char,
            _ => c,
        })
        .collect()
}

fn main() {
    let encoded = rot13("hello");
    let decoded = rot13(&encoded);
    println!("{} {}", encoded, decoded);
}
