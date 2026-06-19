use itertools::Itertools;

fn main() {
    // Split 1..=6 into fixed-size chunks of 2 using itertools' `.chunks`,
    // join each chunk with ',' and the chunks with '|' via `.join`.
    let result = (1..=6)
        .chunks(2)
        .into_iter()
        .map(|c| c.map(|x| x.to_string()).join(","))
        .join("|");
    println!("{}", result);
}
