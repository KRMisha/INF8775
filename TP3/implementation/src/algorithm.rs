use petgraph::graph::{NodeIndex, NodeReferences, UnGraph};
use petgraph::visit::IntoNodeIdentifiers;

pub fn solve(graph: &UnGraph<u16, ()>) -> Vec<NodeIndex> {
    let hamiltonian_path = find_hamiltonian_path_with_backtracking(graph);
    match hamiltonian_path {
        Some(hamiltonian_path) => hamiltonian_path,
        None => {
            // TODO: Replace with Option return type
            // or avoid this problem by integrating the multiple solution loop here
            // or by making the solve function include the backtracking algorithm
            eprintln!("Error: No path found");
            Vec::new()
        }
    }
}

#[allow(dead_code)]
fn find_node_with_minimum_weight(nodes: NodeReferences<u16>) -> NodeIndex {
    nodes.min_by_key(|&(_, weight)| weight).unwrap().0.into()
}

fn find_hamiltonian_path_with_backtracking(graph: &UnGraph<u16, ()>) -> Option<Vec<NodeIndex>> {
    // Stack of partial paths to visit
    let mut paths_to_visit = Vec::new();

    // Any node can act as the starting node for the search tree
    let ordered_starting_nodes = get_ordered_starting_nodes(graph);
    paths_to_visit.extend(ordered_starting_nodes.into_iter().rev().map(|n| vec![n]));

    while let Some(current_path) = paths_to_visit.pop() {
        let is_path_complete = current_path.len() == graph.node_count();
        if is_path_complete {
            // TODO: Yield multiple paths rather than returning on first solution (iterate to improve solution)
            return Some(current_path);
        }

        // Add extended paths in reverse order to pop and visit most promising paths first
        let extended_paths = extend_path(graph, &current_path);
        paths_to_visit.extend(extended_paths.into_iter().rev());
    }

    None
}

fn get_ordered_starting_nodes(graph: &UnGraph<u16, ()>) -> Vec<NodeIndex> {
    // TODO: Select nodes in order of estimated relevance (heuristic)
    graph.node_identifiers().collect()
}

fn extend_path(graph: &UnGraph<u16, ()>, path: &[NodeIndex]) -> Vec<Vec<NodeIndex>> {
    let mut extended_paths = Vec::new();

    // TODO: Consider extending path at either front or back of vector
    if let Some(&last_node_index) = path.last() {
        // The hash set creation overhead is often worse than a linear lookup in this case
        let unvisited_neighbor_node_indices: Vec<_> = graph
            .neighbors(last_node_index)
            .filter(|x| !path.contains(x))
            .collect();
        // TODO: Add extended paths in order of estimated relevance (heuristic)
        for neighbor_node_index in unvisited_neighbor_node_indices {
            extended_paths.push([path, &[neighbor_node_index]].concat());
        }
    }

    extended_paths
}
