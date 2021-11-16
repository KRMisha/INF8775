use std::collections::HashMap;

use petgraph::matrix_graph::{NodeIndex, UnMatrix};

pub fn solve_with_tabu_search(graph: &UnMatrix<u8, ()>) -> HashMap<NodeIndex, usize> {
    HashMap::new()
}
