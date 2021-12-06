use std::collections::HashMap;
use std::iter;

use itertools::Itertools;
use petgraph::graph::{NodeIndex, NodeReferences, UnGraph};
use petgraph::visit::IntoNodeIdentifiers;
use rustc_hash::{FxHashMap, FxHashSet};

use crate::utils::{count_obstructions, print_solution};

// Ideas:
// - If using bidirectional path extension idea, extend with lighter nodes on the left, heavier on the right
// - Use median weight (or close to) for start node
// - Use sort based on a fuzzy mix between node weights and degrees (Euclidian distance to (0, 0) of degree-weight tuples?)
// - Use FxHashSet in extend_path() when the number of neighbors is greater than a certain threshold (tinyset)

pub fn solve_in_loop(graph: &UnGraph<u16, ()>, should_display_full_solution: bool) {
    // Precompute order of exploration for node neighbors
    let ordered_node_neighbors = get_ordered_node_neighbors(graph);

    // Number of obstructions of best solution so far
    let mut min_obstruction_count = graph.node_count() as u32;

    // Stack of partial paths to visit
    let mut paths_to_visit = Vec::new();

    // Any node can act as the starting node for the search tree
    let ordered_starting_nodes = get_ordered_starting_nodes(graph);
    paths_to_visit.extend(
        ordered_starting_nodes
            .into_iter()
            .rev()
            .map(|n| (vec![n], iter::once(n).collect())),
    );

    // Search for Hamiltonian paths with backtracking algorithm
    while let Some((current_path, current_path_set)) = paths_to_visit.pop() {
        let is_path_complete = current_path.len() == graph.node_count();
        // TODO: Branch-and-bound
        if is_path_complete {
            let obstruction_count = count_obstructions(&graph, &current_path);
            if obstruction_count < min_obstruction_count {
                min_obstruction_count = obstruction_count;

                // Print solution when found
                if should_display_full_solution {
                    print_solution(&current_path);

                    // TODO: Remove always-on printing of obstruction count
                    println!("Obstruction count: {}", obstruction_count);
                } else {
                    println!("{}", obstruction_count);
                }
            }
        }

        // Add extended paths in reverse order to pop and visit most promising paths first
        let extended_paths = extend_path(
            graph,
            &ordered_node_neighbors,
            &current_path,
            &current_path_set,
        );
        paths_to_visit.extend(extended_paths.into_iter().rev());
    }
}

fn get_ordered_starting_nodes(graph: &UnGraph<u16, ()>) -> Vec<NodeIndex> {
    // Sort nodes by increasing weight
    let mut node_indices: Vec<_> = graph.node_identifiers().collect();
    node_indices.sort_unstable_by_key(|&n| *graph.node_weight(n).unwrap());
    node_indices
}

fn get_ordered_node_neighbors(graph: &UnGraph<u16, ()>) -> FxHashMap<NodeIndex, Vec<NodeIndex>> {
    let mut ordered_node_neighbors = FxHashMap::default();

    for node_index in graph.node_identifiers() {
        // Sort nodes by increasing weight
        let node_neighbors: Vec<_> = graph
            .neighbors(node_index)
            .sorted_unstable_by_key(|&n| *graph.node_weight(n).unwrap())
            .collect();

        ordered_node_neighbors.insert(node_index, node_neighbors);
    }

    ordered_node_neighbors
}

fn extend_path(
    graph: &UnGraph<u16, ()>,
    ordered_node_neighbors: &FxHashMap<NodeIndex, Vec<NodeIndex>>,
    path: &[NodeIndex],
    path_set: &FxHashSet<NodeIndex>,
) -> Vec<(Vec<NodeIndex>, FxHashSet<NodeIndex>)> {
    let mut extended_paths = Vec::new();

    let unvisited_node_count = graph.node_count() - path.len();

    // Extend path at back of vector
    if let Some(last_node_index) = path.last() {
        let unvisited_neighbor_node_indices = ordered_node_neighbors
            .get(last_node_index)
            .unwrap()
            .iter()
            .filter(|x| !path_set.contains(x));

        for &neighbor_node_index in unvisited_neighbor_node_indices {
            // Unreacheable node heuristic
            let mut has_unreachable_second_neighbor = false;
            if unvisited_node_count > 2 {
                let second_neighbors = graph
                    .neighbors(neighbor_node_index)
                    .filter(|x| !path_set.contains(x));
                for second_neighbor_node_index in second_neighbors {
                    let mut second_neighbor_unvisited_neighbor_node_count = 0;

                    for third_neighbor_node_index in graph.neighbors(second_neighbor_node_index) {
                        if !path_set.contains(&third_neighbor_node_index) {
                            second_neighbor_unvisited_neighbor_node_count += 1;
                            if second_neighbor_unvisited_neighbor_node_count > 1 {
                                break;
                            }
                        }
                    }

                    if second_neighbor_unvisited_neighbor_node_count <= 1 {
                        has_unreachable_second_neighbor = true;
                        break;
                    }
                }
            }

            if !has_unreachable_second_neighbor {
                let extended_path = [path, &[neighbor_node_index]].concat();
                let mut extended_path_set = path_set.clone();
                extended_path_set.insert(neighbor_node_index);
                extended_paths.push((extended_path, extended_path_set));
            }
        }
    }

    extended_paths
}

#[allow(dead_code)]
pub fn get_node_degrees(graph: &UnGraph<u16, ()>) -> HashMap<NodeIndex, u32> {
    let mut node_degrees = HashMap::new();

    for node_index in graph.node_identifiers() {
        let degree = graph.edges(node_index).count() as u32;
        node_degrees.insert(node_index, degree);
    }

    node_degrees
}

#[allow(dead_code)]
fn find_node_with_minimum_weight(nodes: NodeReferences<u16>) -> NodeIndex {
    nodes.min_by_key(|&(_, weight)| weight).unwrap().0.into()
}
