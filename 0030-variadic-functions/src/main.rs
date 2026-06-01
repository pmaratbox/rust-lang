fn total(nums: &[i32]) -> i32 {
    nums.iter().sum()
}

fn main() {
    println!("sum: {}", total(&[1, 2, 3]));
}
