fn main() {
    let words = ["flower", "flow", "flight"];
    let first = words[0];
    let mut end = first.len();
    for w in &words[1..] {
        let common = first
            .chars()
            .zip(w.chars())
            .take_while(|(a, b)| a == b)
            .count();
        end = end.min(common);
    }
    println!("{}", &first[..end]);
}
