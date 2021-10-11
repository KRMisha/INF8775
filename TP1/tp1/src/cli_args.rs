use std::str::FromStr;
use structopt::StructOpt;

#[derive(StructOpt)]
pub enum Algorithm {
    Conventional,
    Strassen,
    StrassenThreshold,
}

impl FromStr for Algorithm {
    type Err = String;
    fn from_str(algorithm: &str) -> Result<Self, Self::Err> {
        match algorithm {
            "conv" => Ok(Algorithm::Conventional),
            "strassen" => Ok(Algorithm::Strassen),
            "strassenSeuil" => Ok(Algorithm::StrassenThreshold),
            _ => Err(format!("Could not parse algorithm: {}", algorithm)),
        }
    }
}

#[derive(StructOpt)]
pub struct Cli {
    /// The multiplication algorithm to use
    #[structopt(short)]
    pub algorithm: Algorithm,

    /// Path to the first matrix file
    #[structopt(long = "e1", parse(from_os_str))]
    pub matrix1_file_path: std::path::PathBuf,

    /// Path to the second matrix file
    #[structopt(long = "e2", parse(from_os_str))]
    pub matrix2_file_path: std::path::PathBuf,

    /// Print the resulting matrix
    #[structopt(short = "p")]
    pub show_result: bool,

    /// Print execution time in ns
    #[structopt(short = "t")]
    pub show_exec_time: bool,
}
