fn main() {
    let nums = [4, 2, 7, 1, 9];
    let target = 7;
    let index = nums.iter().position(|&n| n == target);
    println!("found {} at index {}", target, index.unwrap());
}
