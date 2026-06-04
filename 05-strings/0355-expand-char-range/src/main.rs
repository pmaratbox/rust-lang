fn expand(spec: &str) -> String {
    let chars: Vec<char> = spec.chars().collect();
    let start = chars[0];
    let end = chars[2];
    (start..=end).collect()
}

fn main() {
    println!("{}", expand("a-e"));
}
