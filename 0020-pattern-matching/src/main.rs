fn word(n: i32) -> &'static str {
    match n {
        1 => "one",
        2 => "two",
        _ => "many",
    }
}

fn main() {
    for n in [1, 2, 5] {
        println!("{n}: {}", word(n));
    }
}
