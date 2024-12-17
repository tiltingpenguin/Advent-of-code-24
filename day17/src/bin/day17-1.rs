use nom::bytes::complete::tag;
use nom::character::complete;
use nom::sequence::preceded;
use nom::{multi::separated_list1, IResult};
use std::fs;

fn main() {
    let path = "day17-input.txt";
    let error_msg = "Should be able to read input file";
    let input = fs::read_to_string(path).expect(error_msg);
    let result = process(input);
    println!("{}", result);
}

#[derive(Debug)]
struct Registers {
    a: i64,
    b: i64,
    c: i64,
    output: Vec<usize>,
    pointer: usize,
}

fn parse_regs(line: &str) -> Vec<i64> {
    let result = line
        .to_string()
        .lines()
        .map(|line| line.split_once(": ").unwrap().1.parse::<i64>().unwrap())
        .collect();
    result
}

fn parse_ins(line: &str) -> IResult<&str, Vec<i64>> {
    preceded(tag("Program: "), separated_list1(tag(","), complete::i64))(line)
}

fn combo(regs: &Registers, operand: i64) -> u32 {
    let result = match operand {
        0..4 => operand,
        4 => regs.a,
        5 => regs.b,
        6 => regs.c,
        _ => unreachable!(),
    };
    return result as u32;
}

fn div(regs: &Registers, operand: i64) -> i64 {
    ((regs.a as f64) / (2i64.pow(combo(regs, operand)) as f64)).floor() as i64
}

fn exec(mut regs: &mut Registers, ins_pointer: i64, operand: i64) -> () {
    match ins_pointer {
        0 => regs.a = div(&regs, operand),
        1 => regs.b = regs.b ^ operand,
        2 => regs.b = (combo(&mut regs, operand) % 8) as i64,
        3 => {
            if regs.a == 0 {
                regs.pointer += 2;
                return;
            } else {
                regs.pointer = operand as usize;
                return;
            }
        }
        4 => regs.b = regs.b ^ regs.c,
        5 => {
            regs.output.push((combo(&regs, operand) % 8) as usize);
        }
        6 => regs.b = div(&regs, operand),
        7 => regs.c = div(&regs, operand),
        _ => unreachable!(),
    };
    regs.pointer += 2;
    return;
}

pub fn process(input: String) -> String {
    let (r, i) = input.split_once("\n\n").unwrap();
    let l = parse_regs(r);
    let mut regs = Registers {
        a: l[0],
        b: l[1],
        c: l[2],
        output: Vec::new(),
        pointer: 0,
    };
    let ins = parse_ins(i).unwrap().1;
    loop {
        if ins.get(regs.pointer).is_some() {
            let ins_pointer = *(ins.get(regs.pointer).unwrap());
            let operand = ins
                .get(regs.pointer + 1)
                .expect("A operand should always follow an instruction");
            exec(&mut regs, ins_pointer, *operand);
        } else {
            break;
        }
    }
    let result = regs
        .output
        .iter()
        .fold("".to_string(), |acc, x| format!("{acc},{x}"));
    return result[1..].to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day17_part1_test() {
        let path = "day17-test.txt";
        let test_input = fs::read_to_string(path).expect("Should be able to read input");
        let result = process(test_input);
        assert_eq!(result, "4,6,3,5,6,3,5,2,1,0".to_string());
    }
}
