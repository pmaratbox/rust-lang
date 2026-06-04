fn main() {
    let left = [1, 2, 3];
    let right = ["a", "b"];
    let n = left.len().max(right.len());
    let parts: Vec<String> = (0..n)
        .map(|i| {
            let l = left.get(i).map(|v| v.to_string()).unwrap_or_else(|| "-".to_string());
            let r = right.get(i).map(|v| v.to_string()).unwrap_or_else(|| "-".to_string());
            format!("{}{}", l, r)
        })
        .collect();
    println!("{}", parts.join(" "));
}
