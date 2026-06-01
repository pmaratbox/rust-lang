fn main() {
    let first3: Vec<String> = (1..).map(|n| n * n).take(3).map(|s| s.to_string()).collect();
    println!("{}", first3.join(" "));
}
