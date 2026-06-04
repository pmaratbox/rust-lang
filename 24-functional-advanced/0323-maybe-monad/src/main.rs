fn main() {
    let present = Some(2).and_then(|x| Some(x + 3)).and_then(|x| Some(x * 2));
    let absent: Option<i32> = None;
    let absent = absent.and_then(|x| Some(x + 3)).and_then(|x| Some(x * 2));

    let a = present.map(|x| x.to_string()).unwrap_or_else(|| "none".to_string());
    let b = absent.map(|x| x.to_string()).unwrap_or_else(|| "none".to_string());

    println!("{} {}", a, b);
}
