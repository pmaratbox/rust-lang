fn main() {
    let m = [[1i64, 2], [3, 4]];
    let v = [5i64, 6];
    let result: Vec<i64> = m
        .iter()
        .map(|row| row.iter().zip(v.iter()).map(|(a, b)| a * b).sum())
        .collect();
    let parts: Vec<String> = result.iter().map(|x| x.to_string()).collect();
    println!("{}", parts.join(" "));
}
