fn divisible_by_3(bits: &str) -> bool {
    let mut state = 0u32;
    for c in bits.chars() {
        let b = c as u32 - '0' as u32;
        state = (state * 2 + b) % 3;
    }
    state == 0
}

fn main() {
    let label = |b: bool| if b { "yes" } else { "no" };
    println!("{} {}", label(divisible_by_3("110")), label(divisible_by_3("100")));
}
