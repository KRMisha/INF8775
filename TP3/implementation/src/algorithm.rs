use std::collections::HashMap;

use itertools::Itertools;
use petgraph::graph::{NodeIndex, NodeReferences, UnGraph};
use petgraph::visit::{Dfs, IntoNodeReferences};

pub fn solve(graph: &UnGraph<u16, ()>) -> Vec<NodeIndex> {
    let hamiltonian_paths = compute_hamiltonian_paths(graph);

    // TODO: Replace placeholder algorithm

    let starting_node_index = find_node_with_minimum_weight(graph.node_references());
    let mut dfs = Dfs::new(graph, starting_node_index);

    let mut sorted_node_indices = Vec::with_capacity(graph.node_count());
    while let Some(node_index) = dfs.next(graph) {
        sorted_node_indices.push(node_index);
    }

    sorted_node_indices
}

fn find_node_with_minimum_weight(nodes: NodeReferences<u16>) -> NodeIndex {
    nodes.min_by_key(|&(_, weight)| weight).unwrap().0.into()
}

fn compute_hamiltonian_paths(
    graph: &UnGraph<u16, ()>,
) -> HashMap<(NodeIndex, Vec<NodeIndex>), bool> {
    let mut valid_subpaths = HashMap::new();

    // Paths starting at a node and ending without going through any intermediate nodes are always valid
    for node_index in graph.node_indices() {
        valid_subpaths.insert((node_index, Vec::new()), true);
    }

    // Create subpaths, lengthening each subpath by one node every iteration
    for subpath_length in 1..graph.node_count() - 1 {
        let subpath_combinations = graph.node_indices().combinations(subpath_length);
        for subpath in subpath_combinations {
            println!("{:?}", subpath);
            // TODO
        }
    }

    valid_subpaths
}
