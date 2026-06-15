use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::IResult;

// A choice parser: try `tag("cat")`, otherwise `tag("dog")`.
// `alt` runs each alternative in order and returns the first that succeeds.
fn cat_or_dog(input: &str) -> IResult<&str, &str> {
    alt((tag("cat"), tag("dog")))(input)
}

fn main() {
    let value = cat_or_dog("dog").unwrap().1;
    println!("{}", value);
}
