use std::collections::HashMap;

fn main() {
    let next: HashMap<&str, &str> = [("A", "B"), ("B", "C"), ("C", "A")].into_iter().collect();
    let mut state = "A";
    let mut out = Vec::new();
    for _ in 0..3 {
        state = next[state];
        out.push(state);
    }
    println!("{}", out.join(" "));
}
