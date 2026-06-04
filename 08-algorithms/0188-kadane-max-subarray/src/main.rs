fn main() {
    let a = [-2, 1, -3, 4, -1, 2, 1, -5, 4];
    let mut cur = a[0];
    let mut best = a[0];
    for &x in &a[1..] {
        cur = (cur + x).max(x);
        best = best.max(cur);
    }
    println!("{}", best);
}
