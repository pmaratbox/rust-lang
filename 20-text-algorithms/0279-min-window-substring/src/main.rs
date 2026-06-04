fn min_window(s: &[u8], t: &[u8]) -> String {
    let mut need = [0i32; 128];
    for &c in t {
        need[c as usize] += 1;
    }
    let mut missing = t.len();
    let mut left = 0;
    let mut best: Option<(usize, usize)> = None;
    for right in 0..s.len() {
        let c = s[right] as usize;
        if need[c] > 0 {
            missing -= 1;
        }
        need[c] -= 1;
        while missing == 0 {
            if best.map_or(true, |(l, r)| right + 1 - left < r - l) {
                best = Some((left, right + 1));
            }
            let lc = s[left] as usize;
            need[lc] += 1;
            if need[lc] > 0 {
                missing += 1;
            }
            left += 1;
        }
    }
    match best {
        Some((l, r)) => String::from_utf8(s[l..r].to_vec()).unwrap(),
        None => String::new(),
    }
}

fn main() {
    println!("{}", min_window(b"ADOBECODEBANC", b"ABC"));
}
