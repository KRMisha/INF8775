use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use petgraph::matrix_graph::{UnMatrix, NodeIndex};
use petgraph::visit::{IntoNodeIdentifiers, IntoNodeReferences};

pub fn solve_with_greedy(graph: &UnMatrix<u8, ()>) -> Vec<usize> {
    let node_set: HashSet<_> = graph.node_identifiers().collect();
    let node_degrees = get_node_degrees(graph);

    let mut current_max_color = 0usize;
    let mut node_colors = HashMap::<NodeIndex<u16>, usize>::new();

    // Color starting node
    let starting_node_index = find_node_with_maximal_degree(&node_degrees);
    node_colors.insert(starting_node_index, 0);

    // Color all nodes
    while node_colors.len() < graph.node_count() {
        // Compute remaining uncolored nodes
        let colored_node_set: HashSet<_> = node_colors.keys().cloned().collect();
        let uncolored_nodes_indexes: Vec<_> = node_set.difference(&colored_node_set).cloned().collect();
        
        // Greedy choice: node degree of saturation
        let mut max_saturation_node_index = *uncolored_nodes_indexes.first().unwrap();
        let mut max_saturation = 0usize;
        
        for uncolored_node_index in uncolored_nodes_indexes {
            let saturation = get_neighbor_unique_colors(graph, uncolored_node_index, &node_colors).len();

            if max_saturation > saturation {
                continue;
            }

            if max_saturation == saturation &&
                node_degrees[max_saturation_node_index.index()] < node_degrees[uncolored_node_index.index()] {
                continue;
            }

            max_saturation_node_index = uncolored_node_index;
            max_saturation = saturation;
        }

        // Smallest possible color
        let current_node_index = max_saturation_node_index;
        let neighbor_colors = get_neighbor_unique_colors(graph, current_node_index, &node_colors);

        let mut was_existing_color_available = false;
        for i in 0..current_max_color {
            if !neighbor_colors.contains(&i) {
                node_colors.insert(current_node_index, i);
                was_existing_color_available = true;
                break;
            }
        }

        if !was_existing_color_available {
            current_max_color += 1;
            node_colors.insert(current_node_index, current_max_color);
        }
    }

    let node_colors_vec: Vec<_> = node_colors.iter().sorted().map(|(&_node_index, &color)| color).collect();
    node_colors_vec
}

fn get_node_degrees(graph: &UnMatrix<u8, ()>) -> Vec<usize> {
    let mut node_degrees = Vec::with_capacity(graph.node_count());

    for node in graph.node_references() {
        let node_index = node.0;
        let degree = graph.edges(node_index).count();
        node_degrees.push(degree);
    }

    node_degrees
}

fn find_node_with_maximal_degree(node_degrees: &Vec<usize>) -> NodeIndex {
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

fn get_neighbor_unique_colors(graph: &UnMatrix<u8, ()>, node_index: NodeIndex<u16>, node_colors: &HashMap<NodeIndex<u16>, usize>) -> HashSet<usize> {
    let mut unique_neighbor_colors = HashSet::new();
    for neighbor_node_index in graph.neighbors(node_index) {
        if let Some(color) = node_colors.get(&neighbor_node_index) {
            unique_neighbor_colors.insert(*color);
        }
    }

    unique_neighbor_colors
}
