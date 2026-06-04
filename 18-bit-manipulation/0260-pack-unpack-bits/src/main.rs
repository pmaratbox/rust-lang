fn main() {
    let (r, g, b): (u32, u32, u32) = (1, 2, 3);
    let packed = (r << 16) | (g << 8) | b;
    let ur = (packed >> 16) & 0xff;
    let ug = (packed >> 8) & 0xff;
    let ub = packed & 0xff;
    println!("{} {} {}", ur, ug, ub);
}
