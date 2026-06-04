fn main() {
    let a = [3, 1, 2, 3, 1];
    let max = *a.iter().max().unwrap() as usize;
    let mut counts = vec![0usize; max + 1];
    for &x in &a {
        counts[x as usize] += 1;
    }
    let mut sorted = Vec::with_capacity(a.len());
    for (value, &count) in counts.iter().enumerate() {
        for _ in 0..count {
            sorted.push(value.to_string());
        }
    }
    println!("{}", sorted.join(" "));
}
