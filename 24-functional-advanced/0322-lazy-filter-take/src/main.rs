fn main() {
    let evens: Vec<u32> = (1..).filter(|n| n % 2 == 0).take(3).collect();
    let out: Vec<String> = evens.iter().map(|n| n.to_string()).collect();
    println!("{}", out.join(" "));
}
