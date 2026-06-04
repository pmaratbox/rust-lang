fn main() {
    let a = [1, 3, 5];
    let b = [2, 4, 6];
    let parts: Vec<String> = a
        .iter()
        .zip(b.iter())
        .flat_map(|(x, y)| [x.to_string(), y.to_string()])
        .collect();
    println!("{}", parts.join(" "));
}
