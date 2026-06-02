fn is_balanced(s: &str) -> bool {
    let mut stack = Vec::new();
    for ch in s.chars() {
        if ch == '(' {
            stack.push(ch);
        } else if ch == ')' {
            if stack.pop().is_none() {
                return false;
            }
        }
    }
    stack.is_empty()
}

fn main() {
    for s in ["(())", "(()"] {
        println!("{}", if is_balanced(s) { "yes" } else { "no" });
    }
}
