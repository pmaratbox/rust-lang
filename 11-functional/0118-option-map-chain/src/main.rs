fn main() {
    let present = Some(10);
    let absent: Option<i32> = None;

    let a = present.map(|x| x + 2).map(|x| x.to_string()).unwrap_or_else(|| "none".to_string());
    let b = absent.map(|x| x + 2).map(|x| x.to_string()).unwrap_or_else(|| "none".to_string());

    println!("{} {}", a, b);
}
