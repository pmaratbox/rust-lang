#[derive(PartialEq)]
struct Pair {
    a: i32,
    b: i32,
}

#[derive(PartialEq)]
struct Nested {
    left: Pair,
    right: Pair,
}

fn main() {
    let x = Nested {
        left: Pair { a: 1, b: 2 },
        right: Pair { a: 3, b: 4 },
    };
    let y = Nested {
        left: Pair { a: 1, b: 2 },
        right: Pair { a: 3, b: 4 },
    };
    let equal = if x == y { "yes" } else { "no" };
    println!("equal: {}", equal);
}
