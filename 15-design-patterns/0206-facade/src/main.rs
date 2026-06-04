struct Sub1;
struct Sub2;
struct Sub3;

impl Sub1 {
    fn init(&self) {}
}
impl Sub2 {
    fn init(&self) {}
}
impl Sub3 {
    fn init(&self) {}
}

struct Facade {
    s1: Sub1,
    s2: Sub2,
    s3: Sub3,
}

impl Facade {
    fn start(&self) -> &str {
        self.s1.init();
        self.s2.init();
        self.s3.init();
        "ready"
    }
}

fn main() {
    let facade = Facade {
        s1: Sub1,
        s2: Sub2,
        s3: Sub3,
    };
    println!("{}", facade.start());
}
