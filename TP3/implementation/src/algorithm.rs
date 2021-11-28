use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::visit::Dfs;

pub fn solve(graph: &UnGraph<u16, ()>) -> Vec<NodeIndex> {
    // TODO: Replace placeholder algorithm

    let mut dfs = Dfs::new(graph, 0.into());

    let mut sorted_node_indices = Vec::with_capacity(graph.node_count());
    while let Some(node_index) = dfs.next(graph) {
        sorted_node_indices.push(node_index);
    }

    sorted_node_indices
}
