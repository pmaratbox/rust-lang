fn main() {
    let a = [2, 2, 1, 2, 3, 2];
    let mut candidate = a[0];
    let mut count = 0;
    for &x in &a {
        if count == 0 {
            candidate = x;
        }
        if x == candidate {
            count += 1;
        } else {
            count -= 1;
        }
    }
    println!("{}", candidate);
}
