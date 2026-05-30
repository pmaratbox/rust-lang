trait Shape {
    fn name(&self) -> &str;
    fn area(&self) -> i32;
}

struct Rectangle {
    width: i32,
    height: i32,
}

impl Shape for Rectangle {
    fn name(&self) -> &str {
        "rectangle"
    }
    fn area(&self) -> i32 {
        self.width * self.height
    }
}

struct Square {
    side: i32,
}

impl Shape for Square {
    fn name(&self) -> &str {
        "square"
    }
    fn area(&self) -> i32 {
        self.side * self.side
    }
}

fn main() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Rectangle { width: 3, height: 4 }),
        Box::new(Square { side: 5 }),
    ];
    for s in &shapes {
        println!("{} area: {}", s.name(), s.area());
    }
}
