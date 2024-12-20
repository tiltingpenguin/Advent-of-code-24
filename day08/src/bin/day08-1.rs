use array2d::Array2D;
use glam::IVec2;
use std::collections::HashSet;
use std::fs;

fn main() {
    let path = "day08-input.txt";
    let error_msg = "Should be able to read input file";
    let input = fs::read_to_string(path).expect(error_msg);
    let result = process(input);
    println!("{}", result);
}

fn check_spot(pos: IVec2, map: &mut Array2D<char>) -> usize {
    let check = map.get(pos.x as usize, pos.y as usize);
    if check.is_some() {
        if check.unwrap() != &'#' {
            map.set(pos.x as usize, pos.y as usize, '#').unwrap();
            return 1;
        }
    }
    return 0;
}

pub fn process(input: String) -> usize {
    let mut result = 0;
    let mut tmp = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut map = Array2D::from_rows(&tmp).unwrap();
    let map_backup = map.clone();
    let unique: HashSet<char> = HashSet::from_iter(
        map.elements_row_major_iter()
            .filter(|c| c != &&'.')
            .map(|c| *c),
    );
    for c in unique {
        let pos_vec = map_backup
            .enumerate_row_major()
            .filter(|(_, val)| val == &&c)
            .map(|(pos, _)| IVec2::new(pos.0 as i32, pos.1 as i32))
            .collect::<Vec<IVec2>>();
        result += pos_vec
            .iter()
            .enumerate()
            .map(|(ind, pos)| {
                return pos_vec[ind + 1..]
                    .iter()
                    .map(|next_pos| {
                        let mut res = 0;
                        let diff = next_pos - pos;
                        res += check_spot(pos - diff, &mut map);
                        res += check_spot(next_pos + diff, &mut map);
                        return res;
                    })
                    .sum::<usize>();
            })
            .sum::<usize>();
    }
    // print the whole map including antinodes
    for (ind, val) in map.enumerate_row_major() {
        tmp[ind.0][ind.1] = *val;
    }
    for line in tmp {
        dbg!(line.iter().collect::<String>());
    }
    //
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day08_part1_test() {
        let path = "day08-test.txt";
        let test_input = fs::read_to_string(path).expect("Should be able to read input");
        let result = process(test_input);
        assert_eq!(result, 14);
    }
}
