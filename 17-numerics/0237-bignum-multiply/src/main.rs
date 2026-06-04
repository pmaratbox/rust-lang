fn multiply(a: &str, b: &str) -> String {
    let a: Vec<u32> = a.bytes().rev().map(|c| (c - b'0') as u32).collect();
    let b: Vec<u32> = b.bytes().rev().map(|c| (c - b'0') as u32).collect();
    let mut res = vec![0u32; a.len() + b.len()];
    for (i, &da) in a.iter().enumerate() {
        for (j, &db) in b.iter().enumerate() {
            res[i + j] += da * db;
        }
    }
    let mut carry = 0u32;
    for d in res.iter_mut() {
        let cur = *d + carry;
        *d = cur % 10;
        carry = cur / 10;
    }
    while res.len() > 1 && *res.last().unwrap() == 0 {
        res.pop();
    }
    res.iter()
        .rev()
        .map(|d| char::from_digit(*d, 10).unwrap())
        .collect()
}

fn main() {
    println!("{}", multiply("123", "456"));
}
