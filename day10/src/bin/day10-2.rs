use array2d::{self, Array2D};
use std::fs;

/*
Of course I tried to solve part 1 this way first
*/

fn main() {
    let path = "day10-input.txt";
    let error_msg = "Should be able to read input file";
    let input = fs::read_to_string(path).expect(error_msg);
    let result = process(input);
    println!("{}", result);
}

fn hike(map: &Array2D<usize>, pos: (usize, usize), val: usize, mut result: usize) -> usize {
    if val == 9 {
        result += 1;
        return result;
    }
    let next = val + 1;
    let mut dirs: Vec<(usize, usize)> = Vec::new();
    if !(pos.0 == 0) {
        let up = (pos.0 - 1, pos.1);
        dirs.push(up);
    }
    if !(pos.1 == 0) {
        let left = (pos.0, pos.1 - 1);
        dirs.push(left);
    }
    if !(pos.1 == map.row_len() - 1) {
        let right = (pos.0, pos.1 + 1);
        dirs.push(right);
    }
    if !(pos.0 == map.column_len() - 1) {
        let down = (pos.0 + 1, pos.1);
        dirs.push(down);
    }
    for dir in dirs {
        if map[dir] == next {
            result = hike(&map, dir, next, result);
        }
    }
    return result;
}

pub fn process(input: String) -> usize {
    let mut result: usize = 0;
    let tmp = input
        .lines()
        .map(|line| {
            line.to_string()
                .chars()
                .map(|char| char.to_string().parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();
    let map = Array2D::from_rows(&tmp).unwrap();
    let zeros: Vec<((usize, usize), &usize)> =
        map.enumerate_row_major().filter(|e| e.1 == &0).collect();
    for start in zeros {
        result += hike(&map, start.0, *start.1, 0);
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day10_part1_test() {
        let path = "day10-test.txt";
        let test_input = fs::read_to_string(path).expect("Should be able to read input");
        let result = process(test_input);
        assert_eq!(result, 81);
    }
}
