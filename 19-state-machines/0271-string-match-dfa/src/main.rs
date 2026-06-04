fn main() {
    let pattern = b"ab";
    let text = b"aab";
    // DFA state = number of pattern chars matched so far.
    let mut state = 0usize;
    let mut start = None;
    for (i, &ch) in text.iter().enumerate() {
        if state < pattern.len() && ch == pattern[state] {
            state += 1;
        } else if ch == pattern[0] {
            state = 1;
        } else {
            state = 0;
        }
        if state == pattern.len() {
            start = Some(i + 1 - pattern.len());
            break;
        }
    }
    println!("{}", start.unwrap());
}
