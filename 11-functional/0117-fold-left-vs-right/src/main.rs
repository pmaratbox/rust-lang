fn main() {
    let xs = [1, 2, 3];

    let left = xs.iter().fold(0, |acc, &x| acc - x);
    let right = xs.iter().rfold(0, |acc, &x| x - acc);

    println!("{} {}", left, right);
}
