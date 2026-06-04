fn main() {
    let total = 10 * 60 + 45 + 90;
    let (h, m) = (total / 60, total % 60);
    println!("{:02}:{:02}", h, m);
}
