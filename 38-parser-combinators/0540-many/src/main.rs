use nom::character::complete::char;
use nom::multi::many0;
use nom::IResult;

fn main() {
    // `many0` runs the inner `char('a')` parser zero or more times,
    // collecting each successful match into a Vec.
    let r: IResult<&str, Vec<char>> = many0(char('a'))("aaaa");
    let parsed = r.unwrap().1;
    println!("{}", parsed.len());
}
