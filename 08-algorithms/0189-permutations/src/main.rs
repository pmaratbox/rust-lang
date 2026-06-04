fn build(prefix: &mut Vec<i32>, remaining: &[i32]) {
    if remaining.is_empty() {
        let parts: Vec<String> = prefix.iter().map(|x| x.to_string()).collect();
        println!("{}", parts.join(" "));
        return;
    }
    for i in 0..remaining.len() {
        prefix.push(remaining[i]);
        let mut rest = remaining.to_vec();
        rest.remove(i);
        build(prefix, &rest);
        prefix.pop();
    }
}

fn main() {
    build(&mut Vec::new(), &[1, 2, 3]);
}
