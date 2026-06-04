const READ: u32 = 1;
const WRITE: u32 = 2;

fn main() {
    let flags = READ | WRITE;
    let set = if flags & WRITE != 0 { "yes" } else { "no" };
    println!("{} {}", flags, set);
}
