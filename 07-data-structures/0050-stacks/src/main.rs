fn main() {
    let mut stack = Vec::new();
    for n in [1, 2, 3] {
        stack.push(n);
    }

    let mut popped = Vec::new();
    while let Some(top) = stack.pop() {
        popped.push(top.to_string());
    }

    println!("{}", popped.join(" "));
}
