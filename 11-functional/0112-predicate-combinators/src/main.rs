fn and<F, G>(f: F, g: G) -> impl Fn(i32) -> bool
where
    F: Fn(i32) -> bool,
    G: Fn(i32) -> bool,
{
    move |x| f(x) && g(x)
}

fn main() {
    let is_even = |x: i32| x % 2 == 0;
    let is_positive = |x: i32| x > 0;
    let pred = and(is_even, is_positive);

    let label = |b: bool| if b { "yes" } else { "no" };
    println!("{} {}", label(pred(4)), label(pred(-4)));
}
