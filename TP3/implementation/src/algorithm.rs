use std::iter;

use hashbrown::HashMap;
use itertools::Itertools;
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::visit::IntoNodeIdentifiers;
use tinyset::Set64;

use crate::utils::{count_obstructions, print_solution};

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
            .map(|n| (vec![n], iter::once(n.index() as u32).collect())),
    );

    // Search for Hamiltonian paths with branch and bound
    while let Some((current_path, current_path_set)) = paths_to_visit.pop() {
        // Only count obstructions if the path length is long enough (avoid counting unless necessary)
        if current_path.len() >= min_obstruction_count as usize {
            let obstruction_count = count_obstructions(&graph, &current_path);

            // Skip path search tree if the partial solution is worse than the current best solution
            if obstruction_count >= min_obstruction_count {
                continue;
            }

            // Check solution quality if Hamiltonian path is complete
            let is_path_complete = current_path.len() == graph.node_count();
            if is_path_complete {
                min_obstruction_count = obstruction_count;

                // Print solution when found
                if should_display_full_solution {
                    print_solution(&current_path);

                    // TODO: Remove always-on printing of obstruction count
                    println!("Obstruction count: {}", obstruction_count);
                } else {
                    println!("{}", obstruction_count);
                }

                // Skip extending paths for a complete path
                continue;
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

fn get_ordered_node_neighbors(graph: &UnGraph<u16, ()>) -> HashMap<NodeIndex, Vec<NodeIndex>> {
    let mut ordered_node_neighbors = HashMap::new();

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
    ordered_node_neighbors: &HashMap<NodeIndex, Vec<NodeIndex>>,
    path: &[NodeIndex],
    path_set: &Set64<u32>,
) -> Vec<(Vec<NodeIndex>, Set64<u32>)> {
    let mut extended_paths = Vec::new();

    let unvisited_node_count = graph.node_count() - path.len();

    // Extend path at back of vector
    if let Some(last_node_index) = path.last() {
        let unvisited_neighbor_node_indices = ordered_node_neighbors
            .get(last_node_index)
            .unwrap()
            .iter()
            .filter(|x| !path_set.contains(x.index() as u32));

        for &neighbor_node_index in unvisited_neighbor_node_indices {
            // Unreacheable node heuristic
            let mut has_unreachable_second_neighbor = false;
            if unvisited_node_count > 2 {
                let second_neighbors = graph
                    .neighbors(neighbor_node_index)
                    .filter(|x| !path_set.contains(x.index() as u32));
                for second_neighbor_node_index in second_neighbors {
                    let mut second_neighbor_unvisited_neighbor_node_count = 0;

                    for third_neighbor_node_index in graph.neighbors(second_neighbor_node_index) {
                        if !path_set.contains(third_neighbor_node_index.index() as u32) {
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
                extended_path_set.insert(neighbor_node_index.index() as u32);
                extended_paths.push((extended_path, extended_path_set));
            }
        }
    }

    extended_paths
}
