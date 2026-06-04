fn main() {
    let bit = 1u32;
    let set = 0u32 | (1 << bit);
    let clear = 2u32 & !(1 << bit);
    let toggle = 0u32 ^ (1 << bit);
    println!("{} {} {}", set, clear, toggle);
}
