use im::Vector;

fn main() {
    // Persistent immutable list from the `im` crate.
    let original: Vector<i32> = im::vector![1, 2, 3, 4, 5];

    // filter then map each return a brand-new immutable Vector;
    // `original` is never mutated thanks to structural sharing.
    let result: Vector<i32> = original
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * 10)
        .collect();

    println!(
        "{}",
        result
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
