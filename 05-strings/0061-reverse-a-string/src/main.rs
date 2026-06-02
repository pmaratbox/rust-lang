fn main() {
    let text = "abc";
    let reversed: String = text.chars().rev().collect();
    println!("{}", reversed);
}
