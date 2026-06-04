fn main() {
    let words = ["a", "bbb", "cc"];
    let longest = words.iter().max_by_key(|s| s.len()).unwrap();
    println!("{}", longest);
}
