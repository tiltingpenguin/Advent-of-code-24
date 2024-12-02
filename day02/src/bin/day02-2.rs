use std::fs;

fn main() {
    let path = "day02-input.txt";
    let error_msg = "Should be able to read input file";
    let input = fs::read_to_string(path).expect(error_msg);
    let result = process(input);
    println!("{}", result);
}

fn filter(line: Vec<i64>) -> bool {
    if !line.iter().is_sorted() && !line.iter().rev().is_sorted() {
        return false;
    }
    let mut diffs: Vec<i64> = Vec::new();
    let peek = (*line).to_vec().clone();
    for (index, num) in line.to_vec().iter().enumerate() {
        if index + 1 >= peek.len() {
            break;
        }
        diffs.push(num - peek[index + 1])
    }
    let min = *diffs.iter().min().unwrap();
    let max = *diffs.iter().max().unwrap();
    if line.iter().is_sorted() {
        if min < -3 || max > -1 {
            return false;
        } else {
            return true;
        }
    } else {
        if max > 3 || min < 1 {
            return false;
        } else {
            return true;
        }
    }
}

pub fn process(input: String) -> usize {
    let list: Vec<&str> = input.lines().collect();
    let intlist: Vec<Vec<i64>> = list
        .iter()
        .map(|x| {
            x.split_whitespace()
                .map(|y| y.parse::<i64>().unwrap())
                .collect()
        })
        .collect();
    let result: usize = intlist
        .iter()
        .filter(|line| {
            let mut options: Vec<Vec<i64>> = vec![];
            let copy: Vec<i64> = line.iter().cloned().collect();
            options.push(copy.clone());
            let mut tmp_arr: Vec<i64>;
            let length: Vec<usize> = (0..copy.len()).collect();
            for i in length.iter() {
                tmp_arr = copy.clone();
                tmp_arr.remove((*i) as usize);
                options.push(tmp_arr);
            }
            let mut results: Vec<bool> = Vec::new();
            for j in options {
                results.push(filter(j))
            }
            return results.contains(&true);
        })
        .collect::<Vec<&Vec<i64>>>()
        .len();
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day02_part2_test() {
        let path = "day02-test.txt";
        let test_input = fs::read_to_string(path).expect("Should be able to read input");
        let result = process(test_input);
        assert_eq!(result, 4);
    }
}
