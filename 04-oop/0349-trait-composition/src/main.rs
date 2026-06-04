trait A {
    fn a(&self) -> &str {
        "a"
    }
}

trait B {
    fn b(&self) -> &str {
        "b"
    }
}

struct Both;

impl A for Both {}
impl B for Both {}

fn main() {
    let t = Both;
    println!("{} {}", t.a(), t.b());
}
