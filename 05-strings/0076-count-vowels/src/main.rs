fn main() {
    let word = "hello";
    let count = word.chars().filter(|c| "aeiou".contains(*c)).count();
    println!("{}", count);
}
