fn main() {
    let labels: Vec<&str> = [1, 2, 3, 4]
        .iter()
        .map(|n| if n % 2 == 0 { "even" } else { "odd" })
        .collect();
    println!("{}", labels.join(" "));
}
