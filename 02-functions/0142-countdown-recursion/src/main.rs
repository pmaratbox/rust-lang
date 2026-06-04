fn countdown(n: u32, out: &mut Vec<String>) {
    if n == 0 {
        return;
    }
    out.push(n.to_string());
    countdown(n - 1, out);
}

fn main() {
    let mut out = Vec::new();
    countdown(5, &mut out);
    println!("{}", out.join(" "));
}
