fn main() {
    let nums = [10, 9, 2, 5, 3, 7, 101, 18];
    let mut tails: Vec<i32> = Vec::new();
    for &x in &nums {
        match tails.binary_search(&x) {
            Ok(_) => {}
            Err(pos) => {
                if pos == tails.len() {
                    tails.push(x);
                } else {
                    tails[pos] = x;
                }
            }
        }
    }
    println!("{}", tails.len());
}
