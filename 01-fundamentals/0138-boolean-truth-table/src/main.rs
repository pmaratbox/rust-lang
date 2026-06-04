fn main() {
    for &(a, b) in &[(true, true), (true, false), (false, true), (false, false)] {
        println!("{} {} {} {} {}", a, b, a && b, a || b, a ^ b);
    }
}
