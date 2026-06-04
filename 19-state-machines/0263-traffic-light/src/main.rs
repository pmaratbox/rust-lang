fn next(state: &str) -> &'static str {
    match state {
        "red" => "green",
        "green" => "yellow",
        "yellow" => "red",
        _ => "red",
    }
}

fn main() {
    let mut state = "red";
    let mut out = Vec::new();
    for _ in 0..4 {
        state = next(state);
        out.push(state);
    }
    println!("{}", out.join(" "));
}
