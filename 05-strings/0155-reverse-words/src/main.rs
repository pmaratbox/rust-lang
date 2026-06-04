fn main() {
    let s = "hello world";
    let reversed: Vec<&str> = s.split(' ').rev().collect();
    println!("{}", reversed.join(" "));
}
