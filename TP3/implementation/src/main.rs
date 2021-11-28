use std::error::Error;

use structopt::StructOpt;

mod cli_args;
use cli_args::Cli;

mod utils;
use utils::{load_graph, print_result};

mod algorithm;
use algorithm::solve;

fn main() -> Result<(), Box<dyn Error>> {
    // Parse args
    let args = Cli::from_args();

    // Load data as a graph
    let graph = load_graph(&args.filename)?;

    // TODO: Loop
    let result = solve(&graph);

    if args.show_obstruction_count_only {
        // TODO
    } else {
        print_result(&result);
    }

    Ok(())
}
