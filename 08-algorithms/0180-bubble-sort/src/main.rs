fn main() {
    let mut a = [5, 1, 4, 2, 8];
    let n = a.len();
    for i in 0..n {
        for j in 0..n - 1 - i {
            if a[j] > a[j + 1] {
                a.swap(j, j + 1);
            }
        }
    }
    let parts: Vec<String> = a.iter().map(|x| x.to_string()).collect();
    println!("{}", parts.join(" "));
}
