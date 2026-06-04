fn min_max(xs: &[i32]) -> (i32, i32) {
    let mut min = xs[0];
    let mut max = xs[0];
    for &x in &xs[1..] {
        if x < min {
            min = x;
        }
        if x > max {
            max = x;
        }
    }
    (min, max)
}

fn main() {
    let (min, max) = min_max(&[4, 1, 7]);
    println!("{} {}", min, max);
}
