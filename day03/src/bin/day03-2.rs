use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{complete::anychar, streaming},
    combinator::value,
    multi::{many1, many_till},
    sequence::{preceded, separated_pair, terminated},
    IResult, Parser,
};
use std::fs;

#[derive(Debug, Clone, PartialEq)]
enum Instruction {
    Pair(u64, u64),
    True,
    False,
}

fn main() {
    let path = "day03-input.txt";
    let error_msg = "Should be able to read input file";
    let input = fs::read_to_string(path).expect(error_msg);
    let result = process(input);
    println!("{}", result);
}

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(many_till(anychar, cond).map(|(_junk, instruction)| instruction))(input)
}

fn mul(input: &str) -> IResult<&str, Instruction> {
    let (input, parsed) = preceded(
        tag("mul("),
        terminated(
            separated_pair(streaming::u64, tag(","), streaming::u64),
            tag(")"),
        ),
    )(input)?;
    return Ok((input, Instruction::Pair(parsed.0, parsed.1)));
}

fn cond(input: &str) -> IResult<&str, Instruction> {
    let instruction = alt((
        value(Instruction::False, tag("don't()")),
        value(Instruction::True, tag("do()")),
        mul,
    ))(input);
    return instruction;
}

pub fn process(input: String) -> u64 {
    let (_rest, instructions) = parse(&input).unwrap();
    let result = instructions
        .iter()
        .fold(
            (Instruction::True, 0),
            |(action, acc), instruction| match instruction {
                Instruction::Pair(a, b) => {
                    if action == Instruction::True {
                        (action, acc + a * b)
                    } else {
                        (action, acc)
                    }
                }
                Instruction::True => (Instruction::True, acc),
                Instruction::False => (Instruction::False, acc),
            },
        )
        .1;
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day03_part2_test() {
        // I got stuck on this so long because I didn't notice the test case for part 2 was
        // different
        let path = "day03-2-test.txt";
        let test_input = fs::read_to_string(path).expect("Should be able to read input");
        let result = process(test_input);
        assert_eq!(result, 48);
    }
}
