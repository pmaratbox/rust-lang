fn main() {
    let nums = [1, 2];
    let letters = ["a", "b"];
    let pairs: Vec<String> = nums
        .iter()
        .flat_map(|n| letters.iter().map(move |l| format!("{}{}", n, l)))
        .collect();
    println!("{}", pairs.join(" "));
}
