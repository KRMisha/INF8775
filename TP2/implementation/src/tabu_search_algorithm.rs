use std::collections::{HashMap, HashSet};

use petgraph::matrix_graph::{NodeIndex, UnMatrix};
use petgraph::visit::IntoEdgeReferences;
use rand::Rng;

use crate::graph_utils::count_colors;
use crate::greedy_algorithm::solve_with_greedy;

const MAX_TABU_ITERATION_COUNT: usize = 128;
const ALPHA: usize = 2;
const G: usize = 10;

pub fn solve_with_tabu_search(graph: &UnMatrix<(), ()>) -> HashMap<NodeIndex, usize> {
    // Get initial best solution using greedy algorithm
    let mut best_node_colors = solve_with_greedy(&graph);

    // Reduce colors until no longer possible
    loop {
        // Reassign smaller color with minimum conflicts to nodes with max color
        let reduced_node_colors = reduce_node_colors(graph, &best_node_colors);

        // Fix conflicts with tabu search until there are no more conflicts or if max iterations have been exhausted
        let tabu_search_result = fix_conflicts_with_tabu_search(graph, &reduced_node_colors);
        match tabu_search_result {
            Some(resolved_reduced_node_colors) => best_node_colors = resolved_reduced_node_colors,
            None => break, // If tabu search failed, the best solution is the previous tabu search's result
        }
    }

    best_node_colors
}

fn reduce_node_colors(
    graph: &UnMatrix<(), ()>,
    node_colors: &HashMap<NodeIndex, usize>,
) -> HashMap<NodeIndex, usize> {
    let color_count = count_colors(node_colors);

    let mut reduced_node_colors = node_colors.clone();

    for (node_index, color) in node_colors.iter() {
        // Only reassign color for nodes with the current max color
        if *color != color_count - 1 {
            continue;
        }

        // Find new node color minimizing conflicts with neighbors
        let mut best_new_color = 0usize;
        let mut min_conflict_count = graph.neighbors(*node_index).count();

        for new_color in 0..color_count - 1 {
            let conflict_count =
                count_conflicts_for_node_color(graph, *node_index, new_color, node_colors);

            if conflict_count < min_conflict_count {
                best_new_color = new_color;
                min_conflict_count = conflict_count;
            }
        }

        // Update node with new color
        reduced_node_colors.insert(*node_index, best_new_color);
    }

    reduced_node_colors
}

fn fix_conflicts_with_tabu_search(
    graph: &UnMatrix<(), ()>,
    node_colors: &HashMap<NodeIndex, usize>,
) -> Option<HashMap<NodeIndex, usize>> {
    // Tabu list
    let mut tabu_list = HashSet::new();
    let mut tabu_expiration_ticks = HashMap::<usize, Vec<(NodeIndex, usize)>>::new();
    let mut current_tick = 0usize;

    // Current node color combination
    let mut current_node_colors = node_colors.clone();

    // Count total number of conflicts in current node color combination
    let mut current_conflict_count = 0usize;
    for (source_node_index, target_node_index, _) in graph.edge_references() {
        let source_node_color = current_node_colors.get(&source_node_index).unwrap();
        let target_node_color = current_node_colors.get(&target_node_index).unwrap();
        if source_node_color == target_node_color {
            current_conflict_count += 1;
        }
    }

    // Tabu search
    while current_tick < MAX_TABU_ITERATION_COUNT {
        // Generate neighbors
        let neighboring_node_color_tuples =
            generate_neighboring_node_colors(&current_node_colors, &tabu_list);

        // Find neighboring node color minimizing conflicts
        let mut best_neighbor_node_color_tuple = neighboring_node_color_tuples[0];
        let mut best_neighbor_conflict_count = graph.node_count();

        for node_color_tuple in neighboring_node_color_tuples {
            // Subtract conflicts caused by current node color to be replaced
            let conflicts_for_current_node_color = count_conflicts_for_node_color(
                graph,
                node_color_tuple.0,
                *current_node_colors.get(&node_color_tuple.0).unwrap(),
                &current_node_colors,
            );

            // Add conflicts caused by new neighbor node color
            let conflicts_for_neighbor_node_color = count_conflicts_for_node_color(
                graph,
                node_color_tuple.0,
                node_color_tuple.1,
                &current_node_colors,
            );

            let conflict_count = current_conflict_count - conflicts_for_current_node_color
                + conflicts_for_neighbor_node_color;

            if conflict_count < best_neighbor_conflict_count {
                best_neighbor_node_color_tuple = node_color_tuple;
                best_neighbor_conflict_count = conflict_count;
            }
        }

        // Update current node color combination with best neighbor node color tuple
        current_node_colors.insert(
            best_neighbor_node_color_tuple.0,
            best_neighbor_node_color_tuple.1,
        );
        current_conflict_count = best_neighbor_conflict_count;

        // Update tabu list
        tabu_list.insert(best_neighbor_node_color_tuple);

        let expiration_tick = current_tick
            + ALPHA * best_neighbor_conflict_count
            + rand::thread_rng().gen_range(1..G);
        tabu_expiration_ticks
            .entry(expiration_tick)
            .or_default()
            .push(best_neighbor_node_color_tuple);

        current_tick += 1;

        // Remove expired tabu list entries
        if let Some(expired_node_color_tuples) = tabu_expiration_ticks.remove(&current_tick) {
            for expired_node_color_tuple in expired_node_color_tuples {
                tabu_list.remove(&expired_node_color_tuple);
            }
        }

        // Stop tabu search and return new node color combinations when there are no more conflicts
        if best_neighbor_conflict_count == 0 {
            return Some(current_node_colors);
        }
    }

    // Return None if the max number of iterations has been reached (tabu search failed)
    None
}

fn generate_neighboring_node_colors(
    node_colors: &HashMap<NodeIndex, usize>,
    tabu_list: &HashSet<(NodeIndex, usize)>,
) -> Vec<(NodeIndex, usize)> {
    let color_count = count_colors(node_colors);

    let mut neighboring_node_colors = Vec::new();

    for node_index in node_colors.keys() {
        for color in 0..color_count {
            // Exclude current color
            if color == *node_colors.get(node_index).unwrap() {
                continue;
            }

            let node_color_tuple = (*node_index, color);

            // Exclude color if in tabu list
            if tabu_list.contains(&node_color_tuple) {
                continue;
            }

            neighboring_node_colors.push(node_color_tuple);
        }
    }

    neighboring_node_colors
}

fn count_conflicts_for_node_color(
    graph: &UnMatrix<(), ()>,
    node_index: NodeIndex,
    new_color: usize,
    existing_node_colors: &HashMap<NodeIndex, usize>,
) -> usize {
    let mut conflict_count = 0usize;
    for neighbor_node_index in graph.neighbors(node_index) {
        let neighbor_color = *existing_node_colors.get(&neighbor_node_index).unwrap();
        if new_color == neighbor_color {
            conflict_count += 1;
        }
    }
    conflict_count
}
