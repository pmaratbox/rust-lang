fn first<T>(items: &[T]) -> &T {
    &items[0]
}

fn main() {
    let ints = [1, 2, 3];
    let strs = ["a", "b", "c"];

    println!("first int: {}", first(&ints));
    println!("first string: {}", first(&strs));
}
