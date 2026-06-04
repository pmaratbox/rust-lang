fn main() {
    let mut a = [5, 2, 8, 1, 9, 3];
    let n = a.len();
    let mut gap = n / 2;
    while gap > 0 {
        for i in gap..n {
            let temp = a[i];
            let mut j = i;
            while j >= gap && a[j - gap] > temp {
                a[j] = a[j - gap];
                j -= gap;
            }
            a[j] = temp;
        }
        gap /= 2;
    }
    let out: Vec<String> = a.iter().map(|x| x.to_string()).collect();
    println!("{}", out.join(" "));
}
