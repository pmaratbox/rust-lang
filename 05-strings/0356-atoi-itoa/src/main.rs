fn atoi(s: &str) -> i64 {
    let mut chars = s.chars().peekable();
    let mut neg = false;
    if let Some(&c) = chars.peek() {
        if c == '-' {
            neg = true;
            chars.next();
        } else if c == '+' {
            chars.next();
        }
    }
    let mut value: i64 = 0;
    for c in chars {
        if let Some(d) = c.to_digit(10) {
            value = value * 10 + d as i64;
        }
    }
    if neg {
        -value
    } else {
        value
    }
}

fn itoa(mut n: i64) -> String {
    if n == 0 {
        return "0".to_string();
    }
    let neg = n < 0;
    let mut digits = Vec::new();
    if neg {
        n = -n;
    }
    while n > 0 {
        digits.push((b'0' + (n % 10) as u8) as char);
        n /= 10;
    }
    if neg {
        digits.push('-');
    }
    digits.iter().rev().collect()
}

fn main() {
    let parsed = atoi("-42");
    let formatted = itoa(parsed);
    println!("{} {}", parsed, formatted);
}
