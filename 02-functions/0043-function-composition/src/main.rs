fn inc(x: i32) -> i32 {
    x + 1
}

fn twice(x: i32) -> i32 {
    x * 2
}

fn compose(f: fn(i32) -> i32, g: fn(i32) -> i32) -> impl Fn(i32) -> i32 {
    move |x| f(g(x))
}

fn main() {
    println!("{}", compose(inc, twice)(3));
}
