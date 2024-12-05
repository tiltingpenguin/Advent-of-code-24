use std::fs;

fn main() {
    let path = "day05-input.txt";
    let error_msg = "Should be able to read input file";
    let input = fs::read_to_string(path).expect(error_msg);
    let result = process(input);
    println!("{}", result);
}

fn ordering(a: u64, b: u64, rules: &Vec<(u64, u64)>) -> bool {
    if rules.contains(&(a, b)) {
        return true;
    } else if rules.contains(&(b, a)) {
        return false;
    }
    return true;
}

pub fn process(input: String) -> u64 {
    let (r, m) = input.split_once("\n\n").unwrap();
    let ru: Vec<&str> = r.lines().collect();
    let mut rul: Vec<String> = vec![];
    for s in ru.iter() {
        rul.push(s.to_string());
    }
    let rules: Vec<(&str, &str)> = rul
        .iter()
        .map(|pair| pair.split_once("|").unwrap())
        .collect();
    let newrules: Vec<(u64, u64)> = rules
        .iter()
        .map(|(left, right)| (left.parse::<u64>().unwrap(), right.parse::<u64>().unwrap()))
        .collect();
    let man: Vec<&str> = m.lines().collect();
    let manual: Vec<Vec<u64>> = man
        .iter()
        .map(|line| {
            line.split(",")
                .map(|num| num.parse::<u64>().unwrap())
                .collect()
        })
        .collect();
    let ordered: Vec<&Vec<u64>> = manual
        .iter()
        .filter(|line| line.is_sorted_by(|a, b| ordering(*a, *b, &newrules)))
        .collect();
    return ordered
        .iter()
        .map(|line| {
            let middle = line.len() / 2;
            dbg!(&line);
            return line[middle];
        })
        .sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day05_part1_test() {
        let path = "day05-test.txt";
        let test_input = fs::read_to_string(path).expect("Should be able to read input");
        let result = process(test_input);
        assert_eq!(result, 143);
    }
}
