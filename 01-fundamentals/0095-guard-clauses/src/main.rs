fn classify(n: i32) -> &'static str {
    if n < 0 {
        return "negative";
    }
    if n == 0 {
        return "zero";
    }
    "positive"
}

fn main() {
    for n in [-1, 0, 5] {
        println!("{}", classify(n));
    }
}
