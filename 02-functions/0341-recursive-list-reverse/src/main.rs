fn reverse(xs: &[i32]) -> Vec<i32> {
    match xs {
        [] => Vec::new(),
        [head, tail @ ..] => {
            let mut rest = reverse(tail);
            rest.push(*head);
            rest
        }
    }
}

fn main() {
    let reversed = reverse(&[1, 2, 3]);
    let parts: Vec<String> = reversed.iter().map(|n| n.to_string()).collect();
    println!("{}", parts.join(" "));
}
