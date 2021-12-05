use std::error::Error;

use structopt::StructOpt;

mod cli_args;
use cli_args::Cli;

mod utils;
use utils::load_graph;

mod algorithm;
use algorithm::solve_in_loop;

fn main() -> Result<(), Box<dyn Error>> {
    // Parse args
    let args = Cli::from_args();

    // Load data as a graph
    let graph = load_graph(&args.filename)?;

    // Loop until best solution is found
    let ordered_node_indices = solve_in_loop(&graph, args.print_solution);

    Ok(())
}
