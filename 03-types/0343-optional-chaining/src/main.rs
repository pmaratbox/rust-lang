struct A {
    b: Option<B>,
}

struct B {
    c: Option<i64>,
}

fn deep(a: &A) -> i64 {
    a.b.as_ref().and_then(|b| b.c).unwrap_or(0)
}

fn main() {
    let present = A {
        b: Some(B { c: Some(5) }),
    };
    let absent = A { b: None };
    println!("{} {}", deep(&present), deep(&absent));
}
