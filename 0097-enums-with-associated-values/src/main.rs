enum Shape {
    Rect(i32, i32),
    Square(i32),
}

fn area(shape: &Shape) -> i32 {
    match shape {
        Shape::Rect(w, h) => w * h,
        Shape::Square(s) => s * s,
    }
}

fn main() {
    let shapes = [Shape::Rect(2, 3), Shape::Square(4)];
    for shape in &shapes {
        println!("{}", area(shape));
    }
}
