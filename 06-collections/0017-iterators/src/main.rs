fn main() {
    let nums = [1, 2, 3, 4, 5];

    let sum: i32 = nums.iter().filter(|&&n| n % 2 == 0).map(|&n| n * 2).sum();

    println!("sum: {}", sum);
}
