use nom::character::complete::char;
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;

// Sequence combinator: run `char('a')` THEN `char('b')`, combining both
// matched characters into a single owned String.
fn ab(input: &str) -> IResult<&str, String> {
    map(tuple((char('a'), char('b'))), |(a, b)| {
        let mut s = String::new();
        s.push(a);
        s.push(b);
        s
    })(input)
}

fn main() {
    let result = ab("ab").unwrap().1;
    println!("{}", result);
}
