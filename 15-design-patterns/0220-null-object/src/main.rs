trait Logger {
    fn log(&self, count: &mut i32);
}

struct NullLogger;
impl Logger for NullLogger {
    fn log(&self, _count: &mut i32) {}
}

struct RealLogger;
impl Logger for RealLogger {
    fn log(&self, count: &mut i32) {
        *count += 1;
    }
}

fn main() {
    let mut count = 0;
    let null: Box<dyn Logger> = Box::new(NullLogger);
    let real: Box<dyn Logger> = Box::new(RealLogger);
    null.log(&mut count);
    real.log(&mut count);
    println!("{}", count);
}
