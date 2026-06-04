fn is_leap(year: i64) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

fn main() {
    let results: Vec<&str> = [2000, 1900, 2024]
        .iter()
        .map(|&y| if is_leap(y) { "yes" } else { "no" })
        .collect();
    println!("{}", results.join(" "));
}
