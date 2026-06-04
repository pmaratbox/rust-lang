fn rabin_karp(text: &[u8], pat: &[u8]) -> Vec<usize> {
    let m = pat.len();
    let n = text.len();
    let mut hits = Vec::new();
    if m == 0 || m > n {
        return hits;
    }
    const BASE: u64 = 256;
    const MOD: u64 = 1_000_000_007;
    let mut high: u64 = 1;
    for _ in 0..m - 1 {
        high = high.wrapping_mul(BASE) % MOD;
    }
    let mut ph: u64 = 0;
    let mut th: u64 = 0;
    for i in 0..m {
        ph = (ph.wrapping_mul(BASE) + pat[i] as u64) % MOD;
        th = (th.wrapping_mul(BASE) + text[i] as u64) % MOD;
    }
    for i in 0..=n - m {
        if ph == th && &text[i..i + m] == pat {
            hits.push(i);
        }
        if i + m < n {
            th = (th + MOD - text[i] as u64 * high % MOD) % MOD;
            th = (th.wrapping_mul(BASE) + text[i + m] as u64) % MOD;
        }
    }
    hits
}

fn main() {
    let hits = rabin_karp(b"xabxab", b"ab");
    let out: Vec<String> = hits.iter().map(|x| x.to_string()).collect();
    println!("{}", out.join(" "));
}
