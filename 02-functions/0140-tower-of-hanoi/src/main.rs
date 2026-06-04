fn moves(n: u32) -> u64 {
    if n == 0 {
        0
    } else {
        2 * moves(n - 1) + 1
    }
}

fn main() {
    println!("{}", moves(3));
}
