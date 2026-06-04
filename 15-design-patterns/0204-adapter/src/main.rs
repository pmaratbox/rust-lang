struct CelsiusSource {
    c: i32,
}

trait Fahrenheit {
    fn fahrenheit(&self) -> i32;
}

struct CelsiusAdapter {
    source: CelsiusSource,
}

impl Fahrenheit for CelsiusAdapter {
    fn fahrenheit(&self) -> i32 {
        self.source.c * 9 / 5 + 32
    }
}

fn main() {
    let adapter = CelsiusAdapter {
        source: CelsiusSource { c: 100 },
    };
    println!("{}", adapter.fahrenheit());
}
