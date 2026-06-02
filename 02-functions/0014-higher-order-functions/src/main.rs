fn inc(x: i32) -> i32 {
    x + 1
}

fn double(x: i32) -> i32 {
    x * 2
}

fn apply(f: fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}

fn main() {
    println!("inc 5 = {}", apply(inc, 5));
    println!("double 5 = {}", apply(double, 5));
}
