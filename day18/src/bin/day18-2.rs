use array2d::Array2D;
use glam::IVec2;
use pathfinding::prelude::dijkstra;
use std::fs;

fn main() {
    let path = "day18-input.txt";
    let error_msg = "Should be able to read input file";
    let input = fs::read_to_string(path).expect(error_msg);
    let result = process(input, 71, 1024);
    println!("{}", result);
}

pub fn process(input: String, size: usize, max_iter: usize) -> u64 {
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
            dbg!(&next_tiles.len());
            return next_tiles;
        },
        |&n| n == end,
    );
    return result.expect("no path found").1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day18_part2_test() {
        let path = "day18-test.txt";
        let test_input = fs::read_to_string(path).expect("Should be able to read input");
        let result = process(test_input, 7, 12);
        assert_eq!(result, 22);
    }
}
