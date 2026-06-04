fn label(flag: bool) -> &'static str {
    if flag {
        "enabled"
    } else {
        "disabled"
    }
}

fn main() {
    println!("{} {}", label(true), label(false));
}
