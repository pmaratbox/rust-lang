use std::cell::Cell;

fn counted<'a, F: Fn() + 'a>(f: F, counter: &'a Cell<u32>) -> impl Fn() + 'a {
    move || {
        counter.set(counter.get() + 1);
        f();
    }
}

fn main() {
    let counter = Cell::new(0u32);
    let wrapped = counted(|| {}, &counter);
    for _ in 0..5 {
        wrapped();
    }
    println!("calls: {}", counter.get());
}
