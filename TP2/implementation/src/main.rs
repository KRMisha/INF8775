use std::time::Instant;

use petgraph::matrix_graph::UnMatrix;
use structopt::StructOpt;

mod cli_args;
use cli_args::{Algorithm, Cli};

mod graph_utils;
use graph_utils::{load_graph, print_result};

mod greedy_algorithm;
use greedy_algorithm::solve_with_greedy;

fn main() {
    // Parse args
    let args = Cli::from_args();

    // Load graph
    let graph = load_graph(&args.filename).expect("Error parsing graph adjacency matrix from file");

    // Start clock
    let now = Instant::now();

    // Execute selected algorithm
    let result = match args.algorithm {
        Algorithm::Greedy => solve_with_greedy(&graph),
        Algorithm::BranchBound => solve_with_branch_bound(&graph),
        Algorithm::Taboo => solve_with_taboo(&graph),
    };

    // Calculate elapsed time
    let elapsed_ms = now.elapsed().as_secs_f64() * 1000.0;

    if args.show_result {
        print_result(&result);
    }

    if args.show_exec_time {
        println!("{}", elapsed_ms);
    }
}

fn solve_with_branch_bound(graph: &UnMatrix<u8, ()>) -> Vec<usize> {
    vec![1, 2, 3]
}

fn solve_with_taboo(graph: &UnMatrix<u8, ()>) -> Vec<usize> {
    vec![1, 2, 3]
}
