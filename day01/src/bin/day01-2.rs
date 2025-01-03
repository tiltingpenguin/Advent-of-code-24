use std::fs;

fn main() {
    let path = "day01-input.txt";
    let error_msg = "Should be able to read input file";
    let input = fs::read_to_string(path).expect(error_msg);
    let result = process(input);
    println!("{}", result);
}

fn process(input: String) -> i64 {
    let lines: Vec<&str> = input.lines().collect();
    let pairs: Vec<(i64, i64)> = lines
        .iter()
        .map(|x| {
            let tup = x.split_once("   ").unwrap();
            return (tup.0.parse::<i64>().unwrap(), tup.1.parse::<i64>().unwrap());
        })
        .collect();
    let (first, second): (Vec<i64>, Vec<i64>) = pairs.into_iter().unzip();

    let mut result: i64 = 0;
    for num in first.iter() {
        result += num * i64::try_from(
            second
                .iter()
                .filter(|&x| x == num)
                .collect::<Vec<&i64>>()
                .len(),
        )
        .unwrap();
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day01_test() {
        let path = "day01-test.txt";
        let test_input = fs::read_to_string(path).expect("Should be able to read input");
        let result = process(test_input);
        assert_eq!(result, 31);
    }
}
