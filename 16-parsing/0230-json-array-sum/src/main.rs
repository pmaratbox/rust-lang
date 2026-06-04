fn main() {
    let text = "[1,2,3]";
    let inner = text.trim_start_matches('[').trim_end_matches(']');
    let sum: i64 = inner
        .split(',')
        .map(|s| s.trim().parse::<i64>().unwrap())
        .sum();
    println!("{}", sum);
}
