fn main() {
    let mut sb = String::new();
    for (i, n) in [1, 2, 3].iter().enumerate() {
        if i > 0 {
            sb.push('-');
        }
        sb.push_str(&n.to_string());
    }
    println!("{}", sb);
}
