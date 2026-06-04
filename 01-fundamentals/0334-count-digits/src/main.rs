fn main() {
    let mut n = 90210u32;
    let mut count = 0;
    while n > 0 {
        n /= 10;
        count += 1;
    }
    println!("{}", count);
}
