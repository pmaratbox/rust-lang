fn main() {
    let xs = [1, 2, 3, 4];
    let pairs: Vec<String> = xs
        .windows(2)
        .map(|w| format!("{},{}", w[0], w[1]))
        .collect();
    println!("{}", pairs.join(" "));
}
