use nom::{
    self,
    branch::alt,
    bytes::complete::tag,
    character::complete,
    sequence::{preceded, separated_pair},
    IResult,
};
use std::fs;

fn main() {
    let path = "day13-input.txt";
    let error_msg = "Should be able to read input file";
    let input = fs::read_to_string(path).expect(error_msg);
    let result = process(input);
    println!("{}", result);
}

fn parse(line: &str) -> IResult<&str, (i64, i64)> {
    alt((
        (preceded(
            tag("Button A: X+"),
            separated_pair(complete::i64, tag(", Y+"), complete::i64),
        )),
        (preceded(
            tag("Button B: X+"),
            separated_pair(complete::i64, tag(", Y+"), complete::i64),
        )),
        (preceded(
            tag("Prize: X="),
            separated_pair(complete::i64, tag(", Y="), complete::i64),
        )),
    ))(line)
}

pub fn process(input: String) -> i64 {
    let result: i64;
    let tmp = input.split("\n\n").collect::<Vec<&str>>();
    let machines = tmp
        .iter()
        .map(|machine| {
            machine
                .lines()
                .map(|line| parse(line).unwrap().1)
                .collect::<Vec<(i64, i64)>>()
        })
        .collect::<Vec<Vec<(i64, i64)>>>();
    result = machines
        .iter()
        .map(|machine| {
            let (ax, ay) = machine[0];
            let (bx, by) = machine[1];
            let (mut px, mut py) = machine[2];
            px += 10000000000000;
            py += 10000000000000;
            let det = ax * by - ay * bx;
            if det == 0 {
                return 0;
            }
            let mut a_presses = px * by - py * bx;
            let mut b_presses = ax * py - ay * px;

            if a_presses % det != 0 || b_presses % det != 0 {
                return 0;
            }

            a_presses = a_presses / det;
            b_presses = b_presses / det;

            return a_presses * 3 + b_presses;
        })
        .sum();
    return result;
}
