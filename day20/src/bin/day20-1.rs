use array2d::Array2D;
use glam::IVec2;
use pathfinding::prelude::{astar, dijkstra};
use std::collections::HashMap;
use std::fs;

fn main() {
    let path = "day20-input.txt";
    let error_msg = "Should be able to read input file";
    let input = fs::read_to_string(path).expect(error_msg);
    let result = process(input, IVec2::Y, 100);
    println!("{}", result);
}

fn dij(
    m: &HashMap<IVec2, char>,
    start: IVec2,
    start_dir: IVec2,
    end: IVec2,
) -> (Vec<(IVec2, IVec2)>, usize) {
    //gets stuck somewhere
    let result = astar(
        &(start, start_dir),
        |(pos, dir): &(IVec2, IVec2)| {
            let next = pos + dir;
            let wall = match m.get(&next) {
                Some(c) => {
                    if c == &'#' {
                        true
                    } else {
                        false
                    }
                }
                None => false,
            };
            if wall {
                vec![((*pos, dir.perp()), 0), ((*pos, -dir.perp()), 0)]
            } else {
                vec![
                    ((next, *dir), 1),
                    ((*pos, dir.perp()), 0),
                    ((*pos, -dir.perp()), 0),
                ]
            }
        },
        |_| 0,
        |&(n, _)| n == end,
    )
    .unwrap();
    dbg!(result.1);
    return result;
}

fn distance(x: IVec2, y: IVec2) -> i32 {
    return (x.x - y.x).abs() + (x.y - y.y).abs();
}

pub fn process(input: String, start_dir: IVec2, threshold: usize) -> u64 {
    let mut result = 0;
    let map = input
        .lines()
        .map(|line| line.to_string().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let g = Array2D::from_rows(&map).unwrap();
    let start_tup = g
        .enumerate_row_major()
        .filter(|c| c.1 == &'S')
        .collect::<Vec<((usize, usize), &char)>>()[0];
    let start = IVec2::new(start_tup.0 .0 as i32, start_tup.0 .1 as i32);
    let end_tup = g
        .enumerate_row_major()
        .filter(|c| c.1 == &'E')
        .collect::<Vec<((usize, usize), &char)>>()[0];
    let end = IVec2::new(end_tup.0 .0 as i32, end_tup.0 .1 as i32);
    let mut m: HashMap<IVec2, char> = HashMap::new();
    for c in g.enumerate_row_major() {
        let a = IVec2::new(c.0 .0 as i32, c.0 .1 as i32);
        m.insert(a, *c.1);
    }
    let no_cheat = dij(&m, start, start_dir, end);
    /*
    for (i, (pos, dir)) in no_cheat.0.iter().enumerate() {
        for (j, (pos2, dir2)) in no_cheat.0[i + 1..].iter().enumerate() {
            if distance(*pos, *pos2) == 2 {
                let cheat_dist = no_cheat.0[..i + 1].len() + no_cheat.0[j..].len();
                if cheat_dist < no_cheat.1 - threshold {
                    dbg!("shortcut", pos, pos2);
                    result += 1;
                }
            }
        }
    }
    */
    let walls = m
        .iter()
        .filter(|(pos, wall)| wall == &&'#')
        .map(|(pos, wall)| (*pos, *wall))
        .collect::<Vec<(IVec2, char)>>();
    let valid_walls = walls
        .iter()
        .filter(|wall| {
            let dirs = vec![IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y];
            dirs.iter()
                .filter(|direction| {
                    let next_pos = wall.0 + **direction;
                    (1..g.row_len()).contains(&(next_pos.x as usize))
                        && (1..g.column_len()).contains(&(next_pos.x as usize))
                        && !walls.contains(&(next_pos, '#'))
                })
                .count()
                > 1
        })
        .map(|x| *x)
        .collect::<Vec<(IVec2, char)>>();
    for (pos, wall) in valid_walls {
        dbg!("next wall");
        let mut cheat_map = m.clone();
        cheat_map
            .insert(IVec2::new(pos.x as i32, pos.y as i32), '.')
            .unwrap();
        let cheat_result = dij(&cheat_map, start, start_dir, end);
        if cheat_result.1 < no_cheat.1 - threshold {
            println!("shortcut found! ");
            result += 1;
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day20_part1_test() {
        let path = "day20-test.txt";
        let test_input = fs::read_to_string(path).expect("Should be able to read input");
        let result = process(test_input, IVec2::NEG_Y, 0);
        //total number of cheats
        assert_eq!(result, 45);
    }
}
