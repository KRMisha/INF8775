use petgraph::graph::{NodeIndex, NodeReferences, UnGraph};
use petgraph::visit::{Dfs, IntoNodeReferences};

pub fn solve(graph: &UnGraph<u16, ()>) -> Vec<NodeIndex> {
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
