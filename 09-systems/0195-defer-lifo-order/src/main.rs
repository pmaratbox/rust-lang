struct Defer(u32, bool);

impl Drop for Defer {
    fn drop(&mut self) {
        // Drop runs in reverse declaration order, giving LIFO semantics.
        if self.1 {
            print!("{} ", self.0);
        } else {
            print!("{}", self.0);
        }
    }
}

fn main() {
    // RAII guards drop in last-in-first-out order: _c, then _b, then _a.
    let _a = Defer(1, false);
    let _b = Defer(2, true);
    let _c = Defer(3, true);
    // Drops happen here at end of scope: prints "3 2 1".
}
