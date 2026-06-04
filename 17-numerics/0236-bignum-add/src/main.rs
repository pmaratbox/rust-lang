fn add(a: &str, b: &str) -> String {
    let a: Vec<u8> = a.bytes().rev().map(|c| c - b'0').collect();
    let b: Vec<u8> = b.bytes().rev().map(|c| c - b'0').collect();
    let mut out = Vec::new();
    let mut carry = 0u8;
    for i in 0..a.len().max(b.len()) {
        let da = *a.get(i).unwrap_or(&0);
        let db = *b.get(i).unwrap_or(&0);
        let sum = da + db + carry;
        out.push(sum % 10);
        carry = sum / 10;
    }
    if carry > 0 {
        out.push(carry);
    }
    out.iter().rev().map(|d| (d + b'0') as char).collect()
}

fn main() {
    println!("{}", add("999999999999", "1"));
}
