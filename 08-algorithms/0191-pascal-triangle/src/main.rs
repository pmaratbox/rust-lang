fn main() {
    let mut row = vec![1];
    for _ in 0..4 {
        let parts: Vec<String> = row.iter().map(|x| x.to_string()).collect();
        println!("{}", parts.join(" "));
        let mut next = vec![1];
        for w in row.windows(2) {
            next.push(w[0] + w[1]);
        }
        next.push(1);
        row = next;
    }
}
