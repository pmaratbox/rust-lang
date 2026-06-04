fn snake_to_camel(s: &str) -> String {
    let mut result = String::new();
    for (i, word) in s.split('_').enumerate() {
        if i == 0 {
            result.push_str(word);
        } else if let Some(first) = word.chars().next() {
            result.extend(first.to_uppercase());
            result.push_str(&word[first.len_utf8()..]);
        }
    }
    result
}

fn main() {
    println!("{}", snake_to_camel("hello_world"));
}
