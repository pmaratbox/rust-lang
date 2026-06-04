fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        Err("err")
    } else {
        Ok(a / b)
    }
}

fn main() {
    let ok = divide(8, 2).and_then(|x| divide(x, 2));
    let bad = divide(8, 0).and_then(|x| divide(x, 2));

    let a = match ok {
        Ok(v) => v.to_string(),
        Err(e) => e.to_string(),
    };
    let b = match bad {
        Ok(v) => v.to_string(),
        Err(e) => e.to_string(),
    };

    println!("{} {}", a, b);
}
