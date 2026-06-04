trait Shape {
    fn kind(&self) -> &str;
}

struct Circle;
struct Square;

impl Shape for Circle {
    fn kind(&self) -> &str {
        "circle"
    }
}
impl Shape for Square {
    fn kind(&self) -> &str {
        "square"
    }
}

fn make(name: &str) -> Box<dyn Shape> {
    match name {
        "circle" => Box::new(Circle),
        "square" => Box::new(Square),
        _ => unreachable!(),
    }
}

fn main() {
    let a = make("circle");
    let b = make("square");
    println!("{} {}", a.kind(), b.kind());
}
