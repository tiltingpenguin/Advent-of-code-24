use array2d::Array2D;
use glam::IVec2;
use pathfinding::prelude::dijkstra;
use std::fs;

fn main() {
    let path = "day18-input.txt";
    let error_msg = "Should be able to read input file";
    let input = fs::read_to_string(path).expect(error_msg);
    let result = process(input, 71, 1024);
    let a = result.0;
    let b = result.1;
    println!("{a},{b}");
}

pub fn process(input: String, size: usize, max_iter: usize) -> (i32, i32) {
    let coords: Vec<IVec2> = input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(",").unwrap();
            IVec2::new(b.parse::<i32>().unwrap(), a.parse::<i32>().unwrap())
        })
        .collect();
    let mut map = Array2D::from_rows(&vec![vec!['.'; size]; size]).unwrap();
    for (counter, c) in coords.iter().enumerate() {
        map.set(c.y as usize, c.x as usize, '#').unwrap();
        if counter >= max_iter {
            break;
        }
    }
    let start = IVec2::new(0, 0);
    let end = IVec2::new((size - 1) as i32, (size - 1) as i32);
    let coords_next = coords[max_iter - 1..].into_iter().collect::<Vec<&IVec2>>();
    let mut last_pos = (0, 0);
    for i in coords_next {
        map.set(i.y as usize, i.x as usize, '#').unwrap();

        let result = dijkstra(
            &start,
            |pos| {
                let mut next_tiles: Vec<(IVec2, u64)> = Vec::new();
                for dir in vec![IVec2::NEG_Y, IVec2::X, IVec2::NEG_X, IVec2::Y].iter() {
                    let next = pos + dir;
                    let val = map.get(next.y as usize, next.x as usize);
                    if val.is_some() {
                        if !(val.unwrap() == &'#') {
                            next_tiles.push((next, 1));
                        }
                    }
                }
                return next_tiles;
            },
            |&n| n == end,
        );
        if result.is_none() {
            last_pos = (i.y, i.x);
            break;
        }
    }
    return last_pos;
}

#[cfg(test)]
mod tests {
    use super::*;
    // test fails but works with real input
    #[test]
    fn day18_part1_test() {
        let path = "day18-test.txt";
        let test_input = fs::read_to_string(path).expect("Should be able to read input");
        let result = process(test_input, 7, 12);
        assert_eq!(result, (6, 1));
    }
}
