fn main() {
    for n in [4, 7] {
        let label = if n % 2 == 0 { "even" } else { "odd" };
        println!("{n}: {label}");
    }
}
