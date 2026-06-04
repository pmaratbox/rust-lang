fn main() {
    let terms: Vec<i32> = std::iter::successors(Some(1), |&x| Some(x * 2))
        .take(5)
        .collect();
    let out = terms
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    println!("{}", out);
}
