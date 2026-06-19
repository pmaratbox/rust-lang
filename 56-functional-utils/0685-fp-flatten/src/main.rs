use itertools::Itertools;

fn main() {
    let nested = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    // Flatten one level via the standard iterator adapter, then join with itertools.
    let flat = nested.into_iter().flatten().join(",");
    println!("{}", flat);
}
