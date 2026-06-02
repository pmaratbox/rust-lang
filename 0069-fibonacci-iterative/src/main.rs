fn main() {
    let mut a = 0;
    let mut b = 1;
    let mut nums = Vec::new();
    for _ in 0..7 {
        nums.push(a.to_string());
        let t = a + b;
        a = b;
        b = t;
    }
    println!("{}", nums.join(" "));
}
