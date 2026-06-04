fn test_add() -> bool {
    1 + 1 == 2
}

fn test_mul() -> bool {
    2 * 3 == 6
}

fn test_str() -> bool {
    "ab".len() == 2
}

fn main() {
    let tests: Vec<(&str, fn() -> bool)> = vec![
        ("test_add", test_add),
        ("test_mul", test_mul),
        ("test_str", test_str),
    ];

    let mut passed = 0;
    let mut failed = 0;
    for (_name, t) in &tests {
        if t() {
            passed += 1;
        } else {
            failed += 1;
        }
    }

    println!("{passed} passed, {failed} failed");
}
