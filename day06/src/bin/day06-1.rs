use array2d::Array2D;
use glam::IVec2;
use std::collections::HashSet;
use std::fs;

fn main() {
    let path = "day06-input.txt";
    let error_msg = "Should be able to read input file";
    let input = fs::read_to_string(path).expect(error_msg);
    let result = process(input);
    println!("{}", result);
}

pub fn process(input: String) -> usize {
    let tmp = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let map = Array2D::from_rows(&tmp).unwrap();
    let (s, _) = map
        .enumerate_row_major()
        .filter(|c| c.1 == &'^')
        .collect::<Vec<((usize, usize), &char)>>()[0];
    let mut pos = IVec2::new(s.1 as i32, s.0 as i32);
    let mut dir = IVec2::NEG_Y;
    let mut fields: HashSet<IVec2> = HashSet::new();
    loop {
        fields.insert(pos);
        let n = pos + dir;
        if let Some(next) = map.get(n.y as usize, n.x as usize) {
            if next == &'#' {
                dir = dir.perp();
            } else {
                pos = n;
            }
        } else {
            break;
        }
    }
    return fields.len();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day06_part1_test() {
        let path = "day06-test.txt";
        let test_input = fs::read_to_string(path).expect("Should be able to read input");
        let result = process(test_input);
        assert_eq!(result, 41);
    }
}
