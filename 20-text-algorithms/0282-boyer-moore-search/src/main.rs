fn boyer_moore(text: &[u8], pat: &[u8]) -> Option<usize> {
    let m = pat.len();
    let n = text.len();
    if m == 0 || m > n {
        return None;
    }
    let mut last = [-1i32; 256];
    for (i, &c) in pat.iter().enumerate() {
        last[c as usize] = i as i32;
    }
    let mut s = 0i32;
    while (s as usize) + m <= n {
        let mut j = (m - 1) as i32;
        while j >= 0 && pat[j as usize] == text[s as usize + j as usize] {
            j -= 1;
        }
        if j < 0 {
            return Some(s as usize);
        }
        let bc = last[text[s as usize + j as usize] as usize];
        s += (j - bc).max(1);
    }
    None
}

fn main() {
    if let Some(i) = boyer_moore(b"zzabc", b"abc") {
        println!("{}", i);
    }
}
