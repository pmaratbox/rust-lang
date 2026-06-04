fn weekday(mut year: i64, mut month: i64, day: i64) -> &'static str {
    // Treat January and February as months 13 and 14 of the previous year.
    if month < 3 {
        month += 12;
        year -= 1;
    }
    let k = year % 100;
    let j = year / 100;
    let h = (day + (13 * (month + 1)) / 5 + k + k / 4 + j / 4 + 5 * j) % 7;
    // Zeller: h = 0 -> Saturday, 1 -> Sunday, ...
    const NAMES: [&str; 7] = [
        "Saturday",
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
    ];
    NAMES[h as usize]
}

fn main() {
    println!("{}", weekday(2000, 1, 1));
}
