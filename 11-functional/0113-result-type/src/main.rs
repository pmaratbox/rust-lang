fn safe_div(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn main() {
    for r in [safe_div(10, 2), safe_div(1, 0)] {
        match r {
            Ok(v) => println!("ok: {}", v),
            Err(e) => println!("err: {}", e),
        }
    }
}
