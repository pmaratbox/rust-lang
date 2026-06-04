fn add(a: i32) -> impl Fn(i32) -> i32 {
    move |b| a + b
}

fn main() {
    println!("{}", add(2)(3));
}
