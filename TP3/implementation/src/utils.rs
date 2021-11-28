use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use petgraph::graph::{NodeIndex, UnGraph};

pub fn load_graph(filename: &Path) -> Result<UnGraph<u16, ()>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let student_count: usize = lines
        .next()
        .ok_or("Invalid file format: missing number of students")??
        .parse()?;
    let conflict_free_pairs_count: usize = lines
        .next()
        .ok_or("Invalid file format: missing number of conflict-free pairs")??
        .parse()?;

    let mut graph = UnGraph::with_capacity(student_count, conflict_free_pairs_count);

    for line in lines.by_ref().take(student_count) {
        let weight: u16 = line?.parse()?;
        graph.add_node(weight);
    }

    for line in lines.take(conflict_free_pairs_count) {
        let edge_indices: Vec<u32> = line?
            .split(" ")
            .map(|s| s.parse())
            .collect::<Result<Vec<_>, _>>()?;

        graph.add_edge(
            (edge_indices[0] - 1).into(), // Convert from 1-indexed to 0-indexed nodes
            (edge_indices[1] - 1).into(),
            (),
        );
    }

    Ok(graph)
}

pub fn print_result(node_indices: &Vec<NodeIndex>) {
    let node_indices_str: String = node_indices
        .iter()
        .map(|n| (n.index() + 1).to_string()) // Convert back from 0-indexed to 1-indexed nodes
        .collect::<Vec<_>>()
        .join(" ");
    println!("{}", node_indices_str);
}
