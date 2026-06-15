fn main() {
    let numbers: Vec<i32> = vec![1, 2, 3];
    println!("{}", serde_json::to_string(&numbers).unwrap());
}
