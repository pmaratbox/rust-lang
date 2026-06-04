fn step(state: &str, event: &str) -> &'static str {
    match (state, event) {
        ("locked", "coin") => "unlocked",
        ("unlocked", "push") => "locked",
        ("locked", "push") => "locked",
        _ => "locked",
    }
}

fn main() {
    let mut state = "locked";
    let mut out = Vec::new();
    for event in ["coin", "push", "push"] {
        state = step(state, event);
        out.push(state);
    }
    println!("{}", out.join(" "));
}
