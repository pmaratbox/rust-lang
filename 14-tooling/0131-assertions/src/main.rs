fn check(name: &str, cond: bool) {
    if !cond {
        panic!("assertion failed: {name}");
    }
}

fn main() {
    check("1 + 1 == 2", 1 + 1 == 2);
    check("2 * 3 == 6", 2 * 3 == 6);
    check("\"ab\".len() == 2", "ab".len() == 2);
    println!("all passed");
}
