use std::collections::HashMap;
use std::fs;

fn main() {
    let path = "day19-input.txt";
    let error_msg = "Should be able to read input file";
    let input = fs::read_to_string(path).expect(error_msg);
    let result = process(input);
    println!("{}", result);
}

fn consume(design: &str, options: &Vec<&str>, cache: &mut HashMap<String, u64>) -> u64 {
    if cache.contains_key(design) {
        return *(cache.get(design).unwrap());
    }
    let starts: Vec<&str> = options
        .clone()
        .iter()
        .filter(|s| design.starts_with(*s))
        .map(|s| *s)
        .collect();
    if starts.is_empty() {
        return 0;
    }
    let result = starts
        .iter()
        .map(|opt| {
            let x = &design[opt.len()..];
            if x.is_empty() {
                return 1;
            } else {
                return consume(x, options, cache);
            }
        })
        .sum();
    cache.insert(design.to_string(), result);

    result
}

pub fn process(input: String) -> u64 {
    let parts = input.split_once("\n\n").unwrap();
    let stripes: Vec<&str> = parts.0.split(", ").collect();
    let designs: Vec<&str> = parts.1.lines().collect();
    let mut cache: HashMap<String, u64> = HashMap::new();

    let result = designs
        .iter()
        .map(|design| consume(design, &stripes, &mut cache))
        .sum();
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day19_part2_test() {
        let path = "day19-test.txt";
        let test_input = fs::read_to_string(path).expect("Should be able to read input");
        let result = process(test_input);
        assert_eq!(result, 16);
    }
}
