use nom::character::complete::{char, digit1};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::IResult;

// A single integer: digit1 matched, then mapped to i32.
fn integer(input: &str) -> IResult<&str, i32> {
    map(digit1, |d: &str| d.parse::<i32>().unwrap())(input)
}

// A '+'-separated list of integers, mapped to their sum.
fn expression(input: &str) -> IResult<&str, i32> {
    map(separated_list1(char('+'), integer), |nums| {
        nums.into_iter().sum::<i32>()
    })(input)
}

fn main() {
    let result = expression("10+20+30").unwrap().1;
    println!("{}", result);
}
