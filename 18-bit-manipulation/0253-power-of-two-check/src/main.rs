fn is_power_of_two(n: u32) -> bool {
    n > 0 && (n & (n - 1)) == 0
}

fn main() {
    let label = |b: bool| if b { "yes" } else { "no" };
    println!("{} {}", label(is_power_of_two(16)), label(is_power_of_two(18)));
}
