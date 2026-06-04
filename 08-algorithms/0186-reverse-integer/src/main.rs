fn main() {
    let mut n = 1234;
    let mut rev = 0;
    while n > 0 {
        rev = rev * 10 + n % 10;
        n /= 10;
    }
    println!("{}", rev);
}
