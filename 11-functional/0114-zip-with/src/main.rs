fn main() {
    let a = [1, 2, 3];
    let b = [4, 5, 6];

    let summed: Vec<String> = a
        .iter()
        .zip(b.iter())
        .map(|(x, y)| (x + y).to_string())
        .collect();

    println!("{}", summed.join(" "));
}
