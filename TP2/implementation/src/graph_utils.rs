use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use itertools::Itertools;
use petgraph::matrix_graph::{NodeIndex, UnMatrix};
use petgraph::visit::IntoNodeIdentifiers;

#[allow(dead_code)]
pub fn load_graph_from_matrix(filename: &Path) -> Result<UnMatrix<(), ()>, Box<dyn Error>> {
    let buffered = BufReader::new(File::open(filename)?);
    let mut lines_it = buffered.lines().map(|l| l.unwrap());

    // Read adjacency matrix size
    let first_line = lines_it.next().unwrap();
    let matrix_size = first_line.trim().parse()?;

    // Initialize graph
    let mut graph = UnMatrix::with_capacity(matrix_size);

    // Create nodes
    for _ in 0..matrix_size {
        graph.add_node(());
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

pub fn load_graph_from_edge_list(filename: &Path) -> Result<UnMatrix<(), ()>, Box<dyn Error>> {
    let buffered = BufReader::new(File::open(filename)?);
    let lines_it = buffered.lines().map(|l| l.unwrap());

    // Parse edges
    let mut edges = Vec::new();
    for line in lines_it {
        if line.starts_with("e") {
            let edge_indices: Vec<u16> = line
                .split(" ")
                .skip(1)
                .take(2)
                .map(|s| s.parse())
                .collect::<Result<Vec<_>, _>>()?;
            edges.push((edge_indices[0] - 1, edge_indices[1] - 1)); // Convert from 1-indexed to 0-indexed nodes
        }
    }

    // Create graph
    let graph = UnMatrix::from_edges(edges);

    Ok(graph)
}

pub fn get_node_degrees(graph: &UnMatrix<(), ()>) -> Vec<usize> {
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
    graph: &UnMatrix<(), ()>,
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

pub fn count_colors(node_colors: &HashMap<NodeIndex, usize>) -> usize {
    node_colors.values().max().unwrap() + 1
}

pub fn print_result(node_colors: &HashMap<NodeIndex, usize>) {
    println!("{}", count_colors(node_colors));

    let color_str = node_colors
        .iter()
        .sorted()
        .map(|(_node_index, color)| color.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    println!("{}", color_str);
}
