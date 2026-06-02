fn main() {
    let nums = [10, 20, 30, 40, 50];
    let slice = &nums[1..4];
    let parts: Vec<String> = slice.iter().map(|n| n.to_string()).collect();
    println!("slice: {}", parts.join(" "));
}
