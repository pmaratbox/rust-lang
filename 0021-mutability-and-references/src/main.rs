fn inc(n: &mut i32) {
    *n += 1;
}

fn main() {
    let mut n = 1;
    println!("before: {n}");
    inc(&mut n);
    println!("after: {n}");
}
