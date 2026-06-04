#[derive(Clone)]
struct Prototype {
    value: i32,
}

fn main() {
    let original = Prototype { value: 1 };
    let mut clone = original.clone();
    clone.value = 2;
    println!("{} {}", original.value, clone.value);
}
