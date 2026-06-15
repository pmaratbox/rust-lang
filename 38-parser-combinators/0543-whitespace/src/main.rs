use nom::character::complete::{digit1, multispace0};
use nom::combinator::map;
use nom::sequence::delimited;
use nom::IResult;

// `delimited(multispace0, digit1, multispace0)` runs three combinators in
// sequence and keeps only the middle one's result: it skips leading
// whitespace, parses one-or-more digits, then skips trailing whitespace.
// `map` turns the matched digit slice into an i32.
fn parse_integer_ws(input: &str) -> IResult<&str, i32> {
    map(delimited(multispace0, digit1, multispace0), |s: &str| {
        s.parse::<i32>().unwrap()
    })(input)
}

fn main() {
    let value = parse_integer_ws("  42  ").unwrap().1;
    println!("{}", value);
}
