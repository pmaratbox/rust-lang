fn apply_twice<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(f(x))
}

fn inc(x: i32) -> i32 {
    x + 1
}

fn main() {
    println!("{}", apply_twice(inc, 3));
}
