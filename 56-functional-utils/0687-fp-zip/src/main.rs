use itertools::Itertools;

fn main() {
    let result = [1, 2, 3]
        .iter()
        .zip(["a", "b", "c"].iter())
        .map(|(n, s)| format!("{}{}", n, s))
        .join(",");
    println!("{}", result);
}
