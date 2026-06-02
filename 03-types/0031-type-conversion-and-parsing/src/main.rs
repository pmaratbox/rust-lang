fn main() {
    let n: i32 = "42".parse().unwrap();
    let f: f64 = "3.5".parse().unwrap();
    let s = n.to_string();
    println!("int: {n}");
    println!("float: {f}");
    println!("str: {s}");
}
