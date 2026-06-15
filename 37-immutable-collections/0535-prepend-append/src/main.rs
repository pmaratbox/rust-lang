use im::Vector;

fn main() {
    // Persistent immutable list from the `im` crate.
    let original: Vector<i32> = im::vector![2, 3];

    // Prepend 1: clone shares structure, push_front mutates only the clone.
    let mut prepended = original.clone();
    prepended.push_front(1);

    // Append 4 onto the prepended list, again on a fresh clone.
    let mut final_list = prepended.clone();
    final_list.push_back(4);

    // The new list reflects both updates; the original [2, 3] is untouched.
    println!(
        "{}",
        final_list
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
