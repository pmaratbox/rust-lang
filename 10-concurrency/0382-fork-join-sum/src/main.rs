use std::thread;

fn sum_range(nums: &[i32]) -> i32 {
    if nums.len() <= 1 {
        return nums.iter().sum();
    }
    let mid = nums.len() / 2;
    let (left, right) = nums.split_at(mid);
    thread::scope(|s| {
        let lh = s.spawn(|| sum_range(left));
        let rh = s.spawn(|| sum_range(right));
        lh.join().unwrap() + rh.join().unwrap()
    })
}

fn main() {
    let nums: Vec<i32> = (1..=8).collect();
    println!("{}", sum_range(&nums));
}
