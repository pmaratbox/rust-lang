fn main() {
    let data = [1, 2, 3, 4];
    let products: Vec<i32> = data
        .iter()
        .scan(1, |acc, &x| {
            *acc *= x;
            Some(*acc)
        })
        .collect();
    let out = products
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    println!("{}", out);
}
