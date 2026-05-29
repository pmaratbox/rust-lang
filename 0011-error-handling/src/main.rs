fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err(String::from("division by zero"));
    }
    Ok(a / b)
}

fn main() {
    match divide(10, 2) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("error: {}", e),
    }

    match divide(10, 0) {
        Ok(result) => println!("10 / 0 = {}", result),
        Err(e) => println!("error: {}", e),
    }
}
