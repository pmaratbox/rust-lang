fn main() {
    let mask: i32 = 5;
    let mut subs = Vec::new();
    let mut sub = mask;
    loop {
        subs.push(sub.to_string());
        if sub == 0 {
            break;
        }
        sub = (sub - 1) & mask;
    }
    println!("{}", subs.join(" "));
}
