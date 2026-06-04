fn main() {
    let set = [1, 2, 3];
    let n = set.len();
    for mask in 0..(1 << n) {
        let mut parts = Vec::new();
        for i in 0..n {
            if mask & (1 << i) != 0 {
                parts.push(set[i].to_string());
            }
        }
        if parts.is_empty() {
            println!("{{}}");
        } else {
            println!("{}", parts.join(" "));
        }
    }
}
