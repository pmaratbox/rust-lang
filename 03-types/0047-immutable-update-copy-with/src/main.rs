#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 9, ..p1 };
    println!("original: ({}, {})", p1.x, p1.y);
    println!("updated: ({}, {})", p2.x, p2.y);
}
