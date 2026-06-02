fn main() {
    let present: Option<i32> = Some(42);
    let absent: Option<i32> = None;

    println!("present: {}", present.unwrap_or(-1));
    println!("absent: {}", absent.unwrap_or(-1));
}
