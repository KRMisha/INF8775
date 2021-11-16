use std::collections::{HashMap, HashSet};

use petgraph::matrix_graph::{NodeIndex, UnMatrix};
use petgraph::visit::IntoNodeIdentifiers;

use crate::graph_utils::{
    find_node_with_maximum_degree, get_neighbor_unique_colors, get_node_degrees,
};

pub fn solve_with_greedy(graph: &UnMatrix<(), ()>) -> HashMap<NodeIndex, usize> {
    let node_set: HashSet<_> = graph.node_identifiers().collect();
    let node_degrees = get_node_degrees(graph);

    let mut node_colors = HashMap::new();
    let mut color_count = 1usize;

    // Color starting node
    let starting_node_index = find_node_with_maximum_degree(&node_degrees);
    node_colors.insert(starting_node_index, 0);

    // Color all nodes
    while node_colors.len() < graph.node_count() {
        // Get next uncolored node to color
        let current_node_index =
            find_node_with_greedy_choice(graph, &node_set, &node_degrees, &node_colors);

        // Assign smallest possible color to node
        let color =
            get_smallest_color_for_node(graph, current_node_index, &node_colors, color_count)
                .unwrap();
        node_colors.insert(current_node_index, color);

        // Increment color count if color assigned to node was greater than any current color
        let is_node_color_new = color == color_count;
        if is_node_color_new {
            color_count += 1;
        }
    }

    node_colors
}

pub fn find_node_with_greedy_choice(
    graph: &UnMatrix<(), ()>,
    node_set: &HashSet<NodeIndex>,
    node_degrees: &HashMap<NodeIndex, usize>,
    node_colors: &HashMap<NodeIndex, usize>,
) -> NodeIndex {
    // Compute remaining uncolored nodes
    let colored_node_set: HashSet<_> = node_colors.keys().cloned().collect();
    let uncolored_nodes_indexes: Vec<_> = node_set.difference(&colored_node_set).cloned().collect();

    // Find node with max degree of saturation among uncolored nodes, with max number of neighbors in case of equality
    let mut max_saturation_node_index = *uncolored_nodes_indexes.first().unwrap();
    let mut max_saturation = 0usize;

    for uncolored_node_index in uncolored_nodes_indexes {
        let saturation = get_neighbor_unique_colors(graph, uncolored_node_index, node_colors).len();

        if saturation < max_saturation {
            continue;
        }

        let is_uncolored_node_degree_greater = node_degrees.get(&uncolored_node_index).unwrap()
            > node_degrees.get(&max_saturation_node_index).unwrap();
        if saturation == max_saturation && !is_uncolored_node_degree_greater {
            continue;
        }

        max_saturation_node_index = uncolored_node_index;
        max_saturation = saturation;
    }

    max_saturation_node_index
}

fn get_smallest_color_for_node(
    graph: &UnMatrix<(), ()>,
    node_index: NodeIndex,
    node_colors: &HashMap<NodeIndex, usize>,
    color_count: usize,
) -> Option<usize> {
    let neighbor_colors = get_neighbor_unique_colors(graph, node_index, node_colors);

    for i in 0..color_count + 1 {
        if !neighbor_colors.contains(&i) {
            return Some(i);
        }
    }

    None
}
