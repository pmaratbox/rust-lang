use im::Vector;

fn main() {
    // `im::Vector` is a persistent vector: clone is cheap (structural sharing)
    // and mutating the clone leaves the original untouched.
    let original: Vector<i32> = im::vector![1, 2, 3];

    // Adding returns a new list; `original` stays unchanged.
    let mut updated = original.clone();
    updated.push_back(4);

    println!(
        "{}",
        updated
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
    println!(
        "{}",
        original
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
