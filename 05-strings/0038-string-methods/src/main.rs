fn main() {
    let upper: Vec<String> = "a,b,c".split(',').map(str::to_uppercase).collect();
    println!("{}", upper.join("-"));
}
