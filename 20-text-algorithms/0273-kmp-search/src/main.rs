fn prefix_function(p: &[u8]) -> Vec<usize> {
    let mut pi = vec![0usize; p.len()];
    let mut k = 0;
    for i in 1..p.len() {
        while k > 0 && p[i] != p[k] {
            k = pi[k - 1];
        }
        if p[i] == p[k] {
            k += 1;
        }
        pi[i] = k;
    }
    pi
}

fn main() {
    let text = b"ababab";
    let pat = b"ab";
    let pi = prefix_function(pat);
    let mut k = 0;
    let mut hits = Vec::new();
    for i in 0..text.len() {
        while k > 0 && text[i] != pat[k] {
            k = pi[k - 1];
        }
        if text[i] == pat[k] {
            k += 1;
        }
        if k == pat.len() {
            hits.push(i + 1 - pat.len());
            k = pi[k - 1];
        }
    }
    let out: Vec<String> = hits.iter().map(|x| x.to_string()).collect();
    println!("{}", out.join(" "));
}
