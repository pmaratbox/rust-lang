trait Greeter {
    fn greet(&self) -> &str {
        "hi"
    }
}

struct Default;
struct Custom;

impl Greeter for Default {}

impl Greeter for Custom {
    fn greet(&self) -> &str {
        "hey"
    }
}

fn main() {
    let a = Default;
    let b = Custom;
    println!("{} {}", a.greet(), b.greet());
}
