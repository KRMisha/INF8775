use structopt::StructOpt;

mod cli_args;
use cli_args::Cli;

mod utils;
use utils::load_data;

mod algorithm;
use algorithm::solve;

fn main() {
    // Parse args
    let args = Cli::from_args();

    // Load dataset
    let data = load_data(&args.filename);

    // TODO: Loop
    let result = solve(&data);

    if args.show_obstruction_count_only {
        // TODO
    } else {
        // TODO
    }
}
