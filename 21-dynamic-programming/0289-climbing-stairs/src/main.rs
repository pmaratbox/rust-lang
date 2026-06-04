fn main() {
    let n = 5;
    let (mut a, mut b) = (1u64, 1u64);
    for _ in 0..n {
        let next = a + b;
        a = b;
        b = next;
    }
    println!("{}", a);
}
