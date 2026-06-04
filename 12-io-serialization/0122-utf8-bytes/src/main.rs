fn main() {
    let text = "Hi";

    let bytes: Vec<String> = text.as_bytes().iter().map(|b| b.to_string()).collect();

    println!("{}", bytes.join(" "));
}
