fn main() {
    let mut parts = Vec::new();
    let mut i = 1;
    loop {
        parts.push(i.to_string());
        i += 1;
        if i > 3 {
            break;
        }
    }
    println!("{}", parts.join(" "));
}
