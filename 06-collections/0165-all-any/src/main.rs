fn main() {
    let nums = [2, 4, 6];
    let all_even = nums.iter().all(|n| n % 2 == 0);
    let any_odd = nums.iter().any(|n| n % 2 != 0);
    let yn = |b: bool| if b { "yes" } else { "no" };
    println!("{} {}", yn(all_even), yn(any_odd));
}
