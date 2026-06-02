fn main() {
    let mut nums = vec![1, 2, 3, 4, 5];
    nums.rotate_left(2);
    let parts: Vec<String> = nums.iter().map(|x| x.to_string()).collect();
    println!("{}", parts.join(" "));
}
