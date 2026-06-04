trait Shape {
    fn area(&self) -> f64;
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

struct Triangle {
    base: f64,
    height: f64,
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        self.base * self.height / 2.0
    }
}

fn main() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Rectangle { width: 2.0, height: 3.0 }),
        Box::new(Triangle { base: 4.0, height: 4.0 }),
    ];
    let total: f64 = shapes.iter().map(|s| s.area()).sum();
    println!("total area: {}", total);
}
