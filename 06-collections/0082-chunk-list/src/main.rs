fn main() {
    let nums = [1, 2, 3, 4, 5, 6, 7];
    for chunk in nums.chunks(3) {
        let parts: Vec<String> = chunk.iter().map(|x| x.to_string()).collect();
        println!("{}", parts.join(" "));
    }
}
