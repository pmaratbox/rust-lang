fn pipe<A: 'static>(fns: Vec<Box<dyn Fn(A) -> A>>) -> impl Fn(A) -> A {
    move |x| fns.iter().fold(x, |acc, f| f(acc))
}

fn main() {
    let inc = Box::new(|x: i32| x + 1) as Box<dyn Fn(i32) -> i32>;
    let double = Box::new(|x: i32| x * 2) as Box<dyn Fn(i32) -> i32>;
    let neg = Box::new(|x: i32| -x) as Box<dyn Fn(i32) -> i32>;

    let p = pipe(vec![inc, double, neg]);
    println!("{}", p(3));
}
