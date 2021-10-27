use petgraph::matrix_graph::{UnMatrix, NodeIndex};
use petgraph::visit::IntoNodeReferences;

pub fn solve_with_greedy(graph: &UnMatrix<u8, ()>) {
    let starting_node_index = find_node_with_maximal_degree(&graph);
    println!("Starting node: {}", starting_node_index.index());
}

fn find_node_with_maximal_degree(graph: &UnMatrix<u8, ()>) -> NodeIndex {
    let mut max_degree_node_index = NodeIndex::new(0);
    let mut max_degree = 0usize;
    
    for node in graph.node_references() {
        let node_index = node.0;
        let degree = graph.edges(node_index).count();
        if degree > max_degree {
            max_degree_node_index = node_index;
            max_degree = degree;
        }
    }

    max_degree_node_index
}
