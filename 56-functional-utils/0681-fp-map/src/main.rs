use itertools::Itertools;

fn main() {
    // Map x -> x*2 over [1,2,3] using the iterator's map adaptor,
    // then comma-join the results with itertools' `.join`.
    let result = [1, 2, 3].iter().map(|x| x * 2).join(",");
    println!("{}", result);
}
