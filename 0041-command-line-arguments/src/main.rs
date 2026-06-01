use std::env;

fn main() {
    let name = env::args().nth(1).unwrap();
    println!("hello, {name}");
}
