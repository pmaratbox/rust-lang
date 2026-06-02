fn main() {
    let nested = [[1, 2], [3, 4]];
    let flat: Vec<i32> = nested.iter().flatten().copied().collect();
    let parts: Vec<String> = flat.iter().map(|x| x.to_string()).collect();
    println!("{}", parts.join(" "));
}
