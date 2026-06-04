fn main() {
    let values: Vec<i32> = std::iter::successors(Some(1), |&x| Some(x * 3))
        .take(4)
        .collect();
    let out = values
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    println!("{}", out);
}
