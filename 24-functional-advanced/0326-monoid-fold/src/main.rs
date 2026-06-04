fn fold_monoid<T, F>(items: &[T], identity: T, combine: F) -> T
where
    T: Clone,
    F: Fn(T, &T) -> T,
{
    items.iter().fold(identity, |acc, x| combine(acc, x))
}

fn main() {
    let words = ["a".to_string(), "b".to_string(), "c".to_string()];
    let concatenated = fold_monoid(&words, String::new(), |acc, x| acc + x);

    let nums = [1, 2, 3];
    let total = fold_monoid(&nums, 0, |acc, x| acc + x);

    println!("{} {}", concatenated, total);
}
