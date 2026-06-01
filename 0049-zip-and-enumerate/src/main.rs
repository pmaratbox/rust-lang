fn main() {
    let letters = ["a", "b", "c"];
    let nums = [1, 2, 3];
    let pairs: Vec<String> = letters
        .iter()
        .zip(nums.iter())
        .map(|(k, n)| format!("{k}={n}"))
        .collect();
    println!("{}", pairs.join(" "));
}
