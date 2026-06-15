use nom::character::complete::digit1;
use nom::combinator::map;
use nom::IResult;

// Parse one-or-more digits, then MAP the parsed integer to value * 2.
fn doubled(input: &str) -> IResult<&str, i32> {
    map(digit1, |s: &str| s.parse::<i32>().unwrap() * 2)(input)
}

fn main() {
    let value = doubled("21").unwrap().1;
    println!("{}", value);
}
