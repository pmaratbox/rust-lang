fn main() {
    let nums = [1, 2, 3, 4];
    for window in nums.windows(2) {
        let parts: Vec<String> = window.iter().map(|x| x.to_string()).collect();
        println!("{}", parts.join(" "));
    }
}
