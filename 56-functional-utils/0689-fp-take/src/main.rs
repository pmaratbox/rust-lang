use itertools::Itertools;

fn main() {
    // Take the first 3 elements of [1..=10] using the iterator's `take` adapter,
    // then comma-join with itertools' `join`.
    println!("{}", (1..=10).take(3).join(","));
}
