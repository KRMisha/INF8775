use std::error::Error;

use structopt::StructOpt;

mod cli_args;
use cli_args::Cli;

mod utils;
use utils::{count_obstructions, load_graph, print_solution};

mod algorithm;
use algorithm::solve;

fn main() -> Result<(), Box<dyn Error>> {
    // Parse args
    let args = Cli::from_args();

    // Load data as a graph
    let graph = load_graph(&args.filename)?;

    // TODO: Loop
    let ordered_node_indices = solve(&graph);

    if args.print_solution {
        print_solution(&ordered_node_indices);
    } else {
        let obstruction_count = count_obstructions(&graph, &ordered_node_indices);
        println!("{}", obstruction_count);
    }

    Ok(())
}
