fn main() {
    let nums = [1, 2, 3, 4, 5, 6];
    let (evens, odds): (Vec<i32>, Vec<i32>) = nums.into_iter().partition(|n| n % 2 == 0);
    println!("evens: {}", evens.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
    println!("odds: {}", odds.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
}
