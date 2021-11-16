use std::collections::{HashMap, HashSet};

use petgraph::matrix_graph::{NodeIndex, UnMatrix};
use petgraph::visit::IntoNodeIdentifiers;

use crate::graph_utils::{
    count_colors, find_node_with_maximum_degree, get_neighbor_unique_colors, get_node_degrees,
};
use crate::greedy_algorithm::{find_node_with_greedy_choice, solve_with_greedy};

pub fn solve_with_branch_and_bound(graph: &UnMatrix<(), ()>) -> HashMap<NodeIndex, usize> {
    let node_set: HashSet<_> = graph.node_identifiers().collect();
    let node_degrees = get_node_degrees(graph);

    // Get initial best solution and upper bound using greedy algorithm
    let mut best_node_colors = solve_with_greedy(graph);
    let mut best_color_count = count_colors(&best_node_colors);

    // Stack of node color combinations to visit
    let mut color_combinations_to_visit = Vec::new();

    // Create new incomplete node color combination with starting node
    let starting_node_index = find_node_with_maximum_degree(&node_degrees);
    let starting_node_color = HashMap::from([(starting_node_index, 0)]);
    color_combinations_to_visit.push(starting_node_color);

    // Visit node color combinations using branch and bound
    while let Some(current_node_colors) = color_combinations_to_visit.pop() {
        let is_coloring_complete = current_node_colors.len() == graph.node_count();
        let current_color_count = count_colors(&current_node_colors);

        if is_coloring_complete && current_color_count < best_color_count {
            best_node_colors = current_node_colors;
            best_color_count = current_color_count;
        } else if current_color_count < best_color_count {
            let new_color_combinations =
                extend_node_colors(graph, &node_set, &node_degrees, &current_node_colors);
            for node_colors in new_color_combinations {
                color_combinations_to_visit.push(node_colors)
            }
        }
    }

    best_node_colors
}

fn extend_node_colors(
    graph: &UnMatrix<(), ()>,
    node_set: &HashSet<NodeIndex>,
    node_degrees: &Vec<usize>,
    node_colors: &HashMap<NodeIndex, usize>,
) -> Vec<HashMap<NodeIndex, usize>> {
    let mut color_combinations = Vec::new();

    // Get next uncolored node to color
    let uncolored_node_index =
        find_node_with_greedy_choice(graph, node_set, node_degrees, node_colors);

    // Generate new partial color combinations from next uncolored node
    let neighbor_colors = get_neighbor_unique_colors(graph, uncolored_node_index, node_colors);
    let color_count = count_colors(node_colors);
    for i in 0..color_count + 1 {
        if !neighbor_colors.contains(&i) {
            let mut new_node_colors = node_colors.clone();
            new_node_colors.insert(uncolored_node_index, i);
            color_combinations.push(new_node_colors);
        }
    }

    color_combinations
}
