fn min_max(a: i32, b: i32) -> (i32, i32) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}

fn main() {
    let (lo, hi) = min_max(3, 7);
    println!("min: {lo}");
    println!("max: {hi}");
}
