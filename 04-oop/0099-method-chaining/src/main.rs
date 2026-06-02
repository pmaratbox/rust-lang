struct Calc {
    value: i32,
}

impl Calc {
    fn new(value: i32) -> Calc {
        Calc { value }
    }
    fn add(mut self, n: i32) -> Calc {
        self.value += n;
        self
    }
    fn multiply(mut self, n: i32) -> Calc {
        self.value *= n;
        self
    }
    fn result(self) -> i32 {
        self.value
    }
}

fn main() {
    println!("{}", Calc::new(5).add(3).multiply(2).result());
}
