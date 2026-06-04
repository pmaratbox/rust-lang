fn main() {
    let mut a = [5, 1, 4, 2, 8];
    for i in 1..a.len() {
        let key = a[i];
        let mut j = i;
        while j > 0 && a[j - 1] > key {
            a[j] = a[j - 1];
            j -= 1;
        }
        a[j] = key;
    }
    let parts: Vec<String> = a.iter().map(|x| x.to_string()).collect();
    println!("{}", parts.join(" "));
}
