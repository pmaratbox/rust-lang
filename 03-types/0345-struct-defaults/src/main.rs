#[derive(Default)]
struct Point {
    x: i64,
    y: i64,
}

fn main() {
    let a = Point::default();
    let b = Point {
        x: 5,
        ..Default::default()
    };
    println!("{} {}", a.x, a.y);
    println!("{} {}", b.x, b.y);
}
