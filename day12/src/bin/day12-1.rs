use array2d::{self, Array2D};
use std::collections::{HashMap, HashSet};
use std::fs;

/*
* Code straight from hell. First I parsed the whole 2d array into a graph/hashmap with the sort
* function, then I had to separate the different areas by letter, and during that I ran a
* depth-first search to differenciate between different areas that share the same letter. This is
* probably the most convoluted way to do this and I wouldn't be shocked if the whole copying of
* Vecs and HashMaps caused a memory leak, despite the guarantees that rust makes.
*/

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Tile {
    value: char,
    x: usize,
    y: usize,
}

fn main() {
    let path = "day12-input.txt";
    let error_msg = "Should be able to read input file";
    let input = fs::read_to_string(path).expect(error_msg);
    let result = process(input);
    println!("{}", result);
}

fn sort(map: &Array2D<char>) -> HashMap<Tile, Vec<Tile>> {
    let mut neigh: HashMap<Tile, Vec<Tile>> = HashMap::new();
    for ((x, y), val) in map.enumerate_row_major() {
        let current = Tile { value: *val, x, y };
        let mut surrounding: Vec<Tile> = Vec::new();
        if x > 0 {
            surrounding.push(Tile {
                value: *map.get(x - 1, y).unwrap(),
                x: x - 1,
                y,
            });
        }
        if y > 0 {
            surrounding.push(Tile {
                value: *map.get(x, y - 1).unwrap(),
                x,
                y: y - 1,
            });
        }
        if x < map.column_len() - 1 {
            surrounding.push(Tile {
                value: *map.get(x + 1, y).unwrap(),
                x: x + 1,
                y,
            });
        }
        if y < map.row_len() - 1 {
            surrounding.push(Tile {
                value: *map.get(x, y + 1).unwrap(),
                x,
                y: y + 1,
            });
        }
        neigh.insert(current, surrounding);
    }
    return neigh;
}

fn find_connected(
    areas: &HashMap<Tile, Vec<Tile>>,
    letter: &char,
) -> Vec<HashMap<Tile, Vec<Tile>>> {
    let mut list: Vec<&Tile> = areas.keys().filter(|key| key.value == *letter).collect();
    let mut plots = Vec::new();
    loop {
        if list.len() == 0 {
            break;
        }
        let first = list[0];
        let mut this_plot = HashMap::new();
        plots.push(dfs(areas, &mut list, &mut this_plot, first));
    }
    return plots;
}

fn dfs(
    areas: &HashMap<Tile, Vec<Tile>>,
    list: &mut Vec<&Tile>,
    this_plot: &mut HashMap<Tile, Vec<Tile>>,
    current: &Tile,
) -> HashMap<Tile, Vec<Tile>> {
    let values = areas.get(current).unwrap().clone();
    this_plot.insert(current.clone(), values.clone());
    let index = list.iter().position(|x| x == &current);
    match index {
        Some(_) => {}
        None => return this_plot.clone(),
    }
    list.remove(index.unwrap());
    let mut new: HashMap<Tile, Vec<Tile>> = HashMap::new();
    let mut found = false;
    for n in values {
        if n.value == current.value {
            found = true;
            new = dfs(areas, list, this_plot, &n);
        }
    }
    if found == false {
        new = this_plot.clone();
    }
    return new;
}

pub fn process(input: String) -> usize {
    let result: usize;
    let tmp = input
        .lines()
        .map(|line| line.to_string().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let map = Array2D::from_rows(&tmp).unwrap();
    let unique: HashSet<&char> = HashSet::from_iter(tmp.iter().flatten());
    let areas = sort(&map);
    let mut area_list: Vec<HashMap<Tile, Vec<Tile>>> = Vec::new();
    for letter in unique {
        area_list.append(&mut find_connected(&areas, letter));
    }
    result = area_list
        .iter()
        .map(|plot| {
            let area = plot.len();
            let border = plot
                .iter()
                .map(|(key, val)| {
                    let others = val.iter().filter(|k| k.value != key.value).count();
                    let empty = 4 - val.len();
                    let total = others + empty;
                    return total;
                })
                .sum::<usize>();
            return area * border;
        })
        .sum();
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day12_part1_test() {
        let path = "day12-test.txt";
        let test_input = fs::read_to_string(path).expect("Should be able to read input");
        let result = process(test_input);
        assert_eq!(result, 1930);
    }
}
