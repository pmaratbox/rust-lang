use std::fmt;

struct Pizza {
    size: String,
    toppings: Vec<String>,
}

impl fmt::Display for Pizza {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Pizza({}, {})", self.size, self.toppings.join(", "))
    }
}

#[derive(Default)]
struct PizzaBuilder {
    size: String,
    toppings: Vec<String>,
}

impl PizzaBuilder {
    fn set_size(mut self, size: &str) -> Self {
        self.size = size.to_string();
        self
    }

    fn add_topping(mut self, topping: &str) -> Self {
        self.toppings.push(topping.to_string());
        self
    }

    fn build(self) -> Pizza {
        Pizza {
            size: self.size,
            toppings: self.toppings,
        }
    }
}

fn main() {
    let pizza = PizzaBuilder::default()
        .set_size("M")
        .add_topping("cheese")
        .build();
    println!("{}", pizza);
}
