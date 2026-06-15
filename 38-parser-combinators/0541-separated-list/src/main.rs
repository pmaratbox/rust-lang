use nom::character::complete::{char, digit1};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::IResult;

// Parse one integer.
fn integer(input: &str) -> IResult<&str, i32> {
    map(digit1, |s: &str| s.parse::<i32>().unwrap())(input)
}

// Parse integers separated by ','.
fn int_list(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(char(','), integer)(input)
}

fn main() {
    let nums = int_list("1,2,3").unwrap().1;
    let sum: i32 = nums.iter().sum();
    println!("{}", sum);
}
