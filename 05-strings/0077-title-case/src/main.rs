fn main() {
    let text = "hello world";
    let result: Vec<String> = text
        .split(' ')
        .map(|w| {
            let mut chars = w.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect();
    println!("{}", result.join(" "));
}
