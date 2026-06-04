trait Color {
    fn name(&self) -> &str;
}

struct Red;
impl Color for Red {
    fn name(&self) -> &str {
        "red"
    }
}

struct Circle {
    color: Box<dyn Color>,
}
impl Circle {
    fn describe(&self) -> String {
        format!("{} circle", self.color.name())
    }
}

fn main() {
    let circle = Circle {
        color: Box::new(Red),
    };
    println!("{}", circle.describe());
}
