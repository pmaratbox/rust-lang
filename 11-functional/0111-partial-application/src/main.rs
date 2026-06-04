fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let add10 = |b| add(10, b);
    println!("{}", add10(3));
}
