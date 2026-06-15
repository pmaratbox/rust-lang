use nom::character::complete::digit1;
use nom::combinator::map;
use nom::IResult;

// Build a parser for one-or-more digits with `digit1`, then `map` the matched
// slice into an i32. The whole thing is a combinator that yields the integer.
fn parse_integer(input: &str) -> IResult<&str, i32> {
    map(digit1, |s: &str| s.parse::<i32>().unwrap())(input)
}

fn main() {
    let value = parse_integer("42").unwrap().1;
    println!("{}", value);
}
