use itertools::Itertools;
use petgraph::{
    dot::Dot,
    graph::{Graph, NodeIndex},
    Undirected,
};
use std::{
    collections::{HashMap, HashSet},
    fs::{self, File},
    io::Write,
    process::Command,
};

fn main() {
    let path = "day23-input.txt";
    let error_msg = "Should be able to read input file";
    let input = fs::read_to_string(path).expect(error_msg);
    let result = process(input);
    println!("{}", result);
}

pub fn process(input: String) -> usize {
    let edges: Vec<(&str, &str)> = input
        .lines()
        .map(|line| line.split_once("-").unwrap())
        .collect();
    let mut nodes: HashMap<&str, NodeIndex> = HashMap::new();
    let mut t_nodes: HashSet<NodeIndex> = HashSet::new();
    let mut graph_edges = Vec::new();
    let mut graph = Graph::<&str, &str, Undirected>::new_undirected();
    for edge in edges.iter() {
        let first_ind;
        let sec_ind;
        if !nodes.contains_key(edge.0) {
            first_ind = graph.add_node(edge.0);
            nodes.insert(edge.0, first_ind);
            if edge.0.starts_with("t") {
                t_nodes.insert(first_ind);
            }
        } else {
            first_ind = nodes.get(edge.0).unwrap().clone();
        }
        if !nodes.contains_key(edge.1) {
            sec_ind = graph.add_node(edge.1);
            nodes.insert(edge.1, sec_ind);
            if edge.1.starts_with("t") {
                t_nodes.insert(sec_ind);
            }
        } else {
            sec_ind = nodes.get(edge.1).unwrap().clone();
        }
        graph_edges.push((first_ind, sec_ind));
    }
    graph.extend_with_edges(graph_edges);
    let mut sets: HashSet<Vec<&&str>> = HashSet::new();
    // render graph for debugging, not recommended with real input
    /*
    let dot = Dot::new(&graph);
    let mut file = File::create("graph.dot").unwrap();
    write!(file, "{:?}", dot).unwrap();
    let mut cmd = Command::new("dot");
    cmd.arg("-Tpng").arg("graph.dot").arg(">").arg("graph.png");
    cmd.status().expect("Couldn't render graph");
    */
    for t in t_nodes.iter() {
        let pairs = graph.neighbors(*t).combinations(2);
        for pair in pairs {
            if graph.contains_edge(pair[0], pair[1]) {
                let mut set = vec![
                    graph.node_weight(*t).unwrap(),
                    graph.node_weight(pair[0]).unwrap(),
                    graph.node_weight(pair[1]).unwrap(),
                ];
                set.sort();
                sets.insert(set);
            }
        }
    }

    return sets.len();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day23_part1_test() {
        let path = "day23-test.txt";
        let test_input = fs::read_to_string(path).expect("Should be able to read input");
        let result = process(test_input);
        assert_eq!(result, 7);
    }
}
