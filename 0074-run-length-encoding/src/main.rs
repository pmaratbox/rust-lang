fn main() {
    let text = "aaabbc";
    let chars: Vec<char> = text.chars().collect();
    let mut result = String::new();
    let mut i = 0;
    while i < chars.len() {
        let ch = chars[i];
        let mut count = 0;
        while i < chars.len() && chars[i] == ch {
            count += 1;
            i += 1;
        }
        result.push(ch);
        result.push_str(&count.to_string());
    }
    println!("{}", result);
}
