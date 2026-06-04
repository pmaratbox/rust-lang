fn clamp(x: i32, lo: i32, hi: i32) -> i32 {
    lo.max(x.min(hi))
}

fn main() {
    println!("{} {}", clamp(15, 0, 10), clamp(-3, 0, 10));
}
