trait Shape {
    fn describe(&self) -> &str;
}

struct Circle;
struct Square;
struct Triangle;

impl Shape for Circle {
    fn describe(&self) -> &str {
        "circle"
    }
}

impl Shape for Square {
    fn describe(&self) -> &str {
        "square"
    }
}

impl Shape for Triangle {
    fn describe(&self) -> &str {
        "triangle"
    }
}

fn main() {
    let shapes: Vec<Box<dyn Shape>> = vec![Box::new(Circle), Box::new(Square), Box::new(Triangle)];
    let parts: Vec<&str> = shapes.iter().map(|s| s.describe()).collect();
    println!("{}", parts.join(" "));
}
