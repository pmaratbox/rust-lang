trait Animal {
    fn speak(&self) -> String {
        "some sound".to_string()
    }
}

struct Generic;
struct Dog;

impl Animal for Generic {}

impl Animal for Dog {
    fn speak(&self) -> String {
        "Woof".to_string()
    }
}

fn main() {
    println!("animal: {}", Generic.speak());
    println!("dog: {}", Dog.speak());
}
