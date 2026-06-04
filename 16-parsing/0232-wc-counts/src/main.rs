fn main() {
    let text = "a b\nc";
    let words = text.split_whitespace().count();
    let lines = text.lines().count();
    let chars = text.chars().count();
    println!("{} {} {}", words, lines, chars);
}
