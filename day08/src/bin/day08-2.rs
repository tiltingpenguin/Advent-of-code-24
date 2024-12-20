use array2d::Array2D;
use glam::IVec2;
use std::collections::HashSet;
use std::fs;

/*
* There is a bunch of code left from when I was actually counting
* found antinodes, instead of editing the map and counting elements,
* but I'm too lazy to remove it rn
*/

fn main() {
    let path = "day08-input.txt";
    let error_msg = "Should be able to read input file";
    let input = fs::read_to_string(path).expect(error_msg);
    let result = process(input);
    println!("{}", result);
}

enum Dir {
    Forward,
    Backward,
}

fn check_spot(pos: IVec2, next_pos: IVec2, map: &mut Array2D<char>, dir: Dir) -> usize {
    let diff = next_pos - pos;
    let mut res_point: IVec2;
    let mut pos_cache = Vec::new();
    match dir {
        Dir::Forward => res_point = next_pos + diff,
        Dir::Backward => res_point = pos - diff,
    }
    let mut check = map.get(res_point.x as usize, res_point.y as usize);
    while check.is_some() {
        if check.unwrap() != &'#' {
            pos_cache.push((res_point.x as usize, res_point.y as usize));
        }
        match dir {
            Dir::Forward => res_point = res_point + diff,
            Dir::Backward => res_point = res_point - diff,
        }
        check = map.get(res_point.x as usize, res_point.y as usize);
    }
    for p in pos_cache.iter() {
        if map.get(p.0, p.1).unwrap() == &'.' {
            map.set(p.0, p.1, '#').unwrap();
        }
    }
    return pos_cache.len();
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
        if pos_vec.len() == 1 {
            map.set(pos_vec[0].x as usize, pos_vec[0].y as usize, '.')
                .unwrap();
        }
        result += pos_vec
            .iter()
            .enumerate()
            .map(|(ind, pos)| {
                return pos_vec[ind + 1..]
                    .iter()
                    .map(|next_pos| {
                        let mut res = 0;
                        res += check_spot(*pos, *next_pos, &mut map, Dir::Backward);
                        res += check_spot(*pos, *next_pos, &mut map, Dir::Forward);
                        res
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
    return map.elements_row_major_iter().filter(|c| c != &&'.').count();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day08_part1_test() {
        let path = "day08-test.txt";
        let test_input = fs::read_to_string(path).expect("Should be able to read input");
        let result = process(test_input);
        assert_eq!(result, 34);
    }
}
