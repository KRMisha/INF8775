use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use petgraph::matrix_graph::{NodeIndex, UnMatrix};
use petgraph::visit::IntoNodeIdentifiers;

pub fn load_graph(filename: &Path) -> Result<UnMatrix<u8, ()>, Box<dyn Error>> {
    let buffered = BufReader::new(File::open(filename)?);
    let mut lines_it = buffered.lines().map(|l| l.unwrap());

    // Read adjacency matrix size
    let first_line = lines_it.next().unwrap();
    let matrix_size = first_line.trim().parse()?;

    // Initialize graph
    let mut graph = UnMatrix::with_capacity(matrix_size);

    // Create nodes
    for _ in 0..matrix_size {
        graph.add_node(0u8); // TODO: Check if storing a weight in the node is still needed
    }

    // Create edges
    for (i, line) in lines_it.enumerate() {
        for (j, number) in line.trim().split(char::is_whitespace).enumerate() {
            let value: u8 = number.trim().parse()?;
            if value == 1 {
                graph.update_edge(NodeIndex::new(i), NodeIndex::new(j), ());
            }
        }
    }

    Ok(graph)
}

pub fn get_node_degrees(graph: &UnMatrix<u8, ()>) -> Vec<usize> {
    let mut node_degrees = Vec::with_capacity(graph.node_count());

    for node_index in graph.node_identifiers() {
        let degree = graph.edges(node_index).count();
        node_degrees.push(degree);
    }

    node_degrees
}

pub fn find_node_with_maximum_degree(node_degrees: &Vec<usize>) -> NodeIndex {
    let mut max_degree_node_index = NodeIndex::new(0);
    let mut max_degree = 0usize;

    for (index, degree) in node_degrees.iter().enumerate() {
        if max_degree < *degree {
            max_degree = *degree;
            max_degree_node_index = NodeIndex::new(index);
        }
    }

    max_degree_node_index
}

pub fn get_neighbor_unique_colors(
    graph: &UnMatrix<u8, ()>,
    node_index: NodeIndex,
    node_colors: &HashMap<NodeIndex, usize>,
) -> HashSet<usize> {
    let mut unique_neighbor_colors = HashSet::new();
    for neighbor_node_index in graph.neighbors(node_index) {
        if let Some(color) = node_colors.get(&neighbor_node_index) {
            unique_neighbor_colors.insert(*color);
        }
    }

    unique_neighbor_colors
}

pub fn print_result(colors: &Vec<usize>) {
    let color_count = colors.iter().max().unwrap() + 1;
    println!("{}", color_count);

    let color_str = colors
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    println!("{}", color_str);
}
