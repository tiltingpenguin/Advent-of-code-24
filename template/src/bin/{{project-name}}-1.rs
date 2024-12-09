use std::fs;

fn main() {
    let path = "{{project-name}}-input.txt";
    let error_msg = "Should be able to read input file";
    let input = fs::read_to_string(path).expect(error_msg);
    let result = process(input);
    println!("{}", result);
}

pub fn process(input: String) -> u64 {
    todo!("{{project-name}} - part 1");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn {{project-name}}_part1_test() {
        let path = "{{project-name}}-test.txt";
        let test_input = fs::read_to_string(path).expect("Should be able to read input");
        let result = process(test_input);
        todo!("haven't put in test result yet");
        assert_eq!(result, 0);
    }
}
