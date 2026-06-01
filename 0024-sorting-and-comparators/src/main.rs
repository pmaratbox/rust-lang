fn join(v: &[i32]) -> String {
    v.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" ")
}

fn main() {
    let mut asc = vec![3, 1, 2];
    asc.sort();
    let mut desc = vec![3, 1, 2];
    desc.sort_by(|a, b| b.cmp(a));
    println!("asc: {}", join(&asc));
    println!("desc: {}", join(&desc));
}
