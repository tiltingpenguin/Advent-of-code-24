use array2d::Array2D;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::sequence::{preceded, separated_pair};
use nom::IResult;
use std::fs;

fn main() {
    let path = "day14-input.txt";
    let error_msg = "Should be able to read input file";
    let input = fs::read_to_string(path).expect(error_msg);
    let dim = (101, 103);
    let result = process(input, dim);
    println!("{:?}", result);
}

fn parse(line: &str) -> IResult<&str, ((i32, i32), (i32, i32))> {
    preceded(
        tag("p="),
        separated_pair(
            separated_pair(complete::i32, tag(","), complete::i32),
            tag(" v="),
            separated_pair(complete::i32, tag(","), complete::i32),
        ),
    )(line)
}

pub fn process(input: String, dim: (i32, i32)) -> usize {
    let robots: Vec<((i32, i32), (i32, i32))> =
        input.lines().map(|line| parse(line).unwrap().1).collect();
    let final_pos: Vec<(i32, i32)> = robots
        .iter()
        .map(|r| {
            let mut rx = (r.0 .0 + (r.1 .0 * 100)) % dim.0;
            let mut ry = (r.0 .1 + (r.1 .1 * 100)) % dim.1;
            if rx < 0 {
                rx = dim.0 - rx.abs();
            }
            if ry < 0 {
                ry = dim.1 - ry.abs();
            }
            (rx, ry)
        })
        .collect();
    let mut upleft = 0;
    let mut upright = 0;
    let mut downleft = 0;
    let mut downright = 0;
    for pos in final_pos.iter() {
        if pos.0 == dim.0 / 2 || pos.1 == dim.1 / 2 {
            continue;
        }
        if pos.0 < dim.0 / 2 {
            if pos.1 < dim.1 / 2 {
                upleft += 1;
            } else {
                downleft += 1;
            }
        } else {
            if pos.1 < dim.1 / 2 {
                upright += 1;
            } else {
                downright += 1;
            }
        }
    }
    return upleft * upright * downleft * downright;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day14_part1_test() {
        let path = "day14-test.txt";
        let test_input = fs::read_to_string(path).expect("Should be able to read input");
        let result = process(test_input, (11, 7));
        assert_eq!(result, 12);
    }
}
