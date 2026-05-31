fn counter() -> impl FnMut() -> i32 {
    let mut count = 0;
    move || {
        count += 1;
        count
    }
}

fn main() {
    let mut next = counter();
    println!("count: {}", next());
    println!("count: {}", next());
}
