use nom::bytes::complete::tag;
use nom::multi::many1;
use nom::IResult;
use std::fs;

fn main() {
    let path = "day11-input.txt";
    let error_msg = "Should be able to read input file";
    let input = fs::read_to_string(path).expect(error_msg);
    let result = process(input);
    println!("{}", result);
}

fn parser(i: &str) -> IResult<&str, Vec<&str>> {
    many1(tag("0"))(i)
}

pub fn process(input: String) -> usize {
    let mut list: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();
    for _ in 0..25 {
        list = list
            .iter()
            .map(|x| {
                if *x == "0" || *x == "" {
                    return vec!["1".to_string()];
                }
                if x.len() % 2 == 0 {
                    let left = x[..(x.len() / 2)].to_string();
                    let mut right = x[x.len() / 2..].to_string();
                    if parser(&right).is_ok() {
                        right = parser(&right).unwrap().0.to_string();
                    }
                    return vec![left, right];
                }
                let tmp = (x.parse::<u64>().unwrap() * 2024).to_string();
                return vec![tmp];
            })
            .flatten()
            .collect();
    }
    return list.len();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day11_part1_test() {
        let path = "day11-test.txt";
        let test_input = fs::read_to_string(path).expect("Should be able to read input");
        let result = process(test_input);
        assert_eq!(result, 55312);
    }
}
