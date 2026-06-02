fn main() {
    let nums = [1, 2, 3, 4];
    let sums: Vec<i32> = nums
        .iter()
        .scan(0, |total, &n| {
            *total += n;
            Some(*total)
        })
        .collect();
    let parts: Vec<String> = sums.iter().map(|x| x.to_string()).collect();
    println!("{}", parts.join(" "));
}
