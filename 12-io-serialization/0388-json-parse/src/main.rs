fn main() {
    let json = r#"{"x":1,"y":2}"#;
    let body = json.trim_matches(|c| c == '{' || c == '}');
    let parts: Vec<String> = body
        .split(',')
        .map(|pair| {
            let mut it = pair.splitn(2, ':');
            let key = it.next().unwrap().trim().trim_matches('"');
            let val = it.next().unwrap().trim();
            format!("{}={}", key, val)
        })
        .collect();
    println!("{}", parts.join(" "));
}
