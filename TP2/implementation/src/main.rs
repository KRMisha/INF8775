use std::time::Instant;

use ndarray::{Array2};
use structopt::StructOpt;

mod cli_args;
mod matrix_utils;

use cli_args::{Algorithm, Cli};
use matrix_utils::{load_matrix};

fn main() {
    // Parse args
    let args = Cli::from_args();

    // Load matrices
    let graph_matrix = load_matrix(&args.filename).expect("Error parsing graph adjacency matrix from file");

    // Start clock
    let now = Instant::now();

    // Execute selected algorithm
    let result = match args.algorithm {
        Algorithm::Greedy => solve_with_greedy(&graph_matrix),
        Algorithm::BranchBound => solve_with_branch_bound(&graph_matrix),
        Algorithm::Taboo => solve_with_taboo(&graph_matrix),
    };

    // Calculate elapsed time
    let elapsed_ms = now.elapsed().as_secs_f64() * 1000.0;

    if args.show_result {
        // TODO
    }

    if args.show_exec_time {
        println!("{}", elapsed_ms);
    }
}

fn solve_with_greedy(graph_matrix: &Array2<u8>) {

}

fn solve_with_branch_bound(graph_matrix: &Array2<u8>) {
    
}

fn solve_with_taboo(graph_matrix: &Array2<u8>) {
    
}
