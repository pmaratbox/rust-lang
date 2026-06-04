fn main() {
    let expr = "3 4 + 5 *";
    let mut stack: Vec<i64> = Vec::new();
    for tok in expr.split_whitespace() {
        match tok {
            "+" | "-" | "*" | "/" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                let r = match tok {
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    _ => a / b,
                };
                stack.push(r);
            }
            n => stack.push(n.parse().unwrap()),
        }
    }
    println!("{}", stack.pop().unwrap());
}
