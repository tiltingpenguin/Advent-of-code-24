use std::fs;

/*
Holy shit, this passed first try!
*/

fn main() {
    let path = "day07-input.txt";
    let error_msg = "Should be able to read input file";
    let input = fs::read_to_string(path).expect(error_msg);
    let result = process(input);
    println!("{}", result);
}

fn branch(line1: &(u64, Vec<u64>)) -> bool {
    let line = line1.clone();
    if line.1.len() == 1 {
        return line.0 == line.1[0];
    } else if line.1.len() > 1 {
        let first = line.1[0];
        let sec = line.1[1];
        let rest = line.1[2..].to_vec();
        let sum_res = branch(&(line.0, {
            let mut tmp = vec![first + sec];
            tmp.append(&mut rest.clone());
            tmp
        }));
        let mul_res = branch(&(line.0, {
            let mut tmp = vec![first * sec];
            tmp.append(&mut rest.clone());
            tmp
        }));
        return sum_res || mul_res;
    } else {
        unreachable!("Vec should alsways be longer than 0");
    }
}

pub fn process(input: String) -> u64 {
    let lines: Vec<&str> = input.lines().collect();
    let equations: Vec<(u64, Vec<u64>)> = lines
        .iter()
        .map(|line| line.split_once(":").unwrap())
        .map(|line| {
            (
                line.0.parse::<u64>().unwrap(),
                line.1
                    .split_whitespace()
                    .map(|num| num.parse::<u64>().unwrap())
                    .collect(),
            )
        })
        .collect();
    return equations
        .iter()
        .filter(|line| branch(line))
        .map(|l| l.0)
        .sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day07_part1_test() {
        let path = "day07-test.txt";
        let test_input = fs::read_to_string(path).expect("Should be able to read input");
        let result = process(test_input);
        assert_eq!(result, 3749);
    }
}
