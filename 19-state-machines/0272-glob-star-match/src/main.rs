fn glob_match(pattern: &[u8], text: &[u8]) -> bool {
    if pattern.is_empty() {
        return text.is_empty();
    }
    match pattern[0] {
        b'*' => {
            // match zero or more characters
            glob_match(&pattern[1..], text)
                || (!text.is_empty() && glob_match(pattern, &text[1..]))
        }
        c => !text.is_empty() && text[0] == c && glob_match(&pattern[1..], &text[1..]),
    }
}

fn main() {
    let label = |b: bool| if b { "yes" } else { "no" };
    println!(
        "{} {}",
        label(glob_match(b"a*b", b"aaab")),
        label(glob_match(b"a*b", b"aac"))
    );
}
