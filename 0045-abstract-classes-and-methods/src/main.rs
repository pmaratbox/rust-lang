trait Shape {
    fn area(&self) -> i32;

    fn describe(&self) -> String {
        format!("area: {}", self.area())
    }
}

struct Square {
    side: i32,
}

impl Shape for Square {
    fn area(&self) -> i32 {
        self.side * self.side
    }
}

fn main() {
    println!("{}", Square { side: 3 }.describe());
}
