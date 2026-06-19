use itertools::Itertools;

fn main() {
    // Map each x to the small list [x, x*10] and flatten in one pass using the
    // standard iterator's .flat_map adapter, then comma-join with itertools.
    let out = [1, 2, 3].iter().flat_map(|x| vec![*x, x * 10]).join(",");
    println!("{}", out);
}
