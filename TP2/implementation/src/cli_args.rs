use std::path::PathBuf;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(StructOpt)]
pub enum Algorithm {
    Greedy,
    BranchAndBound,
    Tabu,
}

impl FromStr for Algorithm {
    type Err = String;
    fn from_str(algorithm: &str) -> Result<Self, Self::Err> {
        match algorithm {
            "glouton" => Ok(Algorithm::Greedy),
            "branch_bound" => Ok(Algorithm::BranchAndBound),
            "tabou" => Ok(Algorithm::Tabu),
            _ => Err(format!("Could not parse algorithm: {}", algorithm)),
        }
    }
}

#[derive(StructOpt)]
pub struct Cli {
    /// The multiplication algorithm to use
    #[structopt(short)]
    pub algorithm: Algorithm,

    /// Path to the graph adjacency matrix file
    #[structopt(short = "e", parse(from_os_str))]
    pub filename: PathBuf,

    /// Prints the solution
    #[structopt(short = "p")]
    pub show_result: bool,

    /// Prints execution time in ns
    #[structopt(short = "t")]
    pub show_exec_time: bool,
}
