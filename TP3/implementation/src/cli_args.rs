use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Cli {
    /// Path to the input data file
    #[structopt(short = "e", parse(from_os_str))]
    pub filename: PathBuf,

    /// Prints the full solution rather than just the number of students whose view is obstructed
    #[structopt(short = "p")]
    pub show_obstruction_count_only: bool,
}
