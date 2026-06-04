trait Coffee {
    fn cost(&self) -> i32;
}

struct Base;
impl Coffee for Base {
    fn cost(&self) -> i32 {
        2
    }
}

struct Milk {
    inner: Box<dyn Coffee>,
}
impl Coffee for Milk {
    fn cost(&self) -> i32 {
        self.inner.cost() + 1
    }
}

struct Sugar {
    inner: Box<dyn Coffee>,
}
impl Coffee for Sugar {
    fn cost(&self) -> i32 {
        self.inner.cost() + 1
    }
}

fn main() {
    let coffee: Box<dyn Coffee> = Box::new(Sugar {
        inner: Box::new(Milk {
            inner: Box::new(Base),
        }),
    });
    println!("{}", coffee.cost());
}
