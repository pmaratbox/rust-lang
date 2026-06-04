fn prec(op: char) -> u8 {
    match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        _ => 0,
    }
}

fn main() {
    let expr = "3 + 4 * 2";
    let mut output: Vec<String> = Vec::new();
    let mut ops: Vec<char> = Vec::new();
    for tok in expr.split_whitespace() {
        let c = tok.chars().next().unwrap();
        if c.is_ascii_digit() {
            output.push(tok.to_string());
        } else {
            while let Some(&top) = ops.last() {
                if prec(top) >= prec(c) {
                    output.push(ops.pop().unwrap().to_string());
                } else {
                    break;
                }
            }
            ops.push(c);
        }
    }
    while let Some(op) = ops.pop() {
        output.push(op.to_string());
    }
    println!("{}", output.join(" "));
}
