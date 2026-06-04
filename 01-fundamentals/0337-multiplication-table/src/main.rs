fn main() {
    for i in 1..=3 {
        let row: Vec<String> = (1..=3).map(|j| (i * j).to_string()).collect();
        println!("{}", row.join(" "));
    }
}
