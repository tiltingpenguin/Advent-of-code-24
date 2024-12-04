use nom::{
    bytes::complete::tag,
    character::{complete::anychar, streaming},
    multi::{many1, many_till},
    sequence::{preceded, separated_pair, terminated},
    IResult, Parser,
};
use std::fs;

fn main() {
    let path = "day03-input.txt";
    let error_msg = "Should be able to read input file";
    let input = fs::read_to_string(path).expect(error_msg);
    let result = process(input);
    println!("{}", result);
}

fn parse(input: &str) -> IResult<&str, Vec<(u64, u64)>> {
    many1(many_till(anychar, mul).map(|(_junk, instruction)| instruction))(input)
}

fn mul(input: &str) -> IResult<&str, (u64, u64)> {
    let (input, parsed) = preceded(
        tag("mul("),
        terminated(
            separated_pair(streaming::u64, tag(","), streaming::u64),
            tag(")"),
        ),
    )(input)?;
    return Ok((input, (parsed.0, parsed.1)));
}

pub fn process(input: String) -> u64 {
    let muls = parse(&input);
    let result = muls.unwrap().1.iter().map(|pair| pair.0 * pair.1).sum();
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day03_part1_test() {
        let path = "day03-test.txt";
        let test_input = fs::read_to_string(path).expect("Should be able to read input");
        let result = process(test_input);
        assert_eq!(result, 161);
    }
}
