fn main() {
    let (mut a, mut b) = (1, 2);
    (a, b) = (b, a);
    println!("{} {}", a, b);
}
