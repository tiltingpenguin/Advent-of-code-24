use std::fs;

fn main() {
    let path = "day19-input.txt";
    let error_msg = "Should be able to read input file";
    let input = fs::read_to_string(path).expect(error_msg);
    let result = process(input);
    println!("{}", result);
}

fn consume(design: &str, options: &Vec<&str>) -> bool {
    let starts: Vec<&str> = options
        .clone()
        .iter()
        .filter(|s| design.starts_with(*s))
        .map(|s| *s)
        .collect();
    let result = starts
        .iter()
        .map(|opt| {
            let x = &design[opt.len()..];
            if x.is_empty() {
                return true;
            } else {
                return consume(x, options);
            }
        })
        .any(|res| res == true);

    result
}

pub fn process(input: String) -> usize {
    let parts = input.split_once("\n\n").unwrap();
    let stripes: Vec<&str> = parts.0.split(", ").collect();
    let designs: Vec<&str> = parts.1.lines().collect();

    let result = designs
        .iter()
        .filter(|design| consume(design, &stripes))
        .count();
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day19_part1_test() {
        let path = "day19-test.txt";
        let test_input = fs::read_to_string(path).expect("Should be able to read input");
        let result = process(test_input);
        assert_eq!(result, 6);
    }
}
