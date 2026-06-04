use std::sync::atomic::{AtomicUsize, Ordering};

static COUNT: AtomicUsize = AtomicUsize::new(0);

struct Widget;

impl Widget {
    fn new() -> Widget {
        COUNT.fetch_add(1, Ordering::SeqCst);
        Widget
    }
}

fn main() {
    let _a = Widget::new();
    let _b = Widget::new();
    let _c = Widget::new();
    println!("{}", COUNT.load(Ordering::SeqCst));
}
