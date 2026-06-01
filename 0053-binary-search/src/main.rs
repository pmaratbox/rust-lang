fn main() {
    let nums = [1, 3, 5, 7, 9];
    let target = 7;

    let mut lo: i32 = 0;
    let mut hi: i32 = nums.len() as i32 - 1;
    let mut index: i32 = -1;
    while lo <= hi {
        let mid = (lo + hi) / 2;
        let v = nums[mid as usize];
        if v == target {
            index = mid;
            break;
        } else if v < target {
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }

    println!("found {} at index {}", target, index);
}
