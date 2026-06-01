fn largest<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

fn main() {
    println!("{}", largest(3, 9));
    println!("{}", largest("apple", "pear"));
}
