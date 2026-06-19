use itertools::Itertools;

fn main() {
    // Remove duplicates from [1,2,2,3,3,3] preserving first-seen order
    // using itertools' `.unique` adaptor, then comma-join with `.join`.
    let result = [1, 2, 2, 3, 3, 3].iter().unique().join(",");
    println!("{}", result);
}
