use std::{collections::HashMap, fs};

fn main() {
    let path = "day21-input.txt";
    let error_msg = "Should be able to read input file";
    let input = fs::read_to_string(path).expect(error_msg);
    let result = process(input);
    println!("{}", result);
}

/*
* I really liked this problem at first and tried to solve it with
* a Vector for the distance and walking the Vector with prioritized
* directions. The problem is that the numpad and direction pad aren't
* rectangles and you can't walk the missing square which made me want to
* pull out my hair. So this extremely ugly solution of hardcoding all paths
* was the next best idea.
*/

fn go_numpad(start: char, end: char) -> String {
    let way_lookup: HashMap<char, HashMap<char, &str>> = HashMap::from([
        (
            'A',
            HashMap::from([
                ('A', "A"),
                ('0', "<A"),
                ('1', "^<<A"),
                ('2', "<^A"),
                ('3', "^A"),
                ('4', "^^<<A"),
                ('5', "<^^A"),
                ('6', "^^A"),
                ('7', "^^^<<A"),
                ('8', "<^^^A"),
                ('9', "^^^A"),
            ]),
        ),
        (
            '0',
            HashMap::from([
                ('A', ">A"),
                ('0', "A"),
                ('1', "^<A"),
                ('2', "^A"),
                ('3', ">^A"),
                ('4', "^^<A"),
                ('5', "^^A"),
                ('6', ">^^A"),
                ('7', "^^^<A"),
                ('8', "^^^A"),
                ('9', ">^^^A"),
            ]),
        ),
        (
            '1',
            HashMap::from([
                ('A', ">>vA"),
                ('0', ">vA"),
                ('1', "A"),
                ('2', ">A"),
                ('3', ">>A"),
                ('4', "^A"),
                ('5', "^>A"),
                ('6', ">>^A"),
                ('7', "^^A"),
                ('8', "^^>A"),
                ('9', "^^>>A"),
            ]),
        ),
        (
            '2',
            HashMap::from([
                ('A', "v>A"),
                ('0', "vA"),
                ('1', "<A"),
                ('2', "A"),
                ('3', ">A"),
                ('4', "<^A"),
                ('5', "^A"),
                ('6', ">^A"),
                ('7', "<^^A"),
                ('8', "^^A"),
                ('9', ">^^A"),
            ]),
        ),
        (
            '3',
            HashMap::from([
                ('A', "vA"),
                ('0', "<vA"),
                ('1', "<<A"),
                ('2', "<A"),
                ('3', "A"),
                ('4', "<<^A"),
                ('5', "<^A"),
                ('6', "^A"),
                ('7', "<<^^A"),
                ('8', "<^^A"),
                ('9', "^^A"),
            ]),
        ),
        (
            '4',
            HashMap::from([
                ('A', ">>vvA"),
                ('0', ">vvA"),
                ('1', "vA"),
                ('2', "v>A"),
                ('3', "v>>A"),
                ('4', "A"),
                ('5', ">A"),
                ('6', ">>A"),
                ('7', "^A"),
                ('8', ">^A"),
                ('9', ">>^A"),
            ]),
        ),
        (
            '5',
            HashMap::from([
                ('A', "vv>A"),
                ('0', "vvA"),
                ('1', "<vA"),
                ('2', "vA"),
                ('3', "v>A"),
                ('4', "<A"),
                ('5', "A"),
                ('6', ">A"),
                ('7', "<^A"),
                ('8', "^A"),
                ('9', "^>A"),
            ]),
        ),
        (
            '6',
            HashMap::from([
                ('A', "vvA"),
                ('0', "<vvA"),
                ('1', "<<vA"),
                ('2', "<vA"),
                ('3', "vA"),
                ('4', "<<A"),
                ('5', "<A"),
                ('6', "A"),
                ('7', "<<^A"),
                ('8', "<^A"),
                ('9', "^A"),
            ]),
        ),
        (
            '7',
            HashMap::from([
                ('A', ">>vvvA"),
                ('0', ">vvvA"),
                ('1', "vvA"),
                ('2', "vv>A"),
                ('3', "vv>>A"),
                ('4', "vA"),
                ('5', "v>A"),
                ('6', ">>vA"),
                ('7', "A"),
                ('8', ">A"),
                ('9', ">>A"),
            ]),
        ),
        (
            '8',
            HashMap::from([
                ('A', "vvv>A"),
                ('0', "vvvA"),
                ('1', "vv<A"),
                ('2', "vvA"),
                ('3', "vv>A"),
                ('4', "<vA"),
                ('5', "vA"),
                ('6', "v>A"),
                ('7', "<A"),
                ('8', "A"),
                ('9', ">A"),
            ]),
        ),
        (
            '9',
            HashMap::from([
                ('A', "vvvA"),
                ('0', "<vvvA"),
                ('1', "<<vA"),
                ('2', "<vvA"),
                ('3', "vvA"),
                ('4', "<<vA"),
                ('5', "<vA"),
                ('6', "vA"),
                ('7', "<<A"),
                ('8', "<A"),
                ('9', "A"),
            ]),
        ),
    ]);

    return way_lookup
        .get(&start)
        .unwrap()
        .get(&end)
        .unwrap()
        .to_string();
}

fn go(start: char, end: char) -> String {
    let way_lookup: HashMap<char, HashMap<char, &str>> = HashMap::from([
        (
            'A',
            HashMap::from([
                ('A', "A"),
                ('<', "v<<A"),
                ('>', "vA"),
                ('^', "<A"),
                ('v', "<vA"),
            ]),
        ),
        (
            '<',
            HashMap::from([
                ('A', ">>^A"),
                ('<', "A"),
                ('>', ">>A"),
                ('^', ">^A"),
                ('v', ">A"),
            ]),
        ),
        (
            '>',
            HashMap::from([
                ('A', "^A"),
                ('<', "<<A"),
                ('>', "A"),
                ('^', "<^A"),
                ('v', "<A"),
            ]),
        ),
        (
            '^',
            HashMap::from([
                ('A', ">A"),
                ('<', "v<A"),
                ('>', "v>A"),
                ('^', "A"),
                ('v', "vA"),
            ]),
        ),
        (
            'v',
            HashMap::from([
                ('A', "^>A"),
                ('<', "<A"),
                ('>', ">A"),
                ('^', "^A"),
                ('v', "A"),
            ]),
        ),
    ]);
    return way_lookup
        .get(&start)
        .unwrap()
        .get(&end)
        .unwrap()
        .to_string();
}

pub fn process(input: String) -> usize {
    let instructions = input.lines().collect::<Vec<&str>>();
    let num_complexity = instructions
        .iter()
        .map(|line| line.split_once('A').unwrap().0.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let mut robot_pos = 'A';
    let first_robot = instructions
        .iter()
        .map(|instruction| {
            instruction
                .chars()
                .map(|key| {
                    let res = go_numpad(robot_pos, key);
                    robot_pos = key;
                    res
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>();

    robot_pos = 'A';
    let second_robot = first_robot
        .iter()
        .map(|instruction| {
            instruction
                .chars()
                .map(|key| {
                    let res = go(robot_pos, key);
                    robot_pos = key;
                    res
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>();

    robot_pos = 'A';
    let you = second_robot
        .iter()
        .map(|instruction| {
            instruction
                .chars()
                .map(|key| {
                    let res = go(robot_pos, key);
                    robot_pos = key;
                    res
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>();

    return you
        .iter()
        .enumerate()
        .map(|(index, instruction)| instruction.len() * num_complexity[index] as usize)
        .sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day21_part1_test() {
        let path = "day21-test.txt";
        let test_input = fs::read_to_string(path).expect("Should be able to read input");
        let result = process(test_input);
        assert_eq!(result, 126384);
    }
}
