use itertools::Itertools;

fn main() {
    // itertools' `.join` over a std `filter` keeps only the even numbers.
    let result = (1..=6).filter(|x| x % 2 == 0).join(",");
    println!("{}", result);
}
