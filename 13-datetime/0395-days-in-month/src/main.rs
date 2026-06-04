fn is_leap(y: i32) -> bool {
    y % 4 == 0 && (y % 100 != 0 || y % 400 == 0)
}

fn feb_days(y: i32) -> i32 {
    if is_leap(y) {
        29
    } else {
        28
    }
}

fn main() {
    println!("{} {}", feb_days(2000), feb_days(2001));
}
