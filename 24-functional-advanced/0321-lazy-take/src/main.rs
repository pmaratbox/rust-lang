fn main() {
    let firstfive: Vec<u32> = (1..).take(5).collect();
    let out: Vec<String> = firstfive.iter().map(|n| n.to_string()).collect();
    println!("{}", out.join(" "));
}
