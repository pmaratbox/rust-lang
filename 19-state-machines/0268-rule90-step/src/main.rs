fn main() {
    let row: Vec<u8> = "00100".bytes().map(|b| b - b'0').collect();
    let n = row.len();
    let at = |i: isize| -> u8 {
        if i >= 0 && (i as usize) < n {
            row[i as usize]
        } else {
            0
        }
    };
    let next: String = (0..n)
        .map(|i| {
            let v = at(i as isize - 1) ^ at(i as isize + 1);
            (b'0' + v) as char
        })
        .collect();
    println!("{}", next);
}
