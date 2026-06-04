fn main() {
    let mut a = [2, 0, 2, 1, 1, 0];
    let mut low = 0;
    let mut mid = 0;
    let mut high = a.len();
    while mid < high {
        match a[mid] {
            0 => {
                a.swap(low, mid);
                low += 1;
                mid += 1;
            }
            1 => mid += 1,
            _ => {
                high -= 1;
                a.swap(mid, high);
            }
        }
    }
    let out: Vec<String> = a.iter().map(|x| x.to_string()).collect();
    println!("{}", out.join(" "));
}
