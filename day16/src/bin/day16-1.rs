use array2d::Array2D;
use glam::IVec2;
use pathfinding::prelude::dijkstra;
use std::collections::HashMap;
use std::fs;

fn main() {
    let path = "day16-input.txt";
    let error_msg = "Should be able to read input file";
    let input = fs::read_to_string(path).expect(error_msg);
    let start_dir = IVec2::X;
    let result = process(input, start_dir);
    println!("{}", result);
}

pub fn process(input: String, start_dir: IVec2) -> usize {
    let map = input
        .lines()
        .map(|line| line.to_string().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let start = IVec2::new(1, (map.len() - 2) as i32);
    let end = IVec2::new((map[0].len() - 2) as i32, 1);
    let g = Array2D::from_rows(&map).unwrap();
    let mut m: HashMap<IVec2, char> = HashMap::new();
    for c in g.enumerate_row_major() {
        let a = IVec2::new(c.0 .0 as i32, c.0 .1 as i32);
        m.insert(a, *c.1);
    }
    let result = dijkstra(
        &(start, start_dir),
        |(pos, dir): &(IVec2, IVec2)| {
            let next = pos + dir;
            if m.get(&next).is_none() | (m.get(&next).unwrap() == &'#') {
                vec![((*pos, dir.perp()), 1000), ((*pos, -dir.perp()), 1000)]
            } else {
                vec![
                    ((next, *dir), 1),
                    ((*pos, dir.perp()), 1000),
                    ((*pos, -dir.perp()), 1000),
                ]
            }
        },
        |&(n, _)| n == end,
    );
    let x = result.unwrap().1;
    return x;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day16_part1_test() {
        let path = "day16-test.txt";
        let test_input = fs::read_to_string(path).expect("Should be able to read input");
        let test_dir = IVec2::NEG_Y;
        let result = process(test_input, test_dir);
        assert_eq!(result, 7036);
    }

    #[test]
    fn day16_part1_test_2() {
        let path = "day16-2-test.txt";
        let test_input = fs::read_to_string(path).expect("Should be able to read input");
        let test_dir = IVec2::NEG_Y;
        let result = process(test_input, test_dir);
        assert_eq!(result, 11048);
    }
}
