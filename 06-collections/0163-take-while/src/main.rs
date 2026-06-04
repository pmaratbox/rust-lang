fn main() {
    let nums = [1, 2, 3, 4, 1];
    let taken: Vec<String> = nums
        .iter()
        .take_while(|&&n| n < 3)
        .map(|n| n.to_string())
        .collect();
    println!("{}", taken.join(" "));
}
