fn main() {
    let result: Vec<i32> = [1, 2, 3, 4]
        .iter()
        .map(|x| x + 1)
        .filter(|x| x % 2 == 0)
        .collect();

    let out: Vec<String> = result.iter().map(|n| n.to_string()).collect();
    println!("{}", out.join(" "));
}
