fn value(c: char) -> i64 {
    match c {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => 0,
    }
}

fn main() {
    let s = "XIV";
    let chars: Vec<char> = s.chars().collect();
    let mut total = 0i64;
    for i in 0..chars.len() {
        let v = value(chars[i]);
        if i + 1 < chars.len() && v < value(chars[i + 1]) {
            total -= v;
        } else {
            total += v;
        }
    }
    println!("{}", total);
}
