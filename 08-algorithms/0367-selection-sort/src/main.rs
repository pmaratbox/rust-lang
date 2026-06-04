fn main() {
    let mut a = [5, 1, 4, 2];
    let n = a.len();
    for i in 0..n {
        let mut min = i;
        for j in (i + 1)..n {
            if a[j] < a[min] {
                min = j;
            }
        }
        a.swap(i, min);
    }
    let out: Vec<String> = a.iter().map(|x| x.to_string()).collect();
    println!("{}", out.join(" "));
}
