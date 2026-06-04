fn main() {
    let input = "1 + 2";
    let mut tokens: Vec<&str> = Vec::new();
    for ch in input.chars() {
        if ch.is_ascii_digit() {
            tokens.push("NUM");
        } else if ch == '+' {
            tokens.push("PLUS");
        }
    }
    println!("{}", tokens.join(" "));
}
