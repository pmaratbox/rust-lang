fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(a: u32, b: u32) -> u32 {
    a / gcd(a, b) * b
}

fn main() {
    println!("{}", lcm(4, 6));
}
