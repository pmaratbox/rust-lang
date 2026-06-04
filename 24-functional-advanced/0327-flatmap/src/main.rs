fn main() {
    let result: Vec<i32> = [1, 2, 3].iter().flat_map(|&x| vec![x, x * 10]).collect();
    let out: Vec<String> = result.iter().map(|n| n.to_string()).collect();
    println!("{}", out.join(" "));
}
